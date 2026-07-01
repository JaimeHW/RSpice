#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_288(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign79480_e122098, assign79480_e122098_d_n0, assign79480_e122098_d_n2, assign79480_e122098_d_n4, assign79480_e122098_d_n5, assign79480_e122098_d_n6, assign79480_e122098_d_n7, assign79480_e122098_d_n8, assign79480_e122098_d_n9, assign79480_e122098_d_n10, assign79480_e122098_d_n11, assign79480_e122098_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1845 != 0.0)) {
        let (assign79480_e122096, assign79480_e122096_d_n0, assign79480_e122096_d_n2, assign79480_e122096_d_n4, assign79480_e122096_d_n5, assign79480_e122096_d_n6, assign79480_e122096_d_n7, assign79480_e122096_d_n8, assign79480_e122096_d_n9, assign79480_e122096_d_n10, assign79480_e122096_d_n11, assign79480_e122096_d_n14,) = {
            if (locals.var_fbsq__blk1772 >= 0.0) {
                let (assign79480_e122091,) = {
                    if (locals.var_chi >= 0.0) {
                        (1.0,)
                    } else {
                        let assign79480_e122090: f64 = (-1.0);
                        (assign79480_e122090,)
                    }
                };
                let assign79480_e122093: f64 = (locals.var_fbsq__blk1772).sqrt();
                let assign79480_e122094: f64 = (assign79480_e122091 * assign79480_e122093);
                (assign79480_e122094, (assign79480_e122091 * (locals.var_fbsq__blk1772_dn0 / (2.0 * assign79480_e122093))), (assign79480_e122091 * (locals.var_fbsq__blk1772_dn2 / (2.0 * assign79480_e122093))), (assign79480_e122091 * (locals.var_fbsq__blk1772_dn4 / (2.0 * assign79480_e122093))), (assign79480_e122091 * (locals.var_fbsq__blk1772_dn5 / (2.0 * assign79480_e122093))), (assign79480_e122091 * (locals.var_fbsq__blk1772_dn6 / (2.0 * assign79480_e122093))), (assign79480_e122091 * (locals.var_fbsq__blk1772_dn7 / (2.0 * assign79480_e122093))), (assign79480_e122091 * (locals.var_fbsq__blk1772_dn8 / (2.0 * assign79480_e122093))), (assign79480_e122091 * (locals.var_fbsq__blk1772_dn9 / (2.0 * assign79480_e122093))), (assign79480_e122091 * (locals.var_fbsq__blk1772_dn10 / (2.0 * assign79480_e122093))), (assign79480_e122091 * (locals.var_fbsq__blk1772_dn11 / (2.0 * assign79480_e122093))), (assign79480_e122091 * (locals.var_fbsq__blk1772_dn14 / (2.0 * assign79480_e122093))),)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign79480_e122096, assign79480_e122096_d_n0, assign79480_e122096_d_n2, assign79480_e122096_d_n4, assign79480_e122096_d_n5, assign79480_e122096_d_n6, assign79480_e122096_d_n7, assign79480_e122096_d_n8, assign79480_e122096_d_n9, assign79480_e122096_d_n10, assign79480_e122096_d_n11, assign79480_e122096_d_n14,)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn14,)
    }
};
        locals.var_fb = assign79480_e122098;
        locals.var_fb_dn0 = assign79480_e122098_d_n0;
        locals.var_fb_dn2 = assign79480_e122098_d_n2;
        locals.var_fb_dn4 = assign79480_e122098_d_n4;
        locals.var_fb_dn5 = assign79480_e122098_d_n5;
        locals.var_fb_dn6 = assign79480_e122098_d_n6;
        locals.var_fb_dn7 = assign79480_e122098_d_n7;
        locals.var_fb_dn8 = assign79480_e122098_d_n8;
        locals.var_fb_dn9 = assign79480_e122098_d_n9;
        locals.var_fb_dn10 = assign79480_e122098_d_n10;
        locals.var_fb_dn11 = assign79480_e122098_d_n11;
        locals.var_fb_dn14 = assign79480_e122098_d_n14;

        let (assign79490_e122106, assign79490_e122106_d_n0, assign79490_e122106_d_n2, assign79490_e122106_d_n4, assign79490_e122106_d_n5, assign79490_e122106_d_n6, assign79490_e122106_d_n7, assign79490_e122106_d_n8, assign79490_e122106_d_n9, assign79490_e122106_d_n10, assign79490_e122106_d_n11, assign79490_e122106_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1845 != 0.0)) {
        let assign79490_e122104: f64 = (locals.var_c_w_ld * locals.var_fb);
        (assign79490_e122104, ((locals.var_c_w_ld_dn0 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn0)), ((locals.var_c_w_ld_dn2 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn2)), ((locals.var_c_w_ld_dn4 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn4)), ((locals.var_c_w_ld_dn5 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn5)), ((locals.var_c_w_ld_dn6 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn6)), ((locals.var_c_w_ld_dn7 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn7)), ((locals.var_c_w_ld_dn8 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn8)), ((locals.var_c_w_ld_dn9 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn9)), ((locals.var_c_w_ld_dn10 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn10)), ((locals.var_c_w_ld_dn11 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn11)), ((locals.var_c_w_ld_dn14 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn14)),)
    } else {
        (locals.var_wdld__blk1762, locals.var_wdld__blk1762_dn0, locals.var_wdld__blk1762_dn2, locals.var_wdld__blk1762_dn4, locals.var_wdld__blk1762_dn5, locals.var_wdld__blk1762_dn6, locals.var_wdld__blk1762_dn7, locals.var_wdld__blk1762_dn8, locals.var_wdld__blk1762_dn9, locals.var_wdld__blk1762_dn10, locals.var_wdld__blk1762_dn11, locals.var_wdld__blk1762_dn14,)
    }
};
        locals.var_wdld__blk1762 = assign79490_e122106;
        locals.var_wdld__blk1762_dn0 = assign79490_e122106_d_n0;
        locals.var_wdld__blk1762_dn2 = assign79490_e122106_d_n2;
        locals.var_wdld__blk1762_dn4 = assign79490_e122106_d_n4;
        locals.var_wdld__blk1762_dn5 = assign79490_e122106_d_n5;
        locals.var_wdld__blk1762_dn6 = assign79490_e122106_d_n6;
        locals.var_wdld__blk1762_dn7 = assign79490_e122106_d_n7;
        locals.var_wdld__blk1762_dn8 = assign79490_e122106_d_n8;
        locals.var_wdld__blk1762_dn9 = assign79490_e122106_d_n9;
        locals.var_wdld__blk1762_dn10 = assign79490_e122106_d_n10;
        locals.var_wdld__blk1762_dn11 = assign79490_e122106_d_n11;
        locals.var_wdld__blk1762_dn14 = assign79490_e122106_d_n14;

        let (assign79500_e122114, assign79500_e122114_d_n0, assign79500_e122114_d_n2, assign79500_e122114_d_n4, assign79500_e122114_d_n5, assign79500_e122114_d_n6, assign79500_e122114_d_n7, assign79500_e122114_d_n8, assign79500_e122114_d_n9, assign79500_e122114_d_n10, assign79500_e122114_d_n11, assign79500_e122114_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1845 != 0.0)) {
        let assign79500_e122112: f64 = (locals.var_q_nsubld__blk1764 * locals.var_wdld__blk1762);
        (assign79500_e122112, (locals.var_q_nsubld__blk1764 * locals.var_wdld__blk1762_dn0), (locals.var_q_nsubld__blk1764 * locals.var_wdld__blk1762_dn2), (locals.var_q_nsubld__blk1764 * locals.var_wdld__blk1762_dn4), (locals.var_q_nsubld__blk1764 * locals.var_wdld__blk1762_dn5), (locals.var_q_nsubld__blk1764 * locals.var_wdld__blk1762_dn6), (locals.var_q_nsubld__blk1764 * locals.var_wdld__blk1762_dn7), (locals.var_q_nsubld__blk1764 * locals.var_wdld__blk1762_dn8), (locals.var_q_nsubld__blk1764 * locals.var_wdld__blk1762_dn9), (locals.var_q_nsubld__blk1764 * locals.var_wdld__blk1762_dn10), (locals.var_q_nsubld__blk1764 * locals.var_wdld__blk1762_dn11), (locals.var_q_nsubld__blk1764 * locals.var_wdld__blk1762_dn14),)
    } else {
        (locals.var_q_dep_ld__blk1763, locals.var_q_dep_ld__blk1763_dn0, locals.var_q_dep_ld__blk1763_dn2, locals.var_q_dep_ld__blk1763_dn4, locals.var_q_dep_ld__blk1763_dn5, locals.var_q_dep_ld__blk1763_dn6, locals.var_q_dep_ld__blk1763_dn7, locals.var_q_dep_ld__blk1763_dn8, locals.var_q_dep_ld__blk1763_dn9, locals.var_q_dep_ld__blk1763_dn10, locals.var_q_dep_ld__blk1763_dn11, locals.var_q_dep_ld__blk1763_dn14,)
    }
};
        locals.var_q_dep_ld__blk1763 = assign79500_e122114;
        locals.var_q_dep_ld__blk1763_dn0 = assign79500_e122114_d_n0;
        locals.var_q_dep_ld__blk1763_dn2 = assign79500_e122114_d_n2;
        locals.var_q_dep_ld__blk1763_dn4 = assign79500_e122114_d_n4;
        locals.var_q_dep_ld__blk1763_dn5 = assign79500_e122114_d_n5;
        locals.var_q_dep_ld__blk1763_dn6 = assign79500_e122114_d_n6;
        locals.var_q_dep_ld__blk1763_dn7 = assign79500_e122114_d_n7;
        locals.var_q_dep_ld__blk1763_dn8 = assign79500_e122114_d_n8;
        locals.var_q_dep_ld__blk1763_dn9 = assign79500_e122114_d_n9;
        locals.var_q_dep_ld__blk1763_dn10 = assign79500_e122114_d_n10;
        locals.var_q_dep_ld__blk1763_dn11 = assign79500_e122114_d_n11;
        locals.var_q_dep_ld__blk1763_dn14 = assign79500_e122114_d_n14;

        let (assign79510_e122126, assign79510_e122126_d_n0, assign79510_e122126_d_n2, assign79510_e122126_d_n4, assign79510_e122126_d_n5, assign79510_e122126_d_n6, assign79510_e122126_d_n7, assign79510_e122126_d_n8, assign79510_e122126_d_n9, assign79510_e122126_d_n10, assign79510_e122126_d_n11, assign79510_e122126_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1845 != 0.0)) {
        let assign79510_e122120: f64 = (locals.var_q_dep_ld__blk1763 / locals.var_cnst0over_func);
        let assign79510_e122123: f64 = (10.0 * 2.220446049250313e-16);
        let assign79510_e122124: f64 = (assign79510_e122120 + assign79510_e122123);
        (assign79510_e122124, (((locals.var_q_dep_ld__blk1763_dn0 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1763 * locals.var_cnst0over_func_dn0)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1763_dn2 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1763 * locals.var_cnst0over_func_dn2)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1763_dn4 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1763 * locals.var_cnst0over_func_dn4)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1763_dn5 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1763 * locals.var_cnst0over_func_dn5)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1763_dn6 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1763 * locals.var_cnst0over_func_dn6)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1763_dn7 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1763 * locals.var_cnst0over_func_dn7)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1763_dn8 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1763 * locals.var_cnst0over_func_dn8)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1763_dn9 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1763 * locals.var_cnst0over_func_dn9)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1763_dn10 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1763 * locals.var_cnst0over_func_dn10)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1763_dn11 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1763 * locals.var_cnst0over_func_dn11)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1763_dn14 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1763 * locals.var_cnst0over_func_dn14)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)),)
    } else {
        (locals.var_xi0p12, locals.var_xi0p12_dn0, locals.var_xi0p12_dn2, locals.var_xi0p12_dn4, locals.var_xi0p12_dn5, locals.var_xi0p12_dn6, locals.var_xi0p12_dn7, locals.var_xi0p12_dn8, locals.var_xi0p12_dn9, locals.var_xi0p12_dn10, locals.var_xi0p12_dn11, locals.var_xi0p12_dn14,)
    }
};
        locals.var_xi0p12 = assign79510_e122126;
        locals.var_xi0p12_dn0 = assign79510_e122126_d_n0;
        locals.var_xi0p12_dn2 = assign79510_e122126_d_n2;
        locals.var_xi0p12_dn4 = assign79510_e122126_d_n4;
        locals.var_xi0p12_dn5 = assign79510_e122126_d_n5;
        locals.var_xi0p12_dn6 = assign79510_e122126_d_n6;
        locals.var_xi0p12_dn7 = assign79510_e122126_d_n7;
        locals.var_xi0p12_dn8 = assign79510_e122126_d_n8;
        locals.var_xi0p12_dn9 = assign79510_e122126_d_n9;
        locals.var_xi0p12_dn10 = assign79510_e122126_d_n10;
        locals.var_xi0p12_dn11 = assign79510_e122126_d_n11;
        locals.var_xi0p12_dn14 = assign79510_e122126_d_n14;

        let (assign79520_e122134, assign79520_e122134_d_n0, assign79520_e122134_d_n2, assign79520_e122134_d_n4, assign79520_e122134_d_n5, assign79520_e122134_d_n6, assign79520_e122134_d_n7, assign79520_e122134_d_n8, assign79520_e122134_d_n9, assign79520_e122134_d_n10, assign79520_e122134_d_n11, assign79520_e122134_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1845 != 0.0)) {
        let assign79520_e122132: f64 = (locals.var_cnst0over_func * locals.var_xi0p12);
        (assign79520_e122132, ((locals.var_cnst0over_func_dn0 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn0)), ((locals.var_cnst0over_func_dn2 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn2)), ((locals.var_cnst0over_func_dn4 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn4)), ((locals.var_cnst0over_func_dn5 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn5)), ((locals.var_cnst0over_func_dn6 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn6)), ((locals.var_cnst0over_func_dn7 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn7)), ((locals.var_cnst0over_func_dn8 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn8)), ((locals.var_cnst0over_func_dn9 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn9)), ((locals.var_cnst0over_func_dn10 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn10)), ((locals.var_cnst0over_func_dn11 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn11)), ((locals.var_cnst0over_func_dn14 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn14)),)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn8, locals.var_qbuld_dn9, locals.var_qbuld_dn10, locals.var_qbuld_dn11, locals.var_qbuld_dn14,)
    }
};
        locals.var_qbuld = assign79520_e122134;
        locals.var_qbuld_dn0 = assign79520_e122134_d_n0;
        locals.var_qbuld_dn2 = assign79520_e122134_d_n2;
        locals.var_qbuld_dn4 = assign79520_e122134_d_n4;
        locals.var_qbuld_dn5 = assign79520_e122134_d_n5;
        locals.var_qbuld_dn6 = assign79520_e122134_d_n6;
        locals.var_qbuld_dn7 = assign79520_e122134_d_n7;
        locals.var_qbuld_dn8 = assign79520_e122134_d_n8;
        locals.var_qbuld_dn9 = assign79520_e122134_d_n9;
        locals.var_qbuld_dn10 = assign79520_e122134_d_n10;
        locals.var_qbuld_dn11 = assign79520_e122134_d_n11;
        locals.var_qbuld_dn14 = assign79520_e122134_d_n14;

        let (assign79530_e122144, assign79530_e122144_d_n0, assign79530_e122144_d_n2, assign79530_e122144_d_n4, assign79530_e122144_d_n5, assign79530_e122144_d_n6, assign79530_e122144_d_n7, assign79530_e122144_d_n8, assign79530_e122144_d_n9, assign79530_e122144_d_n10, assign79530_e122144_d_n11, assign79530_e122144_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1845 != 0.0)) {
        let assign79530_e122141: f64 = (locals.var_fs02 + locals.var_xi0p12);
        let assign79530_e122142: f64 = (1.0 / assign79530_e122141);
        (assign79530_e122142, (-((locals.var_fs02_dn0 + locals.var_xi0p12_dn0) / (assign79530_e122141 * assign79530_e122141))), (-((locals.var_fs02_dn2 + locals.var_xi0p12_dn2) / (assign79530_e122141 * assign79530_e122141))), (-((locals.var_fs02_dn4 + locals.var_xi0p12_dn4) / (assign79530_e122141 * assign79530_e122141))), (-((locals.var_fs02_dn5 + locals.var_xi0p12_dn5) / (assign79530_e122141 * assign79530_e122141))), (-((locals.var_fs02_dn6 + locals.var_xi0p12_dn6) / (assign79530_e122141 * assign79530_e122141))), (-((locals.var_fs02_dn7 + locals.var_xi0p12_dn7) / (assign79530_e122141 * assign79530_e122141))), (-((locals.var_fs02_dn8 + locals.var_xi0p12_dn8) / (assign79530_e122141 * assign79530_e122141))), (-((locals.var_fs02_dn9 + locals.var_xi0p12_dn9) / (assign79530_e122141 * assign79530_e122141))), (-((locals.var_fs02_dn10 + locals.var_xi0p12_dn10) / (assign79530_e122141 * assign79530_e122141))), (-((locals.var_fs02_dn11 + locals.var_xi0p12_dn11) / (assign79530_e122141 * assign79530_e122141))), (-((locals.var_fs02_dn14 + locals.var_xi0p12_dn14) / (assign79530_e122141 * assign79530_e122141))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign79530_e122144;
        locals.var_t1_dn0 = assign79530_e122144_d_n0;
        locals.var_t1_dn2 = assign79530_e122144_d_n2;
        locals.var_t1_dn4 = assign79530_e122144_d_n4;
        locals.var_t1_dn5 = assign79530_e122144_d_n5;
        locals.var_t1_dn6 = assign79530_e122144_d_n6;
        locals.var_t1_dn7 = assign79530_e122144_d_n7;
        locals.var_t1_dn8 = assign79530_e122144_d_n8;
        locals.var_t1_dn9 = assign79530_e122144_d_n9;
        locals.var_t1_dn10 = assign79530_e122144_d_n10;
        locals.var_t1_dn11 = assign79530_e122144_d_n11;
        locals.var_t1_dn14 = assign79530_e122144_d_n14;

        let (assign79540_e122154, assign79540_e122154_d_n0, assign79540_e122154_d_n2, assign79540_e122154_d_n4, assign79540_e122154_d_n5, assign79540_e122154_d_n6, assign79540_e122154_d_n7, assign79540_e122154_d_n8, assign79540_e122154_d_n9, assign79540_e122154_d_n10, assign79540_e122154_d_n11, assign79540_e122154_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1845 != 0.0)) {
        let assign79540_e122150: f64 = (locals.var_cnst0over_func * locals.var_fs01);
        let assign79540_e122152: f64 = (assign79540_e122150 * locals.var_t1);
        (assign79540_e122152, ((((locals.var_cnst0over_func_dn0 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn0)) * locals.var_t1) + (assign79540_e122150 * locals.var_t1_dn0)), ((((locals.var_cnst0over_func_dn2 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn2)) * locals.var_t1) + (assign79540_e122150 * locals.var_t1_dn2)), ((((locals.var_cnst0over_func_dn4 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn4)) * locals.var_t1) + (assign79540_e122150 * locals.var_t1_dn4)), ((((locals.var_cnst0over_func_dn5 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn5)) * locals.var_t1) + (assign79540_e122150 * locals.var_t1_dn5)), ((((locals.var_cnst0over_func_dn6 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn6)) * locals.var_t1) + (assign79540_e122150 * locals.var_t1_dn6)), ((((locals.var_cnst0over_func_dn7 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn7)) * locals.var_t1) + (assign79540_e122150 * locals.var_t1_dn7)), ((((locals.var_cnst0over_func_dn8 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn8)) * locals.var_t1) + (assign79540_e122150 * locals.var_t1_dn8)), ((((locals.var_cnst0over_func_dn9 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn9)) * locals.var_t1) + (assign79540_e122150 * locals.var_t1_dn9)), ((((locals.var_cnst0over_func_dn10 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn10)) * locals.var_t1) + (assign79540_e122150 * locals.var_t1_dn10)), ((((locals.var_cnst0over_func_dn11 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn11)) * locals.var_t1) + (assign79540_e122150 * locals.var_t1_dn11)), ((((locals.var_cnst0over_func_dn14 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn14)) * locals.var_t1) + (assign79540_e122150 * locals.var_t1_dn14)),)
    } else {
        (locals.var_qiuld, locals.var_qiuld_dn0, locals.var_qiuld_dn2, locals.var_qiuld_dn4, locals.var_qiuld_dn5, locals.var_qiuld_dn6, locals.var_qiuld_dn7, locals.var_qiuld_dn8, locals.var_qiuld_dn9, locals.var_qiuld_dn10, locals.var_qiuld_dn11, locals.var_qiuld_dn14,)
    }
};
        locals.var_qiuld = assign79540_e122154;
        locals.var_qiuld_dn0 = assign79540_e122154_d_n0;
        locals.var_qiuld_dn2 = assign79540_e122154_d_n2;
        locals.var_qiuld_dn4 = assign79540_e122154_d_n4;
        locals.var_qiuld_dn5 = assign79540_e122154_d_n5;
        locals.var_qiuld_dn6 = assign79540_e122154_d_n6;
        locals.var_qiuld_dn7 = assign79540_e122154_d_n7;
        locals.var_qiuld_dn8 = assign79540_e122154_d_n8;
        locals.var_qiuld_dn9 = assign79540_e122154_d_n9;
        locals.var_qiuld_dn10 = assign79540_e122154_d_n10;
        locals.var_qiuld_dn11 = assign79540_e122154_d_n11;
        locals.var_qiuld_dn14 = assign79540_e122154_d_n14;

        let (assign79550_e122162, assign79550_e122162_d_n0, assign79550_e122162_d_n2, assign79550_e122162_d_n4, assign79550_e122162_d_n5, assign79550_e122162_d_n6, assign79550_e122162_d_n7, assign79550_e122162_d_n8, assign79550_e122162_d_n9, assign79550_e122162_d_n10, assign79550_e122162_d_n11, assign79550_e122162_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1845 != 0.0)) {
        let assign79550_e122160: f64 = (locals.var_qbuld + locals.var_qiuld);
        (assign79550_e122160, (locals.var_qbuld_dn0 + locals.var_qiuld_dn0), (locals.var_qbuld_dn2 + locals.var_qiuld_dn2), (locals.var_qbuld_dn4 + locals.var_qiuld_dn4), (locals.var_qbuld_dn5 + locals.var_qiuld_dn5), (locals.var_qbuld_dn6 + locals.var_qiuld_dn6), (locals.var_qbuld_dn7 + locals.var_qiuld_dn7), (locals.var_qbuld_dn8 + locals.var_qiuld_dn8), (locals.var_qbuld_dn9 + locals.var_qiuld_dn9), (locals.var_qbuld_dn10 + locals.var_qiuld_dn10), (locals.var_qbuld_dn11 + locals.var_qiuld_dn11), (locals.var_qbuld_dn14 + locals.var_qiuld_dn14),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn4, locals.var_qsuld_dn5, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn8, locals.var_qsuld_dn9, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn14,)
    }
};
        locals.var_qsuld = assign79550_e122162;
        locals.var_qsuld_dn0 = assign79550_e122162_d_n0;
        locals.var_qsuld_dn2 = assign79550_e122162_d_n2;
        locals.var_qsuld_dn4 = assign79550_e122162_d_n4;
        locals.var_qsuld_dn5 = assign79550_e122162_d_n5;
        locals.var_qsuld_dn6 = assign79550_e122162_d_n6;
        locals.var_qsuld_dn7 = assign79550_e122162_d_n7;
        locals.var_qsuld_dn8 = assign79550_e122162_d_n8;
        locals.var_qsuld_dn9 = assign79550_e122162_d_n9;
        locals.var_qsuld_dn10 = assign79550_e122162_d_n10;
        locals.var_qsuld_dn11 = assign79550_e122162_d_n11;
        locals.var_qsuld_dn14 = assign79550_e122162_d_n14;

        let (assign79560_e122168, assign79560_e122168_d_n0, assign79560_e122168_d_n2, assign79560_e122168_d_n4, assign79560_e122168_d_n5, assign79560_e122168_d_n6, assign79560_e122168_d_n7, assign79560_e122168_d_n8, assign79560_e122168_d_n9, assign79560_e122168_d_n10, assign79560_e122168_d_n11, assign79560_e122168_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign79560_e122166: f64 = (locals.var_qsuld - locals.var_qbuld);
        (assign79560_e122166, (locals.var_qsuld_dn0 - locals.var_qbuld_dn0), (locals.var_qsuld_dn2 - locals.var_qbuld_dn2), (locals.var_qsuld_dn4 - locals.var_qbuld_dn4), (locals.var_qsuld_dn5 - locals.var_qbuld_dn5), (locals.var_qsuld_dn6 - locals.var_qbuld_dn6), (locals.var_qsuld_dn7 - locals.var_qbuld_dn7), (locals.var_qsuld_dn8 - locals.var_qbuld_dn8), (locals.var_qsuld_dn9 - locals.var_qbuld_dn9), (locals.var_qsuld_dn10 - locals.var_qbuld_dn10), (locals.var_qsuld_dn11 - locals.var_qbuld_dn11), (locals.var_qsuld_dn14 - locals.var_qbuld_dn14),)
    } else {
        (locals.var_qiuld, locals.var_qiuld_dn0, locals.var_qiuld_dn2, locals.var_qiuld_dn4, locals.var_qiuld_dn5, locals.var_qiuld_dn6, locals.var_qiuld_dn7, locals.var_qiuld_dn8, locals.var_qiuld_dn9, locals.var_qiuld_dn10, locals.var_qiuld_dn11, locals.var_qiuld_dn14,)
    }
};
        locals.var_qiuld = assign79560_e122168;
        locals.var_qiuld_dn0 = assign79560_e122168_d_n0;
        locals.var_qiuld_dn2 = assign79560_e122168_d_n2;
        locals.var_qiuld_dn4 = assign79560_e122168_d_n4;
        locals.var_qiuld_dn5 = assign79560_e122168_d_n5;
        locals.var_qiuld_dn6 = assign79560_e122168_d_n6;
        locals.var_qiuld_dn7 = assign79560_e122168_d_n7;
        locals.var_qiuld_dn8 = assign79560_e122168_d_n8;
        locals.var_qiuld_dn9 = assign79560_e122168_d_n9;
        locals.var_qiuld_dn10 = assign79560_e122168_d_n10;
        locals.var_qiuld_dn11 = assign79560_e122168_d_n11;
        locals.var_qiuld_dn14 = assign79560_e122168_d_n14;

        let assign79570_e122171: f64 = if locals.var_lover_func < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1860 = assign79570_e122171;

        let (assign79580_e122178, assign79580_e122178_d_n0, assign79580_e122178_d_n2, assign79580_e122178_d_n4, assign79580_e122178_d_n5, assign79580_e122178_d_n6, assign79580_e122178_d_n7, assign79580_e122178_d_n8, assign79580_e122178_d_n9, assign79580_e122178_d_n10, assign79580_e122178_d_n11, assign79580_e122178_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1860 != 0.0)) {
        let assign79580_e122176: f64 = (-locals.var_lover_func);
        (assign79580_e122176, (-locals.var_lover_func_dn0), (-locals.var_lover_func_dn2), (-locals.var_lover_func_dn4), (-locals.var_lover_func_dn5), (-locals.var_lover_func_dn6), (-locals.var_lover_func_dn7), (-locals.var_lover_func_dn8), (-locals.var_lover_func_dn9), (-locals.var_lover_func_dn10), (-locals.var_lover_func_dn11), (-locals.var_lover_func_dn14),)
    } else {
        (locals.var_lover_func, locals.var_lover_func_dn0, locals.var_lover_func_dn2, locals.var_lover_func_dn4, locals.var_lover_func_dn5, locals.var_lover_func_dn6, locals.var_lover_func_dn7, locals.var_lover_func_dn8, locals.var_lover_func_dn9, locals.var_lover_func_dn10, locals.var_lover_func_dn11, locals.var_lover_func_dn14,)
    }
};
        locals.var_lover_func = assign79580_e122178;
        locals.var_lover_func_dn0 = assign79580_e122178_d_n0;
        locals.var_lover_func_dn2 = assign79580_e122178_d_n2;
        locals.var_lover_func_dn4 = assign79580_e122178_d_n4;
        locals.var_lover_func_dn5 = assign79580_e122178_d_n5;
        locals.var_lover_func_dn6 = assign79580_e122178_d_n6;
        locals.var_lover_func_dn7 = assign79580_e122178_d_n7;
        locals.var_lover_func_dn8 = assign79580_e122178_d_n8;
        locals.var_lover_func_dn9 = assign79580_e122178_d_n9;
        locals.var_lover_func_dn10 = assign79580_e122178_d_n10;
        locals.var_lover_func_dn11 = assign79580_e122178_d_n11;
        locals.var_lover_func_dn14 = assign79580_e122178_d_n14;

        let assign79590_e122181: f64 = if p.p55 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1861 = assign79590_e122181;

        let assign79600_e122184: f64 = if p.p50 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1862 = assign79600_e122184;

        let (assign79610_e122195, assign79610_e122195_d_n0, assign79610_e122195_d_n2, assign79610_e122195_d_n4, assign79610_e122195_d_n5, assign79610_e122195_d_n6, assign79610_e122195_d_n7, assign79610_e122195_d_n8, assign79610_e122195_d_n9, assign79610_e122195_d_n10, assign79610_e122195_d_n11, assign79610_e122195_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1860 != 0.0)) && (locals.var_guard1861 != 0.0)) && (locals.var_guard1862 != 0.0)) {
        let assign79610_e122193: f64 = (-locals.var_ps0ld);
        (assign79610_e122193, (-locals.var_ps0ld_dn0), (-locals.var_ps0ld_dn2), (-locals.var_ps0ld_dn4), (-locals.var_ps0ld_dn5), (-locals.var_ps0ld_dn6), (-locals.var_ps0ld_dn7), (-locals.var_ps0ld_dn8), (-locals.var_ps0ld_dn9), (-locals.var_ps0ld_dn10), (-locals.var_ps0ld_dn11), (-locals.var_ps0ld_dn14),)
    } else {
        (locals.var_vx__blk1765, locals.var_vx__blk1765_dn0, locals.var_vx__blk1765_dn2, locals.var_vx__blk1765_dn4, locals.var_vx__blk1765_dn5, locals.var_vx__blk1765_dn6, locals.var_vx__blk1765_dn7, locals.var_vx__blk1765_dn8, locals.var_vx__blk1765_dn9, locals.var_vx__blk1765_dn10, locals.var_vx__blk1765_dn11, locals.var_vx__blk1765_dn14,)
    }
};
        locals.var_vx__blk1765 = assign79610_e122195;
        locals.var_vx__blk1765_dn0 = assign79610_e122195_d_n0;
        locals.var_vx__blk1765_dn2 = assign79610_e122195_d_n2;
        locals.var_vx__blk1765_dn4 = assign79610_e122195_d_n4;
        locals.var_vx__blk1765_dn5 = assign79610_e122195_d_n5;
        locals.var_vx__blk1765_dn6 = assign79610_e122195_d_n6;
        locals.var_vx__blk1765_dn7 = assign79610_e122195_d_n7;
        locals.var_vx__blk1765_dn8 = assign79610_e122195_d_n8;
        locals.var_vx__blk1765_dn9 = assign79610_e122195_d_n9;
        locals.var_vx__blk1765_dn10 = assign79610_e122195_d_n10;
        locals.var_vx__blk1765_dn11 = assign79610_e122195_d_n11;
        locals.var_vx__blk1765_dn14 = assign79610_e122195_d_n14;

        let (assign79620_e122206, assign79620_e122206_d_n0, assign79620_e122206_d_n2, assign79620_e122206_d_n4, assign79620_e122206_d_n5, assign79620_e122206_d_n6, assign79620_e122206_d_n7, assign79620_e122206_d_n8, assign79620_e122206_d_n9, assign79620_e122206_d_n10, assign79620_e122206_d_n11, assign79620_e122206_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1860 != 0.0)) && (locals.var_guard1861 != 0.0)) && (locals.var_guard1862 == 0.0)) {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn11, locals.var_vxbgmt_dn14,)
    } else {
        (locals.var_vx__blk1765, locals.var_vx__blk1765_dn0, locals.var_vx__blk1765_dn2, locals.var_vx__blk1765_dn4, locals.var_vx__blk1765_dn5, locals.var_vx__blk1765_dn6, locals.var_vx__blk1765_dn7, locals.var_vx__blk1765_dn8, locals.var_vx__blk1765_dn9, locals.var_vx__blk1765_dn10, locals.var_vx__blk1765_dn11, locals.var_vx__blk1765_dn14,)
    }
};
        locals.var_vx__blk1765 = assign79620_e122206;
        locals.var_vx__blk1765_dn0 = assign79620_e122206_d_n0;
        locals.var_vx__blk1765_dn2 = assign79620_e122206_d_n2;
        locals.var_vx__blk1765_dn4 = assign79620_e122206_d_n4;
        locals.var_vx__blk1765_dn5 = assign79620_e122206_d_n5;
        locals.var_vx__blk1765_dn6 = assign79620_e122206_d_n6;
        locals.var_vx__blk1765_dn7 = assign79620_e122206_d_n7;
        locals.var_vx__blk1765_dn8 = assign79620_e122206_d_n8;
        locals.var_vx__blk1765_dn9 = assign79620_e122206_d_n9;
        locals.var_vx__blk1765_dn10 = assign79620_e122206_d_n10;
        locals.var_vx__blk1765_dn11 = assign79620_e122206_d_n11;
        locals.var_vx__blk1765_dn14 = assign79620_e122206_d_n14;

        let (assign79630_e122227, assign79630_e122227_d_n0, assign79630_e122227_d_n2, assign79630_e122227_d_n4, assign79630_e122227_d_n5, assign79630_e122227_d_n6, assign79630_e122227_d_n7, assign79630_e122227_d_n8, assign79630_e122227_d_n9, assign79630_e122227_d_n10, assign79630_e122227_d_n11, assign79630_e122227_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1860 != 0.0)) && (locals.var_guard1861 != 0.0)) {
        let assign79630_e122214: f64 = (locals.var_vx__blk1765 + p.p137);
        let assign79630_e122217: f64 = (locals.var_vx__blk1765 + p.p137);
        let assign79630_e122218: f64 = (assign79630_e122214 * assign79630_e122217);
        let assign79630_e122221: f64 = (4.0 * 0.1);
        let assign79630_e122223: f64 = (assign79630_e122221 * 0.1);
        let assign79630_e122224: f64 = (assign79630_e122218 + assign79630_e122223);
        let assign79630_e122225: f64 = (assign79630_e122224).sqrt();
        (assign79630_e122225, (((locals.var_vx__blk1765_dn0 * assign79630_e122217) + (assign79630_e122214 * locals.var_vx__blk1765_dn0)) / (2.0 * assign79630_e122225)), (((locals.var_vx__blk1765_dn2 * assign79630_e122217) + (assign79630_e122214 * locals.var_vx__blk1765_dn2)) / (2.0 * assign79630_e122225)), (((locals.var_vx__blk1765_dn4 * assign79630_e122217) + (assign79630_e122214 * locals.var_vx__blk1765_dn4)) / (2.0 * assign79630_e122225)), (((locals.var_vx__blk1765_dn5 * assign79630_e122217) + (assign79630_e122214 * locals.var_vx__blk1765_dn5)) / (2.0 * assign79630_e122225)), (((locals.var_vx__blk1765_dn6 * assign79630_e122217) + (assign79630_e122214 * locals.var_vx__blk1765_dn6)) / (2.0 * assign79630_e122225)), (((locals.var_vx__blk1765_dn7 * assign79630_e122217) + (assign79630_e122214 * locals.var_vx__blk1765_dn7)) / (2.0 * assign79630_e122225)), (((locals.var_vx__blk1765_dn8 * assign79630_e122217) + (assign79630_e122214 * locals.var_vx__blk1765_dn8)) / (2.0 * assign79630_e122225)), (((locals.var_vx__blk1765_dn9 * assign79630_e122217) + (assign79630_e122214 * locals.var_vx__blk1765_dn9)) / (2.0 * assign79630_e122225)), (((locals.var_vx__blk1765_dn10 * assign79630_e122217) + (assign79630_e122214 * locals.var_vx__blk1765_dn10)) / (2.0 * assign79630_e122225)), (((locals.var_vx__blk1765_dn11 * assign79630_e122217) + (assign79630_e122214 * locals.var_vx__blk1765_dn11)) / (2.0 * assign79630_e122225)), (((locals.var_vx__blk1765_dn14 * assign79630_e122217) + (assign79630_e122214 * locals.var_vx__blk1765_dn14)) / (2.0 * assign79630_e122225)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign79630_e122227;
        locals.var_tmf2_dn0 = assign79630_e122227_d_n0;
        locals.var_tmf2_dn2 = assign79630_e122227_d_n2;
        locals.var_tmf2_dn4 = assign79630_e122227_d_n4;
        locals.var_tmf2_dn5 = assign79630_e122227_d_n5;
        locals.var_tmf2_dn6 = assign79630_e122227_d_n6;
        locals.var_tmf2_dn7 = assign79630_e122227_d_n7;
        locals.var_tmf2_dn8 = assign79630_e122227_d_n8;
        locals.var_tmf2_dn9 = assign79630_e122227_d_n9;
        locals.var_tmf2_dn10 = assign79630_e122227_d_n10;
        locals.var_tmf2_dn11 = assign79630_e122227_d_n11;
        locals.var_tmf2_dn14 = assign79630_e122227_d_n14;

        let (assign79640_e122243, assign79640_e122243_d_n0, assign79640_e122243_d_n2, assign79640_e122243_d_n4, assign79640_e122243_d_n5, assign79640_e122243_d_n6, assign79640_e122243_d_n7, assign79640_e122243_d_n8, assign79640_e122243_d_n9, assign79640_e122243_d_n10, assign79640_e122243_d_n11, assign79640_e122243_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1860 != 0.0)) && (locals.var_guard1861 != 0.0)) {
        let assign79640_e122237: f64 = (locals.var_vx__blk1765 + p.p137);
        let assign79640_e122239: f64 = (assign79640_e122237 / locals.var_tmf2);
        let assign79640_e122240: f64 = (1.0 + assign79640_e122239);
        let assign79640_e122241: f64 = (0.5 * assign79640_e122240);
        (assign79640_e122241, (0.5 * (((locals.var_vx__blk1765_dn0 * locals.var_tmf2) - (assign79640_e122237 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk1765_dn2 * locals.var_tmf2) - (assign79640_e122237 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk1765_dn4 * locals.var_tmf2) - (assign79640_e122237 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk1765_dn5 * locals.var_tmf2) - (assign79640_e122237 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk1765_dn6 * locals.var_tmf2) - (assign79640_e122237 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk1765_dn7 * locals.var_tmf2) - (assign79640_e122237 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk1765_dn8 * locals.var_tmf2) - (assign79640_e122237 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk1765_dn9 * locals.var_tmf2) - (assign79640_e122237 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk1765_dn10 * locals.var_tmf2) - (assign79640_e122237 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk1765_dn11 * locals.var_tmf2) - (assign79640_e122237 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk1765_dn14 * locals.var_tmf2) - (assign79640_e122237 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign79640_e122243;
        locals.var_t9_dn0 = assign79640_e122243_d_n0;
        locals.var_t9_dn2 = assign79640_e122243_d_n2;
        locals.var_t9_dn4 = assign79640_e122243_d_n4;
        locals.var_t9_dn5 = assign79640_e122243_d_n5;
        locals.var_t9_dn6 = assign79640_e122243_d_n6;
        locals.var_t9_dn7 = assign79640_e122243_d_n7;
        locals.var_t9_dn8 = assign79640_e122243_d_n8;
        locals.var_t9_dn9 = assign79640_e122243_d_n9;
        locals.var_t9_dn10 = assign79640_e122243_d_n10;
        locals.var_t9_dn11 = assign79640_e122243_d_n11;
        locals.var_t9_dn14 = assign79640_e122243_d_n14;

        let (assign79650_e122257, assign79650_e122257_d_n0, assign79650_e122257_d_n2, assign79650_e122257_d_n4, assign79650_e122257_d_n5, assign79650_e122257_d_n6, assign79650_e122257_d_n7, assign79650_e122257_d_n8, assign79650_e122257_d_n9, assign79650_e122257_d_n10, assign79650_e122257_d_n11, assign79650_e122257_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1860 != 0.0)) && (locals.var_guard1861 != 0.0)) {
        let assign79650_e122252: f64 = (locals.var_vx__blk1765 + p.p137);
        let assign79650_e122254: f64 = (assign79650_e122252 + locals.var_tmf2);
        let assign79650_e122255: f64 = (0.5 * assign79650_e122254);
        (assign79650_e122255, (0.5 * (locals.var_vx__blk1765_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_vx__blk1765_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_vx__blk1765_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_vx__blk1765_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_vx__blk1765_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_vx__blk1765_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_vx__blk1765_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_vx__blk1765_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_vx__blk1765_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_vx__blk1765_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_vx__blk1765_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign79650_e122257;
        locals.var_t2_dn0 = assign79650_e122257_d_n0;
        locals.var_t2_dn2 = assign79650_e122257_d_n2;
        locals.var_t2_dn4 = assign79650_e122257_d_n4;
        locals.var_t2_dn5 = assign79650_e122257_d_n5;
        locals.var_t2_dn6 = assign79650_e122257_d_n6;
        locals.var_t2_dn7 = assign79650_e122257_d_n7;
        locals.var_t2_dn8 = assign79650_e122257_d_n8;
        locals.var_t2_dn9 = assign79650_e122257_d_n9;
        locals.var_t2_dn10 = assign79650_e122257_d_n10;
        locals.var_t2_dn11 = assign79650_e122257_d_n11;
        locals.var_t2_dn14 = assign79650_e122257_d_n14;

        let assign79660_e122260: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1863 = assign79660_e122260;

        let (assign79670_e122270, assign79670_e122270_d_n0, assign79670_e122270_d_n2, assign79670_e122270_d_n4, assign79670_e122270_d_n5, assign79670_e122270_d_n6, assign79670_e122270_d_n7, assign79670_e122270_d_n8, assign79670_e122270_d_n9, assign79670_e122270_d_n10, assign79670_e122270_d_n11, assign79670_e122270_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1860 != 0.0)) && (locals.var_guard1861 != 0.0)) && (locals.var_guard1863 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign79670_e122270;
        locals.var_t2_dn0 = assign79670_e122270_d_n0;
        locals.var_t2_dn2 = assign79670_e122270_d_n2;
        locals.var_t2_dn4 = assign79670_e122270_d_n4;
        locals.var_t2_dn5 = assign79670_e122270_d_n5;
        locals.var_t2_dn6 = assign79670_e122270_d_n6;
        locals.var_t2_dn7 = assign79670_e122270_d_n7;
        locals.var_t2_dn8 = assign79670_e122270_d_n8;
        locals.var_t2_dn9 = assign79670_e122270_d_n9;
        locals.var_t2_dn10 = assign79670_e122270_d_n10;
        locals.var_t2_dn11 = assign79670_e122270_d_n11;
        locals.var_t2_dn14 = assign79670_e122270_d_n14;

        let (assign79680_e122280, assign79680_e122280_d_n0, assign79680_e122280_d_n2, assign79680_e122280_d_n4, assign79680_e122280_d_n5, assign79680_e122280_d_n6, assign79680_e122280_d_n7, assign79680_e122280_d_n8, assign79680_e122280_d_n9, assign79680_e122280_d_n10, assign79680_e122280_d_n11, assign79680_e122280_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1860 != 0.0)) && (locals.var_guard1861 != 0.0)) && (locals.var_guard1863 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign79680_e122280;
        locals.var_t9_dn0 = assign79680_e122280_d_n0;
        locals.var_t9_dn2 = assign79680_e122280_d_n2;
        locals.var_t9_dn4 = assign79680_e122280_d_n4;
        locals.var_t9_dn5 = assign79680_e122280_d_n5;
        locals.var_t9_dn6 = assign79680_e122280_d_n6;
        locals.var_t9_dn7 = assign79680_e122280_d_n7;
        locals.var_t9_dn8 = assign79680_e122280_d_n8;
        locals.var_t9_dn9 = assign79680_e122280_d_n9;
        locals.var_t9_dn10 = assign79680_e122280_d_n10;
        locals.var_t9_dn11 = assign79680_e122280_d_n11;
        locals.var_t9_dn14 = assign79680_e122280_d_n14;

        let (assign79690_e122293, assign79690_e122293_d_n0, assign79690_e122293_d_n2, assign79690_e122293_d_n4, assign79690_e122293_d_n5, assign79690_e122293_d_n6, assign79690_e122293_d_n7, assign79690_e122293_d_n8, assign79690_e122293_d_n9, assign79690_e122293_d_n10, assign79690_e122293_d_n11, assign79690_e122293_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1860 != 0.0)) && (locals.var_guard1861 != 0.0)) {
        let assign79690_e122288: f64 = (locals.var_kjunc * locals.var_t2);
        let assign79690_e122289: f64 = (assign79690_e122288).sqrt();
        let assign79690_e122291: f64 = (assign79690_e122289 * p.p432);
        (assign79690_e122291, ((((locals.var_kjunc_dn0 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn0)) / (2.0 * assign79690_e122289)) * p.p432), ((((locals.var_kjunc_dn2 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn2)) / (2.0 * assign79690_e122289)) * p.p432), ((((locals.var_kjunc_dn4 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn4)) / (2.0 * assign79690_e122289)) * p.p432), ((((locals.var_kjunc_dn5 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn5)) / (2.0 * assign79690_e122289)) * p.p432), ((((locals.var_kjunc_dn6 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn6)) / (2.0 * assign79690_e122289)) * p.p432), ((((locals.var_kjunc_dn7 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn7)) / (2.0 * assign79690_e122289)) * p.p432), ((((locals.var_kjunc_dn8 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn8)) / (2.0 * assign79690_e122289)) * p.p432), ((((locals.var_kjunc_dn9 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn9)) / (2.0 * assign79690_e122289)) * p.p432), ((((locals.var_kjunc_dn10 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn10)) / (2.0 * assign79690_e122289)) * p.p432), ((((locals.var_kjunc_dn11 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn11)) / (2.0 * assign79690_e122289)) * p.p432), ((((locals.var_kjunc_dn14 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn14)) / (2.0 * assign79690_e122289)) * p.p432),)
    } else {
        (locals.var_wjunc0, locals.var_wjunc0_dn0, locals.var_wjunc0_dn2, locals.var_wjunc0_dn4, locals.var_wjunc0_dn5, locals.var_wjunc0_dn6, locals.var_wjunc0_dn7, locals.var_wjunc0_dn8, locals.var_wjunc0_dn9, locals.var_wjunc0_dn10, locals.var_wjunc0_dn11, locals.var_wjunc0_dn14,)
    }
};
        locals.var_wjunc0 = assign79690_e122293;
        locals.var_wjunc0_dn0 = assign79690_e122293_d_n0;
        locals.var_wjunc0_dn2 = assign79690_e122293_d_n2;
        locals.var_wjunc0_dn4 = assign79690_e122293_d_n4;
        locals.var_wjunc0_dn5 = assign79690_e122293_d_n5;
        locals.var_wjunc0_dn6 = assign79690_e122293_d_n6;
        locals.var_wjunc0_dn7 = assign79690_e122293_d_n7;
        locals.var_wjunc0_dn8 = assign79690_e122293_d_n8;
        locals.var_wjunc0_dn9 = assign79690_e122293_d_n9;
        locals.var_wjunc0_dn10 = assign79690_e122293_d_n10;
        locals.var_wjunc0_dn11 = assign79690_e122293_d_n11;
        locals.var_wjunc0_dn14 = assign79690_e122293_d_n14;

        let (assign79700_e122307, assign79700_e122307_d_n0, assign79700_e122307_d_n2, assign79700_e122307_d_n4, assign79700_e122307_d_n5, assign79700_e122307_d_n6, assign79700_e122307_d_n7, assign79700_e122307_d_n8, assign79700_e122307_d_n9, assign79700_e122307_d_n10, assign79700_e122307_d_n11, assign79700_e122307_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1860 != 0.0)) && (locals.var_guard1861 != 0.0)) {
        let assign79700_e122301: f64 = (locals.var_lover_func - locals.var_wjunc0);
        let assign79700_e122304: f64 = (0.1 * locals.var_lover_func);
        let assign79700_e122305: f64 = (assign79700_e122301 - assign79700_e122304);
        (assign79700_e122305, ((locals.var_lover_func_dn0 - locals.var_wjunc0_dn0) - (0.1 * locals.var_lover_func_dn0)), ((locals.var_lover_func_dn2 - locals.var_wjunc0_dn2) - (0.1 * locals.var_lover_func_dn2)), ((locals.var_lover_func_dn4 - locals.var_wjunc0_dn4) - (0.1 * locals.var_lover_func_dn4)), ((locals.var_lover_func_dn5 - locals.var_wjunc0_dn5) - (0.1 * locals.var_lover_func_dn5)), ((locals.var_lover_func_dn6 - locals.var_wjunc0_dn6) - (0.1 * locals.var_lover_func_dn6)), ((locals.var_lover_func_dn7 - locals.var_wjunc0_dn7) - (0.1 * locals.var_lover_func_dn7)), ((locals.var_lover_func_dn8 - locals.var_wjunc0_dn8) - (0.1 * locals.var_lover_func_dn8)), ((locals.var_lover_func_dn9 - locals.var_wjunc0_dn9) - (0.1 * locals.var_lover_func_dn9)), ((locals.var_lover_func_dn10 - locals.var_wjunc0_dn10) - (0.1 * locals.var_lover_func_dn10)), ((locals.var_lover_func_dn11 - locals.var_wjunc0_dn11) - (0.1 * locals.var_lover_func_dn11)), ((locals.var_lover_func_dn14 - locals.var_wjunc0_dn14) - (0.1 * locals.var_lover_func_dn14)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign79700_e122307;
        locals.var_tmf1_dn0 = assign79700_e122307_d_n0;
        locals.var_tmf1_dn2 = assign79700_e122307_d_n2;
        locals.var_tmf1_dn4 = assign79700_e122307_d_n4;
        locals.var_tmf1_dn5 = assign79700_e122307_d_n5;
        locals.var_tmf1_dn6 = assign79700_e122307_d_n6;
        locals.var_tmf1_dn7 = assign79700_e122307_d_n7;
        locals.var_tmf1_dn8 = assign79700_e122307_d_n8;
        locals.var_tmf1_dn9 = assign79700_e122307_d_n9;
        locals.var_tmf1_dn10 = assign79700_e122307_d_n10;
        locals.var_tmf1_dn11 = assign79700_e122307_d_n11;
        locals.var_tmf1_dn14 = assign79700_e122307_d_n14;

        let (assign79710_e122321, assign79710_e122321_d_n0, assign79710_e122321_d_n2, assign79710_e122321_d_n4, assign79710_e122321_d_n5, assign79710_e122321_d_n6, assign79710_e122321_d_n7, assign79710_e122321_d_n8, assign79710_e122321_d_n9, assign79710_e122321_d_n10, assign79710_e122321_d_n11, assign79710_e122321_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1860 != 0.0)) && (locals.var_guard1861 != 0.0)) {
        let assign79710_e122315: f64 = (4.0 * locals.var_lover_func);
        let assign79710_e122318: f64 = (0.1 * locals.var_lover_func);
        let assign79710_e122319: f64 = (assign79710_e122315 * assign79710_e122318);
        (assign79710_e122319, (((4.0 * locals.var_lover_func_dn0) * assign79710_e122318) + (assign79710_e122315 * (0.1 * locals.var_lover_func_dn0))), (((4.0 * locals.var_lover_func_dn2) * assign79710_e122318) + (assign79710_e122315 * (0.1 * locals.var_lover_func_dn2))), (((4.0 * locals.var_lover_func_dn4) * assign79710_e122318) + (assign79710_e122315 * (0.1 * locals.var_lover_func_dn4))), (((4.0 * locals.var_lover_func_dn5) * assign79710_e122318) + (assign79710_e122315 * (0.1 * locals.var_lover_func_dn5))), (((4.0 * locals.var_lover_func_dn6) * assign79710_e122318) + (assign79710_e122315 * (0.1 * locals.var_lover_func_dn6))), (((4.0 * locals.var_lover_func_dn7) * assign79710_e122318) + (assign79710_e122315 * (0.1 * locals.var_lover_func_dn7))), (((4.0 * locals.var_lover_func_dn8) * assign79710_e122318) + (assign79710_e122315 * (0.1 * locals.var_lover_func_dn8))), (((4.0 * locals.var_lover_func_dn9) * assign79710_e122318) + (assign79710_e122315 * (0.1 * locals.var_lover_func_dn9))), (((4.0 * locals.var_lover_func_dn10) * assign79710_e122318) + (assign79710_e122315 * (0.1 * locals.var_lover_func_dn10))), (((4.0 * locals.var_lover_func_dn11) * assign79710_e122318) + (assign79710_e122315 * (0.1 * locals.var_lover_func_dn11))), (((4.0 * locals.var_lover_func_dn14) * assign79710_e122318) + (assign79710_e122315 * (0.1 * locals.var_lover_func_dn14))),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign79710_e122321;
        locals.var_tmf2_dn0 = assign79710_e122321_d_n0;
        locals.var_tmf2_dn2 = assign79710_e122321_d_n2;
        locals.var_tmf2_dn4 = assign79710_e122321_d_n4;
        locals.var_tmf2_dn5 = assign79710_e122321_d_n5;
        locals.var_tmf2_dn6 = assign79710_e122321_d_n6;
        locals.var_tmf2_dn7 = assign79710_e122321_d_n7;
        locals.var_tmf2_dn8 = assign79710_e122321_d_n8;
        locals.var_tmf2_dn9 = assign79710_e122321_d_n9;
        locals.var_tmf2_dn10 = assign79710_e122321_d_n10;
        locals.var_tmf2_dn11 = assign79710_e122321_d_n11;
        locals.var_tmf2_dn14 = assign79710_e122321_d_n14;

        let (assign79720_e122335, assign79720_e122335_d_n0, assign79720_e122335_d_n2, assign79720_e122335_d_n4, assign79720_e122335_d_n5, assign79720_e122335_d_n6, assign79720_e122335_d_n7, assign79720_e122335_d_n8, assign79720_e122335_d_n9, assign79720_e122335_d_n10, assign79720_e122335_d_n11, assign79720_e122335_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1860 != 0.0)) && (locals.var_guard1861 != 0.0)) {
        let (assign79720_e122333, assign79720_e122333_d_n0, assign79720_e122333_d_n2, assign79720_e122333_d_n4, assign79720_e122333_d_n5, assign79720_e122333_d_n6, assign79720_e122333_d_n7, assign79720_e122333_d_n8, assign79720_e122333_d_n9, assign79720_e122333_d_n10, assign79720_e122333_d_n11, assign79720_e122333_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign79720_e122332: f64 = (-locals.var_tmf2);
                (assign79720_e122332, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign79720_e122333, assign79720_e122333_d_n0, assign79720_e122333_d_n2, assign79720_e122333_d_n4, assign79720_e122333_d_n5, assign79720_e122333_d_n6, assign79720_e122333_d_n7, assign79720_e122333_d_n8, assign79720_e122333_d_n9, assign79720_e122333_d_n10, assign79720_e122333_d_n11, assign79720_e122333_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign79720_e122335;
        locals.var_tmf2_dn0 = assign79720_e122335_d_n0;
        locals.var_tmf2_dn2 = assign79720_e122335_d_n2;
        locals.var_tmf2_dn4 = assign79720_e122335_d_n4;
        locals.var_tmf2_dn5 = assign79720_e122335_d_n5;
        locals.var_tmf2_dn6 = assign79720_e122335_d_n6;
        locals.var_tmf2_dn7 = assign79720_e122335_d_n7;
        locals.var_tmf2_dn8 = assign79720_e122335_d_n8;
        locals.var_tmf2_dn9 = assign79720_e122335_d_n9;
        locals.var_tmf2_dn10 = assign79720_e122335_d_n10;
        locals.var_tmf2_dn11 = assign79720_e122335_d_n11;
        locals.var_tmf2_dn14 = assign79720_e122335_d_n14;

    }

    pub(super) fn stamp_transient_block_289(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign79730_e122348, assign79730_e122348_d_n0, assign79730_e122348_d_n2, assign79730_e122348_d_n4, assign79730_e122348_d_n5, assign79730_e122348_d_n6, assign79730_e122348_d_n7, assign79730_e122348_d_n8, assign79730_e122348_d_n9, assign79730_e122348_d_n10, assign79730_e122348_d_n11, assign79730_e122348_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1860 != 0.0)) && (locals.var_guard1861 != 0.0)) {
        let assign79730_e122343: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign79730_e122345: f64 = (assign79730_e122343 + locals.var_tmf2);
        let assign79730_e122346: f64 = (assign79730_e122345).sqrt();
        (assign79730_e122346, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign79730_e122346)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign79730_e122346)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign79730_e122346)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign79730_e122346)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign79730_e122346)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign79730_e122346)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign79730_e122346)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign79730_e122346)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign79730_e122346)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign79730_e122346)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign79730_e122346)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign79730_e122348;
        locals.var_tmf2_dn0 = assign79730_e122348_d_n0;
        locals.var_tmf2_dn2 = assign79730_e122348_d_n2;
        locals.var_tmf2_dn4 = assign79730_e122348_d_n4;
        locals.var_tmf2_dn5 = assign79730_e122348_d_n5;
        locals.var_tmf2_dn6 = assign79730_e122348_d_n6;
        locals.var_tmf2_dn7 = assign79730_e122348_d_n7;
        locals.var_tmf2_dn8 = assign79730_e122348_d_n8;
        locals.var_tmf2_dn9 = assign79730_e122348_d_n9;
        locals.var_tmf2_dn10 = assign79730_e122348_d_n10;
        locals.var_tmf2_dn11 = assign79730_e122348_d_n11;
        locals.var_tmf2_dn14 = assign79730_e122348_d_n14;

        let (assign79740_e122362, assign79740_e122362_d_n0, assign79740_e122362_d_n2, assign79740_e122362_d_n4, assign79740_e122362_d_n5, assign79740_e122362_d_n6, assign79740_e122362_d_n7, assign79740_e122362_d_n8, assign79740_e122362_d_n9, assign79740_e122362_d_n10, assign79740_e122362_d_n11, assign79740_e122362_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1860 != 0.0)) && (locals.var_guard1861 != 0.0)) {
        let assign79740_e122358: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign79740_e122359: f64 = (1.0 + assign79740_e122358);
        let assign79740_e122360: f64 = (0.5 * assign79740_e122359);
        (assign79740_e122360, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign79740_e122362;
        locals.var_t0_dn0 = assign79740_e122362_d_n0;
        locals.var_t0_dn2 = assign79740_e122362_d_n2;
        locals.var_t0_dn4 = assign79740_e122362_d_n4;
        locals.var_t0_dn5 = assign79740_e122362_d_n5;
        locals.var_t0_dn6 = assign79740_e122362_d_n6;
        locals.var_t0_dn7 = assign79740_e122362_d_n7;
        locals.var_t0_dn8 = assign79740_e122362_d_n8;
        locals.var_t0_dn9 = assign79740_e122362_d_n9;
        locals.var_t0_dn10 = assign79740_e122362_d_n10;
        locals.var_t0_dn11 = assign79740_e122362_d_n11;
        locals.var_t0_dn14 = assign79740_e122362_d_n14;

        let (assign79750_e122376, assign79750_e122376_d_n0, assign79750_e122376_d_n2, assign79750_e122376_d_n4, assign79750_e122376_d_n5, assign79750_e122376_d_n6, assign79750_e122376_d_n7, assign79750_e122376_d_n8, assign79750_e122376_d_n9, assign79750_e122376_d_n10, assign79750_e122376_d_n11, assign79750_e122376_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1860 != 0.0)) && (locals.var_guard1861 != 0.0)) {
        let assign79750_e122372: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign79750_e122373: f64 = (0.5 * assign79750_e122372);
        let assign79750_e122374: f64 = (locals.var_lover_func - assign79750_e122373);
        (assign79750_e122374, (locals.var_lover_func_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_lover_func_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_lover_func_dn4 - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_lover_func_dn5 - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_lover_func_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_lover_func_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_lover_func_dn8 - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_lover_func_dn9 - (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (locals.var_lover_func_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_lover_func_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_lover_func_dn14 - (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_wjuncld, locals.var_wjuncld_dn0, locals.var_wjuncld_dn2, locals.var_wjuncld_dn4, locals.var_wjuncld_dn5, locals.var_wjuncld_dn6, locals.var_wjuncld_dn7, locals.var_wjuncld_dn8, locals.var_wjuncld_dn9, locals.var_wjuncld_dn10, locals.var_wjuncld_dn11, locals.var_wjuncld_dn14,)
    }
};
        locals.var_wjuncld = assign79750_e122376;
        locals.var_wjuncld_dn0 = assign79750_e122376_d_n0;
        locals.var_wjuncld_dn2 = assign79750_e122376_d_n2;
        locals.var_wjuncld_dn4 = assign79750_e122376_d_n4;
        locals.var_wjuncld_dn5 = assign79750_e122376_d_n5;
        locals.var_wjuncld_dn6 = assign79750_e122376_d_n6;
        locals.var_wjuncld_dn7 = assign79750_e122376_d_n7;
        locals.var_wjuncld_dn8 = assign79750_e122376_d_n8;
        locals.var_wjuncld_dn9 = assign79750_e122376_d_n9;
        locals.var_wjuncld_dn10 = assign79750_e122376_d_n10;
        locals.var_wjuncld_dn11 = assign79750_e122376_d_n11;
        locals.var_wjuncld_dn14 = assign79750_e122376_d_n14;

        let (assign79760_e122386, assign79760_e122386_d_n0, assign79760_e122386_d_n2, assign79760_e122386_d_n4, assign79760_e122386_d_n5, assign79760_e122386_d_n6, assign79760_e122386_d_n7, assign79760_e122386_d_n8, assign79760_e122386_d_n9, assign79760_e122386_d_n10, assign79760_e122386_d_n11, assign79760_e122386_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1860 != 0.0)) && (locals.var_guard1861 != 0.0)) {
        let assign79760_e122384: f64 = (locals.var_lover_func - locals.var_wjuncld);
        (assign79760_e122384, (locals.var_lover_func_dn0 - locals.var_wjuncld_dn0), (locals.var_lover_func_dn2 - locals.var_wjuncld_dn2), (locals.var_lover_func_dn4 - locals.var_wjuncld_dn4), (locals.var_lover_func_dn5 - locals.var_wjuncld_dn5), (locals.var_lover_func_dn6 - locals.var_wjuncld_dn6), (locals.var_lover_func_dn7 - locals.var_wjuncld_dn7), (locals.var_lover_func_dn8 - locals.var_wjuncld_dn8), (locals.var_lover_func_dn9 - locals.var_wjuncld_dn9), (locals.var_lover_func_dn10 - locals.var_wjuncld_dn10), (locals.var_lover_func_dn11 - locals.var_wjuncld_dn11), (locals.var_lover_func_dn14 - locals.var_wjuncld_dn14),)
    } else {
        (locals.var_lover_func, locals.var_lover_func_dn0, locals.var_lover_func_dn2, locals.var_lover_func_dn4, locals.var_lover_func_dn5, locals.var_lover_func_dn6, locals.var_lover_func_dn7, locals.var_lover_func_dn8, locals.var_lover_func_dn9, locals.var_lover_func_dn10, locals.var_lover_func_dn11, locals.var_lover_func_dn14,)
    }
};
        locals.var_lover_func = assign79760_e122386;
        locals.var_lover_func_dn0 = assign79760_e122386_d_n0;
        locals.var_lover_func_dn2 = assign79760_e122386_d_n2;
        locals.var_lover_func_dn4 = assign79760_e122386_d_n4;
        locals.var_lover_func_dn5 = assign79760_e122386_d_n5;
        locals.var_lover_func_dn6 = assign79760_e122386_d_n6;
        locals.var_lover_func_dn7 = assign79760_e122386_d_n7;
        locals.var_lover_func_dn8 = assign79760_e122386_d_n8;
        locals.var_lover_func_dn9 = assign79760_e122386_d_n9;
        locals.var_lover_func_dn10 = assign79760_e122386_d_n10;
        locals.var_lover_func_dn11 = assign79760_e122386_d_n11;
        locals.var_lover_func_dn14 = assign79760_e122386_d_n14;

        let assign79770_e122389: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1864 = assign79770_e122389;

        let assign79780_e122392: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1865 = assign79780_e122392;

        let assign79790_e122395: f64 = if 2.0 == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard1866 = assign79790_e122395;

        let assign79800_e122398: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1867 = assign79800_e122398;

        let assign79810_e122401: f64 = if p.p55 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1868 = assign79810_e122401;

        let (assign79820_e122411, assign79820_e122411_d_n0, assign79820_e122411_d_n2, assign79820_e122411_d_n4, assign79820_e122411_d_n5, assign79820_e122411_d_n6, assign79820_e122411_d_n7, assign79820_e122411_d_n8, assign79820_e122411_d_n9, assign79820_e122411_d_n10, assign79820_e122411_d_n11, assign79820_e122411_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1864 != 0.0)) && (locals.var_guard1868 != 0.0)) {
        let assign79820_e122409: f64 = (locals.var_weffcv_nf * locals.var_lover_func);
        (assign79820_e122409, (locals.var_weffcv_nf * locals.var_lover_func_dn0), (locals.var_weffcv_nf * locals.var_lover_func_dn2), (locals.var_weffcv_nf * locals.var_lover_func_dn4), (locals.var_weffcv_nf * locals.var_lover_func_dn5), (locals.var_weffcv_nf * locals.var_lover_func_dn6), (locals.var_weffcv_nf * locals.var_lover_func_dn7), (locals.var_weffcv_nf * locals.var_lover_func_dn8), (locals.var_weffcv_nf * locals.var_lover_func_dn9), (locals.var_weffcv_nf * locals.var_lover_func_dn10), (locals.var_weffcv_nf * locals.var_lover_func_dn11), (locals.var_weffcv_nf * locals.var_lover_func_dn14),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign79820_e122411;
        locals.var_t4_dn0 = assign79820_e122411_d_n0;
        locals.var_t4_dn2 = assign79820_e122411_d_n2;
        locals.var_t4_dn4 = assign79820_e122411_d_n4;
        locals.var_t4_dn5 = assign79820_e122411_d_n5;
        locals.var_t4_dn6 = assign79820_e122411_d_n6;
        locals.var_t4_dn7 = assign79820_e122411_d_n7;
        locals.var_t4_dn8 = assign79820_e122411_d_n8;
        locals.var_t4_dn9 = assign79820_e122411_d_n9;
        locals.var_t4_dn10 = assign79820_e122411_d_n10;
        locals.var_t4_dn11 = assign79820_e122411_d_n11;
        locals.var_t4_dn14 = assign79820_e122411_d_n14;

        let (assign79830_e122426, assign79830_e122426_d_n0, assign79830_e122426_d_n2, assign79830_e122426_d_n4, assign79830_e122426_d_n5, assign79830_e122426_d_n6, assign79830_e122426_d_n7, assign79830_e122426_d_n8, assign79830_e122426_d_n9, assign79830_e122426_d_n10, assign79830_e122426_d_n11, assign79830_e122426_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1864 != 0.0)) && (locals.var_guard1868 == 0.0)) {
        let assign79830_e122420: f64 = (locals.var_weffcv_nf * locals.var_lover_func);
        let assign79830_e122423: f64 = (1.0 - locals.var_uc_cvdsover);
        let assign79830_e122424: f64 = (assign79830_e122420 * assign79830_e122423);
        (assign79830_e122424, ((locals.var_weffcv_nf * locals.var_lover_func_dn0) * assign79830_e122423), ((locals.var_weffcv_nf * locals.var_lover_func_dn2) * assign79830_e122423), ((locals.var_weffcv_nf * locals.var_lover_func_dn4) * assign79830_e122423), ((locals.var_weffcv_nf * locals.var_lover_func_dn5) * assign79830_e122423), ((locals.var_weffcv_nf * locals.var_lover_func_dn6) * assign79830_e122423), ((locals.var_weffcv_nf * locals.var_lover_func_dn7) * assign79830_e122423), ((locals.var_weffcv_nf * locals.var_lover_func_dn8) * assign79830_e122423), ((locals.var_weffcv_nf * locals.var_lover_func_dn9) * assign79830_e122423), ((locals.var_weffcv_nf * locals.var_lover_func_dn10) * assign79830_e122423), ((locals.var_weffcv_nf * locals.var_lover_func_dn11) * assign79830_e122423), ((locals.var_weffcv_nf * locals.var_lover_func_dn14) * assign79830_e122423),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign79830_e122426;
        locals.var_t4_dn0 = assign79830_e122426_d_n0;
        locals.var_t4_dn2 = assign79830_e122426_d_n2;
        locals.var_t4_dn4 = assign79830_e122426_d_n4;
        locals.var_t4_dn5 = assign79830_e122426_d_n5;
        locals.var_t4_dn6 = assign79830_e122426_d_n6;
        locals.var_t4_dn7 = assign79830_e122426_d_n7;
        locals.var_t4_dn8 = assign79830_e122426_d_n8;
        locals.var_t4_dn9 = assign79830_e122426_d_n9;
        locals.var_t4_dn10 = assign79830_e122426_d_n10;
        locals.var_t4_dn11 = assign79830_e122426_d_n11;
        locals.var_t4_dn14 = assign79830_e122426_d_n14;

        let (assign79840_e122434, assign79840_e122434_d_n0, assign79840_e122434_d_n2, assign79840_e122434_d_n4, assign79840_e122434_d_n5, assign79840_e122434_d_n6, assign79840_e122434_d_n7, assign79840_e122434_d_n8, assign79840_e122434_d_n9, assign79840_e122434_d_n10, assign79840_e122434_d_n11, assign79840_e122434_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1864 != 0.0)) {
        let assign79840_e122432: f64 = (locals.var_t4 * locals.var_qsuld);
        (assign79840_e122432, ((locals.var_t4_dn0 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn0)), ((locals.var_t4_dn2 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn2)), ((locals.var_t4_dn4 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn4)), ((locals.var_t4_dn5 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn5)), ((locals.var_t4_dn6 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn6)), ((locals.var_t4_dn7 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn7)), ((locals.var_t4_dn8 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn8)), ((locals.var_t4_dn9 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn9)), ((locals.var_t4_dn10 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn10)), ((locals.var_t4_dn11 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn11)), ((locals.var_t4_dn14 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn14)),)
    } else {
        (locals.var_qovs, locals.var_qovs_dn0, locals.var_qovs_dn2, locals.var_qovs_dn4, locals.var_qovs_dn5, locals.var_qovs_dn6, locals.var_qovs_dn7, locals.var_qovs_dn8, locals.var_qovs_dn9, locals.var_qovs_dn10, locals.var_qovs_dn11, locals.var_qovs_dn14,)
    }
};
        locals.var_qovs = assign79840_e122434;
        locals.var_qovs_dn0 = assign79840_e122434_d_n0;
        locals.var_qovs_dn2 = assign79840_e122434_d_n2;
        locals.var_qovs_dn4 = assign79840_e122434_d_n4;
        locals.var_qovs_dn5 = assign79840_e122434_d_n5;
        locals.var_qovs_dn6 = assign79840_e122434_d_n6;
        locals.var_qovs_dn7 = assign79840_e122434_d_n7;
        locals.var_qovs_dn8 = assign79840_e122434_d_n8;
        locals.var_qovs_dn9 = assign79840_e122434_d_n9;
        locals.var_qovs_dn10 = assign79840_e122434_d_n10;
        locals.var_qovs_dn11 = assign79840_e122434_d_n11;
        locals.var_qovs_dn14 = assign79840_e122434_d_n14;

        let (assign79850_e122442, assign79850_e122442_d_n0, assign79850_e122442_d_n2, assign79850_e122442_d_n4, assign79850_e122442_d_n5, assign79850_e122442_d_n6, assign79850_e122442_d_n7, assign79850_e122442_d_n8, assign79850_e122442_d_n9, assign79850_e122442_d_n10, assign79850_e122442_d_n11, assign79850_e122442_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1864 != 0.0)) {
        let assign79850_e122440: f64 = (locals.var_t4 * locals.var_qbuld);
        (assign79850_e122440, ((locals.var_t4_dn0 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn0)), ((locals.var_t4_dn2 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn2)), ((locals.var_t4_dn4 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn4)), ((locals.var_t4_dn5 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn5)), ((locals.var_t4_dn6 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn6)), ((locals.var_t4_dn7 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn7)), ((locals.var_t4_dn8 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn8)), ((locals.var_t4_dn9 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn9)), ((locals.var_t4_dn10 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn10)), ((locals.var_t4_dn11 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn11)), ((locals.var_t4_dn14 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn14)),)
    } else {
        (locals.var_qbsld, locals.var_qbsld_dn0, locals.var_qbsld_dn2, locals.var_qbsld_dn4, locals.var_qbsld_dn5, locals.var_qbsld_dn6, locals.var_qbsld_dn7, locals.var_qbsld_dn8, locals.var_qbsld_dn9, locals.var_qbsld_dn10, locals.var_qbsld_dn11, locals.var_qbsld_dn14,)
    }
};
        locals.var_qbsld = assign79850_e122442;
        locals.var_qbsld_dn0 = assign79850_e122442_d_n0;
        locals.var_qbsld_dn2 = assign79850_e122442_d_n2;
        locals.var_qbsld_dn4 = assign79850_e122442_d_n4;
        locals.var_qbsld_dn5 = assign79850_e122442_d_n5;
        locals.var_qbsld_dn6 = assign79850_e122442_d_n6;
        locals.var_qbsld_dn7 = assign79850_e122442_d_n7;
        locals.var_qbsld_dn8 = assign79850_e122442_d_n8;
        locals.var_qbsld_dn9 = assign79850_e122442_d_n9;
        locals.var_qbsld_dn10 = assign79850_e122442_d_n10;
        locals.var_qbsld_dn11 = assign79850_e122442_d_n11;
        locals.var_qbsld_dn14 = assign79850_e122442_d_n14;

        let (assign79880_e122467, assign79880_e122467_d_n0, assign79880_e122467_d_n2, assign79880_e122467_d_n4, assign79880_e122467_d_n5, assign79880_e122467_d_n6, assign79880_e122467_d_n7, assign79880_e122467_d_n8, assign79880_e122467_d_n9, assign79880_e122467_d_n10, assign79880_e122467_d_n11, assign79880_e122467_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard1865 != 0.0) && (locals.var_guard1864 == 0.0))) {
        let assign79880_e122463: f64 = (locals.var_weffcv_nf * locals.var_lover_func);
        let assign79880_e122465: f64 = (assign79880_e122463 * locals.var_uc_cvdsover);
        (assign79880_e122465, ((locals.var_weffcv_nf * locals.var_lover_func_dn0) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn2) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn4) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn5) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn6) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn7) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn8) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn9) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn10) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn11) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn14) * locals.var_uc_cvdsover),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign79880_e122467;
        locals.var_t4_dn0 = assign79880_e122467_d_n0;
        locals.var_t4_dn2 = assign79880_e122467_d_n2;
        locals.var_t4_dn4 = assign79880_e122467_d_n4;
        locals.var_t4_dn5 = assign79880_e122467_d_n5;
        locals.var_t4_dn6 = assign79880_e122467_d_n6;
        locals.var_t4_dn7 = assign79880_e122467_d_n7;
        locals.var_t4_dn8 = assign79880_e122467_d_n8;
        locals.var_t4_dn9 = assign79880_e122467_d_n9;
        locals.var_t4_dn10 = assign79880_e122467_d_n10;
        locals.var_t4_dn11 = assign79880_e122467_d_n11;
        locals.var_t4_dn14 = assign79880_e122467_d_n14;

        let (assign79890_e122478, assign79890_e122478_d_n0, assign79890_e122478_d_n2, assign79890_e122478_d_n4, assign79890_e122478_d_n5, assign79890_e122478_d_n6, assign79890_e122478_d_n7, assign79890_e122478_d_n8, assign79890_e122478_d_n9, assign79890_e122478_d_n10, assign79890_e122478_d_n11, assign79890_e122478_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard1865 != 0.0) && (locals.var_guard1864 == 0.0))) {
        let assign79890_e122476: f64 = (locals.var_t4 * locals.var_qsuld);
        (assign79890_e122476, ((locals.var_t4_dn0 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn0)), ((locals.var_t4_dn2 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn2)), ((locals.var_t4_dn4 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn4)), ((locals.var_t4_dn5 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn5)), ((locals.var_t4_dn6 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn6)), ((locals.var_t4_dn7 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn7)), ((locals.var_t4_dn8 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn8)), ((locals.var_t4_dn9 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn9)), ((locals.var_t4_dn10 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn10)), ((locals.var_t4_dn11 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn11)), ((locals.var_t4_dn14 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn14)),)
    } else {
        (locals.var_qovsext, locals.var_qovsext_dn0, locals.var_qovsext_dn2, locals.var_qovsext_dn4, locals.var_qovsext_dn5, locals.var_qovsext_dn6, locals.var_qovsext_dn7, locals.var_qovsext_dn8, locals.var_qovsext_dn9, locals.var_qovsext_dn10, locals.var_qovsext_dn11, locals.var_qovsext_dn14,)
    }
};
        locals.var_qovsext = assign79890_e122478;
        locals.var_qovsext_dn0 = assign79890_e122478_d_n0;
        locals.var_qovsext_dn2 = assign79890_e122478_d_n2;
        locals.var_qovsext_dn4 = assign79890_e122478_d_n4;
        locals.var_qovsext_dn5 = assign79890_e122478_d_n5;
        locals.var_qovsext_dn6 = assign79890_e122478_d_n6;
        locals.var_qovsext_dn7 = assign79890_e122478_d_n7;
        locals.var_qovsext_dn8 = assign79890_e122478_d_n8;
        locals.var_qovsext_dn9 = assign79890_e122478_d_n9;
        locals.var_qovsext_dn10 = assign79890_e122478_d_n10;
        locals.var_qovsext_dn11 = assign79890_e122478_d_n11;
        locals.var_qovsext_dn14 = assign79890_e122478_d_n14;

        let (assign79900_e122489, assign79900_e122489_d_n0, assign79900_e122489_d_n2, assign79900_e122489_d_n4, assign79900_e122489_d_n5, assign79900_e122489_d_n6, assign79900_e122489_d_n7, assign79900_e122489_d_n8, assign79900_e122489_d_n9, assign79900_e122489_d_n10, assign79900_e122489_d_n11, assign79900_e122489_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard1865 != 0.0) && (locals.var_guard1864 == 0.0))) {
        let assign79900_e122487: f64 = (locals.var_t4 * locals.var_qbuld);
        (assign79900_e122487, ((locals.var_t4_dn0 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn0)), ((locals.var_t4_dn2 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn2)), ((locals.var_t4_dn4 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn4)), ((locals.var_t4_dn5 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn5)), ((locals.var_t4_dn6 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn6)), ((locals.var_t4_dn7 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn7)), ((locals.var_t4_dn8 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn8)), ((locals.var_t4_dn9 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn9)), ((locals.var_t4_dn10 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn10)), ((locals.var_t4_dn11 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn11)), ((locals.var_t4_dn14 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn14)),)
    } else {
        (locals.var_qbsldext, locals.var_qbsldext_dn0, locals.var_qbsldext_dn2, locals.var_qbsldext_dn4, locals.var_qbsldext_dn5, locals.var_qbsldext_dn6, locals.var_qbsldext_dn7, locals.var_qbsldext_dn8, locals.var_qbsldext_dn9, locals.var_qbsldext_dn10, locals.var_qbsldext_dn11, locals.var_qbsldext_dn14,)
    }
};
        locals.var_qbsldext = assign79900_e122489;
        locals.var_qbsldext_dn0 = assign79900_e122489_d_n0;
        locals.var_qbsldext_dn2 = assign79900_e122489_d_n2;
        locals.var_qbsldext_dn4 = assign79900_e122489_d_n4;
        locals.var_qbsldext_dn5 = assign79900_e122489_d_n5;
        locals.var_qbsldext_dn6 = assign79900_e122489_d_n6;
        locals.var_qbsldext_dn7 = assign79900_e122489_d_n7;
        locals.var_qbsldext_dn8 = assign79900_e122489_d_n8;
        locals.var_qbsldext_dn9 = assign79900_e122489_d_n9;
        locals.var_qbsldext_dn10 = assign79900_e122489_d_n10;
        locals.var_qbsldext_dn11 = assign79900_e122489_d_n11;
        locals.var_qbsldext_dn14 = assign79900_e122489_d_n14;

        let assign79910_e122492: f64 = if p.p55 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1869 = assign79910_e122492;

        let (assign79920_e122507, assign79920_e122507_d_n0, assign79920_e122507_d_n2, assign79920_e122507_d_n4, assign79920_e122507_d_n5, assign79920_e122507_d_n6, assign79920_e122507_d_n7, assign79920_e122507_d_n8, assign79920_e122507_d_n9, assign79920_e122507_d_n10, assign79920_e122507_d_n11, assign79920_e122507_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && ((locals.var_guard1866 != 0.0) && (!((locals.var_guard1864 != 0.0) || (locals.var_guard1865 != 0.0))))) && (locals.var_guard1869 != 0.0)) {
        let assign79920_e122505: f64 = (locals.var_weffcv_nf * locals.var_lover_func);
        (assign79920_e122505, (locals.var_weffcv_nf * locals.var_lover_func_dn0), (locals.var_weffcv_nf * locals.var_lover_func_dn2), (locals.var_weffcv_nf * locals.var_lover_func_dn4), (locals.var_weffcv_nf * locals.var_lover_func_dn5), (locals.var_weffcv_nf * locals.var_lover_func_dn6), (locals.var_weffcv_nf * locals.var_lover_func_dn7), (locals.var_weffcv_nf * locals.var_lover_func_dn8), (locals.var_weffcv_nf * locals.var_lover_func_dn9), (locals.var_weffcv_nf * locals.var_lover_func_dn10), (locals.var_weffcv_nf * locals.var_lover_func_dn11), (locals.var_weffcv_nf * locals.var_lover_func_dn14),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign79920_e122507;
        locals.var_t4_dn0 = assign79920_e122507_d_n0;
        locals.var_t4_dn2 = assign79920_e122507_d_n2;
        locals.var_t4_dn4 = assign79920_e122507_d_n4;
        locals.var_t4_dn5 = assign79920_e122507_d_n5;
        locals.var_t4_dn6 = assign79920_e122507_d_n6;
        locals.var_t4_dn7 = assign79920_e122507_d_n7;
        locals.var_t4_dn8 = assign79920_e122507_d_n8;
        locals.var_t4_dn9 = assign79920_e122507_d_n9;
        locals.var_t4_dn10 = assign79920_e122507_d_n10;
        locals.var_t4_dn11 = assign79920_e122507_d_n11;
        locals.var_t4_dn14 = assign79920_e122507_d_n14;

        let (assign79930_e122527, assign79930_e122527_d_n0, assign79930_e122527_d_n2, assign79930_e122527_d_n4, assign79930_e122527_d_n5, assign79930_e122527_d_n6, assign79930_e122527_d_n7, assign79930_e122527_d_n8, assign79930_e122527_d_n9, assign79930_e122527_d_n10, assign79930_e122527_d_n11, assign79930_e122527_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && ((locals.var_guard1866 != 0.0) && (!((locals.var_guard1864 != 0.0) || (locals.var_guard1865 != 0.0))))) && (locals.var_guard1869 == 0.0)) {
        let assign79930_e122521: f64 = (locals.var_weffcv_nf * locals.var_lover_func);
        let assign79930_e122524: f64 = (1.0 - locals.var_uc_cvdsover);
        let assign79930_e122525: f64 = (assign79930_e122521 * assign79930_e122524);
        (assign79930_e122525, ((locals.var_weffcv_nf * locals.var_lover_func_dn0) * assign79930_e122524), ((locals.var_weffcv_nf * locals.var_lover_func_dn2) * assign79930_e122524), ((locals.var_weffcv_nf * locals.var_lover_func_dn4) * assign79930_e122524), ((locals.var_weffcv_nf * locals.var_lover_func_dn5) * assign79930_e122524), ((locals.var_weffcv_nf * locals.var_lover_func_dn6) * assign79930_e122524), ((locals.var_weffcv_nf * locals.var_lover_func_dn7) * assign79930_e122524), ((locals.var_weffcv_nf * locals.var_lover_func_dn8) * assign79930_e122524), ((locals.var_weffcv_nf * locals.var_lover_func_dn9) * assign79930_e122524), ((locals.var_weffcv_nf * locals.var_lover_func_dn10) * assign79930_e122524), ((locals.var_weffcv_nf * locals.var_lover_func_dn11) * assign79930_e122524), ((locals.var_weffcv_nf * locals.var_lover_func_dn14) * assign79930_e122524),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign79930_e122527;
        locals.var_t4_dn0 = assign79930_e122527_d_n0;
        locals.var_t4_dn2 = assign79930_e122527_d_n2;
        locals.var_t4_dn4 = assign79930_e122527_d_n4;
        locals.var_t4_dn5 = assign79930_e122527_d_n5;
        locals.var_t4_dn6 = assign79930_e122527_d_n6;
        locals.var_t4_dn7 = assign79930_e122527_d_n7;
        locals.var_t4_dn8 = assign79930_e122527_d_n8;
        locals.var_t4_dn9 = assign79930_e122527_d_n9;
        locals.var_t4_dn10 = assign79930_e122527_d_n10;
        locals.var_t4_dn11 = assign79930_e122527_d_n11;
        locals.var_t4_dn14 = assign79930_e122527_d_n14;

        let (assign79940_e122538, assign79940_e122538_d_n0, assign79940_e122538_d_n2, assign79940_e122538_d_n4, assign79940_e122538_d_n5, assign79940_e122538_d_n6, assign79940_e122538_d_n7, assign79940_e122538_d_n8, assign79940_e122538_d_n9, assign79940_e122538_d_n10, assign79940_e122538_d_n11, assign79940_e122538_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard1866 != 0.0) && (!((locals.var_guard1864 != 0.0) || (locals.var_guard1865 != 0.0))))) {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn14,)
    } else {
        (locals.var_rd_ps0ld, locals.var_rd_ps0ld_dn0, locals.var_rd_ps0ld_dn2, locals.var_rd_ps0ld_dn4, locals.var_rd_ps0ld_dn5, locals.var_rd_ps0ld_dn6, locals.var_rd_ps0ld_dn7, locals.var_rd_ps0ld_dn8, locals.var_rd_ps0ld_dn9, locals.var_rd_ps0ld_dn10, locals.var_rd_ps0ld_dn11, locals.var_rd_ps0ld_dn14,)
    }
};
        locals.var_rd_ps0ld = assign79940_e122538;
        locals.var_rd_ps0ld_dn0 = assign79940_e122538_d_n0;
        locals.var_rd_ps0ld_dn2 = assign79940_e122538_d_n2;
        locals.var_rd_ps0ld_dn4 = assign79940_e122538_d_n4;
        locals.var_rd_ps0ld_dn5 = assign79940_e122538_d_n5;
        locals.var_rd_ps0ld_dn6 = assign79940_e122538_d_n6;
        locals.var_rd_ps0ld_dn7 = assign79940_e122538_d_n7;
        locals.var_rd_ps0ld_dn8 = assign79940_e122538_d_n8;
        locals.var_rd_ps0ld_dn9 = assign79940_e122538_d_n9;
        locals.var_rd_ps0ld_dn10 = assign79940_e122538_d_n10;
        locals.var_rd_ps0ld_dn11 = assign79940_e122538_d_n11;
        locals.var_rd_ps0ld_dn14 = assign79940_e122538_d_n14;

        let assign79950_e122541: f64 = if p.p430 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1870 = assign79950_e122541;

        let (assign79960_e122554, assign79960_e122554_d_n0, assign79960_e122554_d_n2, assign79960_e122554_d_n4, assign79960_e122554_d_n5, assign79960_e122554_d_n6, assign79960_e122554_d_n7, assign79960_e122554_d_n8, assign79960_e122554_d_n9, assign79960_e122554_d_n10, assign79960_e122554_d_n11, assign79960_e122554_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && ((locals.var_guard1866 != 0.0) && (!((locals.var_guard1864 != 0.0) || (locals.var_guard1865 != 0.0))))) && (locals.var_guard1870 != 0.0)) {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn8, locals.var_qbuld_dn9, locals.var_qbuld_dn10, locals.var_qbuld_dn11, locals.var_qbuld_dn14,)
    } else {
        (locals.var_rd_qbuld, locals.var_rd_qbuld_dn0, locals.var_rd_qbuld_dn2, locals.var_rd_qbuld_dn4, locals.var_rd_qbuld_dn5, locals.var_rd_qbuld_dn6, locals.var_rd_qbuld_dn7, locals.var_rd_qbuld_dn8, locals.var_rd_qbuld_dn9, locals.var_rd_qbuld_dn10, locals.var_rd_qbuld_dn11, locals.var_rd_qbuld_dn14,)
    }
};
        locals.var_rd_qbuld = assign79960_e122554;
        locals.var_rd_qbuld_dn0 = assign79960_e122554_d_n0;
        locals.var_rd_qbuld_dn2 = assign79960_e122554_d_n2;
        locals.var_rd_qbuld_dn4 = assign79960_e122554_d_n4;
        locals.var_rd_qbuld_dn5 = assign79960_e122554_d_n5;
        locals.var_rd_qbuld_dn6 = assign79960_e122554_d_n6;
        locals.var_rd_qbuld_dn7 = assign79960_e122554_d_n7;
        locals.var_rd_qbuld_dn8 = assign79960_e122554_d_n8;
        locals.var_rd_qbuld_dn9 = assign79960_e122554_d_n9;
        locals.var_rd_qbuld_dn10 = assign79960_e122554_d_n10;
        locals.var_rd_qbuld_dn11 = assign79960_e122554_d_n11;
        locals.var_rd_qbuld_dn14 = assign79960_e122554_d_n14;

        let (assign79970_e122567, assign79970_e122567_d_n0, assign79970_e122567_d_n2, assign79970_e122567_d_n4, assign79970_e122567_d_n5, assign79970_e122567_d_n6, assign79970_e122567_d_n7, assign79970_e122567_d_n8, assign79970_e122567_d_n9, assign79970_e122567_d_n10, assign79970_e122567_d_n11, assign79970_e122567_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard1866 != 0.0) && (!((locals.var_guard1864 != 0.0) || (locals.var_guard1865 != 0.0))))) {
        let assign79970_e122565: f64 = (locals.var_t4 * locals.var_qsuld);
        (assign79970_e122565, ((locals.var_t4_dn0 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn0)), ((locals.var_t4_dn2 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn2)), ((locals.var_t4_dn4 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn4)), ((locals.var_t4_dn5 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn5)), ((locals.var_t4_dn6 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn6)), ((locals.var_t4_dn7 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn7)), ((locals.var_t4_dn8 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn8)), ((locals.var_t4_dn9 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn9)), ((locals.var_t4_dn10 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn10)), ((locals.var_t4_dn11 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn11)), ((locals.var_t4_dn14 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn14)),)
    } else {
        (locals.var_qovd, locals.var_qovd_dn0, locals.var_qovd_dn2, locals.var_qovd_dn4, locals.var_qovd_dn5, locals.var_qovd_dn6, locals.var_qovd_dn7, locals.var_qovd_dn8, locals.var_qovd_dn9, locals.var_qovd_dn10, locals.var_qovd_dn11, locals.var_qovd_dn14,)
    }
};
        locals.var_qovd = assign79970_e122567;
        locals.var_qovd_dn0 = assign79970_e122567_d_n0;
        locals.var_qovd_dn2 = assign79970_e122567_d_n2;
        locals.var_qovd_dn4 = assign79970_e122567_d_n4;
        locals.var_qovd_dn5 = assign79970_e122567_d_n5;
        locals.var_qovd_dn6 = assign79970_e122567_d_n6;
        locals.var_qovd_dn7 = assign79970_e122567_d_n7;
        locals.var_qovd_dn8 = assign79970_e122567_d_n8;
        locals.var_qovd_dn9 = assign79970_e122567_d_n9;
        locals.var_qovd_dn10 = assign79970_e122567_d_n10;
        locals.var_qovd_dn11 = assign79970_e122567_d_n11;
        locals.var_qovd_dn14 = assign79970_e122567_d_n14;

        let (assign79980_e122580, assign79980_e122580_d_n0, assign79980_e122580_d_n2, assign79980_e122580_d_n4, assign79980_e122580_d_n5, assign79980_e122580_d_n6, assign79980_e122580_d_n7, assign79980_e122580_d_n8, assign79980_e122580_d_n9, assign79980_e122580_d_n10, assign79980_e122580_d_n11, assign79980_e122580_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard1866 != 0.0) && (!((locals.var_guard1864 != 0.0) || (locals.var_guard1865 != 0.0))))) {
        let assign79980_e122578: f64 = (locals.var_t4 * locals.var_qbuld);
        (assign79980_e122578, ((locals.var_t4_dn0 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn0)), ((locals.var_t4_dn2 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn2)), ((locals.var_t4_dn4 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn4)), ((locals.var_t4_dn5 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn5)), ((locals.var_t4_dn6 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn6)), ((locals.var_t4_dn7 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn7)), ((locals.var_t4_dn8 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn8)), ((locals.var_t4_dn9 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn9)), ((locals.var_t4_dn10 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn10)), ((locals.var_t4_dn11 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn11)), ((locals.var_t4_dn14 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn14)),)
    } else {
        (locals.var_qbdld, locals.var_qbdld_dn0, locals.var_qbdld_dn2, locals.var_qbdld_dn4, locals.var_qbdld_dn5, locals.var_qbdld_dn6, locals.var_qbdld_dn7, locals.var_qbdld_dn8, locals.var_qbdld_dn9, locals.var_qbdld_dn10, locals.var_qbdld_dn11, locals.var_qbdld_dn14,)
    }
};
        locals.var_qbdld = assign79980_e122580;
        locals.var_qbdld_dn0 = assign79980_e122580_d_n0;
        locals.var_qbdld_dn2 = assign79980_e122580_d_n2;
        locals.var_qbdld_dn4 = assign79980_e122580_d_n4;
        locals.var_qbdld_dn5 = assign79980_e122580_d_n5;
        locals.var_qbdld_dn6 = assign79980_e122580_d_n6;
        locals.var_qbdld_dn7 = assign79980_e122580_d_n7;
        locals.var_qbdld_dn8 = assign79980_e122580_d_n8;
        locals.var_qbdld_dn9 = assign79980_e122580_d_n9;
        locals.var_qbdld_dn10 = assign79980_e122580_d_n10;
        locals.var_qbdld_dn11 = assign79980_e122580_d_n11;
        locals.var_qbdld_dn14 = assign79980_e122580_d_n14;

        let (assign79990_e122591, assign79990_e122591_d_n0, assign79990_e122591_d_n2, assign79990_e122591_d_n4, assign79990_e122591_d_n5, assign79990_e122591_d_n6, assign79990_e122591_d_n7, assign79990_e122591_d_n8, assign79990_e122591_d_n9, assign79990_e122591_d_n10, assign79990_e122591_d_n11, assign79990_e122591_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard1866 != 0.0) && (!((locals.var_guard1864 != 0.0) || (locals.var_guard1865 != 0.0))))) {
        (locals.var_qbdld, locals.var_qbdld_dn0, locals.var_qbdld_dn2, locals.var_qbdld_dn4, locals.var_qbdld_dn5, locals.var_qbdld_dn6, locals.var_qbdld_dn7, locals.var_qbdld_dn8, locals.var_qbdld_dn9, locals.var_qbdld_dn10, locals.var_qbdld_dn11, locals.var_qbdld_dn14,)
    } else {
        (locals.var_qbd_qs, locals.var_qbd_qs_dn0, locals.var_qbd_qs_dn2, locals.var_qbd_qs_dn4, locals.var_qbd_qs_dn5, locals.var_qbd_qs_dn6, locals.var_qbd_qs_dn7, locals.var_qbd_qs_dn8, locals.var_qbd_qs_dn9, locals.var_qbd_qs_dn10, locals.var_qbd_qs_dn11, locals.var_qbd_qs_dn14,)
    }
};
        locals.var_qbd_qs = assign79990_e122591;
        locals.var_qbd_qs_dn0 = assign79990_e122591_d_n0;
        locals.var_qbd_qs_dn2 = assign79990_e122591_d_n2;
        locals.var_qbd_qs_dn4 = assign79990_e122591_d_n4;
        locals.var_qbd_qs_dn5 = assign79990_e122591_d_n5;
        locals.var_qbd_qs_dn6 = assign79990_e122591_d_n6;
        locals.var_qbd_qs_dn7 = assign79990_e122591_d_n7;
        locals.var_qbd_qs_dn8 = assign79990_e122591_d_n8;
        locals.var_qbd_qs_dn9 = assign79990_e122591_d_n9;
        locals.var_qbd_qs_dn10 = assign79990_e122591_d_n10;
        locals.var_qbd_qs_dn11 = assign79990_e122591_d_n11;
        locals.var_qbd_qs_dn14 = assign79990_e122591_d_n14;

        let (assign80000_e122608, assign80000_e122608_d_n0, assign80000_e122608_d_n2, assign80000_e122608_d_n4, assign80000_e122608_d_n5, assign80000_e122608_d_n6, assign80000_e122608_d_n7, assign80000_e122608_d_n8, assign80000_e122608_d_n9, assign80000_e122608_d_n10, assign80000_e122608_d_n11, assign80000_e122608_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard1867 != 0.0) && (!(((locals.var_guard1864 != 0.0) || (locals.var_guard1865 != 0.0)) || (locals.var_guard1866 != 0.0))))) {
        let assign80000_e122604: f64 = (locals.var_weffcv_nf * locals.var_lover_func);
        let assign80000_e122606: f64 = (assign80000_e122604 * locals.var_uc_cvdsover);
        (assign80000_e122606, ((locals.var_weffcv_nf * locals.var_lover_func_dn0) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn2) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn4) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn5) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn6) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn7) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn8) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn9) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn10) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn11) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn14) * locals.var_uc_cvdsover),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign80000_e122608;
        locals.var_t4_dn0 = assign80000_e122608_d_n0;
        locals.var_t4_dn2 = assign80000_e122608_d_n2;
        locals.var_t4_dn4 = assign80000_e122608_d_n4;
        locals.var_t4_dn5 = assign80000_e122608_d_n5;
        locals.var_t4_dn6 = assign80000_e122608_d_n6;
        locals.var_t4_dn7 = assign80000_e122608_d_n7;
        locals.var_t4_dn8 = assign80000_e122608_d_n8;
        locals.var_t4_dn9 = assign80000_e122608_d_n9;
        locals.var_t4_dn10 = assign80000_e122608_d_n10;
        locals.var_t4_dn11 = assign80000_e122608_d_n11;
        locals.var_t4_dn14 = assign80000_e122608_d_n14;

        let (assign80010_e122623, assign80010_e122623_d_n0, assign80010_e122623_d_n2, assign80010_e122623_d_n4, assign80010_e122623_d_n5, assign80010_e122623_d_n6, assign80010_e122623_d_n7, assign80010_e122623_d_n8, assign80010_e122623_d_n9, assign80010_e122623_d_n10, assign80010_e122623_d_n11, assign80010_e122623_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard1867 != 0.0) && (!(((locals.var_guard1864 != 0.0) || (locals.var_guard1865 != 0.0)) || (locals.var_guard1866 != 0.0))))) {
        let assign80010_e122621: f64 = (locals.var_t4 * locals.var_qsuld);
        (assign80010_e122621, ((locals.var_t4_dn0 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn0)), ((locals.var_t4_dn2 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn2)), ((locals.var_t4_dn4 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn4)), ((locals.var_t4_dn5 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn5)), ((locals.var_t4_dn6 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn6)), ((locals.var_t4_dn7 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn7)), ((locals.var_t4_dn8 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn8)), ((locals.var_t4_dn9 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn9)), ((locals.var_t4_dn10 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn10)), ((locals.var_t4_dn11 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn11)), ((locals.var_t4_dn14 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn14)),)
    } else {
        (locals.var_qovdext, locals.var_qovdext_dn0, locals.var_qovdext_dn2, locals.var_qovdext_dn4, locals.var_qovdext_dn5, locals.var_qovdext_dn6, locals.var_qovdext_dn7, locals.var_qovdext_dn8, locals.var_qovdext_dn9, locals.var_qovdext_dn10, locals.var_qovdext_dn11, locals.var_qovdext_dn14,)
    }
};
        locals.var_qovdext = assign80010_e122623;
        locals.var_qovdext_dn0 = assign80010_e122623_d_n0;
        locals.var_qovdext_dn2 = assign80010_e122623_d_n2;
        locals.var_qovdext_dn4 = assign80010_e122623_d_n4;
        locals.var_qovdext_dn5 = assign80010_e122623_d_n5;
        locals.var_qovdext_dn6 = assign80010_e122623_d_n6;
        locals.var_qovdext_dn7 = assign80010_e122623_d_n7;
        locals.var_qovdext_dn8 = assign80010_e122623_d_n8;
        locals.var_qovdext_dn9 = assign80010_e122623_d_n9;
        locals.var_qovdext_dn10 = assign80010_e122623_d_n10;
        locals.var_qovdext_dn11 = assign80010_e122623_d_n11;
        locals.var_qovdext_dn14 = assign80010_e122623_d_n14;

        let (assign80020_e122638, assign80020_e122638_d_n0, assign80020_e122638_d_n2, assign80020_e122638_d_n4, assign80020_e122638_d_n5, assign80020_e122638_d_n6, assign80020_e122638_d_n7, assign80020_e122638_d_n8, assign80020_e122638_d_n9, assign80020_e122638_d_n10, assign80020_e122638_d_n11, assign80020_e122638_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard1867 != 0.0) && (!(((locals.var_guard1864 != 0.0) || (locals.var_guard1865 != 0.0)) || (locals.var_guard1866 != 0.0))))) {
        let assign80020_e122636: f64 = (locals.var_t4 * locals.var_qbuld);
        (assign80020_e122636, ((locals.var_t4_dn0 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn0)), ((locals.var_t4_dn2 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn2)), ((locals.var_t4_dn4 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn4)), ((locals.var_t4_dn5 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn5)), ((locals.var_t4_dn6 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn6)), ((locals.var_t4_dn7 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn7)), ((locals.var_t4_dn8 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn8)), ((locals.var_t4_dn9 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn9)), ((locals.var_t4_dn10 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn10)), ((locals.var_t4_dn11 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn11)), ((locals.var_t4_dn14 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn14)),)
    } else {
        (locals.var_qbdldext, locals.var_qbdldext_dn0, locals.var_qbdldext_dn2, locals.var_qbdldext_dn4, locals.var_qbdldext_dn5, locals.var_qbdldext_dn6, locals.var_qbdldext_dn7, locals.var_qbdldext_dn8, locals.var_qbdldext_dn9, locals.var_qbdldext_dn10, locals.var_qbdldext_dn11, locals.var_qbdldext_dn14,)
    }
};
        locals.var_qbdldext = assign80020_e122638;
        locals.var_qbdldext_dn0 = assign80020_e122638_d_n0;
        locals.var_qbdldext_dn2 = assign80020_e122638_d_n2;
        locals.var_qbdldext_dn4 = assign80020_e122638_d_n4;
        locals.var_qbdldext_dn5 = assign80020_e122638_d_n5;
        locals.var_qbdldext_dn6 = assign80020_e122638_d_n6;
        locals.var_qbdldext_dn7 = assign80020_e122638_d_n7;
        locals.var_qbdldext_dn8 = assign80020_e122638_d_n8;
        locals.var_qbdldext_dn9 = assign80020_e122638_d_n9;
        locals.var_qbdldext_dn10 = assign80020_e122638_d_n10;
        locals.var_qbdldext_dn11 = assign80020_e122638_d_n11;
        locals.var_qbdldext_dn14 = assign80020_e122638_d_n14;

        locals.var_flg_calcqover = 0.0;

        let assign80040_e122642: f64 = if 3.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1871 = assign80040_e122642;

        let assign80050_e122645: f64 = if 3.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1872 = assign80050_e122645;

        let assign80060_e122648: f64 = if 3.0 == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard1873 = assign80060_e122648;

        let assign80070_e122651: f64 = if 3.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1874 = assign80070_e122651;

        let assign80080_e122662: f64 = if (((p.p36 == 1.0) && (p.p66 > 0.0)) && (locals.var_uc_novers > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1875 = assign80080_e122662;

        let (assign80090_e122668,) = {
    if ((locals.var_guard1871 != 0.0) && (locals.var_guard1875 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_calcqover,)
    }
};
        locals.var_flg_calcqover = assign80090_e122668;

        let (assign80100_e122674,) = {
    if ((locals.var_guard1871 != 0.0) && (locals.var_guard1875 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_coovlps,)
    }
};
        locals.var_flg_coovlps = assign80100_e122674;

    }

    pub(super) fn stamp_transient_block_290(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign80110_e122682, assign80110_e122682_d_n2, assign80110_e122682_d_n7, assign80110_e122682_d_n8, assign80110_e122682_d_n9,) = {
    if ((locals.var_guard1871 != 0.0) && (locals.var_guard1875 != 0.0)) {
        let assign80110_e122680: f64 = (locals.var_vgsi - locals.var_vbsi);
        (assign80110_e122680, 0.0, locals.var_vgsi_dn7, (locals.var_vgsi_dn8 - locals.var_vbsi_dn8), (-locals.var_vbsi_dn9),)
    } else {
        (locals.var_vgbgmt, locals.var_vgbgmt_dn2, locals.var_vgbgmt_dn7, locals.var_vgbgmt_dn8, locals.var_vgbgmt_dn9,)
    }
};
        locals.var_vgbgmt = assign80110_e122682;
        locals.var_vgbgmt_dn2 = assign80110_e122682_d_n2;
        locals.var_vgbgmt_dn7 = assign80110_e122682_d_n7;
        locals.var_vgbgmt_dn8 = assign80110_e122682_d_n8;
        locals.var_vgbgmt_dn9 = assign80110_e122682_d_n9;

        let (assign80120_e122689, assign80120_e122689_d_n0, assign80120_e122689_d_n2, assign80120_e122689_d_n4, assign80120_e122689_d_n5, assign80120_e122689_d_n6, assign80120_e122689_d_n7, assign80120_e122689_d_n8, assign80120_e122689_d_n9, assign80120_e122689_d_n10, assign80120_e122689_d_n11, assign80120_e122689_d_n14,) = {
    if ((locals.var_guard1871 != 0.0) && (locals.var_guard1875 != 0.0)) {
        let assign80120_e122687: f64 = (-locals.var_vbsi);
        (assign80120_e122687, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, (-locals.var_vbsi_dn8), (-locals.var_vbsi_dn9), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn11, locals.var_vxbgmt_dn14,)
    }
};
        locals.var_vxbgmt = assign80120_e122689;
        locals.var_vxbgmt_dn0 = assign80120_e122689_d_n0;
        locals.var_vxbgmt_dn2 = assign80120_e122689_d_n2;
        locals.var_vxbgmt_dn4 = assign80120_e122689_d_n4;
        locals.var_vxbgmt_dn5 = assign80120_e122689_d_n5;
        locals.var_vxbgmt_dn6 = assign80120_e122689_d_n6;
        locals.var_vxbgmt_dn7 = assign80120_e122689_d_n7;
        locals.var_vxbgmt_dn8 = assign80120_e122689_d_n8;
        locals.var_vxbgmt_dn9 = assign80120_e122689_d_n9;
        locals.var_vxbgmt_dn10 = assign80120_e122689_d_n10;
        locals.var_vxbgmt_dn11 = assign80120_e122689_d_n11;
        locals.var_vxbgmt_dn14 = assign80120_e122689_d_n14;

        let (assign80130_e122695,) = {
    if ((locals.var_guard1871 != 0.0) && (locals.var_guard1875 != 0.0)) {
        (locals.var_uc_novers,)
    } else {
        (locals.var_nover_func,)
    }
};
        locals.var_nover_func = assign80130_e122695;

        let (assign80140_e122701, assign80140_e122701_d_n0, assign80140_e122701_d_n2, assign80140_e122701_d_n4, assign80140_e122701_d_n5, assign80140_e122701_d_n6, assign80140_e122701_d_n7, assign80140_e122701_d_n8, assign80140_e122701_d_n9, assign80140_e122701_d_n10, assign80140_e122701_d_n11, assign80140_e122701_d_n14,) = {
    if ((locals.var_guard1871 != 0.0) && (locals.var_guard1875 != 0.0)) {
        (p.p66, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_lover_func, locals.var_lover_func_dn0, locals.var_lover_func_dn2, locals.var_lover_func_dn4, locals.var_lover_func_dn5, locals.var_lover_func_dn6, locals.var_lover_func_dn7, locals.var_lover_func_dn8, locals.var_lover_func_dn9, locals.var_lover_func_dn10, locals.var_lover_func_dn11, locals.var_lover_func_dn14,)
    }
};
        locals.var_lover_func = assign80140_e122701;
        locals.var_lover_func_dn0 = assign80140_e122701_d_n0;
        locals.var_lover_func_dn2 = assign80140_e122701_d_n2;
        locals.var_lover_func_dn4 = assign80140_e122701_d_n4;
        locals.var_lover_func_dn5 = assign80140_e122701_d_n5;
        locals.var_lover_func_dn6 = assign80140_e122701_d_n6;
        locals.var_lover_func_dn7 = assign80140_e122701_d_n7;
        locals.var_lover_func_dn8 = assign80140_e122701_d_n8;
        locals.var_lover_func_dn9 = assign80140_e122701_d_n9;
        locals.var_lover_func_dn10 = assign80140_e122701_d_n10;
        locals.var_lover_func_dn11 = assign80140_e122701_d_n11;
        locals.var_lover_func_dn14 = assign80140_e122701_d_n14;

        let (assign80150_e122707, assign80150_e122707_d_n0, assign80150_e122707_d_n2, assign80150_e122707_d_n4, assign80150_e122707_d_n5, assign80150_e122707_d_n6, assign80150_e122707_d_n7, assign80150_e122707_d_n8, assign80150_e122707_d_n9, assign80150_e122707_d_n10, assign80150_e122707_d_n11, assign80150_e122707_d_n14,) = {
    if ((locals.var_guard1871 != 0.0) && (locals.var_guard1875 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_wdep_func, locals.var_wdep_func_dn0, locals.var_wdep_func_dn2, locals.var_wdep_func_dn4, locals.var_wdep_func_dn5, locals.var_wdep_func_dn6, locals.var_wdep_func_dn7, locals.var_wdep_func_dn8, locals.var_wdep_func_dn9, locals.var_wdep_func_dn10, locals.var_wdep_func_dn11, locals.var_wdep_func_dn14,)
    }
};
        locals.var_wdep_func = assign80150_e122707;
        locals.var_wdep_func_dn0 = assign80150_e122707_d_n0;
        locals.var_wdep_func_dn2 = assign80150_e122707_d_n2;
        locals.var_wdep_func_dn4 = assign80150_e122707_d_n4;
        locals.var_wdep_func_dn5 = assign80150_e122707_d_n5;
        locals.var_wdep_func_dn6 = assign80150_e122707_d_n6;
        locals.var_wdep_func_dn7 = assign80150_e122707_d_n7;
        locals.var_wdep_func_dn8 = assign80150_e122707_d_n8;
        locals.var_wdep_func_dn9 = assign80150_e122707_d_n9;
        locals.var_wdep_func_dn10 = assign80150_e122707_d_n10;
        locals.var_wdep_func_dn11 = assign80150_e122707_d_n11;
        locals.var_wdep_func_dn14 = assign80150_e122707_d_n14;

        let (assign80160_e122713, assign80160_e122713_d_n0, assign80160_e122713_d_n2, assign80160_e122713_d_n4, assign80160_e122713_d_n5, assign80160_e122713_d_n6, assign80160_e122713_d_n7, assign80160_e122713_d_n8, assign80160_e122713_d_n9, assign80160_e122713_d_n10, assign80160_e122713_d_n11, assign80160_e122713_d_n14,) = {
    if ((locals.var_guard1871 != 0.0) && (locals.var_guard1875 != 0.0)) {
        (locals.var_cnst0overs, locals.var_cnst0overs_dn0, locals.var_cnst0overs_dn2, locals.var_cnst0overs_dn4, locals.var_cnst0overs_dn5, locals.var_cnst0overs_dn6, locals.var_cnst0overs_dn7, locals.var_cnst0overs_dn8, locals.var_cnst0overs_dn9, locals.var_cnst0overs_dn10, locals.var_cnst0overs_dn11, locals.var_cnst0overs_dn14,)
    } else {
        (locals.var_cnst0over_func, locals.var_cnst0over_func_dn0, locals.var_cnst0over_func_dn2, locals.var_cnst0over_func_dn4, locals.var_cnst0over_func_dn5, locals.var_cnst0over_func_dn6, locals.var_cnst0over_func_dn7, locals.var_cnst0over_func_dn8, locals.var_cnst0over_func_dn9, locals.var_cnst0over_func_dn10, locals.var_cnst0over_func_dn11, locals.var_cnst0over_func_dn14,)
    }
};
        locals.var_cnst0over_func = assign80160_e122713;
        locals.var_cnst0over_func_dn0 = assign80160_e122713_d_n0;
        locals.var_cnst0over_func_dn2 = assign80160_e122713_d_n2;
        locals.var_cnst0over_func_dn4 = assign80160_e122713_d_n4;
        locals.var_cnst0over_func_dn5 = assign80160_e122713_d_n5;
        locals.var_cnst0over_func_dn6 = assign80160_e122713_d_n6;
        locals.var_cnst0over_func_dn7 = assign80160_e122713_d_n7;
        locals.var_cnst0over_func_dn8 = assign80160_e122713_d_n8;
        locals.var_cnst0over_func_dn9 = assign80160_e122713_d_n9;
        locals.var_cnst0over_func_dn10 = assign80160_e122713_d_n10;
        locals.var_cnst0over_func_dn11 = assign80160_e122713_d_n11;
        locals.var_cnst0over_func_dn14 = assign80160_e122713_d_n14;

        let (assign80170_e122719,) = {
    if ((locals.var_guard1871 != 0.0) && (locals.var_guard1875 != 0.0)) {
        (locals.var_cox0,)
    } else {
        (locals.var_cox0_func,)
    }
};
        locals.var_cox0_func = assign80170_e122719;

        let assign80180_e122738: f64 = if (((((p.p36 == 1.0) && (p.p66 > 0.0)) && (locals.var_uc_novers > 0.0)) && (locals.var_uc_cvdsover != 0.0)) && (p.p55 != 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard1876 = assign80180_e122738;

        let (assign80190_e122747,) = {
    if (((locals.var_guard1872 != 0.0) && (locals.var_guard1871 == 0.0)) && (locals.var_guard1876 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_calcqover,)
    }
};
        locals.var_flg_calcqover = assign80190_e122747;

        let (assign80200_e122758, assign80200_e122758_d_n2, assign80200_e122758_d_n7, assign80200_e122758_d_n8, assign80200_e122758_d_n9,) = {
    if (((locals.var_guard1872 != 0.0) && (locals.var_guard1871 == 0.0)) && (locals.var_guard1876 != 0.0)) {
        let assign80200_e122756: f64 = (locals.var_vgsei - locals.var_vbsei);
        (assign80200_e122756, (locals.var_vgsei_dn2 - locals.var_vbsei_dn2), locals.var_vgsei_dn7, 0.0, (-locals.var_vbsei_dn9),)
    } else {
        (locals.var_vgbgmt, locals.var_vgbgmt_dn2, locals.var_vgbgmt_dn7, locals.var_vgbgmt_dn8, locals.var_vgbgmt_dn9,)
    }
};
        locals.var_vgbgmt = assign80200_e122758;
        locals.var_vgbgmt_dn2 = assign80200_e122758_d_n2;
        locals.var_vgbgmt_dn7 = assign80200_e122758_d_n7;
        locals.var_vgbgmt_dn8 = assign80200_e122758_d_n8;
        locals.var_vgbgmt_dn9 = assign80200_e122758_d_n9;

        let (assign80210_e122768, assign80210_e122768_d_n0, assign80210_e122768_d_n2, assign80210_e122768_d_n4, assign80210_e122768_d_n5, assign80210_e122768_d_n6, assign80210_e122768_d_n7, assign80210_e122768_d_n8, assign80210_e122768_d_n9, assign80210_e122768_d_n10, assign80210_e122768_d_n11, assign80210_e122768_d_n14,) = {
    if (((locals.var_guard1872 != 0.0) && (locals.var_guard1871 == 0.0)) && (locals.var_guard1876 != 0.0)) {
        let assign80210_e122766: f64 = (-locals.var_vbsei);
        (assign80210_e122766, 0.0, (-locals.var_vbsei_dn2), 0.0, 0.0, 0.0, 0.0, 0.0, (-locals.var_vbsei_dn9), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn11, locals.var_vxbgmt_dn14,)
    }
};
        locals.var_vxbgmt = assign80210_e122768;
        locals.var_vxbgmt_dn0 = assign80210_e122768_d_n0;
        locals.var_vxbgmt_dn2 = assign80210_e122768_d_n2;
        locals.var_vxbgmt_dn4 = assign80210_e122768_d_n4;
        locals.var_vxbgmt_dn5 = assign80210_e122768_d_n5;
        locals.var_vxbgmt_dn6 = assign80210_e122768_d_n6;
        locals.var_vxbgmt_dn7 = assign80210_e122768_d_n7;
        locals.var_vxbgmt_dn8 = assign80210_e122768_d_n8;
        locals.var_vxbgmt_dn9 = assign80210_e122768_d_n9;
        locals.var_vxbgmt_dn10 = assign80210_e122768_d_n10;
        locals.var_vxbgmt_dn11 = assign80210_e122768_d_n11;
        locals.var_vxbgmt_dn14 = assign80210_e122768_d_n14;

        let assign80220_e122779: f64 = if (((p.p35 == 1.0) && (p.p63 > 0.0)) && (locals.var_uc_nover > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1877 = assign80220_e122779;

        let (assign80230_e122790,) = {
    if (((locals.var_guard1873 != 0.0) && (!((locals.var_guard1871 != 0.0) || (locals.var_guard1872 != 0.0)))) && (locals.var_guard1877 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_calcqover,)
    }
};
        locals.var_flg_calcqover = assign80230_e122790;

        let (assign80240_e122801,) = {
    if (((locals.var_guard1873 != 0.0) && (!((locals.var_guard1871 != 0.0) || (locals.var_guard1872 != 0.0)))) && (locals.var_guard1877 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_coovlp,)
    }
};
        locals.var_flg_coovlp = assign80240_e122801;

        let (assign80250_e122814, assign80250_e122814_d_n2, assign80250_e122814_d_n7, assign80250_e122814_d_n8, assign80250_e122814_d_n9,) = {
    if (((locals.var_guard1873 != 0.0) && (!((locals.var_guard1871 != 0.0) || (locals.var_guard1872 != 0.0)))) && (locals.var_guard1877 != 0.0)) {
        let assign80250_e122812: f64 = (locals.var_vgsi - locals.var_vbsi);
        (assign80250_e122812, 0.0, locals.var_vgsi_dn7, (locals.var_vgsi_dn8 - locals.var_vbsi_dn8), (-locals.var_vbsi_dn9),)
    } else {
        (locals.var_vgbgmt, locals.var_vgbgmt_dn2, locals.var_vgbgmt_dn7, locals.var_vgbgmt_dn8, locals.var_vgbgmt_dn9,)
    }
};
        locals.var_vgbgmt = assign80250_e122814;
        locals.var_vgbgmt_dn2 = assign80250_e122814_d_n2;
        locals.var_vgbgmt_dn7 = assign80250_e122814_d_n7;
        locals.var_vgbgmt_dn8 = assign80250_e122814_d_n8;
        locals.var_vgbgmt_dn9 = assign80250_e122814_d_n9;

        let (assign80260_e122827, assign80260_e122827_d_n0, assign80260_e122827_d_n2, assign80260_e122827_d_n4, assign80260_e122827_d_n5, assign80260_e122827_d_n6, assign80260_e122827_d_n7, assign80260_e122827_d_n8, assign80260_e122827_d_n9, assign80260_e122827_d_n10, assign80260_e122827_d_n11, assign80260_e122827_d_n14,) = {
    if (((locals.var_guard1873 != 0.0) && (!((locals.var_guard1871 != 0.0) || (locals.var_guard1872 != 0.0)))) && (locals.var_guard1877 != 0.0)) {
        let assign80260_e122825: f64 = (locals.var_vdsi - locals.var_vbsi);
        (assign80260_e122825, 0.0, 0.0, 0.0, 0.0, locals.var_vdsi_dn6, 0.0, (locals.var_vdsi_dn8 - locals.var_vbsi_dn8), (-locals.var_vbsi_dn9), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn11, locals.var_vxbgmt_dn14,)
    }
};
        locals.var_vxbgmt = assign80260_e122827;
        locals.var_vxbgmt_dn0 = assign80260_e122827_d_n0;
        locals.var_vxbgmt_dn2 = assign80260_e122827_d_n2;
        locals.var_vxbgmt_dn4 = assign80260_e122827_d_n4;
        locals.var_vxbgmt_dn5 = assign80260_e122827_d_n5;
        locals.var_vxbgmt_dn6 = assign80260_e122827_d_n6;
        locals.var_vxbgmt_dn7 = assign80260_e122827_d_n7;
        locals.var_vxbgmt_dn8 = assign80260_e122827_d_n8;
        locals.var_vxbgmt_dn9 = assign80260_e122827_d_n9;
        locals.var_vxbgmt_dn10 = assign80260_e122827_d_n10;
        locals.var_vxbgmt_dn11 = assign80260_e122827_d_n11;
        locals.var_vxbgmt_dn14 = assign80260_e122827_d_n14;

        let (assign80270_e122838,) = {
    if (((locals.var_guard1873 != 0.0) && (!((locals.var_guard1871 != 0.0) || (locals.var_guard1872 != 0.0)))) && (locals.var_guard1877 != 0.0)) {
        (locals.var_uc_nover,)
    } else {
        (locals.var_nover_func,)
    }
};
        locals.var_nover_func = assign80270_e122838;

        let (assign80280_e122853, assign80280_e122853_d_n0, assign80280_e122853_d_n2, assign80280_e122853_d_n4, assign80280_e122853_d_n5, assign80280_e122853_d_n6, assign80280_e122853_d_n7, assign80280_e122853_d_n8, assign80280_e122853_d_n9, assign80280_e122853_d_n10, assign80280_e122853_d_n11, assign80280_e122853_d_n14,) = {
    if (((locals.var_guard1873 != 0.0) && (!((locals.var_guard1871 != 0.0) || (locals.var_guard1872 != 0.0)))) && (locals.var_guard1877 != 0.0)) {
        let assign80280_e122850: f64 = (p.p64 * p.p55);
        let assign80280_e122851: f64 = (p.p63 + assign80280_e122850);
        (assign80280_e122851, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_lover_func, locals.var_lover_func_dn0, locals.var_lover_func_dn2, locals.var_lover_func_dn4, locals.var_lover_func_dn5, locals.var_lover_func_dn6, locals.var_lover_func_dn7, locals.var_lover_func_dn8, locals.var_lover_func_dn9, locals.var_lover_func_dn10, locals.var_lover_func_dn11, locals.var_lover_func_dn14,)
    }
};
        locals.var_lover_func = assign80280_e122853;
        locals.var_lover_func_dn0 = assign80280_e122853_d_n0;
        locals.var_lover_func_dn2 = assign80280_e122853_d_n2;
        locals.var_lover_func_dn4 = assign80280_e122853_d_n4;
        locals.var_lover_func_dn5 = assign80280_e122853_d_n5;
        locals.var_lover_func_dn6 = assign80280_e122853_d_n6;
        locals.var_lover_func_dn7 = assign80280_e122853_d_n7;
        locals.var_lover_func_dn8 = assign80280_e122853_d_n8;
        locals.var_lover_func_dn9 = assign80280_e122853_d_n9;
        locals.var_lover_func_dn10 = assign80280_e122853_d_n10;
        locals.var_lover_func_dn11 = assign80280_e122853_d_n11;
        locals.var_lover_func_dn14 = assign80280_e122853_d_n14;

        let (assign80290_e122864, assign80290_e122864_d_n0, assign80290_e122864_d_n2, assign80290_e122864_d_n4, assign80290_e122864_d_n5, assign80290_e122864_d_n6, assign80290_e122864_d_n7, assign80290_e122864_d_n8, assign80290_e122864_d_n9, assign80290_e122864_d_n10, assign80290_e122864_d_n11, assign80290_e122864_d_n14,) = {
    if (((locals.var_guard1873 != 0.0) && (!((locals.var_guard1871 != 0.0) || (locals.var_guard1872 != 0.0)))) && (locals.var_guard1877 != 0.0)) {
        (locals.var_wdep, locals.var_wdep_dn0, locals.var_wdep_dn2, locals.var_wdep_dn4, locals.var_wdep_dn5, locals.var_wdep_dn6, locals.var_wdep_dn7, locals.var_wdep_dn8, locals.var_wdep_dn9, locals.var_wdep_dn10, locals.var_wdep_dn11, locals.var_wdep_dn14,)
    } else {
        (locals.var_wdep_func, locals.var_wdep_func_dn0, locals.var_wdep_func_dn2, locals.var_wdep_func_dn4, locals.var_wdep_func_dn5, locals.var_wdep_func_dn6, locals.var_wdep_func_dn7, locals.var_wdep_func_dn8, locals.var_wdep_func_dn9, locals.var_wdep_func_dn10, locals.var_wdep_func_dn11, locals.var_wdep_func_dn14,)
    }
};
        locals.var_wdep_func = assign80290_e122864;
        locals.var_wdep_func_dn0 = assign80290_e122864_d_n0;
        locals.var_wdep_func_dn2 = assign80290_e122864_d_n2;
        locals.var_wdep_func_dn4 = assign80290_e122864_d_n4;
        locals.var_wdep_func_dn5 = assign80290_e122864_d_n5;
        locals.var_wdep_func_dn6 = assign80290_e122864_d_n6;
        locals.var_wdep_func_dn7 = assign80290_e122864_d_n7;
        locals.var_wdep_func_dn8 = assign80290_e122864_d_n8;
        locals.var_wdep_func_dn9 = assign80290_e122864_d_n9;
        locals.var_wdep_func_dn10 = assign80290_e122864_d_n10;
        locals.var_wdep_func_dn11 = assign80290_e122864_d_n11;
        locals.var_wdep_func_dn14 = assign80290_e122864_d_n14;

        let (assign80300_e122875, assign80300_e122875_d_n0, assign80300_e122875_d_n2, assign80300_e122875_d_n4, assign80300_e122875_d_n5, assign80300_e122875_d_n6, assign80300_e122875_d_n7, assign80300_e122875_d_n8, assign80300_e122875_d_n9, assign80300_e122875_d_n10, assign80300_e122875_d_n11, assign80300_e122875_d_n14,) = {
    if (((locals.var_guard1873 != 0.0) && (!((locals.var_guard1871 != 0.0) || (locals.var_guard1872 != 0.0)))) && (locals.var_guard1877 != 0.0)) {
        (locals.var_cnst0over, locals.var_cnst0over_dn0, locals.var_cnst0over_dn2, locals.var_cnst0over_dn4, locals.var_cnst0over_dn5, locals.var_cnst0over_dn6, locals.var_cnst0over_dn7, locals.var_cnst0over_dn8, locals.var_cnst0over_dn9, locals.var_cnst0over_dn10, locals.var_cnst0over_dn11, locals.var_cnst0over_dn14,)
    } else {
        (locals.var_cnst0over_func, locals.var_cnst0over_func_dn0, locals.var_cnst0over_func_dn2, locals.var_cnst0over_func_dn4, locals.var_cnst0over_func_dn5, locals.var_cnst0over_func_dn6, locals.var_cnst0over_func_dn7, locals.var_cnst0over_func_dn8, locals.var_cnst0over_func_dn9, locals.var_cnst0over_func_dn10, locals.var_cnst0over_func_dn11, locals.var_cnst0over_func_dn14,)
    }
};
        locals.var_cnst0over_func = assign80300_e122875;
        locals.var_cnst0over_func_dn0 = assign80300_e122875_d_n0;
        locals.var_cnst0over_func_dn2 = assign80300_e122875_d_n2;
        locals.var_cnst0over_func_dn4 = assign80300_e122875_d_n4;
        locals.var_cnst0over_func_dn5 = assign80300_e122875_d_n5;
        locals.var_cnst0over_func_dn6 = assign80300_e122875_d_n6;
        locals.var_cnst0over_func_dn7 = assign80300_e122875_d_n7;
        locals.var_cnst0over_func_dn8 = assign80300_e122875_d_n8;
        locals.var_cnst0over_func_dn9 = assign80300_e122875_d_n9;
        locals.var_cnst0over_func_dn10 = assign80300_e122875_d_n10;
        locals.var_cnst0over_func_dn11 = assign80300_e122875_d_n11;
        locals.var_cnst0over_func_dn14 = assign80300_e122875_d_n14;

        let (assign80310_e122886,) = {
    if (((locals.var_guard1873 != 0.0) && (!((locals.var_guard1871 != 0.0) || (locals.var_guard1872 != 0.0)))) && (locals.var_guard1877 != 0.0)) {
        (locals.var_coxb0,)
    } else {
        (locals.var_cox0_func,)
    }
};
        locals.var_cox0_func = assign80310_e122886;

        let (assign80320_e122898, assign80320_e122898_d_n0, assign80320_e122898_d_n2, assign80320_e122898_d_n4, assign80320_e122898_d_n5, assign80320_e122898_d_n6, assign80320_e122898_d_n7, assign80320_e122898_d_n8, assign80320_e122898_d_n9, assign80320_e122898_d_n10, assign80320_e122898_d_n11, assign80320_e122898_d_n14,) = {
    if (((locals.var_guard1873 != 0.0) && (!((locals.var_guard1871 != 0.0) || (locals.var_guard1872 != 0.0)))) && (locals.var_guard1877 != 0.0)) {
        let assign80320_e122896: f64 = (-locals.var_lover_func);
        (assign80320_e122896, (-locals.var_lover_func_dn0), (-locals.var_lover_func_dn2), (-locals.var_lover_func_dn4), (-locals.var_lover_func_dn5), (-locals.var_lover_func_dn6), (-locals.var_lover_func_dn7), (-locals.var_lover_func_dn8), (-locals.var_lover_func_dn9), (-locals.var_lover_func_dn10), (-locals.var_lover_func_dn11), (-locals.var_lover_func_dn14),)
    } else {
        (locals.var_lover_func, locals.var_lover_func_dn0, locals.var_lover_func_dn2, locals.var_lover_func_dn4, locals.var_lover_func_dn5, locals.var_lover_func_dn6, locals.var_lover_func_dn7, locals.var_lover_func_dn8, locals.var_lover_func_dn9, locals.var_lover_func_dn10, locals.var_lover_func_dn11, locals.var_lover_func_dn14,)
    }
};
        locals.var_lover_func = assign80320_e122898;
        locals.var_lover_func_dn0 = assign80320_e122898_d_n0;
        locals.var_lover_func_dn2 = assign80320_e122898_d_n2;
        locals.var_lover_func_dn4 = assign80320_e122898_d_n4;
        locals.var_lover_func_dn5 = assign80320_e122898_d_n5;
        locals.var_lover_func_dn6 = assign80320_e122898_d_n6;
        locals.var_lover_func_dn7 = assign80320_e122898_d_n7;
        locals.var_lover_func_dn8 = assign80320_e122898_d_n8;
        locals.var_lover_func_dn9 = assign80320_e122898_d_n9;
        locals.var_lover_func_dn10 = assign80320_e122898_d_n10;
        locals.var_lover_func_dn11 = assign80320_e122898_d_n11;
        locals.var_lover_func_dn14 = assign80320_e122898_d_n14;

        let assign80330_e122909: f64 = if (((locals.var_lover_func < 0.0) && (p.p432 > 0.0)) && (p.p55 == 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard1878 = assign80330_e122909;

        let (assign80340_e122923, assign80340_e122923_d_n0, assign80340_e122923_d_n2, assign80340_e122923_d_n4, assign80340_e122923_d_n5, assign80340_e122923_d_n6, assign80340_e122923_d_n7, assign80340_e122923_d_n8, assign80340_e122923_d_n9, assign80340_e122923_d_n10, assign80340_e122923_d_n11, assign80340_e122923_d_n14,) = {
    if ((((locals.var_guard1873 != 0.0) && (!((locals.var_guard1871 != 0.0) || (locals.var_guard1872 != 0.0)))) && (locals.var_guard1877 != 0.0)) && (locals.var_guard1878 != 0.0)) {
        let assign80340_e122921: f64 = (-locals.var_lover_func);
        (assign80340_e122921, (-locals.var_lover_func_dn0), (-locals.var_lover_func_dn2), (-locals.var_lover_func_dn4), (-locals.var_lover_func_dn5), (-locals.var_lover_func_dn6), (-locals.var_lover_func_dn7), (-locals.var_lover_func_dn8), (-locals.var_lover_func_dn9), (-locals.var_lover_func_dn10), (-locals.var_lover_func_dn11), (-locals.var_lover_func_dn14),)
    } else {
        (locals.var_lover_func, locals.var_lover_func_dn0, locals.var_lover_func_dn2, locals.var_lover_func_dn4, locals.var_lover_func_dn5, locals.var_lover_func_dn6, locals.var_lover_func_dn7, locals.var_lover_func_dn8, locals.var_lover_func_dn9, locals.var_lover_func_dn10, locals.var_lover_func_dn11, locals.var_lover_func_dn14,)
    }
};
        locals.var_lover_func = assign80340_e122923;
        locals.var_lover_func_dn0 = assign80340_e122923_d_n0;
        locals.var_lover_func_dn2 = assign80340_e122923_d_n2;
        locals.var_lover_func_dn4 = assign80340_e122923_d_n4;
        locals.var_lover_func_dn5 = assign80340_e122923_d_n5;
        locals.var_lover_func_dn6 = assign80340_e122923_d_n6;
        locals.var_lover_func_dn7 = assign80340_e122923_d_n7;
        locals.var_lover_func_dn8 = assign80340_e122923_d_n8;
        locals.var_lover_func_dn9 = assign80340_e122923_d_n9;
        locals.var_lover_func_dn10 = assign80340_e122923_d_n10;
        locals.var_lover_func_dn11 = assign80340_e122923_d_n11;
        locals.var_lover_func_dn14 = assign80340_e122923_d_n14;

        let (assign80350_e122936, assign80350_e122936_d_n0, assign80350_e122936_d_n2, assign80350_e122936_d_n4, assign80350_e122936_d_n5, assign80350_e122936_d_n6, assign80350_e122936_d_n7, assign80350_e122936_d_n8, assign80350_e122936_d_n9, assign80350_e122936_d_n10, assign80350_e122936_d_n11, assign80350_e122936_d_n14,) = {
    if ((((locals.var_guard1873 != 0.0) && (!((locals.var_guard1871 != 0.0) || (locals.var_guard1872 != 0.0)))) && (locals.var_guard1877 != 0.0)) && (locals.var_guard1878 != 0.0)) {
        (p.p63, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign80350_e122936;
        locals.var_t1_dn0 = assign80350_e122936_d_n0;
        locals.var_t1_dn2 = assign80350_e122936_d_n2;
        locals.var_t1_dn4 = assign80350_e122936_d_n4;
        locals.var_t1_dn5 = assign80350_e122936_d_n5;
        locals.var_t1_dn6 = assign80350_e122936_d_n6;
        locals.var_t1_dn7 = assign80350_e122936_d_n7;
        locals.var_t1_dn8 = assign80350_e122936_d_n8;
        locals.var_t1_dn9 = assign80350_e122936_d_n9;
        locals.var_t1_dn10 = assign80350_e122936_d_n10;
        locals.var_t1_dn11 = assign80350_e122936_d_n11;
        locals.var_t1_dn14 = assign80350_e122936_d_n14;

        let (assign80360_e122955, assign80360_e122955_d_n0, assign80360_e122955_d_n2, assign80360_e122955_d_n4, assign80360_e122955_d_n5, assign80360_e122955_d_n6, assign80360_e122955_d_n7, assign80360_e122955_d_n8, assign80360_e122955_d_n9, assign80360_e122955_d_n10, assign80360_e122955_d_n11, assign80360_e122955_d_n14,) = {
    if ((((locals.var_guard1873 != 0.0) && (!((locals.var_guard1871 != 0.0) || (locals.var_guard1872 != 0.0)))) && (locals.var_guard1877 != 0.0)) && (locals.var_guard1878 != 0.0)) {
        let assign80360_e122949: f64 = (locals.var_t1 * locals.var_t1);
        let assign80360_e122951: f64 = (assign80360_e122949 / locals.var_kjunc);
        let assign80360_e122953: f64 = (assign80360_e122951 - p.p137);
        (assign80360_e122953, (((((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) * locals.var_kjunc) - (assign80360_e122949 * locals.var_kjunc_dn0)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) * locals.var_kjunc) - (assign80360_e122949 * locals.var_kjunc_dn2)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) * locals.var_kjunc) - (assign80360_e122949 * locals.var_kjunc_dn4)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) * locals.var_kjunc) - (assign80360_e122949 * locals.var_kjunc_dn5)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) * locals.var_kjunc) - (assign80360_e122949 * locals.var_kjunc_dn6)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) * locals.var_kjunc) - (assign80360_e122949 * locals.var_kjunc_dn7)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) * locals.var_kjunc) - (assign80360_e122949 * locals.var_kjunc_dn8)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) * locals.var_kjunc) - (assign80360_e122949 * locals.var_kjunc_dn9)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) * locals.var_kjunc) - (assign80360_e122949 * locals.var_kjunc_dn10)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) * locals.var_kjunc) - (assign80360_e122949 * locals.var_kjunc_dn11)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14)) * locals.var_kjunc) - (assign80360_e122949 * locals.var_kjunc_dn14)) / (locals.var_kjunc * locals.var_kjunc)),)
    } else {
        (locals.var_vxb_lim, locals.var_vxb_lim_dn0, locals.var_vxb_lim_dn2, locals.var_vxb_lim_dn4, locals.var_vxb_lim_dn5, locals.var_vxb_lim_dn6, locals.var_vxb_lim_dn7, locals.var_vxb_lim_dn8, locals.var_vxb_lim_dn9, locals.var_vxb_lim_dn10, locals.var_vxb_lim_dn11, locals.var_vxb_lim_dn14,)
    }
};
        locals.var_vxb_lim = assign80360_e122955;
        locals.var_vxb_lim_dn0 = assign80360_e122955_d_n0;
        locals.var_vxb_lim_dn2 = assign80360_e122955_d_n2;
        locals.var_vxb_lim_dn4 = assign80360_e122955_d_n4;
        locals.var_vxb_lim_dn5 = assign80360_e122955_d_n5;
        locals.var_vxb_lim_dn6 = assign80360_e122955_d_n6;
        locals.var_vxb_lim_dn7 = assign80360_e122955_d_n7;
        locals.var_vxb_lim_dn8 = assign80360_e122955_d_n8;
        locals.var_vxb_lim_dn9 = assign80360_e122955_d_n9;
        locals.var_vxb_lim_dn10 = assign80360_e122955_d_n10;
        locals.var_vxb_lim_dn11 = assign80360_e122955_d_n11;
        locals.var_vxb_lim_dn14 = assign80360_e122955_d_n14;

        let assign80370_e122958: f64 = if p.p113 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1879 = assign80370_e122958;

        let assign80380_e122965: f64 = if ((locals.var_vxbgmt == 0.0) || (p.p113 <= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1880 = assign80380_e122965;

        let (assign80390_e122982, assign80390_e122982_d_n0, assign80390_e122982_d_n2, assign80390_e122982_d_n4, assign80390_e122982_d_n5, assign80390_e122982_d_n6, assign80390_e122982_d_n7, assign80390_e122982_d_n8, assign80390_e122982_d_n9, assign80390_e122982_d_n10, assign80390_e122982_d_n11, assign80390_e122982_d_n14,) = {
    if ((((((locals.var_guard1873 != 0.0) && (!((locals.var_guard1871 != 0.0) || (locals.var_guard1872 != 0.0)))) && (locals.var_guard1877 != 0.0)) && (locals.var_guard1878 != 0.0)) && (locals.var_guard1879 != 0.0)) && (locals.var_guard1880 != 0.0)) {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn11, locals.var_vxbgmt_dn14,)
    } else {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn11, locals.var_vxbgmt_dn14,)
    }
};
        locals.var_vxbgmt = assign80390_e122982;
        locals.var_vxbgmt_dn0 = assign80390_e122982_d_n0;
        locals.var_vxbgmt_dn2 = assign80390_e122982_d_n2;
        locals.var_vxbgmt_dn4 = assign80390_e122982_d_n4;
        locals.var_vxbgmt_dn5 = assign80390_e122982_d_n5;
        locals.var_vxbgmt_dn6 = assign80390_e122982_d_n6;
        locals.var_vxbgmt_dn7 = assign80390_e122982_d_n7;
        locals.var_vxbgmt_dn8 = assign80390_e122982_d_n8;
        locals.var_vxbgmt_dn9 = assign80390_e122982_d_n9;
        locals.var_vxbgmt_dn10 = assign80390_e122982_d_n10;
        locals.var_vxbgmt_dn11 = assign80390_e122982_d_n11;
        locals.var_vxbgmt_dn14 = assign80390_e122982_d_n14;

        let (assign80400_e123006, assign80400_e123006_d_n0, assign80400_e123006_d_n2, assign80400_e123006_d_n4, assign80400_e123006_d_n5, assign80400_e123006_d_n6, assign80400_e123006_d_n7, assign80400_e123006_d_n8, assign80400_e123006_d_n9, assign80400_e123006_d_n10, assign80400_e123006_d_n11, assign80400_e123006_d_n14,) = {
    if ((((((locals.var_guard1873 != 0.0) && (!((locals.var_guard1871 != 0.0) || (locals.var_guard1872 != 0.0)))) && (locals.var_guard1877 != 0.0)) && (locals.var_guard1878 != 0.0)) && (locals.var_guard1879 != 0.0)) && (locals.var_guard1880 == 0.0)) {
        let (assign80400_e123004,) = {
            if (locals.var_vxbgmt < 0.0) {
                let assign80400_e123002: f64 = (-1.0);
                (assign80400_e123002,)
            } else {
                (1.0,)
            }
        };
        (assign80400_e123004, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf3, locals.var_tmf3_dn0, locals.var_tmf3_dn2, locals.var_tmf3_dn4, locals.var_tmf3_dn5, locals.var_tmf3_dn6, locals.var_tmf3_dn7, locals.var_tmf3_dn8, locals.var_tmf3_dn9, locals.var_tmf3_dn10, locals.var_tmf3_dn11, locals.var_tmf3_dn14,)
    }
};
        locals.var_tmf3 = assign80400_e123006;
        locals.var_tmf3_dn0 = assign80400_e123006_d_n0;
        locals.var_tmf3_dn2 = assign80400_e123006_d_n2;
        locals.var_tmf3_dn4 = assign80400_e123006_d_n4;
        locals.var_tmf3_dn5 = assign80400_e123006_d_n5;
        locals.var_tmf3_dn6 = assign80400_e123006_d_n6;
        locals.var_tmf3_dn7 = assign80400_e123006_d_n7;
        locals.var_tmf3_dn8 = assign80400_e123006_d_n8;
        locals.var_tmf3_dn9 = assign80400_e123006_d_n9;
        locals.var_tmf3_dn10 = assign80400_e123006_d_n10;
        locals.var_tmf3_dn11 = assign80400_e123006_d_n11;
        locals.var_tmf3_dn14 = assign80400_e123006_d_n14;

        let (assign80410_e123026, assign80410_e123026_d_n0, assign80410_e123026_d_n2, assign80410_e123026_d_n4, assign80410_e123026_d_n5, assign80410_e123026_d_n6, assign80410_e123026_d_n7, assign80410_e123026_d_n8, assign80410_e123026_d_n9, assign80410_e123026_d_n10, assign80410_e123026_d_n11, assign80410_e123026_d_n14,) = {
    if ((((((locals.var_guard1873 != 0.0) && (!((locals.var_guard1871 != 0.0) || (locals.var_guard1872 != 0.0)))) && (locals.var_guard1877 != 0.0)) && (locals.var_guard1878 != 0.0)) && (locals.var_guard1879 != 0.0)) && (locals.var_guard1880 == 0.0)) {
        let assign80410_e123024: f64 = (locals.var_tmf3 * locals.var_vxbgmt);
        (assign80410_e123024, ((locals.var_tmf3_dn0 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn0)), ((locals.var_tmf3_dn2 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn2)), ((locals.var_tmf3_dn4 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn4)), ((locals.var_tmf3_dn5 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn5)), ((locals.var_tmf3_dn6 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn6)), ((locals.var_tmf3_dn7 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn7)), ((locals.var_tmf3_dn8 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn8)), ((locals.var_tmf3_dn9 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn9)), ((locals.var_tmf3_dn10 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn10)), ((locals.var_tmf3_dn11 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn11)), ((locals.var_tmf3_dn14 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn14)),)
    } else {
        (locals.var_tmf4, locals.var_tmf4_dn0, locals.var_tmf4_dn2, locals.var_tmf4_dn4, locals.var_tmf4_dn5, locals.var_tmf4_dn6, locals.var_tmf4_dn7, locals.var_tmf4_dn8, locals.var_tmf4_dn9, locals.var_tmf4_dn10, locals.var_tmf4_dn11, locals.var_tmf4_dn14,)
    }
};
        locals.var_tmf4 = assign80410_e123026;
        locals.var_tmf4_dn0 = assign80410_e123026_d_n0;
        locals.var_tmf4_dn2 = assign80410_e123026_d_n2;
        locals.var_tmf4_dn4 = assign80410_e123026_d_n4;
        locals.var_tmf4_dn5 = assign80410_e123026_d_n5;
        locals.var_tmf4_dn6 = assign80410_e123026_d_n6;
        locals.var_tmf4_dn7 = assign80410_e123026_d_n7;
        locals.var_tmf4_dn8 = assign80410_e123026_d_n8;
        locals.var_tmf4_dn9 = assign80410_e123026_d_n9;
        locals.var_tmf4_dn10 = assign80410_e123026_d_n10;
        locals.var_tmf4_dn11 = assign80410_e123026_d_n11;
        locals.var_tmf4_dn14 = assign80410_e123026_d_n14;

        let (assign80420_e123050, assign80420_e123050_d_n0, assign80420_e123050_d_n2, assign80420_e123050_d_n4, assign80420_e123050_d_n5, assign80420_e123050_d_n6, assign80420_e123050_d_n7, assign80420_e123050_d_n8, assign80420_e123050_d_n9, assign80420_e123050_d_n10, assign80420_e123050_d_n11, assign80420_e123050_d_n14,) = {
    if ((((((locals.var_guard1873 != 0.0) && (!((locals.var_guard1871 != 0.0) || (locals.var_guard1872 != 0.0)))) && (locals.var_guard1877 != 0.0)) && (locals.var_guard1878 != 0.0)) && (locals.var_guard1879 != 0.0)) && (locals.var_guard1880 == 0.0)) {
        let assign80420_e123045: f64 = (locals.var_tmf4 / locals.var_vxb_lim);
        let assign80420_e123047: f64 = (assign80420_e123045).powf(p.p113);
        let assign80420_e123048: f64 = (1.0 + assign80420_e123047);
        (assign80420_e123048, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign80420_e123045).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn0 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn0)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign80420_e123047 * (p.p113 * ((((locals.var_tmf4_dn0 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn0)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign80420_e123045))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign80420_e123045).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn2 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn2)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign80420_e123047 * (p.p113 * ((((locals.var_tmf4_dn2 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn2)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign80420_e123045))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign80420_e123045).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn4 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn4)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign80420_e123047 * (p.p113 * ((((locals.var_tmf4_dn4 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn4)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign80420_e123045))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign80420_e123045).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn5 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn5)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign80420_e123047 * (p.p113 * ((((locals.var_tmf4_dn5 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn5)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign80420_e123045))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign80420_e123045).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn6 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn6)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign80420_e123047 * (p.p113 * ((((locals.var_tmf4_dn6 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn6)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign80420_e123045))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign80420_e123045).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn7 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn7)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign80420_e123047 * (p.p113 * ((((locals.var_tmf4_dn7 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn7)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign80420_e123045))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign80420_e123045).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn8 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn8)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign80420_e123047 * (p.p113 * ((((locals.var_tmf4_dn8 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn8)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign80420_e123045))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign80420_e123045).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn9 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn9)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign80420_e123047 * (p.p113 * ((((locals.var_tmf4_dn9 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn9)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign80420_e123045))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign80420_e123045).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn10 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn10)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign80420_e123047 * (p.p113 * ((((locals.var_tmf4_dn10 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn10)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign80420_e123045))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign80420_e123045).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn11 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn11)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign80420_e123047 * (p.p113 * ((((locals.var_tmf4_dn11 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn11)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign80420_e123045))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign80420_e123045).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn14 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn14)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign80420_e123047 * (p.p113 * ((((locals.var_tmf4_dn14 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn14)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign80420_e123045))) },)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign80420_e123050;
        locals.var_tmf1_dn0 = assign80420_e123050_d_n0;
        locals.var_tmf1_dn2 = assign80420_e123050_d_n2;
        locals.var_tmf1_dn4 = assign80420_e123050_d_n4;
        locals.var_tmf1_dn5 = assign80420_e123050_d_n5;
        locals.var_tmf1_dn6 = assign80420_e123050_d_n6;
        locals.var_tmf1_dn7 = assign80420_e123050_d_n7;
        locals.var_tmf1_dn8 = assign80420_e123050_d_n8;
        locals.var_tmf1_dn9 = assign80420_e123050_d_n9;
        locals.var_tmf1_dn10 = assign80420_e123050_d_n10;
        locals.var_tmf1_dn11 = assign80420_e123050_d_n11;
        locals.var_tmf1_dn14 = assign80420_e123050_d_n14;

        let (assign80430_e123072, assign80430_e123072_d_n0, assign80430_e123072_d_n2, assign80430_e123072_d_n4, assign80430_e123072_d_n5, assign80430_e123072_d_n6, assign80430_e123072_d_n7, assign80430_e123072_d_n8, assign80430_e123072_d_n9, assign80430_e123072_d_n10, assign80430_e123072_d_n11, assign80430_e123072_d_n14,) = {
    if ((((((locals.var_guard1873 != 0.0) && (!((locals.var_guard1871 != 0.0) || (locals.var_guard1872 != 0.0)))) && (locals.var_guard1877 != 0.0)) && (locals.var_guard1878 != 0.0)) && (locals.var_guard1879 != 0.0)) && (locals.var_guard1880 == 0.0)) {
        let assign80430_e123069: f64 = (1.0 / p.p113);
        let assign80430_e123070: f64 = (locals.var_tmf1).powf(assign80430_e123069);
        (assign80430_e123070, if 0.0 == 0.0 && ((assign80430_e123069) as f64).is_finite() && ((assign80430_e123069) as f64).fract() == 0.0 { if assign80430_e123069 == 0.0 { 0.0 } else { (assign80430_e123069 * ((locals.var_tmf1).powf(assign80430_e123069 - 1.0) * locals.var_tmf1_dn0)) } } else { (assign80430_e123070 * (assign80430_e123069 * (locals.var_tmf1_dn0 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign80430_e123069) as f64).is_finite() && ((assign80430_e123069) as f64).fract() == 0.0 { if assign80430_e123069 == 0.0 { 0.0 } else { (assign80430_e123069 * ((locals.var_tmf1).powf(assign80430_e123069 - 1.0) * locals.var_tmf1_dn2)) } } else { (assign80430_e123070 * (assign80430_e123069 * (locals.var_tmf1_dn2 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign80430_e123069) as f64).is_finite() && ((assign80430_e123069) as f64).fract() == 0.0 { if assign80430_e123069 == 0.0 { 0.0 } else { (assign80430_e123069 * ((locals.var_tmf1).powf(assign80430_e123069 - 1.0) * locals.var_tmf1_dn4)) } } else { (assign80430_e123070 * (assign80430_e123069 * (locals.var_tmf1_dn4 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign80430_e123069) as f64).is_finite() && ((assign80430_e123069) as f64).fract() == 0.0 { if assign80430_e123069 == 0.0 { 0.0 } else { (assign80430_e123069 * ((locals.var_tmf1).powf(assign80430_e123069 - 1.0) * locals.var_tmf1_dn5)) } } else { (assign80430_e123070 * (assign80430_e123069 * (locals.var_tmf1_dn5 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign80430_e123069) as f64).is_finite() && ((assign80430_e123069) as f64).fract() == 0.0 { if assign80430_e123069 == 0.0 { 0.0 } else { (assign80430_e123069 * ((locals.var_tmf1).powf(assign80430_e123069 - 1.0) * locals.var_tmf1_dn6)) } } else { (assign80430_e123070 * (assign80430_e123069 * (locals.var_tmf1_dn6 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign80430_e123069) as f64).is_finite() && ((assign80430_e123069) as f64).fract() == 0.0 { if assign80430_e123069 == 0.0 { 0.0 } else { (assign80430_e123069 * ((locals.var_tmf1).powf(assign80430_e123069 - 1.0) * locals.var_tmf1_dn7)) } } else { (assign80430_e123070 * (assign80430_e123069 * (locals.var_tmf1_dn7 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign80430_e123069) as f64).is_finite() && ((assign80430_e123069) as f64).fract() == 0.0 { if assign80430_e123069 == 0.0 { 0.0 } else { (assign80430_e123069 * ((locals.var_tmf1).powf(assign80430_e123069 - 1.0) * locals.var_tmf1_dn8)) } } else { (assign80430_e123070 * (assign80430_e123069 * (locals.var_tmf1_dn8 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign80430_e123069) as f64).is_finite() && ((assign80430_e123069) as f64).fract() == 0.0 { if assign80430_e123069 == 0.0 { 0.0 } else { (assign80430_e123069 * ((locals.var_tmf1).powf(assign80430_e123069 - 1.0) * locals.var_tmf1_dn9)) } } else { (assign80430_e123070 * (assign80430_e123069 * (locals.var_tmf1_dn9 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign80430_e123069) as f64).is_finite() && ((assign80430_e123069) as f64).fract() == 0.0 { if assign80430_e123069 == 0.0 { 0.0 } else { (assign80430_e123069 * ((locals.var_tmf1).powf(assign80430_e123069 - 1.0) * locals.var_tmf1_dn10)) } } else { (assign80430_e123070 * (assign80430_e123069 * (locals.var_tmf1_dn10 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign80430_e123069) as f64).is_finite() && ((assign80430_e123069) as f64).fract() == 0.0 { if assign80430_e123069 == 0.0 { 0.0 } else { (assign80430_e123069 * ((locals.var_tmf1).powf(assign80430_e123069 - 1.0) * locals.var_tmf1_dn11)) } } else { (assign80430_e123070 * (assign80430_e123069 * (locals.var_tmf1_dn11 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign80430_e123069) as f64).is_finite() && ((assign80430_e123069) as f64).fract() == 0.0 { if assign80430_e123069 == 0.0 { 0.0 } else { (assign80430_e123069 * ((locals.var_tmf1).powf(assign80430_e123069 - 1.0) * locals.var_tmf1_dn14)) } } else { (assign80430_e123070 * (assign80430_e123069 * (locals.var_tmf1_dn14 / locals.var_tmf1))) },)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign80430_e123072;
        locals.var_tmf2_dn0 = assign80430_e123072_d_n0;
        locals.var_tmf2_dn2 = assign80430_e123072_d_n2;
        locals.var_tmf2_dn4 = assign80430_e123072_d_n4;
        locals.var_tmf2_dn5 = assign80430_e123072_d_n5;
        locals.var_tmf2_dn6 = assign80430_e123072_d_n6;
        locals.var_tmf2_dn7 = assign80430_e123072_d_n7;
        locals.var_tmf2_dn8 = assign80430_e123072_d_n8;
        locals.var_tmf2_dn9 = assign80430_e123072_d_n9;
        locals.var_tmf2_dn10 = assign80430_e123072_d_n10;
        locals.var_tmf2_dn11 = assign80430_e123072_d_n11;
        locals.var_tmf2_dn14 = assign80430_e123072_d_n14;

    }

    pub(super) fn stamp_transient_block_291(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign80440_e123094, assign80440_e123094_d_n0, assign80440_e123094_d_n2, assign80440_e123094_d_n4, assign80440_e123094_d_n5, assign80440_e123094_d_n6, assign80440_e123094_d_n7, assign80440_e123094_d_n8, assign80440_e123094_d_n9, assign80440_e123094_d_n10, assign80440_e123094_d_n11, assign80440_e123094_d_n14,) = {
    if ((((((locals.var_guard1873 != 0.0) && (!((locals.var_guard1871 != 0.0) || (locals.var_guard1872 != 0.0)))) && (locals.var_guard1877 != 0.0)) && (locals.var_guard1878 != 0.0)) && (locals.var_guard1879 != 0.0)) && (locals.var_guard1880 == 0.0)) {
        let assign80440_e123090: f64 = (locals.var_tmf3 * locals.var_tmf4);
        let assign80440_e123092: f64 = (assign80440_e123090 / locals.var_tmf2);
        (assign80440_e123092, (((((locals.var_tmf3_dn0 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn0)) * locals.var_tmf2) - (assign80440_e123090 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn2 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn2)) * locals.var_tmf2) - (assign80440_e123090 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn4 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn4)) * locals.var_tmf2) - (assign80440_e123090 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn5 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn5)) * locals.var_tmf2) - (assign80440_e123090 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn6 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn6)) * locals.var_tmf2) - (assign80440_e123090 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn7 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn7)) * locals.var_tmf2) - (assign80440_e123090 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn8 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn8)) * locals.var_tmf2) - (assign80440_e123090 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn9 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn9)) * locals.var_tmf2) - (assign80440_e123090 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn10 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn10)) * locals.var_tmf2) - (assign80440_e123090 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn11 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn11)) * locals.var_tmf2) - (assign80440_e123090 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn14 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn14)) * locals.var_tmf2) - (assign80440_e123090 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2)),)
    } else {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn11, locals.var_vxbgmt_dn14,)
    }
};
        locals.var_vxbgmt = assign80440_e123094;
        locals.var_vxbgmt_dn0 = assign80440_e123094_d_n0;
        locals.var_vxbgmt_dn2 = assign80440_e123094_d_n2;
        locals.var_vxbgmt_dn4 = assign80440_e123094_d_n4;
        locals.var_vxbgmt_dn5 = assign80440_e123094_d_n5;
        locals.var_vxbgmt_dn6 = assign80440_e123094_d_n6;
        locals.var_vxbgmt_dn7 = assign80440_e123094_d_n7;
        locals.var_vxbgmt_dn8 = assign80440_e123094_d_n8;
        locals.var_vxbgmt_dn9 = assign80440_e123094_d_n9;
        locals.var_vxbgmt_dn10 = assign80440_e123094_d_n10;
        locals.var_vxbgmt_dn11 = assign80440_e123094_d_n11;
        locals.var_vxbgmt_dn14 = assign80440_e123094_d_n14;

        let (assign80450_e123122, assign80450_e123122_d_n0, assign80450_e123122_d_n2, assign80450_e123122_d_n4, assign80450_e123122_d_n5, assign80450_e123122_d_n6, assign80450_e123122_d_n7, assign80450_e123122_d_n8, assign80450_e123122_d_n9, assign80450_e123122_d_n10, assign80450_e123122_d_n11, assign80450_e123122_d_n14,) = {
    if (((((locals.var_guard1873 != 0.0) && (!((locals.var_guard1871 != 0.0) || (locals.var_guard1872 != 0.0)))) && (locals.var_guard1877 != 0.0)) && (locals.var_guard1878 != 0.0)) && (locals.var_guard1879 != 0.0)) {
        let assign80450_e123109: f64 = (locals.var_vxbgmt + p.p137);
        let assign80450_e123112: f64 = (locals.var_vxbgmt + p.p137);
        let assign80450_e123113: f64 = (assign80450_e123109 * assign80450_e123112);
        let assign80450_e123116: f64 = (4.0 * 0.1);
        let assign80450_e123118: f64 = (assign80450_e123116 * 0.1);
        let assign80450_e123119: f64 = (assign80450_e123113 + assign80450_e123118);
        let assign80450_e123120: f64 = (assign80450_e123119).sqrt();
        (assign80450_e123120, (((locals.var_vxbgmt_dn0 * assign80450_e123112) + (assign80450_e123109 * locals.var_vxbgmt_dn0)) / (2.0 * assign80450_e123120)), (((locals.var_vxbgmt_dn2 * assign80450_e123112) + (assign80450_e123109 * locals.var_vxbgmt_dn2)) / (2.0 * assign80450_e123120)), (((locals.var_vxbgmt_dn4 * assign80450_e123112) + (assign80450_e123109 * locals.var_vxbgmt_dn4)) / (2.0 * assign80450_e123120)), (((locals.var_vxbgmt_dn5 * assign80450_e123112) + (assign80450_e123109 * locals.var_vxbgmt_dn5)) / (2.0 * assign80450_e123120)), (((locals.var_vxbgmt_dn6 * assign80450_e123112) + (assign80450_e123109 * locals.var_vxbgmt_dn6)) / (2.0 * assign80450_e123120)), (((locals.var_vxbgmt_dn7 * assign80450_e123112) + (assign80450_e123109 * locals.var_vxbgmt_dn7)) / (2.0 * assign80450_e123120)), (((locals.var_vxbgmt_dn8 * assign80450_e123112) + (assign80450_e123109 * locals.var_vxbgmt_dn8)) / (2.0 * assign80450_e123120)), (((locals.var_vxbgmt_dn9 * assign80450_e123112) + (assign80450_e123109 * locals.var_vxbgmt_dn9)) / (2.0 * assign80450_e123120)), (((locals.var_vxbgmt_dn10 * assign80450_e123112) + (assign80450_e123109 * locals.var_vxbgmt_dn10)) / (2.0 * assign80450_e123120)), (((locals.var_vxbgmt_dn11 * assign80450_e123112) + (assign80450_e123109 * locals.var_vxbgmt_dn11)) / (2.0 * assign80450_e123120)), (((locals.var_vxbgmt_dn14 * assign80450_e123112) + (assign80450_e123109 * locals.var_vxbgmt_dn14)) / (2.0 * assign80450_e123120)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign80450_e123122;
        locals.var_tmf2_dn0 = assign80450_e123122_d_n0;
        locals.var_tmf2_dn2 = assign80450_e123122_d_n2;
        locals.var_tmf2_dn4 = assign80450_e123122_d_n4;
        locals.var_tmf2_dn5 = assign80450_e123122_d_n5;
        locals.var_tmf2_dn6 = assign80450_e123122_d_n6;
        locals.var_tmf2_dn7 = assign80450_e123122_d_n7;
        locals.var_tmf2_dn8 = assign80450_e123122_d_n8;
        locals.var_tmf2_dn9 = assign80450_e123122_d_n9;
        locals.var_tmf2_dn10 = assign80450_e123122_d_n10;
        locals.var_tmf2_dn11 = assign80450_e123122_d_n11;
        locals.var_tmf2_dn14 = assign80450_e123122_d_n14;

        let (assign80460_e123145, assign80460_e123145_d_n0, assign80460_e123145_d_n2, assign80460_e123145_d_n4, assign80460_e123145_d_n5, assign80460_e123145_d_n6, assign80460_e123145_d_n7, assign80460_e123145_d_n8, assign80460_e123145_d_n9, assign80460_e123145_d_n10, assign80460_e123145_d_n11, assign80460_e123145_d_n14,) = {
    if (((((locals.var_guard1873 != 0.0) && (!((locals.var_guard1871 != 0.0) || (locals.var_guard1872 != 0.0)))) && (locals.var_guard1877 != 0.0)) && (locals.var_guard1878 != 0.0)) && (locals.var_guard1879 != 0.0)) {
        let assign80460_e123139: f64 = (locals.var_vxbgmt + p.p137);
        let assign80460_e123141: f64 = (assign80460_e123139 / locals.var_tmf2);
        let assign80460_e123142: f64 = (1.0 + assign80460_e123141);
        let assign80460_e123143: f64 = (0.5 * assign80460_e123142);
        (assign80460_e123143, (0.5 * (((locals.var_vxbgmt_dn0 * locals.var_tmf2) - (assign80460_e123139 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn2 * locals.var_tmf2) - (assign80460_e123139 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn4 * locals.var_tmf2) - (assign80460_e123139 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn5 * locals.var_tmf2) - (assign80460_e123139 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn6 * locals.var_tmf2) - (assign80460_e123139 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn7 * locals.var_tmf2) - (assign80460_e123139 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn8 * locals.var_tmf2) - (assign80460_e123139 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn9 * locals.var_tmf2) - (assign80460_e123139 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn10 * locals.var_tmf2) - (assign80460_e123139 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn11 * locals.var_tmf2) - (assign80460_e123139 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn14 * locals.var_tmf2) - (assign80460_e123139 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign80460_e123145;
        locals.var_t9_dn0 = assign80460_e123145_d_n0;
        locals.var_t9_dn2 = assign80460_e123145_d_n2;
        locals.var_t9_dn4 = assign80460_e123145_d_n4;
        locals.var_t9_dn5 = assign80460_e123145_d_n5;
        locals.var_t9_dn6 = assign80460_e123145_d_n6;
        locals.var_t9_dn7 = assign80460_e123145_d_n7;
        locals.var_t9_dn8 = assign80460_e123145_d_n8;
        locals.var_t9_dn9 = assign80460_e123145_d_n9;
        locals.var_t9_dn10 = assign80460_e123145_d_n10;
        locals.var_t9_dn11 = assign80460_e123145_d_n11;
        locals.var_t9_dn14 = assign80460_e123145_d_n14;

        let (assign80470_e123166, assign80470_e123166_d_n0, assign80470_e123166_d_n2, assign80470_e123166_d_n4, assign80470_e123166_d_n5, assign80470_e123166_d_n6, assign80470_e123166_d_n7, assign80470_e123166_d_n8, assign80470_e123166_d_n9, assign80470_e123166_d_n10, assign80470_e123166_d_n11, assign80470_e123166_d_n14,) = {
    if (((((locals.var_guard1873 != 0.0) && (!((locals.var_guard1871 != 0.0) || (locals.var_guard1872 != 0.0)))) && (locals.var_guard1877 != 0.0)) && (locals.var_guard1878 != 0.0)) && (locals.var_guard1879 != 0.0)) {
        let assign80470_e123161: f64 = (locals.var_vxbgmt + p.p137);
        let assign80470_e123163: f64 = (assign80470_e123161 + locals.var_tmf2);
        let assign80470_e123164: f64 = (0.5 * assign80470_e123163);
        (assign80470_e123164, (0.5 * (locals.var_vxbgmt_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_vxbgmt_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_vxbgmt_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_vxbgmt_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_vxbgmt_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_vxbgmt_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_vxbgmt_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_vxbgmt_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_vxbgmt_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_vxbgmt_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_vxbgmt_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign80470_e123166;
        locals.var_t2_dn0 = assign80470_e123166_d_n0;
        locals.var_t2_dn2 = assign80470_e123166_d_n2;
        locals.var_t2_dn4 = assign80470_e123166_d_n4;
        locals.var_t2_dn5 = assign80470_e123166_d_n5;
        locals.var_t2_dn6 = assign80470_e123166_d_n6;
        locals.var_t2_dn7 = assign80470_e123166_d_n7;
        locals.var_t2_dn8 = assign80470_e123166_d_n8;
        locals.var_t2_dn9 = assign80470_e123166_d_n9;
        locals.var_t2_dn10 = assign80470_e123166_d_n10;
        locals.var_t2_dn11 = assign80470_e123166_d_n11;
        locals.var_t2_dn14 = assign80470_e123166_d_n14;

        let assign80480_e123169: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1881 = assign80480_e123169;

        let (assign80490_e123186, assign80490_e123186_d_n0, assign80490_e123186_d_n2, assign80490_e123186_d_n4, assign80490_e123186_d_n5, assign80490_e123186_d_n6, assign80490_e123186_d_n7, assign80490_e123186_d_n8, assign80490_e123186_d_n9, assign80490_e123186_d_n10, assign80490_e123186_d_n11, assign80490_e123186_d_n14,) = {
    if ((((((locals.var_guard1873 != 0.0) && (!((locals.var_guard1871 != 0.0) || (locals.var_guard1872 != 0.0)))) && (locals.var_guard1877 != 0.0)) && (locals.var_guard1878 != 0.0)) && (locals.var_guard1879 != 0.0)) && (locals.var_guard1881 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign80490_e123186;
        locals.var_t2_dn0 = assign80490_e123186_d_n0;
        locals.var_t2_dn2 = assign80490_e123186_d_n2;
        locals.var_t2_dn4 = assign80490_e123186_d_n4;
        locals.var_t2_dn5 = assign80490_e123186_d_n5;
        locals.var_t2_dn6 = assign80490_e123186_d_n6;
        locals.var_t2_dn7 = assign80490_e123186_d_n7;
        locals.var_t2_dn8 = assign80490_e123186_d_n8;
        locals.var_t2_dn9 = assign80490_e123186_d_n9;
        locals.var_t2_dn10 = assign80490_e123186_d_n10;
        locals.var_t2_dn11 = assign80490_e123186_d_n11;
        locals.var_t2_dn14 = assign80490_e123186_d_n14;

        let (assign80500_e123203, assign80500_e123203_d_n0, assign80500_e123203_d_n2, assign80500_e123203_d_n4, assign80500_e123203_d_n5, assign80500_e123203_d_n6, assign80500_e123203_d_n7, assign80500_e123203_d_n8, assign80500_e123203_d_n9, assign80500_e123203_d_n10, assign80500_e123203_d_n11, assign80500_e123203_d_n14,) = {
    if ((((((locals.var_guard1873 != 0.0) && (!((locals.var_guard1871 != 0.0) || (locals.var_guard1872 != 0.0)))) && (locals.var_guard1877 != 0.0)) && (locals.var_guard1878 != 0.0)) && (locals.var_guard1879 != 0.0)) && (locals.var_guard1881 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign80500_e123203;
        locals.var_t9_dn0 = assign80500_e123203_d_n0;
        locals.var_t9_dn2 = assign80500_e123203_d_n2;
        locals.var_t9_dn4 = assign80500_e123203_d_n4;
        locals.var_t9_dn5 = assign80500_e123203_d_n5;
        locals.var_t9_dn6 = assign80500_e123203_d_n6;
        locals.var_t9_dn7 = assign80500_e123203_d_n7;
        locals.var_t9_dn8 = assign80500_e123203_d_n8;
        locals.var_t9_dn9 = assign80500_e123203_d_n9;
        locals.var_t9_dn10 = assign80500_e123203_d_n10;
        locals.var_t9_dn11 = assign80500_e123203_d_n11;
        locals.var_t9_dn14 = assign80500_e123203_d_n14;

        let (assign80510_e123223, assign80510_e123223_d_n0, assign80510_e123223_d_n2, assign80510_e123223_d_n4, assign80510_e123223_d_n5, assign80510_e123223_d_n6, assign80510_e123223_d_n7, assign80510_e123223_d_n8, assign80510_e123223_d_n9, assign80510_e123223_d_n10, assign80510_e123223_d_n11, assign80510_e123223_d_n14,) = {
    if (((((locals.var_guard1873 != 0.0) && (!((locals.var_guard1871 != 0.0) || (locals.var_guard1872 != 0.0)))) && (locals.var_guard1877 != 0.0)) && (locals.var_guard1878 != 0.0)) && (locals.var_guard1879 != 0.0)) {
        let assign80510_e123218: f64 = (locals.var_kjunc * locals.var_t2);
        let assign80510_e123219: f64 = (assign80510_e123218).sqrt();
        let assign80510_e123221: f64 = (assign80510_e123219 * p.p432);
        (assign80510_e123221, ((((locals.var_kjunc_dn0 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn0)) / (2.0 * assign80510_e123219)) * p.p432), ((((locals.var_kjunc_dn2 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn2)) / (2.0 * assign80510_e123219)) * p.p432), ((((locals.var_kjunc_dn4 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn4)) / (2.0 * assign80510_e123219)) * p.p432), ((((locals.var_kjunc_dn5 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn5)) / (2.0 * assign80510_e123219)) * p.p432), ((((locals.var_kjunc_dn6 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn6)) / (2.0 * assign80510_e123219)) * p.p432), ((((locals.var_kjunc_dn7 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn7)) / (2.0 * assign80510_e123219)) * p.p432), ((((locals.var_kjunc_dn8 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn8)) / (2.0 * assign80510_e123219)) * p.p432), ((((locals.var_kjunc_dn9 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn9)) / (2.0 * assign80510_e123219)) * p.p432), ((((locals.var_kjunc_dn10 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn10)) / (2.0 * assign80510_e123219)) * p.p432), ((((locals.var_kjunc_dn11 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn11)) / (2.0 * assign80510_e123219)) * p.p432), ((((locals.var_kjunc_dn14 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn14)) / (2.0 * assign80510_e123219)) * p.p432),)
    } else {
        (locals.var_wjunc0, locals.var_wjunc0_dn0, locals.var_wjunc0_dn2, locals.var_wjunc0_dn4, locals.var_wjunc0_dn5, locals.var_wjunc0_dn6, locals.var_wjunc0_dn7, locals.var_wjunc0_dn8, locals.var_wjunc0_dn9, locals.var_wjunc0_dn10, locals.var_wjunc0_dn11, locals.var_wjunc0_dn14,)
    }
};
        locals.var_wjunc0 = assign80510_e123223;
        locals.var_wjunc0_dn0 = assign80510_e123223_d_n0;
        locals.var_wjunc0_dn2 = assign80510_e123223_d_n2;
        locals.var_wjunc0_dn4 = assign80510_e123223_d_n4;
        locals.var_wjunc0_dn5 = assign80510_e123223_d_n5;
        locals.var_wjunc0_dn6 = assign80510_e123223_d_n6;
        locals.var_wjunc0_dn7 = assign80510_e123223_d_n7;
        locals.var_wjunc0_dn8 = assign80510_e123223_d_n8;
        locals.var_wjunc0_dn9 = assign80510_e123223_d_n9;
        locals.var_wjunc0_dn10 = assign80510_e123223_d_n10;
        locals.var_wjunc0_dn11 = assign80510_e123223_d_n11;
        locals.var_wjunc0_dn14 = assign80510_e123223_d_n14;

        let (assign80520_e123240, assign80520_e123240_d_n0, assign80520_e123240_d_n2, assign80520_e123240_d_n4, assign80520_e123240_d_n5, assign80520_e123240_d_n6, assign80520_e123240_d_n7, assign80520_e123240_d_n8, assign80520_e123240_d_n9, assign80520_e123240_d_n10, assign80520_e123240_d_n11, assign80520_e123240_d_n14,) = {
    if (((((locals.var_guard1873 != 0.0) && (!((locals.var_guard1871 != 0.0) || (locals.var_guard1872 != 0.0)))) && (locals.var_guard1877 != 0.0)) && (locals.var_guard1878 != 0.0)) && (locals.var_guard1879 != 0.0)) {
        let assign80520_e123238: f64 = (locals.var_lover_func - locals.var_wjunc0);
        (assign80520_e123238, (locals.var_lover_func_dn0 - locals.var_wjunc0_dn0), (locals.var_lover_func_dn2 - locals.var_wjunc0_dn2), (locals.var_lover_func_dn4 - locals.var_wjunc0_dn4), (locals.var_lover_func_dn5 - locals.var_wjunc0_dn5), (locals.var_lover_func_dn6 - locals.var_wjunc0_dn6), (locals.var_lover_func_dn7 - locals.var_wjunc0_dn7), (locals.var_lover_func_dn8 - locals.var_wjunc0_dn8), (locals.var_lover_func_dn9 - locals.var_wjunc0_dn9), (locals.var_lover_func_dn10 - locals.var_wjunc0_dn10), (locals.var_lover_func_dn11 - locals.var_wjunc0_dn11), (locals.var_lover_func_dn14 - locals.var_wjunc0_dn14),)
    } else {
        (locals.var_lover_func, locals.var_lover_func_dn0, locals.var_lover_func_dn2, locals.var_lover_func_dn4, locals.var_lover_func_dn5, locals.var_lover_func_dn6, locals.var_lover_func_dn7, locals.var_lover_func_dn8, locals.var_lover_func_dn9, locals.var_lover_func_dn10, locals.var_lover_func_dn11, locals.var_lover_func_dn14,)
    }
};
        locals.var_lover_func = assign80520_e123240;
        locals.var_lover_func_dn0 = assign80520_e123240_d_n0;
        locals.var_lover_func_dn2 = assign80520_e123240_d_n2;
        locals.var_lover_func_dn4 = assign80520_e123240_d_n4;
        locals.var_lover_func_dn5 = assign80520_e123240_d_n5;
        locals.var_lover_func_dn6 = assign80520_e123240_d_n6;
        locals.var_lover_func_dn7 = assign80520_e123240_d_n7;
        locals.var_lover_func_dn8 = assign80520_e123240_d_n8;
        locals.var_lover_func_dn9 = assign80520_e123240_d_n9;
        locals.var_lover_func_dn10 = assign80520_e123240_d_n10;
        locals.var_lover_func_dn11 = assign80520_e123240_d_n11;
        locals.var_lover_func_dn14 = assign80520_e123240_d_n14;

        let assign80530_e123259: f64 = if (((((p.p35 == 1.0) && (p.p63 > 0.0)) && (locals.var_uc_nover > 0.0)) && (locals.var_uc_cvdsover != 0.0)) && (p.p55 != 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard1882 = assign80530_e123259;

        let (assign80540_e123272,) = {
    if (((locals.var_guard1874 != 0.0) && (!(((locals.var_guard1871 != 0.0) || (locals.var_guard1872 != 0.0)) || (locals.var_guard1873 != 0.0)))) && (locals.var_guard1882 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_calcqover,)
    }
};
        locals.var_flg_calcqover = assign80540_e123272;

        let (assign80550_e123287, assign80550_e123287_d_n2, assign80550_e123287_d_n7, assign80550_e123287_d_n8, assign80550_e123287_d_n9,) = {
    if (((locals.var_guard1874 != 0.0) && (!(((locals.var_guard1871 != 0.0) || (locals.var_guard1872 != 0.0)) || (locals.var_guard1873 != 0.0)))) && (locals.var_guard1882 != 0.0)) {
        let assign80550_e123285: f64 = (locals.var_vgsei - locals.var_vbsei);
        (assign80550_e123285, (locals.var_vgsei_dn2 - locals.var_vbsei_dn2), locals.var_vgsei_dn7, 0.0, (-locals.var_vbsei_dn9),)
    } else {
        (locals.var_vgbgmt, locals.var_vgbgmt_dn2, locals.var_vgbgmt_dn7, locals.var_vgbgmt_dn8, locals.var_vgbgmt_dn9,)
    }
};
        locals.var_vgbgmt = assign80550_e123287;
        locals.var_vgbgmt_dn2 = assign80550_e123287_d_n2;
        locals.var_vgbgmt_dn7 = assign80550_e123287_d_n7;
        locals.var_vgbgmt_dn8 = assign80550_e123287_d_n8;
        locals.var_vgbgmt_dn9 = assign80550_e123287_d_n9;

        let (assign80560_e123302, assign80560_e123302_d_n0, assign80560_e123302_d_n2, assign80560_e123302_d_n4, assign80560_e123302_d_n5, assign80560_e123302_d_n6, assign80560_e123302_d_n7, assign80560_e123302_d_n8, assign80560_e123302_d_n9, assign80560_e123302_d_n10, assign80560_e123302_d_n11, assign80560_e123302_d_n14,) = {
    if (((locals.var_guard1874 != 0.0) && (!(((locals.var_guard1871 != 0.0) || (locals.var_guard1872 != 0.0)) || (locals.var_guard1873 != 0.0)))) && (locals.var_guard1882 != 0.0)) {
        let assign80560_e123300: f64 = (locals.var_vdsei - locals.var_vbsei);
        (assign80560_e123300, locals.var_vdsei_dn0, (locals.var_vdsei_dn2 - locals.var_vbsei_dn2), 0.0, 0.0, 0.0, 0.0, 0.0, (-locals.var_vbsei_dn9), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn11, locals.var_vxbgmt_dn14,)
    }
};
        locals.var_vxbgmt = assign80560_e123302;
        locals.var_vxbgmt_dn0 = assign80560_e123302_d_n0;
        locals.var_vxbgmt_dn2 = assign80560_e123302_d_n2;
        locals.var_vxbgmt_dn4 = assign80560_e123302_d_n4;
        locals.var_vxbgmt_dn5 = assign80560_e123302_d_n5;
        locals.var_vxbgmt_dn6 = assign80560_e123302_d_n6;
        locals.var_vxbgmt_dn7 = assign80560_e123302_d_n7;
        locals.var_vxbgmt_dn8 = assign80560_e123302_d_n8;
        locals.var_vxbgmt_dn9 = assign80560_e123302_d_n9;
        locals.var_vxbgmt_dn10 = assign80560_e123302_d_n10;
        locals.var_vxbgmt_dn11 = assign80560_e123302_d_n11;
        locals.var_vxbgmt_dn14 = assign80560_e123302_d_n14;

        let (assign80570_e123306, assign80570_e123306_d_n0, assign80570_e123306_d_n2, assign80570_e123306_d_n4, assign80570_e123306_d_n5, assign80570_e123306_d_n6, assign80570_e123306_d_n7, assign80570_e123306_d_n8, assign80570_e123306_d_n9, assign80570_e123306_d_n10, assign80570_e123306_d_n11, assign80570_e123306_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.4, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbs_bnd_over__blk1890, locals.var_vbs_bnd_over__blk1890_dn0, locals.var_vbs_bnd_over__blk1890_dn2, locals.var_vbs_bnd_over__blk1890_dn4, locals.var_vbs_bnd_over__blk1890_dn5, locals.var_vbs_bnd_over__blk1890_dn6, locals.var_vbs_bnd_over__blk1890_dn7, locals.var_vbs_bnd_over__blk1890_dn8, locals.var_vbs_bnd_over__blk1890_dn9, locals.var_vbs_bnd_over__blk1890_dn10, locals.var_vbs_bnd_over__blk1890_dn11, locals.var_vbs_bnd_over__blk1890_dn14,)
    }
};
        locals.var_vbs_bnd_over__blk1890 = assign80570_e123306;
        locals.var_vbs_bnd_over__blk1890_dn0 = assign80570_e123306_d_n0;
        locals.var_vbs_bnd_over__blk1890_dn2 = assign80570_e123306_d_n2;
        locals.var_vbs_bnd_over__blk1890_dn4 = assign80570_e123306_d_n4;
        locals.var_vbs_bnd_over__blk1890_dn5 = assign80570_e123306_d_n5;
        locals.var_vbs_bnd_over__blk1890_dn6 = assign80570_e123306_d_n6;
        locals.var_vbs_bnd_over__blk1890_dn7 = assign80570_e123306_d_n7;
        locals.var_vbs_bnd_over__blk1890_dn8 = assign80570_e123306_d_n8;
        locals.var_vbs_bnd_over__blk1890_dn9 = assign80570_e123306_d_n9;
        locals.var_vbs_bnd_over__blk1890_dn10 = assign80570_e123306_d_n10;
        locals.var_vbs_bnd_over__blk1890_dn11 = assign80570_e123306_d_n11;
        locals.var_vbs_bnd_over__blk1890_dn14 = assign80570_e123306_d_n14;

        let (assign80590_e123314,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0,)
    } else {
        (locals.var_flg_fd_mode__blk1891,)
    }
};
        locals.var_flg_fd_mode__blk1891 = assign80590_e123314;

        let (assign80600_e123318, assign80600_e123318_d_n0, assign80600_e123318_d_n2, assign80600_e123318_d_n4, assign80600_e123318_d_n5, assign80600_e123318_d_n6, assign80600_e123318_d_n7, assign80600_e123318_d_n8, assign80600_e123318_d_n9, assign80600_e123318_d_n10, assign80600_e123318_d_n11, assign80600_e123318_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn14,)
    }
};
        locals.var_fb = assign80600_e123318;
        locals.var_fb_dn0 = assign80600_e123318_d_n0;
        locals.var_fb_dn2 = assign80600_e123318_d_n2;
        locals.var_fb_dn4 = assign80600_e123318_d_n4;
        locals.var_fb_dn5 = assign80600_e123318_d_n5;
        locals.var_fb_dn6 = assign80600_e123318_d_n6;
        locals.var_fb_dn7 = assign80600_e123318_d_n7;
        locals.var_fb_dn8 = assign80600_e123318_d_n8;
        locals.var_fb_dn9 = assign80600_e123318_d_n9;
        locals.var_fb_dn10 = assign80600_e123318_d_n10;
        locals.var_fb_dn11 = assign80600_e123318_d_n11;
        locals.var_fb_dn14 = assign80600_e123318_d_n14;

        let (assign80610_e123322, assign80610_e123322_d_n0, assign80610_e123322_d_n2, assign80610_e123322_d_n4, assign80610_e123322_d_n5, assign80610_e123322_d_n6, assign80610_e123322_d_n7, assign80610_e123322_d_n8, assign80610_e123322_d_n9, assign80610_e123322_d_n10, assign80610_e123322_d_n11, assign80610_e123322_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn11, locals.var_fs01_dn14,)
    }
};
        locals.var_fs01 = assign80610_e123322;
        locals.var_fs01_dn0 = assign80610_e123322_d_n0;
        locals.var_fs01_dn2 = assign80610_e123322_d_n2;
        locals.var_fs01_dn4 = assign80610_e123322_d_n4;
        locals.var_fs01_dn5 = assign80610_e123322_d_n5;
        locals.var_fs01_dn6 = assign80610_e123322_d_n6;
        locals.var_fs01_dn7 = assign80610_e123322_d_n7;
        locals.var_fs01_dn8 = assign80610_e123322_d_n8;
        locals.var_fs01_dn9 = assign80610_e123322_d_n9;
        locals.var_fs01_dn10 = assign80610_e123322_d_n10;
        locals.var_fs01_dn11 = assign80610_e123322_d_n11;
        locals.var_fs01_dn14 = assign80610_e123322_d_n14;

        let (assign80620_e123326, assign80620_e123326_d_n0, assign80620_e123326_d_n2, assign80620_e123326_d_n4, assign80620_e123326_d_n5, assign80620_e123326_d_n6, assign80620_e123326_d_n7, assign80620_e123326_d_n8, assign80620_e123326_d_n9, assign80620_e123326_d_n10, assign80620_e123326_d_n11, assign80620_e123326_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn11, locals.var_fs02_dn14,)
    }
};
        locals.var_fs02 = assign80620_e123326;
        locals.var_fs02_dn0 = assign80620_e123326_d_n0;
        locals.var_fs02_dn2 = assign80620_e123326_d_n2;
        locals.var_fs02_dn4 = assign80620_e123326_d_n4;
        locals.var_fs02_dn5 = assign80620_e123326_d_n5;
        locals.var_fs02_dn6 = assign80620_e123326_d_n6;
        locals.var_fs02_dn7 = assign80620_e123326_d_n7;
        locals.var_fs02_dn8 = assign80620_e123326_d_n8;
        locals.var_fs02_dn9 = assign80620_e123326_d_n9;
        locals.var_fs02_dn10 = assign80620_e123326_d_n10;
        locals.var_fs02_dn11 = assign80620_e123326_d_n11;
        locals.var_fs02_dn14 = assign80620_e123326_d_n14;

        let (assign80630_e123330, assign80630_e123330_d_n0, assign80630_e123330_d_n2, assign80630_e123330_d_n4, assign80630_e123330_d_n5, assign80630_e123330_d_n6, assign80630_e123330_d_n7, assign80630_e123330_d_n8, assign80630_e123330_d_n9, assign80630_e123330_d_n10, assign80630_e123330_d_n11, assign80630_e123330_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs0, locals.var_fs0_dn0, locals.var_fs0_dn2, locals.var_fs0_dn4, locals.var_fs0_dn5, locals.var_fs0_dn6, locals.var_fs0_dn7, locals.var_fs0_dn8, locals.var_fs0_dn9, locals.var_fs0_dn10, locals.var_fs0_dn11, locals.var_fs0_dn14,)
    }
};
        locals.var_fs0 = assign80630_e123330;
        locals.var_fs0_dn0 = assign80630_e123330_d_n0;
        locals.var_fs0_dn2 = assign80630_e123330_d_n2;
        locals.var_fs0_dn4 = assign80630_e123330_d_n4;
        locals.var_fs0_dn5 = assign80630_e123330_d_n5;
        locals.var_fs0_dn6 = assign80630_e123330_d_n6;
        locals.var_fs0_dn7 = assign80630_e123330_d_n7;
        locals.var_fs0_dn8 = assign80630_e123330_d_n8;
        locals.var_fs0_dn9 = assign80630_e123330_d_n9;
        locals.var_fs0_dn10 = assign80630_e123330_d_n10;
        locals.var_fs0_dn11 = assign80630_e123330_d_n11;
        locals.var_fs0_dn14 = assign80630_e123330_d_n14;

        let (assign80640_e123334, assign80640_e123334_d_n0, assign80640_e123334_d_n2, assign80640_e123334_d_n4, assign80640_e123334_d_n5, assign80640_e123334_d_n6, assign80640_e123334_d_n7, assign80640_e123334_d_n8, assign80640_e123334_d_n9, assign80640_e123334_d_n10, assign80640_e123334_d_n11, assign80640_e123334_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn4, locals.var_dps0_dn5, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn8, locals.var_dps0_dn9, locals.var_dps0_dn10, locals.var_dps0_dn11, locals.var_dps0_dn14,)
    }
};
        locals.var_dps0 = assign80640_e123334;
        locals.var_dps0_dn0 = assign80640_e123334_d_n0;
        locals.var_dps0_dn2 = assign80640_e123334_d_n2;
        locals.var_dps0_dn4 = assign80640_e123334_d_n4;
        locals.var_dps0_dn5 = assign80640_e123334_d_n5;
        locals.var_dps0_dn6 = assign80640_e123334_d_n6;
        locals.var_dps0_dn7 = assign80640_e123334_d_n7;
        locals.var_dps0_dn8 = assign80640_e123334_d_n8;
        locals.var_dps0_dn9 = assign80640_e123334_d_n9;
        locals.var_dps0_dn10 = assign80640_e123334_d_n10;
        locals.var_dps0_dn11 = assign80640_e123334_d_n11;
        locals.var_dps0_dn14 = assign80640_e123334_d_n14;

        let (assign80650_e123338, assign80650_e123338_d_n0, assign80650_e123338_d_n2, assign80650_e123338_d_n4, assign80650_e123338_d_n5, assign80650_e123338_d_n6, assign80650_e123338_d_n7, assign80650_e123338_d_n8, assign80650_e123338_d_n9, assign80650_e123338_d_n10, assign80650_e123338_d_n11, assign80650_e123338_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs0_dps0, locals.var_fs0_dps0_dn0, locals.var_fs0_dps0_dn2, locals.var_fs0_dps0_dn4, locals.var_fs0_dps0_dn5, locals.var_fs0_dps0_dn6, locals.var_fs0_dps0_dn7, locals.var_fs0_dps0_dn8, locals.var_fs0_dps0_dn9, locals.var_fs0_dps0_dn10, locals.var_fs0_dps0_dn11, locals.var_fs0_dps0_dn14,)
    }
};
        locals.var_fs0_dps0 = assign80650_e123338;
        locals.var_fs0_dps0_dn0 = assign80650_e123338_d_n0;
        locals.var_fs0_dps0_dn2 = assign80650_e123338_d_n2;
        locals.var_fs0_dps0_dn4 = assign80650_e123338_d_n4;
        locals.var_fs0_dps0_dn5 = assign80650_e123338_d_n5;
        locals.var_fs0_dps0_dn6 = assign80650_e123338_d_n6;
        locals.var_fs0_dps0_dn7 = assign80650_e123338_d_n7;
        locals.var_fs0_dps0_dn8 = assign80650_e123338_d_n8;
        locals.var_fs0_dps0_dn9 = assign80650_e123338_d_n9;
        locals.var_fs0_dps0_dn10 = assign80650_e123338_d_n10;
        locals.var_fs0_dps0_dn11 = assign80650_e123338_d_n11;
        locals.var_fs0_dps0_dn14 = assign80650_e123338_d_n14;

        let (assign80660_e123342, assign80660_e123342_d_n0, assign80660_e123342_d_n2, assign80660_e123342_d_n4, assign80660_e123342_d_n5, assign80660_e123342_d_n6, assign80660_e123342_d_n7, assign80660_e123342_d_n8, assign80660_e123342_d_n9, assign80660_e123342_d_n10, assign80660_e123342_d_n11, assign80660_e123342_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn11, locals.var_fs02_dps0_dn14,)
    }
};
        locals.var_fs02_dps0 = assign80660_e123342;
        locals.var_fs02_dps0_dn0 = assign80660_e123342_d_n0;
        locals.var_fs02_dps0_dn2 = assign80660_e123342_d_n2;
        locals.var_fs02_dps0_dn4 = assign80660_e123342_d_n4;
        locals.var_fs02_dps0_dn5 = assign80660_e123342_d_n5;
        locals.var_fs02_dps0_dn6 = assign80660_e123342_d_n6;
        locals.var_fs02_dps0_dn7 = assign80660_e123342_d_n7;
        locals.var_fs02_dps0_dn8 = assign80660_e123342_d_n8;
        locals.var_fs02_dps0_dn9 = assign80660_e123342_d_n9;
        locals.var_fs02_dps0_dn10 = assign80660_e123342_d_n10;
        locals.var_fs02_dps0_dn11 = assign80660_e123342_d_n11;
        locals.var_fs02_dps0_dn14 = assign80660_e123342_d_n14;

        let (assign80670_e123346, assign80670_e123346_d_n0, assign80670_e123346_d_n2, assign80670_e123346_d_n4, assign80670_e123346_d_n5, assign80670_e123346_d_n6, assign80670_e123346_d_n7, assign80670_e123346_d_n8, assign80670_e123346_d_n9, assign80670_e123346_d_n10, assign80670_e123346_d_n11, assign80670_e123346_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn14,)
    }
};
        locals.var_fb_dpss = assign80670_e123346;
        locals.var_fb_dpss_dn0 = assign80670_e123346_d_n0;
        locals.var_fb_dpss_dn2 = assign80670_e123346_d_n2;
        locals.var_fb_dpss_dn4 = assign80670_e123346_d_n4;
        locals.var_fb_dpss_dn5 = assign80670_e123346_d_n5;
        locals.var_fb_dpss_dn6 = assign80670_e123346_d_n6;
        locals.var_fb_dpss_dn7 = assign80670_e123346_d_n7;
        locals.var_fb_dpss_dn8 = assign80670_e123346_d_n8;
        locals.var_fb_dpss_dn9 = assign80670_e123346_d_n9;
        locals.var_fb_dpss_dn10 = assign80670_e123346_d_n10;
        locals.var_fb_dpss_dn11 = assign80670_e123346_d_n11;
        locals.var_fb_dpss_dn14 = assign80670_e123346_d_n14;

        let (assign80680_e123350, assign80680_e123350_d_n0, assign80680_e123350_d_n2, assign80680_e123350_d_n4, assign80680_e123350_d_n5, assign80680_e123350_d_n6, assign80680_e123350_d_n7, assign80680_e123350_d_n8, assign80680_e123350_d_n9, assign80680_e123350_d_n10, assign80680_e123350_d_n11, assign80680_e123350_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn11, locals.var_fs01_dps0_dn14,)
    }
};
        locals.var_fs01_dps0 = assign80680_e123350;
        locals.var_fs01_dps0_dn0 = assign80680_e123350_d_n0;
        locals.var_fs01_dps0_dn2 = assign80680_e123350_d_n2;
        locals.var_fs01_dps0_dn4 = assign80680_e123350_d_n4;
        locals.var_fs01_dps0_dn5 = assign80680_e123350_d_n5;
        locals.var_fs01_dps0_dn6 = assign80680_e123350_d_n6;
        locals.var_fs01_dps0_dn7 = assign80680_e123350_d_n7;
        locals.var_fs01_dps0_dn8 = assign80680_e123350_d_n8;
        locals.var_fs01_dps0_dn9 = assign80680_e123350_d_n9;
        locals.var_fs01_dps0_dn10 = assign80680_e123350_d_n10;
        locals.var_fs01_dps0_dn11 = assign80680_e123350_d_n11;
        locals.var_fs01_dps0_dn14 = assign80680_e123350_d_n14;

        let (assign80690_e123354, assign80690_e123354_d_n0, assign80690_e123354_d_n2, assign80690_e123354_d_n4, assign80690_e123354_d_n5, assign80690_e123354_d_n6, assign80690_e123354_d_n7, assign80690_e123354_d_n8, assign80690_e123354_d_n9, assign80690_e123354_d_n10, assign80690_e123354_d_n11, assign80690_e123354_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn11, locals.var_chi_1_dn14,)
    }
};
        locals.var_chi_1 = assign80690_e123354;
        locals.var_chi_1_dn0 = assign80690_e123354_d_n0;
        locals.var_chi_1_dn2 = assign80690_e123354_d_n2;
        locals.var_chi_1_dn4 = assign80690_e123354_d_n4;
        locals.var_chi_1_dn5 = assign80690_e123354_d_n5;
        locals.var_chi_1_dn6 = assign80690_e123354_d_n6;
        locals.var_chi_1_dn7 = assign80690_e123354_d_n7;
        locals.var_chi_1_dn8 = assign80690_e123354_d_n8;
        locals.var_chi_1_dn9 = assign80690_e123354_d_n9;
        locals.var_chi_1_dn10 = assign80690_e123354_d_n10;
        locals.var_chi_1_dn11 = assign80690_e123354_d_n11;
        locals.var_chi_1_dn14 = assign80690_e123354_d_n14;

        let (assign80700_e123358, assign80700_e123358_d_n0, assign80700_e123358_d_n2, assign80700_e123358_d_n4, assign80700_e123358_d_n5, assign80700_e123358_d_n6, assign80700_e123358_d_n7, assign80700_e123358_d_n8, assign80700_e123358_d_n9, assign80700_e123358_d_n10, assign80700_e123358_d_n11, assign80700_e123358_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_chi_a, locals.var_chi_a_dn0, locals.var_chi_a_dn2, locals.var_chi_a_dn4, locals.var_chi_a_dn5, locals.var_chi_a_dn6, locals.var_chi_a_dn7, locals.var_chi_a_dn8, locals.var_chi_a_dn9, locals.var_chi_a_dn10, locals.var_chi_a_dn11, locals.var_chi_a_dn14,)
    }
};
        locals.var_chi_a = assign80700_e123358;
        locals.var_chi_a_dn0 = assign80700_e123358_d_n0;
        locals.var_chi_a_dn2 = assign80700_e123358_d_n2;
        locals.var_chi_a_dn4 = assign80700_e123358_d_n4;
        locals.var_chi_a_dn5 = assign80700_e123358_d_n5;
        locals.var_chi_a_dn6 = assign80700_e123358_d_n6;
        locals.var_chi_a_dn7 = assign80700_e123358_d_n7;
        locals.var_chi_a_dn8 = assign80700_e123358_d_n8;
        locals.var_chi_a_dn9 = assign80700_e123358_d_n9;
        locals.var_chi_a_dn10 = assign80700_e123358_d_n10;
        locals.var_chi_a_dn11 = assign80700_e123358_d_n11;
        locals.var_chi_a_dn14 = assign80700_e123358_d_n14;

        let (assign80710_e123362, assign80710_e123362_d_n0, assign80710_e123362_d_n2, assign80710_e123362_d_n4, assign80710_e123362_d_n5, assign80710_e123362_d_n6, assign80710_e123362_d_n7, assign80710_e123362_d_n8, assign80710_e123362_d_n9, assign80710_e123362_d_n10, assign80710_e123362_d_n11, assign80710_e123362_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_chi_b, locals.var_chi_b_dn0, locals.var_chi_b_dn2, locals.var_chi_b_dn4, locals.var_chi_b_dn5, locals.var_chi_b_dn6, locals.var_chi_b_dn7, locals.var_chi_b_dn8, locals.var_chi_b_dn9, locals.var_chi_b_dn10, locals.var_chi_b_dn11, locals.var_chi_b_dn14,)
    }
};
        locals.var_chi_b = assign80710_e123362;
        locals.var_chi_b_dn0 = assign80710_e123362_d_n0;
        locals.var_chi_b_dn2 = assign80710_e123362_d_n2;
        locals.var_chi_b_dn4 = assign80710_e123362_d_n4;
        locals.var_chi_b_dn5 = assign80710_e123362_d_n5;
        locals.var_chi_b_dn6 = assign80710_e123362_d_n6;
        locals.var_chi_b_dn7 = assign80710_e123362_d_n7;
        locals.var_chi_b_dn8 = assign80710_e123362_d_n8;
        locals.var_chi_b_dn9 = assign80710_e123362_d_n9;
        locals.var_chi_b_dn10 = assign80710_e123362_d_n10;
        locals.var_chi_b_dn11 = assign80710_e123362_d_n11;
        locals.var_chi_b_dn14 = assign80710_e123362_d_n14;

        let (assign80720_e123367,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign80720_e123365: f64 = (-1.0);
        (assign80720_e123365,)
    } else {
        (locals.var_flg_conv,)
    }
};
        locals.var_flg_conv = assign80720_e123367;

    }

    pub(super) fn stamp_transient_block_292(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign80730_e123371, assign80730_e123371_d_n0, assign80730_e123371_d_n2, assign80730_e123371_d_n4, assign80730_e123371_d_n5, assign80730_e123371_d_n6, assign80730_e123371_d_n7, assign80730_e123371_d_n8, assign80730_e123371_d_n9, assign80730_e123371_d_n10, assign80730_e123371_d_n11, assign80730_e123371_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ps0ld_ini__blk1892, locals.var_ps0ld_ini__blk1892_dn0, locals.var_ps0ld_ini__blk1892_dn2, locals.var_ps0ld_ini__blk1892_dn4, locals.var_ps0ld_ini__blk1892_dn5, locals.var_ps0ld_ini__blk1892_dn6, locals.var_ps0ld_ini__blk1892_dn7, locals.var_ps0ld_ini__blk1892_dn8, locals.var_ps0ld_ini__blk1892_dn9, locals.var_ps0ld_ini__blk1892_dn10, locals.var_ps0ld_ini__blk1892_dn11, locals.var_ps0ld_ini__blk1892_dn14,)
    }
};
        locals.var_ps0ld_ini__blk1892 = assign80730_e123371;
        locals.var_ps0ld_ini__blk1892_dn0 = assign80730_e123371_d_n0;
        locals.var_ps0ld_ini__blk1892_dn2 = assign80730_e123371_d_n2;
        locals.var_ps0ld_ini__blk1892_dn4 = assign80730_e123371_d_n4;
        locals.var_ps0ld_ini__blk1892_dn5 = assign80730_e123371_d_n5;
        locals.var_ps0ld_ini__blk1892_dn6 = assign80730_e123371_d_n6;
        locals.var_ps0ld_ini__blk1892_dn7 = assign80730_e123371_d_n7;
        locals.var_ps0ld_ini__blk1892_dn8 = assign80730_e123371_d_n8;
        locals.var_ps0ld_ini__blk1892_dn9 = assign80730_e123371_d_n9;
        locals.var_ps0ld_ini__blk1892_dn10 = assign80730_e123371_d_n10;
        locals.var_ps0ld_ini__blk1892_dn11 = assign80730_e123371_d_n11;
        locals.var_ps0ld_ini__blk1892_dn14 = assign80730_e123371_d_n14;

        let (assign80740_e123375, assign80740_e123375_d_n0, assign80740_e123375_d_n2, assign80740_e123375_d_n4, assign80740_e123375_d_n5, assign80740_e123375_d_n6, assign80740_e123375_d_n7, assign80740_e123375_d_n8, assign80740_e123375_d_n9, assign80740_e123375_d_n10, assign80740_e123375_d_n11, assign80740_e123375_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fbsq__blk1893, locals.var_fbsq__blk1893_dn0, locals.var_fbsq__blk1893_dn2, locals.var_fbsq__blk1893_dn4, locals.var_fbsq__blk1893_dn5, locals.var_fbsq__blk1893_dn6, locals.var_fbsq__blk1893_dn7, locals.var_fbsq__blk1893_dn8, locals.var_fbsq__blk1893_dn9, locals.var_fbsq__blk1893_dn10, locals.var_fbsq__blk1893_dn11, locals.var_fbsq__blk1893_dn14,)
    }
};
        locals.var_fbsq__blk1893 = assign80740_e123375;
        locals.var_fbsq__blk1893_dn0 = assign80740_e123375_d_n0;
        locals.var_fbsq__blk1893_dn2 = assign80740_e123375_d_n2;
        locals.var_fbsq__blk1893_dn4 = assign80740_e123375_d_n4;
        locals.var_fbsq__blk1893_dn5 = assign80740_e123375_d_n5;
        locals.var_fbsq__blk1893_dn6 = assign80740_e123375_d_n6;
        locals.var_fbsq__blk1893_dn7 = assign80740_e123375_d_n7;
        locals.var_fbsq__blk1893_dn8 = assign80740_e123375_d_n8;
        locals.var_fbsq__blk1893_dn9 = assign80740_e123375_d_n9;
        locals.var_fbsq__blk1893_dn10 = assign80740_e123375_d_n10;
        locals.var_fbsq__blk1893_dn11 = assign80740_e123375_d_n11;
        locals.var_fbsq__blk1893_dn14 = assign80740_e123375_d_n14;

        let (assign80750_e123386, assign80750_e123386_d_n0, assign80750_e123386_d_n2, assign80750_e123386_d_n4, assign80750_e123386_d_n5, assign80750_e123386_d_n6, assign80750_e123386_d_n7, assign80750_e123386_d_n8, assign80750_e123386_d_n9, assign80750_e123386_d_n10, assign80750_e123386_d_n11, assign80750_e123386_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign80750_e123379: f64 = (2.0 * locals.var_beta_inv);
        let assign80750_e123382: f64 = (locals.var_nover_func / locals.var_nin);
        let assign80750_e123383: f64 = (assign80750_e123382).ln();
        let assign80750_e123384: f64 = (assign80750_e123379 * assign80750_e123383);
        (assign80750_e123384, (((2.0 * locals.var_beta_inv_dn0) * assign80750_e123383) + (assign80750_e123379 * ((-((locals.var_nover_func * locals.var_nin_dn0) / (locals.var_nin * locals.var_nin))) / assign80750_e123382))), (((2.0 * locals.var_beta_inv_dn2) * assign80750_e123383) + (assign80750_e123379 * ((-((locals.var_nover_func * locals.var_nin_dn2) / (locals.var_nin * locals.var_nin))) / assign80750_e123382))), (((2.0 * locals.var_beta_inv_dn4) * assign80750_e123383) + (assign80750_e123379 * ((-((locals.var_nover_func * locals.var_nin_dn4) / (locals.var_nin * locals.var_nin))) / assign80750_e123382))), (((2.0 * locals.var_beta_inv_dn5) * assign80750_e123383) + (assign80750_e123379 * ((-((locals.var_nover_func * locals.var_nin_dn5) / (locals.var_nin * locals.var_nin))) / assign80750_e123382))), (((2.0 * locals.var_beta_inv_dn6) * assign80750_e123383) + (assign80750_e123379 * ((-((locals.var_nover_func * locals.var_nin_dn6) / (locals.var_nin * locals.var_nin))) / assign80750_e123382))), (((2.0 * locals.var_beta_inv_dn7) * assign80750_e123383) + (assign80750_e123379 * ((-((locals.var_nover_func * locals.var_nin_dn7) / (locals.var_nin * locals.var_nin))) / assign80750_e123382))), (((2.0 * locals.var_beta_inv_dn8) * assign80750_e123383) + (assign80750_e123379 * ((-((locals.var_nover_func * locals.var_nin_dn8) / (locals.var_nin * locals.var_nin))) / assign80750_e123382))), (((2.0 * locals.var_beta_inv_dn9) * assign80750_e123383) + (assign80750_e123379 * ((-((locals.var_nover_func * locals.var_nin_dn9) / (locals.var_nin * locals.var_nin))) / assign80750_e123382))), (((2.0 * locals.var_beta_inv_dn10) * assign80750_e123383) + (assign80750_e123379 * ((-((locals.var_nover_func * locals.var_nin_dn10) / (locals.var_nin * locals.var_nin))) / assign80750_e123382))), (((2.0 * locals.var_beta_inv_dn11) * assign80750_e123383) + (assign80750_e123379 * ((-((locals.var_nover_func * locals.var_nin_dn11) / (locals.var_nin * locals.var_nin))) / assign80750_e123382))), (((2.0 * locals.var_beta_inv_dn14) * assign80750_e123383) + (assign80750_e123379 * ((-((locals.var_nover_func * locals.var_nin_dn14) / (locals.var_nin * locals.var_nin))) / assign80750_e123382))),)
    } else {
        (locals.var_pb2over__blk1888, locals.var_pb2over__blk1888_dn0, locals.var_pb2over__blk1888_dn2, locals.var_pb2over__blk1888_dn4, locals.var_pb2over__blk1888_dn5, locals.var_pb2over__blk1888_dn6, locals.var_pb2over__blk1888_dn7, locals.var_pb2over__blk1888_dn8, locals.var_pb2over__blk1888_dn9, locals.var_pb2over__blk1888_dn10, locals.var_pb2over__blk1888_dn11, locals.var_pb2over__blk1888_dn14,)
    }
};
        locals.var_pb2over__blk1888 = assign80750_e123386;
        locals.var_pb2over__blk1888_dn0 = assign80750_e123386_d_n0;
        locals.var_pb2over__blk1888_dn2 = assign80750_e123386_d_n2;
        locals.var_pb2over__blk1888_dn4 = assign80750_e123386_d_n4;
        locals.var_pb2over__blk1888_dn5 = assign80750_e123386_d_n5;
        locals.var_pb2over__blk1888_dn6 = assign80750_e123386_d_n6;
        locals.var_pb2over__blk1888_dn7 = assign80750_e123386_d_n7;
        locals.var_pb2over__blk1888_dn8 = assign80750_e123386_d_n8;
        locals.var_pb2over__blk1888_dn9 = assign80750_e123386_d_n9;
        locals.var_pb2over__blk1888_dn10 = assign80750_e123386_d_n10;
        locals.var_pb2over__blk1888_dn11 = assign80750_e123386_d_n11;
        locals.var_pb2over__blk1888_dn14 = assign80750_e123386_d_n14;

        let (assign80760_e123394, assign80760_e123394_d_n0, assign80760_e123394_d_n2, assign80760_e123394_d_n4, assign80760_e123394_d_n5, assign80760_e123394_d_n6, assign80760_e123394_d_n7, assign80760_e123394_d_n8, assign80760_e123394_d_n9, assign80760_e123394_d_n10, assign80760_e123394_d_n11, assign80760_e123394_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign80760_e123390: f64 = (0.8 - locals.var_pb2over__blk1888);
        let assign80760_e123392: f64 = (assign80760_e123390 - 0.1);
        (assign80760_e123392, (-locals.var_pb2over__blk1888_dn0), (-locals.var_pb2over__blk1888_dn2), (-locals.var_pb2over__blk1888_dn4), (-locals.var_pb2over__blk1888_dn5), (-locals.var_pb2over__blk1888_dn6), (-locals.var_pb2over__blk1888_dn7), (-locals.var_pb2over__blk1888_dn8), (-locals.var_pb2over__blk1888_dn9), (-locals.var_pb2over__blk1888_dn10), (-locals.var_pb2over__blk1888_dn11), (-locals.var_pb2over__blk1888_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign80760_e123394;
        locals.var_tmf1_dn0 = assign80760_e123394_d_n0;
        locals.var_tmf1_dn2 = assign80760_e123394_d_n2;
        locals.var_tmf1_dn4 = assign80760_e123394_d_n4;
        locals.var_tmf1_dn5 = assign80760_e123394_d_n5;
        locals.var_tmf1_dn6 = assign80760_e123394_d_n6;
        locals.var_tmf1_dn7 = assign80760_e123394_d_n7;
        locals.var_tmf1_dn8 = assign80760_e123394_d_n8;
        locals.var_tmf1_dn9 = assign80760_e123394_d_n9;
        locals.var_tmf1_dn10 = assign80760_e123394_d_n10;
        locals.var_tmf1_dn11 = assign80760_e123394_d_n11;
        locals.var_tmf1_dn14 = assign80760_e123394_d_n14;

        let (assign80770_e123402, assign80770_e123402_d_n0, assign80770_e123402_d_n2, assign80770_e123402_d_n4, assign80770_e123402_d_n5, assign80770_e123402_d_n6, assign80770_e123402_d_n7, assign80770_e123402_d_n8, assign80770_e123402_d_n9, assign80770_e123402_d_n10, assign80770_e123402_d_n11, assign80770_e123402_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign80770_e123398: f64 = (4.0 * 0.8);
        let assign80770_e123400: f64 = (assign80770_e123398 * 0.1);
        (assign80770_e123400, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign80770_e123402;
        locals.var_tmf2_dn0 = assign80770_e123402_d_n0;
        locals.var_tmf2_dn2 = assign80770_e123402_d_n2;
        locals.var_tmf2_dn4 = assign80770_e123402_d_n4;
        locals.var_tmf2_dn5 = assign80770_e123402_d_n5;
        locals.var_tmf2_dn6 = assign80770_e123402_d_n6;
        locals.var_tmf2_dn7 = assign80770_e123402_d_n7;
        locals.var_tmf2_dn8 = assign80770_e123402_d_n8;
        locals.var_tmf2_dn9 = assign80770_e123402_d_n9;
        locals.var_tmf2_dn10 = assign80770_e123402_d_n10;
        locals.var_tmf2_dn11 = assign80770_e123402_d_n11;
        locals.var_tmf2_dn14 = assign80770_e123402_d_n14;

        let (assign80780_e123412, assign80780_e123412_d_n0, assign80780_e123412_d_n2, assign80780_e123412_d_n4, assign80780_e123412_d_n5, assign80780_e123412_d_n6, assign80780_e123412_d_n7, assign80780_e123412_d_n8, assign80780_e123412_d_n9, assign80780_e123412_d_n10, assign80780_e123412_d_n11, assign80780_e123412_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let (assign80780_e123410, assign80780_e123410_d_n0, assign80780_e123410_d_n2, assign80780_e123410_d_n4, assign80780_e123410_d_n5, assign80780_e123410_d_n6, assign80780_e123410_d_n7, assign80780_e123410_d_n8, assign80780_e123410_d_n9, assign80780_e123410_d_n10, assign80780_e123410_d_n11, assign80780_e123410_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign80780_e123409: f64 = (-locals.var_tmf2);
                (assign80780_e123409, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign80780_e123410, assign80780_e123410_d_n0, assign80780_e123410_d_n2, assign80780_e123410_d_n4, assign80780_e123410_d_n5, assign80780_e123410_d_n6, assign80780_e123410_d_n7, assign80780_e123410_d_n8, assign80780_e123410_d_n9, assign80780_e123410_d_n10, assign80780_e123410_d_n11, assign80780_e123410_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign80780_e123412;
        locals.var_tmf2_dn0 = assign80780_e123412_d_n0;
        locals.var_tmf2_dn2 = assign80780_e123412_d_n2;
        locals.var_tmf2_dn4 = assign80780_e123412_d_n4;
        locals.var_tmf2_dn5 = assign80780_e123412_d_n5;
        locals.var_tmf2_dn6 = assign80780_e123412_d_n6;
        locals.var_tmf2_dn7 = assign80780_e123412_d_n7;
        locals.var_tmf2_dn8 = assign80780_e123412_d_n8;
        locals.var_tmf2_dn9 = assign80780_e123412_d_n9;
        locals.var_tmf2_dn10 = assign80780_e123412_d_n10;
        locals.var_tmf2_dn11 = assign80780_e123412_d_n11;
        locals.var_tmf2_dn14 = assign80780_e123412_d_n14;

        let (assign80790_e123421, assign80790_e123421_d_n0, assign80790_e123421_d_n2, assign80790_e123421_d_n4, assign80790_e123421_d_n5, assign80790_e123421_d_n6, assign80790_e123421_d_n7, assign80790_e123421_d_n8, assign80790_e123421_d_n9, assign80790_e123421_d_n10, assign80790_e123421_d_n11, assign80790_e123421_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign80790_e123416: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign80790_e123418: f64 = (assign80790_e123416 + locals.var_tmf2);
        let assign80790_e123419: f64 = (assign80790_e123418).sqrt();
        (assign80790_e123419, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign80790_e123419)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign80790_e123419)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign80790_e123419)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign80790_e123419)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign80790_e123419)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign80790_e123419)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign80790_e123419)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign80790_e123419)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign80790_e123419)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign80790_e123419)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign80790_e123419)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign80790_e123421;
        locals.var_tmf2_dn0 = assign80790_e123421_d_n0;
        locals.var_tmf2_dn2 = assign80790_e123421_d_n2;
        locals.var_tmf2_dn4 = assign80790_e123421_d_n4;
        locals.var_tmf2_dn5 = assign80790_e123421_d_n5;
        locals.var_tmf2_dn6 = assign80790_e123421_d_n6;
        locals.var_tmf2_dn7 = assign80790_e123421_d_n7;
        locals.var_tmf2_dn8 = assign80790_e123421_d_n8;
        locals.var_tmf2_dn9 = assign80790_e123421_d_n9;
        locals.var_tmf2_dn10 = assign80790_e123421_d_n10;
        locals.var_tmf2_dn11 = assign80790_e123421_d_n11;
        locals.var_tmf2_dn14 = assign80790_e123421_d_n14;

        let (assign80800_e123431, assign80800_e123431_d_n0, assign80800_e123431_d_n2, assign80800_e123431_d_n4, assign80800_e123431_d_n5, assign80800_e123431_d_n6, assign80800_e123431_d_n7, assign80800_e123431_d_n8, assign80800_e123431_d_n9, assign80800_e123431_d_n10, assign80800_e123431_d_n11, assign80800_e123431_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign80800_e123427: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign80800_e123428: f64 = (1.0 + assign80800_e123427);
        let assign80800_e123429: f64 = (0.5 * assign80800_e123428);
        (assign80800_e123429, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign80800_e123431;
        locals.var_t0_dn0 = assign80800_e123431_d_n0;
        locals.var_t0_dn2 = assign80800_e123431_d_n2;
        locals.var_t0_dn4 = assign80800_e123431_d_n4;
        locals.var_t0_dn5 = assign80800_e123431_d_n5;
        locals.var_t0_dn6 = assign80800_e123431_d_n6;
        locals.var_t0_dn7 = assign80800_e123431_d_n7;
        locals.var_t0_dn8 = assign80800_e123431_d_n8;
        locals.var_t0_dn9 = assign80800_e123431_d_n9;
        locals.var_t0_dn10 = assign80800_e123431_d_n10;
        locals.var_t0_dn11 = assign80800_e123431_d_n11;
        locals.var_t0_dn14 = assign80800_e123431_d_n14;

        let (assign80810_e123441, assign80810_e123441_d_n0, assign80810_e123441_d_n2, assign80810_e123441_d_n4, assign80810_e123441_d_n5, assign80810_e123441_d_n6, assign80810_e123441_d_n7, assign80810_e123441_d_n8, assign80810_e123441_d_n9, assign80810_e123441_d_n10, assign80810_e123441_d_n11, assign80810_e123441_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign80810_e123437: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign80810_e123438: f64 = (0.5 * assign80810_e123437);
        let assign80810_e123439: f64 = (0.8 - assign80810_e123438);
        (assign80810_e123439, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (-(0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (-(0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (-(0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (-(0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (-(0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (-(0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (-(0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (-(0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (-(0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_vbs_max_over__blk1889, locals.var_vbs_max_over__blk1889_dn0, locals.var_vbs_max_over__blk1889_dn2, locals.var_vbs_max_over__blk1889_dn4, locals.var_vbs_max_over__blk1889_dn5, locals.var_vbs_max_over__blk1889_dn6, locals.var_vbs_max_over__blk1889_dn7, locals.var_vbs_max_over__blk1889_dn8, locals.var_vbs_max_over__blk1889_dn9, locals.var_vbs_max_over__blk1889_dn10, locals.var_vbs_max_over__blk1889_dn11, locals.var_vbs_max_over__blk1889_dn14,)
    }
};
        locals.var_vbs_max_over__blk1889 = assign80810_e123441;
        locals.var_vbs_max_over__blk1889_dn0 = assign80810_e123441_d_n0;
        locals.var_vbs_max_over__blk1889_dn2 = assign80810_e123441_d_n2;
        locals.var_vbs_max_over__blk1889_dn4 = assign80810_e123441_d_n4;
        locals.var_vbs_max_over__blk1889_dn5 = assign80810_e123441_d_n5;
        locals.var_vbs_max_over__blk1889_dn6 = assign80810_e123441_d_n6;
        locals.var_vbs_max_over__blk1889_dn7 = assign80810_e123441_d_n7;
        locals.var_vbs_max_over__blk1889_dn8 = assign80810_e123441_d_n8;
        locals.var_vbs_max_over__blk1889_dn9 = assign80810_e123441_d_n9;
        locals.var_vbs_max_over__blk1889_dn10 = assign80810_e123441_d_n10;
        locals.var_vbs_max_over__blk1889_dn11 = assign80810_e123441_d_n11;
        locals.var_vbs_max_over__blk1889_dn14 = assign80810_e123441_d_n14;

        let assign80820_e123445: f64 = (locals.var_vbs_max_over__blk1889 * 0.5);
        let assign80820_e123446: f64 = if locals.var_vbs_bnd_over__blk1890 > assign80820_e123445 { 1.0 } else { 0.0 };
        locals.var_guard1895 = assign80820_e123446;

        let (assign80830_e123454, assign80830_e123454_d_n0, assign80830_e123454_d_n2, assign80830_e123454_d_n4, assign80830_e123454_d_n5, assign80830_e123454_d_n6, assign80830_e123454_d_n7, assign80830_e123454_d_n8, assign80830_e123454_d_n9, assign80830_e123454_d_n10, assign80830_e123454_d_n11, assign80830_e123454_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1895 != 0.0)) {
        let assign80830_e123452: f64 = (0.5 * locals.var_vbs_max_over__blk1889);
        (assign80830_e123452, (0.5 * locals.var_vbs_max_over__blk1889_dn0), (0.5 * locals.var_vbs_max_over__blk1889_dn2), (0.5 * locals.var_vbs_max_over__blk1889_dn4), (0.5 * locals.var_vbs_max_over__blk1889_dn5), (0.5 * locals.var_vbs_max_over__blk1889_dn6), (0.5 * locals.var_vbs_max_over__blk1889_dn7), (0.5 * locals.var_vbs_max_over__blk1889_dn8), (0.5 * locals.var_vbs_max_over__blk1889_dn9), (0.5 * locals.var_vbs_max_over__blk1889_dn10), (0.5 * locals.var_vbs_max_over__blk1889_dn11), (0.5 * locals.var_vbs_max_over__blk1889_dn14),)
    } else {
        (locals.var_vbs_bnd_over__blk1890, locals.var_vbs_bnd_over__blk1890_dn0, locals.var_vbs_bnd_over__blk1890_dn2, locals.var_vbs_bnd_over__blk1890_dn4, locals.var_vbs_bnd_over__blk1890_dn5, locals.var_vbs_bnd_over__blk1890_dn6, locals.var_vbs_bnd_over__blk1890_dn7, locals.var_vbs_bnd_over__blk1890_dn8, locals.var_vbs_bnd_over__blk1890_dn9, locals.var_vbs_bnd_over__blk1890_dn10, locals.var_vbs_bnd_over__blk1890_dn11, locals.var_vbs_bnd_over__blk1890_dn14,)
    }
};
        locals.var_vbs_bnd_over__blk1890 = assign80830_e123454;
        locals.var_vbs_bnd_over__blk1890_dn0 = assign80830_e123454_d_n0;
        locals.var_vbs_bnd_over__blk1890_dn2 = assign80830_e123454_d_n2;
        locals.var_vbs_bnd_over__blk1890_dn4 = assign80830_e123454_d_n4;
        locals.var_vbs_bnd_over__blk1890_dn5 = assign80830_e123454_d_n5;
        locals.var_vbs_bnd_over__blk1890_dn6 = assign80830_e123454_d_n6;
        locals.var_vbs_bnd_over__blk1890_dn7 = assign80830_e123454_d_n7;
        locals.var_vbs_bnd_over__blk1890_dn8 = assign80830_e123454_d_n8;
        locals.var_vbs_bnd_over__blk1890_dn9 = assign80830_e123454_d_n9;
        locals.var_vbs_bnd_over__blk1890_dn10 = assign80830_e123454_d_n10;
        locals.var_vbs_bnd_over__blk1890_dn11 = assign80830_e123454_d_n11;
        locals.var_vbs_bnd_over__blk1890_dn14 = assign80830_e123454_d_n14;

        let assign80840_e123456: f64 = if param_given[338] { 1.0 } else { 0.0 };
        locals.var_guard1896 = assign80840_e123456;

        let (assign80850_e123462, assign80850_e123462_d_n0, assign80850_e123462_d_n2, assign80850_e123462_d_n4, assign80850_e123462_d_n5, assign80850_e123462_d_n6, assign80850_e123462_d_n7, assign80850_e123462_d_n8, assign80850_e123462_d_n9, assign80850_e123462_d_n10, assign80850_e123462_d_n11, assign80850_e123462_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1896 != 0.0)) {
        (p.p338, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbs_max_over__blk1889, locals.var_vbs_max_over__blk1889_dn0, locals.var_vbs_max_over__blk1889_dn2, locals.var_vbs_max_over__blk1889_dn4, locals.var_vbs_max_over__blk1889_dn5, locals.var_vbs_max_over__blk1889_dn6, locals.var_vbs_max_over__blk1889_dn7, locals.var_vbs_max_over__blk1889_dn8, locals.var_vbs_max_over__blk1889_dn9, locals.var_vbs_max_over__blk1889_dn10, locals.var_vbs_max_over__blk1889_dn11, locals.var_vbs_max_over__blk1889_dn14,)
    }
};
        locals.var_vbs_max_over__blk1889 = assign80850_e123462;
        locals.var_vbs_max_over__blk1889_dn0 = assign80850_e123462_d_n0;
        locals.var_vbs_max_over__blk1889_dn2 = assign80850_e123462_d_n2;
        locals.var_vbs_max_over__blk1889_dn4 = assign80850_e123462_d_n4;
        locals.var_vbs_max_over__blk1889_dn5 = assign80850_e123462_d_n5;
        locals.var_vbs_max_over__blk1889_dn6 = assign80850_e123462_d_n6;
        locals.var_vbs_max_over__blk1889_dn7 = assign80850_e123462_d_n7;
        locals.var_vbs_max_over__blk1889_dn8 = assign80850_e123462_d_n8;
        locals.var_vbs_max_over__blk1889_dn9 = assign80850_e123462_d_n9;
        locals.var_vbs_max_over__blk1889_dn10 = assign80850_e123462_d_n10;
        locals.var_vbs_max_over__blk1889_dn11 = assign80850_e123462_d_n11;
        locals.var_vbs_max_over__blk1889_dn14 = assign80850_e123462_d_n14;

        let assign80860_e123464: f64 = if param_given[339] { 1.0 } else { 0.0 };
        locals.var_guard1897 = assign80860_e123464;

        let (assign80870_e123470, assign80870_e123470_d_n0, assign80870_e123470_d_n2, assign80870_e123470_d_n4, assign80870_e123470_d_n5, assign80870_e123470_d_n6, assign80870_e123470_d_n7, assign80870_e123470_d_n8, assign80870_e123470_d_n9, assign80870_e123470_d_n10, assign80870_e123470_d_n11, assign80870_e123470_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1897 != 0.0)) {
        (p.p339, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbs_bnd_over__blk1890, locals.var_vbs_bnd_over__blk1890_dn0, locals.var_vbs_bnd_over__blk1890_dn2, locals.var_vbs_bnd_over__blk1890_dn4, locals.var_vbs_bnd_over__blk1890_dn5, locals.var_vbs_bnd_over__blk1890_dn6, locals.var_vbs_bnd_over__blk1890_dn7, locals.var_vbs_bnd_over__blk1890_dn8, locals.var_vbs_bnd_over__blk1890_dn9, locals.var_vbs_bnd_over__blk1890_dn10, locals.var_vbs_bnd_over__blk1890_dn11, locals.var_vbs_bnd_over__blk1890_dn14,)
    }
};
        locals.var_vbs_bnd_over__blk1890 = assign80870_e123470;
        locals.var_vbs_bnd_over__blk1890_dn0 = assign80870_e123470_d_n0;
        locals.var_vbs_bnd_over__blk1890_dn2 = assign80870_e123470_d_n2;
        locals.var_vbs_bnd_over__blk1890_dn4 = assign80870_e123470_d_n4;
        locals.var_vbs_bnd_over__blk1890_dn5 = assign80870_e123470_d_n5;
        locals.var_vbs_bnd_over__blk1890_dn6 = assign80870_e123470_d_n6;
        locals.var_vbs_bnd_over__blk1890_dn7 = assign80870_e123470_d_n7;
        locals.var_vbs_bnd_over__blk1890_dn8 = assign80870_e123470_d_n8;
        locals.var_vbs_bnd_over__blk1890_dn9 = assign80870_e123470_d_n9;
        locals.var_vbs_bnd_over__blk1890_dn10 = assign80870_e123470_d_n10;
        locals.var_vbs_bnd_over__blk1890_dn11 = assign80870_e123470_d_n11;
        locals.var_vbs_bnd_over__blk1890_dn14 = assign80870_e123470_d_n14;

        let assign80880_e123472: f64 = if param_given[338] { 1.0 } else { 0.0 };
        locals.var_guard1898 = assign80880_e123472;

        let (assign80890_e123483, assign80890_e123483_d_n0, assign80890_e123483_d_n2, assign80890_e123483_d_n4, assign80890_e123483_d_n5, assign80890_e123483_d_n6, assign80890_e123483_d_n7, assign80890_e123483_d_n8, assign80890_e123483_d_n9, assign80890_e123483_d_n10, assign80890_e123483_d_n11, assign80890_e123483_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1897 == 0.0)) && (locals.var_guard1898 != 0.0)) {
        let assign80890_e123481: f64 = (0.5 * locals.var_vbs_max_over__blk1889);
        (assign80890_e123481, (0.5 * locals.var_vbs_max_over__blk1889_dn0), (0.5 * locals.var_vbs_max_over__blk1889_dn2), (0.5 * locals.var_vbs_max_over__blk1889_dn4), (0.5 * locals.var_vbs_max_over__blk1889_dn5), (0.5 * locals.var_vbs_max_over__blk1889_dn6), (0.5 * locals.var_vbs_max_over__blk1889_dn7), (0.5 * locals.var_vbs_max_over__blk1889_dn8), (0.5 * locals.var_vbs_max_over__blk1889_dn9), (0.5 * locals.var_vbs_max_over__blk1889_dn10), (0.5 * locals.var_vbs_max_over__blk1889_dn11), (0.5 * locals.var_vbs_max_over__blk1889_dn14),)
    } else {
        (locals.var_vbs_bnd_over__blk1890, locals.var_vbs_bnd_over__blk1890_dn0, locals.var_vbs_bnd_over__blk1890_dn2, locals.var_vbs_bnd_over__blk1890_dn4, locals.var_vbs_bnd_over__blk1890_dn5, locals.var_vbs_bnd_over__blk1890_dn6, locals.var_vbs_bnd_over__blk1890_dn7, locals.var_vbs_bnd_over__blk1890_dn8, locals.var_vbs_bnd_over__blk1890_dn9, locals.var_vbs_bnd_over__blk1890_dn10, locals.var_vbs_bnd_over__blk1890_dn11, locals.var_vbs_bnd_over__blk1890_dn14,)
    }
};
        locals.var_vbs_bnd_over__blk1890 = assign80890_e123483;
        locals.var_vbs_bnd_over__blk1890_dn0 = assign80890_e123483_d_n0;
        locals.var_vbs_bnd_over__blk1890_dn2 = assign80890_e123483_d_n2;
        locals.var_vbs_bnd_over__blk1890_dn4 = assign80890_e123483_d_n4;
        locals.var_vbs_bnd_over__blk1890_dn5 = assign80890_e123483_d_n5;
        locals.var_vbs_bnd_over__blk1890_dn6 = assign80890_e123483_d_n6;
        locals.var_vbs_bnd_over__blk1890_dn7 = assign80890_e123483_d_n7;
        locals.var_vbs_bnd_over__blk1890_dn8 = assign80890_e123483_d_n8;
        locals.var_vbs_bnd_over__blk1890_dn9 = assign80890_e123483_d_n9;
        locals.var_vbs_bnd_over__blk1890_dn10 = assign80890_e123483_d_n10;
        locals.var_vbs_bnd_over__blk1890_dn11 = assign80890_e123483_d_n11;
        locals.var_vbs_bnd_over__blk1890_dn14 = assign80890_e123483_d_n14;

        let assign80900_e123487: f64 = (locals.var_vbs_max_over__blk1889 * 0.5);
        let assign80900_e123488: f64 = if locals.var_vbs_bnd_over__blk1890 > assign80900_e123487 { 1.0 } else { 0.0 };
        locals.var_guard1899 = assign80900_e123488;

        let (assign80910_e123496, assign80910_e123496_d_n0, assign80910_e123496_d_n2, assign80910_e123496_d_n4, assign80910_e123496_d_n5, assign80910_e123496_d_n6, assign80910_e123496_d_n7, assign80910_e123496_d_n8, assign80910_e123496_d_n9, assign80910_e123496_d_n10, assign80910_e123496_d_n11, assign80910_e123496_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1899 != 0.0)) {
        let assign80910_e123494: f64 = (0.5 * locals.var_vbs_max_over__blk1889);
        (assign80910_e123494, (0.5 * locals.var_vbs_max_over__blk1889_dn0), (0.5 * locals.var_vbs_max_over__blk1889_dn2), (0.5 * locals.var_vbs_max_over__blk1889_dn4), (0.5 * locals.var_vbs_max_over__blk1889_dn5), (0.5 * locals.var_vbs_max_over__blk1889_dn6), (0.5 * locals.var_vbs_max_over__blk1889_dn7), (0.5 * locals.var_vbs_max_over__blk1889_dn8), (0.5 * locals.var_vbs_max_over__blk1889_dn9), (0.5 * locals.var_vbs_max_over__blk1889_dn10), (0.5 * locals.var_vbs_max_over__blk1889_dn11), (0.5 * locals.var_vbs_max_over__blk1889_dn14),)
    } else {
        (locals.var_vbs_bnd_over__blk1890, locals.var_vbs_bnd_over__blk1890_dn0, locals.var_vbs_bnd_over__blk1890_dn2, locals.var_vbs_bnd_over__blk1890_dn4, locals.var_vbs_bnd_over__blk1890_dn5, locals.var_vbs_bnd_over__blk1890_dn6, locals.var_vbs_bnd_over__blk1890_dn7, locals.var_vbs_bnd_over__blk1890_dn8, locals.var_vbs_bnd_over__blk1890_dn9, locals.var_vbs_bnd_over__blk1890_dn10, locals.var_vbs_bnd_over__blk1890_dn11, locals.var_vbs_bnd_over__blk1890_dn14,)
    }
};
        locals.var_vbs_bnd_over__blk1890 = assign80910_e123496;
        locals.var_vbs_bnd_over__blk1890_dn0 = assign80910_e123496_d_n0;
        locals.var_vbs_bnd_over__blk1890_dn2 = assign80910_e123496_d_n2;
        locals.var_vbs_bnd_over__blk1890_dn4 = assign80910_e123496_d_n4;
        locals.var_vbs_bnd_over__blk1890_dn5 = assign80910_e123496_d_n5;
        locals.var_vbs_bnd_over__blk1890_dn6 = assign80910_e123496_d_n6;
        locals.var_vbs_bnd_over__blk1890_dn7 = assign80910_e123496_d_n7;
        locals.var_vbs_bnd_over__blk1890_dn8 = assign80910_e123496_d_n8;
        locals.var_vbs_bnd_over__blk1890_dn9 = assign80910_e123496_d_n9;
        locals.var_vbs_bnd_over__blk1890_dn10 = assign80910_e123496_d_n10;
        locals.var_vbs_bnd_over__blk1890_dn11 = assign80910_e123496_d_n11;
        locals.var_vbs_bnd_over__blk1890_dn14 = assign80910_e123496_d_n14;

        let assign80920_e123499: f64 = if p.p38 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1900 = assign80920_e123499;

        let (assign80930_e123506, assign80930_e123506_d_n0, assign80930_e123506_d_n2, assign80930_e123506_d_n4, assign80930_e123506_d_n5, assign80930_e123506_d_n6, assign80930_e123506_d_n7, assign80930_e123506_d_n8, assign80930_e123506_d_n9, assign80930_e123506_d_n10, assign80930_e123506_d_n11, assign80930_e123506_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1900 != 0.0)) {
        let assign80930_e123504: f64 = (-locals.var_vxbgmt);
        (assign80930_e123504, (-locals.var_vxbgmt_dn0), (-locals.var_vxbgmt_dn2), (-locals.var_vxbgmt_dn4), (-locals.var_vxbgmt_dn5), (-locals.var_vxbgmt_dn6), (-locals.var_vxbgmt_dn7), (-locals.var_vxbgmt_dn8), (-locals.var_vxbgmt_dn9), (-locals.var_vxbgmt_dn10), (-locals.var_vxbgmt_dn11), (-locals.var_vxbgmt_dn14),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign80930_e123506;
        locals.var_t0_dn0 = assign80930_e123506_d_n0;
        locals.var_t0_dn2 = assign80930_e123506_d_n2;
        locals.var_t0_dn4 = assign80930_e123506_d_n4;
        locals.var_t0_dn5 = assign80930_e123506_d_n5;
        locals.var_t0_dn6 = assign80930_e123506_d_n6;
        locals.var_t0_dn7 = assign80930_e123506_d_n7;
        locals.var_t0_dn8 = assign80930_e123506_d_n8;
        locals.var_t0_dn9 = assign80930_e123506_d_n9;
        locals.var_t0_dn10 = assign80930_e123506_d_n10;
        locals.var_t0_dn11 = assign80930_e123506_d_n11;
        locals.var_t0_dn14 = assign80930_e123506_d_n14;

        let assign80940_e123509: f64 = if locals.var_t0 > locals.var_vbs_bnd_over__blk1890 { 1.0 } else { 0.0 };
        locals.var_guard1901 = assign80940_e123509;

        let (assign80950_e123519, assign80950_e123519_d_n0, assign80950_e123519_d_n2, assign80950_e123519_d_n4, assign80950_e123519_d_n5, assign80950_e123519_d_n6, assign80950_e123519_d_n7, assign80950_e123519_d_n8, assign80950_e123519_d_n9, assign80950_e123519_d_n10, assign80950_e123519_d_n11, assign80950_e123519_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1900 != 0.0)) && (locals.var_guard1901 != 0.0)) {
        let assign80950_e123517: f64 = (locals.var_t0 - locals.var_vbs_bnd_over__blk1890);
        (assign80950_e123517, (locals.var_t0_dn0 - locals.var_vbs_bnd_over__blk1890_dn0), (locals.var_t0_dn2 - locals.var_vbs_bnd_over__blk1890_dn2), (locals.var_t0_dn4 - locals.var_vbs_bnd_over__blk1890_dn4), (locals.var_t0_dn5 - locals.var_vbs_bnd_over__blk1890_dn5), (locals.var_t0_dn6 - locals.var_vbs_bnd_over__blk1890_dn6), (locals.var_t0_dn7 - locals.var_vbs_bnd_over__blk1890_dn7), (locals.var_t0_dn8 - locals.var_vbs_bnd_over__blk1890_dn8), (locals.var_t0_dn9 - locals.var_vbs_bnd_over__blk1890_dn9), (locals.var_t0_dn10 - locals.var_vbs_bnd_over__blk1890_dn10), (locals.var_t0_dn11 - locals.var_vbs_bnd_over__blk1890_dn11), (locals.var_t0_dn14 - locals.var_vbs_bnd_over__blk1890_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign80950_e123519;
        locals.var_t1_dn0 = assign80950_e123519_d_n0;
        locals.var_t1_dn2 = assign80950_e123519_d_n2;
        locals.var_t1_dn4 = assign80950_e123519_d_n4;
        locals.var_t1_dn5 = assign80950_e123519_d_n5;
        locals.var_t1_dn6 = assign80950_e123519_d_n6;
        locals.var_t1_dn7 = assign80950_e123519_d_n7;
        locals.var_t1_dn8 = assign80950_e123519_d_n8;
        locals.var_t1_dn9 = assign80950_e123519_d_n9;
        locals.var_t1_dn10 = assign80950_e123519_d_n10;
        locals.var_t1_dn11 = assign80950_e123519_d_n11;
        locals.var_t1_dn14 = assign80950_e123519_d_n14;

        let (assign80960_e123529, assign80960_e123529_d_n0, assign80960_e123529_d_n2, assign80960_e123529_d_n4, assign80960_e123529_d_n5, assign80960_e123529_d_n6, assign80960_e123529_d_n7, assign80960_e123529_d_n8, assign80960_e123529_d_n9, assign80960_e123529_d_n10, assign80960_e123529_d_n11, assign80960_e123529_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1900 != 0.0)) && (locals.var_guard1901 != 0.0)) {
        let assign80960_e123527: f64 = (locals.var_vbs_max_over__blk1889 - locals.var_vbs_bnd_over__blk1890);
        (assign80960_e123527, (locals.var_vbs_max_over__blk1889_dn0 - locals.var_vbs_bnd_over__blk1890_dn0), (locals.var_vbs_max_over__blk1889_dn2 - locals.var_vbs_bnd_over__blk1890_dn2), (locals.var_vbs_max_over__blk1889_dn4 - locals.var_vbs_bnd_over__blk1890_dn4), (locals.var_vbs_max_over__blk1889_dn5 - locals.var_vbs_bnd_over__blk1890_dn5), (locals.var_vbs_max_over__blk1889_dn6 - locals.var_vbs_bnd_over__blk1890_dn6), (locals.var_vbs_max_over__blk1889_dn7 - locals.var_vbs_bnd_over__blk1890_dn7), (locals.var_vbs_max_over__blk1889_dn8 - locals.var_vbs_bnd_over__blk1890_dn8), (locals.var_vbs_max_over__blk1889_dn9 - locals.var_vbs_bnd_over__blk1890_dn9), (locals.var_vbs_max_over__blk1889_dn10 - locals.var_vbs_bnd_over__blk1890_dn10), (locals.var_vbs_max_over__blk1889_dn11 - locals.var_vbs_bnd_over__blk1890_dn11), (locals.var_vbs_max_over__blk1889_dn14 - locals.var_vbs_bnd_over__blk1890_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign80960_e123529;
        locals.var_t2_dn0 = assign80960_e123529_d_n0;
        locals.var_t2_dn2 = assign80960_e123529_d_n2;
        locals.var_t2_dn4 = assign80960_e123529_d_n4;
        locals.var_t2_dn5 = assign80960_e123529_d_n5;
        locals.var_t2_dn6 = assign80960_e123529_d_n6;
        locals.var_t2_dn7 = assign80960_e123529_d_n7;
        locals.var_t2_dn8 = assign80960_e123529_d_n8;
        locals.var_t2_dn9 = assign80960_e123529_d_n9;
        locals.var_t2_dn10 = assign80960_e123529_d_n10;
        locals.var_t2_dn11 = assign80960_e123529_d_n11;
        locals.var_t2_dn14 = assign80960_e123529_d_n14;

        let (assign80970_e123539, assign80970_e123539_d_n0, assign80970_e123539_d_n2, assign80970_e123539_d_n4, assign80970_e123539_d_n5, assign80970_e123539_d_n6, assign80970_e123539_d_n7, assign80970_e123539_d_n8, assign80970_e123539_d_n9, assign80970_e123539_d_n10, assign80970_e123539_d_n11, assign80970_e123539_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1900 != 0.0)) && (locals.var_guard1901 != 0.0)) {
        let assign80970_e123537: f64 = (locals.var_t1 / locals.var_t2);
        (assign80970_e123537, (((locals.var_t1_dn0 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn0)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn2 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn2)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn4 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn4)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn5 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn5)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn6 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn6)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn7 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn7)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn8 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn8)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn9 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn9)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn10 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn10)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn11 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn11)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn14 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn14)) / (locals.var_t2 * locals.var_t2)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign80970_e123539;
        locals.var_tmf1_dn0 = assign80970_e123539_d_n0;
        locals.var_tmf1_dn2 = assign80970_e123539_d_n2;
        locals.var_tmf1_dn4 = assign80970_e123539_d_n4;
        locals.var_tmf1_dn5 = assign80970_e123539_d_n5;
        locals.var_tmf1_dn6 = assign80970_e123539_d_n6;
        locals.var_tmf1_dn7 = assign80970_e123539_d_n7;
        locals.var_tmf1_dn8 = assign80970_e123539_d_n8;
        locals.var_tmf1_dn9 = assign80970_e123539_d_n9;
        locals.var_tmf1_dn10 = assign80970_e123539_d_n10;
        locals.var_tmf1_dn11 = assign80970_e123539_d_n11;
        locals.var_tmf1_dn14 = assign80970_e123539_d_n14;

        let (assign80980_e123549, assign80980_e123549_d_n0, assign80980_e123549_d_n2, assign80980_e123549_d_n4, assign80980_e123549_d_n5, assign80980_e123549_d_n6, assign80980_e123549_d_n7, assign80980_e123549_d_n8, assign80980_e123549_d_n9, assign80980_e123549_d_n10, assign80980_e123549_d_n11, assign80980_e123549_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1900 != 0.0)) && (locals.var_guard1901 != 0.0)) {
        let assign80980_e123547: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign80980_e123547, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign80980_e123549;
        locals.var_tmf2_dn0 = assign80980_e123549_d_n0;
        locals.var_tmf2_dn2 = assign80980_e123549_d_n2;
        locals.var_tmf2_dn4 = assign80980_e123549_d_n4;
        locals.var_tmf2_dn5 = assign80980_e123549_d_n5;
        locals.var_tmf2_dn6 = assign80980_e123549_d_n6;
        locals.var_tmf2_dn7 = assign80980_e123549_d_n7;
        locals.var_tmf2_dn8 = assign80980_e123549_d_n8;
        locals.var_tmf2_dn9 = assign80980_e123549_d_n9;
        locals.var_tmf2_dn10 = assign80980_e123549_d_n10;
        locals.var_tmf2_dn11 = assign80980_e123549_d_n11;
        locals.var_tmf2_dn14 = assign80980_e123549_d_n14;

        let (assign80990_e123559, assign80990_e123559_d_n0, assign80990_e123559_d_n2, assign80990_e123559_d_n4, assign80990_e123559_d_n5, assign80990_e123559_d_n6, assign80990_e123559_d_n7, assign80990_e123559_d_n8, assign80990_e123559_d_n9, assign80990_e123559_d_n10, assign80990_e123559_d_n11, assign80990_e123559_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1900 != 0.0)) && (locals.var_guard1901 != 0.0)) {
        let assign80990_e123557: f64 = (locals.var_tmf2 * locals.var_tmf1);
        (assign80990_e123557, ((locals.var_tmf2_dn0 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn0)), ((locals.var_tmf2_dn2 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn2)), ((locals.var_tmf2_dn4 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn4)), ((locals.var_tmf2_dn5 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn5)), ((locals.var_tmf2_dn6 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn6)), ((locals.var_tmf2_dn7 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn7)), ((locals.var_tmf2_dn8 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn8)), ((locals.var_tmf2_dn9 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn9)), ((locals.var_tmf2_dn10 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn10)), ((locals.var_tmf2_dn11 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn11)), ((locals.var_tmf2_dn14 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_tmf3, locals.var_tmf3_dn0, locals.var_tmf3_dn2, locals.var_tmf3_dn4, locals.var_tmf3_dn5, locals.var_tmf3_dn6, locals.var_tmf3_dn7, locals.var_tmf3_dn8, locals.var_tmf3_dn9, locals.var_tmf3_dn10, locals.var_tmf3_dn11, locals.var_tmf3_dn14,)
    }
};
        locals.var_tmf3 = assign80990_e123559;
        locals.var_tmf3_dn0 = assign80990_e123559_d_n0;
        locals.var_tmf3_dn2 = assign80990_e123559_d_n2;
        locals.var_tmf3_dn4 = assign80990_e123559_d_n4;
        locals.var_tmf3_dn5 = assign80990_e123559_d_n5;
        locals.var_tmf3_dn6 = assign80990_e123559_d_n6;
        locals.var_tmf3_dn7 = assign80990_e123559_d_n7;
        locals.var_tmf3_dn8 = assign80990_e123559_d_n8;
        locals.var_tmf3_dn9 = assign80990_e123559_d_n9;
        locals.var_tmf3_dn10 = assign80990_e123559_d_n10;
        locals.var_tmf3_dn11 = assign80990_e123559_d_n11;
        locals.var_tmf3_dn14 = assign80990_e123559_d_n14;

        let (assign81000_e123569, assign81000_e123569_d_n0, assign81000_e123569_d_n2, assign81000_e123569_d_n4, assign81000_e123569_d_n5, assign81000_e123569_d_n6, assign81000_e123569_d_n7, assign81000_e123569_d_n8, assign81000_e123569_d_n9, assign81000_e123569_d_n10, assign81000_e123569_d_n11, assign81000_e123569_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1900 != 0.0)) && (locals.var_guard1901 != 0.0)) {
        let assign81000_e123567: f64 = (locals.var_tmf2 * locals.var_tmf2);
        (assign81000_e123567, ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0)), ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2)), ((locals.var_tmf2_dn4 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn4)), ((locals.var_tmf2_dn5 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn5)), ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6)), ((locals.var_tmf2_dn7 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn7)), ((locals.var_tmf2_dn8 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn8)), ((locals.var_tmf2_dn9 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn9)), ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10)), ((locals.var_tmf2_dn11 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn11)), ((locals.var_tmf2_dn14 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn14)),)
    } else {
        (locals.var_tmf4, locals.var_tmf4_dn0, locals.var_tmf4_dn2, locals.var_tmf4_dn4, locals.var_tmf4_dn5, locals.var_tmf4_dn6, locals.var_tmf4_dn7, locals.var_tmf4_dn8, locals.var_tmf4_dn9, locals.var_tmf4_dn10, locals.var_tmf4_dn11, locals.var_tmf4_dn14,)
    }
};
        locals.var_tmf4 = assign81000_e123569;
        locals.var_tmf4_dn0 = assign81000_e123569_d_n0;
        locals.var_tmf4_dn2 = assign81000_e123569_d_n2;
        locals.var_tmf4_dn4 = assign81000_e123569_d_n4;
        locals.var_tmf4_dn5 = assign81000_e123569_d_n5;
        locals.var_tmf4_dn6 = assign81000_e123569_d_n6;
        locals.var_tmf4_dn7 = assign81000_e123569_d_n7;
        locals.var_tmf4_dn8 = assign81000_e123569_d_n8;
        locals.var_tmf4_dn9 = assign81000_e123569_d_n9;
        locals.var_tmf4_dn10 = assign81000_e123569_d_n10;
        locals.var_tmf4_dn11 = assign81000_e123569_d_n11;
        locals.var_tmf4_dn14 = assign81000_e123569_d_n14;

        let (assign81010_e123587, assign81010_e123587_d_n0, assign81010_e123587_d_n2, assign81010_e123587_d_n4, assign81010_e123587_d_n5, assign81010_e123587_d_n6, assign81010_e123587_d_n7, assign81010_e123587_d_n8, assign81010_e123587_d_n9, assign81010_e123587_d_n10, assign81010_e123587_d_n11, assign81010_e123587_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1900 != 0.0)) && (locals.var_guard1901 != 0.0)) {
        let assign81010_e123578: f64 = (1.0 + locals.var_tmf1);
        let assign81010_e123580: f64 = (assign81010_e123578 + locals.var_tmf2);
        let assign81010_e123582: f64 = (assign81010_e123580 + locals.var_tmf3);
        let assign81010_e123584: f64 = (assign81010_e123582 + locals.var_tmf4);
        let assign81010_e123585: f64 = (1.0 / assign81010_e123584);
        (assign81010_e123585, (-((((locals.var_tmf1_dn0 + locals.var_tmf2_dn0) + locals.var_tmf3_dn0) + locals.var_tmf4_dn0) / (assign81010_e123584 * assign81010_e123584))), (-((((locals.var_tmf1_dn2 + locals.var_tmf2_dn2) + locals.var_tmf3_dn2) + locals.var_tmf4_dn2) / (assign81010_e123584 * assign81010_e123584))), (-((((locals.var_tmf1_dn4 + locals.var_tmf2_dn4) + locals.var_tmf3_dn4) + locals.var_tmf4_dn4) / (assign81010_e123584 * assign81010_e123584))), (-((((locals.var_tmf1_dn5 + locals.var_tmf2_dn5) + locals.var_tmf3_dn5) + locals.var_tmf4_dn5) / (assign81010_e123584 * assign81010_e123584))), (-((((locals.var_tmf1_dn6 + locals.var_tmf2_dn6) + locals.var_tmf3_dn6) + locals.var_tmf4_dn6) / (assign81010_e123584 * assign81010_e123584))), (-((((locals.var_tmf1_dn7 + locals.var_tmf2_dn7) + locals.var_tmf3_dn7) + locals.var_tmf4_dn7) / (assign81010_e123584 * assign81010_e123584))), (-((((locals.var_tmf1_dn8 + locals.var_tmf2_dn8) + locals.var_tmf3_dn8) + locals.var_tmf4_dn8) / (assign81010_e123584 * assign81010_e123584))), (-((((locals.var_tmf1_dn9 + locals.var_tmf2_dn9) + locals.var_tmf3_dn9) + locals.var_tmf4_dn9) / (assign81010_e123584 * assign81010_e123584))), (-((((locals.var_tmf1_dn10 + locals.var_tmf2_dn10) + locals.var_tmf3_dn10) + locals.var_tmf4_dn10) / (assign81010_e123584 * assign81010_e123584))), (-((((locals.var_tmf1_dn11 + locals.var_tmf2_dn11) + locals.var_tmf3_dn11) + locals.var_tmf4_dn11) / (assign81010_e123584 * assign81010_e123584))), (-((((locals.var_tmf1_dn14 + locals.var_tmf2_dn14) + locals.var_tmf3_dn14) + locals.var_tmf4_dn14) / (assign81010_e123584 * assign81010_e123584))),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign81010_e123587;
        locals.var_tmf0_dn0 = assign81010_e123587_d_n0;
        locals.var_tmf0_dn2 = assign81010_e123587_d_n2;
        locals.var_tmf0_dn4 = assign81010_e123587_d_n4;
        locals.var_tmf0_dn5 = assign81010_e123587_d_n5;
        locals.var_tmf0_dn6 = assign81010_e123587_d_n6;
        locals.var_tmf0_dn7 = assign81010_e123587_d_n7;
        locals.var_tmf0_dn8 = assign81010_e123587_d_n8;
        locals.var_tmf0_dn9 = assign81010_e123587_d_n9;
        locals.var_tmf0_dn10 = assign81010_e123587_d_n10;
        locals.var_tmf0_dn11 = assign81010_e123587_d_n11;
        locals.var_tmf0_dn14 = assign81010_e123587_d_n14;

    }

    pub(super) fn stamp_transient_block_293(
        locals: &mut StampLocals,
    ) {
        let (assign81020_e123612, assign81020_e123612_d_n0, assign81020_e123612_d_n2, assign81020_e123612_d_n4, assign81020_e123612_d_n5, assign81020_e123612_d_n6, assign81020_e123612_d_n7, assign81020_e123612_d_n8, assign81020_e123612_d_n9, assign81020_e123612_d_n10, assign81020_e123612_d_n11, assign81020_e123612_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1900 != 0.0)) && (locals.var_guard1901 != 0.0)) {
        let assign81020_e123596: f64 = (2.0 * locals.var_tmf1);
        let assign81020_e123597: f64 = (1.0 + assign81020_e123596);
        let assign81020_e123600: f64 = (3.0 * locals.var_tmf2);
        let assign81020_e123601: f64 = (assign81020_e123597 + assign81020_e123600);
        let assign81020_e123604: f64 = (4.0 * locals.var_tmf3);
        let assign81020_e123605: f64 = (assign81020_e123601 + assign81020_e123604);
        let assign81020_e123606: f64 = (-assign81020_e123605);
        let assign81020_e123608: f64 = (assign81020_e123606 * locals.var_tmf0);
        let assign81020_e123610: f64 = (assign81020_e123608 * locals.var_tmf0);
        (assign81020_e123610, (((((-(((2.0 * locals.var_tmf1_dn0) + (3.0 * locals.var_tmf2_dn0)) + (4.0 * locals.var_tmf3_dn0))) * locals.var_tmf0) + (assign81020_e123606 * locals.var_tmf0_dn0)) * locals.var_tmf0) + (assign81020_e123608 * locals.var_tmf0_dn0)), (((((-(((2.0 * locals.var_tmf1_dn2) + (3.0 * locals.var_tmf2_dn2)) + (4.0 * locals.var_tmf3_dn2))) * locals.var_tmf0) + (assign81020_e123606 * locals.var_tmf0_dn2)) * locals.var_tmf0) + (assign81020_e123608 * locals.var_tmf0_dn2)), (((((-(((2.0 * locals.var_tmf1_dn4) + (3.0 * locals.var_tmf2_dn4)) + (4.0 * locals.var_tmf3_dn4))) * locals.var_tmf0) + (assign81020_e123606 * locals.var_tmf0_dn4)) * locals.var_tmf0) + (assign81020_e123608 * locals.var_tmf0_dn4)), (((((-(((2.0 * locals.var_tmf1_dn5) + (3.0 * locals.var_tmf2_dn5)) + (4.0 * locals.var_tmf3_dn5))) * locals.var_tmf0) + (assign81020_e123606 * locals.var_tmf0_dn5)) * locals.var_tmf0) + (assign81020_e123608 * locals.var_tmf0_dn5)), (((((-(((2.0 * locals.var_tmf1_dn6) + (3.0 * locals.var_tmf2_dn6)) + (4.0 * locals.var_tmf3_dn6))) * locals.var_tmf0) + (assign81020_e123606 * locals.var_tmf0_dn6)) * locals.var_tmf0) + (assign81020_e123608 * locals.var_tmf0_dn6)), (((((-(((2.0 * locals.var_tmf1_dn7) + (3.0 * locals.var_tmf2_dn7)) + (4.0 * locals.var_tmf3_dn7))) * locals.var_tmf0) + (assign81020_e123606 * locals.var_tmf0_dn7)) * locals.var_tmf0) + (assign81020_e123608 * locals.var_tmf0_dn7)), (((((-(((2.0 * locals.var_tmf1_dn8) + (3.0 * locals.var_tmf2_dn8)) + (4.0 * locals.var_tmf3_dn8))) * locals.var_tmf0) + (assign81020_e123606 * locals.var_tmf0_dn8)) * locals.var_tmf0) + (assign81020_e123608 * locals.var_tmf0_dn8)), (((((-(((2.0 * locals.var_tmf1_dn9) + (3.0 * locals.var_tmf2_dn9)) + (4.0 * locals.var_tmf3_dn9))) * locals.var_tmf0) + (assign81020_e123606 * locals.var_tmf0_dn9)) * locals.var_tmf0) + (assign81020_e123608 * locals.var_tmf0_dn9)), (((((-(((2.0 * locals.var_tmf1_dn10) + (3.0 * locals.var_tmf2_dn10)) + (4.0 * locals.var_tmf3_dn10))) * locals.var_tmf0) + (assign81020_e123606 * locals.var_tmf0_dn10)) * locals.var_tmf0) + (assign81020_e123608 * locals.var_tmf0_dn10)), (((((-(((2.0 * locals.var_tmf1_dn11) + (3.0 * locals.var_tmf2_dn11)) + (4.0 * locals.var_tmf3_dn11))) * locals.var_tmf0) + (assign81020_e123606 * locals.var_tmf0_dn11)) * locals.var_tmf0) + (assign81020_e123608 * locals.var_tmf0_dn11)), (((((-(((2.0 * locals.var_tmf1_dn14) + (3.0 * locals.var_tmf2_dn14)) + (4.0 * locals.var_tmf3_dn14))) * locals.var_tmf0) + (assign81020_e123606 * locals.var_tmf0_dn14)) * locals.var_tmf0) + (assign81020_e123608 * locals.var_tmf0_dn14)),)
    } else {
        (locals.var_t11, locals.var_t11_dn0, locals.var_t11_dn2, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11, locals.var_t11_dn14,)
    }
};
        locals.var_t11 = assign81020_e123612;
        locals.var_t11_dn0 = assign81020_e123612_d_n0;
        locals.var_t11_dn2 = assign81020_e123612_d_n2;
        locals.var_t11_dn4 = assign81020_e123612_d_n4;
        locals.var_t11_dn5 = assign81020_e123612_d_n5;
        locals.var_t11_dn6 = assign81020_e123612_d_n6;
        locals.var_t11_dn7 = assign81020_e123612_d_n7;
        locals.var_t11_dn8 = assign81020_e123612_d_n8;
        locals.var_t11_dn9 = assign81020_e123612_d_n9;
        locals.var_t11_dn10 = assign81020_e123612_d_n10;
        locals.var_t11_dn11 = assign81020_e123612_d_n11;
        locals.var_t11_dn14 = assign81020_e123612_d_n14;

        let (assign81030_e123624, assign81030_e123624_d_n0, assign81030_e123624_d_n2, assign81030_e123624_d_n4, assign81030_e123624_d_n5, assign81030_e123624_d_n6, assign81030_e123624_d_n7, assign81030_e123624_d_n8, assign81030_e123624_d_n9, assign81030_e123624_d_n10, assign81030_e123624_d_n11, assign81030_e123624_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1900 != 0.0)) && (locals.var_guard1901 != 0.0)) {
        let assign81030_e123621: f64 = (1.0 - locals.var_tmf0);
        let assign81030_e123622: f64 = (locals.var_t2 * assign81030_e123621);
        (assign81030_e123622, ((locals.var_t2_dn0 * assign81030_e123621) + (locals.var_t2 * (-locals.var_tmf0_dn0))), ((locals.var_t2_dn2 * assign81030_e123621) + (locals.var_t2 * (-locals.var_tmf0_dn2))), ((locals.var_t2_dn4 * assign81030_e123621) + (locals.var_t2 * (-locals.var_tmf0_dn4))), ((locals.var_t2_dn5 * assign81030_e123621) + (locals.var_t2 * (-locals.var_tmf0_dn5))), ((locals.var_t2_dn6 * assign81030_e123621) + (locals.var_t2 * (-locals.var_tmf0_dn6))), ((locals.var_t2_dn7 * assign81030_e123621) + (locals.var_t2 * (-locals.var_tmf0_dn7))), ((locals.var_t2_dn8 * assign81030_e123621) + (locals.var_t2 * (-locals.var_tmf0_dn8))), ((locals.var_t2_dn9 * assign81030_e123621) + (locals.var_t2 * (-locals.var_tmf0_dn9))), ((locals.var_t2_dn10 * assign81030_e123621) + (locals.var_t2 * (-locals.var_tmf0_dn10))), ((locals.var_t2_dn11 * assign81030_e123621) + (locals.var_t2 * (-locals.var_tmf0_dn11))), ((locals.var_t2_dn14 * assign81030_e123621) + (locals.var_t2 * (-locals.var_tmf0_dn14))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn14,)
    }
};
        locals.var_ty = assign81030_e123624;
        locals.var_ty_dn0 = assign81030_e123624_d_n0;
        locals.var_ty_dn2 = assign81030_e123624_d_n2;
        locals.var_ty_dn4 = assign81030_e123624_d_n4;
        locals.var_ty_dn5 = assign81030_e123624_d_n5;
        locals.var_ty_dn6 = assign81030_e123624_d_n6;
        locals.var_ty_dn7 = assign81030_e123624_d_n7;
        locals.var_ty_dn8 = assign81030_e123624_d_n8;
        locals.var_ty_dn9 = assign81030_e123624_d_n9;
        locals.var_ty_dn10 = assign81030_e123624_d_n10;
        locals.var_ty_dn11 = assign81030_e123624_d_n11;
        locals.var_ty_dn14 = assign81030_e123624_d_n14;

        let (assign81040_e123638, assign81040_e123638_d_n0, assign81040_e123638_d_n2, assign81040_e123638_d_n4, assign81040_e123638_d_n5, assign81040_e123638_d_n6, assign81040_e123638_d_n7, assign81040_e123638_d_n8, assign81040_e123638_d_n9, assign81040_e123638_d_n10, assign81040_e123638_d_n11, assign81040_e123638_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1900 != 0.0)) && (locals.var_guard1901 != 0.0)) {
        let assign81040_e123632: f64 = (1.0 - locals.var_tmf0);
        let assign81040_e123635: f64 = (locals.var_tmf1 * locals.var_t11);
        let assign81040_e123636: f64 = (assign81040_e123632 + assign81040_e123635);
        (assign81040_e123636, ((-locals.var_tmf0_dn0) + ((locals.var_tmf1_dn0 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn0))), ((-locals.var_tmf0_dn2) + ((locals.var_tmf1_dn2 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn2))), ((-locals.var_tmf0_dn4) + ((locals.var_tmf1_dn4 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn4))), ((-locals.var_tmf0_dn5) + ((locals.var_tmf1_dn5 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn5))), ((-locals.var_tmf0_dn6) + ((locals.var_tmf1_dn6 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn6))), ((-locals.var_tmf0_dn7) + ((locals.var_tmf1_dn7 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn7))), ((-locals.var_tmf0_dn8) + ((locals.var_tmf1_dn8 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn8))), ((-locals.var_tmf0_dn9) + ((locals.var_tmf1_dn9 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn9))), ((-locals.var_tmf0_dn10) + ((locals.var_tmf1_dn10 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn10))), ((-locals.var_tmf0_dn11) + ((locals.var_tmf1_dn11 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn11))), ((-locals.var_tmf0_dn14) + ((locals.var_tmf1_dn14 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn14))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign81040_e123638;
        locals.var_t0_dn0 = assign81040_e123638_d_n0;
        locals.var_t0_dn2 = assign81040_e123638_d_n2;
        locals.var_t0_dn4 = assign81040_e123638_d_n4;
        locals.var_t0_dn5 = assign81040_e123638_d_n5;
        locals.var_t0_dn6 = assign81040_e123638_d_n6;
        locals.var_t0_dn7 = assign81040_e123638_d_n7;
        locals.var_t0_dn8 = assign81040_e123638_d_n8;
        locals.var_t0_dn9 = assign81040_e123638_d_n9;
        locals.var_t0_dn10 = assign81040_e123638_d_n10;
        locals.var_t0_dn11 = assign81040_e123638_d_n11;
        locals.var_t0_dn14 = assign81040_e123638_d_n14;

        let (assign81050_e123647, assign81050_e123647_d_n0, assign81050_e123647_d_n2, assign81050_e123647_d_n4, assign81050_e123647_d_n5, assign81050_e123647_d_n6, assign81050_e123647_d_n7, assign81050_e123647_d_n8, assign81050_e123647_d_n9, assign81050_e123647_d_n10, assign81050_e123647_d_n11, assign81050_e123647_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1900 != 0.0)) && (locals.var_guard1901 != 0.0)) {
        let assign81050_e123645: f64 = (-locals.var_t11);
        (assign81050_e123645, (-locals.var_t11_dn0), (-locals.var_t11_dn2), (-locals.var_t11_dn4), (-locals.var_t11_dn5), (-locals.var_t11_dn6), (-locals.var_t11_dn7), (-locals.var_t11_dn8), (-locals.var_t11_dn9), (-locals.var_t11_dn10), (-locals.var_t11_dn11), (-locals.var_t11_dn14),)
    } else {
        (locals.var_t11, locals.var_t11_dn0, locals.var_t11_dn2, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11, locals.var_t11_dn14,)
    }
};
        locals.var_t11 = assign81050_e123647;
        locals.var_t11_dn0 = assign81050_e123647_d_n0;
        locals.var_t11_dn2 = assign81050_e123647_d_n2;
        locals.var_t11_dn4 = assign81050_e123647_d_n4;
        locals.var_t11_dn5 = assign81050_e123647_d_n5;
        locals.var_t11_dn6 = assign81050_e123647_d_n6;
        locals.var_t11_dn7 = assign81050_e123647_d_n7;
        locals.var_t11_dn8 = assign81050_e123647_d_n8;
        locals.var_t11_dn9 = assign81050_e123647_d_n9;
        locals.var_t11_dn10 = assign81050_e123647_d_n10;
        locals.var_t11_dn11 = assign81050_e123647_d_n11;
        locals.var_t11_dn14 = assign81050_e123647_d_n14;

        let (assign81060_e123657, assign81060_e123657_d_n0, assign81060_e123657_d_n2, assign81060_e123657_d_n4, assign81060_e123657_d_n5, assign81060_e123657_d_n6, assign81060_e123657_d_n7, assign81060_e123657_d_n8, assign81060_e123657_d_n9, assign81060_e123657_d_n10, assign81060_e123657_d_n11, assign81060_e123657_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1900 != 0.0)) && (locals.var_guard1901 != 0.0)) {
        let assign81060_e123655: f64 = (locals.var_vbs_bnd_over__blk1890 + locals.var_ty);
        (assign81060_e123655, (locals.var_vbs_bnd_over__blk1890_dn0 + locals.var_ty_dn0), (locals.var_vbs_bnd_over__blk1890_dn2 + locals.var_ty_dn2), (locals.var_vbs_bnd_over__blk1890_dn4 + locals.var_ty_dn4), (locals.var_vbs_bnd_over__blk1890_dn5 + locals.var_ty_dn5), (locals.var_vbs_bnd_over__blk1890_dn6 + locals.var_ty_dn6), (locals.var_vbs_bnd_over__blk1890_dn7 + locals.var_ty_dn7), (locals.var_vbs_bnd_over__blk1890_dn8 + locals.var_ty_dn8), (locals.var_vbs_bnd_over__blk1890_dn9 + locals.var_ty_dn9), (locals.var_vbs_bnd_over__blk1890_dn10 + locals.var_ty_dn10), (locals.var_vbs_bnd_over__blk1890_dn11 + locals.var_ty_dn11), (locals.var_vbs_bnd_over__blk1890_dn14 + locals.var_ty_dn14),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign81060_e123657;
        locals.var_t10_dn0 = assign81060_e123657_d_n0;
        locals.var_t10_dn2 = assign81060_e123657_d_n2;
        locals.var_t10_dn4 = assign81060_e123657_d_n4;
        locals.var_t10_dn5 = assign81060_e123657_d_n5;
        locals.var_t10_dn6 = assign81060_e123657_d_n6;
        locals.var_t10_dn7 = assign81060_e123657_d_n7;
        locals.var_t10_dn8 = assign81060_e123657_d_n8;
        locals.var_t10_dn9 = assign81060_e123657_d_n9;
        locals.var_t10_dn10 = assign81060_e123657_d_n10;
        locals.var_t10_dn11 = assign81060_e123657_d_n11;
        locals.var_t10_dn14 = assign81060_e123657_d_n14;

        let (assign81070_e123666, assign81070_e123666_d_n0, assign81070_e123666_d_n2, assign81070_e123666_d_n4, assign81070_e123666_d_n5, assign81070_e123666_d_n6, assign81070_e123666_d_n7, assign81070_e123666_d_n8, assign81070_e123666_d_n9, assign81070_e123666_d_n10, assign81070_e123666_d_n11, assign81070_e123666_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1900 != 0.0)) && (locals.var_guard1901 == 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign81070_e123666;
        locals.var_t10_dn0 = assign81070_e123666_d_n0;
        locals.var_t10_dn2 = assign81070_e123666_d_n2;
        locals.var_t10_dn4 = assign81070_e123666_d_n4;
        locals.var_t10_dn5 = assign81070_e123666_d_n5;
        locals.var_t10_dn6 = assign81070_e123666_d_n6;
        locals.var_t10_dn7 = assign81070_e123666_d_n7;
        locals.var_t10_dn8 = assign81070_e123666_d_n8;
        locals.var_t10_dn9 = assign81070_e123666_d_n9;
        locals.var_t10_dn10 = assign81070_e123666_d_n10;
        locals.var_t10_dn11 = assign81070_e123666_d_n11;
        locals.var_t10_dn14 = assign81070_e123666_d_n14;

        let (assign81080_e123673, assign81080_e123673_d_n0, assign81080_e123673_d_n2, assign81080_e123673_d_n4, assign81080_e123673_d_n5, assign81080_e123673_d_n6, assign81080_e123673_d_n7, assign81080_e123673_d_n8, assign81080_e123673_d_n9, assign81080_e123673_d_n10, assign81080_e123673_d_n11, assign81080_e123673_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1900 != 0.0)) {
        let assign81080_e123671: f64 = (-locals.var_t10);
        (assign81080_e123671, (-locals.var_t10_dn0), (-locals.var_t10_dn2), (-locals.var_t10_dn4), (-locals.var_t10_dn5), (-locals.var_t10_dn6), (-locals.var_t10_dn7), (-locals.var_t10_dn8), (-locals.var_t10_dn9), (-locals.var_t10_dn10), (-locals.var_t10_dn11), (-locals.var_t10_dn14),)
    } else {
        (locals.var_vxbgmtcl, locals.var_vxbgmtcl_dn0, locals.var_vxbgmtcl_dn2, locals.var_vxbgmtcl_dn4, locals.var_vxbgmtcl_dn5, locals.var_vxbgmtcl_dn6, locals.var_vxbgmtcl_dn7, locals.var_vxbgmtcl_dn8, locals.var_vxbgmtcl_dn9, locals.var_vxbgmtcl_dn10, locals.var_vxbgmtcl_dn11, locals.var_vxbgmtcl_dn14,)
    }
};
        locals.var_vxbgmtcl = assign81080_e123673;
        locals.var_vxbgmtcl_dn0 = assign81080_e123673_d_n0;
        locals.var_vxbgmtcl_dn2 = assign81080_e123673_d_n2;
        locals.var_vxbgmtcl_dn4 = assign81080_e123673_d_n4;
        locals.var_vxbgmtcl_dn5 = assign81080_e123673_d_n5;
        locals.var_vxbgmtcl_dn6 = assign81080_e123673_d_n6;
        locals.var_vxbgmtcl_dn7 = assign81080_e123673_d_n7;
        locals.var_vxbgmtcl_dn8 = assign81080_e123673_d_n8;
        locals.var_vxbgmtcl_dn9 = assign81080_e123673_d_n9;
        locals.var_vxbgmtcl_dn10 = assign81080_e123673_d_n10;
        locals.var_vxbgmtcl_dn11 = assign81080_e123673_d_n11;
        locals.var_vxbgmtcl_dn14 = assign81080_e123673_d_n14;

        let (assign81090_e123680, assign81090_e123680_d_n0, assign81090_e123680_d_n2, assign81090_e123680_d_n4, assign81090_e123680_d_n5, assign81090_e123680_d_n6, assign81090_e123680_d_n7, assign81090_e123680_d_n8, assign81090_e123680_d_n9, assign81090_e123680_d_n10, assign81090_e123680_d_n11, assign81090_e123680_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1900 == 0.0)) {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn11, locals.var_vxbgmt_dn14,)
    } else {
        (locals.var_vxbgmtcl, locals.var_vxbgmtcl_dn0, locals.var_vxbgmtcl_dn2, locals.var_vxbgmtcl_dn4, locals.var_vxbgmtcl_dn5, locals.var_vxbgmtcl_dn6, locals.var_vxbgmtcl_dn7, locals.var_vxbgmtcl_dn8, locals.var_vxbgmtcl_dn9, locals.var_vxbgmtcl_dn10, locals.var_vxbgmtcl_dn11, locals.var_vxbgmtcl_dn14,)
    }
};
        locals.var_vxbgmtcl = assign81090_e123680;
        locals.var_vxbgmtcl_dn0 = assign81090_e123680_d_n0;
        locals.var_vxbgmtcl_dn2 = assign81090_e123680_d_n2;
        locals.var_vxbgmtcl_dn4 = assign81090_e123680_d_n4;
        locals.var_vxbgmtcl_dn5 = assign81090_e123680_d_n5;
        locals.var_vxbgmtcl_dn6 = assign81090_e123680_d_n6;
        locals.var_vxbgmtcl_dn7 = assign81090_e123680_d_n7;
        locals.var_vxbgmtcl_dn8 = assign81090_e123680_d_n8;
        locals.var_vxbgmtcl_dn9 = assign81090_e123680_d_n9;
        locals.var_vxbgmtcl_dn10 = assign81090_e123680_d_n10;
        locals.var_vxbgmtcl_dn11 = assign81090_e123680_d_n11;
        locals.var_vxbgmtcl_dn14 = assign81090_e123680_d_n14;

        let (assign81100_e123686, assign81100_e123686_d_n0, assign81100_e123686_d_n2, assign81100_e123686_d_n4, assign81100_e123686_d_n5, assign81100_e123686_d_n6, assign81100_e123686_d_n7, assign81100_e123686_d_n8, assign81100_e123686_d_n9, assign81100_e123686_d_n10, assign81100_e123686_d_n11, assign81100_e123686_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign81100_e123684: f64 = (locals.var_cnst0over_func / locals.var_cox0_func);
        (assign81100_e123684, (locals.var_cnst0over_func_dn0 / locals.var_cox0_func), (locals.var_cnst0over_func_dn2 / locals.var_cox0_func), (locals.var_cnst0over_func_dn4 / locals.var_cox0_func), (locals.var_cnst0over_func_dn5 / locals.var_cox0_func), (locals.var_cnst0over_func_dn6 / locals.var_cox0_func), (locals.var_cnst0over_func_dn7 / locals.var_cox0_func), (locals.var_cnst0over_func_dn8 / locals.var_cox0_func), (locals.var_cnst0over_func_dn9 / locals.var_cox0_func), (locals.var_cnst0over_func_dn10 / locals.var_cox0_func), (locals.var_cnst0over_func_dn11 / locals.var_cox0_func), (locals.var_cnst0over_func_dn14 / locals.var_cox0_func),)
    } else {
        (locals.var_fac1, locals.var_fac1_dn0, locals.var_fac1_dn2, locals.var_fac1_dn4, locals.var_fac1_dn5, locals.var_fac1_dn6, locals.var_fac1_dn7, locals.var_fac1_dn8, locals.var_fac1_dn9, locals.var_fac1_dn10, locals.var_fac1_dn11, locals.var_fac1_dn14,)
    }
};
        locals.var_fac1 = assign81100_e123686;
        locals.var_fac1_dn0 = assign81100_e123686_d_n0;
        locals.var_fac1_dn2 = assign81100_e123686_d_n2;
        locals.var_fac1_dn4 = assign81100_e123686_d_n4;
        locals.var_fac1_dn5 = assign81100_e123686_d_n5;
        locals.var_fac1_dn6 = assign81100_e123686_d_n6;
        locals.var_fac1_dn7 = assign81100_e123686_d_n7;
        locals.var_fac1_dn8 = assign81100_e123686_d_n8;
        locals.var_fac1_dn9 = assign81100_e123686_d_n9;
        locals.var_fac1_dn10 = assign81100_e123686_d_n10;
        locals.var_fac1_dn11 = assign81100_e123686_d_n11;
        locals.var_fac1_dn14 = assign81100_e123686_d_n14;

        let (assign81110_e123692, assign81110_e123692_d_n0, assign81110_e123692_d_n2, assign81110_e123692_d_n4, assign81110_e123692_d_n5, assign81110_e123692_d_n6, assign81110_e123692_d_n7, assign81110_e123692_d_n8, assign81110_e123692_d_n9, assign81110_e123692_d_n10, assign81110_e123692_d_n11, assign81110_e123692_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign81110_e123690: f64 = (locals.var_fac1 * locals.var_fac1);
        (assign81110_e123690, ((locals.var_fac1_dn0 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn0)), ((locals.var_fac1_dn2 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn2)), ((locals.var_fac1_dn4 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn4)), ((locals.var_fac1_dn5 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn5)), ((locals.var_fac1_dn6 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn6)), ((locals.var_fac1_dn7 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn7)), ((locals.var_fac1_dn8 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn8)), ((locals.var_fac1_dn9 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn9)), ((locals.var_fac1_dn10 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn10)), ((locals.var_fac1_dn11 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn11)), ((locals.var_fac1_dn14 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn14)),)
    } else {
        (locals.var_fac1p2, locals.var_fac1p2_dn0, locals.var_fac1p2_dn2, locals.var_fac1p2_dn4, locals.var_fac1p2_dn5, locals.var_fac1p2_dn6, locals.var_fac1p2_dn7, locals.var_fac1p2_dn8, locals.var_fac1p2_dn9, locals.var_fac1p2_dn10, locals.var_fac1p2_dn11, locals.var_fac1p2_dn14,)
    }
};
        locals.var_fac1p2 = assign81110_e123692;
        locals.var_fac1p2_dn0 = assign81110_e123692_d_n0;
        locals.var_fac1p2_dn2 = assign81110_e123692_d_n2;
        locals.var_fac1p2_dn4 = assign81110_e123692_d_n4;
        locals.var_fac1p2_dn5 = assign81110_e123692_d_n5;
        locals.var_fac1p2_dn6 = assign81110_e123692_d_n6;
        locals.var_fac1p2_dn7 = assign81110_e123692_d_n7;
        locals.var_fac1p2_dn8 = assign81110_e123692_d_n8;
        locals.var_fac1p2_dn9 = assign81110_e123692_d_n9;
        locals.var_fac1p2_dn10 = assign81110_e123692_d_n10;
        locals.var_fac1p2_dn11 = assign81110_e123692_d_n11;
        locals.var_fac1p2_dn14 = assign81110_e123692_d_n14;

        let (assign81120_e123699, assign81120_e123699_d_n2, assign81120_e123699_d_n7, assign81120_e123699_d_n8, assign81120_e123699_d_n9,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign81120_e123695: f64 = (-locals.var_vgbgmt);
        let assign81120_e123697: f64 = (assign81120_e123695 + locals.var_uc_vfbover);
        (assign81120_e123697, (-locals.var_vgbgmt_dn2), (-locals.var_vgbgmt_dn7), (-locals.var_vgbgmt_dn8), (-locals.var_vgbgmt_dn9),)
    } else {
        (locals.var_vgpld, locals.var_vgpld_dn2, locals.var_vgpld_dn7, locals.var_vgpld_dn8, locals.var_vgpld_dn9,)
    }
};
        locals.var_vgpld = assign81120_e123699;
        locals.var_vgpld_dn2 = assign81120_e123699_d_n2;
        locals.var_vgpld_dn7 = assign81120_e123699_d_n7;
        locals.var_vgpld_dn8 = assign81120_e123699_d_n8;
        locals.var_vgpld_dn9 = assign81120_e123699_d_n9;

        let (assign81130_e123708,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign81130_e123702: f64 = (-locals.var_vxbgmtcl);
        let assign81130_e123705: f64 = (10.0 * 2.220446049250313e-16);
        let assign81130_e123706: f64 = (assign81130_e123702 + assign81130_e123705);
        (assign81130_e123706,)
    } else {
        (locals.var_vgb_fb_ld,)
    }
};
        locals.var_vgb_fb_ld = assign81130_e123708;

        let (assign81140_e123712, assign81140_e123712_d_n0, assign81140_e123712_d_n2, assign81140_e123712_d_n4, assign81140_e123712_d_n5, assign81140_e123712_d_n6, assign81140_e123712_d_n7, assign81140_e123712_d_n8, assign81140_e123712_d_n9, assign81140_e123712_d_n10, assign81140_e123712_d_n11, assign81140_e123712_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_dep_ld__blk1884, locals.var_q_dep_ld__blk1884_dn0, locals.var_q_dep_ld__blk1884_dn2, locals.var_q_dep_ld__blk1884_dn4, locals.var_q_dep_ld__blk1884_dn5, locals.var_q_dep_ld__blk1884_dn6, locals.var_q_dep_ld__blk1884_dn7, locals.var_q_dep_ld__blk1884_dn8, locals.var_q_dep_ld__blk1884_dn9, locals.var_q_dep_ld__blk1884_dn10, locals.var_q_dep_ld__blk1884_dn11, locals.var_q_dep_ld__blk1884_dn14,)
    }
};
        locals.var_q_dep_ld__blk1884 = assign81140_e123712;
        locals.var_q_dep_ld__blk1884_dn0 = assign81140_e123712_d_n0;
        locals.var_q_dep_ld__blk1884_dn2 = assign81140_e123712_d_n2;
        locals.var_q_dep_ld__blk1884_dn4 = assign81140_e123712_d_n4;
        locals.var_q_dep_ld__blk1884_dn5 = assign81140_e123712_d_n5;
        locals.var_q_dep_ld__blk1884_dn6 = assign81140_e123712_d_n6;
        locals.var_q_dep_ld__blk1884_dn7 = assign81140_e123712_d_n7;
        locals.var_q_dep_ld__blk1884_dn8 = assign81140_e123712_d_n8;
        locals.var_q_dep_ld__blk1884_dn9 = assign81140_e123712_d_n9;
        locals.var_q_dep_ld__blk1884_dn10 = assign81140_e123712_d_n10;
        locals.var_q_dep_ld__blk1884_dn11 = assign81140_e123712_d_n11;
        locals.var_q_dep_ld__blk1884_dn14 = assign81140_e123712_d_n14;

        let (assign81150_e123718,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign81150_e123716: f64 = (1.6021918e-19 * locals.var_nover_func);
        (assign81150_e123716,)
    } else {
        (locals.var_q_nsubld__blk1885,)
    }
};
        locals.var_q_nsubld__blk1885 = assign81150_e123718;

        let (assign81160_e123724, assign81160_e123724_d_n0, assign81160_e123724_d_n2, assign81160_e123724_d_n4, assign81160_e123724_d_n5, assign81160_e123724_d_n6, assign81160_e123724_d_n7, assign81160_e123724_d_n8, assign81160_e123724_d_n9, assign81160_e123724_d_n10, assign81160_e123724_d_n11, assign81160_e123724_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign81160_e123722: f64 = (locals.var_nin / locals.var_nover_func);
        (assign81160_e123722, (locals.var_nin_dn0 / locals.var_nover_func), (locals.var_nin_dn2 / locals.var_nover_func), (locals.var_nin_dn4 / locals.var_nover_func), (locals.var_nin_dn5 / locals.var_nover_func), (locals.var_nin_dn6 / locals.var_nover_func), (locals.var_nin_dn7 / locals.var_nover_func), (locals.var_nin_dn8 / locals.var_nover_func), (locals.var_nin_dn9 / locals.var_nover_func), (locals.var_nin_dn10 / locals.var_nover_func), (locals.var_nin_dn11 / locals.var_nover_func), (locals.var_nin_dn14 / locals.var_nover_func),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign81160_e123724;
        locals.var_t0_dn0 = assign81160_e123724_d_n0;
        locals.var_t0_dn2 = assign81160_e123724_d_n2;
        locals.var_t0_dn4 = assign81160_e123724_d_n4;
        locals.var_t0_dn5 = assign81160_e123724_d_n5;
        locals.var_t0_dn6 = assign81160_e123724_d_n6;
        locals.var_t0_dn7 = assign81160_e123724_d_n7;
        locals.var_t0_dn8 = assign81160_e123724_d_n8;
        locals.var_t0_dn9 = assign81160_e123724_d_n9;
        locals.var_t0_dn10 = assign81160_e123724_d_n10;
        locals.var_t0_dn11 = assign81160_e123724_d_n11;
        locals.var_t0_dn14 = assign81160_e123724_d_n14;

        let (assign81170_e123730, assign81170_e123730_d_n0, assign81170_e123730_d_n2, assign81170_e123730_d_n4, assign81170_e123730_d_n5, assign81170_e123730_d_n6, assign81170_e123730_d_n7, assign81170_e123730_d_n8, assign81170_e123730_d_n9, assign81170_e123730_d_n10, assign81170_e123730_d_n11, assign81170_e123730_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign81170_e123728: f64 = (locals.var_t0 * locals.var_t0);
        (assign81170_e123728, ((locals.var_t0_dn0 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn0)), ((locals.var_t0_dn2 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn2)), ((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)), ((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)), ((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)), ((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)), ((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)), ((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)), ((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)), ((locals.var_t0_dn11 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn11)), ((locals.var_t0_dn14 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn14)),)
    } else {
        (locals.var_cnst1over, locals.var_cnst1over_dn0, locals.var_cnst1over_dn2, locals.var_cnst1over_dn4, locals.var_cnst1over_dn5, locals.var_cnst1over_dn6, locals.var_cnst1over_dn7, locals.var_cnst1over_dn8, locals.var_cnst1over_dn9, locals.var_cnst1over_dn10, locals.var_cnst1over_dn11, locals.var_cnst1over_dn14,)
    }
};
        locals.var_cnst1over = assign81170_e123730;
        locals.var_cnst1over_dn0 = assign81170_e123730_d_n0;
        locals.var_cnst1over_dn2 = assign81170_e123730_d_n2;
        locals.var_cnst1over_dn4 = assign81170_e123730_d_n4;
        locals.var_cnst1over_dn5 = assign81170_e123730_d_n5;
        locals.var_cnst1over_dn6 = assign81170_e123730_d_n6;
        locals.var_cnst1over_dn7 = assign81170_e123730_d_n7;
        locals.var_cnst1over_dn8 = assign81170_e123730_d_n8;
        locals.var_cnst1over_dn9 = assign81170_e123730_d_n9;
        locals.var_cnst1over_dn10 = assign81170_e123730_d_n10;
        locals.var_cnst1over_dn11 = assign81170_e123730_d_n11;
        locals.var_cnst1over_dn14 = assign81170_e123730_d_n14;

        let assign81180_e123733: f64 = (-locals.var_vxbgmtcl);
        let assign81180_e123734: f64 = (locals.var_beta * assign81180_e123733);
        let assign81180_e123736: f64 = if assign81180_e123734 >= 500.0 { 1.0 } else { 0.0 };
        locals.var_guard1902 = assign81180_e123736;

        let (assign81190_e123751, assign81190_e123751_d_n0, assign81190_e123751_d_n2, assign81190_e123751_d_n4, assign81190_e123751_d_n5, assign81190_e123751_d_n6, assign81190_e123751_d_n7, assign81190_e123751_d_n8, assign81190_e123751_d_n9, assign81190_e123751_d_n10, assign81190_e123751_d_n11, assign81190_e123751_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1902 != 0.0)) {
        let assign81190_e123744: f64 = (-locals.var_vxbgmtcl);
        let assign81190_e123745: f64 = (locals.var_beta * assign81190_e123744);
        let assign81190_e123746: f64 = (1.0 + assign81190_e123745);
        let assign81190_e123748: f64 = (assign81190_e123746 - 500.0);
        let assign81190_e123749: f64 = (1.403592217853e217 * assign81190_e123748);
        (assign81190_e123749, (1.403592217853e217 * ((locals.var_beta_dn0 * assign81190_e123744) + (locals.var_beta * (-locals.var_vxbgmtcl_dn0)))), (1.403592217853e217 * ((locals.var_beta_dn2 * assign81190_e123744) + (locals.var_beta * (-locals.var_vxbgmtcl_dn2)))), (1.403592217853e217 * ((locals.var_beta_dn4 * assign81190_e123744) + (locals.var_beta * (-locals.var_vxbgmtcl_dn4)))), (1.403592217853e217 * ((locals.var_beta_dn5 * assign81190_e123744) + (locals.var_beta * (-locals.var_vxbgmtcl_dn5)))), (1.403592217853e217 * ((locals.var_beta_dn6 * assign81190_e123744) + (locals.var_beta * (-locals.var_vxbgmtcl_dn6)))), (1.403592217853e217 * ((locals.var_beta_dn7 * assign81190_e123744) + (locals.var_beta * (-locals.var_vxbgmtcl_dn7)))), (1.403592217853e217 * ((locals.var_beta_dn8 * assign81190_e123744) + (locals.var_beta * (-locals.var_vxbgmtcl_dn8)))), (1.403592217853e217 * ((locals.var_beta_dn9 * assign81190_e123744) + (locals.var_beta * (-locals.var_vxbgmtcl_dn9)))), (1.403592217853e217 * ((locals.var_beta_dn10 * assign81190_e123744) + (locals.var_beta * (-locals.var_vxbgmtcl_dn10)))), (1.403592217853e217 * ((locals.var_beta_dn11 * assign81190_e123744) + (locals.var_beta * (-locals.var_vxbgmtcl_dn11)))), (1.403592217853e217 * ((locals.var_beta_dn14 * assign81190_e123744) + (locals.var_beta * (-locals.var_vxbgmtcl_dn14)))),)
    } else {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn11, locals.var_exp_bvbs_dn14,)
    }
};
        locals.var_exp_bvbs = assign81190_e123751;
        locals.var_exp_bvbs_dn0 = assign81190_e123751_d_n0;
        locals.var_exp_bvbs_dn2 = assign81190_e123751_d_n2;
        locals.var_exp_bvbs_dn4 = assign81190_e123751_d_n4;
        locals.var_exp_bvbs_dn5 = assign81190_e123751_d_n5;
        locals.var_exp_bvbs_dn6 = assign81190_e123751_d_n6;
        locals.var_exp_bvbs_dn7 = assign81190_e123751_d_n7;
        locals.var_exp_bvbs_dn8 = assign81190_e123751_d_n8;
        locals.var_exp_bvbs_dn9 = assign81190_e123751_d_n9;
        locals.var_exp_bvbs_dn10 = assign81190_e123751_d_n10;
        locals.var_exp_bvbs_dn11 = assign81190_e123751_d_n11;
        locals.var_exp_bvbs_dn14 = assign81190_e123751_d_n14;

        let (assign81200_e123757, assign81200_e123757_d_n0, assign81200_e123757_d_n2, assign81200_e123757_d_n4, assign81200_e123757_d_n5, assign81200_e123757_d_n6, assign81200_e123757_d_n7, assign81200_e123757_d_n8, assign81200_e123757_d_n9, assign81200_e123757_d_n10, assign81200_e123757_d_n11, assign81200_e123757_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1902 != 0.0)) {
        (1.403592217853e217, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign81200_e123757;
        locals.var_t0_dn0 = assign81200_e123757_d_n0;
        locals.var_t0_dn2 = assign81200_e123757_d_n2;
        locals.var_t0_dn4 = assign81200_e123757_d_n4;
        locals.var_t0_dn5 = assign81200_e123757_d_n5;
        locals.var_t0_dn6 = assign81200_e123757_d_n6;
        locals.var_t0_dn7 = assign81200_e123757_d_n7;
        locals.var_t0_dn8 = assign81200_e123757_d_n8;
        locals.var_t0_dn9 = assign81200_e123757_d_n9;
        locals.var_t0_dn10 = assign81200_e123757_d_n10;
        locals.var_t0_dn11 = assign81200_e123757_d_n11;
        locals.var_t0_dn14 = assign81200_e123757_d_n14;

        let (assign81210_e123767, assign81210_e123767_d_n0, assign81210_e123767_d_n2, assign81210_e123767_d_n4, assign81210_e123767_d_n5, assign81210_e123767_d_n6, assign81210_e123767_d_n7, assign81210_e123767_d_n8, assign81210_e123767_d_n9, assign81210_e123767_d_n10, assign81210_e123767_d_n11, assign81210_e123767_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1902 == 0.0)) {
        let assign81210_e123764: f64 = (-locals.var_vxbgmtcl);
        let assign81210_e123765: f64 = (locals.var_beta * assign81210_e123764);
        (assign81210_e123765, ((locals.var_beta_dn0 * assign81210_e123764) + (locals.var_beta * (-locals.var_vxbgmtcl_dn0))), ((locals.var_beta_dn2 * assign81210_e123764) + (locals.var_beta * (-locals.var_vxbgmtcl_dn2))), ((locals.var_beta_dn4 * assign81210_e123764) + (locals.var_beta * (-locals.var_vxbgmtcl_dn4))), ((locals.var_beta_dn5 * assign81210_e123764) + (locals.var_beta * (-locals.var_vxbgmtcl_dn5))), ((locals.var_beta_dn6 * assign81210_e123764) + (locals.var_beta * (-locals.var_vxbgmtcl_dn6))), ((locals.var_beta_dn7 * assign81210_e123764) + (locals.var_beta * (-locals.var_vxbgmtcl_dn7))), ((locals.var_beta_dn8 * assign81210_e123764) + (locals.var_beta * (-locals.var_vxbgmtcl_dn8))), ((locals.var_beta_dn9 * assign81210_e123764) + (locals.var_beta * (-locals.var_vxbgmtcl_dn9))), ((locals.var_beta_dn10 * assign81210_e123764) + (locals.var_beta * (-locals.var_vxbgmtcl_dn10))), ((locals.var_beta_dn11 * assign81210_e123764) + (locals.var_beta * (-locals.var_vxbgmtcl_dn11))), ((locals.var_beta_dn14 * assign81210_e123764) + (locals.var_beta * (-locals.var_vxbgmtcl_dn14))),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign81210_e123767;
        locals.var_tmf1_dn0 = assign81210_e123767_d_n0;
        locals.var_tmf1_dn2 = assign81210_e123767_d_n2;
        locals.var_tmf1_dn4 = assign81210_e123767_d_n4;
        locals.var_tmf1_dn5 = assign81210_e123767_d_n5;
        locals.var_tmf1_dn6 = assign81210_e123767_d_n6;
        locals.var_tmf1_dn7 = assign81210_e123767_d_n7;
        locals.var_tmf1_dn8 = assign81210_e123767_d_n8;
        locals.var_tmf1_dn9 = assign81210_e123767_d_n9;
        locals.var_tmf1_dn10 = assign81210_e123767_d_n10;
        locals.var_tmf1_dn11 = assign81210_e123767_d_n11;
        locals.var_tmf1_dn14 = assign81210_e123767_d_n14;

        let (assign81220_e123774, assign81220_e123774_d_n0, assign81220_e123774_d_n2, assign81220_e123774_d_n4, assign81220_e123774_d_n5, assign81220_e123774_d_n6, assign81220_e123774_d_n7, assign81220_e123774_d_n8, assign81220_e123774_d_n9, assign81220_e123774_d_n10, assign81220_e123774_d_n11, assign81220_e123774_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1902 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn11, locals.var_exp_bvbs_dn14,)
    }
};
        locals.var_exp_bvbs = assign81220_e123774;
        locals.var_exp_bvbs_dn0 = assign81220_e123774_d_n0;
        locals.var_exp_bvbs_dn2 = assign81220_e123774_d_n2;
        locals.var_exp_bvbs_dn4 = assign81220_e123774_d_n4;
        locals.var_exp_bvbs_dn5 = assign81220_e123774_d_n5;
        locals.var_exp_bvbs_dn6 = assign81220_e123774_d_n6;
        locals.var_exp_bvbs_dn7 = assign81220_e123774_d_n7;
        locals.var_exp_bvbs_dn8 = assign81220_e123774_d_n8;
        locals.var_exp_bvbs_dn9 = assign81220_e123774_d_n9;
        locals.var_exp_bvbs_dn10 = assign81220_e123774_d_n10;
        locals.var_exp_bvbs_dn11 = assign81220_e123774_d_n11;
        locals.var_exp_bvbs_dn14 = assign81220_e123774_d_n14;

        let mut assign81230_loop_guard: usize = 0;
        while {
            let assign81230_cond_e123782: f64 = if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1902 == 0.0)) && (locals.var_tmf1 >= 60.0)) { 1.0 } else { 0.0 };
            assign81230_cond_e123782 != 0.0
        } {
            assign81230_loop_guard += 1;
            assert!(assign81230_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign81230_body0_e123791, assign81230_body0_e123791_d_n0, assign81230_body0_e123791_d_n2, assign81230_body0_e123791_d_n4, assign81230_body0_e123791_d_n5, assign81230_body0_e123791_d_n6, assign81230_body0_e123791_d_n7, assign81230_body0_e123791_d_n8, assign81230_body0_e123791_d_n9, assign81230_body0_e123791_d_n10, assign81230_body0_e123791_d_n11, assign81230_body0_e123791_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1902 == 0.0)) {
        let assign81230_body0_e123789: f64 = (locals.var_exp_bvbs * 1.14200738981568e26);
        (assign81230_body0_e123789, (locals.var_exp_bvbs_dn0 * 1.14200738981568e26), (locals.var_exp_bvbs_dn2 * 1.14200738981568e26), (locals.var_exp_bvbs_dn4 * 1.14200738981568e26), (locals.var_exp_bvbs_dn5 * 1.14200738981568e26), (locals.var_exp_bvbs_dn6 * 1.14200738981568e26), (locals.var_exp_bvbs_dn7 * 1.14200738981568e26), (locals.var_exp_bvbs_dn8 * 1.14200738981568e26), (locals.var_exp_bvbs_dn9 * 1.14200738981568e26), (locals.var_exp_bvbs_dn10 * 1.14200738981568e26), (locals.var_exp_bvbs_dn11 * 1.14200738981568e26), (locals.var_exp_bvbs_dn14 * 1.14200738981568e26),)
    } else {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn11, locals.var_exp_bvbs_dn14,)
    }
};
            locals.var_exp_bvbs = assign81230_body0_e123791;
            locals.var_exp_bvbs_dn0 = assign81230_body0_e123791_d_n0;
            locals.var_exp_bvbs_dn2 = assign81230_body0_e123791_d_n2;
            locals.var_exp_bvbs_dn4 = assign81230_body0_e123791_d_n4;
            locals.var_exp_bvbs_dn5 = assign81230_body0_e123791_d_n5;
            locals.var_exp_bvbs_dn6 = assign81230_body0_e123791_d_n6;
            locals.var_exp_bvbs_dn7 = assign81230_body0_e123791_d_n7;
            locals.var_exp_bvbs_dn8 = assign81230_body0_e123791_d_n8;
            locals.var_exp_bvbs_dn9 = assign81230_body0_e123791_d_n9;
            locals.var_exp_bvbs_dn10 = assign81230_body0_e123791_d_n10;
            locals.var_exp_bvbs_dn11 = assign81230_body0_e123791_d_n11;
            locals.var_exp_bvbs_dn14 = assign81230_body0_e123791_d_n14;
            let (assign81230_body1_e123800, assign81230_body1_e123800_d_n0, assign81230_body1_e123800_d_n2, assign81230_body1_e123800_d_n4, assign81230_body1_e123800_d_n5, assign81230_body1_e123800_d_n6, assign81230_body1_e123800_d_n7, assign81230_body1_e123800_d_n8, assign81230_body1_e123800_d_n9, assign81230_body1_e123800_d_n10, assign81230_body1_e123800_d_n11, assign81230_body1_e123800_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1902 == 0.0)) {
        let assign81230_body1_e123798: f64 = (locals.var_tmf1 - 60.0);
        (assign81230_body1_e123798, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
            locals.var_tmf1 = assign81230_body1_e123800;
            locals.var_tmf1_dn0 = assign81230_body1_e123800_d_n0;
            locals.var_tmf1_dn2 = assign81230_body1_e123800_d_n2;
            locals.var_tmf1_dn4 = assign81230_body1_e123800_d_n4;
            locals.var_tmf1_dn5 = assign81230_body1_e123800_d_n5;
            locals.var_tmf1_dn6 = assign81230_body1_e123800_d_n6;
            locals.var_tmf1_dn7 = assign81230_body1_e123800_d_n7;
            locals.var_tmf1_dn8 = assign81230_body1_e123800_d_n8;
            locals.var_tmf1_dn9 = assign81230_body1_e123800_d_n9;
            locals.var_tmf1_dn10 = assign81230_body1_e123800_d_n10;
            locals.var_tmf1_dn11 = assign81230_body1_e123800_d_n11;
            locals.var_tmf1_dn14 = assign81230_body1_e123800_d_n14;
        }

        let (assign81240_e123810, assign81240_e123810_d_n0, assign81240_e123810_d_n2, assign81240_e123810_d_n4, assign81240_e123810_d_n5, assign81240_e123810_d_n6, assign81240_e123810_d_n7, assign81240_e123810_d_n8, assign81240_e123810_d_n9, assign81240_e123810_d_n10, assign81240_e123810_d_n11, assign81240_e123810_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1902 == 0.0)) {
        let assign81240_e123807: f64 = (locals.var_tmf1).exp();
        let assign81240_e123808: f64 = (locals.var_exp_bvbs * assign81240_e123807);
        (assign81240_e123808, ((locals.var_exp_bvbs_dn0 * assign81240_e123807) + (locals.var_exp_bvbs * (assign81240_e123807 * locals.var_tmf1_dn0))), ((locals.var_exp_bvbs_dn2 * assign81240_e123807) + (locals.var_exp_bvbs * (assign81240_e123807 * locals.var_tmf1_dn2))), ((locals.var_exp_bvbs_dn4 * assign81240_e123807) + (locals.var_exp_bvbs * (assign81240_e123807 * locals.var_tmf1_dn4))), ((locals.var_exp_bvbs_dn5 * assign81240_e123807) + (locals.var_exp_bvbs * (assign81240_e123807 * locals.var_tmf1_dn5))), ((locals.var_exp_bvbs_dn6 * assign81240_e123807) + (locals.var_exp_bvbs * (assign81240_e123807 * locals.var_tmf1_dn6))), ((locals.var_exp_bvbs_dn7 * assign81240_e123807) + (locals.var_exp_bvbs * (assign81240_e123807 * locals.var_tmf1_dn7))), ((locals.var_exp_bvbs_dn8 * assign81240_e123807) + (locals.var_exp_bvbs * (assign81240_e123807 * locals.var_tmf1_dn8))), ((locals.var_exp_bvbs_dn9 * assign81240_e123807) + (locals.var_exp_bvbs * (assign81240_e123807 * locals.var_tmf1_dn9))), ((locals.var_exp_bvbs_dn10 * assign81240_e123807) + (locals.var_exp_bvbs * (assign81240_e123807 * locals.var_tmf1_dn10))), ((locals.var_exp_bvbs_dn11 * assign81240_e123807) + (locals.var_exp_bvbs * (assign81240_e123807 * locals.var_tmf1_dn11))), ((locals.var_exp_bvbs_dn14 * assign81240_e123807) + (locals.var_exp_bvbs * (assign81240_e123807 * locals.var_tmf1_dn14))),)
    } else {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn11, locals.var_exp_bvbs_dn14,)
    }
};
        locals.var_exp_bvbs = assign81240_e123810;
        locals.var_exp_bvbs_dn0 = assign81240_e123810_d_n0;
        locals.var_exp_bvbs_dn2 = assign81240_e123810_d_n2;
        locals.var_exp_bvbs_dn4 = assign81240_e123810_d_n4;
        locals.var_exp_bvbs_dn5 = assign81240_e123810_d_n5;
        locals.var_exp_bvbs_dn6 = assign81240_e123810_d_n6;
        locals.var_exp_bvbs_dn7 = assign81240_e123810_d_n7;
        locals.var_exp_bvbs_dn8 = assign81240_e123810_d_n8;
        locals.var_exp_bvbs_dn9 = assign81240_e123810_d_n9;
        locals.var_exp_bvbs_dn10 = assign81240_e123810_d_n10;
        locals.var_exp_bvbs_dn11 = assign81240_e123810_d_n11;
        locals.var_exp_bvbs_dn14 = assign81240_e123810_d_n14;

        let (assign81250_e123817, assign81250_e123817_d_n0, assign81250_e123817_d_n2, assign81250_e123817_d_n4, assign81250_e123817_d_n5, assign81250_e123817_d_n6, assign81250_e123817_d_n7, assign81250_e123817_d_n8, assign81250_e123817_d_n9, assign81250_e123817_d_n10, assign81250_e123817_d_n11, assign81250_e123817_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1902 == 0.0)) {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn11, locals.var_exp_bvbs_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign81250_e123817;
        locals.var_t0_dn0 = assign81250_e123817_d_n0;
        locals.var_t0_dn2 = assign81250_e123817_d_n2;
        locals.var_t0_dn4 = assign81250_e123817_d_n4;
        locals.var_t0_dn5 = assign81250_e123817_d_n5;
        locals.var_t0_dn6 = assign81250_e123817_d_n6;
        locals.var_t0_dn7 = assign81250_e123817_d_n7;
        locals.var_t0_dn8 = assign81250_e123817_d_n8;
        locals.var_t0_dn9 = assign81250_e123817_d_n9;
        locals.var_t0_dn10 = assign81250_e123817_d_n10;
        locals.var_t0_dn11 = assign81250_e123817_d_n11;
        locals.var_t0_dn14 = assign81250_e123817_d_n14;

    }

    pub(super) fn stamp_transient_block_294(
        locals: &mut StampLocals,
    ) {
        let (assign81260_e123830, assign81260_e123830_d_n0, assign81260_e123830_d_n2, assign81260_e123830_d_n4, assign81260_e123830_d_n5, assign81260_e123830_d_n6, assign81260_e123830_d_n7, assign81260_e123830_d_n8, assign81260_e123830_d_n9, assign81260_e123830_d_n10, assign81260_e123830_d_n11, assign81260_e123830_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign81260_e123822: f64 = (-locals.var_vgpld);
        let assign81260_e123824: f64 = (assign81260_e123822 * 0.5);
        let assign81260_e123826: f64 = (assign81260_e123824 - 0.5);
        let assign81260_e123828: f64 = (assign81260_e123826 - 1.0);
        (assign81260_e123828, 0.0, ((-locals.var_vgpld_dn2) * 0.5), 0.0, 0.0, 0.0, ((-locals.var_vgpld_dn7) * 0.5), ((-locals.var_vgpld_dn8) * 0.5), ((-locals.var_vgpld_dn9) * 0.5), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign81260_e123830;
        locals.var_tmf1_dn0 = assign81260_e123830_d_n0;
        locals.var_tmf1_dn2 = assign81260_e123830_d_n2;
        locals.var_tmf1_dn4 = assign81260_e123830_d_n4;
        locals.var_tmf1_dn5 = assign81260_e123830_d_n5;
        locals.var_tmf1_dn6 = assign81260_e123830_d_n6;
        locals.var_tmf1_dn7 = assign81260_e123830_d_n7;
        locals.var_tmf1_dn8 = assign81260_e123830_d_n8;
        locals.var_tmf1_dn9 = assign81260_e123830_d_n9;
        locals.var_tmf1_dn10 = assign81260_e123830_d_n10;
        locals.var_tmf1_dn11 = assign81260_e123830_d_n11;
        locals.var_tmf1_dn14 = assign81260_e123830_d_n14;

        let (assign81270_e123840, assign81270_e123840_d_n0, assign81270_e123840_d_n2, assign81270_e123840_d_n4, assign81270_e123840_d_n5, assign81270_e123840_d_n6, assign81270_e123840_d_n7, assign81270_e123840_d_n8, assign81270_e123840_d_n9, assign81270_e123840_d_n10, assign81270_e123840_d_n11, assign81270_e123840_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign81270_e123836: f64 = (4.0 * 0.5);
        let assign81270_e123838: f64 = assign81270_e123836;
        (assign81270_e123838, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign81270_e123840;
        locals.var_tmf2_dn0 = assign81270_e123840_d_n0;
        locals.var_tmf2_dn2 = assign81270_e123840_d_n2;
        locals.var_tmf2_dn4 = assign81270_e123840_d_n4;
        locals.var_tmf2_dn5 = assign81270_e123840_d_n5;
        locals.var_tmf2_dn6 = assign81270_e123840_d_n6;
        locals.var_tmf2_dn7 = assign81270_e123840_d_n7;
        locals.var_tmf2_dn8 = assign81270_e123840_d_n8;
        locals.var_tmf2_dn9 = assign81270_e123840_d_n9;
        locals.var_tmf2_dn10 = assign81270_e123840_d_n10;
        locals.var_tmf2_dn11 = assign81270_e123840_d_n11;
        locals.var_tmf2_dn14 = assign81270_e123840_d_n14;

        let (assign81280_e123852, assign81280_e123852_d_n0, assign81280_e123852_d_n2, assign81280_e123852_d_n4, assign81280_e123852_d_n5, assign81280_e123852_d_n6, assign81280_e123852_d_n7, assign81280_e123852_d_n8, assign81280_e123852_d_n9, assign81280_e123852_d_n10, assign81280_e123852_d_n11, assign81280_e123852_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let (assign81280_e123850, assign81280_e123850_d_n0, assign81280_e123850_d_n2, assign81280_e123850_d_n4, assign81280_e123850_d_n5, assign81280_e123850_d_n6, assign81280_e123850_d_n7, assign81280_e123850_d_n8, assign81280_e123850_d_n9, assign81280_e123850_d_n10, assign81280_e123850_d_n11, assign81280_e123850_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign81280_e123849: f64 = (-locals.var_tmf2);
                (assign81280_e123849, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign81280_e123850, assign81280_e123850_d_n0, assign81280_e123850_d_n2, assign81280_e123850_d_n4, assign81280_e123850_d_n5, assign81280_e123850_d_n6, assign81280_e123850_d_n7, assign81280_e123850_d_n8, assign81280_e123850_d_n9, assign81280_e123850_d_n10, assign81280_e123850_d_n11, assign81280_e123850_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign81280_e123852;
        locals.var_tmf2_dn0 = assign81280_e123852_d_n0;
        locals.var_tmf2_dn2 = assign81280_e123852_d_n2;
        locals.var_tmf2_dn4 = assign81280_e123852_d_n4;
        locals.var_tmf2_dn5 = assign81280_e123852_d_n5;
        locals.var_tmf2_dn6 = assign81280_e123852_d_n6;
        locals.var_tmf2_dn7 = assign81280_e123852_d_n7;
        locals.var_tmf2_dn8 = assign81280_e123852_d_n8;
        locals.var_tmf2_dn9 = assign81280_e123852_d_n9;
        locals.var_tmf2_dn10 = assign81280_e123852_d_n10;
        locals.var_tmf2_dn11 = assign81280_e123852_d_n11;
        locals.var_tmf2_dn14 = assign81280_e123852_d_n14;

        let (assign81290_e123863, assign81290_e123863_d_n0, assign81290_e123863_d_n2, assign81290_e123863_d_n4, assign81290_e123863_d_n5, assign81290_e123863_d_n6, assign81290_e123863_d_n7, assign81290_e123863_d_n8, assign81290_e123863_d_n9, assign81290_e123863_d_n10, assign81290_e123863_d_n11, assign81290_e123863_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign81290_e123858: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign81290_e123860: f64 = (assign81290_e123858 + locals.var_tmf2);
        let assign81290_e123861: f64 = (assign81290_e123860).sqrt();
        (assign81290_e123861, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign81290_e123861)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign81290_e123861)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign81290_e123861)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign81290_e123861)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign81290_e123861)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign81290_e123861)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign81290_e123861)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign81290_e123861)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign81290_e123861)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign81290_e123861)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign81290_e123861)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign81290_e123863;
        locals.var_tmf2_dn0 = assign81290_e123863_d_n0;
        locals.var_tmf2_dn2 = assign81290_e123863_d_n2;
        locals.var_tmf2_dn4 = assign81290_e123863_d_n4;
        locals.var_tmf2_dn5 = assign81290_e123863_d_n5;
        locals.var_tmf2_dn6 = assign81290_e123863_d_n6;
        locals.var_tmf2_dn7 = assign81290_e123863_d_n7;
        locals.var_tmf2_dn8 = assign81290_e123863_d_n8;
        locals.var_tmf2_dn9 = assign81290_e123863_d_n9;
        locals.var_tmf2_dn10 = assign81290_e123863_d_n10;
        locals.var_tmf2_dn11 = assign81290_e123863_d_n11;
        locals.var_tmf2_dn14 = assign81290_e123863_d_n14;

        let (assign81300_e123875, assign81300_e123875_d_n0, assign81300_e123875_d_n2, assign81300_e123875_d_n4, assign81300_e123875_d_n5, assign81300_e123875_d_n6, assign81300_e123875_d_n7, assign81300_e123875_d_n8, assign81300_e123875_d_n9, assign81300_e123875_d_n10, assign81300_e123875_d_n11, assign81300_e123875_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign81300_e123871: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign81300_e123872: f64 = (1.0 + assign81300_e123871);
        let assign81300_e123873: f64 = (0.5 * assign81300_e123872);
        (assign81300_e123873, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign81300_e123875;
        locals.var_t0_dn0 = assign81300_e123875_d_n0;
        locals.var_t0_dn2 = assign81300_e123875_d_n2;
        locals.var_t0_dn4 = assign81300_e123875_d_n4;
        locals.var_t0_dn5 = assign81300_e123875_d_n5;
        locals.var_t0_dn6 = assign81300_e123875_d_n6;
        locals.var_t0_dn7 = assign81300_e123875_d_n7;
        locals.var_t0_dn8 = assign81300_e123875_d_n8;
        locals.var_t0_dn9 = assign81300_e123875_d_n9;
        locals.var_t0_dn10 = assign81300_e123875_d_n10;
        locals.var_t0_dn11 = assign81300_e123875_d_n11;
        locals.var_t0_dn14 = assign81300_e123875_d_n14;

        let (assign81310_e123887, assign81310_e123887_d_n0, assign81310_e123887_d_n2, assign81310_e123887_d_n4, assign81310_e123887_d_n5, assign81310_e123887_d_n6, assign81310_e123887_d_n7, assign81310_e123887_d_n8, assign81310_e123887_d_n9, assign81310_e123887_d_n10, assign81310_e123887_d_n11, assign81310_e123887_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign81310_e123883: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign81310_e123884: f64 = (0.5 * assign81310_e123883);
        let assign81310_e123885: f64 = (0.5 + assign81310_e123884);
        (assign81310_e123885, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign81310_e123887;
        locals.var_t1_dn0 = assign81310_e123887_d_n0;
        locals.var_t1_dn2 = assign81310_e123887_d_n2;
        locals.var_t1_dn4 = assign81310_e123887_d_n4;
        locals.var_t1_dn5 = assign81310_e123887_d_n5;
        locals.var_t1_dn6 = assign81310_e123887_d_n6;
        locals.var_t1_dn7 = assign81310_e123887_d_n7;
        locals.var_t1_dn8 = assign81310_e123887_d_n8;
        locals.var_t1_dn9 = assign81310_e123887_d_n9;
        locals.var_t1_dn10 = assign81310_e123887_d_n10;
        locals.var_t1_dn11 = assign81310_e123887_d_n11;
        locals.var_t1_dn14 = assign81310_e123887_d_n14;

        let assign81320_e123890: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign81320_e123893: f64 = (-locals.var_t1);
        let assign81320_e123898: f64 = if ((assign81320_e123890 > assign81320_e123893) && (locals.var_t1 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1903 = assign81320_e123898;

        let (assign81330_e123912, assign81330_e123912_d_n0, assign81330_e123912_d_n2, assign81330_e123912_d_n4, assign81330_e123912_d_n5, assign81330_e123912_d_n6, assign81330_e123912_d_n7, assign81330_e123912_d_n8, assign81330_e123912_d_n9, assign81330_e123912_d_n10, assign81330_e123912_d_n11, assign81330_e123912_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1903 != 0.0)) {
        let assign81330_e123906: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign81330_e123908: f64 = assign81330_e123906;
        let assign81330_e123910: f64 = (assign81330_e123908 + locals.var_t1);
        (assign81330_e123910, (locals.var_vxbgmtcl_dn0 + locals.var_t1_dn0), ((locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2) + locals.var_t1_dn2), (locals.var_vxbgmtcl_dn4 + locals.var_t1_dn4), (locals.var_vxbgmtcl_dn5 + locals.var_t1_dn5), (locals.var_vxbgmtcl_dn6 + locals.var_t1_dn6), ((locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7) + locals.var_t1_dn7), ((locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8) + locals.var_t1_dn8), ((locals.var_vgpld_dn9 + locals.var_vxbgmtcl_dn9) + locals.var_t1_dn9), (locals.var_vxbgmtcl_dn10 + locals.var_t1_dn10), (locals.var_vxbgmtcl_dn11 + locals.var_t1_dn11), (locals.var_vxbgmtcl_dn14 + locals.var_t1_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign81330_e123912;
        locals.var_tmf1_dn0 = assign81330_e123912_d_n0;
        locals.var_tmf1_dn2 = assign81330_e123912_d_n2;
        locals.var_tmf1_dn4 = assign81330_e123912_d_n4;
        locals.var_tmf1_dn5 = assign81330_e123912_d_n5;
        locals.var_tmf1_dn6 = assign81330_e123912_d_n6;
        locals.var_tmf1_dn7 = assign81330_e123912_d_n7;
        locals.var_tmf1_dn8 = assign81330_e123912_d_n8;
        locals.var_tmf1_dn9 = assign81330_e123912_d_n9;
        locals.var_tmf1_dn10 = assign81330_e123912_d_n10;
        locals.var_tmf1_dn11 = assign81330_e123912_d_n11;
        locals.var_tmf1_dn14 = assign81330_e123912_d_n14;

        let (assign81340_e123922, assign81340_e123922_d_n0, assign81340_e123922_d_n2, assign81340_e123922_d_n4, assign81340_e123922_d_n5, assign81340_e123922_d_n6, assign81340_e123922_d_n7, assign81340_e123922_d_n8, assign81340_e123922_d_n9, assign81340_e123922_d_n10, assign81340_e123922_d_n11, assign81340_e123922_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1903 != 0.0)) {
        let assign81340_e123920: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign81340_e123920, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign81340_e123922;
        locals.var_x2_dn0 = assign81340_e123922_d_n0;
        locals.var_x2_dn2 = assign81340_e123922_d_n2;
        locals.var_x2_dn4 = assign81340_e123922_d_n4;
        locals.var_x2_dn5 = assign81340_e123922_d_n5;
        locals.var_x2_dn6 = assign81340_e123922_d_n6;
        locals.var_x2_dn7 = assign81340_e123922_d_n7;
        locals.var_x2_dn8 = assign81340_e123922_d_n8;
        locals.var_x2_dn9 = assign81340_e123922_d_n9;
        locals.var_x2_dn10 = assign81340_e123922_d_n10;
        locals.var_x2_dn11 = assign81340_e123922_d_n11;
        locals.var_x2_dn14 = assign81340_e123922_d_n14;

        let (assign81350_e123932, assign81350_e123932_d_n0, assign81350_e123932_d_n2, assign81350_e123932_d_n4, assign81350_e123932_d_n5, assign81350_e123932_d_n6, assign81350_e123932_d_n7, assign81350_e123932_d_n8, assign81350_e123932_d_n9, assign81350_e123932_d_n10, assign81350_e123932_d_n11, assign81350_e123932_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1903 != 0.0)) {
        let assign81350_e123930: f64 = (locals.var_t1 * locals.var_t1);
        (assign81350_e123930, ((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)), ((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)), ((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)), ((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)), ((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)), ((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)), ((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)), ((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)), ((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)), ((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)), ((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14)),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign81350_e123932;
        locals.var_xmax2_dn0 = assign81350_e123932_d_n0;
        locals.var_xmax2_dn2 = assign81350_e123932_d_n2;
        locals.var_xmax2_dn4 = assign81350_e123932_d_n4;
        locals.var_xmax2_dn5 = assign81350_e123932_d_n5;
        locals.var_xmax2_dn6 = assign81350_e123932_d_n6;
        locals.var_xmax2_dn7 = assign81350_e123932_d_n7;
        locals.var_xmax2_dn8 = assign81350_e123932_d_n8;
        locals.var_xmax2_dn9 = assign81350_e123932_d_n9;
        locals.var_xmax2_dn10 = assign81350_e123932_d_n10;
        locals.var_xmax2_dn11 = assign81350_e123932_d_n11;
        locals.var_xmax2_dn14 = assign81350_e123932_d_n14;

        let (assign81360_e123940, assign81360_e123940_d_n0, assign81360_e123940_d_n2, assign81360_e123940_d_n4, assign81360_e123940_d_n5, assign81360_e123940_d_n6, assign81360_e123940_d_n7, assign81360_e123940_d_n8, assign81360_e123940_d_n9, assign81360_e123940_d_n10, assign81360_e123940_d_n11, assign81360_e123940_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1903 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign81360_e123940;
        locals.var_xp_dn0 = assign81360_e123940_d_n0;
        locals.var_xp_dn2 = assign81360_e123940_d_n2;
        locals.var_xp_dn4 = assign81360_e123940_d_n4;
        locals.var_xp_dn5 = assign81360_e123940_d_n5;
        locals.var_xp_dn6 = assign81360_e123940_d_n6;
        locals.var_xp_dn7 = assign81360_e123940_d_n7;
        locals.var_xp_dn8 = assign81360_e123940_d_n8;
        locals.var_xp_dn9 = assign81360_e123940_d_n9;
        locals.var_xp_dn10 = assign81360_e123940_d_n10;
        locals.var_xp_dn11 = assign81360_e123940_d_n11;
        locals.var_xp_dn14 = assign81360_e123940_d_n14;

        let (assign81370_e123948, assign81370_e123948_d_n0, assign81370_e123948_d_n2, assign81370_e123948_d_n4, assign81370_e123948_d_n5, assign81370_e123948_d_n6, assign81370_e123948_d_n7, assign81370_e123948_d_n8, assign81370_e123948_d_n9, assign81370_e123948_d_n10, assign81370_e123948_d_n11, assign81370_e123948_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1903 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign81370_e123948;
        locals.var_xmp_dn0 = assign81370_e123948_d_n0;
        locals.var_xmp_dn2 = assign81370_e123948_d_n2;
        locals.var_xmp_dn4 = assign81370_e123948_d_n4;
        locals.var_xmp_dn5 = assign81370_e123948_d_n5;
        locals.var_xmp_dn6 = assign81370_e123948_d_n6;
        locals.var_xmp_dn7 = assign81370_e123948_d_n7;
        locals.var_xmp_dn8 = assign81370_e123948_d_n8;
        locals.var_xmp_dn9 = assign81370_e123948_d_n9;
        locals.var_xmp_dn10 = assign81370_e123948_d_n10;
        locals.var_xmp_dn11 = assign81370_e123948_d_n11;
        locals.var_xmp_dn14 = assign81370_e123948_d_n14;

        let (assign81380_e123956,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1903 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign81380_e123956;

        let (assign81390_e123964,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1903 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign81390_e123964;

        let (assign81400_e123972, assign81400_e123972_d_n0, assign81400_e123972_d_n2, assign81400_e123972_d_n4, assign81400_e123972_d_n5, assign81400_e123972_d_n6, assign81400_e123972_d_n7, assign81400_e123972_d_n8, assign81400_e123972_d_n9, assign81400_e123972_d_n10, assign81400_e123972_d_n11, assign81400_e123972_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1903 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign81400_e123972;
        locals.var_arg_dn0 = assign81400_e123972_d_n0;
        locals.var_arg_dn2 = assign81400_e123972_d_n2;
        locals.var_arg_dn4 = assign81400_e123972_d_n4;
        locals.var_arg_dn5 = assign81400_e123972_d_n5;
        locals.var_arg_dn6 = assign81400_e123972_d_n6;
        locals.var_arg_dn7 = assign81400_e123972_d_n7;
        locals.var_arg_dn8 = assign81400_e123972_d_n8;
        locals.var_arg_dn9 = assign81400_e123972_d_n9;
        locals.var_arg_dn10 = assign81400_e123972_d_n10;
        locals.var_arg_dn11 = assign81400_e123972_d_n11;
        locals.var_arg_dn14 = assign81400_e123972_d_n14;

        let (assign81410_e123980, assign81410_e123980_d_n0, assign81410_e123980_d_n2, assign81410_e123980_d_n4, assign81410_e123980_d_n5, assign81410_e123980_d_n6, assign81410_e123980_d_n7, assign81410_e123980_d_n8, assign81410_e123980_d_n9, assign81410_e123980_d_n10, assign81410_e123980_d_n11, assign81410_e123980_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1903 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign81410_e123980;
        locals.var_dnm_dn0 = assign81410_e123980_d_n0;
        locals.var_dnm_dn2 = assign81410_e123980_d_n2;
        locals.var_dnm_dn4 = assign81410_e123980_d_n4;
        locals.var_dnm_dn5 = assign81410_e123980_d_n5;
        locals.var_dnm_dn6 = assign81410_e123980_d_n6;
        locals.var_dnm_dn7 = assign81410_e123980_d_n7;
        locals.var_dnm_dn8 = assign81410_e123980_d_n8;
        locals.var_dnm_dn9 = assign81410_e123980_d_n9;
        locals.var_dnm_dn10 = assign81410_e123980_d_n10;
        locals.var_dnm_dn11 = assign81410_e123980_d_n11;
        locals.var_dnm_dn14 = assign81410_e123980_d_n14;

        let (assign81420_e123990, assign81420_e123990_d_n0, assign81420_e123990_d_n2, assign81420_e123990_d_n4, assign81420_e123990_d_n5, assign81420_e123990_d_n6, assign81420_e123990_d_n7, assign81420_e123990_d_n8, assign81420_e123990_d_n9, assign81420_e123990_d_n10, assign81420_e123990_d_n11, assign81420_e123990_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1903 != 0.0)) {
        let assign81420_e123988: f64 = (locals.var_xp * locals.var_x2);
        (assign81420_e123988, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign81420_e123990;
        locals.var_xp_dn0 = assign81420_e123990_d_n0;
        locals.var_xp_dn2 = assign81420_e123990_d_n2;
        locals.var_xp_dn4 = assign81420_e123990_d_n4;
        locals.var_xp_dn5 = assign81420_e123990_d_n5;
        locals.var_xp_dn6 = assign81420_e123990_d_n6;
        locals.var_xp_dn7 = assign81420_e123990_d_n7;
        locals.var_xp_dn8 = assign81420_e123990_d_n8;
        locals.var_xp_dn9 = assign81420_e123990_d_n9;
        locals.var_xp_dn10 = assign81420_e123990_d_n10;
        locals.var_xp_dn11 = assign81420_e123990_d_n11;
        locals.var_xp_dn14 = assign81420_e123990_d_n14;

        let (assign81430_e124000, assign81430_e124000_d_n0, assign81430_e124000_d_n2, assign81430_e124000_d_n4, assign81430_e124000_d_n5, assign81430_e124000_d_n6, assign81430_e124000_d_n7, assign81430_e124000_d_n8, assign81430_e124000_d_n9, assign81430_e124000_d_n10, assign81430_e124000_d_n11, assign81430_e124000_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1903 != 0.0)) {
        let assign81430_e123998: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign81430_e123998, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign81430_e124000;
        locals.var_xmp_dn0 = assign81430_e124000_d_n0;
        locals.var_xmp_dn2 = assign81430_e124000_d_n2;
        locals.var_xmp_dn4 = assign81430_e124000_d_n4;
        locals.var_xmp_dn5 = assign81430_e124000_d_n5;
        locals.var_xmp_dn6 = assign81430_e124000_d_n6;
        locals.var_xmp_dn7 = assign81430_e124000_d_n7;
        locals.var_xmp_dn8 = assign81430_e124000_d_n8;
        locals.var_xmp_dn9 = assign81430_e124000_d_n9;
        locals.var_xmp_dn10 = assign81430_e124000_d_n10;
        locals.var_xmp_dn11 = assign81430_e124000_d_n11;
        locals.var_xmp_dn14 = assign81430_e124000_d_n14;

        let (assign81440_e124010, assign81440_e124010_d_n0, assign81440_e124010_d_n2, assign81440_e124010_d_n4, assign81440_e124010_d_n5, assign81440_e124010_d_n6, assign81440_e124010_d_n7, assign81440_e124010_d_n8, assign81440_e124010_d_n9, assign81440_e124010_d_n10, assign81440_e124010_d_n11, assign81440_e124010_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1903 != 0.0)) {
        let assign81440_e124008: f64 = (locals.var_xp + locals.var_xmp);
        (assign81440_e124008, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign81440_e124010;
        locals.var_arg_dn0 = assign81440_e124010_d_n0;
        locals.var_arg_dn2 = assign81440_e124010_d_n2;
        locals.var_arg_dn4 = assign81440_e124010_d_n4;
        locals.var_arg_dn5 = assign81440_e124010_d_n5;
        locals.var_arg_dn6 = assign81440_e124010_d_n6;
        locals.var_arg_dn7 = assign81440_e124010_d_n7;
        locals.var_arg_dn8 = assign81440_e124010_d_n8;
        locals.var_arg_dn9 = assign81440_e124010_d_n9;
        locals.var_arg_dn10 = assign81440_e124010_d_n10;
        locals.var_arg_dn11 = assign81440_e124010_d_n11;
        locals.var_arg_dn14 = assign81440_e124010_d_n14;

        let (assign81450_e124018, assign81450_e124018_d_n0, assign81450_e124018_d_n2, assign81450_e124018_d_n4, assign81450_e124018_d_n5, assign81450_e124018_d_n6, assign81450_e124018_d_n7, assign81450_e124018_d_n8, assign81450_e124018_d_n9, assign81450_e124018_d_n10, assign81450_e124018_d_n11, assign81450_e124018_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1903 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign81450_e124018;
        locals.var_dnm_dn0 = assign81450_e124018_d_n0;
        locals.var_dnm_dn2 = assign81450_e124018_d_n2;
        locals.var_dnm_dn4 = assign81450_e124018_d_n4;
        locals.var_dnm_dn5 = assign81450_e124018_d_n5;
        locals.var_dnm_dn6 = assign81450_e124018_d_n6;
        locals.var_dnm_dn7 = assign81450_e124018_d_n7;
        locals.var_dnm_dn8 = assign81450_e124018_d_n8;
        locals.var_dnm_dn9 = assign81450_e124018_d_n9;
        locals.var_dnm_dn10 = assign81450_e124018_d_n10;
        locals.var_dnm_dn11 = assign81450_e124018_d_n11;
        locals.var_dnm_dn14 = assign81450_e124018_d_n14;

        let assign81460_e124033: f64 = if ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1904 = assign81460_e124033;

        let assign81470_e124036: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1905 = assign81470_e124036;

        let (assign81480_e124048,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1903 != 0.0)) && (locals.var_guard1904 != 0.0)) && (locals.var_guard1905 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign81480_e124048;

        let assign81490_e124051: f64 = if 1.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1906 = assign81490_e124051;

        let (assign81500_e124066,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1903 != 0.0)) && (locals.var_guard1904 != 0.0)) && (locals.var_guard1905 == 0.0)) && (locals.var_guard1906 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign81500_e124066;

        let assign81510_e124069: f64 = if 1.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1907 = assign81510_e124069;

        let (assign81520_e124087,) = {
    if (((((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1903 != 0.0)) && (locals.var_guard1904 != 0.0)) && (locals.var_guard1905 == 0.0)) && (locals.var_guard1906 == 0.0)) && (locals.var_guard1907 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign81520_e124087;

        let assign81530_e124090: f64 = if 1.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1908 = assign81530_e124090;

        let (assign81540_e124111,) = {
    if ((((((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1903 != 0.0)) && (locals.var_guard1904 != 0.0)) && (locals.var_guard1905 == 0.0)) && (locals.var_guard1906 == 0.0)) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1908 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign81540_e124111;

        let (assign81550_e124121,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1903 != 0.0)) && (locals.var_guard1904 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign81550_e124121;

        let mut assign81560_loop_guard: usize = 0;
        while {
            let assign81560_cond_e124132: f64 = if (((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1903 != 0.0)) && (locals.var_guard1904 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign81560_cond_e124132 != 0.0
        } {
            assign81560_loop_guard += 1;
            assert!(assign81560_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign81560_body0_e124143, assign81560_body0_e124143_d_n0, assign81560_body0_e124143_d_n2, assign81560_body0_e124143_d_n4, assign81560_body0_e124143_d_n5, assign81560_body0_e124143_d_n6, assign81560_body0_e124143_d_n7, assign81560_body0_e124143_d_n8, assign81560_body0_e124143_d_n9, assign81560_body0_e124143_d_n10, assign81560_body0_e124143_d_n11, assign81560_body0_e124143_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1903 != 0.0)) && (locals.var_guard1904 != 0.0)) {
        let assign81560_body0_e124141: f64 = (locals.var_dnm).sqrt();
        (assign81560_body0_e124141, (locals.var_dnm_dn0 / (2.0 * assign81560_body0_e124141)), (locals.var_dnm_dn2 / (2.0 * assign81560_body0_e124141)), (locals.var_dnm_dn4 / (2.0 * assign81560_body0_e124141)), (locals.var_dnm_dn5 / (2.0 * assign81560_body0_e124141)), (locals.var_dnm_dn6 / (2.0 * assign81560_body0_e124141)), (locals.var_dnm_dn7 / (2.0 * assign81560_body0_e124141)), (locals.var_dnm_dn8 / (2.0 * assign81560_body0_e124141)), (locals.var_dnm_dn9 / (2.0 * assign81560_body0_e124141)), (locals.var_dnm_dn10 / (2.0 * assign81560_body0_e124141)), (locals.var_dnm_dn11 / (2.0 * assign81560_body0_e124141)), (locals.var_dnm_dn14 / (2.0 * assign81560_body0_e124141)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign81560_body0_e124143;
            locals.var_dnm_dn0 = assign81560_body0_e124143_d_n0;
            locals.var_dnm_dn2 = assign81560_body0_e124143_d_n2;
            locals.var_dnm_dn4 = assign81560_body0_e124143_d_n4;
            locals.var_dnm_dn5 = assign81560_body0_e124143_d_n5;
            locals.var_dnm_dn6 = assign81560_body0_e124143_d_n6;
            locals.var_dnm_dn7 = assign81560_body0_e124143_d_n7;
            locals.var_dnm_dn8 = assign81560_body0_e124143_d_n8;
            locals.var_dnm_dn9 = assign81560_body0_e124143_d_n9;
            locals.var_dnm_dn10 = assign81560_body0_e124143_d_n10;
            locals.var_dnm_dn11 = assign81560_body0_e124143_d_n11;
            locals.var_dnm_dn14 = assign81560_body0_e124143_d_n14;
            let (assign81560_body1_e124155,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1903 != 0.0)) && (locals.var_guard1904 != 0.0)) {
        let assign81560_body1_e124153: f64 = (locals.var_m0 + 1.0);
        (assign81560_body1_e124153,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign81560_body1_e124155;
        }

    }

    pub(super) fn stamp_transient_block_295(
        locals: &mut StampLocals,
    ) {
        let (assign81570_e124177, assign81570_e124177_d_n0, assign81570_e124177_d_n2, assign81570_e124177_d_n4, assign81570_e124177_d_n5, assign81570_e124177_d_n6, assign81570_e124177_d_n7, assign81570_e124177_d_n8, assign81570_e124177_d_n9, assign81570_e124177_d_n10, assign81570_e124177_d_n11, assign81570_e124177_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1903 != 0.0)) && (locals.var_guard1904 == 0.0)) {
        let (assign81570_e124175, assign81570_e124175_d_n0, assign81570_e124175_d_n2, assign81570_e124175_d_n4, assign81570_e124175_d_n5, assign81570_e124175_d_n6, assign81570_e124175_d_n7, assign81570_e124175_d_n8, assign81570_e124175_d_n9, assign81570_e124175_d_n10, assign81570_e124175_d_n11, assign81570_e124175_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign81570_e124172: f64 = 2.0;
                let assign81570_e124173: f64 = (1.0 / assign81570_e124172);
                let assign81570_e124174: f64 = (locals.var_dnm).powf(assign81570_e124173);
                (assign81570_e124174, if 0.0 == 0.0 && ((assign81570_e124173) as f64).is_finite() && ((assign81570_e124173) as f64).fract() == 0.0 { if assign81570_e124173 == 0.0 { 0.0 } else { (assign81570_e124173 * ((locals.var_dnm).powf(assign81570_e124173 - 1.0) * locals.var_dnm_dn0)) } } else { (assign81570_e124174 * (assign81570_e124173 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign81570_e124173) as f64).is_finite() && ((assign81570_e124173) as f64).fract() == 0.0 { if assign81570_e124173 == 0.0 { 0.0 } else { (assign81570_e124173 * ((locals.var_dnm).powf(assign81570_e124173 - 1.0) * locals.var_dnm_dn2)) } } else { (assign81570_e124174 * (assign81570_e124173 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign81570_e124173) as f64).is_finite() && ((assign81570_e124173) as f64).fract() == 0.0 { if assign81570_e124173 == 0.0 { 0.0 } else { (assign81570_e124173 * ((locals.var_dnm).powf(assign81570_e124173 - 1.0) * locals.var_dnm_dn4)) } } else { (assign81570_e124174 * (assign81570_e124173 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign81570_e124173) as f64).is_finite() && ((assign81570_e124173) as f64).fract() == 0.0 { if assign81570_e124173 == 0.0 { 0.0 } else { (assign81570_e124173 * ((locals.var_dnm).powf(assign81570_e124173 - 1.0) * locals.var_dnm_dn5)) } } else { (assign81570_e124174 * (assign81570_e124173 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign81570_e124173) as f64).is_finite() && ((assign81570_e124173) as f64).fract() == 0.0 { if assign81570_e124173 == 0.0 { 0.0 } else { (assign81570_e124173 * ((locals.var_dnm).powf(assign81570_e124173 - 1.0) * locals.var_dnm_dn6)) } } else { (assign81570_e124174 * (assign81570_e124173 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign81570_e124173) as f64).is_finite() && ((assign81570_e124173) as f64).fract() == 0.0 { if assign81570_e124173 == 0.0 { 0.0 } else { (assign81570_e124173 * ((locals.var_dnm).powf(assign81570_e124173 - 1.0) * locals.var_dnm_dn7)) } } else { (assign81570_e124174 * (assign81570_e124173 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign81570_e124173) as f64).is_finite() && ((assign81570_e124173) as f64).fract() == 0.0 { if assign81570_e124173 == 0.0 { 0.0 } else { (assign81570_e124173 * ((locals.var_dnm).powf(assign81570_e124173 - 1.0) * locals.var_dnm_dn8)) } } else { (assign81570_e124174 * (assign81570_e124173 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign81570_e124173) as f64).is_finite() && ((assign81570_e124173) as f64).fract() == 0.0 { if assign81570_e124173 == 0.0 { 0.0 } else { (assign81570_e124173 * ((locals.var_dnm).powf(assign81570_e124173 - 1.0) * locals.var_dnm_dn9)) } } else { (assign81570_e124174 * (assign81570_e124173 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign81570_e124173) as f64).is_finite() && ((assign81570_e124173) as f64).fract() == 0.0 { if assign81570_e124173 == 0.0 { 0.0 } else { (assign81570_e124173 * ((locals.var_dnm).powf(assign81570_e124173 - 1.0) * locals.var_dnm_dn10)) } } else { (assign81570_e124174 * (assign81570_e124173 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign81570_e124173) as f64).is_finite() && ((assign81570_e124173) as f64).fract() == 0.0 { if assign81570_e124173 == 0.0 { 0.0 } else { (assign81570_e124173 * ((locals.var_dnm).powf(assign81570_e124173 - 1.0) * locals.var_dnm_dn11)) } } else { (assign81570_e124174 * (assign81570_e124173 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign81570_e124173) as f64).is_finite() && ((assign81570_e124173) as f64).fract() == 0.0 { if assign81570_e124173 == 0.0 { 0.0 } else { (assign81570_e124173 * ((locals.var_dnm).powf(assign81570_e124173 - 1.0) * locals.var_dnm_dn14)) } } else { (assign81570_e124174 * (assign81570_e124173 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign81570_e124175, assign81570_e124175_d_n0, assign81570_e124175_d_n2, assign81570_e124175_d_n4, assign81570_e124175_d_n5, assign81570_e124175_d_n6, assign81570_e124175_d_n7, assign81570_e124175_d_n8, assign81570_e124175_d_n9, assign81570_e124175_d_n10, assign81570_e124175_d_n11, assign81570_e124175_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign81570_e124177;
        locals.var_dnm_dn0 = assign81570_e124177_d_n0;
        locals.var_dnm_dn2 = assign81570_e124177_d_n2;
        locals.var_dnm_dn4 = assign81570_e124177_d_n4;
        locals.var_dnm_dn5 = assign81570_e124177_d_n5;
        locals.var_dnm_dn6 = assign81570_e124177_d_n6;
        locals.var_dnm_dn7 = assign81570_e124177_d_n7;
        locals.var_dnm_dn8 = assign81570_e124177_d_n8;
        locals.var_dnm_dn9 = assign81570_e124177_d_n9;
        locals.var_dnm_dn10 = assign81570_e124177_d_n10;
        locals.var_dnm_dn11 = assign81570_e124177_d_n11;
        locals.var_dnm_dn14 = assign81570_e124177_d_n14;

        let (assign81580_e124187, assign81580_e124187_d_n0, assign81580_e124187_d_n2, assign81580_e124187_d_n4, assign81580_e124187_d_n5, assign81580_e124187_d_n6, assign81580_e124187_d_n7, assign81580_e124187_d_n8, assign81580_e124187_d_n9, assign81580_e124187_d_n10, assign81580_e124187_d_n11, assign81580_e124187_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1903 != 0.0)) {
        let assign81580_e124185: f64 = (1.0 / locals.var_dnm);
        (assign81580_e124185, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign81580_e124187;
        locals.var_dnm_dn0 = assign81580_e124187_d_n0;
        locals.var_dnm_dn2 = assign81580_e124187_d_n2;
        locals.var_dnm_dn4 = assign81580_e124187_d_n4;
        locals.var_dnm_dn5 = assign81580_e124187_d_n5;
        locals.var_dnm_dn6 = assign81580_e124187_d_n6;
        locals.var_dnm_dn7 = assign81580_e124187_d_n7;
        locals.var_dnm_dn8 = assign81580_e124187_d_n8;
        locals.var_dnm_dn9 = assign81580_e124187_d_n9;
        locals.var_dnm_dn10 = assign81580_e124187_d_n10;
        locals.var_dnm_dn11 = assign81580_e124187_d_n11;
        locals.var_dnm_dn14 = assign81580_e124187_d_n14;

        let (assign81590_e124199, assign81590_e124199_d_n0, assign81590_e124199_d_n2, assign81590_e124199_d_n4, assign81590_e124199_d_n5, assign81590_e124199_d_n6, assign81590_e124199_d_n7, assign81590_e124199_d_n8, assign81590_e124199_d_n9, assign81590_e124199_d_n10, assign81590_e124199_d_n11, assign81590_e124199_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1903 != 0.0)) {
        let assign81590_e124195: f64 = (locals.var_tmf1 * locals.var_t1);
        let assign81590_e124197: f64 = (assign81590_e124195 * locals.var_dnm);
        (assign81590_e124197, ((((locals.var_tmf1_dn0 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn0)) * locals.var_dnm) + (assign81590_e124195 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn2)) * locals.var_dnm) + (assign81590_e124195 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn4 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn4)) * locals.var_dnm) + (assign81590_e124195 * locals.var_dnm_dn4)), ((((locals.var_tmf1_dn5 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn5)) * locals.var_dnm) + (assign81590_e124195 * locals.var_dnm_dn5)), ((((locals.var_tmf1_dn6 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn6)) * locals.var_dnm) + (assign81590_e124195 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn7)) * locals.var_dnm) + (assign81590_e124195 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn8 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn8)) * locals.var_dnm) + (assign81590_e124195 * locals.var_dnm_dn8)), ((((locals.var_tmf1_dn9 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn9)) * locals.var_dnm) + (assign81590_e124195 * locals.var_dnm_dn9)), ((((locals.var_tmf1_dn10 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn10)) * locals.var_dnm) + (assign81590_e124195 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn11 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn11)) * locals.var_dnm) + (assign81590_e124195 * locals.var_dnm_dn11)), ((((locals.var_tmf1_dn14 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn14)) * locals.var_dnm) + (assign81590_e124195 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign81590_e124199;
        locals.var_tmf0_dn0 = assign81590_e124199_d_n0;
        locals.var_tmf0_dn2 = assign81590_e124199_d_n2;
        locals.var_tmf0_dn4 = assign81590_e124199_d_n4;
        locals.var_tmf0_dn5 = assign81590_e124199_d_n5;
        locals.var_tmf0_dn6 = assign81590_e124199_d_n6;
        locals.var_tmf0_dn7 = assign81590_e124199_d_n7;
        locals.var_tmf0_dn8 = assign81590_e124199_d_n8;
        locals.var_tmf0_dn9 = assign81590_e124199_d_n9;
        locals.var_tmf0_dn10 = assign81590_e124199_d_n10;
        locals.var_tmf0_dn11 = assign81590_e124199_d_n11;
        locals.var_tmf0_dn14 = assign81590_e124199_d_n14;

        let (assign81600_e124213, assign81600_e124213_d_n0, assign81600_e124213_d_n2, assign81600_e124213_d_n4, assign81600_e124213_d_n5, assign81600_e124213_d_n6, assign81600_e124213_d_n7, assign81600_e124213_d_n8, assign81600_e124213_d_n9, assign81600_e124213_d_n10, assign81600_e124213_d_n11, assign81600_e124213_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1903 != 0.0)) {
        let assign81600_e124207: f64 = (locals.var_t1 * locals.var_xmp);
        let assign81600_e124209: f64 = (assign81600_e124207 * locals.var_dnm);
        let assign81600_e124211: f64 = (assign81600_e124209 / locals.var_arg);
        (assign81600_e124211, (((((((locals.var_t1_dn0 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn0)) * locals.var_dnm) + (assign81600_e124207 * locals.var_dnm_dn0)) * locals.var_arg) - (assign81600_e124209 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn2 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn2)) * locals.var_dnm) + (assign81600_e124207 * locals.var_dnm_dn2)) * locals.var_arg) - (assign81600_e124209 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn4 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn4)) * locals.var_dnm) + (assign81600_e124207 * locals.var_dnm_dn4)) * locals.var_arg) - (assign81600_e124209 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn5 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn5)) * locals.var_dnm) + (assign81600_e124207 * locals.var_dnm_dn5)) * locals.var_arg) - (assign81600_e124209 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn6 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn6)) * locals.var_dnm) + (assign81600_e124207 * locals.var_dnm_dn6)) * locals.var_arg) - (assign81600_e124209 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn7 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn7)) * locals.var_dnm) + (assign81600_e124207 * locals.var_dnm_dn7)) * locals.var_arg) - (assign81600_e124209 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn8 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn8)) * locals.var_dnm) + (assign81600_e124207 * locals.var_dnm_dn8)) * locals.var_arg) - (assign81600_e124209 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn9 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn9)) * locals.var_dnm) + (assign81600_e124207 * locals.var_dnm_dn9)) * locals.var_arg) - (assign81600_e124209 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn10 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn10)) * locals.var_dnm) + (assign81600_e124207 * locals.var_dnm_dn10)) * locals.var_arg) - (assign81600_e124209 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn11 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn11)) * locals.var_dnm) + (assign81600_e124207 * locals.var_dnm_dn11)) * locals.var_arg) - (assign81600_e124209 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn14 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn14)) * locals.var_dnm) + (assign81600_e124207 * locals.var_dnm_dn14)) * locals.var_arg) - (assign81600_e124209 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign81600_e124213;
        locals.var_t0_dn0 = assign81600_e124213_d_n0;
        locals.var_t0_dn2 = assign81600_e124213_d_n2;
        locals.var_t0_dn4 = assign81600_e124213_d_n4;
        locals.var_t0_dn5 = assign81600_e124213_d_n5;
        locals.var_t0_dn6 = assign81600_e124213_d_n6;
        locals.var_t0_dn7 = assign81600_e124213_d_n7;
        locals.var_t0_dn8 = assign81600_e124213_d_n8;
        locals.var_t0_dn9 = assign81600_e124213_d_n9;
        locals.var_t0_dn10 = assign81600_e124213_d_n10;
        locals.var_t0_dn11 = assign81600_e124213_d_n11;
        locals.var_t0_dn14 = assign81600_e124213_d_n14;

        let (assign81610_e124225, assign81610_e124225_d_n0, assign81610_e124225_d_n2, assign81610_e124225_d_n4, assign81610_e124225_d_n5, assign81610_e124225_d_n6, assign81610_e124225_d_n7, assign81610_e124225_d_n8, assign81610_e124225_d_n9, assign81610_e124225_d_n10, assign81610_e124225_d_n11, assign81610_e124225_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1903 != 0.0)) {
        let assign81610_e124221: f64 = (-locals.var_t1);
        let assign81610_e124223: f64 = (assign81610_e124221 + locals.var_tmf0);
        (assign81610_e124223, ((-locals.var_t1_dn0) + locals.var_tmf0_dn0), ((-locals.var_t1_dn2) + locals.var_tmf0_dn2), ((-locals.var_t1_dn4) + locals.var_tmf0_dn4), ((-locals.var_t1_dn5) + locals.var_tmf0_dn5), ((-locals.var_t1_dn6) + locals.var_tmf0_dn6), ((-locals.var_t1_dn7) + locals.var_tmf0_dn7), ((-locals.var_t1_dn8) + locals.var_tmf0_dn8), ((-locals.var_t1_dn9) + locals.var_tmf0_dn9), ((-locals.var_t1_dn10) + locals.var_tmf0_dn10), ((-locals.var_t1_dn11) + locals.var_tmf0_dn11), ((-locals.var_t1_dn14) + locals.var_tmf0_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign81610_e124225;
        locals.var_t1_dn0 = assign81610_e124225_d_n0;
        locals.var_t1_dn2 = assign81610_e124225_d_n2;
        locals.var_t1_dn4 = assign81610_e124225_d_n4;
        locals.var_t1_dn5 = assign81610_e124225_d_n5;
        locals.var_t1_dn6 = assign81610_e124225_d_n6;
        locals.var_t1_dn7 = assign81610_e124225_d_n7;
        locals.var_t1_dn8 = assign81610_e124225_d_n8;
        locals.var_t1_dn9 = assign81610_e124225_d_n9;
        locals.var_t1_dn10 = assign81610_e124225_d_n10;
        locals.var_t1_dn11 = assign81610_e124225_d_n11;
        locals.var_t1_dn14 = assign81610_e124225_d_n14;

        let (assign81620_e124233, assign81620_e124233_d_n0, assign81620_e124233_d_n2, assign81620_e124233_d_n4, assign81620_e124233_d_n5, assign81620_e124233_d_n6, assign81620_e124233_d_n7, assign81620_e124233_d_n8, assign81620_e124233_d_n9, assign81620_e124233_d_n10, assign81620_e124233_d_n11, assign81620_e124233_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1903 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign81620_e124233;
        locals.var_t0_dn0 = assign81620_e124233_d_n0;
        locals.var_t0_dn2 = assign81620_e124233_d_n2;
        locals.var_t0_dn4 = assign81620_e124233_d_n4;
        locals.var_t0_dn5 = assign81620_e124233_d_n5;
        locals.var_t0_dn6 = assign81620_e124233_d_n6;
        locals.var_t0_dn7 = assign81620_e124233_d_n7;
        locals.var_t0_dn8 = assign81620_e124233_d_n8;
        locals.var_t0_dn9 = assign81620_e124233_d_n9;
        locals.var_t0_dn10 = assign81620_e124233_d_n10;
        locals.var_t0_dn11 = assign81620_e124233_d_n11;
        locals.var_t0_dn14 = assign81620_e124233_d_n14;

        let (assign81630_e124244, assign81630_e124244_d_n0, assign81630_e124244_d_n2, assign81630_e124244_d_n4, assign81630_e124244_d_n5, assign81630_e124244_d_n6, assign81630_e124244_d_n7, assign81630_e124244_d_n8, assign81630_e124244_d_n9, assign81630_e124244_d_n10, assign81630_e124244_d_n11, assign81630_e124244_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1903 == 0.0)) {
        let assign81630_e124242: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        (assign81630_e124242, locals.var_vxbgmtcl_dn0, (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2), locals.var_vxbgmtcl_dn4, locals.var_vxbgmtcl_dn5, locals.var_vxbgmtcl_dn6, (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7), (locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8), (locals.var_vgpld_dn9 + locals.var_vxbgmtcl_dn9), locals.var_vxbgmtcl_dn10, locals.var_vxbgmtcl_dn11, locals.var_vxbgmtcl_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign81630_e124244;
        locals.var_t1_dn0 = assign81630_e124244_d_n0;
        locals.var_t1_dn2 = assign81630_e124244_d_n2;
        locals.var_t1_dn4 = assign81630_e124244_d_n4;
        locals.var_t1_dn5 = assign81630_e124244_d_n5;
        locals.var_t1_dn6 = assign81630_e124244_d_n6;
        locals.var_t1_dn7 = assign81630_e124244_d_n7;
        locals.var_t1_dn8 = assign81630_e124244_d_n8;
        locals.var_t1_dn9 = assign81630_e124244_d_n9;
        locals.var_t1_dn10 = assign81630_e124244_d_n10;
        locals.var_t1_dn11 = assign81630_e124244_d_n11;
        locals.var_t1_dn14 = assign81630_e124244_d_n14;

        let (assign81640_e124253, assign81640_e124253_d_n0, assign81640_e124253_d_n2, assign81640_e124253_d_n4, assign81640_e124253_d_n5, assign81640_e124253_d_n6, assign81640_e124253_d_n7, assign81640_e124253_d_n8, assign81640_e124253_d_n9, assign81640_e124253_d_n10, assign81640_e124253_d_n11, assign81640_e124253_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1903 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign81640_e124253;
        locals.var_t0_dn0 = assign81640_e124253_d_n0;
        locals.var_t0_dn2 = assign81640_e124253_d_n2;
        locals.var_t0_dn4 = assign81640_e124253_d_n4;
        locals.var_t0_dn5 = assign81640_e124253_d_n5;
        locals.var_t0_dn6 = assign81640_e124253_d_n6;
        locals.var_t0_dn7 = assign81640_e124253_d_n7;
        locals.var_t0_dn8 = assign81640_e124253_d_n8;
        locals.var_t0_dn9 = assign81640_e124253_d_n9;
        locals.var_t0_dn10 = assign81640_e124253_d_n10;
        locals.var_t0_dn11 = assign81640_e124253_d_n11;
        locals.var_t0_dn14 = assign81640_e124253_d_n14;

        let (assign81650_e124261, assign81650_e124261_d_n0, assign81650_e124261_d_n2, assign81650_e124261_d_n4, assign81650_e124261_d_n5, assign81650_e124261_d_n6, assign81650_e124261_d_n7, assign81650_e124261_d_n8, assign81650_e124261_d_n9, assign81650_e124261_d_n10, assign81650_e124261_d_n11, assign81650_e124261_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign81650_e124259: f64 = (locals.var_t1 - locals.var_vgpld);
        (assign81650_e124259, locals.var_t1_dn0, (locals.var_t1_dn2 - locals.var_vgpld_dn2), locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, (locals.var_t1_dn7 - locals.var_vgpld_dn7), (locals.var_t1_dn8 - locals.var_vgpld_dn8), (locals.var_t1_dn9 - locals.var_vgpld_dn9), locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    } else {
        (locals.var_vxbgmtcl, locals.var_vxbgmtcl_dn0, locals.var_vxbgmtcl_dn2, locals.var_vxbgmtcl_dn4, locals.var_vxbgmtcl_dn5, locals.var_vxbgmtcl_dn6, locals.var_vxbgmtcl_dn7, locals.var_vxbgmtcl_dn8, locals.var_vxbgmtcl_dn9, locals.var_vxbgmtcl_dn10, locals.var_vxbgmtcl_dn11, locals.var_vxbgmtcl_dn14,)
    }
};
        locals.var_vxbgmtcl = assign81650_e124261;
        locals.var_vxbgmtcl_dn0 = assign81650_e124261_d_n0;
        locals.var_vxbgmtcl_dn2 = assign81650_e124261_d_n2;
        locals.var_vxbgmtcl_dn4 = assign81650_e124261_d_n4;
        locals.var_vxbgmtcl_dn5 = assign81650_e124261_d_n5;
        locals.var_vxbgmtcl_dn6 = assign81650_e124261_d_n6;
        locals.var_vxbgmtcl_dn7 = assign81650_e124261_d_n7;
        locals.var_vxbgmtcl_dn8 = assign81650_e124261_d_n8;
        locals.var_vxbgmtcl_dn9 = assign81650_e124261_d_n9;
        locals.var_vxbgmtcl_dn10 = assign81650_e124261_d_n10;
        locals.var_vxbgmtcl_dn11 = assign81650_e124261_d_n11;
        locals.var_vxbgmtcl_dn14 = assign81650_e124261_d_n14;

        let (assign81660_e124272,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign81660_e124266: f64 = (-locals.var_vxbgmtcl);
        let assign81660_e124269: f64 = (10.0 * 2.220446049250313e-16);
        let assign81660_e124270: f64 = (assign81660_e124266 + assign81660_e124269);
        (assign81660_e124270,)
    } else {
        (locals.var_vgb_fb_ld,)
    }
};
        locals.var_vgb_fb_ld = assign81660_e124272;

        let assign81670_e124275: f64 = if locals.var_vgpld < locals.var_vgb_fb_ld { 1.0 } else { 0.0 };
        locals.var_guard1909 = assign81670_e124275;

        let (assign81690_e124296, assign81690_e124296_d_n0, assign81690_e124296_d_n2, assign81690_e124296_d_n4, assign81690_e124296_d_n5, assign81690_e124296_d_n6, assign81690_e124296_d_n7, assign81690_e124296_d_n8, assign81690_e124296_d_n9, assign81690_e124296_d_n10, assign81690_e124296_d_n11, assign81690_e124296_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 != 0.0)) {
        let assign81690_e124288: f64 = (2.0 * locals.var_beta_inv);
        let assign81690_e124290: f64 = (-locals.var_vgs_min);
        let assign81690_e124292: f64 = (assign81690_e124290 / locals.var_fac1);
        let assign81690_e124293: f64 = (assign81690_e124292).ln();
        let assign81690_e124294: f64 = (assign81690_e124288 * assign81690_e124293);
        (assign81690_e124294, (((2.0 * locals.var_beta_inv_dn0) * assign81690_e124293) + (assign81690_e124288 * ((-((assign81690_e124290 * locals.var_fac1_dn0) / (locals.var_fac1 * locals.var_fac1))) / assign81690_e124292))), (((2.0 * locals.var_beta_inv_dn2) * assign81690_e124293) + (assign81690_e124288 * ((-((assign81690_e124290 * locals.var_fac1_dn2) / (locals.var_fac1 * locals.var_fac1))) / assign81690_e124292))), (((2.0 * locals.var_beta_inv_dn4) * assign81690_e124293) + (assign81690_e124288 * ((-((assign81690_e124290 * locals.var_fac1_dn4) / (locals.var_fac1 * locals.var_fac1))) / assign81690_e124292))), (((2.0 * locals.var_beta_inv_dn5) * assign81690_e124293) + (assign81690_e124288 * ((-((assign81690_e124290 * locals.var_fac1_dn5) / (locals.var_fac1 * locals.var_fac1))) / assign81690_e124292))), (((2.0 * locals.var_beta_inv_dn6) * assign81690_e124293) + (assign81690_e124288 * ((-((assign81690_e124290 * locals.var_fac1_dn6) / (locals.var_fac1 * locals.var_fac1))) / assign81690_e124292))), (((2.0 * locals.var_beta_inv_dn7) * assign81690_e124293) + (assign81690_e124288 * ((-((assign81690_e124290 * locals.var_fac1_dn7) / (locals.var_fac1 * locals.var_fac1))) / assign81690_e124292))), (((2.0 * locals.var_beta_inv_dn8) * assign81690_e124293) + (assign81690_e124288 * ((-((assign81690_e124290 * locals.var_fac1_dn8) / (locals.var_fac1 * locals.var_fac1))) / assign81690_e124292))), (((2.0 * locals.var_beta_inv_dn9) * assign81690_e124293) + (assign81690_e124288 * ((-((assign81690_e124290 * locals.var_fac1_dn9) / (locals.var_fac1 * locals.var_fac1))) / assign81690_e124292))), (((2.0 * locals.var_beta_inv_dn10) * assign81690_e124293) + (assign81690_e124288 * ((-((assign81690_e124290 * locals.var_fac1_dn10) / (locals.var_fac1 * locals.var_fac1))) / assign81690_e124292))), (((2.0 * locals.var_beta_inv_dn11) * assign81690_e124293) + (assign81690_e124288 * ((-((assign81690_e124290 * locals.var_fac1_dn11) / (locals.var_fac1 * locals.var_fac1))) / assign81690_e124292))), (((2.0 * locals.var_beta_inv_dn14) * assign81690_e124293) + (assign81690_e124288 * ((-((assign81690_e124290 * locals.var_fac1_dn14) / (locals.var_fac1 * locals.var_fac1))) / assign81690_e124292))),)
    } else {
        (locals.var_ps0_min, locals.var_ps0_min_dn0, locals.var_ps0_min_dn2, locals.var_ps0_min_dn4, locals.var_ps0_min_dn5, locals.var_ps0_min_dn6, locals.var_ps0_min_dn7, locals.var_ps0_min_dn8, locals.var_ps0_min_dn9, locals.var_ps0_min_dn10, locals.var_ps0_min_dn11, locals.var_ps0_min_dn14,)
    }
};
        locals.var_ps0_min = assign81690_e124296;
        locals.var_ps0_min_dn0 = assign81690_e124296_d_n0;
        locals.var_ps0_min_dn2 = assign81690_e124296_d_n2;
        locals.var_ps0_min_dn4 = assign81690_e124296_d_n4;
        locals.var_ps0_min_dn5 = assign81690_e124296_d_n5;
        locals.var_ps0_min_dn6 = assign81690_e124296_d_n6;
        locals.var_ps0_min_dn7 = assign81690_e124296_d_n7;
        locals.var_ps0_min_dn8 = assign81690_e124296_d_n8;
        locals.var_ps0_min_dn9 = assign81690_e124296_d_n9;
        locals.var_ps0_min_dn10 = assign81690_e124296_d_n10;
        locals.var_ps0_min_dn11 = assign81690_e124296_d_n11;
        locals.var_ps0_min_dn14 = assign81690_e124296_d_n14;

        let (assign81700_e124306, assign81700_e124306_d_n0, assign81700_e124306_d_n2, assign81700_e124306_d_n4, assign81700_e124306_d_n5, assign81700_e124306_d_n6, assign81700_e124306_d_n7, assign81700_e124306_d_n8, assign81700_e124306_d_n9, assign81700_e124306_d_n10, assign81700_e124306_d_n11, assign81700_e124306_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 != 0.0)) {
        let assign81700_e124303: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign81700_e124304: f64 = (locals.var_beta * assign81700_e124303);
        (assign81700_e124304, ((locals.var_beta_dn0 * assign81700_e124303) + (locals.var_beta * locals.var_vxbgmtcl_dn0)), ((locals.var_beta_dn2 * assign81700_e124303) + (locals.var_beta * (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2))), ((locals.var_beta_dn4 * assign81700_e124303) + (locals.var_beta * locals.var_vxbgmtcl_dn4)), ((locals.var_beta_dn5 * assign81700_e124303) + (locals.var_beta * locals.var_vxbgmtcl_dn5)), ((locals.var_beta_dn6 * assign81700_e124303) + (locals.var_beta * locals.var_vxbgmtcl_dn6)), ((locals.var_beta_dn7 * assign81700_e124303) + (locals.var_beta * (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7))), ((locals.var_beta_dn8 * assign81700_e124303) + (locals.var_beta * (locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8))), ((locals.var_beta_dn9 * assign81700_e124303) + (locals.var_beta * (locals.var_vgpld_dn9 + locals.var_vxbgmtcl_dn9))), ((locals.var_beta_dn10 * assign81700_e124303) + (locals.var_beta * locals.var_vxbgmtcl_dn10)), ((locals.var_beta_dn11 * assign81700_e124303) + (locals.var_beta * locals.var_vxbgmtcl_dn11)), ((locals.var_beta_dn14 * assign81700_e124303) + (locals.var_beta * locals.var_vxbgmtcl_dn14)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign81700_e124306;
        locals.var_tx_dn0 = assign81700_e124306_d_n0;
        locals.var_tx_dn2 = assign81700_e124306_d_n2;
        locals.var_tx_dn4 = assign81700_e124306_d_n4;
        locals.var_tx_dn5 = assign81700_e124306_d_n5;
        locals.var_tx_dn6 = assign81700_e124306_d_n6;
        locals.var_tx_dn7 = assign81700_e124306_d_n7;
        locals.var_tx_dn8 = assign81700_e124306_d_n8;
        locals.var_tx_dn9 = assign81700_e124306_d_n9;
        locals.var_tx_dn10 = assign81700_e124306_d_n10;
        locals.var_tx_dn11 = assign81700_e124306_d_n11;
        locals.var_tx_dn14 = assign81700_e124306_d_n14;

        let (assign81710_e124316, assign81710_e124316_d_n0, assign81710_e124316_d_n2, assign81710_e124316_d_n4, assign81710_e124316_d_n5, assign81710_e124316_d_n6, assign81710_e124316_d_n7, assign81710_e124316_d_n8, assign81710_e124316_d_n9, assign81710_e124316_d_n10, assign81710_e124316_d_n11, assign81710_e124316_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 != 0.0)) {
        let assign81710_e124313: f64 = (locals.var_beta * locals.var_cnst0over_func);
        let assign81710_e124314: f64 = (1.0 / assign81710_e124313);
        (assign81710_e124314, (-(((locals.var_beta_dn0 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn0)) / (assign81710_e124313 * assign81710_e124313))), (-(((locals.var_beta_dn2 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn2)) / (assign81710_e124313 * assign81710_e124313))), (-(((locals.var_beta_dn4 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn4)) / (assign81710_e124313 * assign81710_e124313))), (-(((locals.var_beta_dn5 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn5)) / (assign81710_e124313 * assign81710_e124313))), (-(((locals.var_beta_dn6 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn6)) / (assign81710_e124313 * assign81710_e124313))), (-(((locals.var_beta_dn7 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn7)) / (assign81710_e124313 * assign81710_e124313))), (-(((locals.var_beta_dn8 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn8)) / (assign81710_e124313 * assign81710_e124313))), (-(((locals.var_beta_dn9 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn9)) / (assign81710_e124313 * assign81710_e124313))), (-(((locals.var_beta_dn10 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn10)) / (assign81710_e124313 * assign81710_e124313))), (-(((locals.var_beta_dn11 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn11)) / (assign81710_e124313 * assign81710_e124313))), (-(((locals.var_beta_dn14 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn14)) / (assign81710_e124313 * assign81710_e124313))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign81710_e124316;
        locals.var_t1_dn0 = assign81710_e124316_d_n0;
        locals.var_t1_dn2 = assign81710_e124316_d_n2;
        locals.var_t1_dn4 = assign81710_e124316_d_n4;
        locals.var_t1_dn5 = assign81710_e124316_d_n5;
        locals.var_t1_dn6 = assign81710_e124316_d_n6;
        locals.var_t1_dn7 = assign81710_e124316_d_n7;
        locals.var_t1_dn8 = assign81710_e124316_d_n8;
        locals.var_t1_dn9 = assign81710_e124316_d_n9;
        locals.var_t1_dn10 = assign81710_e124316_d_n10;
        locals.var_t1_dn11 = assign81710_e124316_d_n11;
        locals.var_t1_dn14 = assign81710_e124316_d_n14;

        let (assign81720_e124324, assign81720_e124324_d_n0, assign81720_e124324_d_n2, assign81720_e124324_d_n4, assign81720_e124324_d_n5, assign81720_e124324_d_n6, assign81720_e124324_d_n7, assign81720_e124324_d_n8, assign81720_e124324_d_n9, assign81720_e124324_d_n10, assign81720_e124324_d_n11, assign81720_e124324_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 != 0.0)) {
        let assign81720_e124322: f64 = (locals.var_t1 * locals.var_cox0_func);
        (assign81720_e124322, (locals.var_t1_dn0 * locals.var_cox0_func), (locals.var_t1_dn2 * locals.var_cox0_func), (locals.var_t1_dn4 * locals.var_cox0_func), (locals.var_t1_dn5 * locals.var_cox0_func), (locals.var_t1_dn6 * locals.var_cox0_func), (locals.var_t1_dn7 * locals.var_cox0_func), (locals.var_t1_dn8 * locals.var_cox0_func), (locals.var_t1_dn9 * locals.var_cox0_func), (locals.var_t1_dn10 * locals.var_cox0_func), (locals.var_t1_dn11 * locals.var_cox0_func), (locals.var_t1_dn14 * locals.var_cox0_func),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn14,)
    }
};
        locals.var_ty = assign81720_e124324;
        locals.var_ty_dn0 = assign81720_e124324_d_n0;
        locals.var_ty_dn2 = assign81720_e124324_d_n2;
        locals.var_ty_dn4 = assign81720_e124324_d_n4;
        locals.var_ty_dn5 = assign81720_e124324_d_n5;
        locals.var_ty_dn6 = assign81720_e124324_d_n6;
        locals.var_ty_dn7 = assign81720_e124324_d_n7;
        locals.var_ty_dn8 = assign81720_e124324_d_n8;
        locals.var_ty_dn9 = assign81720_e124324_d_n9;
        locals.var_ty_dn10 = assign81720_e124324_d_n10;
        locals.var_ty_dn11 = assign81720_e124324_d_n11;
        locals.var_ty_dn14 = assign81720_e124324_d_n14;

        let (assign81730_e124336, assign81730_e124336_d_n0, assign81730_e124336_d_n2, assign81730_e124336_d_n4, assign81730_e124336_d_n5, assign81730_e124336_d_n6, assign81730_e124336_d_n7, assign81730_e124336_d_n8, assign81730_e124336_d_n9, assign81730_e124336_d_n10, assign81730_e124336_d_n11, assign81730_e124336_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 != 0.0)) {
        let assign81730_e124331: f64 = (3.0 * 1.414213562373095);
        let assign81730_e124333: f64 = (assign81730_e124331 * locals.var_ty);
        let assign81730_e124334: f64 = (2.0 + assign81730_e124333);
        (assign81730_e124334, (assign81730_e124331 * locals.var_ty_dn0), (assign81730_e124331 * locals.var_ty_dn2), (assign81730_e124331 * locals.var_ty_dn4), (assign81730_e124331 * locals.var_ty_dn5), (assign81730_e124331 * locals.var_ty_dn6), (assign81730_e124331 * locals.var_ty_dn7), (assign81730_e124331 * locals.var_ty_dn8), (assign81730_e124331 * locals.var_ty_dn9), (assign81730_e124331 * locals.var_ty_dn10), (assign81730_e124331 * locals.var_ty_dn11), (assign81730_e124331 * locals.var_ty_dn14),)
    } else {
        (locals.var_ac41, locals.var_ac41_dn0, locals.var_ac41_dn2, locals.var_ac41_dn4, locals.var_ac41_dn5, locals.var_ac41_dn6, locals.var_ac41_dn7, locals.var_ac41_dn8, locals.var_ac41_dn9, locals.var_ac41_dn10, locals.var_ac41_dn11, locals.var_ac41_dn14,)
    }
};
        locals.var_ac41 = assign81730_e124336;
        locals.var_ac41_dn0 = assign81730_e124336_d_n0;
        locals.var_ac41_dn2 = assign81730_e124336_d_n2;
        locals.var_ac41_dn4 = assign81730_e124336_d_n4;
        locals.var_ac41_dn5 = assign81730_e124336_d_n5;
        locals.var_ac41_dn6 = assign81730_e124336_d_n6;
        locals.var_ac41_dn7 = assign81730_e124336_d_n7;
        locals.var_ac41_dn8 = assign81730_e124336_d_n8;
        locals.var_ac41_dn9 = assign81730_e124336_d_n9;
        locals.var_ac41_dn10 = assign81730_e124336_d_n10;
        locals.var_ac41_dn11 = assign81730_e124336_d_n11;
        locals.var_ac41_dn14 = assign81730_e124336_d_n14;

        let (assign81740_e124348, assign81740_e124348_d_n0, assign81740_e124348_d_n2, assign81740_e124348_d_n4, assign81740_e124348_d_n5, assign81740_e124348_d_n6, assign81740_e124348_d_n7, assign81740_e124348_d_n8, assign81740_e124348_d_n9, assign81740_e124348_d_n10, assign81740_e124348_d_n11, assign81740_e124348_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 != 0.0)) {
        let assign81740_e124342: f64 = (8.0 * locals.var_ac41);
        let assign81740_e124344: f64 = (assign81740_e124342 * locals.var_ac41);
        let assign81740_e124346: f64 = (assign81740_e124344 * locals.var_ac41);
        (assign81740_e124346, (((((8.0 * locals.var_ac41_dn0) * locals.var_ac41) + (assign81740_e124342 * locals.var_ac41_dn0)) * locals.var_ac41) + (assign81740_e124344 * locals.var_ac41_dn0)), (((((8.0 * locals.var_ac41_dn2) * locals.var_ac41) + (assign81740_e124342 * locals.var_ac41_dn2)) * locals.var_ac41) + (assign81740_e124344 * locals.var_ac41_dn2)), (((((8.0 * locals.var_ac41_dn4) * locals.var_ac41) + (assign81740_e124342 * locals.var_ac41_dn4)) * locals.var_ac41) + (assign81740_e124344 * locals.var_ac41_dn4)), (((((8.0 * locals.var_ac41_dn5) * locals.var_ac41) + (assign81740_e124342 * locals.var_ac41_dn5)) * locals.var_ac41) + (assign81740_e124344 * locals.var_ac41_dn5)), (((((8.0 * locals.var_ac41_dn6) * locals.var_ac41) + (assign81740_e124342 * locals.var_ac41_dn6)) * locals.var_ac41) + (assign81740_e124344 * locals.var_ac41_dn6)), (((((8.0 * locals.var_ac41_dn7) * locals.var_ac41) + (assign81740_e124342 * locals.var_ac41_dn7)) * locals.var_ac41) + (assign81740_e124344 * locals.var_ac41_dn7)), (((((8.0 * locals.var_ac41_dn8) * locals.var_ac41) + (assign81740_e124342 * locals.var_ac41_dn8)) * locals.var_ac41) + (assign81740_e124344 * locals.var_ac41_dn8)), (((((8.0 * locals.var_ac41_dn9) * locals.var_ac41) + (assign81740_e124342 * locals.var_ac41_dn9)) * locals.var_ac41) + (assign81740_e124344 * locals.var_ac41_dn9)), (((((8.0 * locals.var_ac41_dn10) * locals.var_ac41) + (assign81740_e124342 * locals.var_ac41_dn10)) * locals.var_ac41) + (assign81740_e124344 * locals.var_ac41_dn10)), (((((8.0 * locals.var_ac41_dn11) * locals.var_ac41) + (assign81740_e124342 * locals.var_ac41_dn11)) * locals.var_ac41) + (assign81740_e124344 * locals.var_ac41_dn11)), (((((8.0 * locals.var_ac41_dn14) * locals.var_ac41) + (assign81740_e124342 * locals.var_ac41_dn14)) * locals.var_ac41) + (assign81740_e124344 * locals.var_ac41_dn14)),)
    } else {
        (locals.var_ac4, locals.var_ac4_dn0, locals.var_ac4_dn2, locals.var_ac4_dn4, locals.var_ac4_dn5, locals.var_ac4_dn6, locals.var_ac4_dn7, locals.var_ac4_dn8, locals.var_ac4_dn9, locals.var_ac4_dn10, locals.var_ac4_dn11, locals.var_ac4_dn14,)
    }
};
        locals.var_ac4 = assign81740_e124348;
        locals.var_ac4_dn0 = assign81740_e124348_d_n0;
        locals.var_ac4_dn2 = assign81740_e124348_d_n2;
        locals.var_ac4_dn4 = assign81740_e124348_d_n4;
        locals.var_ac4_dn5 = assign81740_e124348_d_n5;
        locals.var_ac4_dn6 = assign81740_e124348_d_n6;
        locals.var_ac4_dn7 = assign81740_e124348_d_n7;
        locals.var_ac4_dn8 = assign81740_e124348_d_n8;
        locals.var_ac4_dn9 = assign81740_e124348_d_n9;
        locals.var_ac4_dn10 = assign81740_e124348_d_n10;
        locals.var_ac4_dn11 = assign81740_e124348_d_n11;
        locals.var_ac4_dn14 = assign81740_e124348_d_n14;

        let (assign81750_e124364, assign81750_e124364_d_n0, assign81750_e124364_d_n2, assign81750_e124364_d_n4, assign81750_e124364_d_n5, assign81750_e124364_d_n6, assign81750_e124364_d_n7, assign81750_e124364_d_n8, assign81750_e124364_d_n9, assign81750_e124364_d_n10, assign81750_e124364_d_n11, assign81750_e124364_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 != 0.0)) {
        let assign81750_e124354: f64 = (7.0 * 1.414213562373095);
        let assign81750_e124357: f64 = (9.0 * locals.var_ty);
        let assign81750_e124360: f64 = (locals.var_tx - 2.0);
        let assign81750_e124361: f64 = (assign81750_e124357 * assign81750_e124360);
        let assign81750_e124362: f64 = (assign81750_e124354 - assign81750_e124361);
        (assign81750_e124362, (-(((9.0 * locals.var_ty_dn0) * assign81750_e124360) + (assign81750_e124357 * locals.var_tx_dn0))), (-(((9.0 * locals.var_ty_dn2) * assign81750_e124360) + (assign81750_e124357 * locals.var_tx_dn2))), (-(((9.0 * locals.var_ty_dn4) * assign81750_e124360) + (assign81750_e124357 * locals.var_tx_dn4))), (-(((9.0 * locals.var_ty_dn5) * assign81750_e124360) + (assign81750_e124357 * locals.var_tx_dn5))), (-(((9.0 * locals.var_ty_dn6) * assign81750_e124360) + (assign81750_e124357 * locals.var_tx_dn6))), (-(((9.0 * locals.var_ty_dn7) * assign81750_e124360) + (assign81750_e124357 * locals.var_tx_dn7))), (-(((9.0 * locals.var_ty_dn8) * assign81750_e124360) + (assign81750_e124357 * locals.var_tx_dn8))), (-(((9.0 * locals.var_ty_dn9) * assign81750_e124360) + (assign81750_e124357 * locals.var_tx_dn9))), (-(((9.0 * locals.var_ty_dn10) * assign81750_e124360) + (assign81750_e124357 * locals.var_tx_dn10))), (-(((9.0 * locals.var_ty_dn11) * assign81750_e124360) + (assign81750_e124357 * locals.var_tx_dn11))), (-(((9.0 * locals.var_ty_dn14) * assign81750_e124360) + (assign81750_e124357 * locals.var_tx_dn14))),)
    } else {
        (locals.var_ac31, locals.var_ac31_dn0, locals.var_ac31_dn2, locals.var_ac31_dn4, locals.var_ac31_dn5, locals.var_ac31_dn6, locals.var_ac31_dn7, locals.var_ac31_dn8, locals.var_ac31_dn9, locals.var_ac31_dn10, locals.var_ac31_dn11, locals.var_ac31_dn14,)
    }
};
        locals.var_ac31 = assign81750_e124364;
        locals.var_ac31_dn0 = assign81750_e124364_d_n0;
        locals.var_ac31_dn2 = assign81750_e124364_d_n2;
        locals.var_ac31_dn4 = assign81750_e124364_d_n4;
        locals.var_ac31_dn5 = assign81750_e124364_d_n5;
        locals.var_ac31_dn6 = assign81750_e124364_d_n6;
        locals.var_ac31_dn7 = assign81750_e124364_d_n7;
        locals.var_ac31_dn8 = assign81750_e124364_d_n8;
        locals.var_ac31_dn9 = assign81750_e124364_d_n9;
        locals.var_ac31_dn10 = assign81750_e124364_d_n10;
        locals.var_ac31_dn11 = assign81750_e124364_d_n11;
        locals.var_ac31_dn14 = assign81750_e124364_d_n14;

        let (assign81760_e124372, assign81760_e124372_d_n0, assign81760_e124372_d_n2, assign81760_e124372_d_n4, assign81760_e124372_d_n5, assign81760_e124372_d_n6, assign81760_e124372_d_n7, assign81760_e124372_d_n8, assign81760_e124372_d_n9, assign81760_e124372_d_n10, assign81760_e124372_d_n11, assign81760_e124372_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 != 0.0)) {
        let assign81760_e124370: f64 = (locals.var_ac31 * locals.var_ac31);
        (assign81760_e124370, ((locals.var_ac31_dn0 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn0)), ((locals.var_ac31_dn2 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn2)), ((locals.var_ac31_dn4 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn4)), ((locals.var_ac31_dn5 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn5)), ((locals.var_ac31_dn6 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn6)), ((locals.var_ac31_dn7 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn7)), ((locals.var_ac31_dn8 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn8)), ((locals.var_ac31_dn9 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn9)), ((locals.var_ac31_dn10 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn10)), ((locals.var_ac31_dn11 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn11)), ((locals.var_ac31_dn14 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn14)),)
    } else {
        (locals.var_ac3, locals.var_ac3_dn0, locals.var_ac3_dn2, locals.var_ac3_dn4, locals.var_ac3_dn5, locals.var_ac3_dn6, locals.var_ac3_dn7, locals.var_ac3_dn8, locals.var_ac3_dn9, locals.var_ac3_dn10, locals.var_ac3_dn11, locals.var_ac3_dn14,)
    }
};
        locals.var_ac3 = assign81760_e124372;
        locals.var_ac3_dn0 = assign81760_e124372_d_n0;
        locals.var_ac3_dn2 = assign81760_e124372_d_n2;
        locals.var_ac3_dn4 = assign81760_e124372_d_n4;
        locals.var_ac3_dn5 = assign81760_e124372_d_n5;
        locals.var_ac3_dn6 = assign81760_e124372_d_n6;
        locals.var_ac3_dn7 = assign81760_e124372_d_n7;
        locals.var_ac3_dn8 = assign81760_e124372_d_n8;
        locals.var_ac3_dn9 = assign81760_e124372_d_n9;
        locals.var_ac3_dn10 = assign81760_e124372_d_n10;
        locals.var_ac3_dn11 = assign81760_e124372_d_n11;
        locals.var_ac3_dn14 = assign81760_e124372_d_n14;

        let assign81770_e124376: f64 = (locals.var_ac3 * 1e-8);
        let assign81770_e124377: f64 = if locals.var_ac4 < assign81770_e124376 { 1.0 } else { 0.0 };
        locals.var_guard1910 = assign81770_e124377;

        let (assign81790_e124398, assign81790_e124398_d_n0, assign81790_e124398_d_n2, assign81790_e124398_d_n4, assign81790_e124398_d_n5, assign81790_e124398_d_n6, assign81790_e124398_d_n7, assign81790_e124398_d_n8, assign81790_e124398_d_n9, assign81790_e124398_d_n10, assign81790_e124398_d_n11, assign81790_e124398_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 != 0.0)) && (locals.var_guard1910 != 0.0)) {
        let assign81790_e124394: f64 = (0.5 * locals.var_ac4);
        let assign81790_e124396: f64 = (assign81790_e124394 / locals.var_ac31);
        (assign81790_e124396, ((((0.5 * locals.var_ac4_dn0) * locals.var_ac31) - (assign81790_e124394 * locals.var_ac31_dn0)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn2) * locals.var_ac31) - (assign81790_e124394 * locals.var_ac31_dn2)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn4) * locals.var_ac31) - (assign81790_e124394 * locals.var_ac31_dn4)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn5) * locals.var_ac31) - (assign81790_e124394 * locals.var_ac31_dn5)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn6) * locals.var_ac31) - (assign81790_e124394 * locals.var_ac31_dn6)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn7) * locals.var_ac31) - (assign81790_e124394 * locals.var_ac31_dn7)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn8) * locals.var_ac31) - (assign81790_e124394 * locals.var_ac31_dn8)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn9) * locals.var_ac31) - (assign81790_e124394 * locals.var_ac31_dn9)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn10) * locals.var_ac31) - (assign81790_e124394 * locals.var_ac31_dn10)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn11) * locals.var_ac31) - (assign81790_e124394 * locals.var_ac31_dn11)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn14) * locals.var_ac31) - (assign81790_e124394 * locals.var_ac31_dn14)) / (locals.var_ac31 * locals.var_ac31)),)
    } else {
        (locals.var_ac1, locals.var_ac1_dn0, locals.var_ac1_dn2, locals.var_ac1_dn4, locals.var_ac1_dn5, locals.var_ac1_dn6, locals.var_ac1_dn7, locals.var_ac1_dn8, locals.var_ac1_dn9, locals.var_ac1_dn10, locals.var_ac1_dn11, locals.var_ac1_dn14,)
    }
};
        locals.var_ac1 = assign81790_e124398;
        locals.var_ac1_dn0 = assign81790_e124398_d_n0;
        locals.var_ac1_dn2 = assign81790_e124398_d_n2;
        locals.var_ac1_dn4 = assign81790_e124398_d_n4;
        locals.var_ac1_dn5 = assign81790_e124398_d_n5;
        locals.var_ac1_dn6 = assign81790_e124398_d_n6;
        locals.var_ac1_dn7 = assign81790_e124398_d_n7;
        locals.var_ac1_dn8 = assign81790_e124398_d_n8;
        locals.var_ac1_dn9 = assign81790_e124398_d_n9;
        locals.var_ac1_dn10 = assign81790_e124398_d_n10;
        locals.var_ac1_dn11 = assign81790_e124398_d_n11;
        locals.var_ac1_dn14 = assign81790_e124398_d_n14;

        let (assign81800_e124410, assign81800_e124410_d_n0, assign81800_e124410_d_n2, assign81800_e124410_d_n4, assign81800_e124410_d_n5, assign81800_e124410_d_n6, assign81800_e124410_d_n7, assign81800_e124410_d_n8, assign81800_e124410_d_n9, assign81800_e124410_d_n10, assign81800_e124410_d_n11, assign81800_e124410_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 != 0.0)) && (locals.var_guard1910 == 0.0)) {
        let assign81800_e124407: f64 = (locals.var_ac4 + locals.var_ac3);
        let assign81800_e124408: f64 = (assign81800_e124407).sqrt();
        (assign81800_e124408, ((locals.var_ac4_dn0 + locals.var_ac3_dn0) / (2.0 * assign81800_e124408)), ((locals.var_ac4_dn2 + locals.var_ac3_dn2) / (2.0 * assign81800_e124408)), ((locals.var_ac4_dn4 + locals.var_ac3_dn4) / (2.0 * assign81800_e124408)), ((locals.var_ac4_dn5 + locals.var_ac3_dn5) / (2.0 * assign81800_e124408)), ((locals.var_ac4_dn6 + locals.var_ac3_dn6) / (2.0 * assign81800_e124408)), ((locals.var_ac4_dn7 + locals.var_ac3_dn7) / (2.0 * assign81800_e124408)), ((locals.var_ac4_dn8 + locals.var_ac3_dn8) / (2.0 * assign81800_e124408)), ((locals.var_ac4_dn9 + locals.var_ac3_dn9) / (2.0 * assign81800_e124408)), ((locals.var_ac4_dn10 + locals.var_ac3_dn10) / (2.0 * assign81800_e124408)), ((locals.var_ac4_dn11 + locals.var_ac3_dn11) / (2.0 * assign81800_e124408)), ((locals.var_ac4_dn14 + locals.var_ac3_dn14) / (2.0 * assign81800_e124408)),)
    } else {
        (locals.var_ac2, locals.var_ac2_dn0, locals.var_ac2_dn2, locals.var_ac2_dn4, locals.var_ac2_dn5, locals.var_ac2_dn6, locals.var_ac2_dn7, locals.var_ac2_dn8, locals.var_ac2_dn9, locals.var_ac2_dn10, locals.var_ac2_dn11, locals.var_ac2_dn14,)
    }
};
        locals.var_ac2 = assign81800_e124410;
        locals.var_ac2_dn0 = assign81800_e124410_d_n0;
        locals.var_ac2_dn2 = assign81800_e124410_d_n2;
        locals.var_ac2_dn4 = assign81800_e124410_d_n4;
        locals.var_ac2_dn5 = assign81800_e124410_d_n5;
        locals.var_ac2_dn6 = assign81800_e124410_d_n6;
        locals.var_ac2_dn7 = assign81800_e124410_d_n7;
        locals.var_ac2_dn8 = assign81800_e124410_d_n8;
        locals.var_ac2_dn9 = assign81800_e124410_d_n9;
        locals.var_ac2_dn10 = assign81800_e124410_d_n10;
        locals.var_ac2_dn11 = assign81800_e124410_d_n11;
        locals.var_ac2_dn14 = assign81800_e124410_d_n14;

        let (assign81810_e124422, assign81810_e124422_d_n0, assign81810_e124422_d_n2, assign81810_e124422_d_n4, assign81810_e124422_d_n5, assign81810_e124422_d_n6, assign81810_e124422_d_n7, assign81810_e124422_d_n8, assign81810_e124422_d_n9, assign81810_e124422_d_n10, assign81810_e124422_d_n11, assign81810_e124422_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 != 0.0)) && (locals.var_guard1910 == 0.0)) {
        let assign81810_e124418: f64 = (-locals.var_ac31);
        let assign81810_e124420: f64 = (assign81810_e124418 + locals.var_ac2);
        (assign81810_e124420, ((-locals.var_ac31_dn0) + locals.var_ac2_dn0), ((-locals.var_ac31_dn2) + locals.var_ac2_dn2), ((-locals.var_ac31_dn4) + locals.var_ac2_dn4), ((-locals.var_ac31_dn5) + locals.var_ac2_dn5), ((-locals.var_ac31_dn6) + locals.var_ac2_dn6), ((-locals.var_ac31_dn7) + locals.var_ac2_dn7), ((-locals.var_ac31_dn8) + locals.var_ac2_dn8), ((-locals.var_ac31_dn9) + locals.var_ac2_dn9), ((-locals.var_ac31_dn10) + locals.var_ac2_dn10), ((-locals.var_ac31_dn11) + locals.var_ac2_dn11), ((-locals.var_ac31_dn14) + locals.var_ac2_dn14),)
    } else {
        (locals.var_ac1, locals.var_ac1_dn0, locals.var_ac1_dn2, locals.var_ac1_dn4, locals.var_ac1_dn5, locals.var_ac1_dn6, locals.var_ac1_dn7, locals.var_ac1_dn8, locals.var_ac1_dn9, locals.var_ac1_dn10, locals.var_ac1_dn11, locals.var_ac1_dn14,)
    }
};
        locals.var_ac1 = assign81810_e124422;
        locals.var_ac1_dn0 = assign81810_e124422_d_n0;
        locals.var_ac1_dn2 = assign81810_e124422_d_n2;
        locals.var_ac1_dn4 = assign81810_e124422_d_n4;
        locals.var_ac1_dn5 = assign81810_e124422_d_n5;
        locals.var_ac1_dn6 = assign81810_e124422_d_n6;
        locals.var_ac1_dn7 = assign81810_e124422_d_n7;
        locals.var_ac1_dn8 = assign81810_e124422_d_n8;
        locals.var_ac1_dn9 = assign81810_e124422_d_n9;
        locals.var_ac1_dn10 = assign81810_e124422_d_n10;
        locals.var_ac1_dn11 = assign81810_e124422_d_n11;
        locals.var_ac1_dn14 = assign81810_e124422_d_n14;

        let (assign81820_e124430, assign81820_e124430_d_n0, assign81820_e124430_d_n2, assign81820_e124430_d_n4, assign81820_e124430_d_n5, assign81820_e124430_d_n6, assign81820_e124430_d_n7, assign81820_e124430_d_n8, assign81820_e124430_d_n9, assign81820_e124430_d_n10, assign81820_e124430_d_n11, assign81820_e124430_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 != 0.0)) {
        let assign81820_e124428: f64 = (locals.var_ac1).powf(0.3333333333333333);
        (assign81820_e124428, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn0)) } } else { (assign81820_e124428 * (0.3333333333333333 * (locals.var_ac1_dn0 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn2)) } } else { (assign81820_e124428 * (0.3333333333333333 * (locals.var_ac1_dn2 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn4)) } } else { (assign81820_e124428 * (0.3333333333333333 * (locals.var_ac1_dn4 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn5)) } } else { (assign81820_e124428 * (0.3333333333333333 * (locals.var_ac1_dn5 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn6)) } } else { (assign81820_e124428 * (0.3333333333333333 * (locals.var_ac1_dn6 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn7)) } } else { (assign81820_e124428 * (0.3333333333333333 * (locals.var_ac1_dn7 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn8)) } } else { (assign81820_e124428 * (0.3333333333333333 * (locals.var_ac1_dn8 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn9)) } } else { (assign81820_e124428 * (0.3333333333333333 * (locals.var_ac1_dn9 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn10)) } } else { (assign81820_e124428 * (0.3333333333333333 * (locals.var_ac1_dn10 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn11)) } } else { (assign81820_e124428 * (0.3333333333333333 * (locals.var_ac1_dn11 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn14)) } } else { (assign81820_e124428 * (0.3333333333333333 * (locals.var_ac1_dn14 / locals.var_ac1))) },)
    } else {
        (locals.var_acd, locals.var_acd_dn0, locals.var_acd_dn2, locals.var_acd_dn4, locals.var_acd_dn5, locals.var_acd_dn6, locals.var_acd_dn7, locals.var_acd_dn8, locals.var_acd_dn9, locals.var_acd_dn10, locals.var_acd_dn11, locals.var_acd_dn14,)
    }
};
        locals.var_acd = assign81820_e124430;
        locals.var_acd_dn0 = assign81820_e124430_d_n0;
        locals.var_acd_dn2 = assign81820_e124430_d_n2;
        locals.var_acd_dn4 = assign81820_e124430_d_n4;
        locals.var_acd_dn5 = assign81820_e124430_d_n5;
        locals.var_acd_dn6 = assign81820_e124430_d_n6;
        locals.var_acd_dn7 = assign81820_e124430_d_n7;
        locals.var_acd_dn8 = assign81820_e124430_d_n8;
        locals.var_acd_dn9 = assign81820_e124430_d_n9;
        locals.var_acd_dn10 = assign81820_e124430_d_n10;
        locals.var_acd_dn11 = assign81820_e124430_d_n11;
        locals.var_acd_dn14 = assign81820_e124430_d_n14;

    }

    pub(super) fn stamp_transient_block_296(
        locals: &mut StampLocals,
    ) {
        let (assign81830_e124453, assign81830_e124453_d_n0, assign81830_e124453_d_n2, assign81830_e124453_d_n4, assign81830_e124453_d_n5, assign81830_e124453_d_n6, assign81830_e124453_d_n7, assign81830_e124453_d_n8, assign81830_e124453_d_n9, assign81830_e124453_d_n10, assign81830_e124453_d_n11, assign81830_e124453_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 != 0.0)) {
        let assign81830_e124435: f64 = (-4.0);
        let assign81830_e124437: f64 = (assign81830_e124435 * 1.414213562373095);
        let assign81830_e124440: f64 = (12.0 * locals.var_ty);
        let assign81830_e124441: f64 = (assign81830_e124437 - assign81830_e124440);
        let assign81830_e124444: f64 = (2.0 * locals.var_acd);
        let assign81830_e124445: f64 = (assign81830_e124441 + assign81830_e124444);
        let assign81830_e124448: f64 = (1.414213562373095 * locals.var_acd);
        let assign81830_e124450: f64 = (assign81830_e124448 * locals.var_acd);
        let assign81830_e124451: f64 = (assign81830_e124445 + assign81830_e124450);
        (assign81830_e124451, (((-(12.0 * locals.var_ty_dn0)) + (2.0 * locals.var_acd_dn0)) + (((1.414213562373095 * locals.var_acd_dn0) * locals.var_acd) + (assign81830_e124448 * locals.var_acd_dn0))), (((-(12.0 * locals.var_ty_dn2)) + (2.0 * locals.var_acd_dn2)) + (((1.414213562373095 * locals.var_acd_dn2) * locals.var_acd) + (assign81830_e124448 * locals.var_acd_dn2))), (((-(12.0 * locals.var_ty_dn4)) + (2.0 * locals.var_acd_dn4)) + (((1.414213562373095 * locals.var_acd_dn4) * locals.var_acd) + (assign81830_e124448 * locals.var_acd_dn4))), (((-(12.0 * locals.var_ty_dn5)) + (2.0 * locals.var_acd_dn5)) + (((1.414213562373095 * locals.var_acd_dn5) * locals.var_acd) + (assign81830_e124448 * locals.var_acd_dn5))), (((-(12.0 * locals.var_ty_dn6)) + (2.0 * locals.var_acd_dn6)) + (((1.414213562373095 * locals.var_acd_dn6) * locals.var_acd) + (assign81830_e124448 * locals.var_acd_dn6))), (((-(12.0 * locals.var_ty_dn7)) + (2.0 * locals.var_acd_dn7)) + (((1.414213562373095 * locals.var_acd_dn7) * locals.var_acd) + (assign81830_e124448 * locals.var_acd_dn7))), (((-(12.0 * locals.var_ty_dn8)) + (2.0 * locals.var_acd_dn8)) + (((1.414213562373095 * locals.var_acd_dn8) * locals.var_acd) + (assign81830_e124448 * locals.var_acd_dn8))), (((-(12.0 * locals.var_ty_dn9)) + (2.0 * locals.var_acd_dn9)) + (((1.414213562373095 * locals.var_acd_dn9) * locals.var_acd) + (assign81830_e124448 * locals.var_acd_dn9))), (((-(12.0 * locals.var_ty_dn10)) + (2.0 * locals.var_acd_dn10)) + (((1.414213562373095 * locals.var_acd_dn10) * locals.var_acd) + (assign81830_e124448 * locals.var_acd_dn10))), (((-(12.0 * locals.var_ty_dn11)) + (2.0 * locals.var_acd_dn11)) + (((1.414213562373095 * locals.var_acd_dn11) * locals.var_acd) + (assign81830_e124448 * locals.var_acd_dn11))), (((-(12.0 * locals.var_ty_dn14)) + (2.0 * locals.var_acd_dn14)) + (((1.414213562373095 * locals.var_acd_dn14) * locals.var_acd) + (assign81830_e124448 * locals.var_acd_dn14))),)
    } else {
        (locals.var_acn, locals.var_acn_dn0, locals.var_acn_dn2, locals.var_acn_dn4, locals.var_acn_dn5, locals.var_acn_dn6, locals.var_acn_dn7, locals.var_acn_dn8, locals.var_acn_dn9, locals.var_acn_dn10, locals.var_acn_dn11, locals.var_acn_dn14,)
    }
};
        locals.var_acn = assign81830_e124453;
        locals.var_acn_dn0 = assign81830_e124453_d_n0;
        locals.var_acn_dn2 = assign81830_e124453_d_n2;
        locals.var_acn_dn4 = assign81830_e124453_d_n4;
        locals.var_acn_dn5 = assign81830_e124453_d_n5;
        locals.var_acn_dn6 = assign81830_e124453_d_n6;
        locals.var_acn_dn7 = assign81830_e124453_d_n7;
        locals.var_acn_dn8 = assign81830_e124453_d_n8;
        locals.var_acn_dn9 = assign81830_e124453_d_n9;
        locals.var_acn_dn10 = assign81830_e124453_d_n10;
        locals.var_acn_dn11 = assign81830_e124453_d_n11;
        locals.var_acn_dn14 = assign81830_e124453_d_n14;

        let (assign81840_e124461, assign81840_e124461_d_n0, assign81840_e124461_d_n2, assign81840_e124461_d_n4, assign81840_e124461_d_n5, assign81840_e124461_d_n6, assign81840_e124461_d_n7, assign81840_e124461_d_n8, assign81840_e124461_d_n9, assign81840_e124461_d_n10, assign81840_e124461_d_n11, assign81840_e124461_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 != 0.0)) {
        let assign81840_e124459: f64 = (locals.var_acn / locals.var_acd);
        (assign81840_e124459, (((locals.var_acn_dn0 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn0)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn2 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn2)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn4 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn4)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn5 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn5)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn6 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn6)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn7 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn7)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn8 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn8)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn9 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn9)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn10 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn10)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn11 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn11)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn14 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn14)) / (locals.var_acd * locals.var_acd)),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
        locals.var_chi = assign81840_e124461;
        locals.var_chi_dn0 = assign81840_e124461_d_n0;
        locals.var_chi_dn2 = assign81840_e124461_d_n2;
        locals.var_chi_dn4 = assign81840_e124461_d_n4;
        locals.var_chi_dn5 = assign81840_e124461_d_n5;
        locals.var_chi_dn6 = assign81840_e124461_d_n6;
        locals.var_chi_dn7 = assign81840_e124461_d_n7;
        locals.var_chi_dn8 = assign81840_e124461_d_n8;
        locals.var_chi_dn9 = assign81840_e124461_d_n9;
        locals.var_chi_dn10 = assign81840_e124461_d_n10;
        locals.var_chi_dn11 = assign81840_e124461_d_n11;
        locals.var_chi_dn14 = assign81840_e124461_d_n14;

        let (assign81850_e124469, assign81850_e124469_d_n0, assign81850_e124469_d_n2, assign81850_e124469_d_n4, assign81850_e124469_d_n5, assign81850_e124469_d_n6, assign81850_e124469_d_n7, assign81850_e124469_d_n8, assign81850_e124469_d_n9, assign81850_e124469_d_n10, assign81850_e124469_d_n11, assign81850_e124469_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 != 0.0)) {
        let assign81850_e124467: f64 = (locals.var_chi * locals.var_beta_inv);
        (assign81850_e124467, ((locals.var_chi_dn0 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn0)), ((locals.var_chi_dn2 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn2)), ((locals.var_chi_dn4 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn4)), ((locals.var_chi_dn5 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn5)), ((locals.var_chi_dn6 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn6)), ((locals.var_chi_dn7 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn7)), ((locals.var_chi_dn8 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn8)), ((locals.var_chi_dn9 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn9)), ((locals.var_chi_dn10 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn10)), ((locals.var_chi_dn11 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn11)), ((locals.var_chi_dn14 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign81850_e124469;
        locals.var_t1_dn0 = assign81850_e124469_d_n0;
        locals.var_t1_dn2 = assign81850_e124469_d_n2;
        locals.var_t1_dn4 = assign81850_e124469_d_n4;
        locals.var_t1_dn5 = assign81850_e124469_d_n5;
        locals.var_t1_dn6 = assign81850_e124469_d_n6;
        locals.var_t1_dn7 = assign81850_e124469_d_n7;
        locals.var_t1_dn8 = assign81850_e124469_d_n8;
        locals.var_t1_dn9 = assign81850_e124469_d_n9;
        locals.var_t1_dn10 = assign81850_e124469_d_n10;
        locals.var_t1_dn11 = assign81850_e124469_d_n11;
        locals.var_t1_dn14 = assign81850_e124469_d_n14;

        let (assign81860_e124477, assign81860_e124477_d_n0, assign81860_e124477_d_n2, assign81860_e124477_d_n4, assign81860_e124477_d_n5, assign81860_e124477_d_n6, assign81860_e124477_d_n7, assign81860_e124477_d_n8, assign81860_e124477_d_n9, assign81860_e124477_d_n10, assign81860_e124477_d_n11, assign81860_e124477_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 != 0.0)) {
        let assign81860_e124475: f64 = (locals.var_t1 / locals.var_ps0_min);
        (assign81860_e124475, (((locals.var_t1_dn0 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn0)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn2 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn2)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn4 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn4)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn5 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn5)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn6 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn6)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn7 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn7)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn8 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn8)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn9 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn9)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn10 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn10)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn11 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn11)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn14 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn14)) / (locals.var_ps0_min * locals.var_ps0_min)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign81860_e124477;
        locals.var_t2_dn0 = assign81860_e124477_d_n0;
        locals.var_t2_dn2 = assign81860_e124477_d_n2;
        locals.var_t2_dn4 = assign81860_e124477_d_n4;
        locals.var_t2_dn5 = assign81860_e124477_d_n5;
        locals.var_t2_dn6 = assign81860_e124477_d_n6;
        locals.var_t2_dn7 = assign81860_e124477_d_n7;
        locals.var_t2_dn8 = assign81860_e124477_d_n8;
        locals.var_t2_dn9 = assign81860_e124477_d_n9;
        locals.var_t2_dn10 = assign81860_e124477_d_n10;
        locals.var_t2_dn11 = assign81860_e124477_d_n11;
        locals.var_t2_dn14 = assign81860_e124477_d_n14;

        let (assign81870_e124488, assign81870_e124488_d_n0, assign81870_e124488_d_n2, assign81870_e124488_d_n4, assign81870_e124488_d_n5, assign81870_e124488_d_n6, assign81870_e124488_d_n7, assign81870_e124488_d_n8, assign81870_e124488_d_n9, assign81870_e124488_d_n10, assign81870_e124488_d_n11, assign81870_e124488_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 != 0.0)) {
        let assign81870_e124484: f64 = (locals.var_t2 * locals.var_t2);
        let assign81870_e124485: f64 = (1.0 + assign81870_e124484);
        let assign81870_e124486: f64 = (assign81870_e124485).sqrt();
        (assign81870_e124486, (((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)) / (2.0 * assign81870_e124486)), (((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)) / (2.0 * assign81870_e124486)), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign81870_e124486)), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign81870_e124486)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign81870_e124486)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign81870_e124486)), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign81870_e124486)), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign81870_e124486)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign81870_e124486)), (((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign81870_e124486)), (((locals.var_t2_dn14 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn14)) / (2.0 * assign81870_e124486)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign81870_e124488;
        locals.var_t3_dn0 = assign81870_e124488_d_n0;
        locals.var_t3_dn2 = assign81870_e124488_d_n2;
        locals.var_t3_dn4 = assign81870_e124488_d_n4;
        locals.var_t3_dn5 = assign81870_e124488_d_n5;
        locals.var_t3_dn6 = assign81870_e124488_d_n6;
        locals.var_t3_dn7 = assign81870_e124488_d_n7;
        locals.var_t3_dn8 = assign81870_e124488_d_n8;
        locals.var_t3_dn9 = assign81870_e124488_d_n9;
        locals.var_t3_dn10 = assign81870_e124488_d_n10;
        locals.var_t3_dn11 = assign81870_e124488_d_n11;
        locals.var_t3_dn14 = assign81870_e124488_d_n14;

        let (assign81880_e124498, assign81880_e124498_d_n0, assign81880_e124498_d_n2, assign81880_e124498_d_n4, assign81880_e124498_d_n5, assign81880_e124498_d_n6, assign81880_e124498_d_n7, assign81880_e124498_d_n8, assign81880_e124498_d_n9, assign81880_e124498_d_n10, assign81880_e124498_d_n11, assign81880_e124498_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 != 0.0)) {
        let assign81880_e124494: f64 = (locals.var_t1 / locals.var_t3);
        let assign81880_e124496: f64 = (assign81880_e124494 - locals.var_vxbgmtcl);
        (assign81880_e124496, ((((locals.var_t1_dn0 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn0)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn0), ((((locals.var_t1_dn2 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn2)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn2), ((((locals.var_t1_dn4 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn4)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn4), ((((locals.var_t1_dn5 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn5)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn5), ((((locals.var_t1_dn6 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn6)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn6), ((((locals.var_t1_dn7 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn7)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn7), ((((locals.var_t1_dn8 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn8)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn8), ((((locals.var_t1_dn9 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn9)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn9), ((((locals.var_t1_dn10 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn10)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn10), ((((locals.var_t1_dn11 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn11)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn11), ((((locals.var_t1_dn14 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn14)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn14),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn14,)
    }
};
        locals.var_ps0ld = assign81880_e124498;
        locals.var_ps0ld_dn0 = assign81880_e124498_d_n0;
        locals.var_ps0ld_dn2 = assign81880_e124498_d_n2;
        locals.var_ps0ld_dn4 = assign81880_e124498_d_n4;
        locals.var_ps0ld_dn5 = assign81880_e124498_d_n5;
        locals.var_ps0ld_dn6 = assign81880_e124498_d_n6;
        locals.var_ps0ld_dn7 = assign81880_e124498_d_n7;
        locals.var_ps0ld_dn8 = assign81880_e124498_d_n8;
        locals.var_ps0ld_dn9 = assign81880_e124498_d_n9;
        locals.var_ps0ld_dn10 = assign81880_e124498_d_n10;
        locals.var_ps0ld_dn11 = assign81880_e124498_d_n11;
        locals.var_ps0ld_dn14 = assign81880_e124498_d_n14;

        let (assign81890_e124506, assign81890_e124506_d_n0, assign81890_e124506_d_n2, assign81890_e124506_d_n4, assign81890_e124506_d_n5, assign81890_e124506_d_n6, assign81890_e124506_d_n7, assign81890_e124506_d_n8, assign81890_e124506_d_n9, assign81890_e124506_d_n10, assign81890_e124506_d_n11, assign81890_e124506_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 != 0.0)) {
        let assign81890_e124504: f64 = (locals.var_vgpld - locals.var_ps0ld);
        (assign81890_e124504, (-locals.var_ps0ld_dn0), (locals.var_vgpld_dn2 - locals.var_ps0ld_dn2), (-locals.var_ps0ld_dn4), (-locals.var_ps0ld_dn5), (-locals.var_ps0ld_dn6), (locals.var_vgpld_dn7 - locals.var_ps0ld_dn7), (locals.var_vgpld_dn8 - locals.var_ps0ld_dn8), (locals.var_vgpld_dn9 - locals.var_ps0ld_dn9), (-locals.var_ps0ld_dn10), (-locals.var_ps0ld_dn11), (-locals.var_ps0ld_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign81890_e124506;
        locals.var_t2_dn0 = assign81890_e124506_d_n0;
        locals.var_t2_dn2 = assign81890_e124506_d_n2;
        locals.var_t2_dn4 = assign81890_e124506_d_n4;
        locals.var_t2_dn5 = assign81890_e124506_d_n5;
        locals.var_t2_dn6 = assign81890_e124506_d_n6;
        locals.var_t2_dn7 = assign81890_e124506_d_n7;
        locals.var_t2_dn8 = assign81890_e124506_d_n8;
        locals.var_t2_dn9 = assign81890_e124506_d_n9;
        locals.var_t2_dn10 = assign81890_e124506_d_n10;
        locals.var_t2_dn11 = assign81890_e124506_d_n11;
        locals.var_t2_dn14 = assign81890_e124506_d_n14;

        let (assign81900_e124514, assign81900_e124514_d_n0, assign81900_e124514_d_n2, assign81900_e124514_d_n4, assign81900_e124514_d_n5, assign81900_e124514_d_n6, assign81900_e124514_d_n7, assign81900_e124514_d_n8, assign81900_e124514_d_n9, assign81900_e124514_d_n10, assign81900_e124514_d_n11, assign81900_e124514_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 != 0.0)) {
        let assign81900_e124512: f64 = (locals.var_cox0_func * locals.var_t2);
        (assign81900_e124512, (locals.var_cox0_func * locals.var_t2_dn0), (locals.var_cox0_func * locals.var_t2_dn2), (locals.var_cox0_func * locals.var_t2_dn4), (locals.var_cox0_func * locals.var_t2_dn5), (locals.var_cox0_func * locals.var_t2_dn6), (locals.var_cox0_func * locals.var_t2_dn7), (locals.var_cox0_func * locals.var_t2_dn8), (locals.var_cox0_func * locals.var_t2_dn9), (locals.var_cox0_func * locals.var_t2_dn10), (locals.var_cox0_func * locals.var_t2_dn11), (locals.var_cox0_func * locals.var_t2_dn14),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn4, locals.var_qsuld_dn5, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn8, locals.var_qsuld_dn9, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn14,)
    }
};
        locals.var_qsuld = assign81900_e124514;
        locals.var_qsuld_dn0 = assign81900_e124514_d_n0;
        locals.var_qsuld_dn2 = assign81900_e124514_d_n2;
        locals.var_qsuld_dn4 = assign81900_e124514_d_n4;
        locals.var_qsuld_dn5 = assign81900_e124514_d_n5;
        locals.var_qsuld_dn6 = assign81900_e124514_d_n6;
        locals.var_qsuld_dn7 = assign81900_e124514_d_n7;
        locals.var_qsuld_dn8 = assign81900_e124514_d_n8;
        locals.var_qsuld_dn9 = assign81900_e124514_d_n9;
        locals.var_qsuld_dn10 = assign81900_e124514_d_n10;
        locals.var_qsuld_dn11 = assign81900_e124514_d_n11;
        locals.var_qsuld_dn14 = assign81900_e124514_d_n14;

        let (assign81910_e124520, assign81910_e124520_d_n0, assign81910_e124520_d_n2, assign81910_e124520_d_n4, assign81910_e124520_d_n5, assign81910_e124520_d_n6, assign81910_e124520_d_n7, assign81910_e124520_d_n8, assign81910_e124520_d_n9, assign81910_e124520_d_n10, assign81910_e124520_d_n11, assign81910_e124520_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 != 0.0)) {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn4, locals.var_qsuld_dn5, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn8, locals.var_qsuld_dn9, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn14,)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn8, locals.var_qbuld_dn9, locals.var_qbuld_dn10, locals.var_qbuld_dn11, locals.var_qbuld_dn14,)
    }
};
        locals.var_qbuld = assign81910_e124520;
        locals.var_qbuld_dn0 = assign81910_e124520_d_n0;
        locals.var_qbuld_dn2 = assign81910_e124520_d_n2;
        locals.var_qbuld_dn4 = assign81910_e124520_d_n4;
        locals.var_qbuld_dn5 = assign81910_e124520_d_n5;
        locals.var_qbuld_dn6 = assign81910_e124520_d_n6;
        locals.var_qbuld_dn7 = assign81910_e124520_d_n7;
        locals.var_qbuld_dn8 = assign81910_e124520_d_n8;
        locals.var_qbuld_dn9 = assign81910_e124520_d_n9;
        locals.var_qbuld_dn10 = assign81910_e124520_d_n10;
        locals.var_qbuld_dn11 = assign81910_e124520_d_n11;
        locals.var_qbuld_dn14 = assign81910_e124520_d_n14;

        let (assign81920_e124526, assign81920_e124526_d_n0, assign81920_e124526_d_n2, assign81920_e124526_d_n4, assign81920_e124526_d_n5, assign81920_e124526_d_n6, assign81920_e124526_d_n7, assign81920_e124526_d_n8, assign81920_e124526_d_n9, assign81920_e124526_d_n10, assign81920_e124526_d_n11, assign81920_e124526_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 != 0.0)) {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn14,)
    } else {
        (locals.var_ps0ld_ini__blk1892, locals.var_ps0ld_ini__blk1892_dn0, locals.var_ps0ld_ini__blk1892_dn2, locals.var_ps0ld_ini__blk1892_dn4, locals.var_ps0ld_ini__blk1892_dn5, locals.var_ps0ld_ini__blk1892_dn6, locals.var_ps0ld_ini__blk1892_dn7, locals.var_ps0ld_ini__blk1892_dn8, locals.var_ps0ld_ini__blk1892_dn9, locals.var_ps0ld_ini__blk1892_dn10, locals.var_ps0ld_ini__blk1892_dn11, locals.var_ps0ld_ini__blk1892_dn14,)
    }
};
        locals.var_ps0ld_ini__blk1892 = assign81920_e124526;
        locals.var_ps0ld_ini__blk1892_dn0 = assign81920_e124526_d_n0;
        locals.var_ps0ld_ini__blk1892_dn2 = assign81920_e124526_d_n2;
        locals.var_ps0ld_ini__blk1892_dn4 = assign81920_e124526_d_n4;
        locals.var_ps0ld_ini__blk1892_dn5 = assign81920_e124526_d_n5;
        locals.var_ps0ld_ini__blk1892_dn6 = assign81920_e124526_d_n6;
        locals.var_ps0ld_ini__blk1892_dn7 = assign81920_e124526_d_n7;
        locals.var_ps0ld_ini__blk1892_dn8 = assign81920_e124526_d_n8;
        locals.var_ps0ld_ini__blk1892_dn9 = assign81920_e124526_d_n9;
        locals.var_ps0ld_ini__blk1892_dn10 = assign81920_e124526_d_n10;
        locals.var_ps0ld_ini__blk1892_dn11 = assign81920_e124526_d_n11;
        locals.var_ps0ld_ini__blk1892_dn14 = assign81920_e124526_d_n14;

        let assign81930_e124530: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign81930_e124531: f64 = (locals.var_beta * assign81930_e124530);
        let assign81930_e124535: f64 = (10.0 * 2.220446049250313e-16);
        let assign81930_e124537: f64 = (assign81930_e124535 - 1.0);
        let assign81930_e124539: f64 = (assign81930_e124537 * locals.var_fac1p2);
        let assign81930_e124541: f64 = (assign81930_e124539 * locals.var_beta2);
        let assign81930_e124543: f64 = (assign81930_e124541 / 4.0);
        let assign81930_e124544: f64 = (1.0 + assign81930_e124543);
        let assign81930_e124545: f64 = if assign81930_e124531 < assign81930_e124544 { 1.0 } else { 0.0 };
        locals.var_guard1911 = assign81930_e124545;

        let (assign81940_e124560, assign81940_e124560_d_n0, assign81940_e124560_d_n2, assign81940_e124560_d_n4, assign81940_e124560_d_n5, assign81940_e124560_d_n6, assign81940_e124560_d_n7, assign81940_e124560_d_n8, assign81940_e124560_d_n9, assign81940_e124560_d_n10, assign81940_e124560_d_n11, assign81940_e124560_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1911 != 0.0)) {
        let assign81940_e124555: f64 = (locals.var_fac1p2 * locals.var_beta);
        let assign81940_e124557: f64 = (assign81940_e124555 / 2.0);
        let assign81940_e124558: f64 = (locals.var_vgpld + assign81940_e124557);
        (assign81940_e124558, (((locals.var_fac1p2_dn0 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn0)) / 2.0), (locals.var_vgpld_dn2 + (((locals.var_fac1p2_dn2 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn2)) / 2.0)), (((locals.var_fac1p2_dn4 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn4)) / 2.0), (((locals.var_fac1p2_dn5 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn5)) / 2.0), (((locals.var_fac1p2_dn6 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn6)) / 2.0), (locals.var_vgpld_dn7 + (((locals.var_fac1p2_dn7 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn7)) / 2.0)), (locals.var_vgpld_dn8 + (((locals.var_fac1p2_dn8 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn8)) / 2.0)), (locals.var_vgpld_dn9 + (((locals.var_fac1p2_dn9 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn9)) / 2.0)), (((locals.var_fac1p2_dn10 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn10)) / 2.0), (((locals.var_fac1p2_dn11 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn11)) / 2.0), (((locals.var_fac1p2_dn14 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn14)) / 2.0),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn14,)
    }
};
        locals.var_ps0_inia = assign81940_e124560;
        locals.var_ps0_inia_dn0 = assign81940_e124560_d_n0;
        locals.var_ps0_inia_dn2 = assign81940_e124560_d_n2;
        locals.var_ps0_inia_dn4 = assign81940_e124560_d_n4;
        locals.var_ps0_inia_dn5 = assign81940_e124560_d_n5;
        locals.var_ps0_inia_dn6 = assign81940_e124560_d_n6;
        locals.var_ps0_inia_dn7 = assign81940_e124560_d_n7;
        locals.var_ps0_inia_dn8 = assign81940_e124560_d_n8;
        locals.var_ps0_inia_dn9 = assign81940_e124560_d_n9;
        locals.var_ps0_inia_dn10 = assign81940_e124560_d_n10;
        locals.var_ps0_inia_dn11 = assign81940_e124560_d_n11;
        locals.var_ps0_inia_dn14 = assign81940_e124560_d_n14;

        let (assign81950_e124584, assign81950_e124584_d_n0, assign81950_e124584_d_n2, assign81950_e124584_d_n4, assign81950_e124584_d_n5, assign81950_e124584_d_n6, assign81950_e124584_d_n7, assign81950_e124584_d_n8, assign81950_e124584_d_n9, assign81950_e124584_d_n10, assign81950_e124584_d_n11, assign81950_e124584_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1911 == 0.0)) {
        let assign81950_e124573: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign81950_e124574: f64 = (locals.var_beta * assign81950_e124573);
        let assign81950_e124576: f64 = (assign81950_e124574 - 1.0);
        let assign81950_e124577: f64 = (4.0 * assign81950_e124576);
        let assign81950_e124580: f64 = (locals.var_fac1p2 * locals.var_beta2);
        let assign81950_e124581: f64 = (assign81950_e124577 / assign81950_e124580);
        let assign81950_e124582: f64 = (1.0 + assign81950_e124581);
        (assign81950_e124582, ((((4.0 * ((locals.var_beta_dn0 * assign81950_e124573) + (locals.var_beta * locals.var_vxbgmtcl_dn0))) * assign81950_e124580) - (assign81950_e124577 * ((locals.var_fac1p2_dn0 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn0)))) / (assign81950_e124580 * assign81950_e124580)), ((((4.0 * ((locals.var_beta_dn2 * assign81950_e124573) + (locals.var_beta * (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2)))) * assign81950_e124580) - (assign81950_e124577 * ((locals.var_fac1p2_dn2 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn2)))) / (assign81950_e124580 * assign81950_e124580)), ((((4.0 * ((locals.var_beta_dn4 * assign81950_e124573) + (locals.var_beta * locals.var_vxbgmtcl_dn4))) * assign81950_e124580) - (assign81950_e124577 * ((locals.var_fac1p2_dn4 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn4)))) / (assign81950_e124580 * assign81950_e124580)), ((((4.0 * ((locals.var_beta_dn5 * assign81950_e124573) + (locals.var_beta * locals.var_vxbgmtcl_dn5))) * assign81950_e124580) - (assign81950_e124577 * ((locals.var_fac1p2_dn5 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn5)))) / (assign81950_e124580 * assign81950_e124580)), ((((4.0 * ((locals.var_beta_dn6 * assign81950_e124573) + (locals.var_beta * locals.var_vxbgmtcl_dn6))) * assign81950_e124580) - (assign81950_e124577 * ((locals.var_fac1p2_dn6 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn6)))) / (assign81950_e124580 * assign81950_e124580)), ((((4.0 * ((locals.var_beta_dn7 * assign81950_e124573) + (locals.var_beta * (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7)))) * assign81950_e124580) - (assign81950_e124577 * ((locals.var_fac1p2_dn7 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn7)))) / (assign81950_e124580 * assign81950_e124580)), ((((4.0 * ((locals.var_beta_dn8 * assign81950_e124573) + (locals.var_beta * (locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8)))) * assign81950_e124580) - (assign81950_e124577 * ((locals.var_fac1p2_dn8 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn8)))) / (assign81950_e124580 * assign81950_e124580)), ((((4.0 * ((locals.var_beta_dn9 * assign81950_e124573) + (locals.var_beta * (locals.var_vgpld_dn9 + locals.var_vxbgmtcl_dn9)))) * assign81950_e124580) - (assign81950_e124577 * ((locals.var_fac1p2_dn9 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn9)))) / (assign81950_e124580 * assign81950_e124580)), ((((4.0 * ((locals.var_beta_dn10 * assign81950_e124573) + (locals.var_beta * locals.var_vxbgmtcl_dn10))) * assign81950_e124580) - (assign81950_e124577 * ((locals.var_fac1p2_dn10 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn10)))) / (assign81950_e124580 * assign81950_e124580)), ((((4.0 * ((locals.var_beta_dn11 * assign81950_e124573) + (locals.var_beta * locals.var_vxbgmtcl_dn11))) * assign81950_e124580) - (assign81950_e124577 * ((locals.var_fac1p2_dn11 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn11)))) / (assign81950_e124580 * assign81950_e124580)), ((((4.0 * ((locals.var_beta_dn14 * assign81950_e124573) + (locals.var_beta * locals.var_vxbgmtcl_dn14))) * assign81950_e124580) - (assign81950_e124577 * ((locals.var_fac1p2_dn14 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn14)))) / (assign81950_e124580 * assign81950_e124580)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign81950_e124584;
        locals.var_tx_dn0 = assign81950_e124584_d_n0;
        locals.var_tx_dn2 = assign81950_e124584_d_n2;
        locals.var_tx_dn4 = assign81950_e124584_d_n4;
        locals.var_tx_dn5 = assign81950_e124584_d_n5;
        locals.var_tx_dn6 = assign81950_e124584_d_n6;
        locals.var_tx_dn7 = assign81950_e124584_d_n7;
        locals.var_tx_dn8 = assign81950_e124584_d_n8;
        locals.var_tx_dn9 = assign81950_e124584_d_n9;
        locals.var_tx_dn10 = assign81950_e124584_d_n10;
        locals.var_tx_dn11 = assign81950_e124584_d_n11;
        locals.var_tx_dn14 = assign81950_e124584_d_n14;

        let (assign81960_e124605, assign81960_e124605_d_n0, assign81960_e124605_d_n2, assign81960_e124605_d_n4, assign81960_e124605_d_n5, assign81960_e124605_d_n6, assign81960_e124605_d_n7, assign81960_e124605_d_n8, assign81960_e124605_d_n9, assign81960_e124605_d_n10, assign81960_e124605_d_n11, assign81960_e124605_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1911 == 0.0)) {
        let assign81960_e124595: f64 = (locals.var_fac1p2 * locals.var_beta);
        let assign81960_e124597: f64 = (assign81960_e124595 / 2.0);
        let assign81960_e124600: f64 = (locals.var_tx).sqrt();
        let assign81960_e124601: f64 = (1.0 - assign81960_e124600);
        let assign81960_e124602: f64 = (assign81960_e124597 * assign81960_e124601);
        let assign81960_e124603: f64 = (locals.var_vgpld + assign81960_e124602);
        (assign81960_e124603, (((((locals.var_fac1p2_dn0 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn0)) / 2.0) * assign81960_e124601) + (assign81960_e124597 * (-(locals.var_tx_dn0 / (2.0 * assign81960_e124600))))), (locals.var_vgpld_dn2 + (((((locals.var_fac1p2_dn2 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn2)) / 2.0) * assign81960_e124601) + (assign81960_e124597 * (-(locals.var_tx_dn2 / (2.0 * assign81960_e124600)))))), (((((locals.var_fac1p2_dn4 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn4)) / 2.0) * assign81960_e124601) + (assign81960_e124597 * (-(locals.var_tx_dn4 / (2.0 * assign81960_e124600))))), (((((locals.var_fac1p2_dn5 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn5)) / 2.0) * assign81960_e124601) + (assign81960_e124597 * (-(locals.var_tx_dn5 / (2.0 * assign81960_e124600))))), (((((locals.var_fac1p2_dn6 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn6)) / 2.0) * assign81960_e124601) + (assign81960_e124597 * (-(locals.var_tx_dn6 / (2.0 * assign81960_e124600))))), (locals.var_vgpld_dn7 + (((((locals.var_fac1p2_dn7 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn7)) / 2.0) * assign81960_e124601) + (assign81960_e124597 * (-(locals.var_tx_dn7 / (2.0 * assign81960_e124600)))))), (locals.var_vgpld_dn8 + (((((locals.var_fac1p2_dn8 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn8)) / 2.0) * assign81960_e124601) + (assign81960_e124597 * (-(locals.var_tx_dn8 / (2.0 * assign81960_e124600)))))), (locals.var_vgpld_dn9 + (((((locals.var_fac1p2_dn9 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn9)) / 2.0) * assign81960_e124601) + (assign81960_e124597 * (-(locals.var_tx_dn9 / (2.0 * assign81960_e124600)))))), (((((locals.var_fac1p2_dn10 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn10)) / 2.0) * assign81960_e124601) + (assign81960_e124597 * (-(locals.var_tx_dn10 / (2.0 * assign81960_e124600))))), (((((locals.var_fac1p2_dn11 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn11)) / 2.0) * assign81960_e124601) + (assign81960_e124597 * (-(locals.var_tx_dn11 / (2.0 * assign81960_e124600))))), (((((locals.var_fac1p2_dn14 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn14)) / 2.0) * assign81960_e124601) + (assign81960_e124597 * (-(locals.var_tx_dn14 / (2.0 * assign81960_e124600))))),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn14,)
    }
};
        locals.var_ps0_inia = assign81960_e124605;
        locals.var_ps0_inia_dn0 = assign81960_e124605_d_n0;
        locals.var_ps0_inia_dn2 = assign81960_e124605_d_n2;
        locals.var_ps0_inia_dn4 = assign81960_e124605_d_n4;
        locals.var_ps0_inia_dn5 = assign81960_e124605_d_n5;
        locals.var_ps0_inia_dn6 = assign81960_e124605_d_n6;
        locals.var_ps0_inia_dn7 = assign81960_e124605_d_n7;
        locals.var_ps0_inia_dn8 = assign81960_e124605_d_n8;
        locals.var_ps0_inia_dn9 = assign81960_e124605_d_n9;
        locals.var_ps0_inia_dn10 = assign81960_e124605_d_n10;
        locals.var_ps0_inia_dn11 = assign81960_e124605_d_n11;
        locals.var_ps0_inia_dn14 = assign81960_e124605_d_n14;

        let (assign81970_e124616, assign81970_e124616_d_n0, assign81970_e124616_d_n2, assign81970_e124616_d_n4, assign81970_e124616_d_n5, assign81970_e124616_d_n6, assign81970_e124616_d_n7, assign81970_e124616_d_n8, assign81970_e124616_d_n9, assign81970_e124616_d_n10, assign81970_e124616_d_n11, assign81970_e124616_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) {
        let assign81970_e124613: f64 = (locals.var_ps0_inia + locals.var_vxbgmtcl);
        let assign81970_e124614: f64 = (locals.var_beta * assign81970_e124613);
        (assign81970_e124614, ((locals.var_beta_dn0 * assign81970_e124613) + (locals.var_beta * (locals.var_ps0_inia_dn0 + locals.var_vxbgmtcl_dn0))), ((locals.var_beta_dn2 * assign81970_e124613) + (locals.var_beta * (locals.var_ps0_inia_dn2 + locals.var_vxbgmtcl_dn2))), ((locals.var_beta_dn4 * assign81970_e124613) + (locals.var_beta * (locals.var_ps0_inia_dn4 + locals.var_vxbgmtcl_dn4))), ((locals.var_beta_dn5 * assign81970_e124613) + (locals.var_beta * (locals.var_ps0_inia_dn5 + locals.var_vxbgmtcl_dn5))), ((locals.var_beta_dn6 * assign81970_e124613) + (locals.var_beta * (locals.var_ps0_inia_dn6 + locals.var_vxbgmtcl_dn6))), ((locals.var_beta_dn7 * assign81970_e124613) + (locals.var_beta * (locals.var_ps0_inia_dn7 + locals.var_vxbgmtcl_dn7))), ((locals.var_beta_dn8 * assign81970_e124613) + (locals.var_beta * (locals.var_ps0_inia_dn8 + locals.var_vxbgmtcl_dn8))), ((locals.var_beta_dn9 * assign81970_e124613) + (locals.var_beta * (locals.var_ps0_inia_dn9 + locals.var_vxbgmtcl_dn9))), ((locals.var_beta_dn10 * assign81970_e124613) + (locals.var_beta * (locals.var_ps0_inia_dn10 + locals.var_vxbgmtcl_dn10))), ((locals.var_beta_dn11 * assign81970_e124613) + (locals.var_beta * (locals.var_ps0_inia_dn11 + locals.var_vxbgmtcl_dn11))), ((locals.var_beta_dn14 * assign81970_e124613) + (locals.var_beta * (locals.var_ps0_inia_dn14 + locals.var_vxbgmtcl_dn14))),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
        locals.var_chi = assign81970_e124616;
        locals.var_chi_dn0 = assign81970_e124616_d_n0;
        locals.var_chi_dn2 = assign81970_e124616_d_n2;
        locals.var_chi_dn4 = assign81970_e124616_d_n4;
        locals.var_chi_dn5 = assign81970_e124616_d_n5;
        locals.var_chi_dn6 = assign81970_e124616_d_n6;
        locals.var_chi_dn7 = assign81970_e124616_d_n7;
        locals.var_chi_dn8 = assign81970_e124616_d_n8;
        locals.var_chi_dn9 = assign81970_e124616_d_n9;
        locals.var_chi_dn10 = assign81970_e124616_d_n10;
        locals.var_chi_dn11 = assign81970_e124616_d_n11;
        locals.var_chi_dn14 = assign81970_e124616_d_n14;

        let assign81980_e124619: f64 = if locals.var_chi >= 3.0 { 1.0 } else { 0.0 };
        locals.var_guard1912 = assign81980_e124619;

        let (assign82000_e124639, assign82000_e124639_d_n0, assign82000_e124639_d_n2, assign82000_e124639_d_n4, assign82000_e124639_d_n5, assign82000_e124639_d_n6, assign82000_e124639_d_n7, assign82000_e124639_d_n8, assign82000_e124639_d_n9, assign82000_e124639_d_n10, assign82000_e124639_d_n11, assign82000_e124639_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1912 != 0.0)) {
        let assign82000_e124636: f64 = (-locals.var_chi);
        let assign82000_e124637: f64 = (assign82000_e124636).exp();
        (assign82000_e124637, (assign82000_e124637 * (-locals.var_chi_dn0)), (assign82000_e124637 * (-locals.var_chi_dn2)), (assign82000_e124637 * (-locals.var_chi_dn4)), (assign82000_e124637 * (-locals.var_chi_dn5)), (assign82000_e124637 * (-locals.var_chi_dn6)), (assign82000_e124637 * (-locals.var_chi_dn7)), (assign82000_e124637 * (-locals.var_chi_dn8)), (assign82000_e124637 * (-locals.var_chi_dn9)), (assign82000_e124637 * (-locals.var_chi_dn10)), (assign82000_e124637 * (-locals.var_chi_dn11)), (assign82000_e124637 * (-locals.var_chi_dn14)),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn14,)
    }
};
        locals.var_ty = assign82000_e124639;
        locals.var_ty_dn0 = assign82000_e124639_d_n0;
        locals.var_ty_dn2 = assign82000_e124639_d_n2;
        locals.var_ty_dn4 = assign82000_e124639_d_n4;
        locals.var_ty_dn5 = assign82000_e124639_d_n5;
        locals.var_ty_dn6 = assign82000_e124639_d_n6;
        locals.var_ty_dn7 = assign82000_e124639_d_n7;
        locals.var_ty_dn8 = assign82000_e124639_d_n8;
        locals.var_ty_dn9 = assign82000_e124639_d_n9;
        locals.var_ty_dn10 = assign82000_e124639_d_n10;
        locals.var_ty_dn11 = assign82000_e124639_d_n11;
        locals.var_ty_dn14 = assign82000_e124639_d_n14;

        let (assign82010_e124664, assign82010_e124664_d_n0, assign82010_e124664_d_n2, assign82010_e124664_d_n4, assign82010_e124664_d_n5, assign82010_e124664_d_n6, assign82010_e124664_d_n7, assign82010_e124664_d_n8, assign82010_e124664_d_n9, assign82010_e124664_d_n10, assign82010_e124664_d_n11, assign82010_e124664_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1912 != 0.0)) {
        let assign82010_e124651: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign82010_e124652: f64 = (locals.var_beta * assign82010_e124651);
        let assign82010_e124654: f64 = (assign82010_e124652 - 1.0);
        let assign82010_e124656: f64 = (assign82010_e124654 + locals.var_ty);
        let assign82010_e124657: f64 = (4.0 * assign82010_e124656);
        let assign82010_e124660: f64 = (locals.var_fac1p2 * locals.var_beta2);
        let assign82010_e124661: f64 = (assign82010_e124657 / assign82010_e124660);
        let assign82010_e124662: f64 = (1.0 + assign82010_e124661);
        (assign82010_e124662, ((((4.0 * (((locals.var_beta_dn0 * assign82010_e124651) + (locals.var_beta * locals.var_vxbgmtcl_dn0)) + locals.var_ty_dn0)) * assign82010_e124660) - (assign82010_e124657 * ((locals.var_fac1p2_dn0 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn0)))) / (assign82010_e124660 * assign82010_e124660)), ((((4.0 * (((locals.var_beta_dn2 * assign82010_e124651) + (locals.var_beta * (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2))) + locals.var_ty_dn2)) * assign82010_e124660) - (assign82010_e124657 * ((locals.var_fac1p2_dn2 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn2)))) / (assign82010_e124660 * assign82010_e124660)), ((((4.0 * (((locals.var_beta_dn4 * assign82010_e124651) + (locals.var_beta * locals.var_vxbgmtcl_dn4)) + locals.var_ty_dn4)) * assign82010_e124660) - (assign82010_e124657 * ((locals.var_fac1p2_dn4 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn4)))) / (assign82010_e124660 * assign82010_e124660)), ((((4.0 * (((locals.var_beta_dn5 * assign82010_e124651) + (locals.var_beta * locals.var_vxbgmtcl_dn5)) + locals.var_ty_dn5)) * assign82010_e124660) - (assign82010_e124657 * ((locals.var_fac1p2_dn5 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn5)))) / (assign82010_e124660 * assign82010_e124660)), ((((4.0 * (((locals.var_beta_dn6 * assign82010_e124651) + (locals.var_beta * locals.var_vxbgmtcl_dn6)) + locals.var_ty_dn6)) * assign82010_e124660) - (assign82010_e124657 * ((locals.var_fac1p2_dn6 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn6)))) / (assign82010_e124660 * assign82010_e124660)), ((((4.0 * (((locals.var_beta_dn7 * assign82010_e124651) + (locals.var_beta * (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7))) + locals.var_ty_dn7)) * assign82010_e124660) - (assign82010_e124657 * ((locals.var_fac1p2_dn7 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn7)))) / (assign82010_e124660 * assign82010_e124660)), ((((4.0 * (((locals.var_beta_dn8 * assign82010_e124651) + (locals.var_beta * (locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8))) + locals.var_ty_dn8)) * assign82010_e124660) - (assign82010_e124657 * ((locals.var_fac1p2_dn8 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn8)))) / (assign82010_e124660 * assign82010_e124660)), ((((4.0 * (((locals.var_beta_dn9 * assign82010_e124651) + (locals.var_beta * (locals.var_vgpld_dn9 + locals.var_vxbgmtcl_dn9))) + locals.var_ty_dn9)) * assign82010_e124660) - (assign82010_e124657 * ((locals.var_fac1p2_dn9 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn9)))) / (assign82010_e124660 * assign82010_e124660)), ((((4.0 * (((locals.var_beta_dn10 * assign82010_e124651) + (locals.var_beta * locals.var_vxbgmtcl_dn10)) + locals.var_ty_dn10)) * assign82010_e124660) - (assign82010_e124657 * ((locals.var_fac1p2_dn10 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn10)))) / (assign82010_e124660 * assign82010_e124660)), ((((4.0 * (((locals.var_beta_dn11 * assign82010_e124651) + (locals.var_beta * locals.var_vxbgmtcl_dn11)) + locals.var_ty_dn11)) * assign82010_e124660) - (assign82010_e124657 * ((locals.var_fac1p2_dn11 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn11)))) / (assign82010_e124660 * assign82010_e124660)), ((((4.0 * (((locals.var_beta_dn14 * assign82010_e124651) + (locals.var_beta * locals.var_vxbgmtcl_dn14)) + locals.var_ty_dn14)) * assign82010_e124660) - (assign82010_e124657 * ((locals.var_fac1p2_dn14 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn14)))) / (assign82010_e124660 * assign82010_e124660)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign82010_e124664;
        locals.var_tx_dn0 = assign82010_e124664_d_n0;
        locals.var_tx_dn2 = assign82010_e124664_d_n2;
        locals.var_tx_dn4 = assign82010_e124664_d_n4;
        locals.var_tx_dn5 = assign82010_e124664_d_n5;
        locals.var_tx_dn6 = assign82010_e124664_d_n6;
        locals.var_tx_dn7 = assign82010_e124664_d_n7;
        locals.var_tx_dn8 = assign82010_e124664_d_n8;
        locals.var_tx_dn9 = assign82010_e124664_d_n9;
        locals.var_tx_dn10 = assign82010_e124664_d_n10;
        locals.var_tx_dn11 = assign82010_e124664_d_n11;
        locals.var_tx_dn14 = assign82010_e124664_d_n14;

        let (assign82020_e124684, assign82020_e124684_d_n0, assign82020_e124684_d_n2, assign82020_e124684_d_n4, assign82020_e124684_d_n5, assign82020_e124684_d_n6, assign82020_e124684_d_n7, assign82020_e124684_d_n8, assign82020_e124684_d_n9, assign82020_e124684_d_n10, assign82020_e124684_d_n11, assign82020_e124684_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1912 != 0.0)) {
        let assign82020_e124674: f64 = (locals.var_fac1p2 * locals.var_beta);
        let assign82020_e124676: f64 = (assign82020_e124674 / 2.0);
        let assign82020_e124679: f64 = (locals.var_tx).sqrt();
        let assign82020_e124680: f64 = (1.0 - assign82020_e124679);
        let assign82020_e124681: f64 = (assign82020_e124676 * assign82020_e124680);
        let assign82020_e124682: f64 = (locals.var_vgpld + assign82020_e124681);
        (assign82020_e124682, (((((locals.var_fac1p2_dn0 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn0)) / 2.0) * assign82020_e124680) + (assign82020_e124676 * (-(locals.var_tx_dn0 / (2.0 * assign82020_e124679))))), (locals.var_vgpld_dn2 + (((((locals.var_fac1p2_dn2 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn2)) / 2.0) * assign82020_e124680) + (assign82020_e124676 * (-(locals.var_tx_dn2 / (2.0 * assign82020_e124679)))))), (((((locals.var_fac1p2_dn4 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn4)) / 2.0) * assign82020_e124680) + (assign82020_e124676 * (-(locals.var_tx_dn4 / (2.0 * assign82020_e124679))))), (((((locals.var_fac1p2_dn5 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn5)) / 2.0) * assign82020_e124680) + (assign82020_e124676 * (-(locals.var_tx_dn5 / (2.0 * assign82020_e124679))))), (((((locals.var_fac1p2_dn6 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn6)) / 2.0) * assign82020_e124680) + (assign82020_e124676 * (-(locals.var_tx_dn6 / (2.0 * assign82020_e124679))))), (locals.var_vgpld_dn7 + (((((locals.var_fac1p2_dn7 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn7)) / 2.0) * assign82020_e124680) + (assign82020_e124676 * (-(locals.var_tx_dn7 / (2.0 * assign82020_e124679)))))), (locals.var_vgpld_dn8 + (((((locals.var_fac1p2_dn8 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn8)) / 2.0) * assign82020_e124680) + (assign82020_e124676 * (-(locals.var_tx_dn8 / (2.0 * assign82020_e124679)))))), (locals.var_vgpld_dn9 + (((((locals.var_fac1p2_dn9 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn9)) / 2.0) * assign82020_e124680) + (assign82020_e124676 * (-(locals.var_tx_dn9 / (2.0 * assign82020_e124679)))))), (((((locals.var_fac1p2_dn10 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn10)) / 2.0) * assign82020_e124680) + (assign82020_e124676 * (-(locals.var_tx_dn10 / (2.0 * assign82020_e124679))))), (((((locals.var_fac1p2_dn11 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn11)) / 2.0) * assign82020_e124680) + (assign82020_e124676 * (-(locals.var_tx_dn11 / (2.0 * assign82020_e124679))))), (((((locals.var_fac1p2_dn14 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn14)) / 2.0) * assign82020_e124680) + (assign82020_e124676 * (-(locals.var_tx_dn14 / (2.0 * assign82020_e124679))))),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn14,)
    }
};
        locals.var_ps0_inia = assign82020_e124684;
        locals.var_ps0_inia_dn0 = assign82020_e124684_d_n0;
        locals.var_ps0_inia_dn2 = assign82020_e124684_d_n2;
        locals.var_ps0_inia_dn4 = assign82020_e124684_d_n4;
        locals.var_ps0_inia_dn5 = assign82020_e124684_d_n5;
        locals.var_ps0_inia_dn6 = assign82020_e124684_d_n6;
        locals.var_ps0_inia_dn7 = assign82020_e124684_d_n7;
        locals.var_ps0_inia_dn8 = assign82020_e124684_d_n8;
        locals.var_ps0_inia_dn9 = assign82020_e124684_d_n9;
        locals.var_ps0_inia_dn10 = assign82020_e124684_d_n10;
        locals.var_ps0_inia_dn11 = assign82020_e124684_d_n11;
        locals.var_ps0_inia_dn14 = assign82020_e124684_d_n14;

        let (assign82030_e124697, assign82030_e124697_d_n0, assign82030_e124697_d_n2, assign82030_e124697_d_n4, assign82030_e124697_d_n5, assign82030_e124697_d_n6, assign82030_e124697_d_n7, assign82030_e124697_d_n8, assign82030_e124697_d_n9, assign82030_e124697_d_n10, assign82030_e124697_d_n11, assign82030_e124697_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1912 != 0.0)) {
        let assign82030_e124694: f64 = (locals.var_ps0_inia + locals.var_vxbgmtcl);
        let assign82030_e124695: f64 = (locals.var_beta * assign82030_e124694);
        (assign82030_e124695, ((locals.var_beta_dn0 * assign82030_e124694) + (locals.var_beta * (locals.var_ps0_inia_dn0 + locals.var_vxbgmtcl_dn0))), ((locals.var_beta_dn2 * assign82030_e124694) + (locals.var_beta * (locals.var_ps0_inia_dn2 + locals.var_vxbgmtcl_dn2))), ((locals.var_beta_dn4 * assign82030_e124694) + (locals.var_beta * (locals.var_ps0_inia_dn4 + locals.var_vxbgmtcl_dn4))), ((locals.var_beta_dn5 * assign82030_e124694) + (locals.var_beta * (locals.var_ps0_inia_dn5 + locals.var_vxbgmtcl_dn5))), ((locals.var_beta_dn6 * assign82030_e124694) + (locals.var_beta * (locals.var_ps0_inia_dn6 + locals.var_vxbgmtcl_dn6))), ((locals.var_beta_dn7 * assign82030_e124694) + (locals.var_beta * (locals.var_ps0_inia_dn7 + locals.var_vxbgmtcl_dn7))), ((locals.var_beta_dn8 * assign82030_e124694) + (locals.var_beta * (locals.var_ps0_inia_dn8 + locals.var_vxbgmtcl_dn8))), ((locals.var_beta_dn9 * assign82030_e124694) + (locals.var_beta * (locals.var_ps0_inia_dn9 + locals.var_vxbgmtcl_dn9))), ((locals.var_beta_dn10 * assign82030_e124694) + (locals.var_beta * (locals.var_ps0_inia_dn10 + locals.var_vxbgmtcl_dn10))), ((locals.var_beta_dn11 * assign82030_e124694) + (locals.var_beta * (locals.var_ps0_inia_dn11 + locals.var_vxbgmtcl_dn11))), ((locals.var_beta_dn14 * assign82030_e124694) + (locals.var_beta * (locals.var_ps0_inia_dn14 + locals.var_vxbgmtcl_dn14))),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
        locals.var_chi = assign82030_e124697;
        locals.var_chi_dn0 = assign82030_e124697_d_n0;
        locals.var_chi_dn2 = assign82030_e124697_d_n2;
        locals.var_chi_dn4 = assign82030_e124697_d_n4;
        locals.var_chi_dn5 = assign82030_e124697_d_n5;
        locals.var_chi_dn6 = assign82030_e124697_d_n6;
        locals.var_chi_dn7 = assign82030_e124697_d_n7;
        locals.var_chi_dn8 = assign82030_e124697_d_n8;
        locals.var_chi_dn9 = assign82030_e124697_d_n9;
        locals.var_chi_dn10 = assign82030_e124697_d_n10;
        locals.var_chi_dn11 = assign82030_e124697_d_n11;
        locals.var_chi_dn14 = assign82030_e124697_d_n14;

        let (assign82040_e124708, assign82040_e124708_d_n0, assign82040_e124708_d_n2, assign82040_e124708_d_n4, assign82040_e124708_d_n5, assign82040_e124708_d_n6, assign82040_e124708_d_n7, assign82040_e124708_d_n8, assign82040_e124708_d_n9, assign82040_e124708_d_n10, assign82040_e124708_d_n11, assign82040_e124708_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1912 != 0.0)) {
        let assign82040_e124705: f64 = (-locals.var_chi);
        let assign82040_e124706: f64 = (assign82040_e124705).exp();
        (assign82040_e124706, (assign82040_e124706 * (-locals.var_chi_dn0)), (assign82040_e124706 * (-locals.var_chi_dn2)), (assign82040_e124706 * (-locals.var_chi_dn4)), (assign82040_e124706 * (-locals.var_chi_dn5)), (assign82040_e124706 * (-locals.var_chi_dn6)), (assign82040_e124706 * (-locals.var_chi_dn7)), (assign82040_e124706 * (-locals.var_chi_dn8)), (assign82040_e124706 * (-locals.var_chi_dn9)), (assign82040_e124706 * (-locals.var_chi_dn10)), (assign82040_e124706 * (-locals.var_chi_dn11)), (assign82040_e124706 * (-locals.var_chi_dn14)),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn14,)
    }
};
        locals.var_ty = assign82040_e124708;
        locals.var_ty_dn0 = assign82040_e124708_d_n0;
        locals.var_ty_dn2 = assign82040_e124708_d_n2;
        locals.var_ty_dn4 = assign82040_e124708_d_n4;
        locals.var_ty_dn5 = assign82040_e124708_d_n5;
        locals.var_ty_dn6 = assign82040_e124708_d_n6;
        locals.var_ty_dn7 = assign82040_e124708_d_n7;
        locals.var_ty_dn8 = assign82040_e124708_d_n8;
        locals.var_ty_dn9 = assign82040_e124708_d_n9;
        locals.var_ty_dn10 = assign82040_e124708_d_n10;
        locals.var_ty_dn11 = assign82040_e124708_d_n11;
        locals.var_ty_dn14 = assign82040_e124708_d_n14;

        let (assign82050_e124733, assign82050_e124733_d_n0, assign82050_e124733_d_n2, assign82050_e124733_d_n4, assign82050_e124733_d_n5, assign82050_e124733_d_n6, assign82050_e124733_d_n7, assign82050_e124733_d_n8, assign82050_e124733_d_n9, assign82050_e124733_d_n10, assign82050_e124733_d_n11, assign82050_e124733_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1912 != 0.0)) {
        let assign82050_e124720: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign82050_e124721: f64 = (locals.var_beta * assign82050_e124720);
        let assign82050_e124723: f64 = (assign82050_e124721 - 1.0);
        let assign82050_e124725: f64 = (assign82050_e124723 + locals.var_ty);
        let assign82050_e124726: f64 = (4.0 * assign82050_e124725);
        let assign82050_e124729: f64 = (locals.var_fac1p2 * locals.var_beta2);
        let assign82050_e124730: f64 = (assign82050_e124726 / assign82050_e124729);
        let assign82050_e124731: f64 = (1.0 + assign82050_e124730);
        (assign82050_e124731, ((((4.0 * (((locals.var_beta_dn0 * assign82050_e124720) + (locals.var_beta * locals.var_vxbgmtcl_dn0)) + locals.var_ty_dn0)) * assign82050_e124729) - (assign82050_e124726 * ((locals.var_fac1p2_dn0 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn0)))) / (assign82050_e124729 * assign82050_e124729)), ((((4.0 * (((locals.var_beta_dn2 * assign82050_e124720) + (locals.var_beta * (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2))) + locals.var_ty_dn2)) * assign82050_e124729) - (assign82050_e124726 * ((locals.var_fac1p2_dn2 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn2)))) / (assign82050_e124729 * assign82050_e124729)), ((((4.0 * (((locals.var_beta_dn4 * assign82050_e124720) + (locals.var_beta * locals.var_vxbgmtcl_dn4)) + locals.var_ty_dn4)) * assign82050_e124729) - (assign82050_e124726 * ((locals.var_fac1p2_dn4 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn4)))) / (assign82050_e124729 * assign82050_e124729)), ((((4.0 * (((locals.var_beta_dn5 * assign82050_e124720) + (locals.var_beta * locals.var_vxbgmtcl_dn5)) + locals.var_ty_dn5)) * assign82050_e124729) - (assign82050_e124726 * ((locals.var_fac1p2_dn5 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn5)))) / (assign82050_e124729 * assign82050_e124729)), ((((4.0 * (((locals.var_beta_dn6 * assign82050_e124720) + (locals.var_beta * locals.var_vxbgmtcl_dn6)) + locals.var_ty_dn6)) * assign82050_e124729) - (assign82050_e124726 * ((locals.var_fac1p2_dn6 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn6)))) / (assign82050_e124729 * assign82050_e124729)), ((((4.0 * (((locals.var_beta_dn7 * assign82050_e124720) + (locals.var_beta * (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7))) + locals.var_ty_dn7)) * assign82050_e124729) - (assign82050_e124726 * ((locals.var_fac1p2_dn7 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn7)))) / (assign82050_e124729 * assign82050_e124729)), ((((4.0 * (((locals.var_beta_dn8 * assign82050_e124720) + (locals.var_beta * (locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8))) + locals.var_ty_dn8)) * assign82050_e124729) - (assign82050_e124726 * ((locals.var_fac1p2_dn8 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn8)))) / (assign82050_e124729 * assign82050_e124729)), ((((4.0 * (((locals.var_beta_dn9 * assign82050_e124720) + (locals.var_beta * (locals.var_vgpld_dn9 + locals.var_vxbgmtcl_dn9))) + locals.var_ty_dn9)) * assign82050_e124729) - (assign82050_e124726 * ((locals.var_fac1p2_dn9 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn9)))) / (assign82050_e124729 * assign82050_e124729)), ((((4.0 * (((locals.var_beta_dn10 * assign82050_e124720) + (locals.var_beta * locals.var_vxbgmtcl_dn10)) + locals.var_ty_dn10)) * assign82050_e124729) - (assign82050_e124726 * ((locals.var_fac1p2_dn10 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn10)))) / (assign82050_e124729 * assign82050_e124729)), ((((4.0 * (((locals.var_beta_dn11 * assign82050_e124720) + (locals.var_beta * locals.var_vxbgmtcl_dn11)) + locals.var_ty_dn11)) * assign82050_e124729) - (assign82050_e124726 * ((locals.var_fac1p2_dn11 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn11)))) / (assign82050_e124729 * assign82050_e124729)), ((((4.0 * (((locals.var_beta_dn14 * assign82050_e124720) + (locals.var_beta * locals.var_vxbgmtcl_dn14)) + locals.var_ty_dn14)) * assign82050_e124729) - (assign82050_e124726 * ((locals.var_fac1p2_dn14 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn14)))) / (assign82050_e124729 * assign82050_e124729)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign82050_e124733;
        locals.var_tx_dn0 = assign82050_e124733_d_n0;
        locals.var_tx_dn2 = assign82050_e124733_d_n2;
        locals.var_tx_dn4 = assign82050_e124733_d_n4;
        locals.var_tx_dn5 = assign82050_e124733_d_n5;
        locals.var_tx_dn6 = assign82050_e124733_d_n6;
        locals.var_tx_dn7 = assign82050_e124733_d_n7;
        locals.var_tx_dn8 = assign82050_e124733_d_n8;
        locals.var_tx_dn9 = assign82050_e124733_d_n9;
        locals.var_tx_dn10 = assign82050_e124733_d_n10;
        locals.var_tx_dn11 = assign82050_e124733_d_n11;
        locals.var_tx_dn14 = assign82050_e124733_d_n14;

        let (assign82060_e124753, assign82060_e124753_d_n0, assign82060_e124753_d_n2, assign82060_e124753_d_n4, assign82060_e124753_d_n5, assign82060_e124753_d_n6, assign82060_e124753_d_n7, assign82060_e124753_d_n8, assign82060_e124753_d_n9, assign82060_e124753_d_n10, assign82060_e124753_d_n11, assign82060_e124753_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1912 != 0.0)) {
        let assign82060_e124743: f64 = (locals.var_fac1p2 * locals.var_beta);
        let assign82060_e124745: f64 = (assign82060_e124743 / 2.0);
        let assign82060_e124748: f64 = (locals.var_tx).sqrt();
        let assign82060_e124749: f64 = (1.0 - assign82060_e124748);
        let assign82060_e124750: f64 = (assign82060_e124745 * assign82060_e124749);
        let assign82060_e124751: f64 = (locals.var_vgpld + assign82060_e124750);
        (assign82060_e124751, (((((locals.var_fac1p2_dn0 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn0)) / 2.0) * assign82060_e124749) + (assign82060_e124745 * (-(locals.var_tx_dn0 / (2.0 * assign82060_e124748))))), (locals.var_vgpld_dn2 + (((((locals.var_fac1p2_dn2 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn2)) / 2.0) * assign82060_e124749) + (assign82060_e124745 * (-(locals.var_tx_dn2 / (2.0 * assign82060_e124748)))))), (((((locals.var_fac1p2_dn4 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn4)) / 2.0) * assign82060_e124749) + (assign82060_e124745 * (-(locals.var_tx_dn4 / (2.0 * assign82060_e124748))))), (((((locals.var_fac1p2_dn5 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn5)) / 2.0) * assign82060_e124749) + (assign82060_e124745 * (-(locals.var_tx_dn5 / (2.0 * assign82060_e124748))))), (((((locals.var_fac1p2_dn6 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn6)) / 2.0) * assign82060_e124749) + (assign82060_e124745 * (-(locals.var_tx_dn6 / (2.0 * assign82060_e124748))))), (locals.var_vgpld_dn7 + (((((locals.var_fac1p2_dn7 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn7)) / 2.0) * assign82060_e124749) + (assign82060_e124745 * (-(locals.var_tx_dn7 / (2.0 * assign82060_e124748)))))), (locals.var_vgpld_dn8 + (((((locals.var_fac1p2_dn8 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn8)) / 2.0) * assign82060_e124749) + (assign82060_e124745 * (-(locals.var_tx_dn8 / (2.0 * assign82060_e124748)))))), (locals.var_vgpld_dn9 + (((((locals.var_fac1p2_dn9 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn9)) / 2.0) * assign82060_e124749) + (assign82060_e124745 * (-(locals.var_tx_dn9 / (2.0 * assign82060_e124748)))))), (((((locals.var_fac1p2_dn10 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn10)) / 2.0) * assign82060_e124749) + (assign82060_e124745 * (-(locals.var_tx_dn10 / (2.0 * assign82060_e124748))))), (((((locals.var_fac1p2_dn11 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn11)) / 2.0) * assign82060_e124749) + (assign82060_e124745 * (-(locals.var_tx_dn11 / (2.0 * assign82060_e124748))))), (((((locals.var_fac1p2_dn14 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn14)) / 2.0) * assign82060_e124749) + (assign82060_e124745 * (-(locals.var_tx_dn14 / (2.0 * assign82060_e124748))))),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn14,)
    }
};
        locals.var_ps0_inia = assign82060_e124753;
        locals.var_ps0_inia_dn0 = assign82060_e124753_d_n0;
        locals.var_ps0_inia_dn2 = assign82060_e124753_d_n2;
        locals.var_ps0_inia_dn4 = assign82060_e124753_d_n4;
        locals.var_ps0_inia_dn5 = assign82060_e124753_d_n5;
        locals.var_ps0_inia_dn6 = assign82060_e124753_d_n6;
        locals.var_ps0_inia_dn7 = assign82060_e124753_d_n7;
        locals.var_ps0_inia_dn8 = assign82060_e124753_d_n8;
        locals.var_ps0_inia_dn9 = assign82060_e124753_d_n9;
        locals.var_ps0_inia_dn10 = assign82060_e124753_d_n10;
        locals.var_ps0_inia_dn11 = assign82060_e124753_d_n11;
        locals.var_ps0_inia_dn14 = assign82060_e124753_d_n14;

    }

    pub(super) fn stamp_transient_block_297(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign82070_e124766, assign82070_e124766_d_n0, assign82070_e124766_d_n2, assign82070_e124766_d_n4, assign82070_e124766_d_n5, assign82070_e124766_d_n6, assign82070_e124766_d_n7, assign82070_e124766_d_n8, assign82070_e124766_d_n9, assign82070_e124766_d_n10, assign82070_e124766_d_n11, assign82070_e124766_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1912 != 0.0)) {
        let assign82070_e124763: f64 = (locals.var_ps0_inia + locals.var_vxbgmtcl);
        let assign82070_e124764: f64 = (locals.var_beta * assign82070_e124763);
        (assign82070_e124764, ((locals.var_beta_dn0 * assign82070_e124763) + (locals.var_beta * (locals.var_ps0_inia_dn0 + locals.var_vxbgmtcl_dn0))), ((locals.var_beta_dn2 * assign82070_e124763) + (locals.var_beta * (locals.var_ps0_inia_dn2 + locals.var_vxbgmtcl_dn2))), ((locals.var_beta_dn4 * assign82070_e124763) + (locals.var_beta * (locals.var_ps0_inia_dn4 + locals.var_vxbgmtcl_dn4))), ((locals.var_beta_dn5 * assign82070_e124763) + (locals.var_beta * (locals.var_ps0_inia_dn5 + locals.var_vxbgmtcl_dn5))), ((locals.var_beta_dn6 * assign82070_e124763) + (locals.var_beta * (locals.var_ps0_inia_dn6 + locals.var_vxbgmtcl_dn6))), ((locals.var_beta_dn7 * assign82070_e124763) + (locals.var_beta * (locals.var_ps0_inia_dn7 + locals.var_vxbgmtcl_dn7))), ((locals.var_beta_dn8 * assign82070_e124763) + (locals.var_beta * (locals.var_ps0_inia_dn8 + locals.var_vxbgmtcl_dn8))), ((locals.var_beta_dn9 * assign82070_e124763) + (locals.var_beta * (locals.var_ps0_inia_dn9 + locals.var_vxbgmtcl_dn9))), ((locals.var_beta_dn10 * assign82070_e124763) + (locals.var_beta * (locals.var_ps0_inia_dn10 + locals.var_vxbgmtcl_dn10))), ((locals.var_beta_dn11 * assign82070_e124763) + (locals.var_beta * (locals.var_ps0_inia_dn11 + locals.var_vxbgmtcl_dn11))), ((locals.var_beta_dn14 * assign82070_e124763) + (locals.var_beta * (locals.var_ps0_inia_dn14 + locals.var_vxbgmtcl_dn14))),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
        locals.var_chi = assign82070_e124766;
        locals.var_chi_dn0 = assign82070_e124766_d_n0;
        locals.var_chi_dn2 = assign82070_e124766_d_n2;
        locals.var_chi_dn4 = assign82070_e124766_d_n4;
        locals.var_chi_dn5 = assign82070_e124766_d_n5;
        locals.var_chi_dn6 = assign82070_e124766_d_n6;
        locals.var_chi_dn7 = assign82070_e124766_d_n7;
        locals.var_chi_dn8 = assign82070_e124766_d_n8;
        locals.var_chi_dn9 = assign82070_e124766_d_n9;
        locals.var_chi_dn10 = assign82070_e124766_d_n10;
        locals.var_chi_dn11 = assign82070_e124766_d_n11;
        locals.var_chi_dn14 = assign82070_e124766_d_n14;

        let (assign82090_e124808,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1912 == 0.0)) {
        let assign82090_e124787: f64 = (2.0_f64).sqrt();
        let assign82090_e124788: f64 = (9.0 * assign82090_e124787);
        let assign82090_e124789: f64 = (1.0 / assign82090_e124788);
        let assign82090_e124793: f64 = (-3.0);
        let assign82090_e124794: f64 = (assign82090_e124793).exp();
        let assign82090_e124795: f64 = (7.0 * assign82090_e124794);
        let assign82090_e124796: f64 = (5.0 + assign82090_e124795);
        let assign82090_e124800: f64 = (-3.0);
        let assign82090_e124801: f64 = (assign82090_e124800).exp();
        let assign82090_e124802: f64 = (2.0 + assign82090_e124801);
        let assign82090_e124803: f64 = (assign82090_e124802).sqrt();
        let assign82090_e124804: f64 = (54.0 * assign82090_e124803);
        let assign82090_e124805: f64 = (assign82090_e124796 / assign82090_e124804);
        let assign82090_e124806: f64 = (assign82090_e124789 - assign82090_e124805);
        (assign82090_e124806,)
    } else {
        (locals.var_ta,)
    }
};
        locals.var_ta = assign82090_e124808;

        let (assign82100_e124836,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1912 == 0.0)) {
        let assign82100_e124818: f64 = (-3.0);
        let assign82100_e124819: f64 = (assign82100_e124818).exp();
        let assign82100_e124820: f64 = (1.0 + assign82100_e124819);
        let assign82100_e124824: f64 = (-3.0);
        let assign82100_e124825: f64 = (assign82100_e124824).exp();
        let assign82100_e124826: f64 = (2.0 + assign82100_e124825);
        let assign82100_e124827: f64 = (assign82100_e124826).sqrt();
        let assign82100_e124828: f64 = (2.0 * assign82100_e124827);
        let assign82100_e124829: f64 = (assign82100_e124820 / assign82100_e124828);
        let assign82100_e124831: f64 = (2.0_f64).sqrt();
        let assign82100_e124833: f64 = (assign82100_e124831 / 3.0);
        let assign82100_e124834: f64 = (assign82100_e124829 - assign82100_e124833);
        (assign82100_e124834,)
    } else {
        (locals.var_tb,)
    }
};
        locals.var_tb = assign82100_e124836;

        let (assign82110_e124855, assign82110_e124855_d_n0, assign82110_e124855_d_n2, assign82110_e124855_d_n4, assign82110_e124855_d_n5, assign82110_e124855_d_n6, assign82110_e124855_d_n7, assign82110_e124855_d_n8, assign82110_e124855_d_n9, assign82110_e124855_d_n10, assign82110_e124855_d_n11, assign82110_e124855_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1912 == 0.0)) {
        let assign82110_e124846: f64 = (2.0_f64).sqrt();
        let assign82110_e124847: f64 = (1.0 / assign82110_e124846);
        let assign82110_e124851: f64 = (locals.var_beta * locals.var_fac1);
        let assign82110_e124852: f64 = (1.0 / assign82110_e124851);
        let assign82110_e124853: f64 = (assign82110_e124847 + assign82110_e124852);
        (assign82110_e124853, (-(((locals.var_beta_dn0 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn0)) / (assign82110_e124851 * assign82110_e124851))), (-(((locals.var_beta_dn2 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn2)) / (assign82110_e124851 * assign82110_e124851))), (-(((locals.var_beta_dn4 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn4)) / (assign82110_e124851 * assign82110_e124851))), (-(((locals.var_beta_dn5 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn5)) / (assign82110_e124851 * assign82110_e124851))), (-(((locals.var_beta_dn6 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn6)) / (assign82110_e124851 * assign82110_e124851))), (-(((locals.var_beta_dn7 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn7)) / (assign82110_e124851 * assign82110_e124851))), (-(((locals.var_beta_dn8 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn8)) / (assign82110_e124851 * assign82110_e124851))), (-(((locals.var_beta_dn9 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn9)) / (assign82110_e124851 * assign82110_e124851))), (-(((locals.var_beta_dn10 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn10)) / (assign82110_e124851 * assign82110_e124851))), (-(((locals.var_beta_dn11 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn11)) / (assign82110_e124851 * assign82110_e124851))), (-(((locals.var_beta_dn14 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn14)) / (assign82110_e124851 * assign82110_e124851))),)
    } else {
        (locals.var_tc, locals.var_tc_dn0, locals.var_tc_dn2, locals.var_tc_dn4, locals.var_tc_dn5, locals.var_tc_dn6, locals.var_tc_dn7, locals.var_tc_dn8, locals.var_tc_dn9, locals.var_tc_dn10, locals.var_tc_dn11, locals.var_tc_dn14,)
    }
};
        locals.var_tc = assign82110_e124855;
        locals.var_tc_dn0 = assign82110_e124855_d_n0;
        locals.var_tc_dn2 = assign82110_e124855_d_n2;
        locals.var_tc_dn4 = assign82110_e124855_d_n4;
        locals.var_tc_dn5 = assign82110_e124855_d_n5;
        locals.var_tc_dn6 = assign82110_e124855_d_n6;
        locals.var_tc_dn7 = assign82110_e124855_d_n7;
        locals.var_tc_dn8 = assign82110_e124855_d_n8;
        locals.var_tc_dn9 = assign82110_e124855_d_n9;
        locals.var_tc_dn10 = assign82110_e124855_d_n10;
        locals.var_tc_dn11 = assign82110_e124855_d_n11;
        locals.var_tc_dn14 = assign82110_e124855_d_n14;

        let (assign82120_e124870, assign82120_e124870_d_n0, assign82120_e124870_d_n2, assign82120_e124870_d_n4, assign82120_e124870_d_n5, assign82120_e124870_d_n6, assign82120_e124870_d_n7, assign82120_e124870_d_n8, assign82120_e124870_d_n9, assign82120_e124870_d_n10, assign82120_e124870_d_n11, assign82120_e124870_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1912 == 0.0)) {
        let assign82120_e124865: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign82120_e124866: f64 = (-assign82120_e124865);
        let assign82120_e124868: f64 = (assign82120_e124866 / locals.var_fac1);
        (assign82120_e124868, ((((-locals.var_vxbgmtcl_dn0) * locals.var_fac1) - (assign82120_e124866 * locals.var_fac1_dn0)) / (locals.var_fac1 * locals.var_fac1)), ((((-(locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2)) * locals.var_fac1) - (assign82120_e124866 * locals.var_fac1_dn2)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn4) * locals.var_fac1) - (assign82120_e124866 * locals.var_fac1_dn4)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn5) * locals.var_fac1) - (assign82120_e124866 * locals.var_fac1_dn5)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn6) * locals.var_fac1) - (assign82120_e124866 * locals.var_fac1_dn6)) / (locals.var_fac1 * locals.var_fac1)), ((((-(locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7)) * locals.var_fac1) - (assign82120_e124866 * locals.var_fac1_dn7)) / (locals.var_fac1 * locals.var_fac1)), ((((-(locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8)) * locals.var_fac1) - (assign82120_e124866 * locals.var_fac1_dn8)) / (locals.var_fac1 * locals.var_fac1)), ((((-(locals.var_vgpld_dn9 + locals.var_vxbgmtcl_dn9)) * locals.var_fac1) - (assign82120_e124866 * locals.var_fac1_dn9)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn10) * locals.var_fac1) - (assign82120_e124866 * locals.var_fac1_dn10)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn11) * locals.var_fac1) - (assign82120_e124866 * locals.var_fac1_dn11)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn14) * locals.var_fac1) - (assign82120_e124866 * locals.var_fac1_dn14)) / (locals.var_fac1 * locals.var_fac1)),)
    } else {
        (locals.var_td, locals.var_td_dn0, locals.var_td_dn2, locals.var_td_dn4, locals.var_td_dn5, locals.var_td_dn6, locals.var_td_dn7, locals.var_td_dn8, locals.var_td_dn9, locals.var_td_dn10, locals.var_td_dn11, locals.var_td_dn14,)
    }
};
        locals.var_td = assign82120_e124870;
        locals.var_td_dn0 = assign82120_e124870_d_n0;
        locals.var_td_dn2 = assign82120_e124870_d_n2;
        locals.var_td_dn4 = assign82120_e124870_d_n4;
        locals.var_td_dn5 = assign82120_e124870_d_n5;
        locals.var_td_dn6 = assign82120_e124870_d_n6;
        locals.var_td_dn7 = assign82120_e124870_d_n7;
        locals.var_td_dn8 = assign82120_e124870_d_n8;
        locals.var_td_dn9 = assign82120_e124870_d_n9;
        locals.var_td_dn10 = assign82120_e124870_d_n10;
        locals.var_td_dn11 = assign82120_e124870_d_n11;
        locals.var_td_dn14 = assign82120_e124870_d_n14;

        let (assign82130_e124908, assign82130_e124908_d_n0, assign82130_e124908_d_n2, assign82130_e124908_d_n4, assign82130_e124908_d_n5, assign82130_e124908_d_n6, assign82130_e124908_d_n7, assign82130_e124908_d_n8, assign82130_e124908_d_n9, assign82130_e124908_d_n10, assign82130_e124908_d_n11, assign82130_e124908_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1912 == 0.0)) {
        let assign82130_e124880: f64 = (locals.var_tb * locals.var_tb);
        let assign82130_e124882: f64 = (assign82130_e124880 * locals.var_tb);
        let assign82130_e124885: f64 = (27.0 * locals.var_ta);
        let assign82130_e124887: f64 = (assign82130_e124885 * locals.var_ta);
        let assign82130_e124889: f64 = (assign82130_e124887 * locals.var_ta);
        let assign82130_e124890: f64 = (assign82130_e124882 / assign82130_e124889);
        let assign82130_e124893: f64 = (locals.var_tb * locals.var_tc);
        let assign82130_e124896: f64 = (6.0 * locals.var_ta);
        let assign82130_e124898: f64 = (assign82130_e124896 * locals.var_ta);
        let assign82130_e124899: f64 = (assign82130_e124893 / assign82130_e124898);
        let assign82130_e124900: f64 = (assign82130_e124890 - assign82130_e124899);
        let assign82130_e124904: f64 = (2.0 * locals.var_ta);
        let assign82130_e124905: f64 = (locals.var_td / assign82130_e124904);
        let assign82130_e124906: f64 = (assign82130_e124900 + assign82130_e124905);
        (assign82130_e124906, ((-((locals.var_tb * locals.var_tc_dn0) / assign82130_e124898)) + (locals.var_td_dn0 / assign82130_e124904)), ((-((locals.var_tb * locals.var_tc_dn2) / assign82130_e124898)) + (locals.var_td_dn2 / assign82130_e124904)), ((-((locals.var_tb * locals.var_tc_dn4) / assign82130_e124898)) + (locals.var_td_dn4 / assign82130_e124904)), ((-((locals.var_tb * locals.var_tc_dn5) / assign82130_e124898)) + (locals.var_td_dn5 / assign82130_e124904)), ((-((locals.var_tb * locals.var_tc_dn6) / assign82130_e124898)) + (locals.var_td_dn6 / assign82130_e124904)), ((-((locals.var_tb * locals.var_tc_dn7) / assign82130_e124898)) + (locals.var_td_dn7 / assign82130_e124904)), ((-((locals.var_tb * locals.var_tc_dn8) / assign82130_e124898)) + (locals.var_td_dn8 / assign82130_e124904)), ((-((locals.var_tb * locals.var_tc_dn9) / assign82130_e124898)) + (locals.var_td_dn9 / assign82130_e124904)), ((-((locals.var_tb * locals.var_tc_dn10) / assign82130_e124898)) + (locals.var_td_dn10 / assign82130_e124904)), ((-((locals.var_tb * locals.var_tc_dn11) / assign82130_e124898)) + (locals.var_td_dn11 / assign82130_e124904)), ((-((locals.var_tb * locals.var_tc_dn14) / assign82130_e124898)) + (locals.var_td_dn14 / assign82130_e124904)),)
    } else {
        (locals.var_tq, locals.var_tq_dn0, locals.var_tq_dn2, locals.var_tq_dn4, locals.var_tq_dn5, locals.var_tq_dn6, locals.var_tq_dn7, locals.var_tq_dn8, locals.var_tq_dn9, locals.var_tq_dn10, locals.var_tq_dn11, locals.var_tq_dn14,)
    }
};
        locals.var_tq = assign82130_e124908;
        locals.var_tq_dn0 = assign82130_e124908_d_n0;
        locals.var_tq_dn2 = assign82130_e124908_d_n2;
        locals.var_tq_dn4 = assign82130_e124908_d_n4;
        locals.var_tq_dn5 = assign82130_e124908_d_n5;
        locals.var_tq_dn6 = assign82130_e124908_d_n6;
        locals.var_tq_dn7 = assign82130_e124908_d_n7;
        locals.var_tq_dn8 = assign82130_e124908_d_n8;
        locals.var_tq_dn9 = assign82130_e124908_d_n9;
        locals.var_tq_dn10 = assign82130_e124908_d_n10;
        locals.var_tq_dn11 = assign82130_e124908_d_n11;
        locals.var_tq_dn14 = assign82130_e124908_d_n14;

        let (assign82140_e124932, assign82140_e124932_d_n0, assign82140_e124932_d_n2, assign82140_e124932_d_n4, assign82140_e124932_d_n5, assign82140_e124932_d_n6, assign82140_e124932_d_n7, assign82140_e124932_d_n8, assign82140_e124932_d_n9, assign82140_e124932_d_n10, assign82140_e124932_d_n11, assign82140_e124932_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1912 == 0.0)) {
        let assign82140_e124918: f64 = (3.0 * locals.var_ta);
        let assign82140_e124920: f64 = (assign82140_e124918 * locals.var_tc);
        let assign82140_e124923: f64 = (locals.var_tb * locals.var_tb);
        let assign82140_e124924: f64 = (assign82140_e124920 - assign82140_e124923);
        let assign82140_e124927: f64 = (9.0 * locals.var_ta);
        let assign82140_e124929: f64 = (assign82140_e124927 * locals.var_ta);
        let assign82140_e124930: f64 = (assign82140_e124924 / assign82140_e124929);
        (assign82140_e124930, ((assign82140_e124918 * locals.var_tc_dn0) / assign82140_e124929), ((assign82140_e124918 * locals.var_tc_dn2) / assign82140_e124929), ((assign82140_e124918 * locals.var_tc_dn4) / assign82140_e124929), ((assign82140_e124918 * locals.var_tc_dn5) / assign82140_e124929), ((assign82140_e124918 * locals.var_tc_dn6) / assign82140_e124929), ((assign82140_e124918 * locals.var_tc_dn7) / assign82140_e124929), ((assign82140_e124918 * locals.var_tc_dn8) / assign82140_e124929), ((assign82140_e124918 * locals.var_tc_dn9) / assign82140_e124929), ((assign82140_e124918 * locals.var_tc_dn10) / assign82140_e124929), ((assign82140_e124918 * locals.var_tc_dn11) / assign82140_e124929), ((assign82140_e124918 * locals.var_tc_dn14) / assign82140_e124929),)
    } else {
        (locals.var_tp, locals.var_tp_dn0, locals.var_tp_dn2, locals.var_tp_dn4, locals.var_tp_dn5, locals.var_tp_dn6, locals.var_tp_dn7, locals.var_tp_dn8, locals.var_tp_dn9, locals.var_tp_dn10, locals.var_tp_dn11, locals.var_tp_dn14,)
    }
};
        locals.var_tp = assign82140_e124932;
        locals.var_tp_dn0 = assign82140_e124932_d_n0;
        locals.var_tp_dn2 = assign82140_e124932_d_n2;
        locals.var_tp_dn4 = assign82140_e124932_d_n4;
        locals.var_tp_dn5 = assign82140_e124932_d_n5;
        locals.var_tp_dn6 = assign82140_e124932_d_n6;
        locals.var_tp_dn7 = assign82140_e124932_d_n7;
        locals.var_tp_dn8 = assign82140_e124932_d_n8;
        locals.var_tp_dn9 = assign82140_e124932_d_n9;
        locals.var_tp_dn10 = assign82140_e124932_d_n10;
        locals.var_tp_dn11 = assign82140_e124932_d_n11;
        locals.var_tp_dn14 = assign82140_e124932_d_n14;

        let (assign82150_e124951, assign82150_e124951_d_n0, assign82150_e124951_d_n2, assign82150_e124951_d_n4, assign82150_e124951_d_n5, assign82150_e124951_d_n6, assign82150_e124951_d_n7, assign82150_e124951_d_n8, assign82150_e124951_d_n9, assign82150_e124951_d_n10, assign82150_e124951_d_n11, assign82150_e124951_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1912 == 0.0)) {
        let assign82150_e124942: f64 = (locals.var_tq * locals.var_tq);
        let assign82150_e124945: f64 = (locals.var_tp * locals.var_tp);
        let assign82150_e124947: f64 = (assign82150_e124945 * locals.var_tp);
        let assign82150_e124948: f64 = (assign82150_e124942 + assign82150_e124947);
        let assign82150_e124949: f64 = (assign82150_e124948).sqrt();
        (assign82150_e124949, ((((locals.var_tq_dn0 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn0)) + ((((locals.var_tp_dn0 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn0)) * locals.var_tp) + (assign82150_e124945 * locals.var_tp_dn0))) / (2.0 * assign82150_e124949)), ((((locals.var_tq_dn2 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn2)) + ((((locals.var_tp_dn2 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn2)) * locals.var_tp) + (assign82150_e124945 * locals.var_tp_dn2))) / (2.0 * assign82150_e124949)), ((((locals.var_tq_dn4 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn4)) + ((((locals.var_tp_dn4 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn4)) * locals.var_tp) + (assign82150_e124945 * locals.var_tp_dn4))) / (2.0 * assign82150_e124949)), ((((locals.var_tq_dn5 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn5)) + ((((locals.var_tp_dn5 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn5)) * locals.var_tp) + (assign82150_e124945 * locals.var_tp_dn5))) / (2.0 * assign82150_e124949)), ((((locals.var_tq_dn6 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn6)) + ((((locals.var_tp_dn6 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn6)) * locals.var_tp) + (assign82150_e124945 * locals.var_tp_dn6))) / (2.0 * assign82150_e124949)), ((((locals.var_tq_dn7 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn7)) + ((((locals.var_tp_dn7 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn7)) * locals.var_tp) + (assign82150_e124945 * locals.var_tp_dn7))) / (2.0 * assign82150_e124949)), ((((locals.var_tq_dn8 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn8)) + ((((locals.var_tp_dn8 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn8)) * locals.var_tp) + (assign82150_e124945 * locals.var_tp_dn8))) / (2.0 * assign82150_e124949)), ((((locals.var_tq_dn9 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn9)) + ((((locals.var_tp_dn9 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn9)) * locals.var_tp) + (assign82150_e124945 * locals.var_tp_dn9))) / (2.0 * assign82150_e124949)), ((((locals.var_tq_dn10 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn10)) + ((((locals.var_tp_dn10 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn10)) * locals.var_tp) + (assign82150_e124945 * locals.var_tp_dn10))) / (2.0 * assign82150_e124949)), ((((locals.var_tq_dn11 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn11)) + ((((locals.var_tp_dn11 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn11)) * locals.var_tp) + (assign82150_e124945 * locals.var_tp_dn11))) / (2.0 * assign82150_e124949)), ((((locals.var_tq_dn14 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn14)) + ((((locals.var_tp_dn14 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn14)) * locals.var_tp) + (assign82150_e124945 * locals.var_tp_dn14))) / (2.0 * assign82150_e124949)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign82150_e124951;
        locals.var_t5_dn0 = assign82150_e124951_d_n0;
        locals.var_t5_dn2 = assign82150_e124951_d_n2;
        locals.var_t5_dn4 = assign82150_e124951_d_n4;
        locals.var_t5_dn5 = assign82150_e124951_d_n5;
        locals.var_t5_dn6 = assign82150_e124951_d_n6;
        locals.var_t5_dn7 = assign82150_e124951_d_n7;
        locals.var_t5_dn8 = assign82150_e124951_d_n8;
        locals.var_t5_dn9 = assign82150_e124951_d_n9;
        locals.var_t5_dn10 = assign82150_e124951_d_n10;
        locals.var_t5_dn11 = assign82150_e124951_d_n11;
        locals.var_t5_dn14 = assign82150_e124951_d_n14;

        let (assign82160_e124966, assign82160_e124966_d_n0, assign82160_e124966_d_n2, assign82160_e124966_d_n4, assign82160_e124966_d_n5, assign82160_e124966_d_n6, assign82160_e124966_d_n7, assign82160_e124966_d_n8, assign82160_e124966_d_n9, assign82160_e124966_d_n10, assign82160_e124966_d_n11, assign82160_e124966_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1912 == 0.0)) {
        let assign82160_e124960: f64 = (-locals.var_tq);
        let assign82160_e124962: f64 = (assign82160_e124960 + locals.var_t5);
        let assign82160_e124964: f64 = (assign82160_e124962).powf(0.3333333333333333);
        (assign82160_e124964, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign82160_e124962).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn0) + locals.var_t5_dn0))) } } else { (assign82160_e124964 * (0.3333333333333333 * (((-locals.var_tq_dn0) + locals.var_t5_dn0) / assign82160_e124962))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign82160_e124962).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn2) + locals.var_t5_dn2))) } } else { (assign82160_e124964 * (0.3333333333333333 * (((-locals.var_tq_dn2) + locals.var_t5_dn2) / assign82160_e124962))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign82160_e124962).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn4) + locals.var_t5_dn4))) } } else { (assign82160_e124964 * (0.3333333333333333 * (((-locals.var_tq_dn4) + locals.var_t5_dn4) / assign82160_e124962))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign82160_e124962).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn5) + locals.var_t5_dn5))) } } else { (assign82160_e124964 * (0.3333333333333333 * (((-locals.var_tq_dn5) + locals.var_t5_dn5) / assign82160_e124962))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign82160_e124962).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn6) + locals.var_t5_dn6))) } } else { (assign82160_e124964 * (0.3333333333333333 * (((-locals.var_tq_dn6) + locals.var_t5_dn6) / assign82160_e124962))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign82160_e124962).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn7) + locals.var_t5_dn7))) } } else { (assign82160_e124964 * (0.3333333333333333 * (((-locals.var_tq_dn7) + locals.var_t5_dn7) / assign82160_e124962))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign82160_e124962).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn8) + locals.var_t5_dn8))) } } else { (assign82160_e124964 * (0.3333333333333333 * (((-locals.var_tq_dn8) + locals.var_t5_dn8) / assign82160_e124962))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign82160_e124962).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn9) + locals.var_t5_dn9))) } } else { (assign82160_e124964 * (0.3333333333333333 * (((-locals.var_tq_dn9) + locals.var_t5_dn9) / assign82160_e124962))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign82160_e124962).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn10) + locals.var_t5_dn10))) } } else { (assign82160_e124964 * (0.3333333333333333 * (((-locals.var_tq_dn10) + locals.var_t5_dn10) / assign82160_e124962))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign82160_e124962).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn11) + locals.var_t5_dn11))) } } else { (assign82160_e124964 * (0.3333333333333333 * (((-locals.var_tq_dn11) + locals.var_t5_dn11) / assign82160_e124962))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign82160_e124962).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn14) + locals.var_t5_dn14))) } } else { (assign82160_e124964 * (0.3333333333333333 * (((-locals.var_tq_dn14) + locals.var_t5_dn14) / assign82160_e124962))) },)
    } else {
        (locals.var_tu, locals.var_tu_dn0, locals.var_tu_dn2, locals.var_tu_dn4, locals.var_tu_dn5, locals.var_tu_dn6, locals.var_tu_dn7, locals.var_tu_dn8, locals.var_tu_dn9, locals.var_tu_dn10, locals.var_tu_dn11, locals.var_tu_dn14,)
    }
};
        locals.var_tu = assign82160_e124966;
        locals.var_tu_dn0 = assign82160_e124966_d_n0;
        locals.var_tu_dn2 = assign82160_e124966_d_n2;
        locals.var_tu_dn4 = assign82160_e124966_d_n4;
        locals.var_tu_dn5 = assign82160_e124966_d_n5;
        locals.var_tu_dn6 = assign82160_e124966_d_n6;
        locals.var_tu_dn7 = assign82160_e124966_d_n7;
        locals.var_tu_dn8 = assign82160_e124966_d_n8;
        locals.var_tu_dn9 = assign82160_e124966_d_n9;
        locals.var_tu_dn10 = assign82160_e124966_d_n10;
        locals.var_tu_dn11 = assign82160_e124966_d_n11;
        locals.var_tu_dn14 = assign82160_e124966_d_n14;

        let (assign82170_e124981, assign82170_e124981_d_n0, assign82170_e124981_d_n2, assign82170_e124981_d_n4, assign82170_e124981_d_n5, assign82170_e124981_d_n6, assign82170_e124981_d_n7, assign82170_e124981_d_n8, assign82170_e124981_d_n9, assign82170_e124981_d_n10, assign82170_e124981_d_n11, assign82170_e124981_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1912 == 0.0)) {
        let assign82170_e124976: f64 = (locals.var_tq + locals.var_t5);
        let assign82170_e124978: f64 = (assign82170_e124976).powf(0.3333333333333333);
        let assign82170_e124979: f64 = (-assign82170_e124978);
        (assign82170_e124979, (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign82170_e124976).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn0 + locals.var_t5_dn0))) } } else { (assign82170_e124978 * (0.3333333333333333 * ((locals.var_tq_dn0 + locals.var_t5_dn0) / assign82170_e124976))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign82170_e124976).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn2 + locals.var_t5_dn2))) } } else { (assign82170_e124978 * (0.3333333333333333 * ((locals.var_tq_dn2 + locals.var_t5_dn2) / assign82170_e124976))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign82170_e124976).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn4 + locals.var_t5_dn4))) } } else { (assign82170_e124978 * (0.3333333333333333 * ((locals.var_tq_dn4 + locals.var_t5_dn4) / assign82170_e124976))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign82170_e124976).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn5 + locals.var_t5_dn5))) } } else { (assign82170_e124978 * (0.3333333333333333 * ((locals.var_tq_dn5 + locals.var_t5_dn5) / assign82170_e124976))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign82170_e124976).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn6 + locals.var_t5_dn6))) } } else { (assign82170_e124978 * (0.3333333333333333 * ((locals.var_tq_dn6 + locals.var_t5_dn6) / assign82170_e124976))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign82170_e124976).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn7 + locals.var_t5_dn7))) } } else { (assign82170_e124978 * (0.3333333333333333 * ((locals.var_tq_dn7 + locals.var_t5_dn7) / assign82170_e124976))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign82170_e124976).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn8 + locals.var_t5_dn8))) } } else { (assign82170_e124978 * (0.3333333333333333 * ((locals.var_tq_dn8 + locals.var_t5_dn8) / assign82170_e124976))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign82170_e124976).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn9 + locals.var_t5_dn9))) } } else { (assign82170_e124978 * (0.3333333333333333 * ((locals.var_tq_dn9 + locals.var_t5_dn9) / assign82170_e124976))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign82170_e124976).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn10 + locals.var_t5_dn10))) } } else { (assign82170_e124978 * (0.3333333333333333 * ((locals.var_tq_dn10 + locals.var_t5_dn10) / assign82170_e124976))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign82170_e124976).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn11 + locals.var_t5_dn11))) } } else { (assign82170_e124978 * (0.3333333333333333 * ((locals.var_tq_dn11 + locals.var_t5_dn11) / assign82170_e124976))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign82170_e124976).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn14 + locals.var_t5_dn14))) } } else { (assign82170_e124978 * (0.3333333333333333 * ((locals.var_tq_dn14 + locals.var_t5_dn14) / assign82170_e124976))) }),)
    } else {
        (locals.var_tv, locals.var_tv_dn0, locals.var_tv_dn2, locals.var_tv_dn4, locals.var_tv_dn5, locals.var_tv_dn6, locals.var_tv_dn7, locals.var_tv_dn8, locals.var_tv_dn9, locals.var_tv_dn10, locals.var_tv_dn11, locals.var_tv_dn14,)
    }
};
        locals.var_tv = assign82170_e124981;
        locals.var_tv_dn0 = assign82170_e124981_d_n0;
        locals.var_tv_dn2 = assign82170_e124981_d_n2;
        locals.var_tv_dn4 = assign82170_e124981_d_n4;
        locals.var_tv_dn5 = assign82170_e124981_d_n5;
        locals.var_tv_dn6 = assign82170_e124981_d_n6;
        locals.var_tv_dn7 = assign82170_e124981_d_n7;
        locals.var_tv_dn8 = assign82170_e124981_d_n8;
        locals.var_tv_dn9 = assign82170_e124981_d_n9;
        locals.var_tv_dn10 = assign82170_e124981_d_n10;
        locals.var_tv_dn11 = assign82170_e124981_d_n11;
        locals.var_tv_dn14 = assign82170_e124981_d_n14;

        let (assign82180_e124999, assign82180_e124999_d_n0, assign82180_e124999_d_n2, assign82180_e124999_d_n4, assign82180_e124999_d_n5, assign82180_e124999_d_n6, assign82180_e124999_d_n7, assign82180_e124999_d_n8, assign82180_e124999_d_n9, assign82180_e124999_d_n10, assign82180_e124999_d_n11, assign82180_e124999_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1912 == 0.0)) {
        let assign82180_e124991: f64 = (locals.var_tu + locals.var_tv);
        let assign82180_e124995: f64 = (3.0 * locals.var_ta);
        let assign82180_e124996: f64 = (locals.var_tb / assign82180_e124995);
        let assign82180_e124997: f64 = (assign82180_e124991 - assign82180_e124996);
        (assign82180_e124997, (locals.var_tu_dn0 + locals.var_tv_dn0), (locals.var_tu_dn2 + locals.var_tv_dn2), (locals.var_tu_dn4 + locals.var_tv_dn4), (locals.var_tu_dn5 + locals.var_tv_dn5), (locals.var_tu_dn6 + locals.var_tv_dn6), (locals.var_tu_dn7 + locals.var_tv_dn7), (locals.var_tu_dn8 + locals.var_tv_dn8), (locals.var_tu_dn9 + locals.var_tv_dn9), (locals.var_tu_dn10 + locals.var_tv_dn10), (locals.var_tu_dn11 + locals.var_tv_dn11), (locals.var_tu_dn14 + locals.var_tv_dn14),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
        locals.var_chi = assign82180_e124999;
        locals.var_chi_dn0 = assign82180_e124999_d_n0;
        locals.var_chi_dn2 = assign82180_e124999_d_n2;
        locals.var_chi_dn4 = assign82180_e124999_d_n4;
        locals.var_chi_dn5 = assign82180_e124999_d_n5;
        locals.var_chi_dn6 = assign82180_e124999_d_n6;
        locals.var_chi_dn7 = assign82180_e124999_d_n7;
        locals.var_chi_dn8 = assign82180_e124999_d_n8;
        locals.var_chi_dn9 = assign82180_e124999_d_n9;
        locals.var_chi_dn10 = assign82180_e124999_d_n10;
        locals.var_chi_dn11 = assign82180_e124999_d_n11;
        locals.var_chi_dn14 = assign82180_e124999_d_n14;

        let (assign82190_e125013, assign82190_e125013_d_n0, assign82190_e125013_d_n2, assign82190_e125013_d_n4, assign82190_e125013_d_n5, assign82190_e125013_d_n6, assign82190_e125013_d_n7, assign82190_e125013_d_n8, assign82190_e125013_d_n9, assign82190_e125013_d_n10, assign82190_e125013_d_n11, assign82190_e125013_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1912 == 0.0)) {
        let assign82190_e125009: f64 = (locals.var_chi * locals.var_beta_inv);
        let assign82190_e125011: f64 = (assign82190_e125009 - locals.var_vxbgmtcl);
        (assign82190_e125011, (((locals.var_chi_dn0 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn0)) - locals.var_vxbgmtcl_dn0), (((locals.var_chi_dn2 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn2)) - locals.var_vxbgmtcl_dn2), (((locals.var_chi_dn4 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn4)) - locals.var_vxbgmtcl_dn4), (((locals.var_chi_dn5 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn5)) - locals.var_vxbgmtcl_dn5), (((locals.var_chi_dn6 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn6)) - locals.var_vxbgmtcl_dn6), (((locals.var_chi_dn7 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn7)) - locals.var_vxbgmtcl_dn7), (((locals.var_chi_dn8 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn8)) - locals.var_vxbgmtcl_dn8), (((locals.var_chi_dn9 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn9)) - locals.var_vxbgmtcl_dn9), (((locals.var_chi_dn10 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn10)) - locals.var_vxbgmtcl_dn10), (((locals.var_chi_dn11 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn11)) - locals.var_vxbgmtcl_dn11), (((locals.var_chi_dn14 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn14)) - locals.var_vxbgmtcl_dn14),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn14,)
    }
};
        locals.var_ps0_inia = assign82190_e125013;
        locals.var_ps0_inia_dn0 = assign82190_e125013_d_n0;
        locals.var_ps0_inia_dn2 = assign82190_e125013_d_n2;
        locals.var_ps0_inia_dn4 = assign82190_e125013_d_n4;
        locals.var_ps0_inia_dn5 = assign82190_e125013_d_n5;
        locals.var_ps0_inia_dn6 = assign82190_e125013_d_n6;
        locals.var_ps0_inia_dn7 = assign82190_e125013_d_n7;
        locals.var_ps0_inia_dn8 = assign82190_e125013_d_n8;
        locals.var_ps0_inia_dn9 = assign82190_e125013_d_n9;
        locals.var_ps0_inia_dn10 = assign82190_e125013_d_n10;
        locals.var_ps0_inia_dn11 = assign82190_e125013_d_n11;
        locals.var_ps0_inia_dn14 = assign82190_e125013_d_n14;

        let assign82200_e125016: f64 = if p.p33 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1913 = assign82200_e125016;

        let (assign82210_e125029, assign82210_e125029_d_n0, assign82210_e125029_d_n2, assign82210_e125029_d_n4, assign82210_e125029_d_n5, assign82210_e125029_d_n6, assign82210_e125029_d_n7, assign82210_e125029_d_n8, assign82210_e125029_d_n9, assign82210_e125029_d_n10, assign82210_e125029_d_n11, assign82210_e125029_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1913 != 0.0)) {
        let assign82210_e125025: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign82210_e125027: f64 = (assign82210_e125025 + 0.1);
        (assign82210_e125027, locals.var_vxbgmtcl_dn0, (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2), locals.var_vxbgmtcl_dn4, locals.var_vxbgmtcl_dn5, locals.var_vxbgmtcl_dn6, (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7), (locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8), (locals.var_vgpld_dn9 + locals.var_vxbgmtcl_dn9), locals.var_vxbgmtcl_dn10, locals.var_vxbgmtcl_dn11, locals.var_vxbgmtcl_dn14,)
    } else {
        (locals.var_vgpld_shift, locals.var_vgpld_shift_dn0, locals.var_vgpld_shift_dn2, locals.var_vgpld_shift_dn4, locals.var_vgpld_shift_dn5, locals.var_vgpld_shift_dn6, locals.var_vgpld_shift_dn7, locals.var_vgpld_shift_dn8, locals.var_vgpld_shift_dn9, locals.var_vgpld_shift_dn10, locals.var_vgpld_shift_dn11, locals.var_vgpld_shift_dn14,)
    }
};
        locals.var_vgpld_shift = assign82210_e125029;
        locals.var_vgpld_shift_dn0 = assign82210_e125029_d_n0;
        locals.var_vgpld_shift_dn2 = assign82210_e125029_d_n2;
        locals.var_vgpld_shift_dn4 = assign82210_e125029_d_n4;
        locals.var_vgpld_shift_dn5 = assign82210_e125029_d_n5;
        locals.var_vgpld_shift_dn6 = assign82210_e125029_d_n6;
        locals.var_vgpld_shift_dn7 = assign82210_e125029_d_n7;
        locals.var_vgpld_shift_dn8 = assign82210_e125029_d_n8;
        locals.var_vgpld_shift_dn9 = assign82210_e125029_d_n9;
        locals.var_vgpld_shift_dn10 = assign82210_e125029_d_n10;
        locals.var_vgpld_shift_dn11 = assign82210_e125029_d_n11;
        locals.var_vgpld_shift_dn14 = assign82210_e125029_d_n14;

        let (assign82220_e125040, assign82220_e125040_d_n0, assign82220_e125040_d_n2, assign82220_e125040_d_n4, assign82220_e125040_d_n5, assign82220_e125040_d_n6, assign82220_e125040_d_n7, assign82220_e125040_d_n8, assign82220_e125040_d_n9, assign82220_e125040_d_n10, assign82220_e125040_d_n11, assign82220_e125040_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1913 != 0.0)) {
        let assign82220_e125038: f64 = (locals.var_cnst1over * locals.var_exp_bvbs);
        (assign82220_e125038, ((locals.var_cnst1over_dn0 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn0)), ((locals.var_cnst1over_dn2 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn2)), ((locals.var_cnst1over_dn4 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn4)), ((locals.var_cnst1over_dn5 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn5)), ((locals.var_cnst1over_dn6 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn6)), ((locals.var_cnst1over_dn7 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn7)), ((locals.var_cnst1over_dn8 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn8)), ((locals.var_cnst1over_dn9 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn9)), ((locals.var_cnst1over_dn10 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn10)), ((locals.var_cnst1over_dn11 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn11)), ((locals.var_cnst1over_dn14 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn14)),)
    } else {
        (locals.var_cfs1, locals.var_cfs1_dn0, locals.var_cfs1_dn2, locals.var_cfs1_dn4, locals.var_cfs1_dn5, locals.var_cfs1_dn6, locals.var_cfs1_dn7, locals.var_cfs1_dn8, locals.var_cfs1_dn9, locals.var_cfs1_dn10, locals.var_cfs1_dn11, locals.var_cfs1_dn14,)
    }
};
        locals.var_cfs1 = assign82220_e125040;
        locals.var_cfs1_dn0 = assign82220_e125040_d_n0;
        locals.var_cfs1_dn2 = assign82220_e125040_d_n2;
        locals.var_cfs1_dn4 = assign82220_e125040_d_n4;
        locals.var_cfs1_dn5 = assign82220_e125040_d_n5;
        locals.var_cfs1_dn6 = assign82220_e125040_d_n6;
        locals.var_cfs1_dn7 = assign82220_e125040_d_n7;
        locals.var_cfs1_dn8 = assign82220_e125040_d_n8;
        locals.var_cfs1_dn9 = assign82220_e125040_d_n9;
        locals.var_cfs1_dn10 = assign82220_e125040_d_n10;
        locals.var_cfs1_dn11 = assign82220_e125040_d_n11;
        locals.var_cfs1_dn14 = assign82220_e125040_d_n14;

        let (assign82230_e125051, assign82230_e125051_d_n0, assign82230_e125051_d_n2, assign82230_e125051_d_n4, assign82230_e125051_d_n5, assign82230_e125051_d_n6, assign82230_e125051_d_n7, assign82230_e125051_d_n8, assign82230_e125051_d_n9, assign82230_e125051_d_n10, assign82230_e125051_d_n11, assign82230_e125051_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1913 != 0.0)) {
        let assign82230_e125049: f64 = (locals.var_cnst1over * locals.var_exp_bvbs);
        (assign82230_e125049, ((locals.var_cnst1over_dn0 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn0)), ((locals.var_cnst1over_dn2 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn2)), ((locals.var_cnst1over_dn4 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn4)), ((locals.var_cnst1over_dn5 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn5)), ((locals.var_cnst1over_dn6 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn6)), ((locals.var_cnst1over_dn7 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn7)), ((locals.var_cnst1over_dn8 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn8)), ((locals.var_cnst1over_dn9 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn9)), ((locals.var_cnst1over_dn10 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn10)), ((locals.var_cnst1over_dn11 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn11)), ((locals.var_cnst1over_dn14 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn14)),)
    } else {
        (locals.var_gammachi, locals.var_gammachi_dn0, locals.var_gammachi_dn2, locals.var_gammachi_dn4, locals.var_gammachi_dn5, locals.var_gammachi_dn6, locals.var_gammachi_dn7, locals.var_gammachi_dn8, locals.var_gammachi_dn9, locals.var_gammachi_dn10, locals.var_gammachi_dn11, locals.var_gammachi_dn14,)
    }
};
        locals.var_gammachi = assign82230_e125051;
        locals.var_gammachi_dn0 = assign82230_e125051_d_n0;
        locals.var_gammachi_dn2 = assign82230_e125051_d_n2;
        locals.var_gammachi_dn4 = assign82230_e125051_d_n4;
        locals.var_gammachi_dn5 = assign82230_e125051_d_n5;
        locals.var_gammachi_dn6 = assign82230_e125051_d_n6;
        locals.var_gammachi_dn7 = assign82230_e125051_d_n7;
        locals.var_gammachi_dn8 = assign82230_e125051_d_n8;
        locals.var_gammachi_dn9 = assign82230_e125051_d_n9;
        locals.var_gammachi_dn10 = assign82230_e125051_d_n10;
        locals.var_gammachi_dn11 = assign82230_e125051_d_n11;
        locals.var_gammachi_dn14 = assign82230_e125051_d_n14;

        let (assign82240_e125062, assign82240_e125062_d_n0, assign82240_e125062_d_n2, assign82240_e125062_d_n4, assign82240_e125062_d_n5, assign82240_e125062_d_n6, assign82240_e125062_d_n7, assign82240_e125062_d_n8, assign82240_e125062_d_n9, assign82240_e125062_d_n10, assign82240_e125062_d_n11, assign82240_e125062_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1913 != 0.0)) {
        let assign82240_e125060: f64 = (locals.var_beta2 * locals.var_fac1p2);
        (assign82240_e125060, ((locals.var_beta2_dn0 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn0)), ((locals.var_beta2_dn2 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn2)), ((locals.var_beta2_dn4 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn4)), ((locals.var_beta2_dn5 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn5)), ((locals.var_beta2_dn6 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn6)), ((locals.var_beta2_dn7 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn7)), ((locals.var_beta2_dn8 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn8)), ((locals.var_beta2_dn9 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn9)), ((locals.var_beta2_dn10 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn10)), ((locals.var_beta2_dn11 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn11)), ((locals.var_beta2_dn14 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign82240_e125062;
        locals.var_t0_dn0 = assign82240_e125062_d_n0;
        locals.var_t0_dn2 = assign82240_e125062_d_n2;
        locals.var_t0_dn4 = assign82240_e125062_d_n4;
        locals.var_t0_dn5 = assign82240_e125062_d_n5;
        locals.var_t0_dn6 = assign82240_e125062_d_n6;
        locals.var_t0_dn7 = assign82240_e125062_d_n7;
        locals.var_t0_dn8 = assign82240_e125062_d_n8;
        locals.var_t0_dn9 = assign82240_e125062_d_n9;
        locals.var_t0_dn10 = assign82240_e125062_d_n10;
        locals.var_t0_dn11 = assign82240_e125062_d_n11;
        locals.var_t0_dn14 = assign82240_e125062_d_n14;

        let (assign82250_e125073, assign82250_e125073_d_n0, assign82250_e125073_d_n2, assign82250_e125073_d_n4, assign82250_e125073_d_n5, assign82250_e125073_d_n6, assign82250_e125073_d_n7, assign82250_e125073_d_n8, assign82250_e125073_d_n9, assign82250_e125073_d_n10, assign82250_e125073_d_n11, assign82250_e125073_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1913 != 0.0)) {
        let assign82250_e125071: f64 = (locals.var_beta * locals.var_vgpld_shift);
        (assign82250_e125071, ((locals.var_beta_dn0 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn0)), ((locals.var_beta_dn2 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn2)), ((locals.var_beta_dn4 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn4)), ((locals.var_beta_dn5 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn5)), ((locals.var_beta_dn6 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn6)), ((locals.var_beta_dn7 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn7)), ((locals.var_beta_dn8 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn8)), ((locals.var_beta_dn9 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn9)), ((locals.var_beta_dn10 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn10)), ((locals.var_beta_dn11 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn11)), ((locals.var_beta_dn14 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn14)),)
    } else {
        (locals.var_psi, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn4, locals.var_psi_dn5, locals.var_psi_dn6, locals.var_psi_dn7, locals.var_psi_dn8, locals.var_psi_dn9, locals.var_psi_dn10, locals.var_psi_dn11, locals.var_psi_dn14,)
    }
};
        locals.var_psi = assign82250_e125073;
        locals.var_psi_dn0 = assign82250_e125073_d_n0;
        locals.var_psi_dn2 = assign82250_e125073_d_n2;
        locals.var_psi_dn4 = assign82250_e125073_d_n4;
        locals.var_psi_dn5 = assign82250_e125073_d_n5;
        locals.var_psi_dn6 = assign82250_e125073_d_n6;
        locals.var_psi_dn7 = assign82250_e125073_d_n7;
        locals.var_psi_dn8 = assign82250_e125073_d_n8;
        locals.var_psi_dn9 = assign82250_e125073_d_n9;
        locals.var_psi_dn10 = assign82250_e125073_d_n10;
        locals.var_psi_dn11 = assign82250_e125073_d_n11;
        locals.var_psi_dn14 = assign82250_e125073_d_n14;

        let (assign82260_e125098, assign82260_e125098_d_n0, assign82260_e125098_d_n2, assign82260_e125098_d_n4, assign82260_e125098_d_n5, assign82260_e125098_d_n6, assign82260_e125098_d_n7, assign82260_e125098_d_n8, assign82260_e125098_d_n9, assign82260_e125098_d_n10, assign82260_e125098_d_n11, assign82260_e125098_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1913 != 0.0)) {
        let assign82260_e125082: f64 = (locals.var_gammachi * locals.var_t0);
        let assign82260_e125085: f64 = (locals.var_psi * locals.var_psi);
        let assign82260_e125086: f64 = (assign82260_e125082 + assign82260_e125085);
        let assign82260_e125087: f64 = (assign82260_e125086).ln();
        let assign82260_e125090: f64 = (locals.var_cnst1over * locals.var_t0);
        let assign82260_e125091: f64 = (assign82260_e125090).ln();
        let assign82260_e125092: f64 = (assign82260_e125087 - assign82260_e125091);
        let assign82260_e125095: f64 = (locals.var_beta * locals.var_vxbgmtcl);
        let assign82260_e125096: f64 = (assign82260_e125092 + assign82260_e125095);
        (assign82260_e125096, ((((((locals.var_gammachi_dn0 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn0)) + ((locals.var_psi_dn0 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn0))) / assign82260_e125086) - (((locals.var_cnst1over_dn0 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn0)) / assign82260_e125090)) + ((locals.var_beta_dn0 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn0))), ((((((locals.var_gammachi_dn2 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn2)) + ((locals.var_psi_dn2 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn2))) / assign82260_e125086) - (((locals.var_cnst1over_dn2 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn2)) / assign82260_e125090)) + ((locals.var_beta_dn2 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn2))), ((((((locals.var_gammachi_dn4 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn4)) + ((locals.var_psi_dn4 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn4))) / assign82260_e125086) - (((locals.var_cnst1over_dn4 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn4)) / assign82260_e125090)) + ((locals.var_beta_dn4 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn4))), ((((((locals.var_gammachi_dn5 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn5)) + ((locals.var_psi_dn5 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn5))) / assign82260_e125086) - (((locals.var_cnst1over_dn5 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn5)) / assign82260_e125090)) + ((locals.var_beta_dn5 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn5))), ((((((locals.var_gammachi_dn6 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn6)) + ((locals.var_psi_dn6 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn6))) / assign82260_e125086) - (((locals.var_cnst1over_dn6 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn6)) / assign82260_e125090)) + ((locals.var_beta_dn6 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn6))), ((((((locals.var_gammachi_dn7 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn7)) + ((locals.var_psi_dn7 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn7))) / assign82260_e125086) - (((locals.var_cnst1over_dn7 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn7)) / assign82260_e125090)) + ((locals.var_beta_dn7 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn7))), ((((((locals.var_gammachi_dn8 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn8)) + ((locals.var_psi_dn8 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn8))) / assign82260_e125086) - (((locals.var_cnst1over_dn8 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn8)) / assign82260_e125090)) + ((locals.var_beta_dn8 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn8))), ((((((locals.var_gammachi_dn9 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn9)) + ((locals.var_psi_dn9 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn9))) / assign82260_e125086) - (((locals.var_cnst1over_dn9 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn9)) / assign82260_e125090)) + ((locals.var_beta_dn9 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn9))), ((((((locals.var_gammachi_dn10 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn10)) + ((locals.var_psi_dn10 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn10))) / assign82260_e125086) - (((locals.var_cnst1over_dn10 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn10)) / assign82260_e125090)) + ((locals.var_beta_dn10 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn10))), ((((((locals.var_gammachi_dn11 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn11)) + ((locals.var_psi_dn11 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn11))) / assign82260_e125086) - (((locals.var_cnst1over_dn11 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn11)) / assign82260_e125090)) + ((locals.var_beta_dn11 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn11))), ((((((locals.var_gammachi_dn14 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn14)) + ((locals.var_psi_dn14 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn14))) / assign82260_e125086) - (((locals.var_cnst1over_dn14 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn14)) / assign82260_e125090)) + ((locals.var_beta_dn14 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn14))),)
    } else {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn11, locals.var_chi_1_dn14,)
    }
};
        locals.var_chi_1 = assign82260_e125098;
        locals.var_chi_1_dn0 = assign82260_e125098_d_n0;
        locals.var_chi_1_dn2 = assign82260_e125098_d_n2;
        locals.var_chi_1_dn4 = assign82260_e125098_d_n4;
        locals.var_chi_1_dn5 = assign82260_e125098_d_n5;
        locals.var_chi_1_dn6 = assign82260_e125098_d_n6;
        locals.var_chi_1_dn7 = assign82260_e125098_d_n7;
        locals.var_chi_1_dn8 = assign82260_e125098_d_n8;
        locals.var_chi_1_dn9 = assign82260_e125098_d_n9;
        locals.var_chi_1_dn10 = assign82260_e125098_d_n10;
        locals.var_chi_1_dn11 = assign82260_e125098_d_n11;
        locals.var_chi_1_dn14 = assign82260_e125098_d_n14;

        let assign82270_e125101: f64 = if p.p33 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1914 = assign82270_e125101;

        let (assign82280_e125116, assign82280_e125116_d_n0, assign82280_e125116_d_n2, assign82280_e125116_d_n4, assign82280_e125116_d_n5, assign82280_e125116_d_n6, assign82280_e125116_d_n7, assign82280_e125116_d_n8, assign82280_e125116_d_n9, assign82280_e125116_d_n10, assign82280_e125116_d_n11, assign82280_e125116_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1914 != 0.0)) {
        let assign82280_e125112: f64 = (locals.var_psi - locals.var_chi_1);
        let assign82280_e125114: f64 = (assign82280_e125112 - 1.0);
        (assign82280_e125114, (locals.var_psi_dn0 - locals.var_chi_1_dn0), (locals.var_psi_dn2 - locals.var_chi_1_dn2), (locals.var_psi_dn4 - locals.var_chi_1_dn4), (locals.var_psi_dn5 - locals.var_chi_1_dn5), (locals.var_psi_dn6 - locals.var_chi_1_dn6), (locals.var_psi_dn7 - locals.var_chi_1_dn7), (locals.var_psi_dn8 - locals.var_chi_1_dn8), (locals.var_psi_dn9 - locals.var_chi_1_dn9), (locals.var_psi_dn10 - locals.var_chi_1_dn10), (locals.var_psi_dn11 - locals.var_chi_1_dn11), (locals.var_psi_dn14 - locals.var_chi_1_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign82280_e125116;
        locals.var_tmf1_dn0 = assign82280_e125116_d_n0;
        locals.var_tmf1_dn2 = assign82280_e125116_d_n2;
        locals.var_tmf1_dn4 = assign82280_e125116_d_n4;
        locals.var_tmf1_dn5 = assign82280_e125116_d_n5;
        locals.var_tmf1_dn6 = assign82280_e125116_d_n6;
        locals.var_tmf1_dn7 = assign82280_e125116_d_n7;
        locals.var_tmf1_dn8 = assign82280_e125116_d_n8;
        locals.var_tmf1_dn9 = assign82280_e125116_d_n9;
        locals.var_tmf1_dn10 = assign82280_e125116_d_n10;
        locals.var_tmf1_dn11 = assign82280_e125116_d_n11;
        locals.var_tmf1_dn14 = assign82280_e125116_d_n14;

        let (assign82290_e125131, assign82290_e125131_d_n0, assign82290_e125131_d_n2, assign82290_e125131_d_n4, assign82290_e125131_d_n5, assign82290_e125131_d_n6, assign82290_e125131_d_n7, assign82290_e125131_d_n8, assign82290_e125131_d_n9, assign82290_e125131_d_n10, assign82290_e125131_d_n11, assign82290_e125131_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1914 != 0.0)) {
        let assign82290_e125127: f64 = (4.0 * locals.var_psi);
        let assign82290_e125129: f64 = assign82290_e125127;
        (assign82290_e125129, (4.0 * locals.var_psi_dn0), (4.0 * locals.var_psi_dn2), (4.0 * locals.var_psi_dn4), (4.0 * locals.var_psi_dn5), (4.0 * locals.var_psi_dn6), (4.0 * locals.var_psi_dn7), (4.0 * locals.var_psi_dn8), (4.0 * locals.var_psi_dn9), (4.0 * locals.var_psi_dn10), (4.0 * locals.var_psi_dn11), (4.0 * locals.var_psi_dn14),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign82290_e125131;
        locals.var_tmf2_dn0 = assign82290_e125131_d_n0;
        locals.var_tmf2_dn2 = assign82290_e125131_d_n2;
        locals.var_tmf2_dn4 = assign82290_e125131_d_n4;
        locals.var_tmf2_dn5 = assign82290_e125131_d_n5;
        locals.var_tmf2_dn6 = assign82290_e125131_d_n6;
        locals.var_tmf2_dn7 = assign82290_e125131_d_n7;
        locals.var_tmf2_dn8 = assign82290_e125131_d_n8;
        locals.var_tmf2_dn9 = assign82290_e125131_d_n9;
        locals.var_tmf2_dn10 = assign82290_e125131_d_n10;
        locals.var_tmf2_dn11 = assign82290_e125131_d_n11;
        locals.var_tmf2_dn14 = assign82290_e125131_d_n14;

        let (assign82300_e125148, assign82300_e125148_d_n0, assign82300_e125148_d_n2, assign82300_e125148_d_n4, assign82300_e125148_d_n5, assign82300_e125148_d_n6, assign82300_e125148_d_n7, assign82300_e125148_d_n8, assign82300_e125148_d_n9, assign82300_e125148_d_n10, assign82300_e125148_d_n11, assign82300_e125148_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1914 != 0.0)) {
        let (assign82300_e125146, assign82300_e125146_d_n0, assign82300_e125146_d_n2, assign82300_e125146_d_n4, assign82300_e125146_d_n5, assign82300_e125146_d_n6, assign82300_e125146_d_n7, assign82300_e125146_d_n8, assign82300_e125146_d_n9, assign82300_e125146_d_n10, assign82300_e125146_d_n11, assign82300_e125146_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign82300_e125145: f64 = (-locals.var_tmf2);
                (assign82300_e125145, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign82300_e125146, assign82300_e125146_d_n0, assign82300_e125146_d_n2, assign82300_e125146_d_n4, assign82300_e125146_d_n5, assign82300_e125146_d_n6, assign82300_e125146_d_n7, assign82300_e125146_d_n8, assign82300_e125146_d_n9, assign82300_e125146_d_n10, assign82300_e125146_d_n11, assign82300_e125146_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign82300_e125148;
        locals.var_tmf2_dn0 = assign82300_e125148_d_n0;
        locals.var_tmf2_dn2 = assign82300_e125148_d_n2;
        locals.var_tmf2_dn4 = assign82300_e125148_d_n4;
        locals.var_tmf2_dn5 = assign82300_e125148_d_n5;
        locals.var_tmf2_dn6 = assign82300_e125148_d_n6;
        locals.var_tmf2_dn7 = assign82300_e125148_d_n7;
        locals.var_tmf2_dn8 = assign82300_e125148_d_n8;
        locals.var_tmf2_dn9 = assign82300_e125148_d_n9;
        locals.var_tmf2_dn10 = assign82300_e125148_d_n10;
        locals.var_tmf2_dn11 = assign82300_e125148_d_n11;
        locals.var_tmf2_dn14 = assign82300_e125148_d_n14;

    }

    pub(super) fn stamp_transient_block_298(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign82310_e125164, assign82310_e125164_d_n0, assign82310_e125164_d_n2, assign82310_e125164_d_n4, assign82310_e125164_d_n5, assign82310_e125164_d_n6, assign82310_e125164_d_n7, assign82310_e125164_d_n8, assign82310_e125164_d_n9, assign82310_e125164_d_n10, assign82310_e125164_d_n11, assign82310_e125164_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1914 != 0.0)) {
        let assign82310_e125159: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign82310_e125161: f64 = (assign82310_e125159 + locals.var_tmf2);
        let assign82310_e125162: f64 = (assign82310_e125161).sqrt();
        (assign82310_e125162, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign82310_e125162)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign82310_e125162)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign82310_e125162)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign82310_e125162)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign82310_e125162)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign82310_e125162)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign82310_e125162)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign82310_e125162)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign82310_e125162)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign82310_e125162)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign82310_e125162)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign82310_e125164;
        locals.var_tmf2_dn0 = assign82310_e125164_d_n0;
        locals.var_tmf2_dn2 = assign82310_e125164_d_n2;
        locals.var_tmf2_dn4 = assign82310_e125164_d_n4;
        locals.var_tmf2_dn5 = assign82310_e125164_d_n5;
        locals.var_tmf2_dn6 = assign82310_e125164_d_n6;
        locals.var_tmf2_dn7 = assign82310_e125164_d_n7;
        locals.var_tmf2_dn8 = assign82310_e125164_d_n8;
        locals.var_tmf2_dn9 = assign82310_e125164_d_n9;
        locals.var_tmf2_dn10 = assign82310_e125164_d_n10;
        locals.var_tmf2_dn11 = assign82310_e125164_d_n11;
        locals.var_tmf2_dn14 = assign82310_e125164_d_n14;

        let (assign82320_e125181, assign82320_e125181_d_n0, assign82320_e125181_d_n2, assign82320_e125181_d_n4, assign82320_e125181_d_n5, assign82320_e125181_d_n6, assign82320_e125181_d_n7, assign82320_e125181_d_n8, assign82320_e125181_d_n9, assign82320_e125181_d_n10, assign82320_e125181_d_n11, assign82320_e125181_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1914 != 0.0)) {
        let assign82320_e125177: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign82320_e125178: f64 = (1.0 + assign82320_e125177);
        let assign82320_e125179: f64 = (0.5 * assign82320_e125178);
        (assign82320_e125179, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign82320_e125181;
        locals.var_t1_dn0 = assign82320_e125181_d_n0;
        locals.var_t1_dn2 = assign82320_e125181_d_n2;
        locals.var_t1_dn4 = assign82320_e125181_d_n4;
        locals.var_t1_dn5 = assign82320_e125181_d_n5;
        locals.var_t1_dn6 = assign82320_e125181_d_n6;
        locals.var_t1_dn7 = assign82320_e125181_d_n7;
        locals.var_t1_dn8 = assign82320_e125181_d_n8;
        locals.var_t1_dn9 = assign82320_e125181_d_n9;
        locals.var_t1_dn10 = assign82320_e125181_d_n10;
        locals.var_t1_dn11 = assign82320_e125181_d_n11;
        locals.var_t1_dn14 = assign82320_e125181_d_n14;

        let (assign82330_e125198, assign82330_e125198_d_n0, assign82330_e125198_d_n2, assign82330_e125198_d_n4, assign82330_e125198_d_n5, assign82330_e125198_d_n6, assign82330_e125198_d_n7, assign82330_e125198_d_n8, assign82330_e125198_d_n9, assign82330_e125198_d_n10, assign82330_e125198_d_n11, assign82330_e125198_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1914 != 0.0)) {
        let assign82330_e125194: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign82330_e125195: f64 = (0.5 * assign82330_e125194);
        let assign82330_e125196: f64 = (locals.var_psi - assign82330_e125195);
        (assign82330_e125196, (locals.var_psi_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_psi_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_psi_dn4 - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_psi_dn5 - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_psi_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_psi_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_psi_dn8 - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_psi_dn9 - (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (locals.var_psi_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_psi_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_psi_dn14 - (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn11, locals.var_chi_1_dn14,)
    }
};
        locals.var_chi_1 = assign82330_e125198;
        locals.var_chi_1_dn0 = assign82330_e125198_d_n0;
        locals.var_chi_1_dn2 = assign82330_e125198_d_n2;
        locals.var_chi_1_dn4 = assign82330_e125198_d_n4;
        locals.var_chi_1_dn5 = assign82330_e125198_d_n5;
        locals.var_chi_1_dn6 = assign82330_e125198_d_n6;
        locals.var_chi_1_dn7 = assign82330_e125198_d_n7;
        locals.var_chi_1_dn8 = assign82330_e125198_d_n8;
        locals.var_chi_1_dn9 = assign82330_e125198_d_n9;
        locals.var_chi_1_dn10 = assign82330_e125198_d_n10;
        locals.var_chi_1_dn11 = assign82330_e125198_d_n11;
        locals.var_chi_1_dn14 = assign82330_e125198_d_n14;

        let (assign82340_e125215, assign82340_e125215_d_n0, assign82340_e125215_d_n2, assign82340_e125215_d_n4, assign82340_e125215_d_n5, assign82340_e125215_d_n6, assign82340_e125215_d_n7, assign82340_e125215_d_n8, assign82340_e125215_d_n9, assign82340_e125215_d_n10, assign82340_e125215_d_n11, assign82340_e125215_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1914 == 0.0)) {
        let (assign82340_e125213, assign82340_e125213_d_n0, assign82340_e125213_d_n2, assign82340_e125213_d_n4, assign82340_e125213_d_n5, assign82340_e125213_d_n6, assign82340_e125213_d_n7, assign82340_e125213_d_n8, assign82340_e125213_d_n9, assign82340_e125213_d_n10, assign82340_e125213_d_n11, assign82340_e125213_d_n14,) = {
            if (locals.var_chi_1 <= locals.var_psi) {
                (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn11, locals.var_chi_1_dn14,)
            } else {
                (locals.var_psi, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn4, locals.var_psi_dn5, locals.var_psi_dn6, locals.var_psi_dn7, locals.var_psi_dn8, locals.var_psi_dn9, locals.var_psi_dn10, locals.var_psi_dn11, locals.var_psi_dn14,)
            }
        };
        (assign82340_e125213, assign82340_e125213_d_n0, assign82340_e125213_d_n2, assign82340_e125213_d_n4, assign82340_e125213_d_n5, assign82340_e125213_d_n6, assign82340_e125213_d_n7, assign82340_e125213_d_n8, assign82340_e125213_d_n9, assign82340_e125213_d_n10, assign82340_e125213_d_n11, assign82340_e125213_d_n14,)
    } else {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn11, locals.var_chi_1_dn14,)
    }
};
        locals.var_chi_1 = assign82340_e125215;
        locals.var_chi_1_dn0 = assign82340_e125215_d_n0;
        locals.var_chi_1_dn2 = assign82340_e125215_d_n2;
        locals.var_chi_1_dn4 = assign82340_e125215_d_n4;
        locals.var_chi_1_dn5 = assign82340_e125215_d_n5;
        locals.var_chi_1_dn6 = assign82340_e125215_d_n6;
        locals.var_chi_1_dn7 = assign82340_e125215_d_n7;
        locals.var_chi_1_dn8 = assign82340_e125215_d_n8;
        locals.var_chi_1_dn9 = assign82340_e125215_d_n9;
        locals.var_chi_1_dn10 = assign82340_e125215_d_n10;
        locals.var_chi_1_dn11 = assign82340_e125215_d_n11;
        locals.var_chi_1_dn14 = assign82340_e125215_d_n14;

        let (assign82350_e125229, assign82350_e125229_d_n0, assign82350_e125229_d_n2, assign82350_e125229_d_n4, assign82350_e125229_d_n5, assign82350_e125229_d_n6, assign82350_e125229_d_n7, assign82350_e125229_d_n8, assign82350_e125229_d_n9, assign82350_e125229_d_n10, assign82350_e125229_d_n11, assign82350_e125229_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1913 != 0.0)) {
        let (assign82350_e125227, assign82350_e125227_d_n0, assign82350_e125227_d_n2, assign82350_e125227_d_n4, assign82350_e125227_d_n5, assign82350_e125227_d_n6, assign82350_e125227_d_n7, assign82350_e125227_d_n8, assign82350_e125227_d_n9, assign82350_e125227_d_n10, assign82350_e125227_d_n11, assign82350_e125227_d_n14,) = {
            if (locals.var_chi_1 >= 0.0) {
                (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn11, locals.var_chi_1_dn14,)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign82350_e125227, assign82350_e125227_d_n0, assign82350_e125227_d_n2, assign82350_e125227_d_n4, assign82350_e125227_d_n5, assign82350_e125227_d_n6, assign82350_e125227_d_n7, assign82350_e125227_d_n8, assign82350_e125227_d_n9, assign82350_e125227_d_n10, assign82350_e125227_d_n11, assign82350_e125227_d_n14,)
    } else {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn11, locals.var_chi_1_dn14,)
    }
};
        locals.var_chi_1 = assign82350_e125229;
        locals.var_chi_1_dn0 = assign82350_e125229_d_n0;
        locals.var_chi_1_dn2 = assign82350_e125229_d_n2;
        locals.var_chi_1_dn4 = assign82350_e125229_d_n4;
        locals.var_chi_1_dn5 = assign82350_e125229_d_n5;
        locals.var_chi_1_dn6 = assign82350_e125229_d_n6;
        locals.var_chi_1_dn7 = assign82350_e125229_d_n7;
        locals.var_chi_1_dn8 = assign82350_e125229_d_n8;
        locals.var_chi_1_dn9 = assign82350_e125229_d_n9;
        locals.var_chi_1_dn10 = assign82350_e125229_d_n10;
        locals.var_chi_1_dn11 = assign82350_e125229_d_n11;
        locals.var_chi_1_dn14 = assign82350_e125229_d_n14;

        let (assign82360_e125240, assign82360_e125240_d_n0, assign82360_e125240_d_n2, assign82360_e125240_d_n4, assign82360_e125240_d_n5, assign82360_e125240_d_n6, assign82360_e125240_d_n7, assign82360_e125240_d_n8, assign82360_e125240_d_n9, assign82360_e125240_d_n10, assign82360_e125240_d_n11, assign82360_e125240_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1913 != 0.0)) {
        let assign82360_e125238: f64 = (locals.var_psi - locals.var_chi_1);
        (assign82360_e125238, (locals.var_psi_dn0 - locals.var_chi_1_dn0), (locals.var_psi_dn2 - locals.var_chi_1_dn2), (locals.var_psi_dn4 - locals.var_chi_1_dn4), (locals.var_psi_dn5 - locals.var_chi_1_dn5), (locals.var_psi_dn6 - locals.var_chi_1_dn6), (locals.var_psi_dn7 - locals.var_chi_1_dn7), (locals.var_psi_dn8 - locals.var_chi_1_dn8), (locals.var_psi_dn9 - locals.var_chi_1_dn9), (locals.var_psi_dn10 - locals.var_chi_1_dn10), (locals.var_psi_dn11 - locals.var_chi_1_dn11), (locals.var_psi_dn14 - locals.var_chi_1_dn14),)
    } else {
        (locals.var_psi, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn4, locals.var_psi_dn5, locals.var_psi_dn6, locals.var_psi_dn7, locals.var_psi_dn8, locals.var_psi_dn9, locals.var_psi_dn10, locals.var_psi_dn11, locals.var_psi_dn14,)
    }
};
        locals.var_psi = assign82360_e125240;
        locals.var_psi_dn0 = assign82360_e125240_d_n0;
        locals.var_psi_dn2 = assign82360_e125240_d_n2;
        locals.var_psi_dn4 = assign82360_e125240_d_n4;
        locals.var_psi_dn5 = assign82360_e125240_d_n5;
        locals.var_psi_dn6 = assign82360_e125240_d_n6;
        locals.var_psi_dn7 = assign82360_e125240_d_n7;
        locals.var_psi_dn8 = assign82360_e125240_d_n8;
        locals.var_psi_dn9 = assign82360_e125240_d_n9;
        locals.var_psi_dn10 = assign82360_e125240_d_n10;
        locals.var_psi_dn11 = assign82360_e125240_d_n11;
        locals.var_psi_dn14 = assign82360_e125240_d_n14;

        let (assign82370_e125253, assign82370_e125253_d_n0, assign82370_e125253_d_n2, assign82370_e125253_d_n4, assign82370_e125253_d_n5, assign82370_e125253_d_n6, assign82370_e125253_d_n7, assign82370_e125253_d_n8, assign82370_e125253_d_n9, assign82370_e125253_d_n10, assign82370_e125253_d_n11, assign82370_e125253_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1913 != 0.0)) {
        let assign82370_e125250: f64 = (locals.var_beta * 0.1);
        let assign82370_e125251: f64 = (locals.var_psi + assign82370_e125250);
        (assign82370_e125251, (locals.var_psi_dn0 + (locals.var_beta_dn0 * 0.1)), (locals.var_psi_dn2 + (locals.var_beta_dn2 * 0.1)), (locals.var_psi_dn4 + (locals.var_beta_dn4 * 0.1)), (locals.var_psi_dn5 + (locals.var_beta_dn5 * 0.1)), (locals.var_psi_dn6 + (locals.var_beta_dn6 * 0.1)), (locals.var_psi_dn7 + (locals.var_beta_dn7 * 0.1)), (locals.var_psi_dn8 + (locals.var_beta_dn8 * 0.1)), (locals.var_psi_dn9 + (locals.var_beta_dn9 * 0.1)), (locals.var_psi_dn10 + (locals.var_beta_dn10 * 0.1)), (locals.var_psi_dn11 + (locals.var_beta_dn11 * 0.1)), (locals.var_psi_dn14 + (locals.var_beta_dn14 * 0.1)),)
    } else {
        (locals.var_psi, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn4, locals.var_psi_dn5, locals.var_psi_dn6, locals.var_psi_dn7, locals.var_psi_dn8, locals.var_psi_dn9, locals.var_psi_dn10, locals.var_psi_dn11, locals.var_psi_dn14,)
    }
};
        locals.var_psi = assign82370_e125253;
        locals.var_psi_dn0 = assign82370_e125253_d_n0;
        locals.var_psi_dn2 = assign82370_e125253_d_n2;
        locals.var_psi_dn4 = assign82370_e125253_d_n4;
        locals.var_psi_dn5 = assign82370_e125253_d_n5;
        locals.var_psi_dn6 = assign82370_e125253_d_n6;
        locals.var_psi_dn7 = assign82370_e125253_d_n7;
        locals.var_psi_dn8 = assign82370_e125253_d_n8;
        locals.var_psi_dn9 = assign82370_e125253_d_n9;
        locals.var_psi_dn10 = assign82370_e125253_d_n10;
        locals.var_psi_dn11 = assign82370_e125253_d_n11;
        locals.var_psi_dn14 = assign82370_e125253_d_n14;

        let (assign82380_e125274, assign82380_e125274_d_n0, assign82380_e125274_d_n2, assign82380_e125274_d_n4, assign82380_e125274_d_n5, assign82380_e125274_d_n6, assign82380_e125274_d_n7, assign82380_e125274_d_n8, assign82380_e125274_d_n9, assign82380_e125274_d_n10, assign82380_e125274_d_n11, assign82380_e125274_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1913 != 0.0)) {
        let assign82380_e125262: f64 = (locals.var_gammachi * locals.var_t0);
        let assign82380_e125265: f64 = (locals.var_psi * locals.var_psi);
        let assign82380_e125266: f64 = (assign82380_e125262 + assign82380_e125265);
        let assign82380_e125267: f64 = (assign82380_e125266).ln();
        let assign82380_e125270: f64 = (locals.var_cnst1over * locals.var_t0);
        let assign82380_e125271: f64 = (assign82380_e125270).ln();
        let assign82380_e125272: f64 = (assign82380_e125267 - assign82380_e125271);
        (assign82380_e125272, (((((locals.var_gammachi_dn0 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn0)) + ((locals.var_psi_dn0 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn0))) / assign82380_e125266) - (((locals.var_cnst1over_dn0 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn0)) / assign82380_e125270)), (((((locals.var_gammachi_dn2 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn2)) + ((locals.var_psi_dn2 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn2))) / assign82380_e125266) - (((locals.var_cnst1over_dn2 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn2)) / assign82380_e125270)), (((((locals.var_gammachi_dn4 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn4)) + ((locals.var_psi_dn4 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn4))) / assign82380_e125266) - (((locals.var_cnst1over_dn4 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn4)) / assign82380_e125270)), (((((locals.var_gammachi_dn5 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn5)) + ((locals.var_psi_dn5 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn5))) / assign82380_e125266) - (((locals.var_cnst1over_dn5 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn5)) / assign82380_e125270)), (((((locals.var_gammachi_dn6 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn6)) + ((locals.var_psi_dn6 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn6))) / assign82380_e125266) - (((locals.var_cnst1over_dn6 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn6)) / assign82380_e125270)), (((((locals.var_gammachi_dn7 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn7)) + ((locals.var_psi_dn7 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn7))) / assign82380_e125266) - (((locals.var_cnst1over_dn7 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn7)) / assign82380_e125270)), (((((locals.var_gammachi_dn8 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn8)) + ((locals.var_psi_dn8 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn8))) / assign82380_e125266) - (((locals.var_cnst1over_dn8 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn8)) / assign82380_e125270)), (((((locals.var_gammachi_dn9 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn9)) + ((locals.var_psi_dn9 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn9))) / assign82380_e125266) - (((locals.var_cnst1over_dn9 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn9)) / assign82380_e125270)), (((((locals.var_gammachi_dn10 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn10)) + ((locals.var_psi_dn10 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn10))) / assign82380_e125266) - (((locals.var_cnst1over_dn10 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn10)) / assign82380_e125270)), (((((locals.var_gammachi_dn11 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn11)) + ((locals.var_psi_dn11 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn11))) / assign82380_e125266) - (((locals.var_cnst1over_dn11 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn11)) / assign82380_e125270)), (((((locals.var_gammachi_dn14 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn14)) + ((locals.var_psi_dn14 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn14))) / assign82380_e125266) - (((locals.var_cnst1over_dn14 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn14)) / assign82380_e125270)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign82380_e125274;
        locals.var_t1_dn0 = assign82380_e125274_d_n0;
        locals.var_t1_dn2 = assign82380_e125274_d_n2;
        locals.var_t1_dn4 = assign82380_e125274_d_n4;
        locals.var_t1_dn5 = assign82380_e125274_d_n5;
        locals.var_t1_dn6 = assign82380_e125274_d_n6;
        locals.var_t1_dn7 = assign82380_e125274_d_n7;
        locals.var_t1_dn8 = assign82380_e125274_d_n8;
        locals.var_t1_dn9 = assign82380_e125274_d_n9;
        locals.var_t1_dn10 = assign82380_e125274_d_n10;
        locals.var_t1_dn11 = assign82380_e125274_d_n11;
        locals.var_t1_dn14 = assign82380_e125274_d_n14;

        let (assign82390_e125287, assign82390_e125287_d_n0, assign82390_e125287_d_n2, assign82390_e125287_d_n4, assign82390_e125287_d_n5, assign82390_e125287_d_n6, assign82390_e125287_d_n7, assign82390_e125287_d_n8, assign82390_e125287_d_n9, assign82390_e125287_d_n10, assign82390_e125287_d_n11, assign82390_e125287_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1913 != 0.0)) {
        let assign82390_e125284: f64 = (locals.var_beta * locals.var_vxbgmtcl);
        let assign82390_e125285: f64 = (locals.var_t1 + assign82390_e125284);
        (assign82390_e125285, (locals.var_t1_dn0 + ((locals.var_beta_dn0 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn0))), (locals.var_t1_dn2 + ((locals.var_beta_dn2 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn2))), (locals.var_t1_dn4 + ((locals.var_beta_dn4 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn4))), (locals.var_t1_dn5 + ((locals.var_beta_dn5 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn5))), (locals.var_t1_dn6 + ((locals.var_beta_dn6 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn6))), (locals.var_t1_dn7 + ((locals.var_beta_dn7 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn7))), (locals.var_t1_dn8 + ((locals.var_beta_dn8 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn8))), (locals.var_t1_dn9 + ((locals.var_beta_dn9 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn9))), (locals.var_t1_dn10 + ((locals.var_beta_dn10 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn10))), (locals.var_t1_dn11 + ((locals.var_beta_dn11 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn11))), (locals.var_t1_dn14 + ((locals.var_beta_dn14 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn14))),)
    } else {
        (locals.var_chi_b, locals.var_chi_b_dn0, locals.var_chi_b_dn2, locals.var_chi_b_dn4, locals.var_chi_b_dn5, locals.var_chi_b_dn6, locals.var_chi_b_dn7, locals.var_chi_b_dn8, locals.var_chi_b_dn9, locals.var_chi_b_dn10, locals.var_chi_b_dn11, locals.var_chi_b_dn14,)
    }
};
        locals.var_chi_b = assign82390_e125287;
        locals.var_chi_b_dn0 = assign82390_e125287_d_n0;
        locals.var_chi_b_dn2 = assign82390_e125287_d_n2;
        locals.var_chi_b_dn4 = assign82390_e125287_d_n4;
        locals.var_chi_b_dn5 = assign82390_e125287_d_n5;
        locals.var_chi_b_dn6 = assign82390_e125287_d_n6;
        locals.var_chi_b_dn7 = assign82390_e125287_d_n7;
        locals.var_chi_b_dn8 = assign82390_e125287_d_n8;
        locals.var_chi_b_dn9 = assign82390_e125287_d_n9;
        locals.var_chi_b_dn10 = assign82390_e125287_d_n10;
        locals.var_chi_b_dn11 = assign82390_e125287_d_n11;
        locals.var_chi_b_dn14 = assign82390_e125287_d_n14;

        let (assign82400_e125301, assign82400_e125301_d_n0, assign82400_e125301_d_n2, assign82400_e125301_d_n4, assign82400_e125301_d_n5, assign82400_e125301_d_n6, assign82400_e125301_d_n7, assign82400_e125301_d_n8, assign82400_e125301_d_n9, assign82400_e125301_d_n10, assign82400_e125301_d_n11, assign82400_e125301_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1913 != 0.0)) {
        let (assign82400_e125299, assign82400_e125299_d_n0, assign82400_e125299_d_n2, assign82400_e125299_d_n4, assign82400_e125299_d_n5, assign82400_e125299_d_n6, assign82400_e125299_d_n7, assign82400_e125299_d_n8, assign82400_e125299_d_n9, assign82400_e125299_d_n10, assign82400_e125299_d_n11, assign82400_e125299_d_n14,) = {
            if (locals.var_chi_b >= 0.0) {
                (locals.var_chi_b, locals.var_chi_b_dn0, locals.var_chi_b_dn2, locals.var_chi_b_dn4, locals.var_chi_b_dn5, locals.var_chi_b_dn6, locals.var_chi_b_dn7, locals.var_chi_b_dn8, locals.var_chi_b_dn9, locals.var_chi_b_dn10, locals.var_chi_b_dn11, locals.var_chi_b_dn14,)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign82400_e125299, assign82400_e125299_d_n0, assign82400_e125299_d_n2, assign82400_e125299_d_n4, assign82400_e125299_d_n5, assign82400_e125299_d_n6, assign82400_e125299_d_n7, assign82400_e125299_d_n8, assign82400_e125299_d_n9, assign82400_e125299_d_n10, assign82400_e125299_d_n11, assign82400_e125299_d_n14,)
    } else {
        (locals.var_chi_b, locals.var_chi_b_dn0, locals.var_chi_b_dn2, locals.var_chi_b_dn4, locals.var_chi_b_dn5, locals.var_chi_b_dn6, locals.var_chi_b_dn7, locals.var_chi_b_dn8, locals.var_chi_b_dn9, locals.var_chi_b_dn10, locals.var_chi_b_dn11, locals.var_chi_b_dn14,)
    }
};
        locals.var_chi_b = assign82400_e125301;
        locals.var_chi_b_dn0 = assign82400_e125301_d_n0;
        locals.var_chi_b_dn2 = assign82400_e125301_d_n2;
        locals.var_chi_b_dn4 = assign82400_e125301_d_n4;
        locals.var_chi_b_dn5 = assign82400_e125301_d_n5;
        locals.var_chi_b_dn6 = assign82400_e125301_d_n6;
        locals.var_chi_b_dn7 = assign82400_e125301_d_n7;
        locals.var_chi_b_dn8 = assign82400_e125301_d_n8;
        locals.var_chi_b_dn9 = assign82400_e125301_d_n9;
        locals.var_chi_b_dn10 = assign82400_e125301_d_n10;
        locals.var_chi_b_dn11 = assign82400_e125301_d_n11;
        locals.var_chi_b_dn14 = assign82400_e125301_d_n14;

        let (assign82410_e125310, assign82410_e125310_d_n0, assign82410_e125310_d_n2, assign82410_e125310_d_n4, assign82410_e125310_d_n5, assign82410_e125310_d_n6, assign82410_e125310_d_n7, assign82410_e125310_d_n8, assign82410_e125310_d_n9, assign82410_e125310_d_n10, assign82410_e125310_d_n11, assign82410_e125310_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1913 != 0.0)) {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    } else {
        (locals.var_chi_a, locals.var_chi_a_dn0, locals.var_chi_a_dn2, locals.var_chi_a_dn4, locals.var_chi_a_dn5, locals.var_chi_a_dn6, locals.var_chi_a_dn7, locals.var_chi_a_dn8, locals.var_chi_a_dn9, locals.var_chi_a_dn10, locals.var_chi_a_dn11, locals.var_chi_a_dn14,)
    }
};
        locals.var_chi_a = assign82410_e125310;
        locals.var_chi_a_dn0 = assign82410_e125310_d_n0;
        locals.var_chi_a_dn2 = assign82410_e125310_d_n2;
        locals.var_chi_a_dn4 = assign82410_e125310_d_n4;
        locals.var_chi_a_dn5 = assign82410_e125310_d_n5;
        locals.var_chi_a_dn6 = assign82410_e125310_d_n6;
        locals.var_chi_a_dn7 = assign82410_e125310_d_n7;
        locals.var_chi_a_dn8 = assign82410_e125310_d_n8;
        locals.var_chi_a_dn9 = assign82410_e125310_d_n9;
        locals.var_chi_a_dn10 = assign82410_e125310_d_n10;
        locals.var_chi_a_dn11 = assign82410_e125310_d_n11;
        locals.var_chi_a_dn14 = assign82410_e125310_d_n14;

        let assign82420_e125313: f64 = if p.p33 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1915 = assign82420_e125313;

        let assign82430_e125318: f64 = (0.2 * locals.var_chi_b);
        let assign82430_e125319: f64 = (locals.var_chi_b - assign82430_e125318);
        let assign82430_e125323: f64 = (0.2 * locals.var_chi_b);
        let assign82430_e125326: f64 = if ((locals.var_chi_a > assign82430_e125319) && (assign82430_e125323 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1916 = assign82430_e125326;

        let (assign82440_e125345, assign82440_e125345_d_n0, assign82440_e125345_d_n2, assign82440_e125345_d_n4, assign82440_e125345_d_n5, assign82440_e125345_d_n6, assign82440_e125345_d_n7, assign82440_e125345_d_n8, assign82440_e125345_d_n9, assign82440_e125345_d_n10, assign82440_e125345_d_n11, assign82440_e125345_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1915 != 0.0)) && (locals.var_guard1916 != 0.0)) {
        let assign82440_e125339: f64 = (locals.var_chi_a - locals.var_chi_b);
        let assign82440_e125342: f64 = (0.2 * locals.var_chi_b);
        let assign82440_e125343: f64 = (assign82440_e125339 + assign82440_e125342);
        (assign82440_e125343, ((locals.var_chi_a_dn0 - locals.var_chi_b_dn0) + (0.2 * locals.var_chi_b_dn0)), ((locals.var_chi_a_dn2 - locals.var_chi_b_dn2) + (0.2 * locals.var_chi_b_dn2)), ((locals.var_chi_a_dn4 - locals.var_chi_b_dn4) + (0.2 * locals.var_chi_b_dn4)), ((locals.var_chi_a_dn5 - locals.var_chi_b_dn5) + (0.2 * locals.var_chi_b_dn5)), ((locals.var_chi_a_dn6 - locals.var_chi_b_dn6) + (0.2 * locals.var_chi_b_dn6)), ((locals.var_chi_a_dn7 - locals.var_chi_b_dn7) + (0.2 * locals.var_chi_b_dn7)), ((locals.var_chi_a_dn8 - locals.var_chi_b_dn8) + (0.2 * locals.var_chi_b_dn8)), ((locals.var_chi_a_dn9 - locals.var_chi_b_dn9) + (0.2 * locals.var_chi_b_dn9)), ((locals.var_chi_a_dn10 - locals.var_chi_b_dn10) + (0.2 * locals.var_chi_b_dn10)), ((locals.var_chi_a_dn11 - locals.var_chi_b_dn11) + (0.2 * locals.var_chi_b_dn11)), ((locals.var_chi_a_dn14 - locals.var_chi_b_dn14) + (0.2 * locals.var_chi_b_dn14)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign82440_e125345;
        locals.var_tmf1_dn0 = assign82440_e125345_d_n0;
        locals.var_tmf1_dn2 = assign82440_e125345_d_n2;
        locals.var_tmf1_dn4 = assign82440_e125345_d_n4;
        locals.var_tmf1_dn5 = assign82440_e125345_d_n5;
        locals.var_tmf1_dn6 = assign82440_e125345_d_n6;
        locals.var_tmf1_dn7 = assign82440_e125345_d_n7;
        locals.var_tmf1_dn8 = assign82440_e125345_d_n8;
        locals.var_tmf1_dn9 = assign82440_e125345_d_n9;
        locals.var_tmf1_dn10 = assign82440_e125345_d_n10;
        locals.var_tmf1_dn11 = assign82440_e125345_d_n11;
        locals.var_tmf1_dn14 = assign82440_e125345_d_n14;

        let (assign82450_e125360, assign82450_e125360_d_n0, assign82450_e125360_d_n2, assign82450_e125360_d_n4, assign82450_e125360_d_n5, assign82450_e125360_d_n6, assign82450_e125360_d_n7, assign82450_e125360_d_n8, assign82450_e125360_d_n9, assign82450_e125360_d_n10, assign82450_e125360_d_n11, assign82450_e125360_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1915 != 0.0)) && (locals.var_guard1916 != 0.0)) {
        let assign82450_e125358: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign82450_e125358, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign82450_e125360;
        locals.var_x2_dn0 = assign82450_e125360_d_n0;
        locals.var_x2_dn2 = assign82450_e125360_d_n2;
        locals.var_x2_dn4 = assign82450_e125360_d_n4;
        locals.var_x2_dn5 = assign82450_e125360_d_n5;
        locals.var_x2_dn6 = assign82450_e125360_d_n6;
        locals.var_x2_dn7 = assign82450_e125360_d_n7;
        locals.var_x2_dn8 = assign82450_e125360_d_n8;
        locals.var_x2_dn9 = assign82450_e125360_d_n9;
        locals.var_x2_dn10 = assign82450_e125360_d_n10;
        locals.var_x2_dn11 = assign82450_e125360_d_n11;
        locals.var_x2_dn14 = assign82450_e125360_d_n14;

        let (assign82460_e125379, assign82460_e125379_d_n0, assign82460_e125379_d_n2, assign82460_e125379_d_n4, assign82460_e125379_d_n5, assign82460_e125379_d_n6, assign82460_e125379_d_n7, assign82460_e125379_d_n8, assign82460_e125379_d_n9, assign82460_e125379_d_n10, assign82460_e125379_d_n11, assign82460_e125379_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1915 != 0.0)) && (locals.var_guard1916 != 0.0)) {
        let assign82460_e125373: f64 = (0.2 * locals.var_chi_b);
        let assign82460_e125376: f64 = (0.2 * locals.var_chi_b);
        let assign82460_e125377: f64 = (assign82460_e125373 * assign82460_e125376);
        (assign82460_e125377, (((0.2 * locals.var_chi_b_dn0) * assign82460_e125376) + (assign82460_e125373 * (0.2 * locals.var_chi_b_dn0))), (((0.2 * locals.var_chi_b_dn2) * assign82460_e125376) + (assign82460_e125373 * (0.2 * locals.var_chi_b_dn2))), (((0.2 * locals.var_chi_b_dn4) * assign82460_e125376) + (assign82460_e125373 * (0.2 * locals.var_chi_b_dn4))), (((0.2 * locals.var_chi_b_dn5) * assign82460_e125376) + (assign82460_e125373 * (0.2 * locals.var_chi_b_dn5))), (((0.2 * locals.var_chi_b_dn6) * assign82460_e125376) + (assign82460_e125373 * (0.2 * locals.var_chi_b_dn6))), (((0.2 * locals.var_chi_b_dn7) * assign82460_e125376) + (assign82460_e125373 * (0.2 * locals.var_chi_b_dn7))), (((0.2 * locals.var_chi_b_dn8) * assign82460_e125376) + (assign82460_e125373 * (0.2 * locals.var_chi_b_dn8))), (((0.2 * locals.var_chi_b_dn9) * assign82460_e125376) + (assign82460_e125373 * (0.2 * locals.var_chi_b_dn9))), (((0.2 * locals.var_chi_b_dn10) * assign82460_e125376) + (assign82460_e125373 * (0.2 * locals.var_chi_b_dn10))), (((0.2 * locals.var_chi_b_dn11) * assign82460_e125376) + (assign82460_e125373 * (0.2 * locals.var_chi_b_dn11))), (((0.2 * locals.var_chi_b_dn14) * assign82460_e125376) + (assign82460_e125373 * (0.2 * locals.var_chi_b_dn14))),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign82460_e125379;
        locals.var_xmax2_dn0 = assign82460_e125379_d_n0;
        locals.var_xmax2_dn2 = assign82460_e125379_d_n2;
        locals.var_xmax2_dn4 = assign82460_e125379_d_n4;
        locals.var_xmax2_dn5 = assign82460_e125379_d_n5;
        locals.var_xmax2_dn6 = assign82460_e125379_d_n6;
        locals.var_xmax2_dn7 = assign82460_e125379_d_n7;
        locals.var_xmax2_dn8 = assign82460_e125379_d_n8;
        locals.var_xmax2_dn9 = assign82460_e125379_d_n9;
        locals.var_xmax2_dn10 = assign82460_e125379_d_n10;
        locals.var_xmax2_dn11 = assign82460_e125379_d_n11;
        locals.var_xmax2_dn14 = assign82460_e125379_d_n14;

        let (assign82470_e125392, assign82470_e125392_d_n0, assign82470_e125392_d_n2, assign82470_e125392_d_n4, assign82470_e125392_d_n5, assign82470_e125392_d_n6, assign82470_e125392_d_n7, assign82470_e125392_d_n8, assign82470_e125392_d_n9, assign82470_e125392_d_n10, assign82470_e125392_d_n11, assign82470_e125392_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1915 != 0.0)) && (locals.var_guard1916 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign82470_e125392;
        locals.var_xp_dn0 = assign82470_e125392_d_n0;
        locals.var_xp_dn2 = assign82470_e125392_d_n2;
        locals.var_xp_dn4 = assign82470_e125392_d_n4;
        locals.var_xp_dn5 = assign82470_e125392_d_n5;
        locals.var_xp_dn6 = assign82470_e125392_d_n6;
        locals.var_xp_dn7 = assign82470_e125392_d_n7;
        locals.var_xp_dn8 = assign82470_e125392_d_n8;
        locals.var_xp_dn9 = assign82470_e125392_d_n9;
        locals.var_xp_dn10 = assign82470_e125392_d_n10;
        locals.var_xp_dn11 = assign82470_e125392_d_n11;
        locals.var_xp_dn14 = assign82470_e125392_d_n14;

        let (assign82480_e125405, assign82480_e125405_d_n0, assign82480_e125405_d_n2, assign82480_e125405_d_n4, assign82480_e125405_d_n5, assign82480_e125405_d_n6, assign82480_e125405_d_n7, assign82480_e125405_d_n8, assign82480_e125405_d_n9, assign82480_e125405_d_n10, assign82480_e125405_d_n11, assign82480_e125405_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1915 != 0.0)) && (locals.var_guard1916 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign82480_e125405;
        locals.var_xmp_dn0 = assign82480_e125405_d_n0;
        locals.var_xmp_dn2 = assign82480_e125405_d_n2;
        locals.var_xmp_dn4 = assign82480_e125405_d_n4;
        locals.var_xmp_dn5 = assign82480_e125405_d_n5;
        locals.var_xmp_dn6 = assign82480_e125405_d_n6;
        locals.var_xmp_dn7 = assign82480_e125405_d_n7;
        locals.var_xmp_dn8 = assign82480_e125405_d_n8;
        locals.var_xmp_dn9 = assign82480_e125405_d_n9;
        locals.var_xmp_dn10 = assign82480_e125405_d_n10;
        locals.var_xmp_dn11 = assign82480_e125405_d_n11;
        locals.var_xmp_dn14 = assign82480_e125405_d_n14;

        let (assign82490_e125418,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1915 != 0.0)) && (locals.var_guard1916 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign82490_e125418;

        let (assign82500_e125431,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1915 != 0.0)) && (locals.var_guard1916 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign82500_e125431;

        let (assign82510_e125444, assign82510_e125444_d_n0, assign82510_e125444_d_n2, assign82510_e125444_d_n4, assign82510_e125444_d_n5, assign82510_e125444_d_n6, assign82510_e125444_d_n7, assign82510_e125444_d_n8, assign82510_e125444_d_n9, assign82510_e125444_d_n10, assign82510_e125444_d_n11, assign82510_e125444_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1915 != 0.0)) && (locals.var_guard1916 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign82510_e125444;
        locals.var_arg_dn0 = assign82510_e125444_d_n0;
        locals.var_arg_dn2 = assign82510_e125444_d_n2;
        locals.var_arg_dn4 = assign82510_e125444_d_n4;
        locals.var_arg_dn5 = assign82510_e125444_d_n5;
        locals.var_arg_dn6 = assign82510_e125444_d_n6;
        locals.var_arg_dn7 = assign82510_e125444_d_n7;
        locals.var_arg_dn8 = assign82510_e125444_d_n8;
        locals.var_arg_dn9 = assign82510_e125444_d_n9;
        locals.var_arg_dn10 = assign82510_e125444_d_n10;
        locals.var_arg_dn11 = assign82510_e125444_d_n11;
        locals.var_arg_dn14 = assign82510_e125444_d_n14;

        let (assign82520_e125457, assign82520_e125457_d_n0, assign82520_e125457_d_n2, assign82520_e125457_d_n4, assign82520_e125457_d_n5, assign82520_e125457_d_n6, assign82520_e125457_d_n7, assign82520_e125457_d_n8, assign82520_e125457_d_n9, assign82520_e125457_d_n10, assign82520_e125457_d_n11, assign82520_e125457_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1915 != 0.0)) && (locals.var_guard1916 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign82520_e125457;
        locals.var_dnm_dn0 = assign82520_e125457_d_n0;
        locals.var_dnm_dn2 = assign82520_e125457_d_n2;
        locals.var_dnm_dn4 = assign82520_e125457_d_n4;
        locals.var_dnm_dn5 = assign82520_e125457_d_n5;
        locals.var_dnm_dn6 = assign82520_e125457_d_n6;
        locals.var_dnm_dn7 = assign82520_e125457_d_n7;
        locals.var_dnm_dn8 = assign82520_e125457_d_n8;
        locals.var_dnm_dn9 = assign82520_e125457_d_n9;
        locals.var_dnm_dn10 = assign82520_e125457_d_n10;
        locals.var_dnm_dn11 = assign82520_e125457_d_n11;
        locals.var_dnm_dn14 = assign82520_e125457_d_n14;

        let (assign82530_e125472, assign82530_e125472_d_n0, assign82530_e125472_d_n2, assign82530_e125472_d_n4, assign82530_e125472_d_n5, assign82530_e125472_d_n6, assign82530_e125472_d_n7, assign82530_e125472_d_n8, assign82530_e125472_d_n9, assign82530_e125472_d_n10, assign82530_e125472_d_n11, assign82530_e125472_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1915 != 0.0)) && (locals.var_guard1916 != 0.0)) {
        let assign82530_e125470: f64 = (locals.var_xp * locals.var_x2);
        (assign82530_e125470, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign82530_e125472;
        locals.var_xp_dn0 = assign82530_e125472_d_n0;
        locals.var_xp_dn2 = assign82530_e125472_d_n2;
        locals.var_xp_dn4 = assign82530_e125472_d_n4;
        locals.var_xp_dn5 = assign82530_e125472_d_n5;
        locals.var_xp_dn6 = assign82530_e125472_d_n6;
        locals.var_xp_dn7 = assign82530_e125472_d_n7;
        locals.var_xp_dn8 = assign82530_e125472_d_n8;
        locals.var_xp_dn9 = assign82530_e125472_d_n9;
        locals.var_xp_dn10 = assign82530_e125472_d_n10;
        locals.var_xp_dn11 = assign82530_e125472_d_n11;
        locals.var_xp_dn14 = assign82530_e125472_d_n14;

        let (assign82540_e125487, assign82540_e125487_d_n0, assign82540_e125487_d_n2, assign82540_e125487_d_n4, assign82540_e125487_d_n5, assign82540_e125487_d_n6, assign82540_e125487_d_n7, assign82540_e125487_d_n8, assign82540_e125487_d_n9, assign82540_e125487_d_n10, assign82540_e125487_d_n11, assign82540_e125487_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1915 != 0.0)) && (locals.var_guard1916 != 0.0)) {
        let assign82540_e125485: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign82540_e125485, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign82540_e125487;
        locals.var_xmp_dn0 = assign82540_e125487_d_n0;
        locals.var_xmp_dn2 = assign82540_e125487_d_n2;
        locals.var_xmp_dn4 = assign82540_e125487_d_n4;
        locals.var_xmp_dn5 = assign82540_e125487_d_n5;
        locals.var_xmp_dn6 = assign82540_e125487_d_n6;
        locals.var_xmp_dn7 = assign82540_e125487_d_n7;
        locals.var_xmp_dn8 = assign82540_e125487_d_n8;
        locals.var_xmp_dn9 = assign82540_e125487_d_n9;
        locals.var_xmp_dn10 = assign82540_e125487_d_n10;
        locals.var_xmp_dn11 = assign82540_e125487_d_n11;
        locals.var_xmp_dn14 = assign82540_e125487_d_n14;

        let (assign82550_e125502, assign82550_e125502_d_n0, assign82550_e125502_d_n2, assign82550_e125502_d_n4, assign82550_e125502_d_n5, assign82550_e125502_d_n6, assign82550_e125502_d_n7, assign82550_e125502_d_n8, assign82550_e125502_d_n9, assign82550_e125502_d_n10, assign82550_e125502_d_n11, assign82550_e125502_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1915 != 0.0)) && (locals.var_guard1916 != 0.0)) {
        let assign82550_e125500: f64 = (locals.var_xp * locals.var_x2);
        (assign82550_e125500, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign82550_e125502;
        locals.var_xp_dn0 = assign82550_e125502_d_n0;
        locals.var_xp_dn2 = assign82550_e125502_d_n2;
        locals.var_xp_dn4 = assign82550_e125502_d_n4;
        locals.var_xp_dn5 = assign82550_e125502_d_n5;
        locals.var_xp_dn6 = assign82550_e125502_d_n6;
        locals.var_xp_dn7 = assign82550_e125502_d_n7;
        locals.var_xp_dn8 = assign82550_e125502_d_n8;
        locals.var_xp_dn9 = assign82550_e125502_d_n9;
        locals.var_xp_dn10 = assign82550_e125502_d_n10;
        locals.var_xp_dn11 = assign82550_e125502_d_n11;
        locals.var_xp_dn14 = assign82550_e125502_d_n14;

    }

    pub(super) fn stamp_transient_block_299(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign82560_e125517, assign82560_e125517_d_n0, assign82560_e125517_d_n2, assign82560_e125517_d_n4, assign82560_e125517_d_n5, assign82560_e125517_d_n6, assign82560_e125517_d_n7, assign82560_e125517_d_n8, assign82560_e125517_d_n9, assign82560_e125517_d_n10, assign82560_e125517_d_n11, assign82560_e125517_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1915 != 0.0)) && (locals.var_guard1916 != 0.0)) {
        let assign82560_e125515: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign82560_e125515, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign82560_e125517;
        locals.var_xmp_dn0 = assign82560_e125517_d_n0;
        locals.var_xmp_dn2 = assign82560_e125517_d_n2;
        locals.var_xmp_dn4 = assign82560_e125517_d_n4;
        locals.var_xmp_dn5 = assign82560_e125517_d_n5;
        locals.var_xmp_dn6 = assign82560_e125517_d_n6;
        locals.var_xmp_dn7 = assign82560_e125517_d_n7;
        locals.var_xmp_dn8 = assign82560_e125517_d_n8;
        locals.var_xmp_dn9 = assign82560_e125517_d_n9;
        locals.var_xmp_dn10 = assign82560_e125517_d_n10;
        locals.var_xmp_dn11 = assign82560_e125517_d_n11;
        locals.var_xmp_dn14 = assign82560_e125517_d_n14;

        let (assign82570_e125532, assign82570_e125532_d_n0, assign82570_e125532_d_n2, assign82570_e125532_d_n4, assign82570_e125532_d_n5, assign82570_e125532_d_n6, assign82570_e125532_d_n7, assign82570_e125532_d_n8, assign82570_e125532_d_n9, assign82570_e125532_d_n10, assign82570_e125532_d_n11, assign82570_e125532_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1915 != 0.0)) && (locals.var_guard1916 != 0.0)) {
        let assign82570_e125530: f64 = (locals.var_xp + locals.var_xmp);
        (assign82570_e125530, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign82570_e125532;
        locals.var_arg_dn0 = assign82570_e125532_d_n0;
        locals.var_arg_dn2 = assign82570_e125532_d_n2;
        locals.var_arg_dn4 = assign82570_e125532_d_n4;
        locals.var_arg_dn5 = assign82570_e125532_d_n5;
        locals.var_arg_dn6 = assign82570_e125532_d_n6;
        locals.var_arg_dn7 = assign82570_e125532_d_n7;
        locals.var_arg_dn8 = assign82570_e125532_d_n8;
        locals.var_arg_dn9 = assign82570_e125532_d_n9;
        locals.var_arg_dn10 = assign82570_e125532_d_n10;
        locals.var_arg_dn11 = assign82570_e125532_d_n11;
        locals.var_arg_dn14 = assign82570_e125532_d_n14;

        let (assign82580_e125545, assign82580_e125545_d_n0, assign82580_e125545_d_n2, assign82580_e125545_d_n4, assign82580_e125545_d_n5, assign82580_e125545_d_n6, assign82580_e125545_d_n7, assign82580_e125545_d_n8, assign82580_e125545_d_n9, assign82580_e125545_d_n10, assign82580_e125545_d_n11, assign82580_e125545_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1915 != 0.0)) && (locals.var_guard1916 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign82580_e125545;
        locals.var_dnm_dn0 = assign82580_e125545_d_n0;
        locals.var_dnm_dn2 = assign82580_e125545_d_n2;
        locals.var_dnm_dn4 = assign82580_e125545_d_n4;
        locals.var_dnm_dn5 = assign82580_e125545_d_n5;
        locals.var_dnm_dn6 = assign82580_e125545_d_n6;
        locals.var_dnm_dn7 = assign82580_e125545_d_n7;
        locals.var_dnm_dn8 = assign82580_e125545_d_n8;
        locals.var_dnm_dn9 = assign82580_e125545_d_n9;
        locals.var_dnm_dn10 = assign82580_e125545_d_n10;
        locals.var_dnm_dn11 = assign82580_e125545_d_n11;
        locals.var_dnm_dn14 = assign82580_e125545_d_n14;

        let assign82590_e125560: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1917 = assign82590_e125560;

        let assign82600_e125563: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1918 = assign82600_e125563;

        let (assign82610_e125580,) = {
    if (((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1915 != 0.0)) && (locals.var_guard1916 != 0.0)) && (locals.var_guard1917 != 0.0)) && (locals.var_guard1918 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign82610_e125580;

        let assign82620_e125583: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1919 = assign82620_e125583;

        let (assign82630_e125603,) = {
    if ((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1915 != 0.0)) && (locals.var_guard1916 != 0.0)) && (locals.var_guard1917 != 0.0)) && (locals.var_guard1918 == 0.0)) && (locals.var_guard1919 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign82630_e125603;

        let assign82640_e125606: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1920 = assign82640_e125606;

        let (assign82650_e125629,) = {
    if (((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1915 != 0.0)) && (locals.var_guard1916 != 0.0)) && (locals.var_guard1917 != 0.0)) && (locals.var_guard1918 == 0.0)) && (locals.var_guard1919 == 0.0)) && (locals.var_guard1920 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign82650_e125629;

        let assign82660_e125632: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1921 = assign82660_e125632;

        let (assign82670_e125658,) = {
    if ((((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1915 != 0.0)) && (locals.var_guard1916 != 0.0)) && (locals.var_guard1917 != 0.0)) && (locals.var_guard1918 == 0.0)) && (locals.var_guard1919 == 0.0)) && (locals.var_guard1920 == 0.0)) && (locals.var_guard1921 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign82670_e125658;

        let (assign82680_e125673,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1915 != 0.0)) && (locals.var_guard1916 != 0.0)) && (locals.var_guard1917 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign82680_e125673;

        let mut assign82690_loop_guard: usize = 0;
        while {
            let assign82690_cond_e125689: f64 = if (((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1915 != 0.0)) && (locals.var_guard1916 != 0.0)) && (locals.var_guard1917 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign82690_cond_e125689 != 0.0
        } {
            assign82690_loop_guard += 1;
            assert!(assign82690_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign82690_body0_e125705, assign82690_body0_e125705_d_n0, assign82690_body0_e125705_d_n2, assign82690_body0_e125705_d_n4, assign82690_body0_e125705_d_n5, assign82690_body0_e125705_d_n6, assign82690_body0_e125705_d_n7, assign82690_body0_e125705_d_n8, assign82690_body0_e125705_d_n9, assign82690_body0_e125705_d_n10, assign82690_body0_e125705_d_n11, assign82690_body0_e125705_d_n14,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1915 != 0.0)) && (locals.var_guard1916 != 0.0)) && (locals.var_guard1917 != 0.0)) {
        let assign82690_body0_e125703: f64 = (locals.var_dnm).sqrt();
        (assign82690_body0_e125703, (locals.var_dnm_dn0 / (2.0 * assign82690_body0_e125703)), (locals.var_dnm_dn2 / (2.0 * assign82690_body0_e125703)), (locals.var_dnm_dn4 / (2.0 * assign82690_body0_e125703)), (locals.var_dnm_dn5 / (2.0 * assign82690_body0_e125703)), (locals.var_dnm_dn6 / (2.0 * assign82690_body0_e125703)), (locals.var_dnm_dn7 / (2.0 * assign82690_body0_e125703)), (locals.var_dnm_dn8 / (2.0 * assign82690_body0_e125703)), (locals.var_dnm_dn9 / (2.0 * assign82690_body0_e125703)), (locals.var_dnm_dn10 / (2.0 * assign82690_body0_e125703)), (locals.var_dnm_dn11 / (2.0 * assign82690_body0_e125703)), (locals.var_dnm_dn14 / (2.0 * assign82690_body0_e125703)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign82690_body0_e125705;
            locals.var_dnm_dn0 = assign82690_body0_e125705_d_n0;
            locals.var_dnm_dn2 = assign82690_body0_e125705_d_n2;
            locals.var_dnm_dn4 = assign82690_body0_e125705_d_n4;
            locals.var_dnm_dn5 = assign82690_body0_e125705_d_n5;
            locals.var_dnm_dn6 = assign82690_body0_e125705_d_n6;
            locals.var_dnm_dn7 = assign82690_body0_e125705_d_n7;
            locals.var_dnm_dn8 = assign82690_body0_e125705_d_n8;
            locals.var_dnm_dn9 = assign82690_body0_e125705_d_n9;
            locals.var_dnm_dn10 = assign82690_body0_e125705_d_n10;
            locals.var_dnm_dn11 = assign82690_body0_e125705_d_n11;
            locals.var_dnm_dn14 = assign82690_body0_e125705_d_n14;
            let (assign82690_body1_e125722,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1915 != 0.0)) && (locals.var_guard1916 != 0.0)) && (locals.var_guard1917 != 0.0)) {
        let assign82690_body1_e125720: f64 = (locals.var_m0 + 1.0);
        (assign82690_body1_e125720,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign82690_body1_e125722;
        }

        let (assign82700_e125749, assign82700_e125749_d_n0, assign82700_e125749_d_n2, assign82700_e125749_d_n4, assign82700_e125749_d_n5, assign82700_e125749_d_n6, assign82700_e125749_d_n7, assign82700_e125749_d_n8, assign82700_e125749_d_n9, assign82700_e125749_d_n10, assign82700_e125749_d_n11, assign82700_e125749_d_n14,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1915 != 0.0)) && (locals.var_guard1916 != 0.0)) && (locals.var_guard1917 == 0.0)) {
        let (assign82700_e125747, assign82700_e125747_d_n0, assign82700_e125747_d_n2, assign82700_e125747_d_n4, assign82700_e125747_d_n5, assign82700_e125747_d_n6, assign82700_e125747_d_n7, assign82700_e125747_d_n8, assign82700_e125747_d_n9, assign82700_e125747_d_n10, assign82700_e125747_d_n11, assign82700_e125747_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign82700_e125744: f64 = (2.0 * 2.0);
                let assign82700_e125745: f64 = (1.0 / assign82700_e125744);
                let assign82700_e125746: f64 = (locals.var_dnm).powf(assign82700_e125745);
                (assign82700_e125746, if 0.0 == 0.0 && ((assign82700_e125745) as f64).is_finite() && ((assign82700_e125745) as f64).fract() == 0.0 { if assign82700_e125745 == 0.0 { 0.0 } else { (assign82700_e125745 * ((locals.var_dnm).powf(assign82700_e125745 - 1.0) * locals.var_dnm_dn0)) } } else { (assign82700_e125746 * (assign82700_e125745 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign82700_e125745) as f64).is_finite() && ((assign82700_e125745) as f64).fract() == 0.0 { if assign82700_e125745 == 0.0 { 0.0 } else { (assign82700_e125745 * ((locals.var_dnm).powf(assign82700_e125745 - 1.0) * locals.var_dnm_dn2)) } } else { (assign82700_e125746 * (assign82700_e125745 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign82700_e125745) as f64).is_finite() && ((assign82700_e125745) as f64).fract() == 0.0 { if assign82700_e125745 == 0.0 { 0.0 } else { (assign82700_e125745 * ((locals.var_dnm).powf(assign82700_e125745 - 1.0) * locals.var_dnm_dn4)) } } else { (assign82700_e125746 * (assign82700_e125745 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign82700_e125745) as f64).is_finite() && ((assign82700_e125745) as f64).fract() == 0.0 { if assign82700_e125745 == 0.0 { 0.0 } else { (assign82700_e125745 * ((locals.var_dnm).powf(assign82700_e125745 - 1.0) * locals.var_dnm_dn5)) } } else { (assign82700_e125746 * (assign82700_e125745 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign82700_e125745) as f64).is_finite() && ((assign82700_e125745) as f64).fract() == 0.0 { if assign82700_e125745 == 0.0 { 0.0 } else { (assign82700_e125745 * ((locals.var_dnm).powf(assign82700_e125745 - 1.0) * locals.var_dnm_dn6)) } } else { (assign82700_e125746 * (assign82700_e125745 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign82700_e125745) as f64).is_finite() && ((assign82700_e125745) as f64).fract() == 0.0 { if assign82700_e125745 == 0.0 { 0.0 } else { (assign82700_e125745 * ((locals.var_dnm).powf(assign82700_e125745 - 1.0) * locals.var_dnm_dn7)) } } else { (assign82700_e125746 * (assign82700_e125745 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign82700_e125745) as f64).is_finite() && ((assign82700_e125745) as f64).fract() == 0.0 { if assign82700_e125745 == 0.0 { 0.0 } else { (assign82700_e125745 * ((locals.var_dnm).powf(assign82700_e125745 - 1.0) * locals.var_dnm_dn8)) } } else { (assign82700_e125746 * (assign82700_e125745 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign82700_e125745) as f64).is_finite() && ((assign82700_e125745) as f64).fract() == 0.0 { if assign82700_e125745 == 0.0 { 0.0 } else { (assign82700_e125745 * ((locals.var_dnm).powf(assign82700_e125745 - 1.0) * locals.var_dnm_dn9)) } } else { (assign82700_e125746 * (assign82700_e125745 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign82700_e125745) as f64).is_finite() && ((assign82700_e125745) as f64).fract() == 0.0 { if assign82700_e125745 == 0.0 { 0.0 } else { (assign82700_e125745 * ((locals.var_dnm).powf(assign82700_e125745 - 1.0) * locals.var_dnm_dn10)) } } else { (assign82700_e125746 * (assign82700_e125745 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign82700_e125745) as f64).is_finite() && ((assign82700_e125745) as f64).fract() == 0.0 { if assign82700_e125745 == 0.0 { 0.0 } else { (assign82700_e125745 * ((locals.var_dnm).powf(assign82700_e125745 - 1.0) * locals.var_dnm_dn11)) } } else { (assign82700_e125746 * (assign82700_e125745 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign82700_e125745) as f64).is_finite() && ((assign82700_e125745) as f64).fract() == 0.0 { if assign82700_e125745 == 0.0 { 0.0 } else { (assign82700_e125745 * ((locals.var_dnm).powf(assign82700_e125745 - 1.0) * locals.var_dnm_dn14)) } } else { (assign82700_e125746 * (assign82700_e125745 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign82700_e125747, assign82700_e125747_d_n0, assign82700_e125747_d_n2, assign82700_e125747_d_n4, assign82700_e125747_d_n5, assign82700_e125747_d_n6, assign82700_e125747_d_n7, assign82700_e125747_d_n8, assign82700_e125747_d_n9, assign82700_e125747_d_n10, assign82700_e125747_d_n11, assign82700_e125747_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign82700_e125749;
        locals.var_dnm_dn0 = assign82700_e125749_d_n0;
        locals.var_dnm_dn2 = assign82700_e125749_d_n2;
        locals.var_dnm_dn4 = assign82700_e125749_d_n4;
        locals.var_dnm_dn5 = assign82700_e125749_d_n5;
        locals.var_dnm_dn6 = assign82700_e125749_d_n6;
        locals.var_dnm_dn7 = assign82700_e125749_d_n7;
        locals.var_dnm_dn8 = assign82700_e125749_d_n8;
        locals.var_dnm_dn9 = assign82700_e125749_d_n9;
        locals.var_dnm_dn10 = assign82700_e125749_d_n10;
        locals.var_dnm_dn11 = assign82700_e125749_d_n11;
        locals.var_dnm_dn14 = assign82700_e125749_d_n14;

        let (assign82710_e125764, assign82710_e125764_d_n0, assign82710_e125764_d_n2, assign82710_e125764_d_n4, assign82710_e125764_d_n5, assign82710_e125764_d_n6, assign82710_e125764_d_n7, assign82710_e125764_d_n8, assign82710_e125764_d_n9, assign82710_e125764_d_n10, assign82710_e125764_d_n11, assign82710_e125764_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1915 != 0.0)) && (locals.var_guard1916 != 0.0)) {
        let assign82710_e125762: f64 = (1.0 / locals.var_dnm);
        (assign82710_e125762, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign82710_e125764;
        locals.var_dnm_dn0 = assign82710_e125764_d_n0;
        locals.var_dnm_dn2 = assign82710_e125764_d_n2;
        locals.var_dnm_dn4 = assign82710_e125764_d_n4;
        locals.var_dnm_dn5 = assign82710_e125764_d_n5;
        locals.var_dnm_dn6 = assign82710_e125764_d_n6;
        locals.var_dnm_dn7 = assign82710_e125764_d_n7;
        locals.var_dnm_dn8 = assign82710_e125764_d_n8;
        locals.var_dnm_dn9 = assign82710_e125764_d_n9;
        locals.var_dnm_dn10 = assign82710_e125764_d_n10;
        locals.var_dnm_dn11 = assign82710_e125764_d_n11;
        locals.var_dnm_dn14 = assign82710_e125764_d_n14;

        let (assign82720_e125783, assign82720_e125783_d_n0, assign82720_e125783_d_n2, assign82720_e125783_d_n4, assign82720_e125783_d_n5, assign82720_e125783_d_n6, assign82720_e125783_d_n7, assign82720_e125783_d_n8, assign82720_e125783_d_n9, assign82720_e125783_d_n10, assign82720_e125783_d_n11, assign82720_e125783_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1915 != 0.0)) && (locals.var_guard1916 != 0.0)) {
        let assign82720_e125778: f64 = (0.2 * locals.var_chi_b);
        let assign82720_e125779: f64 = (locals.var_tmf1 * assign82720_e125778);
        let assign82720_e125781: f64 = (assign82720_e125779 * locals.var_dnm);
        (assign82720_e125781, ((((locals.var_tmf1_dn0 * assign82720_e125778) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn0))) * locals.var_dnm) + (assign82720_e125779 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * assign82720_e125778) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn2))) * locals.var_dnm) + (assign82720_e125779 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn4 * assign82720_e125778) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn4))) * locals.var_dnm) + (assign82720_e125779 * locals.var_dnm_dn4)), ((((locals.var_tmf1_dn5 * assign82720_e125778) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn5))) * locals.var_dnm) + (assign82720_e125779 * locals.var_dnm_dn5)), ((((locals.var_tmf1_dn6 * assign82720_e125778) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn6))) * locals.var_dnm) + (assign82720_e125779 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * assign82720_e125778) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn7))) * locals.var_dnm) + (assign82720_e125779 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn8 * assign82720_e125778) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn8))) * locals.var_dnm) + (assign82720_e125779 * locals.var_dnm_dn8)), ((((locals.var_tmf1_dn9 * assign82720_e125778) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn9))) * locals.var_dnm) + (assign82720_e125779 * locals.var_dnm_dn9)), ((((locals.var_tmf1_dn10 * assign82720_e125778) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn10))) * locals.var_dnm) + (assign82720_e125779 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn11 * assign82720_e125778) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn11))) * locals.var_dnm) + (assign82720_e125779 * locals.var_dnm_dn11)), ((((locals.var_tmf1_dn14 * assign82720_e125778) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn14))) * locals.var_dnm) + (assign82720_e125779 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign82720_e125783;
        locals.var_tmf0_dn0 = assign82720_e125783_d_n0;
        locals.var_tmf0_dn2 = assign82720_e125783_d_n2;
        locals.var_tmf0_dn4 = assign82720_e125783_d_n4;
        locals.var_tmf0_dn5 = assign82720_e125783_d_n5;
        locals.var_tmf0_dn6 = assign82720_e125783_d_n6;
        locals.var_tmf0_dn7 = assign82720_e125783_d_n7;
        locals.var_tmf0_dn8 = assign82720_e125783_d_n8;
        locals.var_tmf0_dn9 = assign82720_e125783_d_n9;
        locals.var_tmf0_dn10 = assign82720_e125783_d_n10;
        locals.var_tmf0_dn11 = assign82720_e125783_d_n11;
        locals.var_tmf0_dn14 = assign82720_e125783_d_n14;

        let (assign82730_e125804, assign82730_e125804_d_n0, assign82730_e125804_d_n2, assign82730_e125804_d_n4, assign82730_e125804_d_n5, assign82730_e125804_d_n6, assign82730_e125804_d_n7, assign82730_e125804_d_n8, assign82730_e125804_d_n9, assign82730_e125804_d_n10, assign82730_e125804_d_n11, assign82730_e125804_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1915 != 0.0)) && (locals.var_guard1916 != 0.0)) {
        let assign82730_e125796: f64 = (0.2 * locals.var_chi_b);
        let assign82730_e125798: f64 = (assign82730_e125796 * locals.var_xmp);
        let assign82730_e125800: f64 = (assign82730_e125798 * locals.var_dnm);
        let assign82730_e125802: f64 = (assign82730_e125800 / locals.var_arg);
        (assign82730_e125802, ((((((((0.2 * locals.var_chi_b_dn0) * locals.var_xmp) + (assign82730_e125796 * locals.var_xmp_dn0)) * locals.var_dnm) + (assign82730_e125798 * locals.var_dnm_dn0)) * locals.var_arg) - (assign82730_e125800 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn2) * locals.var_xmp) + (assign82730_e125796 * locals.var_xmp_dn2)) * locals.var_dnm) + (assign82730_e125798 * locals.var_dnm_dn2)) * locals.var_arg) - (assign82730_e125800 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn4) * locals.var_xmp) + (assign82730_e125796 * locals.var_xmp_dn4)) * locals.var_dnm) + (assign82730_e125798 * locals.var_dnm_dn4)) * locals.var_arg) - (assign82730_e125800 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn5) * locals.var_xmp) + (assign82730_e125796 * locals.var_xmp_dn5)) * locals.var_dnm) + (assign82730_e125798 * locals.var_dnm_dn5)) * locals.var_arg) - (assign82730_e125800 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn6) * locals.var_xmp) + (assign82730_e125796 * locals.var_xmp_dn6)) * locals.var_dnm) + (assign82730_e125798 * locals.var_dnm_dn6)) * locals.var_arg) - (assign82730_e125800 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn7) * locals.var_xmp) + (assign82730_e125796 * locals.var_xmp_dn7)) * locals.var_dnm) + (assign82730_e125798 * locals.var_dnm_dn7)) * locals.var_arg) - (assign82730_e125800 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn8) * locals.var_xmp) + (assign82730_e125796 * locals.var_xmp_dn8)) * locals.var_dnm) + (assign82730_e125798 * locals.var_dnm_dn8)) * locals.var_arg) - (assign82730_e125800 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn9) * locals.var_xmp) + (assign82730_e125796 * locals.var_xmp_dn9)) * locals.var_dnm) + (assign82730_e125798 * locals.var_dnm_dn9)) * locals.var_arg) - (assign82730_e125800 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn10) * locals.var_xmp) + (assign82730_e125796 * locals.var_xmp_dn10)) * locals.var_dnm) + (assign82730_e125798 * locals.var_dnm_dn10)) * locals.var_arg) - (assign82730_e125800 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn11) * locals.var_xmp) + (assign82730_e125796 * locals.var_xmp_dn11)) * locals.var_dnm) + (assign82730_e125798 * locals.var_dnm_dn11)) * locals.var_arg) - (assign82730_e125800 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn14) * locals.var_xmp) + (assign82730_e125796 * locals.var_xmp_dn14)) * locals.var_dnm) + (assign82730_e125798 * locals.var_dnm_dn14)) * locals.var_arg) - (assign82730_e125800 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign82730_e125804;
        locals.var_t1_dn0 = assign82730_e125804_d_n0;
        locals.var_t1_dn2 = assign82730_e125804_d_n2;
        locals.var_t1_dn4 = assign82730_e125804_d_n4;
        locals.var_t1_dn5 = assign82730_e125804_d_n5;
        locals.var_t1_dn6 = assign82730_e125804_d_n6;
        locals.var_t1_dn7 = assign82730_e125804_d_n7;
        locals.var_t1_dn8 = assign82730_e125804_d_n8;
        locals.var_t1_dn9 = assign82730_e125804_d_n9;
        locals.var_t1_dn10 = assign82730_e125804_d_n10;
        locals.var_t1_dn11 = assign82730_e125804_d_n11;
        locals.var_t1_dn14 = assign82730_e125804_d_n14;

        let (assign82740_e125823, assign82740_e125823_d_n0, assign82740_e125823_d_n2, assign82740_e125823_d_n4, assign82740_e125823_d_n5, assign82740_e125823_d_n6, assign82740_e125823_d_n7, assign82740_e125823_d_n8, assign82740_e125823_d_n9, assign82740_e125823_d_n10, assign82740_e125823_d_n11, assign82740_e125823_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1915 != 0.0)) && (locals.var_guard1916 != 0.0)) {
        let assign82740_e125818: f64 = (0.2 * locals.var_chi_b);
        let assign82740_e125819: f64 = (locals.var_chi_b - assign82740_e125818);
        let assign82740_e125821: f64 = (assign82740_e125819 + locals.var_tmf0);
        (assign82740_e125821, ((locals.var_chi_b_dn0 - (0.2 * locals.var_chi_b_dn0)) + locals.var_tmf0_dn0), ((locals.var_chi_b_dn2 - (0.2 * locals.var_chi_b_dn2)) + locals.var_tmf0_dn2), ((locals.var_chi_b_dn4 - (0.2 * locals.var_chi_b_dn4)) + locals.var_tmf0_dn4), ((locals.var_chi_b_dn5 - (0.2 * locals.var_chi_b_dn5)) + locals.var_tmf0_dn5), ((locals.var_chi_b_dn6 - (0.2 * locals.var_chi_b_dn6)) + locals.var_tmf0_dn6), ((locals.var_chi_b_dn7 - (0.2 * locals.var_chi_b_dn7)) + locals.var_tmf0_dn7), ((locals.var_chi_b_dn8 - (0.2 * locals.var_chi_b_dn8)) + locals.var_tmf0_dn8), ((locals.var_chi_b_dn9 - (0.2 * locals.var_chi_b_dn9)) + locals.var_tmf0_dn9), ((locals.var_chi_b_dn10 - (0.2 * locals.var_chi_b_dn10)) + locals.var_tmf0_dn10), ((locals.var_chi_b_dn11 - (0.2 * locals.var_chi_b_dn11)) + locals.var_tmf0_dn11), ((locals.var_chi_b_dn14 - (0.2 * locals.var_chi_b_dn14)) + locals.var_tmf0_dn14),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
        locals.var_chi = assign82740_e125823;
        locals.var_chi_dn0 = assign82740_e125823_d_n0;
        locals.var_chi_dn2 = assign82740_e125823_d_n2;
        locals.var_chi_dn4 = assign82740_e125823_d_n4;
        locals.var_chi_dn5 = assign82740_e125823_d_n5;
        locals.var_chi_dn6 = assign82740_e125823_d_n6;
        locals.var_chi_dn7 = assign82740_e125823_d_n7;
        locals.var_chi_dn8 = assign82740_e125823_d_n8;
        locals.var_chi_dn9 = assign82740_e125823_d_n9;
        locals.var_chi_dn10 = assign82740_e125823_d_n10;
        locals.var_chi_dn11 = assign82740_e125823_d_n11;
        locals.var_chi_dn14 = assign82740_e125823_d_n14;

        let (assign82750_e125836, assign82750_e125836_d_n0, assign82750_e125836_d_n2, assign82750_e125836_d_n4, assign82750_e125836_d_n5, assign82750_e125836_d_n6, assign82750_e125836_d_n7, assign82750_e125836_d_n8, assign82750_e125836_d_n9, assign82750_e125836_d_n10, assign82750_e125836_d_n11, assign82750_e125836_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1915 != 0.0)) && (locals.var_guard1916 != 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign82750_e125836;
        locals.var_t1_dn0 = assign82750_e125836_d_n0;
        locals.var_t1_dn2 = assign82750_e125836_d_n2;
        locals.var_t1_dn4 = assign82750_e125836_d_n4;
        locals.var_t1_dn5 = assign82750_e125836_d_n5;
        locals.var_t1_dn6 = assign82750_e125836_d_n6;
        locals.var_t1_dn7 = assign82750_e125836_d_n7;
        locals.var_t1_dn8 = assign82750_e125836_d_n8;
        locals.var_t1_dn9 = assign82750_e125836_d_n9;
        locals.var_t1_dn10 = assign82750_e125836_d_n10;
        locals.var_t1_dn11 = assign82750_e125836_d_n11;
        locals.var_t1_dn14 = assign82750_e125836_d_n14;

        let (assign82760_e125850, assign82760_e125850_d_n0, assign82760_e125850_d_n2, assign82760_e125850_d_n4, assign82760_e125850_d_n5, assign82760_e125850_d_n6, assign82760_e125850_d_n7, assign82760_e125850_d_n8, assign82760_e125850_d_n9, assign82760_e125850_d_n10, assign82760_e125850_d_n11, assign82760_e125850_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1915 != 0.0)) && (locals.var_guard1916 == 0.0)) {
        (locals.var_chi_a, locals.var_chi_a_dn0, locals.var_chi_a_dn2, locals.var_chi_a_dn4, locals.var_chi_a_dn5, locals.var_chi_a_dn6, locals.var_chi_a_dn7, locals.var_chi_a_dn8, locals.var_chi_a_dn9, locals.var_chi_a_dn10, locals.var_chi_a_dn11, locals.var_chi_a_dn14,)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
        locals.var_chi = assign82760_e125850;
        locals.var_chi_dn0 = assign82760_e125850_d_n0;
        locals.var_chi_dn2 = assign82760_e125850_d_n2;
        locals.var_chi_dn4 = assign82760_e125850_d_n4;
        locals.var_chi_dn5 = assign82760_e125850_d_n5;
        locals.var_chi_dn6 = assign82760_e125850_d_n6;
        locals.var_chi_dn7 = assign82760_e125850_d_n7;
        locals.var_chi_dn8 = assign82760_e125850_d_n8;
        locals.var_chi_dn9 = assign82760_e125850_d_n9;
        locals.var_chi_dn10 = assign82760_e125850_d_n10;
        locals.var_chi_dn11 = assign82760_e125850_d_n11;
        locals.var_chi_dn14 = assign82760_e125850_d_n14;

        let (assign82770_e125864, assign82770_e125864_d_n0, assign82770_e125864_d_n2, assign82770_e125864_d_n4, assign82770_e125864_d_n5, assign82770_e125864_d_n6, assign82770_e125864_d_n7, assign82770_e125864_d_n8, assign82770_e125864_d_n9, assign82770_e125864_d_n10, assign82770_e125864_d_n11, assign82770_e125864_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1915 != 0.0)) && (locals.var_guard1916 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign82770_e125864;
        locals.var_t1_dn0 = assign82770_e125864_d_n0;
        locals.var_t1_dn2 = assign82770_e125864_d_n2;
        locals.var_t1_dn4 = assign82770_e125864_d_n4;
        locals.var_t1_dn5 = assign82770_e125864_d_n5;
        locals.var_t1_dn6 = assign82770_e125864_d_n6;
        locals.var_t1_dn7 = assign82770_e125864_d_n7;
        locals.var_t1_dn8 = assign82770_e125864_d_n8;
        locals.var_t1_dn9 = assign82770_e125864_d_n9;
        locals.var_t1_dn10 = assign82770_e125864_d_n10;
        locals.var_t1_dn11 = assign82770_e125864_d_n11;
        locals.var_t1_dn14 = assign82770_e125864_d_n14;

        let (assign82780_e125881, assign82780_e125881_d_n0, assign82780_e125881_d_n2, assign82780_e125881_d_n4, assign82780_e125881_d_n5, assign82780_e125881_d_n6, assign82780_e125881_d_n7, assign82780_e125881_d_n8, assign82780_e125881_d_n9, assign82780_e125881_d_n10, assign82780_e125881_d_n11, assign82780_e125881_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1915 == 0.0)) {
        let (assign82780_e125879, assign82780_e125879_d_n0, assign82780_e125879_d_n2, assign82780_e125879_d_n4, assign82780_e125879_d_n5, assign82780_e125879_d_n6, assign82780_e125879_d_n7, assign82780_e125879_d_n8, assign82780_e125879_d_n9, assign82780_e125879_d_n10, assign82780_e125879_d_n11, assign82780_e125879_d_n14,) = {
            if (locals.var_chi_a <= locals.var_chi_b) {
                (locals.var_chi_a, locals.var_chi_a_dn0, locals.var_chi_a_dn2, locals.var_chi_a_dn4, locals.var_chi_a_dn5, locals.var_chi_a_dn6, locals.var_chi_a_dn7, locals.var_chi_a_dn8, locals.var_chi_a_dn9, locals.var_chi_a_dn10, locals.var_chi_a_dn11, locals.var_chi_a_dn14,)
            } else {
                (locals.var_chi_b, locals.var_chi_b_dn0, locals.var_chi_b_dn2, locals.var_chi_b_dn4, locals.var_chi_b_dn5, locals.var_chi_b_dn6, locals.var_chi_b_dn7, locals.var_chi_b_dn8, locals.var_chi_b_dn9, locals.var_chi_b_dn10, locals.var_chi_b_dn11, locals.var_chi_b_dn14,)
            }
        };
        (assign82780_e125879, assign82780_e125879_d_n0, assign82780_e125879_d_n2, assign82780_e125879_d_n4, assign82780_e125879_d_n5, assign82780_e125879_d_n6, assign82780_e125879_d_n7, assign82780_e125879_d_n8, assign82780_e125879_d_n9, assign82780_e125879_d_n10, assign82780_e125879_d_n11, assign82780_e125879_d_n14,)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
        locals.var_chi = assign82780_e125881;
        locals.var_chi_dn0 = assign82780_e125881_d_n0;
        locals.var_chi_dn2 = assign82780_e125881_d_n2;
        locals.var_chi_dn4 = assign82780_e125881_d_n4;
        locals.var_chi_dn5 = assign82780_e125881_d_n5;
        locals.var_chi_dn6 = assign82780_e125881_d_n6;
        locals.var_chi_dn7 = assign82780_e125881_d_n7;
        locals.var_chi_dn8 = assign82780_e125881_d_n8;
        locals.var_chi_dn9 = assign82780_e125881_d_n9;
        locals.var_chi_dn10 = assign82780_e125881_d_n10;
        locals.var_chi_dn11 = assign82780_e125881_d_n11;
        locals.var_chi_dn14 = assign82780_e125881_d_n14;

        let assign82790_e125884: f64 = if p.p33 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1922 = assign82790_e125884;

        let (assign82800_e125897, assign82800_e125897_d_n0, assign82800_e125897_d_n2, assign82800_e125897_d_n4, assign82800_e125897_d_n5, assign82800_e125897_d_n6, assign82800_e125897_d_n7, assign82800_e125897_d_n8, assign82800_e125897_d_n9, assign82800_e125897_d_n10, assign82800_e125897_d_n11, assign82800_e125897_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1922 != 0.0)) {
        let assign82800_e125893: f64 = (locals.var_chi * locals.var_beta_inv);
        let assign82800_e125895: f64 = (assign82800_e125893 - locals.var_vxbgmtcl);
        (assign82800_e125895, (((locals.var_chi_dn0 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn0)) - locals.var_vxbgmtcl_dn0), (((locals.var_chi_dn2 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn2)) - locals.var_vxbgmtcl_dn2), (((locals.var_chi_dn4 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn4)) - locals.var_vxbgmtcl_dn4), (((locals.var_chi_dn5 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn5)) - locals.var_vxbgmtcl_dn5), (((locals.var_chi_dn6 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn6)) - locals.var_vxbgmtcl_dn6), (((locals.var_chi_dn7 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn7)) - locals.var_vxbgmtcl_dn7), (((locals.var_chi_dn8 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn8)) - locals.var_vxbgmtcl_dn8), (((locals.var_chi_dn9 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn9)) - locals.var_vxbgmtcl_dn9), (((locals.var_chi_dn10 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn10)) - locals.var_vxbgmtcl_dn10), (((locals.var_chi_dn11 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn11)) - locals.var_vxbgmtcl_dn11), (((locals.var_chi_dn14 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn14)) - locals.var_vxbgmtcl_dn14),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn14,)
    }
};
        locals.var_ps0ld = assign82800_e125897;
        locals.var_ps0ld_dn0 = assign82800_e125897_d_n0;
        locals.var_ps0ld_dn2 = assign82800_e125897_d_n2;
        locals.var_ps0ld_dn4 = assign82800_e125897_d_n4;
        locals.var_ps0ld_dn5 = assign82800_e125897_d_n5;
        locals.var_ps0ld_dn6 = assign82800_e125897_d_n6;
        locals.var_ps0ld_dn7 = assign82800_e125897_d_n7;
        locals.var_ps0ld_dn8 = assign82800_e125897_d_n8;
        locals.var_ps0ld_dn9 = assign82800_e125897_d_n9;
        locals.var_ps0ld_dn10 = assign82800_e125897_d_n10;
        locals.var_ps0ld_dn11 = assign82800_e125897_d_n11;
        locals.var_ps0ld_dn14 = assign82800_e125897_d_n14;

        let assign82810_e125900: f64 = if locals.var_wdep_func > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1923 = assign82810_e125900;

        let (assign82820_e125913, assign82820_e125913_d_n0, assign82820_e125913_d_n2, assign82820_e125913_d_n4, assign82820_e125913_d_n5, assign82820_e125913_d_n6, assign82820_e125913_d_n7, assign82820_e125913_d_n8, assign82820_e125913_d_n9, assign82820_e125913_d_n10, assign82820_e125913_d_n11, assign82820_e125913_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1922 != 0.0)) && (locals.var_guard1923 != 0.0)) {
        let assign82820_e125911: f64 = (p.p334 - locals.var_wdep_func);
        (assign82820_e125911, (-locals.var_wdep_func_dn0), (-locals.var_wdep_func_dn2), (-locals.var_wdep_func_dn4), (-locals.var_wdep_func_dn5), (-locals.var_wdep_func_dn6), (-locals.var_wdep_func_dn7), (-locals.var_wdep_func_dn8), (-locals.var_wdep_func_dn9), (-locals.var_wdep_func_dn10), (-locals.var_wdep_func_dn11), (-locals.var_wdep_func_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign82820_e125913;
        locals.var_t2_dn0 = assign82820_e125913_d_n0;
        locals.var_t2_dn2 = assign82820_e125913_d_n2;
        locals.var_t2_dn4 = assign82820_e125913_d_n4;
        locals.var_t2_dn5 = assign82820_e125913_d_n5;
        locals.var_t2_dn6 = assign82820_e125913_d_n6;
        locals.var_t2_dn7 = assign82820_e125913_d_n7;
        locals.var_t2_dn8 = assign82820_e125913_d_n8;
        locals.var_t2_dn9 = assign82820_e125913_d_n9;
        locals.var_t2_dn10 = assign82820_e125913_d_n10;
        locals.var_t2_dn11 = assign82820_e125913_d_n11;
        locals.var_t2_dn14 = assign82820_e125913_d_n14;

        let (assign82830_e125938, assign82830_e125938_d_n0, assign82830_e125938_d_n2, assign82830_e125938_d_n4, assign82830_e125938_d_n5, assign82830_e125938_d_n6, assign82830_e125938_d_n7, assign82830_e125938_d_n8, assign82830_e125938_d_n9, assign82830_e125938_d_n10, assign82830_e125938_d_n11, assign82830_e125938_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1922 != 0.0)) && (locals.var_guard1923 == 0.0)) {
        let assign82830_e125925: f64 = (locals.var_vdsi + p.p137);
        let assign82830_e125928: f64 = (locals.var_vdsi + p.p137);
        let assign82830_e125929: f64 = (assign82830_e125925 * assign82830_e125928);
        let assign82830_e125932: f64 = (4.0 * 0.1);
        let assign82830_e125934: f64 = (assign82830_e125932 * 0.1);
        let assign82830_e125935: f64 = (assign82830_e125929 + assign82830_e125934);
        let assign82830_e125936: f64 = (assign82830_e125935).sqrt();
        (assign82830_e125936, 0.0, 0.0, 0.0, 0.0, (((locals.var_vdsi_dn6 * assign82830_e125928) + (assign82830_e125925 * locals.var_vdsi_dn6)) / (2.0 * assign82830_e125936)), 0.0, (((locals.var_vdsi_dn8 * assign82830_e125928) + (assign82830_e125925 * locals.var_vdsi_dn8)) / (2.0 * assign82830_e125936)), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign82830_e125938;
        locals.var_tmf2_dn0 = assign82830_e125938_d_n0;
        locals.var_tmf2_dn2 = assign82830_e125938_d_n2;
        locals.var_tmf2_dn4 = assign82830_e125938_d_n4;
        locals.var_tmf2_dn5 = assign82830_e125938_d_n5;
        locals.var_tmf2_dn6 = assign82830_e125938_d_n6;
        locals.var_tmf2_dn7 = assign82830_e125938_d_n7;
        locals.var_tmf2_dn8 = assign82830_e125938_d_n8;
        locals.var_tmf2_dn9 = assign82830_e125938_d_n9;
        locals.var_tmf2_dn10 = assign82830_e125938_d_n10;
        locals.var_tmf2_dn11 = assign82830_e125938_d_n11;
        locals.var_tmf2_dn14 = assign82830_e125938_d_n14;

        let (assign82840_e125958, assign82840_e125958_d_n0, assign82840_e125958_d_n2, assign82840_e125958_d_n4, assign82840_e125958_d_n5, assign82840_e125958_d_n6, assign82840_e125958_d_n7, assign82840_e125958_d_n8, assign82840_e125958_d_n9, assign82840_e125958_d_n10, assign82840_e125958_d_n11, assign82840_e125958_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1922 != 0.0)) && (locals.var_guard1923 == 0.0)) {
        let assign82840_e125952: f64 = (locals.var_vdsi + p.p137);
        let assign82840_e125954: f64 = (assign82840_e125952 / locals.var_tmf2);
        let assign82840_e125955: f64 = (1.0 + assign82840_e125954);
        let assign82840_e125956: f64 = (0.5 * assign82840_e125955);
        (assign82840_e125956, (0.5 * (-((assign82840_e125952 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign82840_e125952 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign82840_e125952 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign82840_e125952 * locals.var_tmf2_dn5) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (((locals.var_vdsi_dn6 * locals.var_tmf2) - (assign82840_e125952 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (-((assign82840_e125952 * locals.var_tmf2_dn7) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (((locals.var_vdsi_dn8 * locals.var_tmf2) - (assign82840_e125952 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (-((assign82840_e125952 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign82840_e125952 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign82840_e125952 * locals.var_tmf2_dn11) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign82840_e125952 * locals.var_tmf2_dn14) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign82840_e125958;
        locals.var_t9_dn0 = assign82840_e125958_d_n0;
        locals.var_t9_dn2 = assign82840_e125958_d_n2;
        locals.var_t9_dn4 = assign82840_e125958_d_n4;
        locals.var_t9_dn5 = assign82840_e125958_d_n5;
        locals.var_t9_dn6 = assign82840_e125958_d_n6;
        locals.var_t9_dn7 = assign82840_e125958_d_n7;
        locals.var_t9_dn8 = assign82840_e125958_d_n8;
        locals.var_t9_dn9 = assign82840_e125958_d_n9;
        locals.var_t9_dn10 = assign82840_e125958_d_n10;
        locals.var_t9_dn11 = assign82840_e125958_d_n11;
        locals.var_t9_dn14 = assign82840_e125958_d_n14;

        let (assign82850_e125976, assign82850_e125976_d_n0, assign82850_e125976_d_n2, assign82850_e125976_d_n4, assign82850_e125976_d_n5, assign82850_e125976_d_n6, assign82850_e125976_d_n7, assign82850_e125976_d_n8, assign82850_e125976_d_n9, assign82850_e125976_d_n10, assign82850_e125976_d_n11, assign82850_e125976_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1922 != 0.0)) && (locals.var_guard1923 == 0.0)) {
        let assign82850_e125971: f64 = (locals.var_vdsi + p.p137);
        let assign82850_e125973: f64 = (assign82850_e125971 + locals.var_tmf2);
        let assign82850_e125974: f64 = (0.5 * assign82850_e125973);
        (assign82850_e125974, (0.5 * locals.var_tmf2_dn0), (0.5 * locals.var_tmf2_dn2), (0.5 * locals.var_tmf2_dn4), (0.5 * locals.var_tmf2_dn5), (0.5 * (locals.var_vdsi_dn6 + locals.var_tmf2_dn6)), (0.5 * locals.var_tmf2_dn7), (0.5 * (locals.var_vdsi_dn8 + locals.var_tmf2_dn8)), (0.5 * locals.var_tmf2_dn9), (0.5 * locals.var_tmf2_dn10), (0.5 * locals.var_tmf2_dn11), (0.5 * locals.var_tmf2_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign82850_e125976;
        locals.var_t2_dn0 = assign82850_e125976_d_n0;
        locals.var_t2_dn2 = assign82850_e125976_d_n2;
        locals.var_t2_dn4 = assign82850_e125976_d_n4;
        locals.var_t2_dn5 = assign82850_e125976_d_n5;
        locals.var_t2_dn6 = assign82850_e125976_d_n6;
        locals.var_t2_dn7 = assign82850_e125976_d_n7;
        locals.var_t2_dn8 = assign82850_e125976_d_n8;
        locals.var_t2_dn9 = assign82850_e125976_d_n9;
        locals.var_t2_dn10 = assign82850_e125976_d_n10;
        locals.var_t2_dn11 = assign82850_e125976_d_n11;
        locals.var_t2_dn14 = assign82850_e125976_d_n14;

        let assign82860_e125979: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1924 = assign82860_e125979;

    }

    pub(super) fn stamp_transient_block_300(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign82870_e125993, assign82870_e125993_d_n0, assign82870_e125993_d_n2, assign82870_e125993_d_n4, assign82870_e125993_d_n5, assign82870_e125993_d_n6, assign82870_e125993_d_n7, assign82870_e125993_d_n8, assign82870_e125993_d_n9, assign82870_e125993_d_n10, assign82870_e125993_d_n11, assign82870_e125993_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1922 != 0.0)) && (locals.var_guard1923 == 0.0)) && (locals.var_guard1924 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign82870_e125993;
        locals.var_t2_dn0 = assign82870_e125993_d_n0;
        locals.var_t2_dn2 = assign82870_e125993_d_n2;
        locals.var_t2_dn4 = assign82870_e125993_d_n4;
        locals.var_t2_dn5 = assign82870_e125993_d_n5;
        locals.var_t2_dn6 = assign82870_e125993_d_n6;
        locals.var_t2_dn7 = assign82870_e125993_d_n7;
        locals.var_t2_dn8 = assign82870_e125993_d_n8;
        locals.var_t2_dn9 = assign82870_e125993_d_n9;
        locals.var_t2_dn10 = assign82870_e125993_d_n10;
        locals.var_t2_dn11 = assign82870_e125993_d_n11;
        locals.var_t2_dn14 = assign82870_e125993_d_n14;

        let (assign82880_e126007, assign82880_e126007_d_n0, assign82880_e126007_d_n2, assign82880_e126007_d_n4, assign82880_e126007_d_n5, assign82880_e126007_d_n6, assign82880_e126007_d_n7, assign82880_e126007_d_n8, assign82880_e126007_d_n9, assign82880_e126007_d_n10, assign82880_e126007_d_n11, assign82880_e126007_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1922 != 0.0)) && (locals.var_guard1923 == 0.0)) && (locals.var_guard1924 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign82880_e126007;
        locals.var_t9_dn0 = assign82880_e126007_d_n0;
        locals.var_t9_dn2 = assign82880_e126007_d_n2;
        locals.var_t9_dn4 = assign82880_e126007_d_n4;
        locals.var_t9_dn5 = assign82880_e126007_d_n5;
        locals.var_t9_dn6 = assign82880_e126007_d_n6;
        locals.var_t9_dn7 = assign82880_e126007_d_n7;
        locals.var_t9_dn8 = assign82880_e126007_d_n8;
        locals.var_t9_dn9 = assign82880_e126007_d_n9;
        locals.var_t9_dn10 = assign82880_e126007_d_n10;
        locals.var_t9_dn11 = assign82880_e126007_d_n11;
        locals.var_t9_dn14 = assign82880_e126007_d_n14;

        let (assign82890_e126024, assign82890_e126024_d_n0, assign82890_e126024_d_n2, assign82890_e126024_d_n4, assign82890_e126024_d_n5, assign82890_e126024_d_n6, assign82890_e126024_d_n7, assign82890_e126024_d_n8, assign82890_e126024_d_n9, assign82890_e126024_d_n10, assign82890_e126024_d_n11, assign82890_e126024_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1922 != 0.0)) && (locals.var_guard1923 == 0.0)) {
        let assign82890_e126019: f64 = (locals.var_kjunc * locals.var_t2);
        let assign82890_e126020: f64 = (assign82890_e126019).sqrt();
        let assign82890_e126022: f64 = (assign82890_e126020 * p.p432);
        (assign82890_e126022, ((((locals.var_kjunc_dn0 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn0)) / (2.0 * assign82890_e126020)) * p.p432), ((((locals.var_kjunc_dn2 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn2)) / (2.0 * assign82890_e126020)) * p.p432), ((((locals.var_kjunc_dn4 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn4)) / (2.0 * assign82890_e126020)) * p.p432), ((((locals.var_kjunc_dn5 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn5)) / (2.0 * assign82890_e126020)) * p.p432), ((((locals.var_kjunc_dn6 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn6)) / (2.0 * assign82890_e126020)) * p.p432), ((((locals.var_kjunc_dn7 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn7)) / (2.0 * assign82890_e126020)) * p.p432), ((((locals.var_kjunc_dn8 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn8)) / (2.0 * assign82890_e126020)) * p.p432), ((((locals.var_kjunc_dn9 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn9)) / (2.0 * assign82890_e126020)) * p.p432), ((((locals.var_kjunc_dn10 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn10)) / (2.0 * assign82890_e126020)) * p.p432), ((((locals.var_kjunc_dn11 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn11)) / (2.0 * assign82890_e126020)) * p.p432), ((((locals.var_kjunc_dn14 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn14)) / (2.0 * assign82890_e126020)) * p.p432),)
    } else {
        (locals.var_wjunc0, locals.var_wjunc0_dn0, locals.var_wjunc0_dn2, locals.var_wjunc0_dn4, locals.var_wjunc0_dn5, locals.var_wjunc0_dn6, locals.var_wjunc0_dn7, locals.var_wjunc0_dn8, locals.var_wjunc0_dn9, locals.var_wjunc0_dn10, locals.var_wjunc0_dn11, locals.var_wjunc0_dn14,)
    }
};
        locals.var_wjunc0 = assign82890_e126024;
        locals.var_wjunc0_dn0 = assign82890_e126024_d_n0;
        locals.var_wjunc0_dn2 = assign82890_e126024_d_n2;
        locals.var_wjunc0_dn4 = assign82890_e126024_d_n4;
        locals.var_wjunc0_dn5 = assign82890_e126024_d_n5;
        locals.var_wjunc0_dn6 = assign82890_e126024_d_n6;
        locals.var_wjunc0_dn7 = assign82890_e126024_d_n7;
        locals.var_wjunc0_dn8 = assign82890_e126024_d_n8;
        locals.var_wjunc0_dn9 = assign82890_e126024_d_n9;
        locals.var_wjunc0_dn10 = assign82890_e126024_d_n10;
        locals.var_wjunc0_dn11 = assign82890_e126024_d_n11;
        locals.var_wjunc0_dn14 = assign82890_e126024_d_n14;

        let (assign82900_e126038, assign82900_e126038_d_n0, assign82900_e126038_d_n2, assign82900_e126038_d_n4, assign82900_e126038_d_n5, assign82900_e126038_d_n6, assign82900_e126038_d_n7, assign82900_e126038_d_n8, assign82900_e126038_d_n9, assign82900_e126038_d_n10, assign82900_e126038_d_n11, assign82900_e126038_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1922 != 0.0)) && (locals.var_guard1923 == 0.0)) {
        let assign82900_e126036: f64 = (p.p334 - locals.var_wjunc0);
        (assign82900_e126036, (-locals.var_wjunc0_dn0), (-locals.var_wjunc0_dn2), (-locals.var_wjunc0_dn4), (-locals.var_wjunc0_dn5), (-locals.var_wjunc0_dn6), (-locals.var_wjunc0_dn7), (-locals.var_wjunc0_dn8), (-locals.var_wjunc0_dn9), (-locals.var_wjunc0_dn10), (-locals.var_wjunc0_dn11), (-locals.var_wjunc0_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign82900_e126038;
        locals.var_t2_dn0 = assign82900_e126038_d_n0;
        locals.var_t2_dn2 = assign82900_e126038_d_n2;
        locals.var_t2_dn4 = assign82900_e126038_d_n4;
        locals.var_t2_dn5 = assign82900_e126038_d_n5;
        locals.var_t2_dn6 = assign82900_e126038_d_n6;
        locals.var_t2_dn7 = assign82900_e126038_d_n7;
        locals.var_t2_dn8 = assign82900_e126038_d_n8;
        locals.var_t2_dn9 = assign82900_e126038_d_n9;
        locals.var_t2_dn10 = assign82900_e126038_d_n10;
        locals.var_t2_dn11 = assign82900_e126038_d_n11;
        locals.var_t2_dn14 = assign82900_e126038_d_n14;

        let (assign82910_e126060, assign82910_e126060_d_n0, assign82910_e126060_d_n2, assign82910_e126060_d_n4, assign82910_e126060_d_n5, assign82910_e126060_d_n6, assign82910_e126060_d_n7, assign82910_e126060_d_n8, assign82910_e126060_d_n9, assign82910_e126060_d_n10, assign82910_e126060_d_n11, assign82910_e126060_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1922 != 0.0)) {
        let assign82910_e126047: f64 = (locals.var_t2 * locals.var_t2);
        let assign82910_e126051: f64 = (p.p334 * 0.01);
        let assign82910_e126052: f64 = (4.0 * assign82910_e126051);
        let assign82910_e126055: f64 = (p.p334 * 0.01);
        let assign82910_e126056: f64 = (assign82910_e126052 * assign82910_e126055);
        let assign82910_e126057: f64 = (assign82910_e126047 + assign82910_e126056);
        let assign82910_e126058: f64 = (assign82910_e126057).sqrt();
        (assign82910_e126058, (((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)) / (2.0 * assign82910_e126058)), (((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)) / (2.0 * assign82910_e126058)), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign82910_e126058)), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign82910_e126058)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign82910_e126058)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign82910_e126058)), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign82910_e126058)), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign82910_e126058)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign82910_e126058)), (((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign82910_e126058)), (((locals.var_t2_dn14 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn14)) / (2.0 * assign82910_e126058)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign82910_e126060;
        locals.var_tmf2_dn0 = assign82910_e126060_d_n0;
        locals.var_tmf2_dn2 = assign82910_e126060_d_n2;
        locals.var_tmf2_dn4 = assign82910_e126060_d_n4;
        locals.var_tmf2_dn5 = assign82910_e126060_d_n5;
        locals.var_tmf2_dn6 = assign82910_e126060_d_n6;
        locals.var_tmf2_dn7 = assign82910_e126060_d_n7;
        locals.var_tmf2_dn8 = assign82910_e126060_d_n8;
        locals.var_tmf2_dn9 = assign82910_e126060_d_n9;
        locals.var_tmf2_dn10 = assign82910_e126060_d_n10;
        locals.var_tmf2_dn11 = assign82910_e126060_d_n11;
        locals.var_tmf2_dn14 = assign82910_e126060_d_n14;

        let (assign82920_e126075, assign82920_e126075_d_n0, assign82920_e126075_d_n2, assign82920_e126075_d_n4, assign82920_e126075_d_n5, assign82920_e126075_d_n6, assign82920_e126075_d_n7, assign82920_e126075_d_n8, assign82920_e126075_d_n9, assign82920_e126075_d_n10, assign82920_e126075_d_n11, assign82920_e126075_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1922 != 0.0)) {
        let assign82920_e126071: f64 = (locals.var_t2 / locals.var_tmf2);
        let assign82920_e126072: f64 = (1.0 + assign82920_e126071);
        let assign82920_e126073: f64 = (0.5 * assign82920_e126072);
        (assign82920_e126073, (0.5 * (((locals.var_t2_dn0 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn2 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn4 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn5 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn6 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn7 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn8 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn9 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn10 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn11 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn14 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign82920_e126075;
        locals.var_t9_dn0 = assign82920_e126075_d_n0;
        locals.var_t9_dn2 = assign82920_e126075_d_n2;
        locals.var_t9_dn4 = assign82920_e126075_d_n4;
        locals.var_t9_dn5 = assign82920_e126075_d_n5;
        locals.var_t9_dn6 = assign82920_e126075_d_n6;
        locals.var_t9_dn7 = assign82920_e126075_d_n7;
        locals.var_t9_dn8 = assign82920_e126075_d_n8;
        locals.var_t9_dn9 = assign82920_e126075_d_n9;
        locals.var_t9_dn10 = assign82920_e126075_d_n10;
        locals.var_t9_dn11 = assign82920_e126075_d_n11;
        locals.var_t9_dn14 = assign82920_e126075_d_n14;

        let (assign82930_e126088, assign82930_e126088_d_n0, assign82930_e126088_d_n2, assign82930_e126088_d_n4, assign82930_e126088_d_n5, assign82930_e126088_d_n6, assign82930_e126088_d_n7, assign82930_e126088_d_n8, assign82930_e126088_d_n9, assign82930_e126088_d_n10, assign82930_e126088_d_n11, assign82930_e126088_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1922 != 0.0)) {
        let assign82930_e126085: f64 = (locals.var_t2 + locals.var_tmf2);
        let assign82930_e126086: f64 = (0.5 * assign82930_e126085);
        (assign82930_e126086, (0.5 * (locals.var_t2_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t2_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t2_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t2_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t2_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t2_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t2_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t2_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t2_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t2_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_t2_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign82930_e126088;
        locals.var_t2_dn0 = assign82930_e126088_d_n0;
        locals.var_t2_dn2 = assign82930_e126088_d_n2;
        locals.var_t2_dn4 = assign82930_e126088_d_n4;
        locals.var_t2_dn5 = assign82930_e126088_d_n5;
        locals.var_t2_dn6 = assign82930_e126088_d_n6;
        locals.var_t2_dn7 = assign82930_e126088_d_n7;
        locals.var_t2_dn8 = assign82930_e126088_d_n8;
        locals.var_t2_dn9 = assign82930_e126088_d_n9;
        locals.var_t2_dn10 = assign82930_e126088_d_n10;
        locals.var_t2_dn11 = assign82930_e126088_d_n11;
        locals.var_t2_dn14 = assign82930_e126088_d_n14;

        let assign82940_e126091: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1925 = assign82940_e126091;

        let (assign82950_e126102, assign82950_e126102_d_n0, assign82950_e126102_d_n2, assign82950_e126102_d_n4, assign82950_e126102_d_n5, assign82950_e126102_d_n6, assign82950_e126102_d_n7, assign82950_e126102_d_n8, assign82950_e126102_d_n9, assign82950_e126102_d_n10, assign82950_e126102_d_n11, assign82950_e126102_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1922 != 0.0)) && (locals.var_guard1925 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign82950_e126102;
        locals.var_t2_dn0 = assign82950_e126102_d_n0;
        locals.var_t2_dn2 = assign82950_e126102_d_n2;
        locals.var_t2_dn4 = assign82950_e126102_d_n4;
        locals.var_t2_dn5 = assign82950_e126102_d_n5;
        locals.var_t2_dn6 = assign82950_e126102_d_n6;
        locals.var_t2_dn7 = assign82950_e126102_d_n7;
        locals.var_t2_dn8 = assign82950_e126102_d_n8;
        locals.var_t2_dn9 = assign82950_e126102_d_n9;
        locals.var_t2_dn10 = assign82950_e126102_d_n10;
        locals.var_t2_dn11 = assign82950_e126102_d_n11;
        locals.var_t2_dn14 = assign82950_e126102_d_n14;

        let (assign82960_e126113, assign82960_e126113_d_n0, assign82960_e126113_d_n2, assign82960_e126113_d_n4, assign82960_e126113_d_n5, assign82960_e126113_d_n6, assign82960_e126113_d_n7, assign82960_e126113_d_n8, assign82960_e126113_d_n9, assign82960_e126113_d_n10, assign82960_e126113_d_n11, assign82960_e126113_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1922 != 0.0)) && (locals.var_guard1925 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign82960_e126113;
        locals.var_t9_dn0 = assign82960_e126113_d_n0;
        locals.var_t9_dn2 = assign82960_e126113_d_n2;
        locals.var_t9_dn4 = assign82960_e126113_d_n4;
        locals.var_t9_dn5 = assign82960_e126113_d_n5;
        locals.var_t9_dn6 = assign82960_e126113_d_n6;
        locals.var_t9_dn7 = assign82960_e126113_d_n7;
        locals.var_t9_dn8 = assign82960_e126113_d_n8;
        locals.var_t9_dn9 = assign82960_e126113_d_n9;
        locals.var_t9_dn10 = assign82960_e126113_d_n10;
        locals.var_t9_dn11 = assign82960_e126113_d_n11;
        locals.var_t9_dn14 = assign82960_e126113_d_n14;

        let (assign82970_e126122, assign82970_e126122_d_n0, assign82970_e126122_d_n2, assign82970_e126122_d_n4, assign82970_e126122_d_n5, assign82970_e126122_d_n6, assign82970_e126122_d_n7, assign82970_e126122_d_n8, assign82970_e126122_d_n9, assign82970_e126122_d_n10, assign82970_e126122_d_n11, assign82970_e126122_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1922 != 0.0)) {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    } else {
        (locals.var_ddriftldc, locals.var_ddriftldc_dn0, locals.var_ddriftldc_dn2, locals.var_ddriftldc_dn4, locals.var_ddriftldc_dn5, locals.var_ddriftldc_dn6, locals.var_ddriftldc_dn7, locals.var_ddriftldc_dn8, locals.var_ddriftldc_dn9, locals.var_ddriftldc_dn10, locals.var_ddriftldc_dn11, locals.var_ddriftldc_dn14,)
    }
};
        locals.var_ddriftldc = assign82970_e126122;
        locals.var_ddriftldc_dn0 = assign82970_e126122_d_n0;
        locals.var_ddriftldc_dn2 = assign82970_e126122_d_n2;
        locals.var_ddriftldc_dn4 = assign82970_e126122_d_n4;
        locals.var_ddriftldc_dn5 = assign82970_e126122_d_n5;
        locals.var_ddriftldc_dn6 = assign82970_e126122_d_n6;
        locals.var_ddriftldc_dn7 = assign82970_e126122_d_n7;
        locals.var_ddriftldc_dn8 = assign82970_e126122_d_n8;
        locals.var_ddriftldc_dn9 = assign82970_e126122_d_n9;
        locals.var_ddriftldc_dn10 = assign82970_e126122_d_n10;
        locals.var_ddriftldc_dn11 = assign82970_e126122_d_n11;
        locals.var_ddriftldc_dn14 = assign82970_e126122_d_n14;

        let (assign82980_e126139, assign82980_e126139_d_n0, assign82980_e126139_d_n2, assign82980_e126139_d_n4, assign82980_e126139_d_n5, assign82980_e126139_d_n6, assign82980_e126139_d_n7, assign82980_e126139_d_n8, assign82980_e126139_d_n9, assign82980_e126139_d_n10, assign82980_e126139_d_n11, assign82980_e126139_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1922 != 0.0)) {
        let assign82980_e126131: f64 = (locals.var_q_nsubld__blk1885 * locals.var_ddriftldc);
        let assign82980_e126133: f64 = (assign82980_e126131 * locals.var_ddriftldc);
        let assign82980_e126135: f64 = (assign82980_e126133 / 2.0);
        let assign82980_e126137: f64 = (assign82980_e126135 / 1.034943e-10);
        (assign82980_e126137, (((((locals.var_q_nsubld__blk1885 * locals.var_ddriftldc_dn0) * locals.var_ddriftldc) + (assign82980_e126131 * locals.var_ddriftldc_dn0)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1885 * locals.var_ddriftldc_dn2) * locals.var_ddriftldc) + (assign82980_e126131 * locals.var_ddriftldc_dn2)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1885 * locals.var_ddriftldc_dn4) * locals.var_ddriftldc) + (assign82980_e126131 * locals.var_ddriftldc_dn4)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1885 * locals.var_ddriftldc_dn5) * locals.var_ddriftldc) + (assign82980_e126131 * locals.var_ddriftldc_dn5)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1885 * locals.var_ddriftldc_dn6) * locals.var_ddriftldc) + (assign82980_e126131 * locals.var_ddriftldc_dn6)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1885 * locals.var_ddriftldc_dn7) * locals.var_ddriftldc) + (assign82980_e126131 * locals.var_ddriftldc_dn7)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1885 * locals.var_ddriftldc_dn8) * locals.var_ddriftldc) + (assign82980_e126131 * locals.var_ddriftldc_dn8)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1885 * locals.var_ddriftldc_dn9) * locals.var_ddriftldc) + (assign82980_e126131 * locals.var_ddriftldc_dn9)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1885 * locals.var_ddriftldc_dn10) * locals.var_ddriftldc) + (assign82980_e126131 * locals.var_ddriftldc_dn10)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1885 * locals.var_ddriftldc_dn11) * locals.var_ddriftldc) + (assign82980_e126131 * locals.var_ddriftldc_dn11)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1885 * locals.var_ddriftldc_dn14) * locals.var_ddriftldc) + (assign82980_e126131 * locals.var_ddriftldc_dn14)) / 2.0) / 1.034943e-10),)
    } else {
        (locals.var_dphi_sb, locals.var_dphi_sb_dn0, locals.var_dphi_sb_dn2, locals.var_dphi_sb_dn4, locals.var_dphi_sb_dn5, locals.var_dphi_sb_dn6, locals.var_dphi_sb_dn7, locals.var_dphi_sb_dn8, locals.var_dphi_sb_dn9, locals.var_dphi_sb_dn10, locals.var_dphi_sb_dn11, locals.var_dphi_sb_dn14,)
    }
};
        locals.var_dphi_sb = assign82980_e126139;
        locals.var_dphi_sb_dn0 = assign82980_e126139_d_n0;
        locals.var_dphi_sb_dn2 = assign82980_e126139_d_n2;
        locals.var_dphi_sb_dn4 = assign82980_e126139_d_n4;
        locals.var_dphi_sb_dn5 = assign82980_e126139_d_n5;
        locals.var_dphi_sb_dn6 = assign82980_e126139_d_n6;
        locals.var_dphi_sb_dn7 = assign82980_e126139_d_n7;
        locals.var_dphi_sb_dn8 = assign82980_e126139_d_n8;
        locals.var_dphi_sb_dn9 = assign82980_e126139_d_n9;
        locals.var_dphi_sb_dn10 = assign82980_e126139_d_n10;
        locals.var_dphi_sb_dn11 = assign82980_e126139_d_n11;
        locals.var_dphi_sb_dn14 = assign82980_e126139_d_n14;

        let (assign82990_e126153, assign82990_e126153_d_n0, assign82990_e126153_d_n2, assign82990_e126153_d_n4, assign82990_e126153_d_n5, assign82990_e126153_d_n6, assign82990_e126153_d_n7, assign82990_e126153_d_n8, assign82990_e126153_d_n9, assign82990_e126153_d_n10, assign82990_e126153_d_n11, assign82990_e126153_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1922 != 0.0)) {
        let assign82990_e126148: f64 = (2.0 * locals.var_beta);
        let assign82990_e126150: f64 = (assign82990_e126148 * locals.var_dphi_sb);
        let assign82990_e126151: f64 = (assign82990_e126150).sqrt();
        (assign82990_e126151, ((((2.0 * locals.var_beta_dn0) * locals.var_dphi_sb) + (assign82990_e126148 * locals.var_dphi_sb_dn0)) / (2.0 * assign82990_e126151)), ((((2.0 * locals.var_beta_dn2) * locals.var_dphi_sb) + (assign82990_e126148 * locals.var_dphi_sb_dn2)) / (2.0 * assign82990_e126151)), ((((2.0 * locals.var_beta_dn4) * locals.var_dphi_sb) + (assign82990_e126148 * locals.var_dphi_sb_dn4)) / (2.0 * assign82990_e126151)), ((((2.0 * locals.var_beta_dn5) * locals.var_dphi_sb) + (assign82990_e126148 * locals.var_dphi_sb_dn5)) / (2.0 * assign82990_e126151)), ((((2.0 * locals.var_beta_dn6) * locals.var_dphi_sb) + (assign82990_e126148 * locals.var_dphi_sb_dn6)) / (2.0 * assign82990_e126151)), ((((2.0 * locals.var_beta_dn7) * locals.var_dphi_sb) + (assign82990_e126148 * locals.var_dphi_sb_dn7)) / (2.0 * assign82990_e126151)), ((((2.0 * locals.var_beta_dn8) * locals.var_dphi_sb) + (assign82990_e126148 * locals.var_dphi_sb_dn8)) / (2.0 * assign82990_e126151)), ((((2.0 * locals.var_beta_dn9) * locals.var_dphi_sb) + (assign82990_e126148 * locals.var_dphi_sb_dn9)) / (2.0 * assign82990_e126151)), ((((2.0 * locals.var_beta_dn10) * locals.var_dphi_sb) + (assign82990_e126148 * locals.var_dphi_sb_dn10)) / (2.0 * assign82990_e126151)), ((((2.0 * locals.var_beta_dn11) * locals.var_dphi_sb) + (assign82990_e126148 * locals.var_dphi_sb_dn11)) / (2.0 * assign82990_e126151)), ((((2.0 * locals.var_beta_dn14) * locals.var_dphi_sb) + (assign82990_e126148 * locals.var_dphi_sb_dn14)) / (2.0 * assign82990_e126151)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign82990_e126153;
        locals.var_t0_dn0 = assign82990_e126153_d_n0;
        locals.var_t0_dn2 = assign82990_e126153_d_n2;
        locals.var_t0_dn4 = assign82990_e126153_d_n4;
        locals.var_t0_dn5 = assign82990_e126153_d_n5;
        locals.var_t0_dn6 = assign82990_e126153_d_n6;
        locals.var_t0_dn7 = assign82990_e126153_d_n7;
        locals.var_t0_dn8 = assign82990_e126153_d_n8;
        locals.var_t0_dn9 = assign82990_e126153_d_n9;
        locals.var_t0_dn10 = assign82990_e126153_d_n10;
        locals.var_t0_dn11 = assign82990_e126153_d_n11;
        locals.var_t0_dn14 = assign82990_e126153_d_n14;

        let (assign83000_e126169, assign83000_e126169_d_n0, assign83000_e126169_d_n2, assign83000_e126169_d_n4, assign83000_e126169_d_n5, assign83000_e126169_d_n6, assign83000_e126169_d_n7, assign83000_e126169_d_n8, assign83000_e126169_d_n9, assign83000_e126169_d_n10, assign83000_e126169_d_n11, assign83000_e126169_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1922 != 0.0)) {
        let assign83000_e126161: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign83000_e126163: f64 = (-locals.var_t0);
        let assign83000_e126164: f64 = { let limited_exp_arg = assign83000_e126163; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign83000_e126165: f64 = (assign83000_e126161 + assign83000_e126164);
        let assign83000_e126167: f64 = (assign83000_e126165 / 2.0);
        (assign83000_e126167, ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn0) + ({ let limited_exp_arg = assign83000_e126163; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn0))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn2) + ({ let limited_exp_arg = assign83000_e126163; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn2))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4) + ({ let limited_exp_arg = assign83000_e126163; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn4))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5) + ({ let limited_exp_arg = assign83000_e126163; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn5))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6) + ({ let limited_exp_arg = assign83000_e126163; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn6))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7) + ({ let limited_exp_arg = assign83000_e126163; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn7))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8) + ({ let limited_exp_arg = assign83000_e126163; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn8))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9) + ({ let limited_exp_arg = assign83000_e126163; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn9))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10) + ({ let limited_exp_arg = assign83000_e126163; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn10))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11) + ({ let limited_exp_arg = assign83000_e126163; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn11))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn14) + ({ let limited_exp_arg = assign83000_e126163; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn14))) / 2.0),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign83000_e126169;
        locals.var_t1_dn0 = assign83000_e126169_d_n0;
        locals.var_t1_dn2 = assign83000_e126169_d_n2;
        locals.var_t1_dn4 = assign83000_e126169_d_n4;
        locals.var_t1_dn5 = assign83000_e126169_d_n5;
        locals.var_t1_dn6 = assign83000_e126169_d_n6;
        locals.var_t1_dn7 = assign83000_e126169_d_n7;
        locals.var_t1_dn8 = assign83000_e126169_d_n8;
        locals.var_t1_dn9 = assign83000_e126169_d_n9;
        locals.var_t1_dn10 = assign83000_e126169_d_n10;
        locals.var_t1_dn11 = assign83000_e126169_d_n11;
        locals.var_t1_dn14 = assign83000_e126169_d_n14;

        let (assign83010_e126181, assign83010_e126181_d_n0, assign83010_e126181_d_n2, assign83010_e126181_d_n4, assign83010_e126181_d_n5, assign83010_e126181_d_n6, assign83010_e126181_d_n7, assign83010_e126181_d_n8, assign83010_e126181_d_n9, assign83010_e126181_d_n10, assign83010_e126181_d_n11, assign83010_e126181_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1922 != 0.0)) {
        let assign83010_e126177: f64 = (locals.var_t1).ln();
        let assign83010_e126179: f64 = (assign83010_e126177 / locals.var_dphi_sb);
        (assign83010_e126179, ((((locals.var_t1_dn0 / locals.var_t1) * locals.var_dphi_sb) - (assign83010_e126177 * locals.var_dphi_sb_dn0)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn2 / locals.var_t1) * locals.var_dphi_sb) - (assign83010_e126177 * locals.var_dphi_sb_dn2)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn4 / locals.var_t1) * locals.var_dphi_sb) - (assign83010_e126177 * locals.var_dphi_sb_dn4)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn5 / locals.var_t1) * locals.var_dphi_sb) - (assign83010_e126177 * locals.var_dphi_sb_dn5)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn6 / locals.var_t1) * locals.var_dphi_sb) - (assign83010_e126177 * locals.var_dphi_sb_dn6)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn7 / locals.var_t1) * locals.var_dphi_sb) - (assign83010_e126177 * locals.var_dphi_sb_dn7)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn8 / locals.var_t1) * locals.var_dphi_sb) - (assign83010_e126177 * locals.var_dphi_sb_dn8)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn9 / locals.var_t1) * locals.var_dphi_sb) - (assign83010_e126177 * locals.var_dphi_sb_dn9)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn10 / locals.var_t1) * locals.var_dphi_sb) - (assign83010_e126177 * locals.var_dphi_sb_dn10)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn11 / locals.var_t1) * locals.var_dphi_sb) - (assign83010_e126177 * locals.var_dphi_sb_dn11)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn14 / locals.var_t1) * locals.var_dphi_sb) - (assign83010_e126177 * locals.var_dphi_sb_dn14)) / (locals.var_dphi_sb * locals.var_dphi_sb)),)
    } else {
        (locals.var_c_sb, locals.var_c_sb_dn0, locals.var_c_sb_dn2, locals.var_c_sb_dn4, locals.var_c_sb_dn5, locals.var_c_sb_dn6, locals.var_c_sb_dn7, locals.var_c_sb_dn8, locals.var_c_sb_dn9, locals.var_c_sb_dn10, locals.var_c_sb_dn11, locals.var_c_sb_dn14,)
    }
};
        locals.var_c_sb = assign83010_e126181;
        locals.var_c_sb_dn0 = assign83010_e126181_d_n0;
        locals.var_c_sb_dn2 = assign83010_e126181_d_n2;
        locals.var_c_sb_dn4 = assign83010_e126181_d_n4;
        locals.var_c_sb_dn5 = assign83010_e126181_d_n5;
        locals.var_c_sb_dn6 = assign83010_e126181_d_n6;
        locals.var_c_sb_dn7 = assign83010_e126181_d_n7;
        locals.var_c_sb_dn8 = assign83010_e126181_d_n8;
        locals.var_c_sb_dn9 = assign83010_e126181_d_n9;
        locals.var_c_sb_dn10 = assign83010_e126181_d_n10;
        locals.var_c_sb_dn11 = assign83010_e126181_d_n11;
        locals.var_c_sb_dn14 = assign83010_e126181_d_n14;

        let (assign83020_e126192, assign83020_e126192_d_n0, assign83020_e126192_d_n2, assign83020_e126192_d_n4, assign83020_e126192_d_n5, assign83020_e126192_d_n6, assign83020_e126192_d_n7, assign83020_e126192_d_n8, assign83020_e126192_d_n9, assign83020_e126192_d_n10, assign83020_e126192_d_n11, assign83020_e126192_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1922 != 0.0)) {
        let assign83020_e126190: f64 = (locals.var_ps0ld + locals.var_vxbgmtcl);
        (assign83020_e126190, (locals.var_ps0ld_dn0 + locals.var_vxbgmtcl_dn0), (locals.var_ps0ld_dn2 + locals.var_vxbgmtcl_dn2), (locals.var_ps0ld_dn4 + locals.var_vxbgmtcl_dn4), (locals.var_ps0ld_dn5 + locals.var_vxbgmtcl_dn5), (locals.var_ps0ld_dn6 + locals.var_vxbgmtcl_dn6), (locals.var_ps0ld_dn7 + locals.var_vxbgmtcl_dn7), (locals.var_ps0ld_dn8 + locals.var_vxbgmtcl_dn8), (locals.var_ps0ld_dn9 + locals.var_vxbgmtcl_dn9), (locals.var_ps0ld_dn10 + locals.var_vxbgmtcl_dn10), (locals.var_ps0ld_dn11 + locals.var_vxbgmtcl_dn11), (locals.var_ps0ld_dn14 + locals.var_vxbgmtcl_dn14),)
    } else {
        (locals.var_ps0ld_vxb, locals.var_ps0ld_vxb_dn0, locals.var_ps0ld_vxb_dn2, locals.var_ps0ld_vxb_dn4, locals.var_ps0ld_vxb_dn5, locals.var_ps0ld_vxb_dn6, locals.var_ps0ld_vxb_dn7, locals.var_ps0ld_vxb_dn8, locals.var_ps0ld_vxb_dn9, locals.var_ps0ld_vxb_dn10, locals.var_ps0ld_vxb_dn11, locals.var_ps0ld_vxb_dn14,)
    }
};
        locals.var_ps0ld_vxb = assign83020_e126192;
        locals.var_ps0ld_vxb_dn0 = assign83020_e126192_d_n0;
        locals.var_ps0ld_vxb_dn2 = assign83020_e126192_d_n2;
        locals.var_ps0ld_vxb_dn4 = assign83020_e126192_d_n4;
        locals.var_ps0ld_vxb_dn5 = assign83020_e126192_d_n5;
        locals.var_ps0ld_vxb_dn6 = assign83020_e126192_d_n6;
        locals.var_ps0ld_vxb_dn7 = assign83020_e126192_d_n7;
        locals.var_ps0ld_vxb_dn8 = assign83020_e126192_d_n8;
        locals.var_ps0ld_vxb_dn9 = assign83020_e126192_d_n9;
        locals.var_ps0ld_vxb_dn10 = assign83020_e126192_d_n10;
        locals.var_ps0ld_vxb_dn11 = assign83020_e126192_d_n11;
        locals.var_ps0ld_vxb_dn14 = assign83020_e126192_d_n14;

        let (assign83030_e126205, assign83030_e126205_d_n0, assign83030_e126205_d_n2, assign83030_e126205_d_n4, assign83030_e126205_d_n5, assign83030_e126205_d_n6, assign83030_e126205_d_n7, assign83030_e126205_d_n8, assign83030_e126205_d_n9, assign83030_e126205_d_n10, assign83030_e126205_d_n11, assign83030_e126205_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1922 != 0.0)) {
        let assign83030_e126202: f64 = (locals.var_ps0ld_vxb - locals.var_dphi_sb);
        let assign83030_e126203: f64 = (locals.var_c_sb * assign83030_e126202);
        (assign83030_e126203, ((locals.var_c_sb_dn0 * assign83030_e126202) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn0 - locals.var_dphi_sb_dn0))), ((locals.var_c_sb_dn2 * assign83030_e126202) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn2 - locals.var_dphi_sb_dn2))), ((locals.var_c_sb_dn4 * assign83030_e126202) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn4 - locals.var_dphi_sb_dn4))), ((locals.var_c_sb_dn5 * assign83030_e126202) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn5 - locals.var_dphi_sb_dn5))), ((locals.var_c_sb_dn6 * assign83030_e126202) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn6 - locals.var_dphi_sb_dn6))), ((locals.var_c_sb_dn7 * assign83030_e126202) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn7 - locals.var_dphi_sb_dn7))), ((locals.var_c_sb_dn8 * assign83030_e126202) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn8 - locals.var_dphi_sb_dn8))), ((locals.var_c_sb_dn9 * assign83030_e126202) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn9 - locals.var_dphi_sb_dn9))), ((locals.var_c_sb_dn10 * assign83030_e126202) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn10 - locals.var_dphi_sb_dn10))), ((locals.var_c_sb_dn11 * assign83030_e126202) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn11 - locals.var_dphi_sb_dn11))), ((locals.var_c_sb_dn14 * assign83030_e126202) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn14 - locals.var_dphi_sb_dn14))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn14,)
    }
};
        locals.var_ty = assign83030_e126205;
        locals.var_ty_dn0 = assign83030_e126205_d_n0;
        locals.var_ty_dn2 = assign83030_e126205_d_n2;
        locals.var_ty_dn4 = assign83030_e126205_d_n4;
        locals.var_ty_dn5 = assign83030_e126205_d_n5;
        locals.var_ty_dn6 = assign83030_e126205_d_n6;
        locals.var_ty_dn7 = assign83030_e126205_d_n7;
        locals.var_ty_dn8 = assign83030_e126205_d_n8;
        locals.var_ty_dn9 = assign83030_e126205_d_n9;
        locals.var_ty_dn10 = assign83030_e126205_d_n10;
        locals.var_ty_dn11 = assign83030_e126205_d_n11;
        locals.var_ty_dn14 = assign83030_e126205_d_n14;

        let assign83040_e126208: f64 = if locals.var_ty < 60.0 { 1.0 } else { 0.0 };
        locals.var_guard1926 = assign83040_e126208;

        let (assign83050_e126220, assign83050_e126220_d_n0, assign83050_e126220_d_n2, assign83050_e126220_d_n4, assign83050_e126220_d_n5, assign83050_e126220_d_n6, assign83050_e126220_d_n7, assign83050_e126220_d_n8, assign83050_e126220_d_n9, assign83050_e126220_d_n10, assign83050_e126220_d_n11, assign83050_e126220_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1922 != 0.0)) && (locals.var_guard1926 != 0.0)) {
        let assign83050_e126218: f64 = (locals.var_ty).exp();
        (assign83050_e126218, (assign83050_e126218 * locals.var_ty_dn0), (assign83050_e126218 * locals.var_ty_dn2), (assign83050_e126218 * locals.var_ty_dn4), (assign83050_e126218 * locals.var_ty_dn5), (assign83050_e126218 * locals.var_ty_dn6), (assign83050_e126218 * locals.var_ty_dn7), (assign83050_e126218 * locals.var_ty_dn8), (assign83050_e126218 * locals.var_ty_dn9), (assign83050_e126218 * locals.var_ty_dn10), (assign83050_e126218 * locals.var_ty_dn11), (assign83050_e126218 * locals.var_ty_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign83050_e126220;
        locals.var_t1_dn0 = assign83050_e126220_d_n0;
        locals.var_t1_dn2 = assign83050_e126220_d_n2;
        locals.var_t1_dn4 = assign83050_e126220_d_n4;
        locals.var_t1_dn5 = assign83050_e126220_d_n5;
        locals.var_t1_dn6 = assign83050_e126220_d_n6;
        locals.var_t1_dn7 = assign83050_e126220_d_n7;
        locals.var_t1_dn8 = assign83050_e126220_d_n8;
        locals.var_t1_dn9 = assign83050_e126220_d_n9;
        locals.var_t1_dn10 = assign83050_e126220_d_n10;
        locals.var_t1_dn11 = assign83050_e126220_d_n11;
        locals.var_t1_dn14 = assign83050_e126220_d_n14;

        let (assign83060_e126235, assign83060_e126235_d_n0, assign83060_e126235_d_n2, assign83060_e126235_d_n4, assign83060_e126235_d_n5, assign83060_e126235_d_n6, assign83060_e126235_d_n7, assign83060_e126235_d_n8, assign83060_e126235_d_n9, assign83060_e126235_d_n10, assign83060_e126235_d_n11, assign83060_e126235_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1922 != 0.0)) && (locals.var_guard1926 != 0.0)) {
        let assign83060_e126230: f64 = (-locals.var_c_sb);
        let assign83060_e126232: f64 = (assign83060_e126230 * locals.var_dphi_sb);
        let assign83060_e126233: f64 = (assign83060_e126232).exp();
        (assign83060_e126233, (assign83060_e126233 * (((-locals.var_c_sb_dn0) * locals.var_dphi_sb) + (assign83060_e126230 * locals.var_dphi_sb_dn0))), (assign83060_e126233 * (((-locals.var_c_sb_dn2) * locals.var_dphi_sb) + (assign83060_e126230 * locals.var_dphi_sb_dn2))), (assign83060_e126233 * (((-locals.var_c_sb_dn4) * locals.var_dphi_sb) + (assign83060_e126230 * locals.var_dphi_sb_dn4))), (assign83060_e126233 * (((-locals.var_c_sb_dn5) * locals.var_dphi_sb) + (assign83060_e126230 * locals.var_dphi_sb_dn5))), (assign83060_e126233 * (((-locals.var_c_sb_dn6) * locals.var_dphi_sb) + (assign83060_e126230 * locals.var_dphi_sb_dn6))), (assign83060_e126233 * (((-locals.var_c_sb_dn7) * locals.var_dphi_sb) + (assign83060_e126230 * locals.var_dphi_sb_dn7))), (assign83060_e126233 * (((-locals.var_c_sb_dn8) * locals.var_dphi_sb) + (assign83060_e126230 * locals.var_dphi_sb_dn8))), (assign83060_e126233 * (((-locals.var_c_sb_dn9) * locals.var_dphi_sb) + (assign83060_e126230 * locals.var_dphi_sb_dn9))), (assign83060_e126233 * (((-locals.var_c_sb_dn10) * locals.var_dphi_sb) + (assign83060_e126230 * locals.var_dphi_sb_dn10))), (assign83060_e126233 * (((-locals.var_c_sb_dn11) * locals.var_dphi_sb) + (assign83060_e126230 * locals.var_dphi_sb_dn11))), (assign83060_e126233 * (((-locals.var_c_sb_dn14) * locals.var_dphi_sb) + (assign83060_e126230 * locals.var_dphi_sb_dn14))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign83060_e126235;
        locals.var_t0_dn0 = assign83060_e126235_d_n0;
        locals.var_t0_dn2 = assign83060_e126235_d_n2;
        locals.var_t0_dn4 = assign83060_e126235_d_n4;
        locals.var_t0_dn5 = assign83060_e126235_d_n5;
        locals.var_t0_dn6 = assign83060_e126235_d_n6;
        locals.var_t0_dn7 = assign83060_e126235_d_n7;
        locals.var_t0_dn8 = assign83060_e126235_d_n8;
        locals.var_t0_dn9 = assign83060_e126235_d_n9;
        locals.var_t0_dn10 = assign83060_e126235_d_n10;
        locals.var_t0_dn11 = assign83060_e126235_d_n11;
        locals.var_t0_dn14 = assign83060_e126235_d_n14;

        let (assign83070_e126248, assign83070_e126248_d_n0, assign83070_e126248_d_n2, assign83070_e126248_d_n4, assign83070_e126248_d_n5, assign83070_e126248_d_n6, assign83070_e126248_d_n7, assign83070_e126248_d_n8, assign83070_e126248_d_n9, assign83070_e126248_d_n10, assign83070_e126248_d_n11, assign83070_e126248_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1922 != 0.0)) && (locals.var_guard1926 != 0.0)) {
        let assign83070_e126246: f64 = (locals.var_t1 - locals.var_t0);
        (assign83070_e126246, (locals.var_t1_dn0 - locals.var_t0_dn0), (locals.var_t1_dn2 - locals.var_t0_dn2), (locals.var_t1_dn4 - locals.var_t0_dn4), (locals.var_t1_dn5 - locals.var_t0_dn5), (locals.var_t1_dn6 - locals.var_t0_dn6), (locals.var_t1_dn7 - locals.var_t0_dn7), (locals.var_t1_dn8 - locals.var_t0_dn8), (locals.var_t1_dn9 - locals.var_t0_dn9), (locals.var_t1_dn10 - locals.var_t0_dn10), (locals.var_t1_dn11 - locals.var_t0_dn11), (locals.var_t1_dn14 - locals.var_t0_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign83070_e126248;
        locals.var_t2_dn0 = assign83070_e126248_d_n0;
        locals.var_t2_dn2 = assign83070_e126248_d_n2;
        locals.var_t2_dn4 = assign83070_e126248_d_n4;
        locals.var_t2_dn5 = assign83070_e126248_d_n5;
        locals.var_t2_dn6 = assign83070_e126248_d_n6;
        locals.var_t2_dn7 = assign83070_e126248_d_n7;
        locals.var_t2_dn8 = assign83070_e126248_d_n8;
        locals.var_t2_dn9 = assign83070_e126248_d_n9;
        locals.var_t2_dn10 = assign83070_e126248_d_n10;
        locals.var_t2_dn11 = assign83070_e126248_d_n11;
        locals.var_t2_dn14 = assign83070_e126248_d_n14;

        let (assign83080_e126264, assign83080_e126264_d_n0, assign83080_e126264_d_n2, assign83080_e126264_d_n4, assign83080_e126264_d_n5, assign83080_e126264_d_n6, assign83080_e126264_d_n7, assign83080_e126264_d_n8, assign83080_e126264_d_n9, assign83080_e126264_d_n10, assign83080_e126264_d_n11, assign83080_e126264_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1922 != 0.0)) && (locals.var_guard1926 != 0.0)) {
        let assign83080_e126259: f64 = (1.0 + locals.var_t2);
        let assign83080_e126260: f64 = (assign83080_e126259).ln();
        let assign83080_e126262: f64 = (assign83080_e126260 / locals.var_c_sb);
        (assign83080_e126262, ((((locals.var_t2_dn0 / assign83080_e126259) * locals.var_c_sb) - (assign83080_e126260 * locals.var_c_sb_dn0)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn2 / assign83080_e126259) * locals.var_c_sb) - (assign83080_e126260 * locals.var_c_sb_dn2)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn4 / assign83080_e126259) * locals.var_c_sb) - (assign83080_e126260 * locals.var_c_sb_dn4)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn5 / assign83080_e126259) * locals.var_c_sb) - (assign83080_e126260 * locals.var_c_sb_dn5)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn6 / assign83080_e126259) * locals.var_c_sb) - (assign83080_e126260 * locals.var_c_sb_dn6)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn7 / assign83080_e126259) * locals.var_c_sb) - (assign83080_e126260 * locals.var_c_sb_dn7)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn8 / assign83080_e126259) * locals.var_c_sb) - (assign83080_e126260 * locals.var_c_sb_dn8)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn9 / assign83080_e126259) * locals.var_c_sb) - (assign83080_e126260 * locals.var_c_sb_dn9)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn10 / assign83080_e126259) * locals.var_c_sb) - (assign83080_e126260 * locals.var_c_sb_dn10)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn11 / assign83080_e126259) * locals.var_c_sb) - (assign83080_e126260 * locals.var_c_sb_dn11)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn14 / assign83080_e126259) * locals.var_c_sb) - (assign83080_e126260 * locals.var_c_sb_dn14)) / (locals.var_c_sb * locals.var_c_sb)),)
    } else {
        (locals.var_phi_b, locals.var_phi_b_dn0, locals.var_phi_b_dn2, locals.var_phi_b_dn4, locals.var_phi_b_dn5, locals.var_phi_b_dn6, locals.var_phi_b_dn7, locals.var_phi_b_dn8, locals.var_phi_b_dn9, locals.var_phi_b_dn10, locals.var_phi_b_dn11, locals.var_phi_b_dn14,)
    }
};
        locals.var_phi_b = assign83080_e126264;
        locals.var_phi_b_dn0 = assign83080_e126264_d_n0;
        locals.var_phi_b_dn2 = assign83080_e126264_d_n2;
        locals.var_phi_b_dn4 = assign83080_e126264_d_n4;
        locals.var_phi_b_dn5 = assign83080_e126264_d_n5;
        locals.var_phi_b_dn6 = assign83080_e126264_d_n6;
        locals.var_phi_b_dn7 = assign83080_e126264_d_n7;
        locals.var_phi_b_dn8 = assign83080_e126264_d_n8;
        locals.var_phi_b_dn9 = assign83080_e126264_d_n9;
        locals.var_phi_b_dn10 = assign83080_e126264_d_n10;
        locals.var_phi_b_dn11 = assign83080_e126264_d_n11;
        locals.var_phi_b_dn14 = assign83080_e126264_d_n14;

        let (assign83090_e126278, assign83090_e126278_d_n0, assign83090_e126278_d_n2, assign83090_e126278_d_n4, assign83090_e126278_d_n5, assign83090_e126278_d_n6, assign83090_e126278_d_n7, assign83090_e126278_d_n8, assign83090_e126278_d_n9, assign83090_e126278_d_n10, assign83090_e126278_d_n11, assign83090_e126278_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1922 != 0.0)) && (locals.var_guard1926 == 0.0)) {
        let assign83090_e126276: f64 = (locals.var_ps0ld_vxb - locals.var_dphi_sb);
        (assign83090_e126276, (locals.var_ps0ld_vxb_dn0 - locals.var_dphi_sb_dn0), (locals.var_ps0ld_vxb_dn2 - locals.var_dphi_sb_dn2), (locals.var_ps0ld_vxb_dn4 - locals.var_dphi_sb_dn4), (locals.var_ps0ld_vxb_dn5 - locals.var_dphi_sb_dn5), (locals.var_ps0ld_vxb_dn6 - locals.var_dphi_sb_dn6), (locals.var_ps0ld_vxb_dn7 - locals.var_dphi_sb_dn7), (locals.var_ps0ld_vxb_dn8 - locals.var_dphi_sb_dn8), (locals.var_ps0ld_vxb_dn9 - locals.var_dphi_sb_dn9), (locals.var_ps0ld_vxb_dn10 - locals.var_dphi_sb_dn10), (locals.var_ps0ld_vxb_dn11 - locals.var_dphi_sb_dn11), (locals.var_ps0ld_vxb_dn14 - locals.var_dphi_sb_dn14),)
    } else {
        (locals.var_phi_b, locals.var_phi_b_dn0, locals.var_phi_b_dn2, locals.var_phi_b_dn4, locals.var_phi_b_dn5, locals.var_phi_b_dn6, locals.var_phi_b_dn7, locals.var_phi_b_dn8, locals.var_phi_b_dn9, locals.var_phi_b_dn10, locals.var_phi_b_dn11, locals.var_phi_b_dn14,)
    }
};
        locals.var_phi_b = assign83090_e126278;
        locals.var_phi_b_dn0 = assign83090_e126278_d_n0;
        locals.var_phi_b_dn2 = assign83090_e126278_d_n2;
        locals.var_phi_b_dn4 = assign83090_e126278_d_n4;
        locals.var_phi_b_dn5 = assign83090_e126278_d_n5;
        locals.var_phi_b_dn6 = assign83090_e126278_d_n6;
        locals.var_phi_b_dn7 = assign83090_e126278_d_n7;
        locals.var_phi_b_dn8 = assign83090_e126278_d_n8;
        locals.var_phi_b_dn9 = assign83090_e126278_d_n9;
        locals.var_phi_b_dn10 = assign83090_e126278_d_n10;
        locals.var_phi_b_dn11 = assign83090_e126278_d_n11;
        locals.var_phi_b_dn14 = assign83090_e126278_d_n14;

        let (assign83100_e126289, assign83100_e126289_d_n0, assign83100_e126289_d_n2, assign83100_e126289_d_n4, assign83100_e126289_d_n5, assign83100_e126289_d_n6, assign83100_e126289_d_n7, assign83100_e126289_d_n8, assign83100_e126289_d_n9, assign83100_e126289_d_n10, assign83100_e126289_d_n11, assign83100_e126289_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1922 != 0.0)) {
        let assign83100_e126287: f64 = (locals.var_beta * locals.var_phi_b);
        (assign83100_e126287, ((locals.var_beta_dn0 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn0)), ((locals.var_beta_dn2 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn2)), ((locals.var_beta_dn4 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn4)), ((locals.var_beta_dn5 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn5)), ((locals.var_beta_dn6 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn6)), ((locals.var_beta_dn7 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn7)), ((locals.var_beta_dn8 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn8)), ((locals.var_beta_dn9 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn9)), ((locals.var_beta_dn10 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn10)), ((locals.var_beta_dn11 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn11)), ((locals.var_beta_dn14 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn14)),)
    } else {
        (locals.var_chib, locals.var_chib_dn0, locals.var_chib_dn2, locals.var_chib_dn4, locals.var_chib_dn5, locals.var_chib_dn6, locals.var_chib_dn7, locals.var_chib_dn8, locals.var_chib_dn9, locals.var_chib_dn10, locals.var_chib_dn11, locals.var_chib_dn14,)
    }
};
        locals.var_chib = assign83100_e126289;
        locals.var_chib_dn0 = assign83100_e126289_d_n0;
        locals.var_chib_dn2 = assign83100_e126289_d_n2;
        locals.var_chib_dn4 = assign83100_e126289_d_n4;
        locals.var_chib_dn5 = assign83100_e126289_d_n5;
        locals.var_chib_dn6 = assign83100_e126289_d_n6;
        locals.var_chib_dn7 = assign83100_e126289_d_n7;
        locals.var_chib_dn8 = assign83100_e126289_d_n8;
        locals.var_chib_dn9 = assign83100_e126289_d_n9;
        locals.var_chib_dn10 = assign83100_e126289_d_n10;
        locals.var_chib_dn11 = assign83100_e126289_d_n11;
        locals.var_chib_dn14 = assign83100_e126289_d_n14;

        let assign83110_e126293: f64 = (locals.var_chi / 100.0);
        let assign83110_e126298: f64 = if ((locals.var_chib > assign83110_e126293) && (locals.var_chib > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1927 = assign83110_e126298;

        let (assign83120_e126311,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1922 != 0.0)) && (locals.var_guard1927 != 0.0)) {
        let assign83120_e126309: f64 = (locals.var_flg_fd_mode__blk1891 + 1.0);
        (assign83120_e126309,)
    } else {
        (locals.var_flg_fd_mode__blk1891,)
    }
};
        locals.var_flg_fd_mode__blk1891 = assign83120_e126311;

    }

    pub(super) fn stamp_transient_block_301(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign83130_e126322, assign83130_e126322_d_n0, assign83130_e126322_d_n2, assign83130_e126322_d_n4, assign83130_e126322_d_n5, assign83130_e126322_d_n6, assign83130_e126322_d_n7, assign83130_e126322_d_n8, assign83130_e126322_d_n9, assign83130_e126322_d_n10, assign83130_e126322_d_n11, assign83130_e126322_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1922 != 0.0)) && (locals.var_guard1927 != 0.0)) {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn11, locals.var_chi_1_dn14,)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
        locals.var_chi = assign83130_e126322;
        locals.var_chi_dn0 = assign83130_e126322_d_n0;
        locals.var_chi_dn2 = assign83130_e126322_d_n2;
        locals.var_chi_dn4 = assign83130_e126322_d_n4;
        locals.var_chi_dn5 = assign83130_e126322_d_n5;
        locals.var_chi_dn6 = assign83130_e126322_d_n6;
        locals.var_chi_dn7 = assign83130_e126322_d_n7;
        locals.var_chi_dn8 = assign83130_e126322_d_n8;
        locals.var_chi_dn9 = assign83130_e126322_d_n9;
        locals.var_chi_dn10 = assign83130_e126322_d_n10;
        locals.var_chi_dn11 = assign83130_e126322_d_n11;
        locals.var_chi_dn14 = assign83130_e126322_d_n14;

        let (assign83140_e126333, assign83140_e126333_d_n0, assign83140_e126333_d_n2, assign83140_e126333_d_n4, assign83140_e126333_d_n5, assign83140_e126333_d_n6, assign83140_e126333_d_n7, assign83140_e126333_d_n8, assign83140_e126333_d_n9, assign83140_e126333_d_n10, assign83140_e126333_d_n11, assign83140_e126333_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) {
        let assign83140_e126329: f64 = (locals.var_chi * locals.var_beta_inv);
        let assign83140_e126331: f64 = (assign83140_e126329 - locals.var_vxbgmtcl);
        (assign83140_e126331, (((locals.var_chi_dn0 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn0)) - locals.var_vxbgmtcl_dn0), (((locals.var_chi_dn2 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn2)) - locals.var_vxbgmtcl_dn2), (((locals.var_chi_dn4 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn4)) - locals.var_vxbgmtcl_dn4), (((locals.var_chi_dn5 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn5)) - locals.var_vxbgmtcl_dn5), (((locals.var_chi_dn6 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn6)) - locals.var_vxbgmtcl_dn6), (((locals.var_chi_dn7 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn7)) - locals.var_vxbgmtcl_dn7), (((locals.var_chi_dn8 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn8)) - locals.var_vxbgmtcl_dn8), (((locals.var_chi_dn9 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn9)) - locals.var_vxbgmtcl_dn9), (((locals.var_chi_dn10 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn10)) - locals.var_vxbgmtcl_dn10), (((locals.var_chi_dn11 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn11)) - locals.var_vxbgmtcl_dn11), (((locals.var_chi_dn14 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn14)) - locals.var_vxbgmtcl_dn14),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn14,)
    }
};
        locals.var_ps0ld = assign83140_e126333;
        locals.var_ps0ld_dn0 = assign83140_e126333_d_n0;
        locals.var_ps0ld_dn2 = assign83140_e126333_d_n2;
        locals.var_ps0ld_dn4 = assign83140_e126333_d_n4;
        locals.var_ps0ld_dn5 = assign83140_e126333_d_n5;
        locals.var_ps0ld_dn6 = assign83140_e126333_d_n6;
        locals.var_ps0ld_dn7 = assign83140_e126333_d_n7;
        locals.var_ps0ld_dn8 = assign83140_e126333_d_n8;
        locals.var_ps0ld_dn9 = assign83140_e126333_d_n9;
        locals.var_ps0ld_dn10 = assign83140_e126333_d_n10;
        locals.var_ps0ld_dn11 = assign83140_e126333_d_n11;
        locals.var_ps0ld_dn14 = assign83140_e126333_d_n14;

        let assign83150_e126335: f64 = (locals.var_chi).abs();
        let assign83150_e126337: f64 = if assign83150_e126335 > 1e-6 { 1.0 } else { 0.0 };
        locals.var_guard1928 = assign83150_e126337;

        let (assign83160_e126352, assign83160_e126352_d_n0, assign83160_e126352_d_n2, assign83160_e126352_d_n4, assign83160_e126352_d_n5, assign83160_e126352_d_n6, assign83160_e126352_d_n7, assign83160_e126352_d_n8, assign83160_e126352_d_n9, assign83160_e126352_d_n10, assign83160_e126352_d_n11, assign83160_e126352_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1928 != 0.0)) {
        let assign83160_e126346: f64 = (locals.var_chi - 1.0);
        let assign83160_e126348: f64 = (-locals.var_chi);
        let assign83160_e126349: f64 = (assign83160_e126348).exp();
        let assign83160_e126350: f64 = (assign83160_e126346 + assign83160_e126349);
        (assign83160_e126350, (locals.var_chi_dn0 + (assign83160_e126349 * (-locals.var_chi_dn0))), (locals.var_chi_dn2 + (assign83160_e126349 * (-locals.var_chi_dn2))), (locals.var_chi_dn4 + (assign83160_e126349 * (-locals.var_chi_dn4))), (locals.var_chi_dn5 + (assign83160_e126349 * (-locals.var_chi_dn5))), (locals.var_chi_dn6 + (assign83160_e126349 * (-locals.var_chi_dn6))), (locals.var_chi_dn7 + (assign83160_e126349 * (-locals.var_chi_dn7))), (locals.var_chi_dn8 + (assign83160_e126349 * (-locals.var_chi_dn8))), (locals.var_chi_dn9 + (assign83160_e126349 * (-locals.var_chi_dn9))), (locals.var_chi_dn10 + (assign83160_e126349 * (-locals.var_chi_dn10))), (locals.var_chi_dn11 + (assign83160_e126349 * (-locals.var_chi_dn11))), (locals.var_chi_dn14 + (assign83160_e126349 * (-locals.var_chi_dn14))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign83160_e126352;
        locals.var_t1_dn0 = assign83160_e126352_d_n0;
        locals.var_t1_dn2 = assign83160_e126352_d_n2;
        locals.var_t1_dn4 = assign83160_e126352_d_n4;
        locals.var_t1_dn5 = assign83160_e126352_d_n5;
        locals.var_t1_dn6 = assign83160_e126352_d_n6;
        locals.var_t1_dn7 = assign83160_e126352_d_n7;
        locals.var_t1_dn8 = assign83160_e126352_d_n8;
        locals.var_t1_dn9 = assign83160_e126352_d_n9;
        locals.var_t1_dn10 = assign83160_e126352_d_n10;
        locals.var_t1_dn11 = assign83160_e126352_d_n11;
        locals.var_t1_dn14 = assign83160_e126352_d_n14;

        let (assign83170_e126362, assign83170_e126362_d_n0, assign83170_e126362_d_n2, assign83170_e126362_d_n4, assign83170_e126362_d_n5, assign83170_e126362_d_n6, assign83170_e126362_d_n7, assign83170_e126362_d_n8, assign83170_e126362_d_n9, assign83170_e126362_d_n10, assign83170_e126362_d_n11, assign83170_e126362_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1928 != 0.0)) {
        let assign83170_e126360: f64 = (locals.var_t1).sqrt();
        (assign83170_e126360, (locals.var_t1_dn0 / (2.0 * assign83170_e126360)), (locals.var_t1_dn2 / (2.0 * assign83170_e126360)), (locals.var_t1_dn4 / (2.0 * assign83170_e126360)), (locals.var_t1_dn5 / (2.0 * assign83170_e126360)), (locals.var_t1_dn6 / (2.0 * assign83170_e126360)), (locals.var_t1_dn7 / (2.0 * assign83170_e126360)), (locals.var_t1_dn8 / (2.0 * assign83170_e126360)), (locals.var_t1_dn9 / (2.0 * assign83170_e126360)), (locals.var_t1_dn10 / (2.0 * assign83170_e126360)), (locals.var_t1_dn11 / (2.0 * assign83170_e126360)), (locals.var_t1_dn14 / (2.0 * assign83170_e126360)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign83170_e126362;
        locals.var_t2_dn0 = assign83170_e126362_d_n0;
        locals.var_t2_dn2 = assign83170_e126362_d_n2;
        locals.var_t2_dn4 = assign83170_e126362_d_n4;
        locals.var_t2_dn5 = assign83170_e126362_d_n5;
        locals.var_t2_dn6 = assign83170_e126362_d_n6;
        locals.var_t2_dn7 = assign83170_e126362_d_n7;
        locals.var_t2_dn8 = assign83170_e126362_d_n8;
        locals.var_t2_dn9 = assign83170_e126362_d_n9;
        locals.var_t2_dn10 = assign83170_e126362_d_n10;
        locals.var_t2_dn11 = assign83170_e126362_d_n11;
        locals.var_t2_dn14 = assign83170_e126362_d_n14;

        let (assign83190_e126393, assign83190_e126393_d_n0, assign83190_e126393_d_n2, assign83190_e126393_d_n4, assign83190_e126393_d_n5, assign83190_e126393_d_n6, assign83190_e126393_d_n7, assign83190_e126393_d_n8, assign83190_e126393_d_n9, assign83190_e126393_d_n10, assign83190_e126393_d_n11, assign83190_e126393_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1928 == 0.0)) {
        let assign83190_e126384: f64 = (0.7071067811865475 * locals.var_chi);
        let assign83190_e126388: f64 = (locals.var_chi * 0.3333333333333333);
        let assign83190_e126389: f64 = (1.0 - assign83190_e126388);
        let assign83190_e126390: f64 = (assign83190_e126389).sqrt();
        let assign83190_e126391: f64 = (assign83190_e126384 * assign83190_e126390);
        (assign83190_e126391, (((0.7071067811865475 * locals.var_chi_dn0) * assign83190_e126390) + (assign83190_e126384 * ((-(locals.var_chi_dn0 * 0.3333333333333333)) / (2.0 * assign83190_e126390)))), (((0.7071067811865475 * locals.var_chi_dn2) * assign83190_e126390) + (assign83190_e126384 * ((-(locals.var_chi_dn2 * 0.3333333333333333)) / (2.0 * assign83190_e126390)))), (((0.7071067811865475 * locals.var_chi_dn4) * assign83190_e126390) + (assign83190_e126384 * ((-(locals.var_chi_dn4 * 0.3333333333333333)) / (2.0 * assign83190_e126390)))), (((0.7071067811865475 * locals.var_chi_dn5) * assign83190_e126390) + (assign83190_e126384 * ((-(locals.var_chi_dn5 * 0.3333333333333333)) / (2.0 * assign83190_e126390)))), (((0.7071067811865475 * locals.var_chi_dn6) * assign83190_e126390) + (assign83190_e126384 * ((-(locals.var_chi_dn6 * 0.3333333333333333)) / (2.0 * assign83190_e126390)))), (((0.7071067811865475 * locals.var_chi_dn7) * assign83190_e126390) + (assign83190_e126384 * ((-(locals.var_chi_dn7 * 0.3333333333333333)) / (2.0 * assign83190_e126390)))), (((0.7071067811865475 * locals.var_chi_dn8) * assign83190_e126390) + (assign83190_e126384 * ((-(locals.var_chi_dn8 * 0.3333333333333333)) / (2.0 * assign83190_e126390)))), (((0.7071067811865475 * locals.var_chi_dn9) * assign83190_e126390) + (assign83190_e126384 * ((-(locals.var_chi_dn9 * 0.3333333333333333)) / (2.0 * assign83190_e126390)))), (((0.7071067811865475 * locals.var_chi_dn10) * assign83190_e126390) + (assign83190_e126384 * ((-(locals.var_chi_dn10 * 0.3333333333333333)) / (2.0 * assign83190_e126390)))), (((0.7071067811865475 * locals.var_chi_dn11) * assign83190_e126390) + (assign83190_e126384 * ((-(locals.var_chi_dn11 * 0.3333333333333333)) / (2.0 * assign83190_e126390)))), (((0.7071067811865475 * locals.var_chi_dn14) * assign83190_e126390) + (assign83190_e126384 * ((-(locals.var_chi_dn14 * 0.3333333333333333)) / (2.0 * assign83190_e126390)))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign83190_e126393;
        locals.var_t2_dn0 = assign83190_e126393_d_n0;
        locals.var_t2_dn2 = assign83190_e126393_d_n2;
        locals.var_t2_dn4 = assign83190_e126393_d_n4;
        locals.var_t2_dn5 = assign83190_e126393_d_n5;
        locals.var_t2_dn6 = assign83190_e126393_d_n6;
        locals.var_t2_dn7 = assign83190_e126393_d_n7;
        locals.var_t2_dn8 = assign83190_e126393_d_n8;
        locals.var_t2_dn9 = assign83190_e126393_d_n9;
        locals.var_t2_dn10 = assign83190_e126393_d_n10;
        locals.var_t2_dn11 = assign83190_e126393_d_n11;
        locals.var_t2_dn14 = assign83190_e126393_d_n14;

        let (assign83200_e126402, assign83200_e126402_d_n0, assign83200_e126402_d_n2, assign83200_e126402_d_n4, assign83200_e126402_d_n5, assign83200_e126402_d_n6, assign83200_e126402_d_n7, assign83200_e126402_d_n8, assign83200_e126402_d_n9, assign83200_e126402_d_n10, assign83200_e126402_d_n11, assign83200_e126402_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) {
        let assign83200_e126400: f64 = (locals.var_cnst0over_func * locals.var_t2);
        (assign83200_e126400, ((locals.var_cnst0over_func_dn0 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn0)), ((locals.var_cnst0over_func_dn2 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn2)), ((locals.var_cnst0over_func_dn4 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn4)), ((locals.var_cnst0over_func_dn5 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn5)), ((locals.var_cnst0over_func_dn6 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn6)), ((locals.var_cnst0over_func_dn7 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn7)), ((locals.var_cnst0over_func_dn8 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn8)), ((locals.var_cnst0over_func_dn9 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn9)), ((locals.var_cnst0over_func_dn10 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn10)), ((locals.var_cnst0over_func_dn11 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn11)), ((locals.var_cnst0over_func_dn14 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn14)),)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn8, locals.var_qbuld_dn9, locals.var_qbuld_dn10, locals.var_qbuld_dn11, locals.var_qbuld_dn14,)
    }
};
        locals.var_qbuld = assign83200_e126402;
        locals.var_qbuld_dn0 = assign83200_e126402_d_n0;
        locals.var_qbuld_dn2 = assign83200_e126402_d_n2;
        locals.var_qbuld_dn4 = assign83200_e126402_d_n4;
        locals.var_qbuld_dn5 = assign83200_e126402_d_n5;
        locals.var_qbuld_dn6 = assign83200_e126402_d_n6;
        locals.var_qbuld_dn7 = assign83200_e126402_d_n7;
        locals.var_qbuld_dn8 = assign83200_e126402_d_n8;
        locals.var_qbuld_dn9 = assign83200_e126402_d_n9;
        locals.var_qbuld_dn10 = assign83200_e126402_d_n10;
        locals.var_qbuld_dn11 = assign83200_e126402_d_n11;
        locals.var_qbuld_dn14 = assign83200_e126402_d_n14;

        let (assign83210_e126413, assign83210_e126413_d_n0, assign83210_e126413_d_n2, assign83210_e126413_d_n4, assign83210_e126413_d_n5, assign83210_e126413_d_n6, assign83210_e126413_d_n7, assign83210_e126413_d_n8, assign83210_e126413_d_n9, assign83210_e126413_d_n10, assign83210_e126413_d_n11, assign83210_e126413_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) {
        let assign83210_e126410: f64 = (locals.var_vgpld - locals.var_ps0ld);
        let assign83210_e126411: f64 = (locals.var_cox0_func * assign83210_e126410);
        (assign83210_e126411, (locals.var_cox0_func * (-locals.var_ps0ld_dn0)), (locals.var_cox0_func * (locals.var_vgpld_dn2 - locals.var_ps0ld_dn2)), (locals.var_cox0_func * (-locals.var_ps0ld_dn4)), (locals.var_cox0_func * (-locals.var_ps0ld_dn5)), (locals.var_cox0_func * (-locals.var_ps0ld_dn6)), (locals.var_cox0_func * (locals.var_vgpld_dn7 - locals.var_ps0ld_dn7)), (locals.var_cox0_func * (locals.var_vgpld_dn8 - locals.var_ps0ld_dn8)), (locals.var_cox0_func * (locals.var_vgpld_dn9 - locals.var_ps0ld_dn9)), (locals.var_cox0_func * (-locals.var_ps0ld_dn10)), (locals.var_cox0_func * (-locals.var_ps0ld_dn11)), (locals.var_cox0_func * (-locals.var_ps0ld_dn14)),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn4, locals.var_qsuld_dn5, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn8, locals.var_qsuld_dn9, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn14,)
    }
};
        locals.var_qsuld = assign83210_e126413;
        locals.var_qsuld_dn0 = assign83210_e126413_d_n0;
        locals.var_qsuld_dn2 = assign83210_e126413_d_n2;
        locals.var_qsuld_dn4 = assign83210_e126413_d_n4;
        locals.var_qsuld_dn5 = assign83210_e126413_d_n5;
        locals.var_qsuld_dn6 = assign83210_e126413_d_n6;
        locals.var_qsuld_dn7 = assign83210_e126413_d_n7;
        locals.var_qsuld_dn8 = assign83210_e126413_d_n8;
        locals.var_qsuld_dn9 = assign83210_e126413_d_n9;
        locals.var_qsuld_dn10 = assign83210_e126413_d_n10;
        locals.var_qsuld_dn11 = assign83210_e126413_d_n11;
        locals.var_qsuld_dn14 = assign83210_e126413_d_n14;

        let (assign83220_e126422, assign83220_e126422_d_n0, assign83220_e126422_d_n2, assign83220_e126422_d_n4, assign83220_e126422_d_n5, assign83220_e126422_d_n6, assign83220_e126422_d_n7, assign83220_e126422_d_n8, assign83220_e126422_d_n9, assign83220_e126422_d_n10, assign83220_e126422_d_n11, assign83220_e126422_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) {
        let assign83220_e126420: f64 = (locals.var_qbuld / locals.var_q_nsubld__blk1885);
        (assign83220_e126420, (locals.var_qbuld_dn0 / locals.var_q_nsubld__blk1885), (locals.var_qbuld_dn2 / locals.var_q_nsubld__blk1885), (locals.var_qbuld_dn4 / locals.var_q_nsubld__blk1885), (locals.var_qbuld_dn5 / locals.var_q_nsubld__blk1885), (locals.var_qbuld_dn6 / locals.var_q_nsubld__blk1885), (locals.var_qbuld_dn7 / locals.var_q_nsubld__blk1885), (locals.var_qbuld_dn8 / locals.var_q_nsubld__blk1885), (locals.var_qbuld_dn9 / locals.var_q_nsubld__blk1885), (locals.var_qbuld_dn10 / locals.var_q_nsubld__blk1885), (locals.var_qbuld_dn11 / locals.var_q_nsubld__blk1885), (locals.var_qbuld_dn14 / locals.var_q_nsubld__blk1885),)
    } else {
        (locals.var_wdld0__blk1929, locals.var_wdld0__blk1929_dn0, locals.var_wdld0__blk1929_dn2, locals.var_wdld0__blk1929_dn4, locals.var_wdld0__blk1929_dn5, locals.var_wdld0__blk1929_dn6, locals.var_wdld0__blk1929_dn7, locals.var_wdld0__blk1929_dn8, locals.var_wdld0__blk1929_dn9, locals.var_wdld0__blk1929_dn10, locals.var_wdld0__blk1929_dn11, locals.var_wdld0__blk1929_dn14,)
    }
};
        locals.var_wdld0__blk1929 = assign83220_e126422;
        locals.var_wdld0__blk1929_dn0 = assign83220_e126422_d_n0;
        locals.var_wdld0__blk1929_dn2 = assign83220_e126422_d_n2;
        locals.var_wdld0__blk1929_dn4 = assign83220_e126422_d_n4;
        locals.var_wdld0__blk1929_dn5 = assign83220_e126422_d_n5;
        locals.var_wdld0__blk1929_dn6 = assign83220_e126422_d_n6;
        locals.var_wdld0__blk1929_dn7 = assign83220_e126422_d_n7;
        locals.var_wdld0__blk1929_dn8 = assign83220_e126422_d_n8;
        locals.var_wdld0__blk1929_dn9 = assign83220_e126422_d_n9;
        locals.var_wdld0__blk1929_dn10 = assign83220_e126422_d_n10;
        locals.var_wdld0__blk1929_dn11 = assign83220_e126422_d_n11;
        locals.var_wdld0__blk1929_dn14 = assign83220_e126422_d_n14;

        let assign83230_e126425: f64 = if p.p33 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1931 = assign83230_e126425;

        let assign83240_e126430: f64 = (locals.var_ddriftldc * 0.1);
        let assign83240_e126431: f64 = (locals.var_ddriftldc - assign83240_e126430);
        let assign83240_e126435: f64 = (locals.var_ddriftldc * 0.1);
        let assign83240_e126438: f64 = if ((locals.var_wdld0__blk1929 > assign83240_e126431) && (assign83240_e126435 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1932 = assign83240_e126438;

        let (assign83250_e126455, assign83250_e126455_d_n0, assign83250_e126455_d_n2, assign83250_e126455_d_n4, assign83250_e126455_d_n5, assign83250_e126455_d_n6, assign83250_e126455_d_n7, assign83250_e126455_d_n8, assign83250_e126455_d_n9, assign83250_e126455_d_n10, assign83250_e126455_d_n11, assign83250_e126455_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1931 != 0.0)) && (locals.var_guard1932 != 0.0)) {
        let assign83250_e126449: f64 = (locals.var_wdld0__blk1929 - locals.var_ddriftldc);
        let assign83250_e126452: f64 = (locals.var_ddriftldc * 0.1);
        let assign83250_e126453: f64 = (assign83250_e126449 + assign83250_e126452);
        (assign83250_e126453, ((locals.var_wdld0__blk1929_dn0 - locals.var_ddriftldc_dn0) + (locals.var_ddriftldc_dn0 * 0.1)), ((locals.var_wdld0__blk1929_dn2 - locals.var_ddriftldc_dn2) + (locals.var_ddriftldc_dn2 * 0.1)), ((locals.var_wdld0__blk1929_dn4 - locals.var_ddriftldc_dn4) + (locals.var_ddriftldc_dn4 * 0.1)), ((locals.var_wdld0__blk1929_dn5 - locals.var_ddriftldc_dn5) + (locals.var_ddriftldc_dn5 * 0.1)), ((locals.var_wdld0__blk1929_dn6 - locals.var_ddriftldc_dn6) + (locals.var_ddriftldc_dn6 * 0.1)), ((locals.var_wdld0__blk1929_dn7 - locals.var_ddriftldc_dn7) + (locals.var_ddriftldc_dn7 * 0.1)), ((locals.var_wdld0__blk1929_dn8 - locals.var_ddriftldc_dn8) + (locals.var_ddriftldc_dn8 * 0.1)), ((locals.var_wdld0__blk1929_dn9 - locals.var_ddriftldc_dn9) + (locals.var_ddriftldc_dn9 * 0.1)), ((locals.var_wdld0__blk1929_dn10 - locals.var_ddriftldc_dn10) + (locals.var_ddriftldc_dn10 * 0.1)), ((locals.var_wdld0__blk1929_dn11 - locals.var_ddriftldc_dn11) + (locals.var_ddriftldc_dn11 * 0.1)), ((locals.var_wdld0__blk1929_dn14 - locals.var_ddriftldc_dn14) + (locals.var_ddriftldc_dn14 * 0.1)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign83250_e126455;
        locals.var_tmf1_dn0 = assign83250_e126455_d_n0;
        locals.var_tmf1_dn2 = assign83250_e126455_d_n2;
        locals.var_tmf1_dn4 = assign83250_e126455_d_n4;
        locals.var_tmf1_dn5 = assign83250_e126455_d_n5;
        locals.var_tmf1_dn6 = assign83250_e126455_d_n6;
        locals.var_tmf1_dn7 = assign83250_e126455_d_n7;
        locals.var_tmf1_dn8 = assign83250_e126455_d_n8;
        locals.var_tmf1_dn9 = assign83250_e126455_d_n9;
        locals.var_tmf1_dn10 = assign83250_e126455_d_n10;
        locals.var_tmf1_dn11 = assign83250_e126455_d_n11;
        locals.var_tmf1_dn14 = assign83250_e126455_d_n14;

        let (assign83260_e126468, assign83260_e126468_d_n0, assign83260_e126468_d_n2, assign83260_e126468_d_n4, assign83260_e126468_d_n5, assign83260_e126468_d_n6, assign83260_e126468_d_n7, assign83260_e126468_d_n8, assign83260_e126468_d_n9, assign83260_e126468_d_n10, assign83260_e126468_d_n11, assign83260_e126468_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1931 != 0.0)) && (locals.var_guard1932 != 0.0)) {
        let assign83260_e126466: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign83260_e126466, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign83260_e126468;
        locals.var_x2_dn0 = assign83260_e126468_d_n0;
        locals.var_x2_dn2 = assign83260_e126468_d_n2;
        locals.var_x2_dn4 = assign83260_e126468_d_n4;
        locals.var_x2_dn5 = assign83260_e126468_d_n5;
        locals.var_x2_dn6 = assign83260_e126468_d_n6;
        locals.var_x2_dn7 = assign83260_e126468_d_n7;
        locals.var_x2_dn8 = assign83260_e126468_d_n8;
        locals.var_x2_dn9 = assign83260_e126468_d_n9;
        locals.var_x2_dn10 = assign83260_e126468_d_n10;
        locals.var_x2_dn11 = assign83260_e126468_d_n11;
        locals.var_x2_dn14 = assign83260_e126468_d_n14;

        let (assign83270_e126485, assign83270_e126485_d_n0, assign83270_e126485_d_n2, assign83270_e126485_d_n4, assign83270_e126485_d_n5, assign83270_e126485_d_n6, assign83270_e126485_d_n7, assign83270_e126485_d_n8, assign83270_e126485_d_n9, assign83270_e126485_d_n10, assign83270_e126485_d_n11, assign83270_e126485_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1931 != 0.0)) && (locals.var_guard1932 != 0.0)) {
        let assign83270_e126479: f64 = (locals.var_ddriftldc * 0.1);
        let assign83270_e126482: f64 = (locals.var_ddriftldc * 0.1);
        let assign83270_e126483: f64 = (assign83270_e126479 * assign83270_e126482);
        (assign83270_e126483, (((locals.var_ddriftldc_dn0 * 0.1) * assign83270_e126482) + (assign83270_e126479 * (locals.var_ddriftldc_dn0 * 0.1))), (((locals.var_ddriftldc_dn2 * 0.1) * assign83270_e126482) + (assign83270_e126479 * (locals.var_ddriftldc_dn2 * 0.1))), (((locals.var_ddriftldc_dn4 * 0.1) * assign83270_e126482) + (assign83270_e126479 * (locals.var_ddriftldc_dn4 * 0.1))), (((locals.var_ddriftldc_dn5 * 0.1) * assign83270_e126482) + (assign83270_e126479 * (locals.var_ddriftldc_dn5 * 0.1))), (((locals.var_ddriftldc_dn6 * 0.1) * assign83270_e126482) + (assign83270_e126479 * (locals.var_ddriftldc_dn6 * 0.1))), (((locals.var_ddriftldc_dn7 * 0.1) * assign83270_e126482) + (assign83270_e126479 * (locals.var_ddriftldc_dn7 * 0.1))), (((locals.var_ddriftldc_dn8 * 0.1) * assign83270_e126482) + (assign83270_e126479 * (locals.var_ddriftldc_dn8 * 0.1))), (((locals.var_ddriftldc_dn9 * 0.1) * assign83270_e126482) + (assign83270_e126479 * (locals.var_ddriftldc_dn9 * 0.1))), (((locals.var_ddriftldc_dn10 * 0.1) * assign83270_e126482) + (assign83270_e126479 * (locals.var_ddriftldc_dn10 * 0.1))), (((locals.var_ddriftldc_dn11 * 0.1) * assign83270_e126482) + (assign83270_e126479 * (locals.var_ddriftldc_dn11 * 0.1))), (((locals.var_ddriftldc_dn14 * 0.1) * assign83270_e126482) + (assign83270_e126479 * (locals.var_ddriftldc_dn14 * 0.1))),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign83270_e126485;
        locals.var_xmax2_dn0 = assign83270_e126485_d_n0;
        locals.var_xmax2_dn2 = assign83270_e126485_d_n2;
        locals.var_xmax2_dn4 = assign83270_e126485_d_n4;
        locals.var_xmax2_dn5 = assign83270_e126485_d_n5;
        locals.var_xmax2_dn6 = assign83270_e126485_d_n6;
        locals.var_xmax2_dn7 = assign83270_e126485_d_n7;
        locals.var_xmax2_dn8 = assign83270_e126485_d_n8;
        locals.var_xmax2_dn9 = assign83270_e126485_d_n9;
        locals.var_xmax2_dn10 = assign83270_e126485_d_n10;
        locals.var_xmax2_dn11 = assign83270_e126485_d_n11;
        locals.var_xmax2_dn14 = assign83270_e126485_d_n14;

        let (assign83280_e126496, assign83280_e126496_d_n0, assign83280_e126496_d_n2, assign83280_e126496_d_n4, assign83280_e126496_d_n5, assign83280_e126496_d_n6, assign83280_e126496_d_n7, assign83280_e126496_d_n8, assign83280_e126496_d_n9, assign83280_e126496_d_n10, assign83280_e126496_d_n11, assign83280_e126496_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1931 != 0.0)) && (locals.var_guard1932 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign83280_e126496;
        locals.var_xp_dn0 = assign83280_e126496_d_n0;
        locals.var_xp_dn2 = assign83280_e126496_d_n2;
        locals.var_xp_dn4 = assign83280_e126496_d_n4;
        locals.var_xp_dn5 = assign83280_e126496_d_n5;
        locals.var_xp_dn6 = assign83280_e126496_d_n6;
        locals.var_xp_dn7 = assign83280_e126496_d_n7;
        locals.var_xp_dn8 = assign83280_e126496_d_n8;
        locals.var_xp_dn9 = assign83280_e126496_d_n9;
        locals.var_xp_dn10 = assign83280_e126496_d_n10;
        locals.var_xp_dn11 = assign83280_e126496_d_n11;
        locals.var_xp_dn14 = assign83280_e126496_d_n14;

        let (assign83290_e126507, assign83290_e126507_d_n0, assign83290_e126507_d_n2, assign83290_e126507_d_n4, assign83290_e126507_d_n5, assign83290_e126507_d_n6, assign83290_e126507_d_n7, assign83290_e126507_d_n8, assign83290_e126507_d_n9, assign83290_e126507_d_n10, assign83290_e126507_d_n11, assign83290_e126507_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1931 != 0.0)) && (locals.var_guard1932 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign83290_e126507;
        locals.var_xmp_dn0 = assign83290_e126507_d_n0;
        locals.var_xmp_dn2 = assign83290_e126507_d_n2;
        locals.var_xmp_dn4 = assign83290_e126507_d_n4;
        locals.var_xmp_dn5 = assign83290_e126507_d_n5;
        locals.var_xmp_dn6 = assign83290_e126507_d_n6;
        locals.var_xmp_dn7 = assign83290_e126507_d_n7;
        locals.var_xmp_dn8 = assign83290_e126507_d_n8;
        locals.var_xmp_dn9 = assign83290_e126507_d_n9;
        locals.var_xmp_dn10 = assign83290_e126507_d_n10;
        locals.var_xmp_dn11 = assign83290_e126507_d_n11;
        locals.var_xmp_dn14 = assign83290_e126507_d_n14;

        let (assign83300_e126518,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1931 != 0.0)) && (locals.var_guard1932 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign83300_e126518;

        let (assign83310_e126529,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1931 != 0.0)) && (locals.var_guard1932 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign83310_e126529;

        let (assign83320_e126540, assign83320_e126540_d_n0, assign83320_e126540_d_n2, assign83320_e126540_d_n4, assign83320_e126540_d_n5, assign83320_e126540_d_n6, assign83320_e126540_d_n7, assign83320_e126540_d_n8, assign83320_e126540_d_n9, assign83320_e126540_d_n10, assign83320_e126540_d_n11, assign83320_e126540_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1931 != 0.0)) && (locals.var_guard1932 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign83320_e126540;
        locals.var_arg_dn0 = assign83320_e126540_d_n0;
        locals.var_arg_dn2 = assign83320_e126540_d_n2;
        locals.var_arg_dn4 = assign83320_e126540_d_n4;
        locals.var_arg_dn5 = assign83320_e126540_d_n5;
        locals.var_arg_dn6 = assign83320_e126540_d_n6;
        locals.var_arg_dn7 = assign83320_e126540_d_n7;
        locals.var_arg_dn8 = assign83320_e126540_d_n8;
        locals.var_arg_dn9 = assign83320_e126540_d_n9;
        locals.var_arg_dn10 = assign83320_e126540_d_n10;
        locals.var_arg_dn11 = assign83320_e126540_d_n11;
        locals.var_arg_dn14 = assign83320_e126540_d_n14;

        let (assign83330_e126551, assign83330_e126551_d_n0, assign83330_e126551_d_n2, assign83330_e126551_d_n4, assign83330_e126551_d_n5, assign83330_e126551_d_n6, assign83330_e126551_d_n7, assign83330_e126551_d_n8, assign83330_e126551_d_n9, assign83330_e126551_d_n10, assign83330_e126551_d_n11, assign83330_e126551_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1931 != 0.0)) && (locals.var_guard1932 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign83330_e126551;
        locals.var_dnm_dn0 = assign83330_e126551_d_n0;
        locals.var_dnm_dn2 = assign83330_e126551_d_n2;
        locals.var_dnm_dn4 = assign83330_e126551_d_n4;
        locals.var_dnm_dn5 = assign83330_e126551_d_n5;
        locals.var_dnm_dn6 = assign83330_e126551_d_n6;
        locals.var_dnm_dn7 = assign83330_e126551_d_n7;
        locals.var_dnm_dn8 = assign83330_e126551_d_n8;
        locals.var_dnm_dn9 = assign83330_e126551_d_n9;
        locals.var_dnm_dn10 = assign83330_e126551_d_n10;
        locals.var_dnm_dn11 = assign83330_e126551_d_n11;
        locals.var_dnm_dn14 = assign83330_e126551_d_n14;

        let (assign83340_e126564, assign83340_e126564_d_n0, assign83340_e126564_d_n2, assign83340_e126564_d_n4, assign83340_e126564_d_n5, assign83340_e126564_d_n6, assign83340_e126564_d_n7, assign83340_e126564_d_n8, assign83340_e126564_d_n9, assign83340_e126564_d_n10, assign83340_e126564_d_n11, assign83340_e126564_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1931 != 0.0)) && (locals.var_guard1932 != 0.0)) {
        let assign83340_e126562: f64 = (locals.var_xp * locals.var_x2);
        (assign83340_e126562, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign83340_e126564;
        locals.var_xp_dn0 = assign83340_e126564_d_n0;
        locals.var_xp_dn2 = assign83340_e126564_d_n2;
        locals.var_xp_dn4 = assign83340_e126564_d_n4;
        locals.var_xp_dn5 = assign83340_e126564_d_n5;
        locals.var_xp_dn6 = assign83340_e126564_d_n6;
        locals.var_xp_dn7 = assign83340_e126564_d_n7;
        locals.var_xp_dn8 = assign83340_e126564_d_n8;
        locals.var_xp_dn9 = assign83340_e126564_d_n9;
        locals.var_xp_dn10 = assign83340_e126564_d_n10;
        locals.var_xp_dn11 = assign83340_e126564_d_n11;
        locals.var_xp_dn14 = assign83340_e126564_d_n14;

        let (assign83350_e126577, assign83350_e126577_d_n0, assign83350_e126577_d_n2, assign83350_e126577_d_n4, assign83350_e126577_d_n5, assign83350_e126577_d_n6, assign83350_e126577_d_n7, assign83350_e126577_d_n8, assign83350_e126577_d_n9, assign83350_e126577_d_n10, assign83350_e126577_d_n11, assign83350_e126577_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1931 != 0.0)) && (locals.var_guard1932 != 0.0)) {
        let assign83350_e126575: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign83350_e126575, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign83350_e126577;
        locals.var_xmp_dn0 = assign83350_e126577_d_n0;
        locals.var_xmp_dn2 = assign83350_e126577_d_n2;
        locals.var_xmp_dn4 = assign83350_e126577_d_n4;
        locals.var_xmp_dn5 = assign83350_e126577_d_n5;
        locals.var_xmp_dn6 = assign83350_e126577_d_n6;
        locals.var_xmp_dn7 = assign83350_e126577_d_n7;
        locals.var_xmp_dn8 = assign83350_e126577_d_n8;
        locals.var_xmp_dn9 = assign83350_e126577_d_n9;
        locals.var_xmp_dn10 = assign83350_e126577_d_n10;
        locals.var_xmp_dn11 = assign83350_e126577_d_n11;
        locals.var_xmp_dn14 = assign83350_e126577_d_n14;

        let (assign83360_e126590, assign83360_e126590_d_n0, assign83360_e126590_d_n2, assign83360_e126590_d_n4, assign83360_e126590_d_n5, assign83360_e126590_d_n6, assign83360_e126590_d_n7, assign83360_e126590_d_n8, assign83360_e126590_d_n9, assign83360_e126590_d_n10, assign83360_e126590_d_n11, assign83360_e126590_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1931 != 0.0)) && (locals.var_guard1932 != 0.0)) {
        let assign83360_e126588: f64 = (locals.var_xp * locals.var_x2);
        (assign83360_e126588, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign83360_e126590;
        locals.var_xp_dn0 = assign83360_e126590_d_n0;
        locals.var_xp_dn2 = assign83360_e126590_d_n2;
        locals.var_xp_dn4 = assign83360_e126590_d_n4;
        locals.var_xp_dn5 = assign83360_e126590_d_n5;
        locals.var_xp_dn6 = assign83360_e126590_d_n6;
        locals.var_xp_dn7 = assign83360_e126590_d_n7;
        locals.var_xp_dn8 = assign83360_e126590_d_n8;
        locals.var_xp_dn9 = assign83360_e126590_d_n9;
        locals.var_xp_dn10 = assign83360_e126590_d_n10;
        locals.var_xp_dn11 = assign83360_e126590_d_n11;
        locals.var_xp_dn14 = assign83360_e126590_d_n14;

        let (assign83370_e126603, assign83370_e126603_d_n0, assign83370_e126603_d_n2, assign83370_e126603_d_n4, assign83370_e126603_d_n5, assign83370_e126603_d_n6, assign83370_e126603_d_n7, assign83370_e126603_d_n8, assign83370_e126603_d_n9, assign83370_e126603_d_n10, assign83370_e126603_d_n11, assign83370_e126603_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1931 != 0.0)) && (locals.var_guard1932 != 0.0)) {
        let assign83370_e126601: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign83370_e126601, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign83370_e126603;
        locals.var_xmp_dn0 = assign83370_e126603_d_n0;
        locals.var_xmp_dn2 = assign83370_e126603_d_n2;
        locals.var_xmp_dn4 = assign83370_e126603_d_n4;
        locals.var_xmp_dn5 = assign83370_e126603_d_n5;
        locals.var_xmp_dn6 = assign83370_e126603_d_n6;
        locals.var_xmp_dn7 = assign83370_e126603_d_n7;
        locals.var_xmp_dn8 = assign83370_e126603_d_n8;
        locals.var_xmp_dn9 = assign83370_e126603_d_n9;
        locals.var_xmp_dn10 = assign83370_e126603_d_n10;
        locals.var_xmp_dn11 = assign83370_e126603_d_n11;
        locals.var_xmp_dn14 = assign83370_e126603_d_n14;

        let (assign83380_e126616, assign83380_e126616_d_n0, assign83380_e126616_d_n2, assign83380_e126616_d_n4, assign83380_e126616_d_n5, assign83380_e126616_d_n6, assign83380_e126616_d_n7, assign83380_e126616_d_n8, assign83380_e126616_d_n9, assign83380_e126616_d_n10, assign83380_e126616_d_n11, assign83380_e126616_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1931 != 0.0)) && (locals.var_guard1932 != 0.0)) {
        let assign83380_e126614: f64 = (locals.var_xp + locals.var_xmp);
        (assign83380_e126614, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign83380_e126616;
        locals.var_arg_dn0 = assign83380_e126616_d_n0;
        locals.var_arg_dn2 = assign83380_e126616_d_n2;
        locals.var_arg_dn4 = assign83380_e126616_d_n4;
        locals.var_arg_dn5 = assign83380_e126616_d_n5;
        locals.var_arg_dn6 = assign83380_e126616_d_n6;
        locals.var_arg_dn7 = assign83380_e126616_d_n7;
        locals.var_arg_dn8 = assign83380_e126616_d_n8;
        locals.var_arg_dn9 = assign83380_e126616_d_n9;
        locals.var_arg_dn10 = assign83380_e126616_d_n10;
        locals.var_arg_dn11 = assign83380_e126616_d_n11;
        locals.var_arg_dn14 = assign83380_e126616_d_n14;

        let (assign83390_e126627, assign83390_e126627_d_n0, assign83390_e126627_d_n2, assign83390_e126627_d_n4, assign83390_e126627_d_n5, assign83390_e126627_d_n6, assign83390_e126627_d_n7, assign83390_e126627_d_n8, assign83390_e126627_d_n9, assign83390_e126627_d_n10, assign83390_e126627_d_n11, assign83390_e126627_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1931 != 0.0)) && (locals.var_guard1932 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign83390_e126627;
        locals.var_dnm_dn0 = assign83390_e126627_d_n0;
        locals.var_dnm_dn2 = assign83390_e126627_d_n2;
        locals.var_dnm_dn4 = assign83390_e126627_d_n4;
        locals.var_dnm_dn5 = assign83390_e126627_d_n5;
        locals.var_dnm_dn6 = assign83390_e126627_d_n6;
        locals.var_dnm_dn7 = assign83390_e126627_d_n7;
        locals.var_dnm_dn8 = assign83390_e126627_d_n8;
        locals.var_dnm_dn9 = assign83390_e126627_d_n9;
        locals.var_dnm_dn10 = assign83390_e126627_d_n10;
        locals.var_dnm_dn11 = assign83390_e126627_d_n11;
        locals.var_dnm_dn14 = assign83390_e126627_d_n14;

        let assign83400_e126642: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1933 = assign83400_e126642;

        let assign83410_e126645: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1934 = assign83410_e126645;

        let (assign83420_e126660,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1931 != 0.0)) && (locals.var_guard1932 != 0.0)) && (locals.var_guard1933 != 0.0)) && (locals.var_guard1934 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign83420_e126660;

        let assign83430_e126663: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1935 = assign83430_e126663;

        let (assign83440_e126681,) = {
    if (((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1931 != 0.0)) && (locals.var_guard1932 != 0.0)) && (locals.var_guard1933 != 0.0)) && (locals.var_guard1934 == 0.0)) && (locals.var_guard1935 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign83440_e126681;

        let assign83450_e126684: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1936 = assign83450_e126684;

    }

    pub(super) fn stamp_transient_block_302(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign83460_e126705,) = {
    if ((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1931 != 0.0)) && (locals.var_guard1932 != 0.0)) && (locals.var_guard1933 != 0.0)) && (locals.var_guard1934 == 0.0)) && (locals.var_guard1935 == 0.0)) && (locals.var_guard1936 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign83460_e126705;

        let assign83470_e126708: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1937 = assign83470_e126708;

        let (assign83480_e126732,) = {
    if (((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1931 != 0.0)) && (locals.var_guard1932 != 0.0)) && (locals.var_guard1933 != 0.0)) && (locals.var_guard1934 == 0.0)) && (locals.var_guard1935 == 0.0)) && (locals.var_guard1936 == 0.0)) && (locals.var_guard1937 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign83480_e126732;

        let (assign83490_e126745,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1931 != 0.0)) && (locals.var_guard1932 != 0.0)) && (locals.var_guard1933 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign83490_e126745;

        let mut assign83500_loop_guard: usize = 0;
        while {
            let assign83500_cond_e126759: f64 = if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1931 != 0.0)) && (locals.var_guard1932 != 0.0)) && (locals.var_guard1933 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign83500_cond_e126759 != 0.0
        } {
            assign83500_loop_guard += 1;
            assert!(assign83500_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign83500_body0_e126773, assign83500_body0_e126773_d_n0, assign83500_body0_e126773_d_n2, assign83500_body0_e126773_d_n4, assign83500_body0_e126773_d_n5, assign83500_body0_e126773_d_n6, assign83500_body0_e126773_d_n7, assign83500_body0_e126773_d_n8, assign83500_body0_e126773_d_n9, assign83500_body0_e126773_d_n10, assign83500_body0_e126773_d_n11, assign83500_body0_e126773_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1931 != 0.0)) && (locals.var_guard1932 != 0.0)) && (locals.var_guard1933 != 0.0)) {
        let assign83500_body0_e126771: f64 = (locals.var_dnm).sqrt();
        (assign83500_body0_e126771, (locals.var_dnm_dn0 / (2.0 * assign83500_body0_e126771)), (locals.var_dnm_dn2 / (2.0 * assign83500_body0_e126771)), (locals.var_dnm_dn4 / (2.0 * assign83500_body0_e126771)), (locals.var_dnm_dn5 / (2.0 * assign83500_body0_e126771)), (locals.var_dnm_dn6 / (2.0 * assign83500_body0_e126771)), (locals.var_dnm_dn7 / (2.0 * assign83500_body0_e126771)), (locals.var_dnm_dn8 / (2.0 * assign83500_body0_e126771)), (locals.var_dnm_dn9 / (2.0 * assign83500_body0_e126771)), (locals.var_dnm_dn10 / (2.0 * assign83500_body0_e126771)), (locals.var_dnm_dn11 / (2.0 * assign83500_body0_e126771)), (locals.var_dnm_dn14 / (2.0 * assign83500_body0_e126771)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign83500_body0_e126773;
            locals.var_dnm_dn0 = assign83500_body0_e126773_d_n0;
            locals.var_dnm_dn2 = assign83500_body0_e126773_d_n2;
            locals.var_dnm_dn4 = assign83500_body0_e126773_d_n4;
            locals.var_dnm_dn5 = assign83500_body0_e126773_d_n5;
            locals.var_dnm_dn6 = assign83500_body0_e126773_d_n6;
            locals.var_dnm_dn7 = assign83500_body0_e126773_d_n7;
            locals.var_dnm_dn8 = assign83500_body0_e126773_d_n8;
            locals.var_dnm_dn9 = assign83500_body0_e126773_d_n9;
            locals.var_dnm_dn10 = assign83500_body0_e126773_d_n10;
            locals.var_dnm_dn11 = assign83500_body0_e126773_d_n11;
            locals.var_dnm_dn14 = assign83500_body0_e126773_d_n14;
            let (assign83500_body1_e126788,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1931 != 0.0)) && (locals.var_guard1932 != 0.0)) && (locals.var_guard1933 != 0.0)) {
        let assign83500_body1_e126786: f64 = (locals.var_m0 + 1.0);
        (assign83500_body1_e126786,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign83500_body1_e126788;
        }

        let (assign83510_e126813, assign83510_e126813_d_n0, assign83510_e126813_d_n2, assign83510_e126813_d_n4, assign83510_e126813_d_n5, assign83510_e126813_d_n6, assign83510_e126813_d_n7, assign83510_e126813_d_n8, assign83510_e126813_d_n9, assign83510_e126813_d_n10, assign83510_e126813_d_n11, assign83510_e126813_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1931 != 0.0)) && (locals.var_guard1932 != 0.0)) && (locals.var_guard1933 == 0.0)) {
        let (assign83510_e126811, assign83510_e126811_d_n0, assign83510_e126811_d_n2, assign83510_e126811_d_n4, assign83510_e126811_d_n5, assign83510_e126811_d_n6, assign83510_e126811_d_n7, assign83510_e126811_d_n8, assign83510_e126811_d_n9, assign83510_e126811_d_n10, assign83510_e126811_d_n11, assign83510_e126811_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign83510_e126808: f64 = (2.0 * 2.0);
                let assign83510_e126809: f64 = (1.0 / assign83510_e126808);
                let assign83510_e126810: f64 = (locals.var_dnm).powf(assign83510_e126809);
                (assign83510_e126810, if 0.0 == 0.0 && ((assign83510_e126809) as f64).is_finite() && ((assign83510_e126809) as f64).fract() == 0.0 { if assign83510_e126809 == 0.0 { 0.0 } else { (assign83510_e126809 * ((locals.var_dnm).powf(assign83510_e126809 - 1.0) * locals.var_dnm_dn0)) } } else { (assign83510_e126810 * (assign83510_e126809 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign83510_e126809) as f64).is_finite() && ((assign83510_e126809) as f64).fract() == 0.0 { if assign83510_e126809 == 0.0 { 0.0 } else { (assign83510_e126809 * ((locals.var_dnm).powf(assign83510_e126809 - 1.0) * locals.var_dnm_dn2)) } } else { (assign83510_e126810 * (assign83510_e126809 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign83510_e126809) as f64).is_finite() && ((assign83510_e126809) as f64).fract() == 0.0 { if assign83510_e126809 == 0.0 { 0.0 } else { (assign83510_e126809 * ((locals.var_dnm).powf(assign83510_e126809 - 1.0) * locals.var_dnm_dn4)) } } else { (assign83510_e126810 * (assign83510_e126809 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign83510_e126809) as f64).is_finite() && ((assign83510_e126809) as f64).fract() == 0.0 { if assign83510_e126809 == 0.0 { 0.0 } else { (assign83510_e126809 * ((locals.var_dnm).powf(assign83510_e126809 - 1.0) * locals.var_dnm_dn5)) } } else { (assign83510_e126810 * (assign83510_e126809 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign83510_e126809) as f64).is_finite() && ((assign83510_e126809) as f64).fract() == 0.0 { if assign83510_e126809 == 0.0 { 0.0 } else { (assign83510_e126809 * ((locals.var_dnm).powf(assign83510_e126809 - 1.0) * locals.var_dnm_dn6)) } } else { (assign83510_e126810 * (assign83510_e126809 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign83510_e126809) as f64).is_finite() && ((assign83510_e126809) as f64).fract() == 0.0 { if assign83510_e126809 == 0.0 { 0.0 } else { (assign83510_e126809 * ((locals.var_dnm).powf(assign83510_e126809 - 1.0) * locals.var_dnm_dn7)) } } else { (assign83510_e126810 * (assign83510_e126809 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign83510_e126809) as f64).is_finite() && ((assign83510_e126809) as f64).fract() == 0.0 { if assign83510_e126809 == 0.0 { 0.0 } else { (assign83510_e126809 * ((locals.var_dnm).powf(assign83510_e126809 - 1.0) * locals.var_dnm_dn8)) } } else { (assign83510_e126810 * (assign83510_e126809 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign83510_e126809) as f64).is_finite() && ((assign83510_e126809) as f64).fract() == 0.0 { if assign83510_e126809 == 0.0 { 0.0 } else { (assign83510_e126809 * ((locals.var_dnm).powf(assign83510_e126809 - 1.0) * locals.var_dnm_dn9)) } } else { (assign83510_e126810 * (assign83510_e126809 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign83510_e126809) as f64).is_finite() && ((assign83510_e126809) as f64).fract() == 0.0 { if assign83510_e126809 == 0.0 { 0.0 } else { (assign83510_e126809 * ((locals.var_dnm).powf(assign83510_e126809 - 1.0) * locals.var_dnm_dn10)) } } else { (assign83510_e126810 * (assign83510_e126809 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign83510_e126809) as f64).is_finite() && ((assign83510_e126809) as f64).fract() == 0.0 { if assign83510_e126809 == 0.0 { 0.0 } else { (assign83510_e126809 * ((locals.var_dnm).powf(assign83510_e126809 - 1.0) * locals.var_dnm_dn11)) } } else { (assign83510_e126810 * (assign83510_e126809 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign83510_e126809) as f64).is_finite() && ((assign83510_e126809) as f64).fract() == 0.0 { if assign83510_e126809 == 0.0 { 0.0 } else { (assign83510_e126809 * ((locals.var_dnm).powf(assign83510_e126809 - 1.0) * locals.var_dnm_dn14)) } } else { (assign83510_e126810 * (assign83510_e126809 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign83510_e126811, assign83510_e126811_d_n0, assign83510_e126811_d_n2, assign83510_e126811_d_n4, assign83510_e126811_d_n5, assign83510_e126811_d_n6, assign83510_e126811_d_n7, assign83510_e126811_d_n8, assign83510_e126811_d_n9, assign83510_e126811_d_n10, assign83510_e126811_d_n11, assign83510_e126811_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign83510_e126813;
        locals.var_dnm_dn0 = assign83510_e126813_d_n0;
        locals.var_dnm_dn2 = assign83510_e126813_d_n2;
        locals.var_dnm_dn4 = assign83510_e126813_d_n4;
        locals.var_dnm_dn5 = assign83510_e126813_d_n5;
        locals.var_dnm_dn6 = assign83510_e126813_d_n6;
        locals.var_dnm_dn7 = assign83510_e126813_d_n7;
        locals.var_dnm_dn8 = assign83510_e126813_d_n8;
        locals.var_dnm_dn9 = assign83510_e126813_d_n9;
        locals.var_dnm_dn10 = assign83510_e126813_d_n10;
        locals.var_dnm_dn11 = assign83510_e126813_d_n11;
        locals.var_dnm_dn14 = assign83510_e126813_d_n14;

        let (assign83520_e126826, assign83520_e126826_d_n0, assign83520_e126826_d_n2, assign83520_e126826_d_n4, assign83520_e126826_d_n5, assign83520_e126826_d_n6, assign83520_e126826_d_n7, assign83520_e126826_d_n8, assign83520_e126826_d_n9, assign83520_e126826_d_n10, assign83520_e126826_d_n11, assign83520_e126826_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1931 != 0.0)) && (locals.var_guard1932 != 0.0)) {
        let assign83520_e126824: f64 = (1.0 / locals.var_dnm);
        (assign83520_e126824, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign83520_e126826;
        locals.var_dnm_dn0 = assign83520_e126826_d_n0;
        locals.var_dnm_dn2 = assign83520_e126826_d_n2;
        locals.var_dnm_dn4 = assign83520_e126826_d_n4;
        locals.var_dnm_dn5 = assign83520_e126826_d_n5;
        locals.var_dnm_dn6 = assign83520_e126826_d_n6;
        locals.var_dnm_dn7 = assign83520_e126826_d_n7;
        locals.var_dnm_dn8 = assign83520_e126826_d_n8;
        locals.var_dnm_dn9 = assign83520_e126826_d_n9;
        locals.var_dnm_dn10 = assign83520_e126826_d_n10;
        locals.var_dnm_dn11 = assign83520_e126826_d_n11;
        locals.var_dnm_dn14 = assign83520_e126826_d_n14;

        let (assign83530_e126843, assign83530_e126843_d_n0, assign83530_e126843_d_n2, assign83530_e126843_d_n4, assign83530_e126843_d_n5, assign83530_e126843_d_n6, assign83530_e126843_d_n7, assign83530_e126843_d_n8, assign83530_e126843_d_n9, assign83530_e126843_d_n10, assign83530_e126843_d_n11, assign83530_e126843_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1931 != 0.0)) && (locals.var_guard1932 != 0.0)) {
        let assign83530_e126838: f64 = (locals.var_ddriftldc * 0.1);
        let assign83530_e126839: f64 = (locals.var_tmf1 * assign83530_e126838);
        let assign83530_e126841: f64 = (assign83530_e126839 * locals.var_dnm);
        (assign83530_e126841, ((((locals.var_tmf1_dn0 * assign83530_e126838) + (locals.var_tmf1 * (locals.var_ddriftldc_dn0 * 0.1))) * locals.var_dnm) + (assign83530_e126839 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * assign83530_e126838) + (locals.var_tmf1 * (locals.var_ddriftldc_dn2 * 0.1))) * locals.var_dnm) + (assign83530_e126839 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn4 * assign83530_e126838) + (locals.var_tmf1 * (locals.var_ddriftldc_dn4 * 0.1))) * locals.var_dnm) + (assign83530_e126839 * locals.var_dnm_dn4)), ((((locals.var_tmf1_dn5 * assign83530_e126838) + (locals.var_tmf1 * (locals.var_ddriftldc_dn5 * 0.1))) * locals.var_dnm) + (assign83530_e126839 * locals.var_dnm_dn5)), ((((locals.var_tmf1_dn6 * assign83530_e126838) + (locals.var_tmf1 * (locals.var_ddriftldc_dn6 * 0.1))) * locals.var_dnm) + (assign83530_e126839 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * assign83530_e126838) + (locals.var_tmf1 * (locals.var_ddriftldc_dn7 * 0.1))) * locals.var_dnm) + (assign83530_e126839 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn8 * assign83530_e126838) + (locals.var_tmf1 * (locals.var_ddriftldc_dn8 * 0.1))) * locals.var_dnm) + (assign83530_e126839 * locals.var_dnm_dn8)), ((((locals.var_tmf1_dn9 * assign83530_e126838) + (locals.var_tmf1 * (locals.var_ddriftldc_dn9 * 0.1))) * locals.var_dnm) + (assign83530_e126839 * locals.var_dnm_dn9)), ((((locals.var_tmf1_dn10 * assign83530_e126838) + (locals.var_tmf1 * (locals.var_ddriftldc_dn10 * 0.1))) * locals.var_dnm) + (assign83530_e126839 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn11 * assign83530_e126838) + (locals.var_tmf1 * (locals.var_ddriftldc_dn11 * 0.1))) * locals.var_dnm) + (assign83530_e126839 * locals.var_dnm_dn11)), ((((locals.var_tmf1_dn14 * assign83530_e126838) + (locals.var_tmf1 * (locals.var_ddriftldc_dn14 * 0.1))) * locals.var_dnm) + (assign83530_e126839 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign83530_e126843;
        locals.var_tmf0_dn0 = assign83530_e126843_d_n0;
        locals.var_tmf0_dn2 = assign83530_e126843_d_n2;
        locals.var_tmf0_dn4 = assign83530_e126843_d_n4;
        locals.var_tmf0_dn5 = assign83530_e126843_d_n5;
        locals.var_tmf0_dn6 = assign83530_e126843_d_n6;
        locals.var_tmf0_dn7 = assign83530_e126843_d_n7;
        locals.var_tmf0_dn8 = assign83530_e126843_d_n8;
        locals.var_tmf0_dn9 = assign83530_e126843_d_n9;
        locals.var_tmf0_dn10 = assign83530_e126843_d_n10;
        locals.var_tmf0_dn11 = assign83530_e126843_d_n11;
        locals.var_tmf0_dn14 = assign83530_e126843_d_n14;

        let (assign83540_e126862, assign83540_e126862_d_n0, assign83540_e126862_d_n2, assign83540_e126862_d_n4, assign83540_e126862_d_n5, assign83540_e126862_d_n6, assign83540_e126862_d_n7, assign83540_e126862_d_n8, assign83540_e126862_d_n9, assign83540_e126862_d_n10, assign83540_e126862_d_n11, assign83540_e126862_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1931 != 0.0)) && (locals.var_guard1932 != 0.0)) {
        let assign83540_e126854: f64 = (locals.var_ddriftldc * 0.1);
        let assign83540_e126856: f64 = (assign83540_e126854 * locals.var_xmp);
        let assign83540_e126858: f64 = (assign83540_e126856 * locals.var_dnm);
        let assign83540_e126860: f64 = (assign83540_e126858 / locals.var_arg);
        (assign83540_e126860, ((((((((locals.var_ddriftldc_dn0 * 0.1) * locals.var_xmp) + (assign83540_e126854 * locals.var_xmp_dn0)) * locals.var_dnm) + (assign83540_e126856 * locals.var_dnm_dn0)) * locals.var_arg) - (assign83540_e126858 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn2 * 0.1) * locals.var_xmp) + (assign83540_e126854 * locals.var_xmp_dn2)) * locals.var_dnm) + (assign83540_e126856 * locals.var_dnm_dn2)) * locals.var_arg) - (assign83540_e126858 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn4 * 0.1) * locals.var_xmp) + (assign83540_e126854 * locals.var_xmp_dn4)) * locals.var_dnm) + (assign83540_e126856 * locals.var_dnm_dn4)) * locals.var_arg) - (assign83540_e126858 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn5 * 0.1) * locals.var_xmp) + (assign83540_e126854 * locals.var_xmp_dn5)) * locals.var_dnm) + (assign83540_e126856 * locals.var_dnm_dn5)) * locals.var_arg) - (assign83540_e126858 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn6 * 0.1) * locals.var_xmp) + (assign83540_e126854 * locals.var_xmp_dn6)) * locals.var_dnm) + (assign83540_e126856 * locals.var_dnm_dn6)) * locals.var_arg) - (assign83540_e126858 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn7 * 0.1) * locals.var_xmp) + (assign83540_e126854 * locals.var_xmp_dn7)) * locals.var_dnm) + (assign83540_e126856 * locals.var_dnm_dn7)) * locals.var_arg) - (assign83540_e126858 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn8 * 0.1) * locals.var_xmp) + (assign83540_e126854 * locals.var_xmp_dn8)) * locals.var_dnm) + (assign83540_e126856 * locals.var_dnm_dn8)) * locals.var_arg) - (assign83540_e126858 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn9 * 0.1) * locals.var_xmp) + (assign83540_e126854 * locals.var_xmp_dn9)) * locals.var_dnm) + (assign83540_e126856 * locals.var_dnm_dn9)) * locals.var_arg) - (assign83540_e126858 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn10 * 0.1) * locals.var_xmp) + (assign83540_e126854 * locals.var_xmp_dn10)) * locals.var_dnm) + (assign83540_e126856 * locals.var_dnm_dn10)) * locals.var_arg) - (assign83540_e126858 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn11 * 0.1) * locals.var_xmp) + (assign83540_e126854 * locals.var_xmp_dn11)) * locals.var_dnm) + (assign83540_e126856 * locals.var_dnm_dn11)) * locals.var_arg) - (assign83540_e126858 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn14 * 0.1) * locals.var_xmp) + (assign83540_e126854 * locals.var_xmp_dn14)) * locals.var_dnm) + (assign83540_e126856 * locals.var_dnm_dn14)) * locals.var_arg) - (assign83540_e126858 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign83540_e126862;
        locals.var_t0_dn0 = assign83540_e126862_d_n0;
        locals.var_t0_dn2 = assign83540_e126862_d_n2;
        locals.var_t0_dn4 = assign83540_e126862_d_n4;
        locals.var_t0_dn5 = assign83540_e126862_d_n5;
        locals.var_t0_dn6 = assign83540_e126862_d_n6;
        locals.var_t0_dn7 = assign83540_e126862_d_n7;
        locals.var_t0_dn8 = assign83540_e126862_d_n8;
        locals.var_t0_dn9 = assign83540_e126862_d_n9;
        locals.var_t0_dn10 = assign83540_e126862_d_n10;
        locals.var_t0_dn11 = assign83540_e126862_d_n11;
        locals.var_t0_dn14 = assign83540_e126862_d_n14;

        let (assign83550_e126879, assign83550_e126879_d_n0, assign83550_e126879_d_n2, assign83550_e126879_d_n4, assign83550_e126879_d_n5, assign83550_e126879_d_n6, assign83550_e126879_d_n7, assign83550_e126879_d_n8, assign83550_e126879_d_n9, assign83550_e126879_d_n10, assign83550_e126879_d_n11, assign83550_e126879_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1931 != 0.0)) && (locals.var_guard1932 != 0.0)) {
        let assign83550_e126874: f64 = (locals.var_ddriftldc * 0.1);
        let assign83550_e126875: f64 = (locals.var_ddriftldc - assign83550_e126874);
        let assign83550_e126877: f64 = (assign83550_e126875 + locals.var_tmf0);
        (assign83550_e126877, ((locals.var_ddriftldc_dn0 - (locals.var_ddriftldc_dn0 * 0.1)) + locals.var_tmf0_dn0), ((locals.var_ddriftldc_dn2 - (locals.var_ddriftldc_dn2 * 0.1)) + locals.var_tmf0_dn2), ((locals.var_ddriftldc_dn4 - (locals.var_ddriftldc_dn4 * 0.1)) + locals.var_tmf0_dn4), ((locals.var_ddriftldc_dn5 - (locals.var_ddriftldc_dn5 * 0.1)) + locals.var_tmf0_dn5), ((locals.var_ddriftldc_dn6 - (locals.var_ddriftldc_dn6 * 0.1)) + locals.var_tmf0_dn6), ((locals.var_ddriftldc_dn7 - (locals.var_ddriftldc_dn7 * 0.1)) + locals.var_tmf0_dn7), ((locals.var_ddriftldc_dn8 - (locals.var_ddriftldc_dn8 * 0.1)) + locals.var_tmf0_dn8), ((locals.var_ddriftldc_dn9 - (locals.var_ddriftldc_dn9 * 0.1)) + locals.var_tmf0_dn9), ((locals.var_ddriftldc_dn10 - (locals.var_ddriftldc_dn10 * 0.1)) + locals.var_tmf0_dn10), ((locals.var_ddriftldc_dn11 - (locals.var_ddriftldc_dn11 * 0.1)) + locals.var_tmf0_dn11), ((locals.var_ddriftldc_dn14 - (locals.var_ddriftldc_dn14 * 0.1)) + locals.var_tmf0_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign83550_e126879;
        locals.var_t1_dn0 = assign83550_e126879_d_n0;
        locals.var_t1_dn2 = assign83550_e126879_d_n2;
        locals.var_t1_dn4 = assign83550_e126879_d_n4;
        locals.var_t1_dn5 = assign83550_e126879_d_n5;
        locals.var_t1_dn6 = assign83550_e126879_d_n6;
        locals.var_t1_dn7 = assign83550_e126879_d_n7;
        locals.var_t1_dn8 = assign83550_e126879_d_n8;
        locals.var_t1_dn9 = assign83550_e126879_d_n9;
        locals.var_t1_dn10 = assign83550_e126879_d_n10;
        locals.var_t1_dn11 = assign83550_e126879_d_n11;
        locals.var_t1_dn14 = assign83550_e126879_d_n14;

        let (assign83560_e126890, assign83560_e126890_d_n0, assign83560_e126890_d_n2, assign83560_e126890_d_n4, assign83560_e126890_d_n5, assign83560_e126890_d_n6, assign83560_e126890_d_n7, assign83560_e126890_d_n8, assign83560_e126890_d_n9, assign83560_e126890_d_n10, assign83560_e126890_d_n11, assign83560_e126890_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1931 != 0.0)) && (locals.var_guard1932 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign83560_e126890;
        locals.var_t0_dn0 = assign83560_e126890_d_n0;
        locals.var_t0_dn2 = assign83560_e126890_d_n2;
        locals.var_t0_dn4 = assign83560_e126890_d_n4;
        locals.var_t0_dn5 = assign83560_e126890_d_n5;
        locals.var_t0_dn6 = assign83560_e126890_d_n6;
        locals.var_t0_dn7 = assign83560_e126890_d_n7;
        locals.var_t0_dn8 = assign83560_e126890_d_n8;
        locals.var_t0_dn9 = assign83560_e126890_d_n9;
        locals.var_t0_dn10 = assign83560_e126890_d_n10;
        locals.var_t0_dn11 = assign83560_e126890_d_n11;
        locals.var_t0_dn14 = assign83560_e126890_d_n14;

        let (assign83570_e126902, assign83570_e126902_d_n0, assign83570_e126902_d_n2, assign83570_e126902_d_n4, assign83570_e126902_d_n5, assign83570_e126902_d_n6, assign83570_e126902_d_n7, assign83570_e126902_d_n8, assign83570_e126902_d_n9, assign83570_e126902_d_n10, assign83570_e126902_d_n11, assign83570_e126902_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1931 != 0.0)) && (locals.var_guard1932 == 0.0)) {
        (locals.var_wdld0__blk1929, locals.var_wdld0__blk1929_dn0, locals.var_wdld0__blk1929_dn2, locals.var_wdld0__blk1929_dn4, locals.var_wdld0__blk1929_dn5, locals.var_wdld0__blk1929_dn6, locals.var_wdld0__blk1929_dn7, locals.var_wdld0__blk1929_dn8, locals.var_wdld0__blk1929_dn9, locals.var_wdld0__blk1929_dn10, locals.var_wdld0__blk1929_dn11, locals.var_wdld0__blk1929_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign83570_e126902;
        locals.var_t1_dn0 = assign83570_e126902_d_n0;
        locals.var_t1_dn2 = assign83570_e126902_d_n2;
        locals.var_t1_dn4 = assign83570_e126902_d_n4;
        locals.var_t1_dn5 = assign83570_e126902_d_n5;
        locals.var_t1_dn6 = assign83570_e126902_d_n6;
        locals.var_t1_dn7 = assign83570_e126902_d_n7;
        locals.var_t1_dn8 = assign83570_e126902_d_n8;
        locals.var_t1_dn9 = assign83570_e126902_d_n9;
        locals.var_t1_dn10 = assign83570_e126902_d_n10;
        locals.var_t1_dn11 = assign83570_e126902_d_n11;
        locals.var_t1_dn14 = assign83570_e126902_d_n14;

        let (assign83580_e126914, assign83580_e126914_d_n0, assign83580_e126914_d_n2, assign83580_e126914_d_n4, assign83580_e126914_d_n5, assign83580_e126914_d_n6, assign83580_e126914_d_n7, assign83580_e126914_d_n8, assign83580_e126914_d_n9, assign83580_e126914_d_n10, assign83580_e126914_d_n11, assign83580_e126914_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1931 != 0.0)) && (locals.var_guard1932 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign83580_e126914;
        locals.var_t0_dn0 = assign83580_e126914_d_n0;
        locals.var_t0_dn2 = assign83580_e126914_d_n2;
        locals.var_t0_dn4 = assign83580_e126914_d_n4;
        locals.var_t0_dn5 = assign83580_e126914_d_n5;
        locals.var_t0_dn6 = assign83580_e126914_d_n6;
        locals.var_t0_dn7 = assign83580_e126914_d_n7;
        locals.var_t0_dn8 = assign83580_e126914_d_n8;
        locals.var_t0_dn9 = assign83580_e126914_d_n9;
        locals.var_t0_dn10 = assign83580_e126914_d_n10;
        locals.var_t0_dn11 = assign83580_e126914_d_n11;
        locals.var_t0_dn14 = assign83580_e126914_d_n14;

        let assign83590_e126917: f64 = if locals.var_t0 < 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1938 = assign83590_e126917;

        let (assign83600_e126930,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1931 != 0.0)) && (locals.var_guard1938 != 0.0)) {
        let assign83600_e126928: f64 = (locals.var_flg_fd_mode__blk1891 + 2.0);
        (assign83600_e126928,)
    } else {
        (locals.var_flg_fd_mode__blk1891,)
    }
};
        locals.var_flg_fd_mode__blk1891 = assign83600_e126930;

        let (assign83610_e126945, assign83610_e126945_d_n0, assign83610_e126945_d_n2, assign83610_e126945_d_n4, assign83610_e126945_d_n5, assign83610_e126945_d_n6, assign83610_e126945_d_n7, assign83610_e126945_d_n8, assign83610_e126945_d_n9, assign83610_e126945_d_n10, assign83610_e126945_d_n11, assign83610_e126945_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1931 == 0.0)) {
        let (assign83610_e126943, assign83610_e126943_d_n0, assign83610_e126943_d_n2, assign83610_e126943_d_n4, assign83610_e126943_d_n5, assign83610_e126943_d_n6, assign83610_e126943_d_n7, assign83610_e126943_d_n8, assign83610_e126943_d_n9, assign83610_e126943_d_n10, assign83610_e126943_d_n11, assign83610_e126943_d_n14,) = {
            if (locals.var_wdld0__blk1929 <= locals.var_ddriftldc) {
                (locals.var_wdld0__blk1929, locals.var_wdld0__blk1929_dn0, locals.var_wdld0__blk1929_dn2, locals.var_wdld0__blk1929_dn4, locals.var_wdld0__blk1929_dn5, locals.var_wdld0__blk1929_dn6, locals.var_wdld0__blk1929_dn7, locals.var_wdld0__blk1929_dn8, locals.var_wdld0__blk1929_dn9, locals.var_wdld0__blk1929_dn10, locals.var_wdld0__blk1929_dn11, locals.var_wdld0__blk1929_dn14,)
            } else {
                (locals.var_ddriftldc, locals.var_ddriftldc_dn0, locals.var_ddriftldc_dn2, locals.var_ddriftldc_dn4, locals.var_ddriftldc_dn5, locals.var_ddriftldc_dn6, locals.var_ddriftldc_dn7, locals.var_ddriftldc_dn8, locals.var_ddriftldc_dn9, locals.var_ddriftldc_dn10, locals.var_ddriftldc_dn11, locals.var_ddriftldc_dn14,)
            }
        };
        (assign83610_e126943, assign83610_e126943_d_n0, assign83610_e126943_d_n2, assign83610_e126943_d_n4, assign83610_e126943_d_n5, assign83610_e126943_d_n6, assign83610_e126943_d_n7, assign83610_e126943_d_n8, assign83610_e126943_d_n9, assign83610_e126943_d_n10, assign83610_e126943_d_n11, assign83610_e126943_d_n14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign83610_e126945;
        locals.var_t1_dn0 = assign83610_e126945_d_n0;
        locals.var_t1_dn2 = assign83610_e126945_d_n2;
        locals.var_t1_dn4 = assign83610_e126945_d_n4;
        locals.var_t1_dn5 = assign83610_e126945_d_n5;
        locals.var_t1_dn6 = assign83610_e126945_d_n6;
        locals.var_t1_dn7 = assign83610_e126945_d_n7;
        locals.var_t1_dn8 = assign83610_e126945_d_n8;
        locals.var_t1_dn9 = assign83610_e126945_d_n9;
        locals.var_t1_dn10 = assign83610_e126945_d_n10;
        locals.var_t1_dn11 = assign83610_e126945_d_n11;
        locals.var_t1_dn14 = assign83610_e126945_d_n14;

        let assign83620_e126948: f64 = if locals.var_wdld0__blk1929 >= locals.var_ddriftldc { 1.0 } else { 0.0 };
        locals.var_guard1939 = assign83620_e126948;

        let (assign83630_e126962,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1931 == 0.0)) && (locals.var_guard1939 != 0.0)) {
        let assign83630_e126960: f64 = (locals.var_flg_fd_mode__blk1891 + 2.0);
        (assign83630_e126960,)
    } else {
        (locals.var_flg_fd_mode__blk1891,)
    }
};
        locals.var_flg_fd_mode__blk1891 = assign83630_e126962;

        let assign83640_e126965: f64 = if locals.var_flg_fd_mode__blk1891 >= 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1940 = assign83640_e126965;

        let (assign83650_e126974, assign83650_e126974_d_n0, assign83650_e126974_d_n2, assign83650_e126974_d_n4, assign83650_e126974_d_n5, assign83650_e126974_d_n6, assign83650_e126974_d_n7, assign83650_e126974_d_n8, assign83650_e126974_d_n9, assign83650_e126974_d_n10, assign83650_e126974_d_n11, assign83650_e126974_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1940 != 0.0)) {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn14,)
    } else {
        (locals.var_ps0ld_bef1__blk1930, locals.var_ps0ld_bef1__blk1930_dn0, locals.var_ps0ld_bef1__blk1930_dn2, locals.var_ps0ld_bef1__blk1930_dn4, locals.var_ps0ld_bef1__blk1930_dn5, locals.var_ps0ld_bef1__blk1930_dn6, locals.var_ps0ld_bef1__blk1930_dn7, locals.var_ps0ld_bef1__blk1930_dn8, locals.var_ps0ld_bef1__blk1930_dn9, locals.var_ps0ld_bef1__blk1930_dn10, locals.var_ps0ld_bef1__blk1930_dn11, locals.var_ps0ld_bef1__blk1930_dn14,)
    }
};
        locals.var_ps0ld_bef1__blk1930 = assign83650_e126974;
        locals.var_ps0ld_bef1__blk1930_dn0 = assign83650_e126974_d_n0;
        locals.var_ps0ld_bef1__blk1930_dn2 = assign83650_e126974_d_n2;
        locals.var_ps0ld_bef1__blk1930_dn4 = assign83650_e126974_d_n4;
        locals.var_ps0ld_bef1__blk1930_dn5 = assign83650_e126974_d_n5;
        locals.var_ps0ld_bef1__blk1930_dn6 = assign83650_e126974_d_n6;
        locals.var_ps0ld_bef1__blk1930_dn7 = assign83650_e126974_d_n7;
        locals.var_ps0ld_bef1__blk1930_dn8 = assign83650_e126974_d_n8;
        locals.var_ps0ld_bef1__blk1930_dn9 = assign83650_e126974_d_n9;
        locals.var_ps0ld_bef1__blk1930_dn10 = assign83650_e126974_d_n10;
        locals.var_ps0ld_bef1__blk1930_dn11 = assign83650_e126974_d_n11;
        locals.var_ps0ld_bef1__blk1930_dn14 = assign83650_e126974_d_n14;

        let (assign83660_e126985, assign83660_e126985_d_n0, assign83660_e126985_d_n2, assign83660_e126985_d_n4, assign83660_e126985_d_n5, assign83660_e126985_d_n6, assign83660_e126985_d_n7, assign83660_e126985_d_n8, assign83660_e126985_d_n9, assign83660_e126985_d_n10, assign83660_e126985_d_n11, assign83660_e126985_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1940 != 0.0)) {
        let assign83660_e126983: f64 = (locals.var_t1 * locals.var_q_nsubld__blk1885);
        (assign83660_e126983, (locals.var_t1_dn0 * locals.var_q_nsubld__blk1885), (locals.var_t1_dn2 * locals.var_q_nsubld__blk1885), (locals.var_t1_dn4 * locals.var_q_nsubld__blk1885), (locals.var_t1_dn5 * locals.var_q_nsubld__blk1885), (locals.var_t1_dn6 * locals.var_q_nsubld__blk1885), (locals.var_t1_dn7 * locals.var_q_nsubld__blk1885), (locals.var_t1_dn8 * locals.var_q_nsubld__blk1885), (locals.var_t1_dn9 * locals.var_q_nsubld__blk1885), (locals.var_t1_dn10 * locals.var_q_nsubld__blk1885), (locals.var_t1_dn11 * locals.var_q_nsubld__blk1885), (locals.var_t1_dn14 * locals.var_q_nsubld__blk1885),)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn8, locals.var_qbuld_dn9, locals.var_qbuld_dn10, locals.var_qbuld_dn11, locals.var_qbuld_dn14,)
    }
};
        locals.var_qbuld = assign83660_e126985;
        locals.var_qbuld_dn0 = assign83660_e126985_d_n0;
        locals.var_qbuld_dn2 = assign83660_e126985_d_n2;
        locals.var_qbuld_dn4 = assign83660_e126985_d_n4;
        locals.var_qbuld_dn5 = assign83660_e126985_d_n5;
        locals.var_qbuld_dn6 = assign83660_e126985_d_n6;
        locals.var_qbuld_dn7 = assign83660_e126985_d_n7;
        locals.var_qbuld_dn8 = assign83660_e126985_d_n8;
        locals.var_qbuld_dn9 = assign83660_e126985_d_n9;
        locals.var_qbuld_dn10 = assign83660_e126985_d_n10;
        locals.var_qbuld_dn11 = assign83660_e126985_d_n11;
        locals.var_qbuld_dn14 = assign83660_e126985_d_n14;

        let (assign83670_e126998, assign83670_e126998_d_n0, assign83670_e126998_d_n2, assign83670_e126998_d_n4, assign83670_e126998_d_n5, assign83670_e126998_d_n6, assign83670_e126998_d_n7, assign83670_e126998_d_n8, assign83670_e126998_d_n9, assign83670_e126998_d_n10, assign83670_e126998_d_n11, assign83670_e126998_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1940 != 0.0)) {
        let assign83670_e126995: f64 = (locals.var_qbuld / locals.var_cox0_func);
        let assign83670_e126996: f64 = (locals.var_vgpld - assign83670_e126995);
        (assign83670_e126996, (-(locals.var_qbuld_dn0 / locals.var_cox0_func)), (locals.var_vgpld_dn2 - (locals.var_qbuld_dn2 / locals.var_cox0_func)), (-(locals.var_qbuld_dn4 / locals.var_cox0_func)), (-(locals.var_qbuld_dn5 / locals.var_cox0_func)), (-(locals.var_qbuld_dn6 / locals.var_cox0_func)), (locals.var_vgpld_dn7 - (locals.var_qbuld_dn7 / locals.var_cox0_func)), (locals.var_vgpld_dn8 - (locals.var_qbuld_dn8 / locals.var_cox0_func)), (locals.var_vgpld_dn9 - (locals.var_qbuld_dn9 / locals.var_cox0_func)), (-(locals.var_qbuld_dn10 / locals.var_cox0_func)), (-(locals.var_qbuld_dn11 / locals.var_cox0_func)), (-(locals.var_qbuld_dn14 / locals.var_cox0_func)),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn14,)
    }
};
        locals.var_ps0ld = assign83670_e126998;
        locals.var_ps0ld_dn0 = assign83670_e126998_d_n0;
        locals.var_ps0ld_dn2 = assign83670_e126998_d_n2;
        locals.var_ps0ld_dn4 = assign83670_e126998_d_n4;
        locals.var_ps0ld_dn5 = assign83670_e126998_d_n5;
        locals.var_ps0ld_dn6 = assign83670_e126998_d_n6;
        locals.var_ps0ld_dn7 = assign83670_e126998_d_n7;
        locals.var_ps0ld_dn8 = assign83670_e126998_d_n8;
        locals.var_ps0ld_dn9 = assign83670_e126998_d_n9;
        locals.var_ps0ld_dn10 = assign83670_e126998_d_n10;
        locals.var_ps0ld_dn11 = assign83670_e126998_d_n11;
        locals.var_ps0ld_dn14 = assign83670_e126998_d_n14;

        let assign83680_e127001: f64 = if p.p33 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1941 = assign83680_e127001;

        let assign83690_e127005: f64 = (locals.var_ps0ld_bef1__blk1930 - 0.1);
        let assign83690_e127010: f64 = if ((locals.var_ps0ld > assign83690_e127005) && (0.1 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1942 = assign83690_e127010;

        let (assign83700_e127027, assign83700_e127027_d_n0, assign83700_e127027_d_n2, assign83700_e127027_d_n4, assign83700_e127027_d_n5, assign83700_e127027_d_n6, assign83700_e127027_d_n7, assign83700_e127027_d_n8, assign83700_e127027_d_n9, assign83700_e127027_d_n10, assign83700_e127027_d_n11, assign83700_e127027_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1940 != 0.0)) && (locals.var_guard1941 != 0.0)) && (locals.var_guard1942 != 0.0)) {
        let assign83700_e127023: f64 = (locals.var_ps0ld - locals.var_ps0ld_bef1__blk1930);
        let assign83700_e127025: f64 = (assign83700_e127023 + 0.1);
        (assign83700_e127025, (locals.var_ps0ld_dn0 - locals.var_ps0ld_bef1__blk1930_dn0), (locals.var_ps0ld_dn2 - locals.var_ps0ld_bef1__blk1930_dn2), (locals.var_ps0ld_dn4 - locals.var_ps0ld_bef1__blk1930_dn4), (locals.var_ps0ld_dn5 - locals.var_ps0ld_bef1__blk1930_dn5), (locals.var_ps0ld_dn6 - locals.var_ps0ld_bef1__blk1930_dn6), (locals.var_ps0ld_dn7 - locals.var_ps0ld_bef1__blk1930_dn7), (locals.var_ps0ld_dn8 - locals.var_ps0ld_bef1__blk1930_dn8), (locals.var_ps0ld_dn9 - locals.var_ps0ld_bef1__blk1930_dn9), (locals.var_ps0ld_dn10 - locals.var_ps0ld_bef1__blk1930_dn10), (locals.var_ps0ld_dn11 - locals.var_ps0ld_bef1__blk1930_dn11), (locals.var_ps0ld_dn14 - locals.var_ps0ld_bef1__blk1930_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign83700_e127027;
        locals.var_tmf1_dn0 = assign83700_e127027_d_n0;
        locals.var_tmf1_dn2 = assign83700_e127027_d_n2;
        locals.var_tmf1_dn4 = assign83700_e127027_d_n4;
        locals.var_tmf1_dn5 = assign83700_e127027_d_n5;
        locals.var_tmf1_dn6 = assign83700_e127027_d_n6;
        locals.var_tmf1_dn7 = assign83700_e127027_d_n7;
        locals.var_tmf1_dn8 = assign83700_e127027_d_n8;
        locals.var_tmf1_dn9 = assign83700_e127027_d_n9;
        locals.var_tmf1_dn10 = assign83700_e127027_d_n10;
        locals.var_tmf1_dn11 = assign83700_e127027_d_n11;
        locals.var_tmf1_dn14 = assign83700_e127027_d_n14;

        let (assign83710_e127042, assign83710_e127042_d_n0, assign83710_e127042_d_n2, assign83710_e127042_d_n4, assign83710_e127042_d_n5, assign83710_e127042_d_n6, assign83710_e127042_d_n7, assign83710_e127042_d_n8, assign83710_e127042_d_n9, assign83710_e127042_d_n10, assign83710_e127042_d_n11, assign83710_e127042_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1940 != 0.0)) && (locals.var_guard1941 != 0.0)) && (locals.var_guard1942 != 0.0)) {
        let assign83710_e127040: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign83710_e127040, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign83710_e127042;
        locals.var_x2_dn0 = assign83710_e127042_d_n0;
        locals.var_x2_dn2 = assign83710_e127042_d_n2;
        locals.var_x2_dn4 = assign83710_e127042_d_n4;
        locals.var_x2_dn5 = assign83710_e127042_d_n5;
        locals.var_x2_dn6 = assign83710_e127042_d_n6;
        locals.var_x2_dn7 = assign83710_e127042_d_n7;
        locals.var_x2_dn8 = assign83710_e127042_d_n8;
        locals.var_x2_dn9 = assign83710_e127042_d_n9;
        locals.var_x2_dn10 = assign83710_e127042_d_n10;
        locals.var_x2_dn11 = assign83710_e127042_d_n11;
        locals.var_x2_dn14 = assign83710_e127042_d_n14;

        let (assign83720_e127057, assign83720_e127057_d_n0, assign83720_e127057_d_n2, assign83720_e127057_d_n4, assign83720_e127057_d_n5, assign83720_e127057_d_n6, assign83720_e127057_d_n7, assign83720_e127057_d_n8, assign83720_e127057_d_n9, assign83720_e127057_d_n10, assign83720_e127057_d_n11, assign83720_e127057_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1940 != 0.0)) && (locals.var_guard1941 != 0.0)) && (locals.var_guard1942 != 0.0)) {
        let assign83720_e127055: f64 = (0.1 * 0.1);
        (assign83720_e127055, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign83720_e127057;
        locals.var_xmax2_dn0 = assign83720_e127057_d_n0;
        locals.var_xmax2_dn2 = assign83720_e127057_d_n2;
        locals.var_xmax2_dn4 = assign83720_e127057_d_n4;
        locals.var_xmax2_dn5 = assign83720_e127057_d_n5;
        locals.var_xmax2_dn6 = assign83720_e127057_d_n6;
        locals.var_xmax2_dn7 = assign83720_e127057_d_n7;
        locals.var_xmax2_dn8 = assign83720_e127057_d_n8;
        locals.var_xmax2_dn9 = assign83720_e127057_d_n9;
        locals.var_xmax2_dn10 = assign83720_e127057_d_n10;
        locals.var_xmax2_dn11 = assign83720_e127057_d_n11;
        locals.var_xmax2_dn14 = assign83720_e127057_d_n14;

        let (assign83730_e127070, assign83730_e127070_d_n0, assign83730_e127070_d_n2, assign83730_e127070_d_n4, assign83730_e127070_d_n5, assign83730_e127070_d_n6, assign83730_e127070_d_n7, assign83730_e127070_d_n8, assign83730_e127070_d_n9, assign83730_e127070_d_n10, assign83730_e127070_d_n11, assign83730_e127070_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1940 != 0.0)) && (locals.var_guard1941 != 0.0)) && (locals.var_guard1942 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign83730_e127070;
        locals.var_xp_dn0 = assign83730_e127070_d_n0;
        locals.var_xp_dn2 = assign83730_e127070_d_n2;
        locals.var_xp_dn4 = assign83730_e127070_d_n4;
        locals.var_xp_dn5 = assign83730_e127070_d_n5;
        locals.var_xp_dn6 = assign83730_e127070_d_n6;
        locals.var_xp_dn7 = assign83730_e127070_d_n7;
        locals.var_xp_dn8 = assign83730_e127070_d_n8;
        locals.var_xp_dn9 = assign83730_e127070_d_n9;
        locals.var_xp_dn10 = assign83730_e127070_d_n10;
        locals.var_xp_dn11 = assign83730_e127070_d_n11;
        locals.var_xp_dn14 = assign83730_e127070_d_n14;

        let (assign83740_e127083, assign83740_e127083_d_n0, assign83740_e127083_d_n2, assign83740_e127083_d_n4, assign83740_e127083_d_n5, assign83740_e127083_d_n6, assign83740_e127083_d_n7, assign83740_e127083_d_n8, assign83740_e127083_d_n9, assign83740_e127083_d_n10, assign83740_e127083_d_n11, assign83740_e127083_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1940 != 0.0)) && (locals.var_guard1941 != 0.0)) && (locals.var_guard1942 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign83740_e127083;
        locals.var_xmp_dn0 = assign83740_e127083_d_n0;
        locals.var_xmp_dn2 = assign83740_e127083_d_n2;
        locals.var_xmp_dn4 = assign83740_e127083_d_n4;
        locals.var_xmp_dn5 = assign83740_e127083_d_n5;
        locals.var_xmp_dn6 = assign83740_e127083_d_n6;
        locals.var_xmp_dn7 = assign83740_e127083_d_n7;
        locals.var_xmp_dn8 = assign83740_e127083_d_n8;
        locals.var_xmp_dn9 = assign83740_e127083_d_n9;
        locals.var_xmp_dn10 = assign83740_e127083_d_n10;
        locals.var_xmp_dn11 = assign83740_e127083_d_n11;
        locals.var_xmp_dn14 = assign83740_e127083_d_n14;

        let (assign83750_e127096,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1940 != 0.0)) && (locals.var_guard1941 != 0.0)) && (locals.var_guard1942 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign83750_e127096;

        let (assign83760_e127109,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1940 != 0.0)) && (locals.var_guard1941 != 0.0)) && (locals.var_guard1942 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign83760_e127109;

    }

    pub(super) fn stamp_transient_block_303(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign83770_e127122, assign83770_e127122_d_n0, assign83770_e127122_d_n2, assign83770_e127122_d_n4, assign83770_e127122_d_n5, assign83770_e127122_d_n6, assign83770_e127122_d_n7, assign83770_e127122_d_n8, assign83770_e127122_d_n9, assign83770_e127122_d_n10, assign83770_e127122_d_n11, assign83770_e127122_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1940 != 0.0)) && (locals.var_guard1941 != 0.0)) && (locals.var_guard1942 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign83770_e127122;
        locals.var_arg_dn0 = assign83770_e127122_d_n0;
        locals.var_arg_dn2 = assign83770_e127122_d_n2;
        locals.var_arg_dn4 = assign83770_e127122_d_n4;
        locals.var_arg_dn5 = assign83770_e127122_d_n5;
        locals.var_arg_dn6 = assign83770_e127122_d_n6;
        locals.var_arg_dn7 = assign83770_e127122_d_n7;
        locals.var_arg_dn8 = assign83770_e127122_d_n8;
        locals.var_arg_dn9 = assign83770_e127122_d_n9;
        locals.var_arg_dn10 = assign83770_e127122_d_n10;
        locals.var_arg_dn11 = assign83770_e127122_d_n11;
        locals.var_arg_dn14 = assign83770_e127122_d_n14;

        let (assign83780_e127135, assign83780_e127135_d_n0, assign83780_e127135_d_n2, assign83780_e127135_d_n4, assign83780_e127135_d_n5, assign83780_e127135_d_n6, assign83780_e127135_d_n7, assign83780_e127135_d_n8, assign83780_e127135_d_n9, assign83780_e127135_d_n10, assign83780_e127135_d_n11, assign83780_e127135_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1940 != 0.0)) && (locals.var_guard1941 != 0.0)) && (locals.var_guard1942 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign83780_e127135;
        locals.var_dnm_dn0 = assign83780_e127135_d_n0;
        locals.var_dnm_dn2 = assign83780_e127135_d_n2;
        locals.var_dnm_dn4 = assign83780_e127135_d_n4;
        locals.var_dnm_dn5 = assign83780_e127135_d_n5;
        locals.var_dnm_dn6 = assign83780_e127135_d_n6;
        locals.var_dnm_dn7 = assign83780_e127135_d_n7;
        locals.var_dnm_dn8 = assign83780_e127135_d_n8;
        locals.var_dnm_dn9 = assign83780_e127135_d_n9;
        locals.var_dnm_dn10 = assign83780_e127135_d_n10;
        locals.var_dnm_dn11 = assign83780_e127135_d_n11;
        locals.var_dnm_dn14 = assign83780_e127135_d_n14;

        let (assign83790_e127150, assign83790_e127150_d_n0, assign83790_e127150_d_n2, assign83790_e127150_d_n4, assign83790_e127150_d_n5, assign83790_e127150_d_n6, assign83790_e127150_d_n7, assign83790_e127150_d_n8, assign83790_e127150_d_n9, assign83790_e127150_d_n10, assign83790_e127150_d_n11, assign83790_e127150_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1940 != 0.0)) && (locals.var_guard1941 != 0.0)) && (locals.var_guard1942 != 0.0)) {
        let assign83790_e127148: f64 = (locals.var_xp * locals.var_x2);
        (assign83790_e127148, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign83790_e127150;
        locals.var_xp_dn0 = assign83790_e127150_d_n0;
        locals.var_xp_dn2 = assign83790_e127150_d_n2;
        locals.var_xp_dn4 = assign83790_e127150_d_n4;
        locals.var_xp_dn5 = assign83790_e127150_d_n5;
        locals.var_xp_dn6 = assign83790_e127150_d_n6;
        locals.var_xp_dn7 = assign83790_e127150_d_n7;
        locals.var_xp_dn8 = assign83790_e127150_d_n8;
        locals.var_xp_dn9 = assign83790_e127150_d_n9;
        locals.var_xp_dn10 = assign83790_e127150_d_n10;
        locals.var_xp_dn11 = assign83790_e127150_d_n11;
        locals.var_xp_dn14 = assign83790_e127150_d_n14;

        let (assign83800_e127165, assign83800_e127165_d_n0, assign83800_e127165_d_n2, assign83800_e127165_d_n4, assign83800_e127165_d_n5, assign83800_e127165_d_n6, assign83800_e127165_d_n7, assign83800_e127165_d_n8, assign83800_e127165_d_n9, assign83800_e127165_d_n10, assign83800_e127165_d_n11, assign83800_e127165_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1940 != 0.0)) && (locals.var_guard1941 != 0.0)) && (locals.var_guard1942 != 0.0)) {
        let assign83800_e127163: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign83800_e127163, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign83800_e127165;
        locals.var_xmp_dn0 = assign83800_e127165_d_n0;
        locals.var_xmp_dn2 = assign83800_e127165_d_n2;
        locals.var_xmp_dn4 = assign83800_e127165_d_n4;
        locals.var_xmp_dn5 = assign83800_e127165_d_n5;
        locals.var_xmp_dn6 = assign83800_e127165_d_n6;
        locals.var_xmp_dn7 = assign83800_e127165_d_n7;
        locals.var_xmp_dn8 = assign83800_e127165_d_n8;
        locals.var_xmp_dn9 = assign83800_e127165_d_n9;
        locals.var_xmp_dn10 = assign83800_e127165_d_n10;
        locals.var_xmp_dn11 = assign83800_e127165_d_n11;
        locals.var_xmp_dn14 = assign83800_e127165_d_n14;

        let (assign83810_e127180, assign83810_e127180_d_n0, assign83810_e127180_d_n2, assign83810_e127180_d_n4, assign83810_e127180_d_n5, assign83810_e127180_d_n6, assign83810_e127180_d_n7, assign83810_e127180_d_n8, assign83810_e127180_d_n9, assign83810_e127180_d_n10, assign83810_e127180_d_n11, assign83810_e127180_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1940 != 0.0)) && (locals.var_guard1941 != 0.0)) && (locals.var_guard1942 != 0.0)) {
        let assign83810_e127178: f64 = (locals.var_xp * locals.var_x2);
        (assign83810_e127178, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign83810_e127180;
        locals.var_xp_dn0 = assign83810_e127180_d_n0;
        locals.var_xp_dn2 = assign83810_e127180_d_n2;
        locals.var_xp_dn4 = assign83810_e127180_d_n4;
        locals.var_xp_dn5 = assign83810_e127180_d_n5;
        locals.var_xp_dn6 = assign83810_e127180_d_n6;
        locals.var_xp_dn7 = assign83810_e127180_d_n7;
        locals.var_xp_dn8 = assign83810_e127180_d_n8;
        locals.var_xp_dn9 = assign83810_e127180_d_n9;
        locals.var_xp_dn10 = assign83810_e127180_d_n10;
        locals.var_xp_dn11 = assign83810_e127180_d_n11;
        locals.var_xp_dn14 = assign83810_e127180_d_n14;

        let (assign83820_e127195, assign83820_e127195_d_n0, assign83820_e127195_d_n2, assign83820_e127195_d_n4, assign83820_e127195_d_n5, assign83820_e127195_d_n6, assign83820_e127195_d_n7, assign83820_e127195_d_n8, assign83820_e127195_d_n9, assign83820_e127195_d_n10, assign83820_e127195_d_n11, assign83820_e127195_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1940 != 0.0)) && (locals.var_guard1941 != 0.0)) && (locals.var_guard1942 != 0.0)) {
        let assign83820_e127193: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign83820_e127193, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign83820_e127195;
        locals.var_xmp_dn0 = assign83820_e127195_d_n0;
        locals.var_xmp_dn2 = assign83820_e127195_d_n2;
        locals.var_xmp_dn4 = assign83820_e127195_d_n4;
        locals.var_xmp_dn5 = assign83820_e127195_d_n5;
        locals.var_xmp_dn6 = assign83820_e127195_d_n6;
        locals.var_xmp_dn7 = assign83820_e127195_d_n7;
        locals.var_xmp_dn8 = assign83820_e127195_d_n8;
        locals.var_xmp_dn9 = assign83820_e127195_d_n9;
        locals.var_xmp_dn10 = assign83820_e127195_d_n10;
        locals.var_xmp_dn11 = assign83820_e127195_d_n11;
        locals.var_xmp_dn14 = assign83820_e127195_d_n14;

        let (assign83830_e127210, assign83830_e127210_d_n0, assign83830_e127210_d_n2, assign83830_e127210_d_n4, assign83830_e127210_d_n5, assign83830_e127210_d_n6, assign83830_e127210_d_n7, assign83830_e127210_d_n8, assign83830_e127210_d_n9, assign83830_e127210_d_n10, assign83830_e127210_d_n11, assign83830_e127210_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1940 != 0.0)) && (locals.var_guard1941 != 0.0)) && (locals.var_guard1942 != 0.0)) {
        let assign83830_e127208: f64 = (locals.var_xp + locals.var_xmp);
        (assign83830_e127208, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign83830_e127210;
        locals.var_arg_dn0 = assign83830_e127210_d_n0;
        locals.var_arg_dn2 = assign83830_e127210_d_n2;
        locals.var_arg_dn4 = assign83830_e127210_d_n4;
        locals.var_arg_dn5 = assign83830_e127210_d_n5;
        locals.var_arg_dn6 = assign83830_e127210_d_n6;
        locals.var_arg_dn7 = assign83830_e127210_d_n7;
        locals.var_arg_dn8 = assign83830_e127210_d_n8;
        locals.var_arg_dn9 = assign83830_e127210_d_n9;
        locals.var_arg_dn10 = assign83830_e127210_d_n10;
        locals.var_arg_dn11 = assign83830_e127210_d_n11;
        locals.var_arg_dn14 = assign83830_e127210_d_n14;

        let (assign83840_e127223, assign83840_e127223_d_n0, assign83840_e127223_d_n2, assign83840_e127223_d_n4, assign83840_e127223_d_n5, assign83840_e127223_d_n6, assign83840_e127223_d_n7, assign83840_e127223_d_n8, assign83840_e127223_d_n9, assign83840_e127223_d_n10, assign83840_e127223_d_n11, assign83840_e127223_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1940 != 0.0)) && (locals.var_guard1941 != 0.0)) && (locals.var_guard1942 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign83840_e127223;
        locals.var_dnm_dn0 = assign83840_e127223_d_n0;
        locals.var_dnm_dn2 = assign83840_e127223_d_n2;
        locals.var_dnm_dn4 = assign83840_e127223_d_n4;
        locals.var_dnm_dn5 = assign83840_e127223_d_n5;
        locals.var_dnm_dn6 = assign83840_e127223_d_n6;
        locals.var_dnm_dn7 = assign83840_e127223_d_n7;
        locals.var_dnm_dn8 = assign83840_e127223_d_n8;
        locals.var_dnm_dn9 = assign83840_e127223_d_n9;
        locals.var_dnm_dn10 = assign83840_e127223_d_n10;
        locals.var_dnm_dn11 = assign83840_e127223_d_n11;
        locals.var_dnm_dn14 = assign83840_e127223_d_n14;

        let assign83850_e127238: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1943 = assign83850_e127238;

        let assign83860_e127241: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1944 = assign83860_e127241;

        let (assign83870_e127258,) = {
    if (((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1940 != 0.0)) && (locals.var_guard1941 != 0.0)) && (locals.var_guard1942 != 0.0)) && (locals.var_guard1943 != 0.0)) && (locals.var_guard1944 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign83870_e127258;

        let assign83880_e127261: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1945 = assign83880_e127261;

        let (assign83890_e127281,) = {
    if ((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1940 != 0.0)) && (locals.var_guard1941 != 0.0)) && (locals.var_guard1942 != 0.0)) && (locals.var_guard1943 != 0.0)) && (locals.var_guard1944 == 0.0)) && (locals.var_guard1945 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign83890_e127281;

        let assign83900_e127284: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1946 = assign83900_e127284;

        let (assign83910_e127307,) = {
    if (((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1940 != 0.0)) && (locals.var_guard1941 != 0.0)) && (locals.var_guard1942 != 0.0)) && (locals.var_guard1943 != 0.0)) && (locals.var_guard1944 == 0.0)) && (locals.var_guard1945 == 0.0)) && (locals.var_guard1946 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign83910_e127307;

        let assign83920_e127310: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1947 = assign83920_e127310;

        let (assign83930_e127336,) = {
    if ((((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1940 != 0.0)) && (locals.var_guard1941 != 0.0)) && (locals.var_guard1942 != 0.0)) && (locals.var_guard1943 != 0.0)) && (locals.var_guard1944 == 0.0)) && (locals.var_guard1945 == 0.0)) && (locals.var_guard1946 == 0.0)) && (locals.var_guard1947 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign83930_e127336;

        let (assign83940_e127351,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1940 != 0.0)) && (locals.var_guard1941 != 0.0)) && (locals.var_guard1942 != 0.0)) && (locals.var_guard1943 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign83940_e127351;

        let mut assign83950_loop_guard: usize = 0;
        while {
            let assign83950_cond_e127367: f64 = if (((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1940 != 0.0)) && (locals.var_guard1941 != 0.0)) && (locals.var_guard1942 != 0.0)) && (locals.var_guard1943 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign83950_cond_e127367 != 0.0
        } {
            assign83950_loop_guard += 1;
            assert!(assign83950_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign83950_body0_e127383, assign83950_body0_e127383_d_n0, assign83950_body0_e127383_d_n2, assign83950_body0_e127383_d_n4, assign83950_body0_e127383_d_n5, assign83950_body0_e127383_d_n6, assign83950_body0_e127383_d_n7, assign83950_body0_e127383_d_n8, assign83950_body0_e127383_d_n9, assign83950_body0_e127383_d_n10, assign83950_body0_e127383_d_n11, assign83950_body0_e127383_d_n14,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1940 != 0.0)) && (locals.var_guard1941 != 0.0)) && (locals.var_guard1942 != 0.0)) && (locals.var_guard1943 != 0.0)) {
        let assign83950_body0_e127381: f64 = (locals.var_dnm).sqrt();
        (assign83950_body0_e127381, (locals.var_dnm_dn0 / (2.0 * assign83950_body0_e127381)), (locals.var_dnm_dn2 / (2.0 * assign83950_body0_e127381)), (locals.var_dnm_dn4 / (2.0 * assign83950_body0_e127381)), (locals.var_dnm_dn5 / (2.0 * assign83950_body0_e127381)), (locals.var_dnm_dn6 / (2.0 * assign83950_body0_e127381)), (locals.var_dnm_dn7 / (2.0 * assign83950_body0_e127381)), (locals.var_dnm_dn8 / (2.0 * assign83950_body0_e127381)), (locals.var_dnm_dn9 / (2.0 * assign83950_body0_e127381)), (locals.var_dnm_dn10 / (2.0 * assign83950_body0_e127381)), (locals.var_dnm_dn11 / (2.0 * assign83950_body0_e127381)), (locals.var_dnm_dn14 / (2.0 * assign83950_body0_e127381)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign83950_body0_e127383;
            locals.var_dnm_dn0 = assign83950_body0_e127383_d_n0;
            locals.var_dnm_dn2 = assign83950_body0_e127383_d_n2;
            locals.var_dnm_dn4 = assign83950_body0_e127383_d_n4;
            locals.var_dnm_dn5 = assign83950_body0_e127383_d_n5;
            locals.var_dnm_dn6 = assign83950_body0_e127383_d_n6;
            locals.var_dnm_dn7 = assign83950_body0_e127383_d_n7;
            locals.var_dnm_dn8 = assign83950_body0_e127383_d_n8;
            locals.var_dnm_dn9 = assign83950_body0_e127383_d_n9;
            locals.var_dnm_dn10 = assign83950_body0_e127383_d_n10;
            locals.var_dnm_dn11 = assign83950_body0_e127383_d_n11;
            locals.var_dnm_dn14 = assign83950_body0_e127383_d_n14;
            let (assign83950_body1_e127400,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1940 != 0.0)) && (locals.var_guard1941 != 0.0)) && (locals.var_guard1942 != 0.0)) && (locals.var_guard1943 != 0.0)) {
        let assign83950_body1_e127398: f64 = (locals.var_m0 + 1.0);
        (assign83950_body1_e127398,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign83950_body1_e127400;
        }

        let (assign83960_e127427, assign83960_e127427_d_n0, assign83960_e127427_d_n2, assign83960_e127427_d_n4, assign83960_e127427_d_n5, assign83960_e127427_d_n6, assign83960_e127427_d_n7, assign83960_e127427_d_n8, assign83960_e127427_d_n9, assign83960_e127427_d_n10, assign83960_e127427_d_n11, assign83960_e127427_d_n14,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1940 != 0.0)) && (locals.var_guard1941 != 0.0)) && (locals.var_guard1942 != 0.0)) && (locals.var_guard1943 == 0.0)) {
        let (assign83960_e127425, assign83960_e127425_d_n0, assign83960_e127425_d_n2, assign83960_e127425_d_n4, assign83960_e127425_d_n5, assign83960_e127425_d_n6, assign83960_e127425_d_n7, assign83960_e127425_d_n8, assign83960_e127425_d_n9, assign83960_e127425_d_n10, assign83960_e127425_d_n11, assign83960_e127425_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign83960_e127422: f64 = (2.0 * 2.0);
                let assign83960_e127423: f64 = (1.0 / assign83960_e127422);
                let assign83960_e127424: f64 = (locals.var_dnm).powf(assign83960_e127423);
                (assign83960_e127424, if 0.0 == 0.0 && ((assign83960_e127423) as f64).is_finite() && ((assign83960_e127423) as f64).fract() == 0.0 { if assign83960_e127423 == 0.0 { 0.0 } else { (assign83960_e127423 * ((locals.var_dnm).powf(assign83960_e127423 - 1.0) * locals.var_dnm_dn0)) } } else { (assign83960_e127424 * (assign83960_e127423 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign83960_e127423) as f64).is_finite() && ((assign83960_e127423) as f64).fract() == 0.0 { if assign83960_e127423 == 0.0 { 0.0 } else { (assign83960_e127423 * ((locals.var_dnm).powf(assign83960_e127423 - 1.0) * locals.var_dnm_dn2)) } } else { (assign83960_e127424 * (assign83960_e127423 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign83960_e127423) as f64).is_finite() && ((assign83960_e127423) as f64).fract() == 0.0 { if assign83960_e127423 == 0.0 { 0.0 } else { (assign83960_e127423 * ((locals.var_dnm).powf(assign83960_e127423 - 1.0) * locals.var_dnm_dn4)) } } else { (assign83960_e127424 * (assign83960_e127423 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign83960_e127423) as f64).is_finite() && ((assign83960_e127423) as f64).fract() == 0.0 { if assign83960_e127423 == 0.0 { 0.0 } else { (assign83960_e127423 * ((locals.var_dnm).powf(assign83960_e127423 - 1.0) * locals.var_dnm_dn5)) } } else { (assign83960_e127424 * (assign83960_e127423 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign83960_e127423) as f64).is_finite() && ((assign83960_e127423) as f64).fract() == 0.0 { if assign83960_e127423 == 0.0 { 0.0 } else { (assign83960_e127423 * ((locals.var_dnm).powf(assign83960_e127423 - 1.0) * locals.var_dnm_dn6)) } } else { (assign83960_e127424 * (assign83960_e127423 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign83960_e127423) as f64).is_finite() && ((assign83960_e127423) as f64).fract() == 0.0 { if assign83960_e127423 == 0.0 { 0.0 } else { (assign83960_e127423 * ((locals.var_dnm).powf(assign83960_e127423 - 1.0) * locals.var_dnm_dn7)) } } else { (assign83960_e127424 * (assign83960_e127423 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign83960_e127423) as f64).is_finite() && ((assign83960_e127423) as f64).fract() == 0.0 { if assign83960_e127423 == 0.0 { 0.0 } else { (assign83960_e127423 * ((locals.var_dnm).powf(assign83960_e127423 - 1.0) * locals.var_dnm_dn8)) } } else { (assign83960_e127424 * (assign83960_e127423 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign83960_e127423) as f64).is_finite() && ((assign83960_e127423) as f64).fract() == 0.0 { if assign83960_e127423 == 0.0 { 0.0 } else { (assign83960_e127423 * ((locals.var_dnm).powf(assign83960_e127423 - 1.0) * locals.var_dnm_dn9)) } } else { (assign83960_e127424 * (assign83960_e127423 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign83960_e127423) as f64).is_finite() && ((assign83960_e127423) as f64).fract() == 0.0 { if assign83960_e127423 == 0.0 { 0.0 } else { (assign83960_e127423 * ((locals.var_dnm).powf(assign83960_e127423 - 1.0) * locals.var_dnm_dn10)) } } else { (assign83960_e127424 * (assign83960_e127423 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign83960_e127423) as f64).is_finite() && ((assign83960_e127423) as f64).fract() == 0.0 { if assign83960_e127423 == 0.0 { 0.0 } else { (assign83960_e127423 * ((locals.var_dnm).powf(assign83960_e127423 - 1.0) * locals.var_dnm_dn11)) } } else { (assign83960_e127424 * (assign83960_e127423 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign83960_e127423) as f64).is_finite() && ((assign83960_e127423) as f64).fract() == 0.0 { if assign83960_e127423 == 0.0 { 0.0 } else { (assign83960_e127423 * ((locals.var_dnm).powf(assign83960_e127423 - 1.0) * locals.var_dnm_dn14)) } } else { (assign83960_e127424 * (assign83960_e127423 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign83960_e127425, assign83960_e127425_d_n0, assign83960_e127425_d_n2, assign83960_e127425_d_n4, assign83960_e127425_d_n5, assign83960_e127425_d_n6, assign83960_e127425_d_n7, assign83960_e127425_d_n8, assign83960_e127425_d_n9, assign83960_e127425_d_n10, assign83960_e127425_d_n11, assign83960_e127425_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign83960_e127427;
        locals.var_dnm_dn0 = assign83960_e127427_d_n0;
        locals.var_dnm_dn2 = assign83960_e127427_d_n2;
        locals.var_dnm_dn4 = assign83960_e127427_d_n4;
        locals.var_dnm_dn5 = assign83960_e127427_d_n5;
        locals.var_dnm_dn6 = assign83960_e127427_d_n6;
        locals.var_dnm_dn7 = assign83960_e127427_d_n7;
        locals.var_dnm_dn8 = assign83960_e127427_d_n8;
        locals.var_dnm_dn9 = assign83960_e127427_d_n9;
        locals.var_dnm_dn10 = assign83960_e127427_d_n10;
        locals.var_dnm_dn11 = assign83960_e127427_d_n11;
        locals.var_dnm_dn14 = assign83960_e127427_d_n14;

        let (assign83970_e127442, assign83970_e127442_d_n0, assign83970_e127442_d_n2, assign83970_e127442_d_n4, assign83970_e127442_d_n5, assign83970_e127442_d_n6, assign83970_e127442_d_n7, assign83970_e127442_d_n8, assign83970_e127442_d_n9, assign83970_e127442_d_n10, assign83970_e127442_d_n11, assign83970_e127442_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1940 != 0.0)) && (locals.var_guard1941 != 0.0)) && (locals.var_guard1942 != 0.0)) {
        let assign83970_e127440: f64 = (1.0 / locals.var_dnm);
        (assign83970_e127440, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign83970_e127442;
        locals.var_dnm_dn0 = assign83970_e127442_d_n0;
        locals.var_dnm_dn2 = assign83970_e127442_d_n2;
        locals.var_dnm_dn4 = assign83970_e127442_d_n4;
        locals.var_dnm_dn5 = assign83970_e127442_d_n5;
        locals.var_dnm_dn6 = assign83970_e127442_d_n6;
        locals.var_dnm_dn7 = assign83970_e127442_d_n7;
        locals.var_dnm_dn8 = assign83970_e127442_d_n8;
        locals.var_dnm_dn9 = assign83970_e127442_d_n9;
        locals.var_dnm_dn10 = assign83970_e127442_d_n10;
        locals.var_dnm_dn11 = assign83970_e127442_d_n11;
        locals.var_dnm_dn14 = assign83970_e127442_d_n14;

        let (assign83980_e127459, assign83980_e127459_d_n0, assign83980_e127459_d_n2, assign83980_e127459_d_n4, assign83980_e127459_d_n5, assign83980_e127459_d_n6, assign83980_e127459_d_n7, assign83980_e127459_d_n8, assign83980_e127459_d_n9, assign83980_e127459_d_n10, assign83980_e127459_d_n11, assign83980_e127459_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1940 != 0.0)) && (locals.var_guard1941 != 0.0)) && (locals.var_guard1942 != 0.0)) {
        let assign83980_e127455: f64 = (locals.var_tmf1 * 0.1);
        let assign83980_e127457: f64 = (assign83980_e127455 * locals.var_dnm);
        (assign83980_e127457, (((locals.var_tmf1_dn0 * 0.1) * locals.var_dnm) + (assign83980_e127455 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 0.1) * locals.var_dnm) + (assign83980_e127455 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 0.1) * locals.var_dnm) + (assign83980_e127455 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 0.1) * locals.var_dnm) + (assign83980_e127455 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 0.1) * locals.var_dnm) + (assign83980_e127455 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 0.1) * locals.var_dnm) + (assign83980_e127455 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 0.1) * locals.var_dnm) + (assign83980_e127455 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 0.1) * locals.var_dnm) + (assign83980_e127455 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 0.1) * locals.var_dnm) + (assign83980_e127455 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * 0.1) * locals.var_dnm) + (assign83980_e127455 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * 0.1) * locals.var_dnm) + (assign83980_e127455 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign83980_e127459;
        locals.var_tmf0_dn0 = assign83980_e127459_d_n0;
        locals.var_tmf0_dn2 = assign83980_e127459_d_n2;
        locals.var_tmf0_dn4 = assign83980_e127459_d_n4;
        locals.var_tmf0_dn5 = assign83980_e127459_d_n5;
        locals.var_tmf0_dn6 = assign83980_e127459_d_n6;
        locals.var_tmf0_dn7 = assign83980_e127459_d_n7;
        locals.var_tmf0_dn8 = assign83980_e127459_d_n8;
        locals.var_tmf0_dn9 = assign83980_e127459_d_n9;
        locals.var_tmf0_dn10 = assign83980_e127459_d_n10;
        locals.var_tmf0_dn11 = assign83980_e127459_d_n11;
        locals.var_tmf0_dn14 = assign83980_e127459_d_n14;

        let (assign83990_e127478, assign83990_e127478_d_n0, assign83990_e127478_d_n2, assign83990_e127478_d_n4, assign83990_e127478_d_n5, assign83990_e127478_d_n6, assign83990_e127478_d_n7, assign83990_e127478_d_n8, assign83990_e127478_d_n9, assign83990_e127478_d_n10, assign83990_e127478_d_n11, assign83990_e127478_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1940 != 0.0)) && (locals.var_guard1941 != 0.0)) && (locals.var_guard1942 != 0.0)) {
        let assign83990_e127472: f64 = (0.1 * locals.var_xmp);
        let assign83990_e127474: f64 = (assign83990_e127472 * locals.var_dnm);
        let assign83990_e127476: f64 = (assign83990_e127474 / locals.var_arg);
        (assign83990_e127476, ((((((0.1 * locals.var_xmp_dn0) * locals.var_dnm) + (assign83990_e127472 * locals.var_dnm_dn0)) * locals.var_arg) - (assign83990_e127474 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn2) * locals.var_dnm) + (assign83990_e127472 * locals.var_dnm_dn2)) * locals.var_arg) - (assign83990_e127474 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn4) * locals.var_dnm) + (assign83990_e127472 * locals.var_dnm_dn4)) * locals.var_arg) - (assign83990_e127474 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn5) * locals.var_dnm) + (assign83990_e127472 * locals.var_dnm_dn5)) * locals.var_arg) - (assign83990_e127474 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn6) * locals.var_dnm) + (assign83990_e127472 * locals.var_dnm_dn6)) * locals.var_arg) - (assign83990_e127474 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn7) * locals.var_dnm) + (assign83990_e127472 * locals.var_dnm_dn7)) * locals.var_arg) - (assign83990_e127474 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn8) * locals.var_dnm) + (assign83990_e127472 * locals.var_dnm_dn8)) * locals.var_arg) - (assign83990_e127474 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn9) * locals.var_dnm) + (assign83990_e127472 * locals.var_dnm_dn9)) * locals.var_arg) - (assign83990_e127474 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn10) * locals.var_dnm) + (assign83990_e127472 * locals.var_dnm_dn10)) * locals.var_arg) - (assign83990_e127474 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn11) * locals.var_dnm) + (assign83990_e127472 * locals.var_dnm_dn11)) * locals.var_arg) - (assign83990_e127474 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn14) * locals.var_dnm) + (assign83990_e127472 * locals.var_dnm_dn14)) * locals.var_arg) - (assign83990_e127474 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign83990_e127478;
        locals.var_t0_dn0 = assign83990_e127478_d_n0;
        locals.var_t0_dn2 = assign83990_e127478_d_n2;
        locals.var_t0_dn4 = assign83990_e127478_d_n4;
        locals.var_t0_dn5 = assign83990_e127478_d_n5;
        locals.var_t0_dn6 = assign83990_e127478_d_n6;
        locals.var_t0_dn7 = assign83990_e127478_d_n7;
        locals.var_t0_dn8 = assign83990_e127478_d_n8;
        locals.var_t0_dn9 = assign83990_e127478_d_n9;
        locals.var_t0_dn10 = assign83990_e127478_d_n10;
        locals.var_t0_dn11 = assign83990_e127478_d_n11;
        locals.var_t0_dn14 = assign83990_e127478_d_n14;

        let (assign84000_e127495, assign84000_e127495_d_n0, assign84000_e127495_d_n2, assign84000_e127495_d_n4, assign84000_e127495_d_n5, assign84000_e127495_d_n6, assign84000_e127495_d_n7, assign84000_e127495_d_n8, assign84000_e127495_d_n9, assign84000_e127495_d_n10, assign84000_e127495_d_n11, assign84000_e127495_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1940 != 0.0)) && (locals.var_guard1941 != 0.0)) && (locals.var_guard1942 != 0.0)) {
        let assign84000_e127491: f64 = (locals.var_ps0ld_bef1__blk1930 - 0.1);
        let assign84000_e127493: f64 = (assign84000_e127491 + locals.var_tmf0);
        (assign84000_e127493, (locals.var_ps0ld_bef1__blk1930_dn0 + locals.var_tmf0_dn0), (locals.var_ps0ld_bef1__blk1930_dn2 + locals.var_tmf0_dn2), (locals.var_ps0ld_bef1__blk1930_dn4 + locals.var_tmf0_dn4), (locals.var_ps0ld_bef1__blk1930_dn5 + locals.var_tmf0_dn5), (locals.var_ps0ld_bef1__blk1930_dn6 + locals.var_tmf0_dn6), (locals.var_ps0ld_bef1__blk1930_dn7 + locals.var_tmf0_dn7), (locals.var_ps0ld_bef1__blk1930_dn8 + locals.var_tmf0_dn8), (locals.var_ps0ld_bef1__blk1930_dn9 + locals.var_tmf0_dn9), (locals.var_ps0ld_bef1__blk1930_dn10 + locals.var_tmf0_dn10), (locals.var_ps0ld_bef1__blk1930_dn11 + locals.var_tmf0_dn11), (locals.var_ps0ld_bef1__blk1930_dn14 + locals.var_tmf0_dn14),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn14,)
    }
};
        locals.var_ps0ld = assign84000_e127495;
        locals.var_ps0ld_dn0 = assign84000_e127495_d_n0;
        locals.var_ps0ld_dn2 = assign84000_e127495_d_n2;
        locals.var_ps0ld_dn4 = assign84000_e127495_d_n4;
        locals.var_ps0ld_dn5 = assign84000_e127495_d_n5;
        locals.var_ps0ld_dn6 = assign84000_e127495_d_n6;
        locals.var_ps0ld_dn7 = assign84000_e127495_d_n7;
        locals.var_ps0ld_dn8 = assign84000_e127495_d_n8;
        locals.var_ps0ld_dn9 = assign84000_e127495_d_n9;
        locals.var_ps0ld_dn10 = assign84000_e127495_d_n10;
        locals.var_ps0ld_dn11 = assign84000_e127495_d_n11;
        locals.var_ps0ld_dn14 = assign84000_e127495_d_n14;

        let (assign84010_e127508, assign84010_e127508_d_n0, assign84010_e127508_d_n2, assign84010_e127508_d_n4, assign84010_e127508_d_n5, assign84010_e127508_d_n6, assign84010_e127508_d_n7, assign84010_e127508_d_n8, assign84010_e127508_d_n9, assign84010_e127508_d_n10, assign84010_e127508_d_n11, assign84010_e127508_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1940 != 0.0)) && (locals.var_guard1941 != 0.0)) && (locals.var_guard1942 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign84010_e127508;
        locals.var_t0_dn0 = assign84010_e127508_d_n0;
        locals.var_t0_dn2 = assign84010_e127508_d_n2;
        locals.var_t0_dn4 = assign84010_e127508_d_n4;
        locals.var_t0_dn5 = assign84010_e127508_d_n5;
        locals.var_t0_dn6 = assign84010_e127508_d_n6;
        locals.var_t0_dn7 = assign84010_e127508_d_n7;
        locals.var_t0_dn8 = assign84010_e127508_d_n8;
        locals.var_t0_dn9 = assign84010_e127508_d_n9;
        locals.var_t0_dn10 = assign84010_e127508_d_n10;
        locals.var_t0_dn11 = assign84010_e127508_d_n11;
        locals.var_t0_dn14 = assign84010_e127508_d_n14;

        let (assign84020_e127522, assign84020_e127522_d_n0, assign84020_e127522_d_n2, assign84020_e127522_d_n4, assign84020_e127522_d_n5, assign84020_e127522_d_n6, assign84020_e127522_d_n7, assign84020_e127522_d_n8, assign84020_e127522_d_n9, assign84020_e127522_d_n10, assign84020_e127522_d_n11, assign84020_e127522_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1940 != 0.0)) && (locals.var_guard1941 != 0.0)) && (locals.var_guard1942 == 0.0)) {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn14,)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn14,)
    }
};
        locals.var_ps0ld = assign84020_e127522;
        locals.var_ps0ld_dn0 = assign84020_e127522_d_n0;
        locals.var_ps0ld_dn2 = assign84020_e127522_d_n2;
        locals.var_ps0ld_dn4 = assign84020_e127522_d_n4;
        locals.var_ps0ld_dn5 = assign84020_e127522_d_n5;
        locals.var_ps0ld_dn6 = assign84020_e127522_d_n6;
        locals.var_ps0ld_dn7 = assign84020_e127522_d_n7;
        locals.var_ps0ld_dn8 = assign84020_e127522_d_n8;
        locals.var_ps0ld_dn9 = assign84020_e127522_d_n9;
        locals.var_ps0ld_dn10 = assign84020_e127522_d_n10;
        locals.var_ps0ld_dn11 = assign84020_e127522_d_n11;
        locals.var_ps0ld_dn14 = assign84020_e127522_d_n14;

        let (assign84030_e127536, assign84030_e127536_d_n0, assign84030_e127536_d_n2, assign84030_e127536_d_n4, assign84030_e127536_d_n5, assign84030_e127536_d_n6, assign84030_e127536_d_n7, assign84030_e127536_d_n8, assign84030_e127536_d_n9, assign84030_e127536_d_n10, assign84030_e127536_d_n11, assign84030_e127536_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1940 != 0.0)) && (locals.var_guard1941 != 0.0)) && (locals.var_guard1942 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign84030_e127536;
        locals.var_t0_dn0 = assign84030_e127536_d_n0;
        locals.var_t0_dn2 = assign84030_e127536_d_n2;
        locals.var_t0_dn4 = assign84030_e127536_d_n4;
        locals.var_t0_dn5 = assign84030_e127536_d_n5;
        locals.var_t0_dn6 = assign84030_e127536_d_n6;
        locals.var_t0_dn7 = assign84030_e127536_d_n7;
        locals.var_t0_dn8 = assign84030_e127536_d_n8;
        locals.var_t0_dn9 = assign84030_e127536_d_n9;
        locals.var_t0_dn10 = assign84030_e127536_d_n10;
        locals.var_t0_dn11 = assign84030_e127536_d_n11;
        locals.var_t0_dn14 = assign84030_e127536_d_n14;

        let (assign84040_e127553, assign84040_e127553_d_n0, assign84040_e127553_d_n2, assign84040_e127553_d_n4, assign84040_e127553_d_n5, assign84040_e127553_d_n6, assign84040_e127553_d_n7, assign84040_e127553_d_n8, assign84040_e127553_d_n9, assign84040_e127553_d_n10, assign84040_e127553_d_n11, assign84040_e127553_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1940 != 0.0)) && (locals.var_guard1941 == 0.0)) {
        let (assign84040_e127551, assign84040_e127551_d_n0, assign84040_e127551_d_n2, assign84040_e127551_d_n4, assign84040_e127551_d_n5, assign84040_e127551_d_n6, assign84040_e127551_d_n7, assign84040_e127551_d_n8, assign84040_e127551_d_n9, assign84040_e127551_d_n10, assign84040_e127551_d_n11, assign84040_e127551_d_n14,) = {
            if (locals.var_ps0ld <= locals.var_ps0ld_bef1__blk1930) {
                (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn14,)
            } else {
                (locals.var_ps0ld_bef1__blk1930, locals.var_ps0ld_bef1__blk1930_dn0, locals.var_ps0ld_bef1__blk1930_dn2, locals.var_ps0ld_bef1__blk1930_dn4, locals.var_ps0ld_bef1__blk1930_dn5, locals.var_ps0ld_bef1__blk1930_dn6, locals.var_ps0ld_bef1__blk1930_dn7, locals.var_ps0ld_bef1__blk1930_dn8, locals.var_ps0ld_bef1__blk1930_dn9, locals.var_ps0ld_bef1__blk1930_dn10, locals.var_ps0ld_bef1__blk1930_dn11, locals.var_ps0ld_bef1__blk1930_dn14,)
            }
        };
        (assign84040_e127551, assign84040_e127551_d_n0, assign84040_e127551_d_n2, assign84040_e127551_d_n4, assign84040_e127551_d_n5, assign84040_e127551_d_n6, assign84040_e127551_d_n7, assign84040_e127551_d_n8, assign84040_e127551_d_n9, assign84040_e127551_d_n10, assign84040_e127551_d_n11, assign84040_e127551_d_n14,)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn14,)
    }
};
        locals.var_ps0ld = assign84040_e127553;
        locals.var_ps0ld_dn0 = assign84040_e127553_d_n0;
        locals.var_ps0ld_dn2 = assign84040_e127553_d_n2;
        locals.var_ps0ld_dn4 = assign84040_e127553_d_n4;
        locals.var_ps0ld_dn5 = assign84040_e127553_d_n5;
        locals.var_ps0ld_dn6 = assign84040_e127553_d_n6;
        locals.var_ps0ld_dn7 = assign84040_e127553_d_n7;
        locals.var_ps0ld_dn8 = assign84040_e127553_d_n8;
        locals.var_ps0ld_dn9 = assign84040_e127553_d_n9;
        locals.var_ps0ld_dn10 = assign84040_e127553_d_n10;
        locals.var_ps0ld_dn11 = assign84040_e127553_d_n11;
        locals.var_ps0ld_dn14 = assign84040_e127553_d_n14;

        let (assign84050_e127560, assign84050_e127560_d_n0, assign84050_e127560_d_n2, assign84050_e127560_d_n4, assign84050_e127560_d_n5, assign84050_e127560_d_n6, assign84050_e127560_d_n7, assign84050_e127560_d_n8, assign84050_e127560_d_n9, assign84050_e127560_d_n10, assign84050_e127560_d_n11, assign84050_e127560_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn14,)
    } else {
        (locals.var_ps0ld_ini__blk1892, locals.var_ps0ld_ini__blk1892_dn0, locals.var_ps0ld_ini__blk1892_dn2, locals.var_ps0ld_ini__blk1892_dn4, locals.var_ps0ld_ini__blk1892_dn5, locals.var_ps0ld_ini__blk1892_dn6, locals.var_ps0ld_ini__blk1892_dn7, locals.var_ps0ld_ini__blk1892_dn8, locals.var_ps0ld_ini__blk1892_dn9, locals.var_ps0ld_ini__blk1892_dn10, locals.var_ps0ld_ini__blk1892_dn11, locals.var_ps0ld_ini__blk1892_dn14,)
    }
};
        locals.var_ps0ld_ini__blk1892 = assign84050_e127560;
        locals.var_ps0ld_ini__blk1892_dn0 = assign84050_e127560_d_n0;
        locals.var_ps0ld_ini__blk1892_dn2 = assign84050_e127560_d_n2;
        locals.var_ps0ld_ini__blk1892_dn4 = assign84050_e127560_d_n4;
        locals.var_ps0ld_ini__blk1892_dn5 = assign84050_e127560_d_n5;
        locals.var_ps0ld_ini__blk1892_dn6 = assign84050_e127560_d_n6;
        locals.var_ps0ld_ini__blk1892_dn7 = assign84050_e127560_d_n7;
        locals.var_ps0ld_ini__blk1892_dn8 = assign84050_e127560_d_n8;
        locals.var_ps0ld_ini__blk1892_dn9 = assign84050_e127560_d_n9;
        locals.var_ps0ld_ini__blk1892_dn10 = assign84050_e127560_d_n10;
        locals.var_ps0ld_ini__blk1892_dn11 = assign84050_e127560_d_n11;
        locals.var_ps0ld_ini__blk1892_dn14 = assign84050_e127560_d_n14;

        let assign84060_e127563: f64 = if p.p33 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1948 = assign84060_e127563;

        let (assign84070_e127572,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1909 == 0.0)) && (locals.var_guard1948 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
        locals.var_flg_conv = assign84070_e127572;

    }
}
