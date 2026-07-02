#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_311(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign84310_e129154, assign84310_e129154_d_n0, assign84310_e129154_d_n2, assign84310_e129154_d_n4, assign84310_e129154_d_n5, assign84310_e129154_d_n6, assign84310_e129154_d_n7, assign84310_e129154_d_n8, assign84310_e129154_d_n9, assign84310_e129154_d_n10, assign84310_e129154_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) {
        let assign84310_e129152: f64 = (locals.var_c_w_ld * locals.var_fb);
        (assign84310_e129152, ((locals.var_c_w_ld_dn0 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn0)), ((locals.var_c_w_ld_dn2 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn2)), ((locals.var_c_w_ld_dn4 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn4)), ((locals.var_c_w_ld_dn5 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn5)), ((locals.var_c_w_ld_dn6 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn6)), ((locals.var_c_w_ld_dn7 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn7)), ((locals.var_c_w_ld_dn8 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn8)), ((locals.var_c_w_ld_dn9 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn9)), ((locals.var_c_w_ld_dn10 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn10)), ((locals.var_c_w_ld_dn13 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn13)),)
    } else {
        (locals.var_wdld__blk1881, locals.var_wdld__blk1881_dn0, locals.var_wdld__blk1881_dn2, locals.var_wdld__blk1881_dn4, locals.var_wdld__blk1881_dn5, locals.var_wdld__blk1881_dn6, locals.var_wdld__blk1881_dn7, locals.var_wdld__blk1881_dn8, locals.var_wdld__blk1881_dn9, locals.var_wdld__blk1881_dn10, locals.var_wdld__blk1881_dn13,)
    }
};
        locals.var_wdld__blk1881 = assign84310_e129154;
        locals.var_wdld__blk1881_dn0 = assign84310_e129154_d_n0;
        locals.var_wdld__blk1881_dn2 = assign84310_e129154_d_n2;
        locals.var_wdld__blk1881_dn4 = assign84310_e129154_d_n4;
        locals.var_wdld__blk1881_dn5 = assign84310_e129154_d_n5;
        locals.var_wdld__blk1881_dn6 = assign84310_e129154_d_n6;
        locals.var_wdld__blk1881_dn7 = assign84310_e129154_d_n7;
        locals.var_wdld__blk1881_dn8 = assign84310_e129154_d_n8;
        locals.var_wdld__blk1881_dn9 = assign84310_e129154_d_n9;
        locals.var_wdld__blk1881_dn10 = assign84310_e129154_d_n10;
        locals.var_wdld__blk1881_dn13 = assign84310_e129154_d_n13;
        locals.var_wdld__blk1881_rv = 0.0;

        let (assign84320_e129165, assign84320_e129165_d_n0, assign84320_e129165_d_n2, assign84320_e129165_d_n4, assign84320_e129165_d_n5, assign84320_e129165_d_n6, assign84320_e129165_d_n7, assign84320_e129165_d_n8, assign84320_e129165_d_n9, assign84320_e129165_d_n10, assign84320_e129165_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) {
        let assign84320_e129163: f64 = (locals.var_q_nsubld__blk1883 * locals.var_wdld__blk1881);
        (assign84320_e129163, (locals.var_q_nsubld__blk1883 * locals.var_wdld__blk1881_dn0), (locals.var_q_nsubld__blk1883 * locals.var_wdld__blk1881_dn2), (locals.var_q_nsubld__blk1883 * locals.var_wdld__blk1881_dn4), (locals.var_q_nsubld__blk1883 * locals.var_wdld__blk1881_dn5), (locals.var_q_nsubld__blk1883 * locals.var_wdld__blk1881_dn6), (locals.var_q_nsubld__blk1883 * locals.var_wdld__blk1881_dn7), (locals.var_q_nsubld__blk1883 * locals.var_wdld__blk1881_dn8), (locals.var_q_nsubld__blk1883 * locals.var_wdld__blk1881_dn9), (locals.var_q_nsubld__blk1883 * locals.var_wdld__blk1881_dn10), (locals.var_q_nsubld__blk1883 * locals.var_wdld__blk1881_dn13),)
    } else {
        (locals.var_q_dep_ld__blk1882, locals.var_q_dep_ld__blk1882_dn0, locals.var_q_dep_ld__blk1882_dn2, locals.var_q_dep_ld__blk1882_dn4, locals.var_q_dep_ld__blk1882_dn5, locals.var_q_dep_ld__blk1882_dn6, locals.var_q_dep_ld__blk1882_dn7, locals.var_q_dep_ld__blk1882_dn8, locals.var_q_dep_ld__blk1882_dn9, locals.var_q_dep_ld__blk1882_dn10, locals.var_q_dep_ld__blk1882_dn13,)
    }
};
        locals.var_q_dep_ld__blk1882 = assign84320_e129165;
        locals.var_q_dep_ld__blk1882_dn0 = assign84320_e129165_d_n0;
        locals.var_q_dep_ld__blk1882_dn2 = assign84320_e129165_d_n2;
        locals.var_q_dep_ld__blk1882_dn4 = assign84320_e129165_d_n4;
        locals.var_q_dep_ld__blk1882_dn5 = assign84320_e129165_d_n5;
        locals.var_q_dep_ld__blk1882_dn6 = assign84320_e129165_d_n6;
        locals.var_q_dep_ld__blk1882_dn7 = assign84320_e129165_d_n7;
        locals.var_q_dep_ld__blk1882_dn8 = assign84320_e129165_d_n8;
        locals.var_q_dep_ld__blk1882_dn9 = assign84320_e129165_d_n9;
        locals.var_q_dep_ld__blk1882_dn10 = assign84320_e129165_d_n10;
        locals.var_q_dep_ld__blk1882_dn13 = assign84320_e129165_d_n13;
        locals.var_q_dep_ld__blk1882_rv = 0.0;

        let (assign84330_e129180, assign84330_e129180_d_n0, assign84330_e129180_d_n2, assign84330_e129180_d_n4, assign84330_e129180_d_n5, assign84330_e129180_d_n6, assign84330_e129180_d_n7, assign84330_e129180_d_n8, assign84330_e129180_d_n9, assign84330_e129180_d_n10, assign84330_e129180_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) {
        let assign84330_e129174: f64 = (locals.var_q_dep_ld__blk1882 / locals.var_cnst0over_func);
        let assign84330_e129177: f64 = (10.0 * 2.220446049250313e-16);
        let assign84330_e129178: f64 = (assign84330_e129174 + assign84330_e129177);
        (assign84330_e129178, (((locals.var_q_dep_ld__blk1882_dn0 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1882 * locals.var_cnst0over_func_dn0)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1882_dn2 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1882 * locals.var_cnst0over_func_dn2)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1882_dn4 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1882 * locals.var_cnst0over_func_dn4)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1882_dn5 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1882 * locals.var_cnst0over_func_dn5)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1882_dn6 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1882 * locals.var_cnst0over_func_dn6)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1882_dn7 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1882 * locals.var_cnst0over_func_dn7)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1882_dn8 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1882 * locals.var_cnst0over_func_dn8)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1882_dn9 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1882 * locals.var_cnst0over_func_dn9)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1882_dn10 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1882 * locals.var_cnst0over_func_dn10)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1882_dn13 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1882 * locals.var_cnst0over_func_dn13)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)),)
    } else {
        (locals.var_xi0p12, locals.var_xi0p12_dn0, locals.var_xi0p12_dn2, locals.var_xi0p12_dn4, locals.var_xi0p12_dn5, locals.var_xi0p12_dn6, locals.var_xi0p12_dn7, locals.var_xi0p12_dn8, locals.var_xi0p12_dn9, locals.var_xi0p12_dn10, locals.var_xi0p12_dn13,)
    }
};
        locals.var_xi0p12 = assign84330_e129180;
        locals.var_xi0p12_dn0 = assign84330_e129180_d_n0;
        locals.var_xi0p12_dn2 = assign84330_e129180_d_n2;
        locals.var_xi0p12_dn4 = assign84330_e129180_d_n4;
        locals.var_xi0p12_dn5 = assign84330_e129180_d_n5;
        locals.var_xi0p12_dn6 = assign84330_e129180_d_n6;
        locals.var_xi0p12_dn7 = assign84330_e129180_d_n7;
        locals.var_xi0p12_dn8 = assign84330_e129180_d_n8;
        locals.var_xi0p12_dn9 = assign84330_e129180_d_n9;
        locals.var_xi0p12_dn10 = assign84330_e129180_d_n10;
        locals.var_xi0p12_dn13 = assign84330_e129180_d_n13;
        locals.var_xi0p12_rv = 0.0;

        let (assign84340_e129191, assign84340_e129191_d_n0, assign84340_e129191_d_n2, assign84340_e129191_d_n4, assign84340_e129191_d_n5, assign84340_e129191_d_n6, assign84340_e129191_d_n7, assign84340_e129191_d_n8, assign84340_e129191_d_n9, assign84340_e129191_d_n10, assign84340_e129191_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) {
        let assign84340_e129189: f64 = (locals.var_cnst0over_func * locals.var_xi0p12);
        (assign84340_e129189, ((locals.var_cnst0over_func_dn0 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn0)), ((locals.var_cnst0over_func_dn2 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn2)), ((locals.var_cnst0over_func_dn4 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn4)), ((locals.var_cnst0over_func_dn5 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn5)), ((locals.var_cnst0over_func_dn6 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn6)), ((locals.var_cnst0over_func_dn7 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn7)), ((locals.var_cnst0over_func_dn8 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn8)), ((locals.var_cnst0over_func_dn9 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn9)), ((locals.var_cnst0over_func_dn10 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn10)), ((locals.var_cnst0over_func_dn13 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn13)),)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn8, locals.var_qbuld_dn9, locals.var_qbuld_dn10, locals.var_qbuld_dn13,)
    }
};
        locals.var_qbuld = assign84340_e129191;
        locals.var_qbuld_dn0 = assign84340_e129191_d_n0;
        locals.var_qbuld_dn2 = assign84340_e129191_d_n2;
        locals.var_qbuld_dn4 = assign84340_e129191_d_n4;
        locals.var_qbuld_dn5 = assign84340_e129191_d_n5;
        locals.var_qbuld_dn6 = assign84340_e129191_d_n6;
        locals.var_qbuld_dn7 = assign84340_e129191_d_n7;
        locals.var_qbuld_dn8 = assign84340_e129191_d_n8;
        locals.var_qbuld_dn9 = assign84340_e129191_d_n9;
        locals.var_qbuld_dn10 = assign84340_e129191_d_n10;
        locals.var_qbuld_dn13 = assign84340_e129191_d_n13;
        locals.var_qbuld_rv = 0.0;

        let (assign84350_e129204, assign84350_e129204_d_n0, assign84350_e129204_d_n2, assign84350_e129204_d_n4, assign84350_e129204_d_n5, assign84350_e129204_d_n6, assign84350_e129204_d_n7, assign84350_e129204_d_n8, assign84350_e129204_d_n9, assign84350_e129204_d_n10, assign84350_e129204_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) {
        let assign84350_e129201: f64 = (locals.var_fs02 + locals.var_xi0p12);
        let assign84350_e129202: f64 = (1.0 / assign84350_e129201);
        (assign84350_e129202, (-((locals.var_fs02_dn0 + locals.var_xi0p12_dn0) / (assign84350_e129201 * assign84350_e129201))), (-((locals.var_fs02_dn2 + locals.var_xi0p12_dn2) / (assign84350_e129201 * assign84350_e129201))), (-((locals.var_fs02_dn4 + locals.var_xi0p12_dn4) / (assign84350_e129201 * assign84350_e129201))), (-((locals.var_fs02_dn5 + locals.var_xi0p12_dn5) / (assign84350_e129201 * assign84350_e129201))), (-((locals.var_fs02_dn6 + locals.var_xi0p12_dn6) / (assign84350_e129201 * assign84350_e129201))), (-((locals.var_fs02_dn7 + locals.var_xi0p12_dn7) / (assign84350_e129201 * assign84350_e129201))), (-((locals.var_fs02_dn8 + locals.var_xi0p12_dn8) / (assign84350_e129201 * assign84350_e129201))), (-((locals.var_fs02_dn9 + locals.var_xi0p12_dn9) / (assign84350_e129201 * assign84350_e129201))), (-((locals.var_fs02_dn10 + locals.var_xi0p12_dn10) / (assign84350_e129201 * assign84350_e129201))), (-((locals.var_fs02_dn13 + locals.var_xi0p12_dn13) / (assign84350_e129201 * assign84350_e129201))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign84350_e129204;
        locals.var_t1_dn0 = assign84350_e129204_d_n0;
        locals.var_t1_dn2 = assign84350_e129204_d_n2;
        locals.var_t1_dn4 = assign84350_e129204_d_n4;
        locals.var_t1_dn5 = assign84350_e129204_d_n5;
        locals.var_t1_dn6 = assign84350_e129204_d_n6;
        locals.var_t1_dn7 = assign84350_e129204_d_n7;
        locals.var_t1_dn8 = assign84350_e129204_d_n8;
        locals.var_t1_dn9 = assign84350_e129204_d_n9;
        locals.var_t1_dn10 = assign84350_e129204_d_n10;
        locals.var_t1_dn13 = assign84350_e129204_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign84360_e129217, assign84360_e129217_d_n0, assign84360_e129217_d_n2, assign84360_e129217_d_n4, assign84360_e129217_d_n5, assign84360_e129217_d_n6, assign84360_e129217_d_n7, assign84360_e129217_d_n8, assign84360_e129217_d_n9, assign84360_e129217_d_n10, assign84360_e129217_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) {
        let assign84360_e129213: f64 = (locals.var_cnst0over_func * locals.var_fs01);
        let assign84360_e129215: f64 = (assign84360_e129213 * locals.var_t1);
        (assign84360_e129215, ((((locals.var_cnst0over_func_dn0 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn0)) * locals.var_t1) + (assign84360_e129213 * locals.var_t1_dn0)), ((((locals.var_cnst0over_func_dn2 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn2)) * locals.var_t1) + (assign84360_e129213 * locals.var_t1_dn2)), ((((locals.var_cnst0over_func_dn4 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn4)) * locals.var_t1) + (assign84360_e129213 * locals.var_t1_dn4)), ((((locals.var_cnst0over_func_dn5 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn5)) * locals.var_t1) + (assign84360_e129213 * locals.var_t1_dn5)), ((((locals.var_cnst0over_func_dn6 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn6)) * locals.var_t1) + (assign84360_e129213 * locals.var_t1_dn6)), ((((locals.var_cnst0over_func_dn7 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn7)) * locals.var_t1) + (assign84360_e129213 * locals.var_t1_dn7)), ((((locals.var_cnst0over_func_dn8 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn8)) * locals.var_t1) + (assign84360_e129213 * locals.var_t1_dn8)), ((((locals.var_cnst0over_func_dn9 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn9)) * locals.var_t1) + (assign84360_e129213 * locals.var_t1_dn9)), ((((locals.var_cnst0over_func_dn10 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn10)) * locals.var_t1) + (assign84360_e129213 * locals.var_t1_dn10)), ((((locals.var_cnst0over_func_dn13 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn13)) * locals.var_t1) + (assign84360_e129213 * locals.var_t1_dn13)),)
    } else {
        (locals.var_qiuld, locals.var_qiuld_dn0, locals.var_qiuld_dn2, locals.var_qiuld_dn4, locals.var_qiuld_dn5, locals.var_qiuld_dn6, locals.var_qiuld_dn7, locals.var_qiuld_dn8, locals.var_qiuld_dn9, locals.var_qiuld_dn10, locals.var_qiuld_dn13,)
    }
};
        locals.var_qiuld = assign84360_e129217;
        locals.var_qiuld_dn0 = assign84360_e129217_d_n0;
        locals.var_qiuld_dn2 = assign84360_e129217_d_n2;
        locals.var_qiuld_dn4 = assign84360_e129217_d_n4;
        locals.var_qiuld_dn5 = assign84360_e129217_d_n5;
        locals.var_qiuld_dn6 = assign84360_e129217_d_n6;
        locals.var_qiuld_dn7 = assign84360_e129217_d_n7;
        locals.var_qiuld_dn8 = assign84360_e129217_d_n8;
        locals.var_qiuld_dn9 = assign84360_e129217_d_n9;
        locals.var_qiuld_dn10 = assign84360_e129217_d_n10;
        locals.var_qiuld_dn13 = assign84360_e129217_d_n13;
        locals.var_qiuld_rv = 0.0;

        let (assign84370_e129228, assign84370_e129228_d_n0, assign84370_e129228_d_n2, assign84370_e129228_d_n4, assign84370_e129228_d_n5, assign84370_e129228_d_n6, assign84370_e129228_d_n7, assign84370_e129228_d_n8, assign84370_e129228_d_n9, assign84370_e129228_d_n10, assign84370_e129228_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) {
        let assign84370_e129226: f64 = (locals.var_qbuld + locals.var_qiuld);
        (assign84370_e129226, (locals.var_qbuld_dn0 + locals.var_qiuld_dn0), (locals.var_qbuld_dn2 + locals.var_qiuld_dn2), (locals.var_qbuld_dn4 + locals.var_qiuld_dn4), (locals.var_qbuld_dn5 + locals.var_qiuld_dn5), (locals.var_qbuld_dn6 + locals.var_qiuld_dn6), (locals.var_qbuld_dn7 + locals.var_qiuld_dn7), (locals.var_qbuld_dn8 + locals.var_qiuld_dn8), (locals.var_qbuld_dn9 + locals.var_qiuld_dn9), (locals.var_qbuld_dn10 + locals.var_qiuld_dn10), (locals.var_qbuld_dn13 + locals.var_qiuld_dn13),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn4, locals.var_qsuld_dn5, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn8, locals.var_qsuld_dn9, locals.var_qsuld_dn10, locals.var_qsuld_dn13,)
    }
};
        locals.var_qsuld = assign84370_e129228;
        locals.var_qsuld_dn0 = assign84370_e129228_d_n0;
        locals.var_qsuld_dn2 = assign84370_e129228_d_n2;
        locals.var_qsuld_dn4 = assign84370_e129228_d_n4;
        locals.var_qsuld_dn5 = assign84370_e129228_d_n5;
        locals.var_qsuld_dn6 = assign84370_e129228_d_n6;
        locals.var_qsuld_dn7 = assign84370_e129228_d_n7;
        locals.var_qsuld_dn8 = assign84370_e129228_d_n8;
        locals.var_qsuld_dn9 = assign84370_e129228_d_n9;
        locals.var_qsuld_dn10 = assign84370_e129228_d_n10;
        locals.var_qsuld_dn13 = assign84370_e129228_d_n13;
        locals.var_qsuld_rv = 0.0;

        let assign84380_e129231: f64 = if p.p33 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1964 = assign84380_e129231;
        locals.var_guard1964_rv = 0.0;

        let (assign84390_e129241, assign84390_e129241_d_n0, assign84390_e129241_d_n2, assign84390_e129241_d_n4, assign84390_e129241_d_n5, assign84390_e129241_d_n6, assign84390_e129241_d_n7, assign84390_e129241_d_n8, assign84390_e129241_d_n9, assign84390_e129241_d_n10, assign84390_e129241_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) {
        let assign84390_e129237: f64 = (-locals.var_vxbgmtcl);
        let assign84390_e129238: f64 = (locals.var_beta * assign84390_e129237);
        let assign84390_e129239: f64 = (assign84390_e129238).exp();
        (assign84390_e129239, (assign84390_e129239 * ((locals.var_beta_dn0 * assign84390_e129237) + (locals.var_beta * (-locals.var_vxbgmtcl_dn0)))), (assign84390_e129239 * ((locals.var_beta_dn2 * assign84390_e129237) + (locals.var_beta * (-locals.var_vxbgmtcl_dn2)))), (assign84390_e129239 * ((locals.var_beta_dn4 * assign84390_e129237) + (locals.var_beta * (-locals.var_vxbgmtcl_dn4)))), (assign84390_e129239 * ((locals.var_beta_dn5 * assign84390_e129237) + (locals.var_beta * (-locals.var_vxbgmtcl_dn5)))), (assign84390_e129239 * ((locals.var_beta_dn6 * assign84390_e129237) + (locals.var_beta * (-locals.var_vxbgmtcl_dn6)))), (assign84390_e129239 * ((locals.var_beta_dn7 * assign84390_e129237) + (locals.var_beta * (-locals.var_vxbgmtcl_dn7)))), (assign84390_e129239 * ((locals.var_beta_dn8 * assign84390_e129237) + (locals.var_beta * (-locals.var_vxbgmtcl_dn8)))), (assign84390_e129239 * ((locals.var_beta_dn9 * assign84390_e129237) + (locals.var_beta * (-locals.var_vxbgmtcl_dn9)))), (assign84390_e129239 * ((locals.var_beta_dn10 * assign84390_e129237) + (locals.var_beta * (-locals.var_vxbgmtcl_dn10)))), (assign84390_e129239 * ((locals.var_beta_dn13 * assign84390_e129237) + (locals.var_beta * (-locals.var_vxbgmtcl_dn13)))),)
    } else {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn13,)
    }
};
        locals.var_exp_bvbs = assign84390_e129241;
        locals.var_exp_bvbs_dn0 = assign84390_e129241_d_n0;
        locals.var_exp_bvbs_dn2 = assign84390_e129241_d_n2;
        locals.var_exp_bvbs_dn4 = assign84390_e129241_d_n4;
        locals.var_exp_bvbs_dn5 = assign84390_e129241_d_n5;
        locals.var_exp_bvbs_dn6 = assign84390_e129241_d_n6;
        locals.var_exp_bvbs_dn7 = assign84390_e129241_d_n7;
        locals.var_exp_bvbs_dn8 = assign84390_e129241_d_n8;
        locals.var_exp_bvbs_dn9 = assign84390_e129241_d_n9;
        locals.var_exp_bvbs_dn10 = assign84390_e129241_d_n10;
        locals.var_exp_bvbs_dn13 = assign84390_e129241_d_n13;
        locals.var_exp_bvbs_rv = 0.0;

        let (assign84400_e129249, assign84400_e129249_d_n0, assign84400_e129249_d_n2, assign84400_e129249_d_n4, assign84400_e129249_d_n5, assign84400_e129249_d_n6, assign84400_e129249_d_n7, assign84400_e129249_d_n8, assign84400_e129249_d_n9, assign84400_e129249_d_n10, assign84400_e129249_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) {
        let assign84400_e129247: f64 = (locals.var_nin / locals.var_nover_func);
        (assign84400_e129247, (locals.var_nin_dn0 / locals.var_nover_func), (locals.var_nin_dn2 / locals.var_nover_func), (locals.var_nin_dn4 / locals.var_nover_func), (locals.var_nin_dn5 / locals.var_nover_func), (locals.var_nin_dn6 / locals.var_nover_func), (locals.var_nin_dn7 / locals.var_nover_func), (locals.var_nin_dn8 / locals.var_nover_func), (locals.var_nin_dn9 / locals.var_nover_func), (locals.var_nin_dn10 / locals.var_nover_func), (locals.var_nin_dn13 / locals.var_nover_func),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign84400_e129249;
        locals.var_t0_dn0 = assign84400_e129249_d_n0;
        locals.var_t0_dn2 = assign84400_e129249_d_n2;
        locals.var_t0_dn4 = assign84400_e129249_d_n4;
        locals.var_t0_dn5 = assign84400_e129249_d_n5;
        locals.var_t0_dn6 = assign84400_e129249_d_n6;
        locals.var_t0_dn7 = assign84400_e129249_d_n7;
        locals.var_t0_dn8 = assign84400_e129249_d_n8;
        locals.var_t0_dn9 = assign84400_e129249_d_n9;
        locals.var_t0_dn10 = assign84400_e129249_d_n10;
        locals.var_t0_dn13 = assign84400_e129249_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign84410_e129257, assign84410_e129257_d_n0, assign84410_e129257_d_n2, assign84410_e129257_d_n4, assign84410_e129257_d_n5, assign84410_e129257_d_n6, assign84410_e129257_d_n7, assign84410_e129257_d_n8, assign84410_e129257_d_n9, assign84410_e129257_d_n10, assign84410_e129257_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) {
        let assign84410_e129255: f64 = (locals.var_t0 * locals.var_t0);
        (assign84410_e129255, ((locals.var_t0_dn0 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn0)), ((locals.var_t0_dn2 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn2)), ((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)), ((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)), ((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)), ((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)), ((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)), ((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)), ((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)), ((locals.var_t0_dn13 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn13)),)
    } else {
        (locals.var_cnst1over, locals.var_cnst1over_dn0, locals.var_cnst1over_dn2, locals.var_cnst1over_dn4, locals.var_cnst1over_dn5, locals.var_cnst1over_dn6, locals.var_cnst1over_dn7, locals.var_cnst1over_dn8, locals.var_cnst1over_dn9, locals.var_cnst1over_dn10, locals.var_cnst1over_dn13,)
    }
};
        locals.var_cnst1over = assign84410_e129257;
        locals.var_cnst1over_dn0 = assign84410_e129257_d_n0;
        locals.var_cnst1over_dn2 = assign84410_e129257_d_n2;
        locals.var_cnst1over_dn4 = assign84410_e129257_d_n4;
        locals.var_cnst1over_dn5 = assign84410_e129257_d_n5;
        locals.var_cnst1over_dn6 = assign84410_e129257_d_n6;
        locals.var_cnst1over_dn7 = assign84410_e129257_d_n7;
        locals.var_cnst1over_dn8 = assign84410_e129257_d_n8;
        locals.var_cnst1over_dn9 = assign84410_e129257_d_n9;
        locals.var_cnst1over_dn10 = assign84410_e129257_d_n10;
        locals.var_cnst1over_dn13 = assign84410_e129257_d_n13;
        locals.var_cnst1over_rv = 0.0;

        let (assign84420_e129265, assign84420_e129265_d_n0, assign84420_e129265_d_n2, assign84420_e129265_d_n4, assign84420_e129265_d_n5, assign84420_e129265_d_n6, assign84420_e129265_d_n7, assign84420_e129265_d_n8, assign84420_e129265_d_n9, assign84420_e129265_d_n10, assign84420_e129265_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) {
        let assign84420_e129263: f64 = (locals.var_cnst1over * locals.var_exp_bvbs);
        (assign84420_e129263, ((locals.var_cnst1over_dn0 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn0)), ((locals.var_cnst1over_dn2 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn2)), ((locals.var_cnst1over_dn4 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn4)), ((locals.var_cnst1over_dn5 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn5)), ((locals.var_cnst1over_dn6 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn6)), ((locals.var_cnst1over_dn7 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn7)), ((locals.var_cnst1over_dn8 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn8)), ((locals.var_cnst1over_dn9 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn9)), ((locals.var_cnst1over_dn10 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn10)), ((locals.var_cnst1over_dn13 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn13)),)
    } else {
        (locals.var_cfs1, locals.var_cfs1_dn0, locals.var_cfs1_dn2, locals.var_cfs1_dn4, locals.var_cfs1_dn5, locals.var_cfs1_dn6, locals.var_cfs1_dn7, locals.var_cfs1_dn8, locals.var_cfs1_dn9, locals.var_cfs1_dn10, locals.var_cfs1_dn13,)
    }
};
        locals.var_cfs1 = assign84420_e129265;
        locals.var_cfs1_dn0 = assign84420_e129265_d_n0;
        locals.var_cfs1_dn2 = assign84420_e129265_d_n2;
        locals.var_cfs1_dn4 = assign84420_e129265_d_n4;
        locals.var_cfs1_dn5 = assign84420_e129265_d_n5;
        locals.var_cfs1_dn6 = assign84420_e129265_d_n6;
        locals.var_cfs1_dn7 = assign84420_e129265_d_n7;
        locals.var_cfs1_dn8 = assign84420_e129265_d_n8;
        locals.var_cfs1_dn9 = assign84420_e129265_d_n9;
        locals.var_cfs1_dn10 = assign84420_e129265_d_n10;
        locals.var_cfs1_dn13 = assign84420_e129265_d_n13;
        locals.var_cfs1_rv = 0.0;

        let (assign84430_e129271, assign84430_e129271_d_n0, assign84430_e129271_d_n2, assign84430_e129271_d_n4, assign84430_e129271_d_n5, assign84430_e129271_d_n6, assign84430_e129271_d_n7, assign84430_e129271_d_n8, assign84430_e129271_d_n9, assign84430_e129271_d_n10, assign84430_e129271_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) {
        (locals.var_ps0ld_ini__blk1890, locals.var_ps0ld_ini__blk1890_dn0, locals.var_ps0ld_ini__blk1890_dn2, locals.var_ps0ld_ini__blk1890_dn4, locals.var_ps0ld_ini__blk1890_dn5, locals.var_ps0ld_ini__blk1890_dn6, locals.var_ps0ld_ini__blk1890_dn7, locals.var_ps0ld_ini__blk1890_dn8, locals.var_ps0ld_ini__blk1890_dn9, locals.var_ps0ld_ini__blk1890_dn10, locals.var_ps0ld_ini__blk1890_dn13,)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    }
};
        locals.var_ps0ld = assign84430_e129271;
        locals.var_ps0ld_dn0 = assign84430_e129271_d_n0;
        locals.var_ps0ld_dn2 = assign84430_e129271_d_n2;
        locals.var_ps0ld_dn4 = assign84430_e129271_d_n4;
        locals.var_ps0ld_dn5 = assign84430_e129271_d_n5;
        locals.var_ps0ld_dn6 = assign84430_e129271_d_n6;
        locals.var_ps0ld_dn7 = assign84430_e129271_d_n7;
        locals.var_ps0ld_dn8 = assign84430_e129271_d_n8;
        locals.var_ps0ld_dn9 = assign84430_e129271_d_n9;
        locals.var_ps0ld_dn10 = assign84430_e129271_d_n10;
        locals.var_ps0ld_dn13 = assign84430_e129271_d_n13;
        locals.var_ps0ld_rv = 0.0;

        let (assign84440_e129277,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
        locals.var_flg_conv = assign84440_e129277;
        locals.var_flg_conv_rv = 0.0;

        let (assign84450_e129290, assign84450_e129290_d_n0, assign84450_e129290_d_n2, assign84450_e129290_d_n4, assign84450_e129290_d_n5, assign84450_e129290_d_n6, assign84450_e129290_d_n7, assign84450_e129290_d_n8, assign84450_e129290_d_n9, assign84450_e129290_d_n10, assign84450_e129290_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) {
        let assign84450_e129284: f64 = (1.034943e-10 / locals.var_q_nsubld__blk1883);
        let assign84450_e129286: f64 = (assign84450_e129284 * locals.var_beta_inv);
        let assign84450_e129287: f64 = (2.0 * assign84450_e129286);
        let assign84450_e129288: f64 = (assign84450_e129287).sqrt();
        (assign84450_e129288, ((2.0 * (assign84450_e129284 * locals.var_beta_inv_dn0)) / (2.0 * assign84450_e129288)), ((2.0 * (assign84450_e129284 * locals.var_beta_inv_dn2)) / (2.0 * assign84450_e129288)), ((2.0 * (assign84450_e129284 * locals.var_beta_inv_dn4)) / (2.0 * assign84450_e129288)), ((2.0 * (assign84450_e129284 * locals.var_beta_inv_dn5)) / (2.0 * assign84450_e129288)), ((2.0 * (assign84450_e129284 * locals.var_beta_inv_dn6)) / (2.0 * assign84450_e129288)), ((2.0 * (assign84450_e129284 * locals.var_beta_inv_dn7)) / (2.0 * assign84450_e129288)), ((2.0 * (assign84450_e129284 * locals.var_beta_inv_dn8)) / (2.0 * assign84450_e129288)), ((2.0 * (assign84450_e129284 * locals.var_beta_inv_dn9)) / (2.0 * assign84450_e129288)), ((2.0 * (assign84450_e129284 * locals.var_beta_inv_dn10)) / (2.0 * assign84450_e129288)), ((2.0 * (assign84450_e129284 * locals.var_beta_inv_dn13)) / (2.0 * assign84450_e129288)),)
    } else {
        (locals.var_c_w_ld, locals.var_c_w_ld_dn0, locals.var_c_w_ld_dn2, locals.var_c_w_ld_dn4, locals.var_c_w_ld_dn5, locals.var_c_w_ld_dn6, locals.var_c_w_ld_dn7, locals.var_c_w_ld_dn8, locals.var_c_w_ld_dn9, locals.var_c_w_ld_dn10, locals.var_c_w_ld_dn13,)
    }
};
        locals.var_c_w_ld = assign84450_e129290;
        locals.var_c_w_ld_dn0 = assign84450_e129290_d_n0;
        locals.var_c_w_ld_dn2 = assign84450_e129290_d_n2;
        locals.var_c_w_ld_dn4 = assign84450_e129290_d_n4;
        locals.var_c_w_ld_dn5 = assign84450_e129290_d_n5;
        locals.var_c_w_ld_dn6 = assign84450_e129290_d_n6;
        locals.var_c_w_ld_dn7 = assign84450_e129290_d_n7;
        locals.var_c_w_ld_dn8 = assign84450_e129290_d_n8;
        locals.var_c_w_ld_dn9 = assign84450_e129290_d_n9;
        locals.var_c_w_ld_dn10 = assign84450_e129290_d_n10;
        locals.var_c_w_ld_dn13 = assign84450_e129290_d_n13;
        locals.var_c_w_ld_rv = 0.0;

        let assign84460_e129293: f64 = if locals.var_wdep_func > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1965 = assign84460_e129293;
        locals.var_guard1965_rv = 0.0;

        let (assign84470_e129303, assign84470_e129303_d_n0, assign84470_e129303_d_n2, assign84470_e129303_d_n4, assign84470_e129303_d_n5, assign84470_e129303_d_n6, assign84470_e129303_d_n7, assign84470_e129303_d_n8, assign84470_e129303_d_n9, assign84470_e129303_d_n10, assign84470_e129303_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1965 != 0.0)) {
        let assign84470_e129301: f64 = (p.p334 - locals.var_wdep_func);
        (assign84470_e129301, (-locals.var_wdep_func_dn0), (-locals.var_wdep_func_dn2), (-locals.var_wdep_func_dn4), (-locals.var_wdep_func_dn5), (-locals.var_wdep_func_dn6), (-locals.var_wdep_func_dn7), (-locals.var_wdep_func_dn8), (-locals.var_wdep_func_dn9), (-locals.var_wdep_func_dn10), (-locals.var_wdep_func_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign84470_e129303;
        locals.var_t2_dn0 = assign84470_e129303_d_n0;
        locals.var_t2_dn2 = assign84470_e129303_d_n2;
        locals.var_t2_dn4 = assign84470_e129303_d_n4;
        locals.var_t2_dn5 = assign84470_e129303_d_n5;
        locals.var_t2_dn6 = assign84470_e129303_d_n6;
        locals.var_t2_dn7 = assign84470_e129303_d_n7;
        locals.var_t2_dn8 = assign84470_e129303_d_n8;
        locals.var_t2_dn9 = assign84470_e129303_d_n9;
        locals.var_t2_dn10 = assign84470_e129303_d_n10;
        locals.var_t2_dn13 = assign84470_e129303_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign84480_e129325, assign84480_e129325_d_n0, assign84480_e129325_d_n2, assign84480_e129325_d_n4, assign84480_e129325_d_n5, assign84480_e129325_d_n6, assign84480_e129325_d_n7, assign84480_e129325_d_n8, assign84480_e129325_d_n9, assign84480_e129325_d_n10, assign84480_e129325_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1965 == 0.0)) {
        let assign84480_e129312: f64 = (locals.var_vdsi + p.p137);
        let assign84480_e129315: f64 = (locals.var_vdsi + p.p137);
        let assign84480_e129316: f64 = (assign84480_e129312 * assign84480_e129315);
        let assign84480_e129319: f64 = (4.0 * 0.1);
        let assign84480_e129321: f64 = (assign84480_e129319 * 0.1);
        let assign84480_e129322: f64 = (assign84480_e129316 + assign84480_e129321);
        let assign84480_e129323: f64 = (assign84480_e129322).sqrt();
        (assign84480_e129323, 0.0, 0.0, 0.0, (((locals.var_vdsi_dn5 * assign84480_e129315) + (assign84480_e129312 * locals.var_vdsi_dn5)) / (2.0 * assign84480_e129323)), 0.0, (((locals.var_vdsi_dn7 * assign84480_e129315) + (assign84480_e129312 * locals.var_vdsi_dn7)) / (2.0 * assign84480_e129323)), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign84480_e129325;
        locals.var_tmf2_dn0 = assign84480_e129325_d_n0;
        locals.var_tmf2_dn2 = assign84480_e129325_d_n2;
        locals.var_tmf2_dn4 = assign84480_e129325_d_n4;
        locals.var_tmf2_dn5 = assign84480_e129325_d_n5;
        locals.var_tmf2_dn6 = assign84480_e129325_d_n6;
        locals.var_tmf2_dn7 = assign84480_e129325_d_n7;
        locals.var_tmf2_dn8 = assign84480_e129325_d_n8;
        locals.var_tmf2_dn9 = assign84480_e129325_d_n9;
        locals.var_tmf2_dn10 = assign84480_e129325_d_n10;
        locals.var_tmf2_dn13 = assign84480_e129325_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign84490_e129342, assign84490_e129342_d_n0, assign84490_e129342_d_n2, assign84490_e129342_d_n4, assign84490_e129342_d_n5, assign84490_e129342_d_n6, assign84490_e129342_d_n7, assign84490_e129342_d_n8, assign84490_e129342_d_n9, assign84490_e129342_d_n10, assign84490_e129342_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1965 == 0.0)) {
        let assign84490_e129336: f64 = (locals.var_vdsi + p.p137);
        let assign84490_e129338: f64 = (assign84490_e129336 / locals.var_tmf2);
        let assign84490_e129339: f64 = (1.0 + assign84490_e129338);
        let assign84490_e129340: f64 = (0.5 * assign84490_e129339);
        (assign84490_e129340, (0.5 * (-((assign84490_e129336 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign84490_e129336 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign84490_e129336 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (((locals.var_vdsi_dn5 * locals.var_tmf2) - (assign84490_e129336 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (-((assign84490_e129336 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (((locals.var_vdsi_dn7 * locals.var_tmf2) - (assign84490_e129336 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (-((assign84490_e129336 * locals.var_tmf2_dn8) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign84490_e129336 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign84490_e129336 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign84490_e129336 * locals.var_tmf2_dn13) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign84490_e129342;
        locals.var_t9_dn0 = assign84490_e129342_d_n0;
        locals.var_t9_dn2 = assign84490_e129342_d_n2;
        locals.var_t9_dn4 = assign84490_e129342_d_n4;
        locals.var_t9_dn5 = assign84490_e129342_d_n5;
        locals.var_t9_dn6 = assign84490_e129342_d_n6;
        locals.var_t9_dn7 = assign84490_e129342_d_n7;
        locals.var_t9_dn8 = assign84490_e129342_d_n8;
        locals.var_t9_dn9 = assign84490_e129342_d_n9;
        locals.var_t9_dn10 = assign84490_e129342_d_n10;
        locals.var_t9_dn13 = assign84490_e129342_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign84500_e129357, assign84500_e129357_d_n0, assign84500_e129357_d_n2, assign84500_e129357_d_n4, assign84500_e129357_d_n5, assign84500_e129357_d_n6, assign84500_e129357_d_n7, assign84500_e129357_d_n8, assign84500_e129357_d_n9, assign84500_e129357_d_n10, assign84500_e129357_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1965 == 0.0)) {
        let assign84500_e129352: f64 = (locals.var_vdsi + p.p137);
        let assign84500_e129354: f64 = (assign84500_e129352 + locals.var_tmf2);
        let assign84500_e129355: f64 = (0.5 * assign84500_e129354);
        (assign84500_e129355, (0.5 * locals.var_tmf2_dn0), (0.5 * locals.var_tmf2_dn2), (0.5 * locals.var_tmf2_dn4), (0.5 * (locals.var_vdsi_dn5 + locals.var_tmf2_dn5)), (0.5 * locals.var_tmf2_dn6), (0.5 * (locals.var_vdsi_dn7 + locals.var_tmf2_dn7)), (0.5 * locals.var_tmf2_dn8), (0.5 * locals.var_tmf2_dn9), (0.5 * locals.var_tmf2_dn10), (0.5 * locals.var_tmf2_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign84500_e129357;
        locals.var_t2_dn0 = assign84500_e129357_d_n0;
        locals.var_t2_dn2 = assign84500_e129357_d_n2;
        locals.var_t2_dn4 = assign84500_e129357_d_n4;
        locals.var_t2_dn5 = assign84500_e129357_d_n5;
        locals.var_t2_dn6 = assign84500_e129357_d_n6;
        locals.var_t2_dn7 = assign84500_e129357_d_n7;
        locals.var_t2_dn8 = assign84500_e129357_d_n8;
        locals.var_t2_dn9 = assign84500_e129357_d_n9;
        locals.var_t2_dn10 = assign84500_e129357_d_n10;
        locals.var_t2_dn13 = assign84500_e129357_d_n13;
        locals.var_t2_rv = 0.0;

        let assign84510_e129360: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1966 = assign84510_e129360;
        locals.var_guard1966_rv = 0.0;

        let (assign84520_e129371, assign84520_e129371_d_n0, assign84520_e129371_d_n2, assign84520_e129371_d_n4, assign84520_e129371_d_n5, assign84520_e129371_d_n6, assign84520_e129371_d_n7, assign84520_e129371_d_n8, assign84520_e129371_d_n9, assign84520_e129371_d_n10, assign84520_e129371_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1965 == 0.0)) && (locals.var_guard1966 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign84520_e129371;
        locals.var_t2_dn0 = assign84520_e129371_d_n0;
        locals.var_t2_dn2 = assign84520_e129371_d_n2;
        locals.var_t2_dn4 = assign84520_e129371_d_n4;
        locals.var_t2_dn5 = assign84520_e129371_d_n5;
        locals.var_t2_dn6 = assign84520_e129371_d_n6;
        locals.var_t2_dn7 = assign84520_e129371_d_n7;
        locals.var_t2_dn8 = assign84520_e129371_d_n8;
        locals.var_t2_dn9 = assign84520_e129371_d_n9;
        locals.var_t2_dn10 = assign84520_e129371_d_n10;
        locals.var_t2_dn13 = assign84520_e129371_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign84530_e129382, assign84530_e129382_d_n0, assign84530_e129382_d_n2, assign84530_e129382_d_n4, assign84530_e129382_d_n5, assign84530_e129382_d_n6, assign84530_e129382_d_n7, assign84530_e129382_d_n8, assign84530_e129382_d_n9, assign84530_e129382_d_n10, assign84530_e129382_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1965 == 0.0)) && (locals.var_guard1966 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign84530_e129382;
        locals.var_t9_dn0 = assign84530_e129382_d_n0;
        locals.var_t9_dn2 = assign84530_e129382_d_n2;
        locals.var_t9_dn4 = assign84530_e129382_d_n4;
        locals.var_t9_dn5 = assign84530_e129382_d_n5;
        locals.var_t9_dn6 = assign84530_e129382_d_n6;
        locals.var_t9_dn7 = assign84530_e129382_d_n7;
        locals.var_t9_dn8 = assign84530_e129382_d_n8;
        locals.var_t9_dn9 = assign84530_e129382_d_n9;
        locals.var_t9_dn10 = assign84530_e129382_d_n10;
        locals.var_t9_dn13 = assign84530_e129382_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign84540_e129396, assign84540_e129396_d_n0, assign84540_e129396_d_n2, assign84540_e129396_d_n4, assign84540_e129396_d_n5, assign84540_e129396_d_n6, assign84540_e129396_d_n7, assign84540_e129396_d_n8, assign84540_e129396_d_n9, assign84540_e129396_d_n10, assign84540_e129396_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1965 == 0.0)) {
        let assign84540_e129391: f64 = (locals.var_kjunc * locals.var_t2);
        let assign84540_e129392: f64 = (assign84540_e129391).sqrt();
        let assign84540_e129394: f64 = (assign84540_e129392 * p.p432);
        (assign84540_e129394, ((((locals.var_kjunc_dn0 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn0)) / (2.0 * assign84540_e129392)) * p.p432), ((((locals.var_kjunc_dn2 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn2)) / (2.0 * assign84540_e129392)) * p.p432), ((((locals.var_kjunc_dn4 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn4)) / (2.0 * assign84540_e129392)) * p.p432), ((((locals.var_kjunc_dn5 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn5)) / (2.0 * assign84540_e129392)) * p.p432), ((((locals.var_kjunc_dn6 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn6)) / (2.0 * assign84540_e129392)) * p.p432), ((((locals.var_kjunc_dn7 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn7)) / (2.0 * assign84540_e129392)) * p.p432), ((((locals.var_kjunc_dn8 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn8)) / (2.0 * assign84540_e129392)) * p.p432), ((((locals.var_kjunc_dn9 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn9)) / (2.0 * assign84540_e129392)) * p.p432), ((((locals.var_kjunc_dn10 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn10)) / (2.0 * assign84540_e129392)) * p.p432), ((((locals.var_kjunc_dn13 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn13)) / (2.0 * assign84540_e129392)) * p.p432),)
    } else {
        (locals.var_wjunc0, locals.var_wjunc0_dn0, locals.var_wjunc0_dn2, locals.var_wjunc0_dn4, locals.var_wjunc0_dn5, locals.var_wjunc0_dn6, locals.var_wjunc0_dn7, locals.var_wjunc0_dn8, locals.var_wjunc0_dn9, locals.var_wjunc0_dn10, locals.var_wjunc0_dn13,)
    }
};
        locals.var_wjunc0 = assign84540_e129396;
        locals.var_wjunc0_dn0 = assign84540_e129396_d_n0;
        locals.var_wjunc0_dn2 = assign84540_e129396_d_n2;
        locals.var_wjunc0_dn4 = assign84540_e129396_d_n4;
        locals.var_wjunc0_dn5 = assign84540_e129396_d_n5;
        locals.var_wjunc0_dn6 = assign84540_e129396_d_n6;
        locals.var_wjunc0_dn7 = assign84540_e129396_d_n7;
        locals.var_wjunc0_dn8 = assign84540_e129396_d_n8;
        locals.var_wjunc0_dn9 = assign84540_e129396_d_n9;
        locals.var_wjunc0_dn10 = assign84540_e129396_d_n10;
        locals.var_wjunc0_dn13 = assign84540_e129396_d_n13;
        locals.var_wjunc0_rv = 0.0;

        let (assign84550_e129407, assign84550_e129407_d_n0, assign84550_e129407_d_n2, assign84550_e129407_d_n4, assign84550_e129407_d_n5, assign84550_e129407_d_n6, assign84550_e129407_d_n7, assign84550_e129407_d_n8, assign84550_e129407_d_n9, assign84550_e129407_d_n10, assign84550_e129407_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1965 == 0.0)) {
        let assign84550_e129405: f64 = (p.p334 - locals.var_wjunc0);
        (assign84550_e129405, (-locals.var_wjunc0_dn0), (-locals.var_wjunc0_dn2), (-locals.var_wjunc0_dn4), (-locals.var_wjunc0_dn5), (-locals.var_wjunc0_dn6), (-locals.var_wjunc0_dn7), (-locals.var_wjunc0_dn8), (-locals.var_wjunc0_dn9), (-locals.var_wjunc0_dn10), (-locals.var_wjunc0_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign84550_e129407;
        locals.var_t2_dn0 = assign84550_e129407_d_n0;
        locals.var_t2_dn2 = assign84550_e129407_d_n2;
        locals.var_t2_dn4 = assign84550_e129407_d_n4;
        locals.var_t2_dn5 = assign84550_e129407_d_n5;
        locals.var_t2_dn6 = assign84550_e129407_d_n6;
        locals.var_t2_dn7 = assign84550_e129407_d_n7;
        locals.var_t2_dn8 = assign84550_e129407_d_n8;
        locals.var_t2_dn9 = assign84550_e129407_d_n9;
        locals.var_t2_dn10 = assign84550_e129407_d_n10;
        locals.var_t2_dn13 = assign84550_e129407_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign84560_e129426, assign84560_e129426_d_n0, assign84560_e129426_d_n2, assign84560_e129426_d_n4, assign84560_e129426_d_n5, assign84560_e129426_d_n6, assign84560_e129426_d_n7, assign84560_e129426_d_n8, assign84560_e129426_d_n9, assign84560_e129426_d_n10, assign84560_e129426_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) {
        let assign84560_e129413: f64 = (locals.var_t2 * locals.var_t2);
        let assign84560_e129417: f64 = (p.p334 * 0.01);
        let assign84560_e129418: f64 = (4.0 * assign84560_e129417);
        let assign84560_e129421: f64 = (p.p334 * 0.01);
        let assign84560_e129422: f64 = (assign84560_e129418 * assign84560_e129421);
        let assign84560_e129423: f64 = (assign84560_e129413 + assign84560_e129422);
        let assign84560_e129424: f64 = (assign84560_e129423).sqrt();
        (assign84560_e129424, (((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)) / (2.0 * assign84560_e129424)), (((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)) / (2.0 * assign84560_e129424)), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign84560_e129424)), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign84560_e129424)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign84560_e129424)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign84560_e129424)), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign84560_e129424)), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign84560_e129424)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign84560_e129424)), (((locals.var_t2_dn13 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn13)) / (2.0 * assign84560_e129424)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign84560_e129426;
        locals.var_tmf2_dn0 = assign84560_e129426_d_n0;
        locals.var_tmf2_dn2 = assign84560_e129426_d_n2;
        locals.var_tmf2_dn4 = assign84560_e129426_d_n4;
        locals.var_tmf2_dn5 = assign84560_e129426_d_n5;
        locals.var_tmf2_dn6 = assign84560_e129426_d_n6;
        locals.var_tmf2_dn7 = assign84560_e129426_d_n7;
        locals.var_tmf2_dn8 = assign84560_e129426_d_n8;
        locals.var_tmf2_dn9 = assign84560_e129426_d_n9;
        locals.var_tmf2_dn10 = assign84560_e129426_d_n10;
        locals.var_tmf2_dn13 = assign84560_e129426_d_n13;
        locals.var_tmf2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_312(
        locals: &mut StampLocals,
    ) {
        let (assign84570_e129438, assign84570_e129438_d_n0, assign84570_e129438_d_n2, assign84570_e129438_d_n4, assign84570_e129438_d_n5, assign84570_e129438_d_n6, assign84570_e129438_d_n7, assign84570_e129438_d_n8, assign84570_e129438_d_n9, assign84570_e129438_d_n10, assign84570_e129438_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) {
        let assign84570_e129434: f64 = (locals.var_t2 / locals.var_tmf2);
        let assign84570_e129435: f64 = (1.0 + assign84570_e129434);
        let assign84570_e129436: f64 = (0.5 * assign84570_e129435);
        (assign84570_e129436, (0.5 * (((locals.var_t2_dn0 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn2 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn4 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn5 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn6 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn7 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn8 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn9 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn10 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn13 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign84570_e129438;
        locals.var_t9_dn0 = assign84570_e129438_d_n0;
        locals.var_t9_dn2 = assign84570_e129438_d_n2;
        locals.var_t9_dn4 = assign84570_e129438_d_n4;
        locals.var_t9_dn5 = assign84570_e129438_d_n5;
        locals.var_t9_dn6 = assign84570_e129438_d_n6;
        locals.var_t9_dn7 = assign84570_e129438_d_n7;
        locals.var_t9_dn8 = assign84570_e129438_d_n8;
        locals.var_t9_dn9 = assign84570_e129438_d_n9;
        locals.var_t9_dn10 = assign84570_e129438_d_n10;
        locals.var_t9_dn13 = assign84570_e129438_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign84580_e129448, assign84580_e129448_d_n0, assign84580_e129448_d_n2, assign84580_e129448_d_n4, assign84580_e129448_d_n5, assign84580_e129448_d_n6, assign84580_e129448_d_n7, assign84580_e129448_d_n8, assign84580_e129448_d_n9, assign84580_e129448_d_n10, assign84580_e129448_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) {
        let assign84580_e129445: f64 = (locals.var_t2 + locals.var_tmf2);
        let assign84580_e129446: f64 = (0.5 * assign84580_e129445);
        (assign84580_e129446, (0.5 * (locals.var_t2_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t2_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t2_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t2_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t2_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t2_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t2_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t2_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t2_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t2_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign84580_e129448;
        locals.var_t2_dn0 = assign84580_e129448_d_n0;
        locals.var_t2_dn2 = assign84580_e129448_d_n2;
        locals.var_t2_dn4 = assign84580_e129448_d_n4;
        locals.var_t2_dn5 = assign84580_e129448_d_n5;
        locals.var_t2_dn6 = assign84580_e129448_d_n6;
        locals.var_t2_dn7 = assign84580_e129448_d_n7;
        locals.var_t2_dn8 = assign84580_e129448_d_n8;
        locals.var_t2_dn9 = assign84580_e129448_d_n9;
        locals.var_t2_dn10 = assign84580_e129448_d_n10;
        locals.var_t2_dn13 = assign84580_e129448_d_n13;
        locals.var_t2_rv = 0.0;

        let assign84590_e129451: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1967 = assign84590_e129451;
        locals.var_guard1967_rv = 0.0;

        let (assign84600_e129459, assign84600_e129459_d_n0, assign84600_e129459_d_n2, assign84600_e129459_d_n4, assign84600_e129459_d_n5, assign84600_e129459_d_n6, assign84600_e129459_d_n7, assign84600_e129459_d_n8, assign84600_e129459_d_n9, assign84600_e129459_d_n10, assign84600_e129459_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1967 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign84600_e129459;
        locals.var_t2_dn0 = assign84600_e129459_d_n0;
        locals.var_t2_dn2 = assign84600_e129459_d_n2;
        locals.var_t2_dn4 = assign84600_e129459_d_n4;
        locals.var_t2_dn5 = assign84600_e129459_d_n5;
        locals.var_t2_dn6 = assign84600_e129459_d_n6;
        locals.var_t2_dn7 = assign84600_e129459_d_n7;
        locals.var_t2_dn8 = assign84600_e129459_d_n8;
        locals.var_t2_dn9 = assign84600_e129459_d_n9;
        locals.var_t2_dn10 = assign84600_e129459_d_n10;
        locals.var_t2_dn13 = assign84600_e129459_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign84610_e129467, assign84610_e129467_d_n0, assign84610_e129467_d_n2, assign84610_e129467_d_n4, assign84610_e129467_d_n5, assign84610_e129467_d_n6, assign84610_e129467_d_n7, assign84610_e129467_d_n8, assign84610_e129467_d_n9, assign84610_e129467_d_n10, assign84610_e129467_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1967 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign84610_e129467;
        locals.var_t9_dn0 = assign84610_e129467_d_n0;
        locals.var_t9_dn2 = assign84610_e129467_d_n2;
        locals.var_t9_dn4 = assign84610_e129467_d_n4;
        locals.var_t9_dn5 = assign84610_e129467_d_n5;
        locals.var_t9_dn6 = assign84610_e129467_d_n6;
        locals.var_t9_dn7 = assign84610_e129467_d_n7;
        locals.var_t9_dn8 = assign84610_e129467_d_n8;
        locals.var_t9_dn9 = assign84610_e129467_d_n9;
        locals.var_t9_dn10 = assign84610_e129467_d_n10;
        locals.var_t9_dn13 = assign84610_e129467_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign84620_e129473, assign84620_e129473_d_n0, assign84620_e129473_d_n2, assign84620_e129473_d_n4, assign84620_e129473_d_n5, assign84620_e129473_d_n6, assign84620_e129473_d_n7, assign84620_e129473_d_n8, assign84620_e129473_d_n9, assign84620_e129473_d_n10, assign84620_e129473_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    } else {
        (locals.var_ddriftldc, locals.var_ddriftldc_dn0, locals.var_ddriftldc_dn2, locals.var_ddriftldc_dn4, locals.var_ddriftldc_dn5, locals.var_ddriftldc_dn6, locals.var_ddriftldc_dn7, locals.var_ddriftldc_dn8, locals.var_ddriftldc_dn9, locals.var_ddriftldc_dn10, locals.var_ddriftldc_dn13,)
    }
};
        locals.var_ddriftldc = assign84620_e129473;
        locals.var_ddriftldc_dn0 = assign84620_e129473_d_n0;
        locals.var_ddriftldc_dn2 = assign84620_e129473_d_n2;
        locals.var_ddriftldc_dn4 = assign84620_e129473_d_n4;
        locals.var_ddriftldc_dn5 = assign84620_e129473_d_n5;
        locals.var_ddriftldc_dn6 = assign84620_e129473_d_n6;
        locals.var_ddriftldc_dn7 = assign84620_e129473_d_n7;
        locals.var_ddriftldc_dn8 = assign84620_e129473_d_n8;
        locals.var_ddriftldc_dn9 = assign84620_e129473_d_n9;
        locals.var_ddriftldc_dn10 = assign84620_e129473_d_n10;
        locals.var_ddriftldc_dn13 = assign84620_e129473_d_n13;
        locals.var_ddriftldc_rv = 0.0;

        let (assign84630_e129487, assign84630_e129487_d_n0, assign84630_e129487_d_n2, assign84630_e129487_d_n4, assign84630_e129487_d_n5, assign84630_e129487_d_n6, assign84630_e129487_d_n7, assign84630_e129487_d_n8, assign84630_e129487_d_n9, assign84630_e129487_d_n10, assign84630_e129487_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) {
        let assign84630_e129479: f64 = (locals.var_q_nsubld__blk1883 * locals.var_ddriftldc);
        let assign84630_e129481: f64 = (assign84630_e129479 * locals.var_ddriftldc);
        let assign84630_e129483: f64 = (assign84630_e129481 / 2.0);
        let assign84630_e129485: f64 = (assign84630_e129483 / 1.034943e-10);
        (assign84630_e129485, (((((locals.var_q_nsubld__blk1883 * locals.var_ddriftldc_dn0) * locals.var_ddriftldc) + (assign84630_e129479 * locals.var_ddriftldc_dn0)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1883 * locals.var_ddriftldc_dn2) * locals.var_ddriftldc) + (assign84630_e129479 * locals.var_ddriftldc_dn2)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1883 * locals.var_ddriftldc_dn4) * locals.var_ddriftldc) + (assign84630_e129479 * locals.var_ddriftldc_dn4)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1883 * locals.var_ddriftldc_dn5) * locals.var_ddriftldc) + (assign84630_e129479 * locals.var_ddriftldc_dn5)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1883 * locals.var_ddriftldc_dn6) * locals.var_ddriftldc) + (assign84630_e129479 * locals.var_ddriftldc_dn6)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1883 * locals.var_ddriftldc_dn7) * locals.var_ddriftldc) + (assign84630_e129479 * locals.var_ddriftldc_dn7)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1883 * locals.var_ddriftldc_dn8) * locals.var_ddriftldc) + (assign84630_e129479 * locals.var_ddriftldc_dn8)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1883 * locals.var_ddriftldc_dn9) * locals.var_ddriftldc) + (assign84630_e129479 * locals.var_ddriftldc_dn9)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1883 * locals.var_ddriftldc_dn10) * locals.var_ddriftldc) + (assign84630_e129479 * locals.var_ddriftldc_dn10)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1883 * locals.var_ddriftldc_dn13) * locals.var_ddriftldc) + (assign84630_e129479 * locals.var_ddriftldc_dn13)) / 2.0) / 1.034943e-10),)
    } else {
        (locals.var_dphi_sb, locals.var_dphi_sb_dn0, locals.var_dphi_sb_dn2, locals.var_dphi_sb_dn4, locals.var_dphi_sb_dn5, locals.var_dphi_sb_dn6, locals.var_dphi_sb_dn7, locals.var_dphi_sb_dn8, locals.var_dphi_sb_dn9, locals.var_dphi_sb_dn10, locals.var_dphi_sb_dn13,)
    }
};
        locals.var_dphi_sb = assign84630_e129487;
        locals.var_dphi_sb_dn0 = assign84630_e129487_d_n0;
        locals.var_dphi_sb_dn2 = assign84630_e129487_d_n2;
        locals.var_dphi_sb_dn4 = assign84630_e129487_d_n4;
        locals.var_dphi_sb_dn5 = assign84630_e129487_d_n5;
        locals.var_dphi_sb_dn6 = assign84630_e129487_d_n6;
        locals.var_dphi_sb_dn7 = assign84630_e129487_d_n7;
        locals.var_dphi_sb_dn8 = assign84630_e129487_d_n8;
        locals.var_dphi_sb_dn9 = assign84630_e129487_d_n9;
        locals.var_dphi_sb_dn10 = assign84630_e129487_d_n10;
        locals.var_dphi_sb_dn13 = assign84630_e129487_d_n13;
        locals.var_dphi_sb_rv = 0.0;

        let (assign84640_e129498, assign84640_e129498_d_n0, assign84640_e129498_d_n2, assign84640_e129498_d_n4, assign84640_e129498_d_n5, assign84640_e129498_d_n6, assign84640_e129498_d_n7, assign84640_e129498_d_n8, assign84640_e129498_d_n9, assign84640_e129498_d_n10, assign84640_e129498_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) {
        let assign84640_e129493: f64 = (2.0 * locals.var_beta);
        let assign84640_e129495: f64 = (assign84640_e129493 * locals.var_dphi_sb);
        let assign84640_e129496: f64 = (assign84640_e129495).sqrt();
        (assign84640_e129496, ((((2.0 * locals.var_beta_dn0) * locals.var_dphi_sb) + (assign84640_e129493 * locals.var_dphi_sb_dn0)) / (2.0 * assign84640_e129496)), ((((2.0 * locals.var_beta_dn2) * locals.var_dphi_sb) + (assign84640_e129493 * locals.var_dphi_sb_dn2)) / (2.0 * assign84640_e129496)), ((((2.0 * locals.var_beta_dn4) * locals.var_dphi_sb) + (assign84640_e129493 * locals.var_dphi_sb_dn4)) / (2.0 * assign84640_e129496)), ((((2.0 * locals.var_beta_dn5) * locals.var_dphi_sb) + (assign84640_e129493 * locals.var_dphi_sb_dn5)) / (2.0 * assign84640_e129496)), ((((2.0 * locals.var_beta_dn6) * locals.var_dphi_sb) + (assign84640_e129493 * locals.var_dphi_sb_dn6)) / (2.0 * assign84640_e129496)), ((((2.0 * locals.var_beta_dn7) * locals.var_dphi_sb) + (assign84640_e129493 * locals.var_dphi_sb_dn7)) / (2.0 * assign84640_e129496)), ((((2.0 * locals.var_beta_dn8) * locals.var_dphi_sb) + (assign84640_e129493 * locals.var_dphi_sb_dn8)) / (2.0 * assign84640_e129496)), ((((2.0 * locals.var_beta_dn9) * locals.var_dphi_sb) + (assign84640_e129493 * locals.var_dphi_sb_dn9)) / (2.0 * assign84640_e129496)), ((((2.0 * locals.var_beta_dn10) * locals.var_dphi_sb) + (assign84640_e129493 * locals.var_dphi_sb_dn10)) / (2.0 * assign84640_e129496)), ((((2.0 * locals.var_beta_dn13) * locals.var_dphi_sb) + (assign84640_e129493 * locals.var_dphi_sb_dn13)) / (2.0 * assign84640_e129496)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign84640_e129498;
        locals.var_t0_dn0 = assign84640_e129498_d_n0;
        locals.var_t0_dn2 = assign84640_e129498_d_n2;
        locals.var_t0_dn4 = assign84640_e129498_d_n4;
        locals.var_t0_dn5 = assign84640_e129498_d_n5;
        locals.var_t0_dn6 = assign84640_e129498_d_n6;
        locals.var_t0_dn7 = assign84640_e129498_d_n7;
        locals.var_t0_dn8 = assign84640_e129498_d_n8;
        locals.var_t0_dn9 = assign84640_e129498_d_n9;
        locals.var_t0_dn10 = assign84640_e129498_d_n10;
        locals.var_t0_dn13 = assign84640_e129498_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign84650_e129511, assign84650_e129511_d_n0, assign84650_e129511_d_n2, assign84650_e129511_d_n4, assign84650_e129511_d_n5, assign84650_e129511_d_n6, assign84650_e129511_d_n7, assign84650_e129511_d_n8, assign84650_e129511_d_n9, assign84650_e129511_d_n10, assign84650_e129511_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) {
        let assign84650_e129503: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign84650_e129505: f64 = (-locals.var_t0);
        let assign84650_e129506: f64 = { let limited_exp_arg = assign84650_e129505; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign84650_e129507: f64 = (assign84650_e129503 + assign84650_e129506);
        let assign84650_e129509: f64 = (assign84650_e129507 / 2.0);
        (assign84650_e129509, ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn0) + ({ let limited_exp_arg = assign84650_e129505; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn0))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn2) + ({ let limited_exp_arg = assign84650_e129505; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn2))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4) + ({ let limited_exp_arg = assign84650_e129505; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn4))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5) + ({ let limited_exp_arg = assign84650_e129505; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn5))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6) + ({ let limited_exp_arg = assign84650_e129505; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn6))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7) + ({ let limited_exp_arg = assign84650_e129505; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn7))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8) + ({ let limited_exp_arg = assign84650_e129505; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn8))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9) + ({ let limited_exp_arg = assign84650_e129505; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn9))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10) + ({ let limited_exp_arg = assign84650_e129505; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn10))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn13) + ({ let limited_exp_arg = assign84650_e129505; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn13))) / 2.0),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign84650_e129511;
        locals.var_t1_dn0 = assign84650_e129511_d_n0;
        locals.var_t1_dn2 = assign84650_e129511_d_n2;
        locals.var_t1_dn4 = assign84650_e129511_d_n4;
        locals.var_t1_dn5 = assign84650_e129511_d_n5;
        locals.var_t1_dn6 = assign84650_e129511_d_n6;
        locals.var_t1_dn7 = assign84650_e129511_d_n7;
        locals.var_t1_dn8 = assign84650_e129511_d_n8;
        locals.var_t1_dn9 = assign84650_e129511_d_n9;
        locals.var_t1_dn10 = assign84650_e129511_d_n10;
        locals.var_t1_dn13 = assign84650_e129511_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign84660_e129520, assign84660_e129520_d_n0, assign84660_e129520_d_n2, assign84660_e129520_d_n4, assign84660_e129520_d_n5, assign84660_e129520_d_n6, assign84660_e129520_d_n7, assign84660_e129520_d_n8, assign84660_e129520_d_n9, assign84660_e129520_d_n10, assign84660_e129520_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) {
        let assign84660_e129516: f64 = (locals.var_t1).ln();
        let assign84660_e129518: f64 = (assign84660_e129516 / locals.var_dphi_sb);
        (assign84660_e129518, ((((locals.var_t1_dn0 / locals.var_t1) * locals.var_dphi_sb) - (assign84660_e129516 * locals.var_dphi_sb_dn0)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn2 / locals.var_t1) * locals.var_dphi_sb) - (assign84660_e129516 * locals.var_dphi_sb_dn2)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn4 / locals.var_t1) * locals.var_dphi_sb) - (assign84660_e129516 * locals.var_dphi_sb_dn4)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn5 / locals.var_t1) * locals.var_dphi_sb) - (assign84660_e129516 * locals.var_dphi_sb_dn5)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn6 / locals.var_t1) * locals.var_dphi_sb) - (assign84660_e129516 * locals.var_dphi_sb_dn6)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn7 / locals.var_t1) * locals.var_dphi_sb) - (assign84660_e129516 * locals.var_dphi_sb_dn7)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn8 / locals.var_t1) * locals.var_dphi_sb) - (assign84660_e129516 * locals.var_dphi_sb_dn8)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn9 / locals.var_t1) * locals.var_dphi_sb) - (assign84660_e129516 * locals.var_dphi_sb_dn9)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn10 / locals.var_t1) * locals.var_dphi_sb) - (assign84660_e129516 * locals.var_dphi_sb_dn10)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn13 / locals.var_t1) * locals.var_dphi_sb) - (assign84660_e129516 * locals.var_dphi_sb_dn13)) / (locals.var_dphi_sb * locals.var_dphi_sb)),)
    } else {
        (locals.var_c_sb, locals.var_c_sb_dn0, locals.var_c_sb_dn2, locals.var_c_sb_dn4, locals.var_c_sb_dn5, locals.var_c_sb_dn6, locals.var_c_sb_dn7, locals.var_c_sb_dn8, locals.var_c_sb_dn9, locals.var_c_sb_dn10, locals.var_c_sb_dn13,)
    }
};
        locals.var_c_sb = assign84660_e129520;
        locals.var_c_sb_dn0 = assign84660_e129520_d_n0;
        locals.var_c_sb_dn2 = assign84660_e129520_d_n2;
        locals.var_c_sb_dn4 = assign84660_e129520_d_n4;
        locals.var_c_sb_dn5 = assign84660_e129520_d_n5;
        locals.var_c_sb_dn6 = assign84660_e129520_d_n6;
        locals.var_c_sb_dn7 = assign84660_e129520_d_n7;
        locals.var_c_sb_dn8 = assign84660_e129520_d_n8;
        locals.var_c_sb_dn9 = assign84660_e129520_d_n9;
        locals.var_c_sb_dn10 = assign84660_e129520_d_n10;
        locals.var_c_sb_dn13 = assign84660_e129520_d_n13;
        locals.var_c_sb_rv = 0.0;

        let (assign84670_e129526,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign84670_e129526;
        locals.var_lp_s0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_313(
        locals: &mut StampLocals,
    ) {
        let mut assign84680_loop_guard: usize = 0;
        while {
            let assign84680_cond_e129533: f64 = (locals.var_lp_s0_max + 1.0);
            let assign84680_cond_e129535: f64 = if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_lp_s0 <= assign84680_cond_e129533)) { 1.0 } else { 0.0 };
            assign84680_cond_e129535 != 0.0
        } {
            assign84680_loop_guard += 1;
            assert!(assign84680_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign84680_body3_e129562, assign84680_body3_e129562_d_n0, assign84680_body3_e129562_d_n2, assign84680_body3_e129562_d_n4, assign84680_body3_e129562_d_n5, assign84680_body3_e129562_d_n6, assign84680_body3_e129562_d_n7, assign84680_body3_e129562_d_n8, assign84680_body3_e129562_d_n9, assign84680_body3_e129562_d_n10, assign84680_body3_e129562_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) {
        let assign84680_body3_e129560: f64 = (locals.var_ps0ld + locals.var_vxbgmtcl);
        (assign84680_body3_e129560, (locals.var_ps0ld_dn0 + locals.var_vxbgmtcl_dn0), (locals.var_ps0ld_dn2 + locals.var_vxbgmtcl_dn2), (locals.var_ps0ld_dn4 + locals.var_vxbgmtcl_dn4), (locals.var_ps0ld_dn5 + locals.var_vxbgmtcl_dn5), (locals.var_ps0ld_dn6 + locals.var_vxbgmtcl_dn6), (locals.var_ps0ld_dn7 + locals.var_vxbgmtcl_dn7), (locals.var_ps0ld_dn8 + locals.var_vxbgmtcl_dn8), (locals.var_ps0ld_dn9 + locals.var_vxbgmtcl_dn9), (locals.var_ps0ld_dn10 + locals.var_vxbgmtcl_dn10), (locals.var_ps0ld_dn13 + locals.var_vxbgmtcl_dn13),)
    } else {
        (locals.var_ps0ld_vxb, locals.var_ps0ld_vxb_dn0, locals.var_ps0ld_vxb_dn2, locals.var_ps0ld_vxb_dn4, locals.var_ps0ld_vxb_dn5, locals.var_ps0ld_vxb_dn6, locals.var_ps0ld_vxb_dn7, locals.var_ps0ld_vxb_dn8, locals.var_ps0ld_vxb_dn9, locals.var_ps0ld_vxb_dn10, locals.var_ps0ld_vxb_dn13,)
    }
};
            locals.var_ps0ld_vxb = assign84680_body3_e129562;
            locals.var_ps0ld_vxb_dn0 = assign84680_body3_e129562_d_n0;
            locals.var_ps0ld_vxb_dn2 = assign84680_body3_e129562_d_n2;
            locals.var_ps0ld_vxb_dn4 = assign84680_body3_e129562_d_n4;
            locals.var_ps0ld_vxb_dn5 = assign84680_body3_e129562_d_n5;
            locals.var_ps0ld_vxb_dn6 = assign84680_body3_e129562_d_n6;
            locals.var_ps0ld_vxb_dn7 = assign84680_body3_e129562_d_n7;
            locals.var_ps0ld_vxb_dn8 = assign84680_body3_e129562_d_n8;
            locals.var_ps0ld_vxb_dn9 = assign84680_body3_e129562_d_n9;
            locals.var_ps0ld_vxb_dn10 = assign84680_body3_e129562_d_n10;
            locals.var_ps0ld_vxb_dn13 = assign84680_body3_e129562_d_n13;
            locals.var_ps0ld_vxb_rv = 0.0;
            let (assign84680_body4_e129570, assign84680_body4_e129570_d_n0, assign84680_body4_e129570_d_n2, assign84680_body4_e129570_d_n4, assign84680_body4_e129570_d_n5, assign84680_body4_e129570_d_n6, assign84680_body4_e129570_d_n7, assign84680_body4_e129570_d_n8, assign84680_body4_e129570_d_n9, assign84680_body4_e129570_d_n10, assign84680_body4_e129570_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) {
        let assign84680_body4_e129568: f64 = (locals.var_beta * locals.var_ps0ld_vxb);
        (assign84680_body4_e129568, ((locals.var_beta_dn0 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn0)), ((locals.var_beta_dn2 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn2)), ((locals.var_beta_dn4 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn4)), ((locals.var_beta_dn5 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn5)), ((locals.var_beta_dn6 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn6)), ((locals.var_beta_dn7 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn7)), ((locals.var_beta_dn8 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn8)), ((locals.var_beta_dn9 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn9)), ((locals.var_beta_dn10 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn10)), ((locals.var_beta_dn13 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn13)),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    }
};
            locals.var_chi = assign84680_body4_e129570;
            locals.var_chi_dn0 = assign84680_body4_e129570_d_n0;
            locals.var_chi_dn2 = assign84680_body4_e129570_d_n2;
            locals.var_chi_dn4 = assign84680_body4_e129570_d_n4;
            locals.var_chi_dn5 = assign84680_body4_e129570_d_n5;
            locals.var_chi_dn6 = assign84680_body4_e129570_d_n6;
            locals.var_chi_dn7 = assign84680_body4_e129570_d_n7;
            locals.var_chi_dn8 = assign84680_body4_e129570_d_n8;
            locals.var_chi_dn9 = assign84680_body4_e129570_d_n9;
            locals.var_chi_dn10 = assign84680_body4_e129570_d_n10;
            locals.var_chi_dn13 = assign84680_body4_e129570_d_n13;
            locals.var_chi_rv = 0.0;
            let (assign84680_body5_e129580, assign84680_body5_e129580_d_n0, assign84680_body5_e129580_d_n2, assign84680_body5_e129580_d_n4, assign84680_body5_e129580_d_n5, assign84680_body5_e129580_d_n6, assign84680_body5_e129580_d_n7, assign84680_body5_e129580_d_n8, assign84680_body5_e129580_d_n9, assign84680_body5_e129580_d_n10, assign84680_body5_e129580_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) {
        let assign84680_body5_e129577: f64 = (locals.var_ps0ld_vxb - locals.var_dphi_sb);
        let assign84680_body5_e129578: f64 = (locals.var_c_sb * assign84680_body5_e129577);
        (assign84680_body5_e129578, ((locals.var_c_sb_dn0 * assign84680_body5_e129577) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn0 - locals.var_dphi_sb_dn0))), ((locals.var_c_sb_dn2 * assign84680_body5_e129577) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn2 - locals.var_dphi_sb_dn2))), ((locals.var_c_sb_dn4 * assign84680_body5_e129577) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn4 - locals.var_dphi_sb_dn4))), ((locals.var_c_sb_dn5 * assign84680_body5_e129577) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn5 - locals.var_dphi_sb_dn5))), ((locals.var_c_sb_dn6 * assign84680_body5_e129577) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn6 - locals.var_dphi_sb_dn6))), ((locals.var_c_sb_dn7 * assign84680_body5_e129577) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn7 - locals.var_dphi_sb_dn7))), ((locals.var_c_sb_dn8 * assign84680_body5_e129577) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn8 - locals.var_dphi_sb_dn8))), ((locals.var_c_sb_dn9 * assign84680_body5_e129577) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn9 - locals.var_dphi_sb_dn9))), ((locals.var_c_sb_dn10 * assign84680_body5_e129577) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn10 - locals.var_dphi_sb_dn10))), ((locals.var_c_sb_dn13 * assign84680_body5_e129577) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn13 - locals.var_dphi_sb_dn13))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn13,)
    }
};
            locals.var_ty = assign84680_body5_e129580;
            locals.var_ty_dn0 = assign84680_body5_e129580_d_n0;
            locals.var_ty_dn2 = assign84680_body5_e129580_d_n2;
            locals.var_ty_dn4 = assign84680_body5_e129580_d_n4;
            locals.var_ty_dn5 = assign84680_body5_e129580_d_n5;
            locals.var_ty_dn6 = assign84680_body5_e129580_d_n6;
            locals.var_ty_dn7 = assign84680_body5_e129580_d_n7;
            locals.var_ty_dn8 = assign84680_body5_e129580_d_n8;
            locals.var_ty_dn9 = assign84680_body5_e129580_d_n9;
            locals.var_ty_dn10 = assign84680_body5_e129580_d_n10;
            locals.var_ty_dn13 = assign84680_body5_e129580_d_n13;
            locals.var_ty_rv = 0.0;
            let assign84680_body6_e129583: f64 = if locals.var_ty < 60.0 { 1.0 } else { 0.0 };
            locals.var_guard1969 = assign84680_body6_e129583;
            locals.var_guard1969_rv = 0.0;
            let (assign84680_body7_e129592, assign84680_body7_e129592_d_n0, assign84680_body7_e129592_d_n2, assign84680_body7_e129592_d_n4, assign84680_body7_e129592_d_n5, assign84680_body7_e129592_d_n6, assign84680_body7_e129592_d_n7, assign84680_body7_e129592_d_n8, assign84680_body7_e129592_d_n9, assign84680_body7_e129592_d_n10, assign84680_body7_e129592_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1969 != 0.0)) {
        let assign84680_body7_e129590: f64 = (locals.var_ty).exp();
        (assign84680_body7_e129590, (assign84680_body7_e129590 * locals.var_ty_dn0), (assign84680_body7_e129590 * locals.var_ty_dn2), (assign84680_body7_e129590 * locals.var_ty_dn4), (assign84680_body7_e129590 * locals.var_ty_dn5), (assign84680_body7_e129590 * locals.var_ty_dn6), (assign84680_body7_e129590 * locals.var_ty_dn7), (assign84680_body7_e129590 * locals.var_ty_dn8), (assign84680_body7_e129590 * locals.var_ty_dn9), (assign84680_body7_e129590 * locals.var_ty_dn10), (assign84680_body7_e129590 * locals.var_ty_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign84680_body7_e129592;
            locals.var_t1_dn0 = assign84680_body7_e129592_d_n0;
            locals.var_t1_dn2 = assign84680_body7_e129592_d_n2;
            locals.var_t1_dn4 = assign84680_body7_e129592_d_n4;
            locals.var_t1_dn5 = assign84680_body7_e129592_d_n5;
            locals.var_t1_dn6 = assign84680_body7_e129592_d_n6;
            locals.var_t1_dn7 = assign84680_body7_e129592_d_n7;
            locals.var_t1_dn8 = assign84680_body7_e129592_d_n8;
            locals.var_t1_dn9 = assign84680_body7_e129592_d_n9;
            locals.var_t1_dn10 = assign84680_body7_e129592_d_n10;
            locals.var_t1_dn13 = assign84680_body7_e129592_d_n13;
            locals.var_t1_rv = 0.0;
            let (assign84680_body8_e129604, assign84680_body8_e129604_d_n0, assign84680_body8_e129604_d_n2, assign84680_body8_e129604_d_n4, assign84680_body8_e129604_d_n5, assign84680_body8_e129604_d_n6, assign84680_body8_e129604_d_n7, assign84680_body8_e129604_d_n8, assign84680_body8_e129604_d_n9, assign84680_body8_e129604_d_n10, assign84680_body8_e129604_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1969 != 0.0)) {
        let assign84680_body8_e129599: f64 = (-locals.var_c_sb);
        let assign84680_body8_e129601: f64 = (assign84680_body8_e129599 * locals.var_dphi_sb);
        let assign84680_body8_e129602: f64 = (assign84680_body8_e129601).exp();
        (assign84680_body8_e129602, (assign84680_body8_e129602 * (((-locals.var_c_sb_dn0) * locals.var_dphi_sb) + (assign84680_body8_e129599 * locals.var_dphi_sb_dn0))), (assign84680_body8_e129602 * (((-locals.var_c_sb_dn2) * locals.var_dphi_sb) + (assign84680_body8_e129599 * locals.var_dphi_sb_dn2))), (assign84680_body8_e129602 * (((-locals.var_c_sb_dn4) * locals.var_dphi_sb) + (assign84680_body8_e129599 * locals.var_dphi_sb_dn4))), (assign84680_body8_e129602 * (((-locals.var_c_sb_dn5) * locals.var_dphi_sb) + (assign84680_body8_e129599 * locals.var_dphi_sb_dn5))), (assign84680_body8_e129602 * (((-locals.var_c_sb_dn6) * locals.var_dphi_sb) + (assign84680_body8_e129599 * locals.var_dphi_sb_dn6))), (assign84680_body8_e129602 * (((-locals.var_c_sb_dn7) * locals.var_dphi_sb) + (assign84680_body8_e129599 * locals.var_dphi_sb_dn7))), (assign84680_body8_e129602 * (((-locals.var_c_sb_dn8) * locals.var_dphi_sb) + (assign84680_body8_e129599 * locals.var_dphi_sb_dn8))), (assign84680_body8_e129602 * (((-locals.var_c_sb_dn9) * locals.var_dphi_sb) + (assign84680_body8_e129599 * locals.var_dphi_sb_dn9))), (assign84680_body8_e129602 * (((-locals.var_c_sb_dn10) * locals.var_dphi_sb) + (assign84680_body8_e129599 * locals.var_dphi_sb_dn10))), (assign84680_body8_e129602 * (((-locals.var_c_sb_dn13) * locals.var_dphi_sb) + (assign84680_body8_e129599 * locals.var_dphi_sb_dn13))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
            locals.var_t0 = assign84680_body8_e129604;
            locals.var_t0_dn0 = assign84680_body8_e129604_d_n0;
            locals.var_t0_dn2 = assign84680_body8_e129604_d_n2;
            locals.var_t0_dn4 = assign84680_body8_e129604_d_n4;
            locals.var_t0_dn5 = assign84680_body8_e129604_d_n5;
            locals.var_t0_dn6 = assign84680_body8_e129604_d_n6;
            locals.var_t0_dn7 = assign84680_body8_e129604_d_n7;
            locals.var_t0_dn8 = assign84680_body8_e129604_d_n8;
            locals.var_t0_dn9 = assign84680_body8_e129604_d_n9;
            locals.var_t0_dn10 = assign84680_body8_e129604_d_n10;
            locals.var_t0_dn13 = assign84680_body8_e129604_d_n13;
            locals.var_t0_rv = 0.0;
            let (assign84680_body9_e129614, assign84680_body9_e129614_d_n0, assign84680_body9_e129614_d_n2, assign84680_body9_e129614_d_n4, assign84680_body9_e129614_d_n5, assign84680_body9_e129614_d_n6, assign84680_body9_e129614_d_n7, assign84680_body9_e129614_d_n8, assign84680_body9_e129614_d_n9, assign84680_body9_e129614_d_n10, assign84680_body9_e129614_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1969 != 0.0)) {
        let assign84680_body9_e129612: f64 = (locals.var_t1 - locals.var_t0);
        (assign84680_body9_e129612, (locals.var_t1_dn0 - locals.var_t0_dn0), (locals.var_t1_dn2 - locals.var_t0_dn2), (locals.var_t1_dn4 - locals.var_t0_dn4), (locals.var_t1_dn5 - locals.var_t0_dn5), (locals.var_t1_dn6 - locals.var_t0_dn6), (locals.var_t1_dn7 - locals.var_t0_dn7), (locals.var_t1_dn8 - locals.var_t0_dn8), (locals.var_t1_dn9 - locals.var_t0_dn9), (locals.var_t1_dn10 - locals.var_t0_dn10), (locals.var_t1_dn13 - locals.var_t0_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
            locals.var_t2 = assign84680_body9_e129614;
            locals.var_t2_dn0 = assign84680_body9_e129614_d_n0;
            locals.var_t2_dn2 = assign84680_body9_e129614_d_n2;
            locals.var_t2_dn4 = assign84680_body9_e129614_d_n4;
            locals.var_t2_dn5 = assign84680_body9_e129614_d_n5;
            locals.var_t2_dn6 = assign84680_body9_e129614_d_n6;
            locals.var_t2_dn7 = assign84680_body9_e129614_d_n7;
            locals.var_t2_dn8 = assign84680_body9_e129614_d_n8;
            locals.var_t2_dn9 = assign84680_body9_e129614_d_n9;
            locals.var_t2_dn10 = assign84680_body9_e129614_d_n10;
            locals.var_t2_dn13 = assign84680_body9_e129614_d_n13;
            locals.var_t2_rv = 0.0;
            let (assign84680_body10_e129627, assign84680_body10_e129627_d_n0, assign84680_body10_e129627_d_n2, assign84680_body10_e129627_d_n4, assign84680_body10_e129627_d_n5, assign84680_body10_e129627_d_n6, assign84680_body10_e129627_d_n7, assign84680_body10_e129627_d_n8, assign84680_body10_e129627_d_n9, assign84680_body10_e129627_d_n10, assign84680_body10_e129627_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1969 != 0.0)) {
        let assign84680_body10_e129622: f64 = (1.0 + locals.var_t2);
        let assign84680_body10_e129623: f64 = (assign84680_body10_e129622).ln();
        let assign84680_body10_e129625: f64 = (assign84680_body10_e129623 / locals.var_c_sb);
        (assign84680_body10_e129625, ((((locals.var_t2_dn0 / assign84680_body10_e129622) * locals.var_c_sb) - (assign84680_body10_e129623 * locals.var_c_sb_dn0)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn2 / assign84680_body10_e129622) * locals.var_c_sb) - (assign84680_body10_e129623 * locals.var_c_sb_dn2)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn4 / assign84680_body10_e129622) * locals.var_c_sb) - (assign84680_body10_e129623 * locals.var_c_sb_dn4)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn5 / assign84680_body10_e129622) * locals.var_c_sb) - (assign84680_body10_e129623 * locals.var_c_sb_dn5)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn6 / assign84680_body10_e129622) * locals.var_c_sb) - (assign84680_body10_e129623 * locals.var_c_sb_dn6)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn7 / assign84680_body10_e129622) * locals.var_c_sb) - (assign84680_body10_e129623 * locals.var_c_sb_dn7)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn8 / assign84680_body10_e129622) * locals.var_c_sb) - (assign84680_body10_e129623 * locals.var_c_sb_dn8)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn9 / assign84680_body10_e129622) * locals.var_c_sb) - (assign84680_body10_e129623 * locals.var_c_sb_dn9)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn10 / assign84680_body10_e129622) * locals.var_c_sb) - (assign84680_body10_e129623 * locals.var_c_sb_dn10)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn13 / assign84680_body10_e129622) * locals.var_c_sb) - (assign84680_body10_e129623 * locals.var_c_sb_dn13)) / (locals.var_c_sb * locals.var_c_sb)),)
    } else {
        (locals.var_phi_b, locals.var_phi_b_dn0, locals.var_phi_b_dn2, locals.var_phi_b_dn4, locals.var_phi_b_dn5, locals.var_phi_b_dn6, locals.var_phi_b_dn7, locals.var_phi_b_dn8, locals.var_phi_b_dn9, locals.var_phi_b_dn10, locals.var_phi_b_dn13,)
    }
};
            locals.var_phi_b = assign84680_body10_e129627;
            locals.var_phi_b_dn0 = assign84680_body10_e129627_d_n0;
            locals.var_phi_b_dn2 = assign84680_body10_e129627_d_n2;
            locals.var_phi_b_dn4 = assign84680_body10_e129627_d_n4;
            locals.var_phi_b_dn5 = assign84680_body10_e129627_d_n5;
            locals.var_phi_b_dn6 = assign84680_body10_e129627_d_n6;
            locals.var_phi_b_dn7 = assign84680_body10_e129627_d_n7;
            locals.var_phi_b_dn8 = assign84680_body10_e129627_d_n8;
            locals.var_phi_b_dn9 = assign84680_body10_e129627_d_n9;
            locals.var_phi_b_dn10 = assign84680_body10_e129627_d_n10;
            locals.var_phi_b_dn13 = assign84680_body10_e129627_d_n13;
            locals.var_phi_b_rv = 0.0;
            let (assign84680_body11_e129639, assign84680_body11_e129639_d_n0, assign84680_body11_e129639_d_n2, assign84680_body11_e129639_d_n4, assign84680_body11_e129639_d_n5, assign84680_body11_e129639_d_n6, assign84680_body11_e129639_d_n7, assign84680_body11_e129639_d_n8, assign84680_body11_e129639_d_n9, assign84680_body11_e129639_d_n10, assign84680_body11_e129639_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1969 != 0.0)) {
        let assign84680_body11_e129636: f64 = (1.0 + locals.var_t2);
        let assign84680_body11_e129637: f64 = (locals.var_t1 / assign84680_body11_e129636);
        (assign84680_body11_e129637, (((locals.var_t1_dn0 * assign84680_body11_e129636) - (locals.var_t1 * locals.var_t2_dn0)) / (assign84680_body11_e129636 * assign84680_body11_e129636)), (((locals.var_t1_dn2 * assign84680_body11_e129636) - (locals.var_t1 * locals.var_t2_dn2)) / (assign84680_body11_e129636 * assign84680_body11_e129636)), (((locals.var_t1_dn4 * assign84680_body11_e129636) - (locals.var_t1 * locals.var_t2_dn4)) / (assign84680_body11_e129636 * assign84680_body11_e129636)), (((locals.var_t1_dn5 * assign84680_body11_e129636) - (locals.var_t1 * locals.var_t2_dn5)) / (assign84680_body11_e129636 * assign84680_body11_e129636)), (((locals.var_t1_dn6 * assign84680_body11_e129636) - (locals.var_t1 * locals.var_t2_dn6)) / (assign84680_body11_e129636 * assign84680_body11_e129636)), (((locals.var_t1_dn7 * assign84680_body11_e129636) - (locals.var_t1 * locals.var_t2_dn7)) / (assign84680_body11_e129636 * assign84680_body11_e129636)), (((locals.var_t1_dn8 * assign84680_body11_e129636) - (locals.var_t1 * locals.var_t2_dn8)) / (assign84680_body11_e129636 * assign84680_body11_e129636)), (((locals.var_t1_dn9 * assign84680_body11_e129636) - (locals.var_t1 * locals.var_t2_dn9)) / (assign84680_body11_e129636 * assign84680_body11_e129636)), (((locals.var_t1_dn10 * assign84680_body11_e129636) - (locals.var_t1 * locals.var_t2_dn10)) / (assign84680_body11_e129636 * assign84680_body11_e129636)), (((locals.var_t1_dn13 * assign84680_body11_e129636) - (locals.var_t1 * locals.var_t2_dn13)) / (assign84680_body11_e129636 * assign84680_body11_e129636)),)
    } else {
        (locals.var_phi_b_dpss, locals.var_phi_b_dpss_dn0, locals.var_phi_b_dpss_dn2, locals.var_phi_b_dpss_dn4, locals.var_phi_b_dpss_dn5, locals.var_phi_b_dpss_dn6, locals.var_phi_b_dpss_dn7, locals.var_phi_b_dpss_dn8, locals.var_phi_b_dpss_dn9, locals.var_phi_b_dpss_dn10, locals.var_phi_b_dpss_dn13,)
    }
};
            locals.var_phi_b_dpss = assign84680_body11_e129639;
            locals.var_phi_b_dpss_dn0 = assign84680_body11_e129639_d_n0;
            locals.var_phi_b_dpss_dn2 = assign84680_body11_e129639_d_n2;
            locals.var_phi_b_dpss_dn4 = assign84680_body11_e129639_d_n4;
            locals.var_phi_b_dpss_dn5 = assign84680_body11_e129639_d_n5;
            locals.var_phi_b_dpss_dn6 = assign84680_body11_e129639_d_n6;
            locals.var_phi_b_dpss_dn7 = assign84680_body11_e129639_d_n7;
            locals.var_phi_b_dpss_dn8 = assign84680_body11_e129639_d_n8;
            locals.var_phi_b_dpss_dn9 = assign84680_body11_e129639_d_n9;
            locals.var_phi_b_dpss_dn10 = assign84680_body11_e129639_d_n10;
            locals.var_phi_b_dpss_dn13 = assign84680_body11_e129639_d_n13;
            locals.var_phi_b_dpss_rv = 0.0;
            let (assign84680_body12_e129650, assign84680_body12_e129650_d_n0, assign84680_body12_e129650_d_n2, assign84680_body12_e129650_d_n4, assign84680_body12_e129650_d_n5, assign84680_body12_e129650_d_n6, assign84680_body12_e129650_d_n7, assign84680_body12_e129650_d_n8, assign84680_body12_e129650_d_n9, assign84680_body12_e129650_d_n10, assign84680_body12_e129650_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1969 == 0.0)) {
        let assign84680_body12_e129648: f64 = (locals.var_ps0ld_vxb - locals.var_dphi_sb);
        (assign84680_body12_e129648, (locals.var_ps0ld_vxb_dn0 - locals.var_dphi_sb_dn0), (locals.var_ps0ld_vxb_dn2 - locals.var_dphi_sb_dn2), (locals.var_ps0ld_vxb_dn4 - locals.var_dphi_sb_dn4), (locals.var_ps0ld_vxb_dn5 - locals.var_dphi_sb_dn5), (locals.var_ps0ld_vxb_dn6 - locals.var_dphi_sb_dn6), (locals.var_ps0ld_vxb_dn7 - locals.var_dphi_sb_dn7), (locals.var_ps0ld_vxb_dn8 - locals.var_dphi_sb_dn8), (locals.var_ps0ld_vxb_dn9 - locals.var_dphi_sb_dn9), (locals.var_ps0ld_vxb_dn10 - locals.var_dphi_sb_dn10), (locals.var_ps0ld_vxb_dn13 - locals.var_dphi_sb_dn13),)
    } else {
        (locals.var_phi_b, locals.var_phi_b_dn0, locals.var_phi_b_dn2, locals.var_phi_b_dn4, locals.var_phi_b_dn5, locals.var_phi_b_dn6, locals.var_phi_b_dn7, locals.var_phi_b_dn8, locals.var_phi_b_dn9, locals.var_phi_b_dn10, locals.var_phi_b_dn13,)
    }
};
            locals.var_phi_b = assign84680_body12_e129650;
            locals.var_phi_b_dn0 = assign84680_body12_e129650_d_n0;
            locals.var_phi_b_dn2 = assign84680_body12_e129650_d_n2;
            locals.var_phi_b_dn4 = assign84680_body12_e129650_d_n4;
            locals.var_phi_b_dn5 = assign84680_body12_e129650_d_n5;
            locals.var_phi_b_dn6 = assign84680_body12_e129650_d_n6;
            locals.var_phi_b_dn7 = assign84680_body12_e129650_d_n7;
            locals.var_phi_b_dn8 = assign84680_body12_e129650_d_n8;
            locals.var_phi_b_dn9 = assign84680_body12_e129650_d_n9;
            locals.var_phi_b_dn10 = assign84680_body12_e129650_d_n10;
            locals.var_phi_b_dn13 = assign84680_body12_e129650_d_n13;
            locals.var_phi_b_rv = 0.0;
            let (assign84680_body13_e129659, assign84680_body13_e129659_d_n0, assign84680_body13_e129659_d_n2, assign84680_body13_e129659_d_n4, assign84680_body13_e129659_d_n5, assign84680_body13_e129659_d_n6, assign84680_body13_e129659_d_n7, assign84680_body13_e129659_d_n8, assign84680_body13_e129659_d_n9, assign84680_body13_e129659_d_n10, assign84680_body13_e129659_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1969 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phi_b_dpss, locals.var_phi_b_dpss_dn0, locals.var_phi_b_dpss_dn2, locals.var_phi_b_dpss_dn4, locals.var_phi_b_dpss_dn5, locals.var_phi_b_dpss_dn6, locals.var_phi_b_dpss_dn7, locals.var_phi_b_dpss_dn8, locals.var_phi_b_dpss_dn9, locals.var_phi_b_dpss_dn10, locals.var_phi_b_dpss_dn13,)
    }
};
            locals.var_phi_b_dpss = assign84680_body13_e129659;
            locals.var_phi_b_dpss_dn0 = assign84680_body13_e129659_d_n0;
            locals.var_phi_b_dpss_dn2 = assign84680_body13_e129659_d_n2;
            locals.var_phi_b_dpss_dn4 = assign84680_body13_e129659_d_n4;
            locals.var_phi_b_dpss_dn5 = assign84680_body13_e129659_d_n5;
            locals.var_phi_b_dpss_dn6 = assign84680_body13_e129659_d_n6;
            locals.var_phi_b_dpss_dn7 = assign84680_body13_e129659_d_n7;
            locals.var_phi_b_dpss_dn8 = assign84680_body13_e129659_d_n8;
            locals.var_phi_b_dpss_dn9 = assign84680_body13_e129659_d_n9;
            locals.var_phi_b_dpss_dn10 = assign84680_body13_e129659_d_n10;
            locals.var_phi_b_dpss_dn13 = assign84680_body13_e129659_d_n13;
            locals.var_phi_b_dpss_rv = 0.0;
            let (assign84680_body14_e129667, assign84680_body14_e129667_d_n0, assign84680_body14_e129667_d_n2, assign84680_body14_e129667_d_n4, assign84680_body14_e129667_d_n5, assign84680_body14_e129667_d_n6, assign84680_body14_e129667_d_n7, assign84680_body14_e129667_d_n8, assign84680_body14_e129667_d_n9, assign84680_body14_e129667_d_n10, assign84680_body14_e129667_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) {
        let assign84680_body14_e129665: f64 = (locals.var_beta * locals.var_phi_b);
        (assign84680_body14_e129665, ((locals.var_beta_dn0 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn0)), ((locals.var_beta_dn2 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn2)), ((locals.var_beta_dn4 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn4)), ((locals.var_beta_dn5 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn5)), ((locals.var_beta_dn6 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn6)), ((locals.var_beta_dn7 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn7)), ((locals.var_beta_dn8 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn8)), ((locals.var_beta_dn9 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn9)), ((locals.var_beta_dn10 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn10)), ((locals.var_beta_dn13 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn13)),)
    } else {
        (locals.var_chib, locals.var_chib_dn0, locals.var_chib_dn2, locals.var_chib_dn4, locals.var_chib_dn5, locals.var_chib_dn6, locals.var_chib_dn7, locals.var_chib_dn8, locals.var_chib_dn9, locals.var_chib_dn10, locals.var_chib_dn13,)
    }
};
            locals.var_chib = assign84680_body14_e129667;
            locals.var_chib_dn0 = assign84680_body14_e129667_d_n0;
            locals.var_chib_dn2 = assign84680_body14_e129667_d_n2;
            locals.var_chib_dn4 = assign84680_body14_e129667_d_n4;
            locals.var_chib_dn5 = assign84680_body14_e129667_d_n5;
            locals.var_chib_dn6 = assign84680_body14_e129667_d_n6;
            locals.var_chib_dn7 = assign84680_body14_e129667_d_n7;
            locals.var_chib_dn8 = assign84680_body14_e129667_d_n8;
            locals.var_chib_dn9 = assign84680_body14_e129667_d_n9;
            locals.var_chib_dn10 = assign84680_body14_e129667_d_n10;
            locals.var_chib_dn13 = assign84680_body14_e129667_d_n13;
            locals.var_chib_rv = 0.0;
            let assign84680_body15_e129669: f64 = (locals.var_chi).abs();
            let assign84680_body15_e129671: f64 = if assign84680_body15_e129669 < 1e-6 { 1.0 } else { 0.0 };
            locals.var_guard1970 = assign84680_body15_e129671;
            locals.var_guard1970_rv = 0.0;
            let (assign84680_body17_e129717, assign84680_body17_e129717_d_n0, assign84680_body17_e129717_d_n2, assign84680_body17_e129717_d_n4, assign84680_body17_e129717_d_n5, assign84680_body17_e129717_d_n6, assign84680_body17_e129717_d_n7, assign84680_body17_e129717_d_n8, assign84680_body17_e129717_d_n9, assign84680_body17_e129717_d_n10, assign84680_body17_e129717_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1970 != 0.0)) {
        let assign84680_body17_e129695: f64 = (locals.var_chi * locals.var_chi);
        let assign84680_body17_e129697: f64 = (assign84680_body17_e129695 / 2.0);
        let assign84680_body17_e129701: f64 = (locals.var_chi / 3.0);
        let assign84680_body17_e129705: f64 = (locals.var_chi / 4.0);
        let assign84680_body17_e129709: f64 = (locals.var_chi / 5.0);
        let assign84680_body17_e129710: f64 = (1.0 - assign84680_body17_e129709);
        let assign84680_body17_e129711: f64 = (assign84680_body17_e129705 * assign84680_body17_e129710);
        let assign84680_body17_e129712: f64 = (1.0 - assign84680_body17_e129711);
        let assign84680_body17_e129713: f64 = (assign84680_body17_e129701 * assign84680_body17_e129712);
        let assign84680_body17_e129714: f64 = (1.0 - assign84680_body17_e129713);
        let assign84680_body17_e129715: f64 = (assign84680_body17_e129697 * assign84680_body17_e129714);
        (assign84680_body17_e129715, (((((locals.var_chi_dn0 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn0)) / 2.0) * assign84680_body17_e129714) + (assign84680_body17_e129697 * (-(((locals.var_chi_dn0 / 3.0) * assign84680_body17_e129712) + (assign84680_body17_e129701 * (-(((locals.var_chi_dn0 / 4.0) * assign84680_body17_e129710) + (assign84680_body17_e129705 * (-(locals.var_chi_dn0 / 5.0)))))))))), (((((locals.var_chi_dn2 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn2)) / 2.0) * assign84680_body17_e129714) + (assign84680_body17_e129697 * (-(((locals.var_chi_dn2 / 3.0) * assign84680_body17_e129712) + (assign84680_body17_e129701 * (-(((locals.var_chi_dn2 / 4.0) * assign84680_body17_e129710) + (assign84680_body17_e129705 * (-(locals.var_chi_dn2 / 5.0)))))))))), (((((locals.var_chi_dn4 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn4)) / 2.0) * assign84680_body17_e129714) + (assign84680_body17_e129697 * (-(((locals.var_chi_dn4 / 3.0) * assign84680_body17_e129712) + (assign84680_body17_e129701 * (-(((locals.var_chi_dn4 / 4.0) * assign84680_body17_e129710) + (assign84680_body17_e129705 * (-(locals.var_chi_dn4 / 5.0)))))))))), (((((locals.var_chi_dn5 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn5)) / 2.0) * assign84680_body17_e129714) + (assign84680_body17_e129697 * (-(((locals.var_chi_dn5 / 3.0) * assign84680_body17_e129712) + (assign84680_body17_e129701 * (-(((locals.var_chi_dn5 / 4.0) * assign84680_body17_e129710) + (assign84680_body17_e129705 * (-(locals.var_chi_dn5 / 5.0)))))))))), (((((locals.var_chi_dn6 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn6)) / 2.0) * assign84680_body17_e129714) + (assign84680_body17_e129697 * (-(((locals.var_chi_dn6 / 3.0) * assign84680_body17_e129712) + (assign84680_body17_e129701 * (-(((locals.var_chi_dn6 / 4.0) * assign84680_body17_e129710) + (assign84680_body17_e129705 * (-(locals.var_chi_dn6 / 5.0)))))))))), (((((locals.var_chi_dn7 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn7)) / 2.0) * assign84680_body17_e129714) + (assign84680_body17_e129697 * (-(((locals.var_chi_dn7 / 3.0) * assign84680_body17_e129712) + (assign84680_body17_e129701 * (-(((locals.var_chi_dn7 / 4.0) * assign84680_body17_e129710) + (assign84680_body17_e129705 * (-(locals.var_chi_dn7 / 5.0)))))))))), (((((locals.var_chi_dn8 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn8)) / 2.0) * assign84680_body17_e129714) + (assign84680_body17_e129697 * (-(((locals.var_chi_dn8 / 3.0) * assign84680_body17_e129712) + (assign84680_body17_e129701 * (-(((locals.var_chi_dn8 / 4.0) * assign84680_body17_e129710) + (assign84680_body17_e129705 * (-(locals.var_chi_dn8 / 5.0)))))))))), (((((locals.var_chi_dn9 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn9)) / 2.0) * assign84680_body17_e129714) + (assign84680_body17_e129697 * (-(((locals.var_chi_dn9 / 3.0) * assign84680_body17_e129712) + (assign84680_body17_e129701 * (-(((locals.var_chi_dn9 / 4.0) * assign84680_body17_e129710) + (assign84680_body17_e129705 * (-(locals.var_chi_dn9 / 5.0)))))))))), (((((locals.var_chi_dn10 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn10)) / 2.0) * assign84680_body17_e129714) + (assign84680_body17_e129697 * (-(((locals.var_chi_dn10 / 3.0) * assign84680_body17_e129712) + (assign84680_body17_e129701 * (-(((locals.var_chi_dn10 / 4.0) * assign84680_body17_e129710) + (assign84680_body17_e129705 * (-(locals.var_chi_dn10 / 5.0)))))))))), (((((locals.var_chi_dn13 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn13)) / 2.0) * assign84680_body17_e129714) + (assign84680_body17_e129697 * (-(((locals.var_chi_dn13 / 3.0) * assign84680_body17_e129712) + (assign84680_body17_e129701 * (-(((locals.var_chi_dn13 / 4.0) * assign84680_body17_e129710) + (assign84680_body17_e129705 * (-(locals.var_chi_dn13 / 5.0)))))))))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
            locals.var_t0 = assign84680_body17_e129717;
            locals.var_t0_dn0 = assign84680_body17_e129717_d_n0;
            locals.var_t0_dn2 = assign84680_body17_e129717_d_n2;
            locals.var_t0_dn4 = assign84680_body17_e129717_d_n4;
            locals.var_t0_dn5 = assign84680_body17_e129717_d_n5;
            locals.var_t0_dn6 = assign84680_body17_e129717_d_n6;
            locals.var_t0_dn7 = assign84680_body17_e129717_d_n7;
            locals.var_t0_dn8 = assign84680_body17_e129717_d_n8;
            locals.var_t0_dn9 = assign84680_body17_e129717_d_n9;
            locals.var_t0_dn10 = assign84680_body17_e129717_d_n10;
            locals.var_t0_dn13 = assign84680_body17_e129717_d_n13;
            locals.var_t0_rv = 0.0;
            let (assign84680_body18_e129743, assign84680_body18_e129743_d_n0, assign84680_body18_e129743_d_n2, assign84680_body18_e129743_d_n4, assign84680_body18_e129743_d_n5, assign84680_body18_e129743_d_n6, assign84680_body18_e129743_d_n7, assign84680_body18_e129743_d_n8, assign84680_body18_e129743_d_n9, assign84680_body18_e129743_d_n10, assign84680_body18_e129743_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1970 != 0.0)) {
        let assign84680_body18_e129727: f64 = (locals.var_chi / 2.0);
        let assign84680_body18_e129731: f64 = (locals.var_chi / 3.0);
        let assign84680_body18_e129735: f64 = (locals.var_chi / 4.0);
        let assign84680_body18_e129736: f64 = (1.0 - assign84680_body18_e129735);
        let assign84680_body18_e129737: f64 = (assign84680_body18_e129731 * assign84680_body18_e129736);
        let assign84680_body18_e129738: f64 = (1.0 - assign84680_body18_e129737);
        let assign84680_body18_e129739: f64 = (assign84680_body18_e129727 * assign84680_body18_e129738);
        let assign84680_body18_e129740: f64 = (1.0 - assign84680_body18_e129739);
        let assign84680_body18_e129741: f64 = (locals.var_chi * assign84680_body18_e129740);
        (assign84680_body18_e129741, ((locals.var_chi_dn0 * assign84680_body18_e129740) + (locals.var_chi * (-(((locals.var_chi_dn0 / 2.0) * assign84680_body18_e129738) + (assign84680_body18_e129727 * (-(((locals.var_chi_dn0 / 3.0) * assign84680_body18_e129736) + (assign84680_body18_e129731 * (-(locals.var_chi_dn0 / 4.0)))))))))), ((locals.var_chi_dn2 * assign84680_body18_e129740) + (locals.var_chi * (-(((locals.var_chi_dn2 / 2.0) * assign84680_body18_e129738) + (assign84680_body18_e129727 * (-(((locals.var_chi_dn2 / 3.0) * assign84680_body18_e129736) + (assign84680_body18_e129731 * (-(locals.var_chi_dn2 / 4.0)))))))))), ((locals.var_chi_dn4 * assign84680_body18_e129740) + (locals.var_chi * (-(((locals.var_chi_dn4 / 2.0) * assign84680_body18_e129738) + (assign84680_body18_e129727 * (-(((locals.var_chi_dn4 / 3.0) * assign84680_body18_e129736) + (assign84680_body18_e129731 * (-(locals.var_chi_dn4 / 4.0)))))))))), ((locals.var_chi_dn5 * assign84680_body18_e129740) + (locals.var_chi * (-(((locals.var_chi_dn5 / 2.0) * assign84680_body18_e129738) + (assign84680_body18_e129727 * (-(((locals.var_chi_dn5 / 3.0) * assign84680_body18_e129736) + (assign84680_body18_e129731 * (-(locals.var_chi_dn5 / 4.0)))))))))), ((locals.var_chi_dn6 * assign84680_body18_e129740) + (locals.var_chi * (-(((locals.var_chi_dn6 / 2.0) * assign84680_body18_e129738) + (assign84680_body18_e129727 * (-(((locals.var_chi_dn6 / 3.0) * assign84680_body18_e129736) + (assign84680_body18_e129731 * (-(locals.var_chi_dn6 / 4.0)))))))))), ((locals.var_chi_dn7 * assign84680_body18_e129740) + (locals.var_chi * (-(((locals.var_chi_dn7 / 2.0) * assign84680_body18_e129738) + (assign84680_body18_e129727 * (-(((locals.var_chi_dn7 / 3.0) * assign84680_body18_e129736) + (assign84680_body18_e129731 * (-(locals.var_chi_dn7 / 4.0)))))))))), ((locals.var_chi_dn8 * assign84680_body18_e129740) + (locals.var_chi * (-(((locals.var_chi_dn8 / 2.0) * assign84680_body18_e129738) + (assign84680_body18_e129727 * (-(((locals.var_chi_dn8 / 3.0) * assign84680_body18_e129736) + (assign84680_body18_e129731 * (-(locals.var_chi_dn8 / 4.0)))))))))), ((locals.var_chi_dn9 * assign84680_body18_e129740) + (locals.var_chi * (-(((locals.var_chi_dn9 / 2.0) * assign84680_body18_e129738) + (assign84680_body18_e129727 * (-(((locals.var_chi_dn9 / 3.0) * assign84680_body18_e129736) + (assign84680_body18_e129731 * (-(locals.var_chi_dn9 / 4.0)))))))))), ((locals.var_chi_dn10 * assign84680_body18_e129740) + (locals.var_chi * (-(((locals.var_chi_dn10 / 2.0) * assign84680_body18_e129738) + (assign84680_body18_e129727 * (-(((locals.var_chi_dn10 / 3.0) * assign84680_body18_e129736) + (assign84680_body18_e129731 * (-(locals.var_chi_dn10 / 4.0)))))))))), ((locals.var_chi_dn13 * assign84680_body18_e129740) + (locals.var_chi * (-(((locals.var_chi_dn13 / 2.0) * assign84680_body18_e129738) + (assign84680_body18_e129727 * (-(((locals.var_chi_dn13 / 3.0) * assign84680_body18_e129736) + (assign84680_body18_e129731 * (-(locals.var_chi_dn13 / 4.0)))))))))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign84680_body18_e129743;
            locals.var_t1_dn0 = assign84680_body18_e129743_d_n0;
            locals.var_t1_dn2 = assign84680_body18_e129743_d_n2;
            locals.var_t1_dn4 = assign84680_body18_e129743_d_n4;
            locals.var_t1_dn5 = assign84680_body18_e129743_d_n5;
            locals.var_t1_dn6 = assign84680_body18_e129743_d_n6;
            locals.var_t1_dn7 = assign84680_body18_e129743_d_n7;
            locals.var_t1_dn8 = assign84680_body18_e129743_d_n8;
            locals.var_t1_dn9 = assign84680_body18_e129743_d_n9;
            locals.var_t1_dn10 = assign84680_body18_e129743_d_n10;
            locals.var_t1_dn13 = assign84680_body18_e129743_d_n13;
            locals.var_t1_rv = 0.0;
            let (assign84680_body19_e129773, assign84680_body19_e129773_d_n0, assign84680_body19_e129773_d_n2, assign84680_body19_e129773_d_n4, assign84680_body19_e129773_d_n5, assign84680_body19_e129773_d_n6, assign84680_body19_e129773_d_n7, assign84680_body19_e129773_d_n8, assign84680_body19_e129773_d_n9, assign84680_body19_e129773_d_n10, assign84680_body19_e129773_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1970 != 0.0)) {
        let assign84680_body19_e129751: f64 = (locals.var_chib * locals.var_chib);
        let assign84680_body19_e129753: f64 = (assign84680_body19_e129751 / 2.0);
        let assign84680_body19_e129757: f64 = (locals.var_chib / 3.0);
        let assign84680_body19_e129761: f64 = (locals.var_chib / 4.0);
        let assign84680_body19_e129765: f64 = (locals.var_chib / 5.0);
        let assign84680_body19_e129766: f64 = (1.0 - assign84680_body19_e129765);
        let assign84680_body19_e129767: f64 = (assign84680_body19_e129761 * assign84680_body19_e129766);
        let assign84680_body19_e129768: f64 = (1.0 - assign84680_body19_e129767);
        let assign84680_body19_e129769: f64 = (assign84680_body19_e129757 * assign84680_body19_e129768);
        let assign84680_body19_e129770: f64 = (1.0 - assign84680_body19_e129769);
        let assign84680_body19_e129771: f64 = (assign84680_body19_e129753 * assign84680_body19_e129770);
        (assign84680_body19_e129771, (((((locals.var_chib_dn0 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn0)) / 2.0) * assign84680_body19_e129770) + (assign84680_body19_e129753 * (-(((locals.var_chib_dn0 / 3.0) * assign84680_body19_e129768) + (assign84680_body19_e129757 * (-(((locals.var_chib_dn0 / 4.0) * assign84680_body19_e129766) + (assign84680_body19_e129761 * (-(locals.var_chib_dn0 / 5.0)))))))))), (((((locals.var_chib_dn2 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn2)) / 2.0) * assign84680_body19_e129770) + (assign84680_body19_e129753 * (-(((locals.var_chib_dn2 / 3.0) * assign84680_body19_e129768) + (assign84680_body19_e129757 * (-(((locals.var_chib_dn2 / 4.0) * assign84680_body19_e129766) + (assign84680_body19_e129761 * (-(locals.var_chib_dn2 / 5.0)))))))))), (((((locals.var_chib_dn4 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn4)) / 2.0) * assign84680_body19_e129770) + (assign84680_body19_e129753 * (-(((locals.var_chib_dn4 / 3.0) * assign84680_body19_e129768) + (assign84680_body19_e129757 * (-(((locals.var_chib_dn4 / 4.0) * assign84680_body19_e129766) + (assign84680_body19_e129761 * (-(locals.var_chib_dn4 / 5.0)))))))))), (((((locals.var_chib_dn5 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn5)) / 2.0) * assign84680_body19_e129770) + (assign84680_body19_e129753 * (-(((locals.var_chib_dn5 / 3.0) * assign84680_body19_e129768) + (assign84680_body19_e129757 * (-(((locals.var_chib_dn5 / 4.0) * assign84680_body19_e129766) + (assign84680_body19_e129761 * (-(locals.var_chib_dn5 / 5.0)))))))))), (((((locals.var_chib_dn6 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn6)) / 2.0) * assign84680_body19_e129770) + (assign84680_body19_e129753 * (-(((locals.var_chib_dn6 / 3.0) * assign84680_body19_e129768) + (assign84680_body19_e129757 * (-(((locals.var_chib_dn6 / 4.0) * assign84680_body19_e129766) + (assign84680_body19_e129761 * (-(locals.var_chib_dn6 / 5.0)))))))))), (((((locals.var_chib_dn7 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn7)) / 2.0) * assign84680_body19_e129770) + (assign84680_body19_e129753 * (-(((locals.var_chib_dn7 / 3.0) * assign84680_body19_e129768) + (assign84680_body19_e129757 * (-(((locals.var_chib_dn7 / 4.0) * assign84680_body19_e129766) + (assign84680_body19_e129761 * (-(locals.var_chib_dn7 / 5.0)))))))))), (((((locals.var_chib_dn8 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn8)) / 2.0) * assign84680_body19_e129770) + (assign84680_body19_e129753 * (-(((locals.var_chib_dn8 / 3.0) * assign84680_body19_e129768) + (assign84680_body19_e129757 * (-(((locals.var_chib_dn8 / 4.0) * assign84680_body19_e129766) + (assign84680_body19_e129761 * (-(locals.var_chib_dn8 / 5.0)))))))))), (((((locals.var_chib_dn9 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn9)) / 2.0) * assign84680_body19_e129770) + (assign84680_body19_e129753 * (-(((locals.var_chib_dn9 / 3.0) * assign84680_body19_e129768) + (assign84680_body19_e129757 * (-(((locals.var_chib_dn9 / 4.0) * assign84680_body19_e129766) + (assign84680_body19_e129761 * (-(locals.var_chib_dn9 / 5.0)))))))))), (((((locals.var_chib_dn10 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn10)) / 2.0) * assign84680_body19_e129770) + (assign84680_body19_e129753 * (-(((locals.var_chib_dn10 / 3.0) * assign84680_body19_e129768) + (assign84680_body19_e129757 * (-(((locals.var_chib_dn10 / 4.0) * assign84680_body19_e129766) + (assign84680_body19_e129761 * (-(locals.var_chib_dn10 / 5.0)))))))))), (((((locals.var_chib_dn13 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn13)) / 2.0) * assign84680_body19_e129770) + (assign84680_body19_e129753 * (-(((locals.var_chib_dn13 / 3.0) * assign84680_body19_e129768) + (assign84680_body19_e129757 * (-(((locals.var_chib_dn13 / 4.0) * assign84680_body19_e129766) + (assign84680_body19_e129761 * (-(locals.var_chib_dn13 / 5.0)))))))))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
            locals.var_t2 = assign84680_body19_e129773;
            locals.var_t2_dn0 = assign84680_body19_e129773_d_n0;
            locals.var_t2_dn2 = assign84680_body19_e129773_d_n2;
            locals.var_t2_dn4 = assign84680_body19_e129773_d_n4;
            locals.var_t2_dn5 = assign84680_body19_e129773_d_n5;
            locals.var_t2_dn6 = assign84680_body19_e129773_d_n6;
            locals.var_t2_dn7 = assign84680_body19_e129773_d_n7;
            locals.var_t2_dn8 = assign84680_body19_e129773_d_n8;
            locals.var_t2_dn9 = assign84680_body19_e129773_d_n9;
            locals.var_t2_dn10 = assign84680_body19_e129773_d_n10;
            locals.var_t2_dn13 = assign84680_body19_e129773_d_n13;
            locals.var_t2_rv = 0.0;
            let (assign84680_body20_e129799, assign84680_body20_e129799_d_n0, assign84680_body20_e129799_d_n2, assign84680_body20_e129799_d_n4, assign84680_body20_e129799_d_n5, assign84680_body20_e129799_d_n6, assign84680_body20_e129799_d_n7, assign84680_body20_e129799_d_n8, assign84680_body20_e129799_d_n9, assign84680_body20_e129799_d_n10, assign84680_body20_e129799_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1970 != 0.0)) {
        let assign84680_body20_e129783: f64 = (locals.var_chib / 2.0);
        let assign84680_body20_e129787: f64 = (locals.var_chib / 3.0);
        let assign84680_body20_e129791: f64 = (locals.var_chib / 4.0);
        let assign84680_body20_e129792: f64 = (1.0 - assign84680_body20_e129791);
        let assign84680_body20_e129793: f64 = (assign84680_body20_e129787 * assign84680_body20_e129792);
        let assign84680_body20_e129794: f64 = (1.0 - assign84680_body20_e129793);
        let assign84680_body20_e129795: f64 = (assign84680_body20_e129783 * assign84680_body20_e129794);
        let assign84680_body20_e129796: f64 = (1.0 - assign84680_body20_e129795);
        let assign84680_body20_e129797: f64 = (locals.var_chib * assign84680_body20_e129796);
        (assign84680_body20_e129797, ((locals.var_chib_dn0 * assign84680_body20_e129796) + (locals.var_chib * (-(((locals.var_chib_dn0 / 2.0) * assign84680_body20_e129794) + (assign84680_body20_e129783 * (-(((locals.var_chib_dn0 / 3.0) * assign84680_body20_e129792) + (assign84680_body20_e129787 * (-(locals.var_chib_dn0 / 4.0)))))))))), ((locals.var_chib_dn2 * assign84680_body20_e129796) + (locals.var_chib * (-(((locals.var_chib_dn2 / 2.0) * assign84680_body20_e129794) + (assign84680_body20_e129783 * (-(((locals.var_chib_dn2 / 3.0) * assign84680_body20_e129792) + (assign84680_body20_e129787 * (-(locals.var_chib_dn2 / 4.0)))))))))), ((locals.var_chib_dn4 * assign84680_body20_e129796) + (locals.var_chib * (-(((locals.var_chib_dn4 / 2.0) * assign84680_body20_e129794) + (assign84680_body20_e129783 * (-(((locals.var_chib_dn4 / 3.0) * assign84680_body20_e129792) + (assign84680_body20_e129787 * (-(locals.var_chib_dn4 / 4.0)))))))))), ((locals.var_chib_dn5 * assign84680_body20_e129796) + (locals.var_chib * (-(((locals.var_chib_dn5 / 2.0) * assign84680_body20_e129794) + (assign84680_body20_e129783 * (-(((locals.var_chib_dn5 / 3.0) * assign84680_body20_e129792) + (assign84680_body20_e129787 * (-(locals.var_chib_dn5 / 4.0)))))))))), ((locals.var_chib_dn6 * assign84680_body20_e129796) + (locals.var_chib * (-(((locals.var_chib_dn6 / 2.0) * assign84680_body20_e129794) + (assign84680_body20_e129783 * (-(((locals.var_chib_dn6 / 3.0) * assign84680_body20_e129792) + (assign84680_body20_e129787 * (-(locals.var_chib_dn6 / 4.0)))))))))), ((locals.var_chib_dn7 * assign84680_body20_e129796) + (locals.var_chib * (-(((locals.var_chib_dn7 / 2.0) * assign84680_body20_e129794) + (assign84680_body20_e129783 * (-(((locals.var_chib_dn7 / 3.0) * assign84680_body20_e129792) + (assign84680_body20_e129787 * (-(locals.var_chib_dn7 / 4.0)))))))))), ((locals.var_chib_dn8 * assign84680_body20_e129796) + (locals.var_chib * (-(((locals.var_chib_dn8 / 2.0) * assign84680_body20_e129794) + (assign84680_body20_e129783 * (-(((locals.var_chib_dn8 / 3.0) * assign84680_body20_e129792) + (assign84680_body20_e129787 * (-(locals.var_chib_dn8 / 4.0)))))))))), ((locals.var_chib_dn9 * assign84680_body20_e129796) + (locals.var_chib * (-(((locals.var_chib_dn9 / 2.0) * assign84680_body20_e129794) + (assign84680_body20_e129783 * (-(((locals.var_chib_dn9 / 3.0) * assign84680_body20_e129792) + (assign84680_body20_e129787 * (-(locals.var_chib_dn9 / 4.0)))))))))), ((locals.var_chib_dn10 * assign84680_body20_e129796) + (locals.var_chib * (-(((locals.var_chib_dn10 / 2.0) * assign84680_body20_e129794) + (assign84680_body20_e129783 * (-(((locals.var_chib_dn10 / 3.0) * assign84680_body20_e129792) + (assign84680_body20_e129787 * (-(locals.var_chib_dn10 / 4.0)))))))))), ((locals.var_chib_dn13 * assign84680_body20_e129796) + (locals.var_chib * (-(((locals.var_chib_dn13 / 2.0) * assign84680_body20_e129794) + (assign84680_body20_e129783 * (-(((locals.var_chib_dn13 / 3.0) * assign84680_body20_e129792) + (assign84680_body20_e129787 * (-(locals.var_chib_dn13 / 4.0)))))))))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
            locals.var_t3 = assign84680_body20_e129799;
            locals.var_t3_dn0 = assign84680_body20_e129799_d_n0;
            locals.var_t3_dn2 = assign84680_body20_e129799_d_n2;
            locals.var_t3_dn4 = assign84680_body20_e129799_d_n4;
            locals.var_t3_dn5 = assign84680_body20_e129799_d_n5;
            locals.var_t3_dn6 = assign84680_body20_e129799_d_n6;
            locals.var_t3_dn7 = assign84680_body20_e129799_d_n7;
            locals.var_t3_dn8 = assign84680_body20_e129799_d_n8;
            locals.var_t3_dn9 = assign84680_body20_e129799_d_n9;
            locals.var_t3_dn10 = assign84680_body20_e129799_d_n10;
            locals.var_t3_dn13 = assign84680_body20_e129799_d_n13;
            locals.var_t3_rv = 0.0;
            let (assign84680_body21_e129809, assign84680_body21_e129809_d_n0, assign84680_body21_e129809_d_n2, assign84680_body21_e129809_d_n4, assign84680_body21_e129809_d_n5, assign84680_body21_e129809_d_n6, assign84680_body21_e129809_d_n7, assign84680_body21_e129809_d_n8, assign84680_body21_e129809_d_n9, assign84680_body21_e129809_d_n10, assign84680_body21_e129809_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1970 != 0.0)) {
        let assign84680_body21_e129807: f64 = (locals.var_t0 - locals.var_t2);
        (assign84680_body21_e129807, (locals.var_t0_dn0 - locals.var_t2_dn0), (locals.var_t0_dn2 - locals.var_t2_dn2), (locals.var_t0_dn4 - locals.var_t2_dn4), (locals.var_t0_dn5 - locals.var_t2_dn5), (locals.var_t0_dn6 - locals.var_t2_dn6), (locals.var_t0_dn7 - locals.var_t2_dn7), (locals.var_t0_dn8 - locals.var_t2_dn8), (locals.var_t0_dn9 - locals.var_t2_dn9), (locals.var_t0_dn10 - locals.var_t2_dn10), (locals.var_t0_dn13 - locals.var_t2_dn13),)
    } else {
        (locals.var_fbsq__blk1891, locals.var_fbsq__blk1891_dn0, locals.var_fbsq__blk1891_dn2, locals.var_fbsq__blk1891_dn4, locals.var_fbsq__blk1891_dn5, locals.var_fbsq__blk1891_dn6, locals.var_fbsq__blk1891_dn7, locals.var_fbsq__blk1891_dn8, locals.var_fbsq__blk1891_dn9, locals.var_fbsq__blk1891_dn10, locals.var_fbsq__blk1891_dn13,)
    }
};
            locals.var_fbsq__blk1891 = assign84680_body21_e129809;
            locals.var_fbsq__blk1891_dn0 = assign84680_body21_e129809_d_n0;
            locals.var_fbsq__blk1891_dn2 = assign84680_body21_e129809_d_n2;
            locals.var_fbsq__blk1891_dn4 = assign84680_body21_e129809_d_n4;
            locals.var_fbsq__blk1891_dn5 = assign84680_body21_e129809_d_n5;
            locals.var_fbsq__blk1891_dn6 = assign84680_body21_e129809_d_n6;
            locals.var_fbsq__blk1891_dn7 = assign84680_body21_e129809_d_n7;
            locals.var_fbsq__blk1891_dn8 = assign84680_body21_e129809_d_n8;
            locals.var_fbsq__blk1891_dn9 = assign84680_body21_e129809_d_n9;
            locals.var_fbsq__blk1891_dn10 = assign84680_body21_e129809_d_n10;
            locals.var_fbsq__blk1891_dn13 = assign84680_body21_e129809_d_n13;
            locals.var_fbsq__blk1891_rv = 0.0;
            let (assign84680_body22_e129823, assign84680_body22_e129823_d_n0, assign84680_body22_e129823_d_n2, assign84680_body22_e129823_d_n4, assign84680_body22_e129823_d_n5, assign84680_body22_e129823_d_n6, assign84680_body22_e129823_d_n7, assign84680_body22_e129823_d_n8, assign84680_body22_e129823_d_n9, assign84680_body22_e129823_d_n10, assign84680_body22_e129823_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1970 != 0.0)) {
        let assign84680_body22_e129819: f64 = (locals.var_phi_b_dpss * locals.var_t3);
        let assign84680_body22_e129820: f64 = (locals.var_t1 - assign84680_body22_e129819);
        let assign84680_body22_e129821: f64 = (locals.var_beta * assign84680_body22_e129820);
        (assign84680_body22_e129821, ((locals.var_beta_dn0 * assign84680_body22_e129820) + (locals.var_beta * (locals.var_t1_dn0 - ((locals.var_phi_b_dpss_dn0 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn0))))), ((locals.var_beta_dn2 * assign84680_body22_e129820) + (locals.var_beta * (locals.var_t1_dn2 - ((locals.var_phi_b_dpss_dn2 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn2))))), ((locals.var_beta_dn4 * assign84680_body22_e129820) + (locals.var_beta * (locals.var_t1_dn4 - ((locals.var_phi_b_dpss_dn4 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn4))))), ((locals.var_beta_dn5 * assign84680_body22_e129820) + (locals.var_beta * (locals.var_t1_dn5 - ((locals.var_phi_b_dpss_dn5 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn5))))), ((locals.var_beta_dn6 * assign84680_body22_e129820) + (locals.var_beta * (locals.var_t1_dn6 - ((locals.var_phi_b_dpss_dn6 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn6))))), ((locals.var_beta_dn7 * assign84680_body22_e129820) + (locals.var_beta * (locals.var_t1_dn7 - ((locals.var_phi_b_dpss_dn7 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn7))))), ((locals.var_beta_dn8 * assign84680_body22_e129820) + (locals.var_beta * (locals.var_t1_dn8 - ((locals.var_phi_b_dpss_dn8 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn8))))), ((locals.var_beta_dn9 * assign84680_body22_e129820) + (locals.var_beta * (locals.var_t1_dn9 - ((locals.var_phi_b_dpss_dn9 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn9))))), ((locals.var_beta_dn10 * assign84680_body22_e129820) + (locals.var_beta * (locals.var_t1_dn10 - ((locals.var_phi_b_dpss_dn10 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn10))))), ((locals.var_beta_dn13 * assign84680_body22_e129820) + (locals.var_beta * (locals.var_t1_dn13 - ((locals.var_phi_b_dpss_dn13 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn13))))),)
    } else {
        (locals.var_fbsq_dpss__blk1892, locals.var_fbsq_dpss__blk1892_dn0, locals.var_fbsq_dpss__blk1892_dn2, locals.var_fbsq_dpss__blk1892_dn4, locals.var_fbsq_dpss__blk1892_dn5, locals.var_fbsq_dpss__blk1892_dn6, locals.var_fbsq_dpss__blk1892_dn7, locals.var_fbsq_dpss__blk1892_dn8, locals.var_fbsq_dpss__blk1892_dn9, locals.var_fbsq_dpss__blk1892_dn10, locals.var_fbsq_dpss__blk1892_dn13,)
    }
};
            locals.var_fbsq_dpss__blk1892 = assign84680_body22_e129823;
            locals.var_fbsq_dpss__blk1892_dn0 = assign84680_body22_e129823_d_n0;
            locals.var_fbsq_dpss__blk1892_dn2 = assign84680_body22_e129823_d_n2;
            locals.var_fbsq_dpss__blk1892_dn4 = assign84680_body22_e129823_d_n4;
            locals.var_fbsq_dpss__blk1892_dn5 = assign84680_body22_e129823_d_n5;
            locals.var_fbsq_dpss__blk1892_dn6 = assign84680_body22_e129823_d_n6;
            locals.var_fbsq_dpss__blk1892_dn7 = assign84680_body22_e129823_d_n7;
            locals.var_fbsq_dpss__blk1892_dn8 = assign84680_body22_e129823_d_n8;
            locals.var_fbsq_dpss__blk1892_dn9 = assign84680_body22_e129823_d_n9;
            locals.var_fbsq_dpss__blk1892_dn10 = assign84680_body22_e129823_d_n10;
            locals.var_fbsq_dpss__blk1892_dn13 = assign84680_body22_e129823_d_n13;
            locals.var_fbsq_dpss__blk1892_rv = 0.0;
            let (assign84680_body24_e129851, assign84680_body24_e129851_d_n0, assign84680_body24_e129851_d_n2, assign84680_body24_e129851_d_n4, assign84680_body24_e129851_d_n5, assign84680_body24_e129851_d_n6, assign84680_body24_e129851_d_n7, assign84680_body24_e129851_d_n8, assign84680_body24_e129851_d_n9, assign84680_body24_e129851_d_n10, assign84680_body24_e129851_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1970 == 0.0)) {
        let assign84680_body24_e129848: f64 = (-locals.var_chi);
        let assign84680_body24_e129849: f64 = (assign84680_body24_e129848).exp();
        (assign84680_body24_e129849, (assign84680_body24_e129849 * (-locals.var_chi_dn0)), (assign84680_body24_e129849 * (-locals.var_chi_dn2)), (assign84680_body24_e129849 * (-locals.var_chi_dn4)), (assign84680_body24_e129849 * (-locals.var_chi_dn5)), (assign84680_body24_e129849 * (-locals.var_chi_dn6)), (assign84680_body24_e129849 * (-locals.var_chi_dn7)), (assign84680_body24_e129849 * (-locals.var_chi_dn8)), (assign84680_body24_e129849 * (-locals.var_chi_dn9)), (assign84680_body24_e129849 * (-locals.var_chi_dn10)), (assign84680_body24_e129849 * (-locals.var_chi_dn13)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
            locals.var_t0 = assign84680_body24_e129851;
            locals.var_t0_dn0 = assign84680_body24_e129851_d_n0;
            locals.var_t0_dn2 = assign84680_body24_e129851_d_n2;
            locals.var_t0_dn4 = assign84680_body24_e129851_d_n4;
            locals.var_t0_dn5 = assign84680_body24_e129851_d_n5;
            locals.var_t0_dn6 = assign84680_body24_e129851_d_n6;
            locals.var_t0_dn7 = assign84680_body24_e129851_d_n7;
            locals.var_t0_dn8 = assign84680_body24_e129851_d_n8;
            locals.var_t0_dn9 = assign84680_body24_e129851_d_n9;
            locals.var_t0_dn10 = assign84680_body24_e129851_d_n10;
            locals.var_t0_dn13 = assign84680_body24_e129851_d_n13;
            locals.var_t0_rv = 0.0;
            let (assign84680_body25_e129862, assign84680_body25_e129862_d_n0, assign84680_body25_e129862_d_n2, assign84680_body25_e129862_d_n4, assign84680_body25_e129862_d_n5, assign84680_body25_e129862_d_n6, assign84680_body25_e129862_d_n7, assign84680_body25_e129862_d_n8, assign84680_body25_e129862_d_n9, assign84680_body25_e129862_d_n10, assign84680_body25_e129862_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1970 == 0.0)) {
        let assign84680_body25_e129859: f64 = (-locals.var_chib);
        let assign84680_body25_e129860: f64 = (assign84680_body25_e129859).exp();
        (assign84680_body25_e129860, (assign84680_body25_e129860 * (-locals.var_chib_dn0)), (assign84680_body25_e129860 * (-locals.var_chib_dn2)), (assign84680_body25_e129860 * (-locals.var_chib_dn4)), (assign84680_body25_e129860 * (-locals.var_chib_dn5)), (assign84680_body25_e129860 * (-locals.var_chib_dn6)), (assign84680_body25_e129860 * (-locals.var_chib_dn7)), (assign84680_body25_e129860 * (-locals.var_chib_dn8)), (assign84680_body25_e129860 * (-locals.var_chib_dn9)), (assign84680_body25_e129860 * (-locals.var_chib_dn10)), (assign84680_body25_e129860 * (-locals.var_chib_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign84680_body25_e129862;
            locals.var_t1_dn0 = assign84680_body25_e129862_d_n0;
            locals.var_t1_dn2 = assign84680_body25_e129862_d_n2;
            locals.var_t1_dn4 = assign84680_body25_e129862_d_n4;
            locals.var_t1_dn5 = assign84680_body25_e129862_d_n5;
            locals.var_t1_dn6 = assign84680_body25_e129862_d_n6;
            locals.var_t1_dn7 = assign84680_body25_e129862_d_n7;
            locals.var_t1_dn8 = assign84680_body25_e129862_d_n8;
            locals.var_t1_dn9 = assign84680_body25_e129862_d_n9;
            locals.var_t1_dn10 = assign84680_body25_e129862_d_n10;
            locals.var_t1_dn13 = assign84680_body25_e129862_d_n13;
            locals.var_t1_rv = 0.0;
            let (assign84680_body26_e129877, assign84680_body26_e129877_d_n0, assign84680_body26_e129877_d_n2, assign84680_body26_e129877_d_n4, assign84680_body26_e129877_d_n5, assign84680_body26_e129877_d_n6, assign84680_body26_e129877_d_n7, assign84680_body26_e129877_d_n8, assign84680_body26_e129877_d_n9, assign84680_body26_e129877_d_n10, assign84680_body26_e129877_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1970 == 0.0)) {
        let assign84680_body26_e129871: f64 = (locals.var_chi - locals.var_chib);
        let assign84680_body26_e129874: f64 = (locals.var_t0 - locals.var_t1);
        let assign84680_body26_e129875: f64 = (assign84680_body26_e129871 + assign84680_body26_e129874);
        (assign84680_body26_e129875, ((locals.var_chi_dn0 - locals.var_chib_dn0) + (locals.var_t0_dn0 - locals.var_t1_dn0)), ((locals.var_chi_dn2 - locals.var_chib_dn2) + (locals.var_t0_dn2 - locals.var_t1_dn2)), ((locals.var_chi_dn4 - locals.var_chib_dn4) + (locals.var_t0_dn4 - locals.var_t1_dn4)), ((locals.var_chi_dn5 - locals.var_chib_dn5) + (locals.var_t0_dn5 - locals.var_t1_dn5)), ((locals.var_chi_dn6 - locals.var_chib_dn6) + (locals.var_t0_dn6 - locals.var_t1_dn6)), ((locals.var_chi_dn7 - locals.var_chib_dn7) + (locals.var_t0_dn7 - locals.var_t1_dn7)), ((locals.var_chi_dn8 - locals.var_chib_dn8) + (locals.var_t0_dn8 - locals.var_t1_dn8)), ((locals.var_chi_dn9 - locals.var_chib_dn9) + (locals.var_t0_dn9 - locals.var_t1_dn9)), ((locals.var_chi_dn10 - locals.var_chib_dn10) + (locals.var_t0_dn10 - locals.var_t1_dn10)), ((locals.var_chi_dn13 - locals.var_chib_dn13) + (locals.var_t0_dn13 - locals.var_t1_dn13)),)
    } else {
        (locals.var_fbsq__blk1891, locals.var_fbsq__blk1891_dn0, locals.var_fbsq__blk1891_dn2, locals.var_fbsq__blk1891_dn4, locals.var_fbsq__blk1891_dn5, locals.var_fbsq__blk1891_dn6, locals.var_fbsq__blk1891_dn7, locals.var_fbsq__blk1891_dn8, locals.var_fbsq__blk1891_dn9, locals.var_fbsq__blk1891_dn10, locals.var_fbsq__blk1891_dn13,)
    }
};
            locals.var_fbsq__blk1891 = assign84680_body26_e129877;
            locals.var_fbsq__blk1891_dn0 = assign84680_body26_e129877_d_n0;
            locals.var_fbsq__blk1891_dn2 = assign84680_body26_e129877_d_n2;
            locals.var_fbsq__blk1891_dn4 = assign84680_body26_e129877_d_n4;
            locals.var_fbsq__blk1891_dn5 = assign84680_body26_e129877_d_n5;
            locals.var_fbsq__blk1891_dn6 = assign84680_body26_e129877_d_n6;
            locals.var_fbsq__blk1891_dn7 = assign84680_body26_e129877_d_n7;
            locals.var_fbsq__blk1891_dn8 = assign84680_body26_e129877_d_n8;
            locals.var_fbsq__blk1891_dn9 = assign84680_body26_e129877_d_n9;
            locals.var_fbsq__blk1891_dn10 = assign84680_body26_e129877_d_n10;
            locals.var_fbsq__blk1891_dn13 = assign84680_body26_e129877_d_n13;
            locals.var_fbsq__blk1891_rv = 0.0;
            let (assign84680_body27_e129896, assign84680_body27_e129896_d_n0, assign84680_body27_e129896_d_n2, assign84680_body27_e129896_d_n4, assign84680_body27_e129896_d_n5, assign84680_body27_e129896_d_n6, assign84680_body27_e129896_d_n7, assign84680_body27_e129896_d_n8, assign84680_body27_e129896_d_n9, assign84680_body27_e129896_d_n10, assign84680_body27_e129896_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1970 == 0.0)) {
        let assign84680_body27_e129887: f64 = (1.0 - locals.var_t0);
        let assign84680_body27_e129891: f64 = (1.0 - locals.var_t1);
        let assign84680_body27_e129892: f64 = (locals.var_phi_b_dpss * assign84680_body27_e129891);
        let assign84680_body27_e129893: f64 = (assign84680_body27_e129887 - assign84680_body27_e129892);
        let assign84680_body27_e129894: f64 = (locals.var_beta * assign84680_body27_e129893);
        (assign84680_body27_e129894, ((locals.var_beta_dn0 * assign84680_body27_e129893) + (locals.var_beta * ((-locals.var_t0_dn0) - ((locals.var_phi_b_dpss_dn0 * assign84680_body27_e129891) + (locals.var_phi_b_dpss * (-locals.var_t1_dn0)))))), ((locals.var_beta_dn2 * assign84680_body27_e129893) + (locals.var_beta * ((-locals.var_t0_dn2) - ((locals.var_phi_b_dpss_dn2 * assign84680_body27_e129891) + (locals.var_phi_b_dpss * (-locals.var_t1_dn2)))))), ((locals.var_beta_dn4 * assign84680_body27_e129893) + (locals.var_beta * ((-locals.var_t0_dn4) - ((locals.var_phi_b_dpss_dn4 * assign84680_body27_e129891) + (locals.var_phi_b_dpss * (-locals.var_t1_dn4)))))), ((locals.var_beta_dn5 * assign84680_body27_e129893) + (locals.var_beta * ((-locals.var_t0_dn5) - ((locals.var_phi_b_dpss_dn5 * assign84680_body27_e129891) + (locals.var_phi_b_dpss * (-locals.var_t1_dn5)))))), ((locals.var_beta_dn6 * assign84680_body27_e129893) + (locals.var_beta * ((-locals.var_t0_dn6) - ((locals.var_phi_b_dpss_dn6 * assign84680_body27_e129891) + (locals.var_phi_b_dpss * (-locals.var_t1_dn6)))))), ((locals.var_beta_dn7 * assign84680_body27_e129893) + (locals.var_beta * ((-locals.var_t0_dn7) - ((locals.var_phi_b_dpss_dn7 * assign84680_body27_e129891) + (locals.var_phi_b_dpss * (-locals.var_t1_dn7)))))), ((locals.var_beta_dn8 * assign84680_body27_e129893) + (locals.var_beta * ((-locals.var_t0_dn8) - ((locals.var_phi_b_dpss_dn8 * assign84680_body27_e129891) + (locals.var_phi_b_dpss * (-locals.var_t1_dn8)))))), ((locals.var_beta_dn9 * assign84680_body27_e129893) + (locals.var_beta * ((-locals.var_t0_dn9) - ((locals.var_phi_b_dpss_dn9 * assign84680_body27_e129891) + (locals.var_phi_b_dpss * (-locals.var_t1_dn9)))))), ((locals.var_beta_dn10 * assign84680_body27_e129893) + (locals.var_beta * ((-locals.var_t0_dn10) - ((locals.var_phi_b_dpss_dn10 * assign84680_body27_e129891) + (locals.var_phi_b_dpss * (-locals.var_t1_dn10)))))), ((locals.var_beta_dn13 * assign84680_body27_e129893) + (locals.var_beta * ((-locals.var_t0_dn13) - ((locals.var_phi_b_dpss_dn13 * assign84680_body27_e129891) + (locals.var_phi_b_dpss * (-locals.var_t1_dn13)))))),)
    } else {
        (locals.var_fbsq_dpss__blk1892, locals.var_fbsq_dpss__blk1892_dn0, locals.var_fbsq_dpss__blk1892_dn2, locals.var_fbsq_dpss__blk1892_dn4, locals.var_fbsq_dpss__blk1892_dn5, locals.var_fbsq_dpss__blk1892_dn6, locals.var_fbsq_dpss__blk1892_dn7, locals.var_fbsq_dpss__blk1892_dn8, locals.var_fbsq_dpss__blk1892_dn9, locals.var_fbsq_dpss__blk1892_dn10, locals.var_fbsq_dpss__blk1892_dn13,)
    }
};
            locals.var_fbsq_dpss__blk1892 = assign84680_body27_e129896;
            locals.var_fbsq_dpss__blk1892_dn0 = assign84680_body27_e129896_d_n0;
            locals.var_fbsq_dpss__blk1892_dn2 = assign84680_body27_e129896_d_n2;
            locals.var_fbsq_dpss__blk1892_dn4 = assign84680_body27_e129896_d_n4;
            locals.var_fbsq_dpss__blk1892_dn5 = assign84680_body27_e129896_d_n5;
            locals.var_fbsq_dpss__blk1892_dn6 = assign84680_body27_e129896_d_n6;
            locals.var_fbsq_dpss__blk1892_dn7 = assign84680_body27_e129896_d_n7;
            locals.var_fbsq_dpss__blk1892_dn8 = assign84680_body27_e129896_d_n8;
            locals.var_fbsq_dpss__blk1892_dn9 = assign84680_body27_e129896_d_n9;
            locals.var_fbsq_dpss__blk1892_dn10 = assign84680_body27_e129896_d_n10;
            locals.var_fbsq_dpss__blk1892_dn13 = assign84680_body27_e129896_d_n13;
            locals.var_fbsq_dpss__blk1892_rv = 0.0;
            let assign84680_body28_e129898: f64 = (locals.var_chi).abs();
            let assign84680_body28_e129900: f64 = if assign84680_body28_e129898 < 5e-5 { 1.0 } else { 0.0 };
            locals.var_guard1971 = assign84680_body28_e129900;
            locals.var_guard1971_rv = 0.0;
            let (assign84680_body29_e129930, assign84680_body29_e129930_d_n0, assign84680_body29_e129930_d_n2, assign84680_body29_e129930_d_n4, assign84680_body29_e129930_d_n5, assign84680_body29_e129930_d_n6, assign84680_body29_e129930_d_n7, assign84680_body29_e129930_d_n8, assign84680_body29_e129930_d_n9, assign84680_body29_e129930_d_n10, assign84680_body29_e129930_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1971 != 0.0)) {
        let assign84680_body29_e129908: f64 = (locals.var_chi * locals.var_chi);
        let assign84680_body29_e129910: f64 = (assign84680_body29_e129908 / 2.0);
        let assign84680_body29_e129914: f64 = (locals.var_chi / 3.0);
        let assign84680_body29_e129918: f64 = (locals.var_chi / 4.0);
        let assign84680_body29_e129922: f64 = (locals.var_chi / 5.0);
        let assign84680_body29_e129923: f64 = (1.0 + assign84680_body29_e129922);
        let assign84680_body29_e129924: f64 = (assign84680_body29_e129918 * assign84680_body29_e129923);
        let assign84680_body29_e129925: f64 = (1.0 + assign84680_body29_e129924);
        let assign84680_body29_e129926: f64 = (assign84680_body29_e129914 * assign84680_body29_e129925);
        let assign84680_body29_e129927: f64 = (1.0 + assign84680_body29_e129926);
        let assign84680_body29_e129928: f64 = (assign84680_body29_e129910 * assign84680_body29_e129927);
        (assign84680_body29_e129928, (((((locals.var_chi_dn0 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn0)) / 2.0) * assign84680_body29_e129927) + (assign84680_body29_e129910 * (((locals.var_chi_dn0 / 3.0) * assign84680_body29_e129925) + (assign84680_body29_e129914 * (((locals.var_chi_dn0 / 4.0) * assign84680_body29_e129923) + (assign84680_body29_e129918 * (locals.var_chi_dn0 / 5.0))))))), (((((locals.var_chi_dn2 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn2)) / 2.0) * assign84680_body29_e129927) + (assign84680_body29_e129910 * (((locals.var_chi_dn2 / 3.0) * assign84680_body29_e129925) + (assign84680_body29_e129914 * (((locals.var_chi_dn2 / 4.0) * assign84680_body29_e129923) + (assign84680_body29_e129918 * (locals.var_chi_dn2 / 5.0))))))), (((((locals.var_chi_dn4 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn4)) / 2.0) * assign84680_body29_e129927) + (assign84680_body29_e129910 * (((locals.var_chi_dn4 / 3.0) * assign84680_body29_e129925) + (assign84680_body29_e129914 * (((locals.var_chi_dn4 / 4.0) * assign84680_body29_e129923) + (assign84680_body29_e129918 * (locals.var_chi_dn4 / 5.0))))))), (((((locals.var_chi_dn5 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn5)) / 2.0) * assign84680_body29_e129927) + (assign84680_body29_e129910 * (((locals.var_chi_dn5 / 3.0) * assign84680_body29_e129925) + (assign84680_body29_e129914 * (((locals.var_chi_dn5 / 4.0) * assign84680_body29_e129923) + (assign84680_body29_e129918 * (locals.var_chi_dn5 / 5.0))))))), (((((locals.var_chi_dn6 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn6)) / 2.0) * assign84680_body29_e129927) + (assign84680_body29_e129910 * (((locals.var_chi_dn6 / 3.0) * assign84680_body29_e129925) + (assign84680_body29_e129914 * (((locals.var_chi_dn6 / 4.0) * assign84680_body29_e129923) + (assign84680_body29_e129918 * (locals.var_chi_dn6 / 5.0))))))), (((((locals.var_chi_dn7 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn7)) / 2.0) * assign84680_body29_e129927) + (assign84680_body29_e129910 * (((locals.var_chi_dn7 / 3.0) * assign84680_body29_e129925) + (assign84680_body29_e129914 * (((locals.var_chi_dn7 / 4.0) * assign84680_body29_e129923) + (assign84680_body29_e129918 * (locals.var_chi_dn7 / 5.0))))))), (((((locals.var_chi_dn8 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn8)) / 2.0) * assign84680_body29_e129927) + (assign84680_body29_e129910 * (((locals.var_chi_dn8 / 3.0) * assign84680_body29_e129925) + (assign84680_body29_e129914 * (((locals.var_chi_dn8 / 4.0) * assign84680_body29_e129923) + (assign84680_body29_e129918 * (locals.var_chi_dn8 / 5.0))))))), (((((locals.var_chi_dn9 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn9)) / 2.0) * assign84680_body29_e129927) + (assign84680_body29_e129910 * (((locals.var_chi_dn9 / 3.0) * assign84680_body29_e129925) + (assign84680_body29_e129914 * (((locals.var_chi_dn9 / 4.0) * assign84680_body29_e129923) + (assign84680_body29_e129918 * (locals.var_chi_dn9 / 5.0))))))), (((((locals.var_chi_dn10 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn10)) / 2.0) * assign84680_body29_e129927) + (assign84680_body29_e129910 * (((locals.var_chi_dn10 / 3.0) * assign84680_body29_e129925) + (assign84680_body29_e129914 * (((locals.var_chi_dn10 / 4.0) * assign84680_body29_e129923) + (assign84680_body29_e129918 * (locals.var_chi_dn10 / 5.0))))))), (((((locals.var_chi_dn13 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn13)) / 2.0) * assign84680_body29_e129927) + (assign84680_body29_e129910 * (((locals.var_chi_dn13 / 3.0) * assign84680_body29_e129925) + (assign84680_body29_e129914 * (((locals.var_chi_dn13 / 4.0) * assign84680_body29_e129923) + (assign84680_body29_e129918 * (locals.var_chi_dn13 / 5.0))))))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
            locals.var_t0 = assign84680_body29_e129930;
            locals.var_t0_dn0 = assign84680_body29_e129930_d_n0;
            locals.var_t0_dn2 = assign84680_body29_e129930_d_n2;
            locals.var_t0_dn4 = assign84680_body29_e129930_d_n4;
            locals.var_t0_dn5 = assign84680_body29_e129930_d_n5;
            locals.var_t0_dn6 = assign84680_body29_e129930_d_n6;
            locals.var_t0_dn7 = assign84680_body29_e129930_d_n7;
            locals.var_t0_dn8 = assign84680_body29_e129930_d_n8;
            locals.var_t0_dn9 = assign84680_body29_e129930_d_n9;
            locals.var_t0_dn10 = assign84680_body29_e129930_d_n10;
            locals.var_t0_dn13 = assign84680_body29_e129930_d_n13;
            locals.var_t0_rv = 0.0;
            let (assign84680_body30_e129956, assign84680_body30_e129956_d_n0, assign84680_body30_e129956_d_n2, assign84680_body30_e129956_d_n4, assign84680_body30_e129956_d_n5, assign84680_body30_e129956_d_n6, assign84680_body30_e129956_d_n7, assign84680_body30_e129956_d_n8, assign84680_body30_e129956_d_n9, assign84680_body30_e129956_d_n10, assign84680_body30_e129956_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1971 != 0.0)) {
        let assign84680_body30_e129940: f64 = (locals.var_chi / 2.0);
        let assign84680_body30_e129944: f64 = (locals.var_chi / 3.0);
        let assign84680_body30_e129948: f64 = (locals.var_chi / 4.0);
        let assign84680_body30_e129949: f64 = (1.0 + assign84680_body30_e129948);
        let assign84680_body30_e129950: f64 = (assign84680_body30_e129944 * assign84680_body30_e129949);
        let assign84680_body30_e129951: f64 = (1.0 + assign84680_body30_e129950);
        let assign84680_body30_e129952: f64 = (assign84680_body30_e129940 * assign84680_body30_e129951);
        let assign84680_body30_e129953: f64 = (1.0 + assign84680_body30_e129952);
        let assign84680_body30_e129954: f64 = (locals.var_chi * assign84680_body30_e129953);
        (assign84680_body30_e129954, ((locals.var_chi_dn0 * assign84680_body30_e129953) + (locals.var_chi * (((locals.var_chi_dn0 / 2.0) * assign84680_body30_e129951) + (assign84680_body30_e129940 * (((locals.var_chi_dn0 / 3.0) * assign84680_body30_e129949) + (assign84680_body30_e129944 * (locals.var_chi_dn0 / 4.0))))))), ((locals.var_chi_dn2 * assign84680_body30_e129953) + (locals.var_chi * (((locals.var_chi_dn2 / 2.0) * assign84680_body30_e129951) + (assign84680_body30_e129940 * (((locals.var_chi_dn2 / 3.0) * assign84680_body30_e129949) + (assign84680_body30_e129944 * (locals.var_chi_dn2 / 4.0))))))), ((locals.var_chi_dn4 * assign84680_body30_e129953) + (locals.var_chi * (((locals.var_chi_dn4 / 2.0) * assign84680_body30_e129951) + (assign84680_body30_e129940 * (((locals.var_chi_dn4 / 3.0) * assign84680_body30_e129949) + (assign84680_body30_e129944 * (locals.var_chi_dn4 / 4.0))))))), ((locals.var_chi_dn5 * assign84680_body30_e129953) + (locals.var_chi * (((locals.var_chi_dn5 / 2.0) * assign84680_body30_e129951) + (assign84680_body30_e129940 * (((locals.var_chi_dn5 / 3.0) * assign84680_body30_e129949) + (assign84680_body30_e129944 * (locals.var_chi_dn5 / 4.0))))))), ((locals.var_chi_dn6 * assign84680_body30_e129953) + (locals.var_chi * (((locals.var_chi_dn6 / 2.0) * assign84680_body30_e129951) + (assign84680_body30_e129940 * (((locals.var_chi_dn6 / 3.0) * assign84680_body30_e129949) + (assign84680_body30_e129944 * (locals.var_chi_dn6 / 4.0))))))), ((locals.var_chi_dn7 * assign84680_body30_e129953) + (locals.var_chi * (((locals.var_chi_dn7 / 2.0) * assign84680_body30_e129951) + (assign84680_body30_e129940 * (((locals.var_chi_dn7 / 3.0) * assign84680_body30_e129949) + (assign84680_body30_e129944 * (locals.var_chi_dn7 / 4.0))))))), ((locals.var_chi_dn8 * assign84680_body30_e129953) + (locals.var_chi * (((locals.var_chi_dn8 / 2.0) * assign84680_body30_e129951) + (assign84680_body30_e129940 * (((locals.var_chi_dn8 / 3.0) * assign84680_body30_e129949) + (assign84680_body30_e129944 * (locals.var_chi_dn8 / 4.0))))))), ((locals.var_chi_dn9 * assign84680_body30_e129953) + (locals.var_chi * (((locals.var_chi_dn9 / 2.0) * assign84680_body30_e129951) + (assign84680_body30_e129940 * (((locals.var_chi_dn9 / 3.0) * assign84680_body30_e129949) + (assign84680_body30_e129944 * (locals.var_chi_dn9 / 4.0))))))), ((locals.var_chi_dn10 * assign84680_body30_e129953) + (locals.var_chi * (((locals.var_chi_dn10 / 2.0) * assign84680_body30_e129951) + (assign84680_body30_e129940 * (((locals.var_chi_dn10 / 3.0) * assign84680_body30_e129949) + (assign84680_body30_e129944 * (locals.var_chi_dn10 / 4.0))))))), ((locals.var_chi_dn13 * assign84680_body30_e129953) + (locals.var_chi * (((locals.var_chi_dn13 / 2.0) * assign84680_body30_e129951) + (assign84680_body30_e129940 * (((locals.var_chi_dn13 / 3.0) * assign84680_body30_e129949) + (assign84680_body30_e129944 * (locals.var_chi_dn13 / 4.0))))))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign84680_body30_e129956;
            locals.var_t1_dn0 = assign84680_body30_e129956_d_n0;
            locals.var_t1_dn2 = assign84680_body30_e129956_d_n2;
            locals.var_t1_dn4 = assign84680_body30_e129956_d_n4;
            locals.var_t1_dn5 = assign84680_body30_e129956_d_n5;
            locals.var_t1_dn6 = assign84680_body30_e129956_d_n6;
            locals.var_t1_dn7 = assign84680_body30_e129956_d_n7;
            locals.var_t1_dn8 = assign84680_body30_e129956_d_n8;
            locals.var_t1_dn9 = assign84680_body30_e129956_d_n9;
            locals.var_t1_dn10 = assign84680_body30_e129956_d_n10;
            locals.var_t1_dn13 = assign84680_body30_e129956_d_n13;
            locals.var_t1_rv = 0.0;
            let (assign84680_body31_e129966, assign84680_body31_e129966_d_n0, assign84680_body31_e129966_d_n2, assign84680_body31_e129966_d_n4, assign84680_body31_e129966_d_n5, assign84680_body31_e129966_d_n6, assign84680_body31_e129966_d_n7, assign84680_body31_e129966_d_n8, assign84680_body31_e129966_d_n9, assign84680_body31_e129966_d_n10, assign84680_body31_e129966_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1971 != 0.0)) {
        let assign84680_body31_e129964: f64 = (locals.var_cfs1 * locals.var_t0);
        (assign84680_body31_e129964, ((locals.var_cfs1_dn0 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn0)), ((locals.var_cfs1_dn2 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn2)), ((locals.var_cfs1_dn4 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn4)), ((locals.var_cfs1_dn5 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn5)), ((locals.var_cfs1_dn6 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn6)), ((locals.var_cfs1_dn7 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn7)), ((locals.var_cfs1_dn8 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn8)), ((locals.var_cfs1_dn9 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn9)), ((locals.var_cfs1_dn10 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn10)), ((locals.var_cfs1_dn13 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn13)),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn13,)
    }
};
            locals.var_fs01 = assign84680_body31_e129966;
            locals.var_fs01_dn0 = assign84680_body31_e129966_d_n0;
            locals.var_fs01_dn2 = assign84680_body31_e129966_d_n2;
            locals.var_fs01_dn4 = assign84680_body31_e129966_d_n4;
            locals.var_fs01_dn5 = assign84680_body31_e129966_d_n5;
            locals.var_fs01_dn6 = assign84680_body31_e129966_d_n6;
            locals.var_fs01_dn7 = assign84680_body31_e129966_d_n7;
            locals.var_fs01_dn8 = assign84680_body31_e129966_d_n8;
            locals.var_fs01_dn9 = assign84680_body31_e129966_d_n9;
            locals.var_fs01_dn10 = assign84680_body31_e129966_d_n10;
            locals.var_fs01_dn13 = assign84680_body31_e129966_d_n13;
            locals.var_fs01_rv = 0.0;
            let (assign84680_body32_e129978, assign84680_body32_e129978_d_n0, assign84680_body32_e129978_d_n2, assign84680_body32_e129978_d_n4, assign84680_body32_e129978_d_n5, assign84680_body32_e129978_d_n6, assign84680_body32_e129978_d_n7, assign84680_body32_e129978_d_n8, assign84680_body32_e129978_d_n9, assign84680_body32_e129978_d_n10, assign84680_body32_e129978_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1971 != 0.0)) {
        let assign84680_body32_e129974: f64 = (locals.var_cfs1 * locals.var_t1);
        let assign84680_body32_e129976: f64 = (assign84680_body32_e129974 * locals.var_beta);
        (assign84680_body32_e129976, ((((locals.var_cfs1_dn0 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn0)) * locals.var_beta) + (assign84680_body32_e129974 * locals.var_beta_dn0)), ((((locals.var_cfs1_dn2 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn2)) * locals.var_beta) + (assign84680_body32_e129974 * locals.var_beta_dn2)), ((((locals.var_cfs1_dn4 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn4)) * locals.var_beta) + (assign84680_body32_e129974 * locals.var_beta_dn4)), ((((locals.var_cfs1_dn5 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn5)) * locals.var_beta) + (assign84680_body32_e129974 * locals.var_beta_dn5)), ((((locals.var_cfs1_dn6 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn6)) * locals.var_beta) + (assign84680_body32_e129974 * locals.var_beta_dn6)), ((((locals.var_cfs1_dn7 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn7)) * locals.var_beta) + (assign84680_body32_e129974 * locals.var_beta_dn7)), ((((locals.var_cfs1_dn8 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn8)) * locals.var_beta) + (assign84680_body32_e129974 * locals.var_beta_dn8)), ((((locals.var_cfs1_dn9 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn9)) * locals.var_beta) + (assign84680_body32_e129974 * locals.var_beta_dn9)), ((((locals.var_cfs1_dn10 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn10)) * locals.var_beta) + (assign84680_body32_e129974 * locals.var_beta_dn10)), ((((locals.var_cfs1_dn13 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn13)) * locals.var_beta) + (assign84680_body32_e129974 * locals.var_beta_dn13)),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn13,)
    }
};
            locals.var_fs01_dps0 = assign84680_body32_e129978;
            locals.var_fs01_dps0_dn0 = assign84680_body32_e129978_d_n0;
            locals.var_fs01_dps0_dn2 = assign84680_body32_e129978_d_n2;
            locals.var_fs01_dps0_dn4 = assign84680_body32_e129978_d_n4;
            locals.var_fs01_dps0_dn5 = assign84680_body32_e129978_d_n5;
            locals.var_fs01_dps0_dn6 = assign84680_body32_e129978_d_n6;
            locals.var_fs01_dps0_dn7 = assign84680_body32_e129978_d_n7;
            locals.var_fs01_dps0_dn8 = assign84680_body32_e129978_d_n8;
            locals.var_fs01_dps0_dn9 = assign84680_body32_e129978_d_n9;
            locals.var_fs01_dps0_dn10 = assign84680_body32_e129978_d_n10;
            locals.var_fs01_dps0_dn13 = assign84680_body32_e129978_d_n13;
            locals.var_fs01_dps0_rv = 0.0;
            let assign84680_body33_e129980: f64 = (locals.var_chi).abs();
            let assign84680_body33_e129982: f64 = if assign84680_body33_e129980 < 60.0 { 1.0 } else { 0.0 };
            locals.var_guard1972 = assign84680_body33_e129982;
            locals.var_guard1972_rv = 0.0;
            let (assign84680_body35_e130013, assign84680_body35_e130013_d_n0, assign84680_body35_e130013_d_n2, assign84680_body35_e130013_d_n4, assign84680_body35_e130013_d_n5, assign84680_body35_e130013_d_n6, assign84680_body35_e130013_d_n7, assign84680_body35_e130013_d_n8, assign84680_body35_e130013_d_n9, assign84680_body35_e130013_d_n10, assign84680_body35_e130013_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1971 == 0.0)) && (locals.var_guard1972 != 0.0)) {
        let assign84680_body35_e130011: f64 = (locals.var_chi).exp();
        (assign84680_body35_e130011, (assign84680_body35_e130011 * locals.var_chi_dn0), (assign84680_body35_e130011 * locals.var_chi_dn2), (assign84680_body35_e130011 * locals.var_chi_dn4), (assign84680_body35_e130011 * locals.var_chi_dn5), (assign84680_body35_e130011 * locals.var_chi_dn6), (assign84680_body35_e130011 * locals.var_chi_dn7), (assign84680_body35_e130011 * locals.var_chi_dn8), (assign84680_body35_e130011 * locals.var_chi_dn9), (assign84680_body35_e130011 * locals.var_chi_dn10), (assign84680_body35_e130011 * locals.var_chi_dn13),)
    } else {
        (locals.var_exp_chi, locals.var_exp_chi_dn0, locals.var_exp_chi_dn2, locals.var_exp_chi_dn4, locals.var_exp_chi_dn5, locals.var_exp_chi_dn6, locals.var_exp_chi_dn7, locals.var_exp_chi_dn8, locals.var_exp_chi_dn9, locals.var_exp_chi_dn10, locals.var_exp_chi_dn13,)
    }
};
            locals.var_exp_chi = assign84680_body35_e130013;
            locals.var_exp_chi_dn0 = assign84680_body35_e130013_d_n0;
            locals.var_exp_chi_dn2 = assign84680_body35_e130013_d_n2;
            locals.var_exp_chi_dn4 = assign84680_body35_e130013_d_n4;
            locals.var_exp_chi_dn5 = assign84680_body35_e130013_d_n5;
            locals.var_exp_chi_dn6 = assign84680_body35_e130013_d_n6;
            locals.var_exp_chi_dn7 = assign84680_body35_e130013_d_n7;
            locals.var_exp_chi_dn8 = assign84680_body35_e130013_d_n8;
            locals.var_exp_chi_dn9 = assign84680_body35_e130013_d_n9;
            locals.var_exp_chi_dn10 = assign84680_body35_e130013_d_n10;
            locals.var_exp_chi_dn13 = assign84680_body35_e130013_d_n13;
            locals.var_exp_chi_rv = 0.0;
            let (assign84680_body36_e130026, assign84680_body36_e130026_d_n0, assign84680_body36_e130026_d_n2, assign84680_body36_e130026_d_n4, assign84680_body36_e130026_d_n5, assign84680_body36_e130026_d_n6, assign84680_body36_e130026_d_n7, assign84680_body36_e130026_d_n8, assign84680_body36_e130026_d_n9, assign84680_body36_e130026_d_n10, assign84680_body36_e130026_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1971 == 0.0)) && (locals.var_guard1972 != 0.0)) {
        let assign84680_body36_e130024: f64 = (locals.var_exp_chi - 1.0);
        (assign84680_body36_e130024, locals.var_exp_chi_dn0, locals.var_exp_chi_dn2, locals.var_exp_chi_dn4, locals.var_exp_chi_dn5, locals.var_exp_chi_dn6, locals.var_exp_chi_dn7, locals.var_exp_chi_dn8, locals.var_exp_chi_dn9, locals.var_exp_chi_dn10, locals.var_exp_chi_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign84680_body36_e130026;
            locals.var_t1_dn0 = assign84680_body36_e130026_d_n0;
            locals.var_t1_dn2 = assign84680_body36_e130026_d_n2;
            locals.var_t1_dn4 = assign84680_body36_e130026_d_n4;
            locals.var_t1_dn5 = assign84680_body36_e130026_d_n5;
            locals.var_t1_dn6 = assign84680_body36_e130026_d_n6;
            locals.var_t1_dn7 = assign84680_body36_e130026_d_n7;
            locals.var_t1_dn8 = assign84680_body36_e130026_d_n8;
            locals.var_t1_dn9 = assign84680_body36_e130026_d_n9;
            locals.var_t1_dn10 = assign84680_body36_e130026_d_n10;
            locals.var_t1_dn13 = assign84680_body36_e130026_d_n13;
            locals.var_t1_rv = 0.0;
            let (assign84680_body37_e130041, assign84680_body37_e130041_d_n0, assign84680_body37_e130041_d_n2, assign84680_body37_e130041_d_n4, assign84680_body37_e130041_d_n5, assign84680_body37_e130041_d_n6, assign84680_body37_e130041_d_n7, assign84680_body37_e130041_d_n8, assign84680_body37_e130041_d_n9, assign84680_body37_e130041_d_n10, assign84680_body37_e130041_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1971 == 0.0)) && (locals.var_guard1972 != 0.0)) {
        let assign84680_body37_e130038: f64 = (locals.var_t1 - locals.var_chi);
        let assign84680_body37_e130039: f64 = (locals.var_cfs1 * assign84680_body37_e130038);
        (assign84680_body37_e130039, ((locals.var_cfs1_dn0 * assign84680_body37_e130038) + (locals.var_cfs1 * (locals.var_t1_dn0 - locals.var_chi_dn0))), ((locals.var_cfs1_dn2 * assign84680_body37_e130038) + (locals.var_cfs1 * (locals.var_t1_dn2 - locals.var_chi_dn2))), ((locals.var_cfs1_dn4 * assign84680_body37_e130038) + (locals.var_cfs1 * (locals.var_t1_dn4 - locals.var_chi_dn4))), ((locals.var_cfs1_dn5 * assign84680_body37_e130038) + (locals.var_cfs1 * (locals.var_t1_dn5 - locals.var_chi_dn5))), ((locals.var_cfs1_dn6 * assign84680_body37_e130038) + (locals.var_cfs1 * (locals.var_t1_dn6 - locals.var_chi_dn6))), ((locals.var_cfs1_dn7 * assign84680_body37_e130038) + (locals.var_cfs1 * (locals.var_t1_dn7 - locals.var_chi_dn7))), ((locals.var_cfs1_dn8 * assign84680_body37_e130038) + (locals.var_cfs1 * (locals.var_t1_dn8 - locals.var_chi_dn8))), ((locals.var_cfs1_dn9 * assign84680_body37_e130038) + (locals.var_cfs1 * (locals.var_t1_dn9 - locals.var_chi_dn9))), ((locals.var_cfs1_dn10 * assign84680_body37_e130038) + (locals.var_cfs1 * (locals.var_t1_dn10 - locals.var_chi_dn10))), ((locals.var_cfs1_dn13 * assign84680_body37_e130038) + (locals.var_cfs1 * (locals.var_t1_dn13 - locals.var_chi_dn13))),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn13,)
    }
};
            locals.var_fs01 = assign84680_body37_e130041;
            locals.var_fs01_dn0 = assign84680_body37_e130041_d_n0;
            locals.var_fs01_dn2 = assign84680_body37_e130041_d_n2;
            locals.var_fs01_dn4 = assign84680_body37_e130041_d_n4;
            locals.var_fs01_dn5 = assign84680_body37_e130041_d_n5;
            locals.var_fs01_dn6 = assign84680_body37_e130041_d_n6;
            locals.var_fs01_dn7 = assign84680_body37_e130041_d_n7;
            locals.var_fs01_dn8 = assign84680_body37_e130041_d_n8;
            locals.var_fs01_dn9 = assign84680_body37_e130041_d_n9;
            locals.var_fs01_dn10 = assign84680_body37_e130041_d_n10;
            locals.var_fs01_dn13 = assign84680_body37_e130041_d_n13;
            locals.var_fs01_rv = 0.0;
            let (assign84680_body38_e130056, assign84680_body38_e130056_d_n0, assign84680_body38_e130056_d_n2, assign84680_body38_e130056_d_n4, assign84680_body38_e130056_d_n5, assign84680_body38_e130056_d_n6, assign84680_body38_e130056_d_n7, assign84680_body38_e130056_d_n8, assign84680_body38_e130056_d_n9, assign84680_body38_e130056_d_n10, assign84680_body38_e130056_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1971 == 0.0)) && (locals.var_guard1972 != 0.0)) {
        let assign84680_body38_e130052: f64 = (locals.var_cfs1 * locals.var_beta);
        let assign84680_body38_e130054: f64 = (assign84680_body38_e130052 * locals.var_t1);
        (assign84680_body38_e130054, ((((locals.var_cfs1_dn0 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn0)) * locals.var_t1) + (assign84680_body38_e130052 * locals.var_t1_dn0)), ((((locals.var_cfs1_dn2 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn2)) * locals.var_t1) + (assign84680_body38_e130052 * locals.var_t1_dn2)), ((((locals.var_cfs1_dn4 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn4)) * locals.var_t1) + (assign84680_body38_e130052 * locals.var_t1_dn4)), ((((locals.var_cfs1_dn5 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn5)) * locals.var_t1) + (assign84680_body38_e130052 * locals.var_t1_dn5)), ((((locals.var_cfs1_dn6 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn6)) * locals.var_t1) + (assign84680_body38_e130052 * locals.var_t1_dn6)), ((((locals.var_cfs1_dn7 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn7)) * locals.var_t1) + (assign84680_body38_e130052 * locals.var_t1_dn7)), ((((locals.var_cfs1_dn8 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn8)) * locals.var_t1) + (assign84680_body38_e130052 * locals.var_t1_dn8)), ((((locals.var_cfs1_dn9 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn9)) * locals.var_t1) + (assign84680_body38_e130052 * locals.var_t1_dn9)), ((((locals.var_cfs1_dn10 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn10)) * locals.var_t1) + (assign84680_body38_e130052 * locals.var_t1_dn10)), ((((locals.var_cfs1_dn13 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn13)) * locals.var_t1) + (assign84680_body38_e130052 * locals.var_t1_dn13)),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn13,)
    }
};
            locals.var_fs01_dps0 = assign84680_body38_e130056;
            locals.var_fs01_dps0_dn0 = assign84680_body38_e130056_d_n0;
            locals.var_fs01_dps0_dn2 = assign84680_body38_e130056_d_n2;
            locals.var_fs01_dps0_dn4 = assign84680_body38_e130056_d_n4;
            locals.var_fs01_dps0_dn5 = assign84680_body38_e130056_d_n5;
            locals.var_fs01_dps0_dn6 = assign84680_body38_e130056_d_n6;
            locals.var_fs01_dps0_dn7 = assign84680_body38_e130056_d_n7;
            locals.var_fs01_dps0_dn8 = assign84680_body38_e130056_d_n8;
            locals.var_fs01_dps0_dn9 = assign84680_body38_e130056_d_n9;
            locals.var_fs01_dps0_dn10 = assign84680_body38_e130056_d_n10;
            locals.var_fs01_dps0_dn13 = assign84680_body38_e130056_d_n13;
            locals.var_fs01_dps0_rv = 0.0;
            let (assign84680_body40_e130091, assign84680_body40_e130091_d_n0, assign84680_body40_e130091_d_n2, assign84680_body40_e130091_d_n4, assign84680_body40_e130091_d_n5, assign84680_body40_e130091_d_n6, assign84680_body40_e130091_d_n7, assign84680_body40_e130091_d_n8, assign84680_body40_e130091_d_n9, assign84680_body40_e130091_d_n10, assign84680_body40_e130091_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1971 == 0.0)) && (locals.var_guard1972 == 0.0)) {
        let assign84680_body40_e130088: f64 = (locals.var_beta * locals.var_ps0ld);
        let assign84680_body40_e130089: f64 = (assign84680_body40_e130088).exp();
        (assign84680_body40_e130089, (assign84680_body40_e130089 * ((locals.var_beta_dn0 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn0))), (assign84680_body40_e130089 * ((locals.var_beta_dn2 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn2))), (assign84680_body40_e130089 * ((locals.var_beta_dn4 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn4))), (assign84680_body40_e130089 * ((locals.var_beta_dn5 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn5))), (assign84680_body40_e130089 * ((locals.var_beta_dn6 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn6))), (assign84680_body40_e130089 * ((locals.var_beta_dn7 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn7))), (assign84680_body40_e130089 * ((locals.var_beta_dn8 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn8))), (assign84680_body40_e130089 * ((locals.var_beta_dn9 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn9))), (assign84680_body40_e130089 * ((locals.var_beta_dn10 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn10))), (assign84680_body40_e130089 * ((locals.var_beta_dn13 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn13))),)
    } else {
        (locals.var_exp_bps0, locals.var_exp_bps0_dn0, locals.var_exp_bps0_dn2, locals.var_exp_bps0_dn4, locals.var_exp_bps0_dn5, locals.var_exp_bps0_dn6, locals.var_exp_bps0_dn7, locals.var_exp_bps0_dn8, locals.var_exp_bps0_dn9, locals.var_exp_bps0_dn10, locals.var_exp_bps0_dn13,)
    }
};
            locals.var_exp_bps0 = assign84680_body40_e130091;
            locals.var_exp_bps0_dn0 = assign84680_body40_e130091_d_n0;
            locals.var_exp_bps0_dn2 = assign84680_body40_e130091_d_n2;
            locals.var_exp_bps0_dn4 = assign84680_body40_e130091_d_n4;
            locals.var_exp_bps0_dn5 = assign84680_body40_e130091_d_n5;
            locals.var_exp_bps0_dn6 = assign84680_body40_e130091_d_n6;
            locals.var_exp_bps0_dn7 = assign84680_body40_e130091_d_n7;
            locals.var_exp_bps0_dn8 = assign84680_body40_e130091_d_n8;
            locals.var_exp_bps0_dn9 = assign84680_body40_e130091_d_n9;
            locals.var_exp_bps0_dn10 = assign84680_body40_e130091_d_n10;
            locals.var_exp_bps0_dn13 = assign84680_body40_e130091_d_n13;
            locals.var_exp_bps0_rv = 0.0;
            let (assign84680_body41_e130111, assign84680_body41_e130111_d_n0, assign84680_body41_e130111_d_n2, assign84680_body41_e130111_d_n4, assign84680_body41_e130111_d_n5, assign84680_body41_e130111_d_n6, assign84680_body41_e130111_d_n7, assign84680_body41_e130111_d_n8, assign84680_body41_e130111_d_n9, assign84680_body41_e130111_d_n10, assign84680_body41_e130111_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1971 == 0.0)) && (locals.var_guard1972 == 0.0)) {
        let assign84680_body41_e130106: f64 = (locals.var_chi + 1.0);
        let assign84680_body41_e130107: f64 = (locals.var_exp_bvbs * assign84680_body41_e130106);
        let assign84680_body41_e130108: f64 = (locals.var_exp_bps0 - assign84680_body41_e130107);
        let assign84680_body41_e130109: f64 = (locals.var_cnst1over * assign84680_body41_e130108);
        (assign84680_body41_e130109, ((locals.var_cnst1over_dn0 * assign84680_body41_e130108) + (locals.var_cnst1over * (locals.var_exp_bps0_dn0 - ((locals.var_exp_bvbs_dn0 * assign84680_body41_e130106) + (locals.var_exp_bvbs * locals.var_chi_dn0))))), ((locals.var_cnst1over_dn2 * assign84680_body41_e130108) + (locals.var_cnst1over * (locals.var_exp_bps0_dn2 - ((locals.var_exp_bvbs_dn2 * assign84680_body41_e130106) + (locals.var_exp_bvbs * locals.var_chi_dn2))))), ((locals.var_cnst1over_dn4 * assign84680_body41_e130108) + (locals.var_cnst1over * (locals.var_exp_bps0_dn4 - ((locals.var_exp_bvbs_dn4 * assign84680_body41_e130106) + (locals.var_exp_bvbs * locals.var_chi_dn4))))), ((locals.var_cnst1over_dn5 * assign84680_body41_e130108) + (locals.var_cnst1over * (locals.var_exp_bps0_dn5 - ((locals.var_exp_bvbs_dn5 * assign84680_body41_e130106) + (locals.var_exp_bvbs * locals.var_chi_dn5))))), ((locals.var_cnst1over_dn6 * assign84680_body41_e130108) + (locals.var_cnst1over * (locals.var_exp_bps0_dn6 - ((locals.var_exp_bvbs_dn6 * assign84680_body41_e130106) + (locals.var_exp_bvbs * locals.var_chi_dn6))))), ((locals.var_cnst1over_dn7 * assign84680_body41_e130108) + (locals.var_cnst1over * (locals.var_exp_bps0_dn7 - ((locals.var_exp_bvbs_dn7 * assign84680_body41_e130106) + (locals.var_exp_bvbs * locals.var_chi_dn7))))), ((locals.var_cnst1over_dn8 * assign84680_body41_e130108) + (locals.var_cnst1over * (locals.var_exp_bps0_dn8 - ((locals.var_exp_bvbs_dn8 * assign84680_body41_e130106) + (locals.var_exp_bvbs * locals.var_chi_dn8))))), ((locals.var_cnst1over_dn9 * assign84680_body41_e130108) + (locals.var_cnst1over * (locals.var_exp_bps0_dn9 - ((locals.var_exp_bvbs_dn9 * assign84680_body41_e130106) + (locals.var_exp_bvbs * locals.var_chi_dn9))))), ((locals.var_cnst1over_dn10 * assign84680_body41_e130108) + (locals.var_cnst1over * (locals.var_exp_bps0_dn10 - ((locals.var_exp_bvbs_dn10 * assign84680_body41_e130106) + (locals.var_exp_bvbs * locals.var_chi_dn10))))), ((locals.var_cnst1over_dn13 * assign84680_body41_e130108) + (locals.var_cnst1over * (locals.var_exp_bps0_dn13 - ((locals.var_exp_bvbs_dn13 * assign84680_body41_e130106) + (locals.var_exp_bvbs * locals.var_chi_dn13))))),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn13,)
    }
};
            locals.var_fs01 = assign84680_body41_e130111;
            locals.var_fs01_dn0 = assign84680_body41_e130111_d_n0;
            locals.var_fs01_dn2 = assign84680_body41_e130111_d_n2;
            locals.var_fs01_dn4 = assign84680_body41_e130111_d_n4;
            locals.var_fs01_dn5 = assign84680_body41_e130111_d_n5;
            locals.var_fs01_dn6 = assign84680_body41_e130111_d_n6;
            locals.var_fs01_dn7 = assign84680_body41_e130111_d_n7;
            locals.var_fs01_dn8 = assign84680_body41_e130111_d_n8;
            locals.var_fs01_dn9 = assign84680_body41_e130111_d_n9;
            locals.var_fs01_dn10 = assign84680_body41_e130111_d_n10;
            locals.var_fs01_dn13 = assign84680_body41_e130111_d_n13;
            locals.var_fs01_rv = 0.0;
            let (assign84680_body42_e130129, assign84680_body42_e130129_d_n0, assign84680_body42_e130129_d_n2, assign84680_body42_e130129_d_n4, assign84680_body42_e130129_d_n5, assign84680_body42_e130129_d_n6, assign84680_body42_e130129_d_n7, assign84680_body42_e130129_d_n8, assign84680_body42_e130129_d_n9, assign84680_body42_e130129_d_n10, assign84680_body42_e130129_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1971 == 0.0)) && (locals.var_guard1972 == 0.0)) {
        let assign84680_body42_e130123: f64 = (locals.var_cnst1over * locals.var_beta);
        let assign84680_body42_e130126: f64 = (locals.var_exp_bps0 - locals.var_exp_bvbs);
        let assign84680_body42_e130127: f64 = (assign84680_body42_e130123 * assign84680_body42_e130126);
        (assign84680_body42_e130127, ((((locals.var_cnst1over_dn0 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn0)) * assign84680_body42_e130126) + (assign84680_body42_e130123 * (locals.var_exp_bps0_dn0 - locals.var_exp_bvbs_dn0))), ((((locals.var_cnst1over_dn2 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn2)) * assign84680_body42_e130126) + (assign84680_body42_e130123 * (locals.var_exp_bps0_dn2 - locals.var_exp_bvbs_dn2))), ((((locals.var_cnst1over_dn4 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn4)) * assign84680_body42_e130126) + (assign84680_body42_e130123 * (locals.var_exp_bps0_dn4 - locals.var_exp_bvbs_dn4))), ((((locals.var_cnst1over_dn5 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn5)) * assign84680_body42_e130126) + (assign84680_body42_e130123 * (locals.var_exp_bps0_dn5 - locals.var_exp_bvbs_dn5))), ((((locals.var_cnst1over_dn6 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn6)) * assign84680_body42_e130126) + (assign84680_body42_e130123 * (locals.var_exp_bps0_dn6 - locals.var_exp_bvbs_dn6))), ((((locals.var_cnst1over_dn7 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn7)) * assign84680_body42_e130126) + (assign84680_body42_e130123 * (locals.var_exp_bps0_dn7 - locals.var_exp_bvbs_dn7))), ((((locals.var_cnst1over_dn8 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn8)) * assign84680_body42_e130126) + (assign84680_body42_e130123 * (locals.var_exp_bps0_dn8 - locals.var_exp_bvbs_dn8))), ((((locals.var_cnst1over_dn9 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn9)) * assign84680_body42_e130126) + (assign84680_body42_e130123 * (locals.var_exp_bps0_dn9 - locals.var_exp_bvbs_dn9))), ((((locals.var_cnst1over_dn10 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn10)) * assign84680_body42_e130126) + (assign84680_body42_e130123 * (locals.var_exp_bps0_dn10 - locals.var_exp_bvbs_dn10))), ((((locals.var_cnst1over_dn13 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn13)) * assign84680_body42_e130126) + (assign84680_body42_e130123 * (locals.var_exp_bps0_dn13 - locals.var_exp_bvbs_dn13))),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn13,)
    }
};
            locals.var_fs01_dps0 = assign84680_body42_e130129;
            locals.var_fs01_dps0_dn0 = assign84680_body42_e130129_d_n0;
            locals.var_fs01_dps0_dn2 = assign84680_body42_e130129_d_n2;
            locals.var_fs01_dps0_dn4 = assign84680_body42_e130129_d_n4;
            locals.var_fs01_dps0_dn5 = assign84680_body42_e130129_d_n5;
            locals.var_fs01_dps0_dn6 = assign84680_body42_e130129_d_n6;
            locals.var_fs01_dps0_dn7 = assign84680_body42_e130129_d_n7;
            locals.var_fs01_dps0_dn8 = assign84680_body42_e130129_d_n8;
            locals.var_fs01_dps0_dn9 = assign84680_body42_e130129_d_n9;
            locals.var_fs01_dps0_dn10 = assign84680_body42_e130129_d_n10;
            locals.var_fs01_dps0_dn13 = assign84680_body42_e130129_d_n13;
            locals.var_fs01_dps0_rv = 0.0;
            let assign84680_body43_e130132: f64 = if locals.var_fs01 > 0.0 { 1.0 } else { 0.0 };
            locals.var_guard1973 = assign84680_body43_e130132;
            locals.var_guard1973_rv = 0.0;
            let (assign84680_body44_e130143, assign84680_body44_e130143_d_n0, assign84680_body44_e130143_d_n2, assign84680_body44_e130143_d_n4, assign84680_body44_e130143_d_n5, assign84680_body44_e130143_d_n6, assign84680_body44_e130143_d_n7, assign84680_body44_e130143_d_n8, assign84680_body44_e130143_d_n9, assign84680_body44_e130143_d_n10, assign84680_body44_e130143_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1973 != 0.0)) {
        let assign84680_body44_e130140: f64 = (locals.var_fbsq__blk1891 + locals.var_fs01);
        let assign84680_body44_e130141: f64 = (assign84680_body44_e130140).sqrt();
        (assign84680_body44_e130141, ((locals.var_fbsq__blk1891_dn0 + locals.var_fs01_dn0) / (2.0 * assign84680_body44_e130141)), ((locals.var_fbsq__blk1891_dn2 + locals.var_fs01_dn2) / (2.0 * assign84680_body44_e130141)), ((locals.var_fbsq__blk1891_dn4 + locals.var_fs01_dn4) / (2.0 * assign84680_body44_e130141)), ((locals.var_fbsq__blk1891_dn5 + locals.var_fs01_dn5) / (2.0 * assign84680_body44_e130141)), ((locals.var_fbsq__blk1891_dn6 + locals.var_fs01_dn6) / (2.0 * assign84680_body44_e130141)), ((locals.var_fbsq__blk1891_dn7 + locals.var_fs01_dn7) / (2.0 * assign84680_body44_e130141)), ((locals.var_fbsq__blk1891_dn8 + locals.var_fs01_dn8) / (2.0 * assign84680_body44_e130141)), ((locals.var_fbsq__blk1891_dn9 + locals.var_fs01_dn9) / (2.0 * assign84680_body44_e130141)), ((locals.var_fbsq__blk1891_dn10 + locals.var_fs01_dn10) / (2.0 * assign84680_body44_e130141)), ((locals.var_fbsq__blk1891_dn13 + locals.var_fs01_dn13) / (2.0 * assign84680_body44_e130141)),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn13,)
    }
};
            locals.var_fs02 = assign84680_body44_e130143;
            locals.var_fs02_dn0 = assign84680_body44_e130143_d_n0;
            locals.var_fs02_dn2 = assign84680_body44_e130143_d_n2;
            locals.var_fs02_dn4 = assign84680_body44_e130143_d_n4;
            locals.var_fs02_dn5 = assign84680_body44_e130143_d_n5;
            locals.var_fs02_dn6 = assign84680_body44_e130143_d_n6;
            locals.var_fs02_dn7 = assign84680_body44_e130143_d_n7;
            locals.var_fs02_dn8 = assign84680_body44_e130143_d_n8;
            locals.var_fs02_dn9 = assign84680_body44_e130143_d_n9;
            locals.var_fs02_dn10 = assign84680_body44_e130143_d_n10;
            locals.var_fs02_dn13 = assign84680_body44_e130143_d_n13;
            locals.var_fs02_rv = 0.0;
            let (assign84680_body45_e130157, assign84680_body45_e130157_d_n0, assign84680_body45_e130157_d_n2, assign84680_body45_e130157_d_n4, assign84680_body45_e130157_d_n5, assign84680_body45_e130157_d_n6, assign84680_body45_e130157_d_n7, assign84680_body45_e130157_d_n8, assign84680_body45_e130157_d_n9, assign84680_body45_e130157_d_n10, assign84680_body45_e130157_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1973 != 0.0)) {
        let assign84680_body45_e130152: f64 = (locals.var_fbsq_dpss__blk1892 + locals.var_fs01_dps0);
        let assign84680_body45_e130153: f64 = (0.5 * assign84680_body45_e130152);
        let assign84680_body45_e130155: f64 = (assign84680_body45_e130153 / locals.var_fs02);
        (assign84680_body45_e130155, ((((0.5 * (locals.var_fbsq_dpss__blk1892_dn0 + locals.var_fs01_dps0_dn0)) * locals.var_fs02) - (assign84680_body45_e130153 * locals.var_fs02_dn0)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk1892_dn2 + locals.var_fs01_dps0_dn2)) * locals.var_fs02) - (assign84680_body45_e130153 * locals.var_fs02_dn2)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk1892_dn4 + locals.var_fs01_dps0_dn4)) * locals.var_fs02) - (assign84680_body45_e130153 * locals.var_fs02_dn4)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk1892_dn5 + locals.var_fs01_dps0_dn5)) * locals.var_fs02) - (assign84680_body45_e130153 * locals.var_fs02_dn5)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk1892_dn6 + locals.var_fs01_dps0_dn6)) * locals.var_fs02) - (assign84680_body45_e130153 * locals.var_fs02_dn6)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk1892_dn7 + locals.var_fs01_dps0_dn7)) * locals.var_fs02) - (assign84680_body45_e130153 * locals.var_fs02_dn7)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk1892_dn8 + locals.var_fs01_dps0_dn8)) * locals.var_fs02) - (assign84680_body45_e130153 * locals.var_fs02_dn8)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk1892_dn9 + locals.var_fs01_dps0_dn9)) * locals.var_fs02) - (assign84680_body45_e130153 * locals.var_fs02_dn9)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk1892_dn10 + locals.var_fs01_dps0_dn10)) * locals.var_fs02) - (assign84680_body45_e130153 * locals.var_fs02_dn10)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk1892_dn13 + locals.var_fs01_dps0_dn13)) * locals.var_fs02) - (assign84680_body45_e130153 * locals.var_fs02_dn13)) / (locals.var_fs02 * locals.var_fs02)),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn13,)
    }
};
            locals.var_fs02_dps0 = assign84680_body45_e130157;
            locals.var_fs02_dps0_dn0 = assign84680_body45_e130157_d_n0;
            locals.var_fs02_dps0_dn2 = assign84680_body45_e130157_d_n2;
            locals.var_fs02_dps0_dn4 = assign84680_body45_e130157_d_n4;
            locals.var_fs02_dps0_dn5 = assign84680_body45_e130157_d_n5;
            locals.var_fs02_dps0_dn6 = assign84680_body45_e130157_d_n6;
            locals.var_fs02_dps0_dn7 = assign84680_body45_e130157_d_n7;
            locals.var_fs02_dps0_dn8 = assign84680_body45_e130157_d_n8;
            locals.var_fs02_dps0_dn9 = assign84680_body45_e130157_d_n9;
            locals.var_fs02_dps0_dn10 = assign84680_body45_e130157_d_n10;
            locals.var_fs02_dps0_dn13 = assign84680_body45_e130157_d_n13;
            locals.var_fs02_dps0_rv = 0.0;
            let assign84680_body46_e130160: f64 = if locals.var_fbsq__blk1891 > 0.0 { 1.0 } else { 0.0 };
            locals.var_guard1974 = assign84680_body46_e130160;
            locals.var_guard1974_rv = 0.0;
            let (assign84680_body47_e130172, assign84680_body47_e130172_d_n0, assign84680_body47_e130172_d_n2, assign84680_body47_e130172_d_n4, assign84680_body47_e130172_d_n5, assign84680_body47_e130172_d_n6, assign84680_body47_e130172_d_n7, assign84680_body47_e130172_d_n8, assign84680_body47_e130172_d_n9, assign84680_body47_e130172_d_n10, assign84680_body47_e130172_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1973 == 0.0)) && (locals.var_guard1974 != 0.0)) {
        let assign84680_body47_e130170: f64 = (locals.var_fbsq__blk1891).sqrt();
        (assign84680_body47_e130170, (locals.var_fbsq__blk1891_dn0 / (2.0 * assign84680_body47_e130170)), (locals.var_fbsq__blk1891_dn2 / (2.0 * assign84680_body47_e130170)), (locals.var_fbsq__blk1891_dn4 / (2.0 * assign84680_body47_e130170)), (locals.var_fbsq__blk1891_dn5 / (2.0 * assign84680_body47_e130170)), (locals.var_fbsq__blk1891_dn6 / (2.0 * assign84680_body47_e130170)), (locals.var_fbsq__blk1891_dn7 / (2.0 * assign84680_body47_e130170)), (locals.var_fbsq__blk1891_dn8 / (2.0 * assign84680_body47_e130170)), (locals.var_fbsq__blk1891_dn9 / (2.0 * assign84680_body47_e130170)), (locals.var_fbsq__blk1891_dn10 / (2.0 * assign84680_body47_e130170)), (locals.var_fbsq__blk1891_dn13 / (2.0 * assign84680_body47_e130170)),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn13,)
    }
};
            locals.var_fs02 = assign84680_body47_e130172;
            locals.var_fs02_dn0 = assign84680_body47_e130172_d_n0;
            locals.var_fs02_dn2 = assign84680_body47_e130172_d_n2;
            locals.var_fs02_dn4 = assign84680_body47_e130172_d_n4;
            locals.var_fs02_dn5 = assign84680_body47_e130172_d_n5;
            locals.var_fs02_dn6 = assign84680_body47_e130172_d_n6;
            locals.var_fs02_dn7 = assign84680_body47_e130172_d_n7;
            locals.var_fs02_dn8 = assign84680_body47_e130172_d_n8;
            locals.var_fs02_dn9 = assign84680_body47_e130172_d_n9;
            locals.var_fs02_dn10 = assign84680_body47_e130172_d_n10;
            locals.var_fs02_dn13 = assign84680_body47_e130172_d_n13;
            locals.var_fs02_rv = 0.0;
            let (assign84680_body48_e130187, assign84680_body48_e130187_d_n0, assign84680_body48_e130187_d_n2, assign84680_body48_e130187_d_n4, assign84680_body48_e130187_d_n5, assign84680_body48_e130187_d_n6, assign84680_body48_e130187_d_n7, assign84680_body48_e130187_d_n8, assign84680_body48_e130187_d_n9, assign84680_body48_e130187_d_n10, assign84680_body48_e130187_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1973 == 0.0)) && (locals.var_guard1974 != 0.0)) {
        let assign84680_body48_e130183: f64 = (0.5 * locals.var_fbsq_dpss__blk1892);
        let assign84680_body48_e130185: f64 = (assign84680_body48_e130183 / locals.var_fs02);
        (assign84680_body48_e130185, ((((0.5 * locals.var_fbsq_dpss__blk1892_dn0) * locals.var_fs02) - (assign84680_body48_e130183 * locals.var_fs02_dn0)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk1892_dn2) * locals.var_fs02) - (assign84680_body48_e130183 * locals.var_fs02_dn2)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk1892_dn4) * locals.var_fs02) - (assign84680_body48_e130183 * locals.var_fs02_dn4)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk1892_dn5) * locals.var_fs02) - (assign84680_body48_e130183 * locals.var_fs02_dn5)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk1892_dn6) * locals.var_fs02) - (assign84680_body48_e130183 * locals.var_fs02_dn6)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk1892_dn7) * locals.var_fs02) - (assign84680_body48_e130183 * locals.var_fs02_dn7)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk1892_dn8) * locals.var_fs02) - (assign84680_body48_e130183 * locals.var_fs02_dn8)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk1892_dn9) * locals.var_fs02) - (assign84680_body48_e130183 * locals.var_fs02_dn9)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk1892_dn10) * locals.var_fs02) - (assign84680_body48_e130183 * locals.var_fs02_dn10)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk1892_dn13) * locals.var_fs02) - (assign84680_body48_e130183 * locals.var_fs02_dn13)) / (locals.var_fs02 * locals.var_fs02)),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn13,)
    }
};
            locals.var_fs02_dps0 = assign84680_body48_e130187;
            locals.var_fs02_dps0_dn0 = assign84680_body48_e130187_d_n0;
            locals.var_fs02_dps0_dn2 = assign84680_body48_e130187_d_n2;
            locals.var_fs02_dps0_dn4 = assign84680_body48_e130187_d_n4;
            locals.var_fs02_dps0_dn5 = assign84680_body48_e130187_d_n5;
            locals.var_fs02_dps0_dn6 = assign84680_body48_e130187_d_n6;
            locals.var_fs02_dps0_dn7 = assign84680_body48_e130187_d_n7;
            locals.var_fs02_dps0_dn8 = assign84680_body48_e130187_d_n8;
            locals.var_fs02_dps0_dn9 = assign84680_body48_e130187_d_n9;
            locals.var_fs02_dps0_dn10 = assign84680_body48_e130187_d_n10;
            locals.var_fs02_dps0_dn13 = assign84680_body48_e130187_d_n13;
            locals.var_fs02_dps0_rv = 0.0;
            let (assign84680_body49_e130199, assign84680_body49_e130199_d_n0, assign84680_body49_e130199_d_n2, assign84680_body49_e130199_d_n4, assign84680_body49_e130199_d_n5, assign84680_body49_e130199_d_n6, assign84680_body49_e130199_d_n7, assign84680_body49_e130199_d_n8, assign84680_body49_e130199_d_n9, assign84680_body49_e130199_d_n10, assign84680_body49_e130199_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1973 == 0.0)) && (locals.var_guard1974 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn13,)
    }
};
            locals.var_fs02 = assign84680_body49_e130199;
            locals.var_fs02_dn0 = assign84680_body49_e130199_d_n0;
            locals.var_fs02_dn2 = assign84680_body49_e130199_d_n2;
            locals.var_fs02_dn4 = assign84680_body49_e130199_d_n4;
            locals.var_fs02_dn5 = assign84680_body49_e130199_d_n5;
            locals.var_fs02_dn6 = assign84680_body49_e130199_d_n6;
            locals.var_fs02_dn7 = assign84680_body49_e130199_d_n7;
            locals.var_fs02_dn8 = assign84680_body49_e130199_d_n8;
            locals.var_fs02_dn9 = assign84680_body49_e130199_d_n9;
            locals.var_fs02_dn10 = assign84680_body49_e130199_d_n10;
            locals.var_fs02_dn13 = assign84680_body49_e130199_d_n13;
            locals.var_fs02_rv = 0.0;
            let (assign84680_body50_e130211, assign84680_body50_e130211_d_n0, assign84680_body50_e130211_d_n2, assign84680_body50_e130211_d_n4, assign84680_body50_e130211_d_n5, assign84680_body50_e130211_d_n6, assign84680_body50_e130211_d_n7, assign84680_body50_e130211_d_n8, assign84680_body50_e130211_d_n9, assign84680_body50_e130211_d_n10, assign84680_body50_e130211_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1973 == 0.0)) && (locals.var_guard1974 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn13,)
    }
};
            locals.var_fs02_dps0 = assign84680_body50_e130211;
            locals.var_fs02_dps0_dn0 = assign84680_body50_e130211_d_n0;
            locals.var_fs02_dps0_dn2 = assign84680_body50_e130211_d_n2;
            locals.var_fs02_dps0_dn4 = assign84680_body50_e130211_d_n4;
            locals.var_fs02_dps0_dn5 = assign84680_body50_e130211_d_n5;
            locals.var_fs02_dps0_dn6 = assign84680_body50_e130211_d_n6;
            locals.var_fs02_dps0_dn7 = assign84680_body50_e130211_d_n7;
            locals.var_fs02_dps0_dn8 = assign84680_body50_e130211_d_n8;
            locals.var_fs02_dps0_dn9 = assign84680_body50_e130211_d_n9;
            locals.var_fs02_dps0_dn10 = assign84680_body50_e130211_d_n10;
            locals.var_fs02_dps0_dn13 = assign84680_body50_e130211_d_n13;
            locals.var_fs02_dps0_rv = 0.0;
            let (assign84680_body51_e130225, assign84680_body51_e130225_d_n0, assign84680_body51_e130225_d_n2, assign84680_body51_e130225_d_n4, assign84680_body51_e130225_d_n5, assign84680_body51_e130225_d_n6, assign84680_body51_e130225_d_n7, assign84680_body51_e130225_d_n8, assign84680_body51_e130225_d_n9, assign84680_body51_e130225_d_n10, assign84680_body51_e130225_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) {
        let (assign84680_body51_e130221,) = {
            if (locals.var_chi >= 0.0) {
                (1.0,)
            } else {
                let assign84680_body51_e130220: f64 = (-1.0);
                (assign84680_body51_e130220,)
            }
        };
        let assign84680_body51_e130223: f64 = (assign84680_body51_e130221 * locals.var_fs02);
        (assign84680_body51_e130223, (assign84680_body51_e130221 * locals.var_fs02_dn0), (assign84680_body51_e130221 * locals.var_fs02_dn2), (assign84680_body51_e130221 * locals.var_fs02_dn4), (assign84680_body51_e130221 * locals.var_fs02_dn5), (assign84680_body51_e130221 * locals.var_fs02_dn6), (assign84680_body51_e130221 * locals.var_fs02_dn7), (assign84680_body51_e130221 * locals.var_fs02_dn8), (assign84680_body51_e130221 * locals.var_fs02_dn9), (assign84680_body51_e130221 * locals.var_fs02_dn10), (assign84680_body51_e130221 * locals.var_fs02_dn13),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn13,)
    }
};
            locals.var_fs02 = assign84680_body51_e130225;
            locals.var_fs02_dn0 = assign84680_body51_e130225_d_n0;
            locals.var_fs02_dn2 = assign84680_body51_e130225_d_n2;
            locals.var_fs02_dn4 = assign84680_body51_e130225_d_n4;
            locals.var_fs02_dn5 = assign84680_body51_e130225_d_n5;
            locals.var_fs02_dn6 = assign84680_body51_e130225_d_n6;
            locals.var_fs02_dn7 = assign84680_body51_e130225_d_n7;
            locals.var_fs02_dn8 = assign84680_body51_e130225_d_n8;
            locals.var_fs02_dn9 = assign84680_body51_e130225_d_n9;
            locals.var_fs02_dn10 = assign84680_body51_e130225_d_n10;
            locals.var_fs02_dn13 = assign84680_body51_e130225_d_n13;
            locals.var_fs02_rv = 0.0;
            let (assign84680_body52_e130239, assign84680_body52_e130239_d_n0, assign84680_body52_e130239_d_n2, assign84680_body52_e130239_d_n4, assign84680_body52_e130239_d_n5, assign84680_body52_e130239_d_n6, assign84680_body52_e130239_d_n7, assign84680_body52_e130239_d_n8, assign84680_body52_e130239_d_n9, assign84680_body52_e130239_d_n10, assign84680_body52_e130239_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) {
        let (assign84680_body52_e130235,) = {
            if (locals.var_chi >= 0.0) {
                (1.0,)
            } else {
                let assign84680_body52_e130234: f64 = (-1.0);
                (assign84680_body52_e130234,)
            }
        };
        let assign84680_body52_e130237: f64 = (assign84680_body52_e130235 * locals.var_fs02_dps0);
        (assign84680_body52_e130237, (assign84680_body52_e130235 * locals.var_fs02_dps0_dn0), (assign84680_body52_e130235 * locals.var_fs02_dps0_dn2), (assign84680_body52_e130235 * locals.var_fs02_dps0_dn4), (assign84680_body52_e130235 * locals.var_fs02_dps0_dn5), (assign84680_body52_e130235 * locals.var_fs02_dps0_dn6), (assign84680_body52_e130235 * locals.var_fs02_dps0_dn7), (assign84680_body52_e130235 * locals.var_fs02_dps0_dn8), (assign84680_body52_e130235 * locals.var_fs02_dps0_dn9), (assign84680_body52_e130235 * locals.var_fs02_dps0_dn10), (assign84680_body52_e130235 * locals.var_fs02_dps0_dn13),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn13,)
    }
};
            locals.var_fs02_dps0 = assign84680_body52_e130239;
            locals.var_fs02_dps0_dn0 = assign84680_body52_e130239_d_n0;
            locals.var_fs02_dps0_dn2 = assign84680_body52_e130239_d_n2;
            locals.var_fs02_dps0_dn4 = assign84680_body52_e130239_d_n4;
            locals.var_fs02_dps0_dn5 = assign84680_body52_e130239_d_n5;
            locals.var_fs02_dps0_dn6 = assign84680_body52_e130239_d_n6;
            locals.var_fs02_dps0_dn7 = assign84680_body52_e130239_d_n7;
            locals.var_fs02_dps0_dn8 = assign84680_body52_e130239_d_n8;
            locals.var_fs02_dps0_dn9 = assign84680_body52_e130239_d_n9;
            locals.var_fs02_dps0_dn10 = assign84680_body52_e130239_d_n10;
            locals.var_fs02_dps0_dn13 = assign84680_body52_e130239_d_n13;
            locals.var_fs02_dps0_rv = 0.0;
            let (assign84680_body53_e130252, assign84680_body53_e130252_d_n0, assign84680_body53_e130252_d_n2, assign84680_body53_e130252_d_n4, assign84680_body53_e130252_d_n5, assign84680_body53_e130252_d_n6, assign84680_body53_e130252_d_n7, assign84680_body53_e130252_d_n8, assign84680_body53_e130252_d_n9, assign84680_body53_e130252_d_n10, assign84680_body53_e130252_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) {
        let assign84680_body53_e130244: f64 = (-locals.var_vgpld);
        let assign84680_body53_e130246: f64 = (assign84680_body53_e130244 + locals.var_ps0ld);
        let assign84680_body53_e130249: f64 = (locals.var_fac1 * locals.var_fs02);
        let assign84680_body53_e130250: f64 = (assign84680_body53_e130246 + assign84680_body53_e130249);
        (assign84680_body53_e130250, (locals.var_ps0ld_dn0 + ((locals.var_fac1_dn0 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn0))), (((-locals.var_vgpld_dn2) + locals.var_ps0ld_dn2) + ((locals.var_fac1_dn2 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn2))), (locals.var_ps0ld_dn4 + ((locals.var_fac1_dn4 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn4))), (locals.var_ps0ld_dn5 + ((locals.var_fac1_dn5 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn5))), (((-locals.var_vgpld_dn6) + locals.var_ps0ld_dn6) + ((locals.var_fac1_dn6 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn6))), (((-locals.var_vgpld_dn7) + locals.var_ps0ld_dn7) + ((locals.var_fac1_dn7 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn7))), (((-locals.var_vgpld_dn8) + locals.var_ps0ld_dn8) + ((locals.var_fac1_dn8 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn8))), (locals.var_ps0ld_dn9 + ((locals.var_fac1_dn9 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn9))), (locals.var_ps0ld_dn10 + ((locals.var_fac1_dn10 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn10))), (locals.var_ps0ld_dn13 + ((locals.var_fac1_dn13 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn13))),)
    } else {
        (locals.var_fs0, locals.var_fs0_dn0, locals.var_fs0_dn2, locals.var_fs0_dn4, locals.var_fs0_dn5, locals.var_fs0_dn6, locals.var_fs0_dn7, locals.var_fs0_dn8, locals.var_fs0_dn9, locals.var_fs0_dn10, locals.var_fs0_dn13,)
    }
};
            locals.var_fs0 = assign84680_body53_e130252;
            locals.var_fs0_dn0 = assign84680_body53_e130252_d_n0;
            locals.var_fs0_dn2 = assign84680_body53_e130252_d_n2;
            locals.var_fs0_dn4 = assign84680_body53_e130252_d_n4;
            locals.var_fs0_dn5 = assign84680_body53_e130252_d_n5;
            locals.var_fs0_dn6 = assign84680_body53_e130252_d_n6;
            locals.var_fs0_dn7 = assign84680_body53_e130252_d_n7;
            locals.var_fs0_dn8 = assign84680_body53_e130252_d_n8;
            locals.var_fs0_dn9 = assign84680_body53_e130252_d_n9;
            locals.var_fs0_dn10 = assign84680_body53_e130252_d_n10;
            locals.var_fs0_dn13 = assign84680_body53_e130252_d_n13;
            locals.var_fs0_rv = 0.0;
            let (assign84680_body54_e130262, assign84680_body54_e130262_d_n0, assign84680_body54_e130262_d_n2, assign84680_body54_e130262_d_n4, assign84680_body54_e130262_d_n5, assign84680_body54_e130262_d_n6, assign84680_body54_e130262_d_n7, assign84680_body54_e130262_d_n8, assign84680_body54_e130262_d_n9, assign84680_body54_e130262_d_n10, assign84680_body54_e130262_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) {
        let assign84680_body54_e130259: f64 = (locals.var_fac1 * locals.var_fs02_dps0);
        let assign84680_body54_e130260: f64 = (1.0 + assign84680_body54_e130259);
        (assign84680_body54_e130260, ((locals.var_fac1_dn0 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn0)), ((locals.var_fac1_dn2 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn2)), ((locals.var_fac1_dn4 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn4)), ((locals.var_fac1_dn5 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn5)), ((locals.var_fac1_dn6 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn6)), ((locals.var_fac1_dn7 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn7)), ((locals.var_fac1_dn8 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn8)), ((locals.var_fac1_dn9 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn9)), ((locals.var_fac1_dn10 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn10)), ((locals.var_fac1_dn13 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn13)),)
    } else {
        (locals.var_fs0_dps0, locals.var_fs0_dps0_dn0, locals.var_fs0_dps0_dn2, locals.var_fs0_dps0_dn4, locals.var_fs0_dps0_dn5, locals.var_fs0_dps0_dn6, locals.var_fs0_dps0_dn7, locals.var_fs0_dps0_dn8, locals.var_fs0_dps0_dn9, locals.var_fs0_dps0_dn10, locals.var_fs0_dps0_dn13,)
    }
};
            locals.var_fs0_dps0 = assign84680_body54_e130262;
            locals.var_fs0_dps0_dn0 = assign84680_body54_e130262_d_n0;
            locals.var_fs0_dps0_dn2 = assign84680_body54_e130262_d_n2;
            locals.var_fs0_dps0_dn4 = assign84680_body54_e130262_d_n4;
            locals.var_fs0_dps0_dn5 = assign84680_body54_e130262_d_n5;
            locals.var_fs0_dps0_dn6 = assign84680_body54_e130262_d_n6;
            locals.var_fs0_dps0_dn7 = assign84680_body54_e130262_d_n7;
            locals.var_fs0_dps0_dn8 = assign84680_body54_e130262_d_n8;
            locals.var_fs0_dps0_dn9 = assign84680_body54_e130262_d_n9;
            locals.var_fs0_dps0_dn10 = assign84680_body54_e130262_d_n10;
            locals.var_fs0_dps0_dn13 = assign84680_body54_e130262_d_n13;
            locals.var_fs0_dps0_rv = 0.0;
            let assign84680_body55_e130265: f64 = if locals.var_flg_conv > 0.0 { 1.0 } else { 0.0 };
            locals.var_guard1975 = assign84680_body55_e130265;
            locals.var_guard1975_rv = 0.0;
            let (assign84680_body56_e130275,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1975 != 0.0)) {
        let assign84680_body56_e130273: f64 = (locals.var_lp_s0_max + 1.0);
        (assign84680_body56_e130273,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign84680_body56_e130275;
            locals.var_lp_s0_rv = 0.0;
            let (assign84680_body57_e130287, assign84680_body57_e130287_d_n0, assign84680_body57_e130287_d_n2, assign84680_body57_e130287_d_n4, assign84680_body57_e130287_d_n5, assign84680_body57_e130287_d_n6, assign84680_body57_e130287_d_n7, assign84680_body57_e130287_d_n8, assign84680_body57_e130287_d_n9, assign84680_body57_e130287_d_n10, assign84680_body57_e130287_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1975 == 0.0)) {
        let assign84680_body57_e130283: f64 = (-locals.var_fs0);
        let assign84680_body57_e130285: f64 = (assign84680_body57_e130283 / locals.var_fs0_dps0);
        (assign84680_body57_e130285, ((((-locals.var_fs0_dn0) * locals.var_fs0_dps0) - (assign84680_body57_e130283 * locals.var_fs0_dps0_dn0)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn2) * locals.var_fs0_dps0) - (assign84680_body57_e130283 * locals.var_fs0_dps0_dn2)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn4) * locals.var_fs0_dps0) - (assign84680_body57_e130283 * locals.var_fs0_dps0_dn4)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn5) * locals.var_fs0_dps0) - (assign84680_body57_e130283 * locals.var_fs0_dps0_dn5)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn6) * locals.var_fs0_dps0) - (assign84680_body57_e130283 * locals.var_fs0_dps0_dn6)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn7) * locals.var_fs0_dps0) - (assign84680_body57_e130283 * locals.var_fs0_dps0_dn7)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn8) * locals.var_fs0_dps0) - (assign84680_body57_e130283 * locals.var_fs0_dps0_dn8)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn9) * locals.var_fs0_dps0) - (assign84680_body57_e130283 * locals.var_fs0_dps0_dn9)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn10) * locals.var_fs0_dps0) - (assign84680_body57_e130283 * locals.var_fs0_dps0_dn10)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn13) * locals.var_fs0_dps0) - (assign84680_body57_e130283 * locals.var_fs0_dps0_dn13)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn4, locals.var_dps0_dn5, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn8, locals.var_dps0_dn9, locals.var_dps0_dn10, locals.var_dps0_dn13,)
    }
};
            locals.var_dps0 = assign84680_body57_e130287;
            locals.var_dps0_dn0 = assign84680_body57_e130287_d_n0;
            locals.var_dps0_dn2 = assign84680_body57_e130287_d_n2;
            locals.var_dps0_dn4 = assign84680_body57_e130287_d_n4;
            locals.var_dps0_dn5 = assign84680_body57_e130287_d_n5;
            locals.var_dps0_dn6 = assign84680_body57_e130287_d_n6;
            locals.var_dps0_dn7 = assign84680_body57_e130287_d_n7;
            locals.var_dps0_dn8 = assign84680_body57_e130287_d_n8;
            locals.var_dps0_dn9 = assign84680_body57_e130287_d_n9;
            locals.var_dps0_dn10 = assign84680_body57_e130287_d_n10;
            locals.var_dps0_dn13 = assign84680_body57_e130287_d_n13;
            locals.var_dps0_rv = 0.0;
            let (assign84680_body58_e130309, assign84680_body58_e130309_d_n0, assign84680_body58_e130309_d_n2, assign84680_body58_e130309_d_n4, assign84680_body58_e130309_d_n5, assign84680_body58_e130309_d_n6, assign84680_body58_e130309_d_n7, assign84680_body58_e130309_d_n8, assign84680_body58_e130309_d_n9, assign84680_body58_e130309_d_n10, assign84680_body58_e130309_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1975 == 0.0)) {
        let assign84680_body58_e130296: f64 = (0.5 * 0.1);
        let assign84680_body58_e130300: f64 = (locals.var_ps0ld).abs();
        let (assign84680_body58_e130305, assign84680_body58_e130305_d_n0, assign84680_body58_e130305_d_n2, assign84680_body58_e130305_d_n4, assign84680_body58_e130305_d_n5, assign84680_body58_e130305_d_n6, assign84680_body58_e130305_d_n7, assign84680_body58_e130305_d_n8, assign84680_body58_e130305_d_n9, assign84680_body58_e130305_d_n10, assign84680_body58_e130305_d_n13,) = {
            if (1.0 >= assign84680_body58_e130300) {
                (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign84680_body58_e130304: f64 = (locals.var_ps0ld).abs();
                (assign84680_body58_e130304, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn0 } else { (-locals.var_ps0ld_dn0) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn2 } else { (-locals.var_ps0ld_dn2) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn4 } else { (-locals.var_ps0ld_dn4) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn5 } else { (-locals.var_ps0ld_dn5) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn6 } else { (-locals.var_ps0ld_dn6) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn7 } else { (-locals.var_ps0ld_dn7) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn8 } else { (-locals.var_ps0ld_dn8) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn9 } else { (-locals.var_ps0ld_dn9) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn10 } else { (-locals.var_ps0ld_dn10) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn13 } else { (-locals.var_ps0ld_dn13) },)
            }
        };
        let assign84680_body58_e130306: f64 = (1.0 + assign84680_body58_e130305);
        let assign84680_body58_e130307: f64 = (assign84680_body58_e130296 * assign84680_body58_e130306);
        (assign84680_body58_e130307, (assign84680_body58_e130296 * assign84680_body58_e130305_d_n0), (assign84680_body58_e130296 * assign84680_body58_e130305_d_n2), (assign84680_body58_e130296 * assign84680_body58_e130305_d_n4), (assign84680_body58_e130296 * assign84680_body58_e130305_d_n5), (assign84680_body58_e130296 * assign84680_body58_e130305_d_n6), (assign84680_body58_e130296 * assign84680_body58_e130305_d_n7), (assign84680_body58_e130296 * assign84680_body58_e130305_d_n8), (assign84680_body58_e130296 * assign84680_body58_e130305_d_n9), (assign84680_body58_e130296 * assign84680_body58_e130305_d_n10), (assign84680_body58_e130296 * assign84680_body58_e130305_d_n13),)
    } else {
        (locals.var_dplim, locals.var_dplim_dn0, locals.var_dplim_dn2, locals.var_dplim_dn4, locals.var_dplim_dn5, locals.var_dplim_dn6, locals.var_dplim_dn7, locals.var_dplim_dn8, locals.var_dplim_dn9, locals.var_dplim_dn10, locals.var_dplim_dn13,)
    }
};
            locals.var_dplim = assign84680_body58_e130309;
            locals.var_dplim_dn0 = assign84680_body58_e130309_d_n0;
            locals.var_dplim_dn2 = assign84680_body58_e130309_d_n2;
            locals.var_dplim_dn4 = assign84680_body58_e130309_d_n4;
            locals.var_dplim_dn5 = assign84680_body58_e130309_d_n5;
            locals.var_dplim_dn6 = assign84680_body58_e130309_d_n6;
            locals.var_dplim_dn7 = assign84680_body58_e130309_d_n7;
            locals.var_dplim_dn8 = assign84680_body58_e130309_d_n8;
            locals.var_dplim_dn9 = assign84680_body58_e130309_d_n9;
            locals.var_dplim_dn10 = assign84680_body58_e130309_d_n10;
            locals.var_dplim_dn13 = assign84680_body58_e130309_d_n13;
            locals.var_dplim_rv = 0.0;
            let assign84680_body59_e130311: f64 = (locals.var_dps0).abs();
            let assign84680_body59_e130313: f64 = if assign84680_body59_e130311 > locals.var_dplim { 1.0 } else { 0.0 };
            locals.var_guard1976 = assign84680_body59_e130313;
            locals.var_guard1976_rv = 0.0;
            let (assign84680_body60_e130332, assign84680_body60_e130332_d_n0, assign84680_body60_e130332_d_n2, assign84680_body60_e130332_d_n4, assign84680_body60_e130332_d_n5, assign84680_body60_e130332_d_n6, assign84680_body60_e130332_d_n7, assign84680_body60_e130332_d_n8, assign84680_body60_e130332_d_n9, assign84680_body60_e130332_d_n10, assign84680_body60_e130332_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1975 == 0.0)) && (locals.var_guard1976 != 0.0)) {
        let (assign84680_body60_e130329,) = {
            if (locals.var_dps0 >= 0.0) {
                (1.0,)
            } else {
                let assign84680_body60_e130328: f64 = (-1.0);
                (assign84680_body60_e130328,)
            }
        };
        let assign84680_body60_e130330: f64 = (locals.var_dplim * assign84680_body60_e130329);
        (assign84680_body60_e130330, (locals.var_dplim_dn0 * assign84680_body60_e130329), (locals.var_dplim_dn2 * assign84680_body60_e130329), (locals.var_dplim_dn4 * assign84680_body60_e130329), (locals.var_dplim_dn5 * assign84680_body60_e130329), (locals.var_dplim_dn6 * assign84680_body60_e130329), (locals.var_dplim_dn7 * assign84680_body60_e130329), (locals.var_dplim_dn8 * assign84680_body60_e130329), (locals.var_dplim_dn9 * assign84680_body60_e130329), (locals.var_dplim_dn10 * assign84680_body60_e130329), (locals.var_dplim_dn13 * assign84680_body60_e130329),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn4, locals.var_dps0_dn5, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn8, locals.var_dps0_dn9, locals.var_dps0_dn10, locals.var_dps0_dn13,)
    }
};
            locals.var_dps0 = assign84680_body60_e130332;
            locals.var_dps0_dn0 = assign84680_body60_e130332_d_n0;
            locals.var_dps0_dn2 = assign84680_body60_e130332_d_n2;
            locals.var_dps0_dn4 = assign84680_body60_e130332_d_n4;
            locals.var_dps0_dn5 = assign84680_body60_e130332_d_n5;
            locals.var_dps0_dn6 = assign84680_body60_e130332_d_n6;
            locals.var_dps0_dn7 = assign84680_body60_e130332_d_n7;
            locals.var_dps0_dn8 = assign84680_body60_e130332_d_n8;
            locals.var_dps0_dn9 = assign84680_body60_e130332_d_n9;
            locals.var_dps0_dn10 = assign84680_body60_e130332_d_n10;
            locals.var_dps0_dn13 = assign84680_body60_e130332_d_n13;
            locals.var_dps0_rv = 0.0;
            let (assign84680_body61_e130343, assign84680_body61_e130343_d_n0, assign84680_body61_e130343_d_n2, assign84680_body61_e130343_d_n4, assign84680_body61_e130343_d_n5, assign84680_body61_e130343_d_n6, assign84680_body61_e130343_d_n7, assign84680_body61_e130343_d_n8, assign84680_body61_e130343_d_n9, assign84680_body61_e130343_d_n10, assign84680_body61_e130343_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1975 == 0.0)) {
        let assign84680_body61_e130341: f64 = (locals.var_ps0ld + locals.var_dps0);
        (assign84680_body61_e130341, (locals.var_ps0ld_dn0 + locals.var_dps0_dn0), (locals.var_ps0ld_dn2 + locals.var_dps0_dn2), (locals.var_ps0ld_dn4 + locals.var_dps0_dn4), (locals.var_ps0ld_dn5 + locals.var_dps0_dn5), (locals.var_ps0ld_dn6 + locals.var_dps0_dn6), (locals.var_ps0ld_dn7 + locals.var_dps0_dn7), (locals.var_ps0ld_dn8 + locals.var_dps0_dn8), (locals.var_ps0ld_dn9 + locals.var_dps0_dn9), (locals.var_ps0ld_dn10 + locals.var_dps0_dn10), (locals.var_ps0ld_dn13 + locals.var_dps0_dn13),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    }
};
            locals.var_ps0ld = assign84680_body61_e130343;
            locals.var_ps0ld_dn0 = assign84680_body61_e130343_d_n0;
            locals.var_ps0ld_dn2 = assign84680_body61_e130343_d_n2;
            locals.var_ps0ld_dn4 = assign84680_body61_e130343_d_n4;
            locals.var_ps0ld_dn5 = assign84680_body61_e130343_d_n5;
            locals.var_ps0ld_dn6 = assign84680_body61_e130343_d_n6;
            locals.var_ps0ld_dn7 = assign84680_body61_e130343_d_n7;
            locals.var_ps0ld_dn8 = assign84680_body61_e130343_d_n8;
            locals.var_ps0ld_dn9 = assign84680_body61_e130343_d_n9;
            locals.var_ps0ld_dn10 = assign84680_body61_e130343_d_n10;
            locals.var_ps0ld_dn13 = assign84680_body61_e130343_d_n13;
            locals.var_ps0ld_rv = 0.0;
            let assign84680_body62_e130345: f64 = (locals.var_dps0).abs();
            let assign84680_body62_e130349: f64 = (locals.var_fs0).abs();
            let assign84680_body62_e130352: f64 = if ((assign84680_body62_e130345 <= 1e-12) && (assign84680_body62_e130349 <= 1e-8)) { 1.0 } else { 0.0 };
            locals.var_guard1977 = assign84680_body62_e130352;
            locals.var_guard1977_rv = 0.0;
            let (assign84680_body63_e130365,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1975 == 0.0)) && (locals.var_guard1977 != 0.0)) {
        let assign84680_body63_e130363: f64 = (locals.var_flg_conv + 2.0);
        (assign84680_body63_e130363,)
    } else {
        (locals.var_flg_conv,)
    }
};
            locals.var_flg_conv = assign84680_body63_e130365;
            locals.var_flg_conv_rv = 0.0;
            let (assign84680_body64_e130373,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) {
        let assign84680_body64_e130371: f64 = (locals.var_lp_s0 + 1.0);
        (assign84680_body64_e130371,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign84680_body64_e130373;
            locals.var_lp_s0_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_314(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign84700_e130396, assign84700_e130396_d_n0, assign84700_e130396_d_n2, assign84700_e130396_d_n4, assign84700_e130396_d_n5, assign84700_e130396_d_n6, assign84700_e130396_d_n7, assign84700_e130396_d_n8, assign84700_e130396_d_n9, assign84700_e130396_d_n10, assign84700_e130396_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) {
        let (assign84700_e130394, assign84700_e130394_d_n0, assign84700_e130394_d_n2, assign84700_e130394_d_n4, assign84700_e130394_d_n5, assign84700_e130394_d_n6, assign84700_e130394_d_n7, assign84700_e130394_d_n8, assign84700_e130394_d_n9, assign84700_e130394_d_n10, assign84700_e130394_d_n13,) = {
            if (locals.var_fbsq__blk1891 >= 0.0) {
                let (assign84700_e130389,) = {
                    if (locals.var_chi >= 0.0) {
                        (1.0,)
                    } else {
                        let assign84700_e130388: f64 = (-1.0);
                        (assign84700_e130388,)
                    }
                };
                let assign84700_e130391: f64 = (locals.var_fbsq__blk1891).sqrt();
                let assign84700_e130392: f64 = (assign84700_e130389 * assign84700_e130391);
                (assign84700_e130392, (assign84700_e130389 * (locals.var_fbsq__blk1891_dn0 / (2.0 * assign84700_e130391))), (assign84700_e130389 * (locals.var_fbsq__blk1891_dn2 / (2.0 * assign84700_e130391))), (assign84700_e130389 * (locals.var_fbsq__blk1891_dn4 / (2.0 * assign84700_e130391))), (assign84700_e130389 * (locals.var_fbsq__blk1891_dn5 / (2.0 * assign84700_e130391))), (assign84700_e130389 * (locals.var_fbsq__blk1891_dn6 / (2.0 * assign84700_e130391))), (assign84700_e130389 * (locals.var_fbsq__blk1891_dn7 / (2.0 * assign84700_e130391))), (assign84700_e130389 * (locals.var_fbsq__blk1891_dn8 / (2.0 * assign84700_e130391))), (assign84700_e130389 * (locals.var_fbsq__blk1891_dn9 / (2.0 * assign84700_e130391))), (assign84700_e130389 * (locals.var_fbsq__blk1891_dn10 / (2.0 * assign84700_e130391))), (assign84700_e130389 * (locals.var_fbsq__blk1891_dn13 / (2.0 * assign84700_e130391))),)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign84700_e130394, assign84700_e130394_d_n0, assign84700_e130394_d_n2, assign84700_e130394_d_n4, assign84700_e130394_d_n5, assign84700_e130394_d_n6, assign84700_e130394_d_n7, assign84700_e130394_d_n8, assign84700_e130394_d_n9, assign84700_e130394_d_n10, assign84700_e130394_d_n13,)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn13,)
    }
};
        locals.var_fb = assign84700_e130396;
        locals.var_fb_dn0 = assign84700_e130396_d_n0;
        locals.var_fb_dn2 = assign84700_e130396_d_n2;
        locals.var_fb_dn4 = assign84700_e130396_d_n4;
        locals.var_fb_dn5 = assign84700_e130396_d_n5;
        locals.var_fb_dn6 = assign84700_e130396_d_n6;
        locals.var_fb_dn7 = assign84700_e130396_d_n7;
        locals.var_fb_dn8 = assign84700_e130396_d_n8;
        locals.var_fb_dn9 = assign84700_e130396_d_n9;
        locals.var_fb_dn10 = assign84700_e130396_d_n10;
        locals.var_fb_dn13 = assign84700_e130396_d_n13;
        locals.var_fb_rv = 0.0;

        let (assign84710_e130404, assign84710_e130404_d_n0, assign84710_e130404_d_n2, assign84710_e130404_d_n4, assign84710_e130404_d_n5, assign84710_e130404_d_n6, assign84710_e130404_d_n7, assign84710_e130404_d_n8, assign84710_e130404_d_n9, assign84710_e130404_d_n10, assign84710_e130404_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) {
        let assign84710_e130402: f64 = (locals.var_c_w_ld * locals.var_fb);
        (assign84710_e130402, ((locals.var_c_w_ld_dn0 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn0)), ((locals.var_c_w_ld_dn2 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn2)), ((locals.var_c_w_ld_dn4 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn4)), ((locals.var_c_w_ld_dn5 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn5)), ((locals.var_c_w_ld_dn6 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn6)), ((locals.var_c_w_ld_dn7 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn7)), ((locals.var_c_w_ld_dn8 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn8)), ((locals.var_c_w_ld_dn9 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn9)), ((locals.var_c_w_ld_dn10 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn10)), ((locals.var_c_w_ld_dn13 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn13)),)
    } else {
        (locals.var_wdld__blk1881, locals.var_wdld__blk1881_dn0, locals.var_wdld__blk1881_dn2, locals.var_wdld__blk1881_dn4, locals.var_wdld__blk1881_dn5, locals.var_wdld__blk1881_dn6, locals.var_wdld__blk1881_dn7, locals.var_wdld__blk1881_dn8, locals.var_wdld__blk1881_dn9, locals.var_wdld__blk1881_dn10, locals.var_wdld__blk1881_dn13,)
    }
};
        locals.var_wdld__blk1881 = assign84710_e130404;
        locals.var_wdld__blk1881_dn0 = assign84710_e130404_d_n0;
        locals.var_wdld__blk1881_dn2 = assign84710_e130404_d_n2;
        locals.var_wdld__blk1881_dn4 = assign84710_e130404_d_n4;
        locals.var_wdld__blk1881_dn5 = assign84710_e130404_d_n5;
        locals.var_wdld__blk1881_dn6 = assign84710_e130404_d_n6;
        locals.var_wdld__blk1881_dn7 = assign84710_e130404_d_n7;
        locals.var_wdld__blk1881_dn8 = assign84710_e130404_d_n8;
        locals.var_wdld__blk1881_dn9 = assign84710_e130404_d_n9;
        locals.var_wdld__blk1881_dn10 = assign84710_e130404_d_n10;
        locals.var_wdld__blk1881_dn13 = assign84710_e130404_d_n13;
        locals.var_wdld__blk1881_rv = 0.0;

        let (assign84720_e130412, assign84720_e130412_d_n0, assign84720_e130412_d_n2, assign84720_e130412_d_n4, assign84720_e130412_d_n5, assign84720_e130412_d_n6, assign84720_e130412_d_n7, assign84720_e130412_d_n8, assign84720_e130412_d_n9, assign84720_e130412_d_n10, assign84720_e130412_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) {
        let assign84720_e130410: f64 = (locals.var_q_nsubld__blk1883 * locals.var_wdld__blk1881);
        (assign84720_e130410, (locals.var_q_nsubld__blk1883 * locals.var_wdld__blk1881_dn0), (locals.var_q_nsubld__blk1883 * locals.var_wdld__blk1881_dn2), (locals.var_q_nsubld__blk1883 * locals.var_wdld__blk1881_dn4), (locals.var_q_nsubld__blk1883 * locals.var_wdld__blk1881_dn5), (locals.var_q_nsubld__blk1883 * locals.var_wdld__blk1881_dn6), (locals.var_q_nsubld__blk1883 * locals.var_wdld__blk1881_dn7), (locals.var_q_nsubld__blk1883 * locals.var_wdld__blk1881_dn8), (locals.var_q_nsubld__blk1883 * locals.var_wdld__blk1881_dn9), (locals.var_q_nsubld__blk1883 * locals.var_wdld__blk1881_dn10), (locals.var_q_nsubld__blk1883 * locals.var_wdld__blk1881_dn13),)
    } else {
        (locals.var_q_dep_ld__blk1882, locals.var_q_dep_ld__blk1882_dn0, locals.var_q_dep_ld__blk1882_dn2, locals.var_q_dep_ld__blk1882_dn4, locals.var_q_dep_ld__blk1882_dn5, locals.var_q_dep_ld__blk1882_dn6, locals.var_q_dep_ld__blk1882_dn7, locals.var_q_dep_ld__blk1882_dn8, locals.var_q_dep_ld__blk1882_dn9, locals.var_q_dep_ld__blk1882_dn10, locals.var_q_dep_ld__blk1882_dn13,)
    }
};
        locals.var_q_dep_ld__blk1882 = assign84720_e130412;
        locals.var_q_dep_ld__blk1882_dn0 = assign84720_e130412_d_n0;
        locals.var_q_dep_ld__blk1882_dn2 = assign84720_e130412_d_n2;
        locals.var_q_dep_ld__blk1882_dn4 = assign84720_e130412_d_n4;
        locals.var_q_dep_ld__blk1882_dn5 = assign84720_e130412_d_n5;
        locals.var_q_dep_ld__blk1882_dn6 = assign84720_e130412_d_n6;
        locals.var_q_dep_ld__blk1882_dn7 = assign84720_e130412_d_n7;
        locals.var_q_dep_ld__blk1882_dn8 = assign84720_e130412_d_n8;
        locals.var_q_dep_ld__blk1882_dn9 = assign84720_e130412_d_n9;
        locals.var_q_dep_ld__blk1882_dn10 = assign84720_e130412_d_n10;
        locals.var_q_dep_ld__blk1882_dn13 = assign84720_e130412_d_n13;
        locals.var_q_dep_ld__blk1882_rv = 0.0;

        let (assign84730_e130424, assign84730_e130424_d_n0, assign84730_e130424_d_n2, assign84730_e130424_d_n4, assign84730_e130424_d_n5, assign84730_e130424_d_n6, assign84730_e130424_d_n7, assign84730_e130424_d_n8, assign84730_e130424_d_n9, assign84730_e130424_d_n10, assign84730_e130424_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) {
        let assign84730_e130418: f64 = (locals.var_q_dep_ld__blk1882 / locals.var_cnst0over_func);
        let assign84730_e130421: f64 = (10.0 * 2.220446049250313e-16);
        let assign84730_e130422: f64 = (assign84730_e130418 + assign84730_e130421);
        (assign84730_e130422, (((locals.var_q_dep_ld__blk1882_dn0 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1882 * locals.var_cnst0over_func_dn0)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1882_dn2 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1882 * locals.var_cnst0over_func_dn2)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1882_dn4 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1882 * locals.var_cnst0over_func_dn4)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1882_dn5 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1882 * locals.var_cnst0over_func_dn5)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1882_dn6 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1882 * locals.var_cnst0over_func_dn6)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1882_dn7 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1882 * locals.var_cnst0over_func_dn7)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1882_dn8 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1882 * locals.var_cnst0over_func_dn8)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1882_dn9 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1882 * locals.var_cnst0over_func_dn9)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1882_dn10 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1882 * locals.var_cnst0over_func_dn10)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1882_dn13 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1882 * locals.var_cnst0over_func_dn13)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)),)
    } else {
        (locals.var_xi0p12, locals.var_xi0p12_dn0, locals.var_xi0p12_dn2, locals.var_xi0p12_dn4, locals.var_xi0p12_dn5, locals.var_xi0p12_dn6, locals.var_xi0p12_dn7, locals.var_xi0p12_dn8, locals.var_xi0p12_dn9, locals.var_xi0p12_dn10, locals.var_xi0p12_dn13,)
    }
};
        locals.var_xi0p12 = assign84730_e130424;
        locals.var_xi0p12_dn0 = assign84730_e130424_d_n0;
        locals.var_xi0p12_dn2 = assign84730_e130424_d_n2;
        locals.var_xi0p12_dn4 = assign84730_e130424_d_n4;
        locals.var_xi0p12_dn5 = assign84730_e130424_d_n5;
        locals.var_xi0p12_dn6 = assign84730_e130424_d_n6;
        locals.var_xi0p12_dn7 = assign84730_e130424_d_n7;
        locals.var_xi0p12_dn8 = assign84730_e130424_d_n8;
        locals.var_xi0p12_dn9 = assign84730_e130424_d_n9;
        locals.var_xi0p12_dn10 = assign84730_e130424_d_n10;
        locals.var_xi0p12_dn13 = assign84730_e130424_d_n13;
        locals.var_xi0p12_rv = 0.0;

        let (assign84740_e130432, assign84740_e130432_d_n0, assign84740_e130432_d_n2, assign84740_e130432_d_n4, assign84740_e130432_d_n5, assign84740_e130432_d_n6, assign84740_e130432_d_n7, assign84740_e130432_d_n8, assign84740_e130432_d_n9, assign84740_e130432_d_n10, assign84740_e130432_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) {
        let assign84740_e130430: f64 = (locals.var_cnst0over_func * locals.var_xi0p12);
        (assign84740_e130430, ((locals.var_cnst0over_func_dn0 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn0)), ((locals.var_cnst0over_func_dn2 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn2)), ((locals.var_cnst0over_func_dn4 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn4)), ((locals.var_cnst0over_func_dn5 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn5)), ((locals.var_cnst0over_func_dn6 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn6)), ((locals.var_cnst0over_func_dn7 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn7)), ((locals.var_cnst0over_func_dn8 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn8)), ((locals.var_cnst0over_func_dn9 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn9)), ((locals.var_cnst0over_func_dn10 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn10)), ((locals.var_cnst0over_func_dn13 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn13)),)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn8, locals.var_qbuld_dn9, locals.var_qbuld_dn10, locals.var_qbuld_dn13,)
    }
};
        locals.var_qbuld = assign84740_e130432;
        locals.var_qbuld_dn0 = assign84740_e130432_d_n0;
        locals.var_qbuld_dn2 = assign84740_e130432_d_n2;
        locals.var_qbuld_dn4 = assign84740_e130432_d_n4;
        locals.var_qbuld_dn5 = assign84740_e130432_d_n5;
        locals.var_qbuld_dn6 = assign84740_e130432_d_n6;
        locals.var_qbuld_dn7 = assign84740_e130432_d_n7;
        locals.var_qbuld_dn8 = assign84740_e130432_d_n8;
        locals.var_qbuld_dn9 = assign84740_e130432_d_n9;
        locals.var_qbuld_dn10 = assign84740_e130432_d_n10;
        locals.var_qbuld_dn13 = assign84740_e130432_d_n13;
        locals.var_qbuld_rv = 0.0;

        let (assign84750_e130442, assign84750_e130442_d_n0, assign84750_e130442_d_n2, assign84750_e130442_d_n4, assign84750_e130442_d_n5, assign84750_e130442_d_n6, assign84750_e130442_d_n7, assign84750_e130442_d_n8, assign84750_e130442_d_n9, assign84750_e130442_d_n10, assign84750_e130442_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) {
        let assign84750_e130439: f64 = (locals.var_fs02 + locals.var_xi0p12);
        let assign84750_e130440: f64 = (1.0 / assign84750_e130439);
        (assign84750_e130440, (-((locals.var_fs02_dn0 + locals.var_xi0p12_dn0) / (assign84750_e130439 * assign84750_e130439))), (-((locals.var_fs02_dn2 + locals.var_xi0p12_dn2) / (assign84750_e130439 * assign84750_e130439))), (-((locals.var_fs02_dn4 + locals.var_xi0p12_dn4) / (assign84750_e130439 * assign84750_e130439))), (-((locals.var_fs02_dn5 + locals.var_xi0p12_dn5) / (assign84750_e130439 * assign84750_e130439))), (-((locals.var_fs02_dn6 + locals.var_xi0p12_dn6) / (assign84750_e130439 * assign84750_e130439))), (-((locals.var_fs02_dn7 + locals.var_xi0p12_dn7) / (assign84750_e130439 * assign84750_e130439))), (-((locals.var_fs02_dn8 + locals.var_xi0p12_dn8) / (assign84750_e130439 * assign84750_e130439))), (-((locals.var_fs02_dn9 + locals.var_xi0p12_dn9) / (assign84750_e130439 * assign84750_e130439))), (-((locals.var_fs02_dn10 + locals.var_xi0p12_dn10) / (assign84750_e130439 * assign84750_e130439))), (-((locals.var_fs02_dn13 + locals.var_xi0p12_dn13) / (assign84750_e130439 * assign84750_e130439))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign84750_e130442;
        locals.var_t1_dn0 = assign84750_e130442_d_n0;
        locals.var_t1_dn2 = assign84750_e130442_d_n2;
        locals.var_t1_dn4 = assign84750_e130442_d_n4;
        locals.var_t1_dn5 = assign84750_e130442_d_n5;
        locals.var_t1_dn6 = assign84750_e130442_d_n6;
        locals.var_t1_dn7 = assign84750_e130442_d_n7;
        locals.var_t1_dn8 = assign84750_e130442_d_n8;
        locals.var_t1_dn9 = assign84750_e130442_d_n9;
        locals.var_t1_dn10 = assign84750_e130442_d_n10;
        locals.var_t1_dn13 = assign84750_e130442_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign84760_e130452, assign84760_e130452_d_n0, assign84760_e130452_d_n2, assign84760_e130452_d_n4, assign84760_e130452_d_n5, assign84760_e130452_d_n6, assign84760_e130452_d_n7, assign84760_e130452_d_n8, assign84760_e130452_d_n9, assign84760_e130452_d_n10, assign84760_e130452_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) {
        let assign84760_e130448: f64 = (locals.var_cnst0over_func * locals.var_fs01);
        let assign84760_e130450: f64 = (assign84760_e130448 * locals.var_t1);
        (assign84760_e130450, ((((locals.var_cnst0over_func_dn0 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn0)) * locals.var_t1) + (assign84760_e130448 * locals.var_t1_dn0)), ((((locals.var_cnst0over_func_dn2 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn2)) * locals.var_t1) + (assign84760_e130448 * locals.var_t1_dn2)), ((((locals.var_cnst0over_func_dn4 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn4)) * locals.var_t1) + (assign84760_e130448 * locals.var_t1_dn4)), ((((locals.var_cnst0over_func_dn5 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn5)) * locals.var_t1) + (assign84760_e130448 * locals.var_t1_dn5)), ((((locals.var_cnst0over_func_dn6 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn6)) * locals.var_t1) + (assign84760_e130448 * locals.var_t1_dn6)), ((((locals.var_cnst0over_func_dn7 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn7)) * locals.var_t1) + (assign84760_e130448 * locals.var_t1_dn7)), ((((locals.var_cnst0over_func_dn8 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn8)) * locals.var_t1) + (assign84760_e130448 * locals.var_t1_dn8)), ((((locals.var_cnst0over_func_dn9 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn9)) * locals.var_t1) + (assign84760_e130448 * locals.var_t1_dn9)), ((((locals.var_cnst0over_func_dn10 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn10)) * locals.var_t1) + (assign84760_e130448 * locals.var_t1_dn10)), ((((locals.var_cnst0over_func_dn13 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn13)) * locals.var_t1) + (assign84760_e130448 * locals.var_t1_dn13)),)
    } else {
        (locals.var_qiuld, locals.var_qiuld_dn0, locals.var_qiuld_dn2, locals.var_qiuld_dn4, locals.var_qiuld_dn5, locals.var_qiuld_dn6, locals.var_qiuld_dn7, locals.var_qiuld_dn8, locals.var_qiuld_dn9, locals.var_qiuld_dn10, locals.var_qiuld_dn13,)
    }
};
        locals.var_qiuld = assign84760_e130452;
        locals.var_qiuld_dn0 = assign84760_e130452_d_n0;
        locals.var_qiuld_dn2 = assign84760_e130452_d_n2;
        locals.var_qiuld_dn4 = assign84760_e130452_d_n4;
        locals.var_qiuld_dn5 = assign84760_e130452_d_n5;
        locals.var_qiuld_dn6 = assign84760_e130452_d_n6;
        locals.var_qiuld_dn7 = assign84760_e130452_d_n7;
        locals.var_qiuld_dn8 = assign84760_e130452_d_n8;
        locals.var_qiuld_dn9 = assign84760_e130452_d_n9;
        locals.var_qiuld_dn10 = assign84760_e130452_d_n10;
        locals.var_qiuld_dn13 = assign84760_e130452_d_n13;
        locals.var_qiuld_rv = 0.0;

        let (assign84770_e130460, assign84770_e130460_d_n0, assign84770_e130460_d_n2, assign84770_e130460_d_n4, assign84770_e130460_d_n5, assign84770_e130460_d_n6, assign84770_e130460_d_n7, assign84770_e130460_d_n8, assign84770_e130460_d_n9, assign84770_e130460_d_n10, assign84770_e130460_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) {
        let assign84770_e130458: f64 = (locals.var_qbuld + locals.var_qiuld);
        (assign84770_e130458, (locals.var_qbuld_dn0 + locals.var_qiuld_dn0), (locals.var_qbuld_dn2 + locals.var_qiuld_dn2), (locals.var_qbuld_dn4 + locals.var_qiuld_dn4), (locals.var_qbuld_dn5 + locals.var_qiuld_dn5), (locals.var_qbuld_dn6 + locals.var_qiuld_dn6), (locals.var_qbuld_dn7 + locals.var_qiuld_dn7), (locals.var_qbuld_dn8 + locals.var_qiuld_dn8), (locals.var_qbuld_dn9 + locals.var_qiuld_dn9), (locals.var_qbuld_dn10 + locals.var_qiuld_dn10), (locals.var_qbuld_dn13 + locals.var_qiuld_dn13),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn4, locals.var_qsuld_dn5, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn8, locals.var_qsuld_dn9, locals.var_qsuld_dn10, locals.var_qsuld_dn13,)
    }
};
        locals.var_qsuld = assign84770_e130460;
        locals.var_qsuld_dn0 = assign84770_e130460_d_n0;
        locals.var_qsuld_dn2 = assign84770_e130460_d_n2;
        locals.var_qsuld_dn4 = assign84770_e130460_d_n4;
        locals.var_qsuld_dn5 = assign84770_e130460_d_n5;
        locals.var_qsuld_dn6 = assign84770_e130460_d_n6;
        locals.var_qsuld_dn7 = assign84770_e130460_d_n7;
        locals.var_qsuld_dn8 = assign84770_e130460_d_n8;
        locals.var_qsuld_dn9 = assign84770_e130460_d_n9;
        locals.var_qsuld_dn10 = assign84770_e130460_d_n10;
        locals.var_qsuld_dn13 = assign84770_e130460_d_n13;
        locals.var_qsuld_rv = 0.0;

        let (assign84780_e130466, assign84780_e130466_d_n0, assign84780_e130466_d_n2, assign84780_e130466_d_n4, assign84780_e130466_d_n5, assign84780_e130466_d_n6, assign84780_e130466_d_n7, assign84780_e130466_d_n8, assign84780_e130466_d_n9, assign84780_e130466_d_n10, assign84780_e130466_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign84780_e130464: f64 = (locals.var_qsuld - locals.var_qbuld);
        (assign84780_e130464, (locals.var_qsuld_dn0 - locals.var_qbuld_dn0), (locals.var_qsuld_dn2 - locals.var_qbuld_dn2), (locals.var_qsuld_dn4 - locals.var_qbuld_dn4), (locals.var_qsuld_dn5 - locals.var_qbuld_dn5), (locals.var_qsuld_dn6 - locals.var_qbuld_dn6), (locals.var_qsuld_dn7 - locals.var_qbuld_dn7), (locals.var_qsuld_dn8 - locals.var_qbuld_dn8), (locals.var_qsuld_dn9 - locals.var_qbuld_dn9), (locals.var_qsuld_dn10 - locals.var_qbuld_dn10), (locals.var_qsuld_dn13 - locals.var_qbuld_dn13),)
    } else {
        (locals.var_qiuld, locals.var_qiuld_dn0, locals.var_qiuld_dn2, locals.var_qiuld_dn4, locals.var_qiuld_dn5, locals.var_qiuld_dn6, locals.var_qiuld_dn7, locals.var_qiuld_dn8, locals.var_qiuld_dn9, locals.var_qiuld_dn10, locals.var_qiuld_dn13,)
    }
};
        locals.var_qiuld = assign84780_e130466;
        locals.var_qiuld_dn0 = assign84780_e130466_d_n0;
        locals.var_qiuld_dn2 = assign84780_e130466_d_n2;
        locals.var_qiuld_dn4 = assign84780_e130466_d_n4;
        locals.var_qiuld_dn5 = assign84780_e130466_d_n5;
        locals.var_qiuld_dn6 = assign84780_e130466_d_n6;
        locals.var_qiuld_dn7 = assign84780_e130466_d_n7;
        locals.var_qiuld_dn8 = assign84780_e130466_d_n8;
        locals.var_qiuld_dn9 = assign84780_e130466_d_n9;
        locals.var_qiuld_dn10 = assign84780_e130466_d_n10;
        locals.var_qiuld_dn13 = assign84780_e130466_d_n13;
        locals.var_qiuld_rv = 0.0;

        let assign84790_e130469: f64 = if locals.var_lover_func < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1979 = assign84790_e130469;
        locals.var_guard1979_rv = 0.0;

        let (assign84800_e130476, assign84800_e130476_d_n0, assign84800_e130476_d_n2, assign84800_e130476_d_n4, assign84800_e130476_d_n5, assign84800_e130476_d_n6, assign84800_e130476_d_n7, assign84800_e130476_d_n8, assign84800_e130476_d_n9, assign84800_e130476_d_n10, assign84800_e130476_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1979 != 0.0)) {
        let assign84800_e130474: f64 = (-locals.var_lover_func);
        (assign84800_e130474, (-locals.var_lover_func_dn0), (-locals.var_lover_func_dn2), (-locals.var_lover_func_dn4), (-locals.var_lover_func_dn5), (-locals.var_lover_func_dn6), (-locals.var_lover_func_dn7), (-locals.var_lover_func_dn8), (-locals.var_lover_func_dn9), (-locals.var_lover_func_dn10), (-locals.var_lover_func_dn13),)
    } else {
        (locals.var_lover_func, locals.var_lover_func_dn0, locals.var_lover_func_dn2, locals.var_lover_func_dn4, locals.var_lover_func_dn5, locals.var_lover_func_dn6, locals.var_lover_func_dn7, locals.var_lover_func_dn8, locals.var_lover_func_dn9, locals.var_lover_func_dn10, locals.var_lover_func_dn13,)
    }
};
        locals.var_lover_func = assign84800_e130476;
        locals.var_lover_func_dn0 = assign84800_e130476_d_n0;
        locals.var_lover_func_dn2 = assign84800_e130476_d_n2;
        locals.var_lover_func_dn4 = assign84800_e130476_d_n4;
        locals.var_lover_func_dn5 = assign84800_e130476_d_n5;
        locals.var_lover_func_dn6 = assign84800_e130476_d_n6;
        locals.var_lover_func_dn7 = assign84800_e130476_d_n7;
        locals.var_lover_func_dn8 = assign84800_e130476_d_n8;
        locals.var_lover_func_dn9 = assign84800_e130476_d_n9;
        locals.var_lover_func_dn10 = assign84800_e130476_d_n10;
        locals.var_lover_func_dn13 = assign84800_e130476_d_n13;
        locals.var_lover_func_rv = 0.0;

        let assign84810_e130479: f64 = if p.p55 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1980 = assign84810_e130479;
        locals.var_guard1980_rv = 0.0;

        let assign84820_e130482: f64 = if p.p50 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1981 = assign84820_e130482;
        locals.var_guard1981_rv = 0.0;

        let (assign84830_e130493, assign84830_e130493_d_n0, assign84830_e130493_d_n2, assign84830_e130493_d_n4, assign84830_e130493_d_n5, assign84830_e130493_d_n6, assign84830_e130493_d_n7, assign84830_e130493_d_n8, assign84830_e130493_d_n9, assign84830_e130493_d_n10, assign84830_e130493_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1979 != 0.0)) && (locals.var_guard1980 != 0.0)) && (locals.var_guard1981 != 0.0)) {
        let assign84830_e130491: f64 = (-locals.var_ps0ld);
        (assign84830_e130491, (-locals.var_ps0ld_dn0), (-locals.var_ps0ld_dn2), (-locals.var_ps0ld_dn4), (-locals.var_ps0ld_dn5), (-locals.var_ps0ld_dn6), (-locals.var_ps0ld_dn7), (-locals.var_ps0ld_dn8), (-locals.var_ps0ld_dn9), (-locals.var_ps0ld_dn10), (-locals.var_ps0ld_dn13),)
    } else {
        (locals.var_vx__blk1884, locals.var_vx__blk1884_dn0, locals.var_vx__blk1884_dn2, locals.var_vx__blk1884_dn4, locals.var_vx__blk1884_dn5, locals.var_vx__blk1884_dn6, locals.var_vx__blk1884_dn7, locals.var_vx__blk1884_dn8, locals.var_vx__blk1884_dn9, locals.var_vx__blk1884_dn10, locals.var_vx__blk1884_dn13,)
    }
};
        locals.var_vx__blk1884 = assign84830_e130493;
        locals.var_vx__blk1884_dn0 = assign84830_e130493_d_n0;
        locals.var_vx__blk1884_dn2 = assign84830_e130493_d_n2;
        locals.var_vx__blk1884_dn4 = assign84830_e130493_d_n4;
        locals.var_vx__blk1884_dn5 = assign84830_e130493_d_n5;
        locals.var_vx__blk1884_dn6 = assign84830_e130493_d_n6;
        locals.var_vx__blk1884_dn7 = assign84830_e130493_d_n7;
        locals.var_vx__blk1884_dn8 = assign84830_e130493_d_n8;
        locals.var_vx__blk1884_dn9 = assign84830_e130493_d_n9;
        locals.var_vx__blk1884_dn10 = assign84830_e130493_d_n10;
        locals.var_vx__blk1884_dn13 = assign84830_e130493_d_n13;
        locals.var_vx__blk1884_rv = 0.0;

        let (assign84840_e130504, assign84840_e130504_d_n0, assign84840_e130504_d_n2, assign84840_e130504_d_n4, assign84840_e130504_d_n5, assign84840_e130504_d_n6, assign84840_e130504_d_n7, assign84840_e130504_d_n8, assign84840_e130504_d_n9, assign84840_e130504_d_n10, assign84840_e130504_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1979 != 0.0)) && (locals.var_guard1980 != 0.0)) && (locals.var_guard1981 == 0.0)) {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn13,)
    } else {
        (locals.var_vx__blk1884, locals.var_vx__blk1884_dn0, locals.var_vx__blk1884_dn2, locals.var_vx__blk1884_dn4, locals.var_vx__blk1884_dn5, locals.var_vx__blk1884_dn6, locals.var_vx__blk1884_dn7, locals.var_vx__blk1884_dn8, locals.var_vx__blk1884_dn9, locals.var_vx__blk1884_dn10, locals.var_vx__blk1884_dn13,)
    }
};
        locals.var_vx__blk1884 = assign84840_e130504;
        locals.var_vx__blk1884_dn0 = assign84840_e130504_d_n0;
        locals.var_vx__blk1884_dn2 = assign84840_e130504_d_n2;
        locals.var_vx__blk1884_dn4 = assign84840_e130504_d_n4;
        locals.var_vx__blk1884_dn5 = assign84840_e130504_d_n5;
        locals.var_vx__blk1884_dn6 = assign84840_e130504_d_n6;
        locals.var_vx__blk1884_dn7 = assign84840_e130504_d_n7;
        locals.var_vx__blk1884_dn8 = assign84840_e130504_d_n8;
        locals.var_vx__blk1884_dn9 = assign84840_e130504_d_n9;
        locals.var_vx__blk1884_dn10 = assign84840_e130504_d_n10;
        locals.var_vx__blk1884_dn13 = assign84840_e130504_d_n13;
        locals.var_vx__blk1884_rv = 0.0;

        let (assign84850_e130525, assign84850_e130525_d_n0, assign84850_e130525_d_n2, assign84850_e130525_d_n4, assign84850_e130525_d_n5, assign84850_e130525_d_n6, assign84850_e130525_d_n7, assign84850_e130525_d_n8, assign84850_e130525_d_n9, assign84850_e130525_d_n10, assign84850_e130525_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1979 != 0.0)) && (locals.var_guard1980 != 0.0)) {
        let assign84850_e130512: f64 = (locals.var_vx__blk1884 + p.p137);
        let assign84850_e130515: f64 = (locals.var_vx__blk1884 + p.p137);
        let assign84850_e130516: f64 = (assign84850_e130512 * assign84850_e130515);
        let assign84850_e130519: f64 = (4.0 * 0.1);
        let assign84850_e130521: f64 = (assign84850_e130519 * 0.1);
        let assign84850_e130522: f64 = (assign84850_e130516 + assign84850_e130521);
        let assign84850_e130523: f64 = (assign84850_e130522).sqrt();
        (assign84850_e130523, (((locals.var_vx__blk1884_dn0 * assign84850_e130515) + (assign84850_e130512 * locals.var_vx__blk1884_dn0)) / (2.0 * assign84850_e130523)), (((locals.var_vx__blk1884_dn2 * assign84850_e130515) + (assign84850_e130512 * locals.var_vx__blk1884_dn2)) / (2.0 * assign84850_e130523)), (((locals.var_vx__blk1884_dn4 * assign84850_e130515) + (assign84850_e130512 * locals.var_vx__blk1884_dn4)) / (2.0 * assign84850_e130523)), (((locals.var_vx__blk1884_dn5 * assign84850_e130515) + (assign84850_e130512 * locals.var_vx__blk1884_dn5)) / (2.0 * assign84850_e130523)), (((locals.var_vx__blk1884_dn6 * assign84850_e130515) + (assign84850_e130512 * locals.var_vx__blk1884_dn6)) / (2.0 * assign84850_e130523)), (((locals.var_vx__blk1884_dn7 * assign84850_e130515) + (assign84850_e130512 * locals.var_vx__blk1884_dn7)) / (2.0 * assign84850_e130523)), (((locals.var_vx__blk1884_dn8 * assign84850_e130515) + (assign84850_e130512 * locals.var_vx__blk1884_dn8)) / (2.0 * assign84850_e130523)), (((locals.var_vx__blk1884_dn9 * assign84850_e130515) + (assign84850_e130512 * locals.var_vx__blk1884_dn9)) / (2.0 * assign84850_e130523)), (((locals.var_vx__blk1884_dn10 * assign84850_e130515) + (assign84850_e130512 * locals.var_vx__blk1884_dn10)) / (2.0 * assign84850_e130523)), (((locals.var_vx__blk1884_dn13 * assign84850_e130515) + (assign84850_e130512 * locals.var_vx__blk1884_dn13)) / (2.0 * assign84850_e130523)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign84850_e130525;
        locals.var_tmf2_dn0 = assign84850_e130525_d_n0;
        locals.var_tmf2_dn2 = assign84850_e130525_d_n2;
        locals.var_tmf2_dn4 = assign84850_e130525_d_n4;
        locals.var_tmf2_dn5 = assign84850_e130525_d_n5;
        locals.var_tmf2_dn6 = assign84850_e130525_d_n6;
        locals.var_tmf2_dn7 = assign84850_e130525_d_n7;
        locals.var_tmf2_dn8 = assign84850_e130525_d_n8;
        locals.var_tmf2_dn9 = assign84850_e130525_d_n9;
        locals.var_tmf2_dn10 = assign84850_e130525_d_n10;
        locals.var_tmf2_dn13 = assign84850_e130525_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign84860_e130541, assign84860_e130541_d_n0, assign84860_e130541_d_n2, assign84860_e130541_d_n4, assign84860_e130541_d_n5, assign84860_e130541_d_n6, assign84860_e130541_d_n7, assign84860_e130541_d_n8, assign84860_e130541_d_n9, assign84860_e130541_d_n10, assign84860_e130541_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1979 != 0.0)) && (locals.var_guard1980 != 0.0)) {
        let assign84860_e130535: f64 = (locals.var_vx__blk1884 + p.p137);
        let assign84860_e130537: f64 = (assign84860_e130535 / locals.var_tmf2);
        let assign84860_e130538: f64 = (1.0 + assign84860_e130537);
        let assign84860_e130539: f64 = (0.5 * assign84860_e130538);
        (assign84860_e130539, (0.5 * (((locals.var_vx__blk1884_dn0 * locals.var_tmf2) - (assign84860_e130535 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk1884_dn2 * locals.var_tmf2) - (assign84860_e130535 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk1884_dn4 * locals.var_tmf2) - (assign84860_e130535 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk1884_dn5 * locals.var_tmf2) - (assign84860_e130535 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk1884_dn6 * locals.var_tmf2) - (assign84860_e130535 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk1884_dn7 * locals.var_tmf2) - (assign84860_e130535 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk1884_dn8 * locals.var_tmf2) - (assign84860_e130535 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk1884_dn9 * locals.var_tmf2) - (assign84860_e130535 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk1884_dn10 * locals.var_tmf2) - (assign84860_e130535 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk1884_dn13 * locals.var_tmf2) - (assign84860_e130535 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign84860_e130541;
        locals.var_t9_dn0 = assign84860_e130541_d_n0;
        locals.var_t9_dn2 = assign84860_e130541_d_n2;
        locals.var_t9_dn4 = assign84860_e130541_d_n4;
        locals.var_t9_dn5 = assign84860_e130541_d_n5;
        locals.var_t9_dn6 = assign84860_e130541_d_n6;
        locals.var_t9_dn7 = assign84860_e130541_d_n7;
        locals.var_t9_dn8 = assign84860_e130541_d_n8;
        locals.var_t9_dn9 = assign84860_e130541_d_n9;
        locals.var_t9_dn10 = assign84860_e130541_d_n10;
        locals.var_t9_dn13 = assign84860_e130541_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign84870_e130555, assign84870_e130555_d_n0, assign84870_e130555_d_n2, assign84870_e130555_d_n4, assign84870_e130555_d_n5, assign84870_e130555_d_n6, assign84870_e130555_d_n7, assign84870_e130555_d_n8, assign84870_e130555_d_n9, assign84870_e130555_d_n10, assign84870_e130555_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1979 != 0.0)) && (locals.var_guard1980 != 0.0)) {
        let assign84870_e130550: f64 = (locals.var_vx__blk1884 + p.p137);
        let assign84870_e130552: f64 = (assign84870_e130550 + locals.var_tmf2);
        let assign84870_e130553: f64 = (0.5 * assign84870_e130552);
        (assign84870_e130553, (0.5 * (locals.var_vx__blk1884_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_vx__blk1884_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_vx__blk1884_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_vx__blk1884_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_vx__blk1884_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_vx__blk1884_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_vx__blk1884_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_vx__blk1884_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_vx__blk1884_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_vx__blk1884_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign84870_e130555;
        locals.var_t2_dn0 = assign84870_e130555_d_n0;
        locals.var_t2_dn2 = assign84870_e130555_d_n2;
        locals.var_t2_dn4 = assign84870_e130555_d_n4;
        locals.var_t2_dn5 = assign84870_e130555_d_n5;
        locals.var_t2_dn6 = assign84870_e130555_d_n6;
        locals.var_t2_dn7 = assign84870_e130555_d_n7;
        locals.var_t2_dn8 = assign84870_e130555_d_n8;
        locals.var_t2_dn9 = assign84870_e130555_d_n9;
        locals.var_t2_dn10 = assign84870_e130555_d_n10;
        locals.var_t2_dn13 = assign84870_e130555_d_n13;
        locals.var_t2_rv = 0.0;

        let assign84880_e130558: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1982 = assign84880_e130558;
        locals.var_guard1982_rv = 0.0;

        let (assign84890_e130568, assign84890_e130568_d_n0, assign84890_e130568_d_n2, assign84890_e130568_d_n4, assign84890_e130568_d_n5, assign84890_e130568_d_n6, assign84890_e130568_d_n7, assign84890_e130568_d_n8, assign84890_e130568_d_n9, assign84890_e130568_d_n10, assign84890_e130568_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1979 != 0.0)) && (locals.var_guard1980 != 0.0)) && (locals.var_guard1982 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign84890_e130568;
        locals.var_t2_dn0 = assign84890_e130568_d_n0;
        locals.var_t2_dn2 = assign84890_e130568_d_n2;
        locals.var_t2_dn4 = assign84890_e130568_d_n4;
        locals.var_t2_dn5 = assign84890_e130568_d_n5;
        locals.var_t2_dn6 = assign84890_e130568_d_n6;
        locals.var_t2_dn7 = assign84890_e130568_d_n7;
        locals.var_t2_dn8 = assign84890_e130568_d_n8;
        locals.var_t2_dn9 = assign84890_e130568_d_n9;
        locals.var_t2_dn10 = assign84890_e130568_d_n10;
        locals.var_t2_dn13 = assign84890_e130568_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign84900_e130578, assign84900_e130578_d_n0, assign84900_e130578_d_n2, assign84900_e130578_d_n4, assign84900_e130578_d_n5, assign84900_e130578_d_n6, assign84900_e130578_d_n7, assign84900_e130578_d_n8, assign84900_e130578_d_n9, assign84900_e130578_d_n10, assign84900_e130578_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1979 != 0.0)) && (locals.var_guard1980 != 0.0)) && (locals.var_guard1982 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign84900_e130578;
        locals.var_t9_dn0 = assign84900_e130578_d_n0;
        locals.var_t9_dn2 = assign84900_e130578_d_n2;
        locals.var_t9_dn4 = assign84900_e130578_d_n4;
        locals.var_t9_dn5 = assign84900_e130578_d_n5;
        locals.var_t9_dn6 = assign84900_e130578_d_n6;
        locals.var_t9_dn7 = assign84900_e130578_d_n7;
        locals.var_t9_dn8 = assign84900_e130578_d_n8;
        locals.var_t9_dn9 = assign84900_e130578_d_n9;
        locals.var_t9_dn10 = assign84900_e130578_d_n10;
        locals.var_t9_dn13 = assign84900_e130578_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign84910_e130591, assign84910_e130591_d_n0, assign84910_e130591_d_n2, assign84910_e130591_d_n4, assign84910_e130591_d_n5, assign84910_e130591_d_n6, assign84910_e130591_d_n7, assign84910_e130591_d_n8, assign84910_e130591_d_n9, assign84910_e130591_d_n10, assign84910_e130591_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1979 != 0.0)) && (locals.var_guard1980 != 0.0)) {
        let assign84910_e130586: f64 = (locals.var_kjunc * locals.var_t2);
        let assign84910_e130587: f64 = (assign84910_e130586).sqrt();
        let assign84910_e130589: f64 = (assign84910_e130587 * p.p432);
        (assign84910_e130589, ((((locals.var_kjunc_dn0 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn0)) / (2.0 * assign84910_e130587)) * p.p432), ((((locals.var_kjunc_dn2 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn2)) / (2.0 * assign84910_e130587)) * p.p432), ((((locals.var_kjunc_dn4 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn4)) / (2.0 * assign84910_e130587)) * p.p432), ((((locals.var_kjunc_dn5 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn5)) / (2.0 * assign84910_e130587)) * p.p432), ((((locals.var_kjunc_dn6 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn6)) / (2.0 * assign84910_e130587)) * p.p432), ((((locals.var_kjunc_dn7 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn7)) / (2.0 * assign84910_e130587)) * p.p432), ((((locals.var_kjunc_dn8 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn8)) / (2.0 * assign84910_e130587)) * p.p432), ((((locals.var_kjunc_dn9 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn9)) / (2.0 * assign84910_e130587)) * p.p432), ((((locals.var_kjunc_dn10 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn10)) / (2.0 * assign84910_e130587)) * p.p432), ((((locals.var_kjunc_dn13 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn13)) / (2.0 * assign84910_e130587)) * p.p432),)
    } else {
        (locals.var_wjunc0, locals.var_wjunc0_dn0, locals.var_wjunc0_dn2, locals.var_wjunc0_dn4, locals.var_wjunc0_dn5, locals.var_wjunc0_dn6, locals.var_wjunc0_dn7, locals.var_wjunc0_dn8, locals.var_wjunc0_dn9, locals.var_wjunc0_dn10, locals.var_wjunc0_dn13,)
    }
};
        locals.var_wjunc0 = assign84910_e130591;
        locals.var_wjunc0_dn0 = assign84910_e130591_d_n0;
        locals.var_wjunc0_dn2 = assign84910_e130591_d_n2;
        locals.var_wjunc0_dn4 = assign84910_e130591_d_n4;
        locals.var_wjunc0_dn5 = assign84910_e130591_d_n5;
        locals.var_wjunc0_dn6 = assign84910_e130591_d_n6;
        locals.var_wjunc0_dn7 = assign84910_e130591_d_n7;
        locals.var_wjunc0_dn8 = assign84910_e130591_d_n8;
        locals.var_wjunc0_dn9 = assign84910_e130591_d_n9;
        locals.var_wjunc0_dn10 = assign84910_e130591_d_n10;
        locals.var_wjunc0_dn13 = assign84910_e130591_d_n13;
        locals.var_wjunc0_rv = 0.0;

        let (assign84920_e130605, assign84920_e130605_d_n0, assign84920_e130605_d_n2, assign84920_e130605_d_n4, assign84920_e130605_d_n5, assign84920_e130605_d_n6, assign84920_e130605_d_n7, assign84920_e130605_d_n8, assign84920_e130605_d_n9, assign84920_e130605_d_n10, assign84920_e130605_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1979 != 0.0)) && (locals.var_guard1980 != 0.0)) {
        let assign84920_e130599: f64 = (locals.var_lover_func - locals.var_wjunc0);
        let assign84920_e130602: f64 = (0.1 * locals.var_lover_func);
        let assign84920_e130603: f64 = (assign84920_e130599 - assign84920_e130602);
        (assign84920_e130603, ((locals.var_lover_func_dn0 - locals.var_wjunc0_dn0) - (0.1 * locals.var_lover_func_dn0)), ((locals.var_lover_func_dn2 - locals.var_wjunc0_dn2) - (0.1 * locals.var_lover_func_dn2)), ((locals.var_lover_func_dn4 - locals.var_wjunc0_dn4) - (0.1 * locals.var_lover_func_dn4)), ((locals.var_lover_func_dn5 - locals.var_wjunc0_dn5) - (0.1 * locals.var_lover_func_dn5)), ((locals.var_lover_func_dn6 - locals.var_wjunc0_dn6) - (0.1 * locals.var_lover_func_dn6)), ((locals.var_lover_func_dn7 - locals.var_wjunc0_dn7) - (0.1 * locals.var_lover_func_dn7)), ((locals.var_lover_func_dn8 - locals.var_wjunc0_dn8) - (0.1 * locals.var_lover_func_dn8)), ((locals.var_lover_func_dn9 - locals.var_wjunc0_dn9) - (0.1 * locals.var_lover_func_dn9)), ((locals.var_lover_func_dn10 - locals.var_wjunc0_dn10) - (0.1 * locals.var_lover_func_dn10)), ((locals.var_lover_func_dn13 - locals.var_wjunc0_dn13) - (0.1 * locals.var_lover_func_dn13)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign84920_e130605;
        locals.var_tmf1_dn0 = assign84920_e130605_d_n0;
        locals.var_tmf1_dn2 = assign84920_e130605_d_n2;
        locals.var_tmf1_dn4 = assign84920_e130605_d_n4;
        locals.var_tmf1_dn5 = assign84920_e130605_d_n5;
        locals.var_tmf1_dn6 = assign84920_e130605_d_n6;
        locals.var_tmf1_dn7 = assign84920_e130605_d_n7;
        locals.var_tmf1_dn8 = assign84920_e130605_d_n8;
        locals.var_tmf1_dn9 = assign84920_e130605_d_n9;
        locals.var_tmf1_dn10 = assign84920_e130605_d_n10;
        locals.var_tmf1_dn13 = assign84920_e130605_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign84930_e130619, assign84930_e130619_d_n0, assign84930_e130619_d_n2, assign84930_e130619_d_n4, assign84930_e130619_d_n5, assign84930_e130619_d_n6, assign84930_e130619_d_n7, assign84930_e130619_d_n8, assign84930_e130619_d_n9, assign84930_e130619_d_n10, assign84930_e130619_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1979 != 0.0)) && (locals.var_guard1980 != 0.0)) {
        let assign84930_e130613: f64 = (4.0 * locals.var_lover_func);
        let assign84930_e130616: f64 = (0.1 * locals.var_lover_func);
        let assign84930_e130617: f64 = (assign84930_e130613 * assign84930_e130616);
        (assign84930_e130617, (((4.0 * locals.var_lover_func_dn0) * assign84930_e130616) + (assign84930_e130613 * (0.1 * locals.var_lover_func_dn0))), (((4.0 * locals.var_lover_func_dn2) * assign84930_e130616) + (assign84930_e130613 * (0.1 * locals.var_lover_func_dn2))), (((4.0 * locals.var_lover_func_dn4) * assign84930_e130616) + (assign84930_e130613 * (0.1 * locals.var_lover_func_dn4))), (((4.0 * locals.var_lover_func_dn5) * assign84930_e130616) + (assign84930_e130613 * (0.1 * locals.var_lover_func_dn5))), (((4.0 * locals.var_lover_func_dn6) * assign84930_e130616) + (assign84930_e130613 * (0.1 * locals.var_lover_func_dn6))), (((4.0 * locals.var_lover_func_dn7) * assign84930_e130616) + (assign84930_e130613 * (0.1 * locals.var_lover_func_dn7))), (((4.0 * locals.var_lover_func_dn8) * assign84930_e130616) + (assign84930_e130613 * (0.1 * locals.var_lover_func_dn8))), (((4.0 * locals.var_lover_func_dn9) * assign84930_e130616) + (assign84930_e130613 * (0.1 * locals.var_lover_func_dn9))), (((4.0 * locals.var_lover_func_dn10) * assign84930_e130616) + (assign84930_e130613 * (0.1 * locals.var_lover_func_dn10))), (((4.0 * locals.var_lover_func_dn13) * assign84930_e130616) + (assign84930_e130613 * (0.1 * locals.var_lover_func_dn13))),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign84930_e130619;
        locals.var_tmf2_dn0 = assign84930_e130619_d_n0;
        locals.var_tmf2_dn2 = assign84930_e130619_d_n2;
        locals.var_tmf2_dn4 = assign84930_e130619_d_n4;
        locals.var_tmf2_dn5 = assign84930_e130619_d_n5;
        locals.var_tmf2_dn6 = assign84930_e130619_d_n6;
        locals.var_tmf2_dn7 = assign84930_e130619_d_n7;
        locals.var_tmf2_dn8 = assign84930_e130619_d_n8;
        locals.var_tmf2_dn9 = assign84930_e130619_d_n9;
        locals.var_tmf2_dn10 = assign84930_e130619_d_n10;
        locals.var_tmf2_dn13 = assign84930_e130619_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign84940_e130633, assign84940_e130633_d_n0, assign84940_e130633_d_n2, assign84940_e130633_d_n4, assign84940_e130633_d_n5, assign84940_e130633_d_n6, assign84940_e130633_d_n7, assign84940_e130633_d_n8, assign84940_e130633_d_n9, assign84940_e130633_d_n10, assign84940_e130633_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1979 != 0.0)) && (locals.var_guard1980 != 0.0)) {
        let (assign84940_e130631, assign84940_e130631_d_n0, assign84940_e130631_d_n2, assign84940_e130631_d_n4, assign84940_e130631_d_n5, assign84940_e130631_d_n6, assign84940_e130631_d_n7, assign84940_e130631_d_n8, assign84940_e130631_d_n9, assign84940_e130631_d_n10, assign84940_e130631_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign84940_e130630: f64 = (-locals.var_tmf2);
                (assign84940_e130630, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign84940_e130631, assign84940_e130631_d_n0, assign84940_e130631_d_n2, assign84940_e130631_d_n4, assign84940_e130631_d_n5, assign84940_e130631_d_n6, assign84940_e130631_d_n7, assign84940_e130631_d_n8, assign84940_e130631_d_n9, assign84940_e130631_d_n10, assign84940_e130631_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign84940_e130633;
        locals.var_tmf2_dn0 = assign84940_e130633_d_n0;
        locals.var_tmf2_dn2 = assign84940_e130633_d_n2;
        locals.var_tmf2_dn4 = assign84940_e130633_d_n4;
        locals.var_tmf2_dn5 = assign84940_e130633_d_n5;
        locals.var_tmf2_dn6 = assign84940_e130633_d_n6;
        locals.var_tmf2_dn7 = assign84940_e130633_d_n7;
        locals.var_tmf2_dn8 = assign84940_e130633_d_n8;
        locals.var_tmf2_dn9 = assign84940_e130633_d_n9;
        locals.var_tmf2_dn10 = assign84940_e130633_d_n10;
        locals.var_tmf2_dn13 = assign84940_e130633_d_n13;
        locals.var_tmf2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_315(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign84950_e130646, assign84950_e130646_d_n0, assign84950_e130646_d_n2, assign84950_e130646_d_n4, assign84950_e130646_d_n5, assign84950_e130646_d_n6, assign84950_e130646_d_n7, assign84950_e130646_d_n8, assign84950_e130646_d_n9, assign84950_e130646_d_n10, assign84950_e130646_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1979 != 0.0)) && (locals.var_guard1980 != 0.0)) {
        let assign84950_e130641: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign84950_e130643: f64 = (assign84950_e130641 + locals.var_tmf2);
        let assign84950_e130644: f64 = (assign84950_e130643).sqrt();
        (assign84950_e130644, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign84950_e130644)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign84950_e130644)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign84950_e130644)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign84950_e130644)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign84950_e130644)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign84950_e130644)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign84950_e130644)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign84950_e130644)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign84950_e130644)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign84950_e130644)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign84950_e130646;
        locals.var_tmf2_dn0 = assign84950_e130646_d_n0;
        locals.var_tmf2_dn2 = assign84950_e130646_d_n2;
        locals.var_tmf2_dn4 = assign84950_e130646_d_n4;
        locals.var_tmf2_dn5 = assign84950_e130646_d_n5;
        locals.var_tmf2_dn6 = assign84950_e130646_d_n6;
        locals.var_tmf2_dn7 = assign84950_e130646_d_n7;
        locals.var_tmf2_dn8 = assign84950_e130646_d_n8;
        locals.var_tmf2_dn9 = assign84950_e130646_d_n9;
        locals.var_tmf2_dn10 = assign84950_e130646_d_n10;
        locals.var_tmf2_dn13 = assign84950_e130646_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign84960_e130660, assign84960_e130660_d_n0, assign84960_e130660_d_n2, assign84960_e130660_d_n4, assign84960_e130660_d_n5, assign84960_e130660_d_n6, assign84960_e130660_d_n7, assign84960_e130660_d_n8, assign84960_e130660_d_n9, assign84960_e130660_d_n10, assign84960_e130660_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1979 != 0.0)) && (locals.var_guard1980 != 0.0)) {
        let assign84960_e130656: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign84960_e130657: f64 = (1.0 + assign84960_e130656);
        let assign84960_e130658: f64 = (0.5 * assign84960_e130657);
        (assign84960_e130658, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign84960_e130660;
        locals.var_t0_dn0 = assign84960_e130660_d_n0;
        locals.var_t0_dn2 = assign84960_e130660_d_n2;
        locals.var_t0_dn4 = assign84960_e130660_d_n4;
        locals.var_t0_dn5 = assign84960_e130660_d_n5;
        locals.var_t0_dn6 = assign84960_e130660_d_n6;
        locals.var_t0_dn7 = assign84960_e130660_d_n7;
        locals.var_t0_dn8 = assign84960_e130660_d_n8;
        locals.var_t0_dn9 = assign84960_e130660_d_n9;
        locals.var_t0_dn10 = assign84960_e130660_d_n10;
        locals.var_t0_dn13 = assign84960_e130660_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign84970_e130674, assign84970_e130674_d_n0, assign84970_e130674_d_n2, assign84970_e130674_d_n4, assign84970_e130674_d_n5, assign84970_e130674_d_n6, assign84970_e130674_d_n7, assign84970_e130674_d_n8, assign84970_e130674_d_n9, assign84970_e130674_d_n10, assign84970_e130674_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1979 != 0.0)) && (locals.var_guard1980 != 0.0)) {
        let assign84970_e130670: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign84970_e130671: f64 = (0.5 * assign84970_e130670);
        let assign84970_e130672: f64 = (locals.var_lover_func - assign84970_e130671);
        (assign84970_e130672, (locals.var_lover_func_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_lover_func_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_lover_func_dn4 - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_lover_func_dn5 - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_lover_func_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_lover_func_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_lover_func_dn8 - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_lover_func_dn9 - (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (locals.var_lover_func_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_lover_func_dn13 - (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13))),)
    } else {
        (locals.var_wjuncld, locals.var_wjuncld_dn0, locals.var_wjuncld_dn2, locals.var_wjuncld_dn4, locals.var_wjuncld_dn5, locals.var_wjuncld_dn6, locals.var_wjuncld_dn7, locals.var_wjuncld_dn8, locals.var_wjuncld_dn9, locals.var_wjuncld_dn10, locals.var_wjuncld_dn13,)
    }
};
        locals.var_wjuncld = assign84970_e130674;
        locals.var_wjuncld_dn0 = assign84970_e130674_d_n0;
        locals.var_wjuncld_dn2 = assign84970_e130674_d_n2;
        locals.var_wjuncld_dn4 = assign84970_e130674_d_n4;
        locals.var_wjuncld_dn5 = assign84970_e130674_d_n5;
        locals.var_wjuncld_dn6 = assign84970_e130674_d_n6;
        locals.var_wjuncld_dn7 = assign84970_e130674_d_n7;
        locals.var_wjuncld_dn8 = assign84970_e130674_d_n8;
        locals.var_wjuncld_dn9 = assign84970_e130674_d_n9;
        locals.var_wjuncld_dn10 = assign84970_e130674_d_n10;
        locals.var_wjuncld_dn13 = assign84970_e130674_d_n13;
        locals.var_wjuncld_rv = 0.0;

        let (assign84980_e130684, assign84980_e130684_d_n0, assign84980_e130684_d_n2, assign84980_e130684_d_n4, assign84980_e130684_d_n5, assign84980_e130684_d_n6, assign84980_e130684_d_n7, assign84980_e130684_d_n8, assign84980_e130684_d_n9, assign84980_e130684_d_n10, assign84980_e130684_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1979 != 0.0)) && (locals.var_guard1980 != 0.0)) {
        let assign84980_e130682: f64 = (locals.var_lover_func - locals.var_wjuncld);
        (assign84980_e130682, (locals.var_lover_func_dn0 - locals.var_wjuncld_dn0), (locals.var_lover_func_dn2 - locals.var_wjuncld_dn2), (locals.var_lover_func_dn4 - locals.var_wjuncld_dn4), (locals.var_lover_func_dn5 - locals.var_wjuncld_dn5), (locals.var_lover_func_dn6 - locals.var_wjuncld_dn6), (locals.var_lover_func_dn7 - locals.var_wjuncld_dn7), (locals.var_lover_func_dn8 - locals.var_wjuncld_dn8), (locals.var_lover_func_dn9 - locals.var_wjuncld_dn9), (locals.var_lover_func_dn10 - locals.var_wjuncld_dn10), (locals.var_lover_func_dn13 - locals.var_wjuncld_dn13),)
    } else {
        (locals.var_lover_func, locals.var_lover_func_dn0, locals.var_lover_func_dn2, locals.var_lover_func_dn4, locals.var_lover_func_dn5, locals.var_lover_func_dn6, locals.var_lover_func_dn7, locals.var_lover_func_dn8, locals.var_lover_func_dn9, locals.var_lover_func_dn10, locals.var_lover_func_dn13,)
    }
};
        locals.var_lover_func = assign84980_e130684;
        locals.var_lover_func_dn0 = assign84980_e130684_d_n0;
        locals.var_lover_func_dn2 = assign84980_e130684_d_n2;
        locals.var_lover_func_dn4 = assign84980_e130684_d_n4;
        locals.var_lover_func_dn5 = assign84980_e130684_d_n5;
        locals.var_lover_func_dn6 = assign84980_e130684_d_n6;
        locals.var_lover_func_dn7 = assign84980_e130684_d_n7;
        locals.var_lover_func_dn8 = assign84980_e130684_d_n8;
        locals.var_lover_func_dn9 = assign84980_e130684_d_n9;
        locals.var_lover_func_dn10 = assign84980_e130684_d_n10;
        locals.var_lover_func_dn13 = assign84980_e130684_d_n13;
        locals.var_lover_func_rv = 0.0;

        let assign84990_e130687: f64 = if 3.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1983 = assign84990_e130687;
        locals.var_guard1983_rv = 0.0;

        let assign85000_e130690: f64 = if 3.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1984 = assign85000_e130690;
        locals.var_guard1984_rv = 0.0;

        let assign85010_e130693: f64 = if 3.0 == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard1985 = assign85010_e130693;
        locals.var_guard1985_rv = 0.0;

        let assign85020_e130696: f64 = if 3.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1986 = assign85020_e130696;
        locals.var_guard1986_rv = 0.0;

        let assign85030_e130699: f64 = if p.p55 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1987 = assign85030_e130699;
        locals.var_guard1987_rv = 0.0;

        let (assign85040_e130709, assign85040_e130709_d_n0, assign85040_e130709_d_n2, assign85040_e130709_d_n4, assign85040_e130709_d_n5, assign85040_e130709_d_n6, assign85040_e130709_d_n7, assign85040_e130709_d_n8, assign85040_e130709_d_n9, assign85040_e130709_d_n10, assign85040_e130709_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1983 != 0.0)) && (locals.var_guard1987 != 0.0)) {
        let assign85040_e130707: f64 = (locals.var_weffcv_nf * locals.var_lover_func);
        (assign85040_e130707, (locals.var_weffcv_nf * locals.var_lover_func_dn0), (locals.var_weffcv_nf * locals.var_lover_func_dn2), (locals.var_weffcv_nf * locals.var_lover_func_dn4), (locals.var_weffcv_nf * locals.var_lover_func_dn5), (locals.var_weffcv_nf * locals.var_lover_func_dn6), (locals.var_weffcv_nf * locals.var_lover_func_dn7), (locals.var_weffcv_nf * locals.var_lover_func_dn8), (locals.var_weffcv_nf * locals.var_lover_func_dn9), (locals.var_weffcv_nf * locals.var_lover_func_dn10), (locals.var_weffcv_nf * locals.var_lover_func_dn13),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign85040_e130709;
        locals.var_t4_dn0 = assign85040_e130709_d_n0;
        locals.var_t4_dn2 = assign85040_e130709_d_n2;
        locals.var_t4_dn4 = assign85040_e130709_d_n4;
        locals.var_t4_dn5 = assign85040_e130709_d_n5;
        locals.var_t4_dn6 = assign85040_e130709_d_n6;
        locals.var_t4_dn7 = assign85040_e130709_d_n7;
        locals.var_t4_dn8 = assign85040_e130709_d_n8;
        locals.var_t4_dn9 = assign85040_e130709_d_n9;
        locals.var_t4_dn10 = assign85040_e130709_d_n10;
        locals.var_t4_dn13 = assign85040_e130709_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign85050_e130724, assign85050_e130724_d_n0, assign85050_e130724_d_n2, assign85050_e130724_d_n4, assign85050_e130724_d_n5, assign85050_e130724_d_n6, assign85050_e130724_d_n7, assign85050_e130724_d_n8, assign85050_e130724_d_n9, assign85050_e130724_d_n10, assign85050_e130724_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1983 != 0.0)) && (locals.var_guard1987 == 0.0)) {
        let assign85050_e130718: f64 = (locals.var_weffcv_nf * locals.var_lover_func);
        let assign85050_e130721: f64 = (1.0 - locals.var_uc_cvdsover);
        let assign85050_e130722: f64 = (assign85050_e130718 * assign85050_e130721);
        (assign85050_e130722, ((locals.var_weffcv_nf * locals.var_lover_func_dn0) * assign85050_e130721), ((locals.var_weffcv_nf * locals.var_lover_func_dn2) * assign85050_e130721), ((locals.var_weffcv_nf * locals.var_lover_func_dn4) * assign85050_e130721), ((locals.var_weffcv_nf * locals.var_lover_func_dn5) * assign85050_e130721), ((locals.var_weffcv_nf * locals.var_lover_func_dn6) * assign85050_e130721), ((locals.var_weffcv_nf * locals.var_lover_func_dn7) * assign85050_e130721), ((locals.var_weffcv_nf * locals.var_lover_func_dn8) * assign85050_e130721), ((locals.var_weffcv_nf * locals.var_lover_func_dn9) * assign85050_e130721), ((locals.var_weffcv_nf * locals.var_lover_func_dn10) * assign85050_e130721), ((locals.var_weffcv_nf * locals.var_lover_func_dn13) * assign85050_e130721),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign85050_e130724;
        locals.var_t4_dn0 = assign85050_e130724_d_n0;
        locals.var_t4_dn2 = assign85050_e130724_d_n2;
        locals.var_t4_dn4 = assign85050_e130724_d_n4;
        locals.var_t4_dn5 = assign85050_e130724_d_n5;
        locals.var_t4_dn6 = assign85050_e130724_d_n6;
        locals.var_t4_dn7 = assign85050_e130724_d_n7;
        locals.var_t4_dn8 = assign85050_e130724_d_n8;
        locals.var_t4_dn9 = assign85050_e130724_d_n9;
        locals.var_t4_dn10 = assign85050_e130724_d_n10;
        locals.var_t4_dn13 = assign85050_e130724_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign85060_e130732, assign85060_e130732_d_n0, assign85060_e130732_d_n2, assign85060_e130732_d_n4, assign85060_e130732_d_n5, assign85060_e130732_d_n6, assign85060_e130732_d_n7, assign85060_e130732_d_n8, assign85060_e130732_d_n9, assign85060_e130732_d_n10, assign85060_e130732_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1983 != 0.0)) {
        let assign85060_e130730: f64 = (locals.var_t4 * locals.var_qsuld);
        (assign85060_e130730, ((locals.var_t4_dn0 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn0)), ((locals.var_t4_dn2 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn2)), ((locals.var_t4_dn4 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn4)), ((locals.var_t4_dn5 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn5)), ((locals.var_t4_dn6 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn6)), ((locals.var_t4_dn7 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn7)), ((locals.var_t4_dn8 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn8)), ((locals.var_t4_dn9 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn9)), ((locals.var_t4_dn10 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn10)), ((locals.var_t4_dn13 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn13)),)
    } else {
        (locals.var_qovs, locals.var_qovs_dn0, locals.var_qovs_dn2, locals.var_qovs_dn4, locals.var_qovs_dn5, locals.var_qovs_dn6, locals.var_qovs_dn7, locals.var_qovs_dn8, locals.var_qovs_dn9, locals.var_qovs_dn10, locals.var_qovs_dn13,)
    }
};
        locals.var_qovs = assign85060_e130732;
        locals.var_qovs_dn0 = assign85060_e130732_d_n0;
        locals.var_qovs_dn2 = assign85060_e130732_d_n2;
        locals.var_qovs_dn4 = assign85060_e130732_d_n4;
        locals.var_qovs_dn5 = assign85060_e130732_d_n5;
        locals.var_qovs_dn6 = assign85060_e130732_d_n6;
        locals.var_qovs_dn7 = assign85060_e130732_d_n7;
        locals.var_qovs_dn8 = assign85060_e130732_d_n8;
        locals.var_qovs_dn9 = assign85060_e130732_d_n9;
        locals.var_qovs_dn10 = assign85060_e130732_d_n10;
        locals.var_qovs_dn13 = assign85060_e130732_d_n13;
        locals.var_qovs_rv = 0.0;

        let (assign85070_e130740, assign85070_e130740_d_n0, assign85070_e130740_d_n2, assign85070_e130740_d_n4, assign85070_e130740_d_n5, assign85070_e130740_d_n6, assign85070_e130740_d_n7, assign85070_e130740_d_n8, assign85070_e130740_d_n9, assign85070_e130740_d_n10, assign85070_e130740_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1983 != 0.0)) {
        let assign85070_e130738: f64 = (locals.var_t4 * locals.var_qbuld);
        (assign85070_e130738, ((locals.var_t4_dn0 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn0)), ((locals.var_t4_dn2 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn2)), ((locals.var_t4_dn4 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn4)), ((locals.var_t4_dn5 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn5)), ((locals.var_t4_dn6 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn6)), ((locals.var_t4_dn7 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn7)), ((locals.var_t4_dn8 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn8)), ((locals.var_t4_dn9 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn9)), ((locals.var_t4_dn10 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn10)), ((locals.var_t4_dn13 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn13)),)
    } else {
        (locals.var_qbsld, locals.var_qbsld_dn0, locals.var_qbsld_dn2, locals.var_qbsld_dn4, locals.var_qbsld_dn5, locals.var_qbsld_dn6, locals.var_qbsld_dn7, locals.var_qbsld_dn8, locals.var_qbsld_dn9, locals.var_qbsld_dn10, locals.var_qbsld_dn13,)
    }
};
        locals.var_qbsld = assign85070_e130740;
        locals.var_qbsld_dn0 = assign85070_e130740_d_n0;
        locals.var_qbsld_dn2 = assign85070_e130740_d_n2;
        locals.var_qbsld_dn4 = assign85070_e130740_d_n4;
        locals.var_qbsld_dn5 = assign85070_e130740_d_n5;
        locals.var_qbsld_dn6 = assign85070_e130740_d_n6;
        locals.var_qbsld_dn7 = assign85070_e130740_d_n7;
        locals.var_qbsld_dn8 = assign85070_e130740_d_n8;
        locals.var_qbsld_dn9 = assign85070_e130740_d_n9;
        locals.var_qbsld_dn10 = assign85070_e130740_d_n10;
        locals.var_qbsld_dn13 = assign85070_e130740_d_n13;
        locals.var_qbsld_rv = 0.0;

        let (assign85100_e130765, assign85100_e130765_d_n0, assign85100_e130765_d_n2, assign85100_e130765_d_n4, assign85100_e130765_d_n5, assign85100_e130765_d_n6, assign85100_e130765_d_n7, assign85100_e130765_d_n8, assign85100_e130765_d_n9, assign85100_e130765_d_n10, assign85100_e130765_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard1984 != 0.0) && (locals.var_guard1983 == 0.0))) {
        let assign85100_e130761: f64 = (locals.var_weffcv_nf * locals.var_lover_func);
        let assign85100_e130763: f64 = (assign85100_e130761 * locals.var_uc_cvdsover);
        (assign85100_e130763, ((locals.var_weffcv_nf * locals.var_lover_func_dn0) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn2) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn4) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn5) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn6) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn7) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn8) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn9) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn10) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn13) * locals.var_uc_cvdsover),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign85100_e130765;
        locals.var_t4_dn0 = assign85100_e130765_d_n0;
        locals.var_t4_dn2 = assign85100_e130765_d_n2;
        locals.var_t4_dn4 = assign85100_e130765_d_n4;
        locals.var_t4_dn5 = assign85100_e130765_d_n5;
        locals.var_t4_dn6 = assign85100_e130765_d_n6;
        locals.var_t4_dn7 = assign85100_e130765_d_n7;
        locals.var_t4_dn8 = assign85100_e130765_d_n8;
        locals.var_t4_dn9 = assign85100_e130765_d_n9;
        locals.var_t4_dn10 = assign85100_e130765_d_n10;
        locals.var_t4_dn13 = assign85100_e130765_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign85110_e130776, assign85110_e130776_d_n0, assign85110_e130776_d_n2, assign85110_e130776_d_n4, assign85110_e130776_d_n5, assign85110_e130776_d_n6, assign85110_e130776_d_n7, assign85110_e130776_d_n8, assign85110_e130776_d_n9, assign85110_e130776_d_n10, assign85110_e130776_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard1984 != 0.0) && (locals.var_guard1983 == 0.0))) {
        let assign85110_e130774: f64 = (locals.var_t4 * locals.var_qsuld);
        (assign85110_e130774, ((locals.var_t4_dn0 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn0)), ((locals.var_t4_dn2 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn2)), ((locals.var_t4_dn4 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn4)), ((locals.var_t4_dn5 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn5)), ((locals.var_t4_dn6 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn6)), ((locals.var_t4_dn7 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn7)), ((locals.var_t4_dn8 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn8)), ((locals.var_t4_dn9 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn9)), ((locals.var_t4_dn10 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn10)), ((locals.var_t4_dn13 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn13)),)
    } else {
        (locals.var_qovsext, locals.var_qovsext_dn0, locals.var_qovsext_dn2, locals.var_qovsext_dn4, locals.var_qovsext_dn5, locals.var_qovsext_dn6, locals.var_qovsext_dn7, locals.var_qovsext_dn8, locals.var_qovsext_dn9, locals.var_qovsext_dn10, locals.var_qovsext_dn13,)
    }
};
        locals.var_qovsext = assign85110_e130776;
        locals.var_qovsext_dn0 = assign85110_e130776_d_n0;
        locals.var_qovsext_dn2 = assign85110_e130776_d_n2;
        locals.var_qovsext_dn4 = assign85110_e130776_d_n4;
        locals.var_qovsext_dn5 = assign85110_e130776_d_n5;
        locals.var_qovsext_dn6 = assign85110_e130776_d_n6;
        locals.var_qovsext_dn7 = assign85110_e130776_d_n7;
        locals.var_qovsext_dn8 = assign85110_e130776_d_n8;
        locals.var_qovsext_dn9 = assign85110_e130776_d_n9;
        locals.var_qovsext_dn10 = assign85110_e130776_d_n10;
        locals.var_qovsext_dn13 = assign85110_e130776_d_n13;
        locals.var_qovsext_rv = 0.0;

        let (assign85120_e130787, assign85120_e130787_d_n0, assign85120_e130787_d_n2, assign85120_e130787_d_n4, assign85120_e130787_d_n5, assign85120_e130787_d_n6, assign85120_e130787_d_n7, assign85120_e130787_d_n8, assign85120_e130787_d_n9, assign85120_e130787_d_n10, assign85120_e130787_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard1984 != 0.0) && (locals.var_guard1983 == 0.0))) {
        let assign85120_e130785: f64 = (locals.var_t4 * locals.var_qbuld);
        (assign85120_e130785, ((locals.var_t4_dn0 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn0)), ((locals.var_t4_dn2 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn2)), ((locals.var_t4_dn4 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn4)), ((locals.var_t4_dn5 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn5)), ((locals.var_t4_dn6 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn6)), ((locals.var_t4_dn7 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn7)), ((locals.var_t4_dn8 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn8)), ((locals.var_t4_dn9 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn9)), ((locals.var_t4_dn10 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn10)), ((locals.var_t4_dn13 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn13)),)
    } else {
        (locals.var_qbsldext, locals.var_qbsldext_dn0, locals.var_qbsldext_dn2, locals.var_qbsldext_dn4, locals.var_qbsldext_dn5, locals.var_qbsldext_dn6, locals.var_qbsldext_dn7, locals.var_qbsldext_dn8, locals.var_qbsldext_dn9, locals.var_qbsldext_dn10, locals.var_qbsldext_dn13,)
    }
};
        locals.var_qbsldext = assign85120_e130787;
        locals.var_qbsldext_dn0 = assign85120_e130787_d_n0;
        locals.var_qbsldext_dn2 = assign85120_e130787_d_n2;
        locals.var_qbsldext_dn4 = assign85120_e130787_d_n4;
        locals.var_qbsldext_dn5 = assign85120_e130787_d_n5;
        locals.var_qbsldext_dn6 = assign85120_e130787_d_n6;
        locals.var_qbsldext_dn7 = assign85120_e130787_d_n7;
        locals.var_qbsldext_dn8 = assign85120_e130787_d_n8;
        locals.var_qbsldext_dn9 = assign85120_e130787_d_n9;
        locals.var_qbsldext_dn10 = assign85120_e130787_d_n10;
        locals.var_qbsldext_dn13 = assign85120_e130787_d_n13;
        locals.var_qbsldext_rv = 0.0;

        let assign85130_e130790: f64 = if p.p55 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1988 = assign85130_e130790;
        locals.var_guard1988_rv = 0.0;

        let (assign85140_e130805, assign85140_e130805_d_n0, assign85140_e130805_d_n2, assign85140_e130805_d_n4, assign85140_e130805_d_n5, assign85140_e130805_d_n6, assign85140_e130805_d_n7, assign85140_e130805_d_n8, assign85140_e130805_d_n9, assign85140_e130805_d_n10, assign85140_e130805_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && ((locals.var_guard1985 != 0.0) && (!((locals.var_guard1983 != 0.0) || (locals.var_guard1984 != 0.0))))) && (locals.var_guard1988 != 0.0)) {
        let assign85140_e130803: f64 = (locals.var_weffcv_nf * locals.var_lover_func);
        (assign85140_e130803, (locals.var_weffcv_nf * locals.var_lover_func_dn0), (locals.var_weffcv_nf * locals.var_lover_func_dn2), (locals.var_weffcv_nf * locals.var_lover_func_dn4), (locals.var_weffcv_nf * locals.var_lover_func_dn5), (locals.var_weffcv_nf * locals.var_lover_func_dn6), (locals.var_weffcv_nf * locals.var_lover_func_dn7), (locals.var_weffcv_nf * locals.var_lover_func_dn8), (locals.var_weffcv_nf * locals.var_lover_func_dn9), (locals.var_weffcv_nf * locals.var_lover_func_dn10), (locals.var_weffcv_nf * locals.var_lover_func_dn13),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign85140_e130805;
        locals.var_t4_dn0 = assign85140_e130805_d_n0;
        locals.var_t4_dn2 = assign85140_e130805_d_n2;
        locals.var_t4_dn4 = assign85140_e130805_d_n4;
        locals.var_t4_dn5 = assign85140_e130805_d_n5;
        locals.var_t4_dn6 = assign85140_e130805_d_n6;
        locals.var_t4_dn7 = assign85140_e130805_d_n7;
        locals.var_t4_dn8 = assign85140_e130805_d_n8;
        locals.var_t4_dn9 = assign85140_e130805_d_n9;
        locals.var_t4_dn10 = assign85140_e130805_d_n10;
        locals.var_t4_dn13 = assign85140_e130805_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign85150_e130825, assign85150_e130825_d_n0, assign85150_e130825_d_n2, assign85150_e130825_d_n4, assign85150_e130825_d_n5, assign85150_e130825_d_n6, assign85150_e130825_d_n7, assign85150_e130825_d_n8, assign85150_e130825_d_n9, assign85150_e130825_d_n10, assign85150_e130825_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && ((locals.var_guard1985 != 0.0) && (!((locals.var_guard1983 != 0.0) || (locals.var_guard1984 != 0.0))))) && (locals.var_guard1988 == 0.0)) {
        let assign85150_e130819: f64 = (locals.var_weffcv_nf * locals.var_lover_func);
        let assign85150_e130822: f64 = (1.0 - locals.var_uc_cvdsover);
        let assign85150_e130823: f64 = (assign85150_e130819 * assign85150_e130822);
        (assign85150_e130823, ((locals.var_weffcv_nf * locals.var_lover_func_dn0) * assign85150_e130822), ((locals.var_weffcv_nf * locals.var_lover_func_dn2) * assign85150_e130822), ((locals.var_weffcv_nf * locals.var_lover_func_dn4) * assign85150_e130822), ((locals.var_weffcv_nf * locals.var_lover_func_dn5) * assign85150_e130822), ((locals.var_weffcv_nf * locals.var_lover_func_dn6) * assign85150_e130822), ((locals.var_weffcv_nf * locals.var_lover_func_dn7) * assign85150_e130822), ((locals.var_weffcv_nf * locals.var_lover_func_dn8) * assign85150_e130822), ((locals.var_weffcv_nf * locals.var_lover_func_dn9) * assign85150_e130822), ((locals.var_weffcv_nf * locals.var_lover_func_dn10) * assign85150_e130822), ((locals.var_weffcv_nf * locals.var_lover_func_dn13) * assign85150_e130822),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign85150_e130825;
        locals.var_t4_dn0 = assign85150_e130825_d_n0;
        locals.var_t4_dn2 = assign85150_e130825_d_n2;
        locals.var_t4_dn4 = assign85150_e130825_d_n4;
        locals.var_t4_dn5 = assign85150_e130825_d_n5;
        locals.var_t4_dn6 = assign85150_e130825_d_n6;
        locals.var_t4_dn7 = assign85150_e130825_d_n7;
        locals.var_t4_dn8 = assign85150_e130825_d_n8;
        locals.var_t4_dn9 = assign85150_e130825_d_n9;
        locals.var_t4_dn10 = assign85150_e130825_d_n10;
        locals.var_t4_dn13 = assign85150_e130825_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign85160_e130836, assign85160_e130836_d_n0, assign85160_e130836_d_n2, assign85160_e130836_d_n4, assign85160_e130836_d_n5, assign85160_e130836_d_n6, assign85160_e130836_d_n7, assign85160_e130836_d_n8, assign85160_e130836_d_n9, assign85160_e130836_d_n10, assign85160_e130836_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard1985 != 0.0) && (!((locals.var_guard1983 != 0.0) || (locals.var_guard1984 != 0.0))))) {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    } else {
        (locals.var_rd_ps0ld, locals.var_rd_ps0ld_dn0, locals.var_rd_ps0ld_dn2, locals.var_rd_ps0ld_dn4, locals.var_rd_ps0ld_dn5, locals.var_rd_ps0ld_dn6, locals.var_rd_ps0ld_dn7, locals.var_rd_ps0ld_dn8, locals.var_rd_ps0ld_dn9, locals.var_rd_ps0ld_dn10, locals.var_rd_ps0ld_dn13,)
    }
};
        locals.var_rd_ps0ld = assign85160_e130836;
        locals.var_rd_ps0ld_dn0 = assign85160_e130836_d_n0;
        locals.var_rd_ps0ld_dn2 = assign85160_e130836_d_n2;
        locals.var_rd_ps0ld_dn4 = assign85160_e130836_d_n4;
        locals.var_rd_ps0ld_dn5 = assign85160_e130836_d_n5;
        locals.var_rd_ps0ld_dn6 = assign85160_e130836_d_n6;
        locals.var_rd_ps0ld_dn7 = assign85160_e130836_d_n7;
        locals.var_rd_ps0ld_dn8 = assign85160_e130836_d_n8;
        locals.var_rd_ps0ld_dn9 = assign85160_e130836_d_n9;
        locals.var_rd_ps0ld_dn10 = assign85160_e130836_d_n10;
        locals.var_rd_ps0ld_dn13 = assign85160_e130836_d_n13;
        locals.var_rd_ps0ld_rv = 0.0;

        let assign85170_e130839: f64 = if p.p430 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1989 = assign85170_e130839;
        locals.var_guard1989_rv = 0.0;

        let (assign85180_e130852, assign85180_e130852_d_n0, assign85180_e130852_d_n2, assign85180_e130852_d_n4, assign85180_e130852_d_n5, assign85180_e130852_d_n6, assign85180_e130852_d_n7, assign85180_e130852_d_n8, assign85180_e130852_d_n9, assign85180_e130852_d_n10, assign85180_e130852_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && ((locals.var_guard1985 != 0.0) && (!((locals.var_guard1983 != 0.0) || (locals.var_guard1984 != 0.0))))) && (locals.var_guard1989 != 0.0)) {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn8, locals.var_qbuld_dn9, locals.var_qbuld_dn10, locals.var_qbuld_dn13,)
    } else {
        (locals.var_rd_qbuld, locals.var_rd_qbuld_dn0, locals.var_rd_qbuld_dn2, locals.var_rd_qbuld_dn4, locals.var_rd_qbuld_dn5, locals.var_rd_qbuld_dn6, locals.var_rd_qbuld_dn7, locals.var_rd_qbuld_dn8, locals.var_rd_qbuld_dn9, locals.var_rd_qbuld_dn10, locals.var_rd_qbuld_dn13,)
    }
};
        locals.var_rd_qbuld = assign85180_e130852;
        locals.var_rd_qbuld_dn0 = assign85180_e130852_d_n0;
        locals.var_rd_qbuld_dn2 = assign85180_e130852_d_n2;
        locals.var_rd_qbuld_dn4 = assign85180_e130852_d_n4;
        locals.var_rd_qbuld_dn5 = assign85180_e130852_d_n5;
        locals.var_rd_qbuld_dn6 = assign85180_e130852_d_n6;
        locals.var_rd_qbuld_dn7 = assign85180_e130852_d_n7;
        locals.var_rd_qbuld_dn8 = assign85180_e130852_d_n8;
        locals.var_rd_qbuld_dn9 = assign85180_e130852_d_n9;
        locals.var_rd_qbuld_dn10 = assign85180_e130852_d_n10;
        locals.var_rd_qbuld_dn13 = assign85180_e130852_d_n13;
        locals.var_rd_qbuld_rv = 0.0;

        let (assign85190_e130865, assign85190_e130865_d_n0, assign85190_e130865_d_n2, assign85190_e130865_d_n4, assign85190_e130865_d_n5, assign85190_e130865_d_n6, assign85190_e130865_d_n7, assign85190_e130865_d_n8, assign85190_e130865_d_n9, assign85190_e130865_d_n10, assign85190_e130865_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard1985 != 0.0) && (!((locals.var_guard1983 != 0.0) || (locals.var_guard1984 != 0.0))))) {
        let assign85190_e130863: f64 = (locals.var_t4 * locals.var_qsuld);
        (assign85190_e130863, ((locals.var_t4_dn0 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn0)), ((locals.var_t4_dn2 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn2)), ((locals.var_t4_dn4 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn4)), ((locals.var_t4_dn5 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn5)), ((locals.var_t4_dn6 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn6)), ((locals.var_t4_dn7 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn7)), ((locals.var_t4_dn8 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn8)), ((locals.var_t4_dn9 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn9)), ((locals.var_t4_dn10 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn10)), ((locals.var_t4_dn13 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn13)),)
    } else {
        (locals.var_qovd, locals.var_qovd_dn0, locals.var_qovd_dn2, locals.var_qovd_dn4, locals.var_qovd_dn5, locals.var_qovd_dn6, locals.var_qovd_dn7, locals.var_qovd_dn8, locals.var_qovd_dn9, locals.var_qovd_dn10, locals.var_qovd_dn13,)
    }
};
        locals.var_qovd = assign85190_e130865;
        locals.var_qovd_dn0 = assign85190_e130865_d_n0;
        locals.var_qovd_dn2 = assign85190_e130865_d_n2;
        locals.var_qovd_dn4 = assign85190_e130865_d_n4;
        locals.var_qovd_dn5 = assign85190_e130865_d_n5;
        locals.var_qovd_dn6 = assign85190_e130865_d_n6;
        locals.var_qovd_dn7 = assign85190_e130865_d_n7;
        locals.var_qovd_dn8 = assign85190_e130865_d_n8;
        locals.var_qovd_dn9 = assign85190_e130865_d_n9;
        locals.var_qovd_dn10 = assign85190_e130865_d_n10;
        locals.var_qovd_dn13 = assign85190_e130865_d_n13;
        locals.var_qovd_rv = 0.0;

        let (assign85200_e130878, assign85200_e130878_d_n0, assign85200_e130878_d_n2, assign85200_e130878_d_n4, assign85200_e130878_d_n5, assign85200_e130878_d_n6, assign85200_e130878_d_n7, assign85200_e130878_d_n8, assign85200_e130878_d_n9, assign85200_e130878_d_n10, assign85200_e130878_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard1985 != 0.0) && (!((locals.var_guard1983 != 0.0) || (locals.var_guard1984 != 0.0))))) {
        let assign85200_e130876: f64 = (locals.var_t4 * locals.var_qbuld);
        (assign85200_e130876, ((locals.var_t4_dn0 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn0)), ((locals.var_t4_dn2 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn2)), ((locals.var_t4_dn4 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn4)), ((locals.var_t4_dn5 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn5)), ((locals.var_t4_dn6 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn6)), ((locals.var_t4_dn7 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn7)), ((locals.var_t4_dn8 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn8)), ((locals.var_t4_dn9 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn9)), ((locals.var_t4_dn10 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn10)), ((locals.var_t4_dn13 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn13)),)
    } else {
        (locals.var_qbdld, locals.var_qbdld_dn0, locals.var_qbdld_dn2, locals.var_qbdld_dn4, locals.var_qbdld_dn5, locals.var_qbdld_dn6, locals.var_qbdld_dn7, locals.var_qbdld_dn8, locals.var_qbdld_dn9, locals.var_qbdld_dn10, locals.var_qbdld_dn13,)
    }
};
        locals.var_qbdld = assign85200_e130878;
        locals.var_qbdld_dn0 = assign85200_e130878_d_n0;
        locals.var_qbdld_dn2 = assign85200_e130878_d_n2;
        locals.var_qbdld_dn4 = assign85200_e130878_d_n4;
        locals.var_qbdld_dn5 = assign85200_e130878_d_n5;
        locals.var_qbdld_dn6 = assign85200_e130878_d_n6;
        locals.var_qbdld_dn7 = assign85200_e130878_d_n7;
        locals.var_qbdld_dn8 = assign85200_e130878_d_n8;
        locals.var_qbdld_dn9 = assign85200_e130878_d_n9;
        locals.var_qbdld_dn10 = assign85200_e130878_d_n10;
        locals.var_qbdld_dn13 = assign85200_e130878_d_n13;
        locals.var_qbdld_rv = 0.0;

        let (assign85210_e130889, assign85210_e130889_d_n0, assign85210_e130889_d_n2, assign85210_e130889_d_n4, assign85210_e130889_d_n5, assign85210_e130889_d_n6, assign85210_e130889_d_n7, assign85210_e130889_d_n8, assign85210_e130889_d_n9, assign85210_e130889_d_n10, assign85210_e130889_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard1985 != 0.0) && (!((locals.var_guard1983 != 0.0) || (locals.var_guard1984 != 0.0))))) {
        (locals.var_qbdld, locals.var_qbdld_dn0, locals.var_qbdld_dn2, locals.var_qbdld_dn4, locals.var_qbdld_dn5, locals.var_qbdld_dn6, locals.var_qbdld_dn7, locals.var_qbdld_dn8, locals.var_qbdld_dn9, locals.var_qbdld_dn10, locals.var_qbdld_dn13,)
    } else {
        (locals.var_qbd_qs, locals.var_qbd_qs_dn0, locals.var_qbd_qs_dn2, locals.var_qbd_qs_dn4, locals.var_qbd_qs_dn5, locals.var_qbd_qs_dn6, locals.var_qbd_qs_dn7, locals.var_qbd_qs_dn8, locals.var_qbd_qs_dn9, locals.var_qbd_qs_dn10, locals.var_qbd_qs_dn13,)
    }
};
        locals.var_qbd_qs = assign85210_e130889;
        locals.var_qbd_qs_dn0 = assign85210_e130889_d_n0;
        locals.var_qbd_qs_dn2 = assign85210_e130889_d_n2;
        locals.var_qbd_qs_dn4 = assign85210_e130889_d_n4;
        locals.var_qbd_qs_dn5 = assign85210_e130889_d_n5;
        locals.var_qbd_qs_dn6 = assign85210_e130889_d_n6;
        locals.var_qbd_qs_dn7 = assign85210_e130889_d_n7;
        locals.var_qbd_qs_dn8 = assign85210_e130889_d_n8;
        locals.var_qbd_qs_dn9 = assign85210_e130889_d_n9;
        locals.var_qbd_qs_dn10 = assign85210_e130889_d_n10;
        locals.var_qbd_qs_dn13 = assign85210_e130889_d_n13;
        locals.var_qbd_qs_rv = 0.0;

        let (assign85220_e130906, assign85220_e130906_d_n0, assign85220_e130906_d_n2, assign85220_e130906_d_n4, assign85220_e130906_d_n5, assign85220_e130906_d_n6, assign85220_e130906_d_n7, assign85220_e130906_d_n8, assign85220_e130906_d_n9, assign85220_e130906_d_n10, assign85220_e130906_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard1986 != 0.0) && (!(((locals.var_guard1983 != 0.0) || (locals.var_guard1984 != 0.0)) || (locals.var_guard1985 != 0.0))))) {
        let assign85220_e130902: f64 = (locals.var_weffcv_nf * locals.var_lover_func);
        let assign85220_e130904: f64 = (assign85220_e130902 * locals.var_uc_cvdsover);
        (assign85220_e130904, ((locals.var_weffcv_nf * locals.var_lover_func_dn0) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn2) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn4) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn5) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn6) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn7) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn8) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn9) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn10) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn13) * locals.var_uc_cvdsover),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign85220_e130906;
        locals.var_t4_dn0 = assign85220_e130906_d_n0;
        locals.var_t4_dn2 = assign85220_e130906_d_n2;
        locals.var_t4_dn4 = assign85220_e130906_d_n4;
        locals.var_t4_dn5 = assign85220_e130906_d_n5;
        locals.var_t4_dn6 = assign85220_e130906_d_n6;
        locals.var_t4_dn7 = assign85220_e130906_d_n7;
        locals.var_t4_dn8 = assign85220_e130906_d_n8;
        locals.var_t4_dn9 = assign85220_e130906_d_n9;
        locals.var_t4_dn10 = assign85220_e130906_d_n10;
        locals.var_t4_dn13 = assign85220_e130906_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign85230_e130921, assign85230_e130921_d_n0, assign85230_e130921_d_n2, assign85230_e130921_d_n4, assign85230_e130921_d_n5, assign85230_e130921_d_n6, assign85230_e130921_d_n7, assign85230_e130921_d_n8, assign85230_e130921_d_n9, assign85230_e130921_d_n10, assign85230_e130921_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard1986 != 0.0) && (!(((locals.var_guard1983 != 0.0) || (locals.var_guard1984 != 0.0)) || (locals.var_guard1985 != 0.0))))) {
        let assign85230_e130919: f64 = (locals.var_t4 * locals.var_qsuld);
        (assign85230_e130919, ((locals.var_t4_dn0 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn0)), ((locals.var_t4_dn2 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn2)), ((locals.var_t4_dn4 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn4)), ((locals.var_t4_dn5 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn5)), ((locals.var_t4_dn6 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn6)), ((locals.var_t4_dn7 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn7)), ((locals.var_t4_dn8 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn8)), ((locals.var_t4_dn9 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn9)), ((locals.var_t4_dn10 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn10)), ((locals.var_t4_dn13 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn13)),)
    } else {
        (locals.var_qovdext, locals.var_qovdext_dn0, locals.var_qovdext_dn2, locals.var_qovdext_dn4, locals.var_qovdext_dn5, locals.var_qovdext_dn6, locals.var_qovdext_dn7, locals.var_qovdext_dn8, locals.var_qovdext_dn9, locals.var_qovdext_dn10, locals.var_qovdext_dn13,)
    }
};
        locals.var_qovdext = assign85230_e130921;
        locals.var_qovdext_dn0 = assign85230_e130921_d_n0;
        locals.var_qovdext_dn2 = assign85230_e130921_d_n2;
        locals.var_qovdext_dn4 = assign85230_e130921_d_n4;
        locals.var_qovdext_dn5 = assign85230_e130921_d_n5;
        locals.var_qovdext_dn6 = assign85230_e130921_d_n6;
        locals.var_qovdext_dn7 = assign85230_e130921_d_n7;
        locals.var_qovdext_dn8 = assign85230_e130921_d_n8;
        locals.var_qovdext_dn9 = assign85230_e130921_d_n9;
        locals.var_qovdext_dn10 = assign85230_e130921_d_n10;
        locals.var_qovdext_dn13 = assign85230_e130921_d_n13;
        locals.var_qovdext_rv = 0.0;

        let (assign85240_e130936, assign85240_e130936_d_n0, assign85240_e130936_d_n2, assign85240_e130936_d_n4, assign85240_e130936_d_n5, assign85240_e130936_d_n6, assign85240_e130936_d_n7, assign85240_e130936_d_n8, assign85240_e130936_d_n9, assign85240_e130936_d_n10, assign85240_e130936_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard1986 != 0.0) && (!(((locals.var_guard1983 != 0.0) || (locals.var_guard1984 != 0.0)) || (locals.var_guard1985 != 0.0))))) {
        let assign85240_e130934: f64 = (locals.var_t4 * locals.var_qbuld);
        (assign85240_e130934, ((locals.var_t4_dn0 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn0)), ((locals.var_t4_dn2 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn2)), ((locals.var_t4_dn4 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn4)), ((locals.var_t4_dn5 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn5)), ((locals.var_t4_dn6 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn6)), ((locals.var_t4_dn7 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn7)), ((locals.var_t4_dn8 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn8)), ((locals.var_t4_dn9 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn9)), ((locals.var_t4_dn10 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn10)), ((locals.var_t4_dn13 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn13)),)
    } else {
        (locals.var_qbdldext, locals.var_qbdldext_dn0, locals.var_qbdldext_dn2, locals.var_qbdldext_dn4, locals.var_qbdldext_dn5, locals.var_qbdldext_dn6, locals.var_qbdldext_dn7, locals.var_qbdldext_dn8, locals.var_qbdldext_dn9, locals.var_qbdldext_dn10, locals.var_qbdldext_dn13,)
    }
};
        locals.var_qbdldext = assign85240_e130936;
        locals.var_qbdldext_dn0 = assign85240_e130936_d_n0;
        locals.var_qbdldext_dn2 = assign85240_e130936_d_n2;
        locals.var_qbdldext_dn4 = assign85240_e130936_d_n4;
        locals.var_qbdldext_dn5 = assign85240_e130936_d_n5;
        locals.var_qbdldext_dn6 = assign85240_e130936_d_n6;
        locals.var_qbdldext_dn7 = assign85240_e130936_d_n7;
        locals.var_qbdldext_dn8 = assign85240_e130936_d_n8;
        locals.var_qbdldext_dn9 = assign85240_e130936_d_n9;
        locals.var_qbdldext_dn10 = assign85240_e130936_d_n10;
        locals.var_qbdldext_dn13 = assign85240_e130936_d_n13;
        locals.var_qbdldext_rv = 0.0;

        locals.var_flg_calcqover = 0.0;
        locals.var_flg_calcqover_rv = 0.0;

        let assign85260_e130940: f64 = if 4.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1990 = assign85260_e130940;
        locals.var_guard1990_rv = 0.0;

        let assign85270_e130943: f64 = if 4.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1991 = assign85270_e130943;
        locals.var_guard1991_rv = 0.0;

        let assign85280_e130946: f64 = if 4.0 == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard1992 = assign85280_e130946;
        locals.var_guard1992_rv = 0.0;

        let assign85290_e130949: f64 = if 4.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1993 = assign85290_e130949;
        locals.var_guard1993_rv = 0.0;

        let assign85300_e130960: f64 = if (((p.p36 == 1.0) && (p.p66 > 0.0)) && (locals.var_uc_novers > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1994 = assign85300_e130960;
        locals.var_guard1994_rv = 0.0;

        let (assign85310_e130966,) = {
    if ((locals.var_guard1990 != 0.0) && (locals.var_guard1994 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_calcqover,)
    }
};
        locals.var_flg_calcqover = assign85310_e130966;
        locals.var_flg_calcqover_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_316(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign85320_e130972,) = {
    if ((locals.var_guard1990 != 0.0) && (locals.var_guard1994 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_coovlps,)
    }
};
        locals.var_flg_coovlps = assign85320_e130972;
        locals.var_flg_coovlps_rv = 0.0;

        let (assign85330_e130980, assign85330_e130980_d_n2, assign85330_e130980_d_n6, assign85330_e130980_d_n7, assign85330_e130980_d_n8,) = {
    if ((locals.var_guard1990 != 0.0) && (locals.var_guard1994 != 0.0)) {
        let assign85330_e130978: f64 = (locals.var_vgsi - locals.var_vbsi);
        (assign85330_e130978, 0.0, locals.var_vgsi_dn6, (locals.var_vgsi_dn7 - locals.var_vbsi_dn7), (-locals.var_vbsi_dn8),)
    } else {
        (locals.var_vgbgmt, locals.var_vgbgmt_dn2, locals.var_vgbgmt_dn6, locals.var_vgbgmt_dn7, locals.var_vgbgmt_dn8,)
    }
};
        locals.var_vgbgmt = assign85330_e130980;
        locals.var_vgbgmt_dn2 = assign85330_e130980_d_n2;
        locals.var_vgbgmt_dn6 = assign85330_e130980_d_n6;
        locals.var_vgbgmt_dn7 = assign85330_e130980_d_n7;
        locals.var_vgbgmt_dn8 = assign85330_e130980_d_n8;
        locals.var_vgbgmt_rv = 0.0;

        let (assign85340_e130987, assign85340_e130987_d_n0, assign85340_e130987_d_n2, assign85340_e130987_d_n4, assign85340_e130987_d_n5, assign85340_e130987_d_n6, assign85340_e130987_d_n7, assign85340_e130987_d_n8, assign85340_e130987_d_n9, assign85340_e130987_d_n10, assign85340_e130987_d_n13,) = {
    if ((locals.var_guard1990 != 0.0) && (locals.var_guard1994 != 0.0)) {
        let assign85340_e130985: f64 = (-locals.var_vbsi);
        (assign85340_e130985, 0.0, 0.0, 0.0, 0.0, 0.0, (-locals.var_vbsi_dn7), (-locals.var_vbsi_dn8), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn13,)
    }
};
        locals.var_vxbgmt = assign85340_e130987;
        locals.var_vxbgmt_dn0 = assign85340_e130987_d_n0;
        locals.var_vxbgmt_dn2 = assign85340_e130987_d_n2;
        locals.var_vxbgmt_dn4 = assign85340_e130987_d_n4;
        locals.var_vxbgmt_dn5 = assign85340_e130987_d_n5;
        locals.var_vxbgmt_dn6 = assign85340_e130987_d_n6;
        locals.var_vxbgmt_dn7 = assign85340_e130987_d_n7;
        locals.var_vxbgmt_dn8 = assign85340_e130987_d_n8;
        locals.var_vxbgmt_dn9 = assign85340_e130987_d_n9;
        locals.var_vxbgmt_dn10 = assign85340_e130987_d_n10;
        locals.var_vxbgmt_dn13 = assign85340_e130987_d_n13;
        locals.var_vxbgmt_rv = 0.0;

        let (assign85350_e130993,) = {
    if ((locals.var_guard1990 != 0.0) && (locals.var_guard1994 != 0.0)) {
        (locals.var_uc_novers,)
    } else {
        (locals.var_nover_func,)
    }
};
        locals.var_nover_func = assign85350_e130993;
        locals.var_nover_func_rv = 0.0;

        let (assign85360_e130999, assign85360_e130999_d_n0, assign85360_e130999_d_n2, assign85360_e130999_d_n4, assign85360_e130999_d_n5, assign85360_e130999_d_n6, assign85360_e130999_d_n7, assign85360_e130999_d_n8, assign85360_e130999_d_n9, assign85360_e130999_d_n10, assign85360_e130999_d_n13,) = {
    if ((locals.var_guard1990 != 0.0) && (locals.var_guard1994 != 0.0)) {
        (p.p66, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_lover_func, locals.var_lover_func_dn0, locals.var_lover_func_dn2, locals.var_lover_func_dn4, locals.var_lover_func_dn5, locals.var_lover_func_dn6, locals.var_lover_func_dn7, locals.var_lover_func_dn8, locals.var_lover_func_dn9, locals.var_lover_func_dn10, locals.var_lover_func_dn13,)
    }
};
        locals.var_lover_func = assign85360_e130999;
        locals.var_lover_func_dn0 = assign85360_e130999_d_n0;
        locals.var_lover_func_dn2 = assign85360_e130999_d_n2;
        locals.var_lover_func_dn4 = assign85360_e130999_d_n4;
        locals.var_lover_func_dn5 = assign85360_e130999_d_n5;
        locals.var_lover_func_dn6 = assign85360_e130999_d_n6;
        locals.var_lover_func_dn7 = assign85360_e130999_d_n7;
        locals.var_lover_func_dn8 = assign85360_e130999_d_n8;
        locals.var_lover_func_dn9 = assign85360_e130999_d_n9;
        locals.var_lover_func_dn10 = assign85360_e130999_d_n10;
        locals.var_lover_func_dn13 = assign85360_e130999_d_n13;
        locals.var_lover_func_rv = 0.0;

        let (assign85370_e131005, assign85370_e131005_d_n0, assign85370_e131005_d_n2, assign85370_e131005_d_n4, assign85370_e131005_d_n5, assign85370_e131005_d_n6, assign85370_e131005_d_n7, assign85370_e131005_d_n8, assign85370_e131005_d_n9, assign85370_e131005_d_n10, assign85370_e131005_d_n13,) = {
    if ((locals.var_guard1990 != 0.0) && (locals.var_guard1994 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_wdep_func, locals.var_wdep_func_dn0, locals.var_wdep_func_dn2, locals.var_wdep_func_dn4, locals.var_wdep_func_dn5, locals.var_wdep_func_dn6, locals.var_wdep_func_dn7, locals.var_wdep_func_dn8, locals.var_wdep_func_dn9, locals.var_wdep_func_dn10, locals.var_wdep_func_dn13,)
    }
};
        locals.var_wdep_func = assign85370_e131005;
        locals.var_wdep_func_dn0 = assign85370_e131005_d_n0;
        locals.var_wdep_func_dn2 = assign85370_e131005_d_n2;
        locals.var_wdep_func_dn4 = assign85370_e131005_d_n4;
        locals.var_wdep_func_dn5 = assign85370_e131005_d_n5;
        locals.var_wdep_func_dn6 = assign85370_e131005_d_n6;
        locals.var_wdep_func_dn7 = assign85370_e131005_d_n7;
        locals.var_wdep_func_dn8 = assign85370_e131005_d_n8;
        locals.var_wdep_func_dn9 = assign85370_e131005_d_n9;
        locals.var_wdep_func_dn10 = assign85370_e131005_d_n10;
        locals.var_wdep_func_dn13 = assign85370_e131005_d_n13;
        locals.var_wdep_func_rv = 0.0;

        let (assign85380_e131011, assign85380_e131011_d_n0, assign85380_e131011_d_n2, assign85380_e131011_d_n4, assign85380_e131011_d_n5, assign85380_e131011_d_n6, assign85380_e131011_d_n7, assign85380_e131011_d_n8, assign85380_e131011_d_n9, assign85380_e131011_d_n10, assign85380_e131011_d_n13,) = {
    if ((locals.var_guard1990 != 0.0) && (locals.var_guard1994 != 0.0)) {
        (locals.var_cnst0overs, locals.var_cnst0overs_dn0, locals.var_cnst0overs_dn2, locals.var_cnst0overs_dn4, locals.var_cnst0overs_dn5, locals.var_cnst0overs_dn6, locals.var_cnst0overs_dn7, locals.var_cnst0overs_dn8, locals.var_cnst0overs_dn9, locals.var_cnst0overs_dn10, locals.var_cnst0overs_dn13,)
    } else {
        (locals.var_cnst0over_func, locals.var_cnst0over_func_dn0, locals.var_cnst0over_func_dn2, locals.var_cnst0over_func_dn4, locals.var_cnst0over_func_dn5, locals.var_cnst0over_func_dn6, locals.var_cnst0over_func_dn7, locals.var_cnst0over_func_dn8, locals.var_cnst0over_func_dn9, locals.var_cnst0over_func_dn10, locals.var_cnst0over_func_dn13,)
    }
};
        locals.var_cnst0over_func = assign85380_e131011;
        locals.var_cnst0over_func_dn0 = assign85380_e131011_d_n0;
        locals.var_cnst0over_func_dn2 = assign85380_e131011_d_n2;
        locals.var_cnst0over_func_dn4 = assign85380_e131011_d_n4;
        locals.var_cnst0over_func_dn5 = assign85380_e131011_d_n5;
        locals.var_cnst0over_func_dn6 = assign85380_e131011_d_n6;
        locals.var_cnst0over_func_dn7 = assign85380_e131011_d_n7;
        locals.var_cnst0over_func_dn8 = assign85380_e131011_d_n8;
        locals.var_cnst0over_func_dn9 = assign85380_e131011_d_n9;
        locals.var_cnst0over_func_dn10 = assign85380_e131011_d_n10;
        locals.var_cnst0over_func_dn13 = assign85380_e131011_d_n13;
        locals.var_cnst0over_func_rv = 0.0;

        let (assign85390_e131017,) = {
    if ((locals.var_guard1990 != 0.0) && (locals.var_guard1994 != 0.0)) {
        (locals.var_cox0,)
    } else {
        (locals.var_cox0_func,)
    }
};
        locals.var_cox0_func = assign85390_e131017;
        locals.var_cox0_func_rv = 0.0;

        let assign85400_e131036: f64 = if (((((p.p36 == 1.0) && (p.p66 > 0.0)) && (locals.var_uc_novers > 0.0)) && (locals.var_uc_cvdsover != 0.0)) && (p.p55 != 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard1995 = assign85400_e131036;
        locals.var_guard1995_rv = 0.0;

        let (assign85410_e131045,) = {
    if (((locals.var_guard1991 != 0.0) && (locals.var_guard1990 == 0.0)) && (locals.var_guard1995 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_calcqover,)
    }
};
        locals.var_flg_calcqover = assign85410_e131045;
        locals.var_flg_calcqover_rv = 0.0;

        let (assign85420_e131056, assign85420_e131056_d_n2, assign85420_e131056_d_n6, assign85420_e131056_d_n7, assign85420_e131056_d_n8,) = {
    if (((locals.var_guard1991 != 0.0) && (locals.var_guard1990 == 0.0)) && (locals.var_guard1995 != 0.0)) {
        let assign85420_e131054: f64 = (locals.var_vgsei - locals.var_vbsei);
        (assign85420_e131054, (locals.var_vgsei_dn2 - locals.var_vbsei_dn2), locals.var_vgsei_dn6, 0.0, (-locals.var_vbsei_dn8),)
    } else {
        (locals.var_vgbgmt, locals.var_vgbgmt_dn2, locals.var_vgbgmt_dn6, locals.var_vgbgmt_dn7, locals.var_vgbgmt_dn8,)
    }
};
        locals.var_vgbgmt = assign85420_e131056;
        locals.var_vgbgmt_dn2 = assign85420_e131056_d_n2;
        locals.var_vgbgmt_dn6 = assign85420_e131056_d_n6;
        locals.var_vgbgmt_dn7 = assign85420_e131056_d_n7;
        locals.var_vgbgmt_dn8 = assign85420_e131056_d_n8;
        locals.var_vgbgmt_rv = 0.0;

        let (assign85430_e131066, assign85430_e131066_d_n0, assign85430_e131066_d_n2, assign85430_e131066_d_n4, assign85430_e131066_d_n5, assign85430_e131066_d_n6, assign85430_e131066_d_n7, assign85430_e131066_d_n8, assign85430_e131066_d_n9, assign85430_e131066_d_n10, assign85430_e131066_d_n13,) = {
    if (((locals.var_guard1991 != 0.0) && (locals.var_guard1990 == 0.0)) && (locals.var_guard1995 != 0.0)) {
        let assign85430_e131064: f64 = (-locals.var_vbsei);
        (assign85430_e131064, 0.0, (-locals.var_vbsei_dn2), 0.0, 0.0, 0.0, 0.0, (-locals.var_vbsei_dn8), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn13,)
    }
};
        locals.var_vxbgmt = assign85430_e131066;
        locals.var_vxbgmt_dn0 = assign85430_e131066_d_n0;
        locals.var_vxbgmt_dn2 = assign85430_e131066_d_n2;
        locals.var_vxbgmt_dn4 = assign85430_e131066_d_n4;
        locals.var_vxbgmt_dn5 = assign85430_e131066_d_n5;
        locals.var_vxbgmt_dn6 = assign85430_e131066_d_n6;
        locals.var_vxbgmt_dn7 = assign85430_e131066_d_n7;
        locals.var_vxbgmt_dn8 = assign85430_e131066_d_n8;
        locals.var_vxbgmt_dn9 = assign85430_e131066_d_n9;
        locals.var_vxbgmt_dn10 = assign85430_e131066_d_n10;
        locals.var_vxbgmt_dn13 = assign85430_e131066_d_n13;
        locals.var_vxbgmt_rv = 0.0;

        let assign85440_e131077: f64 = if (((p.p35 == 1.0) && (p.p63 > 0.0)) && (locals.var_uc_nover > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1996 = assign85440_e131077;
        locals.var_guard1996_rv = 0.0;

        let (assign85450_e131088,) = {
    if (((locals.var_guard1992 != 0.0) && (!((locals.var_guard1990 != 0.0) || (locals.var_guard1991 != 0.0)))) && (locals.var_guard1996 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_calcqover,)
    }
};
        locals.var_flg_calcqover = assign85450_e131088;
        locals.var_flg_calcqover_rv = 0.0;

        let (assign85460_e131099,) = {
    if (((locals.var_guard1992 != 0.0) && (!((locals.var_guard1990 != 0.0) || (locals.var_guard1991 != 0.0)))) && (locals.var_guard1996 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_coovlp,)
    }
};
        locals.var_flg_coovlp = assign85460_e131099;
        locals.var_flg_coovlp_rv = 0.0;

        let (assign85470_e131112, assign85470_e131112_d_n2, assign85470_e131112_d_n6, assign85470_e131112_d_n7, assign85470_e131112_d_n8,) = {
    if (((locals.var_guard1992 != 0.0) && (!((locals.var_guard1990 != 0.0) || (locals.var_guard1991 != 0.0)))) && (locals.var_guard1996 != 0.0)) {
        let assign85470_e131110: f64 = (locals.var_vgsi - locals.var_vbsi);
        (assign85470_e131110, 0.0, locals.var_vgsi_dn6, (locals.var_vgsi_dn7 - locals.var_vbsi_dn7), (-locals.var_vbsi_dn8),)
    } else {
        (locals.var_vgbgmt, locals.var_vgbgmt_dn2, locals.var_vgbgmt_dn6, locals.var_vgbgmt_dn7, locals.var_vgbgmt_dn8,)
    }
};
        locals.var_vgbgmt = assign85470_e131112;
        locals.var_vgbgmt_dn2 = assign85470_e131112_d_n2;
        locals.var_vgbgmt_dn6 = assign85470_e131112_d_n6;
        locals.var_vgbgmt_dn7 = assign85470_e131112_d_n7;
        locals.var_vgbgmt_dn8 = assign85470_e131112_d_n8;
        locals.var_vgbgmt_rv = 0.0;

        let (assign85480_e131125, assign85480_e131125_d_n0, assign85480_e131125_d_n2, assign85480_e131125_d_n4, assign85480_e131125_d_n5, assign85480_e131125_d_n6, assign85480_e131125_d_n7, assign85480_e131125_d_n8, assign85480_e131125_d_n9, assign85480_e131125_d_n10, assign85480_e131125_d_n13,) = {
    if (((locals.var_guard1992 != 0.0) && (!((locals.var_guard1990 != 0.0) || (locals.var_guard1991 != 0.0)))) && (locals.var_guard1996 != 0.0)) {
        let assign85480_e131123: f64 = (locals.var_vdsi - locals.var_vbsi);
        (assign85480_e131123, 0.0, 0.0, 0.0, locals.var_vdsi_dn5, 0.0, (locals.var_vdsi_dn7 - locals.var_vbsi_dn7), (-locals.var_vbsi_dn8), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn13,)
    }
};
        locals.var_vxbgmt = assign85480_e131125;
        locals.var_vxbgmt_dn0 = assign85480_e131125_d_n0;
        locals.var_vxbgmt_dn2 = assign85480_e131125_d_n2;
        locals.var_vxbgmt_dn4 = assign85480_e131125_d_n4;
        locals.var_vxbgmt_dn5 = assign85480_e131125_d_n5;
        locals.var_vxbgmt_dn6 = assign85480_e131125_d_n6;
        locals.var_vxbgmt_dn7 = assign85480_e131125_d_n7;
        locals.var_vxbgmt_dn8 = assign85480_e131125_d_n8;
        locals.var_vxbgmt_dn9 = assign85480_e131125_d_n9;
        locals.var_vxbgmt_dn10 = assign85480_e131125_d_n10;
        locals.var_vxbgmt_dn13 = assign85480_e131125_d_n13;
        locals.var_vxbgmt_rv = 0.0;

        let (assign85490_e131136,) = {
    if (((locals.var_guard1992 != 0.0) && (!((locals.var_guard1990 != 0.0) || (locals.var_guard1991 != 0.0)))) && (locals.var_guard1996 != 0.0)) {
        (locals.var_uc_nover,)
    } else {
        (locals.var_nover_func,)
    }
};
        locals.var_nover_func = assign85490_e131136;
        locals.var_nover_func_rv = 0.0;

        let (assign85500_e131151, assign85500_e131151_d_n0, assign85500_e131151_d_n2, assign85500_e131151_d_n4, assign85500_e131151_d_n5, assign85500_e131151_d_n6, assign85500_e131151_d_n7, assign85500_e131151_d_n8, assign85500_e131151_d_n9, assign85500_e131151_d_n10, assign85500_e131151_d_n13,) = {
    if (((locals.var_guard1992 != 0.0) && (!((locals.var_guard1990 != 0.0) || (locals.var_guard1991 != 0.0)))) && (locals.var_guard1996 != 0.0)) {
        let assign85500_e131148: f64 = (p.p64 * p.p55);
        let assign85500_e131149: f64 = (p.p63 + assign85500_e131148);
        (assign85500_e131149, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_lover_func, locals.var_lover_func_dn0, locals.var_lover_func_dn2, locals.var_lover_func_dn4, locals.var_lover_func_dn5, locals.var_lover_func_dn6, locals.var_lover_func_dn7, locals.var_lover_func_dn8, locals.var_lover_func_dn9, locals.var_lover_func_dn10, locals.var_lover_func_dn13,)
    }
};
        locals.var_lover_func = assign85500_e131151;
        locals.var_lover_func_dn0 = assign85500_e131151_d_n0;
        locals.var_lover_func_dn2 = assign85500_e131151_d_n2;
        locals.var_lover_func_dn4 = assign85500_e131151_d_n4;
        locals.var_lover_func_dn5 = assign85500_e131151_d_n5;
        locals.var_lover_func_dn6 = assign85500_e131151_d_n6;
        locals.var_lover_func_dn7 = assign85500_e131151_d_n7;
        locals.var_lover_func_dn8 = assign85500_e131151_d_n8;
        locals.var_lover_func_dn9 = assign85500_e131151_d_n9;
        locals.var_lover_func_dn10 = assign85500_e131151_d_n10;
        locals.var_lover_func_dn13 = assign85500_e131151_d_n13;
        locals.var_lover_func_rv = 0.0;

        let (assign85510_e131162, assign85510_e131162_d_n0, assign85510_e131162_d_n2, assign85510_e131162_d_n4, assign85510_e131162_d_n5, assign85510_e131162_d_n6, assign85510_e131162_d_n7, assign85510_e131162_d_n8, assign85510_e131162_d_n9, assign85510_e131162_d_n10, assign85510_e131162_d_n13,) = {
    if (((locals.var_guard1992 != 0.0) && (!((locals.var_guard1990 != 0.0) || (locals.var_guard1991 != 0.0)))) && (locals.var_guard1996 != 0.0)) {
        (locals.var_wdep, locals.var_wdep_dn0, locals.var_wdep_dn2, locals.var_wdep_dn4, locals.var_wdep_dn5, locals.var_wdep_dn6, locals.var_wdep_dn7, locals.var_wdep_dn8, locals.var_wdep_dn9, locals.var_wdep_dn10, locals.var_wdep_dn13,)
    } else {
        (locals.var_wdep_func, locals.var_wdep_func_dn0, locals.var_wdep_func_dn2, locals.var_wdep_func_dn4, locals.var_wdep_func_dn5, locals.var_wdep_func_dn6, locals.var_wdep_func_dn7, locals.var_wdep_func_dn8, locals.var_wdep_func_dn9, locals.var_wdep_func_dn10, locals.var_wdep_func_dn13,)
    }
};
        locals.var_wdep_func = assign85510_e131162;
        locals.var_wdep_func_dn0 = assign85510_e131162_d_n0;
        locals.var_wdep_func_dn2 = assign85510_e131162_d_n2;
        locals.var_wdep_func_dn4 = assign85510_e131162_d_n4;
        locals.var_wdep_func_dn5 = assign85510_e131162_d_n5;
        locals.var_wdep_func_dn6 = assign85510_e131162_d_n6;
        locals.var_wdep_func_dn7 = assign85510_e131162_d_n7;
        locals.var_wdep_func_dn8 = assign85510_e131162_d_n8;
        locals.var_wdep_func_dn9 = assign85510_e131162_d_n9;
        locals.var_wdep_func_dn10 = assign85510_e131162_d_n10;
        locals.var_wdep_func_dn13 = assign85510_e131162_d_n13;
        locals.var_wdep_func_rv = 0.0;

        let (assign85520_e131173, assign85520_e131173_d_n0, assign85520_e131173_d_n2, assign85520_e131173_d_n4, assign85520_e131173_d_n5, assign85520_e131173_d_n6, assign85520_e131173_d_n7, assign85520_e131173_d_n8, assign85520_e131173_d_n9, assign85520_e131173_d_n10, assign85520_e131173_d_n13,) = {
    if (((locals.var_guard1992 != 0.0) && (!((locals.var_guard1990 != 0.0) || (locals.var_guard1991 != 0.0)))) && (locals.var_guard1996 != 0.0)) {
        (locals.var_cnst0over, locals.var_cnst0over_dn0, locals.var_cnst0over_dn2, locals.var_cnst0over_dn4, locals.var_cnst0over_dn5, locals.var_cnst0over_dn6, locals.var_cnst0over_dn7, locals.var_cnst0over_dn8, locals.var_cnst0over_dn9, locals.var_cnst0over_dn10, locals.var_cnst0over_dn13,)
    } else {
        (locals.var_cnst0over_func, locals.var_cnst0over_func_dn0, locals.var_cnst0over_func_dn2, locals.var_cnst0over_func_dn4, locals.var_cnst0over_func_dn5, locals.var_cnst0over_func_dn6, locals.var_cnst0over_func_dn7, locals.var_cnst0over_func_dn8, locals.var_cnst0over_func_dn9, locals.var_cnst0over_func_dn10, locals.var_cnst0over_func_dn13,)
    }
};
        locals.var_cnst0over_func = assign85520_e131173;
        locals.var_cnst0over_func_dn0 = assign85520_e131173_d_n0;
        locals.var_cnst0over_func_dn2 = assign85520_e131173_d_n2;
        locals.var_cnst0over_func_dn4 = assign85520_e131173_d_n4;
        locals.var_cnst0over_func_dn5 = assign85520_e131173_d_n5;
        locals.var_cnst0over_func_dn6 = assign85520_e131173_d_n6;
        locals.var_cnst0over_func_dn7 = assign85520_e131173_d_n7;
        locals.var_cnst0over_func_dn8 = assign85520_e131173_d_n8;
        locals.var_cnst0over_func_dn9 = assign85520_e131173_d_n9;
        locals.var_cnst0over_func_dn10 = assign85520_e131173_d_n10;
        locals.var_cnst0over_func_dn13 = assign85520_e131173_d_n13;
        locals.var_cnst0over_func_rv = 0.0;

        let (assign85530_e131184,) = {
    if (((locals.var_guard1992 != 0.0) && (!((locals.var_guard1990 != 0.0) || (locals.var_guard1991 != 0.0)))) && (locals.var_guard1996 != 0.0)) {
        (locals.var_coxb0,)
    } else {
        (locals.var_cox0_func,)
    }
};
        locals.var_cox0_func = assign85530_e131184;
        locals.var_cox0_func_rv = 0.0;

        let (assign85540_e131196, assign85540_e131196_d_n0, assign85540_e131196_d_n2, assign85540_e131196_d_n4, assign85540_e131196_d_n5, assign85540_e131196_d_n6, assign85540_e131196_d_n7, assign85540_e131196_d_n8, assign85540_e131196_d_n9, assign85540_e131196_d_n10, assign85540_e131196_d_n13,) = {
    if (((locals.var_guard1992 != 0.0) && (!((locals.var_guard1990 != 0.0) || (locals.var_guard1991 != 0.0)))) && (locals.var_guard1996 != 0.0)) {
        let assign85540_e131194: f64 = (-locals.var_lover_func);
        (assign85540_e131194, (-locals.var_lover_func_dn0), (-locals.var_lover_func_dn2), (-locals.var_lover_func_dn4), (-locals.var_lover_func_dn5), (-locals.var_lover_func_dn6), (-locals.var_lover_func_dn7), (-locals.var_lover_func_dn8), (-locals.var_lover_func_dn9), (-locals.var_lover_func_dn10), (-locals.var_lover_func_dn13),)
    } else {
        (locals.var_lover_func, locals.var_lover_func_dn0, locals.var_lover_func_dn2, locals.var_lover_func_dn4, locals.var_lover_func_dn5, locals.var_lover_func_dn6, locals.var_lover_func_dn7, locals.var_lover_func_dn8, locals.var_lover_func_dn9, locals.var_lover_func_dn10, locals.var_lover_func_dn13,)
    }
};
        locals.var_lover_func = assign85540_e131196;
        locals.var_lover_func_dn0 = assign85540_e131196_d_n0;
        locals.var_lover_func_dn2 = assign85540_e131196_d_n2;
        locals.var_lover_func_dn4 = assign85540_e131196_d_n4;
        locals.var_lover_func_dn5 = assign85540_e131196_d_n5;
        locals.var_lover_func_dn6 = assign85540_e131196_d_n6;
        locals.var_lover_func_dn7 = assign85540_e131196_d_n7;
        locals.var_lover_func_dn8 = assign85540_e131196_d_n8;
        locals.var_lover_func_dn9 = assign85540_e131196_d_n9;
        locals.var_lover_func_dn10 = assign85540_e131196_d_n10;
        locals.var_lover_func_dn13 = assign85540_e131196_d_n13;
        locals.var_lover_func_rv = 0.0;

        let assign85550_e131207: f64 = if (((locals.var_lover_func < 0.0) && (p.p432 > 0.0)) && (p.p55 == 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard1997 = assign85550_e131207;
        locals.var_guard1997_rv = 0.0;

        let (assign85560_e131221, assign85560_e131221_d_n0, assign85560_e131221_d_n2, assign85560_e131221_d_n4, assign85560_e131221_d_n5, assign85560_e131221_d_n6, assign85560_e131221_d_n7, assign85560_e131221_d_n8, assign85560_e131221_d_n9, assign85560_e131221_d_n10, assign85560_e131221_d_n13,) = {
    if ((((locals.var_guard1992 != 0.0) && (!((locals.var_guard1990 != 0.0) || (locals.var_guard1991 != 0.0)))) && (locals.var_guard1996 != 0.0)) && (locals.var_guard1997 != 0.0)) {
        let assign85560_e131219: f64 = (-locals.var_lover_func);
        (assign85560_e131219, (-locals.var_lover_func_dn0), (-locals.var_lover_func_dn2), (-locals.var_lover_func_dn4), (-locals.var_lover_func_dn5), (-locals.var_lover_func_dn6), (-locals.var_lover_func_dn7), (-locals.var_lover_func_dn8), (-locals.var_lover_func_dn9), (-locals.var_lover_func_dn10), (-locals.var_lover_func_dn13),)
    } else {
        (locals.var_lover_func, locals.var_lover_func_dn0, locals.var_lover_func_dn2, locals.var_lover_func_dn4, locals.var_lover_func_dn5, locals.var_lover_func_dn6, locals.var_lover_func_dn7, locals.var_lover_func_dn8, locals.var_lover_func_dn9, locals.var_lover_func_dn10, locals.var_lover_func_dn13,)
    }
};
        locals.var_lover_func = assign85560_e131221;
        locals.var_lover_func_dn0 = assign85560_e131221_d_n0;
        locals.var_lover_func_dn2 = assign85560_e131221_d_n2;
        locals.var_lover_func_dn4 = assign85560_e131221_d_n4;
        locals.var_lover_func_dn5 = assign85560_e131221_d_n5;
        locals.var_lover_func_dn6 = assign85560_e131221_d_n6;
        locals.var_lover_func_dn7 = assign85560_e131221_d_n7;
        locals.var_lover_func_dn8 = assign85560_e131221_d_n8;
        locals.var_lover_func_dn9 = assign85560_e131221_d_n9;
        locals.var_lover_func_dn10 = assign85560_e131221_d_n10;
        locals.var_lover_func_dn13 = assign85560_e131221_d_n13;
        locals.var_lover_func_rv = 0.0;

        let (assign85570_e131234, assign85570_e131234_d_n0, assign85570_e131234_d_n2, assign85570_e131234_d_n4, assign85570_e131234_d_n5, assign85570_e131234_d_n6, assign85570_e131234_d_n7, assign85570_e131234_d_n8, assign85570_e131234_d_n9, assign85570_e131234_d_n10, assign85570_e131234_d_n13,) = {
    if ((((locals.var_guard1992 != 0.0) && (!((locals.var_guard1990 != 0.0) || (locals.var_guard1991 != 0.0)))) && (locals.var_guard1996 != 0.0)) && (locals.var_guard1997 != 0.0)) {
        (p.p63, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign85570_e131234;
        locals.var_t1_dn0 = assign85570_e131234_d_n0;
        locals.var_t1_dn2 = assign85570_e131234_d_n2;
        locals.var_t1_dn4 = assign85570_e131234_d_n4;
        locals.var_t1_dn5 = assign85570_e131234_d_n5;
        locals.var_t1_dn6 = assign85570_e131234_d_n6;
        locals.var_t1_dn7 = assign85570_e131234_d_n7;
        locals.var_t1_dn8 = assign85570_e131234_d_n8;
        locals.var_t1_dn9 = assign85570_e131234_d_n9;
        locals.var_t1_dn10 = assign85570_e131234_d_n10;
        locals.var_t1_dn13 = assign85570_e131234_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign85580_e131253, assign85580_e131253_d_n0, assign85580_e131253_d_n2, assign85580_e131253_d_n4, assign85580_e131253_d_n5, assign85580_e131253_d_n6, assign85580_e131253_d_n7, assign85580_e131253_d_n8, assign85580_e131253_d_n9, assign85580_e131253_d_n10, assign85580_e131253_d_n13,) = {
    if ((((locals.var_guard1992 != 0.0) && (!((locals.var_guard1990 != 0.0) || (locals.var_guard1991 != 0.0)))) && (locals.var_guard1996 != 0.0)) && (locals.var_guard1997 != 0.0)) {
        let assign85580_e131247: f64 = (locals.var_t1 * locals.var_t1);
        let assign85580_e131249: f64 = (assign85580_e131247 / locals.var_kjunc);
        let assign85580_e131251: f64 = (assign85580_e131249 - p.p137);
        (assign85580_e131251, (((((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) * locals.var_kjunc) - (assign85580_e131247 * locals.var_kjunc_dn0)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) * locals.var_kjunc) - (assign85580_e131247 * locals.var_kjunc_dn2)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) * locals.var_kjunc) - (assign85580_e131247 * locals.var_kjunc_dn4)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) * locals.var_kjunc) - (assign85580_e131247 * locals.var_kjunc_dn5)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) * locals.var_kjunc) - (assign85580_e131247 * locals.var_kjunc_dn6)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) * locals.var_kjunc) - (assign85580_e131247 * locals.var_kjunc_dn7)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) * locals.var_kjunc) - (assign85580_e131247 * locals.var_kjunc_dn8)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) * locals.var_kjunc) - (assign85580_e131247 * locals.var_kjunc_dn9)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) * locals.var_kjunc) - (assign85580_e131247 * locals.var_kjunc_dn10)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn13 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn13)) * locals.var_kjunc) - (assign85580_e131247 * locals.var_kjunc_dn13)) / (locals.var_kjunc * locals.var_kjunc)),)
    } else {
        (locals.var_vxb_lim, locals.var_vxb_lim_dn0, locals.var_vxb_lim_dn2, locals.var_vxb_lim_dn4, locals.var_vxb_lim_dn5, locals.var_vxb_lim_dn6, locals.var_vxb_lim_dn7, locals.var_vxb_lim_dn8, locals.var_vxb_lim_dn9, locals.var_vxb_lim_dn10, locals.var_vxb_lim_dn13,)
    }
};
        locals.var_vxb_lim = assign85580_e131253;
        locals.var_vxb_lim_dn0 = assign85580_e131253_d_n0;
        locals.var_vxb_lim_dn2 = assign85580_e131253_d_n2;
        locals.var_vxb_lim_dn4 = assign85580_e131253_d_n4;
        locals.var_vxb_lim_dn5 = assign85580_e131253_d_n5;
        locals.var_vxb_lim_dn6 = assign85580_e131253_d_n6;
        locals.var_vxb_lim_dn7 = assign85580_e131253_d_n7;
        locals.var_vxb_lim_dn8 = assign85580_e131253_d_n8;
        locals.var_vxb_lim_dn9 = assign85580_e131253_d_n9;
        locals.var_vxb_lim_dn10 = assign85580_e131253_d_n10;
        locals.var_vxb_lim_dn13 = assign85580_e131253_d_n13;
        locals.var_vxb_lim_rv = 0.0;

        let assign85590_e131256: f64 = if p.p113 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1998 = assign85590_e131256;
        locals.var_guard1998_rv = 0.0;

        let assign85600_e131263: f64 = if ((locals.var_vxbgmt == 0.0) || (p.p113 <= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1999 = assign85600_e131263;
        locals.var_guard1999_rv = 0.0;

        let (assign85610_e131280, assign85610_e131280_d_n0, assign85610_e131280_d_n2, assign85610_e131280_d_n4, assign85610_e131280_d_n5, assign85610_e131280_d_n6, assign85610_e131280_d_n7, assign85610_e131280_d_n8, assign85610_e131280_d_n9, assign85610_e131280_d_n10, assign85610_e131280_d_n13,) = {
    if ((((((locals.var_guard1992 != 0.0) && (!((locals.var_guard1990 != 0.0) || (locals.var_guard1991 != 0.0)))) && (locals.var_guard1996 != 0.0)) && (locals.var_guard1997 != 0.0)) && (locals.var_guard1998 != 0.0)) && (locals.var_guard1999 != 0.0)) {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn13,)
    } else {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn13,)
    }
};
        locals.var_vxbgmt = assign85610_e131280;
        locals.var_vxbgmt_dn0 = assign85610_e131280_d_n0;
        locals.var_vxbgmt_dn2 = assign85610_e131280_d_n2;
        locals.var_vxbgmt_dn4 = assign85610_e131280_d_n4;
        locals.var_vxbgmt_dn5 = assign85610_e131280_d_n5;
        locals.var_vxbgmt_dn6 = assign85610_e131280_d_n6;
        locals.var_vxbgmt_dn7 = assign85610_e131280_d_n7;
        locals.var_vxbgmt_dn8 = assign85610_e131280_d_n8;
        locals.var_vxbgmt_dn9 = assign85610_e131280_d_n9;
        locals.var_vxbgmt_dn10 = assign85610_e131280_d_n10;
        locals.var_vxbgmt_dn13 = assign85610_e131280_d_n13;
        locals.var_vxbgmt_rv = 0.0;

        let (assign85620_e131304, assign85620_e131304_d_n0, assign85620_e131304_d_n2, assign85620_e131304_d_n4, assign85620_e131304_d_n5, assign85620_e131304_d_n6, assign85620_e131304_d_n7, assign85620_e131304_d_n8, assign85620_e131304_d_n9, assign85620_e131304_d_n10, assign85620_e131304_d_n13,) = {
    if ((((((locals.var_guard1992 != 0.0) && (!((locals.var_guard1990 != 0.0) || (locals.var_guard1991 != 0.0)))) && (locals.var_guard1996 != 0.0)) && (locals.var_guard1997 != 0.0)) && (locals.var_guard1998 != 0.0)) && (locals.var_guard1999 == 0.0)) {
        let (assign85620_e131302,) = {
            if (locals.var_vxbgmt < 0.0) {
                let assign85620_e131300: f64 = (-1.0);
                (assign85620_e131300,)
            } else {
                (1.0,)
            }
        };
        (assign85620_e131302, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf3, locals.var_tmf3_dn0, locals.var_tmf3_dn2, locals.var_tmf3_dn4, locals.var_tmf3_dn5, locals.var_tmf3_dn6, locals.var_tmf3_dn7, locals.var_tmf3_dn8, locals.var_tmf3_dn9, locals.var_tmf3_dn10, locals.var_tmf3_dn13,)
    }
};
        locals.var_tmf3 = assign85620_e131304;
        locals.var_tmf3_dn0 = assign85620_e131304_d_n0;
        locals.var_tmf3_dn2 = assign85620_e131304_d_n2;
        locals.var_tmf3_dn4 = assign85620_e131304_d_n4;
        locals.var_tmf3_dn5 = assign85620_e131304_d_n5;
        locals.var_tmf3_dn6 = assign85620_e131304_d_n6;
        locals.var_tmf3_dn7 = assign85620_e131304_d_n7;
        locals.var_tmf3_dn8 = assign85620_e131304_d_n8;
        locals.var_tmf3_dn9 = assign85620_e131304_d_n9;
        locals.var_tmf3_dn10 = assign85620_e131304_d_n10;
        locals.var_tmf3_dn13 = assign85620_e131304_d_n13;
        locals.var_tmf3_rv = 0.0;

        let (assign85630_e131324, assign85630_e131324_d_n0, assign85630_e131324_d_n2, assign85630_e131324_d_n4, assign85630_e131324_d_n5, assign85630_e131324_d_n6, assign85630_e131324_d_n7, assign85630_e131324_d_n8, assign85630_e131324_d_n9, assign85630_e131324_d_n10, assign85630_e131324_d_n13,) = {
    if ((((((locals.var_guard1992 != 0.0) && (!((locals.var_guard1990 != 0.0) || (locals.var_guard1991 != 0.0)))) && (locals.var_guard1996 != 0.0)) && (locals.var_guard1997 != 0.0)) && (locals.var_guard1998 != 0.0)) && (locals.var_guard1999 == 0.0)) {
        let assign85630_e131322: f64 = (locals.var_tmf3 * locals.var_vxbgmt);
        (assign85630_e131322, ((locals.var_tmf3_dn0 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn0)), ((locals.var_tmf3_dn2 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn2)), ((locals.var_tmf3_dn4 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn4)), ((locals.var_tmf3_dn5 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn5)), ((locals.var_tmf3_dn6 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn6)), ((locals.var_tmf3_dn7 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn7)), ((locals.var_tmf3_dn8 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn8)), ((locals.var_tmf3_dn9 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn9)), ((locals.var_tmf3_dn10 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn10)), ((locals.var_tmf3_dn13 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn13)),)
    } else {
        (locals.var_tmf4, locals.var_tmf4_dn0, locals.var_tmf4_dn2, locals.var_tmf4_dn4, locals.var_tmf4_dn5, locals.var_tmf4_dn6, locals.var_tmf4_dn7, locals.var_tmf4_dn8, locals.var_tmf4_dn9, locals.var_tmf4_dn10, locals.var_tmf4_dn13,)
    }
};
        locals.var_tmf4 = assign85630_e131324;
        locals.var_tmf4_dn0 = assign85630_e131324_d_n0;
        locals.var_tmf4_dn2 = assign85630_e131324_d_n2;
        locals.var_tmf4_dn4 = assign85630_e131324_d_n4;
        locals.var_tmf4_dn5 = assign85630_e131324_d_n5;
        locals.var_tmf4_dn6 = assign85630_e131324_d_n6;
        locals.var_tmf4_dn7 = assign85630_e131324_d_n7;
        locals.var_tmf4_dn8 = assign85630_e131324_d_n8;
        locals.var_tmf4_dn9 = assign85630_e131324_d_n9;
        locals.var_tmf4_dn10 = assign85630_e131324_d_n10;
        locals.var_tmf4_dn13 = assign85630_e131324_d_n13;
        locals.var_tmf4_rv = 0.0;

        let (assign85640_e131348, assign85640_e131348_d_n0, assign85640_e131348_d_n2, assign85640_e131348_d_n4, assign85640_e131348_d_n5, assign85640_e131348_d_n6, assign85640_e131348_d_n7, assign85640_e131348_d_n8, assign85640_e131348_d_n9, assign85640_e131348_d_n10, assign85640_e131348_d_n13,) = {
    if ((((((locals.var_guard1992 != 0.0) && (!((locals.var_guard1990 != 0.0) || (locals.var_guard1991 != 0.0)))) && (locals.var_guard1996 != 0.0)) && (locals.var_guard1997 != 0.0)) && (locals.var_guard1998 != 0.0)) && (locals.var_guard1999 == 0.0)) {
        let assign85640_e131343: f64 = (locals.var_tmf4 / locals.var_vxb_lim);
        let assign85640_e131345: f64 = (assign85640_e131343).powf(p.p113);
        let assign85640_e131346: f64 = (1.0 + assign85640_e131345);
        (assign85640_e131346, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign85640_e131343).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn0 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn0)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign85640_e131345 * (p.p113 * ((((locals.var_tmf4_dn0 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn0)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign85640_e131343))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign85640_e131343).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn2 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn2)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign85640_e131345 * (p.p113 * ((((locals.var_tmf4_dn2 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn2)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign85640_e131343))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign85640_e131343).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn4 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn4)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign85640_e131345 * (p.p113 * ((((locals.var_tmf4_dn4 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn4)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign85640_e131343))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign85640_e131343).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn5 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn5)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign85640_e131345 * (p.p113 * ((((locals.var_tmf4_dn5 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn5)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign85640_e131343))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign85640_e131343).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn6 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn6)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign85640_e131345 * (p.p113 * ((((locals.var_tmf4_dn6 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn6)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign85640_e131343))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign85640_e131343).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn7 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn7)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign85640_e131345 * (p.p113 * ((((locals.var_tmf4_dn7 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn7)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign85640_e131343))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign85640_e131343).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn8 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn8)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign85640_e131345 * (p.p113 * ((((locals.var_tmf4_dn8 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn8)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign85640_e131343))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign85640_e131343).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn9 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn9)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign85640_e131345 * (p.p113 * ((((locals.var_tmf4_dn9 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn9)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign85640_e131343))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign85640_e131343).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn10 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn10)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign85640_e131345 * (p.p113 * ((((locals.var_tmf4_dn10 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn10)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign85640_e131343))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign85640_e131343).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn13 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn13)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign85640_e131345 * (p.p113 * ((((locals.var_tmf4_dn13 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn13)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign85640_e131343))) },)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign85640_e131348;
        locals.var_tmf1_dn0 = assign85640_e131348_d_n0;
        locals.var_tmf1_dn2 = assign85640_e131348_d_n2;
        locals.var_tmf1_dn4 = assign85640_e131348_d_n4;
        locals.var_tmf1_dn5 = assign85640_e131348_d_n5;
        locals.var_tmf1_dn6 = assign85640_e131348_d_n6;
        locals.var_tmf1_dn7 = assign85640_e131348_d_n7;
        locals.var_tmf1_dn8 = assign85640_e131348_d_n8;
        locals.var_tmf1_dn9 = assign85640_e131348_d_n9;
        locals.var_tmf1_dn10 = assign85640_e131348_d_n10;
        locals.var_tmf1_dn13 = assign85640_e131348_d_n13;
        locals.var_tmf1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_317(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign85650_e131370, assign85650_e131370_d_n0, assign85650_e131370_d_n2, assign85650_e131370_d_n4, assign85650_e131370_d_n5, assign85650_e131370_d_n6, assign85650_e131370_d_n7, assign85650_e131370_d_n8, assign85650_e131370_d_n9, assign85650_e131370_d_n10, assign85650_e131370_d_n13,) = {
    if ((((((locals.var_guard1992 != 0.0) && (!((locals.var_guard1990 != 0.0) || (locals.var_guard1991 != 0.0)))) && (locals.var_guard1996 != 0.0)) && (locals.var_guard1997 != 0.0)) && (locals.var_guard1998 != 0.0)) && (locals.var_guard1999 == 0.0)) {
        let assign85650_e131367: f64 = (1.0 / p.p113);
        let assign85650_e131368: f64 = (locals.var_tmf1).powf(assign85650_e131367);
        (assign85650_e131368, if 0.0 == 0.0 && ((assign85650_e131367) as f64).is_finite() && ((assign85650_e131367) as f64).fract() == 0.0 { if assign85650_e131367 == 0.0 { 0.0 } else { (assign85650_e131367 * ((locals.var_tmf1).powf(assign85650_e131367 - 1.0) * locals.var_tmf1_dn0)) } } else { (assign85650_e131368 * (assign85650_e131367 * (locals.var_tmf1_dn0 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign85650_e131367) as f64).is_finite() && ((assign85650_e131367) as f64).fract() == 0.0 { if assign85650_e131367 == 0.0 { 0.0 } else { (assign85650_e131367 * ((locals.var_tmf1).powf(assign85650_e131367 - 1.0) * locals.var_tmf1_dn2)) } } else { (assign85650_e131368 * (assign85650_e131367 * (locals.var_tmf1_dn2 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign85650_e131367) as f64).is_finite() && ((assign85650_e131367) as f64).fract() == 0.0 { if assign85650_e131367 == 0.0 { 0.0 } else { (assign85650_e131367 * ((locals.var_tmf1).powf(assign85650_e131367 - 1.0) * locals.var_tmf1_dn4)) } } else { (assign85650_e131368 * (assign85650_e131367 * (locals.var_tmf1_dn4 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign85650_e131367) as f64).is_finite() && ((assign85650_e131367) as f64).fract() == 0.0 { if assign85650_e131367 == 0.0 { 0.0 } else { (assign85650_e131367 * ((locals.var_tmf1).powf(assign85650_e131367 - 1.0) * locals.var_tmf1_dn5)) } } else { (assign85650_e131368 * (assign85650_e131367 * (locals.var_tmf1_dn5 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign85650_e131367) as f64).is_finite() && ((assign85650_e131367) as f64).fract() == 0.0 { if assign85650_e131367 == 0.0 { 0.0 } else { (assign85650_e131367 * ((locals.var_tmf1).powf(assign85650_e131367 - 1.0) * locals.var_tmf1_dn6)) } } else { (assign85650_e131368 * (assign85650_e131367 * (locals.var_tmf1_dn6 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign85650_e131367) as f64).is_finite() && ((assign85650_e131367) as f64).fract() == 0.0 { if assign85650_e131367 == 0.0 { 0.0 } else { (assign85650_e131367 * ((locals.var_tmf1).powf(assign85650_e131367 - 1.0) * locals.var_tmf1_dn7)) } } else { (assign85650_e131368 * (assign85650_e131367 * (locals.var_tmf1_dn7 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign85650_e131367) as f64).is_finite() && ((assign85650_e131367) as f64).fract() == 0.0 { if assign85650_e131367 == 0.0 { 0.0 } else { (assign85650_e131367 * ((locals.var_tmf1).powf(assign85650_e131367 - 1.0) * locals.var_tmf1_dn8)) } } else { (assign85650_e131368 * (assign85650_e131367 * (locals.var_tmf1_dn8 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign85650_e131367) as f64).is_finite() && ((assign85650_e131367) as f64).fract() == 0.0 { if assign85650_e131367 == 0.0 { 0.0 } else { (assign85650_e131367 * ((locals.var_tmf1).powf(assign85650_e131367 - 1.0) * locals.var_tmf1_dn9)) } } else { (assign85650_e131368 * (assign85650_e131367 * (locals.var_tmf1_dn9 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign85650_e131367) as f64).is_finite() && ((assign85650_e131367) as f64).fract() == 0.0 { if assign85650_e131367 == 0.0 { 0.0 } else { (assign85650_e131367 * ((locals.var_tmf1).powf(assign85650_e131367 - 1.0) * locals.var_tmf1_dn10)) } } else { (assign85650_e131368 * (assign85650_e131367 * (locals.var_tmf1_dn10 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign85650_e131367) as f64).is_finite() && ((assign85650_e131367) as f64).fract() == 0.0 { if assign85650_e131367 == 0.0 { 0.0 } else { (assign85650_e131367 * ((locals.var_tmf1).powf(assign85650_e131367 - 1.0) * locals.var_tmf1_dn13)) } } else { (assign85650_e131368 * (assign85650_e131367 * (locals.var_tmf1_dn13 / locals.var_tmf1))) },)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign85650_e131370;
        locals.var_tmf2_dn0 = assign85650_e131370_d_n0;
        locals.var_tmf2_dn2 = assign85650_e131370_d_n2;
        locals.var_tmf2_dn4 = assign85650_e131370_d_n4;
        locals.var_tmf2_dn5 = assign85650_e131370_d_n5;
        locals.var_tmf2_dn6 = assign85650_e131370_d_n6;
        locals.var_tmf2_dn7 = assign85650_e131370_d_n7;
        locals.var_tmf2_dn8 = assign85650_e131370_d_n8;
        locals.var_tmf2_dn9 = assign85650_e131370_d_n9;
        locals.var_tmf2_dn10 = assign85650_e131370_d_n10;
        locals.var_tmf2_dn13 = assign85650_e131370_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign85660_e131392, assign85660_e131392_d_n0, assign85660_e131392_d_n2, assign85660_e131392_d_n4, assign85660_e131392_d_n5, assign85660_e131392_d_n6, assign85660_e131392_d_n7, assign85660_e131392_d_n8, assign85660_e131392_d_n9, assign85660_e131392_d_n10, assign85660_e131392_d_n13,) = {
    if ((((((locals.var_guard1992 != 0.0) && (!((locals.var_guard1990 != 0.0) || (locals.var_guard1991 != 0.0)))) && (locals.var_guard1996 != 0.0)) && (locals.var_guard1997 != 0.0)) && (locals.var_guard1998 != 0.0)) && (locals.var_guard1999 == 0.0)) {
        let assign85660_e131388: f64 = (locals.var_tmf3 * locals.var_tmf4);
        let assign85660_e131390: f64 = (assign85660_e131388 / locals.var_tmf2);
        (assign85660_e131390, (((((locals.var_tmf3_dn0 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn0)) * locals.var_tmf2) - (assign85660_e131388 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn2 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn2)) * locals.var_tmf2) - (assign85660_e131388 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn4 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn4)) * locals.var_tmf2) - (assign85660_e131388 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn5 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn5)) * locals.var_tmf2) - (assign85660_e131388 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn6 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn6)) * locals.var_tmf2) - (assign85660_e131388 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn7 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn7)) * locals.var_tmf2) - (assign85660_e131388 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn8 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn8)) * locals.var_tmf2) - (assign85660_e131388 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn9 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn9)) * locals.var_tmf2) - (assign85660_e131388 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn10 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn10)) * locals.var_tmf2) - (assign85660_e131388 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn13 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn13)) * locals.var_tmf2) - (assign85660_e131388 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2)),)
    } else {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn13,)
    }
};
        locals.var_vxbgmt = assign85660_e131392;
        locals.var_vxbgmt_dn0 = assign85660_e131392_d_n0;
        locals.var_vxbgmt_dn2 = assign85660_e131392_d_n2;
        locals.var_vxbgmt_dn4 = assign85660_e131392_d_n4;
        locals.var_vxbgmt_dn5 = assign85660_e131392_d_n5;
        locals.var_vxbgmt_dn6 = assign85660_e131392_d_n6;
        locals.var_vxbgmt_dn7 = assign85660_e131392_d_n7;
        locals.var_vxbgmt_dn8 = assign85660_e131392_d_n8;
        locals.var_vxbgmt_dn9 = assign85660_e131392_d_n9;
        locals.var_vxbgmt_dn10 = assign85660_e131392_d_n10;
        locals.var_vxbgmt_dn13 = assign85660_e131392_d_n13;
        locals.var_vxbgmt_rv = 0.0;

        let (assign85670_e131420, assign85670_e131420_d_n0, assign85670_e131420_d_n2, assign85670_e131420_d_n4, assign85670_e131420_d_n5, assign85670_e131420_d_n6, assign85670_e131420_d_n7, assign85670_e131420_d_n8, assign85670_e131420_d_n9, assign85670_e131420_d_n10, assign85670_e131420_d_n13,) = {
    if (((((locals.var_guard1992 != 0.0) && (!((locals.var_guard1990 != 0.0) || (locals.var_guard1991 != 0.0)))) && (locals.var_guard1996 != 0.0)) && (locals.var_guard1997 != 0.0)) && (locals.var_guard1998 != 0.0)) {
        let assign85670_e131407: f64 = (locals.var_vxbgmt + p.p137);
        let assign85670_e131410: f64 = (locals.var_vxbgmt + p.p137);
        let assign85670_e131411: f64 = (assign85670_e131407 * assign85670_e131410);
        let assign85670_e131414: f64 = (4.0 * 0.1);
        let assign85670_e131416: f64 = (assign85670_e131414 * 0.1);
        let assign85670_e131417: f64 = (assign85670_e131411 + assign85670_e131416);
        let assign85670_e131418: f64 = (assign85670_e131417).sqrt();
        (assign85670_e131418, (((locals.var_vxbgmt_dn0 * assign85670_e131410) + (assign85670_e131407 * locals.var_vxbgmt_dn0)) / (2.0 * assign85670_e131418)), (((locals.var_vxbgmt_dn2 * assign85670_e131410) + (assign85670_e131407 * locals.var_vxbgmt_dn2)) / (2.0 * assign85670_e131418)), (((locals.var_vxbgmt_dn4 * assign85670_e131410) + (assign85670_e131407 * locals.var_vxbgmt_dn4)) / (2.0 * assign85670_e131418)), (((locals.var_vxbgmt_dn5 * assign85670_e131410) + (assign85670_e131407 * locals.var_vxbgmt_dn5)) / (2.0 * assign85670_e131418)), (((locals.var_vxbgmt_dn6 * assign85670_e131410) + (assign85670_e131407 * locals.var_vxbgmt_dn6)) / (2.0 * assign85670_e131418)), (((locals.var_vxbgmt_dn7 * assign85670_e131410) + (assign85670_e131407 * locals.var_vxbgmt_dn7)) / (2.0 * assign85670_e131418)), (((locals.var_vxbgmt_dn8 * assign85670_e131410) + (assign85670_e131407 * locals.var_vxbgmt_dn8)) / (2.0 * assign85670_e131418)), (((locals.var_vxbgmt_dn9 * assign85670_e131410) + (assign85670_e131407 * locals.var_vxbgmt_dn9)) / (2.0 * assign85670_e131418)), (((locals.var_vxbgmt_dn10 * assign85670_e131410) + (assign85670_e131407 * locals.var_vxbgmt_dn10)) / (2.0 * assign85670_e131418)), (((locals.var_vxbgmt_dn13 * assign85670_e131410) + (assign85670_e131407 * locals.var_vxbgmt_dn13)) / (2.0 * assign85670_e131418)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign85670_e131420;
        locals.var_tmf2_dn0 = assign85670_e131420_d_n0;
        locals.var_tmf2_dn2 = assign85670_e131420_d_n2;
        locals.var_tmf2_dn4 = assign85670_e131420_d_n4;
        locals.var_tmf2_dn5 = assign85670_e131420_d_n5;
        locals.var_tmf2_dn6 = assign85670_e131420_d_n6;
        locals.var_tmf2_dn7 = assign85670_e131420_d_n7;
        locals.var_tmf2_dn8 = assign85670_e131420_d_n8;
        locals.var_tmf2_dn9 = assign85670_e131420_d_n9;
        locals.var_tmf2_dn10 = assign85670_e131420_d_n10;
        locals.var_tmf2_dn13 = assign85670_e131420_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign85680_e131443, assign85680_e131443_d_n0, assign85680_e131443_d_n2, assign85680_e131443_d_n4, assign85680_e131443_d_n5, assign85680_e131443_d_n6, assign85680_e131443_d_n7, assign85680_e131443_d_n8, assign85680_e131443_d_n9, assign85680_e131443_d_n10, assign85680_e131443_d_n13,) = {
    if (((((locals.var_guard1992 != 0.0) && (!((locals.var_guard1990 != 0.0) || (locals.var_guard1991 != 0.0)))) && (locals.var_guard1996 != 0.0)) && (locals.var_guard1997 != 0.0)) && (locals.var_guard1998 != 0.0)) {
        let assign85680_e131437: f64 = (locals.var_vxbgmt + p.p137);
        let assign85680_e131439: f64 = (assign85680_e131437 / locals.var_tmf2);
        let assign85680_e131440: f64 = (1.0 + assign85680_e131439);
        let assign85680_e131441: f64 = (0.5 * assign85680_e131440);
        (assign85680_e131441, (0.5 * (((locals.var_vxbgmt_dn0 * locals.var_tmf2) - (assign85680_e131437 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn2 * locals.var_tmf2) - (assign85680_e131437 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn4 * locals.var_tmf2) - (assign85680_e131437 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn5 * locals.var_tmf2) - (assign85680_e131437 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn6 * locals.var_tmf2) - (assign85680_e131437 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn7 * locals.var_tmf2) - (assign85680_e131437 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn8 * locals.var_tmf2) - (assign85680_e131437 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn9 * locals.var_tmf2) - (assign85680_e131437 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn10 * locals.var_tmf2) - (assign85680_e131437 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn13 * locals.var_tmf2) - (assign85680_e131437 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign85680_e131443;
        locals.var_t9_dn0 = assign85680_e131443_d_n0;
        locals.var_t9_dn2 = assign85680_e131443_d_n2;
        locals.var_t9_dn4 = assign85680_e131443_d_n4;
        locals.var_t9_dn5 = assign85680_e131443_d_n5;
        locals.var_t9_dn6 = assign85680_e131443_d_n6;
        locals.var_t9_dn7 = assign85680_e131443_d_n7;
        locals.var_t9_dn8 = assign85680_e131443_d_n8;
        locals.var_t9_dn9 = assign85680_e131443_d_n9;
        locals.var_t9_dn10 = assign85680_e131443_d_n10;
        locals.var_t9_dn13 = assign85680_e131443_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign85690_e131464, assign85690_e131464_d_n0, assign85690_e131464_d_n2, assign85690_e131464_d_n4, assign85690_e131464_d_n5, assign85690_e131464_d_n6, assign85690_e131464_d_n7, assign85690_e131464_d_n8, assign85690_e131464_d_n9, assign85690_e131464_d_n10, assign85690_e131464_d_n13,) = {
    if (((((locals.var_guard1992 != 0.0) && (!((locals.var_guard1990 != 0.0) || (locals.var_guard1991 != 0.0)))) && (locals.var_guard1996 != 0.0)) && (locals.var_guard1997 != 0.0)) && (locals.var_guard1998 != 0.0)) {
        let assign85690_e131459: f64 = (locals.var_vxbgmt + p.p137);
        let assign85690_e131461: f64 = (assign85690_e131459 + locals.var_tmf2);
        let assign85690_e131462: f64 = (0.5 * assign85690_e131461);
        (assign85690_e131462, (0.5 * (locals.var_vxbgmt_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_vxbgmt_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_vxbgmt_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_vxbgmt_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_vxbgmt_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_vxbgmt_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_vxbgmt_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_vxbgmt_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_vxbgmt_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_vxbgmt_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign85690_e131464;
        locals.var_t2_dn0 = assign85690_e131464_d_n0;
        locals.var_t2_dn2 = assign85690_e131464_d_n2;
        locals.var_t2_dn4 = assign85690_e131464_d_n4;
        locals.var_t2_dn5 = assign85690_e131464_d_n5;
        locals.var_t2_dn6 = assign85690_e131464_d_n6;
        locals.var_t2_dn7 = assign85690_e131464_d_n7;
        locals.var_t2_dn8 = assign85690_e131464_d_n8;
        locals.var_t2_dn9 = assign85690_e131464_d_n9;
        locals.var_t2_dn10 = assign85690_e131464_d_n10;
        locals.var_t2_dn13 = assign85690_e131464_d_n13;
        locals.var_t2_rv = 0.0;

        let assign85700_e131467: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2000 = assign85700_e131467;
        locals.var_guard2000_rv = 0.0;

        let (assign85710_e131484, assign85710_e131484_d_n0, assign85710_e131484_d_n2, assign85710_e131484_d_n4, assign85710_e131484_d_n5, assign85710_e131484_d_n6, assign85710_e131484_d_n7, assign85710_e131484_d_n8, assign85710_e131484_d_n9, assign85710_e131484_d_n10, assign85710_e131484_d_n13,) = {
    if ((((((locals.var_guard1992 != 0.0) && (!((locals.var_guard1990 != 0.0) || (locals.var_guard1991 != 0.0)))) && (locals.var_guard1996 != 0.0)) && (locals.var_guard1997 != 0.0)) && (locals.var_guard1998 != 0.0)) && (locals.var_guard2000 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign85710_e131484;
        locals.var_t2_dn0 = assign85710_e131484_d_n0;
        locals.var_t2_dn2 = assign85710_e131484_d_n2;
        locals.var_t2_dn4 = assign85710_e131484_d_n4;
        locals.var_t2_dn5 = assign85710_e131484_d_n5;
        locals.var_t2_dn6 = assign85710_e131484_d_n6;
        locals.var_t2_dn7 = assign85710_e131484_d_n7;
        locals.var_t2_dn8 = assign85710_e131484_d_n8;
        locals.var_t2_dn9 = assign85710_e131484_d_n9;
        locals.var_t2_dn10 = assign85710_e131484_d_n10;
        locals.var_t2_dn13 = assign85710_e131484_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign85720_e131501, assign85720_e131501_d_n0, assign85720_e131501_d_n2, assign85720_e131501_d_n4, assign85720_e131501_d_n5, assign85720_e131501_d_n6, assign85720_e131501_d_n7, assign85720_e131501_d_n8, assign85720_e131501_d_n9, assign85720_e131501_d_n10, assign85720_e131501_d_n13,) = {
    if ((((((locals.var_guard1992 != 0.0) && (!((locals.var_guard1990 != 0.0) || (locals.var_guard1991 != 0.0)))) && (locals.var_guard1996 != 0.0)) && (locals.var_guard1997 != 0.0)) && (locals.var_guard1998 != 0.0)) && (locals.var_guard2000 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign85720_e131501;
        locals.var_t9_dn0 = assign85720_e131501_d_n0;
        locals.var_t9_dn2 = assign85720_e131501_d_n2;
        locals.var_t9_dn4 = assign85720_e131501_d_n4;
        locals.var_t9_dn5 = assign85720_e131501_d_n5;
        locals.var_t9_dn6 = assign85720_e131501_d_n6;
        locals.var_t9_dn7 = assign85720_e131501_d_n7;
        locals.var_t9_dn8 = assign85720_e131501_d_n8;
        locals.var_t9_dn9 = assign85720_e131501_d_n9;
        locals.var_t9_dn10 = assign85720_e131501_d_n10;
        locals.var_t9_dn13 = assign85720_e131501_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign85730_e131521, assign85730_e131521_d_n0, assign85730_e131521_d_n2, assign85730_e131521_d_n4, assign85730_e131521_d_n5, assign85730_e131521_d_n6, assign85730_e131521_d_n7, assign85730_e131521_d_n8, assign85730_e131521_d_n9, assign85730_e131521_d_n10, assign85730_e131521_d_n13,) = {
    if (((((locals.var_guard1992 != 0.0) && (!((locals.var_guard1990 != 0.0) || (locals.var_guard1991 != 0.0)))) && (locals.var_guard1996 != 0.0)) && (locals.var_guard1997 != 0.0)) && (locals.var_guard1998 != 0.0)) {
        let assign85730_e131516: f64 = (locals.var_kjunc * locals.var_t2);
        let assign85730_e131517: f64 = (assign85730_e131516).sqrt();
        let assign85730_e131519: f64 = (assign85730_e131517 * p.p432);
        (assign85730_e131519, ((((locals.var_kjunc_dn0 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn0)) / (2.0 * assign85730_e131517)) * p.p432), ((((locals.var_kjunc_dn2 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn2)) / (2.0 * assign85730_e131517)) * p.p432), ((((locals.var_kjunc_dn4 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn4)) / (2.0 * assign85730_e131517)) * p.p432), ((((locals.var_kjunc_dn5 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn5)) / (2.0 * assign85730_e131517)) * p.p432), ((((locals.var_kjunc_dn6 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn6)) / (2.0 * assign85730_e131517)) * p.p432), ((((locals.var_kjunc_dn7 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn7)) / (2.0 * assign85730_e131517)) * p.p432), ((((locals.var_kjunc_dn8 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn8)) / (2.0 * assign85730_e131517)) * p.p432), ((((locals.var_kjunc_dn9 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn9)) / (2.0 * assign85730_e131517)) * p.p432), ((((locals.var_kjunc_dn10 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn10)) / (2.0 * assign85730_e131517)) * p.p432), ((((locals.var_kjunc_dn13 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn13)) / (2.0 * assign85730_e131517)) * p.p432),)
    } else {
        (locals.var_wjunc0, locals.var_wjunc0_dn0, locals.var_wjunc0_dn2, locals.var_wjunc0_dn4, locals.var_wjunc0_dn5, locals.var_wjunc0_dn6, locals.var_wjunc0_dn7, locals.var_wjunc0_dn8, locals.var_wjunc0_dn9, locals.var_wjunc0_dn10, locals.var_wjunc0_dn13,)
    }
};
        locals.var_wjunc0 = assign85730_e131521;
        locals.var_wjunc0_dn0 = assign85730_e131521_d_n0;
        locals.var_wjunc0_dn2 = assign85730_e131521_d_n2;
        locals.var_wjunc0_dn4 = assign85730_e131521_d_n4;
        locals.var_wjunc0_dn5 = assign85730_e131521_d_n5;
        locals.var_wjunc0_dn6 = assign85730_e131521_d_n6;
        locals.var_wjunc0_dn7 = assign85730_e131521_d_n7;
        locals.var_wjunc0_dn8 = assign85730_e131521_d_n8;
        locals.var_wjunc0_dn9 = assign85730_e131521_d_n9;
        locals.var_wjunc0_dn10 = assign85730_e131521_d_n10;
        locals.var_wjunc0_dn13 = assign85730_e131521_d_n13;
        locals.var_wjunc0_rv = 0.0;

        let (assign85740_e131538, assign85740_e131538_d_n0, assign85740_e131538_d_n2, assign85740_e131538_d_n4, assign85740_e131538_d_n5, assign85740_e131538_d_n6, assign85740_e131538_d_n7, assign85740_e131538_d_n8, assign85740_e131538_d_n9, assign85740_e131538_d_n10, assign85740_e131538_d_n13,) = {
    if (((((locals.var_guard1992 != 0.0) && (!((locals.var_guard1990 != 0.0) || (locals.var_guard1991 != 0.0)))) && (locals.var_guard1996 != 0.0)) && (locals.var_guard1997 != 0.0)) && (locals.var_guard1998 != 0.0)) {
        let assign85740_e131536: f64 = (locals.var_lover_func - locals.var_wjunc0);
        (assign85740_e131536, (locals.var_lover_func_dn0 - locals.var_wjunc0_dn0), (locals.var_lover_func_dn2 - locals.var_wjunc0_dn2), (locals.var_lover_func_dn4 - locals.var_wjunc0_dn4), (locals.var_lover_func_dn5 - locals.var_wjunc0_dn5), (locals.var_lover_func_dn6 - locals.var_wjunc0_dn6), (locals.var_lover_func_dn7 - locals.var_wjunc0_dn7), (locals.var_lover_func_dn8 - locals.var_wjunc0_dn8), (locals.var_lover_func_dn9 - locals.var_wjunc0_dn9), (locals.var_lover_func_dn10 - locals.var_wjunc0_dn10), (locals.var_lover_func_dn13 - locals.var_wjunc0_dn13),)
    } else {
        (locals.var_lover_func, locals.var_lover_func_dn0, locals.var_lover_func_dn2, locals.var_lover_func_dn4, locals.var_lover_func_dn5, locals.var_lover_func_dn6, locals.var_lover_func_dn7, locals.var_lover_func_dn8, locals.var_lover_func_dn9, locals.var_lover_func_dn10, locals.var_lover_func_dn13,)
    }
};
        locals.var_lover_func = assign85740_e131538;
        locals.var_lover_func_dn0 = assign85740_e131538_d_n0;
        locals.var_lover_func_dn2 = assign85740_e131538_d_n2;
        locals.var_lover_func_dn4 = assign85740_e131538_d_n4;
        locals.var_lover_func_dn5 = assign85740_e131538_d_n5;
        locals.var_lover_func_dn6 = assign85740_e131538_d_n6;
        locals.var_lover_func_dn7 = assign85740_e131538_d_n7;
        locals.var_lover_func_dn8 = assign85740_e131538_d_n8;
        locals.var_lover_func_dn9 = assign85740_e131538_d_n9;
        locals.var_lover_func_dn10 = assign85740_e131538_d_n10;
        locals.var_lover_func_dn13 = assign85740_e131538_d_n13;
        locals.var_lover_func_rv = 0.0;

        let assign85750_e131557: f64 = if (((((p.p35 == 1.0) && (p.p63 > 0.0)) && (locals.var_uc_nover > 0.0)) && (locals.var_uc_cvdsover != 0.0)) && (p.p55 != 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard2001 = assign85750_e131557;
        locals.var_guard2001_rv = 0.0;

        let (assign85760_e131570,) = {
    if (((locals.var_guard1993 != 0.0) && (!(((locals.var_guard1990 != 0.0) || (locals.var_guard1991 != 0.0)) || (locals.var_guard1992 != 0.0)))) && (locals.var_guard2001 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_calcqover,)
    }
};
        locals.var_flg_calcqover = assign85760_e131570;
        locals.var_flg_calcqover_rv = 0.0;

        let (assign85770_e131585, assign85770_e131585_d_n2, assign85770_e131585_d_n6, assign85770_e131585_d_n7, assign85770_e131585_d_n8,) = {
    if (((locals.var_guard1993 != 0.0) && (!(((locals.var_guard1990 != 0.0) || (locals.var_guard1991 != 0.0)) || (locals.var_guard1992 != 0.0)))) && (locals.var_guard2001 != 0.0)) {
        let assign85770_e131583: f64 = (locals.var_vgsei - locals.var_vbsei);
        (assign85770_e131583, (locals.var_vgsei_dn2 - locals.var_vbsei_dn2), locals.var_vgsei_dn6, 0.0, (-locals.var_vbsei_dn8),)
    } else {
        (locals.var_vgbgmt, locals.var_vgbgmt_dn2, locals.var_vgbgmt_dn6, locals.var_vgbgmt_dn7, locals.var_vgbgmt_dn8,)
    }
};
        locals.var_vgbgmt = assign85770_e131585;
        locals.var_vgbgmt_dn2 = assign85770_e131585_d_n2;
        locals.var_vgbgmt_dn6 = assign85770_e131585_d_n6;
        locals.var_vgbgmt_dn7 = assign85770_e131585_d_n7;
        locals.var_vgbgmt_dn8 = assign85770_e131585_d_n8;
        locals.var_vgbgmt_rv = 0.0;

        let (assign85780_e131600, assign85780_e131600_d_n0, assign85780_e131600_d_n2, assign85780_e131600_d_n4, assign85780_e131600_d_n5, assign85780_e131600_d_n6, assign85780_e131600_d_n7, assign85780_e131600_d_n8, assign85780_e131600_d_n9, assign85780_e131600_d_n10, assign85780_e131600_d_n13,) = {
    if (((locals.var_guard1993 != 0.0) && (!(((locals.var_guard1990 != 0.0) || (locals.var_guard1991 != 0.0)) || (locals.var_guard1992 != 0.0)))) && (locals.var_guard2001 != 0.0)) {
        let assign85780_e131598: f64 = (locals.var_vdsei - locals.var_vbsei);
        (assign85780_e131598, locals.var_vdsei_dn0, (locals.var_vdsei_dn2 - locals.var_vbsei_dn2), 0.0, 0.0, 0.0, 0.0, (-locals.var_vbsei_dn8), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn13,)
    }
};
        locals.var_vxbgmt = assign85780_e131600;
        locals.var_vxbgmt_dn0 = assign85780_e131600_d_n0;
        locals.var_vxbgmt_dn2 = assign85780_e131600_d_n2;
        locals.var_vxbgmt_dn4 = assign85780_e131600_d_n4;
        locals.var_vxbgmt_dn5 = assign85780_e131600_d_n5;
        locals.var_vxbgmt_dn6 = assign85780_e131600_d_n6;
        locals.var_vxbgmt_dn7 = assign85780_e131600_d_n7;
        locals.var_vxbgmt_dn8 = assign85780_e131600_d_n8;
        locals.var_vxbgmt_dn9 = assign85780_e131600_d_n9;
        locals.var_vxbgmt_dn10 = assign85780_e131600_d_n10;
        locals.var_vxbgmt_dn13 = assign85780_e131600_d_n13;
        locals.var_vxbgmt_rv = 0.0;

        let (assign85790_e131604, assign85790_e131604_d_n0, assign85790_e131604_d_n2, assign85790_e131604_d_n4, assign85790_e131604_d_n5, assign85790_e131604_d_n6, assign85790_e131604_d_n7, assign85790_e131604_d_n8, assign85790_e131604_d_n9, assign85790_e131604_d_n10, assign85790_e131604_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.4, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbs_bnd_over__blk2009, locals.var_vbs_bnd_over__blk2009_dn0, locals.var_vbs_bnd_over__blk2009_dn2, locals.var_vbs_bnd_over__blk2009_dn4, locals.var_vbs_bnd_over__blk2009_dn5, locals.var_vbs_bnd_over__blk2009_dn6, locals.var_vbs_bnd_over__blk2009_dn7, locals.var_vbs_bnd_over__blk2009_dn8, locals.var_vbs_bnd_over__blk2009_dn9, locals.var_vbs_bnd_over__blk2009_dn10, locals.var_vbs_bnd_over__blk2009_dn13,)
    }
};
        locals.var_vbs_bnd_over__blk2009 = assign85790_e131604;
        locals.var_vbs_bnd_over__blk2009_dn0 = assign85790_e131604_d_n0;
        locals.var_vbs_bnd_over__blk2009_dn2 = assign85790_e131604_d_n2;
        locals.var_vbs_bnd_over__blk2009_dn4 = assign85790_e131604_d_n4;
        locals.var_vbs_bnd_over__blk2009_dn5 = assign85790_e131604_d_n5;
        locals.var_vbs_bnd_over__blk2009_dn6 = assign85790_e131604_d_n6;
        locals.var_vbs_bnd_over__blk2009_dn7 = assign85790_e131604_d_n7;
        locals.var_vbs_bnd_over__blk2009_dn8 = assign85790_e131604_d_n8;
        locals.var_vbs_bnd_over__blk2009_dn9 = assign85790_e131604_d_n9;
        locals.var_vbs_bnd_over__blk2009_dn10 = assign85790_e131604_d_n10;
        locals.var_vbs_bnd_over__blk2009_dn13 = assign85790_e131604_d_n13;
        locals.var_vbs_bnd_over__blk2009_rv = 0.0;

        let (assign85810_e131612,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0,)
    } else {
        (locals.var_flg_fd_mode__blk2010,)
    }
};
        locals.var_flg_fd_mode__blk2010 = assign85810_e131612;
        locals.var_flg_fd_mode__blk2010_rv = 0.0;

        let (assign85820_e131616, assign85820_e131616_d_n0, assign85820_e131616_d_n2, assign85820_e131616_d_n4, assign85820_e131616_d_n5, assign85820_e131616_d_n6, assign85820_e131616_d_n7, assign85820_e131616_d_n8, assign85820_e131616_d_n9, assign85820_e131616_d_n10, assign85820_e131616_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn13,)
    }
};
        locals.var_fb = assign85820_e131616;
        locals.var_fb_dn0 = assign85820_e131616_d_n0;
        locals.var_fb_dn2 = assign85820_e131616_d_n2;
        locals.var_fb_dn4 = assign85820_e131616_d_n4;
        locals.var_fb_dn5 = assign85820_e131616_d_n5;
        locals.var_fb_dn6 = assign85820_e131616_d_n6;
        locals.var_fb_dn7 = assign85820_e131616_d_n7;
        locals.var_fb_dn8 = assign85820_e131616_d_n8;
        locals.var_fb_dn9 = assign85820_e131616_d_n9;
        locals.var_fb_dn10 = assign85820_e131616_d_n10;
        locals.var_fb_dn13 = assign85820_e131616_d_n13;
        locals.var_fb_rv = 0.0;

        let (assign85830_e131620, assign85830_e131620_d_n0, assign85830_e131620_d_n2, assign85830_e131620_d_n4, assign85830_e131620_d_n5, assign85830_e131620_d_n6, assign85830_e131620_d_n7, assign85830_e131620_d_n8, assign85830_e131620_d_n9, assign85830_e131620_d_n10, assign85830_e131620_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn13,)
    }
};
        locals.var_fs01 = assign85830_e131620;
        locals.var_fs01_dn0 = assign85830_e131620_d_n0;
        locals.var_fs01_dn2 = assign85830_e131620_d_n2;
        locals.var_fs01_dn4 = assign85830_e131620_d_n4;
        locals.var_fs01_dn5 = assign85830_e131620_d_n5;
        locals.var_fs01_dn6 = assign85830_e131620_d_n6;
        locals.var_fs01_dn7 = assign85830_e131620_d_n7;
        locals.var_fs01_dn8 = assign85830_e131620_d_n8;
        locals.var_fs01_dn9 = assign85830_e131620_d_n9;
        locals.var_fs01_dn10 = assign85830_e131620_d_n10;
        locals.var_fs01_dn13 = assign85830_e131620_d_n13;
        locals.var_fs01_rv = 0.0;

        let (assign85840_e131624, assign85840_e131624_d_n0, assign85840_e131624_d_n2, assign85840_e131624_d_n4, assign85840_e131624_d_n5, assign85840_e131624_d_n6, assign85840_e131624_d_n7, assign85840_e131624_d_n8, assign85840_e131624_d_n9, assign85840_e131624_d_n10, assign85840_e131624_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn13,)
    }
};
        locals.var_fs02 = assign85840_e131624;
        locals.var_fs02_dn0 = assign85840_e131624_d_n0;
        locals.var_fs02_dn2 = assign85840_e131624_d_n2;
        locals.var_fs02_dn4 = assign85840_e131624_d_n4;
        locals.var_fs02_dn5 = assign85840_e131624_d_n5;
        locals.var_fs02_dn6 = assign85840_e131624_d_n6;
        locals.var_fs02_dn7 = assign85840_e131624_d_n7;
        locals.var_fs02_dn8 = assign85840_e131624_d_n8;
        locals.var_fs02_dn9 = assign85840_e131624_d_n9;
        locals.var_fs02_dn10 = assign85840_e131624_d_n10;
        locals.var_fs02_dn13 = assign85840_e131624_d_n13;
        locals.var_fs02_rv = 0.0;

        let (assign85850_e131628, assign85850_e131628_d_n0, assign85850_e131628_d_n2, assign85850_e131628_d_n4, assign85850_e131628_d_n5, assign85850_e131628_d_n6, assign85850_e131628_d_n7, assign85850_e131628_d_n8, assign85850_e131628_d_n9, assign85850_e131628_d_n10, assign85850_e131628_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs0, locals.var_fs0_dn0, locals.var_fs0_dn2, locals.var_fs0_dn4, locals.var_fs0_dn5, locals.var_fs0_dn6, locals.var_fs0_dn7, locals.var_fs0_dn8, locals.var_fs0_dn9, locals.var_fs0_dn10, locals.var_fs0_dn13,)
    }
};
        locals.var_fs0 = assign85850_e131628;
        locals.var_fs0_dn0 = assign85850_e131628_d_n0;
        locals.var_fs0_dn2 = assign85850_e131628_d_n2;
        locals.var_fs0_dn4 = assign85850_e131628_d_n4;
        locals.var_fs0_dn5 = assign85850_e131628_d_n5;
        locals.var_fs0_dn6 = assign85850_e131628_d_n6;
        locals.var_fs0_dn7 = assign85850_e131628_d_n7;
        locals.var_fs0_dn8 = assign85850_e131628_d_n8;
        locals.var_fs0_dn9 = assign85850_e131628_d_n9;
        locals.var_fs0_dn10 = assign85850_e131628_d_n10;
        locals.var_fs0_dn13 = assign85850_e131628_d_n13;
        locals.var_fs0_rv = 0.0;

        let (assign85860_e131632, assign85860_e131632_d_n0, assign85860_e131632_d_n2, assign85860_e131632_d_n4, assign85860_e131632_d_n5, assign85860_e131632_d_n6, assign85860_e131632_d_n7, assign85860_e131632_d_n8, assign85860_e131632_d_n9, assign85860_e131632_d_n10, assign85860_e131632_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn4, locals.var_dps0_dn5, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn8, locals.var_dps0_dn9, locals.var_dps0_dn10, locals.var_dps0_dn13,)
    }
};
        locals.var_dps0 = assign85860_e131632;
        locals.var_dps0_dn0 = assign85860_e131632_d_n0;
        locals.var_dps0_dn2 = assign85860_e131632_d_n2;
        locals.var_dps0_dn4 = assign85860_e131632_d_n4;
        locals.var_dps0_dn5 = assign85860_e131632_d_n5;
        locals.var_dps0_dn6 = assign85860_e131632_d_n6;
        locals.var_dps0_dn7 = assign85860_e131632_d_n7;
        locals.var_dps0_dn8 = assign85860_e131632_d_n8;
        locals.var_dps0_dn9 = assign85860_e131632_d_n9;
        locals.var_dps0_dn10 = assign85860_e131632_d_n10;
        locals.var_dps0_dn13 = assign85860_e131632_d_n13;
        locals.var_dps0_rv = 0.0;

        let (assign85870_e131636, assign85870_e131636_d_n0, assign85870_e131636_d_n2, assign85870_e131636_d_n4, assign85870_e131636_d_n5, assign85870_e131636_d_n6, assign85870_e131636_d_n7, assign85870_e131636_d_n8, assign85870_e131636_d_n9, assign85870_e131636_d_n10, assign85870_e131636_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs0_dps0, locals.var_fs0_dps0_dn0, locals.var_fs0_dps0_dn2, locals.var_fs0_dps0_dn4, locals.var_fs0_dps0_dn5, locals.var_fs0_dps0_dn6, locals.var_fs0_dps0_dn7, locals.var_fs0_dps0_dn8, locals.var_fs0_dps0_dn9, locals.var_fs0_dps0_dn10, locals.var_fs0_dps0_dn13,)
    }
};
        locals.var_fs0_dps0 = assign85870_e131636;
        locals.var_fs0_dps0_dn0 = assign85870_e131636_d_n0;
        locals.var_fs0_dps0_dn2 = assign85870_e131636_d_n2;
        locals.var_fs0_dps0_dn4 = assign85870_e131636_d_n4;
        locals.var_fs0_dps0_dn5 = assign85870_e131636_d_n5;
        locals.var_fs0_dps0_dn6 = assign85870_e131636_d_n6;
        locals.var_fs0_dps0_dn7 = assign85870_e131636_d_n7;
        locals.var_fs0_dps0_dn8 = assign85870_e131636_d_n8;
        locals.var_fs0_dps0_dn9 = assign85870_e131636_d_n9;
        locals.var_fs0_dps0_dn10 = assign85870_e131636_d_n10;
        locals.var_fs0_dps0_dn13 = assign85870_e131636_d_n13;
        locals.var_fs0_dps0_rv = 0.0;

        let (assign85880_e131640, assign85880_e131640_d_n0, assign85880_e131640_d_n2, assign85880_e131640_d_n4, assign85880_e131640_d_n5, assign85880_e131640_d_n6, assign85880_e131640_d_n7, assign85880_e131640_d_n8, assign85880_e131640_d_n9, assign85880_e131640_d_n10, assign85880_e131640_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn13,)
    }
};
        locals.var_fs02_dps0 = assign85880_e131640;
        locals.var_fs02_dps0_dn0 = assign85880_e131640_d_n0;
        locals.var_fs02_dps0_dn2 = assign85880_e131640_d_n2;
        locals.var_fs02_dps0_dn4 = assign85880_e131640_d_n4;
        locals.var_fs02_dps0_dn5 = assign85880_e131640_d_n5;
        locals.var_fs02_dps0_dn6 = assign85880_e131640_d_n6;
        locals.var_fs02_dps0_dn7 = assign85880_e131640_d_n7;
        locals.var_fs02_dps0_dn8 = assign85880_e131640_d_n8;
        locals.var_fs02_dps0_dn9 = assign85880_e131640_d_n9;
        locals.var_fs02_dps0_dn10 = assign85880_e131640_d_n10;
        locals.var_fs02_dps0_dn13 = assign85880_e131640_d_n13;
        locals.var_fs02_dps0_rv = 0.0;

        let (assign85890_e131644, assign85890_e131644_d_n0, assign85890_e131644_d_n2, assign85890_e131644_d_n4, assign85890_e131644_d_n5, assign85890_e131644_d_n6, assign85890_e131644_d_n7, assign85890_e131644_d_n8, assign85890_e131644_d_n9, assign85890_e131644_d_n10, assign85890_e131644_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn13,)
    }
};
        locals.var_fb_dpss = assign85890_e131644;
        locals.var_fb_dpss_dn0 = assign85890_e131644_d_n0;
        locals.var_fb_dpss_dn2 = assign85890_e131644_d_n2;
        locals.var_fb_dpss_dn4 = assign85890_e131644_d_n4;
        locals.var_fb_dpss_dn5 = assign85890_e131644_d_n5;
        locals.var_fb_dpss_dn6 = assign85890_e131644_d_n6;
        locals.var_fb_dpss_dn7 = assign85890_e131644_d_n7;
        locals.var_fb_dpss_dn8 = assign85890_e131644_d_n8;
        locals.var_fb_dpss_dn9 = assign85890_e131644_d_n9;
        locals.var_fb_dpss_dn10 = assign85890_e131644_d_n10;
        locals.var_fb_dpss_dn13 = assign85890_e131644_d_n13;
        locals.var_fb_dpss_rv = 0.0;

        let (assign85900_e131648, assign85900_e131648_d_n0, assign85900_e131648_d_n2, assign85900_e131648_d_n4, assign85900_e131648_d_n5, assign85900_e131648_d_n6, assign85900_e131648_d_n7, assign85900_e131648_d_n8, assign85900_e131648_d_n9, assign85900_e131648_d_n10, assign85900_e131648_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn13,)
    }
};
        locals.var_fs01_dps0 = assign85900_e131648;
        locals.var_fs01_dps0_dn0 = assign85900_e131648_d_n0;
        locals.var_fs01_dps0_dn2 = assign85900_e131648_d_n2;
        locals.var_fs01_dps0_dn4 = assign85900_e131648_d_n4;
        locals.var_fs01_dps0_dn5 = assign85900_e131648_d_n5;
        locals.var_fs01_dps0_dn6 = assign85900_e131648_d_n6;
        locals.var_fs01_dps0_dn7 = assign85900_e131648_d_n7;
        locals.var_fs01_dps0_dn8 = assign85900_e131648_d_n8;
        locals.var_fs01_dps0_dn9 = assign85900_e131648_d_n9;
        locals.var_fs01_dps0_dn10 = assign85900_e131648_d_n10;
        locals.var_fs01_dps0_dn13 = assign85900_e131648_d_n13;
        locals.var_fs01_dps0_rv = 0.0;

        let (assign85910_e131652, assign85910_e131652_d_n0, assign85910_e131652_d_n2, assign85910_e131652_d_n4, assign85910_e131652_d_n5, assign85910_e131652_d_n6, assign85910_e131652_d_n7, assign85910_e131652_d_n8, assign85910_e131652_d_n9, assign85910_e131652_d_n10, assign85910_e131652_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn13,)
    }
};
        locals.var_chi_1 = assign85910_e131652;
        locals.var_chi_1_dn0 = assign85910_e131652_d_n0;
        locals.var_chi_1_dn2 = assign85910_e131652_d_n2;
        locals.var_chi_1_dn4 = assign85910_e131652_d_n4;
        locals.var_chi_1_dn5 = assign85910_e131652_d_n5;
        locals.var_chi_1_dn6 = assign85910_e131652_d_n6;
        locals.var_chi_1_dn7 = assign85910_e131652_d_n7;
        locals.var_chi_1_dn8 = assign85910_e131652_d_n8;
        locals.var_chi_1_dn9 = assign85910_e131652_d_n9;
        locals.var_chi_1_dn10 = assign85910_e131652_d_n10;
        locals.var_chi_1_dn13 = assign85910_e131652_d_n13;
        locals.var_chi_1_rv = 0.0;

        let (assign85920_e131656, assign85920_e131656_d_n0, assign85920_e131656_d_n2, assign85920_e131656_d_n4, assign85920_e131656_d_n5, assign85920_e131656_d_n6, assign85920_e131656_d_n7, assign85920_e131656_d_n8, assign85920_e131656_d_n9, assign85920_e131656_d_n10, assign85920_e131656_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_chi_a, locals.var_chi_a_dn0, locals.var_chi_a_dn2, locals.var_chi_a_dn4, locals.var_chi_a_dn5, locals.var_chi_a_dn6, locals.var_chi_a_dn7, locals.var_chi_a_dn8, locals.var_chi_a_dn9, locals.var_chi_a_dn10, locals.var_chi_a_dn13,)
    }
};
        locals.var_chi_a = assign85920_e131656;
        locals.var_chi_a_dn0 = assign85920_e131656_d_n0;
        locals.var_chi_a_dn2 = assign85920_e131656_d_n2;
        locals.var_chi_a_dn4 = assign85920_e131656_d_n4;
        locals.var_chi_a_dn5 = assign85920_e131656_d_n5;
        locals.var_chi_a_dn6 = assign85920_e131656_d_n6;
        locals.var_chi_a_dn7 = assign85920_e131656_d_n7;
        locals.var_chi_a_dn8 = assign85920_e131656_d_n8;
        locals.var_chi_a_dn9 = assign85920_e131656_d_n9;
        locals.var_chi_a_dn10 = assign85920_e131656_d_n10;
        locals.var_chi_a_dn13 = assign85920_e131656_d_n13;
        locals.var_chi_a_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_318(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign85930_e131660, assign85930_e131660_d_n0, assign85930_e131660_d_n2, assign85930_e131660_d_n4, assign85930_e131660_d_n5, assign85930_e131660_d_n6, assign85930_e131660_d_n7, assign85930_e131660_d_n8, assign85930_e131660_d_n9, assign85930_e131660_d_n10, assign85930_e131660_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_chi_b, locals.var_chi_b_dn0, locals.var_chi_b_dn2, locals.var_chi_b_dn4, locals.var_chi_b_dn5, locals.var_chi_b_dn6, locals.var_chi_b_dn7, locals.var_chi_b_dn8, locals.var_chi_b_dn9, locals.var_chi_b_dn10, locals.var_chi_b_dn13,)
    }
};
        locals.var_chi_b = assign85930_e131660;
        locals.var_chi_b_dn0 = assign85930_e131660_d_n0;
        locals.var_chi_b_dn2 = assign85930_e131660_d_n2;
        locals.var_chi_b_dn4 = assign85930_e131660_d_n4;
        locals.var_chi_b_dn5 = assign85930_e131660_d_n5;
        locals.var_chi_b_dn6 = assign85930_e131660_d_n6;
        locals.var_chi_b_dn7 = assign85930_e131660_d_n7;
        locals.var_chi_b_dn8 = assign85930_e131660_d_n8;
        locals.var_chi_b_dn9 = assign85930_e131660_d_n9;
        locals.var_chi_b_dn10 = assign85930_e131660_d_n10;
        locals.var_chi_b_dn13 = assign85930_e131660_d_n13;
        locals.var_chi_b_rv = 0.0;

        let (assign85940_e131665,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign85940_e131663: f64 = (-1.0);
        (assign85940_e131663,)
    } else {
        (locals.var_flg_conv,)
    }
};
        locals.var_flg_conv = assign85940_e131665;
        locals.var_flg_conv_rv = 0.0;

        let (assign85950_e131669, assign85950_e131669_d_n0, assign85950_e131669_d_n2, assign85950_e131669_d_n4, assign85950_e131669_d_n5, assign85950_e131669_d_n6, assign85950_e131669_d_n7, assign85950_e131669_d_n8, assign85950_e131669_d_n9, assign85950_e131669_d_n10, assign85950_e131669_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ps0ld_ini__blk2011, locals.var_ps0ld_ini__blk2011_dn0, locals.var_ps0ld_ini__blk2011_dn2, locals.var_ps0ld_ini__blk2011_dn4, locals.var_ps0ld_ini__blk2011_dn5, locals.var_ps0ld_ini__blk2011_dn6, locals.var_ps0ld_ini__blk2011_dn7, locals.var_ps0ld_ini__blk2011_dn8, locals.var_ps0ld_ini__blk2011_dn9, locals.var_ps0ld_ini__blk2011_dn10, locals.var_ps0ld_ini__blk2011_dn13,)
    }
};
        locals.var_ps0ld_ini__blk2011 = assign85950_e131669;
        locals.var_ps0ld_ini__blk2011_dn0 = assign85950_e131669_d_n0;
        locals.var_ps0ld_ini__blk2011_dn2 = assign85950_e131669_d_n2;
        locals.var_ps0ld_ini__blk2011_dn4 = assign85950_e131669_d_n4;
        locals.var_ps0ld_ini__blk2011_dn5 = assign85950_e131669_d_n5;
        locals.var_ps0ld_ini__blk2011_dn6 = assign85950_e131669_d_n6;
        locals.var_ps0ld_ini__blk2011_dn7 = assign85950_e131669_d_n7;
        locals.var_ps0ld_ini__blk2011_dn8 = assign85950_e131669_d_n8;
        locals.var_ps0ld_ini__blk2011_dn9 = assign85950_e131669_d_n9;
        locals.var_ps0ld_ini__blk2011_dn10 = assign85950_e131669_d_n10;
        locals.var_ps0ld_ini__blk2011_dn13 = assign85950_e131669_d_n13;
        locals.var_ps0ld_ini__blk2011_rv = 0.0;

        let (assign85960_e131673, assign85960_e131673_d_n0, assign85960_e131673_d_n2, assign85960_e131673_d_n4, assign85960_e131673_d_n5, assign85960_e131673_d_n6, assign85960_e131673_d_n7, assign85960_e131673_d_n8, assign85960_e131673_d_n9, assign85960_e131673_d_n10, assign85960_e131673_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fbsq__blk2012, locals.var_fbsq__blk2012_dn0, locals.var_fbsq__blk2012_dn2, locals.var_fbsq__blk2012_dn4, locals.var_fbsq__blk2012_dn5, locals.var_fbsq__blk2012_dn6, locals.var_fbsq__blk2012_dn7, locals.var_fbsq__blk2012_dn8, locals.var_fbsq__blk2012_dn9, locals.var_fbsq__blk2012_dn10, locals.var_fbsq__blk2012_dn13,)
    }
};
        locals.var_fbsq__blk2012 = assign85960_e131673;
        locals.var_fbsq__blk2012_dn0 = assign85960_e131673_d_n0;
        locals.var_fbsq__blk2012_dn2 = assign85960_e131673_d_n2;
        locals.var_fbsq__blk2012_dn4 = assign85960_e131673_d_n4;
        locals.var_fbsq__blk2012_dn5 = assign85960_e131673_d_n5;
        locals.var_fbsq__blk2012_dn6 = assign85960_e131673_d_n6;
        locals.var_fbsq__blk2012_dn7 = assign85960_e131673_d_n7;
        locals.var_fbsq__blk2012_dn8 = assign85960_e131673_d_n8;
        locals.var_fbsq__blk2012_dn9 = assign85960_e131673_d_n9;
        locals.var_fbsq__blk2012_dn10 = assign85960_e131673_d_n10;
        locals.var_fbsq__blk2012_dn13 = assign85960_e131673_d_n13;
        locals.var_fbsq__blk2012_rv = 0.0;

        let (assign85970_e131684, assign85970_e131684_d_n0, assign85970_e131684_d_n2, assign85970_e131684_d_n4, assign85970_e131684_d_n5, assign85970_e131684_d_n6, assign85970_e131684_d_n7, assign85970_e131684_d_n8, assign85970_e131684_d_n9, assign85970_e131684_d_n10, assign85970_e131684_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign85970_e131677: f64 = (2.0 * locals.var_beta_inv);
        let assign85970_e131680: f64 = (locals.var_nover_func / locals.var_nin);
        let assign85970_e131681: f64 = (assign85970_e131680).ln();
        let assign85970_e131682: f64 = (assign85970_e131677 * assign85970_e131681);
        (assign85970_e131682, (((2.0 * locals.var_beta_inv_dn0) * assign85970_e131681) + (assign85970_e131677 * ((-((locals.var_nover_func * locals.var_nin_dn0) / (locals.var_nin * locals.var_nin))) / assign85970_e131680))), (((2.0 * locals.var_beta_inv_dn2) * assign85970_e131681) + (assign85970_e131677 * ((-((locals.var_nover_func * locals.var_nin_dn2) / (locals.var_nin * locals.var_nin))) / assign85970_e131680))), (((2.0 * locals.var_beta_inv_dn4) * assign85970_e131681) + (assign85970_e131677 * ((-((locals.var_nover_func * locals.var_nin_dn4) / (locals.var_nin * locals.var_nin))) / assign85970_e131680))), (((2.0 * locals.var_beta_inv_dn5) * assign85970_e131681) + (assign85970_e131677 * ((-((locals.var_nover_func * locals.var_nin_dn5) / (locals.var_nin * locals.var_nin))) / assign85970_e131680))), (((2.0 * locals.var_beta_inv_dn6) * assign85970_e131681) + (assign85970_e131677 * ((-((locals.var_nover_func * locals.var_nin_dn6) / (locals.var_nin * locals.var_nin))) / assign85970_e131680))), (((2.0 * locals.var_beta_inv_dn7) * assign85970_e131681) + (assign85970_e131677 * ((-((locals.var_nover_func * locals.var_nin_dn7) / (locals.var_nin * locals.var_nin))) / assign85970_e131680))), (((2.0 * locals.var_beta_inv_dn8) * assign85970_e131681) + (assign85970_e131677 * ((-((locals.var_nover_func * locals.var_nin_dn8) / (locals.var_nin * locals.var_nin))) / assign85970_e131680))), (((2.0 * locals.var_beta_inv_dn9) * assign85970_e131681) + (assign85970_e131677 * ((-((locals.var_nover_func * locals.var_nin_dn9) / (locals.var_nin * locals.var_nin))) / assign85970_e131680))), (((2.0 * locals.var_beta_inv_dn10) * assign85970_e131681) + (assign85970_e131677 * ((-((locals.var_nover_func * locals.var_nin_dn10) / (locals.var_nin * locals.var_nin))) / assign85970_e131680))), (((2.0 * locals.var_beta_inv_dn13) * assign85970_e131681) + (assign85970_e131677 * ((-((locals.var_nover_func * locals.var_nin_dn13) / (locals.var_nin * locals.var_nin))) / assign85970_e131680))),)
    } else {
        (locals.var_pb2over__blk2007, locals.var_pb2over__blk2007_dn0, locals.var_pb2over__blk2007_dn2, locals.var_pb2over__blk2007_dn4, locals.var_pb2over__blk2007_dn5, locals.var_pb2over__blk2007_dn6, locals.var_pb2over__blk2007_dn7, locals.var_pb2over__blk2007_dn8, locals.var_pb2over__blk2007_dn9, locals.var_pb2over__blk2007_dn10, locals.var_pb2over__blk2007_dn13,)
    }
};
        locals.var_pb2over__blk2007 = assign85970_e131684;
        locals.var_pb2over__blk2007_dn0 = assign85970_e131684_d_n0;
        locals.var_pb2over__blk2007_dn2 = assign85970_e131684_d_n2;
        locals.var_pb2over__blk2007_dn4 = assign85970_e131684_d_n4;
        locals.var_pb2over__blk2007_dn5 = assign85970_e131684_d_n5;
        locals.var_pb2over__blk2007_dn6 = assign85970_e131684_d_n6;
        locals.var_pb2over__blk2007_dn7 = assign85970_e131684_d_n7;
        locals.var_pb2over__blk2007_dn8 = assign85970_e131684_d_n8;
        locals.var_pb2over__blk2007_dn9 = assign85970_e131684_d_n9;
        locals.var_pb2over__blk2007_dn10 = assign85970_e131684_d_n10;
        locals.var_pb2over__blk2007_dn13 = assign85970_e131684_d_n13;
        locals.var_pb2over__blk2007_rv = 0.0;

        let (assign85980_e131692, assign85980_e131692_d_n0, assign85980_e131692_d_n2, assign85980_e131692_d_n4, assign85980_e131692_d_n5, assign85980_e131692_d_n6, assign85980_e131692_d_n7, assign85980_e131692_d_n8, assign85980_e131692_d_n9, assign85980_e131692_d_n10, assign85980_e131692_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign85980_e131688: f64 = (0.8 - locals.var_pb2over__blk2007);
        let assign85980_e131690: f64 = (assign85980_e131688 - 0.1);
        (assign85980_e131690, (-locals.var_pb2over__blk2007_dn0), (-locals.var_pb2over__blk2007_dn2), (-locals.var_pb2over__blk2007_dn4), (-locals.var_pb2over__blk2007_dn5), (-locals.var_pb2over__blk2007_dn6), (-locals.var_pb2over__blk2007_dn7), (-locals.var_pb2over__blk2007_dn8), (-locals.var_pb2over__blk2007_dn9), (-locals.var_pb2over__blk2007_dn10), (-locals.var_pb2over__blk2007_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign85980_e131692;
        locals.var_tmf1_dn0 = assign85980_e131692_d_n0;
        locals.var_tmf1_dn2 = assign85980_e131692_d_n2;
        locals.var_tmf1_dn4 = assign85980_e131692_d_n4;
        locals.var_tmf1_dn5 = assign85980_e131692_d_n5;
        locals.var_tmf1_dn6 = assign85980_e131692_d_n6;
        locals.var_tmf1_dn7 = assign85980_e131692_d_n7;
        locals.var_tmf1_dn8 = assign85980_e131692_d_n8;
        locals.var_tmf1_dn9 = assign85980_e131692_d_n9;
        locals.var_tmf1_dn10 = assign85980_e131692_d_n10;
        locals.var_tmf1_dn13 = assign85980_e131692_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign85990_e131700, assign85990_e131700_d_n0, assign85990_e131700_d_n2, assign85990_e131700_d_n4, assign85990_e131700_d_n5, assign85990_e131700_d_n6, assign85990_e131700_d_n7, assign85990_e131700_d_n8, assign85990_e131700_d_n9, assign85990_e131700_d_n10, assign85990_e131700_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign85990_e131696: f64 = (4.0 * 0.8);
        let assign85990_e131698: f64 = (assign85990_e131696 * 0.1);
        (assign85990_e131698, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign85990_e131700;
        locals.var_tmf2_dn0 = assign85990_e131700_d_n0;
        locals.var_tmf2_dn2 = assign85990_e131700_d_n2;
        locals.var_tmf2_dn4 = assign85990_e131700_d_n4;
        locals.var_tmf2_dn5 = assign85990_e131700_d_n5;
        locals.var_tmf2_dn6 = assign85990_e131700_d_n6;
        locals.var_tmf2_dn7 = assign85990_e131700_d_n7;
        locals.var_tmf2_dn8 = assign85990_e131700_d_n8;
        locals.var_tmf2_dn9 = assign85990_e131700_d_n9;
        locals.var_tmf2_dn10 = assign85990_e131700_d_n10;
        locals.var_tmf2_dn13 = assign85990_e131700_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign86000_e131710, assign86000_e131710_d_n0, assign86000_e131710_d_n2, assign86000_e131710_d_n4, assign86000_e131710_d_n5, assign86000_e131710_d_n6, assign86000_e131710_d_n7, assign86000_e131710_d_n8, assign86000_e131710_d_n9, assign86000_e131710_d_n10, assign86000_e131710_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let (assign86000_e131708, assign86000_e131708_d_n0, assign86000_e131708_d_n2, assign86000_e131708_d_n4, assign86000_e131708_d_n5, assign86000_e131708_d_n6, assign86000_e131708_d_n7, assign86000_e131708_d_n8, assign86000_e131708_d_n9, assign86000_e131708_d_n10, assign86000_e131708_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign86000_e131707: f64 = (-locals.var_tmf2);
                (assign86000_e131707, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign86000_e131708, assign86000_e131708_d_n0, assign86000_e131708_d_n2, assign86000_e131708_d_n4, assign86000_e131708_d_n5, assign86000_e131708_d_n6, assign86000_e131708_d_n7, assign86000_e131708_d_n8, assign86000_e131708_d_n9, assign86000_e131708_d_n10, assign86000_e131708_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign86000_e131710;
        locals.var_tmf2_dn0 = assign86000_e131710_d_n0;
        locals.var_tmf2_dn2 = assign86000_e131710_d_n2;
        locals.var_tmf2_dn4 = assign86000_e131710_d_n4;
        locals.var_tmf2_dn5 = assign86000_e131710_d_n5;
        locals.var_tmf2_dn6 = assign86000_e131710_d_n6;
        locals.var_tmf2_dn7 = assign86000_e131710_d_n7;
        locals.var_tmf2_dn8 = assign86000_e131710_d_n8;
        locals.var_tmf2_dn9 = assign86000_e131710_d_n9;
        locals.var_tmf2_dn10 = assign86000_e131710_d_n10;
        locals.var_tmf2_dn13 = assign86000_e131710_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign86010_e131719, assign86010_e131719_d_n0, assign86010_e131719_d_n2, assign86010_e131719_d_n4, assign86010_e131719_d_n5, assign86010_e131719_d_n6, assign86010_e131719_d_n7, assign86010_e131719_d_n8, assign86010_e131719_d_n9, assign86010_e131719_d_n10, assign86010_e131719_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign86010_e131714: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign86010_e131716: f64 = (assign86010_e131714 + locals.var_tmf2);
        let assign86010_e131717: f64 = (assign86010_e131716).sqrt();
        (assign86010_e131717, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign86010_e131717)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign86010_e131717)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign86010_e131717)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign86010_e131717)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign86010_e131717)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign86010_e131717)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign86010_e131717)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign86010_e131717)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign86010_e131717)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign86010_e131717)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign86010_e131719;
        locals.var_tmf2_dn0 = assign86010_e131719_d_n0;
        locals.var_tmf2_dn2 = assign86010_e131719_d_n2;
        locals.var_tmf2_dn4 = assign86010_e131719_d_n4;
        locals.var_tmf2_dn5 = assign86010_e131719_d_n5;
        locals.var_tmf2_dn6 = assign86010_e131719_d_n6;
        locals.var_tmf2_dn7 = assign86010_e131719_d_n7;
        locals.var_tmf2_dn8 = assign86010_e131719_d_n8;
        locals.var_tmf2_dn9 = assign86010_e131719_d_n9;
        locals.var_tmf2_dn10 = assign86010_e131719_d_n10;
        locals.var_tmf2_dn13 = assign86010_e131719_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign86020_e131729, assign86020_e131729_d_n0, assign86020_e131729_d_n2, assign86020_e131729_d_n4, assign86020_e131729_d_n5, assign86020_e131729_d_n6, assign86020_e131729_d_n7, assign86020_e131729_d_n8, assign86020_e131729_d_n9, assign86020_e131729_d_n10, assign86020_e131729_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign86020_e131725: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign86020_e131726: f64 = (1.0 + assign86020_e131725);
        let assign86020_e131727: f64 = (0.5 * assign86020_e131726);
        (assign86020_e131727, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign86020_e131729;
        locals.var_t0_dn0 = assign86020_e131729_d_n0;
        locals.var_t0_dn2 = assign86020_e131729_d_n2;
        locals.var_t0_dn4 = assign86020_e131729_d_n4;
        locals.var_t0_dn5 = assign86020_e131729_d_n5;
        locals.var_t0_dn6 = assign86020_e131729_d_n6;
        locals.var_t0_dn7 = assign86020_e131729_d_n7;
        locals.var_t0_dn8 = assign86020_e131729_d_n8;
        locals.var_t0_dn9 = assign86020_e131729_d_n9;
        locals.var_t0_dn10 = assign86020_e131729_d_n10;
        locals.var_t0_dn13 = assign86020_e131729_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign86030_e131739, assign86030_e131739_d_n0, assign86030_e131739_d_n2, assign86030_e131739_d_n4, assign86030_e131739_d_n5, assign86030_e131739_d_n6, assign86030_e131739_d_n7, assign86030_e131739_d_n8, assign86030_e131739_d_n9, assign86030_e131739_d_n10, assign86030_e131739_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign86030_e131735: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign86030_e131736: f64 = (0.5 * assign86030_e131735);
        let assign86030_e131737: f64 = (0.8 - assign86030_e131736);
        (assign86030_e131737, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (-(0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (-(0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (-(0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (-(0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (-(0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (-(0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (-(0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (-(0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13))),)
    } else {
        (locals.var_vbs_max_over__blk2008, locals.var_vbs_max_over__blk2008_dn0, locals.var_vbs_max_over__blk2008_dn2, locals.var_vbs_max_over__blk2008_dn4, locals.var_vbs_max_over__blk2008_dn5, locals.var_vbs_max_over__blk2008_dn6, locals.var_vbs_max_over__blk2008_dn7, locals.var_vbs_max_over__blk2008_dn8, locals.var_vbs_max_over__blk2008_dn9, locals.var_vbs_max_over__blk2008_dn10, locals.var_vbs_max_over__blk2008_dn13,)
    }
};
        locals.var_vbs_max_over__blk2008 = assign86030_e131739;
        locals.var_vbs_max_over__blk2008_dn0 = assign86030_e131739_d_n0;
        locals.var_vbs_max_over__blk2008_dn2 = assign86030_e131739_d_n2;
        locals.var_vbs_max_over__blk2008_dn4 = assign86030_e131739_d_n4;
        locals.var_vbs_max_over__blk2008_dn5 = assign86030_e131739_d_n5;
        locals.var_vbs_max_over__blk2008_dn6 = assign86030_e131739_d_n6;
        locals.var_vbs_max_over__blk2008_dn7 = assign86030_e131739_d_n7;
        locals.var_vbs_max_over__blk2008_dn8 = assign86030_e131739_d_n8;
        locals.var_vbs_max_over__blk2008_dn9 = assign86030_e131739_d_n9;
        locals.var_vbs_max_over__blk2008_dn10 = assign86030_e131739_d_n10;
        locals.var_vbs_max_over__blk2008_dn13 = assign86030_e131739_d_n13;
        locals.var_vbs_max_over__blk2008_rv = 0.0;

        let assign86040_e131743: f64 = (locals.var_vbs_max_over__blk2008 * 0.5);
        let assign86040_e131744: f64 = if locals.var_vbs_bnd_over__blk2009 > assign86040_e131743 { 1.0 } else { 0.0 };
        locals.var_guard2014 = assign86040_e131744;
        locals.var_guard2014_rv = 0.0;

        let (assign86050_e131752, assign86050_e131752_d_n0, assign86050_e131752_d_n2, assign86050_e131752_d_n4, assign86050_e131752_d_n5, assign86050_e131752_d_n6, assign86050_e131752_d_n7, assign86050_e131752_d_n8, assign86050_e131752_d_n9, assign86050_e131752_d_n10, assign86050_e131752_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2014 != 0.0)) {
        let assign86050_e131750: f64 = (0.5 * locals.var_vbs_max_over__blk2008);
        (assign86050_e131750, (0.5 * locals.var_vbs_max_over__blk2008_dn0), (0.5 * locals.var_vbs_max_over__blk2008_dn2), (0.5 * locals.var_vbs_max_over__blk2008_dn4), (0.5 * locals.var_vbs_max_over__blk2008_dn5), (0.5 * locals.var_vbs_max_over__blk2008_dn6), (0.5 * locals.var_vbs_max_over__blk2008_dn7), (0.5 * locals.var_vbs_max_over__blk2008_dn8), (0.5 * locals.var_vbs_max_over__blk2008_dn9), (0.5 * locals.var_vbs_max_over__blk2008_dn10), (0.5 * locals.var_vbs_max_over__blk2008_dn13),)
    } else {
        (locals.var_vbs_bnd_over__blk2009, locals.var_vbs_bnd_over__blk2009_dn0, locals.var_vbs_bnd_over__blk2009_dn2, locals.var_vbs_bnd_over__blk2009_dn4, locals.var_vbs_bnd_over__blk2009_dn5, locals.var_vbs_bnd_over__blk2009_dn6, locals.var_vbs_bnd_over__blk2009_dn7, locals.var_vbs_bnd_over__blk2009_dn8, locals.var_vbs_bnd_over__blk2009_dn9, locals.var_vbs_bnd_over__blk2009_dn10, locals.var_vbs_bnd_over__blk2009_dn13,)
    }
};
        locals.var_vbs_bnd_over__blk2009 = assign86050_e131752;
        locals.var_vbs_bnd_over__blk2009_dn0 = assign86050_e131752_d_n0;
        locals.var_vbs_bnd_over__blk2009_dn2 = assign86050_e131752_d_n2;
        locals.var_vbs_bnd_over__blk2009_dn4 = assign86050_e131752_d_n4;
        locals.var_vbs_bnd_over__blk2009_dn5 = assign86050_e131752_d_n5;
        locals.var_vbs_bnd_over__blk2009_dn6 = assign86050_e131752_d_n6;
        locals.var_vbs_bnd_over__blk2009_dn7 = assign86050_e131752_d_n7;
        locals.var_vbs_bnd_over__blk2009_dn8 = assign86050_e131752_d_n8;
        locals.var_vbs_bnd_over__blk2009_dn9 = assign86050_e131752_d_n9;
        locals.var_vbs_bnd_over__blk2009_dn10 = assign86050_e131752_d_n10;
        locals.var_vbs_bnd_over__blk2009_dn13 = assign86050_e131752_d_n13;
        locals.var_vbs_bnd_over__blk2009_rv = 0.0;

        let assign86060_e131754: f64 = if param_given[338] { 1.0 } else { 0.0 };
        locals.var_guard2015 = assign86060_e131754;
        locals.var_guard2015_rv = 0.0;

        let (assign86070_e131760, assign86070_e131760_d_n0, assign86070_e131760_d_n2, assign86070_e131760_d_n4, assign86070_e131760_d_n5, assign86070_e131760_d_n6, assign86070_e131760_d_n7, assign86070_e131760_d_n8, assign86070_e131760_d_n9, assign86070_e131760_d_n10, assign86070_e131760_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2015 != 0.0)) {
        (p.p338, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbs_max_over__blk2008, locals.var_vbs_max_over__blk2008_dn0, locals.var_vbs_max_over__blk2008_dn2, locals.var_vbs_max_over__blk2008_dn4, locals.var_vbs_max_over__blk2008_dn5, locals.var_vbs_max_over__blk2008_dn6, locals.var_vbs_max_over__blk2008_dn7, locals.var_vbs_max_over__blk2008_dn8, locals.var_vbs_max_over__blk2008_dn9, locals.var_vbs_max_over__blk2008_dn10, locals.var_vbs_max_over__blk2008_dn13,)
    }
};
        locals.var_vbs_max_over__blk2008 = assign86070_e131760;
        locals.var_vbs_max_over__blk2008_dn0 = assign86070_e131760_d_n0;
        locals.var_vbs_max_over__blk2008_dn2 = assign86070_e131760_d_n2;
        locals.var_vbs_max_over__blk2008_dn4 = assign86070_e131760_d_n4;
        locals.var_vbs_max_over__blk2008_dn5 = assign86070_e131760_d_n5;
        locals.var_vbs_max_over__blk2008_dn6 = assign86070_e131760_d_n6;
        locals.var_vbs_max_over__blk2008_dn7 = assign86070_e131760_d_n7;
        locals.var_vbs_max_over__blk2008_dn8 = assign86070_e131760_d_n8;
        locals.var_vbs_max_over__blk2008_dn9 = assign86070_e131760_d_n9;
        locals.var_vbs_max_over__blk2008_dn10 = assign86070_e131760_d_n10;
        locals.var_vbs_max_over__blk2008_dn13 = assign86070_e131760_d_n13;
        locals.var_vbs_max_over__blk2008_rv = 0.0;

        let assign86080_e131762: f64 = if param_given[339] { 1.0 } else { 0.0 };
        locals.var_guard2016 = assign86080_e131762;
        locals.var_guard2016_rv = 0.0;

        let (assign86090_e131768, assign86090_e131768_d_n0, assign86090_e131768_d_n2, assign86090_e131768_d_n4, assign86090_e131768_d_n5, assign86090_e131768_d_n6, assign86090_e131768_d_n7, assign86090_e131768_d_n8, assign86090_e131768_d_n9, assign86090_e131768_d_n10, assign86090_e131768_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2016 != 0.0)) {
        (p.p339, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbs_bnd_over__blk2009, locals.var_vbs_bnd_over__blk2009_dn0, locals.var_vbs_bnd_over__blk2009_dn2, locals.var_vbs_bnd_over__blk2009_dn4, locals.var_vbs_bnd_over__blk2009_dn5, locals.var_vbs_bnd_over__blk2009_dn6, locals.var_vbs_bnd_over__blk2009_dn7, locals.var_vbs_bnd_over__blk2009_dn8, locals.var_vbs_bnd_over__blk2009_dn9, locals.var_vbs_bnd_over__blk2009_dn10, locals.var_vbs_bnd_over__blk2009_dn13,)
    }
};
        locals.var_vbs_bnd_over__blk2009 = assign86090_e131768;
        locals.var_vbs_bnd_over__blk2009_dn0 = assign86090_e131768_d_n0;
        locals.var_vbs_bnd_over__blk2009_dn2 = assign86090_e131768_d_n2;
        locals.var_vbs_bnd_over__blk2009_dn4 = assign86090_e131768_d_n4;
        locals.var_vbs_bnd_over__blk2009_dn5 = assign86090_e131768_d_n5;
        locals.var_vbs_bnd_over__blk2009_dn6 = assign86090_e131768_d_n6;
        locals.var_vbs_bnd_over__blk2009_dn7 = assign86090_e131768_d_n7;
        locals.var_vbs_bnd_over__blk2009_dn8 = assign86090_e131768_d_n8;
        locals.var_vbs_bnd_over__blk2009_dn9 = assign86090_e131768_d_n9;
        locals.var_vbs_bnd_over__blk2009_dn10 = assign86090_e131768_d_n10;
        locals.var_vbs_bnd_over__blk2009_dn13 = assign86090_e131768_d_n13;
        locals.var_vbs_bnd_over__blk2009_rv = 0.0;

        let assign86100_e131770: f64 = if param_given[338] { 1.0 } else { 0.0 };
        locals.var_guard2017 = assign86100_e131770;
        locals.var_guard2017_rv = 0.0;

        let (assign86110_e131781, assign86110_e131781_d_n0, assign86110_e131781_d_n2, assign86110_e131781_d_n4, assign86110_e131781_d_n5, assign86110_e131781_d_n6, assign86110_e131781_d_n7, assign86110_e131781_d_n8, assign86110_e131781_d_n9, assign86110_e131781_d_n10, assign86110_e131781_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2016 == 0.0)) && (locals.var_guard2017 != 0.0)) {
        let assign86110_e131779: f64 = (0.5 * locals.var_vbs_max_over__blk2008);
        (assign86110_e131779, (0.5 * locals.var_vbs_max_over__blk2008_dn0), (0.5 * locals.var_vbs_max_over__blk2008_dn2), (0.5 * locals.var_vbs_max_over__blk2008_dn4), (0.5 * locals.var_vbs_max_over__blk2008_dn5), (0.5 * locals.var_vbs_max_over__blk2008_dn6), (0.5 * locals.var_vbs_max_over__blk2008_dn7), (0.5 * locals.var_vbs_max_over__blk2008_dn8), (0.5 * locals.var_vbs_max_over__blk2008_dn9), (0.5 * locals.var_vbs_max_over__blk2008_dn10), (0.5 * locals.var_vbs_max_over__blk2008_dn13),)
    } else {
        (locals.var_vbs_bnd_over__blk2009, locals.var_vbs_bnd_over__blk2009_dn0, locals.var_vbs_bnd_over__blk2009_dn2, locals.var_vbs_bnd_over__blk2009_dn4, locals.var_vbs_bnd_over__blk2009_dn5, locals.var_vbs_bnd_over__blk2009_dn6, locals.var_vbs_bnd_over__blk2009_dn7, locals.var_vbs_bnd_over__blk2009_dn8, locals.var_vbs_bnd_over__blk2009_dn9, locals.var_vbs_bnd_over__blk2009_dn10, locals.var_vbs_bnd_over__blk2009_dn13,)
    }
};
        locals.var_vbs_bnd_over__blk2009 = assign86110_e131781;
        locals.var_vbs_bnd_over__blk2009_dn0 = assign86110_e131781_d_n0;
        locals.var_vbs_bnd_over__blk2009_dn2 = assign86110_e131781_d_n2;
        locals.var_vbs_bnd_over__blk2009_dn4 = assign86110_e131781_d_n4;
        locals.var_vbs_bnd_over__blk2009_dn5 = assign86110_e131781_d_n5;
        locals.var_vbs_bnd_over__blk2009_dn6 = assign86110_e131781_d_n6;
        locals.var_vbs_bnd_over__blk2009_dn7 = assign86110_e131781_d_n7;
        locals.var_vbs_bnd_over__blk2009_dn8 = assign86110_e131781_d_n8;
        locals.var_vbs_bnd_over__blk2009_dn9 = assign86110_e131781_d_n9;
        locals.var_vbs_bnd_over__blk2009_dn10 = assign86110_e131781_d_n10;
        locals.var_vbs_bnd_over__blk2009_dn13 = assign86110_e131781_d_n13;
        locals.var_vbs_bnd_over__blk2009_rv = 0.0;

        let assign86120_e131785: f64 = (locals.var_vbs_max_over__blk2008 * 0.5);
        let assign86120_e131786: f64 = if locals.var_vbs_bnd_over__blk2009 > assign86120_e131785 { 1.0 } else { 0.0 };
        locals.var_guard2018 = assign86120_e131786;
        locals.var_guard2018_rv = 0.0;

        let (assign86130_e131794, assign86130_e131794_d_n0, assign86130_e131794_d_n2, assign86130_e131794_d_n4, assign86130_e131794_d_n5, assign86130_e131794_d_n6, assign86130_e131794_d_n7, assign86130_e131794_d_n8, assign86130_e131794_d_n9, assign86130_e131794_d_n10, assign86130_e131794_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2018 != 0.0)) {
        let assign86130_e131792: f64 = (0.5 * locals.var_vbs_max_over__blk2008);
        (assign86130_e131792, (0.5 * locals.var_vbs_max_over__blk2008_dn0), (0.5 * locals.var_vbs_max_over__blk2008_dn2), (0.5 * locals.var_vbs_max_over__blk2008_dn4), (0.5 * locals.var_vbs_max_over__blk2008_dn5), (0.5 * locals.var_vbs_max_over__blk2008_dn6), (0.5 * locals.var_vbs_max_over__blk2008_dn7), (0.5 * locals.var_vbs_max_over__blk2008_dn8), (0.5 * locals.var_vbs_max_over__blk2008_dn9), (0.5 * locals.var_vbs_max_over__blk2008_dn10), (0.5 * locals.var_vbs_max_over__blk2008_dn13),)
    } else {
        (locals.var_vbs_bnd_over__blk2009, locals.var_vbs_bnd_over__blk2009_dn0, locals.var_vbs_bnd_over__blk2009_dn2, locals.var_vbs_bnd_over__blk2009_dn4, locals.var_vbs_bnd_over__blk2009_dn5, locals.var_vbs_bnd_over__blk2009_dn6, locals.var_vbs_bnd_over__blk2009_dn7, locals.var_vbs_bnd_over__blk2009_dn8, locals.var_vbs_bnd_over__blk2009_dn9, locals.var_vbs_bnd_over__blk2009_dn10, locals.var_vbs_bnd_over__blk2009_dn13,)
    }
};
        locals.var_vbs_bnd_over__blk2009 = assign86130_e131794;
        locals.var_vbs_bnd_over__blk2009_dn0 = assign86130_e131794_d_n0;
        locals.var_vbs_bnd_over__blk2009_dn2 = assign86130_e131794_d_n2;
        locals.var_vbs_bnd_over__blk2009_dn4 = assign86130_e131794_d_n4;
        locals.var_vbs_bnd_over__blk2009_dn5 = assign86130_e131794_d_n5;
        locals.var_vbs_bnd_over__blk2009_dn6 = assign86130_e131794_d_n6;
        locals.var_vbs_bnd_over__blk2009_dn7 = assign86130_e131794_d_n7;
        locals.var_vbs_bnd_over__blk2009_dn8 = assign86130_e131794_d_n8;
        locals.var_vbs_bnd_over__blk2009_dn9 = assign86130_e131794_d_n9;
        locals.var_vbs_bnd_over__blk2009_dn10 = assign86130_e131794_d_n10;
        locals.var_vbs_bnd_over__blk2009_dn13 = assign86130_e131794_d_n13;
        locals.var_vbs_bnd_over__blk2009_rv = 0.0;

        let assign86140_e131797: f64 = if p.p38 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2019 = assign86140_e131797;
        locals.var_guard2019_rv = 0.0;

        let (assign86150_e131804, assign86150_e131804_d_n0, assign86150_e131804_d_n2, assign86150_e131804_d_n4, assign86150_e131804_d_n5, assign86150_e131804_d_n6, assign86150_e131804_d_n7, assign86150_e131804_d_n8, assign86150_e131804_d_n9, assign86150_e131804_d_n10, assign86150_e131804_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2019 != 0.0)) {
        let assign86150_e131802: f64 = (-locals.var_vxbgmt);
        (assign86150_e131802, (-locals.var_vxbgmt_dn0), (-locals.var_vxbgmt_dn2), (-locals.var_vxbgmt_dn4), (-locals.var_vxbgmt_dn5), (-locals.var_vxbgmt_dn6), (-locals.var_vxbgmt_dn7), (-locals.var_vxbgmt_dn8), (-locals.var_vxbgmt_dn9), (-locals.var_vxbgmt_dn10), (-locals.var_vxbgmt_dn13),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign86150_e131804;
        locals.var_t0_dn0 = assign86150_e131804_d_n0;
        locals.var_t0_dn2 = assign86150_e131804_d_n2;
        locals.var_t0_dn4 = assign86150_e131804_d_n4;
        locals.var_t0_dn5 = assign86150_e131804_d_n5;
        locals.var_t0_dn6 = assign86150_e131804_d_n6;
        locals.var_t0_dn7 = assign86150_e131804_d_n7;
        locals.var_t0_dn8 = assign86150_e131804_d_n8;
        locals.var_t0_dn9 = assign86150_e131804_d_n9;
        locals.var_t0_dn10 = assign86150_e131804_d_n10;
        locals.var_t0_dn13 = assign86150_e131804_d_n13;
        locals.var_t0_rv = 0.0;

        let assign86160_e131807: f64 = if locals.var_t0 > locals.var_vbs_bnd_over__blk2009 { 1.0 } else { 0.0 };
        locals.var_guard2020 = assign86160_e131807;
        locals.var_guard2020_rv = 0.0;

        let (assign86170_e131817, assign86170_e131817_d_n0, assign86170_e131817_d_n2, assign86170_e131817_d_n4, assign86170_e131817_d_n5, assign86170_e131817_d_n6, assign86170_e131817_d_n7, assign86170_e131817_d_n8, assign86170_e131817_d_n9, assign86170_e131817_d_n10, assign86170_e131817_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2019 != 0.0)) && (locals.var_guard2020 != 0.0)) {
        let assign86170_e131815: f64 = (locals.var_t0 - locals.var_vbs_bnd_over__blk2009);
        (assign86170_e131815, (locals.var_t0_dn0 - locals.var_vbs_bnd_over__blk2009_dn0), (locals.var_t0_dn2 - locals.var_vbs_bnd_over__blk2009_dn2), (locals.var_t0_dn4 - locals.var_vbs_bnd_over__blk2009_dn4), (locals.var_t0_dn5 - locals.var_vbs_bnd_over__blk2009_dn5), (locals.var_t0_dn6 - locals.var_vbs_bnd_over__blk2009_dn6), (locals.var_t0_dn7 - locals.var_vbs_bnd_over__blk2009_dn7), (locals.var_t0_dn8 - locals.var_vbs_bnd_over__blk2009_dn8), (locals.var_t0_dn9 - locals.var_vbs_bnd_over__blk2009_dn9), (locals.var_t0_dn10 - locals.var_vbs_bnd_over__blk2009_dn10), (locals.var_t0_dn13 - locals.var_vbs_bnd_over__blk2009_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign86170_e131817;
        locals.var_t1_dn0 = assign86170_e131817_d_n0;
        locals.var_t1_dn2 = assign86170_e131817_d_n2;
        locals.var_t1_dn4 = assign86170_e131817_d_n4;
        locals.var_t1_dn5 = assign86170_e131817_d_n5;
        locals.var_t1_dn6 = assign86170_e131817_d_n6;
        locals.var_t1_dn7 = assign86170_e131817_d_n7;
        locals.var_t1_dn8 = assign86170_e131817_d_n8;
        locals.var_t1_dn9 = assign86170_e131817_d_n9;
        locals.var_t1_dn10 = assign86170_e131817_d_n10;
        locals.var_t1_dn13 = assign86170_e131817_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign86180_e131827, assign86180_e131827_d_n0, assign86180_e131827_d_n2, assign86180_e131827_d_n4, assign86180_e131827_d_n5, assign86180_e131827_d_n6, assign86180_e131827_d_n7, assign86180_e131827_d_n8, assign86180_e131827_d_n9, assign86180_e131827_d_n10, assign86180_e131827_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2019 != 0.0)) && (locals.var_guard2020 != 0.0)) {
        let assign86180_e131825: f64 = (locals.var_vbs_max_over__blk2008 - locals.var_vbs_bnd_over__blk2009);
        (assign86180_e131825, (locals.var_vbs_max_over__blk2008_dn0 - locals.var_vbs_bnd_over__blk2009_dn0), (locals.var_vbs_max_over__blk2008_dn2 - locals.var_vbs_bnd_over__blk2009_dn2), (locals.var_vbs_max_over__blk2008_dn4 - locals.var_vbs_bnd_over__blk2009_dn4), (locals.var_vbs_max_over__blk2008_dn5 - locals.var_vbs_bnd_over__blk2009_dn5), (locals.var_vbs_max_over__blk2008_dn6 - locals.var_vbs_bnd_over__blk2009_dn6), (locals.var_vbs_max_over__blk2008_dn7 - locals.var_vbs_bnd_over__blk2009_dn7), (locals.var_vbs_max_over__blk2008_dn8 - locals.var_vbs_bnd_over__blk2009_dn8), (locals.var_vbs_max_over__blk2008_dn9 - locals.var_vbs_bnd_over__blk2009_dn9), (locals.var_vbs_max_over__blk2008_dn10 - locals.var_vbs_bnd_over__blk2009_dn10), (locals.var_vbs_max_over__blk2008_dn13 - locals.var_vbs_bnd_over__blk2009_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign86180_e131827;
        locals.var_t2_dn0 = assign86180_e131827_d_n0;
        locals.var_t2_dn2 = assign86180_e131827_d_n2;
        locals.var_t2_dn4 = assign86180_e131827_d_n4;
        locals.var_t2_dn5 = assign86180_e131827_d_n5;
        locals.var_t2_dn6 = assign86180_e131827_d_n6;
        locals.var_t2_dn7 = assign86180_e131827_d_n7;
        locals.var_t2_dn8 = assign86180_e131827_d_n8;
        locals.var_t2_dn9 = assign86180_e131827_d_n9;
        locals.var_t2_dn10 = assign86180_e131827_d_n10;
        locals.var_t2_dn13 = assign86180_e131827_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign86190_e131837, assign86190_e131837_d_n0, assign86190_e131837_d_n2, assign86190_e131837_d_n4, assign86190_e131837_d_n5, assign86190_e131837_d_n6, assign86190_e131837_d_n7, assign86190_e131837_d_n8, assign86190_e131837_d_n9, assign86190_e131837_d_n10, assign86190_e131837_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2019 != 0.0)) && (locals.var_guard2020 != 0.0)) {
        let assign86190_e131835: f64 = (locals.var_t1 / locals.var_t2);
        (assign86190_e131835, (((locals.var_t1_dn0 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn0)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn2 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn2)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn4 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn4)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn5 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn5)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn6 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn6)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn7 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn7)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn8 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn8)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn9 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn9)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn10 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn10)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn13 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn13)) / (locals.var_t2 * locals.var_t2)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign86190_e131837;
        locals.var_tmf1_dn0 = assign86190_e131837_d_n0;
        locals.var_tmf1_dn2 = assign86190_e131837_d_n2;
        locals.var_tmf1_dn4 = assign86190_e131837_d_n4;
        locals.var_tmf1_dn5 = assign86190_e131837_d_n5;
        locals.var_tmf1_dn6 = assign86190_e131837_d_n6;
        locals.var_tmf1_dn7 = assign86190_e131837_d_n7;
        locals.var_tmf1_dn8 = assign86190_e131837_d_n8;
        locals.var_tmf1_dn9 = assign86190_e131837_d_n9;
        locals.var_tmf1_dn10 = assign86190_e131837_d_n10;
        locals.var_tmf1_dn13 = assign86190_e131837_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign86200_e131847, assign86200_e131847_d_n0, assign86200_e131847_d_n2, assign86200_e131847_d_n4, assign86200_e131847_d_n5, assign86200_e131847_d_n6, assign86200_e131847_d_n7, assign86200_e131847_d_n8, assign86200_e131847_d_n9, assign86200_e131847_d_n10, assign86200_e131847_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2019 != 0.0)) && (locals.var_guard2020 != 0.0)) {
        let assign86200_e131845: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign86200_e131845, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign86200_e131847;
        locals.var_tmf2_dn0 = assign86200_e131847_d_n0;
        locals.var_tmf2_dn2 = assign86200_e131847_d_n2;
        locals.var_tmf2_dn4 = assign86200_e131847_d_n4;
        locals.var_tmf2_dn5 = assign86200_e131847_d_n5;
        locals.var_tmf2_dn6 = assign86200_e131847_d_n6;
        locals.var_tmf2_dn7 = assign86200_e131847_d_n7;
        locals.var_tmf2_dn8 = assign86200_e131847_d_n8;
        locals.var_tmf2_dn9 = assign86200_e131847_d_n9;
        locals.var_tmf2_dn10 = assign86200_e131847_d_n10;
        locals.var_tmf2_dn13 = assign86200_e131847_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign86210_e131857, assign86210_e131857_d_n0, assign86210_e131857_d_n2, assign86210_e131857_d_n4, assign86210_e131857_d_n5, assign86210_e131857_d_n6, assign86210_e131857_d_n7, assign86210_e131857_d_n8, assign86210_e131857_d_n9, assign86210_e131857_d_n10, assign86210_e131857_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2019 != 0.0)) && (locals.var_guard2020 != 0.0)) {
        let assign86210_e131855: f64 = (locals.var_tmf2 * locals.var_tmf1);
        (assign86210_e131855, ((locals.var_tmf2_dn0 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn0)), ((locals.var_tmf2_dn2 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn2)), ((locals.var_tmf2_dn4 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn4)), ((locals.var_tmf2_dn5 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn5)), ((locals.var_tmf2_dn6 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn6)), ((locals.var_tmf2_dn7 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn7)), ((locals.var_tmf2_dn8 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn8)), ((locals.var_tmf2_dn9 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn9)), ((locals.var_tmf2_dn10 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn10)), ((locals.var_tmf2_dn13 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_tmf3, locals.var_tmf3_dn0, locals.var_tmf3_dn2, locals.var_tmf3_dn4, locals.var_tmf3_dn5, locals.var_tmf3_dn6, locals.var_tmf3_dn7, locals.var_tmf3_dn8, locals.var_tmf3_dn9, locals.var_tmf3_dn10, locals.var_tmf3_dn13,)
    }
};
        locals.var_tmf3 = assign86210_e131857;
        locals.var_tmf3_dn0 = assign86210_e131857_d_n0;
        locals.var_tmf3_dn2 = assign86210_e131857_d_n2;
        locals.var_tmf3_dn4 = assign86210_e131857_d_n4;
        locals.var_tmf3_dn5 = assign86210_e131857_d_n5;
        locals.var_tmf3_dn6 = assign86210_e131857_d_n6;
        locals.var_tmf3_dn7 = assign86210_e131857_d_n7;
        locals.var_tmf3_dn8 = assign86210_e131857_d_n8;
        locals.var_tmf3_dn9 = assign86210_e131857_d_n9;
        locals.var_tmf3_dn10 = assign86210_e131857_d_n10;
        locals.var_tmf3_dn13 = assign86210_e131857_d_n13;
        locals.var_tmf3_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_319(
        locals: &mut StampLocals,
    ) {
        let (assign86220_e131867, assign86220_e131867_d_n0, assign86220_e131867_d_n2, assign86220_e131867_d_n4, assign86220_e131867_d_n5, assign86220_e131867_d_n6, assign86220_e131867_d_n7, assign86220_e131867_d_n8, assign86220_e131867_d_n9, assign86220_e131867_d_n10, assign86220_e131867_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2019 != 0.0)) && (locals.var_guard2020 != 0.0)) {
        let assign86220_e131865: f64 = (locals.var_tmf2 * locals.var_tmf2);
        (assign86220_e131865, ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0)), ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2)), ((locals.var_tmf2_dn4 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn4)), ((locals.var_tmf2_dn5 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn5)), ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6)), ((locals.var_tmf2_dn7 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn7)), ((locals.var_tmf2_dn8 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn8)), ((locals.var_tmf2_dn9 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn9)), ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10)), ((locals.var_tmf2_dn13 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn13)),)
    } else {
        (locals.var_tmf4, locals.var_tmf4_dn0, locals.var_tmf4_dn2, locals.var_tmf4_dn4, locals.var_tmf4_dn5, locals.var_tmf4_dn6, locals.var_tmf4_dn7, locals.var_tmf4_dn8, locals.var_tmf4_dn9, locals.var_tmf4_dn10, locals.var_tmf4_dn13,)
    }
};
        locals.var_tmf4 = assign86220_e131867;
        locals.var_tmf4_dn0 = assign86220_e131867_d_n0;
        locals.var_tmf4_dn2 = assign86220_e131867_d_n2;
        locals.var_tmf4_dn4 = assign86220_e131867_d_n4;
        locals.var_tmf4_dn5 = assign86220_e131867_d_n5;
        locals.var_tmf4_dn6 = assign86220_e131867_d_n6;
        locals.var_tmf4_dn7 = assign86220_e131867_d_n7;
        locals.var_tmf4_dn8 = assign86220_e131867_d_n8;
        locals.var_tmf4_dn9 = assign86220_e131867_d_n9;
        locals.var_tmf4_dn10 = assign86220_e131867_d_n10;
        locals.var_tmf4_dn13 = assign86220_e131867_d_n13;
        locals.var_tmf4_rv = 0.0;

        let (assign86230_e131885, assign86230_e131885_d_n0, assign86230_e131885_d_n2, assign86230_e131885_d_n4, assign86230_e131885_d_n5, assign86230_e131885_d_n6, assign86230_e131885_d_n7, assign86230_e131885_d_n8, assign86230_e131885_d_n9, assign86230_e131885_d_n10, assign86230_e131885_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2019 != 0.0)) && (locals.var_guard2020 != 0.0)) {
        let assign86230_e131876: f64 = (1.0 + locals.var_tmf1);
        let assign86230_e131878: f64 = (assign86230_e131876 + locals.var_tmf2);
        let assign86230_e131880: f64 = (assign86230_e131878 + locals.var_tmf3);
        let assign86230_e131882: f64 = (assign86230_e131880 + locals.var_tmf4);
        let assign86230_e131883: f64 = (1.0 / assign86230_e131882);
        (assign86230_e131883, (-((((locals.var_tmf1_dn0 + locals.var_tmf2_dn0) + locals.var_tmf3_dn0) + locals.var_tmf4_dn0) / (assign86230_e131882 * assign86230_e131882))), (-((((locals.var_tmf1_dn2 + locals.var_tmf2_dn2) + locals.var_tmf3_dn2) + locals.var_tmf4_dn2) / (assign86230_e131882 * assign86230_e131882))), (-((((locals.var_tmf1_dn4 + locals.var_tmf2_dn4) + locals.var_tmf3_dn4) + locals.var_tmf4_dn4) / (assign86230_e131882 * assign86230_e131882))), (-((((locals.var_tmf1_dn5 + locals.var_tmf2_dn5) + locals.var_tmf3_dn5) + locals.var_tmf4_dn5) / (assign86230_e131882 * assign86230_e131882))), (-((((locals.var_tmf1_dn6 + locals.var_tmf2_dn6) + locals.var_tmf3_dn6) + locals.var_tmf4_dn6) / (assign86230_e131882 * assign86230_e131882))), (-((((locals.var_tmf1_dn7 + locals.var_tmf2_dn7) + locals.var_tmf3_dn7) + locals.var_tmf4_dn7) / (assign86230_e131882 * assign86230_e131882))), (-((((locals.var_tmf1_dn8 + locals.var_tmf2_dn8) + locals.var_tmf3_dn8) + locals.var_tmf4_dn8) / (assign86230_e131882 * assign86230_e131882))), (-((((locals.var_tmf1_dn9 + locals.var_tmf2_dn9) + locals.var_tmf3_dn9) + locals.var_tmf4_dn9) / (assign86230_e131882 * assign86230_e131882))), (-((((locals.var_tmf1_dn10 + locals.var_tmf2_dn10) + locals.var_tmf3_dn10) + locals.var_tmf4_dn10) / (assign86230_e131882 * assign86230_e131882))), (-((((locals.var_tmf1_dn13 + locals.var_tmf2_dn13) + locals.var_tmf3_dn13) + locals.var_tmf4_dn13) / (assign86230_e131882 * assign86230_e131882))),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign86230_e131885;
        locals.var_tmf0_dn0 = assign86230_e131885_d_n0;
        locals.var_tmf0_dn2 = assign86230_e131885_d_n2;
        locals.var_tmf0_dn4 = assign86230_e131885_d_n4;
        locals.var_tmf0_dn5 = assign86230_e131885_d_n5;
        locals.var_tmf0_dn6 = assign86230_e131885_d_n6;
        locals.var_tmf0_dn7 = assign86230_e131885_d_n7;
        locals.var_tmf0_dn8 = assign86230_e131885_d_n8;
        locals.var_tmf0_dn9 = assign86230_e131885_d_n9;
        locals.var_tmf0_dn10 = assign86230_e131885_d_n10;
        locals.var_tmf0_dn13 = assign86230_e131885_d_n13;
        locals.var_tmf0_rv = 0.0;

        let (assign86240_e131910, assign86240_e131910_d_n0, assign86240_e131910_d_n2, assign86240_e131910_d_n4, assign86240_e131910_d_n5, assign86240_e131910_d_n6, assign86240_e131910_d_n7, assign86240_e131910_d_n8, assign86240_e131910_d_n9, assign86240_e131910_d_n10, assign86240_e131910_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2019 != 0.0)) && (locals.var_guard2020 != 0.0)) {
        let assign86240_e131894: f64 = (2.0 * locals.var_tmf1);
        let assign86240_e131895: f64 = (1.0 + assign86240_e131894);
        let assign86240_e131898: f64 = (3.0 * locals.var_tmf2);
        let assign86240_e131899: f64 = (assign86240_e131895 + assign86240_e131898);
        let assign86240_e131902: f64 = (4.0 * locals.var_tmf3);
        let assign86240_e131903: f64 = (assign86240_e131899 + assign86240_e131902);
        let assign86240_e131904: f64 = (-assign86240_e131903);
        let assign86240_e131906: f64 = (assign86240_e131904 * locals.var_tmf0);
        let assign86240_e131908: f64 = (assign86240_e131906 * locals.var_tmf0);
        (assign86240_e131908, (((((-(((2.0 * locals.var_tmf1_dn0) + (3.0 * locals.var_tmf2_dn0)) + (4.0 * locals.var_tmf3_dn0))) * locals.var_tmf0) + (assign86240_e131904 * locals.var_tmf0_dn0)) * locals.var_tmf0) + (assign86240_e131906 * locals.var_tmf0_dn0)), (((((-(((2.0 * locals.var_tmf1_dn2) + (3.0 * locals.var_tmf2_dn2)) + (4.0 * locals.var_tmf3_dn2))) * locals.var_tmf0) + (assign86240_e131904 * locals.var_tmf0_dn2)) * locals.var_tmf0) + (assign86240_e131906 * locals.var_tmf0_dn2)), (((((-(((2.0 * locals.var_tmf1_dn4) + (3.0 * locals.var_tmf2_dn4)) + (4.0 * locals.var_tmf3_dn4))) * locals.var_tmf0) + (assign86240_e131904 * locals.var_tmf0_dn4)) * locals.var_tmf0) + (assign86240_e131906 * locals.var_tmf0_dn4)), (((((-(((2.0 * locals.var_tmf1_dn5) + (3.0 * locals.var_tmf2_dn5)) + (4.0 * locals.var_tmf3_dn5))) * locals.var_tmf0) + (assign86240_e131904 * locals.var_tmf0_dn5)) * locals.var_tmf0) + (assign86240_e131906 * locals.var_tmf0_dn5)), (((((-(((2.0 * locals.var_tmf1_dn6) + (3.0 * locals.var_tmf2_dn6)) + (4.0 * locals.var_tmf3_dn6))) * locals.var_tmf0) + (assign86240_e131904 * locals.var_tmf0_dn6)) * locals.var_tmf0) + (assign86240_e131906 * locals.var_tmf0_dn6)), (((((-(((2.0 * locals.var_tmf1_dn7) + (3.0 * locals.var_tmf2_dn7)) + (4.0 * locals.var_tmf3_dn7))) * locals.var_tmf0) + (assign86240_e131904 * locals.var_tmf0_dn7)) * locals.var_tmf0) + (assign86240_e131906 * locals.var_tmf0_dn7)), (((((-(((2.0 * locals.var_tmf1_dn8) + (3.0 * locals.var_tmf2_dn8)) + (4.0 * locals.var_tmf3_dn8))) * locals.var_tmf0) + (assign86240_e131904 * locals.var_tmf0_dn8)) * locals.var_tmf0) + (assign86240_e131906 * locals.var_tmf0_dn8)), (((((-(((2.0 * locals.var_tmf1_dn9) + (3.0 * locals.var_tmf2_dn9)) + (4.0 * locals.var_tmf3_dn9))) * locals.var_tmf0) + (assign86240_e131904 * locals.var_tmf0_dn9)) * locals.var_tmf0) + (assign86240_e131906 * locals.var_tmf0_dn9)), (((((-(((2.0 * locals.var_tmf1_dn10) + (3.0 * locals.var_tmf2_dn10)) + (4.0 * locals.var_tmf3_dn10))) * locals.var_tmf0) + (assign86240_e131904 * locals.var_tmf0_dn10)) * locals.var_tmf0) + (assign86240_e131906 * locals.var_tmf0_dn10)), (((((-(((2.0 * locals.var_tmf1_dn13) + (3.0 * locals.var_tmf2_dn13)) + (4.0 * locals.var_tmf3_dn13))) * locals.var_tmf0) + (assign86240_e131904 * locals.var_tmf0_dn13)) * locals.var_tmf0) + (assign86240_e131906 * locals.var_tmf0_dn13)),)
    } else {
        (locals.var_t11, locals.var_t11_dn0, locals.var_t11_dn2, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn13,)
    }
};
        locals.var_t11 = assign86240_e131910;
        locals.var_t11_dn0 = assign86240_e131910_d_n0;
        locals.var_t11_dn2 = assign86240_e131910_d_n2;
        locals.var_t11_dn4 = assign86240_e131910_d_n4;
        locals.var_t11_dn5 = assign86240_e131910_d_n5;
        locals.var_t11_dn6 = assign86240_e131910_d_n6;
        locals.var_t11_dn7 = assign86240_e131910_d_n7;
        locals.var_t11_dn8 = assign86240_e131910_d_n8;
        locals.var_t11_dn9 = assign86240_e131910_d_n9;
        locals.var_t11_dn10 = assign86240_e131910_d_n10;
        locals.var_t11_dn13 = assign86240_e131910_d_n13;
        locals.var_t11_rv = 0.0;

        let (assign86250_e131922, assign86250_e131922_d_n0, assign86250_e131922_d_n2, assign86250_e131922_d_n4, assign86250_e131922_d_n5, assign86250_e131922_d_n6, assign86250_e131922_d_n7, assign86250_e131922_d_n8, assign86250_e131922_d_n9, assign86250_e131922_d_n10, assign86250_e131922_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2019 != 0.0)) && (locals.var_guard2020 != 0.0)) {
        let assign86250_e131919: f64 = (1.0 - locals.var_tmf0);
        let assign86250_e131920: f64 = (locals.var_t2 * assign86250_e131919);
        (assign86250_e131920, ((locals.var_t2_dn0 * assign86250_e131919) + (locals.var_t2 * (-locals.var_tmf0_dn0))), ((locals.var_t2_dn2 * assign86250_e131919) + (locals.var_t2 * (-locals.var_tmf0_dn2))), ((locals.var_t2_dn4 * assign86250_e131919) + (locals.var_t2 * (-locals.var_tmf0_dn4))), ((locals.var_t2_dn5 * assign86250_e131919) + (locals.var_t2 * (-locals.var_tmf0_dn5))), ((locals.var_t2_dn6 * assign86250_e131919) + (locals.var_t2 * (-locals.var_tmf0_dn6))), ((locals.var_t2_dn7 * assign86250_e131919) + (locals.var_t2 * (-locals.var_tmf0_dn7))), ((locals.var_t2_dn8 * assign86250_e131919) + (locals.var_t2 * (-locals.var_tmf0_dn8))), ((locals.var_t2_dn9 * assign86250_e131919) + (locals.var_t2 * (-locals.var_tmf0_dn9))), ((locals.var_t2_dn10 * assign86250_e131919) + (locals.var_t2 * (-locals.var_tmf0_dn10))), ((locals.var_t2_dn13 * assign86250_e131919) + (locals.var_t2 * (-locals.var_tmf0_dn13))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn13,)
    }
};
        locals.var_ty = assign86250_e131922;
        locals.var_ty_dn0 = assign86250_e131922_d_n0;
        locals.var_ty_dn2 = assign86250_e131922_d_n2;
        locals.var_ty_dn4 = assign86250_e131922_d_n4;
        locals.var_ty_dn5 = assign86250_e131922_d_n5;
        locals.var_ty_dn6 = assign86250_e131922_d_n6;
        locals.var_ty_dn7 = assign86250_e131922_d_n7;
        locals.var_ty_dn8 = assign86250_e131922_d_n8;
        locals.var_ty_dn9 = assign86250_e131922_d_n9;
        locals.var_ty_dn10 = assign86250_e131922_d_n10;
        locals.var_ty_dn13 = assign86250_e131922_d_n13;
        locals.var_ty_rv = 0.0;

        let (assign86260_e131936, assign86260_e131936_d_n0, assign86260_e131936_d_n2, assign86260_e131936_d_n4, assign86260_e131936_d_n5, assign86260_e131936_d_n6, assign86260_e131936_d_n7, assign86260_e131936_d_n8, assign86260_e131936_d_n9, assign86260_e131936_d_n10, assign86260_e131936_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2019 != 0.0)) && (locals.var_guard2020 != 0.0)) {
        let assign86260_e131930: f64 = (1.0 - locals.var_tmf0);
        let assign86260_e131933: f64 = (locals.var_tmf1 * locals.var_t11);
        let assign86260_e131934: f64 = (assign86260_e131930 + assign86260_e131933);
        (assign86260_e131934, ((-locals.var_tmf0_dn0) + ((locals.var_tmf1_dn0 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn0))), ((-locals.var_tmf0_dn2) + ((locals.var_tmf1_dn2 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn2))), ((-locals.var_tmf0_dn4) + ((locals.var_tmf1_dn4 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn4))), ((-locals.var_tmf0_dn5) + ((locals.var_tmf1_dn5 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn5))), ((-locals.var_tmf0_dn6) + ((locals.var_tmf1_dn6 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn6))), ((-locals.var_tmf0_dn7) + ((locals.var_tmf1_dn7 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn7))), ((-locals.var_tmf0_dn8) + ((locals.var_tmf1_dn8 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn8))), ((-locals.var_tmf0_dn9) + ((locals.var_tmf1_dn9 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn9))), ((-locals.var_tmf0_dn10) + ((locals.var_tmf1_dn10 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn10))), ((-locals.var_tmf0_dn13) + ((locals.var_tmf1_dn13 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn13))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign86260_e131936;
        locals.var_t0_dn0 = assign86260_e131936_d_n0;
        locals.var_t0_dn2 = assign86260_e131936_d_n2;
        locals.var_t0_dn4 = assign86260_e131936_d_n4;
        locals.var_t0_dn5 = assign86260_e131936_d_n5;
        locals.var_t0_dn6 = assign86260_e131936_d_n6;
        locals.var_t0_dn7 = assign86260_e131936_d_n7;
        locals.var_t0_dn8 = assign86260_e131936_d_n8;
        locals.var_t0_dn9 = assign86260_e131936_d_n9;
        locals.var_t0_dn10 = assign86260_e131936_d_n10;
        locals.var_t0_dn13 = assign86260_e131936_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign86270_e131945, assign86270_e131945_d_n0, assign86270_e131945_d_n2, assign86270_e131945_d_n4, assign86270_e131945_d_n5, assign86270_e131945_d_n6, assign86270_e131945_d_n7, assign86270_e131945_d_n8, assign86270_e131945_d_n9, assign86270_e131945_d_n10, assign86270_e131945_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2019 != 0.0)) && (locals.var_guard2020 != 0.0)) {
        let assign86270_e131943: f64 = (-locals.var_t11);
        (assign86270_e131943, (-locals.var_t11_dn0), (-locals.var_t11_dn2), (-locals.var_t11_dn4), (-locals.var_t11_dn5), (-locals.var_t11_dn6), (-locals.var_t11_dn7), (-locals.var_t11_dn8), (-locals.var_t11_dn9), (-locals.var_t11_dn10), (-locals.var_t11_dn13),)
    } else {
        (locals.var_t11, locals.var_t11_dn0, locals.var_t11_dn2, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn13,)
    }
};
        locals.var_t11 = assign86270_e131945;
        locals.var_t11_dn0 = assign86270_e131945_d_n0;
        locals.var_t11_dn2 = assign86270_e131945_d_n2;
        locals.var_t11_dn4 = assign86270_e131945_d_n4;
        locals.var_t11_dn5 = assign86270_e131945_d_n5;
        locals.var_t11_dn6 = assign86270_e131945_d_n6;
        locals.var_t11_dn7 = assign86270_e131945_d_n7;
        locals.var_t11_dn8 = assign86270_e131945_d_n8;
        locals.var_t11_dn9 = assign86270_e131945_d_n9;
        locals.var_t11_dn10 = assign86270_e131945_d_n10;
        locals.var_t11_dn13 = assign86270_e131945_d_n13;
        locals.var_t11_rv = 0.0;

        let (assign86280_e131955, assign86280_e131955_d_n0, assign86280_e131955_d_n2, assign86280_e131955_d_n4, assign86280_e131955_d_n5, assign86280_e131955_d_n6, assign86280_e131955_d_n7, assign86280_e131955_d_n8, assign86280_e131955_d_n9, assign86280_e131955_d_n10, assign86280_e131955_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2019 != 0.0)) && (locals.var_guard2020 != 0.0)) {
        let assign86280_e131953: f64 = (locals.var_vbs_bnd_over__blk2009 + locals.var_ty);
        (assign86280_e131953, (locals.var_vbs_bnd_over__blk2009_dn0 + locals.var_ty_dn0), (locals.var_vbs_bnd_over__blk2009_dn2 + locals.var_ty_dn2), (locals.var_vbs_bnd_over__blk2009_dn4 + locals.var_ty_dn4), (locals.var_vbs_bnd_over__blk2009_dn5 + locals.var_ty_dn5), (locals.var_vbs_bnd_over__blk2009_dn6 + locals.var_ty_dn6), (locals.var_vbs_bnd_over__blk2009_dn7 + locals.var_ty_dn7), (locals.var_vbs_bnd_over__blk2009_dn8 + locals.var_ty_dn8), (locals.var_vbs_bnd_over__blk2009_dn9 + locals.var_ty_dn9), (locals.var_vbs_bnd_over__blk2009_dn10 + locals.var_ty_dn10), (locals.var_vbs_bnd_over__blk2009_dn13 + locals.var_ty_dn13),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn13,)
    }
};
        locals.var_t10 = assign86280_e131955;
        locals.var_t10_dn0 = assign86280_e131955_d_n0;
        locals.var_t10_dn2 = assign86280_e131955_d_n2;
        locals.var_t10_dn4 = assign86280_e131955_d_n4;
        locals.var_t10_dn5 = assign86280_e131955_d_n5;
        locals.var_t10_dn6 = assign86280_e131955_d_n6;
        locals.var_t10_dn7 = assign86280_e131955_d_n7;
        locals.var_t10_dn8 = assign86280_e131955_d_n8;
        locals.var_t10_dn9 = assign86280_e131955_d_n9;
        locals.var_t10_dn10 = assign86280_e131955_d_n10;
        locals.var_t10_dn13 = assign86280_e131955_d_n13;
        locals.var_t10_rv = 0.0;

        let (assign86290_e131964, assign86290_e131964_d_n0, assign86290_e131964_d_n2, assign86290_e131964_d_n4, assign86290_e131964_d_n5, assign86290_e131964_d_n6, assign86290_e131964_d_n7, assign86290_e131964_d_n8, assign86290_e131964_d_n9, assign86290_e131964_d_n10, assign86290_e131964_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2019 != 0.0)) && (locals.var_guard2020 == 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn13,)
    }
};
        locals.var_t10 = assign86290_e131964;
        locals.var_t10_dn0 = assign86290_e131964_d_n0;
        locals.var_t10_dn2 = assign86290_e131964_d_n2;
        locals.var_t10_dn4 = assign86290_e131964_d_n4;
        locals.var_t10_dn5 = assign86290_e131964_d_n5;
        locals.var_t10_dn6 = assign86290_e131964_d_n6;
        locals.var_t10_dn7 = assign86290_e131964_d_n7;
        locals.var_t10_dn8 = assign86290_e131964_d_n8;
        locals.var_t10_dn9 = assign86290_e131964_d_n9;
        locals.var_t10_dn10 = assign86290_e131964_d_n10;
        locals.var_t10_dn13 = assign86290_e131964_d_n13;
        locals.var_t10_rv = 0.0;

        let (assign86300_e131971, assign86300_e131971_d_n0, assign86300_e131971_d_n2, assign86300_e131971_d_n4, assign86300_e131971_d_n5, assign86300_e131971_d_n6, assign86300_e131971_d_n7, assign86300_e131971_d_n8, assign86300_e131971_d_n9, assign86300_e131971_d_n10, assign86300_e131971_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2019 != 0.0)) {
        let assign86300_e131969: f64 = (-locals.var_t10);
        (assign86300_e131969, (-locals.var_t10_dn0), (-locals.var_t10_dn2), (-locals.var_t10_dn4), (-locals.var_t10_dn5), (-locals.var_t10_dn6), (-locals.var_t10_dn7), (-locals.var_t10_dn8), (-locals.var_t10_dn9), (-locals.var_t10_dn10), (-locals.var_t10_dn13),)
    } else {
        (locals.var_vxbgmtcl, locals.var_vxbgmtcl_dn0, locals.var_vxbgmtcl_dn2, locals.var_vxbgmtcl_dn4, locals.var_vxbgmtcl_dn5, locals.var_vxbgmtcl_dn6, locals.var_vxbgmtcl_dn7, locals.var_vxbgmtcl_dn8, locals.var_vxbgmtcl_dn9, locals.var_vxbgmtcl_dn10, locals.var_vxbgmtcl_dn13,)
    }
};
        locals.var_vxbgmtcl = assign86300_e131971;
        locals.var_vxbgmtcl_dn0 = assign86300_e131971_d_n0;
        locals.var_vxbgmtcl_dn2 = assign86300_e131971_d_n2;
        locals.var_vxbgmtcl_dn4 = assign86300_e131971_d_n4;
        locals.var_vxbgmtcl_dn5 = assign86300_e131971_d_n5;
        locals.var_vxbgmtcl_dn6 = assign86300_e131971_d_n6;
        locals.var_vxbgmtcl_dn7 = assign86300_e131971_d_n7;
        locals.var_vxbgmtcl_dn8 = assign86300_e131971_d_n8;
        locals.var_vxbgmtcl_dn9 = assign86300_e131971_d_n9;
        locals.var_vxbgmtcl_dn10 = assign86300_e131971_d_n10;
        locals.var_vxbgmtcl_dn13 = assign86300_e131971_d_n13;
        locals.var_vxbgmtcl_rv = 0.0;

        let (assign86310_e131978, assign86310_e131978_d_n0, assign86310_e131978_d_n2, assign86310_e131978_d_n4, assign86310_e131978_d_n5, assign86310_e131978_d_n6, assign86310_e131978_d_n7, assign86310_e131978_d_n8, assign86310_e131978_d_n9, assign86310_e131978_d_n10, assign86310_e131978_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2019 == 0.0)) {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn13,)
    } else {
        (locals.var_vxbgmtcl, locals.var_vxbgmtcl_dn0, locals.var_vxbgmtcl_dn2, locals.var_vxbgmtcl_dn4, locals.var_vxbgmtcl_dn5, locals.var_vxbgmtcl_dn6, locals.var_vxbgmtcl_dn7, locals.var_vxbgmtcl_dn8, locals.var_vxbgmtcl_dn9, locals.var_vxbgmtcl_dn10, locals.var_vxbgmtcl_dn13,)
    }
};
        locals.var_vxbgmtcl = assign86310_e131978;
        locals.var_vxbgmtcl_dn0 = assign86310_e131978_d_n0;
        locals.var_vxbgmtcl_dn2 = assign86310_e131978_d_n2;
        locals.var_vxbgmtcl_dn4 = assign86310_e131978_d_n4;
        locals.var_vxbgmtcl_dn5 = assign86310_e131978_d_n5;
        locals.var_vxbgmtcl_dn6 = assign86310_e131978_d_n6;
        locals.var_vxbgmtcl_dn7 = assign86310_e131978_d_n7;
        locals.var_vxbgmtcl_dn8 = assign86310_e131978_d_n8;
        locals.var_vxbgmtcl_dn9 = assign86310_e131978_d_n9;
        locals.var_vxbgmtcl_dn10 = assign86310_e131978_d_n10;
        locals.var_vxbgmtcl_dn13 = assign86310_e131978_d_n13;
        locals.var_vxbgmtcl_rv = 0.0;

        let (assign86320_e131984, assign86320_e131984_d_n0, assign86320_e131984_d_n2, assign86320_e131984_d_n4, assign86320_e131984_d_n5, assign86320_e131984_d_n6, assign86320_e131984_d_n7, assign86320_e131984_d_n8, assign86320_e131984_d_n9, assign86320_e131984_d_n10, assign86320_e131984_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign86320_e131982: f64 = (locals.var_cnst0over_func / locals.var_cox0_func);
        (assign86320_e131982, (locals.var_cnst0over_func_dn0 / locals.var_cox0_func), (locals.var_cnst0over_func_dn2 / locals.var_cox0_func), (locals.var_cnst0over_func_dn4 / locals.var_cox0_func), (locals.var_cnst0over_func_dn5 / locals.var_cox0_func), (locals.var_cnst0over_func_dn6 / locals.var_cox0_func), (locals.var_cnst0over_func_dn7 / locals.var_cox0_func), (locals.var_cnst0over_func_dn8 / locals.var_cox0_func), (locals.var_cnst0over_func_dn9 / locals.var_cox0_func), (locals.var_cnst0over_func_dn10 / locals.var_cox0_func), (locals.var_cnst0over_func_dn13 / locals.var_cox0_func),)
    } else {
        (locals.var_fac1, locals.var_fac1_dn0, locals.var_fac1_dn2, locals.var_fac1_dn4, locals.var_fac1_dn5, locals.var_fac1_dn6, locals.var_fac1_dn7, locals.var_fac1_dn8, locals.var_fac1_dn9, locals.var_fac1_dn10, locals.var_fac1_dn13,)
    }
};
        locals.var_fac1 = assign86320_e131984;
        locals.var_fac1_dn0 = assign86320_e131984_d_n0;
        locals.var_fac1_dn2 = assign86320_e131984_d_n2;
        locals.var_fac1_dn4 = assign86320_e131984_d_n4;
        locals.var_fac1_dn5 = assign86320_e131984_d_n5;
        locals.var_fac1_dn6 = assign86320_e131984_d_n6;
        locals.var_fac1_dn7 = assign86320_e131984_d_n7;
        locals.var_fac1_dn8 = assign86320_e131984_d_n8;
        locals.var_fac1_dn9 = assign86320_e131984_d_n9;
        locals.var_fac1_dn10 = assign86320_e131984_d_n10;
        locals.var_fac1_dn13 = assign86320_e131984_d_n13;
        locals.var_fac1_rv = 0.0;

        let (assign86330_e131990, assign86330_e131990_d_n0, assign86330_e131990_d_n2, assign86330_e131990_d_n4, assign86330_e131990_d_n5, assign86330_e131990_d_n6, assign86330_e131990_d_n7, assign86330_e131990_d_n8, assign86330_e131990_d_n9, assign86330_e131990_d_n10, assign86330_e131990_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign86330_e131988: f64 = (locals.var_fac1 * locals.var_fac1);
        (assign86330_e131988, ((locals.var_fac1_dn0 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn0)), ((locals.var_fac1_dn2 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn2)), ((locals.var_fac1_dn4 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn4)), ((locals.var_fac1_dn5 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn5)), ((locals.var_fac1_dn6 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn6)), ((locals.var_fac1_dn7 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn7)), ((locals.var_fac1_dn8 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn8)), ((locals.var_fac1_dn9 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn9)), ((locals.var_fac1_dn10 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn10)), ((locals.var_fac1_dn13 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn13)),)
    } else {
        (locals.var_fac1p2, locals.var_fac1p2_dn0, locals.var_fac1p2_dn2, locals.var_fac1p2_dn4, locals.var_fac1p2_dn5, locals.var_fac1p2_dn6, locals.var_fac1p2_dn7, locals.var_fac1p2_dn8, locals.var_fac1p2_dn9, locals.var_fac1p2_dn10, locals.var_fac1p2_dn13,)
    }
};
        locals.var_fac1p2 = assign86330_e131990;
        locals.var_fac1p2_dn0 = assign86330_e131990_d_n0;
        locals.var_fac1p2_dn2 = assign86330_e131990_d_n2;
        locals.var_fac1p2_dn4 = assign86330_e131990_d_n4;
        locals.var_fac1p2_dn5 = assign86330_e131990_d_n5;
        locals.var_fac1p2_dn6 = assign86330_e131990_d_n6;
        locals.var_fac1p2_dn7 = assign86330_e131990_d_n7;
        locals.var_fac1p2_dn8 = assign86330_e131990_d_n8;
        locals.var_fac1p2_dn9 = assign86330_e131990_d_n9;
        locals.var_fac1p2_dn10 = assign86330_e131990_d_n10;
        locals.var_fac1p2_dn13 = assign86330_e131990_d_n13;
        locals.var_fac1p2_rv = 0.0;

        let (assign86340_e131997, assign86340_e131997_d_n2, assign86340_e131997_d_n6, assign86340_e131997_d_n7, assign86340_e131997_d_n8,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign86340_e131993: f64 = (-locals.var_vgbgmt);
        let assign86340_e131995: f64 = (assign86340_e131993 + locals.var_uc_vfbover);
        (assign86340_e131995, (-locals.var_vgbgmt_dn2), (-locals.var_vgbgmt_dn6), (-locals.var_vgbgmt_dn7), (-locals.var_vgbgmt_dn8),)
    } else {
        (locals.var_vgpld, locals.var_vgpld_dn2, locals.var_vgpld_dn6, locals.var_vgpld_dn7, locals.var_vgpld_dn8,)
    }
};
        locals.var_vgpld = assign86340_e131997;
        locals.var_vgpld_dn2 = assign86340_e131997_d_n2;
        locals.var_vgpld_dn6 = assign86340_e131997_d_n6;
        locals.var_vgpld_dn7 = assign86340_e131997_d_n7;
        locals.var_vgpld_dn8 = assign86340_e131997_d_n8;
        locals.var_vgpld_rv = 0.0;

        let (assign86350_e132006, assign86350_e132006_d_n0, assign86350_e132006_d_n2, assign86350_e132006_d_n4, assign86350_e132006_d_n5, assign86350_e132006_d_n6, assign86350_e132006_d_n7, assign86350_e132006_d_n8, assign86350_e132006_d_n9, assign86350_e132006_d_n10, assign86350_e132006_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign86350_e132000: f64 = (-locals.var_vxbgmtcl);
        let assign86350_e132003: f64 = (10.0 * 2.220446049250313e-16);
        let assign86350_e132004: f64 = (assign86350_e132000 + assign86350_e132003);
        (assign86350_e132004, (-locals.var_vxbgmtcl_dn0), (-locals.var_vxbgmtcl_dn2), (-locals.var_vxbgmtcl_dn4), (-locals.var_vxbgmtcl_dn5), (-locals.var_vxbgmtcl_dn6), (-locals.var_vxbgmtcl_dn7), (-locals.var_vxbgmtcl_dn8), (-locals.var_vxbgmtcl_dn9), (-locals.var_vxbgmtcl_dn10), (-locals.var_vxbgmtcl_dn13),)
    } else {
        (locals.var_vgb_fb_ld, locals.var_vgb_fb_ld_dn0, locals.var_vgb_fb_ld_dn2, locals.var_vgb_fb_ld_dn4, locals.var_vgb_fb_ld_dn5, locals.var_vgb_fb_ld_dn6, locals.var_vgb_fb_ld_dn7, locals.var_vgb_fb_ld_dn8, locals.var_vgb_fb_ld_dn9, locals.var_vgb_fb_ld_dn10, locals.var_vgb_fb_ld_dn13,)
    }
};
        locals.var_vgb_fb_ld = assign86350_e132006;
        locals.var_vgb_fb_ld_dn0 = assign86350_e132006_d_n0;
        locals.var_vgb_fb_ld_dn2 = assign86350_e132006_d_n2;
        locals.var_vgb_fb_ld_dn4 = assign86350_e132006_d_n4;
        locals.var_vgb_fb_ld_dn5 = assign86350_e132006_d_n5;
        locals.var_vgb_fb_ld_dn6 = assign86350_e132006_d_n6;
        locals.var_vgb_fb_ld_dn7 = assign86350_e132006_d_n7;
        locals.var_vgb_fb_ld_dn8 = assign86350_e132006_d_n8;
        locals.var_vgb_fb_ld_dn9 = assign86350_e132006_d_n9;
        locals.var_vgb_fb_ld_dn10 = assign86350_e132006_d_n10;
        locals.var_vgb_fb_ld_dn13 = assign86350_e132006_d_n13;
        locals.var_vgb_fb_ld_rv = 0.0;

        let (assign86360_e132010, assign86360_e132010_d_n0, assign86360_e132010_d_n2, assign86360_e132010_d_n4, assign86360_e132010_d_n5, assign86360_e132010_d_n6, assign86360_e132010_d_n7, assign86360_e132010_d_n8, assign86360_e132010_d_n9, assign86360_e132010_d_n10, assign86360_e132010_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_dep_ld__blk2003, locals.var_q_dep_ld__blk2003_dn0, locals.var_q_dep_ld__blk2003_dn2, locals.var_q_dep_ld__blk2003_dn4, locals.var_q_dep_ld__blk2003_dn5, locals.var_q_dep_ld__blk2003_dn6, locals.var_q_dep_ld__blk2003_dn7, locals.var_q_dep_ld__blk2003_dn8, locals.var_q_dep_ld__blk2003_dn9, locals.var_q_dep_ld__blk2003_dn10, locals.var_q_dep_ld__blk2003_dn13,)
    }
};
        locals.var_q_dep_ld__blk2003 = assign86360_e132010;
        locals.var_q_dep_ld__blk2003_dn0 = assign86360_e132010_d_n0;
        locals.var_q_dep_ld__blk2003_dn2 = assign86360_e132010_d_n2;
        locals.var_q_dep_ld__blk2003_dn4 = assign86360_e132010_d_n4;
        locals.var_q_dep_ld__blk2003_dn5 = assign86360_e132010_d_n5;
        locals.var_q_dep_ld__blk2003_dn6 = assign86360_e132010_d_n6;
        locals.var_q_dep_ld__blk2003_dn7 = assign86360_e132010_d_n7;
        locals.var_q_dep_ld__blk2003_dn8 = assign86360_e132010_d_n8;
        locals.var_q_dep_ld__blk2003_dn9 = assign86360_e132010_d_n9;
        locals.var_q_dep_ld__blk2003_dn10 = assign86360_e132010_d_n10;
        locals.var_q_dep_ld__blk2003_dn13 = assign86360_e132010_d_n13;
        locals.var_q_dep_ld__blk2003_rv = 0.0;

        let (assign86370_e132016,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign86370_e132014: f64 = (1.6021918e-19 * locals.var_nover_func);
        (assign86370_e132014,)
    } else {
        (locals.var_q_nsubld__blk2004,)
    }
};
        locals.var_q_nsubld__blk2004 = assign86370_e132016;
        locals.var_q_nsubld__blk2004_rv = 0.0;

        let (assign86380_e132022, assign86380_e132022_d_n0, assign86380_e132022_d_n2, assign86380_e132022_d_n4, assign86380_e132022_d_n5, assign86380_e132022_d_n6, assign86380_e132022_d_n7, assign86380_e132022_d_n8, assign86380_e132022_d_n9, assign86380_e132022_d_n10, assign86380_e132022_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign86380_e132020: f64 = (locals.var_nin / locals.var_nover_func);
        (assign86380_e132020, (locals.var_nin_dn0 / locals.var_nover_func), (locals.var_nin_dn2 / locals.var_nover_func), (locals.var_nin_dn4 / locals.var_nover_func), (locals.var_nin_dn5 / locals.var_nover_func), (locals.var_nin_dn6 / locals.var_nover_func), (locals.var_nin_dn7 / locals.var_nover_func), (locals.var_nin_dn8 / locals.var_nover_func), (locals.var_nin_dn9 / locals.var_nover_func), (locals.var_nin_dn10 / locals.var_nover_func), (locals.var_nin_dn13 / locals.var_nover_func),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign86380_e132022;
        locals.var_t0_dn0 = assign86380_e132022_d_n0;
        locals.var_t0_dn2 = assign86380_e132022_d_n2;
        locals.var_t0_dn4 = assign86380_e132022_d_n4;
        locals.var_t0_dn5 = assign86380_e132022_d_n5;
        locals.var_t0_dn6 = assign86380_e132022_d_n6;
        locals.var_t0_dn7 = assign86380_e132022_d_n7;
        locals.var_t0_dn8 = assign86380_e132022_d_n8;
        locals.var_t0_dn9 = assign86380_e132022_d_n9;
        locals.var_t0_dn10 = assign86380_e132022_d_n10;
        locals.var_t0_dn13 = assign86380_e132022_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign86390_e132028, assign86390_e132028_d_n0, assign86390_e132028_d_n2, assign86390_e132028_d_n4, assign86390_e132028_d_n5, assign86390_e132028_d_n6, assign86390_e132028_d_n7, assign86390_e132028_d_n8, assign86390_e132028_d_n9, assign86390_e132028_d_n10, assign86390_e132028_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign86390_e132026: f64 = (locals.var_t0 * locals.var_t0);
        (assign86390_e132026, ((locals.var_t0_dn0 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn0)), ((locals.var_t0_dn2 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn2)), ((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)), ((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)), ((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)), ((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)), ((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)), ((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)), ((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)), ((locals.var_t0_dn13 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn13)),)
    } else {
        (locals.var_cnst1over, locals.var_cnst1over_dn0, locals.var_cnst1over_dn2, locals.var_cnst1over_dn4, locals.var_cnst1over_dn5, locals.var_cnst1over_dn6, locals.var_cnst1over_dn7, locals.var_cnst1over_dn8, locals.var_cnst1over_dn9, locals.var_cnst1over_dn10, locals.var_cnst1over_dn13,)
    }
};
        locals.var_cnst1over = assign86390_e132028;
        locals.var_cnst1over_dn0 = assign86390_e132028_d_n0;
        locals.var_cnst1over_dn2 = assign86390_e132028_d_n2;
        locals.var_cnst1over_dn4 = assign86390_e132028_d_n4;
        locals.var_cnst1over_dn5 = assign86390_e132028_d_n5;
        locals.var_cnst1over_dn6 = assign86390_e132028_d_n6;
        locals.var_cnst1over_dn7 = assign86390_e132028_d_n7;
        locals.var_cnst1over_dn8 = assign86390_e132028_d_n8;
        locals.var_cnst1over_dn9 = assign86390_e132028_d_n9;
        locals.var_cnst1over_dn10 = assign86390_e132028_d_n10;
        locals.var_cnst1over_dn13 = assign86390_e132028_d_n13;
        locals.var_cnst1over_rv = 0.0;

        let assign86400_e132031: f64 = (-locals.var_vxbgmtcl);
        let assign86400_e132032: f64 = (locals.var_beta * assign86400_e132031);
        let assign86400_e132034: f64 = if assign86400_e132032 >= 500.0 { 1.0 } else { 0.0 };
        locals.var_guard2021 = assign86400_e132034;
        locals.var_guard2021_rv = 0.0;

        let (assign86410_e132049, assign86410_e132049_d_n0, assign86410_e132049_d_n2, assign86410_e132049_d_n4, assign86410_e132049_d_n5, assign86410_e132049_d_n6, assign86410_e132049_d_n7, assign86410_e132049_d_n8, assign86410_e132049_d_n9, assign86410_e132049_d_n10, assign86410_e132049_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2021 != 0.0)) {
        let assign86410_e132042: f64 = (-locals.var_vxbgmtcl);
        let assign86410_e132043: f64 = (locals.var_beta * assign86410_e132042);
        let assign86410_e132044: f64 = (1.0 + assign86410_e132043);
        let assign86410_e132046: f64 = (assign86410_e132044 - 500.0);
        let assign86410_e132047: f64 = (1.403592217853e217 * assign86410_e132046);
        (assign86410_e132047, (1.403592217853e217 * ((locals.var_beta_dn0 * assign86410_e132042) + (locals.var_beta * (-locals.var_vxbgmtcl_dn0)))), (1.403592217853e217 * ((locals.var_beta_dn2 * assign86410_e132042) + (locals.var_beta * (-locals.var_vxbgmtcl_dn2)))), (1.403592217853e217 * ((locals.var_beta_dn4 * assign86410_e132042) + (locals.var_beta * (-locals.var_vxbgmtcl_dn4)))), (1.403592217853e217 * ((locals.var_beta_dn5 * assign86410_e132042) + (locals.var_beta * (-locals.var_vxbgmtcl_dn5)))), (1.403592217853e217 * ((locals.var_beta_dn6 * assign86410_e132042) + (locals.var_beta * (-locals.var_vxbgmtcl_dn6)))), (1.403592217853e217 * ((locals.var_beta_dn7 * assign86410_e132042) + (locals.var_beta * (-locals.var_vxbgmtcl_dn7)))), (1.403592217853e217 * ((locals.var_beta_dn8 * assign86410_e132042) + (locals.var_beta * (-locals.var_vxbgmtcl_dn8)))), (1.403592217853e217 * ((locals.var_beta_dn9 * assign86410_e132042) + (locals.var_beta * (-locals.var_vxbgmtcl_dn9)))), (1.403592217853e217 * ((locals.var_beta_dn10 * assign86410_e132042) + (locals.var_beta * (-locals.var_vxbgmtcl_dn10)))), (1.403592217853e217 * ((locals.var_beta_dn13 * assign86410_e132042) + (locals.var_beta * (-locals.var_vxbgmtcl_dn13)))),)
    } else {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn13,)
    }
};
        locals.var_exp_bvbs = assign86410_e132049;
        locals.var_exp_bvbs_dn0 = assign86410_e132049_d_n0;
        locals.var_exp_bvbs_dn2 = assign86410_e132049_d_n2;
        locals.var_exp_bvbs_dn4 = assign86410_e132049_d_n4;
        locals.var_exp_bvbs_dn5 = assign86410_e132049_d_n5;
        locals.var_exp_bvbs_dn6 = assign86410_e132049_d_n6;
        locals.var_exp_bvbs_dn7 = assign86410_e132049_d_n7;
        locals.var_exp_bvbs_dn8 = assign86410_e132049_d_n8;
        locals.var_exp_bvbs_dn9 = assign86410_e132049_d_n9;
        locals.var_exp_bvbs_dn10 = assign86410_e132049_d_n10;
        locals.var_exp_bvbs_dn13 = assign86410_e132049_d_n13;
        locals.var_exp_bvbs_rv = 0.0;

        let (assign86420_e132055, assign86420_e132055_d_n0, assign86420_e132055_d_n2, assign86420_e132055_d_n4, assign86420_e132055_d_n5, assign86420_e132055_d_n6, assign86420_e132055_d_n7, assign86420_e132055_d_n8, assign86420_e132055_d_n9, assign86420_e132055_d_n10, assign86420_e132055_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2021 != 0.0)) {
        (1.403592217853e217, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign86420_e132055;
        locals.var_t0_dn0 = assign86420_e132055_d_n0;
        locals.var_t0_dn2 = assign86420_e132055_d_n2;
        locals.var_t0_dn4 = assign86420_e132055_d_n4;
        locals.var_t0_dn5 = assign86420_e132055_d_n5;
        locals.var_t0_dn6 = assign86420_e132055_d_n6;
        locals.var_t0_dn7 = assign86420_e132055_d_n7;
        locals.var_t0_dn8 = assign86420_e132055_d_n8;
        locals.var_t0_dn9 = assign86420_e132055_d_n9;
        locals.var_t0_dn10 = assign86420_e132055_d_n10;
        locals.var_t0_dn13 = assign86420_e132055_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign86430_e132065, assign86430_e132065_d_n0, assign86430_e132065_d_n2, assign86430_e132065_d_n4, assign86430_e132065_d_n5, assign86430_e132065_d_n6, assign86430_e132065_d_n7, assign86430_e132065_d_n8, assign86430_e132065_d_n9, assign86430_e132065_d_n10, assign86430_e132065_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2021 == 0.0)) {
        let assign86430_e132062: f64 = (-locals.var_vxbgmtcl);
        let assign86430_e132063: f64 = (locals.var_beta * assign86430_e132062);
        (assign86430_e132063, ((locals.var_beta_dn0 * assign86430_e132062) + (locals.var_beta * (-locals.var_vxbgmtcl_dn0))), ((locals.var_beta_dn2 * assign86430_e132062) + (locals.var_beta * (-locals.var_vxbgmtcl_dn2))), ((locals.var_beta_dn4 * assign86430_e132062) + (locals.var_beta * (-locals.var_vxbgmtcl_dn4))), ((locals.var_beta_dn5 * assign86430_e132062) + (locals.var_beta * (-locals.var_vxbgmtcl_dn5))), ((locals.var_beta_dn6 * assign86430_e132062) + (locals.var_beta * (-locals.var_vxbgmtcl_dn6))), ((locals.var_beta_dn7 * assign86430_e132062) + (locals.var_beta * (-locals.var_vxbgmtcl_dn7))), ((locals.var_beta_dn8 * assign86430_e132062) + (locals.var_beta * (-locals.var_vxbgmtcl_dn8))), ((locals.var_beta_dn9 * assign86430_e132062) + (locals.var_beta * (-locals.var_vxbgmtcl_dn9))), ((locals.var_beta_dn10 * assign86430_e132062) + (locals.var_beta * (-locals.var_vxbgmtcl_dn10))), ((locals.var_beta_dn13 * assign86430_e132062) + (locals.var_beta * (-locals.var_vxbgmtcl_dn13))),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign86430_e132065;
        locals.var_tmf1_dn0 = assign86430_e132065_d_n0;
        locals.var_tmf1_dn2 = assign86430_e132065_d_n2;
        locals.var_tmf1_dn4 = assign86430_e132065_d_n4;
        locals.var_tmf1_dn5 = assign86430_e132065_d_n5;
        locals.var_tmf1_dn6 = assign86430_e132065_d_n6;
        locals.var_tmf1_dn7 = assign86430_e132065_d_n7;
        locals.var_tmf1_dn8 = assign86430_e132065_d_n8;
        locals.var_tmf1_dn9 = assign86430_e132065_d_n9;
        locals.var_tmf1_dn10 = assign86430_e132065_d_n10;
        locals.var_tmf1_dn13 = assign86430_e132065_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign86440_e132072, assign86440_e132072_d_n0, assign86440_e132072_d_n2, assign86440_e132072_d_n4, assign86440_e132072_d_n5, assign86440_e132072_d_n6, assign86440_e132072_d_n7, assign86440_e132072_d_n8, assign86440_e132072_d_n9, assign86440_e132072_d_n10, assign86440_e132072_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2021 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn13,)
    }
};
        locals.var_exp_bvbs = assign86440_e132072;
        locals.var_exp_bvbs_dn0 = assign86440_e132072_d_n0;
        locals.var_exp_bvbs_dn2 = assign86440_e132072_d_n2;
        locals.var_exp_bvbs_dn4 = assign86440_e132072_d_n4;
        locals.var_exp_bvbs_dn5 = assign86440_e132072_d_n5;
        locals.var_exp_bvbs_dn6 = assign86440_e132072_d_n6;
        locals.var_exp_bvbs_dn7 = assign86440_e132072_d_n7;
        locals.var_exp_bvbs_dn8 = assign86440_e132072_d_n8;
        locals.var_exp_bvbs_dn9 = assign86440_e132072_d_n9;
        locals.var_exp_bvbs_dn10 = assign86440_e132072_d_n10;
        locals.var_exp_bvbs_dn13 = assign86440_e132072_d_n13;
        locals.var_exp_bvbs_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_320(
        locals: &mut StampLocals,
    ) {
        let mut assign86450_loop_guard: usize = 0;
        while {
            let assign86450_cond_e132080: f64 = if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2021 == 0.0)) && (locals.var_tmf1 >= 60.0)) { 1.0 } else { 0.0 };
            assign86450_cond_e132080 != 0.0
        } {
            assign86450_loop_guard += 1;
            assert!(assign86450_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign86450_body0_e132089, assign86450_body0_e132089_d_n0, assign86450_body0_e132089_d_n2, assign86450_body0_e132089_d_n4, assign86450_body0_e132089_d_n5, assign86450_body0_e132089_d_n6, assign86450_body0_e132089_d_n7, assign86450_body0_e132089_d_n8, assign86450_body0_e132089_d_n9, assign86450_body0_e132089_d_n10, assign86450_body0_e132089_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2021 == 0.0)) {
        let assign86450_body0_e132087: f64 = (locals.var_exp_bvbs * 1.14200738981568e26);
        (assign86450_body0_e132087, (locals.var_exp_bvbs_dn0 * 1.14200738981568e26), (locals.var_exp_bvbs_dn2 * 1.14200738981568e26), (locals.var_exp_bvbs_dn4 * 1.14200738981568e26), (locals.var_exp_bvbs_dn5 * 1.14200738981568e26), (locals.var_exp_bvbs_dn6 * 1.14200738981568e26), (locals.var_exp_bvbs_dn7 * 1.14200738981568e26), (locals.var_exp_bvbs_dn8 * 1.14200738981568e26), (locals.var_exp_bvbs_dn9 * 1.14200738981568e26), (locals.var_exp_bvbs_dn10 * 1.14200738981568e26), (locals.var_exp_bvbs_dn13 * 1.14200738981568e26),)
    } else {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn13,)
    }
};
            locals.var_exp_bvbs = assign86450_body0_e132089;
            locals.var_exp_bvbs_dn0 = assign86450_body0_e132089_d_n0;
            locals.var_exp_bvbs_dn2 = assign86450_body0_e132089_d_n2;
            locals.var_exp_bvbs_dn4 = assign86450_body0_e132089_d_n4;
            locals.var_exp_bvbs_dn5 = assign86450_body0_e132089_d_n5;
            locals.var_exp_bvbs_dn6 = assign86450_body0_e132089_d_n6;
            locals.var_exp_bvbs_dn7 = assign86450_body0_e132089_d_n7;
            locals.var_exp_bvbs_dn8 = assign86450_body0_e132089_d_n8;
            locals.var_exp_bvbs_dn9 = assign86450_body0_e132089_d_n9;
            locals.var_exp_bvbs_dn10 = assign86450_body0_e132089_d_n10;
            locals.var_exp_bvbs_dn13 = assign86450_body0_e132089_d_n13;
            locals.var_exp_bvbs_rv = 0.0;
            let (assign86450_body1_e132098, assign86450_body1_e132098_d_n0, assign86450_body1_e132098_d_n2, assign86450_body1_e132098_d_n4, assign86450_body1_e132098_d_n5, assign86450_body1_e132098_d_n6, assign86450_body1_e132098_d_n7, assign86450_body1_e132098_d_n8, assign86450_body1_e132098_d_n9, assign86450_body1_e132098_d_n10, assign86450_body1_e132098_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2021 == 0.0)) {
        let assign86450_body1_e132096: f64 = (locals.var_tmf1 - 60.0);
        (assign86450_body1_e132096, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
            locals.var_tmf1 = assign86450_body1_e132098;
            locals.var_tmf1_dn0 = assign86450_body1_e132098_d_n0;
            locals.var_tmf1_dn2 = assign86450_body1_e132098_d_n2;
            locals.var_tmf1_dn4 = assign86450_body1_e132098_d_n4;
            locals.var_tmf1_dn5 = assign86450_body1_e132098_d_n5;
            locals.var_tmf1_dn6 = assign86450_body1_e132098_d_n6;
            locals.var_tmf1_dn7 = assign86450_body1_e132098_d_n7;
            locals.var_tmf1_dn8 = assign86450_body1_e132098_d_n8;
            locals.var_tmf1_dn9 = assign86450_body1_e132098_d_n9;
            locals.var_tmf1_dn10 = assign86450_body1_e132098_d_n10;
            locals.var_tmf1_dn13 = assign86450_body1_e132098_d_n13;
            locals.var_tmf1_rv = 0.0;
        }

        let (assign86460_e132108, assign86460_e132108_d_n0, assign86460_e132108_d_n2, assign86460_e132108_d_n4, assign86460_e132108_d_n5, assign86460_e132108_d_n6, assign86460_e132108_d_n7, assign86460_e132108_d_n8, assign86460_e132108_d_n9, assign86460_e132108_d_n10, assign86460_e132108_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2021 == 0.0)) {
        let assign86460_e132105: f64 = (locals.var_tmf1).exp();
        let assign86460_e132106: f64 = (locals.var_exp_bvbs * assign86460_e132105);
        (assign86460_e132106, ((locals.var_exp_bvbs_dn0 * assign86460_e132105) + (locals.var_exp_bvbs * (assign86460_e132105 * locals.var_tmf1_dn0))), ((locals.var_exp_bvbs_dn2 * assign86460_e132105) + (locals.var_exp_bvbs * (assign86460_e132105 * locals.var_tmf1_dn2))), ((locals.var_exp_bvbs_dn4 * assign86460_e132105) + (locals.var_exp_bvbs * (assign86460_e132105 * locals.var_tmf1_dn4))), ((locals.var_exp_bvbs_dn5 * assign86460_e132105) + (locals.var_exp_bvbs * (assign86460_e132105 * locals.var_tmf1_dn5))), ((locals.var_exp_bvbs_dn6 * assign86460_e132105) + (locals.var_exp_bvbs * (assign86460_e132105 * locals.var_tmf1_dn6))), ((locals.var_exp_bvbs_dn7 * assign86460_e132105) + (locals.var_exp_bvbs * (assign86460_e132105 * locals.var_tmf1_dn7))), ((locals.var_exp_bvbs_dn8 * assign86460_e132105) + (locals.var_exp_bvbs * (assign86460_e132105 * locals.var_tmf1_dn8))), ((locals.var_exp_bvbs_dn9 * assign86460_e132105) + (locals.var_exp_bvbs * (assign86460_e132105 * locals.var_tmf1_dn9))), ((locals.var_exp_bvbs_dn10 * assign86460_e132105) + (locals.var_exp_bvbs * (assign86460_e132105 * locals.var_tmf1_dn10))), ((locals.var_exp_bvbs_dn13 * assign86460_e132105) + (locals.var_exp_bvbs * (assign86460_e132105 * locals.var_tmf1_dn13))),)
    } else {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn13,)
    }
};
        locals.var_exp_bvbs = assign86460_e132108;
        locals.var_exp_bvbs_dn0 = assign86460_e132108_d_n0;
        locals.var_exp_bvbs_dn2 = assign86460_e132108_d_n2;
        locals.var_exp_bvbs_dn4 = assign86460_e132108_d_n4;
        locals.var_exp_bvbs_dn5 = assign86460_e132108_d_n5;
        locals.var_exp_bvbs_dn6 = assign86460_e132108_d_n6;
        locals.var_exp_bvbs_dn7 = assign86460_e132108_d_n7;
        locals.var_exp_bvbs_dn8 = assign86460_e132108_d_n8;
        locals.var_exp_bvbs_dn9 = assign86460_e132108_d_n9;
        locals.var_exp_bvbs_dn10 = assign86460_e132108_d_n10;
        locals.var_exp_bvbs_dn13 = assign86460_e132108_d_n13;
        locals.var_exp_bvbs_rv = 0.0;

        let (assign86470_e132115, assign86470_e132115_d_n0, assign86470_e132115_d_n2, assign86470_e132115_d_n4, assign86470_e132115_d_n5, assign86470_e132115_d_n6, assign86470_e132115_d_n7, assign86470_e132115_d_n8, assign86470_e132115_d_n9, assign86470_e132115_d_n10, assign86470_e132115_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2021 == 0.0)) {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign86470_e132115;
        locals.var_t0_dn0 = assign86470_e132115_d_n0;
        locals.var_t0_dn2 = assign86470_e132115_d_n2;
        locals.var_t0_dn4 = assign86470_e132115_d_n4;
        locals.var_t0_dn5 = assign86470_e132115_d_n5;
        locals.var_t0_dn6 = assign86470_e132115_d_n6;
        locals.var_t0_dn7 = assign86470_e132115_d_n7;
        locals.var_t0_dn8 = assign86470_e132115_d_n8;
        locals.var_t0_dn9 = assign86470_e132115_d_n9;
        locals.var_t0_dn10 = assign86470_e132115_d_n10;
        locals.var_t0_dn13 = assign86470_e132115_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign86480_e132128, assign86480_e132128_d_n0, assign86480_e132128_d_n2, assign86480_e132128_d_n4, assign86480_e132128_d_n5, assign86480_e132128_d_n6, assign86480_e132128_d_n7, assign86480_e132128_d_n8, assign86480_e132128_d_n9, assign86480_e132128_d_n10, assign86480_e132128_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign86480_e132120: f64 = (-locals.var_vgpld);
        let assign86480_e132122: f64 = (assign86480_e132120 * 0.5);
        let assign86480_e132124: f64 = (assign86480_e132122 - 0.5);
        let assign86480_e132126: f64 = (assign86480_e132124 - 1.0);
        (assign86480_e132126, 0.0, ((-locals.var_vgpld_dn2) * 0.5), 0.0, 0.0, ((-locals.var_vgpld_dn6) * 0.5), ((-locals.var_vgpld_dn7) * 0.5), ((-locals.var_vgpld_dn8) * 0.5), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign86480_e132128;
        locals.var_tmf1_dn0 = assign86480_e132128_d_n0;
        locals.var_tmf1_dn2 = assign86480_e132128_d_n2;
        locals.var_tmf1_dn4 = assign86480_e132128_d_n4;
        locals.var_tmf1_dn5 = assign86480_e132128_d_n5;
        locals.var_tmf1_dn6 = assign86480_e132128_d_n6;
        locals.var_tmf1_dn7 = assign86480_e132128_d_n7;
        locals.var_tmf1_dn8 = assign86480_e132128_d_n8;
        locals.var_tmf1_dn9 = assign86480_e132128_d_n9;
        locals.var_tmf1_dn10 = assign86480_e132128_d_n10;
        locals.var_tmf1_dn13 = assign86480_e132128_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign86490_e132138, assign86490_e132138_d_n0, assign86490_e132138_d_n2, assign86490_e132138_d_n4, assign86490_e132138_d_n5, assign86490_e132138_d_n6, assign86490_e132138_d_n7, assign86490_e132138_d_n8, assign86490_e132138_d_n9, assign86490_e132138_d_n10, assign86490_e132138_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign86490_e132134: f64 = (4.0 * 0.5);
        let assign86490_e132136: f64 = assign86490_e132134;
        (assign86490_e132136, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign86490_e132138;
        locals.var_tmf2_dn0 = assign86490_e132138_d_n0;
        locals.var_tmf2_dn2 = assign86490_e132138_d_n2;
        locals.var_tmf2_dn4 = assign86490_e132138_d_n4;
        locals.var_tmf2_dn5 = assign86490_e132138_d_n5;
        locals.var_tmf2_dn6 = assign86490_e132138_d_n6;
        locals.var_tmf2_dn7 = assign86490_e132138_d_n7;
        locals.var_tmf2_dn8 = assign86490_e132138_d_n8;
        locals.var_tmf2_dn9 = assign86490_e132138_d_n9;
        locals.var_tmf2_dn10 = assign86490_e132138_d_n10;
        locals.var_tmf2_dn13 = assign86490_e132138_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign86500_e132150, assign86500_e132150_d_n0, assign86500_e132150_d_n2, assign86500_e132150_d_n4, assign86500_e132150_d_n5, assign86500_e132150_d_n6, assign86500_e132150_d_n7, assign86500_e132150_d_n8, assign86500_e132150_d_n9, assign86500_e132150_d_n10, assign86500_e132150_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let (assign86500_e132148, assign86500_e132148_d_n0, assign86500_e132148_d_n2, assign86500_e132148_d_n4, assign86500_e132148_d_n5, assign86500_e132148_d_n6, assign86500_e132148_d_n7, assign86500_e132148_d_n8, assign86500_e132148_d_n9, assign86500_e132148_d_n10, assign86500_e132148_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign86500_e132147: f64 = (-locals.var_tmf2);
                (assign86500_e132147, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign86500_e132148, assign86500_e132148_d_n0, assign86500_e132148_d_n2, assign86500_e132148_d_n4, assign86500_e132148_d_n5, assign86500_e132148_d_n6, assign86500_e132148_d_n7, assign86500_e132148_d_n8, assign86500_e132148_d_n9, assign86500_e132148_d_n10, assign86500_e132148_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign86500_e132150;
        locals.var_tmf2_dn0 = assign86500_e132150_d_n0;
        locals.var_tmf2_dn2 = assign86500_e132150_d_n2;
        locals.var_tmf2_dn4 = assign86500_e132150_d_n4;
        locals.var_tmf2_dn5 = assign86500_e132150_d_n5;
        locals.var_tmf2_dn6 = assign86500_e132150_d_n6;
        locals.var_tmf2_dn7 = assign86500_e132150_d_n7;
        locals.var_tmf2_dn8 = assign86500_e132150_d_n8;
        locals.var_tmf2_dn9 = assign86500_e132150_d_n9;
        locals.var_tmf2_dn10 = assign86500_e132150_d_n10;
        locals.var_tmf2_dn13 = assign86500_e132150_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign86510_e132161, assign86510_e132161_d_n0, assign86510_e132161_d_n2, assign86510_e132161_d_n4, assign86510_e132161_d_n5, assign86510_e132161_d_n6, assign86510_e132161_d_n7, assign86510_e132161_d_n8, assign86510_e132161_d_n9, assign86510_e132161_d_n10, assign86510_e132161_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign86510_e132156: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign86510_e132158: f64 = (assign86510_e132156 + locals.var_tmf2);
        let assign86510_e132159: f64 = (assign86510_e132158).sqrt();
        (assign86510_e132159, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign86510_e132159)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign86510_e132159)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign86510_e132159)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign86510_e132159)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign86510_e132159)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign86510_e132159)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign86510_e132159)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign86510_e132159)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign86510_e132159)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign86510_e132159)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign86510_e132161;
        locals.var_tmf2_dn0 = assign86510_e132161_d_n0;
        locals.var_tmf2_dn2 = assign86510_e132161_d_n2;
        locals.var_tmf2_dn4 = assign86510_e132161_d_n4;
        locals.var_tmf2_dn5 = assign86510_e132161_d_n5;
        locals.var_tmf2_dn6 = assign86510_e132161_d_n6;
        locals.var_tmf2_dn7 = assign86510_e132161_d_n7;
        locals.var_tmf2_dn8 = assign86510_e132161_d_n8;
        locals.var_tmf2_dn9 = assign86510_e132161_d_n9;
        locals.var_tmf2_dn10 = assign86510_e132161_d_n10;
        locals.var_tmf2_dn13 = assign86510_e132161_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign86520_e132173, assign86520_e132173_d_n0, assign86520_e132173_d_n2, assign86520_e132173_d_n4, assign86520_e132173_d_n5, assign86520_e132173_d_n6, assign86520_e132173_d_n7, assign86520_e132173_d_n8, assign86520_e132173_d_n9, assign86520_e132173_d_n10, assign86520_e132173_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign86520_e132169: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign86520_e132170: f64 = (1.0 + assign86520_e132169);
        let assign86520_e132171: f64 = (0.5 * assign86520_e132170);
        (assign86520_e132171, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign86520_e132173;
        locals.var_t0_dn0 = assign86520_e132173_d_n0;
        locals.var_t0_dn2 = assign86520_e132173_d_n2;
        locals.var_t0_dn4 = assign86520_e132173_d_n4;
        locals.var_t0_dn5 = assign86520_e132173_d_n5;
        locals.var_t0_dn6 = assign86520_e132173_d_n6;
        locals.var_t0_dn7 = assign86520_e132173_d_n7;
        locals.var_t0_dn8 = assign86520_e132173_d_n8;
        locals.var_t0_dn9 = assign86520_e132173_d_n9;
        locals.var_t0_dn10 = assign86520_e132173_d_n10;
        locals.var_t0_dn13 = assign86520_e132173_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign86530_e132185, assign86530_e132185_d_n0, assign86530_e132185_d_n2, assign86530_e132185_d_n4, assign86530_e132185_d_n5, assign86530_e132185_d_n6, assign86530_e132185_d_n7, assign86530_e132185_d_n8, assign86530_e132185_d_n9, assign86530_e132185_d_n10, assign86530_e132185_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign86530_e132181: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign86530_e132182: f64 = (0.5 * assign86530_e132181);
        let assign86530_e132183: f64 = (0.5 + assign86530_e132182);
        (assign86530_e132183, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign86530_e132185;
        locals.var_t1_dn0 = assign86530_e132185_d_n0;
        locals.var_t1_dn2 = assign86530_e132185_d_n2;
        locals.var_t1_dn4 = assign86530_e132185_d_n4;
        locals.var_t1_dn5 = assign86530_e132185_d_n5;
        locals.var_t1_dn6 = assign86530_e132185_d_n6;
        locals.var_t1_dn7 = assign86530_e132185_d_n7;
        locals.var_t1_dn8 = assign86530_e132185_d_n8;
        locals.var_t1_dn9 = assign86530_e132185_d_n9;
        locals.var_t1_dn10 = assign86530_e132185_d_n10;
        locals.var_t1_dn13 = assign86530_e132185_d_n13;
        locals.var_t1_rv = 0.0;

        let assign86540_e132188: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign86540_e132191: f64 = (-locals.var_t1);
        let assign86540_e132196: f64 = if ((assign86540_e132188 > assign86540_e132191) && (locals.var_t1 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2022 = assign86540_e132196;
        locals.var_guard2022_rv = 0.0;

        let (assign86550_e132210, assign86550_e132210_d_n0, assign86550_e132210_d_n2, assign86550_e132210_d_n4, assign86550_e132210_d_n5, assign86550_e132210_d_n6, assign86550_e132210_d_n7, assign86550_e132210_d_n8, assign86550_e132210_d_n9, assign86550_e132210_d_n10, assign86550_e132210_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2022 != 0.0)) {
        let assign86550_e132204: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign86550_e132206: f64 = assign86550_e132204;
        let assign86550_e132208: f64 = (assign86550_e132206 + locals.var_t1);
        (assign86550_e132208, (locals.var_vxbgmtcl_dn0 + locals.var_t1_dn0), ((locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2) + locals.var_t1_dn2), (locals.var_vxbgmtcl_dn4 + locals.var_t1_dn4), (locals.var_vxbgmtcl_dn5 + locals.var_t1_dn5), ((locals.var_vgpld_dn6 + locals.var_vxbgmtcl_dn6) + locals.var_t1_dn6), ((locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7) + locals.var_t1_dn7), ((locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8) + locals.var_t1_dn8), (locals.var_vxbgmtcl_dn9 + locals.var_t1_dn9), (locals.var_vxbgmtcl_dn10 + locals.var_t1_dn10), (locals.var_vxbgmtcl_dn13 + locals.var_t1_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign86550_e132210;
        locals.var_tmf1_dn0 = assign86550_e132210_d_n0;
        locals.var_tmf1_dn2 = assign86550_e132210_d_n2;
        locals.var_tmf1_dn4 = assign86550_e132210_d_n4;
        locals.var_tmf1_dn5 = assign86550_e132210_d_n5;
        locals.var_tmf1_dn6 = assign86550_e132210_d_n6;
        locals.var_tmf1_dn7 = assign86550_e132210_d_n7;
        locals.var_tmf1_dn8 = assign86550_e132210_d_n8;
        locals.var_tmf1_dn9 = assign86550_e132210_d_n9;
        locals.var_tmf1_dn10 = assign86550_e132210_d_n10;
        locals.var_tmf1_dn13 = assign86550_e132210_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign86560_e132220, assign86560_e132220_d_n0, assign86560_e132220_d_n2, assign86560_e132220_d_n4, assign86560_e132220_d_n5, assign86560_e132220_d_n6, assign86560_e132220_d_n7, assign86560_e132220_d_n8, assign86560_e132220_d_n9, assign86560_e132220_d_n10, assign86560_e132220_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2022 != 0.0)) {
        let assign86560_e132218: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign86560_e132218, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
        locals.var_x2 = assign86560_e132220;
        locals.var_x2_dn0 = assign86560_e132220_d_n0;
        locals.var_x2_dn2 = assign86560_e132220_d_n2;
        locals.var_x2_dn4 = assign86560_e132220_d_n4;
        locals.var_x2_dn5 = assign86560_e132220_d_n5;
        locals.var_x2_dn6 = assign86560_e132220_d_n6;
        locals.var_x2_dn7 = assign86560_e132220_d_n7;
        locals.var_x2_dn8 = assign86560_e132220_d_n8;
        locals.var_x2_dn9 = assign86560_e132220_d_n9;
        locals.var_x2_dn10 = assign86560_e132220_d_n10;
        locals.var_x2_dn13 = assign86560_e132220_d_n13;
        locals.var_x2_rv = 0.0;

        let (assign86570_e132230, assign86570_e132230_d_n0, assign86570_e132230_d_n2, assign86570_e132230_d_n4, assign86570_e132230_d_n5, assign86570_e132230_d_n6, assign86570_e132230_d_n7, assign86570_e132230_d_n8, assign86570_e132230_d_n9, assign86570_e132230_d_n10, assign86570_e132230_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2022 != 0.0)) {
        let assign86570_e132228: f64 = (locals.var_t1 * locals.var_t1);
        (assign86570_e132228, ((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)), ((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)), ((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)), ((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)), ((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)), ((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)), ((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)), ((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)), ((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)), ((locals.var_t1_dn13 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn13)),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
        locals.var_xmax2 = assign86570_e132230;
        locals.var_xmax2_dn0 = assign86570_e132230_d_n0;
        locals.var_xmax2_dn2 = assign86570_e132230_d_n2;
        locals.var_xmax2_dn4 = assign86570_e132230_d_n4;
        locals.var_xmax2_dn5 = assign86570_e132230_d_n5;
        locals.var_xmax2_dn6 = assign86570_e132230_d_n6;
        locals.var_xmax2_dn7 = assign86570_e132230_d_n7;
        locals.var_xmax2_dn8 = assign86570_e132230_d_n8;
        locals.var_xmax2_dn9 = assign86570_e132230_d_n9;
        locals.var_xmax2_dn10 = assign86570_e132230_d_n10;
        locals.var_xmax2_dn13 = assign86570_e132230_d_n13;
        locals.var_xmax2_rv = 0.0;

        let (assign86580_e132238, assign86580_e132238_d_n0, assign86580_e132238_d_n2, assign86580_e132238_d_n4, assign86580_e132238_d_n5, assign86580_e132238_d_n6, assign86580_e132238_d_n7, assign86580_e132238_d_n8, assign86580_e132238_d_n9, assign86580_e132238_d_n10, assign86580_e132238_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2022 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign86580_e132238;
        locals.var_xp_dn0 = assign86580_e132238_d_n0;
        locals.var_xp_dn2 = assign86580_e132238_d_n2;
        locals.var_xp_dn4 = assign86580_e132238_d_n4;
        locals.var_xp_dn5 = assign86580_e132238_d_n5;
        locals.var_xp_dn6 = assign86580_e132238_d_n6;
        locals.var_xp_dn7 = assign86580_e132238_d_n7;
        locals.var_xp_dn8 = assign86580_e132238_d_n8;
        locals.var_xp_dn9 = assign86580_e132238_d_n9;
        locals.var_xp_dn10 = assign86580_e132238_d_n10;
        locals.var_xp_dn13 = assign86580_e132238_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign86590_e132246, assign86590_e132246_d_n0, assign86590_e132246_d_n2, assign86590_e132246_d_n4, assign86590_e132246_d_n5, assign86590_e132246_d_n6, assign86590_e132246_d_n7, assign86590_e132246_d_n8, assign86590_e132246_d_n9, assign86590_e132246_d_n10, assign86590_e132246_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2022 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign86590_e132246;
        locals.var_xmp_dn0 = assign86590_e132246_d_n0;
        locals.var_xmp_dn2 = assign86590_e132246_d_n2;
        locals.var_xmp_dn4 = assign86590_e132246_d_n4;
        locals.var_xmp_dn5 = assign86590_e132246_d_n5;
        locals.var_xmp_dn6 = assign86590_e132246_d_n6;
        locals.var_xmp_dn7 = assign86590_e132246_d_n7;
        locals.var_xmp_dn8 = assign86590_e132246_d_n8;
        locals.var_xmp_dn9 = assign86590_e132246_d_n9;
        locals.var_xmp_dn10 = assign86590_e132246_d_n10;
        locals.var_xmp_dn13 = assign86590_e132246_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign86600_e132254,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2022 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign86600_e132254;
        locals.var_m0_rv = 0.0;

        let (assign86610_e132262,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2022 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign86610_e132262;
        locals.var_mm_rv = 0.0;

        let (assign86620_e132270, assign86620_e132270_d_n0, assign86620_e132270_d_n2, assign86620_e132270_d_n4, assign86620_e132270_d_n5, assign86620_e132270_d_n6, assign86620_e132270_d_n7, assign86620_e132270_d_n8, assign86620_e132270_d_n9, assign86620_e132270_d_n10, assign86620_e132270_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2022 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign86620_e132270;
        locals.var_arg_dn0 = assign86620_e132270_d_n0;
        locals.var_arg_dn2 = assign86620_e132270_d_n2;
        locals.var_arg_dn4 = assign86620_e132270_d_n4;
        locals.var_arg_dn5 = assign86620_e132270_d_n5;
        locals.var_arg_dn6 = assign86620_e132270_d_n6;
        locals.var_arg_dn7 = assign86620_e132270_d_n7;
        locals.var_arg_dn8 = assign86620_e132270_d_n8;
        locals.var_arg_dn9 = assign86620_e132270_d_n9;
        locals.var_arg_dn10 = assign86620_e132270_d_n10;
        locals.var_arg_dn13 = assign86620_e132270_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign86630_e132278, assign86630_e132278_d_n0, assign86630_e132278_d_n2, assign86630_e132278_d_n4, assign86630_e132278_d_n5, assign86630_e132278_d_n6, assign86630_e132278_d_n7, assign86630_e132278_d_n8, assign86630_e132278_d_n9, assign86630_e132278_d_n10, assign86630_e132278_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2022 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign86630_e132278;
        locals.var_dnm_dn0 = assign86630_e132278_d_n0;
        locals.var_dnm_dn2 = assign86630_e132278_d_n2;
        locals.var_dnm_dn4 = assign86630_e132278_d_n4;
        locals.var_dnm_dn5 = assign86630_e132278_d_n5;
        locals.var_dnm_dn6 = assign86630_e132278_d_n6;
        locals.var_dnm_dn7 = assign86630_e132278_d_n7;
        locals.var_dnm_dn8 = assign86630_e132278_d_n8;
        locals.var_dnm_dn9 = assign86630_e132278_d_n9;
        locals.var_dnm_dn10 = assign86630_e132278_d_n10;
        locals.var_dnm_dn13 = assign86630_e132278_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign86640_e132288, assign86640_e132288_d_n0, assign86640_e132288_d_n2, assign86640_e132288_d_n4, assign86640_e132288_d_n5, assign86640_e132288_d_n6, assign86640_e132288_d_n7, assign86640_e132288_d_n8, assign86640_e132288_d_n9, assign86640_e132288_d_n10, assign86640_e132288_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2022 != 0.0)) {
        let assign86640_e132286: f64 = (locals.var_xp * locals.var_x2);
        (assign86640_e132286, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign86640_e132288;
        locals.var_xp_dn0 = assign86640_e132288_d_n0;
        locals.var_xp_dn2 = assign86640_e132288_d_n2;
        locals.var_xp_dn4 = assign86640_e132288_d_n4;
        locals.var_xp_dn5 = assign86640_e132288_d_n5;
        locals.var_xp_dn6 = assign86640_e132288_d_n6;
        locals.var_xp_dn7 = assign86640_e132288_d_n7;
        locals.var_xp_dn8 = assign86640_e132288_d_n8;
        locals.var_xp_dn9 = assign86640_e132288_d_n9;
        locals.var_xp_dn10 = assign86640_e132288_d_n10;
        locals.var_xp_dn13 = assign86640_e132288_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign86650_e132298, assign86650_e132298_d_n0, assign86650_e132298_d_n2, assign86650_e132298_d_n4, assign86650_e132298_d_n5, assign86650_e132298_d_n6, assign86650_e132298_d_n7, assign86650_e132298_d_n8, assign86650_e132298_d_n9, assign86650_e132298_d_n10, assign86650_e132298_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2022 != 0.0)) {
        let assign86650_e132296: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign86650_e132296, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign86650_e132298;
        locals.var_xmp_dn0 = assign86650_e132298_d_n0;
        locals.var_xmp_dn2 = assign86650_e132298_d_n2;
        locals.var_xmp_dn4 = assign86650_e132298_d_n4;
        locals.var_xmp_dn5 = assign86650_e132298_d_n5;
        locals.var_xmp_dn6 = assign86650_e132298_d_n6;
        locals.var_xmp_dn7 = assign86650_e132298_d_n7;
        locals.var_xmp_dn8 = assign86650_e132298_d_n8;
        locals.var_xmp_dn9 = assign86650_e132298_d_n9;
        locals.var_xmp_dn10 = assign86650_e132298_d_n10;
        locals.var_xmp_dn13 = assign86650_e132298_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign86660_e132308, assign86660_e132308_d_n0, assign86660_e132308_d_n2, assign86660_e132308_d_n4, assign86660_e132308_d_n5, assign86660_e132308_d_n6, assign86660_e132308_d_n7, assign86660_e132308_d_n8, assign86660_e132308_d_n9, assign86660_e132308_d_n10, assign86660_e132308_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2022 != 0.0)) {
        let assign86660_e132306: f64 = (locals.var_xp + locals.var_xmp);
        (assign86660_e132306, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign86660_e132308;
        locals.var_arg_dn0 = assign86660_e132308_d_n0;
        locals.var_arg_dn2 = assign86660_e132308_d_n2;
        locals.var_arg_dn4 = assign86660_e132308_d_n4;
        locals.var_arg_dn5 = assign86660_e132308_d_n5;
        locals.var_arg_dn6 = assign86660_e132308_d_n6;
        locals.var_arg_dn7 = assign86660_e132308_d_n7;
        locals.var_arg_dn8 = assign86660_e132308_d_n8;
        locals.var_arg_dn9 = assign86660_e132308_d_n9;
        locals.var_arg_dn10 = assign86660_e132308_d_n10;
        locals.var_arg_dn13 = assign86660_e132308_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign86670_e132316, assign86670_e132316_d_n0, assign86670_e132316_d_n2, assign86670_e132316_d_n4, assign86670_e132316_d_n5, assign86670_e132316_d_n6, assign86670_e132316_d_n7, assign86670_e132316_d_n8, assign86670_e132316_d_n9, assign86670_e132316_d_n10, assign86670_e132316_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2022 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign86670_e132316;
        locals.var_dnm_dn0 = assign86670_e132316_d_n0;
        locals.var_dnm_dn2 = assign86670_e132316_d_n2;
        locals.var_dnm_dn4 = assign86670_e132316_d_n4;
        locals.var_dnm_dn5 = assign86670_e132316_d_n5;
        locals.var_dnm_dn6 = assign86670_e132316_d_n6;
        locals.var_dnm_dn7 = assign86670_e132316_d_n7;
        locals.var_dnm_dn8 = assign86670_e132316_d_n8;
        locals.var_dnm_dn9 = assign86670_e132316_d_n9;
        locals.var_dnm_dn10 = assign86670_e132316_d_n10;
        locals.var_dnm_dn13 = assign86670_e132316_d_n13;
        locals.var_dnm_rv = 0.0;

        let assign86680_e132331: f64 = if ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard2023 = assign86680_e132331;
        locals.var_guard2023_rv = 0.0;

        let assign86690_e132334: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2024 = assign86690_e132334;
        locals.var_guard2024_rv = 0.0;

        let (assign86700_e132346,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2022 != 0.0)) && (locals.var_guard2023 != 0.0)) && (locals.var_guard2024 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign86700_e132346;
        locals.var_mm_rv = 0.0;

        let assign86710_e132349: f64 = if 1.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard2025 = assign86710_e132349;
        locals.var_guard2025_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_321(
        locals: &mut StampLocals,
    ) {
        let (assign86720_e132364,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2022 != 0.0)) && (locals.var_guard2023 != 0.0)) && (locals.var_guard2024 == 0.0)) && (locals.var_guard2025 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign86720_e132364;
        locals.var_mm_rv = 0.0;

        let assign86730_e132367: f64 = if 1.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard2026 = assign86730_e132367;
        locals.var_guard2026_rv = 0.0;

        let (assign86740_e132385,) = {
    if (((((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2022 != 0.0)) && (locals.var_guard2023 != 0.0)) && (locals.var_guard2024 == 0.0)) && (locals.var_guard2025 == 0.0)) && (locals.var_guard2026 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign86740_e132385;
        locals.var_mm_rv = 0.0;

        let assign86750_e132388: f64 = if 1.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard2027 = assign86750_e132388;
        locals.var_guard2027_rv = 0.0;

        let (assign86760_e132409,) = {
    if ((((((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2022 != 0.0)) && (locals.var_guard2023 != 0.0)) && (locals.var_guard2024 == 0.0)) && (locals.var_guard2025 == 0.0)) && (locals.var_guard2026 == 0.0)) && (locals.var_guard2027 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign86760_e132409;
        locals.var_mm_rv = 0.0;

        let (assign86770_e132419,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2022 != 0.0)) && (locals.var_guard2023 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign86770_e132419;
        locals.var_m0_rv = 0.0;

        let mut assign86780_loop_guard: usize = 0;
        while {
            let assign86780_cond_e132430: f64 = if (((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2022 != 0.0)) && (locals.var_guard2023 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign86780_cond_e132430 != 0.0
        } {
            assign86780_loop_guard += 1;
            assert!(assign86780_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign86780_body0_e132441, assign86780_body0_e132441_d_n0, assign86780_body0_e132441_d_n2, assign86780_body0_e132441_d_n4, assign86780_body0_e132441_d_n5, assign86780_body0_e132441_d_n6, assign86780_body0_e132441_d_n7, assign86780_body0_e132441_d_n8, assign86780_body0_e132441_d_n9, assign86780_body0_e132441_d_n10, assign86780_body0_e132441_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2022 != 0.0)) && (locals.var_guard2023 != 0.0)) {
        let assign86780_body0_e132439: f64 = (locals.var_dnm).sqrt();
        (assign86780_body0_e132439, (locals.var_dnm_dn0 / (2.0 * assign86780_body0_e132439)), (locals.var_dnm_dn2 / (2.0 * assign86780_body0_e132439)), (locals.var_dnm_dn4 / (2.0 * assign86780_body0_e132439)), (locals.var_dnm_dn5 / (2.0 * assign86780_body0_e132439)), (locals.var_dnm_dn6 / (2.0 * assign86780_body0_e132439)), (locals.var_dnm_dn7 / (2.0 * assign86780_body0_e132439)), (locals.var_dnm_dn8 / (2.0 * assign86780_body0_e132439)), (locals.var_dnm_dn9 / (2.0 * assign86780_body0_e132439)), (locals.var_dnm_dn10 / (2.0 * assign86780_body0_e132439)), (locals.var_dnm_dn13 / (2.0 * assign86780_body0_e132439)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign86780_body0_e132441;
            locals.var_dnm_dn0 = assign86780_body0_e132441_d_n0;
            locals.var_dnm_dn2 = assign86780_body0_e132441_d_n2;
            locals.var_dnm_dn4 = assign86780_body0_e132441_d_n4;
            locals.var_dnm_dn5 = assign86780_body0_e132441_d_n5;
            locals.var_dnm_dn6 = assign86780_body0_e132441_d_n6;
            locals.var_dnm_dn7 = assign86780_body0_e132441_d_n7;
            locals.var_dnm_dn8 = assign86780_body0_e132441_d_n8;
            locals.var_dnm_dn9 = assign86780_body0_e132441_d_n9;
            locals.var_dnm_dn10 = assign86780_body0_e132441_d_n10;
            locals.var_dnm_dn13 = assign86780_body0_e132441_d_n13;
            locals.var_dnm_rv = 0.0;
            let (assign86780_body1_e132453,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2022 != 0.0)) && (locals.var_guard2023 != 0.0)) {
        let assign86780_body1_e132451: f64 = (locals.var_m0 + 1.0);
        (assign86780_body1_e132451,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign86780_body1_e132453;
            locals.var_m0_rv = 0.0;
        }

        let (assign86790_e132475, assign86790_e132475_d_n0, assign86790_e132475_d_n2, assign86790_e132475_d_n4, assign86790_e132475_d_n5, assign86790_e132475_d_n6, assign86790_e132475_d_n7, assign86790_e132475_d_n8, assign86790_e132475_d_n9, assign86790_e132475_d_n10, assign86790_e132475_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2022 != 0.0)) && (locals.var_guard2023 == 0.0)) {
        let (assign86790_e132473, assign86790_e132473_d_n0, assign86790_e132473_d_n2, assign86790_e132473_d_n4, assign86790_e132473_d_n5, assign86790_e132473_d_n6, assign86790_e132473_d_n7, assign86790_e132473_d_n8, assign86790_e132473_d_n9, assign86790_e132473_d_n10, assign86790_e132473_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign86790_e132470: f64 = 2.0;
                let assign86790_e132471: f64 = (1.0 / assign86790_e132470);
                let assign86790_e132472: f64 = (locals.var_dnm).powf(assign86790_e132471);
                (assign86790_e132472, if 0.0 == 0.0 && ((assign86790_e132471) as f64).is_finite() && ((assign86790_e132471) as f64).fract() == 0.0 { if assign86790_e132471 == 0.0 { 0.0 } else { (assign86790_e132471 * ((locals.var_dnm).powf(assign86790_e132471 - 1.0) * locals.var_dnm_dn0)) } } else { (assign86790_e132472 * (assign86790_e132471 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign86790_e132471) as f64).is_finite() && ((assign86790_e132471) as f64).fract() == 0.0 { if assign86790_e132471 == 0.0 { 0.0 } else { (assign86790_e132471 * ((locals.var_dnm).powf(assign86790_e132471 - 1.0) * locals.var_dnm_dn2)) } } else { (assign86790_e132472 * (assign86790_e132471 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign86790_e132471) as f64).is_finite() && ((assign86790_e132471) as f64).fract() == 0.0 { if assign86790_e132471 == 0.0 { 0.0 } else { (assign86790_e132471 * ((locals.var_dnm).powf(assign86790_e132471 - 1.0) * locals.var_dnm_dn4)) } } else { (assign86790_e132472 * (assign86790_e132471 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign86790_e132471) as f64).is_finite() && ((assign86790_e132471) as f64).fract() == 0.0 { if assign86790_e132471 == 0.0 { 0.0 } else { (assign86790_e132471 * ((locals.var_dnm).powf(assign86790_e132471 - 1.0) * locals.var_dnm_dn5)) } } else { (assign86790_e132472 * (assign86790_e132471 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign86790_e132471) as f64).is_finite() && ((assign86790_e132471) as f64).fract() == 0.0 { if assign86790_e132471 == 0.0 { 0.0 } else { (assign86790_e132471 * ((locals.var_dnm).powf(assign86790_e132471 - 1.0) * locals.var_dnm_dn6)) } } else { (assign86790_e132472 * (assign86790_e132471 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign86790_e132471) as f64).is_finite() && ((assign86790_e132471) as f64).fract() == 0.0 { if assign86790_e132471 == 0.0 { 0.0 } else { (assign86790_e132471 * ((locals.var_dnm).powf(assign86790_e132471 - 1.0) * locals.var_dnm_dn7)) } } else { (assign86790_e132472 * (assign86790_e132471 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign86790_e132471) as f64).is_finite() && ((assign86790_e132471) as f64).fract() == 0.0 { if assign86790_e132471 == 0.0 { 0.0 } else { (assign86790_e132471 * ((locals.var_dnm).powf(assign86790_e132471 - 1.0) * locals.var_dnm_dn8)) } } else { (assign86790_e132472 * (assign86790_e132471 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign86790_e132471) as f64).is_finite() && ((assign86790_e132471) as f64).fract() == 0.0 { if assign86790_e132471 == 0.0 { 0.0 } else { (assign86790_e132471 * ((locals.var_dnm).powf(assign86790_e132471 - 1.0) * locals.var_dnm_dn9)) } } else { (assign86790_e132472 * (assign86790_e132471 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign86790_e132471) as f64).is_finite() && ((assign86790_e132471) as f64).fract() == 0.0 { if assign86790_e132471 == 0.0 { 0.0 } else { (assign86790_e132471 * ((locals.var_dnm).powf(assign86790_e132471 - 1.0) * locals.var_dnm_dn10)) } } else { (assign86790_e132472 * (assign86790_e132471 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign86790_e132471) as f64).is_finite() && ((assign86790_e132471) as f64).fract() == 0.0 { if assign86790_e132471 == 0.0 { 0.0 } else { (assign86790_e132471 * ((locals.var_dnm).powf(assign86790_e132471 - 1.0) * locals.var_dnm_dn13)) } } else { (assign86790_e132472 * (assign86790_e132471 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign86790_e132473, assign86790_e132473_d_n0, assign86790_e132473_d_n2, assign86790_e132473_d_n4, assign86790_e132473_d_n5, assign86790_e132473_d_n6, assign86790_e132473_d_n7, assign86790_e132473_d_n8, assign86790_e132473_d_n9, assign86790_e132473_d_n10, assign86790_e132473_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign86790_e132475;
        locals.var_dnm_dn0 = assign86790_e132475_d_n0;
        locals.var_dnm_dn2 = assign86790_e132475_d_n2;
        locals.var_dnm_dn4 = assign86790_e132475_d_n4;
        locals.var_dnm_dn5 = assign86790_e132475_d_n5;
        locals.var_dnm_dn6 = assign86790_e132475_d_n6;
        locals.var_dnm_dn7 = assign86790_e132475_d_n7;
        locals.var_dnm_dn8 = assign86790_e132475_d_n8;
        locals.var_dnm_dn9 = assign86790_e132475_d_n9;
        locals.var_dnm_dn10 = assign86790_e132475_d_n10;
        locals.var_dnm_dn13 = assign86790_e132475_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign86800_e132485, assign86800_e132485_d_n0, assign86800_e132485_d_n2, assign86800_e132485_d_n4, assign86800_e132485_d_n5, assign86800_e132485_d_n6, assign86800_e132485_d_n7, assign86800_e132485_d_n8, assign86800_e132485_d_n9, assign86800_e132485_d_n10, assign86800_e132485_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2022 != 0.0)) {
        let assign86800_e132483: f64 = (1.0 / locals.var_dnm);
        (assign86800_e132483, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign86800_e132485;
        locals.var_dnm_dn0 = assign86800_e132485_d_n0;
        locals.var_dnm_dn2 = assign86800_e132485_d_n2;
        locals.var_dnm_dn4 = assign86800_e132485_d_n4;
        locals.var_dnm_dn5 = assign86800_e132485_d_n5;
        locals.var_dnm_dn6 = assign86800_e132485_d_n6;
        locals.var_dnm_dn7 = assign86800_e132485_d_n7;
        locals.var_dnm_dn8 = assign86800_e132485_d_n8;
        locals.var_dnm_dn9 = assign86800_e132485_d_n9;
        locals.var_dnm_dn10 = assign86800_e132485_d_n10;
        locals.var_dnm_dn13 = assign86800_e132485_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign86810_e132497, assign86810_e132497_d_n0, assign86810_e132497_d_n2, assign86810_e132497_d_n4, assign86810_e132497_d_n5, assign86810_e132497_d_n6, assign86810_e132497_d_n7, assign86810_e132497_d_n8, assign86810_e132497_d_n9, assign86810_e132497_d_n10, assign86810_e132497_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2022 != 0.0)) {
        let assign86810_e132493: f64 = (locals.var_tmf1 * locals.var_t1);
        let assign86810_e132495: f64 = (assign86810_e132493 * locals.var_dnm);
        (assign86810_e132495, ((((locals.var_tmf1_dn0 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn0)) * locals.var_dnm) + (assign86810_e132493 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn2)) * locals.var_dnm) + (assign86810_e132493 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn4 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn4)) * locals.var_dnm) + (assign86810_e132493 * locals.var_dnm_dn4)), ((((locals.var_tmf1_dn5 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn5)) * locals.var_dnm) + (assign86810_e132493 * locals.var_dnm_dn5)), ((((locals.var_tmf1_dn6 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn6)) * locals.var_dnm) + (assign86810_e132493 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn7)) * locals.var_dnm) + (assign86810_e132493 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn8 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn8)) * locals.var_dnm) + (assign86810_e132493 * locals.var_dnm_dn8)), ((((locals.var_tmf1_dn9 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn9)) * locals.var_dnm) + (assign86810_e132493 * locals.var_dnm_dn9)), ((((locals.var_tmf1_dn10 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn10)) * locals.var_dnm) + (assign86810_e132493 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn13 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn13)) * locals.var_dnm) + (assign86810_e132493 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign86810_e132497;
        locals.var_tmf0_dn0 = assign86810_e132497_d_n0;
        locals.var_tmf0_dn2 = assign86810_e132497_d_n2;
        locals.var_tmf0_dn4 = assign86810_e132497_d_n4;
        locals.var_tmf0_dn5 = assign86810_e132497_d_n5;
        locals.var_tmf0_dn6 = assign86810_e132497_d_n6;
        locals.var_tmf0_dn7 = assign86810_e132497_d_n7;
        locals.var_tmf0_dn8 = assign86810_e132497_d_n8;
        locals.var_tmf0_dn9 = assign86810_e132497_d_n9;
        locals.var_tmf0_dn10 = assign86810_e132497_d_n10;
        locals.var_tmf0_dn13 = assign86810_e132497_d_n13;
        locals.var_tmf0_rv = 0.0;

        let (assign86820_e132511, assign86820_e132511_d_n0, assign86820_e132511_d_n2, assign86820_e132511_d_n4, assign86820_e132511_d_n5, assign86820_e132511_d_n6, assign86820_e132511_d_n7, assign86820_e132511_d_n8, assign86820_e132511_d_n9, assign86820_e132511_d_n10, assign86820_e132511_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2022 != 0.0)) {
        let assign86820_e132505: f64 = (locals.var_t1 * locals.var_xmp);
        let assign86820_e132507: f64 = (assign86820_e132505 * locals.var_dnm);
        let assign86820_e132509: f64 = (assign86820_e132507 / locals.var_arg);
        (assign86820_e132509, (((((((locals.var_t1_dn0 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn0)) * locals.var_dnm) + (assign86820_e132505 * locals.var_dnm_dn0)) * locals.var_arg) - (assign86820_e132507 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn2 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn2)) * locals.var_dnm) + (assign86820_e132505 * locals.var_dnm_dn2)) * locals.var_arg) - (assign86820_e132507 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn4 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn4)) * locals.var_dnm) + (assign86820_e132505 * locals.var_dnm_dn4)) * locals.var_arg) - (assign86820_e132507 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn5 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn5)) * locals.var_dnm) + (assign86820_e132505 * locals.var_dnm_dn5)) * locals.var_arg) - (assign86820_e132507 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn6 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn6)) * locals.var_dnm) + (assign86820_e132505 * locals.var_dnm_dn6)) * locals.var_arg) - (assign86820_e132507 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn7 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn7)) * locals.var_dnm) + (assign86820_e132505 * locals.var_dnm_dn7)) * locals.var_arg) - (assign86820_e132507 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn8 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn8)) * locals.var_dnm) + (assign86820_e132505 * locals.var_dnm_dn8)) * locals.var_arg) - (assign86820_e132507 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn9 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn9)) * locals.var_dnm) + (assign86820_e132505 * locals.var_dnm_dn9)) * locals.var_arg) - (assign86820_e132507 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn10 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn10)) * locals.var_dnm) + (assign86820_e132505 * locals.var_dnm_dn10)) * locals.var_arg) - (assign86820_e132507 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn13 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn13)) * locals.var_dnm) + (assign86820_e132505 * locals.var_dnm_dn13)) * locals.var_arg) - (assign86820_e132507 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign86820_e132511;
        locals.var_t0_dn0 = assign86820_e132511_d_n0;
        locals.var_t0_dn2 = assign86820_e132511_d_n2;
        locals.var_t0_dn4 = assign86820_e132511_d_n4;
        locals.var_t0_dn5 = assign86820_e132511_d_n5;
        locals.var_t0_dn6 = assign86820_e132511_d_n6;
        locals.var_t0_dn7 = assign86820_e132511_d_n7;
        locals.var_t0_dn8 = assign86820_e132511_d_n8;
        locals.var_t0_dn9 = assign86820_e132511_d_n9;
        locals.var_t0_dn10 = assign86820_e132511_d_n10;
        locals.var_t0_dn13 = assign86820_e132511_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign86830_e132523, assign86830_e132523_d_n0, assign86830_e132523_d_n2, assign86830_e132523_d_n4, assign86830_e132523_d_n5, assign86830_e132523_d_n6, assign86830_e132523_d_n7, assign86830_e132523_d_n8, assign86830_e132523_d_n9, assign86830_e132523_d_n10, assign86830_e132523_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2022 != 0.0)) {
        let assign86830_e132519: f64 = (-locals.var_t1);
        let assign86830_e132521: f64 = (assign86830_e132519 + locals.var_tmf0);
        (assign86830_e132521, ((-locals.var_t1_dn0) + locals.var_tmf0_dn0), ((-locals.var_t1_dn2) + locals.var_tmf0_dn2), ((-locals.var_t1_dn4) + locals.var_tmf0_dn4), ((-locals.var_t1_dn5) + locals.var_tmf0_dn5), ((-locals.var_t1_dn6) + locals.var_tmf0_dn6), ((-locals.var_t1_dn7) + locals.var_tmf0_dn7), ((-locals.var_t1_dn8) + locals.var_tmf0_dn8), ((-locals.var_t1_dn9) + locals.var_tmf0_dn9), ((-locals.var_t1_dn10) + locals.var_tmf0_dn10), ((-locals.var_t1_dn13) + locals.var_tmf0_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign86830_e132523;
        locals.var_t1_dn0 = assign86830_e132523_d_n0;
        locals.var_t1_dn2 = assign86830_e132523_d_n2;
        locals.var_t1_dn4 = assign86830_e132523_d_n4;
        locals.var_t1_dn5 = assign86830_e132523_d_n5;
        locals.var_t1_dn6 = assign86830_e132523_d_n6;
        locals.var_t1_dn7 = assign86830_e132523_d_n7;
        locals.var_t1_dn8 = assign86830_e132523_d_n8;
        locals.var_t1_dn9 = assign86830_e132523_d_n9;
        locals.var_t1_dn10 = assign86830_e132523_d_n10;
        locals.var_t1_dn13 = assign86830_e132523_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign86840_e132531, assign86840_e132531_d_n0, assign86840_e132531_d_n2, assign86840_e132531_d_n4, assign86840_e132531_d_n5, assign86840_e132531_d_n6, assign86840_e132531_d_n7, assign86840_e132531_d_n8, assign86840_e132531_d_n9, assign86840_e132531_d_n10, assign86840_e132531_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2022 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign86840_e132531;
        locals.var_t0_dn0 = assign86840_e132531_d_n0;
        locals.var_t0_dn2 = assign86840_e132531_d_n2;
        locals.var_t0_dn4 = assign86840_e132531_d_n4;
        locals.var_t0_dn5 = assign86840_e132531_d_n5;
        locals.var_t0_dn6 = assign86840_e132531_d_n6;
        locals.var_t0_dn7 = assign86840_e132531_d_n7;
        locals.var_t0_dn8 = assign86840_e132531_d_n8;
        locals.var_t0_dn9 = assign86840_e132531_d_n9;
        locals.var_t0_dn10 = assign86840_e132531_d_n10;
        locals.var_t0_dn13 = assign86840_e132531_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign86850_e132542, assign86850_e132542_d_n0, assign86850_e132542_d_n2, assign86850_e132542_d_n4, assign86850_e132542_d_n5, assign86850_e132542_d_n6, assign86850_e132542_d_n7, assign86850_e132542_d_n8, assign86850_e132542_d_n9, assign86850_e132542_d_n10, assign86850_e132542_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2022 == 0.0)) {
        let assign86850_e132540: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        (assign86850_e132540, locals.var_vxbgmtcl_dn0, (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2), locals.var_vxbgmtcl_dn4, locals.var_vxbgmtcl_dn5, (locals.var_vgpld_dn6 + locals.var_vxbgmtcl_dn6), (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7), (locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8), locals.var_vxbgmtcl_dn9, locals.var_vxbgmtcl_dn10, locals.var_vxbgmtcl_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign86850_e132542;
        locals.var_t1_dn0 = assign86850_e132542_d_n0;
        locals.var_t1_dn2 = assign86850_e132542_d_n2;
        locals.var_t1_dn4 = assign86850_e132542_d_n4;
        locals.var_t1_dn5 = assign86850_e132542_d_n5;
        locals.var_t1_dn6 = assign86850_e132542_d_n6;
        locals.var_t1_dn7 = assign86850_e132542_d_n7;
        locals.var_t1_dn8 = assign86850_e132542_d_n8;
        locals.var_t1_dn9 = assign86850_e132542_d_n9;
        locals.var_t1_dn10 = assign86850_e132542_d_n10;
        locals.var_t1_dn13 = assign86850_e132542_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign86860_e132551, assign86860_e132551_d_n0, assign86860_e132551_d_n2, assign86860_e132551_d_n4, assign86860_e132551_d_n5, assign86860_e132551_d_n6, assign86860_e132551_d_n7, assign86860_e132551_d_n8, assign86860_e132551_d_n9, assign86860_e132551_d_n10, assign86860_e132551_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2022 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign86860_e132551;
        locals.var_t0_dn0 = assign86860_e132551_d_n0;
        locals.var_t0_dn2 = assign86860_e132551_d_n2;
        locals.var_t0_dn4 = assign86860_e132551_d_n4;
        locals.var_t0_dn5 = assign86860_e132551_d_n5;
        locals.var_t0_dn6 = assign86860_e132551_d_n6;
        locals.var_t0_dn7 = assign86860_e132551_d_n7;
        locals.var_t0_dn8 = assign86860_e132551_d_n8;
        locals.var_t0_dn9 = assign86860_e132551_d_n9;
        locals.var_t0_dn10 = assign86860_e132551_d_n10;
        locals.var_t0_dn13 = assign86860_e132551_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign86870_e132559, assign86870_e132559_d_n0, assign86870_e132559_d_n2, assign86870_e132559_d_n4, assign86870_e132559_d_n5, assign86870_e132559_d_n6, assign86870_e132559_d_n7, assign86870_e132559_d_n8, assign86870_e132559_d_n9, assign86870_e132559_d_n10, assign86870_e132559_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign86870_e132557: f64 = (locals.var_t1 - locals.var_vgpld);
        (assign86870_e132557, locals.var_t1_dn0, (locals.var_t1_dn2 - locals.var_vgpld_dn2), locals.var_t1_dn4, locals.var_t1_dn5, (locals.var_t1_dn6 - locals.var_vgpld_dn6), (locals.var_t1_dn7 - locals.var_vgpld_dn7), (locals.var_t1_dn8 - locals.var_vgpld_dn8), locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    } else {
        (locals.var_vxbgmtcl, locals.var_vxbgmtcl_dn0, locals.var_vxbgmtcl_dn2, locals.var_vxbgmtcl_dn4, locals.var_vxbgmtcl_dn5, locals.var_vxbgmtcl_dn6, locals.var_vxbgmtcl_dn7, locals.var_vxbgmtcl_dn8, locals.var_vxbgmtcl_dn9, locals.var_vxbgmtcl_dn10, locals.var_vxbgmtcl_dn13,)
    }
};
        locals.var_vxbgmtcl = assign86870_e132559;
        locals.var_vxbgmtcl_dn0 = assign86870_e132559_d_n0;
        locals.var_vxbgmtcl_dn2 = assign86870_e132559_d_n2;
        locals.var_vxbgmtcl_dn4 = assign86870_e132559_d_n4;
        locals.var_vxbgmtcl_dn5 = assign86870_e132559_d_n5;
        locals.var_vxbgmtcl_dn6 = assign86870_e132559_d_n6;
        locals.var_vxbgmtcl_dn7 = assign86870_e132559_d_n7;
        locals.var_vxbgmtcl_dn8 = assign86870_e132559_d_n8;
        locals.var_vxbgmtcl_dn9 = assign86870_e132559_d_n9;
        locals.var_vxbgmtcl_dn10 = assign86870_e132559_d_n10;
        locals.var_vxbgmtcl_dn13 = assign86870_e132559_d_n13;
        locals.var_vxbgmtcl_rv = 0.0;

        let (assign86880_e132570, assign86880_e132570_d_n0, assign86880_e132570_d_n2, assign86880_e132570_d_n4, assign86880_e132570_d_n5, assign86880_e132570_d_n6, assign86880_e132570_d_n7, assign86880_e132570_d_n8, assign86880_e132570_d_n9, assign86880_e132570_d_n10, assign86880_e132570_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign86880_e132564: f64 = (-locals.var_vxbgmtcl);
        let assign86880_e132567: f64 = (10.0 * 2.220446049250313e-16);
        let assign86880_e132568: f64 = (assign86880_e132564 + assign86880_e132567);
        (assign86880_e132568, (-locals.var_vxbgmtcl_dn0), (-locals.var_vxbgmtcl_dn2), (-locals.var_vxbgmtcl_dn4), (-locals.var_vxbgmtcl_dn5), (-locals.var_vxbgmtcl_dn6), (-locals.var_vxbgmtcl_dn7), (-locals.var_vxbgmtcl_dn8), (-locals.var_vxbgmtcl_dn9), (-locals.var_vxbgmtcl_dn10), (-locals.var_vxbgmtcl_dn13),)
    } else {
        (locals.var_vgb_fb_ld, locals.var_vgb_fb_ld_dn0, locals.var_vgb_fb_ld_dn2, locals.var_vgb_fb_ld_dn4, locals.var_vgb_fb_ld_dn5, locals.var_vgb_fb_ld_dn6, locals.var_vgb_fb_ld_dn7, locals.var_vgb_fb_ld_dn8, locals.var_vgb_fb_ld_dn9, locals.var_vgb_fb_ld_dn10, locals.var_vgb_fb_ld_dn13,)
    }
};
        locals.var_vgb_fb_ld = assign86880_e132570;
        locals.var_vgb_fb_ld_dn0 = assign86880_e132570_d_n0;
        locals.var_vgb_fb_ld_dn2 = assign86880_e132570_d_n2;
        locals.var_vgb_fb_ld_dn4 = assign86880_e132570_d_n4;
        locals.var_vgb_fb_ld_dn5 = assign86880_e132570_d_n5;
        locals.var_vgb_fb_ld_dn6 = assign86880_e132570_d_n6;
        locals.var_vgb_fb_ld_dn7 = assign86880_e132570_d_n7;
        locals.var_vgb_fb_ld_dn8 = assign86880_e132570_d_n8;
        locals.var_vgb_fb_ld_dn9 = assign86880_e132570_d_n9;
        locals.var_vgb_fb_ld_dn10 = assign86880_e132570_d_n10;
        locals.var_vgb_fb_ld_dn13 = assign86880_e132570_d_n13;
        locals.var_vgb_fb_ld_rv = 0.0;

        let assign86890_e132573: f64 = if locals.var_vgpld < locals.var_vgb_fb_ld { 1.0 } else { 0.0 };
        locals.var_guard2028 = assign86890_e132573;
        locals.var_guard2028_rv = 0.0;

        let (assign86910_e132594, assign86910_e132594_d_n0, assign86910_e132594_d_n2, assign86910_e132594_d_n4, assign86910_e132594_d_n5, assign86910_e132594_d_n6, assign86910_e132594_d_n7, assign86910_e132594_d_n8, assign86910_e132594_d_n9, assign86910_e132594_d_n10, assign86910_e132594_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 != 0.0)) {
        let assign86910_e132586: f64 = (2.0 * locals.var_beta_inv);
        let assign86910_e132588: f64 = (-locals.var_vgs_min);
        let assign86910_e132590: f64 = (assign86910_e132588 / locals.var_fac1);
        let assign86910_e132591: f64 = (assign86910_e132590).ln();
        let assign86910_e132592: f64 = (assign86910_e132586 * assign86910_e132591);
        (assign86910_e132592, (((2.0 * locals.var_beta_inv_dn0) * assign86910_e132591) + (assign86910_e132586 * ((-((assign86910_e132588 * locals.var_fac1_dn0) / (locals.var_fac1 * locals.var_fac1))) / assign86910_e132590))), (((2.0 * locals.var_beta_inv_dn2) * assign86910_e132591) + (assign86910_e132586 * ((-((assign86910_e132588 * locals.var_fac1_dn2) / (locals.var_fac1 * locals.var_fac1))) / assign86910_e132590))), (((2.0 * locals.var_beta_inv_dn4) * assign86910_e132591) + (assign86910_e132586 * ((-((assign86910_e132588 * locals.var_fac1_dn4) / (locals.var_fac1 * locals.var_fac1))) / assign86910_e132590))), (((2.0 * locals.var_beta_inv_dn5) * assign86910_e132591) + (assign86910_e132586 * ((-((assign86910_e132588 * locals.var_fac1_dn5) / (locals.var_fac1 * locals.var_fac1))) / assign86910_e132590))), (((2.0 * locals.var_beta_inv_dn6) * assign86910_e132591) + (assign86910_e132586 * ((-((assign86910_e132588 * locals.var_fac1_dn6) / (locals.var_fac1 * locals.var_fac1))) / assign86910_e132590))), (((2.0 * locals.var_beta_inv_dn7) * assign86910_e132591) + (assign86910_e132586 * ((-((assign86910_e132588 * locals.var_fac1_dn7) / (locals.var_fac1 * locals.var_fac1))) / assign86910_e132590))), (((2.0 * locals.var_beta_inv_dn8) * assign86910_e132591) + (assign86910_e132586 * ((-((assign86910_e132588 * locals.var_fac1_dn8) / (locals.var_fac1 * locals.var_fac1))) / assign86910_e132590))), (((2.0 * locals.var_beta_inv_dn9) * assign86910_e132591) + (assign86910_e132586 * ((-((assign86910_e132588 * locals.var_fac1_dn9) / (locals.var_fac1 * locals.var_fac1))) / assign86910_e132590))), (((2.0 * locals.var_beta_inv_dn10) * assign86910_e132591) + (assign86910_e132586 * ((-((assign86910_e132588 * locals.var_fac1_dn10) / (locals.var_fac1 * locals.var_fac1))) / assign86910_e132590))), (((2.0 * locals.var_beta_inv_dn13) * assign86910_e132591) + (assign86910_e132586 * ((-((assign86910_e132588 * locals.var_fac1_dn13) / (locals.var_fac1 * locals.var_fac1))) / assign86910_e132590))),)
    } else {
        (locals.var_ps0_min, locals.var_ps0_min_dn0, locals.var_ps0_min_dn2, locals.var_ps0_min_dn4, locals.var_ps0_min_dn5, locals.var_ps0_min_dn6, locals.var_ps0_min_dn7, locals.var_ps0_min_dn8, locals.var_ps0_min_dn9, locals.var_ps0_min_dn10, locals.var_ps0_min_dn13,)
    }
};
        locals.var_ps0_min = assign86910_e132594;
        locals.var_ps0_min_dn0 = assign86910_e132594_d_n0;
        locals.var_ps0_min_dn2 = assign86910_e132594_d_n2;
        locals.var_ps0_min_dn4 = assign86910_e132594_d_n4;
        locals.var_ps0_min_dn5 = assign86910_e132594_d_n5;
        locals.var_ps0_min_dn6 = assign86910_e132594_d_n6;
        locals.var_ps0_min_dn7 = assign86910_e132594_d_n7;
        locals.var_ps0_min_dn8 = assign86910_e132594_d_n8;
        locals.var_ps0_min_dn9 = assign86910_e132594_d_n9;
        locals.var_ps0_min_dn10 = assign86910_e132594_d_n10;
        locals.var_ps0_min_dn13 = assign86910_e132594_d_n13;
        locals.var_ps0_min_rv = 0.0;

        let (assign86920_e132604, assign86920_e132604_d_n0, assign86920_e132604_d_n2, assign86920_e132604_d_n4, assign86920_e132604_d_n5, assign86920_e132604_d_n6, assign86920_e132604_d_n7, assign86920_e132604_d_n8, assign86920_e132604_d_n9, assign86920_e132604_d_n10, assign86920_e132604_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 != 0.0)) {
        let assign86920_e132601: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign86920_e132602: f64 = (locals.var_beta * assign86920_e132601);
        (assign86920_e132602, ((locals.var_beta_dn0 * assign86920_e132601) + (locals.var_beta * locals.var_vxbgmtcl_dn0)), ((locals.var_beta_dn2 * assign86920_e132601) + (locals.var_beta * (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2))), ((locals.var_beta_dn4 * assign86920_e132601) + (locals.var_beta * locals.var_vxbgmtcl_dn4)), ((locals.var_beta_dn5 * assign86920_e132601) + (locals.var_beta * locals.var_vxbgmtcl_dn5)), ((locals.var_beta_dn6 * assign86920_e132601) + (locals.var_beta * (locals.var_vgpld_dn6 + locals.var_vxbgmtcl_dn6))), ((locals.var_beta_dn7 * assign86920_e132601) + (locals.var_beta * (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7))), ((locals.var_beta_dn8 * assign86920_e132601) + (locals.var_beta * (locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8))), ((locals.var_beta_dn9 * assign86920_e132601) + (locals.var_beta * locals.var_vxbgmtcl_dn9)), ((locals.var_beta_dn10 * assign86920_e132601) + (locals.var_beta * locals.var_vxbgmtcl_dn10)), ((locals.var_beta_dn13 * assign86920_e132601) + (locals.var_beta * locals.var_vxbgmtcl_dn13)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn13,)
    }
};
        locals.var_tx = assign86920_e132604;
        locals.var_tx_dn0 = assign86920_e132604_d_n0;
        locals.var_tx_dn2 = assign86920_e132604_d_n2;
        locals.var_tx_dn4 = assign86920_e132604_d_n4;
        locals.var_tx_dn5 = assign86920_e132604_d_n5;
        locals.var_tx_dn6 = assign86920_e132604_d_n6;
        locals.var_tx_dn7 = assign86920_e132604_d_n7;
        locals.var_tx_dn8 = assign86920_e132604_d_n8;
        locals.var_tx_dn9 = assign86920_e132604_d_n9;
        locals.var_tx_dn10 = assign86920_e132604_d_n10;
        locals.var_tx_dn13 = assign86920_e132604_d_n13;
        locals.var_tx_rv = 0.0;

        let (assign86930_e132614, assign86930_e132614_d_n0, assign86930_e132614_d_n2, assign86930_e132614_d_n4, assign86930_e132614_d_n5, assign86930_e132614_d_n6, assign86930_e132614_d_n7, assign86930_e132614_d_n8, assign86930_e132614_d_n9, assign86930_e132614_d_n10, assign86930_e132614_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 != 0.0)) {
        let assign86930_e132611: f64 = (locals.var_beta * locals.var_cnst0over_func);
        let assign86930_e132612: f64 = (1.0 / assign86930_e132611);
        (assign86930_e132612, (-(((locals.var_beta_dn0 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn0)) / (assign86930_e132611 * assign86930_e132611))), (-(((locals.var_beta_dn2 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn2)) / (assign86930_e132611 * assign86930_e132611))), (-(((locals.var_beta_dn4 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn4)) / (assign86930_e132611 * assign86930_e132611))), (-(((locals.var_beta_dn5 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn5)) / (assign86930_e132611 * assign86930_e132611))), (-(((locals.var_beta_dn6 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn6)) / (assign86930_e132611 * assign86930_e132611))), (-(((locals.var_beta_dn7 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn7)) / (assign86930_e132611 * assign86930_e132611))), (-(((locals.var_beta_dn8 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn8)) / (assign86930_e132611 * assign86930_e132611))), (-(((locals.var_beta_dn9 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn9)) / (assign86930_e132611 * assign86930_e132611))), (-(((locals.var_beta_dn10 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn10)) / (assign86930_e132611 * assign86930_e132611))), (-(((locals.var_beta_dn13 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn13)) / (assign86930_e132611 * assign86930_e132611))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign86930_e132614;
        locals.var_t1_dn0 = assign86930_e132614_d_n0;
        locals.var_t1_dn2 = assign86930_e132614_d_n2;
        locals.var_t1_dn4 = assign86930_e132614_d_n4;
        locals.var_t1_dn5 = assign86930_e132614_d_n5;
        locals.var_t1_dn6 = assign86930_e132614_d_n6;
        locals.var_t1_dn7 = assign86930_e132614_d_n7;
        locals.var_t1_dn8 = assign86930_e132614_d_n8;
        locals.var_t1_dn9 = assign86930_e132614_d_n9;
        locals.var_t1_dn10 = assign86930_e132614_d_n10;
        locals.var_t1_dn13 = assign86930_e132614_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign86940_e132622, assign86940_e132622_d_n0, assign86940_e132622_d_n2, assign86940_e132622_d_n4, assign86940_e132622_d_n5, assign86940_e132622_d_n6, assign86940_e132622_d_n7, assign86940_e132622_d_n8, assign86940_e132622_d_n9, assign86940_e132622_d_n10, assign86940_e132622_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 != 0.0)) {
        let assign86940_e132620: f64 = (locals.var_t1 * locals.var_cox0_func);
        (assign86940_e132620, (locals.var_t1_dn0 * locals.var_cox0_func), (locals.var_t1_dn2 * locals.var_cox0_func), (locals.var_t1_dn4 * locals.var_cox0_func), (locals.var_t1_dn5 * locals.var_cox0_func), (locals.var_t1_dn6 * locals.var_cox0_func), (locals.var_t1_dn7 * locals.var_cox0_func), (locals.var_t1_dn8 * locals.var_cox0_func), (locals.var_t1_dn9 * locals.var_cox0_func), (locals.var_t1_dn10 * locals.var_cox0_func), (locals.var_t1_dn13 * locals.var_cox0_func),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn13,)
    }
};
        locals.var_ty = assign86940_e132622;
        locals.var_ty_dn0 = assign86940_e132622_d_n0;
        locals.var_ty_dn2 = assign86940_e132622_d_n2;
        locals.var_ty_dn4 = assign86940_e132622_d_n4;
        locals.var_ty_dn5 = assign86940_e132622_d_n5;
        locals.var_ty_dn6 = assign86940_e132622_d_n6;
        locals.var_ty_dn7 = assign86940_e132622_d_n7;
        locals.var_ty_dn8 = assign86940_e132622_d_n8;
        locals.var_ty_dn9 = assign86940_e132622_d_n9;
        locals.var_ty_dn10 = assign86940_e132622_d_n10;
        locals.var_ty_dn13 = assign86940_e132622_d_n13;
        locals.var_ty_rv = 0.0;

        let (assign86950_e132634, assign86950_e132634_d_n0, assign86950_e132634_d_n2, assign86950_e132634_d_n4, assign86950_e132634_d_n5, assign86950_e132634_d_n6, assign86950_e132634_d_n7, assign86950_e132634_d_n8, assign86950_e132634_d_n9, assign86950_e132634_d_n10, assign86950_e132634_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 != 0.0)) {
        let assign86950_e132629: f64 = (3.0 * 1.414213562373095);
        let assign86950_e132631: f64 = (assign86950_e132629 * locals.var_ty);
        let assign86950_e132632: f64 = (2.0 + assign86950_e132631);
        (assign86950_e132632, (assign86950_e132629 * locals.var_ty_dn0), (assign86950_e132629 * locals.var_ty_dn2), (assign86950_e132629 * locals.var_ty_dn4), (assign86950_e132629 * locals.var_ty_dn5), (assign86950_e132629 * locals.var_ty_dn6), (assign86950_e132629 * locals.var_ty_dn7), (assign86950_e132629 * locals.var_ty_dn8), (assign86950_e132629 * locals.var_ty_dn9), (assign86950_e132629 * locals.var_ty_dn10), (assign86950_e132629 * locals.var_ty_dn13),)
    } else {
        (locals.var_ac41, locals.var_ac41_dn0, locals.var_ac41_dn2, locals.var_ac41_dn4, locals.var_ac41_dn5, locals.var_ac41_dn6, locals.var_ac41_dn7, locals.var_ac41_dn8, locals.var_ac41_dn9, locals.var_ac41_dn10, locals.var_ac41_dn13,)
    }
};
        locals.var_ac41 = assign86950_e132634;
        locals.var_ac41_dn0 = assign86950_e132634_d_n0;
        locals.var_ac41_dn2 = assign86950_e132634_d_n2;
        locals.var_ac41_dn4 = assign86950_e132634_d_n4;
        locals.var_ac41_dn5 = assign86950_e132634_d_n5;
        locals.var_ac41_dn6 = assign86950_e132634_d_n6;
        locals.var_ac41_dn7 = assign86950_e132634_d_n7;
        locals.var_ac41_dn8 = assign86950_e132634_d_n8;
        locals.var_ac41_dn9 = assign86950_e132634_d_n9;
        locals.var_ac41_dn10 = assign86950_e132634_d_n10;
        locals.var_ac41_dn13 = assign86950_e132634_d_n13;
        locals.var_ac41_rv = 0.0;

        let (assign86960_e132646, assign86960_e132646_d_n0, assign86960_e132646_d_n2, assign86960_e132646_d_n4, assign86960_e132646_d_n5, assign86960_e132646_d_n6, assign86960_e132646_d_n7, assign86960_e132646_d_n8, assign86960_e132646_d_n9, assign86960_e132646_d_n10, assign86960_e132646_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 != 0.0)) {
        let assign86960_e132640: f64 = (8.0 * locals.var_ac41);
        let assign86960_e132642: f64 = (assign86960_e132640 * locals.var_ac41);
        let assign86960_e132644: f64 = (assign86960_e132642 * locals.var_ac41);
        (assign86960_e132644, (((((8.0 * locals.var_ac41_dn0) * locals.var_ac41) + (assign86960_e132640 * locals.var_ac41_dn0)) * locals.var_ac41) + (assign86960_e132642 * locals.var_ac41_dn0)), (((((8.0 * locals.var_ac41_dn2) * locals.var_ac41) + (assign86960_e132640 * locals.var_ac41_dn2)) * locals.var_ac41) + (assign86960_e132642 * locals.var_ac41_dn2)), (((((8.0 * locals.var_ac41_dn4) * locals.var_ac41) + (assign86960_e132640 * locals.var_ac41_dn4)) * locals.var_ac41) + (assign86960_e132642 * locals.var_ac41_dn4)), (((((8.0 * locals.var_ac41_dn5) * locals.var_ac41) + (assign86960_e132640 * locals.var_ac41_dn5)) * locals.var_ac41) + (assign86960_e132642 * locals.var_ac41_dn5)), (((((8.0 * locals.var_ac41_dn6) * locals.var_ac41) + (assign86960_e132640 * locals.var_ac41_dn6)) * locals.var_ac41) + (assign86960_e132642 * locals.var_ac41_dn6)), (((((8.0 * locals.var_ac41_dn7) * locals.var_ac41) + (assign86960_e132640 * locals.var_ac41_dn7)) * locals.var_ac41) + (assign86960_e132642 * locals.var_ac41_dn7)), (((((8.0 * locals.var_ac41_dn8) * locals.var_ac41) + (assign86960_e132640 * locals.var_ac41_dn8)) * locals.var_ac41) + (assign86960_e132642 * locals.var_ac41_dn8)), (((((8.0 * locals.var_ac41_dn9) * locals.var_ac41) + (assign86960_e132640 * locals.var_ac41_dn9)) * locals.var_ac41) + (assign86960_e132642 * locals.var_ac41_dn9)), (((((8.0 * locals.var_ac41_dn10) * locals.var_ac41) + (assign86960_e132640 * locals.var_ac41_dn10)) * locals.var_ac41) + (assign86960_e132642 * locals.var_ac41_dn10)), (((((8.0 * locals.var_ac41_dn13) * locals.var_ac41) + (assign86960_e132640 * locals.var_ac41_dn13)) * locals.var_ac41) + (assign86960_e132642 * locals.var_ac41_dn13)),)
    } else {
        (locals.var_ac4, locals.var_ac4_dn0, locals.var_ac4_dn2, locals.var_ac4_dn4, locals.var_ac4_dn5, locals.var_ac4_dn6, locals.var_ac4_dn7, locals.var_ac4_dn8, locals.var_ac4_dn9, locals.var_ac4_dn10, locals.var_ac4_dn13,)
    }
};
        locals.var_ac4 = assign86960_e132646;
        locals.var_ac4_dn0 = assign86960_e132646_d_n0;
        locals.var_ac4_dn2 = assign86960_e132646_d_n2;
        locals.var_ac4_dn4 = assign86960_e132646_d_n4;
        locals.var_ac4_dn5 = assign86960_e132646_d_n5;
        locals.var_ac4_dn6 = assign86960_e132646_d_n6;
        locals.var_ac4_dn7 = assign86960_e132646_d_n7;
        locals.var_ac4_dn8 = assign86960_e132646_d_n8;
        locals.var_ac4_dn9 = assign86960_e132646_d_n9;
        locals.var_ac4_dn10 = assign86960_e132646_d_n10;
        locals.var_ac4_dn13 = assign86960_e132646_d_n13;
        locals.var_ac4_rv = 0.0;

        let (assign86970_e132662, assign86970_e132662_d_n0, assign86970_e132662_d_n2, assign86970_e132662_d_n4, assign86970_e132662_d_n5, assign86970_e132662_d_n6, assign86970_e132662_d_n7, assign86970_e132662_d_n8, assign86970_e132662_d_n9, assign86970_e132662_d_n10, assign86970_e132662_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 != 0.0)) {
        let assign86970_e132652: f64 = (7.0 * 1.414213562373095);
        let assign86970_e132655: f64 = (9.0 * locals.var_ty);
        let assign86970_e132658: f64 = (locals.var_tx - 2.0);
        let assign86970_e132659: f64 = (assign86970_e132655 * assign86970_e132658);
        let assign86970_e132660: f64 = (assign86970_e132652 - assign86970_e132659);
        (assign86970_e132660, (-(((9.0 * locals.var_ty_dn0) * assign86970_e132658) + (assign86970_e132655 * locals.var_tx_dn0))), (-(((9.0 * locals.var_ty_dn2) * assign86970_e132658) + (assign86970_e132655 * locals.var_tx_dn2))), (-(((9.0 * locals.var_ty_dn4) * assign86970_e132658) + (assign86970_e132655 * locals.var_tx_dn4))), (-(((9.0 * locals.var_ty_dn5) * assign86970_e132658) + (assign86970_e132655 * locals.var_tx_dn5))), (-(((9.0 * locals.var_ty_dn6) * assign86970_e132658) + (assign86970_e132655 * locals.var_tx_dn6))), (-(((9.0 * locals.var_ty_dn7) * assign86970_e132658) + (assign86970_e132655 * locals.var_tx_dn7))), (-(((9.0 * locals.var_ty_dn8) * assign86970_e132658) + (assign86970_e132655 * locals.var_tx_dn8))), (-(((9.0 * locals.var_ty_dn9) * assign86970_e132658) + (assign86970_e132655 * locals.var_tx_dn9))), (-(((9.0 * locals.var_ty_dn10) * assign86970_e132658) + (assign86970_e132655 * locals.var_tx_dn10))), (-(((9.0 * locals.var_ty_dn13) * assign86970_e132658) + (assign86970_e132655 * locals.var_tx_dn13))),)
    } else {
        (locals.var_ac31, locals.var_ac31_dn0, locals.var_ac31_dn2, locals.var_ac31_dn4, locals.var_ac31_dn5, locals.var_ac31_dn6, locals.var_ac31_dn7, locals.var_ac31_dn8, locals.var_ac31_dn9, locals.var_ac31_dn10, locals.var_ac31_dn13,)
    }
};
        locals.var_ac31 = assign86970_e132662;
        locals.var_ac31_dn0 = assign86970_e132662_d_n0;
        locals.var_ac31_dn2 = assign86970_e132662_d_n2;
        locals.var_ac31_dn4 = assign86970_e132662_d_n4;
        locals.var_ac31_dn5 = assign86970_e132662_d_n5;
        locals.var_ac31_dn6 = assign86970_e132662_d_n6;
        locals.var_ac31_dn7 = assign86970_e132662_d_n7;
        locals.var_ac31_dn8 = assign86970_e132662_d_n8;
        locals.var_ac31_dn9 = assign86970_e132662_d_n9;
        locals.var_ac31_dn10 = assign86970_e132662_d_n10;
        locals.var_ac31_dn13 = assign86970_e132662_d_n13;
        locals.var_ac31_rv = 0.0;

        let (assign86980_e132670, assign86980_e132670_d_n0, assign86980_e132670_d_n2, assign86980_e132670_d_n4, assign86980_e132670_d_n5, assign86980_e132670_d_n6, assign86980_e132670_d_n7, assign86980_e132670_d_n8, assign86980_e132670_d_n9, assign86980_e132670_d_n10, assign86980_e132670_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 != 0.0)) {
        let assign86980_e132668: f64 = (locals.var_ac31 * locals.var_ac31);
        (assign86980_e132668, ((locals.var_ac31_dn0 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn0)), ((locals.var_ac31_dn2 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn2)), ((locals.var_ac31_dn4 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn4)), ((locals.var_ac31_dn5 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn5)), ((locals.var_ac31_dn6 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn6)), ((locals.var_ac31_dn7 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn7)), ((locals.var_ac31_dn8 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn8)), ((locals.var_ac31_dn9 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn9)), ((locals.var_ac31_dn10 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn10)), ((locals.var_ac31_dn13 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn13)),)
    } else {
        (locals.var_ac3, locals.var_ac3_dn0, locals.var_ac3_dn2, locals.var_ac3_dn4, locals.var_ac3_dn5, locals.var_ac3_dn6, locals.var_ac3_dn7, locals.var_ac3_dn8, locals.var_ac3_dn9, locals.var_ac3_dn10, locals.var_ac3_dn13,)
    }
};
        locals.var_ac3 = assign86980_e132670;
        locals.var_ac3_dn0 = assign86980_e132670_d_n0;
        locals.var_ac3_dn2 = assign86980_e132670_d_n2;
        locals.var_ac3_dn4 = assign86980_e132670_d_n4;
        locals.var_ac3_dn5 = assign86980_e132670_d_n5;
        locals.var_ac3_dn6 = assign86980_e132670_d_n6;
        locals.var_ac3_dn7 = assign86980_e132670_d_n7;
        locals.var_ac3_dn8 = assign86980_e132670_d_n8;
        locals.var_ac3_dn9 = assign86980_e132670_d_n9;
        locals.var_ac3_dn10 = assign86980_e132670_d_n10;
        locals.var_ac3_dn13 = assign86980_e132670_d_n13;
        locals.var_ac3_rv = 0.0;

        let assign86990_e132674: f64 = (locals.var_ac3 * 1e-8);
        let assign86990_e132675: f64 = if locals.var_ac4 < assign86990_e132674 { 1.0 } else { 0.0 };
        locals.var_guard2029 = assign86990_e132675;
        locals.var_guard2029_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_322(
        locals: &mut StampLocals,
    ) {
        let (assign87010_e132696, assign87010_e132696_d_n0, assign87010_e132696_d_n2, assign87010_e132696_d_n4, assign87010_e132696_d_n5, assign87010_e132696_d_n6, assign87010_e132696_d_n7, assign87010_e132696_d_n8, assign87010_e132696_d_n9, assign87010_e132696_d_n10, assign87010_e132696_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 != 0.0)) && (locals.var_guard2029 != 0.0)) {
        let assign87010_e132692: f64 = (0.5 * locals.var_ac4);
        let assign87010_e132694: f64 = (assign87010_e132692 / locals.var_ac31);
        (assign87010_e132694, ((((0.5 * locals.var_ac4_dn0) * locals.var_ac31) - (assign87010_e132692 * locals.var_ac31_dn0)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn2) * locals.var_ac31) - (assign87010_e132692 * locals.var_ac31_dn2)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn4) * locals.var_ac31) - (assign87010_e132692 * locals.var_ac31_dn4)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn5) * locals.var_ac31) - (assign87010_e132692 * locals.var_ac31_dn5)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn6) * locals.var_ac31) - (assign87010_e132692 * locals.var_ac31_dn6)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn7) * locals.var_ac31) - (assign87010_e132692 * locals.var_ac31_dn7)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn8) * locals.var_ac31) - (assign87010_e132692 * locals.var_ac31_dn8)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn9) * locals.var_ac31) - (assign87010_e132692 * locals.var_ac31_dn9)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn10) * locals.var_ac31) - (assign87010_e132692 * locals.var_ac31_dn10)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn13) * locals.var_ac31) - (assign87010_e132692 * locals.var_ac31_dn13)) / (locals.var_ac31 * locals.var_ac31)),)
    } else {
        (locals.var_ac1, locals.var_ac1_dn0, locals.var_ac1_dn2, locals.var_ac1_dn4, locals.var_ac1_dn5, locals.var_ac1_dn6, locals.var_ac1_dn7, locals.var_ac1_dn8, locals.var_ac1_dn9, locals.var_ac1_dn10, locals.var_ac1_dn13,)
    }
};
        locals.var_ac1 = assign87010_e132696;
        locals.var_ac1_dn0 = assign87010_e132696_d_n0;
        locals.var_ac1_dn2 = assign87010_e132696_d_n2;
        locals.var_ac1_dn4 = assign87010_e132696_d_n4;
        locals.var_ac1_dn5 = assign87010_e132696_d_n5;
        locals.var_ac1_dn6 = assign87010_e132696_d_n6;
        locals.var_ac1_dn7 = assign87010_e132696_d_n7;
        locals.var_ac1_dn8 = assign87010_e132696_d_n8;
        locals.var_ac1_dn9 = assign87010_e132696_d_n9;
        locals.var_ac1_dn10 = assign87010_e132696_d_n10;
        locals.var_ac1_dn13 = assign87010_e132696_d_n13;
        locals.var_ac1_rv = 0.0;

        let (assign87020_e132708, assign87020_e132708_d_n0, assign87020_e132708_d_n2, assign87020_e132708_d_n4, assign87020_e132708_d_n5, assign87020_e132708_d_n6, assign87020_e132708_d_n7, assign87020_e132708_d_n8, assign87020_e132708_d_n9, assign87020_e132708_d_n10, assign87020_e132708_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 != 0.0)) && (locals.var_guard2029 == 0.0)) {
        let assign87020_e132705: f64 = (locals.var_ac4 + locals.var_ac3);
        let assign87020_e132706: f64 = (assign87020_e132705).sqrt();
        (assign87020_e132706, ((locals.var_ac4_dn0 + locals.var_ac3_dn0) / (2.0 * assign87020_e132706)), ((locals.var_ac4_dn2 + locals.var_ac3_dn2) / (2.0 * assign87020_e132706)), ((locals.var_ac4_dn4 + locals.var_ac3_dn4) / (2.0 * assign87020_e132706)), ((locals.var_ac4_dn5 + locals.var_ac3_dn5) / (2.0 * assign87020_e132706)), ((locals.var_ac4_dn6 + locals.var_ac3_dn6) / (2.0 * assign87020_e132706)), ((locals.var_ac4_dn7 + locals.var_ac3_dn7) / (2.0 * assign87020_e132706)), ((locals.var_ac4_dn8 + locals.var_ac3_dn8) / (2.0 * assign87020_e132706)), ((locals.var_ac4_dn9 + locals.var_ac3_dn9) / (2.0 * assign87020_e132706)), ((locals.var_ac4_dn10 + locals.var_ac3_dn10) / (2.0 * assign87020_e132706)), ((locals.var_ac4_dn13 + locals.var_ac3_dn13) / (2.0 * assign87020_e132706)),)
    } else {
        (locals.var_ac2, locals.var_ac2_dn0, locals.var_ac2_dn2, locals.var_ac2_dn4, locals.var_ac2_dn5, locals.var_ac2_dn6, locals.var_ac2_dn7, locals.var_ac2_dn8, locals.var_ac2_dn9, locals.var_ac2_dn10, locals.var_ac2_dn13,)
    }
};
        locals.var_ac2 = assign87020_e132708;
        locals.var_ac2_dn0 = assign87020_e132708_d_n0;
        locals.var_ac2_dn2 = assign87020_e132708_d_n2;
        locals.var_ac2_dn4 = assign87020_e132708_d_n4;
        locals.var_ac2_dn5 = assign87020_e132708_d_n5;
        locals.var_ac2_dn6 = assign87020_e132708_d_n6;
        locals.var_ac2_dn7 = assign87020_e132708_d_n7;
        locals.var_ac2_dn8 = assign87020_e132708_d_n8;
        locals.var_ac2_dn9 = assign87020_e132708_d_n9;
        locals.var_ac2_dn10 = assign87020_e132708_d_n10;
        locals.var_ac2_dn13 = assign87020_e132708_d_n13;
        locals.var_ac2_rv = 0.0;

        let (assign87030_e132720, assign87030_e132720_d_n0, assign87030_e132720_d_n2, assign87030_e132720_d_n4, assign87030_e132720_d_n5, assign87030_e132720_d_n6, assign87030_e132720_d_n7, assign87030_e132720_d_n8, assign87030_e132720_d_n9, assign87030_e132720_d_n10, assign87030_e132720_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 != 0.0)) && (locals.var_guard2029 == 0.0)) {
        let assign87030_e132716: f64 = (-locals.var_ac31);
        let assign87030_e132718: f64 = (assign87030_e132716 + locals.var_ac2);
        (assign87030_e132718, ((-locals.var_ac31_dn0) + locals.var_ac2_dn0), ((-locals.var_ac31_dn2) + locals.var_ac2_dn2), ((-locals.var_ac31_dn4) + locals.var_ac2_dn4), ((-locals.var_ac31_dn5) + locals.var_ac2_dn5), ((-locals.var_ac31_dn6) + locals.var_ac2_dn6), ((-locals.var_ac31_dn7) + locals.var_ac2_dn7), ((-locals.var_ac31_dn8) + locals.var_ac2_dn8), ((-locals.var_ac31_dn9) + locals.var_ac2_dn9), ((-locals.var_ac31_dn10) + locals.var_ac2_dn10), ((-locals.var_ac31_dn13) + locals.var_ac2_dn13),)
    } else {
        (locals.var_ac1, locals.var_ac1_dn0, locals.var_ac1_dn2, locals.var_ac1_dn4, locals.var_ac1_dn5, locals.var_ac1_dn6, locals.var_ac1_dn7, locals.var_ac1_dn8, locals.var_ac1_dn9, locals.var_ac1_dn10, locals.var_ac1_dn13,)
    }
};
        locals.var_ac1 = assign87030_e132720;
        locals.var_ac1_dn0 = assign87030_e132720_d_n0;
        locals.var_ac1_dn2 = assign87030_e132720_d_n2;
        locals.var_ac1_dn4 = assign87030_e132720_d_n4;
        locals.var_ac1_dn5 = assign87030_e132720_d_n5;
        locals.var_ac1_dn6 = assign87030_e132720_d_n6;
        locals.var_ac1_dn7 = assign87030_e132720_d_n7;
        locals.var_ac1_dn8 = assign87030_e132720_d_n8;
        locals.var_ac1_dn9 = assign87030_e132720_d_n9;
        locals.var_ac1_dn10 = assign87030_e132720_d_n10;
        locals.var_ac1_dn13 = assign87030_e132720_d_n13;
        locals.var_ac1_rv = 0.0;

        let (assign87040_e132728, assign87040_e132728_d_n0, assign87040_e132728_d_n2, assign87040_e132728_d_n4, assign87040_e132728_d_n5, assign87040_e132728_d_n6, assign87040_e132728_d_n7, assign87040_e132728_d_n8, assign87040_e132728_d_n9, assign87040_e132728_d_n10, assign87040_e132728_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 != 0.0)) {
        let assign87040_e132726: f64 = (locals.var_ac1).powf(0.3333333333333333);
        (assign87040_e132726, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn0)) } } else { (assign87040_e132726 * (0.3333333333333333 * (locals.var_ac1_dn0 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn2)) } } else { (assign87040_e132726 * (0.3333333333333333 * (locals.var_ac1_dn2 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn4)) } } else { (assign87040_e132726 * (0.3333333333333333 * (locals.var_ac1_dn4 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn5)) } } else { (assign87040_e132726 * (0.3333333333333333 * (locals.var_ac1_dn5 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn6)) } } else { (assign87040_e132726 * (0.3333333333333333 * (locals.var_ac1_dn6 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn7)) } } else { (assign87040_e132726 * (0.3333333333333333 * (locals.var_ac1_dn7 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn8)) } } else { (assign87040_e132726 * (0.3333333333333333 * (locals.var_ac1_dn8 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn9)) } } else { (assign87040_e132726 * (0.3333333333333333 * (locals.var_ac1_dn9 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn10)) } } else { (assign87040_e132726 * (0.3333333333333333 * (locals.var_ac1_dn10 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn13)) } } else { (assign87040_e132726 * (0.3333333333333333 * (locals.var_ac1_dn13 / locals.var_ac1))) },)
    } else {
        (locals.var_acd, locals.var_acd_dn0, locals.var_acd_dn2, locals.var_acd_dn4, locals.var_acd_dn5, locals.var_acd_dn6, locals.var_acd_dn7, locals.var_acd_dn8, locals.var_acd_dn9, locals.var_acd_dn10, locals.var_acd_dn13,)
    }
};
        locals.var_acd = assign87040_e132728;
        locals.var_acd_dn0 = assign87040_e132728_d_n0;
        locals.var_acd_dn2 = assign87040_e132728_d_n2;
        locals.var_acd_dn4 = assign87040_e132728_d_n4;
        locals.var_acd_dn5 = assign87040_e132728_d_n5;
        locals.var_acd_dn6 = assign87040_e132728_d_n6;
        locals.var_acd_dn7 = assign87040_e132728_d_n7;
        locals.var_acd_dn8 = assign87040_e132728_d_n8;
        locals.var_acd_dn9 = assign87040_e132728_d_n9;
        locals.var_acd_dn10 = assign87040_e132728_d_n10;
        locals.var_acd_dn13 = assign87040_e132728_d_n13;
        locals.var_acd_rv = 0.0;

        let (assign87050_e132751, assign87050_e132751_d_n0, assign87050_e132751_d_n2, assign87050_e132751_d_n4, assign87050_e132751_d_n5, assign87050_e132751_d_n6, assign87050_e132751_d_n7, assign87050_e132751_d_n8, assign87050_e132751_d_n9, assign87050_e132751_d_n10, assign87050_e132751_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 != 0.0)) {
        let assign87050_e132733: f64 = (-4.0);
        let assign87050_e132735: f64 = (assign87050_e132733 * 1.414213562373095);
        let assign87050_e132738: f64 = (12.0 * locals.var_ty);
        let assign87050_e132739: f64 = (assign87050_e132735 - assign87050_e132738);
        let assign87050_e132742: f64 = (2.0 * locals.var_acd);
        let assign87050_e132743: f64 = (assign87050_e132739 + assign87050_e132742);
        let assign87050_e132746: f64 = (1.414213562373095 * locals.var_acd);
        let assign87050_e132748: f64 = (assign87050_e132746 * locals.var_acd);
        let assign87050_e132749: f64 = (assign87050_e132743 + assign87050_e132748);
        (assign87050_e132749, (((-(12.0 * locals.var_ty_dn0)) + (2.0 * locals.var_acd_dn0)) + (((1.414213562373095 * locals.var_acd_dn0) * locals.var_acd) + (assign87050_e132746 * locals.var_acd_dn0))), (((-(12.0 * locals.var_ty_dn2)) + (2.0 * locals.var_acd_dn2)) + (((1.414213562373095 * locals.var_acd_dn2) * locals.var_acd) + (assign87050_e132746 * locals.var_acd_dn2))), (((-(12.0 * locals.var_ty_dn4)) + (2.0 * locals.var_acd_dn4)) + (((1.414213562373095 * locals.var_acd_dn4) * locals.var_acd) + (assign87050_e132746 * locals.var_acd_dn4))), (((-(12.0 * locals.var_ty_dn5)) + (2.0 * locals.var_acd_dn5)) + (((1.414213562373095 * locals.var_acd_dn5) * locals.var_acd) + (assign87050_e132746 * locals.var_acd_dn5))), (((-(12.0 * locals.var_ty_dn6)) + (2.0 * locals.var_acd_dn6)) + (((1.414213562373095 * locals.var_acd_dn6) * locals.var_acd) + (assign87050_e132746 * locals.var_acd_dn6))), (((-(12.0 * locals.var_ty_dn7)) + (2.0 * locals.var_acd_dn7)) + (((1.414213562373095 * locals.var_acd_dn7) * locals.var_acd) + (assign87050_e132746 * locals.var_acd_dn7))), (((-(12.0 * locals.var_ty_dn8)) + (2.0 * locals.var_acd_dn8)) + (((1.414213562373095 * locals.var_acd_dn8) * locals.var_acd) + (assign87050_e132746 * locals.var_acd_dn8))), (((-(12.0 * locals.var_ty_dn9)) + (2.0 * locals.var_acd_dn9)) + (((1.414213562373095 * locals.var_acd_dn9) * locals.var_acd) + (assign87050_e132746 * locals.var_acd_dn9))), (((-(12.0 * locals.var_ty_dn10)) + (2.0 * locals.var_acd_dn10)) + (((1.414213562373095 * locals.var_acd_dn10) * locals.var_acd) + (assign87050_e132746 * locals.var_acd_dn10))), (((-(12.0 * locals.var_ty_dn13)) + (2.0 * locals.var_acd_dn13)) + (((1.414213562373095 * locals.var_acd_dn13) * locals.var_acd) + (assign87050_e132746 * locals.var_acd_dn13))),)
    } else {
        (locals.var_acn, locals.var_acn_dn0, locals.var_acn_dn2, locals.var_acn_dn4, locals.var_acn_dn5, locals.var_acn_dn6, locals.var_acn_dn7, locals.var_acn_dn8, locals.var_acn_dn9, locals.var_acn_dn10, locals.var_acn_dn13,)
    }
};
        locals.var_acn = assign87050_e132751;
        locals.var_acn_dn0 = assign87050_e132751_d_n0;
        locals.var_acn_dn2 = assign87050_e132751_d_n2;
        locals.var_acn_dn4 = assign87050_e132751_d_n4;
        locals.var_acn_dn5 = assign87050_e132751_d_n5;
        locals.var_acn_dn6 = assign87050_e132751_d_n6;
        locals.var_acn_dn7 = assign87050_e132751_d_n7;
        locals.var_acn_dn8 = assign87050_e132751_d_n8;
        locals.var_acn_dn9 = assign87050_e132751_d_n9;
        locals.var_acn_dn10 = assign87050_e132751_d_n10;
        locals.var_acn_dn13 = assign87050_e132751_d_n13;
        locals.var_acn_rv = 0.0;

        let (assign87060_e132759, assign87060_e132759_d_n0, assign87060_e132759_d_n2, assign87060_e132759_d_n4, assign87060_e132759_d_n5, assign87060_e132759_d_n6, assign87060_e132759_d_n7, assign87060_e132759_d_n8, assign87060_e132759_d_n9, assign87060_e132759_d_n10, assign87060_e132759_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 != 0.0)) {
        let assign87060_e132757: f64 = (locals.var_acn / locals.var_acd);
        (assign87060_e132757, (((locals.var_acn_dn0 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn0)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn2 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn2)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn4 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn4)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn5 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn5)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn6 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn6)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn7 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn7)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn8 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn8)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn9 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn9)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn10 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn10)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn13 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn13)) / (locals.var_acd * locals.var_acd)),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    }
};
        locals.var_chi = assign87060_e132759;
        locals.var_chi_dn0 = assign87060_e132759_d_n0;
        locals.var_chi_dn2 = assign87060_e132759_d_n2;
        locals.var_chi_dn4 = assign87060_e132759_d_n4;
        locals.var_chi_dn5 = assign87060_e132759_d_n5;
        locals.var_chi_dn6 = assign87060_e132759_d_n6;
        locals.var_chi_dn7 = assign87060_e132759_d_n7;
        locals.var_chi_dn8 = assign87060_e132759_d_n8;
        locals.var_chi_dn9 = assign87060_e132759_d_n9;
        locals.var_chi_dn10 = assign87060_e132759_d_n10;
        locals.var_chi_dn13 = assign87060_e132759_d_n13;
        locals.var_chi_rv = 0.0;

        let (assign87070_e132767, assign87070_e132767_d_n0, assign87070_e132767_d_n2, assign87070_e132767_d_n4, assign87070_e132767_d_n5, assign87070_e132767_d_n6, assign87070_e132767_d_n7, assign87070_e132767_d_n8, assign87070_e132767_d_n9, assign87070_e132767_d_n10, assign87070_e132767_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 != 0.0)) {
        let assign87070_e132765: f64 = (locals.var_chi * locals.var_beta_inv);
        (assign87070_e132765, ((locals.var_chi_dn0 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn0)), ((locals.var_chi_dn2 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn2)), ((locals.var_chi_dn4 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn4)), ((locals.var_chi_dn5 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn5)), ((locals.var_chi_dn6 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn6)), ((locals.var_chi_dn7 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn7)), ((locals.var_chi_dn8 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn8)), ((locals.var_chi_dn9 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn9)), ((locals.var_chi_dn10 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn10)), ((locals.var_chi_dn13 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign87070_e132767;
        locals.var_t1_dn0 = assign87070_e132767_d_n0;
        locals.var_t1_dn2 = assign87070_e132767_d_n2;
        locals.var_t1_dn4 = assign87070_e132767_d_n4;
        locals.var_t1_dn5 = assign87070_e132767_d_n5;
        locals.var_t1_dn6 = assign87070_e132767_d_n6;
        locals.var_t1_dn7 = assign87070_e132767_d_n7;
        locals.var_t1_dn8 = assign87070_e132767_d_n8;
        locals.var_t1_dn9 = assign87070_e132767_d_n9;
        locals.var_t1_dn10 = assign87070_e132767_d_n10;
        locals.var_t1_dn13 = assign87070_e132767_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign87080_e132775, assign87080_e132775_d_n0, assign87080_e132775_d_n2, assign87080_e132775_d_n4, assign87080_e132775_d_n5, assign87080_e132775_d_n6, assign87080_e132775_d_n7, assign87080_e132775_d_n8, assign87080_e132775_d_n9, assign87080_e132775_d_n10, assign87080_e132775_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 != 0.0)) {
        let assign87080_e132773: f64 = (locals.var_t1 / locals.var_ps0_min);
        (assign87080_e132773, (((locals.var_t1_dn0 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn0)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn2 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn2)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn4 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn4)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn5 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn5)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn6 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn6)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn7 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn7)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn8 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn8)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn9 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn9)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn10 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn10)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn13 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn13)) / (locals.var_ps0_min * locals.var_ps0_min)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign87080_e132775;
        locals.var_t2_dn0 = assign87080_e132775_d_n0;
        locals.var_t2_dn2 = assign87080_e132775_d_n2;
        locals.var_t2_dn4 = assign87080_e132775_d_n4;
        locals.var_t2_dn5 = assign87080_e132775_d_n5;
        locals.var_t2_dn6 = assign87080_e132775_d_n6;
        locals.var_t2_dn7 = assign87080_e132775_d_n7;
        locals.var_t2_dn8 = assign87080_e132775_d_n8;
        locals.var_t2_dn9 = assign87080_e132775_d_n9;
        locals.var_t2_dn10 = assign87080_e132775_d_n10;
        locals.var_t2_dn13 = assign87080_e132775_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign87090_e132786, assign87090_e132786_d_n0, assign87090_e132786_d_n2, assign87090_e132786_d_n4, assign87090_e132786_d_n5, assign87090_e132786_d_n6, assign87090_e132786_d_n7, assign87090_e132786_d_n8, assign87090_e132786_d_n9, assign87090_e132786_d_n10, assign87090_e132786_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 != 0.0)) {
        let assign87090_e132782: f64 = (locals.var_t2 * locals.var_t2);
        let assign87090_e132783: f64 = (1.0 + assign87090_e132782);
        let assign87090_e132784: f64 = (assign87090_e132783).sqrt();
        (assign87090_e132784, (((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)) / (2.0 * assign87090_e132784)), (((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)) / (2.0 * assign87090_e132784)), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign87090_e132784)), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign87090_e132784)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign87090_e132784)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign87090_e132784)), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign87090_e132784)), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign87090_e132784)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign87090_e132784)), (((locals.var_t2_dn13 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn13)) / (2.0 * assign87090_e132784)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign87090_e132786;
        locals.var_t3_dn0 = assign87090_e132786_d_n0;
        locals.var_t3_dn2 = assign87090_e132786_d_n2;
        locals.var_t3_dn4 = assign87090_e132786_d_n4;
        locals.var_t3_dn5 = assign87090_e132786_d_n5;
        locals.var_t3_dn6 = assign87090_e132786_d_n6;
        locals.var_t3_dn7 = assign87090_e132786_d_n7;
        locals.var_t3_dn8 = assign87090_e132786_d_n8;
        locals.var_t3_dn9 = assign87090_e132786_d_n9;
        locals.var_t3_dn10 = assign87090_e132786_d_n10;
        locals.var_t3_dn13 = assign87090_e132786_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign87100_e132796, assign87100_e132796_d_n0, assign87100_e132796_d_n2, assign87100_e132796_d_n4, assign87100_e132796_d_n5, assign87100_e132796_d_n6, assign87100_e132796_d_n7, assign87100_e132796_d_n8, assign87100_e132796_d_n9, assign87100_e132796_d_n10, assign87100_e132796_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 != 0.0)) {
        let assign87100_e132792: f64 = (locals.var_t1 / locals.var_t3);
        let assign87100_e132794: f64 = (assign87100_e132792 - locals.var_vxbgmtcl);
        (assign87100_e132794, ((((locals.var_t1_dn0 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn0)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn0), ((((locals.var_t1_dn2 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn2)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn2), ((((locals.var_t1_dn4 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn4)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn4), ((((locals.var_t1_dn5 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn5)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn5), ((((locals.var_t1_dn6 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn6)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn6), ((((locals.var_t1_dn7 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn7)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn7), ((((locals.var_t1_dn8 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn8)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn8), ((((locals.var_t1_dn9 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn9)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn9), ((((locals.var_t1_dn10 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn10)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn10), ((((locals.var_t1_dn13 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn13)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn13),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    }
};
        locals.var_ps0ld = assign87100_e132796;
        locals.var_ps0ld_dn0 = assign87100_e132796_d_n0;
        locals.var_ps0ld_dn2 = assign87100_e132796_d_n2;
        locals.var_ps0ld_dn4 = assign87100_e132796_d_n4;
        locals.var_ps0ld_dn5 = assign87100_e132796_d_n5;
        locals.var_ps0ld_dn6 = assign87100_e132796_d_n6;
        locals.var_ps0ld_dn7 = assign87100_e132796_d_n7;
        locals.var_ps0ld_dn8 = assign87100_e132796_d_n8;
        locals.var_ps0ld_dn9 = assign87100_e132796_d_n9;
        locals.var_ps0ld_dn10 = assign87100_e132796_d_n10;
        locals.var_ps0ld_dn13 = assign87100_e132796_d_n13;
        locals.var_ps0ld_rv = 0.0;

        let (assign87110_e132804, assign87110_e132804_d_n0, assign87110_e132804_d_n2, assign87110_e132804_d_n4, assign87110_e132804_d_n5, assign87110_e132804_d_n6, assign87110_e132804_d_n7, assign87110_e132804_d_n8, assign87110_e132804_d_n9, assign87110_e132804_d_n10, assign87110_e132804_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 != 0.0)) {
        let assign87110_e132802: f64 = (locals.var_vgpld - locals.var_ps0ld);
        (assign87110_e132802, (-locals.var_ps0ld_dn0), (locals.var_vgpld_dn2 - locals.var_ps0ld_dn2), (-locals.var_ps0ld_dn4), (-locals.var_ps0ld_dn5), (locals.var_vgpld_dn6 - locals.var_ps0ld_dn6), (locals.var_vgpld_dn7 - locals.var_ps0ld_dn7), (locals.var_vgpld_dn8 - locals.var_ps0ld_dn8), (-locals.var_ps0ld_dn9), (-locals.var_ps0ld_dn10), (-locals.var_ps0ld_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign87110_e132804;
        locals.var_t2_dn0 = assign87110_e132804_d_n0;
        locals.var_t2_dn2 = assign87110_e132804_d_n2;
        locals.var_t2_dn4 = assign87110_e132804_d_n4;
        locals.var_t2_dn5 = assign87110_e132804_d_n5;
        locals.var_t2_dn6 = assign87110_e132804_d_n6;
        locals.var_t2_dn7 = assign87110_e132804_d_n7;
        locals.var_t2_dn8 = assign87110_e132804_d_n8;
        locals.var_t2_dn9 = assign87110_e132804_d_n9;
        locals.var_t2_dn10 = assign87110_e132804_d_n10;
        locals.var_t2_dn13 = assign87110_e132804_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign87120_e132812, assign87120_e132812_d_n0, assign87120_e132812_d_n2, assign87120_e132812_d_n4, assign87120_e132812_d_n5, assign87120_e132812_d_n6, assign87120_e132812_d_n7, assign87120_e132812_d_n8, assign87120_e132812_d_n9, assign87120_e132812_d_n10, assign87120_e132812_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 != 0.0)) {
        let assign87120_e132810: f64 = (locals.var_cox0_func * locals.var_t2);
        (assign87120_e132810, (locals.var_cox0_func * locals.var_t2_dn0), (locals.var_cox0_func * locals.var_t2_dn2), (locals.var_cox0_func * locals.var_t2_dn4), (locals.var_cox0_func * locals.var_t2_dn5), (locals.var_cox0_func * locals.var_t2_dn6), (locals.var_cox0_func * locals.var_t2_dn7), (locals.var_cox0_func * locals.var_t2_dn8), (locals.var_cox0_func * locals.var_t2_dn9), (locals.var_cox0_func * locals.var_t2_dn10), (locals.var_cox0_func * locals.var_t2_dn13),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn4, locals.var_qsuld_dn5, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn8, locals.var_qsuld_dn9, locals.var_qsuld_dn10, locals.var_qsuld_dn13,)
    }
};
        locals.var_qsuld = assign87120_e132812;
        locals.var_qsuld_dn0 = assign87120_e132812_d_n0;
        locals.var_qsuld_dn2 = assign87120_e132812_d_n2;
        locals.var_qsuld_dn4 = assign87120_e132812_d_n4;
        locals.var_qsuld_dn5 = assign87120_e132812_d_n5;
        locals.var_qsuld_dn6 = assign87120_e132812_d_n6;
        locals.var_qsuld_dn7 = assign87120_e132812_d_n7;
        locals.var_qsuld_dn8 = assign87120_e132812_d_n8;
        locals.var_qsuld_dn9 = assign87120_e132812_d_n9;
        locals.var_qsuld_dn10 = assign87120_e132812_d_n10;
        locals.var_qsuld_dn13 = assign87120_e132812_d_n13;
        locals.var_qsuld_rv = 0.0;

        let (assign87130_e132818, assign87130_e132818_d_n0, assign87130_e132818_d_n2, assign87130_e132818_d_n4, assign87130_e132818_d_n5, assign87130_e132818_d_n6, assign87130_e132818_d_n7, assign87130_e132818_d_n8, assign87130_e132818_d_n9, assign87130_e132818_d_n10, assign87130_e132818_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 != 0.0)) {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn4, locals.var_qsuld_dn5, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn8, locals.var_qsuld_dn9, locals.var_qsuld_dn10, locals.var_qsuld_dn13,)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn8, locals.var_qbuld_dn9, locals.var_qbuld_dn10, locals.var_qbuld_dn13,)
    }
};
        locals.var_qbuld = assign87130_e132818;
        locals.var_qbuld_dn0 = assign87130_e132818_d_n0;
        locals.var_qbuld_dn2 = assign87130_e132818_d_n2;
        locals.var_qbuld_dn4 = assign87130_e132818_d_n4;
        locals.var_qbuld_dn5 = assign87130_e132818_d_n5;
        locals.var_qbuld_dn6 = assign87130_e132818_d_n6;
        locals.var_qbuld_dn7 = assign87130_e132818_d_n7;
        locals.var_qbuld_dn8 = assign87130_e132818_d_n8;
        locals.var_qbuld_dn9 = assign87130_e132818_d_n9;
        locals.var_qbuld_dn10 = assign87130_e132818_d_n10;
        locals.var_qbuld_dn13 = assign87130_e132818_d_n13;
        locals.var_qbuld_rv = 0.0;

        let (assign87140_e132824, assign87140_e132824_d_n0, assign87140_e132824_d_n2, assign87140_e132824_d_n4, assign87140_e132824_d_n5, assign87140_e132824_d_n6, assign87140_e132824_d_n7, assign87140_e132824_d_n8, assign87140_e132824_d_n9, assign87140_e132824_d_n10, assign87140_e132824_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 != 0.0)) {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    } else {
        (locals.var_ps0ld_ini__blk2011, locals.var_ps0ld_ini__blk2011_dn0, locals.var_ps0ld_ini__blk2011_dn2, locals.var_ps0ld_ini__blk2011_dn4, locals.var_ps0ld_ini__blk2011_dn5, locals.var_ps0ld_ini__blk2011_dn6, locals.var_ps0ld_ini__blk2011_dn7, locals.var_ps0ld_ini__blk2011_dn8, locals.var_ps0ld_ini__blk2011_dn9, locals.var_ps0ld_ini__blk2011_dn10, locals.var_ps0ld_ini__blk2011_dn13,)
    }
};
        locals.var_ps0ld_ini__blk2011 = assign87140_e132824;
        locals.var_ps0ld_ini__blk2011_dn0 = assign87140_e132824_d_n0;
        locals.var_ps0ld_ini__blk2011_dn2 = assign87140_e132824_d_n2;
        locals.var_ps0ld_ini__blk2011_dn4 = assign87140_e132824_d_n4;
        locals.var_ps0ld_ini__blk2011_dn5 = assign87140_e132824_d_n5;
        locals.var_ps0ld_ini__blk2011_dn6 = assign87140_e132824_d_n6;
        locals.var_ps0ld_ini__blk2011_dn7 = assign87140_e132824_d_n7;
        locals.var_ps0ld_ini__blk2011_dn8 = assign87140_e132824_d_n8;
        locals.var_ps0ld_ini__blk2011_dn9 = assign87140_e132824_d_n9;
        locals.var_ps0ld_ini__blk2011_dn10 = assign87140_e132824_d_n10;
        locals.var_ps0ld_ini__blk2011_dn13 = assign87140_e132824_d_n13;
        locals.var_ps0ld_ini__blk2011_rv = 0.0;

        let assign87150_e132828: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign87150_e132829: f64 = (locals.var_beta * assign87150_e132828);
        let assign87150_e132833: f64 = (10.0 * 2.220446049250313e-16);
        let assign87150_e132835: f64 = (assign87150_e132833 - 1.0);
        let assign87150_e132837: f64 = (assign87150_e132835 * locals.var_fac1p2);
        let assign87150_e132839: f64 = (assign87150_e132837 * locals.var_beta2);
        let assign87150_e132841: f64 = (assign87150_e132839 / 4.0);
        let assign87150_e132842: f64 = (1.0 + assign87150_e132841);
        let assign87150_e132843: f64 = if assign87150_e132829 < assign87150_e132842 { 1.0 } else { 0.0 };
        locals.var_guard2030 = assign87150_e132843;
        locals.var_guard2030_rv = 0.0;

        let (assign87160_e132858, assign87160_e132858_d_n0, assign87160_e132858_d_n2, assign87160_e132858_d_n4, assign87160_e132858_d_n5, assign87160_e132858_d_n6, assign87160_e132858_d_n7, assign87160_e132858_d_n8, assign87160_e132858_d_n9, assign87160_e132858_d_n10, assign87160_e132858_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2030 != 0.0)) {
        let assign87160_e132853: f64 = (locals.var_fac1p2 * locals.var_beta);
        let assign87160_e132855: f64 = (assign87160_e132853 / 2.0);
        let assign87160_e132856: f64 = (locals.var_vgpld + assign87160_e132855);
        (assign87160_e132856, (((locals.var_fac1p2_dn0 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn0)) / 2.0), (locals.var_vgpld_dn2 + (((locals.var_fac1p2_dn2 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn2)) / 2.0)), (((locals.var_fac1p2_dn4 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn4)) / 2.0), (((locals.var_fac1p2_dn5 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn5)) / 2.0), (locals.var_vgpld_dn6 + (((locals.var_fac1p2_dn6 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn6)) / 2.0)), (locals.var_vgpld_dn7 + (((locals.var_fac1p2_dn7 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn7)) / 2.0)), (locals.var_vgpld_dn8 + (((locals.var_fac1p2_dn8 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn8)) / 2.0)), (((locals.var_fac1p2_dn9 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn9)) / 2.0), (((locals.var_fac1p2_dn10 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn10)) / 2.0), (((locals.var_fac1p2_dn13 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn13)) / 2.0),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn13,)
    }
};
        locals.var_ps0_inia = assign87160_e132858;
        locals.var_ps0_inia_dn0 = assign87160_e132858_d_n0;
        locals.var_ps0_inia_dn2 = assign87160_e132858_d_n2;
        locals.var_ps0_inia_dn4 = assign87160_e132858_d_n4;
        locals.var_ps0_inia_dn5 = assign87160_e132858_d_n5;
        locals.var_ps0_inia_dn6 = assign87160_e132858_d_n6;
        locals.var_ps0_inia_dn7 = assign87160_e132858_d_n7;
        locals.var_ps0_inia_dn8 = assign87160_e132858_d_n8;
        locals.var_ps0_inia_dn9 = assign87160_e132858_d_n9;
        locals.var_ps0_inia_dn10 = assign87160_e132858_d_n10;
        locals.var_ps0_inia_dn13 = assign87160_e132858_d_n13;
        locals.var_ps0_inia_rv = 0.0;

        let (assign87170_e132882, assign87170_e132882_d_n0, assign87170_e132882_d_n2, assign87170_e132882_d_n4, assign87170_e132882_d_n5, assign87170_e132882_d_n6, assign87170_e132882_d_n7, assign87170_e132882_d_n8, assign87170_e132882_d_n9, assign87170_e132882_d_n10, assign87170_e132882_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2030 == 0.0)) {
        let assign87170_e132871: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign87170_e132872: f64 = (locals.var_beta * assign87170_e132871);
        let assign87170_e132874: f64 = (assign87170_e132872 - 1.0);
        let assign87170_e132875: f64 = (4.0 * assign87170_e132874);
        let assign87170_e132878: f64 = (locals.var_fac1p2 * locals.var_beta2);
        let assign87170_e132879: f64 = (assign87170_e132875 / assign87170_e132878);
        let assign87170_e132880: f64 = (1.0 + assign87170_e132879);
        (assign87170_e132880, ((((4.0 * ((locals.var_beta_dn0 * assign87170_e132871) + (locals.var_beta * locals.var_vxbgmtcl_dn0))) * assign87170_e132878) - (assign87170_e132875 * ((locals.var_fac1p2_dn0 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn0)))) / (assign87170_e132878 * assign87170_e132878)), ((((4.0 * ((locals.var_beta_dn2 * assign87170_e132871) + (locals.var_beta * (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2)))) * assign87170_e132878) - (assign87170_e132875 * ((locals.var_fac1p2_dn2 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn2)))) / (assign87170_e132878 * assign87170_e132878)), ((((4.0 * ((locals.var_beta_dn4 * assign87170_e132871) + (locals.var_beta * locals.var_vxbgmtcl_dn4))) * assign87170_e132878) - (assign87170_e132875 * ((locals.var_fac1p2_dn4 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn4)))) / (assign87170_e132878 * assign87170_e132878)), ((((4.0 * ((locals.var_beta_dn5 * assign87170_e132871) + (locals.var_beta * locals.var_vxbgmtcl_dn5))) * assign87170_e132878) - (assign87170_e132875 * ((locals.var_fac1p2_dn5 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn5)))) / (assign87170_e132878 * assign87170_e132878)), ((((4.0 * ((locals.var_beta_dn6 * assign87170_e132871) + (locals.var_beta * (locals.var_vgpld_dn6 + locals.var_vxbgmtcl_dn6)))) * assign87170_e132878) - (assign87170_e132875 * ((locals.var_fac1p2_dn6 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn6)))) / (assign87170_e132878 * assign87170_e132878)), ((((4.0 * ((locals.var_beta_dn7 * assign87170_e132871) + (locals.var_beta * (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7)))) * assign87170_e132878) - (assign87170_e132875 * ((locals.var_fac1p2_dn7 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn7)))) / (assign87170_e132878 * assign87170_e132878)), ((((4.0 * ((locals.var_beta_dn8 * assign87170_e132871) + (locals.var_beta * (locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8)))) * assign87170_e132878) - (assign87170_e132875 * ((locals.var_fac1p2_dn8 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn8)))) / (assign87170_e132878 * assign87170_e132878)), ((((4.0 * ((locals.var_beta_dn9 * assign87170_e132871) + (locals.var_beta * locals.var_vxbgmtcl_dn9))) * assign87170_e132878) - (assign87170_e132875 * ((locals.var_fac1p2_dn9 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn9)))) / (assign87170_e132878 * assign87170_e132878)), ((((4.0 * ((locals.var_beta_dn10 * assign87170_e132871) + (locals.var_beta * locals.var_vxbgmtcl_dn10))) * assign87170_e132878) - (assign87170_e132875 * ((locals.var_fac1p2_dn10 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn10)))) / (assign87170_e132878 * assign87170_e132878)), ((((4.0 * ((locals.var_beta_dn13 * assign87170_e132871) + (locals.var_beta * locals.var_vxbgmtcl_dn13))) * assign87170_e132878) - (assign87170_e132875 * ((locals.var_fac1p2_dn13 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn13)))) / (assign87170_e132878 * assign87170_e132878)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn13,)
    }
};
        locals.var_tx = assign87170_e132882;
        locals.var_tx_dn0 = assign87170_e132882_d_n0;
        locals.var_tx_dn2 = assign87170_e132882_d_n2;
        locals.var_tx_dn4 = assign87170_e132882_d_n4;
        locals.var_tx_dn5 = assign87170_e132882_d_n5;
        locals.var_tx_dn6 = assign87170_e132882_d_n6;
        locals.var_tx_dn7 = assign87170_e132882_d_n7;
        locals.var_tx_dn8 = assign87170_e132882_d_n8;
        locals.var_tx_dn9 = assign87170_e132882_d_n9;
        locals.var_tx_dn10 = assign87170_e132882_d_n10;
        locals.var_tx_dn13 = assign87170_e132882_d_n13;
        locals.var_tx_rv = 0.0;

        let (assign87180_e132903, assign87180_e132903_d_n0, assign87180_e132903_d_n2, assign87180_e132903_d_n4, assign87180_e132903_d_n5, assign87180_e132903_d_n6, assign87180_e132903_d_n7, assign87180_e132903_d_n8, assign87180_e132903_d_n9, assign87180_e132903_d_n10, assign87180_e132903_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2030 == 0.0)) {
        let assign87180_e132893: f64 = (locals.var_fac1p2 * locals.var_beta);
        let assign87180_e132895: f64 = (assign87180_e132893 / 2.0);
        let assign87180_e132898: f64 = (locals.var_tx).sqrt();
        let assign87180_e132899: f64 = (1.0 - assign87180_e132898);
        let assign87180_e132900: f64 = (assign87180_e132895 * assign87180_e132899);
        let assign87180_e132901: f64 = (locals.var_vgpld + assign87180_e132900);
        (assign87180_e132901, (((((locals.var_fac1p2_dn0 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn0)) / 2.0) * assign87180_e132899) + (assign87180_e132895 * (-(locals.var_tx_dn0 / (2.0 * assign87180_e132898))))), (locals.var_vgpld_dn2 + (((((locals.var_fac1p2_dn2 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn2)) / 2.0) * assign87180_e132899) + (assign87180_e132895 * (-(locals.var_tx_dn2 / (2.0 * assign87180_e132898)))))), (((((locals.var_fac1p2_dn4 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn4)) / 2.0) * assign87180_e132899) + (assign87180_e132895 * (-(locals.var_tx_dn4 / (2.0 * assign87180_e132898))))), (((((locals.var_fac1p2_dn5 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn5)) / 2.0) * assign87180_e132899) + (assign87180_e132895 * (-(locals.var_tx_dn5 / (2.0 * assign87180_e132898))))), (locals.var_vgpld_dn6 + (((((locals.var_fac1p2_dn6 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn6)) / 2.0) * assign87180_e132899) + (assign87180_e132895 * (-(locals.var_tx_dn6 / (2.0 * assign87180_e132898)))))), (locals.var_vgpld_dn7 + (((((locals.var_fac1p2_dn7 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn7)) / 2.0) * assign87180_e132899) + (assign87180_e132895 * (-(locals.var_tx_dn7 / (2.0 * assign87180_e132898)))))), (locals.var_vgpld_dn8 + (((((locals.var_fac1p2_dn8 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn8)) / 2.0) * assign87180_e132899) + (assign87180_e132895 * (-(locals.var_tx_dn8 / (2.0 * assign87180_e132898)))))), (((((locals.var_fac1p2_dn9 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn9)) / 2.0) * assign87180_e132899) + (assign87180_e132895 * (-(locals.var_tx_dn9 / (2.0 * assign87180_e132898))))), (((((locals.var_fac1p2_dn10 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn10)) / 2.0) * assign87180_e132899) + (assign87180_e132895 * (-(locals.var_tx_dn10 / (2.0 * assign87180_e132898))))), (((((locals.var_fac1p2_dn13 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn13)) / 2.0) * assign87180_e132899) + (assign87180_e132895 * (-(locals.var_tx_dn13 / (2.0 * assign87180_e132898))))),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn13,)
    }
};
        locals.var_ps0_inia = assign87180_e132903;
        locals.var_ps0_inia_dn0 = assign87180_e132903_d_n0;
        locals.var_ps0_inia_dn2 = assign87180_e132903_d_n2;
        locals.var_ps0_inia_dn4 = assign87180_e132903_d_n4;
        locals.var_ps0_inia_dn5 = assign87180_e132903_d_n5;
        locals.var_ps0_inia_dn6 = assign87180_e132903_d_n6;
        locals.var_ps0_inia_dn7 = assign87180_e132903_d_n7;
        locals.var_ps0_inia_dn8 = assign87180_e132903_d_n8;
        locals.var_ps0_inia_dn9 = assign87180_e132903_d_n9;
        locals.var_ps0_inia_dn10 = assign87180_e132903_d_n10;
        locals.var_ps0_inia_dn13 = assign87180_e132903_d_n13;
        locals.var_ps0_inia_rv = 0.0;

        let (assign87190_e132914, assign87190_e132914_d_n0, assign87190_e132914_d_n2, assign87190_e132914_d_n4, assign87190_e132914_d_n5, assign87190_e132914_d_n6, assign87190_e132914_d_n7, assign87190_e132914_d_n8, assign87190_e132914_d_n9, assign87190_e132914_d_n10, assign87190_e132914_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) {
        let assign87190_e132911: f64 = (locals.var_ps0_inia + locals.var_vxbgmtcl);
        let assign87190_e132912: f64 = (locals.var_beta * assign87190_e132911);
        (assign87190_e132912, ((locals.var_beta_dn0 * assign87190_e132911) + (locals.var_beta * (locals.var_ps0_inia_dn0 + locals.var_vxbgmtcl_dn0))), ((locals.var_beta_dn2 * assign87190_e132911) + (locals.var_beta * (locals.var_ps0_inia_dn2 + locals.var_vxbgmtcl_dn2))), ((locals.var_beta_dn4 * assign87190_e132911) + (locals.var_beta * (locals.var_ps0_inia_dn4 + locals.var_vxbgmtcl_dn4))), ((locals.var_beta_dn5 * assign87190_e132911) + (locals.var_beta * (locals.var_ps0_inia_dn5 + locals.var_vxbgmtcl_dn5))), ((locals.var_beta_dn6 * assign87190_e132911) + (locals.var_beta * (locals.var_ps0_inia_dn6 + locals.var_vxbgmtcl_dn6))), ((locals.var_beta_dn7 * assign87190_e132911) + (locals.var_beta * (locals.var_ps0_inia_dn7 + locals.var_vxbgmtcl_dn7))), ((locals.var_beta_dn8 * assign87190_e132911) + (locals.var_beta * (locals.var_ps0_inia_dn8 + locals.var_vxbgmtcl_dn8))), ((locals.var_beta_dn9 * assign87190_e132911) + (locals.var_beta * (locals.var_ps0_inia_dn9 + locals.var_vxbgmtcl_dn9))), ((locals.var_beta_dn10 * assign87190_e132911) + (locals.var_beta * (locals.var_ps0_inia_dn10 + locals.var_vxbgmtcl_dn10))), ((locals.var_beta_dn13 * assign87190_e132911) + (locals.var_beta * (locals.var_ps0_inia_dn13 + locals.var_vxbgmtcl_dn13))),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    }
};
        locals.var_chi = assign87190_e132914;
        locals.var_chi_dn0 = assign87190_e132914_d_n0;
        locals.var_chi_dn2 = assign87190_e132914_d_n2;
        locals.var_chi_dn4 = assign87190_e132914_d_n4;
        locals.var_chi_dn5 = assign87190_e132914_d_n5;
        locals.var_chi_dn6 = assign87190_e132914_d_n6;
        locals.var_chi_dn7 = assign87190_e132914_d_n7;
        locals.var_chi_dn8 = assign87190_e132914_d_n8;
        locals.var_chi_dn9 = assign87190_e132914_d_n9;
        locals.var_chi_dn10 = assign87190_e132914_d_n10;
        locals.var_chi_dn13 = assign87190_e132914_d_n13;
        locals.var_chi_rv = 0.0;

        let assign87200_e132917: f64 = if locals.var_chi >= 3.0 { 1.0 } else { 0.0 };
        locals.var_guard2031 = assign87200_e132917;
        locals.var_guard2031_rv = 0.0;

        let (assign87220_e132937, assign87220_e132937_d_n0, assign87220_e132937_d_n2, assign87220_e132937_d_n4, assign87220_e132937_d_n5, assign87220_e132937_d_n6, assign87220_e132937_d_n7, assign87220_e132937_d_n8, assign87220_e132937_d_n9, assign87220_e132937_d_n10, assign87220_e132937_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2031 != 0.0)) {
        let assign87220_e132934: f64 = (-locals.var_chi);
        let assign87220_e132935: f64 = (assign87220_e132934).exp();
        (assign87220_e132935, (assign87220_e132935 * (-locals.var_chi_dn0)), (assign87220_e132935 * (-locals.var_chi_dn2)), (assign87220_e132935 * (-locals.var_chi_dn4)), (assign87220_e132935 * (-locals.var_chi_dn5)), (assign87220_e132935 * (-locals.var_chi_dn6)), (assign87220_e132935 * (-locals.var_chi_dn7)), (assign87220_e132935 * (-locals.var_chi_dn8)), (assign87220_e132935 * (-locals.var_chi_dn9)), (assign87220_e132935 * (-locals.var_chi_dn10)), (assign87220_e132935 * (-locals.var_chi_dn13)),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn13,)
    }
};
        locals.var_ty = assign87220_e132937;
        locals.var_ty_dn0 = assign87220_e132937_d_n0;
        locals.var_ty_dn2 = assign87220_e132937_d_n2;
        locals.var_ty_dn4 = assign87220_e132937_d_n4;
        locals.var_ty_dn5 = assign87220_e132937_d_n5;
        locals.var_ty_dn6 = assign87220_e132937_d_n6;
        locals.var_ty_dn7 = assign87220_e132937_d_n7;
        locals.var_ty_dn8 = assign87220_e132937_d_n8;
        locals.var_ty_dn9 = assign87220_e132937_d_n9;
        locals.var_ty_dn10 = assign87220_e132937_d_n10;
        locals.var_ty_dn13 = assign87220_e132937_d_n13;
        locals.var_ty_rv = 0.0;

        let (assign87230_e132962, assign87230_e132962_d_n0, assign87230_e132962_d_n2, assign87230_e132962_d_n4, assign87230_e132962_d_n5, assign87230_e132962_d_n6, assign87230_e132962_d_n7, assign87230_e132962_d_n8, assign87230_e132962_d_n9, assign87230_e132962_d_n10, assign87230_e132962_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2031 != 0.0)) {
        let assign87230_e132949: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign87230_e132950: f64 = (locals.var_beta * assign87230_e132949);
        let assign87230_e132952: f64 = (assign87230_e132950 - 1.0);
        let assign87230_e132954: f64 = (assign87230_e132952 + locals.var_ty);
        let assign87230_e132955: f64 = (4.0 * assign87230_e132954);
        let assign87230_e132958: f64 = (locals.var_fac1p2 * locals.var_beta2);
        let assign87230_e132959: f64 = (assign87230_e132955 / assign87230_e132958);
        let assign87230_e132960: f64 = (1.0 + assign87230_e132959);
        (assign87230_e132960, ((((4.0 * (((locals.var_beta_dn0 * assign87230_e132949) + (locals.var_beta * locals.var_vxbgmtcl_dn0)) + locals.var_ty_dn0)) * assign87230_e132958) - (assign87230_e132955 * ((locals.var_fac1p2_dn0 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn0)))) / (assign87230_e132958 * assign87230_e132958)), ((((4.0 * (((locals.var_beta_dn2 * assign87230_e132949) + (locals.var_beta * (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2))) + locals.var_ty_dn2)) * assign87230_e132958) - (assign87230_e132955 * ((locals.var_fac1p2_dn2 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn2)))) / (assign87230_e132958 * assign87230_e132958)), ((((4.0 * (((locals.var_beta_dn4 * assign87230_e132949) + (locals.var_beta * locals.var_vxbgmtcl_dn4)) + locals.var_ty_dn4)) * assign87230_e132958) - (assign87230_e132955 * ((locals.var_fac1p2_dn4 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn4)))) / (assign87230_e132958 * assign87230_e132958)), ((((4.0 * (((locals.var_beta_dn5 * assign87230_e132949) + (locals.var_beta * locals.var_vxbgmtcl_dn5)) + locals.var_ty_dn5)) * assign87230_e132958) - (assign87230_e132955 * ((locals.var_fac1p2_dn5 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn5)))) / (assign87230_e132958 * assign87230_e132958)), ((((4.0 * (((locals.var_beta_dn6 * assign87230_e132949) + (locals.var_beta * (locals.var_vgpld_dn6 + locals.var_vxbgmtcl_dn6))) + locals.var_ty_dn6)) * assign87230_e132958) - (assign87230_e132955 * ((locals.var_fac1p2_dn6 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn6)))) / (assign87230_e132958 * assign87230_e132958)), ((((4.0 * (((locals.var_beta_dn7 * assign87230_e132949) + (locals.var_beta * (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7))) + locals.var_ty_dn7)) * assign87230_e132958) - (assign87230_e132955 * ((locals.var_fac1p2_dn7 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn7)))) / (assign87230_e132958 * assign87230_e132958)), ((((4.0 * (((locals.var_beta_dn8 * assign87230_e132949) + (locals.var_beta * (locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8))) + locals.var_ty_dn8)) * assign87230_e132958) - (assign87230_e132955 * ((locals.var_fac1p2_dn8 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn8)))) / (assign87230_e132958 * assign87230_e132958)), ((((4.0 * (((locals.var_beta_dn9 * assign87230_e132949) + (locals.var_beta * locals.var_vxbgmtcl_dn9)) + locals.var_ty_dn9)) * assign87230_e132958) - (assign87230_e132955 * ((locals.var_fac1p2_dn9 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn9)))) / (assign87230_e132958 * assign87230_e132958)), ((((4.0 * (((locals.var_beta_dn10 * assign87230_e132949) + (locals.var_beta * locals.var_vxbgmtcl_dn10)) + locals.var_ty_dn10)) * assign87230_e132958) - (assign87230_e132955 * ((locals.var_fac1p2_dn10 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn10)))) / (assign87230_e132958 * assign87230_e132958)), ((((4.0 * (((locals.var_beta_dn13 * assign87230_e132949) + (locals.var_beta * locals.var_vxbgmtcl_dn13)) + locals.var_ty_dn13)) * assign87230_e132958) - (assign87230_e132955 * ((locals.var_fac1p2_dn13 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn13)))) / (assign87230_e132958 * assign87230_e132958)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn13,)
    }
};
        locals.var_tx = assign87230_e132962;
        locals.var_tx_dn0 = assign87230_e132962_d_n0;
        locals.var_tx_dn2 = assign87230_e132962_d_n2;
        locals.var_tx_dn4 = assign87230_e132962_d_n4;
        locals.var_tx_dn5 = assign87230_e132962_d_n5;
        locals.var_tx_dn6 = assign87230_e132962_d_n6;
        locals.var_tx_dn7 = assign87230_e132962_d_n7;
        locals.var_tx_dn8 = assign87230_e132962_d_n8;
        locals.var_tx_dn9 = assign87230_e132962_d_n9;
        locals.var_tx_dn10 = assign87230_e132962_d_n10;
        locals.var_tx_dn13 = assign87230_e132962_d_n13;
        locals.var_tx_rv = 0.0;

        let (assign87240_e132982, assign87240_e132982_d_n0, assign87240_e132982_d_n2, assign87240_e132982_d_n4, assign87240_e132982_d_n5, assign87240_e132982_d_n6, assign87240_e132982_d_n7, assign87240_e132982_d_n8, assign87240_e132982_d_n9, assign87240_e132982_d_n10, assign87240_e132982_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2031 != 0.0)) {
        let assign87240_e132972: f64 = (locals.var_fac1p2 * locals.var_beta);
        let assign87240_e132974: f64 = (assign87240_e132972 / 2.0);
        let assign87240_e132977: f64 = (locals.var_tx).sqrt();
        let assign87240_e132978: f64 = (1.0 - assign87240_e132977);
        let assign87240_e132979: f64 = (assign87240_e132974 * assign87240_e132978);
        let assign87240_e132980: f64 = (locals.var_vgpld + assign87240_e132979);
        (assign87240_e132980, (((((locals.var_fac1p2_dn0 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn0)) / 2.0) * assign87240_e132978) + (assign87240_e132974 * (-(locals.var_tx_dn0 / (2.0 * assign87240_e132977))))), (locals.var_vgpld_dn2 + (((((locals.var_fac1p2_dn2 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn2)) / 2.0) * assign87240_e132978) + (assign87240_e132974 * (-(locals.var_tx_dn2 / (2.0 * assign87240_e132977)))))), (((((locals.var_fac1p2_dn4 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn4)) / 2.0) * assign87240_e132978) + (assign87240_e132974 * (-(locals.var_tx_dn4 / (2.0 * assign87240_e132977))))), (((((locals.var_fac1p2_dn5 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn5)) / 2.0) * assign87240_e132978) + (assign87240_e132974 * (-(locals.var_tx_dn5 / (2.0 * assign87240_e132977))))), (locals.var_vgpld_dn6 + (((((locals.var_fac1p2_dn6 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn6)) / 2.0) * assign87240_e132978) + (assign87240_e132974 * (-(locals.var_tx_dn6 / (2.0 * assign87240_e132977)))))), (locals.var_vgpld_dn7 + (((((locals.var_fac1p2_dn7 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn7)) / 2.0) * assign87240_e132978) + (assign87240_e132974 * (-(locals.var_tx_dn7 / (2.0 * assign87240_e132977)))))), (locals.var_vgpld_dn8 + (((((locals.var_fac1p2_dn8 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn8)) / 2.0) * assign87240_e132978) + (assign87240_e132974 * (-(locals.var_tx_dn8 / (2.0 * assign87240_e132977)))))), (((((locals.var_fac1p2_dn9 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn9)) / 2.0) * assign87240_e132978) + (assign87240_e132974 * (-(locals.var_tx_dn9 / (2.0 * assign87240_e132977))))), (((((locals.var_fac1p2_dn10 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn10)) / 2.0) * assign87240_e132978) + (assign87240_e132974 * (-(locals.var_tx_dn10 / (2.0 * assign87240_e132977))))), (((((locals.var_fac1p2_dn13 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn13)) / 2.0) * assign87240_e132978) + (assign87240_e132974 * (-(locals.var_tx_dn13 / (2.0 * assign87240_e132977))))),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn13,)
    }
};
        locals.var_ps0_inia = assign87240_e132982;
        locals.var_ps0_inia_dn0 = assign87240_e132982_d_n0;
        locals.var_ps0_inia_dn2 = assign87240_e132982_d_n2;
        locals.var_ps0_inia_dn4 = assign87240_e132982_d_n4;
        locals.var_ps0_inia_dn5 = assign87240_e132982_d_n5;
        locals.var_ps0_inia_dn6 = assign87240_e132982_d_n6;
        locals.var_ps0_inia_dn7 = assign87240_e132982_d_n7;
        locals.var_ps0_inia_dn8 = assign87240_e132982_d_n8;
        locals.var_ps0_inia_dn9 = assign87240_e132982_d_n9;
        locals.var_ps0_inia_dn10 = assign87240_e132982_d_n10;
        locals.var_ps0_inia_dn13 = assign87240_e132982_d_n13;
        locals.var_ps0_inia_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_323(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign87250_e132995, assign87250_e132995_d_n0, assign87250_e132995_d_n2, assign87250_e132995_d_n4, assign87250_e132995_d_n5, assign87250_e132995_d_n6, assign87250_e132995_d_n7, assign87250_e132995_d_n8, assign87250_e132995_d_n9, assign87250_e132995_d_n10, assign87250_e132995_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2031 != 0.0)) {
        let assign87250_e132992: f64 = (locals.var_ps0_inia + locals.var_vxbgmtcl);
        let assign87250_e132993: f64 = (locals.var_beta * assign87250_e132992);
        (assign87250_e132993, ((locals.var_beta_dn0 * assign87250_e132992) + (locals.var_beta * (locals.var_ps0_inia_dn0 + locals.var_vxbgmtcl_dn0))), ((locals.var_beta_dn2 * assign87250_e132992) + (locals.var_beta * (locals.var_ps0_inia_dn2 + locals.var_vxbgmtcl_dn2))), ((locals.var_beta_dn4 * assign87250_e132992) + (locals.var_beta * (locals.var_ps0_inia_dn4 + locals.var_vxbgmtcl_dn4))), ((locals.var_beta_dn5 * assign87250_e132992) + (locals.var_beta * (locals.var_ps0_inia_dn5 + locals.var_vxbgmtcl_dn5))), ((locals.var_beta_dn6 * assign87250_e132992) + (locals.var_beta * (locals.var_ps0_inia_dn6 + locals.var_vxbgmtcl_dn6))), ((locals.var_beta_dn7 * assign87250_e132992) + (locals.var_beta * (locals.var_ps0_inia_dn7 + locals.var_vxbgmtcl_dn7))), ((locals.var_beta_dn8 * assign87250_e132992) + (locals.var_beta * (locals.var_ps0_inia_dn8 + locals.var_vxbgmtcl_dn8))), ((locals.var_beta_dn9 * assign87250_e132992) + (locals.var_beta * (locals.var_ps0_inia_dn9 + locals.var_vxbgmtcl_dn9))), ((locals.var_beta_dn10 * assign87250_e132992) + (locals.var_beta * (locals.var_ps0_inia_dn10 + locals.var_vxbgmtcl_dn10))), ((locals.var_beta_dn13 * assign87250_e132992) + (locals.var_beta * (locals.var_ps0_inia_dn13 + locals.var_vxbgmtcl_dn13))),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    }
};
        locals.var_chi = assign87250_e132995;
        locals.var_chi_dn0 = assign87250_e132995_d_n0;
        locals.var_chi_dn2 = assign87250_e132995_d_n2;
        locals.var_chi_dn4 = assign87250_e132995_d_n4;
        locals.var_chi_dn5 = assign87250_e132995_d_n5;
        locals.var_chi_dn6 = assign87250_e132995_d_n6;
        locals.var_chi_dn7 = assign87250_e132995_d_n7;
        locals.var_chi_dn8 = assign87250_e132995_d_n8;
        locals.var_chi_dn9 = assign87250_e132995_d_n9;
        locals.var_chi_dn10 = assign87250_e132995_d_n10;
        locals.var_chi_dn13 = assign87250_e132995_d_n13;
        locals.var_chi_rv = 0.0;

        let (assign87260_e133006, assign87260_e133006_d_n0, assign87260_e133006_d_n2, assign87260_e133006_d_n4, assign87260_e133006_d_n5, assign87260_e133006_d_n6, assign87260_e133006_d_n7, assign87260_e133006_d_n8, assign87260_e133006_d_n9, assign87260_e133006_d_n10, assign87260_e133006_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2031 != 0.0)) {
        let assign87260_e133003: f64 = (-locals.var_chi);
        let assign87260_e133004: f64 = (assign87260_e133003).exp();
        (assign87260_e133004, (assign87260_e133004 * (-locals.var_chi_dn0)), (assign87260_e133004 * (-locals.var_chi_dn2)), (assign87260_e133004 * (-locals.var_chi_dn4)), (assign87260_e133004 * (-locals.var_chi_dn5)), (assign87260_e133004 * (-locals.var_chi_dn6)), (assign87260_e133004 * (-locals.var_chi_dn7)), (assign87260_e133004 * (-locals.var_chi_dn8)), (assign87260_e133004 * (-locals.var_chi_dn9)), (assign87260_e133004 * (-locals.var_chi_dn10)), (assign87260_e133004 * (-locals.var_chi_dn13)),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn13,)
    }
};
        locals.var_ty = assign87260_e133006;
        locals.var_ty_dn0 = assign87260_e133006_d_n0;
        locals.var_ty_dn2 = assign87260_e133006_d_n2;
        locals.var_ty_dn4 = assign87260_e133006_d_n4;
        locals.var_ty_dn5 = assign87260_e133006_d_n5;
        locals.var_ty_dn6 = assign87260_e133006_d_n6;
        locals.var_ty_dn7 = assign87260_e133006_d_n7;
        locals.var_ty_dn8 = assign87260_e133006_d_n8;
        locals.var_ty_dn9 = assign87260_e133006_d_n9;
        locals.var_ty_dn10 = assign87260_e133006_d_n10;
        locals.var_ty_dn13 = assign87260_e133006_d_n13;
        locals.var_ty_rv = 0.0;

        let (assign87270_e133031, assign87270_e133031_d_n0, assign87270_e133031_d_n2, assign87270_e133031_d_n4, assign87270_e133031_d_n5, assign87270_e133031_d_n6, assign87270_e133031_d_n7, assign87270_e133031_d_n8, assign87270_e133031_d_n9, assign87270_e133031_d_n10, assign87270_e133031_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2031 != 0.0)) {
        let assign87270_e133018: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign87270_e133019: f64 = (locals.var_beta * assign87270_e133018);
        let assign87270_e133021: f64 = (assign87270_e133019 - 1.0);
        let assign87270_e133023: f64 = (assign87270_e133021 + locals.var_ty);
        let assign87270_e133024: f64 = (4.0 * assign87270_e133023);
        let assign87270_e133027: f64 = (locals.var_fac1p2 * locals.var_beta2);
        let assign87270_e133028: f64 = (assign87270_e133024 / assign87270_e133027);
        let assign87270_e133029: f64 = (1.0 + assign87270_e133028);
        (assign87270_e133029, ((((4.0 * (((locals.var_beta_dn0 * assign87270_e133018) + (locals.var_beta * locals.var_vxbgmtcl_dn0)) + locals.var_ty_dn0)) * assign87270_e133027) - (assign87270_e133024 * ((locals.var_fac1p2_dn0 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn0)))) / (assign87270_e133027 * assign87270_e133027)), ((((4.0 * (((locals.var_beta_dn2 * assign87270_e133018) + (locals.var_beta * (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2))) + locals.var_ty_dn2)) * assign87270_e133027) - (assign87270_e133024 * ((locals.var_fac1p2_dn2 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn2)))) / (assign87270_e133027 * assign87270_e133027)), ((((4.0 * (((locals.var_beta_dn4 * assign87270_e133018) + (locals.var_beta * locals.var_vxbgmtcl_dn4)) + locals.var_ty_dn4)) * assign87270_e133027) - (assign87270_e133024 * ((locals.var_fac1p2_dn4 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn4)))) / (assign87270_e133027 * assign87270_e133027)), ((((4.0 * (((locals.var_beta_dn5 * assign87270_e133018) + (locals.var_beta * locals.var_vxbgmtcl_dn5)) + locals.var_ty_dn5)) * assign87270_e133027) - (assign87270_e133024 * ((locals.var_fac1p2_dn5 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn5)))) / (assign87270_e133027 * assign87270_e133027)), ((((4.0 * (((locals.var_beta_dn6 * assign87270_e133018) + (locals.var_beta * (locals.var_vgpld_dn6 + locals.var_vxbgmtcl_dn6))) + locals.var_ty_dn6)) * assign87270_e133027) - (assign87270_e133024 * ((locals.var_fac1p2_dn6 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn6)))) / (assign87270_e133027 * assign87270_e133027)), ((((4.0 * (((locals.var_beta_dn7 * assign87270_e133018) + (locals.var_beta * (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7))) + locals.var_ty_dn7)) * assign87270_e133027) - (assign87270_e133024 * ((locals.var_fac1p2_dn7 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn7)))) / (assign87270_e133027 * assign87270_e133027)), ((((4.0 * (((locals.var_beta_dn8 * assign87270_e133018) + (locals.var_beta * (locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8))) + locals.var_ty_dn8)) * assign87270_e133027) - (assign87270_e133024 * ((locals.var_fac1p2_dn8 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn8)))) / (assign87270_e133027 * assign87270_e133027)), ((((4.0 * (((locals.var_beta_dn9 * assign87270_e133018) + (locals.var_beta * locals.var_vxbgmtcl_dn9)) + locals.var_ty_dn9)) * assign87270_e133027) - (assign87270_e133024 * ((locals.var_fac1p2_dn9 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn9)))) / (assign87270_e133027 * assign87270_e133027)), ((((4.0 * (((locals.var_beta_dn10 * assign87270_e133018) + (locals.var_beta * locals.var_vxbgmtcl_dn10)) + locals.var_ty_dn10)) * assign87270_e133027) - (assign87270_e133024 * ((locals.var_fac1p2_dn10 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn10)))) / (assign87270_e133027 * assign87270_e133027)), ((((4.0 * (((locals.var_beta_dn13 * assign87270_e133018) + (locals.var_beta * locals.var_vxbgmtcl_dn13)) + locals.var_ty_dn13)) * assign87270_e133027) - (assign87270_e133024 * ((locals.var_fac1p2_dn13 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn13)))) / (assign87270_e133027 * assign87270_e133027)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn13,)
    }
};
        locals.var_tx = assign87270_e133031;
        locals.var_tx_dn0 = assign87270_e133031_d_n0;
        locals.var_tx_dn2 = assign87270_e133031_d_n2;
        locals.var_tx_dn4 = assign87270_e133031_d_n4;
        locals.var_tx_dn5 = assign87270_e133031_d_n5;
        locals.var_tx_dn6 = assign87270_e133031_d_n6;
        locals.var_tx_dn7 = assign87270_e133031_d_n7;
        locals.var_tx_dn8 = assign87270_e133031_d_n8;
        locals.var_tx_dn9 = assign87270_e133031_d_n9;
        locals.var_tx_dn10 = assign87270_e133031_d_n10;
        locals.var_tx_dn13 = assign87270_e133031_d_n13;
        locals.var_tx_rv = 0.0;

        let (assign87280_e133051, assign87280_e133051_d_n0, assign87280_e133051_d_n2, assign87280_e133051_d_n4, assign87280_e133051_d_n5, assign87280_e133051_d_n6, assign87280_e133051_d_n7, assign87280_e133051_d_n8, assign87280_e133051_d_n9, assign87280_e133051_d_n10, assign87280_e133051_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2031 != 0.0)) {
        let assign87280_e133041: f64 = (locals.var_fac1p2 * locals.var_beta);
        let assign87280_e133043: f64 = (assign87280_e133041 / 2.0);
        let assign87280_e133046: f64 = (locals.var_tx).sqrt();
        let assign87280_e133047: f64 = (1.0 - assign87280_e133046);
        let assign87280_e133048: f64 = (assign87280_e133043 * assign87280_e133047);
        let assign87280_e133049: f64 = (locals.var_vgpld + assign87280_e133048);
        (assign87280_e133049, (((((locals.var_fac1p2_dn0 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn0)) / 2.0) * assign87280_e133047) + (assign87280_e133043 * (-(locals.var_tx_dn0 / (2.0 * assign87280_e133046))))), (locals.var_vgpld_dn2 + (((((locals.var_fac1p2_dn2 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn2)) / 2.0) * assign87280_e133047) + (assign87280_e133043 * (-(locals.var_tx_dn2 / (2.0 * assign87280_e133046)))))), (((((locals.var_fac1p2_dn4 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn4)) / 2.0) * assign87280_e133047) + (assign87280_e133043 * (-(locals.var_tx_dn4 / (2.0 * assign87280_e133046))))), (((((locals.var_fac1p2_dn5 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn5)) / 2.0) * assign87280_e133047) + (assign87280_e133043 * (-(locals.var_tx_dn5 / (2.0 * assign87280_e133046))))), (locals.var_vgpld_dn6 + (((((locals.var_fac1p2_dn6 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn6)) / 2.0) * assign87280_e133047) + (assign87280_e133043 * (-(locals.var_tx_dn6 / (2.0 * assign87280_e133046)))))), (locals.var_vgpld_dn7 + (((((locals.var_fac1p2_dn7 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn7)) / 2.0) * assign87280_e133047) + (assign87280_e133043 * (-(locals.var_tx_dn7 / (2.0 * assign87280_e133046)))))), (locals.var_vgpld_dn8 + (((((locals.var_fac1p2_dn8 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn8)) / 2.0) * assign87280_e133047) + (assign87280_e133043 * (-(locals.var_tx_dn8 / (2.0 * assign87280_e133046)))))), (((((locals.var_fac1p2_dn9 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn9)) / 2.0) * assign87280_e133047) + (assign87280_e133043 * (-(locals.var_tx_dn9 / (2.0 * assign87280_e133046))))), (((((locals.var_fac1p2_dn10 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn10)) / 2.0) * assign87280_e133047) + (assign87280_e133043 * (-(locals.var_tx_dn10 / (2.0 * assign87280_e133046))))), (((((locals.var_fac1p2_dn13 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn13)) / 2.0) * assign87280_e133047) + (assign87280_e133043 * (-(locals.var_tx_dn13 / (2.0 * assign87280_e133046))))),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn13,)
    }
};
        locals.var_ps0_inia = assign87280_e133051;
        locals.var_ps0_inia_dn0 = assign87280_e133051_d_n0;
        locals.var_ps0_inia_dn2 = assign87280_e133051_d_n2;
        locals.var_ps0_inia_dn4 = assign87280_e133051_d_n4;
        locals.var_ps0_inia_dn5 = assign87280_e133051_d_n5;
        locals.var_ps0_inia_dn6 = assign87280_e133051_d_n6;
        locals.var_ps0_inia_dn7 = assign87280_e133051_d_n7;
        locals.var_ps0_inia_dn8 = assign87280_e133051_d_n8;
        locals.var_ps0_inia_dn9 = assign87280_e133051_d_n9;
        locals.var_ps0_inia_dn10 = assign87280_e133051_d_n10;
        locals.var_ps0_inia_dn13 = assign87280_e133051_d_n13;
        locals.var_ps0_inia_rv = 0.0;

        let (assign87290_e133064, assign87290_e133064_d_n0, assign87290_e133064_d_n2, assign87290_e133064_d_n4, assign87290_e133064_d_n5, assign87290_e133064_d_n6, assign87290_e133064_d_n7, assign87290_e133064_d_n8, assign87290_e133064_d_n9, assign87290_e133064_d_n10, assign87290_e133064_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2031 != 0.0)) {
        let assign87290_e133061: f64 = (locals.var_ps0_inia + locals.var_vxbgmtcl);
        let assign87290_e133062: f64 = (locals.var_beta * assign87290_e133061);
        (assign87290_e133062, ((locals.var_beta_dn0 * assign87290_e133061) + (locals.var_beta * (locals.var_ps0_inia_dn0 + locals.var_vxbgmtcl_dn0))), ((locals.var_beta_dn2 * assign87290_e133061) + (locals.var_beta * (locals.var_ps0_inia_dn2 + locals.var_vxbgmtcl_dn2))), ((locals.var_beta_dn4 * assign87290_e133061) + (locals.var_beta * (locals.var_ps0_inia_dn4 + locals.var_vxbgmtcl_dn4))), ((locals.var_beta_dn5 * assign87290_e133061) + (locals.var_beta * (locals.var_ps0_inia_dn5 + locals.var_vxbgmtcl_dn5))), ((locals.var_beta_dn6 * assign87290_e133061) + (locals.var_beta * (locals.var_ps0_inia_dn6 + locals.var_vxbgmtcl_dn6))), ((locals.var_beta_dn7 * assign87290_e133061) + (locals.var_beta * (locals.var_ps0_inia_dn7 + locals.var_vxbgmtcl_dn7))), ((locals.var_beta_dn8 * assign87290_e133061) + (locals.var_beta * (locals.var_ps0_inia_dn8 + locals.var_vxbgmtcl_dn8))), ((locals.var_beta_dn9 * assign87290_e133061) + (locals.var_beta * (locals.var_ps0_inia_dn9 + locals.var_vxbgmtcl_dn9))), ((locals.var_beta_dn10 * assign87290_e133061) + (locals.var_beta * (locals.var_ps0_inia_dn10 + locals.var_vxbgmtcl_dn10))), ((locals.var_beta_dn13 * assign87290_e133061) + (locals.var_beta * (locals.var_ps0_inia_dn13 + locals.var_vxbgmtcl_dn13))),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    }
};
        locals.var_chi = assign87290_e133064;
        locals.var_chi_dn0 = assign87290_e133064_d_n0;
        locals.var_chi_dn2 = assign87290_e133064_d_n2;
        locals.var_chi_dn4 = assign87290_e133064_d_n4;
        locals.var_chi_dn5 = assign87290_e133064_d_n5;
        locals.var_chi_dn6 = assign87290_e133064_d_n6;
        locals.var_chi_dn7 = assign87290_e133064_d_n7;
        locals.var_chi_dn8 = assign87290_e133064_d_n8;
        locals.var_chi_dn9 = assign87290_e133064_d_n9;
        locals.var_chi_dn10 = assign87290_e133064_d_n10;
        locals.var_chi_dn13 = assign87290_e133064_d_n13;
        locals.var_chi_rv = 0.0;

        let (assign87310_e133106,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2031 == 0.0)) {
        let assign87310_e133085: f64 = (2.0_f64).sqrt();
        let assign87310_e133086: f64 = (9.0 * assign87310_e133085);
        let assign87310_e133087: f64 = (1.0 / assign87310_e133086);
        let assign87310_e133091: f64 = (-3.0);
        let assign87310_e133092: f64 = (assign87310_e133091).exp();
        let assign87310_e133093: f64 = (7.0 * assign87310_e133092);
        let assign87310_e133094: f64 = (5.0 + assign87310_e133093);
        let assign87310_e133098: f64 = (-3.0);
        let assign87310_e133099: f64 = (assign87310_e133098).exp();
        let assign87310_e133100: f64 = (2.0 + assign87310_e133099);
        let assign87310_e133101: f64 = (assign87310_e133100).sqrt();
        let assign87310_e133102: f64 = (54.0 * assign87310_e133101);
        let assign87310_e133103: f64 = (assign87310_e133094 / assign87310_e133102);
        let assign87310_e133104: f64 = (assign87310_e133087 - assign87310_e133103);
        (assign87310_e133104,)
    } else {
        (locals.var_ta,)
    }
};
        locals.var_ta = assign87310_e133106;
        locals.var_ta_rv = 0.0;

        let (assign87320_e133134,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2031 == 0.0)) {
        let assign87320_e133116: f64 = (-3.0);
        let assign87320_e133117: f64 = (assign87320_e133116).exp();
        let assign87320_e133118: f64 = (1.0 + assign87320_e133117);
        let assign87320_e133122: f64 = (-3.0);
        let assign87320_e133123: f64 = (assign87320_e133122).exp();
        let assign87320_e133124: f64 = (2.0 + assign87320_e133123);
        let assign87320_e133125: f64 = (assign87320_e133124).sqrt();
        let assign87320_e133126: f64 = (2.0 * assign87320_e133125);
        let assign87320_e133127: f64 = (assign87320_e133118 / assign87320_e133126);
        let assign87320_e133129: f64 = (2.0_f64).sqrt();
        let assign87320_e133131: f64 = (assign87320_e133129 / 3.0);
        let assign87320_e133132: f64 = (assign87320_e133127 - assign87320_e133131);
        (assign87320_e133132,)
    } else {
        (locals.var_tb,)
    }
};
        locals.var_tb = assign87320_e133134;
        locals.var_tb_rv = 0.0;

        let (assign87330_e133153, assign87330_e133153_d_n0, assign87330_e133153_d_n2, assign87330_e133153_d_n4, assign87330_e133153_d_n5, assign87330_e133153_d_n6, assign87330_e133153_d_n7, assign87330_e133153_d_n8, assign87330_e133153_d_n9, assign87330_e133153_d_n10, assign87330_e133153_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2031 == 0.0)) {
        let assign87330_e133144: f64 = (2.0_f64).sqrt();
        let assign87330_e133145: f64 = (1.0 / assign87330_e133144);
        let assign87330_e133149: f64 = (locals.var_beta * locals.var_fac1);
        let assign87330_e133150: f64 = (1.0 / assign87330_e133149);
        let assign87330_e133151: f64 = (assign87330_e133145 + assign87330_e133150);
        (assign87330_e133151, (-(((locals.var_beta_dn0 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn0)) / (assign87330_e133149 * assign87330_e133149))), (-(((locals.var_beta_dn2 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn2)) / (assign87330_e133149 * assign87330_e133149))), (-(((locals.var_beta_dn4 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn4)) / (assign87330_e133149 * assign87330_e133149))), (-(((locals.var_beta_dn5 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn5)) / (assign87330_e133149 * assign87330_e133149))), (-(((locals.var_beta_dn6 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn6)) / (assign87330_e133149 * assign87330_e133149))), (-(((locals.var_beta_dn7 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn7)) / (assign87330_e133149 * assign87330_e133149))), (-(((locals.var_beta_dn8 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn8)) / (assign87330_e133149 * assign87330_e133149))), (-(((locals.var_beta_dn9 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn9)) / (assign87330_e133149 * assign87330_e133149))), (-(((locals.var_beta_dn10 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn10)) / (assign87330_e133149 * assign87330_e133149))), (-(((locals.var_beta_dn13 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn13)) / (assign87330_e133149 * assign87330_e133149))),)
    } else {
        (locals.var_tc, locals.var_tc_dn0, locals.var_tc_dn2, locals.var_tc_dn4, locals.var_tc_dn5, locals.var_tc_dn6, locals.var_tc_dn7, locals.var_tc_dn8, locals.var_tc_dn9, locals.var_tc_dn10, locals.var_tc_dn13,)
    }
};
        locals.var_tc = assign87330_e133153;
        locals.var_tc_dn0 = assign87330_e133153_d_n0;
        locals.var_tc_dn2 = assign87330_e133153_d_n2;
        locals.var_tc_dn4 = assign87330_e133153_d_n4;
        locals.var_tc_dn5 = assign87330_e133153_d_n5;
        locals.var_tc_dn6 = assign87330_e133153_d_n6;
        locals.var_tc_dn7 = assign87330_e133153_d_n7;
        locals.var_tc_dn8 = assign87330_e133153_d_n8;
        locals.var_tc_dn9 = assign87330_e133153_d_n9;
        locals.var_tc_dn10 = assign87330_e133153_d_n10;
        locals.var_tc_dn13 = assign87330_e133153_d_n13;
        locals.var_tc_rv = 0.0;

        let (assign87340_e133168, assign87340_e133168_d_n0, assign87340_e133168_d_n2, assign87340_e133168_d_n4, assign87340_e133168_d_n5, assign87340_e133168_d_n6, assign87340_e133168_d_n7, assign87340_e133168_d_n8, assign87340_e133168_d_n9, assign87340_e133168_d_n10, assign87340_e133168_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2031 == 0.0)) {
        let assign87340_e133163: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign87340_e133164: f64 = (-assign87340_e133163);
        let assign87340_e133166: f64 = (assign87340_e133164 / locals.var_fac1);
        (assign87340_e133166, ((((-locals.var_vxbgmtcl_dn0) * locals.var_fac1) - (assign87340_e133164 * locals.var_fac1_dn0)) / (locals.var_fac1 * locals.var_fac1)), ((((-(locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2)) * locals.var_fac1) - (assign87340_e133164 * locals.var_fac1_dn2)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn4) * locals.var_fac1) - (assign87340_e133164 * locals.var_fac1_dn4)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn5) * locals.var_fac1) - (assign87340_e133164 * locals.var_fac1_dn5)) / (locals.var_fac1 * locals.var_fac1)), ((((-(locals.var_vgpld_dn6 + locals.var_vxbgmtcl_dn6)) * locals.var_fac1) - (assign87340_e133164 * locals.var_fac1_dn6)) / (locals.var_fac1 * locals.var_fac1)), ((((-(locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7)) * locals.var_fac1) - (assign87340_e133164 * locals.var_fac1_dn7)) / (locals.var_fac1 * locals.var_fac1)), ((((-(locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8)) * locals.var_fac1) - (assign87340_e133164 * locals.var_fac1_dn8)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn9) * locals.var_fac1) - (assign87340_e133164 * locals.var_fac1_dn9)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn10) * locals.var_fac1) - (assign87340_e133164 * locals.var_fac1_dn10)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn13) * locals.var_fac1) - (assign87340_e133164 * locals.var_fac1_dn13)) / (locals.var_fac1 * locals.var_fac1)),)
    } else {
        (locals.var_td, locals.var_td_dn0, locals.var_td_dn2, locals.var_td_dn4, locals.var_td_dn5, locals.var_td_dn6, locals.var_td_dn7, locals.var_td_dn8, locals.var_td_dn9, locals.var_td_dn10, locals.var_td_dn13,)
    }
};
        locals.var_td = assign87340_e133168;
        locals.var_td_dn0 = assign87340_e133168_d_n0;
        locals.var_td_dn2 = assign87340_e133168_d_n2;
        locals.var_td_dn4 = assign87340_e133168_d_n4;
        locals.var_td_dn5 = assign87340_e133168_d_n5;
        locals.var_td_dn6 = assign87340_e133168_d_n6;
        locals.var_td_dn7 = assign87340_e133168_d_n7;
        locals.var_td_dn8 = assign87340_e133168_d_n8;
        locals.var_td_dn9 = assign87340_e133168_d_n9;
        locals.var_td_dn10 = assign87340_e133168_d_n10;
        locals.var_td_dn13 = assign87340_e133168_d_n13;
        locals.var_td_rv = 0.0;

        let (assign87350_e133206, assign87350_e133206_d_n0, assign87350_e133206_d_n2, assign87350_e133206_d_n4, assign87350_e133206_d_n5, assign87350_e133206_d_n6, assign87350_e133206_d_n7, assign87350_e133206_d_n8, assign87350_e133206_d_n9, assign87350_e133206_d_n10, assign87350_e133206_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2031 == 0.0)) {
        let assign87350_e133178: f64 = (locals.var_tb * locals.var_tb);
        let assign87350_e133180: f64 = (assign87350_e133178 * locals.var_tb);
        let assign87350_e133183: f64 = (27.0 * locals.var_ta);
        let assign87350_e133185: f64 = (assign87350_e133183 * locals.var_ta);
        let assign87350_e133187: f64 = (assign87350_e133185 * locals.var_ta);
        let assign87350_e133188: f64 = (assign87350_e133180 / assign87350_e133187);
        let assign87350_e133191: f64 = (locals.var_tb * locals.var_tc);
        let assign87350_e133194: f64 = (6.0 * locals.var_ta);
        let assign87350_e133196: f64 = (assign87350_e133194 * locals.var_ta);
        let assign87350_e133197: f64 = (assign87350_e133191 / assign87350_e133196);
        let assign87350_e133198: f64 = (assign87350_e133188 - assign87350_e133197);
        let assign87350_e133202: f64 = (2.0 * locals.var_ta);
        let assign87350_e133203: f64 = (locals.var_td / assign87350_e133202);
        let assign87350_e133204: f64 = (assign87350_e133198 + assign87350_e133203);
        (assign87350_e133204, ((-((locals.var_tb * locals.var_tc_dn0) / assign87350_e133196)) + (locals.var_td_dn0 / assign87350_e133202)), ((-((locals.var_tb * locals.var_tc_dn2) / assign87350_e133196)) + (locals.var_td_dn2 / assign87350_e133202)), ((-((locals.var_tb * locals.var_tc_dn4) / assign87350_e133196)) + (locals.var_td_dn4 / assign87350_e133202)), ((-((locals.var_tb * locals.var_tc_dn5) / assign87350_e133196)) + (locals.var_td_dn5 / assign87350_e133202)), ((-((locals.var_tb * locals.var_tc_dn6) / assign87350_e133196)) + (locals.var_td_dn6 / assign87350_e133202)), ((-((locals.var_tb * locals.var_tc_dn7) / assign87350_e133196)) + (locals.var_td_dn7 / assign87350_e133202)), ((-((locals.var_tb * locals.var_tc_dn8) / assign87350_e133196)) + (locals.var_td_dn8 / assign87350_e133202)), ((-((locals.var_tb * locals.var_tc_dn9) / assign87350_e133196)) + (locals.var_td_dn9 / assign87350_e133202)), ((-((locals.var_tb * locals.var_tc_dn10) / assign87350_e133196)) + (locals.var_td_dn10 / assign87350_e133202)), ((-((locals.var_tb * locals.var_tc_dn13) / assign87350_e133196)) + (locals.var_td_dn13 / assign87350_e133202)),)
    } else {
        (locals.var_tq, locals.var_tq_dn0, locals.var_tq_dn2, locals.var_tq_dn4, locals.var_tq_dn5, locals.var_tq_dn6, locals.var_tq_dn7, locals.var_tq_dn8, locals.var_tq_dn9, locals.var_tq_dn10, locals.var_tq_dn13,)
    }
};
        locals.var_tq = assign87350_e133206;
        locals.var_tq_dn0 = assign87350_e133206_d_n0;
        locals.var_tq_dn2 = assign87350_e133206_d_n2;
        locals.var_tq_dn4 = assign87350_e133206_d_n4;
        locals.var_tq_dn5 = assign87350_e133206_d_n5;
        locals.var_tq_dn6 = assign87350_e133206_d_n6;
        locals.var_tq_dn7 = assign87350_e133206_d_n7;
        locals.var_tq_dn8 = assign87350_e133206_d_n8;
        locals.var_tq_dn9 = assign87350_e133206_d_n9;
        locals.var_tq_dn10 = assign87350_e133206_d_n10;
        locals.var_tq_dn13 = assign87350_e133206_d_n13;
        locals.var_tq_rv = 0.0;

        let (assign87360_e133230, assign87360_e133230_d_n0, assign87360_e133230_d_n2, assign87360_e133230_d_n4, assign87360_e133230_d_n5, assign87360_e133230_d_n6, assign87360_e133230_d_n7, assign87360_e133230_d_n8, assign87360_e133230_d_n9, assign87360_e133230_d_n10, assign87360_e133230_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2031 == 0.0)) {
        let assign87360_e133216: f64 = (3.0 * locals.var_ta);
        let assign87360_e133218: f64 = (assign87360_e133216 * locals.var_tc);
        let assign87360_e133221: f64 = (locals.var_tb * locals.var_tb);
        let assign87360_e133222: f64 = (assign87360_e133218 - assign87360_e133221);
        let assign87360_e133225: f64 = (9.0 * locals.var_ta);
        let assign87360_e133227: f64 = (assign87360_e133225 * locals.var_ta);
        let assign87360_e133228: f64 = (assign87360_e133222 / assign87360_e133227);
        (assign87360_e133228, ((assign87360_e133216 * locals.var_tc_dn0) / assign87360_e133227), ((assign87360_e133216 * locals.var_tc_dn2) / assign87360_e133227), ((assign87360_e133216 * locals.var_tc_dn4) / assign87360_e133227), ((assign87360_e133216 * locals.var_tc_dn5) / assign87360_e133227), ((assign87360_e133216 * locals.var_tc_dn6) / assign87360_e133227), ((assign87360_e133216 * locals.var_tc_dn7) / assign87360_e133227), ((assign87360_e133216 * locals.var_tc_dn8) / assign87360_e133227), ((assign87360_e133216 * locals.var_tc_dn9) / assign87360_e133227), ((assign87360_e133216 * locals.var_tc_dn10) / assign87360_e133227), ((assign87360_e133216 * locals.var_tc_dn13) / assign87360_e133227),)
    } else {
        (locals.var_tp, locals.var_tp_dn0, locals.var_tp_dn2, locals.var_tp_dn4, locals.var_tp_dn5, locals.var_tp_dn6, locals.var_tp_dn7, locals.var_tp_dn8, locals.var_tp_dn9, locals.var_tp_dn10, locals.var_tp_dn13,)
    }
};
        locals.var_tp = assign87360_e133230;
        locals.var_tp_dn0 = assign87360_e133230_d_n0;
        locals.var_tp_dn2 = assign87360_e133230_d_n2;
        locals.var_tp_dn4 = assign87360_e133230_d_n4;
        locals.var_tp_dn5 = assign87360_e133230_d_n5;
        locals.var_tp_dn6 = assign87360_e133230_d_n6;
        locals.var_tp_dn7 = assign87360_e133230_d_n7;
        locals.var_tp_dn8 = assign87360_e133230_d_n8;
        locals.var_tp_dn9 = assign87360_e133230_d_n9;
        locals.var_tp_dn10 = assign87360_e133230_d_n10;
        locals.var_tp_dn13 = assign87360_e133230_d_n13;
        locals.var_tp_rv = 0.0;

        let (assign87370_e133249, assign87370_e133249_d_n0, assign87370_e133249_d_n2, assign87370_e133249_d_n4, assign87370_e133249_d_n5, assign87370_e133249_d_n6, assign87370_e133249_d_n7, assign87370_e133249_d_n8, assign87370_e133249_d_n9, assign87370_e133249_d_n10, assign87370_e133249_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2031 == 0.0)) {
        let assign87370_e133240: f64 = (locals.var_tq * locals.var_tq);
        let assign87370_e133243: f64 = (locals.var_tp * locals.var_tp);
        let assign87370_e133245: f64 = (assign87370_e133243 * locals.var_tp);
        let assign87370_e133246: f64 = (assign87370_e133240 + assign87370_e133245);
        let assign87370_e133247: f64 = (assign87370_e133246).sqrt();
        (assign87370_e133247, ((((locals.var_tq_dn0 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn0)) + ((((locals.var_tp_dn0 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn0)) * locals.var_tp) + (assign87370_e133243 * locals.var_tp_dn0))) / (2.0 * assign87370_e133247)), ((((locals.var_tq_dn2 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn2)) + ((((locals.var_tp_dn2 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn2)) * locals.var_tp) + (assign87370_e133243 * locals.var_tp_dn2))) / (2.0 * assign87370_e133247)), ((((locals.var_tq_dn4 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn4)) + ((((locals.var_tp_dn4 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn4)) * locals.var_tp) + (assign87370_e133243 * locals.var_tp_dn4))) / (2.0 * assign87370_e133247)), ((((locals.var_tq_dn5 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn5)) + ((((locals.var_tp_dn5 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn5)) * locals.var_tp) + (assign87370_e133243 * locals.var_tp_dn5))) / (2.0 * assign87370_e133247)), ((((locals.var_tq_dn6 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn6)) + ((((locals.var_tp_dn6 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn6)) * locals.var_tp) + (assign87370_e133243 * locals.var_tp_dn6))) / (2.0 * assign87370_e133247)), ((((locals.var_tq_dn7 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn7)) + ((((locals.var_tp_dn7 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn7)) * locals.var_tp) + (assign87370_e133243 * locals.var_tp_dn7))) / (2.0 * assign87370_e133247)), ((((locals.var_tq_dn8 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn8)) + ((((locals.var_tp_dn8 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn8)) * locals.var_tp) + (assign87370_e133243 * locals.var_tp_dn8))) / (2.0 * assign87370_e133247)), ((((locals.var_tq_dn9 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn9)) + ((((locals.var_tp_dn9 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn9)) * locals.var_tp) + (assign87370_e133243 * locals.var_tp_dn9))) / (2.0 * assign87370_e133247)), ((((locals.var_tq_dn10 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn10)) + ((((locals.var_tp_dn10 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn10)) * locals.var_tp) + (assign87370_e133243 * locals.var_tp_dn10))) / (2.0 * assign87370_e133247)), ((((locals.var_tq_dn13 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn13)) + ((((locals.var_tp_dn13 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn13)) * locals.var_tp) + (assign87370_e133243 * locals.var_tp_dn13))) / (2.0 * assign87370_e133247)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign87370_e133249;
        locals.var_t5_dn0 = assign87370_e133249_d_n0;
        locals.var_t5_dn2 = assign87370_e133249_d_n2;
        locals.var_t5_dn4 = assign87370_e133249_d_n4;
        locals.var_t5_dn5 = assign87370_e133249_d_n5;
        locals.var_t5_dn6 = assign87370_e133249_d_n6;
        locals.var_t5_dn7 = assign87370_e133249_d_n7;
        locals.var_t5_dn8 = assign87370_e133249_d_n8;
        locals.var_t5_dn9 = assign87370_e133249_d_n9;
        locals.var_t5_dn10 = assign87370_e133249_d_n10;
        locals.var_t5_dn13 = assign87370_e133249_d_n13;
        locals.var_t5_rv = 0.0;

        let (assign87380_e133264, assign87380_e133264_d_n0, assign87380_e133264_d_n2, assign87380_e133264_d_n4, assign87380_e133264_d_n5, assign87380_e133264_d_n6, assign87380_e133264_d_n7, assign87380_e133264_d_n8, assign87380_e133264_d_n9, assign87380_e133264_d_n10, assign87380_e133264_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2031 == 0.0)) {
        let assign87380_e133258: f64 = (-locals.var_tq);
        let assign87380_e133260: f64 = (assign87380_e133258 + locals.var_t5);
        let assign87380_e133262: f64 = (assign87380_e133260).powf(0.3333333333333333);
        (assign87380_e133262, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign87380_e133260).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn0) + locals.var_t5_dn0))) } } else { (assign87380_e133262 * (0.3333333333333333 * (((-locals.var_tq_dn0) + locals.var_t5_dn0) / assign87380_e133260))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign87380_e133260).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn2) + locals.var_t5_dn2))) } } else { (assign87380_e133262 * (0.3333333333333333 * (((-locals.var_tq_dn2) + locals.var_t5_dn2) / assign87380_e133260))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign87380_e133260).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn4) + locals.var_t5_dn4))) } } else { (assign87380_e133262 * (0.3333333333333333 * (((-locals.var_tq_dn4) + locals.var_t5_dn4) / assign87380_e133260))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign87380_e133260).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn5) + locals.var_t5_dn5))) } } else { (assign87380_e133262 * (0.3333333333333333 * (((-locals.var_tq_dn5) + locals.var_t5_dn5) / assign87380_e133260))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign87380_e133260).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn6) + locals.var_t5_dn6))) } } else { (assign87380_e133262 * (0.3333333333333333 * (((-locals.var_tq_dn6) + locals.var_t5_dn6) / assign87380_e133260))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign87380_e133260).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn7) + locals.var_t5_dn7))) } } else { (assign87380_e133262 * (0.3333333333333333 * (((-locals.var_tq_dn7) + locals.var_t5_dn7) / assign87380_e133260))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign87380_e133260).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn8) + locals.var_t5_dn8))) } } else { (assign87380_e133262 * (0.3333333333333333 * (((-locals.var_tq_dn8) + locals.var_t5_dn8) / assign87380_e133260))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign87380_e133260).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn9) + locals.var_t5_dn9))) } } else { (assign87380_e133262 * (0.3333333333333333 * (((-locals.var_tq_dn9) + locals.var_t5_dn9) / assign87380_e133260))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign87380_e133260).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn10) + locals.var_t5_dn10))) } } else { (assign87380_e133262 * (0.3333333333333333 * (((-locals.var_tq_dn10) + locals.var_t5_dn10) / assign87380_e133260))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign87380_e133260).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn13) + locals.var_t5_dn13))) } } else { (assign87380_e133262 * (0.3333333333333333 * (((-locals.var_tq_dn13) + locals.var_t5_dn13) / assign87380_e133260))) },)
    } else {
        (locals.var_tu, locals.var_tu_dn0, locals.var_tu_dn2, locals.var_tu_dn4, locals.var_tu_dn5, locals.var_tu_dn6, locals.var_tu_dn7, locals.var_tu_dn8, locals.var_tu_dn9, locals.var_tu_dn10, locals.var_tu_dn13,)
    }
};
        locals.var_tu = assign87380_e133264;
        locals.var_tu_dn0 = assign87380_e133264_d_n0;
        locals.var_tu_dn2 = assign87380_e133264_d_n2;
        locals.var_tu_dn4 = assign87380_e133264_d_n4;
        locals.var_tu_dn5 = assign87380_e133264_d_n5;
        locals.var_tu_dn6 = assign87380_e133264_d_n6;
        locals.var_tu_dn7 = assign87380_e133264_d_n7;
        locals.var_tu_dn8 = assign87380_e133264_d_n8;
        locals.var_tu_dn9 = assign87380_e133264_d_n9;
        locals.var_tu_dn10 = assign87380_e133264_d_n10;
        locals.var_tu_dn13 = assign87380_e133264_d_n13;
        locals.var_tu_rv = 0.0;

        let (assign87390_e133279, assign87390_e133279_d_n0, assign87390_e133279_d_n2, assign87390_e133279_d_n4, assign87390_e133279_d_n5, assign87390_e133279_d_n6, assign87390_e133279_d_n7, assign87390_e133279_d_n8, assign87390_e133279_d_n9, assign87390_e133279_d_n10, assign87390_e133279_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2031 == 0.0)) {
        let assign87390_e133274: f64 = (locals.var_tq + locals.var_t5);
        let assign87390_e133276: f64 = (assign87390_e133274).powf(0.3333333333333333);
        let assign87390_e133277: f64 = (-assign87390_e133276);
        (assign87390_e133277, (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign87390_e133274).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn0 + locals.var_t5_dn0))) } } else { (assign87390_e133276 * (0.3333333333333333 * ((locals.var_tq_dn0 + locals.var_t5_dn0) / assign87390_e133274))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign87390_e133274).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn2 + locals.var_t5_dn2))) } } else { (assign87390_e133276 * (0.3333333333333333 * ((locals.var_tq_dn2 + locals.var_t5_dn2) / assign87390_e133274))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign87390_e133274).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn4 + locals.var_t5_dn4))) } } else { (assign87390_e133276 * (0.3333333333333333 * ((locals.var_tq_dn4 + locals.var_t5_dn4) / assign87390_e133274))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign87390_e133274).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn5 + locals.var_t5_dn5))) } } else { (assign87390_e133276 * (0.3333333333333333 * ((locals.var_tq_dn5 + locals.var_t5_dn5) / assign87390_e133274))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign87390_e133274).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn6 + locals.var_t5_dn6))) } } else { (assign87390_e133276 * (0.3333333333333333 * ((locals.var_tq_dn6 + locals.var_t5_dn6) / assign87390_e133274))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign87390_e133274).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn7 + locals.var_t5_dn7))) } } else { (assign87390_e133276 * (0.3333333333333333 * ((locals.var_tq_dn7 + locals.var_t5_dn7) / assign87390_e133274))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign87390_e133274).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn8 + locals.var_t5_dn8))) } } else { (assign87390_e133276 * (0.3333333333333333 * ((locals.var_tq_dn8 + locals.var_t5_dn8) / assign87390_e133274))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign87390_e133274).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn9 + locals.var_t5_dn9))) } } else { (assign87390_e133276 * (0.3333333333333333 * ((locals.var_tq_dn9 + locals.var_t5_dn9) / assign87390_e133274))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign87390_e133274).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn10 + locals.var_t5_dn10))) } } else { (assign87390_e133276 * (0.3333333333333333 * ((locals.var_tq_dn10 + locals.var_t5_dn10) / assign87390_e133274))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign87390_e133274).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn13 + locals.var_t5_dn13))) } } else { (assign87390_e133276 * (0.3333333333333333 * ((locals.var_tq_dn13 + locals.var_t5_dn13) / assign87390_e133274))) }),)
    } else {
        (locals.var_tv, locals.var_tv_dn0, locals.var_tv_dn2, locals.var_tv_dn4, locals.var_tv_dn5, locals.var_tv_dn6, locals.var_tv_dn7, locals.var_tv_dn8, locals.var_tv_dn9, locals.var_tv_dn10, locals.var_tv_dn13,)
    }
};
        locals.var_tv = assign87390_e133279;
        locals.var_tv_dn0 = assign87390_e133279_d_n0;
        locals.var_tv_dn2 = assign87390_e133279_d_n2;
        locals.var_tv_dn4 = assign87390_e133279_d_n4;
        locals.var_tv_dn5 = assign87390_e133279_d_n5;
        locals.var_tv_dn6 = assign87390_e133279_d_n6;
        locals.var_tv_dn7 = assign87390_e133279_d_n7;
        locals.var_tv_dn8 = assign87390_e133279_d_n8;
        locals.var_tv_dn9 = assign87390_e133279_d_n9;
        locals.var_tv_dn10 = assign87390_e133279_d_n10;
        locals.var_tv_dn13 = assign87390_e133279_d_n13;
        locals.var_tv_rv = 0.0;

        let (assign87400_e133297, assign87400_e133297_d_n0, assign87400_e133297_d_n2, assign87400_e133297_d_n4, assign87400_e133297_d_n5, assign87400_e133297_d_n6, assign87400_e133297_d_n7, assign87400_e133297_d_n8, assign87400_e133297_d_n9, assign87400_e133297_d_n10, assign87400_e133297_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2031 == 0.0)) {
        let assign87400_e133289: f64 = (locals.var_tu + locals.var_tv);
        let assign87400_e133293: f64 = (3.0 * locals.var_ta);
        let assign87400_e133294: f64 = (locals.var_tb / assign87400_e133293);
        let assign87400_e133295: f64 = (assign87400_e133289 - assign87400_e133294);
        (assign87400_e133295, (locals.var_tu_dn0 + locals.var_tv_dn0), (locals.var_tu_dn2 + locals.var_tv_dn2), (locals.var_tu_dn4 + locals.var_tv_dn4), (locals.var_tu_dn5 + locals.var_tv_dn5), (locals.var_tu_dn6 + locals.var_tv_dn6), (locals.var_tu_dn7 + locals.var_tv_dn7), (locals.var_tu_dn8 + locals.var_tv_dn8), (locals.var_tu_dn9 + locals.var_tv_dn9), (locals.var_tu_dn10 + locals.var_tv_dn10), (locals.var_tu_dn13 + locals.var_tv_dn13),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    }
};
        locals.var_chi = assign87400_e133297;
        locals.var_chi_dn0 = assign87400_e133297_d_n0;
        locals.var_chi_dn2 = assign87400_e133297_d_n2;
        locals.var_chi_dn4 = assign87400_e133297_d_n4;
        locals.var_chi_dn5 = assign87400_e133297_d_n5;
        locals.var_chi_dn6 = assign87400_e133297_d_n6;
        locals.var_chi_dn7 = assign87400_e133297_d_n7;
        locals.var_chi_dn8 = assign87400_e133297_d_n8;
        locals.var_chi_dn9 = assign87400_e133297_d_n9;
        locals.var_chi_dn10 = assign87400_e133297_d_n10;
        locals.var_chi_dn13 = assign87400_e133297_d_n13;
        locals.var_chi_rv = 0.0;

        let (assign87410_e133311, assign87410_e133311_d_n0, assign87410_e133311_d_n2, assign87410_e133311_d_n4, assign87410_e133311_d_n5, assign87410_e133311_d_n6, assign87410_e133311_d_n7, assign87410_e133311_d_n8, assign87410_e133311_d_n9, assign87410_e133311_d_n10, assign87410_e133311_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2031 == 0.0)) {
        let assign87410_e133307: f64 = (locals.var_chi * locals.var_beta_inv);
        let assign87410_e133309: f64 = (assign87410_e133307 - locals.var_vxbgmtcl);
        (assign87410_e133309, (((locals.var_chi_dn0 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn0)) - locals.var_vxbgmtcl_dn0), (((locals.var_chi_dn2 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn2)) - locals.var_vxbgmtcl_dn2), (((locals.var_chi_dn4 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn4)) - locals.var_vxbgmtcl_dn4), (((locals.var_chi_dn5 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn5)) - locals.var_vxbgmtcl_dn5), (((locals.var_chi_dn6 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn6)) - locals.var_vxbgmtcl_dn6), (((locals.var_chi_dn7 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn7)) - locals.var_vxbgmtcl_dn7), (((locals.var_chi_dn8 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn8)) - locals.var_vxbgmtcl_dn8), (((locals.var_chi_dn9 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn9)) - locals.var_vxbgmtcl_dn9), (((locals.var_chi_dn10 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn10)) - locals.var_vxbgmtcl_dn10), (((locals.var_chi_dn13 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn13)) - locals.var_vxbgmtcl_dn13),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn13,)
    }
};
        locals.var_ps0_inia = assign87410_e133311;
        locals.var_ps0_inia_dn0 = assign87410_e133311_d_n0;
        locals.var_ps0_inia_dn2 = assign87410_e133311_d_n2;
        locals.var_ps0_inia_dn4 = assign87410_e133311_d_n4;
        locals.var_ps0_inia_dn5 = assign87410_e133311_d_n5;
        locals.var_ps0_inia_dn6 = assign87410_e133311_d_n6;
        locals.var_ps0_inia_dn7 = assign87410_e133311_d_n7;
        locals.var_ps0_inia_dn8 = assign87410_e133311_d_n8;
        locals.var_ps0_inia_dn9 = assign87410_e133311_d_n9;
        locals.var_ps0_inia_dn10 = assign87410_e133311_d_n10;
        locals.var_ps0_inia_dn13 = assign87410_e133311_d_n13;
        locals.var_ps0_inia_rv = 0.0;

        let assign87420_e133314: f64 = if p.p33 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2032 = assign87420_e133314;
        locals.var_guard2032_rv = 0.0;

        let (assign87430_e133327, assign87430_e133327_d_n0, assign87430_e133327_d_n2, assign87430_e133327_d_n4, assign87430_e133327_d_n5, assign87430_e133327_d_n6, assign87430_e133327_d_n7, assign87430_e133327_d_n8, assign87430_e133327_d_n9, assign87430_e133327_d_n10, assign87430_e133327_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) {
        let assign87430_e133323: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign87430_e133325: f64 = (assign87430_e133323 + 0.1);
        (assign87430_e133325, locals.var_vxbgmtcl_dn0, (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2), locals.var_vxbgmtcl_dn4, locals.var_vxbgmtcl_dn5, (locals.var_vgpld_dn6 + locals.var_vxbgmtcl_dn6), (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7), (locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8), locals.var_vxbgmtcl_dn9, locals.var_vxbgmtcl_dn10, locals.var_vxbgmtcl_dn13,)
    } else {
        (locals.var_vgpld_shift, locals.var_vgpld_shift_dn0, locals.var_vgpld_shift_dn2, locals.var_vgpld_shift_dn4, locals.var_vgpld_shift_dn5, locals.var_vgpld_shift_dn6, locals.var_vgpld_shift_dn7, locals.var_vgpld_shift_dn8, locals.var_vgpld_shift_dn9, locals.var_vgpld_shift_dn10, locals.var_vgpld_shift_dn13,)
    }
};
        locals.var_vgpld_shift = assign87430_e133327;
        locals.var_vgpld_shift_dn0 = assign87430_e133327_d_n0;
        locals.var_vgpld_shift_dn2 = assign87430_e133327_d_n2;
        locals.var_vgpld_shift_dn4 = assign87430_e133327_d_n4;
        locals.var_vgpld_shift_dn5 = assign87430_e133327_d_n5;
        locals.var_vgpld_shift_dn6 = assign87430_e133327_d_n6;
        locals.var_vgpld_shift_dn7 = assign87430_e133327_d_n7;
        locals.var_vgpld_shift_dn8 = assign87430_e133327_d_n8;
        locals.var_vgpld_shift_dn9 = assign87430_e133327_d_n9;
        locals.var_vgpld_shift_dn10 = assign87430_e133327_d_n10;
        locals.var_vgpld_shift_dn13 = assign87430_e133327_d_n13;
        locals.var_vgpld_shift_rv = 0.0;

        let (assign87440_e133338, assign87440_e133338_d_n0, assign87440_e133338_d_n2, assign87440_e133338_d_n4, assign87440_e133338_d_n5, assign87440_e133338_d_n6, assign87440_e133338_d_n7, assign87440_e133338_d_n8, assign87440_e133338_d_n9, assign87440_e133338_d_n10, assign87440_e133338_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) {
        let assign87440_e133336: f64 = (locals.var_cnst1over * locals.var_exp_bvbs);
        (assign87440_e133336, ((locals.var_cnst1over_dn0 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn0)), ((locals.var_cnst1over_dn2 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn2)), ((locals.var_cnst1over_dn4 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn4)), ((locals.var_cnst1over_dn5 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn5)), ((locals.var_cnst1over_dn6 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn6)), ((locals.var_cnst1over_dn7 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn7)), ((locals.var_cnst1over_dn8 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn8)), ((locals.var_cnst1over_dn9 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn9)), ((locals.var_cnst1over_dn10 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn10)), ((locals.var_cnst1over_dn13 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn13)),)
    } else {
        (locals.var_cfs1, locals.var_cfs1_dn0, locals.var_cfs1_dn2, locals.var_cfs1_dn4, locals.var_cfs1_dn5, locals.var_cfs1_dn6, locals.var_cfs1_dn7, locals.var_cfs1_dn8, locals.var_cfs1_dn9, locals.var_cfs1_dn10, locals.var_cfs1_dn13,)
    }
};
        locals.var_cfs1 = assign87440_e133338;
        locals.var_cfs1_dn0 = assign87440_e133338_d_n0;
        locals.var_cfs1_dn2 = assign87440_e133338_d_n2;
        locals.var_cfs1_dn4 = assign87440_e133338_d_n4;
        locals.var_cfs1_dn5 = assign87440_e133338_d_n5;
        locals.var_cfs1_dn6 = assign87440_e133338_d_n6;
        locals.var_cfs1_dn7 = assign87440_e133338_d_n7;
        locals.var_cfs1_dn8 = assign87440_e133338_d_n8;
        locals.var_cfs1_dn9 = assign87440_e133338_d_n9;
        locals.var_cfs1_dn10 = assign87440_e133338_d_n10;
        locals.var_cfs1_dn13 = assign87440_e133338_d_n13;
        locals.var_cfs1_rv = 0.0;

        let (assign87450_e133349, assign87450_e133349_d_n0, assign87450_e133349_d_n2, assign87450_e133349_d_n4, assign87450_e133349_d_n5, assign87450_e133349_d_n6, assign87450_e133349_d_n7, assign87450_e133349_d_n8, assign87450_e133349_d_n9, assign87450_e133349_d_n10, assign87450_e133349_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) {
        let assign87450_e133347: f64 = (locals.var_cnst1over * locals.var_exp_bvbs);
        (assign87450_e133347, ((locals.var_cnst1over_dn0 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn0)), ((locals.var_cnst1over_dn2 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn2)), ((locals.var_cnst1over_dn4 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn4)), ((locals.var_cnst1over_dn5 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn5)), ((locals.var_cnst1over_dn6 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn6)), ((locals.var_cnst1over_dn7 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn7)), ((locals.var_cnst1over_dn8 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn8)), ((locals.var_cnst1over_dn9 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn9)), ((locals.var_cnst1over_dn10 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn10)), ((locals.var_cnst1over_dn13 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn13)),)
    } else {
        (locals.var_gammachi, locals.var_gammachi_dn0, locals.var_gammachi_dn2, locals.var_gammachi_dn4, locals.var_gammachi_dn5, locals.var_gammachi_dn6, locals.var_gammachi_dn7, locals.var_gammachi_dn8, locals.var_gammachi_dn9, locals.var_gammachi_dn10, locals.var_gammachi_dn13,)
    }
};
        locals.var_gammachi = assign87450_e133349;
        locals.var_gammachi_dn0 = assign87450_e133349_d_n0;
        locals.var_gammachi_dn2 = assign87450_e133349_d_n2;
        locals.var_gammachi_dn4 = assign87450_e133349_d_n4;
        locals.var_gammachi_dn5 = assign87450_e133349_d_n5;
        locals.var_gammachi_dn6 = assign87450_e133349_d_n6;
        locals.var_gammachi_dn7 = assign87450_e133349_d_n7;
        locals.var_gammachi_dn8 = assign87450_e133349_d_n8;
        locals.var_gammachi_dn9 = assign87450_e133349_d_n9;
        locals.var_gammachi_dn10 = assign87450_e133349_d_n10;
        locals.var_gammachi_dn13 = assign87450_e133349_d_n13;
        locals.var_gammachi_rv = 0.0;

        let (assign87460_e133360, assign87460_e133360_d_n0, assign87460_e133360_d_n2, assign87460_e133360_d_n4, assign87460_e133360_d_n5, assign87460_e133360_d_n6, assign87460_e133360_d_n7, assign87460_e133360_d_n8, assign87460_e133360_d_n9, assign87460_e133360_d_n10, assign87460_e133360_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) {
        let assign87460_e133358: f64 = (locals.var_beta2 * locals.var_fac1p2);
        (assign87460_e133358, ((locals.var_beta2_dn0 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn0)), ((locals.var_beta2_dn2 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn2)), ((locals.var_beta2_dn4 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn4)), ((locals.var_beta2_dn5 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn5)), ((locals.var_beta2_dn6 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn6)), ((locals.var_beta2_dn7 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn7)), ((locals.var_beta2_dn8 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn8)), ((locals.var_beta2_dn9 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn9)), ((locals.var_beta2_dn10 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn10)), ((locals.var_beta2_dn13 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn13)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign87460_e133360;
        locals.var_t0_dn0 = assign87460_e133360_d_n0;
        locals.var_t0_dn2 = assign87460_e133360_d_n2;
        locals.var_t0_dn4 = assign87460_e133360_d_n4;
        locals.var_t0_dn5 = assign87460_e133360_d_n5;
        locals.var_t0_dn6 = assign87460_e133360_d_n6;
        locals.var_t0_dn7 = assign87460_e133360_d_n7;
        locals.var_t0_dn8 = assign87460_e133360_d_n8;
        locals.var_t0_dn9 = assign87460_e133360_d_n9;
        locals.var_t0_dn10 = assign87460_e133360_d_n10;
        locals.var_t0_dn13 = assign87460_e133360_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign87470_e133371, assign87470_e133371_d_n0, assign87470_e133371_d_n2, assign87470_e133371_d_n4, assign87470_e133371_d_n5, assign87470_e133371_d_n6, assign87470_e133371_d_n7, assign87470_e133371_d_n8, assign87470_e133371_d_n9, assign87470_e133371_d_n10, assign87470_e133371_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) {
        let assign87470_e133369: f64 = (locals.var_beta * locals.var_vgpld_shift);
        (assign87470_e133369, ((locals.var_beta_dn0 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn0)), ((locals.var_beta_dn2 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn2)), ((locals.var_beta_dn4 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn4)), ((locals.var_beta_dn5 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn5)), ((locals.var_beta_dn6 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn6)), ((locals.var_beta_dn7 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn7)), ((locals.var_beta_dn8 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn8)), ((locals.var_beta_dn9 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn9)), ((locals.var_beta_dn10 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn10)), ((locals.var_beta_dn13 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn13)),)
    } else {
        (locals.var_psi, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn4, locals.var_psi_dn5, locals.var_psi_dn6, locals.var_psi_dn7, locals.var_psi_dn8, locals.var_psi_dn9, locals.var_psi_dn10, locals.var_psi_dn13,)
    }
};
        locals.var_psi = assign87470_e133371;
        locals.var_psi_dn0 = assign87470_e133371_d_n0;
        locals.var_psi_dn2 = assign87470_e133371_d_n2;
        locals.var_psi_dn4 = assign87470_e133371_d_n4;
        locals.var_psi_dn5 = assign87470_e133371_d_n5;
        locals.var_psi_dn6 = assign87470_e133371_d_n6;
        locals.var_psi_dn7 = assign87470_e133371_d_n7;
        locals.var_psi_dn8 = assign87470_e133371_d_n8;
        locals.var_psi_dn9 = assign87470_e133371_d_n9;
        locals.var_psi_dn10 = assign87470_e133371_d_n10;
        locals.var_psi_dn13 = assign87470_e133371_d_n13;
        locals.var_psi_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_324(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign87480_e133396, assign87480_e133396_d_n0, assign87480_e133396_d_n2, assign87480_e133396_d_n4, assign87480_e133396_d_n5, assign87480_e133396_d_n6, assign87480_e133396_d_n7, assign87480_e133396_d_n8, assign87480_e133396_d_n9, assign87480_e133396_d_n10, assign87480_e133396_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) {
        let assign87480_e133380: f64 = (locals.var_gammachi * locals.var_t0);
        let assign87480_e133383: f64 = (locals.var_psi * locals.var_psi);
        let assign87480_e133384: f64 = (assign87480_e133380 + assign87480_e133383);
        let assign87480_e133385: f64 = (assign87480_e133384).ln();
        let assign87480_e133388: f64 = (locals.var_cnst1over * locals.var_t0);
        let assign87480_e133389: f64 = (assign87480_e133388).ln();
        let assign87480_e133390: f64 = (assign87480_e133385 - assign87480_e133389);
        let assign87480_e133393: f64 = (locals.var_beta * locals.var_vxbgmtcl);
        let assign87480_e133394: f64 = (assign87480_e133390 + assign87480_e133393);
        (assign87480_e133394, ((((((locals.var_gammachi_dn0 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn0)) + ((locals.var_psi_dn0 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn0))) / assign87480_e133384) - (((locals.var_cnst1over_dn0 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn0)) / assign87480_e133388)) + ((locals.var_beta_dn0 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn0))), ((((((locals.var_gammachi_dn2 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn2)) + ((locals.var_psi_dn2 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn2))) / assign87480_e133384) - (((locals.var_cnst1over_dn2 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn2)) / assign87480_e133388)) + ((locals.var_beta_dn2 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn2))), ((((((locals.var_gammachi_dn4 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn4)) + ((locals.var_psi_dn4 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn4))) / assign87480_e133384) - (((locals.var_cnst1over_dn4 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn4)) / assign87480_e133388)) + ((locals.var_beta_dn4 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn4))), ((((((locals.var_gammachi_dn5 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn5)) + ((locals.var_psi_dn5 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn5))) / assign87480_e133384) - (((locals.var_cnst1over_dn5 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn5)) / assign87480_e133388)) + ((locals.var_beta_dn5 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn5))), ((((((locals.var_gammachi_dn6 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn6)) + ((locals.var_psi_dn6 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn6))) / assign87480_e133384) - (((locals.var_cnst1over_dn6 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn6)) / assign87480_e133388)) + ((locals.var_beta_dn6 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn6))), ((((((locals.var_gammachi_dn7 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn7)) + ((locals.var_psi_dn7 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn7))) / assign87480_e133384) - (((locals.var_cnst1over_dn7 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn7)) / assign87480_e133388)) + ((locals.var_beta_dn7 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn7))), ((((((locals.var_gammachi_dn8 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn8)) + ((locals.var_psi_dn8 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn8))) / assign87480_e133384) - (((locals.var_cnst1over_dn8 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn8)) / assign87480_e133388)) + ((locals.var_beta_dn8 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn8))), ((((((locals.var_gammachi_dn9 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn9)) + ((locals.var_psi_dn9 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn9))) / assign87480_e133384) - (((locals.var_cnst1over_dn9 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn9)) / assign87480_e133388)) + ((locals.var_beta_dn9 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn9))), ((((((locals.var_gammachi_dn10 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn10)) + ((locals.var_psi_dn10 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn10))) / assign87480_e133384) - (((locals.var_cnst1over_dn10 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn10)) / assign87480_e133388)) + ((locals.var_beta_dn10 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn10))), ((((((locals.var_gammachi_dn13 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn13)) + ((locals.var_psi_dn13 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn13))) / assign87480_e133384) - (((locals.var_cnst1over_dn13 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn13)) / assign87480_e133388)) + ((locals.var_beta_dn13 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn13))),)
    } else {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn13,)
    }
};
        locals.var_chi_1 = assign87480_e133396;
        locals.var_chi_1_dn0 = assign87480_e133396_d_n0;
        locals.var_chi_1_dn2 = assign87480_e133396_d_n2;
        locals.var_chi_1_dn4 = assign87480_e133396_d_n4;
        locals.var_chi_1_dn5 = assign87480_e133396_d_n5;
        locals.var_chi_1_dn6 = assign87480_e133396_d_n6;
        locals.var_chi_1_dn7 = assign87480_e133396_d_n7;
        locals.var_chi_1_dn8 = assign87480_e133396_d_n8;
        locals.var_chi_1_dn9 = assign87480_e133396_d_n9;
        locals.var_chi_1_dn10 = assign87480_e133396_d_n10;
        locals.var_chi_1_dn13 = assign87480_e133396_d_n13;
        locals.var_chi_1_rv = 0.0;

        let assign87490_e133399: f64 = if p.p33 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard2033 = assign87490_e133399;
        locals.var_guard2033_rv = 0.0;

        let (assign87500_e133414, assign87500_e133414_d_n0, assign87500_e133414_d_n2, assign87500_e133414_d_n4, assign87500_e133414_d_n5, assign87500_e133414_d_n6, assign87500_e133414_d_n7, assign87500_e133414_d_n8, assign87500_e133414_d_n9, assign87500_e133414_d_n10, assign87500_e133414_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) && (locals.var_guard2033 != 0.0)) {
        let assign87500_e133410: f64 = (locals.var_psi - locals.var_chi_1);
        let assign87500_e133412: f64 = (assign87500_e133410 - 1.0);
        (assign87500_e133412, (locals.var_psi_dn0 - locals.var_chi_1_dn0), (locals.var_psi_dn2 - locals.var_chi_1_dn2), (locals.var_psi_dn4 - locals.var_chi_1_dn4), (locals.var_psi_dn5 - locals.var_chi_1_dn5), (locals.var_psi_dn6 - locals.var_chi_1_dn6), (locals.var_psi_dn7 - locals.var_chi_1_dn7), (locals.var_psi_dn8 - locals.var_chi_1_dn8), (locals.var_psi_dn9 - locals.var_chi_1_dn9), (locals.var_psi_dn10 - locals.var_chi_1_dn10), (locals.var_psi_dn13 - locals.var_chi_1_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign87500_e133414;
        locals.var_tmf1_dn0 = assign87500_e133414_d_n0;
        locals.var_tmf1_dn2 = assign87500_e133414_d_n2;
        locals.var_tmf1_dn4 = assign87500_e133414_d_n4;
        locals.var_tmf1_dn5 = assign87500_e133414_d_n5;
        locals.var_tmf1_dn6 = assign87500_e133414_d_n6;
        locals.var_tmf1_dn7 = assign87500_e133414_d_n7;
        locals.var_tmf1_dn8 = assign87500_e133414_d_n8;
        locals.var_tmf1_dn9 = assign87500_e133414_d_n9;
        locals.var_tmf1_dn10 = assign87500_e133414_d_n10;
        locals.var_tmf1_dn13 = assign87500_e133414_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign87510_e133429, assign87510_e133429_d_n0, assign87510_e133429_d_n2, assign87510_e133429_d_n4, assign87510_e133429_d_n5, assign87510_e133429_d_n6, assign87510_e133429_d_n7, assign87510_e133429_d_n8, assign87510_e133429_d_n9, assign87510_e133429_d_n10, assign87510_e133429_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) && (locals.var_guard2033 != 0.0)) {
        let assign87510_e133425: f64 = (4.0 * locals.var_psi);
        let assign87510_e133427: f64 = assign87510_e133425;
        (assign87510_e133427, (4.0 * locals.var_psi_dn0), (4.0 * locals.var_psi_dn2), (4.0 * locals.var_psi_dn4), (4.0 * locals.var_psi_dn5), (4.0 * locals.var_psi_dn6), (4.0 * locals.var_psi_dn7), (4.0 * locals.var_psi_dn8), (4.0 * locals.var_psi_dn9), (4.0 * locals.var_psi_dn10), (4.0 * locals.var_psi_dn13),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign87510_e133429;
        locals.var_tmf2_dn0 = assign87510_e133429_d_n0;
        locals.var_tmf2_dn2 = assign87510_e133429_d_n2;
        locals.var_tmf2_dn4 = assign87510_e133429_d_n4;
        locals.var_tmf2_dn5 = assign87510_e133429_d_n5;
        locals.var_tmf2_dn6 = assign87510_e133429_d_n6;
        locals.var_tmf2_dn7 = assign87510_e133429_d_n7;
        locals.var_tmf2_dn8 = assign87510_e133429_d_n8;
        locals.var_tmf2_dn9 = assign87510_e133429_d_n9;
        locals.var_tmf2_dn10 = assign87510_e133429_d_n10;
        locals.var_tmf2_dn13 = assign87510_e133429_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign87520_e133446, assign87520_e133446_d_n0, assign87520_e133446_d_n2, assign87520_e133446_d_n4, assign87520_e133446_d_n5, assign87520_e133446_d_n6, assign87520_e133446_d_n7, assign87520_e133446_d_n8, assign87520_e133446_d_n9, assign87520_e133446_d_n10, assign87520_e133446_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) && (locals.var_guard2033 != 0.0)) {
        let (assign87520_e133444, assign87520_e133444_d_n0, assign87520_e133444_d_n2, assign87520_e133444_d_n4, assign87520_e133444_d_n5, assign87520_e133444_d_n6, assign87520_e133444_d_n7, assign87520_e133444_d_n8, assign87520_e133444_d_n9, assign87520_e133444_d_n10, assign87520_e133444_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign87520_e133443: f64 = (-locals.var_tmf2);
                (assign87520_e133443, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign87520_e133444, assign87520_e133444_d_n0, assign87520_e133444_d_n2, assign87520_e133444_d_n4, assign87520_e133444_d_n5, assign87520_e133444_d_n6, assign87520_e133444_d_n7, assign87520_e133444_d_n8, assign87520_e133444_d_n9, assign87520_e133444_d_n10, assign87520_e133444_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign87520_e133446;
        locals.var_tmf2_dn0 = assign87520_e133446_d_n0;
        locals.var_tmf2_dn2 = assign87520_e133446_d_n2;
        locals.var_tmf2_dn4 = assign87520_e133446_d_n4;
        locals.var_tmf2_dn5 = assign87520_e133446_d_n5;
        locals.var_tmf2_dn6 = assign87520_e133446_d_n6;
        locals.var_tmf2_dn7 = assign87520_e133446_d_n7;
        locals.var_tmf2_dn8 = assign87520_e133446_d_n8;
        locals.var_tmf2_dn9 = assign87520_e133446_d_n9;
        locals.var_tmf2_dn10 = assign87520_e133446_d_n10;
        locals.var_tmf2_dn13 = assign87520_e133446_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign87530_e133462, assign87530_e133462_d_n0, assign87530_e133462_d_n2, assign87530_e133462_d_n4, assign87530_e133462_d_n5, assign87530_e133462_d_n6, assign87530_e133462_d_n7, assign87530_e133462_d_n8, assign87530_e133462_d_n9, assign87530_e133462_d_n10, assign87530_e133462_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) && (locals.var_guard2033 != 0.0)) {
        let assign87530_e133457: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign87530_e133459: f64 = (assign87530_e133457 + locals.var_tmf2);
        let assign87530_e133460: f64 = (assign87530_e133459).sqrt();
        (assign87530_e133460, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign87530_e133460)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign87530_e133460)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign87530_e133460)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign87530_e133460)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign87530_e133460)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign87530_e133460)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign87530_e133460)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign87530_e133460)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign87530_e133460)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign87530_e133460)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign87530_e133462;
        locals.var_tmf2_dn0 = assign87530_e133462_d_n0;
        locals.var_tmf2_dn2 = assign87530_e133462_d_n2;
        locals.var_tmf2_dn4 = assign87530_e133462_d_n4;
        locals.var_tmf2_dn5 = assign87530_e133462_d_n5;
        locals.var_tmf2_dn6 = assign87530_e133462_d_n6;
        locals.var_tmf2_dn7 = assign87530_e133462_d_n7;
        locals.var_tmf2_dn8 = assign87530_e133462_d_n8;
        locals.var_tmf2_dn9 = assign87530_e133462_d_n9;
        locals.var_tmf2_dn10 = assign87530_e133462_d_n10;
        locals.var_tmf2_dn13 = assign87530_e133462_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign87540_e133479, assign87540_e133479_d_n0, assign87540_e133479_d_n2, assign87540_e133479_d_n4, assign87540_e133479_d_n5, assign87540_e133479_d_n6, assign87540_e133479_d_n7, assign87540_e133479_d_n8, assign87540_e133479_d_n9, assign87540_e133479_d_n10, assign87540_e133479_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) && (locals.var_guard2033 != 0.0)) {
        let assign87540_e133475: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign87540_e133476: f64 = (1.0 + assign87540_e133475);
        let assign87540_e133477: f64 = (0.5 * assign87540_e133476);
        (assign87540_e133477, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign87540_e133479;
        locals.var_t1_dn0 = assign87540_e133479_d_n0;
        locals.var_t1_dn2 = assign87540_e133479_d_n2;
        locals.var_t1_dn4 = assign87540_e133479_d_n4;
        locals.var_t1_dn5 = assign87540_e133479_d_n5;
        locals.var_t1_dn6 = assign87540_e133479_d_n6;
        locals.var_t1_dn7 = assign87540_e133479_d_n7;
        locals.var_t1_dn8 = assign87540_e133479_d_n8;
        locals.var_t1_dn9 = assign87540_e133479_d_n9;
        locals.var_t1_dn10 = assign87540_e133479_d_n10;
        locals.var_t1_dn13 = assign87540_e133479_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign87550_e133496, assign87550_e133496_d_n0, assign87550_e133496_d_n2, assign87550_e133496_d_n4, assign87550_e133496_d_n5, assign87550_e133496_d_n6, assign87550_e133496_d_n7, assign87550_e133496_d_n8, assign87550_e133496_d_n9, assign87550_e133496_d_n10, assign87550_e133496_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) && (locals.var_guard2033 != 0.0)) {
        let assign87550_e133492: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign87550_e133493: f64 = (0.5 * assign87550_e133492);
        let assign87550_e133494: f64 = (locals.var_psi - assign87550_e133493);
        (assign87550_e133494, (locals.var_psi_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_psi_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_psi_dn4 - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_psi_dn5 - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_psi_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_psi_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_psi_dn8 - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_psi_dn9 - (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (locals.var_psi_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_psi_dn13 - (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13))),)
    } else {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn13,)
    }
};
        locals.var_chi_1 = assign87550_e133496;
        locals.var_chi_1_dn0 = assign87550_e133496_d_n0;
        locals.var_chi_1_dn2 = assign87550_e133496_d_n2;
        locals.var_chi_1_dn4 = assign87550_e133496_d_n4;
        locals.var_chi_1_dn5 = assign87550_e133496_d_n5;
        locals.var_chi_1_dn6 = assign87550_e133496_d_n6;
        locals.var_chi_1_dn7 = assign87550_e133496_d_n7;
        locals.var_chi_1_dn8 = assign87550_e133496_d_n8;
        locals.var_chi_1_dn9 = assign87550_e133496_d_n9;
        locals.var_chi_1_dn10 = assign87550_e133496_d_n10;
        locals.var_chi_1_dn13 = assign87550_e133496_d_n13;
        locals.var_chi_1_rv = 0.0;

        let (assign87560_e133513, assign87560_e133513_d_n0, assign87560_e133513_d_n2, assign87560_e133513_d_n4, assign87560_e133513_d_n5, assign87560_e133513_d_n6, assign87560_e133513_d_n7, assign87560_e133513_d_n8, assign87560_e133513_d_n9, assign87560_e133513_d_n10, assign87560_e133513_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) && (locals.var_guard2033 == 0.0)) {
        let (assign87560_e133511, assign87560_e133511_d_n0, assign87560_e133511_d_n2, assign87560_e133511_d_n4, assign87560_e133511_d_n5, assign87560_e133511_d_n6, assign87560_e133511_d_n7, assign87560_e133511_d_n8, assign87560_e133511_d_n9, assign87560_e133511_d_n10, assign87560_e133511_d_n13,) = {
            if (locals.var_chi_1 <= locals.var_psi) {
                (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn13,)
            } else {
                (locals.var_psi, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn4, locals.var_psi_dn5, locals.var_psi_dn6, locals.var_psi_dn7, locals.var_psi_dn8, locals.var_psi_dn9, locals.var_psi_dn10, locals.var_psi_dn13,)
            }
        };
        (assign87560_e133511, assign87560_e133511_d_n0, assign87560_e133511_d_n2, assign87560_e133511_d_n4, assign87560_e133511_d_n5, assign87560_e133511_d_n6, assign87560_e133511_d_n7, assign87560_e133511_d_n8, assign87560_e133511_d_n9, assign87560_e133511_d_n10, assign87560_e133511_d_n13,)
    } else {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn13,)
    }
};
        locals.var_chi_1 = assign87560_e133513;
        locals.var_chi_1_dn0 = assign87560_e133513_d_n0;
        locals.var_chi_1_dn2 = assign87560_e133513_d_n2;
        locals.var_chi_1_dn4 = assign87560_e133513_d_n4;
        locals.var_chi_1_dn5 = assign87560_e133513_d_n5;
        locals.var_chi_1_dn6 = assign87560_e133513_d_n6;
        locals.var_chi_1_dn7 = assign87560_e133513_d_n7;
        locals.var_chi_1_dn8 = assign87560_e133513_d_n8;
        locals.var_chi_1_dn9 = assign87560_e133513_d_n9;
        locals.var_chi_1_dn10 = assign87560_e133513_d_n10;
        locals.var_chi_1_dn13 = assign87560_e133513_d_n13;
        locals.var_chi_1_rv = 0.0;

        let (assign87570_e133527, assign87570_e133527_d_n0, assign87570_e133527_d_n2, assign87570_e133527_d_n4, assign87570_e133527_d_n5, assign87570_e133527_d_n6, assign87570_e133527_d_n7, assign87570_e133527_d_n8, assign87570_e133527_d_n9, assign87570_e133527_d_n10, assign87570_e133527_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) {
        let (assign87570_e133525, assign87570_e133525_d_n0, assign87570_e133525_d_n2, assign87570_e133525_d_n4, assign87570_e133525_d_n5, assign87570_e133525_d_n6, assign87570_e133525_d_n7, assign87570_e133525_d_n8, assign87570_e133525_d_n9, assign87570_e133525_d_n10, assign87570_e133525_d_n13,) = {
            if (locals.var_chi_1 >= 0.0) {
                (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn13,)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign87570_e133525, assign87570_e133525_d_n0, assign87570_e133525_d_n2, assign87570_e133525_d_n4, assign87570_e133525_d_n5, assign87570_e133525_d_n6, assign87570_e133525_d_n7, assign87570_e133525_d_n8, assign87570_e133525_d_n9, assign87570_e133525_d_n10, assign87570_e133525_d_n13,)
    } else {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn13,)
    }
};
        locals.var_chi_1 = assign87570_e133527;
        locals.var_chi_1_dn0 = assign87570_e133527_d_n0;
        locals.var_chi_1_dn2 = assign87570_e133527_d_n2;
        locals.var_chi_1_dn4 = assign87570_e133527_d_n4;
        locals.var_chi_1_dn5 = assign87570_e133527_d_n5;
        locals.var_chi_1_dn6 = assign87570_e133527_d_n6;
        locals.var_chi_1_dn7 = assign87570_e133527_d_n7;
        locals.var_chi_1_dn8 = assign87570_e133527_d_n8;
        locals.var_chi_1_dn9 = assign87570_e133527_d_n9;
        locals.var_chi_1_dn10 = assign87570_e133527_d_n10;
        locals.var_chi_1_dn13 = assign87570_e133527_d_n13;
        locals.var_chi_1_rv = 0.0;

        let (assign87580_e133538, assign87580_e133538_d_n0, assign87580_e133538_d_n2, assign87580_e133538_d_n4, assign87580_e133538_d_n5, assign87580_e133538_d_n6, assign87580_e133538_d_n7, assign87580_e133538_d_n8, assign87580_e133538_d_n9, assign87580_e133538_d_n10, assign87580_e133538_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) {
        let assign87580_e133536: f64 = (locals.var_psi - locals.var_chi_1);
        (assign87580_e133536, (locals.var_psi_dn0 - locals.var_chi_1_dn0), (locals.var_psi_dn2 - locals.var_chi_1_dn2), (locals.var_psi_dn4 - locals.var_chi_1_dn4), (locals.var_psi_dn5 - locals.var_chi_1_dn5), (locals.var_psi_dn6 - locals.var_chi_1_dn6), (locals.var_psi_dn7 - locals.var_chi_1_dn7), (locals.var_psi_dn8 - locals.var_chi_1_dn8), (locals.var_psi_dn9 - locals.var_chi_1_dn9), (locals.var_psi_dn10 - locals.var_chi_1_dn10), (locals.var_psi_dn13 - locals.var_chi_1_dn13),)
    } else {
        (locals.var_psi, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn4, locals.var_psi_dn5, locals.var_psi_dn6, locals.var_psi_dn7, locals.var_psi_dn8, locals.var_psi_dn9, locals.var_psi_dn10, locals.var_psi_dn13,)
    }
};
        locals.var_psi = assign87580_e133538;
        locals.var_psi_dn0 = assign87580_e133538_d_n0;
        locals.var_psi_dn2 = assign87580_e133538_d_n2;
        locals.var_psi_dn4 = assign87580_e133538_d_n4;
        locals.var_psi_dn5 = assign87580_e133538_d_n5;
        locals.var_psi_dn6 = assign87580_e133538_d_n6;
        locals.var_psi_dn7 = assign87580_e133538_d_n7;
        locals.var_psi_dn8 = assign87580_e133538_d_n8;
        locals.var_psi_dn9 = assign87580_e133538_d_n9;
        locals.var_psi_dn10 = assign87580_e133538_d_n10;
        locals.var_psi_dn13 = assign87580_e133538_d_n13;
        locals.var_psi_rv = 0.0;

        let (assign87590_e133551, assign87590_e133551_d_n0, assign87590_e133551_d_n2, assign87590_e133551_d_n4, assign87590_e133551_d_n5, assign87590_e133551_d_n6, assign87590_e133551_d_n7, assign87590_e133551_d_n8, assign87590_e133551_d_n9, assign87590_e133551_d_n10, assign87590_e133551_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) {
        let assign87590_e133548: f64 = (locals.var_beta * 0.1);
        let assign87590_e133549: f64 = (locals.var_psi + assign87590_e133548);
        (assign87590_e133549, (locals.var_psi_dn0 + (locals.var_beta_dn0 * 0.1)), (locals.var_psi_dn2 + (locals.var_beta_dn2 * 0.1)), (locals.var_psi_dn4 + (locals.var_beta_dn4 * 0.1)), (locals.var_psi_dn5 + (locals.var_beta_dn5 * 0.1)), (locals.var_psi_dn6 + (locals.var_beta_dn6 * 0.1)), (locals.var_psi_dn7 + (locals.var_beta_dn7 * 0.1)), (locals.var_psi_dn8 + (locals.var_beta_dn8 * 0.1)), (locals.var_psi_dn9 + (locals.var_beta_dn9 * 0.1)), (locals.var_psi_dn10 + (locals.var_beta_dn10 * 0.1)), (locals.var_psi_dn13 + (locals.var_beta_dn13 * 0.1)),)
    } else {
        (locals.var_psi, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn4, locals.var_psi_dn5, locals.var_psi_dn6, locals.var_psi_dn7, locals.var_psi_dn8, locals.var_psi_dn9, locals.var_psi_dn10, locals.var_psi_dn13,)
    }
};
        locals.var_psi = assign87590_e133551;
        locals.var_psi_dn0 = assign87590_e133551_d_n0;
        locals.var_psi_dn2 = assign87590_e133551_d_n2;
        locals.var_psi_dn4 = assign87590_e133551_d_n4;
        locals.var_psi_dn5 = assign87590_e133551_d_n5;
        locals.var_psi_dn6 = assign87590_e133551_d_n6;
        locals.var_psi_dn7 = assign87590_e133551_d_n7;
        locals.var_psi_dn8 = assign87590_e133551_d_n8;
        locals.var_psi_dn9 = assign87590_e133551_d_n9;
        locals.var_psi_dn10 = assign87590_e133551_d_n10;
        locals.var_psi_dn13 = assign87590_e133551_d_n13;
        locals.var_psi_rv = 0.0;

        let (assign87600_e133572, assign87600_e133572_d_n0, assign87600_e133572_d_n2, assign87600_e133572_d_n4, assign87600_e133572_d_n5, assign87600_e133572_d_n6, assign87600_e133572_d_n7, assign87600_e133572_d_n8, assign87600_e133572_d_n9, assign87600_e133572_d_n10, assign87600_e133572_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) {
        let assign87600_e133560: f64 = (locals.var_gammachi * locals.var_t0);
        let assign87600_e133563: f64 = (locals.var_psi * locals.var_psi);
        let assign87600_e133564: f64 = (assign87600_e133560 + assign87600_e133563);
        let assign87600_e133565: f64 = (assign87600_e133564).ln();
        let assign87600_e133568: f64 = (locals.var_cnst1over * locals.var_t0);
        let assign87600_e133569: f64 = (assign87600_e133568).ln();
        let assign87600_e133570: f64 = (assign87600_e133565 - assign87600_e133569);
        (assign87600_e133570, (((((locals.var_gammachi_dn0 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn0)) + ((locals.var_psi_dn0 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn0))) / assign87600_e133564) - (((locals.var_cnst1over_dn0 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn0)) / assign87600_e133568)), (((((locals.var_gammachi_dn2 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn2)) + ((locals.var_psi_dn2 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn2))) / assign87600_e133564) - (((locals.var_cnst1over_dn2 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn2)) / assign87600_e133568)), (((((locals.var_gammachi_dn4 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn4)) + ((locals.var_psi_dn4 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn4))) / assign87600_e133564) - (((locals.var_cnst1over_dn4 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn4)) / assign87600_e133568)), (((((locals.var_gammachi_dn5 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn5)) + ((locals.var_psi_dn5 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn5))) / assign87600_e133564) - (((locals.var_cnst1over_dn5 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn5)) / assign87600_e133568)), (((((locals.var_gammachi_dn6 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn6)) + ((locals.var_psi_dn6 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn6))) / assign87600_e133564) - (((locals.var_cnst1over_dn6 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn6)) / assign87600_e133568)), (((((locals.var_gammachi_dn7 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn7)) + ((locals.var_psi_dn7 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn7))) / assign87600_e133564) - (((locals.var_cnst1over_dn7 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn7)) / assign87600_e133568)), (((((locals.var_gammachi_dn8 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn8)) + ((locals.var_psi_dn8 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn8))) / assign87600_e133564) - (((locals.var_cnst1over_dn8 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn8)) / assign87600_e133568)), (((((locals.var_gammachi_dn9 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn9)) + ((locals.var_psi_dn9 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn9))) / assign87600_e133564) - (((locals.var_cnst1over_dn9 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn9)) / assign87600_e133568)), (((((locals.var_gammachi_dn10 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn10)) + ((locals.var_psi_dn10 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn10))) / assign87600_e133564) - (((locals.var_cnst1over_dn10 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn10)) / assign87600_e133568)), (((((locals.var_gammachi_dn13 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn13)) + ((locals.var_psi_dn13 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn13))) / assign87600_e133564) - (((locals.var_cnst1over_dn13 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn13)) / assign87600_e133568)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign87600_e133572;
        locals.var_t1_dn0 = assign87600_e133572_d_n0;
        locals.var_t1_dn2 = assign87600_e133572_d_n2;
        locals.var_t1_dn4 = assign87600_e133572_d_n4;
        locals.var_t1_dn5 = assign87600_e133572_d_n5;
        locals.var_t1_dn6 = assign87600_e133572_d_n6;
        locals.var_t1_dn7 = assign87600_e133572_d_n7;
        locals.var_t1_dn8 = assign87600_e133572_d_n8;
        locals.var_t1_dn9 = assign87600_e133572_d_n9;
        locals.var_t1_dn10 = assign87600_e133572_d_n10;
        locals.var_t1_dn13 = assign87600_e133572_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign87610_e133585, assign87610_e133585_d_n0, assign87610_e133585_d_n2, assign87610_e133585_d_n4, assign87610_e133585_d_n5, assign87610_e133585_d_n6, assign87610_e133585_d_n7, assign87610_e133585_d_n8, assign87610_e133585_d_n9, assign87610_e133585_d_n10, assign87610_e133585_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) {
        let assign87610_e133582: f64 = (locals.var_beta * locals.var_vxbgmtcl);
        let assign87610_e133583: f64 = (locals.var_t1 + assign87610_e133582);
        (assign87610_e133583, (locals.var_t1_dn0 + ((locals.var_beta_dn0 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn0))), (locals.var_t1_dn2 + ((locals.var_beta_dn2 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn2))), (locals.var_t1_dn4 + ((locals.var_beta_dn4 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn4))), (locals.var_t1_dn5 + ((locals.var_beta_dn5 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn5))), (locals.var_t1_dn6 + ((locals.var_beta_dn6 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn6))), (locals.var_t1_dn7 + ((locals.var_beta_dn7 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn7))), (locals.var_t1_dn8 + ((locals.var_beta_dn8 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn8))), (locals.var_t1_dn9 + ((locals.var_beta_dn9 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn9))), (locals.var_t1_dn10 + ((locals.var_beta_dn10 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn10))), (locals.var_t1_dn13 + ((locals.var_beta_dn13 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn13))),)
    } else {
        (locals.var_chi_b, locals.var_chi_b_dn0, locals.var_chi_b_dn2, locals.var_chi_b_dn4, locals.var_chi_b_dn5, locals.var_chi_b_dn6, locals.var_chi_b_dn7, locals.var_chi_b_dn8, locals.var_chi_b_dn9, locals.var_chi_b_dn10, locals.var_chi_b_dn13,)
    }
};
        locals.var_chi_b = assign87610_e133585;
        locals.var_chi_b_dn0 = assign87610_e133585_d_n0;
        locals.var_chi_b_dn2 = assign87610_e133585_d_n2;
        locals.var_chi_b_dn4 = assign87610_e133585_d_n4;
        locals.var_chi_b_dn5 = assign87610_e133585_d_n5;
        locals.var_chi_b_dn6 = assign87610_e133585_d_n6;
        locals.var_chi_b_dn7 = assign87610_e133585_d_n7;
        locals.var_chi_b_dn8 = assign87610_e133585_d_n8;
        locals.var_chi_b_dn9 = assign87610_e133585_d_n9;
        locals.var_chi_b_dn10 = assign87610_e133585_d_n10;
        locals.var_chi_b_dn13 = assign87610_e133585_d_n13;
        locals.var_chi_b_rv = 0.0;

        let (assign87620_e133599, assign87620_e133599_d_n0, assign87620_e133599_d_n2, assign87620_e133599_d_n4, assign87620_e133599_d_n5, assign87620_e133599_d_n6, assign87620_e133599_d_n7, assign87620_e133599_d_n8, assign87620_e133599_d_n9, assign87620_e133599_d_n10, assign87620_e133599_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) {
        let (assign87620_e133597, assign87620_e133597_d_n0, assign87620_e133597_d_n2, assign87620_e133597_d_n4, assign87620_e133597_d_n5, assign87620_e133597_d_n6, assign87620_e133597_d_n7, assign87620_e133597_d_n8, assign87620_e133597_d_n9, assign87620_e133597_d_n10, assign87620_e133597_d_n13,) = {
            if (locals.var_chi_b >= 0.0) {
                (locals.var_chi_b, locals.var_chi_b_dn0, locals.var_chi_b_dn2, locals.var_chi_b_dn4, locals.var_chi_b_dn5, locals.var_chi_b_dn6, locals.var_chi_b_dn7, locals.var_chi_b_dn8, locals.var_chi_b_dn9, locals.var_chi_b_dn10, locals.var_chi_b_dn13,)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign87620_e133597, assign87620_e133597_d_n0, assign87620_e133597_d_n2, assign87620_e133597_d_n4, assign87620_e133597_d_n5, assign87620_e133597_d_n6, assign87620_e133597_d_n7, assign87620_e133597_d_n8, assign87620_e133597_d_n9, assign87620_e133597_d_n10, assign87620_e133597_d_n13,)
    } else {
        (locals.var_chi_b, locals.var_chi_b_dn0, locals.var_chi_b_dn2, locals.var_chi_b_dn4, locals.var_chi_b_dn5, locals.var_chi_b_dn6, locals.var_chi_b_dn7, locals.var_chi_b_dn8, locals.var_chi_b_dn9, locals.var_chi_b_dn10, locals.var_chi_b_dn13,)
    }
};
        locals.var_chi_b = assign87620_e133599;
        locals.var_chi_b_dn0 = assign87620_e133599_d_n0;
        locals.var_chi_b_dn2 = assign87620_e133599_d_n2;
        locals.var_chi_b_dn4 = assign87620_e133599_d_n4;
        locals.var_chi_b_dn5 = assign87620_e133599_d_n5;
        locals.var_chi_b_dn6 = assign87620_e133599_d_n6;
        locals.var_chi_b_dn7 = assign87620_e133599_d_n7;
        locals.var_chi_b_dn8 = assign87620_e133599_d_n8;
        locals.var_chi_b_dn9 = assign87620_e133599_d_n9;
        locals.var_chi_b_dn10 = assign87620_e133599_d_n10;
        locals.var_chi_b_dn13 = assign87620_e133599_d_n13;
        locals.var_chi_b_rv = 0.0;

        let (assign87630_e133608, assign87630_e133608_d_n0, assign87630_e133608_d_n2, assign87630_e133608_d_n4, assign87630_e133608_d_n5, assign87630_e133608_d_n6, assign87630_e133608_d_n7, assign87630_e133608_d_n8, assign87630_e133608_d_n9, assign87630_e133608_d_n10, assign87630_e133608_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    } else {
        (locals.var_chi_a, locals.var_chi_a_dn0, locals.var_chi_a_dn2, locals.var_chi_a_dn4, locals.var_chi_a_dn5, locals.var_chi_a_dn6, locals.var_chi_a_dn7, locals.var_chi_a_dn8, locals.var_chi_a_dn9, locals.var_chi_a_dn10, locals.var_chi_a_dn13,)
    }
};
        locals.var_chi_a = assign87630_e133608;
        locals.var_chi_a_dn0 = assign87630_e133608_d_n0;
        locals.var_chi_a_dn2 = assign87630_e133608_d_n2;
        locals.var_chi_a_dn4 = assign87630_e133608_d_n4;
        locals.var_chi_a_dn5 = assign87630_e133608_d_n5;
        locals.var_chi_a_dn6 = assign87630_e133608_d_n6;
        locals.var_chi_a_dn7 = assign87630_e133608_d_n7;
        locals.var_chi_a_dn8 = assign87630_e133608_d_n8;
        locals.var_chi_a_dn9 = assign87630_e133608_d_n9;
        locals.var_chi_a_dn10 = assign87630_e133608_d_n10;
        locals.var_chi_a_dn13 = assign87630_e133608_d_n13;
        locals.var_chi_a_rv = 0.0;

        let assign87640_e133611: f64 = if p.p33 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard2034 = assign87640_e133611;
        locals.var_guard2034_rv = 0.0;

        let assign87650_e133616: f64 = (0.2 * locals.var_chi_b);
        let assign87650_e133617: f64 = (locals.var_chi_b - assign87650_e133616);
        let assign87650_e133621: f64 = (0.2 * locals.var_chi_b);
        let assign87650_e133624: f64 = if ((locals.var_chi_a > assign87650_e133617) && (assign87650_e133621 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2035 = assign87650_e133624;
        locals.var_guard2035_rv = 0.0;

        let (assign87660_e133643, assign87660_e133643_d_n0, assign87660_e133643_d_n2, assign87660_e133643_d_n4, assign87660_e133643_d_n5, assign87660_e133643_d_n6, assign87660_e133643_d_n7, assign87660_e133643_d_n8, assign87660_e133643_d_n9, assign87660_e133643_d_n10, assign87660_e133643_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2035 != 0.0)) {
        let assign87660_e133637: f64 = (locals.var_chi_a - locals.var_chi_b);
        let assign87660_e133640: f64 = (0.2 * locals.var_chi_b);
        let assign87660_e133641: f64 = (assign87660_e133637 + assign87660_e133640);
        (assign87660_e133641, ((locals.var_chi_a_dn0 - locals.var_chi_b_dn0) + (0.2 * locals.var_chi_b_dn0)), ((locals.var_chi_a_dn2 - locals.var_chi_b_dn2) + (0.2 * locals.var_chi_b_dn2)), ((locals.var_chi_a_dn4 - locals.var_chi_b_dn4) + (0.2 * locals.var_chi_b_dn4)), ((locals.var_chi_a_dn5 - locals.var_chi_b_dn5) + (0.2 * locals.var_chi_b_dn5)), ((locals.var_chi_a_dn6 - locals.var_chi_b_dn6) + (0.2 * locals.var_chi_b_dn6)), ((locals.var_chi_a_dn7 - locals.var_chi_b_dn7) + (0.2 * locals.var_chi_b_dn7)), ((locals.var_chi_a_dn8 - locals.var_chi_b_dn8) + (0.2 * locals.var_chi_b_dn8)), ((locals.var_chi_a_dn9 - locals.var_chi_b_dn9) + (0.2 * locals.var_chi_b_dn9)), ((locals.var_chi_a_dn10 - locals.var_chi_b_dn10) + (0.2 * locals.var_chi_b_dn10)), ((locals.var_chi_a_dn13 - locals.var_chi_b_dn13) + (0.2 * locals.var_chi_b_dn13)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign87660_e133643;
        locals.var_tmf1_dn0 = assign87660_e133643_d_n0;
        locals.var_tmf1_dn2 = assign87660_e133643_d_n2;
        locals.var_tmf1_dn4 = assign87660_e133643_d_n4;
        locals.var_tmf1_dn5 = assign87660_e133643_d_n5;
        locals.var_tmf1_dn6 = assign87660_e133643_d_n6;
        locals.var_tmf1_dn7 = assign87660_e133643_d_n7;
        locals.var_tmf1_dn8 = assign87660_e133643_d_n8;
        locals.var_tmf1_dn9 = assign87660_e133643_d_n9;
        locals.var_tmf1_dn10 = assign87660_e133643_d_n10;
        locals.var_tmf1_dn13 = assign87660_e133643_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign87670_e133658, assign87670_e133658_d_n0, assign87670_e133658_d_n2, assign87670_e133658_d_n4, assign87670_e133658_d_n5, assign87670_e133658_d_n6, assign87670_e133658_d_n7, assign87670_e133658_d_n8, assign87670_e133658_d_n9, assign87670_e133658_d_n10, assign87670_e133658_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2035 != 0.0)) {
        let assign87670_e133656: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign87670_e133656, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
        locals.var_x2 = assign87670_e133658;
        locals.var_x2_dn0 = assign87670_e133658_d_n0;
        locals.var_x2_dn2 = assign87670_e133658_d_n2;
        locals.var_x2_dn4 = assign87670_e133658_d_n4;
        locals.var_x2_dn5 = assign87670_e133658_d_n5;
        locals.var_x2_dn6 = assign87670_e133658_d_n6;
        locals.var_x2_dn7 = assign87670_e133658_d_n7;
        locals.var_x2_dn8 = assign87670_e133658_d_n8;
        locals.var_x2_dn9 = assign87670_e133658_d_n9;
        locals.var_x2_dn10 = assign87670_e133658_d_n10;
        locals.var_x2_dn13 = assign87670_e133658_d_n13;
        locals.var_x2_rv = 0.0;

        let (assign87680_e133677, assign87680_e133677_d_n0, assign87680_e133677_d_n2, assign87680_e133677_d_n4, assign87680_e133677_d_n5, assign87680_e133677_d_n6, assign87680_e133677_d_n7, assign87680_e133677_d_n8, assign87680_e133677_d_n9, assign87680_e133677_d_n10, assign87680_e133677_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2035 != 0.0)) {
        let assign87680_e133671: f64 = (0.2 * locals.var_chi_b);
        let assign87680_e133674: f64 = (0.2 * locals.var_chi_b);
        let assign87680_e133675: f64 = (assign87680_e133671 * assign87680_e133674);
        (assign87680_e133675, (((0.2 * locals.var_chi_b_dn0) * assign87680_e133674) + (assign87680_e133671 * (0.2 * locals.var_chi_b_dn0))), (((0.2 * locals.var_chi_b_dn2) * assign87680_e133674) + (assign87680_e133671 * (0.2 * locals.var_chi_b_dn2))), (((0.2 * locals.var_chi_b_dn4) * assign87680_e133674) + (assign87680_e133671 * (0.2 * locals.var_chi_b_dn4))), (((0.2 * locals.var_chi_b_dn5) * assign87680_e133674) + (assign87680_e133671 * (0.2 * locals.var_chi_b_dn5))), (((0.2 * locals.var_chi_b_dn6) * assign87680_e133674) + (assign87680_e133671 * (0.2 * locals.var_chi_b_dn6))), (((0.2 * locals.var_chi_b_dn7) * assign87680_e133674) + (assign87680_e133671 * (0.2 * locals.var_chi_b_dn7))), (((0.2 * locals.var_chi_b_dn8) * assign87680_e133674) + (assign87680_e133671 * (0.2 * locals.var_chi_b_dn8))), (((0.2 * locals.var_chi_b_dn9) * assign87680_e133674) + (assign87680_e133671 * (0.2 * locals.var_chi_b_dn9))), (((0.2 * locals.var_chi_b_dn10) * assign87680_e133674) + (assign87680_e133671 * (0.2 * locals.var_chi_b_dn10))), (((0.2 * locals.var_chi_b_dn13) * assign87680_e133674) + (assign87680_e133671 * (0.2 * locals.var_chi_b_dn13))),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
        locals.var_xmax2 = assign87680_e133677;
        locals.var_xmax2_dn0 = assign87680_e133677_d_n0;
        locals.var_xmax2_dn2 = assign87680_e133677_d_n2;
        locals.var_xmax2_dn4 = assign87680_e133677_d_n4;
        locals.var_xmax2_dn5 = assign87680_e133677_d_n5;
        locals.var_xmax2_dn6 = assign87680_e133677_d_n6;
        locals.var_xmax2_dn7 = assign87680_e133677_d_n7;
        locals.var_xmax2_dn8 = assign87680_e133677_d_n8;
        locals.var_xmax2_dn9 = assign87680_e133677_d_n9;
        locals.var_xmax2_dn10 = assign87680_e133677_d_n10;
        locals.var_xmax2_dn13 = assign87680_e133677_d_n13;
        locals.var_xmax2_rv = 0.0;

        let (assign87690_e133690, assign87690_e133690_d_n0, assign87690_e133690_d_n2, assign87690_e133690_d_n4, assign87690_e133690_d_n5, assign87690_e133690_d_n6, assign87690_e133690_d_n7, assign87690_e133690_d_n8, assign87690_e133690_d_n9, assign87690_e133690_d_n10, assign87690_e133690_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2035 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign87690_e133690;
        locals.var_xp_dn0 = assign87690_e133690_d_n0;
        locals.var_xp_dn2 = assign87690_e133690_d_n2;
        locals.var_xp_dn4 = assign87690_e133690_d_n4;
        locals.var_xp_dn5 = assign87690_e133690_d_n5;
        locals.var_xp_dn6 = assign87690_e133690_d_n6;
        locals.var_xp_dn7 = assign87690_e133690_d_n7;
        locals.var_xp_dn8 = assign87690_e133690_d_n8;
        locals.var_xp_dn9 = assign87690_e133690_d_n9;
        locals.var_xp_dn10 = assign87690_e133690_d_n10;
        locals.var_xp_dn13 = assign87690_e133690_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign87700_e133703, assign87700_e133703_d_n0, assign87700_e133703_d_n2, assign87700_e133703_d_n4, assign87700_e133703_d_n5, assign87700_e133703_d_n6, assign87700_e133703_d_n7, assign87700_e133703_d_n8, assign87700_e133703_d_n9, assign87700_e133703_d_n10, assign87700_e133703_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2035 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign87700_e133703;
        locals.var_xmp_dn0 = assign87700_e133703_d_n0;
        locals.var_xmp_dn2 = assign87700_e133703_d_n2;
        locals.var_xmp_dn4 = assign87700_e133703_d_n4;
        locals.var_xmp_dn5 = assign87700_e133703_d_n5;
        locals.var_xmp_dn6 = assign87700_e133703_d_n6;
        locals.var_xmp_dn7 = assign87700_e133703_d_n7;
        locals.var_xmp_dn8 = assign87700_e133703_d_n8;
        locals.var_xmp_dn9 = assign87700_e133703_d_n9;
        locals.var_xmp_dn10 = assign87700_e133703_d_n10;
        locals.var_xmp_dn13 = assign87700_e133703_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign87710_e133716,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2035 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign87710_e133716;
        locals.var_m0_rv = 0.0;

        let (assign87720_e133729,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2035 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign87720_e133729;
        locals.var_mm_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_325(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign87730_e133742, assign87730_e133742_d_n0, assign87730_e133742_d_n2, assign87730_e133742_d_n4, assign87730_e133742_d_n5, assign87730_e133742_d_n6, assign87730_e133742_d_n7, assign87730_e133742_d_n8, assign87730_e133742_d_n9, assign87730_e133742_d_n10, assign87730_e133742_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2035 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign87730_e133742;
        locals.var_arg_dn0 = assign87730_e133742_d_n0;
        locals.var_arg_dn2 = assign87730_e133742_d_n2;
        locals.var_arg_dn4 = assign87730_e133742_d_n4;
        locals.var_arg_dn5 = assign87730_e133742_d_n5;
        locals.var_arg_dn6 = assign87730_e133742_d_n6;
        locals.var_arg_dn7 = assign87730_e133742_d_n7;
        locals.var_arg_dn8 = assign87730_e133742_d_n8;
        locals.var_arg_dn9 = assign87730_e133742_d_n9;
        locals.var_arg_dn10 = assign87730_e133742_d_n10;
        locals.var_arg_dn13 = assign87730_e133742_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign87740_e133755, assign87740_e133755_d_n0, assign87740_e133755_d_n2, assign87740_e133755_d_n4, assign87740_e133755_d_n5, assign87740_e133755_d_n6, assign87740_e133755_d_n7, assign87740_e133755_d_n8, assign87740_e133755_d_n9, assign87740_e133755_d_n10, assign87740_e133755_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2035 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign87740_e133755;
        locals.var_dnm_dn0 = assign87740_e133755_d_n0;
        locals.var_dnm_dn2 = assign87740_e133755_d_n2;
        locals.var_dnm_dn4 = assign87740_e133755_d_n4;
        locals.var_dnm_dn5 = assign87740_e133755_d_n5;
        locals.var_dnm_dn6 = assign87740_e133755_d_n6;
        locals.var_dnm_dn7 = assign87740_e133755_d_n7;
        locals.var_dnm_dn8 = assign87740_e133755_d_n8;
        locals.var_dnm_dn9 = assign87740_e133755_d_n9;
        locals.var_dnm_dn10 = assign87740_e133755_d_n10;
        locals.var_dnm_dn13 = assign87740_e133755_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign87750_e133770, assign87750_e133770_d_n0, assign87750_e133770_d_n2, assign87750_e133770_d_n4, assign87750_e133770_d_n5, assign87750_e133770_d_n6, assign87750_e133770_d_n7, assign87750_e133770_d_n8, assign87750_e133770_d_n9, assign87750_e133770_d_n10, assign87750_e133770_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2035 != 0.0)) {
        let assign87750_e133768: f64 = (locals.var_xp * locals.var_x2);
        (assign87750_e133768, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign87750_e133770;
        locals.var_xp_dn0 = assign87750_e133770_d_n0;
        locals.var_xp_dn2 = assign87750_e133770_d_n2;
        locals.var_xp_dn4 = assign87750_e133770_d_n4;
        locals.var_xp_dn5 = assign87750_e133770_d_n5;
        locals.var_xp_dn6 = assign87750_e133770_d_n6;
        locals.var_xp_dn7 = assign87750_e133770_d_n7;
        locals.var_xp_dn8 = assign87750_e133770_d_n8;
        locals.var_xp_dn9 = assign87750_e133770_d_n9;
        locals.var_xp_dn10 = assign87750_e133770_d_n10;
        locals.var_xp_dn13 = assign87750_e133770_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign87760_e133785, assign87760_e133785_d_n0, assign87760_e133785_d_n2, assign87760_e133785_d_n4, assign87760_e133785_d_n5, assign87760_e133785_d_n6, assign87760_e133785_d_n7, assign87760_e133785_d_n8, assign87760_e133785_d_n9, assign87760_e133785_d_n10, assign87760_e133785_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2035 != 0.0)) {
        let assign87760_e133783: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign87760_e133783, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign87760_e133785;
        locals.var_xmp_dn0 = assign87760_e133785_d_n0;
        locals.var_xmp_dn2 = assign87760_e133785_d_n2;
        locals.var_xmp_dn4 = assign87760_e133785_d_n4;
        locals.var_xmp_dn5 = assign87760_e133785_d_n5;
        locals.var_xmp_dn6 = assign87760_e133785_d_n6;
        locals.var_xmp_dn7 = assign87760_e133785_d_n7;
        locals.var_xmp_dn8 = assign87760_e133785_d_n8;
        locals.var_xmp_dn9 = assign87760_e133785_d_n9;
        locals.var_xmp_dn10 = assign87760_e133785_d_n10;
        locals.var_xmp_dn13 = assign87760_e133785_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign87770_e133800, assign87770_e133800_d_n0, assign87770_e133800_d_n2, assign87770_e133800_d_n4, assign87770_e133800_d_n5, assign87770_e133800_d_n6, assign87770_e133800_d_n7, assign87770_e133800_d_n8, assign87770_e133800_d_n9, assign87770_e133800_d_n10, assign87770_e133800_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2035 != 0.0)) {
        let assign87770_e133798: f64 = (locals.var_xp * locals.var_x2);
        (assign87770_e133798, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign87770_e133800;
        locals.var_xp_dn0 = assign87770_e133800_d_n0;
        locals.var_xp_dn2 = assign87770_e133800_d_n2;
        locals.var_xp_dn4 = assign87770_e133800_d_n4;
        locals.var_xp_dn5 = assign87770_e133800_d_n5;
        locals.var_xp_dn6 = assign87770_e133800_d_n6;
        locals.var_xp_dn7 = assign87770_e133800_d_n7;
        locals.var_xp_dn8 = assign87770_e133800_d_n8;
        locals.var_xp_dn9 = assign87770_e133800_d_n9;
        locals.var_xp_dn10 = assign87770_e133800_d_n10;
        locals.var_xp_dn13 = assign87770_e133800_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign87780_e133815, assign87780_e133815_d_n0, assign87780_e133815_d_n2, assign87780_e133815_d_n4, assign87780_e133815_d_n5, assign87780_e133815_d_n6, assign87780_e133815_d_n7, assign87780_e133815_d_n8, assign87780_e133815_d_n9, assign87780_e133815_d_n10, assign87780_e133815_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2035 != 0.0)) {
        let assign87780_e133813: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign87780_e133813, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign87780_e133815;
        locals.var_xmp_dn0 = assign87780_e133815_d_n0;
        locals.var_xmp_dn2 = assign87780_e133815_d_n2;
        locals.var_xmp_dn4 = assign87780_e133815_d_n4;
        locals.var_xmp_dn5 = assign87780_e133815_d_n5;
        locals.var_xmp_dn6 = assign87780_e133815_d_n6;
        locals.var_xmp_dn7 = assign87780_e133815_d_n7;
        locals.var_xmp_dn8 = assign87780_e133815_d_n8;
        locals.var_xmp_dn9 = assign87780_e133815_d_n9;
        locals.var_xmp_dn10 = assign87780_e133815_d_n10;
        locals.var_xmp_dn13 = assign87780_e133815_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign87790_e133830, assign87790_e133830_d_n0, assign87790_e133830_d_n2, assign87790_e133830_d_n4, assign87790_e133830_d_n5, assign87790_e133830_d_n6, assign87790_e133830_d_n7, assign87790_e133830_d_n8, assign87790_e133830_d_n9, assign87790_e133830_d_n10, assign87790_e133830_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2035 != 0.0)) {
        let assign87790_e133828: f64 = (locals.var_xp + locals.var_xmp);
        (assign87790_e133828, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign87790_e133830;
        locals.var_arg_dn0 = assign87790_e133830_d_n0;
        locals.var_arg_dn2 = assign87790_e133830_d_n2;
        locals.var_arg_dn4 = assign87790_e133830_d_n4;
        locals.var_arg_dn5 = assign87790_e133830_d_n5;
        locals.var_arg_dn6 = assign87790_e133830_d_n6;
        locals.var_arg_dn7 = assign87790_e133830_d_n7;
        locals.var_arg_dn8 = assign87790_e133830_d_n8;
        locals.var_arg_dn9 = assign87790_e133830_d_n9;
        locals.var_arg_dn10 = assign87790_e133830_d_n10;
        locals.var_arg_dn13 = assign87790_e133830_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign87800_e133843, assign87800_e133843_d_n0, assign87800_e133843_d_n2, assign87800_e133843_d_n4, assign87800_e133843_d_n5, assign87800_e133843_d_n6, assign87800_e133843_d_n7, assign87800_e133843_d_n8, assign87800_e133843_d_n9, assign87800_e133843_d_n10, assign87800_e133843_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2035 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign87800_e133843;
        locals.var_dnm_dn0 = assign87800_e133843_d_n0;
        locals.var_dnm_dn2 = assign87800_e133843_d_n2;
        locals.var_dnm_dn4 = assign87800_e133843_d_n4;
        locals.var_dnm_dn5 = assign87800_e133843_d_n5;
        locals.var_dnm_dn6 = assign87800_e133843_d_n6;
        locals.var_dnm_dn7 = assign87800_e133843_d_n7;
        locals.var_dnm_dn8 = assign87800_e133843_d_n8;
        locals.var_dnm_dn9 = assign87800_e133843_d_n9;
        locals.var_dnm_dn10 = assign87800_e133843_d_n10;
        locals.var_dnm_dn13 = assign87800_e133843_d_n13;
        locals.var_dnm_rv = 0.0;

        let assign87810_e133858: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard2036 = assign87810_e133858;
        locals.var_guard2036_rv = 0.0;

        let assign87820_e133861: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2037 = assign87820_e133861;
        locals.var_guard2037_rv = 0.0;

        let (assign87830_e133878,) = {
    if (((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2035 != 0.0)) && (locals.var_guard2036 != 0.0)) && (locals.var_guard2037 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign87830_e133878;
        locals.var_mm_rv = 0.0;

        let assign87840_e133881: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard2038 = assign87840_e133881;
        locals.var_guard2038_rv = 0.0;

        let (assign87850_e133901,) = {
    if ((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2035 != 0.0)) && (locals.var_guard2036 != 0.0)) && (locals.var_guard2037 == 0.0)) && (locals.var_guard2038 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign87850_e133901;
        locals.var_mm_rv = 0.0;

        let assign87860_e133904: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard2039 = assign87860_e133904;
        locals.var_guard2039_rv = 0.0;

        let (assign87870_e133927,) = {
    if (((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2035 != 0.0)) && (locals.var_guard2036 != 0.0)) && (locals.var_guard2037 == 0.0)) && (locals.var_guard2038 == 0.0)) && (locals.var_guard2039 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign87870_e133927;
        locals.var_mm_rv = 0.0;

        let assign87880_e133930: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard2040 = assign87880_e133930;
        locals.var_guard2040_rv = 0.0;

        let (assign87890_e133956,) = {
    if ((((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2035 != 0.0)) && (locals.var_guard2036 != 0.0)) && (locals.var_guard2037 == 0.0)) && (locals.var_guard2038 == 0.0)) && (locals.var_guard2039 == 0.0)) && (locals.var_guard2040 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign87890_e133956;
        locals.var_mm_rv = 0.0;

        let (assign87900_e133971,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2035 != 0.0)) && (locals.var_guard2036 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign87900_e133971;
        locals.var_m0_rv = 0.0;

        let mut assign87910_loop_guard: usize = 0;
        while {
            let assign87910_cond_e133987: f64 = if (((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2035 != 0.0)) && (locals.var_guard2036 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign87910_cond_e133987 != 0.0
        } {
            assign87910_loop_guard += 1;
            assert!(assign87910_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign87910_body0_e134003, assign87910_body0_e134003_d_n0, assign87910_body0_e134003_d_n2, assign87910_body0_e134003_d_n4, assign87910_body0_e134003_d_n5, assign87910_body0_e134003_d_n6, assign87910_body0_e134003_d_n7, assign87910_body0_e134003_d_n8, assign87910_body0_e134003_d_n9, assign87910_body0_e134003_d_n10, assign87910_body0_e134003_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2035 != 0.0)) && (locals.var_guard2036 != 0.0)) {
        let assign87910_body0_e134001: f64 = (locals.var_dnm).sqrt();
        (assign87910_body0_e134001, (locals.var_dnm_dn0 / (2.0 * assign87910_body0_e134001)), (locals.var_dnm_dn2 / (2.0 * assign87910_body0_e134001)), (locals.var_dnm_dn4 / (2.0 * assign87910_body0_e134001)), (locals.var_dnm_dn5 / (2.0 * assign87910_body0_e134001)), (locals.var_dnm_dn6 / (2.0 * assign87910_body0_e134001)), (locals.var_dnm_dn7 / (2.0 * assign87910_body0_e134001)), (locals.var_dnm_dn8 / (2.0 * assign87910_body0_e134001)), (locals.var_dnm_dn9 / (2.0 * assign87910_body0_e134001)), (locals.var_dnm_dn10 / (2.0 * assign87910_body0_e134001)), (locals.var_dnm_dn13 / (2.0 * assign87910_body0_e134001)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign87910_body0_e134003;
            locals.var_dnm_dn0 = assign87910_body0_e134003_d_n0;
            locals.var_dnm_dn2 = assign87910_body0_e134003_d_n2;
            locals.var_dnm_dn4 = assign87910_body0_e134003_d_n4;
            locals.var_dnm_dn5 = assign87910_body0_e134003_d_n5;
            locals.var_dnm_dn6 = assign87910_body0_e134003_d_n6;
            locals.var_dnm_dn7 = assign87910_body0_e134003_d_n7;
            locals.var_dnm_dn8 = assign87910_body0_e134003_d_n8;
            locals.var_dnm_dn9 = assign87910_body0_e134003_d_n9;
            locals.var_dnm_dn10 = assign87910_body0_e134003_d_n10;
            locals.var_dnm_dn13 = assign87910_body0_e134003_d_n13;
            locals.var_dnm_rv = 0.0;
            let (assign87910_body1_e134020,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2035 != 0.0)) && (locals.var_guard2036 != 0.0)) {
        let assign87910_body1_e134018: f64 = (locals.var_m0 + 1.0);
        (assign87910_body1_e134018,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign87910_body1_e134020;
            locals.var_m0_rv = 0.0;
        }

        let (assign87920_e134047, assign87920_e134047_d_n0, assign87920_e134047_d_n2, assign87920_e134047_d_n4, assign87920_e134047_d_n5, assign87920_e134047_d_n6, assign87920_e134047_d_n7, assign87920_e134047_d_n8, assign87920_e134047_d_n9, assign87920_e134047_d_n10, assign87920_e134047_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2035 != 0.0)) && (locals.var_guard2036 == 0.0)) {
        let (assign87920_e134045, assign87920_e134045_d_n0, assign87920_e134045_d_n2, assign87920_e134045_d_n4, assign87920_e134045_d_n5, assign87920_e134045_d_n6, assign87920_e134045_d_n7, assign87920_e134045_d_n8, assign87920_e134045_d_n9, assign87920_e134045_d_n10, assign87920_e134045_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign87920_e134042: f64 = (2.0 * 2.0);
                let assign87920_e134043: f64 = (1.0 / assign87920_e134042);
                let assign87920_e134044: f64 = (locals.var_dnm).powf(assign87920_e134043);
                (assign87920_e134044, if 0.0 == 0.0 && ((assign87920_e134043) as f64).is_finite() && ((assign87920_e134043) as f64).fract() == 0.0 { if assign87920_e134043 == 0.0 { 0.0 } else { (assign87920_e134043 * ((locals.var_dnm).powf(assign87920_e134043 - 1.0) * locals.var_dnm_dn0)) } } else { (assign87920_e134044 * (assign87920_e134043 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign87920_e134043) as f64).is_finite() && ((assign87920_e134043) as f64).fract() == 0.0 { if assign87920_e134043 == 0.0 { 0.0 } else { (assign87920_e134043 * ((locals.var_dnm).powf(assign87920_e134043 - 1.0) * locals.var_dnm_dn2)) } } else { (assign87920_e134044 * (assign87920_e134043 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign87920_e134043) as f64).is_finite() && ((assign87920_e134043) as f64).fract() == 0.0 { if assign87920_e134043 == 0.0 { 0.0 } else { (assign87920_e134043 * ((locals.var_dnm).powf(assign87920_e134043 - 1.0) * locals.var_dnm_dn4)) } } else { (assign87920_e134044 * (assign87920_e134043 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign87920_e134043) as f64).is_finite() && ((assign87920_e134043) as f64).fract() == 0.0 { if assign87920_e134043 == 0.0 { 0.0 } else { (assign87920_e134043 * ((locals.var_dnm).powf(assign87920_e134043 - 1.0) * locals.var_dnm_dn5)) } } else { (assign87920_e134044 * (assign87920_e134043 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign87920_e134043) as f64).is_finite() && ((assign87920_e134043) as f64).fract() == 0.0 { if assign87920_e134043 == 0.0 { 0.0 } else { (assign87920_e134043 * ((locals.var_dnm).powf(assign87920_e134043 - 1.0) * locals.var_dnm_dn6)) } } else { (assign87920_e134044 * (assign87920_e134043 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign87920_e134043) as f64).is_finite() && ((assign87920_e134043) as f64).fract() == 0.0 { if assign87920_e134043 == 0.0 { 0.0 } else { (assign87920_e134043 * ((locals.var_dnm).powf(assign87920_e134043 - 1.0) * locals.var_dnm_dn7)) } } else { (assign87920_e134044 * (assign87920_e134043 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign87920_e134043) as f64).is_finite() && ((assign87920_e134043) as f64).fract() == 0.0 { if assign87920_e134043 == 0.0 { 0.0 } else { (assign87920_e134043 * ((locals.var_dnm).powf(assign87920_e134043 - 1.0) * locals.var_dnm_dn8)) } } else { (assign87920_e134044 * (assign87920_e134043 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign87920_e134043) as f64).is_finite() && ((assign87920_e134043) as f64).fract() == 0.0 { if assign87920_e134043 == 0.0 { 0.0 } else { (assign87920_e134043 * ((locals.var_dnm).powf(assign87920_e134043 - 1.0) * locals.var_dnm_dn9)) } } else { (assign87920_e134044 * (assign87920_e134043 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign87920_e134043) as f64).is_finite() && ((assign87920_e134043) as f64).fract() == 0.0 { if assign87920_e134043 == 0.0 { 0.0 } else { (assign87920_e134043 * ((locals.var_dnm).powf(assign87920_e134043 - 1.0) * locals.var_dnm_dn10)) } } else { (assign87920_e134044 * (assign87920_e134043 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign87920_e134043) as f64).is_finite() && ((assign87920_e134043) as f64).fract() == 0.0 { if assign87920_e134043 == 0.0 { 0.0 } else { (assign87920_e134043 * ((locals.var_dnm).powf(assign87920_e134043 - 1.0) * locals.var_dnm_dn13)) } } else { (assign87920_e134044 * (assign87920_e134043 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign87920_e134045, assign87920_e134045_d_n0, assign87920_e134045_d_n2, assign87920_e134045_d_n4, assign87920_e134045_d_n5, assign87920_e134045_d_n6, assign87920_e134045_d_n7, assign87920_e134045_d_n8, assign87920_e134045_d_n9, assign87920_e134045_d_n10, assign87920_e134045_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign87920_e134047;
        locals.var_dnm_dn0 = assign87920_e134047_d_n0;
        locals.var_dnm_dn2 = assign87920_e134047_d_n2;
        locals.var_dnm_dn4 = assign87920_e134047_d_n4;
        locals.var_dnm_dn5 = assign87920_e134047_d_n5;
        locals.var_dnm_dn6 = assign87920_e134047_d_n6;
        locals.var_dnm_dn7 = assign87920_e134047_d_n7;
        locals.var_dnm_dn8 = assign87920_e134047_d_n8;
        locals.var_dnm_dn9 = assign87920_e134047_d_n9;
        locals.var_dnm_dn10 = assign87920_e134047_d_n10;
        locals.var_dnm_dn13 = assign87920_e134047_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign87930_e134062, assign87930_e134062_d_n0, assign87930_e134062_d_n2, assign87930_e134062_d_n4, assign87930_e134062_d_n5, assign87930_e134062_d_n6, assign87930_e134062_d_n7, assign87930_e134062_d_n8, assign87930_e134062_d_n9, assign87930_e134062_d_n10, assign87930_e134062_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2035 != 0.0)) {
        let assign87930_e134060: f64 = (1.0 / locals.var_dnm);
        (assign87930_e134060, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign87930_e134062;
        locals.var_dnm_dn0 = assign87930_e134062_d_n0;
        locals.var_dnm_dn2 = assign87930_e134062_d_n2;
        locals.var_dnm_dn4 = assign87930_e134062_d_n4;
        locals.var_dnm_dn5 = assign87930_e134062_d_n5;
        locals.var_dnm_dn6 = assign87930_e134062_d_n6;
        locals.var_dnm_dn7 = assign87930_e134062_d_n7;
        locals.var_dnm_dn8 = assign87930_e134062_d_n8;
        locals.var_dnm_dn9 = assign87930_e134062_d_n9;
        locals.var_dnm_dn10 = assign87930_e134062_d_n10;
        locals.var_dnm_dn13 = assign87930_e134062_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign87940_e134081, assign87940_e134081_d_n0, assign87940_e134081_d_n2, assign87940_e134081_d_n4, assign87940_e134081_d_n5, assign87940_e134081_d_n6, assign87940_e134081_d_n7, assign87940_e134081_d_n8, assign87940_e134081_d_n9, assign87940_e134081_d_n10, assign87940_e134081_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2035 != 0.0)) {
        let assign87940_e134076: f64 = (0.2 * locals.var_chi_b);
        let assign87940_e134077: f64 = (locals.var_tmf1 * assign87940_e134076);
        let assign87940_e134079: f64 = (assign87940_e134077 * locals.var_dnm);
        (assign87940_e134079, ((((locals.var_tmf1_dn0 * assign87940_e134076) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn0))) * locals.var_dnm) + (assign87940_e134077 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * assign87940_e134076) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn2))) * locals.var_dnm) + (assign87940_e134077 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn4 * assign87940_e134076) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn4))) * locals.var_dnm) + (assign87940_e134077 * locals.var_dnm_dn4)), ((((locals.var_tmf1_dn5 * assign87940_e134076) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn5))) * locals.var_dnm) + (assign87940_e134077 * locals.var_dnm_dn5)), ((((locals.var_tmf1_dn6 * assign87940_e134076) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn6))) * locals.var_dnm) + (assign87940_e134077 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * assign87940_e134076) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn7))) * locals.var_dnm) + (assign87940_e134077 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn8 * assign87940_e134076) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn8))) * locals.var_dnm) + (assign87940_e134077 * locals.var_dnm_dn8)), ((((locals.var_tmf1_dn9 * assign87940_e134076) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn9))) * locals.var_dnm) + (assign87940_e134077 * locals.var_dnm_dn9)), ((((locals.var_tmf1_dn10 * assign87940_e134076) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn10))) * locals.var_dnm) + (assign87940_e134077 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn13 * assign87940_e134076) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn13))) * locals.var_dnm) + (assign87940_e134077 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign87940_e134081;
        locals.var_tmf0_dn0 = assign87940_e134081_d_n0;
        locals.var_tmf0_dn2 = assign87940_e134081_d_n2;
        locals.var_tmf0_dn4 = assign87940_e134081_d_n4;
        locals.var_tmf0_dn5 = assign87940_e134081_d_n5;
        locals.var_tmf0_dn6 = assign87940_e134081_d_n6;
        locals.var_tmf0_dn7 = assign87940_e134081_d_n7;
        locals.var_tmf0_dn8 = assign87940_e134081_d_n8;
        locals.var_tmf0_dn9 = assign87940_e134081_d_n9;
        locals.var_tmf0_dn10 = assign87940_e134081_d_n10;
        locals.var_tmf0_dn13 = assign87940_e134081_d_n13;
        locals.var_tmf0_rv = 0.0;

        let (assign87950_e134102, assign87950_e134102_d_n0, assign87950_e134102_d_n2, assign87950_e134102_d_n4, assign87950_e134102_d_n5, assign87950_e134102_d_n6, assign87950_e134102_d_n7, assign87950_e134102_d_n8, assign87950_e134102_d_n9, assign87950_e134102_d_n10, assign87950_e134102_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2035 != 0.0)) {
        let assign87950_e134094: f64 = (0.2 * locals.var_chi_b);
        let assign87950_e134096: f64 = (assign87950_e134094 * locals.var_xmp);
        let assign87950_e134098: f64 = (assign87950_e134096 * locals.var_dnm);
        let assign87950_e134100: f64 = (assign87950_e134098 / locals.var_arg);
        (assign87950_e134100, ((((((((0.2 * locals.var_chi_b_dn0) * locals.var_xmp) + (assign87950_e134094 * locals.var_xmp_dn0)) * locals.var_dnm) + (assign87950_e134096 * locals.var_dnm_dn0)) * locals.var_arg) - (assign87950_e134098 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn2) * locals.var_xmp) + (assign87950_e134094 * locals.var_xmp_dn2)) * locals.var_dnm) + (assign87950_e134096 * locals.var_dnm_dn2)) * locals.var_arg) - (assign87950_e134098 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn4) * locals.var_xmp) + (assign87950_e134094 * locals.var_xmp_dn4)) * locals.var_dnm) + (assign87950_e134096 * locals.var_dnm_dn4)) * locals.var_arg) - (assign87950_e134098 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn5) * locals.var_xmp) + (assign87950_e134094 * locals.var_xmp_dn5)) * locals.var_dnm) + (assign87950_e134096 * locals.var_dnm_dn5)) * locals.var_arg) - (assign87950_e134098 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn6) * locals.var_xmp) + (assign87950_e134094 * locals.var_xmp_dn6)) * locals.var_dnm) + (assign87950_e134096 * locals.var_dnm_dn6)) * locals.var_arg) - (assign87950_e134098 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn7) * locals.var_xmp) + (assign87950_e134094 * locals.var_xmp_dn7)) * locals.var_dnm) + (assign87950_e134096 * locals.var_dnm_dn7)) * locals.var_arg) - (assign87950_e134098 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn8) * locals.var_xmp) + (assign87950_e134094 * locals.var_xmp_dn8)) * locals.var_dnm) + (assign87950_e134096 * locals.var_dnm_dn8)) * locals.var_arg) - (assign87950_e134098 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn9) * locals.var_xmp) + (assign87950_e134094 * locals.var_xmp_dn9)) * locals.var_dnm) + (assign87950_e134096 * locals.var_dnm_dn9)) * locals.var_arg) - (assign87950_e134098 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn10) * locals.var_xmp) + (assign87950_e134094 * locals.var_xmp_dn10)) * locals.var_dnm) + (assign87950_e134096 * locals.var_dnm_dn10)) * locals.var_arg) - (assign87950_e134098 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn13) * locals.var_xmp) + (assign87950_e134094 * locals.var_xmp_dn13)) * locals.var_dnm) + (assign87950_e134096 * locals.var_dnm_dn13)) * locals.var_arg) - (assign87950_e134098 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign87950_e134102;
        locals.var_t1_dn0 = assign87950_e134102_d_n0;
        locals.var_t1_dn2 = assign87950_e134102_d_n2;
        locals.var_t1_dn4 = assign87950_e134102_d_n4;
        locals.var_t1_dn5 = assign87950_e134102_d_n5;
        locals.var_t1_dn6 = assign87950_e134102_d_n6;
        locals.var_t1_dn7 = assign87950_e134102_d_n7;
        locals.var_t1_dn8 = assign87950_e134102_d_n8;
        locals.var_t1_dn9 = assign87950_e134102_d_n9;
        locals.var_t1_dn10 = assign87950_e134102_d_n10;
        locals.var_t1_dn13 = assign87950_e134102_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign87960_e134121, assign87960_e134121_d_n0, assign87960_e134121_d_n2, assign87960_e134121_d_n4, assign87960_e134121_d_n5, assign87960_e134121_d_n6, assign87960_e134121_d_n7, assign87960_e134121_d_n8, assign87960_e134121_d_n9, assign87960_e134121_d_n10, assign87960_e134121_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2035 != 0.0)) {
        let assign87960_e134116: f64 = (0.2 * locals.var_chi_b);
        let assign87960_e134117: f64 = (locals.var_chi_b - assign87960_e134116);
        let assign87960_e134119: f64 = (assign87960_e134117 + locals.var_tmf0);
        (assign87960_e134119, ((locals.var_chi_b_dn0 - (0.2 * locals.var_chi_b_dn0)) + locals.var_tmf0_dn0), ((locals.var_chi_b_dn2 - (0.2 * locals.var_chi_b_dn2)) + locals.var_tmf0_dn2), ((locals.var_chi_b_dn4 - (0.2 * locals.var_chi_b_dn4)) + locals.var_tmf0_dn4), ((locals.var_chi_b_dn5 - (0.2 * locals.var_chi_b_dn5)) + locals.var_tmf0_dn5), ((locals.var_chi_b_dn6 - (0.2 * locals.var_chi_b_dn6)) + locals.var_tmf0_dn6), ((locals.var_chi_b_dn7 - (0.2 * locals.var_chi_b_dn7)) + locals.var_tmf0_dn7), ((locals.var_chi_b_dn8 - (0.2 * locals.var_chi_b_dn8)) + locals.var_tmf0_dn8), ((locals.var_chi_b_dn9 - (0.2 * locals.var_chi_b_dn9)) + locals.var_tmf0_dn9), ((locals.var_chi_b_dn10 - (0.2 * locals.var_chi_b_dn10)) + locals.var_tmf0_dn10), ((locals.var_chi_b_dn13 - (0.2 * locals.var_chi_b_dn13)) + locals.var_tmf0_dn13),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    }
};
        locals.var_chi = assign87960_e134121;
        locals.var_chi_dn0 = assign87960_e134121_d_n0;
        locals.var_chi_dn2 = assign87960_e134121_d_n2;
        locals.var_chi_dn4 = assign87960_e134121_d_n4;
        locals.var_chi_dn5 = assign87960_e134121_d_n5;
        locals.var_chi_dn6 = assign87960_e134121_d_n6;
        locals.var_chi_dn7 = assign87960_e134121_d_n7;
        locals.var_chi_dn8 = assign87960_e134121_d_n8;
        locals.var_chi_dn9 = assign87960_e134121_d_n9;
        locals.var_chi_dn10 = assign87960_e134121_d_n10;
        locals.var_chi_dn13 = assign87960_e134121_d_n13;
        locals.var_chi_rv = 0.0;

        let (assign87970_e134134, assign87970_e134134_d_n0, assign87970_e134134_d_n2, assign87970_e134134_d_n4, assign87970_e134134_d_n5, assign87970_e134134_d_n6, assign87970_e134134_d_n7, assign87970_e134134_d_n8, assign87970_e134134_d_n9, assign87970_e134134_d_n10, assign87970_e134134_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2035 != 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign87970_e134134;
        locals.var_t1_dn0 = assign87970_e134134_d_n0;
        locals.var_t1_dn2 = assign87970_e134134_d_n2;
        locals.var_t1_dn4 = assign87970_e134134_d_n4;
        locals.var_t1_dn5 = assign87970_e134134_d_n5;
        locals.var_t1_dn6 = assign87970_e134134_d_n6;
        locals.var_t1_dn7 = assign87970_e134134_d_n7;
        locals.var_t1_dn8 = assign87970_e134134_d_n8;
        locals.var_t1_dn9 = assign87970_e134134_d_n9;
        locals.var_t1_dn10 = assign87970_e134134_d_n10;
        locals.var_t1_dn13 = assign87970_e134134_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign87980_e134148, assign87980_e134148_d_n0, assign87980_e134148_d_n2, assign87980_e134148_d_n4, assign87980_e134148_d_n5, assign87980_e134148_d_n6, assign87980_e134148_d_n7, assign87980_e134148_d_n8, assign87980_e134148_d_n9, assign87980_e134148_d_n10, assign87980_e134148_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2035 == 0.0)) {
        (locals.var_chi_a, locals.var_chi_a_dn0, locals.var_chi_a_dn2, locals.var_chi_a_dn4, locals.var_chi_a_dn5, locals.var_chi_a_dn6, locals.var_chi_a_dn7, locals.var_chi_a_dn8, locals.var_chi_a_dn9, locals.var_chi_a_dn10, locals.var_chi_a_dn13,)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    }
};
        locals.var_chi = assign87980_e134148;
        locals.var_chi_dn0 = assign87980_e134148_d_n0;
        locals.var_chi_dn2 = assign87980_e134148_d_n2;
        locals.var_chi_dn4 = assign87980_e134148_d_n4;
        locals.var_chi_dn5 = assign87980_e134148_d_n5;
        locals.var_chi_dn6 = assign87980_e134148_d_n6;
        locals.var_chi_dn7 = assign87980_e134148_d_n7;
        locals.var_chi_dn8 = assign87980_e134148_d_n8;
        locals.var_chi_dn9 = assign87980_e134148_d_n9;
        locals.var_chi_dn10 = assign87980_e134148_d_n10;
        locals.var_chi_dn13 = assign87980_e134148_d_n13;
        locals.var_chi_rv = 0.0;

        let (assign87990_e134162, assign87990_e134162_d_n0, assign87990_e134162_d_n2, assign87990_e134162_d_n4, assign87990_e134162_d_n5, assign87990_e134162_d_n6, assign87990_e134162_d_n7, assign87990_e134162_d_n8, assign87990_e134162_d_n9, assign87990_e134162_d_n10, assign87990_e134162_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2035 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign87990_e134162;
        locals.var_t1_dn0 = assign87990_e134162_d_n0;
        locals.var_t1_dn2 = assign87990_e134162_d_n2;
        locals.var_t1_dn4 = assign87990_e134162_d_n4;
        locals.var_t1_dn5 = assign87990_e134162_d_n5;
        locals.var_t1_dn6 = assign87990_e134162_d_n6;
        locals.var_t1_dn7 = assign87990_e134162_d_n7;
        locals.var_t1_dn8 = assign87990_e134162_d_n8;
        locals.var_t1_dn9 = assign87990_e134162_d_n9;
        locals.var_t1_dn10 = assign87990_e134162_d_n10;
        locals.var_t1_dn13 = assign87990_e134162_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign88000_e134179, assign88000_e134179_d_n0, assign88000_e134179_d_n2, assign88000_e134179_d_n4, assign88000_e134179_d_n5, assign88000_e134179_d_n6, assign88000_e134179_d_n7, assign88000_e134179_d_n8, assign88000_e134179_d_n9, assign88000_e134179_d_n10, assign88000_e134179_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) && (locals.var_guard2034 == 0.0)) {
        let (assign88000_e134177, assign88000_e134177_d_n0, assign88000_e134177_d_n2, assign88000_e134177_d_n4, assign88000_e134177_d_n5, assign88000_e134177_d_n6, assign88000_e134177_d_n7, assign88000_e134177_d_n8, assign88000_e134177_d_n9, assign88000_e134177_d_n10, assign88000_e134177_d_n13,) = {
            if (locals.var_chi_a <= locals.var_chi_b) {
                (locals.var_chi_a, locals.var_chi_a_dn0, locals.var_chi_a_dn2, locals.var_chi_a_dn4, locals.var_chi_a_dn5, locals.var_chi_a_dn6, locals.var_chi_a_dn7, locals.var_chi_a_dn8, locals.var_chi_a_dn9, locals.var_chi_a_dn10, locals.var_chi_a_dn13,)
            } else {
                (locals.var_chi_b, locals.var_chi_b_dn0, locals.var_chi_b_dn2, locals.var_chi_b_dn4, locals.var_chi_b_dn5, locals.var_chi_b_dn6, locals.var_chi_b_dn7, locals.var_chi_b_dn8, locals.var_chi_b_dn9, locals.var_chi_b_dn10, locals.var_chi_b_dn13,)
            }
        };
        (assign88000_e134177, assign88000_e134177_d_n0, assign88000_e134177_d_n2, assign88000_e134177_d_n4, assign88000_e134177_d_n5, assign88000_e134177_d_n6, assign88000_e134177_d_n7, assign88000_e134177_d_n8, assign88000_e134177_d_n9, assign88000_e134177_d_n10, assign88000_e134177_d_n13,)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    }
};
        locals.var_chi = assign88000_e134179;
        locals.var_chi_dn0 = assign88000_e134179_d_n0;
        locals.var_chi_dn2 = assign88000_e134179_d_n2;
        locals.var_chi_dn4 = assign88000_e134179_d_n4;
        locals.var_chi_dn5 = assign88000_e134179_d_n5;
        locals.var_chi_dn6 = assign88000_e134179_d_n6;
        locals.var_chi_dn7 = assign88000_e134179_d_n7;
        locals.var_chi_dn8 = assign88000_e134179_d_n8;
        locals.var_chi_dn9 = assign88000_e134179_d_n9;
        locals.var_chi_dn10 = assign88000_e134179_d_n10;
        locals.var_chi_dn13 = assign88000_e134179_d_n13;
        locals.var_chi_rv = 0.0;

        let assign88010_e134182: f64 = if p.p33 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2041 = assign88010_e134182;
        locals.var_guard2041_rv = 0.0;

        let (assign88020_e134195, assign88020_e134195_d_n0, assign88020_e134195_d_n2, assign88020_e134195_d_n4, assign88020_e134195_d_n5, assign88020_e134195_d_n6, assign88020_e134195_d_n7, assign88020_e134195_d_n8, assign88020_e134195_d_n9, assign88020_e134195_d_n10, assign88020_e134195_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2041 != 0.0)) {
        let assign88020_e134191: f64 = (locals.var_chi * locals.var_beta_inv);
        let assign88020_e134193: f64 = (assign88020_e134191 - locals.var_vxbgmtcl);
        (assign88020_e134193, (((locals.var_chi_dn0 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn0)) - locals.var_vxbgmtcl_dn0), (((locals.var_chi_dn2 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn2)) - locals.var_vxbgmtcl_dn2), (((locals.var_chi_dn4 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn4)) - locals.var_vxbgmtcl_dn4), (((locals.var_chi_dn5 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn5)) - locals.var_vxbgmtcl_dn5), (((locals.var_chi_dn6 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn6)) - locals.var_vxbgmtcl_dn6), (((locals.var_chi_dn7 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn7)) - locals.var_vxbgmtcl_dn7), (((locals.var_chi_dn8 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn8)) - locals.var_vxbgmtcl_dn8), (((locals.var_chi_dn9 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn9)) - locals.var_vxbgmtcl_dn9), (((locals.var_chi_dn10 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn10)) - locals.var_vxbgmtcl_dn10), (((locals.var_chi_dn13 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn13)) - locals.var_vxbgmtcl_dn13),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    }
};
        locals.var_ps0ld = assign88020_e134195;
        locals.var_ps0ld_dn0 = assign88020_e134195_d_n0;
        locals.var_ps0ld_dn2 = assign88020_e134195_d_n2;
        locals.var_ps0ld_dn4 = assign88020_e134195_d_n4;
        locals.var_ps0ld_dn5 = assign88020_e134195_d_n5;
        locals.var_ps0ld_dn6 = assign88020_e134195_d_n6;
        locals.var_ps0ld_dn7 = assign88020_e134195_d_n7;
        locals.var_ps0ld_dn8 = assign88020_e134195_d_n8;
        locals.var_ps0ld_dn9 = assign88020_e134195_d_n9;
        locals.var_ps0ld_dn10 = assign88020_e134195_d_n10;
        locals.var_ps0ld_dn13 = assign88020_e134195_d_n13;
        locals.var_ps0ld_rv = 0.0;

        let assign88030_e134198: f64 = if locals.var_wdep_func > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2042 = assign88030_e134198;
        locals.var_guard2042_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_326(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign88040_e134211, assign88040_e134211_d_n0, assign88040_e134211_d_n2, assign88040_e134211_d_n4, assign88040_e134211_d_n5, assign88040_e134211_d_n6, assign88040_e134211_d_n7, assign88040_e134211_d_n8, assign88040_e134211_d_n9, assign88040_e134211_d_n10, assign88040_e134211_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2041 != 0.0)) && (locals.var_guard2042 != 0.0)) {
        let assign88040_e134209: f64 = (p.p334 - locals.var_wdep_func);
        (assign88040_e134209, (-locals.var_wdep_func_dn0), (-locals.var_wdep_func_dn2), (-locals.var_wdep_func_dn4), (-locals.var_wdep_func_dn5), (-locals.var_wdep_func_dn6), (-locals.var_wdep_func_dn7), (-locals.var_wdep_func_dn8), (-locals.var_wdep_func_dn9), (-locals.var_wdep_func_dn10), (-locals.var_wdep_func_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign88040_e134211;
        locals.var_t2_dn0 = assign88040_e134211_d_n0;
        locals.var_t2_dn2 = assign88040_e134211_d_n2;
        locals.var_t2_dn4 = assign88040_e134211_d_n4;
        locals.var_t2_dn5 = assign88040_e134211_d_n5;
        locals.var_t2_dn6 = assign88040_e134211_d_n6;
        locals.var_t2_dn7 = assign88040_e134211_d_n7;
        locals.var_t2_dn8 = assign88040_e134211_d_n8;
        locals.var_t2_dn9 = assign88040_e134211_d_n9;
        locals.var_t2_dn10 = assign88040_e134211_d_n10;
        locals.var_t2_dn13 = assign88040_e134211_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign88050_e134236, assign88050_e134236_d_n0, assign88050_e134236_d_n2, assign88050_e134236_d_n4, assign88050_e134236_d_n5, assign88050_e134236_d_n6, assign88050_e134236_d_n7, assign88050_e134236_d_n8, assign88050_e134236_d_n9, assign88050_e134236_d_n10, assign88050_e134236_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2041 != 0.0)) && (locals.var_guard2042 == 0.0)) {
        let assign88050_e134223: f64 = (locals.var_vdsi + p.p137);
        let assign88050_e134226: f64 = (locals.var_vdsi + p.p137);
        let assign88050_e134227: f64 = (assign88050_e134223 * assign88050_e134226);
        let assign88050_e134230: f64 = (4.0 * 0.1);
        let assign88050_e134232: f64 = (assign88050_e134230 * 0.1);
        let assign88050_e134233: f64 = (assign88050_e134227 + assign88050_e134232);
        let assign88050_e134234: f64 = (assign88050_e134233).sqrt();
        (assign88050_e134234, 0.0, 0.0, 0.0, (((locals.var_vdsi_dn5 * assign88050_e134226) + (assign88050_e134223 * locals.var_vdsi_dn5)) / (2.0 * assign88050_e134234)), 0.0, (((locals.var_vdsi_dn7 * assign88050_e134226) + (assign88050_e134223 * locals.var_vdsi_dn7)) / (2.0 * assign88050_e134234)), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign88050_e134236;
        locals.var_tmf2_dn0 = assign88050_e134236_d_n0;
        locals.var_tmf2_dn2 = assign88050_e134236_d_n2;
        locals.var_tmf2_dn4 = assign88050_e134236_d_n4;
        locals.var_tmf2_dn5 = assign88050_e134236_d_n5;
        locals.var_tmf2_dn6 = assign88050_e134236_d_n6;
        locals.var_tmf2_dn7 = assign88050_e134236_d_n7;
        locals.var_tmf2_dn8 = assign88050_e134236_d_n8;
        locals.var_tmf2_dn9 = assign88050_e134236_d_n9;
        locals.var_tmf2_dn10 = assign88050_e134236_d_n10;
        locals.var_tmf2_dn13 = assign88050_e134236_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign88060_e134256, assign88060_e134256_d_n0, assign88060_e134256_d_n2, assign88060_e134256_d_n4, assign88060_e134256_d_n5, assign88060_e134256_d_n6, assign88060_e134256_d_n7, assign88060_e134256_d_n8, assign88060_e134256_d_n9, assign88060_e134256_d_n10, assign88060_e134256_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2041 != 0.0)) && (locals.var_guard2042 == 0.0)) {
        let assign88060_e134250: f64 = (locals.var_vdsi + p.p137);
        let assign88060_e134252: f64 = (assign88060_e134250 / locals.var_tmf2);
        let assign88060_e134253: f64 = (1.0 + assign88060_e134252);
        let assign88060_e134254: f64 = (0.5 * assign88060_e134253);
        (assign88060_e134254, (0.5 * (-((assign88060_e134250 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign88060_e134250 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign88060_e134250 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (((locals.var_vdsi_dn5 * locals.var_tmf2) - (assign88060_e134250 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (-((assign88060_e134250 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (((locals.var_vdsi_dn7 * locals.var_tmf2) - (assign88060_e134250 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (-((assign88060_e134250 * locals.var_tmf2_dn8) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign88060_e134250 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign88060_e134250 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign88060_e134250 * locals.var_tmf2_dn13) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign88060_e134256;
        locals.var_t9_dn0 = assign88060_e134256_d_n0;
        locals.var_t9_dn2 = assign88060_e134256_d_n2;
        locals.var_t9_dn4 = assign88060_e134256_d_n4;
        locals.var_t9_dn5 = assign88060_e134256_d_n5;
        locals.var_t9_dn6 = assign88060_e134256_d_n6;
        locals.var_t9_dn7 = assign88060_e134256_d_n7;
        locals.var_t9_dn8 = assign88060_e134256_d_n8;
        locals.var_t9_dn9 = assign88060_e134256_d_n9;
        locals.var_t9_dn10 = assign88060_e134256_d_n10;
        locals.var_t9_dn13 = assign88060_e134256_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign88070_e134274, assign88070_e134274_d_n0, assign88070_e134274_d_n2, assign88070_e134274_d_n4, assign88070_e134274_d_n5, assign88070_e134274_d_n6, assign88070_e134274_d_n7, assign88070_e134274_d_n8, assign88070_e134274_d_n9, assign88070_e134274_d_n10, assign88070_e134274_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2041 != 0.0)) && (locals.var_guard2042 == 0.0)) {
        let assign88070_e134269: f64 = (locals.var_vdsi + p.p137);
        let assign88070_e134271: f64 = (assign88070_e134269 + locals.var_tmf2);
        let assign88070_e134272: f64 = (0.5 * assign88070_e134271);
        (assign88070_e134272, (0.5 * locals.var_tmf2_dn0), (0.5 * locals.var_tmf2_dn2), (0.5 * locals.var_tmf2_dn4), (0.5 * (locals.var_vdsi_dn5 + locals.var_tmf2_dn5)), (0.5 * locals.var_tmf2_dn6), (0.5 * (locals.var_vdsi_dn7 + locals.var_tmf2_dn7)), (0.5 * locals.var_tmf2_dn8), (0.5 * locals.var_tmf2_dn9), (0.5 * locals.var_tmf2_dn10), (0.5 * locals.var_tmf2_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign88070_e134274;
        locals.var_t2_dn0 = assign88070_e134274_d_n0;
        locals.var_t2_dn2 = assign88070_e134274_d_n2;
        locals.var_t2_dn4 = assign88070_e134274_d_n4;
        locals.var_t2_dn5 = assign88070_e134274_d_n5;
        locals.var_t2_dn6 = assign88070_e134274_d_n6;
        locals.var_t2_dn7 = assign88070_e134274_d_n7;
        locals.var_t2_dn8 = assign88070_e134274_d_n8;
        locals.var_t2_dn9 = assign88070_e134274_d_n9;
        locals.var_t2_dn10 = assign88070_e134274_d_n10;
        locals.var_t2_dn13 = assign88070_e134274_d_n13;
        locals.var_t2_rv = 0.0;

        let assign88080_e134277: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2043 = assign88080_e134277;
        locals.var_guard2043_rv = 0.0;

        let (assign88090_e134291, assign88090_e134291_d_n0, assign88090_e134291_d_n2, assign88090_e134291_d_n4, assign88090_e134291_d_n5, assign88090_e134291_d_n6, assign88090_e134291_d_n7, assign88090_e134291_d_n8, assign88090_e134291_d_n9, assign88090_e134291_d_n10, assign88090_e134291_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2041 != 0.0)) && (locals.var_guard2042 == 0.0)) && (locals.var_guard2043 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign88090_e134291;
        locals.var_t2_dn0 = assign88090_e134291_d_n0;
        locals.var_t2_dn2 = assign88090_e134291_d_n2;
        locals.var_t2_dn4 = assign88090_e134291_d_n4;
        locals.var_t2_dn5 = assign88090_e134291_d_n5;
        locals.var_t2_dn6 = assign88090_e134291_d_n6;
        locals.var_t2_dn7 = assign88090_e134291_d_n7;
        locals.var_t2_dn8 = assign88090_e134291_d_n8;
        locals.var_t2_dn9 = assign88090_e134291_d_n9;
        locals.var_t2_dn10 = assign88090_e134291_d_n10;
        locals.var_t2_dn13 = assign88090_e134291_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign88100_e134305, assign88100_e134305_d_n0, assign88100_e134305_d_n2, assign88100_e134305_d_n4, assign88100_e134305_d_n5, assign88100_e134305_d_n6, assign88100_e134305_d_n7, assign88100_e134305_d_n8, assign88100_e134305_d_n9, assign88100_e134305_d_n10, assign88100_e134305_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2041 != 0.0)) && (locals.var_guard2042 == 0.0)) && (locals.var_guard2043 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign88100_e134305;
        locals.var_t9_dn0 = assign88100_e134305_d_n0;
        locals.var_t9_dn2 = assign88100_e134305_d_n2;
        locals.var_t9_dn4 = assign88100_e134305_d_n4;
        locals.var_t9_dn5 = assign88100_e134305_d_n5;
        locals.var_t9_dn6 = assign88100_e134305_d_n6;
        locals.var_t9_dn7 = assign88100_e134305_d_n7;
        locals.var_t9_dn8 = assign88100_e134305_d_n8;
        locals.var_t9_dn9 = assign88100_e134305_d_n9;
        locals.var_t9_dn10 = assign88100_e134305_d_n10;
        locals.var_t9_dn13 = assign88100_e134305_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign88110_e134322, assign88110_e134322_d_n0, assign88110_e134322_d_n2, assign88110_e134322_d_n4, assign88110_e134322_d_n5, assign88110_e134322_d_n6, assign88110_e134322_d_n7, assign88110_e134322_d_n8, assign88110_e134322_d_n9, assign88110_e134322_d_n10, assign88110_e134322_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2041 != 0.0)) && (locals.var_guard2042 == 0.0)) {
        let assign88110_e134317: f64 = (locals.var_kjunc * locals.var_t2);
        let assign88110_e134318: f64 = (assign88110_e134317).sqrt();
        let assign88110_e134320: f64 = (assign88110_e134318 * p.p432);
        (assign88110_e134320, ((((locals.var_kjunc_dn0 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn0)) / (2.0 * assign88110_e134318)) * p.p432), ((((locals.var_kjunc_dn2 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn2)) / (2.0 * assign88110_e134318)) * p.p432), ((((locals.var_kjunc_dn4 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn4)) / (2.0 * assign88110_e134318)) * p.p432), ((((locals.var_kjunc_dn5 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn5)) / (2.0 * assign88110_e134318)) * p.p432), ((((locals.var_kjunc_dn6 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn6)) / (2.0 * assign88110_e134318)) * p.p432), ((((locals.var_kjunc_dn7 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn7)) / (2.0 * assign88110_e134318)) * p.p432), ((((locals.var_kjunc_dn8 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn8)) / (2.0 * assign88110_e134318)) * p.p432), ((((locals.var_kjunc_dn9 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn9)) / (2.0 * assign88110_e134318)) * p.p432), ((((locals.var_kjunc_dn10 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn10)) / (2.0 * assign88110_e134318)) * p.p432), ((((locals.var_kjunc_dn13 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn13)) / (2.0 * assign88110_e134318)) * p.p432),)
    } else {
        (locals.var_wjunc0, locals.var_wjunc0_dn0, locals.var_wjunc0_dn2, locals.var_wjunc0_dn4, locals.var_wjunc0_dn5, locals.var_wjunc0_dn6, locals.var_wjunc0_dn7, locals.var_wjunc0_dn8, locals.var_wjunc0_dn9, locals.var_wjunc0_dn10, locals.var_wjunc0_dn13,)
    }
};
        locals.var_wjunc0 = assign88110_e134322;
        locals.var_wjunc0_dn0 = assign88110_e134322_d_n0;
        locals.var_wjunc0_dn2 = assign88110_e134322_d_n2;
        locals.var_wjunc0_dn4 = assign88110_e134322_d_n4;
        locals.var_wjunc0_dn5 = assign88110_e134322_d_n5;
        locals.var_wjunc0_dn6 = assign88110_e134322_d_n6;
        locals.var_wjunc0_dn7 = assign88110_e134322_d_n7;
        locals.var_wjunc0_dn8 = assign88110_e134322_d_n8;
        locals.var_wjunc0_dn9 = assign88110_e134322_d_n9;
        locals.var_wjunc0_dn10 = assign88110_e134322_d_n10;
        locals.var_wjunc0_dn13 = assign88110_e134322_d_n13;
        locals.var_wjunc0_rv = 0.0;

        let (assign88120_e134336, assign88120_e134336_d_n0, assign88120_e134336_d_n2, assign88120_e134336_d_n4, assign88120_e134336_d_n5, assign88120_e134336_d_n6, assign88120_e134336_d_n7, assign88120_e134336_d_n8, assign88120_e134336_d_n9, assign88120_e134336_d_n10, assign88120_e134336_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2041 != 0.0)) && (locals.var_guard2042 == 0.0)) {
        let assign88120_e134334: f64 = (p.p334 - locals.var_wjunc0);
        (assign88120_e134334, (-locals.var_wjunc0_dn0), (-locals.var_wjunc0_dn2), (-locals.var_wjunc0_dn4), (-locals.var_wjunc0_dn5), (-locals.var_wjunc0_dn6), (-locals.var_wjunc0_dn7), (-locals.var_wjunc0_dn8), (-locals.var_wjunc0_dn9), (-locals.var_wjunc0_dn10), (-locals.var_wjunc0_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign88120_e134336;
        locals.var_t2_dn0 = assign88120_e134336_d_n0;
        locals.var_t2_dn2 = assign88120_e134336_d_n2;
        locals.var_t2_dn4 = assign88120_e134336_d_n4;
        locals.var_t2_dn5 = assign88120_e134336_d_n5;
        locals.var_t2_dn6 = assign88120_e134336_d_n6;
        locals.var_t2_dn7 = assign88120_e134336_d_n7;
        locals.var_t2_dn8 = assign88120_e134336_d_n8;
        locals.var_t2_dn9 = assign88120_e134336_d_n9;
        locals.var_t2_dn10 = assign88120_e134336_d_n10;
        locals.var_t2_dn13 = assign88120_e134336_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign88130_e134358, assign88130_e134358_d_n0, assign88130_e134358_d_n2, assign88130_e134358_d_n4, assign88130_e134358_d_n5, assign88130_e134358_d_n6, assign88130_e134358_d_n7, assign88130_e134358_d_n8, assign88130_e134358_d_n9, assign88130_e134358_d_n10, assign88130_e134358_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2041 != 0.0)) {
        let assign88130_e134345: f64 = (locals.var_t2 * locals.var_t2);
        let assign88130_e134349: f64 = (p.p334 * 0.01);
        let assign88130_e134350: f64 = (4.0 * assign88130_e134349);
        let assign88130_e134353: f64 = (p.p334 * 0.01);
        let assign88130_e134354: f64 = (assign88130_e134350 * assign88130_e134353);
        let assign88130_e134355: f64 = (assign88130_e134345 + assign88130_e134354);
        let assign88130_e134356: f64 = (assign88130_e134355).sqrt();
        (assign88130_e134356, (((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)) / (2.0 * assign88130_e134356)), (((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)) / (2.0 * assign88130_e134356)), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign88130_e134356)), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign88130_e134356)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign88130_e134356)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign88130_e134356)), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign88130_e134356)), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign88130_e134356)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign88130_e134356)), (((locals.var_t2_dn13 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn13)) / (2.0 * assign88130_e134356)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign88130_e134358;
        locals.var_tmf2_dn0 = assign88130_e134358_d_n0;
        locals.var_tmf2_dn2 = assign88130_e134358_d_n2;
        locals.var_tmf2_dn4 = assign88130_e134358_d_n4;
        locals.var_tmf2_dn5 = assign88130_e134358_d_n5;
        locals.var_tmf2_dn6 = assign88130_e134358_d_n6;
        locals.var_tmf2_dn7 = assign88130_e134358_d_n7;
        locals.var_tmf2_dn8 = assign88130_e134358_d_n8;
        locals.var_tmf2_dn9 = assign88130_e134358_d_n9;
        locals.var_tmf2_dn10 = assign88130_e134358_d_n10;
        locals.var_tmf2_dn13 = assign88130_e134358_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign88140_e134373, assign88140_e134373_d_n0, assign88140_e134373_d_n2, assign88140_e134373_d_n4, assign88140_e134373_d_n5, assign88140_e134373_d_n6, assign88140_e134373_d_n7, assign88140_e134373_d_n8, assign88140_e134373_d_n9, assign88140_e134373_d_n10, assign88140_e134373_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2041 != 0.0)) {
        let assign88140_e134369: f64 = (locals.var_t2 / locals.var_tmf2);
        let assign88140_e134370: f64 = (1.0 + assign88140_e134369);
        let assign88140_e134371: f64 = (0.5 * assign88140_e134370);
        (assign88140_e134371, (0.5 * (((locals.var_t2_dn0 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn2 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn4 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn5 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn6 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn7 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn8 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn9 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn10 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn13 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign88140_e134373;
        locals.var_t9_dn0 = assign88140_e134373_d_n0;
        locals.var_t9_dn2 = assign88140_e134373_d_n2;
        locals.var_t9_dn4 = assign88140_e134373_d_n4;
        locals.var_t9_dn5 = assign88140_e134373_d_n5;
        locals.var_t9_dn6 = assign88140_e134373_d_n6;
        locals.var_t9_dn7 = assign88140_e134373_d_n7;
        locals.var_t9_dn8 = assign88140_e134373_d_n8;
        locals.var_t9_dn9 = assign88140_e134373_d_n9;
        locals.var_t9_dn10 = assign88140_e134373_d_n10;
        locals.var_t9_dn13 = assign88140_e134373_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign88150_e134386, assign88150_e134386_d_n0, assign88150_e134386_d_n2, assign88150_e134386_d_n4, assign88150_e134386_d_n5, assign88150_e134386_d_n6, assign88150_e134386_d_n7, assign88150_e134386_d_n8, assign88150_e134386_d_n9, assign88150_e134386_d_n10, assign88150_e134386_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2041 != 0.0)) {
        let assign88150_e134383: f64 = (locals.var_t2 + locals.var_tmf2);
        let assign88150_e134384: f64 = (0.5 * assign88150_e134383);
        (assign88150_e134384, (0.5 * (locals.var_t2_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t2_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t2_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t2_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t2_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t2_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t2_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t2_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t2_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t2_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign88150_e134386;
        locals.var_t2_dn0 = assign88150_e134386_d_n0;
        locals.var_t2_dn2 = assign88150_e134386_d_n2;
        locals.var_t2_dn4 = assign88150_e134386_d_n4;
        locals.var_t2_dn5 = assign88150_e134386_d_n5;
        locals.var_t2_dn6 = assign88150_e134386_d_n6;
        locals.var_t2_dn7 = assign88150_e134386_d_n7;
        locals.var_t2_dn8 = assign88150_e134386_d_n8;
        locals.var_t2_dn9 = assign88150_e134386_d_n9;
        locals.var_t2_dn10 = assign88150_e134386_d_n10;
        locals.var_t2_dn13 = assign88150_e134386_d_n13;
        locals.var_t2_rv = 0.0;

        let assign88160_e134389: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2044 = assign88160_e134389;
        locals.var_guard2044_rv = 0.0;

        let (assign88170_e134400, assign88170_e134400_d_n0, assign88170_e134400_d_n2, assign88170_e134400_d_n4, assign88170_e134400_d_n5, assign88170_e134400_d_n6, assign88170_e134400_d_n7, assign88170_e134400_d_n8, assign88170_e134400_d_n9, assign88170_e134400_d_n10, assign88170_e134400_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2041 != 0.0)) && (locals.var_guard2044 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign88170_e134400;
        locals.var_t2_dn0 = assign88170_e134400_d_n0;
        locals.var_t2_dn2 = assign88170_e134400_d_n2;
        locals.var_t2_dn4 = assign88170_e134400_d_n4;
        locals.var_t2_dn5 = assign88170_e134400_d_n5;
        locals.var_t2_dn6 = assign88170_e134400_d_n6;
        locals.var_t2_dn7 = assign88170_e134400_d_n7;
        locals.var_t2_dn8 = assign88170_e134400_d_n8;
        locals.var_t2_dn9 = assign88170_e134400_d_n9;
        locals.var_t2_dn10 = assign88170_e134400_d_n10;
        locals.var_t2_dn13 = assign88170_e134400_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign88180_e134411, assign88180_e134411_d_n0, assign88180_e134411_d_n2, assign88180_e134411_d_n4, assign88180_e134411_d_n5, assign88180_e134411_d_n6, assign88180_e134411_d_n7, assign88180_e134411_d_n8, assign88180_e134411_d_n9, assign88180_e134411_d_n10, assign88180_e134411_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2041 != 0.0)) && (locals.var_guard2044 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign88180_e134411;
        locals.var_t9_dn0 = assign88180_e134411_d_n0;
        locals.var_t9_dn2 = assign88180_e134411_d_n2;
        locals.var_t9_dn4 = assign88180_e134411_d_n4;
        locals.var_t9_dn5 = assign88180_e134411_d_n5;
        locals.var_t9_dn6 = assign88180_e134411_d_n6;
        locals.var_t9_dn7 = assign88180_e134411_d_n7;
        locals.var_t9_dn8 = assign88180_e134411_d_n8;
        locals.var_t9_dn9 = assign88180_e134411_d_n9;
        locals.var_t9_dn10 = assign88180_e134411_d_n10;
        locals.var_t9_dn13 = assign88180_e134411_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign88190_e134420, assign88190_e134420_d_n0, assign88190_e134420_d_n2, assign88190_e134420_d_n4, assign88190_e134420_d_n5, assign88190_e134420_d_n6, assign88190_e134420_d_n7, assign88190_e134420_d_n8, assign88190_e134420_d_n9, assign88190_e134420_d_n10, assign88190_e134420_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2041 != 0.0)) {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    } else {
        (locals.var_ddriftldc, locals.var_ddriftldc_dn0, locals.var_ddriftldc_dn2, locals.var_ddriftldc_dn4, locals.var_ddriftldc_dn5, locals.var_ddriftldc_dn6, locals.var_ddriftldc_dn7, locals.var_ddriftldc_dn8, locals.var_ddriftldc_dn9, locals.var_ddriftldc_dn10, locals.var_ddriftldc_dn13,)
    }
};
        locals.var_ddriftldc = assign88190_e134420;
        locals.var_ddriftldc_dn0 = assign88190_e134420_d_n0;
        locals.var_ddriftldc_dn2 = assign88190_e134420_d_n2;
        locals.var_ddriftldc_dn4 = assign88190_e134420_d_n4;
        locals.var_ddriftldc_dn5 = assign88190_e134420_d_n5;
        locals.var_ddriftldc_dn6 = assign88190_e134420_d_n6;
        locals.var_ddriftldc_dn7 = assign88190_e134420_d_n7;
        locals.var_ddriftldc_dn8 = assign88190_e134420_d_n8;
        locals.var_ddriftldc_dn9 = assign88190_e134420_d_n9;
        locals.var_ddriftldc_dn10 = assign88190_e134420_d_n10;
        locals.var_ddriftldc_dn13 = assign88190_e134420_d_n13;
        locals.var_ddriftldc_rv = 0.0;

        let (assign88200_e134437, assign88200_e134437_d_n0, assign88200_e134437_d_n2, assign88200_e134437_d_n4, assign88200_e134437_d_n5, assign88200_e134437_d_n6, assign88200_e134437_d_n7, assign88200_e134437_d_n8, assign88200_e134437_d_n9, assign88200_e134437_d_n10, assign88200_e134437_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2041 != 0.0)) {
        let assign88200_e134429: f64 = (locals.var_q_nsubld__blk2004 * locals.var_ddriftldc);
        let assign88200_e134431: f64 = (assign88200_e134429 * locals.var_ddriftldc);
        let assign88200_e134433: f64 = (assign88200_e134431 / 2.0);
        let assign88200_e134435: f64 = (assign88200_e134433 / 1.034943e-10);
        (assign88200_e134435, (((((locals.var_q_nsubld__blk2004 * locals.var_ddriftldc_dn0) * locals.var_ddriftldc) + (assign88200_e134429 * locals.var_ddriftldc_dn0)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2004 * locals.var_ddriftldc_dn2) * locals.var_ddriftldc) + (assign88200_e134429 * locals.var_ddriftldc_dn2)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2004 * locals.var_ddriftldc_dn4) * locals.var_ddriftldc) + (assign88200_e134429 * locals.var_ddriftldc_dn4)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2004 * locals.var_ddriftldc_dn5) * locals.var_ddriftldc) + (assign88200_e134429 * locals.var_ddriftldc_dn5)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2004 * locals.var_ddriftldc_dn6) * locals.var_ddriftldc) + (assign88200_e134429 * locals.var_ddriftldc_dn6)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2004 * locals.var_ddriftldc_dn7) * locals.var_ddriftldc) + (assign88200_e134429 * locals.var_ddriftldc_dn7)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2004 * locals.var_ddriftldc_dn8) * locals.var_ddriftldc) + (assign88200_e134429 * locals.var_ddriftldc_dn8)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2004 * locals.var_ddriftldc_dn9) * locals.var_ddriftldc) + (assign88200_e134429 * locals.var_ddriftldc_dn9)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2004 * locals.var_ddriftldc_dn10) * locals.var_ddriftldc) + (assign88200_e134429 * locals.var_ddriftldc_dn10)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2004 * locals.var_ddriftldc_dn13) * locals.var_ddriftldc) + (assign88200_e134429 * locals.var_ddriftldc_dn13)) / 2.0) / 1.034943e-10),)
    } else {
        (locals.var_dphi_sb, locals.var_dphi_sb_dn0, locals.var_dphi_sb_dn2, locals.var_dphi_sb_dn4, locals.var_dphi_sb_dn5, locals.var_dphi_sb_dn6, locals.var_dphi_sb_dn7, locals.var_dphi_sb_dn8, locals.var_dphi_sb_dn9, locals.var_dphi_sb_dn10, locals.var_dphi_sb_dn13,)
    }
};
        locals.var_dphi_sb = assign88200_e134437;
        locals.var_dphi_sb_dn0 = assign88200_e134437_d_n0;
        locals.var_dphi_sb_dn2 = assign88200_e134437_d_n2;
        locals.var_dphi_sb_dn4 = assign88200_e134437_d_n4;
        locals.var_dphi_sb_dn5 = assign88200_e134437_d_n5;
        locals.var_dphi_sb_dn6 = assign88200_e134437_d_n6;
        locals.var_dphi_sb_dn7 = assign88200_e134437_d_n7;
        locals.var_dphi_sb_dn8 = assign88200_e134437_d_n8;
        locals.var_dphi_sb_dn9 = assign88200_e134437_d_n9;
        locals.var_dphi_sb_dn10 = assign88200_e134437_d_n10;
        locals.var_dphi_sb_dn13 = assign88200_e134437_d_n13;
        locals.var_dphi_sb_rv = 0.0;

        let (assign88210_e134451, assign88210_e134451_d_n0, assign88210_e134451_d_n2, assign88210_e134451_d_n4, assign88210_e134451_d_n5, assign88210_e134451_d_n6, assign88210_e134451_d_n7, assign88210_e134451_d_n8, assign88210_e134451_d_n9, assign88210_e134451_d_n10, assign88210_e134451_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2041 != 0.0)) {
        let assign88210_e134446: f64 = (2.0 * locals.var_beta);
        let assign88210_e134448: f64 = (assign88210_e134446 * locals.var_dphi_sb);
        let assign88210_e134449: f64 = (assign88210_e134448).sqrt();
        (assign88210_e134449, ((((2.0 * locals.var_beta_dn0) * locals.var_dphi_sb) + (assign88210_e134446 * locals.var_dphi_sb_dn0)) / (2.0 * assign88210_e134449)), ((((2.0 * locals.var_beta_dn2) * locals.var_dphi_sb) + (assign88210_e134446 * locals.var_dphi_sb_dn2)) / (2.0 * assign88210_e134449)), ((((2.0 * locals.var_beta_dn4) * locals.var_dphi_sb) + (assign88210_e134446 * locals.var_dphi_sb_dn4)) / (2.0 * assign88210_e134449)), ((((2.0 * locals.var_beta_dn5) * locals.var_dphi_sb) + (assign88210_e134446 * locals.var_dphi_sb_dn5)) / (2.0 * assign88210_e134449)), ((((2.0 * locals.var_beta_dn6) * locals.var_dphi_sb) + (assign88210_e134446 * locals.var_dphi_sb_dn6)) / (2.0 * assign88210_e134449)), ((((2.0 * locals.var_beta_dn7) * locals.var_dphi_sb) + (assign88210_e134446 * locals.var_dphi_sb_dn7)) / (2.0 * assign88210_e134449)), ((((2.0 * locals.var_beta_dn8) * locals.var_dphi_sb) + (assign88210_e134446 * locals.var_dphi_sb_dn8)) / (2.0 * assign88210_e134449)), ((((2.0 * locals.var_beta_dn9) * locals.var_dphi_sb) + (assign88210_e134446 * locals.var_dphi_sb_dn9)) / (2.0 * assign88210_e134449)), ((((2.0 * locals.var_beta_dn10) * locals.var_dphi_sb) + (assign88210_e134446 * locals.var_dphi_sb_dn10)) / (2.0 * assign88210_e134449)), ((((2.0 * locals.var_beta_dn13) * locals.var_dphi_sb) + (assign88210_e134446 * locals.var_dphi_sb_dn13)) / (2.0 * assign88210_e134449)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign88210_e134451;
        locals.var_t0_dn0 = assign88210_e134451_d_n0;
        locals.var_t0_dn2 = assign88210_e134451_d_n2;
        locals.var_t0_dn4 = assign88210_e134451_d_n4;
        locals.var_t0_dn5 = assign88210_e134451_d_n5;
        locals.var_t0_dn6 = assign88210_e134451_d_n6;
        locals.var_t0_dn7 = assign88210_e134451_d_n7;
        locals.var_t0_dn8 = assign88210_e134451_d_n8;
        locals.var_t0_dn9 = assign88210_e134451_d_n9;
        locals.var_t0_dn10 = assign88210_e134451_d_n10;
        locals.var_t0_dn13 = assign88210_e134451_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign88220_e134467, assign88220_e134467_d_n0, assign88220_e134467_d_n2, assign88220_e134467_d_n4, assign88220_e134467_d_n5, assign88220_e134467_d_n6, assign88220_e134467_d_n7, assign88220_e134467_d_n8, assign88220_e134467_d_n9, assign88220_e134467_d_n10, assign88220_e134467_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2041 != 0.0)) {
        let assign88220_e134459: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign88220_e134461: f64 = (-locals.var_t0);
        let assign88220_e134462: f64 = { let limited_exp_arg = assign88220_e134461; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign88220_e134463: f64 = (assign88220_e134459 + assign88220_e134462);
        let assign88220_e134465: f64 = (assign88220_e134463 / 2.0);
        (assign88220_e134465, ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn0) + ({ let limited_exp_arg = assign88220_e134461; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn0))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn2) + ({ let limited_exp_arg = assign88220_e134461; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn2))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4) + ({ let limited_exp_arg = assign88220_e134461; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn4))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5) + ({ let limited_exp_arg = assign88220_e134461; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn5))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6) + ({ let limited_exp_arg = assign88220_e134461; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn6))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7) + ({ let limited_exp_arg = assign88220_e134461; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn7))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8) + ({ let limited_exp_arg = assign88220_e134461; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn8))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9) + ({ let limited_exp_arg = assign88220_e134461; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn9))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10) + ({ let limited_exp_arg = assign88220_e134461; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn10))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn13) + ({ let limited_exp_arg = assign88220_e134461; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn13))) / 2.0),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign88220_e134467;
        locals.var_t1_dn0 = assign88220_e134467_d_n0;
        locals.var_t1_dn2 = assign88220_e134467_d_n2;
        locals.var_t1_dn4 = assign88220_e134467_d_n4;
        locals.var_t1_dn5 = assign88220_e134467_d_n5;
        locals.var_t1_dn6 = assign88220_e134467_d_n6;
        locals.var_t1_dn7 = assign88220_e134467_d_n7;
        locals.var_t1_dn8 = assign88220_e134467_d_n8;
        locals.var_t1_dn9 = assign88220_e134467_d_n9;
        locals.var_t1_dn10 = assign88220_e134467_d_n10;
        locals.var_t1_dn13 = assign88220_e134467_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign88230_e134479, assign88230_e134479_d_n0, assign88230_e134479_d_n2, assign88230_e134479_d_n4, assign88230_e134479_d_n5, assign88230_e134479_d_n6, assign88230_e134479_d_n7, assign88230_e134479_d_n8, assign88230_e134479_d_n9, assign88230_e134479_d_n10, assign88230_e134479_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2041 != 0.0)) {
        let assign88230_e134475: f64 = (locals.var_t1).ln();
        let assign88230_e134477: f64 = (assign88230_e134475 / locals.var_dphi_sb);
        (assign88230_e134477, ((((locals.var_t1_dn0 / locals.var_t1) * locals.var_dphi_sb) - (assign88230_e134475 * locals.var_dphi_sb_dn0)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn2 / locals.var_t1) * locals.var_dphi_sb) - (assign88230_e134475 * locals.var_dphi_sb_dn2)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn4 / locals.var_t1) * locals.var_dphi_sb) - (assign88230_e134475 * locals.var_dphi_sb_dn4)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn5 / locals.var_t1) * locals.var_dphi_sb) - (assign88230_e134475 * locals.var_dphi_sb_dn5)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn6 / locals.var_t1) * locals.var_dphi_sb) - (assign88230_e134475 * locals.var_dphi_sb_dn6)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn7 / locals.var_t1) * locals.var_dphi_sb) - (assign88230_e134475 * locals.var_dphi_sb_dn7)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn8 / locals.var_t1) * locals.var_dphi_sb) - (assign88230_e134475 * locals.var_dphi_sb_dn8)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn9 / locals.var_t1) * locals.var_dphi_sb) - (assign88230_e134475 * locals.var_dphi_sb_dn9)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn10 / locals.var_t1) * locals.var_dphi_sb) - (assign88230_e134475 * locals.var_dphi_sb_dn10)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn13 / locals.var_t1) * locals.var_dphi_sb) - (assign88230_e134475 * locals.var_dphi_sb_dn13)) / (locals.var_dphi_sb * locals.var_dphi_sb)),)
    } else {
        (locals.var_c_sb, locals.var_c_sb_dn0, locals.var_c_sb_dn2, locals.var_c_sb_dn4, locals.var_c_sb_dn5, locals.var_c_sb_dn6, locals.var_c_sb_dn7, locals.var_c_sb_dn8, locals.var_c_sb_dn9, locals.var_c_sb_dn10, locals.var_c_sb_dn13,)
    }
};
        locals.var_c_sb = assign88230_e134479;
        locals.var_c_sb_dn0 = assign88230_e134479_d_n0;
        locals.var_c_sb_dn2 = assign88230_e134479_d_n2;
        locals.var_c_sb_dn4 = assign88230_e134479_d_n4;
        locals.var_c_sb_dn5 = assign88230_e134479_d_n5;
        locals.var_c_sb_dn6 = assign88230_e134479_d_n6;
        locals.var_c_sb_dn7 = assign88230_e134479_d_n7;
        locals.var_c_sb_dn8 = assign88230_e134479_d_n8;
        locals.var_c_sb_dn9 = assign88230_e134479_d_n9;
        locals.var_c_sb_dn10 = assign88230_e134479_d_n10;
        locals.var_c_sb_dn13 = assign88230_e134479_d_n13;
        locals.var_c_sb_rv = 0.0;

        let (assign88240_e134490, assign88240_e134490_d_n0, assign88240_e134490_d_n2, assign88240_e134490_d_n4, assign88240_e134490_d_n5, assign88240_e134490_d_n6, assign88240_e134490_d_n7, assign88240_e134490_d_n8, assign88240_e134490_d_n9, assign88240_e134490_d_n10, assign88240_e134490_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2041 != 0.0)) {
        let assign88240_e134488: f64 = (locals.var_ps0ld + locals.var_vxbgmtcl);
        (assign88240_e134488, (locals.var_ps0ld_dn0 + locals.var_vxbgmtcl_dn0), (locals.var_ps0ld_dn2 + locals.var_vxbgmtcl_dn2), (locals.var_ps0ld_dn4 + locals.var_vxbgmtcl_dn4), (locals.var_ps0ld_dn5 + locals.var_vxbgmtcl_dn5), (locals.var_ps0ld_dn6 + locals.var_vxbgmtcl_dn6), (locals.var_ps0ld_dn7 + locals.var_vxbgmtcl_dn7), (locals.var_ps0ld_dn8 + locals.var_vxbgmtcl_dn8), (locals.var_ps0ld_dn9 + locals.var_vxbgmtcl_dn9), (locals.var_ps0ld_dn10 + locals.var_vxbgmtcl_dn10), (locals.var_ps0ld_dn13 + locals.var_vxbgmtcl_dn13),)
    } else {
        (locals.var_ps0ld_vxb, locals.var_ps0ld_vxb_dn0, locals.var_ps0ld_vxb_dn2, locals.var_ps0ld_vxb_dn4, locals.var_ps0ld_vxb_dn5, locals.var_ps0ld_vxb_dn6, locals.var_ps0ld_vxb_dn7, locals.var_ps0ld_vxb_dn8, locals.var_ps0ld_vxb_dn9, locals.var_ps0ld_vxb_dn10, locals.var_ps0ld_vxb_dn13,)
    }
};
        locals.var_ps0ld_vxb = assign88240_e134490;
        locals.var_ps0ld_vxb_dn0 = assign88240_e134490_d_n0;
        locals.var_ps0ld_vxb_dn2 = assign88240_e134490_d_n2;
        locals.var_ps0ld_vxb_dn4 = assign88240_e134490_d_n4;
        locals.var_ps0ld_vxb_dn5 = assign88240_e134490_d_n5;
        locals.var_ps0ld_vxb_dn6 = assign88240_e134490_d_n6;
        locals.var_ps0ld_vxb_dn7 = assign88240_e134490_d_n7;
        locals.var_ps0ld_vxb_dn8 = assign88240_e134490_d_n8;
        locals.var_ps0ld_vxb_dn9 = assign88240_e134490_d_n9;
        locals.var_ps0ld_vxb_dn10 = assign88240_e134490_d_n10;
        locals.var_ps0ld_vxb_dn13 = assign88240_e134490_d_n13;
        locals.var_ps0ld_vxb_rv = 0.0;

        let (assign88250_e134503, assign88250_e134503_d_n0, assign88250_e134503_d_n2, assign88250_e134503_d_n4, assign88250_e134503_d_n5, assign88250_e134503_d_n6, assign88250_e134503_d_n7, assign88250_e134503_d_n8, assign88250_e134503_d_n9, assign88250_e134503_d_n10, assign88250_e134503_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2041 != 0.0)) {
        let assign88250_e134500: f64 = (locals.var_ps0ld_vxb - locals.var_dphi_sb);
        let assign88250_e134501: f64 = (locals.var_c_sb * assign88250_e134500);
        (assign88250_e134501, ((locals.var_c_sb_dn0 * assign88250_e134500) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn0 - locals.var_dphi_sb_dn0))), ((locals.var_c_sb_dn2 * assign88250_e134500) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn2 - locals.var_dphi_sb_dn2))), ((locals.var_c_sb_dn4 * assign88250_e134500) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn4 - locals.var_dphi_sb_dn4))), ((locals.var_c_sb_dn5 * assign88250_e134500) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn5 - locals.var_dphi_sb_dn5))), ((locals.var_c_sb_dn6 * assign88250_e134500) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn6 - locals.var_dphi_sb_dn6))), ((locals.var_c_sb_dn7 * assign88250_e134500) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn7 - locals.var_dphi_sb_dn7))), ((locals.var_c_sb_dn8 * assign88250_e134500) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn8 - locals.var_dphi_sb_dn8))), ((locals.var_c_sb_dn9 * assign88250_e134500) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn9 - locals.var_dphi_sb_dn9))), ((locals.var_c_sb_dn10 * assign88250_e134500) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn10 - locals.var_dphi_sb_dn10))), ((locals.var_c_sb_dn13 * assign88250_e134500) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn13 - locals.var_dphi_sb_dn13))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn13,)
    }
};
        locals.var_ty = assign88250_e134503;
        locals.var_ty_dn0 = assign88250_e134503_d_n0;
        locals.var_ty_dn2 = assign88250_e134503_d_n2;
        locals.var_ty_dn4 = assign88250_e134503_d_n4;
        locals.var_ty_dn5 = assign88250_e134503_d_n5;
        locals.var_ty_dn6 = assign88250_e134503_d_n6;
        locals.var_ty_dn7 = assign88250_e134503_d_n7;
        locals.var_ty_dn8 = assign88250_e134503_d_n8;
        locals.var_ty_dn9 = assign88250_e134503_d_n9;
        locals.var_ty_dn10 = assign88250_e134503_d_n10;
        locals.var_ty_dn13 = assign88250_e134503_d_n13;
        locals.var_ty_rv = 0.0;

        let assign88260_e134506: f64 = if locals.var_ty < 60.0 { 1.0 } else { 0.0 };
        locals.var_guard2045 = assign88260_e134506;
        locals.var_guard2045_rv = 0.0;

        let (assign88270_e134518, assign88270_e134518_d_n0, assign88270_e134518_d_n2, assign88270_e134518_d_n4, assign88270_e134518_d_n5, assign88270_e134518_d_n6, assign88270_e134518_d_n7, assign88270_e134518_d_n8, assign88270_e134518_d_n9, assign88270_e134518_d_n10, assign88270_e134518_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2041 != 0.0)) && (locals.var_guard2045 != 0.0)) {
        let assign88270_e134516: f64 = (locals.var_ty).exp();
        (assign88270_e134516, (assign88270_e134516 * locals.var_ty_dn0), (assign88270_e134516 * locals.var_ty_dn2), (assign88270_e134516 * locals.var_ty_dn4), (assign88270_e134516 * locals.var_ty_dn5), (assign88270_e134516 * locals.var_ty_dn6), (assign88270_e134516 * locals.var_ty_dn7), (assign88270_e134516 * locals.var_ty_dn8), (assign88270_e134516 * locals.var_ty_dn9), (assign88270_e134516 * locals.var_ty_dn10), (assign88270_e134516 * locals.var_ty_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign88270_e134518;
        locals.var_t1_dn0 = assign88270_e134518_d_n0;
        locals.var_t1_dn2 = assign88270_e134518_d_n2;
        locals.var_t1_dn4 = assign88270_e134518_d_n4;
        locals.var_t1_dn5 = assign88270_e134518_d_n5;
        locals.var_t1_dn6 = assign88270_e134518_d_n6;
        locals.var_t1_dn7 = assign88270_e134518_d_n7;
        locals.var_t1_dn8 = assign88270_e134518_d_n8;
        locals.var_t1_dn9 = assign88270_e134518_d_n9;
        locals.var_t1_dn10 = assign88270_e134518_d_n10;
        locals.var_t1_dn13 = assign88270_e134518_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign88280_e134533, assign88280_e134533_d_n0, assign88280_e134533_d_n2, assign88280_e134533_d_n4, assign88280_e134533_d_n5, assign88280_e134533_d_n6, assign88280_e134533_d_n7, assign88280_e134533_d_n8, assign88280_e134533_d_n9, assign88280_e134533_d_n10, assign88280_e134533_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2041 != 0.0)) && (locals.var_guard2045 != 0.0)) {
        let assign88280_e134528: f64 = (-locals.var_c_sb);
        let assign88280_e134530: f64 = (assign88280_e134528 * locals.var_dphi_sb);
        let assign88280_e134531: f64 = (assign88280_e134530).exp();
        (assign88280_e134531, (assign88280_e134531 * (((-locals.var_c_sb_dn0) * locals.var_dphi_sb) + (assign88280_e134528 * locals.var_dphi_sb_dn0))), (assign88280_e134531 * (((-locals.var_c_sb_dn2) * locals.var_dphi_sb) + (assign88280_e134528 * locals.var_dphi_sb_dn2))), (assign88280_e134531 * (((-locals.var_c_sb_dn4) * locals.var_dphi_sb) + (assign88280_e134528 * locals.var_dphi_sb_dn4))), (assign88280_e134531 * (((-locals.var_c_sb_dn5) * locals.var_dphi_sb) + (assign88280_e134528 * locals.var_dphi_sb_dn5))), (assign88280_e134531 * (((-locals.var_c_sb_dn6) * locals.var_dphi_sb) + (assign88280_e134528 * locals.var_dphi_sb_dn6))), (assign88280_e134531 * (((-locals.var_c_sb_dn7) * locals.var_dphi_sb) + (assign88280_e134528 * locals.var_dphi_sb_dn7))), (assign88280_e134531 * (((-locals.var_c_sb_dn8) * locals.var_dphi_sb) + (assign88280_e134528 * locals.var_dphi_sb_dn8))), (assign88280_e134531 * (((-locals.var_c_sb_dn9) * locals.var_dphi_sb) + (assign88280_e134528 * locals.var_dphi_sb_dn9))), (assign88280_e134531 * (((-locals.var_c_sb_dn10) * locals.var_dphi_sb) + (assign88280_e134528 * locals.var_dphi_sb_dn10))), (assign88280_e134531 * (((-locals.var_c_sb_dn13) * locals.var_dphi_sb) + (assign88280_e134528 * locals.var_dphi_sb_dn13))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign88280_e134533;
        locals.var_t0_dn0 = assign88280_e134533_d_n0;
        locals.var_t0_dn2 = assign88280_e134533_d_n2;
        locals.var_t0_dn4 = assign88280_e134533_d_n4;
        locals.var_t0_dn5 = assign88280_e134533_d_n5;
        locals.var_t0_dn6 = assign88280_e134533_d_n6;
        locals.var_t0_dn7 = assign88280_e134533_d_n7;
        locals.var_t0_dn8 = assign88280_e134533_d_n8;
        locals.var_t0_dn9 = assign88280_e134533_d_n9;
        locals.var_t0_dn10 = assign88280_e134533_d_n10;
        locals.var_t0_dn13 = assign88280_e134533_d_n13;
        locals.var_t0_rv = 0.0;

    }
}
