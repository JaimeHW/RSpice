#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_345(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign89820_e137750, assign89820_e137750_d_n0, assign89820_e137750_d_n2, assign89820_e137750_d_n4, assign89820_e137750_d_n5, assign89820_e137750_d_n6, assign89820_e137750_d_n7, assign89820_e137750_d_n8, assign89820_e137750_d_n9, assign89820_e137750_d_n10, assign89820_e137750_d_n11, assign89820_e137750_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2087 != 0.0)) {
        let assign89820_e137737: f64 = (locals.var_t2 * locals.var_t2);
        let assign89820_e137741: f64 = (p.p334 * 0.01);
        let assign89820_e137742: f64 = (4.0 * assign89820_e137741);
        let assign89820_e137745: f64 = (p.p334 * 0.01);
        let assign89820_e137746: f64 = (assign89820_e137742 * assign89820_e137745);
        let assign89820_e137747: f64 = (assign89820_e137737 + assign89820_e137746);
        let assign89820_e137748: f64 = (assign89820_e137747).sqrt();
        (assign89820_e137748, (((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)) / (2.0 * assign89820_e137748)), (((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)) / (2.0 * assign89820_e137748)), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign89820_e137748)), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign89820_e137748)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign89820_e137748)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign89820_e137748)), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign89820_e137748)), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign89820_e137748)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign89820_e137748)), (((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign89820_e137748)), (((locals.var_t2_dn14 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn14)) / (2.0 * assign89820_e137748)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign89820_e137750;
        locals.var_tmf2_dn0 = assign89820_e137750_d_n0;
        locals.var_tmf2_dn2 = assign89820_e137750_d_n2;
        locals.var_tmf2_dn4 = assign89820_e137750_d_n4;
        locals.var_tmf2_dn5 = assign89820_e137750_d_n5;
        locals.var_tmf2_dn6 = assign89820_e137750_d_n6;
        locals.var_tmf2_dn7 = assign89820_e137750_d_n7;
        locals.var_tmf2_dn8 = assign89820_e137750_d_n8;
        locals.var_tmf2_dn9 = assign89820_e137750_d_n9;
        locals.var_tmf2_dn10 = assign89820_e137750_d_n10;
        locals.var_tmf2_dn11 = assign89820_e137750_d_n11;
        locals.var_tmf2_dn14 = assign89820_e137750_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign89830_e137762, assign89830_e137762_d_n0, assign89830_e137762_d_n2, assign89830_e137762_d_n4, assign89830_e137762_d_n5, assign89830_e137762_d_n6, assign89830_e137762_d_n7, assign89830_e137762_d_n8, assign89830_e137762_d_n9, assign89830_e137762_d_n10, assign89830_e137762_d_n11, assign89830_e137762_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2087 != 0.0)) {
        let assign89830_e137758: f64 = (locals.var_t2 / locals.var_tmf2);
        let assign89830_e137759: f64 = (1.0 + assign89830_e137758);
        let assign89830_e137760: f64 = (0.5 * assign89830_e137759);
        (assign89830_e137760, (0.5 * (((locals.var_t2_dn0 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn2 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn4 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn5 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn6 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn7 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn8 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn9 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn10 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn11 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn14 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign89830_e137762;
        locals.var_t9_dn0 = assign89830_e137762_d_n0;
        locals.var_t9_dn2 = assign89830_e137762_d_n2;
        locals.var_t9_dn4 = assign89830_e137762_d_n4;
        locals.var_t9_dn5 = assign89830_e137762_d_n5;
        locals.var_t9_dn6 = assign89830_e137762_d_n6;
        locals.var_t9_dn7 = assign89830_e137762_d_n7;
        locals.var_t9_dn8 = assign89830_e137762_d_n8;
        locals.var_t9_dn9 = assign89830_e137762_d_n9;
        locals.var_t9_dn10 = assign89830_e137762_d_n10;
        locals.var_t9_dn11 = assign89830_e137762_d_n11;
        locals.var_t9_dn14 = assign89830_e137762_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign89840_e137772, assign89840_e137772_d_n0, assign89840_e137772_d_n2, assign89840_e137772_d_n4, assign89840_e137772_d_n5, assign89840_e137772_d_n6, assign89840_e137772_d_n7, assign89840_e137772_d_n8, assign89840_e137772_d_n9, assign89840_e137772_d_n10, assign89840_e137772_d_n11, assign89840_e137772_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2087 != 0.0)) {
        let assign89840_e137769: f64 = (locals.var_t2 + locals.var_tmf2);
        let assign89840_e137770: f64 = (0.5 * assign89840_e137769);
        (assign89840_e137770, (0.5 * (locals.var_t2_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t2_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t2_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t2_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t2_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t2_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t2_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t2_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t2_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t2_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_t2_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign89840_e137772;
        locals.var_t2_dn0 = assign89840_e137772_d_n0;
        locals.var_t2_dn2 = assign89840_e137772_d_n2;
        locals.var_t2_dn4 = assign89840_e137772_d_n4;
        locals.var_t2_dn5 = assign89840_e137772_d_n5;
        locals.var_t2_dn6 = assign89840_e137772_d_n6;
        locals.var_t2_dn7 = assign89840_e137772_d_n7;
        locals.var_t2_dn8 = assign89840_e137772_d_n8;
        locals.var_t2_dn9 = assign89840_e137772_d_n9;
        locals.var_t2_dn10 = assign89840_e137772_d_n10;
        locals.var_t2_dn11 = assign89840_e137772_d_n11;
        locals.var_t2_dn14 = assign89840_e137772_d_n14;
        locals.var_t2_rv = 0.0;

        let assign89850_e137775: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2090 = assign89850_e137775;
        locals.var_guard2090_rv = 0.0;

        let (assign89860_e137783, assign89860_e137783_d_n0, assign89860_e137783_d_n2, assign89860_e137783_d_n4, assign89860_e137783_d_n5, assign89860_e137783_d_n6, assign89860_e137783_d_n7, assign89860_e137783_d_n8, assign89860_e137783_d_n9, assign89860_e137783_d_n10, assign89860_e137783_d_n11, assign89860_e137783_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2087 != 0.0)) && (locals.var_guard2090 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign89860_e137783;
        locals.var_t2_dn0 = assign89860_e137783_d_n0;
        locals.var_t2_dn2 = assign89860_e137783_d_n2;
        locals.var_t2_dn4 = assign89860_e137783_d_n4;
        locals.var_t2_dn5 = assign89860_e137783_d_n5;
        locals.var_t2_dn6 = assign89860_e137783_d_n6;
        locals.var_t2_dn7 = assign89860_e137783_d_n7;
        locals.var_t2_dn8 = assign89860_e137783_d_n8;
        locals.var_t2_dn9 = assign89860_e137783_d_n9;
        locals.var_t2_dn10 = assign89860_e137783_d_n10;
        locals.var_t2_dn11 = assign89860_e137783_d_n11;
        locals.var_t2_dn14 = assign89860_e137783_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign89870_e137791, assign89870_e137791_d_n0, assign89870_e137791_d_n2, assign89870_e137791_d_n4, assign89870_e137791_d_n5, assign89870_e137791_d_n6, assign89870_e137791_d_n7, assign89870_e137791_d_n8, assign89870_e137791_d_n9, assign89870_e137791_d_n10, assign89870_e137791_d_n11, assign89870_e137791_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2087 != 0.0)) && (locals.var_guard2090 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign89870_e137791;
        locals.var_t9_dn0 = assign89870_e137791_d_n0;
        locals.var_t9_dn2 = assign89870_e137791_d_n2;
        locals.var_t9_dn4 = assign89870_e137791_d_n4;
        locals.var_t9_dn5 = assign89870_e137791_d_n5;
        locals.var_t9_dn6 = assign89870_e137791_d_n6;
        locals.var_t9_dn7 = assign89870_e137791_d_n7;
        locals.var_t9_dn8 = assign89870_e137791_d_n8;
        locals.var_t9_dn9 = assign89870_e137791_d_n9;
        locals.var_t9_dn10 = assign89870_e137791_d_n10;
        locals.var_t9_dn11 = assign89870_e137791_d_n11;
        locals.var_t9_dn14 = assign89870_e137791_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign89880_e137797, assign89880_e137797_d_n0, assign89880_e137797_d_n2, assign89880_e137797_d_n4, assign89880_e137797_d_n5, assign89880_e137797_d_n6, assign89880_e137797_d_n7, assign89880_e137797_d_n8, assign89880_e137797_d_n9, assign89880_e137797_d_n10, assign89880_e137797_d_n11, assign89880_e137797_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2087 != 0.0)) {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    } else {
        (locals.var_ddriftldc, locals.var_ddriftldc_dn0, locals.var_ddriftldc_dn2, locals.var_ddriftldc_dn4, locals.var_ddriftldc_dn5, locals.var_ddriftldc_dn6, locals.var_ddriftldc_dn7, locals.var_ddriftldc_dn8, locals.var_ddriftldc_dn9, locals.var_ddriftldc_dn10, locals.var_ddriftldc_dn11, locals.var_ddriftldc_dn14,)
    }
};
        locals.var_ddriftldc = assign89880_e137797;
        locals.var_ddriftldc_dn0 = assign89880_e137797_d_n0;
        locals.var_ddriftldc_dn2 = assign89880_e137797_d_n2;
        locals.var_ddriftldc_dn4 = assign89880_e137797_d_n4;
        locals.var_ddriftldc_dn5 = assign89880_e137797_d_n5;
        locals.var_ddriftldc_dn6 = assign89880_e137797_d_n6;
        locals.var_ddriftldc_dn7 = assign89880_e137797_d_n7;
        locals.var_ddriftldc_dn8 = assign89880_e137797_d_n8;
        locals.var_ddriftldc_dn9 = assign89880_e137797_d_n9;
        locals.var_ddriftldc_dn10 = assign89880_e137797_d_n10;
        locals.var_ddriftldc_dn11 = assign89880_e137797_d_n11;
        locals.var_ddriftldc_dn14 = assign89880_e137797_d_n14;
        locals.var_ddriftldc_rv = 0.0;

        let (assign89890_e137811, assign89890_e137811_d_n0, assign89890_e137811_d_n2, assign89890_e137811_d_n4, assign89890_e137811_d_n5, assign89890_e137811_d_n6, assign89890_e137811_d_n7, assign89890_e137811_d_n8, assign89890_e137811_d_n9, assign89890_e137811_d_n10, assign89890_e137811_d_n11, assign89890_e137811_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2087 != 0.0)) {
        let assign89890_e137803: f64 = (locals.var_q_nsubld__blk2006 * locals.var_ddriftldc);
        let assign89890_e137805: f64 = (assign89890_e137803 * locals.var_ddriftldc);
        let assign89890_e137807: f64 = (assign89890_e137805 / 2.0);
        let assign89890_e137809: f64 = (assign89890_e137807 / 1.034943e-10);
        (assign89890_e137809, (((((locals.var_q_nsubld__blk2006 * locals.var_ddriftldc_dn0) * locals.var_ddriftldc) + (assign89890_e137803 * locals.var_ddriftldc_dn0)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2006 * locals.var_ddriftldc_dn2) * locals.var_ddriftldc) + (assign89890_e137803 * locals.var_ddriftldc_dn2)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2006 * locals.var_ddriftldc_dn4) * locals.var_ddriftldc) + (assign89890_e137803 * locals.var_ddriftldc_dn4)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2006 * locals.var_ddriftldc_dn5) * locals.var_ddriftldc) + (assign89890_e137803 * locals.var_ddriftldc_dn5)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2006 * locals.var_ddriftldc_dn6) * locals.var_ddriftldc) + (assign89890_e137803 * locals.var_ddriftldc_dn6)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2006 * locals.var_ddriftldc_dn7) * locals.var_ddriftldc) + (assign89890_e137803 * locals.var_ddriftldc_dn7)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2006 * locals.var_ddriftldc_dn8) * locals.var_ddriftldc) + (assign89890_e137803 * locals.var_ddriftldc_dn8)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2006 * locals.var_ddriftldc_dn9) * locals.var_ddriftldc) + (assign89890_e137803 * locals.var_ddriftldc_dn9)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2006 * locals.var_ddriftldc_dn10) * locals.var_ddriftldc) + (assign89890_e137803 * locals.var_ddriftldc_dn10)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2006 * locals.var_ddriftldc_dn11) * locals.var_ddriftldc) + (assign89890_e137803 * locals.var_ddriftldc_dn11)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2006 * locals.var_ddriftldc_dn14) * locals.var_ddriftldc) + (assign89890_e137803 * locals.var_ddriftldc_dn14)) / 2.0) / 1.034943e-10),)
    } else {
        (locals.var_dphi_sb, locals.var_dphi_sb_dn0, locals.var_dphi_sb_dn2, locals.var_dphi_sb_dn4, locals.var_dphi_sb_dn5, locals.var_dphi_sb_dn6, locals.var_dphi_sb_dn7, locals.var_dphi_sb_dn8, locals.var_dphi_sb_dn9, locals.var_dphi_sb_dn10, locals.var_dphi_sb_dn11, locals.var_dphi_sb_dn14,)
    }
};
        locals.var_dphi_sb = assign89890_e137811;
        locals.var_dphi_sb_dn0 = assign89890_e137811_d_n0;
        locals.var_dphi_sb_dn2 = assign89890_e137811_d_n2;
        locals.var_dphi_sb_dn4 = assign89890_e137811_d_n4;
        locals.var_dphi_sb_dn5 = assign89890_e137811_d_n5;
        locals.var_dphi_sb_dn6 = assign89890_e137811_d_n6;
        locals.var_dphi_sb_dn7 = assign89890_e137811_d_n7;
        locals.var_dphi_sb_dn8 = assign89890_e137811_d_n8;
        locals.var_dphi_sb_dn9 = assign89890_e137811_d_n9;
        locals.var_dphi_sb_dn10 = assign89890_e137811_d_n10;
        locals.var_dphi_sb_dn11 = assign89890_e137811_d_n11;
        locals.var_dphi_sb_dn14 = assign89890_e137811_d_n14;
        locals.var_dphi_sb_rv = 0.0;

        let (assign89900_e137822, assign89900_e137822_d_n0, assign89900_e137822_d_n2, assign89900_e137822_d_n4, assign89900_e137822_d_n5, assign89900_e137822_d_n6, assign89900_e137822_d_n7, assign89900_e137822_d_n8, assign89900_e137822_d_n9, assign89900_e137822_d_n10, assign89900_e137822_d_n11, assign89900_e137822_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2087 != 0.0)) {
        let assign89900_e137817: f64 = (2.0 * locals.var_beta);
        let assign89900_e137819: f64 = (assign89900_e137817 * locals.var_dphi_sb);
        let assign89900_e137820: f64 = (assign89900_e137819).sqrt();
        (assign89900_e137820, ((((2.0 * locals.var_beta_dn0) * locals.var_dphi_sb) + (assign89900_e137817 * locals.var_dphi_sb_dn0)) / (2.0 * assign89900_e137820)), ((((2.0 * locals.var_beta_dn2) * locals.var_dphi_sb) + (assign89900_e137817 * locals.var_dphi_sb_dn2)) / (2.0 * assign89900_e137820)), ((((2.0 * locals.var_beta_dn4) * locals.var_dphi_sb) + (assign89900_e137817 * locals.var_dphi_sb_dn4)) / (2.0 * assign89900_e137820)), ((((2.0 * locals.var_beta_dn5) * locals.var_dphi_sb) + (assign89900_e137817 * locals.var_dphi_sb_dn5)) / (2.0 * assign89900_e137820)), ((((2.0 * locals.var_beta_dn6) * locals.var_dphi_sb) + (assign89900_e137817 * locals.var_dphi_sb_dn6)) / (2.0 * assign89900_e137820)), ((((2.0 * locals.var_beta_dn7) * locals.var_dphi_sb) + (assign89900_e137817 * locals.var_dphi_sb_dn7)) / (2.0 * assign89900_e137820)), ((((2.0 * locals.var_beta_dn8) * locals.var_dphi_sb) + (assign89900_e137817 * locals.var_dphi_sb_dn8)) / (2.0 * assign89900_e137820)), ((((2.0 * locals.var_beta_dn9) * locals.var_dphi_sb) + (assign89900_e137817 * locals.var_dphi_sb_dn9)) / (2.0 * assign89900_e137820)), ((((2.0 * locals.var_beta_dn10) * locals.var_dphi_sb) + (assign89900_e137817 * locals.var_dphi_sb_dn10)) / (2.0 * assign89900_e137820)), ((((2.0 * locals.var_beta_dn11) * locals.var_dphi_sb) + (assign89900_e137817 * locals.var_dphi_sb_dn11)) / (2.0 * assign89900_e137820)), ((((2.0 * locals.var_beta_dn14) * locals.var_dphi_sb) + (assign89900_e137817 * locals.var_dphi_sb_dn14)) / (2.0 * assign89900_e137820)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign89900_e137822;
        locals.var_t0_dn0 = assign89900_e137822_d_n0;
        locals.var_t0_dn2 = assign89900_e137822_d_n2;
        locals.var_t0_dn4 = assign89900_e137822_d_n4;
        locals.var_t0_dn5 = assign89900_e137822_d_n5;
        locals.var_t0_dn6 = assign89900_e137822_d_n6;
        locals.var_t0_dn7 = assign89900_e137822_d_n7;
        locals.var_t0_dn8 = assign89900_e137822_d_n8;
        locals.var_t0_dn9 = assign89900_e137822_d_n9;
        locals.var_t0_dn10 = assign89900_e137822_d_n10;
        locals.var_t0_dn11 = assign89900_e137822_d_n11;
        locals.var_t0_dn14 = assign89900_e137822_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign89910_e137835, assign89910_e137835_d_n0, assign89910_e137835_d_n2, assign89910_e137835_d_n4, assign89910_e137835_d_n5, assign89910_e137835_d_n6, assign89910_e137835_d_n7, assign89910_e137835_d_n8, assign89910_e137835_d_n9, assign89910_e137835_d_n10, assign89910_e137835_d_n11, assign89910_e137835_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2087 != 0.0)) {
        let assign89910_e137827: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign89910_e137829: f64 = (-locals.var_t0);
        let assign89910_e137830: f64 = { let limited_exp_arg = assign89910_e137829; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign89910_e137831: f64 = (assign89910_e137827 + assign89910_e137830);
        let assign89910_e137833: f64 = (assign89910_e137831 / 2.0);
        (assign89910_e137833, ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn0) + ({ let limited_exp_arg = assign89910_e137829; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn0))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn2) + ({ let limited_exp_arg = assign89910_e137829; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn2))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4) + ({ let limited_exp_arg = assign89910_e137829; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn4))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5) + ({ let limited_exp_arg = assign89910_e137829; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn5))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6) + ({ let limited_exp_arg = assign89910_e137829; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn6))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7) + ({ let limited_exp_arg = assign89910_e137829; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn7))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8) + ({ let limited_exp_arg = assign89910_e137829; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn8))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9) + ({ let limited_exp_arg = assign89910_e137829; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn9))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10) + ({ let limited_exp_arg = assign89910_e137829; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn10))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11) + ({ let limited_exp_arg = assign89910_e137829; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn11))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn14) + ({ let limited_exp_arg = assign89910_e137829; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn14))) / 2.0),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign89910_e137835;
        locals.var_t1_dn0 = assign89910_e137835_d_n0;
        locals.var_t1_dn2 = assign89910_e137835_d_n2;
        locals.var_t1_dn4 = assign89910_e137835_d_n4;
        locals.var_t1_dn5 = assign89910_e137835_d_n5;
        locals.var_t1_dn6 = assign89910_e137835_d_n6;
        locals.var_t1_dn7 = assign89910_e137835_d_n7;
        locals.var_t1_dn8 = assign89910_e137835_d_n8;
        locals.var_t1_dn9 = assign89910_e137835_d_n9;
        locals.var_t1_dn10 = assign89910_e137835_d_n10;
        locals.var_t1_dn11 = assign89910_e137835_d_n11;
        locals.var_t1_dn14 = assign89910_e137835_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign89920_e137844, assign89920_e137844_d_n0, assign89920_e137844_d_n2, assign89920_e137844_d_n4, assign89920_e137844_d_n5, assign89920_e137844_d_n6, assign89920_e137844_d_n7, assign89920_e137844_d_n8, assign89920_e137844_d_n9, assign89920_e137844_d_n10, assign89920_e137844_d_n11, assign89920_e137844_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2087 != 0.0)) {
        let assign89920_e137840: f64 = (locals.var_t1).ln();
        let assign89920_e137842: f64 = (assign89920_e137840 / locals.var_dphi_sb);
        (assign89920_e137842, ((((locals.var_t1_dn0 / locals.var_t1) * locals.var_dphi_sb) - (assign89920_e137840 * locals.var_dphi_sb_dn0)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn2 / locals.var_t1) * locals.var_dphi_sb) - (assign89920_e137840 * locals.var_dphi_sb_dn2)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn4 / locals.var_t1) * locals.var_dphi_sb) - (assign89920_e137840 * locals.var_dphi_sb_dn4)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn5 / locals.var_t1) * locals.var_dphi_sb) - (assign89920_e137840 * locals.var_dphi_sb_dn5)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn6 / locals.var_t1) * locals.var_dphi_sb) - (assign89920_e137840 * locals.var_dphi_sb_dn6)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn7 / locals.var_t1) * locals.var_dphi_sb) - (assign89920_e137840 * locals.var_dphi_sb_dn7)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn8 / locals.var_t1) * locals.var_dphi_sb) - (assign89920_e137840 * locals.var_dphi_sb_dn8)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn9 / locals.var_t1) * locals.var_dphi_sb) - (assign89920_e137840 * locals.var_dphi_sb_dn9)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn10 / locals.var_t1) * locals.var_dphi_sb) - (assign89920_e137840 * locals.var_dphi_sb_dn10)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn11 / locals.var_t1) * locals.var_dphi_sb) - (assign89920_e137840 * locals.var_dphi_sb_dn11)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn14 / locals.var_t1) * locals.var_dphi_sb) - (assign89920_e137840 * locals.var_dphi_sb_dn14)) / (locals.var_dphi_sb * locals.var_dphi_sb)),)
    } else {
        (locals.var_c_sb, locals.var_c_sb_dn0, locals.var_c_sb_dn2, locals.var_c_sb_dn4, locals.var_c_sb_dn5, locals.var_c_sb_dn6, locals.var_c_sb_dn7, locals.var_c_sb_dn8, locals.var_c_sb_dn9, locals.var_c_sb_dn10, locals.var_c_sb_dn11, locals.var_c_sb_dn14,)
    }
};
        locals.var_c_sb = assign89920_e137844;
        locals.var_c_sb_dn0 = assign89920_e137844_d_n0;
        locals.var_c_sb_dn2 = assign89920_e137844_d_n2;
        locals.var_c_sb_dn4 = assign89920_e137844_d_n4;
        locals.var_c_sb_dn5 = assign89920_e137844_d_n5;
        locals.var_c_sb_dn6 = assign89920_e137844_d_n6;
        locals.var_c_sb_dn7 = assign89920_e137844_d_n7;
        locals.var_c_sb_dn8 = assign89920_e137844_d_n8;
        locals.var_c_sb_dn9 = assign89920_e137844_d_n9;
        locals.var_c_sb_dn10 = assign89920_e137844_d_n10;
        locals.var_c_sb_dn11 = assign89920_e137844_d_n11;
        locals.var_c_sb_dn14 = assign89920_e137844_d_n14;
        locals.var_c_sb_rv = 0.0;

        let (assign89930_e137850,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2087 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign89930_e137850;
        locals.var_lp_s0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_346(
        locals: &mut StampLocals,
    ) {
        let mut assign89940_loop_guard: usize = 0;
        while {
            let assign89940_cond_e137857: f64 = (locals.var_lp_s0_max + 1.0);
            let assign89940_cond_e137859: f64 = if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2087 != 0.0)) && (locals.var_lp_s0 <= assign89940_cond_e137857)) { 1.0 } else { 0.0 };
            assign89940_cond_e137859 != 0.0
        } {
            assign89940_loop_guard += 1;
            assert!(assign89940_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign89940_body3_e137886, assign89940_body3_e137886_d_n0, assign89940_body3_e137886_d_n2, assign89940_body3_e137886_d_n4, assign89940_body3_e137886_d_n5, assign89940_body3_e137886_d_n6, assign89940_body3_e137886_d_n7, assign89940_body3_e137886_d_n8, assign89940_body3_e137886_d_n9, assign89940_body3_e137886_d_n10, assign89940_body3_e137886_d_n11, assign89940_body3_e137886_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2087 != 0.0)) {
        let assign89940_body3_e137884: f64 = (locals.var_ps0ld + locals.var_vxbgmtcl);
        (assign89940_body3_e137884, (locals.var_ps0ld_dn0 + locals.var_vxbgmtcl_dn0), (locals.var_ps0ld_dn2 + locals.var_vxbgmtcl_dn2), (locals.var_ps0ld_dn4 + locals.var_vxbgmtcl_dn4), (locals.var_ps0ld_dn5 + locals.var_vxbgmtcl_dn5), (locals.var_ps0ld_dn6 + locals.var_vxbgmtcl_dn6), (locals.var_ps0ld_dn7 + locals.var_vxbgmtcl_dn7), (locals.var_ps0ld_dn8 + locals.var_vxbgmtcl_dn8), (locals.var_ps0ld_dn9 + locals.var_vxbgmtcl_dn9), (locals.var_ps0ld_dn10 + locals.var_vxbgmtcl_dn10), (locals.var_ps0ld_dn11 + locals.var_vxbgmtcl_dn11), (locals.var_ps0ld_dn14 + locals.var_vxbgmtcl_dn14),)
    } else {
        (locals.var_ps0ld_vxb, locals.var_ps0ld_vxb_dn0, locals.var_ps0ld_vxb_dn2, locals.var_ps0ld_vxb_dn4, locals.var_ps0ld_vxb_dn5, locals.var_ps0ld_vxb_dn6, locals.var_ps0ld_vxb_dn7, locals.var_ps0ld_vxb_dn8, locals.var_ps0ld_vxb_dn9, locals.var_ps0ld_vxb_dn10, locals.var_ps0ld_vxb_dn11, locals.var_ps0ld_vxb_dn14,)
    }
};
            locals.var_ps0ld_vxb = assign89940_body3_e137886;
            locals.var_ps0ld_vxb_dn0 = assign89940_body3_e137886_d_n0;
            locals.var_ps0ld_vxb_dn2 = assign89940_body3_e137886_d_n2;
            locals.var_ps0ld_vxb_dn4 = assign89940_body3_e137886_d_n4;
            locals.var_ps0ld_vxb_dn5 = assign89940_body3_e137886_d_n5;
            locals.var_ps0ld_vxb_dn6 = assign89940_body3_e137886_d_n6;
            locals.var_ps0ld_vxb_dn7 = assign89940_body3_e137886_d_n7;
            locals.var_ps0ld_vxb_dn8 = assign89940_body3_e137886_d_n8;
            locals.var_ps0ld_vxb_dn9 = assign89940_body3_e137886_d_n9;
            locals.var_ps0ld_vxb_dn10 = assign89940_body3_e137886_d_n10;
            locals.var_ps0ld_vxb_dn11 = assign89940_body3_e137886_d_n11;
            locals.var_ps0ld_vxb_dn14 = assign89940_body3_e137886_d_n14;
            locals.var_ps0ld_vxb_rv = 0.0;
            let (assign89940_body4_e137894, assign89940_body4_e137894_d_n0, assign89940_body4_e137894_d_n2, assign89940_body4_e137894_d_n4, assign89940_body4_e137894_d_n5, assign89940_body4_e137894_d_n6, assign89940_body4_e137894_d_n7, assign89940_body4_e137894_d_n8, assign89940_body4_e137894_d_n9, assign89940_body4_e137894_d_n10, assign89940_body4_e137894_d_n11, assign89940_body4_e137894_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2087 != 0.0)) {
        let assign89940_body4_e137892: f64 = (locals.var_beta * locals.var_ps0ld_vxb);
        (assign89940_body4_e137892, ((locals.var_beta_dn0 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn0)), ((locals.var_beta_dn2 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn2)), ((locals.var_beta_dn4 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn4)), ((locals.var_beta_dn5 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn5)), ((locals.var_beta_dn6 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn6)), ((locals.var_beta_dn7 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn7)), ((locals.var_beta_dn8 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn8)), ((locals.var_beta_dn9 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn9)), ((locals.var_beta_dn10 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn10)), ((locals.var_beta_dn11 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn11)), ((locals.var_beta_dn14 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn14)),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
            locals.var_chi = assign89940_body4_e137894;
            locals.var_chi_dn0 = assign89940_body4_e137894_d_n0;
            locals.var_chi_dn2 = assign89940_body4_e137894_d_n2;
            locals.var_chi_dn4 = assign89940_body4_e137894_d_n4;
            locals.var_chi_dn5 = assign89940_body4_e137894_d_n5;
            locals.var_chi_dn6 = assign89940_body4_e137894_d_n6;
            locals.var_chi_dn7 = assign89940_body4_e137894_d_n7;
            locals.var_chi_dn8 = assign89940_body4_e137894_d_n8;
            locals.var_chi_dn9 = assign89940_body4_e137894_d_n9;
            locals.var_chi_dn10 = assign89940_body4_e137894_d_n10;
            locals.var_chi_dn11 = assign89940_body4_e137894_d_n11;
            locals.var_chi_dn14 = assign89940_body4_e137894_d_n14;
            locals.var_chi_rv = 0.0;
            let (assign89940_body5_e137904, assign89940_body5_e137904_d_n0, assign89940_body5_e137904_d_n2, assign89940_body5_e137904_d_n4, assign89940_body5_e137904_d_n5, assign89940_body5_e137904_d_n6, assign89940_body5_e137904_d_n7, assign89940_body5_e137904_d_n8, assign89940_body5_e137904_d_n9, assign89940_body5_e137904_d_n10, assign89940_body5_e137904_d_n11, assign89940_body5_e137904_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2087 != 0.0)) {
        let assign89940_body5_e137901: f64 = (locals.var_ps0ld_vxb - locals.var_dphi_sb);
        let assign89940_body5_e137902: f64 = (locals.var_c_sb * assign89940_body5_e137901);
        (assign89940_body5_e137902, ((locals.var_c_sb_dn0 * assign89940_body5_e137901) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn0 - locals.var_dphi_sb_dn0))), ((locals.var_c_sb_dn2 * assign89940_body5_e137901) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn2 - locals.var_dphi_sb_dn2))), ((locals.var_c_sb_dn4 * assign89940_body5_e137901) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn4 - locals.var_dphi_sb_dn4))), ((locals.var_c_sb_dn5 * assign89940_body5_e137901) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn5 - locals.var_dphi_sb_dn5))), ((locals.var_c_sb_dn6 * assign89940_body5_e137901) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn6 - locals.var_dphi_sb_dn6))), ((locals.var_c_sb_dn7 * assign89940_body5_e137901) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn7 - locals.var_dphi_sb_dn7))), ((locals.var_c_sb_dn8 * assign89940_body5_e137901) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn8 - locals.var_dphi_sb_dn8))), ((locals.var_c_sb_dn9 * assign89940_body5_e137901) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn9 - locals.var_dphi_sb_dn9))), ((locals.var_c_sb_dn10 * assign89940_body5_e137901) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn10 - locals.var_dphi_sb_dn10))), ((locals.var_c_sb_dn11 * assign89940_body5_e137901) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn11 - locals.var_dphi_sb_dn11))), ((locals.var_c_sb_dn14 * assign89940_body5_e137901) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn14 - locals.var_dphi_sb_dn14))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn14,)
    }
};
            locals.var_ty = assign89940_body5_e137904;
            locals.var_ty_dn0 = assign89940_body5_e137904_d_n0;
            locals.var_ty_dn2 = assign89940_body5_e137904_d_n2;
            locals.var_ty_dn4 = assign89940_body5_e137904_d_n4;
            locals.var_ty_dn5 = assign89940_body5_e137904_d_n5;
            locals.var_ty_dn6 = assign89940_body5_e137904_d_n6;
            locals.var_ty_dn7 = assign89940_body5_e137904_d_n7;
            locals.var_ty_dn8 = assign89940_body5_e137904_d_n8;
            locals.var_ty_dn9 = assign89940_body5_e137904_d_n9;
            locals.var_ty_dn10 = assign89940_body5_e137904_d_n10;
            locals.var_ty_dn11 = assign89940_body5_e137904_d_n11;
            locals.var_ty_dn14 = assign89940_body5_e137904_d_n14;
            locals.var_ty_rv = 0.0;
            let assign89940_body6_e137907: f64 = if locals.var_ty < 60.0 { 1.0 } else { 0.0 };
            locals.var_guard2092 = assign89940_body6_e137907;
            locals.var_guard2092_rv = 0.0;
            let (assign89940_body7_e137916, assign89940_body7_e137916_d_n0, assign89940_body7_e137916_d_n2, assign89940_body7_e137916_d_n4, assign89940_body7_e137916_d_n5, assign89940_body7_e137916_d_n6, assign89940_body7_e137916_d_n7, assign89940_body7_e137916_d_n8, assign89940_body7_e137916_d_n9, assign89940_body7_e137916_d_n10, assign89940_body7_e137916_d_n11, assign89940_body7_e137916_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2087 != 0.0)) && (locals.var_guard2092 != 0.0)) {
        let assign89940_body7_e137914: f64 = (locals.var_ty).exp();
        (assign89940_body7_e137914, (assign89940_body7_e137914 * locals.var_ty_dn0), (assign89940_body7_e137914 * locals.var_ty_dn2), (assign89940_body7_e137914 * locals.var_ty_dn4), (assign89940_body7_e137914 * locals.var_ty_dn5), (assign89940_body7_e137914 * locals.var_ty_dn6), (assign89940_body7_e137914 * locals.var_ty_dn7), (assign89940_body7_e137914 * locals.var_ty_dn8), (assign89940_body7_e137914 * locals.var_ty_dn9), (assign89940_body7_e137914 * locals.var_ty_dn10), (assign89940_body7_e137914 * locals.var_ty_dn11), (assign89940_body7_e137914 * locals.var_ty_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
            locals.var_t1 = assign89940_body7_e137916;
            locals.var_t1_dn0 = assign89940_body7_e137916_d_n0;
            locals.var_t1_dn2 = assign89940_body7_e137916_d_n2;
            locals.var_t1_dn4 = assign89940_body7_e137916_d_n4;
            locals.var_t1_dn5 = assign89940_body7_e137916_d_n5;
            locals.var_t1_dn6 = assign89940_body7_e137916_d_n6;
            locals.var_t1_dn7 = assign89940_body7_e137916_d_n7;
            locals.var_t1_dn8 = assign89940_body7_e137916_d_n8;
            locals.var_t1_dn9 = assign89940_body7_e137916_d_n9;
            locals.var_t1_dn10 = assign89940_body7_e137916_d_n10;
            locals.var_t1_dn11 = assign89940_body7_e137916_d_n11;
            locals.var_t1_dn14 = assign89940_body7_e137916_d_n14;
            locals.var_t1_rv = 0.0;
            let (assign89940_body8_e137928, assign89940_body8_e137928_d_n0, assign89940_body8_e137928_d_n2, assign89940_body8_e137928_d_n4, assign89940_body8_e137928_d_n5, assign89940_body8_e137928_d_n6, assign89940_body8_e137928_d_n7, assign89940_body8_e137928_d_n8, assign89940_body8_e137928_d_n9, assign89940_body8_e137928_d_n10, assign89940_body8_e137928_d_n11, assign89940_body8_e137928_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2087 != 0.0)) && (locals.var_guard2092 != 0.0)) {
        let assign89940_body8_e137923: f64 = (-locals.var_c_sb);
        let assign89940_body8_e137925: f64 = (assign89940_body8_e137923 * locals.var_dphi_sb);
        let assign89940_body8_e137926: f64 = (assign89940_body8_e137925).exp();
        (assign89940_body8_e137926, (assign89940_body8_e137926 * (((-locals.var_c_sb_dn0) * locals.var_dphi_sb) + (assign89940_body8_e137923 * locals.var_dphi_sb_dn0))), (assign89940_body8_e137926 * (((-locals.var_c_sb_dn2) * locals.var_dphi_sb) + (assign89940_body8_e137923 * locals.var_dphi_sb_dn2))), (assign89940_body8_e137926 * (((-locals.var_c_sb_dn4) * locals.var_dphi_sb) + (assign89940_body8_e137923 * locals.var_dphi_sb_dn4))), (assign89940_body8_e137926 * (((-locals.var_c_sb_dn5) * locals.var_dphi_sb) + (assign89940_body8_e137923 * locals.var_dphi_sb_dn5))), (assign89940_body8_e137926 * (((-locals.var_c_sb_dn6) * locals.var_dphi_sb) + (assign89940_body8_e137923 * locals.var_dphi_sb_dn6))), (assign89940_body8_e137926 * (((-locals.var_c_sb_dn7) * locals.var_dphi_sb) + (assign89940_body8_e137923 * locals.var_dphi_sb_dn7))), (assign89940_body8_e137926 * (((-locals.var_c_sb_dn8) * locals.var_dphi_sb) + (assign89940_body8_e137923 * locals.var_dphi_sb_dn8))), (assign89940_body8_e137926 * (((-locals.var_c_sb_dn9) * locals.var_dphi_sb) + (assign89940_body8_e137923 * locals.var_dphi_sb_dn9))), (assign89940_body8_e137926 * (((-locals.var_c_sb_dn10) * locals.var_dphi_sb) + (assign89940_body8_e137923 * locals.var_dphi_sb_dn10))), (assign89940_body8_e137926 * (((-locals.var_c_sb_dn11) * locals.var_dphi_sb) + (assign89940_body8_e137923 * locals.var_dphi_sb_dn11))), (assign89940_body8_e137926 * (((-locals.var_c_sb_dn14) * locals.var_dphi_sb) + (assign89940_body8_e137923 * locals.var_dphi_sb_dn14))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
            locals.var_t0 = assign89940_body8_e137928;
            locals.var_t0_dn0 = assign89940_body8_e137928_d_n0;
            locals.var_t0_dn2 = assign89940_body8_e137928_d_n2;
            locals.var_t0_dn4 = assign89940_body8_e137928_d_n4;
            locals.var_t0_dn5 = assign89940_body8_e137928_d_n5;
            locals.var_t0_dn6 = assign89940_body8_e137928_d_n6;
            locals.var_t0_dn7 = assign89940_body8_e137928_d_n7;
            locals.var_t0_dn8 = assign89940_body8_e137928_d_n8;
            locals.var_t0_dn9 = assign89940_body8_e137928_d_n9;
            locals.var_t0_dn10 = assign89940_body8_e137928_d_n10;
            locals.var_t0_dn11 = assign89940_body8_e137928_d_n11;
            locals.var_t0_dn14 = assign89940_body8_e137928_d_n14;
            locals.var_t0_rv = 0.0;
            let (assign89940_body9_e137938, assign89940_body9_e137938_d_n0, assign89940_body9_e137938_d_n2, assign89940_body9_e137938_d_n4, assign89940_body9_e137938_d_n5, assign89940_body9_e137938_d_n6, assign89940_body9_e137938_d_n7, assign89940_body9_e137938_d_n8, assign89940_body9_e137938_d_n9, assign89940_body9_e137938_d_n10, assign89940_body9_e137938_d_n11, assign89940_body9_e137938_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2087 != 0.0)) && (locals.var_guard2092 != 0.0)) {
        let assign89940_body9_e137936: f64 = (locals.var_t1 - locals.var_t0);
        (assign89940_body9_e137936, (locals.var_t1_dn0 - locals.var_t0_dn0), (locals.var_t1_dn2 - locals.var_t0_dn2), (locals.var_t1_dn4 - locals.var_t0_dn4), (locals.var_t1_dn5 - locals.var_t0_dn5), (locals.var_t1_dn6 - locals.var_t0_dn6), (locals.var_t1_dn7 - locals.var_t0_dn7), (locals.var_t1_dn8 - locals.var_t0_dn8), (locals.var_t1_dn9 - locals.var_t0_dn9), (locals.var_t1_dn10 - locals.var_t0_dn10), (locals.var_t1_dn11 - locals.var_t0_dn11), (locals.var_t1_dn14 - locals.var_t0_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
            locals.var_t2 = assign89940_body9_e137938;
            locals.var_t2_dn0 = assign89940_body9_e137938_d_n0;
            locals.var_t2_dn2 = assign89940_body9_e137938_d_n2;
            locals.var_t2_dn4 = assign89940_body9_e137938_d_n4;
            locals.var_t2_dn5 = assign89940_body9_e137938_d_n5;
            locals.var_t2_dn6 = assign89940_body9_e137938_d_n6;
            locals.var_t2_dn7 = assign89940_body9_e137938_d_n7;
            locals.var_t2_dn8 = assign89940_body9_e137938_d_n8;
            locals.var_t2_dn9 = assign89940_body9_e137938_d_n9;
            locals.var_t2_dn10 = assign89940_body9_e137938_d_n10;
            locals.var_t2_dn11 = assign89940_body9_e137938_d_n11;
            locals.var_t2_dn14 = assign89940_body9_e137938_d_n14;
            locals.var_t2_rv = 0.0;
            let (assign89940_body10_e137951, assign89940_body10_e137951_d_n0, assign89940_body10_e137951_d_n2, assign89940_body10_e137951_d_n4, assign89940_body10_e137951_d_n5, assign89940_body10_e137951_d_n6, assign89940_body10_e137951_d_n7, assign89940_body10_e137951_d_n8, assign89940_body10_e137951_d_n9, assign89940_body10_e137951_d_n10, assign89940_body10_e137951_d_n11, assign89940_body10_e137951_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2087 != 0.0)) && (locals.var_guard2092 != 0.0)) {
        let assign89940_body10_e137946: f64 = (1.0 + locals.var_t2);
        let assign89940_body10_e137947: f64 = (assign89940_body10_e137946).ln();
        let assign89940_body10_e137949: f64 = (assign89940_body10_e137947 / locals.var_c_sb);
        (assign89940_body10_e137949, ((((locals.var_t2_dn0 / assign89940_body10_e137946) * locals.var_c_sb) - (assign89940_body10_e137947 * locals.var_c_sb_dn0)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn2 / assign89940_body10_e137946) * locals.var_c_sb) - (assign89940_body10_e137947 * locals.var_c_sb_dn2)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn4 / assign89940_body10_e137946) * locals.var_c_sb) - (assign89940_body10_e137947 * locals.var_c_sb_dn4)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn5 / assign89940_body10_e137946) * locals.var_c_sb) - (assign89940_body10_e137947 * locals.var_c_sb_dn5)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn6 / assign89940_body10_e137946) * locals.var_c_sb) - (assign89940_body10_e137947 * locals.var_c_sb_dn6)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn7 / assign89940_body10_e137946) * locals.var_c_sb) - (assign89940_body10_e137947 * locals.var_c_sb_dn7)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn8 / assign89940_body10_e137946) * locals.var_c_sb) - (assign89940_body10_e137947 * locals.var_c_sb_dn8)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn9 / assign89940_body10_e137946) * locals.var_c_sb) - (assign89940_body10_e137947 * locals.var_c_sb_dn9)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn10 / assign89940_body10_e137946) * locals.var_c_sb) - (assign89940_body10_e137947 * locals.var_c_sb_dn10)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn11 / assign89940_body10_e137946) * locals.var_c_sb) - (assign89940_body10_e137947 * locals.var_c_sb_dn11)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn14 / assign89940_body10_e137946) * locals.var_c_sb) - (assign89940_body10_e137947 * locals.var_c_sb_dn14)) / (locals.var_c_sb * locals.var_c_sb)),)
    } else {
        (locals.var_phi_b, locals.var_phi_b_dn0, locals.var_phi_b_dn2, locals.var_phi_b_dn4, locals.var_phi_b_dn5, locals.var_phi_b_dn6, locals.var_phi_b_dn7, locals.var_phi_b_dn8, locals.var_phi_b_dn9, locals.var_phi_b_dn10, locals.var_phi_b_dn11, locals.var_phi_b_dn14,)
    }
};
            locals.var_phi_b = assign89940_body10_e137951;
            locals.var_phi_b_dn0 = assign89940_body10_e137951_d_n0;
            locals.var_phi_b_dn2 = assign89940_body10_e137951_d_n2;
            locals.var_phi_b_dn4 = assign89940_body10_e137951_d_n4;
            locals.var_phi_b_dn5 = assign89940_body10_e137951_d_n5;
            locals.var_phi_b_dn6 = assign89940_body10_e137951_d_n6;
            locals.var_phi_b_dn7 = assign89940_body10_e137951_d_n7;
            locals.var_phi_b_dn8 = assign89940_body10_e137951_d_n8;
            locals.var_phi_b_dn9 = assign89940_body10_e137951_d_n9;
            locals.var_phi_b_dn10 = assign89940_body10_e137951_d_n10;
            locals.var_phi_b_dn11 = assign89940_body10_e137951_d_n11;
            locals.var_phi_b_dn14 = assign89940_body10_e137951_d_n14;
            locals.var_phi_b_rv = 0.0;
            let (assign89940_body11_e137963, assign89940_body11_e137963_d_n0, assign89940_body11_e137963_d_n2, assign89940_body11_e137963_d_n4, assign89940_body11_e137963_d_n5, assign89940_body11_e137963_d_n6, assign89940_body11_e137963_d_n7, assign89940_body11_e137963_d_n8, assign89940_body11_e137963_d_n9, assign89940_body11_e137963_d_n10, assign89940_body11_e137963_d_n11, assign89940_body11_e137963_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2087 != 0.0)) && (locals.var_guard2092 != 0.0)) {
        let assign89940_body11_e137960: f64 = (1.0 + locals.var_t2);
        let assign89940_body11_e137961: f64 = (locals.var_t1 / assign89940_body11_e137960);
        (assign89940_body11_e137961, (((locals.var_t1_dn0 * assign89940_body11_e137960) - (locals.var_t1 * locals.var_t2_dn0)) / (assign89940_body11_e137960 * assign89940_body11_e137960)), (((locals.var_t1_dn2 * assign89940_body11_e137960) - (locals.var_t1 * locals.var_t2_dn2)) / (assign89940_body11_e137960 * assign89940_body11_e137960)), (((locals.var_t1_dn4 * assign89940_body11_e137960) - (locals.var_t1 * locals.var_t2_dn4)) / (assign89940_body11_e137960 * assign89940_body11_e137960)), (((locals.var_t1_dn5 * assign89940_body11_e137960) - (locals.var_t1 * locals.var_t2_dn5)) / (assign89940_body11_e137960 * assign89940_body11_e137960)), (((locals.var_t1_dn6 * assign89940_body11_e137960) - (locals.var_t1 * locals.var_t2_dn6)) / (assign89940_body11_e137960 * assign89940_body11_e137960)), (((locals.var_t1_dn7 * assign89940_body11_e137960) - (locals.var_t1 * locals.var_t2_dn7)) / (assign89940_body11_e137960 * assign89940_body11_e137960)), (((locals.var_t1_dn8 * assign89940_body11_e137960) - (locals.var_t1 * locals.var_t2_dn8)) / (assign89940_body11_e137960 * assign89940_body11_e137960)), (((locals.var_t1_dn9 * assign89940_body11_e137960) - (locals.var_t1 * locals.var_t2_dn9)) / (assign89940_body11_e137960 * assign89940_body11_e137960)), (((locals.var_t1_dn10 * assign89940_body11_e137960) - (locals.var_t1 * locals.var_t2_dn10)) / (assign89940_body11_e137960 * assign89940_body11_e137960)), (((locals.var_t1_dn11 * assign89940_body11_e137960) - (locals.var_t1 * locals.var_t2_dn11)) / (assign89940_body11_e137960 * assign89940_body11_e137960)), (((locals.var_t1_dn14 * assign89940_body11_e137960) - (locals.var_t1 * locals.var_t2_dn14)) / (assign89940_body11_e137960 * assign89940_body11_e137960)),)
    } else {
        (locals.var_phi_b_dpss, locals.var_phi_b_dpss_dn0, locals.var_phi_b_dpss_dn2, locals.var_phi_b_dpss_dn4, locals.var_phi_b_dpss_dn5, locals.var_phi_b_dpss_dn6, locals.var_phi_b_dpss_dn7, locals.var_phi_b_dpss_dn8, locals.var_phi_b_dpss_dn9, locals.var_phi_b_dpss_dn10, locals.var_phi_b_dpss_dn11, locals.var_phi_b_dpss_dn14,)
    }
};
            locals.var_phi_b_dpss = assign89940_body11_e137963;
            locals.var_phi_b_dpss_dn0 = assign89940_body11_e137963_d_n0;
            locals.var_phi_b_dpss_dn2 = assign89940_body11_e137963_d_n2;
            locals.var_phi_b_dpss_dn4 = assign89940_body11_e137963_d_n4;
            locals.var_phi_b_dpss_dn5 = assign89940_body11_e137963_d_n5;
            locals.var_phi_b_dpss_dn6 = assign89940_body11_e137963_d_n6;
            locals.var_phi_b_dpss_dn7 = assign89940_body11_e137963_d_n7;
            locals.var_phi_b_dpss_dn8 = assign89940_body11_e137963_d_n8;
            locals.var_phi_b_dpss_dn9 = assign89940_body11_e137963_d_n9;
            locals.var_phi_b_dpss_dn10 = assign89940_body11_e137963_d_n10;
            locals.var_phi_b_dpss_dn11 = assign89940_body11_e137963_d_n11;
            locals.var_phi_b_dpss_dn14 = assign89940_body11_e137963_d_n14;
            locals.var_phi_b_dpss_rv = 0.0;
            let (assign89940_body12_e137974, assign89940_body12_e137974_d_n0, assign89940_body12_e137974_d_n2, assign89940_body12_e137974_d_n4, assign89940_body12_e137974_d_n5, assign89940_body12_e137974_d_n6, assign89940_body12_e137974_d_n7, assign89940_body12_e137974_d_n8, assign89940_body12_e137974_d_n9, assign89940_body12_e137974_d_n10, assign89940_body12_e137974_d_n11, assign89940_body12_e137974_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2087 != 0.0)) && (locals.var_guard2092 == 0.0)) {
        let assign89940_body12_e137972: f64 = (locals.var_ps0ld_vxb - locals.var_dphi_sb);
        (assign89940_body12_e137972, (locals.var_ps0ld_vxb_dn0 - locals.var_dphi_sb_dn0), (locals.var_ps0ld_vxb_dn2 - locals.var_dphi_sb_dn2), (locals.var_ps0ld_vxb_dn4 - locals.var_dphi_sb_dn4), (locals.var_ps0ld_vxb_dn5 - locals.var_dphi_sb_dn5), (locals.var_ps0ld_vxb_dn6 - locals.var_dphi_sb_dn6), (locals.var_ps0ld_vxb_dn7 - locals.var_dphi_sb_dn7), (locals.var_ps0ld_vxb_dn8 - locals.var_dphi_sb_dn8), (locals.var_ps0ld_vxb_dn9 - locals.var_dphi_sb_dn9), (locals.var_ps0ld_vxb_dn10 - locals.var_dphi_sb_dn10), (locals.var_ps0ld_vxb_dn11 - locals.var_dphi_sb_dn11), (locals.var_ps0ld_vxb_dn14 - locals.var_dphi_sb_dn14),)
    } else {
        (locals.var_phi_b, locals.var_phi_b_dn0, locals.var_phi_b_dn2, locals.var_phi_b_dn4, locals.var_phi_b_dn5, locals.var_phi_b_dn6, locals.var_phi_b_dn7, locals.var_phi_b_dn8, locals.var_phi_b_dn9, locals.var_phi_b_dn10, locals.var_phi_b_dn11, locals.var_phi_b_dn14,)
    }
};
            locals.var_phi_b = assign89940_body12_e137974;
            locals.var_phi_b_dn0 = assign89940_body12_e137974_d_n0;
            locals.var_phi_b_dn2 = assign89940_body12_e137974_d_n2;
            locals.var_phi_b_dn4 = assign89940_body12_e137974_d_n4;
            locals.var_phi_b_dn5 = assign89940_body12_e137974_d_n5;
            locals.var_phi_b_dn6 = assign89940_body12_e137974_d_n6;
            locals.var_phi_b_dn7 = assign89940_body12_e137974_d_n7;
            locals.var_phi_b_dn8 = assign89940_body12_e137974_d_n8;
            locals.var_phi_b_dn9 = assign89940_body12_e137974_d_n9;
            locals.var_phi_b_dn10 = assign89940_body12_e137974_d_n10;
            locals.var_phi_b_dn11 = assign89940_body12_e137974_d_n11;
            locals.var_phi_b_dn14 = assign89940_body12_e137974_d_n14;
            locals.var_phi_b_rv = 0.0;
            let (assign89940_body13_e137983, assign89940_body13_e137983_d_n0, assign89940_body13_e137983_d_n2, assign89940_body13_e137983_d_n4, assign89940_body13_e137983_d_n5, assign89940_body13_e137983_d_n6, assign89940_body13_e137983_d_n7, assign89940_body13_e137983_d_n8, assign89940_body13_e137983_d_n9, assign89940_body13_e137983_d_n10, assign89940_body13_e137983_d_n11, assign89940_body13_e137983_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2087 != 0.0)) && (locals.var_guard2092 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phi_b_dpss, locals.var_phi_b_dpss_dn0, locals.var_phi_b_dpss_dn2, locals.var_phi_b_dpss_dn4, locals.var_phi_b_dpss_dn5, locals.var_phi_b_dpss_dn6, locals.var_phi_b_dpss_dn7, locals.var_phi_b_dpss_dn8, locals.var_phi_b_dpss_dn9, locals.var_phi_b_dpss_dn10, locals.var_phi_b_dpss_dn11, locals.var_phi_b_dpss_dn14,)
    }
};
            locals.var_phi_b_dpss = assign89940_body13_e137983;
            locals.var_phi_b_dpss_dn0 = assign89940_body13_e137983_d_n0;
            locals.var_phi_b_dpss_dn2 = assign89940_body13_e137983_d_n2;
            locals.var_phi_b_dpss_dn4 = assign89940_body13_e137983_d_n4;
            locals.var_phi_b_dpss_dn5 = assign89940_body13_e137983_d_n5;
            locals.var_phi_b_dpss_dn6 = assign89940_body13_e137983_d_n6;
            locals.var_phi_b_dpss_dn7 = assign89940_body13_e137983_d_n7;
            locals.var_phi_b_dpss_dn8 = assign89940_body13_e137983_d_n8;
            locals.var_phi_b_dpss_dn9 = assign89940_body13_e137983_d_n9;
            locals.var_phi_b_dpss_dn10 = assign89940_body13_e137983_d_n10;
            locals.var_phi_b_dpss_dn11 = assign89940_body13_e137983_d_n11;
            locals.var_phi_b_dpss_dn14 = assign89940_body13_e137983_d_n14;
            locals.var_phi_b_dpss_rv = 0.0;
            let (assign89940_body14_e137991, assign89940_body14_e137991_d_n0, assign89940_body14_e137991_d_n2, assign89940_body14_e137991_d_n4, assign89940_body14_e137991_d_n5, assign89940_body14_e137991_d_n6, assign89940_body14_e137991_d_n7, assign89940_body14_e137991_d_n8, assign89940_body14_e137991_d_n9, assign89940_body14_e137991_d_n10, assign89940_body14_e137991_d_n11, assign89940_body14_e137991_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2087 != 0.0)) {
        let assign89940_body14_e137989: f64 = (locals.var_beta * locals.var_phi_b);
        (assign89940_body14_e137989, ((locals.var_beta_dn0 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn0)), ((locals.var_beta_dn2 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn2)), ((locals.var_beta_dn4 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn4)), ((locals.var_beta_dn5 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn5)), ((locals.var_beta_dn6 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn6)), ((locals.var_beta_dn7 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn7)), ((locals.var_beta_dn8 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn8)), ((locals.var_beta_dn9 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn9)), ((locals.var_beta_dn10 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn10)), ((locals.var_beta_dn11 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn11)), ((locals.var_beta_dn14 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn14)),)
    } else {
        (locals.var_chib, locals.var_chib_dn0, locals.var_chib_dn2, locals.var_chib_dn4, locals.var_chib_dn5, locals.var_chib_dn6, locals.var_chib_dn7, locals.var_chib_dn8, locals.var_chib_dn9, locals.var_chib_dn10, locals.var_chib_dn11, locals.var_chib_dn14,)
    }
};
            locals.var_chib = assign89940_body14_e137991;
            locals.var_chib_dn0 = assign89940_body14_e137991_d_n0;
            locals.var_chib_dn2 = assign89940_body14_e137991_d_n2;
            locals.var_chib_dn4 = assign89940_body14_e137991_d_n4;
            locals.var_chib_dn5 = assign89940_body14_e137991_d_n5;
            locals.var_chib_dn6 = assign89940_body14_e137991_d_n6;
            locals.var_chib_dn7 = assign89940_body14_e137991_d_n7;
            locals.var_chib_dn8 = assign89940_body14_e137991_d_n8;
            locals.var_chib_dn9 = assign89940_body14_e137991_d_n9;
            locals.var_chib_dn10 = assign89940_body14_e137991_d_n10;
            locals.var_chib_dn11 = assign89940_body14_e137991_d_n11;
            locals.var_chib_dn14 = assign89940_body14_e137991_d_n14;
            locals.var_chib_rv = 0.0;
            let assign89940_body15_e137993: f64 = (locals.var_chi).abs();
            let assign89940_body15_e137995: f64 = if assign89940_body15_e137993 < 1e-6 { 1.0 } else { 0.0 };
            locals.var_guard2093 = assign89940_body15_e137995;
            locals.var_guard2093_rv = 0.0;
            let (assign89940_body17_e138041, assign89940_body17_e138041_d_n0, assign89940_body17_e138041_d_n2, assign89940_body17_e138041_d_n4, assign89940_body17_e138041_d_n5, assign89940_body17_e138041_d_n6, assign89940_body17_e138041_d_n7, assign89940_body17_e138041_d_n8, assign89940_body17_e138041_d_n9, assign89940_body17_e138041_d_n10, assign89940_body17_e138041_d_n11, assign89940_body17_e138041_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2087 != 0.0)) && (locals.var_guard2093 != 0.0)) {
        let assign89940_body17_e138019: f64 = (locals.var_chi * locals.var_chi);
        let assign89940_body17_e138021: f64 = (assign89940_body17_e138019 / 2.0);
        let assign89940_body17_e138025: f64 = (locals.var_chi / 3.0);
        let assign89940_body17_e138029: f64 = (locals.var_chi / 4.0);
        let assign89940_body17_e138033: f64 = (locals.var_chi / 5.0);
        let assign89940_body17_e138034: f64 = (1.0 - assign89940_body17_e138033);
        let assign89940_body17_e138035: f64 = (assign89940_body17_e138029 * assign89940_body17_e138034);
        let assign89940_body17_e138036: f64 = (1.0 - assign89940_body17_e138035);
        let assign89940_body17_e138037: f64 = (assign89940_body17_e138025 * assign89940_body17_e138036);
        let assign89940_body17_e138038: f64 = (1.0 - assign89940_body17_e138037);
        let assign89940_body17_e138039: f64 = (assign89940_body17_e138021 * assign89940_body17_e138038);
        (assign89940_body17_e138039, (((((locals.var_chi_dn0 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn0)) / 2.0) * assign89940_body17_e138038) + (assign89940_body17_e138021 * (-(((locals.var_chi_dn0 / 3.0) * assign89940_body17_e138036) + (assign89940_body17_e138025 * (-(((locals.var_chi_dn0 / 4.0) * assign89940_body17_e138034) + (assign89940_body17_e138029 * (-(locals.var_chi_dn0 / 5.0)))))))))), (((((locals.var_chi_dn2 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn2)) / 2.0) * assign89940_body17_e138038) + (assign89940_body17_e138021 * (-(((locals.var_chi_dn2 / 3.0) * assign89940_body17_e138036) + (assign89940_body17_e138025 * (-(((locals.var_chi_dn2 / 4.0) * assign89940_body17_e138034) + (assign89940_body17_e138029 * (-(locals.var_chi_dn2 / 5.0)))))))))), (((((locals.var_chi_dn4 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn4)) / 2.0) * assign89940_body17_e138038) + (assign89940_body17_e138021 * (-(((locals.var_chi_dn4 / 3.0) * assign89940_body17_e138036) + (assign89940_body17_e138025 * (-(((locals.var_chi_dn4 / 4.0) * assign89940_body17_e138034) + (assign89940_body17_e138029 * (-(locals.var_chi_dn4 / 5.0)))))))))), (((((locals.var_chi_dn5 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn5)) / 2.0) * assign89940_body17_e138038) + (assign89940_body17_e138021 * (-(((locals.var_chi_dn5 / 3.0) * assign89940_body17_e138036) + (assign89940_body17_e138025 * (-(((locals.var_chi_dn5 / 4.0) * assign89940_body17_e138034) + (assign89940_body17_e138029 * (-(locals.var_chi_dn5 / 5.0)))))))))), (((((locals.var_chi_dn6 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn6)) / 2.0) * assign89940_body17_e138038) + (assign89940_body17_e138021 * (-(((locals.var_chi_dn6 / 3.0) * assign89940_body17_e138036) + (assign89940_body17_e138025 * (-(((locals.var_chi_dn6 / 4.0) * assign89940_body17_e138034) + (assign89940_body17_e138029 * (-(locals.var_chi_dn6 / 5.0)))))))))), (((((locals.var_chi_dn7 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn7)) / 2.0) * assign89940_body17_e138038) + (assign89940_body17_e138021 * (-(((locals.var_chi_dn7 / 3.0) * assign89940_body17_e138036) + (assign89940_body17_e138025 * (-(((locals.var_chi_dn7 / 4.0) * assign89940_body17_e138034) + (assign89940_body17_e138029 * (-(locals.var_chi_dn7 / 5.0)))))))))), (((((locals.var_chi_dn8 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn8)) / 2.0) * assign89940_body17_e138038) + (assign89940_body17_e138021 * (-(((locals.var_chi_dn8 / 3.0) * assign89940_body17_e138036) + (assign89940_body17_e138025 * (-(((locals.var_chi_dn8 / 4.0) * assign89940_body17_e138034) + (assign89940_body17_e138029 * (-(locals.var_chi_dn8 / 5.0)))))))))), (((((locals.var_chi_dn9 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn9)) / 2.0) * assign89940_body17_e138038) + (assign89940_body17_e138021 * (-(((locals.var_chi_dn9 / 3.0) * assign89940_body17_e138036) + (assign89940_body17_e138025 * (-(((locals.var_chi_dn9 / 4.0) * assign89940_body17_e138034) + (assign89940_body17_e138029 * (-(locals.var_chi_dn9 / 5.0)))))))))), (((((locals.var_chi_dn10 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn10)) / 2.0) * assign89940_body17_e138038) + (assign89940_body17_e138021 * (-(((locals.var_chi_dn10 / 3.0) * assign89940_body17_e138036) + (assign89940_body17_e138025 * (-(((locals.var_chi_dn10 / 4.0) * assign89940_body17_e138034) + (assign89940_body17_e138029 * (-(locals.var_chi_dn10 / 5.0)))))))))), (((((locals.var_chi_dn11 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn11)) / 2.0) * assign89940_body17_e138038) + (assign89940_body17_e138021 * (-(((locals.var_chi_dn11 / 3.0) * assign89940_body17_e138036) + (assign89940_body17_e138025 * (-(((locals.var_chi_dn11 / 4.0) * assign89940_body17_e138034) + (assign89940_body17_e138029 * (-(locals.var_chi_dn11 / 5.0)))))))))), (((((locals.var_chi_dn14 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn14)) / 2.0) * assign89940_body17_e138038) + (assign89940_body17_e138021 * (-(((locals.var_chi_dn14 / 3.0) * assign89940_body17_e138036) + (assign89940_body17_e138025 * (-(((locals.var_chi_dn14 / 4.0) * assign89940_body17_e138034) + (assign89940_body17_e138029 * (-(locals.var_chi_dn14 / 5.0)))))))))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
            locals.var_t0 = assign89940_body17_e138041;
            locals.var_t0_dn0 = assign89940_body17_e138041_d_n0;
            locals.var_t0_dn2 = assign89940_body17_e138041_d_n2;
            locals.var_t0_dn4 = assign89940_body17_e138041_d_n4;
            locals.var_t0_dn5 = assign89940_body17_e138041_d_n5;
            locals.var_t0_dn6 = assign89940_body17_e138041_d_n6;
            locals.var_t0_dn7 = assign89940_body17_e138041_d_n7;
            locals.var_t0_dn8 = assign89940_body17_e138041_d_n8;
            locals.var_t0_dn9 = assign89940_body17_e138041_d_n9;
            locals.var_t0_dn10 = assign89940_body17_e138041_d_n10;
            locals.var_t0_dn11 = assign89940_body17_e138041_d_n11;
            locals.var_t0_dn14 = assign89940_body17_e138041_d_n14;
            locals.var_t0_rv = 0.0;
            let (assign89940_body18_e138067, assign89940_body18_e138067_d_n0, assign89940_body18_e138067_d_n2, assign89940_body18_e138067_d_n4, assign89940_body18_e138067_d_n5, assign89940_body18_e138067_d_n6, assign89940_body18_e138067_d_n7, assign89940_body18_e138067_d_n8, assign89940_body18_e138067_d_n9, assign89940_body18_e138067_d_n10, assign89940_body18_e138067_d_n11, assign89940_body18_e138067_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2087 != 0.0)) && (locals.var_guard2093 != 0.0)) {
        let assign89940_body18_e138051: f64 = (locals.var_chi / 2.0);
        let assign89940_body18_e138055: f64 = (locals.var_chi / 3.0);
        let assign89940_body18_e138059: f64 = (locals.var_chi / 4.0);
        let assign89940_body18_e138060: f64 = (1.0 - assign89940_body18_e138059);
        let assign89940_body18_e138061: f64 = (assign89940_body18_e138055 * assign89940_body18_e138060);
        let assign89940_body18_e138062: f64 = (1.0 - assign89940_body18_e138061);
        let assign89940_body18_e138063: f64 = (assign89940_body18_e138051 * assign89940_body18_e138062);
        let assign89940_body18_e138064: f64 = (1.0 - assign89940_body18_e138063);
        let assign89940_body18_e138065: f64 = (locals.var_chi * assign89940_body18_e138064);
        (assign89940_body18_e138065, ((locals.var_chi_dn0 * assign89940_body18_e138064) + (locals.var_chi * (-(((locals.var_chi_dn0 / 2.0) * assign89940_body18_e138062) + (assign89940_body18_e138051 * (-(((locals.var_chi_dn0 / 3.0) * assign89940_body18_e138060) + (assign89940_body18_e138055 * (-(locals.var_chi_dn0 / 4.0)))))))))), ((locals.var_chi_dn2 * assign89940_body18_e138064) + (locals.var_chi * (-(((locals.var_chi_dn2 / 2.0) * assign89940_body18_e138062) + (assign89940_body18_e138051 * (-(((locals.var_chi_dn2 / 3.0) * assign89940_body18_e138060) + (assign89940_body18_e138055 * (-(locals.var_chi_dn2 / 4.0)))))))))), ((locals.var_chi_dn4 * assign89940_body18_e138064) + (locals.var_chi * (-(((locals.var_chi_dn4 / 2.0) * assign89940_body18_e138062) + (assign89940_body18_e138051 * (-(((locals.var_chi_dn4 / 3.0) * assign89940_body18_e138060) + (assign89940_body18_e138055 * (-(locals.var_chi_dn4 / 4.0)))))))))), ((locals.var_chi_dn5 * assign89940_body18_e138064) + (locals.var_chi * (-(((locals.var_chi_dn5 / 2.0) * assign89940_body18_e138062) + (assign89940_body18_e138051 * (-(((locals.var_chi_dn5 / 3.0) * assign89940_body18_e138060) + (assign89940_body18_e138055 * (-(locals.var_chi_dn5 / 4.0)))))))))), ((locals.var_chi_dn6 * assign89940_body18_e138064) + (locals.var_chi * (-(((locals.var_chi_dn6 / 2.0) * assign89940_body18_e138062) + (assign89940_body18_e138051 * (-(((locals.var_chi_dn6 / 3.0) * assign89940_body18_e138060) + (assign89940_body18_e138055 * (-(locals.var_chi_dn6 / 4.0)))))))))), ((locals.var_chi_dn7 * assign89940_body18_e138064) + (locals.var_chi * (-(((locals.var_chi_dn7 / 2.0) * assign89940_body18_e138062) + (assign89940_body18_e138051 * (-(((locals.var_chi_dn7 / 3.0) * assign89940_body18_e138060) + (assign89940_body18_e138055 * (-(locals.var_chi_dn7 / 4.0)))))))))), ((locals.var_chi_dn8 * assign89940_body18_e138064) + (locals.var_chi * (-(((locals.var_chi_dn8 / 2.0) * assign89940_body18_e138062) + (assign89940_body18_e138051 * (-(((locals.var_chi_dn8 / 3.0) * assign89940_body18_e138060) + (assign89940_body18_e138055 * (-(locals.var_chi_dn8 / 4.0)))))))))), ((locals.var_chi_dn9 * assign89940_body18_e138064) + (locals.var_chi * (-(((locals.var_chi_dn9 / 2.0) * assign89940_body18_e138062) + (assign89940_body18_e138051 * (-(((locals.var_chi_dn9 / 3.0) * assign89940_body18_e138060) + (assign89940_body18_e138055 * (-(locals.var_chi_dn9 / 4.0)))))))))), ((locals.var_chi_dn10 * assign89940_body18_e138064) + (locals.var_chi * (-(((locals.var_chi_dn10 / 2.0) * assign89940_body18_e138062) + (assign89940_body18_e138051 * (-(((locals.var_chi_dn10 / 3.0) * assign89940_body18_e138060) + (assign89940_body18_e138055 * (-(locals.var_chi_dn10 / 4.0)))))))))), ((locals.var_chi_dn11 * assign89940_body18_e138064) + (locals.var_chi * (-(((locals.var_chi_dn11 / 2.0) * assign89940_body18_e138062) + (assign89940_body18_e138051 * (-(((locals.var_chi_dn11 / 3.0) * assign89940_body18_e138060) + (assign89940_body18_e138055 * (-(locals.var_chi_dn11 / 4.0)))))))))), ((locals.var_chi_dn14 * assign89940_body18_e138064) + (locals.var_chi * (-(((locals.var_chi_dn14 / 2.0) * assign89940_body18_e138062) + (assign89940_body18_e138051 * (-(((locals.var_chi_dn14 / 3.0) * assign89940_body18_e138060) + (assign89940_body18_e138055 * (-(locals.var_chi_dn14 / 4.0)))))))))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
            locals.var_t1 = assign89940_body18_e138067;
            locals.var_t1_dn0 = assign89940_body18_e138067_d_n0;
            locals.var_t1_dn2 = assign89940_body18_e138067_d_n2;
            locals.var_t1_dn4 = assign89940_body18_e138067_d_n4;
            locals.var_t1_dn5 = assign89940_body18_e138067_d_n5;
            locals.var_t1_dn6 = assign89940_body18_e138067_d_n6;
            locals.var_t1_dn7 = assign89940_body18_e138067_d_n7;
            locals.var_t1_dn8 = assign89940_body18_e138067_d_n8;
            locals.var_t1_dn9 = assign89940_body18_e138067_d_n9;
            locals.var_t1_dn10 = assign89940_body18_e138067_d_n10;
            locals.var_t1_dn11 = assign89940_body18_e138067_d_n11;
            locals.var_t1_dn14 = assign89940_body18_e138067_d_n14;
            locals.var_t1_rv = 0.0;
            let (assign89940_body19_e138097, assign89940_body19_e138097_d_n0, assign89940_body19_e138097_d_n2, assign89940_body19_e138097_d_n4, assign89940_body19_e138097_d_n5, assign89940_body19_e138097_d_n6, assign89940_body19_e138097_d_n7, assign89940_body19_e138097_d_n8, assign89940_body19_e138097_d_n9, assign89940_body19_e138097_d_n10, assign89940_body19_e138097_d_n11, assign89940_body19_e138097_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2087 != 0.0)) && (locals.var_guard2093 != 0.0)) {
        let assign89940_body19_e138075: f64 = (locals.var_chib * locals.var_chib);
        let assign89940_body19_e138077: f64 = (assign89940_body19_e138075 / 2.0);
        let assign89940_body19_e138081: f64 = (locals.var_chib / 3.0);
        let assign89940_body19_e138085: f64 = (locals.var_chib / 4.0);
        let assign89940_body19_e138089: f64 = (locals.var_chib / 5.0);
        let assign89940_body19_e138090: f64 = (1.0 - assign89940_body19_e138089);
        let assign89940_body19_e138091: f64 = (assign89940_body19_e138085 * assign89940_body19_e138090);
        let assign89940_body19_e138092: f64 = (1.0 - assign89940_body19_e138091);
        let assign89940_body19_e138093: f64 = (assign89940_body19_e138081 * assign89940_body19_e138092);
        let assign89940_body19_e138094: f64 = (1.0 - assign89940_body19_e138093);
        let assign89940_body19_e138095: f64 = (assign89940_body19_e138077 * assign89940_body19_e138094);
        (assign89940_body19_e138095, (((((locals.var_chib_dn0 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn0)) / 2.0) * assign89940_body19_e138094) + (assign89940_body19_e138077 * (-(((locals.var_chib_dn0 / 3.0) * assign89940_body19_e138092) + (assign89940_body19_e138081 * (-(((locals.var_chib_dn0 / 4.0) * assign89940_body19_e138090) + (assign89940_body19_e138085 * (-(locals.var_chib_dn0 / 5.0)))))))))), (((((locals.var_chib_dn2 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn2)) / 2.0) * assign89940_body19_e138094) + (assign89940_body19_e138077 * (-(((locals.var_chib_dn2 / 3.0) * assign89940_body19_e138092) + (assign89940_body19_e138081 * (-(((locals.var_chib_dn2 / 4.0) * assign89940_body19_e138090) + (assign89940_body19_e138085 * (-(locals.var_chib_dn2 / 5.0)))))))))), (((((locals.var_chib_dn4 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn4)) / 2.0) * assign89940_body19_e138094) + (assign89940_body19_e138077 * (-(((locals.var_chib_dn4 / 3.0) * assign89940_body19_e138092) + (assign89940_body19_e138081 * (-(((locals.var_chib_dn4 / 4.0) * assign89940_body19_e138090) + (assign89940_body19_e138085 * (-(locals.var_chib_dn4 / 5.0)))))))))), (((((locals.var_chib_dn5 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn5)) / 2.0) * assign89940_body19_e138094) + (assign89940_body19_e138077 * (-(((locals.var_chib_dn5 / 3.0) * assign89940_body19_e138092) + (assign89940_body19_e138081 * (-(((locals.var_chib_dn5 / 4.0) * assign89940_body19_e138090) + (assign89940_body19_e138085 * (-(locals.var_chib_dn5 / 5.0)))))))))), (((((locals.var_chib_dn6 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn6)) / 2.0) * assign89940_body19_e138094) + (assign89940_body19_e138077 * (-(((locals.var_chib_dn6 / 3.0) * assign89940_body19_e138092) + (assign89940_body19_e138081 * (-(((locals.var_chib_dn6 / 4.0) * assign89940_body19_e138090) + (assign89940_body19_e138085 * (-(locals.var_chib_dn6 / 5.0)))))))))), (((((locals.var_chib_dn7 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn7)) / 2.0) * assign89940_body19_e138094) + (assign89940_body19_e138077 * (-(((locals.var_chib_dn7 / 3.0) * assign89940_body19_e138092) + (assign89940_body19_e138081 * (-(((locals.var_chib_dn7 / 4.0) * assign89940_body19_e138090) + (assign89940_body19_e138085 * (-(locals.var_chib_dn7 / 5.0)))))))))), (((((locals.var_chib_dn8 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn8)) / 2.0) * assign89940_body19_e138094) + (assign89940_body19_e138077 * (-(((locals.var_chib_dn8 / 3.0) * assign89940_body19_e138092) + (assign89940_body19_e138081 * (-(((locals.var_chib_dn8 / 4.0) * assign89940_body19_e138090) + (assign89940_body19_e138085 * (-(locals.var_chib_dn8 / 5.0)))))))))), (((((locals.var_chib_dn9 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn9)) / 2.0) * assign89940_body19_e138094) + (assign89940_body19_e138077 * (-(((locals.var_chib_dn9 / 3.0) * assign89940_body19_e138092) + (assign89940_body19_e138081 * (-(((locals.var_chib_dn9 / 4.0) * assign89940_body19_e138090) + (assign89940_body19_e138085 * (-(locals.var_chib_dn9 / 5.0)))))))))), (((((locals.var_chib_dn10 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn10)) / 2.0) * assign89940_body19_e138094) + (assign89940_body19_e138077 * (-(((locals.var_chib_dn10 / 3.0) * assign89940_body19_e138092) + (assign89940_body19_e138081 * (-(((locals.var_chib_dn10 / 4.0) * assign89940_body19_e138090) + (assign89940_body19_e138085 * (-(locals.var_chib_dn10 / 5.0)))))))))), (((((locals.var_chib_dn11 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn11)) / 2.0) * assign89940_body19_e138094) + (assign89940_body19_e138077 * (-(((locals.var_chib_dn11 / 3.0) * assign89940_body19_e138092) + (assign89940_body19_e138081 * (-(((locals.var_chib_dn11 / 4.0) * assign89940_body19_e138090) + (assign89940_body19_e138085 * (-(locals.var_chib_dn11 / 5.0)))))))))), (((((locals.var_chib_dn14 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn14)) / 2.0) * assign89940_body19_e138094) + (assign89940_body19_e138077 * (-(((locals.var_chib_dn14 / 3.0) * assign89940_body19_e138092) + (assign89940_body19_e138081 * (-(((locals.var_chib_dn14 / 4.0) * assign89940_body19_e138090) + (assign89940_body19_e138085 * (-(locals.var_chib_dn14 / 5.0)))))))))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
            locals.var_t2 = assign89940_body19_e138097;
            locals.var_t2_dn0 = assign89940_body19_e138097_d_n0;
            locals.var_t2_dn2 = assign89940_body19_e138097_d_n2;
            locals.var_t2_dn4 = assign89940_body19_e138097_d_n4;
            locals.var_t2_dn5 = assign89940_body19_e138097_d_n5;
            locals.var_t2_dn6 = assign89940_body19_e138097_d_n6;
            locals.var_t2_dn7 = assign89940_body19_e138097_d_n7;
            locals.var_t2_dn8 = assign89940_body19_e138097_d_n8;
            locals.var_t2_dn9 = assign89940_body19_e138097_d_n9;
            locals.var_t2_dn10 = assign89940_body19_e138097_d_n10;
            locals.var_t2_dn11 = assign89940_body19_e138097_d_n11;
            locals.var_t2_dn14 = assign89940_body19_e138097_d_n14;
            locals.var_t2_rv = 0.0;
            let (assign89940_body20_e138123, assign89940_body20_e138123_d_n0, assign89940_body20_e138123_d_n2, assign89940_body20_e138123_d_n4, assign89940_body20_e138123_d_n5, assign89940_body20_e138123_d_n6, assign89940_body20_e138123_d_n7, assign89940_body20_e138123_d_n8, assign89940_body20_e138123_d_n9, assign89940_body20_e138123_d_n10, assign89940_body20_e138123_d_n11, assign89940_body20_e138123_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2087 != 0.0)) && (locals.var_guard2093 != 0.0)) {
        let assign89940_body20_e138107: f64 = (locals.var_chib / 2.0);
        let assign89940_body20_e138111: f64 = (locals.var_chib / 3.0);
        let assign89940_body20_e138115: f64 = (locals.var_chib / 4.0);
        let assign89940_body20_e138116: f64 = (1.0 - assign89940_body20_e138115);
        let assign89940_body20_e138117: f64 = (assign89940_body20_e138111 * assign89940_body20_e138116);
        let assign89940_body20_e138118: f64 = (1.0 - assign89940_body20_e138117);
        let assign89940_body20_e138119: f64 = (assign89940_body20_e138107 * assign89940_body20_e138118);
        let assign89940_body20_e138120: f64 = (1.0 - assign89940_body20_e138119);
        let assign89940_body20_e138121: f64 = (locals.var_chib * assign89940_body20_e138120);
        (assign89940_body20_e138121, ((locals.var_chib_dn0 * assign89940_body20_e138120) + (locals.var_chib * (-(((locals.var_chib_dn0 / 2.0) * assign89940_body20_e138118) + (assign89940_body20_e138107 * (-(((locals.var_chib_dn0 / 3.0) * assign89940_body20_e138116) + (assign89940_body20_e138111 * (-(locals.var_chib_dn0 / 4.0)))))))))), ((locals.var_chib_dn2 * assign89940_body20_e138120) + (locals.var_chib * (-(((locals.var_chib_dn2 / 2.0) * assign89940_body20_e138118) + (assign89940_body20_e138107 * (-(((locals.var_chib_dn2 / 3.0) * assign89940_body20_e138116) + (assign89940_body20_e138111 * (-(locals.var_chib_dn2 / 4.0)))))))))), ((locals.var_chib_dn4 * assign89940_body20_e138120) + (locals.var_chib * (-(((locals.var_chib_dn4 / 2.0) * assign89940_body20_e138118) + (assign89940_body20_e138107 * (-(((locals.var_chib_dn4 / 3.0) * assign89940_body20_e138116) + (assign89940_body20_e138111 * (-(locals.var_chib_dn4 / 4.0)))))))))), ((locals.var_chib_dn5 * assign89940_body20_e138120) + (locals.var_chib * (-(((locals.var_chib_dn5 / 2.0) * assign89940_body20_e138118) + (assign89940_body20_e138107 * (-(((locals.var_chib_dn5 / 3.0) * assign89940_body20_e138116) + (assign89940_body20_e138111 * (-(locals.var_chib_dn5 / 4.0)))))))))), ((locals.var_chib_dn6 * assign89940_body20_e138120) + (locals.var_chib * (-(((locals.var_chib_dn6 / 2.0) * assign89940_body20_e138118) + (assign89940_body20_e138107 * (-(((locals.var_chib_dn6 / 3.0) * assign89940_body20_e138116) + (assign89940_body20_e138111 * (-(locals.var_chib_dn6 / 4.0)))))))))), ((locals.var_chib_dn7 * assign89940_body20_e138120) + (locals.var_chib * (-(((locals.var_chib_dn7 / 2.0) * assign89940_body20_e138118) + (assign89940_body20_e138107 * (-(((locals.var_chib_dn7 / 3.0) * assign89940_body20_e138116) + (assign89940_body20_e138111 * (-(locals.var_chib_dn7 / 4.0)))))))))), ((locals.var_chib_dn8 * assign89940_body20_e138120) + (locals.var_chib * (-(((locals.var_chib_dn8 / 2.0) * assign89940_body20_e138118) + (assign89940_body20_e138107 * (-(((locals.var_chib_dn8 / 3.0) * assign89940_body20_e138116) + (assign89940_body20_e138111 * (-(locals.var_chib_dn8 / 4.0)))))))))), ((locals.var_chib_dn9 * assign89940_body20_e138120) + (locals.var_chib * (-(((locals.var_chib_dn9 / 2.0) * assign89940_body20_e138118) + (assign89940_body20_e138107 * (-(((locals.var_chib_dn9 / 3.0) * assign89940_body20_e138116) + (assign89940_body20_e138111 * (-(locals.var_chib_dn9 / 4.0)))))))))), ((locals.var_chib_dn10 * assign89940_body20_e138120) + (locals.var_chib * (-(((locals.var_chib_dn10 / 2.0) * assign89940_body20_e138118) + (assign89940_body20_e138107 * (-(((locals.var_chib_dn10 / 3.0) * assign89940_body20_e138116) + (assign89940_body20_e138111 * (-(locals.var_chib_dn10 / 4.0)))))))))), ((locals.var_chib_dn11 * assign89940_body20_e138120) + (locals.var_chib * (-(((locals.var_chib_dn11 / 2.0) * assign89940_body20_e138118) + (assign89940_body20_e138107 * (-(((locals.var_chib_dn11 / 3.0) * assign89940_body20_e138116) + (assign89940_body20_e138111 * (-(locals.var_chib_dn11 / 4.0)))))))))), ((locals.var_chib_dn14 * assign89940_body20_e138120) + (locals.var_chib * (-(((locals.var_chib_dn14 / 2.0) * assign89940_body20_e138118) + (assign89940_body20_e138107 * (-(((locals.var_chib_dn14 / 3.0) * assign89940_body20_e138116) + (assign89940_body20_e138111 * (-(locals.var_chib_dn14 / 4.0)))))))))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
            locals.var_t3 = assign89940_body20_e138123;
            locals.var_t3_dn0 = assign89940_body20_e138123_d_n0;
            locals.var_t3_dn2 = assign89940_body20_e138123_d_n2;
            locals.var_t3_dn4 = assign89940_body20_e138123_d_n4;
            locals.var_t3_dn5 = assign89940_body20_e138123_d_n5;
            locals.var_t3_dn6 = assign89940_body20_e138123_d_n6;
            locals.var_t3_dn7 = assign89940_body20_e138123_d_n7;
            locals.var_t3_dn8 = assign89940_body20_e138123_d_n8;
            locals.var_t3_dn9 = assign89940_body20_e138123_d_n9;
            locals.var_t3_dn10 = assign89940_body20_e138123_d_n10;
            locals.var_t3_dn11 = assign89940_body20_e138123_d_n11;
            locals.var_t3_dn14 = assign89940_body20_e138123_d_n14;
            locals.var_t3_rv = 0.0;
            let (assign89940_body21_e138133, assign89940_body21_e138133_d_n0, assign89940_body21_e138133_d_n2, assign89940_body21_e138133_d_n4, assign89940_body21_e138133_d_n5, assign89940_body21_e138133_d_n6, assign89940_body21_e138133_d_n7, assign89940_body21_e138133_d_n8, assign89940_body21_e138133_d_n9, assign89940_body21_e138133_d_n10, assign89940_body21_e138133_d_n11, assign89940_body21_e138133_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2087 != 0.0)) && (locals.var_guard2093 != 0.0)) {
        let assign89940_body21_e138131: f64 = (locals.var_t0 - locals.var_t2);
        (assign89940_body21_e138131, (locals.var_t0_dn0 - locals.var_t2_dn0), (locals.var_t0_dn2 - locals.var_t2_dn2), (locals.var_t0_dn4 - locals.var_t2_dn4), (locals.var_t0_dn5 - locals.var_t2_dn5), (locals.var_t0_dn6 - locals.var_t2_dn6), (locals.var_t0_dn7 - locals.var_t2_dn7), (locals.var_t0_dn8 - locals.var_t2_dn8), (locals.var_t0_dn9 - locals.var_t2_dn9), (locals.var_t0_dn10 - locals.var_t2_dn10), (locals.var_t0_dn11 - locals.var_t2_dn11), (locals.var_t0_dn14 - locals.var_t2_dn14),)
    } else {
        (locals.var_fbsq__blk2014, locals.var_fbsq__blk2014_dn0, locals.var_fbsq__blk2014_dn2, locals.var_fbsq__blk2014_dn4, locals.var_fbsq__blk2014_dn5, locals.var_fbsq__blk2014_dn6, locals.var_fbsq__blk2014_dn7, locals.var_fbsq__blk2014_dn8, locals.var_fbsq__blk2014_dn9, locals.var_fbsq__blk2014_dn10, locals.var_fbsq__blk2014_dn11, locals.var_fbsq__blk2014_dn14,)
    }
};
            locals.var_fbsq__blk2014 = assign89940_body21_e138133;
            locals.var_fbsq__blk2014_dn0 = assign89940_body21_e138133_d_n0;
            locals.var_fbsq__blk2014_dn2 = assign89940_body21_e138133_d_n2;
            locals.var_fbsq__blk2014_dn4 = assign89940_body21_e138133_d_n4;
            locals.var_fbsq__blk2014_dn5 = assign89940_body21_e138133_d_n5;
            locals.var_fbsq__blk2014_dn6 = assign89940_body21_e138133_d_n6;
            locals.var_fbsq__blk2014_dn7 = assign89940_body21_e138133_d_n7;
            locals.var_fbsq__blk2014_dn8 = assign89940_body21_e138133_d_n8;
            locals.var_fbsq__blk2014_dn9 = assign89940_body21_e138133_d_n9;
            locals.var_fbsq__blk2014_dn10 = assign89940_body21_e138133_d_n10;
            locals.var_fbsq__blk2014_dn11 = assign89940_body21_e138133_d_n11;
            locals.var_fbsq__blk2014_dn14 = assign89940_body21_e138133_d_n14;
            locals.var_fbsq__blk2014_rv = 0.0;
            let (assign89940_body22_e138147, assign89940_body22_e138147_d_n0, assign89940_body22_e138147_d_n2, assign89940_body22_e138147_d_n4, assign89940_body22_e138147_d_n5, assign89940_body22_e138147_d_n6, assign89940_body22_e138147_d_n7, assign89940_body22_e138147_d_n8, assign89940_body22_e138147_d_n9, assign89940_body22_e138147_d_n10, assign89940_body22_e138147_d_n11, assign89940_body22_e138147_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2087 != 0.0)) && (locals.var_guard2093 != 0.0)) {
        let assign89940_body22_e138143: f64 = (locals.var_phi_b_dpss * locals.var_t3);
        let assign89940_body22_e138144: f64 = (locals.var_t1 - assign89940_body22_e138143);
        let assign89940_body22_e138145: f64 = (locals.var_beta * assign89940_body22_e138144);
        (assign89940_body22_e138145, ((locals.var_beta_dn0 * assign89940_body22_e138144) + (locals.var_beta * (locals.var_t1_dn0 - ((locals.var_phi_b_dpss_dn0 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn0))))), ((locals.var_beta_dn2 * assign89940_body22_e138144) + (locals.var_beta * (locals.var_t1_dn2 - ((locals.var_phi_b_dpss_dn2 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn2))))), ((locals.var_beta_dn4 * assign89940_body22_e138144) + (locals.var_beta * (locals.var_t1_dn4 - ((locals.var_phi_b_dpss_dn4 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn4))))), ((locals.var_beta_dn5 * assign89940_body22_e138144) + (locals.var_beta * (locals.var_t1_dn5 - ((locals.var_phi_b_dpss_dn5 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn5))))), ((locals.var_beta_dn6 * assign89940_body22_e138144) + (locals.var_beta * (locals.var_t1_dn6 - ((locals.var_phi_b_dpss_dn6 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn6))))), ((locals.var_beta_dn7 * assign89940_body22_e138144) + (locals.var_beta * (locals.var_t1_dn7 - ((locals.var_phi_b_dpss_dn7 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn7))))), ((locals.var_beta_dn8 * assign89940_body22_e138144) + (locals.var_beta * (locals.var_t1_dn8 - ((locals.var_phi_b_dpss_dn8 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn8))))), ((locals.var_beta_dn9 * assign89940_body22_e138144) + (locals.var_beta * (locals.var_t1_dn9 - ((locals.var_phi_b_dpss_dn9 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn9))))), ((locals.var_beta_dn10 * assign89940_body22_e138144) + (locals.var_beta * (locals.var_t1_dn10 - ((locals.var_phi_b_dpss_dn10 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn10))))), ((locals.var_beta_dn11 * assign89940_body22_e138144) + (locals.var_beta * (locals.var_t1_dn11 - ((locals.var_phi_b_dpss_dn11 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn11))))), ((locals.var_beta_dn14 * assign89940_body22_e138144) + (locals.var_beta * (locals.var_t1_dn14 - ((locals.var_phi_b_dpss_dn14 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn14))))),)
    } else {
        (locals.var_fbsq_dpss__blk2015, locals.var_fbsq_dpss__blk2015_dn0, locals.var_fbsq_dpss__blk2015_dn2, locals.var_fbsq_dpss__blk2015_dn4, locals.var_fbsq_dpss__blk2015_dn5, locals.var_fbsq_dpss__blk2015_dn6, locals.var_fbsq_dpss__blk2015_dn7, locals.var_fbsq_dpss__blk2015_dn8, locals.var_fbsq_dpss__blk2015_dn9, locals.var_fbsq_dpss__blk2015_dn10, locals.var_fbsq_dpss__blk2015_dn11, locals.var_fbsq_dpss__blk2015_dn14,)
    }
};
            locals.var_fbsq_dpss__blk2015 = assign89940_body22_e138147;
            locals.var_fbsq_dpss__blk2015_dn0 = assign89940_body22_e138147_d_n0;
            locals.var_fbsq_dpss__blk2015_dn2 = assign89940_body22_e138147_d_n2;
            locals.var_fbsq_dpss__blk2015_dn4 = assign89940_body22_e138147_d_n4;
            locals.var_fbsq_dpss__blk2015_dn5 = assign89940_body22_e138147_d_n5;
            locals.var_fbsq_dpss__blk2015_dn6 = assign89940_body22_e138147_d_n6;
            locals.var_fbsq_dpss__blk2015_dn7 = assign89940_body22_e138147_d_n7;
            locals.var_fbsq_dpss__blk2015_dn8 = assign89940_body22_e138147_d_n8;
            locals.var_fbsq_dpss__blk2015_dn9 = assign89940_body22_e138147_d_n9;
            locals.var_fbsq_dpss__blk2015_dn10 = assign89940_body22_e138147_d_n10;
            locals.var_fbsq_dpss__blk2015_dn11 = assign89940_body22_e138147_d_n11;
            locals.var_fbsq_dpss__blk2015_dn14 = assign89940_body22_e138147_d_n14;
            locals.var_fbsq_dpss__blk2015_rv = 0.0;
            let (assign89940_body24_e138175, assign89940_body24_e138175_d_n0, assign89940_body24_e138175_d_n2, assign89940_body24_e138175_d_n4, assign89940_body24_e138175_d_n5, assign89940_body24_e138175_d_n6, assign89940_body24_e138175_d_n7, assign89940_body24_e138175_d_n8, assign89940_body24_e138175_d_n9, assign89940_body24_e138175_d_n10, assign89940_body24_e138175_d_n11, assign89940_body24_e138175_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2087 != 0.0)) && (locals.var_guard2093 == 0.0)) {
        let assign89940_body24_e138172: f64 = (-locals.var_chi);
        let assign89940_body24_e138173: f64 = (assign89940_body24_e138172).exp();
        (assign89940_body24_e138173, (assign89940_body24_e138173 * (-locals.var_chi_dn0)), (assign89940_body24_e138173 * (-locals.var_chi_dn2)), (assign89940_body24_e138173 * (-locals.var_chi_dn4)), (assign89940_body24_e138173 * (-locals.var_chi_dn5)), (assign89940_body24_e138173 * (-locals.var_chi_dn6)), (assign89940_body24_e138173 * (-locals.var_chi_dn7)), (assign89940_body24_e138173 * (-locals.var_chi_dn8)), (assign89940_body24_e138173 * (-locals.var_chi_dn9)), (assign89940_body24_e138173 * (-locals.var_chi_dn10)), (assign89940_body24_e138173 * (-locals.var_chi_dn11)), (assign89940_body24_e138173 * (-locals.var_chi_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
            locals.var_t0 = assign89940_body24_e138175;
            locals.var_t0_dn0 = assign89940_body24_e138175_d_n0;
            locals.var_t0_dn2 = assign89940_body24_e138175_d_n2;
            locals.var_t0_dn4 = assign89940_body24_e138175_d_n4;
            locals.var_t0_dn5 = assign89940_body24_e138175_d_n5;
            locals.var_t0_dn6 = assign89940_body24_e138175_d_n6;
            locals.var_t0_dn7 = assign89940_body24_e138175_d_n7;
            locals.var_t0_dn8 = assign89940_body24_e138175_d_n8;
            locals.var_t0_dn9 = assign89940_body24_e138175_d_n9;
            locals.var_t0_dn10 = assign89940_body24_e138175_d_n10;
            locals.var_t0_dn11 = assign89940_body24_e138175_d_n11;
            locals.var_t0_dn14 = assign89940_body24_e138175_d_n14;
            locals.var_t0_rv = 0.0;
            let (assign89940_body25_e138186, assign89940_body25_e138186_d_n0, assign89940_body25_e138186_d_n2, assign89940_body25_e138186_d_n4, assign89940_body25_e138186_d_n5, assign89940_body25_e138186_d_n6, assign89940_body25_e138186_d_n7, assign89940_body25_e138186_d_n8, assign89940_body25_e138186_d_n9, assign89940_body25_e138186_d_n10, assign89940_body25_e138186_d_n11, assign89940_body25_e138186_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2087 != 0.0)) && (locals.var_guard2093 == 0.0)) {
        let assign89940_body25_e138183: f64 = (-locals.var_chib);
        let assign89940_body25_e138184: f64 = (assign89940_body25_e138183).exp();
        (assign89940_body25_e138184, (assign89940_body25_e138184 * (-locals.var_chib_dn0)), (assign89940_body25_e138184 * (-locals.var_chib_dn2)), (assign89940_body25_e138184 * (-locals.var_chib_dn4)), (assign89940_body25_e138184 * (-locals.var_chib_dn5)), (assign89940_body25_e138184 * (-locals.var_chib_dn6)), (assign89940_body25_e138184 * (-locals.var_chib_dn7)), (assign89940_body25_e138184 * (-locals.var_chib_dn8)), (assign89940_body25_e138184 * (-locals.var_chib_dn9)), (assign89940_body25_e138184 * (-locals.var_chib_dn10)), (assign89940_body25_e138184 * (-locals.var_chib_dn11)), (assign89940_body25_e138184 * (-locals.var_chib_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
            locals.var_t1 = assign89940_body25_e138186;
            locals.var_t1_dn0 = assign89940_body25_e138186_d_n0;
            locals.var_t1_dn2 = assign89940_body25_e138186_d_n2;
            locals.var_t1_dn4 = assign89940_body25_e138186_d_n4;
            locals.var_t1_dn5 = assign89940_body25_e138186_d_n5;
            locals.var_t1_dn6 = assign89940_body25_e138186_d_n6;
            locals.var_t1_dn7 = assign89940_body25_e138186_d_n7;
            locals.var_t1_dn8 = assign89940_body25_e138186_d_n8;
            locals.var_t1_dn9 = assign89940_body25_e138186_d_n9;
            locals.var_t1_dn10 = assign89940_body25_e138186_d_n10;
            locals.var_t1_dn11 = assign89940_body25_e138186_d_n11;
            locals.var_t1_dn14 = assign89940_body25_e138186_d_n14;
            locals.var_t1_rv = 0.0;
            let (assign89940_body26_e138201, assign89940_body26_e138201_d_n0, assign89940_body26_e138201_d_n2, assign89940_body26_e138201_d_n4, assign89940_body26_e138201_d_n5, assign89940_body26_e138201_d_n6, assign89940_body26_e138201_d_n7, assign89940_body26_e138201_d_n8, assign89940_body26_e138201_d_n9, assign89940_body26_e138201_d_n10, assign89940_body26_e138201_d_n11, assign89940_body26_e138201_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2087 != 0.0)) && (locals.var_guard2093 == 0.0)) {
        let assign89940_body26_e138195: f64 = (locals.var_chi - locals.var_chib);
        let assign89940_body26_e138198: f64 = (locals.var_t0 - locals.var_t1);
        let assign89940_body26_e138199: f64 = (assign89940_body26_e138195 + assign89940_body26_e138198);
        (assign89940_body26_e138199, ((locals.var_chi_dn0 - locals.var_chib_dn0) + (locals.var_t0_dn0 - locals.var_t1_dn0)), ((locals.var_chi_dn2 - locals.var_chib_dn2) + (locals.var_t0_dn2 - locals.var_t1_dn2)), ((locals.var_chi_dn4 - locals.var_chib_dn4) + (locals.var_t0_dn4 - locals.var_t1_dn4)), ((locals.var_chi_dn5 - locals.var_chib_dn5) + (locals.var_t0_dn5 - locals.var_t1_dn5)), ((locals.var_chi_dn6 - locals.var_chib_dn6) + (locals.var_t0_dn6 - locals.var_t1_dn6)), ((locals.var_chi_dn7 - locals.var_chib_dn7) + (locals.var_t0_dn7 - locals.var_t1_dn7)), ((locals.var_chi_dn8 - locals.var_chib_dn8) + (locals.var_t0_dn8 - locals.var_t1_dn8)), ((locals.var_chi_dn9 - locals.var_chib_dn9) + (locals.var_t0_dn9 - locals.var_t1_dn9)), ((locals.var_chi_dn10 - locals.var_chib_dn10) + (locals.var_t0_dn10 - locals.var_t1_dn10)), ((locals.var_chi_dn11 - locals.var_chib_dn11) + (locals.var_t0_dn11 - locals.var_t1_dn11)), ((locals.var_chi_dn14 - locals.var_chib_dn14) + (locals.var_t0_dn14 - locals.var_t1_dn14)),)
    } else {
        (locals.var_fbsq__blk2014, locals.var_fbsq__blk2014_dn0, locals.var_fbsq__blk2014_dn2, locals.var_fbsq__blk2014_dn4, locals.var_fbsq__blk2014_dn5, locals.var_fbsq__blk2014_dn6, locals.var_fbsq__blk2014_dn7, locals.var_fbsq__blk2014_dn8, locals.var_fbsq__blk2014_dn9, locals.var_fbsq__blk2014_dn10, locals.var_fbsq__blk2014_dn11, locals.var_fbsq__blk2014_dn14,)
    }
};
            locals.var_fbsq__blk2014 = assign89940_body26_e138201;
            locals.var_fbsq__blk2014_dn0 = assign89940_body26_e138201_d_n0;
            locals.var_fbsq__blk2014_dn2 = assign89940_body26_e138201_d_n2;
            locals.var_fbsq__blk2014_dn4 = assign89940_body26_e138201_d_n4;
            locals.var_fbsq__blk2014_dn5 = assign89940_body26_e138201_d_n5;
            locals.var_fbsq__blk2014_dn6 = assign89940_body26_e138201_d_n6;
            locals.var_fbsq__blk2014_dn7 = assign89940_body26_e138201_d_n7;
            locals.var_fbsq__blk2014_dn8 = assign89940_body26_e138201_d_n8;
            locals.var_fbsq__blk2014_dn9 = assign89940_body26_e138201_d_n9;
            locals.var_fbsq__blk2014_dn10 = assign89940_body26_e138201_d_n10;
            locals.var_fbsq__blk2014_dn11 = assign89940_body26_e138201_d_n11;
            locals.var_fbsq__blk2014_dn14 = assign89940_body26_e138201_d_n14;
            locals.var_fbsq__blk2014_rv = 0.0;
            let (assign89940_body27_e138220, assign89940_body27_e138220_d_n0, assign89940_body27_e138220_d_n2, assign89940_body27_e138220_d_n4, assign89940_body27_e138220_d_n5, assign89940_body27_e138220_d_n6, assign89940_body27_e138220_d_n7, assign89940_body27_e138220_d_n8, assign89940_body27_e138220_d_n9, assign89940_body27_e138220_d_n10, assign89940_body27_e138220_d_n11, assign89940_body27_e138220_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2087 != 0.0)) && (locals.var_guard2093 == 0.0)) {
        let assign89940_body27_e138211: f64 = (1.0 - locals.var_t0);
        let assign89940_body27_e138215: f64 = (1.0 - locals.var_t1);
        let assign89940_body27_e138216: f64 = (locals.var_phi_b_dpss * assign89940_body27_e138215);
        let assign89940_body27_e138217: f64 = (assign89940_body27_e138211 - assign89940_body27_e138216);
        let assign89940_body27_e138218: f64 = (locals.var_beta * assign89940_body27_e138217);
        (assign89940_body27_e138218, ((locals.var_beta_dn0 * assign89940_body27_e138217) + (locals.var_beta * ((-locals.var_t0_dn0) - ((locals.var_phi_b_dpss_dn0 * assign89940_body27_e138215) + (locals.var_phi_b_dpss * (-locals.var_t1_dn0)))))), ((locals.var_beta_dn2 * assign89940_body27_e138217) + (locals.var_beta * ((-locals.var_t0_dn2) - ((locals.var_phi_b_dpss_dn2 * assign89940_body27_e138215) + (locals.var_phi_b_dpss * (-locals.var_t1_dn2)))))), ((locals.var_beta_dn4 * assign89940_body27_e138217) + (locals.var_beta * ((-locals.var_t0_dn4) - ((locals.var_phi_b_dpss_dn4 * assign89940_body27_e138215) + (locals.var_phi_b_dpss * (-locals.var_t1_dn4)))))), ((locals.var_beta_dn5 * assign89940_body27_e138217) + (locals.var_beta * ((-locals.var_t0_dn5) - ((locals.var_phi_b_dpss_dn5 * assign89940_body27_e138215) + (locals.var_phi_b_dpss * (-locals.var_t1_dn5)))))), ((locals.var_beta_dn6 * assign89940_body27_e138217) + (locals.var_beta * ((-locals.var_t0_dn6) - ((locals.var_phi_b_dpss_dn6 * assign89940_body27_e138215) + (locals.var_phi_b_dpss * (-locals.var_t1_dn6)))))), ((locals.var_beta_dn7 * assign89940_body27_e138217) + (locals.var_beta * ((-locals.var_t0_dn7) - ((locals.var_phi_b_dpss_dn7 * assign89940_body27_e138215) + (locals.var_phi_b_dpss * (-locals.var_t1_dn7)))))), ((locals.var_beta_dn8 * assign89940_body27_e138217) + (locals.var_beta * ((-locals.var_t0_dn8) - ((locals.var_phi_b_dpss_dn8 * assign89940_body27_e138215) + (locals.var_phi_b_dpss * (-locals.var_t1_dn8)))))), ((locals.var_beta_dn9 * assign89940_body27_e138217) + (locals.var_beta * ((-locals.var_t0_dn9) - ((locals.var_phi_b_dpss_dn9 * assign89940_body27_e138215) + (locals.var_phi_b_dpss * (-locals.var_t1_dn9)))))), ((locals.var_beta_dn10 * assign89940_body27_e138217) + (locals.var_beta * ((-locals.var_t0_dn10) - ((locals.var_phi_b_dpss_dn10 * assign89940_body27_e138215) + (locals.var_phi_b_dpss * (-locals.var_t1_dn10)))))), ((locals.var_beta_dn11 * assign89940_body27_e138217) + (locals.var_beta * ((-locals.var_t0_dn11) - ((locals.var_phi_b_dpss_dn11 * assign89940_body27_e138215) + (locals.var_phi_b_dpss * (-locals.var_t1_dn11)))))), ((locals.var_beta_dn14 * assign89940_body27_e138217) + (locals.var_beta * ((-locals.var_t0_dn14) - ((locals.var_phi_b_dpss_dn14 * assign89940_body27_e138215) + (locals.var_phi_b_dpss * (-locals.var_t1_dn14)))))),)
    } else {
        (locals.var_fbsq_dpss__blk2015, locals.var_fbsq_dpss__blk2015_dn0, locals.var_fbsq_dpss__blk2015_dn2, locals.var_fbsq_dpss__blk2015_dn4, locals.var_fbsq_dpss__blk2015_dn5, locals.var_fbsq_dpss__blk2015_dn6, locals.var_fbsq_dpss__blk2015_dn7, locals.var_fbsq_dpss__blk2015_dn8, locals.var_fbsq_dpss__blk2015_dn9, locals.var_fbsq_dpss__blk2015_dn10, locals.var_fbsq_dpss__blk2015_dn11, locals.var_fbsq_dpss__blk2015_dn14,)
    }
};
            locals.var_fbsq_dpss__blk2015 = assign89940_body27_e138220;
            locals.var_fbsq_dpss__blk2015_dn0 = assign89940_body27_e138220_d_n0;
            locals.var_fbsq_dpss__blk2015_dn2 = assign89940_body27_e138220_d_n2;
            locals.var_fbsq_dpss__blk2015_dn4 = assign89940_body27_e138220_d_n4;
            locals.var_fbsq_dpss__blk2015_dn5 = assign89940_body27_e138220_d_n5;
            locals.var_fbsq_dpss__blk2015_dn6 = assign89940_body27_e138220_d_n6;
            locals.var_fbsq_dpss__blk2015_dn7 = assign89940_body27_e138220_d_n7;
            locals.var_fbsq_dpss__blk2015_dn8 = assign89940_body27_e138220_d_n8;
            locals.var_fbsq_dpss__blk2015_dn9 = assign89940_body27_e138220_d_n9;
            locals.var_fbsq_dpss__blk2015_dn10 = assign89940_body27_e138220_d_n10;
            locals.var_fbsq_dpss__blk2015_dn11 = assign89940_body27_e138220_d_n11;
            locals.var_fbsq_dpss__blk2015_dn14 = assign89940_body27_e138220_d_n14;
            locals.var_fbsq_dpss__blk2015_rv = 0.0;
            let assign89940_body28_e138222: f64 = (locals.var_chi).abs();
            let assign89940_body28_e138224: f64 = if assign89940_body28_e138222 < 5e-5 { 1.0 } else { 0.0 };
            locals.var_guard2094 = assign89940_body28_e138224;
            locals.var_guard2094_rv = 0.0;
            let (assign89940_body29_e138254, assign89940_body29_e138254_d_n0, assign89940_body29_e138254_d_n2, assign89940_body29_e138254_d_n4, assign89940_body29_e138254_d_n5, assign89940_body29_e138254_d_n6, assign89940_body29_e138254_d_n7, assign89940_body29_e138254_d_n8, assign89940_body29_e138254_d_n9, assign89940_body29_e138254_d_n10, assign89940_body29_e138254_d_n11, assign89940_body29_e138254_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2087 != 0.0)) && (locals.var_guard2094 != 0.0)) {
        let assign89940_body29_e138232: f64 = (locals.var_chi * locals.var_chi);
        let assign89940_body29_e138234: f64 = (assign89940_body29_e138232 / 2.0);
        let assign89940_body29_e138238: f64 = (locals.var_chi / 3.0);
        let assign89940_body29_e138242: f64 = (locals.var_chi / 4.0);
        let assign89940_body29_e138246: f64 = (locals.var_chi / 5.0);
        let assign89940_body29_e138247: f64 = (1.0 + assign89940_body29_e138246);
        let assign89940_body29_e138248: f64 = (assign89940_body29_e138242 * assign89940_body29_e138247);
        let assign89940_body29_e138249: f64 = (1.0 + assign89940_body29_e138248);
        let assign89940_body29_e138250: f64 = (assign89940_body29_e138238 * assign89940_body29_e138249);
        let assign89940_body29_e138251: f64 = (1.0 + assign89940_body29_e138250);
        let assign89940_body29_e138252: f64 = (assign89940_body29_e138234 * assign89940_body29_e138251);
        (assign89940_body29_e138252, (((((locals.var_chi_dn0 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn0)) / 2.0) * assign89940_body29_e138251) + (assign89940_body29_e138234 * (((locals.var_chi_dn0 / 3.0) * assign89940_body29_e138249) + (assign89940_body29_e138238 * (((locals.var_chi_dn0 / 4.0) * assign89940_body29_e138247) + (assign89940_body29_e138242 * (locals.var_chi_dn0 / 5.0))))))), (((((locals.var_chi_dn2 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn2)) / 2.0) * assign89940_body29_e138251) + (assign89940_body29_e138234 * (((locals.var_chi_dn2 / 3.0) * assign89940_body29_e138249) + (assign89940_body29_e138238 * (((locals.var_chi_dn2 / 4.0) * assign89940_body29_e138247) + (assign89940_body29_e138242 * (locals.var_chi_dn2 / 5.0))))))), (((((locals.var_chi_dn4 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn4)) / 2.0) * assign89940_body29_e138251) + (assign89940_body29_e138234 * (((locals.var_chi_dn4 / 3.0) * assign89940_body29_e138249) + (assign89940_body29_e138238 * (((locals.var_chi_dn4 / 4.0) * assign89940_body29_e138247) + (assign89940_body29_e138242 * (locals.var_chi_dn4 / 5.0))))))), (((((locals.var_chi_dn5 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn5)) / 2.0) * assign89940_body29_e138251) + (assign89940_body29_e138234 * (((locals.var_chi_dn5 / 3.0) * assign89940_body29_e138249) + (assign89940_body29_e138238 * (((locals.var_chi_dn5 / 4.0) * assign89940_body29_e138247) + (assign89940_body29_e138242 * (locals.var_chi_dn5 / 5.0))))))), (((((locals.var_chi_dn6 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn6)) / 2.0) * assign89940_body29_e138251) + (assign89940_body29_e138234 * (((locals.var_chi_dn6 / 3.0) * assign89940_body29_e138249) + (assign89940_body29_e138238 * (((locals.var_chi_dn6 / 4.0) * assign89940_body29_e138247) + (assign89940_body29_e138242 * (locals.var_chi_dn6 / 5.0))))))), (((((locals.var_chi_dn7 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn7)) / 2.0) * assign89940_body29_e138251) + (assign89940_body29_e138234 * (((locals.var_chi_dn7 / 3.0) * assign89940_body29_e138249) + (assign89940_body29_e138238 * (((locals.var_chi_dn7 / 4.0) * assign89940_body29_e138247) + (assign89940_body29_e138242 * (locals.var_chi_dn7 / 5.0))))))), (((((locals.var_chi_dn8 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn8)) / 2.0) * assign89940_body29_e138251) + (assign89940_body29_e138234 * (((locals.var_chi_dn8 / 3.0) * assign89940_body29_e138249) + (assign89940_body29_e138238 * (((locals.var_chi_dn8 / 4.0) * assign89940_body29_e138247) + (assign89940_body29_e138242 * (locals.var_chi_dn8 / 5.0))))))), (((((locals.var_chi_dn9 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn9)) / 2.0) * assign89940_body29_e138251) + (assign89940_body29_e138234 * (((locals.var_chi_dn9 / 3.0) * assign89940_body29_e138249) + (assign89940_body29_e138238 * (((locals.var_chi_dn9 / 4.0) * assign89940_body29_e138247) + (assign89940_body29_e138242 * (locals.var_chi_dn9 / 5.0))))))), (((((locals.var_chi_dn10 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn10)) / 2.0) * assign89940_body29_e138251) + (assign89940_body29_e138234 * (((locals.var_chi_dn10 / 3.0) * assign89940_body29_e138249) + (assign89940_body29_e138238 * (((locals.var_chi_dn10 / 4.0) * assign89940_body29_e138247) + (assign89940_body29_e138242 * (locals.var_chi_dn10 / 5.0))))))), (((((locals.var_chi_dn11 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn11)) / 2.0) * assign89940_body29_e138251) + (assign89940_body29_e138234 * (((locals.var_chi_dn11 / 3.0) * assign89940_body29_e138249) + (assign89940_body29_e138238 * (((locals.var_chi_dn11 / 4.0) * assign89940_body29_e138247) + (assign89940_body29_e138242 * (locals.var_chi_dn11 / 5.0))))))), (((((locals.var_chi_dn14 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn14)) / 2.0) * assign89940_body29_e138251) + (assign89940_body29_e138234 * (((locals.var_chi_dn14 / 3.0) * assign89940_body29_e138249) + (assign89940_body29_e138238 * (((locals.var_chi_dn14 / 4.0) * assign89940_body29_e138247) + (assign89940_body29_e138242 * (locals.var_chi_dn14 / 5.0))))))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
            locals.var_t0 = assign89940_body29_e138254;
            locals.var_t0_dn0 = assign89940_body29_e138254_d_n0;
            locals.var_t0_dn2 = assign89940_body29_e138254_d_n2;
            locals.var_t0_dn4 = assign89940_body29_e138254_d_n4;
            locals.var_t0_dn5 = assign89940_body29_e138254_d_n5;
            locals.var_t0_dn6 = assign89940_body29_e138254_d_n6;
            locals.var_t0_dn7 = assign89940_body29_e138254_d_n7;
            locals.var_t0_dn8 = assign89940_body29_e138254_d_n8;
            locals.var_t0_dn9 = assign89940_body29_e138254_d_n9;
            locals.var_t0_dn10 = assign89940_body29_e138254_d_n10;
            locals.var_t0_dn11 = assign89940_body29_e138254_d_n11;
            locals.var_t0_dn14 = assign89940_body29_e138254_d_n14;
            locals.var_t0_rv = 0.0;
            let (assign89940_body30_e138280, assign89940_body30_e138280_d_n0, assign89940_body30_e138280_d_n2, assign89940_body30_e138280_d_n4, assign89940_body30_e138280_d_n5, assign89940_body30_e138280_d_n6, assign89940_body30_e138280_d_n7, assign89940_body30_e138280_d_n8, assign89940_body30_e138280_d_n9, assign89940_body30_e138280_d_n10, assign89940_body30_e138280_d_n11, assign89940_body30_e138280_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2087 != 0.0)) && (locals.var_guard2094 != 0.0)) {
        let assign89940_body30_e138264: f64 = (locals.var_chi / 2.0);
        let assign89940_body30_e138268: f64 = (locals.var_chi / 3.0);
        let assign89940_body30_e138272: f64 = (locals.var_chi / 4.0);
        let assign89940_body30_e138273: f64 = (1.0 + assign89940_body30_e138272);
        let assign89940_body30_e138274: f64 = (assign89940_body30_e138268 * assign89940_body30_e138273);
        let assign89940_body30_e138275: f64 = (1.0 + assign89940_body30_e138274);
        let assign89940_body30_e138276: f64 = (assign89940_body30_e138264 * assign89940_body30_e138275);
        let assign89940_body30_e138277: f64 = (1.0 + assign89940_body30_e138276);
        let assign89940_body30_e138278: f64 = (locals.var_chi * assign89940_body30_e138277);
        (assign89940_body30_e138278, ((locals.var_chi_dn0 * assign89940_body30_e138277) + (locals.var_chi * (((locals.var_chi_dn0 / 2.0) * assign89940_body30_e138275) + (assign89940_body30_e138264 * (((locals.var_chi_dn0 / 3.0) * assign89940_body30_e138273) + (assign89940_body30_e138268 * (locals.var_chi_dn0 / 4.0))))))), ((locals.var_chi_dn2 * assign89940_body30_e138277) + (locals.var_chi * (((locals.var_chi_dn2 / 2.0) * assign89940_body30_e138275) + (assign89940_body30_e138264 * (((locals.var_chi_dn2 / 3.0) * assign89940_body30_e138273) + (assign89940_body30_e138268 * (locals.var_chi_dn2 / 4.0))))))), ((locals.var_chi_dn4 * assign89940_body30_e138277) + (locals.var_chi * (((locals.var_chi_dn4 / 2.0) * assign89940_body30_e138275) + (assign89940_body30_e138264 * (((locals.var_chi_dn4 / 3.0) * assign89940_body30_e138273) + (assign89940_body30_e138268 * (locals.var_chi_dn4 / 4.0))))))), ((locals.var_chi_dn5 * assign89940_body30_e138277) + (locals.var_chi * (((locals.var_chi_dn5 / 2.0) * assign89940_body30_e138275) + (assign89940_body30_e138264 * (((locals.var_chi_dn5 / 3.0) * assign89940_body30_e138273) + (assign89940_body30_e138268 * (locals.var_chi_dn5 / 4.0))))))), ((locals.var_chi_dn6 * assign89940_body30_e138277) + (locals.var_chi * (((locals.var_chi_dn6 / 2.0) * assign89940_body30_e138275) + (assign89940_body30_e138264 * (((locals.var_chi_dn6 / 3.0) * assign89940_body30_e138273) + (assign89940_body30_e138268 * (locals.var_chi_dn6 / 4.0))))))), ((locals.var_chi_dn7 * assign89940_body30_e138277) + (locals.var_chi * (((locals.var_chi_dn7 / 2.0) * assign89940_body30_e138275) + (assign89940_body30_e138264 * (((locals.var_chi_dn7 / 3.0) * assign89940_body30_e138273) + (assign89940_body30_e138268 * (locals.var_chi_dn7 / 4.0))))))), ((locals.var_chi_dn8 * assign89940_body30_e138277) + (locals.var_chi * (((locals.var_chi_dn8 / 2.0) * assign89940_body30_e138275) + (assign89940_body30_e138264 * (((locals.var_chi_dn8 / 3.0) * assign89940_body30_e138273) + (assign89940_body30_e138268 * (locals.var_chi_dn8 / 4.0))))))), ((locals.var_chi_dn9 * assign89940_body30_e138277) + (locals.var_chi * (((locals.var_chi_dn9 / 2.0) * assign89940_body30_e138275) + (assign89940_body30_e138264 * (((locals.var_chi_dn9 / 3.0) * assign89940_body30_e138273) + (assign89940_body30_e138268 * (locals.var_chi_dn9 / 4.0))))))), ((locals.var_chi_dn10 * assign89940_body30_e138277) + (locals.var_chi * (((locals.var_chi_dn10 / 2.0) * assign89940_body30_e138275) + (assign89940_body30_e138264 * (((locals.var_chi_dn10 / 3.0) * assign89940_body30_e138273) + (assign89940_body30_e138268 * (locals.var_chi_dn10 / 4.0))))))), ((locals.var_chi_dn11 * assign89940_body30_e138277) + (locals.var_chi * (((locals.var_chi_dn11 / 2.0) * assign89940_body30_e138275) + (assign89940_body30_e138264 * (((locals.var_chi_dn11 / 3.0) * assign89940_body30_e138273) + (assign89940_body30_e138268 * (locals.var_chi_dn11 / 4.0))))))), ((locals.var_chi_dn14 * assign89940_body30_e138277) + (locals.var_chi * (((locals.var_chi_dn14 / 2.0) * assign89940_body30_e138275) + (assign89940_body30_e138264 * (((locals.var_chi_dn14 / 3.0) * assign89940_body30_e138273) + (assign89940_body30_e138268 * (locals.var_chi_dn14 / 4.0))))))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
            locals.var_t1 = assign89940_body30_e138280;
            locals.var_t1_dn0 = assign89940_body30_e138280_d_n0;
            locals.var_t1_dn2 = assign89940_body30_e138280_d_n2;
            locals.var_t1_dn4 = assign89940_body30_e138280_d_n4;
            locals.var_t1_dn5 = assign89940_body30_e138280_d_n5;
            locals.var_t1_dn6 = assign89940_body30_e138280_d_n6;
            locals.var_t1_dn7 = assign89940_body30_e138280_d_n7;
            locals.var_t1_dn8 = assign89940_body30_e138280_d_n8;
            locals.var_t1_dn9 = assign89940_body30_e138280_d_n9;
            locals.var_t1_dn10 = assign89940_body30_e138280_d_n10;
            locals.var_t1_dn11 = assign89940_body30_e138280_d_n11;
            locals.var_t1_dn14 = assign89940_body30_e138280_d_n14;
            locals.var_t1_rv = 0.0;
            let (assign89940_body31_e138290, assign89940_body31_e138290_d_n0, assign89940_body31_e138290_d_n2, assign89940_body31_e138290_d_n4, assign89940_body31_e138290_d_n5, assign89940_body31_e138290_d_n6, assign89940_body31_e138290_d_n7, assign89940_body31_e138290_d_n8, assign89940_body31_e138290_d_n9, assign89940_body31_e138290_d_n10, assign89940_body31_e138290_d_n11, assign89940_body31_e138290_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2087 != 0.0)) && (locals.var_guard2094 != 0.0)) {
        let assign89940_body31_e138288: f64 = (locals.var_cfs1 * locals.var_t0);
        (assign89940_body31_e138288, ((locals.var_cfs1_dn0 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn0)), ((locals.var_cfs1_dn2 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn2)), ((locals.var_cfs1_dn4 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn4)), ((locals.var_cfs1_dn5 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn5)), ((locals.var_cfs1_dn6 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn6)), ((locals.var_cfs1_dn7 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn7)), ((locals.var_cfs1_dn8 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn8)), ((locals.var_cfs1_dn9 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn9)), ((locals.var_cfs1_dn10 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn10)), ((locals.var_cfs1_dn11 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn11)), ((locals.var_cfs1_dn14 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn14)),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn11, locals.var_fs01_dn14,)
    }
};
            locals.var_fs01 = assign89940_body31_e138290;
            locals.var_fs01_dn0 = assign89940_body31_e138290_d_n0;
            locals.var_fs01_dn2 = assign89940_body31_e138290_d_n2;
            locals.var_fs01_dn4 = assign89940_body31_e138290_d_n4;
            locals.var_fs01_dn5 = assign89940_body31_e138290_d_n5;
            locals.var_fs01_dn6 = assign89940_body31_e138290_d_n6;
            locals.var_fs01_dn7 = assign89940_body31_e138290_d_n7;
            locals.var_fs01_dn8 = assign89940_body31_e138290_d_n8;
            locals.var_fs01_dn9 = assign89940_body31_e138290_d_n9;
            locals.var_fs01_dn10 = assign89940_body31_e138290_d_n10;
            locals.var_fs01_dn11 = assign89940_body31_e138290_d_n11;
            locals.var_fs01_dn14 = assign89940_body31_e138290_d_n14;
            locals.var_fs01_rv = 0.0;
            let (assign89940_body32_e138302, assign89940_body32_e138302_d_n0, assign89940_body32_e138302_d_n2, assign89940_body32_e138302_d_n4, assign89940_body32_e138302_d_n5, assign89940_body32_e138302_d_n6, assign89940_body32_e138302_d_n7, assign89940_body32_e138302_d_n8, assign89940_body32_e138302_d_n9, assign89940_body32_e138302_d_n10, assign89940_body32_e138302_d_n11, assign89940_body32_e138302_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2087 != 0.0)) && (locals.var_guard2094 != 0.0)) {
        let assign89940_body32_e138298: f64 = (locals.var_cfs1 * locals.var_t1);
        let assign89940_body32_e138300: f64 = (assign89940_body32_e138298 * locals.var_beta);
        (assign89940_body32_e138300, ((((locals.var_cfs1_dn0 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn0)) * locals.var_beta) + (assign89940_body32_e138298 * locals.var_beta_dn0)), ((((locals.var_cfs1_dn2 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn2)) * locals.var_beta) + (assign89940_body32_e138298 * locals.var_beta_dn2)), ((((locals.var_cfs1_dn4 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn4)) * locals.var_beta) + (assign89940_body32_e138298 * locals.var_beta_dn4)), ((((locals.var_cfs1_dn5 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn5)) * locals.var_beta) + (assign89940_body32_e138298 * locals.var_beta_dn5)), ((((locals.var_cfs1_dn6 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn6)) * locals.var_beta) + (assign89940_body32_e138298 * locals.var_beta_dn6)), ((((locals.var_cfs1_dn7 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn7)) * locals.var_beta) + (assign89940_body32_e138298 * locals.var_beta_dn7)), ((((locals.var_cfs1_dn8 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn8)) * locals.var_beta) + (assign89940_body32_e138298 * locals.var_beta_dn8)), ((((locals.var_cfs1_dn9 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn9)) * locals.var_beta) + (assign89940_body32_e138298 * locals.var_beta_dn9)), ((((locals.var_cfs1_dn10 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn10)) * locals.var_beta) + (assign89940_body32_e138298 * locals.var_beta_dn10)), ((((locals.var_cfs1_dn11 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn11)) * locals.var_beta) + (assign89940_body32_e138298 * locals.var_beta_dn11)), ((((locals.var_cfs1_dn14 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn14)) * locals.var_beta) + (assign89940_body32_e138298 * locals.var_beta_dn14)),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn11, locals.var_fs01_dps0_dn14,)
    }
};
            locals.var_fs01_dps0 = assign89940_body32_e138302;
            locals.var_fs01_dps0_dn0 = assign89940_body32_e138302_d_n0;
            locals.var_fs01_dps0_dn2 = assign89940_body32_e138302_d_n2;
            locals.var_fs01_dps0_dn4 = assign89940_body32_e138302_d_n4;
            locals.var_fs01_dps0_dn5 = assign89940_body32_e138302_d_n5;
            locals.var_fs01_dps0_dn6 = assign89940_body32_e138302_d_n6;
            locals.var_fs01_dps0_dn7 = assign89940_body32_e138302_d_n7;
            locals.var_fs01_dps0_dn8 = assign89940_body32_e138302_d_n8;
            locals.var_fs01_dps0_dn9 = assign89940_body32_e138302_d_n9;
            locals.var_fs01_dps0_dn10 = assign89940_body32_e138302_d_n10;
            locals.var_fs01_dps0_dn11 = assign89940_body32_e138302_d_n11;
            locals.var_fs01_dps0_dn14 = assign89940_body32_e138302_d_n14;
            locals.var_fs01_dps0_rv = 0.0;
            let assign89940_body33_e138304: f64 = (locals.var_chi).abs();
            let assign89940_body33_e138306: f64 = if assign89940_body33_e138304 < 60.0 { 1.0 } else { 0.0 };
            locals.var_guard2095 = assign89940_body33_e138306;
            locals.var_guard2095_rv = 0.0;
            let (assign89940_body35_e138337, assign89940_body35_e138337_d_n0, assign89940_body35_e138337_d_n2, assign89940_body35_e138337_d_n4, assign89940_body35_e138337_d_n5, assign89940_body35_e138337_d_n6, assign89940_body35_e138337_d_n7, assign89940_body35_e138337_d_n8, assign89940_body35_e138337_d_n9, assign89940_body35_e138337_d_n10, assign89940_body35_e138337_d_n11, assign89940_body35_e138337_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2087 != 0.0)) && (locals.var_guard2094 == 0.0)) && (locals.var_guard2095 != 0.0)) {
        let assign89940_body35_e138335: f64 = (locals.var_chi).exp();
        (assign89940_body35_e138335, (assign89940_body35_e138335 * locals.var_chi_dn0), (assign89940_body35_e138335 * locals.var_chi_dn2), (assign89940_body35_e138335 * locals.var_chi_dn4), (assign89940_body35_e138335 * locals.var_chi_dn5), (assign89940_body35_e138335 * locals.var_chi_dn6), (assign89940_body35_e138335 * locals.var_chi_dn7), (assign89940_body35_e138335 * locals.var_chi_dn8), (assign89940_body35_e138335 * locals.var_chi_dn9), (assign89940_body35_e138335 * locals.var_chi_dn10), (assign89940_body35_e138335 * locals.var_chi_dn11), (assign89940_body35_e138335 * locals.var_chi_dn14),)
    } else {
        (locals.var_exp_chi, locals.var_exp_chi_dn0, locals.var_exp_chi_dn2, locals.var_exp_chi_dn4, locals.var_exp_chi_dn5, locals.var_exp_chi_dn6, locals.var_exp_chi_dn7, locals.var_exp_chi_dn8, locals.var_exp_chi_dn9, locals.var_exp_chi_dn10, locals.var_exp_chi_dn11, locals.var_exp_chi_dn14,)
    }
};
            locals.var_exp_chi = assign89940_body35_e138337;
            locals.var_exp_chi_dn0 = assign89940_body35_e138337_d_n0;
            locals.var_exp_chi_dn2 = assign89940_body35_e138337_d_n2;
            locals.var_exp_chi_dn4 = assign89940_body35_e138337_d_n4;
            locals.var_exp_chi_dn5 = assign89940_body35_e138337_d_n5;
            locals.var_exp_chi_dn6 = assign89940_body35_e138337_d_n6;
            locals.var_exp_chi_dn7 = assign89940_body35_e138337_d_n7;
            locals.var_exp_chi_dn8 = assign89940_body35_e138337_d_n8;
            locals.var_exp_chi_dn9 = assign89940_body35_e138337_d_n9;
            locals.var_exp_chi_dn10 = assign89940_body35_e138337_d_n10;
            locals.var_exp_chi_dn11 = assign89940_body35_e138337_d_n11;
            locals.var_exp_chi_dn14 = assign89940_body35_e138337_d_n14;
            locals.var_exp_chi_rv = 0.0;
            let (assign89940_body36_e138350, assign89940_body36_e138350_d_n0, assign89940_body36_e138350_d_n2, assign89940_body36_e138350_d_n4, assign89940_body36_e138350_d_n5, assign89940_body36_e138350_d_n6, assign89940_body36_e138350_d_n7, assign89940_body36_e138350_d_n8, assign89940_body36_e138350_d_n9, assign89940_body36_e138350_d_n10, assign89940_body36_e138350_d_n11, assign89940_body36_e138350_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2087 != 0.0)) && (locals.var_guard2094 == 0.0)) && (locals.var_guard2095 != 0.0)) {
        let assign89940_body36_e138348: f64 = (locals.var_exp_chi - 1.0);
        (assign89940_body36_e138348, locals.var_exp_chi_dn0, locals.var_exp_chi_dn2, locals.var_exp_chi_dn4, locals.var_exp_chi_dn5, locals.var_exp_chi_dn6, locals.var_exp_chi_dn7, locals.var_exp_chi_dn8, locals.var_exp_chi_dn9, locals.var_exp_chi_dn10, locals.var_exp_chi_dn11, locals.var_exp_chi_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
            locals.var_t1 = assign89940_body36_e138350;
            locals.var_t1_dn0 = assign89940_body36_e138350_d_n0;
            locals.var_t1_dn2 = assign89940_body36_e138350_d_n2;
            locals.var_t1_dn4 = assign89940_body36_e138350_d_n4;
            locals.var_t1_dn5 = assign89940_body36_e138350_d_n5;
            locals.var_t1_dn6 = assign89940_body36_e138350_d_n6;
            locals.var_t1_dn7 = assign89940_body36_e138350_d_n7;
            locals.var_t1_dn8 = assign89940_body36_e138350_d_n8;
            locals.var_t1_dn9 = assign89940_body36_e138350_d_n9;
            locals.var_t1_dn10 = assign89940_body36_e138350_d_n10;
            locals.var_t1_dn11 = assign89940_body36_e138350_d_n11;
            locals.var_t1_dn14 = assign89940_body36_e138350_d_n14;
            locals.var_t1_rv = 0.0;
            let (assign89940_body37_e138365, assign89940_body37_e138365_d_n0, assign89940_body37_e138365_d_n2, assign89940_body37_e138365_d_n4, assign89940_body37_e138365_d_n5, assign89940_body37_e138365_d_n6, assign89940_body37_e138365_d_n7, assign89940_body37_e138365_d_n8, assign89940_body37_e138365_d_n9, assign89940_body37_e138365_d_n10, assign89940_body37_e138365_d_n11, assign89940_body37_e138365_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2087 != 0.0)) && (locals.var_guard2094 == 0.0)) && (locals.var_guard2095 != 0.0)) {
        let assign89940_body37_e138362: f64 = (locals.var_t1 - locals.var_chi);
        let assign89940_body37_e138363: f64 = (locals.var_cfs1 * assign89940_body37_e138362);
        (assign89940_body37_e138363, ((locals.var_cfs1_dn0 * assign89940_body37_e138362) + (locals.var_cfs1 * (locals.var_t1_dn0 - locals.var_chi_dn0))), ((locals.var_cfs1_dn2 * assign89940_body37_e138362) + (locals.var_cfs1 * (locals.var_t1_dn2 - locals.var_chi_dn2))), ((locals.var_cfs1_dn4 * assign89940_body37_e138362) + (locals.var_cfs1 * (locals.var_t1_dn4 - locals.var_chi_dn4))), ((locals.var_cfs1_dn5 * assign89940_body37_e138362) + (locals.var_cfs1 * (locals.var_t1_dn5 - locals.var_chi_dn5))), ((locals.var_cfs1_dn6 * assign89940_body37_e138362) + (locals.var_cfs1 * (locals.var_t1_dn6 - locals.var_chi_dn6))), ((locals.var_cfs1_dn7 * assign89940_body37_e138362) + (locals.var_cfs1 * (locals.var_t1_dn7 - locals.var_chi_dn7))), ((locals.var_cfs1_dn8 * assign89940_body37_e138362) + (locals.var_cfs1 * (locals.var_t1_dn8 - locals.var_chi_dn8))), ((locals.var_cfs1_dn9 * assign89940_body37_e138362) + (locals.var_cfs1 * (locals.var_t1_dn9 - locals.var_chi_dn9))), ((locals.var_cfs1_dn10 * assign89940_body37_e138362) + (locals.var_cfs1 * (locals.var_t1_dn10 - locals.var_chi_dn10))), ((locals.var_cfs1_dn11 * assign89940_body37_e138362) + (locals.var_cfs1 * (locals.var_t1_dn11 - locals.var_chi_dn11))), ((locals.var_cfs1_dn14 * assign89940_body37_e138362) + (locals.var_cfs1 * (locals.var_t1_dn14 - locals.var_chi_dn14))),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn11, locals.var_fs01_dn14,)
    }
};
            locals.var_fs01 = assign89940_body37_e138365;
            locals.var_fs01_dn0 = assign89940_body37_e138365_d_n0;
            locals.var_fs01_dn2 = assign89940_body37_e138365_d_n2;
            locals.var_fs01_dn4 = assign89940_body37_e138365_d_n4;
            locals.var_fs01_dn5 = assign89940_body37_e138365_d_n5;
            locals.var_fs01_dn6 = assign89940_body37_e138365_d_n6;
            locals.var_fs01_dn7 = assign89940_body37_e138365_d_n7;
            locals.var_fs01_dn8 = assign89940_body37_e138365_d_n8;
            locals.var_fs01_dn9 = assign89940_body37_e138365_d_n9;
            locals.var_fs01_dn10 = assign89940_body37_e138365_d_n10;
            locals.var_fs01_dn11 = assign89940_body37_e138365_d_n11;
            locals.var_fs01_dn14 = assign89940_body37_e138365_d_n14;
            locals.var_fs01_rv = 0.0;
            let (assign89940_body38_e138380, assign89940_body38_e138380_d_n0, assign89940_body38_e138380_d_n2, assign89940_body38_e138380_d_n4, assign89940_body38_e138380_d_n5, assign89940_body38_e138380_d_n6, assign89940_body38_e138380_d_n7, assign89940_body38_e138380_d_n8, assign89940_body38_e138380_d_n9, assign89940_body38_e138380_d_n10, assign89940_body38_e138380_d_n11, assign89940_body38_e138380_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2087 != 0.0)) && (locals.var_guard2094 == 0.0)) && (locals.var_guard2095 != 0.0)) {
        let assign89940_body38_e138376: f64 = (locals.var_cfs1 * locals.var_beta);
        let assign89940_body38_e138378: f64 = (assign89940_body38_e138376 * locals.var_t1);
        (assign89940_body38_e138378, ((((locals.var_cfs1_dn0 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn0)) * locals.var_t1) + (assign89940_body38_e138376 * locals.var_t1_dn0)), ((((locals.var_cfs1_dn2 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn2)) * locals.var_t1) + (assign89940_body38_e138376 * locals.var_t1_dn2)), ((((locals.var_cfs1_dn4 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn4)) * locals.var_t1) + (assign89940_body38_e138376 * locals.var_t1_dn4)), ((((locals.var_cfs1_dn5 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn5)) * locals.var_t1) + (assign89940_body38_e138376 * locals.var_t1_dn5)), ((((locals.var_cfs1_dn6 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn6)) * locals.var_t1) + (assign89940_body38_e138376 * locals.var_t1_dn6)), ((((locals.var_cfs1_dn7 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn7)) * locals.var_t1) + (assign89940_body38_e138376 * locals.var_t1_dn7)), ((((locals.var_cfs1_dn8 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn8)) * locals.var_t1) + (assign89940_body38_e138376 * locals.var_t1_dn8)), ((((locals.var_cfs1_dn9 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn9)) * locals.var_t1) + (assign89940_body38_e138376 * locals.var_t1_dn9)), ((((locals.var_cfs1_dn10 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn10)) * locals.var_t1) + (assign89940_body38_e138376 * locals.var_t1_dn10)), ((((locals.var_cfs1_dn11 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn11)) * locals.var_t1) + (assign89940_body38_e138376 * locals.var_t1_dn11)), ((((locals.var_cfs1_dn14 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn14)) * locals.var_t1) + (assign89940_body38_e138376 * locals.var_t1_dn14)),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn11, locals.var_fs01_dps0_dn14,)
    }
};
            locals.var_fs01_dps0 = assign89940_body38_e138380;
            locals.var_fs01_dps0_dn0 = assign89940_body38_e138380_d_n0;
            locals.var_fs01_dps0_dn2 = assign89940_body38_e138380_d_n2;
            locals.var_fs01_dps0_dn4 = assign89940_body38_e138380_d_n4;
            locals.var_fs01_dps0_dn5 = assign89940_body38_e138380_d_n5;
            locals.var_fs01_dps0_dn6 = assign89940_body38_e138380_d_n6;
            locals.var_fs01_dps0_dn7 = assign89940_body38_e138380_d_n7;
            locals.var_fs01_dps0_dn8 = assign89940_body38_e138380_d_n8;
            locals.var_fs01_dps0_dn9 = assign89940_body38_e138380_d_n9;
            locals.var_fs01_dps0_dn10 = assign89940_body38_e138380_d_n10;
            locals.var_fs01_dps0_dn11 = assign89940_body38_e138380_d_n11;
            locals.var_fs01_dps0_dn14 = assign89940_body38_e138380_d_n14;
            locals.var_fs01_dps0_rv = 0.0;
            let (assign89940_body40_e138415, assign89940_body40_e138415_d_n0, assign89940_body40_e138415_d_n2, assign89940_body40_e138415_d_n4, assign89940_body40_e138415_d_n5, assign89940_body40_e138415_d_n6, assign89940_body40_e138415_d_n7, assign89940_body40_e138415_d_n8, assign89940_body40_e138415_d_n9, assign89940_body40_e138415_d_n10, assign89940_body40_e138415_d_n11, assign89940_body40_e138415_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2087 != 0.0)) && (locals.var_guard2094 == 0.0)) && (locals.var_guard2095 == 0.0)) {
        let assign89940_body40_e138412: f64 = (locals.var_beta * locals.var_ps0ld);
        let assign89940_body40_e138413: f64 = (assign89940_body40_e138412).exp();
        (assign89940_body40_e138413, (assign89940_body40_e138413 * ((locals.var_beta_dn0 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn0))), (assign89940_body40_e138413 * ((locals.var_beta_dn2 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn2))), (assign89940_body40_e138413 * ((locals.var_beta_dn4 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn4))), (assign89940_body40_e138413 * ((locals.var_beta_dn5 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn5))), (assign89940_body40_e138413 * ((locals.var_beta_dn6 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn6))), (assign89940_body40_e138413 * ((locals.var_beta_dn7 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn7))), (assign89940_body40_e138413 * ((locals.var_beta_dn8 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn8))), (assign89940_body40_e138413 * ((locals.var_beta_dn9 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn9))), (assign89940_body40_e138413 * ((locals.var_beta_dn10 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn10))), (assign89940_body40_e138413 * ((locals.var_beta_dn11 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn11))), (assign89940_body40_e138413 * ((locals.var_beta_dn14 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn14))),)
    } else {
        (locals.var_exp_bps0, locals.var_exp_bps0_dn0, locals.var_exp_bps0_dn2, locals.var_exp_bps0_dn4, locals.var_exp_bps0_dn5, locals.var_exp_bps0_dn6, locals.var_exp_bps0_dn7, locals.var_exp_bps0_dn8, locals.var_exp_bps0_dn9, locals.var_exp_bps0_dn10, locals.var_exp_bps0_dn11, locals.var_exp_bps0_dn14,)
    }
};
            locals.var_exp_bps0 = assign89940_body40_e138415;
            locals.var_exp_bps0_dn0 = assign89940_body40_e138415_d_n0;
            locals.var_exp_bps0_dn2 = assign89940_body40_e138415_d_n2;
            locals.var_exp_bps0_dn4 = assign89940_body40_e138415_d_n4;
            locals.var_exp_bps0_dn5 = assign89940_body40_e138415_d_n5;
            locals.var_exp_bps0_dn6 = assign89940_body40_e138415_d_n6;
            locals.var_exp_bps0_dn7 = assign89940_body40_e138415_d_n7;
            locals.var_exp_bps0_dn8 = assign89940_body40_e138415_d_n8;
            locals.var_exp_bps0_dn9 = assign89940_body40_e138415_d_n9;
            locals.var_exp_bps0_dn10 = assign89940_body40_e138415_d_n10;
            locals.var_exp_bps0_dn11 = assign89940_body40_e138415_d_n11;
            locals.var_exp_bps0_dn14 = assign89940_body40_e138415_d_n14;
            locals.var_exp_bps0_rv = 0.0;
            let (assign89940_body41_e138435, assign89940_body41_e138435_d_n0, assign89940_body41_e138435_d_n2, assign89940_body41_e138435_d_n4, assign89940_body41_e138435_d_n5, assign89940_body41_e138435_d_n6, assign89940_body41_e138435_d_n7, assign89940_body41_e138435_d_n8, assign89940_body41_e138435_d_n9, assign89940_body41_e138435_d_n10, assign89940_body41_e138435_d_n11, assign89940_body41_e138435_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2087 != 0.0)) && (locals.var_guard2094 == 0.0)) && (locals.var_guard2095 == 0.0)) {
        let assign89940_body41_e138430: f64 = (locals.var_chi + 1.0);
        let assign89940_body41_e138431: f64 = (locals.var_exp_bvbs * assign89940_body41_e138430);
        let assign89940_body41_e138432: f64 = (locals.var_exp_bps0 - assign89940_body41_e138431);
        let assign89940_body41_e138433: f64 = (locals.var_cnst1over * assign89940_body41_e138432);
        (assign89940_body41_e138433, ((locals.var_cnst1over_dn0 * assign89940_body41_e138432) + (locals.var_cnst1over * (locals.var_exp_bps0_dn0 - ((locals.var_exp_bvbs_dn0 * assign89940_body41_e138430) + (locals.var_exp_bvbs * locals.var_chi_dn0))))), ((locals.var_cnst1over_dn2 * assign89940_body41_e138432) + (locals.var_cnst1over * (locals.var_exp_bps0_dn2 - ((locals.var_exp_bvbs_dn2 * assign89940_body41_e138430) + (locals.var_exp_bvbs * locals.var_chi_dn2))))), ((locals.var_cnst1over_dn4 * assign89940_body41_e138432) + (locals.var_cnst1over * (locals.var_exp_bps0_dn4 - ((locals.var_exp_bvbs_dn4 * assign89940_body41_e138430) + (locals.var_exp_bvbs * locals.var_chi_dn4))))), ((locals.var_cnst1over_dn5 * assign89940_body41_e138432) + (locals.var_cnst1over * (locals.var_exp_bps0_dn5 - ((locals.var_exp_bvbs_dn5 * assign89940_body41_e138430) + (locals.var_exp_bvbs * locals.var_chi_dn5))))), ((locals.var_cnst1over_dn6 * assign89940_body41_e138432) + (locals.var_cnst1over * (locals.var_exp_bps0_dn6 - ((locals.var_exp_bvbs_dn6 * assign89940_body41_e138430) + (locals.var_exp_bvbs * locals.var_chi_dn6))))), ((locals.var_cnst1over_dn7 * assign89940_body41_e138432) + (locals.var_cnst1over * (locals.var_exp_bps0_dn7 - ((locals.var_exp_bvbs_dn7 * assign89940_body41_e138430) + (locals.var_exp_bvbs * locals.var_chi_dn7))))), ((locals.var_cnst1over_dn8 * assign89940_body41_e138432) + (locals.var_cnst1over * (locals.var_exp_bps0_dn8 - ((locals.var_exp_bvbs_dn8 * assign89940_body41_e138430) + (locals.var_exp_bvbs * locals.var_chi_dn8))))), ((locals.var_cnst1over_dn9 * assign89940_body41_e138432) + (locals.var_cnst1over * (locals.var_exp_bps0_dn9 - ((locals.var_exp_bvbs_dn9 * assign89940_body41_e138430) + (locals.var_exp_bvbs * locals.var_chi_dn9))))), ((locals.var_cnst1over_dn10 * assign89940_body41_e138432) + (locals.var_cnst1over * (locals.var_exp_bps0_dn10 - ((locals.var_exp_bvbs_dn10 * assign89940_body41_e138430) + (locals.var_exp_bvbs * locals.var_chi_dn10))))), ((locals.var_cnst1over_dn11 * assign89940_body41_e138432) + (locals.var_cnst1over * (locals.var_exp_bps0_dn11 - ((locals.var_exp_bvbs_dn11 * assign89940_body41_e138430) + (locals.var_exp_bvbs * locals.var_chi_dn11))))), ((locals.var_cnst1over_dn14 * assign89940_body41_e138432) + (locals.var_cnst1over * (locals.var_exp_bps0_dn14 - ((locals.var_exp_bvbs_dn14 * assign89940_body41_e138430) + (locals.var_exp_bvbs * locals.var_chi_dn14))))),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn11, locals.var_fs01_dn14,)
    }
};
            locals.var_fs01 = assign89940_body41_e138435;
            locals.var_fs01_dn0 = assign89940_body41_e138435_d_n0;
            locals.var_fs01_dn2 = assign89940_body41_e138435_d_n2;
            locals.var_fs01_dn4 = assign89940_body41_e138435_d_n4;
            locals.var_fs01_dn5 = assign89940_body41_e138435_d_n5;
            locals.var_fs01_dn6 = assign89940_body41_e138435_d_n6;
            locals.var_fs01_dn7 = assign89940_body41_e138435_d_n7;
            locals.var_fs01_dn8 = assign89940_body41_e138435_d_n8;
            locals.var_fs01_dn9 = assign89940_body41_e138435_d_n9;
            locals.var_fs01_dn10 = assign89940_body41_e138435_d_n10;
            locals.var_fs01_dn11 = assign89940_body41_e138435_d_n11;
            locals.var_fs01_dn14 = assign89940_body41_e138435_d_n14;
            locals.var_fs01_rv = 0.0;
            let (assign89940_body42_e138453, assign89940_body42_e138453_d_n0, assign89940_body42_e138453_d_n2, assign89940_body42_e138453_d_n4, assign89940_body42_e138453_d_n5, assign89940_body42_e138453_d_n6, assign89940_body42_e138453_d_n7, assign89940_body42_e138453_d_n8, assign89940_body42_e138453_d_n9, assign89940_body42_e138453_d_n10, assign89940_body42_e138453_d_n11, assign89940_body42_e138453_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2087 != 0.0)) && (locals.var_guard2094 == 0.0)) && (locals.var_guard2095 == 0.0)) {
        let assign89940_body42_e138447: f64 = (locals.var_cnst1over * locals.var_beta);
        let assign89940_body42_e138450: f64 = (locals.var_exp_bps0 - locals.var_exp_bvbs);
        let assign89940_body42_e138451: f64 = (assign89940_body42_e138447 * assign89940_body42_e138450);
        (assign89940_body42_e138451, ((((locals.var_cnst1over_dn0 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn0)) * assign89940_body42_e138450) + (assign89940_body42_e138447 * (locals.var_exp_bps0_dn0 - locals.var_exp_bvbs_dn0))), ((((locals.var_cnst1over_dn2 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn2)) * assign89940_body42_e138450) + (assign89940_body42_e138447 * (locals.var_exp_bps0_dn2 - locals.var_exp_bvbs_dn2))), ((((locals.var_cnst1over_dn4 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn4)) * assign89940_body42_e138450) + (assign89940_body42_e138447 * (locals.var_exp_bps0_dn4 - locals.var_exp_bvbs_dn4))), ((((locals.var_cnst1over_dn5 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn5)) * assign89940_body42_e138450) + (assign89940_body42_e138447 * (locals.var_exp_bps0_dn5 - locals.var_exp_bvbs_dn5))), ((((locals.var_cnst1over_dn6 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn6)) * assign89940_body42_e138450) + (assign89940_body42_e138447 * (locals.var_exp_bps0_dn6 - locals.var_exp_bvbs_dn6))), ((((locals.var_cnst1over_dn7 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn7)) * assign89940_body42_e138450) + (assign89940_body42_e138447 * (locals.var_exp_bps0_dn7 - locals.var_exp_bvbs_dn7))), ((((locals.var_cnst1over_dn8 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn8)) * assign89940_body42_e138450) + (assign89940_body42_e138447 * (locals.var_exp_bps0_dn8 - locals.var_exp_bvbs_dn8))), ((((locals.var_cnst1over_dn9 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn9)) * assign89940_body42_e138450) + (assign89940_body42_e138447 * (locals.var_exp_bps0_dn9 - locals.var_exp_bvbs_dn9))), ((((locals.var_cnst1over_dn10 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn10)) * assign89940_body42_e138450) + (assign89940_body42_e138447 * (locals.var_exp_bps0_dn10 - locals.var_exp_bvbs_dn10))), ((((locals.var_cnst1over_dn11 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn11)) * assign89940_body42_e138450) + (assign89940_body42_e138447 * (locals.var_exp_bps0_dn11 - locals.var_exp_bvbs_dn11))), ((((locals.var_cnst1over_dn14 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn14)) * assign89940_body42_e138450) + (assign89940_body42_e138447 * (locals.var_exp_bps0_dn14 - locals.var_exp_bvbs_dn14))),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn11, locals.var_fs01_dps0_dn14,)
    }
};
            locals.var_fs01_dps0 = assign89940_body42_e138453;
            locals.var_fs01_dps0_dn0 = assign89940_body42_e138453_d_n0;
            locals.var_fs01_dps0_dn2 = assign89940_body42_e138453_d_n2;
            locals.var_fs01_dps0_dn4 = assign89940_body42_e138453_d_n4;
            locals.var_fs01_dps0_dn5 = assign89940_body42_e138453_d_n5;
            locals.var_fs01_dps0_dn6 = assign89940_body42_e138453_d_n6;
            locals.var_fs01_dps0_dn7 = assign89940_body42_e138453_d_n7;
            locals.var_fs01_dps0_dn8 = assign89940_body42_e138453_d_n8;
            locals.var_fs01_dps0_dn9 = assign89940_body42_e138453_d_n9;
            locals.var_fs01_dps0_dn10 = assign89940_body42_e138453_d_n10;
            locals.var_fs01_dps0_dn11 = assign89940_body42_e138453_d_n11;
            locals.var_fs01_dps0_dn14 = assign89940_body42_e138453_d_n14;
            locals.var_fs01_dps0_rv = 0.0;
            let assign89940_body43_e138456: f64 = if locals.var_fs01 > 0.0 { 1.0 } else { 0.0 };
            locals.var_guard2096 = assign89940_body43_e138456;
            locals.var_guard2096_rv = 0.0;
            let (assign89940_body44_e138467, assign89940_body44_e138467_d_n0, assign89940_body44_e138467_d_n2, assign89940_body44_e138467_d_n4, assign89940_body44_e138467_d_n5, assign89940_body44_e138467_d_n6, assign89940_body44_e138467_d_n7, assign89940_body44_e138467_d_n8, assign89940_body44_e138467_d_n9, assign89940_body44_e138467_d_n10, assign89940_body44_e138467_d_n11, assign89940_body44_e138467_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2087 != 0.0)) && (locals.var_guard2096 != 0.0)) {
        let assign89940_body44_e138464: f64 = (locals.var_fbsq__blk2014 + locals.var_fs01);
        let assign89940_body44_e138465: f64 = (assign89940_body44_e138464).sqrt();
        (assign89940_body44_e138465, ((locals.var_fbsq__blk2014_dn0 + locals.var_fs01_dn0) / (2.0 * assign89940_body44_e138465)), ((locals.var_fbsq__blk2014_dn2 + locals.var_fs01_dn2) / (2.0 * assign89940_body44_e138465)), ((locals.var_fbsq__blk2014_dn4 + locals.var_fs01_dn4) / (2.0 * assign89940_body44_e138465)), ((locals.var_fbsq__blk2014_dn5 + locals.var_fs01_dn5) / (2.0 * assign89940_body44_e138465)), ((locals.var_fbsq__blk2014_dn6 + locals.var_fs01_dn6) / (2.0 * assign89940_body44_e138465)), ((locals.var_fbsq__blk2014_dn7 + locals.var_fs01_dn7) / (2.0 * assign89940_body44_e138465)), ((locals.var_fbsq__blk2014_dn8 + locals.var_fs01_dn8) / (2.0 * assign89940_body44_e138465)), ((locals.var_fbsq__blk2014_dn9 + locals.var_fs01_dn9) / (2.0 * assign89940_body44_e138465)), ((locals.var_fbsq__blk2014_dn10 + locals.var_fs01_dn10) / (2.0 * assign89940_body44_e138465)), ((locals.var_fbsq__blk2014_dn11 + locals.var_fs01_dn11) / (2.0 * assign89940_body44_e138465)), ((locals.var_fbsq__blk2014_dn14 + locals.var_fs01_dn14) / (2.0 * assign89940_body44_e138465)),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn11, locals.var_fs02_dn14,)
    }
};
            locals.var_fs02 = assign89940_body44_e138467;
            locals.var_fs02_dn0 = assign89940_body44_e138467_d_n0;
            locals.var_fs02_dn2 = assign89940_body44_e138467_d_n2;
            locals.var_fs02_dn4 = assign89940_body44_e138467_d_n4;
            locals.var_fs02_dn5 = assign89940_body44_e138467_d_n5;
            locals.var_fs02_dn6 = assign89940_body44_e138467_d_n6;
            locals.var_fs02_dn7 = assign89940_body44_e138467_d_n7;
            locals.var_fs02_dn8 = assign89940_body44_e138467_d_n8;
            locals.var_fs02_dn9 = assign89940_body44_e138467_d_n9;
            locals.var_fs02_dn10 = assign89940_body44_e138467_d_n10;
            locals.var_fs02_dn11 = assign89940_body44_e138467_d_n11;
            locals.var_fs02_dn14 = assign89940_body44_e138467_d_n14;
            locals.var_fs02_rv = 0.0;
            let (assign89940_body45_e138481, assign89940_body45_e138481_d_n0, assign89940_body45_e138481_d_n2, assign89940_body45_e138481_d_n4, assign89940_body45_e138481_d_n5, assign89940_body45_e138481_d_n6, assign89940_body45_e138481_d_n7, assign89940_body45_e138481_d_n8, assign89940_body45_e138481_d_n9, assign89940_body45_e138481_d_n10, assign89940_body45_e138481_d_n11, assign89940_body45_e138481_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2087 != 0.0)) && (locals.var_guard2096 != 0.0)) {
        let assign89940_body45_e138476: f64 = (locals.var_fbsq_dpss__blk2015 + locals.var_fs01_dps0);
        let assign89940_body45_e138477: f64 = (0.5 * assign89940_body45_e138476);
        let assign89940_body45_e138479: f64 = (assign89940_body45_e138477 / locals.var_fs02);
        (assign89940_body45_e138479, ((((0.5 * (locals.var_fbsq_dpss__blk2015_dn0 + locals.var_fs01_dps0_dn0)) * locals.var_fs02) - (assign89940_body45_e138477 * locals.var_fs02_dn0)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk2015_dn2 + locals.var_fs01_dps0_dn2)) * locals.var_fs02) - (assign89940_body45_e138477 * locals.var_fs02_dn2)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk2015_dn4 + locals.var_fs01_dps0_dn4)) * locals.var_fs02) - (assign89940_body45_e138477 * locals.var_fs02_dn4)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk2015_dn5 + locals.var_fs01_dps0_dn5)) * locals.var_fs02) - (assign89940_body45_e138477 * locals.var_fs02_dn5)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk2015_dn6 + locals.var_fs01_dps0_dn6)) * locals.var_fs02) - (assign89940_body45_e138477 * locals.var_fs02_dn6)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk2015_dn7 + locals.var_fs01_dps0_dn7)) * locals.var_fs02) - (assign89940_body45_e138477 * locals.var_fs02_dn7)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk2015_dn8 + locals.var_fs01_dps0_dn8)) * locals.var_fs02) - (assign89940_body45_e138477 * locals.var_fs02_dn8)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk2015_dn9 + locals.var_fs01_dps0_dn9)) * locals.var_fs02) - (assign89940_body45_e138477 * locals.var_fs02_dn9)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk2015_dn10 + locals.var_fs01_dps0_dn10)) * locals.var_fs02) - (assign89940_body45_e138477 * locals.var_fs02_dn10)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk2015_dn11 + locals.var_fs01_dps0_dn11)) * locals.var_fs02) - (assign89940_body45_e138477 * locals.var_fs02_dn11)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk2015_dn14 + locals.var_fs01_dps0_dn14)) * locals.var_fs02) - (assign89940_body45_e138477 * locals.var_fs02_dn14)) / (locals.var_fs02 * locals.var_fs02)),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn11, locals.var_fs02_dps0_dn14,)
    }
};
            locals.var_fs02_dps0 = assign89940_body45_e138481;
            locals.var_fs02_dps0_dn0 = assign89940_body45_e138481_d_n0;
            locals.var_fs02_dps0_dn2 = assign89940_body45_e138481_d_n2;
            locals.var_fs02_dps0_dn4 = assign89940_body45_e138481_d_n4;
            locals.var_fs02_dps0_dn5 = assign89940_body45_e138481_d_n5;
            locals.var_fs02_dps0_dn6 = assign89940_body45_e138481_d_n6;
            locals.var_fs02_dps0_dn7 = assign89940_body45_e138481_d_n7;
            locals.var_fs02_dps0_dn8 = assign89940_body45_e138481_d_n8;
            locals.var_fs02_dps0_dn9 = assign89940_body45_e138481_d_n9;
            locals.var_fs02_dps0_dn10 = assign89940_body45_e138481_d_n10;
            locals.var_fs02_dps0_dn11 = assign89940_body45_e138481_d_n11;
            locals.var_fs02_dps0_dn14 = assign89940_body45_e138481_d_n14;
            locals.var_fs02_dps0_rv = 0.0;
            let assign89940_body46_e138484: f64 = if locals.var_fbsq__blk2014 > 0.0 { 1.0 } else { 0.0 };
            locals.var_guard2097 = assign89940_body46_e138484;
            locals.var_guard2097_rv = 0.0;
            let (assign89940_body47_e138496, assign89940_body47_e138496_d_n0, assign89940_body47_e138496_d_n2, assign89940_body47_e138496_d_n4, assign89940_body47_e138496_d_n5, assign89940_body47_e138496_d_n6, assign89940_body47_e138496_d_n7, assign89940_body47_e138496_d_n8, assign89940_body47_e138496_d_n9, assign89940_body47_e138496_d_n10, assign89940_body47_e138496_d_n11, assign89940_body47_e138496_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2087 != 0.0)) && (locals.var_guard2096 == 0.0)) && (locals.var_guard2097 != 0.0)) {
        let assign89940_body47_e138494: f64 = (locals.var_fbsq__blk2014).sqrt();
        (assign89940_body47_e138494, (locals.var_fbsq__blk2014_dn0 / (2.0 * assign89940_body47_e138494)), (locals.var_fbsq__blk2014_dn2 / (2.0 * assign89940_body47_e138494)), (locals.var_fbsq__blk2014_dn4 / (2.0 * assign89940_body47_e138494)), (locals.var_fbsq__blk2014_dn5 / (2.0 * assign89940_body47_e138494)), (locals.var_fbsq__blk2014_dn6 / (2.0 * assign89940_body47_e138494)), (locals.var_fbsq__blk2014_dn7 / (2.0 * assign89940_body47_e138494)), (locals.var_fbsq__blk2014_dn8 / (2.0 * assign89940_body47_e138494)), (locals.var_fbsq__blk2014_dn9 / (2.0 * assign89940_body47_e138494)), (locals.var_fbsq__blk2014_dn10 / (2.0 * assign89940_body47_e138494)), (locals.var_fbsq__blk2014_dn11 / (2.0 * assign89940_body47_e138494)), (locals.var_fbsq__blk2014_dn14 / (2.0 * assign89940_body47_e138494)),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn11, locals.var_fs02_dn14,)
    }
};
            locals.var_fs02 = assign89940_body47_e138496;
            locals.var_fs02_dn0 = assign89940_body47_e138496_d_n0;
            locals.var_fs02_dn2 = assign89940_body47_e138496_d_n2;
            locals.var_fs02_dn4 = assign89940_body47_e138496_d_n4;
            locals.var_fs02_dn5 = assign89940_body47_e138496_d_n5;
            locals.var_fs02_dn6 = assign89940_body47_e138496_d_n6;
            locals.var_fs02_dn7 = assign89940_body47_e138496_d_n7;
            locals.var_fs02_dn8 = assign89940_body47_e138496_d_n8;
            locals.var_fs02_dn9 = assign89940_body47_e138496_d_n9;
            locals.var_fs02_dn10 = assign89940_body47_e138496_d_n10;
            locals.var_fs02_dn11 = assign89940_body47_e138496_d_n11;
            locals.var_fs02_dn14 = assign89940_body47_e138496_d_n14;
            locals.var_fs02_rv = 0.0;
            let (assign89940_body48_e138511, assign89940_body48_e138511_d_n0, assign89940_body48_e138511_d_n2, assign89940_body48_e138511_d_n4, assign89940_body48_e138511_d_n5, assign89940_body48_e138511_d_n6, assign89940_body48_e138511_d_n7, assign89940_body48_e138511_d_n8, assign89940_body48_e138511_d_n9, assign89940_body48_e138511_d_n10, assign89940_body48_e138511_d_n11, assign89940_body48_e138511_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2087 != 0.0)) && (locals.var_guard2096 == 0.0)) && (locals.var_guard2097 != 0.0)) {
        let assign89940_body48_e138507: f64 = (0.5 * locals.var_fbsq_dpss__blk2015);
        let assign89940_body48_e138509: f64 = (assign89940_body48_e138507 / locals.var_fs02);
        (assign89940_body48_e138509, ((((0.5 * locals.var_fbsq_dpss__blk2015_dn0) * locals.var_fs02) - (assign89940_body48_e138507 * locals.var_fs02_dn0)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk2015_dn2) * locals.var_fs02) - (assign89940_body48_e138507 * locals.var_fs02_dn2)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk2015_dn4) * locals.var_fs02) - (assign89940_body48_e138507 * locals.var_fs02_dn4)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk2015_dn5) * locals.var_fs02) - (assign89940_body48_e138507 * locals.var_fs02_dn5)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk2015_dn6) * locals.var_fs02) - (assign89940_body48_e138507 * locals.var_fs02_dn6)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk2015_dn7) * locals.var_fs02) - (assign89940_body48_e138507 * locals.var_fs02_dn7)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk2015_dn8) * locals.var_fs02) - (assign89940_body48_e138507 * locals.var_fs02_dn8)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk2015_dn9) * locals.var_fs02) - (assign89940_body48_e138507 * locals.var_fs02_dn9)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk2015_dn10) * locals.var_fs02) - (assign89940_body48_e138507 * locals.var_fs02_dn10)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk2015_dn11) * locals.var_fs02) - (assign89940_body48_e138507 * locals.var_fs02_dn11)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk2015_dn14) * locals.var_fs02) - (assign89940_body48_e138507 * locals.var_fs02_dn14)) / (locals.var_fs02 * locals.var_fs02)),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn11, locals.var_fs02_dps0_dn14,)
    }
};
            locals.var_fs02_dps0 = assign89940_body48_e138511;
            locals.var_fs02_dps0_dn0 = assign89940_body48_e138511_d_n0;
            locals.var_fs02_dps0_dn2 = assign89940_body48_e138511_d_n2;
            locals.var_fs02_dps0_dn4 = assign89940_body48_e138511_d_n4;
            locals.var_fs02_dps0_dn5 = assign89940_body48_e138511_d_n5;
            locals.var_fs02_dps0_dn6 = assign89940_body48_e138511_d_n6;
            locals.var_fs02_dps0_dn7 = assign89940_body48_e138511_d_n7;
            locals.var_fs02_dps0_dn8 = assign89940_body48_e138511_d_n8;
            locals.var_fs02_dps0_dn9 = assign89940_body48_e138511_d_n9;
            locals.var_fs02_dps0_dn10 = assign89940_body48_e138511_d_n10;
            locals.var_fs02_dps0_dn11 = assign89940_body48_e138511_d_n11;
            locals.var_fs02_dps0_dn14 = assign89940_body48_e138511_d_n14;
            locals.var_fs02_dps0_rv = 0.0;
            let (assign89940_body49_e138523, assign89940_body49_e138523_d_n0, assign89940_body49_e138523_d_n2, assign89940_body49_e138523_d_n4, assign89940_body49_e138523_d_n5, assign89940_body49_e138523_d_n6, assign89940_body49_e138523_d_n7, assign89940_body49_e138523_d_n8, assign89940_body49_e138523_d_n9, assign89940_body49_e138523_d_n10, assign89940_body49_e138523_d_n11, assign89940_body49_e138523_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2087 != 0.0)) && (locals.var_guard2096 == 0.0)) && (locals.var_guard2097 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn11, locals.var_fs02_dn14,)
    }
};
            locals.var_fs02 = assign89940_body49_e138523;
            locals.var_fs02_dn0 = assign89940_body49_e138523_d_n0;
            locals.var_fs02_dn2 = assign89940_body49_e138523_d_n2;
            locals.var_fs02_dn4 = assign89940_body49_e138523_d_n4;
            locals.var_fs02_dn5 = assign89940_body49_e138523_d_n5;
            locals.var_fs02_dn6 = assign89940_body49_e138523_d_n6;
            locals.var_fs02_dn7 = assign89940_body49_e138523_d_n7;
            locals.var_fs02_dn8 = assign89940_body49_e138523_d_n8;
            locals.var_fs02_dn9 = assign89940_body49_e138523_d_n9;
            locals.var_fs02_dn10 = assign89940_body49_e138523_d_n10;
            locals.var_fs02_dn11 = assign89940_body49_e138523_d_n11;
            locals.var_fs02_dn14 = assign89940_body49_e138523_d_n14;
            locals.var_fs02_rv = 0.0;
            let (assign89940_body50_e138535, assign89940_body50_e138535_d_n0, assign89940_body50_e138535_d_n2, assign89940_body50_e138535_d_n4, assign89940_body50_e138535_d_n5, assign89940_body50_e138535_d_n6, assign89940_body50_e138535_d_n7, assign89940_body50_e138535_d_n8, assign89940_body50_e138535_d_n9, assign89940_body50_e138535_d_n10, assign89940_body50_e138535_d_n11, assign89940_body50_e138535_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2087 != 0.0)) && (locals.var_guard2096 == 0.0)) && (locals.var_guard2097 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn11, locals.var_fs02_dps0_dn14,)
    }
};
            locals.var_fs02_dps0 = assign89940_body50_e138535;
            locals.var_fs02_dps0_dn0 = assign89940_body50_e138535_d_n0;
            locals.var_fs02_dps0_dn2 = assign89940_body50_e138535_d_n2;
            locals.var_fs02_dps0_dn4 = assign89940_body50_e138535_d_n4;
            locals.var_fs02_dps0_dn5 = assign89940_body50_e138535_d_n5;
            locals.var_fs02_dps0_dn6 = assign89940_body50_e138535_d_n6;
            locals.var_fs02_dps0_dn7 = assign89940_body50_e138535_d_n7;
            locals.var_fs02_dps0_dn8 = assign89940_body50_e138535_d_n8;
            locals.var_fs02_dps0_dn9 = assign89940_body50_e138535_d_n9;
            locals.var_fs02_dps0_dn10 = assign89940_body50_e138535_d_n10;
            locals.var_fs02_dps0_dn11 = assign89940_body50_e138535_d_n11;
            locals.var_fs02_dps0_dn14 = assign89940_body50_e138535_d_n14;
            locals.var_fs02_dps0_rv = 0.0;
            let (assign89940_body51_e138549, assign89940_body51_e138549_d_n0, assign89940_body51_e138549_d_n2, assign89940_body51_e138549_d_n4, assign89940_body51_e138549_d_n5, assign89940_body51_e138549_d_n6, assign89940_body51_e138549_d_n7, assign89940_body51_e138549_d_n8, assign89940_body51_e138549_d_n9, assign89940_body51_e138549_d_n10, assign89940_body51_e138549_d_n11, assign89940_body51_e138549_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2087 != 0.0)) {
        let (assign89940_body51_e138545,) = {
            if (locals.var_chi >= 0.0) {
                (1.0,)
            } else {
                let assign89940_body51_e138544: f64 = (-1.0);
                (assign89940_body51_e138544,)
            }
        };
        let assign89940_body51_e138547: f64 = (assign89940_body51_e138545 * locals.var_fs02);
        (assign89940_body51_e138547, (assign89940_body51_e138545 * locals.var_fs02_dn0), (assign89940_body51_e138545 * locals.var_fs02_dn2), (assign89940_body51_e138545 * locals.var_fs02_dn4), (assign89940_body51_e138545 * locals.var_fs02_dn5), (assign89940_body51_e138545 * locals.var_fs02_dn6), (assign89940_body51_e138545 * locals.var_fs02_dn7), (assign89940_body51_e138545 * locals.var_fs02_dn8), (assign89940_body51_e138545 * locals.var_fs02_dn9), (assign89940_body51_e138545 * locals.var_fs02_dn10), (assign89940_body51_e138545 * locals.var_fs02_dn11), (assign89940_body51_e138545 * locals.var_fs02_dn14),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn11, locals.var_fs02_dn14,)
    }
};
            locals.var_fs02 = assign89940_body51_e138549;
            locals.var_fs02_dn0 = assign89940_body51_e138549_d_n0;
            locals.var_fs02_dn2 = assign89940_body51_e138549_d_n2;
            locals.var_fs02_dn4 = assign89940_body51_e138549_d_n4;
            locals.var_fs02_dn5 = assign89940_body51_e138549_d_n5;
            locals.var_fs02_dn6 = assign89940_body51_e138549_d_n6;
            locals.var_fs02_dn7 = assign89940_body51_e138549_d_n7;
            locals.var_fs02_dn8 = assign89940_body51_e138549_d_n8;
            locals.var_fs02_dn9 = assign89940_body51_e138549_d_n9;
            locals.var_fs02_dn10 = assign89940_body51_e138549_d_n10;
            locals.var_fs02_dn11 = assign89940_body51_e138549_d_n11;
            locals.var_fs02_dn14 = assign89940_body51_e138549_d_n14;
            locals.var_fs02_rv = 0.0;
            let (assign89940_body52_e138563, assign89940_body52_e138563_d_n0, assign89940_body52_e138563_d_n2, assign89940_body52_e138563_d_n4, assign89940_body52_e138563_d_n5, assign89940_body52_e138563_d_n6, assign89940_body52_e138563_d_n7, assign89940_body52_e138563_d_n8, assign89940_body52_e138563_d_n9, assign89940_body52_e138563_d_n10, assign89940_body52_e138563_d_n11, assign89940_body52_e138563_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2087 != 0.0)) {
        let (assign89940_body52_e138559,) = {
            if (locals.var_chi >= 0.0) {
                (1.0,)
            } else {
                let assign89940_body52_e138558: f64 = (-1.0);
                (assign89940_body52_e138558,)
            }
        };
        let assign89940_body52_e138561: f64 = (assign89940_body52_e138559 * locals.var_fs02_dps0);
        (assign89940_body52_e138561, (assign89940_body52_e138559 * locals.var_fs02_dps0_dn0), (assign89940_body52_e138559 * locals.var_fs02_dps0_dn2), (assign89940_body52_e138559 * locals.var_fs02_dps0_dn4), (assign89940_body52_e138559 * locals.var_fs02_dps0_dn5), (assign89940_body52_e138559 * locals.var_fs02_dps0_dn6), (assign89940_body52_e138559 * locals.var_fs02_dps0_dn7), (assign89940_body52_e138559 * locals.var_fs02_dps0_dn8), (assign89940_body52_e138559 * locals.var_fs02_dps0_dn9), (assign89940_body52_e138559 * locals.var_fs02_dps0_dn10), (assign89940_body52_e138559 * locals.var_fs02_dps0_dn11), (assign89940_body52_e138559 * locals.var_fs02_dps0_dn14),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn11, locals.var_fs02_dps0_dn14,)
    }
};
            locals.var_fs02_dps0 = assign89940_body52_e138563;
            locals.var_fs02_dps0_dn0 = assign89940_body52_e138563_d_n0;
            locals.var_fs02_dps0_dn2 = assign89940_body52_e138563_d_n2;
            locals.var_fs02_dps0_dn4 = assign89940_body52_e138563_d_n4;
            locals.var_fs02_dps0_dn5 = assign89940_body52_e138563_d_n5;
            locals.var_fs02_dps0_dn6 = assign89940_body52_e138563_d_n6;
            locals.var_fs02_dps0_dn7 = assign89940_body52_e138563_d_n7;
            locals.var_fs02_dps0_dn8 = assign89940_body52_e138563_d_n8;
            locals.var_fs02_dps0_dn9 = assign89940_body52_e138563_d_n9;
            locals.var_fs02_dps0_dn10 = assign89940_body52_e138563_d_n10;
            locals.var_fs02_dps0_dn11 = assign89940_body52_e138563_d_n11;
            locals.var_fs02_dps0_dn14 = assign89940_body52_e138563_d_n14;
            locals.var_fs02_dps0_rv = 0.0;
            let (assign89940_body53_e138576, assign89940_body53_e138576_d_n0, assign89940_body53_e138576_d_n2, assign89940_body53_e138576_d_n4, assign89940_body53_e138576_d_n5, assign89940_body53_e138576_d_n6, assign89940_body53_e138576_d_n7, assign89940_body53_e138576_d_n8, assign89940_body53_e138576_d_n9, assign89940_body53_e138576_d_n10, assign89940_body53_e138576_d_n11, assign89940_body53_e138576_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2087 != 0.0)) {
        let assign89940_body53_e138568: f64 = (-locals.var_vgpld);
        let assign89940_body53_e138570: f64 = (assign89940_body53_e138568 + locals.var_ps0ld);
        let assign89940_body53_e138573: f64 = (locals.var_fac1 * locals.var_fs02);
        let assign89940_body53_e138574: f64 = (assign89940_body53_e138570 + assign89940_body53_e138573);
        (assign89940_body53_e138574, (locals.var_ps0ld_dn0 + ((locals.var_fac1_dn0 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn0))), (((-locals.var_vgpld_dn2) + locals.var_ps0ld_dn2) + ((locals.var_fac1_dn2 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn2))), (locals.var_ps0ld_dn4 + ((locals.var_fac1_dn4 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn4))), (locals.var_ps0ld_dn5 + ((locals.var_fac1_dn5 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn5))), (locals.var_ps0ld_dn6 + ((locals.var_fac1_dn6 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn6))), (((-locals.var_vgpld_dn7) + locals.var_ps0ld_dn7) + ((locals.var_fac1_dn7 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn7))), (((-locals.var_vgpld_dn8) + locals.var_ps0ld_dn8) + ((locals.var_fac1_dn8 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn8))), (((-locals.var_vgpld_dn9) + locals.var_ps0ld_dn9) + ((locals.var_fac1_dn9 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn9))), (locals.var_ps0ld_dn10 + ((locals.var_fac1_dn10 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn10))), (locals.var_ps0ld_dn11 + ((locals.var_fac1_dn11 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn11))), (locals.var_ps0ld_dn14 + ((locals.var_fac1_dn14 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn14))),)
    } else {
        (locals.var_fs0, locals.var_fs0_dn0, locals.var_fs0_dn2, locals.var_fs0_dn4, locals.var_fs0_dn5, locals.var_fs0_dn6, locals.var_fs0_dn7, locals.var_fs0_dn8, locals.var_fs0_dn9, locals.var_fs0_dn10, locals.var_fs0_dn11, locals.var_fs0_dn14,)
    }
};
            locals.var_fs0 = assign89940_body53_e138576;
            locals.var_fs0_dn0 = assign89940_body53_e138576_d_n0;
            locals.var_fs0_dn2 = assign89940_body53_e138576_d_n2;
            locals.var_fs0_dn4 = assign89940_body53_e138576_d_n4;
            locals.var_fs0_dn5 = assign89940_body53_e138576_d_n5;
            locals.var_fs0_dn6 = assign89940_body53_e138576_d_n6;
            locals.var_fs0_dn7 = assign89940_body53_e138576_d_n7;
            locals.var_fs0_dn8 = assign89940_body53_e138576_d_n8;
            locals.var_fs0_dn9 = assign89940_body53_e138576_d_n9;
            locals.var_fs0_dn10 = assign89940_body53_e138576_d_n10;
            locals.var_fs0_dn11 = assign89940_body53_e138576_d_n11;
            locals.var_fs0_dn14 = assign89940_body53_e138576_d_n14;
            locals.var_fs0_rv = 0.0;
            let (assign89940_body54_e138586, assign89940_body54_e138586_d_n0, assign89940_body54_e138586_d_n2, assign89940_body54_e138586_d_n4, assign89940_body54_e138586_d_n5, assign89940_body54_e138586_d_n6, assign89940_body54_e138586_d_n7, assign89940_body54_e138586_d_n8, assign89940_body54_e138586_d_n9, assign89940_body54_e138586_d_n10, assign89940_body54_e138586_d_n11, assign89940_body54_e138586_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2087 != 0.0)) {
        let assign89940_body54_e138583: f64 = (locals.var_fac1 * locals.var_fs02_dps0);
        let assign89940_body54_e138584: f64 = (1.0 + assign89940_body54_e138583);
        (assign89940_body54_e138584, ((locals.var_fac1_dn0 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn0)), ((locals.var_fac1_dn2 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn2)), ((locals.var_fac1_dn4 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn4)), ((locals.var_fac1_dn5 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn5)), ((locals.var_fac1_dn6 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn6)), ((locals.var_fac1_dn7 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn7)), ((locals.var_fac1_dn8 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn8)), ((locals.var_fac1_dn9 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn9)), ((locals.var_fac1_dn10 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn10)), ((locals.var_fac1_dn11 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn11)), ((locals.var_fac1_dn14 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn14)),)
    } else {
        (locals.var_fs0_dps0, locals.var_fs0_dps0_dn0, locals.var_fs0_dps0_dn2, locals.var_fs0_dps0_dn4, locals.var_fs0_dps0_dn5, locals.var_fs0_dps0_dn6, locals.var_fs0_dps0_dn7, locals.var_fs0_dps0_dn8, locals.var_fs0_dps0_dn9, locals.var_fs0_dps0_dn10, locals.var_fs0_dps0_dn11, locals.var_fs0_dps0_dn14,)
    }
};
            locals.var_fs0_dps0 = assign89940_body54_e138586;
            locals.var_fs0_dps0_dn0 = assign89940_body54_e138586_d_n0;
            locals.var_fs0_dps0_dn2 = assign89940_body54_e138586_d_n2;
            locals.var_fs0_dps0_dn4 = assign89940_body54_e138586_d_n4;
            locals.var_fs0_dps0_dn5 = assign89940_body54_e138586_d_n5;
            locals.var_fs0_dps0_dn6 = assign89940_body54_e138586_d_n6;
            locals.var_fs0_dps0_dn7 = assign89940_body54_e138586_d_n7;
            locals.var_fs0_dps0_dn8 = assign89940_body54_e138586_d_n8;
            locals.var_fs0_dps0_dn9 = assign89940_body54_e138586_d_n9;
            locals.var_fs0_dps0_dn10 = assign89940_body54_e138586_d_n10;
            locals.var_fs0_dps0_dn11 = assign89940_body54_e138586_d_n11;
            locals.var_fs0_dps0_dn14 = assign89940_body54_e138586_d_n14;
            locals.var_fs0_dps0_rv = 0.0;
            let assign89940_body55_e138589: f64 = if locals.var_flg_conv > 0.0 { 1.0 } else { 0.0 };
            locals.var_guard2098 = assign89940_body55_e138589;
            locals.var_guard2098_rv = 0.0;
            let (assign89940_body56_e138599,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2087 != 0.0)) && (locals.var_guard2098 != 0.0)) {
        let assign89940_body56_e138597: f64 = (locals.var_lp_s0_max + 1.0);
        (assign89940_body56_e138597,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign89940_body56_e138599;
            locals.var_lp_s0_rv = 0.0;
            let (assign89940_body57_e138611, assign89940_body57_e138611_d_n0, assign89940_body57_e138611_d_n2, assign89940_body57_e138611_d_n4, assign89940_body57_e138611_d_n5, assign89940_body57_e138611_d_n6, assign89940_body57_e138611_d_n7, assign89940_body57_e138611_d_n8, assign89940_body57_e138611_d_n9, assign89940_body57_e138611_d_n10, assign89940_body57_e138611_d_n11, assign89940_body57_e138611_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2087 != 0.0)) && (locals.var_guard2098 == 0.0)) {
        let assign89940_body57_e138607: f64 = (-locals.var_fs0);
        let assign89940_body57_e138609: f64 = (assign89940_body57_e138607 / locals.var_fs0_dps0);
        (assign89940_body57_e138609, ((((-locals.var_fs0_dn0) * locals.var_fs0_dps0) - (assign89940_body57_e138607 * locals.var_fs0_dps0_dn0)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn2) * locals.var_fs0_dps0) - (assign89940_body57_e138607 * locals.var_fs0_dps0_dn2)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn4) * locals.var_fs0_dps0) - (assign89940_body57_e138607 * locals.var_fs0_dps0_dn4)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn5) * locals.var_fs0_dps0) - (assign89940_body57_e138607 * locals.var_fs0_dps0_dn5)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn6) * locals.var_fs0_dps0) - (assign89940_body57_e138607 * locals.var_fs0_dps0_dn6)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn7) * locals.var_fs0_dps0) - (assign89940_body57_e138607 * locals.var_fs0_dps0_dn7)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn8) * locals.var_fs0_dps0) - (assign89940_body57_e138607 * locals.var_fs0_dps0_dn8)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn9) * locals.var_fs0_dps0) - (assign89940_body57_e138607 * locals.var_fs0_dps0_dn9)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn10) * locals.var_fs0_dps0) - (assign89940_body57_e138607 * locals.var_fs0_dps0_dn10)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn11) * locals.var_fs0_dps0) - (assign89940_body57_e138607 * locals.var_fs0_dps0_dn11)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn14) * locals.var_fs0_dps0) - (assign89940_body57_e138607 * locals.var_fs0_dps0_dn14)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn4, locals.var_dps0_dn5, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn8, locals.var_dps0_dn9, locals.var_dps0_dn10, locals.var_dps0_dn11, locals.var_dps0_dn14,)
    }
};
            locals.var_dps0 = assign89940_body57_e138611;
            locals.var_dps0_dn0 = assign89940_body57_e138611_d_n0;
            locals.var_dps0_dn2 = assign89940_body57_e138611_d_n2;
            locals.var_dps0_dn4 = assign89940_body57_e138611_d_n4;
            locals.var_dps0_dn5 = assign89940_body57_e138611_d_n5;
            locals.var_dps0_dn6 = assign89940_body57_e138611_d_n6;
            locals.var_dps0_dn7 = assign89940_body57_e138611_d_n7;
            locals.var_dps0_dn8 = assign89940_body57_e138611_d_n8;
            locals.var_dps0_dn9 = assign89940_body57_e138611_d_n9;
            locals.var_dps0_dn10 = assign89940_body57_e138611_d_n10;
            locals.var_dps0_dn11 = assign89940_body57_e138611_d_n11;
            locals.var_dps0_dn14 = assign89940_body57_e138611_d_n14;
            locals.var_dps0_rv = 0.0;
            let (assign89940_body58_e138633, assign89940_body58_e138633_d_n0, assign89940_body58_e138633_d_n2, assign89940_body58_e138633_d_n4, assign89940_body58_e138633_d_n5, assign89940_body58_e138633_d_n6, assign89940_body58_e138633_d_n7, assign89940_body58_e138633_d_n8, assign89940_body58_e138633_d_n9, assign89940_body58_e138633_d_n10, assign89940_body58_e138633_d_n11, assign89940_body58_e138633_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2087 != 0.0)) && (locals.var_guard2098 == 0.0)) {
        let assign89940_body58_e138620: f64 = (0.5 * 0.1);
        let assign89940_body58_e138624: f64 = (locals.var_ps0ld).abs();
        let (assign89940_body58_e138629, assign89940_body58_e138629_d_n0, assign89940_body58_e138629_d_n2, assign89940_body58_e138629_d_n4, assign89940_body58_e138629_d_n5, assign89940_body58_e138629_d_n6, assign89940_body58_e138629_d_n7, assign89940_body58_e138629_d_n8, assign89940_body58_e138629_d_n9, assign89940_body58_e138629_d_n10, assign89940_body58_e138629_d_n11, assign89940_body58_e138629_d_n14,) = {
            if (1.0 >= assign89940_body58_e138624) {
                (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign89940_body58_e138628: f64 = (locals.var_ps0ld).abs();
                (assign89940_body58_e138628, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn0 } else { (-locals.var_ps0ld_dn0) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn2 } else { (-locals.var_ps0ld_dn2) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn4 } else { (-locals.var_ps0ld_dn4) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn5 } else { (-locals.var_ps0ld_dn5) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn6 } else { (-locals.var_ps0ld_dn6) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn7 } else { (-locals.var_ps0ld_dn7) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn8 } else { (-locals.var_ps0ld_dn8) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn9 } else { (-locals.var_ps0ld_dn9) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn10 } else { (-locals.var_ps0ld_dn10) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn11 } else { (-locals.var_ps0ld_dn11) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn14 } else { (-locals.var_ps0ld_dn14) },)
            }
        };
        let assign89940_body58_e138630: f64 = (1.0 + assign89940_body58_e138629);
        let assign89940_body58_e138631: f64 = (assign89940_body58_e138620 * assign89940_body58_e138630);
        (assign89940_body58_e138631, (assign89940_body58_e138620 * assign89940_body58_e138629_d_n0), (assign89940_body58_e138620 * assign89940_body58_e138629_d_n2), (assign89940_body58_e138620 * assign89940_body58_e138629_d_n4), (assign89940_body58_e138620 * assign89940_body58_e138629_d_n5), (assign89940_body58_e138620 * assign89940_body58_e138629_d_n6), (assign89940_body58_e138620 * assign89940_body58_e138629_d_n7), (assign89940_body58_e138620 * assign89940_body58_e138629_d_n8), (assign89940_body58_e138620 * assign89940_body58_e138629_d_n9), (assign89940_body58_e138620 * assign89940_body58_e138629_d_n10), (assign89940_body58_e138620 * assign89940_body58_e138629_d_n11), (assign89940_body58_e138620 * assign89940_body58_e138629_d_n14),)
    } else {
        (locals.var_dplim, locals.var_dplim_dn0, locals.var_dplim_dn2, locals.var_dplim_dn4, locals.var_dplim_dn5, locals.var_dplim_dn6, locals.var_dplim_dn7, locals.var_dplim_dn8, locals.var_dplim_dn9, locals.var_dplim_dn10, locals.var_dplim_dn11, locals.var_dplim_dn14,)
    }
};
            locals.var_dplim = assign89940_body58_e138633;
            locals.var_dplim_dn0 = assign89940_body58_e138633_d_n0;
            locals.var_dplim_dn2 = assign89940_body58_e138633_d_n2;
            locals.var_dplim_dn4 = assign89940_body58_e138633_d_n4;
            locals.var_dplim_dn5 = assign89940_body58_e138633_d_n5;
            locals.var_dplim_dn6 = assign89940_body58_e138633_d_n6;
            locals.var_dplim_dn7 = assign89940_body58_e138633_d_n7;
            locals.var_dplim_dn8 = assign89940_body58_e138633_d_n8;
            locals.var_dplim_dn9 = assign89940_body58_e138633_d_n9;
            locals.var_dplim_dn10 = assign89940_body58_e138633_d_n10;
            locals.var_dplim_dn11 = assign89940_body58_e138633_d_n11;
            locals.var_dplim_dn14 = assign89940_body58_e138633_d_n14;
            locals.var_dplim_rv = 0.0;
            let assign89940_body59_e138635: f64 = (locals.var_dps0).abs();
            let assign89940_body59_e138637: f64 = if assign89940_body59_e138635 > locals.var_dplim { 1.0 } else { 0.0 };
            locals.var_guard2099 = assign89940_body59_e138637;
            locals.var_guard2099_rv = 0.0;
            let (assign89940_body60_e138656, assign89940_body60_e138656_d_n0, assign89940_body60_e138656_d_n2, assign89940_body60_e138656_d_n4, assign89940_body60_e138656_d_n5, assign89940_body60_e138656_d_n6, assign89940_body60_e138656_d_n7, assign89940_body60_e138656_d_n8, assign89940_body60_e138656_d_n9, assign89940_body60_e138656_d_n10, assign89940_body60_e138656_d_n11, assign89940_body60_e138656_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2087 != 0.0)) && (locals.var_guard2098 == 0.0)) && (locals.var_guard2099 != 0.0)) {
        let (assign89940_body60_e138653,) = {
            if (locals.var_dps0 >= 0.0) {
                (1.0,)
            } else {
                let assign89940_body60_e138652: f64 = (-1.0);
                (assign89940_body60_e138652,)
            }
        };
        let assign89940_body60_e138654: f64 = (locals.var_dplim * assign89940_body60_e138653);
        (assign89940_body60_e138654, (locals.var_dplim_dn0 * assign89940_body60_e138653), (locals.var_dplim_dn2 * assign89940_body60_e138653), (locals.var_dplim_dn4 * assign89940_body60_e138653), (locals.var_dplim_dn5 * assign89940_body60_e138653), (locals.var_dplim_dn6 * assign89940_body60_e138653), (locals.var_dplim_dn7 * assign89940_body60_e138653), (locals.var_dplim_dn8 * assign89940_body60_e138653), (locals.var_dplim_dn9 * assign89940_body60_e138653), (locals.var_dplim_dn10 * assign89940_body60_e138653), (locals.var_dplim_dn11 * assign89940_body60_e138653), (locals.var_dplim_dn14 * assign89940_body60_e138653),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn4, locals.var_dps0_dn5, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn8, locals.var_dps0_dn9, locals.var_dps0_dn10, locals.var_dps0_dn11, locals.var_dps0_dn14,)
    }
};
            locals.var_dps0 = assign89940_body60_e138656;
            locals.var_dps0_dn0 = assign89940_body60_e138656_d_n0;
            locals.var_dps0_dn2 = assign89940_body60_e138656_d_n2;
            locals.var_dps0_dn4 = assign89940_body60_e138656_d_n4;
            locals.var_dps0_dn5 = assign89940_body60_e138656_d_n5;
            locals.var_dps0_dn6 = assign89940_body60_e138656_d_n6;
            locals.var_dps0_dn7 = assign89940_body60_e138656_d_n7;
            locals.var_dps0_dn8 = assign89940_body60_e138656_d_n8;
            locals.var_dps0_dn9 = assign89940_body60_e138656_d_n9;
            locals.var_dps0_dn10 = assign89940_body60_e138656_d_n10;
            locals.var_dps0_dn11 = assign89940_body60_e138656_d_n11;
            locals.var_dps0_dn14 = assign89940_body60_e138656_d_n14;
            locals.var_dps0_rv = 0.0;
            let (assign89940_body61_e138667, assign89940_body61_e138667_d_n0, assign89940_body61_e138667_d_n2, assign89940_body61_e138667_d_n4, assign89940_body61_e138667_d_n5, assign89940_body61_e138667_d_n6, assign89940_body61_e138667_d_n7, assign89940_body61_e138667_d_n8, assign89940_body61_e138667_d_n9, assign89940_body61_e138667_d_n10, assign89940_body61_e138667_d_n11, assign89940_body61_e138667_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2087 != 0.0)) && (locals.var_guard2098 == 0.0)) {
        let assign89940_body61_e138665: f64 = (locals.var_ps0ld + locals.var_dps0);
        (assign89940_body61_e138665, (locals.var_ps0ld_dn0 + locals.var_dps0_dn0), (locals.var_ps0ld_dn2 + locals.var_dps0_dn2), (locals.var_ps0ld_dn4 + locals.var_dps0_dn4), (locals.var_ps0ld_dn5 + locals.var_dps0_dn5), (locals.var_ps0ld_dn6 + locals.var_dps0_dn6), (locals.var_ps0ld_dn7 + locals.var_dps0_dn7), (locals.var_ps0ld_dn8 + locals.var_dps0_dn8), (locals.var_ps0ld_dn9 + locals.var_dps0_dn9), (locals.var_ps0ld_dn10 + locals.var_dps0_dn10), (locals.var_ps0ld_dn11 + locals.var_dps0_dn11), (locals.var_ps0ld_dn14 + locals.var_dps0_dn14),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn14,)
    }
};
            locals.var_ps0ld = assign89940_body61_e138667;
            locals.var_ps0ld_dn0 = assign89940_body61_e138667_d_n0;
            locals.var_ps0ld_dn2 = assign89940_body61_e138667_d_n2;
            locals.var_ps0ld_dn4 = assign89940_body61_e138667_d_n4;
            locals.var_ps0ld_dn5 = assign89940_body61_e138667_d_n5;
            locals.var_ps0ld_dn6 = assign89940_body61_e138667_d_n6;
            locals.var_ps0ld_dn7 = assign89940_body61_e138667_d_n7;
            locals.var_ps0ld_dn8 = assign89940_body61_e138667_d_n8;
            locals.var_ps0ld_dn9 = assign89940_body61_e138667_d_n9;
            locals.var_ps0ld_dn10 = assign89940_body61_e138667_d_n10;
            locals.var_ps0ld_dn11 = assign89940_body61_e138667_d_n11;
            locals.var_ps0ld_dn14 = assign89940_body61_e138667_d_n14;
            locals.var_ps0ld_rv = 0.0;
            let assign89940_body62_e138669: f64 = (locals.var_dps0).abs();
            let assign89940_body62_e138673: f64 = (locals.var_fs0).abs();
            let assign89940_body62_e138676: f64 = if ((assign89940_body62_e138669 <= 1e-12) && (assign89940_body62_e138673 <= 1e-8)) { 1.0 } else { 0.0 };
            locals.var_guard2100 = assign89940_body62_e138676;
            locals.var_guard2100_rv = 0.0;
            let (assign89940_body63_e138689,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2087 != 0.0)) && (locals.var_guard2098 == 0.0)) && (locals.var_guard2100 != 0.0)) {
        let assign89940_body63_e138687: f64 = (locals.var_flg_conv + 2.0);
        (assign89940_body63_e138687,)
    } else {
        (locals.var_flg_conv,)
    }
};
            locals.var_flg_conv = assign89940_body63_e138689;
            locals.var_flg_conv_rv = 0.0;
            let (assign89940_body64_e138697,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2087 != 0.0)) {
        let assign89940_body64_e138695: f64 = (locals.var_lp_s0 + 1.0);
        (assign89940_body64_e138695,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign89940_body64_e138697;
            locals.var_lp_s0_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_347(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign89960_e138720, assign89960_e138720_d_n0, assign89960_e138720_d_n2, assign89960_e138720_d_n4, assign89960_e138720_d_n5, assign89960_e138720_d_n6, assign89960_e138720_d_n7, assign89960_e138720_d_n8, assign89960_e138720_d_n9, assign89960_e138720_d_n10, assign89960_e138720_d_n11, assign89960_e138720_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2087 != 0.0)) {
        let (assign89960_e138718, assign89960_e138718_d_n0, assign89960_e138718_d_n2, assign89960_e138718_d_n4, assign89960_e138718_d_n5, assign89960_e138718_d_n6, assign89960_e138718_d_n7, assign89960_e138718_d_n8, assign89960_e138718_d_n9, assign89960_e138718_d_n10, assign89960_e138718_d_n11, assign89960_e138718_d_n14,) = {
            if (locals.var_fbsq__blk2014 >= 0.0) {
                let (assign89960_e138713,) = {
                    if (locals.var_chi >= 0.0) {
                        (1.0,)
                    } else {
                        let assign89960_e138712: f64 = (-1.0);
                        (assign89960_e138712,)
                    }
                };
                let assign89960_e138715: f64 = (locals.var_fbsq__blk2014).sqrt();
                let assign89960_e138716: f64 = (assign89960_e138713 * assign89960_e138715);
                (assign89960_e138716, (assign89960_e138713 * (locals.var_fbsq__blk2014_dn0 / (2.0 * assign89960_e138715))), (assign89960_e138713 * (locals.var_fbsq__blk2014_dn2 / (2.0 * assign89960_e138715))), (assign89960_e138713 * (locals.var_fbsq__blk2014_dn4 / (2.0 * assign89960_e138715))), (assign89960_e138713 * (locals.var_fbsq__blk2014_dn5 / (2.0 * assign89960_e138715))), (assign89960_e138713 * (locals.var_fbsq__blk2014_dn6 / (2.0 * assign89960_e138715))), (assign89960_e138713 * (locals.var_fbsq__blk2014_dn7 / (2.0 * assign89960_e138715))), (assign89960_e138713 * (locals.var_fbsq__blk2014_dn8 / (2.0 * assign89960_e138715))), (assign89960_e138713 * (locals.var_fbsq__blk2014_dn9 / (2.0 * assign89960_e138715))), (assign89960_e138713 * (locals.var_fbsq__blk2014_dn10 / (2.0 * assign89960_e138715))), (assign89960_e138713 * (locals.var_fbsq__blk2014_dn11 / (2.0 * assign89960_e138715))), (assign89960_e138713 * (locals.var_fbsq__blk2014_dn14 / (2.0 * assign89960_e138715))),)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign89960_e138718, assign89960_e138718_d_n0, assign89960_e138718_d_n2, assign89960_e138718_d_n4, assign89960_e138718_d_n5, assign89960_e138718_d_n6, assign89960_e138718_d_n7, assign89960_e138718_d_n8, assign89960_e138718_d_n9, assign89960_e138718_d_n10, assign89960_e138718_d_n11, assign89960_e138718_d_n14,)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn14,)
    }
};
        locals.var_fb = assign89960_e138720;
        locals.var_fb_dn0 = assign89960_e138720_d_n0;
        locals.var_fb_dn2 = assign89960_e138720_d_n2;
        locals.var_fb_dn4 = assign89960_e138720_d_n4;
        locals.var_fb_dn5 = assign89960_e138720_d_n5;
        locals.var_fb_dn6 = assign89960_e138720_d_n6;
        locals.var_fb_dn7 = assign89960_e138720_d_n7;
        locals.var_fb_dn8 = assign89960_e138720_d_n8;
        locals.var_fb_dn9 = assign89960_e138720_d_n9;
        locals.var_fb_dn10 = assign89960_e138720_d_n10;
        locals.var_fb_dn11 = assign89960_e138720_d_n11;
        locals.var_fb_dn14 = assign89960_e138720_d_n14;
        locals.var_fb_rv = 0.0;

        let (assign89970_e138728, assign89970_e138728_d_n0, assign89970_e138728_d_n2, assign89970_e138728_d_n4, assign89970_e138728_d_n5, assign89970_e138728_d_n6, assign89970_e138728_d_n7, assign89970_e138728_d_n8, assign89970_e138728_d_n9, assign89970_e138728_d_n10, assign89970_e138728_d_n11, assign89970_e138728_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2087 != 0.0)) {
        let assign89970_e138726: f64 = (locals.var_c_w_ld * locals.var_fb);
        (assign89970_e138726, ((locals.var_c_w_ld_dn0 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn0)), ((locals.var_c_w_ld_dn2 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn2)), ((locals.var_c_w_ld_dn4 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn4)), ((locals.var_c_w_ld_dn5 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn5)), ((locals.var_c_w_ld_dn6 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn6)), ((locals.var_c_w_ld_dn7 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn7)), ((locals.var_c_w_ld_dn8 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn8)), ((locals.var_c_w_ld_dn9 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn9)), ((locals.var_c_w_ld_dn10 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn10)), ((locals.var_c_w_ld_dn11 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn11)), ((locals.var_c_w_ld_dn14 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn14)),)
    } else {
        (locals.var_wdld__blk2004, locals.var_wdld__blk2004_dn0, locals.var_wdld__blk2004_dn2, locals.var_wdld__blk2004_dn4, locals.var_wdld__blk2004_dn5, locals.var_wdld__blk2004_dn6, locals.var_wdld__blk2004_dn7, locals.var_wdld__blk2004_dn8, locals.var_wdld__blk2004_dn9, locals.var_wdld__blk2004_dn10, locals.var_wdld__blk2004_dn11, locals.var_wdld__blk2004_dn14,)
    }
};
        locals.var_wdld__blk2004 = assign89970_e138728;
        locals.var_wdld__blk2004_dn0 = assign89970_e138728_d_n0;
        locals.var_wdld__blk2004_dn2 = assign89970_e138728_d_n2;
        locals.var_wdld__blk2004_dn4 = assign89970_e138728_d_n4;
        locals.var_wdld__blk2004_dn5 = assign89970_e138728_d_n5;
        locals.var_wdld__blk2004_dn6 = assign89970_e138728_d_n6;
        locals.var_wdld__blk2004_dn7 = assign89970_e138728_d_n7;
        locals.var_wdld__blk2004_dn8 = assign89970_e138728_d_n8;
        locals.var_wdld__blk2004_dn9 = assign89970_e138728_d_n9;
        locals.var_wdld__blk2004_dn10 = assign89970_e138728_d_n10;
        locals.var_wdld__blk2004_dn11 = assign89970_e138728_d_n11;
        locals.var_wdld__blk2004_dn14 = assign89970_e138728_d_n14;
        locals.var_wdld__blk2004_rv = 0.0;

        let (assign89980_e138736, assign89980_e138736_d_n0, assign89980_e138736_d_n2, assign89980_e138736_d_n4, assign89980_e138736_d_n5, assign89980_e138736_d_n6, assign89980_e138736_d_n7, assign89980_e138736_d_n8, assign89980_e138736_d_n9, assign89980_e138736_d_n10, assign89980_e138736_d_n11, assign89980_e138736_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2087 != 0.0)) {
        let assign89980_e138734: f64 = (locals.var_q_nsubld__blk2006 * locals.var_wdld__blk2004);
        (assign89980_e138734, (locals.var_q_nsubld__blk2006 * locals.var_wdld__blk2004_dn0), (locals.var_q_nsubld__blk2006 * locals.var_wdld__blk2004_dn2), (locals.var_q_nsubld__blk2006 * locals.var_wdld__blk2004_dn4), (locals.var_q_nsubld__blk2006 * locals.var_wdld__blk2004_dn5), (locals.var_q_nsubld__blk2006 * locals.var_wdld__blk2004_dn6), (locals.var_q_nsubld__blk2006 * locals.var_wdld__blk2004_dn7), (locals.var_q_nsubld__blk2006 * locals.var_wdld__blk2004_dn8), (locals.var_q_nsubld__blk2006 * locals.var_wdld__blk2004_dn9), (locals.var_q_nsubld__blk2006 * locals.var_wdld__blk2004_dn10), (locals.var_q_nsubld__blk2006 * locals.var_wdld__blk2004_dn11), (locals.var_q_nsubld__blk2006 * locals.var_wdld__blk2004_dn14),)
    } else {
        (locals.var_q_dep_ld__blk2005, locals.var_q_dep_ld__blk2005_dn0, locals.var_q_dep_ld__blk2005_dn2, locals.var_q_dep_ld__blk2005_dn4, locals.var_q_dep_ld__blk2005_dn5, locals.var_q_dep_ld__blk2005_dn6, locals.var_q_dep_ld__blk2005_dn7, locals.var_q_dep_ld__blk2005_dn8, locals.var_q_dep_ld__blk2005_dn9, locals.var_q_dep_ld__blk2005_dn10, locals.var_q_dep_ld__blk2005_dn11, locals.var_q_dep_ld__blk2005_dn14,)
    }
};
        locals.var_q_dep_ld__blk2005 = assign89980_e138736;
        locals.var_q_dep_ld__blk2005_dn0 = assign89980_e138736_d_n0;
        locals.var_q_dep_ld__blk2005_dn2 = assign89980_e138736_d_n2;
        locals.var_q_dep_ld__blk2005_dn4 = assign89980_e138736_d_n4;
        locals.var_q_dep_ld__blk2005_dn5 = assign89980_e138736_d_n5;
        locals.var_q_dep_ld__blk2005_dn6 = assign89980_e138736_d_n6;
        locals.var_q_dep_ld__blk2005_dn7 = assign89980_e138736_d_n7;
        locals.var_q_dep_ld__blk2005_dn8 = assign89980_e138736_d_n8;
        locals.var_q_dep_ld__blk2005_dn9 = assign89980_e138736_d_n9;
        locals.var_q_dep_ld__blk2005_dn10 = assign89980_e138736_d_n10;
        locals.var_q_dep_ld__blk2005_dn11 = assign89980_e138736_d_n11;
        locals.var_q_dep_ld__blk2005_dn14 = assign89980_e138736_d_n14;
        locals.var_q_dep_ld__blk2005_rv = 0.0;

        let (assign89990_e138748, assign89990_e138748_d_n0, assign89990_e138748_d_n2, assign89990_e138748_d_n4, assign89990_e138748_d_n5, assign89990_e138748_d_n6, assign89990_e138748_d_n7, assign89990_e138748_d_n8, assign89990_e138748_d_n9, assign89990_e138748_d_n10, assign89990_e138748_d_n11, assign89990_e138748_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2087 != 0.0)) {
        let assign89990_e138742: f64 = (locals.var_q_dep_ld__blk2005 / locals.var_cnst0over_func);
        let assign89990_e138745: f64 = (10.0 * 2.220446049250313e-16);
        let assign89990_e138746: f64 = (assign89990_e138742 + assign89990_e138745);
        (assign89990_e138746, (((locals.var_q_dep_ld__blk2005_dn0 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2005 * locals.var_cnst0over_func_dn0)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2005_dn2 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2005 * locals.var_cnst0over_func_dn2)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2005_dn4 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2005 * locals.var_cnst0over_func_dn4)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2005_dn5 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2005 * locals.var_cnst0over_func_dn5)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2005_dn6 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2005 * locals.var_cnst0over_func_dn6)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2005_dn7 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2005 * locals.var_cnst0over_func_dn7)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2005_dn8 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2005 * locals.var_cnst0over_func_dn8)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2005_dn9 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2005 * locals.var_cnst0over_func_dn9)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2005_dn10 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2005 * locals.var_cnst0over_func_dn10)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2005_dn11 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2005 * locals.var_cnst0over_func_dn11)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2005_dn14 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2005 * locals.var_cnst0over_func_dn14)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)),)
    } else {
        (locals.var_xi0p12, locals.var_xi0p12_dn0, locals.var_xi0p12_dn2, locals.var_xi0p12_dn4, locals.var_xi0p12_dn5, locals.var_xi0p12_dn6, locals.var_xi0p12_dn7, locals.var_xi0p12_dn8, locals.var_xi0p12_dn9, locals.var_xi0p12_dn10, locals.var_xi0p12_dn11, locals.var_xi0p12_dn14,)
    }
};
        locals.var_xi0p12 = assign89990_e138748;
        locals.var_xi0p12_dn0 = assign89990_e138748_d_n0;
        locals.var_xi0p12_dn2 = assign89990_e138748_d_n2;
        locals.var_xi0p12_dn4 = assign89990_e138748_d_n4;
        locals.var_xi0p12_dn5 = assign89990_e138748_d_n5;
        locals.var_xi0p12_dn6 = assign89990_e138748_d_n6;
        locals.var_xi0p12_dn7 = assign89990_e138748_d_n7;
        locals.var_xi0p12_dn8 = assign89990_e138748_d_n8;
        locals.var_xi0p12_dn9 = assign89990_e138748_d_n9;
        locals.var_xi0p12_dn10 = assign89990_e138748_d_n10;
        locals.var_xi0p12_dn11 = assign89990_e138748_d_n11;
        locals.var_xi0p12_dn14 = assign89990_e138748_d_n14;
        locals.var_xi0p12_rv = 0.0;

        let (assign90000_e138756, assign90000_e138756_d_n0, assign90000_e138756_d_n2, assign90000_e138756_d_n4, assign90000_e138756_d_n5, assign90000_e138756_d_n6, assign90000_e138756_d_n7, assign90000_e138756_d_n8, assign90000_e138756_d_n9, assign90000_e138756_d_n10, assign90000_e138756_d_n11, assign90000_e138756_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2087 != 0.0)) {
        let assign90000_e138754: f64 = (locals.var_cnst0over_func * locals.var_xi0p12);
        (assign90000_e138754, ((locals.var_cnst0over_func_dn0 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn0)), ((locals.var_cnst0over_func_dn2 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn2)), ((locals.var_cnst0over_func_dn4 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn4)), ((locals.var_cnst0over_func_dn5 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn5)), ((locals.var_cnst0over_func_dn6 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn6)), ((locals.var_cnst0over_func_dn7 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn7)), ((locals.var_cnst0over_func_dn8 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn8)), ((locals.var_cnst0over_func_dn9 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn9)), ((locals.var_cnst0over_func_dn10 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn10)), ((locals.var_cnst0over_func_dn11 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn11)), ((locals.var_cnst0over_func_dn14 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn14)),)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn8, locals.var_qbuld_dn9, locals.var_qbuld_dn10, locals.var_qbuld_dn11, locals.var_qbuld_dn14,)
    }
};
        locals.var_qbuld = assign90000_e138756;
        locals.var_qbuld_dn0 = assign90000_e138756_d_n0;
        locals.var_qbuld_dn2 = assign90000_e138756_d_n2;
        locals.var_qbuld_dn4 = assign90000_e138756_d_n4;
        locals.var_qbuld_dn5 = assign90000_e138756_d_n5;
        locals.var_qbuld_dn6 = assign90000_e138756_d_n6;
        locals.var_qbuld_dn7 = assign90000_e138756_d_n7;
        locals.var_qbuld_dn8 = assign90000_e138756_d_n8;
        locals.var_qbuld_dn9 = assign90000_e138756_d_n9;
        locals.var_qbuld_dn10 = assign90000_e138756_d_n10;
        locals.var_qbuld_dn11 = assign90000_e138756_d_n11;
        locals.var_qbuld_dn14 = assign90000_e138756_d_n14;
        locals.var_qbuld_rv = 0.0;

        let (assign90010_e138766, assign90010_e138766_d_n0, assign90010_e138766_d_n2, assign90010_e138766_d_n4, assign90010_e138766_d_n5, assign90010_e138766_d_n6, assign90010_e138766_d_n7, assign90010_e138766_d_n8, assign90010_e138766_d_n9, assign90010_e138766_d_n10, assign90010_e138766_d_n11, assign90010_e138766_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2087 != 0.0)) {
        let assign90010_e138763: f64 = (locals.var_fs02 + locals.var_xi0p12);
        let assign90010_e138764: f64 = (1.0 / assign90010_e138763);
        (assign90010_e138764, (-((locals.var_fs02_dn0 + locals.var_xi0p12_dn0) / (assign90010_e138763 * assign90010_e138763))), (-((locals.var_fs02_dn2 + locals.var_xi0p12_dn2) / (assign90010_e138763 * assign90010_e138763))), (-((locals.var_fs02_dn4 + locals.var_xi0p12_dn4) / (assign90010_e138763 * assign90010_e138763))), (-((locals.var_fs02_dn5 + locals.var_xi0p12_dn5) / (assign90010_e138763 * assign90010_e138763))), (-((locals.var_fs02_dn6 + locals.var_xi0p12_dn6) / (assign90010_e138763 * assign90010_e138763))), (-((locals.var_fs02_dn7 + locals.var_xi0p12_dn7) / (assign90010_e138763 * assign90010_e138763))), (-((locals.var_fs02_dn8 + locals.var_xi0p12_dn8) / (assign90010_e138763 * assign90010_e138763))), (-((locals.var_fs02_dn9 + locals.var_xi0p12_dn9) / (assign90010_e138763 * assign90010_e138763))), (-((locals.var_fs02_dn10 + locals.var_xi0p12_dn10) / (assign90010_e138763 * assign90010_e138763))), (-((locals.var_fs02_dn11 + locals.var_xi0p12_dn11) / (assign90010_e138763 * assign90010_e138763))), (-((locals.var_fs02_dn14 + locals.var_xi0p12_dn14) / (assign90010_e138763 * assign90010_e138763))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign90010_e138766;
        locals.var_t1_dn0 = assign90010_e138766_d_n0;
        locals.var_t1_dn2 = assign90010_e138766_d_n2;
        locals.var_t1_dn4 = assign90010_e138766_d_n4;
        locals.var_t1_dn5 = assign90010_e138766_d_n5;
        locals.var_t1_dn6 = assign90010_e138766_d_n6;
        locals.var_t1_dn7 = assign90010_e138766_d_n7;
        locals.var_t1_dn8 = assign90010_e138766_d_n8;
        locals.var_t1_dn9 = assign90010_e138766_d_n9;
        locals.var_t1_dn10 = assign90010_e138766_d_n10;
        locals.var_t1_dn11 = assign90010_e138766_d_n11;
        locals.var_t1_dn14 = assign90010_e138766_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign90020_e138776, assign90020_e138776_d_n0, assign90020_e138776_d_n2, assign90020_e138776_d_n4, assign90020_e138776_d_n5, assign90020_e138776_d_n6, assign90020_e138776_d_n7, assign90020_e138776_d_n8, assign90020_e138776_d_n9, assign90020_e138776_d_n10, assign90020_e138776_d_n11, assign90020_e138776_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2087 != 0.0)) {
        let assign90020_e138772: f64 = (locals.var_cnst0over_func * locals.var_fs01);
        let assign90020_e138774: f64 = (assign90020_e138772 * locals.var_t1);
        (assign90020_e138774, ((((locals.var_cnst0over_func_dn0 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn0)) * locals.var_t1) + (assign90020_e138772 * locals.var_t1_dn0)), ((((locals.var_cnst0over_func_dn2 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn2)) * locals.var_t1) + (assign90020_e138772 * locals.var_t1_dn2)), ((((locals.var_cnst0over_func_dn4 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn4)) * locals.var_t1) + (assign90020_e138772 * locals.var_t1_dn4)), ((((locals.var_cnst0over_func_dn5 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn5)) * locals.var_t1) + (assign90020_e138772 * locals.var_t1_dn5)), ((((locals.var_cnst0over_func_dn6 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn6)) * locals.var_t1) + (assign90020_e138772 * locals.var_t1_dn6)), ((((locals.var_cnst0over_func_dn7 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn7)) * locals.var_t1) + (assign90020_e138772 * locals.var_t1_dn7)), ((((locals.var_cnst0over_func_dn8 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn8)) * locals.var_t1) + (assign90020_e138772 * locals.var_t1_dn8)), ((((locals.var_cnst0over_func_dn9 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn9)) * locals.var_t1) + (assign90020_e138772 * locals.var_t1_dn9)), ((((locals.var_cnst0over_func_dn10 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn10)) * locals.var_t1) + (assign90020_e138772 * locals.var_t1_dn10)), ((((locals.var_cnst0over_func_dn11 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn11)) * locals.var_t1) + (assign90020_e138772 * locals.var_t1_dn11)), ((((locals.var_cnst0over_func_dn14 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn14)) * locals.var_t1) + (assign90020_e138772 * locals.var_t1_dn14)),)
    } else {
        (locals.var_qiuld, locals.var_qiuld_dn0, locals.var_qiuld_dn2, locals.var_qiuld_dn4, locals.var_qiuld_dn5, locals.var_qiuld_dn6, locals.var_qiuld_dn7, locals.var_qiuld_dn8, locals.var_qiuld_dn9, locals.var_qiuld_dn10, locals.var_qiuld_dn11, locals.var_qiuld_dn14,)
    }
};
        locals.var_qiuld = assign90020_e138776;
        locals.var_qiuld_dn0 = assign90020_e138776_d_n0;
        locals.var_qiuld_dn2 = assign90020_e138776_d_n2;
        locals.var_qiuld_dn4 = assign90020_e138776_d_n4;
        locals.var_qiuld_dn5 = assign90020_e138776_d_n5;
        locals.var_qiuld_dn6 = assign90020_e138776_d_n6;
        locals.var_qiuld_dn7 = assign90020_e138776_d_n7;
        locals.var_qiuld_dn8 = assign90020_e138776_d_n8;
        locals.var_qiuld_dn9 = assign90020_e138776_d_n9;
        locals.var_qiuld_dn10 = assign90020_e138776_d_n10;
        locals.var_qiuld_dn11 = assign90020_e138776_d_n11;
        locals.var_qiuld_dn14 = assign90020_e138776_d_n14;
        locals.var_qiuld_rv = 0.0;

        let (assign90030_e138784, assign90030_e138784_d_n0, assign90030_e138784_d_n2, assign90030_e138784_d_n4, assign90030_e138784_d_n5, assign90030_e138784_d_n6, assign90030_e138784_d_n7, assign90030_e138784_d_n8, assign90030_e138784_d_n9, assign90030_e138784_d_n10, assign90030_e138784_d_n11, assign90030_e138784_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2087 != 0.0)) {
        let assign90030_e138782: f64 = (locals.var_qbuld + locals.var_qiuld);
        (assign90030_e138782, (locals.var_qbuld_dn0 + locals.var_qiuld_dn0), (locals.var_qbuld_dn2 + locals.var_qiuld_dn2), (locals.var_qbuld_dn4 + locals.var_qiuld_dn4), (locals.var_qbuld_dn5 + locals.var_qiuld_dn5), (locals.var_qbuld_dn6 + locals.var_qiuld_dn6), (locals.var_qbuld_dn7 + locals.var_qiuld_dn7), (locals.var_qbuld_dn8 + locals.var_qiuld_dn8), (locals.var_qbuld_dn9 + locals.var_qiuld_dn9), (locals.var_qbuld_dn10 + locals.var_qiuld_dn10), (locals.var_qbuld_dn11 + locals.var_qiuld_dn11), (locals.var_qbuld_dn14 + locals.var_qiuld_dn14),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn4, locals.var_qsuld_dn5, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn8, locals.var_qsuld_dn9, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn14,)
    }
};
        locals.var_qsuld = assign90030_e138784;
        locals.var_qsuld_dn0 = assign90030_e138784_d_n0;
        locals.var_qsuld_dn2 = assign90030_e138784_d_n2;
        locals.var_qsuld_dn4 = assign90030_e138784_d_n4;
        locals.var_qsuld_dn5 = assign90030_e138784_d_n5;
        locals.var_qsuld_dn6 = assign90030_e138784_d_n6;
        locals.var_qsuld_dn7 = assign90030_e138784_d_n7;
        locals.var_qsuld_dn8 = assign90030_e138784_d_n8;
        locals.var_qsuld_dn9 = assign90030_e138784_d_n9;
        locals.var_qsuld_dn10 = assign90030_e138784_d_n10;
        locals.var_qsuld_dn11 = assign90030_e138784_d_n11;
        locals.var_qsuld_dn14 = assign90030_e138784_d_n14;
        locals.var_qsuld_rv = 0.0;

        let (assign90040_e138790, assign90040_e138790_d_n0, assign90040_e138790_d_n2, assign90040_e138790_d_n4, assign90040_e138790_d_n5, assign90040_e138790_d_n6, assign90040_e138790_d_n7, assign90040_e138790_d_n8, assign90040_e138790_d_n9, assign90040_e138790_d_n10, assign90040_e138790_d_n11, assign90040_e138790_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign90040_e138788: f64 = (locals.var_qsuld - locals.var_qbuld);
        (assign90040_e138788, (locals.var_qsuld_dn0 - locals.var_qbuld_dn0), (locals.var_qsuld_dn2 - locals.var_qbuld_dn2), (locals.var_qsuld_dn4 - locals.var_qbuld_dn4), (locals.var_qsuld_dn5 - locals.var_qbuld_dn5), (locals.var_qsuld_dn6 - locals.var_qbuld_dn6), (locals.var_qsuld_dn7 - locals.var_qbuld_dn7), (locals.var_qsuld_dn8 - locals.var_qbuld_dn8), (locals.var_qsuld_dn9 - locals.var_qbuld_dn9), (locals.var_qsuld_dn10 - locals.var_qbuld_dn10), (locals.var_qsuld_dn11 - locals.var_qbuld_dn11), (locals.var_qsuld_dn14 - locals.var_qbuld_dn14),)
    } else {
        (locals.var_qiuld, locals.var_qiuld_dn0, locals.var_qiuld_dn2, locals.var_qiuld_dn4, locals.var_qiuld_dn5, locals.var_qiuld_dn6, locals.var_qiuld_dn7, locals.var_qiuld_dn8, locals.var_qiuld_dn9, locals.var_qiuld_dn10, locals.var_qiuld_dn11, locals.var_qiuld_dn14,)
    }
};
        locals.var_qiuld = assign90040_e138790;
        locals.var_qiuld_dn0 = assign90040_e138790_d_n0;
        locals.var_qiuld_dn2 = assign90040_e138790_d_n2;
        locals.var_qiuld_dn4 = assign90040_e138790_d_n4;
        locals.var_qiuld_dn5 = assign90040_e138790_d_n5;
        locals.var_qiuld_dn6 = assign90040_e138790_d_n6;
        locals.var_qiuld_dn7 = assign90040_e138790_d_n7;
        locals.var_qiuld_dn8 = assign90040_e138790_d_n8;
        locals.var_qiuld_dn9 = assign90040_e138790_d_n9;
        locals.var_qiuld_dn10 = assign90040_e138790_d_n10;
        locals.var_qiuld_dn11 = assign90040_e138790_d_n11;
        locals.var_qiuld_dn14 = assign90040_e138790_d_n14;
        locals.var_qiuld_rv = 0.0;

        let assign90050_e138793: f64 = if locals.var_lover_func < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2102 = assign90050_e138793;
        locals.var_guard2102_rv = 0.0;

        let (assign90060_e138800, assign90060_e138800_d_n0, assign90060_e138800_d_n2, assign90060_e138800_d_n4, assign90060_e138800_d_n5, assign90060_e138800_d_n6, assign90060_e138800_d_n7, assign90060_e138800_d_n8, assign90060_e138800_d_n9, assign90060_e138800_d_n10, assign90060_e138800_d_n11, assign90060_e138800_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2102 != 0.0)) {
        let assign90060_e138798: f64 = (-locals.var_lover_func);
        (assign90060_e138798, (-locals.var_lover_func_dn0), (-locals.var_lover_func_dn2), (-locals.var_lover_func_dn4), (-locals.var_lover_func_dn5), (-locals.var_lover_func_dn6), (-locals.var_lover_func_dn7), (-locals.var_lover_func_dn8), (-locals.var_lover_func_dn9), (-locals.var_lover_func_dn10), (-locals.var_lover_func_dn11), (-locals.var_lover_func_dn14),)
    } else {
        (locals.var_lover_func, locals.var_lover_func_dn0, locals.var_lover_func_dn2, locals.var_lover_func_dn4, locals.var_lover_func_dn5, locals.var_lover_func_dn6, locals.var_lover_func_dn7, locals.var_lover_func_dn8, locals.var_lover_func_dn9, locals.var_lover_func_dn10, locals.var_lover_func_dn11, locals.var_lover_func_dn14,)
    }
};
        locals.var_lover_func = assign90060_e138800;
        locals.var_lover_func_dn0 = assign90060_e138800_d_n0;
        locals.var_lover_func_dn2 = assign90060_e138800_d_n2;
        locals.var_lover_func_dn4 = assign90060_e138800_d_n4;
        locals.var_lover_func_dn5 = assign90060_e138800_d_n5;
        locals.var_lover_func_dn6 = assign90060_e138800_d_n6;
        locals.var_lover_func_dn7 = assign90060_e138800_d_n7;
        locals.var_lover_func_dn8 = assign90060_e138800_d_n8;
        locals.var_lover_func_dn9 = assign90060_e138800_d_n9;
        locals.var_lover_func_dn10 = assign90060_e138800_d_n10;
        locals.var_lover_func_dn11 = assign90060_e138800_d_n11;
        locals.var_lover_func_dn14 = assign90060_e138800_d_n14;
        locals.var_lover_func_rv = 0.0;

        let assign90070_e138803: f64 = if p.p55 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2103 = assign90070_e138803;
        locals.var_guard2103_rv = 0.0;

        let assign90080_e138806: f64 = if p.p50 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2104 = assign90080_e138806;
        locals.var_guard2104_rv = 0.0;

        let (assign90090_e138817, assign90090_e138817_d_n0, assign90090_e138817_d_n2, assign90090_e138817_d_n4, assign90090_e138817_d_n5, assign90090_e138817_d_n6, assign90090_e138817_d_n7, assign90090_e138817_d_n8, assign90090_e138817_d_n9, assign90090_e138817_d_n10, assign90090_e138817_d_n11, assign90090_e138817_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2102 != 0.0)) && (locals.var_guard2103 != 0.0)) && (locals.var_guard2104 != 0.0)) {
        let assign90090_e138815: f64 = (-locals.var_ps0ld);
        (assign90090_e138815, (-locals.var_ps0ld_dn0), (-locals.var_ps0ld_dn2), (-locals.var_ps0ld_dn4), (-locals.var_ps0ld_dn5), (-locals.var_ps0ld_dn6), (-locals.var_ps0ld_dn7), (-locals.var_ps0ld_dn8), (-locals.var_ps0ld_dn9), (-locals.var_ps0ld_dn10), (-locals.var_ps0ld_dn11), (-locals.var_ps0ld_dn14),)
    } else {
        (locals.var_vx__blk2007, locals.var_vx__blk2007_dn0, locals.var_vx__blk2007_dn2, locals.var_vx__blk2007_dn4, locals.var_vx__blk2007_dn5, locals.var_vx__blk2007_dn6, locals.var_vx__blk2007_dn7, locals.var_vx__blk2007_dn8, locals.var_vx__blk2007_dn9, locals.var_vx__blk2007_dn10, locals.var_vx__blk2007_dn11, locals.var_vx__blk2007_dn14,)
    }
};
        locals.var_vx__blk2007 = assign90090_e138817;
        locals.var_vx__blk2007_dn0 = assign90090_e138817_d_n0;
        locals.var_vx__blk2007_dn2 = assign90090_e138817_d_n2;
        locals.var_vx__blk2007_dn4 = assign90090_e138817_d_n4;
        locals.var_vx__blk2007_dn5 = assign90090_e138817_d_n5;
        locals.var_vx__blk2007_dn6 = assign90090_e138817_d_n6;
        locals.var_vx__blk2007_dn7 = assign90090_e138817_d_n7;
        locals.var_vx__blk2007_dn8 = assign90090_e138817_d_n8;
        locals.var_vx__blk2007_dn9 = assign90090_e138817_d_n9;
        locals.var_vx__blk2007_dn10 = assign90090_e138817_d_n10;
        locals.var_vx__blk2007_dn11 = assign90090_e138817_d_n11;
        locals.var_vx__blk2007_dn14 = assign90090_e138817_d_n14;
        locals.var_vx__blk2007_rv = 0.0;

        let (assign90100_e138828, assign90100_e138828_d_n0, assign90100_e138828_d_n2, assign90100_e138828_d_n4, assign90100_e138828_d_n5, assign90100_e138828_d_n6, assign90100_e138828_d_n7, assign90100_e138828_d_n8, assign90100_e138828_d_n9, assign90100_e138828_d_n10, assign90100_e138828_d_n11, assign90100_e138828_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2102 != 0.0)) && (locals.var_guard2103 != 0.0)) && (locals.var_guard2104 == 0.0)) {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn11, locals.var_vxbgmt_dn14,)
    } else {
        (locals.var_vx__blk2007, locals.var_vx__blk2007_dn0, locals.var_vx__blk2007_dn2, locals.var_vx__blk2007_dn4, locals.var_vx__blk2007_dn5, locals.var_vx__blk2007_dn6, locals.var_vx__blk2007_dn7, locals.var_vx__blk2007_dn8, locals.var_vx__blk2007_dn9, locals.var_vx__blk2007_dn10, locals.var_vx__blk2007_dn11, locals.var_vx__blk2007_dn14,)
    }
};
        locals.var_vx__blk2007 = assign90100_e138828;
        locals.var_vx__blk2007_dn0 = assign90100_e138828_d_n0;
        locals.var_vx__blk2007_dn2 = assign90100_e138828_d_n2;
        locals.var_vx__blk2007_dn4 = assign90100_e138828_d_n4;
        locals.var_vx__blk2007_dn5 = assign90100_e138828_d_n5;
        locals.var_vx__blk2007_dn6 = assign90100_e138828_d_n6;
        locals.var_vx__blk2007_dn7 = assign90100_e138828_d_n7;
        locals.var_vx__blk2007_dn8 = assign90100_e138828_d_n8;
        locals.var_vx__blk2007_dn9 = assign90100_e138828_d_n9;
        locals.var_vx__blk2007_dn10 = assign90100_e138828_d_n10;
        locals.var_vx__blk2007_dn11 = assign90100_e138828_d_n11;
        locals.var_vx__blk2007_dn14 = assign90100_e138828_d_n14;
        locals.var_vx__blk2007_rv = 0.0;

        let (assign90110_e138849, assign90110_e138849_d_n0, assign90110_e138849_d_n2, assign90110_e138849_d_n4, assign90110_e138849_d_n5, assign90110_e138849_d_n6, assign90110_e138849_d_n7, assign90110_e138849_d_n8, assign90110_e138849_d_n9, assign90110_e138849_d_n10, assign90110_e138849_d_n11, assign90110_e138849_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2102 != 0.0)) && (locals.var_guard2103 != 0.0)) {
        let assign90110_e138836: f64 = (locals.var_vx__blk2007 + p.p137);
        let assign90110_e138839: f64 = (locals.var_vx__blk2007 + p.p137);
        let assign90110_e138840: f64 = (assign90110_e138836 * assign90110_e138839);
        let assign90110_e138843: f64 = (4.0 * 0.1);
        let assign90110_e138845: f64 = (assign90110_e138843 * 0.1);
        let assign90110_e138846: f64 = (assign90110_e138840 + assign90110_e138845);
        let assign90110_e138847: f64 = (assign90110_e138846).sqrt();
        (assign90110_e138847, (((locals.var_vx__blk2007_dn0 * assign90110_e138839) + (assign90110_e138836 * locals.var_vx__blk2007_dn0)) / (2.0 * assign90110_e138847)), (((locals.var_vx__blk2007_dn2 * assign90110_e138839) + (assign90110_e138836 * locals.var_vx__blk2007_dn2)) / (2.0 * assign90110_e138847)), (((locals.var_vx__blk2007_dn4 * assign90110_e138839) + (assign90110_e138836 * locals.var_vx__blk2007_dn4)) / (2.0 * assign90110_e138847)), (((locals.var_vx__blk2007_dn5 * assign90110_e138839) + (assign90110_e138836 * locals.var_vx__blk2007_dn5)) / (2.0 * assign90110_e138847)), (((locals.var_vx__blk2007_dn6 * assign90110_e138839) + (assign90110_e138836 * locals.var_vx__blk2007_dn6)) / (2.0 * assign90110_e138847)), (((locals.var_vx__blk2007_dn7 * assign90110_e138839) + (assign90110_e138836 * locals.var_vx__blk2007_dn7)) / (2.0 * assign90110_e138847)), (((locals.var_vx__blk2007_dn8 * assign90110_e138839) + (assign90110_e138836 * locals.var_vx__blk2007_dn8)) / (2.0 * assign90110_e138847)), (((locals.var_vx__blk2007_dn9 * assign90110_e138839) + (assign90110_e138836 * locals.var_vx__blk2007_dn9)) / (2.0 * assign90110_e138847)), (((locals.var_vx__blk2007_dn10 * assign90110_e138839) + (assign90110_e138836 * locals.var_vx__blk2007_dn10)) / (2.0 * assign90110_e138847)), (((locals.var_vx__blk2007_dn11 * assign90110_e138839) + (assign90110_e138836 * locals.var_vx__blk2007_dn11)) / (2.0 * assign90110_e138847)), (((locals.var_vx__blk2007_dn14 * assign90110_e138839) + (assign90110_e138836 * locals.var_vx__blk2007_dn14)) / (2.0 * assign90110_e138847)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign90110_e138849;
        locals.var_tmf2_dn0 = assign90110_e138849_d_n0;
        locals.var_tmf2_dn2 = assign90110_e138849_d_n2;
        locals.var_tmf2_dn4 = assign90110_e138849_d_n4;
        locals.var_tmf2_dn5 = assign90110_e138849_d_n5;
        locals.var_tmf2_dn6 = assign90110_e138849_d_n6;
        locals.var_tmf2_dn7 = assign90110_e138849_d_n7;
        locals.var_tmf2_dn8 = assign90110_e138849_d_n8;
        locals.var_tmf2_dn9 = assign90110_e138849_d_n9;
        locals.var_tmf2_dn10 = assign90110_e138849_d_n10;
        locals.var_tmf2_dn11 = assign90110_e138849_d_n11;
        locals.var_tmf2_dn14 = assign90110_e138849_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign90120_e138865, assign90120_e138865_d_n0, assign90120_e138865_d_n2, assign90120_e138865_d_n4, assign90120_e138865_d_n5, assign90120_e138865_d_n6, assign90120_e138865_d_n7, assign90120_e138865_d_n8, assign90120_e138865_d_n9, assign90120_e138865_d_n10, assign90120_e138865_d_n11, assign90120_e138865_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2102 != 0.0)) && (locals.var_guard2103 != 0.0)) {
        let assign90120_e138859: f64 = (locals.var_vx__blk2007 + p.p137);
        let assign90120_e138861: f64 = (assign90120_e138859 / locals.var_tmf2);
        let assign90120_e138862: f64 = (1.0 + assign90120_e138861);
        let assign90120_e138863: f64 = (0.5 * assign90120_e138862);
        (assign90120_e138863, (0.5 * (((locals.var_vx__blk2007_dn0 * locals.var_tmf2) - (assign90120_e138859 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk2007_dn2 * locals.var_tmf2) - (assign90120_e138859 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk2007_dn4 * locals.var_tmf2) - (assign90120_e138859 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk2007_dn5 * locals.var_tmf2) - (assign90120_e138859 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk2007_dn6 * locals.var_tmf2) - (assign90120_e138859 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk2007_dn7 * locals.var_tmf2) - (assign90120_e138859 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk2007_dn8 * locals.var_tmf2) - (assign90120_e138859 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk2007_dn9 * locals.var_tmf2) - (assign90120_e138859 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk2007_dn10 * locals.var_tmf2) - (assign90120_e138859 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk2007_dn11 * locals.var_tmf2) - (assign90120_e138859 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk2007_dn14 * locals.var_tmf2) - (assign90120_e138859 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign90120_e138865;
        locals.var_t9_dn0 = assign90120_e138865_d_n0;
        locals.var_t9_dn2 = assign90120_e138865_d_n2;
        locals.var_t9_dn4 = assign90120_e138865_d_n4;
        locals.var_t9_dn5 = assign90120_e138865_d_n5;
        locals.var_t9_dn6 = assign90120_e138865_d_n6;
        locals.var_t9_dn7 = assign90120_e138865_d_n7;
        locals.var_t9_dn8 = assign90120_e138865_d_n8;
        locals.var_t9_dn9 = assign90120_e138865_d_n9;
        locals.var_t9_dn10 = assign90120_e138865_d_n10;
        locals.var_t9_dn11 = assign90120_e138865_d_n11;
        locals.var_t9_dn14 = assign90120_e138865_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign90130_e138879, assign90130_e138879_d_n0, assign90130_e138879_d_n2, assign90130_e138879_d_n4, assign90130_e138879_d_n5, assign90130_e138879_d_n6, assign90130_e138879_d_n7, assign90130_e138879_d_n8, assign90130_e138879_d_n9, assign90130_e138879_d_n10, assign90130_e138879_d_n11, assign90130_e138879_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2102 != 0.0)) && (locals.var_guard2103 != 0.0)) {
        let assign90130_e138874: f64 = (locals.var_vx__blk2007 + p.p137);
        let assign90130_e138876: f64 = (assign90130_e138874 + locals.var_tmf2);
        let assign90130_e138877: f64 = (0.5 * assign90130_e138876);
        (assign90130_e138877, (0.5 * (locals.var_vx__blk2007_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_vx__blk2007_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_vx__blk2007_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_vx__blk2007_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_vx__blk2007_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_vx__blk2007_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_vx__blk2007_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_vx__blk2007_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_vx__blk2007_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_vx__blk2007_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_vx__blk2007_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign90130_e138879;
        locals.var_t2_dn0 = assign90130_e138879_d_n0;
        locals.var_t2_dn2 = assign90130_e138879_d_n2;
        locals.var_t2_dn4 = assign90130_e138879_d_n4;
        locals.var_t2_dn5 = assign90130_e138879_d_n5;
        locals.var_t2_dn6 = assign90130_e138879_d_n6;
        locals.var_t2_dn7 = assign90130_e138879_d_n7;
        locals.var_t2_dn8 = assign90130_e138879_d_n8;
        locals.var_t2_dn9 = assign90130_e138879_d_n9;
        locals.var_t2_dn10 = assign90130_e138879_d_n10;
        locals.var_t2_dn11 = assign90130_e138879_d_n11;
        locals.var_t2_dn14 = assign90130_e138879_d_n14;
        locals.var_t2_rv = 0.0;

        let assign90140_e138882: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2105 = assign90140_e138882;
        locals.var_guard2105_rv = 0.0;

        let (assign90150_e138892, assign90150_e138892_d_n0, assign90150_e138892_d_n2, assign90150_e138892_d_n4, assign90150_e138892_d_n5, assign90150_e138892_d_n6, assign90150_e138892_d_n7, assign90150_e138892_d_n8, assign90150_e138892_d_n9, assign90150_e138892_d_n10, assign90150_e138892_d_n11, assign90150_e138892_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2102 != 0.0)) && (locals.var_guard2103 != 0.0)) && (locals.var_guard2105 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign90150_e138892;
        locals.var_t2_dn0 = assign90150_e138892_d_n0;
        locals.var_t2_dn2 = assign90150_e138892_d_n2;
        locals.var_t2_dn4 = assign90150_e138892_d_n4;
        locals.var_t2_dn5 = assign90150_e138892_d_n5;
        locals.var_t2_dn6 = assign90150_e138892_d_n6;
        locals.var_t2_dn7 = assign90150_e138892_d_n7;
        locals.var_t2_dn8 = assign90150_e138892_d_n8;
        locals.var_t2_dn9 = assign90150_e138892_d_n9;
        locals.var_t2_dn10 = assign90150_e138892_d_n10;
        locals.var_t2_dn11 = assign90150_e138892_d_n11;
        locals.var_t2_dn14 = assign90150_e138892_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign90160_e138902, assign90160_e138902_d_n0, assign90160_e138902_d_n2, assign90160_e138902_d_n4, assign90160_e138902_d_n5, assign90160_e138902_d_n6, assign90160_e138902_d_n7, assign90160_e138902_d_n8, assign90160_e138902_d_n9, assign90160_e138902_d_n10, assign90160_e138902_d_n11, assign90160_e138902_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2102 != 0.0)) && (locals.var_guard2103 != 0.0)) && (locals.var_guard2105 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign90160_e138902;
        locals.var_t9_dn0 = assign90160_e138902_d_n0;
        locals.var_t9_dn2 = assign90160_e138902_d_n2;
        locals.var_t9_dn4 = assign90160_e138902_d_n4;
        locals.var_t9_dn5 = assign90160_e138902_d_n5;
        locals.var_t9_dn6 = assign90160_e138902_d_n6;
        locals.var_t9_dn7 = assign90160_e138902_d_n7;
        locals.var_t9_dn8 = assign90160_e138902_d_n8;
        locals.var_t9_dn9 = assign90160_e138902_d_n9;
        locals.var_t9_dn10 = assign90160_e138902_d_n10;
        locals.var_t9_dn11 = assign90160_e138902_d_n11;
        locals.var_t9_dn14 = assign90160_e138902_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign90170_e138915, assign90170_e138915_d_n0, assign90170_e138915_d_n2, assign90170_e138915_d_n4, assign90170_e138915_d_n5, assign90170_e138915_d_n6, assign90170_e138915_d_n7, assign90170_e138915_d_n8, assign90170_e138915_d_n9, assign90170_e138915_d_n10, assign90170_e138915_d_n11, assign90170_e138915_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2102 != 0.0)) && (locals.var_guard2103 != 0.0)) {
        let assign90170_e138910: f64 = (locals.var_kjunc * locals.var_t2);
        let assign90170_e138911: f64 = (assign90170_e138910).sqrt();
        let assign90170_e138913: f64 = (assign90170_e138911 * p.p432);
        (assign90170_e138913, ((((locals.var_kjunc_dn0 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn0)) / (2.0 * assign90170_e138911)) * p.p432), ((((locals.var_kjunc_dn2 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn2)) / (2.0 * assign90170_e138911)) * p.p432), ((((locals.var_kjunc_dn4 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn4)) / (2.0 * assign90170_e138911)) * p.p432), ((((locals.var_kjunc_dn5 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn5)) / (2.0 * assign90170_e138911)) * p.p432), ((((locals.var_kjunc_dn6 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn6)) / (2.0 * assign90170_e138911)) * p.p432), ((((locals.var_kjunc_dn7 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn7)) / (2.0 * assign90170_e138911)) * p.p432), ((((locals.var_kjunc_dn8 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn8)) / (2.0 * assign90170_e138911)) * p.p432), ((((locals.var_kjunc_dn9 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn9)) / (2.0 * assign90170_e138911)) * p.p432), ((((locals.var_kjunc_dn10 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn10)) / (2.0 * assign90170_e138911)) * p.p432), ((((locals.var_kjunc_dn11 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn11)) / (2.0 * assign90170_e138911)) * p.p432), ((((locals.var_kjunc_dn14 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn14)) / (2.0 * assign90170_e138911)) * p.p432),)
    } else {
        (locals.var_wjunc0, locals.var_wjunc0_dn0, locals.var_wjunc0_dn2, locals.var_wjunc0_dn4, locals.var_wjunc0_dn5, locals.var_wjunc0_dn6, locals.var_wjunc0_dn7, locals.var_wjunc0_dn8, locals.var_wjunc0_dn9, locals.var_wjunc0_dn10, locals.var_wjunc0_dn11, locals.var_wjunc0_dn14,)
    }
};
        locals.var_wjunc0 = assign90170_e138915;
        locals.var_wjunc0_dn0 = assign90170_e138915_d_n0;
        locals.var_wjunc0_dn2 = assign90170_e138915_d_n2;
        locals.var_wjunc0_dn4 = assign90170_e138915_d_n4;
        locals.var_wjunc0_dn5 = assign90170_e138915_d_n5;
        locals.var_wjunc0_dn6 = assign90170_e138915_d_n6;
        locals.var_wjunc0_dn7 = assign90170_e138915_d_n7;
        locals.var_wjunc0_dn8 = assign90170_e138915_d_n8;
        locals.var_wjunc0_dn9 = assign90170_e138915_d_n9;
        locals.var_wjunc0_dn10 = assign90170_e138915_d_n10;
        locals.var_wjunc0_dn11 = assign90170_e138915_d_n11;
        locals.var_wjunc0_dn14 = assign90170_e138915_d_n14;
        locals.var_wjunc0_rv = 0.0;

        let (assign90180_e138929, assign90180_e138929_d_n0, assign90180_e138929_d_n2, assign90180_e138929_d_n4, assign90180_e138929_d_n5, assign90180_e138929_d_n6, assign90180_e138929_d_n7, assign90180_e138929_d_n8, assign90180_e138929_d_n9, assign90180_e138929_d_n10, assign90180_e138929_d_n11, assign90180_e138929_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2102 != 0.0)) && (locals.var_guard2103 != 0.0)) {
        let assign90180_e138923: f64 = (locals.var_lover_func - locals.var_wjunc0);
        let assign90180_e138926: f64 = (0.1 * locals.var_lover_func);
        let assign90180_e138927: f64 = (assign90180_e138923 - assign90180_e138926);
        (assign90180_e138927, ((locals.var_lover_func_dn0 - locals.var_wjunc0_dn0) - (0.1 * locals.var_lover_func_dn0)), ((locals.var_lover_func_dn2 - locals.var_wjunc0_dn2) - (0.1 * locals.var_lover_func_dn2)), ((locals.var_lover_func_dn4 - locals.var_wjunc0_dn4) - (0.1 * locals.var_lover_func_dn4)), ((locals.var_lover_func_dn5 - locals.var_wjunc0_dn5) - (0.1 * locals.var_lover_func_dn5)), ((locals.var_lover_func_dn6 - locals.var_wjunc0_dn6) - (0.1 * locals.var_lover_func_dn6)), ((locals.var_lover_func_dn7 - locals.var_wjunc0_dn7) - (0.1 * locals.var_lover_func_dn7)), ((locals.var_lover_func_dn8 - locals.var_wjunc0_dn8) - (0.1 * locals.var_lover_func_dn8)), ((locals.var_lover_func_dn9 - locals.var_wjunc0_dn9) - (0.1 * locals.var_lover_func_dn9)), ((locals.var_lover_func_dn10 - locals.var_wjunc0_dn10) - (0.1 * locals.var_lover_func_dn10)), ((locals.var_lover_func_dn11 - locals.var_wjunc0_dn11) - (0.1 * locals.var_lover_func_dn11)), ((locals.var_lover_func_dn14 - locals.var_wjunc0_dn14) - (0.1 * locals.var_lover_func_dn14)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign90180_e138929;
        locals.var_tmf1_dn0 = assign90180_e138929_d_n0;
        locals.var_tmf1_dn2 = assign90180_e138929_d_n2;
        locals.var_tmf1_dn4 = assign90180_e138929_d_n4;
        locals.var_tmf1_dn5 = assign90180_e138929_d_n5;
        locals.var_tmf1_dn6 = assign90180_e138929_d_n6;
        locals.var_tmf1_dn7 = assign90180_e138929_d_n7;
        locals.var_tmf1_dn8 = assign90180_e138929_d_n8;
        locals.var_tmf1_dn9 = assign90180_e138929_d_n9;
        locals.var_tmf1_dn10 = assign90180_e138929_d_n10;
        locals.var_tmf1_dn11 = assign90180_e138929_d_n11;
        locals.var_tmf1_dn14 = assign90180_e138929_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign90190_e138943, assign90190_e138943_d_n0, assign90190_e138943_d_n2, assign90190_e138943_d_n4, assign90190_e138943_d_n5, assign90190_e138943_d_n6, assign90190_e138943_d_n7, assign90190_e138943_d_n8, assign90190_e138943_d_n9, assign90190_e138943_d_n10, assign90190_e138943_d_n11, assign90190_e138943_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2102 != 0.0)) && (locals.var_guard2103 != 0.0)) {
        let assign90190_e138937: f64 = (4.0 * locals.var_lover_func);
        let assign90190_e138940: f64 = (0.1 * locals.var_lover_func);
        let assign90190_e138941: f64 = (assign90190_e138937 * assign90190_e138940);
        (assign90190_e138941, (((4.0 * locals.var_lover_func_dn0) * assign90190_e138940) + (assign90190_e138937 * (0.1 * locals.var_lover_func_dn0))), (((4.0 * locals.var_lover_func_dn2) * assign90190_e138940) + (assign90190_e138937 * (0.1 * locals.var_lover_func_dn2))), (((4.0 * locals.var_lover_func_dn4) * assign90190_e138940) + (assign90190_e138937 * (0.1 * locals.var_lover_func_dn4))), (((4.0 * locals.var_lover_func_dn5) * assign90190_e138940) + (assign90190_e138937 * (0.1 * locals.var_lover_func_dn5))), (((4.0 * locals.var_lover_func_dn6) * assign90190_e138940) + (assign90190_e138937 * (0.1 * locals.var_lover_func_dn6))), (((4.0 * locals.var_lover_func_dn7) * assign90190_e138940) + (assign90190_e138937 * (0.1 * locals.var_lover_func_dn7))), (((4.0 * locals.var_lover_func_dn8) * assign90190_e138940) + (assign90190_e138937 * (0.1 * locals.var_lover_func_dn8))), (((4.0 * locals.var_lover_func_dn9) * assign90190_e138940) + (assign90190_e138937 * (0.1 * locals.var_lover_func_dn9))), (((4.0 * locals.var_lover_func_dn10) * assign90190_e138940) + (assign90190_e138937 * (0.1 * locals.var_lover_func_dn10))), (((4.0 * locals.var_lover_func_dn11) * assign90190_e138940) + (assign90190_e138937 * (0.1 * locals.var_lover_func_dn11))), (((4.0 * locals.var_lover_func_dn14) * assign90190_e138940) + (assign90190_e138937 * (0.1 * locals.var_lover_func_dn14))),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign90190_e138943;
        locals.var_tmf2_dn0 = assign90190_e138943_d_n0;
        locals.var_tmf2_dn2 = assign90190_e138943_d_n2;
        locals.var_tmf2_dn4 = assign90190_e138943_d_n4;
        locals.var_tmf2_dn5 = assign90190_e138943_d_n5;
        locals.var_tmf2_dn6 = assign90190_e138943_d_n6;
        locals.var_tmf2_dn7 = assign90190_e138943_d_n7;
        locals.var_tmf2_dn8 = assign90190_e138943_d_n8;
        locals.var_tmf2_dn9 = assign90190_e138943_d_n9;
        locals.var_tmf2_dn10 = assign90190_e138943_d_n10;
        locals.var_tmf2_dn11 = assign90190_e138943_d_n11;
        locals.var_tmf2_dn14 = assign90190_e138943_d_n14;
        locals.var_tmf2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_348(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign90200_e138957, assign90200_e138957_d_n0, assign90200_e138957_d_n2, assign90200_e138957_d_n4, assign90200_e138957_d_n5, assign90200_e138957_d_n6, assign90200_e138957_d_n7, assign90200_e138957_d_n8, assign90200_e138957_d_n9, assign90200_e138957_d_n10, assign90200_e138957_d_n11, assign90200_e138957_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2102 != 0.0)) && (locals.var_guard2103 != 0.0)) {
        let (assign90200_e138955, assign90200_e138955_d_n0, assign90200_e138955_d_n2, assign90200_e138955_d_n4, assign90200_e138955_d_n5, assign90200_e138955_d_n6, assign90200_e138955_d_n7, assign90200_e138955_d_n8, assign90200_e138955_d_n9, assign90200_e138955_d_n10, assign90200_e138955_d_n11, assign90200_e138955_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign90200_e138954: f64 = (-locals.var_tmf2);
                (assign90200_e138954, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign90200_e138955, assign90200_e138955_d_n0, assign90200_e138955_d_n2, assign90200_e138955_d_n4, assign90200_e138955_d_n5, assign90200_e138955_d_n6, assign90200_e138955_d_n7, assign90200_e138955_d_n8, assign90200_e138955_d_n9, assign90200_e138955_d_n10, assign90200_e138955_d_n11, assign90200_e138955_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign90200_e138957;
        locals.var_tmf2_dn0 = assign90200_e138957_d_n0;
        locals.var_tmf2_dn2 = assign90200_e138957_d_n2;
        locals.var_tmf2_dn4 = assign90200_e138957_d_n4;
        locals.var_tmf2_dn5 = assign90200_e138957_d_n5;
        locals.var_tmf2_dn6 = assign90200_e138957_d_n6;
        locals.var_tmf2_dn7 = assign90200_e138957_d_n7;
        locals.var_tmf2_dn8 = assign90200_e138957_d_n8;
        locals.var_tmf2_dn9 = assign90200_e138957_d_n9;
        locals.var_tmf2_dn10 = assign90200_e138957_d_n10;
        locals.var_tmf2_dn11 = assign90200_e138957_d_n11;
        locals.var_tmf2_dn14 = assign90200_e138957_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign90210_e138970, assign90210_e138970_d_n0, assign90210_e138970_d_n2, assign90210_e138970_d_n4, assign90210_e138970_d_n5, assign90210_e138970_d_n6, assign90210_e138970_d_n7, assign90210_e138970_d_n8, assign90210_e138970_d_n9, assign90210_e138970_d_n10, assign90210_e138970_d_n11, assign90210_e138970_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2102 != 0.0)) && (locals.var_guard2103 != 0.0)) {
        let assign90210_e138965: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign90210_e138967: f64 = (assign90210_e138965 + locals.var_tmf2);
        let assign90210_e138968: f64 = (assign90210_e138967).sqrt();
        (assign90210_e138968, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign90210_e138968)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign90210_e138968)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign90210_e138968)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign90210_e138968)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign90210_e138968)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign90210_e138968)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign90210_e138968)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign90210_e138968)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign90210_e138968)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign90210_e138968)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign90210_e138968)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign90210_e138970;
        locals.var_tmf2_dn0 = assign90210_e138970_d_n0;
        locals.var_tmf2_dn2 = assign90210_e138970_d_n2;
        locals.var_tmf2_dn4 = assign90210_e138970_d_n4;
        locals.var_tmf2_dn5 = assign90210_e138970_d_n5;
        locals.var_tmf2_dn6 = assign90210_e138970_d_n6;
        locals.var_tmf2_dn7 = assign90210_e138970_d_n7;
        locals.var_tmf2_dn8 = assign90210_e138970_d_n8;
        locals.var_tmf2_dn9 = assign90210_e138970_d_n9;
        locals.var_tmf2_dn10 = assign90210_e138970_d_n10;
        locals.var_tmf2_dn11 = assign90210_e138970_d_n11;
        locals.var_tmf2_dn14 = assign90210_e138970_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign90220_e138984, assign90220_e138984_d_n0, assign90220_e138984_d_n2, assign90220_e138984_d_n4, assign90220_e138984_d_n5, assign90220_e138984_d_n6, assign90220_e138984_d_n7, assign90220_e138984_d_n8, assign90220_e138984_d_n9, assign90220_e138984_d_n10, assign90220_e138984_d_n11, assign90220_e138984_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2102 != 0.0)) && (locals.var_guard2103 != 0.0)) {
        let assign90220_e138980: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign90220_e138981: f64 = (1.0 + assign90220_e138980);
        let assign90220_e138982: f64 = (0.5 * assign90220_e138981);
        (assign90220_e138982, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign90220_e138984;
        locals.var_t0_dn0 = assign90220_e138984_d_n0;
        locals.var_t0_dn2 = assign90220_e138984_d_n2;
        locals.var_t0_dn4 = assign90220_e138984_d_n4;
        locals.var_t0_dn5 = assign90220_e138984_d_n5;
        locals.var_t0_dn6 = assign90220_e138984_d_n6;
        locals.var_t0_dn7 = assign90220_e138984_d_n7;
        locals.var_t0_dn8 = assign90220_e138984_d_n8;
        locals.var_t0_dn9 = assign90220_e138984_d_n9;
        locals.var_t0_dn10 = assign90220_e138984_d_n10;
        locals.var_t0_dn11 = assign90220_e138984_d_n11;
        locals.var_t0_dn14 = assign90220_e138984_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign90230_e138998, assign90230_e138998_d_n0, assign90230_e138998_d_n2, assign90230_e138998_d_n4, assign90230_e138998_d_n5, assign90230_e138998_d_n6, assign90230_e138998_d_n7, assign90230_e138998_d_n8, assign90230_e138998_d_n9, assign90230_e138998_d_n10, assign90230_e138998_d_n11, assign90230_e138998_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2102 != 0.0)) && (locals.var_guard2103 != 0.0)) {
        let assign90230_e138994: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign90230_e138995: f64 = (0.5 * assign90230_e138994);
        let assign90230_e138996: f64 = (locals.var_lover_func - assign90230_e138995);
        (assign90230_e138996, (locals.var_lover_func_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_lover_func_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_lover_func_dn4 - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_lover_func_dn5 - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_lover_func_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_lover_func_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_lover_func_dn8 - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_lover_func_dn9 - (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (locals.var_lover_func_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_lover_func_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_lover_func_dn14 - (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_wjuncld, locals.var_wjuncld_dn0, locals.var_wjuncld_dn2, locals.var_wjuncld_dn4, locals.var_wjuncld_dn5, locals.var_wjuncld_dn6, locals.var_wjuncld_dn7, locals.var_wjuncld_dn8, locals.var_wjuncld_dn9, locals.var_wjuncld_dn10, locals.var_wjuncld_dn11, locals.var_wjuncld_dn14,)
    }
};
        locals.var_wjuncld = assign90230_e138998;
        locals.var_wjuncld_dn0 = assign90230_e138998_d_n0;
        locals.var_wjuncld_dn2 = assign90230_e138998_d_n2;
        locals.var_wjuncld_dn4 = assign90230_e138998_d_n4;
        locals.var_wjuncld_dn5 = assign90230_e138998_d_n5;
        locals.var_wjuncld_dn6 = assign90230_e138998_d_n6;
        locals.var_wjuncld_dn7 = assign90230_e138998_d_n7;
        locals.var_wjuncld_dn8 = assign90230_e138998_d_n8;
        locals.var_wjuncld_dn9 = assign90230_e138998_d_n9;
        locals.var_wjuncld_dn10 = assign90230_e138998_d_n10;
        locals.var_wjuncld_dn11 = assign90230_e138998_d_n11;
        locals.var_wjuncld_dn14 = assign90230_e138998_d_n14;
        locals.var_wjuncld_rv = 0.0;

        let (assign90240_e139008, assign90240_e139008_d_n0, assign90240_e139008_d_n2, assign90240_e139008_d_n4, assign90240_e139008_d_n5, assign90240_e139008_d_n6, assign90240_e139008_d_n7, assign90240_e139008_d_n8, assign90240_e139008_d_n9, assign90240_e139008_d_n10, assign90240_e139008_d_n11, assign90240_e139008_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2102 != 0.0)) && (locals.var_guard2103 != 0.0)) {
        let assign90240_e139006: f64 = (locals.var_lover_func - locals.var_wjuncld);
        (assign90240_e139006, (locals.var_lover_func_dn0 - locals.var_wjuncld_dn0), (locals.var_lover_func_dn2 - locals.var_wjuncld_dn2), (locals.var_lover_func_dn4 - locals.var_wjuncld_dn4), (locals.var_lover_func_dn5 - locals.var_wjuncld_dn5), (locals.var_lover_func_dn6 - locals.var_wjuncld_dn6), (locals.var_lover_func_dn7 - locals.var_wjuncld_dn7), (locals.var_lover_func_dn8 - locals.var_wjuncld_dn8), (locals.var_lover_func_dn9 - locals.var_wjuncld_dn9), (locals.var_lover_func_dn10 - locals.var_wjuncld_dn10), (locals.var_lover_func_dn11 - locals.var_wjuncld_dn11), (locals.var_lover_func_dn14 - locals.var_wjuncld_dn14),)
    } else {
        (locals.var_lover_func, locals.var_lover_func_dn0, locals.var_lover_func_dn2, locals.var_lover_func_dn4, locals.var_lover_func_dn5, locals.var_lover_func_dn6, locals.var_lover_func_dn7, locals.var_lover_func_dn8, locals.var_lover_func_dn9, locals.var_lover_func_dn10, locals.var_lover_func_dn11, locals.var_lover_func_dn14,)
    }
};
        locals.var_lover_func = assign90240_e139008;
        locals.var_lover_func_dn0 = assign90240_e139008_d_n0;
        locals.var_lover_func_dn2 = assign90240_e139008_d_n2;
        locals.var_lover_func_dn4 = assign90240_e139008_d_n4;
        locals.var_lover_func_dn5 = assign90240_e139008_d_n5;
        locals.var_lover_func_dn6 = assign90240_e139008_d_n6;
        locals.var_lover_func_dn7 = assign90240_e139008_d_n7;
        locals.var_lover_func_dn8 = assign90240_e139008_d_n8;
        locals.var_lover_func_dn9 = assign90240_e139008_d_n9;
        locals.var_lover_func_dn10 = assign90240_e139008_d_n10;
        locals.var_lover_func_dn11 = assign90240_e139008_d_n11;
        locals.var_lover_func_dn14 = assign90240_e139008_d_n14;
        locals.var_lover_func_rv = 0.0;

        let assign90250_e139011: f64 = if 4.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2106 = assign90250_e139011;
        locals.var_guard2106_rv = 0.0;

        let assign90260_e139014: f64 = if 4.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard2107 = assign90260_e139014;
        locals.var_guard2107_rv = 0.0;

        let assign90270_e139017: f64 = if 4.0 == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard2108 = assign90270_e139017;
        locals.var_guard2108_rv = 0.0;

        let assign90280_e139020: f64 = if 4.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard2109 = assign90280_e139020;
        locals.var_guard2109_rv = 0.0;

        let assign90290_e139023: f64 = if p.p55 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2110 = assign90290_e139023;
        locals.var_guard2110_rv = 0.0;

        let (assign90300_e139033, assign90300_e139033_d_n0, assign90300_e139033_d_n2, assign90300_e139033_d_n4, assign90300_e139033_d_n5, assign90300_e139033_d_n6, assign90300_e139033_d_n7, assign90300_e139033_d_n8, assign90300_e139033_d_n9, assign90300_e139033_d_n10, assign90300_e139033_d_n11, assign90300_e139033_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2106 != 0.0)) && (locals.var_guard2110 != 0.0)) {
        let assign90300_e139031: f64 = (locals.var_weffcv_nf * locals.var_lover_func);
        (assign90300_e139031, (locals.var_weffcv_nf * locals.var_lover_func_dn0), (locals.var_weffcv_nf * locals.var_lover_func_dn2), (locals.var_weffcv_nf * locals.var_lover_func_dn4), (locals.var_weffcv_nf * locals.var_lover_func_dn5), (locals.var_weffcv_nf * locals.var_lover_func_dn6), (locals.var_weffcv_nf * locals.var_lover_func_dn7), (locals.var_weffcv_nf * locals.var_lover_func_dn8), (locals.var_weffcv_nf * locals.var_lover_func_dn9), (locals.var_weffcv_nf * locals.var_lover_func_dn10), (locals.var_weffcv_nf * locals.var_lover_func_dn11), (locals.var_weffcv_nf * locals.var_lover_func_dn14),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign90300_e139033;
        locals.var_t4_dn0 = assign90300_e139033_d_n0;
        locals.var_t4_dn2 = assign90300_e139033_d_n2;
        locals.var_t4_dn4 = assign90300_e139033_d_n4;
        locals.var_t4_dn5 = assign90300_e139033_d_n5;
        locals.var_t4_dn6 = assign90300_e139033_d_n6;
        locals.var_t4_dn7 = assign90300_e139033_d_n7;
        locals.var_t4_dn8 = assign90300_e139033_d_n8;
        locals.var_t4_dn9 = assign90300_e139033_d_n9;
        locals.var_t4_dn10 = assign90300_e139033_d_n10;
        locals.var_t4_dn11 = assign90300_e139033_d_n11;
        locals.var_t4_dn14 = assign90300_e139033_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign90310_e139048, assign90310_e139048_d_n0, assign90310_e139048_d_n2, assign90310_e139048_d_n4, assign90310_e139048_d_n5, assign90310_e139048_d_n6, assign90310_e139048_d_n7, assign90310_e139048_d_n8, assign90310_e139048_d_n9, assign90310_e139048_d_n10, assign90310_e139048_d_n11, assign90310_e139048_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2106 != 0.0)) && (locals.var_guard2110 == 0.0)) {
        let assign90310_e139042: f64 = (locals.var_weffcv_nf * locals.var_lover_func);
        let assign90310_e139045: f64 = (1.0 - locals.var_uc_cvdsover);
        let assign90310_e139046: f64 = (assign90310_e139042 * assign90310_e139045);
        (assign90310_e139046, ((locals.var_weffcv_nf * locals.var_lover_func_dn0) * assign90310_e139045), ((locals.var_weffcv_nf * locals.var_lover_func_dn2) * assign90310_e139045), ((locals.var_weffcv_nf * locals.var_lover_func_dn4) * assign90310_e139045), ((locals.var_weffcv_nf * locals.var_lover_func_dn5) * assign90310_e139045), ((locals.var_weffcv_nf * locals.var_lover_func_dn6) * assign90310_e139045), ((locals.var_weffcv_nf * locals.var_lover_func_dn7) * assign90310_e139045), ((locals.var_weffcv_nf * locals.var_lover_func_dn8) * assign90310_e139045), ((locals.var_weffcv_nf * locals.var_lover_func_dn9) * assign90310_e139045), ((locals.var_weffcv_nf * locals.var_lover_func_dn10) * assign90310_e139045), ((locals.var_weffcv_nf * locals.var_lover_func_dn11) * assign90310_e139045), ((locals.var_weffcv_nf * locals.var_lover_func_dn14) * assign90310_e139045),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign90310_e139048;
        locals.var_t4_dn0 = assign90310_e139048_d_n0;
        locals.var_t4_dn2 = assign90310_e139048_d_n2;
        locals.var_t4_dn4 = assign90310_e139048_d_n4;
        locals.var_t4_dn5 = assign90310_e139048_d_n5;
        locals.var_t4_dn6 = assign90310_e139048_d_n6;
        locals.var_t4_dn7 = assign90310_e139048_d_n7;
        locals.var_t4_dn8 = assign90310_e139048_d_n8;
        locals.var_t4_dn9 = assign90310_e139048_d_n9;
        locals.var_t4_dn10 = assign90310_e139048_d_n10;
        locals.var_t4_dn11 = assign90310_e139048_d_n11;
        locals.var_t4_dn14 = assign90310_e139048_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign90320_e139056, assign90320_e139056_d_n0, assign90320_e139056_d_n2, assign90320_e139056_d_n4, assign90320_e139056_d_n5, assign90320_e139056_d_n6, assign90320_e139056_d_n7, assign90320_e139056_d_n8, assign90320_e139056_d_n9, assign90320_e139056_d_n10, assign90320_e139056_d_n11, assign90320_e139056_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2106 != 0.0)) {
        let assign90320_e139054: f64 = (locals.var_t4 * locals.var_qsuld);
        (assign90320_e139054, ((locals.var_t4_dn0 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn0)), ((locals.var_t4_dn2 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn2)), ((locals.var_t4_dn4 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn4)), ((locals.var_t4_dn5 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn5)), ((locals.var_t4_dn6 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn6)), ((locals.var_t4_dn7 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn7)), ((locals.var_t4_dn8 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn8)), ((locals.var_t4_dn9 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn9)), ((locals.var_t4_dn10 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn10)), ((locals.var_t4_dn11 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn11)), ((locals.var_t4_dn14 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn14)),)
    } else {
        (locals.var_qovs, locals.var_qovs_dn0, locals.var_qovs_dn2, locals.var_qovs_dn4, locals.var_qovs_dn5, locals.var_qovs_dn6, locals.var_qovs_dn7, locals.var_qovs_dn8, locals.var_qovs_dn9, locals.var_qovs_dn10, locals.var_qovs_dn11, locals.var_qovs_dn14,)
    }
};
        locals.var_qovs = assign90320_e139056;
        locals.var_qovs_dn0 = assign90320_e139056_d_n0;
        locals.var_qovs_dn2 = assign90320_e139056_d_n2;
        locals.var_qovs_dn4 = assign90320_e139056_d_n4;
        locals.var_qovs_dn5 = assign90320_e139056_d_n5;
        locals.var_qovs_dn6 = assign90320_e139056_d_n6;
        locals.var_qovs_dn7 = assign90320_e139056_d_n7;
        locals.var_qovs_dn8 = assign90320_e139056_d_n8;
        locals.var_qovs_dn9 = assign90320_e139056_d_n9;
        locals.var_qovs_dn10 = assign90320_e139056_d_n10;
        locals.var_qovs_dn11 = assign90320_e139056_d_n11;
        locals.var_qovs_dn14 = assign90320_e139056_d_n14;
        locals.var_qovs_rv = 0.0;

        let (assign90330_e139064, assign90330_e139064_d_n0, assign90330_e139064_d_n2, assign90330_e139064_d_n4, assign90330_e139064_d_n5, assign90330_e139064_d_n6, assign90330_e139064_d_n7, assign90330_e139064_d_n8, assign90330_e139064_d_n9, assign90330_e139064_d_n10, assign90330_e139064_d_n11, assign90330_e139064_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2106 != 0.0)) {
        let assign90330_e139062: f64 = (locals.var_t4 * locals.var_qbuld);
        (assign90330_e139062, ((locals.var_t4_dn0 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn0)), ((locals.var_t4_dn2 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn2)), ((locals.var_t4_dn4 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn4)), ((locals.var_t4_dn5 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn5)), ((locals.var_t4_dn6 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn6)), ((locals.var_t4_dn7 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn7)), ((locals.var_t4_dn8 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn8)), ((locals.var_t4_dn9 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn9)), ((locals.var_t4_dn10 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn10)), ((locals.var_t4_dn11 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn11)), ((locals.var_t4_dn14 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn14)),)
    } else {
        (locals.var_qbsld, locals.var_qbsld_dn0, locals.var_qbsld_dn2, locals.var_qbsld_dn4, locals.var_qbsld_dn5, locals.var_qbsld_dn6, locals.var_qbsld_dn7, locals.var_qbsld_dn8, locals.var_qbsld_dn9, locals.var_qbsld_dn10, locals.var_qbsld_dn11, locals.var_qbsld_dn14,)
    }
};
        locals.var_qbsld = assign90330_e139064;
        locals.var_qbsld_dn0 = assign90330_e139064_d_n0;
        locals.var_qbsld_dn2 = assign90330_e139064_d_n2;
        locals.var_qbsld_dn4 = assign90330_e139064_d_n4;
        locals.var_qbsld_dn5 = assign90330_e139064_d_n5;
        locals.var_qbsld_dn6 = assign90330_e139064_d_n6;
        locals.var_qbsld_dn7 = assign90330_e139064_d_n7;
        locals.var_qbsld_dn8 = assign90330_e139064_d_n8;
        locals.var_qbsld_dn9 = assign90330_e139064_d_n9;
        locals.var_qbsld_dn10 = assign90330_e139064_d_n10;
        locals.var_qbsld_dn11 = assign90330_e139064_d_n11;
        locals.var_qbsld_dn14 = assign90330_e139064_d_n14;
        locals.var_qbsld_rv = 0.0;

        let (assign90360_e139089, assign90360_e139089_d_n0, assign90360_e139089_d_n2, assign90360_e139089_d_n4, assign90360_e139089_d_n5, assign90360_e139089_d_n6, assign90360_e139089_d_n7, assign90360_e139089_d_n8, assign90360_e139089_d_n9, assign90360_e139089_d_n10, assign90360_e139089_d_n11, assign90360_e139089_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard2107 != 0.0) && (locals.var_guard2106 == 0.0))) {
        let assign90360_e139085: f64 = (locals.var_weffcv_nf * locals.var_lover_func);
        let assign90360_e139087: f64 = (assign90360_e139085 * locals.var_uc_cvdsover);
        (assign90360_e139087, ((locals.var_weffcv_nf * locals.var_lover_func_dn0) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn2) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn4) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn5) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn6) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn7) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn8) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn9) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn10) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn11) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn14) * locals.var_uc_cvdsover),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign90360_e139089;
        locals.var_t4_dn0 = assign90360_e139089_d_n0;
        locals.var_t4_dn2 = assign90360_e139089_d_n2;
        locals.var_t4_dn4 = assign90360_e139089_d_n4;
        locals.var_t4_dn5 = assign90360_e139089_d_n5;
        locals.var_t4_dn6 = assign90360_e139089_d_n6;
        locals.var_t4_dn7 = assign90360_e139089_d_n7;
        locals.var_t4_dn8 = assign90360_e139089_d_n8;
        locals.var_t4_dn9 = assign90360_e139089_d_n9;
        locals.var_t4_dn10 = assign90360_e139089_d_n10;
        locals.var_t4_dn11 = assign90360_e139089_d_n11;
        locals.var_t4_dn14 = assign90360_e139089_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign90370_e139100, assign90370_e139100_d_n0, assign90370_e139100_d_n2, assign90370_e139100_d_n4, assign90370_e139100_d_n5, assign90370_e139100_d_n6, assign90370_e139100_d_n7, assign90370_e139100_d_n8, assign90370_e139100_d_n9, assign90370_e139100_d_n10, assign90370_e139100_d_n11, assign90370_e139100_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard2107 != 0.0) && (locals.var_guard2106 == 0.0))) {
        let assign90370_e139098: f64 = (locals.var_t4 * locals.var_qsuld);
        (assign90370_e139098, ((locals.var_t4_dn0 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn0)), ((locals.var_t4_dn2 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn2)), ((locals.var_t4_dn4 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn4)), ((locals.var_t4_dn5 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn5)), ((locals.var_t4_dn6 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn6)), ((locals.var_t4_dn7 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn7)), ((locals.var_t4_dn8 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn8)), ((locals.var_t4_dn9 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn9)), ((locals.var_t4_dn10 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn10)), ((locals.var_t4_dn11 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn11)), ((locals.var_t4_dn14 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn14)),)
    } else {
        (locals.var_qovsext, locals.var_qovsext_dn0, locals.var_qovsext_dn2, locals.var_qovsext_dn4, locals.var_qovsext_dn5, locals.var_qovsext_dn6, locals.var_qovsext_dn7, locals.var_qovsext_dn8, locals.var_qovsext_dn9, locals.var_qovsext_dn10, locals.var_qovsext_dn11, locals.var_qovsext_dn14,)
    }
};
        locals.var_qovsext = assign90370_e139100;
        locals.var_qovsext_dn0 = assign90370_e139100_d_n0;
        locals.var_qovsext_dn2 = assign90370_e139100_d_n2;
        locals.var_qovsext_dn4 = assign90370_e139100_d_n4;
        locals.var_qovsext_dn5 = assign90370_e139100_d_n5;
        locals.var_qovsext_dn6 = assign90370_e139100_d_n6;
        locals.var_qovsext_dn7 = assign90370_e139100_d_n7;
        locals.var_qovsext_dn8 = assign90370_e139100_d_n8;
        locals.var_qovsext_dn9 = assign90370_e139100_d_n9;
        locals.var_qovsext_dn10 = assign90370_e139100_d_n10;
        locals.var_qovsext_dn11 = assign90370_e139100_d_n11;
        locals.var_qovsext_dn14 = assign90370_e139100_d_n14;
        locals.var_qovsext_rv = 0.0;

        let (assign90380_e139111, assign90380_e139111_d_n0, assign90380_e139111_d_n2, assign90380_e139111_d_n4, assign90380_e139111_d_n5, assign90380_e139111_d_n6, assign90380_e139111_d_n7, assign90380_e139111_d_n8, assign90380_e139111_d_n9, assign90380_e139111_d_n10, assign90380_e139111_d_n11, assign90380_e139111_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard2107 != 0.0) && (locals.var_guard2106 == 0.0))) {
        let assign90380_e139109: f64 = (locals.var_t4 * locals.var_qbuld);
        (assign90380_e139109, ((locals.var_t4_dn0 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn0)), ((locals.var_t4_dn2 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn2)), ((locals.var_t4_dn4 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn4)), ((locals.var_t4_dn5 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn5)), ((locals.var_t4_dn6 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn6)), ((locals.var_t4_dn7 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn7)), ((locals.var_t4_dn8 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn8)), ((locals.var_t4_dn9 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn9)), ((locals.var_t4_dn10 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn10)), ((locals.var_t4_dn11 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn11)), ((locals.var_t4_dn14 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn14)),)
    } else {
        (locals.var_qbsldext, locals.var_qbsldext_dn0, locals.var_qbsldext_dn2, locals.var_qbsldext_dn4, locals.var_qbsldext_dn5, locals.var_qbsldext_dn6, locals.var_qbsldext_dn7, locals.var_qbsldext_dn8, locals.var_qbsldext_dn9, locals.var_qbsldext_dn10, locals.var_qbsldext_dn11, locals.var_qbsldext_dn14,)
    }
};
        locals.var_qbsldext = assign90380_e139111;
        locals.var_qbsldext_dn0 = assign90380_e139111_d_n0;
        locals.var_qbsldext_dn2 = assign90380_e139111_d_n2;
        locals.var_qbsldext_dn4 = assign90380_e139111_d_n4;
        locals.var_qbsldext_dn5 = assign90380_e139111_d_n5;
        locals.var_qbsldext_dn6 = assign90380_e139111_d_n6;
        locals.var_qbsldext_dn7 = assign90380_e139111_d_n7;
        locals.var_qbsldext_dn8 = assign90380_e139111_d_n8;
        locals.var_qbsldext_dn9 = assign90380_e139111_d_n9;
        locals.var_qbsldext_dn10 = assign90380_e139111_d_n10;
        locals.var_qbsldext_dn11 = assign90380_e139111_d_n11;
        locals.var_qbsldext_dn14 = assign90380_e139111_d_n14;
        locals.var_qbsldext_rv = 0.0;

        let assign90390_e139114: f64 = if p.p55 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2111 = assign90390_e139114;
        locals.var_guard2111_rv = 0.0;

        let (assign90400_e139129, assign90400_e139129_d_n0, assign90400_e139129_d_n2, assign90400_e139129_d_n4, assign90400_e139129_d_n5, assign90400_e139129_d_n6, assign90400_e139129_d_n7, assign90400_e139129_d_n8, assign90400_e139129_d_n9, assign90400_e139129_d_n10, assign90400_e139129_d_n11, assign90400_e139129_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && ((locals.var_guard2108 != 0.0) && (!((locals.var_guard2106 != 0.0) || (locals.var_guard2107 != 0.0))))) && (locals.var_guard2111 != 0.0)) {
        let assign90400_e139127: f64 = (locals.var_weffcv_nf * locals.var_lover_func);
        (assign90400_e139127, (locals.var_weffcv_nf * locals.var_lover_func_dn0), (locals.var_weffcv_nf * locals.var_lover_func_dn2), (locals.var_weffcv_nf * locals.var_lover_func_dn4), (locals.var_weffcv_nf * locals.var_lover_func_dn5), (locals.var_weffcv_nf * locals.var_lover_func_dn6), (locals.var_weffcv_nf * locals.var_lover_func_dn7), (locals.var_weffcv_nf * locals.var_lover_func_dn8), (locals.var_weffcv_nf * locals.var_lover_func_dn9), (locals.var_weffcv_nf * locals.var_lover_func_dn10), (locals.var_weffcv_nf * locals.var_lover_func_dn11), (locals.var_weffcv_nf * locals.var_lover_func_dn14),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign90400_e139129;
        locals.var_t4_dn0 = assign90400_e139129_d_n0;
        locals.var_t4_dn2 = assign90400_e139129_d_n2;
        locals.var_t4_dn4 = assign90400_e139129_d_n4;
        locals.var_t4_dn5 = assign90400_e139129_d_n5;
        locals.var_t4_dn6 = assign90400_e139129_d_n6;
        locals.var_t4_dn7 = assign90400_e139129_d_n7;
        locals.var_t4_dn8 = assign90400_e139129_d_n8;
        locals.var_t4_dn9 = assign90400_e139129_d_n9;
        locals.var_t4_dn10 = assign90400_e139129_d_n10;
        locals.var_t4_dn11 = assign90400_e139129_d_n11;
        locals.var_t4_dn14 = assign90400_e139129_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign90410_e139149, assign90410_e139149_d_n0, assign90410_e139149_d_n2, assign90410_e139149_d_n4, assign90410_e139149_d_n5, assign90410_e139149_d_n6, assign90410_e139149_d_n7, assign90410_e139149_d_n8, assign90410_e139149_d_n9, assign90410_e139149_d_n10, assign90410_e139149_d_n11, assign90410_e139149_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && ((locals.var_guard2108 != 0.0) && (!((locals.var_guard2106 != 0.0) || (locals.var_guard2107 != 0.0))))) && (locals.var_guard2111 == 0.0)) {
        let assign90410_e139143: f64 = (locals.var_weffcv_nf * locals.var_lover_func);
        let assign90410_e139146: f64 = (1.0 - locals.var_uc_cvdsover);
        let assign90410_e139147: f64 = (assign90410_e139143 * assign90410_e139146);
        (assign90410_e139147, ((locals.var_weffcv_nf * locals.var_lover_func_dn0) * assign90410_e139146), ((locals.var_weffcv_nf * locals.var_lover_func_dn2) * assign90410_e139146), ((locals.var_weffcv_nf * locals.var_lover_func_dn4) * assign90410_e139146), ((locals.var_weffcv_nf * locals.var_lover_func_dn5) * assign90410_e139146), ((locals.var_weffcv_nf * locals.var_lover_func_dn6) * assign90410_e139146), ((locals.var_weffcv_nf * locals.var_lover_func_dn7) * assign90410_e139146), ((locals.var_weffcv_nf * locals.var_lover_func_dn8) * assign90410_e139146), ((locals.var_weffcv_nf * locals.var_lover_func_dn9) * assign90410_e139146), ((locals.var_weffcv_nf * locals.var_lover_func_dn10) * assign90410_e139146), ((locals.var_weffcv_nf * locals.var_lover_func_dn11) * assign90410_e139146), ((locals.var_weffcv_nf * locals.var_lover_func_dn14) * assign90410_e139146),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign90410_e139149;
        locals.var_t4_dn0 = assign90410_e139149_d_n0;
        locals.var_t4_dn2 = assign90410_e139149_d_n2;
        locals.var_t4_dn4 = assign90410_e139149_d_n4;
        locals.var_t4_dn5 = assign90410_e139149_d_n5;
        locals.var_t4_dn6 = assign90410_e139149_d_n6;
        locals.var_t4_dn7 = assign90410_e139149_d_n7;
        locals.var_t4_dn8 = assign90410_e139149_d_n8;
        locals.var_t4_dn9 = assign90410_e139149_d_n9;
        locals.var_t4_dn10 = assign90410_e139149_d_n10;
        locals.var_t4_dn11 = assign90410_e139149_d_n11;
        locals.var_t4_dn14 = assign90410_e139149_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign90420_e139160, assign90420_e139160_d_n0, assign90420_e139160_d_n2, assign90420_e139160_d_n4, assign90420_e139160_d_n5, assign90420_e139160_d_n6, assign90420_e139160_d_n7, assign90420_e139160_d_n8, assign90420_e139160_d_n9, assign90420_e139160_d_n10, assign90420_e139160_d_n11, assign90420_e139160_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard2108 != 0.0) && (!((locals.var_guard2106 != 0.0) || (locals.var_guard2107 != 0.0))))) {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn14,)
    } else {
        (locals.var_rd_ps0ld, locals.var_rd_ps0ld_dn0, locals.var_rd_ps0ld_dn2, locals.var_rd_ps0ld_dn4, locals.var_rd_ps0ld_dn5, locals.var_rd_ps0ld_dn6, locals.var_rd_ps0ld_dn7, locals.var_rd_ps0ld_dn8, locals.var_rd_ps0ld_dn9, locals.var_rd_ps0ld_dn10, locals.var_rd_ps0ld_dn11, locals.var_rd_ps0ld_dn14,)
    }
};
        locals.var_rd_ps0ld = assign90420_e139160;
        locals.var_rd_ps0ld_dn0 = assign90420_e139160_d_n0;
        locals.var_rd_ps0ld_dn2 = assign90420_e139160_d_n2;
        locals.var_rd_ps0ld_dn4 = assign90420_e139160_d_n4;
        locals.var_rd_ps0ld_dn5 = assign90420_e139160_d_n5;
        locals.var_rd_ps0ld_dn6 = assign90420_e139160_d_n6;
        locals.var_rd_ps0ld_dn7 = assign90420_e139160_d_n7;
        locals.var_rd_ps0ld_dn8 = assign90420_e139160_d_n8;
        locals.var_rd_ps0ld_dn9 = assign90420_e139160_d_n9;
        locals.var_rd_ps0ld_dn10 = assign90420_e139160_d_n10;
        locals.var_rd_ps0ld_dn11 = assign90420_e139160_d_n11;
        locals.var_rd_ps0ld_dn14 = assign90420_e139160_d_n14;
        locals.var_rd_ps0ld_rv = 0.0;

        let assign90430_e139163: f64 = if p.p430 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2112 = assign90430_e139163;
        locals.var_guard2112_rv = 0.0;

        let (assign90440_e139176, assign90440_e139176_d_n0, assign90440_e139176_d_n2, assign90440_e139176_d_n4, assign90440_e139176_d_n5, assign90440_e139176_d_n6, assign90440_e139176_d_n7, assign90440_e139176_d_n8, assign90440_e139176_d_n9, assign90440_e139176_d_n10, assign90440_e139176_d_n11, assign90440_e139176_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && ((locals.var_guard2108 != 0.0) && (!((locals.var_guard2106 != 0.0) || (locals.var_guard2107 != 0.0))))) && (locals.var_guard2112 != 0.0)) {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn8, locals.var_qbuld_dn9, locals.var_qbuld_dn10, locals.var_qbuld_dn11, locals.var_qbuld_dn14,)
    } else {
        (locals.var_rd_qbuld, locals.var_rd_qbuld_dn0, locals.var_rd_qbuld_dn2, locals.var_rd_qbuld_dn4, locals.var_rd_qbuld_dn5, locals.var_rd_qbuld_dn6, locals.var_rd_qbuld_dn7, locals.var_rd_qbuld_dn8, locals.var_rd_qbuld_dn9, locals.var_rd_qbuld_dn10, locals.var_rd_qbuld_dn11, locals.var_rd_qbuld_dn14,)
    }
};
        locals.var_rd_qbuld = assign90440_e139176;
        locals.var_rd_qbuld_dn0 = assign90440_e139176_d_n0;
        locals.var_rd_qbuld_dn2 = assign90440_e139176_d_n2;
        locals.var_rd_qbuld_dn4 = assign90440_e139176_d_n4;
        locals.var_rd_qbuld_dn5 = assign90440_e139176_d_n5;
        locals.var_rd_qbuld_dn6 = assign90440_e139176_d_n6;
        locals.var_rd_qbuld_dn7 = assign90440_e139176_d_n7;
        locals.var_rd_qbuld_dn8 = assign90440_e139176_d_n8;
        locals.var_rd_qbuld_dn9 = assign90440_e139176_d_n9;
        locals.var_rd_qbuld_dn10 = assign90440_e139176_d_n10;
        locals.var_rd_qbuld_dn11 = assign90440_e139176_d_n11;
        locals.var_rd_qbuld_dn14 = assign90440_e139176_d_n14;
        locals.var_rd_qbuld_rv = 0.0;

        let (assign90450_e139189, assign90450_e139189_d_n0, assign90450_e139189_d_n2, assign90450_e139189_d_n4, assign90450_e139189_d_n5, assign90450_e139189_d_n6, assign90450_e139189_d_n7, assign90450_e139189_d_n8, assign90450_e139189_d_n9, assign90450_e139189_d_n10, assign90450_e139189_d_n11, assign90450_e139189_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard2108 != 0.0) && (!((locals.var_guard2106 != 0.0) || (locals.var_guard2107 != 0.0))))) {
        let assign90450_e139187: f64 = (locals.var_t4 * locals.var_qsuld);
        (assign90450_e139187, ((locals.var_t4_dn0 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn0)), ((locals.var_t4_dn2 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn2)), ((locals.var_t4_dn4 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn4)), ((locals.var_t4_dn5 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn5)), ((locals.var_t4_dn6 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn6)), ((locals.var_t4_dn7 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn7)), ((locals.var_t4_dn8 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn8)), ((locals.var_t4_dn9 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn9)), ((locals.var_t4_dn10 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn10)), ((locals.var_t4_dn11 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn11)), ((locals.var_t4_dn14 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn14)),)
    } else {
        (locals.var_qovd, locals.var_qovd_dn0, locals.var_qovd_dn2, locals.var_qovd_dn4, locals.var_qovd_dn5, locals.var_qovd_dn6, locals.var_qovd_dn7, locals.var_qovd_dn8, locals.var_qovd_dn9, locals.var_qovd_dn10, locals.var_qovd_dn11, locals.var_qovd_dn14,)
    }
};
        locals.var_qovd = assign90450_e139189;
        locals.var_qovd_dn0 = assign90450_e139189_d_n0;
        locals.var_qovd_dn2 = assign90450_e139189_d_n2;
        locals.var_qovd_dn4 = assign90450_e139189_d_n4;
        locals.var_qovd_dn5 = assign90450_e139189_d_n5;
        locals.var_qovd_dn6 = assign90450_e139189_d_n6;
        locals.var_qovd_dn7 = assign90450_e139189_d_n7;
        locals.var_qovd_dn8 = assign90450_e139189_d_n8;
        locals.var_qovd_dn9 = assign90450_e139189_d_n9;
        locals.var_qovd_dn10 = assign90450_e139189_d_n10;
        locals.var_qovd_dn11 = assign90450_e139189_d_n11;
        locals.var_qovd_dn14 = assign90450_e139189_d_n14;
        locals.var_qovd_rv = 0.0;

        let (assign90460_e139202, assign90460_e139202_d_n0, assign90460_e139202_d_n2, assign90460_e139202_d_n4, assign90460_e139202_d_n5, assign90460_e139202_d_n6, assign90460_e139202_d_n7, assign90460_e139202_d_n8, assign90460_e139202_d_n9, assign90460_e139202_d_n10, assign90460_e139202_d_n11, assign90460_e139202_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard2108 != 0.0) && (!((locals.var_guard2106 != 0.0) || (locals.var_guard2107 != 0.0))))) {
        let assign90460_e139200: f64 = (locals.var_t4 * locals.var_qbuld);
        (assign90460_e139200, ((locals.var_t4_dn0 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn0)), ((locals.var_t4_dn2 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn2)), ((locals.var_t4_dn4 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn4)), ((locals.var_t4_dn5 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn5)), ((locals.var_t4_dn6 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn6)), ((locals.var_t4_dn7 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn7)), ((locals.var_t4_dn8 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn8)), ((locals.var_t4_dn9 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn9)), ((locals.var_t4_dn10 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn10)), ((locals.var_t4_dn11 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn11)), ((locals.var_t4_dn14 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn14)),)
    } else {
        (locals.var_qbdld, locals.var_qbdld_dn0, locals.var_qbdld_dn2, locals.var_qbdld_dn4, locals.var_qbdld_dn5, locals.var_qbdld_dn6, locals.var_qbdld_dn7, locals.var_qbdld_dn8, locals.var_qbdld_dn9, locals.var_qbdld_dn10, locals.var_qbdld_dn11, locals.var_qbdld_dn14,)
    }
};
        locals.var_qbdld = assign90460_e139202;
        locals.var_qbdld_dn0 = assign90460_e139202_d_n0;
        locals.var_qbdld_dn2 = assign90460_e139202_d_n2;
        locals.var_qbdld_dn4 = assign90460_e139202_d_n4;
        locals.var_qbdld_dn5 = assign90460_e139202_d_n5;
        locals.var_qbdld_dn6 = assign90460_e139202_d_n6;
        locals.var_qbdld_dn7 = assign90460_e139202_d_n7;
        locals.var_qbdld_dn8 = assign90460_e139202_d_n8;
        locals.var_qbdld_dn9 = assign90460_e139202_d_n9;
        locals.var_qbdld_dn10 = assign90460_e139202_d_n10;
        locals.var_qbdld_dn11 = assign90460_e139202_d_n11;
        locals.var_qbdld_dn14 = assign90460_e139202_d_n14;
        locals.var_qbdld_rv = 0.0;

        let (assign90470_e139213, assign90470_e139213_d_n0, assign90470_e139213_d_n2, assign90470_e139213_d_n4, assign90470_e139213_d_n5, assign90470_e139213_d_n6, assign90470_e139213_d_n7, assign90470_e139213_d_n8, assign90470_e139213_d_n9, assign90470_e139213_d_n10, assign90470_e139213_d_n11, assign90470_e139213_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard2108 != 0.0) && (!((locals.var_guard2106 != 0.0) || (locals.var_guard2107 != 0.0))))) {
        (locals.var_qbdld, locals.var_qbdld_dn0, locals.var_qbdld_dn2, locals.var_qbdld_dn4, locals.var_qbdld_dn5, locals.var_qbdld_dn6, locals.var_qbdld_dn7, locals.var_qbdld_dn8, locals.var_qbdld_dn9, locals.var_qbdld_dn10, locals.var_qbdld_dn11, locals.var_qbdld_dn14,)
    } else {
        (locals.var_qbd_qs, locals.var_qbd_qs_dn0, locals.var_qbd_qs_dn2, locals.var_qbd_qs_dn4, locals.var_qbd_qs_dn5, locals.var_qbd_qs_dn6, locals.var_qbd_qs_dn7, locals.var_qbd_qs_dn8, locals.var_qbd_qs_dn9, locals.var_qbd_qs_dn10, locals.var_qbd_qs_dn11, locals.var_qbd_qs_dn14,)
    }
};
        locals.var_qbd_qs = assign90470_e139213;
        locals.var_qbd_qs_dn0 = assign90470_e139213_d_n0;
        locals.var_qbd_qs_dn2 = assign90470_e139213_d_n2;
        locals.var_qbd_qs_dn4 = assign90470_e139213_d_n4;
        locals.var_qbd_qs_dn5 = assign90470_e139213_d_n5;
        locals.var_qbd_qs_dn6 = assign90470_e139213_d_n6;
        locals.var_qbd_qs_dn7 = assign90470_e139213_d_n7;
        locals.var_qbd_qs_dn8 = assign90470_e139213_d_n8;
        locals.var_qbd_qs_dn9 = assign90470_e139213_d_n9;
        locals.var_qbd_qs_dn10 = assign90470_e139213_d_n10;
        locals.var_qbd_qs_dn11 = assign90470_e139213_d_n11;
        locals.var_qbd_qs_dn14 = assign90470_e139213_d_n14;
        locals.var_qbd_qs_rv = 0.0;

        let (assign90480_e139230, assign90480_e139230_d_n0, assign90480_e139230_d_n2, assign90480_e139230_d_n4, assign90480_e139230_d_n5, assign90480_e139230_d_n6, assign90480_e139230_d_n7, assign90480_e139230_d_n8, assign90480_e139230_d_n9, assign90480_e139230_d_n10, assign90480_e139230_d_n11, assign90480_e139230_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard2109 != 0.0) && (!(((locals.var_guard2106 != 0.0) || (locals.var_guard2107 != 0.0)) || (locals.var_guard2108 != 0.0))))) {
        let assign90480_e139226: f64 = (locals.var_weffcv_nf * locals.var_lover_func);
        let assign90480_e139228: f64 = (assign90480_e139226 * locals.var_uc_cvdsover);
        (assign90480_e139228, ((locals.var_weffcv_nf * locals.var_lover_func_dn0) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn2) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn4) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn5) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn6) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn7) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn8) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn9) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn10) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn11) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn14) * locals.var_uc_cvdsover),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign90480_e139230;
        locals.var_t4_dn0 = assign90480_e139230_d_n0;
        locals.var_t4_dn2 = assign90480_e139230_d_n2;
        locals.var_t4_dn4 = assign90480_e139230_d_n4;
        locals.var_t4_dn5 = assign90480_e139230_d_n5;
        locals.var_t4_dn6 = assign90480_e139230_d_n6;
        locals.var_t4_dn7 = assign90480_e139230_d_n7;
        locals.var_t4_dn8 = assign90480_e139230_d_n8;
        locals.var_t4_dn9 = assign90480_e139230_d_n9;
        locals.var_t4_dn10 = assign90480_e139230_d_n10;
        locals.var_t4_dn11 = assign90480_e139230_d_n11;
        locals.var_t4_dn14 = assign90480_e139230_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign90490_e139245, assign90490_e139245_d_n0, assign90490_e139245_d_n2, assign90490_e139245_d_n4, assign90490_e139245_d_n5, assign90490_e139245_d_n6, assign90490_e139245_d_n7, assign90490_e139245_d_n8, assign90490_e139245_d_n9, assign90490_e139245_d_n10, assign90490_e139245_d_n11, assign90490_e139245_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard2109 != 0.0) && (!(((locals.var_guard2106 != 0.0) || (locals.var_guard2107 != 0.0)) || (locals.var_guard2108 != 0.0))))) {
        let assign90490_e139243: f64 = (locals.var_t4 * locals.var_qsuld);
        (assign90490_e139243, ((locals.var_t4_dn0 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn0)), ((locals.var_t4_dn2 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn2)), ((locals.var_t4_dn4 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn4)), ((locals.var_t4_dn5 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn5)), ((locals.var_t4_dn6 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn6)), ((locals.var_t4_dn7 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn7)), ((locals.var_t4_dn8 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn8)), ((locals.var_t4_dn9 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn9)), ((locals.var_t4_dn10 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn10)), ((locals.var_t4_dn11 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn11)), ((locals.var_t4_dn14 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn14)),)
    } else {
        (locals.var_qovdext, locals.var_qovdext_dn0, locals.var_qovdext_dn2, locals.var_qovdext_dn4, locals.var_qovdext_dn5, locals.var_qovdext_dn6, locals.var_qovdext_dn7, locals.var_qovdext_dn8, locals.var_qovdext_dn9, locals.var_qovdext_dn10, locals.var_qovdext_dn11, locals.var_qovdext_dn14,)
    }
};
        locals.var_qovdext = assign90490_e139245;
        locals.var_qovdext_dn0 = assign90490_e139245_d_n0;
        locals.var_qovdext_dn2 = assign90490_e139245_d_n2;
        locals.var_qovdext_dn4 = assign90490_e139245_d_n4;
        locals.var_qovdext_dn5 = assign90490_e139245_d_n5;
        locals.var_qovdext_dn6 = assign90490_e139245_d_n6;
        locals.var_qovdext_dn7 = assign90490_e139245_d_n7;
        locals.var_qovdext_dn8 = assign90490_e139245_d_n8;
        locals.var_qovdext_dn9 = assign90490_e139245_d_n9;
        locals.var_qovdext_dn10 = assign90490_e139245_d_n10;
        locals.var_qovdext_dn11 = assign90490_e139245_d_n11;
        locals.var_qovdext_dn14 = assign90490_e139245_d_n14;
        locals.var_qovdext_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_349(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign90500_e139260, assign90500_e139260_d_n0, assign90500_e139260_d_n2, assign90500_e139260_d_n4, assign90500_e139260_d_n5, assign90500_e139260_d_n6, assign90500_e139260_d_n7, assign90500_e139260_d_n8, assign90500_e139260_d_n9, assign90500_e139260_d_n10, assign90500_e139260_d_n11, assign90500_e139260_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard2109 != 0.0) && (!(((locals.var_guard2106 != 0.0) || (locals.var_guard2107 != 0.0)) || (locals.var_guard2108 != 0.0))))) {
        let assign90500_e139258: f64 = (locals.var_t4 * locals.var_qbuld);
        (assign90500_e139258, ((locals.var_t4_dn0 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn0)), ((locals.var_t4_dn2 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn2)), ((locals.var_t4_dn4 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn4)), ((locals.var_t4_dn5 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn5)), ((locals.var_t4_dn6 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn6)), ((locals.var_t4_dn7 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn7)), ((locals.var_t4_dn8 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn8)), ((locals.var_t4_dn9 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn9)), ((locals.var_t4_dn10 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn10)), ((locals.var_t4_dn11 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn11)), ((locals.var_t4_dn14 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn14)),)
    } else {
        (locals.var_qbdldext, locals.var_qbdldext_dn0, locals.var_qbdldext_dn2, locals.var_qbdldext_dn4, locals.var_qbdldext_dn5, locals.var_qbdldext_dn6, locals.var_qbdldext_dn7, locals.var_qbdldext_dn8, locals.var_qbdldext_dn9, locals.var_qbdldext_dn10, locals.var_qbdldext_dn11, locals.var_qbdldext_dn14,)
    }
};
        locals.var_qbdldext = assign90500_e139260;
        locals.var_qbdldext_dn0 = assign90500_e139260_d_n0;
        locals.var_qbdldext_dn2 = assign90500_e139260_d_n2;
        locals.var_qbdldext_dn4 = assign90500_e139260_d_n4;
        locals.var_qbdldext_dn5 = assign90500_e139260_d_n5;
        locals.var_qbdldext_dn6 = assign90500_e139260_d_n6;
        locals.var_qbdldext_dn7 = assign90500_e139260_d_n7;
        locals.var_qbdldext_dn8 = assign90500_e139260_d_n8;
        locals.var_qbdldext_dn9 = assign90500_e139260_d_n9;
        locals.var_qbdldext_dn10 = assign90500_e139260_d_n10;
        locals.var_qbdldext_dn11 = assign90500_e139260_d_n11;
        locals.var_qbdldext_dn14 = assign90500_e139260_d_n14;
        locals.var_qbdldext_rv = 0.0;

        let assign90510_e139263: f64 = if p.p430 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2113 = assign90510_e139263;
        locals.var_guard2113_rv = 0.0;

        let (assign90520_e139267,) = {
    if (locals.var_guard2113 != 0.0) {
        (1.0,)
    } else {
        (locals.var_flg_never_reach_vfbover,)
    }
};
        locals.var_flg_never_reach_vfbover = assign90520_e139267;
        locals.var_flg_never_reach_vfbover_rv = 0.0;

        let assign90530_e139278: f64 = if (((p.p35 == 1.0) && (p.p63 > 0.0)) && (locals.var_uc_nover > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2114 = assign90530_e139278;
        locals.var_guard2114_rv = 0.0;

        let (assign90540_e139286, assign90540_e139286_d_n2, assign90540_e139286_d_n7, assign90540_e139286_d_n8, assign90540_e139286_d_n9,) = {
    if ((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) {
        let assign90540_e139284: f64 = (locals.var_vgsi - locals.var_vbsi);
        (assign90540_e139284, 0.0, locals.var_vgsi_dn7, (locals.var_vgsi_dn8 - locals.var_vbsi_dn8), (-locals.var_vbsi_dn9),)
    } else {
        (locals.var_vgbgmt, locals.var_vgbgmt_dn2, locals.var_vgbgmt_dn7, locals.var_vgbgmt_dn8, locals.var_vgbgmt_dn9,)
    }
};
        locals.var_vgbgmt = assign90540_e139286;
        locals.var_vgbgmt_dn2 = assign90540_e139286_d_n2;
        locals.var_vgbgmt_dn7 = assign90540_e139286_d_n7;
        locals.var_vgbgmt_dn8 = assign90540_e139286_d_n8;
        locals.var_vgbgmt_dn9 = assign90540_e139286_d_n9;
        locals.var_vgbgmt_rv = 0.0;

        let (assign90550_e139294, assign90550_e139294_d_n0, assign90550_e139294_d_n2, assign90550_e139294_d_n4, assign90550_e139294_d_n5, assign90550_e139294_d_n6, assign90550_e139294_d_n7, assign90550_e139294_d_n8, assign90550_e139294_d_n9, assign90550_e139294_d_n10, assign90550_e139294_d_n11, assign90550_e139294_d_n14,) = {
    if ((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) {
        let assign90550_e139292: f64 = (locals.var_vdsi - locals.var_vbsi);
        (assign90550_e139292, 0.0, 0.0, 0.0, 0.0, locals.var_vdsi_dn6, 0.0, (locals.var_vdsi_dn8 - locals.var_vbsi_dn8), (-locals.var_vbsi_dn9), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn11, locals.var_vxbgmt_dn14,)
    }
};
        locals.var_vxbgmt = assign90550_e139294;
        locals.var_vxbgmt_dn0 = assign90550_e139294_d_n0;
        locals.var_vxbgmt_dn2 = assign90550_e139294_d_n2;
        locals.var_vxbgmt_dn4 = assign90550_e139294_d_n4;
        locals.var_vxbgmt_dn5 = assign90550_e139294_d_n5;
        locals.var_vxbgmt_dn6 = assign90550_e139294_d_n6;
        locals.var_vxbgmt_dn7 = assign90550_e139294_d_n7;
        locals.var_vxbgmt_dn8 = assign90550_e139294_d_n8;
        locals.var_vxbgmt_dn9 = assign90550_e139294_d_n9;
        locals.var_vxbgmt_dn10 = assign90550_e139294_d_n10;
        locals.var_vxbgmt_dn11 = assign90550_e139294_d_n11;
        locals.var_vxbgmt_dn14 = assign90550_e139294_d_n14;
        locals.var_vxbgmt_rv = 0.0;

        let (assign90560_e139300,) = {
    if ((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) {
        (locals.var_uc_nover,)
    } else {
        (locals.var_nover_func,)
    }
};
        locals.var_nover_func = assign90560_e139300;
        locals.var_nover_func_rv = 0.0;

        let (assign90570_e139306, assign90570_e139306_d_n0, assign90570_e139306_d_n2, assign90570_e139306_d_n4, assign90570_e139306_d_n5, assign90570_e139306_d_n6, assign90570_e139306_d_n7, assign90570_e139306_d_n8, assign90570_e139306_d_n9, assign90570_e139306_d_n10, assign90570_e139306_d_n11, assign90570_e139306_d_n14,) = {
    if ((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_lover_func, locals.var_lover_func_dn0, locals.var_lover_func_dn2, locals.var_lover_func_dn4, locals.var_lover_func_dn5, locals.var_lover_func_dn6, locals.var_lover_func_dn7, locals.var_lover_func_dn8, locals.var_lover_func_dn9, locals.var_lover_func_dn10, locals.var_lover_func_dn11, locals.var_lover_func_dn14,)
    }
};
        locals.var_lover_func = assign90570_e139306;
        locals.var_lover_func_dn0 = assign90570_e139306_d_n0;
        locals.var_lover_func_dn2 = assign90570_e139306_d_n2;
        locals.var_lover_func_dn4 = assign90570_e139306_d_n4;
        locals.var_lover_func_dn5 = assign90570_e139306_d_n5;
        locals.var_lover_func_dn6 = assign90570_e139306_d_n6;
        locals.var_lover_func_dn7 = assign90570_e139306_d_n7;
        locals.var_lover_func_dn8 = assign90570_e139306_d_n8;
        locals.var_lover_func_dn9 = assign90570_e139306_d_n9;
        locals.var_lover_func_dn10 = assign90570_e139306_d_n10;
        locals.var_lover_func_dn11 = assign90570_e139306_d_n11;
        locals.var_lover_func_dn14 = assign90570_e139306_d_n14;
        locals.var_lover_func_rv = 0.0;

        let (assign90580_e139312, assign90580_e139312_d_n0, assign90580_e139312_d_n2, assign90580_e139312_d_n4, assign90580_e139312_d_n5, assign90580_e139312_d_n6, assign90580_e139312_d_n7, assign90580_e139312_d_n8, assign90580_e139312_d_n9, assign90580_e139312_d_n10, assign90580_e139312_d_n11, assign90580_e139312_d_n14,) = {
    if ((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) {
        (locals.var_wdep, locals.var_wdep_dn0, locals.var_wdep_dn2, locals.var_wdep_dn4, locals.var_wdep_dn5, locals.var_wdep_dn6, locals.var_wdep_dn7, locals.var_wdep_dn8, locals.var_wdep_dn9, locals.var_wdep_dn10, locals.var_wdep_dn11, locals.var_wdep_dn14,)
    } else {
        (locals.var_wdep_func, locals.var_wdep_func_dn0, locals.var_wdep_func_dn2, locals.var_wdep_func_dn4, locals.var_wdep_func_dn5, locals.var_wdep_func_dn6, locals.var_wdep_func_dn7, locals.var_wdep_func_dn8, locals.var_wdep_func_dn9, locals.var_wdep_func_dn10, locals.var_wdep_func_dn11, locals.var_wdep_func_dn14,)
    }
};
        locals.var_wdep_func = assign90580_e139312;
        locals.var_wdep_func_dn0 = assign90580_e139312_d_n0;
        locals.var_wdep_func_dn2 = assign90580_e139312_d_n2;
        locals.var_wdep_func_dn4 = assign90580_e139312_d_n4;
        locals.var_wdep_func_dn5 = assign90580_e139312_d_n5;
        locals.var_wdep_func_dn6 = assign90580_e139312_d_n6;
        locals.var_wdep_func_dn7 = assign90580_e139312_d_n7;
        locals.var_wdep_func_dn8 = assign90580_e139312_d_n8;
        locals.var_wdep_func_dn9 = assign90580_e139312_d_n9;
        locals.var_wdep_func_dn10 = assign90580_e139312_d_n10;
        locals.var_wdep_func_dn11 = assign90580_e139312_d_n11;
        locals.var_wdep_func_dn14 = assign90580_e139312_d_n14;
        locals.var_wdep_func_rv = 0.0;

        let (assign90590_e139318, assign90590_e139318_d_n0, assign90590_e139318_d_n2, assign90590_e139318_d_n4, assign90590_e139318_d_n5, assign90590_e139318_d_n6, assign90590_e139318_d_n7, assign90590_e139318_d_n8, assign90590_e139318_d_n9, assign90590_e139318_d_n10, assign90590_e139318_d_n11, assign90590_e139318_d_n14,) = {
    if ((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) {
        (locals.var_cnst0over, locals.var_cnst0over_dn0, locals.var_cnst0over_dn2, locals.var_cnst0over_dn4, locals.var_cnst0over_dn5, locals.var_cnst0over_dn6, locals.var_cnst0over_dn7, locals.var_cnst0over_dn8, locals.var_cnst0over_dn9, locals.var_cnst0over_dn10, locals.var_cnst0over_dn11, locals.var_cnst0over_dn14,)
    } else {
        (locals.var_cnst0over_func, locals.var_cnst0over_func_dn0, locals.var_cnst0over_func_dn2, locals.var_cnst0over_func_dn4, locals.var_cnst0over_func_dn5, locals.var_cnst0over_func_dn6, locals.var_cnst0over_func_dn7, locals.var_cnst0over_func_dn8, locals.var_cnst0over_func_dn9, locals.var_cnst0over_func_dn10, locals.var_cnst0over_func_dn11, locals.var_cnst0over_func_dn14,)
    }
};
        locals.var_cnst0over_func = assign90590_e139318;
        locals.var_cnst0over_func_dn0 = assign90590_e139318_d_n0;
        locals.var_cnst0over_func_dn2 = assign90590_e139318_d_n2;
        locals.var_cnst0over_func_dn4 = assign90590_e139318_d_n4;
        locals.var_cnst0over_func_dn5 = assign90590_e139318_d_n5;
        locals.var_cnst0over_func_dn6 = assign90590_e139318_d_n6;
        locals.var_cnst0over_func_dn7 = assign90590_e139318_d_n7;
        locals.var_cnst0over_func_dn8 = assign90590_e139318_d_n8;
        locals.var_cnst0over_func_dn9 = assign90590_e139318_d_n9;
        locals.var_cnst0over_func_dn10 = assign90590_e139318_d_n10;
        locals.var_cnst0over_func_dn11 = assign90590_e139318_d_n11;
        locals.var_cnst0over_func_dn14 = assign90590_e139318_d_n14;
        locals.var_cnst0over_func_rv = 0.0;

        let (assign90600_e139324,) = {
    if ((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) {
        (locals.var_coxb0,)
    } else {
        (locals.var_cox0_func,)
    }
};
        locals.var_cox0_func = assign90600_e139324;
        locals.var_cox0_func_rv = 0.0;

        let (assign90610_e139330, assign90610_e139330_d_n0, assign90610_e139330_d_n2, assign90610_e139330_d_n4, assign90610_e139330_d_n5, assign90610_e139330_d_n6, assign90610_e139330_d_n7, assign90610_e139330_d_n8, assign90610_e139330_d_n9, assign90610_e139330_d_n10, assign90610_e139330_d_n11, assign90610_e139330_d_n14,) = {
    if ((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) {
        (0.4, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbs_bnd_over__blk2122, locals.var_vbs_bnd_over__blk2122_dn0, locals.var_vbs_bnd_over__blk2122_dn2, locals.var_vbs_bnd_over__blk2122_dn4, locals.var_vbs_bnd_over__blk2122_dn5, locals.var_vbs_bnd_over__blk2122_dn6, locals.var_vbs_bnd_over__blk2122_dn7, locals.var_vbs_bnd_over__blk2122_dn8, locals.var_vbs_bnd_over__blk2122_dn9, locals.var_vbs_bnd_over__blk2122_dn10, locals.var_vbs_bnd_over__blk2122_dn11, locals.var_vbs_bnd_over__blk2122_dn14,)
    }
};
        locals.var_vbs_bnd_over__blk2122 = assign90610_e139330;
        locals.var_vbs_bnd_over__blk2122_dn0 = assign90610_e139330_d_n0;
        locals.var_vbs_bnd_over__blk2122_dn2 = assign90610_e139330_d_n2;
        locals.var_vbs_bnd_over__blk2122_dn4 = assign90610_e139330_d_n4;
        locals.var_vbs_bnd_over__blk2122_dn5 = assign90610_e139330_d_n5;
        locals.var_vbs_bnd_over__blk2122_dn6 = assign90610_e139330_d_n6;
        locals.var_vbs_bnd_over__blk2122_dn7 = assign90610_e139330_d_n7;
        locals.var_vbs_bnd_over__blk2122_dn8 = assign90610_e139330_d_n8;
        locals.var_vbs_bnd_over__blk2122_dn9 = assign90610_e139330_d_n9;
        locals.var_vbs_bnd_over__blk2122_dn10 = assign90610_e139330_d_n10;
        locals.var_vbs_bnd_over__blk2122_dn11 = assign90610_e139330_d_n11;
        locals.var_vbs_bnd_over__blk2122_dn14 = assign90610_e139330_d_n14;
        locals.var_vbs_bnd_over__blk2122_rv = 0.0;

        let (assign90630_e139342,) = {
    if ((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_fd_mode__blk2123,)
    }
};
        locals.var_flg_fd_mode__blk2123 = assign90630_e139342;
        locals.var_flg_fd_mode__blk2123_rv = 0.0;

        let (assign90640_e139348, assign90640_e139348_d_n0, assign90640_e139348_d_n2, assign90640_e139348_d_n4, assign90640_e139348_d_n5, assign90640_e139348_d_n6, assign90640_e139348_d_n7, assign90640_e139348_d_n8, assign90640_e139348_d_n9, assign90640_e139348_d_n10, assign90640_e139348_d_n11, assign90640_e139348_d_n14,) = {
    if ((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn14,)
    }
};
        locals.var_fb = assign90640_e139348;
        locals.var_fb_dn0 = assign90640_e139348_d_n0;
        locals.var_fb_dn2 = assign90640_e139348_d_n2;
        locals.var_fb_dn4 = assign90640_e139348_d_n4;
        locals.var_fb_dn5 = assign90640_e139348_d_n5;
        locals.var_fb_dn6 = assign90640_e139348_d_n6;
        locals.var_fb_dn7 = assign90640_e139348_d_n7;
        locals.var_fb_dn8 = assign90640_e139348_d_n8;
        locals.var_fb_dn9 = assign90640_e139348_d_n9;
        locals.var_fb_dn10 = assign90640_e139348_d_n10;
        locals.var_fb_dn11 = assign90640_e139348_d_n11;
        locals.var_fb_dn14 = assign90640_e139348_d_n14;
        locals.var_fb_rv = 0.0;

        let (assign90650_e139354, assign90650_e139354_d_n0, assign90650_e139354_d_n2, assign90650_e139354_d_n4, assign90650_e139354_d_n5, assign90650_e139354_d_n6, assign90650_e139354_d_n7, assign90650_e139354_d_n8, assign90650_e139354_d_n9, assign90650_e139354_d_n10, assign90650_e139354_d_n11, assign90650_e139354_d_n14,) = {
    if ((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn11, locals.var_fs01_dn14,)
    }
};
        locals.var_fs01 = assign90650_e139354;
        locals.var_fs01_dn0 = assign90650_e139354_d_n0;
        locals.var_fs01_dn2 = assign90650_e139354_d_n2;
        locals.var_fs01_dn4 = assign90650_e139354_d_n4;
        locals.var_fs01_dn5 = assign90650_e139354_d_n5;
        locals.var_fs01_dn6 = assign90650_e139354_d_n6;
        locals.var_fs01_dn7 = assign90650_e139354_d_n7;
        locals.var_fs01_dn8 = assign90650_e139354_d_n8;
        locals.var_fs01_dn9 = assign90650_e139354_d_n9;
        locals.var_fs01_dn10 = assign90650_e139354_d_n10;
        locals.var_fs01_dn11 = assign90650_e139354_d_n11;
        locals.var_fs01_dn14 = assign90650_e139354_d_n14;
        locals.var_fs01_rv = 0.0;

        let (assign90660_e139360, assign90660_e139360_d_n0, assign90660_e139360_d_n2, assign90660_e139360_d_n4, assign90660_e139360_d_n5, assign90660_e139360_d_n6, assign90660_e139360_d_n7, assign90660_e139360_d_n8, assign90660_e139360_d_n9, assign90660_e139360_d_n10, assign90660_e139360_d_n11, assign90660_e139360_d_n14,) = {
    if ((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn11, locals.var_fs02_dn14,)
    }
};
        locals.var_fs02 = assign90660_e139360;
        locals.var_fs02_dn0 = assign90660_e139360_d_n0;
        locals.var_fs02_dn2 = assign90660_e139360_d_n2;
        locals.var_fs02_dn4 = assign90660_e139360_d_n4;
        locals.var_fs02_dn5 = assign90660_e139360_d_n5;
        locals.var_fs02_dn6 = assign90660_e139360_d_n6;
        locals.var_fs02_dn7 = assign90660_e139360_d_n7;
        locals.var_fs02_dn8 = assign90660_e139360_d_n8;
        locals.var_fs02_dn9 = assign90660_e139360_d_n9;
        locals.var_fs02_dn10 = assign90660_e139360_d_n10;
        locals.var_fs02_dn11 = assign90660_e139360_d_n11;
        locals.var_fs02_dn14 = assign90660_e139360_d_n14;
        locals.var_fs02_rv = 0.0;

        let (assign90670_e139366, assign90670_e139366_d_n0, assign90670_e139366_d_n2, assign90670_e139366_d_n4, assign90670_e139366_d_n5, assign90670_e139366_d_n6, assign90670_e139366_d_n7, assign90670_e139366_d_n8, assign90670_e139366_d_n9, assign90670_e139366_d_n10, assign90670_e139366_d_n11, assign90670_e139366_d_n14,) = {
    if ((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs0, locals.var_fs0_dn0, locals.var_fs0_dn2, locals.var_fs0_dn4, locals.var_fs0_dn5, locals.var_fs0_dn6, locals.var_fs0_dn7, locals.var_fs0_dn8, locals.var_fs0_dn9, locals.var_fs0_dn10, locals.var_fs0_dn11, locals.var_fs0_dn14,)
    }
};
        locals.var_fs0 = assign90670_e139366;
        locals.var_fs0_dn0 = assign90670_e139366_d_n0;
        locals.var_fs0_dn2 = assign90670_e139366_d_n2;
        locals.var_fs0_dn4 = assign90670_e139366_d_n4;
        locals.var_fs0_dn5 = assign90670_e139366_d_n5;
        locals.var_fs0_dn6 = assign90670_e139366_d_n6;
        locals.var_fs0_dn7 = assign90670_e139366_d_n7;
        locals.var_fs0_dn8 = assign90670_e139366_d_n8;
        locals.var_fs0_dn9 = assign90670_e139366_d_n9;
        locals.var_fs0_dn10 = assign90670_e139366_d_n10;
        locals.var_fs0_dn11 = assign90670_e139366_d_n11;
        locals.var_fs0_dn14 = assign90670_e139366_d_n14;
        locals.var_fs0_rv = 0.0;

        let (assign90680_e139372, assign90680_e139372_d_n0, assign90680_e139372_d_n2, assign90680_e139372_d_n4, assign90680_e139372_d_n5, assign90680_e139372_d_n6, assign90680_e139372_d_n7, assign90680_e139372_d_n8, assign90680_e139372_d_n9, assign90680_e139372_d_n10, assign90680_e139372_d_n11, assign90680_e139372_d_n14,) = {
    if ((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn4, locals.var_dps0_dn5, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn8, locals.var_dps0_dn9, locals.var_dps0_dn10, locals.var_dps0_dn11, locals.var_dps0_dn14,)
    }
};
        locals.var_dps0 = assign90680_e139372;
        locals.var_dps0_dn0 = assign90680_e139372_d_n0;
        locals.var_dps0_dn2 = assign90680_e139372_d_n2;
        locals.var_dps0_dn4 = assign90680_e139372_d_n4;
        locals.var_dps0_dn5 = assign90680_e139372_d_n5;
        locals.var_dps0_dn6 = assign90680_e139372_d_n6;
        locals.var_dps0_dn7 = assign90680_e139372_d_n7;
        locals.var_dps0_dn8 = assign90680_e139372_d_n8;
        locals.var_dps0_dn9 = assign90680_e139372_d_n9;
        locals.var_dps0_dn10 = assign90680_e139372_d_n10;
        locals.var_dps0_dn11 = assign90680_e139372_d_n11;
        locals.var_dps0_dn14 = assign90680_e139372_d_n14;
        locals.var_dps0_rv = 0.0;

        let (assign90690_e139378, assign90690_e139378_d_n0, assign90690_e139378_d_n2, assign90690_e139378_d_n4, assign90690_e139378_d_n5, assign90690_e139378_d_n6, assign90690_e139378_d_n7, assign90690_e139378_d_n8, assign90690_e139378_d_n9, assign90690_e139378_d_n10, assign90690_e139378_d_n11, assign90690_e139378_d_n14,) = {
    if ((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs0_dps0, locals.var_fs0_dps0_dn0, locals.var_fs0_dps0_dn2, locals.var_fs0_dps0_dn4, locals.var_fs0_dps0_dn5, locals.var_fs0_dps0_dn6, locals.var_fs0_dps0_dn7, locals.var_fs0_dps0_dn8, locals.var_fs0_dps0_dn9, locals.var_fs0_dps0_dn10, locals.var_fs0_dps0_dn11, locals.var_fs0_dps0_dn14,)
    }
};
        locals.var_fs0_dps0 = assign90690_e139378;
        locals.var_fs0_dps0_dn0 = assign90690_e139378_d_n0;
        locals.var_fs0_dps0_dn2 = assign90690_e139378_d_n2;
        locals.var_fs0_dps0_dn4 = assign90690_e139378_d_n4;
        locals.var_fs0_dps0_dn5 = assign90690_e139378_d_n5;
        locals.var_fs0_dps0_dn6 = assign90690_e139378_d_n6;
        locals.var_fs0_dps0_dn7 = assign90690_e139378_d_n7;
        locals.var_fs0_dps0_dn8 = assign90690_e139378_d_n8;
        locals.var_fs0_dps0_dn9 = assign90690_e139378_d_n9;
        locals.var_fs0_dps0_dn10 = assign90690_e139378_d_n10;
        locals.var_fs0_dps0_dn11 = assign90690_e139378_d_n11;
        locals.var_fs0_dps0_dn14 = assign90690_e139378_d_n14;
        locals.var_fs0_dps0_rv = 0.0;

        let (assign90700_e139384, assign90700_e139384_d_n0, assign90700_e139384_d_n2, assign90700_e139384_d_n4, assign90700_e139384_d_n5, assign90700_e139384_d_n6, assign90700_e139384_d_n7, assign90700_e139384_d_n8, assign90700_e139384_d_n9, assign90700_e139384_d_n10, assign90700_e139384_d_n11, assign90700_e139384_d_n14,) = {
    if ((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn11, locals.var_fs02_dps0_dn14,)
    }
};
        locals.var_fs02_dps0 = assign90700_e139384;
        locals.var_fs02_dps0_dn0 = assign90700_e139384_d_n0;
        locals.var_fs02_dps0_dn2 = assign90700_e139384_d_n2;
        locals.var_fs02_dps0_dn4 = assign90700_e139384_d_n4;
        locals.var_fs02_dps0_dn5 = assign90700_e139384_d_n5;
        locals.var_fs02_dps0_dn6 = assign90700_e139384_d_n6;
        locals.var_fs02_dps0_dn7 = assign90700_e139384_d_n7;
        locals.var_fs02_dps0_dn8 = assign90700_e139384_d_n8;
        locals.var_fs02_dps0_dn9 = assign90700_e139384_d_n9;
        locals.var_fs02_dps0_dn10 = assign90700_e139384_d_n10;
        locals.var_fs02_dps0_dn11 = assign90700_e139384_d_n11;
        locals.var_fs02_dps0_dn14 = assign90700_e139384_d_n14;
        locals.var_fs02_dps0_rv = 0.0;

        let (assign90710_e139390, assign90710_e139390_d_n0, assign90710_e139390_d_n2, assign90710_e139390_d_n4, assign90710_e139390_d_n5, assign90710_e139390_d_n6, assign90710_e139390_d_n7, assign90710_e139390_d_n8, assign90710_e139390_d_n9, assign90710_e139390_d_n10, assign90710_e139390_d_n11, assign90710_e139390_d_n14,) = {
    if ((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn14,)
    }
};
        locals.var_fb_dpss = assign90710_e139390;
        locals.var_fb_dpss_dn0 = assign90710_e139390_d_n0;
        locals.var_fb_dpss_dn2 = assign90710_e139390_d_n2;
        locals.var_fb_dpss_dn4 = assign90710_e139390_d_n4;
        locals.var_fb_dpss_dn5 = assign90710_e139390_d_n5;
        locals.var_fb_dpss_dn6 = assign90710_e139390_d_n6;
        locals.var_fb_dpss_dn7 = assign90710_e139390_d_n7;
        locals.var_fb_dpss_dn8 = assign90710_e139390_d_n8;
        locals.var_fb_dpss_dn9 = assign90710_e139390_d_n9;
        locals.var_fb_dpss_dn10 = assign90710_e139390_d_n10;
        locals.var_fb_dpss_dn11 = assign90710_e139390_d_n11;
        locals.var_fb_dpss_dn14 = assign90710_e139390_d_n14;
        locals.var_fb_dpss_rv = 0.0;

        let (assign90720_e139396, assign90720_e139396_d_n0, assign90720_e139396_d_n2, assign90720_e139396_d_n4, assign90720_e139396_d_n5, assign90720_e139396_d_n6, assign90720_e139396_d_n7, assign90720_e139396_d_n8, assign90720_e139396_d_n9, assign90720_e139396_d_n10, assign90720_e139396_d_n11, assign90720_e139396_d_n14,) = {
    if ((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn11, locals.var_fs01_dps0_dn14,)
    }
};
        locals.var_fs01_dps0 = assign90720_e139396;
        locals.var_fs01_dps0_dn0 = assign90720_e139396_d_n0;
        locals.var_fs01_dps0_dn2 = assign90720_e139396_d_n2;
        locals.var_fs01_dps0_dn4 = assign90720_e139396_d_n4;
        locals.var_fs01_dps0_dn5 = assign90720_e139396_d_n5;
        locals.var_fs01_dps0_dn6 = assign90720_e139396_d_n6;
        locals.var_fs01_dps0_dn7 = assign90720_e139396_d_n7;
        locals.var_fs01_dps0_dn8 = assign90720_e139396_d_n8;
        locals.var_fs01_dps0_dn9 = assign90720_e139396_d_n9;
        locals.var_fs01_dps0_dn10 = assign90720_e139396_d_n10;
        locals.var_fs01_dps0_dn11 = assign90720_e139396_d_n11;
        locals.var_fs01_dps0_dn14 = assign90720_e139396_d_n14;
        locals.var_fs01_dps0_rv = 0.0;

        let (assign90730_e139402, assign90730_e139402_d_n0, assign90730_e139402_d_n2, assign90730_e139402_d_n4, assign90730_e139402_d_n5, assign90730_e139402_d_n6, assign90730_e139402_d_n7, assign90730_e139402_d_n8, assign90730_e139402_d_n9, assign90730_e139402_d_n10, assign90730_e139402_d_n11, assign90730_e139402_d_n14,) = {
    if ((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn11, locals.var_chi_1_dn14,)
    }
};
        locals.var_chi_1 = assign90730_e139402;
        locals.var_chi_1_dn0 = assign90730_e139402_d_n0;
        locals.var_chi_1_dn2 = assign90730_e139402_d_n2;
        locals.var_chi_1_dn4 = assign90730_e139402_d_n4;
        locals.var_chi_1_dn5 = assign90730_e139402_d_n5;
        locals.var_chi_1_dn6 = assign90730_e139402_d_n6;
        locals.var_chi_1_dn7 = assign90730_e139402_d_n7;
        locals.var_chi_1_dn8 = assign90730_e139402_d_n8;
        locals.var_chi_1_dn9 = assign90730_e139402_d_n9;
        locals.var_chi_1_dn10 = assign90730_e139402_d_n10;
        locals.var_chi_1_dn11 = assign90730_e139402_d_n11;
        locals.var_chi_1_dn14 = assign90730_e139402_d_n14;
        locals.var_chi_1_rv = 0.0;

        let (assign90740_e139408, assign90740_e139408_d_n0, assign90740_e139408_d_n2, assign90740_e139408_d_n4, assign90740_e139408_d_n5, assign90740_e139408_d_n6, assign90740_e139408_d_n7, assign90740_e139408_d_n8, assign90740_e139408_d_n9, assign90740_e139408_d_n10, assign90740_e139408_d_n11, assign90740_e139408_d_n14,) = {
    if ((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_chi_a, locals.var_chi_a_dn0, locals.var_chi_a_dn2, locals.var_chi_a_dn4, locals.var_chi_a_dn5, locals.var_chi_a_dn6, locals.var_chi_a_dn7, locals.var_chi_a_dn8, locals.var_chi_a_dn9, locals.var_chi_a_dn10, locals.var_chi_a_dn11, locals.var_chi_a_dn14,)
    }
};
        locals.var_chi_a = assign90740_e139408;
        locals.var_chi_a_dn0 = assign90740_e139408_d_n0;
        locals.var_chi_a_dn2 = assign90740_e139408_d_n2;
        locals.var_chi_a_dn4 = assign90740_e139408_d_n4;
        locals.var_chi_a_dn5 = assign90740_e139408_d_n5;
        locals.var_chi_a_dn6 = assign90740_e139408_d_n6;
        locals.var_chi_a_dn7 = assign90740_e139408_d_n7;
        locals.var_chi_a_dn8 = assign90740_e139408_d_n8;
        locals.var_chi_a_dn9 = assign90740_e139408_d_n9;
        locals.var_chi_a_dn10 = assign90740_e139408_d_n10;
        locals.var_chi_a_dn11 = assign90740_e139408_d_n11;
        locals.var_chi_a_dn14 = assign90740_e139408_d_n14;
        locals.var_chi_a_rv = 0.0;

        let (assign90750_e139414, assign90750_e139414_d_n0, assign90750_e139414_d_n2, assign90750_e139414_d_n4, assign90750_e139414_d_n5, assign90750_e139414_d_n6, assign90750_e139414_d_n7, assign90750_e139414_d_n8, assign90750_e139414_d_n9, assign90750_e139414_d_n10, assign90750_e139414_d_n11, assign90750_e139414_d_n14,) = {
    if ((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_chi_b, locals.var_chi_b_dn0, locals.var_chi_b_dn2, locals.var_chi_b_dn4, locals.var_chi_b_dn5, locals.var_chi_b_dn6, locals.var_chi_b_dn7, locals.var_chi_b_dn8, locals.var_chi_b_dn9, locals.var_chi_b_dn10, locals.var_chi_b_dn11, locals.var_chi_b_dn14,)
    }
};
        locals.var_chi_b = assign90750_e139414;
        locals.var_chi_b_dn0 = assign90750_e139414_d_n0;
        locals.var_chi_b_dn2 = assign90750_e139414_d_n2;
        locals.var_chi_b_dn4 = assign90750_e139414_d_n4;
        locals.var_chi_b_dn5 = assign90750_e139414_d_n5;
        locals.var_chi_b_dn6 = assign90750_e139414_d_n6;
        locals.var_chi_b_dn7 = assign90750_e139414_d_n7;
        locals.var_chi_b_dn8 = assign90750_e139414_d_n8;
        locals.var_chi_b_dn9 = assign90750_e139414_d_n9;
        locals.var_chi_b_dn10 = assign90750_e139414_d_n10;
        locals.var_chi_b_dn11 = assign90750_e139414_d_n11;
        locals.var_chi_b_dn14 = assign90750_e139414_d_n14;
        locals.var_chi_b_rv = 0.0;

        let (assign90760_e139421,) = {
    if ((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) {
        let assign90760_e139419: f64 = (-1.0);
        (assign90760_e139419,)
    } else {
        (locals.var_flg_conv,)
    }
};
        locals.var_flg_conv = assign90760_e139421;
        locals.var_flg_conv_rv = 0.0;

        let (assign90770_e139427, assign90770_e139427_d_n0, assign90770_e139427_d_n2, assign90770_e139427_d_n4, assign90770_e139427_d_n5, assign90770_e139427_d_n6, assign90770_e139427_d_n7, assign90770_e139427_d_n8, assign90770_e139427_d_n9, assign90770_e139427_d_n10, assign90770_e139427_d_n11, assign90770_e139427_d_n14,) = {
    if ((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ps0ld_ini__blk2124, locals.var_ps0ld_ini__blk2124_dn0, locals.var_ps0ld_ini__blk2124_dn2, locals.var_ps0ld_ini__blk2124_dn4, locals.var_ps0ld_ini__blk2124_dn5, locals.var_ps0ld_ini__blk2124_dn6, locals.var_ps0ld_ini__blk2124_dn7, locals.var_ps0ld_ini__blk2124_dn8, locals.var_ps0ld_ini__blk2124_dn9, locals.var_ps0ld_ini__blk2124_dn10, locals.var_ps0ld_ini__blk2124_dn11, locals.var_ps0ld_ini__blk2124_dn14,)
    }
};
        locals.var_ps0ld_ini__blk2124 = assign90770_e139427;
        locals.var_ps0ld_ini__blk2124_dn0 = assign90770_e139427_d_n0;
        locals.var_ps0ld_ini__blk2124_dn2 = assign90770_e139427_d_n2;
        locals.var_ps0ld_ini__blk2124_dn4 = assign90770_e139427_d_n4;
        locals.var_ps0ld_ini__blk2124_dn5 = assign90770_e139427_d_n5;
        locals.var_ps0ld_ini__blk2124_dn6 = assign90770_e139427_d_n6;
        locals.var_ps0ld_ini__blk2124_dn7 = assign90770_e139427_d_n7;
        locals.var_ps0ld_ini__blk2124_dn8 = assign90770_e139427_d_n8;
        locals.var_ps0ld_ini__blk2124_dn9 = assign90770_e139427_d_n9;
        locals.var_ps0ld_ini__blk2124_dn10 = assign90770_e139427_d_n10;
        locals.var_ps0ld_ini__blk2124_dn11 = assign90770_e139427_d_n11;
        locals.var_ps0ld_ini__blk2124_dn14 = assign90770_e139427_d_n14;
        locals.var_ps0ld_ini__blk2124_rv = 0.0;

        let (assign90780_e139433, assign90780_e139433_d_n0, assign90780_e139433_d_n2, assign90780_e139433_d_n4, assign90780_e139433_d_n5, assign90780_e139433_d_n6, assign90780_e139433_d_n7, assign90780_e139433_d_n8, assign90780_e139433_d_n9, assign90780_e139433_d_n10, assign90780_e139433_d_n11, assign90780_e139433_d_n14,) = {
    if ((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fbsq__blk2125, locals.var_fbsq__blk2125_dn0, locals.var_fbsq__blk2125_dn2, locals.var_fbsq__blk2125_dn4, locals.var_fbsq__blk2125_dn5, locals.var_fbsq__blk2125_dn6, locals.var_fbsq__blk2125_dn7, locals.var_fbsq__blk2125_dn8, locals.var_fbsq__blk2125_dn9, locals.var_fbsq__blk2125_dn10, locals.var_fbsq__blk2125_dn11, locals.var_fbsq__blk2125_dn14,)
    }
};
        locals.var_fbsq__blk2125 = assign90780_e139433;
        locals.var_fbsq__blk2125_dn0 = assign90780_e139433_d_n0;
        locals.var_fbsq__blk2125_dn2 = assign90780_e139433_d_n2;
        locals.var_fbsq__blk2125_dn4 = assign90780_e139433_d_n4;
        locals.var_fbsq__blk2125_dn5 = assign90780_e139433_d_n5;
        locals.var_fbsq__blk2125_dn6 = assign90780_e139433_d_n6;
        locals.var_fbsq__blk2125_dn7 = assign90780_e139433_d_n7;
        locals.var_fbsq__blk2125_dn8 = assign90780_e139433_d_n8;
        locals.var_fbsq__blk2125_dn9 = assign90780_e139433_d_n9;
        locals.var_fbsq__blk2125_dn10 = assign90780_e139433_d_n10;
        locals.var_fbsq__blk2125_dn11 = assign90780_e139433_d_n11;
        locals.var_fbsq__blk2125_dn14 = assign90780_e139433_d_n14;
        locals.var_fbsq__blk2125_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_350(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign90790_e139446, assign90790_e139446_d_n0, assign90790_e139446_d_n2, assign90790_e139446_d_n4, assign90790_e139446_d_n5, assign90790_e139446_d_n6, assign90790_e139446_d_n7, assign90790_e139446_d_n8, assign90790_e139446_d_n9, assign90790_e139446_d_n10, assign90790_e139446_d_n11, assign90790_e139446_d_n14,) = {
    if ((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) {
        let assign90790_e139439: f64 = (2.0 * locals.var_beta_inv);
        let assign90790_e139442: f64 = (locals.var_nover_func / locals.var_nin);
        let assign90790_e139443: f64 = (assign90790_e139442).ln();
        let assign90790_e139444: f64 = (assign90790_e139439 * assign90790_e139443);
        (assign90790_e139444, (((2.0 * locals.var_beta_inv_dn0) * assign90790_e139443) + (assign90790_e139439 * ((-((locals.var_nover_func * locals.var_nin_dn0) / (locals.var_nin * locals.var_nin))) / assign90790_e139442))), (((2.0 * locals.var_beta_inv_dn2) * assign90790_e139443) + (assign90790_e139439 * ((-((locals.var_nover_func * locals.var_nin_dn2) / (locals.var_nin * locals.var_nin))) / assign90790_e139442))), (((2.0 * locals.var_beta_inv_dn4) * assign90790_e139443) + (assign90790_e139439 * ((-((locals.var_nover_func * locals.var_nin_dn4) / (locals.var_nin * locals.var_nin))) / assign90790_e139442))), (((2.0 * locals.var_beta_inv_dn5) * assign90790_e139443) + (assign90790_e139439 * ((-((locals.var_nover_func * locals.var_nin_dn5) / (locals.var_nin * locals.var_nin))) / assign90790_e139442))), (((2.0 * locals.var_beta_inv_dn6) * assign90790_e139443) + (assign90790_e139439 * ((-((locals.var_nover_func * locals.var_nin_dn6) / (locals.var_nin * locals.var_nin))) / assign90790_e139442))), (((2.0 * locals.var_beta_inv_dn7) * assign90790_e139443) + (assign90790_e139439 * ((-((locals.var_nover_func * locals.var_nin_dn7) / (locals.var_nin * locals.var_nin))) / assign90790_e139442))), (((2.0 * locals.var_beta_inv_dn8) * assign90790_e139443) + (assign90790_e139439 * ((-((locals.var_nover_func * locals.var_nin_dn8) / (locals.var_nin * locals.var_nin))) / assign90790_e139442))), (((2.0 * locals.var_beta_inv_dn9) * assign90790_e139443) + (assign90790_e139439 * ((-((locals.var_nover_func * locals.var_nin_dn9) / (locals.var_nin * locals.var_nin))) / assign90790_e139442))), (((2.0 * locals.var_beta_inv_dn10) * assign90790_e139443) + (assign90790_e139439 * ((-((locals.var_nover_func * locals.var_nin_dn10) / (locals.var_nin * locals.var_nin))) / assign90790_e139442))), (((2.0 * locals.var_beta_inv_dn11) * assign90790_e139443) + (assign90790_e139439 * ((-((locals.var_nover_func * locals.var_nin_dn11) / (locals.var_nin * locals.var_nin))) / assign90790_e139442))), (((2.0 * locals.var_beta_inv_dn14) * assign90790_e139443) + (assign90790_e139439 * ((-((locals.var_nover_func * locals.var_nin_dn14) / (locals.var_nin * locals.var_nin))) / assign90790_e139442))),)
    } else {
        (locals.var_pb2over__blk2120, locals.var_pb2over__blk2120_dn0, locals.var_pb2over__blk2120_dn2, locals.var_pb2over__blk2120_dn4, locals.var_pb2over__blk2120_dn5, locals.var_pb2over__blk2120_dn6, locals.var_pb2over__blk2120_dn7, locals.var_pb2over__blk2120_dn8, locals.var_pb2over__blk2120_dn9, locals.var_pb2over__blk2120_dn10, locals.var_pb2over__blk2120_dn11, locals.var_pb2over__blk2120_dn14,)
    }
};
        locals.var_pb2over__blk2120 = assign90790_e139446;
        locals.var_pb2over__blk2120_dn0 = assign90790_e139446_d_n0;
        locals.var_pb2over__blk2120_dn2 = assign90790_e139446_d_n2;
        locals.var_pb2over__blk2120_dn4 = assign90790_e139446_d_n4;
        locals.var_pb2over__blk2120_dn5 = assign90790_e139446_d_n5;
        locals.var_pb2over__blk2120_dn6 = assign90790_e139446_d_n6;
        locals.var_pb2over__blk2120_dn7 = assign90790_e139446_d_n7;
        locals.var_pb2over__blk2120_dn8 = assign90790_e139446_d_n8;
        locals.var_pb2over__blk2120_dn9 = assign90790_e139446_d_n9;
        locals.var_pb2over__blk2120_dn10 = assign90790_e139446_d_n10;
        locals.var_pb2over__blk2120_dn11 = assign90790_e139446_d_n11;
        locals.var_pb2over__blk2120_dn14 = assign90790_e139446_d_n14;
        locals.var_pb2over__blk2120_rv = 0.0;

        let (assign90800_e139456, assign90800_e139456_d_n0, assign90800_e139456_d_n2, assign90800_e139456_d_n4, assign90800_e139456_d_n5, assign90800_e139456_d_n6, assign90800_e139456_d_n7, assign90800_e139456_d_n8, assign90800_e139456_d_n9, assign90800_e139456_d_n10, assign90800_e139456_d_n11, assign90800_e139456_d_n14,) = {
    if ((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) {
        let assign90800_e139452: f64 = (0.8 - locals.var_pb2over__blk2120);
        let assign90800_e139454: f64 = (assign90800_e139452 - 0.1);
        (assign90800_e139454, (-locals.var_pb2over__blk2120_dn0), (-locals.var_pb2over__blk2120_dn2), (-locals.var_pb2over__blk2120_dn4), (-locals.var_pb2over__blk2120_dn5), (-locals.var_pb2over__blk2120_dn6), (-locals.var_pb2over__blk2120_dn7), (-locals.var_pb2over__blk2120_dn8), (-locals.var_pb2over__blk2120_dn9), (-locals.var_pb2over__blk2120_dn10), (-locals.var_pb2over__blk2120_dn11), (-locals.var_pb2over__blk2120_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign90800_e139456;
        locals.var_tmf1_dn0 = assign90800_e139456_d_n0;
        locals.var_tmf1_dn2 = assign90800_e139456_d_n2;
        locals.var_tmf1_dn4 = assign90800_e139456_d_n4;
        locals.var_tmf1_dn5 = assign90800_e139456_d_n5;
        locals.var_tmf1_dn6 = assign90800_e139456_d_n6;
        locals.var_tmf1_dn7 = assign90800_e139456_d_n7;
        locals.var_tmf1_dn8 = assign90800_e139456_d_n8;
        locals.var_tmf1_dn9 = assign90800_e139456_d_n9;
        locals.var_tmf1_dn10 = assign90800_e139456_d_n10;
        locals.var_tmf1_dn11 = assign90800_e139456_d_n11;
        locals.var_tmf1_dn14 = assign90800_e139456_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign90810_e139466, assign90810_e139466_d_n0, assign90810_e139466_d_n2, assign90810_e139466_d_n4, assign90810_e139466_d_n5, assign90810_e139466_d_n6, assign90810_e139466_d_n7, assign90810_e139466_d_n8, assign90810_e139466_d_n9, assign90810_e139466_d_n10, assign90810_e139466_d_n11, assign90810_e139466_d_n14,) = {
    if ((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) {
        let assign90810_e139462: f64 = (4.0 * 0.8);
        let assign90810_e139464: f64 = (assign90810_e139462 * 0.1);
        (assign90810_e139464, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign90810_e139466;
        locals.var_tmf2_dn0 = assign90810_e139466_d_n0;
        locals.var_tmf2_dn2 = assign90810_e139466_d_n2;
        locals.var_tmf2_dn4 = assign90810_e139466_d_n4;
        locals.var_tmf2_dn5 = assign90810_e139466_d_n5;
        locals.var_tmf2_dn6 = assign90810_e139466_d_n6;
        locals.var_tmf2_dn7 = assign90810_e139466_d_n7;
        locals.var_tmf2_dn8 = assign90810_e139466_d_n8;
        locals.var_tmf2_dn9 = assign90810_e139466_d_n9;
        locals.var_tmf2_dn10 = assign90810_e139466_d_n10;
        locals.var_tmf2_dn11 = assign90810_e139466_d_n11;
        locals.var_tmf2_dn14 = assign90810_e139466_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign90820_e139478, assign90820_e139478_d_n0, assign90820_e139478_d_n2, assign90820_e139478_d_n4, assign90820_e139478_d_n5, assign90820_e139478_d_n6, assign90820_e139478_d_n7, assign90820_e139478_d_n8, assign90820_e139478_d_n9, assign90820_e139478_d_n10, assign90820_e139478_d_n11, assign90820_e139478_d_n14,) = {
    if ((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) {
        let (assign90820_e139476, assign90820_e139476_d_n0, assign90820_e139476_d_n2, assign90820_e139476_d_n4, assign90820_e139476_d_n5, assign90820_e139476_d_n6, assign90820_e139476_d_n7, assign90820_e139476_d_n8, assign90820_e139476_d_n9, assign90820_e139476_d_n10, assign90820_e139476_d_n11, assign90820_e139476_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign90820_e139475: f64 = (-locals.var_tmf2);
                (assign90820_e139475, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign90820_e139476, assign90820_e139476_d_n0, assign90820_e139476_d_n2, assign90820_e139476_d_n4, assign90820_e139476_d_n5, assign90820_e139476_d_n6, assign90820_e139476_d_n7, assign90820_e139476_d_n8, assign90820_e139476_d_n9, assign90820_e139476_d_n10, assign90820_e139476_d_n11, assign90820_e139476_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign90820_e139478;
        locals.var_tmf2_dn0 = assign90820_e139478_d_n0;
        locals.var_tmf2_dn2 = assign90820_e139478_d_n2;
        locals.var_tmf2_dn4 = assign90820_e139478_d_n4;
        locals.var_tmf2_dn5 = assign90820_e139478_d_n5;
        locals.var_tmf2_dn6 = assign90820_e139478_d_n6;
        locals.var_tmf2_dn7 = assign90820_e139478_d_n7;
        locals.var_tmf2_dn8 = assign90820_e139478_d_n8;
        locals.var_tmf2_dn9 = assign90820_e139478_d_n9;
        locals.var_tmf2_dn10 = assign90820_e139478_d_n10;
        locals.var_tmf2_dn11 = assign90820_e139478_d_n11;
        locals.var_tmf2_dn14 = assign90820_e139478_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign90830_e139489, assign90830_e139489_d_n0, assign90830_e139489_d_n2, assign90830_e139489_d_n4, assign90830_e139489_d_n5, assign90830_e139489_d_n6, assign90830_e139489_d_n7, assign90830_e139489_d_n8, assign90830_e139489_d_n9, assign90830_e139489_d_n10, assign90830_e139489_d_n11, assign90830_e139489_d_n14,) = {
    if ((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) {
        let assign90830_e139484: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign90830_e139486: f64 = (assign90830_e139484 + locals.var_tmf2);
        let assign90830_e139487: f64 = (assign90830_e139486).sqrt();
        (assign90830_e139487, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign90830_e139487)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign90830_e139487)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign90830_e139487)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign90830_e139487)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign90830_e139487)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign90830_e139487)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign90830_e139487)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign90830_e139487)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign90830_e139487)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign90830_e139487)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign90830_e139487)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign90830_e139489;
        locals.var_tmf2_dn0 = assign90830_e139489_d_n0;
        locals.var_tmf2_dn2 = assign90830_e139489_d_n2;
        locals.var_tmf2_dn4 = assign90830_e139489_d_n4;
        locals.var_tmf2_dn5 = assign90830_e139489_d_n5;
        locals.var_tmf2_dn6 = assign90830_e139489_d_n6;
        locals.var_tmf2_dn7 = assign90830_e139489_d_n7;
        locals.var_tmf2_dn8 = assign90830_e139489_d_n8;
        locals.var_tmf2_dn9 = assign90830_e139489_d_n9;
        locals.var_tmf2_dn10 = assign90830_e139489_d_n10;
        locals.var_tmf2_dn11 = assign90830_e139489_d_n11;
        locals.var_tmf2_dn14 = assign90830_e139489_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign90840_e139501, assign90840_e139501_d_n0, assign90840_e139501_d_n2, assign90840_e139501_d_n4, assign90840_e139501_d_n5, assign90840_e139501_d_n6, assign90840_e139501_d_n7, assign90840_e139501_d_n8, assign90840_e139501_d_n9, assign90840_e139501_d_n10, assign90840_e139501_d_n11, assign90840_e139501_d_n14,) = {
    if ((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) {
        let assign90840_e139497: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign90840_e139498: f64 = (1.0 + assign90840_e139497);
        let assign90840_e139499: f64 = (0.5 * assign90840_e139498);
        (assign90840_e139499, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign90840_e139501;
        locals.var_t0_dn0 = assign90840_e139501_d_n0;
        locals.var_t0_dn2 = assign90840_e139501_d_n2;
        locals.var_t0_dn4 = assign90840_e139501_d_n4;
        locals.var_t0_dn5 = assign90840_e139501_d_n5;
        locals.var_t0_dn6 = assign90840_e139501_d_n6;
        locals.var_t0_dn7 = assign90840_e139501_d_n7;
        locals.var_t0_dn8 = assign90840_e139501_d_n8;
        locals.var_t0_dn9 = assign90840_e139501_d_n9;
        locals.var_t0_dn10 = assign90840_e139501_d_n10;
        locals.var_t0_dn11 = assign90840_e139501_d_n11;
        locals.var_t0_dn14 = assign90840_e139501_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign90850_e139513, assign90850_e139513_d_n0, assign90850_e139513_d_n2, assign90850_e139513_d_n4, assign90850_e139513_d_n5, assign90850_e139513_d_n6, assign90850_e139513_d_n7, assign90850_e139513_d_n8, assign90850_e139513_d_n9, assign90850_e139513_d_n10, assign90850_e139513_d_n11, assign90850_e139513_d_n14,) = {
    if ((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) {
        let assign90850_e139509: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign90850_e139510: f64 = (0.5 * assign90850_e139509);
        let assign90850_e139511: f64 = (0.8 - assign90850_e139510);
        (assign90850_e139511, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (-(0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (-(0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (-(0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (-(0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (-(0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (-(0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (-(0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (-(0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (-(0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_vbs_max_over__blk2121, locals.var_vbs_max_over__blk2121_dn0, locals.var_vbs_max_over__blk2121_dn2, locals.var_vbs_max_over__blk2121_dn4, locals.var_vbs_max_over__blk2121_dn5, locals.var_vbs_max_over__blk2121_dn6, locals.var_vbs_max_over__blk2121_dn7, locals.var_vbs_max_over__blk2121_dn8, locals.var_vbs_max_over__blk2121_dn9, locals.var_vbs_max_over__blk2121_dn10, locals.var_vbs_max_over__blk2121_dn11, locals.var_vbs_max_over__blk2121_dn14,)
    }
};
        locals.var_vbs_max_over__blk2121 = assign90850_e139513;
        locals.var_vbs_max_over__blk2121_dn0 = assign90850_e139513_d_n0;
        locals.var_vbs_max_over__blk2121_dn2 = assign90850_e139513_d_n2;
        locals.var_vbs_max_over__blk2121_dn4 = assign90850_e139513_d_n4;
        locals.var_vbs_max_over__blk2121_dn5 = assign90850_e139513_d_n5;
        locals.var_vbs_max_over__blk2121_dn6 = assign90850_e139513_d_n6;
        locals.var_vbs_max_over__blk2121_dn7 = assign90850_e139513_d_n7;
        locals.var_vbs_max_over__blk2121_dn8 = assign90850_e139513_d_n8;
        locals.var_vbs_max_over__blk2121_dn9 = assign90850_e139513_d_n9;
        locals.var_vbs_max_over__blk2121_dn10 = assign90850_e139513_d_n10;
        locals.var_vbs_max_over__blk2121_dn11 = assign90850_e139513_d_n11;
        locals.var_vbs_max_over__blk2121_dn14 = assign90850_e139513_d_n14;
        locals.var_vbs_max_over__blk2121_rv = 0.0;

        let assign90860_e139517: f64 = (locals.var_vbs_max_over__blk2121 * 0.5);
        let assign90860_e139518: f64 = if locals.var_vbs_bnd_over__blk2122 > assign90860_e139517 { 1.0 } else { 0.0 };
        locals.var_guard2127 = assign90860_e139518;
        locals.var_guard2127_rv = 0.0;

        let (assign90870_e139528, assign90870_e139528_d_n0, assign90870_e139528_d_n2, assign90870_e139528_d_n4, assign90870_e139528_d_n5, assign90870_e139528_d_n6, assign90870_e139528_d_n7, assign90870_e139528_d_n8, assign90870_e139528_d_n9, assign90870_e139528_d_n10, assign90870_e139528_d_n11, assign90870_e139528_d_n14,) = {
    if (((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2127 != 0.0)) {
        let assign90870_e139526: f64 = (0.5 * locals.var_vbs_max_over__blk2121);
        (assign90870_e139526, (0.5 * locals.var_vbs_max_over__blk2121_dn0), (0.5 * locals.var_vbs_max_over__blk2121_dn2), (0.5 * locals.var_vbs_max_over__blk2121_dn4), (0.5 * locals.var_vbs_max_over__blk2121_dn5), (0.5 * locals.var_vbs_max_over__blk2121_dn6), (0.5 * locals.var_vbs_max_over__blk2121_dn7), (0.5 * locals.var_vbs_max_over__blk2121_dn8), (0.5 * locals.var_vbs_max_over__blk2121_dn9), (0.5 * locals.var_vbs_max_over__blk2121_dn10), (0.5 * locals.var_vbs_max_over__blk2121_dn11), (0.5 * locals.var_vbs_max_over__blk2121_dn14),)
    } else {
        (locals.var_vbs_bnd_over__blk2122, locals.var_vbs_bnd_over__blk2122_dn0, locals.var_vbs_bnd_over__blk2122_dn2, locals.var_vbs_bnd_over__blk2122_dn4, locals.var_vbs_bnd_over__blk2122_dn5, locals.var_vbs_bnd_over__blk2122_dn6, locals.var_vbs_bnd_over__blk2122_dn7, locals.var_vbs_bnd_over__blk2122_dn8, locals.var_vbs_bnd_over__blk2122_dn9, locals.var_vbs_bnd_over__blk2122_dn10, locals.var_vbs_bnd_over__blk2122_dn11, locals.var_vbs_bnd_over__blk2122_dn14,)
    }
};
        locals.var_vbs_bnd_over__blk2122 = assign90870_e139528;
        locals.var_vbs_bnd_over__blk2122_dn0 = assign90870_e139528_d_n0;
        locals.var_vbs_bnd_over__blk2122_dn2 = assign90870_e139528_d_n2;
        locals.var_vbs_bnd_over__blk2122_dn4 = assign90870_e139528_d_n4;
        locals.var_vbs_bnd_over__blk2122_dn5 = assign90870_e139528_d_n5;
        locals.var_vbs_bnd_over__blk2122_dn6 = assign90870_e139528_d_n6;
        locals.var_vbs_bnd_over__blk2122_dn7 = assign90870_e139528_d_n7;
        locals.var_vbs_bnd_over__blk2122_dn8 = assign90870_e139528_d_n8;
        locals.var_vbs_bnd_over__blk2122_dn9 = assign90870_e139528_d_n9;
        locals.var_vbs_bnd_over__blk2122_dn10 = assign90870_e139528_d_n10;
        locals.var_vbs_bnd_over__blk2122_dn11 = assign90870_e139528_d_n11;
        locals.var_vbs_bnd_over__blk2122_dn14 = assign90870_e139528_d_n14;
        locals.var_vbs_bnd_over__blk2122_rv = 0.0;

        let assign90880_e139530: f64 = if param_given[338] { 1.0 } else { 0.0 };
        locals.var_guard2128 = assign90880_e139530;
        locals.var_guard2128_rv = 0.0;

        let (assign90890_e139538, assign90890_e139538_d_n0, assign90890_e139538_d_n2, assign90890_e139538_d_n4, assign90890_e139538_d_n5, assign90890_e139538_d_n6, assign90890_e139538_d_n7, assign90890_e139538_d_n8, assign90890_e139538_d_n9, assign90890_e139538_d_n10, assign90890_e139538_d_n11, assign90890_e139538_d_n14,) = {
    if (((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2128 != 0.0)) {
        (p.p338, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbs_max_over__blk2121, locals.var_vbs_max_over__blk2121_dn0, locals.var_vbs_max_over__blk2121_dn2, locals.var_vbs_max_over__blk2121_dn4, locals.var_vbs_max_over__blk2121_dn5, locals.var_vbs_max_over__blk2121_dn6, locals.var_vbs_max_over__blk2121_dn7, locals.var_vbs_max_over__blk2121_dn8, locals.var_vbs_max_over__blk2121_dn9, locals.var_vbs_max_over__blk2121_dn10, locals.var_vbs_max_over__blk2121_dn11, locals.var_vbs_max_over__blk2121_dn14,)
    }
};
        locals.var_vbs_max_over__blk2121 = assign90890_e139538;
        locals.var_vbs_max_over__blk2121_dn0 = assign90890_e139538_d_n0;
        locals.var_vbs_max_over__blk2121_dn2 = assign90890_e139538_d_n2;
        locals.var_vbs_max_over__blk2121_dn4 = assign90890_e139538_d_n4;
        locals.var_vbs_max_over__blk2121_dn5 = assign90890_e139538_d_n5;
        locals.var_vbs_max_over__blk2121_dn6 = assign90890_e139538_d_n6;
        locals.var_vbs_max_over__blk2121_dn7 = assign90890_e139538_d_n7;
        locals.var_vbs_max_over__blk2121_dn8 = assign90890_e139538_d_n8;
        locals.var_vbs_max_over__blk2121_dn9 = assign90890_e139538_d_n9;
        locals.var_vbs_max_over__blk2121_dn10 = assign90890_e139538_d_n10;
        locals.var_vbs_max_over__blk2121_dn11 = assign90890_e139538_d_n11;
        locals.var_vbs_max_over__blk2121_dn14 = assign90890_e139538_d_n14;
        locals.var_vbs_max_over__blk2121_rv = 0.0;

        let assign90900_e139540: f64 = if param_given[339] { 1.0 } else { 0.0 };
        locals.var_guard2129 = assign90900_e139540;
        locals.var_guard2129_rv = 0.0;

        let (assign90910_e139548, assign90910_e139548_d_n0, assign90910_e139548_d_n2, assign90910_e139548_d_n4, assign90910_e139548_d_n5, assign90910_e139548_d_n6, assign90910_e139548_d_n7, assign90910_e139548_d_n8, assign90910_e139548_d_n9, assign90910_e139548_d_n10, assign90910_e139548_d_n11, assign90910_e139548_d_n14,) = {
    if (((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2129 != 0.0)) {
        (p.p339, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbs_bnd_over__blk2122, locals.var_vbs_bnd_over__blk2122_dn0, locals.var_vbs_bnd_over__blk2122_dn2, locals.var_vbs_bnd_over__blk2122_dn4, locals.var_vbs_bnd_over__blk2122_dn5, locals.var_vbs_bnd_over__blk2122_dn6, locals.var_vbs_bnd_over__blk2122_dn7, locals.var_vbs_bnd_over__blk2122_dn8, locals.var_vbs_bnd_over__blk2122_dn9, locals.var_vbs_bnd_over__blk2122_dn10, locals.var_vbs_bnd_over__blk2122_dn11, locals.var_vbs_bnd_over__blk2122_dn14,)
    }
};
        locals.var_vbs_bnd_over__blk2122 = assign90910_e139548;
        locals.var_vbs_bnd_over__blk2122_dn0 = assign90910_e139548_d_n0;
        locals.var_vbs_bnd_over__blk2122_dn2 = assign90910_e139548_d_n2;
        locals.var_vbs_bnd_over__blk2122_dn4 = assign90910_e139548_d_n4;
        locals.var_vbs_bnd_over__blk2122_dn5 = assign90910_e139548_d_n5;
        locals.var_vbs_bnd_over__blk2122_dn6 = assign90910_e139548_d_n6;
        locals.var_vbs_bnd_over__blk2122_dn7 = assign90910_e139548_d_n7;
        locals.var_vbs_bnd_over__blk2122_dn8 = assign90910_e139548_d_n8;
        locals.var_vbs_bnd_over__blk2122_dn9 = assign90910_e139548_d_n9;
        locals.var_vbs_bnd_over__blk2122_dn10 = assign90910_e139548_d_n10;
        locals.var_vbs_bnd_over__blk2122_dn11 = assign90910_e139548_d_n11;
        locals.var_vbs_bnd_over__blk2122_dn14 = assign90910_e139548_d_n14;
        locals.var_vbs_bnd_over__blk2122_rv = 0.0;

        let assign90920_e139550: f64 = if param_given[338] { 1.0 } else { 0.0 };
        locals.var_guard2130 = assign90920_e139550;
        locals.var_guard2130_rv = 0.0;

        let (assign90930_e139563, assign90930_e139563_d_n0, assign90930_e139563_d_n2, assign90930_e139563_d_n4, assign90930_e139563_d_n5, assign90930_e139563_d_n6, assign90930_e139563_d_n7, assign90930_e139563_d_n8, assign90930_e139563_d_n9, assign90930_e139563_d_n10, assign90930_e139563_d_n11, assign90930_e139563_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2129 == 0.0)) && (locals.var_guard2130 != 0.0)) {
        let assign90930_e139561: f64 = (0.5 * locals.var_vbs_max_over__blk2121);
        (assign90930_e139561, (0.5 * locals.var_vbs_max_over__blk2121_dn0), (0.5 * locals.var_vbs_max_over__blk2121_dn2), (0.5 * locals.var_vbs_max_over__blk2121_dn4), (0.5 * locals.var_vbs_max_over__blk2121_dn5), (0.5 * locals.var_vbs_max_over__blk2121_dn6), (0.5 * locals.var_vbs_max_over__blk2121_dn7), (0.5 * locals.var_vbs_max_over__blk2121_dn8), (0.5 * locals.var_vbs_max_over__blk2121_dn9), (0.5 * locals.var_vbs_max_over__blk2121_dn10), (0.5 * locals.var_vbs_max_over__blk2121_dn11), (0.5 * locals.var_vbs_max_over__blk2121_dn14),)
    } else {
        (locals.var_vbs_bnd_over__blk2122, locals.var_vbs_bnd_over__blk2122_dn0, locals.var_vbs_bnd_over__blk2122_dn2, locals.var_vbs_bnd_over__blk2122_dn4, locals.var_vbs_bnd_over__blk2122_dn5, locals.var_vbs_bnd_over__blk2122_dn6, locals.var_vbs_bnd_over__blk2122_dn7, locals.var_vbs_bnd_over__blk2122_dn8, locals.var_vbs_bnd_over__blk2122_dn9, locals.var_vbs_bnd_over__blk2122_dn10, locals.var_vbs_bnd_over__blk2122_dn11, locals.var_vbs_bnd_over__blk2122_dn14,)
    }
};
        locals.var_vbs_bnd_over__blk2122 = assign90930_e139563;
        locals.var_vbs_bnd_over__blk2122_dn0 = assign90930_e139563_d_n0;
        locals.var_vbs_bnd_over__blk2122_dn2 = assign90930_e139563_d_n2;
        locals.var_vbs_bnd_over__blk2122_dn4 = assign90930_e139563_d_n4;
        locals.var_vbs_bnd_over__blk2122_dn5 = assign90930_e139563_d_n5;
        locals.var_vbs_bnd_over__blk2122_dn6 = assign90930_e139563_d_n6;
        locals.var_vbs_bnd_over__blk2122_dn7 = assign90930_e139563_d_n7;
        locals.var_vbs_bnd_over__blk2122_dn8 = assign90930_e139563_d_n8;
        locals.var_vbs_bnd_over__blk2122_dn9 = assign90930_e139563_d_n9;
        locals.var_vbs_bnd_over__blk2122_dn10 = assign90930_e139563_d_n10;
        locals.var_vbs_bnd_over__blk2122_dn11 = assign90930_e139563_d_n11;
        locals.var_vbs_bnd_over__blk2122_dn14 = assign90930_e139563_d_n14;
        locals.var_vbs_bnd_over__blk2122_rv = 0.0;

        let assign90940_e139567: f64 = (locals.var_vbs_max_over__blk2121 * 0.5);
        let assign90940_e139568: f64 = if locals.var_vbs_bnd_over__blk2122 > assign90940_e139567 { 1.0 } else { 0.0 };
        locals.var_guard2131 = assign90940_e139568;
        locals.var_guard2131_rv = 0.0;

        let (assign90950_e139578, assign90950_e139578_d_n0, assign90950_e139578_d_n2, assign90950_e139578_d_n4, assign90950_e139578_d_n5, assign90950_e139578_d_n6, assign90950_e139578_d_n7, assign90950_e139578_d_n8, assign90950_e139578_d_n9, assign90950_e139578_d_n10, assign90950_e139578_d_n11, assign90950_e139578_d_n14,) = {
    if (((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2131 != 0.0)) {
        let assign90950_e139576: f64 = (0.5 * locals.var_vbs_max_over__blk2121);
        (assign90950_e139576, (0.5 * locals.var_vbs_max_over__blk2121_dn0), (0.5 * locals.var_vbs_max_over__blk2121_dn2), (0.5 * locals.var_vbs_max_over__blk2121_dn4), (0.5 * locals.var_vbs_max_over__blk2121_dn5), (0.5 * locals.var_vbs_max_over__blk2121_dn6), (0.5 * locals.var_vbs_max_over__blk2121_dn7), (0.5 * locals.var_vbs_max_over__blk2121_dn8), (0.5 * locals.var_vbs_max_over__blk2121_dn9), (0.5 * locals.var_vbs_max_over__blk2121_dn10), (0.5 * locals.var_vbs_max_over__blk2121_dn11), (0.5 * locals.var_vbs_max_over__blk2121_dn14),)
    } else {
        (locals.var_vbs_bnd_over__blk2122, locals.var_vbs_bnd_over__blk2122_dn0, locals.var_vbs_bnd_over__blk2122_dn2, locals.var_vbs_bnd_over__blk2122_dn4, locals.var_vbs_bnd_over__blk2122_dn5, locals.var_vbs_bnd_over__blk2122_dn6, locals.var_vbs_bnd_over__blk2122_dn7, locals.var_vbs_bnd_over__blk2122_dn8, locals.var_vbs_bnd_over__blk2122_dn9, locals.var_vbs_bnd_over__blk2122_dn10, locals.var_vbs_bnd_over__blk2122_dn11, locals.var_vbs_bnd_over__blk2122_dn14,)
    }
};
        locals.var_vbs_bnd_over__blk2122 = assign90950_e139578;
        locals.var_vbs_bnd_over__blk2122_dn0 = assign90950_e139578_d_n0;
        locals.var_vbs_bnd_over__blk2122_dn2 = assign90950_e139578_d_n2;
        locals.var_vbs_bnd_over__blk2122_dn4 = assign90950_e139578_d_n4;
        locals.var_vbs_bnd_over__blk2122_dn5 = assign90950_e139578_d_n5;
        locals.var_vbs_bnd_over__blk2122_dn6 = assign90950_e139578_d_n6;
        locals.var_vbs_bnd_over__blk2122_dn7 = assign90950_e139578_d_n7;
        locals.var_vbs_bnd_over__blk2122_dn8 = assign90950_e139578_d_n8;
        locals.var_vbs_bnd_over__blk2122_dn9 = assign90950_e139578_d_n9;
        locals.var_vbs_bnd_over__blk2122_dn10 = assign90950_e139578_d_n10;
        locals.var_vbs_bnd_over__blk2122_dn11 = assign90950_e139578_d_n11;
        locals.var_vbs_bnd_over__blk2122_dn14 = assign90950_e139578_d_n14;
        locals.var_vbs_bnd_over__blk2122_rv = 0.0;

        let assign90960_e139581: f64 = if p.p38 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2132 = assign90960_e139581;
        locals.var_guard2132_rv = 0.0;

        let (assign90970_e139590, assign90970_e139590_d_n0, assign90970_e139590_d_n2, assign90970_e139590_d_n4, assign90970_e139590_d_n5, assign90970_e139590_d_n6, assign90970_e139590_d_n7, assign90970_e139590_d_n8, assign90970_e139590_d_n9, assign90970_e139590_d_n10, assign90970_e139590_d_n11, assign90970_e139590_d_n14,) = {
    if (((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2132 != 0.0)) {
        let assign90970_e139588: f64 = (-locals.var_vxbgmt);
        (assign90970_e139588, (-locals.var_vxbgmt_dn0), (-locals.var_vxbgmt_dn2), (-locals.var_vxbgmt_dn4), (-locals.var_vxbgmt_dn5), (-locals.var_vxbgmt_dn6), (-locals.var_vxbgmt_dn7), (-locals.var_vxbgmt_dn8), (-locals.var_vxbgmt_dn9), (-locals.var_vxbgmt_dn10), (-locals.var_vxbgmt_dn11), (-locals.var_vxbgmt_dn14),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign90970_e139590;
        locals.var_t0_dn0 = assign90970_e139590_d_n0;
        locals.var_t0_dn2 = assign90970_e139590_d_n2;
        locals.var_t0_dn4 = assign90970_e139590_d_n4;
        locals.var_t0_dn5 = assign90970_e139590_d_n5;
        locals.var_t0_dn6 = assign90970_e139590_d_n6;
        locals.var_t0_dn7 = assign90970_e139590_d_n7;
        locals.var_t0_dn8 = assign90970_e139590_d_n8;
        locals.var_t0_dn9 = assign90970_e139590_d_n9;
        locals.var_t0_dn10 = assign90970_e139590_d_n10;
        locals.var_t0_dn11 = assign90970_e139590_d_n11;
        locals.var_t0_dn14 = assign90970_e139590_d_n14;
        locals.var_t0_rv = 0.0;

        let assign90980_e139593: f64 = if locals.var_t0 > locals.var_vbs_bnd_over__blk2122 { 1.0 } else { 0.0 };
        locals.var_guard2133 = assign90980_e139593;
        locals.var_guard2133_rv = 0.0;

        let (assign90990_e139605, assign90990_e139605_d_n0, assign90990_e139605_d_n2, assign90990_e139605_d_n4, assign90990_e139605_d_n5, assign90990_e139605_d_n6, assign90990_e139605_d_n7, assign90990_e139605_d_n8, assign90990_e139605_d_n9, assign90990_e139605_d_n10, assign90990_e139605_d_n11, assign90990_e139605_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2132 != 0.0)) && (locals.var_guard2133 != 0.0)) {
        let assign90990_e139603: f64 = (locals.var_t0 - locals.var_vbs_bnd_over__blk2122);
        (assign90990_e139603, (locals.var_t0_dn0 - locals.var_vbs_bnd_over__blk2122_dn0), (locals.var_t0_dn2 - locals.var_vbs_bnd_over__blk2122_dn2), (locals.var_t0_dn4 - locals.var_vbs_bnd_over__blk2122_dn4), (locals.var_t0_dn5 - locals.var_vbs_bnd_over__blk2122_dn5), (locals.var_t0_dn6 - locals.var_vbs_bnd_over__blk2122_dn6), (locals.var_t0_dn7 - locals.var_vbs_bnd_over__blk2122_dn7), (locals.var_t0_dn8 - locals.var_vbs_bnd_over__blk2122_dn8), (locals.var_t0_dn9 - locals.var_vbs_bnd_over__blk2122_dn9), (locals.var_t0_dn10 - locals.var_vbs_bnd_over__blk2122_dn10), (locals.var_t0_dn11 - locals.var_vbs_bnd_over__blk2122_dn11), (locals.var_t0_dn14 - locals.var_vbs_bnd_over__blk2122_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign90990_e139605;
        locals.var_t1_dn0 = assign90990_e139605_d_n0;
        locals.var_t1_dn2 = assign90990_e139605_d_n2;
        locals.var_t1_dn4 = assign90990_e139605_d_n4;
        locals.var_t1_dn5 = assign90990_e139605_d_n5;
        locals.var_t1_dn6 = assign90990_e139605_d_n6;
        locals.var_t1_dn7 = assign90990_e139605_d_n7;
        locals.var_t1_dn8 = assign90990_e139605_d_n8;
        locals.var_t1_dn9 = assign90990_e139605_d_n9;
        locals.var_t1_dn10 = assign90990_e139605_d_n10;
        locals.var_t1_dn11 = assign90990_e139605_d_n11;
        locals.var_t1_dn14 = assign90990_e139605_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign91000_e139617, assign91000_e139617_d_n0, assign91000_e139617_d_n2, assign91000_e139617_d_n4, assign91000_e139617_d_n5, assign91000_e139617_d_n6, assign91000_e139617_d_n7, assign91000_e139617_d_n8, assign91000_e139617_d_n9, assign91000_e139617_d_n10, assign91000_e139617_d_n11, assign91000_e139617_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2132 != 0.0)) && (locals.var_guard2133 != 0.0)) {
        let assign91000_e139615: f64 = (locals.var_vbs_max_over__blk2121 - locals.var_vbs_bnd_over__blk2122);
        (assign91000_e139615, (locals.var_vbs_max_over__blk2121_dn0 - locals.var_vbs_bnd_over__blk2122_dn0), (locals.var_vbs_max_over__blk2121_dn2 - locals.var_vbs_bnd_over__blk2122_dn2), (locals.var_vbs_max_over__blk2121_dn4 - locals.var_vbs_bnd_over__blk2122_dn4), (locals.var_vbs_max_over__blk2121_dn5 - locals.var_vbs_bnd_over__blk2122_dn5), (locals.var_vbs_max_over__blk2121_dn6 - locals.var_vbs_bnd_over__blk2122_dn6), (locals.var_vbs_max_over__blk2121_dn7 - locals.var_vbs_bnd_over__blk2122_dn7), (locals.var_vbs_max_over__blk2121_dn8 - locals.var_vbs_bnd_over__blk2122_dn8), (locals.var_vbs_max_over__blk2121_dn9 - locals.var_vbs_bnd_over__blk2122_dn9), (locals.var_vbs_max_over__blk2121_dn10 - locals.var_vbs_bnd_over__blk2122_dn10), (locals.var_vbs_max_over__blk2121_dn11 - locals.var_vbs_bnd_over__blk2122_dn11), (locals.var_vbs_max_over__blk2121_dn14 - locals.var_vbs_bnd_over__blk2122_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign91000_e139617;
        locals.var_t2_dn0 = assign91000_e139617_d_n0;
        locals.var_t2_dn2 = assign91000_e139617_d_n2;
        locals.var_t2_dn4 = assign91000_e139617_d_n4;
        locals.var_t2_dn5 = assign91000_e139617_d_n5;
        locals.var_t2_dn6 = assign91000_e139617_d_n6;
        locals.var_t2_dn7 = assign91000_e139617_d_n7;
        locals.var_t2_dn8 = assign91000_e139617_d_n8;
        locals.var_t2_dn9 = assign91000_e139617_d_n9;
        locals.var_t2_dn10 = assign91000_e139617_d_n10;
        locals.var_t2_dn11 = assign91000_e139617_d_n11;
        locals.var_t2_dn14 = assign91000_e139617_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign91010_e139629, assign91010_e139629_d_n0, assign91010_e139629_d_n2, assign91010_e139629_d_n4, assign91010_e139629_d_n5, assign91010_e139629_d_n6, assign91010_e139629_d_n7, assign91010_e139629_d_n8, assign91010_e139629_d_n9, assign91010_e139629_d_n10, assign91010_e139629_d_n11, assign91010_e139629_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2132 != 0.0)) && (locals.var_guard2133 != 0.0)) {
        let assign91010_e139627: f64 = (locals.var_t1 / locals.var_t2);
        (assign91010_e139627, (((locals.var_t1_dn0 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn0)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn2 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn2)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn4 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn4)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn5 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn5)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn6 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn6)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn7 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn7)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn8 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn8)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn9 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn9)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn10 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn10)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn11 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn11)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn14 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn14)) / (locals.var_t2 * locals.var_t2)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign91010_e139629;
        locals.var_tmf1_dn0 = assign91010_e139629_d_n0;
        locals.var_tmf1_dn2 = assign91010_e139629_d_n2;
        locals.var_tmf1_dn4 = assign91010_e139629_d_n4;
        locals.var_tmf1_dn5 = assign91010_e139629_d_n5;
        locals.var_tmf1_dn6 = assign91010_e139629_d_n6;
        locals.var_tmf1_dn7 = assign91010_e139629_d_n7;
        locals.var_tmf1_dn8 = assign91010_e139629_d_n8;
        locals.var_tmf1_dn9 = assign91010_e139629_d_n9;
        locals.var_tmf1_dn10 = assign91010_e139629_d_n10;
        locals.var_tmf1_dn11 = assign91010_e139629_d_n11;
        locals.var_tmf1_dn14 = assign91010_e139629_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign91020_e139641, assign91020_e139641_d_n0, assign91020_e139641_d_n2, assign91020_e139641_d_n4, assign91020_e139641_d_n5, assign91020_e139641_d_n6, assign91020_e139641_d_n7, assign91020_e139641_d_n8, assign91020_e139641_d_n9, assign91020_e139641_d_n10, assign91020_e139641_d_n11, assign91020_e139641_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2132 != 0.0)) && (locals.var_guard2133 != 0.0)) {
        let assign91020_e139639: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign91020_e139639, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign91020_e139641;
        locals.var_tmf2_dn0 = assign91020_e139641_d_n0;
        locals.var_tmf2_dn2 = assign91020_e139641_d_n2;
        locals.var_tmf2_dn4 = assign91020_e139641_d_n4;
        locals.var_tmf2_dn5 = assign91020_e139641_d_n5;
        locals.var_tmf2_dn6 = assign91020_e139641_d_n6;
        locals.var_tmf2_dn7 = assign91020_e139641_d_n7;
        locals.var_tmf2_dn8 = assign91020_e139641_d_n8;
        locals.var_tmf2_dn9 = assign91020_e139641_d_n9;
        locals.var_tmf2_dn10 = assign91020_e139641_d_n10;
        locals.var_tmf2_dn11 = assign91020_e139641_d_n11;
        locals.var_tmf2_dn14 = assign91020_e139641_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign91030_e139653, assign91030_e139653_d_n0, assign91030_e139653_d_n2, assign91030_e139653_d_n4, assign91030_e139653_d_n5, assign91030_e139653_d_n6, assign91030_e139653_d_n7, assign91030_e139653_d_n8, assign91030_e139653_d_n9, assign91030_e139653_d_n10, assign91030_e139653_d_n11, assign91030_e139653_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2132 != 0.0)) && (locals.var_guard2133 != 0.0)) {
        let assign91030_e139651: f64 = (locals.var_tmf2 * locals.var_tmf1);
        (assign91030_e139651, ((locals.var_tmf2_dn0 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn0)), ((locals.var_tmf2_dn2 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn2)), ((locals.var_tmf2_dn4 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn4)), ((locals.var_tmf2_dn5 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn5)), ((locals.var_tmf2_dn6 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn6)), ((locals.var_tmf2_dn7 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn7)), ((locals.var_tmf2_dn8 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn8)), ((locals.var_tmf2_dn9 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn9)), ((locals.var_tmf2_dn10 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn10)), ((locals.var_tmf2_dn11 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn11)), ((locals.var_tmf2_dn14 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_tmf3, locals.var_tmf3_dn0, locals.var_tmf3_dn2, locals.var_tmf3_dn4, locals.var_tmf3_dn5, locals.var_tmf3_dn6, locals.var_tmf3_dn7, locals.var_tmf3_dn8, locals.var_tmf3_dn9, locals.var_tmf3_dn10, locals.var_tmf3_dn11, locals.var_tmf3_dn14,)
    }
};
        locals.var_tmf3 = assign91030_e139653;
        locals.var_tmf3_dn0 = assign91030_e139653_d_n0;
        locals.var_tmf3_dn2 = assign91030_e139653_d_n2;
        locals.var_tmf3_dn4 = assign91030_e139653_d_n4;
        locals.var_tmf3_dn5 = assign91030_e139653_d_n5;
        locals.var_tmf3_dn6 = assign91030_e139653_d_n6;
        locals.var_tmf3_dn7 = assign91030_e139653_d_n7;
        locals.var_tmf3_dn8 = assign91030_e139653_d_n8;
        locals.var_tmf3_dn9 = assign91030_e139653_d_n9;
        locals.var_tmf3_dn10 = assign91030_e139653_d_n10;
        locals.var_tmf3_dn11 = assign91030_e139653_d_n11;
        locals.var_tmf3_dn14 = assign91030_e139653_d_n14;
        locals.var_tmf3_rv = 0.0;

        let (assign91040_e139665, assign91040_e139665_d_n0, assign91040_e139665_d_n2, assign91040_e139665_d_n4, assign91040_e139665_d_n5, assign91040_e139665_d_n6, assign91040_e139665_d_n7, assign91040_e139665_d_n8, assign91040_e139665_d_n9, assign91040_e139665_d_n10, assign91040_e139665_d_n11, assign91040_e139665_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2132 != 0.0)) && (locals.var_guard2133 != 0.0)) {
        let assign91040_e139663: f64 = (locals.var_tmf2 * locals.var_tmf2);
        (assign91040_e139663, ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0)), ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2)), ((locals.var_tmf2_dn4 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn4)), ((locals.var_tmf2_dn5 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn5)), ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6)), ((locals.var_tmf2_dn7 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn7)), ((locals.var_tmf2_dn8 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn8)), ((locals.var_tmf2_dn9 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn9)), ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10)), ((locals.var_tmf2_dn11 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn11)), ((locals.var_tmf2_dn14 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn14)),)
    } else {
        (locals.var_tmf4, locals.var_tmf4_dn0, locals.var_tmf4_dn2, locals.var_tmf4_dn4, locals.var_tmf4_dn5, locals.var_tmf4_dn6, locals.var_tmf4_dn7, locals.var_tmf4_dn8, locals.var_tmf4_dn9, locals.var_tmf4_dn10, locals.var_tmf4_dn11, locals.var_tmf4_dn14,)
    }
};
        locals.var_tmf4 = assign91040_e139665;
        locals.var_tmf4_dn0 = assign91040_e139665_d_n0;
        locals.var_tmf4_dn2 = assign91040_e139665_d_n2;
        locals.var_tmf4_dn4 = assign91040_e139665_d_n4;
        locals.var_tmf4_dn5 = assign91040_e139665_d_n5;
        locals.var_tmf4_dn6 = assign91040_e139665_d_n6;
        locals.var_tmf4_dn7 = assign91040_e139665_d_n7;
        locals.var_tmf4_dn8 = assign91040_e139665_d_n8;
        locals.var_tmf4_dn9 = assign91040_e139665_d_n9;
        locals.var_tmf4_dn10 = assign91040_e139665_d_n10;
        locals.var_tmf4_dn11 = assign91040_e139665_d_n11;
        locals.var_tmf4_dn14 = assign91040_e139665_d_n14;
        locals.var_tmf4_rv = 0.0;

        let (assign91050_e139685, assign91050_e139685_d_n0, assign91050_e139685_d_n2, assign91050_e139685_d_n4, assign91050_e139685_d_n5, assign91050_e139685_d_n6, assign91050_e139685_d_n7, assign91050_e139685_d_n8, assign91050_e139685_d_n9, assign91050_e139685_d_n10, assign91050_e139685_d_n11, assign91050_e139685_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2132 != 0.0)) && (locals.var_guard2133 != 0.0)) {
        let assign91050_e139676: f64 = (1.0 + locals.var_tmf1);
        let assign91050_e139678: f64 = (assign91050_e139676 + locals.var_tmf2);
        let assign91050_e139680: f64 = (assign91050_e139678 + locals.var_tmf3);
        let assign91050_e139682: f64 = (assign91050_e139680 + locals.var_tmf4);
        let assign91050_e139683: f64 = (1.0 / assign91050_e139682);
        (assign91050_e139683, (-((((locals.var_tmf1_dn0 + locals.var_tmf2_dn0) + locals.var_tmf3_dn0) + locals.var_tmf4_dn0) / (assign91050_e139682 * assign91050_e139682))), (-((((locals.var_tmf1_dn2 + locals.var_tmf2_dn2) + locals.var_tmf3_dn2) + locals.var_tmf4_dn2) / (assign91050_e139682 * assign91050_e139682))), (-((((locals.var_tmf1_dn4 + locals.var_tmf2_dn4) + locals.var_tmf3_dn4) + locals.var_tmf4_dn4) / (assign91050_e139682 * assign91050_e139682))), (-((((locals.var_tmf1_dn5 + locals.var_tmf2_dn5) + locals.var_tmf3_dn5) + locals.var_tmf4_dn5) / (assign91050_e139682 * assign91050_e139682))), (-((((locals.var_tmf1_dn6 + locals.var_tmf2_dn6) + locals.var_tmf3_dn6) + locals.var_tmf4_dn6) / (assign91050_e139682 * assign91050_e139682))), (-((((locals.var_tmf1_dn7 + locals.var_tmf2_dn7) + locals.var_tmf3_dn7) + locals.var_tmf4_dn7) / (assign91050_e139682 * assign91050_e139682))), (-((((locals.var_tmf1_dn8 + locals.var_tmf2_dn8) + locals.var_tmf3_dn8) + locals.var_tmf4_dn8) / (assign91050_e139682 * assign91050_e139682))), (-((((locals.var_tmf1_dn9 + locals.var_tmf2_dn9) + locals.var_tmf3_dn9) + locals.var_tmf4_dn9) / (assign91050_e139682 * assign91050_e139682))), (-((((locals.var_tmf1_dn10 + locals.var_tmf2_dn10) + locals.var_tmf3_dn10) + locals.var_tmf4_dn10) / (assign91050_e139682 * assign91050_e139682))), (-((((locals.var_tmf1_dn11 + locals.var_tmf2_dn11) + locals.var_tmf3_dn11) + locals.var_tmf4_dn11) / (assign91050_e139682 * assign91050_e139682))), (-((((locals.var_tmf1_dn14 + locals.var_tmf2_dn14) + locals.var_tmf3_dn14) + locals.var_tmf4_dn14) / (assign91050_e139682 * assign91050_e139682))),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign91050_e139685;
        locals.var_tmf0_dn0 = assign91050_e139685_d_n0;
        locals.var_tmf0_dn2 = assign91050_e139685_d_n2;
        locals.var_tmf0_dn4 = assign91050_e139685_d_n4;
        locals.var_tmf0_dn5 = assign91050_e139685_d_n5;
        locals.var_tmf0_dn6 = assign91050_e139685_d_n6;
        locals.var_tmf0_dn7 = assign91050_e139685_d_n7;
        locals.var_tmf0_dn8 = assign91050_e139685_d_n8;
        locals.var_tmf0_dn9 = assign91050_e139685_d_n9;
        locals.var_tmf0_dn10 = assign91050_e139685_d_n10;
        locals.var_tmf0_dn11 = assign91050_e139685_d_n11;
        locals.var_tmf0_dn14 = assign91050_e139685_d_n14;
        locals.var_tmf0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_351(
        locals: &mut StampLocals,
    ) {
        let (assign91060_e139712, assign91060_e139712_d_n0, assign91060_e139712_d_n2, assign91060_e139712_d_n4, assign91060_e139712_d_n5, assign91060_e139712_d_n6, assign91060_e139712_d_n7, assign91060_e139712_d_n8, assign91060_e139712_d_n9, assign91060_e139712_d_n10, assign91060_e139712_d_n11, assign91060_e139712_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2132 != 0.0)) && (locals.var_guard2133 != 0.0)) {
        let assign91060_e139696: f64 = (2.0 * locals.var_tmf1);
        let assign91060_e139697: f64 = (1.0 + assign91060_e139696);
        let assign91060_e139700: f64 = (3.0 * locals.var_tmf2);
        let assign91060_e139701: f64 = (assign91060_e139697 + assign91060_e139700);
        let assign91060_e139704: f64 = (4.0 * locals.var_tmf3);
        let assign91060_e139705: f64 = (assign91060_e139701 + assign91060_e139704);
        let assign91060_e139706: f64 = (-assign91060_e139705);
        let assign91060_e139708: f64 = (assign91060_e139706 * locals.var_tmf0);
        let assign91060_e139710: f64 = (assign91060_e139708 * locals.var_tmf0);
        (assign91060_e139710, (((((-(((2.0 * locals.var_tmf1_dn0) + (3.0 * locals.var_tmf2_dn0)) + (4.0 * locals.var_tmf3_dn0))) * locals.var_tmf0) + (assign91060_e139706 * locals.var_tmf0_dn0)) * locals.var_tmf0) + (assign91060_e139708 * locals.var_tmf0_dn0)), (((((-(((2.0 * locals.var_tmf1_dn2) + (3.0 * locals.var_tmf2_dn2)) + (4.0 * locals.var_tmf3_dn2))) * locals.var_tmf0) + (assign91060_e139706 * locals.var_tmf0_dn2)) * locals.var_tmf0) + (assign91060_e139708 * locals.var_tmf0_dn2)), (((((-(((2.0 * locals.var_tmf1_dn4) + (3.0 * locals.var_tmf2_dn4)) + (4.0 * locals.var_tmf3_dn4))) * locals.var_tmf0) + (assign91060_e139706 * locals.var_tmf0_dn4)) * locals.var_tmf0) + (assign91060_e139708 * locals.var_tmf0_dn4)), (((((-(((2.0 * locals.var_tmf1_dn5) + (3.0 * locals.var_tmf2_dn5)) + (4.0 * locals.var_tmf3_dn5))) * locals.var_tmf0) + (assign91060_e139706 * locals.var_tmf0_dn5)) * locals.var_tmf0) + (assign91060_e139708 * locals.var_tmf0_dn5)), (((((-(((2.0 * locals.var_tmf1_dn6) + (3.0 * locals.var_tmf2_dn6)) + (4.0 * locals.var_tmf3_dn6))) * locals.var_tmf0) + (assign91060_e139706 * locals.var_tmf0_dn6)) * locals.var_tmf0) + (assign91060_e139708 * locals.var_tmf0_dn6)), (((((-(((2.0 * locals.var_tmf1_dn7) + (3.0 * locals.var_tmf2_dn7)) + (4.0 * locals.var_tmf3_dn7))) * locals.var_tmf0) + (assign91060_e139706 * locals.var_tmf0_dn7)) * locals.var_tmf0) + (assign91060_e139708 * locals.var_tmf0_dn7)), (((((-(((2.0 * locals.var_tmf1_dn8) + (3.0 * locals.var_tmf2_dn8)) + (4.0 * locals.var_tmf3_dn8))) * locals.var_tmf0) + (assign91060_e139706 * locals.var_tmf0_dn8)) * locals.var_tmf0) + (assign91060_e139708 * locals.var_tmf0_dn8)), (((((-(((2.0 * locals.var_tmf1_dn9) + (3.0 * locals.var_tmf2_dn9)) + (4.0 * locals.var_tmf3_dn9))) * locals.var_tmf0) + (assign91060_e139706 * locals.var_tmf0_dn9)) * locals.var_tmf0) + (assign91060_e139708 * locals.var_tmf0_dn9)), (((((-(((2.0 * locals.var_tmf1_dn10) + (3.0 * locals.var_tmf2_dn10)) + (4.0 * locals.var_tmf3_dn10))) * locals.var_tmf0) + (assign91060_e139706 * locals.var_tmf0_dn10)) * locals.var_tmf0) + (assign91060_e139708 * locals.var_tmf0_dn10)), (((((-(((2.0 * locals.var_tmf1_dn11) + (3.0 * locals.var_tmf2_dn11)) + (4.0 * locals.var_tmf3_dn11))) * locals.var_tmf0) + (assign91060_e139706 * locals.var_tmf0_dn11)) * locals.var_tmf0) + (assign91060_e139708 * locals.var_tmf0_dn11)), (((((-(((2.0 * locals.var_tmf1_dn14) + (3.0 * locals.var_tmf2_dn14)) + (4.0 * locals.var_tmf3_dn14))) * locals.var_tmf0) + (assign91060_e139706 * locals.var_tmf0_dn14)) * locals.var_tmf0) + (assign91060_e139708 * locals.var_tmf0_dn14)),)
    } else {
        (locals.var_t11, locals.var_t11_dn0, locals.var_t11_dn2, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11, locals.var_t11_dn14,)
    }
};
        locals.var_t11 = assign91060_e139712;
        locals.var_t11_dn0 = assign91060_e139712_d_n0;
        locals.var_t11_dn2 = assign91060_e139712_d_n2;
        locals.var_t11_dn4 = assign91060_e139712_d_n4;
        locals.var_t11_dn5 = assign91060_e139712_d_n5;
        locals.var_t11_dn6 = assign91060_e139712_d_n6;
        locals.var_t11_dn7 = assign91060_e139712_d_n7;
        locals.var_t11_dn8 = assign91060_e139712_d_n8;
        locals.var_t11_dn9 = assign91060_e139712_d_n9;
        locals.var_t11_dn10 = assign91060_e139712_d_n10;
        locals.var_t11_dn11 = assign91060_e139712_d_n11;
        locals.var_t11_dn14 = assign91060_e139712_d_n14;
        locals.var_t11_rv = 0.0;

        let (assign91070_e139726, assign91070_e139726_d_n0, assign91070_e139726_d_n2, assign91070_e139726_d_n4, assign91070_e139726_d_n5, assign91070_e139726_d_n6, assign91070_e139726_d_n7, assign91070_e139726_d_n8, assign91070_e139726_d_n9, assign91070_e139726_d_n10, assign91070_e139726_d_n11, assign91070_e139726_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2132 != 0.0)) && (locals.var_guard2133 != 0.0)) {
        let assign91070_e139723: f64 = (1.0 - locals.var_tmf0);
        let assign91070_e139724: f64 = (locals.var_t2 * assign91070_e139723);
        (assign91070_e139724, ((locals.var_t2_dn0 * assign91070_e139723) + (locals.var_t2 * (-locals.var_tmf0_dn0))), ((locals.var_t2_dn2 * assign91070_e139723) + (locals.var_t2 * (-locals.var_tmf0_dn2))), ((locals.var_t2_dn4 * assign91070_e139723) + (locals.var_t2 * (-locals.var_tmf0_dn4))), ((locals.var_t2_dn5 * assign91070_e139723) + (locals.var_t2 * (-locals.var_tmf0_dn5))), ((locals.var_t2_dn6 * assign91070_e139723) + (locals.var_t2 * (-locals.var_tmf0_dn6))), ((locals.var_t2_dn7 * assign91070_e139723) + (locals.var_t2 * (-locals.var_tmf0_dn7))), ((locals.var_t2_dn8 * assign91070_e139723) + (locals.var_t2 * (-locals.var_tmf0_dn8))), ((locals.var_t2_dn9 * assign91070_e139723) + (locals.var_t2 * (-locals.var_tmf0_dn9))), ((locals.var_t2_dn10 * assign91070_e139723) + (locals.var_t2 * (-locals.var_tmf0_dn10))), ((locals.var_t2_dn11 * assign91070_e139723) + (locals.var_t2 * (-locals.var_tmf0_dn11))), ((locals.var_t2_dn14 * assign91070_e139723) + (locals.var_t2 * (-locals.var_tmf0_dn14))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn14,)
    }
};
        locals.var_ty = assign91070_e139726;
        locals.var_ty_dn0 = assign91070_e139726_d_n0;
        locals.var_ty_dn2 = assign91070_e139726_d_n2;
        locals.var_ty_dn4 = assign91070_e139726_d_n4;
        locals.var_ty_dn5 = assign91070_e139726_d_n5;
        locals.var_ty_dn6 = assign91070_e139726_d_n6;
        locals.var_ty_dn7 = assign91070_e139726_d_n7;
        locals.var_ty_dn8 = assign91070_e139726_d_n8;
        locals.var_ty_dn9 = assign91070_e139726_d_n9;
        locals.var_ty_dn10 = assign91070_e139726_d_n10;
        locals.var_ty_dn11 = assign91070_e139726_d_n11;
        locals.var_ty_dn14 = assign91070_e139726_d_n14;
        locals.var_ty_rv = 0.0;

        let (assign91080_e139742, assign91080_e139742_d_n0, assign91080_e139742_d_n2, assign91080_e139742_d_n4, assign91080_e139742_d_n5, assign91080_e139742_d_n6, assign91080_e139742_d_n7, assign91080_e139742_d_n8, assign91080_e139742_d_n9, assign91080_e139742_d_n10, assign91080_e139742_d_n11, assign91080_e139742_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2132 != 0.0)) && (locals.var_guard2133 != 0.0)) {
        let assign91080_e139736: f64 = (1.0 - locals.var_tmf0);
        let assign91080_e139739: f64 = (locals.var_tmf1 * locals.var_t11);
        let assign91080_e139740: f64 = (assign91080_e139736 + assign91080_e139739);
        (assign91080_e139740, ((-locals.var_tmf0_dn0) + ((locals.var_tmf1_dn0 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn0))), ((-locals.var_tmf0_dn2) + ((locals.var_tmf1_dn2 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn2))), ((-locals.var_tmf0_dn4) + ((locals.var_tmf1_dn4 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn4))), ((-locals.var_tmf0_dn5) + ((locals.var_tmf1_dn5 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn5))), ((-locals.var_tmf0_dn6) + ((locals.var_tmf1_dn6 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn6))), ((-locals.var_tmf0_dn7) + ((locals.var_tmf1_dn7 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn7))), ((-locals.var_tmf0_dn8) + ((locals.var_tmf1_dn8 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn8))), ((-locals.var_tmf0_dn9) + ((locals.var_tmf1_dn9 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn9))), ((-locals.var_tmf0_dn10) + ((locals.var_tmf1_dn10 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn10))), ((-locals.var_tmf0_dn11) + ((locals.var_tmf1_dn11 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn11))), ((-locals.var_tmf0_dn14) + ((locals.var_tmf1_dn14 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn14))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign91080_e139742;
        locals.var_t0_dn0 = assign91080_e139742_d_n0;
        locals.var_t0_dn2 = assign91080_e139742_d_n2;
        locals.var_t0_dn4 = assign91080_e139742_d_n4;
        locals.var_t0_dn5 = assign91080_e139742_d_n5;
        locals.var_t0_dn6 = assign91080_e139742_d_n6;
        locals.var_t0_dn7 = assign91080_e139742_d_n7;
        locals.var_t0_dn8 = assign91080_e139742_d_n8;
        locals.var_t0_dn9 = assign91080_e139742_d_n9;
        locals.var_t0_dn10 = assign91080_e139742_d_n10;
        locals.var_t0_dn11 = assign91080_e139742_d_n11;
        locals.var_t0_dn14 = assign91080_e139742_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign91090_e139753, assign91090_e139753_d_n0, assign91090_e139753_d_n2, assign91090_e139753_d_n4, assign91090_e139753_d_n5, assign91090_e139753_d_n6, assign91090_e139753_d_n7, assign91090_e139753_d_n8, assign91090_e139753_d_n9, assign91090_e139753_d_n10, assign91090_e139753_d_n11, assign91090_e139753_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2132 != 0.0)) && (locals.var_guard2133 != 0.0)) {
        let assign91090_e139751: f64 = (-locals.var_t11);
        (assign91090_e139751, (-locals.var_t11_dn0), (-locals.var_t11_dn2), (-locals.var_t11_dn4), (-locals.var_t11_dn5), (-locals.var_t11_dn6), (-locals.var_t11_dn7), (-locals.var_t11_dn8), (-locals.var_t11_dn9), (-locals.var_t11_dn10), (-locals.var_t11_dn11), (-locals.var_t11_dn14),)
    } else {
        (locals.var_t11, locals.var_t11_dn0, locals.var_t11_dn2, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11, locals.var_t11_dn14,)
    }
};
        locals.var_t11 = assign91090_e139753;
        locals.var_t11_dn0 = assign91090_e139753_d_n0;
        locals.var_t11_dn2 = assign91090_e139753_d_n2;
        locals.var_t11_dn4 = assign91090_e139753_d_n4;
        locals.var_t11_dn5 = assign91090_e139753_d_n5;
        locals.var_t11_dn6 = assign91090_e139753_d_n6;
        locals.var_t11_dn7 = assign91090_e139753_d_n7;
        locals.var_t11_dn8 = assign91090_e139753_d_n8;
        locals.var_t11_dn9 = assign91090_e139753_d_n9;
        locals.var_t11_dn10 = assign91090_e139753_d_n10;
        locals.var_t11_dn11 = assign91090_e139753_d_n11;
        locals.var_t11_dn14 = assign91090_e139753_d_n14;
        locals.var_t11_rv = 0.0;

        let (assign91100_e139765, assign91100_e139765_d_n0, assign91100_e139765_d_n2, assign91100_e139765_d_n4, assign91100_e139765_d_n5, assign91100_e139765_d_n6, assign91100_e139765_d_n7, assign91100_e139765_d_n8, assign91100_e139765_d_n9, assign91100_e139765_d_n10, assign91100_e139765_d_n11, assign91100_e139765_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2132 != 0.0)) && (locals.var_guard2133 != 0.0)) {
        let assign91100_e139763: f64 = (locals.var_vbs_bnd_over__blk2122 + locals.var_ty);
        (assign91100_e139763, (locals.var_vbs_bnd_over__blk2122_dn0 + locals.var_ty_dn0), (locals.var_vbs_bnd_over__blk2122_dn2 + locals.var_ty_dn2), (locals.var_vbs_bnd_over__blk2122_dn4 + locals.var_ty_dn4), (locals.var_vbs_bnd_over__blk2122_dn5 + locals.var_ty_dn5), (locals.var_vbs_bnd_over__blk2122_dn6 + locals.var_ty_dn6), (locals.var_vbs_bnd_over__blk2122_dn7 + locals.var_ty_dn7), (locals.var_vbs_bnd_over__blk2122_dn8 + locals.var_ty_dn8), (locals.var_vbs_bnd_over__blk2122_dn9 + locals.var_ty_dn9), (locals.var_vbs_bnd_over__blk2122_dn10 + locals.var_ty_dn10), (locals.var_vbs_bnd_over__blk2122_dn11 + locals.var_ty_dn11), (locals.var_vbs_bnd_over__blk2122_dn14 + locals.var_ty_dn14),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign91100_e139765;
        locals.var_t10_dn0 = assign91100_e139765_d_n0;
        locals.var_t10_dn2 = assign91100_e139765_d_n2;
        locals.var_t10_dn4 = assign91100_e139765_d_n4;
        locals.var_t10_dn5 = assign91100_e139765_d_n5;
        locals.var_t10_dn6 = assign91100_e139765_d_n6;
        locals.var_t10_dn7 = assign91100_e139765_d_n7;
        locals.var_t10_dn8 = assign91100_e139765_d_n8;
        locals.var_t10_dn9 = assign91100_e139765_d_n9;
        locals.var_t10_dn10 = assign91100_e139765_d_n10;
        locals.var_t10_dn11 = assign91100_e139765_d_n11;
        locals.var_t10_dn14 = assign91100_e139765_d_n14;
        locals.var_t10_rv = 0.0;

        let (assign91110_e139776, assign91110_e139776_d_n0, assign91110_e139776_d_n2, assign91110_e139776_d_n4, assign91110_e139776_d_n5, assign91110_e139776_d_n6, assign91110_e139776_d_n7, assign91110_e139776_d_n8, assign91110_e139776_d_n9, assign91110_e139776_d_n10, assign91110_e139776_d_n11, assign91110_e139776_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2132 != 0.0)) && (locals.var_guard2133 == 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign91110_e139776;
        locals.var_t10_dn0 = assign91110_e139776_d_n0;
        locals.var_t10_dn2 = assign91110_e139776_d_n2;
        locals.var_t10_dn4 = assign91110_e139776_d_n4;
        locals.var_t10_dn5 = assign91110_e139776_d_n5;
        locals.var_t10_dn6 = assign91110_e139776_d_n6;
        locals.var_t10_dn7 = assign91110_e139776_d_n7;
        locals.var_t10_dn8 = assign91110_e139776_d_n8;
        locals.var_t10_dn9 = assign91110_e139776_d_n9;
        locals.var_t10_dn10 = assign91110_e139776_d_n10;
        locals.var_t10_dn11 = assign91110_e139776_d_n11;
        locals.var_t10_dn14 = assign91110_e139776_d_n14;
        locals.var_t10_rv = 0.0;

        let (assign91120_e139785, assign91120_e139785_d_n0, assign91120_e139785_d_n2, assign91120_e139785_d_n4, assign91120_e139785_d_n5, assign91120_e139785_d_n6, assign91120_e139785_d_n7, assign91120_e139785_d_n8, assign91120_e139785_d_n9, assign91120_e139785_d_n10, assign91120_e139785_d_n11, assign91120_e139785_d_n14,) = {
    if (((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2132 != 0.0)) {
        let assign91120_e139783: f64 = (-locals.var_t10);
        (assign91120_e139783, (-locals.var_t10_dn0), (-locals.var_t10_dn2), (-locals.var_t10_dn4), (-locals.var_t10_dn5), (-locals.var_t10_dn6), (-locals.var_t10_dn7), (-locals.var_t10_dn8), (-locals.var_t10_dn9), (-locals.var_t10_dn10), (-locals.var_t10_dn11), (-locals.var_t10_dn14),)
    } else {
        (locals.var_vxbgmtcl, locals.var_vxbgmtcl_dn0, locals.var_vxbgmtcl_dn2, locals.var_vxbgmtcl_dn4, locals.var_vxbgmtcl_dn5, locals.var_vxbgmtcl_dn6, locals.var_vxbgmtcl_dn7, locals.var_vxbgmtcl_dn8, locals.var_vxbgmtcl_dn9, locals.var_vxbgmtcl_dn10, locals.var_vxbgmtcl_dn11, locals.var_vxbgmtcl_dn14,)
    }
};
        locals.var_vxbgmtcl = assign91120_e139785;
        locals.var_vxbgmtcl_dn0 = assign91120_e139785_d_n0;
        locals.var_vxbgmtcl_dn2 = assign91120_e139785_d_n2;
        locals.var_vxbgmtcl_dn4 = assign91120_e139785_d_n4;
        locals.var_vxbgmtcl_dn5 = assign91120_e139785_d_n5;
        locals.var_vxbgmtcl_dn6 = assign91120_e139785_d_n6;
        locals.var_vxbgmtcl_dn7 = assign91120_e139785_d_n7;
        locals.var_vxbgmtcl_dn8 = assign91120_e139785_d_n8;
        locals.var_vxbgmtcl_dn9 = assign91120_e139785_d_n9;
        locals.var_vxbgmtcl_dn10 = assign91120_e139785_d_n10;
        locals.var_vxbgmtcl_dn11 = assign91120_e139785_d_n11;
        locals.var_vxbgmtcl_dn14 = assign91120_e139785_d_n14;
        locals.var_vxbgmtcl_rv = 0.0;

        let (assign91130_e139794, assign91130_e139794_d_n0, assign91130_e139794_d_n2, assign91130_e139794_d_n4, assign91130_e139794_d_n5, assign91130_e139794_d_n6, assign91130_e139794_d_n7, assign91130_e139794_d_n8, assign91130_e139794_d_n9, assign91130_e139794_d_n10, assign91130_e139794_d_n11, assign91130_e139794_d_n14,) = {
    if (((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2132 == 0.0)) {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn11, locals.var_vxbgmt_dn14,)
    } else {
        (locals.var_vxbgmtcl, locals.var_vxbgmtcl_dn0, locals.var_vxbgmtcl_dn2, locals.var_vxbgmtcl_dn4, locals.var_vxbgmtcl_dn5, locals.var_vxbgmtcl_dn6, locals.var_vxbgmtcl_dn7, locals.var_vxbgmtcl_dn8, locals.var_vxbgmtcl_dn9, locals.var_vxbgmtcl_dn10, locals.var_vxbgmtcl_dn11, locals.var_vxbgmtcl_dn14,)
    }
};
        locals.var_vxbgmtcl = assign91130_e139794;
        locals.var_vxbgmtcl_dn0 = assign91130_e139794_d_n0;
        locals.var_vxbgmtcl_dn2 = assign91130_e139794_d_n2;
        locals.var_vxbgmtcl_dn4 = assign91130_e139794_d_n4;
        locals.var_vxbgmtcl_dn5 = assign91130_e139794_d_n5;
        locals.var_vxbgmtcl_dn6 = assign91130_e139794_d_n6;
        locals.var_vxbgmtcl_dn7 = assign91130_e139794_d_n7;
        locals.var_vxbgmtcl_dn8 = assign91130_e139794_d_n8;
        locals.var_vxbgmtcl_dn9 = assign91130_e139794_d_n9;
        locals.var_vxbgmtcl_dn10 = assign91130_e139794_d_n10;
        locals.var_vxbgmtcl_dn11 = assign91130_e139794_d_n11;
        locals.var_vxbgmtcl_dn14 = assign91130_e139794_d_n14;
        locals.var_vxbgmtcl_rv = 0.0;

        let (assign91140_e139802, assign91140_e139802_d_n0, assign91140_e139802_d_n2, assign91140_e139802_d_n4, assign91140_e139802_d_n5, assign91140_e139802_d_n6, assign91140_e139802_d_n7, assign91140_e139802_d_n8, assign91140_e139802_d_n9, assign91140_e139802_d_n10, assign91140_e139802_d_n11, assign91140_e139802_d_n14,) = {
    if ((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) {
        let assign91140_e139800: f64 = (locals.var_cnst0over_func / locals.var_cox0_func);
        (assign91140_e139800, (locals.var_cnst0over_func_dn0 / locals.var_cox0_func), (locals.var_cnst0over_func_dn2 / locals.var_cox0_func), (locals.var_cnst0over_func_dn4 / locals.var_cox0_func), (locals.var_cnst0over_func_dn5 / locals.var_cox0_func), (locals.var_cnst0over_func_dn6 / locals.var_cox0_func), (locals.var_cnst0over_func_dn7 / locals.var_cox0_func), (locals.var_cnst0over_func_dn8 / locals.var_cox0_func), (locals.var_cnst0over_func_dn9 / locals.var_cox0_func), (locals.var_cnst0over_func_dn10 / locals.var_cox0_func), (locals.var_cnst0over_func_dn11 / locals.var_cox0_func), (locals.var_cnst0over_func_dn14 / locals.var_cox0_func),)
    } else {
        (locals.var_fac1, locals.var_fac1_dn0, locals.var_fac1_dn2, locals.var_fac1_dn4, locals.var_fac1_dn5, locals.var_fac1_dn6, locals.var_fac1_dn7, locals.var_fac1_dn8, locals.var_fac1_dn9, locals.var_fac1_dn10, locals.var_fac1_dn11, locals.var_fac1_dn14,)
    }
};
        locals.var_fac1 = assign91140_e139802;
        locals.var_fac1_dn0 = assign91140_e139802_d_n0;
        locals.var_fac1_dn2 = assign91140_e139802_d_n2;
        locals.var_fac1_dn4 = assign91140_e139802_d_n4;
        locals.var_fac1_dn5 = assign91140_e139802_d_n5;
        locals.var_fac1_dn6 = assign91140_e139802_d_n6;
        locals.var_fac1_dn7 = assign91140_e139802_d_n7;
        locals.var_fac1_dn8 = assign91140_e139802_d_n8;
        locals.var_fac1_dn9 = assign91140_e139802_d_n9;
        locals.var_fac1_dn10 = assign91140_e139802_d_n10;
        locals.var_fac1_dn11 = assign91140_e139802_d_n11;
        locals.var_fac1_dn14 = assign91140_e139802_d_n14;
        locals.var_fac1_rv = 0.0;

        let (assign91150_e139810, assign91150_e139810_d_n0, assign91150_e139810_d_n2, assign91150_e139810_d_n4, assign91150_e139810_d_n5, assign91150_e139810_d_n6, assign91150_e139810_d_n7, assign91150_e139810_d_n8, assign91150_e139810_d_n9, assign91150_e139810_d_n10, assign91150_e139810_d_n11, assign91150_e139810_d_n14,) = {
    if ((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) {
        let assign91150_e139808: f64 = (locals.var_fac1 * locals.var_fac1);
        (assign91150_e139808, ((locals.var_fac1_dn0 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn0)), ((locals.var_fac1_dn2 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn2)), ((locals.var_fac1_dn4 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn4)), ((locals.var_fac1_dn5 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn5)), ((locals.var_fac1_dn6 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn6)), ((locals.var_fac1_dn7 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn7)), ((locals.var_fac1_dn8 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn8)), ((locals.var_fac1_dn9 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn9)), ((locals.var_fac1_dn10 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn10)), ((locals.var_fac1_dn11 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn11)), ((locals.var_fac1_dn14 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn14)),)
    } else {
        (locals.var_fac1p2, locals.var_fac1p2_dn0, locals.var_fac1p2_dn2, locals.var_fac1p2_dn4, locals.var_fac1p2_dn5, locals.var_fac1p2_dn6, locals.var_fac1p2_dn7, locals.var_fac1p2_dn8, locals.var_fac1p2_dn9, locals.var_fac1p2_dn10, locals.var_fac1p2_dn11, locals.var_fac1p2_dn14,)
    }
};
        locals.var_fac1p2 = assign91150_e139810;
        locals.var_fac1p2_dn0 = assign91150_e139810_d_n0;
        locals.var_fac1p2_dn2 = assign91150_e139810_d_n2;
        locals.var_fac1p2_dn4 = assign91150_e139810_d_n4;
        locals.var_fac1p2_dn5 = assign91150_e139810_d_n5;
        locals.var_fac1p2_dn6 = assign91150_e139810_d_n6;
        locals.var_fac1p2_dn7 = assign91150_e139810_d_n7;
        locals.var_fac1p2_dn8 = assign91150_e139810_d_n8;
        locals.var_fac1p2_dn9 = assign91150_e139810_d_n9;
        locals.var_fac1p2_dn10 = assign91150_e139810_d_n10;
        locals.var_fac1p2_dn11 = assign91150_e139810_d_n11;
        locals.var_fac1p2_dn14 = assign91150_e139810_d_n14;
        locals.var_fac1p2_rv = 0.0;

        let (assign91160_e139819, assign91160_e139819_d_n2, assign91160_e139819_d_n7, assign91160_e139819_d_n8, assign91160_e139819_d_n9,) = {
    if ((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) {
        let assign91160_e139815: f64 = (-locals.var_vgbgmt);
        let assign91160_e139817: f64 = (assign91160_e139815 + locals.var_uc_vfbover);
        (assign91160_e139817, (-locals.var_vgbgmt_dn2), (-locals.var_vgbgmt_dn7), (-locals.var_vgbgmt_dn8), (-locals.var_vgbgmt_dn9),)
    } else {
        (locals.var_vgpld, locals.var_vgpld_dn2, locals.var_vgpld_dn7, locals.var_vgpld_dn8, locals.var_vgpld_dn9,)
    }
};
        locals.var_vgpld = assign91160_e139819;
        locals.var_vgpld_dn2 = assign91160_e139819_d_n2;
        locals.var_vgpld_dn7 = assign91160_e139819_d_n7;
        locals.var_vgpld_dn8 = assign91160_e139819_d_n8;
        locals.var_vgpld_dn9 = assign91160_e139819_d_n9;
        locals.var_vgpld_rv = 0.0;

        let (assign91170_e139830, assign91170_e139830_d_n0, assign91170_e139830_d_n2, assign91170_e139830_d_n4, assign91170_e139830_d_n5, assign91170_e139830_d_n6, assign91170_e139830_d_n7, assign91170_e139830_d_n8, assign91170_e139830_d_n9, assign91170_e139830_d_n10, assign91170_e139830_d_n11, assign91170_e139830_d_n14,) = {
    if ((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) {
        let assign91170_e139824: f64 = (-locals.var_vxbgmtcl);
        let assign91170_e139827: f64 = (10.0 * 2.220446049250313e-16);
        let assign91170_e139828: f64 = (assign91170_e139824 + assign91170_e139827);
        (assign91170_e139828, (-locals.var_vxbgmtcl_dn0), (-locals.var_vxbgmtcl_dn2), (-locals.var_vxbgmtcl_dn4), (-locals.var_vxbgmtcl_dn5), (-locals.var_vxbgmtcl_dn6), (-locals.var_vxbgmtcl_dn7), (-locals.var_vxbgmtcl_dn8), (-locals.var_vxbgmtcl_dn9), (-locals.var_vxbgmtcl_dn10), (-locals.var_vxbgmtcl_dn11), (-locals.var_vxbgmtcl_dn14),)
    } else {
        (locals.var_vgb_fb_ld, locals.var_vgb_fb_ld_dn0, locals.var_vgb_fb_ld_dn2, locals.var_vgb_fb_ld_dn4, locals.var_vgb_fb_ld_dn5, locals.var_vgb_fb_ld_dn6, locals.var_vgb_fb_ld_dn7, locals.var_vgb_fb_ld_dn8, locals.var_vgb_fb_ld_dn9, locals.var_vgb_fb_ld_dn10, locals.var_vgb_fb_ld_dn11, locals.var_vgb_fb_ld_dn14,)
    }
};
        locals.var_vgb_fb_ld = assign91170_e139830;
        locals.var_vgb_fb_ld_dn0 = assign91170_e139830_d_n0;
        locals.var_vgb_fb_ld_dn2 = assign91170_e139830_d_n2;
        locals.var_vgb_fb_ld_dn4 = assign91170_e139830_d_n4;
        locals.var_vgb_fb_ld_dn5 = assign91170_e139830_d_n5;
        locals.var_vgb_fb_ld_dn6 = assign91170_e139830_d_n6;
        locals.var_vgb_fb_ld_dn7 = assign91170_e139830_d_n7;
        locals.var_vgb_fb_ld_dn8 = assign91170_e139830_d_n8;
        locals.var_vgb_fb_ld_dn9 = assign91170_e139830_d_n9;
        locals.var_vgb_fb_ld_dn10 = assign91170_e139830_d_n10;
        locals.var_vgb_fb_ld_dn11 = assign91170_e139830_d_n11;
        locals.var_vgb_fb_ld_dn14 = assign91170_e139830_d_n14;
        locals.var_vgb_fb_ld_rv = 0.0;

        let (assign91180_e139836, assign91180_e139836_d_n0, assign91180_e139836_d_n2, assign91180_e139836_d_n4, assign91180_e139836_d_n5, assign91180_e139836_d_n6, assign91180_e139836_d_n7, assign91180_e139836_d_n8, assign91180_e139836_d_n9, assign91180_e139836_d_n10, assign91180_e139836_d_n11, assign91180_e139836_d_n14,) = {
    if ((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_dep_ld__blk2116, locals.var_q_dep_ld__blk2116_dn0, locals.var_q_dep_ld__blk2116_dn2, locals.var_q_dep_ld__blk2116_dn4, locals.var_q_dep_ld__blk2116_dn5, locals.var_q_dep_ld__blk2116_dn6, locals.var_q_dep_ld__blk2116_dn7, locals.var_q_dep_ld__blk2116_dn8, locals.var_q_dep_ld__blk2116_dn9, locals.var_q_dep_ld__blk2116_dn10, locals.var_q_dep_ld__blk2116_dn11, locals.var_q_dep_ld__blk2116_dn14,)
    }
};
        locals.var_q_dep_ld__blk2116 = assign91180_e139836;
        locals.var_q_dep_ld__blk2116_dn0 = assign91180_e139836_d_n0;
        locals.var_q_dep_ld__blk2116_dn2 = assign91180_e139836_d_n2;
        locals.var_q_dep_ld__blk2116_dn4 = assign91180_e139836_d_n4;
        locals.var_q_dep_ld__blk2116_dn5 = assign91180_e139836_d_n5;
        locals.var_q_dep_ld__blk2116_dn6 = assign91180_e139836_d_n6;
        locals.var_q_dep_ld__blk2116_dn7 = assign91180_e139836_d_n7;
        locals.var_q_dep_ld__blk2116_dn8 = assign91180_e139836_d_n8;
        locals.var_q_dep_ld__blk2116_dn9 = assign91180_e139836_d_n9;
        locals.var_q_dep_ld__blk2116_dn10 = assign91180_e139836_d_n10;
        locals.var_q_dep_ld__blk2116_dn11 = assign91180_e139836_d_n11;
        locals.var_q_dep_ld__blk2116_dn14 = assign91180_e139836_d_n14;
        locals.var_q_dep_ld__blk2116_rv = 0.0;

        let (assign91190_e139844,) = {
    if ((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) {
        let assign91190_e139842: f64 = (1.6021918e-19 * locals.var_nover_func);
        (assign91190_e139842,)
    } else {
        (locals.var_q_nsubld__blk2117,)
    }
};
        locals.var_q_nsubld__blk2117 = assign91190_e139844;
        locals.var_q_nsubld__blk2117_rv = 0.0;

        let (assign91200_e139852, assign91200_e139852_d_n0, assign91200_e139852_d_n2, assign91200_e139852_d_n4, assign91200_e139852_d_n5, assign91200_e139852_d_n6, assign91200_e139852_d_n7, assign91200_e139852_d_n8, assign91200_e139852_d_n9, assign91200_e139852_d_n10, assign91200_e139852_d_n11, assign91200_e139852_d_n14,) = {
    if ((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) {
        let assign91200_e139850: f64 = (locals.var_nin / locals.var_nover_func);
        (assign91200_e139850, (locals.var_nin_dn0 / locals.var_nover_func), (locals.var_nin_dn2 / locals.var_nover_func), (locals.var_nin_dn4 / locals.var_nover_func), (locals.var_nin_dn5 / locals.var_nover_func), (locals.var_nin_dn6 / locals.var_nover_func), (locals.var_nin_dn7 / locals.var_nover_func), (locals.var_nin_dn8 / locals.var_nover_func), (locals.var_nin_dn9 / locals.var_nover_func), (locals.var_nin_dn10 / locals.var_nover_func), (locals.var_nin_dn11 / locals.var_nover_func), (locals.var_nin_dn14 / locals.var_nover_func),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign91200_e139852;
        locals.var_t0_dn0 = assign91200_e139852_d_n0;
        locals.var_t0_dn2 = assign91200_e139852_d_n2;
        locals.var_t0_dn4 = assign91200_e139852_d_n4;
        locals.var_t0_dn5 = assign91200_e139852_d_n5;
        locals.var_t0_dn6 = assign91200_e139852_d_n6;
        locals.var_t0_dn7 = assign91200_e139852_d_n7;
        locals.var_t0_dn8 = assign91200_e139852_d_n8;
        locals.var_t0_dn9 = assign91200_e139852_d_n9;
        locals.var_t0_dn10 = assign91200_e139852_d_n10;
        locals.var_t0_dn11 = assign91200_e139852_d_n11;
        locals.var_t0_dn14 = assign91200_e139852_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign91210_e139860, assign91210_e139860_d_n0, assign91210_e139860_d_n2, assign91210_e139860_d_n4, assign91210_e139860_d_n5, assign91210_e139860_d_n6, assign91210_e139860_d_n7, assign91210_e139860_d_n8, assign91210_e139860_d_n9, assign91210_e139860_d_n10, assign91210_e139860_d_n11, assign91210_e139860_d_n14,) = {
    if ((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) {
        let assign91210_e139858: f64 = (locals.var_t0 * locals.var_t0);
        (assign91210_e139858, ((locals.var_t0_dn0 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn0)), ((locals.var_t0_dn2 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn2)), ((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)), ((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)), ((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)), ((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)), ((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)), ((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)), ((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)), ((locals.var_t0_dn11 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn11)), ((locals.var_t0_dn14 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn14)),)
    } else {
        (locals.var_cnst1over, locals.var_cnst1over_dn0, locals.var_cnst1over_dn2, locals.var_cnst1over_dn4, locals.var_cnst1over_dn5, locals.var_cnst1over_dn6, locals.var_cnst1over_dn7, locals.var_cnst1over_dn8, locals.var_cnst1over_dn9, locals.var_cnst1over_dn10, locals.var_cnst1over_dn11, locals.var_cnst1over_dn14,)
    }
};
        locals.var_cnst1over = assign91210_e139860;
        locals.var_cnst1over_dn0 = assign91210_e139860_d_n0;
        locals.var_cnst1over_dn2 = assign91210_e139860_d_n2;
        locals.var_cnst1over_dn4 = assign91210_e139860_d_n4;
        locals.var_cnst1over_dn5 = assign91210_e139860_d_n5;
        locals.var_cnst1over_dn6 = assign91210_e139860_d_n6;
        locals.var_cnst1over_dn7 = assign91210_e139860_d_n7;
        locals.var_cnst1over_dn8 = assign91210_e139860_d_n8;
        locals.var_cnst1over_dn9 = assign91210_e139860_d_n9;
        locals.var_cnst1over_dn10 = assign91210_e139860_d_n10;
        locals.var_cnst1over_dn11 = assign91210_e139860_d_n11;
        locals.var_cnst1over_dn14 = assign91210_e139860_d_n14;
        locals.var_cnst1over_rv = 0.0;

        let assign91220_e139863: f64 = (-locals.var_vxbgmtcl);
        let assign91220_e139864: f64 = (locals.var_beta * assign91220_e139863);
        let assign91220_e139866: f64 = if assign91220_e139864 >= 500.0 { 1.0 } else { 0.0 };
        locals.var_guard2134 = assign91220_e139866;
        locals.var_guard2134_rv = 0.0;

        let (assign91230_e139883, assign91230_e139883_d_n0, assign91230_e139883_d_n2, assign91230_e139883_d_n4, assign91230_e139883_d_n5, assign91230_e139883_d_n6, assign91230_e139883_d_n7, assign91230_e139883_d_n8, assign91230_e139883_d_n9, assign91230_e139883_d_n10, assign91230_e139883_d_n11, assign91230_e139883_d_n14,) = {
    if (((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2134 != 0.0)) {
        let assign91230_e139876: f64 = (-locals.var_vxbgmtcl);
        let assign91230_e139877: f64 = (locals.var_beta * assign91230_e139876);
        let assign91230_e139878: f64 = (1.0 + assign91230_e139877);
        let assign91230_e139880: f64 = (assign91230_e139878 - 500.0);
        let assign91230_e139881: f64 = (1.403592217853e217 * assign91230_e139880);
        (assign91230_e139881, (1.403592217853e217 * ((locals.var_beta_dn0 * assign91230_e139876) + (locals.var_beta * (-locals.var_vxbgmtcl_dn0)))), (1.403592217853e217 * ((locals.var_beta_dn2 * assign91230_e139876) + (locals.var_beta * (-locals.var_vxbgmtcl_dn2)))), (1.403592217853e217 * ((locals.var_beta_dn4 * assign91230_e139876) + (locals.var_beta * (-locals.var_vxbgmtcl_dn4)))), (1.403592217853e217 * ((locals.var_beta_dn5 * assign91230_e139876) + (locals.var_beta * (-locals.var_vxbgmtcl_dn5)))), (1.403592217853e217 * ((locals.var_beta_dn6 * assign91230_e139876) + (locals.var_beta * (-locals.var_vxbgmtcl_dn6)))), (1.403592217853e217 * ((locals.var_beta_dn7 * assign91230_e139876) + (locals.var_beta * (-locals.var_vxbgmtcl_dn7)))), (1.403592217853e217 * ((locals.var_beta_dn8 * assign91230_e139876) + (locals.var_beta * (-locals.var_vxbgmtcl_dn8)))), (1.403592217853e217 * ((locals.var_beta_dn9 * assign91230_e139876) + (locals.var_beta * (-locals.var_vxbgmtcl_dn9)))), (1.403592217853e217 * ((locals.var_beta_dn10 * assign91230_e139876) + (locals.var_beta * (-locals.var_vxbgmtcl_dn10)))), (1.403592217853e217 * ((locals.var_beta_dn11 * assign91230_e139876) + (locals.var_beta * (-locals.var_vxbgmtcl_dn11)))), (1.403592217853e217 * ((locals.var_beta_dn14 * assign91230_e139876) + (locals.var_beta * (-locals.var_vxbgmtcl_dn14)))),)
    } else {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn11, locals.var_exp_bvbs_dn14,)
    }
};
        locals.var_exp_bvbs = assign91230_e139883;
        locals.var_exp_bvbs_dn0 = assign91230_e139883_d_n0;
        locals.var_exp_bvbs_dn2 = assign91230_e139883_d_n2;
        locals.var_exp_bvbs_dn4 = assign91230_e139883_d_n4;
        locals.var_exp_bvbs_dn5 = assign91230_e139883_d_n5;
        locals.var_exp_bvbs_dn6 = assign91230_e139883_d_n6;
        locals.var_exp_bvbs_dn7 = assign91230_e139883_d_n7;
        locals.var_exp_bvbs_dn8 = assign91230_e139883_d_n8;
        locals.var_exp_bvbs_dn9 = assign91230_e139883_d_n9;
        locals.var_exp_bvbs_dn10 = assign91230_e139883_d_n10;
        locals.var_exp_bvbs_dn11 = assign91230_e139883_d_n11;
        locals.var_exp_bvbs_dn14 = assign91230_e139883_d_n14;
        locals.var_exp_bvbs_rv = 0.0;

        let (assign91240_e139891, assign91240_e139891_d_n0, assign91240_e139891_d_n2, assign91240_e139891_d_n4, assign91240_e139891_d_n5, assign91240_e139891_d_n6, assign91240_e139891_d_n7, assign91240_e139891_d_n8, assign91240_e139891_d_n9, assign91240_e139891_d_n10, assign91240_e139891_d_n11, assign91240_e139891_d_n14,) = {
    if (((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2134 != 0.0)) {
        (1.403592217853e217, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign91240_e139891;
        locals.var_t0_dn0 = assign91240_e139891_d_n0;
        locals.var_t0_dn2 = assign91240_e139891_d_n2;
        locals.var_t0_dn4 = assign91240_e139891_d_n4;
        locals.var_t0_dn5 = assign91240_e139891_d_n5;
        locals.var_t0_dn6 = assign91240_e139891_d_n6;
        locals.var_t0_dn7 = assign91240_e139891_d_n7;
        locals.var_t0_dn8 = assign91240_e139891_d_n8;
        locals.var_t0_dn9 = assign91240_e139891_d_n9;
        locals.var_t0_dn10 = assign91240_e139891_d_n10;
        locals.var_t0_dn11 = assign91240_e139891_d_n11;
        locals.var_t0_dn14 = assign91240_e139891_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign91250_e139903, assign91250_e139903_d_n0, assign91250_e139903_d_n2, assign91250_e139903_d_n4, assign91250_e139903_d_n5, assign91250_e139903_d_n6, assign91250_e139903_d_n7, assign91250_e139903_d_n8, assign91250_e139903_d_n9, assign91250_e139903_d_n10, assign91250_e139903_d_n11, assign91250_e139903_d_n14,) = {
    if (((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2134 == 0.0)) {
        let assign91250_e139900: f64 = (-locals.var_vxbgmtcl);
        let assign91250_e139901: f64 = (locals.var_beta * assign91250_e139900);
        (assign91250_e139901, ((locals.var_beta_dn0 * assign91250_e139900) + (locals.var_beta * (-locals.var_vxbgmtcl_dn0))), ((locals.var_beta_dn2 * assign91250_e139900) + (locals.var_beta * (-locals.var_vxbgmtcl_dn2))), ((locals.var_beta_dn4 * assign91250_e139900) + (locals.var_beta * (-locals.var_vxbgmtcl_dn4))), ((locals.var_beta_dn5 * assign91250_e139900) + (locals.var_beta * (-locals.var_vxbgmtcl_dn5))), ((locals.var_beta_dn6 * assign91250_e139900) + (locals.var_beta * (-locals.var_vxbgmtcl_dn6))), ((locals.var_beta_dn7 * assign91250_e139900) + (locals.var_beta * (-locals.var_vxbgmtcl_dn7))), ((locals.var_beta_dn8 * assign91250_e139900) + (locals.var_beta * (-locals.var_vxbgmtcl_dn8))), ((locals.var_beta_dn9 * assign91250_e139900) + (locals.var_beta * (-locals.var_vxbgmtcl_dn9))), ((locals.var_beta_dn10 * assign91250_e139900) + (locals.var_beta * (-locals.var_vxbgmtcl_dn10))), ((locals.var_beta_dn11 * assign91250_e139900) + (locals.var_beta * (-locals.var_vxbgmtcl_dn11))), ((locals.var_beta_dn14 * assign91250_e139900) + (locals.var_beta * (-locals.var_vxbgmtcl_dn14))),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign91250_e139903;
        locals.var_tmf1_dn0 = assign91250_e139903_d_n0;
        locals.var_tmf1_dn2 = assign91250_e139903_d_n2;
        locals.var_tmf1_dn4 = assign91250_e139903_d_n4;
        locals.var_tmf1_dn5 = assign91250_e139903_d_n5;
        locals.var_tmf1_dn6 = assign91250_e139903_d_n6;
        locals.var_tmf1_dn7 = assign91250_e139903_d_n7;
        locals.var_tmf1_dn8 = assign91250_e139903_d_n8;
        locals.var_tmf1_dn9 = assign91250_e139903_d_n9;
        locals.var_tmf1_dn10 = assign91250_e139903_d_n10;
        locals.var_tmf1_dn11 = assign91250_e139903_d_n11;
        locals.var_tmf1_dn14 = assign91250_e139903_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign91260_e139912, assign91260_e139912_d_n0, assign91260_e139912_d_n2, assign91260_e139912_d_n4, assign91260_e139912_d_n5, assign91260_e139912_d_n6, assign91260_e139912_d_n7, assign91260_e139912_d_n8, assign91260_e139912_d_n9, assign91260_e139912_d_n10, assign91260_e139912_d_n11, assign91260_e139912_d_n14,) = {
    if (((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2134 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn11, locals.var_exp_bvbs_dn14,)
    }
};
        locals.var_exp_bvbs = assign91260_e139912;
        locals.var_exp_bvbs_dn0 = assign91260_e139912_d_n0;
        locals.var_exp_bvbs_dn2 = assign91260_e139912_d_n2;
        locals.var_exp_bvbs_dn4 = assign91260_e139912_d_n4;
        locals.var_exp_bvbs_dn5 = assign91260_e139912_d_n5;
        locals.var_exp_bvbs_dn6 = assign91260_e139912_d_n6;
        locals.var_exp_bvbs_dn7 = assign91260_e139912_d_n7;
        locals.var_exp_bvbs_dn8 = assign91260_e139912_d_n8;
        locals.var_exp_bvbs_dn9 = assign91260_e139912_d_n9;
        locals.var_exp_bvbs_dn10 = assign91260_e139912_d_n10;
        locals.var_exp_bvbs_dn11 = assign91260_e139912_d_n11;
        locals.var_exp_bvbs_dn14 = assign91260_e139912_d_n14;
        locals.var_exp_bvbs_rv = 0.0;

        let mut assign91270_loop_guard: usize = 0;
        while {
            let assign91270_cond_e139922: f64 = if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2134 == 0.0)) && (locals.var_tmf1 >= 60.0)) { 1.0 } else { 0.0 };
            assign91270_cond_e139922 != 0.0
        } {
            assign91270_loop_guard += 1;
            assert!(assign91270_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign91270_body0_e139933, assign91270_body0_e139933_d_n0, assign91270_body0_e139933_d_n2, assign91270_body0_e139933_d_n4, assign91270_body0_e139933_d_n5, assign91270_body0_e139933_d_n6, assign91270_body0_e139933_d_n7, assign91270_body0_e139933_d_n8, assign91270_body0_e139933_d_n9, assign91270_body0_e139933_d_n10, assign91270_body0_e139933_d_n11, assign91270_body0_e139933_d_n14,) = {
    if (((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2134 == 0.0)) {
        let assign91270_body0_e139931: f64 = (locals.var_exp_bvbs * 1.14200738981568e26);
        (assign91270_body0_e139931, (locals.var_exp_bvbs_dn0 * 1.14200738981568e26), (locals.var_exp_bvbs_dn2 * 1.14200738981568e26), (locals.var_exp_bvbs_dn4 * 1.14200738981568e26), (locals.var_exp_bvbs_dn5 * 1.14200738981568e26), (locals.var_exp_bvbs_dn6 * 1.14200738981568e26), (locals.var_exp_bvbs_dn7 * 1.14200738981568e26), (locals.var_exp_bvbs_dn8 * 1.14200738981568e26), (locals.var_exp_bvbs_dn9 * 1.14200738981568e26), (locals.var_exp_bvbs_dn10 * 1.14200738981568e26), (locals.var_exp_bvbs_dn11 * 1.14200738981568e26), (locals.var_exp_bvbs_dn14 * 1.14200738981568e26),)
    } else {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn11, locals.var_exp_bvbs_dn14,)
    }
};
            locals.var_exp_bvbs = assign91270_body0_e139933;
            locals.var_exp_bvbs_dn0 = assign91270_body0_e139933_d_n0;
            locals.var_exp_bvbs_dn2 = assign91270_body0_e139933_d_n2;
            locals.var_exp_bvbs_dn4 = assign91270_body0_e139933_d_n4;
            locals.var_exp_bvbs_dn5 = assign91270_body0_e139933_d_n5;
            locals.var_exp_bvbs_dn6 = assign91270_body0_e139933_d_n6;
            locals.var_exp_bvbs_dn7 = assign91270_body0_e139933_d_n7;
            locals.var_exp_bvbs_dn8 = assign91270_body0_e139933_d_n8;
            locals.var_exp_bvbs_dn9 = assign91270_body0_e139933_d_n9;
            locals.var_exp_bvbs_dn10 = assign91270_body0_e139933_d_n10;
            locals.var_exp_bvbs_dn11 = assign91270_body0_e139933_d_n11;
            locals.var_exp_bvbs_dn14 = assign91270_body0_e139933_d_n14;
            locals.var_exp_bvbs_rv = 0.0;
            let (assign91270_body1_e139944, assign91270_body1_e139944_d_n0, assign91270_body1_e139944_d_n2, assign91270_body1_e139944_d_n4, assign91270_body1_e139944_d_n5, assign91270_body1_e139944_d_n6, assign91270_body1_e139944_d_n7, assign91270_body1_e139944_d_n8, assign91270_body1_e139944_d_n9, assign91270_body1_e139944_d_n10, assign91270_body1_e139944_d_n11, assign91270_body1_e139944_d_n14,) = {
    if (((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2134 == 0.0)) {
        let assign91270_body1_e139942: f64 = (locals.var_tmf1 - 60.0);
        (assign91270_body1_e139942, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
            locals.var_tmf1 = assign91270_body1_e139944;
            locals.var_tmf1_dn0 = assign91270_body1_e139944_d_n0;
            locals.var_tmf1_dn2 = assign91270_body1_e139944_d_n2;
            locals.var_tmf1_dn4 = assign91270_body1_e139944_d_n4;
            locals.var_tmf1_dn5 = assign91270_body1_e139944_d_n5;
            locals.var_tmf1_dn6 = assign91270_body1_e139944_d_n6;
            locals.var_tmf1_dn7 = assign91270_body1_e139944_d_n7;
            locals.var_tmf1_dn8 = assign91270_body1_e139944_d_n8;
            locals.var_tmf1_dn9 = assign91270_body1_e139944_d_n9;
            locals.var_tmf1_dn10 = assign91270_body1_e139944_d_n10;
            locals.var_tmf1_dn11 = assign91270_body1_e139944_d_n11;
            locals.var_tmf1_dn14 = assign91270_body1_e139944_d_n14;
            locals.var_tmf1_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_352(
        locals: &mut StampLocals,
    ) {
        let (assign91280_e139956, assign91280_e139956_d_n0, assign91280_e139956_d_n2, assign91280_e139956_d_n4, assign91280_e139956_d_n5, assign91280_e139956_d_n6, assign91280_e139956_d_n7, assign91280_e139956_d_n8, assign91280_e139956_d_n9, assign91280_e139956_d_n10, assign91280_e139956_d_n11, assign91280_e139956_d_n14,) = {
    if (((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2134 == 0.0)) {
        let assign91280_e139953: f64 = (locals.var_tmf1).exp();
        let assign91280_e139954: f64 = (locals.var_exp_bvbs * assign91280_e139953);
        (assign91280_e139954, ((locals.var_exp_bvbs_dn0 * assign91280_e139953) + (locals.var_exp_bvbs * (assign91280_e139953 * locals.var_tmf1_dn0))), ((locals.var_exp_bvbs_dn2 * assign91280_e139953) + (locals.var_exp_bvbs * (assign91280_e139953 * locals.var_tmf1_dn2))), ((locals.var_exp_bvbs_dn4 * assign91280_e139953) + (locals.var_exp_bvbs * (assign91280_e139953 * locals.var_tmf1_dn4))), ((locals.var_exp_bvbs_dn5 * assign91280_e139953) + (locals.var_exp_bvbs * (assign91280_e139953 * locals.var_tmf1_dn5))), ((locals.var_exp_bvbs_dn6 * assign91280_e139953) + (locals.var_exp_bvbs * (assign91280_e139953 * locals.var_tmf1_dn6))), ((locals.var_exp_bvbs_dn7 * assign91280_e139953) + (locals.var_exp_bvbs * (assign91280_e139953 * locals.var_tmf1_dn7))), ((locals.var_exp_bvbs_dn8 * assign91280_e139953) + (locals.var_exp_bvbs * (assign91280_e139953 * locals.var_tmf1_dn8))), ((locals.var_exp_bvbs_dn9 * assign91280_e139953) + (locals.var_exp_bvbs * (assign91280_e139953 * locals.var_tmf1_dn9))), ((locals.var_exp_bvbs_dn10 * assign91280_e139953) + (locals.var_exp_bvbs * (assign91280_e139953 * locals.var_tmf1_dn10))), ((locals.var_exp_bvbs_dn11 * assign91280_e139953) + (locals.var_exp_bvbs * (assign91280_e139953 * locals.var_tmf1_dn11))), ((locals.var_exp_bvbs_dn14 * assign91280_e139953) + (locals.var_exp_bvbs * (assign91280_e139953 * locals.var_tmf1_dn14))),)
    } else {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn11, locals.var_exp_bvbs_dn14,)
    }
};
        locals.var_exp_bvbs = assign91280_e139956;
        locals.var_exp_bvbs_dn0 = assign91280_e139956_d_n0;
        locals.var_exp_bvbs_dn2 = assign91280_e139956_d_n2;
        locals.var_exp_bvbs_dn4 = assign91280_e139956_d_n4;
        locals.var_exp_bvbs_dn5 = assign91280_e139956_d_n5;
        locals.var_exp_bvbs_dn6 = assign91280_e139956_d_n6;
        locals.var_exp_bvbs_dn7 = assign91280_e139956_d_n7;
        locals.var_exp_bvbs_dn8 = assign91280_e139956_d_n8;
        locals.var_exp_bvbs_dn9 = assign91280_e139956_d_n9;
        locals.var_exp_bvbs_dn10 = assign91280_e139956_d_n10;
        locals.var_exp_bvbs_dn11 = assign91280_e139956_d_n11;
        locals.var_exp_bvbs_dn14 = assign91280_e139956_d_n14;
        locals.var_exp_bvbs_rv = 0.0;

        let (assign91290_e139965, assign91290_e139965_d_n0, assign91290_e139965_d_n2, assign91290_e139965_d_n4, assign91290_e139965_d_n5, assign91290_e139965_d_n6, assign91290_e139965_d_n7, assign91290_e139965_d_n8, assign91290_e139965_d_n9, assign91290_e139965_d_n10, assign91290_e139965_d_n11, assign91290_e139965_d_n14,) = {
    if (((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2134 == 0.0)) {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn11, locals.var_exp_bvbs_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign91290_e139965;
        locals.var_t0_dn0 = assign91290_e139965_d_n0;
        locals.var_t0_dn2 = assign91290_e139965_d_n2;
        locals.var_t0_dn4 = assign91290_e139965_d_n4;
        locals.var_t0_dn5 = assign91290_e139965_d_n5;
        locals.var_t0_dn6 = assign91290_e139965_d_n6;
        locals.var_t0_dn7 = assign91290_e139965_d_n7;
        locals.var_t0_dn8 = assign91290_e139965_d_n8;
        locals.var_t0_dn9 = assign91290_e139965_d_n9;
        locals.var_t0_dn10 = assign91290_e139965_d_n10;
        locals.var_t0_dn11 = assign91290_e139965_d_n11;
        locals.var_t0_dn14 = assign91290_e139965_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign91300_e139980, assign91300_e139980_d_n0, assign91300_e139980_d_n2, assign91300_e139980_d_n4, assign91300_e139980_d_n5, assign91300_e139980_d_n6, assign91300_e139980_d_n7, assign91300_e139980_d_n8, assign91300_e139980_d_n9, assign91300_e139980_d_n10, assign91300_e139980_d_n11, assign91300_e139980_d_n14,) = {
    if (((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign91300_e139972: f64 = (-locals.var_vgpld);
        let assign91300_e139974: f64 = (assign91300_e139972 * 0.5);
        let assign91300_e139976: f64 = (assign91300_e139974 - 0.5);
        let assign91300_e139978: f64 = (assign91300_e139976 - 1.0);
        (assign91300_e139978, 0.0, ((-locals.var_vgpld_dn2) * 0.5), 0.0, 0.0, 0.0, ((-locals.var_vgpld_dn7) * 0.5), ((-locals.var_vgpld_dn8) * 0.5), ((-locals.var_vgpld_dn9) * 0.5), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign91300_e139980;
        locals.var_tmf1_dn0 = assign91300_e139980_d_n0;
        locals.var_tmf1_dn2 = assign91300_e139980_d_n2;
        locals.var_tmf1_dn4 = assign91300_e139980_d_n4;
        locals.var_tmf1_dn5 = assign91300_e139980_d_n5;
        locals.var_tmf1_dn6 = assign91300_e139980_d_n6;
        locals.var_tmf1_dn7 = assign91300_e139980_d_n7;
        locals.var_tmf1_dn8 = assign91300_e139980_d_n8;
        locals.var_tmf1_dn9 = assign91300_e139980_d_n9;
        locals.var_tmf1_dn10 = assign91300_e139980_d_n10;
        locals.var_tmf1_dn11 = assign91300_e139980_d_n11;
        locals.var_tmf1_dn14 = assign91300_e139980_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign91310_e139992, assign91310_e139992_d_n0, assign91310_e139992_d_n2, assign91310_e139992_d_n4, assign91310_e139992_d_n5, assign91310_e139992_d_n6, assign91310_e139992_d_n7, assign91310_e139992_d_n8, assign91310_e139992_d_n9, assign91310_e139992_d_n10, assign91310_e139992_d_n11, assign91310_e139992_d_n14,) = {
    if (((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign91310_e139988: f64 = (4.0 * 0.5);
        let assign91310_e139990: f64 = assign91310_e139988;
        (assign91310_e139990, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign91310_e139992;
        locals.var_tmf2_dn0 = assign91310_e139992_d_n0;
        locals.var_tmf2_dn2 = assign91310_e139992_d_n2;
        locals.var_tmf2_dn4 = assign91310_e139992_d_n4;
        locals.var_tmf2_dn5 = assign91310_e139992_d_n5;
        locals.var_tmf2_dn6 = assign91310_e139992_d_n6;
        locals.var_tmf2_dn7 = assign91310_e139992_d_n7;
        locals.var_tmf2_dn8 = assign91310_e139992_d_n8;
        locals.var_tmf2_dn9 = assign91310_e139992_d_n9;
        locals.var_tmf2_dn10 = assign91310_e139992_d_n10;
        locals.var_tmf2_dn11 = assign91310_e139992_d_n11;
        locals.var_tmf2_dn14 = assign91310_e139992_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign91320_e140006, assign91320_e140006_d_n0, assign91320_e140006_d_n2, assign91320_e140006_d_n4, assign91320_e140006_d_n5, assign91320_e140006_d_n6, assign91320_e140006_d_n7, assign91320_e140006_d_n8, assign91320_e140006_d_n9, assign91320_e140006_d_n10, assign91320_e140006_d_n11, assign91320_e140006_d_n14,) = {
    if (((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let (assign91320_e140004, assign91320_e140004_d_n0, assign91320_e140004_d_n2, assign91320_e140004_d_n4, assign91320_e140004_d_n5, assign91320_e140004_d_n6, assign91320_e140004_d_n7, assign91320_e140004_d_n8, assign91320_e140004_d_n9, assign91320_e140004_d_n10, assign91320_e140004_d_n11, assign91320_e140004_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign91320_e140003: f64 = (-locals.var_tmf2);
                (assign91320_e140003, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign91320_e140004, assign91320_e140004_d_n0, assign91320_e140004_d_n2, assign91320_e140004_d_n4, assign91320_e140004_d_n5, assign91320_e140004_d_n6, assign91320_e140004_d_n7, assign91320_e140004_d_n8, assign91320_e140004_d_n9, assign91320_e140004_d_n10, assign91320_e140004_d_n11, assign91320_e140004_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign91320_e140006;
        locals.var_tmf2_dn0 = assign91320_e140006_d_n0;
        locals.var_tmf2_dn2 = assign91320_e140006_d_n2;
        locals.var_tmf2_dn4 = assign91320_e140006_d_n4;
        locals.var_tmf2_dn5 = assign91320_e140006_d_n5;
        locals.var_tmf2_dn6 = assign91320_e140006_d_n6;
        locals.var_tmf2_dn7 = assign91320_e140006_d_n7;
        locals.var_tmf2_dn8 = assign91320_e140006_d_n8;
        locals.var_tmf2_dn9 = assign91320_e140006_d_n9;
        locals.var_tmf2_dn10 = assign91320_e140006_d_n10;
        locals.var_tmf2_dn11 = assign91320_e140006_d_n11;
        locals.var_tmf2_dn14 = assign91320_e140006_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign91330_e140019, assign91330_e140019_d_n0, assign91330_e140019_d_n2, assign91330_e140019_d_n4, assign91330_e140019_d_n5, assign91330_e140019_d_n6, assign91330_e140019_d_n7, assign91330_e140019_d_n8, assign91330_e140019_d_n9, assign91330_e140019_d_n10, assign91330_e140019_d_n11, assign91330_e140019_d_n14,) = {
    if (((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign91330_e140014: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign91330_e140016: f64 = (assign91330_e140014 + locals.var_tmf2);
        let assign91330_e140017: f64 = (assign91330_e140016).sqrt();
        (assign91330_e140017, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign91330_e140017)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign91330_e140017)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign91330_e140017)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign91330_e140017)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign91330_e140017)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign91330_e140017)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign91330_e140017)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign91330_e140017)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign91330_e140017)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign91330_e140017)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign91330_e140017)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign91330_e140019;
        locals.var_tmf2_dn0 = assign91330_e140019_d_n0;
        locals.var_tmf2_dn2 = assign91330_e140019_d_n2;
        locals.var_tmf2_dn4 = assign91330_e140019_d_n4;
        locals.var_tmf2_dn5 = assign91330_e140019_d_n5;
        locals.var_tmf2_dn6 = assign91330_e140019_d_n6;
        locals.var_tmf2_dn7 = assign91330_e140019_d_n7;
        locals.var_tmf2_dn8 = assign91330_e140019_d_n8;
        locals.var_tmf2_dn9 = assign91330_e140019_d_n9;
        locals.var_tmf2_dn10 = assign91330_e140019_d_n10;
        locals.var_tmf2_dn11 = assign91330_e140019_d_n11;
        locals.var_tmf2_dn14 = assign91330_e140019_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign91340_e140033, assign91340_e140033_d_n0, assign91340_e140033_d_n2, assign91340_e140033_d_n4, assign91340_e140033_d_n5, assign91340_e140033_d_n6, assign91340_e140033_d_n7, assign91340_e140033_d_n8, assign91340_e140033_d_n9, assign91340_e140033_d_n10, assign91340_e140033_d_n11, assign91340_e140033_d_n14,) = {
    if (((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign91340_e140029: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign91340_e140030: f64 = (1.0 + assign91340_e140029);
        let assign91340_e140031: f64 = (0.5 * assign91340_e140030);
        (assign91340_e140031, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign91340_e140033;
        locals.var_t0_dn0 = assign91340_e140033_d_n0;
        locals.var_t0_dn2 = assign91340_e140033_d_n2;
        locals.var_t0_dn4 = assign91340_e140033_d_n4;
        locals.var_t0_dn5 = assign91340_e140033_d_n5;
        locals.var_t0_dn6 = assign91340_e140033_d_n6;
        locals.var_t0_dn7 = assign91340_e140033_d_n7;
        locals.var_t0_dn8 = assign91340_e140033_d_n8;
        locals.var_t0_dn9 = assign91340_e140033_d_n9;
        locals.var_t0_dn10 = assign91340_e140033_d_n10;
        locals.var_t0_dn11 = assign91340_e140033_d_n11;
        locals.var_t0_dn14 = assign91340_e140033_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign91350_e140047, assign91350_e140047_d_n0, assign91350_e140047_d_n2, assign91350_e140047_d_n4, assign91350_e140047_d_n5, assign91350_e140047_d_n6, assign91350_e140047_d_n7, assign91350_e140047_d_n8, assign91350_e140047_d_n9, assign91350_e140047_d_n10, assign91350_e140047_d_n11, assign91350_e140047_d_n14,) = {
    if (((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign91350_e140043: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign91350_e140044: f64 = (0.5 * assign91350_e140043);
        let assign91350_e140045: f64 = (0.5 + assign91350_e140044);
        (assign91350_e140045, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign91350_e140047;
        locals.var_t1_dn0 = assign91350_e140047_d_n0;
        locals.var_t1_dn2 = assign91350_e140047_d_n2;
        locals.var_t1_dn4 = assign91350_e140047_d_n4;
        locals.var_t1_dn5 = assign91350_e140047_d_n5;
        locals.var_t1_dn6 = assign91350_e140047_d_n6;
        locals.var_t1_dn7 = assign91350_e140047_d_n7;
        locals.var_t1_dn8 = assign91350_e140047_d_n8;
        locals.var_t1_dn9 = assign91350_e140047_d_n9;
        locals.var_t1_dn10 = assign91350_e140047_d_n10;
        locals.var_t1_dn11 = assign91350_e140047_d_n11;
        locals.var_t1_dn14 = assign91350_e140047_d_n14;
        locals.var_t1_rv = 0.0;

        let assign91360_e140050: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign91360_e140053: f64 = (-locals.var_t1);
        let assign91360_e140058: f64 = if ((assign91360_e140050 > assign91360_e140053) && (locals.var_t1 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2135 = assign91360_e140058;
        locals.var_guard2135_rv = 0.0;

        let (assign91370_e140074, assign91370_e140074_d_n0, assign91370_e140074_d_n2, assign91370_e140074_d_n4, assign91370_e140074_d_n5, assign91370_e140074_d_n6, assign91370_e140074_d_n7, assign91370_e140074_d_n8, assign91370_e140074_d_n9, assign91370_e140074_d_n10, assign91370_e140074_d_n11, assign91370_e140074_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2135 != 0.0)) {
        let assign91370_e140068: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign91370_e140070: f64 = assign91370_e140068;
        let assign91370_e140072: f64 = (assign91370_e140070 + locals.var_t1);
        (assign91370_e140072, (locals.var_vxbgmtcl_dn0 + locals.var_t1_dn0), ((locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2) + locals.var_t1_dn2), (locals.var_vxbgmtcl_dn4 + locals.var_t1_dn4), (locals.var_vxbgmtcl_dn5 + locals.var_t1_dn5), (locals.var_vxbgmtcl_dn6 + locals.var_t1_dn6), ((locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7) + locals.var_t1_dn7), ((locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8) + locals.var_t1_dn8), ((locals.var_vgpld_dn9 + locals.var_vxbgmtcl_dn9) + locals.var_t1_dn9), (locals.var_vxbgmtcl_dn10 + locals.var_t1_dn10), (locals.var_vxbgmtcl_dn11 + locals.var_t1_dn11), (locals.var_vxbgmtcl_dn14 + locals.var_t1_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign91370_e140074;
        locals.var_tmf1_dn0 = assign91370_e140074_d_n0;
        locals.var_tmf1_dn2 = assign91370_e140074_d_n2;
        locals.var_tmf1_dn4 = assign91370_e140074_d_n4;
        locals.var_tmf1_dn5 = assign91370_e140074_d_n5;
        locals.var_tmf1_dn6 = assign91370_e140074_d_n6;
        locals.var_tmf1_dn7 = assign91370_e140074_d_n7;
        locals.var_tmf1_dn8 = assign91370_e140074_d_n8;
        locals.var_tmf1_dn9 = assign91370_e140074_d_n9;
        locals.var_tmf1_dn10 = assign91370_e140074_d_n10;
        locals.var_tmf1_dn11 = assign91370_e140074_d_n11;
        locals.var_tmf1_dn14 = assign91370_e140074_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign91380_e140086, assign91380_e140086_d_n0, assign91380_e140086_d_n2, assign91380_e140086_d_n4, assign91380_e140086_d_n5, assign91380_e140086_d_n6, assign91380_e140086_d_n7, assign91380_e140086_d_n8, assign91380_e140086_d_n9, assign91380_e140086_d_n10, assign91380_e140086_d_n11, assign91380_e140086_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2135 != 0.0)) {
        let assign91380_e140084: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign91380_e140084, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign91380_e140086;
        locals.var_x2_dn0 = assign91380_e140086_d_n0;
        locals.var_x2_dn2 = assign91380_e140086_d_n2;
        locals.var_x2_dn4 = assign91380_e140086_d_n4;
        locals.var_x2_dn5 = assign91380_e140086_d_n5;
        locals.var_x2_dn6 = assign91380_e140086_d_n6;
        locals.var_x2_dn7 = assign91380_e140086_d_n7;
        locals.var_x2_dn8 = assign91380_e140086_d_n8;
        locals.var_x2_dn9 = assign91380_e140086_d_n9;
        locals.var_x2_dn10 = assign91380_e140086_d_n10;
        locals.var_x2_dn11 = assign91380_e140086_d_n11;
        locals.var_x2_dn14 = assign91380_e140086_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign91390_e140098, assign91390_e140098_d_n0, assign91390_e140098_d_n2, assign91390_e140098_d_n4, assign91390_e140098_d_n5, assign91390_e140098_d_n6, assign91390_e140098_d_n7, assign91390_e140098_d_n8, assign91390_e140098_d_n9, assign91390_e140098_d_n10, assign91390_e140098_d_n11, assign91390_e140098_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2135 != 0.0)) {
        let assign91390_e140096: f64 = (locals.var_t1 * locals.var_t1);
        (assign91390_e140096, ((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)), ((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)), ((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)), ((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)), ((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)), ((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)), ((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)), ((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)), ((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)), ((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)), ((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14)),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign91390_e140098;
        locals.var_xmax2_dn0 = assign91390_e140098_d_n0;
        locals.var_xmax2_dn2 = assign91390_e140098_d_n2;
        locals.var_xmax2_dn4 = assign91390_e140098_d_n4;
        locals.var_xmax2_dn5 = assign91390_e140098_d_n5;
        locals.var_xmax2_dn6 = assign91390_e140098_d_n6;
        locals.var_xmax2_dn7 = assign91390_e140098_d_n7;
        locals.var_xmax2_dn8 = assign91390_e140098_d_n8;
        locals.var_xmax2_dn9 = assign91390_e140098_d_n9;
        locals.var_xmax2_dn10 = assign91390_e140098_d_n10;
        locals.var_xmax2_dn11 = assign91390_e140098_d_n11;
        locals.var_xmax2_dn14 = assign91390_e140098_d_n14;
        locals.var_xmax2_rv = 0.0;

        let (assign91400_e140108, assign91400_e140108_d_n0, assign91400_e140108_d_n2, assign91400_e140108_d_n4, assign91400_e140108_d_n5, assign91400_e140108_d_n6, assign91400_e140108_d_n7, assign91400_e140108_d_n8, assign91400_e140108_d_n9, assign91400_e140108_d_n10, assign91400_e140108_d_n11, assign91400_e140108_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2135 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign91400_e140108;
        locals.var_xp_dn0 = assign91400_e140108_d_n0;
        locals.var_xp_dn2 = assign91400_e140108_d_n2;
        locals.var_xp_dn4 = assign91400_e140108_d_n4;
        locals.var_xp_dn5 = assign91400_e140108_d_n5;
        locals.var_xp_dn6 = assign91400_e140108_d_n6;
        locals.var_xp_dn7 = assign91400_e140108_d_n7;
        locals.var_xp_dn8 = assign91400_e140108_d_n8;
        locals.var_xp_dn9 = assign91400_e140108_d_n9;
        locals.var_xp_dn10 = assign91400_e140108_d_n10;
        locals.var_xp_dn11 = assign91400_e140108_d_n11;
        locals.var_xp_dn14 = assign91400_e140108_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign91410_e140118, assign91410_e140118_d_n0, assign91410_e140118_d_n2, assign91410_e140118_d_n4, assign91410_e140118_d_n5, assign91410_e140118_d_n6, assign91410_e140118_d_n7, assign91410_e140118_d_n8, assign91410_e140118_d_n9, assign91410_e140118_d_n10, assign91410_e140118_d_n11, assign91410_e140118_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2135 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign91410_e140118;
        locals.var_xmp_dn0 = assign91410_e140118_d_n0;
        locals.var_xmp_dn2 = assign91410_e140118_d_n2;
        locals.var_xmp_dn4 = assign91410_e140118_d_n4;
        locals.var_xmp_dn5 = assign91410_e140118_d_n5;
        locals.var_xmp_dn6 = assign91410_e140118_d_n6;
        locals.var_xmp_dn7 = assign91410_e140118_d_n7;
        locals.var_xmp_dn8 = assign91410_e140118_d_n8;
        locals.var_xmp_dn9 = assign91410_e140118_d_n9;
        locals.var_xmp_dn10 = assign91410_e140118_d_n10;
        locals.var_xmp_dn11 = assign91410_e140118_d_n11;
        locals.var_xmp_dn14 = assign91410_e140118_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign91420_e140128,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2135 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign91420_e140128;
        locals.var_m0_rv = 0.0;

        let (assign91430_e140138,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2135 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign91430_e140138;
        locals.var_mm_rv = 0.0;

        let (assign91440_e140148, assign91440_e140148_d_n0, assign91440_e140148_d_n2, assign91440_e140148_d_n4, assign91440_e140148_d_n5, assign91440_e140148_d_n6, assign91440_e140148_d_n7, assign91440_e140148_d_n8, assign91440_e140148_d_n9, assign91440_e140148_d_n10, assign91440_e140148_d_n11, assign91440_e140148_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2135 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign91440_e140148;
        locals.var_arg_dn0 = assign91440_e140148_d_n0;
        locals.var_arg_dn2 = assign91440_e140148_d_n2;
        locals.var_arg_dn4 = assign91440_e140148_d_n4;
        locals.var_arg_dn5 = assign91440_e140148_d_n5;
        locals.var_arg_dn6 = assign91440_e140148_d_n6;
        locals.var_arg_dn7 = assign91440_e140148_d_n7;
        locals.var_arg_dn8 = assign91440_e140148_d_n8;
        locals.var_arg_dn9 = assign91440_e140148_d_n9;
        locals.var_arg_dn10 = assign91440_e140148_d_n10;
        locals.var_arg_dn11 = assign91440_e140148_d_n11;
        locals.var_arg_dn14 = assign91440_e140148_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign91450_e140158, assign91450_e140158_d_n0, assign91450_e140158_d_n2, assign91450_e140158_d_n4, assign91450_e140158_d_n5, assign91450_e140158_d_n6, assign91450_e140158_d_n7, assign91450_e140158_d_n8, assign91450_e140158_d_n9, assign91450_e140158_d_n10, assign91450_e140158_d_n11, assign91450_e140158_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2135 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign91450_e140158;
        locals.var_dnm_dn0 = assign91450_e140158_d_n0;
        locals.var_dnm_dn2 = assign91450_e140158_d_n2;
        locals.var_dnm_dn4 = assign91450_e140158_d_n4;
        locals.var_dnm_dn5 = assign91450_e140158_d_n5;
        locals.var_dnm_dn6 = assign91450_e140158_d_n6;
        locals.var_dnm_dn7 = assign91450_e140158_d_n7;
        locals.var_dnm_dn8 = assign91450_e140158_d_n8;
        locals.var_dnm_dn9 = assign91450_e140158_d_n9;
        locals.var_dnm_dn10 = assign91450_e140158_d_n10;
        locals.var_dnm_dn11 = assign91450_e140158_d_n11;
        locals.var_dnm_dn14 = assign91450_e140158_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign91460_e140170, assign91460_e140170_d_n0, assign91460_e140170_d_n2, assign91460_e140170_d_n4, assign91460_e140170_d_n5, assign91460_e140170_d_n6, assign91460_e140170_d_n7, assign91460_e140170_d_n8, assign91460_e140170_d_n9, assign91460_e140170_d_n10, assign91460_e140170_d_n11, assign91460_e140170_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2135 != 0.0)) {
        let assign91460_e140168: f64 = (locals.var_xp * locals.var_x2);
        (assign91460_e140168, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign91460_e140170;
        locals.var_xp_dn0 = assign91460_e140170_d_n0;
        locals.var_xp_dn2 = assign91460_e140170_d_n2;
        locals.var_xp_dn4 = assign91460_e140170_d_n4;
        locals.var_xp_dn5 = assign91460_e140170_d_n5;
        locals.var_xp_dn6 = assign91460_e140170_d_n6;
        locals.var_xp_dn7 = assign91460_e140170_d_n7;
        locals.var_xp_dn8 = assign91460_e140170_d_n8;
        locals.var_xp_dn9 = assign91460_e140170_d_n9;
        locals.var_xp_dn10 = assign91460_e140170_d_n10;
        locals.var_xp_dn11 = assign91460_e140170_d_n11;
        locals.var_xp_dn14 = assign91460_e140170_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign91470_e140182, assign91470_e140182_d_n0, assign91470_e140182_d_n2, assign91470_e140182_d_n4, assign91470_e140182_d_n5, assign91470_e140182_d_n6, assign91470_e140182_d_n7, assign91470_e140182_d_n8, assign91470_e140182_d_n9, assign91470_e140182_d_n10, assign91470_e140182_d_n11, assign91470_e140182_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2135 != 0.0)) {
        let assign91470_e140180: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign91470_e140180, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign91470_e140182;
        locals.var_xmp_dn0 = assign91470_e140182_d_n0;
        locals.var_xmp_dn2 = assign91470_e140182_d_n2;
        locals.var_xmp_dn4 = assign91470_e140182_d_n4;
        locals.var_xmp_dn5 = assign91470_e140182_d_n5;
        locals.var_xmp_dn6 = assign91470_e140182_d_n6;
        locals.var_xmp_dn7 = assign91470_e140182_d_n7;
        locals.var_xmp_dn8 = assign91470_e140182_d_n8;
        locals.var_xmp_dn9 = assign91470_e140182_d_n9;
        locals.var_xmp_dn10 = assign91470_e140182_d_n10;
        locals.var_xmp_dn11 = assign91470_e140182_d_n11;
        locals.var_xmp_dn14 = assign91470_e140182_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign91480_e140194, assign91480_e140194_d_n0, assign91480_e140194_d_n2, assign91480_e140194_d_n4, assign91480_e140194_d_n5, assign91480_e140194_d_n6, assign91480_e140194_d_n7, assign91480_e140194_d_n8, assign91480_e140194_d_n9, assign91480_e140194_d_n10, assign91480_e140194_d_n11, assign91480_e140194_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2135 != 0.0)) {
        let assign91480_e140192: f64 = (locals.var_xp + locals.var_xmp);
        (assign91480_e140192, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign91480_e140194;
        locals.var_arg_dn0 = assign91480_e140194_d_n0;
        locals.var_arg_dn2 = assign91480_e140194_d_n2;
        locals.var_arg_dn4 = assign91480_e140194_d_n4;
        locals.var_arg_dn5 = assign91480_e140194_d_n5;
        locals.var_arg_dn6 = assign91480_e140194_d_n6;
        locals.var_arg_dn7 = assign91480_e140194_d_n7;
        locals.var_arg_dn8 = assign91480_e140194_d_n8;
        locals.var_arg_dn9 = assign91480_e140194_d_n9;
        locals.var_arg_dn10 = assign91480_e140194_d_n10;
        locals.var_arg_dn11 = assign91480_e140194_d_n11;
        locals.var_arg_dn14 = assign91480_e140194_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign91490_e140204, assign91490_e140204_d_n0, assign91490_e140204_d_n2, assign91490_e140204_d_n4, assign91490_e140204_d_n5, assign91490_e140204_d_n6, assign91490_e140204_d_n7, assign91490_e140204_d_n8, assign91490_e140204_d_n9, assign91490_e140204_d_n10, assign91490_e140204_d_n11, assign91490_e140204_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2135 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign91490_e140204;
        locals.var_dnm_dn0 = assign91490_e140204_d_n0;
        locals.var_dnm_dn2 = assign91490_e140204_d_n2;
        locals.var_dnm_dn4 = assign91490_e140204_d_n4;
        locals.var_dnm_dn5 = assign91490_e140204_d_n5;
        locals.var_dnm_dn6 = assign91490_e140204_d_n6;
        locals.var_dnm_dn7 = assign91490_e140204_d_n7;
        locals.var_dnm_dn8 = assign91490_e140204_d_n8;
        locals.var_dnm_dn9 = assign91490_e140204_d_n9;
        locals.var_dnm_dn10 = assign91490_e140204_d_n10;
        locals.var_dnm_dn11 = assign91490_e140204_d_n11;
        locals.var_dnm_dn14 = assign91490_e140204_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign91500_e140219: f64 = if ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard2136 = assign91500_e140219;
        locals.var_guard2136_rv = 0.0;

        let assign91510_e140222: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2137 = assign91510_e140222;
        locals.var_guard2137_rv = 0.0;

        let (assign91520_e140236,) = {
    if ((((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2135 != 0.0)) && (locals.var_guard2136 != 0.0)) && (locals.var_guard2137 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign91520_e140236;
        locals.var_mm_rv = 0.0;

        let assign91530_e140239: f64 = if 1.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard2138 = assign91530_e140239;
        locals.var_guard2138_rv = 0.0;

        let (assign91540_e140256,) = {
    if (((((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2135 != 0.0)) && (locals.var_guard2136 != 0.0)) && (locals.var_guard2137 == 0.0)) && (locals.var_guard2138 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign91540_e140256;
        locals.var_mm_rv = 0.0;

        let assign91550_e140259: f64 = if 1.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard2139 = assign91550_e140259;
        locals.var_guard2139_rv = 0.0;

        let (assign91560_e140279,) = {
    if ((((((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2135 != 0.0)) && (locals.var_guard2136 != 0.0)) && (locals.var_guard2137 == 0.0)) && (locals.var_guard2138 == 0.0)) && (locals.var_guard2139 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign91560_e140279;
        locals.var_mm_rv = 0.0;

        let assign91570_e140282: f64 = if 1.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard2140 = assign91570_e140282;
        locals.var_guard2140_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_353(
        locals: &mut StampLocals,
    ) {
        let (assign91580_e140305,) = {
    if (((((((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2135 != 0.0)) && (locals.var_guard2136 != 0.0)) && (locals.var_guard2137 == 0.0)) && (locals.var_guard2138 == 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2140 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign91580_e140305;
        locals.var_mm_rv = 0.0;

        let (assign91590_e140317,) = {
    if (((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2135 != 0.0)) && (locals.var_guard2136 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign91590_e140317;
        locals.var_m0_rv = 0.0;

        let mut assign91600_loop_guard: usize = 0;
        while {
            let assign91600_cond_e140330: f64 = if ((((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2135 != 0.0)) && (locals.var_guard2136 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign91600_cond_e140330 != 0.0
        } {
            assign91600_loop_guard += 1;
            assert!(assign91600_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign91600_body0_e140343, assign91600_body0_e140343_d_n0, assign91600_body0_e140343_d_n2, assign91600_body0_e140343_d_n4, assign91600_body0_e140343_d_n5, assign91600_body0_e140343_d_n6, assign91600_body0_e140343_d_n7, assign91600_body0_e140343_d_n8, assign91600_body0_e140343_d_n9, assign91600_body0_e140343_d_n10, assign91600_body0_e140343_d_n11, assign91600_body0_e140343_d_n14,) = {
    if (((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2135 != 0.0)) && (locals.var_guard2136 != 0.0)) {
        let assign91600_body0_e140341: f64 = (locals.var_dnm).sqrt();
        (assign91600_body0_e140341, (locals.var_dnm_dn0 / (2.0 * assign91600_body0_e140341)), (locals.var_dnm_dn2 / (2.0 * assign91600_body0_e140341)), (locals.var_dnm_dn4 / (2.0 * assign91600_body0_e140341)), (locals.var_dnm_dn5 / (2.0 * assign91600_body0_e140341)), (locals.var_dnm_dn6 / (2.0 * assign91600_body0_e140341)), (locals.var_dnm_dn7 / (2.0 * assign91600_body0_e140341)), (locals.var_dnm_dn8 / (2.0 * assign91600_body0_e140341)), (locals.var_dnm_dn9 / (2.0 * assign91600_body0_e140341)), (locals.var_dnm_dn10 / (2.0 * assign91600_body0_e140341)), (locals.var_dnm_dn11 / (2.0 * assign91600_body0_e140341)), (locals.var_dnm_dn14 / (2.0 * assign91600_body0_e140341)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign91600_body0_e140343;
            locals.var_dnm_dn0 = assign91600_body0_e140343_d_n0;
            locals.var_dnm_dn2 = assign91600_body0_e140343_d_n2;
            locals.var_dnm_dn4 = assign91600_body0_e140343_d_n4;
            locals.var_dnm_dn5 = assign91600_body0_e140343_d_n5;
            locals.var_dnm_dn6 = assign91600_body0_e140343_d_n6;
            locals.var_dnm_dn7 = assign91600_body0_e140343_d_n7;
            locals.var_dnm_dn8 = assign91600_body0_e140343_d_n8;
            locals.var_dnm_dn9 = assign91600_body0_e140343_d_n9;
            locals.var_dnm_dn10 = assign91600_body0_e140343_d_n10;
            locals.var_dnm_dn11 = assign91600_body0_e140343_d_n11;
            locals.var_dnm_dn14 = assign91600_body0_e140343_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign91600_body1_e140357,) = {
    if (((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2135 != 0.0)) && (locals.var_guard2136 != 0.0)) {
        let assign91600_body1_e140355: f64 = (locals.var_m0 + 1.0);
        (assign91600_body1_e140355,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign91600_body1_e140357;
            locals.var_m0_rv = 0.0;
        }

        let (assign91610_e140381, assign91610_e140381_d_n0, assign91610_e140381_d_n2, assign91610_e140381_d_n4, assign91610_e140381_d_n5, assign91610_e140381_d_n6, assign91610_e140381_d_n7, assign91610_e140381_d_n8, assign91610_e140381_d_n9, assign91610_e140381_d_n10, assign91610_e140381_d_n11, assign91610_e140381_d_n14,) = {
    if (((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2135 != 0.0)) && (locals.var_guard2136 == 0.0)) {
        let (assign91610_e140379, assign91610_e140379_d_n0, assign91610_e140379_d_n2, assign91610_e140379_d_n4, assign91610_e140379_d_n5, assign91610_e140379_d_n6, assign91610_e140379_d_n7, assign91610_e140379_d_n8, assign91610_e140379_d_n9, assign91610_e140379_d_n10, assign91610_e140379_d_n11, assign91610_e140379_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign91610_e140376: f64 = 2.0;
                let assign91610_e140377: f64 = (1.0 / assign91610_e140376);
                let assign91610_e140378: f64 = (locals.var_dnm).powf(assign91610_e140377);
                (assign91610_e140378, if 0.0 == 0.0 && ((assign91610_e140377) as f64).is_finite() && ((assign91610_e140377) as f64).fract() == 0.0 { if assign91610_e140377 == 0.0 { 0.0 } else { (assign91610_e140377 * ((locals.var_dnm).powf(assign91610_e140377 - 1.0) * locals.var_dnm_dn0)) } } else { (assign91610_e140378 * (assign91610_e140377 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign91610_e140377) as f64).is_finite() && ((assign91610_e140377) as f64).fract() == 0.0 { if assign91610_e140377 == 0.0 { 0.0 } else { (assign91610_e140377 * ((locals.var_dnm).powf(assign91610_e140377 - 1.0) * locals.var_dnm_dn2)) } } else { (assign91610_e140378 * (assign91610_e140377 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign91610_e140377) as f64).is_finite() && ((assign91610_e140377) as f64).fract() == 0.0 { if assign91610_e140377 == 0.0 { 0.0 } else { (assign91610_e140377 * ((locals.var_dnm).powf(assign91610_e140377 - 1.0) * locals.var_dnm_dn4)) } } else { (assign91610_e140378 * (assign91610_e140377 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign91610_e140377) as f64).is_finite() && ((assign91610_e140377) as f64).fract() == 0.0 { if assign91610_e140377 == 0.0 { 0.0 } else { (assign91610_e140377 * ((locals.var_dnm).powf(assign91610_e140377 - 1.0) * locals.var_dnm_dn5)) } } else { (assign91610_e140378 * (assign91610_e140377 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign91610_e140377) as f64).is_finite() && ((assign91610_e140377) as f64).fract() == 0.0 { if assign91610_e140377 == 0.0 { 0.0 } else { (assign91610_e140377 * ((locals.var_dnm).powf(assign91610_e140377 - 1.0) * locals.var_dnm_dn6)) } } else { (assign91610_e140378 * (assign91610_e140377 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign91610_e140377) as f64).is_finite() && ((assign91610_e140377) as f64).fract() == 0.0 { if assign91610_e140377 == 0.0 { 0.0 } else { (assign91610_e140377 * ((locals.var_dnm).powf(assign91610_e140377 - 1.0) * locals.var_dnm_dn7)) } } else { (assign91610_e140378 * (assign91610_e140377 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign91610_e140377) as f64).is_finite() && ((assign91610_e140377) as f64).fract() == 0.0 { if assign91610_e140377 == 0.0 { 0.0 } else { (assign91610_e140377 * ((locals.var_dnm).powf(assign91610_e140377 - 1.0) * locals.var_dnm_dn8)) } } else { (assign91610_e140378 * (assign91610_e140377 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign91610_e140377) as f64).is_finite() && ((assign91610_e140377) as f64).fract() == 0.0 { if assign91610_e140377 == 0.0 { 0.0 } else { (assign91610_e140377 * ((locals.var_dnm).powf(assign91610_e140377 - 1.0) * locals.var_dnm_dn9)) } } else { (assign91610_e140378 * (assign91610_e140377 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign91610_e140377) as f64).is_finite() && ((assign91610_e140377) as f64).fract() == 0.0 { if assign91610_e140377 == 0.0 { 0.0 } else { (assign91610_e140377 * ((locals.var_dnm).powf(assign91610_e140377 - 1.0) * locals.var_dnm_dn10)) } } else { (assign91610_e140378 * (assign91610_e140377 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign91610_e140377) as f64).is_finite() && ((assign91610_e140377) as f64).fract() == 0.0 { if assign91610_e140377 == 0.0 { 0.0 } else { (assign91610_e140377 * ((locals.var_dnm).powf(assign91610_e140377 - 1.0) * locals.var_dnm_dn11)) } } else { (assign91610_e140378 * (assign91610_e140377 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign91610_e140377) as f64).is_finite() && ((assign91610_e140377) as f64).fract() == 0.0 { if assign91610_e140377 == 0.0 { 0.0 } else { (assign91610_e140377 * ((locals.var_dnm).powf(assign91610_e140377 - 1.0) * locals.var_dnm_dn14)) } } else { (assign91610_e140378 * (assign91610_e140377 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign91610_e140379, assign91610_e140379_d_n0, assign91610_e140379_d_n2, assign91610_e140379_d_n4, assign91610_e140379_d_n5, assign91610_e140379_d_n6, assign91610_e140379_d_n7, assign91610_e140379_d_n8, assign91610_e140379_d_n9, assign91610_e140379_d_n10, assign91610_e140379_d_n11, assign91610_e140379_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign91610_e140381;
        locals.var_dnm_dn0 = assign91610_e140381_d_n0;
        locals.var_dnm_dn2 = assign91610_e140381_d_n2;
        locals.var_dnm_dn4 = assign91610_e140381_d_n4;
        locals.var_dnm_dn5 = assign91610_e140381_d_n5;
        locals.var_dnm_dn6 = assign91610_e140381_d_n6;
        locals.var_dnm_dn7 = assign91610_e140381_d_n7;
        locals.var_dnm_dn8 = assign91610_e140381_d_n8;
        locals.var_dnm_dn9 = assign91610_e140381_d_n9;
        locals.var_dnm_dn10 = assign91610_e140381_d_n10;
        locals.var_dnm_dn11 = assign91610_e140381_d_n11;
        locals.var_dnm_dn14 = assign91610_e140381_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign91620_e140393, assign91620_e140393_d_n0, assign91620_e140393_d_n2, assign91620_e140393_d_n4, assign91620_e140393_d_n5, assign91620_e140393_d_n6, assign91620_e140393_d_n7, assign91620_e140393_d_n8, assign91620_e140393_d_n9, assign91620_e140393_d_n10, assign91620_e140393_d_n11, assign91620_e140393_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2135 != 0.0)) {
        let assign91620_e140391: f64 = (1.0 / locals.var_dnm);
        (assign91620_e140391, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign91620_e140393;
        locals.var_dnm_dn0 = assign91620_e140393_d_n0;
        locals.var_dnm_dn2 = assign91620_e140393_d_n2;
        locals.var_dnm_dn4 = assign91620_e140393_d_n4;
        locals.var_dnm_dn5 = assign91620_e140393_d_n5;
        locals.var_dnm_dn6 = assign91620_e140393_d_n6;
        locals.var_dnm_dn7 = assign91620_e140393_d_n7;
        locals.var_dnm_dn8 = assign91620_e140393_d_n8;
        locals.var_dnm_dn9 = assign91620_e140393_d_n9;
        locals.var_dnm_dn10 = assign91620_e140393_d_n10;
        locals.var_dnm_dn11 = assign91620_e140393_d_n11;
        locals.var_dnm_dn14 = assign91620_e140393_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign91630_e140407, assign91630_e140407_d_n0, assign91630_e140407_d_n2, assign91630_e140407_d_n4, assign91630_e140407_d_n5, assign91630_e140407_d_n6, assign91630_e140407_d_n7, assign91630_e140407_d_n8, assign91630_e140407_d_n9, assign91630_e140407_d_n10, assign91630_e140407_d_n11, assign91630_e140407_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2135 != 0.0)) {
        let assign91630_e140403: f64 = (locals.var_tmf1 * locals.var_t1);
        let assign91630_e140405: f64 = (assign91630_e140403 * locals.var_dnm);
        (assign91630_e140405, ((((locals.var_tmf1_dn0 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn0)) * locals.var_dnm) + (assign91630_e140403 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn2)) * locals.var_dnm) + (assign91630_e140403 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn4 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn4)) * locals.var_dnm) + (assign91630_e140403 * locals.var_dnm_dn4)), ((((locals.var_tmf1_dn5 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn5)) * locals.var_dnm) + (assign91630_e140403 * locals.var_dnm_dn5)), ((((locals.var_tmf1_dn6 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn6)) * locals.var_dnm) + (assign91630_e140403 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn7)) * locals.var_dnm) + (assign91630_e140403 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn8 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn8)) * locals.var_dnm) + (assign91630_e140403 * locals.var_dnm_dn8)), ((((locals.var_tmf1_dn9 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn9)) * locals.var_dnm) + (assign91630_e140403 * locals.var_dnm_dn9)), ((((locals.var_tmf1_dn10 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn10)) * locals.var_dnm) + (assign91630_e140403 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn11 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn11)) * locals.var_dnm) + (assign91630_e140403 * locals.var_dnm_dn11)), ((((locals.var_tmf1_dn14 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn14)) * locals.var_dnm) + (assign91630_e140403 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign91630_e140407;
        locals.var_tmf0_dn0 = assign91630_e140407_d_n0;
        locals.var_tmf0_dn2 = assign91630_e140407_d_n2;
        locals.var_tmf0_dn4 = assign91630_e140407_d_n4;
        locals.var_tmf0_dn5 = assign91630_e140407_d_n5;
        locals.var_tmf0_dn6 = assign91630_e140407_d_n6;
        locals.var_tmf0_dn7 = assign91630_e140407_d_n7;
        locals.var_tmf0_dn8 = assign91630_e140407_d_n8;
        locals.var_tmf0_dn9 = assign91630_e140407_d_n9;
        locals.var_tmf0_dn10 = assign91630_e140407_d_n10;
        locals.var_tmf0_dn11 = assign91630_e140407_d_n11;
        locals.var_tmf0_dn14 = assign91630_e140407_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign91640_e140423, assign91640_e140423_d_n0, assign91640_e140423_d_n2, assign91640_e140423_d_n4, assign91640_e140423_d_n5, assign91640_e140423_d_n6, assign91640_e140423_d_n7, assign91640_e140423_d_n8, assign91640_e140423_d_n9, assign91640_e140423_d_n10, assign91640_e140423_d_n11, assign91640_e140423_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2135 != 0.0)) {
        let assign91640_e140417: f64 = (locals.var_t1 * locals.var_xmp);
        let assign91640_e140419: f64 = (assign91640_e140417 * locals.var_dnm);
        let assign91640_e140421: f64 = (assign91640_e140419 / locals.var_arg);
        (assign91640_e140421, (((((((locals.var_t1_dn0 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn0)) * locals.var_dnm) + (assign91640_e140417 * locals.var_dnm_dn0)) * locals.var_arg) - (assign91640_e140419 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn2 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn2)) * locals.var_dnm) + (assign91640_e140417 * locals.var_dnm_dn2)) * locals.var_arg) - (assign91640_e140419 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn4 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn4)) * locals.var_dnm) + (assign91640_e140417 * locals.var_dnm_dn4)) * locals.var_arg) - (assign91640_e140419 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn5 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn5)) * locals.var_dnm) + (assign91640_e140417 * locals.var_dnm_dn5)) * locals.var_arg) - (assign91640_e140419 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn6 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn6)) * locals.var_dnm) + (assign91640_e140417 * locals.var_dnm_dn6)) * locals.var_arg) - (assign91640_e140419 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn7 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn7)) * locals.var_dnm) + (assign91640_e140417 * locals.var_dnm_dn7)) * locals.var_arg) - (assign91640_e140419 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn8 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn8)) * locals.var_dnm) + (assign91640_e140417 * locals.var_dnm_dn8)) * locals.var_arg) - (assign91640_e140419 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn9 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn9)) * locals.var_dnm) + (assign91640_e140417 * locals.var_dnm_dn9)) * locals.var_arg) - (assign91640_e140419 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn10 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn10)) * locals.var_dnm) + (assign91640_e140417 * locals.var_dnm_dn10)) * locals.var_arg) - (assign91640_e140419 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn11 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn11)) * locals.var_dnm) + (assign91640_e140417 * locals.var_dnm_dn11)) * locals.var_arg) - (assign91640_e140419 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn14 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn14)) * locals.var_dnm) + (assign91640_e140417 * locals.var_dnm_dn14)) * locals.var_arg) - (assign91640_e140419 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign91640_e140423;
        locals.var_t0_dn0 = assign91640_e140423_d_n0;
        locals.var_t0_dn2 = assign91640_e140423_d_n2;
        locals.var_t0_dn4 = assign91640_e140423_d_n4;
        locals.var_t0_dn5 = assign91640_e140423_d_n5;
        locals.var_t0_dn6 = assign91640_e140423_d_n6;
        locals.var_t0_dn7 = assign91640_e140423_d_n7;
        locals.var_t0_dn8 = assign91640_e140423_d_n8;
        locals.var_t0_dn9 = assign91640_e140423_d_n9;
        locals.var_t0_dn10 = assign91640_e140423_d_n10;
        locals.var_t0_dn11 = assign91640_e140423_d_n11;
        locals.var_t0_dn14 = assign91640_e140423_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign91650_e140437, assign91650_e140437_d_n0, assign91650_e140437_d_n2, assign91650_e140437_d_n4, assign91650_e140437_d_n5, assign91650_e140437_d_n6, assign91650_e140437_d_n7, assign91650_e140437_d_n8, assign91650_e140437_d_n9, assign91650_e140437_d_n10, assign91650_e140437_d_n11, assign91650_e140437_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2135 != 0.0)) {
        let assign91650_e140433: f64 = (-locals.var_t1);
        let assign91650_e140435: f64 = (assign91650_e140433 + locals.var_tmf0);
        (assign91650_e140435, ((-locals.var_t1_dn0) + locals.var_tmf0_dn0), ((-locals.var_t1_dn2) + locals.var_tmf0_dn2), ((-locals.var_t1_dn4) + locals.var_tmf0_dn4), ((-locals.var_t1_dn5) + locals.var_tmf0_dn5), ((-locals.var_t1_dn6) + locals.var_tmf0_dn6), ((-locals.var_t1_dn7) + locals.var_tmf0_dn7), ((-locals.var_t1_dn8) + locals.var_tmf0_dn8), ((-locals.var_t1_dn9) + locals.var_tmf0_dn9), ((-locals.var_t1_dn10) + locals.var_tmf0_dn10), ((-locals.var_t1_dn11) + locals.var_tmf0_dn11), ((-locals.var_t1_dn14) + locals.var_tmf0_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign91650_e140437;
        locals.var_t1_dn0 = assign91650_e140437_d_n0;
        locals.var_t1_dn2 = assign91650_e140437_d_n2;
        locals.var_t1_dn4 = assign91650_e140437_d_n4;
        locals.var_t1_dn5 = assign91650_e140437_d_n5;
        locals.var_t1_dn6 = assign91650_e140437_d_n6;
        locals.var_t1_dn7 = assign91650_e140437_d_n7;
        locals.var_t1_dn8 = assign91650_e140437_d_n8;
        locals.var_t1_dn9 = assign91650_e140437_d_n9;
        locals.var_t1_dn10 = assign91650_e140437_d_n10;
        locals.var_t1_dn11 = assign91650_e140437_d_n11;
        locals.var_t1_dn14 = assign91650_e140437_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign91660_e140447, assign91660_e140447_d_n0, assign91660_e140447_d_n2, assign91660_e140447_d_n4, assign91660_e140447_d_n5, assign91660_e140447_d_n6, assign91660_e140447_d_n7, assign91660_e140447_d_n8, assign91660_e140447_d_n9, assign91660_e140447_d_n10, assign91660_e140447_d_n11, assign91660_e140447_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2135 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign91660_e140447;
        locals.var_t0_dn0 = assign91660_e140447_d_n0;
        locals.var_t0_dn2 = assign91660_e140447_d_n2;
        locals.var_t0_dn4 = assign91660_e140447_d_n4;
        locals.var_t0_dn5 = assign91660_e140447_d_n5;
        locals.var_t0_dn6 = assign91660_e140447_d_n6;
        locals.var_t0_dn7 = assign91660_e140447_d_n7;
        locals.var_t0_dn8 = assign91660_e140447_d_n8;
        locals.var_t0_dn9 = assign91660_e140447_d_n9;
        locals.var_t0_dn10 = assign91660_e140447_d_n10;
        locals.var_t0_dn11 = assign91660_e140447_d_n11;
        locals.var_t0_dn14 = assign91660_e140447_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign91670_e140460, assign91670_e140460_d_n0, assign91670_e140460_d_n2, assign91670_e140460_d_n4, assign91670_e140460_d_n5, assign91670_e140460_d_n6, assign91670_e140460_d_n7, assign91670_e140460_d_n8, assign91670_e140460_d_n9, assign91670_e140460_d_n10, assign91670_e140460_d_n11, assign91670_e140460_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2135 == 0.0)) {
        let assign91670_e140458: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        (assign91670_e140458, locals.var_vxbgmtcl_dn0, (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2), locals.var_vxbgmtcl_dn4, locals.var_vxbgmtcl_dn5, locals.var_vxbgmtcl_dn6, (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7), (locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8), (locals.var_vgpld_dn9 + locals.var_vxbgmtcl_dn9), locals.var_vxbgmtcl_dn10, locals.var_vxbgmtcl_dn11, locals.var_vxbgmtcl_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign91670_e140460;
        locals.var_t1_dn0 = assign91670_e140460_d_n0;
        locals.var_t1_dn2 = assign91670_e140460_d_n2;
        locals.var_t1_dn4 = assign91670_e140460_d_n4;
        locals.var_t1_dn5 = assign91670_e140460_d_n5;
        locals.var_t1_dn6 = assign91670_e140460_d_n6;
        locals.var_t1_dn7 = assign91670_e140460_d_n7;
        locals.var_t1_dn8 = assign91670_e140460_d_n8;
        locals.var_t1_dn9 = assign91670_e140460_d_n9;
        locals.var_t1_dn10 = assign91670_e140460_d_n10;
        locals.var_t1_dn11 = assign91670_e140460_d_n11;
        locals.var_t1_dn14 = assign91670_e140460_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign91680_e140471, assign91680_e140471_d_n0, assign91680_e140471_d_n2, assign91680_e140471_d_n4, assign91680_e140471_d_n5, assign91680_e140471_d_n6, assign91680_e140471_d_n7, assign91680_e140471_d_n8, assign91680_e140471_d_n9, assign91680_e140471_d_n10, assign91680_e140471_d_n11, assign91680_e140471_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2135 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign91680_e140471;
        locals.var_t0_dn0 = assign91680_e140471_d_n0;
        locals.var_t0_dn2 = assign91680_e140471_d_n2;
        locals.var_t0_dn4 = assign91680_e140471_d_n4;
        locals.var_t0_dn5 = assign91680_e140471_d_n5;
        locals.var_t0_dn6 = assign91680_e140471_d_n6;
        locals.var_t0_dn7 = assign91680_e140471_d_n7;
        locals.var_t0_dn8 = assign91680_e140471_d_n8;
        locals.var_t0_dn9 = assign91680_e140471_d_n9;
        locals.var_t0_dn10 = assign91680_e140471_d_n10;
        locals.var_t0_dn11 = assign91680_e140471_d_n11;
        locals.var_t0_dn14 = assign91680_e140471_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign91690_e140481, assign91690_e140481_d_n0, assign91690_e140481_d_n2, assign91690_e140481_d_n4, assign91690_e140481_d_n5, assign91690_e140481_d_n6, assign91690_e140481_d_n7, assign91690_e140481_d_n8, assign91690_e140481_d_n9, assign91690_e140481_d_n10, assign91690_e140481_d_n11, assign91690_e140481_d_n14,) = {
    if (((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign91690_e140479: f64 = (locals.var_t1 - locals.var_vgpld);
        (assign91690_e140479, locals.var_t1_dn0, (locals.var_t1_dn2 - locals.var_vgpld_dn2), locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, (locals.var_t1_dn7 - locals.var_vgpld_dn7), (locals.var_t1_dn8 - locals.var_vgpld_dn8), (locals.var_t1_dn9 - locals.var_vgpld_dn9), locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    } else {
        (locals.var_vxbgmtcl, locals.var_vxbgmtcl_dn0, locals.var_vxbgmtcl_dn2, locals.var_vxbgmtcl_dn4, locals.var_vxbgmtcl_dn5, locals.var_vxbgmtcl_dn6, locals.var_vxbgmtcl_dn7, locals.var_vxbgmtcl_dn8, locals.var_vxbgmtcl_dn9, locals.var_vxbgmtcl_dn10, locals.var_vxbgmtcl_dn11, locals.var_vxbgmtcl_dn14,)
    }
};
        locals.var_vxbgmtcl = assign91690_e140481;
        locals.var_vxbgmtcl_dn0 = assign91690_e140481_d_n0;
        locals.var_vxbgmtcl_dn2 = assign91690_e140481_d_n2;
        locals.var_vxbgmtcl_dn4 = assign91690_e140481_d_n4;
        locals.var_vxbgmtcl_dn5 = assign91690_e140481_d_n5;
        locals.var_vxbgmtcl_dn6 = assign91690_e140481_d_n6;
        locals.var_vxbgmtcl_dn7 = assign91690_e140481_d_n7;
        locals.var_vxbgmtcl_dn8 = assign91690_e140481_d_n8;
        locals.var_vxbgmtcl_dn9 = assign91690_e140481_d_n9;
        locals.var_vxbgmtcl_dn10 = assign91690_e140481_d_n10;
        locals.var_vxbgmtcl_dn11 = assign91690_e140481_d_n11;
        locals.var_vxbgmtcl_dn14 = assign91690_e140481_d_n14;
        locals.var_vxbgmtcl_rv = 0.0;

        let (assign91700_e140494, assign91700_e140494_d_n0, assign91700_e140494_d_n2, assign91700_e140494_d_n4, assign91700_e140494_d_n5, assign91700_e140494_d_n6, assign91700_e140494_d_n7, assign91700_e140494_d_n8, assign91700_e140494_d_n9, assign91700_e140494_d_n10, assign91700_e140494_d_n11, assign91700_e140494_d_n14,) = {
    if (((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign91700_e140488: f64 = (-locals.var_vxbgmtcl);
        let assign91700_e140491: f64 = (10.0 * 2.220446049250313e-16);
        let assign91700_e140492: f64 = (assign91700_e140488 + assign91700_e140491);
        (assign91700_e140492, (-locals.var_vxbgmtcl_dn0), (-locals.var_vxbgmtcl_dn2), (-locals.var_vxbgmtcl_dn4), (-locals.var_vxbgmtcl_dn5), (-locals.var_vxbgmtcl_dn6), (-locals.var_vxbgmtcl_dn7), (-locals.var_vxbgmtcl_dn8), (-locals.var_vxbgmtcl_dn9), (-locals.var_vxbgmtcl_dn10), (-locals.var_vxbgmtcl_dn11), (-locals.var_vxbgmtcl_dn14),)
    } else {
        (locals.var_vgb_fb_ld, locals.var_vgb_fb_ld_dn0, locals.var_vgb_fb_ld_dn2, locals.var_vgb_fb_ld_dn4, locals.var_vgb_fb_ld_dn5, locals.var_vgb_fb_ld_dn6, locals.var_vgb_fb_ld_dn7, locals.var_vgb_fb_ld_dn8, locals.var_vgb_fb_ld_dn9, locals.var_vgb_fb_ld_dn10, locals.var_vgb_fb_ld_dn11, locals.var_vgb_fb_ld_dn14,)
    }
};
        locals.var_vgb_fb_ld = assign91700_e140494;
        locals.var_vgb_fb_ld_dn0 = assign91700_e140494_d_n0;
        locals.var_vgb_fb_ld_dn2 = assign91700_e140494_d_n2;
        locals.var_vgb_fb_ld_dn4 = assign91700_e140494_d_n4;
        locals.var_vgb_fb_ld_dn5 = assign91700_e140494_d_n5;
        locals.var_vgb_fb_ld_dn6 = assign91700_e140494_d_n6;
        locals.var_vgb_fb_ld_dn7 = assign91700_e140494_d_n7;
        locals.var_vgb_fb_ld_dn8 = assign91700_e140494_d_n8;
        locals.var_vgb_fb_ld_dn9 = assign91700_e140494_d_n9;
        locals.var_vgb_fb_ld_dn10 = assign91700_e140494_d_n10;
        locals.var_vgb_fb_ld_dn11 = assign91700_e140494_d_n11;
        locals.var_vgb_fb_ld_dn14 = assign91700_e140494_d_n14;
        locals.var_vgb_fb_ld_rv = 0.0;

        let assign91710_e140497: f64 = if locals.var_vgpld < locals.var_vgb_fb_ld { 1.0 } else { 0.0 };
        locals.var_guard2141 = assign91710_e140497;
        locals.var_guard2141_rv = 0.0;

        let (assign91730_e140522, assign91730_e140522_d_n0, assign91730_e140522_d_n2, assign91730_e140522_d_n4, assign91730_e140522_d_n5, assign91730_e140522_d_n6, assign91730_e140522_d_n7, assign91730_e140522_d_n8, assign91730_e140522_d_n9, assign91730_e140522_d_n10, assign91730_e140522_d_n11, assign91730_e140522_d_n14,) = {
    if (((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 != 0.0)) {
        let assign91730_e140514: f64 = (2.0 * locals.var_beta_inv);
        let assign91730_e140516: f64 = (-locals.var_vgs_min);
        let assign91730_e140518: f64 = (assign91730_e140516 / locals.var_fac1);
        let assign91730_e140519: f64 = (assign91730_e140518).ln();
        let assign91730_e140520: f64 = (assign91730_e140514 * assign91730_e140519);
        (assign91730_e140520, (((2.0 * locals.var_beta_inv_dn0) * assign91730_e140519) + (assign91730_e140514 * ((-((assign91730_e140516 * locals.var_fac1_dn0) / (locals.var_fac1 * locals.var_fac1))) / assign91730_e140518))), (((2.0 * locals.var_beta_inv_dn2) * assign91730_e140519) + (assign91730_e140514 * ((-((assign91730_e140516 * locals.var_fac1_dn2) / (locals.var_fac1 * locals.var_fac1))) / assign91730_e140518))), (((2.0 * locals.var_beta_inv_dn4) * assign91730_e140519) + (assign91730_e140514 * ((-((assign91730_e140516 * locals.var_fac1_dn4) / (locals.var_fac1 * locals.var_fac1))) / assign91730_e140518))), (((2.0 * locals.var_beta_inv_dn5) * assign91730_e140519) + (assign91730_e140514 * ((-((assign91730_e140516 * locals.var_fac1_dn5) / (locals.var_fac1 * locals.var_fac1))) / assign91730_e140518))), (((2.0 * locals.var_beta_inv_dn6) * assign91730_e140519) + (assign91730_e140514 * ((-((assign91730_e140516 * locals.var_fac1_dn6) / (locals.var_fac1 * locals.var_fac1))) / assign91730_e140518))), (((2.0 * locals.var_beta_inv_dn7) * assign91730_e140519) + (assign91730_e140514 * ((-((assign91730_e140516 * locals.var_fac1_dn7) / (locals.var_fac1 * locals.var_fac1))) / assign91730_e140518))), (((2.0 * locals.var_beta_inv_dn8) * assign91730_e140519) + (assign91730_e140514 * ((-((assign91730_e140516 * locals.var_fac1_dn8) / (locals.var_fac1 * locals.var_fac1))) / assign91730_e140518))), (((2.0 * locals.var_beta_inv_dn9) * assign91730_e140519) + (assign91730_e140514 * ((-((assign91730_e140516 * locals.var_fac1_dn9) / (locals.var_fac1 * locals.var_fac1))) / assign91730_e140518))), (((2.0 * locals.var_beta_inv_dn10) * assign91730_e140519) + (assign91730_e140514 * ((-((assign91730_e140516 * locals.var_fac1_dn10) / (locals.var_fac1 * locals.var_fac1))) / assign91730_e140518))), (((2.0 * locals.var_beta_inv_dn11) * assign91730_e140519) + (assign91730_e140514 * ((-((assign91730_e140516 * locals.var_fac1_dn11) / (locals.var_fac1 * locals.var_fac1))) / assign91730_e140518))), (((2.0 * locals.var_beta_inv_dn14) * assign91730_e140519) + (assign91730_e140514 * ((-((assign91730_e140516 * locals.var_fac1_dn14) / (locals.var_fac1 * locals.var_fac1))) / assign91730_e140518))),)
    } else {
        (locals.var_ps0_min, locals.var_ps0_min_dn0, locals.var_ps0_min_dn2, locals.var_ps0_min_dn4, locals.var_ps0_min_dn5, locals.var_ps0_min_dn6, locals.var_ps0_min_dn7, locals.var_ps0_min_dn8, locals.var_ps0_min_dn9, locals.var_ps0_min_dn10, locals.var_ps0_min_dn11, locals.var_ps0_min_dn14,)
    }
};
        locals.var_ps0_min = assign91730_e140522;
        locals.var_ps0_min_dn0 = assign91730_e140522_d_n0;
        locals.var_ps0_min_dn2 = assign91730_e140522_d_n2;
        locals.var_ps0_min_dn4 = assign91730_e140522_d_n4;
        locals.var_ps0_min_dn5 = assign91730_e140522_d_n5;
        locals.var_ps0_min_dn6 = assign91730_e140522_d_n6;
        locals.var_ps0_min_dn7 = assign91730_e140522_d_n7;
        locals.var_ps0_min_dn8 = assign91730_e140522_d_n8;
        locals.var_ps0_min_dn9 = assign91730_e140522_d_n9;
        locals.var_ps0_min_dn10 = assign91730_e140522_d_n10;
        locals.var_ps0_min_dn11 = assign91730_e140522_d_n11;
        locals.var_ps0_min_dn14 = assign91730_e140522_d_n14;
        locals.var_ps0_min_rv = 0.0;

        let (assign91740_e140534, assign91740_e140534_d_n0, assign91740_e140534_d_n2, assign91740_e140534_d_n4, assign91740_e140534_d_n5, assign91740_e140534_d_n6, assign91740_e140534_d_n7, assign91740_e140534_d_n8, assign91740_e140534_d_n9, assign91740_e140534_d_n10, assign91740_e140534_d_n11, assign91740_e140534_d_n14,) = {
    if (((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 != 0.0)) {
        let assign91740_e140531: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign91740_e140532: f64 = (locals.var_beta * assign91740_e140531);
        (assign91740_e140532, ((locals.var_beta_dn0 * assign91740_e140531) + (locals.var_beta * locals.var_vxbgmtcl_dn0)), ((locals.var_beta_dn2 * assign91740_e140531) + (locals.var_beta * (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2))), ((locals.var_beta_dn4 * assign91740_e140531) + (locals.var_beta * locals.var_vxbgmtcl_dn4)), ((locals.var_beta_dn5 * assign91740_e140531) + (locals.var_beta * locals.var_vxbgmtcl_dn5)), ((locals.var_beta_dn6 * assign91740_e140531) + (locals.var_beta * locals.var_vxbgmtcl_dn6)), ((locals.var_beta_dn7 * assign91740_e140531) + (locals.var_beta * (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7))), ((locals.var_beta_dn8 * assign91740_e140531) + (locals.var_beta * (locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8))), ((locals.var_beta_dn9 * assign91740_e140531) + (locals.var_beta * (locals.var_vgpld_dn9 + locals.var_vxbgmtcl_dn9))), ((locals.var_beta_dn10 * assign91740_e140531) + (locals.var_beta * locals.var_vxbgmtcl_dn10)), ((locals.var_beta_dn11 * assign91740_e140531) + (locals.var_beta * locals.var_vxbgmtcl_dn11)), ((locals.var_beta_dn14 * assign91740_e140531) + (locals.var_beta * locals.var_vxbgmtcl_dn14)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign91740_e140534;
        locals.var_tx_dn0 = assign91740_e140534_d_n0;
        locals.var_tx_dn2 = assign91740_e140534_d_n2;
        locals.var_tx_dn4 = assign91740_e140534_d_n4;
        locals.var_tx_dn5 = assign91740_e140534_d_n5;
        locals.var_tx_dn6 = assign91740_e140534_d_n6;
        locals.var_tx_dn7 = assign91740_e140534_d_n7;
        locals.var_tx_dn8 = assign91740_e140534_d_n8;
        locals.var_tx_dn9 = assign91740_e140534_d_n9;
        locals.var_tx_dn10 = assign91740_e140534_d_n10;
        locals.var_tx_dn11 = assign91740_e140534_d_n11;
        locals.var_tx_dn14 = assign91740_e140534_d_n14;
        locals.var_tx_rv = 0.0;

        let (assign91750_e140546, assign91750_e140546_d_n0, assign91750_e140546_d_n2, assign91750_e140546_d_n4, assign91750_e140546_d_n5, assign91750_e140546_d_n6, assign91750_e140546_d_n7, assign91750_e140546_d_n8, assign91750_e140546_d_n9, assign91750_e140546_d_n10, assign91750_e140546_d_n11, assign91750_e140546_d_n14,) = {
    if (((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 != 0.0)) {
        let assign91750_e140543: f64 = (locals.var_beta * locals.var_cnst0over_func);
        let assign91750_e140544: f64 = (1.0 / assign91750_e140543);
        (assign91750_e140544, (-(((locals.var_beta_dn0 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn0)) / (assign91750_e140543 * assign91750_e140543))), (-(((locals.var_beta_dn2 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn2)) / (assign91750_e140543 * assign91750_e140543))), (-(((locals.var_beta_dn4 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn4)) / (assign91750_e140543 * assign91750_e140543))), (-(((locals.var_beta_dn5 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn5)) / (assign91750_e140543 * assign91750_e140543))), (-(((locals.var_beta_dn6 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn6)) / (assign91750_e140543 * assign91750_e140543))), (-(((locals.var_beta_dn7 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn7)) / (assign91750_e140543 * assign91750_e140543))), (-(((locals.var_beta_dn8 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn8)) / (assign91750_e140543 * assign91750_e140543))), (-(((locals.var_beta_dn9 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn9)) / (assign91750_e140543 * assign91750_e140543))), (-(((locals.var_beta_dn10 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn10)) / (assign91750_e140543 * assign91750_e140543))), (-(((locals.var_beta_dn11 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn11)) / (assign91750_e140543 * assign91750_e140543))), (-(((locals.var_beta_dn14 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn14)) / (assign91750_e140543 * assign91750_e140543))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign91750_e140546;
        locals.var_t1_dn0 = assign91750_e140546_d_n0;
        locals.var_t1_dn2 = assign91750_e140546_d_n2;
        locals.var_t1_dn4 = assign91750_e140546_d_n4;
        locals.var_t1_dn5 = assign91750_e140546_d_n5;
        locals.var_t1_dn6 = assign91750_e140546_d_n6;
        locals.var_t1_dn7 = assign91750_e140546_d_n7;
        locals.var_t1_dn8 = assign91750_e140546_d_n8;
        locals.var_t1_dn9 = assign91750_e140546_d_n9;
        locals.var_t1_dn10 = assign91750_e140546_d_n10;
        locals.var_t1_dn11 = assign91750_e140546_d_n11;
        locals.var_t1_dn14 = assign91750_e140546_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign91760_e140556, assign91760_e140556_d_n0, assign91760_e140556_d_n2, assign91760_e140556_d_n4, assign91760_e140556_d_n5, assign91760_e140556_d_n6, assign91760_e140556_d_n7, assign91760_e140556_d_n8, assign91760_e140556_d_n9, assign91760_e140556_d_n10, assign91760_e140556_d_n11, assign91760_e140556_d_n14,) = {
    if (((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 != 0.0)) {
        let assign91760_e140554: f64 = (locals.var_t1 * locals.var_cox0_func);
        (assign91760_e140554, (locals.var_t1_dn0 * locals.var_cox0_func), (locals.var_t1_dn2 * locals.var_cox0_func), (locals.var_t1_dn4 * locals.var_cox0_func), (locals.var_t1_dn5 * locals.var_cox0_func), (locals.var_t1_dn6 * locals.var_cox0_func), (locals.var_t1_dn7 * locals.var_cox0_func), (locals.var_t1_dn8 * locals.var_cox0_func), (locals.var_t1_dn9 * locals.var_cox0_func), (locals.var_t1_dn10 * locals.var_cox0_func), (locals.var_t1_dn11 * locals.var_cox0_func), (locals.var_t1_dn14 * locals.var_cox0_func),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn14,)
    }
};
        locals.var_ty = assign91760_e140556;
        locals.var_ty_dn0 = assign91760_e140556_d_n0;
        locals.var_ty_dn2 = assign91760_e140556_d_n2;
        locals.var_ty_dn4 = assign91760_e140556_d_n4;
        locals.var_ty_dn5 = assign91760_e140556_d_n5;
        locals.var_ty_dn6 = assign91760_e140556_d_n6;
        locals.var_ty_dn7 = assign91760_e140556_d_n7;
        locals.var_ty_dn8 = assign91760_e140556_d_n8;
        locals.var_ty_dn9 = assign91760_e140556_d_n9;
        locals.var_ty_dn10 = assign91760_e140556_d_n10;
        locals.var_ty_dn11 = assign91760_e140556_d_n11;
        locals.var_ty_dn14 = assign91760_e140556_d_n14;
        locals.var_ty_rv = 0.0;

        let (assign91770_e140570, assign91770_e140570_d_n0, assign91770_e140570_d_n2, assign91770_e140570_d_n4, assign91770_e140570_d_n5, assign91770_e140570_d_n6, assign91770_e140570_d_n7, assign91770_e140570_d_n8, assign91770_e140570_d_n9, assign91770_e140570_d_n10, assign91770_e140570_d_n11, assign91770_e140570_d_n14,) = {
    if (((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 != 0.0)) {
        let assign91770_e140565: f64 = (3.0 * 1.414213562373095);
        let assign91770_e140567: f64 = (assign91770_e140565 * locals.var_ty);
        let assign91770_e140568: f64 = (2.0 + assign91770_e140567);
        (assign91770_e140568, (assign91770_e140565 * locals.var_ty_dn0), (assign91770_e140565 * locals.var_ty_dn2), (assign91770_e140565 * locals.var_ty_dn4), (assign91770_e140565 * locals.var_ty_dn5), (assign91770_e140565 * locals.var_ty_dn6), (assign91770_e140565 * locals.var_ty_dn7), (assign91770_e140565 * locals.var_ty_dn8), (assign91770_e140565 * locals.var_ty_dn9), (assign91770_e140565 * locals.var_ty_dn10), (assign91770_e140565 * locals.var_ty_dn11), (assign91770_e140565 * locals.var_ty_dn14),)
    } else {
        (locals.var_ac41, locals.var_ac41_dn0, locals.var_ac41_dn2, locals.var_ac41_dn4, locals.var_ac41_dn5, locals.var_ac41_dn6, locals.var_ac41_dn7, locals.var_ac41_dn8, locals.var_ac41_dn9, locals.var_ac41_dn10, locals.var_ac41_dn11, locals.var_ac41_dn14,)
    }
};
        locals.var_ac41 = assign91770_e140570;
        locals.var_ac41_dn0 = assign91770_e140570_d_n0;
        locals.var_ac41_dn2 = assign91770_e140570_d_n2;
        locals.var_ac41_dn4 = assign91770_e140570_d_n4;
        locals.var_ac41_dn5 = assign91770_e140570_d_n5;
        locals.var_ac41_dn6 = assign91770_e140570_d_n6;
        locals.var_ac41_dn7 = assign91770_e140570_d_n7;
        locals.var_ac41_dn8 = assign91770_e140570_d_n8;
        locals.var_ac41_dn9 = assign91770_e140570_d_n9;
        locals.var_ac41_dn10 = assign91770_e140570_d_n10;
        locals.var_ac41_dn11 = assign91770_e140570_d_n11;
        locals.var_ac41_dn14 = assign91770_e140570_d_n14;
        locals.var_ac41_rv = 0.0;

        let (assign91780_e140584, assign91780_e140584_d_n0, assign91780_e140584_d_n2, assign91780_e140584_d_n4, assign91780_e140584_d_n5, assign91780_e140584_d_n6, assign91780_e140584_d_n7, assign91780_e140584_d_n8, assign91780_e140584_d_n9, assign91780_e140584_d_n10, assign91780_e140584_d_n11, assign91780_e140584_d_n14,) = {
    if (((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 != 0.0)) {
        let assign91780_e140578: f64 = (8.0 * locals.var_ac41);
        let assign91780_e140580: f64 = (assign91780_e140578 * locals.var_ac41);
        let assign91780_e140582: f64 = (assign91780_e140580 * locals.var_ac41);
        (assign91780_e140582, (((((8.0 * locals.var_ac41_dn0) * locals.var_ac41) + (assign91780_e140578 * locals.var_ac41_dn0)) * locals.var_ac41) + (assign91780_e140580 * locals.var_ac41_dn0)), (((((8.0 * locals.var_ac41_dn2) * locals.var_ac41) + (assign91780_e140578 * locals.var_ac41_dn2)) * locals.var_ac41) + (assign91780_e140580 * locals.var_ac41_dn2)), (((((8.0 * locals.var_ac41_dn4) * locals.var_ac41) + (assign91780_e140578 * locals.var_ac41_dn4)) * locals.var_ac41) + (assign91780_e140580 * locals.var_ac41_dn4)), (((((8.0 * locals.var_ac41_dn5) * locals.var_ac41) + (assign91780_e140578 * locals.var_ac41_dn5)) * locals.var_ac41) + (assign91780_e140580 * locals.var_ac41_dn5)), (((((8.0 * locals.var_ac41_dn6) * locals.var_ac41) + (assign91780_e140578 * locals.var_ac41_dn6)) * locals.var_ac41) + (assign91780_e140580 * locals.var_ac41_dn6)), (((((8.0 * locals.var_ac41_dn7) * locals.var_ac41) + (assign91780_e140578 * locals.var_ac41_dn7)) * locals.var_ac41) + (assign91780_e140580 * locals.var_ac41_dn7)), (((((8.0 * locals.var_ac41_dn8) * locals.var_ac41) + (assign91780_e140578 * locals.var_ac41_dn8)) * locals.var_ac41) + (assign91780_e140580 * locals.var_ac41_dn8)), (((((8.0 * locals.var_ac41_dn9) * locals.var_ac41) + (assign91780_e140578 * locals.var_ac41_dn9)) * locals.var_ac41) + (assign91780_e140580 * locals.var_ac41_dn9)), (((((8.0 * locals.var_ac41_dn10) * locals.var_ac41) + (assign91780_e140578 * locals.var_ac41_dn10)) * locals.var_ac41) + (assign91780_e140580 * locals.var_ac41_dn10)), (((((8.0 * locals.var_ac41_dn11) * locals.var_ac41) + (assign91780_e140578 * locals.var_ac41_dn11)) * locals.var_ac41) + (assign91780_e140580 * locals.var_ac41_dn11)), (((((8.0 * locals.var_ac41_dn14) * locals.var_ac41) + (assign91780_e140578 * locals.var_ac41_dn14)) * locals.var_ac41) + (assign91780_e140580 * locals.var_ac41_dn14)),)
    } else {
        (locals.var_ac4, locals.var_ac4_dn0, locals.var_ac4_dn2, locals.var_ac4_dn4, locals.var_ac4_dn5, locals.var_ac4_dn6, locals.var_ac4_dn7, locals.var_ac4_dn8, locals.var_ac4_dn9, locals.var_ac4_dn10, locals.var_ac4_dn11, locals.var_ac4_dn14,)
    }
};
        locals.var_ac4 = assign91780_e140584;
        locals.var_ac4_dn0 = assign91780_e140584_d_n0;
        locals.var_ac4_dn2 = assign91780_e140584_d_n2;
        locals.var_ac4_dn4 = assign91780_e140584_d_n4;
        locals.var_ac4_dn5 = assign91780_e140584_d_n5;
        locals.var_ac4_dn6 = assign91780_e140584_d_n6;
        locals.var_ac4_dn7 = assign91780_e140584_d_n7;
        locals.var_ac4_dn8 = assign91780_e140584_d_n8;
        locals.var_ac4_dn9 = assign91780_e140584_d_n9;
        locals.var_ac4_dn10 = assign91780_e140584_d_n10;
        locals.var_ac4_dn11 = assign91780_e140584_d_n11;
        locals.var_ac4_dn14 = assign91780_e140584_d_n14;
        locals.var_ac4_rv = 0.0;

        let (assign91790_e140602, assign91790_e140602_d_n0, assign91790_e140602_d_n2, assign91790_e140602_d_n4, assign91790_e140602_d_n5, assign91790_e140602_d_n6, assign91790_e140602_d_n7, assign91790_e140602_d_n8, assign91790_e140602_d_n9, assign91790_e140602_d_n10, assign91790_e140602_d_n11, assign91790_e140602_d_n14,) = {
    if (((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 != 0.0)) {
        let assign91790_e140592: f64 = (7.0 * 1.414213562373095);
        let assign91790_e140595: f64 = (9.0 * locals.var_ty);
        let assign91790_e140598: f64 = (locals.var_tx - 2.0);
        let assign91790_e140599: f64 = (assign91790_e140595 * assign91790_e140598);
        let assign91790_e140600: f64 = (assign91790_e140592 - assign91790_e140599);
        (assign91790_e140600, (-(((9.0 * locals.var_ty_dn0) * assign91790_e140598) + (assign91790_e140595 * locals.var_tx_dn0))), (-(((9.0 * locals.var_ty_dn2) * assign91790_e140598) + (assign91790_e140595 * locals.var_tx_dn2))), (-(((9.0 * locals.var_ty_dn4) * assign91790_e140598) + (assign91790_e140595 * locals.var_tx_dn4))), (-(((9.0 * locals.var_ty_dn5) * assign91790_e140598) + (assign91790_e140595 * locals.var_tx_dn5))), (-(((9.0 * locals.var_ty_dn6) * assign91790_e140598) + (assign91790_e140595 * locals.var_tx_dn6))), (-(((9.0 * locals.var_ty_dn7) * assign91790_e140598) + (assign91790_e140595 * locals.var_tx_dn7))), (-(((9.0 * locals.var_ty_dn8) * assign91790_e140598) + (assign91790_e140595 * locals.var_tx_dn8))), (-(((9.0 * locals.var_ty_dn9) * assign91790_e140598) + (assign91790_e140595 * locals.var_tx_dn9))), (-(((9.0 * locals.var_ty_dn10) * assign91790_e140598) + (assign91790_e140595 * locals.var_tx_dn10))), (-(((9.0 * locals.var_ty_dn11) * assign91790_e140598) + (assign91790_e140595 * locals.var_tx_dn11))), (-(((9.0 * locals.var_ty_dn14) * assign91790_e140598) + (assign91790_e140595 * locals.var_tx_dn14))),)
    } else {
        (locals.var_ac31, locals.var_ac31_dn0, locals.var_ac31_dn2, locals.var_ac31_dn4, locals.var_ac31_dn5, locals.var_ac31_dn6, locals.var_ac31_dn7, locals.var_ac31_dn8, locals.var_ac31_dn9, locals.var_ac31_dn10, locals.var_ac31_dn11, locals.var_ac31_dn14,)
    }
};
        locals.var_ac31 = assign91790_e140602;
        locals.var_ac31_dn0 = assign91790_e140602_d_n0;
        locals.var_ac31_dn2 = assign91790_e140602_d_n2;
        locals.var_ac31_dn4 = assign91790_e140602_d_n4;
        locals.var_ac31_dn5 = assign91790_e140602_d_n5;
        locals.var_ac31_dn6 = assign91790_e140602_d_n6;
        locals.var_ac31_dn7 = assign91790_e140602_d_n7;
        locals.var_ac31_dn8 = assign91790_e140602_d_n8;
        locals.var_ac31_dn9 = assign91790_e140602_d_n9;
        locals.var_ac31_dn10 = assign91790_e140602_d_n10;
        locals.var_ac31_dn11 = assign91790_e140602_d_n11;
        locals.var_ac31_dn14 = assign91790_e140602_d_n14;
        locals.var_ac31_rv = 0.0;

        let (assign91800_e140612, assign91800_e140612_d_n0, assign91800_e140612_d_n2, assign91800_e140612_d_n4, assign91800_e140612_d_n5, assign91800_e140612_d_n6, assign91800_e140612_d_n7, assign91800_e140612_d_n8, assign91800_e140612_d_n9, assign91800_e140612_d_n10, assign91800_e140612_d_n11, assign91800_e140612_d_n14,) = {
    if (((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 != 0.0)) {
        let assign91800_e140610: f64 = (locals.var_ac31 * locals.var_ac31);
        (assign91800_e140610, ((locals.var_ac31_dn0 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn0)), ((locals.var_ac31_dn2 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn2)), ((locals.var_ac31_dn4 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn4)), ((locals.var_ac31_dn5 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn5)), ((locals.var_ac31_dn6 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn6)), ((locals.var_ac31_dn7 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn7)), ((locals.var_ac31_dn8 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn8)), ((locals.var_ac31_dn9 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn9)), ((locals.var_ac31_dn10 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn10)), ((locals.var_ac31_dn11 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn11)), ((locals.var_ac31_dn14 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn14)),)
    } else {
        (locals.var_ac3, locals.var_ac3_dn0, locals.var_ac3_dn2, locals.var_ac3_dn4, locals.var_ac3_dn5, locals.var_ac3_dn6, locals.var_ac3_dn7, locals.var_ac3_dn8, locals.var_ac3_dn9, locals.var_ac3_dn10, locals.var_ac3_dn11, locals.var_ac3_dn14,)
    }
};
        locals.var_ac3 = assign91800_e140612;
        locals.var_ac3_dn0 = assign91800_e140612_d_n0;
        locals.var_ac3_dn2 = assign91800_e140612_d_n2;
        locals.var_ac3_dn4 = assign91800_e140612_d_n4;
        locals.var_ac3_dn5 = assign91800_e140612_d_n5;
        locals.var_ac3_dn6 = assign91800_e140612_d_n6;
        locals.var_ac3_dn7 = assign91800_e140612_d_n7;
        locals.var_ac3_dn8 = assign91800_e140612_d_n8;
        locals.var_ac3_dn9 = assign91800_e140612_d_n9;
        locals.var_ac3_dn10 = assign91800_e140612_d_n10;
        locals.var_ac3_dn11 = assign91800_e140612_d_n11;
        locals.var_ac3_dn14 = assign91800_e140612_d_n14;
        locals.var_ac3_rv = 0.0;

        let assign91810_e140616: f64 = (locals.var_ac3 * 1e-8);
        let assign91810_e140617: f64 = if locals.var_ac4 < assign91810_e140616 { 1.0 } else { 0.0 };
        locals.var_guard2142 = assign91810_e140617;
        locals.var_guard2142_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_354(
        locals: &mut StampLocals,
    ) {
        let (assign91830_e140642, assign91830_e140642_d_n0, assign91830_e140642_d_n2, assign91830_e140642_d_n4, assign91830_e140642_d_n5, assign91830_e140642_d_n6, assign91830_e140642_d_n7, assign91830_e140642_d_n8, assign91830_e140642_d_n9, assign91830_e140642_d_n10, assign91830_e140642_d_n11, assign91830_e140642_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 != 0.0)) && (locals.var_guard2142 != 0.0)) {
        let assign91830_e140638: f64 = (0.5 * locals.var_ac4);
        let assign91830_e140640: f64 = (assign91830_e140638 / locals.var_ac31);
        (assign91830_e140640, ((((0.5 * locals.var_ac4_dn0) * locals.var_ac31) - (assign91830_e140638 * locals.var_ac31_dn0)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn2) * locals.var_ac31) - (assign91830_e140638 * locals.var_ac31_dn2)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn4) * locals.var_ac31) - (assign91830_e140638 * locals.var_ac31_dn4)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn5) * locals.var_ac31) - (assign91830_e140638 * locals.var_ac31_dn5)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn6) * locals.var_ac31) - (assign91830_e140638 * locals.var_ac31_dn6)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn7) * locals.var_ac31) - (assign91830_e140638 * locals.var_ac31_dn7)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn8) * locals.var_ac31) - (assign91830_e140638 * locals.var_ac31_dn8)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn9) * locals.var_ac31) - (assign91830_e140638 * locals.var_ac31_dn9)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn10) * locals.var_ac31) - (assign91830_e140638 * locals.var_ac31_dn10)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn11) * locals.var_ac31) - (assign91830_e140638 * locals.var_ac31_dn11)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn14) * locals.var_ac31) - (assign91830_e140638 * locals.var_ac31_dn14)) / (locals.var_ac31 * locals.var_ac31)),)
    } else {
        (locals.var_ac1, locals.var_ac1_dn0, locals.var_ac1_dn2, locals.var_ac1_dn4, locals.var_ac1_dn5, locals.var_ac1_dn6, locals.var_ac1_dn7, locals.var_ac1_dn8, locals.var_ac1_dn9, locals.var_ac1_dn10, locals.var_ac1_dn11, locals.var_ac1_dn14,)
    }
};
        locals.var_ac1 = assign91830_e140642;
        locals.var_ac1_dn0 = assign91830_e140642_d_n0;
        locals.var_ac1_dn2 = assign91830_e140642_d_n2;
        locals.var_ac1_dn4 = assign91830_e140642_d_n4;
        locals.var_ac1_dn5 = assign91830_e140642_d_n5;
        locals.var_ac1_dn6 = assign91830_e140642_d_n6;
        locals.var_ac1_dn7 = assign91830_e140642_d_n7;
        locals.var_ac1_dn8 = assign91830_e140642_d_n8;
        locals.var_ac1_dn9 = assign91830_e140642_d_n9;
        locals.var_ac1_dn10 = assign91830_e140642_d_n10;
        locals.var_ac1_dn11 = assign91830_e140642_d_n11;
        locals.var_ac1_dn14 = assign91830_e140642_d_n14;
        locals.var_ac1_rv = 0.0;

        let (assign91840_e140656, assign91840_e140656_d_n0, assign91840_e140656_d_n2, assign91840_e140656_d_n4, assign91840_e140656_d_n5, assign91840_e140656_d_n6, assign91840_e140656_d_n7, assign91840_e140656_d_n8, assign91840_e140656_d_n9, assign91840_e140656_d_n10, assign91840_e140656_d_n11, assign91840_e140656_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 != 0.0)) && (locals.var_guard2142 == 0.0)) {
        let assign91840_e140653: f64 = (locals.var_ac4 + locals.var_ac3);
        let assign91840_e140654: f64 = (assign91840_e140653).sqrt();
        (assign91840_e140654, ((locals.var_ac4_dn0 + locals.var_ac3_dn0) / (2.0 * assign91840_e140654)), ((locals.var_ac4_dn2 + locals.var_ac3_dn2) / (2.0 * assign91840_e140654)), ((locals.var_ac4_dn4 + locals.var_ac3_dn4) / (2.0 * assign91840_e140654)), ((locals.var_ac4_dn5 + locals.var_ac3_dn5) / (2.0 * assign91840_e140654)), ((locals.var_ac4_dn6 + locals.var_ac3_dn6) / (2.0 * assign91840_e140654)), ((locals.var_ac4_dn7 + locals.var_ac3_dn7) / (2.0 * assign91840_e140654)), ((locals.var_ac4_dn8 + locals.var_ac3_dn8) / (2.0 * assign91840_e140654)), ((locals.var_ac4_dn9 + locals.var_ac3_dn9) / (2.0 * assign91840_e140654)), ((locals.var_ac4_dn10 + locals.var_ac3_dn10) / (2.0 * assign91840_e140654)), ((locals.var_ac4_dn11 + locals.var_ac3_dn11) / (2.0 * assign91840_e140654)), ((locals.var_ac4_dn14 + locals.var_ac3_dn14) / (2.0 * assign91840_e140654)),)
    } else {
        (locals.var_ac2, locals.var_ac2_dn0, locals.var_ac2_dn2, locals.var_ac2_dn4, locals.var_ac2_dn5, locals.var_ac2_dn6, locals.var_ac2_dn7, locals.var_ac2_dn8, locals.var_ac2_dn9, locals.var_ac2_dn10, locals.var_ac2_dn11, locals.var_ac2_dn14,)
    }
};
        locals.var_ac2 = assign91840_e140656;
        locals.var_ac2_dn0 = assign91840_e140656_d_n0;
        locals.var_ac2_dn2 = assign91840_e140656_d_n2;
        locals.var_ac2_dn4 = assign91840_e140656_d_n4;
        locals.var_ac2_dn5 = assign91840_e140656_d_n5;
        locals.var_ac2_dn6 = assign91840_e140656_d_n6;
        locals.var_ac2_dn7 = assign91840_e140656_d_n7;
        locals.var_ac2_dn8 = assign91840_e140656_d_n8;
        locals.var_ac2_dn9 = assign91840_e140656_d_n9;
        locals.var_ac2_dn10 = assign91840_e140656_d_n10;
        locals.var_ac2_dn11 = assign91840_e140656_d_n11;
        locals.var_ac2_dn14 = assign91840_e140656_d_n14;
        locals.var_ac2_rv = 0.0;

        let (assign91850_e140670, assign91850_e140670_d_n0, assign91850_e140670_d_n2, assign91850_e140670_d_n4, assign91850_e140670_d_n5, assign91850_e140670_d_n6, assign91850_e140670_d_n7, assign91850_e140670_d_n8, assign91850_e140670_d_n9, assign91850_e140670_d_n10, assign91850_e140670_d_n11, assign91850_e140670_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 != 0.0)) && (locals.var_guard2142 == 0.0)) {
        let assign91850_e140666: f64 = (-locals.var_ac31);
        let assign91850_e140668: f64 = (assign91850_e140666 + locals.var_ac2);
        (assign91850_e140668, ((-locals.var_ac31_dn0) + locals.var_ac2_dn0), ((-locals.var_ac31_dn2) + locals.var_ac2_dn2), ((-locals.var_ac31_dn4) + locals.var_ac2_dn4), ((-locals.var_ac31_dn5) + locals.var_ac2_dn5), ((-locals.var_ac31_dn6) + locals.var_ac2_dn6), ((-locals.var_ac31_dn7) + locals.var_ac2_dn7), ((-locals.var_ac31_dn8) + locals.var_ac2_dn8), ((-locals.var_ac31_dn9) + locals.var_ac2_dn9), ((-locals.var_ac31_dn10) + locals.var_ac2_dn10), ((-locals.var_ac31_dn11) + locals.var_ac2_dn11), ((-locals.var_ac31_dn14) + locals.var_ac2_dn14),)
    } else {
        (locals.var_ac1, locals.var_ac1_dn0, locals.var_ac1_dn2, locals.var_ac1_dn4, locals.var_ac1_dn5, locals.var_ac1_dn6, locals.var_ac1_dn7, locals.var_ac1_dn8, locals.var_ac1_dn9, locals.var_ac1_dn10, locals.var_ac1_dn11, locals.var_ac1_dn14,)
    }
};
        locals.var_ac1 = assign91850_e140670;
        locals.var_ac1_dn0 = assign91850_e140670_d_n0;
        locals.var_ac1_dn2 = assign91850_e140670_d_n2;
        locals.var_ac1_dn4 = assign91850_e140670_d_n4;
        locals.var_ac1_dn5 = assign91850_e140670_d_n5;
        locals.var_ac1_dn6 = assign91850_e140670_d_n6;
        locals.var_ac1_dn7 = assign91850_e140670_d_n7;
        locals.var_ac1_dn8 = assign91850_e140670_d_n8;
        locals.var_ac1_dn9 = assign91850_e140670_d_n9;
        locals.var_ac1_dn10 = assign91850_e140670_d_n10;
        locals.var_ac1_dn11 = assign91850_e140670_d_n11;
        locals.var_ac1_dn14 = assign91850_e140670_d_n14;
        locals.var_ac1_rv = 0.0;

        let (assign91860_e140680, assign91860_e140680_d_n0, assign91860_e140680_d_n2, assign91860_e140680_d_n4, assign91860_e140680_d_n5, assign91860_e140680_d_n6, assign91860_e140680_d_n7, assign91860_e140680_d_n8, assign91860_e140680_d_n9, assign91860_e140680_d_n10, assign91860_e140680_d_n11, assign91860_e140680_d_n14,) = {
    if (((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 != 0.0)) {
        let assign91860_e140678: f64 = (locals.var_ac1).powf(0.3333333333333333);
        (assign91860_e140678, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn0)) } } else { (assign91860_e140678 * (0.3333333333333333 * (locals.var_ac1_dn0 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn2)) } } else { (assign91860_e140678 * (0.3333333333333333 * (locals.var_ac1_dn2 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn4)) } } else { (assign91860_e140678 * (0.3333333333333333 * (locals.var_ac1_dn4 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn5)) } } else { (assign91860_e140678 * (0.3333333333333333 * (locals.var_ac1_dn5 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn6)) } } else { (assign91860_e140678 * (0.3333333333333333 * (locals.var_ac1_dn6 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn7)) } } else { (assign91860_e140678 * (0.3333333333333333 * (locals.var_ac1_dn7 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn8)) } } else { (assign91860_e140678 * (0.3333333333333333 * (locals.var_ac1_dn8 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn9)) } } else { (assign91860_e140678 * (0.3333333333333333 * (locals.var_ac1_dn9 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn10)) } } else { (assign91860_e140678 * (0.3333333333333333 * (locals.var_ac1_dn10 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn11)) } } else { (assign91860_e140678 * (0.3333333333333333 * (locals.var_ac1_dn11 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn14)) } } else { (assign91860_e140678 * (0.3333333333333333 * (locals.var_ac1_dn14 / locals.var_ac1))) },)
    } else {
        (locals.var_acd, locals.var_acd_dn0, locals.var_acd_dn2, locals.var_acd_dn4, locals.var_acd_dn5, locals.var_acd_dn6, locals.var_acd_dn7, locals.var_acd_dn8, locals.var_acd_dn9, locals.var_acd_dn10, locals.var_acd_dn11, locals.var_acd_dn14,)
    }
};
        locals.var_acd = assign91860_e140680;
        locals.var_acd_dn0 = assign91860_e140680_d_n0;
        locals.var_acd_dn2 = assign91860_e140680_d_n2;
        locals.var_acd_dn4 = assign91860_e140680_d_n4;
        locals.var_acd_dn5 = assign91860_e140680_d_n5;
        locals.var_acd_dn6 = assign91860_e140680_d_n6;
        locals.var_acd_dn7 = assign91860_e140680_d_n7;
        locals.var_acd_dn8 = assign91860_e140680_d_n8;
        locals.var_acd_dn9 = assign91860_e140680_d_n9;
        locals.var_acd_dn10 = assign91860_e140680_d_n10;
        locals.var_acd_dn11 = assign91860_e140680_d_n11;
        locals.var_acd_dn14 = assign91860_e140680_d_n14;
        locals.var_acd_rv = 0.0;

        let (assign91870_e140705, assign91870_e140705_d_n0, assign91870_e140705_d_n2, assign91870_e140705_d_n4, assign91870_e140705_d_n5, assign91870_e140705_d_n6, assign91870_e140705_d_n7, assign91870_e140705_d_n8, assign91870_e140705_d_n9, assign91870_e140705_d_n10, assign91870_e140705_d_n11, assign91870_e140705_d_n14,) = {
    if (((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 != 0.0)) {
        let assign91870_e140687: f64 = (-4.0);
        let assign91870_e140689: f64 = (assign91870_e140687 * 1.414213562373095);
        let assign91870_e140692: f64 = (12.0 * locals.var_ty);
        let assign91870_e140693: f64 = (assign91870_e140689 - assign91870_e140692);
        let assign91870_e140696: f64 = (2.0 * locals.var_acd);
        let assign91870_e140697: f64 = (assign91870_e140693 + assign91870_e140696);
        let assign91870_e140700: f64 = (1.414213562373095 * locals.var_acd);
        let assign91870_e140702: f64 = (assign91870_e140700 * locals.var_acd);
        let assign91870_e140703: f64 = (assign91870_e140697 + assign91870_e140702);
        (assign91870_e140703, (((-(12.0 * locals.var_ty_dn0)) + (2.0 * locals.var_acd_dn0)) + (((1.414213562373095 * locals.var_acd_dn0) * locals.var_acd) + (assign91870_e140700 * locals.var_acd_dn0))), (((-(12.0 * locals.var_ty_dn2)) + (2.0 * locals.var_acd_dn2)) + (((1.414213562373095 * locals.var_acd_dn2) * locals.var_acd) + (assign91870_e140700 * locals.var_acd_dn2))), (((-(12.0 * locals.var_ty_dn4)) + (2.0 * locals.var_acd_dn4)) + (((1.414213562373095 * locals.var_acd_dn4) * locals.var_acd) + (assign91870_e140700 * locals.var_acd_dn4))), (((-(12.0 * locals.var_ty_dn5)) + (2.0 * locals.var_acd_dn5)) + (((1.414213562373095 * locals.var_acd_dn5) * locals.var_acd) + (assign91870_e140700 * locals.var_acd_dn5))), (((-(12.0 * locals.var_ty_dn6)) + (2.0 * locals.var_acd_dn6)) + (((1.414213562373095 * locals.var_acd_dn6) * locals.var_acd) + (assign91870_e140700 * locals.var_acd_dn6))), (((-(12.0 * locals.var_ty_dn7)) + (2.0 * locals.var_acd_dn7)) + (((1.414213562373095 * locals.var_acd_dn7) * locals.var_acd) + (assign91870_e140700 * locals.var_acd_dn7))), (((-(12.0 * locals.var_ty_dn8)) + (2.0 * locals.var_acd_dn8)) + (((1.414213562373095 * locals.var_acd_dn8) * locals.var_acd) + (assign91870_e140700 * locals.var_acd_dn8))), (((-(12.0 * locals.var_ty_dn9)) + (2.0 * locals.var_acd_dn9)) + (((1.414213562373095 * locals.var_acd_dn9) * locals.var_acd) + (assign91870_e140700 * locals.var_acd_dn9))), (((-(12.0 * locals.var_ty_dn10)) + (2.0 * locals.var_acd_dn10)) + (((1.414213562373095 * locals.var_acd_dn10) * locals.var_acd) + (assign91870_e140700 * locals.var_acd_dn10))), (((-(12.0 * locals.var_ty_dn11)) + (2.0 * locals.var_acd_dn11)) + (((1.414213562373095 * locals.var_acd_dn11) * locals.var_acd) + (assign91870_e140700 * locals.var_acd_dn11))), (((-(12.0 * locals.var_ty_dn14)) + (2.0 * locals.var_acd_dn14)) + (((1.414213562373095 * locals.var_acd_dn14) * locals.var_acd) + (assign91870_e140700 * locals.var_acd_dn14))),)
    } else {
        (locals.var_acn, locals.var_acn_dn0, locals.var_acn_dn2, locals.var_acn_dn4, locals.var_acn_dn5, locals.var_acn_dn6, locals.var_acn_dn7, locals.var_acn_dn8, locals.var_acn_dn9, locals.var_acn_dn10, locals.var_acn_dn11, locals.var_acn_dn14,)
    }
};
        locals.var_acn = assign91870_e140705;
        locals.var_acn_dn0 = assign91870_e140705_d_n0;
        locals.var_acn_dn2 = assign91870_e140705_d_n2;
        locals.var_acn_dn4 = assign91870_e140705_d_n4;
        locals.var_acn_dn5 = assign91870_e140705_d_n5;
        locals.var_acn_dn6 = assign91870_e140705_d_n6;
        locals.var_acn_dn7 = assign91870_e140705_d_n7;
        locals.var_acn_dn8 = assign91870_e140705_d_n8;
        locals.var_acn_dn9 = assign91870_e140705_d_n9;
        locals.var_acn_dn10 = assign91870_e140705_d_n10;
        locals.var_acn_dn11 = assign91870_e140705_d_n11;
        locals.var_acn_dn14 = assign91870_e140705_d_n14;
        locals.var_acn_rv = 0.0;

        let (assign91880_e140715, assign91880_e140715_d_n0, assign91880_e140715_d_n2, assign91880_e140715_d_n4, assign91880_e140715_d_n5, assign91880_e140715_d_n6, assign91880_e140715_d_n7, assign91880_e140715_d_n8, assign91880_e140715_d_n9, assign91880_e140715_d_n10, assign91880_e140715_d_n11, assign91880_e140715_d_n14,) = {
    if (((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 != 0.0)) {
        let assign91880_e140713: f64 = (locals.var_acn / locals.var_acd);
        (assign91880_e140713, (((locals.var_acn_dn0 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn0)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn2 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn2)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn4 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn4)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn5 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn5)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn6 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn6)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn7 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn7)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn8 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn8)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn9 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn9)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn10 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn10)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn11 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn11)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn14 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn14)) / (locals.var_acd * locals.var_acd)),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
        locals.var_chi = assign91880_e140715;
        locals.var_chi_dn0 = assign91880_e140715_d_n0;
        locals.var_chi_dn2 = assign91880_e140715_d_n2;
        locals.var_chi_dn4 = assign91880_e140715_d_n4;
        locals.var_chi_dn5 = assign91880_e140715_d_n5;
        locals.var_chi_dn6 = assign91880_e140715_d_n6;
        locals.var_chi_dn7 = assign91880_e140715_d_n7;
        locals.var_chi_dn8 = assign91880_e140715_d_n8;
        locals.var_chi_dn9 = assign91880_e140715_d_n9;
        locals.var_chi_dn10 = assign91880_e140715_d_n10;
        locals.var_chi_dn11 = assign91880_e140715_d_n11;
        locals.var_chi_dn14 = assign91880_e140715_d_n14;
        locals.var_chi_rv = 0.0;

        let (assign91890_e140725, assign91890_e140725_d_n0, assign91890_e140725_d_n2, assign91890_e140725_d_n4, assign91890_e140725_d_n5, assign91890_e140725_d_n6, assign91890_e140725_d_n7, assign91890_e140725_d_n8, assign91890_e140725_d_n9, assign91890_e140725_d_n10, assign91890_e140725_d_n11, assign91890_e140725_d_n14,) = {
    if (((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 != 0.0)) {
        let assign91890_e140723: f64 = (locals.var_chi * locals.var_beta_inv);
        (assign91890_e140723, ((locals.var_chi_dn0 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn0)), ((locals.var_chi_dn2 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn2)), ((locals.var_chi_dn4 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn4)), ((locals.var_chi_dn5 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn5)), ((locals.var_chi_dn6 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn6)), ((locals.var_chi_dn7 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn7)), ((locals.var_chi_dn8 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn8)), ((locals.var_chi_dn9 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn9)), ((locals.var_chi_dn10 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn10)), ((locals.var_chi_dn11 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn11)), ((locals.var_chi_dn14 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign91890_e140725;
        locals.var_t1_dn0 = assign91890_e140725_d_n0;
        locals.var_t1_dn2 = assign91890_e140725_d_n2;
        locals.var_t1_dn4 = assign91890_e140725_d_n4;
        locals.var_t1_dn5 = assign91890_e140725_d_n5;
        locals.var_t1_dn6 = assign91890_e140725_d_n6;
        locals.var_t1_dn7 = assign91890_e140725_d_n7;
        locals.var_t1_dn8 = assign91890_e140725_d_n8;
        locals.var_t1_dn9 = assign91890_e140725_d_n9;
        locals.var_t1_dn10 = assign91890_e140725_d_n10;
        locals.var_t1_dn11 = assign91890_e140725_d_n11;
        locals.var_t1_dn14 = assign91890_e140725_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign91900_e140735, assign91900_e140735_d_n0, assign91900_e140735_d_n2, assign91900_e140735_d_n4, assign91900_e140735_d_n5, assign91900_e140735_d_n6, assign91900_e140735_d_n7, assign91900_e140735_d_n8, assign91900_e140735_d_n9, assign91900_e140735_d_n10, assign91900_e140735_d_n11, assign91900_e140735_d_n14,) = {
    if (((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 != 0.0)) {
        let assign91900_e140733: f64 = (locals.var_t1 / locals.var_ps0_min);
        (assign91900_e140733, (((locals.var_t1_dn0 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn0)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn2 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn2)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn4 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn4)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn5 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn5)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn6 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn6)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn7 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn7)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn8 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn8)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn9 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn9)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn10 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn10)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn11 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn11)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn14 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn14)) / (locals.var_ps0_min * locals.var_ps0_min)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign91900_e140735;
        locals.var_t2_dn0 = assign91900_e140735_d_n0;
        locals.var_t2_dn2 = assign91900_e140735_d_n2;
        locals.var_t2_dn4 = assign91900_e140735_d_n4;
        locals.var_t2_dn5 = assign91900_e140735_d_n5;
        locals.var_t2_dn6 = assign91900_e140735_d_n6;
        locals.var_t2_dn7 = assign91900_e140735_d_n7;
        locals.var_t2_dn8 = assign91900_e140735_d_n8;
        locals.var_t2_dn9 = assign91900_e140735_d_n9;
        locals.var_t2_dn10 = assign91900_e140735_d_n10;
        locals.var_t2_dn11 = assign91900_e140735_d_n11;
        locals.var_t2_dn14 = assign91900_e140735_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign91910_e140748, assign91910_e140748_d_n0, assign91910_e140748_d_n2, assign91910_e140748_d_n4, assign91910_e140748_d_n5, assign91910_e140748_d_n6, assign91910_e140748_d_n7, assign91910_e140748_d_n8, assign91910_e140748_d_n9, assign91910_e140748_d_n10, assign91910_e140748_d_n11, assign91910_e140748_d_n14,) = {
    if (((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 != 0.0)) {
        let assign91910_e140744: f64 = (locals.var_t2 * locals.var_t2);
        let assign91910_e140745: f64 = (1.0 + assign91910_e140744);
        let assign91910_e140746: f64 = (assign91910_e140745).sqrt();
        (assign91910_e140746, (((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)) / (2.0 * assign91910_e140746)), (((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)) / (2.0 * assign91910_e140746)), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign91910_e140746)), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign91910_e140746)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign91910_e140746)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign91910_e140746)), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign91910_e140746)), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign91910_e140746)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign91910_e140746)), (((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign91910_e140746)), (((locals.var_t2_dn14 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn14)) / (2.0 * assign91910_e140746)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign91910_e140748;
        locals.var_t3_dn0 = assign91910_e140748_d_n0;
        locals.var_t3_dn2 = assign91910_e140748_d_n2;
        locals.var_t3_dn4 = assign91910_e140748_d_n4;
        locals.var_t3_dn5 = assign91910_e140748_d_n5;
        locals.var_t3_dn6 = assign91910_e140748_d_n6;
        locals.var_t3_dn7 = assign91910_e140748_d_n7;
        locals.var_t3_dn8 = assign91910_e140748_d_n8;
        locals.var_t3_dn9 = assign91910_e140748_d_n9;
        locals.var_t3_dn10 = assign91910_e140748_d_n10;
        locals.var_t3_dn11 = assign91910_e140748_d_n11;
        locals.var_t3_dn14 = assign91910_e140748_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign91920_e140760, assign91920_e140760_d_n0, assign91920_e140760_d_n2, assign91920_e140760_d_n4, assign91920_e140760_d_n5, assign91920_e140760_d_n6, assign91920_e140760_d_n7, assign91920_e140760_d_n8, assign91920_e140760_d_n9, assign91920_e140760_d_n10, assign91920_e140760_d_n11, assign91920_e140760_d_n14,) = {
    if (((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 != 0.0)) {
        let assign91920_e140756: f64 = (locals.var_t1 / locals.var_t3);
        let assign91920_e140758: f64 = (assign91920_e140756 - locals.var_vxbgmtcl);
        (assign91920_e140758, ((((locals.var_t1_dn0 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn0)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn0), ((((locals.var_t1_dn2 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn2)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn2), ((((locals.var_t1_dn4 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn4)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn4), ((((locals.var_t1_dn5 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn5)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn5), ((((locals.var_t1_dn6 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn6)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn6), ((((locals.var_t1_dn7 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn7)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn7), ((((locals.var_t1_dn8 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn8)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn8), ((((locals.var_t1_dn9 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn9)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn9), ((((locals.var_t1_dn10 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn10)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn10), ((((locals.var_t1_dn11 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn11)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn11), ((((locals.var_t1_dn14 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn14)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn14),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn14,)
    }
};
        locals.var_ps0ld = assign91920_e140760;
        locals.var_ps0ld_dn0 = assign91920_e140760_d_n0;
        locals.var_ps0ld_dn2 = assign91920_e140760_d_n2;
        locals.var_ps0ld_dn4 = assign91920_e140760_d_n4;
        locals.var_ps0ld_dn5 = assign91920_e140760_d_n5;
        locals.var_ps0ld_dn6 = assign91920_e140760_d_n6;
        locals.var_ps0ld_dn7 = assign91920_e140760_d_n7;
        locals.var_ps0ld_dn8 = assign91920_e140760_d_n8;
        locals.var_ps0ld_dn9 = assign91920_e140760_d_n9;
        locals.var_ps0ld_dn10 = assign91920_e140760_d_n10;
        locals.var_ps0ld_dn11 = assign91920_e140760_d_n11;
        locals.var_ps0ld_dn14 = assign91920_e140760_d_n14;
        locals.var_ps0ld_rv = 0.0;

        let (assign91930_e140770, assign91930_e140770_d_n0, assign91930_e140770_d_n2, assign91930_e140770_d_n4, assign91930_e140770_d_n5, assign91930_e140770_d_n6, assign91930_e140770_d_n7, assign91930_e140770_d_n8, assign91930_e140770_d_n9, assign91930_e140770_d_n10, assign91930_e140770_d_n11, assign91930_e140770_d_n14,) = {
    if (((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 != 0.0)) {
        let assign91930_e140768: f64 = (locals.var_vgpld - locals.var_ps0ld);
        (assign91930_e140768, (-locals.var_ps0ld_dn0), (locals.var_vgpld_dn2 - locals.var_ps0ld_dn2), (-locals.var_ps0ld_dn4), (-locals.var_ps0ld_dn5), (-locals.var_ps0ld_dn6), (locals.var_vgpld_dn7 - locals.var_ps0ld_dn7), (locals.var_vgpld_dn8 - locals.var_ps0ld_dn8), (locals.var_vgpld_dn9 - locals.var_ps0ld_dn9), (-locals.var_ps0ld_dn10), (-locals.var_ps0ld_dn11), (-locals.var_ps0ld_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign91930_e140770;
        locals.var_t2_dn0 = assign91930_e140770_d_n0;
        locals.var_t2_dn2 = assign91930_e140770_d_n2;
        locals.var_t2_dn4 = assign91930_e140770_d_n4;
        locals.var_t2_dn5 = assign91930_e140770_d_n5;
        locals.var_t2_dn6 = assign91930_e140770_d_n6;
        locals.var_t2_dn7 = assign91930_e140770_d_n7;
        locals.var_t2_dn8 = assign91930_e140770_d_n8;
        locals.var_t2_dn9 = assign91930_e140770_d_n9;
        locals.var_t2_dn10 = assign91930_e140770_d_n10;
        locals.var_t2_dn11 = assign91930_e140770_d_n11;
        locals.var_t2_dn14 = assign91930_e140770_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign91940_e140780, assign91940_e140780_d_n0, assign91940_e140780_d_n2, assign91940_e140780_d_n4, assign91940_e140780_d_n5, assign91940_e140780_d_n6, assign91940_e140780_d_n7, assign91940_e140780_d_n8, assign91940_e140780_d_n9, assign91940_e140780_d_n10, assign91940_e140780_d_n11, assign91940_e140780_d_n14,) = {
    if (((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 != 0.0)) {
        let assign91940_e140778: f64 = (locals.var_cox0_func * locals.var_t2);
        (assign91940_e140778, (locals.var_cox0_func * locals.var_t2_dn0), (locals.var_cox0_func * locals.var_t2_dn2), (locals.var_cox0_func * locals.var_t2_dn4), (locals.var_cox0_func * locals.var_t2_dn5), (locals.var_cox0_func * locals.var_t2_dn6), (locals.var_cox0_func * locals.var_t2_dn7), (locals.var_cox0_func * locals.var_t2_dn8), (locals.var_cox0_func * locals.var_t2_dn9), (locals.var_cox0_func * locals.var_t2_dn10), (locals.var_cox0_func * locals.var_t2_dn11), (locals.var_cox0_func * locals.var_t2_dn14),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn4, locals.var_qsuld_dn5, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn8, locals.var_qsuld_dn9, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn14,)
    }
};
        locals.var_qsuld = assign91940_e140780;
        locals.var_qsuld_dn0 = assign91940_e140780_d_n0;
        locals.var_qsuld_dn2 = assign91940_e140780_d_n2;
        locals.var_qsuld_dn4 = assign91940_e140780_d_n4;
        locals.var_qsuld_dn5 = assign91940_e140780_d_n5;
        locals.var_qsuld_dn6 = assign91940_e140780_d_n6;
        locals.var_qsuld_dn7 = assign91940_e140780_d_n7;
        locals.var_qsuld_dn8 = assign91940_e140780_d_n8;
        locals.var_qsuld_dn9 = assign91940_e140780_d_n9;
        locals.var_qsuld_dn10 = assign91940_e140780_d_n10;
        locals.var_qsuld_dn11 = assign91940_e140780_d_n11;
        locals.var_qsuld_dn14 = assign91940_e140780_d_n14;
        locals.var_qsuld_rv = 0.0;

        let (assign91950_e140788, assign91950_e140788_d_n0, assign91950_e140788_d_n2, assign91950_e140788_d_n4, assign91950_e140788_d_n5, assign91950_e140788_d_n6, assign91950_e140788_d_n7, assign91950_e140788_d_n8, assign91950_e140788_d_n9, assign91950_e140788_d_n10, assign91950_e140788_d_n11, assign91950_e140788_d_n14,) = {
    if (((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 != 0.0)) {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn4, locals.var_qsuld_dn5, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn8, locals.var_qsuld_dn9, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn14,)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn8, locals.var_qbuld_dn9, locals.var_qbuld_dn10, locals.var_qbuld_dn11, locals.var_qbuld_dn14,)
    }
};
        locals.var_qbuld = assign91950_e140788;
        locals.var_qbuld_dn0 = assign91950_e140788_d_n0;
        locals.var_qbuld_dn2 = assign91950_e140788_d_n2;
        locals.var_qbuld_dn4 = assign91950_e140788_d_n4;
        locals.var_qbuld_dn5 = assign91950_e140788_d_n5;
        locals.var_qbuld_dn6 = assign91950_e140788_d_n6;
        locals.var_qbuld_dn7 = assign91950_e140788_d_n7;
        locals.var_qbuld_dn8 = assign91950_e140788_d_n8;
        locals.var_qbuld_dn9 = assign91950_e140788_d_n9;
        locals.var_qbuld_dn10 = assign91950_e140788_d_n10;
        locals.var_qbuld_dn11 = assign91950_e140788_d_n11;
        locals.var_qbuld_dn14 = assign91950_e140788_d_n14;
        locals.var_qbuld_rv = 0.0;

        let (assign91960_e140796, assign91960_e140796_d_n0, assign91960_e140796_d_n2, assign91960_e140796_d_n4, assign91960_e140796_d_n5, assign91960_e140796_d_n6, assign91960_e140796_d_n7, assign91960_e140796_d_n8, assign91960_e140796_d_n9, assign91960_e140796_d_n10, assign91960_e140796_d_n11, assign91960_e140796_d_n14,) = {
    if (((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 != 0.0)) {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn14,)
    } else {
        (locals.var_ps0ld_ini__blk2124, locals.var_ps0ld_ini__blk2124_dn0, locals.var_ps0ld_ini__blk2124_dn2, locals.var_ps0ld_ini__blk2124_dn4, locals.var_ps0ld_ini__blk2124_dn5, locals.var_ps0ld_ini__blk2124_dn6, locals.var_ps0ld_ini__blk2124_dn7, locals.var_ps0ld_ini__blk2124_dn8, locals.var_ps0ld_ini__blk2124_dn9, locals.var_ps0ld_ini__blk2124_dn10, locals.var_ps0ld_ini__blk2124_dn11, locals.var_ps0ld_ini__blk2124_dn14,)
    }
};
        locals.var_ps0ld_ini__blk2124 = assign91960_e140796;
        locals.var_ps0ld_ini__blk2124_dn0 = assign91960_e140796_d_n0;
        locals.var_ps0ld_ini__blk2124_dn2 = assign91960_e140796_d_n2;
        locals.var_ps0ld_ini__blk2124_dn4 = assign91960_e140796_d_n4;
        locals.var_ps0ld_ini__blk2124_dn5 = assign91960_e140796_d_n5;
        locals.var_ps0ld_ini__blk2124_dn6 = assign91960_e140796_d_n6;
        locals.var_ps0ld_ini__blk2124_dn7 = assign91960_e140796_d_n7;
        locals.var_ps0ld_ini__blk2124_dn8 = assign91960_e140796_d_n8;
        locals.var_ps0ld_ini__blk2124_dn9 = assign91960_e140796_d_n9;
        locals.var_ps0ld_ini__blk2124_dn10 = assign91960_e140796_d_n10;
        locals.var_ps0ld_ini__blk2124_dn11 = assign91960_e140796_d_n11;
        locals.var_ps0ld_ini__blk2124_dn14 = assign91960_e140796_d_n14;
        locals.var_ps0ld_ini__blk2124_rv = 0.0;

        let assign91970_e140800: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign91970_e140801: f64 = (locals.var_beta * assign91970_e140800);
        let assign91970_e140805: f64 = (10.0 * 2.220446049250313e-16);
        let assign91970_e140807: f64 = (assign91970_e140805 - 1.0);
        let assign91970_e140809: f64 = (assign91970_e140807 * locals.var_fac1p2);
        let assign91970_e140811: f64 = (assign91970_e140809 * locals.var_beta2);
        let assign91970_e140813: f64 = (assign91970_e140811 / 4.0);
        let assign91970_e140814: f64 = (1.0 + assign91970_e140813);
        let assign91970_e140815: f64 = if assign91970_e140801 < assign91970_e140814 { 1.0 } else { 0.0 };
        locals.var_guard2143 = assign91970_e140815;
        locals.var_guard2143_rv = 0.0;

        let (assign91980_e140832, assign91980_e140832_d_n0, assign91980_e140832_d_n2, assign91980_e140832_d_n4, assign91980_e140832_d_n5, assign91980_e140832_d_n6, assign91980_e140832_d_n7, assign91980_e140832_d_n8, assign91980_e140832_d_n9, assign91980_e140832_d_n10, assign91980_e140832_d_n11, assign91980_e140832_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2143 != 0.0)) {
        let assign91980_e140827: f64 = (locals.var_fac1p2 * locals.var_beta);
        let assign91980_e140829: f64 = (assign91980_e140827 / 2.0);
        let assign91980_e140830: f64 = (locals.var_vgpld + assign91980_e140829);
        (assign91980_e140830, (((locals.var_fac1p2_dn0 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn0)) / 2.0), (locals.var_vgpld_dn2 + (((locals.var_fac1p2_dn2 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn2)) / 2.0)), (((locals.var_fac1p2_dn4 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn4)) / 2.0), (((locals.var_fac1p2_dn5 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn5)) / 2.0), (((locals.var_fac1p2_dn6 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn6)) / 2.0), (locals.var_vgpld_dn7 + (((locals.var_fac1p2_dn7 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn7)) / 2.0)), (locals.var_vgpld_dn8 + (((locals.var_fac1p2_dn8 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn8)) / 2.0)), (locals.var_vgpld_dn9 + (((locals.var_fac1p2_dn9 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn9)) / 2.0)), (((locals.var_fac1p2_dn10 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn10)) / 2.0), (((locals.var_fac1p2_dn11 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn11)) / 2.0), (((locals.var_fac1p2_dn14 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn14)) / 2.0),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn14,)
    }
};
        locals.var_ps0_inia = assign91980_e140832;
        locals.var_ps0_inia_dn0 = assign91980_e140832_d_n0;
        locals.var_ps0_inia_dn2 = assign91980_e140832_d_n2;
        locals.var_ps0_inia_dn4 = assign91980_e140832_d_n4;
        locals.var_ps0_inia_dn5 = assign91980_e140832_d_n5;
        locals.var_ps0_inia_dn6 = assign91980_e140832_d_n6;
        locals.var_ps0_inia_dn7 = assign91980_e140832_d_n7;
        locals.var_ps0_inia_dn8 = assign91980_e140832_d_n8;
        locals.var_ps0_inia_dn9 = assign91980_e140832_d_n9;
        locals.var_ps0_inia_dn10 = assign91980_e140832_d_n10;
        locals.var_ps0_inia_dn11 = assign91980_e140832_d_n11;
        locals.var_ps0_inia_dn14 = assign91980_e140832_d_n14;
        locals.var_ps0_inia_rv = 0.0;

        let (assign91990_e140858, assign91990_e140858_d_n0, assign91990_e140858_d_n2, assign91990_e140858_d_n4, assign91990_e140858_d_n5, assign91990_e140858_d_n6, assign91990_e140858_d_n7, assign91990_e140858_d_n8, assign91990_e140858_d_n9, assign91990_e140858_d_n10, assign91990_e140858_d_n11, assign91990_e140858_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2143 == 0.0)) {
        let assign91990_e140847: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign91990_e140848: f64 = (locals.var_beta * assign91990_e140847);
        let assign91990_e140850: f64 = (assign91990_e140848 - 1.0);
        let assign91990_e140851: f64 = (4.0 * assign91990_e140850);
        let assign91990_e140854: f64 = (locals.var_fac1p2 * locals.var_beta2);
        let assign91990_e140855: f64 = (assign91990_e140851 / assign91990_e140854);
        let assign91990_e140856: f64 = (1.0 + assign91990_e140855);
        (assign91990_e140856, ((((4.0 * ((locals.var_beta_dn0 * assign91990_e140847) + (locals.var_beta * locals.var_vxbgmtcl_dn0))) * assign91990_e140854) - (assign91990_e140851 * ((locals.var_fac1p2_dn0 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn0)))) / (assign91990_e140854 * assign91990_e140854)), ((((4.0 * ((locals.var_beta_dn2 * assign91990_e140847) + (locals.var_beta * (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2)))) * assign91990_e140854) - (assign91990_e140851 * ((locals.var_fac1p2_dn2 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn2)))) / (assign91990_e140854 * assign91990_e140854)), ((((4.0 * ((locals.var_beta_dn4 * assign91990_e140847) + (locals.var_beta * locals.var_vxbgmtcl_dn4))) * assign91990_e140854) - (assign91990_e140851 * ((locals.var_fac1p2_dn4 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn4)))) / (assign91990_e140854 * assign91990_e140854)), ((((4.0 * ((locals.var_beta_dn5 * assign91990_e140847) + (locals.var_beta * locals.var_vxbgmtcl_dn5))) * assign91990_e140854) - (assign91990_e140851 * ((locals.var_fac1p2_dn5 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn5)))) / (assign91990_e140854 * assign91990_e140854)), ((((4.0 * ((locals.var_beta_dn6 * assign91990_e140847) + (locals.var_beta * locals.var_vxbgmtcl_dn6))) * assign91990_e140854) - (assign91990_e140851 * ((locals.var_fac1p2_dn6 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn6)))) / (assign91990_e140854 * assign91990_e140854)), ((((4.0 * ((locals.var_beta_dn7 * assign91990_e140847) + (locals.var_beta * (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7)))) * assign91990_e140854) - (assign91990_e140851 * ((locals.var_fac1p2_dn7 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn7)))) / (assign91990_e140854 * assign91990_e140854)), ((((4.0 * ((locals.var_beta_dn8 * assign91990_e140847) + (locals.var_beta * (locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8)))) * assign91990_e140854) - (assign91990_e140851 * ((locals.var_fac1p2_dn8 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn8)))) / (assign91990_e140854 * assign91990_e140854)), ((((4.0 * ((locals.var_beta_dn9 * assign91990_e140847) + (locals.var_beta * (locals.var_vgpld_dn9 + locals.var_vxbgmtcl_dn9)))) * assign91990_e140854) - (assign91990_e140851 * ((locals.var_fac1p2_dn9 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn9)))) / (assign91990_e140854 * assign91990_e140854)), ((((4.0 * ((locals.var_beta_dn10 * assign91990_e140847) + (locals.var_beta * locals.var_vxbgmtcl_dn10))) * assign91990_e140854) - (assign91990_e140851 * ((locals.var_fac1p2_dn10 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn10)))) / (assign91990_e140854 * assign91990_e140854)), ((((4.0 * ((locals.var_beta_dn11 * assign91990_e140847) + (locals.var_beta * locals.var_vxbgmtcl_dn11))) * assign91990_e140854) - (assign91990_e140851 * ((locals.var_fac1p2_dn11 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn11)))) / (assign91990_e140854 * assign91990_e140854)), ((((4.0 * ((locals.var_beta_dn14 * assign91990_e140847) + (locals.var_beta * locals.var_vxbgmtcl_dn14))) * assign91990_e140854) - (assign91990_e140851 * ((locals.var_fac1p2_dn14 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn14)))) / (assign91990_e140854 * assign91990_e140854)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign91990_e140858;
        locals.var_tx_dn0 = assign91990_e140858_d_n0;
        locals.var_tx_dn2 = assign91990_e140858_d_n2;
        locals.var_tx_dn4 = assign91990_e140858_d_n4;
        locals.var_tx_dn5 = assign91990_e140858_d_n5;
        locals.var_tx_dn6 = assign91990_e140858_d_n6;
        locals.var_tx_dn7 = assign91990_e140858_d_n7;
        locals.var_tx_dn8 = assign91990_e140858_d_n8;
        locals.var_tx_dn9 = assign91990_e140858_d_n9;
        locals.var_tx_dn10 = assign91990_e140858_d_n10;
        locals.var_tx_dn11 = assign91990_e140858_d_n11;
        locals.var_tx_dn14 = assign91990_e140858_d_n14;
        locals.var_tx_rv = 0.0;

        let (assign92000_e140881, assign92000_e140881_d_n0, assign92000_e140881_d_n2, assign92000_e140881_d_n4, assign92000_e140881_d_n5, assign92000_e140881_d_n6, assign92000_e140881_d_n7, assign92000_e140881_d_n8, assign92000_e140881_d_n9, assign92000_e140881_d_n10, assign92000_e140881_d_n11, assign92000_e140881_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2143 == 0.0)) {
        let assign92000_e140871: f64 = (locals.var_fac1p2 * locals.var_beta);
        let assign92000_e140873: f64 = (assign92000_e140871 / 2.0);
        let assign92000_e140876: f64 = (locals.var_tx).sqrt();
        let assign92000_e140877: f64 = (1.0 - assign92000_e140876);
        let assign92000_e140878: f64 = (assign92000_e140873 * assign92000_e140877);
        let assign92000_e140879: f64 = (locals.var_vgpld + assign92000_e140878);
        (assign92000_e140879, (((((locals.var_fac1p2_dn0 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn0)) / 2.0) * assign92000_e140877) + (assign92000_e140873 * (-(locals.var_tx_dn0 / (2.0 * assign92000_e140876))))), (locals.var_vgpld_dn2 + (((((locals.var_fac1p2_dn2 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn2)) / 2.0) * assign92000_e140877) + (assign92000_e140873 * (-(locals.var_tx_dn2 / (2.0 * assign92000_e140876)))))), (((((locals.var_fac1p2_dn4 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn4)) / 2.0) * assign92000_e140877) + (assign92000_e140873 * (-(locals.var_tx_dn4 / (2.0 * assign92000_e140876))))), (((((locals.var_fac1p2_dn5 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn5)) / 2.0) * assign92000_e140877) + (assign92000_e140873 * (-(locals.var_tx_dn5 / (2.0 * assign92000_e140876))))), (((((locals.var_fac1p2_dn6 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn6)) / 2.0) * assign92000_e140877) + (assign92000_e140873 * (-(locals.var_tx_dn6 / (2.0 * assign92000_e140876))))), (locals.var_vgpld_dn7 + (((((locals.var_fac1p2_dn7 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn7)) / 2.0) * assign92000_e140877) + (assign92000_e140873 * (-(locals.var_tx_dn7 / (2.0 * assign92000_e140876)))))), (locals.var_vgpld_dn8 + (((((locals.var_fac1p2_dn8 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn8)) / 2.0) * assign92000_e140877) + (assign92000_e140873 * (-(locals.var_tx_dn8 / (2.0 * assign92000_e140876)))))), (locals.var_vgpld_dn9 + (((((locals.var_fac1p2_dn9 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn9)) / 2.0) * assign92000_e140877) + (assign92000_e140873 * (-(locals.var_tx_dn9 / (2.0 * assign92000_e140876)))))), (((((locals.var_fac1p2_dn10 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn10)) / 2.0) * assign92000_e140877) + (assign92000_e140873 * (-(locals.var_tx_dn10 / (2.0 * assign92000_e140876))))), (((((locals.var_fac1p2_dn11 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn11)) / 2.0) * assign92000_e140877) + (assign92000_e140873 * (-(locals.var_tx_dn11 / (2.0 * assign92000_e140876))))), (((((locals.var_fac1p2_dn14 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn14)) / 2.0) * assign92000_e140877) + (assign92000_e140873 * (-(locals.var_tx_dn14 / (2.0 * assign92000_e140876))))),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn14,)
    }
};
        locals.var_ps0_inia = assign92000_e140881;
        locals.var_ps0_inia_dn0 = assign92000_e140881_d_n0;
        locals.var_ps0_inia_dn2 = assign92000_e140881_d_n2;
        locals.var_ps0_inia_dn4 = assign92000_e140881_d_n4;
        locals.var_ps0_inia_dn5 = assign92000_e140881_d_n5;
        locals.var_ps0_inia_dn6 = assign92000_e140881_d_n6;
        locals.var_ps0_inia_dn7 = assign92000_e140881_d_n7;
        locals.var_ps0_inia_dn8 = assign92000_e140881_d_n8;
        locals.var_ps0_inia_dn9 = assign92000_e140881_d_n9;
        locals.var_ps0_inia_dn10 = assign92000_e140881_d_n10;
        locals.var_ps0_inia_dn11 = assign92000_e140881_d_n11;
        locals.var_ps0_inia_dn14 = assign92000_e140881_d_n14;
        locals.var_ps0_inia_rv = 0.0;

        let (assign92010_e140894, assign92010_e140894_d_n0, assign92010_e140894_d_n2, assign92010_e140894_d_n4, assign92010_e140894_d_n5, assign92010_e140894_d_n6, assign92010_e140894_d_n7, assign92010_e140894_d_n8, assign92010_e140894_d_n9, assign92010_e140894_d_n10, assign92010_e140894_d_n11, assign92010_e140894_d_n14,) = {
    if (((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) {
        let assign92010_e140891: f64 = (locals.var_ps0_inia + locals.var_vxbgmtcl);
        let assign92010_e140892: f64 = (locals.var_beta * assign92010_e140891);
        (assign92010_e140892, ((locals.var_beta_dn0 * assign92010_e140891) + (locals.var_beta * (locals.var_ps0_inia_dn0 + locals.var_vxbgmtcl_dn0))), ((locals.var_beta_dn2 * assign92010_e140891) + (locals.var_beta * (locals.var_ps0_inia_dn2 + locals.var_vxbgmtcl_dn2))), ((locals.var_beta_dn4 * assign92010_e140891) + (locals.var_beta * (locals.var_ps0_inia_dn4 + locals.var_vxbgmtcl_dn4))), ((locals.var_beta_dn5 * assign92010_e140891) + (locals.var_beta * (locals.var_ps0_inia_dn5 + locals.var_vxbgmtcl_dn5))), ((locals.var_beta_dn6 * assign92010_e140891) + (locals.var_beta * (locals.var_ps0_inia_dn6 + locals.var_vxbgmtcl_dn6))), ((locals.var_beta_dn7 * assign92010_e140891) + (locals.var_beta * (locals.var_ps0_inia_dn7 + locals.var_vxbgmtcl_dn7))), ((locals.var_beta_dn8 * assign92010_e140891) + (locals.var_beta * (locals.var_ps0_inia_dn8 + locals.var_vxbgmtcl_dn8))), ((locals.var_beta_dn9 * assign92010_e140891) + (locals.var_beta * (locals.var_ps0_inia_dn9 + locals.var_vxbgmtcl_dn9))), ((locals.var_beta_dn10 * assign92010_e140891) + (locals.var_beta * (locals.var_ps0_inia_dn10 + locals.var_vxbgmtcl_dn10))), ((locals.var_beta_dn11 * assign92010_e140891) + (locals.var_beta * (locals.var_ps0_inia_dn11 + locals.var_vxbgmtcl_dn11))), ((locals.var_beta_dn14 * assign92010_e140891) + (locals.var_beta * (locals.var_ps0_inia_dn14 + locals.var_vxbgmtcl_dn14))),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
        locals.var_chi = assign92010_e140894;
        locals.var_chi_dn0 = assign92010_e140894_d_n0;
        locals.var_chi_dn2 = assign92010_e140894_d_n2;
        locals.var_chi_dn4 = assign92010_e140894_d_n4;
        locals.var_chi_dn5 = assign92010_e140894_d_n5;
        locals.var_chi_dn6 = assign92010_e140894_d_n6;
        locals.var_chi_dn7 = assign92010_e140894_d_n7;
        locals.var_chi_dn8 = assign92010_e140894_d_n8;
        locals.var_chi_dn9 = assign92010_e140894_d_n9;
        locals.var_chi_dn10 = assign92010_e140894_d_n10;
        locals.var_chi_dn11 = assign92010_e140894_d_n11;
        locals.var_chi_dn14 = assign92010_e140894_d_n14;
        locals.var_chi_rv = 0.0;

        let assign92020_e140897: f64 = if locals.var_chi >= 3.0 { 1.0 } else { 0.0 };
        locals.var_guard2144 = assign92020_e140897;
        locals.var_guard2144_rv = 0.0;

        let (assign92040_e140921, assign92040_e140921_d_n0, assign92040_e140921_d_n2, assign92040_e140921_d_n4, assign92040_e140921_d_n5, assign92040_e140921_d_n6, assign92040_e140921_d_n7, assign92040_e140921_d_n8, assign92040_e140921_d_n9, assign92040_e140921_d_n10, assign92040_e140921_d_n11, assign92040_e140921_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2144 != 0.0)) {
        let assign92040_e140918: f64 = (-locals.var_chi);
        let assign92040_e140919: f64 = (assign92040_e140918).exp();
        (assign92040_e140919, (assign92040_e140919 * (-locals.var_chi_dn0)), (assign92040_e140919 * (-locals.var_chi_dn2)), (assign92040_e140919 * (-locals.var_chi_dn4)), (assign92040_e140919 * (-locals.var_chi_dn5)), (assign92040_e140919 * (-locals.var_chi_dn6)), (assign92040_e140919 * (-locals.var_chi_dn7)), (assign92040_e140919 * (-locals.var_chi_dn8)), (assign92040_e140919 * (-locals.var_chi_dn9)), (assign92040_e140919 * (-locals.var_chi_dn10)), (assign92040_e140919 * (-locals.var_chi_dn11)), (assign92040_e140919 * (-locals.var_chi_dn14)),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn14,)
    }
};
        locals.var_ty = assign92040_e140921;
        locals.var_ty_dn0 = assign92040_e140921_d_n0;
        locals.var_ty_dn2 = assign92040_e140921_d_n2;
        locals.var_ty_dn4 = assign92040_e140921_d_n4;
        locals.var_ty_dn5 = assign92040_e140921_d_n5;
        locals.var_ty_dn6 = assign92040_e140921_d_n6;
        locals.var_ty_dn7 = assign92040_e140921_d_n7;
        locals.var_ty_dn8 = assign92040_e140921_d_n8;
        locals.var_ty_dn9 = assign92040_e140921_d_n9;
        locals.var_ty_dn10 = assign92040_e140921_d_n10;
        locals.var_ty_dn11 = assign92040_e140921_d_n11;
        locals.var_ty_dn14 = assign92040_e140921_d_n14;
        locals.var_ty_rv = 0.0;

        let (assign92050_e140948, assign92050_e140948_d_n0, assign92050_e140948_d_n2, assign92050_e140948_d_n4, assign92050_e140948_d_n5, assign92050_e140948_d_n6, assign92050_e140948_d_n7, assign92050_e140948_d_n8, assign92050_e140948_d_n9, assign92050_e140948_d_n10, assign92050_e140948_d_n11, assign92050_e140948_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2144 != 0.0)) {
        let assign92050_e140935: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign92050_e140936: f64 = (locals.var_beta * assign92050_e140935);
        let assign92050_e140938: f64 = (assign92050_e140936 - 1.0);
        let assign92050_e140940: f64 = (assign92050_e140938 + locals.var_ty);
        let assign92050_e140941: f64 = (4.0 * assign92050_e140940);
        let assign92050_e140944: f64 = (locals.var_fac1p2 * locals.var_beta2);
        let assign92050_e140945: f64 = (assign92050_e140941 / assign92050_e140944);
        let assign92050_e140946: f64 = (1.0 + assign92050_e140945);
        (assign92050_e140946, ((((4.0 * (((locals.var_beta_dn0 * assign92050_e140935) + (locals.var_beta * locals.var_vxbgmtcl_dn0)) + locals.var_ty_dn0)) * assign92050_e140944) - (assign92050_e140941 * ((locals.var_fac1p2_dn0 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn0)))) / (assign92050_e140944 * assign92050_e140944)), ((((4.0 * (((locals.var_beta_dn2 * assign92050_e140935) + (locals.var_beta * (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2))) + locals.var_ty_dn2)) * assign92050_e140944) - (assign92050_e140941 * ((locals.var_fac1p2_dn2 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn2)))) / (assign92050_e140944 * assign92050_e140944)), ((((4.0 * (((locals.var_beta_dn4 * assign92050_e140935) + (locals.var_beta * locals.var_vxbgmtcl_dn4)) + locals.var_ty_dn4)) * assign92050_e140944) - (assign92050_e140941 * ((locals.var_fac1p2_dn4 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn4)))) / (assign92050_e140944 * assign92050_e140944)), ((((4.0 * (((locals.var_beta_dn5 * assign92050_e140935) + (locals.var_beta * locals.var_vxbgmtcl_dn5)) + locals.var_ty_dn5)) * assign92050_e140944) - (assign92050_e140941 * ((locals.var_fac1p2_dn5 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn5)))) / (assign92050_e140944 * assign92050_e140944)), ((((4.0 * (((locals.var_beta_dn6 * assign92050_e140935) + (locals.var_beta * locals.var_vxbgmtcl_dn6)) + locals.var_ty_dn6)) * assign92050_e140944) - (assign92050_e140941 * ((locals.var_fac1p2_dn6 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn6)))) / (assign92050_e140944 * assign92050_e140944)), ((((4.0 * (((locals.var_beta_dn7 * assign92050_e140935) + (locals.var_beta * (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7))) + locals.var_ty_dn7)) * assign92050_e140944) - (assign92050_e140941 * ((locals.var_fac1p2_dn7 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn7)))) / (assign92050_e140944 * assign92050_e140944)), ((((4.0 * (((locals.var_beta_dn8 * assign92050_e140935) + (locals.var_beta * (locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8))) + locals.var_ty_dn8)) * assign92050_e140944) - (assign92050_e140941 * ((locals.var_fac1p2_dn8 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn8)))) / (assign92050_e140944 * assign92050_e140944)), ((((4.0 * (((locals.var_beta_dn9 * assign92050_e140935) + (locals.var_beta * (locals.var_vgpld_dn9 + locals.var_vxbgmtcl_dn9))) + locals.var_ty_dn9)) * assign92050_e140944) - (assign92050_e140941 * ((locals.var_fac1p2_dn9 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn9)))) / (assign92050_e140944 * assign92050_e140944)), ((((4.0 * (((locals.var_beta_dn10 * assign92050_e140935) + (locals.var_beta * locals.var_vxbgmtcl_dn10)) + locals.var_ty_dn10)) * assign92050_e140944) - (assign92050_e140941 * ((locals.var_fac1p2_dn10 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn10)))) / (assign92050_e140944 * assign92050_e140944)), ((((4.0 * (((locals.var_beta_dn11 * assign92050_e140935) + (locals.var_beta * locals.var_vxbgmtcl_dn11)) + locals.var_ty_dn11)) * assign92050_e140944) - (assign92050_e140941 * ((locals.var_fac1p2_dn11 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn11)))) / (assign92050_e140944 * assign92050_e140944)), ((((4.0 * (((locals.var_beta_dn14 * assign92050_e140935) + (locals.var_beta * locals.var_vxbgmtcl_dn14)) + locals.var_ty_dn14)) * assign92050_e140944) - (assign92050_e140941 * ((locals.var_fac1p2_dn14 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn14)))) / (assign92050_e140944 * assign92050_e140944)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign92050_e140948;
        locals.var_tx_dn0 = assign92050_e140948_d_n0;
        locals.var_tx_dn2 = assign92050_e140948_d_n2;
        locals.var_tx_dn4 = assign92050_e140948_d_n4;
        locals.var_tx_dn5 = assign92050_e140948_d_n5;
        locals.var_tx_dn6 = assign92050_e140948_d_n6;
        locals.var_tx_dn7 = assign92050_e140948_d_n7;
        locals.var_tx_dn8 = assign92050_e140948_d_n8;
        locals.var_tx_dn9 = assign92050_e140948_d_n9;
        locals.var_tx_dn10 = assign92050_e140948_d_n10;
        locals.var_tx_dn11 = assign92050_e140948_d_n11;
        locals.var_tx_dn14 = assign92050_e140948_d_n14;
        locals.var_tx_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_355(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign92060_e140970, assign92060_e140970_d_n0, assign92060_e140970_d_n2, assign92060_e140970_d_n4, assign92060_e140970_d_n5, assign92060_e140970_d_n6, assign92060_e140970_d_n7, assign92060_e140970_d_n8, assign92060_e140970_d_n9, assign92060_e140970_d_n10, assign92060_e140970_d_n11, assign92060_e140970_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2144 != 0.0)) {
        let assign92060_e140960: f64 = (locals.var_fac1p2 * locals.var_beta);
        let assign92060_e140962: f64 = (assign92060_e140960 / 2.0);
        let assign92060_e140965: f64 = (locals.var_tx).sqrt();
        let assign92060_e140966: f64 = (1.0 - assign92060_e140965);
        let assign92060_e140967: f64 = (assign92060_e140962 * assign92060_e140966);
        let assign92060_e140968: f64 = (locals.var_vgpld + assign92060_e140967);
        (assign92060_e140968, (((((locals.var_fac1p2_dn0 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn0)) / 2.0) * assign92060_e140966) + (assign92060_e140962 * (-(locals.var_tx_dn0 / (2.0 * assign92060_e140965))))), (locals.var_vgpld_dn2 + (((((locals.var_fac1p2_dn2 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn2)) / 2.0) * assign92060_e140966) + (assign92060_e140962 * (-(locals.var_tx_dn2 / (2.0 * assign92060_e140965)))))), (((((locals.var_fac1p2_dn4 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn4)) / 2.0) * assign92060_e140966) + (assign92060_e140962 * (-(locals.var_tx_dn4 / (2.0 * assign92060_e140965))))), (((((locals.var_fac1p2_dn5 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn5)) / 2.0) * assign92060_e140966) + (assign92060_e140962 * (-(locals.var_tx_dn5 / (2.0 * assign92060_e140965))))), (((((locals.var_fac1p2_dn6 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn6)) / 2.0) * assign92060_e140966) + (assign92060_e140962 * (-(locals.var_tx_dn6 / (2.0 * assign92060_e140965))))), (locals.var_vgpld_dn7 + (((((locals.var_fac1p2_dn7 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn7)) / 2.0) * assign92060_e140966) + (assign92060_e140962 * (-(locals.var_tx_dn7 / (2.0 * assign92060_e140965)))))), (locals.var_vgpld_dn8 + (((((locals.var_fac1p2_dn8 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn8)) / 2.0) * assign92060_e140966) + (assign92060_e140962 * (-(locals.var_tx_dn8 / (2.0 * assign92060_e140965)))))), (locals.var_vgpld_dn9 + (((((locals.var_fac1p2_dn9 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn9)) / 2.0) * assign92060_e140966) + (assign92060_e140962 * (-(locals.var_tx_dn9 / (2.0 * assign92060_e140965)))))), (((((locals.var_fac1p2_dn10 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn10)) / 2.0) * assign92060_e140966) + (assign92060_e140962 * (-(locals.var_tx_dn10 / (2.0 * assign92060_e140965))))), (((((locals.var_fac1p2_dn11 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn11)) / 2.0) * assign92060_e140966) + (assign92060_e140962 * (-(locals.var_tx_dn11 / (2.0 * assign92060_e140965))))), (((((locals.var_fac1p2_dn14 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn14)) / 2.0) * assign92060_e140966) + (assign92060_e140962 * (-(locals.var_tx_dn14 / (2.0 * assign92060_e140965))))),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn14,)
    }
};
        locals.var_ps0_inia = assign92060_e140970;
        locals.var_ps0_inia_dn0 = assign92060_e140970_d_n0;
        locals.var_ps0_inia_dn2 = assign92060_e140970_d_n2;
        locals.var_ps0_inia_dn4 = assign92060_e140970_d_n4;
        locals.var_ps0_inia_dn5 = assign92060_e140970_d_n5;
        locals.var_ps0_inia_dn6 = assign92060_e140970_d_n6;
        locals.var_ps0_inia_dn7 = assign92060_e140970_d_n7;
        locals.var_ps0_inia_dn8 = assign92060_e140970_d_n8;
        locals.var_ps0_inia_dn9 = assign92060_e140970_d_n9;
        locals.var_ps0_inia_dn10 = assign92060_e140970_d_n10;
        locals.var_ps0_inia_dn11 = assign92060_e140970_d_n11;
        locals.var_ps0_inia_dn14 = assign92060_e140970_d_n14;
        locals.var_ps0_inia_rv = 0.0;

        let (assign92070_e140985, assign92070_e140985_d_n0, assign92070_e140985_d_n2, assign92070_e140985_d_n4, assign92070_e140985_d_n5, assign92070_e140985_d_n6, assign92070_e140985_d_n7, assign92070_e140985_d_n8, assign92070_e140985_d_n9, assign92070_e140985_d_n10, assign92070_e140985_d_n11, assign92070_e140985_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2144 != 0.0)) {
        let assign92070_e140982: f64 = (locals.var_ps0_inia + locals.var_vxbgmtcl);
        let assign92070_e140983: f64 = (locals.var_beta * assign92070_e140982);
        (assign92070_e140983, ((locals.var_beta_dn0 * assign92070_e140982) + (locals.var_beta * (locals.var_ps0_inia_dn0 + locals.var_vxbgmtcl_dn0))), ((locals.var_beta_dn2 * assign92070_e140982) + (locals.var_beta * (locals.var_ps0_inia_dn2 + locals.var_vxbgmtcl_dn2))), ((locals.var_beta_dn4 * assign92070_e140982) + (locals.var_beta * (locals.var_ps0_inia_dn4 + locals.var_vxbgmtcl_dn4))), ((locals.var_beta_dn5 * assign92070_e140982) + (locals.var_beta * (locals.var_ps0_inia_dn5 + locals.var_vxbgmtcl_dn5))), ((locals.var_beta_dn6 * assign92070_e140982) + (locals.var_beta * (locals.var_ps0_inia_dn6 + locals.var_vxbgmtcl_dn6))), ((locals.var_beta_dn7 * assign92070_e140982) + (locals.var_beta * (locals.var_ps0_inia_dn7 + locals.var_vxbgmtcl_dn7))), ((locals.var_beta_dn8 * assign92070_e140982) + (locals.var_beta * (locals.var_ps0_inia_dn8 + locals.var_vxbgmtcl_dn8))), ((locals.var_beta_dn9 * assign92070_e140982) + (locals.var_beta * (locals.var_ps0_inia_dn9 + locals.var_vxbgmtcl_dn9))), ((locals.var_beta_dn10 * assign92070_e140982) + (locals.var_beta * (locals.var_ps0_inia_dn10 + locals.var_vxbgmtcl_dn10))), ((locals.var_beta_dn11 * assign92070_e140982) + (locals.var_beta * (locals.var_ps0_inia_dn11 + locals.var_vxbgmtcl_dn11))), ((locals.var_beta_dn14 * assign92070_e140982) + (locals.var_beta * (locals.var_ps0_inia_dn14 + locals.var_vxbgmtcl_dn14))),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
        locals.var_chi = assign92070_e140985;
        locals.var_chi_dn0 = assign92070_e140985_d_n0;
        locals.var_chi_dn2 = assign92070_e140985_d_n2;
        locals.var_chi_dn4 = assign92070_e140985_d_n4;
        locals.var_chi_dn5 = assign92070_e140985_d_n5;
        locals.var_chi_dn6 = assign92070_e140985_d_n6;
        locals.var_chi_dn7 = assign92070_e140985_d_n7;
        locals.var_chi_dn8 = assign92070_e140985_d_n8;
        locals.var_chi_dn9 = assign92070_e140985_d_n9;
        locals.var_chi_dn10 = assign92070_e140985_d_n10;
        locals.var_chi_dn11 = assign92070_e140985_d_n11;
        locals.var_chi_dn14 = assign92070_e140985_d_n14;
        locals.var_chi_rv = 0.0;

        let (assign92080_e140998, assign92080_e140998_d_n0, assign92080_e140998_d_n2, assign92080_e140998_d_n4, assign92080_e140998_d_n5, assign92080_e140998_d_n6, assign92080_e140998_d_n7, assign92080_e140998_d_n8, assign92080_e140998_d_n9, assign92080_e140998_d_n10, assign92080_e140998_d_n11, assign92080_e140998_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2144 != 0.0)) {
        let assign92080_e140995: f64 = (-locals.var_chi);
        let assign92080_e140996: f64 = (assign92080_e140995).exp();
        (assign92080_e140996, (assign92080_e140996 * (-locals.var_chi_dn0)), (assign92080_e140996 * (-locals.var_chi_dn2)), (assign92080_e140996 * (-locals.var_chi_dn4)), (assign92080_e140996 * (-locals.var_chi_dn5)), (assign92080_e140996 * (-locals.var_chi_dn6)), (assign92080_e140996 * (-locals.var_chi_dn7)), (assign92080_e140996 * (-locals.var_chi_dn8)), (assign92080_e140996 * (-locals.var_chi_dn9)), (assign92080_e140996 * (-locals.var_chi_dn10)), (assign92080_e140996 * (-locals.var_chi_dn11)), (assign92080_e140996 * (-locals.var_chi_dn14)),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn14,)
    }
};
        locals.var_ty = assign92080_e140998;
        locals.var_ty_dn0 = assign92080_e140998_d_n0;
        locals.var_ty_dn2 = assign92080_e140998_d_n2;
        locals.var_ty_dn4 = assign92080_e140998_d_n4;
        locals.var_ty_dn5 = assign92080_e140998_d_n5;
        locals.var_ty_dn6 = assign92080_e140998_d_n6;
        locals.var_ty_dn7 = assign92080_e140998_d_n7;
        locals.var_ty_dn8 = assign92080_e140998_d_n8;
        locals.var_ty_dn9 = assign92080_e140998_d_n9;
        locals.var_ty_dn10 = assign92080_e140998_d_n10;
        locals.var_ty_dn11 = assign92080_e140998_d_n11;
        locals.var_ty_dn14 = assign92080_e140998_d_n14;
        locals.var_ty_rv = 0.0;

        let (assign92090_e141025, assign92090_e141025_d_n0, assign92090_e141025_d_n2, assign92090_e141025_d_n4, assign92090_e141025_d_n5, assign92090_e141025_d_n6, assign92090_e141025_d_n7, assign92090_e141025_d_n8, assign92090_e141025_d_n9, assign92090_e141025_d_n10, assign92090_e141025_d_n11, assign92090_e141025_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2144 != 0.0)) {
        let assign92090_e141012: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign92090_e141013: f64 = (locals.var_beta * assign92090_e141012);
        let assign92090_e141015: f64 = (assign92090_e141013 - 1.0);
        let assign92090_e141017: f64 = (assign92090_e141015 + locals.var_ty);
        let assign92090_e141018: f64 = (4.0 * assign92090_e141017);
        let assign92090_e141021: f64 = (locals.var_fac1p2 * locals.var_beta2);
        let assign92090_e141022: f64 = (assign92090_e141018 / assign92090_e141021);
        let assign92090_e141023: f64 = (1.0 + assign92090_e141022);
        (assign92090_e141023, ((((4.0 * (((locals.var_beta_dn0 * assign92090_e141012) + (locals.var_beta * locals.var_vxbgmtcl_dn0)) + locals.var_ty_dn0)) * assign92090_e141021) - (assign92090_e141018 * ((locals.var_fac1p2_dn0 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn0)))) / (assign92090_e141021 * assign92090_e141021)), ((((4.0 * (((locals.var_beta_dn2 * assign92090_e141012) + (locals.var_beta * (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2))) + locals.var_ty_dn2)) * assign92090_e141021) - (assign92090_e141018 * ((locals.var_fac1p2_dn2 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn2)))) / (assign92090_e141021 * assign92090_e141021)), ((((4.0 * (((locals.var_beta_dn4 * assign92090_e141012) + (locals.var_beta * locals.var_vxbgmtcl_dn4)) + locals.var_ty_dn4)) * assign92090_e141021) - (assign92090_e141018 * ((locals.var_fac1p2_dn4 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn4)))) / (assign92090_e141021 * assign92090_e141021)), ((((4.0 * (((locals.var_beta_dn5 * assign92090_e141012) + (locals.var_beta * locals.var_vxbgmtcl_dn5)) + locals.var_ty_dn5)) * assign92090_e141021) - (assign92090_e141018 * ((locals.var_fac1p2_dn5 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn5)))) / (assign92090_e141021 * assign92090_e141021)), ((((4.0 * (((locals.var_beta_dn6 * assign92090_e141012) + (locals.var_beta * locals.var_vxbgmtcl_dn6)) + locals.var_ty_dn6)) * assign92090_e141021) - (assign92090_e141018 * ((locals.var_fac1p2_dn6 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn6)))) / (assign92090_e141021 * assign92090_e141021)), ((((4.0 * (((locals.var_beta_dn7 * assign92090_e141012) + (locals.var_beta * (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7))) + locals.var_ty_dn7)) * assign92090_e141021) - (assign92090_e141018 * ((locals.var_fac1p2_dn7 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn7)))) / (assign92090_e141021 * assign92090_e141021)), ((((4.0 * (((locals.var_beta_dn8 * assign92090_e141012) + (locals.var_beta * (locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8))) + locals.var_ty_dn8)) * assign92090_e141021) - (assign92090_e141018 * ((locals.var_fac1p2_dn8 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn8)))) / (assign92090_e141021 * assign92090_e141021)), ((((4.0 * (((locals.var_beta_dn9 * assign92090_e141012) + (locals.var_beta * (locals.var_vgpld_dn9 + locals.var_vxbgmtcl_dn9))) + locals.var_ty_dn9)) * assign92090_e141021) - (assign92090_e141018 * ((locals.var_fac1p2_dn9 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn9)))) / (assign92090_e141021 * assign92090_e141021)), ((((4.0 * (((locals.var_beta_dn10 * assign92090_e141012) + (locals.var_beta * locals.var_vxbgmtcl_dn10)) + locals.var_ty_dn10)) * assign92090_e141021) - (assign92090_e141018 * ((locals.var_fac1p2_dn10 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn10)))) / (assign92090_e141021 * assign92090_e141021)), ((((4.0 * (((locals.var_beta_dn11 * assign92090_e141012) + (locals.var_beta * locals.var_vxbgmtcl_dn11)) + locals.var_ty_dn11)) * assign92090_e141021) - (assign92090_e141018 * ((locals.var_fac1p2_dn11 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn11)))) / (assign92090_e141021 * assign92090_e141021)), ((((4.0 * (((locals.var_beta_dn14 * assign92090_e141012) + (locals.var_beta * locals.var_vxbgmtcl_dn14)) + locals.var_ty_dn14)) * assign92090_e141021) - (assign92090_e141018 * ((locals.var_fac1p2_dn14 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn14)))) / (assign92090_e141021 * assign92090_e141021)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign92090_e141025;
        locals.var_tx_dn0 = assign92090_e141025_d_n0;
        locals.var_tx_dn2 = assign92090_e141025_d_n2;
        locals.var_tx_dn4 = assign92090_e141025_d_n4;
        locals.var_tx_dn5 = assign92090_e141025_d_n5;
        locals.var_tx_dn6 = assign92090_e141025_d_n6;
        locals.var_tx_dn7 = assign92090_e141025_d_n7;
        locals.var_tx_dn8 = assign92090_e141025_d_n8;
        locals.var_tx_dn9 = assign92090_e141025_d_n9;
        locals.var_tx_dn10 = assign92090_e141025_d_n10;
        locals.var_tx_dn11 = assign92090_e141025_d_n11;
        locals.var_tx_dn14 = assign92090_e141025_d_n14;
        locals.var_tx_rv = 0.0;

        let (assign92100_e141047, assign92100_e141047_d_n0, assign92100_e141047_d_n2, assign92100_e141047_d_n4, assign92100_e141047_d_n5, assign92100_e141047_d_n6, assign92100_e141047_d_n7, assign92100_e141047_d_n8, assign92100_e141047_d_n9, assign92100_e141047_d_n10, assign92100_e141047_d_n11, assign92100_e141047_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2144 != 0.0)) {
        let assign92100_e141037: f64 = (locals.var_fac1p2 * locals.var_beta);
        let assign92100_e141039: f64 = (assign92100_e141037 / 2.0);
        let assign92100_e141042: f64 = (locals.var_tx).sqrt();
        let assign92100_e141043: f64 = (1.0 - assign92100_e141042);
        let assign92100_e141044: f64 = (assign92100_e141039 * assign92100_e141043);
        let assign92100_e141045: f64 = (locals.var_vgpld + assign92100_e141044);
        (assign92100_e141045, (((((locals.var_fac1p2_dn0 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn0)) / 2.0) * assign92100_e141043) + (assign92100_e141039 * (-(locals.var_tx_dn0 / (2.0 * assign92100_e141042))))), (locals.var_vgpld_dn2 + (((((locals.var_fac1p2_dn2 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn2)) / 2.0) * assign92100_e141043) + (assign92100_e141039 * (-(locals.var_tx_dn2 / (2.0 * assign92100_e141042)))))), (((((locals.var_fac1p2_dn4 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn4)) / 2.0) * assign92100_e141043) + (assign92100_e141039 * (-(locals.var_tx_dn4 / (2.0 * assign92100_e141042))))), (((((locals.var_fac1p2_dn5 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn5)) / 2.0) * assign92100_e141043) + (assign92100_e141039 * (-(locals.var_tx_dn5 / (2.0 * assign92100_e141042))))), (((((locals.var_fac1p2_dn6 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn6)) / 2.0) * assign92100_e141043) + (assign92100_e141039 * (-(locals.var_tx_dn6 / (2.0 * assign92100_e141042))))), (locals.var_vgpld_dn7 + (((((locals.var_fac1p2_dn7 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn7)) / 2.0) * assign92100_e141043) + (assign92100_e141039 * (-(locals.var_tx_dn7 / (2.0 * assign92100_e141042)))))), (locals.var_vgpld_dn8 + (((((locals.var_fac1p2_dn8 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn8)) / 2.0) * assign92100_e141043) + (assign92100_e141039 * (-(locals.var_tx_dn8 / (2.0 * assign92100_e141042)))))), (locals.var_vgpld_dn9 + (((((locals.var_fac1p2_dn9 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn9)) / 2.0) * assign92100_e141043) + (assign92100_e141039 * (-(locals.var_tx_dn9 / (2.0 * assign92100_e141042)))))), (((((locals.var_fac1p2_dn10 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn10)) / 2.0) * assign92100_e141043) + (assign92100_e141039 * (-(locals.var_tx_dn10 / (2.0 * assign92100_e141042))))), (((((locals.var_fac1p2_dn11 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn11)) / 2.0) * assign92100_e141043) + (assign92100_e141039 * (-(locals.var_tx_dn11 / (2.0 * assign92100_e141042))))), (((((locals.var_fac1p2_dn14 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn14)) / 2.0) * assign92100_e141043) + (assign92100_e141039 * (-(locals.var_tx_dn14 / (2.0 * assign92100_e141042))))),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn14,)
    }
};
        locals.var_ps0_inia = assign92100_e141047;
        locals.var_ps0_inia_dn0 = assign92100_e141047_d_n0;
        locals.var_ps0_inia_dn2 = assign92100_e141047_d_n2;
        locals.var_ps0_inia_dn4 = assign92100_e141047_d_n4;
        locals.var_ps0_inia_dn5 = assign92100_e141047_d_n5;
        locals.var_ps0_inia_dn6 = assign92100_e141047_d_n6;
        locals.var_ps0_inia_dn7 = assign92100_e141047_d_n7;
        locals.var_ps0_inia_dn8 = assign92100_e141047_d_n8;
        locals.var_ps0_inia_dn9 = assign92100_e141047_d_n9;
        locals.var_ps0_inia_dn10 = assign92100_e141047_d_n10;
        locals.var_ps0_inia_dn11 = assign92100_e141047_d_n11;
        locals.var_ps0_inia_dn14 = assign92100_e141047_d_n14;
        locals.var_ps0_inia_rv = 0.0;

        let (assign92110_e141062, assign92110_e141062_d_n0, assign92110_e141062_d_n2, assign92110_e141062_d_n4, assign92110_e141062_d_n5, assign92110_e141062_d_n6, assign92110_e141062_d_n7, assign92110_e141062_d_n8, assign92110_e141062_d_n9, assign92110_e141062_d_n10, assign92110_e141062_d_n11, assign92110_e141062_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2144 != 0.0)) {
        let assign92110_e141059: f64 = (locals.var_ps0_inia + locals.var_vxbgmtcl);
        let assign92110_e141060: f64 = (locals.var_beta * assign92110_e141059);
        (assign92110_e141060, ((locals.var_beta_dn0 * assign92110_e141059) + (locals.var_beta * (locals.var_ps0_inia_dn0 + locals.var_vxbgmtcl_dn0))), ((locals.var_beta_dn2 * assign92110_e141059) + (locals.var_beta * (locals.var_ps0_inia_dn2 + locals.var_vxbgmtcl_dn2))), ((locals.var_beta_dn4 * assign92110_e141059) + (locals.var_beta * (locals.var_ps0_inia_dn4 + locals.var_vxbgmtcl_dn4))), ((locals.var_beta_dn5 * assign92110_e141059) + (locals.var_beta * (locals.var_ps0_inia_dn5 + locals.var_vxbgmtcl_dn5))), ((locals.var_beta_dn6 * assign92110_e141059) + (locals.var_beta * (locals.var_ps0_inia_dn6 + locals.var_vxbgmtcl_dn6))), ((locals.var_beta_dn7 * assign92110_e141059) + (locals.var_beta * (locals.var_ps0_inia_dn7 + locals.var_vxbgmtcl_dn7))), ((locals.var_beta_dn8 * assign92110_e141059) + (locals.var_beta * (locals.var_ps0_inia_dn8 + locals.var_vxbgmtcl_dn8))), ((locals.var_beta_dn9 * assign92110_e141059) + (locals.var_beta * (locals.var_ps0_inia_dn9 + locals.var_vxbgmtcl_dn9))), ((locals.var_beta_dn10 * assign92110_e141059) + (locals.var_beta * (locals.var_ps0_inia_dn10 + locals.var_vxbgmtcl_dn10))), ((locals.var_beta_dn11 * assign92110_e141059) + (locals.var_beta * (locals.var_ps0_inia_dn11 + locals.var_vxbgmtcl_dn11))), ((locals.var_beta_dn14 * assign92110_e141059) + (locals.var_beta * (locals.var_ps0_inia_dn14 + locals.var_vxbgmtcl_dn14))),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
        locals.var_chi = assign92110_e141062;
        locals.var_chi_dn0 = assign92110_e141062_d_n0;
        locals.var_chi_dn2 = assign92110_e141062_d_n2;
        locals.var_chi_dn4 = assign92110_e141062_d_n4;
        locals.var_chi_dn5 = assign92110_e141062_d_n5;
        locals.var_chi_dn6 = assign92110_e141062_d_n6;
        locals.var_chi_dn7 = assign92110_e141062_d_n7;
        locals.var_chi_dn8 = assign92110_e141062_d_n8;
        locals.var_chi_dn9 = assign92110_e141062_d_n9;
        locals.var_chi_dn10 = assign92110_e141062_d_n10;
        locals.var_chi_dn11 = assign92110_e141062_d_n11;
        locals.var_chi_dn14 = assign92110_e141062_d_n14;
        locals.var_chi_rv = 0.0;

        let (assign92130_e141108,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2144 == 0.0)) {
        let assign92130_e141087: f64 = (2.0_f64).sqrt();
        let assign92130_e141088: f64 = (9.0 * assign92130_e141087);
        let assign92130_e141089: f64 = (1.0 / assign92130_e141088);
        let assign92130_e141093: f64 = (-3.0);
        let assign92130_e141094: f64 = (assign92130_e141093).exp();
        let assign92130_e141095: f64 = (7.0 * assign92130_e141094);
        let assign92130_e141096: f64 = (5.0 + assign92130_e141095);
        let assign92130_e141100: f64 = (-3.0);
        let assign92130_e141101: f64 = (assign92130_e141100).exp();
        let assign92130_e141102: f64 = (2.0 + assign92130_e141101);
        let assign92130_e141103: f64 = (assign92130_e141102).sqrt();
        let assign92130_e141104: f64 = (54.0 * assign92130_e141103);
        let assign92130_e141105: f64 = (assign92130_e141096 / assign92130_e141104);
        let assign92130_e141106: f64 = (assign92130_e141089 - assign92130_e141105);
        (assign92130_e141106,)
    } else {
        (locals.var_ta,)
    }
};
        locals.var_ta = assign92130_e141108;
        locals.var_ta_rv = 0.0;

        let (assign92140_e141138,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2144 == 0.0)) {
        let assign92140_e141120: f64 = (-3.0);
        let assign92140_e141121: f64 = (assign92140_e141120).exp();
        let assign92140_e141122: f64 = (1.0 + assign92140_e141121);
        let assign92140_e141126: f64 = (-3.0);
        let assign92140_e141127: f64 = (assign92140_e141126).exp();
        let assign92140_e141128: f64 = (2.0 + assign92140_e141127);
        let assign92140_e141129: f64 = (assign92140_e141128).sqrt();
        let assign92140_e141130: f64 = (2.0 * assign92140_e141129);
        let assign92140_e141131: f64 = (assign92140_e141122 / assign92140_e141130);
        let assign92140_e141133: f64 = (2.0_f64).sqrt();
        let assign92140_e141135: f64 = (assign92140_e141133 / 3.0);
        let assign92140_e141136: f64 = (assign92140_e141131 - assign92140_e141135);
        (assign92140_e141136,)
    } else {
        (locals.var_tb,)
    }
};
        locals.var_tb = assign92140_e141138;
        locals.var_tb_rv = 0.0;

        let (assign92150_e141159, assign92150_e141159_d_n0, assign92150_e141159_d_n2, assign92150_e141159_d_n4, assign92150_e141159_d_n5, assign92150_e141159_d_n6, assign92150_e141159_d_n7, assign92150_e141159_d_n8, assign92150_e141159_d_n9, assign92150_e141159_d_n10, assign92150_e141159_d_n11, assign92150_e141159_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2144 == 0.0)) {
        let assign92150_e141150: f64 = (2.0_f64).sqrt();
        let assign92150_e141151: f64 = (1.0 / assign92150_e141150);
        let assign92150_e141155: f64 = (locals.var_beta * locals.var_fac1);
        let assign92150_e141156: f64 = (1.0 / assign92150_e141155);
        let assign92150_e141157: f64 = (assign92150_e141151 + assign92150_e141156);
        (assign92150_e141157, (-(((locals.var_beta_dn0 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn0)) / (assign92150_e141155 * assign92150_e141155))), (-(((locals.var_beta_dn2 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn2)) / (assign92150_e141155 * assign92150_e141155))), (-(((locals.var_beta_dn4 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn4)) / (assign92150_e141155 * assign92150_e141155))), (-(((locals.var_beta_dn5 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn5)) / (assign92150_e141155 * assign92150_e141155))), (-(((locals.var_beta_dn6 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn6)) / (assign92150_e141155 * assign92150_e141155))), (-(((locals.var_beta_dn7 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn7)) / (assign92150_e141155 * assign92150_e141155))), (-(((locals.var_beta_dn8 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn8)) / (assign92150_e141155 * assign92150_e141155))), (-(((locals.var_beta_dn9 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn9)) / (assign92150_e141155 * assign92150_e141155))), (-(((locals.var_beta_dn10 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn10)) / (assign92150_e141155 * assign92150_e141155))), (-(((locals.var_beta_dn11 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn11)) / (assign92150_e141155 * assign92150_e141155))), (-(((locals.var_beta_dn14 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn14)) / (assign92150_e141155 * assign92150_e141155))),)
    } else {
        (locals.var_tc, locals.var_tc_dn0, locals.var_tc_dn2, locals.var_tc_dn4, locals.var_tc_dn5, locals.var_tc_dn6, locals.var_tc_dn7, locals.var_tc_dn8, locals.var_tc_dn9, locals.var_tc_dn10, locals.var_tc_dn11, locals.var_tc_dn14,)
    }
};
        locals.var_tc = assign92150_e141159;
        locals.var_tc_dn0 = assign92150_e141159_d_n0;
        locals.var_tc_dn2 = assign92150_e141159_d_n2;
        locals.var_tc_dn4 = assign92150_e141159_d_n4;
        locals.var_tc_dn5 = assign92150_e141159_d_n5;
        locals.var_tc_dn6 = assign92150_e141159_d_n6;
        locals.var_tc_dn7 = assign92150_e141159_d_n7;
        locals.var_tc_dn8 = assign92150_e141159_d_n8;
        locals.var_tc_dn9 = assign92150_e141159_d_n9;
        locals.var_tc_dn10 = assign92150_e141159_d_n10;
        locals.var_tc_dn11 = assign92150_e141159_d_n11;
        locals.var_tc_dn14 = assign92150_e141159_d_n14;
        locals.var_tc_rv = 0.0;

        let (assign92160_e141176, assign92160_e141176_d_n0, assign92160_e141176_d_n2, assign92160_e141176_d_n4, assign92160_e141176_d_n5, assign92160_e141176_d_n6, assign92160_e141176_d_n7, assign92160_e141176_d_n8, assign92160_e141176_d_n9, assign92160_e141176_d_n10, assign92160_e141176_d_n11, assign92160_e141176_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2144 == 0.0)) {
        let assign92160_e141171: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign92160_e141172: f64 = (-assign92160_e141171);
        let assign92160_e141174: f64 = (assign92160_e141172 / locals.var_fac1);
        (assign92160_e141174, ((((-locals.var_vxbgmtcl_dn0) * locals.var_fac1) - (assign92160_e141172 * locals.var_fac1_dn0)) / (locals.var_fac1 * locals.var_fac1)), ((((-(locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2)) * locals.var_fac1) - (assign92160_e141172 * locals.var_fac1_dn2)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn4) * locals.var_fac1) - (assign92160_e141172 * locals.var_fac1_dn4)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn5) * locals.var_fac1) - (assign92160_e141172 * locals.var_fac1_dn5)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn6) * locals.var_fac1) - (assign92160_e141172 * locals.var_fac1_dn6)) / (locals.var_fac1 * locals.var_fac1)), ((((-(locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7)) * locals.var_fac1) - (assign92160_e141172 * locals.var_fac1_dn7)) / (locals.var_fac1 * locals.var_fac1)), ((((-(locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8)) * locals.var_fac1) - (assign92160_e141172 * locals.var_fac1_dn8)) / (locals.var_fac1 * locals.var_fac1)), ((((-(locals.var_vgpld_dn9 + locals.var_vxbgmtcl_dn9)) * locals.var_fac1) - (assign92160_e141172 * locals.var_fac1_dn9)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn10) * locals.var_fac1) - (assign92160_e141172 * locals.var_fac1_dn10)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn11) * locals.var_fac1) - (assign92160_e141172 * locals.var_fac1_dn11)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn14) * locals.var_fac1) - (assign92160_e141172 * locals.var_fac1_dn14)) / (locals.var_fac1 * locals.var_fac1)),)
    } else {
        (locals.var_td, locals.var_td_dn0, locals.var_td_dn2, locals.var_td_dn4, locals.var_td_dn5, locals.var_td_dn6, locals.var_td_dn7, locals.var_td_dn8, locals.var_td_dn9, locals.var_td_dn10, locals.var_td_dn11, locals.var_td_dn14,)
    }
};
        locals.var_td = assign92160_e141176;
        locals.var_td_dn0 = assign92160_e141176_d_n0;
        locals.var_td_dn2 = assign92160_e141176_d_n2;
        locals.var_td_dn4 = assign92160_e141176_d_n4;
        locals.var_td_dn5 = assign92160_e141176_d_n5;
        locals.var_td_dn6 = assign92160_e141176_d_n6;
        locals.var_td_dn7 = assign92160_e141176_d_n7;
        locals.var_td_dn8 = assign92160_e141176_d_n8;
        locals.var_td_dn9 = assign92160_e141176_d_n9;
        locals.var_td_dn10 = assign92160_e141176_d_n10;
        locals.var_td_dn11 = assign92160_e141176_d_n11;
        locals.var_td_dn14 = assign92160_e141176_d_n14;
        locals.var_td_rv = 0.0;

        let (assign92170_e141216, assign92170_e141216_d_n0, assign92170_e141216_d_n2, assign92170_e141216_d_n4, assign92170_e141216_d_n5, assign92170_e141216_d_n6, assign92170_e141216_d_n7, assign92170_e141216_d_n8, assign92170_e141216_d_n9, assign92170_e141216_d_n10, assign92170_e141216_d_n11, assign92170_e141216_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2144 == 0.0)) {
        let assign92170_e141188: f64 = (locals.var_tb * locals.var_tb);
        let assign92170_e141190: f64 = (assign92170_e141188 * locals.var_tb);
        let assign92170_e141193: f64 = (27.0 * locals.var_ta);
        let assign92170_e141195: f64 = (assign92170_e141193 * locals.var_ta);
        let assign92170_e141197: f64 = (assign92170_e141195 * locals.var_ta);
        let assign92170_e141198: f64 = (assign92170_e141190 / assign92170_e141197);
        let assign92170_e141201: f64 = (locals.var_tb * locals.var_tc);
        let assign92170_e141204: f64 = (6.0 * locals.var_ta);
        let assign92170_e141206: f64 = (assign92170_e141204 * locals.var_ta);
        let assign92170_e141207: f64 = (assign92170_e141201 / assign92170_e141206);
        let assign92170_e141208: f64 = (assign92170_e141198 - assign92170_e141207);
        let assign92170_e141212: f64 = (2.0 * locals.var_ta);
        let assign92170_e141213: f64 = (locals.var_td / assign92170_e141212);
        let assign92170_e141214: f64 = (assign92170_e141208 + assign92170_e141213);
        (assign92170_e141214, ((-((locals.var_tb * locals.var_tc_dn0) / assign92170_e141206)) + (locals.var_td_dn0 / assign92170_e141212)), ((-((locals.var_tb * locals.var_tc_dn2) / assign92170_e141206)) + (locals.var_td_dn2 / assign92170_e141212)), ((-((locals.var_tb * locals.var_tc_dn4) / assign92170_e141206)) + (locals.var_td_dn4 / assign92170_e141212)), ((-((locals.var_tb * locals.var_tc_dn5) / assign92170_e141206)) + (locals.var_td_dn5 / assign92170_e141212)), ((-((locals.var_tb * locals.var_tc_dn6) / assign92170_e141206)) + (locals.var_td_dn6 / assign92170_e141212)), ((-((locals.var_tb * locals.var_tc_dn7) / assign92170_e141206)) + (locals.var_td_dn7 / assign92170_e141212)), ((-((locals.var_tb * locals.var_tc_dn8) / assign92170_e141206)) + (locals.var_td_dn8 / assign92170_e141212)), ((-((locals.var_tb * locals.var_tc_dn9) / assign92170_e141206)) + (locals.var_td_dn9 / assign92170_e141212)), ((-((locals.var_tb * locals.var_tc_dn10) / assign92170_e141206)) + (locals.var_td_dn10 / assign92170_e141212)), ((-((locals.var_tb * locals.var_tc_dn11) / assign92170_e141206)) + (locals.var_td_dn11 / assign92170_e141212)), ((-((locals.var_tb * locals.var_tc_dn14) / assign92170_e141206)) + (locals.var_td_dn14 / assign92170_e141212)),)
    } else {
        (locals.var_tq, locals.var_tq_dn0, locals.var_tq_dn2, locals.var_tq_dn4, locals.var_tq_dn5, locals.var_tq_dn6, locals.var_tq_dn7, locals.var_tq_dn8, locals.var_tq_dn9, locals.var_tq_dn10, locals.var_tq_dn11, locals.var_tq_dn14,)
    }
};
        locals.var_tq = assign92170_e141216;
        locals.var_tq_dn0 = assign92170_e141216_d_n0;
        locals.var_tq_dn2 = assign92170_e141216_d_n2;
        locals.var_tq_dn4 = assign92170_e141216_d_n4;
        locals.var_tq_dn5 = assign92170_e141216_d_n5;
        locals.var_tq_dn6 = assign92170_e141216_d_n6;
        locals.var_tq_dn7 = assign92170_e141216_d_n7;
        locals.var_tq_dn8 = assign92170_e141216_d_n8;
        locals.var_tq_dn9 = assign92170_e141216_d_n9;
        locals.var_tq_dn10 = assign92170_e141216_d_n10;
        locals.var_tq_dn11 = assign92170_e141216_d_n11;
        locals.var_tq_dn14 = assign92170_e141216_d_n14;
        locals.var_tq_rv = 0.0;

        let (assign92180_e141242, assign92180_e141242_d_n0, assign92180_e141242_d_n2, assign92180_e141242_d_n4, assign92180_e141242_d_n5, assign92180_e141242_d_n6, assign92180_e141242_d_n7, assign92180_e141242_d_n8, assign92180_e141242_d_n9, assign92180_e141242_d_n10, assign92180_e141242_d_n11, assign92180_e141242_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2144 == 0.0)) {
        let assign92180_e141228: f64 = (3.0 * locals.var_ta);
        let assign92180_e141230: f64 = (assign92180_e141228 * locals.var_tc);
        let assign92180_e141233: f64 = (locals.var_tb * locals.var_tb);
        let assign92180_e141234: f64 = (assign92180_e141230 - assign92180_e141233);
        let assign92180_e141237: f64 = (9.0 * locals.var_ta);
        let assign92180_e141239: f64 = (assign92180_e141237 * locals.var_ta);
        let assign92180_e141240: f64 = (assign92180_e141234 / assign92180_e141239);
        (assign92180_e141240, ((assign92180_e141228 * locals.var_tc_dn0) / assign92180_e141239), ((assign92180_e141228 * locals.var_tc_dn2) / assign92180_e141239), ((assign92180_e141228 * locals.var_tc_dn4) / assign92180_e141239), ((assign92180_e141228 * locals.var_tc_dn5) / assign92180_e141239), ((assign92180_e141228 * locals.var_tc_dn6) / assign92180_e141239), ((assign92180_e141228 * locals.var_tc_dn7) / assign92180_e141239), ((assign92180_e141228 * locals.var_tc_dn8) / assign92180_e141239), ((assign92180_e141228 * locals.var_tc_dn9) / assign92180_e141239), ((assign92180_e141228 * locals.var_tc_dn10) / assign92180_e141239), ((assign92180_e141228 * locals.var_tc_dn11) / assign92180_e141239), ((assign92180_e141228 * locals.var_tc_dn14) / assign92180_e141239),)
    } else {
        (locals.var_tp, locals.var_tp_dn0, locals.var_tp_dn2, locals.var_tp_dn4, locals.var_tp_dn5, locals.var_tp_dn6, locals.var_tp_dn7, locals.var_tp_dn8, locals.var_tp_dn9, locals.var_tp_dn10, locals.var_tp_dn11, locals.var_tp_dn14,)
    }
};
        locals.var_tp = assign92180_e141242;
        locals.var_tp_dn0 = assign92180_e141242_d_n0;
        locals.var_tp_dn2 = assign92180_e141242_d_n2;
        locals.var_tp_dn4 = assign92180_e141242_d_n4;
        locals.var_tp_dn5 = assign92180_e141242_d_n5;
        locals.var_tp_dn6 = assign92180_e141242_d_n6;
        locals.var_tp_dn7 = assign92180_e141242_d_n7;
        locals.var_tp_dn8 = assign92180_e141242_d_n8;
        locals.var_tp_dn9 = assign92180_e141242_d_n9;
        locals.var_tp_dn10 = assign92180_e141242_d_n10;
        locals.var_tp_dn11 = assign92180_e141242_d_n11;
        locals.var_tp_dn14 = assign92180_e141242_d_n14;
        locals.var_tp_rv = 0.0;

        let (assign92190_e141263, assign92190_e141263_d_n0, assign92190_e141263_d_n2, assign92190_e141263_d_n4, assign92190_e141263_d_n5, assign92190_e141263_d_n6, assign92190_e141263_d_n7, assign92190_e141263_d_n8, assign92190_e141263_d_n9, assign92190_e141263_d_n10, assign92190_e141263_d_n11, assign92190_e141263_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2144 == 0.0)) {
        let assign92190_e141254: f64 = (locals.var_tq * locals.var_tq);
        let assign92190_e141257: f64 = (locals.var_tp * locals.var_tp);
        let assign92190_e141259: f64 = (assign92190_e141257 * locals.var_tp);
        let assign92190_e141260: f64 = (assign92190_e141254 + assign92190_e141259);
        let assign92190_e141261: f64 = (assign92190_e141260).sqrt();
        (assign92190_e141261, ((((locals.var_tq_dn0 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn0)) + ((((locals.var_tp_dn0 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn0)) * locals.var_tp) + (assign92190_e141257 * locals.var_tp_dn0))) / (2.0 * assign92190_e141261)), ((((locals.var_tq_dn2 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn2)) + ((((locals.var_tp_dn2 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn2)) * locals.var_tp) + (assign92190_e141257 * locals.var_tp_dn2))) / (2.0 * assign92190_e141261)), ((((locals.var_tq_dn4 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn4)) + ((((locals.var_tp_dn4 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn4)) * locals.var_tp) + (assign92190_e141257 * locals.var_tp_dn4))) / (2.0 * assign92190_e141261)), ((((locals.var_tq_dn5 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn5)) + ((((locals.var_tp_dn5 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn5)) * locals.var_tp) + (assign92190_e141257 * locals.var_tp_dn5))) / (2.0 * assign92190_e141261)), ((((locals.var_tq_dn6 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn6)) + ((((locals.var_tp_dn6 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn6)) * locals.var_tp) + (assign92190_e141257 * locals.var_tp_dn6))) / (2.0 * assign92190_e141261)), ((((locals.var_tq_dn7 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn7)) + ((((locals.var_tp_dn7 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn7)) * locals.var_tp) + (assign92190_e141257 * locals.var_tp_dn7))) / (2.0 * assign92190_e141261)), ((((locals.var_tq_dn8 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn8)) + ((((locals.var_tp_dn8 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn8)) * locals.var_tp) + (assign92190_e141257 * locals.var_tp_dn8))) / (2.0 * assign92190_e141261)), ((((locals.var_tq_dn9 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn9)) + ((((locals.var_tp_dn9 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn9)) * locals.var_tp) + (assign92190_e141257 * locals.var_tp_dn9))) / (2.0 * assign92190_e141261)), ((((locals.var_tq_dn10 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn10)) + ((((locals.var_tp_dn10 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn10)) * locals.var_tp) + (assign92190_e141257 * locals.var_tp_dn10))) / (2.0 * assign92190_e141261)), ((((locals.var_tq_dn11 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn11)) + ((((locals.var_tp_dn11 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn11)) * locals.var_tp) + (assign92190_e141257 * locals.var_tp_dn11))) / (2.0 * assign92190_e141261)), ((((locals.var_tq_dn14 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn14)) + ((((locals.var_tp_dn14 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn14)) * locals.var_tp) + (assign92190_e141257 * locals.var_tp_dn14))) / (2.0 * assign92190_e141261)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign92190_e141263;
        locals.var_t5_dn0 = assign92190_e141263_d_n0;
        locals.var_t5_dn2 = assign92190_e141263_d_n2;
        locals.var_t5_dn4 = assign92190_e141263_d_n4;
        locals.var_t5_dn5 = assign92190_e141263_d_n5;
        locals.var_t5_dn6 = assign92190_e141263_d_n6;
        locals.var_t5_dn7 = assign92190_e141263_d_n7;
        locals.var_t5_dn8 = assign92190_e141263_d_n8;
        locals.var_t5_dn9 = assign92190_e141263_d_n9;
        locals.var_t5_dn10 = assign92190_e141263_d_n10;
        locals.var_t5_dn11 = assign92190_e141263_d_n11;
        locals.var_t5_dn14 = assign92190_e141263_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign92200_e141280, assign92200_e141280_d_n0, assign92200_e141280_d_n2, assign92200_e141280_d_n4, assign92200_e141280_d_n5, assign92200_e141280_d_n6, assign92200_e141280_d_n7, assign92200_e141280_d_n8, assign92200_e141280_d_n9, assign92200_e141280_d_n10, assign92200_e141280_d_n11, assign92200_e141280_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2144 == 0.0)) {
        let assign92200_e141274: f64 = (-locals.var_tq);
        let assign92200_e141276: f64 = (assign92200_e141274 + locals.var_t5);
        let assign92200_e141278: f64 = (assign92200_e141276).powf(0.3333333333333333);
        (assign92200_e141278, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign92200_e141276).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn0) + locals.var_t5_dn0))) } } else { (assign92200_e141278 * (0.3333333333333333 * (((-locals.var_tq_dn0) + locals.var_t5_dn0) / assign92200_e141276))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign92200_e141276).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn2) + locals.var_t5_dn2))) } } else { (assign92200_e141278 * (0.3333333333333333 * (((-locals.var_tq_dn2) + locals.var_t5_dn2) / assign92200_e141276))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign92200_e141276).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn4) + locals.var_t5_dn4))) } } else { (assign92200_e141278 * (0.3333333333333333 * (((-locals.var_tq_dn4) + locals.var_t5_dn4) / assign92200_e141276))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign92200_e141276).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn5) + locals.var_t5_dn5))) } } else { (assign92200_e141278 * (0.3333333333333333 * (((-locals.var_tq_dn5) + locals.var_t5_dn5) / assign92200_e141276))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign92200_e141276).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn6) + locals.var_t5_dn6))) } } else { (assign92200_e141278 * (0.3333333333333333 * (((-locals.var_tq_dn6) + locals.var_t5_dn6) / assign92200_e141276))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign92200_e141276).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn7) + locals.var_t5_dn7))) } } else { (assign92200_e141278 * (0.3333333333333333 * (((-locals.var_tq_dn7) + locals.var_t5_dn7) / assign92200_e141276))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign92200_e141276).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn8) + locals.var_t5_dn8))) } } else { (assign92200_e141278 * (0.3333333333333333 * (((-locals.var_tq_dn8) + locals.var_t5_dn8) / assign92200_e141276))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign92200_e141276).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn9) + locals.var_t5_dn9))) } } else { (assign92200_e141278 * (0.3333333333333333 * (((-locals.var_tq_dn9) + locals.var_t5_dn9) / assign92200_e141276))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign92200_e141276).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn10) + locals.var_t5_dn10))) } } else { (assign92200_e141278 * (0.3333333333333333 * (((-locals.var_tq_dn10) + locals.var_t5_dn10) / assign92200_e141276))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign92200_e141276).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn11) + locals.var_t5_dn11))) } } else { (assign92200_e141278 * (0.3333333333333333 * (((-locals.var_tq_dn11) + locals.var_t5_dn11) / assign92200_e141276))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign92200_e141276).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn14) + locals.var_t5_dn14))) } } else { (assign92200_e141278 * (0.3333333333333333 * (((-locals.var_tq_dn14) + locals.var_t5_dn14) / assign92200_e141276))) },)
    } else {
        (locals.var_tu, locals.var_tu_dn0, locals.var_tu_dn2, locals.var_tu_dn4, locals.var_tu_dn5, locals.var_tu_dn6, locals.var_tu_dn7, locals.var_tu_dn8, locals.var_tu_dn9, locals.var_tu_dn10, locals.var_tu_dn11, locals.var_tu_dn14,)
    }
};
        locals.var_tu = assign92200_e141280;
        locals.var_tu_dn0 = assign92200_e141280_d_n0;
        locals.var_tu_dn2 = assign92200_e141280_d_n2;
        locals.var_tu_dn4 = assign92200_e141280_d_n4;
        locals.var_tu_dn5 = assign92200_e141280_d_n5;
        locals.var_tu_dn6 = assign92200_e141280_d_n6;
        locals.var_tu_dn7 = assign92200_e141280_d_n7;
        locals.var_tu_dn8 = assign92200_e141280_d_n8;
        locals.var_tu_dn9 = assign92200_e141280_d_n9;
        locals.var_tu_dn10 = assign92200_e141280_d_n10;
        locals.var_tu_dn11 = assign92200_e141280_d_n11;
        locals.var_tu_dn14 = assign92200_e141280_d_n14;
        locals.var_tu_rv = 0.0;

        let (assign92210_e141297, assign92210_e141297_d_n0, assign92210_e141297_d_n2, assign92210_e141297_d_n4, assign92210_e141297_d_n5, assign92210_e141297_d_n6, assign92210_e141297_d_n7, assign92210_e141297_d_n8, assign92210_e141297_d_n9, assign92210_e141297_d_n10, assign92210_e141297_d_n11, assign92210_e141297_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2144 == 0.0)) {
        let assign92210_e141292: f64 = (locals.var_tq + locals.var_t5);
        let assign92210_e141294: f64 = (assign92210_e141292).powf(0.3333333333333333);
        let assign92210_e141295: f64 = (-assign92210_e141294);
        (assign92210_e141295, (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign92210_e141292).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn0 + locals.var_t5_dn0))) } } else { (assign92210_e141294 * (0.3333333333333333 * ((locals.var_tq_dn0 + locals.var_t5_dn0) / assign92210_e141292))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign92210_e141292).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn2 + locals.var_t5_dn2))) } } else { (assign92210_e141294 * (0.3333333333333333 * ((locals.var_tq_dn2 + locals.var_t5_dn2) / assign92210_e141292))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign92210_e141292).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn4 + locals.var_t5_dn4))) } } else { (assign92210_e141294 * (0.3333333333333333 * ((locals.var_tq_dn4 + locals.var_t5_dn4) / assign92210_e141292))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign92210_e141292).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn5 + locals.var_t5_dn5))) } } else { (assign92210_e141294 * (0.3333333333333333 * ((locals.var_tq_dn5 + locals.var_t5_dn5) / assign92210_e141292))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign92210_e141292).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn6 + locals.var_t5_dn6))) } } else { (assign92210_e141294 * (0.3333333333333333 * ((locals.var_tq_dn6 + locals.var_t5_dn6) / assign92210_e141292))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign92210_e141292).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn7 + locals.var_t5_dn7))) } } else { (assign92210_e141294 * (0.3333333333333333 * ((locals.var_tq_dn7 + locals.var_t5_dn7) / assign92210_e141292))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign92210_e141292).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn8 + locals.var_t5_dn8))) } } else { (assign92210_e141294 * (0.3333333333333333 * ((locals.var_tq_dn8 + locals.var_t5_dn8) / assign92210_e141292))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign92210_e141292).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn9 + locals.var_t5_dn9))) } } else { (assign92210_e141294 * (0.3333333333333333 * ((locals.var_tq_dn9 + locals.var_t5_dn9) / assign92210_e141292))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign92210_e141292).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn10 + locals.var_t5_dn10))) } } else { (assign92210_e141294 * (0.3333333333333333 * ((locals.var_tq_dn10 + locals.var_t5_dn10) / assign92210_e141292))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign92210_e141292).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn11 + locals.var_t5_dn11))) } } else { (assign92210_e141294 * (0.3333333333333333 * ((locals.var_tq_dn11 + locals.var_t5_dn11) / assign92210_e141292))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign92210_e141292).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn14 + locals.var_t5_dn14))) } } else { (assign92210_e141294 * (0.3333333333333333 * ((locals.var_tq_dn14 + locals.var_t5_dn14) / assign92210_e141292))) }),)
    } else {
        (locals.var_tv, locals.var_tv_dn0, locals.var_tv_dn2, locals.var_tv_dn4, locals.var_tv_dn5, locals.var_tv_dn6, locals.var_tv_dn7, locals.var_tv_dn8, locals.var_tv_dn9, locals.var_tv_dn10, locals.var_tv_dn11, locals.var_tv_dn14,)
    }
};
        locals.var_tv = assign92210_e141297;
        locals.var_tv_dn0 = assign92210_e141297_d_n0;
        locals.var_tv_dn2 = assign92210_e141297_d_n2;
        locals.var_tv_dn4 = assign92210_e141297_d_n4;
        locals.var_tv_dn5 = assign92210_e141297_d_n5;
        locals.var_tv_dn6 = assign92210_e141297_d_n6;
        locals.var_tv_dn7 = assign92210_e141297_d_n7;
        locals.var_tv_dn8 = assign92210_e141297_d_n8;
        locals.var_tv_dn9 = assign92210_e141297_d_n9;
        locals.var_tv_dn10 = assign92210_e141297_d_n10;
        locals.var_tv_dn11 = assign92210_e141297_d_n11;
        locals.var_tv_dn14 = assign92210_e141297_d_n14;
        locals.var_tv_rv = 0.0;

        let (assign92220_e141317, assign92220_e141317_d_n0, assign92220_e141317_d_n2, assign92220_e141317_d_n4, assign92220_e141317_d_n5, assign92220_e141317_d_n6, assign92220_e141317_d_n7, assign92220_e141317_d_n8, assign92220_e141317_d_n9, assign92220_e141317_d_n10, assign92220_e141317_d_n11, assign92220_e141317_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2144 == 0.0)) {
        let assign92220_e141309: f64 = (locals.var_tu + locals.var_tv);
        let assign92220_e141313: f64 = (3.0 * locals.var_ta);
        let assign92220_e141314: f64 = (locals.var_tb / assign92220_e141313);
        let assign92220_e141315: f64 = (assign92220_e141309 - assign92220_e141314);
        (assign92220_e141315, (locals.var_tu_dn0 + locals.var_tv_dn0), (locals.var_tu_dn2 + locals.var_tv_dn2), (locals.var_tu_dn4 + locals.var_tv_dn4), (locals.var_tu_dn5 + locals.var_tv_dn5), (locals.var_tu_dn6 + locals.var_tv_dn6), (locals.var_tu_dn7 + locals.var_tv_dn7), (locals.var_tu_dn8 + locals.var_tv_dn8), (locals.var_tu_dn9 + locals.var_tv_dn9), (locals.var_tu_dn10 + locals.var_tv_dn10), (locals.var_tu_dn11 + locals.var_tv_dn11), (locals.var_tu_dn14 + locals.var_tv_dn14),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
        locals.var_chi = assign92220_e141317;
        locals.var_chi_dn0 = assign92220_e141317_d_n0;
        locals.var_chi_dn2 = assign92220_e141317_d_n2;
        locals.var_chi_dn4 = assign92220_e141317_d_n4;
        locals.var_chi_dn5 = assign92220_e141317_d_n5;
        locals.var_chi_dn6 = assign92220_e141317_d_n6;
        locals.var_chi_dn7 = assign92220_e141317_d_n7;
        locals.var_chi_dn8 = assign92220_e141317_d_n8;
        locals.var_chi_dn9 = assign92220_e141317_d_n9;
        locals.var_chi_dn10 = assign92220_e141317_d_n10;
        locals.var_chi_dn11 = assign92220_e141317_d_n11;
        locals.var_chi_dn14 = assign92220_e141317_d_n14;
        locals.var_chi_rv = 0.0;

        let (assign92230_e141333, assign92230_e141333_d_n0, assign92230_e141333_d_n2, assign92230_e141333_d_n4, assign92230_e141333_d_n5, assign92230_e141333_d_n6, assign92230_e141333_d_n7, assign92230_e141333_d_n8, assign92230_e141333_d_n9, assign92230_e141333_d_n10, assign92230_e141333_d_n11, assign92230_e141333_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2144 == 0.0)) {
        let assign92230_e141329: f64 = (locals.var_chi * locals.var_beta_inv);
        let assign92230_e141331: f64 = (assign92230_e141329 - locals.var_vxbgmtcl);
        (assign92230_e141331, (((locals.var_chi_dn0 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn0)) - locals.var_vxbgmtcl_dn0), (((locals.var_chi_dn2 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn2)) - locals.var_vxbgmtcl_dn2), (((locals.var_chi_dn4 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn4)) - locals.var_vxbgmtcl_dn4), (((locals.var_chi_dn5 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn5)) - locals.var_vxbgmtcl_dn5), (((locals.var_chi_dn6 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn6)) - locals.var_vxbgmtcl_dn6), (((locals.var_chi_dn7 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn7)) - locals.var_vxbgmtcl_dn7), (((locals.var_chi_dn8 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn8)) - locals.var_vxbgmtcl_dn8), (((locals.var_chi_dn9 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn9)) - locals.var_vxbgmtcl_dn9), (((locals.var_chi_dn10 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn10)) - locals.var_vxbgmtcl_dn10), (((locals.var_chi_dn11 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn11)) - locals.var_vxbgmtcl_dn11), (((locals.var_chi_dn14 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn14)) - locals.var_vxbgmtcl_dn14),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn14,)
    }
};
        locals.var_ps0_inia = assign92230_e141333;
        locals.var_ps0_inia_dn0 = assign92230_e141333_d_n0;
        locals.var_ps0_inia_dn2 = assign92230_e141333_d_n2;
        locals.var_ps0_inia_dn4 = assign92230_e141333_d_n4;
        locals.var_ps0_inia_dn5 = assign92230_e141333_d_n5;
        locals.var_ps0_inia_dn6 = assign92230_e141333_d_n6;
        locals.var_ps0_inia_dn7 = assign92230_e141333_d_n7;
        locals.var_ps0_inia_dn8 = assign92230_e141333_d_n8;
        locals.var_ps0_inia_dn9 = assign92230_e141333_d_n9;
        locals.var_ps0_inia_dn10 = assign92230_e141333_d_n10;
        locals.var_ps0_inia_dn11 = assign92230_e141333_d_n11;
        locals.var_ps0_inia_dn14 = assign92230_e141333_d_n14;
        locals.var_ps0_inia_rv = 0.0;

        let assign92240_e141336: f64 = if p.p33 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2145 = assign92240_e141336;
        locals.var_guard2145_rv = 0.0;

        let (assign92250_e141351, assign92250_e141351_d_n0, assign92250_e141351_d_n2, assign92250_e141351_d_n4, assign92250_e141351_d_n5, assign92250_e141351_d_n6, assign92250_e141351_d_n7, assign92250_e141351_d_n8, assign92250_e141351_d_n9, assign92250_e141351_d_n10, assign92250_e141351_d_n11, assign92250_e141351_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2145 != 0.0)) {
        let assign92250_e141347: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign92250_e141349: f64 = (assign92250_e141347 + 0.1);
        (assign92250_e141349, locals.var_vxbgmtcl_dn0, (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2), locals.var_vxbgmtcl_dn4, locals.var_vxbgmtcl_dn5, locals.var_vxbgmtcl_dn6, (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7), (locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8), (locals.var_vgpld_dn9 + locals.var_vxbgmtcl_dn9), locals.var_vxbgmtcl_dn10, locals.var_vxbgmtcl_dn11, locals.var_vxbgmtcl_dn14,)
    } else {
        (locals.var_vgpld_shift, locals.var_vgpld_shift_dn0, locals.var_vgpld_shift_dn2, locals.var_vgpld_shift_dn4, locals.var_vgpld_shift_dn5, locals.var_vgpld_shift_dn6, locals.var_vgpld_shift_dn7, locals.var_vgpld_shift_dn8, locals.var_vgpld_shift_dn9, locals.var_vgpld_shift_dn10, locals.var_vgpld_shift_dn11, locals.var_vgpld_shift_dn14,)
    }
};
        locals.var_vgpld_shift = assign92250_e141351;
        locals.var_vgpld_shift_dn0 = assign92250_e141351_d_n0;
        locals.var_vgpld_shift_dn2 = assign92250_e141351_d_n2;
        locals.var_vgpld_shift_dn4 = assign92250_e141351_d_n4;
        locals.var_vgpld_shift_dn5 = assign92250_e141351_d_n5;
        locals.var_vgpld_shift_dn6 = assign92250_e141351_d_n6;
        locals.var_vgpld_shift_dn7 = assign92250_e141351_d_n7;
        locals.var_vgpld_shift_dn8 = assign92250_e141351_d_n8;
        locals.var_vgpld_shift_dn9 = assign92250_e141351_d_n9;
        locals.var_vgpld_shift_dn10 = assign92250_e141351_d_n10;
        locals.var_vgpld_shift_dn11 = assign92250_e141351_d_n11;
        locals.var_vgpld_shift_dn14 = assign92250_e141351_d_n14;
        locals.var_vgpld_shift_rv = 0.0;

        let (assign92260_e141364, assign92260_e141364_d_n0, assign92260_e141364_d_n2, assign92260_e141364_d_n4, assign92260_e141364_d_n5, assign92260_e141364_d_n6, assign92260_e141364_d_n7, assign92260_e141364_d_n8, assign92260_e141364_d_n9, assign92260_e141364_d_n10, assign92260_e141364_d_n11, assign92260_e141364_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2145 != 0.0)) {
        let assign92260_e141362: f64 = (locals.var_cnst1over * locals.var_exp_bvbs);
        (assign92260_e141362, ((locals.var_cnst1over_dn0 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn0)), ((locals.var_cnst1over_dn2 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn2)), ((locals.var_cnst1over_dn4 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn4)), ((locals.var_cnst1over_dn5 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn5)), ((locals.var_cnst1over_dn6 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn6)), ((locals.var_cnst1over_dn7 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn7)), ((locals.var_cnst1over_dn8 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn8)), ((locals.var_cnst1over_dn9 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn9)), ((locals.var_cnst1over_dn10 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn10)), ((locals.var_cnst1over_dn11 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn11)), ((locals.var_cnst1over_dn14 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn14)),)
    } else {
        (locals.var_cfs1, locals.var_cfs1_dn0, locals.var_cfs1_dn2, locals.var_cfs1_dn4, locals.var_cfs1_dn5, locals.var_cfs1_dn6, locals.var_cfs1_dn7, locals.var_cfs1_dn8, locals.var_cfs1_dn9, locals.var_cfs1_dn10, locals.var_cfs1_dn11, locals.var_cfs1_dn14,)
    }
};
        locals.var_cfs1 = assign92260_e141364;
        locals.var_cfs1_dn0 = assign92260_e141364_d_n0;
        locals.var_cfs1_dn2 = assign92260_e141364_d_n2;
        locals.var_cfs1_dn4 = assign92260_e141364_d_n4;
        locals.var_cfs1_dn5 = assign92260_e141364_d_n5;
        locals.var_cfs1_dn6 = assign92260_e141364_d_n6;
        locals.var_cfs1_dn7 = assign92260_e141364_d_n7;
        locals.var_cfs1_dn8 = assign92260_e141364_d_n8;
        locals.var_cfs1_dn9 = assign92260_e141364_d_n9;
        locals.var_cfs1_dn10 = assign92260_e141364_d_n10;
        locals.var_cfs1_dn11 = assign92260_e141364_d_n11;
        locals.var_cfs1_dn14 = assign92260_e141364_d_n14;
        locals.var_cfs1_rv = 0.0;

        let (assign92270_e141377, assign92270_e141377_d_n0, assign92270_e141377_d_n2, assign92270_e141377_d_n4, assign92270_e141377_d_n5, assign92270_e141377_d_n6, assign92270_e141377_d_n7, assign92270_e141377_d_n8, assign92270_e141377_d_n9, assign92270_e141377_d_n10, assign92270_e141377_d_n11, assign92270_e141377_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2145 != 0.0)) {
        let assign92270_e141375: f64 = (locals.var_cnst1over * locals.var_exp_bvbs);
        (assign92270_e141375, ((locals.var_cnst1over_dn0 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn0)), ((locals.var_cnst1over_dn2 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn2)), ((locals.var_cnst1over_dn4 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn4)), ((locals.var_cnst1over_dn5 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn5)), ((locals.var_cnst1over_dn6 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn6)), ((locals.var_cnst1over_dn7 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn7)), ((locals.var_cnst1over_dn8 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn8)), ((locals.var_cnst1over_dn9 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn9)), ((locals.var_cnst1over_dn10 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn10)), ((locals.var_cnst1over_dn11 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn11)), ((locals.var_cnst1over_dn14 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn14)),)
    } else {
        (locals.var_gammachi, locals.var_gammachi_dn0, locals.var_gammachi_dn2, locals.var_gammachi_dn4, locals.var_gammachi_dn5, locals.var_gammachi_dn6, locals.var_gammachi_dn7, locals.var_gammachi_dn8, locals.var_gammachi_dn9, locals.var_gammachi_dn10, locals.var_gammachi_dn11, locals.var_gammachi_dn14,)
    }
};
        locals.var_gammachi = assign92270_e141377;
        locals.var_gammachi_dn0 = assign92270_e141377_d_n0;
        locals.var_gammachi_dn2 = assign92270_e141377_d_n2;
        locals.var_gammachi_dn4 = assign92270_e141377_d_n4;
        locals.var_gammachi_dn5 = assign92270_e141377_d_n5;
        locals.var_gammachi_dn6 = assign92270_e141377_d_n6;
        locals.var_gammachi_dn7 = assign92270_e141377_d_n7;
        locals.var_gammachi_dn8 = assign92270_e141377_d_n8;
        locals.var_gammachi_dn9 = assign92270_e141377_d_n9;
        locals.var_gammachi_dn10 = assign92270_e141377_d_n10;
        locals.var_gammachi_dn11 = assign92270_e141377_d_n11;
        locals.var_gammachi_dn14 = assign92270_e141377_d_n14;
        locals.var_gammachi_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_356(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign92280_e141390, assign92280_e141390_d_n0, assign92280_e141390_d_n2, assign92280_e141390_d_n4, assign92280_e141390_d_n5, assign92280_e141390_d_n6, assign92280_e141390_d_n7, assign92280_e141390_d_n8, assign92280_e141390_d_n9, assign92280_e141390_d_n10, assign92280_e141390_d_n11, assign92280_e141390_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2145 != 0.0)) {
        let assign92280_e141388: f64 = (locals.var_beta2 * locals.var_fac1p2);
        (assign92280_e141388, ((locals.var_beta2_dn0 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn0)), ((locals.var_beta2_dn2 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn2)), ((locals.var_beta2_dn4 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn4)), ((locals.var_beta2_dn5 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn5)), ((locals.var_beta2_dn6 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn6)), ((locals.var_beta2_dn7 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn7)), ((locals.var_beta2_dn8 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn8)), ((locals.var_beta2_dn9 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn9)), ((locals.var_beta2_dn10 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn10)), ((locals.var_beta2_dn11 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn11)), ((locals.var_beta2_dn14 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign92280_e141390;
        locals.var_t0_dn0 = assign92280_e141390_d_n0;
        locals.var_t0_dn2 = assign92280_e141390_d_n2;
        locals.var_t0_dn4 = assign92280_e141390_d_n4;
        locals.var_t0_dn5 = assign92280_e141390_d_n5;
        locals.var_t0_dn6 = assign92280_e141390_d_n6;
        locals.var_t0_dn7 = assign92280_e141390_d_n7;
        locals.var_t0_dn8 = assign92280_e141390_d_n8;
        locals.var_t0_dn9 = assign92280_e141390_d_n9;
        locals.var_t0_dn10 = assign92280_e141390_d_n10;
        locals.var_t0_dn11 = assign92280_e141390_d_n11;
        locals.var_t0_dn14 = assign92280_e141390_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign92290_e141403, assign92290_e141403_d_n0, assign92290_e141403_d_n2, assign92290_e141403_d_n4, assign92290_e141403_d_n5, assign92290_e141403_d_n6, assign92290_e141403_d_n7, assign92290_e141403_d_n8, assign92290_e141403_d_n9, assign92290_e141403_d_n10, assign92290_e141403_d_n11, assign92290_e141403_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2145 != 0.0)) {
        let assign92290_e141401: f64 = (locals.var_beta * locals.var_vgpld_shift);
        (assign92290_e141401, ((locals.var_beta_dn0 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn0)), ((locals.var_beta_dn2 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn2)), ((locals.var_beta_dn4 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn4)), ((locals.var_beta_dn5 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn5)), ((locals.var_beta_dn6 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn6)), ((locals.var_beta_dn7 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn7)), ((locals.var_beta_dn8 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn8)), ((locals.var_beta_dn9 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn9)), ((locals.var_beta_dn10 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn10)), ((locals.var_beta_dn11 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn11)), ((locals.var_beta_dn14 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn14)),)
    } else {
        (locals.var_psi, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn4, locals.var_psi_dn5, locals.var_psi_dn6, locals.var_psi_dn7, locals.var_psi_dn8, locals.var_psi_dn9, locals.var_psi_dn10, locals.var_psi_dn11, locals.var_psi_dn14,)
    }
};
        locals.var_psi = assign92290_e141403;
        locals.var_psi_dn0 = assign92290_e141403_d_n0;
        locals.var_psi_dn2 = assign92290_e141403_d_n2;
        locals.var_psi_dn4 = assign92290_e141403_d_n4;
        locals.var_psi_dn5 = assign92290_e141403_d_n5;
        locals.var_psi_dn6 = assign92290_e141403_d_n6;
        locals.var_psi_dn7 = assign92290_e141403_d_n7;
        locals.var_psi_dn8 = assign92290_e141403_d_n8;
        locals.var_psi_dn9 = assign92290_e141403_d_n9;
        locals.var_psi_dn10 = assign92290_e141403_d_n10;
        locals.var_psi_dn11 = assign92290_e141403_d_n11;
        locals.var_psi_dn14 = assign92290_e141403_d_n14;
        locals.var_psi_rv = 0.0;

        let (assign92300_e141430, assign92300_e141430_d_n0, assign92300_e141430_d_n2, assign92300_e141430_d_n4, assign92300_e141430_d_n5, assign92300_e141430_d_n6, assign92300_e141430_d_n7, assign92300_e141430_d_n8, assign92300_e141430_d_n9, assign92300_e141430_d_n10, assign92300_e141430_d_n11, assign92300_e141430_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2145 != 0.0)) {
        let assign92300_e141414: f64 = (locals.var_gammachi * locals.var_t0);
        let assign92300_e141417: f64 = (locals.var_psi * locals.var_psi);
        let assign92300_e141418: f64 = (assign92300_e141414 + assign92300_e141417);
        let assign92300_e141419: f64 = (assign92300_e141418).ln();
        let assign92300_e141422: f64 = (locals.var_cnst1over * locals.var_t0);
        let assign92300_e141423: f64 = (assign92300_e141422).ln();
        let assign92300_e141424: f64 = (assign92300_e141419 - assign92300_e141423);
        let assign92300_e141427: f64 = (locals.var_beta * locals.var_vxbgmtcl);
        let assign92300_e141428: f64 = (assign92300_e141424 + assign92300_e141427);
        (assign92300_e141428, ((((((locals.var_gammachi_dn0 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn0)) + ((locals.var_psi_dn0 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn0))) / assign92300_e141418) - (((locals.var_cnst1over_dn0 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn0)) / assign92300_e141422)) + ((locals.var_beta_dn0 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn0))), ((((((locals.var_gammachi_dn2 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn2)) + ((locals.var_psi_dn2 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn2))) / assign92300_e141418) - (((locals.var_cnst1over_dn2 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn2)) / assign92300_e141422)) + ((locals.var_beta_dn2 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn2))), ((((((locals.var_gammachi_dn4 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn4)) + ((locals.var_psi_dn4 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn4))) / assign92300_e141418) - (((locals.var_cnst1over_dn4 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn4)) / assign92300_e141422)) + ((locals.var_beta_dn4 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn4))), ((((((locals.var_gammachi_dn5 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn5)) + ((locals.var_psi_dn5 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn5))) / assign92300_e141418) - (((locals.var_cnst1over_dn5 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn5)) / assign92300_e141422)) + ((locals.var_beta_dn5 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn5))), ((((((locals.var_gammachi_dn6 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn6)) + ((locals.var_psi_dn6 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn6))) / assign92300_e141418) - (((locals.var_cnst1over_dn6 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn6)) / assign92300_e141422)) + ((locals.var_beta_dn6 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn6))), ((((((locals.var_gammachi_dn7 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn7)) + ((locals.var_psi_dn7 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn7))) / assign92300_e141418) - (((locals.var_cnst1over_dn7 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn7)) / assign92300_e141422)) + ((locals.var_beta_dn7 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn7))), ((((((locals.var_gammachi_dn8 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn8)) + ((locals.var_psi_dn8 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn8))) / assign92300_e141418) - (((locals.var_cnst1over_dn8 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn8)) / assign92300_e141422)) + ((locals.var_beta_dn8 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn8))), ((((((locals.var_gammachi_dn9 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn9)) + ((locals.var_psi_dn9 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn9))) / assign92300_e141418) - (((locals.var_cnst1over_dn9 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn9)) / assign92300_e141422)) + ((locals.var_beta_dn9 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn9))), ((((((locals.var_gammachi_dn10 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn10)) + ((locals.var_psi_dn10 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn10))) / assign92300_e141418) - (((locals.var_cnst1over_dn10 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn10)) / assign92300_e141422)) + ((locals.var_beta_dn10 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn10))), ((((((locals.var_gammachi_dn11 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn11)) + ((locals.var_psi_dn11 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn11))) / assign92300_e141418) - (((locals.var_cnst1over_dn11 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn11)) / assign92300_e141422)) + ((locals.var_beta_dn11 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn11))), ((((((locals.var_gammachi_dn14 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn14)) + ((locals.var_psi_dn14 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn14))) / assign92300_e141418) - (((locals.var_cnst1over_dn14 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn14)) / assign92300_e141422)) + ((locals.var_beta_dn14 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn14))),)
    } else {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn11, locals.var_chi_1_dn14,)
    }
};
        locals.var_chi_1 = assign92300_e141430;
        locals.var_chi_1_dn0 = assign92300_e141430_d_n0;
        locals.var_chi_1_dn2 = assign92300_e141430_d_n2;
        locals.var_chi_1_dn4 = assign92300_e141430_d_n4;
        locals.var_chi_1_dn5 = assign92300_e141430_d_n5;
        locals.var_chi_1_dn6 = assign92300_e141430_d_n6;
        locals.var_chi_1_dn7 = assign92300_e141430_d_n7;
        locals.var_chi_1_dn8 = assign92300_e141430_d_n8;
        locals.var_chi_1_dn9 = assign92300_e141430_d_n9;
        locals.var_chi_1_dn10 = assign92300_e141430_d_n10;
        locals.var_chi_1_dn11 = assign92300_e141430_d_n11;
        locals.var_chi_1_dn14 = assign92300_e141430_d_n14;
        locals.var_chi_1_rv = 0.0;

        let assign92310_e141433: f64 = if p.p33 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard2146 = assign92310_e141433;
        locals.var_guard2146_rv = 0.0;

        let (assign92320_e141450, assign92320_e141450_d_n0, assign92320_e141450_d_n2, assign92320_e141450_d_n4, assign92320_e141450_d_n5, assign92320_e141450_d_n6, assign92320_e141450_d_n7, assign92320_e141450_d_n8, assign92320_e141450_d_n9, assign92320_e141450_d_n10, assign92320_e141450_d_n11, assign92320_e141450_d_n14,) = {
    if (((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2145 != 0.0)) && (locals.var_guard2146 != 0.0)) {
        let assign92320_e141446: f64 = (locals.var_psi - locals.var_chi_1);
        let assign92320_e141448: f64 = (assign92320_e141446 - 1.0);
        (assign92320_e141448, (locals.var_psi_dn0 - locals.var_chi_1_dn0), (locals.var_psi_dn2 - locals.var_chi_1_dn2), (locals.var_psi_dn4 - locals.var_chi_1_dn4), (locals.var_psi_dn5 - locals.var_chi_1_dn5), (locals.var_psi_dn6 - locals.var_chi_1_dn6), (locals.var_psi_dn7 - locals.var_chi_1_dn7), (locals.var_psi_dn8 - locals.var_chi_1_dn8), (locals.var_psi_dn9 - locals.var_chi_1_dn9), (locals.var_psi_dn10 - locals.var_chi_1_dn10), (locals.var_psi_dn11 - locals.var_chi_1_dn11), (locals.var_psi_dn14 - locals.var_chi_1_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign92320_e141450;
        locals.var_tmf1_dn0 = assign92320_e141450_d_n0;
        locals.var_tmf1_dn2 = assign92320_e141450_d_n2;
        locals.var_tmf1_dn4 = assign92320_e141450_d_n4;
        locals.var_tmf1_dn5 = assign92320_e141450_d_n5;
        locals.var_tmf1_dn6 = assign92320_e141450_d_n6;
        locals.var_tmf1_dn7 = assign92320_e141450_d_n7;
        locals.var_tmf1_dn8 = assign92320_e141450_d_n8;
        locals.var_tmf1_dn9 = assign92320_e141450_d_n9;
        locals.var_tmf1_dn10 = assign92320_e141450_d_n10;
        locals.var_tmf1_dn11 = assign92320_e141450_d_n11;
        locals.var_tmf1_dn14 = assign92320_e141450_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign92330_e141467, assign92330_e141467_d_n0, assign92330_e141467_d_n2, assign92330_e141467_d_n4, assign92330_e141467_d_n5, assign92330_e141467_d_n6, assign92330_e141467_d_n7, assign92330_e141467_d_n8, assign92330_e141467_d_n9, assign92330_e141467_d_n10, assign92330_e141467_d_n11, assign92330_e141467_d_n14,) = {
    if (((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2145 != 0.0)) && (locals.var_guard2146 != 0.0)) {
        let assign92330_e141463: f64 = (4.0 * locals.var_psi);
        let assign92330_e141465: f64 = assign92330_e141463;
        (assign92330_e141465, (4.0 * locals.var_psi_dn0), (4.0 * locals.var_psi_dn2), (4.0 * locals.var_psi_dn4), (4.0 * locals.var_psi_dn5), (4.0 * locals.var_psi_dn6), (4.0 * locals.var_psi_dn7), (4.0 * locals.var_psi_dn8), (4.0 * locals.var_psi_dn9), (4.0 * locals.var_psi_dn10), (4.0 * locals.var_psi_dn11), (4.0 * locals.var_psi_dn14),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign92330_e141467;
        locals.var_tmf2_dn0 = assign92330_e141467_d_n0;
        locals.var_tmf2_dn2 = assign92330_e141467_d_n2;
        locals.var_tmf2_dn4 = assign92330_e141467_d_n4;
        locals.var_tmf2_dn5 = assign92330_e141467_d_n5;
        locals.var_tmf2_dn6 = assign92330_e141467_d_n6;
        locals.var_tmf2_dn7 = assign92330_e141467_d_n7;
        locals.var_tmf2_dn8 = assign92330_e141467_d_n8;
        locals.var_tmf2_dn9 = assign92330_e141467_d_n9;
        locals.var_tmf2_dn10 = assign92330_e141467_d_n10;
        locals.var_tmf2_dn11 = assign92330_e141467_d_n11;
        locals.var_tmf2_dn14 = assign92330_e141467_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign92340_e141486, assign92340_e141486_d_n0, assign92340_e141486_d_n2, assign92340_e141486_d_n4, assign92340_e141486_d_n5, assign92340_e141486_d_n6, assign92340_e141486_d_n7, assign92340_e141486_d_n8, assign92340_e141486_d_n9, assign92340_e141486_d_n10, assign92340_e141486_d_n11, assign92340_e141486_d_n14,) = {
    if (((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2145 != 0.0)) && (locals.var_guard2146 != 0.0)) {
        let (assign92340_e141484, assign92340_e141484_d_n0, assign92340_e141484_d_n2, assign92340_e141484_d_n4, assign92340_e141484_d_n5, assign92340_e141484_d_n6, assign92340_e141484_d_n7, assign92340_e141484_d_n8, assign92340_e141484_d_n9, assign92340_e141484_d_n10, assign92340_e141484_d_n11, assign92340_e141484_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign92340_e141483: f64 = (-locals.var_tmf2);
                (assign92340_e141483, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign92340_e141484, assign92340_e141484_d_n0, assign92340_e141484_d_n2, assign92340_e141484_d_n4, assign92340_e141484_d_n5, assign92340_e141484_d_n6, assign92340_e141484_d_n7, assign92340_e141484_d_n8, assign92340_e141484_d_n9, assign92340_e141484_d_n10, assign92340_e141484_d_n11, assign92340_e141484_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign92340_e141486;
        locals.var_tmf2_dn0 = assign92340_e141486_d_n0;
        locals.var_tmf2_dn2 = assign92340_e141486_d_n2;
        locals.var_tmf2_dn4 = assign92340_e141486_d_n4;
        locals.var_tmf2_dn5 = assign92340_e141486_d_n5;
        locals.var_tmf2_dn6 = assign92340_e141486_d_n6;
        locals.var_tmf2_dn7 = assign92340_e141486_d_n7;
        locals.var_tmf2_dn8 = assign92340_e141486_d_n8;
        locals.var_tmf2_dn9 = assign92340_e141486_d_n9;
        locals.var_tmf2_dn10 = assign92340_e141486_d_n10;
        locals.var_tmf2_dn11 = assign92340_e141486_d_n11;
        locals.var_tmf2_dn14 = assign92340_e141486_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign92350_e141504, assign92350_e141504_d_n0, assign92350_e141504_d_n2, assign92350_e141504_d_n4, assign92350_e141504_d_n5, assign92350_e141504_d_n6, assign92350_e141504_d_n7, assign92350_e141504_d_n8, assign92350_e141504_d_n9, assign92350_e141504_d_n10, assign92350_e141504_d_n11, assign92350_e141504_d_n14,) = {
    if (((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2145 != 0.0)) && (locals.var_guard2146 != 0.0)) {
        let assign92350_e141499: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign92350_e141501: f64 = (assign92350_e141499 + locals.var_tmf2);
        let assign92350_e141502: f64 = (assign92350_e141501).sqrt();
        (assign92350_e141502, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign92350_e141502)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign92350_e141502)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign92350_e141502)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign92350_e141502)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign92350_e141502)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign92350_e141502)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign92350_e141502)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign92350_e141502)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign92350_e141502)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign92350_e141502)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign92350_e141502)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign92350_e141504;
        locals.var_tmf2_dn0 = assign92350_e141504_d_n0;
        locals.var_tmf2_dn2 = assign92350_e141504_d_n2;
        locals.var_tmf2_dn4 = assign92350_e141504_d_n4;
        locals.var_tmf2_dn5 = assign92350_e141504_d_n5;
        locals.var_tmf2_dn6 = assign92350_e141504_d_n6;
        locals.var_tmf2_dn7 = assign92350_e141504_d_n7;
        locals.var_tmf2_dn8 = assign92350_e141504_d_n8;
        locals.var_tmf2_dn9 = assign92350_e141504_d_n9;
        locals.var_tmf2_dn10 = assign92350_e141504_d_n10;
        locals.var_tmf2_dn11 = assign92350_e141504_d_n11;
        locals.var_tmf2_dn14 = assign92350_e141504_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign92360_e141523, assign92360_e141523_d_n0, assign92360_e141523_d_n2, assign92360_e141523_d_n4, assign92360_e141523_d_n5, assign92360_e141523_d_n6, assign92360_e141523_d_n7, assign92360_e141523_d_n8, assign92360_e141523_d_n9, assign92360_e141523_d_n10, assign92360_e141523_d_n11, assign92360_e141523_d_n14,) = {
    if (((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2145 != 0.0)) && (locals.var_guard2146 != 0.0)) {
        let assign92360_e141519: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign92360_e141520: f64 = (1.0 + assign92360_e141519);
        let assign92360_e141521: f64 = (0.5 * assign92360_e141520);
        (assign92360_e141521, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign92360_e141523;
        locals.var_t1_dn0 = assign92360_e141523_d_n0;
        locals.var_t1_dn2 = assign92360_e141523_d_n2;
        locals.var_t1_dn4 = assign92360_e141523_d_n4;
        locals.var_t1_dn5 = assign92360_e141523_d_n5;
        locals.var_t1_dn6 = assign92360_e141523_d_n6;
        locals.var_t1_dn7 = assign92360_e141523_d_n7;
        locals.var_t1_dn8 = assign92360_e141523_d_n8;
        locals.var_t1_dn9 = assign92360_e141523_d_n9;
        locals.var_t1_dn10 = assign92360_e141523_d_n10;
        locals.var_t1_dn11 = assign92360_e141523_d_n11;
        locals.var_t1_dn14 = assign92360_e141523_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign92370_e141542, assign92370_e141542_d_n0, assign92370_e141542_d_n2, assign92370_e141542_d_n4, assign92370_e141542_d_n5, assign92370_e141542_d_n6, assign92370_e141542_d_n7, assign92370_e141542_d_n8, assign92370_e141542_d_n9, assign92370_e141542_d_n10, assign92370_e141542_d_n11, assign92370_e141542_d_n14,) = {
    if (((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2145 != 0.0)) && (locals.var_guard2146 != 0.0)) {
        let assign92370_e141538: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign92370_e141539: f64 = (0.5 * assign92370_e141538);
        let assign92370_e141540: f64 = (locals.var_psi - assign92370_e141539);
        (assign92370_e141540, (locals.var_psi_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_psi_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_psi_dn4 - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_psi_dn5 - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_psi_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_psi_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_psi_dn8 - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_psi_dn9 - (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (locals.var_psi_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_psi_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_psi_dn14 - (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn11, locals.var_chi_1_dn14,)
    }
};
        locals.var_chi_1 = assign92370_e141542;
        locals.var_chi_1_dn0 = assign92370_e141542_d_n0;
        locals.var_chi_1_dn2 = assign92370_e141542_d_n2;
        locals.var_chi_1_dn4 = assign92370_e141542_d_n4;
        locals.var_chi_1_dn5 = assign92370_e141542_d_n5;
        locals.var_chi_1_dn6 = assign92370_e141542_d_n6;
        locals.var_chi_1_dn7 = assign92370_e141542_d_n7;
        locals.var_chi_1_dn8 = assign92370_e141542_d_n8;
        locals.var_chi_1_dn9 = assign92370_e141542_d_n9;
        locals.var_chi_1_dn10 = assign92370_e141542_d_n10;
        locals.var_chi_1_dn11 = assign92370_e141542_d_n11;
        locals.var_chi_1_dn14 = assign92370_e141542_d_n14;
        locals.var_chi_1_rv = 0.0;

        let (assign92380_e141561, assign92380_e141561_d_n0, assign92380_e141561_d_n2, assign92380_e141561_d_n4, assign92380_e141561_d_n5, assign92380_e141561_d_n6, assign92380_e141561_d_n7, assign92380_e141561_d_n8, assign92380_e141561_d_n9, assign92380_e141561_d_n10, assign92380_e141561_d_n11, assign92380_e141561_d_n14,) = {
    if (((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2145 != 0.0)) && (locals.var_guard2146 == 0.0)) {
        let (assign92380_e141559, assign92380_e141559_d_n0, assign92380_e141559_d_n2, assign92380_e141559_d_n4, assign92380_e141559_d_n5, assign92380_e141559_d_n6, assign92380_e141559_d_n7, assign92380_e141559_d_n8, assign92380_e141559_d_n9, assign92380_e141559_d_n10, assign92380_e141559_d_n11, assign92380_e141559_d_n14,) = {
            if (locals.var_chi_1 <= locals.var_psi) {
                (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn11, locals.var_chi_1_dn14,)
            } else {
                (locals.var_psi, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn4, locals.var_psi_dn5, locals.var_psi_dn6, locals.var_psi_dn7, locals.var_psi_dn8, locals.var_psi_dn9, locals.var_psi_dn10, locals.var_psi_dn11, locals.var_psi_dn14,)
            }
        };
        (assign92380_e141559, assign92380_e141559_d_n0, assign92380_e141559_d_n2, assign92380_e141559_d_n4, assign92380_e141559_d_n5, assign92380_e141559_d_n6, assign92380_e141559_d_n7, assign92380_e141559_d_n8, assign92380_e141559_d_n9, assign92380_e141559_d_n10, assign92380_e141559_d_n11, assign92380_e141559_d_n14,)
    } else {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn11, locals.var_chi_1_dn14,)
    }
};
        locals.var_chi_1 = assign92380_e141561;
        locals.var_chi_1_dn0 = assign92380_e141561_d_n0;
        locals.var_chi_1_dn2 = assign92380_e141561_d_n2;
        locals.var_chi_1_dn4 = assign92380_e141561_d_n4;
        locals.var_chi_1_dn5 = assign92380_e141561_d_n5;
        locals.var_chi_1_dn6 = assign92380_e141561_d_n6;
        locals.var_chi_1_dn7 = assign92380_e141561_d_n7;
        locals.var_chi_1_dn8 = assign92380_e141561_d_n8;
        locals.var_chi_1_dn9 = assign92380_e141561_d_n9;
        locals.var_chi_1_dn10 = assign92380_e141561_d_n10;
        locals.var_chi_1_dn11 = assign92380_e141561_d_n11;
        locals.var_chi_1_dn14 = assign92380_e141561_d_n14;
        locals.var_chi_1_rv = 0.0;

        let (assign92390_e141577, assign92390_e141577_d_n0, assign92390_e141577_d_n2, assign92390_e141577_d_n4, assign92390_e141577_d_n5, assign92390_e141577_d_n6, assign92390_e141577_d_n7, assign92390_e141577_d_n8, assign92390_e141577_d_n9, assign92390_e141577_d_n10, assign92390_e141577_d_n11, assign92390_e141577_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2145 != 0.0)) {
        let (assign92390_e141575, assign92390_e141575_d_n0, assign92390_e141575_d_n2, assign92390_e141575_d_n4, assign92390_e141575_d_n5, assign92390_e141575_d_n6, assign92390_e141575_d_n7, assign92390_e141575_d_n8, assign92390_e141575_d_n9, assign92390_e141575_d_n10, assign92390_e141575_d_n11, assign92390_e141575_d_n14,) = {
            if (locals.var_chi_1 >= 0.0) {
                (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn11, locals.var_chi_1_dn14,)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign92390_e141575, assign92390_e141575_d_n0, assign92390_e141575_d_n2, assign92390_e141575_d_n4, assign92390_e141575_d_n5, assign92390_e141575_d_n6, assign92390_e141575_d_n7, assign92390_e141575_d_n8, assign92390_e141575_d_n9, assign92390_e141575_d_n10, assign92390_e141575_d_n11, assign92390_e141575_d_n14,)
    } else {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn11, locals.var_chi_1_dn14,)
    }
};
        locals.var_chi_1 = assign92390_e141577;
        locals.var_chi_1_dn0 = assign92390_e141577_d_n0;
        locals.var_chi_1_dn2 = assign92390_e141577_d_n2;
        locals.var_chi_1_dn4 = assign92390_e141577_d_n4;
        locals.var_chi_1_dn5 = assign92390_e141577_d_n5;
        locals.var_chi_1_dn6 = assign92390_e141577_d_n6;
        locals.var_chi_1_dn7 = assign92390_e141577_d_n7;
        locals.var_chi_1_dn8 = assign92390_e141577_d_n8;
        locals.var_chi_1_dn9 = assign92390_e141577_d_n9;
        locals.var_chi_1_dn10 = assign92390_e141577_d_n10;
        locals.var_chi_1_dn11 = assign92390_e141577_d_n11;
        locals.var_chi_1_dn14 = assign92390_e141577_d_n14;
        locals.var_chi_1_rv = 0.0;

        let (assign92400_e141590, assign92400_e141590_d_n0, assign92400_e141590_d_n2, assign92400_e141590_d_n4, assign92400_e141590_d_n5, assign92400_e141590_d_n6, assign92400_e141590_d_n7, assign92400_e141590_d_n8, assign92400_e141590_d_n9, assign92400_e141590_d_n10, assign92400_e141590_d_n11, assign92400_e141590_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2145 != 0.0)) {
        let assign92400_e141588: f64 = (locals.var_psi - locals.var_chi_1);
        (assign92400_e141588, (locals.var_psi_dn0 - locals.var_chi_1_dn0), (locals.var_psi_dn2 - locals.var_chi_1_dn2), (locals.var_psi_dn4 - locals.var_chi_1_dn4), (locals.var_psi_dn5 - locals.var_chi_1_dn5), (locals.var_psi_dn6 - locals.var_chi_1_dn6), (locals.var_psi_dn7 - locals.var_chi_1_dn7), (locals.var_psi_dn8 - locals.var_chi_1_dn8), (locals.var_psi_dn9 - locals.var_chi_1_dn9), (locals.var_psi_dn10 - locals.var_chi_1_dn10), (locals.var_psi_dn11 - locals.var_chi_1_dn11), (locals.var_psi_dn14 - locals.var_chi_1_dn14),)
    } else {
        (locals.var_psi, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn4, locals.var_psi_dn5, locals.var_psi_dn6, locals.var_psi_dn7, locals.var_psi_dn8, locals.var_psi_dn9, locals.var_psi_dn10, locals.var_psi_dn11, locals.var_psi_dn14,)
    }
};
        locals.var_psi = assign92400_e141590;
        locals.var_psi_dn0 = assign92400_e141590_d_n0;
        locals.var_psi_dn2 = assign92400_e141590_d_n2;
        locals.var_psi_dn4 = assign92400_e141590_d_n4;
        locals.var_psi_dn5 = assign92400_e141590_d_n5;
        locals.var_psi_dn6 = assign92400_e141590_d_n6;
        locals.var_psi_dn7 = assign92400_e141590_d_n7;
        locals.var_psi_dn8 = assign92400_e141590_d_n8;
        locals.var_psi_dn9 = assign92400_e141590_d_n9;
        locals.var_psi_dn10 = assign92400_e141590_d_n10;
        locals.var_psi_dn11 = assign92400_e141590_d_n11;
        locals.var_psi_dn14 = assign92400_e141590_d_n14;
        locals.var_psi_rv = 0.0;

        let (assign92410_e141605, assign92410_e141605_d_n0, assign92410_e141605_d_n2, assign92410_e141605_d_n4, assign92410_e141605_d_n5, assign92410_e141605_d_n6, assign92410_e141605_d_n7, assign92410_e141605_d_n8, assign92410_e141605_d_n9, assign92410_e141605_d_n10, assign92410_e141605_d_n11, assign92410_e141605_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2145 != 0.0)) {
        let assign92410_e141602: f64 = (locals.var_beta * 0.1);
        let assign92410_e141603: f64 = (locals.var_psi + assign92410_e141602);
        (assign92410_e141603, (locals.var_psi_dn0 + (locals.var_beta_dn0 * 0.1)), (locals.var_psi_dn2 + (locals.var_beta_dn2 * 0.1)), (locals.var_psi_dn4 + (locals.var_beta_dn4 * 0.1)), (locals.var_psi_dn5 + (locals.var_beta_dn5 * 0.1)), (locals.var_psi_dn6 + (locals.var_beta_dn6 * 0.1)), (locals.var_psi_dn7 + (locals.var_beta_dn7 * 0.1)), (locals.var_psi_dn8 + (locals.var_beta_dn8 * 0.1)), (locals.var_psi_dn9 + (locals.var_beta_dn9 * 0.1)), (locals.var_psi_dn10 + (locals.var_beta_dn10 * 0.1)), (locals.var_psi_dn11 + (locals.var_beta_dn11 * 0.1)), (locals.var_psi_dn14 + (locals.var_beta_dn14 * 0.1)),)
    } else {
        (locals.var_psi, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn4, locals.var_psi_dn5, locals.var_psi_dn6, locals.var_psi_dn7, locals.var_psi_dn8, locals.var_psi_dn9, locals.var_psi_dn10, locals.var_psi_dn11, locals.var_psi_dn14,)
    }
};
        locals.var_psi = assign92410_e141605;
        locals.var_psi_dn0 = assign92410_e141605_d_n0;
        locals.var_psi_dn2 = assign92410_e141605_d_n2;
        locals.var_psi_dn4 = assign92410_e141605_d_n4;
        locals.var_psi_dn5 = assign92410_e141605_d_n5;
        locals.var_psi_dn6 = assign92410_e141605_d_n6;
        locals.var_psi_dn7 = assign92410_e141605_d_n7;
        locals.var_psi_dn8 = assign92410_e141605_d_n8;
        locals.var_psi_dn9 = assign92410_e141605_d_n9;
        locals.var_psi_dn10 = assign92410_e141605_d_n10;
        locals.var_psi_dn11 = assign92410_e141605_d_n11;
        locals.var_psi_dn14 = assign92410_e141605_d_n14;
        locals.var_psi_rv = 0.0;

        let (assign92420_e141628, assign92420_e141628_d_n0, assign92420_e141628_d_n2, assign92420_e141628_d_n4, assign92420_e141628_d_n5, assign92420_e141628_d_n6, assign92420_e141628_d_n7, assign92420_e141628_d_n8, assign92420_e141628_d_n9, assign92420_e141628_d_n10, assign92420_e141628_d_n11, assign92420_e141628_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2145 != 0.0)) {
        let assign92420_e141616: f64 = (locals.var_gammachi * locals.var_t0);
        let assign92420_e141619: f64 = (locals.var_psi * locals.var_psi);
        let assign92420_e141620: f64 = (assign92420_e141616 + assign92420_e141619);
        let assign92420_e141621: f64 = (assign92420_e141620).ln();
        let assign92420_e141624: f64 = (locals.var_cnst1over * locals.var_t0);
        let assign92420_e141625: f64 = (assign92420_e141624).ln();
        let assign92420_e141626: f64 = (assign92420_e141621 - assign92420_e141625);
        (assign92420_e141626, (((((locals.var_gammachi_dn0 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn0)) + ((locals.var_psi_dn0 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn0))) / assign92420_e141620) - (((locals.var_cnst1over_dn0 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn0)) / assign92420_e141624)), (((((locals.var_gammachi_dn2 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn2)) + ((locals.var_psi_dn2 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn2))) / assign92420_e141620) - (((locals.var_cnst1over_dn2 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn2)) / assign92420_e141624)), (((((locals.var_gammachi_dn4 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn4)) + ((locals.var_psi_dn4 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn4))) / assign92420_e141620) - (((locals.var_cnst1over_dn4 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn4)) / assign92420_e141624)), (((((locals.var_gammachi_dn5 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn5)) + ((locals.var_psi_dn5 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn5))) / assign92420_e141620) - (((locals.var_cnst1over_dn5 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn5)) / assign92420_e141624)), (((((locals.var_gammachi_dn6 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn6)) + ((locals.var_psi_dn6 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn6))) / assign92420_e141620) - (((locals.var_cnst1over_dn6 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn6)) / assign92420_e141624)), (((((locals.var_gammachi_dn7 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn7)) + ((locals.var_psi_dn7 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn7))) / assign92420_e141620) - (((locals.var_cnst1over_dn7 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn7)) / assign92420_e141624)), (((((locals.var_gammachi_dn8 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn8)) + ((locals.var_psi_dn8 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn8))) / assign92420_e141620) - (((locals.var_cnst1over_dn8 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn8)) / assign92420_e141624)), (((((locals.var_gammachi_dn9 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn9)) + ((locals.var_psi_dn9 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn9))) / assign92420_e141620) - (((locals.var_cnst1over_dn9 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn9)) / assign92420_e141624)), (((((locals.var_gammachi_dn10 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn10)) + ((locals.var_psi_dn10 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn10))) / assign92420_e141620) - (((locals.var_cnst1over_dn10 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn10)) / assign92420_e141624)), (((((locals.var_gammachi_dn11 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn11)) + ((locals.var_psi_dn11 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn11))) / assign92420_e141620) - (((locals.var_cnst1over_dn11 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn11)) / assign92420_e141624)), (((((locals.var_gammachi_dn14 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn14)) + ((locals.var_psi_dn14 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn14))) / assign92420_e141620) - (((locals.var_cnst1over_dn14 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn14)) / assign92420_e141624)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign92420_e141628;
        locals.var_t1_dn0 = assign92420_e141628_d_n0;
        locals.var_t1_dn2 = assign92420_e141628_d_n2;
        locals.var_t1_dn4 = assign92420_e141628_d_n4;
        locals.var_t1_dn5 = assign92420_e141628_d_n5;
        locals.var_t1_dn6 = assign92420_e141628_d_n6;
        locals.var_t1_dn7 = assign92420_e141628_d_n7;
        locals.var_t1_dn8 = assign92420_e141628_d_n8;
        locals.var_t1_dn9 = assign92420_e141628_d_n9;
        locals.var_t1_dn10 = assign92420_e141628_d_n10;
        locals.var_t1_dn11 = assign92420_e141628_d_n11;
        locals.var_t1_dn14 = assign92420_e141628_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign92430_e141643, assign92430_e141643_d_n0, assign92430_e141643_d_n2, assign92430_e141643_d_n4, assign92430_e141643_d_n5, assign92430_e141643_d_n6, assign92430_e141643_d_n7, assign92430_e141643_d_n8, assign92430_e141643_d_n9, assign92430_e141643_d_n10, assign92430_e141643_d_n11, assign92430_e141643_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2145 != 0.0)) {
        let assign92430_e141640: f64 = (locals.var_beta * locals.var_vxbgmtcl);
        let assign92430_e141641: f64 = (locals.var_t1 + assign92430_e141640);
        (assign92430_e141641, (locals.var_t1_dn0 + ((locals.var_beta_dn0 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn0))), (locals.var_t1_dn2 + ((locals.var_beta_dn2 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn2))), (locals.var_t1_dn4 + ((locals.var_beta_dn4 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn4))), (locals.var_t1_dn5 + ((locals.var_beta_dn5 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn5))), (locals.var_t1_dn6 + ((locals.var_beta_dn6 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn6))), (locals.var_t1_dn7 + ((locals.var_beta_dn7 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn7))), (locals.var_t1_dn8 + ((locals.var_beta_dn8 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn8))), (locals.var_t1_dn9 + ((locals.var_beta_dn9 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn9))), (locals.var_t1_dn10 + ((locals.var_beta_dn10 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn10))), (locals.var_t1_dn11 + ((locals.var_beta_dn11 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn11))), (locals.var_t1_dn14 + ((locals.var_beta_dn14 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn14))),)
    } else {
        (locals.var_chi_b, locals.var_chi_b_dn0, locals.var_chi_b_dn2, locals.var_chi_b_dn4, locals.var_chi_b_dn5, locals.var_chi_b_dn6, locals.var_chi_b_dn7, locals.var_chi_b_dn8, locals.var_chi_b_dn9, locals.var_chi_b_dn10, locals.var_chi_b_dn11, locals.var_chi_b_dn14,)
    }
};
        locals.var_chi_b = assign92430_e141643;
        locals.var_chi_b_dn0 = assign92430_e141643_d_n0;
        locals.var_chi_b_dn2 = assign92430_e141643_d_n2;
        locals.var_chi_b_dn4 = assign92430_e141643_d_n4;
        locals.var_chi_b_dn5 = assign92430_e141643_d_n5;
        locals.var_chi_b_dn6 = assign92430_e141643_d_n6;
        locals.var_chi_b_dn7 = assign92430_e141643_d_n7;
        locals.var_chi_b_dn8 = assign92430_e141643_d_n8;
        locals.var_chi_b_dn9 = assign92430_e141643_d_n9;
        locals.var_chi_b_dn10 = assign92430_e141643_d_n10;
        locals.var_chi_b_dn11 = assign92430_e141643_d_n11;
        locals.var_chi_b_dn14 = assign92430_e141643_d_n14;
        locals.var_chi_b_rv = 0.0;

        let (assign92440_e141659, assign92440_e141659_d_n0, assign92440_e141659_d_n2, assign92440_e141659_d_n4, assign92440_e141659_d_n5, assign92440_e141659_d_n6, assign92440_e141659_d_n7, assign92440_e141659_d_n8, assign92440_e141659_d_n9, assign92440_e141659_d_n10, assign92440_e141659_d_n11, assign92440_e141659_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2145 != 0.0)) {
        let (assign92440_e141657, assign92440_e141657_d_n0, assign92440_e141657_d_n2, assign92440_e141657_d_n4, assign92440_e141657_d_n5, assign92440_e141657_d_n6, assign92440_e141657_d_n7, assign92440_e141657_d_n8, assign92440_e141657_d_n9, assign92440_e141657_d_n10, assign92440_e141657_d_n11, assign92440_e141657_d_n14,) = {
            if (locals.var_chi_b >= 0.0) {
                (locals.var_chi_b, locals.var_chi_b_dn0, locals.var_chi_b_dn2, locals.var_chi_b_dn4, locals.var_chi_b_dn5, locals.var_chi_b_dn6, locals.var_chi_b_dn7, locals.var_chi_b_dn8, locals.var_chi_b_dn9, locals.var_chi_b_dn10, locals.var_chi_b_dn11, locals.var_chi_b_dn14,)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign92440_e141657, assign92440_e141657_d_n0, assign92440_e141657_d_n2, assign92440_e141657_d_n4, assign92440_e141657_d_n5, assign92440_e141657_d_n6, assign92440_e141657_d_n7, assign92440_e141657_d_n8, assign92440_e141657_d_n9, assign92440_e141657_d_n10, assign92440_e141657_d_n11, assign92440_e141657_d_n14,)
    } else {
        (locals.var_chi_b, locals.var_chi_b_dn0, locals.var_chi_b_dn2, locals.var_chi_b_dn4, locals.var_chi_b_dn5, locals.var_chi_b_dn6, locals.var_chi_b_dn7, locals.var_chi_b_dn8, locals.var_chi_b_dn9, locals.var_chi_b_dn10, locals.var_chi_b_dn11, locals.var_chi_b_dn14,)
    }
};
        locals.var_chi_b = assign92440_e141659;
        locals.var_chi_b_dn0 = assign92440_e141659_d_n0;
        locals.var_chi_b_dn2 = assign92440_e141659_d_n2;
        locals.var_chi_b_dn4 = assign92440_e141659_d_n4;
        locals.var_chi_b_dn5 = assign92440_e141659_d_n5;
        locals.var_chi_b_dn6 = assign92440_e141659_d_n6;
        locals.var_chi_b_dn7 = assign92440_e141659_d_n7;
        locals.var_chi_b_dn8 = assign92440_e141659_d_n8;
        locals.var_chi_b_dn9 = assign92440_e141659_d_n9;
        locals.var_chi_b_dn10 = assign92440_e141659_d_n10;
        locals.var_chi_b_dn11 = assign92440_e141659_d_n11;
        locals.var_chi_b_dn14 = assign92440_e141659_d_n14;
        locals.var_chi_b_rv = 0.0;

        let (assign92450_e141670, assign92450_e141670_d_n0, assign92450_e141670_d_n2, assign92450_e141670_d_n4, assign92450_e141670_d_n5, assign92450_e141670_d_n6, assign92450_e141670_d_n7, assign92450_e141670_d_n8, assign92450_e141670_d_n9, assign92450_e141670_d_n10, assign92450_e141670_d_n11, assign92450_e141670_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2145 != 0.0)) {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    } else {
        (locals.var_chi_a, locals.var_chi_a_dn0, locals.var_chi_a_dn2, locals.var_chi_a_dn4, locals.var_chi_a_dn5, locals.var_chi_a_dn6, locals.var_chi_a_dn7, locals.var_chi_a_dn8, locals.var_chi_a_dn9, locals.var_chi_a_dn10, locals.var_chi_a_dn11, locals.var_chi_a_dn14,)
    }
};
        locals.var_chi_a = assign92450_e141670;
        locals.var_chi_a_dn0 = assign92450_e141670_d_n0;
        locals.var_chi_a_dn2 = assign92450_e141670_d_n2;
        locals.var_chi_a_dn4 = assign92450_e141670_d_n4;
        locals.var_chi_a_dn5 = assign92450_e141670_d_n5;
        locals.var_chi_a_dn6 = assign92450_e141670_d_n6;
        locals.var_chi_a_dn7 = assign92450_e141670_d_n7;
        locals.var_chi_a_dn8 = assign92450_e141670_d_n8;
        locals.var_chi_a_dn9 = assign92450_e141670_d_n9;
        locals.var_chi_a_dn10 = assign92450_e141670_d_n10;
        locals.var_chi_a_dn11 = assign92450_e141670_d_n11;
        locals.var_chi_a_dn14 = assign92450_e141670_d_n14;
        locals.var_chi_a_rv = 0.0;

        let assign92460_e141673: f64 = if p.p33 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard2147 = assign92460_e141673;
        locals.var_guard2147_rv = 0.0;

        let assign92470_e141678: f64 = (0.2 * locals.var_chi_b);
        let assign92470_e141679: f64 = (locals.var_chi_b - assign92470_e141678);
        let assign92470_e141683: f64 = (0.2 * locals.var_chi_b);
        let assign92470_e141686: f64 = if ((locals.var_chi_a > assign92470_e141679) && (assign92470_e141683 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2148 = assign92470_e141686;
        locals.var_guard2148_rv = 0.0;

        let (assign92480_e141707, assign92480_e141707_d_n0, assign92480_e141707_d_n2, assign92480_e141707_d_n4, assign92480_e141707_d_n5, assign92480_e141707_d_n6, assign92480_e141707_d_n7, assign92480_e141707_d_n8, assign92480_e141707_d_n9, assign92480_e141707_d_n10, assign92480_e141707_d_n11, assign92480_e141707_d_n14,) = {
    if ((((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2145 != 0.0)) && (locals.var_guard2147 != 0.0)) && (locals.var_guard2148 != 0.0)) {
        let assign92480_e141701: f64 = (locals.var_chi_a - locals.var_chi_b);
        let assign92480_e141704: f64 = (0.2 * locals.var_chi_b);
        let assign92480_e141705: f64 = (assign92480_e141701 + assign92480_e141704);
        (assign92480_e141705, ((locals.var_chi_a_dn0 - locals.var_chi_b_dn0) + (0.2 * locals.var_chi_b_dn0)), ((locals.var_chi_a_dn2 - locals.var_chi_b_dn2) + (0.2 * locals.var_chi_b_dn2)), ((locals.var_chi_a_dn4 - locals.var_chi_b_dn4) + (0.2 * locals.var_chi_b_dn4)), ((locals.var_chi_a_dn5 - locals.var_chi_b_dn5) + (0.2 * locals.var_chi_b_dn5)), ((locals.var_chi_a_dn6 - locals.var_chi_b_dn6) + (0.2 * locals.var_chi_b_dn6)), ((locals.var_chi_a_dn7 - locals.var_chi_b_dn7) + (0.2 * locals.var_chi_b_dn7)), ((locals.var_chi_a_dn8 - locals.var_chi_b_dn8) + (0.2 * locals.var_chi_b_dn8)), ((locals.var_chi_a_dn9 - locals.var_chi_b_dn9) + (0.2 * locals.var_chi_b_dn9)), ((locals.var_chi_a_dn10 - locals.var_chi_b_dn10) + (0.2 * locals.var_chi_b_dn10)), ((locals.var_chi_a_dn11 - locals.var_chi_b_dn11) + (0.2 * locals.var_chi_b_dn11)), ((locals.var_chi_a_dn14 - locals.var_chi_b_dn14) + (0.2 * locals.var_chi_b_dn14)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign92480_e141707;
        locals.var_tmf1_dn0 = assign92480_e141707_d_n0;
        locals.var_tmf1_dn2 = assign92480_e141707_d_n2;
        locals.var_tmf1_dn4 = assign92480_e141707_d_n4;
        locals.var_tmf1_dn5 = assign92480_e141707_d_n5;
        locals.var_tmf1_dn6 = assign92480_e141707_d_n6;
        locals.var_tmf1_dn7 = assign92480_e141707_d_n7;
        locals.var_tmf1_dn8 = assign92480_e141707_d_n8;
        locals.var_tmf1_dn9 = assign92480_e141707_d_n9;
        locals.var_tmf1_dn10 = assign92480_e141707_d_n10;
        locals.var_tmf1_dn11 = assign92480_e141707_d_n11;
        locals.var_tmf1_dn14 = assign92480_e141707_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign92490_e141724, assign92490_e141724_d_n0, assign92490_e141724_d_n2, assign92490_e141724_d_n4, assign92490_e141724_d_n5, assign92490_e141724_d_n6, assign92490_e141724_d_n7, assign92490_e141724_d_n8, assign92490_e141724_d_n9, assign92490_e141724_d_n10, assign92490_e141724_d_n11, assign92490_e141724_d_n14,) = {
    if ((((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2145 != 0.0)) && (locals.var_guard2147 != 0.0)) && (locals.var_guard2148 != 0.0)) {
        let assign92490_e141722: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign92490_e141722, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign92490_e141724;
        locals.var_x2_dn0 = assign92490_e141724_d_n0;
        locals.var_x2_dn2 = assign92490_e141724_d_n2;
        locals.var_x2_dn4 = assign92490_e141724_d_n4;
        locals.var_x2_dn5 = assign92490_e141724_d_n5;
        locals.var_x2_dn6 = assign92490_e141724_d_n6;
        locals.var_x2_dn7 = assign92490_e141724_d_n7;
        locals.var_x2_dn8 = assign92490_e141724_d_n8;
        locals.var_x2_dn9 = assign92490_e141724_d_n9;
        locals.var_x2_dn10 = assign92490_e141724_d_n10;
        locals.var_x2_dn11 = assign92490_e141724_d_n11;
        locals.var_x2_dn14 = assign92490_e141724_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign92500_e141745, assign92500_e141745_d_n0, assign92500_e141745_d_n2, assign92500_e141745_d_n4, assign92500_e141745_d_n5, assign92500_e141745_d_n6, assign92500_e141745_d_n7, assign92500_e141745_d_n8, assign92500_e141745_d_n9, assign92500_e141745_d_n10, assign92500_e141745_d_n11, assign92500_e141745_d_n14,) = {
    if ((((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2145 != 0.0)) && (locals.var_guard2147 != 0.0)) && (locals.var_guard2148 != 0.0)) {
        let assign92500_e141739: f64 = (0.2 * locals.var_chi_b);
        let assign92500_e141742: f64 = (0.2 * locals.var_chi_b);
        let assign92500_e141743: f64 = (assign92500_e141739 * assign92500_e141742);
        (assign92500_e141743, (((0.2 * locals.var_chi_b_dn0) * assign92500_e141742) + (assign92500_e141739 * (0.2 * locals.var_chi_b_dn0))), (((0.2 * locals.var_chi_b_dn2) * assign92500_e141742) + (assign92500_e141739 * (0.2 * locals.var_chi_b_dn2))), (((0.2 * locals.var_chi_b_dn4) * assign92500_e141742) + (assign92500_e141739 * (0.2 * locals.var_chi_b_dn4))), (((0.2 * locals.var_chi_b_dn5) * assign92500_e141742) + (assign92500_e141739 * (0.2 * locals.var_chi_b_dn5))), (((0.2 * locals.var_chi_b_dn6) * assign92500_e141742) + (assign92500_e141739 * (0.2 * locals.var_chi_b_dn6))), (((0.2 * locals.var_chi_b_dn7) * assign92500_e141742) + (assign92500_e141739 * (0.2 * locals.var_chi_b_dn7))), (((0.2 * locals.var_chi_b_dn8) * assign92500_e141742) + (assign92500_e141739 * (0.2 * locals.var_chi_b_dn8))), (((0.2 * locals.var_chi_b_dn9) * assign92500_e141742) + (assign92500_e141739 * (0.2 * locals.var_chi_b_dn9))), (((0.2 * locals.var_chi_b_dn10) * assign92500_e141742) + (assign92500_e141739 * (0.2 * locals.var_chi_b_dn10))), (((0.2 * locals.var_chi_b_dn11) * assign92500_e141742) + (assign92500_e141739 * (0.2 * locals.var_chi_b_dn11))), (((0.2 * locals.var_chi_b_dn14) * assign92500_e141742) + (assign92500_e141739 * (0.2 * locals.var_chi_b_dn14))),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign92500_e141745;
        locals.var_xmax2_dn0 = assign92500_e141745_d_n0;
        locals.var_xmax2_dn2 = assign92500_e141745_d_n2;
        locals.var_xmax2_dn4 = assign92500_e141745_d_n4;
        locals.var_xmax2_dn5 = assign92500_e141745_d_n5;
        locals.var_xmax2_dn6 = assign92500_e141745_d_n6;
        locals.var_xmax2_dn7 = assign92500_e141745_d_n7;
        locals.var_xmax2_dn8 = assign92500_e141745_d_n8;
        locals.var_xmax2_dn9 = assign92500_e141745_d_n9;
        locals.var_xmax2_dn10 = assign92500_e141745_d_n10;
        locals.var_xmax2_dn11 = assign92500_e141745_d_n11;
        locals.var_xmax2_dn14 = assign92500_e141745_d_n14;
        locals.var_xmax2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_357(
        locals: &mut StampLocals,
    ) {
        let (assign92510_e141760, assign92510_e141760_d_n0, assign92510_e141760_d_n2, assign92510_e141760_d_n4, assign92510_e141760_d_n5, assign92510_e141760_d_n6, assign92510_e141760_d_n7, assign92510_e141760_d_n8, assign92510_e141760_d_n9, assign92510_e141760_d_n10, assign92510_e141760_d_n11, assign92510_e141760_d_n14,) = {
    if ((((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2145 != 0.0)) && (locals.var_guard2147 != 0.0)) && (locals.var_guard2148 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign92510_e141760;
        locals.var_xp_dn0 = assign92510_e141760_d_n0;
        locals.var_xp_dn2 = assign92510_e141760_d_n2;
        locals.var_xp_dn4 = assign92510_e141760_d_n4;
        locals.var_xp_dn5 = assign92510_e141760_d_n5;
        locals.var_xp_dn6 = assign92510_e141760_d_n6;
        locals.var_xp_dn7 = assign92510_e141760_d_n7;
        locals.var_xp_dn8 = assign92510_e141760_d_n8;
        locals.var_xp_dn9 = assign92510_e141760_d_n9;
        locals.var_xp_dn10 = assign92510_e141760_d_n10;
        locals.var_xp_dn11 = assign92510_e141760_d_n11;
        locals.var_xp_dn14 = assign92510_e141760_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign92520_e141775, assign92520_e141775_d_n0, assign92520_e141775_d_n2, assign92520_e141775_d_n4, assign92520_e141775_d_n5, assign92520_e141775_d_n6, assign92520_e141775_d_n7, assign92520_e141775_d_n8, assign92520_e141775_d_n9, assign92520_e141775_d_n10, assign92520_e141775_d_n11, assign92520_e141775_d_n14,) = {
    if ((((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2145 != 0.0)) && (locals.var_guard2147 != 0.0)) && (locals.var_guard2148 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign92520_e141775;
        locals.var_xmp_dn0 = assign92520_e141775_d_n0;
        locals.var_xmp_dn2 = assign92520_e141775_d_n2;
        locals.var_xmp_dn4 = assign92520_e141775_d_n4;
        locals.var_xmp_dn5 = assign92520_e141775_d_n5;
        locals.var_xmp_dn6 = assign92520_e141775_d_n6;
        locals.var_xmp_dn7 = assign92520_e141775_d_n7;
        locals.var_xmp_dn8 = assign92520_e141775_d_n8;
        locals.var_xmp_dn9 = assign92520_e141775_d_n9;
        locals.var_xmp_dn10 = assign92520_e141775_d_n10;
        locals.var_xmp_dn11 = assign92520_e141775_d_n11;
        locals.var_xmp_dn14 = assign92520_e141775_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign92530_e141790,) = {
    if ((((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2145 != 0.0)) && (locals.var_guard2147 != 0.0)) && (locals.var_guard2148 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign92530_e141790;
        locals.var_m0_rv = 0.0;

        let (assign92540_e141805,) = {
    if ((((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2145 != 0.0)) && (locals.var_guard2147 != 0.0)) && (locals.var_guard2148 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign92540_e141805;
        locals.var_mm_rv = 0.0;

        let (assign92550_e141820, assign92550_e141820_d_n0, assign92550_e141820_d_n2, assign92550_e141820_d_n4, assign92550_e141820_d_n5, assign92550_e141820_d_n6, assign92550_e141820_d_n7, assign92550_e141820_d_n8, assign92550_e141820_d_n9, assign92550_e141820_d_n10, assign92550_e141820_d_n11, assign92550_e141820_d_n14,) = {
    if ((((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2145 != 0.0)) && (locals.var_guard2147 != 0.0)) && (locals.var_guard2148 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign92550_e141820;
        locals.var_arg_dn0 = assign92550_e141820_d_n0;
        locals.var_arg_dn2 = assign92550_e141820_d_n2;
        locals.var_arg_dn4 = assign92550_e141820_d_n4;
        locals.var_arg_dn5 = assign92550_e141820_d_n5;
        locals.var_arg_dn6 = assign92550_e141820_d_n6;
        locals.var_arg_dn7 = assign92550_e141820_d_n7;
        locals.var_arg_dn8 = assign92550_e141820_d_n8;
        locals.var_arg_dn9 = assign92550_e141820_d_n9;
        locals.var_arg_dn10 = assign92550_e141820_d_n10;
        locals.var_arg_dn11 = assign92550_e141820_d_n11;
        locals.var_arg_dn14 = assign92550_e141820_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign92560_e141835, assign92560_e141835_d_n0, assign92560_e141835_d_n2, assign92560_e141835_d_n4, assign92560_e141835_d_n5, assign92560_e141835_d_n6, assign92560_e141835_d_n7, assign92560_e141835_d_n8, assign92560_e141835_d_n9, assign92560_e141835_d_n10, assign92560_e141835_d_n11, assign92560_e141835_d_n14,) = {
    if ((((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2145 != 0.0)) && (locals.var_guard2147 != 0.0)) && (locals.var_guard2148 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign92560_e141835;
        locals.var_dnm_dn0 = assign92560_e141835_d_n0;
        locals.var_dnm_dn2 = assign92560_e141835_d_n2;
        locals.var_dnm_dn4 = assign92560_e141835_d_n4;
        locals.var_dnm_dn5 = assign92560_e141835_d_n5;
        locals.var_dnm_dn6 = assign92560_e141835_d_n6;
        locals.var_dnm_dn7 = assign92560_e141835_d_n7;
        locals.var_dnm_dn8 = assign92560_e141835_d_n8;
        locals.var_dnm_dn9 = assign92560_e141835_d_n9;
        locals.var_dnm_dn10 = assign92560_e141835_d_n10;
        locals.var_dnm_dn11 = assign92560_e141835_d_n11;
        locals.var_dnm_dn14 = assign92560_e141835_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign92570_e141852, assign92570_e141852_d_n0, assign92570_e141852_d_n2, assign92570_e141852_d_n4, assign92570_e141852_d_n5, assign92570_e141852_d_n6, assign92570_e141852_d_n7, assign92570_e141852_d_n8, assign92570_e141852_d_n9, assign92570_e141852_d_n10, assign92570_e141852_d_n11, assign92570_e141852_d_n14,) = {
    if ((((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2145 != 0.0)) && (locals.var_guard2147 != 0.0)) && (locals.var_guard2148 != 0.0)) {
        let assign92570_e141850: f64 = (locals.var_xp * locals.var_x2);
        (assign92570_e141850, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign92570_e141852;
        locals.var_xp_dn0 = assign92570_e141852_d_n0;
        locals.var_xp_dn2 = assign92570_e141852_d_n2;
        locals.var_xp_dn4 = assign92570_e141852_d_n4;
        locals.var_xp_dn5 = assign92570_e141852_d_n5;
        locals.var_xp_dn6 = assign92570_e141852_d_n6;
        locals.var_xp_dn7 = assign92570_e141852_d_n7;
        locals.var_xp_dn8 = assign92570_e141852_d_n8;
        locals.var_xp_dn9 = assign92570_e141852_d_n9;
        locals.var_xp_dn10 = assign92570_e141852_d_n10;
        locals.var_xp_dn11 = assign92570_e141852_d_n11;
        locals.var_xp_dn14 = assign92570_e141852_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign92580_e141869, assign92580_e141869_d_n0, assign92580_e141869_d_n2, assign92580_e141869_d_n4, assign92580_e141869_d_n5, assign92580_e141869_d_n6, assign92580_e141869_d_n7, assign92580_e141869_d_n8, assign92580_e141869_d_n9, assign92580_e141869_d_n10, assign92580_e141869_d_n11, assign92580_e141869_d_n14,) = {
    if ((((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2145 != 0.0)) && (locals.var_guard2147 != 0.0)) && (locals.var_guard2148 != 0.0)) {
        let assign92580_e141867: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign92580_e141867, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign92580_e141869;
        locals.var_xmp_dn0 = assign92580_e141869_d_n0;
        locals.var_xmp_dn2 = assign92580_e141869_d_n2;
        locals.var_xmp_dn4 = assign92580_e141869_d_n4;
        locals.var_xmp_dn5 = assign92580_e141869_d_n5;
        locals.var_xmp_dn6 = assign92580_e141869_d_n6;
        locals.var_xmp_dn7 = assign92580_e141869_d_n7;
        locals.var_xmp_dn8 = assign92580_e141869_d_n8;
        locals.var_xmp_dn9 = assign92580_e141869_d_n9;
        locals.var_xmp_dn10 = assign92580_e141869_d_n10;
        locals.var_xmp_dn11 = assign92580_e141869_d_n11;
        locals.var_xmp_dn14 = assign92580_e141869_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign92590_e141886, assign92590_e141886_d_n0, assign92590_e141886_d_n2, assign92590_e141886_d_n4, assign92590_e141886_d_n5, assign92590_e141886_d_n6, assign92590_e141886_d_n7, assign92590_e141886_d_n8, assign92590_e141886_d_n9, assign92590_e141886_d_n10, assign92590_e141886_d_n11, assign92590_e141886_d_n14,) = {
    if ((((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2145 != 0.0)) && (locals.var_guard2147 != 0.0)) && (locals.var_guard2148 != 0.0)) {
        let assign92590_e141884: f64 = (locals.var_xp * locals.var_x2);
        (assign92590_e141884, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign92590_e141886;
        locals.var_xp_dn0 = assign92590_e141886_d_n0;
        locals.var_xp_dn2 = assign92590_e141886_d_n2;
        locals.var_xp_dn4 = assign92590_e141886_d_n4;
        locals.var_xp_dn5 = assign92590_e141886_d_n5;
        locals.var_xp_dn6 = assign92590_e141886_d_n6;
        locals.var_xp_dn7 = assign92590_e141886_d_n7;
        locals.var_xp_dn8 = assign92590_e141886_d_n8;
        locals.var_xp_dn9 = assign92590_e141886_d_n9;
        locals.var_xp_dn10 = assign92590_e141886_d_n10;
        locals.var_xp_dn11 = assign92590_e141886_d_n11;
        locals.var_xp_dn14 = assign92590_e141886_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign92600_e141903, assign92600_e141903_d_n0, assign92600_e141903_d_n2, assign92600_e141903_d_n4, assign92600_e141903_d_n5, assign92600_e141903_d_n6, assign92600_e141903_d_n7, assign92600_e141903_d_n8, assign92600_e141903_d_n9, assign92600_e141903_d_n10, assign92600_e141903_d_n11, assign92600_e141903_d_n14,) = {
    if ((((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2145 != 0.0)) && (locals.var_guard2147 != 0.0)) && (locals.var_guard2148 != 0.0)) {
        let assign92600_e141901: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign92600_e141901, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign92600_e141903;
        locals.var_xmp_dn0 = assign92600_e141903_d_n0;
        locals.var_xmp_dn2 = assign92600_e141903_d_n2;
        locals.var_xmp_dn4 = assign92600_e141903_d_n4;
        locals.var_xmp_dn5 = assign92600_e141903_d_n5;
        locals.var_xmp_dn6 = assign92600_e141903_d_n6;
        locals.var_xmp_dn7 = assign92600_e141903_d_n7;
        locals.var_xmp_dn8 = assign92600_e141903_d_n8;
        locals.var_xmp_dn9 = assign92600_e141903_d_n9;
        locals.var_xmp_dn10 = assign92600_e141903_d_n10;
        locals.var_xmp_dn11 = assign92600_e141903_d_n11;
        locals.var_xmp_dn14 = assign92600_e141903_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign92610_e141920, assign92610_e141920_d_n0, assign92610_e141920_d_n2, assign92610_e141920_d_n4, assign92610_e141920_d_n5, assign92610_e141920_d_n6, assign92610_e141920_d_n7, assign92610_e141920_d_n8, assign92610_e141920_d_n9, assign92610_e141920_d_n10, assign92610_e141920_d_n11, assign92610_e141920_d_n14,) = {
    if ((((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2145 != 0.0)) && (locals.var_guard2147 != 0.0)) && (locals.var_guard2148 != 0.0)) {
        let assign92610_e141918: f64 = (locals.var_xp + locals.var_xmp);
        (assign92610_e141918, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign92610_e141920;
        locals.var_arg_dn0 = assign92610_e141920_d_n0;
        locals.var_arg_dn2 = assign92610_e141920_d_n2;
        locals.var_arg_dn4 = assign92610_e141920_d_n4;
        locals.var_arg_dn5 = assign92610_e141920_d_n5;
        locals.var_arg_dn6 = assign92610_e141920_d_n6;
        locals.var_arg_dn7 = assign92610_e141920_d_n7;
        locals.var_arg_dn8 = assign92610_e141920_d_n8;
        locals.var_arg_dn9 = assign92610_e141920_d_n9;
        locals.var_arg_dn10 = assign92610_e141920_d_n10;
        locals.var_arg_dn11 = assign92610_e141920_d_n11;
        locals.var_arg_dn14 = assign92610_e141920_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign92620_e141935, assign92620_e141935_d_n0, assign92620_e141935_d_n2, assign92620_e141935_d_n4, assign92620_e141935_d_n5, assign92620_e141935_d_n6, assign92620_e141935_d_n7, assign92620_e141935_d_n8, assign92620_e141935_d_n9, assign92620_e141935_d_n10, assign92620_e141935_d_n11, assign92620_e141935_d_n14,) = {
    if ((((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2145 != 0.0)) && (locals.var_guard2147 != 0.0)) && (locals.var_guard2148 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign92620_e141935;
        locals.var_dnm_dn0 = assign92620_e141935_d_n0;
        locals.var_dnm_dn2 = assign92620_e141935_d_n2;
        locals.var_dnm_dn4 = assign92620_e141935_d_n4;
        locals.var_dnm_dn5 = assign92620_e141935_d_n5;
        locals.var_dnm_dn6 = assign92620_e141935_d_n6;
        locals.var_dnm_dn7 = assign92620_e141935_d_n7;
        locals.var_dnm_dn8 = assign92620_e141935_d_n8;
        locals.var_dnm_dn9 = assign92620_e141935_d_n9;
        locals.var_dnm_dn10 = assign92620_e141935_d_n10;
        locals.var_dnm_dn11 = assign92620_e141935_d_n11;
        locals.var_dnm_dn14 = assign92620_e141935_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign92630_e141950: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard2149 = assign92630_e141950;
        locals.var_guard2149_rv = 0.0;

        let assign92640_e141953: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2150 = assign92640_e141953;
        locals.var_guard2150_rv = 0.0;

        let (assign92650_e141972,) = {
    if ((((((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2145 != 0.0)) && (locals.var_guard2147 != 0.0)) && (locals.var_guard2148 != 0.0)) && (locals.var_guard2149 != 0.0)) && (locals.var_guard2150 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign92650_e141972;
        locals.var_mm_rv = 0.0;

        let assign92660_e141975: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard2151 = assign92660_e141975;
        locals.var_guard2151_rv = 0.0;

        let (assign92670_e141997,) = {
    if (((((((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2145 != 0.0)) && (locals.var_guard2147 != 0.0)) && (locals.var_guard2148 != 0.0)) && (locals.var_guard2149 != 0.0)) && (locals.var_guard2150 == 0.0)) && (locals.var_guard2151 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign92670_e141997;
        locals.var_mm_rv = 0.0;

        let assign92680_e142000: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard2152 = assign92680_e142000;
        locals.var_guard2152_rv = 0.0;

        let (assign92690_e142025,) = {
    if ((((((((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2145 != 0.0)) && (locals.var_guard2147 != 0.0)) && (locals.var_guard2148 != 0.0)) && (locals.var_guard2149 != 0.0)) && (locals.var_guard2150 == 0.0)) && (locals.var_guard2151 == 0.0)) && (locals.var_guard2152 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign92690_e142025;
        locals.var_mm_rv = 0.0;

        let assign92700_e142028: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard2153 = assign92700_e142028;
        locals.var_guard2153_rv = 0.0;

        let (assign92710_e142056,) = {
    if (((((((((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2145 != 0.0)) && (locals.var_guard2147 != 0.0)) && (locals.var_guard2148 != 0.0)) && (locals.var_guard2149 != 0.0)) && (locals.var_guard2150 == 0.0)) && (locals.var_guard2151 == 0.0)) && (locals.var_guard2152 == 0.0)) && (locals.var_guard2153 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign92710_e142056;
        locals.var_mm_rv = 0.0;

        let (assign92720_e142073,) = {
    if (((((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2145 != 0.0)) && (locals.var_guard2147 != 0.0)) && (locals.var_guard2148 != 0.0)) && (locals.var_guard2149 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign92720_e142073;
        locals.var_m0_rv = 0.0;

        let mut assign92730_loop_guard: usize = 0;
        while {
            let assign92730_cond_e142091: f64 = if ((((((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2145 != 0.0)) && (locals.var_guard2147 != 0.0)) && (locals.var_guard2148 != 0.0)) && (locals.var_guard2149 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign92730_cond_e142091 != 0.0
        } {
            assign92730_loop_guard += 1;
            assert!(assign92730_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign92730_body0_e142109, assign92730_body0_e142109_d_n0, assign92730_body0_e142109_d_n2, assign92730_body0_e142109_d_n4, assign92730_body0_e142109_d_n5, assign92730_body0_e142109_d_n6, assign92730_body0_e142109_d_n7, assign92730_body0_e142109_d_n8, assign92730_body0_e142109_d_n9, assign92730_body0_e142109_d_n10, assign92730_body0_e142109_d_n11, assign92730_body0_e142109_d_n14,) = {
    if (((((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2145 != 0.0)) && (locals.var_guard2147 != 0.0)) && (locals.var_guard2148 != 0.0)) && (locals.var_guard2149 != 0.0)) {
        let assign92730_body0_e142107: f64 = (locals.var_dnm).sqrt();
        (assign92730_body0_e142107, (locals.var_dnm_dn0 / (2.0 * assign92730_body0_e142107)), (locals.var_dnm_dn2 / (2.0 * assign92730_body0_e142107)), (locals.var_dnm_dn4 / (2.0 * assign92730_body0_e142107)), (locals.var_dnm_dn5 / (2.0 * assign92730_body0_e142107)), (locals.var_dnm_dn6 / (2.0 * assign92730_body0_e142107)), (locals.var_dnm_dn7 / (2.0 * assign92730_body0_e142107)), (locals.var_dnm_dn8 / (2.0 * assign92730_body0_e142107)), (locals.var_dnm_dn9 / (2.0 * assign92730_body0_e142107)), (locals.var_dnm_dn10 / (2.0 * assign92730_body0_e142107)), (locals.var_dnm_dn11 / (2.0 * assign92730_body0_e142107)), (locals.var_dnm_dn14 / (2.0 * assign92730_body0_e142107)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign92730_body0_e142109;
            locals.var_dnm_dn0 = assign92730_body0_e142109_d_n0;
            locals.var_dnm_dn2 = assign92730_body0_e142109_d_n2;
            locals.var_dnm_dn4 = assign92730_body0_e142109_d_n4;
            locals.var_dnm_dn5 = assign92730_body0_e142109_d_n5;
            locals.var_dnm_dn6 = assign92730_body0_e142109_d_n6;
            locals.var_dnm_dn7 = assign92730_body0_e142109_d_n7;
            locals.var_dnm_dn8 = assign92730_body0_e142109_d_n8;
            locals.var_dnm_dn9 = assign92730_body0_e142109_d_n9;
            locals.var_dnm_dn10 = assign92730_body0_e142109_d_n10;
            locals.var_dnm_dn11 = assign92730_body0_e142109_d_n11;
            locals.var_dnm_dn14 = assign92730_body0_e142109_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign92730_body1_e142128,) = {
    if (((((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2145 != 0.0)) && (locals.var_guard2147 != 0.0)) && (locals.var_guard2148 != 0.0)) && (locals.var_guard2149 != 0.0)) {
        let assign92730_body1_e142126: f64 = (locals.var_m0 + 1.0);
        (assign92730_body1_e142126,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign92730_body1_e142128;
            locals.var_m0_rv = 0.0;
        }

        let (assign92740_e142157, assign92740_e142157_d_n0, assign92740_e142157_d_n2, assign92740_e142157_d_n4, assign92740_e142157_d_n5, assign92740_e142157_d_n6, assign92740_e142157_d_n7, assign92740_e142157_d_n8, assign92740_e142157_d_n9, assign92740_e142157_d_n10, assign92740_e142157_d_n11, assign92740_e142157_d_n14,) = {
    if (((((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2145 != 0.0)) && (locals.var_guard2147 != 0.0)) && (locals.var_guard2148 != 0.0)) && (locals.var_guard2149 == 0.0)) {
        let (assign92740_e142155, assign92740_e142155_d_n0, assign92740_e142155_d_n2, assign92740_e142155_d_n4, assign92740_e142155_d_n5, assign92740_e142155_d_n6, assign92740_e142155_d_n7, assign92740_e142155_d_n8, assign92740_e142155_d_n9, assign92740_e142155_d_n10, assign92740_e142155_d_n11, assign92740_e142155_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign92740_e142152: f64 = (2.0 * 2.0);
                let assign92740_e142153: f64 = (1.0 / assign92740_e142152);
                let assign92740_e142154: f64 = (locals.var_dnm).powf(assign92740_e142153);
                (assign92740_e142154, if 0.0 == 0.0 && ((assign92740_e142153) as f64).is_finite() && ((assign92740_e142153) as f64).fract() == 0.0 { if assign92740_e142153 == 0.0 { 0.0 } else { (assign92740_e142153 * ((locals.var_dnm).powf(assign92740_e142153 - 1.0) * locals.var_dnm_dn0)) } } else { (assign92740_e142154 * (assign92740_e142153 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign92740_e142153) as f64).is_finite() && ((assign92740_e142153) as f64).fract() == 0.0 { if assign92740_e142153 == 0.0 { 0.0 } else { (assign92740_e142153 * ((locals.var_dnm).powf(assign92740_e142153 - 1.0) * locals.var_dnm_dn2)) } } else { (assign92740_e142154 * (assign92740_e142153 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign92740_e142153) as f64).is_finite() && ((assign92740_e142153) as f64).fract() == 0.0 { if assign92740_e142153 == 0.0 { 0.0 } else { (assign92740_e142153 * ((locals.var_dnm).powf(assign92740_e142153 - 1.0) * locals.var_dnm_dn4)) } } else { (assign92740_e142154 * (assign92740_e142153 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign92740_e142153) as f64).is_finite() && ((assign92740_e142153) as f64).fract() == 0.0 { if assign92740_e142153 == 0.0 { 0.0 } else { (assign92740_e142153 * ((locals.var_dnm).powf(assign92740_e142153 - 1.0) * locals.var_dnm_dn5)) } } else { (assign92740_e142154 * (assign92740_e142153 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign92740_e142153) as f64).is_finite() && ((assign92740_e142153) as f64).fract() == 0.0 { if assign92740_e142153 == 0.0 { 0.0 } else { (assign92740_e142153 * ((locals.var_dnm).powf(assign92740_e142153 - 1.0) * locals.var_dnm_dn6)) } } else { (assign92740_e142154 * (assign92740_e142153 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign92740_e142153) as f64).is_finite() && ((assign92740_e142153) as f64).fract() == 0.0 { if assign92740_e142153 == 0.0 { 0.0 } else { (assign92740_e142153 * ((locals.var_dnm).powf(assign92740_e142153 - 1.0) * locals.var_dnm_dn7)) } } else { (assign92740_e142154 * (assign92740_e142153 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign92740_e142153) as f64).is_finite() && ((assign92740_e142153) as f64).fract() == 0.0 { if assign92740_e142153 == 0.0 { 0.0 } else { (assign92740_e142153 * ((locals.var_dnm).powf(assign92740_e142153 - 1.0) * locals.var_dnm_dn8)) } } else { (assign92740_e142154 * (assign92740_e142153 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign92740_e142153) as f64).is_finite() && ((assign92740_e142153) as f64).fract() == 0.0 { if assign92740_e142153 == 0.0 { 0.0 } else { (assign92740_e142153 * ((locals.var_dnm).powf(assign92740_e142153 - 1.0) * locals.var_dnm_dn9)) } } else { (assign92740_e142154 * (assign92740_e142153 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign92740_e142153) as f64).is_finite() && ((assign92740_e142153) as f64).fract() == 0.0 { if assign92740_e142153 == 0.0 { 0.0 } else { (assign92740_e142153 * ((locals.var_dnm).powf(assign92740_e142153 - 1.0) * locals.var_dnm_dn10)) } } else { (assign92740_e142154 * (assign92740_e142153 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign92740_e142153) as f64).is_finite() && ((assign92740_e142153) as f64).fract() == 0.0 { if assign92740_e142153 == 0.0 { 0.0 } else { (assign92740_e142153 * ((locals.var_dnm).powf(assign92740_e142153 - 1.0) * locals.var_dnm_dn11)) } } else { (assign92740_e142154 * (assign92740_e142153 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign92740_e142153) as f64).is_finite() && ((assign92740_e142153) as f64).fract() == 0.0 { if assign92740_e142153 == 0.0 { 0.0 } else { (assign92740_e142153 * ((locals.var_dnm).powf(assign92740_e142153 - 1.0) * locals.var_dnm_dn14)) } } else { (assign92740_e142154 * (assign92740_e142153 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign92740_e142155, assign92740_e142155_d_n0, assign92740_e142155_d_n2, assign92740_e142155_d_n4, assign92740_e142155_d_n5, assign92740_e142155_d_n6, assign92740_e142155_d_n7, assign92740_e142155_d_n8, assign92740_e142155_d_n9, assign92740_e142155_d_n10, assign92740_e142155_d_n11, assign92740_e142155_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign92740_e142157;
        locals.var_dnm_dn0 = assign92740_e142157_d_n0;
        locals.var_dnm_dn2 = assign92740_e142157_d_n2;
        locals.var_dnm_dn4 = assign92740_e142157_d_n4;
        locals.var_dnm_dn5 = assign92740_e142157_d_n5;
        locals.var_dnm_dn6 = assign92740_e142157_d_n6;
        locals.var_dnm_dn7 = assign92740_e142157_d_n7;
        locals.var_dnm_dn8 = assign92740_e142157_d_n8;
        locals.var_dnm_dn9 = assign92740_e142157_d_n9;
        locals.var_dnm_dn10 = assign92740_e142157_d_n10;
        locals.var_dnm_dn11 = assign92740_e142157_d_n11;
        locals.var_dnm_dn14 = assign92740_e142157_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign92750_e142174, assign92750_e142174_d_n0, assign92750_e142174_d_n2, assign92750_e142174_d_n4, assign92750_e142174_d_n5, assign92750_e142174_d_n6, assign92750_e142174_d_n7, assign92750_e142174_d_n8, assign92750_e142174_d_n9, assign92750_e142174_d_n10, assign92750_e142174_d_n11, assign92750_e142174_d_n14,) = {
    if ((((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2145 != 0.0)) && (locals.var_guard2147 != 0.0)) && (locals.var_guard2148 != 0.0)) {
        let assign92750_e142172: f64 = (1.0 / locals.var_dnm);
        (assign92750_e142172, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign92750_e142174;
        locals.var_dnm_dn0 = assign92750_e142174_d_n0;
        locals.var_dnm_dn2 = assign92750_e142174_d_n2;
        locals.var_dnm_dn4 = assign92750_e142174_d_n4;
        locals.var_dnm_dn5 = assign92750_e142174_d_n5;
        locals.var_dnm_dn6 = assign92750_e142174_d_n6;
        locals.var_dnm_dn7 = assign92750_e142174_d_n7;
        locals.var_dnm_dn8 = assign92750_e142174_d_n8;
        locals.var_dnm_dn9 = assign92750_e142174_d_n9;
        locals.var_dnm_dn10 = assign92750_e142174_d_n10;
        locals.var_dnm_dn11 = assign92750_e142174_d_n11;
        locals.var_dnm_dn14 = assign92750_e142174_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign92760_e142195, assign92760_e142195_d_n0, assign92760_e142195_d_n2, assign92760_e142195_d_n4, assign92760_e142195_d_n5, assign92760_e142195_d_n6, assign92760_e142195_d_n7, assign92760_e142195_d_n8, assign92760_e142195_d_n9, assign92760_e142195_d_n10, assign92760_e142195_d_n11, assign92760_e142195_d_n14,) = {
    if ((((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2145 != 0.0)) && (locals.var_guard2147 != 0.0)) && (locals.var_guard2148 != 0.0)) {
        let assign92760_e142190: f64 = (0.2 * locals.var_chi_b);
        let assign92760_e142191: f64 = (locals.var_tmf1 * assign92760_e142190);
        let assign92760_e142193: f64 = (assign92760_e142191 * locals.var_dnm);
        (assign92760_e142193, ((((locals.var_tmf1_dn0 * assign92760_e142190) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn0))) * locals.var_dnm) + (assign92760_e142191 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * assign92760_e142190) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn2))) * locals.var_dnm) + (assign92760_e142191 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn4 * assign92760_e142190) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn4))) * locals.var_dnm) + (assign92760_e142191 * locals.var_dnm_dn4)), ((((locals.var_tmf1_dn5 * assign92760_e142190) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn5))) * locals.var_dnm) + (assign92760_e142191 * locals.var_dnm_dn5)), ((((locals.var_tmf1_dn6 * assign92760_e142190) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn6))) * locals.var_dnm) + (assign92760_e142191 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * assign92760_e142190) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn7))) * locals.var_dnm) + (assign92760_e142191 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn8 * assign92760_e142190) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn8))) * locals.var_dnm) + (assign92760_e142191 * locals.var_dnm_dn8)), ((((locals.var_tmf1_dn9 * assign92760_e142190) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn9))) * locals.var_dnm) + (assign92760_e142191 * locals.var_dnm_dn9)), ((((locals.var_tmf1_dn10 * assign92760_e142190) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn10))) * locals.var_dnm) + (assign92760_e142191 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn11 * assign92760_e142190) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn11))) * locals.var_dnm) + (assign92760_e142191 * locals.var_dnm_dn11)), ((((locals.var_tmf1_dn14 * assign92760_e142190) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn14))) * locals.var_dnm) + (assign92760_e142191 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign92760_e142195;
        locals.var_tmf0_dn0 = assign92760_e142195_d_n0;
        locals.var_tmf0_dn2 = assign92760_e142195_d_n2;
        locals.var_tmf0_dn4 = assign92760_e142195_d_n4;
        locals.var_tmf0_dn5 = assign92760_e142195_d_n5;
        locals.var_tmf0_dn6 = assign92760_e142195_d_n6;
        locals.var_tmf0_dn7 = assign92760_e142195_d_n7;
        locals.var_tmf0_dn8 = assign92760_e142195_d_n8;
        locals.var_tmf0_dn9 = assign92760_e142195_d_n9;
        locals.var_tmf0_dn10 = assign92760_e142195_d_n10;
        locals.var_tmf0_dn11 = assign92760_e142195_d_n11;
        locals.var_tmf0_dn14 = assign92760_e142195_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign92770_e142218, assign92770_e142218_d_n0, assign92770_e142218_d_n2, assign92770_e142218_d_n4, assign92770_e142218_d_n5, assign92770_e142218_d_n6, assign92770_e142218_d_n7, assign92770_e142218_d_n8, assign92770_e142218_d_n9, assign92770_e142218_d_n10, assign92770_e142218_d_n11, assign92770_e142218_d_n14,) = {
    if ((((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2145 != 0.0)) && (locals.var_guard2147 != 0.0)) && (locals.var_guard2148 != 0.0)) {
        let assign92770_e142210: f64 = (0.2 * locals.var_chi_b);
        let assign92770_e142212: f64 = (assign92770_e142210 * locals.var_xmp);
        let assign92770_e142214: f64 = (assign92770_e142212 * locals.var_dnm);
        let assign92770_e142216: f64 = (assign92770_e142214 / locals.var_arg);
        (assign92770_e142216, ((((((((0.2 * locals.var_chi_b_dn0) * locals.var_xmp) + (assign92770_e142210 * locals.var_xmp_dn0)) * locals.var_dnm) + (assign92770_e142212 * locals.var_dnm_dn0)) * locals.var_arg) - (assign92770_e142214 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn2) * locals.var_xmp) + (assign92770_e142210 * locals.var_xmp_dn2)) * locals.var_dnm) + (assign92770_e142212 * locals.var_dnm_dn2)) * locals.var_arg) - (assign92770_e142214 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn4) * locals.var_xmp) + (assign92770_e142210 * locals.var_xmp_dn4)) * locals.var_dnm) + (assign92770_e142212 * locals.var_dnm_dn4)) * locals.var_arg) - (assign92770_e142214 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn5) * locals.var_xmp) + (assign92770_e142210 * locals.var_xmp_dn5)) * locals.var_dnm) + (assign92770_e142212 * locals.var_dnm_dn5)) * locals.var_arg) - (assign92770_e142214 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn6) * locals.var_xmp) + (assign92770_e142210 * locals.var_xmp_dn6)) * locals.var_dnm) + (assign92770_e142212 * locals.var_dnm_dn6)) * locals.var_arg) - (assign92770_e142214 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn7) * locals.var_xmp) + (assign92770_e142210 * locals.var_xmp_dn7)) * locals.var_dnm) + (assign92770_e142212 * locals.var_dnm_dn7)) * locals.var_arg) - (assign92770_e142214 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn8) * locals.var_xmp) + (assign92770_e142210 * locals.var_xmp_dn8)) * locals.var_dnm) + (assign92770_e142212 * locals.var_dnm_dn8)) * locals.var_arg) - (assign92770_e142214 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn9) * locals.var_xmp) + (assign92770_e142210 * locals.var_xmp_dn9)) * locals.var_dnm) + (assign92770_e142212 * locals.var_dnm_dn9)) * locals.var_arg) - (assign92770_e142214 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn10) * locals.var_xmp) + (assign92770_e142210 * locals.var_xmp_dn10)) * locals.var_dnm) + (assign92770_e142212 * locals.var_dnm_dn10)) * locals.var_arg) - (assign92770_e142214 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn11) * locals.var_xmp) + (assign92770_e142210 * locals.var_xmp_dn11)) * locals.var_dnm) + (assign92770_e142212 * locals.var_dnm_dn11)) * locals.var_arg) - (assign92770_e142214 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn14) * locals.var_xmp) + (assign92770_e142210 * locals.var_xmp_dn14)) * locals.var_dnm) + (assign92770_e142212 * locals.var_dnm_dn14)) * locals.var_arg) - (assign92770_e142214 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign92770_e142218;
        locals.var_t1_dn0 = assign92770_e142218_d_n0;
        locals.var_t1_dn2 = assign92770_e142218_d_n2;
        locals.var_t1_dn4 = assign92770_e142218_d_n4;
        locals.var_t1_dn5 = assign92770_e142218_d_n5;
        locals.var_t1_dn6 = assign92770_e142218_d_n6;
        locals.var_t1_dn7 = assign92770_e142218_d_n7;
        locals.var_t1_dn8 = assign92770_e142218_d_n8;
        locals.var_t1_dn9 = assign92770_e142218_d_n9;
        locals.var_t1_dn10 = assign92770_e142218_d_n10;
        locals.var_t1_dn11 = assign92770_e142218_d_n11;
        locals.var_t1_dn14 = assign92770_e142218_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign92780_e142239, assign92780_e142239_d_n0, assign92780_e142239_d_n2, assign92780_e142239_d_n4, assign92780_e142239_d_n5, assign92780_e142239_d_n6, assign92780_e142239_d_n7, assign92780_e142239_d_n8, assign92780_e142239_d_n9, assign92780_e142239_d_n10, assign92780_e142239_d_n11, assign92780_e142239_d_n14,) = {
    if ((((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2145 != 0.0)) && (locals.var_guard2147 != 0.0)) && (locals.var_guard2148 != 0.0)) {
        let assign92780_e142234: f64 = (0.2 * locals.var_chi_b);
        let assign92780_e142235: f64 = (locals.var_chi_b - assign92780_e142234);
        let assign92780_e142237: f64 = (assign92780_e142235 + locals.var_tmf0);
        (assign92780_e142237, ((locals.var_chi_b_dn0 - (0.2 * locals.var_chi_b_dn0)) + locals.var_tmf0_dn0), ((locals.var_chi_b_dn2 - (0.2 * locals.var_chi_b_dn2)) + locals.var_tmf0_dn2), ((locals.var_chi_b_dn4 - (0.2 * locals.var_chi_b_dn4)) + locals.var_tmf0_dn4), ((locals.var_chi_b_dn5 - (0.2 * locals.var_chi_b_dn5)) + locals.var_tmf0_dn5), ((locals.var_chi_b_dn6 - (0.2 * locals.var_chi_b_dn6)) + locals.var_tmf0_dn6), ((locals.var_chi_b_dn7 - (0.2 * locals.var_chi_b_dn7)) + locals.var_tmf0_dn7), ((locals.var_chi_b_dn8 - (0.2 * locals.var_chi_b_dn8)) + locals.var_tmf0_dn8), ((locals.var_chi_b_dn9 - (0.2 * locals.var_chi_b_dn9)) + locals.var_tmf0_dn9), ((locals.var_chi_b_dn10 - (0.2 * locals.var_chi_b_dn10)) + locals.var_tmf0_dn10), ((locals.var_chi_b_dn11 - (0.2 * locals.var_chi_b_dn11)) + locals.var_tmf0_dn11), ((locals.var_chi_b_dn14 - (0.2 * locals.var_chi_b_dn14)) + locals.var_tmf0_dn14),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
        locals.var_chi = assign92780_e142239;
        locals.var_chi_dn0 = assign92780_e142239_d_n0;
        locals.var_chi_dn2 = assign92780_e142239_d_n2;
        locals.var_chi_dn4 = assign92780_e142239_d_n4;
        locals.var_chi_dn5 = assign92780_e142239_d_n5;
        locals.var_chi_dn6 = assign92780_e142239_d_n6;
        locals.var_chi_dn7 = assign92780_e142239_d_n7;
        locals.var_chi_dn8 = assign92780_e142239_d_n8;
        locals.var_chi_dn9 = assign92780_e142239_d_n9;
        locals.var_chi_dn10 = assign92780_e142239_d_n10;
        locals.var_chi_dn11 = assign92780_e142239_d_n11;
        locals.var_chi_dn14 = assign92780_e142239_d_n14;
        locals.var_chi_rv = 0.0;

        let (assign92790_e142254, assign92790_e142254_d_n0, assign92790_e142254_d_n2, assign92790_e142254_d_n4, assign92790_e142254_d_n5, assign92790_e142254_d_n6, assign92790_e142254_d_n7, assign92790_e142254_d_n8, assign92790_e142254_d_n9, assign92790_e142254_d_n10, assign92790_e142254_d_n11, assign92790_e142254_d_n14,) = {
    if ((((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2145 != 0.0)) && (locals.var_guard2147 != 0.0)) && (locals.var_guard2148 != 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign92790_e142254;
        locals.var_t1_dn0 = assign92790_e142254_d_n0;
        locals.var_t1_dn2 = assign92790_e142254_d_n2;
        locals.var_t1_dn4 = assign92790_e142254_d_n4;
        locals.var_t1_dn5 = assign92790_e142254_d_n5;
        locals.var_t1_dn6 = assign92790_e142254_d_n6;
        locals.var_t1_dn7 = assign92790_e142254_d_n7;
        locals.var_t1_dn8 = assign92790_e142254_d_n8;
        locals.var_t1_dn9 = assign92790_e142254_d_n9;
        locals.var_t1_dn10 = assign92790_e142254_d_n10;
        locals.var_t1_dn11 = assign92790_e142254_d_n11;
        locals.var_t1_dn14 = assign92790_e142254_d_n14;
        locals.var_t1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_358(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign92800_e142270, assign92800_e142270_d_n0, assign92800_e142270_d_n2, assign92800_e142270_d_n4, assign92800_e142270_d_n5, assign92800_e142270_d_n6, assign92800_e142270_d_n7, assign92800_e142270_d_n8, assign92800_e142270_d_n9, assign92800_e142270_d_n10, assign92800_e142270_d_n11, assign92800_e142270_d_n14,) = {
    if ((((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2145 != 0.0)) && (locals.var_guard2147 != 0.0)) && (locals.var_guard2148 == 0.0)) {
        (locals.var_chi_a, locals.var_chi_a_dn0, locals.var_chi_a_dn2, locals.var_chi_a_dn4, locals.var_chi_a_dn5, locals.var_chi_a_dn6, locals.var_chi_a_dn7, locals.var_chi_a_dn8, locals.var_chi_a_dn9, locals.var_chi_a_dn10, locals.var_chi_a_dn11, locals.var_chi_a_dn14,)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
        locals.var_chi = assign92800_e142270;
        locals.var_chi_dn0 = assign92800_e142270_d_n0;
        locals.var_chi_dn2 = assign92800_e142270_d_n2;
        locals.var_chi_dn4 = assign92800_e142270_d_n4;
        locals.var_chi_dn5 = assign92800_e142270_d_n5;
        locals.var_chi_dn6 = assign92800_e142270_d_n6;
        locals.var_chi_dn7 = assign92800_e142270_d_n7;
        locals.var_chi_dn8 = assign92800_e142270_d_n8;
        locals.var_chi_dn9 = assign92800_e142270_d_n9;
        locals.var_chi_dn10 = assign92800_e142270_d_n10;
        locals.var_chi_dn11 = assign92800_e142270_d_n11;
        locals.var_chi_dn14 = assign92800_e142270_d_n14;
        locals.var_chi_rv = 0.0;

        let (assign92810_e142286, assign92810_e142286_d_n0, assign92810_e142286_d_n2, assign92810_e142286_d_n4, assign92810_e142286_d_n5, assign92810_e142286_d_n6, assign92810_e142286_d_n7, assign92810_e142286_d_n8, assign92810_e142286_d_n9, assign92810_e142286_d_n10, assign92810_e142286_d_n11, assign92810_e142286_d_n14,) = {
    if ((((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2145 != 0.0)) && (locals.var_guard2147 != 0.0)) && (locals.var_guard2148 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign92810_e142286;
        locals.var_t1_dn0 = assign92810_e142286_d_n0;
        locals.var_t1_dn2 = assign92810_e142286_d_n2;
        locals.var_t1_dn4 = assign92810_e142286_d_n4;
        locals.var_t1_dn5 = assign92810_e142286_d_n5;
        locals.var_t1_dn6 = assign92810_e142286_d_n6;
        locals.var_t1_dn7 = assign92810_e142286_d_n7;
        locals.var_t1_dn8 = assign92810_e142286_d_n8;
        locals.var_t1_dn9 = assign92810_e142286_d_n9;
        locals.var_t1_dn10 = assign92810_e142286_d_n10;
        locals.var_t1_dn11 = assign92810_e142286_d_n11;
        locals.var_t1_dn14 = assign92810_e142286_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign92820_e142305, assign92820_e142305_d_n0, assign92820_e142305_d_n2, assign92820_e142305_d_n4, assign92820_e142305_d_n5, assign92820_e142305_d_n6, assign92820_e142305_d_n7, assign92820_e142305_d_n8, assign92820_e142305_d_n9, assign92820_e142305_d_n10, assign92820_e142305_d_n11, assign92820_e142305_d_n14,) = {
    if (((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2145 != 0.0)) && (locals.var_guard2147 == 0.0)) {
        let (assign92820_e142303, assign92820_e142303_d_n0, assign92820_e142303_d_n2, assign92820_e142303_d_n4, assign92820_e142303_d_n5, assign92820_e142303_d_n6, assign92820_e142303_d_n7, assign92820_e142303_d_n8, assign92820_e142303_d_n9, assign92820_e142303_d_n10, assign92820_e142303_d_n11, assign92820_e142303_d_n14,) = {
            if (locals.var_chi_a <= locals.var_chi_b) {
                (locals.var_chi_a, locals.var_chi_a_dn0, locals.var_chi_a_dn2, locals.var_chi_a_dn4, locals.var_chi_a_dn5, locals.var_chi_a_dn6, locals.var_chi_a_dn7, locals.var_chi_a_dn8, locals.var_chi_a_dn9, locals.var_chi_a_dn10, locals.var_chi_a_dn11, locals.var_chi_a_dn14,)
            } else {
                (locals.var_chi_b, locals.var_chi_b_dn0, locals.var_chi_b_dn2, locals.var_chi_b_dn4, locals.var_chi_b_dn5, locals.var_chi_b_dn6, locals.var_chi_b_dn7, locals.var_chi_b_dn8, locals.var_chi_b_dn9, locals.var_chi_b_dn10, locals.var_chi_b_dn11, locals.var_chi_b_dn14,)
            }
        };
        (assign92820_e142303, assign92820_e142303_d_n0, assign92820_e142303_d_n2, assign92820_e142303_d_n4, assign92820_e142303_d_n5, assign92820_e142303_d_n6, assign92820_e142303_d_n7, assign92820_e142303_d_n8, assign92820_e142303_d_n9, assign92820_e142303_d_n10, assign92820_e142303_d_n11, assign92820_e142303_d_n14,)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
        locals.var_chi = assign92820_e142305;
        locals.var_chi_dn0 = assign92820_e142305_d_n0;
        locals.var_chi_dn2 = assign92820_e142305_d_n2;
        locals.var_chi_dn4 = assign92820_e142305_d_n4;
        locals.var_chi_dn5 = assign92820_e142305_d_n5;
        locals.var_chi_dn6 = assign92820_e142305_d_n6;
        locals.var_chi_dn7 = assign92820_e142305_d_n7;
        locals.var_chi_dn8 = assign92820_e142305_d_n8;
        locals.var_chi_dn9 = assign92820_e142305_d_n9;
        locals.var_chi_dn10 = assign92820_e142305_d_n10;
        locals.var_chi_dn11 = assign92820_e142305_d_n11;
        locals.var_chi_dn14 = assign92820_e142305_d_n14;
        locals.var_chi_rv = 0.0;

        let assign92830_e142308: f64 = if p.p33 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2154 = assign92830_e142308;
        locals.var_guard2154_rv = 0.0;

        let (assign92840_e142323, assign92840_e142323_d_n0, assign92840_e142323_d_n2, assign92840_e142323_d_n4, assign92840_e142323_d_n5, assign92840_e142323_d_n6, assign92840_e142323_d_n7, assign92840_e142323_d_n8, assign92840_e142323_d_n9, assign92840_e142323_d_n10, assign92840_e142323_d_n11, assign92840_e142323_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2154 != 0.0)) {
        let assign92840_e142319: f64 = (locals.var_chi * locals.var_beta_inv);
        let assign92840_e142321: f64 = (assign92840_e142319 - locals.var_vxbgmtcl);
        (assign92840_e142321, (((locals.var_chi_dn0 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn0)) - locals.var_vxbgmtcl_dn0), (((locals.var_chi_dn2 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn2)) - locals.var_vxbgmtcl_dn2), (((locals.var_chi_dn4 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn4)) - locals.var_vxbgmtcl_dn4), (((locals.var_chi_dn5 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn5)) - locals.var_vxbgmtcl_dn5), (((locals.var_chi_dn6 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn6)) - locals.var_vxbgmtcl_dn6), (((locals.var_chi_dn7 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn7)) - locals.var_vxbgmtcl_dn7), (((locals.var_chi_dn8 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn8)) - locals.var_vxbgmtcl_dn8), (((locals.var_chi_dn9 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn9)) - locals.var_vxbgmtcl_dn9), (((locals.var_chi_dn10 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn10)) - locals.var_vxbgmtcl_dn10), (((locals.var_chi_dn11 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn11)) - locals.var_vxbgmtcl_dn11), (((locals.var_chi_dn14 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn14)) - locals.var_vxbgmtcl_dn14),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn14,)
    }
};
        locals.var_ps0ld = assign92840_e142323;
        locals.var_ps0ld_dn0 = assign92840_e142323_d_n0;
        locals.var_ps0ld_dn2 = assign92840_e142323_d_n2;
        locals.var_ps0ld_dn4 = assign92840_e142323_d_n4;
        locals.var_ps0ld_dn5 = assign92840_e142323_d_n5;
        locals.var_ps0ld_dn6 = assign92840_e142323_d_n6;
        locals.var_ps0ld_dn7 = assign92840_e142323_d_n7;
        locals.var_ps0ld_dn8 = assign92840_e142323_d_n8;
        locals.var_ps0ld_dn9 = assign92840_e142323_d_n9;
        locals.var_ps0ld_dn10 = assign92840_e142323_d_n10;
        locals.var_ps0ld_dn11 = assign92840_e142323_d_n11;
        locals.var_ps0ld_dn14 = assign92840_e142323_d_n14;
        locals.var_ps0ld_rv = 0.0;

        let assign92850_e142326: f64 = if locals.var_wdep_func > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2155 = assign92850_e142326;
        locals.var_guard2155_rv = 0.0;

        let (assign92860_e142341, assign92860_e142341_d_n0, assign92860_e142341_d_n2, assign92860_e142341_d_n4, assign92860_e142341_d_n5, assign92860_e142341_d_n6, assign92860_e142341_d_n7, assign92860_e142341_d_n8, assign92860_e142341_d_n9, assign92860_e142341_d_n10, assign92860_e142341_d_n11, assign92860_e142341_d_n14,) = {
    if (((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2154 != 0.0)) && (locals.var_guard2155 != 0.0)) {
        let assign92860_e142339: f64 = (p.p334 - locals.var_wdep_func);
        (assign92860_e142339, (-locals.var_wdep_func_dn0), (-locals.var_wdep_func_dn2), (-locals.var_wdep_func_dn4), (-locals.var_wdep_func_dn5), (-locals.var_wdep_func_dn6), (-locals.var_wdep_func_dn7), (-locals.var_wdep_func_dn8), (-locals.var_wdep_func_dn9), (-locals.var_wdep_func_dn10), (-locals.var_wdep_func_dn11), (-locals.var_wdep_func_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign92860_e142341;
        locals.var_t2_dn0 = assign92860_e142341_d_n0;
        locals.var_t2_dn2 = assign92860_e142341_d_n2;
        locals.var_t2_dn4 = assign92860_e142341_d_n4;
        locals.var_t2_dn5 = assign92860_e142341_d_n5;
        locals.var_t2_dn6 = assign92860_e142341_d_n6;
        locals.var_t2_dn7 = assign92860_e142341_d_n7;
        locals.var_t2_dn8 = assign92860_e142341_d_n8;
        locals.var_t2_dn9 = assign92860_e142341_d_n9;
        locals.var_t2_dn10 = assign92860_e142341_d_n10;
        locals.var_t2_dn11 = assign92860_e142341_d_n11;
        locals.var_t2_dn14 = assign92860_e142341_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign92870_e142368, assign92870_e142368_d_n0, assign92870_e142368_d_n2, assign92870_e142368_d_n4, assign92870_e142368_d_n5, assign92870_e142368_d_n6, assign92870_e142368_d_n7, assign92870_e142368_d_n8, assign92870_e142368_d_n9, assign92870_e142368_d_n10, assign92870_e142368_d_n11, assign92870_e142368_d_n14,) = {
    if (((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2154 != 0.0)) && (locals.var_guard2155 == 0.0)) {
        let assign92870_e142355: f64 = (locals.var_vdsi + p.p137);
        let assign92870_e142358: f64 = (locals.var_vdsi + p.p137);
        let assign92870_e142359: f64 = (assign92870_e142355 * assign92870_e142358);
        let assign92870_e142362: f64 = (4.0 * 0.1);
        let assign92870_e142364: f64 = (assign92870_e142362 * 0.1);
        let assign92870_e142365: f64 = (assign92870_e142359 + assign92870_e142364);
        let assign92870_e142366: f64 = (assign92870_e142365).sqrt();
        (assign92870_e142366, 0.0, 0.0, 0.0, 0.0, (((locals.var_vdsi_dn6 * assign92870_e142358) + (assign92870_e142355 * locals.var_vdsi_dn6)) / (2.0 * assign92870_e142366)), 0.0, (((locals.var_vdsi_dn8 * assign92870_e142358) + (assign92870_e142355 * locals.var_vdsi_dn8)) / (2.0 * assign92870_e142366)), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign92870_e142368;
        locals.var_tmf2_dn0 = assign92870_e142368_d_n0;
        locals.var_tmf2_dn2 = assign92870_e142368_d_n2;
        locals.var_tmf2_dn4 = assign92870_e142368_d_n4;
        locals.var_tmf2_dn5 = assign92870_e142368_d_n5;
        locals.var_tmf2_dn6 = assign92870_e142368_d_n6;
        locals.var_tmf2_dn7 = assign92870_e142368_d_n7;
        locals.var_tmf2_dn8 = assign92870_e142368_d_n8;
        locals.var_tmf2_dn9 = assign92870_e142368_d_n9;
        locals.var_tmf2_dn10 = assign92870_e142368_d_n10;
        locals.var_tmf2_dn11 = assign92870_e142368_d_n11;
        locals.var_tmf2_dn14 = assign92870_e142368_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign92880_e142390, assign92880_e142390_d_n0, assign92880_e142390_d_n2, assign92880_e142390_d_n4, assign92880_e142390_d_n5, assign92880_e142390_d_n6, assign92880_e142390_d_n7, assign92880_e142390_d_n8, assign92880_e142390_d_n9, assign92880_e142390_d_n10, assign92880_e142390_d_n11, assign92880_e142390_d_n14,) = {
    if (((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2154 != 0.0)) && (locals.var_guard2155 == 0.0)) {
        let assign92880_e142384: f64 = (locals.var_vdsi + p.p137);
        let assign92880_e142386: f64 = (assign92880_e142384 / locals.var_tmf2);
        let assign92880_e142387: f64 = (1.0 + assign92880_e142386);
        let assign92880_e142388: f64 = (0.5 * assign92880_e142387);
        (assign92880_e142388, (0.5 * (-((assign92880_e142384 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign92880_e142384 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign92880_e142384 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign92880_e142384 * locals.var_tmf2_dn5) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (((locals.var_vdsi_dn6 * locals.var_tmf2) - (assign92880_e142384 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (-((assign92880_e142384 * locals.var_tmf2_dn7) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (((locals.var_vdsi_dn8 * locals.var_tmf2) - (assign92880_e142384 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (-((assign92880_e142384 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign92880_e142384 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign92880_e142384 * locals.var_tmf2_dn11) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign92880_e142384 * locals.var_tmf2_dn14) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign92880_e142390;
        locals.var_t9_dn0 = assign92880_e142390_d_n0;
        locals.var_t9_dn2 = assign92880_e142390_d_n2;
        locals.var_t9_dn4 = assign92880_e142390_d_n4;
        locals.var_t9_dn5 = assign92880_e142390_d_n5;
        locals.var_t9_dn6 = assign92880_e142390_d_n6;
        locals.var_t9_dn7 = assign92880_e142390_d_n7;
        locals.var_t9_dn8 = assign92880_e142390_d_n8;
        locals.var_t9_dn9 = assign92880_e142390_d_n9;
        locals.var_t9_dn10 = assign92880_e142390_d_n10;
        locals.var_t9_dn11 = assign92880_e142390_d_n11;
        locals.var_t9_dn14 = assign92880_e142390_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign92890_e142410, assign92890_e142410_d_n0, assign92890_e142410_d_n2, assign92890_e142410_d_n4, assign92890_e142410_d_n5, assign92890_e142410_d_n6, assign92890_e142410_d_n7, assign92890_e142410_d_n8, assign92890_e142410_d_n9, assign92890_e142410_d_n10, assign92890_e142410_d_n11, assign92890_e142410_d_n14,) = {
    if (((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2154 != 0.0)) && (locals.var_guard2155 == 0.0)) {
        let assign92890_e142405: f64 = (locals.var_vdsi + p.p137);
        let assign92890_e142407: f64 = (assign92890_e142405 + locals.var_tmf2);
        let assign92890_e142408: f64 = (0.5 * assign92890_e142407);
        (assign92890_e142408, (0.5 * locals.var_tmf2_dn0), (0.5 * locals.var_tmf2_dn2), (0.5 * locals.var_tmf2_dn4), (0.5 * locals.var_tmf2_dn5), (0.5 * (locals.var_vdsi_dn6 + locals.var_tmf2_dn6)), (0.5 * locals.var_tmf2_dn7), (0.5 * (locals.var_vdsi_dn8 + locals.var_tmf2_dn8)), (0.5 * locals.var_tmf2_dn9), (0.5 * locals.var_tmf2_dn10), (0.5 * locals.var_tmf2_dn11), (0.5 * locals.var_tmf2_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign92890_e142410;
        locals.var_t2_dn0 = assign92890_e142410_d_n0;
        locals.var_t2_dn2 = assign92890_e142410_d_n2;
        locals.var_t2_dn4 = assign92890_e142410_d_n4;
        locals.var_t2_dn5 = assign92890_e142410_d_n5;
        locals.var_t2_dn6 = assign92890_e142410_d_n6;
        locals.var_t2_dn7 = assign92890_e142410_d_n7;
        locals.var_t2_dn8 = assign92890_e142410_d_n8;
        locals.var_t2_dn9 = assign92890_e142410_d_n9;
        locals.var_t2_dn10 = assign92890_e142410_d_n10;
        locals.var_t2_dn11 = assign92890_e142410_d_n11;
        locals.var_t2_dn14 = assign92890_e142410_d_n14;
        locals.var_t2_rv = 0.0;

        let assign92900_e142413: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2156 = assign92900_e142413;
        locals.var_guard2156_rv = 0.0;

        let (assign92910_e142429, assign92910_e142429_d_n0, assign92910_e142429_d_n2, assign92910_e142429_d_n4, assign92910_e142429_d_n5, assign92910_e142429_d_n6, assign92910_e142429_d_n7, assign92910_e142429_d_n8, assign92910_e142429_d_n9, assign92910_e142429_d_n10, assign92910_e142429_d_n11, assign92910_e142429_d_n14,) = {
    if ((((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2154 != 0.0)) && (locals.var_guard2155 == 0.0)) && (locals.var_guard2156 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign92910_e142429;
        locals.var_t2_dn0 = assign92910_e142429_d_n0;
        locals.var_t2_dn2 = assign92910_e142429_d_n2;
        locals.var_t2_dn4 = assign92910_e142429_d_n4;
        locals.var_t2_dn5 = assign92910_e142429_d_n5;
        locals.var_t2_dn6 = assign92910_e142429_d_n6;
        locals.var_t2_dn7 = assign92910_e142429_d_n7;
        locals.var_t2_dn8 = assign92910_e142429_d_n8;
        locals.var_t2_dn9 = assign92910_e142429_d_n9;
        locals.var_t2_dn10 = assign92910_e142429_d_n10;
        locals.var_t2_dn11 = assign92910_e142429_d_n11;
        locals.var_t2_dn14 = assign92910_e142429_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign92920_e142445, assign92920_e142445_d_n0, assign92920_e142445_d_n2, assign92920_e142445_d_n4, assign92920_e142445_d_n5, assign92920_e142445_d_n6, assign92920_e142445_d_n7, assign92920_e142445_d_n8, assign92920_e142445_d_n9, assign92920_e142445_d_n10, assign92920_e142445_d_n11, assign92920_e142445_d_n14,) = {
    if ((((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2154 != 0.0)) && (locals.var_guard2155 == 0.0)) && (locals.var_guard2156 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign92920_e142445;
        locals.var_t9_dn0 = assign92920_e142445_d_n0;
        locals.var_t9_dn2 = assign92920_e142445_d_n2;
        locals.var_t9_dn4 = assign92920_e142445_d_n4;
        locals.var_t9_dn5 = assign92920_e142445_d_n5;
        locals.var_t9_dn6 = assign92920_e142445_d_n6;
        locals.var_t9_dn7 = assign92920_e142445_d_n7;
        locals.var_t9_dn8 = assign92920_e142445_d_n8;
        locals.var_t9_dn9 = assign92920_e142445_d_n9;
        locals.var_t9_dn10 = assign92920_e142445_d_n10;
        locals.var_t9_dn11 = assign92920_e142445_d_n11;
        locals.var_t9_dn14 = assign92920_e142445_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign92930_e142464, assign92930_e142464_d_n0, assign92930_e142464_d_n2, assign92930_e142464_d_n4, assign92930_e142464_d_n5, assign92930_e142464_d_n6, assign92930_e142464_d_n7, assign92930_e142464_d_n8, assign92930_e142464_d_n9, assign92930_e142464_d_n10, assign92930_e142464_d_n11, assign92930_e142464_d_n14,) = {
    if (((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2154 != 0.0)) && (locals.var_guard2155 == 0.0)) {
        let assign92930_e142459: f64 = (locals.var_kjunc * locals.var_t2);
        let assign92930_e142460: f64 = (assign92930_e142459).sqrt();
        let assign92930_e142462: f64 = (assign92930_e142460 * p.p432);
        (assign92930_e142462, ((((locals.var_kjunc_dn0 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn0)) / (2.0 * assign92930_e142460)) * p.p432), ((((locals.var_kjunc_dn2 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn2)) / (2.0 * assign92930_e142460)) * p.p432), ((((locals.var_kjunc_dn4 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn4)) / (2.0 * assign92930_e142460)) * p.p432), ((((locals.var_kjunc_dn5 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn5)) / (2.0 * assign92930_e142460)) * p.p432), ((((locals.var_kjunc_dn6 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn6)) / (2.0 * assign92930_e142460)) * p.p432), ((((locals.var_kjunc_dn7 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn7)) / (2.0 * assign92930_e142460)) * p.p432), ((((locals.var_kjunc_dn8 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn8)) / (2.0 * assign92930_e142460)) * p.p432), ((((locals.var_kjunc_dn9 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn9)) / (2.0 * assign92930_e142460)) * p.p432), ((((locals.var_kjunc_dn10 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn10)) / (2.0 * assign92930_e142460)) * p.p432), ((((locals.var_kjunc_dn11 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn11)) / (2.0 * assign92930_e142460)) * p.p432), ((((locals.var_kjunc_dn14 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn14)) / (2.0 * assign92930_e142460)) * p.p432),)
    } else {
        (locals.var_wjunc0, locals.var_wjunc0_dn0, locals.var_wjunc0_dn2, locals.var_wjunc0_dn4, locals.var_wjunc0_dn5, locals.var_wjunc0_dn6, locals.var_wjunc0_dn7, locals.var_wjunc0_dn8, locals.var_wjunc0_dn9, locals.var_wjunc0_dn10, locals.var_wjunc0_dn11, locals.var_wjunc0_dn14,)
    }
};
        locals.var_wjunc0 = assign92930_e142464;
        locals.var_wjunc0_dn0 = assign92930_e142464_d_n0;
        locals.var_wjunc0_dn2 = assign92930_e142464_d_n2;
        locals.var_wjunc0_dn4 = assign92930_e142464_d_n4;
        locals.var_wjunc0_dn5 = assign92930_e142464_d_n5;
        locals.var_wjunc0_dn6 = assign92930_e142464_d_n6;
        locals.var_wjunc0_dn7 = assign92930_e142464_d_n7;
        locals.var_wjunc0_dn8 = assign92930_e142464_d_n8;
        locals.var_wjunc0_dn9 = assign92930_e142464_d_n9;
        locals.var_wjunc0_dn10 = assign92930_e142464_d_n10;
        locals.var_wjunc0_dn11 = assign92930_e142464_d_n11;
        locals.var_wjunc0_dn14 = assign92930_e142464_d_n14;
        locals.var_wjunc0_rv = 0.0;

        let (assign92940_e142480, assign92940_e142480_d_n0, assign92940_e142480_d_n2, assign92940_e142480_d_n4, assign92940_e142480_d_n5, assign92940_e142480_d_n6, assign92940_e142480_d_n7, assign92940_e142480_d_n8, assign92940_e142480_d_n9, assign92940_e142480_d_n10, assign92940_e142480_d_n11, assign92940_e142480_d_n14,) = {
    if (((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2154 != 0.0)) && (locals.var_guard2155 == 0.0)) {
        let assign92940_e142478: f64 = (p.p334 - locals.var_wjunc0);
        (assign92940_e142478, (-locals.var_wjunc0_dn0), (-locals.var_wjunc0_dn2), (-locals.var_wjunc0_dn4), (-locals.var_wjunc0_dn5), (-locals.var_wjunc0_dn6), (-locals.var_wjunc0_dn7), (-locals.var_wjunc0_dn8), (-locals.var_wjunc0_dn9), (-locals.var_wjunc0_dn10), (-locals.var_wjunc0_dn11), (-locals.var_wjunc0_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign92940_e142480;
        locals.var_t2_dn0 = assign92940_e142480_d_n0;
        locals.var_t2_dn2 = assign92940_e142480_d_n2;
        locals.var_t2_dn4 = assign92940_e142480_d_n4;
        locals.var_t2_dn5 = assign92940_e142480_d_n5;
        locals.var_t2_dn6 = assign92940_e142480_d_n6;
        locals.var_t2_dn7 = assign92940_e142480_d_n7;
        locals.var_t2_dn8 = assign92940_e142480_d_n8;
        locals.var_t2_dn9 = assign92940_e142480_d_n9;
        locals.var_t2_dn10 = assign92940_e142480_d_n10;
        locals.var_t2_dn11 = assign92940_e142480_d_n11;
        locals.var_t2_dn14 = assign92940_e142480_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign92950_e142504, assign92950_e142504_d_n0, assign92950_e142504_d_n2, assign92950_e142504_d_n4, assign92950_e142504_d_n5, assign92950_e142504_d_n6, assign92950_e142504_d_n7, assign92950_e142504_d_n8, assign92950_e142504_d_n9, assign92950_e142504_d_n10, assign92950_e142504_d_n11, assign92950_e142504_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2154 != 0.0)) {
        let assign92950_e142491: f64 = (locals.var_t2 * locals.var_t2);
        let assign92950_e142495: f64 = (p.p334 * 0.01);
        let assign92950_e142496: f64 = (4.0 * assign92950_e142495);
        let assign92950_e142499: f64 = (p.p334 * 0.01);
        let assign92950_e142500: f64 = (assign92950_e142496 * assign92950_e142499);
        let assign92950_e142501: f64 = (assign92950_e142491 + assign92950_e142500);
        let assign92950_e142502: f64 = (assign92950_e142501).sqrt();
        (assign92950_e142502, (((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)) / (2.0 * assign92950_e142502)), (((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)) / (2.0 * assign92950_e142502)), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign92950_e142502)), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign92950_e142502)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign92950_e142502)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign92950_e142502)), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign92950_e142502)), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign92950_e142502)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign92950_e142502)), (((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign92950_e142502)), (((locals.var_t2_dn14 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn14)) / (2.0 * assign92950_e142502)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign92950_e142504;
        locals.var_tmf2_dn0 = assign92950_e142504_d_n0;
        locals.var_tmf2_dn2 = assign92950_e142504_d_n2;
        locals.var_tmf2_dn4 = assign92950_e142504_d_n4;
        locals.var_tmf2_dn5 = assign92950_e142504_d_n5;
        locals.var_tmf2_dn6 = assign92950_e142504_d_n6;
        locals.var_tmf2_dn7 = assign92950_e142504_d_n7;
        locals.var_tmf2_dn8 = assign92950_e142504_d_n8;
        locals.var_tmf2_dn9 = assign92950_e142504_d_n9;
        locals.var_tmf2_dn10 = assign92950_e142504_d_n10;
        locals.var_tmf2_dn11 = assign92950_e142504_d_n11;
        locals.var_tmf2_dn14 = assign92950_e142504_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign92960_e142521, assign92960_e142521_d_n0, assign92960_e142521_d_n2, assign92960_e142521_d_n4, assign92960_e142521_d_n5, assign92960_e142521_d_n6, assign92960_e142521_d_n7, assign92960_e142521_d_n8, assign92960_e142521_d_n9, assign92960_e142521_d_n10, assign92960_e142521_d_n11, assign92960_e142521_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2154 != 0.0)) {
        let assign92960_e142517: f64 = (locals.var_t2 / locals.var_tmf2);
        let assign92960_e142518: f64 = (1.0 + assign92960_e142517);
        let assign92960_e142519: f64 = (0.5 * assign92960_e142518);
        (assign92960_e142519, (0.5 * (((locals.var_t2_dn0 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn2 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn4 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn5 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn6 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn7 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn8 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn9 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn10 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn11 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn14 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign92960_e142521;
        locals.var_t9_dn0 = assign92960_e142521_d_n0;
        locals.var_t9_dn2 = assign92960_e142521_d_n2;
        locals.var_t9_dn4 = assign92960_e142521_d_n4;
        locals.var_t9_dn5 = assign92960_e142521_d_n5;
        locals.var_t9_dn6 = assign92960_e142521_d_n6;
        locals.var_t9_dn7 = assign92960_e142521_d_n7;
        locals.var_t9_dn8 = assign92960_e142521_d_n8;
        locals.var_t9_dn9 = assign92960_e142521_d_n9;
        locals.var_t9_dn10 = assign92960_e142521_d_n10;
        locals.var_t9_dn11 = assign92960_e142521_d_n11;
        locals.var_t9_dn14 = assign92960_e142521_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign92970_e142536, assign92970_e142536_d_n0, assign92970_e142536_d_n2, assign92970_e142536_d_n4, assign92970_e142536_d_n5, assign92970_e142536_d_n6, assign92970_e142536_d_n7, assign92970_e142536_d_n8, assign92970_e142536_d_n9, assign92970_e142536_d_n10, assign92970_e142536_d_n11, assign92970_e142536_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2154 != 0.0)) {
        let assign92970_e142533: f64 = (locals.var_t2 + locals.var_tmf2);
        let assign92970_e142534: f64 = (0.5 * assign92970_e142533);
        (assign92970_e142534, (0.5 * (locals.var_t2_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t2_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t2_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t2_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t2_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t2_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t2_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t2_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t2_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t2_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_t2_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign92970_e142536;
        locals.var_t2_dn0 = assign92970_e142536_d_n0;
        locals.var_t2_dn2 = assign92970_e142536_d_n2;
        locals.var_t2_dn4 = assign92970_e142536_d_n4;
        locals.var_t2_dn5 = assign92970_e142536_d_n5;
        locals.var_t2_dn6 = assign92970_e142536_d_n6;
        locals.var_t2_dn7 = assign92970_e142536_d_n7;
        locals.var_t2_dn8 = assign92970_e142536_d_n8;
        locals.var_t2_dn9 = assign92970_e142536_d_n9;
        locals.var_t2_dn10 = assign92970_e142536_d_n10;
        locals.var_t2_dn11 = assign92970_e142536_d_n11;
        locals.var_t2_dn14 = assign92970_e142536_d_n14;
        locals.var_t2_rv = 0.0;

        let assign92980_e142539: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2157 = assign92980_e142539;
        locals.var_guard2157_rv = 0.0;

        let (assign92990_e142552, assign92990_e142552_d_n0, assign92990_e142552_d_n2, assign92990_e142552_d_n4, assign92990_e142552_d_n5, assign92990_e142552_d_n6, assign92990_e142552_d_n7, assign92990_e142552_d_n8, assign92990_e142552_d_n9, assign92990_e142552_d_n10, assign92990_e142552_d_n11, assign92990_e142552_d_n14,) = {
    if (((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2154 != 0.0)) && (locals.var_guard2157 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign92990_e142552;
        locals.var_t2_dn0 = assign92990_e142552_d_n0;
        locals.var_t2_dn2 = assign92990_e142552_d_n2;
        locals.var_t2_dn4 = assign92990_e142552_d_n4;
        locals.var_t2_dn5 = assign92990_e142552_d_n5;
        locals.var_t2_dn6 = assign92990_e142552_d_n6;
        locals.var_t2_dn7 = assign92990_e142552_d_n7;
        locals.var_t2_dn8 = assign92990_e142552_d_n8;
        locals.var_t2_dn9 = assign92990_e142552_d_n9;
        locals.var_t2_dn10 = assign92990_e142552_d_n10;
        locals.var_t2_dn11 = assign92990_e142552_d_n11;
        locals.var_t2_dn14 = assign92990_e142552_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign93000_e142565, assign93000_e142565_d_n0, assign93000_e142565_d_n2, assign93000_e142565_d_n4, assign93000_e142565_d_n5, assign93000_e142565_d_n6, assign93000_e142565_d_n7, assign93000_e142565_d_n8, assign93000_e142565_d_n9, assign93000_e142565_d_n10, assign93000_e142565_d_n11, assign93000_e142565_d_n14,) = {
    if (((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2154 != 0.0)) && (locals.var_guard2157 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign93000_e142565;
        locals.var_t9_dn0 = assign93000_e142565_d_n0;
        locals.var_t9_dn2 = assign93000_e142565_d_n2;
        locals.var_t9_dn4 = assign93000_e142565_d_n4;
        locals.var_t9_dn5 = assign93000_e142565_d_n5;
        locals.var_t9_dn6 = assign93000_e142565_d_n6;
        locals.var_t9_dn7 = assign93000_e142565_d_n7;
        locals.var_t9_dn8 = assign93000_e142565_d_n8;
        locals.var_t9_dn9 = assign93000_e142565_d_n9;
        locals.var_t9_dn10 = assign93000_e142565_d_n10;
        locals.var_t9_dn11 = assign93000_e142565_d_n11;
        locals.var_t9_dn14 = assign93000_e142565_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign93010_e142576, assign93010_e142576_d_n0, assign93010_e142576_d_n2, assign93010_e142576_d_n4, assign93010_e142576_d_n5, assign93010_e142576_d_n6, assign93010_e142576_d_n7, assign93010_e142576_d_n8, assign93010_e142576_d_n9, assign93010_e142576_d_n10, assign93010_e142576_d_n11, assign93010_e142576_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2154 != 0.0)) {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    } else {
        (locals.var_ddriftldc, locals.var_ddriftldc_dn0, locals.var_ddriftldc_dn2, locals.var_ddriftldc_dn4, locals.var_ddriftldc_dn5, locals.var_ddriftldc_dn6, locals.var_ddriftldc_dn7, locals.var_ddriftldc_dn8, locals.var_ddriftldc_dn9, locals.var_ddriftldc_dn10, locals.var_ddriftldc_dn11, locals.var_ddriftldc_dn14,)
    }
};
        locals.var_ddriftldc = assign93010_e142576;
        locals.var_ddriftldc_dn0 = assign93010_e142576_d_n0;
        locals.var_ddriftldc_dn2 = assign93010_e142576_d_n2;
        locals.var_ddriftldc_dn4 = assign93010_e142576_d_n4;
        locals.var_ddriftldc_dn5 = assign93010_e142576_d_n5;
        locals.var_ddriftldc_dn6 = assign93010_e142576_d_n6;
        locals.var_ddriftldc_dn7 = assign93010_e142576_d_n7;
        locals.var_ddriftldc_dn8 = assign93010_e142576_d_n8;
        locals.var_ddriftldc_dn9 = assign93010_e142576_d_n9;
        locals.var_ddriftldc_dn10 = assign93010_e142576_d_n10;
        locals.var_ddriftldc_dn11 = assign93010_e142576_d_n11;
        locals.var_ddriftldc_dn14 = assign93010_e142576_d_n14;
        locals.var_ddriftldc_rv = 0.0;

        let (assign93020_e142595, assign93020_e142595_d_n0, assign93020_e142595_d_n2, assign93020_e142595_d_n4, assign93020_e142595_d_n5, assign93020_e142595_d_n6, assign93020_e142595_d_n7, assign93020_e142595_d_n8, assign93020_e142595_d_n9, assign93020_e142595_d_n10, assign93020_e142595_d_n11, assign93020_e142595_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2154 != 0.0)) {
        let assign93020_e142587: f64 = (locals.var_q_nsubld__blk2117 * locals.var_ddriftldc);
        let assign93020_e142589: f64 = (assign93020_e142587 * locals.var_ddriftldc);
        let assign93020_e142591: f64 = (assign93020_e142589 / 2.0);
        let assign93020_e142593: f64 = (assign93020_e142591 / 1.034943e-10);
        (assign93020_e142593, (((((locals.var_q_nsubld__blk2117 * locals.var_ddriftldc_dn0) * locals.var_ddriftldc) + (assign93020_e142587 * locals.var_ddriftldc_dn0)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2117 * locals.var_ddriftldc_dn2) * locals.var_ddriftldc) + (assign93020_e142587 * locals.var_ddriftldc_dn2)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2117 * locals.var_ddriftldc_dn4) * locals.var_ddriftldc) + (assign93020_e142587 * locals.var_ddriftldc_dn4)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2117 * locals.var_ddriftldc_dn5) * locals.var_ddriftldc) + (assign93020_e142587 * locals.var_ddriftldc_dn5)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2117 * locals.var_ddriftldc_dn6) * locals.var_ddriftldc) + (assign93020_e142587 * locals.var_ddriftldc_dn6)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2117 * locals.var_ddriftldc_dn7) * locals.var_ddriftldc) + (assign93020_e142587 * locals.var_ddriftldc_dn7)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2117 * locals.var_ddriftldc_dn8) * locals.var_ddriftldc) + (assign93020_e142587 * locals.var_ddriftldc_dn8)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2117 * locals.var_ddriftldc_dn9) * locals.var_ddriftldc) + (assign93020_e142587 * locals.var_ddriftldc_dn9)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2117 * locals.var_ddriftldc_dn10) * locals.var_ddriftldc) + (assign93020_e142587 * locals.var_ddriftldc_dn10)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2117 * locals.var_ddriftldc_dn11) * locals.var_ddriftldc) + (assign93020_e142587 * locals.var_ddriftldc_dn11)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2117 * locals.var_ddriftldc_dn14) * locals.var_ddriftldc) + (assign93020_e142587 * locals.var_ddriftldc_dn14)) / 2.0) / 1.034943e-10),)
    } else {
        (locals.var_dphi_sb, locals.var_dphi_sb_dn0, locals.var_dphi_sb_dn2, locals.var_dphi_sb_dn4, locals.var_dphi_sb_dn5, locals.var_dphi_sb_dn6, locals.var_dphi_sb_dn7, locals.var_dphi_sb_dn8, locals.var_dphi_sb_dn9, locals.var_dphi_sb_dn10, locals.var_dphi_sb_dn11, locals.var_dphi_sb_dn14,)
    }
};
        locals.var_dphi_sb = assign93020_e142595;
        locals.var_dphi_sb_dn0 = assign93020_e142595_d_n0;
        locals.var_dphi_sb_dn2 = assign93020_e142595_d_n2;
        locals.var_dphi_sb_dn4 = assign93020_e142595_d_n4;
        locals.var_dphi_sb_dn5 = assign93020_e142595_d_n5;
        locals.var_dphi_sb_dn6 = assign93020_e142595_d_n6;
        locals.var_dphi_sb_dn7 = assign93020_e142595_d_n7;
        locals.var_dphi_sb_dn8 = assign93020_e142595_d_n8;
        locals.var_dphi_sb_dn9 = assign93020_e142595_d_n9;
        locals.var_dphi_sb_dn10 = assign93020_e142595_d_n10;
        locals.var_dphi_sb_dn11 = assign93020_e142595_d_n11;
        locals.var_dphi_sb_dn14 = assign93020_e142595_d_n14;
        locals.var_dphi_sb_rv = 0.0;

        let (assign93030_e142611, assign93030_e142611_d_n0, assign93030_e142611_d_n2, assign93030_e142611_d_n4, assign93030_e142611_d_n5, assign93030_e142611_d_n6, assign93030_e142611_d_n7, assign93030_e142611_d_n8, assign93030_e142611_d_n9, assign93030_e142611_d_n10, assign93030_e142611_d_n11, assign93030_e142611_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2154 != 0.0)) {
        let assign93030_e142606: f64 = (2.0 * locals.var_beta);
        let assign93030_e142608: f64 = (assign93030_e142606 * locals.var_dphi_sb);
        let assign93030_e142609: f64 = (assign93030_e142608).sqrt();
        (assign93030_e142609, ((((2.0 * locals.var_beta_dn0) * locals.var_dphi_sb) + (assign93030_e142606 * locals.var_dphi_sb_dn0)) / (2.0 * assign93030_e142609)), ((((2.0 * locals.var_beta_dn2) * locals.var_dphi_sb) + (assign93030_e142606 * locals.var_dphi_sb_dn2)) / (2.0 * assign93030_e142609)), ((((2.0 * locals.var_beta_dn4) * locals.var_dphi_sb) + (assign93030_e142606 * locals.var_dphi_sb_dn4)) / (2.0 * assign93030_e142609)), ((((2.0 * locals.var_beta_dn5) * locals.var_dphi_sb) + (assign93030_e142606 * locals.var_dphi_sb_dn5)) / (2.0 * assign93030_e142609)), ((((2.0 * locals.var_beta_dn6) * locals.var_dphi_sb) + (assign93030_e142606 * locals.var_dphi_sb_dn6)) / (2.0 * assign93030_e142609)), ((((2.0 * locals.var_beta_dn7) * locals.var_dphi_sb) + (assign93030_e142606 * locals.var_dphi_sb_dn7)) / (2.0 * assign93030_e142609)), ((((2.0 * locals.var_beta_dn8) * locals.var_dphi_sb) + (assign93030_e142606 * locals.var_dphi_sb_dn8)) / (2.0 * assign93030_e142609)), ((((2.0 * locals.var_beta_dn9) * locals.var_dphi_sb) + (assign93030_e142606 * locals.var_dphi_sb_dn9)) / (2.0 * assign93030_e142609)), ((((2.0 * locals.var_beta_dn10) * locals.var_dphi_sb) + (assign93030_e142606 * locals.var_dphi_sb_dn10)) / (2.0 * assign93030_e142609)), ((((2.0 * locals.var_beta_dn11) * locals.var_dphi_sb) + (assign93030_e142606 * locals.var_dphi_sb_dn11)) / (2.0 * assign93030_e142609)), ((((2.0 * locals.var_beta_dn14) * locals.var_dphi_sb) + (assign93030_e142606 * locals.var_dphi_sb_dn14)) / (2.0 * assign93030_e142609)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign93030_e142611;
        locals.var_t0_dn0 = assign93030_e142611_d_n0;
        locals.var_t0_dn2 = assign93030_e142611_d_n2;
        locals.var_t0_dn4 = assign93030_e142611_d_n4;
        locals.var_t0_dn5 = assign93030_e142611_d_n5;
        locals.var_t0_dn6 = assign93030_e142611_d_n6;
        locals.var_t0_dn7 = assign93030_e142611_d_n7;
        locals.var_t0_dn8 = assign93030_e142611_d_n8;
        locals.var_t0_dn9 = assign93030_e142611_d_n9;
        locals.var_t0_dn10 = assign93030_e142611_d_n10;
        locals.var_t0_dn11 = assign93030_e142611_d_n11;
        locals.var_t0_dn14 = assign93030_e142611_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign93040_e142629, assign93040_e142629_d_n0, assign93040_e142629_d_n2, assign93040_e142629_d_n4, assign93040_e142629_d_n5, assign93040_e142629_d_n6, assign93040_e142629_d_n7, assign93040_e142629_d_n8, assign93040_e142629_d_n9, assign93040_e142629_d_n10, assign93040_e142629_d_n11, assign93040_e142629_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2154 != 0.0)) {
        let assign93040_e142621: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign93040_e142623: f64 = (-locals.var_t0);
        let assign93040_e142624: f64 = { let limited_exp_arg = assign93040_e142623; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign93040_e142625: f64 = (assign93040_e142621 + assign93040_e142624);
        let assign93040_e142627: f64 = (assign93040_e142625 / 2.0);
        (assign93040_e142627, ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn0) + ({ let limited_exp_arg = assign93040_e142623; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn0))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn2) + ({ let limited_exp_arg = assign93040_e142623; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn2))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4) + ({ let limited_exp_arg = assign93040_e142623; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn4))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5) + ({ let limited_exp_arg = assign93040_e142623; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn5))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6) + ({ let limited_exp_arg = assign93040_e142623; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn6))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7) + ({ let limited_exp_arg = assign93040_e142623; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn7))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8) + ({ let limited_exp_arg = assign93040_e142623; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn8))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9) + ({ let limited_exp_arg = assign93040_e142623; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn9))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10) + ({ let limited_exp_arg = assign93040_e142623; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn10))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11) + ({ let limited_exp_arg = assign93040_e142623; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn11))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn14) + ({ let limited_exp_arg = assign93040_e142623; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn14))) / 2.0),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign93040_e142629;
        locals.var_t1_dn0 = assign93040_e142629_d_n0;
        locals.var_t1_dn2 = assign93040_e142629_d_n2;
        locals.var_t1_dn4 = assign93040_e142629_d_n4;
        locals.var_t1_dn5 = assign93040_e142629_d_n5;
        locals.var_t1_dn6 = assign93040_e142629_d_n6;
        locals.var_t1_dn7 = assign93040_e142629_d_n7;
        locals.var_t1_dn8 = assign93040_e142629_d_n8;
        locals.var_t1_dn9 = assign93040_e142629_d_n9;
        locals.var_t1_dn10 = assign93040_e142629_d_n10;
        locals.var_t1_dn11 = assign93040_e142629_d_n11;
        locals.var_t1_dn14 = assign93040_e142629_d_n14;
        locals.var_t1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_359(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign93050_e142643, assign93050_e142643_d_n0, assign93050_e142643_d_n2, assign93050_e142643_d_n4, assign93050_e142643_d_n5, assign93050_e142643_d_n6, assign93050_e142643_d_n7, assign93050_e142643_d_n8, assign93050_e142643_d_n9, assign93050_e142643_d_n10, assign93050_e142643_d_n11, assign93050_e142643_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2154 != 0.0)) {
        let assign93050_e142639: f64 = (locals.var_t1).ln();
        let assign93050_e142641: f64 = (assign93050_e142639 / locals.var_dphi_sb);
        (assign93050_e142641, ((((locals.var_t1_dn0 / locals.var_t1) * locals.var_dphi_sb) - (assign93050_e142639 * locals.var_dphi_sb_dn0)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn2 / locals.var_t1) * locals.var_dphi_sb) - (assign93050_e142639 * locals.var_dphi_sb_dn2)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn4 / locals.var_t1) * locals.var_dphi_sb) - (assign93050_e142639 * locals.var_dphi_sb_dn4)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn5 / locals.var_t1) * locals.var_dphi_sb) - (assign93050_e142639 * locals.var_dphi_sb_dn5)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn6 / locals.var_t1) * locals.var_dphi_sb) - (assign93050_e142639 * locals.var_dphi_sb_dn6)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn7 / locals.var_t1) * locals.var_dphi_sb) - (assign93050_e142639 * locals.var_dphi_sb_dn7)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn8 / locals.var_t1) * locals.var_dphi_sb) - (assign93050_e142639 * locals.var_dphi_sb_dn8)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn9 / locals.var_t1) * locals.var_dphi_sb) - (assign93050_e142639 * locals.var_dphi_sb_dn9)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn10 / locals.var_t1) * locals.var_dphi_sb) - (assign93050_e142639 * locals.var_dphi_sb_dn10)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn11 / locals.var_t1) * locals.var_dphi_sb) - (assign93050_e142639 * locals.var_dphi_sb_dn11)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn14 / locals.var_t1) * locals.var_dphi_sb) - (assign93050_e142639 * locals.var_dphi_sb_dn14)) / (locals.var_dphi_sb * locals.var_dphi_sb)),)
    } else {
        (locals.var_c_sb, locals.var_c_sb_dn0, locals.var_c_sb_dn2, locals.var_c_sb_dn4, locals.var_c_sb_dn5, locals.var_c_sb_dn6, locals.var_c_sb_dn7, locals.var_c_sb_dn8, locals.var_c_sb_dn9, locals.var_c_sb_dn10, locals.var_c_sb_dn11, locals.var_c_sb_dn14,)
    }
};
        locals.var_c_sb = assign93050_e142643;
        locals.var_c_sb_dn0 = assign93050_e142643_d_n0;
        locals.var_c_sb_dn2 = assign93050_e142643_d_n2;
        locals.var_c_sb_dn4 = assign93050_e142643_d_n4;
        locals.var_c_sb_dn5 = assign93050_e142643_d_n5;
        locals.var_c_sb_dn6 = assign93050_e142643_d_n6;
        locals.var_c_sb_dn7 = assign93050_e142643_d_n7;
        locals.var_c_sb_dn8 = assign93050_e142643_d_n8;
        locals.var_c_sb_dn9 = assign93050_e142643_d_n9;
        locals.var_c_sb_dn10 = assign93050_e142643_d_n10;
        locals.var_c_sb_dn11 = assign93050_e142643_d_n11;
        locals.var_c_sb_dn14 = assign93050_e142643_d_n14;
        locals.var_c_sb_rv = 0.0;

        let (assign93060_e142656, assign93060_e142656_d_n0, assign93060_e142656_d_n2, assign93060_e142656_d_n4, assign93060_e142656_d_n5, assign93060_e142656_d_n6, assign93060_e142656_d_n7, assign93060_e142656_d_n8, assign93060_e142656_d_n9, assign93060_e142656_d_n10, assign93060_e142656_d_n11, assign93060_e142656_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2154 != 0.0)) {
        let assign93060_e142654: f64 = (locals.var_ps0ld + locals.var_vxbgmtcl);
        (assign93060_e142654, (locals.var_ps0ld_dn0 + locals.var_vxbgmtcl_dn0), (locals.var_ps0ld_dn2 + locals.var_vxbgmtcl_dn2), (locals.var_ps0ld_dn4 + locals.var_vxbgmtcl_dn4), (locals.var_ps0ld_dn5 + locals.var_vxbgmtcl_dn5), (locals.var_ps0ld_dn6 + locals.var_vxbgmtcl_dn6), (locals.var_ps0ld_dn7 + locals.var_vxbgmtcl_dn7), (locals.var_ps0ld_dn8 + locals.var_vxbgmtcl_dn8), (locals.var_ps0ld_dn9 + locals.var_vxbgmtcl_dn9), (locals.var_ps0ld_dn10 + locals.var_vxbgmtcl_dn10), (locals.var_ps0ld_dn11 + locals.var_vxbgmtcl_dn11), (locals.var_ps0ld_dn14 + locals.var_vxbgmtcl_dn14),)
    } else {
        (locals.var_ps0ld_vxb, locals.var_ps0ld_vxb_dn0, locals.var_ps0ld_vxb_dn2, locals.var_ps0ld_vxb_dn4, locals.var_ps0ld_vxb_dn5, locals.var_ps0ld_vxb_dn6, locals.var_ps0ld_vxb_dn7, locals.var_ps0ld_vxb_dn8, locals.var_ps0ld_vxb_dn9, locals.var_ps0ld_vxb_dn10, locals.var_ps0ld_vxb_dn11, locals.var_ps0ld_vxb_dn14,)
    }
};
        locals.var_ps0ld_vxb = assign93060_e142656;
        locals.var_ps0ld_vxb_dn0 = assign93060_e142656_d_n0;
        locals.var_ps0ld_vxb_dn2 = assign93060_e142656_d_n2;
        locals.var_ps0ld_vxb_dn4 = assign93060_e142656_d_n4;
        locals.var_ps0ld_vxb_dn5 = assign93060_e142656_d_n5;
        locals.var_ps0ld_vxb_dn6 = assign93060_e142656_d_n6;
        locals.var_ps0ld_vxb_dn7 = assign93060_e142656_d_n7;
        locals.var_ps0ld_vxb_dn8 = assign93060_e142656_d_n8;
        locals.var_ps0ld_vxb_dn9 = assign93060_e142656_d_n9;
        locals.var_ps0ld_vxb_dn10 = assign93060_e142656_d_n10;
        locals.var_ps0ld_vxb_dn11 = assign93060_e142656_d_n11;
        locals.var_ps0ld_vxb_dn14 = assign93060_e142656_d_n14;
        locals.var_ps0ld_vxb_rv = 0.0;

        let (assign93070_e142671, assign93070_e142671_d_n0, assign93070_e142671_d_n2, assign93070_e142671_d_n4, assign93070_e142671_d_n5, assign93070_e142671_d_n6, assign93070_e142671_d_n7, assign93070_e142671_d_n8, assign93070_e142671_d_n9, assign93070_e142671_d_n10, assign93070_e142671_d_n11, assign93070_e142671_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2154 != 0.0)) {
        let assign93070_e142668: f64 = (locals.var_ps0ld_vxb - locals.var_dphi_sb);
        let assign93070_e142669: f64 = (locals.var_c_sb * assign93070_e142668);
        (assign93070_e142669, ((locals.var_c_sb_dn0 * assign93070_e142668) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn0 - locals.var_dphi_sb_dn0))), ((locals.var_c_sb_dn2 * assign93070_e142668) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn2 - locals.var_dphi_sb_dn2))), ((locals.var_c_sb_dn4 * assign93070_e142668) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn4 - locals.var_dphi_sb_dn4))), ((locals.var_c_sb_dn5 * assign93070_e142668) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn5 - locals.var_dphi_sb_dn5))), ((locals.var_c_sb_dn6 * assign93070_e142668) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn6 - locals.var_dphi_sb_dn6))), ((locals.var_c_sb_dn7 * assign93070_e142668) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn7 - locals.var_dphi_sb_dn7))), ((locals.var_c_sb_dn8 * assign93070_e142668) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn8 - locals.var_dphi_sb_dn8))), ((locals.var_c_sb_dn9 * assign93070_e142668) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn9 - locals.var_dphi_sb_dn9))), ((locals.var_c_sb_dn10 * assign93070_e142668) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn10 - locals.var_dphi_sb_dn10))), ((locals.var_c_sb_dn11 * assign93070_e142668) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn11 - locals.var_dphi_sb_dn11))), ((locals.var_c_sb_dn14 * assign93070_e142668) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn14 - locals.var_dphi_sb_dn14))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn14,)
    }
};
        locals.var_ty = assign93070_e142671;
        locals.var_ty_dn0 = assign93070_e142671_d_n0;
        locals.var_ty_dn2 = assign93070_e142671_d_n2;
        locals.var_ty_dn4 = assign93070_e142671_d_n4;
        locals.var_ty_dn5 = assign93070_e142671_d_n5;
        locals.var_ty_dn6 = assign93070_e142671_d_n6;
        locals.var_ty_dn7 = assign93070_e142671_d_n7;
        locals.var_ty_dn8 = assign93070_e142671_d_n8;
        locals.var_ty_dn9 = assign93070_e142671_d_n9;
        locals.var_ty_dn10 = assign93070_e142671_d_n10;
        locals.var_ty_dn11 = assign93070_e142671_d_n11;
        locals.var_ty_dn14 = assign93070_e142671_d_n14;
        locals.var_ty_rv = 0.0;

        let assign93080_e142674: f64 = if locals.var_ty < 60.0 { 1.0 } else { 0.0 };
        locals.var_guard2158 = assign93080_e142674;
        locals.var_guard2158_rv = 0.0;

        let (assign93090_e142688, assign93090_e142688_d_n0, assign93090_e142688_d_n2, assign93090_e142688_d_n4, assign93090_e142688_d_n5, assign93090_e142688_d_n6, assign93090_e142688_d_n7, assign93090_e142688_d_n8, assign93090_e142688_d_n9, assign93090_e142688_d_n10, assign93090_e142688_d_n11, assign93090_e142688_d_n14,) = {
    if (((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2154 != 0.0)) && (locals.var_guard2158 != 0.0)) {
        let assign93090_e142686: f64 = (locals.var_ty).exp();
        (assign93090_e142686, (assign93090_e142686 * locals.var_ty_dn0), (assign93090_e142686 * locals.var_ty_dn2), (assign93090_e142686 * locals.var_ty_dn4), (assign93090_e142686 * locals.var_ty_dn5), (assign93090_e142686 * locals.var_ty_dn6), (assign93090_e142686 * locals.var_ty_dn7), (assign93090_e142686 * locals.var_ty_dn8), (assign93090_e142686 * locals.var_ty_dn9), (assign93090_e142686 * locals.var_ty_dn10), (assign93090_e142686 * locals.var_ty_dn11), (assign93090_e142686 * locals.var_ty_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign93090_e142688;
        locals.var_t1_dn0 = assign93090_e142688_d_n0;
        locals.var_t1_dn2 = assign93090_e142688_d_n2;
        locals.var_t1_dn4 = assign93090_e142688_d_n4;
        locals.var_t1_dn5 = assign93090_e142688_d_n5;
        locals.var_t1_dn6 = assign93090_e142688_d_n6;
        locals.var_t1_dn7 = assign93090_e142688_d_n7;
        locals.var_t1_dn8 = assign93090_e142688_d_n8;
        locals.var_t1_dn9 = assign93090_e142688_d_n9;
        locals.var_t1_dn10 = assign93090_e142688_d_n10;
        locals.var_t1_dn11 = assign93090_e142688_d_n11;
        locals.var_t1_dn14 = assign93090_e142688_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign93100_e142705, assign93100_e142705_d_n0, assign93100_e142705_d_n2, assign93100_e142705_d_n4, assign93100_e142705_d_n5, assign93100_e142705_d_n6, assign93100_e142705_d_n7, assign93100_e142705_d_n8, assign93100_e142705_d_n9, assign93100_e142705_d_n10, assign93100_e142705_d_n11, assign93100_e142705_d_n14,) = {
    if (((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2154 != 0.0)) && (locals.var_guard2158 != 0.0)) {
        let assign93100_e142700: f64 = (-locals.var_c_sb);
        let assign93100_e142702: f64 = (assign93100_e142700 * locals.var_dphi_sb);
        let assign93100_e142703: f64 = (assign93100_e142702).exp();
        (assign93100_e142703, (assign93100_e142703 * (((-locals.var_c_sb_dn0) * locals.var_dphi_sb) + (assign93100_e142700 * locals.var_dphi_sb_dn0))), (assign93100_e142703 * (((-locals.var_c_sb_dn2) * locals.var_dphi_sb) + (assign93100_e142700 * locals.var_dphi_sb_dn2))), (assign93100_e142703 * (((-locals.var_c_sb_dn4) * locals.var_dphi_sb) + (assign93100_e142700 * locals.var_dphi_sb_dn4))), (assign93100_e142703 * (((-locals.var_c_sb_dn5) * locals.var_dphi_sb) + (assign93100_e142700 * locals.var_dphi_sb_dn5))), (assign93100_e142703 * (((-locals.var_c_sb_dn6) * locals.var_dphi_sb) + (assign93100_e142700 * locals.var_dphi_sb_dn6))), (assign93100_e142703 * (((-locals.var_c_sb_dn7) * locals.var_dphi_sb) + (assign93100_e142700 * locals.var_dphi_sb_dn7))), (assign93100_e142703 * (((-locals.var_c_sb_dn8) * locals.var_dphi_sb) + (assign93100_e142700 * locals.var_dphi_sb_dn8))), (assign93100_e142703 * (((-locals.var_c_sb_dn9) * locals.var_dphi_sb) + (assign93100_e142700 * locals.var_dphi_sb_dn9))), (assign93100_e142703 * (((-locals.var_c_sb_dn10) * locals.var_dphi_sb) + (assign93100_e142700 * locals.var_dphi_sb_dn10))), (assign93100_e142703 * (((-locals.var_c_sb_dn11) * locals.var_dphi_sb) + (assign93100_e142700 * locals.var_dphi_sb_dn11))), (assign93100_e142703 * (((-locals.var_c_sb_dn14) * locals.var_dphi_sb) + (assign93100_e142700 * locals.var_dphi_sb_dn14))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign93100_e142705;
        locals.var_t0_dn0 = assign93100_e142705_d_n0;
        locals.var_t0_dn2 = assign93100_e142705_d_n2;
        locals.var_t0_dn4 = assign93100_e142705_d_n4;
        locals.var_t0_dn5 = assign93100_e142705_d_n5;
        locals.var_t0_dn6 = assign93100_e142705_d_n6;
        locals.var_t0_dn7 = assign93100_e142705_d_n7;
        locals.var_t0_dn8 = assign93100_e142705_d_n8;
        locals.var_t0_dn9 = assign93100_e142705_d_n9;
        locals.var_t0_dn10 = assign93100_e142705_d_n10;
        locals.var_t0_dn11 = assign93100_e142705_d_n11;
        locals.var_t0_dn14 = assign93100_e142705_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign93110_e142720, assign93110_e142720_d_n0, assign93110_e142720_d_n2, assign93110_e142720_d_n4, assign93110_e142720_d_n5, assign93110_e142720_d_n6, assign93110_e142720_d_n7, assign93110_e142720_d_n8, assign93110_e142720_d_n9, assign93110_e142720_d_n10, assign93110_e142720_d_n11, assign93110_e142720_d_n14,) = {
    if (((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2154 != 0.0)) && (locals.var_guard2158 != 0.0)) {
        let assign93110_e142718: f64 = (locals.var_t1 - locals.var_t0);
        (assign93110_e142718, (locals.var_t1_dn0 - locals.var_t0_dn0), (locals.var_t1_dn2 - locals.var_t0_dn2), (locals.var_t1_dn4 - locals.var_t0_dn4), (locals.var_t1_dn5 - locals.var_t0_dn5), (locals.var_t1_dn6 - locals.var_t0_dn6), (locals.var_t1_dn7 - locals.var_t0_dn7), (locals.var_t1_dn8 - locals.var_t0_dn8), (locals.var_t1_dn9 - locals.var_t0_dn9), (locals.var_t1_dn10 - locals.var_t0_dn10), (locals.var_t1_dn11 - locals.var_t0_dn11), (locals.var_t1_dn14 - locals.var_t0_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign93110_e142720;
        locals.var_t2_dn0 = assign93110_e142720_d_n0;
        locals.var_t2_dn2 = assign93110_e142720_d_n2;
        locals.var_t2_dn4 = assign93110_e142720_d_n4;
        locals.var_t2_dn5 = assign93110_e142720_d_n5;
        locals.var_t2_dn6 = assign93110_e142720_d_n6;
        locals.var_t2_dn7 = assign93110_e142720_d_n7;
        locals.var_t2_dn8 = assign93110_e142720_d_n8;
        locals.var_t2_dn9 = assign93110_e142720_d_n9;
        locals.var_t2_dn10 = assign93110_e142720_d_n10;
        locals.var_t2_dn11 = assign93110_e142720_d_n11;
        locals.var_t2_dn14 = assign93110_e142720_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign93120_e142738, assign93120_e142738_d_n0, assign93120_e142738_d_n2, assign93120_e142738_d_n4, assign93120_e142738_d_n5, assign93120_e142738_d_n6, assign93120_e142738_d_n7, assign93120_e142738_d_n8, assign93120_e142738_d_n9, assign93120_e142738_d_n10, assign93120_e142738_d_n11, assign93120_e142738_d_n14,) = {
    if (((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2154 != 0.0)) && (locals.var_guard2158 != 0.0)) {
        let assign93120_e142733: f64 = (1.0 + locals.var_t2);
        let assign93120_e142734: f64 = (assign93120_e142733).ln();
        let assign93120_e142736: f64 = (assign93120_e142734 / locals.var_c_sb);
        (assign93120_e142736, ((((locals.var_t2_dn0 / assign93120_e142733) * locals.var_c_sb) - (assign93120_e142734 * locals.var_c_sb_dn0)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn2 / assign93120_e142733) * locals.var_c_sb) - (assign93120_e142734 * locals.var_c_sb_dn2)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn4 / assign93120_e142733) * locals.var_c_sb) - (assign93120_e142734 * locals.var_c_sb_dn4)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn5 / assign93120_e142733) * locals.var_c_sb) - (assign93120_e142734 * locals.var_c_sb_dn5)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn6 / assign93120_e142733) * locals.var_c_sb) - (assign93120_e142734 * locals.var_c_sb_dn6)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn7 / assign93120_e142733) * locals.var_c_sb) - (assign93120_e142734 * locals.var_c_sb_dn7)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn8 / assign93120_e142733) * locals.var_c_sb) - (assign93120_e142734 * locals.var_c_sb_dn8)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn9 / assign93120_e142733) * locals.var_c_sb) - (assign93120_e142734 * locals.var_c_sb_dn9)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn10 / assign93120_e142733) * locals.var_c_sb) - (assign93120_e142734 * locals.var_c_sb_dn10)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn11 / assign93120_e142733) * locals.var_c_sb) - (assign93120_e142734 * locals.var_c_sb_dn11)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn14 / assign93120_e142733) * locals.var_c_sb) - (assign93120_e142734 * locals.var_c_sb_dn14)) / (locals.var_c_sb * locals.var_c_sb)),)
    } else {
        (locals.var_phi_b, locals.var_phi_b_dn0, locals.var_phi_b_dn2, locals.var_phi_b_dn4, locals.var_phi_b_dn5, locals.var_phi_b_dn6, locals.var_phi_b_dn7, locals.var_phi_b_dn8, locals.var_phi_b_dn9, locals.var_phi_b_dn10, locals.var_phi_b_dn11, locals.var_phi_b_dn14,)
    }
};
        locals.var_phi_b = assign93120_e142738;
        locals.var_phi_b_dn0 = assign93120_e142738_d_n0;
        locals.var_phi_b_dn2 = assign93120_e142738_d_n2;
        locals.var_phi_b_dn4 = assign93120_e142738_d_n4;
        locals.var_phi_b_dn5 = assign93120_e142738_d_n5;
        locals.var_phi_b_dn6 = assign93120_e142738_d_n6;
        locals.var_phi_b_dn7 = assign93120_e142738_d_n7;
        locals.var_phi_b_dn8 = assign93120_e142738_d_n8;
        locals.var_phi_b_dn9 = assign93120_e142738_d_n9;
        locals.var_phi_b_dn10 = assign93120_e142738_d_n10;
        locals.var_phi_b_dn11 = assign93120_e142738_d_n11;
        locals.var_phi_b_dn14 = assign93120_e142738_d_n14;
        locals.var_phi_b_rv = 0.0;

        let (assign93130_e142754, assign93130_e142754_d_n0, assign93130_e142754_d_n2, assign93130_e142754_d_n4, assign93130_e142754_d_n5, assign93130_e142754_d_n6, assign93130_e142754_d_n7, assign93130_e142754_d_n8, assign93130_e142754_d_n9, assign93130_e142754_d_n10, assign93130_e142754_d_n11, assign93130_e142754_d_n14,) = {
    if (((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2154 != 0.0)) && (locals.var_guard2158 == 0.0)) {
        let assign93130_e142752: f64 = (locals.var_ps0ld_vxb - locals.var_dphi_sb);
        (assign93130_e142752, (locals.var_ps0ld_vxb_dn0 - locals.var_dphi_sb_dn0), (locals.var_ps0ld_vxb_dn2 - locals.var_dphi_sb_dn2), (locals.var_ps0ld_vxb_dn4 - locals.var_dphi_sb_dn4), (locals.var_ps0ld_vxb_dn5 - locals.var_dphi_sb_dn5), (locals.var_ps0ld_vxb_dn6 - locals.var_dphi_sb_dn6), (locals.var_ps0ld_vxb_dn7 - locals.var_dphi_sb_dn7), (locals.var_ps0ld_vxb_dn8 - locals.var_dphi_sb_dn8), (locals.var_ps0ld_vxb_dn9 - locals.var_dphi_sb_dn9), (locals.var_ps0ld_vxb_dn10 - locals.var_dphi_sb_dn10), (locals.var_ps0ld_vxb_dn11 - locals.var_dphi_sb_dn11), (locals.var_ps0ld_vxb_dn14 - locals.var_dphi_sb_dn14),)
    } else {
        (locals.var_phi_b, locals.var_phi_b_dn0, locals.var_phi_b_dn2, locals.var_phi_b_dn4, locals.var_phi_b_dn5, locals.var_phi_b_dn6, locals.var_phi_b_dn7, locals.var_phi_b_dn8, locals.var_phi_b_dn9, locals.var_phi_b_dn10, locals.var_phi_b_dn11, locals.var_phi_b_dn14,)
    }
};
        locals.var_phi_b = assign93130_e142754;
        locals.var_phi_b_dn0 = assign93130_e142754_d_n0;
        locals.var_phi_b_dn2 = assign93130_e142754_d_n2;
        locals.var_phi_b_dn4 = assign93130_e142754_d_n4;
        locals.var_phi_b_dn5 = assign93130_e142754_d_n5;
        locals.var_phi_b_dn6 = assign93130_e142754_d_n6;
        locals.var_phi_b_dn7 = assign93130_e142754_d_n7;
        locals.var_phi_b_dn8 = assign93130_e142754_d_n8;
        locals.var_phi_b_dn9 = assign93130_e142754_d_n9;
        locals.var_phi_b_dn10 = assign93130_e142754_d_n10;
        locals.var_phi_b_dn11 = assign93130_e142754_d_n11;
        locals.var_phi_b_dn14 = assign93130_e142754_d_n14;
        locals.var_phi_b_rv = 0.0;

        let (assign93140_e142767, assign93140_e142767_d_n0, assign93140_e142767_d_n2, assign93140_e142767_d_n4, assign93140_e142767_d_n5, assign93140_e142767_d_n6, assign93140_e142767_d_n7, assign93140_e142767_d_n8, assign93140_e142767_d_n9, assign93140_e142767_d_n10, assign93140_e142767_d_n11, assign93140_e142767_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2154 != 0.0)) {
        let assign93140_e142765: f64 = (locals.var_beta * locals.var_phi_b);
        (assign93140_e142765, ((locals.var_beta_dn0 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn0)), ((locals.var_beta_dn2 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn2)), ((locals.var_beta_dn4 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn4)), ((locals.var_beta_dn5 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn5)), ((locals.var_beta_dn6 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn6)), ((locals.var_beta_dn7 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn7)), ((locals.var_beta_dn8 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn8)), ((locals.var_beta_dn9 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn9)), ((locals.var_beta_dn10 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn10)), ((locals.var_beta_dn11 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn11)), ((locals.var_beta_dn14 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn14)),)
    } else {
        (locals.var_chib, locals.var_chib_dn0, locals.var_chib_dn2, locals.var_chib_dn4, locals.var_chib_dn5, locals.var_chib_dn6, locals.var_chib_dn7, locals.var_chib_dn8, locals.var_chib_dn9, locals.var_chib_dn10, locals.var_chib_dn11, locals.var_chib_dn14,)
    }
};
        locals.var_chib = assign93140_e142767;
        locals.var_chib_dn0 = assign93140_e142767_d_n0;
        locals.var_chib_dn2 = assign93140_e142767_d_n2;
        locals.var_chib_dn4 = assign93140_e142767_d_n4;
        locals.var_chib_dn5 = assign93140_e142767_d_n5;
        locals.var_chib_dn6 = assign93140_e142767_d_n6;
        locals.var_chib_dn7 = assign93140_e142767_d_n7;
        locals.var_chib_dn8 = assign93140_e142767_d_n8;
        locals.var_chib_dn9 = assign93140_e142767_d_n9;
        locals.var_chib_dn10 = assign93140_e142767_d_n10;
        locals.var_chib_dn11 = assign93140_e142767_d_n11;
        locals.var_chib_dn14 = assign93140_e142767_d_n14;
        locals.var_chib_rv = 0.0;

        let assign93150_e142771: f64 = (locals.var_chi / 100.0);
        let assign93150_e142776: f64 = if ((locals.var_chib > assign93150_e142771) && (locals.var_chib > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2159 = assign93150_e142776;
        locals.var_guard2159_rv = 0.0;

        let (assign93160_e142791,) = {
    if (((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2154 != 0.0)) && (locals.var_guard2159 != 0.0)) {
        let assign93160_e142789: f64 = (locals.var_flg_fd_mode__blk2123 + 1.0);
        (assign93160_e142789,)
    } else {
        (locals.var_flg_fd_mode__blk2123,)
    }
};
        locals.var_flg_fd_mode__blk2123 = assign93160_e142791;
        locals.var_flg_fd_mode__blk2123_rv = 0.0;

        let (assign93170_e142804, assign93170_e142804_d_n0, assign93170_e142804_d_n2, assign93170_e142804_d_n4, assign93170_e142804_d_n5, assign93170_e142804_d_n6, assign93170_e142804_d_n7, assign93170_e142804_d_n8, assign93170_e142804_d_n9, assign93170_e142804_d_n10, assign93170_e142804_d_n11, assign93170_e142804_d_n14,) = {
    if (((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2154 != 0.0)) && (locals.var_guard2159 != 0.0)) {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn11, locals.var_chi_1_dn14,)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
        locals.var_chi = assign93170_e142804;
        locals.var_chi_dn0 = assign93170_e142804_d_n0;
        locals.var_chi_dn2 = assign93170_e142804_d_n2;
        locals.var_chi_dn4 = assign93170_e142804_d_n4;
        locals.var_chi_dn5 = assign93170_e142804_d_n5;
        locals.var_chi_dn6 = assign93170_e142804_d_n6;
        locals.var_chi_dn7 = assign93170_e142804_d_n7;
        locals.var_chi_dn8 = assign93170_e142804_d_n8;
        locals.var_chi_dn9 = assign93170_e142804_d_n9;
        locals.var_chi_dn10 = assign93170_e142804_d_n10;
        locals.var_chi_dn11 = assign93170_e142804_d_n11;
        locals.var_chi_dn14 = assign93170_e142804_d_n14;
        locals.var_chi_rv = 0.0;

        let (assign93180_e142817, assign93180_e142817_d_n0, assign93180_e142817_d_n2, assign93180_e142817_d_n4, assign93180_e142817_d_n5, assign93180_e142817_d_n6, assign93180_e142817_d_n7, assign93180_e142817_d_n8, assign93180_e142817_d_n9, assign93180_e142817_d_n10, assign93180_e142817_d_n11, assign93180_e142817_d_n14,) = {
    if (((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) {
        let assign93180_e142813: f64 = (locals.var_chi * locals.var_beta_inv);
        let assign93180_e142815: f64 = (assign93180_e142813 - locals.var_vxbgmtcl);
        (assign93180_e142815, (((locals.var_chi_dn0 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn0)) - locals.var_vxbgmtcl_dn0), (((locals.var_chi_dn2 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn2)) - locals.var_vxbgmtcl_dn2), (((locals.var_chi_dn4 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn4)) - locals.var_vxbgmtcl_dn4), (((locals.var_chi_dn5 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn5)) - locals.var_vxbgmtcl_dn5), (((locals.var_chi_dn6 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn6)) - locals.var_vxbgmtcl_dn6), (((locals.var_chi_dn7 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn7)) - locals.var_vxbgmtcl_dn7), (((locals.var_chi_dn8 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn8)) - locals.var_vxbgmtcl_dn8), (((locals.var_chi_dn9 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn9)) - locals.var_vxbgmtcl_dn9), (((locals.var_chi_dn10 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn10)) - locals.var_vxbgmtcl_dn10), (((locals.var_chi_dn11 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn11)) - locals.var_vxbgmtcl_dn11), (((locals.var_chi_dn14 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn14)) - locals.var_vxbgmtcl_dn14),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn14,)
    }
};
        locals.var_ps0ld = assign93180_e142817;
        locals.var_ps0ld_dn0 = assign93180_e142817_d_n0;
        locals.var_ps0ld_dn2 = assign93180_e142817_d_n2;
        locals.var_ps0ld_dn4 = assign93180_e142817_d_n4;
        locals.var_ps0ld_dn5 = assign93180_e142817_d_n5;
        locals.var_ps0ld_dn6 = assign93180_e142817_d_n6;
        locals.var_ps0ld_dn7 = assign93180_e142817_d_n7;
        locals.var_ps0ld_dn8 = assign93180_e142817_d_n8;
        locals.var_ps0ld_dn9 = assign93180_e142817_d_n9;
        locals.var_ps0ld_dn10 = assign93180_e142817_d_n10;
        locals.var_ps0ld_dn11 = assign93180_e142817_d_n11;
        locals.var_ps0ld_dn14 = assign93180_e142817_d_n14;
        locals.var_ps0ld_rv = 0.0;

        let assign93190_e142819: f64 = (locals.var_chi).abs();
        let assign93190_e142821: f64 = if assign93190_e142819 > 1e-6 { 1.0 } else { 0.0 };
        locals.var_guard2160 = assign93190_e142821;
        locals.var_guard2160_rv = 0.0;

        let (assign93200_e142838, assign93200_e142838_d_n0, assign93200_e142838_d_n2, assign93200_e142838_d_n4, assign93200_e142838_d_n5, assign93200_e142838_d_n6, assign93200_e142838_d_n7, assign93200_e142838_d_n8, assign93200_e142838_d_n9, assign93200_e142838_d_n10, assign93200_e142838_d_n11, assign93200_e142838_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2160 != 0.0)) {
        let assign93200_e142832: f64 = (locals.var_chi - 1.0);
        let assign93200_e142834: f64 = (-locals.var_chi);
        let assign93200_e142835: f64 = (assign93200_e142834).exp();
        let assign93200_e142836: f64 = (assign93200_e142832 + assign93200_e142835);
        (assign93200_e142836, (locals.var_chi_dn0 + (assign93200_e142835 * (-locals.var_chi_dn0))), (locals.var_chi_dn2 + (assign93200_e142835 * (-locals.var_chi_dn2))), (locals.var_chi_dn4 + (assign93200_e142835 * (-locals.var_chi_dn4))), (locals.var_chi_dn5 + (assign93200_e142835 * (-locals.var_chi_dn5))), (locals.var_chi_dn6 + (assign93200_e142835 * (-locals.var_chi_dn6))), (locals.var_chi_dn7 + (assign93200_e142835 * (-locals.var_chi_dn7))), (locals.var_chi_dn8 + (assign93200_e142835 * (-locals.var_chi_dn8))), (locals.var_chi_dn9 + (assign93200_e142835 * (-locals.var_chi_dn9))), (locals.var_chi_dn10 + (assign93200_e142835 * (-locals.var_chi_dn10))), (locals.var_chi_dn11 + (assign93200_e142835 * (-locals.var_chi_dn11))), (locals.var_chi_dn14 + (assign93200_e142835 * (-locals.var_chi_dn14))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign93200_e142838;
        locals.var_t1_dn0 = assign93200_e142838_d_n0;
        locals.var_t1_dn2 = assign93200_e142838_d_n2;
        locals.var_t1_dn4 = assign93200_e142838_d_n4;
        locals.var_t1_dn5 = assign93200_e142838_d_n5;
        locals.var_t1_dn6 = assign93200_e142838_d_n6;
        locals.var_t1_dn7 = assign93200_e142838_d_n7;
        locals.var_t1_dn8 = assign93200_e142838_d_n8;
        locals.var_t1_dn9 = assign93200_e142838_d_n9;
        locals.var_t1_dn10 = assign93200_e142838_d_n10;
        locals.var_t1_dn11 = assign93200_e142838_d_n11;
        locals.var_t1_dn14 = assign93200_e142838_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign93210_e142850, assign93210_e142850_d_n0, assign93210_e142850_d_n2, assign93210_e142850_d_n4, assign93210_e142850_d_n5, assign93210_e142850_d_n6, assign93210_e142850_d_n7, assign93210_e142850_d_n8, assign93210_e142850_d_n9, assign93210_e142850_d_n10, assign93210_e142850_d_n11, assign93210_e142850_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2160 != 0.0)) {
        let assign93210_e142848: f64 = (locals.var_t1).sqrt();
        (assign93210_e142848, (locals.var_t1_dn0 / (2.0 * assign93210_e142848)), (locals.var_t1_dn2 / (2.0 * assign93210_e142848)), (locals.var_t1_dn4 / (2.0 * assign93210_e142848)), (locals.var_t1_dn5 / (2.0 * assign93210_e142848)), (locals.var_t1_dn6 / (2.0 * assign93210_e142848)), (locals.var_t1_dn7 / (2.0 * assign93210_e142848)), (locals.var_t1_dn8 / (2.0 * assign93210_e142848)), (locals.var_t1_dn9 / (2.0 * assign93210_e142848)), (locals.var_t1_dn10 / (2.0 * assign93210_e142848)), (locals.var_t1_dn11 / (2.0 * assign93210_e142848)), (locals.var_t1_dn14 / (2.0 * assign93210_e142848)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign93210_e142850;
        locals.var_t2_dn0 = assign93210_e142850_d_n0;
        locals.var_t2_dn2 = assign93210_e142850_d_n2;
        locals.var_t2_dn4 = assign93210_e142850_d_n4;
        locals.var_t2_dn5 = assign93210_e142850_d_n5;
        locals.var_t2_dn6 = assign93210_e142850_d_n6;
        locals.var_t2_dn7 = assign93210_e142850_d_n7;
        locals.var_t2_dn8 = assign93210_e142850_d_n8;
        locals.var_t2_dn9 = assign93210_e142850_d_n9;
        locals.var_t2_dn10 = assign93210_e142850_d_n10;
        locals.var_t2_dn11 = assign93210_e142850_d_n11;
        locals.var_t2_dn14 = assign93210_e142850_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign93230_e142885, assign93230_e142885_d_n0, assign93230_e142885_d_n2, assign93230_e142885_d_n4, assign93230_e142885_d_n5, assign93230_e142885_d_n6, assign93230_e142885_d_n7, assign93230_e142885_d_n8, assign93230_e142885_d_n9, assign93230_e142885_d_n10, assign93230_e142885_d_n11, assign93230_e142885_d_n14,) = {
    if ((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2160 == 0.0)) {
        let assign93230_e142876: f64 = (0.7071067811865475 * locals.var_chi);
        let assign93230_e142880: f64 = (locals.var_chi * 0.3333333333333333);
        let assign93230_e142881: f64 = (1.0 - assign93230_e142880);
        let assign93230_e142882: f64 = (assign93230_e142881).sqrt();
        let assign93230_e142883: f64 = (assign93230_e142876 * assign93230_e142882);
        (assign93230_e142883, (((0.7071067811865475 * locals.var_chi_dn0) * assign93230_e142882) + (assign93230_e142876 * ((-(locals.var_chi_dn0 * 0.3333333333333333)) / (2.0 * assign93230_e142882)))), (((0.7071067811865475 * locals.var_chi_dn2) * assign93230_e142882) + (assign93230_e142876 * ((-(locals.var_chi_dn2 * 0.3333333333333333)) / (2.0 * assign93230_e142882)))), (((0.7071067811865475 * locals.var_chi_dn4) * assign93230_e142882) + (assign93230_e142876 * ((-(locals.var_chi_dn4 * 0.3333333333333333)) / (2.0 * assign93230_e142882)))), (((0.7071067811865475 * locals.var_chi_dn5) * assign93230_e142882) + (assign93230_e142876 * ((-(locals.var_chi_dn5 * 0.3333333333333333)) / (2.0 * assign93230_e142882)))), (((0.7071067811865475 * locals.var_chi_dn6) * assign93230_e142882) + (assign93230_e142876 * ((-(locals.var_chi_dn6 * 0.3333333333333333)) / (2.0 * assign93230_e142882)))), (((0.7071067811865475 * locals.var_chi_dn7) * assign93230_e142882) + (assign93230_e142876 * ((-(locals.var_chi_dn7 * 0.3333333333333333)) / (2.0 * assign93230_e142882)))), (((0.7071067811865475 * locals.var_chi_dn8) * assign93230_e142882) + (assign93230_e142876 * ((-(locals.var_chi_dn8 * 0.3333333333333333)) / (2.0 * assign93230_e142882)))), (((0.7071067811865475 * locals.var_chi_dn9) * assign93230_e142882) + (assign93230_e142876 * ((-(locals.var_chi_dn9 * 0.3333333333333333)) / (2.0 * assign93230_e142882)))), (((0.7071067811865475 * locals.var_chi_dn10) * assign93230_e142882) + (assign93230_e142876 * ((-(locals.var_chi_dn10 * 0.3333333333333333)) / (2.0 * assign93230_e142882)))), (((0.7071067811865475 * locals.var_chi_dn11) * assign93230_e142882) + (assign93230_e142876 * ((-(locals.var_chi_dn11 * 0.3333333333333333)) / (2.0 * assign93230_e142882)))), (((0.7071067811865475 * locals.var_chi_dn14) * assign93230_e142882) + (assign93230_e142876 * ((-(locals.var_chi_dn14 * 0.3333333333333333)) / (2.0 * assign93230_e142882)))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign93230_e142885;
        locals.var_t2_dn0 = assign93230_e142885_d_n0;
        locals.var_t2_dn2 = assign93230_e142885_d_n2;
        locals.var_t2_dn4 = assign93230_e142885_d_n4;
        locals.var_t2_dn5 = assign93230_e142885_d_n5;
        locals.var_t2_dn6 = assign93230_e142885_d_n6;
        locals.var_t2_dn7 = assign93230_e142885_d_n7;
        locals.var_t2_dn8 = assign93230_e142885_d_n8;
        locals.var_t2_dn9 = assign93230_e142885_d_n9;
        locals.var_t2_dn10 = assign93230_e142885_d_n10;
        locals.var_t2_dn11 = assign93230_e142885_d_n11;
        locals.var_t2_dn14 = assign93230_e142885_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign93240_e142896, assign93240_e142896_d_n0, assign93240_e142896_d_n2, assign93240_e142896_d_n4, assign93240_e142896_d_n5, assign93240_e142896_d_n6, assign93240_e142896_d_n7, assign93240_e142896_d_n8, assign93240_e142896_d_n9, assign93240_e142896_d_n10, assign93240_e142896_d_n11, assign93240_e142896_d_n14,) = {
    if (((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) {
        let assign93240_e142894: f64 = (locals.var_cnst0over_func * locals.var_t2);
        (assign93240_e142894, ((locals.var_cnst0over_func_dn0 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn0)), ((locals.var_cnst0over_func_dn2 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn2)), ((locals.var_cnst0over_func_dn4 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn4)), ((locals.var_cnst0over_func_dn5 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn5)), ((locals.var_cnst0over_func_dn6 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn6)), ((locals.var_cnst0over_func_dn7 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn7)), ((locals.var_cnst0over_func_dn8 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn8)), ((locals.var_cnst0over_func_dn9 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn9)), ((locals.var_cnst0over_func_dn10 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn10)), ((locals.var_cnst0over_func_dn11 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn11)), ((locals.var_cnst0over_func_dn14 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn14)),)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn8, locals.var_qbuld_dn9, locals.var_qbuld_dn10, locals.var_qbuld_dn11, locals.var_qbuld_dn14,)
    }
};
        locals.var_qbuld = assign93240_e142896;
        locals.var_qbuld_dn0 = assign93240_e142896_d_n0;
        locals.var_qbuld_dn2 = assign93240_e142896_d_n2;
        locals.var_qbuld_dn4 = assign93240_e142896_d_n4;
        locals.var_qbuld_dn5 = assign93240_e142896_d_n5;
        locals.var_qbuld_dn6 = assign93240_e142896_d_n6;
        locals.var_qbuld_dn7 = assign93240_e142896_d_n7;
        locals.var_qbuld_dn8 = assign93240_e142896_d_n8;
        locals.var_qbuld_dn9 = assign93240_e142896_d_n9;
        locals.var_qbuld_dn10 = assign93240_e142896_d_n10;
        locals.var_qbuld_dn11 = assign93240_e142896_d_n11;
        locals.var_qbuld_dn14 = assign93240_e142896_d_n14;
        locals.var_qbuld_rv = 0.0;

        let (assign93250_e142909, assign93250_e142909_d_n0, assign93250_e142909_d_n2, assign93250_e142909_d_n4, assign93250_e142909_d_n5, assign93250_e142909_d_n6, assign93250_e142909_d_n7, assign93250_e142909_d_n8, assign93250_e142909_d_n9, assign93250_e142909_d_n10, assign93250_e142909_d_n11, assign93250_e142909_d_n14,) = {
    if (((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) {
        let assign93250_e142906: f64 = (locals.var_vgpld - locals.var_ps0ld);
        let assign93250_e142907: f64 = (locals.var_cox0_func * assign93250_e142906);
        (assign93250_e142907, (locals.var_cox0_func * (-locals.var_ps0ld_dn0)), (locals.var_cox0_func * (locals.var_vgpld_dn2 - locals.var_ps0ld_dn2)), (locals.var_cox0_func * (-locals.var_ps0ld_dn4)), (locals.var_cox0_func * (-locals.var_ps0ld_dn5)), (locals.var_cox0_func * (-locals.var_ps0ld_dn6)), (locals.var_cox0_func * (locals.var_vgpld_dn7 - locals.var_ps0ld_dn7)), (locals.var_cox0_func * (locals.var_vgpld_dn8 - locals.var_ps0ld_dn8)), (locals.var_cox0_func * (locals.var_vgpld_dn9 - locals.var_ps0ld_dn9)), (locals.var_cox0_func * (-locals.var_ps0ld_dn10)), (locals.var_cox0_func * (-locals.var_ps0ld_dn11)), (locals.var_cox0_func * (-locals.var_ps0ld_dn14)),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn4, locals.var_qsuld_dn5, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn8, locals.var_qsuld_dn9, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn14,)
    }
};
        locals.var_qsuld = assign93250_e142909;
        locals.var_qsuld_dn0 = assign93250_e142909_d_n0;
        locals.var_qsuld_dn2 = assign93250_e142909_d_n2;
        locals.var_qsuld_dn4 = assign93250_e142909_d_n4;
        locals.var_qsuld_dn5 = assign93250_e142909_d_n5;
        locals.var_qsuld_dn6 = assign93250_e142909_d_n6;
        locals.var_qsuld_dn7 = assign93250_e142909_d_n7;
        locals.var_qsuld_dn8 = assign93250_e142909_d_n8;
        locals.var_qsuld_dn9 = assign93250_e142909_d_n9;
        locals.var_qsuld_dn10 = assign93250_e142909_d_n10;
        locals.var_qsuld_dn11 = assign93250_e142909_d_n11;
        locals.var_qsuld_dn14 = assign93250_e142909_d_n14;
        locals.var_qsuld_rv = 0.0;

        let (assign93260_e142920, assign93260_e142920_d_n0, assign93260_e142920_d_n2, assign93260_e142920_d_n4, assign93260_e142920_d_n5, assign93260_e142920_d_n6, assign93260_e142920_d_n7, assign93260_e142920_d_n8, assign93260_e142920_d_n9, assign93260_e142920_d_n10, assign93260_e142920_d_n11, assign93260_e142920_d_n14,) = {
    if (((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) {
        let assign93260_e142918: f64 = (locals.var_qbuld / locals.var_q_nsubld__blk2117);
        (assign93260_e142918, (locals.var_qbuld_dn0 / locals.var_q_nsubld__blk2117), (locals.var_qbuld_dn2 / locals.var_q_nsubld__blk2117), (locals.var_qbuld_dn4 / locals.var_q_nsubld__blk2117), (locals.var_qbuld_dn5 / locals.var_q_nsubld__blk2117), (locals.var_qbuld_dn6 / locals.var_q_nsubld__blk2117), (locals.var_qbuld_dn7 / locals.var_q_nsubld__blk2117), (locals.var_qbuld_dn8 / locals.var_q_nsubld__blk2117), (locals.var_qbuld_dn9 / locals.var_q_nsubld__blk2117), (locals.var_qbuld_dn10 / locals.var_q_nsubld__blk2117), (locals.var_qbuld_dn11 / locals.var_q_nsubld__blk2117), (locals.var_qbuld_dn14 / locals.var_q_nsubld__blk2117),)
    } else {
        (locals.var_wdld0__blk2161, locals.var_wdld0__blk2161_dn0, locals.var_wdld0__blk2161_dn2, locals.var_wdld0__blk2161_dn4, locals.var_wdld0__blk2161_dn5, locals.var_wdld0__blk2161_dn6, locals.var_wdld0__blk2161_dn7, locals.var_wdld0__blk2161_dn8, locals.var_wdld0__blk2161_dn9, locals.var_wdld0__blk2161_dn10, locals.var_wdld0__blk2161_dn11, locals.var_wdld0__blk2161_dn14,)
    }
};
        locals.var_wdld0__blk2161 = assign93260_e142920;
        locals.var_wdld0__blk2161_dn0 = assign93260_e142920_d_n0;
        locals.var_wdld0__blk2161_dn2 = assign93260_e142920_d_n2;
        locals.var_wdld0__blk2161_dn4 = assign93260_e142920_d_n4;
        locals.var_wdld0__blk2161_dn5 = assign93260_e142920_d_n5;
        locals.var_wdld0__blk2161_dn6 = assign93260_e142920_d_n6;
        locals.var_wdld0__blk2161_dn7 = assign93260_e142920_d_n7;
        locals.var_wdld0__blk2161_dn8 = assign93260_e142920_d_n8;
        locals.var_wdld0__blk2161_dn9 = assign93260_e142920_d_n9;
        locals.var_wdld0__blk2161_dn10 = assign93260_e142920_d_n10;
        locals.var_wdld0__blk2161_dn11 = assign93260_e142920_d_n11;
        locals.var_wdld0__blk2161_dn14 = assign93260_e142920_d_n14;
        locals.var_wdld0__blk2161_rv = 0.0;

        let assign93270_e142923: f64 = if p.p33 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard2163 = assign93270_e142923;
        locals.var_guard2163_rv = 0.0;

        let assign93280_e142928: f64 = (locals.var_ddriftldc * 0.1);
        let assign93280_e142929: f64 = (locals.var_ddriftldc - assign93280_e142928);
        let assign93280_e142933: f64 = (locals.var_ddriftldc * 0.1);
        let assign93280_e142936: f64 = if ((locals.var_wdld0__blk2161 > assign93280_e142929) && (assign93280_e142933 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2164 = assign93280_e142936;
        locals.var_guard2164_rv = 0.0;

        let (assign93290_e142955, assign93290_e142955_d_n0, assign93290_e142955_d_n2, assign93290_e142955_d_n4, assign93290_e142955_d_n5, assign93290_e142955_d_n6, assign93290_e142955_d_n7, assign93290_e142955_d_n8, assign93290_e142955_d_n9, assign93290_e142955_d_n10, assign93290_e142955_d_n11, assign93290_e142955_d_n14,) = {
    if (((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2163 != 0.0)) && (locals.var_guard2164 != 0.0)) {
        let assign93290_e142949: f64 = (locals.var_wdld0__blk2161 - locals.var_ddriftldc);
        let assign93290_e142952: f64 = (locals.var_ddriftldc * 0.1);
        let assign93290_e142953: f64 = (assign93290_e142949 + assign93290_e142952);
        (assign93290_e142953, ((locals.var_wdld0__blk2161_dn0 - locals.var_ddriftldc_dn0) + (locals.var_ddriftldc_dn0 * 0.1)), ((locals.var_wdld0__blk2161_dn2 - locals.var_ddriftldc_dn2) + (locals.var_ddriftldc_dn2 * 0.1)), ((locals.var_wdld0__blk2161_dn4 - locals.var_ddriftldc_dn4) + (locals.var_ddriftldc_dn4 * 0.1)), ((locals.var_wdld0__blk2161_dn5 - locals.var_ddriftldc_dn5) + (locals.var_ddriftldc_dn5 * 0.1)), ((locals.var_wdld0__blk2161_dn6 - locals.var_ddriftldc_dn6) + (locals.var_ddriftldc_dn6 * 0.1)), ((locals.var_wdld0__blk2161_dn7 - locals.var_ddriftldc_dn7) + (locals.var_ddriftldc_dn7 * 0.1)), ((locals.var_wdld0__blk2161_dn8 - locals.var_ddriftldc_dn8) + (locals.var_ddriftldc_dn8 * 0.1)), ((locals.var_wdld0__blk2161_dn9 - locals.var_ddriftldc_dn9) + (locals.var_ddriftldc_dn9 * 0.1)), ((locals.var_wdld0__blk2161_dn10 - locals.var_ddriftldc_dn10) + (locals.var_ddriftldc_dn10 * 0.1)), ((locals.var_wdld0__blk2161_dn11 - locals.var_ddriftldc_dn11) + (locals.var_ddriftldc_dn11 * 0.1)), ((locals.var_wdld0__blk2161_dn14 - locals.var_ddriftldc_dn14) + (locals.var_ddriftldc_dn14 * 0.1)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign93290_e142955;
        locals.var_tmf1_dn0 = assign93290_e142955_d_n0;
        locals.var_tmf1_dn2 = assign93290_e142955_d_n2;
        locals.var_tmf1_dn4 = assign93290_e142955_d_n4;
        locals.var_tmf1_dn5 = assign93290_e142955_d_n5;
        locals.var_tmf1_dn6 = assign93290_e142955_d_n6;
        locals.var_tmf1_dn7 = assign93290_e142955_d_n7;
        locals.var_tmf1_dn8 = assign93290_e142955_d_n8;
        locals.var_tmf1_dn9 = assign93290_e142955_d_n9;
        locals.var_tmf1_dn10 = assign93290_e142955_d_n10;
        locals.var_tmf1_dn11 = assign93290_e142955_d_n11;
        locals.var_tmf1_dn14 = assign93290_e142955_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign93300_e142970, assign93300_e142970_d_n0, assign93300_e142970_d_n2, assign93300_e142970_d_n4, assign93300_e142970_d_n5, assign93300_e142970_d_n6, assign93300_e142970_d_n7, assign93300_e142970_d_n8, assign93300_e142970_d_n9, assign93300_e142970_d_n10, assign93300_e142970_d_n11, assign93300_e142970_d_n14,) = {
    if (((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2163 != 0.0)) && (locals.var_guard2164 != 0.0)) {
        let assign93300_e142968: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign93300_e142968, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign93300_e142970;
        locals.var_x2_dn0 = assign93300_e142970_d_n0;
        locals.var_x2_dn2 = assign93300_e142970_d_n2;
        locals.var_x2_dn4 = assign93300_e142970_d_n4;
        locals.var_x2_dn5 = assign93300_e142970_d_n5;
        locals.var_x2_dn6 = assign93300_e142970_d_n6;
        locals.var_x2_dn7 = assign93300_e142970_d_n7;
        locals.var_x2_dn8 = assign93300_e142970_d_n8;
        locals.var_x2_dn9 = assign93300_e142970_d_n9;
        locals.var_x2_dn10 = assign93300_e142970_d_n10;
        locals.var_x2_dn11 = assign93300_e142970_d_n11;
        locals.var_x2_dn14 = assign93300_e142970_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign93310_e142989, assign93310_e142989_d_n0, assign93310_e142989_d_n2, assign93310_e142989_d_n4, assign93310_e142989_d_n5, assign93310_e142989_d_n6, assign93310_e142989_d_n7, assign93310_e142989_d_n8, assign93310_e142989_d_n9, assign93310_e142989_d_n10, assign93310_e142989_d_n11, assign93310_e142989_d_n14,) = {
    if (((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2163 != 0.0)) && (locals.var_guard2164 != 0.0)) {
        let assign93310_e142983: f64 = (locals.var_ddriftldc * 0.1);
        let assign93310_e142986: f64 = (locals.var_ddriftldc * 0.1);
        let assign93310_e142987: f64 = (assign93310_e142983 * assign93310_e142986);
        (assign93310_e142987, (((locals.var_ddriftldc_dn0 * 0.1) * assign93310_e142986) + (assign93310_e142983 * (locals.var_ddriftldc_dn0 * 0.1))), (((locals.var_ddriftldc_dn2 * 0.1) * assign93310_e142986) + (assign93310_e142983 * (locals.var_ddriftldc_dn2 * 0.1))), (((locals.var_ddriftldc_dn4 * 0.1) * assign93310_e142986) + (assign93310_e142983 * (locals.var_ddriftldc_dn4 * 0.1))), (((locals.var_ddriftldc_dn5 * 0.1) * assign93310_e142986) + (assign93310_e142983 * (locals.var_ddriftldc_dn5 * 0.1))), (((locals.var_ddriftldc_dn6 * 0.1) * assign93310_e142986) + (assign93310_e142983 * (locals.var_ddriftldc_dn6 * 0.1))), (((locals.var_ddriftldc_dn7 * 0.1) * assign93310_e142986) + (assign93310_e142983 * (locals.var_ddriftldc_dn7 * 0.1))), (((locals.var_ddriftldc_dn8 * 0.1) * assign93310_e142986) + (assign93310_e142983 * (locals.var_ddriftldc_dn8 * 0.1))), (((locals.var_ddriftldc_dn9 * 0.1) * assign93310_e142986) + (assign93310_e142983 * (locals.var_ddriftldc_dn9 * 0.1))), (((locals.var_ddriftldc_dn10 * 0.1) * assign93310_e142986) + (assign93310_e142983 * (locals.var_ddriftldc_dn10 * 0.1))), (((locals.var_ddriftldc_dn11 * 0.1) * assign93310_e142986) + (assign93310_e142983 * (locals.var_ddriftldc_dn11 * 0.1))), (((locals.var_ddriftldc_dn14 * 0.1) * assign93310_e142986) + (assign93310_e142983 * (locals.var_ddriftldc_dn14 * 0.1))),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign93310_e142989;
        locals.var_xmax2_dn0 = assign93310_e142989_d_n0;
        locals.var_xmax2_dn2 = assign93310_e142989_d_n2;
        locals.var_xmax2_dn4 = assign93310_e142989_d_n4;
        locals.var_xmax2_dn5 = assign93310_e142989_d_n5;
        locals.var_xmax2_dn6 = assign93310_e142989_d_n6;
        locals.var_xmax2_dn7 = assign93310_e142989_d_n7;
        locals.var_xmax2_dn8 = assign93310_e142989_d_n8;
        locals.var_xmax2_dn9 = assign93310_e142989_d_n9;
        locals.var_xmax2_dn10 = assign93310_e142989_d_n10;
        locals.var_xmax2_dn11 = assign93310_e142989_d_n11;
        locals.var_xmax2_dn14 = assign93310_e142989_d_n14;
        locals.var_xmax2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_360(
        locals: &mut StampLocals,
    ) {
        let (assign93320_e143002, assign93320_e143002_d_n0, assign93320_e143002_d_n2, assign93320_e143002_d_n4, assign93320_e143002_d_n5, assign93320_e143002_d_n6, assign93320_e143002_d_n7, assign93320_e143002_d_n8, assign93320_e143002_d_n9, assign93320_e143002_d_n10, assign93320_e143002_d_n11, assign93320_e143002_d_n14,) = {
    if (((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2163 != 0.0)) && (locals.var_guard2164 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign93320_e143002;
        locals.var_xp_dn0 = assign93320_e143002_d_n0;
        locals.var_xp_dn2 = assign93320_e143002_d_n2;
        locals.var_xp_dn4 = assign93320_e143002_d_n4;
        locals.var_xp_dn5 = assign93320_e143002_d_n5;
        locals.var_xp_dn6 = assign93320_e143002_d_n6;
        locals.var_xp_dn7 = assign93320_e143002_d_n7;
        locals.var_xp_dn8 = assign93320_e143002_d_n8;
        locals.var_xp_dn9 = assign93320_e143002_d_n9;
        locals.var_xp_dn10 = assign93320_e143002_d_n10;
        locals.var_xp_dn11 = assign93320_e143002_d_n11;
        locals.var_xp_dn14 = assign93320_e143002_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign93330_e143015, assign93330_e143015_d_n0, assign93330_e143015_d_n2, assign93330_e143015_d_n4, assign93330_e143015_d_n5, assign93330_e143015_d_n6, assign93330_e143015_d_n7, assign93330_e143015_d_n8, assign93330_e143015_d_n9, assign93330_e143015_d_n10, assign93330_e143015_d_n11, assign93330_e143015_d_n14,) = {
    if (((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2163 != 0.0)) && (locals.var_guard2164 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign93330_e143015;
        locals.var_xmp_dn0 = assign93330_e143015_d_n0;
        locals.var_xmp_dn2 = assign93330_e143015_d_n2;
        locals.var_xmp_dn4 = assign93330_e143015_d_n4;
        locals.var_xmp_dn5 = assign93330_e143015_d_n5;
        locals.var_xmp_dn6 = assign93330_e143015_d_n6;
        locals.var_xmp_dn7 = assign93330_e143015_d_n7;
        locals.var_xmp_dn8 = assign93330_e143015_d_n8;
        locals.var_xmp_dn9 = assign93330_e143015_d_n9;
        locals.var_xmp_dn10 = assign93330_e143015_d_n10;
        locals.var_xmp_dn11 = assign93330_e143015_d_n11;
        locals.var_xmp_dn14 = assign93330_e143015_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign93340_e143028,) = {
    if (((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2163 != 0.0)) && (locals.var_guard2164 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign93340_e143028;
        locals.var_m0_rv = 0.0;

        let (assign93350_e143041,) = {
    if (((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2163 != 0.0)) && (locals.var_guard2164 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign93350_e143041;
        locals.var_mm_rv = 0.0;

        let (assign93360_e143054, assign93360_e143054_d_n0, assign93360_e143054_d_n2, assign93360_e143054_d_n4, assign93360_e143054_d_n5, assign93360_e143054_d_n6, assign93360_e143054_d_n7, assign93360_e143054_d_n8, assign93360_e143054_d_n9, assign93360_e143054_d_n10, assign93360_e143054_d_n11, assign93360_e143054_d_n14,) = {
    if (((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2163 != 0.0)) && (locals.var_guard2164 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign93360_e143054;
        locals.var_arg_dn0 = assign93360_e143054_d_n0;
        locals.var_arg_dn2 = assign93360_e143054_d_n2;
        locals.var_arg_dn4 = assign93360_e143054_d_n4;
        locals.var_arg_dn5 = assign93360_e143054_d_n5;
        locals.var_arg_dn6 = assign93360_e143054_d_n6;
        locals.var_arg_dn7 = assign93360_e143054_d_n7;
        locals.var_arg_dn8 = assign93360_e143054_d_n8;
        locals.var_arg_dn9 = assign93360_e143054_d_n9;
        locals.var_arg_dn10 = assign93360_e143054_d_n10;
        locals.var_arg_dn11 = assign93360_e143054_d_n11;
        locals.var_arg_dn14 = assign93360_e143054_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign93370_e143067, assign93370_e143067_d_n0, assign93370_e143067_d_n2, assign93370_e143067_d_n4, assign93370_e143067_d_n5, assign93370_e143067_d_n6, assign93370_e143067_d_n7, assign93370_e143067_d_n8, assign93370_e143067_d_n9, assign93370_e143067_d_n10, assign93370_e143067_d_n11, assign93370_e143067_d_n14,) = {
    if (((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2163 != 0.0)) && (locals.var_guard2164 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign93370_e143067;
        locals.var_dnm_dn0 = assign93370_e143067_d_n0;
        locals.var_dnm_dn2 = assign93370_e143067_d_n2;
        locals.var_dnm_dn4 = assign93370_e143067_d_n4;
        locals.var_dnm_dn5 = assign93370_e143067_d_n5;
        locals.var_dnm_dn6 = assign93370_e143067_d_n6;
        locals.var_dnm_dn7 = assign93370_e143067_d_n7;
        locals.var_dnm_dn8 = assign93370_e143067_d_n8;
        locals.var_dnm_dn9 = assign93370_e143067_d_n9;
        locals.var_dnm_dn10 = assign93370_e143067_d_n10;
        locals.var_dnm_dn11 = assign93370_e143067_d_n11;
        locals.var_dnm_dn14 = assign93370_e143067_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign93380_e143082, assign93380_e143082_d_n0, assign93380_e143082_d_n2, assign93380_e143082_d_n4, assign93380_e143082_d_n5, assign93380_e143082_d_n6, assign93380_e143082_d_n7, assign93380_e143082_d_n8, assign93380_e143082_d_n9, assign93380_e143082_d_n10, assign93380_e143082_d_n11, assign93380_e143082_d_n14,) = {
    if (((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2163 != 0.0)) && (locals.var_guard2164 != 0.0)) {
        let assign93380_e143080: f64 = (locals.var_xp * locals.var_x2);
        (assign93380_e143080, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign93380_e143082;
        locals.var_xp_dn0 = assign93380_e143082_d_n0;
        locals.var_xp_dn2 = assign93380_e143082_d_n2;
        locals.var_xp_dn4 = assign93380_e143082_d_n4;
        locals.var_xp_dn5 = assign93380_e143082_d_n5;
        locals.var_xp_dn6 = assign93380_e143082_d_n6;
        locals.var_xp_dn7 = assign93380_e143082_d_n7;
        locals.var_xp_dn8 = assign93380_e143082_d_n8;
        locals.var_xp_dn9 = assign93380_e143082_d_n9;
        locals.var_xp_dn10 = assign93380_e143082_d_n10;
        locals.var_xp_dn11 = assign93380_e143082_d_n11;
        locals.var_xp_dn14 = assign93380_e143082_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign93390_e143097, assign93390_e143097_d_n0, assign93390_e143097_d_n2, assign93390_e143097_d_n4, assign93390_e143097_d_n5, assign93390_e143097_d_n6, assign93390_e143097_d_n7, assign93390_e143097_d_n8, assign93390_e143097_d_n9, assign93390_e143097_d_n10, assign93390_e143097_d_n11, assign93390_e143097_d_n14,) = {
    if (((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2163 != 0.0)) && (locals.var_guard2164 != 0.0)) {
        let assign93390_e143095: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign93390_e143095, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign93390_e143097;
        locals.var_xmp_dn0 = assign93390_e143097_d_n0;
        locals.var_xmp_dn2 = assign93390_e143097_d_n2;
        locals.var_xmp_dn4 = assign93390_e143097_d_n4;
        locals.var_xmp_dn5 = assign93390_e143097_d_n5;
        locals.var_xmp_dn6 = assign93390_e143097_d_n6;
        locals.var_xmp_dn7 = assign93390_e143097_d_n7;
        locals.var_xmp_dn8 = assign93390_e143097_d_n8;
        locals.var_xmp_dn9 = assign93390_e143097_d_n9;
        locals.var_xmp_dn10 = assign93390_e143097_d_n10;
        locals.var_xmp_dn11 = assign93390_e143097_d_n11;
        locals.var_xmp_dn14 = assign93390_e143097_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign93400_e143112, assign93400_e143112_d_n0, assign93400_e143112_d_n2, assign93400_e143112_d_n4, assign93400_e143112_d_n5, assign93400_e143112_d_n6, assign93400_e143112_d_n7, assign93400_e143112_d_n8, assign93400_e143112_d_n9, assign93400_e143112_d_n10, assign93400_e143112_d_n11, assign93400_e143112_d_n14,) = {
    if (((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2163 != 0.0)) && (locals.var_guard2164 != 0.0)) {
        let assign93400_e143110: f64 = (locals.var_xp * locals.var_x2);
        (assign93400_e143110, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign93400_e143112;
        locals.var_xp_dn0 = assign93400_e143112_d_n0;
        locals.var_xp_dn2 = assign93400_e143112_d_n2;
        locals.var_xp_dn4 = assign93400_e143112_d_n4;
        locals.var_xp_dn5 = assign93400_e143112_d_n5;
        locals.var_xp_dn6 = assign93400_e143112_d_n6;
        locals.var_xp_dn7 = assign93400_e143112_d_n7;
        locals.var_xp_dn8 = assign93400_e143112_d_n8;
        locals.var_xp_dn9 = assign93400_e143112_d_n9;
        locals.var_xp_dn10 = assign93400_e143112_d_n10;
        locals.var_xp_dn11 = assign93400_e143112_d_n11;
        locals.var_xp_dn14 = assign93400_e143112_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign93410_e143127, assign93410_e143127_d_n0, assign93410_e143127_d_n2, assign93410_e143127_d_n4, assign93410_e143127_d_n5, assign93410_e143127_d_n6, assign93410_e143127_d_n7, assign93410_e143127_d_n8, assign93410_e143127_d_n9, assign93410_e143127_d_n10, assign93410_e143127_d_n11, assign93410_e143127_d_n14,) = {
    if (((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2163 != 0.0)) && (locals.var_guard2164 != 0.0)) {
        let assign93410_e143125: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign93410_e143125, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign93410_e143127;
        locals.var_xmp_dn0 = assign93410_e143127_d_n0;
        locals.var_xmp_dn2 = assign93410_e143127_d_n2;
        locals.var_xmp_dn4 = assign93410_e143127_d_n4;
        locals.var_xmp_dn5 = assign93410_e143127_d_n5;
        locals.var_xmp_dn6 = assign93410_e143127_d_n6;
        locals.var_xmp_dn7 = assign93410_e143127_d_n7;
        locals.var_xmp_dn8 = assign93410_e143127_d_n8;
        locals.var_xmp_dn9 = assign93410_e143127_d_n9;
        locals.var_xmp_dn10 = assign93410_e143127_d_n10;
        locals.var_xmp_dn11 = assign93410_e143127_d_n11;
        locals.var_xmp_dn14 = assign93410_e143127_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign93420_e143142, assign93420_e143142_d_n0, assign93420_e143142_d_n2, assign93420_e143142_d_n4, assign93420_e143142_d_n5, assign93420_e143142_d_n6, assign93420_e143142_d_n7, assign93420_e143142_d_n8, assign93420_e143142_d_n9, assign93420_e143142_d_n10, assign93420_e143142_d_n11, assign93420_e143142_d_n14,) = {
    if (((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2163 != 0.0)) && (locals.var_guard2164 != 0.0)) {
        let assign93420_e143140: f64 = (locals.var_xp + locals.var_xmp);
        (assign93420_e143140, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign93420_e143142;
        locals.var_arg_dn0 = assign93420_e143142_d_n0;
        locals.var_arg_dn2 = assign93420_e143142_d_n2;
        locals.var_arg_dn4 = assign93420_e143142_d_n4;
        locals.var_arg_dn5 = assign93420_e143142_d_n5;
        locals.var_arg_dn6 = assign93420_e143142_d_n6;
        locals.var_arg_dn7 = assign93420_e143142_d_n7;
        locals.var_arg_dn8 = assign93420_e143142_d_n8;
        locals.var_arg_dn9 = assign93420_e143142_d_n9;
        locals.var_arg_dn10 = assign93420_e143142_d_n10;
        locals.var_arg_dn11 = assign93420_e143142_d_n11;
        locals.var_arg_dn14 = assign93420_e143142_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign93430_e143155, assign93430_e143155_d_n0, assign93430_e143155_d_n2, assign93430_e143155_d_n4, assign93430_e143155_d_n5, assign93430_e143155_d_n6, assign93430_e143155_d_n7, assign93430_e143155_d_n8, assign93430_e143155_d_n9, assign93430_e143155_d_n10, assign93430_e143155_d_n11, assign93430_e143155_d_n14,) = {
    if (((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2163 != 0.0)) && (locals.var_guard2164 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign93430_e143155;
        locals.var_dnm_dn0 = assign93430_e143155_d_n0;
        locals.var_dnm_dn2 = assign93430_e143155_d_n2;
        locals.var_dnm_dn4 = assign93430_e143155_d_n4;
        locals.var_dnm_dn5 = assign93430_e143155_d_n5;
        locals.var_dnm_dn6 = assign93430_e143155_d_n6;
        locals.var_dnm_dn7 = assign93430_e143155_d_n7;
        locals.var_dnm_dn8 = assign93430_e143155_d_n8;
        locals.var_dnm_dn9 = assign93430_e143155_d_n9;
        locals.var_dnm_dn10 = assign93430_e143155_d_n10;
        locals.var_dnm_dn11 = assign93430_e143155_d_n11;
        locals.var_dnm_dn14 = assign93430_e143155_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign93440_e143170: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard2165 = assign93440_e143170;
        locals.var_guard2165_rv = 0.0;

        let assign93450_e143173: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2166 = assign93450_e143173;
        locals.var_guard2166_rv = 0.0;

        let (assign93460_e143190,) = {
    if (((((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2163 != 0.0)) && (locals.var_guard2164 != 0.0)) && (locals.var_guard2165 != 0.0)) && (locals.var_guard2166 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign93460_e143190;
        locals.var_mm_rv = 0.0;

        let assign93470_e143193: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard2167 = assign93470_e143193;
        locals.var_guard2167_rv = 0.0;

        let (assign93480_e143213,) = {
    if ((((((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2163 != 0.0)) && (locals.var_guard2164 != 0.0)) && (locals.var_guard2165 != 0.0)) && (locals.var_guard2166 == 0.0)) && (locals.var_guard2167 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign93480_e143213;
        locals.var_mm_rv = 0.0;

        let assign93490_e143216: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard2168 = assign93490_e143216;
        locals.var_guard2168_rv = 0.0;

        let (assign93500_e143239,) = {
    if (((((((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2163 != 0.0)) && (locals.var_guard2164 != 0.0)) && (locals.var_guard2165 != 0.0)) && (locals.var_guard2166 == 0.0)) && (locals.var_guard2167 == 0.0)) && (locals.var_guard2168 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign93500_e143239;
        locals.var_mm_rv = 0.0;

        let assign93510_e143242: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard2169 = assign93510_e143242;
        locals.var_guard2169_rv = 0.0;

        let (assign93520_e143268,) = {
    if ((((((((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2163 != 0.0)) && (locals.var_guard2164 != 0.0)) && (locals.var_guard2165 != 0.0)) && (locals.var_guard2166 == 0.0)) && (locals.var_guard2167 == 0.0)) && (locals.var_guard2168 == 0.0)) && (locals.var_guard2169 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign93520_e143268;
        locals.var_mm_rv = 0.0;

        let (assign93530_e143283,) = {
    if ((((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2163 != 0.0)) && (locals.var_guard2164 != 0.0)) && (locals.var_guard2165 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign93530_e143283;
        locals.var_m0_rv = 0.0;

        let mut assign93540_loop_guard: usize = 0;
        while {
            let assign93540_cond_e143299: f64 = if (((((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2163 != 0.0)) && (locals.var_guard2164 != 0.0)) && (locals.var_guard2165 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign93540_cond_e143299 != 0.0
        } {
            assign93540_loop_guard += 1;
            assert!(assign93540_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign93540_body0_e143315, assign93540_body0_e143315_d_n0, assign93540_body0_e143315_d_n2, assign93540_body0_e143315_d_n4, assign93540_body0_e143315_d_n5, assign93540_body0_e143315_d_n6, assign93540_body0_e143315_d_n7, assign93540_body0_e143315_d_n8, assign93540_body0_e143315_d_n9, assign93540_body0_e143315_d_n10, assign93540_body0_e143315_d_n11, assign93540_body0_e143315_d_n14,) = {
    if ((((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2163 != 0.0)) && (locals.var_guard2164 != 0.0)) && (locals.var_guard2165 != 0.0)) {
        let assign93540_body0_e143313: f64 = (locals.var_dnm).sqrt();
        (assign93540_body0_e143313, (locals.var_dnm_dn0 / (2.0 * assign93540_body0_e143313)), (locals.var_dnm_dn2 / (2.0 * assign93540_body0_e143313)), (locals.var_dnm_dn4 / (2.0 * assign93540_body0_e143313)), (locals.var_dnm_dn5 / (2.0 * assign93540_body0_e143313)), (locals.var_dnm_dn6 / (2.0 * assign93540_body0_e143313)), (locals.var_dnm_dn7 / (2.0 * assign93540_body0_e143313)), (locals.var_dnm_dn8 / (2.0 * assign93540_body0_e143313)), (locals.var_dnm_dn9 / (2.0 * assign93540_body0_e143313)), (locals.var_dnm_dn10 / (2.0 * assign93540_body0_e143313)), (locals.var_dnm_dn11 / (2.0 * assign93540_body0_e143313)), (locals.var_dnm_dn14 / (2.0 * assign93540_body0_e143313)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign93540_body0_e143315;
            locals.var_dnm_dn0 = assign93540_body0_e143315_d_n0;
            locals.var_dnm_dn2 = assign93540_body0_e143315_d_n2;
            locals.var_dnm_dn4 = assign93540_body0_e143315_d_n4;
            locals.var_dnm_dn5 = assign93540_body0_e143315_d_n5;
            locals.var_dnm_dn6 = assign93540_body0_e143315_d_n6;
            locals.var_dnm_dn7 = assign93540_body0_e143315_d_n7;
            locals.var_dnm_dn8 = assign93540_body0_e143315_d_n8;
            locals.var_dnm_dn9 = assign93540_body0_e143315_d_n9;
            locals.var_dnm_dn10 = assign93540_body0_e143315_d_n10;
            locals.var_dnm_dn11 = assign93540_body0_e143315_d_n11;
            locals.var_dnm_dn14 = assign93540_body0_e143315_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign93540_body1_e143332,) = {
    if ((((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2163 != 0.0)) && (locals.var_guard2164 != 0.0)) && (locals.var_guard2165 != 0.0)) {
        let assign93540_body1_e143330: f64 = (locals.var_m0 + 1.0);
        (assign93540_body1_e143330,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign93540_body1_e143332;
            locals.var_m0_rv = 0.0;
        }

        let (assign93550_e143359, assign93550_e143359_d_n0, assign93550_e143359_d_n2, assign93550_e143359_d_n4, assign93550_e143359_d_n5, assign93550_e143359_d_n6, assign93550_e143359_d_n7, assign93550_e143359_d_n8, assign93550_e143359_d_n9, assign93550_e143359_d_n10, assign93550_e143359_d_n11, assign93550_e143359_d_n14,) = {
    if ((((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2163 != 0.0)) && (locals.var_guard2164 != 0.0)) && (locals.var_guard2165 == 0.0)) {
        let (assign93550_e143357, assign93550_e143357_d_n0, assign93550_e143357_d_n2, assign93550_e143357_d_n4, assign93550_e143357_d_n5, assign93550_e143357_d_n6, assign93550_e143357_d_n7, assign93550_e143357_d_n8, assign93550_e143357_d_n9, assign93550_e143357_d_n10, assign93550_e143357_d_n11, assign93550_e143357_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign93550_e143354: f64 = (2.0 * 2.0);
                let assign93550_e143355: f64 = (1.0 / assign93550_e143354);
                let assign93550_e143356: f64 = (locals.var_dnm).powf(assign93550_e143355);
                (assign93550_e143356, if 0.0 == 0.0 && ((assign93550_e143355) as f64).is_finite() && ((assign93550_e143355) as f64).fract() == 0.0 { if assign93550_e143355 == 0.0 { 0.0 } else { (assign93550_e143355 * ((locals.var_dnm).powf(assign93550_e143355 - 1.0) * locals.var_dnm_dn0)) } } else { (assign93550_e143356 * (assign93550_e143355 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign93550_e143355) as f64).is_finite() && ((assign93550_e143355) as f64).fract() == 0.0 { if assign93550_e143355 == 0.0 { 0.0 } else { (assign93550_e143355 * ((locals.var_dnm).powf(assign93550_e143355 - 1.0) * locals.var_dnm_dn2)) } } else { (assign93550_e143356 * (assign93550_e143355 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign93550_e143355) as f64).is_finite() && ((assign93550_e143355) as f64).fract() == 0.0 { if assign93550_e143355 == 0.0 { 0.0 } else { (assign93550_e143355 * ((locals.var_dnm).powf(assign93550_e143355 - 1.0) * locals.var_dnm_dn4)) } } else { (assign93550_e143356 * (assign93550_e143355 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign93550_e143355) as f64).is_finite() && ((assign93550_e143355) as f64).fract() == 0.0 { if assign93550_e143355 == 0.0 { 0.0 } else { (assign93550_e143355 * ((locals.var_dnm).powf(assign93550_e143355 - 1.0) * locals.var_dnm_dn5)) } } else { (assign93550_e143356 * (assign93550_e143355 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign93550_e143355) as f64).is_finite() && ((assign93550_e143355) as f64).fract() == 0.0 { if assign93550_e143355 == 0.0 { 0.0 } else { (assign93550_e143355 * ((locals.var_dnm).powf(assign93550_e143355 - 1.0) * locals.var_dnm_dn6)) } } else { (assign93550_e143356 * (assign93550_e143355 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign93550_e143355) as f64).is_finite() && ((assign93550_e143355) as f64).fract() == 0.0 { if assign93550_e143355 == 0.0 { 0.0 } else { (assign93550_e143355 * ((locals.var_dnm).powf(assign93550_e143355 - 1.0) * locals.var_dnm_dn7)) } } else { (assign93550_e143356 * (assign93550_e143355 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign93550_e143355) as f64).is_finite() && ((assign93550_e143355) as f64).fract() == 0.0 { if assign93550_e143355 == 0.0 { 0.0 } else { (assign93550_e143355 * ((locals.var_dnm).powf(assign93550_e143355 - 1.0) * locals.var_dnm_dn8)) } } else { (assign93550_e143356 * (assign93550_e143355 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign93550_e143355) as f64).is_finite() && ((assign93550_e143355) as f64).fract() == 0.0 { if assign93550_e143355 == 0.0 { 0.0 } else { (assign93550_e143355 * ((locals.var_dnm).powf(assign93550_e143355 - 1.0) * locals.var_dnm_dn9)) } } else { (assign93550_e143356 * (assign93550_e143355 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign93550_e143355) as f64).is_finite() && ((assign93550_e143355) as f64).fract() == 0.0 { if assign93550_e143355 == 0.0 { 0.0 } else { (assign93550_e143355 * ((locals.var_dnm).powf(assign93550_e143355 - 1.0) * locals.var_dnm_dn10)) } } else { (assign93550_e143356 * (assign93550_e143355 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign93550_e143355) as f64).is_finite() && ((assign93550_e143355) as f64).fract() == 0.0 { if assign93550_e143355 == 0.0 { 0.0 } else { (assign93550_e143355 * ((locals.var_dnm).powf(assign93550_e143355 - 1.0) * locals.var_dnm_dn11)) } } else { (assign93550_e143356 * (assign93550_e143355 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign93550_e143355) as f64).is_finite() && ((assign93550_e143355) as f64).fract() == 0.0 { if assign93550_e143355 == 0.0 { 0.0 } else { (assign93550_e143355 * ((locals.var_dnm).powf(assign93550_e143355 - 1.0) * locals.var_dnm_dn14)) } } else { (assign93550_e143356 * (assign93550_e143355 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign93550_e143357, assign93550_e143357_d_n0, assign93550_e143357_d_n2, assign93550_e143357_d_n4, assign93550_e143357_d_n5, assign93550_e143357_d_n6, assign93550_e143357_d_n7, assign93550_e143357_d_n8, assign93550_e143357_d_n9, assign93550_e143357_d_n10, assign93550_e143357_d_n11, assign93550_e143357_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign93550_e143359;
        locals.var_dnm_dn0 = assign93550_e143359_d_n0;
        locals.var_dnm_dn2 = assign93550_e143359_d_n2;
        locals.var_dnm_dn4 = assign93550_e143359_d_n4;
        locals.var_dnm_dn5 = assign93550_e143359_d_n5;
        locals.var_dnm_dn6 = assign93550_e143359_d_n6;
        locals.var_dnm_dn7 = assign93550_e143359_d_n7;
        locals.var_dnm_dn8 = assign93550_e143359_d_n8;
        locals.var_dnm_dn9 = assign93550_e143359_d_n9;
        locals.var_dnm_dn10 = assign93550_e143359_d_n10;
        locals.var_dnm_dn11 = assign93550_e143359_d_n11;
        locals.var_dnm_dn14 = assign93550_e143359_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign93560_e143374, assign93560_e143374_d_n0, assign93560_e143374_d_n2, assign93560_e143374_d_n4, assign93560_e143374_d_n5, assign93560_e143374_d_n6, assign93560_e143374_d_n7, assign93560_e143374_d_n8, assign93560_e143374_d_n9, assign93560_e143374_d_n10, assign93560_e143374_d_n11, assign93560_e143374_d_n14,) = {
    if (((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2163 != 0.0)) && (locals.var_guard2164 != 0.0)) {
        let assign93560_e143372: f64 = (1.0 / locals.var_dnm);
        (assign93560_e143372, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign93560_e143374;
        locals.var_dnm_dn0 = assign93560_e143374_d_n0;
        locals.var_dnm_dn2 = assign93560_e143374_d_n2;
        locals.var_dnm_dn4 = assign93560_e143374_d_n4;
        locals.var_dnm_dn5 = assign93560_e143374_d_n5;
        locals.var_dnm_dn6 = assign93560_e143374_d_n6;
        locals.var_dnm_dn7 = assign93560_e143374_d_n7;
        locals.var_dnm_dn8 = assign93560_e143374_d_n8;
        locals.var_dnm_dn9 = assign93560_e143374_d_n9;
        locals.var_dnm_dn10 = assign93560_e143374_d_n10;
        locals.var_dnm_dn11 = assign93560_e143374_d_n11;
        locals.var_dnm_dn14 = assign93560_e143374_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign93570_e143393, assign93570_e143393_d_n0, assign93570_e143393_d_n2, assign93570_e143393_d_n4, assign93570_e143393_d_n5, assign93570_e143393_d_n6, assign93570_e143393_d_n7, assign93570_e143393_d_n8, assign93570_e143393_d_n9, assign93570_e143393_d_n10, assign93570_e143393_d_n11, assign93570_e143393_d_n14,) = {
    if (((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2163 != 0.0)) && (locals.var_guard2164 != 0.0)) {
        let assign93570_e143388: f64 = (locals.var_ddriftldc * 0.1);
        let assign93570_e143389: f64 = (locals.var_tmf1 * assign93570_e143388);
        let assign93570_e143391: f64 = (assign93570_e143389 * locals.var_dnm);
        (assign93570_e143391, ((((locals.var_tmf1_dn0 * assign93570_e143388) + (locals.var_tmf1 * (locals.var_ddriftldc_dn0 * 0.1))) * locals.var_dnm) + (assign93570_e143389 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * assign93570_e143388) + (locals.var_tmf1 * (locals.var_ddriftldc_dn2 * 0.1))) * locals.var_dnm) + (assign93570_e143389 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn4 * assign93570_e143388) + (locals.var_tmf1 * (locals.var_ddriftldc_dn4 * 0.1))) * locals.var_dnm) + (assign93570_e143389 * locals.var_dnm_dn4)), ((((locals.var_tmf1_dn5 * assign93570_e143388) + (locals.var_tmf1 * (locals.var_ddriftldc_dn5 * 0.1))) * locals.var_dnm) + (assign93570_e143389 * locals.var_dnm_dn5)), ((((locals.var_tmf1_dn6 * assign93570_e143388) + (locals.var_tmf1 * (locals.var_ddriftldc_dn6 * 0.1))) * locals.var_dnm) + (assign93570_e143389 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * assign93570_e143388) + (locals.var_tmf1 * (locals.var_ddriftldc_dn7 * 0.1))) * locals.var_dnm) + (assign93570_e143389 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn8 * assign93570_e143388) + (locals.var_tmf1 * (locals.var_ddriftldc_dn8 * 0.1))) * locals.var_dnm) + (assign93570_e143389 * locals.var_dnm_dn8)), ((((locals.var_tmf1_dn9 * assign93570_e143388) + (locals.var_tmf1 * (locals.var_ddriftldc_dn9 * 0.1))) * locals.var_dnm) + (assign93570_e143389 * locals.var_dnm_dn9)), ((((locals.var_tmf1_dn10 * assign93570_e143388) + (locals.var_tmf1 * (locals.var_ddriftldc_dn10 * 0.1))) * locals.var_dnm) + (assign93570_e143389 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn11 * assign93570_e143388) + (locals.var_tmf1 * (locals.var_ddriftldc_dn11 * 0.1))) * locals.var_dnm) + (assign93570_e143389 * locals.var_dnm_dn11)), ((((locals.var_tmf1_dn14 * assign93570_e143388) + (locals.var_tmf1 * (locals.var_ddriftldc_dn14 * 0.1))) * locals.var_dnm) + (assign93570_e143389 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign93570_e143393;
        locals.var_tmf0_dn0 = assign93570_e143393_d_n0;
        locals.var_tmf0_dn2 = assign93570_e143393_d_n2;
        locals.var_tmf0_dn4 = assign93570_e143393_d_n4;
        locals.var_tmf0_dn5 = assign93570_e143393_d_n5;
        locals.var_tmf0_dn6 = assign93570_e143393_d_n6;
        locals.var_tmf0_dn7 = assign93570_e143393_d_n7;
        locals.var_tmf0_dn8 = assign93570_e143393_d_n8;
        locals.var_tmf0_dn9 = assign93570_e143393_d_n9;
        locals.var_tmf0_dn10 = assign93570_e143393_d_n10;
        locals.var_tmf0_dn11 = assign93570_e143393_d_n11;
        locals.var_tmf0_dn14 = assign93570_e143393_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign93580_e143414, assign93580_e143414_d_n0, assign93580_e143414_d_n2, assign93580_e143414_d_n4, assign93580_e143414_d_n5, assign93580_e143414_d_n6, assign93580_e143414_d_n7, assign93580_e143414_d_n8, assign93580_e143414_d_n9, assign93580_e143414_d_n10, assign93580_e143414_d_n11, assign93580_e143414_d_n14,) = {
    if (((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2163 != 0.0)) && (locals.var_guard2164 != 0.0)) {
        let assign93580_e143406: f64 = (locals.var_ddriftldc * 0.1);
        let assign93580_e143408: f64 = (assign93580_e143406 * locals.var_xmp);
        let assign93580_e143410: f64 = (assign93580_e143408 * locals.var_dnm);
        let assign93580_e143412: f64 = (assign93580_e143410 / locals.var_arg);
        (assign93580_e143412, ((((((((locals.var_ddriftldc_dn0 * 0.1) * locals.var_xmp) + (assign93580_e143406 * locals.var_xmp_dn0)) * locals.var_dnm) + (assign93580_e143408 * locals.var_dnm_dn0)) * locals.var_arg) - (assign93580_e143410 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn2 * 0.1) * locals.var_xmp) + (assign93580_e143406 * locals.var_xmp_dn2)) * locals.var_dnm) + (assign93580_e143408 * locals.var_dnm_dn2)) * locals.var_arg) - (assign93580_e143410 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn4 * 0.1) * locals.var_xmp) + (assign93580_e143406 * locals.var_xmp_dn4)) * locals.var_dnm) + (assign93580_e143408 * locals.var_dnm_dn4)) * locals.var_arg) - (assign93580_e143410 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn5 * 0.1) * locals.var_xmp) + (assign93580_e143406 * locals.var_xmp_dn5)) * locals.var_dnm) + (assign93580_e143408 * locals.var_dnm_dn5)) * locals.var_arg) - (assign93580_e143410 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn6 * 0.1) * locals.var_xmp) + (assign93580_e143406 * locals.var_xmp_dn6)) * locals.var_dnm) + (assign93580_e143408 * locals.var_dnm_dn6)) * locals.var_arg) - (assign93580_e143410 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn7 * 0.1) * locals.var_xmp) + (assign93580_e143406 * locals.var_xmp_dn7)) * locals.var_dnm) + (assign93580_e143408 * locals.var_dnm_dn7)) * locals.var_arg) - (assign93580_e143410 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn8 * 0.1) * locals.var_xmp) + (assign93580_e143406 * locals.var_xmp_dn8)) * locals.var_dnm) + (assign93580_e143408 * locals.var_dnm_dn8)) * locals.var_arg) - (assign93580_e143410 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn9 * 0.1) * locals.var_xmp) + (assign93580_e143406 * locals.var_xmp_dn9)) * locals.var_dnm) + (assign93580_e143408 * locals.var_dnm_dn9)) * locals.var_arg) - (assign93580_e143410 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn10 * 0.1) * locals.var_xmp) + (assign93580_e143406 * locals.var_xmp_dn10)) * locals.var_dnm) + (assign93580_e143408 * locals.var_dnm_dn10)) * locals.var_arg) - (assign93580_e143410 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn11 * 0.1) * locals.var_xmp) + (assign93580_e143406 * locals.var_xmp_dn11)) * locals.var_dnm) + (assign93580_e143408 * locals.var_dnm_dn11)) * locals.var_arg) - (assign93580_e143410 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn14 * 0.1) * locals.var_xmp) + (assign93580_e143406 * locals.var_xmp_dn14)) * locals.var_dnm) + (assign93580_e143408 * locals.var_dnm_dn14)) * locals.var_arg) - (assign93580_e143410 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign93580_e143414;
        locals.var_t0_dn0 = assign93580_e143414_d_n0;
        locals.var_t0_dn2 = assign93580_e143414_d_n2;
        locals.var_t0_dn4 = assign93580_e143414_d_n4;
        locals.var_t0_dn5 = assign93580_e143414_d_n5;
        locals.var_t0_dn6 = assign93580_e143414_d_n6;
        locals.var_t0_dn7 = assign93580_e143414_d_n7;
        locals.var_t0_dn8 = assign93580_e143414_d_n8;
        locals.var_t0_dn9 = assign93580_e143414_d_n9;
        locals.var_t0_dn10 = assign93580_e143414_d_n10;
        locals.var_t0_dn11 = assign93580_e143414_d_n11;
        locals.var_t0_dn14 = assign93580_e143414_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign93590_e143433, assign93590_e143433_d_n0, assign93590_e143433_d_n2, assign93590_e143433_d_n4, assign93590_e143433_d_n5, assign93590_e143433_d_n6, assign93590_e143433_d_n7, assign93590_e143433_d_n8, assign93590_e143433_d_n9, assign93590_e143433_d_n10, assign93590_e143433_d_n11, assign93590_e143433_d_n14,) = {
    if (((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2163 != 0.0)) && (locals.var_guard2164 != 0.0)) {
        let assign93590_e143428: f64 = (locals.var_ddriftldc * 0.1);
        let assign93590_e143429: f64 = (locals.var_ddriftldc - assign93590_e143428);
        let assign93590_e143431: f64 = (assign93590_e143429 + locals.var_tmf0);
        (assign93590_e143431, ((locals.var_ddriftldc_dn0 - (locals.var_ddriftldc_dn0 * 0.1)) + locals.var_tmf0_dn0), ((locals.var_ddriftldc_dn2 - (locals.var_ddriftldc_dn2 * 0.1)) + locals.var_tmf0_dn2), ((locals.var_ddriftldc_dn4 - (locals.var_ddriftldc_dn4 * 0.1)) + locals.var_tmf0_dn4), ((locals.var_ddriftldc_dn5 - (locals.var_ddriftldc_dn5 * 0.1)) + locals.var_tmf0_dn5), ((locals.var_ddriftldc_dn6 - (locals.var_ddriftldc_dn6 * 0.1)) + locals.var_tmf0_dn6), ((locals.var_ddriftldc_dn7 - (locals.var_ddriftldc_dn7 * 0.1)) + locals.var_tmf0_dn7), ((locals.var_ddriftldc_dn8 - (locals.var_ddriftldc_dn8 * 0.1)) + locals.var_tmf0_dn8), ((locals.var_ddriftldc_dn9 - (locals.var_ddriftldc_dn9 * 0.1)) + locals.var_tmf0_dn9), ((locals.var_ddriftldc_dn10 - (locals.var_ddriftldc_dn10 * 0.1)) + locals.var_tmf0_dn10), ((locals.var_ddriftldc_dn11 - (locals.var_ddriftldc_dn11 * 0.1)) + locals.var_tmf0_dn11), ((locals.var_ddriftldc_dn14 - (locals.var_ddriftldc_dn14 * 0.1)) + locals.var_tmf0_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign93590_e143433;
        locals.var_t1_dn0 = assign93590_e143433_d_n0;
        locals.var_t1_dn2 = assign93590_e143433_d_n2;
        locals.var_t1_dn4 = assign93590_e143433_d_n4;
        locals.var_t1_dn5 = assign93590_e143433_d_n5;
        locals.var_t1_dn6 = assign93590_e143433_d_n6;
        locals.var_t1_dn7 = assign93590_e143433_d_n7;
        locals.var_t1_dn8 = assign93590_e143433_d_n8;
        locals.var_t1_dn9 = assign93590_e143433_d_n9;
        locals.var_t1_dn10 = assign93590_e143433_d_n10;
        locals.var_t1_dn11 = assign93590_e143433_d_n11;
        locals.var_t1_dn14 = assign93590_e143433_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign93600_e143446, assign93600_e143446_d_n0, assign93600_e143446_d_n2, assign93600_e143446_d_n4, assign93600_e143446_d_n5, assign93600_e143446_d_n6, assign93600_e143446_d_n7, assign93600_e143446_d_n8, assign93600_e143446_d_n9, assign93600_e143446_d_n10, assign93600_e143446_d_n11, assign93600_e143446_d_n14,) = {
    if (((((locals.var_guard2113 != 0.0) && (locals.var_guard2114 != 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2163 != 0.0)) && (locals.var_guard2164 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign93600_e143446;
        locals.var_t0_dn0 = assign93600_e143446_d_n0;
        locals.var_t0_dn2 = assign93600_e143446_d_n2;
        locals.var_t0_dn4 = assign93600_e143446_d_n4;
        locals.var_t0_dn5 = assign93600_e143446_d_n5;
        locals.var_t0_dn6 = assign93600_e143446_d_n6;
        locals.var_t0_dn7 = assign93600_e143446_d_n7;
        locals.var_t0_dn8 = assign93600_e143446_d_n8;
        locals.var_t0_dn9 = assign93600_e143446_d_n9;
        locals.var_t0_dn10 = assign93600_e143446_d_n10;
        locals.var_t0_dn11 = assign93600_e143446_d_n11;
        locals.var_t0_dn14 = assign93600_e143446_d_n14;
        locals.var_t0_rv = 0.0;

    }
}
