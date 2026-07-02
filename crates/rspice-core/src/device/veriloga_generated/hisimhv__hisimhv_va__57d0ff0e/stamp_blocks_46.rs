#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_345(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign89840_e137757, assign89840_e137757_d_n0, assign89840_e137757_d_n2, assign89840_e137757_d_n4, assign89840_e137757_d_n5, assign89840_e137757_d_n6, assign89840_e137757_d_n7, assign89840_e137757_d_n8, assign89840_e137757_d_n9, assign89840_e137757_d_n10, assign89840_e137757_d_n11, assign89840_e137757_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2089 != 0.0)) {
        let assign89840_e137744: f64 = (locals.var_t2 * locals.var_t2);
        let assign89840_e137748: f64 = (p.p334 * 0.01);
        let assign89840_e137749: f64 = (4.0 * assign89840_e137748);
        let assign89840_e137752: f64 = (p.p334 * 0.01);
        let assign89840_e137753: f64 = (assign89840_e137749 * assign89840_e137752);
        let assign89840_e137754: f64 = (assign89840_e137744 + assign89840_e137753);
        let assign89840_e137755: f64 = (assign89840_e137754).sqrt();
        (assign89840_e137755, (((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)) / (2.0 * assign89840_e137755)), (((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)) / (2.0 * assign89840_e137755)), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign89840_e137755)), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign89840_e137755)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign89840_e137755)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign89840_e137755)), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign89840_e137755)), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign89840_e137755)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign89840_e137755)), (((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign89840_e137755)), (((locals.var_t2_dn14 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn14)) / (2.0 * assign89840_e137755)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign89840_e137757;
        locals.var_tmf2_dn0 = assign89840_e137757_d_n0;
        locals.var_tmf2_dn2 = assign89840_e137757_d_n2;
        locals.var_tmf2_dn4 = assign89840_e137757_d_n4;
        locals.var_tmf2_dn5 = assign89840_e137757_d_n5;
        locals.var_tmf2_dn6 = assign89840_e137757_d_n6;
        locals.var_tmf2_dn7 = assign89840_e137757_d_n7;
        locals.var_tmf2_dn8 = assign89840_e137757_d_n8;
        locals.var_tmf2_dn9 = assign89840_e137757_d_n9;
        locals.var_tmf2_dn10 = assign89840_e137757_d_n10;
        locals.var_tmf2_dn11 = assign89840_e137757_d_n11;
        locals.var_tmf2_dn14 = assign89840_e137757_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign89850_e137769, assign89850_e137769_d_n0, assign89850_e137769_d_n2, assign89850_e137769_d_n4, assign89850_e137769_d_n5, assign89850_e137769_d_n6, assign89850_e137769_d_n7, assign89850_e137769_d_n8, assign89850_e137769_d_n9, assign89850_e137769_d_n10, assign89850_e137769_d_n11, assign89850_e137769_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2089 != 0.0)) {
        let assign89850_e137765: f64 = (locals.var_t2 / locals.var_tmf2);
        let assign89850_e137766: f64 = (1.0 + assign89850_e137765);
        let assign89850_e137767: f64 = (0.5 * assign89850_e137766);
        (assign89850_e137767, (0.5 * (((locals.var_t2_dn0 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn2 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn4 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn5 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn6 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn7 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn8 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn9 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn10 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn11 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn14 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign89850_e137769;
        locals.var_t9_dn0 = assign89850_e137769_d_n0;
        locals.var_t9_dn2 = assign89850_e137769_d_n2;
        locals.var_t9_dn4 = assign89850_e137769_d_n4;
        locals.var_t9_dn5 = assign89850_e137769_d_n5;
        locals.var_t9_dn6 = assign89850_e137769_d_n6;
        locals.var_t9_dn7 = assign89850_e137769_d_n7;
        locals.var_t9_dn8 = assign89850_e137769_d_n8;
        locals.var_t9_dn9 = assign89850_e137769_d_n9;
        locals.var_t9_dn10 = assign89850_e137769_d_n10;
        locals.var_t9_dn11 = assign89850_e137769_d_n11;
        locals.var_t9_dn14 = assign89850_e137769_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign89860_e137779, assign89860_e137779_d_n0, assign89860_e137779_d_n2, assign89860_e137779_d_n4, assign89860_e137779_d_n5, assign89860_e137779_d_n6, assign89860_e137779_d_n7, assign89860_e137779_d_n8, assign89860_e137779_d_n9, assign89860_e137779_d_n10, assign89860_e137779_d_n11, assign89860_e137779_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2089 != 0.0)) {
        let assign89860_e137776: f64 = (locals.var_t2 + locals.var_tmf2);
        let assign89860_e137777: f64 = (0.5 * assign89860_e137776);
        (assign89860_e137777, (0.5 * (locals.var_t2_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t2_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t2_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t2_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t2_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t2_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t2_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t2_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t2_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t2_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_t2_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign89860_e137779;
        locals.var_t2_dn0 = assign89860_e137779_d_n0;
        locals.var_t2_dn2 = assign89860_e137779_d_n2;
        locals.var_t2_dn4 = assign89860_e137779_d_n4;
        locals.var_t2_dn5 = assign89860_e137779_d_n5;
        locals.var_t2_dn6 = assign89860_e137779_d_n6;
        locals.var_t2_dn7 = assign89860_e137779_d_n7;
        locals.var_t2_dn8 = assign89860_e137779_d_n8;
        locals.var_t2_dn9 = assign89860_e137779_d_n9;
        locals.var_t2_dn10 = assign89860_e137779_d_n10;
        locals.var_t2_dn11 = assign89860_e137779_d_n11;
        locals.var_t2_dn14 = assign89860_e137779_d_n14;
        locals.var_t2_rv = 0.0;

        let assign89870_e137782: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2092 = assign89870_e137782;
        locals.var_guard2092_rv = 0.0;

        let (assign89880_e137790, assign89880_e137790_d_n0, assign89880_e137790_d_n2, assign89880_e137790_d_n4, assign89880_e137790_d_n5, assign89880_e137790_d_n6, assign89880_e137790_d_n7, assign89880_e137790_d_n8, assign89880_e137790_d_n9, assign89880_e137790_d_n10, assign89880_e137790_d_n11, assign89880_e137790_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2089 != 0.0)) && (locals.var_guard2092 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign89880_e137790;
        locals.var_t2_dn0 = assign89880_e137790_d_n0;
        locals.var_t2_dn2 = assign89880_e137790_d_n2;
        locals.var_t2_dn4 = assign89880_e137790_d_n4;
        locals.var_t2_dn5 = assign89880_e137790_d_n5;
        locals.var_t2_dn6 = assign89880_e137790_d_n6;
        locals.var_t2_dn7 = assign89880_e137790_d_n7;
        locals.var_t2_dn8 = assign89880_e137790_d_n8;
        locals.var_t2_dn9 = assign89880_e137790_d_n9;
        locals.var_t2_dn10 = assign89880_e137790_d_n10;
        locals.var_t2_dn11 = assign89880_e137790_d_n11;
        locals.var_t2_dn14 = assign89880_e137790_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign89890_e137798, assign89890_e137798_d_n0, assign89890_e137798_d_n2, assign89890_e137798_d_n4, assign89890_e137798_d_n5, assign89890_e137798_d_n6, assign89890_e137798_d_n7, assign89890_e137798_d_n8, assign89890_e137798_d_n9, assign89890_e137798_d_n10, assign89890_e137798_d_n11, assign89890_e137798_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2089 != 0.0)) && (locals.var_guard2092 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign89890_e137798;
        locals.var_t9_dn0 = assign89890_e137798_d_n0;
        locals.var_t9_dn2 = assign89890_e137798_d_n2;
        locals.var_t9_dn4 = assign89890_e137798_d_n4;
        locals.var_t9_dn5 = assign89890_e137798_d_n5;
        locals.var_t9_dn6 = assign89890_e137798_d_n6;
        locals.var_t9_dn7 = assign89890_e137798_d_n7;
        locals.var_t9_dn8 = assign89890_e137798_d_n8;
        locals.var_t9_dn9 = assign89890_e137798_d_n9;
        locals.var_t9_dn10 = assign89890_e137798_d_n10;
        locals.var_t9_dn11 = assign89890_e137798_d_n11;
        locals.var_t9_dn14 = assign89890_e137798_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign89900_e137804, assign89900_e137804_d_n0, assign89900_e137804_d_n2, assign89900_e137804_d_n4, assign89900_e137804_d_n5, assign89900_e137804_d_n6, assign89900_e137804_d_n7, assign89900_e137804_d_n8, assign89900_e137804_d_n9, assign89900_e137804_d_n10, assign89900_e137804_d_n11, assign89900_e137804_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2089 != 0.0)) {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    } else {
        (locals.var_ddriftldc, locals.var_ddriftldc_dn0, locals.var_ddriftldc_dn2, locals.var_ddriftldc_dn4, locals.var_ddriftldc_dn5, locals.var_ddriftldc_dn6, locals.var_ddriftldc_dn7, locals.var_ddriftldc_dn8, locals.var_ddriftldc_dn9, locals.var_ddriftldc_dn10, locals.var_ddriftldc_dn11, locals.var_ddriftldc_dn14,)
    }
};
        locals.var_ddriftldc = assign89900_e137804;
        locals.var_ddriftldc_dn0 = assign89900_e137804_d_n0;
        locals.var_ddriftldc_dn2 = assign89900_e137804_d_n2;
        locals.var_ddriftldc_dn4 = assign89900_e137804_d_n4;
        locals.var_ddriftldc_dn5 = assign89900_e137804_d_n5;
        locals.var_ddriftldc_dn6 = assign89900_e137804_d_n6;
        locals.var_ddriftldc_dn7 = assign89900_e137804_d_n7;
        locals.var_ddriftldc_dn8 = assign89900_e137804_d_n8;
        locals.var_ddriftldc_dn9 = assign89900_e137804_d_n9;
        locals.var_ddriftldc_dn10 = assign89900_e137804_d_n10;
        locals.var_ddriftldc_dn11 = assign89900_e137804_d_n11;
        locals.var_ddriftldc_dn14 = assign89900_e137804_d_n14;
        locals.var_ddriftldc_rv = 0.0;

        let (assign89910_e137818, assign89910_e137818_d_n0, assign89910_e137818_d_n2, assign89910_e137818_d_n4, assign89910_e137818_d_n5, assign89910_e137818_d_n6, assign89910_e137818_d_n7, assign89910_e137818_d_n8, assign89910_e137818_d_n9, assign89910_e137818_d_n10, assign89910_e137818_d_n11, assign89910_e137818_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2089 != 0.0)) {
        let assign89910_e137810: f64 = (locals.var_q_nsubld__blk2008 * locals.var_ddriftldc);
        let assign89910_e137812: f64 = (assign89910_e137810 * locals.var_ddriftldc);
        let assign89910_e137814: f64 = (assign89910_e137812 / 2.0);
        let assign89910_e137816: f64 = (assign89910_e137814 / 1.034943e-10);
        (assign89910_e137816, (((((locals.var_q_nsubld__blk2008 * locals.var_ddriftldc_dn0) * locals.var_ddriftldc) + (assign89910_e137810 * locals.var_ddriftldc_dn0)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2008 * locals.var_ddriftldc_dn2) * locals.var_ddriftldc) + (assign89910_e137810 * locals.var_ddriftldc_dn2)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2008 * locals.var_ddriftldc_dn4) * locals.var_ddriftldc) + (assign89910_e137810 * locals.var_ddriftldc_dn4)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2008 * locals.var_ddriftldc_dn5) * locals.var_ddriftldc) + (assign89910_e137810 * locals.var_ddriftldc_dn5)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2008 * locals.var_ddriftldc_dn6) * locals.var_ddriftldc) + (assign89910_e137810 * locals.var_ddriftldc_dn6)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2008 * locals.var_ddriftldc_dn7) * locals.var_ddriftldc) + (assign89910_e137810 * locals.var_ddriftldc_dn7)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2008 * locals.var_ddriftldc_dn8) * locals.var_ddriftldc) + (assign89910_e137810 * locals.var_ddriftldc_dn8)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2008 * locals.var_ddriftldc_dn9) * locals.var_ddriftldc) + (assign89910_e137810 * locals.var_ddriftldc_dn9)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2008 * locals.var_ddriftldc_dn10) * locals.var_ddriftldc) + (assign89910_e137810 * locals.var_ddriftldc_dn10)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2008 * locals.var_ddriftldc_dn11) * locals.var_ddriftldc) + (assign89910_e137810 * locals.var_ddriftldc_dn11)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2008 * locals.var_ddriftldc_dn14) * locals.var_ddriftldc) + (assign89910_e137810 * locals.var_ddriftldc_dn14)) / 2.0) / 1.034943e-10),)
    } else {
        (locals.var_dphi_sb, locals.var_dphi_sb_dn0, locals.var_dphi_sb_dn2, locals.var_dphi_sb_dn4, locals.var_dphi_sb_dn5, locals.var_dphi_sb_dn6, locals.var_dphi_sb_dn7, locals.var_dphi_sb_dn8, locals.var_dphi_sb_dn9, locals.var_dphi_sb_dn10, locals.var_dphi_sb_dn11, locals.var_dphi_sb_dn14,)
    }
};
        locals.var_dphi_sb = assign89910_e137818;
        locals.var_dphi_sb_dn0 = assign89910_e137818_d_n0;
        locals.var_dphi_sb_dn2 = assign89910_e137818_d_n2;
        locals.var_dphi_sb_dn4 = assign89910_e137818_d_n4;
        locals.var_dphi_sb_dn5 = assign89910_e137818_d_n5;
        locals.var_dphi_sb_dn6 = assign89910_e137818_d_n6;
        locals.var_dphi_sb_dn7 = assign89910_e137818_d_n7;
        locals.var_dphi_sb_dn8 = assign89910_e137818_d_n8;
        locals.var_dphi_sb_dn9 = assign89910_e137818_d_n9;
        locals.var_dphi_sb_dn10 = assign89910_e137818_d_n10;
        locals.var_dphi_sb_dn11 = assign89910_e137818_d_n11;
        locals.var_dphi_sb_dn14 = assign89910_e137818_d_n14;
        locals.var_dphi_sb_rv = 0.0;

        let (assign89920_e137829, assign89920_e137829_d_n0, assign89920_e137829_d_n2, assign89920_e137829_d_n4, assign89920_e137829_d_n5, assign89920_e137829_d_n6, assign89920_e137829_d_n7, assign89920_e137829_d_n8, assign89920_e137829_d_n9, assign89920_e137829_d_n10, assign89920_e137829_d_n11, assign89920_e137829_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2089 != 0.0)) {
        let assign89920_e137824: f64 = (2.0 * locals.var_beta);
        let assign89920_e137826: f64 = (assign89920_e137824 * locals.var_dphi_sb);
        let assign89920_e137827: f64 = (assign89920_e137826).sqrt();
        (assign89920_e137827, ((((2.0 * locals.var_beta_dn0) * locals.var_dphi_sb) + (assign89920_e137824 * locals.var_dphi_sb_dn0)) / (2.0 * assign89920_e137827)), ((((2.0 * locals.var_beta_dn2) * locals.var_dphi_sb) + (assign89920_e137824 * locals.var_dphi_sb_dn2)) / (2.0 * assign89920_e137827)), ((((2.0 * locals.var_beta_dn4) * locals.var_dphi_sb) + (assign89920_e137824 * locals.var_dphi_sb_dn4)) / (2.0 * assign89920_e137827)), ((((2.0 * locals.var_beta_dn5) * locals.var_dphi_sb) + (assign89920_e137824 * locals.var_dphi_sb_dn5)) / (2.0 * assign89920_e137827)), ((((2.0 * locals.var_beta_dn6) * locals.var_dphi_sb) + (assign89920_e137824 * locals.var_dphi_sb_dn6)) / (2.0 * assign89920_e137827)), ((((2.0 * locals.var_beta_dn7) * locals.var_dphi_sb) + (assign89920_e137824 * locals.var_dphi_sb_dn7)) / (2.0 * assign89920_e137827)), ((((2.0 * locals.var_beta_dn8) * locals.var_dphi_sb) + (assign89920_e137824 * locals.var_dphi_sb_dn8)) / (2.0 * assign89920_e137827)), ((((2.0 * locals.var_beta_dn9) * locals.var_dphi_sb) + (assign89920_e137824 * locals.var_dphi_sb_dn9)) / (2.0 * assign89920_e137827)), ((((2.0 * locals.var_beta_dn10) * locals.var_dphi_sb) + (assign89920_e137824 * locals.var_dphi_sb_dn10)) / (2.0 * assign89920_e137827)), ((((2.0 * locals.var_beta_dn11) * locals.var_dphi_sb) + (assign89920_e137824 * locals.var_dphi_sb_dn11)) / (2.0 * assign89920_e137827)), ((((2.0 * locals.var_beta_dn14) * locals.var_dphi_sb) + (assign89920_e137824 * locals.var_dphi_sb_dn14)) / (2.0 * assign89920_e137827)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign89920_e137829;
        locals.var_t0_dn0 = assign89920_e137829_d_n0;
        locals.var_t0_dn2 = assign89920_e137829_d_n2;
        locals.var_t0_dn4 = assign89920_e137829_d_n4;
        locals.var_t0_dn5 = assign89920_e137829_d_n5;
        locals.var_t0_dn6 = assign89920_e137829_d_n6;
        locals.var_t0_dn7 = assign89920_e137829_d_n7;
        locals.var_t0_dn8 = assign89920_e137829_d_n8;
        locals.var_t0_dn9 = assign89920_e137829_d_n9;
        locals.var_t0_dn10 = assign89920_e137829_d_n10;
        locals.var_t0_dn11 = assign89920_e137829_d_n11;
        locals.var_t0_dn14 = assign89920_e137829_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign89930_e137842, assign89930_e137842_d_n0, assign89930_e137842_d_n2, assign89930_e137842_d_n4, assign89930_e137842_d_n5, assign89930_e137842_d_n6, assign89930_e137842_d_n7, assign89930_e137842_d_n8, assign89930_e137842_d_n9, assign89930_e137842_d_n10, assign89930_e137842_d_n11, assign89930_e137842_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2089 != 0.0)) {
        let assign89930_e137834: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign89930_e137836: f64 = (-locals.var_t0);
        let assign89930_e137837: f64 = { let limited_exp_arg = assign89930_e137836; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign89930_e137838: f64 = (assign89930_e137834 + assign89930_e137837);
        let assign89930_e137840: f64 = (assign89930_e137838 / 2.0);
        (assign89930_e137840, ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn0) + ({ let limited_exp_arg = assign89930_e137836; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn0))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn2) + ({ let limited_exp_arg = assign89930_e137836; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn2))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4) + ({ let limited_exp_arg = assign89930_e137836; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn4))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5) + ({ let limited_exp_arg = assign89930_e137836; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn5))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6) + ({ let limited_exp_arg = assign89930_e137836; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn6))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7) + ({ let limited_exp_arg = assign89930_e137836; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn7))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8) + ({ let limited_exp_arg = assign89930_e137836; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn8))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9) + ({ let limited_exp_arg = assign89930_e137836; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn9))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10) + ({ let limited_exp_arg = assign89930_e137836; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn10))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11) + ({ let limited_exp_arg = assign89930_e137836; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn11))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn14) + ({ let limited_exp_arg = assign89930_e137836; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn14))) / 2.0),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign89930_e137842;
        locals.var_t1_dn0 = assign89930_e137842_d_n0;
        locals.var_t1_dn2 = assign89930_e137842_d_n2;
        locals.var_t1_dn4 = assign89930_e137842_d_n4;
        locals.var_t1_dn5 = assign89930_e137842_d_n5;
        locals.var_t1_dn6 = assign89930_e137842_d_n6;
        locals.var_t1_dn7 = assign89930_e137842_d_n7;
        locals.var_t1_dn8 = assign89930_e137842_d_n8;
        locals.var_t1_dn9 = assign89930_e137842_d_n9;
        locals.var_t1_dn10 = assign89930_e137842_d_n10;
        locals.var_t1_dn11 = assign89930_e137842_d_n11;
        locals.var_t1_dn14 = assign89930_e137842_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign89940_e137851, assign89940_e137851_d_n0, assign89940_e137851_d_n2, assign89940_e137851_d_n4, assign89940_e137851_d_n5, assign89940_e137851_d_n6, assign89940_e137851_d_n7, assign89940_e137851_d_n8, assign89940_e137851_d_n9, assign89940_e137851_d_n10, assign89940_e137851_d_n11, assign89940_e137851_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2089 != 0.0)) {
        let assign89940_e137847: f64 = (locals.var_t1).ln();
        let assign89940_e137849: f64 = (assign89940_e137847 / locals.var_dphi_sb);
        (assign89940_e137849, ((((locals.var_t1_dn0 / locals.var_t1) * locals.var_dphi_sb) - (assign89940_e137847 * locals.var_dphi_sb_dn0)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn2 / locals.var_t1) * locals.var_dphi_sb) - (assign89940_e137847 * locals.var_dphi_sb_dn2)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn4 / locals.var_t1) * locals.var_dphi_sb) - (assign89940_e137847 * locals.var_dphi_sb_dn4)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn5 / locals.var_t1) * locals.var_dphi_sb) - (assign89940_e137847 * locals.var_dphi_sb_dn5)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn6 / locals.var_t1) * locals.var_dphi_sb) - (assign89940_e137847 * locals.var_dphi_sb_dn6)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn7 / locals.var_t1) * locals.var_dphi_sb) - (assign89940_e137847 * locals.var_dphi_sb_dn7)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn8 / locals.var_t1) * locals.var_dphi_sb) - (assign89940_e137847 * locals.var_dphi_sb_dn8)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn9 / locals.var_t1) * locals.var_dphi_sb) - (assign89940_e137847 * locals.var_dphi_sb_dn9)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn10 / locals.var_t1) * locals.var_dphi_sb) - (assign89940_e137847 * locals.var_dphi_sb_dn10)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn11 / locals.var_t1) * locals.var_dphi_sb) - (assign89940_e137847 * locals.var_dphi_sb_dn11)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn14 / locals.var_t1) * locals.var_dphi_sb) - (assign89940_e137847 * locals.var_dphi_sb_dn14)) / (locals.var_dphi_sb * locals.var_dphi_sb)),)
    } else {
        (locals.var_c_sb, locals.var_c_sb_dn0, locals.var_c_sb_dn2, locals.var_c_sb_dn4, locals.var_c_sb_dn5, locals.var_c_sb_dn6, locals.var_c_sb_dn7, locals.var_c_sb_dn8, locals.var_c_sb_dn9, locals.var_c_sb_dn10, locals.var_c_sb_dn11, locals.var_c_sb_dn14,)
    }
};
        locals.var_c_sb = assign89940_e137851;
        locals.var_c_sb_dn0 = assign89940_e137851_d_n0;
        locals.var_c_sb_dn2 = assign89940_e137851_d_n2;
        locals.var_c_sb_dn4 = assign89940_e137851_d_n4;
        locals.var_c_sb_dn5 = assign89940_e137851_d_n5;
        locals.var_c_sb_dn6 = assign89940_e137851_d_n6;
        locals.var_c_sb_dn7 = assign89940_e137851_d_n7;
        locals.var_c_sb_dn8 = assign89940_e137851_d_n8;
        locals.var_c_sb_dn9 = assign89940_e137851_d_n9;
        locals.var_c_sb_dn10 = assign89940_e137851_d_n10;
        locals.var_c_sb_dn11 = assign89940_e137851_d_n11;
        locals.var_c_sb_dn14 = assign89940_e137851_d_n14;
        locals.var_c_sb_rv = 0.0;

        let (assign89950_e137857,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2089 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign89950_e137857;
        locals.var_lp_s0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_346(
        locals: &mut StampLocals,
    ) {
        let mut assign89960_loop_guard: usize = 0;
        while {
            let assign89960_cond_e137864: f64 = (locals.var_lp_s0_max + 1.0);
            let assign89960_cond_e137866: f64 = if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2089 != 0.0)) && (locals.var_lp_s0 <= assign89960_cond_e137864)) { 1.0 } else { 0.0 };
            assign89960_cond_e137866 != 0.0
        } {
            assign89960_loop_guard += 1;
            assert!(assign89960_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign89960_body3_e137893, assign89960_body3_e137893_d_n0, assign89960_body3_e137893_d_n2, assign89960_body3_e137893_d_n4, assign89960_body3_e137893_d_n5, assign89960_body3_e137893_d_n6, assign89960_body3_e137893_d_n7, assign89960_body3_e137893_d_n8, assign89960_body3_e137893_d_n9, assign89960_body3_e137893_d_n10, assign89960_body3_e137893_d_n11, assign89960_body3_e137893_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2089 != 0.0)) {
        let assign89960_body3_e137891: f64 = (locals.var_ps0ld + locals.var_vxbgmtcl);
        (assign89960_body3_e137891, (locals.var_ps0ld_dn0 + locals.var_vxbgmtcl_dn0), (locals.var_ps0ld_dn2 + locals.var_vxbgmtcl_dn2), (locals.var_ps0ld_dn4 + locals.var_vxbgmtcl_dn4), (locals.var_ps0ld_dn5 + locals.var_vxbgmtcl_dn5), (locals.var_ps0ld_dn6 + locals.var_vxbgmtcl_dn6), (locals.var_ps0ld_dn7 + locals.var_vxbgmtcl_dn7), (locals.var_ps0ld_dn8 + locals.var_vxbgmtcl_dn8), (locals.var_ps0ld_dn9 + locals.var_vxbgmtcl_dn9), (locals.var_ps0ld_dn10 + locals.var_vxbgmtcl_dn10), (locals.var_ps0ld_dn11 + locals.var_vxbgmtcl_dn11), (locals.var_ps0ld_dn14 + locals.var_vxbgmtcl_dn14),)
    } else {
        (locals.var_ps0ld_vxb, locals.var_ps0ld_vxb_dn0, locals.var_ps0ld_vxb_dn2, locals.var_ps0ld_vxb_dn4, locals.var_ps0ld_vxb_dn5, locals.var_ps0ld_vxb_dn6, locals.var_ps0ld_vxb_dn7, locals.var_ps0ld_vxb_dn8, locals.var_ps0ld_vxb_dn9, locals.var_ps0ld_vxb_dn10, locals.var_ps0ld_vxb_dn11, locals.var_ps0ld_vxb_dn14,)
    }
};
            locals.var_ps0ld_vxb = assign89960_body3_e137893;
            locals.var_ps0ld_vxb_dn0 = assign89960_body3_e137893_d_n0;
            locals.var_ps0ld_vxb_dn2 = assign89960_body3_e137893_d_n2;
            locals.var_ps0ld_vxb_dn4 = assign89960_body3_e137893_d_n4;
            locals.var_ps0ld_vxb_dn5 = assign89960_body3_e137893_d_n5;
            locals.var_ps0ld_vxb_dn6 = assign89960_body3_e137893_d_n6;
            locals.var_ps0ld_vxb_dn7 = assign89960_body3_e137893_d_n7;
            locals.var_ps0ld_vxb_dn8 = assign89960_body3_e137893_d_n8;
            locals.var_ps0ld_vxb_dn9 = assign89960_body3_e137893_d_n9;
            locals.var_ps0ld_vxb_dn10 = assign89960_body3_e137893_d_n10;
            locals.var_ps0ld_vxb_dn11 = assign89960_body3_e137893_d_n11;
            locals.var_ps0ld_vxb_dn14 = assign89960_body3_e137893_d_n14;
            locals.var_ps0ld_vxb_rv = 0.0;
            let (assign89960_body4_e137901, assign89960_body4_e137901_d_n0, assign89960_body4_e137901_d_n2, assign89960_body4_e137901_d_n4, assign89960_body4_e137901_d_n5, assign89960_body4_e137901_d_n6, assign89960_body4_e137901_d_n7, assign89960_body4_e137901_d_n8, assign89960_body4_e137901_d_n9, assign89960_body4_e137901_d_n10, assign89960_body4_e137901_d_n11, assign89960_body4_e137901_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2089 != 0.0)) {
        let assign89960_body4_e137899: f64 = (locals.var_beta * locals.var_ps0ld_vxb);
        (assign89960_body4_e137899, ((locals.var_beta_dn0 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn0)), ((locals.var_beta_dn2 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn2)), ((locals.var_beta_dn4 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn4)), ((locals.var_beta_dn5 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn5)), ((locals.var_beta_dn6 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn6)), ((locals.var_beta_dn7 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn7)), ((locals.var_beta_dn8 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn8)), ((locals.var_beta_dn9 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn9)), ((locals.var_beta_dn10 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn10)), ((locals.var_beta_dn11 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn11)), ((locals.var_beta_dn14 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn14)),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
            locals.var_chi = assign89960_body4_e137901;
            locals.var_chi_dn0 = assign89960_body4_e137901_d_n0;
            locals.var_chi_dn2 = assign89960_body4_e137901_d_n2;
            locals.var_chi_dn4 = assign89960_body4_e137901_d_n4;
            locals.var_chi_dn5 = assign89960_body4_e137901_d_n5;
            locals.var_chi_dn6 = assign89960_body4_e137901_d_n6;
            locals.var_chi_dn7 = assign89960_body4_e137901_d_n7;
            locals.var_chi_dn8 = assign89960_body4_e137901_d_n8;
            locals.var_chi_dn9 = assign89960_body4_e137901_d_n9;
            locals.var_chi_dn10 = assign89960_body4_e137901_d_n10;
            locals.var_chi_dn11 = assign89960_body4_e137901_d_n11;
            locals.var_chi_dn14 = assign89960_body4_e137901_d_n14;
            locals.var_chi_rv = 0.0;
            let (assign89960_body5_e137911, assign89960_body5_e137911_d_n0, assign89960_body5_e137911_d_n2, assign89960_body5_e137911_d_n4, assign89960_body5_e137911_d_n5, assign89960_body5_e137911_d_n6, assign89960_body5_e137911_d_n7, assign89960_body5_e137911_d_n8, assign89960_body5_e137911_d_n9, assign89960_body5_e137911_d_n10, assign89960_body5_e137911_d_n11, assign89960_body5_e137911_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2089 != 0.0)) {
        let assign89960_body5_e137908: f64 = (locals.var_ps0ld_vxb - locals.var_dphi_sb);
        let assign89960_body5_e137909: f64 = (locals.var_c_sb * assign89960_body5_e137908);
        (assign89960_body5_e137909, ((locals.var_c_sb_dn0 * assign89960_body5_e137908) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn0 - locals.var_dphi_sb_dn0))), ((locals.var_c_sb_dn2 * assign89960_body5_e137908) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn2 - locals.var_dphi_sb_dn2))), ((locals.var_c_sb_dn4 * assign89960_body5_e137908) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn4 - locals.var_dphi_sb_dn4))), ((locals.var_c_sb_dn5 * assign89960_body5_e137908) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn5 - locals.var_dphi_sb_dn5))), ((locals.var_c_sb_dn6 * assign89960_body5_e137908) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn6 - locals.var_dphi_sb_dn6))), ((locals.var_c_sb_dn7 * assign89960_body5_e137908) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn7 - locals.var_dphi_sb_dn7))), ((locals.var_c_sb_dn8 * assign89960_body5_e137908) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn8 - locals.var_dphi_sb_dn8))), ((locals.var_c_sb_dn9 * assign89960_body5_e137908) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn9 - locals.var_dphi_sb_dn9))), ((locals.var_c_sb_dn10 * assign89960_body5_e137908) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn10 - locals.var_dphi_sb_dn10))), ((locals.var_c_sb_dn11 * assign89960_body5_e137908) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn11 - locals.var_dphi_sb_dn11))), ((locals.var_c_sb_dn14 * assign89960_body5_e137908) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn14 - locals.var_dphi_sb_dn14))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn14,)
    }
};
            locals.var_ty = assign89960_body5_e137911;
            locals.var_ty_dn0 = assign89960_body5_e137911_d_n0;
            locals.var_ty_dn2 = assign89960_body5_e137911_d_n2;
            locals.var_ty_dn4 = assign89960_body5_e137911_d_n4;
            locals.var_ty_dn5 = assign89960_body5_e137911_d_n5;
            locals.var_ty_dn6 = assign89960_body5_e137911_d_n6;
            locals.var_ty_dn7 = assign89960_body5_e137911_d_n7;
            locals.var_ty_dn8 = assign89960_body5_e137911_d_n8;
            locals.var_ty_dn9 = assign89960_body5_e137911_d_n9;
            locals.var_ty_dn10 = assign89960_body5_e137911_d_n10;
            locals.var_ty_dn11 = assign89960_body5_e137911_d_n11;
            locals.var_ty_dn14 = assign89960_body5_e137911_d_n14;
            locals.var_ty_rv = 0.0;
            let assign89960_body6_e137914: f64 = if locals.var_ty < 60.0 { 1.0 } else { 0.0 };
            locals.var_guard2094 = assign89960_body6_e137914;
            locals.var_guard2094_rv = 0.0;
            let (assign89960_body7_e137923, assign89960_body7_e137923_d_n0, assign89960_body7_e137923_d_n2, assign89960_body7_e137923_d_n4, assign89960_body7_e137923_d_n5, assign89960_body7_e137923_d_n6, assign89960_body7_e137923_d_n7, assign89960_body7_e137923_d_n8, assign89960_body7_e137923_d_n9, assign89960_body7_e137923_d_n10, assign89960_body7_e137923_d_n11, assign89960_body7_e137923_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2089 != 0.0)) && (locals.var_guard2094 != 0.0)) {
        let assign89960_body7_e137921: f64 = (locals.var_ty).exp();
        (assign89960_body7_e137921, (assign89960_body7_e137921 * locals.var_ty_dn0), (assign89960_body7_e137921 * locals.var_ty_dn2), (assign89960_body7_e137921 * locals.var_ty_dn4), (assign89960_body7_e137921 * locals.var_ty_dn5), (assign89960_body7_e137921 * locals.var_ty_dn6), (assign89960_body7_e137921 * locals.var_ty_dn7), (assign89960_body7_e137921 * locals.var_ty_dn8), (assign89960_body7_e137921 * locals.var_ty_dn9), (assign89960_body7_e137921 * locals.var_ty_dn10), (assign89960_body7_e137921 * locals.var_ty_dn11), (assign89960_body7_e137921 * locals.var_ty_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
            locals.var_t1 = assign89960_body7_e137923;
            locals.var_t1_dn0 = assign89960_body7_e137923_d_n0;
            locals.var_t1_dn2 = assign89960_body7_e137923_d_n2;
            locals.var_t1_dn4 = assign89960_body7_e137923_d_n4;
            locals.var_t1_dn5 = assign89960_body7_e137923_d_n5;
            locals.var_t1_dn6 = assign89960_body7_e137923_d_n6;
            locals.var_t1_dn7 = assign89960_body7_e137923_d_n7;
            locals.var_t1_dn8 = assign89960_body7_e137923_d_n8;
            locals.var_t1_dn9 = assign89960_body7_e137923_d_n9;
            locals.var_t1_dn10 = assign89960_body7_e137923_d_n10;
            locals.var_t1_dn11 = assign89960_body7_e137923_d_n11;
            locals.var_t1_dn14 = assign89960_body7_e137923_d_n14;
            locals.var_t1_rv = 0.0;
            let (assign89960_body8_e137935, assign89960_body8_e137935_d_n0, assign89960_body8_e137935_d_n2, assign89960_body8_e137935_d_n4, assign89960_body8_e137935_d_n5, assign89960_body8_e137935_d_n6, assign89960_body8_e137935_d_n7, assign89960_body8_e137935_d_n8, assign89960_body8_e137935_d_n9, assign89960_body8_e137935_d_n10, assign89960_body8_e137935_d_n11, assign89960_body8_e137935_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2089 != 0.0)) && (locals.var_guard2094 != 0.0)) {
        let assign89960_body8_e137930: f64 = (-locals.var_c_sb);
        let assign89960_body8_e137932: f64 = (assign89960_body8_e137930 * locals.var_dphi_sb);
        let assign89960_body8_e137933: f64 = (assign89960_body8_e137932).exp();
        (assign89960_body8_e137933, (assign89960_body8_e137933 * (((-locals.var_c_sb_dn0) * locals.var_dphi_sb) + (assign89960_body8_e137930 * locals.var_dphi_sb_dn0))), (assign89960_body8_e137933 * (((-locals.var_c_sb_dn2) * locals.var_dphi_sb) + (assign89960_body8_e137930 * locals.var_dphi_sb_dn2))), (assign89960_body8_e137933 * (((-locals.var_c_sb_dn4) * locals.var_dphi_sb) + (assign89960_body8_e137930 * locals.var_dphi_sb_dn4))), (assign89960_body8_e137933 * (((-locals.var_c_sb_dn5) * locals.var_dphi_sb) + (assign89960_body8_e137930 * locals.var_dphi_sb_dn5))), (assign89960_body8_e137933 * (((-locals.var_c_sb_dn6) * locals.var_dphi_sb) + (assign89960_body8_e137930 * locals.var_dphi_sb_dn6))), (assign89960_body8_e137933 * (((-locals.var_c_sb_dn7) * locals.var_dphi_sb) + (assign89960_body8_e137930 * locals.var_dphi_sb_dn7))), (assign89960_body8_e137933 * (((-locals.var_c_sb_dn8) * locals.var_dphi_sb) + (assign89960_body8_e137930 * locals.var_dphi_sb_dn8))), (assign89960_body8_e137933 * (((-locals.var_c_sb_dn9) * locals.var_dphi_sb) + (assign89960_body8_e137930 * locals.var_dphi_sb_dn9))), (assign89960_body8_e137933 * (((-locals.var_c_sb_dn10) * locals.var_dphi_sb) + (assign89960_body8_e137930 * locals.var_dphi_sb_dn10))), (assign89960_body8_e137933 * (((-locals.var_c_sb_dn11) * locals.var_dphi_sb) + (assign89960_body8_e137930 * locals.var_dphi_sb_dn11))), (assign89960_body8_e137933 * (((-locals.var_c_sb_dn14) * locals.var_dphi_sb) + (assign89960_body8_e137930 * locals.var_dphi_sb_dn14))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
            locals.var_t0 = assign89960_body8_e137935;
            locals.var_t0_dn0 = assign89960_body8_e137935_d_n0;
            locals.var_t0_dn2 = assign89960_body8_e137935_d_n2;
            locals.var_t0_dn4 = assign89960_body8_e137935_d_n4;
            locals.var_t0_dn5 = assign89960_body8_e137935_d_n5;
            locals.var_t0_dn6 = assign89960_body8_e137935_d_n6;
            locals.var_t0_dn7 = assign89960_body8_e137935_d_n7;
            locals.var_t0_dn8 = assign89960_body8_e137935_d_n8;
            locals.var_t0_dn9 = assign89960_body8_e137935_d_n9;
            locals.var_t0_dn10 = assign89960_body8_e137935_d_n10;
            locals.var_t0_dn11 = assign89960_body8_e137935_d_n11;
            locals.var_t0_dn14 = assign89960_body8_e137935_d_n14;
            locals.var_t0_rv = 0.0;
            let (assign89960_body9_e137945, assign89960_body9_e137945_d_n0, assign89960_body9_e137945_d_n2, assign89960_body9_e137945_d_n4, assign89960_body9_e137945_d_n5, assign89960_body9_e137945_d_n6, assign89960_body9_e137945_d_n7, assign89960_body9_e137945_d_n8, assign89960_body9_e137945_d_n9, assign89960_body9_e137945_d_n10, assign89960_body9_e137945_d_n11, assign89960_body9_e137945_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2089 != 0.0)) && (locals.var_guard2094 != 0.0)) {
        let assign89960_body9_e137943: f64 = (locals.var_t1 - locals.var_t0);
        (assign89960_body9_e137943, (locals.var_t1_dn0 - locals.var_t0_dn0), (locals.var_t1_dn2 - locals.var_t0_dn2), (locals.var_t1_dn4 - locals.var_t0_dn4), (locals.var_t1_dn5 - locals.var_t0_dn5), (locals.var_t1_dn6 - locals.var_t0_dn6), (locals.var_t1_dn7 - locals.var_t0_dn7), (locals.var_t1_dn8 - locals.var_t0_dn8), (locals.var_t1_dn9 - locals.var_t0_dn9), (locals.var_t1_dn10 - locals.var_t0_dn10), (locals.var_t1_dn11 - locals.var_t0_dn11), (locals.var_t1_dn14 - locals.var_t0_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
            locals.var_t2 = assign89960_body9_e137945;
            locals.var_t2_dn0 = assign89960_body9_e137945_d_n0;
            locals.var_t2_dn2 = assign89960_body9_e137945_d_n2;
            locals.var_t2_dn4 = assign89960_body9_e137945_d_n4;
            locals.var_t2_dn5 = assign89960_body9_e137945_d_n5;
            locals.var_t2_dn6 = assign89960_body9_e137945_d_n6;
            locals.var_t2_dn7 = assign89960_body9_e137945_d_n7;
            locals.var_t2_dn8 = assign89960_body9_e137945_d_n8;
            locals.var_t2_dn9 = assign89960_body9_e137945_d_n9;
            locals.var_t2_dn10 = assign89960_body9_e137945_d_n10;
            locals.var_t2_dn11 = assign89960_body9_e137945_d_n11;
            locals.var_t2_dn14 = assign89960_body9_e137945_d_n14;
            locals.var_t2_rv = 0.0;
            let (assign89960_body10_e137958, assign89960_body10_e137958_d_n0, assign89960_body10_e137958_d_n2, assign89960_body10_e137958_d_n4, assign89960_body10_e137958_d_n5, assign89960_body10_e137958_d_n6, assign89960_body10_e137958_d_n7, assign89960_body10_e137958_d_n8, assign89960_body10_e137958_d_n9, assign89960_body10_e137958_d_n10, assign89960_body10_e137958_d_n11, assign89960_body10_e137958_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2089 != 0.0)) && (locals.var_guard2094 != 0.0)) {
        let assign89960_body10_e137953: f64 = (1.0 + locals.var_t2);
        let assign89960_body10_e137954: f64 = (assign89960_body10_e137953).ln();
        let assign89960_body10_e137956: f64 = (assign89960_body10_e137954 / locals.var_c_sb);
        (assign89960_body10_e137956, ((((locals.var_t2_dn0 / assign89960_body10_e137953) * locals.var_c_sb) - (assign89960_body10_e137954 * locals.var_c_sb_dn0)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn2 / assign89960_body10_e137953) * locals.var_c_sb) - (assign89960_body10_e137954 * locals.var_c_sb_dn2)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn4 / assign89960_body10_e137953) * locals.var_c_sb) - (assign89960_body10_e137954 * locals.var_c_sb_dn4)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn5 / assign89960_body10_e137953) * locals.var_c_sb) - (assign89960_body10_e137954 * locals.var_c_sb_dn5)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn6 / assign89960_body10_e137953) * locals.var_c_sb) - (assign89960_body10_e137954 * locals.var_c_sb_dn6)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn7 / assign89960_body10_e137953) * locals.var_c_sb) - (assign89960_body10_e137954 * locals.var_c_sb_dn7)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn8 / assign89960_body10_e137953) * locals.var_c_sb) - (assign89960_body10_e137954 * locals.var_c_sb_dn8)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn9 / assign89960_body10_e137953) * locals.var_c_sb) - (assign89960_body10_e137954 * locals.var_c_sb_dn9)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn10 / assign89960_body10_e137953) * locals.var_c_sb) - (assign89960_body10_e137954 * locals.var_c_sb_dn10)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn11 / assign89960_body10_e137953) * locals.var_c_sb) - (assign89960_body10_e137954 * locals.var_c_sb_dn11)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn14 / assign89960_body10_e137953) * locals.var_c_sb) - (assign89960_body10_e137954 * locals.var_c_sb_dn14)) / (locals.var_c_sb * locals.var_c_sb)),)
    } else {
        (locals.var_phi_b, locals.var_phi_b_dn0, locals.var_phi_b_dn2, locals.var_phi_b_dn4, locals.var_phi_b_dn5, locals.var_phi_b_dn6, locals.var_phi_b_dn7, locals.var_phi_b_dn8, locals.var_phi_b_dn9, locals.var_phi_b_dn10, locals.var_phi_b_dn11, locals.var_phi_b_dn14,)
    }
};
            locals.var_phi_b = assign89960_body10_e137958;
            locals.var_phi_b_dn0 = assign89960_body10_e137958_d_n0;
            locals.var_phi_b_dn2 = assign89960_body10_e137958_d_n2;
            locals.var_phi_b_dn4 = assign89960_body10_e137958_d_n4;
            locals.var_phi_b_dn5 = assign89960_body10_e137958_d_n5;
            locals.var_phi_b_dn6 = assign89960_body10_e137958_d_n6;
            locals.var_phi_b_dn7 = assign89960_body10_e137958_d_n7;
            locals.var_phi_b_dn8 = assign89960_body10_e137958_d_n8;
            locals.var_phi_b_dn9 = assign89960_body10_e137958_d_n9;
            locals.var_phi_b_dn10 = assign89960_body10_e137958_d_n10;
            locals.var_phi_b_dn11 = assign89960_body10_e137958_d_n11;
            locals.var_phi_b_dn14 = assign89960_body10_e137958_d_n14;
            locals.var_phi_b_rv = 0.0;
            let (assign89960_body11_e137970, assign89960_body11_e137970_d_n0, assign89960_body11_e137970_d_n2, assign89960_body11_e137970_d_n4, assign89960_body11_e137970_d_n5, assign89960_body11_e137970_d_n6, assign89960_body11_e137970_d_n7, assign89960_body11_e137970_d_n8, assign89960_body11_e137970_d_n9, assign89960_body11_e137970_d_n10, assign89960_body11_e137970_d_n11, assign89960_body11_e137970_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2089 != 0.0)) && (locals.var_guard2094 != 0.0)) {
        let assign89960_body11_e137967: f64 = (1.0 + locals.var_t2);
        let assign89960_body11_e137968: f64 = (locals.var_t1 / assign89960_body11_e137967);
        (assign89960_body11_e137968, (((locals.var_t1_dn0 * assign89960_body11_e137967) - (locals.var_t1 * locals.var_t2_dn0)) / (assign89960_body11_e137967 * assign89960_body11_e137967)), (((locals.var_t1_dn2 * assign89960_body11_e137967) - (locals.var_t1 * locals.var_t2_dn2)) / (assign89960_body11_e137967 * assign89960_body11_e137967)), (((locals.var_t1_dn4 * assign89960_body11_e137967) - (locals.var_t1 * locals.var_t2_dn4)) / (assign89960_body11_e137967 * assign89960_body11_e137967)), (((locals.var_t1_dn5 * assign89960_body11_e137967) - (locals.var_t1 * locals.var_t2_dn5)) / (assign89960_body11_e137967 * assign89960_body11_e137967)), (((locals.var_t1_dn6 * assign89960_body11_e137967) - (locals.var_t1 * locals.var_t2_dn6)) / (assign89960_body11_e137967 * assign89960_body11_e137967)), (((locals.var_t1_dn7 * assign89960_body11_e137967) - (locals.var_t1 * locals.var_t2_dn7)) / (assign89960_body11_e137967 * assign89960_body11_e137967)), (((locals.var_t1_dn8 * assign89960_body11_e137967) - (locals.var_t1 * locals.var_t2_dn8)) / (assign89960_body11_e137967 * assign89960_body11_e137967)), (((locals.var_t1_dn9 * assign89960_body11_e137967) - (locals.var_t1 * locals.var_t2_dn9)) / (assign89960_body11_e137967 * assign89960_body11_e137967)), (((locals.var_t1_dn10 * assign89960_body11_e137967) - (locals.var_t1 * locals.var_t2_dn10)) / (assign89960_body11_e137967 * assign89960_body11_e137967)), (((locals.var_t1_dn11 * assign89960_body11_e137967) - (locals.var_t1 * locals.var_t2_dn11)) / (assign89960_body11_e137967 * assign89960_body11_e137967)), (((locals.var_t1_dn14 * assign89960_body11_e137967) - (locals.var_t1 * locals.var_t2_dn14)) / (assign89960_body11_e137967 * assign89960_body11_e137967)),)
    } else {
        (locals.var_phi_b_dpss, locals.var_phi_b_dpss_dn0, locals.var_phi_b_dpss_dn2, locals.var_phi_b_dpss_dn4, locals.var_phi_b_dpss_dn5, locals.var_phi_b_dpss_dn6, locals.var_phi_b_dpss_dn7, locals.var_phi_b_dpss_dn8, locals.var_phi_b_dpss_dn9, locals.var_phi_b_dpss_dn10, locals.var_phi_b_dpss_dn11, locals.var_phi_b_dpss_dn14,)
    }
};
            locals.var_phi_b_dpss = assign89960_body11_e137970;
            locals.var_phi_b_dpss_dn0 = assign89960_body11_e137970_d_n0;
            locals.var_phi_b_dpss_dn2 = assign89960_body11_e137970_d_n2;
            locals.var_phi_b_dpss_dn4 = assign89960_body11_e137970_d_n4;
            locals.var_phi_b_dpss_dn5 = assign89960_body11_e137970_d_n5;
            locals.var_phi_b_dpss_dn6 = assign89960_body11_e137970_d_n6;
            locals.var_phi_b_dpss_dn7 = assign89960_body11_e137970_d_n7;
            locals.var_phi_b_dpss_dn8 = assign89960_body11_e137970_d_n8;
            locals.var_phi_b_dpss_dn9 = assign89960_body11_e137970_d_n9;
            locals.var_phi_b_dpss_dn10 = assign89960_body11_e137970_d_n10;
            locals.var_phi_b_dpss_dn11 = assign89960_body11_e137970_d_n11;
            locals.var_phi_b_dpss_dn14 = assign89960_body11_e137970_d_n14;
            locals.var_phi_b_dpss_rv = 0.0;
            let (assign89960_body12_e137981, assign89960_body12_e137981_d_n0, assign89960_body12_e137981_d_n2, assign89960_body12_e137981_d_n4, assign89960_body12_e137981_d_n5, assign89960_body12_e137981_d_n6, assign89960_body12_e137981_d_n7, assign89960_body12_e137981_d_n8, assign89960_body12_e137981_d_n9, assign89960_body12_e137981_d_n10, assign89960_body12_e137981_d_n11, assign89960_body12_e137981_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2089 != 0.0)) && (locals.var_guard2094 == 0.0)) {
        let assign89960_body12_e137979: f64 = (locals.var_ps0ld_vxb - locals.var_dphi_sb);
        (assign89960_body12_e137979, (locals.var_ps0ld_vxb_dn0 - locals.var_dphi_sb_dn0), (locals.var_ps0ld_vxb_dn2 - locals.var_dphi_sb_dn2), (locals.var_ps0ld_vxb_dn4 - locals.var_dphi_sb_dn4), (locals.var_ps0ld_vxb_dn5 - locals.var_dphi_sb_dn5), (locals.var_ps0ld_vxb_dn6 - locals.var_dphi_sb_dn6), (locals.var_ps0ld_vxb_dn7 - locals.var_dphi_sb_dn7), (locals.var_ps0ld_vxb_dn8 - locals.var_dphi_sb_dn8), (locals.var_ps0ld_vxb_dn9 - locals.var_dphi_sb_dn9), (locals.var_ps0ld_vxb_dn10 - locals.var_dphi_sb_dn10), (locals.var_ps0ld_vxb_dn11 - locals.var_dphi_sb_dn11), (locals.var_ps0ld_vxb_dn14 - locals.var_dphi_sb_dn14),)
    } else {
        (locals.var_phi_b, locals.var_phi_b_dn0, locals.var_phi_b_dn2, locals.var_phi_b_dn4, locals.var_phi_b_dn5, locals.var_phi_b_dn6, locals.var_phi_b_dn7, locals.var_phi_b_dn8, locals.var_phi_b_dn9, locals.var_phi_b_dn10, locals.var_phi_b_dn11, locals.var_phi_b_dn14,)
    }
};
            locals.var_phi_b = assign89960_body12_e137981;
            locals.var_phi_b_dn0 = assign89960_body12_e137981_d_n0;
            locals.var_phi_b_dn2 = assign89960_body12_e137981_d_n2;
            locals.var_phi_b_dn4 = assign89960_body12_e137981_d_n4;
            locals.var_phi_b_dn5 = assign89960_body12_e137981_d_n5;
            locals.var_phi_b_dn6 = assign89960_body12_e137981_d_n6;
            locals.var_phi_b_dn7 = assign89960_body12_e137981_d_n7;
            locals.var_phi_b_dn8 = assign89960_body12_e137981_d_n8;
            locals.var_phi_b_dn9 = assign89960_body12_e137981_d_n9;
            locals.var_phi_b_dn10 = assign89960_body12_e137981_d_n10;
            locals.var_phi_b_dn11 = assign89960_body12_e137981_d_n11;
            locals.var_phi_b_dn14 = assign89960_body12_e137981_d_n14;
            locals.var_phi_b_rv = 0.0;
            let (assign89960_body13_e137990, assign89960_body13_e137990_d_n0, assign89960_body13_e137990_d_n2, assign89960_body13_e137990_d_n4, assign89960_body13_e137990_d_n5, assign89960_body13_e137990_d_n6, assign89960_body13_e137990_d_n7, assign89960_body13_e137990_d_n8, assign89960_body13_e137990_d_n9, assign89960_body13_e137990_d_n10, assign89960_body13_e137990_d_n11, assign89960_body13_e137990_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2089 != 0.0)) && (locals.var_guard2094 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phi_b_dpss, locals.var_phi_b_dpss_dn0, locals.var_phi_b_dpss_dn2, locals.var_phi_b_dpss_dn4, locals.var_phi_b_dpss_dn5, locals.var_phi_b_dpss_dn6, locals.var_phi_b_dpss_dn7, locals.var_phi_b_dpss_dn8, locals.var_phi_b_dpss_dn9, locals.var_phi_b_dpss_dn10, locals.var_phi_b_dpss_dn11, locals.var_phi_b_dpss_dn14,)
    }
};
            locals.var_phi_b_dpss = assign89960_body13_e137990;
            locals.var_phi_b_dpss_dn0 = assign89960_body13_e137990_d_n0;
            locals.var_phi_b_dpss_dn2 = assign89960_body13_e137990_d_n2;
            locals.var_phi_b_dpss_dn4 = assign89960_body13_e137990_d_n4;
            locals.var_phi_b_dpss_dn5 = assign89960_body13_e137990_d_n5;
            locals.var_phi_b_dpss_dn6 = assign89960_body13_e137990_d_n6;
            locals.var_phi_b_dpss_dn7 = assign89960_body13_e137990_d_n7;
            locals.var_phi_b_dpss_dn8 = assign89960_body13_e137990_d_n8;
            locals.var_phi_b_dpss_dn9 = assign89960_body13_e137990_d_n9;
            locals.var_phi_b_dpss_dn10 = assign89960_body13_e137990_d_n10;
            locals.var_phi_b_dpss_dn11 = assign89960_body13_e137990_d_n11;
            locals.var_phi_b_dpss_dn14 = assign89960_body13_e137990_d_n14;
            locals.var_phi_b_dpss_rv = 0.0;
            let (assign89960_body14_e137998, assign89960_body14_e137998_d_n0, assign89960_body14_e137998_d_n2, assign89960_body14_e137998_d_n4, assign89960_body14_e137998_d_n5, assign89960_body14_e137998_d_n6, assign89960_body14_e137998_d_n7, assign89960_body14_e137998_d_n8, assign89960_body14_e137998_d_n9, assign89960_body14_e137998_d_n10, assign89960_body14_e137998_d_n11, assign89960_body14_e137998_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2089 != 0.0)) {
        let assign89960_body14_e137996: f64 = (locals.var_beta * locals.var_phi_b);
        (assign89960_body14_e137996, ((locals.var_beta_dn0 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn0)), ((locals.var_beta_dn2 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn2)), ((locals.var_beta_dn4 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn4)), ((locals.var_beta_dn5 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn5)), ((locals.var_beta_dn6 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn6)), ((locals.var_beta_dn7 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn7)), ((locals.var_beta_dn8 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn8)), ((locals.var_beta_dn9 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn9)), ((locals.var_beta_dn10 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn10)), ((locals.var_beta_dn11 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn11)), ((locals.var_beta_dn14 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn14)),)
    } else {
        (locals.var_chib, locals.var_chib_dn0, locals.var_chib_dn2, locals.var_chib_dn4, locals.var_chib_dn5, locals.var_chib_dn6, locals.var_chib_dn7, locals.var_chib_dn8, locals.var_chib_dn9, locals.var_chib_dn10, locals.var_chib_dn11, locals.var_chib_dn14,)
    }
};
            locals.var_chib = assign89960_body14_e137998;
            locals.var_chib_dn0 = assign89960_body14_e137998_d_n0;
            locals.var_chib_dn2 = assign89960_body14_e137998_d_n2;
            locals.var_chib_dn4 = assign89960_body14_e137998_d_n4;
            locals.var_chib_dn5 = assign89960_body14_e137998_d_n5;
            locals.var_chib_dn6 = assign89960_body14_e137998_d_n6;
            locals.var_chib_dn7 = assign89960_body14_e137998_d_n7;
            locals.var_chib_dn8 = assign89960_body14_e137998_d_n8;
            locals.var_chib_dn9 = assign89960_body14_e137998_d_n9;
            locals.var_chib_dn10 = assign89960_body14_e137998_d_n10;
            locals.var_chib_dn11 = assign89960_body14_e137998_d_n11;
            locals.var_chib_dn14 = assign89960_body14_e137998_d_n14;
            locals.var_chib_rv = 0.0;
            let assign89960_body15_e138000: f64 = (locals.var_chi).abs();
            let assign89960_body15_e138002: f64 = if assign89960_body15_e138000 < 1e-6 { 1.0 } else { 0.0 };
            locals.var_guard2095 = assign89960_body15_e138002;
            locals.var_guard2095_rv = 0.0;
            let (assign89960_body17_e138048, assign89960_body17_e138048_d_n0, assign89960_body17_e138048_d_n2, assign89960_body17_e138048_d_n4, assign89960_body17_e138048_d_n5, assign89960_body17_e138048_d_n6, assign89960_body17_e138048_d_n7, assign89960_body17_e138048_d_n8, assign89960_body17_e138048_d_n9, assign89960_body17_e138048_d_n10, assign89960_body17_e138048_d_n11, assign89960_body17_e138048_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2089 != 0.0)) && (locals.var_guard2095 != 0.0)) {
        let assign89960_body17_e138026: f64 = (locals.var_chi * locals.var_chi);
        let assign89960_body17_e138028: f64 = (assign89960_body17_e138026 / 2.0);
        let assign89960_body17_e138032: f64 = (locals.var_chi / 3.0);
        let assign89960_body17_e138036: f64 = (locals.var_chi / 4.0);
        let assign89960_body17_e138040: f64 = (locals.var_chi / 5.0);
        let assign89960_body17_e138041: f64 = (1.0 - assign89960_body17_e138040);
        let assign89960_body17_e138042: f64 = (assign89960_body17_e138036 * assign89960_body17_e138041);
        let assign89960_body17_e138043: f64 = (1.0 - assign89960_body17_e138042);
        let assign89960_body17_e138044: f64 = (assign89960_body17_e138032 * assign89960_body17_e138043);
        let assign89960_body17_e138045: f64 = (1.0 - assign89960_body17_e138044);
        let assign89960_body17_e138046: f64 = (assign89960_body17_e138028 * assign89960_body17_e138045);
        (assign89960_body17_e138046, (((((locals.var_chi_dn0 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn0)) / 2.0) * assign89960_body17_e138045) + (assign89960_body17_e138028 * (-(((locals.var_chi_dn0 / 3.0) * assign89960_body17_e138043) + (assign89960_body17_e138032 * (-(((locals.var_chi_dn0 / 4.0) * assign89960_body17_e138041) + (assign89960_body17_e138036 * (-(locals.var_chi_dn0 / 5.0)))))))))), (((((locals.var_chi_dn2 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn2)) / 2.0) * assign89960_body17_e138045) + (assign89960_body17_e138028 * (-(((locals.var_chi_dn2 / 3.0) * assign89960_body17_e138043) + (assign89960_body17_e138032 * (-(((locals.var_chi_dn2 / 4.0) * assign89960_body17_e138041) + (assign89960_body17_e138036 * (-(locals.var_chi_dn2 / 5.0)))))))))), (((((locals.var_chi_dn4 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn4)) / 2.0) * assign89960_body17_e138045) + (assign89960_body17_e138028 * (-(((locals.var_chi_dn4 / 3.0) * assign89960_body17_e138043) + (assign89960_body17_e138032 * (-(((locals.var_chi_dn4 / 4.0) * assign89960_body17_e138041) + (assign89960_body17_e138036 * (-(locals.var_chi_dn4 / 5.0)))))))))), (((((locals.var_chi_dn5 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn5)) / 2.0) * assign89960_body17_e138045) + (assign89960_body17_e138028 * (-(((locals.var_chi_dn5 / 3.0) * assign89960_body17_e138043) + (assign89960_body17_e138032 * (-(((locals.var_chi_dn5 / 4.0) * assign89960_body17_e138041) + (assign89960_body17_e138036 * (-(locals.var_chi_dn5 / 5.0)))))))))), (((((locals.var_chi_dn6 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn6)) / 2.0) * assign89960_body17_e138045) + (assign89960_body17_e138028 * (-(((locals.var_chi_dn6 / 3.0) * assign89960_body17_e138043) + (assign89960_body17_e138032 * (-(((locals.var_chi_dn6 / 4.0) * assign89960_body17_e138041) + (assign89960_body17_e138036 * (-(locals.var_chi_dn6 / 5.0)))))))))), (((((locals.var_chi_dn7 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn7)) / 2.0) * assign89960_body17_e138045) + (assign89960_body17_e138028 * (-(((locals.var_chi_dn7 / 3.0) * assign89960_body17_e138043) + (assign89960_body17_e138032 * (-(((locals.var_chi_dn7 / 4.0) * assign89960_body17_e138041) + (assign89960_body17_e138036 * (-(locals.var_chi_dn7 / 5.0)))))))))), (((((locals.var_chi_dn8 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn8)) / 2.0) * assign89960_body17_e138045) + (assign89960_body17_e138028 * (-(((locals.var_chi_dn8 / 3.0) * assign89960_body17_e138043) + (assign89960_body17_e138032 * (-(((locals.var_chi_dn8 / 4.0) * assign89960_body17_e138041) + (assign89960_body17_e138036 * (-(locals.var_chi_dn8 / 5.0)))))))))), (((((locals.var_chi_dn9 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn9)) / 2.0) * assign89960_body17_e138045) + (assign89960_body17_e138028 * (-(((locals.var_chi_dn9 / 3.0) * assign89960_body17_e138043) + (assign89960_body17_e138032 * (-(((locals.var_chi_dn9 / 4.0) * assign89960_body17_e138041) + (assign89960_body17_e138036 * (-(locals.var_chi_dn9 / 5.0)))))))))), (((((locals.var_chi_dn10 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn10)) / 2.0) * assign89960_body17_e138045) + (assign89960_body17_e138028 * (-(((locals.var_chi_dn10 / 3.0) * assign89960_body17_e138043) + (assign89960_body17_e138032 * (-(((locals.var_chi_dn10 / 4.0) * assign89960_body17_e138041) + (assign89960_body17_e138036 * (-(locals.var_chi_dn10 / 5.0)))))))))), (((((locals.var_chi_dn11 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn11)) / 2.0) * assign89960_body17_e138045) + (assign89960_body17_e138028 * (-(((locals.var_chi_dn11 / 3.0) * assign89960_body17_e138043) + (assign89960_body17_e138032 * (-(((locals.var_chi_dn11 / 4.0) * assign89960_body17_e138041) + (assign89960_body17_e138036 * (-(locals.var_chi_dn11 / 5.0)))))))))), (((((locals.var_chi_dn14 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn14)) / 2.0) * assign89960_body17_e138045) + (assign89960_body17_e138028 * (-(((locals.var_chi_dn14 / 3.0) * assign89960_body17_e138043) + (assign89960_body17_e138032 * (-(((locals.var_chi_dn14 / 4.0) * assign89960_body17_e138041) + (assign89960_body17_e138036 * (-(locals.var_chi_dn14 / 5.0)))))))))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
            locals.var_t0 = assign89960_body17_e138048;
            locals.var_t0_dn0 = assign89960_body17_e138048_d_n0;
            locals.var_t0_dn2 = assign89960_body17_e138048_d_n2;
            locals.var_t0_dn4 = assign89960_body17_e138048_d_n4;
            locals.var_t0_dn5 = assign89960_body17_e138048_d_n5;
            locals.var_t0_dn6 = assign89960_body17_e138048_d_n6;
            locals.var_t0_dn7 = assign89960_body17_e138048_d_n7;
            locals.var_t0_dn8 = assign89960_body17_e138048_d_n8;
            locals.var_t0_dn9 = assign89960_body17_e138048_d_n9;
            locals.var_t0_dn10 = assign89960_body17_e138048_d_n10;
            locals.var_t0_dn11 = assign89960_body17_e138048_d_n11;
            locals.var_t0_dn14 = assign89960_body17_e138048_d_n14;
            locals.var_t0_rv = 0.0;
            let (assign89960_body18_e138074, assign89960_body18_e138074_d_n0, assign89960_body18_e138074_d_n2, assign89960_body18_e138074_d_n4, assign89960_body18_e138074_d_n5, assign89960_body18_e138074_d_n6, assign89960_body18_e138074_d_n7, assign89960_body18_e138074_d_n8, assign89960_body18_e138074_d_n9, assign89960_body18_e138074_d_n10, assign89960_body18_e138074_d_n11, assign89960_body18_e138074_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2089 != 0.0)) && (locals.var_guard2095 != 0.0)) {
        let assign89960_body18_e138058: f64 = (locals.var_chi / 2.0);
        let assign89960_body18_e138062: f64 = (locals.var_chi / 3.0);
        let assign89960_body18_e138066: f64 = (locals.var_chi / 4.0);
        let assign89960_body18_e138067: f64 = (1.0 - assign89960_body18_e138066);
        let assign89960_body18_e138068: f64 = (assign89960_body18_e138062 * assign89960_body18_e138067);
        let assign89960_body18_e138069: f64 = (1.0 - assign89960_body18_e138068);
        let assign89960_body18_e138070: f64 = (assign89960_body18_e138058 * assign89960_body18_e138069);
        let assign89960_body18_e138071: f64 = (1.0 - assign89960_body18_e138070);
        let assign89960_body18_e138072: f64 = (locals.var_chi * assign89960_body18_e138071);
        (assign89960_body18_e138072, ((locals.var_chi_dn0 * assign89960_body18_e138071) + (locals.var_chi * (-(((locals.var_chi_dn0 / 2.0) * assign89960_body18_e138069) + (assign89960_body18_e138058 * (-(((locals.var_chi_dn0 / 3.0) * assign89960_body18_e138067) + (assign89960_body18_e138062 * (-(locals.var_chi_dn0 / 4.0)))))))))), ((locals.var_chi_dn2 * assign89960_body18_e138071) + (locals.var_chi * (-(((locals.var_chi_dn2 / 2.0) * assign89960_body18_e138069) + (assign89960_body18_e138058 * (-(((locals.var_chi_dn2 / 3.0) * assign89960_body18_e138067) + (assign89960_body18_e138062 * (-(locals.var_chi_dn2 / 4.0)))))))))), ((locals.var_chi_dn4 * assign89960_body18_e138071) + (locals.var_chi * (-(((locals.var_chi_dn4 / 2.0) * assign89960_body18_e138069) + (assign89960_body18_e138058 * (-(((locals.var_chi_dn4 / 3.0) * assign89960_body18_e138067) + (assign89960_body18_e138062 * (-(locals.var_chi_dn4 / 4.0)))))))))), ((locals.var_chi_dn5 * assign89960_body18_e138071) + (locals.var_chi * (-(((locals.var_chi_dn5 / 2.0) * assign89960_body18_e138069) + (assign89960_body18_e138058 * (-(((locals.var_chi_dn5 / 3.0) * assign89960_body18_e138067) + (assign89960_body18_e138062 * (-(locals.var_chi_dn5 / 4.0)))))))))), ((locals.var_chi_dn6 * assign89960_body18_e138071) + (locals.var_chi * (-(((locals.var_chi_dn6 / 2.0) * assign89960_body18_e138069) + (assign89960_body18_e138058 * (-(((locals.var_chi_dn6 / 3.0) * assign89960_body18_e138067) + (assign89960_body18_e138062 * (-(locals.var_chi_dn6 / 4.0)))))))))), ((locals.var_chi_dn7 * assign89960_body18_e138071) + (locals.var_chi * (-(((locals.var_chi_dn7 / 2.0) * assign89960_body18_e138069) + (assign89960_body18_e138058 * (-(((locals.var_chi_dn7 / 3.0) * assign89960_body18_e138067) + (assign89960_body18_e138062 * (-(locals.var_chi_dn7 / 4.0)))))))))), ((locals.var_chi_dn8 * assign89960_body18_e138071) + (locals.var_chi * (-(((locals.var_chi_dn8 / 2.0) * assign89960_body18_e138069) + (assign89960_body18_e138058 * (-(((locals.var_chi_dn8 / 3.0) * assign89960_body18_e138067) + (assign89960_body18_e138062 * (-(locals.var_chi_dn8 / 4.0)))))))))), ((locals.var_chi_dn9 * assign89960_body18_e138071) + (locals.var_chi * (-(((locals.var_chi_dn9 / 2.0) * assign89960_body18_e138069) + (assign89960_body18_e138058 * (-(((locals.var_chi_dn9 / 3.0) * assign89960_body18_e138067) + (assign89960_body18_e138062 * (-(locals.var_chi_dn9 / 4.0)))))))))), ((locals.var_chi_dn10 * assign89960_body18_e138071) + (locals.var_chi * (-(((locals.var_chi_dn10 / 2.0) * assign89960_body18_e138069) + (assign89960_body18_e138058 * (-(((locals.var_chi_dn10 / 3.0) * assign89960_body18_e138067) + (assign89960_body18_e138062 * (-(locals.var_chi_dn10 / 4.0)))))))))), ((locals.var_chi_dn11 * assign89960_body18_e138071) + (locals.var_chi * (-(((locals.var_chi_dn11 / 2.0) * assign89960_body18_e138069) + (assign89960_body18_e138058 * (-(((locals.var_chi_dn11 / 3.0) * assign89960_body18_e138067) + (assign89960_body18_e138062 * (-(locals.var_chi_dn11 / 4.0)))))))))), ((locals.var_chi_dn14 * assign89960_body18_e138071) + (locals.var_chi * (-(((locals.var_chi_dn14 / 2.0) * assign89960_body18_e138069) + (assign89960_body18_e138058 * (-(((locals.var_chi_dn14 / 3.0) * assign89960_body18_e138067) + (assign89960_body18_e138062 * (-(locals.var_chi_dn14 / 4.0)))))))))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
            locals.var_t1 = assign89960_body18_e138074;
            locals.var_t1_dn0 = assign89960_body18_e138074_d_n0;
            locals.var_t1_dn2 = assign89960_body18_e138074_d_n2;
            locals.var_t1_dn4 = assign89960_body18_e138074_d_n4;
            locals.var_t1_dn5 = assign89960_body18_e138074_d_n5;
            locals.var_t1_dn6 = assign89960_body18_e138074_d_n6;
            locals.var_t1_dn7 = assign89960_body18_e138074_d_n7;
            locals.var_t1_dn8 = assign89960_body18_e138074_d_n8;
            locals.var_t1_dn9 = assign89960_body18_e138074_d_n9;
            locals.var_t1_dn10 = assign89960_body18_e138074_d_n10;
            locals.var_t1_dn11 = assign89960_body18_e138074_d_n11;
            locals.var_t1_dn14 = assign89960_body18_e138074_d_n14;
            locals.var_t1_rv = 0.0;
            let (assign89960_body19_e138104, assign89960_body19_e138104_d_n0, assign89960_body19_e138104_d_n2, assign89960_body19_e138104_d_n4, assign89960_body19_e138104_d_n5, assign89960_body19_e138104_d_n6, assign89960_body19_e138104_d_n7, assign89960_body19_e138104_d_n8, assign89960_body19_e138104_d_n9, assign89960_body19_e138104_d_n10, assign89960_body19_e138104_d_n11, assign89960_body19_e138104_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2089 != 0.0)) && (locals.var_guard2095 != 0.0)) {
        let assign89960_body19_e138082: f64 = (locals.var_chib * locals.var_chib);
        let assign89960_body19_e138084: f64 = (assign89960_body19_e138082 / 2.0);
        let assign89960_body19_e138088: f64 = (locals.var_chib / 3.0);
        let assign89960_body19_e138092: f64 = (locals.var_chib / 4.0);
        let assign89960_body19_e138096: f64 = (locals.var_chib / 5.0);
        let assign89960_body19_e138097: f64 = (1.0 - assign89960_body19_e138096);
        let assign89960_body19_e138098: f64 = (assign89960_body19_e138092 * assign89960_body19_e138097);
        let assign89960_body19_e138099: f64 = (1.0 - assign89960_body19_e138098);
        let assign89960_body19_e138100: f64 = (assign89960_body19_e138088 * assign89960_body19_e138099);
        let assign89960_body19_e138101: f64 = (1.0 - assign89960_body19_e138100);
        let assign89960_body19_e138102: f64 = (assign89960_body19_e138084 * assign89960_body19_e138101);
        (assign89960_body19_e138102, (((((locals.var_chib_dn0 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn0)) / 2.0) * assign89960_body19_e138101) + (assign89960_body19_e138084 * (-(((locals.var_chib_dn0 / 3.0) * assign89960_body19_e138099) + (assign89960_body19_e138088 * (-(((locals.var_chib_dn0 / 4.0) * assign89960_body19_e138097) + (assign89960_body19_e138092 * (-(locals.var_chib_dn0 / 5.0)))))))))), (((((locals.var_chib_dn2 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn2)) / 2.0) * assign89960_body19_e138101) + (assign89960_body19_e138084 * (-(((locals.var_chib_dn2 / 3.0) * assign89960_body19_e138099) + (assign89960_body19_e138088 * (-(((locals.var_chib_dn2 / 4.0) * assign89960_body19_e138097) + (assign89960_body19_e138092 * (-(locals.var_chib_dn2 / 5.0)))))))))), (((((locals.var_chib_dn4 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn4)) / 2.0) * assign89960_body19_e138101) + (assign89960_body19_e138084 * (-(((locals.var_chib_dn4 / 3.0) * assign89960_body19_e138099) + (assign89960_body19_e138088 * (-(((locals.var_chib_dn4 / 4.0) * assign89960_body19_e138097) + (assign89960_body19_e138092 * (-(locals.var_chib_dn4 / 5.0)))))))))), (((((locals.var_chib_dn5 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn5)) / 2.0) * assign89960_body19_e138101) + (assign89960_body19_e138084 * (-(((locals.var_chib_dn5 / 3.0) * assign89960_body19_e138099) + (assign89960_body19_e138088 * (-(((locals.var_chib_dn5 / 4.0) * assign89960_body19_e138097) + (assign89960_body19_e138092 * (-(locals.var_chib_dn5 / 5.0)))))))))), (((((locals.var_chib_dn6 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn6)) / 2.0) * assign89960_body19_e138101) + (assign89960_body19_e138084 * (-(((locals.var_chib_dn6 / 3.0) * assign89960_body19_e138099) + (assign89960_body19_e138088 * (-(((locals.var_chib_dn6 / 4.0) * assign89960_body19_e138097) + (assign89960_body19_e138092 * (-(locals.var_chib_dn6 / 5.0)))))))))), (((((locals.var_chib_dn7 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn7)) / 2.0) * assign89960_body19_e138101) + (assign89960_body19_e138084 * (-(((locals.var_chib_dn7 / 3.0) * assign89960_body19_e138099) + (assign89960_body19_e138088 * (-(((locals.var_chib_dn7 / 4.0) * assign89960_body19_e138097) + (assign89960_body19_e138092 * (-(locals.var_chib_dn7 / 5.0)))))))))), (((((locals.var_chib_dn8 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn8)) / 2.0) * assign89960_body19_e138101) + (assign89960_body19_e138084 * (-(((locals.var_chib_dn8 / 3.0) * assign89960_body19_e138099) + (assign89960_body19_e138088 * (-(((locals.var_chib_dn8 / 4.0) * assign89960_body19_e138097) + (assign89960_body19_e138092 * (-(locals.var_chib_dn8 / 5.0)))))))))), (((((locals.var_chib_dn9 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn9)) / 2.0) * assign89960_body19_e138101) + (assign89960_body19_e138084 * (-(((locals.var_chib_dn9 / 3.0) * assign89960_body19_e138099) + (assign89960_body19_e138088 * (-(((locals.var_chib_dn9 / 4.0) * assign89960_body19_e138097) + (assign89960_body19_e138092 * (-(locals.var_chib_dn9 / 5.0)))))))))), (((((locals.var_chib_dn10 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn10)) / 2.0) * assign89960_body19_e138101) + (assign89960_body19_e138084 * (-(((locals.var_chib_dn10 / 3.0) * assign89960_body19_e138099) + (assign89960_body19_e138088 * (-(((locals.var_chib_dn10 / 4.0) * assign89960_body19_e138097) + (assign89960_body19_e138092 * (-(locals.var_chib_dn10 / 5.0)))))))))), (((((locals.var_chib_dn11 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn11)) / 2.0) * assign89960_body19_e138101) + (assign89960_body19_e138084 * (-(((locals.var_chib_dn11 / 3.0) * assign89960_body19_e138099) + (assign89960_body19_e138088 * (-(((locals.var_chib_dn11 / 4.0) * assign89960_body19_e138097) + (assign89960_body19_e138092 * (-(locals.var_chib_dn11 / 5.0)))))))))), (((((locals.var_chib_dn14 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn14)) / 2.0) * assign89960_body19_e138101) + (assign89960_body19_e138084 * (-(((locals.var_chib_dn14 / 3.0) * assign89960_body19_e138099) + (assign89960_body19_e138088 * (-(((locals.var_chib_dn14 / 4.0) * assign89960_body19_e138097) + (assign89960_body19_e138092 * (-(locals.var_chib_dn14 / 5.0)))))))))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
            locals.var_t2 = assign89960_body19_e138104;
            locals.var_t2_dn0 = assign89960_body19_e138104_d_n0;
            locals.var_t2_dn2 = assign89960_body19_e138104_d_n2;
            locals.var_t2_dn4 = assign89960_body19_e138104_d_n4;
            locals.var_t2_dn5 = assign89960_body19_e138104_d_n5;
            locals.var_t2_dn6 = assign89960_body19_e138104_d_n6;
            locals.var_t2_dn7 = assign89960_body19_e138104_d_n7;
            locals.var_t2_dn8 = assign89960_body19_e138104_d_n8;
            locals.var_t2_dn9 = assign89960_body19_e138104_d_n9;
            locals.var_t2_dn10 = assign89960_body19_e138104_d_n10;
            locals.var_t2_dn11 = assign89960_body19_e138104_d_n11;
            locals.var_t2_dn14 = assign89960_body19_e138104_d_n14;
            locals.var_t2_rv = 0.0;
            let (assign89960_body20_e138130, assign89960_body20_e138130_d_n0, assign89960_body20_e138130_d_n2, assign89960_body20_e138130_d_n4, assign89960_body20_e138130_d_n5, assign89960_body20_e138130_d_n6, assign89960_body20_e138130_d_n7, assign89960_body20_e138130_d_n8, assign89960_body20_e138130_d_n9, assign89960_body20_e138130_d_n10, assign89960_body20_e138130_d_n11, assign89960_body20_e138130_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2089 != 0.0)) && (locals.var_guard2095 != 0.0)) {
        let assign89960_body20_e138114: f64 = (locals.var_chib / 2.0);
        let assign89960_body20_e138118: f64 = (locals.var_chib / 3.0);
        let assign89960_body20_e138122: f64 = (locals.var_chib / 4.0);
        let assign89960_body20_e138123: f64 = (1.0 - assign89960_body20_e138122);
        let assign89960_body20_e138124: f64 = (assign89960_body20_e138118 * assign89960_body20_e138123);
        let assign89960_body20_e138125: f64 = (1.0 - assign89960_body20_e138124);
        let assign89960_body20_e138126: f64 = (assign89960_body20_e138114 * assign89960_body20_e138125);
        let assign89960_body20_e138127: f64 = (1.0 - assign89960_body20_e138126);
        let assign89960_body20_e138128: f64 = (locals.var_chib * assign89960_body20_e138127);
        (assign89960_body20_e138128, ((locals.var_chib_dn0 * assign89960_body20_e138127) + (locals.var_chib * (-(((locals.var_chib_dn0 / 2.0) * assign89960_body20_e138125) + (assign89960_body20_e138114 * (-(((locals.var_chib_dn0 / 3.0) * assign89960_body20_e138123) + (assign89960_body20_e138118 * (-(locals.var_chib_dn0 / 4.0)))))))))), ((locals.var_chib_dn2 * assign89960_body20_e138127) + (locals.var_chib * (-(((locals.var_chib_dn2 / 2.0) * assign89960_body20_e138125) + (assign89960_body20_e138114 * (-(((locals.var_chib_dn2 / 3.0) * assign89960_body20_e138123) + (assign89960_body20_e138118 * (-(locals.var_chib_dn2 / 4.0)))))))))), ((locals.var_chib_dn4 * assign89960_body20_e138127) + (locals.var_chib * (-(((locals.var_chib_dn4 / 2.0) * assign89960_body20_e138125) + (assign89960_body20_e138114 * (-(((locals.var_chib_dn4 / 3.0) * assign89960_body20_e138123) + (assign89960_body20_e138118 * (-(locals.var_chib_dn4 / 4.0)))))))))), ((locals.var_chib_dn5 * assign89960_body20_e138127) + (locals.var_chib * (-(((locals.var_chib_dn5 / 2.0) * assign89960_body20_e138125) + (assign89960_body20_e138114 * (-(((locals.var_chib_dn5 / 3.0) * assign89960_body20_e138123) + (assign89960_body20_e138118 * (-(locals.var_chib_dn5 / 4.0)))))))))), ((locals.var_chib_dn6 * assign89960_body20_e138127) + (locals.var_chib * (-(((locals.var_chib_dn6 / 2.0) * assign89960_body20_e138125) + (assign89960_body20_e138114 * (-(((locals.var_chib_dn6 / 3.0) * assign89960_body20_e138123) + (assign89960_body20_e138118 * (-(locals.var_chib_dn6 / 4.0)))))))))), ((locals.var_chib_dn7 * assign89960_body20_e138127) + (locals.var_chib * (-(((locals.var_chib_dn7 / 2.0) * assign89960_body20_e138125) + (assign89960_body20_e138114 * (-(((locals.var_chib_dn7 / 3.0) * assign89960_body20_e138123) + (assign89960_body20_e138118 * (-(locals.var_chib_dn7 / 4.0)))))))))), ((locals.var_chib_dn8 * assign89960_body20_e138127) + (locals.var_chib * (-(((locals.var_chib_dn8 / 2.0) * assign89960_body20_e138125) + (assign89960_body20_e138114 * (-(((locals.var_chib_dn8 / 3.0) * assign89960_body20_e138123) + (assign89960_body20_e138118 * (-(locals.var_chib_dn8 / 4.0)))))))))), ((locals.var_chib_dn9 * assign89960_body20_e138127) + (locals.var_chib * (-(((locals.var_chib_dn9 / 2.0) * assign89960_body20_e138125) + (assign89960_body20_e138114 * (-(((locals.var_chib_dn9 / 3.0) * assign89960_body20_e138123) + (assign89960_body20_e138118 * (-(locals.var_chib_dn9 / 4.0)))))))))), ((locals.var_chib_dn10 * assign89960_body20_e138127) + (locals.var_chib * (-(((locals.var_chib_dn10 / 2.0) * assign89960_body20_e138125) + (assign89960_body20_e138114 * (-(((locals.var_chib_dn10 / 3.0) * assign89960_body20_e138123) + (assign89960_body20_e138118 * (-(locals.var_chib_dn10 / 4.0)))))))))), ((locals.var_chib_dn11 * assign89960_body20_e138127) + (locals.var_chib * (-(((locals.var_chib_dn11 / 2.0) * assign89960_body20_e138125) + (assign89960_body20_e138114 * (-(((locals.var_chib_dn11 / 3.0) * assign89960_body20_e138123) + (assign89960_body20_e138118 * (-(locals.var_chib_dn11 / 4.0)))))))))), ((locals.var_chib_dn14 * assign89960_body20_e138127) + (locals.var_chib * (-(((locals.var_chib_dn14 / 2.0) * assign89960_body20_e138125) + (assign89960_body20_e138114 * (-(((locals.var_chib_dn14 / 3.0) * assign89960_body20_e138123) + (assign89960_body20_e138118 * (-(locals.var_chib_dn14 / 4.0)))))))))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
            locals.var_t3 = assign89960_body20_e138130;
            locals.var_t3_dn0 = assign89960_body20_e138130_d_n0;
            locals.var_t3_dn2 = assign89960_body20_e138130_d_n2;
            locals.var_t3_dn4 = assign89960_body20_e138130_d_n4;
            locals.var_t3_dn5 = assign89960_body20_e138130_d_n5;
            locals.var_t3_dn6 = assign89960_body20_e138130_d_n6;
            locals.var_t3_dn7 = assign89960_body20_e138130_d_n7;
            locals.var_t3_dn8 = assign89960_body20_e138130_d_n8;
            locals.var_t3_dn9 = assign89960_body20_e138130_d_n9;
            locals.var_t3_dn10 = assign89960_body20_e138130_d_n10;
            locals.var_t3_dn11 = assign89960_body20_e138130_d_n11;
            locals.var_t3_dn14 = assign89960_body20_e138130_d_n14;
            locals.var_t3_rv = 0.0;
            let (assign89960_body21_e138140, assign89960_body21_e138140_d_n0, assign89960_body21_e138140_d_n2, assign89960_body21_e138140_d_n4, assign89960_body21_e138140_d_n5, assign89960_body21_e138140_d_n6, assign89960_body21_e138140_d_n7, assign89960_body21_e138140_d_n8, assign89960_body21_e138140_d_n9, assign89960_body21_e138140_d_n10, assign89960_body21_e138140_d_n11, assign89960_body21_e138140_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2089 != 0.0)) && (locals.var_guard2095 != 0.0)) {
        let assign89960_body21_e138138: f64 = (locals.var_t0 - locals.var_t2);
        (assign89960_body21_e138138, (locals.var_t0_dn0 - locals.var_t2_dn0), (locals.var_t0_dn2 - locals.var_t2_dn2), (locals.var_t0_dn4 - locals.var_t2_dn4), (locals.var_t0_dn5 - locals.var_t2_dn5), (locals.var_t0_dn6 - locals.var_t2_dn6), (locals.var_t0_dn7 - locals.var_t2_dn7), (locals.var_t0_dn8 - locals.var_t2_dn8), (locals.var_t0_dn9 - locals.var_t2_dn9), (locals.var_t0_dn10 - locals.var_t2_dn10), (locals.var_t0_dn11 - locals.var_t2_dn11), (locals.var_t0_dn14 - locals.var_t2_dn14),)
    } else {
        (locals.var_fbsq__blk2016, locals.var_fbsq__blk2016_dn0, locals.var_fbsq__blk2016_dn2, locals.var_fbsq__blk2016_dn4, locals.var_fbsq__blk2016_dn5, locals.var_fbsq__blk2016_dn6, locals.var_fbsq__blk2016_dn7, locals.var_fbsq__blk2016_dn8, locals.var_fbsq__blk2016_dn9, locals.var_fbsq__blk2016_dn10, locals.var_fbsq__blk2016_dn11, locals.var_fbsq__blk2016_dn14,)
    }
};
            locals.var_fbsq__blk2016 = assign89960_body21_e138140;
            locals.var_fbsq__blk2016_dn0 = assign89960_body21_e138140_d_n0;
            locals.var_fbsq__blk2016_dn2 = assign89960_body21_e138140_d_n2;
            locals.var_fbsq__blk2016_dn4 = assign89960_body21_e138140_d_n4;
            locals.var_fbsq__blk2016_dn5 = assign89960_body21_e138140_d_n5;
            locals.var_fbsq__blk2016_dn6 = assign89960_body21_e138140_d_n6;
            locals.var_fbsq__blk2016_dn7 = assign89960_body21_e138140_d_n7;
            locals.var_fbsq__blk2016_dn8 = assign89960_body21_e138140_d_n8;
            locals.var_fbsq__blk2016_dn9 = assign89960_body21_e138140_d_n9;
            locals.var_fbsq__blk2016_dn10 = assign89960_body21_e138140_d_n10;
            locals.var_fbsq__blk2016_dn11 = assign89960_body21_e138140_d_n11;
            locals.var_fbsq__blk2016_dn14 = assign89960_body21_e138140_d_n14;
            locals.var_fbsq__blk2016_rv = 0.0;
            let (assign89960_body22_e138154, assign89960_body22_e138154_d_n0, assign89960_body22_e138154_d_n2, assign89960_body22_e138154_d_n4, assign89960_body22_e138154_d_n5, assign89960_body22_e138154_d_n6, assign89960_body22_e138154_d_n7, assign89960_body22_e138154_d_n8, assign89960_body22_e138154_d_n9, assign89960_body22_e138154_d_n10, assign89960_body22_e138154_d_n11, assign89960_body22_e138154_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2089 != 0.0)) && (locals.var_guard2095 != 0.0)) {
        let assign89960_body22_e138150: f64 = (locals.var_phi_b_dpss * locals.var_t3);
        let assign89960_body22_e138151: f64 = (locals.var_t1 - assign89960_body22_e138150);
        let assign89960_body22_e138152: f64 = (locals.var_beta * assign89960_body22_e138151);
        (assign89960_body22_e138152, ((locals.var_beta_dn0 * assign89960_body22_e138151) + (locals.var_beta * (locals.var_t1_dn0 - ((locals.var_phi_b_dpss_dn0 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn0))))), ((locals.var_beta_dn2 * assign89960_body22_e138151) + (locals.var_beta * (locals.var_t1_dn2 - ((locals.var_phi_b_dpss_dn2 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn2))))), ((locals.var_beta_dn4 * assign89960_body22_e138151) + (locals.var_beta * (locals.var_t1_dn4 - ((locals.var_phi_b_dpss_dn4 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn4))))), ((locals.var_beta_dn5 * assign89960_body22_e138151) + (locals.var_beta * (locals.var_t1_dn5 - ((locals.var_phi_b_dpss_dn5 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn5))))), ((locals.var_beta_dn6 * assign89960_body22_e138151) + (locals.var_beta * (locals.var_t1_dn6 - ((locals.var_phi_b_dpss_dn6 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn6))))), ((locals.var_beta_dn7 * assign89960_body22_e138151) + (locals.var_beta * (locals.var_t1_dn7 - ((locals.var_phi_b_dpss_dn7 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn7))))), ((locals.var_beta_dn8 * assign89960_body22_e138151) + (locals.var_beta * (locals.var_t1_dn8 - ((locals.var_phi_b_dpss_dn8 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn8))))), ((locals.var_beta_dn9 * assign89960_body22_e138151) + (locals.var_beta * (locals.var_t1_dn9 - ((locals.var_phi_b_dpss_dn9 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn9))))), ((locals.var_beta_dn10 * assign89960_body22_e138151) + (locals.var_beta * (locals.var_t1_dn10 - ((locals.var_phi_b_dpss_dn10 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn10))))), ((locals.var_beta_dn11 * assign89960_body22_e138151) + (locals.var_beta * (locals.var_t1_dn11 - ((locals.var_phi_b_dpss_dn11 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn11))))), ((locals.var_beta_dn14 * assign89960_body22_e138151) + (locals.var_beta * (locals.var_t1_dn14 - ((locals.var_phi_b_dpss_dn14 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn14))))),)
    } else {
        (locals.var_fbsq_dpss__blk2017, locals.var_fbsq_dpss__blk2017_dn0, locals.var_fbsq_dpss__blk2017_dn2, locals.var_fbsq_dpss__blk2017_dn4, locals.var_fbsq_dpss__blk2017_dn5, locals.var_fbsq_dpss__blk2017_dn6, locals.var_fbsq_dpss__blk2017_dn7, locals.var_fbsq_dpss__blk2017_dn8, locals.var_fbsq_dpss__blk2017_dn9, locals.var_fbsq_dpss__blk2017_dn10, locals.var_fbsq_dpss__blk2017_dn11, locals.var_fbsq_dpss__blk2017_dn14,)
    }
};
            locals.var_fbsq_dpss__blk2017 = assign89960_body22_e138154;
            locals.var_fbsq_dpss__blk2017_dn0 = assign89960_body22_e138154_d_n0;
            locals.var_fbsq_dpss__blk2017_dn2 = assign89960_body22_e138154_d_n2;
            locals.var_fbsq_dpss__blk2017_dn4 = assign89960_body22_e138154_d_n4;
            locals.var_fbsq_dpss__blk2017_dn5 = assign89960_body22_e138154_d_n5;
            locals.var_fbsq_dpss__blk2017_dn6 = assign89960_body22_e138154_d_n6;
            locals.var_fbsq_dpss__blk2017_dn7 = assign89960_body22_e138154_d_n7;
            locals.var_fbsq_dpss__blk2017_dn8 = assign89960_body22_e138154_d_n8;
            locals.var_fbsq_dpss__blk2017_dn9 = assign89960_body22_e138154_d_n9;
            locals.var_fbsq_dpss__blk2017_dn10 = assign89960_body22_e138154_d_n10;
            locals.var_fbsq_dpss__blk2017_dn11 = assign89960_body22_e138154_d_n11;
            locals.var_fbsq_dpss__blk2017_dn14 = assign89960_body22_e138154_d_n14;
            locals.var_fbsq_dpss__blk2017_rv = 0.0;
            let (assign89960_body24_e138182, assign89960_body24_e138182_d_n0, assign89960_body24_e138182_d_n2, assign89960_body24_e138182_d_n4, assign89960_body24_e138182_d_n5, assign89960_body24_e138182_d_n6, assign89960_body24_e138182_d_n7, assign89960_body24_e138182_d_n8, assign89960_body24_e138182_d_n9, assign89960_body24_e138182_d_n10, assign89960_body24_e138182_d_n11, assign89960_body24_e138182_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2089 != 0.0)) && (locals.var_guard2095 == 0.0)) {
        let assign89960_body24_e138179: f64 = (-locals.var_chi);
        let assign89960_body24_e138180: f64 = (assign89960_body24_e138179).exp();
        (assign89960_body24_e138180, (assign89960_body24_e138180 * (-locals.var_chi_dn0)), (assign89960_body24_e138180 * (-locals.var_chi_dn2)), (assign89960_body24_e138180 * (-locals.var_chi_dn4)), (assign89960_body24_e138180 * (-locals.var_chi_dn5)), (assign89960_body24_e138180 * (-locals.var_chi_dn6)), (assign89960_body24_e138180 * (-locals.var_chi_dn7)), (assign89960_body24_e138180 * (-locals.var_chi_dn8)), (assign89960_body24_e138180 * (-locals.var_chi_dn9)), (assign89960_body24_e138180 * (-locals.var_chi_dn10)), (assign89960_body24_e138180 * (-locals.var_chi_dn11)), (assign89960_body24_e138180 * (-locals.var_chi_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
            locals.var_t0 = assign89960_body24_e138182;
            locals.var_t0_dn0 = assign89960_body24_e138182_d_n0;
            locals.var_t0_dn2 = assign89960_body24_e138182_d_n2;
            locals.var_t0_dn4 = assign89960_body24_e138182_d_n4;
            locals.var_t0_dn5 = assign89960_body24_e138182_d_n5;
            locals.var_t0_dn6 = assign89960_body24_e138182_d_n6;
            locals.var_t0_dn7 = assign89960_body24_e138182_d_n7;
            locals.var_t0_dn8 = assign89960_body24_e138182_d_n8;
            locals.var_t0_dn9 = assign89960_body24_e138182_d_n9;
            locals.var_t0_dn10 = assign89960_body24_e138182_d_n10;
            locals.var_t0_dn11 = assign89960_body24_e138182_d_n11;
            locals.var_t0_dn14 = assign89960_body24_e138182_d_n14;
            locals.var_t0_rv = 0.0;
            let (assign89960_body25_e138193, assign89960_body25_e138193_d_n0, assign89960_body25_e138193_d_n2, assign89960_body25_e138193_d_n4, assign89960_body25_e138193_d_n5, assign89960_body25_e138193_d_n6, assign89960_body25_e138193_d_n7, assign89960_body25_e138193_d_n8, assign89960_body25_e138193_d_n9, assign89960_body25_e138193_d_n10, assign89960_body25_e138193_d_n11, assign89960_body25_e138193_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2089 != 0.0)) && (locals.var_guard2095 == 0.0)) {
        let assign89960_body25_e138190: f64 = (-locals.var_chib);
        let assign89960_body25_e138191: f64 = (assign89960_body25_e138190).exp();
        (assign89960_body25_e138191, (assign89960_body25_e138191 * (-locals.var_chib_dn0)), (assign89960_body25_e138191 * (-locals.var_chib_dn2)), (assign89960_body25_e138191 * (-locals.var_chib_dn4)), (assign89960_body25_e138191 * (-locals.var_chib_dn5)), (assign89960_body25_e138191 * (-locals.var_chib_dn6)), (assign89960_body25_e138191 * (-locals.var_chib_dn7)), (assign89960_body25_e138191 * (-locals.var_chib_dn8)), (assign89960_body25_e138191 * (-locals.var_chib_dn9)), (assign89960_body25_e138191 * (-locals.var_chib_dn10)), (assign89960_body25_e138191 * (-locals.var_chib_dn11)), (assign89960_body25_e138191 * (-locals.var_chib_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
            locals.var_t1 = assign89960_body25_e138193;
            locals.var_t1_dn0 = assign89960_body25_e138193_d_n0;
            locals.var_t1_dn2 = assign89960_body25_e138193_d_n2;
            locals.var_t1_dn4 = assign89960_body25_e138193_d_n4;
            locals.var_t1_dn5 = assign89960_body25_e138193_d_n5;
            locals.var_t1_dn6 = assign89960_body25_e138193_d_n6;
            locals.var_t1_dn7 = assign89960_body25_e138193_d_n7;
            locals.var_t1_dn8 = assign89960_body25_e138193_d_n8;
            locals.var_t1_dn9 = assign89960_body25_e138193_d_n9;
            locals.var_t1_dn10 = assign89960_body25_e138193_d_n10;
            locals.var_t1_dn11 = assign89960_body25_e138193_d_n11;
            locals.var_t1_dn14 = assign89960_body25_e138193_d_n14;
            locals.var_t1_rv = 0.0;
            let (assign89960_body26_e138208, assign89960_body26_e138208_d_n0, assign89960_body26_e138208_d_n2, assign89960_body26_e138208_d_n4, assign89960_body26_e138208_d_n5, assign89960_body26_e138208_d_n6, assign89960_body26_e138208_d_n7, assign89960_body26_e138208_d_n8, assign89960_body26_e138208_d_n9, assign89960_body26_e138208_d_n10, assign89960_body26_e138208_d_n11, assign89960_body26_e138208_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2089 != 0.0)) && (locals.var_guard2095 == 0.0)) {
        let assign89960_body26_e138202: f64 = (locals.var_chi - locals.var_chib);
        let assign89960_body26_e138205: f64 = (locals.var_t0 - locals.var_t1);
        let assign89960_body26_e138206: f64 = (assign89960_body26_e138202 + assign89960_body26_e138205);
        (assign89960_body26_e138206, ((locals.var_chi_dn0 - locals.var_chib_dn0) + (locals.var_t0_dn0 - locals.var_t1_dn0)), ((locals.var_chi_dn2 - locals.var_chib_dn2) + (locals.var_t0_dn2 - locals.var_t1_dn2)), ((locals.var_chi_dn4 - locals.var_chib_dn4) + (locals.var_t0_dn4 - locals.var_t1_dn4)), ((locals.var_chi_dn5 - locals.var_chib_dn5) + (locals.var_t0_dn5 - locals.var_t1_dn5)), ((locals.var_chi_dn6 - locals.var_chib_dn6) + (locals.var_t0_dn6 - locals.var_t1_dn6)), ((locals.var_chi_dn7 - locals.var_chib_dn7) + (locals.var_t0_dn7 - locals.var_t1_dn7)), ((locals.var_chi_dn8 - locals.var_chib_dn8) + (locals.var_t0_dn8 - locals.var_t1_dn8)), ((locals.var_chi_dn9 - locals.var_chib_dn9) + (locals.var_t0_dn9 - locals.var_t1_dn9)), ((locals.var_chi_dn10 - locals.var_chib_dn10) + (locals.var_t0_dn10 - locals.var_t1_dn10)), ((locals.var_chi_dn11 - locals.var_chib_dn11) + (locals.var_t0_dn11 - locals.var_t1_dn11)), ((locals.var_chi_dn14 - locals.var_chib_dn14) + (locals.var_t0_dn14 - locals.var_t1_dn14)),)
    } else {
        (locals.var_fbsq__blk2016, locals.var_fbsq__blk2016_dn0, locals.var_fbsq__blk2016_dn2, locals.var_fbsq__blk2016_dn4, locals.var_fbsq__blk2016_dn5, locals.var_fbsq__blk2016_dn6, locals.var_fbsq__blk2016_dn7, locals.var_fbsq__blk2016_dn8, locals.var_fbsq__blk2016_dn9, locals.var_fbsq__blk2016_dn10, locals.var_fbsq__blk2016_dn11, locals.var_fbsq__blk2016_dn14,)
    }
};
            locals.var_fbsq__blk2016 = assign89960_body26_e138208;
            locals.var_fbsq__blk2016_dn0 = assign89960_body26_e138208_d_n0;
            locals.var_fbsq__blk2016_dn2 = assign89960_body26_e138208_d_n2;
            locals.var_fbsq__blk2016_dn4 = assign89960_body26_e138208_d_n4;
            locals.var_fbsq__blk2016_dn5 = assign89960_body26_e138208_d_n5;
            locals.var_fbsq__blk2016_dn6 = assign89960_body26_e138208_d_n6;
            locals.var_fbsq__blk2016_dn7 = assign89960_body26_e138208_d_n7;
            locals.var_fbsq__blk2016_dn8 = assign89960_body26_e138208_d_n8;
            locals.var_fbsq__blk2016_dn9 = assign89960_body26_e138208_d_n9;
            locals.var_fbsq__blk2016_dn10 = assign89960_body26_e138208_d_n10;
            locals.var_fbsq__blk2016_dn11 = assign89960_body26_e138208_d_n11;
            locals.var_fbsq__blk2016_dn14 = assign89960_body26_e138208_d_n14;
            locals.var_fbsq__blk2016_rv = 0.0;
            let (assign89960_body27_e138227, assign89960_body27_e138227_d_n0, assign89960_body27_e138227_d_n2, assign89960_body27_e138227_d_n4, assign89960_body27_e138227_d_n5, assign89960_body27_e138227_d_n6, assign89960_body27_e138227_d_n7, assign89960_body27_e138227_d_n8, assign89960_body27_e138227_d_n9, assign89960_body27_e138227_d_n10, assign89960_body27_e138227_d_n11, assign89960_body27_e138227_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2089 != 0.0)) && (locals.var_guard2095 == 0.0)) {
        let assign89960_body27_e138218: f64 = (1.0 - locals.var_t0);
        let assign89960_body27_e138222: f64 = (1.0 - locals.var_t1);
        let assign89960_body27_e138223: f64 = (locals.var_phi_b_dpss * assign89960_body27_e138222);
        let assign89960_body27_e138224: f64 = (assign89960_body27_e138218 - assign89960_body27_e138223);
        let assign89960_body27_e138225: f64 = (locals.var_beta * assign89960_body27_e138224);
        (assign89960_body27_e138225, ((locals.var_beta_dn0 * assign89960_body27_e138224) + (locals.var_beta * ((-locals.var_t0_dn0) - ((locals.var_phi_b_dpss_dn0 * assign89960_body27_e138222) + (locals.var_phi_b_dpss * (-locals.var_t1_dn0)))))), ((locals.var_beta_dn2 * assign89960_body27_e138224) + (locals.var_beta * ((-locals.var_t0_dn2) - ((locals.var_phi_b_dpss_dn2 * assign89960_body27_e138222) + (locals.var_phi_b_dpss * (-locals.var_t1_dn2)))))), ((locals.var_beta_dn4 * assign89960_body27_e138224) + (locals.var_beta * ((-locals.var_t0_dn4) - ((locals.var_phi_b_dpss_dn4 * assign89960_body27_e138222) + (locals.var_phi_b_dpss * (-locals.var_t1_dn4)))))), ((locals.var_beta_dn5 * assign89960_body27_e138224) + (locals.var_beta * ((-locals.var_t0_dn5) - ((locals.var_phi_b_dpss_dn5 * assign89960_body27_e138222) + (locals.var_phi_b_dpss * (-locals.var_t1_dn5)))))), ((locals.var_beta_dn6 * assign89960_body27_e138224) + (locals.var_beta * ((-locals.var_t0_dn6) - ((locals.var_phi_b_dpss_dn6 * assign89960_body27_e138222) + (locals.var_phi_b_dpss * (-locals.var_t1_dn6)))))), ((locals.var_beta_dn7 * assign89960_body27_e138224) + (locals.var_beta * ((-locals.var_t0_dn7) - ((locals.var_phi_b_dpss_dn7 * assign89960_body27_e138222) + (locals.var_phi_b_dpss * (-locals.var_t1_dn7)))))), ((locals.var_beta_dn8 * assign89960_body27_e138224) + (locals.var_beta * ((-locals.var_t0_dn8) - ((locals.var_phi_b_dpss_dn8 * assign89960_body27_e138222) + (locals.var_phi_b_dpss * (-locals.var_t1_dn8)))))), ((locals.var_beta_dn9 * assign89960_body27_e138224) + (locals.var_beta * ((-locals.var_t0_dn9) - ((locals.var_phi_b_dpss_dn9 * assign89960_body27_e138222) + (locals.var_phi_b_dpss * (-locals.var_t1_dn9)))))), ((locals.var_beta_dn10 * assign89960_body27_e138224) + (locals.var_beta * ((-locals.var_t0_dn10) - ((locals.var_phi_b_dpss_dn10 * assign89960_body27_e138222) + (locals.var_phi_b_dpss * (-locals.var_t1_dn10)))))), ((locals.var_beta_dn11 * assign89960_body27_e138224) + (locals.var_beta * ((-locals.var_t0_dn11) - ((locals.var_phi_b_dpss_dn11 * assign89960_body27_e138222) + (locals.var_phi_b_dpss * (-locals.var_t1_dn11)))))), ((locals.var_beta_dn14 * assign89960_body27_e138224) + (locals.var_beta * ((-locals.var_t0_dn14) - ((locals.var_phi_b_dpss_dn14 * assign89960_body27_e138222) + (locals.var_phi_b_dpss * (-locals.var_t1_dn14)))))),)
    } else {
        (locals.var_fbsq_dpss__blk2017, locals.var_fbsq_dpss__blk2017_dn0, locals.var_fbsq_dpss__blk2017_dn2, locals.var_fbsq_dpss__blk2017_dn4, locals.var_fbsq_dpss__blk2017_dn5, locals.var_fbsq_dpss__blk2017_dn6, locals.var_fbsq_dpss__blk2017_dn7, locals.var_fbsq_dpss__blk2017_dn8, locals.var_fbsq_dpss__blk2017_dn9, locals.var_fbsq_dpss__blk2017_dn10, locals.var_fbsq_dpss__blk2017_dn11, locals.var_fbsq_dpss__blk2017_dn14,)
    }
};
            locals.var_fbsq_dpss__blk2017 = assign89960_body27_e138227;
            locals.var_fbsq_dpss__blk2017_dn0 = assign89960_body27_e138227_d_n0;
            locals.var_fbsq_dpss__blk2017_dn2 = assign89960_body27_e138227_d_n2;
            locals.var_fbsq_dpss__blk2017_dn4 = assign89960_body27_e138227_d_n4;
            locals.var_fbsq_dpss__blk2017_dn5 = assign89960_body27_e138227_d_n5;
            locals.var_fbsq_dpss__blk2017_dn6 = assign89960_body27_e138227_d_n6;
            locals.var_fbsq_dpss__blk2017_dn7 = assign89960_body27_e138227_d_n7;
            locals.var_fbsq_dpss__blk2017_dn8 = assign89960_body27_e138227_d_n8;
            locals.var_fbsq_dpss__blk2017_dn9 = assign89960_body27_e138227_d_n9;
            locals.var_fbsq_dpss__blk2017_dn10 = assign89960_body27_e138227_d_n10;
            locals.var_fbsq_dpss__blk2017_dn11 = assign89960_body27_e138227_d_n11;
            locals.var_fbsq_dpss__blk2017_dn14 = assign89960_body27_e138227_d_n14;
            locals.var_fbsq_dpss__blk2017_rv = 0.0;
            let assign89960_body28_e138229: f64 = (locals.var_chi).abs();
            let assign89960_body28_e138231: f64 = if assign89960_body28_e138229 < 5e-5 { 1.0 } else { 0.0 };
            locals.var_guard2096 = assign89960_body28_e138231;
            locals.var_guard2096_rv = 0.0;
            let (assign89960_body29_e138261, assign89960_body29_e138261_d_n0, assign89960_body29_e138261_d_n2, assign89960_body29_e138261_d_n4, assign89960_body29_e138261_d_n5, assign89960_body29_e138261_d_n6, assign89960_body29_e138261_d_n7, assign89960_body29_e138261_d_n8, assign89960_body29_e138261_d_n9, assign89960_body29_e138261_d_n10, assign89960_body29_e138261_d_n11, assign89960_body29_e138261_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2089 != 0.0)) && (locals.var_guard2096 != 0.0)) {
        let assign89960_body29_e138239: f64 = (locals.var_chi * locals.var_chi);
        let assign89960_body29_e138241: f64 = (assign89960_body29_e138239 / 2.0);
        let assign89960_body29_e138245: f64 = (locals.var_chi / 3.0);
        let assign89960_body29_e138249: f64 = (locals.var_chi / 4.0);
        let assign89960_body29_e138253: f64 = (locals.var_chi / 5.0);
        let assign89960_body29_e138254: f64 = (1.0 + assign89960_body29_e138253);
        let assign89960_body29_e138255: f64 = (assign89960_body29_e138249 * assign89960_body29_e138254);
        let assign89960_body29_e138256: f64 = (1.0 + assign89960_body29_e138255);
        let assign89960_body29_e138257: f64 = (assign89960_body29_e138245 * assign89960_body29_e138256);
        let assign89960_body29_e138258: f64 = (1.0 + assign89960_body29_e138257);
        let assign89960_body29_e138259: f64 = (assign89960_body29_e138241 * assign89960_body29_e138258);
        (assign89960_body29_e138259, (((((locals.var_chi_dn0 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn0)) / 2.0) * assign89960_body29_e138258) + (assign89960_body29_e138241 * (((locals.var_chi_dn0 / 3.0) * assign89960_body29_e138256) + (assign89960_body29_e138245 * (((locals.var_chi_dn0 / 4.0) * assign89960_body29_e138254) + (assign89960_body29_e138249 * (locals.var_chi_dn0 / 5.0))))))), (((((locals.var_chi_dn2 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn2)) / 2.0) * assign89960_body29_e138258) + (assign89960_body29_e138241 * (((locals.var_chi_dn2 / 3.0) * assign89960_body29_e138256) + (assign89960_body29_e138245 * (((locals.var_chi_dn2 / 4.0) * assign89960_body29_e138254) + (assign89960_body29_e138249 * (locals.var_chi_dn2 / 5.0))))))), (((((locals.var_chi_dn4 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn4)) / 2.0) * assign89960_body29_e138258) + (assign89960_body29_e138241 * (((locals.var_chi_dn4 / 3.0) * assign89960_body29_e138256) + (assign89960_body29_e138245 * (((locals.var_chi_dn4 / 4.0) * assign89960_body29_e138254) + (assign89960_body29_e138249 * (locals.var_chi_dn4 / 5.0))))))), (((((locals.var_chi_dn5 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn5)) / 2.0) * assign89960_body29_e138258) + (assign89960_body29_e138241 * (((locals.var_chi_dn5 / 3.0) * assign89960_body29_e138256) + (assign89960_body29_e138245 * (((locals.var_chi_dn5 / 4.0) * assign89960_body29_e138254) + (assign89960_body29_e138249 * (locals.var_chi_dn5 / 5.0))))))), (((((locals.var_chi_dn6 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn6)) / 2.0) * assign89960_body29_e138258) + (assign89960_body29_e138241 * (((locals.var_chi_dn6 / 3.0) * assign89960_body29_e138256) + (assign89960_body29_e138245 * (((locals.var_chi_dn6 / 4.0) * assign89960_body29_e138254) + (assign89960_body29_e138249 * (locals.var_chi_dn6 / 5.0))))))), (((((locals.var_chi_dn7 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn7)) / 2.0) * assign89960_body29_e138258) + (assign89960_body29_e138241 * (((locals.var_chi_dn7 / 3.0) * assign89960_body29_e138256) + (assign89960_body29_e138245 * (((locals.var_chi_dn7 / 4.0) * assign89960_body29_e138254) + (assign89960_body29_e138249 * (locals.var_chi_dn7 / 5.0))))))), (((((locals.var_chi_dn8 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn8)) / 2.0) * assign89960_body29_e138258) + (assign89960_body29_e138241 * (((locals.var_chi_dn8 / 3.0) * assign89960_body29_e138256) + (assign89960_body29_e138245 * (((locals.var_chi_dn8 / 4.0) * assign89960_body29_e138254) + (assign89960_body29_e138249 * (locals.var_chi_dn8 / 5.0))))))), (((((locals.var_chi_dn9 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn9)) / 2.0) * assign89960_body29_e138258) + (assign89960_body29_e138241 * (((locals.var_chi_dn9 / 3.0) * assign89960_body29_e138256) + (assign89960_body29_e138245 * (((locals.var_chi_dn9 / 4.0) * assign89960_body29_e138254) + (assign89960_body29_e138249 * (locals.var_chi_dn9 / 5.0))))))), (((((locals.var_chi_dn10 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn10)) / 2.0) * assign89960_body29_e138258) + (assign89960_body29_e138241 * (((locals.var_chi_dn10 / 3.0) * assign89960_body29_e138256) + (assign89960_body29_e138245 * (((locals.var_chi_dn10 / 4.0) * assign89960_body29_e138254) + (assign89960_body29_e138249 * (locals.var_chi_dn10 / 5.0))))))), (((((locals.var_chi_dn11 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn11)) / 2.0) * assign89960_body29_e138258) + (assign89960_body29_e138241 * (((locals.var_chi_dn11 / 3.0) * assign89960_body29_e138256) + (assign89960_body29_e138245 * (((locals.var_chi_dn11 / 4.0) * assign89960_body29_e138254) + (assign89960_body29_e138249 * (locals.var_chi_dn11 / 5.0))))))), (((((locals.var_chi_dn14 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn14)) / 2.0) * assign89960_body29_e138258) + (assign89960_body29_e138241 * (((locals.var_chi_dn14 / 3.0) * assign89960_body29_e138256) + (assign89960_body29_e138245 * (((locals.var_chi_dn14 / 4.0) * assign89960_body29_e138254) + (assign89960_body29_e138249 * (locals.var_chi_dn14 / 5.0))))))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
            locals.var_t0 = assign89960_body29_e138261;
            locals.var_t0_dn0 = assign89960_body29_e138261_d_n0;
            locals.var_t0_dn2 = assign89960_body29_e138261_d_n2;
            locals.var_t0_dn4 = assign89960_body29_e138261_d_n4;
            locals.var_t0_dn5 = assign89960_body29_e138261_d_n5;
            locals.var_t0_dn6 = assign89960_body29_e138261_d_n6;
            locals.var_t0_dn7 = assign89960_body29_e138261_d_n7;
            locals.var_t0_dn8 = assign89960_body29_e138261_d_n8;
            locals.var_t0_dn9 = assign89960_body29_e138261_d_n9;
            locals.var_t0_dn10 = assign89960_body29_e138261_d_n10;
            locals.var_t0_dn11 = assign89960_body29_e138261_d_n11;
            locals.var_t0_dn14 = assign89960_body29_e138261_d_n14;
            locals.var_t0_rv = 0.0;
            let (assign89960_body30_e138287, assign89960_body30_e138287_d_n0, assign89960_body30_e138287_d_n2, assign89960_body30_e138287_d_n4, assign89960_body30_e138287_d_n5, assign89960_body30_e138287_d_n6, assign89960_body30_e138287_d_n7, assign89960_body30_e138287_d_n8, assign89960_body30_e138287_d_n9, assign89960_body30_e138287_d_n10, assign89960_body30_e138287_d_n11, assign89960_body30_e138287_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2089 != 0.0)) && (locals.var_guard2096 != 0.0)) {
        let assign89960_body30_e138271: f64 = (locals.var_chi / 2.0);
        let assign89960_body30_e138275: f64 = (locals.var_chi / 3.0);
        let assign89960_body30_e138279: f64 = (locals.var_chi / 4.0);
        let assign89960_body30_e138280: f64 = (1.0 + assign89960_body30_e138279);
        let assign89960_body30_e138281: f64 = (assign89960_body30_e138275 * assign89960_body30_e138280);
        let assign89960_body30_e138282: f64 = (1.0 + assign89960_body30_e138281);
        let assign89960_body30_e138283: f64 = (assign89960_body30_e138271 * assign89960_body30_e138282);
        let assign89960_body30_e138284: f64 = (1.0 + assign89960_body30_e138283);
        let assign89960_body30_e138285: f64 = (locals.var_chi * assign89960_body30_e138284);
        (assign89960_body30_e138285, ((locals.var_chi_dn0 * assign89960_body30_e138284) + (locals.var_chi * (((locals.var_chi_dn0 / 2.0) * assign89960_body30_e138282) + (assign89960_body30_e138271 * (((locals.var_chi_dn0 / 3.0) * assign89960_body30_e138280) + (assign89960_body30_e138275 * (locals.var_chi_dn0 / 4.0))))))), ((locals.var_chi_dn2 * assign89960_body30_e138284) + (locals.var_chi * (((locals.var_chi_dn2 / 2.0) * assign89960_body30_e138282) + (assign89960_body30_e138271 * (((locals.var_chi_dn2 / 3.0) * assign89960_body30_e138280) + (assign89960_body30_e138275 * (locals.var_chi_dn2 / 4.0))))))), ((locals.var_chi_dn4 * assign89960_body30_e138284) + (locals.var_chi * (((locals.var_chi_dn4 / 2.0) * assign89960_body30_e138282) + (assign89960_body30_e138271 * (((locals.var_chi_dn4 / 3.0) * assign89960_body30_e138280) + (assign89960_body30_e138275 * (locals.var_chi_dn4 / 4.0))))))), ((locals.var_chi_dn5 * assign89960_body30_e138284) + (locals.var_chi * (((locals.var_chi_dn5 / 2.0) * assign89960_body30_e138282) + (assign89960_body30_e138271 * (((locals.var_chi_dn5 / 3.0) * assign89960_body30_e138280) + (assign89960_body30_e138275 * (locals.var_chi_dn5 / 4.0))))))), ((locals.var_chi_dn6 * assign89960_body30_e138284) + (locals.var_chi * (((locals.var_chi_dn6 / 2.0) * assign89960_body30_e138282) + (assign89960_body30_e138271 * (((locals.var_chi_dn6 / 3.0) * assign89960_body30_e138280) + (assign89960_body30_e138275 * (locals.var_chi_dn6 / 4.0))))))), ((locals.var_chi_dn7 * assign89960_body30_e138284) + (locals.var_chi * (((locals.var_chi_dn7 / 2.0) * assign89960_body30_e138282) + (assign89960_body30_e138271 * (((locals.var_chi_dn7 / 3.0) * assign89960_body30_e138280) + (assign89960_body30_e138275 * (locals.var_chi_dn7 / 4.0))))))), ((locals.var_chi_dn8 * assign89960_body30_e138284) + (locals.var_chi * (((locals.var_chi_dn8 / 2.0) * assign89960_body30_e138282) + (assign89960_body30_e138271 * (((locals.var_chi_dn8 / 3.0) * assign89960_body30_e138280) + (assign89960_body30_e138275 * (locals.var_chi_dn8 / 4.0))))))), ((locals.var_chi_dn9 * assign89960_body30_e138284) + (locals.var_chi * (((locals.var_chi_dn9 / 2.0) * assign89960_body30_e138282) + (assign89960_body30_e138271 * (((locals.var_chi_dn9 / 3.0) * assign89960_body30_e138280) + (assign89960_body30_e138275 * (locals.var_chi_dn9 / 4.0))))))), ((locals.var_chi_dn10 * assign89960_body30_e138284) + (locals.var_chi * (((locals.var_chi_dn10 / 2.0) * assign89960_body30_e138282) + (assign89960_body30_e138271 * (((locals.var_chi_dn10 / 3.0) * assign89960_body30_e138280) + (assign89960_body30_e138275 * (locals.var_chi_dn10 / 4.0))))))), ((locals.var_chi_dn11 * assign89960_body30_e138284) + (locals.var_chi * (((locals.var_chi_dn11 / 2.0) * assign89960_body30_e138282) + (assign89960_body30_e138271 * (((locals.var_chi_dn11 / 3.0) * assign89960_body30_e138280) + (assign89960_body30_e138275 * (locals.var_chi_dn11 / 4.0))))))), ((locals.var_chi_dn14 * assign89960_body30_e138284) + (locals.var_chi * (((locals.var_chi_dn14 / 2.0) * assign89960_body30_e138282) + (assign89960_body30_e138271 * (((locals.var_chi_dn14 / 3.0) * assign89960_body30_e138280) + (assign89960_body30_e138275 * (locals.var_chi_dn14 / 4.0))))))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
            locals.var_t1 = assign89960_body30_e138287;
            locals.var_t1_dn0 = assign89960_body30_e138287_d_n0;
            locals.var_t1_dn2 = assign89960_body30_e138287_d_n2;
            locals.var_t1_dn4 = assign89960_body30_e138287_d_n4;
            locals.var_t1_dn5 = assign89960_body30_e138287_d_n5;
            locals.var_t1_dn6 = assign89960_body30_e138287_d_n6;
            locals.var_t1_dn7 = assign89960_body30_e138287_d_n7;
            locals.var_t1_dn8 = assign89960_body30_e138287_d_n8;
            locals.var_t1_dn9 = assign89960_body30_e138287_d_n9;
            locals.var_t1_dn10 = assign89960_body30_e138287_d_n10;
            locals.var_t1_dn11 = assign89960_body30_e138287_d_n11;
            locals.var_t1_dn14 = assign89960_body30_e138287_d_n14;
            locals.var_t1_rv = 0.0;
            let (assign89960_body31_e138297, assign89960_body31_e138297_d_n0, assign89960_body31_e138297_d_n2, assign89960_body31_e138297_d_n4, assign89960_body31_e138297_d_n5, assign89960_body31_e138297_d_n6, assign89960_body31_e138297_d_n7, assign89960_body31_e138297_d_n8, assign89960_body31_e138297_d_n9, assign89960_body31_e138297_d_n10, assign89960_body31_e138297_d_n11, assign89960_body31_e138297_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2089 != 0.0)) && (locals.var_guard2096 != 0.0)) {
        let assign89960_body31_e138295: f64 = (locals.var_cfs1 * locals.var_t0);
        (assign89960_body31_e138295, ((locals.var_cfs1_dn0 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn0)), ((locals.var_cfs1_dn2 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn2)), ((locals.var_cfs1_dn4 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn4)), ((locals.var_cfs1_dn5 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn5)), ((locals.var_cfs1_dn6 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn6)), ((locals.var_cfs1_dn7 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn7)), ((locals.var_cfs1_dn8 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn8)), ((locals.var_cfs1_dn9 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn9)), ((locals.var_cfs1_dn10 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn10)), ((locals.var_cfs1_dn11 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn11)), ((locals.var_cfs1_dn14 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn14)),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn11, locals.var_fs01_dn14,)
    }
};
            locals.var_fs01 = assign89960_body31_e138297;
            locals.var_fs01_dn0 = assign89960_body31_e138297_d_n0;
            locals.var_fs01_dn2 = assign89960_body31_e138297_d_n2;
            locals.var_fs01_dn4 = assign89960_body31_e138297_d_n4;
            locals.var_fs01_dn5 = assign89960_body31_e138297_d_n5;
            locals.var_fs01_dn6 = assign89960_body31_e138297_d_n6;
            locals.var_fs01_dn7 = assign89960_body31_e138297_d_n7;
            locals.var_fs01_dn8 = assign89960_body31_e138297_d_n8;
            locals.var_fs01_dn9 = assign89960_body31_e138297_d_n9;
            locals.var_fs01_dn10 = assign89960_body31_e138297_d_n10;
            locals.var_fs01_dn11 = assign89960_body31_e138297_d_n11;
            locals.var_fs01_dn14 = assign89960_body31_e138297_d_n14;
            locals.var_fs01_rv = 0.0;
            let (assign89960_body32_e138309, assign89960_body32_e138309_d_n0, assign89960_body32_e138309_d_n2, assign89960_body32_e138309_d_n4, assign89960_body32_e138309_d_n5, assign89960_body32_e138309_d_n6, assign89960_body32_e138309_d_n7, assign89960_body32_e138309_d_n8, assign89960_body32_e138309_d_n9, assign89960_body32_e138309_d_n10, assign89960_body32_e138309_d_n11, assign89960_body32_e138309_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2089 != 0.0)) && (locals.var_guard2096 != 0.0)) {
        let assign89960_body32_e138305: f64 = (locals.var_cfs1 * locals.var_t1);
        let assign89960_body32_e138307: f64 = (assign89960_body32_e138305 * locals.var_beta);
        (assign89960_body32_e138307, ((((locals.var_cfs1_dn0 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn0)) * locals.var_beta) + (assign89960_body32_e138305 * locals.var_beta_dn0)), ((((locals.var_cfs1_dn2 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn2)) * locals.var_beta) + (assign89960_body32_e138305 * locals.var_beta_dn2)), ((((locals.var_cfs1_dn4 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn4)) * locals.var_beta) + (assign89960_body32_e138305 * locals.var_beta_dn4)), ((((locals.var_cfs1_dn5 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn5)) * locals.var_beta) + (assign89960_body32_e138305 * locals.var_beta_dn5)), ((((locals.var_cfs1_dn6 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn6)) * locals.var_beta) + (assign89960_body32_e138305 * locals.var_beta_dn6)), ((((locals.var_cfs1_dn7 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn7)) * locals.var_beta) + (assign89960_body32_e138305 * locals.var_beta_dn7)), ((((locals.var_cfs1_dn8 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn8)) * locals.var_beta) + (assign89960_body32_e138305 * locals.var_beta_dn8)), ((((locals.var_cfs1_dn9 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn9)) * locals.var_beta) + (assign89960_body32_e138305 * locals.var_beta_dn9)), ((((locals.var_cfs1_dn10 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn10)) * locals.var_beta) + (assign89960_body32_e138305 * locals.var_beta_dn10)), ((((locals.var_cfs1_dn11 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn11)) * locals.var_beta) + (assign89960_body32_e138305 * locals.var_beta_dn11)), ((((locals.var_cfs1_dn14 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn14)) * locals.var_beta) + (assign89960_body32_e138305 * locals.var_beta_dn14)),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn11, locals.var_fs01_dps0_dn14,)
    }
};
            locals.var_fs01_dps0 = assign89960_body32_e138309;
            locals.var_fs01_dps0_dn0 = assign89960_body32_e138309_d_n0;
            locals.var_fs01_dps0_dn2 = assign89960_body32_e138309_d_n2;
            locals.var_fs01_dps0_dn4 = assign89960_body32_e138309_d_n4;
            locals.var_fs01_dps0_dn5 = assign89960_body32_e138309_d_n5;
            locals.var_fs01_dps0_dn6 = assign89960_body32_e138309_d_n6;
            locals.var_fs01_dps0_dn7 = assign89960_body32_e138309_d_n7;
            locals.var_fs01_dps0_dn8 = assign89960_body32_e138309_d_n8;
            locals.var_fs01_dps0_dn9 = assign89960_body32_e138309_d_n9;
            locals.var_fs01_dps0_dn10 = assign89960_body32_e138309_d_n10;
            locals.var_fs01_dps0_dn11 = assign89960_body32_e138309_d_n11;
            locals.var_fs01_dps0_dn14 = assign89960_body32_e138309_d_n14;
            locals.var_fs01_dps0_rv = 0.0;
            let assign89960_body33_e138311: f64 = (locals.var_chi).abs();
            let assign89960_body33_e138313: f64 = if assign89960_body33_e138311 < 60.0 { 1.0 } else { 0.0 };
            locals.var_guard2097 = assign89960_body33_e138313;
            locals.var_guard2097_rv = 0.0;
            let (assign89960_body35_e138344, assign89960_body35_e138344_d_n0, assign89960_body35_e138344_d_n2, assign89960_body35_e138344_d_n4, assign89960_body35_e138344_d_n5, assign89960_body35_e138344_d_n6, assign89960_body35_e138344_d_n7, assign89960_body35_e138344_d_n8, assign89960_body35_e138344_d_n9, assign89960_body35_e138344_d_n10, assign89960_body35_e138344_d_n11, assign89960_body35_e138344_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2089 != 0.0)) && (locals.var_guard2096 == 0.0)) && (locals.var_guard2097 != 0.0)) {
        let assign89960_body35_e138342: f64 = (locals.var_chi).exp();
        (assign89960_body35_e138342, (assign89960_body35_e138342 * locals.var_chi_dn0), (assign89960_body35_e138342 * locals.var_chi_dn2), (assign89960_body35_e138342 * locals.var_chi_dn4), (assign89960_body35_e138342 * locals.var_chi_dn5), (assign89960_body35_e138342 * locals.var_chi_dn6), (assign89960_body35_e138342 * locals.var_chi_dn7), (assign89960_body35_e138342 * locals.var_chi_dn8), (assign89960_body35_e138342 * locals.var_chi_dn9), (assign89960_body35_e138342 * locals.var_chi_dn10), (assign89960_body35_e138342 * locals.var_chi_dn11), (assign89960_body35_e138342 * locals.var_chi_dn14),)
    } else {
        (locals.var_exp_chi, locals.var_exp_chi_dn0, locals.var_exp_chi_dn2, locals.var_exp_chi_dn4, locals.var_exp_chi_dn5, locals.var_exp_chi_dn6, locals.var_exp_chi_dn7, locals.var_exp_chi_dn8, locals.var_exp_chi_dn9, locals.var_exp_chi_dn10, locals.var_exp_chi_dn11, locals.var_exp_chi_dn14,)
    }
};
            locals.var_exp_chi = assign89960_body35_e138344;
            locals.var_exp_chi_dn0 = assign89960_body35_e138344_d_n0;
            locals.var_exp_chi_dn2 = assign89960_body35_e138344_d_n2;
            locals.var_exp_chi_dn4 = assign89960_body35_e138344_d_n4;
            locals.var_exp_chi_dn5 = assign89960_body35_e138344_d_n5;
            locals.var_exp_chi_dn6 = assign89960_body35_e138344_d_n6;
            locals.var_exp_chi_dn7 = assign89960_body35_e138344_d_n7;
            locals.var_exp_chi_dn8 = assign89960_body35_e138344_d_n8;
            locals.var_exp_chi_dn9 = assign89960_body35_e138344_d_n9;
            locals.var_exp_chi_dn10 = assign89960_body35_e138344_d_n10;
            locals.var_exp_chi_dn11 = assign89960_body35_e138344_d_n11;
            locals.var_exp_chi_dn14 = assign89960_body35_e138344_d_n14;
            locals.var_exp_chi_rv = 0.0;
            let (assign89960_body36_e138357, assign89960_body36_e138357_d_n0, assign89960_body36_e138357_d_n2, assign89960_body36_e138357_d_n4, assign89960_body36_e138357_d_n5, assign89960_body36_e138357_d_n6, assign89960_body36_e138357_d_n7, assign89960_body36_e138357_d_n8, assign89960_body36_e138357_d_n9, assign89960_body36_e138357_d_n10, assign89960_body36_e138357_d_n11, assign89960_body36_e138357_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2089 != 0.0)) && (locals.var_guard2096 == 0.0)) && (locals.var_guard2097 != 0.0)) {
        let assign89960_body36_e138355: f64 = (locals.var_exp_chi - 1.0);
        (assign89960_body36_e138355, locals.var_exp_chi_dn0, locals.var_exp_chi_dn2, locals.var_exp_chi_dn4, locals.var_exp_chi_dn5, locals.var_exp_chi_dn6, locals.var_exp_chi_dn7, locals.var_exp_chi_dn8, locals.var_exp_chi_dn9, locals.var_exp_chi_dn10, locals.var_exp_chi_dn11, locals.var_exp_chi_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
            locals.var_t1 = assign89960_body36_e138357;
            locals.var_t1_dn0 = assign89960_body36_e138357_d_n0;
            locals.var_t1_dn2 = assign89960_body36_e138357_d_n2;
            locals.var_t1_dn4 = assign89960_body36_e138357_d_n4;
            locals.var_t1_dn5 = assign89960_body36_e138357_d_n5;
            locals.var_t1_dn6 = assign89960_body36_e138357_d_n6;
            locals.var_t1_dn7 = assign89960_body36_e138357_d_n7;
            locals.var_t1_dn8 = assign89960_body36_e138357_d_n8;
            locals.var_t1_dn9 = assign89960_body36_e138357_d_n9;
            locals.var_t1_dn10 = assign89960_body36_e138357_d_n10;
            locals.var_t1_dn11 = assign89960_body36_e138357_d_n11;
            locals.var_t1_dn14 = assign89960_body36_e138357_d_n14;
            locals.var_t1_rv = 0.0;
            let (assign89960_body37_e138372, assign89960_body37_e138372_d_n0, assign89960_body37_e138372_d_n2, assign89960_body37_e138372_d_n4, assign89960_body37_e138372_d_n5, assign89960_body37_e138372_d_n6, assign89960_body37_e138372_d_n7, assign89960_body37_e138372_d_n8, assign89960_body37_e138372_d_n9, assign89960_body37_e138372_d_n10, assign89960_body37_e138372_d_n11, assign89960_body37_e138372_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2089 != 0.0)) && (locals.var_guard2096 == 0.0)) && (locals.var_guard2097 != 0.0)) {
        let assign89960_body37_e138369: f64 = (locals.var_t1 - locals.var_chi);
        let assign89960_body37_e138370: f64 = (locals.var_cfs1 * assign89960_body37_e138369);
        (assign89960_body37_e138370, ((locals.var_cfs1_dn0 * assign89960_body37_e138369) + (locals.var_cfs1 * (locals.var_t1_dn0 - locals.var_chi_dn0))), ((locals.var_cfs1_dn2 * assign89960_body37_e138369) + (locals.var_cfs1 * (locals.var_t1_dn2 - locals.var_chi_dn2))), ((locals.var_cfs1_dn4 * assign89960_body37_e138369) + (locals.var_cfs1 * (locals.var_t1_dn4 - locals.var_chi_dn4))), ((locals.var_cfs1_dn5 * assign89960_body37_e138369) + (locals.var_cfs1 * (locals.var_t1_dn5 - locals.var_chi_dn5))), ((locals.var_cfs1_dn6 * assign89960_body37_e138369) + (locals.var_cfs1 * (locals.var_t1_dn6 - locals.var_chi_dn6))), ((locals.var_cfs1_dn7 * assign89960_body37_e138369) + (locals.var_cfs1 * (locals.var_t1_dn7 - locals.var_chi_dn7))), ((locals.var_cfs1_dn8 * assign89960_body37_e138369) + (locals.var_cfs1 * (locals.var_t1_dn8 - locals.var_chi_dn8))), ((locals.var_cfs1_dn9 * assign89960_body37_e138369) + (locals.var_cfs1 * (locals.var_t1_dn9 - locals.var_chi_dn9))), ((locals.var_cfs1_dn10 * assign89960_body37_e138369) + (locals.var_cfs1 * (locals.var_t1_dn10 - locals.var_chi_dn10))), ((locals.var_cfs1_dn11 * assign89960_body37_e138369) + (locals.var_cfs1 * (locals.var_t1_dn11 - locals.var_chi_dn11))), ((locals.var_cfs1_dn14 * assign89960_body37_e138369) + (locals.var_cfs1 * (locals.var_t1_dn14 - locals.var_chi_dn14))),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn11, locals.var_fs01_dn14,)
    }
};
            locals.var_fs01 = assign89960_body37_e138372;
            locals.var_fs01_dn0 = assign89960_body37_e138372_d_n0;
            locals.var_fs01_dn2 = assign89960_body37_e138372_d_n2;
            locals.var_fs01_dn4 = assign89960_body37_e138372_d_n4;
            locals.var_fs01_dn5 = assign89960_body37_e138372_d_n5;
            locals.var_fs01_dn6 = assign89960_body37_e138372_d_n6;
            locals.var_fs01_dn7 = assign89960_body37_e138372_d_n7;
            locals.var_fs01_dn8 = assign89960_body37_e138372_d_n8;
            locals.var_fs01_dn9 = assign89960_body37_e138372_d_n9;
            locals.var_fs01_dn10 = assign89960_body37_e138372_d_n10;
            locals.var_fs01_dn11 = assign89960_body37_e138372_d_n11;
            locals.var_fs01_dn14 = assign89960_body37_e138372_d_n14;
            locals.var_fs01_rv = 0.0;
            let (assign89960_body38_e138387, assign89960_body38_e138387_d_n0, assign89960_body38_e138387_d_n2, assign89960_body38_e138387_d_n4, assign89960_body38_e138387_d_n5, assign89960_body38_e138387_d_n6, assign89960_body38_e138387_d_n7, assign89960_body38_e138387_d_n8, assign89960_body38_e138387_d_n9, assign89960_body38_e138387_d_n10, assign89960_body38_e138387_d_n11, assign89960_body38_e138387_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2089 != 0.0)) && (locals.var_guard2096 == 0.0)) && (locals.var_guard2097 != 0.0)) {
        let assign89960_body38_e138383: f64 = (locals.var_cfs1 * locals.var_beta);
        let assign89960_body38_e138385: f64 = (assign89960_body38_e138383 * locals.var_t1);
        (assign89960_body38_e138385, ((((locals.var_cfs1_dn0 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn0)) * locals.var_t1) + (assign89960_body38_e138383 * locals.var_t1_dn0)), ((((locals.var_cfs1_dn2 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn2)) * locals.var_t1) + (assign89960_body38_e138383 * locals.var_t1_dn2)), ((((locals.var_cfs1_dn4 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn4)) * locals.var_t1) + (assign89960_body38_e138383 * locals.var_t1_dn4)), ((((locals.var_cfs1_dn5 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn5)) * locals.var_t1) + (assign89960_body38_e138383 * locals.var_t1_dn5)), ((((locals.var_cfs1_dn6 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn6)) * locals.var_t1) + (assign89960_body38_e138383 * locals.var_t1_dn6)), ((((locals.var_cfs1_dn7 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn7)) * locals.var_t1) + (assign89960_body38_e138383 * locals.var_t1_dn7)), ((((locals.var_cfs1_dn8 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn8)) * locals.var_t1) + (assign89960_body38_e138383 * locals.var_t1_dn8)), ((((locals.var_cfs1_dn9 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn9)) * locals.var_t1) + (assign89960_body38_e138383 * locals.var_t1_dn9)), ((((locals.var_cfs1_dn10 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn10)) * locals.var_t1) + (assign89960_body38_e138383 * locals.var_t1_dn10)), ((((locals.var_cfs1_dn11 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn11)) * locals.var_t1) + (assign89960_body38_e138383 * locals.var_t1_dn11)), ((((locals.var_cfs1_dn14 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn14)) * locals.var_t1) + (assign89960_body38_e138383 * locals.var_t1_dn14)),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn11, locals.var_fs01_dps0_dn14,)
    }
};
            locals.var_fs01_dps0 = assign89960_body38_e138387;
            locals.var_fs01_dps0_dn0 = assign89960_body38_e138387_d_n0;
            locals.var_fs01_dps0_dn2 = assign89960_body38_e138387_d_n2;
            locals.var_fs01_dps0_dn4 = assign89960_body38_e138387_d_n4;
            locals.var_fs01_dps0_dn5 = assign89960_body38_e138387_d_n5;
            locals.var_fs01_dps0_dn6 = assign89960_body38_e138387_d_n6;
            locals.var_fs01_dps0_dn7 = assign89960_body38_e138387_d_n7;
            locals.var_fs01_dps0_dn8 = assign89960_body38_e138387_d_n8;
            locals.var_fs01_dps0_dn9 = assign89960_body38_e138387_d_n9;
            locals.var_fs01_dps0_dn10 = assign89960_body38_e138387_d_n10;
            locals.var_fs01_dps0_dn11 = assign89960_body38_e138387_d_n11;
            locals.var_fs01_dps0_dn14 = assign89960_body38_e138387_d_n14;
            locals.var_fs01_dps0_rv = 0.0;
            let (assign89960_body40_e138422, assign89960_body40_e138422_d_n0, assign89960_body40_e138422_d_n2, assign89960_body40_e138422_d_n4, assign89960_body40_e138422_d_n5, assign89960_body40_e138422_d_n6, assign89960_body40_e138422_d_n7, assign89960_body40_e138422_d_n8, assign89960_body40_e138422_d_n9, assign89960_body40_e138422_d_n10, assign89960_body40_e138422_d_n11, assign89960_body40_e138422_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2089 != 0.0)) && (locals.var_guard2096 == 0.0)) && (locals.var_guard2097 == 0.0)) {
        let assign89960_body40_e138419: f64 = (locals.var_beta * locals.var_ps0ld);
        let assign89960_body40_e138420: f64 = (assign89960_body40_e138419).exp();
        (assign89960_body40_e138420, (assign89960_body40_e138420 * ((locals.var_beta_dn0 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn0))), (assign89960_body40_e138420 * ((locals.var_beta_dn2 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn2))), (assign89960_body40_e138420 * ((locals.var_beta_dn4 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn4))), (assign89960_body40_e138420 * ((locals.var_beta_dn5 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn5))), (assign89960_body40_e138420 * ((locals.var_beta_dn6 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn6))), (assign89960_body40_e138420 * ((locals.var_beta_dn7 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn7))), (assign89960_body40_e138420 * ((locals.var_beta_dn8 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn8))), (assign89960_body40_e138420 * ((locals.var_beta_dn9 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn9))), (assign89960_body40_e138420 * ((locals.var_beta_dn10 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn10))), (assign89960_body40_e138420 * ((locals.var_beta_dn11 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn11))), (assign89960_body40_e138420 * ((locals.var_beta_dn14 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn14))),)
    } else {
        (locals.var_exp_bps0, locals.var_exp_bps0_dn0, locals.var_exp_bps0_dn2, locals.var_exp_bps0_dn4, locals.var_exp_bps0_dn5, locals.var_exp_bps0_dn6, locals.var_exp_bps0_dn7, locals.var_exp_bps0_dn8, locals.var_exp_bps0_dn9, locals.var_exp_bps0_dn10, locals.var_exp_bps0_dn11, locals.var_exp_bps0_dn14,)
    }
};
            locals.var_exp_bps0 = assign89960_body40_e138422;
            locals.var_exp_bps0_dn0 = assign89960_body40_e138422_d_n0;
            locals.var_exp_bps0_dn2 = assign89960_body40_e138422_d_n2;
            locals.var_exp_bps0_dn4 = assign89960_body40_e138422_d_n4;
            locals.var_exp_bps0_dn5 = assign89960_body40_e138422_d_n5;
            locals.var_exp_bps0_dn6 = assign89960_body40_e138422_d_n6;
            locals.var_exp_bps0_dn7 = assign89960_body40_e138422_d_n7;
            locals.var_exp_bps0_dn8 = assign89960_body40_e138422_d_n8;
            locals.var_exp_bps0_dn9 = assign89960_body40_e138422_d_n9;
            locals.var_exp_bps0_dn10 = assign89960_body40_e138422_d_n10;
            locals.var_exp_bps0_dn11 = assign89960_body40_e138422_d_n11;
            locals.var_exp_bps0_dn14 = assign89960_body40_e138422_d_n14;
            locals.var_exp_bps0_rv = 0.0;
            let (assign89960_body41_e138442, assign89960_body41_e138442_d_n0, assign89960_body41_e138442_d_n2, assign89960_body41_e138442_d_n4, assign89960_body41_e138442_d_n5, assign89960_body41_e138442_d_n6, assign89960_body41_e138442_d_n7, assign89960_body41_e138442_d_n8, assign89960_body41_e138442_d_n9, assign89960_body41_e138442_d_n10, assign89960_body41_e138442_d_n11, assign89960_body41_e138442_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2089 != 0.0)) && (locals.var_guard2096 == 0.0)) && (locals.var_guard2097 == 0.0)) {
        let assign89960_body41_e138437: f64 = (locals.var_chi + 1.0);
        let assign89960_body41_e138438: f64 = (locals.var_exp_bvbs * assign89960_body41_e138437);
        let assign89960_body41_e138439: f64 = (locals.var_exp_bps0 - assign89960_body41_e138438);
        let assign89960_body41_e138440: f64 = (locals.var_cnst1over * assign89960_body41_e138439);
        (assign89960_body41_e138440, ((locals.var_cnst1over_dn0 * assign89960_body41_e138439) + (locals.var_cnst1over * (locals.var_exp_bps0_dn0 - ((locals.var_exp_bvbs_dn0 * assign89960_body41_e138437) + (locals.var_exp_bvbs * locals.var_chi_dn0))))), ((locals.var_cnst1over_dn2 * assign89960_body41_e138439) + (locals.var_cnst1over * (locals.var_exp_bps0_dn2 - ((locals.var_exp_bvbs_dn2 * assign89960_body41_e138437) + (locals.var_exp_bvbs * locals.var_chi_dn2))))), ((locals.var_cnst1over_dn4 * assign89960_body41_e138439) + (locals.var_cnst1over * (locals.var_exp_bps0_dn4 - ((locals.var_exp_bvbs_dn4 * assign89960_body41_e138437) + (locals.var_exp_bvbs * locals.var_chi_dn4))))), ((locals.var_cnst1over_dn5 * assign89960_body41_e138439) + (locals.var_cnst1over * (locals.var_exp_bps0_dn5 - ((locals.var_exp_bvbs_dn5 * assign89960_body41_e138437) + (locals.var_exp_bvbs * locals.var_chi_dn5))))), ((locals.var_cnst1over_dn6 * assign89960_body41_e138439) + (locals.var_cnst1over * (locals.var_exp_bps0_dn6 - ((locals.var_exp_bvbs_dn6 * assign89960_body41_e138437) + (locals.var_exp_bvbs * locals.var_chi_dn6))))), ((locals.var_cnst1over_dn7 * assign89960_body41_e138439) + (locals.var_cnst1over * (locals.var_exp_bps0_dn7 - ((locals.var_exp_bvbs_dn7 * assign89960_body41_e138437) + (locals.var_exp_bvbs * locals.var_chi_dn7))))), ((locals.var_cnst1over_dn8 * assign89960_body41_e138439) + (locals.var_cnst1over * (locals.var_exp_bps0_dn8 - ((locals.var_exp_bvbs_dn8 * assign89960_body41_e138437) + (locals.var_exp_bvbs * locals.var_chi_dn8))))), ((locals.var_cnst1over_dn9 * assign89960_body41_e138439) + (locals.var_cnst1over * (locals.var_exp_bps0_dn9 - ((locals.var_exp_bvbs_dn9 * assign89960_body41_e138437) + (locals.var_exp_bvbs * locals.var_chi_dn9))))), ((locals.var_cnst1over_dn10 * assign89960_body41_e138439) + (locals.var_cnst1over * (locals.var_exp_bps0_dn10 - ((locals.var_exp_bvbs_dn10 * assign89960_body41_e138437) + (locals.var_exp_bvbs * locals.var_chi_dn10))))), ((locals.var_cnst1over_dn11 * assign89960_body41_e138439) + (locals.var_cnst1over * (locals.var_exp_bps0_dn11 - ((locals.var_exp_bvbs_dn11 * assign89960_body41_e138437) + (locals.var_exp_bvbs * locals.var_chi_dn11))))), ((locals.var_cnst1over_dn14 * assign89960_body41_e138439) + (locals.var_cnst1over * (locals.var_exp_bps0_dn14 - ((locals.var_exp_bvbs_dn14 * assign89960_body41_e138437) + (locals.var_exp_bvbs * locals.var_chi_dn14))))),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn11, locals.var_fs01_dn14,)
    }
};
            locals.var_fs01 = assign89960_body41_e138442;
            locals.var_fs01_dn0 = assign89960_body41_e138442_d_n0;
            locals.var_fs01_dn2 = assign89960_body41_e138442_d_n2;
            locals.var_fs01_dn4 = assign89960_body41_e138442_d_n4;
            locals.var_fs01_dn5 = assign89960_body41_e138442_d_n5;
            locals.var_fs01_dn6 = assign89960_body41_e138442_d_n6;
            locals.var_fs01_dn7 = assign89960_body41_e138442_d_n7;
            locals.var_fs01_dn8 = assign89960_body41_e138442_d_n8;
            locals.var_fs01_dn9 = assign89960_body41_e138442_d_n9;
            locals.var_fs01_dn10 = assign89960_body41_e138442_d_n10;
            locals.var_fs01_dn11 = assign89960_body41_e138442_d_n11;
            locals.var_fs01_dn14 = assign89960_body41_e138442_d_n14;
            locals.var_fs01_rv = 0.0;
            let (assign89960_body42_e138460, assign89960_body42_e138460_d_n0, assign89960_body42_e138460_d_n2, assign89960_body42_e138460_d_n4, assign89960_body42_e138460_d_n5, assign89960_body42_e138460_d_n6, assign89960_body42_e138460_d_n7, assign89960_body42_e138460_d_n8, assign89960_body42_e138460_d_n9, assign89960_body42_e138460_d_n10, assign89960_body42_e138460_d_n11, assign89960_body42_e138460_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2089 != 0.0)) && (locals.var_guard2096 == 0.0)) && (locals.var_guard2097 == 0.0)) {
        let assign89960_body42_e138454: f64 = (locals.var_cnst1over * locals.var_beta);
        let assign89960_body42_e138457: f64 = (locals.var_exp_bps0 - locals.var_exp_bvbs);
        let assign89960_body42_e138458: f64 = (assign89960_body42_e138454 * assign89960_body42_e138457);
        (assign89960_body42_e138458, ((((locals.var_cnst1over_dn0 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn0)) * assign89960_body42_e138457) + (assign89960_body42_e138454 * (locals.var_exp_bps0_dn0 - locals.var_exp_bvbs_dn0))), ((((locals.var_cnst1over_dn2 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn2)) * assign89960_body42_e138457) + (assign89960_body42_e138454 * (locals.var_exp_bps0_dn2 - locals.var_exp_bvbs_dn2))), ((((locals.var_cnst1over_dn4 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn4)) * assign89960_body42_e138457) + (assign89960_body42_e138454 * (locals.var_exp_bps0_dn4 - locals.var_exp_bvbs_dn4))), ((((locals.var_cnst1over_dn5 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn5)) * assign89960_body42_e138457) + (assign89960_body42_e138454 * (locals.var_exp_bps0_dn5 - locals.var_exp_bvbs_dn5))), ((((locals.var_cnst1over_dn6 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn6)) * assign89960_body42_e138457) + (assign89960_body42_e138454 * (locals.var_exp_bps0_dn6 - locals.var_exp_bvbs_dn6))), ((((locals.var_cnst1over_dn7 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn7)) * assign89960_body42_e138457) + (assign89960_body42_e138454 * (locals.var_exp_bps0_dn7 - locals.var_exp_bvbs_dn7))), ((((locals.var_cnst1over_dn8 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn8)) * assign89960_body42_e138457) + (assign89960_body42_e138454 * (locals.var_exp_bps0_dn8 - locals.var_exp_bvbs_dn8))), ((((locals.var_cnst1over_dn9 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn9)) * assign89960_body42_e138457) + (assign89960_body42_e138454 * (locals.var_exp_bps0_dn9 - locals.var_exp_bvbs_dn9))), ((((locals.var_cnst1over_dn10 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn10)) * assign89960_body42_e138457) + (assign89960_body42_e138454 * (locals.var_exp_bps0_dn10 - locals.var_exp_bvbs_dn10))), ((((locals.var_cnst1over_dn11 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn11)) * assign89960_body42_e138457) + (assign89960_body42_e138454 * (locals.var_exp_bps0_dn11 - locals.var_exp_bvbs_dn11))), ((((locals.var_cnst1over_dn14 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn14)) * assign89960_body42_e138457) + (assign89960_body42_e138454 * (locals.var_exp_bps0_dn14 - locals.var_exp_bvbs_dn14))),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn11, locals.var_fs01_dps0_dn14,)
    }
};
            locals.var_fs01_dps0 = assign89960_body42_e138460;
            locals.var_fs01_dps0_dn0 = assign89960_body42_e138460_d_n0;
            locals.var_fs01_dps0_dn2 = assign89960_body42_e138460_d_n2;
            locals.var_fs01_dps0_dn4 = assign89960_body42_e138460_d_n4;
            locals.var_fs01_dps0_dn5 = assign89960_body42_e138460_d_n5;
            locals.var_fs01_dps0_dn6 = assign89960_body42_e138460_d_n6;
            locals.var_fs01_dps0_dn7 = assign89960_body42_e138460_d_n7;
            locals.var_fs01_dps0_dn8 = assign89960_body42_e138460_d_n8;
            locals.var_fs01_dps0_dn9 = assign89960_body42_e138460_d_n9;
            locals.var_fs01_dps0_dn10 = assign89960_body42_e138460_d_n10;
            locals.var_fs01_dps0_dn11 = assign89960_body42_e138460_d_n11;
            locals.var_fs01_dps0_dn14 = assign89960_body42_e138460_d_n14;
            locals.var_fs01_dps0_rv = 0.0;
            let assign89960_body43_e138463: f64 = if locals.var_fs01 > 0.0 { 1.0 } else { 0.0 };
            locals.var_guard2098 = assign89960_body43_e138463;
            locals.var_guard2098_rv = 0.0;
            let (assign89960_body44_e138474, assign89960_body44_e138474_d_n0, assign89960_body44_e138474_d_n2, assign89960_body44_e138474_d_n4, assign89960_body44_e138474_d_n5, assign89960_body44_e138474_d_n6, assign89960_body44_e138474_d_n7, assign89960_body44_e138474_d_n8, assign89960_body44_e138474_d_n9, assign89960_body44_e138474_d_n10, assign89960_body44_e138474_d_n11, assign89960_body44_e138474_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2089 != 0.0)) && (locals.var_guard2098 != 0.0)) {
        let assign89960_body44_e138471: f64 = (locals.var_fbsq__blk2016 + locals.var_fs01);
        let assign89960_body44_e138472: f64 = (assign89960_body44_e138471).sqrt();
        (assign89960_body44_e138472, ((locals.var_fbsq__blk2016_dn0 + locals.var_fs01_dn0) / (2.0 * assign89960_body44_e138472)), ((locals.var_fbsq__blk2016_dn2 + locals.var_fs01_dn2) / (2.0 * assign89960_body44_e138472)), ((locals.var_fbsq__blk2016_dn4 + locals.var_fs01_dn4) / (2.0 * assign89960_body44_e138472)), ((locals.var_fbsq__blk2016_dn5 + locals.var_fs01_dn5) / (2.0 * assign89960_body44_e138472)), ((locals.var_fbsq__blk2016_dn6 + locals.var_fs01_dn6) / (2.0 * assign89960_body44_e138472)), ((locals.var_fbsq__blk2016_dn7 + locals.var_fs01_dn7) / (2.0 * assign89960_body44_e138472)), ((locals.var_fbsq__blk2016_dn8 + locals.var_fs01_dn8) / (2.0 * assign89960_body44_e138472)), ((locals.var_fbsq__blk2016_dn9 + locals.var_fs01_dn9) / (2.0 * assign89960_body44_e138472)), ((locals.var_fbsq__blk2016_dn10 + locals.var_fs01_dn10) / (2.0 * assign89960_body44_e138472)), ((locals.var_fbsq__blk2016_dn11 + locals.var_fs01_dn11) / (2.0 * assign89960_body44_e138472)), ((locals.var_fbsq__blk2016_dn14 + locals.var_fs01_dn14) / (2.0 * assign89960_body44_e138472)),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn11, locals.var_fs02_dn14,)
    }
};
            locals.var_fs02 = assign89960_body44_e138474;
            locals.var_fs02_dn0 = assign89960_body44_e138474_d_n0;
            locals.var_fs02_dn2 = assign89960_body44_e138474_d_n2;
            locals.var_fs02_dn4 = assign89960_body44_e138474_d_n4;
            locals.var_fs02_dn5 = assign89960_body44_e138474_d_n5;
            locals.var_fs02_dn6 = assign89960_body44_e138474_d_n6;
            locals.var_fs02_dn7 = assign89960_body44_e138474_d_n7;
            locals.var_fs02_dn8 = assign89960_body44_e138474_d_n8;
            locals.var_fs02_dn9 = assign89960_body44_e138474_d_n9;
            locals.var_fs02_dn10 = assign89960_body44_e138474_d_n10;
            locals.var_fs02_dn11 = assign89960_body44_e138474_d_n11;
            locals.var_fs02_dn14 = assign89960_body44_e138474_d_n14;
            locals.var_fs02_rv = 0.0;
            let (assign89960_body45_e138488, assign89960_body45_e138488_d_n0, assign89960_body45_e138488_d_n2, assign89960_body45_e138488_d_n4, assign89960_body45_e138488_d_n5, assign89960_body45_e138488_d_n6, assign89960_body45_e138488_d_n7, assign89960_body45_e138488_d_n8, assign89960_body45_e138488_d_n9, assign89960_body45_e138488_d_n10, assign89960_body45_e138488_d_n11, assign89960_body45_e138488_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2089 != 0.0)) && (locals.var_guard2098 != 0.0)) {
        let assign89960_body45_e138483: f64 = (locals.var_fbsq_dpss__blk2017 + locals.var_fs01_dps0);
        let assign89960_body45_e138484: f64 = (0.5 * assign89960_body45_e138483);
        let assign89960_body45_e138486: f64 = (assign89960_body45_e138484 / locals.var_fs02);
        (assign89960_body45_e138486, ((((0.5 * (locals.var_fbsq_dpss__blk2017_dn0 + locals.var_fs01_dps0_dn0)) * locals.var_fs02) - (assign89960_body45_e138484 * locals.var_fs02_dn0)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk2017_dn2 + locals.var_fs01_dps0_dn2)) * locals.var_fs02) - (assign89960_body45_e138484 * locals.var_fs02_dn2)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk2017_dn4 + locals.var_fs01_dps0_dn4)) * locals.var_fs02) - (assign89960_body45_e138484 * locals.var_fs02_dn4)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk2017_dn5 + locals.var_fs01_dps0_dn5)) * locals.var_fs02) - (assign89960_body45_e138484 * locals.var_fs02_dn5)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk2017_dn6 + locals.var_fs01_dps0_dn6)) * locals.var_fs02) - (assign89960_body45_e138484 * locals.var_fs02_dn6)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk2017_dn7 + locals.var_fs01_dps0_dn7)) * locals.var_fs02) - (assign89960_body45_e138484 * locals.var_fs02_dn7)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk2017_dn8 + locals.var_fs01_dps0_dn8)) * locals.var_fs02) - (assign89960_body45_e138484 * locals.var_fs02_dn8)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk2017_dn9 + locals.var_fs01_dps0_dn9)) * locals.var_fs02) - (assign89960_body45_e138484 * locals.var_fs02_dn9)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk2017_dn10 + locals.var_fs01_dps0_dn10)) * locals.var_fs02) - (assign89960_body45_e138484 * locals.var_fs02_dn10)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk2017_dn11 + locals.var_fs01_dps0_dn11)) * locals.var_fs02) - (assign89960_body45_e138484 * locals.var_fs02_dn11)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk2017_dn14 + locals.var_fs01_dps0_dn14)) * locals.var_fs02) - (assign89960_body45_e138484 * locals.var_fs02_dn14)) / (locals.var_fs02 * locals.var_fs02)),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn11, locals.var_fs02_dps0_dn14,)
    }
};
            locals.var_fs02_dps0 = assign89960_body45_e138488;
            locals.var_fs02_dps0_dn0 = assign89960_body45_e138488_d_n0;
            locals.var_fs02_dps0_dn2 = assign89960_body45_e138488_d_n2;
            locals.var_fs02_dps0_dn4 = assign89960_body45_e138488_d_n4;
            locals.var_fs02_dps0_dn5 = assign89960_body45_e138488_d_n5;
            locals.var_fs02_dps0_dn6 = assign89960_body45_e138488_d_n6;
            locals.var_fs02_dps0_dn7 = assign89960_body45_e138488_d_n7;
            locals.var_fs02_dps0_dn8 = assign89960_body45_e138488_d_n8;
            locals.var_fs02_dps0_dn9 = assign89960_body45_e138488_d_n9;
            locals.var_fs02_dps0_dn10 = assign89960_body45_e138488_d_n10;
            locals.var_fs02_dps0_dn11 = assign89960_body45_e138488_d_n11;
            locals.var_fs02_dps0_dn14 = assign89960_body45_e138488_d_n14;
            locals.var_fs02_dps0_rv = 0.0;
            let assign89960_body46_e138491: f64 = if locals.var_fbsq__blk2016 > 0.0 { 1.0 } else { 0.0 };
            locals.var_guard2099 = assign89960_body46_e138491;
            locals.var_guard2099_rv = 0.0;
            let (assign89960_body47_e138503, assign89960_body47_e138503_d_n0, assign89960_body47_e138503_d_n2, assign89960_body47_e138503_d_n4, assign89960_body47_e138503_d_n5, assign89960_body47_e138503_d_n6, assign89960_body47_e138503_d_n7, assign89960_body47_e138503_d_n8, assign89960_body47_e138503_d_n9, assign89960_body47_e138503_d_n10, assign89960_body47_e138503_d_n11, assign89960_body47_e138503_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2089 != 0.0)) && (locals.var_guard2098 == 0.0)) && (locals.var_guard2099 != 0.0)) {
        let assign89960_body47_e138501: f64 = (locals.var_fbsq__blk2016).sqrt();
        (assign89960_body47_e138501, (locals.var_fbsq__blk2016_dn0 / (2.0 * assign89960_body47_e138501)), (locals.var_fbsq__blk2016_dn2 / (2.0 * assign89960_body47_e138501)), (locals.var_fbsq__blk2016_dn4 / (2.0 * assign89960_body47_e138501)), (locals.var_fbsq__blk2016_dn5 / (2.0 * assign89960_body47_e138501)), (locals.var_fbsq__blk2016_dn6 / (2.0 * assign89960_body47_e138501)), (locals.var_fbsq__blk2016_dn7 / (2.0 * assign89960_body47_e138501)), (locals.var_fbsq__blk2016_dn8 / (2.0 * assign89960_body47_e138501)), (locals.var_fbsq__blk2016_dn9 / (2.0 * assign89960_body47_e138501)), (locals.var_fbsq__blk2016_dn10 / (2.0 * assign89960_body47_e138501)), (locals.var_fbsq__blk2016_dn11 / (2.0 * assign89960_body47_e138501)), (locals.var_fbsq__blk2016_dn14 / (2.0 * assign89960_body47_e138501)),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn11, locals.var_fs02_dn14,)
    }
};
            locals.var_fs02 = assign89960_body47_e138503;
            locals.var_fs02_dn0 = assign89960_body47_e138503_d_n0;
            locals.var_fs02_dn2 = assign89960_body47_e138503_d_n2;
            locals.var_fs02_dn4 = assign89960_body47_e138503_d_n4;
            locals.var_fs02_dn5 = assign89960_body47_e138503_d_n5;
            locals.var_fs02_dn6 = assign89960_body47_e138503_d_n6;
            locals.var_fs02_dn7 = assign89960_body47_e138503_d_n7;
            locals.var_fs02_dn8 = assign89960_body47_e138503_d_n8;
            locals.var_fs02_dn9 = assign89960_body47_e138503_d_n9;
            locals.var_fs02_dn10 = assign89960_body47_e138503_d_n10;
            locals.var_fs02_dn11 = assign89960_body47_e138503_d_n11;
            locals.var_fs02_dn14 = assign89960_body47_e138503_d_n14;
            locals.var_fs02_rv = 0.0;
            let (assign89960_body48_e138518, assign89960_body48_e138518_d_n0, assign89960_body48_e138518_d_n2, assign89960_body48_e138518_d_n4, assign89960_body48_e138518_d_n5, assign89960_body48_e138518_d_n6, assign89960_body48_e138518_d_n7, assign89960_body48_e138518_d_n8, assign89960_body48_e138518_d_n9, assign89960_body48_e138518_d_n10, assign89960_body48_e138518_d_n11, assign89960_body48_e138518_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2089 != 0.0)) && (locals.var_guard2098 == 0.0)) && (locals.var_guard2099 != 0.0)) {
        let assign89960_body48_e138514: f64 = (0.5 * locals.var_fbsq_dpss__blk2017);
        let assign89960_body48_e138516: f64 = (assign89960_body48_e138514 / locals.var_fs02);
        (assign89960_body48_e138516, ((((0.5 * locals.var_fbsq_dpss__blk2017_dn0) * locals.var_fs02) - (assign89960_body48_e138514 * locals.var_fs02_dn0)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk2017_dn2) * locals.var_fs02) - (assign89960_body48_e138514 * locals.var_fs02_dn2)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk2017_dn4) * locals.var_fs02) - (assign89960_body48_e138514 * locals.var_fs02_dn4)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk2017_dn5) * locals.var_fs02) - (assign89960_body48_e138514 * locals.var_fs02_dn5)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk2017_dn6) * locals.var_fs02) - (assign89960_body48_e138514 * locals.var_fs02_dn6)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk2017_dn7) * locals.var_fs02) - (assign89960_body48_e138514 * locals.var_fs02_dn7)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk2017_dn8) * locals.var_fs02) - (assign89960_body48_e138514 * locals.var_fs02_dn8)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk2017_dn9) * locals.var_fs02) - (assign89960_body48_e138514 * locals.var_fs02_dn9)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk2017_dn10) * locals.var_fs02) - (assign89960_body48_e138514 * locals.var_fs02_dn10)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk2017_dn11) * locals.var_fs02) - (assign89960_body48_e138514 * locals.var_fs02_dn11)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk2017_dn14) * locals.var_fs02) - (assign89960_body48_e138514 * locals.var_fs02_dn14)) / (locals.var_fs02 * locals.var_fs02)),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn11, locals.var_fs02_dps0_dn14,)
    }
};
            locals.var_fs02_dps0 = assign89960_body48_e138518;
            locals.var_fs02_dps0_dn0 = assign89960_body48_e138518_d_n0;
            locals.var_fs02_dps0_dn2 = assign89960_body48_e138518_d_n2;
            locals.var_fs02_dps0_dn4 = assign89960_body48_e138518_d_n4;
            locals.var_fs02_dps0_dn5 = assign89960_body48_e138518_d_n5;
            locals.var_fs02_dps0_dn6 = assign89960_body48_e138518_d_n6;
            locals.var_fs02_dps0_dn7 = assign89960_body48_e138518_d_n7;
            locals.var_fs02_dps0_dn8 = assign89960_body48_e138518_d_n8;
            locals.var_fs02_dps0_dn9 = assign89960_body48_e138518_d_n9;
            locals.var_fs02_dps0_dn10 = assign89960_body48_e138518_d_n10;
            locals.var_fs02_dps0_dn11 = assign89960_body48_e138518_d_n11;
            locals.var_fs02_dps0_dn14 = assign89960_body48_e138518_d_n14;
            locals.var_fs02_dps0_rv = 0.0;
            let (assign89960_body49_e138530, assign89960_body49_e138530_d_n0, assign89960_body49_e138530_d_n2, assign89960_body49_e138530_d_n4, assign89960_body49_e138530_d_n5, assign89960_body49_e138530_d_n6, assign89960_body49_e138530_d_n7, assign89960_body49_e138530_d_n8, assign89960_body49_e138530_d_n9, assign89960_body49_e138530_d_n10, assign89960_body49_e138530_d_n11, assign89960_body49_e138530_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2089 != 0.0)) && (locals.var_guard2098 == 0.0)) && (locals.var_guard2099 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn11, locals.var_fs02_dn14,)
    }
};
            locals.var_fs02 = assign89960_body49_e138530;
            locals.var_fs02_dn0 = assign89960_body49_e138530_d_n0;
            locals.var_fs02_dn2 = assign89960_body49_e138530_d_n2;
            locals.var_fs02_dn4 = assign89960_body49_e138530_d_n4;
            locals.var_fs02_dn5 = assign89960_body49_e138530_d_n5;
            locals.var_fs02_dn6 = assign89960_body49_e138530_d_n6;
            locals.var_fs02_dn7 = assign89960_body49_e138530_d_n7;
            locals.var_fs02_dn8 = assign89960_body49_e138530_d_n8;
            locals.var_fs02_dn9 = assign89960_body49_e138530_d_n9;
            locals.var_fs02_dn10 = assign89960_body49_e138530_d_n10;
            locals.var_fs02_dn11 = assign89960_body49_e138530_d_n11;
            locals.var_fs02_dn14 = assign89960_body49_e138530_d_n14;
            locals.var_fs02_rv = 0.0;
            let (assign89960_body50_e138542, assign89960_body50_e138542_d_n0, assign89960_body50_e138542_d_n2, assign89960_body50_e138542_d_n4, assign89960_body50_e138542_d_n5, assign89960_body50_e138542_d_n6, assign89960_body50_e138542_d_n7, assign89960_body50_e138542_d_n8, assign89960_body50_e138542_d_n9, assign89960_body50_e138542_d_n10, assign89960_body50_e138542_d_n11, assign89960_body50_e138542_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2089 != 0.0)) && (locals.var_guard2098 == 0.0)) && (locals.var_guard2099 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn11, locals.var_fs02_dps0_dn14,)
    }
};
            locals.var_fs02_dps0 = assign89960_body50_e138542;
            locals.var_fs02_dps0_dn0 = assign89960_body50_e138542_d_n0;
            locals.var_fs02_dps0_dn2 = assign89960_body50_e138542_d_n2;
            locals.var_fs02_dps0_dn4 = assign89960_body50_e138542_d_n4;
            locals.var_fs02_dps0_dn5 = assign89960_body50_e138542_d_n5;
            locals.var_fs02_dps0_dn6 = assign89960_body50_e138542_d_n6;
            locals.var_fs02_dps0_dn7 = assign89960_body50_e138542_d_n7;
            locals.var_fs02_dps0_dn8 = assign89960_body50_e138542_d_n8;
            locals.var_fs02_dps0_dn9 = assign89960_body50_e138542_d_n9;
            locals.var_fs02_dps0_dn10 = assign89960_body50_e138542_d_n10;
            locals.var_fs02_dps0_dn11 = assign89960_body50_e138542_d_n11;
            locals.var_fs02_dps0_dn14 = assign89960_body50_e138542_d_n14;
            locals.var_fs02_dps0_rv = 0.0;
            let (assign89960_body51_e138556, assign89960_body51_e138556_d_n0, assign89960_body51_e138556_d_n2, assign89960_body51_e138556_d_n4, assign89960_body51_e138556_d_n5, assign89960_body51_e138556_d_n6, assign89960_body51_e138556_d_n7, assign89960_body51_e138556_d_n8, assign89960_body51_e138556_d_n9, assign89960_body51_e138556_d_n10, assign89960_body51_e138556_d_n11, assign89960_body51_e138556_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2089 != 0.0)) {
        let (assign89960_body51_e138552,) = {
            if (locals.var_chi >= 0.0) {
                (1.0,)
            } else {
                let assign89960_body51_e138551: f64 = (-1.0);
                (assign89960_body51_e138551,)
            }
        };
        let assign89960_body51_e138554: f64 = (assign89960_body51_e138552 * locals.var_fs02);
        (assign89960_body51_e138554, (assign89960_body51_e138552 * locals.var_fs02_dn0), (assign89960_body51_e138552 * locals.var_fs02_dn2), (assign89960_body51_e138552 * locals.var_fs02_dn4), (assign89960_body51_e138552 * locals.var_fs02_dn5), (assign89960_body51_e138552 * locals.var_fs02_dn6), (assign89960_body51_e138552 * locals.var_fs02_dn7), (assign89960_body51_e138552 * locals.var_fs02_dn8), (assign89960_body51_e138552 * locals.var_fs02_dn9), (assign89960_body51_e138552 * locals.var_fs02_dn10), (assign89960_body51_e138552 * locals.var_fs02_dn11), (assign89960_body51_e138552 * locals.var_fs02_dn14),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn11, locals.var_fs02_dn14,)
    }
};
            locals.var_fs02 = assign89960_body51_e138556;
            locals.var_fs02_dn0 = assign89960_body51_e138556_d_n0;
            locals.var_fs02_dn2 = assign89960_body51_e138556_d_n2;
            locals.var_fs02_dn4 = assign89960_body51_e138556_d_n4;
            locals.var_fs02_dn5 = assign89960_body51_e138556_d_n5;
            locals.var_fs02_dn6 = assign89960_body51_e138556_d_n6;
            locals.var_fs02_dn7 = assign89960_body51_e138556_d_n7;
            locals.var_fs02_dn8 = assign89960_body51_e138556_d_n8;
            locals.var_fs02_dn9 = assign89960_body51_e138556_d_n9;
            locals.var_fs02_dn10 = assign89960_body51_e138556_d_n10;
            locals.var_fs02_dn11 = assign89960_body51_e138556_d_n11;
            locals.var_fs02_dn14 = assign89960_body51_e138556_d_n14;
            locals.var_fs02_rv = 0.0;
            let (assign89960_body52_e138570, assign89960_body52_e138570_d_n0, assign89960_body52_e138570_d_n2, assign89960_body52_e138570_d_n4, assign89960_body52_e138570_d_n5, assign89960_body52_e138570_d_n6, assign89960_body52_e138570_d_n7, assign89960_body52_e138570_d_n8, assign89960_body52_e138570_d_n9, assign89960_body52_e138570_d_n10, assign89960_body52_e138570_d_n11, assign89960_body52_e138570_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2089 != 0.0)) {
        let (assign89960_body52_e138566,) = {
            if (locals.var_chi >= 0.0) {
                (1.0,)
            } else {
                let assign89960_body52_e138565: f64 = (-1.0);
                (assign89960_body52_e138565,)
            }
        };
        let assign89960_body52_e138568: f64 = (assign89960_body52_e138566 * locals.var_fs02_dps0);
        (assign89960_body52_e138568, (assign89960_body52_e138566 * locals.var_fs02_dps0_dn0), (assign89960_body52_e138566 * locals.var_fs02_dps0_dn2), (assign89960_body52_e138566 * locals.var_fs02_dps0_dn4), (assign89960_body52_e138566 * locals.var_fs02_dps0_dn5), (assign89960_body52_e138566 * locals.var_fs02_dps0_dn6), (assign89960_body52_e138566 * locals.var_fs02_dps0_dn7), (assign89960_body52_e138566 * locals.var_fs02_dps0_dn8), (assign89960_body52_e138566 * locals.var_fs02_dps0_dn9), (assign89960_body52_e138566 * locals.var_fs02_dps0_dn10), (assign89960_body52_e138566 * locals.var_fs02_dps0_dn11), (assign89960_body52_e138566 * locals.var_fs02_dps0_dn14),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn11, locals.var_fs02_dps0_dn14,)
    }
};
            locals.var_fs02_dps0 = assign89960_body52_e138570;
            locals.var_fs02_dps0_dn0 = assign89960_body52_e138570_d_n0;
            locals.var_fs02_dps0_dn2 = assign89960_body52_e138570_d_n2;
            locals.var_fs02_dps0_dn4 = assign89960_body52_e138570_d_n4;
            locals.var_fs02_dps0_dn5 = assign89960_body52_e138570_d_n5;
            locals.var_fs02_dps0_dn6 = assign89960_body52_e138570_d_n6;
            locals.var_fs02_dps0_dn7 = assign89960_body52_e138570_d_n7;
            locals.var_fs02_dps0_dn8 = assign89960_body52_e138570_d_n8;
            locals.var_fs02_dps0_dn9 = assign89960_body52_e138570_d_n9;
            locals.var_fs02_dps0_dn10 = assign89960_body52_e138570_d_n10;
            locals.var_fs02_dps0_dn11 = assign89960_body52_e138570_d_n11;
            locals.var_fs02_dps0_dn14 = assign89960_body52_e138570_d_n14;
            locals.var_fs02_dps0_rv = 0.0;
            let (assign89960_body53_e138583, assign89960_body53_e138583_d_n0, assign89960_body53_e138583_d_n2, assign89960_body53_e138583_d_n4, assign89960_body53_e138583_d_n5, assign89960_body53_e138583_d_n6, assign89960_body53_e138583_d_n7, assign89960_body53_e138583_d_n8, assign89960_body53_e138583_d_n9, assign89960_body53_e138583_d_n10, assign89960_body53_e138583_d_n11, assign89960_body53_e138583_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2089 != 0.0)) {
        let assign89960_body53_e138575: f64 = (-locals.var_vgpld);
        let assign89960_body53_e138577: f64 = (assign89960_body53_e138575 + locals.var_ps0ld);
        let assign89960_body53_e138580: f64 = (locals.var_fac1 * locals.var_fs02);
        let assign89960_body53_e138581: f64 = (assign89960_body53_e138577 + assign89960_body53_e138580);
        (assign89960_body53_e138581, (locals.var_ps0ld_dn0 + ((locals.var_fac1_dn0 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn0))), (((-locals.var_vgpld_dn2) + locals.var_ps0ld_dn2) + ((locals.var_fac1_dn2 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn2))), (locals.var_ps0ld_dn4 + ((locals.var_fac1_dn4 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn4))), (locals.var_ps0ld_dn5 + ((locals.var_fac1_dn5 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn5))), (locals.var_ps0ld_dn6 + ((locals.var_fac1_dn6 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn6))), (((-locals.var_vgpld_dn7) + locals.var_ps0ld_dn7) + ((locals.var_fac1_dn7 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn7))), (((-locals.var_vgpld_dn8) + locals.var_ps0ld_dn8) + ((locals.var_fac1_dn8 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn8))), (((-locals.var_vgpld_dn9) + locals.var_ps0ld_dn9) + ((locals.var_fac1_dn9 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn9))), (locals.var_ps0ld_dn10 + ((locals.var_fac1_dn10 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn10))), (locals.var_ps0ld_dn11 + ((locals.var_fac1_dn11 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn11))), (locals.var_ps0ld_dn14 + ((locals.var_fac1_dn14 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn14))),)
    } else {
        (locals.var_fs0, locals.var_fs0_dn0, locals.var_fs0_dn2, locals.var_fs0_dn4, locals.var_fs0_dn5, locals.var_fs0_dn6, locals.var_fs0_dn7, locals.var_fs0_dn8, locals.var_fs0_dn9, locals.var_fs0_dn10, locals.var_fs0_dn11, locals.var_fs0_dn14,)
    }
};
            locals.var_fs0 = assign89960_body53_e138583;
            locals.var_fs0_dn0 = assign89960_body53_e138583_d_n0;
            locals.var_fs0_dn2 = assign89960_body53_e138583_d_n2;
            locals.var_fs0_dn4 = assign89960_body53_e138583_d_n4;
            locals.var_fs0_dn5 = assign89960_body53_e138583_d_n5;
            locals.var_fs0_dn6 = assign89960_body53_e138583_d_n6;
            locals.var_fs0_dn7 = assign89960_body53_e138583_d_n7;
            locals.var_fs0_dn8 = assign89960_body53_e138583_d_n8;
            locals.var_fs0_dn9 = assign89960_body53_e138583_d_n9;
            locals.var_fs0_dn10 = assign89960_body53_e138583_d_n10;
            locals.var_fs0_dn11 = assign89960_body53_e138583_d_n11;
            locals.var_fs0_dn14 = assign89960_body53_e138583_d_n14;
            locals.var_fs0_rv = 0.0;
            let (assign89960_body54_e138593, assign89960_body54_e138593_d_n0, assign89960_body54_e138593_d_n2, assign89960_body54_e138593_d_n4, assign89960_body54_e138593_d_n5, assign89960_body54_e138593_d_n6, assign89960_body54_e138593_d_n7, assign89960_body54_e138593_d_n8, assign89960_body54_e138593_d_n9, assign89960_body54_e138593_d_n10, assign89960_body54_e138593_d_n11, assign89960_body54_e138593_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2089 != 0.0)) {
        let assign89960_body54_e138590: f64 = (locals.var_fac1 * locals.var_fs02_dps0);
        let assign89960_body54_e138591: f64 = (1.0 + assign89960_body54_e138590);
        (assign89960_body54_e138591, ((locals.var_fac1_dn0 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn0)), ((locals.var_fac1_dn2 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn2)), ((locals.var_fac1_dn4 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn4)), ((locals.var_fac1_dn5 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn5)), ((locals.var_fac1_dn6 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn6)), ((locals.var_fac1_dn7 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn7)), ((locals.var_fac1_dn8 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn8)), ((locals.var_fac1_dn9 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn9)), ((locals.var_fac1_dn10 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn10)), ((locals.var_fac1_dn11 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn11)), ((locals.var_fac1_dn14 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn14)),)
    } else {
        (locals.var_fs0_dps0, locals.var_fs0_dps0_dn0, locals.var_fs0_dps0_dn2, locals.var_fs0_dps0_dn4, locals.var_fs0_dps0_dn5, locals.var_fs0_dps0_dn6, locals.var_fs0_dps0_dn7, locals.var_fs0_dps0_dn8, locals.var_fs0_dps0_dn9, locals.var_fs0_dps0_dn10, locals.var_fs0_dps0_dn11, locals.var_fs0_dps0_dn14,)
    }
};
            locals.var_fs0_dps0 = assign89960_body54_e138593;
            locals.var_fs0_dps0_dn0 = assign89960_body54_e138593_d_n0;
            locals.var_fs0_dps0_dn2 = assign89960_body54_e138593_d_n2;
            locals.var_fs0_dps0_dn4 = assign89960_body54_e138593_d_n4;
            locals.var_fs0_dps0_dn5 = assign89960_body54_e138593_d_n5;
            locals.var_fs0_dps0_dn6 = assign89960_body54_e138593_d_n6;
            locals.var_fs0_dps0_dn7 = assign89960_body54_e138593_d_n7;
            locals.var_fs0_dps0_dn8 = assign89960_body54_e138593_d_n8;
            locals.var_fs0_dps0_dn9 = assign89960_body54_e138593_d_n9;
            locals.var_fs0_dps0_dn10 = assign89960_body54_e138593_d_n10;
            locals.var_fs0_dps0_dn11 = assign89960_body54_e138593_d_n11;
            locals.var_fs0_dps0_dn14 = assign89960_body54_e138593_d_n14;
            locals.var_fs0_dps0_rv = 0.0;
            let assign89960_body55_e138596: f64 = if locals.var_flg_conv > 0.0 { 1.0 } else { 0.0 };
            locals.var_guard2100 = assign89960_body55_e138596;
            locals.var_guard2100_rv = 0.0;
            let (assign89960_body56_e138606,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2089 != 0.0)) && (locals.var_guard2100 != 0.0)) {
        let assign89960_body56_e138604: f64 = (locals.var_lp_s0_max + 1.0);
        (assign89960_body56_e138604,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign89960_body56_e138606;
            locals.var_lp_s0_rv = 0.0;
            let (assign89960_body57_e138618, assign89960_body57_e138618_d_n0, assign89960_body57_e138618_d_n2, assign89960_body57_e138618_d_n4, assign89960_body57_e138618_d_n5, assign89960_body57_e138618_d_n6, assign89960_body57_e138618_d_n7, assign89960_body57_e138618_d_n8, assign89960_body57_e138618_d_n9, assign89960_body57_e138618_d_n10, assign89960_body57_e138618_d_n11, assign89960_body57_e138618_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2089 != 0.0)) && (locals.var_guard2100 == 0.0)) {
        let assign89960_body57_e138614: f64 = (-locals.var_fs0);
        let assign89960_body57_e138616: f64 = (assign89960_body57_e138614 / locals.var_fs0_dps0);
        (assign89960_body57_e138616, ((((-locals.var_fs0_dn0) * locals.var_fs0_dps0) - (assign89960_body57_e138614 * locals.var_fs0_dps0_dn0)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn2) * locals.var_fs0_dps0) - (assign89960_body57_e138614 * locals.var_fs0_dps0_dn2)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn4) * locals.var_fs0_dps0) - (assign89960_body57_e138614 * locals.var_fs0_dps0_dn4)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn5) * locals.var_fs0_dps0) - (assign89960_body57_e138614 * locals.var_fs0_dps0_dn5)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn6) * locals.var_fs0_dps0) - (assign89960_body57_e138614 * locals.var_fs0_dps0_dn6)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn7) * locals.var_fs0_dps0) - (assign89960_body57_e138614 * locals.var_fs0_dps0_dn7)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn8) * locals.var_fs0_dps0) - (assign89960_body57_e138614 * locals.var_fs0_dps0_dn8)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn9) * locals.var_fs0_dps0) - (assign89960_body57_e138614 * locals.var_fs0_dps0_dn9)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn10) * locals.var_fs0_dps0) - (assign89960_body57_e138614 * locals.var_fs0_dps0_dn10)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn11) * locals.var_fs0_dps0) - (assign89960_body57_e138614 * locals.var_fs0_dps0_dn11)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn14) * locals.var_fs0_dps0) - (assign89960_body57_e138614 * locals.var_fs0_dps0_dn14)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn4, locals.var_dps0_dn5, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn8, locals.var_dps0_dn9, locals.var_dps0_dn10, locals.var_dps0_dn11, locals.var_dps0_dn14,)
    }
};
            locals.var_dps0 = assign89960_body57_e138618;
            locals.var_dps0_dn0 = assign89960_body57_e138618_d_n0;
            locals.var_dps0_dn2 = assign89960_body57_e138618_d_n2;
            locals.var_dps0_dn4 = assign89960_body57_e138618_d_n4;
            locals.var_dps0_dn5 = assign89960_body57_e138618_d_n5;
            locals.var_dps0_dn6 = assign89960_body57_e138618_d_n6;
            locals.var_dps0_dn7 = assign89960_body57_e138618_d_n7;
            locals.var_dps0_dn8 = assign89960_body57_e138618_d_n8;
            locals.var_dps0_dn9 = assign89960_body57_e138618_d_n9;
            locals.var_dps0_dn10 = assign89960_body57_e138618_d_n10;
            locals.var_dps0_dn11 = assign89960_body57_e138618_d_n11;
            locals.var_dps0_dn14 = assign89960_body57_e138618_d_n14;
            locals.var_dps0_rv = 0.0;
            let (assign89960_body58_e138640, assign89960_body58_e138640_d_n0, assign89960_body58_e138640_d_n2, assign89960_body58_e138640_d_n4, assign89960_body58_e138640_d_n5, assign89960_body58_e138640_d_n6, assign89960_body58_e138640_d_n7, assign89960_body58_e138640_d_n8, assign89960_body58_e138640_d_n9, assign89960_body58_e138640_d_n10, assign89960_body58_e138640_d_n11, assign89960_body58_e138640_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2089 != 0.0)) && (locals.var_guard2100 == 0.0)) {
        let assign89960_body58_e138627: f64 = (0.5 * 0.1);
        let assign89960_body58_e138631: f64 = (locals.var_ps0ld).abs();
        let (assign89960_body58_e138636, assign89960_body58_e138636_d_n0, assign89960_body58_e138636_d_n2, assign89960_body58_e138636_d_n4, assign89960_body58_e138636_d_n5, assign89960_body58_e138636_d_n6, assign89960_body58_e138636_d_n7, assign89960_body58_e138636_d_n8, assign89960_body58_e138636_d_n9, assign89960_body58_e138636_d_n10, assign89960_body58_e138636_d_n11, assign89960_body58_e138636_d_n14,) = {
            if (1.0 >= assign89960_body58_e138631) {
                (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign89960_body58_e138635: f64 = (locals.var_ps0ld).abs();
                (assign89960_body58_e138635, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn0 } else { (-locals.var_ps0ld_dn0) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn2 } else { (-locals.var_ps0ld_dn2) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn4 } else { (-locals.var_ps0ld_dn4) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn5 } else { (-locals.var_ps0ld_dn5) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn6 } else { (-locals.var_ps0ld_dn6) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn7 } else { (-locals.var_ps0ld_dn7) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn8 } else { (-locals.var_ps0ld_dn8) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn9 } else { (-locals.var_ps0ld_dn9) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn10 } else { (-locals.var_ps0ld_dn10) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn11 } else { (-locals.var_ps0ld_dn11) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn14 } else { (-locals.var_ps0ld_dn14) },)
            }
        };
        let assign89960_body58_e138637: f64 = (1.0 + assign89960_body58_e138636);
        let assign89960_body58_e138638: f64 = (assign89960_body58_e138627 * assign89960_body58_e138637);
        (assign89960_body58_e138638, (assign89960_body58_e138627 * assign89960_body58_e138636_d_n0), (assign89960_body58_e138627 * assign89960_body58_e138636_d_n2), (assign89960_body58_e138627 * assign89960_body58_e138636_d_n4), (assign89960_body58_e138627 * assign89960_body58_e138636_d_n5), (assign89960_body58_e138627 * assign89960_body58_e138636_d_n6), (assign89960_body58_e138627 * assign89960_body58_e138636_d_n7), (assign89960_body58_e138627 * assign89960_body58_e138636_d_n8), (assign89960_body58_e138627 * assign89960_body58_e138636_d_n9), (assign89960_body58_e138627 * assign89960_body58_e138636_d_n10), (assign89960_body58_e138627 * assign89960_body58_e138636_d_n11), (assign89960_body58_e138627 * assign89960_body58_e138636_d_n14),)
    } else {
        (locals.var_dplim, locals.var_dplim_dn0, locals.var_dplim_dn2, locals.var_dplim_dn4, locals.var_dplim_dn5, locals.var_dplim_dn6, locals.var_dplim_dn7, locals.var_dplim_dn8, locals.var_dplim_dn9, locals.var_dplim_dn10, locals.var_dplim_dn11, locals.var_dplim_dn14,)
    }
};
            locals.var_dplim = assign89960_body58_e138640;
            locals.var_dplim_dn0 = assign89960_body58_e138640_d_n0;
            locals.var_dplim_dn2 = assign89960_body58_e138640_d_n2;
            locals.var_dplim_dn4 = assign89960_body58_e138640_d_n4;
            locals.var_dplim_dn5 = assign89960_body58_e138640_d_n5;
            locals.var_dplim_dn6 = assign89960_body58_e138640_d_n6;
            locals.var_dplim_dn7 = assign89960_body58_e138640_d_n7;
            locals.var_dplim_dn8 = assign89960_body58_e138640_d_n8;
            locals.var_dplim_dn9 = assign89960_body58_e138640_d_n9;
            locals.var_dplim_dn10 = assign89960_body58_e138640_d_n10;
            locals.var_dplim_dn11 = assign89960_body58_e138640_d_n11;
            locals.var_dplim_dn14 = assign89960_body58_e138640_d_n14;
            locals.var_dplim_rv = 0.0;
            let assign89960_body59_e138642: f64 = (locals.var_dps0).abs();
            let assign89960_body59_e138644: f64 = if assign89960_body59_e138642 > locals.var_dplim { 1.0 } else { 0.0 };
            locals.var_guard2101 = assign89960_body59_e138644;
            locals.var_guard2101_rv = 0.0;
            let (assign89960_body60_e138663, assign89960_body60_e138663_d_n0, assign89960_body60_e138663_d_n2, assign89960_body60_e138663_d_n4, assign89960_body60_e138663_d_n5, assign89960_body60_e138663_d_n6, assign89960_body60_e138663_d_n7, assign89960_body60_e138663_d_n8, assign89960_body60_e138663_d_n9, assign89960_body60_e138663_d_n10, assign89960_body60_e138663_d_n11, assign89960_body60_e138663_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2089 != 0.0)) && (locals.var_guard2100 == 0.0)) && (locals.var_guard2101 != 0.0)) {
        let (assign89960_body60_e138660,) = {
            if (locals.var_dps0 >= 0.0) {
                (1.0,)
            } else {
                let assign89960_body60_e138659: f64 = (-1.0);
                (assign89960_body60_e138659,)
            }
        };
        let assign89960_body60_e138661: f64 = (locals.var_dplim * assign89960_body60_e138660);
        (assign89960_body60_e138661, (locals.var_dplim_dn0 * assign89960_body60_e138660), (locals.var_dplim_dn2 * assign89960_body60_e138660), (locals.var_dplim_dn4 * assign89960_body60_e138660), (locals.var_dplim_dn5 * assign89960_body60_e138660), (locals.var_dplim_dn6 * assign89960_body60_e138660), (locals.var_dplim_dn7 * assign89960_body60_e138660), (locals.var_dplim_dn8 * assign89960_body60_e138660), (locals.var_dplim_dn9 * assign89960_body60_e138660), (locals.var_dplim_dn10 * assign89960_body60_e138660), (locals.var_dplim_dn11 * assign89960_body60_e138660), (locals.var_dplim_dn14 * assign89960_body60_e138660),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn4, locals.var_dps0_dn5, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn8, locals.var_dps0_dn9, locals.var_dps0_dn10, locals.var_dps0_dn11, locals.var_dps0_dn14,)
    }
};
            locals.var_dps0 = assign89960_body60_e138663;
            locals.var_dps0_dn0 = assign89960_body60_e138663_d_n0;
            locals.var_dps0_dn2 = assign89960_body60_e138663_d_n2;
            locals.var_dps0_dn4 = assign89960_body60_e138663_d_n4;
            locals.var_dps0_dn5 = assign89960_body60_e138663_d_n5;
            locals.var_dps0_dn6 = assign89960_body60_e138663_d_n6;
            locals.var_dps0_dn7 = assign89960_body60_e138663_d_n7;
            locals.var_dps0_dn8 = assign89960_body60_e138663_d_n8;
            locals.var_dps0_dn9 = assign89960_body60_e138663_d_n9;
            locals.var_dps0_dn10 = assign89960_body60_e138663_d_n10;
            locals.var_dps0_dn11 = assign89960_body60_e138663_d_n11;
            locals.var_dps0_dn14 = assign89960_body60_e138663_d_n14;
            locals.var_dps0_rv = 0.0;
            let (assign89960_body61_e138674, assign89960_body61_e138674_d_n0, assign89960_body61_e138674_d_n2, assign89960_body61_e138674_d_n4, assign89960_body61_e138674_d_n5, assign89960_body61_e138674_d_n6, assign89960_body61_e138674_d_n7, assign89960_body61_e138674_d_n8, assign89960_body61_e138674_d_n9, assign89960_body61_e138674_d_n10, assign89960_body61_e138674_d_n11, assign89960_body61_e138674_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2089 != 0.0)) && (locals.var_guard2100 == 0.0)) {
        let assign89960_body61_e138672: f64 = (locals.var_ps0ld + locals.var_dps0);
        (assign89960_body61_e138672, (locals.var_ps0ld_dn0 + locals.var_dps0_dn0), (locals.var_ps0ld_dn2 + locals.var_dps0_dn2), (locals.var_ps0ld_dn4 + locals.var_dps0_dn4), (locals.var_ps0ld_dn5 + locals.var_dps0_dn5), (locals.var_ps0ld_dn6 + locals.var_dps0_dn6), (locals.var_ps0ld_dn7 + locals.var_dps0_dn7), (locals.var_ps0ld_dn8 + locals.var_dps0_dn8), (locals.var_ps0ld_dn9 + locals.var_dps0_dn9), (locals.var_ps0ld_dn10 + locals.var_dps0_dn10), (locals.var_ps0ld_dn11 + locals.var_dps0_dn11), (locals.var_ps0ld_dn14 + locals.var_dps0_dn14),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn14,)
    }
};
            locals.var_ps0ld = assign89960_body61_e138674;
            locals.var_ps0ld_dn0 = assign89960_body61_e138674_d_n0;
            locals.var_ps0ld_dn2 = assign89960_body61_e138674_d_n2;
            locals.var_ps0ld_dn4 = assign89960_body61_e138674_d_n4;
            locals.var_ps0ld_dn5 = assign89960_body61_e138674_d_n5;
            locals.var_ps0ld_dn6 = assign89960_body61_e138674_d_n6;
            locals.var_ps0ld_dn7 = assign89960_body61_e138674_d_n7;
            locals.var_ps0ld_dn8 = assign89960_body61_e138674_d_n8;
            locals.var_ps0ld_dn9 = assign89960_body61_e138674_d_n9;
            locals.var_ps0ld_dn10 = assign89960_body61_e138674_d_n10;
            locals.var_ps0ld_dn11 = assign89960_body61_e138674_d_n11;
            locals.var_ps0ld_dn14 = assign89960_body61_e138674_d_n14;
            locals.var_ps0ld_rv = 0.0;
            let assign89960_body62_e138676: f64 = (locals.var_dps0).abs();
            let assign89960_body62_e138680: f64 = (locals.var_fs0).abs();
            let assign89960_body62_e138683: f64 = if ((assign89960_body62_e138676 <= 1e-12) && (assign89960_body62_e138680 <= 1e-8)) { 1.0 } else { 0.0 };
            locals.var_guard2102 = assign89960_body62_e138683;
            locals.var_guard2102_rv = 0.0;
            let (assign89960_body63_e138696,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2089 != 0.0)) && (locals.var_guard2100 == 0.0)) && (locals.var_guard2102 != 0.0)) {
        let assign89960_body63_e138694: f64 = (locals.var_flg_conv + 2.0);
        (assign89960_body63_e138694,)
    } else {
        (locals.var_flg_conv,)
    }
};
            locals.var_flg_conv = assign89960_body63_e138696;
            locals.var_flg_conv_rv = 0.0;
            let (assign89960_body64_e138704,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2089 != 0.0)) {
        let assign89960_body64_e138702: f64 = (locals.var_lp_s0 + 1.0);
        (assign89960_body64_e138702,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign89960_body64_e138704;
            locals.var_lp_s0_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_347(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign89980_e138727, assign89980_e138727_d_n0, assign89980_e138727_d_n2, assign89980_e138727_d_n4, assign89980_e138727_d_n5, assign89980_e138727_d_n6, assign89980_e138727_d_n7, assign89980_e138727_d_n8, assign89980_e138727_d_n9, assign89980_e138727_d_n10, assign89980_e138727_d_n11, assign89980_e138727_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2089 != 0.0)) {
        let (assign89980_e138725, assign89980_e138725_d_n0, assign89980_e138725_d_n2, assign89980_e138725_d_n4, assign89980_e138725_d_n5, assign89980_e138725_d_n6, assign89980_e138725_d_n7, assign89980_e138725_d_n8, assign89980_e138725_d_n9, assign89980_e138725_d_n10, assign89980_e138725_d_n11, assign89980_e138725_d_n14,) = {
            if (locals.var_fbsq__blk2016 >= 0.0) {
                let (assign89980_e138720,) = {
                    if (locals.var_chi >= 0.0) {
                        (1.0,)
                    } else {
                        let assign89980_e138719: f64 = (-1.0);
                        (assign89980_e138719,)
                    }
                };
                let assign89980_e138722: f64 = (locals.var_fbsq__blk2016).sqrt();
                let assign89980_e138723: f64 = (assign89980_e138720 * assign89980_e138722);
                (assign89980_e138723, (assign89980_e138720 * (locals.var_fbsq__blk2016_dn0 / (2.0 * assign89980_e138722))), (assign89980_e138720 * (locals.var_fbsq__blk2016_dn2 / (2.0 * assign89980_e138722))), (assign89980_e138720 * (locals.var_fbsq__blk2016_dn4 / (2.0 * assign89980_e138722))), (assign89980_e138720 * (locals.var_fbsq__blk2016_dn5 / (2.0 * assign89980_e138722))), (assign89980_e138720 * (locals.var_fbsq__blk2016_dn6 / (2.0 * assign89980_e138722))), (assign89980_e138720 * (locals.var_fbsq__blk2016_dn7 / (2.0 * assign89980_e138722))), (assign89980_e138720 * (locals.var_fbsq__blk2016_dn8 / (2.0 * assign89980_e138722))), (assign89980_e138720 * (locals.var_fbsq__blk2016_dn9 / (2.0 * assign89980_e138722))), (assign89980_e138720 * (locals.var_fbsq__blk2016_dn10 / (2.0 * assign89980_e138722))), (assign89980_e138720 * (locals.var_fbsq__blk2016_dn11 / (2.0 * assign89980_e138722))), (assign89980_e138720 * (locals.var_fbsq__blk2016_dn14 / (2.0 * assign89980_e138722))),)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign89980_e138725, assign89980_e138725_d_n0, assign89980_e138725_d_n2, assign89980_e138725_d_n4, assign89980_e138725_d_n5, assign89980_e138725_d_n6, assign89980_e138725_d_n7, assign89980_e138725_d_n8, assign89980_e138725_d_n9, assign89980_e138725_d_n10, assign89980_e138725_d_n11, assign89980_e138725_d_n14,)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn14,)
    }
};
        locals.var_fb = assign89980_e138727;
        locals.var_fb_dn0 = assign89980_e138727_d_n0;
        locals.var_fb_dn2 = assign89980_e138727_d_n2;
        locals.var_fb_dn4 = assign89980_e138727_d_n4;
        locals.var_fb_dn5 = assign89980_e138727_d_n5;
        locals.var_fb_dn6 = assign89980_e138727_d_n6;
        locals.var_fb_dn7 = assign89980_e138727_d_n7;
        locals.var_fb_dn8 = assign89980_e138727_d_n8;
        locals.var_fb_dn9 = assign89980_e138727_d_n9;
        locals.var_fb_dn10 = assign89980_e138727_d_n10;
        locals.var_fb_dn11 = assign89980_e138727_d_n11;
        locals.var_fb_dn14 = assign89980_e138727_d_n14;
        locals.var_fb_rv = 0.0;

        let (assign89990_e138735, assign89990_e138735_d_n0, assign89990_e138735_d_n2, assign89990_e138735_d_n4, assign89990_e138735_d_n5, assign89990_e138735_d_n6, assign89990_e138735_d_n7, assign89990_e138735_d_n8, assign89990_e138735_d_n9, assign89990_e138735_d_n10, assign89990_e138735_d_n11, assign89990_e138735_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2089 != 0.0)) {
        let assign89990_e138733: f64 = (locals.var_c_w_ld * locals.var_fb);
        (assign89990_e138733, ((locals.var_c_w_ld_dn0 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn0)), ((locals.var_c_w_ld_dn2 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn2)), ((locals.var_c_w_ld_dn4 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn4)), ((locals.var_c_w_ld_dn5 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn5)), ((locals.var_c_w_ld_dn6 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn6)), ((locals.var_c_w_ld_dn7 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn7)), ((locals.var_c_w_ld_dn8 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn8)), ((locals.var_c_w_ld_dn9 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn9)), ((locals.var_c_w_ld_dn10 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn10)), ((locals.var_c_w_ld_dn11 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn11)), ((locals.var_c_w_ld_dn14 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn14)),)
    } else {
        (locals.var_wdld__blk2006, locals.var_wdld__blk2006_dn0, locals.var_wdld__blk2006_dn2, locals.var_wdld__blk2006_dn4, locals.var_wdld__blk2006_dn5, locals.var_wdld__blk2006_dn6, locals.var_wdld__blk2006_dn7, locals.var_wdld__blk2006_dn8, locals.var_wdld__blk2006_dn9, locals.var_wdld__blk2006_dn10, locals.var_wdld__blk2006_dn11, locals.var_wdld__blk2006_dn14,)
    }
};
        locals.var_wdld__blk2006 = assign89990_e138735;
        locals.var_wdld__blk2006_dn0 = assign89990_e138735_d_n0;
        locals.var_wdld__blk2006_dn2 = assign89990_e138735_d_n2;
        locals.var_wdld__blk2006_dn4 = assign89990_e138735_d_n4;
        locals.var_wdld__blk2006_dn5 = assign89990_e138735_d_n5;
        locals.var_wdld__blk2006_dn6 = assign89990_e138735_d_n6;
        locals.var_wdld__blk2006_dn7 = assign89990_e138735_d_n7;
        locals.var_wdld__blk2006_dn8 = assign89990_e138735_d_n8;
        locals.var_wdld__blk2006_dn9 = assign89990_e138735_d_n9;
        locals.var_wdld__blk2006_dn10 = assign89990_e138735_d_n10;
        locals.var_wdld__blk2006_dn11 = assign89990_e138735_d_n11;
        locals.var_wdld__blk2006_dn14 = assign89990_e138735_d_n14;
        locals.var_wdld__blk2006_rv = 0.0;

        let (assign90000_e138743, assign90000_e138743_d_n0, assign90000_e138743_d_n2, assign90000_e138743_d_n4, assign90000_e138743_d_n5, assign90000_e138743_d_n6, assign90000_e138743_d_n7, assign90000_e138743_d_n8, assign90000_e138743_d_n9, assign90000_e138743_d_n10, assign90000_e138743_d_n11, assign90000_e138743_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2089 != 0.0)) {
        let assign90000_e138741: f64 = (locals.var_q_nsubld__blk2008 * locals.var_wdld__blk2006);
        (assign90000_e138741, (locals.var_q_nsubld__blk2008 * locals.var_wdld__blk2006_dn0), (locals.var_q_nsubld__blk2008 * locals.var_wdld__blk2006_dn2), (locals.var_q_nsubld__blk2008 * locals.var_wdld__blk2006_dn4), (locals.var_q_nsubld__blk2008 * locals.var_wdld__blk2006_dn5), (locals.var_q_nsubld__blk2008 * locals.var_wdld__blk2006_dn6), (locals.var_q_nsubld__blk2008 * locals.var_wdld__blk2006_dn7), (locals.var_q_nsubld__blk2008 * locals.var_wdld__blk2006_dn8), (locals.var_q_nsubld__blk2008 * locals.var_wdld__blk2006_dn9), (locals.var_q_nsubld__blk2008 * locals.var_wdld__blk2006_dn10), (locals.var_q_nsubld__blk2008 * locals.var_wdld__blk2006_dn11), (locals.var_q_nsubld__blk2008 * locals.var_wdld__blk2006_dn14),)
    } else {
        (locals.var_q_dep_ld__blk2007, locals.var_q_dep_ld__blk2007_dn0, locals.var_q_dep_ld__blk2007_dn2, locals.var_q_dep_ld__blk2007_dn4, locals.var_q_dep_ld__blk2007_dn5, locals.var_q_dep_ld__blk2007_dn6, locals.var_q_dep_ld__blk2007_dn7, locals.var_q_dep_ld__blk2007_dn8, locals.var_q_dep_ld__blk2007_dn9, locals.var_q_dep_ld__blk2007_dn10, locals.var_q_dep_ld__blk2007_dn11, locals.var_q_dep_ld__blk2007_dn14,)
    }
};
        locals.var_q_dep_ld__blk2007 = assign90000_e138743;
        locals.var_q_dep_ld__blk2007_dn0 = assign90000_e138743_d_n0;
        locals.var_q_dep_ld__blk2007_dn2 = assign90000_e138743_d_n2;
        locals.var_q_dep_ld__blk2007_dn4 = assign90000_e138743_d_n4;
        locals.var_q_dep_ld__blk2007_dn5 = assign90000_e138743_d_n5;
        locals.var_q_dep_ld__blk2007_dn6 = assign90000_e138743_d_n6;
        locals.var_q_dep_ld__blk2007_dn7 = assign90000_e138743_d_n7;
        locals.var_q_dep_ld__blk2007_dn8 = assign90000_e138743_d_n8;
        locals.var_q_dep_ld__blk2007_dn9 = assign90000_e138743_d_n9;
        locals.var_q_dep_ld__blk2007_dn10 = assign90000_e138743_d_n10;
        locals.var_q_dep_ld__blk2007_dn11 = assign90000_e138743_d_n11;
        locals.var_q_dep_ld__blk2007_dn14 = assign90000_e138743_d_n14;
        locals.var_q_dep_ld__blk2007_rv = 0.0;

        let (assign90010_e138755, assign90010_e138755_d_n0, assign90010_e138755_d_n2, assign90010_e138755_d_n4, assign90010_e138755_d_n5, assign90010_e138755_d_n6, assign90010_e138755_d_n7, assign90010_e138755_d_n8, assign90010_e138755_d_n9, assign90010_e138755_d_n10, assign90010_e138755_d_n11, assign90010_e138755_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2089 != 0.0)) {
        let assign90010_e138749: f64 = (locals.var_q_dep_ld__blk2007 / locals.var_cnst0over_func);
        let assign90010_e138752: f64 = (10.0 * 2.220446049250313e-16);
        let assign90010_e138753: f64 = (assign90010_e138749 + assign90010_e138752);
        (assign90010_e138753, (((locals.var_q_dep_ld__blk2007_dn0 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2007 * locals.var_cnst0over_func_dn0)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2007_dn2 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2007 * locals.var_cnst0over_func_dn2)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2007_dn4 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2007 * locals.var_cnst0over_func_dn4)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2007_dn5 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2007 * locals.var_cnst0over_func_dn5)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2007_dn6 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2007 * locals.var_cnst0over_func_dn6)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2007_dn7 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2007 * locals.var_cnst0over_func_dn7)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2007_dn8 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2007 * locals.var_cnst0over_func_dn8)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2007_dn9 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2007 * locals.var_cnst0over_func_dn9)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2007_dn10 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2007 * locals.var_cnst0over_func_dn10)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2007_dn11 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2007 * locals.var_cnst0over_func_dn11)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2007_dn14 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2007 * locals.var_cnst0over_func_dn14)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)),)
    } else {
        (locals.var_xi0p12, locals.var_xi0p12_dn0, locals.var_xi0p12_dn2, locals.var_xi0p12_dn4, locals.var_xi0p12_dn5, locals.var_xi0p12_dn6, locals.var_xi0p12_dn7, locals.var_xi0p12_dn8, locals.var_xi0p12_dn9, locals.var_xi0p12_dn10, locals.var_xi0p12_dn11, locals.var_xi0p12_dn14,)
    }
};
        locals.var_xi0p12 = assign90010_e138755;
        locals.var_xi0p12_dn0 = assign90010_e138755_d_n0;
        locals.var_xi0p12_dn2 = assign90010_e138755_d_n2;
        locals.var_xi0p12_dn4 = assign90010_e138755_d_n4;
        locals.var_xi0p12_dn5 = assign90010_e138755_d_n5;
        locals.var_xi0p12_dn6 = assign90010_e138755_d_n6;
        locals.var_xi0p12_dn7 = assign90010_e138755_d_n7;
        locals.var_xi0p12_dn8 = assign90010_e138755_d_n8;
        locals.var_xi0p12_dn9 = assign90010_e138755_d_n9;
        locals.var_xi0p12_dn10 = assign90010_e138755_d_n10;
        locals.var_xi0p12_dn11 = assign90010_e138755_d_n11;
        locals.var_xi0p12_dn14 = assign90010_e138755_d_n14;
        locals.var_xi0p12_rv = 0.0;

        let (assign90020_e138763, assign90020_e138763_d_n0, assign90020_e138763_d_n2, assign90020_e138763_d_n4, assign90020_e138763_d_n5, assign90020_e138763_d_n6, assign90020_e138763_d_n7, assign90020_e138763_d_n8, assign90020_e138763_d_n9, assign90020_e138763_d_n10, assign90020_e138763_d_n11, assign90020_e138763_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2089 != 0.0)) {
        let assign90020_e138761: f64 = (locals.var_cnst0over_func * locals.var_xi0p12);
        (assign90020_e138761, ((locals.var_cnst0over_func_dn0 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn0)), ((locals.var_cnst0over_func_dn2 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn2)), ((locals.var_cnst0over_func_dn4 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn4)), ((locals.var_cnst0over_func_dn5 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn5)), ((locals.var_cnst0over_func_dn6 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn6)), ((locals.var_cnst0over_func_dn7 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn7)), ((locals.var_cnst0over_func_dn8 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn8)), ((locals.var_cnst0over_func_dn9 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn9)), ((locals.var_cnst0over_func_dn10 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn10)), ((locals.var_cnst0over_func_dn11 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn11)), ((locals.var_cnst0over_func_dn14 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn14)),)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn8, locals.var_qbuld_dn9, locals.var_qbuld_dn10, locals.var_qbuld_dn11, locals.var_qbuld_dn14,)
    }
};
        locals.var_qbuld = assign90020_e138763;
        locals.var_qbuld_dn0 = assign90020_e138763_d_n0;
        locals.var_qbuld_dn2 = assign90020_e138763_d_n2;
        locals.var_qbuld_dn4 = assign90020_e138763_d_n4;
        locals.var_qbuld_dn5 = assign90020_e138763_d_n5;
        locals.var_qbuld_dn6 = assign90020_e138763_d_n6;
        locals.var_qbuld_dn7 = assign90020_e138763_d_n7;
        locals.var_qbuld_dn8 = assign90020_e138763_d_n8;
        locals.var_qbuld_dn9 = assign90020_e138763_d_n9;
        locals.var_qbuld_dn10 = assign90020_e138763_d_n10;
        locals.var_qbuld_dn11 = assign90020_e138763_d_n11;
        locals.var_qbuld_dn14 = assign90020_e138763_d_n14;
        locals.var_qbuld_rv = 0.0;

        let (assign90030_e138773, assign90030_e138773_d_n0, assign90030_e138773_d_n2, assign90030_e138773_d_n4, assign90030_e138773_d_n5, assign90030_e138773_d_n6, assign90030_e138773_d_n7, assign90030_e138773_d_n8, assign90030_e138773_d_n9, assign90030_e138773_d_n10, assign90030_e138773_d_n11, assign90030_e138773_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2089 != 0.0)) {
        let assign90030_e138770: f64 = (locals.var_fs02 + locals.var_xi0p12);
        let assign90030_e138771: f64 = (1.0 / assign90030_e138770);
        (assign90030_e138771, (-((locals.var_fs02_dn0 + locals.var_xi0p12_dn0) / (assign90030_e138770 * assign90030_e138770))), (-((locals.var_fs02_dn2 + locals.var_xi0p12_dn2) / (assign90030_e138770 * assign90030_e138770))), (-((locals.var_fs02_dn4 + locals.var_xi0p12_dn4) / (assign90030_e138770 * assign90030_e138770))), (-((locals.var_fs02_dn5 + locals.var_xi0p12_dn5) / (assign90030_e138770 * assign90030_e138770))), (-((locals.var_fs02_dn6 + locals.var_xi0p12_dn6) / (assign90030_e138770 * assign90030_e138770))), (-((locals.var_fs02_dn7 + locals.var_xi0p12_dn7) / (assign90030_e138770 * assign90030_e138770))), (-((locals.var_fs02_dn8 + locals.var_xi0p12_dn8) / (assign90030_e138770 * assign90030_e138770))), (-((locals.var_fs02_dn9 + locals.var_xi0p12_dn9) / (assign90030_e138770 * assign90030_e138770))), (-((locals.var_fs02_dn10 + locals.var_xi0p12_dn10) / (assign90030_e138770 * assign90030_e138770))), (-((locals.var_fs02_dn11 + locals.var_xi0p12_dn11) / (assign90030_e138770 * assign90030_e138770))), (-((locals.var_fs02_dn14 + locals.var_xi0p12_dn14) / (assign90030_e138770 * assign90030_e138770))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign90030_e138773;
        locals.var_t1_dn0 = assign90030_e138773_d_n0;
        locals.var_t1_dn2 = assign90030_e138773_d_n2;
        locals.var_t1_dn4 = assign90030_e138773_d_n4;
        locals.var_t1_dn5 = assign90030_e138773_d_n5;
        locals.var_t1_dn6 = assign90030_e138773_d_n6;
        locals.var_t1_dn7 = assign90030_e138773_d_n7;
        locals.var_t1_dn8 = assign90030_e138773_d_n8;
        locals.var_t1_dn9 = assign90030_e138773_d_n9;
        locals.var_t1_dn10 = assign90030_e138773_d_n10;
        locals.var_t1_dn11 = assign90030_e138773_d_n11;
        locals.var_t1_dn14 = assign90030_e138773_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign90040_e138783, assign90040_e138783_d_n0, assign90040_e138783_d_n2, assign90040_e138783_d_n4, assign90040_e138783_d_n5, assign90040_e138783_d_n6, assign90040_e138783_d_n7, assign90040_e138783_d_n8, assign90040_e138783_d_n9, assign90040_e138783_d_n10, assign90040_e138783_d_n11, assign90040_e138783_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2089 != 0.0)) {
        let assign90040_e138779: f64 = (locals.var_cnst0over_func * locals.var_fs01);
        let assign90040_e138781: f64 = (assign90040_e138779 * locals.var_t1);
        (assign90040_e138781, ((((locals.var_cnst0over_func_dn0 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn0)) * locals.var_t1) + (assign90040_e138779 * locals.var_t1_dn0)), ((((locals.var_cnst0over_func_dn2 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn2)) * locals.var_t1) + (assign90040_e138779 * locals.var_t1_dn2)), ((((locals.var_cnst0over_func_dn4 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn4)) * locals.var_t1) + (assign90040_e138779 * locals.var_t1_dn4)), ((((locals.var_cnst0over_func_dn5 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn5)) * locals.var_t1) + (assign90040_e138779 * locals.var_t1_dn5)), ((((locals.var_cnst0over_func_dn6 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn6)) * locals.var_t1) + (assign90040_e138779 * locals.var_t1_dn6)), ((((locals.var_cnst0over_func_dn7 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn7)) * locals.var_t1) + (assign90040_e138779 * locals.var_t1_dn7)), ((((locals.var_cnst0over_func_dn8 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn8)) * locals.var_t1) + (assign90040_e138779 * locals.var_t1_dn8)), ((((locals.var_cnst0over_func_dn9 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn9)) * locals.var_t1) + (assign90040_e138779 * locals.var_t1_dn9)), ((((locals.var_cnst0over_func_dn10 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn10)) * locals.var_t1) + (assign90040_e138779 * locals.var_t1_dn10)), ((((locals.var_cnst0over_func_dn11 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn11)) * locals.var_t1) + (assign90040_e138779 * locals.var_t1_dn11)), ((((locals.var_cnst0over_func_dn14 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn14)) * locals.var_t1) + (assign90040_e138779 * locals.var_t1_dn14)),)
    } else {
        (locals.var_qiuld, locals.var_qiuld_dn0, locals.var_qiuld_dn2, locals.var_qiuld_dn4, locals.var_qiuld_dn5, locals.var_qiuld_dn6, locals.var_qiuld_dn7, locals.var_qiuld_dn8, locals.var_qiuld_dn9, locals.var_qiuld_dn10, locals.var_qiuld_dn11, locals.var_qiuld_dn14,)
    }
};
        locals.var_qiuld = assign90040_e138783;
        locals.var_qiuld_dn0 = assign90040_e138783_d_n0;
        locals.var_qiuld_dn2 = assign90040_e138783_d_n2;
        locals.var_qiuld_dn4 = assign90040_e138783_d_n4;
        locals.var_qiuld_dn5 = assign90040_e138783_d_n5;
        locals.var_qiuld_dn6 = assign90040_e138783_d_n6;
        locals.var_qiuld_dn7 = assign90040_e138783_d_n7;
        locals.var_qiuld_dn8 = assign90040_e138783_d_n8;
        locals.var_qiuld_dn9 = assign90040_e138783_d_n9;
        locals.var_qiuld_dn10 = assign90040_e138783_d_n10;
        locals.var_qiuld_dn11 = assign90040_e138783_d_n11;
        locals.var_qiuld_dn14 = assign90040_e138783_d_n14;
        locals.var_qiuld_rv = 0.0;

        let (assign90050_e138791, assign90050_e138791_d_n0, assign90050_e138791_d_n2, assign90050_e138791_d_n4, assign90050_e138791_d_n5, assign90050_e138791_d_n6, assign90050_e138791_d_n7, assign90050_e138791_d_n8, assign90050_e138791_d_n9, assign90050_e138791_d_n10, assign90050_e138791_d_n11, assign90050_e138791_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2089 != 0.0)) {
        let assign90050_e138789: f64 = (locals.var_qbuld + locals.var_qiuld);
        (assign90050_e138789, (locals.var_qbuld_dn0 + locals.var_qiuld_dn0), (locals.var_qbuld_dn2 + locals.var_qiuld_dn2), (locals.var_qbuld_dn4 + locals.var_qiuld_dn4), (locals.var_qbuld_dn5 + locals.var_qiuld_dn5), (locals.var_qbuld_dn6 + locals.var_qiuld_dn6), (locals.var_qbuld_dn7 + locals.var_qiuld_dn7), (locals.var_qbuld_dn8 + locals.var_qiuld_dn8), (locals.var_qbuld_dn9 + locals.var_qiuld_dn9), (locals.var_qbuld_dn10 + locals.var_qiuld_dn10), (locals.var_qbuld_dn11 + locals.var_qiuld_dn11), (locals.var_qbuld_dn14 + locals.var_qiuld_dn14),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn4, locals.var_qsuld_dn5, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn8, locals.var_qsuld_dn9, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn14,)
    }
};
        locals.var_qsuld = assign90050_e138791;
        locals.var_qsuld_dn0 = assign90050_e138791_d_n0;
        locals.var_qsuld_dn2 = assign90050_e138791_d_n2;
        locals.var_qsuld_dn4 = assign90050_e138791_d_n4;
        locals.var_qsuld_dn5 = assign90050_e138791_d_n5;
        locals.var_qsuld_dn6 = assign90050_e138791_d_n6;
        locals.var_qsuld_dn7 = assign90050_e138791_d_n7;
        locals.var_qsuld_dn8 = assign90050_e138791_d_n8;
        locals.var_qsuld_dn9 = assign90050_e138791_d_n9;
        locals.var_qsuld_dn10 = assign90050_e138791_d_n10;
        locals.var_qsuld_dn11 = assign90050_e138791_d_n11;
        locals.var_qsuld_dn14 = assign90050_e138791_d_n14;
        locals.var_qsuld_rv = 0.0;

        let (assign90060_e138797, assign90060_e138797_d_n0, assign90060_e138797_d_n2, assign90060_e138797_d_n4, assign90060_e138797_d_n5, assign90060_e138797_d_n6, assign90060_e138797_d_n7, assign90060_e138797_d_n8, assign90060_e138797_d_n9, assign90060_e138797_d_n10, assign90060_e138797_d_n11, assign90060_e138797_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign90060_e138795: f64 = (locals.var_qsuld - locals.var_qbuld);
        (assign90060_e138795, (locals.var_qsuld_dn0 - locals.var_qbuld_dn0), (locals.var_qsuld_dn2 - locals.var_qbuld_dn2), (locals.var_qsuld_dn4 - locals.var_qbuld_dn4), (locals.var_qsuld_dn5 - locals.var_qbuld_dn5), (locals.var_qsuld_dn6 - locals.var_qbuld_dn6), (locals.var_qsuld_dn7 - locals.var_qbuld_dn7), (locals.var_qsuld_dn8 - locals.var_qbuld_dn8), (locals.var_qsuld_dn9 - locals.var_qbuld_dn9), (locals.var_qsuld_dn10 - locals.var_qbuld_dn10), (locals.var_qsuld_dn11 - locals.var_qbuld_dn11), (locals.var_qsuld_dn14 - locals.var_qbuld_dn14),)
    } else {
        (locals.var_qiuld, locals.var_qiuld_dn0, locals.var_qiuld_dn2, locals.var_qiuld_dn4, locals.var_qiuld_dn5, locals.var_qiuld_dn6, locals.var_qiuld_dn7, locals.var_qiuld_dn8, locals.var_qiuld_dn9, locals.var_qiuld_dn10, locals.var_qiuld_dn11, locals.var_qiuld_dn14,)
    }
};
        locals.var_qiuld = assign90060_e138797;
        locals.var_qiuld_dn0 = assign90060_e138797_d_n0;
        locals.var_qiuld_dn2 = assign90060_e138797_d_n2;
        locals.var_qiuld_dn4 = assign90060_e138797_d_n4;
        locals.var_qiuld_dn5 = assign90060_e138797_d_n5;
        locals.var_qiuld_dn6 = assign90060_e138797_d_n6;
        locals.var_qiuld_dn7 = assign90060_e138797_d_n7;
        locals.var_qiuld_dn8 = assign90060_e138797_d_n8;
        locals.var_qiuld_dn9 = assign90060_e138797_d_n9;
        locals.var_qiuld_dn10 = assign90060_e138797_d_n10;
        locals.var_qiuld_dn11 = assign90060_e138797_d_n11;
        locals.var_qiuld_dn14 = assign90060_e138797_d_n14;
        locals.var_qiuld_rv = 0.0;

        let assign90070_e138800: f64 = if locals.var_lover_func < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2104 = assign90070_e138800;
        locals.var_guard2104_rv = 0.0;

        let (assign90080_e138807, assign90080_e138807_d_n0, assign90080_e138807_d_n2, assign90080_e138807_d_n4, assign90080_e138807_d_n5, assign90080_e138807_d_n6, assign90080_e138807_d_n7, assign90080_e138807_d_n8, assign90080_e138807_d_n9, assign90080_e138807_d_n10, assign90080_e138807_d_n11, assign90080_e138807_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2104 != 0.0)) {
        let assign90080_e138805: f64 = (-locals.var_lover_func);
        (assign90080_e138805, (-locals.var_lover_func_dn0), (-locals.var_lover_func_dn2), (-locals.var_lover_func_dn4), (-locals.var_lover_func_dn5), (-locals.var_lover_func_dn6), (-locals.var_lover_func_dn7), (-locals.var_lover_func_dn8), (-locals.var_lover_func_dn9), (-locals.var_lover_func_dn10), (-locals.var_lover_func_dn11), (-locals.var_lover_func_dn14),)
    } else {
        (locals.var_lover_func, locals.var_lover_func_dn0, locals.var_lover_func_dn2, locals.var_lover_func_dn4, locals.var_lover_func_dn5, locals.var_lover_func_dn6, locals.var_lover_func_dn7, locals.var_lover_func_dn8, locals.var_lover_func_dn9, locals.var_lover_func_dn10, locals.var_lover_func_dn11, locals.var_lover_func_dn14,)
    }
};
        locals.var_lover_func = assign90080_e138807;
        locals.var_lover_func_dn0 = assign90080_e138807_d_n0;
        locals.var_lover_func_dn2 = assign90080_e138807_d_n2;
        locals.var_lover_func_dn4 = assign90080_e138807_d_n4;
        locals.var_lover_func_dn5 = assign90080_e138807_d_n5;
        locals.var_lover_func_dn6 = assign90080_e138807_d_n6;
        locals.var_lover_func_dn7 = assign90080_e138807_d_n7;
        locals.var_lover_func_dn8 = assign90080_e138807_d_n8;
        locals.var_lover_func_dn9 = assign90080_e138807_d_n9;
        locals.var_lover_func_dn10 = assign90080_e138807_d_n10;
        locals.var_lover_func_dn11 = assign90080_e138807_d_n11;
        locals.var_lover_func_dn14 = assign90080_e138807_d_n14;
        locals.var_lover_func_rv = 0.0;

        let assign90090_e138810: f64 = if p.p55 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2105 = assign90090_e138810;
        locals.var_guard2105_rv = 0.0;

        let assign90100_e138813: f64 = if p.p50 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2106 = assign90100_e138813;
        locals.var_guard2106_rv = 0.0;

        let (assign90110_e138824, assign90110_e138824_d_n0, assign90110_e138824_d_n2, assign90110_e138824_d_n4, assign90110_e138824_d_n5, assign90110_e138824_d_n6, assign90110_e138824_d_n7, assign90110_e138824_d_n8, assign90110_e138824_d_n9, assign90110_e138824_d_n10, assign90110_e138824_d_n11, assign90110_e138824_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2104 != 0.0)) && (locals.var_guard2105 != 0.0)) && (locals.var_guard2106 != 0.0)) {
        let assign90110_e138822: f64 = (-locals.var_ps0ld);
        (assign90110_e138822, (-locals.var_ps0ld_dn0), (-locals.var_ps0ld_dn2), (-locals.var_ps0ld_dn4), (-locals.var_ps0ld_dn5), (-locals.var_ps0ld_dn6), (-locals.var_ps0ld_dn7), (-locals.var_ps0ld_dn8), (-locals.var_ps0ld_dn9), (-locals.var_ps0ld_dn10), (-locals.var_ps0ld_dn11), (-locals.var_ps0ld_dn14),)
    } else {
        (locals.var_vx__blk2009, locals.var_vx__blk2009_dn0, locals.var_vx__blk2009_dn2, locals.var_vx__blk2009_dn4, locals.var_vx__blk2009_dn5, locals.var_vx__blk2009_dn6, locals.var_vx__blk2009_dn7, locals.var_vx__blk2009_dn8, locals.var_vx__blk2009_dn9, locals.var_vx__blk2009_dn10, locals.var_vx__blk2009_dn11, locals.var_vx__blk2009_dn14,)
    }
};
        locals.var_vx__blk2009 = assign90110_e138824;
        locals.var_vx__blk2009_dn0 = assign90110_e138824_d_n0;
        locals.var_vx__blk2009_dn2 = assign90110_e138824_d_n2;
        locals.var_vx__blk2009_dn4 = assign90110_e138824_d_n4;
        locals.var_vx__blk2009_dn5 = assign90110_e138824_d_n5;
        locals.var_vx__blk2009_dn6 = assign90110_e138824_d_n6;
        locals.var_vx__blk2009_dn7 = assign90110_e138824_d_n7;
        locals.var_vx__blk2009_dn8 = assign90110_e138824_d_n8;
        locals.var_vx__blk2009_dn9 = assign90110_e138824_d_n9;
        locals.var_vx__blk2009_dn10 = assign90110_e138824_d_n10;
        locals.var_vx__blk2009_dn11 = assign90110_e138824_d_n11;
        locals.var_vx__blk2009_dn14 = assign90110_e138824_d_n14;
        locals.var_vx__blk2009_rv = 0.0;

        let (assign90120_e138835, assign90120_e138835_d_n0, assign90120_e138835_d_n2, assign90120_e138835_d_n4, assign90120_e138835_d_n5, assign90120_e138835_d_n6, assign90120_e138835_d_n7, assign90120_e138835_d_n8, assign90120_e138835_d_n9, assign90120_e138835_d_n10, assign90120_e138835_d_n11, assign90120_e138835_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2104 != 0.0)) && (locals.var_guard2105 != 0.0)) && (locals.var_guard2106 == 0.0)) {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn11, locals.var_vxbgmt_dn14,)
    } else {
        (locals.var_vx__blk2009, locals.var_vx__blk2009_dn0, locals.var_vx__blk2009_dn2, locals.var_vx__blk2009_dn4, locals.var_vx__blk2009_dn5, locals.var_vx__blk2009_dn6, locals.var_vx__blk2009_dn7, locals.var_vx__blk2009_dn8, locals.var_vx__blk2009_dn9, locals.var_vx__blk2009_dn10, locals.var_vx__blk2009_dn11, locals.var_vx__blk2009_dn14,)
    }
};
        locals.var_vx__blk2009 = assign90120_e138835;
        locals.var_vx__blk2009_dn0 = assign90120_e138835_d_n0;
        locals.var_vx__blk2009_dn2 = assign90120_e138835_d_n2;
        locals.var_vx__blk2009_dn4 = assign90120_e138835_d_n4;
        locals.var_vx__blk2009_dn5 = assign90120_e138835_d_n5;
        locals.var_vx__blk2009_dn6 = assign90120_e138835_d_n6;
        locals.var_vx__blk2009_dn7 = assign90120_e138835_d_n7;
        locals.var_vx__blk2009_dn8 = assign90120_e138835_d_n8;
        locals.var_vx__blk2009_dn9 = assign90120_e138835_d_n9;
        locals.var_vx__blk2009_dn10 = assign90120_e138835_d_n10;
        locals.var_vx__blk2009_dn11 = assign90120_e138835_d_n11;
        locals.var_vx__blk2009_dn14 = assign90120_e138835_d_n14;
        locals.var_vx__blk2009_rv = 0.0;

        let (assign90130_e138856, assign90130_e138856_d_n0, assign90130_e138856_d_n2, assign90130_e138856_d_n4, assign90130_e138856_d_n5, assign90130_e138856_d_n6, assign90130_e138856_d_n7, assign90130_e138856_d_n8, assign90130_e138856_d_n9, assign90130_e138856_d_n10, assign90130_e138856_d_n11, assign90130_e138856_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2104 != 0.0)) && (locals.var_guard2105 != 0.0)) {
        let assign90130_e138843: f64 = (locals.var_vx__blk2009 + p.p137);
        let assign90130_e138846: f64 = (locals.var_vx__blk2009 + p.p137);
        let assign90130_e138847: f64 = (assign90130_e138843 * assign90130_e138846);
        let assign90130_e138850: f64 = (4.0 * 0.1);
        let assign90130_e138852: f64 = (assign90130_e138850 * 0.1);
        let assign90130_e138853: f64 = (assign90130_e138847 + assign90130_e138852);
        let assign90130_e138854: f64 = (assign90130_e138853).sqrt();
        (assign90130_e138854, (((locals.var_vx__blk2009_dn0 * assign90130_e138846) + (assign90130_e138843 * locals.var_vx__blk2009_dn0)) / (2.0 * assign90130_e138854)), (((locals.var_vx__blk2009_dn2 * assign90130_e138846) + (assign90130_e138843 * locals.var_vx__blk2009_dn2)) / (2.0 * assign90130_e138854)), (((locals.var_vx__blk2009_dn4 * assign90130_e138846) + (assign90130_e138843 * locals.var_vx__blk2009_dn4)) / (2.0 * assign90130_e138854)), (((locals.var_vx__blk2009_dn5 * assign90130_e138846) + (assign90130_e138843 * locals.var_vx__blk2009_dn5)) / (2.0 * assign90130_e138854)), (((locals.var_vx__blk2009_dn6 * assign90130_e138846) + (assign90130_e138843 * locals.var_vx__blk2009_dn6)) / (2.0 * assign90130_e138854)), (((locals.var_vx__blk2009_dn7 * assign90130_e138846) + (assign90130_e138843 * locals.var_vx__blk2009_dn7)) / (2.0 * assign90130_e138854)), (((locals.var_vx__blk2009_dn8 * assign90130_e138846) + (assign90130_e138843 * locals.var_vx__blk2009_dn8)) / (2.0 * assign90130_e138854)), (((locals.var_vx__blk2009_dn9 * assign90130_e138846) + (assign90130_e138843 * locals.var_vx__blk2009_dn9)) / (2.0 * assign90130_e138854)), (((locals.var_vx__blk2009_dn10 * assign90130_e138846) + (assign90130_e138843 * locals.var_vx__blk2009_dn10)) / (2.0 * assign90130_e138854)), (((locals.var_vx__blk2009_dn11 * assign90130_e138846) + (assign90130_e138843 * locals.var_vx__blk2009_dn11)) / (2.0 * assign90130_e138854)), (((locals.var_vx__blk2009_dn14 * assign90130_e138846) + (assign90130_e138843 * locals.var_vx__blk2009_dn14)) / (2.0 * assign90130_e138854)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign90130_e138856;
        locals.var_tmf2_dn0 = assign90130_e138856_d_n0;
        locals.var_tmf2_dn2 = assign90130_e138856_d_n2;
        locals.var_tmf2_dn4 = assign90130_e138856_d_n4;
        locals.var_tmf2_dn5 = assign90130_e138856_d_n5;
        locals.var_tmf2_dn6 = assign90130_e138856_d_n6;
        locals.var_tmf2_dn7 = assign90130_e138856_d_n7;
        locals.var_tmf2_dn8 = assign90130_e138856_d_n8;
        locals.var_tmf2_dn9 = assign90130_e138856_d_n9;
        locals.var_tmf2_dn10 = assign90130_e138856_d_n10;
        locals.var_tmf2_dn11 = assign90130_e138856_d_n11;
        locals.var_tmf2_dn14 = assign90130_e138856_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign90140_e138872, assign90140_e138872_d_n0, assign90140_e138872_d_n2, assign90140_e138872_d_n4, assign90140_e138872_d_n5, assign90140_e138872_d_n6, assign90140_e138872_d_n7, assign90140_e138872_d_n8, assign90140_e138872_d_n9, assign90140_e138872_d_n10, assign90140_e138872_d_n11, assign90140_e138872_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2104 != 0.0)) && (locals.var_guard2105 != 0.0)) {
        let assign90140_e138866: f64 = (locals.var_vx__blk2009 + p.p137);
        let assign90140_e138868: f64 = (assign90140_e138866 / locals.var_tmf2);
        let assign90140_e138869: f64 = (1.0 + assign90140_e138868);
        let assign90140_e138870: f64 = (0.5 * assign90140_e138869);
        (assign90140_e138870, (0.5 * (((locals.var_vx__blk2009_dn0 * locals.var_tmf2) - (assign90140_e138866 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk2009_dn2 * locals.var_tmf2) - (assign90140_e138866 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk2009_dn4 * locals.var_tmf2) - (assign90140_e138866 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk2009_dn5 * locals.var_tmf2) - (assign90140_e138866 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk2009_dn6 * locals.var_tmf2) - (assign90140_e138866 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk2009_dn7 * locals.var_tmf2) - (assign90140_e138866 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk2009_dn8 * locals.var_tmf2) - (assign90140_e138866 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk2009_dn9 * locals.var_tmf2) - (assign90140_e138866 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk2009_dn10 * locals.var_tmf2) - (assign90140_e138866 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk2009_dn11 * locals.var_tmf2) - (assign90140_e138866 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk2009_dn14 * locals.var_tmf2) - (assign90140_e138866 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign90140_e138872;
        locals.var_t9_dn0 = assign90140_e138872_d_n0;
        locals.var_t9_dn2 = assign90140_e138872_d_n2;
        locals.var_t9_dn4 = assign90140_e138872_d_n4;
        locals.var_t9_dn5 = assign90140_e138872_d_n5;
        locals.var_t9_dn6 = assign90140_e138872_d_n6;
        locals.var_t9_dn7 = assign90140_e138872_d_n7;
        locals.var_t9_dn8 = assign90140_e138872_d_n8;
        locals.var_t9_dn9 = assign90140_e138872_d_n9;
        locals.var_t9_dn10 = assign90140_e138872_d_n10;
        locals.var_t9_dn11 = assign90140_e138872_d_n11;
        locals.var_t9_dn14 = assign90140_e138872_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign90150_e138886, assign90150_e138886_d_n0, assign90150_e138886_d_n2, assign90150_e138886_d_n4, assign90150_e138886_d_n5, assign90150_e138886_d_n6, assign90150_e138886_d_n7, assign90150_e138886_d_n8, assign90150_e138886_d_n9, assign90150_e138886_d_n10, assign90150_e138886_d_n11, assign90150_e138886_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2104 != 0.0)) && (locals.var_guard2105 != 0.0)) {
        let assign90150_e138881: f64 = (locals.var_vx__blk2009 + p.p137);
        let assign90150_e138883: f64 = (assign90150_e138881 + locals.var_tmf2);
        let assign90150_e138884: f64 = (0.5 * assign90150_e138883);
        (assign90150_e138884, (0.5 * (locals.var_vx__blk2009_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_vx__blk2009_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_vx__blk2009_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_vx__blk2009_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_vx__blk2009_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_vx__blk2009_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_vx__blk2009_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_vx__blk2009_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_vx__blk2009_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_vx__blk2009_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_vx__blk2009_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign90150_e138886;
        locals.var_t2_dn0 = assign90150_e138886_d_n0;
        locals.var_t2_dn2 = assign90150_e138886_d_n2;
        locals.var_t2_dn4 = assign90150_e138886_d_n4;
        locals.var_t2_dn5 = assign90150_e138886_d_n5;
        locals.var_t2_dn6 = assign90150_e138886_d_n6;
        locals.var_t2_dn7 = assign90150_e138886_d_n7;
        locals.var_t2_dn8 = assign90150_e138886_d_n8;
        locals.var_t2_dn9 = assign90150_e138886_d_n9;
        locals.var_t2_dn10 = assign90150_e138886_d_n10;
        locals.var_t2_dn11 = assign90150_e138886_d_n11;
        locals.var_t2_dn14 = assign90150_e138886_d_n14;
        locals.var_t2_rv = 0.0;

        let assign90160_e138889: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2107 = assign90160_e138889;
        locals.var_guard2107_rv = 0.0;

        let (assign90170_e138899, assign90170_e138899_d_n0, assign90170_e138899_d_n2, assign90170_e138899_d_n4, assign90170_e138899_d_n5, assign90170_e138899_d_n6, assign90170_e138899_d_n7, assign90170_e138899_d_n8, assign90170_e138899_d_n9, assign90170_e138899_d_n10, assign90170_e138899_d_n11, assign90170_e138899_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2104 != 0.0)) && (locals.var_guard2105 != 0.0)) && (locals.var_guard2107 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign90170_e138899;
        locals.var_t2_dn0 = assign90170_e138899_d_n0;
        locals.var_t2_dn2 = assign90170_e138899_d_n2;
        locals.var_t2_dn4 = assign90170_e138899_d_n4;
        locals.var_t2_dn5 = assign90170_e138899_d_n5;
        locals.var_t2_dn6 = assign90170_e138899_d_n6;
        locals.var_t2_dn7 = assign90170_e138899_d_n7;
        locals.var_t2_dn8 = assign90170_e138899_d_n8;
        locals.var_t2_dn9 = assign90170_e138899_d_n9;
        locals.var_t2_dn10 = assign90170_e138899_d_n10;
        locals.var_t2_dn11 = assign90170_e138899_d_n11;
        locals.var_t2_dn14 = assign90170_e138899_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign90180_e138909, assign90180_e138909_d_n0, assign90180_e138909_d_n2, assign90180_e138909_d_n4, assign90180_e138909_d_n5, assign90180_e138909_d_n6, assign90180_e138909_d_n7, assign90180_e138909_d_n8, assign90180_e138909_d_n9, assign90180_e138909_d_n10, assign90180_e138909_d_n11, assign90180_e138909_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2104 != 0.0)) && (locals.var_guard2105 != 0.0)) && (locals.var_guard2107 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign90180_e138909;
        locals.var_t9_dn0 = assign90180_e138909_d_n0;
        locals.var_t9_dn2 = assign90180_e138909_d_n2;
        locals.var_t9_dn4 = assign90180_e138909_d_n4;
        locals.var_t9_dn5 = assign90180_e138909_d_n5;
        locals.var_t9_dn6 = assign90180_e138909_d_n6;
        locals.var_t9_dn7 = assign90180_e138909_d_n7;
        locals.var_t9_dn8 = assign90180_e138909_d_n8;
        locals.var_t9_dn9 = assign90180_e138909_d_n9;
        locals.var_t9_dn10 = assign90180_e138909_d_n10;
        locals.var_t9_dn11 = assign90180_e138909_d_n11;
        locals.var_t9_dn14 = assign90180_e138909_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign90190_e138922, assign90190_e138922_d_n0, assign90190_e138922_d_n2, assign90190_e138922_d_n4, assign90190_e138922_d_n5, assign90190_e138922_d_n6, assign90190_e138922_d_n7, assign90190_e138922_d_n8, assign90190_e138922_d_n9, assign90190_e138922_d_n10, assign90190_e138922_d_n11, assign90190_e138922_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2104 != 0.0)) && (locals.var_guard2105 != 0.0)) {
        let assign90190_e138917: f64 = (locals.var_kjunc * locals.var_t2);
        let assign90190_e138918: f64 = (assign90190_e138917).sqrt();
        let assign90190_e138920: f64 = (assign90190_e138918 * p.p432);
        (assign90190_e138920, ((((locals.var_kjunc_dn0 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn0)) / (2.0 * assign90190_e138918)) * p.p432), ((((locals.var_kjunc_dn2 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn2)) / (2.0 * assign90190_e138918)) * p.p432), ((((locals.var_kjunc_dn4 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn4)) / (2.0 * assign90190_e138918)) * p.p432), ((((locals.var_kjunc_dn5 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn5)) / (2.0 * assign90190_e138918)) * p.p432), ((((locals.var_kjunc_dn6 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn6)) / (2.0 * assign90190_e138918)) * p.p432), ((((locals.var_kjunc_dn7 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn7)) / (2.0 * assign90190_e138918)) * p.p432), ((((locals.var_kjunc_dn8 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn8)) / (2.0 * assign90190_e138918)) * p.p432), ((((locals.var_kjunc_dn9 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn9)) / (2.0 * assign90190_e138918)) * p.p432), ((((locals.var_kjunc_dn10 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn10)) / (2.0 * assign90190_e138918)) * p.p432), ((((locals.var_kjunc_dn11 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn11)) / (2.0 * assign90190_e138918)) * p.p432), ((((locals.var_kjunc_dn14 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn14)) / (2.0 * assign90190_e138918)) * p.p432),)
    } else {
        (locals.var_wjunc0, locals.var_wjunc0_dn0, locals.var_wjunc0_dn2, locals.var_wjunc0_dn4, locals.var_wjunc0_dn5, locals.var_wjunc0_dn6, locals.var_wjunc0_dn7, locals.var_wjunc0_dn8, locals.var_wjunc0_dn9, locals.var_wjunc0_dn10, locals.var_wjunc0_dn11, locals.var_wjunc0_dn14,)
    }
};
        locals.var_wjunc0 = assign90190_e138922;
        locals.var_wjunc0_dn0 = assign90190_e138922_d_n0;
        locals.var_wjunc0_dn2 = assign90190_e138922_d_n2;
        locals.var_wjunc0_dn4 = assign90190_e138922_d_n4;
        locals.var_wjunc0_dn5 = assign90190_e138922_d_n5;
        locals.var_wjunc0_dn6 = assign90190_e138922_d_n6;
        locals.var_wjunc0_dn7 = assign90190_e138922_d_n7;
        locals.var_wjunc0_dn8 = assign90190_e138922_d_n8;
        locals.var_wjunc0_dn9 = assign90190_e138922_d_n9;
        locals.var_wjunc0_dn10 = assign90190_e138922_d_n10;
        locals.var_wjunc0_dn11 = assign90190_e138922_d_n11;
        locals.var_wjunc0_dn14 = assign90190_e138922_d_n14;
        locals.var_wjunc0_rv = 0.0;

        let (assign90200_e138936, assign90200_e138936_d_n0, assign90200_e138936_d_n2, assign90200_e138936_d_n4, assign90200_e138936_d_n5, assign90200_e138936_d_n6, assign90200_e138936_d_n7, assign90200_e138936_d_n8, assign90200_e138936_d_n9, assign90200_e138936_d_n10, assign90200_e138936_d_n11, assign90200_e138936_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2104 != 0.0)) && (locals.var_guard2105 != 0.0)) {
        let assign90200_e138930: f64 = (locals.var_lover_func - locals.var_wjunc0);
        let assign90200_e138933: f64 = (0.1 * locals.var_lover_func);
        let assign90200_e138934: f64 = (assign90200_e138930 - assign90200_e138933);
        (assign90200_e138934, ((locals.var_lover_func_dn0 - locals.var_wjunc0_dn0) - (0.1 * locals.var_lover_func_dn0)), ((locals.var_lover_func_dn2 - locals.var_wjunc0_dn2) - (0.1 * locals.var_lover_func_dn2)), ((locals.var_lover_func_dn4 - locals.var_wjunc0_dn4) - (0.1 * locals.var_lover_func_dn4)), ((locals.var_lover_func_dn5 - locals.var_wjunc0_dn5) - (0.1 * locals.var_lover_func_dn5)), ((locals.var_lover_func_dn6 - locals.var_wjunc0_dn6) - (0.1 * locals.var_lover_func_dn6)), ((locals.var_lover_func_dn7 - locals.var_wjunc0_dn7) - (0.1 * locals.var_lover_func_dn7)), ((locals.var_lover_func_dn8 - locals.var_wjunc0_dn8) - (0.1 * locals.var_lover_func_dn8)), ((locals.var_lover_func_dn9 - locals.var_wjunc0_dn9) - (0.1 * locals.var_lover_func_dn9)), ((locals.var_lover_func_dn10 - locals.var_wjunc0_dn10) - (0.1 * locals.var_lover_func_dn10)), ((locals.var_lover_func_dn11 - locals.var_wjunc0_dn11) - (0.1 * locals.var_lover_func_dn11)), ((locals.var_lover_func_dn14 - locals.var_wjunc0_dn14) - (0.1 * locals.var_lover_func_dn14)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign90200_e138936;
        locals.var_tmf1_dn0 = assign90200_e138936_d_n0;
        locals.var_tmf1_dn2 = assign90200_e138936_d_n2;
        locals.var_tmf1_dn4 = assign90200_e138936_d_n4;
        locals.var_tmf1_dn5 = assign90200_e138936_d_n5;
        locals.var_tmf1_dn6 = assign90200_e138936_d_n6;
        locals.var_tmf1_dn7 = assign90200_e138936_d_n7;
        locals.var_tmf1_dn8 = assign90200_e138936_d_n8;
        locals.var_tmf1_dn9 = assign90200_e138936_d_n9;
        locals.var_tmf1_dn10 = assign90200_e138936_d_n10;
        locals.var_tmf1_dn11 = assign90200_e138936_d_n11;
        locals.var_tmf1_dn14 = assign90200_e138936_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign90210_e138950, assign90210_e138950_d_n0, assign90210_e138950_d_n2, assign90210_e138950_d_n4, assign90210_e138950_d_n5, assign90210_e138950_d_n6, assign90210_e138950_d_n7, assign90210_e138950_d_n8, assign90210_e138950_d_n9, assign90210_e138950_d_n10, assign90210_e138950_d_n11, assign90210_e138950_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2104 != 0.0)) && (locals.var_guard2105 != 0.0)) {
        let assign90210_e138944: f64 = (4.0 * locals.var_lover_func);
        let assign90210_e138947: f64 = (0.1 * locals.var_lover_func);
        let assign90210_e138948: f64 = (assign90210_e138944 * assign90210_e138947);
        (assign90210_e138948, (((4.0 * locals.var_lover_func_dn0) * assign90210_e138947) + (assign90210_e138944 * (0.1 * locals.var_lover_func_dn0))), (((4.0 * locals.var_lover_func_dn2) * assign90210_e138947) + (assign90210_e138944 * (0.1 * locals.var_lover_func_dn2))), (((4.0 * locals.var_lover_func_dn4) * assign90210_e138947) + (assign90210_e138944 * (0.1 * locals.var_lover_func_dn4))), (((4.0 * locals.var_lover_func_dn5) * assign90210_e138947) + (assign90210_e138944 * (0.1 * locals.var_lover_func_dn5))), (((4.0 * locals.var_lover_func_dn6) * assign90210_e138947) + (assign90210_e138944 * (0.1 * locals.var_lover_func_dn6))), (((4.0 * locals.var_lover_func_dn7) * assign90210_e138947) + (assign90210_e138944 * (0.1 * locals.var_lover_func_dn7))), (((4.0 * locals.var_lover_func_dn8) * assign90210_e138947) + (assign90210_e138944 * (0.1 * locals.var_lover_func_dn8))), (((4.0 * locals.var_lover_func_dn9) * assign90210_e138947) + (assign90210_e138944 * (0.1 * locals.var_lover_func_dn9))), (((4.0 * locals.var_lover_func_dn10) * assign90210_e138947) + (assign90210_e138944 * (0.1 * locals.var_lover_func_dn10))), (((4.0 * locals.var_lover_func_dn11) * assign90210_e138947) + (assign90210_e138944 * (0.1 * locals.var_lover_func_dn11))), (((4.0 * locals.var_lover_func_dn14) * assign90210_e138947) + (assign90210_e138944 * (0.1 * locals.var_lover_func_dn14))),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign90210_e138950;
        locals.var_tmf2_dn0 = assign90210_e138950_d_n0;
        locals.var_tmf2_dn2 = assign90210_e138950_d_n2;
        locals.var_tmf2_dn4 = assign90210_e138950_d_n4;
        locals.var_tmf2_dn5 = assign90210_e138950_d_n5;
        locals.var_tmf2_dn6 = assign90210_e138950_d_n6;
        locals.var_tmf2_dn7 = assign90210_e138950_d_n7;
        locals.var_tmf2_dn8 = assign90210_e138950_d_n8;
        locals.var_tmf2_dn9 = assign90210_e138950_d_n9;
        locals.var_tmf2_dn10 = assign90210_e138950_d_n10;
        locals.var_tmf2_dn11 = assign90210_e138950_d_n11;
        locals.var_tmf2_dn14 = assign90210_e138950_d_n14;
        locals.var_tmf2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_348(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign90220_e138964, assign90220_e138964_d_n0, assign90220_e138964_d_n2, assign90220_e138964_d_n4, assign90220_e138964_d_n5, assign90220_e138964_d_n6, assign90220_e138964_d_n7, assign90220_e138964_d_n8, assign90220_e138964_d_n9, assign90220_e138964_d_n10, assign90220_e138964_d_n11, assign90220_e138964_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2104 != 0.0)) && (locals.var_guard2105 != 0.0)) {
        let (assign90220_e138962, assign90220_e138962_d_n0, assign90220_e138962_d_n2, assign90220_e138962_d_n4, assign90220_e138962_d_n5, assign90220_e138962_d_n6, assign90220_e138962_d_n7, assign90220_e138962_d_n8, assign90220_e138962_d_n9, assign90220_e138962_d_n10, assign90220_e138962_d_n11, assign90220_e138962_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign90220_e138961: f64 = (-locals.var_tmf2);
                (assign90220_e138961, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign90220_e138962, assign90220_e138962_d_n0, assign90220_e138962_d_n2, assign90220_e138962_d_n4, assign90220_e138962_d_n5, assign90220_e138962_d_n6, assign90220_e138962_d_n7, assign90220_e138962_d_n8, assign90220_e138962_d_n9, assign90220_e138962_d_n10, assign90220_e138962_d_n11, assign90220_e138962_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign90220_e138964;
        locals.var_tmf2_dn0 = assign90220_e138964_d_n0;
        locals.var_tmf2_dn2 = assign90220_e138964_d_n2;
        locals.var_tmf2_dn4 = assign90220_e138964_d_n4;
        locals.var_tmf2_dn5 = assign90220_e138964_d_n5;
        locals.var_tmf2_dn6 = assign90220_e138964_d_n6;
        locals.var_tmf2_dn7 = assign90220_e138964_d_n7;
        locals.var_tmf2_dn8 = assign90220_e138964_d_n8;
        locals.var_tmf2_dn9 = assign90220_e138964_d_n9;
        locals.var_tmf2_dn10 = assign90220_e138964_d_n10;
        locals.var_tmf2_dn11 = assign90220_e138964_d_n11;
        locals.var_tmf2_dn14 = assign90220_e138964_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign90230_e138977, assign90230_e138977_d_n0, assign90230_e138977_d_n2, assign90230_e138977_d_n4, assign90230_e138977_d_n5, assign90230_e138977_d_n6, assign90230_e138977_d_n7, assign90230_e138977_d_n8, assign90230_e138977_d_n9, assign90230_e138977_d_n10, assign90230_e138977_d_n11, assign90230_e138977_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2104 != 0.0)) && (locals.var_guard2105 != 0.0)) {
        let assign90230_e138972: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign90230_e138974: f64 = (assign90230_e138972 + locals.var_tmf2);
        let assign90230_e138975: f64 = (assign90230_e138974).sqrt();
        (assign90230_e138975, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign90230_e138975)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign90230_e138975)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign90230_e138975)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign90230_e138975)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign90230_e138975)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign90230_e138975)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign90230_e138975)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign90230_e138975)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign90230_e138975)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign90230_e138975)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign90230_e138975)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign90230_e138977;
        locals.var_tmf2_dn0 = assign90230_e138977_d_n0;
        locals.var_tmf2_dn2 = assign90230_e138977_d_n2;
        locals.var_tmf2_dn4 = assign90230_e138977_d_n4;
        locals.var_tmf2_dn5 = assign90230_e138977_d_n5;
        locals.var_tmf2_dn6 = assign90230_e138977_d_n6;
        locals.var_tmf2_dn7 = assign90230_e138977_d_n7;
        locals.var_tmf2_dn8 = assign90230_e138977_d_n8;
        locals.var_tmf2_dn9 = assign90230_e138977_d_n9;
        locals.var_tmf2_dn10 = assign90230_e138977_d_n10;
        locals.var_tmf2_dn11 = assign90230_e138977_d_n11;
        locals.var_tmf2_dn14 = assign90230_e138977_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign90240_e138991, assign90240_e138991_d_n0, assign90240_e138991_d_n2, assign90240_e138991_d_n4, assign90240_e138991_d_n5, assign90240_e138991_d_n6, assign90240_e138991_d_n7, assign90240_e138991_d_n8, assign90240_e138991_d_n9, assign90240_e138991_d_n10, assign90240_e138991_d_n11, assign90240_e138991_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2104 != 0.0)) && (locals.var_guard2105 != 0.0)) {
        let assign90240_e138987: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign90240_e138988: f64 = (1.0 + assign90240_e138987);
        let assign90240_e138989: f64 = (0.5 * assign90240_e138988);
        (assign90240_e138989, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign90240_e138991;
        locals.var_t0_dn0 = assign90240_e138991_d_n0;
        locals.var_t0_dn2 = assign90240_e138991_d_n2;
        locals.var_t0_dn4 = assign90240_e138991_d_n4;
        locals.var_t0_dn5 = assign90240_e138991_d_n5;
        locals.var_t0_dn6 = assign90240_e138991_d_n6;
        locals.var_t0_dn7 = assign90240_e138991_d_n7;
        locals.var_t0_dn8 = assign90240_e138991_d_n8;
        locals.var_t0_dn9 = assign90240_e138991_d_n9;
        locals.var_t0_dn10 = assign90240_e138991_d_n10;
        locals.var_t0_dn11 = assign90240_e138991_d_n11;
        locals.var_t0_dn14 = assign90240_e138991_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign90250_e139005, assign90250_e139005_d_n0, assign90250_e139005_d_n2, assign90250_e139005_d_n4, assign90250_e139005_d_n5, assign90250_e139005_d_n6, assign90250_e139005_d_n7, assign90250_e139005_d_n8, assign90250_e139005_d_n9, assign90250_e139005_d_n10, assign90250_e139005_d_n11, assign90250_e139005_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2104 != 0.0)) && (locals.var_guard2105 != 0.0)) {
        let assign90250_e139001: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign90250_e139002: f64 = (0.5 * assign90250_e139001);
        let assign90250_e139003: f64 = (locals.var_lover_func - assign90250_e139002);
        (assign90250_e139003, (locals.var_lover_func_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_lover_func_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_lover_func_dn4 - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_lover_func_dn5 - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_lover_func_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_lover_func_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_lover_func_dn8 - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_lover_func_dn9 - (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (locals.var_lover_func_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_lover_func_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_lover_func_dn14 - (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_wjuncld, locals.var_wjuncld_dn0, locals.var_wjuncld_dn2, locals.var_wjuncld_dn4, locals.var_wjuncld_dn5, locals.var_wjuncld_dn6, locals.var_wjuncld_dn7, locals.var_wjuncld_dn8, locals.var_wjuncld_dn9, locals.var_wjuncld_dn10, locals.var_wjuncld_dn11, locals.var_wjuncld_dn14,)
    }
};
        locals.var_wjuncld = assign90250_e139005;
        locals.var_wjuncld_dn0 = assign90250_e139005_d_n0;
        locals.var_wjuncld_dn2 = assign90250_e139005_d_n2;
        locals.var_wjuncld_dn4 = assign90250_e139005_d_n4;
        locals.var_wjuncld_dn5 = assign90250_e139005_d_n5;
        locals.var_wjuncld_dn6 = assign90250_e139005_d_n6;
        locals.var_wjuncld_dn7 = assign90250_e139005_d_n7;
        locals.var_wjuncld_dn8 = assign90250_e139005_d_n8;
        locals.var_wjuncld_dn9 = assign90250_e139005_d_n9;
        locals.var_wjuncld_dn10 = assign90250_e139005_d_n10;
        locals.var_wjuncld_dn11 = assign90250_e139005_d_n11;
        locals.var_wjuncld_dn14 = assign90250_e139005_d_n14;
        locals.var_wjuncld_rv = 0.0;

        let (assign90260_e139015, assign90260_e139015_d_n0, assign90260_e139015_d_n2, assign90260_e139015_d_n4, assign90260_e139015_d_n5, assign90260_e139015_d_n6, assign90260_e139015_d_n7, assign90260_e139015_d_n8, assign90260_e139015_d_n9, assign90260_e139015_d_n10, assign90260_e139015_d_n11, assign90260_e139015_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2104 != 0.0)) && (locals.var_guard2105 != 0.0)) {
        let assign90260_e139013: f64 = (locals.var_lover_func - locals.var_wjuncld);
        (assign90260_e139013, (locals.var_lover_func_dn0 - locals.var_wjuncld_dn0), (locals.var_lover_func_dn2 - locals.var_wjuncld_dn2), (locals.var_lover_func_dn4 - locals.var_wjuncld_dn4), (locals.var_lover_func_dn5 - locals.var_wjuncld_dn5), (locals.var_lover_func_dn6 - locals.var_wjuncld_dn6), (locals.var_lover_func_dn7 - locals.var_wjuncld_dn7), (locals.var_lover_func_dn8 - locals.var_wjuncld_dn8), (locals.var_lover_func_dn9 - locals.var_wjuncld_dn9), (locals.var_lover_func_dn10 - locals.var_wjuncld_dn10), (locals.var_lover_func_dn11 - locals.var_wjuncld_dn11), (locals.var_lover_func_dn14 - locals.var_wjuncld_dn14),)
    } else {
        (locals.var_lover_func, locals.var_lover_func_dn0, locals.var_lover_func_dn2, locals.var_lover_func_dn4, locals.var_lover_func_dn5, locals.var_lover_func_dn6, locals.var_lover_func_dn7, locals.var_lover_func_dn8, locals.var_lover_func_dn9, locals.var_lover_func_dn10, locals.var_lover_func_dn11, locals.var_lover_func_dn14,)
    }
};
        locals.var_lover_func = assign90260_e139015;
        locals.var_lover_func_dn0 = assign90260_e139015_d_n0;
        locals.var_lover_func_dn2 = assign90260_e139015_d_n2;
        locals.var_lover_func_dn4 = assign90260_e139015_d_n4;
        locals.var_lover_func_dn5 = assign90260_e139015_d_n5;
        locals.var_lover_func_dn6 = assign90260_e139015_d_n6;
        locals.var_lover_func_dn7 = assign90260_e139015_d_n7;
        locals.var_lover_func_dn8 = assign90260_e139015_d_n8;
        locals.var_lover_func_dn9 = assign90260_e139015_d_n9;
        locals.var_lover_func_dn10 = assign90260_e139015_d_n10;
        locals.var_lover_func_dn11 = assign90260_e139015_d_n11;
        locals.var_lover_func_dn14 = assign90260_e139015_d_n14;
        locals.var_lover_func_rv = 0.0;

        let assign90270_e139018: f64 = if 4.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2108 = assign90270_e139018;
        locals.var_guard2108_rv = 0.0;

        let assign90280_e139021: f64 = if 4.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard2109 = assign90280_e139021;
        locals.var_guard2109_rv = 0.0;

        let assign90290_e139024: f64 = if 4.0 == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard2110 = assign90290_e139024;
        locals.var_guard2110_rv = 0.0;

        let assign90300_e139027: f64 = if 4.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard2111 = assign90300_e139027;
        locals.var_guard2111_rv = 0.0;

        let assign90310_e139030: f64 = if p.p55 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2112 = assign90310_e139030;
        locals.var_guard2112_rv = 0.0;

        let (assign90320_e139040, assign90320_e139040_d_n0, assign90320_e139040_d_n2, assign90320_e139040_d_n4, assign90320_e139040_d_n5, assign90320_e139040_d_n6, assign90320_e139040_d_n7, assign90320_e139040_d_n8, assign90320_e139040_d_n9, assign90320_e139040_d_n10, assign90320_e139040_d_n11, assign90320_e139040_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2108 != 0.0)) && (locals.var_guard2112 != 0.0)) {
        let assign90320_e139038: f64 = (locals.var_weffcv_nf * locals.var_lover_func);
        (assign90320_e139038, (locals.var_weffcv_nf * locals.var_lover_func_dn0), (locals.var_weffcv_nf * locals.var_lover_func_dn2), (locals.var_weffcv_nf * locals.var_lover_func_dn4), (locals.var_weffcv_nf * locals.var_lover_func_dn5), (locals.var_weffcv_nf * locals.var_lover_func_dn6), (locals.var_weffcv_nf * locals.var_lover_func_dn7), (locals.var_weffcv_nf * locals.var_lover_func_dn8), (locals.var_weffcv_nf * locals.var_lover_func_dn9), (locals.var_weffcv_nf * locals.var_lover_func_dn10), (locals.var_weffcv_nf * locals.var_lover_func_dn11), (locals.var_weffcv_nf * locals.var_lover_func_dn14),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign90320_e139040;
        locals.var_t4_dn0 = assign90320_e139040_d_n0;
        locals.var_t4_dn2 = assign90320_e139040_d_n2;
        locals.var_t4_dn4 = assign90320_e139040_d_n4;
        locals.var_t4_dn5 = assign90320_e139040_d_n5;
        locals.var_t4_dn6 = assign90320_e139040_d_n6;
        locals.var_t4_dn7 = assign90320_e139040_d_n7;
        locals.var_t4_dn8 = assign90320_e139040_d_n8;
        locals.var_t4_dn9 = assign90320_e139040_d_n9;
        locals.var_t4_dn10 = assign90320_e139040_d_n10;
        locals.var_t4_dn11 = assign90320_e139040_d_n11;
        locals.var_t4_dn14 = assign90320_e139040_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign90330_e139055, assign90330_e139055_d_n0, assign90330_e139055_d_n2, assign90330_e139055_d_n4, assign90330_e139055_d_n5, assign90330_e139055_d_n6, assign90330_e139055_d_n7, assign90330_e139055_d_n8, assign90330_e139055_d_n9, assign90330_e139055_d_n10, assign90330_e139055_d_n11, assign90330_e139055_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2108 != 0.0)) && (locals.var_guard2112 == 0.0)) {
        let assign90330_e139049: f64 = (locals.var_weffcv_nf * locals.var_lover_func);
        let assign90330_e139052: f64 = (1.0 - locals.var_uc_cvdsover);
        let assign90330_e139053: f64 = (assign90330_e139049 * assign90330_e139052);
        (assign90330_e139053, ((locals.var_weffcv_nf * locals.var_lover_func_dn0) * assign90330_e139052), ((locals.var_weffcv_nf * locals.var_lover_func_dn2) * assign90330_e139052), ((locals.var_weffcv_nf * locals.var_lover_func_dn4) * assign90330_e139052), ((locals.var_weffcv_nf * locals.var_lover_func_dn5) * assign90330_e139052), ((locals.var_weffcv_nf * locals.var_lover_func_dn6) * assign90330_e139052), ((locals.var_weffcv_nf * locals.var_lover_func_dn7) * assign90330_e139052), ((locals.var_weffcv_nf * locals.var_lover_func_dn8) * assign90330_e139052), ((locals.var_weffcv_nf * locals.var_lover_func_dn9) * assign90330_e139052), ((locals.var_weffcv_nf * locals.var_lover_func_dn10) * assign90330_e139052), ((locals.var_weffcv_nf * locals.var_lover_func_dn11) * assign90330_e139052), ((locals.var_weffcv_nf * locals.var_lover_func_dn14) * assign90330_e139052),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign90330_e139055;
        locals.var_t4_dn0 = assign90330_e139055_d_n0;
        locals.var_t4_dn2 = assign90330_e139055_d_n2;
        locals.var_t4_dn4 = assign90330_e139055_d_n4;
        locals.var_t4_dn5 = assign90330_e139055_d_n5;
        locals.var_t4_dn6 = assign90330_e139055_d_n6;
        locals.var_t4_dn7 = assign90330_e139055_d_n7;
        locals.var_t4_dn8 = assign90330_e139055_d_n8;
        locals.var_t4_dn9 = assign90330_e139055_d_n9;
        locals.var_t4_dn10 = assign90330_e139055_d_n10;
        locals.var_t4_dn11 = assign90330_e139055_d_n11;
        locals.var_t4_dn14 = assign90330_e139055_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign90340_e139063, assign90340_e139063_d_n0, assign90340_e139063_d_n2, assign90340_e139063_d_n4, assign90340_e139063_d_n5, assign90340_e139063_d_n6, assign90340_e139063_d_n7, assign90340_e139063_d_n8, assign90340_e139063_d_n9, assign90340_e139063_d_n10, assign90340_e139063_d_n11, assign90340_e139063_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2108 != 0.0)) {
        let assign90340_e139061: f64 = (locals.var_t4 * locals.var_qsuld);
        (assign90340_e139061, ((locals.var_t4_dn0 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn0)), ((locals.var_t4_dn2 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn2)), ((locals.var_t4_dn4 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn4)), ((locals.var_t4_dn5 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn5)), ((locals.var_t4_dn6 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn6)), ((locals.var_t4_dn7 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn7)), ((locals.var_t4_dn8 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn8)), ((locals.var_t4_dn9 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn9)), ((locals.var_t4_dn10 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn10)), ((locals.var_t4_dn11 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn11)), ((locals.var_t4_dn14 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn14)),)
    } else {
        (locals.var_qovs, locals.var_qovs_dn0, locals.var_qovs_dn2, locals.var_qovs_dn4, locals.var_qovs_dn5, locals.var_qovs_dn6, locals.var_qovs_dn7, locals.var_qovs_dn8, locals.var_qovs_dn9, locals.var_qovs_dn10, locals.var_qovs_dn11, locals.var_qovs_dn14,)
    }
};
        locals.var_qovs = assign90340_e139063;
        locals.var_qovs_dn0 = assign90340_e139063_d_n0;
        locals.var_qovs_dn2 = assign90340_e139063_d_n2;
        locals.var_qovs_dn4 = assign90340_e139063_d_n4;
        locals.var_qovs_dn5 = assign90340_e139063_d_n5;
        locals.var_qovs_dn6 = assign90340_e139063_d_n6;
        locals.var_qovs_dn7 = assign90340_e139063_d_n7;
        locals.var_qovs_dn8 = assign90340_e139063_d_n8;
        locals.var_qovs_dn9 = assign90340_e139063_d_n9;
        locals.var_qovs_dn10 = assign90340_e139063_d_n10;
        locals.var_qovs_dn11 = assign90340_e139063_d_n11;
        locals.var_qovs_dn14 = assign90340_e139063_d_n14;
        locals.var_qovs_rv = 0.0;

        let (assign90350_e139071, assign90350_e139071_d_n0, assign90350_e139071_d_n2, assign90350_e139071_d_n4, assign90350_e139071_d_n5, assign90350_e139071_d_n6, assign90350_e139071_d_n7, assign90350_e139071_d_n8, assign90350_e139071_d_n9, assign90350_e139071_d_n10, assign90350_e139071_d_n11, assign90350_e139071_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2108 != 0.0)) {
        let assign90350_e139069: f64 = (locals.var_t4 * locals.var_qbuld);
        (assign90350_e139069, ((locals.var_t4_dn0 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn0)), ((locals.var_t4_dn2 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn2)), ((locals.var_t4_dn4 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn4)), ((locals.var_t4_dn5 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn5)), ((locals.var_t4_dn6 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn6)), ((locals.var_t4_dn7 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn7)), ((locals.var_t4_dn8 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn8)), ((locals.var_t4_dn9 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn9)), ((locals.var_t4_dn10 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn10)), ((locals.var_t4_dn11 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn11)), ((locals.var_t4_dn14 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn14)),)
    } else {
        (locals.var_qbsld, locals.var_qbsld_dn0, locals.var_qbsld_dn2, locals.var_qbsld_dn4, locals.var_qbsld_dn5, locals.var_qbsld_dn6, locals.var_qbsld_dn7, locals.var_qbsld_dn8, locals.var_qbsld_dn9, locals.var_qbsld_dn10, locals.var_qbsld_dn11, locals.var_qbsld_dn14,)
    }
};
        locals.var_qbsld = assign90350_e139071;
        locals.var_qbsld_dn0 = assign90350_e139071_d_n0;
        locals.var_qbsld_dn2 = assign90350_e139071_d_n2;
        locals.var_qbsld_dn4 = assign90350_e139071_d_n4;
        locals.var_qbsld_dn5 = assign90350_e139071_d_n5;
        locals.var_qbsld_dn6 = assign90350_e139071_d_n6;
        locals.var_qbsld_dn7 = assign90350_e139071_d_n7;
        locals.var_qbsld_dn8 = assign90350_e139071_d_n8;
        locals.var_qbsld_dn9 = assign90350_e139071_d_n9;
        locals.var_qbsld_dn10 = assign90350_e139071_d_n10;
        locals.var_qbsld_dn11 = assign90350_e139071_d_n11;
        locals.var_qbsld_dn14 = assign90350_e139071_d_n14;
        locals.var_qbsld_rv = 0.0;

        let (assign90380_e139096, assign90380_e139096_d_n0, assign90380_e139096_d_n2, assign90380_e139096_d_n4, assign90380_e139096_d_n5, assign90380_e139096_d_n6, assign90380_e139096_d_n7, assign90380_e139096_d_n8, assign90380_e139096_d_n9, assign90380_e139096_d_n10, assign90380_e139096_d_n11, assign90380_e139096_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard2109 != 0.0) && (locals.var_guard2108 == 0.0))) {
        let assign90380_e139092: f64 = (locals.var_weffcv_nf * locals.var_lover_func);
        let assign90380_e139094: f64 = (assign90380_e139092 * locals.var_uc_cvdsover);
        (assign90380_e139094, ((locals.var_weffcv_nf * locals.var_lover_func_dn0) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn2) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn4) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn5) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn6) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn7) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn8) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn9) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn10) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn11) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn14) * locals.var_uc_cvdsover),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign90380_e139096;
        locals.var_t4_dn0 = assign90380_e139096_d_n0;
        locals.var_t4_dn2 = assign90380_e139096_d_n2;
        locals.var_t4_dn4 = assign90380_e139096_d_n4;
        locals.var_t4_dn5 = assign90380_e139096_d_n5;
        locals.var_t4_dn6 = assign90380_e139096_d_n6;
        locals.var_t4_dn7 = assign90380_e139096_d_n7;
        locals.var_t4_dn8 = assign90380_e139096_d_n8;
        locals.var_t4_dn9 = assign90380_e139096_d_n9;
        locals.var_t4_dn10 = assign90380_e139096_d_n10;
        locals.var_t4_dn11 = assign90380_e139096_d_n11;
        locals.var_t4_dn14 = assign90380_e139096_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign90390_e139107, assign90390_e139107_d_n0, assign90390_e139107_d_n2, assign90390_e139107_d_n4, assign90390_e139107_d_n5, assign90390_e139107_d_n6, assign90390_e139107_d_n7, assign90390_e139107_d_n8, assign90390_e139107_d_n9, assign90390_e139107_d_n10, assign90390_e139107_d_n11, assign90390_e139107_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard2109 != 0.0) && (locals.var_guard2108 == 0.0))) {
        let assign90390_e139105: f64 = (locals.var_t4 * locals.var_qsuld);
        (assign90390_e139105, ((locals.var_t4_dn0 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn0)), ((locals.var_t4_dn2 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn2)), ((locals.var_t4_dn4 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn4)), ((locals.var_t4_dn5 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn5)), ((locals.var_t4_dn6 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn6)), ((locals.var_t4_dn7 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn7)), ((locals.var_t4_dn8 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn8)), ((locals.var_t4_dn9 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn9)), ((locals.var_t4_dn10 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn10)), ((locals.var_t4_dn11 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn11)), ((locals.var_t4_dn14 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn14)),)
    } else {
        (locals.var_qovsext, locals.var_qovsext_dn0, locals.var_qovsext_dn2, locals.var_qovsext_dn4, locals.var_qovsext_dn5, locals.var_qovsext_dn6, locals.var_qovsext_dn7, locals.var_qovsext_dn8, locals.var_qovsext_dn9, locals.var_qovsext_dn10, locals.var_qovsext_dn11, locals.var_qovsext_dn14,)
    }
};
        locals.var_qovsext = assign90390_e139107;
        locals.var_qovsext_dn0 = assign90390_e139107_d_n0;
        locals.var_qovsext_dn2 = assign90390_e139107_d_n2;
        locals.var_qovsext_dn4 = assign90390_e139107_d_n4;
        locals.var_qovsext_dn5 = assign90390_e139107_d_n5;
        locals.var_qovsext_dn6 = assign90390_e139107_d_n6;
        locals.var_qovsext_dn7 = assign90390_e139107_d_n7;
        locals.var_qovsext_dn8 = assign90390_e139107_d_n8;
        locals.var_qovsext_dn9 = assign90390_e139107_d_n9;
        locals.var_qovsext_dn10 = assign90390_e139107_d_n10;
        locals.var_qovsext_dn11 = assign90390_e139107_d_n11;
        locals.var_qovsext_dn14 = assign90390_e139107_d_n14;
        locals.var_qovsext_rv = 0.0;

        let (assign90400_e139118, assign90400_e139118_d_n0, assign90400_e139118_d_n2, assign90400_e139118_d_n4, assign90400_e139118_d_n5, assign90400_e139118_d_n6, assign90400_e139118_d_n7, assign90400_e139118_d_n8, assign90400_e139118_d_n9, assign90400_e139118_d_n10, assign90400_e139118_d_n11, assign90400_e139118_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard2109 != 0.0) && (locals.var_guard2108 == 0.0))) {
        let assign90400_e139116: f64 = (locals.var_t4 * locals.var_qbuld);
        (assign90400_e139116, ((locals.var_t4_dn0 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn0)), ((locals.var_t4_dn2 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn2)), ((locals.var_t4_dn4 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn4)), ((locals.var_t4_dn5 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn5)), ((locals.var_t4_dn6 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn6)), ((locals.var_t4_dn7 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn7)), ((locals.var_t4_dn8 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn8)), ((locals.var_t4_dn9 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn9)), ((locals.var_t4_dn10 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn10)), ((locals.var_t4_dn11 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn11)), ((locals.var_t4_dn14 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn14)),)
    } else {
        (locals.var_qbsldext, locals.var_qbsldext_dn0, locals.var_qbsldext_dn2, locals.var_qbsldext_dn4, locals.var_qbsldext_dn5, locals.var_qbsldext_dn6, locals.var_qbsldext_dn7, locals.var_qbsldext_dn8, locals.var_qbsldext_dn9, locals.var_qbsldext_dn10, locals.var_qbsldext_dn11, locals.var_qbsldext_dn14,)
    }
};
        locals.var_qbsldext = assign90400_e139118;
        locals.var_qbsldext_dn0 = assign90400_e139118_d_n0;
        locals.var_qbsldext_dn2 = assign90400_e139118_d_n2;
        locals.var_qbsldext_dn4 = assign90400_e139118_d_n4;
        locals.var_qbsldext_dn5 = assign90400_e139118_d_n5;
        locals.var_qbsldext_dn6 = assign90400_e139118_d_n6;
        locals.var_qbsldext_dn7 = assign90400_e139118_d_n7;
        locals.var_qbsldext_dn8 = assign90400_e139118_d_n8;
        locals.var_qbsldext_dn9 = assign90400_e139118_d_n9;
        locals.var_qbsldext_dn10 = assign90400_e139118_d_n10;
        locals.var_qbsldext_dn11 = assign90400_e139118_d_n11;
        locals.var_qbsldext_dn14 = assign90400_e139118_d_n14;
        locals.var_qbsldext_rv = 0.0;

        let assign90410_e139121: f64 = if p.p55 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2113 = assign90410_e139121;
        locals.var_guard2113_rv = 0.0;

        let (assign90420_e139136, assign90420_e139136_d_n0, assign90420_e139136_d_n2, assign90420_e139136_d_n4, assign90420_e139136_d_n5, assign90420_e139136_d_n6, assign90420_e139136_d_n7, assign90420_e139136_d_n8, assign90420_e139136_d_n9, assign90420_e139136_d_n10, assign90420_e139136_d_n11, assign90420_e139136_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && ((locals.var_guard2110 != 0.0) && (!((locals.var_guard2108 != 0.0) || (locals.var_guard2109 != 0.0))))) && (locals.var_guard2113 != 0.0)) {
        let assign90420_e139134: f64 = (locals.var_weffcv_nf * locals.var_lover_func);
        (assign90420_e139134, (locals.var_weffcv_nf * locals.var_lover_func_dn0), (locals.var_weffcv_nf * locals.var_lover_func_dn2), (locals.var_weffcv_nf * locals.var_lover_func_dn4), (locals.var_weffcv_nf * locals.var_lover_func_dn5), (locals.var_weffcv_nf * locals.var_lover_func_dn6), (locals.var_weffcv_nf * locals.var_lover_func_dn7), (locals.var_weffcv_nf * locals.var_lover_func_dn8), (locals.var_weffcv_nf * locals.var_lover_func_dn9), (locals.var_weffcv_nf * locals.var_lover_func_dn10), (locals.var_weffcv_nf * locals.var_lover_func_dn11), (locals.var_weffcv_nf * locals.var_lover_func_dn14),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign90420_e139136;
        locals.var_t4_dn0 = assign90420_e139136_d_n0;
        locals.var_t4_dn2 = assign90420_e139136_d_n2;
        locals.var_t4_dn4 = assign90420_e139136_d_n4;
        locals.var_t4_dn5 = assign90420_e139136_d_n5;
        locals.var_t4_dn6 = assign90420_e139136_d_n6;
        locals.var_t4_dn7 = assign90420_e139136_d_n7;
        locals.var_t4_dn8 = assign90420_e139136_d_n8;
        locals.var_t4_dn9 = assign90420_e139136_d_n9;
        locals.var_t4_dn10 = assign90420_e139136_d_n10;
        locals.var_t4_dn11 = assign90420_e139136_d_n11;
        locals.var_t4_dn14 = assign90420_e139136_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign90430_e139156, assign90430_e139156_d_n0, assign90430_e139156_d_n2, assign90430_e139156_d_n4, assign90430_e139156_d_n5, assign90430_e139156_d_n6, assign90430_e139156_d_n7, assign90430_e139156_d_n8, assign90430_e139156_d_n9, assign90430_e139156_d_n10, assign90430_e139156_d_n11, assign90430_e139156_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && ((locals.var_guard2110 != 0.0) && (!((locals.var_guard2108 != 0.0) || (locals.var_guard2109 != 0.0))))) && (locals.var_guard2113 == 0.0)) {
        let assign90430_e139150: f64 = (locals.var_weffcv_nf * locals.var_lover_func);
        let assign90430_e139153: f64 = (1.0 - locals.var_uc_cvdsover);
        let assign90430_e139154: f64 = (assign90430_e139150 * assign90430_e139153);
        (assign90430_e139154, ((locals.var_weffcv_nf * locals.var_lover_func_dn0) * assign90430_e139153), ((locals.var_weffcv_nf * locals.var_lover_func_dn2) * assign90430_e139153), ((locals.var_weffcv_nf * locals.var_lover_func_dn4) * assign90430_e139153), ((locals.var_weffcv_nf * locals.var_lover_func_dn5) * assign90430_e139153), ((locals.var_weffcv_nf * locals.var_lover_func_dn6) * assign90430_e139153), ((locals.var_weffcv_nf * locals.var_lover_func_dn7) * assign90430_e139153), ((locals.var_weffcv_nf * locals.var_lover_func_dn8) * assign90430_e139153), ((locals.var_weffcv_nf * locals.var_lover_func_dn9) * assign90430_e139153), ((locals.var_weffcv_nf * locals.var_lover_func_dn10) * assign90430_e139153), ((locals.var_weffcv_nf * locals.var_lover_func_dn11) * assign90430_e139153), ((locals.var_weffcv_nf * locals.var_lover_func_dn14) * assign90430_e139153),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign90430_e139156;
        locals.var_t4_dn0 = assign90430_e139156_d_n0;
        locals.var_t4_dn2 = assign90430_e139156_d_n2;
        locals.var_t4_dn4 = assign90430_e139156_d_n4;
        locals.var_t4_dn5 = assign90430_e139156_d_n5;
        locals.var_t4_dn6 = assign90430_e139156_d_n6;
        locals.var_t4_dn7 = assign90430_e139156_d_n7;
        locals.var_t4_dn8 = assign90430_e139156_d_n8;
        locals.var_t4_dn9 = assign90430_e139156_d_n9;
        locals.var_t4_dn10 = assign90430_e139156_d_n10;
        locals.var_t4_dn11 = assign90430_e139156_d_n11;
        locals.var_t4_dn14 = assign90430_e139156_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign90440_e139167, assign90440_e139167_d_n0, assign90440_e139167_d_n2, assign90440_e139167_d_n4, assign90440_e139167_d_n5, assign90440_e139167_d_n6, assign90440_e139167_d_n7, assign90440_e139167_d_n8, assign90440_e139167_d_n9, assign90440_e139167_d_n10, assign90440_e139167_d_n11, assign90440_e139167_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard2110 != 0.0) && (!((locals.var_guard2108 != 0.0) || (locals.var_guard2109 != 0.0))))) {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn14,)
    } else {
        (locals.var_rd_ps0ld, locals.var_rd_ps0ld_dn0, locals.var_rd_ps0ld_dn2, locals.var_rd_ps0ld_dn4, locals.var_rd_ps0ld_dn5, locals.var_rd_ps0ld_dn6, locals.var_rd_ps0ld_dn7, locals.var_rd_ps0ld_dn8, locals.var_rd_ps0ld_dn9, locals.var_rd_ps0ld_dn10, locals.var_rd_ps0ld_dn11, locals.var_rd_ps0ld_dn14,)
    }
};
        locals.var_rd_ps0ld = assign90440_e139167;
        locals.var_rd_ps0ld_dn0 = assign90440_e139167_d_n0;
        locals.var_rd_ps0ld_dn2 = assign90440_e139167_d_n2;
        locals.var_rd_ps0ld_dn4 = assign90440_e139167_d_n4;
        locals.var_rd_ps0ld_dn5 = assign90440_e139167_d_n5;
        locals.var_rd_ps0ld_dn6 = assign90440_e139167_d_n6;
        locals.var_rd_ps0ld_dn7 = assign90440_e139167_d_n7;
        locals.var_rd_ps0ld_dn8 = assign90440_e139167_d_n8;
        locals.var_rd_ps0ld_dn9 = assign90440_e139167_d_n9;
        locals.var_rd_ps0ld_dn10 = assign90440_e139167_d_n10;
        locals.var_rd_ps0ld_dn11 = assign90440_e139167_d_n11;
        locals.var_rd_ps0ld_dn14 = assign90440_e139167_d_n14;
        locals.var_rd_ps0ld_rv = 0.0;

        let assign90450_e139170: f64 = if p.p430 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2114 = assign90450_e139170;
        locals.var_guard2114_rv = 0.0;

        let (assign90460_e139183, assign90460_e139183_d_n0, assign90460_e139183_d_n2, assign90460_e139183_d_n4, assign90460_e139183_d_n5, assign90460_e139183_d_n6, assign90460_e139183_d_n7, assign90460_e139183_d_n8, assign90460_e139183_d_n9, assign90460_e139183_d_n10, assign90460_e139183_d_n11, assign90460_e139183_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && ((locals.var_guard2110 != 0.0) && (!((locals.var_guard2108 != 0.0) || (locals.var_guard2109 != 0.0))))) && (locals.var_guard2114 != 0.0)) {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn8, locals.var_qbuld_dn9, locals.var_qbuld_dn10, locals.var_qbuld_dn11, locals.var_qbuld_dn14,)
    } else {
        (locals.var_rd_qbuld, locals.var_rd_qbuld_dn0, locals.var_rd_qbuld_dn2, locals.var_rd_qbuld_dn4, locals.var_rd_qbuld_dn5, locals.var_rd_qbuld_dn6, locals.var_rd_qbuld_dn7, locals.var_rd_qbuld_dn8, locals.var_rd_qbuld_dn9, locals.var_rd_qbuld_dn10, locals.var_rd_qbuld_dn11, locals.var_rd_qbuld_dn14,)
    }
};
        locals.var_rd_qbuld = assign90460_e139183;
        locals.var_rd_qbuld_dn0 = assign90460_e139183_d_n0;
        locals.var_rd_qbuld_dn2 = assign90460_e139183_d_n2;
        locals.var_rd_qbuld_dn4 = assign90460_e139183_d_n4;
        locals.var_rd_qbuld_dn5 = assign90460_e139183_d_n5;
        locals.var_rd_qbuld_dn6 = assign90460_e139183_d_n6;
        locals.var_rd_qbuld_dn7 = assign90460_e139183_d_n7;
        locals.var_rd_qbuld_dn8 = assign90460_e139183_d_n8;
        locals.var_rd_qbuld_dn9 = assign90460_e139183_d_n9;
        locals.var_rd_qbuld_dn10 = assign90460_e139183_d_n10;
        locals.var_rd_qbuld_dn11 = assign90460_e139183_d_n11;
        locals.var_rd_qbuld_dn14 = assign90460_e139183_d_n14;
        locals.var_rd_qbuld_rv = 0.0;

        let (assign90470_e139196, assign90470_e139196_d_n0, assign90470_e139196_d_n2, assign90470_e139196_d_n4, assign90470_e139196_d_n5, assign90470_e139196_d_n6, assign90470_e139196_d_n7, assign90470_e139196_d_n8, assign90470_e139196_d_n9, assign90470_e139196_d_n10, assign90470_e139196_d_n11, assign90470_e139196_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard2110 != 0.0) && (!((locals.var_guard2108 != 0.0) || (locals.var_guard2109 != 0.0))))) {
        let assign90470_e139194: f64 = (locals.var_t4 * locals.var_qsuld);
        (assign90470_e139194, ((locals.var_t4_dn0 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn0)), ((locals.var_t4_dn2 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn2)), ((locals.var_t4_dn4 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn4)), ((locals.var_t4_dn5 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn5)), ((locals.var_t4_dn6 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn6)), ((locals.var_t4_dn7 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn7)), ((locals.var_t4_dn8 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn8)), ((locals.var_t4_dn9 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn9)), ((locals.var_t4_dn10 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn10)), ((locals.var_t4_dn11 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn11)), ((locals.var_t4_dn14 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn14)),)
    } else {
        (locals.var_qovd, locals.var_qovd_dn0, locals.var_qovd_dn2, locals.var_qovd_dn4, locals.var_qovd_dn5, locals.var_qovd_dn6, locals.var_qovd_dn7, locals.var_qovd_dn8, locals.var_qovd_dn9, locals.var_qovd_dn10, locals.var_qovd_dn11, locals.var_qovd_dn14,)
    }
};
        locals.var_qovd = assign90470_e139196;
        locals.var_qovd_dn0 = assign90470_e139196_d_n0;
        locals.var_qovd_dn2 = assign90470_e139196_d_n2;
        locals.var_qovd_dn4 = assign90470_e139196_d_n4;
        locals.var_qovd_dn5 = assign90470_e139196_d_n5;
        locals.var_qovd_dn6 = assign90470_e139196_d_n6;
        locals.var_qovd_dn7 = assign90470_e139196_d_n7;
        locals.var_qovd_dn8 = assign90470_e139196_d_n8;
        locals.var_qovd_dn9 = assign90470_e139196_d_n9;
        locals.var_qovd_dn10 = assign90470_e139196_d_n10;
        locals.var_qovd_dn11 = assign90470_e139196_d_n11;
        locals.var_qovd_dn14 = assign90470_e139196_d_n14;
        locals.var_qovd_rv = 0.0;

        let (assign90480_e139209, assign90480_e139209_d_n0, assign90480_e139209_d_n2, assign90480_e139209_d_n4, assign90480_e139209_d_n5, assign90480_e139209_d_n6, assign90480_e139209_d_n7, assign90480_e139209_d_n8, assign90480_e139209_d_n9, assign90480_e139209_d_n10, assign90480_e139209_d_n11, assign90480_e139209_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard2110 != 0.0) && (!((locals.var_guard2108 != 0.0) || (locals.var_guard2109 != 0.0))))) {
        let assign90480_e139207: f64 = (locals.var_t4 * locals.var_qbuld);
        (assign90480_e139207, ((locals.var_t4_dn0 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn0)), ((locals.var_t4_dn2 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn2)), ((locals.var_t4_dn4 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn4)), ((locals.var_t4_dn5 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn5)), ((locals.var_t4_dn6 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn6)), ((locals.var_t4_dn7 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn7)), ((locals.var_t4_dn8 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn8)), ((locals.var_t4_dn9 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn9)), ((locals.var_t4_dn10 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn10)), ((locals.var_t4_dn11 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn11)), ((locals.var_t4_dn14 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn14)),)
    } else {
        (locals.var_qbdld, locals.var_qbdld_dn0, locals.var_qbdld_dn2, locals.var_qbdld_dn4, locals.var_qbdld_dn5, locals.var_qbdld_dn6, locals.var_qbdld_dn7, locals.var_qbdld_dn8, locals.var_qbdld_dn9, locals.var_qbdld_dn10, locals.var_qbdld_dn11, locals.var_qbdld_dn14,)
    }
};
        locals.var_qbdld = assign90480_e139209;
        locals.var_qbdld_dn0 = assign90480_e139209_d_n0;
        locals.var_qbdld_dn2 = assign90480_e139209_d_n2;
        locals.var_qbdld_dn4 = assign90480_e139209_d_n4;
        locals.var_qbdld_dn5 = assign90480_e139209_d_n5;
        locals.var_qbdld_dn6 = assign90480_e139209_d_n6;
        locals.var_qbdld_dn7 = assign90480_e139209_d_n7;
        locals.var_qbdld_dn8 = assign90480_e139209_d_n8;
        locals.var_qbdld_dn9 = assign90480_e139209_d_n9;
        locals.var_qbdld_dn10 = assign90480_e139209_d_n10;
        locals.var_qbdld_dn11 = assign90480_e139209_d_n11;
        locals.var_qbdld_dn14 = assign90480_e139209_d_n14;
        locals.var_qbdld_rv = 0.0;

        let (assign90490_e139220, assign90490_e139220_d_n0, assign90490_e139220_d_n2, assign90490_e139220_d_n4, assign90490_e139220_d_n5, assign90490_e139220_d_n6, assign90490_e139220_d_n7, assign90490_e139220_d_n8, assign90490_e139220_d_n9, assign90490_e139220_d_n10, assign90490_e139220_d_n11, assign90490_e139220_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard2110 != 0.0) && (!((locals.var_guard2108 != 0.0) || (locals.var_guard2109 != 0.0))))) {
        (locals.var_qbdld, locals.var_qbdld_dn0, locals.var_qbdld_dn2, locals.var_qbdld_dn4, locals.var_qbdld_dn5, locals.var_qbdld_dn6, locals.var_qbdld_dn7, locals.var_qbdld_dn8, locals.var_qbdld_dn9, locals.var_qbdld_dn10, locals.var_qbdld_dn11, locals.var_qbdld_dn14,)
    } else {
        (locals.var_qbd_qs, locals.var_qbd_qs_dn0, locals.var_qbd_qs_dn2, locals.var_qbd_qs_dn4, locals.var_qbd_qs_dn5, locals.var_qbd_qs_dn6, locals.var_qbd_qs_dn7, locals.var_qbd_qs_dn8, locals.var_qbd_qs_dn9, locals.var_qbd_qs_dn10, locals.var_qbd_qs_dn11, locals.var_qbd_qs_dn14,)
    }
};
        locals.var_qbd_qs = assign90490_e139220;
        locals.var_qbd_qs_dn0 = assign90490_e139220_d_n0;
        locals.var_qbd_qs_dn2 = assign90490_e139220_d_n2;
        locals.var_qbd_qs_dn4 = assign90490_e139220_d_n4;
        locals.var_qbd_qs_dn5 = assign90490_e139220_d_n5;
        locals.var_qbd_qs_dn6 = assign90490_e139220_d_n6;
        locals.var_qbd_qs_dn7 = assign90490_e139220_d_n7;
        locals.var_qbd_qs_dn8 = assign90490_e139220_d_n8;
        locals.var_qbd_qs_dn9 = assign90490_e139220_d_n9;
        locals.var_qbd_qs_dn10 = assign90490_e139220_d_n10;
        locals.var_qbd_qs_dn11 = assign90490_e139220_d_n11;
        locals.var_qbd_qs_dn14 = assign90490_e139220_d_n14;
        locals.var_qbd_qs_rv = 0.0;

        let (assign90500_e139237, assign90500_e139237_d_n0, assign90500_e139237_d_n2, assign90500_e139237_d_n4, assign90500_e139237_d_n5, assign90500_e139237_d_n6, assign90500_e139237_d_n7, assign90500_e139237_d_n8, assign90500_e139237_d_n9, assign90500_e139237_d_n10, assign90500_e139237_d_n11, assign90500_e139237_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard2111 != 0.0) && (!(((locals.var_guard2108 != 0.0) || (locals.var_guard2109 != 0.0)) || (locals.var_guard2110 != 0.0))))) {
        let assign90500_e139233: f64 = (locals.var_weffcv_nf * locals.var_lover_func);
        let assign90500_e139235: f64 = (assign90500_e139233 * locals.var_uc_cvdsover);
        (assign90500_e139235, ((locals.var_weffcv_nf * locals.var_lover_func_dn0) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn2) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn4) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn5) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn6) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn7) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn8) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn9) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn10) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn11) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn14) * locals.var_uc_cvdsover),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign90500_e139237;
        locals.var_t4_dn0 = assign90500_e139237_d_n0;
        locals.var_t4_dn2 = assign90500_e139237_d_n2;
        locals.var_t4_dn4 = assign90500_e139237_d_n4;
        locals.var_t4_dn5 = assign90500_e139237_d_n5;
        locals.var_t4_dn6 = assign90500_e139237_d_n6;
        locals.var_t4_dn7 = assign90500_e139237_d_n7;
        locals.var_t4_dn8 = assign90500_e139237_d_n8;
        locals.var_t4_dn9 = assign90500_e139237_d_n9;
        locals.var_t4_dn10 = assign90500_e139237_d_n10;
        locals.var_t4_dn11 = assign90500_e139237_d_n11;
        locals.var_t4_dn14 = assign90500_e139237_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign90510_e139252, assign90510_e139252_d_n0, assign90510_e139252_d_n2, assign90510_e139252_d_n4, assign90510_e139252_d_n5, assign90510_e139252_d_n6, assign90510_e139252_d_n7, assign90510_e139252_d_n8, assign90510_e139252_d_n9, assign90510_e139252_d_n10, assign90510_e139252_d_n11, assign90510_e139252_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard2111 != 0.0) && (!(((locals.var_guard2108 != 0.0) || (locals.var_guard2109 != 0.0)) || (locals.var_guard2110 != 0.0))))) {
        let assign90510_e139250: f64 = (locals.var_t4 * locals.var_qsuld);
        (assign90510_e139250, ((locals.var_t4_dn0 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn0)), ((locals.var_t4_dn2 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn2)), ((locals.var_t4_dn4 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn4)), ((locals.var_t4_dn5 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn5)), ((locals.var_t4_dn6 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn6)), ((locals.var_t4_dn7 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn7)), ((locals.var_t4_dn8 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn8)), ((locals.var_t4_dn9 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn9)), ((locals.var_t4_dn10 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn10)), ((locals.var_t4_dn11 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn11)), ((locals.var_t4_dn14 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn14)),)
    } else {
        (locals.var_qovdext, locals.var_qovdext_dn0, locals.var_qovdext_dn2, locals.var_qovdext_dn4, locals.var_qovdext_dn5, locals.var_qovdext_dn6, locals.var_qovdext_dn7, locals.var_qovdext_dn8, locals.var_qovdext_dn9, locals.var_qovdext_dn10, locals.var_qovdext_dn11, locals.var_qovdext_dn14,)
    }
};
        locals.var_qovdext = assign90510_e139252;
        locals.var_qovdext_dn0 = assign90510_e139252_d_n0;
        locals.var_qovdext_dn2 = assign90510_e139252_d_n2;
        locals.var_qovdext_dn4 = assign90510_e139252_d_n4;
        locals.var_qovdext_dn5 = assign90510_e139252_d_n5;
        locals.var_qovdext_dn6 = assign90510_e139252_d_n6;
        locals.var_qovdext_dn7 = assign90510_e139252_d_n7;
        locals.var_qovdext_dn8 = assign90510_e139252_d_n8;
        locals.var_qovdext_dn9 = assign90510_e139252_d_n9;
        locals.var_qovdext_dn10 = assign90510_e139252_d_n10;
        locals.var_qovdext_dn11 = assign90510_e139252_d_n11;
        locals.var_qovdext_dn14 = assign90510_e139252_d_n14;
        locals.var_qovdext_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_349(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign90520_e139267, assign90520_e139267_d_n0, assign90520_e139267_d_n2, assign90520_e139267_d_n4, assign90520_e139267_d_n5, assign90520_e139267_d_n6, assign90520_e139267_d_n7, assign90520_e139267_d_n8, assign90520_e139267_d_n9, assign90520_e139267_d_n10, assign90520_e139267_d_n11, assign90520_e139267_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard2111 != 0.0) && (!(((locals.var_guard2108 != 0.0) || (locals.var_guard2109 != 0.0)) || (locals.var_guard2110 != 0.0))))) {
        let assign90520_e139265: f64 = (locals.var_t4 * locals.var_qbuld);
        (assign90520_e139265, ((locals.var_t4_dn0 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn0)), ((locals.var_t4_dn2 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn2)), ((locals.var_t4_dn4 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn4)), ((locals.var_t4_dn5 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn5)), ((locals.var_t4_dn6 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn6)), ((locals.var_t4_dn7 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn7)), ((locals.var_t4_dn8 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn8)), ((locals.var_t4_dn9 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn9)), ((locals.var_t4_dn10 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn10)), ((locals.var_t4_dn11 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn11)), ((locals.var_t4_dn14 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn14)),)
    } else {
        (locals.var_qbdldext, locals.var_qbdldext_dn0, locals.var_qbdldext_dn2, locals.var_qbdldext_dn4, locals.var_qbdldext_dn5, locals.var_qbdldext_dn6, locals.var_qbdldext_dn7, locals.var_qbdldext_dn8, locals.var_qbdldext_dn9, locals.var_qbdldext_dn10, locals.var_qbdldext_dn11, locals.var_qbdldext_dn14,)
    }
};
        locals.var_qbdldext = assign90520_e139267;
        locals.var_qbdldext_dn0 = assign90520_e139267_d_n0;
        locals.var_qbdldext_dn2 = assign90520_e139267_d_n2;
        locals.var_qbdldext_dn4 = assign90520_e139267_d_n4;
        locals.var_qbdldext_dn5 = assign90520_e139267_d_n5;
        locals.var_qbdldext_dn6 = assign90520_e139267_d_n6;
        locals.var_qbdldext_dn7 = assign90520_e139267_d_n7;
        locals.var_qbdldext_dn8 = assign90520_e139267_d_n8;
        locals.var_qbdldext_dn9 = assign90520_e139267_d_n9;
        locals.var_qbdldext_dn10 = assign90520_e139267_d_n10;
        locals.var_qbdldext_dn11 = assign90520_e139267_d_n11;
        locals.var_qbdldext_dn14 = assign90520_e139267_d_n14;
        locals.var_qbdldext_rv = 0.0;

        let assign90530_e139270: f64 = if p.p430 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2115 = assign90530_e139270;
        locals.var_guard2115_rv = 0.0;

        let (assign90540_e139274,) = {
    if (locals.var_guard2115 != 0.0) {
        (1.0,)
    } else {
        (locals.var_flg_never_reach_vfbover,)
    }
};
        locals.var_flg_never_reach_vfbover = assign90540_e139274;
        locals.var_flg_never_reach_vfbover_rv = 0.0;

        let assign90550_e139285: f64 = if (((p.p35 == 1.0) && (p.p63 > 0.0)) && (locals.var_uc_nover > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2116 = assign90550_e139285;
        locals.var_guard2116_rv = 0.0;

        let (assign90560_e139293, assign90560_e139293_d_n2, assign90560_e139293_d_n7, assign90560_e139293_d_n8, assign90560_e139293_d_n9,) = {
    if ((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) {
        let assign90560_e139291: f64 = (locals.var_vgsi - locals.var_vbsi);
        (assign90560_e139291, 0.0, locals.var_vgsi_dn7, (locals.var_vgsi_dn8 - locals.var_vbsi_dn8), (-locals.var_vbsi_dn9),)
    } else {
        (locals.var_vgbgmt, locals.var_vgbgmt_dn2, locals.var_vgbgmt_dn7, locals.var_vgbgmt_dn8, locals.var_vgbgmt_dn9,)
    }
};
        locals.var_vgbgmt = assign90560_e139293;
        locals.var_vgbgmt_dn2 = assign90560_e139293_d_n2;
        locals.var_vgbgmt_dn7 = assign90560_e139293_d_n7;
        locals.var_vgbgmt_dn8 = assign90560_e139293_d_n8;
        locals.var_vgbgmt_dn9 = assign90560_e139293_d_n9;
        locals.var_vgbgmt_rv = 0.0;

        let (assign90570_e139301, assign90570_e139301_d_n0, assign90570_e139301_d_n2, assign90570_e139301_d_n4, assign90570_e139301_d_n5, assign90570_e139301_d_n6, assign90570_e139301_d_n7, assign90570_e139301_d_n8, assign90570_e139301_d_n9, assign90570_e139301_d_n10, assign90570_e139301_d_n11, assign90570_e139301_d_n14,) = {
    if ((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) {
        let assign90570_e139299: f64 = (locals.var_vdsi - locals.var_vbsi);
        (assign90570_e139299, 0.0, 0.0, 0.0, 0.0, locals.var_vdsi_dn6, 0.0, (locals.var_vdsi_dn8 - locals.var_vbsi_dn8), (-locals.var_vbsi_dn9), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn11, locals.var_vxbgmt_dn14,)
    }
};
        locals.var_vxbgmt = assign90570_e139301;
        locals.var_vxbgmt_dn0 = assign90570_e139301_d_n0;
        locals.var_vxbgmt_dn2 = assign90570_e139301_d_n2;
        locals.var_vxbgmt_dn4 = assign90570_e139301_d_n4;
        locals.var_vxbgmt_dn5 = assign90570_e139301_d_n5;
        locals.var_vxbgmt_dn6 = assign90570_e139301_d_n6;
        locals.var_vxbgmt_dn7 = assign90570_e139301_d_n7;
        locals.var_vxbgmt_dn8 = assign90570_e139301_d_n8;
        locals.var_vxbgmt_dn9 = assign90570_e139301_d_n9;
        locals.var_vxbgmt_dn10 = assign90570_e139301_d_n10;
        locals.var_vxbgmt_dn11 = assign90570_e139301_d_n11;
        locals.var_vxbgmt_dn14 = assign90570_e139301_d_n14;
        locals.var_vxbgmt_rv = 0.0;

        let (assign90580_e139307,) = {
    if ((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) {
        (locals.var_uc_nover,)
    } else {
        (locals.var_nover_func,)
    }
};
        locals.var_nover_func = assign90580_e139307;
        locals.var_nover_func_rv = 0.0;

        let (assign90590_e139313, assign90590_e139313_d_n0, assign90590_e139313_d_n2, assign90590_e139313_d_n4, assign90590_e139313_d_n5, assign90590_e139313_d_n6, assign90590_e139313_d_n7, assign90590_e139313_d_n8, assign90590_e139313_d_n9, assign90590_e139313_d_n10, assign90590_e139313_d_n11, assign90590_e139313_d_n14,) = {
    if ((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_lover_func, locals.var_lover_func_dn0, locals.var_lover_func_dn2, locals.var_lover_func_dn4, locals.var_lover_func_dn5, locals.var_lover_func_dn6, locals.var_lover_func_dn7, locals.var_lover_func_dn8, locals.var_lover_func_dn9, locals.var_lover_func_dn10, locals.var_lover_func_dn11, locals.var_lover_func_dn14,)
    }
};
        locals.var_lover_func = assign90590_e139313;
        locals.var_lover_func_dn0 = assign90590_e139313_d_n0;
        locals.var_lover_func_dn2 = assign90590_e139313_d_n2;
        locals.var_lover_func_dn4 = assign90590_e139313_d_n4;
        locals.var_lover_func_dn5 = assign90590_e139313_d_n5;
        locals.var_lover_func_dn6 = assign90590_e139313_d_n6;
        locals.var_lover_func_dn7 = assign90590_e139313_d_n7;
        locals.var_lover_func_dn8 = assign90590_e139313_d_n8;
        locals.var_lover_func_dn9 = assign90590_e139313_d_n9;
        locals.var_lover_func_dn10 = assign90590_e139313_d_n10;
        locals.var_lover_func_dn11 = assign90590_e139313_d_n11;
        locals.var_lover_func_dn14 = assign90590_e139313_d_n14;
        locals.var_lover_func_rv = 0.0;

        let (assign90600_e139319, assign90600_e139319_d_n0, assign90600_e139319_d_n2, assign90600_e139319_d_n4, assign90600_e139319_d_n5, assign90600_e139319_d_n6, assign90600_e139319_d_n7, assign90600_e139319_d_n8, assign90600_e139319_d_n9, assign90600_e139319_d_n10, assign90600_e139319_d_n11, assign90600_e139319_d_n14,) = {
    if ((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) {
        (locals.var_wdep, locals.var_wdep_dn0, locals.var_wdep_dn2, locals.var_wdep_dn4, locals.var_wdep_dn5, locals.var_wdep_dn6, locals.var_wdep_dn7, locals.var_wdep_dn8, locals.var_wdep_dn9, locals.var_wdep_dn10, locals.var_wdep_dn11, locals.var_wdep_dn14,)
    } else {
        (locals.var_wdep_func, locals.var_wdep_func_dn0, locals.var_wdep_func_dn2, locals.var_wdep_func_dn4, locals.var_wdep_func_dn5, locals.var_wdep_func_dn6, locals.var_wdep_func_dn7, locals.var_wdep_func_dn8, locals.var_wdep_func_dn9, locals.var_wdep_func_dn10, locals.var_wdep_func_dn11, locals.var_wdep_func_dn14,)
    }
};
        locals.var_wdep_func = assign90600_e139319;
        locals.var_wdep_func_dn0 = assign90600_e139319_d_n0;
        locals.var_wdep_func_dn2 = assign90600_e139319_d_n2;
        locals.var_wdep_func_dn4 = assign90600_e139319_d_n4;
        locals.var_wdep_func_dn5 = assign90600_e139319_d_n5;
        locals.var_wdep_func_dn6 = assign90600_e139319_d_n6;
        locals.var_wdep_func_dn7 = assign90600_e139319_d_n7;
        locals.var_wdep_func_dn8 = assign90600_e139319_d_n8;
        locals.var_wdep_func_dn9 = assign90600_e139319_d_n9;
        locals.var_wdep_func_dn10 = assign90600_e139319_d_n10;
        locals.var_wdep_func_dn11 = assign90600_e139319_d_n11;
        locals.var_wdep_func_dn14 = assign90600_e139319_d_n14;
        locals.var_wdep_func_rv = 0.0;

        let (assign90610_e139325, assign90610_e139325_d_n0, assign90610_e139325_d_n2, assign90610_e139325_d_n4, assign90610_e139325_d_n5, assign90610_e139325_d_n6, assign90610_e139325_d_n7, assign90610_e139325_d_n8, assign90610_e139325_d_n9, assign90610_e139325_d_n10, assign90610_e139325_d_n11, assign90610_e139325_d_n14,) = {
    if ((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) {
        (locals.var_cnst0over, locals.var_cnst0over_dn0, locals.var_cnst0over_dn2, locals.var_cnst0over_dn4, locals.var_cnst0over_dn5, locals.var_cnst0over_dn6, locals.var_cnst0over_dn7, locals.var_cnst0over_dn8, locals.var_cnst0over_dn9, locals.var_cnst0over_dn10, locals.var_cnst0over_dn11, locals.var_cnst0over_dn14,)
    } else {
        (locals.var_cnst0over_func, locals.var_cnst0over_func_dn0, locals.var_cnst0over_func_dn2, locals.var_cnst0over_func_dn4, locals.var_cnst0over_func_dn5, locals.var_cnst0over_func_dn6, locals.var_cnst0over_func_dn7, locals.var_cnst0over_func_dn8, locals.var_cnst0over_func_dn9, locals.var_cnst0over_func_dn10, locals.var_cnst0over_func_dn11, locals.var_cnst0over_func_dn14,)
    }
};
        locals.var_cnst0over_func = assign90610_e139325;
        locals.var_cnst0over_func_dn0 = assign90610_e139325_d_n0;
        locals.var_cnst0over_func_dn2 = assign90610_e139325_d_n2;
        locals.var_cnst0over_func_dn4 = assign90610_e139325_d_n4;
        locals.var_cnst0over_func_dn5 = assign90610_e139325_d_n5;
        locals.var_cnst0over_func_dn6 = assign90610_e139325_d_n6;
        locals.var_cnst0over_func_dn7 = assign90610_e139325_d_n7;
        locals.var_cnst0over_func_dn8 = assign90610_e139325_d_n8;
        locals.var_cnst0over_func_dn9 = assign90610_e139325_d_n9;
        locals.var_cnst0over_func_dn10 = assign90610_e139325_d_n10;
        locals.var_cnst0over_func_dn11 = assign90610_e139325_d_n11;
        locals.var_cnst0over_func_dn14 = assign90610_e139325_d_n14;
        locals.var_cnst0over_func_rv = 0.0;

        let (assign90620_e139331,) = {
    if ((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) {
        (locals.var_coxb0,)
    } else {
        (locals.var_cox0_func,)
    }
};
        locals.var_cox0_func = assign90620_e139331;
        locals.var_cox0_func_rv = 0.0;

        let (assign90630_e139337, assign90630_e139337_d_n0, assign90630_e139337_d_n2, assign90630_e139337_d_n4, assign90630_e139337_d_n5, assign90630_e139337_d_n6, assign90630_e139337_d_n7, assign90630_e139337_d_n8, assign90630_e139337_d_n9, assign90630_e139337_d_n10, assign90630_e139337_d_n11, assign90630_e139337_d_n14,) = {
    if ((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) {
        (0.4, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbs_bnd_over__blk2124, locals.var_vbs_bnd_over__blk2124_dn0, locals.var_vbs_bnd_over__blk2124_dn2, locals.var_vbs_bnd_over__blk2124_dn4, locals.var_vbs_bnd_over__blk2124_dn5, locals.var_vbs_bnd_over__blk2124_dn6, locals.var_vbs_bnd_over__blk2124_dn7, locals.var_vbs_bnd_over__blk2124_dn8, locals.var_vbs_bnd_over__blk2124_dn9, locals.var_vbs_bnd_over__blk2124_dn10, locals.var_vbs_bnd_over__blk2124_dn11, locals.var_vbs_bnd_over__blk2124_dn14,)
    }
};
        locals.var_vbs_bnd_over__blk2124 = assign90630_e139337;
        locals.var_vbs_bnd_over__blk2124_dn0 = assign90630_e139337_d_n0;
        locals.var_vbs_bnd_over__blk2124_dn2 = assign90630_e139337_d_n2;
        locals.var_vbs_bnd_over__blk2124_dn4 = assign90630_e139337_d_n4;
        locals.var_vbs_bnd_over__blk2124_dn5 = assign90630_e139337_d_n5;
        locals.var_vbs_bnd_over__blk2124_dn6 = assign90630_e139337_d_n6;
        locals.var_vbs_bnd_over__blk2124_dn7 = assign90630_e139337_d_n7;
        locals.var_vbs_bnd_over__blk2124_dn8 = assign90630_e139337_d_n8;
        locals.var_vbs_bnd_over__blk2124_dn9 = assign90630_e139337_d_n9;
        locals.var_vbs_bnd_over__blk2124_dn10 = assign90630_e139337_d_n10;
        locals.var_vbs_bnd_over__blk2124_dn11 = assign90630_e139337_d_n11;
        locals.var_vbs_bnd_over__blk2124_dn14 = assign90630_e139337_d_n14;
        locals.var_vbs_bnd_over__blk2124_rv = 0.0;

        let (assign90650_e139349,) = {
    if ((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_fd_mode__blk2125,)
    }
};
        locals.var_flg_fd_mode__blk2125 = assign90650_e139349;
        locals.var_flg_fd_mode__blk2125_rv = 0.0;

        let (assign90660_e139355, assign90660_e139355_d_n0, assign90660_e139355_d_n2, assign90660_e139355_d_n4, assign90660_e139355_d_n5, assign90660_e139355_d_n6, assign90660_e139355_d_n7, assign90660_e139355_d_n8, assign90660_e139355_d_n9, assign90660_e139355_d_n10, assign90660_e139355_d_n11, assign90660_e139355_d_n14,) = {
    if ((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn14,)
    }
};
        locals.var_fb = assign90660_e139355;
        locals.var_fb_dn0 = assign90660_e139355_d_n0;
        locals.var_fb_dn2 = assign90660_e139355_d_n2;
        locals.var_fb_dn4 = assign90660_e139355_d_n4;
        locals.var_fb_dn5 = assign90660_e139355_d_n5;
        locals.var_fb_dn6 = assign90660_e139355_d_n6;
        locals.var_fb_dn7 = assign90660_e139355_d_n7;
        locals.var_fb_dn8 = assign90660_e139355_d_n8;
        locals.var_fb_dn9 = assign90660_e139355_d_n9;
        locals.var_fb_dn10 = assign90660_e139355_d_n10;
        locals.var_fb_dn11 = assign90660_e139355_d_n11;
        locals.var_fb_dn14 = assign90660_e139355_d_n14;
        locals.var_fb_rv = 0.0;

        let (assign90670_e139361, assign90670_e139361_d_n0, assign90670_e139361_d_n2, assign90670_e139361_d_n4, assign90670_e139361_d_n5, assign90670_e139361_d_n6, assign90670_e139361_d_n7, assign90670_e139361_d_n8, assign90670_e139361_d_n9, assign90670_e139361_d_n10, assign90670_e139361_d_n11, assign90670_e139361_d_n14,) = {
    if ((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn11, locals.var_fs01_dn14,)
    }
};
        locals.var_fs01 = assign90670_e139361;
        locals.var_fs01_dn0 = assign90670_e139361_d_n0;
        locals.var_fs01_dn2 = assign90670_e139361_d_n2;
        locals.var_fs01_dn4 = assign90670_e139361_d_n4;
        locals.var_fs01_dn5 = assign90670_e139361_d_n5;
        locals.var_fs01_dn6 = assign90670_e139361_d_n6;
        locals.var_fs01_dn7 = assign90670_e139361_d_n7;
        locals.var_fs01_dn8 = assign90670_e139361_d_n8;
        locals.var_fs01_dn9 = assign90670_e139361_d_n9;
        locals.var_fs01_dn10 = assign90670_e139361_d_n10;
        locals.var_fs01_dn11 = assign90670_e139361_d_n11;
        locals.var_fs01_dn14 = assign90670_e139361_d_n14;
        locals.var_fs01_rv = 0.0;

        let (assign90680_e139367, assign90680_e139367_d_n0, assign90680_e139367_d_n2, assign90680_e139367_d_n4, assign90680_e139367_d_n5, assign90680_e139367_d_n6, assign90680_e139367_d_n7, assign90680_e139367_d_n8, assign90680_e139367_d_n9, assign90680_e139367_d_n10, assign90680_e139367_d_n11, assign90680_e139367_d_n14,) = {
    if ((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn11, locals.var_fs02_dn14,)
    }
};
        locals.var_fs02 = assign90680_e139367;
        locals.var_fs02_dn0 = assign90680_e139367_d_n0;
        locals.var_fs02_dn2 = assign90680_e139367_d_n2;
        locals.var_fs02_dn4 = assign90680_e139367_d_n4;
        locals.var_fs02_dn5 = assign90680_e139367_d_n5;
        locals.var_fs02_dn6 = assign90680_e139367_d_n6;
        locals.var_fs02_dn7 = assign90680_e139367_d_n7;
        locals.var_fs02_dn8 = assign90680_e139367_d_n8;
        locals.var_fs02_dn9 = assign90680_e139367_d_n9;
        locals.var_fs02_dn10 = assign90680_e139367_d_n10;
        locals.var_fs02_dn11 = assign90680_e139367_d_n11;
        locals.var_fs02_dn14 = assign90680_e139367_d_n14;
        locals.var_fs02_rv = 0.0;

        let (assign90690_e139373, assign90690_e139373_d_n0, assign90690_e139373_d_n2, assign90690_e139373_d_n4, assign90690_e139373_d_n5, assign90690_e139373_d_n6, assign90690_e139373_d_n7, assign90690_e139373_d_n8, assign90690_e139373_d_n9, assign90690_e139373_d_n10, assign90690_e139373_d_n11, assign90690_e139373_d_n14,) = {
    if ((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs0, locals.var_fs0_dn0, locals.var_fs0_dn2, locals.var_fs0_dn4, locals.var_fs0_dn5, locals.var_fs0_dn6, locals.var_fs0_dn7, locals.var_fs0_dn8, locals.var_fs0_dn9, locals.var_fs0_dn10, locals.var_fs0_dn11, locals.var_fs0_dn14,)
    }
};
        locals.var_fs0 = assign90690_e139373;
        locals.var_fs0_dn0 = assign90690_e139373_d_n0;
        locals.var_fs0_dn2 = assign90690_e139373_d_n2;
        locals.var_fs0_dn4 = assign90690_e139373_d_n4;
        locals.var_fs0_dn5 = assign90690_e139373_d_n5;
        locals.var_fs0_dn6 = assign90690_e139373_d_n6;
        locals.var_fs0_dn7 = assign90690_e139373_d_n7;
        locals.var_fs0_dn8 = assign90690_e139373_d_n8;
        locals.var_fs0_dn9 = assign90690_e139373_d_n9;
        locals.var_fs0_dn10 = assign90690_e139373_d_n10;
        locals.var_fs0_dn11 = assign90690_e139373_d_n11;
        locals.var_fs0_dn14 = assign90690_e139373_d_n14;
        locals.var_fs0_rv = 0.0;

        let (assign90700_e139379, assign90700_e139379_d_n0, assign90700_e139379_d_n2, assign90700_e139379_d_n4, assign90700_e139379_d_n5, assign90700_e139379_d_n6, assign90700_e139379_d_n7, assign90700_e139379_d_n8, assign90700_e139379_d_n9, assign90700_e139379_d_n10, assign90700_e139379_d_n11, assign90700_e139379_d_n14,) = {
    if ((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn4, locals.var_dps0_dn5, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn8, locals.var_dps0_dn9, locals.var_dps0_dn10, locals.var_dps0_dn11, locals.var_dps0_dn14,)
    }
};
        locals.var_dps0 = assign90700_e139379;
        locals.var_dps0_dn0 = assign90700_e139379_d_n0;
        locals.var_dps0_dn2 = assign90700_e139379_d_n2;
        locals.var_dps0_dn4 = assign90700_e139379_d_n4;
        locals.var_dps0_dn5 = assign90700_e139379_d_n5;
        locals.var_dps0_dn6 = assign90700_e139379_d_n6;
        locals.var_dps0_dn7 = assign90700_e139379_d_n7;
        locals.var_dps0_dn8 = assign90700_e139379_d_n8;
        locals.var_dps0_dn9 = assign90700_e139379_d_n9;
        locals.var_dps0_dn10 = assign90700_e139379_d_n10;
        locals.var_dps0_dn11 = assign90700_e139379_d_n11;
        locals.var_dps0_dn14 = assign90700_e139379_d_n14;
        locals.var_dps0_rv = 0.0;

        let (assign90710_e139385, assign90710_e139385_d_n0, assign90710_e139385_d_n2, assign90710_e139385_d_n4, assign90710_e139385_d_n5, assign90710_e139385_d_n6, assign90710_e139385_d_n7, assign90710_e139385_d_n8, assign90710_e139385_d_n9, assign90710_e139385_d_n10, assign90710_e139385_d_n11, assign90710_e139385_d_n14,) = {
    if ((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs0_dps0, locals.var_fs0_dps0_dn0, locals.var_fs0_dps0_dn2, locals.var_fs0_dps0_dn4, locals.var_fs0_dps0_dn5, locals.var_fs0_dps0_dn6, locals.var_fs0_dps0_dn7, locals.var_fs0_dps0_dn8, locals.var_fs0_dps0_dn9, locals.var_fs0_dps0_dn10, locals.var_fs0_dps0_dn11, locals.var_fs0_dps0_dn14,)
    }
};
        locals.var_fs0_dps0 = assign90710_e139385;
        locals.var_fs0_dps0_dn0 = assign90710_e139385_d_n0;
        locals.var_fs0_dps0_dn2 = assign90710_e139385_d_n2;
        locals.var_fs0_dps0_dn4 = assign90710_e139385_d_n4;
        locals.var_fs0_dps0_dn5 = assign90710_e139385_d_n5;
        locals.var_fs0_dps0_dn6 = assign90710_e139385_d_n6;
        locals.var_fs0_dps0_dn7 = assign90710_e139385_d_n7;
        locals.var_fs0_dps0_dn8 = assign90710_e139385_d_n8;
        locals.var_fs0_dps0_dn9 = assign90710_e139385_d_n9;
        locals.var_fs0_dps0_dn10 = assign90710_e139385_d_n10;
        locals.var_fs0_dps0_dn11 = assign90710_e139385_d_n11;
        locals.var_fs0_dps0_dn14 = assign90710_e139385_d_n14;
        locals.var_fs0_dps0_rv = 0.0;

        let (assign90720_e139391, assign90720_e139391_d_n0, assign90720_e139391_d_n2, assign90720_e139391_d_n4, assign90720_e139391_d_n5, assign90720_e139391_d_n6, assign90720_e139391_d_n7, assign90720_e139391_d_n8, assign90720_e139391_d_n9, assign90720_e139391_d_n10, assign90720_e139391_d_n11, assign90720_e139391_d_n14,) = {
    if ((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn11, locals.var_fs02_dps0_dn14,)
    }
};
        locals.var_fs02_dps0 = assign90720_e139391;
        locals.var_fs02_dps0_dn0 = assign90720_e139391_d_n0;
        locals.var_fs02_dps0_dn2 = assign90720_e139391_d_n2;
        locals.var_fs02_dps0_dn4 = assign90720_e139391_d_n4;
        locals.var_fs02_dps0_dn5 = assign90720_e139391_d_n5;
        locals.var_fs02_dps0_dn6 = assign90720_e139391_d_n6;
        locals.var_fs02_dps0_dn7 = assign90720_e139391_d_n7;
        locals.var_fs02_dps0_dn8 = assign90720_e139391_d_n8;
        locals.var_fs02_dps0_dn9 = assign90720_e139391_d_n9;
        locals.var_fs02_dps0_dn10 = assign90720_e139391_d_n10;
        locals.var_fs02_dps0_dn11 = assign90720_e139391_d_n11;
        locals.var_fs02_dps0_dn14 = assign90720_e139391_d_n14;
        locals.var_fs02_dps0_rv = 0.0;

        let (assign90730_e139397, assign90730_e139397_d_n0, assign90730_e139397_d_n2, assign90730_e139397_d_n4, assign90730_e139397_d_n5, assign90730_e139397_d_n6, assign90730_e139397_d_n7, assign90730_e139397_d_n8, assign90730_e139397_d_n9, assign90730_e139397_d_n10, assign90730_e139397_d_n11, assign90730_e139397_d_n14,) = {
    if ((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn14,)
    }
};
        locals.var_fb_dpss = assign90730_e139397;
        locals.var_fb_dpss_dn0 = assign90730_e139397_d_n0;
        locals.var_fb_dpss_dn2 = assign90730_e139397_d_n2;
        locals.var_fb_dpss_dn4 = assign90730_e139397_d_n4;
        locals.var_fb_dpss_dn5 = assign90730_e139397_d_n5;
        locals.var_fb_dpss_dn6 = assign90730_e139397_d_n6;
        locals.var_fb_dpss_dn7 = assign90730_e139397_d_n7;
        locals.var_fb_dpss_dn8 = assign90730_e139397_d_n8;
        locals.var_fb_dpss_dn9 = assign90730_e139397_d_n9;
        locals.var_fb_dpss_dn10 = assign90730_e139397_d_n10;
        locals.var_fb_dpss_dn11 = assign90730_e139397_d_n11;
        locals.var_fb_dpss_dn14 = assign90730_e139397_d_n14;
        locals.var_fb_dpss_rv = 0.0;

        let (assign90740_e139403, assign90740_e139403_d_n0, assign90740_e139403_d_n2, assign90740_e139403_d_n4, assign90740_e139403_d_n5, assign90740_e139403_d_n6, assign90740_e139403_d_n7, assign90740_e139403_d_n8, assign90740_e139403_d_n9, assign90740_e139403_d_n10, assign90740_e139403_d_n11, assign90740_e139403_d_n14,) = {
    if ((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn11, locals.var_fs01_dps0_dn14,)
    }
};
        locals.var_fs01_dps0 = assign90740_e139403;
        locals.var_fs01_dps0_dn0 = assign90740_e139403_d_n0;
        locals.var_fs01_dps0_dn2 = assign90740_e139403_d_n2;
        locals.var_fs01_dps0_dn4 = assign90740_e139403_d_n4;
        locals.var_fs01_dps0_dn5 = assign90740_e139403_d_n5;
        locals.var_fs01_dps0_dn6 = assign90740_e139403_d_n6;
        locals.var_fs01_dps0_dn7 = assign90740_e139403_d_n7;
        locals.var_fs01_dps0_dn8 = assign90740_e139403_d_n8;
        locals.var_fs01_dps0_dn9 = assign90740_e139403_d_n9;
        locals.var_fs01_dps0_dn10 = assign90740_e139403_d_n10;
        locals.var_fs01_dps0_dn11 = assign90740_e139403_d_n11;
        locals.var_fs01_dps0_dn14 = assign90740_e139403_d_n14;
        locals.var_fs01_dps0_rv = 0.0;

        let (assign90750_e139409, assign90750_e139409_d_n0, assign90750_e139409_d_n2, assign90750_e139409_d_n4, assign90750_e139409_d_n5, assign90750_e139409_d_n6, assign90750_e139409_d_n7, assign90750_e139409_d_n8, assign90750_e139409_d_n9, assign90750_e139409_d_n10, assign90750_e139409_d_n11, assign90750_e139409_d_n14,) = {
    if ((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn11, locals.var_chi_1_dn14,)
    }
};
        locals.var_chi_1 = assign90750_e139409;
        locals.var_chi_1_dn0 = assign90750_e139409_d_n0;
        locals.var_chi_1_dn2 = assign90750_e139409_d_n2;
        locals.var_chi_1_dn4 = assign90750_e139409_d_n4;
        locals.var_chi_1_dn5 = assign90750_e139409_d_n5;
        locals.var_chi_1_dn6 = assign90750_e139409_d_n6;
        locals.var_chi_1_dn7 = assign90750_e139409_d_n7;
        locals.var_chi_1_dn8 = assign90750_e139409_d_n8;
        locals.var_chi_1_dn9 = assign90750_e139409_d_n9;
        locals.var_chi_1_dn10 = assign90750_e139409_d_n10;
        locals.var_chi_1_dn11 = assign90750_e139409_d_n11;
        locals.var_chi_1_dn14 = assign90750_e139409_d_n14;
        locals.var_chi_1_rv = 0.0;

        let (assign90760_e139415, assign90760_e139415_d_n0, assign90760_e139415_d_n2, assign90760_e139415_d_n4, assign90760_e139415_d_n5, assign90760_e139415_d_n6, assign90760_e139415_d_n7, assign90760_e139415_d_n8, assign90760_e139415_d_n9, assign90760_e139415_d_n10, assign90760_e139415_d_n11, assign90760_e139415_d_n14,) = {
    if ((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_chi_a, locals.var_chi_a_dn0, locals.var_chi_a_dn2, locals.var_chi_a_dn4, locals.var_chi_a_dn5, locals.var_chi_a_dn6, locals.var_chi_a_dn7, locals.var_chi_a_dn8, locals.var_chi_a_dn9, locals.var_chi_a_dn10, locals.var_chi_a_dn11, locals.var_chi_a_dn14,)
    }
};
        locals.var_chi_a = assign90760_e139415;
        locals.var_chi_a_dn0 = assign90760_e139415_d_n0;
        locals.var_chi_a_dn2 = assign90760_e139415_d_n2;
        locals.var_chi_a_dn4 = assign90760_e139415_d_n4;
        locals.var_chi_a_dn5 = assign90760_e139415_d_n5;
        locals.var_chi_a_dn6 = assign90760_e139415_d_n6;
        locals.var_chi_a_dn7 = assign90760_e139415_d_n7;
        locals.var_chi_a_dn8 = assign90760_e139415_d_n8;
        locals.var_chi_a_dn9 = assign90760_e139415_d_n9;
        locals.var_chi_a_dn10 = assign90760_e139415_d_n10;
        locals.var_chi_a_dn11 = assign90760_e139415_d_n11;
        locals.var_chi_a_dn14 = assign90760_e139415_d_n14;
        locals.var_chi_a_rv = 0.0;

        let (assign90770_e139421, assign90770_e139421_d_n0, assign90770_e139421_d_n2, assign90770_e139421_d_n4, assign90770_e139421_d_n5, assign90770_e139421_d_n6, assign90770_e139421_d_n7, assign90770_e139421_d_n8, assign90770_e139421_d_n9, assign90770_e139421_d_n10, assign90770_e139421_d_n11, assign90770_e139421_d_n14,) = {
    if ((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_chi_b, locals.var_chi_b_dn0, locals.var_chi_b_dn2, locals.var_chi_b_dn4, locals.var_chi_b_dn5, locals.var_chi_b_dn6, locals.var_chi_b_dn7, locals.var_chi_b_dn8, locals.var_chi_b_dn9, locals.var_chi_b_dn10, locals.var_chi_b_dn11, locals.var_chi_b_dn14,)
    }
};
        locals.var_chi_b = assign90770_e139421;
        locals.var_chi_b_dn0 = assign90770_e139421_d_n0;
        locals.var_chi_b_dn2 = assign90770_e139421_d_n2;
        locals.var_chi_b_dn4 = assign90770_e139421_d_n4;
        locals.var_chi_b_dn5 = assign90770_e139421_d_n5;
        locals.var_chi_b_dn6 = assign90770_e139421_d_n6;
        locals.var_chi_b_dn7 = assign90770_e139421_d_n7;
        locals.var_chi_b_dn8 = assign90770_e139421_d_n8;
        locals.var_chi_b_dn9 = assign90770_e139421_d_n9;
        locals.var_chi_b_dn10 = assign90770_e139421_d_n10;
        locals.var_chi_b_dn11 = assign90770_e139421_d_n11;
        locals.var_chi_b_dn14 = assign90770_e139421_d_n14;
        locals.var_chi_b_rv = 0.0;

        let (assign90780_e139428,) = {
    if ((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) {
        let assign90780_e139426: f64 = (-1.0);
        (assign90780_e139426,)
    } else {
        (locals.var_flg_conv,)
    }
};
        locals.var_flg_conv = assign90780_e139428;
        locals.var_flg_conv_rv = 0.0;

        let (assign90790_e139434, assign90790_e139434_d_n0, assign90790_e139434_d_n2, assign90790_e139434_d_n4, assign90790_e139434_d_n5, assign90790_e139434_d_n6, assign90790_e139434_d_n7, assign90790_e139434_d_n8, assign90790_e139434_d_n9, assign90790_e139434_d_n10, assign90790_e139434_d_n11, assign90790_e139434_d_n14,) = {
    if ((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ps0ld_ini__blk2126, locals.var_ps0ld_ini__blk2126_dn0, locals.var_ps0ld_ini__blk2126_dn2, locals.var_ps0ld_ini__blk2126_dn4, locals.var_ps0ld_ini__blk2126_dn5, locals.var_ps0ld_ini__blk2126_dn6, locals.var_ps0ld_ini__blk2126_dn7, locals.var_ps0ld_ini__blk2126_dn8, locals.var_ps0ld_ini__blk2126_dn9, locals.var_ps0ld_ini__blk2126_dn10, locals.var_ps0ld_ini__blk2126_dn11, locals.var_ps0ld_ini__blk2126_dn14,)
    }
};
        locals.var_ps0ld_ini__blk2126 = assign90790_e139434;
        locals.var_ps0ld_ini__blk2126_dn0 = assign90790_e139434_d_n0;
        locals.var_ps0ld_ini__blk2126_dn2 = assign90790_e139434_d_n2;
        locals.var_ps0ld_ini__blk2126_dn4 = assign90790_e139434_d_n4;
        locals.var_ps0ld_ini__blk2126_dn5 = assign90790_e139434_d_n5;
        locals.var_ps0ld_ini__blk2126_dn6 = assign90790_e139434_d_n6;
        locals.var_ps0ld_ini__blk2126_dn7 = assign90790_e139434_d_n7;
        locals.var_ps0ld_ini__blk2126_dn8 = assign90790_e139434_d_n8;
        locals.var_ps0ld_ini__blk2126_dn9 = assign90790_e139434_d_n9;
        locals.var_ps0ld_ini__blk2126_dn10 = assign90790_e139434_d_n10;
        locals.var_ps0ld_ini__blk2126_dn11 = assign90790_e139434_d_n11;
        locals.var_ps0ld_ini__blk2126_dn14 = assign90790_e139434_d_n14;
        locals.var_ps0ld_ini__blk2126_rv = 0.0;

        let (assign90800_e139440, assign90800_e139440_d_n0, assign90800_e139440_d_n2, assign90800_e139440_d_n4, assign90800_e139440_d_n5, assign90800_e139440_d_n6, assign90800_e139440_d_n7, assign90800_e139440_d_n8, assign90800_e139440_d_n9, assign90800_e139440_d_n10, assign90800_e139440_d_n11, assign90800_e139440_d_n14,) = {
    if ((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fbsq__blk2127, locals.var_fbsq__blk2127_dn0, locals.var_fbsq__blk2127_dn2, locals.var_fbsq__blk2127_dn4, locals.var_fbsq__blk2127_dn5, locals.var_fbsq__blk2127_dn6, locals.var_fbsq__blk2127_dn7, locals.var_fbsq__blk2127_dn8, locals.var_fbsq__blk2127_dn9, locals.var_fbsq__blk2127_dn10, locals.var_fbsq__blk2127_dn11, locals.var_fbsq__blk2127_dn14,)
    }
};
        locals.var_fbsq__blk2127 = assign90800_e139440;
        locals.var_fbsq__blk2127_dn0 = assign90800_e139440_d_n0;
        locals.var_fbsq__blk2127_dn2 = assign90800_e139440_d_n2;
        locals.var_fbsq__blk2127_dn4 = assign90800_e139440_d_n4;
        locals.var_fbsq__blk2127_dn5 = assign90800_e139440_d_n5;
        locals.var_fbsq__blk2127_dn6 = assign90800_e139440_d_n6;
        locals.var_fbsq__blk2127_dn7 = assign90800_e139440_d_n7;
        locals.var_fbsq__blk2127_dn8 = assign90800_e139440_d_n8;
        locals.var_fbsq__blk2127_dn9 = assign90800_e139440_d_n9;
        locals.var_fbsq__blk2127_dn10 = assign90800_e139440_d_n10;
        locals.var_fbsq__blk2127_dn11 = assign90800_e139440_d_n11;
        locals.var_fbsq__blk2127_dn14 = assign90800_e139440_d_n14;
        locals.var_fbsq__blk2127_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_350(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign90810_e139453, assign90810_e139453_d_n0, assign90810_e139453_d_n2, assign90810_e139453_d_n4, assign90810_e139453_d_n5, assign90810_e139453_d_n6, assign90810_e139453_d_n7, assign90810_e139453_d_n8, assign90810_e139453_d_n9, assign90810_e139453_d_n10, assign90810_e139453_d_n11, assign90810_e139453_d_n14,) = {
    if ((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) {
        let assign90810_e139446: f64 = (2.0 * locals.var_beta_inv);
        let assign90810_e139449: f64 = (locals.var_nover_func / locals.var_nin);
        let assign90810_e139450: f64 = (assign90810_e139449).ln();
        let assign90810_e139451: f64 = (assign90810_e139446 * assign90810_e139450);
        (assign90810_e139451, (((2.0 * locals.var_beta_inv_dn0) * assign90810_e139450) + (assign90810_e139446 * ((-((locals.var_nover_func * locals.var_nin_dn0) / (locals.var_nin * locals.var_nin))) / assign90810_e139449))), (((2.0 * locals.var_beta_inv_dn2) * assign90810_e139450) + (assign90810_e139446 * ((-((locals.var_nover_func * locals.var_nin_dn2) / (locals.var_nin * locals.var_nin))) / assign90810_e139449))), (((2.0 * locals.var_beta_inv_dn4) * assign90810_e139450) + (assign90810_e139446 * ((-((locals.var_nover_func * locals.var_nin_dn4) / (locals.var_nin * locals.var_nin))) / assign90810_e139449))), (((2.0 * locals.var_beta_inv_dn5) * assign90810_e139450) + (assign90810_e139446 * ((-((locals.var_nover_func * locals.var_nin_dn5) / (locals.var_nin * locals.var_nin))) / assign90810_e139449))), (((2.0 * locals.var_beta_inv_dn6) * assign90810_e139450) + (assign90810_e139446 * ((-((locals.var_nover_func * locals.var_nin_dn6) / (locals.var_nin * locals.var_nin))) / assign90810_e139449))), (((2.0 * locals.var_beta_inv_dn7) * assign90810_e139450) + (assign90810_e139446 * ((-((locals.var_nover_func * locals.var_nin_dn7) / (locals.var_nin * locals.var_nin))) / assign90810_e139449))), (((2.0 * locals.var_beta_inv_dn8) * assign90810_e139450) + (assign90810_e139446 * ((-((locals.var_nover_func * locals.var_nin_dn8) / (locals.var_nin * locals.var_nin))) / assign90810_e139449))), (((2.0 * locals.var_beta_inv_dn9) * assign90810_e139450) + (assign90810_e139446 * ((-((locals.var_nover_func * locals.var_nin_dn9) / (locals.var_nin * locals.var_nin))) / assign90810_e139449))), (((2.0 * locals.var_beta_inv_dn10) * assign90810_e139450) + (assign90810_e139446 * ((-((locals.var_nover_func * locals.var_nin_dn10) / (locals.var_nin * locals.var_nin))) / assign90810_e139449))), (((2.0 * locals.var_beta_inv_dn11) * assign90810_e139450) + (assign90810_e139446 * ((-((locals.var_nover_func * locals.var_nin_dn11) / (locals.var_nin * locals.var_nin))) / assign90810_e139449))), (((2.0 * locals.var_beta_inv_dn14) * assign90810_e139450) + (assign90810_e139446 * ((-((locals.var_nover_func * locals.var_nin_dn14) / (locals.var_nin * locals.var_nin))) / assign90810_e139449))),)
    } else {
        (locals.var_pb2over__blk2122, locals.var_pb2over__blk2122_dn0, locals.var_pb2over__blk2122_dn2, locals.var_pb2over__blk2122_dn4, locals.var_pb2over__blk2122_dn5, locals.var_pb2over__blk2122_dn6, locals.var_pb2over__blk2122_dn7, locals.var_pb2over__blk2122_dn8, locals.var_pb2over__blk2122_dn9, locals.var_pb2over__blk2122_dn10, locals.var_pb2over__blk2122_dn11, locals.var_pb2over__blk2122_dn14,)
    }
};
        locals.var_pb2over__blk2122 = assign90810_e139453;
        locals.var_pb2over__blk2122_dn0 = assign90810_e139453_d_n0;
        locals.var_pb2over__blk2122_dn2 = assign90810_e139453_d_n2;
        locals.var_pb2over__blk2122_dn4 = assign90810_e139453_d_n4;
        locals.var_pb2over__blk2122_dn5 = assign90810_e139453_d_n5;
        locals.var_pb2over__blk2122_dn6 = assign90810_e139453_d_n6;
        locals.var_pb2over__blk2122_dn7 = assign90810_e139453_d_n7;
        locals.var_pb2over__blk2122_dn8 = assign90810_e139453_d_n8;
        locals.var_pb2over__blk2122_dn9 = assign90810_e139453_d_n9;
        locals.var_pb2over__blk2122_dn10 = assign90810_e139453_d_n10;
        locals.var_pb2over__blk2122_dn11 = assign90810_e139453_d_n11;
        locals.var_pb2over__blk2122_dn14 = assign90810_e139453_d_n14;
        locals.var_pb2over__blk2122_rv = 0.0;

        let (assign90820_e139463, assign90820_e139463_d_n0, assign90820_e139463_d_n2, assign90820_e139463_d_n4, assign90820_e139463_d_n5, assign90820_e139463_d_n6, assign90820_e139463_d_n7, assign90820_e139463_d_n8, assign90820_e139463_d_n9, assign90820_e139463_d_n10, assign90820_e139463_d_n11, assign90820_e139463_d_n14,) = {
    if ((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) {
        let assign90820_e139459: f64 = (0.8 - locals.var_pb2over__blk2122);
        let assign90820_e139461: f64 = (assign90820_e139459 - 0.1);
        (assign90820_e139461, (-locals.var_pb2over__blk2122_dn0), (-locals.var_pb2over__blk2122_dn2), (-locals.var_pb2over__blk2122_dn4), (-locals.var_pb2over__blk2122_dn5), (-locals.var_pb2over__blk2122_dn6), (-locals.var_pb2over__blk2122_dn7), (-locals.var_pb2over__blk2122_dn8), (-locals.var_pb2over__blk2122_dn9), (-locals.var_pb2over__blk2122_dn10), (-locals.var_pb2over__blk2122_dn11), (-locals.var_pb2over__blk2122_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign90820_e139463;
        locals.var_tmf1_dn0 = assign90820_e139463_d_n0;
        locals.var_tmf1_dn2 = assign90820_e139463_d_n2;
        locals.var_tmf1_dn4 = assign90820_e139463_d_n4;
        locals.var_tmf1_dn5 = assign90820_e139463_d_n5;
        locals.var_tmf1_dn6 = assign90820_e139463_d_n6;
        locals.var_tmf1_dn7 = assign90820_e139463_d_n7;
        locals.var_tmf1_dn8 = assign90820_e139463_d_n8;
        locals.var_tmf1_dn9 = assign90820_e139463_d_n9;
        locals.var_tmf1_dn10 = assign90820_e139463_d_n10;
        locals.var_tmf1_dn11 = assign90820_e139463_d_n11;
        locals.var_tmf1_dn14 = assign90820_e139463_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign90830_e139473, assign90830_e139473_d_n0, assign90830_e139473_d_n2, assign90830_e139473_d_n4, assign90830_e139473_d_n5, assign90830_e139473_d_n6, assign90830_e139473_d_n7, assign90830_e139473_d_n8, assign90830_e139473_d_n9, assign90830_e139473_d_n10, assign90830_e139473_d_n11, assign90830_e139473_d_n14,) = {
    if ((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) {
        let assign90830_e139469: f64 = (4.0 * 0.8);
        let assign90830_e139471: f64 = (assign90830_e139469 * 0.1);
        (assign90830_e139471, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign90830_e139473;
        locals.var_tmf2_dn0 = assign90830_e139473_d_n0;
        locals.var_tmf2_dn2 = assign90830_e139473_d_n2;
        locals.var_tmf2_dn4 = assign90830_e139473_d_n4;
        locals.var_tmf2_dn5 = assign90830_e139473_d_n5;
        locals.var_tmf2_dn6 = assign90830_e139473_d_n6;
        locals.var_tmf2_dn7 = assign90830_e139473_d_n7;
        locals.var_tmf2_dn8 = assign90830_e139473_d_n8;
        locals.var_tmf2_dn9 = assign90830_e139473_d_n9;
        locals.var_tmf2_dn10 = assign90830_e139473_d_n10;
        locals.var_tmf2_dn11 = assign90830_e139473_d_n11;
        locals.var_tmf2_dn14 = assign90830_e139473_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign90840_e139485, assign90840_e139485_d_n0, assign90840_e139485_d_n2, assign90840_e139485_d_n4, assign90840_e139485_d_n5, assign90840_e139485_d_n6, assign90840_e139485_d_n7, assign90840_e139485_d_n8, assign90840_e139485_d_n9, assign90840_e139485_d_n10, assign90840_e139485_d_n11, assign90840_e139485_d_n14,) = {
    if ((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) {
        let (assign90840_e139483, assign90840_e139483_d_n0, assign90840_e139483_d_n2, assign90840_e139483_d_n4, assign90840_e139483_d_n5, assign90840_e139483_d_n6, assign90840_e139483_d_n7, assign90840_e139483_d_n8, assign90840_e139483_d_n9, assign90840_e139483_d_n10, assign90840_e139483_d_n11, assign90840_e139483_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign90840_e139482: f64 = (-locals.var_tmf2);
                (assign90840_e139482, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign90840_e139483, assign90840_e139483_d_n0, assign90840_e139483_d_n2, assign90840_e139483_d_n4, assign90840_e139483_d_n5, assign90840_e139483_d_n6, assign90840_e139483_d_n7, assign90840_e139483_d_n8, assign90840_e139483_d_n9, assign90840_e139483_d_n10, assign90840_e139483_d_n11, assign90840_e139483_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign90840_e139485;
        locals.var_tmf2_dn0 = assign90840_e139485_d_n0;
        locals.var_tmf2_dn2 = assign90840_e139485_d_n2;
        locals.var_tmf2_dn4 = assign90840_e139485_d_n4;
        locals.var_tmf2_dn5 = assign90840_e139485_d_n5;
        locals.var_tmf2_dn6 = assign90840_e139485_d_n6;
        locals.var_tmf2_dn7 = assign90840_e139485_d_n7;
        locals.var_tmf2_dn8 = assign90840_e139485_d_n8;
        locals.var_tmf2_dn9 = assign90840_e139485_d_n9;
        locals.var_tmf2_dn10 = assign90840_e139485_d_n10;
        locals.var_tmf2_dn11 = assign90840_e139485_d_n11;
        locals.var_tmf2_dn14 = assign90840_e139485_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign90850_e139496, assign90850_e139496_d_n0, assign90850_e139496_d_n2, assign90850_e139496_d_n4, assign90850_e139496_d_n5, assign90850_e139496_d_n6, assign90850_e139496_d_n7, assign90850_e139496_d_n8, assign90850_e139496_d_n9, assign90850_e139496_d_n10, assign90850_e139496_d_n11, assign90850_e139496_d_n14,) = {
    if ((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) {
        let assign90850_e139491: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign90850_e139493: f64 = (assign90850_e139491 + locals.var_tmf2);
        let assign90850_e139494: f64 = (assign90850_e139493).sqrt();
        (assign90850_e139494, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign90850_e139494)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign90850_e139494)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign90850_e139494)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign90850_e139494)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign90850_e139494)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign90850_e139494)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign90850_e139494)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign90850_e139494)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign90850_e139494)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign90850_e139494)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign90850_e139494)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign90850_e139496;
        locals.var_tmf2_dn0 = assign90850_e139496_d_n0;
        locals.var_tmf2_dn2 = assign90850_e139496_d_n2;
        locals.var_tmf2_dn4 = assign90850_e139496_d_n4;
        locals.var_tmf2_dn5 = assign90850_e139496_d_n5;
        locals.var_tmf2_dn6 = assign90850_e139496_d_n6;
        locals.var_tmf2_dn7 = assign90850_e139496_d_n7;
        locals.var_tmf2_dn8 = assign90850_e139496_d_n8;
        locals.var_tmf2_dn9 = assign90850_e139496_d_n9;
        locals.var_tmf2_dn10 = assign90850_e139496_d_n10;
        locals.var_tmf2_dn11 = assign90850_e139496_d_n11;
        locals.var_tmf2_dn14 = assign90850_e139496_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign90860_e139508, assign90860_e139508_d_n0, assign90860_e139508_d_n2, assign90860_e139508_d_n4, assign90860_e139508_d_n5, assign90860_e139508_d_n6, assign90860_e139508_d_n7, assign90860_e139508_d_n8, assign90860_e139508_d_n9, assign90860_e139508_d_n10, assign90860_e139508_d_n11, assign90860_e139508_d_n14,) = {
    if ((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) {
        let assign90860_e139504: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign90860_e139505: f64 = (1.0 + assign90860_e139504);
        let assign90860_e139506: f64 = (0.5 * assign90860_e139505);
        (assign90860_e139506, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign90860_e139508;
        locals.var_t0_dn0 = assign90860_e139508_d_n0;
        locals.var_t0_dn2 = assign90860_e139508_d_n2;
        locals.var_t0_dn4 = assign90860_e139508_d_n4;
        locals.var_t0_dn5 = assign90860_e139508_d_n5;
        locals.var_t0_dn6 = assign90860_e139508_d_n6;
        locals.var_t0_dn7 = assign90860_e139508_d_n7;
        locals.var_t0_dn8 = assign90860_e139508_d_n8;
        locals.var_t0_dn9 = assign90860_e139508_d_n9;
        locals.var_t0_dn10 = assign90860_e139508_d_n10;
        locals.var_t0_dn11 = assign90860_e139508_d_n11;
        locals.var_t0_dn14 = assign90860_e139508_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign90870_e139520, assign90870_e139520_d_n0, assign90870_e139520_d_n2, assign90870_e139520_d_n4, assign90870_e139520_d_n5, assign90870_e139520_d_n6, assign90870_e139520_d_n7, assign90870_e139520_d_n8, assign90870_e139520_d_n9, assign90870_e139520_d_n10, assign90870_e139520_d_n11, assign90870_e139520_d_n14,) = {
    if ((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) {
        let assign90870_e139516: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign90870_e139517: f64 = (0.5 * assign90870_e139516);
        let assign90870_e139518: f64 = (0.8 - assign90870_e139517);
        (assign90870_e139518, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (-(0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (-(0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (-(0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (-(0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (-(0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (-(0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (-(0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (-(0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (-(0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_vbs_max_over__blk2123, locals.var_vbs_max_over__blk2123_dn0, locals.var_vbs_max_over__blk2123_dn2, locals.var_vbs_max_over__blk2123_dn4, locals.var_vbs_max_over__blk2123_dn5, locals.var_vbs_max_over__blk2123_dn6, locals.var_vbs_max_over__blk2123_dn7, locals.var_vbs_max_over__blk2123_dn8, locals.var_vbs_max_over__blk2123_dn9, locals.var_vbs_max_over__blk2123_dn10, locals.var_vbs_max_over__blk2123_dn11, locals.var_vbs_max_over__blk2123_dn14,)
    }
};
        locals.var_vbs_max_over__blk2123 = assign90870_e139520;
        locals.var_vbs_max_over__blk2123_dn0 = assign90870_e139520_d_n0;
        locals.var_vbs_max_over__blk2123_dn2 = assign90870_e139520_d_n2;
        locals.var_vbs_max_over__blk2123_dn4 = assign90870_e139520_d_n4;
        locals.var_vbs_max_over__blk2123_dn5 = assign90870_e139520_d_n5;
        locals.var_vbs_max_over__blk2123_dn6 = assign90870_e139520_d_n6;
        locals.var_vbs_max_over__blk2123_dn7 = assign90870_e139520_d_n7;
        locals.var_vbs_max_over__blk2123_dn8 = assign90870_e139520_d_n8;
        locals.var_vbs_max_over__blk2123_dn9 = assign90870_e139520_d_n9;
        locals.var_vbs_max_over__blk2123_dn10 = assign90870_e139520_d_n10;
        locals.var_vbs_max_over__blk2123_dn11 = assign90870_e139520_d_n11;
        locals.var_vbs_max_over__blk2123_dn14 = assign90870_e139520_d_n14;
        locals.var_vbs_max_over__blk2123_rv = 0.0;

        let assign90880_e139524: f64 = (locals.var_vbs_max_over__blk2123 * 0.5);
        let assign90880_e139525: f64 = if locals.var_vbs_bnd_over__blk2124 > assign90880_e139524 { 1.0 } else { 0.0 };
        locals.var_guard2129 = assign90880_e139525;
        locals.var_guard2129_rv = 0.0;

        let (assign90890_e139535, assign90890_e139535_d_n0, assign90890_e139535_d_n2, assign90890_e139535_d_n4, assign90890_e139535_d_n5, assign90890_e139535_d_n6, assign90890_e139535_d_n7, assign90890_e139535_d_n8, assign90890_e139535_d_n9, assign90890_e139535_d_n10, assign90890_e139535_d_n11, assign90890_e139535_d_n14,) = {
    if (((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2129 != 0.0)) {
        let assign90890_e139533: f64 = (0.5 * locals.var_vbs_max_over__blk2123);
        (assign90890_e139533, (0.5 * locals.var_vbs_max_over__blk2123_dn0), (0.5 * locals.var_vbs_max_over__blk2123_dn2), (0.5 * locals.var_vbs_max_over__blk2123_dn4), (0.5 * locals.var_vbs_max_over__blk2123_dn5), (0.5 * locals.var_vbs_max_over__blk2123_dn6), (0.5 * locals.var_vbs_max_over__blk2123_dn7), (0.5 * locals.var_vbs_max_over__blk2123_dn8), (0.5 * locals.var_vbs_max_over__blk2123_dn9), (0.5 * locals.var_vbs_max_over__blk2123_dn10), (0.5 * locals.var_vbs_max_over__blk2123_dn11), (0.5 * locals.var_vbs_max_over__blk2123_dn14),)
    } else {
        (locals.var_vbs_bnd_over__blk2124, locals.var_vbs_bnd_over__blk2124_dn0, locals.var_vbs_bnd_over__blk2124_dn2, locals.var_vbs_bnd_over__blk2124_dn4, locals.var_vbs_bnd_over__blk2124_dn5, locals.var_vbs_bnd_over__blk2124_dn6, locals.var_vbs_bnd_over__blk2124_dn7, locals.var_vbs_bnd_over__blk2124_dn8, locals.var_vbs_bnd_over__blk2124_dn9, locals.var_vbs_bnd_over__blk2124_dn10, locals.var_vbs_bnd_over__blk2124_dn11, locals.var_vbs_bnd_over__blk2124_dn14,)
    }
};
        locals.var_vbs_bnd_over__blk2124 = assign90890_e139535;
        locals.var_vbs_bnd_over__blk2124_dn0 = assign90890_e139535_d_n0;
        locals.var_vbs_bnd_over__blk2124_dn2 = assign90890_e139535_d_n2;
        locals.var_vbs_bnd_over__blk2124_dn4 = assign90890_e139535_d_n4;
        locals.var_vbs_bnd_over__blk2124_dn5 = assign90890_e139535_d_n5;
        locals.var_vbs_bnd_over__blk2124_dn6 = assign90890_e139535_d_n6;
        locals.var_vbs_bnd_over__blk2124_dn7 = assign90890_e139535_d_n7;
        locals.var_vbs_bnd_over__blk2124_dn8 = assign90890_e139535_d_n8;
        locals.var_vbs_bnd_over__blk2124_dn9 = assign90890_e139535_d_n9;
        locals.var_vbs_bnd_over__blk2124_dn10 = assign90890_e139535_d_n10;
        locals.var_vbs_bnd_over__blk2124_dn11 = assign90890_e139535_d_n11;
        locals.var_vbs_bnd_over__blk2124_dn14 = assign90890_e139535_d_n14;
        locals.var_vbs_bnd_over__blk2124_rv = 0.0;

        let assign90900_e139537: f64 = if param_given[338] { 1.0 } else { 0.0 };
        locals.var_guard2130 = assign90900_e139537;
        locals.var_guard2130_rv = 0.0;

        let (assign90910_e139545, assign90910_e139545_d_n0, assign90910_e139545_d_n2, assign90910_e139545_d_n4, assign90910_e139545_d_n5, assign90910_e139545_d_n6, assign90910_e139545_d_n7, assign90910_e139545_d_n8, assign90910_e139545_d_n9, assign90910_e139545_d_n10, assign90910_e139545_d_n11, assign90910_e139545_d_n14,) = {
    if (((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2130 != 0.0)) {
        (p.p338, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbs_max_over__blk2123, locals.var_vbs_max_over__blk2123_dn0, locals.var_vbs_max_over__blk2123_dn2, locals.var_vbs_max_over__blk2123_dn4, locals.var_vbs_max_over__blk2123_dn5, locals.var_vbs_max_over__blk2123_dn6, locals.var_vbs_max_over__blk2123_dn7, locals.var_vbs_max_over__blk2123_dn8, locals.var_vbs_max_over__blk2123_dn9, locals.var_vbs_max_over__blk2123_dn10, locals.var_vbs_max_over__blk2123_dn11, locals.var_vbs_max_over__blk2123_dn14,)
    }
};
        locals.var_vbs_max_over__blk2123 = assign90910_e139545;
        locals.var_vbs_max_over__blk2123_dn0 = assign90910_e139545_d_n0;
        locals.var_vbs_max_over__blk2123_dn2 = assign90910_e139545_d_n2;
        locals.var_vbs_max_over__blk2123_dn4 = assign90910_e139545_d_n4;
        locals.var_vbs_max_over__blk2123_dn5 = assign90910_e139545_d_n5;
        locals.var_vbs_max_over__blk2123_dn6 = assign90910_e139545_d_n6;
        locals.var_vbs_max_over__blk2123_dn7 = assign90910_e139545_d_n7;
        locals.var_vbs_max_over__blk2123_dn8 = assign90910_e139545_d_n8;
        locals.var_vbs_max_over__blk2123_dn9 = assign90910_e139545_d_n9;
        locals.var_vbs_max_over__blk2123_dn10 = assign90910_e139545_d_n10;
        locals.var_vbs_max_over__blk2123_dn11 = assign90910_e139545_d_n11;
        locals.var_vbs_max_over__blk2123_dn14 = assign90910_e139545_d_n14;
        locals.var_vbs_max_over__blk2123_rv = 0.0;

        let assign90920_e139547: f64 = if param_given[339] { 1.0 } else { 0.0 };
        locals.var_guard2131 = assign90920_e139547;
        locals.var_guard2131_rv = 0.0;

        let (assign90930_e139555, assign90930_e139555_d_n0, assign90930_e139555_d_n2, assign90930_e139555_d_n4, assign90930_e139555_d_n5, assign90930_e139555_d_n6, assign90930_e139555_d_n7, assign90930_e139555_d_n8, assign90930_e139555_d_n9, assign90930_e139555_d_n10, assign90930_e139555_d_n11, assign90930_e139555_d_n14,) = {
    if (((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2131 != 0.0)) {
        (p.p339, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbs_bnd_over__blk2124, locals.var_vbs_bnd_over__blk2124_dn0, locals.var_vbs_bnd_over__blk2124_dn2, locals.var_vbs_bnd_over__blk2124_dn4, locals.var_vbs_bnd_over__blk2124_dn5, locals.var_vbs_bnd_over__blk2124_dn6, locals.var_vbs_bnd_over__blk2124_dn7, locals.var_vbs_bnd_over__blk2124_dn8, locals.var_vbs_bnd_over__blk2124_dn9, locals.var_vbs_bnd_over__blk2124_dn10, locals.var_vbs_bnd_over__blk2124_dn11, locals.var_vbs_bnd_over__blk2124_dn14,)
    }
};
        locals.var_vbs_bnd_over__blk2124 = assign90930_e139555;
        locals.var_vbs_bnd_over__blk2124_dn0 = assign90930_e139555_d_n0;
        locals.var_vbs_bnd_over__blk2124_dn2 = assign90930_e139555_d_n2;
        locals.var_vbs_bnd_over__blk2124_dn4 = assign90930_e139555_d_n4;
        locals.var_vbs_bnd_over__blk2124_dn5 = assign90930_e139555_d_n5;
        locals.var_vbs_bnd_over__blk2124_dn6 = assign90930_e139555_d_n6;
        locals.var_vbs_bnd_over__blk2124_dn7 = assign90930_e139555_d_n7;
        locals.var_vbs_bnd_over__blk2124_dn8 = assign90930_e139555_d_n8;
        locals.var_vbs_bnd_over__blk2124_dn9 = assign90930_e139555_d_n9;
        locals.var_vbs_bnd_over__blk2124_dn10 = assign90930_e139555_d_n10;
        locals.var_vbs_bnd_over__blk2124_dn11 = assign90930_e139555_d_n11;
        locals.var_vbs_bnd_over__blk2124_dn14 = assign90930_e139555_d_n14;
        locals.var_vbs_bnd_over__blk2124_rv = 0.0;

        let assign90940_e139557: f64 = if param_given[338] { 1.0 } else { 0.0 };
        locals.var_guard2132 = assign90940_e139557;
        locals.var_guard2132_rv = 0.0;

        let (assign90950_e139570, assign90950_e139570_d_n0, assign90950_e139570_d_n2, assign90950_e139570_d_n4, assign90950_e139570_d_n5, assign90950_e139570_d_n6, assign90950_e139570_d_n7, assign90950_e139570_d_n8, assign90950_e139570_d_n9, assign90950_e139570_d_n10, assign90950_e139570_d_n11, assign90950_e139570_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2131 == 0.0)) && (locals.var_guard2132 != 0.0)) {
        let assign90950_e139568: f64 = (0.5 * locals.var_vbs_max_over__blk2123);
        (assign90950_e139568, (0.5 * locals.var_vbs_max_over__blk2123_dn0), (0.5 * locals.var_vbs_max_over__blk2123_dn2), (0.5 * locals.var_vbs_max_over__blk2123_dn4), (0.5 * locals.var_vbs_max_over__blk2123_dn5), (0.5 * locals.var_vbs_max_over__blk2123_dn6), (0.5 * locals.var_vbs_max_over__blk2123_dn7), (0.5 * locals.var_vbs_max_over__blk2123_dn8), (0.5 * locals.var_vbs_max_over__blk2123_dn9), (0.5 * locals.var_vbs_max_over__blk2123_dn10), (0.5 * locals.var_vbs_max_over__blk2123_dn11), (0.5 * locals.var_vbs_max_over__blk2123_dn14),)
    } else {
        (locals.var_vbs_bnd_over__blk2124, locals.var_vbs_bnd_over__blk2124_dn0, locals.var_vbs_bnd_over__blk2124_dn2, locals.var_vbs_bnd_over__blk2124_dn4, locals.var_vbs_bnd_over__blk2124_dn5, locals.var_vbs_bnd_over__blk2124_dn6, locals.var_vbs_bnd_over__blk2124_dn7, locals.var_vbs_bnd_over__blk2124_dn8, locals.var_vbs_bnd_over__blk2124_dn9, locals.var_vbs_bnd_over__blk2124_dn10, locals.var_vbs_bnd_over__blk2124_dn11, locals.var_vbs_bnd_over__blk2124_dn14,)
    }
};
        locals.var_vbs_bnd_over__blk2124 = assign90950_e139570;
        locals.var_vbs_bnd_over__blk2124_dn0 = assign90950_e139570_d_n0;
        locals.var_vbs_bnd_over__blk2124_dn2 = assign90950_e139570_d_n2;
        locals.var_vbs_bnd_over__blk2124_dn4 = assign90950_e139570_d_n4;
        locals.var_vbs_bnd_over__blk2124_dn5 = assign90950_e139570_d_n5;
        locals.var_vbs_bnd_over__blk2124_dn6 = assign90950_e139570_d_n6;
        locals.var_vbs_bnd_over__blk2124_dn7 = assign90950_e139570_d_n7;
        locals.var_vbs_bnd_over__blk2124_dn8 = assign90950_e139570_d_n8;
        locals.var_vbs_bnd_over__blk2124_dn9 = assign90950_e139570_d_n9;
        locals.var_vbs_bnd_over__blk2124_dn10 = assign90950_e139570_d_n10;
        locals.var_vbs_bnd_over__blk2124_dn11 = assign90950_e139570_d_n11;
        locals.var_vbs_bnd_over__blk2124_dn14 = assign90950_e139570_d_n14;
        locals.var_vbs_bnd_over__blk2124_rv = 0.0;

        let assign90960_e139574: f64 = (locals.var_vbs_max_over__blk2123 * 0.5);
        let assign90960_e139575: f64 = if locals.var_vbs_bnd_over__blk2124 > assign90960_e139574 { 1.0 } else { 0.0 };
        locals.var_guard2133 = assign90960_e139575;
        locals.var_guard2133_rv = 0.0;

        let (assign90970_e139585, assign90970_e139585_d_n0, assign90970_e139585_d_n2, assign90970_e139585_d_n4, assign90970_e139585_d_n5, assign90970_e139585_d_n6, assign90970_e139585_d_n7, assign90970_e139585_d_n8, assign90970_e139585_d_n9, assign90970_e139585_d_n10, assign90970_e139585_d_n11, assign90970_e139585_d_n14,) = {
    if (((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2133 != 0.0)) {
        let assign90970_e139583: f64 = (0.5 * locals.var_vbs_max_over__blk2123);
        (assign90970_e139583, (0.5 * locals.var_vbs_max_over__blk2123_dn0), (0.5 * locals.var_vbs_max_over__blk2123_dn2), (0.5 * locals.var_vbs_max_over__blk2123_dn4), (0.5 * locals.var_vbs_max_over__blk2123_dn5), (0.5 * locals.var_vbs_max_over__blk2123_dn6), (0.5 * locals.var_vbs_max_over__blk2123_dn7), (0.5 * locals.var_vbs_max_over__blk2123_dn8), (0.5 * locals.var_vbs_max_over__blk2123_dn9), (0.5 * locals.var_vbs_max_over__blk2123_dn10), (0.5 * locals.var_vbs_max_over__blk2123_dn11), (0.5 * locals.var_vbs_max_over__blk2123_dn14),)
    } else {
        (locals.var_vbs_bnd_over__blk2124, locals.var_vbs_bnd_over__blk2124_dn0, locals.var_vbs_bnd_over__blk2124_dn2, locals.var_vbs_bnd_over__blk2124_dn4, locals.var_vbs_bnd_over__blk2124_dn5, locals.var_vbs_bnd_over__blk2124_dn6, locals.var_vbs_bnd_over__blk2124_dn7, locals.var_vbs_bnd_over__blk2124_dn8, locals.var_vbs_bnd_over__blk2124_dn9, locals.var_vbs_bnd_over__blk2124_dn10, locals.var_vbs_bnd_over__blk2124_dn11, locals.var_vbs_bnd_over__blk2124_dn14,)
    }
};
        locals.var_vbs_bnd_over__blk2124 = assign90970_e139585;
        locals.var_vbs_bnd_over__blk2124_dn0 = assign90970_e139585_d_n0;
        locals.var_vbs_bnd_over__blk2124_dn2 = assign90970_e139585_d_n2;
        locals.var_vbs_bnd_over__blk2124_dn4 = assign90970_e139585_d_n4;
        locals.var_vbs_bnd_over__blk2124_dn5 = assign90970_e139585_d_n5;
        locals.var_vbs_bnd_over__blk2124_dn6 = assign90970_e139585_d_n6;
        locals.var_vbs_bnd_over__blk2124_dn7 = assign90970_e139585_d_n7;
        locals.var_vbs_bnd_over__blk2124_dn8 = assign90970_e139585_d_n8;
        locals.var_vbs_bnd_over__blk2124_dn9 = assign90970_e139585_d_n9;
        locals.var_vbs_bnd_over__blk2124_dn10 = assign90970_e139585_d_n10;
        locals.var_vbs_bnd_over__blk2124_dn11 = assign90970_e139585_d_n11;
        locals.var_vbs_bnd_over__blk2124_dn14 = assign90970_e139585_d_n14;
        locals.var_vbs_bnd_over__blk2124_rv = 0.0;

        let assign90980_e139588: f64 = if p.p38 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2134 = assign90980_e139588;
        locals.var_guard2134_rv = 0.0;

        let (assign90990_e139597, assign90990_e139597_d_n0, assign90990_e139597_d_n2, assign90990_e139597_d_n4, assign90990_e139597_d_n5, assign90990_e139597_d_n6, assign90990_e139597_d_n7, assign90990_e139597_d_n8, assign90990_e139597_d_n9, assign90990_e139597_d_n10, assign90990_e139597_d_n11, assign90990_e139597_d_n14,) = {
    if (((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2134 != 0.0)) {
        let assign90990_e139595: f64 = (-locals.var_vxbgmt);
        (assign90990_e139595, (-locals.var_vxbgmt_dn0), (-locals.var_vxbgmt_dn2), (-locals.var_vxbgmt_dn4), (-locals.var_vxbgmt_dn5), (-locals.var_vxbgmt_dn6), (-locals.var_vxbgmt_dn7), (-locals.var_vxbgmt_dn8), (-locals.var_vxbgmt_dn9), (-locals.var_vxbgmt_dn10), (-locals.var_vxbgmt_dn11), (-locals.var_vxbgmt_dn14),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign90990_e139597;
        locals.var_t0_dn0 = assign90990_e139597_d_n0;
        locals.var_t0_dn2 = assign90990_e139597_d_n2;
        locals.var_t0_dn4 = assign90990_e139597_d_n4;
        locals.var_t0_dn5 = assign90990_e139597_d_n5;
        locals.var_t0_dn6 = assign90990_e139597_d_n6;
        locals.var_t0_dn7 = assign90990_e139597_d_n7;
        locals.var_t0_dn8 = assign90990_e139597_d_n8;
        locals.var_t0_dn9 = assign90990_e139597_d_n9;
        locals.var_t0_dn10 = assign90990_e139597_d_n10;
        locals.var_t0_dn11 = assign90990_e139597_d_n11;
        locals.var_t0_dn14 = assign90990_e139597_d_n14;
        locals.var_t0_rv = 0.0;

        let assign91000_e139600: f64 = if locals.var_t0 > locals.var_vbs_bnd_over__blk2124 { 1.0 } else { 0.0 };
        locals.var_guard2135 = assign91000_e139600;
        locals.var_guard2135_rv = 0.0;

        let (assign91010_e139612, assign91010_e139612_d_n0, assign91010_e139612_d_n2, assign91010_e139612_d_n4, assign91010_e139612_d_n5, assign91010_e139612_d_n6, assign91010_e139612_d_n7, assign91010_e139612_d_n8, assign91010_e139612_d_n9, assign91010_e139612_d_n10, assign91010_e139612_d_n11, assign91010_e139612_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2134 != 0.0)) && (locals.var_guard2135 != 0.0)) {
        let assign91010_e139610: f64 = (locals.var_t0 - locals.var_vbs_bnd_over__blk2124);
        (assign91010_e139610, (locals.var_t0_dn0 - locals.var_vbs_bnd_over__blk2124_dn0), (locals.var_t0_dn2 - locals.var_vbs_bnd_over__blk2124_dn2), (locals.var_t0_dn4 - locals.var_vbs_bnd_over__blk2124_dn4), (locals.var_t0_dn5 - locals.var_vbs_bnd_over__blk2124_dn5), (locals.var_t0_dn6 - locals.var_vbs_bnd_over__blk2124_dn6), (locals.var_t0_dn7 - locals.var_vbs_bnd_over__blk2124_dn7), (locals.var_t0_dn8 - locals.var_vbs_bnd_over__blk2124_dn8), (locals.var_t0_dn9 - locals.var_vbs_bnd_over__blk2124_dn9), (locals.var_t0_dn10 - locals.var_vbs_bnd_over__blk2124_dn10), (locals.var_t0_dn11 - locals.var_vbs_bnd_over__blk2124_dn11), (locals.var_t0_dn14 - locals.var_vbs_bnd_over__blk2124_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign91010_e139612;
        locals.var_t1_dn0 = assign91010_e139612_d_n0;
        locals.var_t1_dn2 = assign91010_e139612_d_n2;
        locals.var_t1_dn4 = assign91010_e139612_d_n4;
        locals.var_t1_dn5 = assign91010_e139612_d_n5;
        locals.var_t1_dn6 = assign91010_e139612_d_n6;
        locals.var_t1_dn7 = assign91010_e139612_d_n7;
        locals.var_t1_dn8 = assign91010_e139612_d_n8;
        locals.var_t1_dn9 = assign91010_e139612_d_n9;
        locals.var_t1_dn10 = assign91010_e139612_d_n10;
        locals.var_t1_dn11 = assign91010_e139612_d_n11;
        locals.var_t1_dn14 = assign91010_e139612_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign91020_e139624, assign91020_e139624_d_n0, assign91020_e139624_d_n2, assign91020_e139624_d_n4, assign91020_e139624_d_n5, assign91020_e139624_d_n6, assign91020_e139624_d_n7, assign91020_e139624_d_n8, assign91020_e139624_d_n9, assign91020_e139624_d_n10, assign91020_e139624_d_n11, assign91020_e139624_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2134 != 0.0)) && (locals.var_guard2135 != 0.0)) {
        let assign91020_e139622: f64 = (locals.var_vbs_max_over__blk2123 - locals.var_vbs_bnd_over__blk2124);
        (assign91020_e139622, (locals.var_vbs_max_over__blk2123_dn0 - locals.var_vbs_bnd_over__blk2124_dn0), (locals.var_vbs_max_over__blk2123_dn2 - locals.var_vbs_bnd_over__blk2124_dn2), (locals.var_vbs_max_over__blk2123_dn4 - locals.var_vbs_bnd_over__blk2124_dn4), (locals.var_vbs_max_over__blk2123_dn5 - locals.var_vbs_bnd_over__blk2124_dn5), (locals.var_vbs_max_over__blk2123_dn6 - locals.var_vbs_bnd_over__blk2124_dn6), (locals.var_vbs_max_over__blk2123_dn7 - locals.var_vbs_bnd_over__blk2124_dn7), (locals.var_vbs_max_over__blk2123_dn8 - locals.var_vbs_bnd_over__blk2124_dn8), (locals.var_vbs_max_over__blk2123_dn9 - locals.var_vbs_bnd_over__blk2124_dn9), (locals.var_vbs_max_over__blk2123_dn10 - locals.var_vbs_bnd_over__blk2124_dn10), (locals.var_vbs_max_over__blk2123_dn11 - locals.var_vbs_bnd_over__blk2124_dn11), (locals.var_vbs_max_over__blk2123_dn14 - locals.var_vbs_bnd_over__blk2124_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign91020_e139624;
        locals.var_t2_dn0 = assign91020_e139624_d_n0;
        locals.var_t2_dn2 = assign91020_e139624_d_n2;
        locals.var_t2_dn4 = assign91020_e139624_d_n4;
        locals.var_t2_dn5 = assign91020_e139624_d_n5;
        locals.var_t2_dn6 = assign91020_e139624_d_n6;
        locals.var_t2_dn7 = assign91020_e139624_d_n7;
        locals.var_t2_dn8 = assign91020_e139624_d_n8;
        locals.var_t2_dn9 = assign91020_e139624_d_n9;
        locals.var_t2_dn10 = assign91020_e139624_d_n10;
        locals.var_t2_dn11 = assign91020_e139624_d_n11;
        locals.var_t2_dn14 = assign91020_e139624_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign91030_e139636, assign91030_e139636_d_n0, assign91030_e139636_d_n2, assign91030_e139636_d_n4, assign91030_e139636_d_n5, assign91030_e139636_d_n6, assign91030_e139636_d_n7, assign91030_e139636_d_n8, assign91030_e139636_d_n9, assign91030_e139636_d_n10, assign91030_e139636_d_n11, assign91030_e139636_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2134 != 0.0)) && (locals.var_guard2135 != 0.0)) {
        let assign91030_e139634: f64 = (locals.var_t1 / locals.var_t2);
        (assign91030_e139634, (((locals.var_t1_dn0 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn0)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn2 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn2)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn4 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn4)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn5 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn5)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn6 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn6)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn7 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn7)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn8 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn8)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn9 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn9)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn10 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn10)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn11 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn11)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn14 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn14)) / (locals.var_t2 * locals.var_t2)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign91030_e139636;
        locals.var_tmf1_dn0 = assign91030_e139636_d_n0;
        locals.var_tmf1_dn2 = assign91030_e139636_d_n2;
        locals.var_tmf1_dn4 = assign91030_e139636_d_n4;
        locals.var_tmf1_dn5 = assign91030_e139636_d_n5;
        locals.var_tmf1_dn6 = assign91030_e139636_d_n6;
        locals.var_tmf1_dn7 = assign91030_e139636_d_n7;
        locals.var_tmf1_dn8 = assign91030_e139636_d_n8;
        locals.var_tmf1_dn9 = assign91030_e139636_d_n9;
        locals.var_tmf1_dn10 = assign91030_e139636_d_n10;
        locals.var_tmf1_dn11 = assign91030_e139636_d_n11;
        locals.var_tmf1_dn14 = assign91030_e139636_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign91040_e139648, assign91040_e139648_d_n0, assign91040_e139648_d_n2, assign91040_e139648_d_n4, assign91040_e139648_d_n5, assign91040_e139648_d_n6, assign91040_e139648_d_n7, assign91040_e139648_d_n8, assign91040_e139648_d_n9, assign91040_e139648_d_n10, assign91040_e139648_d_n11, assign91040_e139648_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2134 != 0.0)) && (locals.var_guard2135 != 0.0)) {
        let assign91040_e139646: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign91040_e139646, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign91040_e139648;
        locals.var_tmf2_dn0 = assign91040_e139648_d_n0;
        locals.var_tmf2_dn2 = assign91040_e139648_d_n2;
        locals.var_tmf2_dn4 = assign91040_e139648_d_n4;
        locals.var_tmf2_dn5 = assign91040_e139648_d_n5;
        locals.var_tmf2_dn6 = assign91040_e139648_d_n6;
        locals.var_tmf2_dn7 = assign91040_e139648_d_n7;
        locals.var_tmf2_dn8 = assign91040_e139648_d_n8;
        locals.var_tmf2_dn9 = assign91040_e139648_d_n9;
        locals.var_tmf2_dn10 = assign91040_e139648_d_n10;
        locals.var_tmf2_dn11 = assign91040_e139648_d_n11;
        locals.var_tmf2_dn14 = assign91040_e139648_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign91050_e139660, assign91050_e139660_d_n0, assign91050_e139660_d_n2, assign91050_e139660_d_n4, assign91050_e139660_d_n5, assign91050_e139660_d_n6, assign91050_e139660_d_n7, assign91050_e139660_d_n8, assign91050_e139660_d_n9, assign91050_e139660_d_n10, assign91050_e139660_d_n11, assign91050_e139660_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2134 != 0.0)) && (locals.var_guard2135 != 0.0)) {
        let assign91050_e139658: f64 = (locals.var_tmf2 * locals.var_tmf1);
        (assign91050_e139658, ((locals.var_tmf2_dn0 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn0)), ((locals.var_tmf2_dn2 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn2)), ((locals.var_tmf2_dn4 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn4)), ((locals.var_tmf2_dn5 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn5)), ((locals.var_tmf2_dn6 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn6)), ((locals.var_tmf2_dn7 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn7)), ((locals.var_tmf2_dn8 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn8)), ((locals.var_tmf2_dn9 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn9)), ((locals.var_tmf2_dn10 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn10)), ((locals.var_tmf2_dn11 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn11)), ((locals.var_tmf2_dn14 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_tmf3, locals.var_tmf3_dn0, locals.var_tmf3_dn2, locals.var_tmf3_dn4, locals.var_tmf3_dn5, locals.var_tmf3_dn6, locals.var_tmf3_dn7, locals.var_tmf3_dn8, locals.var_tmf3_dn9, locals.var_tmf3_dn10, locals.var_tmf3_dn11, locals.var_tmf3_dn14,)
    }
};
        locals.var_tmf3 = assign91050_e139660;
        locals.var_tmf3_dn0 = assign91050_e139660_d_n0;
        locals.var_tmf3_dn2 = assign91050_e139660_d_n2;
        locals.var_tmf3_dn4 = assign91050_e139660_d_n4;
        locals.var_tmf3_dn5 = assign91050_e139660_d_n5;
        locals.var_tmf3_dn6 = assign91050_e139660_d_n6;
        locals.var_tmf3_dn7 = assign91050_e139660_d_n7;
        locals.var_tmf3_dn8 = assign91050_e139660_d_n8;
        locals.var_tmf3_dn9 = assign91050_e139660_d_n9;
        locals.var_tmf3_dn10 = assign91050_e139660_d_n10;
        locals.var_tmf3_dn11 = assign91050_e139660_d_n11;
        locals.var_tmf3_dn14 = assign91050_e139660_d_n14;
        locals.var_tmf3_rv = 0.0;

        let (assign91060_e139672, assign91060_e139672_d_n0, assign91060_e139672_d_n2, assign91060_e139672_d_n4, assign91060_e139672_d_n5, assign91060_e139672_d_n6, assign91060_e139672_d_n7, assign91060_e139672_d_n8, assign91060_e139672_d_n9, assign91060_e139672_d_n10, assign91060_e139672_d_n11, assign91060_e139672_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2134 != 0.0)) && (locals.var_guard2135 != 0.0)) {
        let assign91060_e139670: f64 = (locals.var_tmf2 * locals.var_tmf2);
        (assign91060_e139670, ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0)), ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2)), ((locals.var_tmf2_dn4 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn4)), ((locals.var_tmf2_dn5 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn5)), ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6)), ((locals.var_tmf2_dn7 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn7)), ((locals.var_tmf2_dn8 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn8)), ((locals.var_tmf2_dn9 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn9)), ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10)), ((locals.var_tmf2_dn11 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn11)), ((locals.var_tmf2_dn14 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn14)),)
    } else {
        (locals.var_tmf4, locals.var_tmf4_dn0, locals.var_tmf4_dn2, locals.var_tmf4_dn4, locals.var_tmf4_dn5, locals.var_tmf4_dn6, locals.var_tmf4_dn7, locals.var_tmf4_dn8, locals.var_tmf4_dn9, locals.var_tmf4_dn10, locals.var_tmf4_dn11, locals.var_tmf4_dn14,)
    }
};
        locals.var_tmf4 = assign91060_e139672;
        locals.var_tmf4_dn0 = assign91060_e139672_d_n0;
        locals.var_tmf4_dn2 = assign91060_e139672_d_n2;
        locals.var_tmf4_dn4 = assign91060_e139672_d_n4;
        locals.var_tmf4_dn5 = assign91060_e139672_d_n5;
        locals.var_tmf4_dn6 = assign91060_e139672_d_n6;
        locals.var_tmf4_dn7 = assign91060_e139672_d_n7;
        locals.var_tmf4_dn8 = assign91060_e139672_d_n8;
        locals.var_tmf4_dn9 = assign91060_e139672_d_n9;
        locals.var_tmf4_dn10 = assign91060_e139672_d_n10;
        locals.var_tmf4_dn11 = assign91060_e139672_d_n11;
        locals.var_tmf4_dn14 = assign91060_e139672_d_n14;
        locals.var_tmf4_rv = 0.0;

        let (assign91070_e139692, assign91070_e139692_d_n0, assign91070_e139692_d_n2, assign91070_e139692_d_n4, assign91070_e139692_d_n5, assign91070_e139692_d_n6, assign91070_e139692_d_n7, assign91070_e139692_d_n8, assign91070_e139692_d_n9, assign91070_e139692_d_n10, assign91070_e139692_d_n11, assign91070_e139692_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2134 != 0.0)) && (locals.var_guard2135 != 0.0)) {
        let assign91070_e139683: f64 = (1.0 + locals.var_tmf1);
        let assign91070_e139685: f64 = (assign91070_e139683 + locals.var_tmf2);
        let assign91070_e139687: f64 = (assign91070_e139685 + locals.var_tmf3);
        let assign91070_e139689: f64 = (assign91070_e139687 + locals.var_tmf4);
        let assign91070_e139690: f64 = (1.0 / assign91070_e139689);
        (assign91070_e139690, (-((((locals.var_tmf1_dn0 + locals.var_tmf2_dn0) + locals.var_tmf3_dn0) + locals.var_tmf4_dn0) / (assign91070_e139689 * assign91070_e139689))), (-((((locals.var_tmf1_dn2 + locals.var_tmf2_dn2) + locals.var_tmf3_dn2) + locals.var_tmf4_dn2) / (assign91070_e139689 * assign91070_e139689))), (-((((locals.var_tmf1_dn4 + locals.var_tmf2_dn4) + locals.var_tmf3_dn4) + locals.var_tmf4_dn4) / (assign91070_e139689 * assign91070_e139689))), (-((((locals.var_tmf1_dn5 + locals.var_tmf2_dn5) + locals.var_tmf3_dn5) + locals.var_tmf4_dn5) / (assign91070_e139689 * assign91070_e139689))), (-((((locals.var_tmf1_dn6 + locals.var_tmf2_dn6) + locals.var_tmf3_dn6) + locals.var_tmf4_dn6) / (assign91070_e139689 * assign91070_e139689))), (-((((locals.var_tmf1_dn7 + locals.var_tmf2_dn7) + locals.var_tmf3_dn7) + locals.var_tmf4_dn7) / (assign91070_e139689 * assign91070_e139689))), (-((((locals.var_tmf1_dn8 + locals.var_tmf2_dn8) + locals.var_tmf3_dn8) + locals.var_tmf4_dn8) / (assign91070_e139689 * assign91070_e139689))), (-((((locals.var_tmf1_dn9 + locals.var_tmf2_dn9) + locals.var_tmf3_dn9) + locals.var_tmf4_dn9) / (assign91070_e139689 * assign91070_e139689))), (-((((locals.var_tmf1_dn10 + locals.var_tmf2_dn10) + locals.var_tmf3_dn10) + locals.var_tmf4_dn10) / (assign91070_e139689 * assign91070_e139689))), (-((((locals.var_tmf1_dn11 + locals.var_tmf2_dn11) + locals.var_tmf3_dn11) + locals.var_tmf4_dn11) / (assign91070_e139689 * assign91070_e139689))), (-((((locals.var_tmf1_dn14 + locals.var_tmf2_dn14) + locals.var_tmf3_dn14) + locals.var_tmf4_dn14) / (assign91070_e139689 * assign91070_e139689))),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign91070_e139692;
        locals.var_tmf0_dn0 = assign91070_e139692_d_n0;
        locals.var_tmf0_dn2 = assign91070_e139692_d_n2;
        locals.var_tmf0_dn4 = assign91070_e139692_d_n4;
        locals.var_tmf0_dn5 = assign91070_e139692_d_n5;
        locals.var_tmf0_dn6 = assign91070_e139692_d_n6;
        locals.var_tmf0_dn7 = assign91070_e139692_d_n7;
        locals.var_tmf0_dn8 = assign91070_e139692_d_n8;
        locals.var_tmf0_dn9 = assign91070_e139692_d_n9;
        locals.var_tmf0_dn10 = assign91070_e139692_d_n10;
        locals.var_tmf0_dn11 = assign91070_e139692_d_n11;
        locals.var_tmf0_dn14 = assign91070_e139692_d_n14;
        locals.var_tmf0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_351(
        locals: &mut StampLocals,
    ) {
        let (assign91080_e139719, assign91080_e139719_d_n0, assign91080_e139719_d_n2, assign91080_e139719_d_n4, assign91080_e139719_d_n5, assign91080_e139719_d_n6, assign91080_e139719_d_n7, assign91080_e139719_d_n8, assign91080_e139719_d_n9, assign91080_e139719_d_n10, assign91080_e139719_d_n11, assign91080_e139719_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2134 != 0.0)) && (locals.var_guard2135 != 0.0)) {
        let assign91080_e139703: f64 = (2.0 * locals.var_tmf1);
        let assign91080_e139704: f64 = (1.0 + assign91080_e139703);
        let assign91080_e139707: f64 = (3.0 * locals.var_tmf2);
        let assign91080_e139708: f64 = (assign91080_e139704 + assign91080_e139707);
        let assign91080_e139711: f64 = (4.0 * locals.var_tmf3);
        let assign91080_e139712: f64 = (assign91080_e139708 + assign91080_e139711);
        let assign91080_e139713: f64 = (-assign91080_e139712);
        let assign91080_e139715: f64 = (assign91080_e139713 * locals.var_tmf0);
        let assign91080_e139717: f64 = (assign91080_e139715 * locals.var_tmf0);
        (assign91080_e139717, (((((-(((2.0 * locals.var_tmf1_dn0) + (3.0 * locals.var_tmf2_dn0)) + (4.0 * locals.var_tmf3_dn0))) * locals.var_tmf0) + (assign91080_e139713 * locals.var_tmf0_dn0)) * locals.var_tmf0) + (assign91080_e139715 * locals.var_tmf0_dn0)), (((((-(((2.0 * locals.var_tmf1_dn2) + (3.0 * locals.var_tmf2_dn2)) + (4.0 * locals.var_tmf3_dn2))) * locals.var_tmf0) + (assign91080_e139713 * locals.var_tmf0_dn2)) * locals.var_tmf0) + (assign91080_e139715 * locals.var_tmf0_dn2)), (((((-(((2.0 * locals.var_tmf1_dn4) + (3.0 * locals.var_tmf2_dn4)) + (4.0 * locals.var_tmf3_dn4))) * locals.var_tmf0) + (assign91080_e139713 * locals.var_tmf0_dn4)) * locals.var_tmf0) + (assign91080_e139715 * locals.var_tmf0_dn4)), (((((-(((2.0 * locals.var_tmf1_dn5) + (3.0 * locals.var_tmf2_dn5)) + (4.0 * locals.var_tmf3_dn5))) * locals.var_tmf0) + (assign91080_e139713 * locals.var_tmf0_dn5)) * locals.var_tmf0) + (assign91080_e139715 * locals.var_tmf0_dn5)), (((((-(((2.0 * locals.var_tmf1_dn6) + (3.0 * locals.var_tmf2_dn6)) + (4.0 * locals.var_tmf3_dn6))) * locals.var_tmf0) + (assign91080_e139713 * locals.var_tmf0_dn6)) * locals.var_tmf0) + (assign91080_e139715 * locals.var_tmf0_dn6)), (((((-(((2.0 * locals.var_tmf1_dn7) + (3.0 * locals.var_tmf2_dn7)) + (4.0 * locals.var_tmf3_dn7))) * locals.var_tmf0) + (assign91080_e139713 * locals.var_tmf0_dn7)) * locals.var_tmf0) + (assign91080_e139715 * locals.var_tmf0_dn7)), (((((-(((2.0 * locals.var_tmf1_dn8) + (3.0 * locals.var_tmf2_dn8)) + (4.0 * locals.var_tmf3_dn8))) * locals.var_tmf0) + (assign91080_e139713 * locals.var_tmf0_dn8)) * locals.var_tmf0) + (assign91080_e139715 * locals.var_tmf0_dn8)), (((((-(((2.0 * locals.var_tmf1_dn9) + (3.0 * locals.var_tmf2_dn9)) + (4.0 * locals.var_tmf3_dn9))) * locals.var_tmf0) + (assign91080_e139713 * locals.var_tmf0_dn9)) * locals.var_tmf0) + (assign91080_e139715 * locals.var_tmf0_dn9)), (((((-(((2.0 * locals.var_tmf1_dn10) + (3.0 * locals.var_tmf2_dn10)) + (4.0 * locals.var_tmf3_dn10))) * locals.var_tmf0) + (assign91080_e139713 * locals.var_tmf0_dn10)) * locals.var_tmf0) + (assign91080_e139715 * locals.var_tmf0_dn10)), (((((-(((2.0 * locals.var_tmf1_dn11) + (3.0 * locals.var_tmf2_dn11)) + (4.0 * locals.var_tmf3_dn11))) * locals.var_tmf0) + (assign91080_e139713 * locals.var_tmf0_dn11)) * locals.var_tmf0) + (assign91080_e139715 * locals.var_tmf0_dn11)), (((((-(((2.0 * locals.var_tmf1_dn14) + (3.0 * locals.var_tmf2_dn14)) + (4.0 * locals.var_tmf3_dn14))) * locals.var_tmf0) + (assign91080_e139713 * locals.var_tmf0_dn14)) * locals.var_tmf0) + (assign91080_e139715 * locals.var_tmf0_dn14)),)
    } else {
        (locals.var_t11, locals.var_t11_dn0, locals.var_t11_dn2, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11, locals.var_t11_dn14,)
    }
};
        locals.var_t11 = assign91080_e139719;
        locals.var_t11_dn0 = assign91080_e139719_d_n0;
        locals.var_t11_dn2 = assign91080_e139719_d_n2;
        locals.var_t11_dn4 = assign91080_e139719_d_n4;
        locals.var_t11_dn5 = assign91080_e139719_d_n5;
        locals.var_t11_dn6 = assign91080_e139719_d_n6;
        locals.var_t11_dn7 = assign91080_e139719_d_n7;
        locals.var_t11_dn8 = assign91080_e139719_d_n8;
        locals.var_t11_dn9 = assign91080_e139719_d_n9;
        locals.var_t11_dn10 = assign91080_e139719_d_n10;
        locals.var_t11_dn11 = assign91080_e139719_d_n11;
        locals.var_t11_dn14 = assign91080_e139719_d_n14;
        locals.var_t11_rv = 0.0;

        let (assign91090_e139733, assign91090_e139733_d_n0, assign91090_e139733_d_n2, assign91090_e139733_d_n4, assign91090_e139733_d_n5, assign91090_e139733_d_n6, assign91090_e139733_d_n7, assign91090_e139733_d_n8, assign91090_e139733_d_n9, assign91090_e139733_d_n10, assign91090_e139733_d_n11, assign91090_e139733_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2134 != 0.0)) && (locals.var_guard2135 != 0.0)) {
        let assign91090_e139730: f64 = (1.0 - locals.var_tmf0);
        let assign91090_e139731: f64 = (locals.var_t2 * assign91090_e139730);
        (assign91090_e139731, ((locals.var_t2_dn0 * assign91090_e139730) + (locals.var_t2 * (-locals.var_tmf0_dn0))), ((locals.var_t2_dn2 * assign91090_e139730) + (locals.var_t2 * (-locals.var_tmf0_dn2))), ((locals.var_t2_dn4 * assign91090_e139730) + (locals.var_t2 * (-locals.var_tmf0_dn4))), ((locals.var_t2_dn5 * assign91090_e139730) + (locals.var_t2 * (-locals.var_tmf0_dn5))), ((locals.var_t2_dn6 * assign91090_e139730) + (locals.var_t2 * (-locals.var_tmf0_dn6))), ((locals.var_t2_dn7 * assign91090_e139730) + (locals.var_t2 * (-locals.var_tmf0_dn7))), ((locals.var_t2_dn8 * assign91090_e139730) + (locals.var_t2 * (-locals.var_tmf0_dn8))), ((locals.var_t2_dn9 * assign91090_e139730) + (locals.var_t2 * (-locals.var_tmf0_dn9))), ((locals.var_t2_dn10 * assign91090_e139730) + (locals.var_t2 * (-locals.var_tmf0_dn10))), ((locals.var_t2_dn11 * assign91090_e139730) + (locals.var_t2 * (-locals.var_tmf0_dn11))), ((locals.var_t2_dn14 * assign91090_e139730) + (locals.var_t2 * (-locals.var_tmf0_dn14))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn14,)
    }
};
        locals.var_ty = assign91090_e139733;
        locals.var_ty_dn0 = assign91090_e139733_d_n0;
        locals.var_ty_dn2 = assign91090_e139733_d_n2;
        locals.var_ty_dn4 = assign91090_e139733_d_n4;
        locals.var_ty_dn5 = assign91090_e139733_d_n5;
        locals.var_ty_dn6 = assign91090_e139733_d_n6;
        locals.var_ty_dn7 = assign91090_e139733_d_n7;
        locals.var_ty_dn8 = assign91090_e139733_d_n8;
        locals.var_ty_dn9 = assign91090_e139733_d_n9;
        locals.var_ty_dn10 = assign91090_e139733_d_n10;
        locals.var_ty_dn11 = assign91090_e139733_d_n11;
        locals.var_ty_dn14 = assign91090_e139733_d_n14;
        locals.var_ty_rv = 0.0;

        let (assign91100_e139749, assign91100_e139749_d_n0, assign91100_e139749_d_n2, assign91100_e139749_d_n4, assign91100_e139749_d_n5, assign91100_e139749_d_n6, assign91100_e139749_d_n7, assign91100_e139749_d_n8, assign91100_e139749_d_n9, assign91100_e139749_d_n10, assign91100_e139749_d_n11, assign91100_e139749_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2134 != 0.0)) && (locals.var_guard2135 != 0.0)) {
        let assign91100_e139743: f64 = (1.0 - locals.var_tmf0);
        let assign91100_e139746: f64 = (locals.var_tmf1 * locals.var_t11);
        let assign91100_e139747: f64 = (assign91100_e139743 + assign91100_e139746);
        (assign91100_e139747, ((-locals.var_tmf0_dn0) + ((locals.var_tmf1_dn0 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn0))), ((-locals.var_tmf0_dn2) + ((locals.var_tmf1_dn2 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn2))), ((-locals.var_tmf0_dn4) + ((locals.var_tmf1_dn4 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn4))), ((-locals.var_tmf0_dn5) + ((locals.var_tmf1_dn5 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn5))), ((-locals.var_tmf0_dn6) + ((locals.var_tmf1_dn6 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn6))), ((-locals.var_tmf0_dn7) + ((locals.var_tmf1_dn7 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn7))), ((-locals.var_tmf0_dn8) + ((locals.var_tmf1_dn8 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn8))), ((-locals.var_tmf0_dn9) + ((locals.var_tmf1_dn9 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn9))), ((-locals.var_tmf0_dn10) + ((locals.var_tmf1_dn10 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn10))), ((-locals.var_tmf0_dn11) + ((locals.var_tmf1_dn11 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn11))), ((-locals.var_tmf0_dn14) + ((locals.var_tmf1_dn14 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn14))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign91100_e139749;
        locals.var_t0_dn0 = assign91100_e139749_d_n0;
        locals.var_t0_dn2 = assign91100_e139749_d_n2;
        locals.var_t0_dn4 = assign91100_e139749_d_n4;
        locals.var_t0_dn5 = assign91100_e139749_d_n5;
        locals.var_t0_dn6 = assign91100_e139749_d_n6;
        locals.var_t0_dn7 = assign91100_e139749_d_n7;
        locals.var_t0_dn8 = assign91100_e139749_d_n8;
        locals.var_t0_dn9 = assign91100_e139749_d_n9;
        locals.var_t0_dn10 = assign91100_e139749_d_n10;
        locals.var_t0_dn11 = assign91100_e139749_d_n11;
        locals.var_t0_dn14 = assign91100_e139749_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign91110_e139760, assign91110_e139760_d_n0, assign91110_e139760_d_n2, assign91110_e139760_d_n4, assign91110_e139760_d_n5, assign91110_e139760_d_n6, assign91110_e139760_d_n7, assign91110_e139760_d_n8, assign91110_e139760_d_n9, assign91110_e139760_d_n10, assign91110_e139760_d_n11, assign91110_e139760_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2134 != 0.0)) && (locals.var_guard2135 != 0.0)) {
        let assign91110_e139758: f64 = (-locals.var_t11);
        (assign91110_e139758, (-locals.var_t11_dn0), (-locals.var_t11_dn2), (-locals.var_t11_dn4), (-locals.var_t11_dn5), (-locals.var_t11_dn6), (-locals.var_t11_dn7), (-locals.var_t11_dn8), (-locals.var_t11_dn9), (-locals.var_t11_dn10), (-locals.var_t11_dn11), (-locals.var_t11_dn14),)
    } else {
        (locals.var_t11, locals.var_t11_dn0, locals.var_t11_dn2, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11, locals.var_t11_dn14,)
    }
};
        locals.var_t11 = assign91110_e139760;
        locals.var_t11_dn0 = assign91110_e139760_d_n0;
        locals.var_t11_dn2 = assign91110_e139760_d_n2;
        locals.var_t11_dn4 = assign91110_e139760_d_n4;
        locals.var_t11_dn5 = assign91110_e139760_d_n5;
        locals.var_t11_dn6 = assign91110_e139760_d_n6;
        locals.var_t11_dn7 = assign91110_e139760_d_n7;
        locals.var_t11_dn8 = assign91110_e139760_d_n8;
        locals.var_t11_dn9 = assign91110_e139760_d_n9;
        locals.var_t11_dn10 = assign91110_e139760_d_n10;
        locals.var_t11_dn11 = assign91110_e139760_d_n11;
        locals.var_t11_dn14 = assign91110_e139760_d_n14;
        locals.var_t11_rv = 0.0;

        let (assign91120_e139772, assign91120_e139772_d_n0, assign91120_e139772_d_n2, assign91120_e139772_d_n4, assign91120_e139772_d_n5, assign91120_e139772_d_n6, assign91120_e139772_d_n7, assign91120_e139772_d_n8, assign91120_e139772_d_n9, assign91120_e139772_d_n10, assign91120_e139772_d_n11, assign91120_e139772_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2134 != 0.0)) && (locals.var_guard2135 != 0.0)) {
        let assign91120_e139770: f64 = (locals.var_vbs_bnd_over__blk2124 + locals.var_ty);
        (assign91120_e139770, (locals.var_vbs_bnd_over__blk2124_dn0 + locals.var_ty_dn0), (locals.var_vbs_bnd_over__blk2124_dn2 + locals.var_ty_dn2), (locals.var_vbs_bnd_over__blk2124_dn4 + locals.var_ty_dn4), (locals.var_vbs_bnd_over__blk2124_dn5 + locals.var_ty_dn5), (locals.var_vbs_bnd_over__blk2124_dn6 + locals.var_ty_dn6), (locals.var_vbs_bnd_over__blk2124_dn7 + locals.var_ty_dn7), (locals.var_vbs_bnd_over__blk2124_dn8 + locals.var_ty_dn8), (locals.var_vbs_bnd_over__blk2124_dn9 + locals.var_ty_dn9), (locals.var_vbs_bnd_over__blk2124_dn10 + locals.var_ty_dn10), (locals.var_vbs_bnd_over__blk2124_dn11 + locals.var_ty_dn11), (locals.var_vbs_bnd_over__blk2124_dn14 + locals.var_ty_dn14),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign91120_e139772;
        locals.var_t10_dn0 = assign91120_e139772_d_n0;
        locals.var_t10_dn2 = assign91120_e139772_d_n2;
        locals.var_t10_dn4 = assign91120_e139772_d_n4;
        locals.var_t10_dn5 = assign91120_e139772_d_n5;
        locals.var_t10_dn6 = assign91120_e139772_d_n6;
        locals.var_t10_dn7 = assign91120_e139772_d_n7;
        locals.var_t10_dn8 = assign91120_e139772_d_n8;
        locals.var_t10_dn9 = assign91120_e139772_d_n9;
        locals.var_t10_dn10 = assign91120_e139772_d_n10;
        locals.var_t10_dn11 = assign91120_e139772_d_n11;
        locals.var_t10_dn14 = assign91120_e139772_d_n14;
        locals.var_t10_rv = 0.0;

        let (assign91130_e139783, assign91130_e139783_d_n0, assign91130_e139783_d_n2, assign91130_e139783_d_n4, assign91130_e139783_d_n5, assign91130_e139783_d_n6, assign91130_e139783_d_n7, assign91130_e139783_d_n8, assign91130_e139783_d_n9, assign91130_e139783_d_n10, assign91130_e139783_d_n11, assign91130_e139783_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2134 != 0.0)) && (locals.var_guard2135 == 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign91130_e139783;
        locals.var_t10_dn0 = assign91130_e139783_d_n0;
        locals.var_t10_dn2 = assign91130_e139783_d_n2;
        locals.var_t10_dn4 = assign91130_e139783_d_n4;
        locals.var_t10_dn5 = assign91130_e139783_d_n5;
        locals.var_t10_dn6 = assign91130_e139783_d_n6;
        locals.var_t10_dn7 = assign91130_e139783_d_n7;
        locals.var_t10_dn8 = assign91130_e139783_d_n8;
        locals.var_t10_dn9 = assign91130_e139783_d_n9;
        locals.var_t10_dn10 = assign91130_e139783_d_n10;
        locals.var_t10_dn11 = assign91130_e139783_d_n11;
        locals.var_t10_dn14 = assign91130_e139783_d_n14;
        locals.var_t10_rv = 0.0;

        let (assign91140_e139792, assign91140_e139792_d_n0, assign91140_e139792_d_n2, assign91140_e139792_d_n4, assign91140_e139792_d_n5, assign91140_e139792_d_n6, assign91140_e139792_d_n7, assign91140_e139792_d_n8, assign91140_e139792_d_n9, assign91140_e139792_d_n10, assign91140_e139792_d_n11, assign91140_e139792_d_n14,) = {
    if (((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2134 != 0.0)) {
        let assign91140_e139790: f64 = (-locals.var_t10);
        (assign91140_e139790, (-locals.var_t10_dn0), (-locals.var_t10_dn2), (-locals.var_t10_dn4), (-locals.var_t10_dn5), (-locals.var_t10_dn6), (-locals.var_t10_dn7), (-locals.var_t10_dn8), (-locals.var_t10_dn9), (-locals.var_t10_dn10), (-locals.var_t10_dn11), (-locals.var_t10_dn14),)
    } else {
        (locals.var_vxbgmtcl, locals.var_vxbgmtcl_dn0, locals.var_vxbgmtcl_dn2, locals.var_vxbgmtcl_dn4, locals.var_vxbgmtcl_dn5, locals.var_vxbgmtcl_dn6, locals.var_vxbgmtcl_dn7, locals.var_vxbgmtcl_dn8, locals.var_vxbgmtcl_dn9, locals.var_vxbgmtcl_dn10, locals.var_vxbgmtcl_dn11, locals.var_vxbgmtcl_dn14,)
    }
};
        locals.var_vxbgmtcl = assign91140_e139792;
        locals.var_vxbgmtcl_dn0 = assign91140_e139792_d_n0;
        locals.var_vxbgmtcl_dn2 = assign91140_e139792_d_n2;
        locals.var_vxbgmtcl_dn4 = assign91140_e139792_d_n4;
        locals.var_vxbgmtcl_dn5 = assign91140_e139792_d_n5;
        locals.var_vxbgmtcl_dn6 = assign91140_e139792_d_n6;
        locals.var_vxbgmtcl_dn7 = assign91140_e139792_d_n7;
        locals.var_vxbgmtcl_dn8 = assign91140_e139792_d_n8;
        locals.var_vxbgmtcl_dn9 = assign91140_e139792_d_n9;
        locals.var_vxbgmtcl_dn10 = assign91140_e139792_d_n10;
        locals.var_vxbgmtcl_dn11 = assign91140_e139792_d_n11;
        locals.var_vxbgmtcl_dn14 = assign91140_e139792_d_n14;
        locals.var_vxbgmtcl_rv = 0.0;

        let (assign91150_e139801, assign91150_e139801_d_n0, assign91150_e139801_d_n2, assign91150_e139801_d_n4, assign91150_e139801_d_n5, assign91150_e139801_d_n6, assign91150_e139801_d_n7, assign91150_e139801_d_n8, assign91150_e139801_d_n9, assign91150_e139801_d_n10, assign91150_e139801_d_n11, assign91150_e139801_d_n14,) = {
    if (((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2134 == 0.0)) {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn11, locals.var_vxbgmt_dn14,)
    } else {
        (locals.var_vxbgmtcl, locals.var_vxbgmtcl_dn0, locals.var_vxbgmtcl_dn2, locals.var_vxbgmtcl_dn4, locals.var_vxbgmtcl_dn5, locals.var_vxbgmtcl_dn6, locals.var_vxbgmtcl_dn7, locals.var_vxbgmtcl_dn8, locals.var_vxbgmtcl_dn9, locals.var_vxbgmtcl_dn10, locals.var_vxbgmtcl_dn11, locals.var_vxbgmtcl_dn14,)
    }
};
        locals.var_vxbgmtcl = assign91150_e139801;
        locals.var_vxbgmtcl_dn0 = assign91150_e139801_d_n0;
        locals.var_vxbgmtcl_dn2 = assign91150_e139801_d_n2;
        locals.var_vxbgmtcl_dn4 = assign91150_e139801_d_n4;
        locals.var_vxbgmtcl_dn5 = assign91150_e139801_d_n5;
        locals.var_vxbgmtcl_dn6 = assign91150_e139801_d_n6;
        locals.var_vxbgmtcl_dn7 = assign91150_e139801_d_n7;
        locals.var_vxbgmtcl_dn8 = assign91150_e139801_d_n8;
        locals.var_vxbgmtcl_dn9 = assign91150_e139801_d_n9;
        locals.var_vxbgmtcl_dn10 = assign91150_e139801_d_n10;
        locals.var_vxbgmtcl_dn11 = assign91150_e139801_d_n11;
        locals.var_vxbgmtcl_dn14 = assign91150_e139801_d_n14;
        locals.var_vxbgmtcl_rv = 0.0;

        let (assign91160_e139809, assign91160_e139809_d_n0, assign91160_e139809_d_n2, assign91160_e139809_d_n4, assign91160_e139809_d_n5, assign91160_e139809_d_n6, assign91160_e139809_d_n7, assign91160_e139809_d_n8, assign91160_e139809_d_n9, assign91160_e139809_d_n10, assign91160_e139809_d_n11, assign91160_e139809_d_n14,) = {
    if ((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) {
        let assign91160_e139807: f64 = (locals.var_cnst0over_func / locals.var_cox0_func);
        (assign91160_e139807, (locals.var_cnst0over_func_dn0 / locals.var_cox0_func), (locals.var_cnst0over_func_dn2 / locals.var_cox0_func), (locals.var_cnst0over_func_dn4 / locals.var_cox0_func), (locals.var_cnst0over_func_dn5 / locals.var_cox0_func), (locals.var_cnst0over_func_dn6 / locals.var_cox0_func), (locals.var_cnst0over_func_dn7 / locals.var_cox0_func), (locals.var_cnst0over_func_dn8 / locals.var_cox0_func), (locals.var_cnst0over_func_dn9 / locals.var_cox0_func), (locals.var_cnst0over_func_dn10 / locals.var_cox0_func), (locals.var_cnst0over_func_dn11 / locals.var_cox0_func), (locals.var_cnst0over_func_dn14 / locals.var_cox0_func),)
    } else {
        (locals.var_fac1, locals.var_fac1_dn0, locals.var_fac1_dn2, locals.var_fac1_dn4, locals.var_fac1_dn5, locals.var_fac1_dn6, locals.var_fac1_dn7, locals.var_fac1_dn8, locals.var_fac1_dn9, locals.var_fac1_dn10, locals.var_fac1_dn11, locals.var_fac1_dn14,)
    }
};
        locals.var_fac1 = assign91160_e139809;
        locals.var_fac1_dn0 = assign91160_e139809_d_n0;
        locals.var_fac1_dn2 = assign91160_e139809_d_n2;
        locals.var_fac1_dn4 = assign91160_e139809_d_n4;
        locals.var_fac1_dn5 = assign91160_e139809_d_n5;
        locals.var_fac1_dn6 = assign91160_e139809_d_n6;
        locals.var_fac1_dn7 = assign91160_e139809_d_n7;
        locals.var_fac1_dn8 = assign91160_e139809_d_n8;
        locals.var_fac1_dn9 = assign91160_e139809_d_n9;
        locals.var_fac1_dn10 = assign91160_e139809_d_n10;
        locals.var_fac1_dn11 = assign91160_e139809_d_n11;
        locals.var_fac1_dn14 = assign91160_e139809_d_n14;
        locals.var_fac1_rv = 0.0;

        let (assign91170_e139817, assign91170_e139817_d_n0, assign91170_e139817_d_n2, assign91170_e139817_d_n4, assign91170_e139817_d_n5, assign91170_e139817_d_n6, assign91170_e139817_d_n7, assign91170_e139817_d_n8, assign91170_e139817_d_n9, assign91170_e139817_d_n10, assign91170_e139817_d_n11, assign91170_e139817_d_n14,) = {
    if ((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) {
        let assign91170_e139815: f64 = (locals.var_fac1 * locals.var_fac1);
        (assign91170_e139815, ((locals.var_fac1_dn0 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn0)), ((locals.var_fac1_dn2 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn2)), ((locals.var_fac1_dn4 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn4)), ((locals.var_fac1_dn5 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn5)), ((locals.var_fac1_dn6 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn6)), ((locals.var_fac1_dn7 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn7)), ((locals.var_fac1_dn8 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn8)), ((locals.var_fac1_dn9 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn9)), ((locals.var_fac1_dn10 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn10)), ((locals.var_fac1_dn11 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn11)), ((locals.var_fac1_dn14 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn14)),)
    } else {
        (locals.var_fac1p2, locals.var_fac1p2_dn0, locals.var_fac1p2_dn2, locals.var_fac1p2_dn4, locals.var_fac1p2_dn5, locals.var_fac1p2_dn6, locals.var_fac1p2_dn7, locals.var_fac1p2_dn8, locals.var_fac1p2_dn9, locals.var_fac1p2_dn10, locals.var_fac1p2_dn11, locals.var_fac1p2_dn14,)
    }
};
        locals.var_fac1p2 = assign91170_e139817;
        locals.var_fac1p2_dn0 = assign91170_e139817_d_n0;
        locals.var_fac1p2_dn2 = assign91170_e139817_d_n2;
        locals.var_fac1p2_dn4 = assign91170_e139817_d_n4;
        locals.var_fac1p2_dn5 = assign91170_e139817_d_n5;
        locals.var_fac1p2_dn6 = assign91170_e139817_d_n6;
        locals.var_fac1p2_dn7 = assign91170_e139817_d_n7;
        locals.var_fac1p2_dn8 = assign91170_e139817_d_n8;
        locals.var_fac1p2_dn9 = assign91170_e139817_d_n9;
        locals.var_fac1p2_dn10 = assign91170_e139817_d_n10;
        locals.var_fac1p2_dn11 = assign91170_e139817_d_n11;
        locals.var_fac1p2_dn14 = assign91170_e139817_d_n14;
        locals.var_fac1p2_rv = 0.0;

        let (assign91180_e139826, assign91180_e139826_d_n2, assign91180_e139826_d_n7, assign91180_e139826_d_n8, assign91180_e139826_d_n9,) = {
    if ((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) {
        let assign91180_e139822: f64 = (-locals.var_vgbgmt);
        let assign91180_e139824: f64 = (assign91180_e139822 + locals.var_uc_vfbover);
        (assign91180_e139824, (-locals.var_vgbgmt_dn2), (-locals.var_vgbgmt_dn7), (-locals.var_vgbgmt_dn8), (-locals.var_vgbgmt_dn9),)
    } else {
        (locals.var_vgpld, locals.var_vgpld_dn2, locals.var_vgpld_dn7, locals.var_vgpld_dn8, locals.var_vgpld_dn9,)
    }
};
        locals.var_vgpld = assign91180_e139826;
        locals.var_vgpld_dn2 = assign91180_e139826_d_n2;
        locals.var_vgpld_dn7 = assign91180_e139826_d_n7;
        locals.var_vgpld_dn8 = assign91180_e139826_d_n8;
        locals.var_vgpld_dn9 = assign91180_e139826_d_n9;
        locals.var_vgpld_rv = 0.0;

        let (assign91190_e139837, assign91190_e139837_d_n0, assign91190_e139837_d_n2, assign91190_e139837_d_n4, assign91190_e139837_d_n5, assign91190_e139837_d_n6, assign91190_e139837_d_n7, assign91190_e139837_d_n8, assign91190_e139837_d_n9, assign91190_e139837_d_n10, assign91190_e139837_d_n11, assign91190_e139837_d_n14,) = {
    if ((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) {
        let assign91190_e139831: f64 = (-locals.var_vxbgmtcl);
        let assign91190_e139834: f64 = (10.0 * 2.220446049250313e-16);
        let assign91190_e139835: f64 = (assign91190_e139831 + assign91190_e139834);
        (assign91190_e139835, (-locals.var_vxbgmtcl_dn0), (-locals.var_vxbgmtcl_dn2), (-locals.var_vxbgmtcl_dn4), (-locals.var_vxbgmtcl_dn5), (-locals.var_vxbgmtcl_dn6), (-locals.var_vxbgmtcl_dn7), (-locals.var_vxbgmtcl_dn8), (-locals.var_vxbgmtcl_dn9), (-locals.var_vxbgmtcl_dn10), (-locals.var_vxbgmtcl_dn11), (-locals.var_vxbgmtcl_dn14),)
    } else {
        (locals.var_vgb_fb_ld, locals.var_vgb_fb_ld_dn0, locals.var_vgb_fb_ld_dn2, locals.var_vgb_fb_ld_dn4, locals.var_vgb_fb_ld_dn5, locals.var_vgb_fb_ld_dn6, locals.var_vgb_fb_ld_dn7, locals.var_vgb_fb_ld_dn8, locals.var_vgb_fb_ld_dn9, locals.var_vgb_fb_ld_dn10, locals.var_vgb_fb_ld_dn11, locals.var_vgb_fb_ld_dn14,)
    }
};
        locals.var_vgb_fb_ld = assign91190_e139837;
        locals.var_vgb_fb_ld_dn0 = assign91190_e139837_d_n0;
        locals.var_vgb_fb_ld_dn2 = assign91190_e139837_d_n2;
        locals.var_vgb_fb_ld_dn4 = assign91190_e139837_d_n4;
        locals.var_vgb_fb_ld_dn5 = assign91190_e139837_d_n5;
        locals.var_vgb_fb_ld_dn6 = assign91190_e139837_d_n6;
        locals.var_vgb_fb_ld_dn7 = assign91190_e139837_d_n7;
        locals.var_vgb_fb_ld_dn8 = assign91190_e139837_d_n8;
        locals.var_vgb_fb_ld_dn9 = assign91190_e139837_d_n9;
        locals.var_vgb_fb_ld_dn10 = assign91190_e139837_d_n10;
        locals.var_vgb_fb_ld_dn11 = assign91190_e139837_d_n11;
        locals.var_vgb_fb_ld_dn14 = assign91190_e139837_d_n14;
        locals.var_vgb_fb_ld_rv = 0.0;

        let (assign91200_e139843, assign91200_e139843_d_n0, assign91200_e139843_d_n2, assign91200_e139843_d_n4, assign91200_e139843_d_n5, assign91200_e139843_d_n6, assign91200_e139843_d_n7, assign91200_e139843_d_n8, assign91200_e139843_d_n9, assign91200_e139843_d_n10, assign91200_e139843_d_n11, assign91200_e139843_d_n14,) = {
    if ((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_dep_ld__blk2118, locals.var_q_dep_ld__blk2118_dn0, locals.var_q_dep_ld__blk2118_dn2, locals.var_q_dep_ld__blk2118_dn4, locals.var_q_dep_ld__blk2118_dn5, locals.var_q_dep_ld__blk2118_dn6, locals.var_q_dep_ld__blk2118_dn7, locals.var_q_dep_ld__blk2118_dn8, locals.var_q_dep_ld__blk2118_dn9, locals.var_q_dep_ld__blk2118_dn10, locals.var_q_dep_ld__blk2118_dn11, locals.var_q_dep_ld__blk2118_dn14,)
    }
};
        locals.var_q_dep_ld__blk2118 = assign91200_e139843;
        locals.var_q_dep_ld__blk2118_dn0 = assign91200_e139843_d_n0;
        locals.var_q_dep_ld__blk2118_dn2 = assign91200_e139843_d_n2;
        locals.var_q_dep_ld__blk2118_dn4 = assign91200_e139843_d_n4;
        locals.var_q_dep_ld__blk2118_dn5 = assign91200_e139843_d_n5;
        locals.var_q_dep_ld__blk2118_dn6 = assign91200_e139843_d_n6;
        locals.var_q_dep_ld__blk2118_dn7 = assign91200_e139843_d_n7;
        locals.var_q_dep_ld__blk2118_dn8 = assign91200_e139843_d_n8;
        locals.var_q_dep_ld__blk2118_dn9 = assign91200_e139843_d_n9;
        locals.var_q_dep_ld__blk2118_dn10 = assign91200_e139843_d_n10;
        locals.var_q_dep_ld__blk2118_dn11 = assign91200_e139843_d_n11;
        locals.var_q_dep_ld__blk2118_dn14 = assign91200_e139843_d_n14;
        locals.var_q_dep_ld__blk2118_rv = 0.0;

        let (assign91210_e139851,) = {
    if ((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) {
        let assign91210_e139849: f64 = (1.6021918e-19 * locals.var_nover_func);
        (assign91210_e139849,)
    } else {
        (locals.var_q_nsubld__blk2119,)
    }
};
        locals.var_q_nsubld__blk2119 = assign91210_e139851;
        locals.var_q_nsubld__blk2119_rv = 0.0;

        let (assign91220_e139859, assign91220_e139859_d_n0, assign91220_e139859_d_n2, assign91220_e139859_d_n4, assign91220_e139859_d_n5, assign91220_e139859_d_n6, assign91220_e139859_d_n7, assign91220_e139859_d_n8, assign91220_e139859_d_n9, assign91220_e139859_d_n10, assign91220_e139859_d_n11, assign91220_e139859_d_n14,) = {
    if ((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) {
        let assign91220_e139857: f64 = (locals.var_nin / locals.var_nover_func);
        (assign91220_e139857, (locals.var_nin_dn0 / locals.var_nover_func), (locals.var_nin_dn2 / locals.var_nover_func), (locals.var_nin_dn4 / locals.var_nover_func), (locals.var_nin_dn5 / locals.var_nover_func), (locals.var_nin_dn6 / locals.var_nover_func), (locals.var_nin_dn7 / locals.var_nover_func), (locals.var_nin_dn8 / locals.var_nover_func), (locals.var_nin_dn9 / locals.var_nover_func), (locals.var_nin_dn10 / locals.var_nover_func), (locals.var_nin_dn11 / locals.var_nover_func), (locals.var_nin_dn14 / locals.var_nover_func),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign91220_e139859;
        locals.var_t0_dn0 = assign91220_e139859_d_n0;
        locals.var_t0_dn2 = assign91220_e139859_d_n2;
        locals.var_t0_dn4 = assign91220_e139859_d_n4;
        locals.var_t0_dn5 = assign91220_e139859_d_n5;
        locals.var_t0_dn6 = assign91220_e139859_d_n6;
        locals.var_t0_dn7 = assign91220_e139859_d_n7;
        locals.var_t0_dn8 = assign91220_e139859_d_n8;
        locals.var_t0_dn9 = assign91220_e139859_d_n9;
        locals.var_t0_dn10 = assign91220_e139859_d_n10;
        locals.var_t0_dn11 = assign91220_e139859_d_n11;
        locals.var_t0_dn14 = assign91220_e139859_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign91230_e139867, assign91230_e139867_d_n0, assign91230_e139867_d_n2, assign91230_e139867_d_n4, assign91230_e139867_d_n5, assign91230_e139867_d_n6, assign91230_e139867_d_n7, assign91230_e139867_d_n8, assign91230_e139867_d_n9, assign91230_e139867_d_n10, assign91230_e139867_d_n11, assign91230_e139867_d_n14,) = {
    if ((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) {
        let assign91230_e139865: f64 = (locals.var_t0 * locals.var_t0);
        (assign91230_e139865, ((locals.var_t0_dn0 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn0)), ((locals.var_t0_dn2 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn2)), ((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)), ((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)), ((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)), ((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)), ((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)), ((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)), ((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)), ((locals.var_t0_dn11 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn11)), ((locals.var_t0_dn14 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn14)),)
    } else {
        (locals.var_cnst1over, locals.var_cnst1over_dn0, locals.var_cnst1over_dn2, locals.var_cnst1over_dn4, locals.var_cnst1over_dn5, locals.var_cnst1over_dn6, locals.var_cnst1over_dn7, locals.var_cnst1over_dn8, locals.var_cnst1over_dn9, locals.var_cnst1over_dn10, locals.var_cnst1over_dn11, locals.var_cnst1over_dn14,)
    }
};
        locals.var_cnst1over = assign91230_e139867;
        locals.var_cnst1over_dn0 = assign91230_e139867_d_n0;
        locals.var_cnst1over_dn2 = assign91230_e139867_d_n2;
        locals.var_cnst1over_dn4 = assign91230_e139867_d_n4;
        locals.var_cnst1over_dn5 = assign91230_e139867_d_n5;
        locals.var_cnst1over_dn6 = assign91230_e139867_d_n6;
        locals.var_cnst1over_dn7 = assign91230_e139867_d_n7;
        locals.var_cnst1over_dn8 = assign91230_e139867_d_n8;
        locals.var_cnst1over_dn9 = assign91230_e139867_d_n9;
        locals.var_cnst1over_dn10 = assign91230_e139867_d_n10;
        locals.var_cnst1over_dn11 = assign91230_e139867_d_n11;
        locals.var_cnst1over_dn14 = assign91230_e139867_d_n14;
        locals.var_cnst1over_rv = 0.0;

        let assign91240_e139870: f64 = (-locals.var_vxbgmtcl);
        let assign91240_e139871: f64 = (locals.var_beta * assign91240_e139870);
        let assign91240_e139873: f64 = if assign91240_e139871 >= 500.0 { 1.0 } else { 0.0 };
        locals.var_guard2136 = assign91240_e139873;
        locals.var_guard2136_rv = 0.0;

        let (assign91250_e139890, assign91250_e139890_d_n0, assign91250_e139890_d_n2, assign91250_e139890_d_n4, assign91250_e139890_d_n5, assign91250_e139890_d_n6, assign91250_e139890_d_n7, assign91250_e139890_d_n8, assign91250_e139890_d_n9, assign91250_e139890_d_n10, assign91250_e139890_d_n11, assign91250_e139890_d_n14,) = {
    if (((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2136 != 0.0)) {
        let assign91250_e139883: f64 = (-locals.var_vxbgmtcl);
        let assign91250_e139884: f64 = (locals.var_beta * assign91250_e139883);
        let assign91250_e139885: f64 = (1.0 + assign91250_e139884);
        let assign91250_e139887: f64 = (assign91250_e139885 - 500.0);
        let assign91250_e139888: f64 = (1.403592217853e217 * assign91250_e139887);
        (assign91250_e139888, (1.403592217853e217 * ((locals.var_beta_dn0 * assign91250_e139883) + (locals.var_beta * (-locals.var_vxbgmtcl_dn0)))), (1.403592217853e217 * ((locals.var_beta_dn2 * assign91250_e139883) + (locals.var_beta * (-locals.var_vxbgmtcl_dn2)))), (1.403592217853e217 * ((locals.var_beta_dn4 * assign91250_e139883) + (locals.var_beta * (-locals.var_vxbgmtcl_dn4)))), (1.403592217853e217 * ((locals.var_beta_dn5 * assign91250_e139883) + (locals.var_beta * (-locals.var_vxbgmtcl_dn5)))), (1.403592217853e217 * ((locals.var_beta_dn6 * assign91250_e139883) + (locals.var_beta * (-locals.var_vxbgmtcl_dn6)))), (1.403592217853e217 * ((locals.var_beta_dn7 * assign91250_e139883) + (locals.var_beta * (-locals.var_vxbgmtcl_dn7)))), (1.403592217853e217 * ((locals.var_beta_dn8 * assign91250_e139883) + (locals.var_beta * (-locals.var_vxbgmtcl_dn8)))), (1.403592217853e217 * ((locals.var_beta_dn9 * assign91250_e139883) + (locals.var_beta * (-locals.var_vxbgmtcl_dn9)))), (1.403592217853e217 * ((locals.var_beta_dn10 * assign91250_e139883) + (locals.var_beta * (-locals.var_vxbgmtcl_dn10)))), (1.403592217853e217 * ((locals.var_beta_dn11 * assign91250_e139883) + (locals.var_beta * (-locals.var_vxbgmtcl_dn11)))), (1.403592217853e217 * ((locals.var_beta_dn14 * assign91250_e139883) + (locals.var_beta * (-locals.var_vxbgmtcl_dn14)))),)
    } else {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn11, locals.var_exp_bvbs_dn14,)
    }
};
        locals.var_exp_bvbs = assign91250_e139890;
        locals.var_exp_bvbs_dn0 = assign91250_e139890_d_n0;
        locals.var_exp_bvbs_dn2 = assign91250_e139890_d_n2;
        locals.var_exp_bvbs_dn4 = assign91250_e139890_d_n4;
        locals.var_exp_bvbs_dn5 = assign91250_e139890_d_n5;
        locals.var_exp_bvbs_dn6 = assign91250_e139890_d_n6;
        locals.var_exp_bvbs_dn7 = assign91250_e139890_d_n7;
        locals.var_exp_bvbs_dn8 = assign91250_e139890_d_n8;
        locals.var_exp_bvbs_dn9 = assign91250_e139890_d_n9;
        locals.var_exp_bvbs_dn10 = assign91250_e139890_d_n10;
        locals.var_exp_bvbs_dn11 = assign91250_e139890_d_n11;
        locals.var_exp_bvbs_dn14 = assign91250_e139890_d_n14;
        locals.var_exp_bvbs_rv = 0.0;

        let (assign91260_e139898, assign91260_e139898_d_n0, assign91260_e139898_d_n2, assign91260_e139898_d_n4, assign91260_e139898_d_n5, assign91260_e139898_d_n6, assign91260_e139898_d_n7, assign91260_e139898_d_n8, assign91260_e139898_d_n9, assign91260_e139898_d_n10, assign91260_e139898_d_n11, assign91260_e139898_d_n14,) = {
    if (((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2136 != 0.0)) {
        (1.403592217853e217, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign91260_e139898;
        locals.var_t0_dn0 = assign91260_e139898_d_n0;
        locals.var_t0_dn2 = assign91260_e139898_d_n2;
        locals.var_t0_dn4 = assign91260_e139898_d_n4;
        locals.var_t0_dn5 = assign91260_e139898_d_n5;
        locals.var_t0_dn6 = assign91260_e139898_d_n6;
        locals.var_t0_dn7 = assign91260_e139898_d_n7;
        locals.var_t0_dn8 = assign91260_e139898_d_n8;
        locals.var_t0_dn9 = assign91260_e139898_d_n9;
        locals.var_t0_dn10 = assign91260_e139898_d_n10;
        locals.var_t0_dn11 = assign91260_e139898_d_n11;
        locals.var_t0_dn14 = assign91260_e139898_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign91270_e139910, assign91270_e139910_d_n0, assign91270_e139910_d_n2, assign91270_e139910_d_n4, assign91270_e139910_d_n5, assign91270_e139910_d_n6, assign91270_e139910_d_n7, assign91270_e139910_d_n8, assign91270_e139910_d_n9, assign91270_e139910_d_n10, assign91270_e139910_d_n11, assign91270_e139910_d_n14,) = {
    if (((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2136 == 0.0)) {
        let assign91270_e139907: f64 = (-locals.var_vxbgmtcl);
        let assign91270_e139908: f64 = (locals.var_beta * assign91270_e139907);
        (assign91270_e139908, ((locals.var_beta_dn0 * assign91270_e139907) + (locals.var_beta * (-locals.var_vxbgmtcl_dn0))), ((locals.var_beta_dn2 * assign91270_e139907) + (locals.var_beta * (-locals.var_vxbgmtcl_dn2))), ((locals.var_beta_dn4 * assign91270_e139907) + (locals.var_beta * (-locals.var_vxbgmtcl_dn4))), ((locals.var_beta_dn5 * assign91270_e139907) + (locals.var_beta * (-locals.var_vxbgmtcl_dn5))), ((locals.var_beta_dn6 * assign91270_e139907) + (locals.var_beta * (-locals.var_vxbgmtcl_dn6))), ((locals.var_beta_dn7 * assign91270_e139907) + (locals.var_beta * (-locals.var_vxbgmtcl_dn7))), ((locals.var_beta_dn8 * assign91270_e139907) + (locals.var_beta * (-locals.var_vxbgmtcl_dn8))), ((locals.var_beta_dn9 * assign91270_e139907) + (locals.var_beta * (-locals.var_vxbgmtcl_dn9))), ((locals.var_beta_dn10 * assign91270_e139907) + (locals.var_beta * (-locals.var_vxbgmtcl_dn10))), ((locals.var_beta_dn11 * assign91270_e139907) + (locals.var_beta * (-locals.var_vxbgmtcl_dn11))), ((locals.var_beta_dn14 * assign91270_e139907) + (locals.var_beta * (-locals.var_vxbgmtcl_dn14))),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign91270_e139910;
        locals.var_tmf1_dn0 = assign91270_e139910_d_n0;
        locals.var_tmf1_dn2 = assign91270_e139910_d_n2;
        locals.var_tmf1_dn4 = assign91270_e139910_d_n4;
        locals.var_tmf1_dn5 = assign91270_e139910_d_n5;
        locals.var_tmf1_dn6 = assign91270_e139910_d_n6;
        locals.var_tmf1_dn7 = assign91270_e139910_d_n7;
        locals.var_tmf1_dn8 = assign91270_e139910_d_n8;
        locals.var_tmf1_dn9 = assign91270_e139910_d_n9;
        locals.var_tmf1_dn10 = assign91270_e139910_d_n10;
        locals.var_tmf1_dn11 = assign91270_e139910_d_n11;
        locals.var_tmf1_dn14 = assign91270_e139910_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign91280_e139919, assign91280_e139919_d_n0, assign91280_e139919_d_n2, assign91280_e139919_d_n4, assign91280_e139919_d_n5, assign91280_e139919_d_n6, assign91280_e139919_d_n7, assign91280_e139919_d_n8, assign91280_e139919_d_n9, assign91280_e139919_d_n10, assign91280_e139919_d_n11, assign91280_e139919_d_n14,) = {
    if (((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2136 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn11, locals.var_exp_bvbs_dn14,)
    }
};
        locals.var_exp_bvbs = assign91280_e139919;
        locals.var_exp_bvbs_dn0 = assign91280_e139919_d_n0;
        locals.var_exp_bvbs_dn2 = assign91280_e139919_d_n2;
        locals.var_exp_bvbs_dn4 = assign91280_e139919_d_n4;
        locals.var_exp_bvbs_dn5 = assign91280_e139919_d_n5;
        locals.var_exp_bvbs_dn6 = assign91280_e139919_d_n6;
        locals.var_exp_bvbs_dn7 = assign91280_e139919_d_n7;
        locals.var_exp_bvbs_dn8 = assign91280_e139919_d_n8;
        locals.var_exp_bvbs_dn9 = assign91280_e139919_d_n9;
        locals.var_exp_bvbs_dn10 = assign91280_e139919_d_n10;
        locals.var_exp_bvbs_dn11 = assign91280_e139919_d_n11;
        locals.var_exp_bvbs_dn14 = assign91280_e139919_d_n14;
        locals.var_exp_bvbs_rv = 0.0;

        let mut assign91290_loop_guard: usize = 0;
        while {
            let assign91290_cond_e139929: f64 = if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2136 == 0.0)) && (locals.var_tmf1 >= 60.0)) { 1.0 } else { 0.0 };
            assign91290_cond_e139929 != 0.0
        } {
            assign91290_loop_guard += 1;
            assert!(assign91290_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign91290_body0_e139940, assign91290_body0_e139940_d_n0, assign91290_body0_e139940_d_n2, assign91290_body0_e139940_d_n4, assign91290_body0_e139940_d_n5, assign91290_body0_e139940_d_n6, assign91290_body0_e139940_d_n7, assign91290_body0_e139940_d_n8, assign91290_body0_e139940_d_n9, assign91290_body0_e139940_d_n10, assign91290_body0_e139940_d_n11, assign91290_body0_e139940_d_n14,) = {
    if (((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2136 == 0.0)) {
        let assign91290_body0_e139938: f64 = (locals.var_exp_bvbs * 1.14200738981568e26);
        (assign91290_body0_e139938, (locals.var_exp_bvbs_dn0 * 1.14200738981568e26), (locals.var_exp_bvbs_dn2 * 1.14200738981568e26), (locals.var_exp_bvbs_dn4 * 1.14200738981568e26), (locals.var_exp_bvbs_dn5 * 1.14200738981568e26), (locals.var_exp_bvbs_dn6 * 1.14200738981568e26), (locals.var_exp_bvbs_dn7 * 1.14200738981568e26), (locals.var_exp_bvbs_dn8 * 1.14200738981568e26), (locals.var_exp_bvbs_dn9 * 1.14200738981568e26), (locals.var_exp_bvbs_dn10 * 1.14200738981568e26), (locals.var_exp_bvbs_dn11 * 1.14200738981568e26), (locals.var_exp_bvbs_dn14 * 1.14200738981568e26),)
    } else {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn11, locals.var_exp_bvbs_dn14,)
    }
};
            locals.var_exp_bvbs = assign91290_body0_e139940;
            locals.var_exp_bvbs_dn0 = assign91290_body0_e139940_d_n0;
            locals.var_exp_bvbs_dn2 = assign91290_body0_e139940_d_n2;
            locals.var_exp_bvbs_dn4 = assign91290_body0_e139940_d_n4;
            locals.var_exp_bvbs_dn5 = assign91290_body0_e139940_d_n5;
            locals.var_exp_bvbs_dn6 = assign91290_body0_e139940_d_n6;
            locals.var_exp_bvbs_dn7 = assign91290_body0_e139940_d_n7;
            locals.var_exp_bvbs_dn8 = assign91290_body0_e139940_d_n8;
            locals.var_exp_bvbs_dn9 = assign91290_body0_e139940_d_n9;
            locals.var_exp_bvbs_dn10 = assign91290_body0_e139940_d_n10;
            locals.var_exp_bvbs_dn11 = assign91290_body0_e139940_d_n11;
            locals.var_exp_bvbs_dn14 = assign91290_body0_e139940_d_n14;
            locals.var_exp_bvbs_rv = 0.0;
            let (assign91290_body1_e139951, assign91290_body1_e139951_d_n0, assign91290_body1_e139951_d_n2, assign91290_body1_e139951_d_n4, assign91290_body1_e139951_d_n5, assign91290_body1_e139951_d_n6, assign91290_body1_e139951_d_n7, assign91290_body1_e139951_d_n8, assign91290_body1_e139951_d_n9, assign91290_body1_e139951_d_n10, assign91290_body1_e139951_d_n11, assign91290_body1_e139951_d_n14,) = {
    if (((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2136 == 0.0)) {
        let assign91290_body1_e139949: f64 = (locals.var_tmf1 - 60.0);
        (assign91290_body1_e139949, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
            locals.var_tmf1 = assign91290_body1_e139951;
            locals.var_tmf1_dn0 = assign91290_body1_e139951_d_n0;
            locals.var_tmf1_dn2 = assign91290_body1_e139951_d_n2;
            locals.var_tmf1_dn4 = assign91290_body1_e139951_d_n4;
            locals.var_tmf1_dn5 = assign91290_body1_e139951_d_n5;
            locals.var_tmf1_dn6 = assign91290_body1_e139951_d_n6;
            locals.var_tmf1_dn7 = assign91290_body1_e139951_d_n7;
            locals.var_tmf1_dn8 = assign91290_body1_e139951_d_n8;
            locals.var_tmf1_dn9 = assign91290_body1_e139951_d_n9;
            locals.var_tmf1_dn10 = assign91290_body1_e139951_d_n10;
            locals.var_tmf1_dn11 = assign91290_body1_e139951_d_n11;
            locals.var_tmf1_dn14 = assign91290_body1_e139951_d_n14;
            locals.var_tmf1_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_352(
        locals: &mut StampLocals,
    ) {
        let (assign91300_e139963, assign91300_e139963_d_n0, assign91300_e139963_d_n2, assign91300_e139963_d_n4, assign91300_e139963_d_n5, assign91300_e139963_d_n6, assign91300_e139963_d_n7, assign91300_e139963_d_n8, assign91300_e139963_d_n9, assign91300_e139963_d_n10, assign91300_e139963_d_n11, assign91300_e139963_d_n14,) = {
    if (((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2136 == 0.0)) {
        let assign91300_e139960: f64 = (locals.var_tmf1).exp();
        let assign91300_e139961: f64 = (locals.var_exp_bvbs * assign91300_e139960);
        (assign91300_e139961, ((locals.var_exp_bvbs_dn0 * assign91300_e139960) + (locals.var_exp_bvbs * (assign91300_e139960 * locals.var_tmf1_dn0))), ((locals.var_exp_bvbs_dn2 * assign91300_e139960) + (locals.var_exp_bvbs * (assign91300_e139960 * locals.var_tmf1_dn2))), ((locals.var_exp_bvbs_dn4 * assign91300_e139960) + (locals.var_exp_bvbs * (assign91300_e139960 * locals.var_tmf1_dn4))), ((locals.var_exp_bvbs_dn5 * assign91300_e139960) + (locals.var_exp_bvbs * (assign91300_e139960 * locals.var_tmf1_dn5))), ((locals.var_exp_bvbs_dn6 * assign91300_e139960) + (locals.var_exp_bvbs * (assign91300_e139960 * locals.var_tmf1_dn6))), ((locals.var_exp_bvbs_dn7 * assign91300_e139960) + (locals.var_exp_bvbs * (assign91300_e139960 * locals.var_tmf1_dn7))), ((locals.var_exp_bvbs_dn8 * assign91300_e139960) + (locals.var_exp_bvbs * (assign91300_e139960 * locals.var_tmf1_dn8))), ((locals.var_exp_bvbs_dn9 * assign91300_e139960) + (locals.var_exp_bvbs * (assign91300_e139960 * locals.var_tmf1_dn9))), ((locals.var_exp_bvbs_dn10 * assign91300_e139960) + (locals.var_exp_bvbs * (assign91300_e139960 * locals.var_tmf1_dn10))), ((locals.var_exp_bvbs_dn11 * assign91300_e139960) + (locals.var_exp_bvbs * (assign91300_e139960 * locals.var_tmf1_dn11))), ((locals.var_exp_bvbs_dn14 * assign91300_e139960) + (locals.var_exp_bvbs * (assign91300_e139960 * locals.var_tmf1_dn14))),)
    } else {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn11, locals.var_exp_bvbs_dn14,)
    }
};
        locals.var_exp_bvbs = assign91300_e139963;
        locals.var_exp_bvbs_dn0 = assign91300_e139963_d_n0;
        locals.var_exp_bvbs_dn2 = assign91300_e139963_d_n2;
        locals.var_exp_bvbs_dn4 = assign91300_e139963_d_n4;
        locals.var_exp_bvbs_dn5 = assign91300_e139963_d_n5;
        locals.var_exp_bvbs_dn6 = assign91300_e139963_d_n6;
        locals.var_exp_bvbs_dn7 = assign91300_e139963_d_n7;
        locals.var_exp_bvbs_dn8 = assign91300_e139963_d_n8;
        locals.var_exp_bvbs_dn9 = assign91300_e139963_d_n9;
        locals.var_exp_bvbs_dn10 = assign91300_e139963_d_n10;
        locals.var_exp_bvbs_dn11 = assign91300_e139963_d_n11;
        locals.var_exp_bvbs_dn14 = assign91300_e139963_d_n14;
        locals.var_exp_bvbs_rv = 0.0;

        let (assign91310_e139972, assign91310_e139972_d_n0, assign91310_e139972_d_n2, assign91310_e139972_d_n4, assign91310_e139972_d_n5, assign91310_e139972_d_n6, assign91310_e139972_d_n7, assign91310_e139972_d_n8, assign91310_e139972_d_n9, assign91310_e139972_d_n10, assign91310_e139972_d_n11, assign91310_e139972_d_n14,) = {
    if (((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2136 == 0.0)) {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn11, locals.var_exp_bvbs_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign91310_e139972;
        locals.var_t0_dn0 = assign91310_e139972_d_n0;
        locals.var_t0_dn2 = assign91310_e139972_d_n2;
        locals.var_t0_dn4 = assign91310_e139972_d_n4;
        locals.var_t0_dn5 = assign91310_e139972_d_n5;
        locals.var_t0_dn6 = assign91310_e139972_d_n6;
        locals.var_t0_dn7 = assign91310_e139972_d_n7;
        locals.var_t0_dn8 = assign91310_e139972_d_n8;
        locals.var_t0_dn9 = assign91310_e139972_d_n9;
        locals.var_t0_dn10 = assign91310_e139972_d_n10;
        locals.var_t0_dn11 = assign91310_e139972_d_n11;
        locals.var_t0_dn14 = assign91310_e139972_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign91320_e139987, assign91320_e139987_d_n0, assign91320_e139987_d_n2, assign91320_e139987_d_n4, assign91320_e139987_d_n5, assign91320_e139987_d_n6, assign91320_e139987_d_n7, assign91320_e139987_d_n8, assign91320_e139987_d_n9, assign91320_e139987_d_n10, assign91320_e139987_d_n11, assign91320_e139987_d_n14,) = {
    if (((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign91320_e139979: f64 = (-locals.var_vgpld);
        let assign91320_e139981: f64 = (assign91320_e139979 * 0.5);
        let assign91320_e139983: f64 = (assign91320_e139981 - 0.5);
        let assign91320_e139985: f64 = (assign91320_e139983 - 1.0);
        (assign91320_e139985, 0.0, ((-locals.var_vgpld_dn2) * 0.5), 0.0, 0.0, 0.0, ((-locals.var_vgpld_dn7) * 0.5), ((-locals.var_vgpld_dn8) * 0.5), ((-locals.var_vgpld_dn9) * 0.5), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign91320_e139987;
        locals.var_tmf1_dn0 = assign91320_e139987_d_n0;
        locals.var_tmf1_dn2 = assign91320_e139987_d_n2;
        locals.var_tmf1_dn4 = assign91320_e139987_d_n4;
        locals.var_tmf1_dn5 = assign91320_e139987_d_n5;
        locals.var_tmf1_dn6 = assign91320_e139987_d_n6;
        locals.var_tmf1_dn7 = assign91320_e139987_d_n7;
        locals.var_tmf1_dn8 = assign91320_e139987_d_n8;
        locals.var_tmf1_dn9 = assign91320_e139987_d_n9;
        locals.var_tmf1_dn10 = assign91320_e139987_d_n10;
        locals.var_tmf1_dn11 = assign91320_e139987_d_n11;
        locals.var_tmf1_dn14 = assign91320_e139987_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign91330_e139999, assign91330_e139999_d_n0, assign91330_e139999_d_n2, assign91330_e139999_d_n4, assign91330_e139999_d_n5, assign91330_e139999_d_n6, assign91330_e139999_d_n7, assign91330_e139999_d_n8, assign91330_e139999_d_n9, assign91330_e139999_d_n10, assign91330_e139999_d_n11, assign91330_e139999_d_n14,) = {
    if (((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign91330_e139995: f64 = (4.0 * 0.5);
        let assign91330_e139997: f64 = assign91330_e139995;
        (assign91330_e139997, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign91330_e139999;
        locals.var_tmf2_dn0 = assign91330_e139999_d_n0;
        locals.var_tmf2_dn2 = assign91330_e139999_d_n2;
        locals.var_tmf2_dn4 = assign91330_e139999_d_n4;
        locals.var_tmf2_dn5 = assign91330_e139999_d_n5;
        locals.var_tmf2_dn6 = assign91330_e139999_d_n6;
        locals.var_tmf2_dn7 = assign91330_e139999_d_n7;
        locals.var_tmf2_dn8 = assign91330_e139999_d_n8;
        locals.var_tmf2_dn9 = assign91330_e139999_d_n9;
        locals.var_tmf2_dn10 = assign91330_e139999_d_n10;
        locals.var_tmf2_dn11 = assign91330_e139999_d_n11;
        locals.var_tmf2_dn14 = assign91330_e139999_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign91340_e140013, assign91340_e140013_d_n0, assign91340_e140013_d_n2, assign91340_e140013_d_n4, assign91340_e140013_d_n5, assign91340_e140013_d_n6, assign91340_e140013_d_n7, assign91340_e140013_d_n8, assign91340_e140013_d_n9, assign91340_e140013_d_n10, assign91340_e140013_d_n11, assign91340_e140013_d_n14,) = {
    if (((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let (assign91340_e140011, assign91340_e140011_d_n0, assign91340_e140011_d_n2, assign91340_e140011_d_n4, assign91340_e140011_d_n5, assign91340_e140011_d_n6, assign91340_e140011_d_n7, assign91340_e140011_d_n8, assign91340_e140011_d_n9, assign91340_e140011_d_n10, assign91340_e140011_d_n11, assign91340_e140011_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign91340_e140010: f64 = (-locals.var_tmf2);
                (assign91340_e140010, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign91340_e140011, assign91340_e140011_d_n0, assign91340_e140011_d_n2, assign91340_e140011_d_n4, assign91340_e140011_d_n5, assign91340_e140011_d_n6, assign91340_e140011_d_n7, assign91340_e140011_d_n8, assign91340_e140011_d_n9, assign91340_e140011_d_n10, assign91340_e140011_d_n11, assign91340_e140011_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign91340_e140013;
        locals.var_tmf2_dn0 = assign91340_e140013_d_n0;
        locals.var_tmf2_dn2 = assign91340_e140013_d_n2;
        locals.var_tmf2_dn4 = assign91340_e140013_d_n4;
        locals.var_tmf2_dn5 = assign91340_e140013_d_n5;
        locals.var_tmf2_dn6 = assign91340_e140013_d_n6;
        locals.var_tmf2_dn7 = assign91340_e140013_d_n7;
        locals.var_tmf2_dn8 = assign91340_e140013_d_n8;
        locals.var_tmf2_dn9 = assign91340_e140013_d_n9;
        locals.var_tmf2_dn10 = assign91340_e140013_d_n10;
        locals.var_tmf2_dn11 = assign91340_e140013_d_n11;
        locals.var_tmf2_dn14 = assign91340_e140013_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign91350_e140026, assign91350_e140026_d_n0, assign91350_e140026_d_n2, assign91350_e140026_d_n4, assign91350_e140026_d_n5, assign91350_e140026_d_n6, assign91350_e140026_d_n7, assign91350_e140026_d_n8, assign91350_e140026_d_n9, assign91350_e140026_d_n10, assign91350_e140026_d_n11, assign91350_e140026_d_n14,) = {
    if (((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign91350_e140021: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign91350_e140023: f64 = (assign91350_e140021 + locals.var_tmf2);
        let assign91350_e140024: f64 = (assign91350_e140023).sqrt();
        (assign91350_e140024, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign91350_e140024)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign91350_e140024)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign91350_e140024)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign91350_e140024)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign91350_e140024)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign91350_e140024)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign91350_e140024)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign91350_e140024)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign91350_e140024)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign91350_e140024)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign91350_e140024)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign91350_e140026;
        locals.var_tmf2_dn0 = assign91350_e140026_d_n0;
        locals.var_tmf2_dn2 = assign91350_e140026_d_n2;
        locals.var_tmf2_dn4 = assign91350_e140026_d_n4;
        locals.var_tmf2_dn5 = assign91350_e140026_d_n5;
        locals.var_tmf2_dn6 = assign91350_e140026_d_n6;
        locals.var_tmf2_dn7 = assign91350_e140026_d_n7;
        locals.var_tmf2_dn8 = assign91350_e140026_d_n8;
        locals.var_tmf2_dn9 = assign91350_e140026_d_n9;
        locals.var_tmf2_dn10 = assign91350_e140026_d_n10;
        locals.var_tmf2_dn11 = assign91350_e140026_d_n11;
        locals.var_tmf2_dn14 = assign91350_e140026_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign91360_e140040, assign91360_e140040_d_n0, assign91360_e140040_d_n2, assign91360_e140040_d_n4, assign91360_e140040_d_n5, assign91360_e140040_d_n6, assign91360_e140040_d_n7, assign91360_e140040_d_n8, assign91360_e140040_d_n9, assign91360_e140040_d_n10, assign91360_e140040_d_n11, assign91360_e140040_d_n14,) = {
    if (((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign91360_e140036: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign91360_e140037: f64 = (1.0 + assign91360_e140036);
        let assign91360_e140038: f64 = (0.5 * assign91360_e140037);
        (assign91360_e140038, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign91360_e140040;
        locals.var_t0_dn0 = assign91360_e140040_d_n0;
        locals.var_t0_dn2 = assign91360_e140040_d_n2;
        locals.var_t0_dn4 = assign91360_e140040_d_n4;
        locals.var_t0_dn5 = assign91360_e140040_d_n5;
        locals.var_t0_dn6 = assign91360_e140040_d_n6;
        locals.var_t0_dn7 = assign91360_e140040_d_n7;
        locals.var_t0_dn8 = assign91360_e140040_d_n8;
        locals.var_t0_dn9 = assign91360_e140040_d_n9;
        locals.var_t0_dn10 = assign91360_e140040_d_n10;
        locals.var_t0_dn11 = assign91360_e140040_d_n11;
        locals.var_t0_dn14 = assign91360_e140040_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign91370_e140054, assign91370_e140054_d_n0, assign91370_e140054_d_n2, assign91370_e140054_d_n4, assign91370_e140054_d_n5, assign91370_e140054_d_n6, assign91370_e140054_d_n7, assign91370_e140054_d_n8, assign91370_e140054_d_n9, assign91370_e140054_d_n10, assign91370_e140054_d_n11, assign91370_e140054_d_n14,) = {
    if (((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign91370_e140050: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign91370_e140051: f64 = (0.5 * assign91370_e140050);
        let assign91370_e140052: f64 = (0.5 + assign91370_e140051);
        (assign91370_e140052, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign91370_e140054;
        locals.var_t1_dn0 = assign91370_e140054_d_n0;
        locals.var_t1_dn2 = assign91370_e140054_d_n2;
        locals.var_t1_dn4 = assign91370_e140054_d_n4;
        locals.var_t1_dn5 = assign91370_e140054_d_n5;
        locals.var_t1_dn6 = assign91370_e140054_d_n6;
        locals.var_t1_dn7 = assign91370_e140054_d_n7;
        locals.var_t1_dn8 = assign91370_e140054_d_n8;
        locals.var_t1_dn9 = assign91370_e140054_d_n9;
        locals.var_t1_dn10 = assign91370_e140054_d_n10;
        locals.var_t1_dn11 = assign91370_e140054_d_n11;
        locals.var_t1_dn14 = assign91370_e140054_d_n14;
        locals.var_t1_rv = 0.0;

        let assign91380_e140057: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign91380_e140060: f64 = (-locals.var_t1);
        let assign91380_e140065: f64 = if ((assign91380_e140057 > assign91380_e140060) && (locals.var_t1 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2137 = assign91380_e140065;
        locals.var_guard2137_rv = 0.0;

        let (assign91390_e140081, assign91390_e140081_d_n0, assign91390_e140081_d_n2, assign91390_e140081_d_n4, assign91390_e140081_d_n5, assign91390_e140081_d_n6, assign91390_e140081_d_n7, assign91390_e140081_d_n8, assign91390_e140081_d_n9, assign91390_e140081_d_n10, assign91390_e140081_d_n11, assign91390_e140081_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2137 != 0.0)) {
        let assign91390_e140075: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign91390_e140077: f64 = assign91390_e140075;
        let assign91390_e140079: f64 = (assign91390_e140077 + locals.var_t1);
        (assign91390_e140079, (locals.var_vxbgmtcl_dn0 + locals.var_t1_dn0), ((locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2) + locals.var_t1_dn2), (locals.var_vxbgmtcl_dn4 + locals.var_t1_dn4), (locals.var_vxbgmtcl_dn5 + locals.var_t1_dn5), (locals.var_vxbgmtcl_dn6 + locals.var_t1_dn6), ((locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7) + locals.var_t1_dn7), ((locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8) + locals.var_t1_dn8), ((locals.var_vgpld_dn9 + locals.var_vxbgmtcl_dn9) + locals.var_t1_dn9), (locals.var_vxbgmtcl_dn10 + locals.var_t1_dn10), (locals.var_vxbgmtcl_dn11 + locals.var_t1_dn11), (locals.var_vxbgmtcl_dn14 + locals.var_t1_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign91390_e140081;
        locals.var_tmf1_dn0 = assign91390_e140081_d_n0;
        locals.var_tmf1_dn2 = assign91390_e140081_d_n2;
        locals.var_tmf1_dn4 = assign91390_e140081_d_n4;
        locals.var_tmf1_dn5 = assign91390_e140081_d_n5;
        locals.var_tmf1_dn6 = assign91390_e140081_d_n6;
        locals.var_tmf1_dn7 = assign91390_e140081_d_n7;
        locals.var_tmf1_dn8 = assign91390_e140081_d_n8;
        locals.var_tmf1_dn9 = assign91390_e140081_d_n9;
        locals.var_tmf1_dn10 = assign91390_e140081_d_n10;
        locals.var_tmf1_dn11 = assign91390_e140081_d_n11;
        locals.var_tmf1_dn14 = assign91390_e140081_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign91400_e140093, assign91400_e140093_d_n0, assign91400_e140093_d_n2, assign91400_e140093_d_n4, assign91400_e140093_d_n5, assign91400_e140093_d_n6, assign91400_e140093_d_n7, assign91400_e140093_d_n8, assign91400_e140093_d_n9, assign91400_e140093_d_n10, assign91400_e140093_d_n11, assign91400_e140093_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2137 != 0.0)) {
        let assign91400_e140091: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign91400_e140091, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign91400_e140093;
        locals.var_x2_dn0 = assign91400_e140093_d_n0;
        locals.var_x2_dn2 = assign91400_e140093_d_n2;
        locals.var_x2_dn4 = assign91400_e140093_d_n4;
        locals.var_x2_dn5 = assign91400_e140093_d_n5;
        locals.var_x2_dn6 = assign91400_e140093_d_n6;
        locals.var_x2_dn7 = assign91400_e140093_d_n7;
        locals.var_x2_dn8 = assign91400_e140093_d_n8;
        locals.var_x2_dn9 = assign91400_e140093_d_n9;
        locals.var_x2_dn10 = assign91400_e140093_d_n10;
        locals.var_x2_dn11 = assign91400_e140093_d_n11;
        locals.var_x2_dn14 = assign91400_e140093_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign91410_e140105, assign91410_e140105_d_n0, assign91410_e140105_d_n2, assign91410_e140105_d_n4, assign91410_e140105_d_n5, assign91410_e140105_d_n6, assign91410_e140105_d_n7, assign91410_e140105_d_n8, assign91410_e140105_d_n9, assign91410_e140105_d_n10, assign91410_e140105_d_n11, assign91410_e140105_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2137 != 0.0)) {
        let assign91410_e140103: f64 = (locals.var_t1 * locals.var_t1);
        (assign91410_e140103, ((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)), ((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)), ((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)), ((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)), ((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)), ((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)), ((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)), ((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)), ((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)), ((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)), ((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14)),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign91410_e140105;
        locals.var_xmax2_dn0 = assign91410_e140105_d_n0;
        locals.var_xmax2_dn2 = assign91410_e140105_d_n2;
        locals.var_xmax2_dn4 = assign91410_e140105_d_n4;
        locals.var_xmax2_dn5 = assign91410_e140105_d_n5;
        locals.var_xmax2_dn6 = assign91410_e140105_d_n6;
        locals.var_xmax2_dn7 = assign91410_e140105_d_n7;
        locals.var_xmax2_dn8 = assign91410_e140105_d_n8;
        locals.var_xmax2_dn9 = assign91410_e140105_d_n9;
        locals.var_xmax2_dn10 = assign91410_e140105_d_n10;
        locals.var_xmax2_dn11 = assign91410_e140105_d_n11;
        locals.var_xmax2_dn14 = assign91410_e140105_d_n14;
        locals.var_xmax2_rv = 0.0;

        let (assign91420_e140115, assign91420_e140115_d_n0, assign91420_e140115_d_n2, assign91420_e140115_d_n4, assign91420_e140115_d_n5, assign91420_e140115_d_n6, assign91420_e140115_d_n7, assign91420_e140115_d_n8, assign91420_e140115_d_n9, assign91420_e140115_d_n10, assign91420_e140115_d_n11, assign91420_e140115_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2137 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign91420_e140115;
        locals.var_xp_dn0 = assign91420_e140115_d_n0;
        locals.var_xp_dn2 = assign91420_e140115_d_n2;
        locals.var_xp_dn4 = assign91420_e140115_d_n4;
        locals.var_xp_dn5 = assign91420_e140115_d_n5;
        locals.var_xp_dn6 = assign91420_e140115_d_n6;
        locals.var_xp_dn7 = assign91420_e140115_d_n7;
        locals.var_xp_dn8 = assign91420_e140115_d_n8;
        locals.var_xp_dn9 = assign91420_e140115_d_n9;
        locals.var_xp_dn10 = assign91420_e140115_d_n10;
        locals.var_xp_dn11 = assign91420_e140115_d_n11;
        locals.var_xp_dn14 = assign91420_e140115_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign91430_e140125, assign91430_e140125_d_n0, assign91430_e140125_d_n2, assign91430_e140125_d_n4, assign91430_e140125_d_n5, assign91430_e140125_d_n6, assign91430_e140125_d_n7, assign91430_e140125_d_n8, assign91430_e140125_d_n9, assign91430_e140125_d_n10, assign91430_e140125_d_n11, assign91430_e140125_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2137 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign91430_e140125;
        locals.var_xmp_dn0 = assign91430_e140125_d_n0;
        locals.var_xmp_dn2 = assign91430_e140125_d_n2;
        locals.var_xmp_dn4 = assign91430_e140125_d_n4;
        locals.var_xmp_dn5 = assign91430_e140125_d_n5;
        locals.var_xmp_dn6 = assign91430_e140125_d_n6;
        locals.var_xmp_dn7 = assign91430_e140125_d_n7;
        locals.var_xmp_dn8 = assign91430_e140125_d_n8;
        locals.var_xmp_dn9 = assign91430_e140125_d_n9;
        locals.var_xmp_dn10 = assign91430_e140125_d_n10;
        locals.var_xmp_dn11 = assign91430_e140125_d_n11;
        locals.var_xmp_dn14 = assign91430_e140125_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign91440_e140135,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2137 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign91440_e140135;
        locals.var_m0_rv = 0.0;

        let (assign91450_e140145,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2137 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign91450_e140145;
        locals.var_mm_rv = 0.0;

        let (assign91460_e140155, assign91460_e140155_d_n0, assign91460_e140155_d_n2, assign91460_e140155_d_n4, assign91460_e140155_d_n5, assign91460_e140155_d_n6, assign91460_e140155_d_n7, assign91460_e140155_d_n8, assign91460_e140155_d_n9, assign91460_e140155_d_n10, assign91460_e140155_d_n11, assign91460_e140155_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2137 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign91460_e140155;
        locals.var_arg_dn0 = assign91460_e140155_d_n0;
        locals.var_arg_dn2 = assign91460_e140155_d_n2;
        locals.var_arg_dn4 = assign91460_e140155_d_n4;
        locals.var_arg_dn5 = assign91460_e140155_d_n5;
        locals.var_arg_dn6 = assign91460_e140155_d_n6;
        locals.var_arg_dn7 = assign91460_e140155_d_n7;
        locals.var_arg_dn8 = assign91460_e140155_d_n8;
        locals.var_arg_dn9 = assign91460_e140155_d_n9;
        locals.var_arg_dn10 = assign91460_e140155_d_n10;
        locals.var_arg_dn11 = assign91460_e140155_d_n11;
        locals.var_arg_dn14 = assign91460_e140155_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign91470_e140165, assign91470_e140165_d_n0, assign91470_e140165_d_n2, assign91470_e140165_d_n4, assign91470_e140165_d_n5, assign91470_e140165_d_n6, assign91470_e140165_d_n7, assign91470_e140165_d_n8, assign91470_e140165_d_n9, assign91470_e140165_d_n10, assign91470_e140165_d_n11, assign91470_e140165_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2137 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign91470_e140165;
        locals.var_dnm_dn0 = assign91470_e140165_d_n0;
        locals.var_dnm_dn2 = assign91470_e140165_d_n2;
        locals.var_dnm_dn4 = assign91470_e140165_d_n4;
        locals.var_dnm_dn5 = assign91470_e140165_d_n5;
        locals.var_dnm_dn6 = assign91470_e140165_d_n6;
        locals.var_dnm_dn7 = assign91470_e140165_d_n7;
        locals.var_dnm_dn8 = assign91470_e140165_d_n8;
        locals.var_dnm_dn9 = assign91470_e140165_d_n9;
        locals.var_dnm_dn10 = assign91470_e140165_d_n10;
        locals.var_dnm_dn11 = assign91470_e140165_d_n11;
        locals.var_dnm_dn14 = assign91470_e140165_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign91480_e140177, assign91480_e140177_d_n0, assign91480_e140177_d_n2, assign91480_e140177_d_n4, assign91480_e140177_d_n5, assign91480_e140177_d_n6, assign91480_e140177_d_n7, assign91480_e140177_d_n8, assign91480_e140177_d_n9, assign91480_e140177_d_n10, assign91480_e140177_d_n11, assign91480_e140177_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2137 != 0.0)) {
        let assign91480_e140175: f64 = (locals.var_xp * locals.var_x2);
        (assign91480_e140175, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign91480_e140177;
        locals.var_xp_dn0 = assign91480_e140177_d_n0;
        locals.var_xp_dn2 = assign91480_e140177_d_n2;
        locals.var_xp_dn4 = assign91480_e140177_d_n4;
        locals.var_xp_dn5 = assign91480_e140177_d_n5;
        locals.var_xp_dn6 = assign91480_e140177_d_n6;
        locals.var_xp_dn7 = assign91480_e140177_d_n7;
        locals.var_xp_dn8 = assign91480_e140177_d_n8;
        locals.var_xp_dn9 = assign91480_e140177_d_n9;
        locals.var_xp_dn10 = assign91480_e140177_d_n10;
        locals.var_xp_dn11 = assign91480_e140177_d_n11;
        locals.var_xp_dn14 = assign91480_e140177_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign91490_e140189, assign91490_e140189_d_n0, assign91490_e140189_d_n2, assign91490_e140189_d_n4, assign91490_e140189_d_n5, assign91490_e140189_d_n6, assign91490_e140189_d_n7, assign91490_e140189_d_n8, assign91490_e140189_d_n9, assign91490_e140189_d_n10, assign91490_e140189_d_n11, assign91490_e140189_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2137 != 0.0)) {
        let assign91490_e140187: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign91490_e140187, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign91490_e140189;
        locals.var_xmp_dn0 = assign91490_e140189_d_n0;
        locals.var_xmp_dn2 = assign91490_e140189_d_n2;
        locals.var_xmp_dn4 = assign91490_e140189_d_n4;
        locals.var_xmp_dn5 = assign91490_e140189_d_n5;
        locals.var_xmp_dn6 = assign91490_e140189_d_n6;
        locals.var_xmp_dn7 = assign91490_e140189_d_n7;
        locals.var_xmp_dn8 = assign91490_e140189_d_n8;
        locals.var_xmp_dn9 = assign91490_e140189_d_n9;
        locals.var_xmp_dn10 = assign91490_e140189_d_n10;
        locals.var_xmp_dn11 = assign91490_e140189_d_n11;
        locals.var_xmp_dn14 = assign91490_e140189_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign91500_e140201, assign91500_e140201_d_n0, assign91500_e140201_d_n2, assign91500_e140201_d_n4, assign91500_e140201_d_n5, assign91500_e140201_d_n6, assign91500_e140201_d_n7, assign91500_e140201_d_n8, assign91500_e140201_d_n9, assign91500_e140201_d_n10, assign91500_e140201_d_n11, assign91500_e140201_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2137 != 0.0)) {
        let assign91500_e140199: f64 = (locals.var_xp + locals.var_xmp);
        (assign91500_e140199, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign91500_e140201;
        locals.var_arg_dn0 = assign91500_e140201_d_n0;
        locals.var_arg_dn2 = assign91500_e140201_d_n2;
        locals.var_arg_dn4 = assign91500_e140201_d_n4;
        locals.var_arg_dn5 = assign91500_e140201_d_n5;
        locals.var_arg_dn6 = assign91500_e140201_d_n6;
        locals.var_arg_dn7 = assign91500_e140201_d_n7;
        locals.var_arg_dn8 = assign91500_e140201_d_n8;
        locals.var_arg_dn9 = assign91500_e140201_d_n9;
        locals.var_arg_dn10 = assign91500_e140201_d_n10;
        locals.var_arg_dn11 = assign91500_e140201_d_n11;
        locals.var_arg_dn14 = assign91500_e140201_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign91510_e140211, assign91510_e140211_d_n0, assign91510_e140211_d_n2, assign91510_e140211_d_n4, assign91510_e140211_d_n5, assign91510_e140211_d_n6, assign91510_e140211_d_n7, assign91510_e140211_d_n8, assign91510_e140211_d_n9, assign91510_e140211_d_n10, assign91510_e140211_d_n11, assign91510_e140211_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2137 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign91510_e140211;
        locals.var_dnm_dn0 = assign91510_e140211_d_n0;
        locals.var_dnm_dn2 = assign91510_e140211_d_n2;
        locals.var_dnm_dn4 = assign91510_e140211_d_n4;
        locals.var_dnm_dn5 = assign91510_e140211_d_n5;
        locals.var_dnm_dn6 = assign91510_e140211_d_n6;
        locals.var_dnm_dn7 = assign91510_e140211_d_n7;
        locals.var_dnm_dn8 = assign91510_e140211_d_n8;
        locals.var_dnm_dn9 = assign91510_e140211_d_n9;
        locals.var_dnm_dn10 = assign91510_e140211_d_n10;
        locals.var_dnm_dn11 = assign91510_e140211_d_n11;
        locals.var_dnm_dn14 = assign91510_e140211_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign91520_e140226: f64 = if ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard2138 = assign91520_e140226;
        locals.var_guard2138_rv = 0.0;

        let assign91530_e140229: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2139 = assign91530_e140229;
        locals.var_guard2139_rv = 0.0;

        let (assign91540_e140243,) = {
    if ((((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2137 != 0.0)) && (locals.var_guard2138 != 0.0)) && (locals.var_guard2139 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign91540_e140243;
        locals.var_mm_rv = 0.0;

        let assign91550_e140246: f64 = if 1.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard2140 = assign91550_e140246;
        locals.var_guard2140_rv = 0.0;

        let (assign91560_e140263,) = {
    if (((((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2137 != 0.0)) && (locals.var_guard2138 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2140 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign91560_e140263;
        locals.var_mm_rv = 0.0;

        let assign91570_e140266: f64 = if 1.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard2141 = assign91570_e140266;
        locals.var_guard2141_rv = 0.0;

        let (assign91580_e140286,) = {
    if ((((((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2137 != 0.0)) && (locals.var_guard2138 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2140 == 0.0)) && (locals.var_guard2141 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign91580_e140286;
        locals.var_mm_rv = 0.0;

        let assign91590_e140289: f64 = if 1.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard2142 = assign91590_e140289;
        locals.var_guard2142_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_353(
        locals: &mut StampLocals,
    ) {
        let (assign91600_e140312,) = {
    if (((((((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2137 != 0.0)) && (locals.var_guard2138 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2140 == 0.0)) && (locals.var_guard2141 == 0.0)) && (locals.var_guard2142 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign91600_e140312;
        locals.var_mm_rv = 0.0;

        let (assign91610_e140324,) = {
    if (((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2137 != 0.0)) && (locals.var_guard2138 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign91610_e140324;
        locals.var_m0_rv = 0.0;

        let mut assign91620_loop_guard: usize = 0;
        while {
            let assign91620_cond_e140337: f64 = if ((((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2137 != 0.0)) && (locals.var_guard2138 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign91620_cond_e140337 != 0.0
        } {
            assign91620_loop_guard += 1;
            assert!(assign91620_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign91620_body0_e140350, assign91620_body0_e140350_d_n0, assign91620_body0_e140350_d_n2, assign91620_body0_e140350_d_n4, assign91620_body0_e140350_d_n5, assign91620_body0_e140350_d_n6, assign91620_body0_e140350_d_n7, assign91620_body0_e140350_d_n8, assign91620_body0_e140350_d_n9, assign91620_body0_e140350_d_n10, assign91620_body0_e140350_d_n11, assign91620_body0_e140350_d_n14,) = {
    if (((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2137 != 0.0)) && (locals.var_guard2138 != 0.0)) {
        let assign91620_body0_e140348: f64 = (locals.var_dnm).sqrt();
        (assign91620_body0_e140348, (locals.var_dnm_dn0 / (2.0 * assign91620_body0_e140348)), (locals.var_dnm_dn2 / (2.0 * assign91620_body0_e140348)), (locals.var_dnm_dn4 / (2.0 * assign91620_body0_e140348)), (locals.var_dnm_dn5 / (2.0 * assign91620_body0_e140348)), (locals.var_dnm_dn6 / (2.0 * assign91620_body0_e140348)), (locals.var_dnm_dn7 / (2.0 * assign91620_body0_e140348)), (locals.var_dnm_dn8 / (2.0 * assign91620_body0_e140348)), (locals.var_dnm_dn9 / (2.0 * assign91620_body0_e140348)), (locals.var_dnm_dn10 / (2.0 * assign91620_body0_e140348)), (locals.var_dnm_dn11 / (2.0 * assign91620_body0_e140348)), (locals.var_dnm_dn14 / (2.0 * assign91620_body0_e140348)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign91620_body0_e140350;
            locals.var_dnm_dn0 = assign91620_body0_e140350_d_n0;
            locals.var_dnm_dn2 = assign91620_body0_e140350_d_n2;
            locals.var_dnm_dn4 = assign91620_body0_e140350_d_n4;
            locals.var_dnm_dn5 = assign91620_body0_e140350_d_n5;
            locals.var_dnm_dn6 = assign91620_body0_e140350_d_n6;
            locals.var_dnm_dn7 = assign91620_body0_e140350_d_n7;
            locals.var_dnm_dn8 = assign91620_body0_e140350_d_n8;
            locals.var_dnm_dn9 = assign91620_body0_e140350_d_n9;
            locals.var_dnm_dn10 = assign91620_body0_e140350_d_n10;
            locals.var_dnm_dn11 = assign91620_body0_e140350_d_n11;
            locals.var_dnm_dn14 = assign91620_body0_e140350_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign91620_body1_e140364,) = {
    if (((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2137 != 0.0)) && (locals.var_guard2138 != 0.0)) {
        let assign91620_body1_e140362: f64 = (locals.var_m0 + 1.0);
        (assign91620_body1_e140362,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign91620_body1_e140364;
            locals.var_m0_rv = 0.0;
        }

        let (assign91630_e140388, assign91630_e140388_d_n0, assign91630_e140388_d_n2, assign91630_e140388_d_n4, assign91630_e140388_d_n5, assign91630_e140388_d_n6, assign91630_e140388_d_n7, assign91630_e140388_d_n8, assign91630_e140388_d_n9, assign91630_e140388_d_n10, assign91630_e140388_d_n11, assign91630_e140388_d_n14,) = {
    if (((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2137 != 0.0)) && (locals.var_guard2138 == 0.0)) {
        let (assign91630_e140386, assign91630_e140386_d_n0, assign91630_e140386_d_n2, assign91630_e140386_d_n4, assign91630_e140386_d_n5, assign91630_e140386_d_n6, assign91630_e140386_d_n7, assign91630_e140386_d_n8, assign91630_e140386_d_n9, assign91630_e140386_d_n10, assign91630_e140386_d_n11, assign91630_e140386_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign91630_e140383: f64 = 2.0;
                let assign91630_e140384: f64 = (1.0 / assign91630_e140383);
                let assign91630_e140385: f64 = (locals.var_dnm).powf(assign91630_e140384);
                (assign91630_e140385, if 0.0 == 0.0 && ((assign91630_e140384) as f64).is_finite() && ((assign91630_e140384) as f64).fract() == 0.0 { if assign91630_e140384 == 0.0 { 0.0 } else { (assign91630_e140384 * ((locals.var_dnm).powf(assign91630_e140384 - 1.0) * locals.var_dnm_dn0)) } } else { (assign91630_e140385 * (assign91630_e140384 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign91630_e140384) as f64).is_finite() && ((assign91630_e140384) as f64).fract() == 0.0 { if assign91630_e140384 == 0.0 { 0.0 } else { (assign91630_e140384 * ((locals.var_dnm).powf(assign91630_e140384 - 1.0) * locals.var_dnm_dn2)) } } else { (assign91630_e140385 * (assign91630_e140384 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign91630_e140384) as f64).is_finite() && ((assign91630_e140384) as f64).fract() == 0.0 { if assign91630_e140384 == 0.0 { 0.0 } else { (assign91630_e140384 * ((locals.var_dnm).powf(assign91630_e140384 - 1.0) * locals.var_dnm_dn4)) } } else { (assign91630_e140385 * (assign91630_e140384 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign91630_e140384) as f64).is_finite() && ((assign91630_e140384) as f64).fract() == 0.0 { if assign91630_e140384 == 0.0 { 0.0 } else { (assign91630_e140384 * ((locals.var_dnm).powf(assign91630_e140384 - 1.0) * locals.var_dnm_dn5)) } } else { (assign91630_e140385 * (assign91630_e140384 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign91630_e140384) as f64).is_finite() && ((assign91630_e140384) as f64).fract() == 0.0 { if assign91630_e140384 == 0.0 { 0.0 } else { (assign91630_e140384 * ((locals.var_dnm).powf(assign91630_e140384 - 1.0) * locals.var_dnm_dn6)) } } else { (assign91630_e140385 * (assign91630_e140384 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign91630_e140384) as f64).is_finite() && ((assign91630_e140384) as f64).fract() == 0.0 { if assign91630_e140384 == 0.0 { 0.0 } else { (assign91630_e140384 * ((locals.var_dnm).powf(assign91630_e140384 - 1.0) * locals.var_dnm_dn7)) } } else { (assign91630_e140385 * (assign91630_e140384 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign91630_e140384) as f64).is_finite() && ((assign91630_e140384) as f64).fract() == 0.0 { if assign91630_e140384 == 0.0 { 0.0 } else { (assign91630_e140384 * ((locals.var_dnm).powf(assign91630_e140384 - 1.0) * locals.var_dnm_dn8)) } } else { (assign91630_e140385 * (assign91630_e140384 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign91630_e140384) as f64).is_finite() && ((assign91630_e140384) as f64).fract() == 0.0 { if assign91630_e140384 == 0.0 { 0.0 } else { (assign91630_e140384 * ((locals.var_dnm).powf(assign91630_e140384 - 1.0) * locals.var_dnm_dn9)) } } else { (assign91630_e140385 * (assign91630_e140384 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign91630_e140384) as f64).is_finite() && ((assign91630_e140384) as f64).fract() == 0.0 { if assign91630_e140384 == 0.0 { 0.0 } else { (assign91630_e140384 * ((locals.var_dnm).powf(assign91630_e140384 - 1.0) * locals.var_dnm_dn10)) } } else { (assign91630_e140385 * (assign91630_e140384 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign91630_e140384) as f64).is_finite() && ((assign91630_e140384) as f64).fract() == 0.0 { if assign91630_e140384 == 0.0 { 0.0 } else { (assign91630_e140384 * ((locals.var_dnm).powf(assign91630_e140384 - 1.0) * locals.var_dnm_dn11)) } } else { (assign91630_e140385 * (assign91630_e140384 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign91630_e140384) as f64).is_finite() && ((assign91630_e140384) as f64).fract() == 0.0 { if assign91630_e140384 == 0.0 { 0.0 } else { (assign91630_e140384 * ((locals.var_dnm).powf(assign91630_e140384 - 1.0) * locals.var_dnm_dn14)) } } else { (assign91630_e140385 * (assign91630_e140384 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign91630_e140386, assign91630_e140386_d_n0, assign91630_e140386_d_n2, assign91630_e140386_d_n4, assign91630_e140386_d_n5, assign91630_e140386_d_n6, assign91630_e140386_d_n7, assign91630_e140386_d_n8, assign91630_e140386_d_n9, assign91630_e140386_d_n10, assign91630_e140386_d_n11, assign91630_e140386_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign91630_e140388;
        locals.var_dnm_dn0 = assign91630_e140388_d_n0;
        locals.var_dnm_dn2 = assign91630_e140388_d_n2;
        locals.var_dnm_dn4 = assign91630_e140388_d_n4;
        locals.var_dnm_dn5 = assign91630_e140388_d_n5;
        locals.var_dnm_dn6 = assign91630_e140388_d_n6;
        locals.var_dnm_dn7 = assign91630_e140388_d_n7;
        locals.var_dnm_dn8 = assign91630_e140388_d_n8;
        locals.var_dnm_dn9 = assign91630_e140388_d_n9;
        locals.var_dnm_dn10 = assign91630_e140388_d_n10;
        locals.var_dnm_dn11 = assign91630_e140388_d_n11;
        locals.var_dnm_dn14 = assign91630_e140388_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign91640_e140400, assign91640_e140400_d_n0, assign91640_e140400_d_n2, assign91640_e140400_d_n4, assign91640_e140400_d_n5, assign91640_e140400_d_n6, assign91640_e140400_d_n7, assign91640_e140400_d_n8, assign91640_e140400_d_n9, assign91640_e140400_d_n10, assign91640_e140400_d_n11, assign91640_e140400_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2137 != 0.0)) {
        let assign91640_e140398: f64 = (1.0 / locals.var_dnm);
        (assign91640_e140398, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign91640_e140400;
        locals.var_dnm_dn0 = assign91640_e140400_d_n0;
        locals.var_dnm_dn2 = assign91640_e140400_d_n2;
        locals.var_dnm_dn4 = assign91640_e140400_d_n4;
        locals.var_dnm_dn5 = assign91640_e140400_d_n5;
        locals.var_dnm_dn6 = assign91640_e140400_d_n6;
        locals.var_dnm_dn7 = assign91640_e140400_d_n7;
        locals.var_dnm_dn8 = assign91640_e140400_d_n8;
        locals.var_dnm_dn9 = assign91640_e140400_d_n9;
        locals.var_dnm_dn10 = assign91640_e140400_d_n10;
        locals.var_dnm_dn11 = assign91640_e140400_d_n11;
        locals.var_dnm_dn14 = assign91640_e140400_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign91650_e140414, assign91650_e140414_d_n0, assign91650_e140414_d_n2, assign91650_e140414_d_n4, assign91650_e140414_d_n5, assign91650_e140414_d_n6, assign91650_e140414_d_n7, assign91650_e140414_d_n8, assign91650_e140414_d_n9, assign91650_e140414_d_n10, assign91650_e140414_d_n11, assign91650_e140414_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2137 != 0.0)) {
        let assign91650_e140410: f64 = (locals.var_tmf1 * locals.var_t1);
        let assign91650_e140412: f64 = (assign91650_e140410 * locals.var_dnm);
        (assign91650_e140412, ((((locals.var_tmf1_dn0 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn0)) * locals.var_dnm) + (assign91650_e140410 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn2)) * locals.var_dnm) + (assign91650_e140410 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn4 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn4)) * locals.var_dnm) + (assign91650_e140410 * locals.var_dnm_dn4)), ((((locals.var_tmf1_dn5 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn5)) * locals.var_dnm) + (assign91650_e140410 * locals.var_dnm_dn5)), ((((locals.var_tmf1_dn6 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn6)) * locals.var_dnm) + (assign91650_e140410 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn7)) * locals.var_dnm) + (assign91650_e140410 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn8 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn8)) * locals.var_dnm) + (assign91650_e140410 * locals.var_dnm_dn8)), ((((locals.var_tmf1_dn9 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn9)) * locals.var_dnm) + (assign91650_e140410 * locals.var_dnm_dn9)), ((((locals.var_tmf1_dn10 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn10)) * locals.var_dnm) + (assign91650_e140410 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn11 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn11)) * locals.var_dnm) + (assign91650_e140410 * locals.var_dnm_dn11)), ((((locals.var_tmf1_dn14 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn14)) * locals.var_dnm) + (assign91650_e140410 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign91650_e140414;
        locals.var_tmf0_dn0 = assign91650_e140414_d_n0;
        locals.var_tmf0_dn2 = assign91650_e140414_d_n2;
        locals.var_tmf0_dn4 = assign91650_e140414_d_n4;
        locals.var_tmf0_dn5 = assign91650_e140414_d_n5;
        locals.var_tmf0_dn6 = assign91650_e140414_d_n6;
        locals.var_tmf0_dn7 = assign91650_e140414_d_n7;
        locals.var_tmf0_dn8 = assign91650_e140414_d_n8;
        locals.var_tmf0_dn9 = assign91650_e140414_d_n9;
        locals.var_tmf0_dn10 = assign91650_e140414_d_n10;
        locals.var_tmf0_dn11 = assign91650_e140414_d_n11;
        locals.var_tmf0_dn14 = assign91650_e140414_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign91660_e140430, assign91660_e140430_d_n0, assign91660_e140430_d_n2, assign91660_e140430_d_n4, assign91660_e140430_d_n5, assign91660_e140430_d_n6, assign91660_e140430_d_n7, assign91660_e140430_d_n8, assign91660_e140430_d_n9, assign91660_e140430_d_n10, assign91660_e140430_d_n11, assign91660_e140430_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2137 != 0.0)) {
        let assign91660_e140424: f64 = (locals.var_t1 * locals.var_xmp);
        let assign91660_e140426: f64 = (assign91660_e140424 * locals.var_dnm);
        let assign91660_e140428: f64 = (assign91660_e140426 / locals.var_arg);
        (assign91660_e140428, (((((((locals.var_t1_dn0 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn0)) * locals.var_dnm) + (assign91660_e140424 * locals.var_dnm_dn0)) * locals.var_arg) - (assign91660_e140426 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn2 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn2)) * locals.var_dnm) + (assign91660_e140424 * locals.var_dnm_dn2)) * locals.var_arg) - (assign91660_e140426 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn4 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn4)) * locals.var_dnm) + (assign91660_e140424 * locals.var_dnm_dn4)) * locals.var_arg) - (assign91660_e140426 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn5 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn5)) * locals.var_dnm) + (assign91660_e140424 * locals.var_dnm_dn5)) * locals.var_arg) - (assign91660_e140426 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn6 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn6)) * locals.var_dnm) + (assign91660_e140424 * locals.var_dnm_dn6)) * locals.var_arg) - (assign91660_e140426 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn7 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn7)) * locals.var_dnm) + (assign91660_e140424 * locals.var_dnm_dn7)) * locals.var_arg) - (assign91660_e140426 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn8 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn8)) * locals.var_dnm) + (assign91660_e140424 * locals.var_dnm_dn8)) * locals.var_arg) - (assign91660_e140426 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn9 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn9)) * locals.var_dnm) + (assign91660_e140424 * locals.var_dnm_dn9)) * locals.var_arg) - (assign91660_e140426 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn10 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn10)) * locals.var_dnm) + (assign91660_e140424 * locals.var_dnm_dn10)) * locals.var_arg) - (assign91660_e140426 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn11 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn11)) * locals.var_dnm) + (assign91660_e140424 * locals.var_dnm_dn11)) * locals.var_arg) - (assign91660_e140426 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn14 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn14)) * locals.var_dnm) + (assign91660_e140424 * locals.var_dnm_dn14)) * locals.var_arg) - (assign91660_e140426 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign91660_e140430;
        locals.var_t0_dn0 = assign91660_e140430_d_n0;
        locals.var_t0_dn2 = assign91660_e140430_d_n2;
        locals.var_t0_dn4 = assign91660_e140430_d_n4;
        locals.var_t0_dn5 = assign91660_e140430_d_n5;
        locals.var_t0_dn6 = assign91660_e140430_d_n6;
        locals.var_t0_dn7 = assign91660_e140430_d_n7;
        locals.var_t0_dn8 = assign91660_e140430_d_n8;
        locals.var_t0_dn9 = assign91660_e140430_d_n9;
        locals.var_t0_dn10 = assign91660_e140430_d_n10;
        locals.var_t0_dn11 = assign91660_e140430_d_n11;
        locals.var_t0_dn14 = assign91660_e140430_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign91670_e140444, assign91670_e140444_d_n0, assign91670_e140444_d_n2, assign91670_e140444_d_n4, assign91670_e140444_d_n5, assign91670_e140444_d_n6, assign91670_e140444_d_n7, assign91670_e140444_d_n8, assign91670_e140444_d_n9, assign91670_e140444_d_n10, assign91670_e140444_d_n11, assign91670_e140444_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2137 != 0.0)) {
        let assign91670_e140440: f64 = (-locals.var_t1);
        let assign91670_e140442: f64 = (assign91670_e140440 + locals.var_tmf0);
        (assign91670_e140442, ((-locals.var_t1_dn0) + locals.var_tmf0_dn0), ((-locals.var_t1_dn2) + locals.var_tmf0_dn2), ((-locals.var_t1_dn4) + locals.var_tmf0_dn4), ((-locals.var_t1_dn5) + locals.var_tmf0_dn5), ((-locals.var_t1_dn6) + locals.var_tmf0_dn6), ((-locals.var_t1_dn7) + locals.var_tmf0_dn7), ((-locals.var_t1_dn8) + locals.var_tmf0_dn8), ((-locals.var_t1_dn9) + locals.var_tmf0_dn9), ((-locals.var_t1_dn10) + locals.var_tmf0_dn10), ((-locals.var_t1_dn11) + locals.var_tmf0_dn11), ((-locals.var_t1_dn14) + locals.var_tmf0_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign91670_e140444;
        locals.var_t1_dn0 = assign91670_e140444_d_n0;
        locals.var_t1_dn2 = assign91670_e140444_d_n2;
        locals.var_t1_dn4 = assign91670_e140444_d_n4;
        locals.var_t1_dn5 = assign91670_e140444_d_n5;
        locals.var_t1_dn6 = assign91670_e140444_d_n6;
        locals.var_t1_dn7 = assign91670_e140444_d_n7;
        locals.var_t1_dn8 = assign91670_e140444_d_n8;
        locals.var_t1_dn9 = assign91670_e140444_d_n9;
        locals.var_t1_dn10 = assign91670_e140444_d_n10;
        locals.var_t1_dn11 = assign91670_e140444_d_n11;
        locals.var_t1_dn14 = assign91670_e140444_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign91680_e140454, assign91680_e140454_d_n0, assign91680_e140454_d_n2, assign91680_e140454_d_n4, assign91680_e140454_d_n5, assign91680_e140454_d_n6, assign91680_e140454_d_n7, assign91680_e140454_d_n8, assign91680_e140454_d_n9, assign91680_e140454_d_n10, assign91680_e140454_d_n11, assign91680_e140454_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2137 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign91680_e140454;
        locals.var_t0_dn0 = assign91680_e140454_d_n0;
        locals.var_t0_dn2 = assign91680_e140454_d_n2;
        locals.var_t0_dn4 = assign91680_e140454_d_n4;
        locals.var_t0_dn5 = assign91680_e140454_d_n5;
        locals.var_t0_dn6 = assign91680_e140454_d_n6;
        locals.var_t0_dn7 = assign91680_e140454_d_n7;
        locals.var_t0_dn8 = assign91680_e140454_d_n8;
        locals.var_t0_dn9 = assign91680_e140454_d_n9;
        locals.var_t0_dn10 = assign91680_e140454_d_n10;
        locals.var_t0_dn11 = assign91680_e140454_d_n11;
        locals.var_t0_dn14 = assign91680_e140454_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign91690_e140467, assign91690_e140467_d_n0, assign91690_e140467_d_n2, assign91690_e140467_d_n4, assign91690_e140467_d_n5, assign91690_e140467_d_n6, assign91690_e140467_d_n7, assign91690_e140467_d_n8, assign91690_e140467_d_n9, assign91690_e140467_d_n10, assign91690_e140467_d_n11, assign91690_e140467_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2137 == 0.0)) {
        let assign91690_e140465: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        (assign91690_e140465, locals.var_vxbgmtcl_dn0, (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2), locals.var_vxbgmtcl_dn4, locals.var_vxbgmtcl_dn5, locals.var_vxbgmtcl_dn6, (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7), (locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8), (locals.var_vgpld_dn9 + locals.var_vxbgmtcl_dn9), locals.var_vxbgmtcl_dn10, locals.var_vxbgmtcl_dn11, locals.var_vxbgmtcl_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign91690_e140467;
        locals.var_t1_dn0 = assign91690_e140467_d_n0;
        locals.var_t1_dn2 = assign91690_e140467_d_n2;
        locals.var_t1_dn4 = assign91690_e140467_d_n4;
        locals.var_t1_dn5 = assign91690_e140467_d_n5;
        locals.var_t1_dn6 = assign91690_e140467_d_n6;
        locals.var_t1_dn7 = assign91690_e140467_d_n7;
        locals.var_t1_dn8 = assign91690_e140467_d_n8;
        locals.var_t1_dn9 = assign91690_e140467_d_n9;
        locals.var_t1_dn10 = assign91690_e140467_d_n10;
        locals.var_t1_dn11 = assign91690_e140467_d_n11;
        locals.var_t1_dn14 = assign91690_e140467_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign91700_e140478, assign91700_e140478_d_n0, assign91700_e140478_d_n2, assign91700_e140478_d_n4, assign91700_e140478_d_n5, assign91700_e140478_d_n6, assign91700_e140478_d_n7, assign91700_e140478_d_n8, assign91700_e140478_d_n9, assign91700_e140478_d_n10, assign91700_e140478_d_n11, assign91700_e140478_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2137 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign91700_e140478;
        locals.var_t0_dn0 = assign91700_e140478_d_n0;
        locals.var_t0_dn2 = assign91700_e140478_d_n2;
        locals.var_t0_dn4 = assign91700_e140478_d_n4;
        locals.var_t0_dn5 = assign91700_e140478_d_n5;
        locals.var_t0_dn6 = assign91700_e140478_d_n6;
        locals.var_t0_dn7 = assign91700_e140478_d_n7;
        locals.var_t0_dn8 = assign91700_e140478_d_n8;
        locals.var_t0_dn9 = assign91700_e140478_d_n9;
        locals.var_t0_dn10 = assign91700_e140478_d_n10;
        locals.var_t0_dn11 = assign91700_e140478_d_n11;
        locals.var_t0_dn14 = assign91700_e140478_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign91710_e140488, assign91710_e140488_d_n0, assign91710_e140488_d_n2, assign91710_e140488_d_n4, assign91710_e140488_d_n5, assign91710_e140488_d_n6, assign91710_e140488_d_n7, assign91710_e140488_d_n8, assign91710_e140488_d_n9, assign91710_e140488_d_n10, assign91710_e140488_d_n11, assign91710_e140488_d_n14,) = {
    if (((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign91710_e140486: f64 = (locals.var_t1 - locals.var_vgpld);
        (assign91710_e140486, locals.var_t1_dn0, (locals.var_t1_dn2 - locals.var_vgpld_dn2), locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, (locals.var_t1_dn7 - locals.var_vgpld_dn7), (locals.var_t1_dn8 - locals.var_vgpld_dn8), (locals.var_t1_dn9 - locals.var_vgpld_dn9), locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    } else {
        (locals.var_vxbgmtcl, locals.var_vxbgmtcl_dn0, locals.var_vxbgmtcl_dn2, locals.var_vxbgmtcl_dn4, locals.var_vxbgmtcl_dn5, locals.var_vxbgmtcl_dn6, locals.var_vxbgmtcl_dn7, locals.var_vxbgmtcl_dn8, locals.var_vxbgmtcl_dn9, locals.var_vxbgmtcl_dn10, locals.var_vxbgmtcl_dn11, locals.var_vxbgmtcl_dn14,)
    }
};
        locals.var_vxbgmtcl = assign91710_e140488;
        locals.var_vxbgmtcl_dn0 = assign91710_e140488_d_n0;
        locals.var_vxbgmtcl_dn2 = assign91710_e140488_d_n2;
        locals.var_vxbgmtcl_dn4 = assign91710_e140488_d_n4;
        locals.var_vxbgmtcl_dn5 = assign91710_e140488_d_n5;
        locals.var_vxbgmtcl_dn6 = assign91710_e140488_d_n6;
        locals.var_vxbgmtcl_dn7 = assign91710_e140488_d_n7;
        locals.var_vxbgmtcl_dn8 = assign91710_e140488_d_n8;
        locals.var_vxbgmtcl_dn9 = assign91710_e140488_d_n9;
        locals.var_vxbgmtcl_dn10 = assign91710_e140488_d_n10;
        locals.var_vxbgmtcl_dn11 = assign91710_e140488_d_n11;
        locals.var_vxbgmtcl_dn14 = assign91710_e140488_d_n14;
        locals.var_vxbgmtcl_rv = 0.0;

        let (assign91720_e140501, assign91720_e140501_d_n0, assign91720_e140501_d_n2, assign91720_e140501_d_n4, assign91720_e140501_d_n5, assign91720_e140501_d_n6, assign91720_e140501_d_n7, assign91720_e140501_d_n8, assign91720_e140501_d_n9, assign91720_e140501_d_n10, assign91720_e140501_d_n11, assign91720_e140501_d_n14,) = {
    if (((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign91720_e140495: f64 = (-locals.var_vxbgmtcl);
        let assign91720_e140498: f64 = (10.0 * 2.220446049250313e-16);
        let assign91720_e140499: f64 = (assign91720_e140495 + assign91720_e140498);
        (assign91720_e140499, (-locals.var_vxbgmtcl_dn0), (-locals.var_vxbgmtcl_dn2), (-locals.var_vxbgmtcl_dn4), (-locals.var_vxbgmtcl_dn5), (-locals.var_vxbgmtcl_dn6), (-locals.var_vxbgmtcl_dn7), (-locals.var_vxbgmtcl_dn8), (-locals.var_vxbgmtcl_dn9), (-locals.var_vxbgmtcl_dn10), (-locals.var_vxbgmtcl_dn11), (-locals.var_vxbgmtcl_dn14),)
    } else {
        (locals.var_vgb_fb_ld, locals.var_vgb_fb_ld_dn0, locals.var_vgb_fb_ld_dn2, locals.var_vgb_fb_ld_dn4, locals.var_vgb_fb_ld_dn5, locals.var_vgb_fb_ld_dn6, locals.var_vgb_fb_ld_dn7, locals.var_vgb_fb_ld_dn8, locals.var_vgb_fb_ld_dn9, locals.var_vgb_fb_ld_dn10, locals.var_vgb_fb_ld_dn11, locals.var_vgb_fb_ld_dn14,)
    }
};
        locals.var_vgb_fb_ld = assign91720_e140501;
        locals.var_vgb_fb_ld_dn0 = assign91720_e140501_d_n0;
        locals.var_vgb_fb_ld_dn2 = assign91720_e140501_d_n2;
        locals.var_vgb_fb_ld_dn4 = assign91720_e140501_d_n4;
        locals.var_vgb_fb_ld_dn5 = assign91720_e140501_d_n5;
        locals.var_vgb_fb_ld_dn6 = assign91720_e140501_d_n6;
        locals.var_vgb_fb_ld_dn7 = assign91720_e140501_d_n7;
        locals.var_vgb_fb_ld_dn8 = assign91720_e140501_d_n8;
        locals.var_vgb_fb_ld_dn9 = assign91720_e140501_d_n9;
        locals.var_vgb_fb_ld_dn10 = assign91720_e140501_d_n10;
        locals.var_vgb_fb_ld_dn11 = assign91720_e140501_d_n11;
        locals.var_vgb_fb_ld_dn14 = assign91720_e140501_d_n14;
        locals.var_vgb_fb_ld_rv = 0.0;

        let assign91730_e140504: f64 = if locals.var_vgpld < locals.var_vgb_fb_ld { 1.0 } else { 0.0 };
        locals.var_guard2143 = assign91730_e140504;
        locals.var_guard2143_rv = 0.0;

        let (assign91750_e140529, assign91750_e140529_d_n0, assign91750_e140529_d_n2, assign91750_e140529_d_n4, assign91750_e140529_d_n5, assign91750_e140529_d_n6, assign91750_e140529_d_n7, assign91750_e140529_d_n8, assign91750_e140529_d_n9, assign91750_e140529_d_n10, assign91750_e140529_d_n11, assign91750_e140529_d_n14,) = {
    if (((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 != 0.0)) {
        let assign91750_e140521: f64 = (2.0 * locals.var_beta_inv);
        let assign91750_e140523: f64 = (-locals.var_vgs_min);
        let assign91750_e140525: f64 = (assign91750_e140523 / locals.var_fac1);
        let assign91750_e140526: f64 = (assign91750_e140525).ln();
        let assign91750_e140527: f64 = (assign91750_e140521 * assign91750_e140526);
        (assign91750_e140527, (((2.0 * locals.var_beta_inv_dn0) * assign91750_e140526) + (assign91750_e140521 * ((-((assign91750_e140523 * locals.var_fac1_dn0) / (locals.var_fac1 * locals.var_fac1))) / assign91750_e140525))), (((2.0 * locals.var_beta_inv_dn2) * assign91750_e140526) + (assign91750_e140521 * ((-((assign91750_e140523 * locals.var_fac1_dn2) / (locals.var_fac1 * locals.var_fac1))) / assign91750_e140525))), (((2.0 * locals.var_beta_inv_dn4) * assign91750_e140526) + (assign91750_e140521 * ((-((assign91750_e140523 * locals.var_fac1_dn4) / (locals.var_fac1 * locals.var_fac1))) / assign91750_e140525))), (((2.0 * locals.var_beta_inv_dn5) * assign91750_e140526) + (assign91750_e140521 * ((-((assign91750_e140523 * locals.var_fac1_dn5) / (locals.var_fac1 * locals.var_fac1))) / assign91750_e140525))), (((2.0 * locals.var_beta_inv_dn6) * assign91750_e140526) + (assign91750_e140521 * ((-((assign91750_e140523 * locals.var_fac1_dn6) / (locals.var_fac1 * locals.var_fac1))) / assign91750_e140525))), (((2.0 * locals.var_beta_inv_dn7) * assign91750_e140526) + (assign91750_e140521 * ((-((assign91750_e140523 * locals.var_fac1_dn7) / (locals.var_fac1 * locals.var_fac1))) / assign91750_e140525))), (((2.0 * locals.var_beta_inv_dn8) * assign91750_e140526) + (assign91750_e140521 * ((-((assign91750_e140523 * locals.var_fac1_dn8) / (locals.var_fac1 * locals.var_fac1))) / assign91750_e140525))), (((2.0 * locals.var_beta_inv_dn9) * assign91750_e140526) + (assign91750_e140521 * ((-((assign91750_e140523 * locals.var_fac1_dn9) / (locals.var_fac1 * locals.var_fac1))) / assign91750_e140525))), (((2.0 * locals.var_beta_inv_dn10) * assign91750_e140526) + (assign91750_e140521 * ((-((assign91750_e140523 * locals.var_fac1_dn10) / (locals.var_fac1 * locals.var_fac1))) / assign91750_e140525))), (((2.0 * locals.var_beta_inv_dn11) * assign91750_e140526) + (assign91750_e140521 * ((-((assign91750_e140523 * locals.var_fac1_dn11) / (locals.var_fac1 * locals.var_fac1))) / assign91750_e140525))), (((2.0 * locals.var_beta_inv_dn14) * assign91750_e140526) + (assign91750_e140521 * ((-((assign91750_e140523 * locals.var_fac1_dn14) / (locals.var_fac1 * locals.var_fac1))) / assign91750_e140525))),)
    } else {
        (locals.var_ps0_min, locals.var_ps0_min_dn0, locals.var_ps0_min_dn2, locals.var_ps0_min_dn4, locals.var_ps0_min_dn5, locals.var_ps0_min_dn6, locals.var_ps0_min_dn7, locals.var_ps0_min_dn8, locals.var_ps0_min_dn9, locals.var_ps0_min_dn10, locals.var_ps0_min_dn11, locals.var_ps0_min_dn14,)
    }
};
        locals.var_ps0_min = assign91750_e140529;
        locals.var_ps0_min_dn0 = assign91750_e140529_d_n0;
        locals.var_ps0_min_dn2 = assign91750_e140529_d_n2;
        locals.var_ps0_min_dn4 = assign91750_e140529_d_n4;
        locals.var_ps0_min_dn5 = assign91750_e140529_d_n5;
        locals.var_ps0_min_dn6 = assign91750_e140529_d_n6;
        locals.var_ps0_min_dn7 = assign91750_e140529_d_n7;
        locals.var_ps0_min_dn8 = assign91750_e140529_d_n8;
        locals.var_ps0_min_dn9 = assign91750_e140529_d_n9;
        locals.var_ps0_min_dn10 = assign91750_e140529_d_n10;
        locals.var_ps0_min_dn11 = assign91750_e140529_d_n11;
        locals.var_ps0_min_dn14 = assign91750_e140529_d_n14;
        locals.var_ps0_min_rv = 0.0;

        let (assign91760_e140541, assign91760_e140541_d_n0, assign91760_e140541_d_n2, assign91760_e140541_d_n4, assign91760_e140541_d_n5, assign91760_e140541_d_n6, assign91760_e140541_d_n7, assign91760_e140541_d_n8, assign91760_e140541_d_n9, assign91760_e140541_d_n10, assign91760_e140541_d_n11, assign91760_e140541_d_n14,) = {
    if (((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 != 0.0)) {
        let assign91760_e140538: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign91760_e140539: f64 = (locals.var_beta * assign91760_e140538);
        (assign91760_e140539, ((locals.var_beta_dn0 * assign91760_e140538) + (locals.var_beta * locals.var_vxbgmtcl_dn0)), ((locals.var_beta_dn2 * assign91760_e140538) + (locals.var_beta * (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2))), ((locals.var_beta_dn4 * assign91760_e140538) + (locals.var_beta * locals.var_vxbgmtcl_dn4)), ((locals.var_beta_dn5 * assign91760_e140538) + (locals.var_beta * locals.var_vxbgmtcl_dn5)), ((locals.var_beta_dn6 * assign91760_e140538) + (locals.var_beta * locals.var_vxbgmtcl_dn6)), ((locals.var_beta_dn7 * assign91760_e140538) + (locals.var_beta * (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7))), ((locals.var_beta_dn8 * assign91760_e140538) + (locals.var_beta * (locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8))), ((locals.var_beta_dn9 * assign91760_e140538) + (locals.var_beta * (locals.var_vgpld_dn9 + locals.var_vxbgmtcl_dn9))), ((locals.var_beta_dn10 * assign91760_e140538) + (locals.var_beta * locals.var_vxbgmtcl_dn10)), ((locals.var_beta_dn11 * assign91760_e140538) + (locals.var_beta * locals.var_vxbgmtcl_dn11)), ((locals.var_beta_dn14 * assign91760_e140538) + (locals.var_beta * locals.var_vxbgmtcl_dn14)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign91760_e140541;
        locals.var_tx_dn0 = assign91760_e140541_d_n0;
        locals.var_tx_dn2 = assign91760_e140541_d_n2;
        locals.var_tx_dn4 = assign91760_e140541_d_n4;
        locals.var_tx_dn5 = assign91760_e140541_d_n5;
        locals.var_tx_dn6 = assign91760_e140541_d_n6;
        locals.var_tx_dn7 = assign91760_e140541_d_n7;
        locals.var_tx_dn8 = assign91760_e140541_d_n8;
        locals.var_tx_dn9 = assign91760_e140541_d_n9;
        locals.var_tx_dn10 = assign91760_e140541_d_n10;
        locals.var_tx_dn11 = assign91760_e140541_d_n11;
        locals.var_tx_dn14 = assign91760_e140541_d_n14;
        locals.var_tx_rv = 0.0;

        let (assign91770_e140553, assign91770_e140553_d_n0, assign91770_e140553_d_n2, assign91770_e140553_d_n4, assign91770_e140553_d_n5, assign91770_e140553_d_n6, assign91770_e140553_d_n7, assign91770_e140553_d_n8, assign91770_e140553_d_n9, assign91770_e140553_d_n10, assign91770_e140553_d_n11, assign91770_e140553_d_n14,) = {
    if (((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 != 0.0)) {
        let assign91770_e140550: f64 = (locals.var_beta * locals.var_cnst0over_func);
        let assign91770_e140551: f64 = (1.0 / assign91770_e140550);
        (assign91770_e140551, (-(((locals.var_beta_dn0 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn0)) / (assign91770_e140550 * assign91770_e140550))), (-(((locals.var_beta_dn2 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn2)) / (assign91770_e140550 * assign91770_e140550))), (-(((locals.var_beta_dn4 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn4)) / (assign91770_e140550 * assign91770_e140550))), (-(((locals.var_beta_dn5 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn5)) / (assign91770_e140550 * assign91770_e140550))), (-(((locals.var_beta_dn6 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn6)) / (assign91770_e140550 * assign91770_e140550))), (-(((locals.var_beta_dn7 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn7)) / (assign91770_e140550 * assign91770_e140550))), (-(((locals.var_beta_dn8 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn8)) / (assign91770_e140550 * assign91770_e140550))), (-(((locals.var_beta_dn9 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn9)) / (assign91770_e140550 * assign91770_e140550))), (-(((locals.var_beta_dn10 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn10)) / (assign91770_e140550 * assign91770_e140550))), (-(((locals.var_beta_dn11 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn11)) / (assign91770_e140550 * assign91770_e140550))), (-(((locals.var_beta_dn14 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn14)) / (assign91770_e140550 * assign91770_e140550))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign91770_e140553;
        locals.var_t1_dn0 = assign91770_e140553_d_n0;
        locals.var_t1_dn2 = assign91770_e140553_d_n2;
        locals.var_t1_dn4 = assign91770_e140553_d_n4;
        locals.var_t1_dn5 = assign91770_e140553_d_n5;
        locals.var_t1_dn6 = assign91770_e140553_d_n6;
        locals.var_t1_dn7 = assign91770_e140553_d_n7;
        locals.var_t1_dn8 = assign91770_e140553_d_n8;
        locals.var_t1_dn9 = assign91770_e140553_d_n9;
        locals.var_t1_dn10 = assign91770_e140553_d_n10;
        locals.var_t1_dn11 = assign91770_e140553_d_n11;
        locals.var_t1_dn14 = assign91770_e140553_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign91780_e140563, assign91780_e140563_d_n0, assign91780_e140563_d_n2, assign91780_e140563_d_n4, assign91780_e140563_d_n5, assign91780_e140563_d_n6, assign91780_e140563_d_n7, assign91780_e140563_d_n8, assign91780_e140563_d_n9, assign91780_e140563_d_n10, assign91780_e140563_d_n11, assign91780_e140563_d_n14,) = {
    if (((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 != 0.0)) {
        let assign91780_e140561: f64 = (locals.var_t1 * locals.var_cox0_func);
        (assign91780_e140561, (locals.var_t1_dn0 * locals.var_cox0_func), (locals.var_t1_dn2 * locals.var_cox0_func), (locals.var_t1_dn4 * locals.var_cox0_func), (locals.var_t1_dn5 * locals.var_cox0_func), (locals.var_t1_dn6 * locals.var_cox0_func), (locals.var_t1_dn7 * locals.var_cox0_func), (locals.var_t1_dn8 * locals.var_cox0_func), (locals.var_t1_dn9 * locals.var_cox0_func), (locals.var_t1_dn10 * locals.var_cox0_func), (locals.var_t1_dn11 * locals.var_cox0_func), (locals.var_t1_dn14 * locals.var_cox0_func),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn14,)
    }
};
        locals.var_ty = assign91780_e140563;
        locals.var_ty_dn0 = assign91780_e140563_d_n0;
        locals.var_ty_dn2 = assign91780_e140563_d_n2;
        locals.var_ty_dn4 = assign91780_e140563_d_n4;
        locals.var_ty_dn5 = assign91780_e140563_d_n5;
        locals.var_ty_dn6 = assign91780_e140563_d_n6;
        locals.var_ty_dn7 = assign91780_e140563_d_n7;
        locals.var_ty_dn8 = assign91780_e140563_d_n8;
        locals.var_ty_dn9 = assign91780_e140563_d_n9;
        locals.var_ty_dn10 = assign91780_e140563_d_n10;
        locals.var_ty_dn11 = assign91780_e140563_d_n11;
        locals.var_ty_dn14 = assign91780_e140563_d_n14;
        locals.var_ty_rv = 0.0;

        let (assign91790_e140577, assign91790_e140577_d_n0, assign91790_e140577_d_n2, assign91790_e140577_d_n4, assign91790_e140577_d_n5, assign91790_e140577_d_n6, assign91790_e140577_d_n7, assign91790_e140577_d_n8, assign91790_e140577_d_n9, assign91790_e140577_d_n10, assign91790_e140577_d_n11, assign91790_e140577_d_n14,) = {
    if (((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 != 0.0)) {
        let assign91790_e140572: f64 = (3.0 * 1.414213562373095);
        let assign91790_e140574: f64 = (assign91790_e140572 * locals.var_ty);
        let assign91790_e140575: f64 = (2.0 + assign91790_e140574);
        (assign91790_e140575, (assign91790_e140572 * locals.var_ty_dn0), (assign91790_e140572 * locals.var_ty_dn2), (assign91790_e140572 * locals.var_ty_dn4), (assign91790_e140572 * locals.var_ty_dn5), (assign91790_e140572 * locals.var_ty_dn6), (assign91790_e140572 * locals.var_ty_dn7), (assign91790_e140572 * locals.var_ty_dn8), (assign91790_e140572 * locals.var_ty_dn9), (assign91790_e140572 * locals.var_ty_dn10), (assign91790_e140572 * locals.var_ty_dn11), (assign91790_e140572 * locals.var_ty_dn14),)
    } else {
        (locals.var_ac41, locals.var_ac41_dn0, locals.var_ac41_dn2, locals.var_ac41_dn4, locals.var_ac41_dn5, locals.var_ac41_dn6, locals.var_ac41_dn7, locals.var_ac41_dn8, locals.var_ac41_dn9, locals.var_ac41_dn10, locals.var_ac41_dn11, locals.var_ac41_dn14,)
    }
};
        locals.var_ac41 = assign91790_e140577;
        locals.var_ac41_dn0 = assign91790_e140577_d_n0;
        locals.var_ac41_dn2 = assign91790_e140577_d_n2;
        locals.var_ac41_dn4 = assign91790_e140577_d_n4;
        locals.var_ac41_dn5 = assign91790_e140577_d_n5;
        locals.var_ac41_dn6 = assign91790_e140577_d_n6;
        locals.var_ac41_dn7 = assign91790_e140577_d_n7;
        locals.var_ac41_dn8 = assign91790_e140577_d_n8;
        locals.var_ac41_dn9 = assign91790_e140577_d_n9;
        locals.var_ac41_dn10 = assign91790_e140577_d_n10;
        locals.var_ac41_dn11 = assign91790_e140577_d_n11;
        locals.var_ac41_dn14 = assign91790_e140577_d_n14;
        locals.var_ac41_rv = 0.0;

        let (assign91800_e140591, assign91800_e140591_d_n0, assign91800_e140591_d_n2, assign91800_e140591_d_n4, assign91800_e140591_d_n5, assign91800_e140591_d_n6, assign91800_e140591_d_n7, assign91800_e140591_d_n8, assign91800_e140591_d_n9, assign91800_e140591_d_n10, assign91800_e140591_d_n11, assign91800_e140591_d_n14,) = {
    if (((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 != 0.0)) {
        let assign91800_e140585: f64 = (8.0 * locals.var_ac41);
        let assign91800_e140587: f64 = (assign91800_e140585 * locals.var_ac41);
        let assign91800_e140589: f64 = (assign91800_e140587 * locals.var_ac41);
        (assign91800_e140589, (((((8.0 * locals.var_ac41_dn0) * locals.var_ac41) + (assign91800_e140585 * locals.var_ac41_dn0)) * locals.var_ac41) + (assign91800_e140587 * locals.var_ac41_dn0)), (((((8.0 * locals.var_ac41_dn2) * locals.var_ac41) + (assign91800_e140585 * locals.var_ac41_dn2)) * locals.var_ac41) + (assign91800_e140587 * locals.var_ac41_dn2)), (((((8.0 * locals.var_ac41_dn4) * locals.var_ac41) + (assign91800_e140585 * locals.var_ac41_dn4)) * locals.var_ac41) + (assign91800_e140587 * locals.var_ac41_dn4)), (((((8.0 * locals.var_ac41_dn5) * locals.var_ac41) + (assign91800_e140585 * locals.var_ac41_dn5)) * locals.var_ac41) + (assign91800_e140587 * locals.var_ac41_dn5)), (((((8.0 * locals.var_ac41_dn6) * locals.var_ac41) + (assign91800_e140585 * locals.var_ac41_dn6)) * locals.var_ac41) + (assign91800_e140587 * locals.var_ac41_dn6)), (((((8.0 * locals.var_ac41_dn7) * locals.var_ac41) + (assign91800_e140585 * locals.var_ac41_dn7)) * locals.var_ac41) + (assign91800_e140587 * locals.var_ac41_dn7)), (((((8.0 * locals.var_ac41_dn8) * locals.var_ac41) + (assign91800_e140585 * locals.var_ac41_dn8)) * locals.var_ac41) + (assign91800_e140587 * locals.var_ac41_dn8)), (((((8.0 * locals.var_ac41_dn9) * locals.var_ac41) + (assign91800_e140585 * locals.var_ac41_dn9)) * locals.var_ac41) + (assign91800_e140587 * locals.var_ac41_dn9)), (((((8.0 * locals.var_ac41_dn10) * locals.var_ac41) + (assign91800_e140585 * locals.var_ac41_dn10)) * locals.var_ac41) + (assign91800_e140587 * locals.var_ac41_dn10)), (((((8.0 * locals.var_ac41_dn11) * locals.var_ac41) + (assign91800_e140585 * locals.var_ac41_dn11)) * locals.var_ac41) + (assign91800_e140587 * locals.var_ac41_dn11)), (((((8.0 * locals.var_ac41_dn14) * locals.var_ac41) + (assign91800_e140585 * locals.var_ac41_dn14)) * locals.var_ac41) + (assign91800_e140587 * locals.var_ac41_dn14)),)
    } else {
        (locals.var_ac4, locals.var_ac4_dn0, locals.var_ac4_dn2, locals.var_ac4_dn4, locals.var_ac4_dn5, locals.var_ac4_dn6, locals.var_ac4_dn7, locals.var_ac4_dn8, locals.var_ac4_dn9, locals.var_ac4_dn10, locals.var_ac4_dn11, locals.var_ac4_dn14,)
    }
};
        locals.var_ac4 = assign91800_e140591;
        locals.var_ac4_dn0 = assign91800_e140591_d_n0;
        locals.var_ac4_dn2 = assign91800_e140591_d_n2;
        locals.var_ac4_dn4 = assign91800_e140591_d_n4;
        locals.var_ac4_dn5 = assign91800_e140591_d_n5;
        locals.var_ac4_dn6 = assign91800_e140591_d_n6;
        locals.var_ac4_dn7 = assign91800_e140591_d_n7;
        locals.var_ac4_dn8 = assign91800_e140591_d_n8;
        locals.var_ac4_dn9 = assign91800_e140591_d_n9;
        locals.var_ac4_dn10 = assign91800_e140591_d_n10;
        locals.var_ac4_dn11 = assign91800_e140591_d_n11;
        locals.var_ac4_dn14 = assign91800_e140591_d_n14;
        locals.var_ac4_rv = 0.0;

        let (assign91810_e140609, assign91810_e140609_d_n0, assign91810_e140609_d_n2, assign91810_e140609_d_n4, assign91810_e140609_d_n5, assign91810_e140609_d_n6, assign91810_e140609_d_n7, assign91810_e140609_d_n8, assign91810_e140609_d_n9, assign91810_e140609_d_n10, assign91810_e140609_d_n11, assign91810_e140609_d_n14,) = {
    if (((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 != 0.0)) {
        let assign91810_e140599: f64 = (7.0 * 1.414213562373095);
        let assign91810_e140602: f64 = (9.0 * locals.var_ty);
        let assign91810_e140605: f64 = (locals.var_tx - 2.0);
        let assign91810_e140606: f64 = (assign91810_e140602 * assign91810_e140605);
        let assign91810_e140607: f64 = (assign91810_e140599 - assign91810_e140606);
        (assign91810_e140607, (-(((9.0 * locals.var_ty_dn0) * assign91810_e140605) + (assign91810_e140602 * locals.var_tx_dn0))), (-(((9.0 * locals.var_ty_dn2) * assign91810_e140605) + (assign91810_e140602 * locals.var_tx_dn2))), (-(((9.0 * locals.var_ty_dn4) * assign91810_e140605) + (assign91810_e140602 * locals.var_tx_dn4))), (-(((9.0 * locals.var_ty_dn5) * assign91810_e140605) + (assign91810_e140602 * locals.var_tx_dn5))), (-(((9.0 * locals.var_ty_dn6) * assign91810_e140605) + (assign91810_e140602 * locals.var_tx_dn6))), (-(((9.0 * locals.var_ty_dn7) * assign91810_e140605) + (assign91810_e140602 * locals.var_tx_dn7))), (-(((9.0 * locals.var_ty_dn8) * assign91810_e140605) + (assign91810_e140602 * locals.var_tx_dn8))), (-(((9.0 * locals.var_ty_dn9) * assign91810_e140605) + (assign91810_e140602 * locals.var_tx_dn9))), (-(((9.0 * locals.var_ty_dn10) * assign91810_e140605) + (assign91810_e140602 * locals.var_tx_dn10))), (-(((9.0 * locals.var_ty_dn11) * assign91810_e140605) + (assign91810_e140602 * locals.var_tx_dn11))), (-(((9.0 * locals.var_ty_dn14) * assign91810_e140605) + (assign91810_e140602 * locals.var_tx_dn14))),)
    } else {
        (locals.var_ac31, locals.var_ac31_dn0, locals.var_ac31_dn2, locals.var_ac31_dn4, locals.var_ac31_dn5, locals.var_ac31_dn6, locals.var_ac31_dn7, locals.var_ac31_dn8, locals.var_ac31_dn9, locals.var_ac31_dn10, locals.var_ac31_dn11, locals.var_ac31_dn14,)
    }
};
        locals.var_ac31 = assign91810_e140609;
        locals.var_ac31_dn0 = assign91810_e140609_d_n0;
        locals.var_ac31_dn2 = assign91810_e140609_d_n2;
        locals.var_ac31_dn4 = assign91810_e140609_d_n4;
        locals.var_ac31_dn5 = assign91810_e140609_d_n5;
        locals.var_ac31_dn6 = assign91810_e140609_d_n6;
        locals.var_ac31_dn7 = assign91810_e140609_d_n7;
        locals.var_ac31_dn8 = assign91810_e140609_d_n8;
        locals.var_ac31_dn9 = assign91810_e140609_d_n9;
        locals.var_ac31_dn10 = assign91810_e140609_d_n10;
        locals.var_ac31_dn11 = assign91810_e140609_d_n11;
        locals.var_ac31_dn14 = assign91810_e140609_d_n14;
        locals.var_ac31_rv = 0.0;

        let (assign91820_e140619, assign91820_e140619_d_n0, assign91820_e140619_d_n2, assign91820_e140619_d_n4, assign91820_e140619_d_n5, assign91820_e140619_d_n6, assign91820_e140619_d_n7, assign91820_e140619_d_n8, assign91820_e140619_d_n9, assign91820_e140619_d_n10, assign91820_e140619_d_n11, assign91820_e140619_d_n14,) = {
    if (((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 != 0.0)) {
        let assign91820_e140617: f64 = (locals.var_ac31 * locals.var_ac31);
        (assign91820_e140617, ((locals.var_ac31_dn0 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn0)), ((locals.var_ac31_dn2 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn2)), ((locals.var_ac31_dn4 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn4)), ((locals.var_ac31_dn5 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn5)), ((locals.var_ac31_dn6 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn6)), ((locals.var_ac31_dn7 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn7)), ((locals.var_ac31_dn8 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn8)), ((locals.var_ac31_dn9 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn9)), ((locals.var_ac31_dn10 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn10)), ((locals.var_ac31_dn11 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn11)), ((locals.var_ac31_dn14 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn14)),)
    } else {
        (locals.var_ac3, locals.var_ac3_dn0, locals.var_ac3_dn2, locals.var_ac3_dn4, locals.var_ac3_dn5, locals.var_ac3_dn6, locals.var_ac3_dn7, locals.var_ac3_dn8, locals.var_ac3_dn9, locals.var_ac3_dn10, locals.var_ac3_dn11, locals.var_ac3_dn14,)
    }
};
        locals.var_ac3 = assign91820_e140619;
        locals.var_ac3_dn0 = assign91820_e140619_d_n0;
        locals.var_ac3_dn2 = assign91820_e140619_d_n2;
        locals.var_ac3_dn4 = assign91820_e140619_d_n4;
        locals.var_ac3_dn5 = assign91820_e140619_d_n5;
        locals.var_ac3_dn6 = assign91820_e140619_d_n6;
        locals.var_ac3_dn7 = assign91820_e140619_d_n7;
        locals.var_ac3_dn8 = assign91820_e140619_d_n8;
        locals.var_ac3_dn9 = assign91820_e140619_d_n9;
        locals.var_ac3_dn10 = assign91820_e140619_d_n10;
        locals.var_ac3_dn11 = assign91820_e140619_d_n11;
        locals.var_ac3_dn14 = assign91820_e140619_d_n14;
        locals.var_ac3_rv = 0.0;

        let assign91830_e140623: f64 = (locals.var_ac3 * 1e-8);
        let assign91830_e140624: f64 = if locals.var_ac4 < assign91830_e140623 { 1.0 } else { 0.0 };
        locals.var_guard2144 = assign91830_e140624;
        locals.var_guard2144_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_354(
        locals: &mut StampLocals,
    ) {
        let (assign91850_e140649, assign91850_e140649_d_n0, assign91850_e140649_d_n2, assign91850_e140649_d_n4, assign91850_e140649_d_n5, assign91850_e140649_d_n6, assign91850_e140649_d_n7, assign91850_e140649_d_n8, assign91850_e140649_d_n9, assign91850_e140649_d_n10, assign91850_e140649_d_n11, assign91850_e140649_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 != 0.0)) && (locals.var_guard2144 != 0.0)) {
        let assign91850_e140645: f64 = (0.5 * locals.var_ac4);
        let assign91850_e140647: f64 = (assign91850_e140645 / locals.var_ac31);
        (assign91850_e140647, ((((0.5 * locals.var_ac4_dn0) * locals.var_ac31) - (assign91850_e140645 * locals.var_ac31_dn0)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn2) * locals.var_ac31) - (assign91850_e140645 * locals.var_ac31_dn2)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn4) * locals.var_ac31) - (assign91850_e140645 * locals.var_ac31_dn4)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn5) * locals.var_ac31) - (assign91850_e140645 * locals.var_ac31_dn5)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn6) * locals.var_ac31) - (assign91850_e140645 * locals.var_ac31_dn6)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn7) * locals.var_ac31) - (assign91850_e140645 * locals.var_ac31_dn7)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn8) * locals.var_ac31) - (assign91850_e140645 * locals.var_ac31_dn8)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn9) * locals.var_ac31) - (assign91850_e140645 * locals.var_ac31_dn9)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn10) * locals.var_ac31) - (assign91850_e140645 * locals.var_ac31_dn10)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn11) * locals.var_ac31) - (assign91850_e140645 * locals.var_ac31_dn11)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn14) * locals.var_ac31) - (assign91850_e140645 * locals.var_ac31_dn14)) / (locals.var_ac31 * locals.var_ac31)),)
    } else {
        (locals.var_ac1, locals.var_ac1_dn0, locals.var_ac1_dn2, locals.var_ac1_dn4, locals.var_ac1_dn5, locals.var_ac1_dn6, locals.var_ac1_dn7, locals.var_ac1_dn8, locals.var_ac1_dn9, locals.var_ac1_dn10, locals.var_ac1_dn11, locals.var_ac1_dn14,)
    }
};
        locals.var_ac1 = assign91850_e140649;
        locals.var_ac1_dn0 = assign91850_e140649_d_n0;
        locals.var_ac1_dn2 = assign91850_e140649_d_n2;
        locals.var_ac1_dn4 = assign91850_e140649_d_n4;
        locals.var_ac1_dn5 = assign91850_e140649_d_n5;
        locals.var_ac1_dn6 = assign91850_e140649_d_n6;
        locals.var_ac1_dn7 = assign91850_e140649_d_n7;
        locals.var_ac1_dn8 = assign91850_e140649_d_n8;
        locals.var_ac1_dn9 = assign91850_e140649_d_n9;
        locals.var_ac1_dn10 = assign91850_e140649_d_n10;
        locals.var_ac1_dn11 = assign91850_e140649_d_n11;
        locals.var_ac1_dn14 = assign91850_e140649_d_n14;
        locals.var_ac1_rv = 0.0;

        let (assign91860_e140663, assign91860_e140663_d_n0, assign91860_e140663_d_n2, assign91860_e140663_d_n4, assign91860_e140663_d_n5, assign91860_e140663_d_n6, assign91860_e140663_d_n7, assign91860_e140663_d_n8, assign91860_e140663_d_n9, assign91860_e140663_d_n10, assign91860_e140663_d_n11, assign91860_e140663_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 != 0.0)) && (locals.var_guard2144 == 0.0)) {
        let assign91860_e140660: f64 = (locals.var_ac4 + locals.var_ac3);
        let assign91860_e140661: f64 = (assign91860_e140660).sqrt();
        (assign91860_e140661, ((locals.var_ac4_dn0 + locals.var_ac3_dn0) / (2.0 * assign91860_e140661)), ((locals.var_ac4_dn2 + locals.var_ac3_dn2) / (2.0 * assign91860_e140661)), ((locals.var_ac4_dn4 + locals.var_ac3_dn4) / (2.0 * assign91860_e140661)), ((locals.var_ac4_dn5 + locals.var_ac3_dn5) / (2.0 * assign91860_e140661)), ((locals.var_ac4_dn6 + locals.var_ac3_dn6) / (2.0 * assign91860_e140661)), ((locals.var_ac4_dn7 + locals.var_ac3_dn7) / (2.0 * assign91860_e140661)), ((locals.var_ac4_dn8 + locals.var_ac3_dn8) / (2.0 * assign91860_e140661)), ((locals.var_ac4_dn9 + locals.var_ac3_dn9) / (2.0 * assign91860_e140661)), ((locals.var_ac4_dn10 + locals.var_ac3_dn10) / (2.0 * assign91860_e140661)), ((locals.var_ac4_dn11 + locals.var_ac3_dn11) / (2.0 * assign91860_e140661)), ((locals.var_ac4_dn14 + locals.var_ac3_dn14) / (2.0 * assign91860_e140661)),)
    } else {
        (locals.var_ac2, locals.var_ac2_dn0, locals.var_ac2_dn2, locals.var_ac2_dn4, locals.var_ac2_dn5, locals.var_ac2_dn6, locals.var_ac2_dn7, locals.var_ac2_dn8, locals.var_ac2_dn9, locals.var_ac2_dn10, locals.var_ac2_dn11, locals.var_ac2_dn14,)
    }
};
        locals.var_ac2 = assign91860_e140663;
        locals.var_ac2_dn0 = assign91860_e140663_d_n0;
        locals.var_ac2_dn2 = assign91860_e140663_d_n2;
        locals.var_ac2_dn4 = assign91860_e140663_d_n4;
        locals.var_ac2_dn5 = assign91860_e140663_d_n5;
        locals.var_ac2_dn6 = assign91860_e140663_d_n6;
        locals.var_ac2_dn7 = assign91860_e140663_d_n7;
        locals.var_ac2_dn8 = assign91860_e140663_d_n8;
        locals.var_ac2_dn9 = assign91860_e140663_d_n9;
        locals.var_ac2_dn10 = assign91860_e140663_d_n10;
        locals.var_ac2_dn11 = assign91860_e140663_d_n11;
        locals.var_ac2_dn14 = assign91860_e140663_d_n14;
        locals.var_ac2_rv = 0.0;

        let (assign91870_e140677, assign91870_e140677_d_n0, assign91870_e140677_d_n2, assign91870_e140677_d_n4, assign91870_e140677_d_n5, assign91870_e140677_d_n6, assign91870_e140677_d_n7, assign91870_e140677_d_n8, assign91870_e140677_d_n9, assign91870_e140677_d_n10, assign91870_e140677_d_n11, assign91870_e140677_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 != 0.0)) && (locals.var_guard2144 == 0.0)) {
        let assign91870_e140673: f64 = (-locals.var_ac31);
        let assign91870_e140675: f64 = (assign91870_e140673 + locals.var_ac2);
        (assign91870_e140675, ((-locals.var_ac31_dn0) + locals.var_ac2_dn0), ((-locals.var_ac31_dn2) + locals.var_ac2_dn2), ((-locals.var_ac31_dn4) + locals.var_ac2_dn4), ((-locals.var_ac31_dn5) + locals.var_ac2_dn5), ((-locals.var_ac31_dn6) + locals.var_ac2_dn6), ((-locals.var_ac31_dn7) + locals.var_ac2_dn7), ((-locals.var_ac31_dn8) + locals.var_ac2_dn8), ((-locals.var_ac31_dn9) + locals.var_ac2_dn9), ((-locals.var_ac31_dn10) + locals.var_ac2_dn10), ((-locals.var_ac31_dn11) + locals.var_ac2_dn11), ((-locals.var_ac31_dn14) + locals.var_ac2_dn14),)
    } else {
        (locals.var_ac1, locals.var_ac1_dn0, locals.var_ac1_dn2, locals.var_ac1_dn4, locals.var_ac1_dn5, locals.var_ac1_dn6, locals.var_ac1_dn7, locals.var_ac1_dn8, locals.var_ac1_dn9, locals.var_ac1_dn10, locals.var_ac1_dn11, locals.var_ac1_dn14,)
    }
};
        locals.var_ac1 = assign91870_e140677;
        locals.var_ac1_dn0 = assign91870_e140677_d_n0;
        locals.var_ac1_dn2 = assign91870_e140677_d_n2;
        locals.var_ac1_dn4 = assign91870_e140677_d_n4;
        locals.var_ac1_dn5 = assign91870_e140677_d_n5;
        locals.var_ac1_dn6 = assign91870_e140677_d_n6;
        locals.var_ac1_dn7 = assign91870_e140677_d_n7;
        locals.var_ac1_dn8 = assign91870_e140677_d_n8;
        locals.var_ac1_dn9 = assign91870_e140677_d_n9;
        locals.var_ac1_dn10 = assign91870_e140677_d_n10;
        locals.var_ac1_dn11 = assign91870_e140677_d_n11;
        locals.var_ac1_dn14 = assign91870_e140677_d_n14;
        locals.var_ac1_rv = 0.0;

        let (assign91880_e140687, assign91880_e140687_d_n0, assign91880_e140687_d_n2, assign91880_e140687_d_n4, assign91880_e140687_d_n5, assign91880_e140687_d_n6, assign91880_e140687_d_n7, assign91880_e140687_d_n8, assign91880_e140687_d_n9, assign91880_e140687_d_n10, assign91880_e140687_d_n11, assign91880_e140687_d_n14,) = {
    if (((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 != 0.0)) {
        let assign91880_e140685: f64 = (locals.var_ac1).powf(0.3333333333333333);
        (assign91880_e140685, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn0)) } } else { (assign91880_e140685 * (0.3333333333333333 * (locals.var_ac1_dn0 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn2)) } } else { (assign91880_e140685 * (0.3333333333333333 * (locals.var_ac1_dn2 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn4)) } } else { (assign91880_e140685 * (0.3333333333333333 * (locals.var_ac1_dn4 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn5)) } } else { (assign91880_e140685 * (0.3333333333333333 * (locals.var_ac1_dn5 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn6)) } } else { (assign91880_e140685 * (0.3333333333333333 * (locals.var_ac1_dn6 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn7)) } } else { (assign91880_e140685 * (0.3333333333333333 * (locals.var_ac1_dn7 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn8)) } } else { (assign91880_e140685 * (0.3333333333333333 * (locals.var_ac1_dn8 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn9)) } } else { (assign91880_e140685 * (0.3333333333333333 * (locals.var_ac1_dn9 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn10)) } } else { (assign91880_e140685 * (0.3333333333333333 * (locals.var_ac1_dn10 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn11)) } } else { (assign91880_e140685 * (0.3333333333333333 * (locals.var_ac1_dn11 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn14)) } } else { (assign91880_e140685 * (0.3333333333333333 * (locals.var_ac1_dn14 / locals.var_ac1))) },)
    } else {
        (locals.var_acd, locals.var_acd_dn0, locals.var_acd_dn2, locals.var_acd_dn4, locals.var_acd_dn5, locals.var_acd_dn6, locals.var_acd_dn7, locals.var_acd_dn8, locals.var_acd_dn9, locals.var_acd_dn10, locals.var_acd_dn11, locals.var_acd_dn14,)
    }
};
        locals.var_acd = assign91880_e140687;
        locals.var_acd_dn0 = assign91880_e140687_d_n0;
        locals.var_acd_dn2 = assign91880_e140687_d_n2;
        locals.var_acd_dn4 = assign91880_e140687_d_n4;
        locals.var_acd_dn5 = assign91880_e140687_d_n5;
        locals.var_acd_dn6 = assign91880_e140687_d_n6;
        locals.var_acd_dn7 = assign91880_e140687_d_n7;
        locals.var_acd_dn8 = assign91880_e140687_d_n8;
        locals.var_acd_dn9 = assign91880_e140687_d_n9;
        locals.var_acd_dn10 = assign91880_e140687_d_n10;
        locals.var_acd_dn11 = assign91880_e140687_d_n11;
        locals.var_acd_dn14 = assign91880_e140687_d_n14;
        locals.var_acd_rv = 0.0;

        let (assign91890_e140712, assign91890_e140712_d_n0, assign91890_e140712_d_n2, assign91890_e140712_d_n4, assign91890_e140712_d_n5, assign91890_e140712_d_n6, assign91890_e140712_d_n7, assign91890_e140712_d_n8, assign91890_e140712_d_n9, assign91890_e140712_d_n10, assign91890_e140712_d_n11, assign91890_e140712_d_n14,) = {
    if (((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 != 0.0)) {
        let assign91890_e140694: f64 = (-4.0);
        let assign91890_e140696: f64 = (assign91890_e140694 * 1.414213562373095);
        let assign91890_e140699: f64 = (12.0 * locals.var_ty);
        let assign91890_e140700: f64 = (assign91890_e140696 - assign91890_e140699);
        let assign91890_e140703: f64 = (2.0 * locals.var_acd);
        let assign91890_e140704: f64 = (assign91890_e140700 + assign91890_e140703);
        let assign91890_e140707: f64 = (1.414213562373095 * locals.var_acd);
        let assign91890_e140709: f64 = (assign91890_e140707 * locals.var_acd);
        let assign91890_e140710: f64 = (assign91890_e140704 + assign91890_e140709);
        (assign91890_e140710, (((-(12.0 * locals.var_ty_dn0)) + (2.0 * locals.var_acd_dn0)) + (((1.414213562373095 * locals.var_acd_dn0) * locals.var_acd) + (assign91890_e140707 * locals.var_acd_dn0))), (((-(12.0 * locals.var_ty_dn2)) + (2.0 * locals.var_acd_dn2)) + (((1.414213562373095 * locals.var_acd_dn2) * locals.var_acd) + (assign91890_e140707 * locals.var_acd_dn2))), (((-(12.0 * locals.var_ty_dn4)) + (2.0 * locals.var_acd_dn4)) + (((1.414213562373095 * locals.var_acd_dn4) * locals.var_acd) + (assign91890_e140707 * locals.var_acd_dn4))), (((-(12.0 * locals.var_ty_dn5)) + (2.0 * locals.var_acd_dn5)) + (((1.414213562373095 * locals.var_acd_dn5) * locals.var_acd) + (assign91890_e140707 * locals.var_acd_dn5))), (((-(12.0 * locals.var_ty_dn6)) + (2.0 * locals.var_acd_dn6)) + (((1.414213562373095 * locals.var_acd_dn6) * locals.var_acd) + (assign91890_e140707 * locals.var_acd_dn6))), (((-(12.0 * locals.var_ty_dn7)) + (2.0 * locals.var_acd_dn7)) + (((1.414213562373095 * locals.var_acd_dn7) * locals.var_acd) + (assign91890_e140707 * locals.var_acd_dn7))), (((-(12.0 * locals.var_ty_dn8)) + (2.0 * locals.var_acd_dn8)) + (((1.414213562373095 * locals.var_acd_dn8) * locals.var_acd) + (assign91890_e140707 * locals.var_acd_dn8))), (((-(12.0 * locals.var_ty_dn9)) + (2.0 * locals.var_acd_dn9)) + (((1.414213562373095 * locals.var_acd_dn9) * locals.var_acd) + (assign91890_e140707 * locals.var_acd_dn9))), (((-(12.0 * locals.var_ty_dn10)) + (2.0 * locals.var_acd_dn10)) + (((1.414213562373095 * locals.var_acd_dn10) * locals.var_acd) + (assign91890_e140707 * locals.var_acd_dn10))), (((-(12.0 * locals.var_ty_dn11)) + (2.0 * locals.var_acd_dn11)) + (((1.414213562373095 * locals.var_acd_dn11) * locals.var_acd) + (assign91890_e140707 * locals.var_acd_dn11))), (((-(12.0 * locals.var_ty_dn14)) + (2.0 * locals.var_acd_dn14)) + (((1.414213562373095 * locals.var_acd_dn14) * locals.var_acd) + (assign91890_e140707 * locals.var_acd_dn14))),)
    } else {
        (locals.var_acn, locals.var_acn_dn0, locals.var_acn_dn2, locals.var_acn_dn4, locals.var_acn_dn5, locals.var_acn_dn6, locals.var_acn_dn7, locals.var_acn_dn8, locals.var_acn_dn9, locals.var_acn_dn10, locals.var_acn_dn11, locals.var_acn_dn14,)
    }
};
        locals.var_acn = assign91890_e140712;
        locals.var_acn_dn0 = assign91890_e140712_d_n0;
        locals.var_acn_dn2 = assign91890_e140712_d_n2;
        locals.var_acn_dn4 = assign91890_e140712_d_n4;
        locals.var_acn_dn5 = assign91890_e140712_d_n5;
        locals.var_acn_dn6 = assign91890_e140712_d_n6;
        locals.var_acn_dn7 = assign91890_e140712_d_n7;
        locals.var_acn_dn8 = assign91890_e140712_d_n8;
        locals.var_acn_dn9 = assign91890_e140712_d_n9;
        locals.var_acn_dn10 = assign91890_e140712_d_n10;
        locals.var_acn_dn11 = assign91890_e140712_d_n11;
        locals.var_acn_dn14 = assign91890_e140712_d_n14;
        locals.var_acn_rv = 0.0;

        let (assign91900_e140722, assign91900_e140722_d_n0, assign91900_e140722_d_n2, assign91900_e140722_d_n4, assign91900_e140722_d_n5, assign91900_e140722_d_n6, assign91900_e140722_d_n7, assign91900_e140722_d_n8, assign91900_e140722_d_n9, assign91900_e140722_d_n10, assign91900_e140722_d_n11, assign91900_e140722_d_n14,) = {
    if (((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 != 0.0)) {
        let assign91900_e140720: f64 = (locals.var_acn / locals.var_acd);
        (assign91900_e140720, (((locals.var_acn_dn0 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn0)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn2 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn2)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn4 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn4)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn5 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn5)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn6 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn6)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn7 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn7)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn8 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn8)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn9 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn9)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn10 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn10)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn11 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn11)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn14 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn14)) / (locals.var_acd * locals.var_acd)),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
        locals.var_chi = assign91900_e140722;
        locals.var_chi_dn0 = assign91900_e140722_d_n0;
        locals.var_chi_dn2 = assign91900_e140722_d_n2;
        locals.var_chi_dn4 = assign91900_e140722_d_n4;
        locals.var_chi_dn5 = assign91900_e140722_d_n5;
        locals.var_chi_dn6 = assign91900_e140722_d_n6;
        locals.var_chi_dn7 = assign91900_e140722_d_n7;
        locals.var_chi_dn8 = assign91900_e140722_d_n8;
        locals.var_chi_dn9 = assign91900_e140722_d_n9;
        locals.var_chi_dn10 = assign91900_e140722_d_n10;
        locals.var_chi_dn11 = assign91900_e140722_d_n11;
        locals.var_chi_dn14 = assign91900_e140722_d_n14;
        locals.var_chi_rv = 0.0;

        let (assign91910_e140732, assign91910_e140732_d_n0, assign91910_e140732_d_n2, assign91910_e140732_d_n4, assign91910_e140732_d_n5, assign91910_e140732_d_n6, assign91910_e140732_d_n7, assign91910_e140732_d_n8, assign91910_e140732_d_n9, assign91910_e140732_d_n10, assign91910_e140732_d_n11, assign91910_e140732_d_n14,) = {
    if (((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 != 0.0)) {
        let assign91910_e140730: f64 = (locals.var_chi * locals.var_beta_inv);
        (assign91910_e140730, ((locals.var_chi_dn0 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn0)), ((locals.var_chi_dn2 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn2)), ((locals.var_chi_dn4 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn4)), ((locals.var_chi_dn5 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn5)), ((locals.var_chi_dn6 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn6)), ((locals.var_chi_dn7 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn7)), ((locals.var_chi_dn8 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn8)), ((locals.var_chi_dn9 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn9)), ((locals.var_chi_dn10 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn10)), ((locals.var_chi_dn11 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn11)), ((locals.var_chi_dn14 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign91910_e140732;
        locals.var_t1_dn0 = assign91910_e140732_d_n0;
        locals.var_t1_dn2 = assign91910_e140732_d_n2;
        locals.var_t1_dn4 = assign91910_e140732_d_n4;
        locals.var_t1_dn5 = assign91910_e140732_d_n5;
        locals.var_t1_dn6 = assign91910_e140732_d_n6;
        locals.var_t1_dn7 = assign91910_e140732_d_n7;
        locals.var_t1_dn8 = assign91910_e140732_d_n8;
        locals.var_t1_dn9 = assign91910_e140732_d_n9;
        locals.var_t1_dn10 = assign91910_e140732_d_n10;
        locals.var_t1_dn11 = assign91910_e140732_d_n11;
        locals.var_t1_dn14 = assign91910_e140732_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign91920_e140742, assign91920_e140742_d_n0, assign91920_e140742_d_n2, assign91920_e140742_d_n4, assign91920_e140742_d_n5, assign91920_e140742_d_n6, assign91920_e140742_d_n7, assign91920_e140742_d_n8, assign91920_e140742_d_n9, assign91920_e140742_d_n10, assign91920_e140742_d_n11, assign91920_e140742_d_n14,) = {
    if (((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 != 0.0)) {
        let assign91920_e140740: f64 = (locals.var_t1 / locals.var_ps0_min);
        (assign91920_e140740, (((locals.var_t1_dn0 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn0)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn2 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn2)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn4 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn4)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn5 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn5)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn6 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn6)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn7 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn7)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn8 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn8)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn9 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn9)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn10 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn10)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn11 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn11)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn14 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn14)) / (locals.var_ps0_min * locals.var_ps0_min)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign91920_e140742;
        locals.var_t2_dn0 = assign91920_e140742_d_n0;
        locals.var_t2_dn2 = assign91920_e140742_d_n2;
        locals.var_t2_dn4 = assign91920_e140742_d_n4;
        locals.var_t2_dn5 = assign91920_e140742_d_n5;
        locals.var_t2_dn6 = assign91920_e140742_d_n6;
        locals.var_t2_dn7 = assign91920_e140742_d_n7;
        locals.var_t2_dn8 = assign91920_e140742_d_n8;
        locals.var_t2_dn9 = assign91920_e140742_d_n9;
        locals.var_t2_dn10 = assign91920_e140742_d_n10;
        locals.var_t2_dn11 = assign91920_e140742_d_n11;
        locals.var_t2_dn14 = assign91920_e140742_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign91930_e140755, assign91930_e140755_d_n0, assign91930_e140755_d_n2, assign91930_e140755_d_n4, assign91930_e140755_d_n5, assign91930_e140755_d_n6, assign91930_e140755_d_n7, assign91930_e140755_d_n8, assign91930_e140755_d_n9, assign91930_e140755_d_n10, assign91930_e140755_d_n11, assign91930_e140755_d_n14,) = {
    if (((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 != 0.0)) {
        let assign91930_e140751: f64 = (locals.var_t2 * locals.var_t2);
        let assign91930_e140752: f64 = (1.0 + assign91930_e140751);
        let assign91930_e140753: f64 = (assign91930_e140752).sqrt();
        (assign91930_e140753, (((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)) / (2.0 * assign91930_e140753)), (((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)) / (2.0 * assign91930_e140753)), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign91930_e140753)), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign91930_e140753)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign91930_e140753)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign91930_e140753)), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign91930_e140753)), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign91930_e140753)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign91930_e140753)), (((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign91930_e140753)), (((locals.var_t2_dn14 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn14)) / (2.0 * assign91930_e140753)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign91930_e140755;
        locals.var_t3_dn0 = assign91930_e140755_d_n0;
        locals.var_t3_dn2 = assign91930_e140755_d_n2;
        locals.var_t3_dn4 = assign91930_e140755_d_n4;
        locals.var_t3_dn5 = assign91930_e140755_d_n5;
        locals.var_t3_dn6 = assign91930_e140755_d_n6;
        locals.var_t3_dn7 = assign91930_e140755_d_n7;
        locals.var_t3_dn8 = assign91930_e140755_d_n8;
        locals.var_t3_dn9 = assign91930_e140755_d_n9;
        locals.var_t3_dn10 = assign91930_e140755_d_n10;
        locals.var_t3_dn11 = assign91930_e140755_d_n11;
        locals.var_t3_dn14 = assign91930_e140755_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign91940_e140767, assign91940_e140767_d_n0, assign91940_e140767_d_n2, assign91940_e140767_d_n4, assign91940_e140767_d_n5, assign91940_e140767_d_n6, assign91940_e140767_d_n7, assign91940_e140767_d_n8, assign91940_e140767_d_n9, assign91940_e140767_d_n10, assign91940_e140767_d_n11, assign91940_e140767_d_n14,) = {
    if (((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 != 0.0)) {
        let assign91940_e140763: f64 = (locals.var_t1 / locals.var_t3);
        let assign91940_e140765: f64 = (assign91940_e140763 - locals.var_vxbgmtcl);
        (assign91940_e140765, ((((locals.var_t1_dn0 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn0)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn0), ((((locals.var_t1_dn2 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn2)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn2), ((((locals.var_t1_dn4 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn4)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn4), ((((locals.var_t1_dn5 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn5)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn5), ((((locals.var_t1_dn6 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn6)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn6), ((((locals.var_t1_dn7 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn7)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn7), ((((locals.var_t1_dn8 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn8)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn8), ((((locals.var_t1_dn9 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn9)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn9), ((((locals.var_t1_dn10 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn10)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn10), ((((locals.var_t1_dn11 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn11)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn11), ((((locals.var_t1_dn14 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn14)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn14),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn14,)
    }
};
        locals.var_ps0ld = assign91940_e140767;
        locals.var_ps0ld_dn0 = assign91940_e140767_d_n0;
        locals.var_ps0ld_dn2 = assign91940_e140767_d_n2;
        locals.var_ps0ld_dn4 = assign91940_e140767_d_n4;
        locals.var_ps0ld_dn5 = assign91940_e140767_d_n5;
        locals.var_ps0ld_dn6 = assign91940_e140767_d_n6;
        locals.var_ps0ld_dn7 = assign91940_e140767_d_n7;
        locals.var_ps0ld_dn8 = assign91940_e140767_d_n8;
        locals.var_ps0ld_dn9 = assign91940_e140767_d_n9;
        locals.var_ps0ld_dn10 = assign91940_e140767_d_n10;
        locals.var_ps0ld_dn11 = assign91940_e140767_d_n11;
        locals.var_ps0ld_dn14 = assign91940_e140767_d_n14;
        locals.var_ps0ld_rv = 0.0;

        let (assign91950_e140777, assign91950_e140777_d_n0, assign91950_e140777_d_n2, assign91950_e140777_d_n4, assign91950_e140777_d_n5, assign91950_e140777_d_n6, assign91950_e140777_d_n7, assign91950_e140777_d_n8, assign91950_e140777_d_n9, assign91950_e140777_d_n10, assign91950_e140777_d_n11, assign91950_e140777_d_n14,) = {
    if (((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 != 0.0)) {
        let assign91950_e140775: f64 = (locals.var_vgpld - locals.var_ps0ld);
        (assign91950_e140775, (-locals.var_ps0ld_dn0), (locals.var_vgpld_dn2 - locals.var_ps0ld_dn2), (-locals.var_ps0ld_dn4), (-locals.var_ps0ld_dn5), (-locals.var_ps0ld_dn6), (locals.var_vgpld_dn7 - locals.var_ps0ld_dn7), (locals.var_vgpld_dn8 - locals.var_ps0ld_dn8), (locals.var_vgpld_dn9 - locals.var_ps0ld_dn9), (-locals.var_ps0ld_dn10), (-locals.var_ps0ld_dn11), (-locals.var_ps0ld_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign91950_e140777;
        locals.var_t2_dn0 = assign91950_e140777_d_n0;
        locals.var_t2_dn2 = assign91950_e140777_d_n2;
        locals.var_t2_dn4 = assign91950_e140777_d_n4;
        locals.var_t2_dn5 = assign91950_e140777_d_n5;
        locals.var_t2_dn6 = assign91950_e140777_d_n6;
        locals.var_t2_dn7 = assign91950_e140777_d_n7;
        locals.var_t2_dn8 = assign91950_e140777_d_n8;
        locals.var_t2_dn9 = assign91950_e140777_d_n9;
        locals.var_t2_dn10 = assign91950_e140777_d_n10;
        locals.var_t2_dn11 = assign91950_e140777_d_n11;
        locals.var_t2_dn14 = assign91950_e140777_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign91960_e140787, assign91960_e140787_d_n0, assign91960_e140787_d_n2, assign91960_e140787_d_n4, assign91960_e140787_d_n5, assign91960_e140787_d_n6, assign91960_e140787_d_n7, assign91960_e140787_d_n8, assign91960_e140787_d_n9, assign91960_e140787_d_n10, assign91960_e140787_d_n11, assign91960_e140787_d_n14,) = {
    if (((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 != 0.0)) {
        let assign91960_e140785: f64 = (locals.var_cox0_func * locals.var_t2);
        (assign91960_e140785, (locals.var_cox0_func * locals.var_t2_dn0), (locals.var_cox0_func * locals.var_t2_dn2), (locals.var_cox0_func * locals.var_t2_dn4), (locals.var_cox0_func * locals.var_t2_dn5), (locals.var_cox0_func * locals.var_t2_dn6), (locals.var_cox0_func * locals.var_t2_dn7), (locals.var_cox0_func * locals.var_t2_dn8), (locals.var_cox0_func * locals.var_t2_dn9), (locals.var_cox0_func * locals.var_t2_dn10), (locals.var_cox0_func * locals.var_t2_dn11), (locals.var_cox0_func * locals.var_t2_dn14),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn4, locals.var_qsuld_dn5, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn8, locals.var_qsuld_dn9, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn14,)
    }
};
        locals.var_qsuld = assign91960_e140787;
        locals.var_qsuld_dn0 = assign91960_e140787_d_n0;
        locals.var_qsuld_dn2 = assign91960_e140787_d_n2;
        locals.var_qsuld_dn4 = assign91960_e140787_d_n4;
        locals.var_qsuld_dn5 = assign91960_e140787_d_n5;
        locals.var_qsuld_dn6 = assign91960_e140787_d_n6;
        locals.var_qsuld_dn7 = assign91960_e140787_d_n7;
        locals.var_qsuld_dn8 = assign91960_e140787_d_n8;
        locals.var_qsuld_dn9 = assign91960_e140787_d_n9;
        locals.var_qsuld_dn10 = assign91960_e140787_d_n10;
        locals.var_qsuld_dn11 = assign91960_e140787_d_n11;
        locals.var_qsuld_dn14 = assign91960_e140787_d_n14;
        locals.var_qsuld_rv = 0.0;

        let (assign91970_e140795, assign91970_e140795_d_n0, assign91970_e140795_d_n2, assign91970_e140795_d_n4, assign91970_e140795_d_n5, assign91970_e140795_d_n6, assign91970_e140795_d_n7, assign91970_e140795_d_n8, assign91970_e140795_d_n9, assign91970_e140795_d_n10, assign91970_e140795_d_n11, assign91970_e140795_d_n14,) = {
    if (((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 != 0.0)) {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn4, locals.var_qsuld_dn5, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn8, locals.var_qsuld_dn9, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn14,)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn8, locals.var_qbuld_dn9, locals.var_qbuld_dn10, locals.var_qbuld_dn11, locals.var_qbuld_dn14,)
    }
};
        locals.var_qbuld = assign91970_e140795;
        locals.var_qbuld_dn0 = assign91970_e140795_d_n0;
        locals.var_qbuld_dn2 = assign91970_e140795_d_n2;
        locals.var_qbuld_dn4 = assign91970_e140795_d_n4;
        locals.var_qbuld_dn5 = assign91970_e140795_d_n5;
        locals.var_qbuld_dn6 = assign91970_e140795_d_n6;
        locals.var_qbuld_dn7 = assign91970_e140795_d_n7;
        locals.var_qbuld_dn8 = assign91970_e140795_d_n8;
        locals.var_qbuld_dn9 = assign91970_e140795_d_n9;
        locals.var_qbuld_dn10 = assign91970_e140795_d_n10;
        locals.var_qbuld_dn11 = assign91970_e140795_d_n11;
        locals.var_qbuld_dn14 = assign91970_e140795_d_n14;
        locals.var_qbuld_rv = 0.0;

        let (assign91980_e140803, assign91980_e140803_d_n0, assign91980_e140803_d_n2, assign91980_e140803_d_n4, assign91980_e140803_d_n5, assign91980_e140803_d_n6, assign91980_e140803_d_n7, assign91980_e140803_d_n8, assign91980_e140803_d_n9, assign91980_e140803_d_n10, assign91980_e140803_d_n11, assign91980_e140803_d_n14,) = {
    if (((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 != 0.0)) {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn14,)
    } else {
        (locals.var_ps0ld_ini__blk2126, locals.var_ps0ld_ini__blk2126_dn0, locals.var_ps0ld_ini__blk2126_dn2, locals.var_ps0ld_ini__blk2126_dn4, locals.var_ps0ld_ini__blk2126_dn5, locals.var_ps0ld_ini__blk2126_dn6, locals.var_ps0ld_ini__blk2126_dn7, locals.var_ps0ld_ini__blk2126_dn8, locals.var_ps0ld_ini__blk2126_dn9, locals.var_ps0ld_ini__blk2126_dn10, locals.var_ps0ld_ini__blk2126_dn11, locals.var_ps0ld_ini__blk2126_dn14,)
    }
};
        locals.var_ps0ld_ini__blk2126 = assign91980_e140803;
        locals.var_ps0ld_ini__blk2126_dn0 = assign91980_e140803_d_n0;
        locals.var_ps0ld_ini__blk2126_dn2 = assign91980_e140803_d_n2;
        locals.var_ps0ld_ini__blk2126_dn4 = assign91980_e140803_d_n4;
        locals.var_ps0ld_ini__blk2126_dn5 = assign91980_e140803_d_n5;
        locals.var_ps0ld_ini__blk2126_dn6 = assign91980_e140803_d_n6;
        locals.var_ps0ld_ini__blk2126_dn7 = assign91980_e140803_d_n7;
        locals.var_ps0ld_ini__blk2126_dn8 = assign91980_e140803_d_n8;
        locals.var_ps0ld_ini__blk2126_dn9 = assign91980_e140803_d_n9;
        locals.var_ps0ld_ini__blk2126_dn10 = assign91980_e140803_d_n10;
        locals.var_ps0ld_ini__blk2126_dn11 = assign91980_e140803_d_n11;
        locals.var_ps0ld_ini__blk2126_dn14 = assign91980_e140803_d_n14;
        locals.var_ps0ld_ini__blk2126_rv = 0.0;

        let assign91990_e140807: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign91990_e140808: f64 = (locals.var_beta * assign91990_e140807);
        let assign91990_e140812: f64 = (10.0 * 2.220446049250313e-16);
        let assign91990_e140814: f64 = (assign91990_e140812 - 1.0);
        let assign91990_e140816: f64 = (assign91990_e140814 * locals.var_fac1p2);
        let assign91990_e140818: f64 = (assign91990_e140816 * locals.var_beta2);
        let assign91990_e140820: f64 = (assign91990_e140818 / 4.0);
        let assign91990_e140821: f64 = (1.0 + assign91990_e140820);
        let assign91990_e140822: f64 = if assign91990_e140808 < assign91990_e140821 { 1.0 } else { 0.0 };
        locals.var_guard2145 = assign91990_e140822;
        locals.var_guard2145_rv = 0.0;

        let (assign92000_e140839, assign92000_e140839_d_n0, assign92000_e140839_d_n2, assign92000_e140839_d_n4, assign92000_e140839_d_n5, assign92000_e140839_d_n6, assign92000_e140839_d_n7, assign92000_e140839_d_n8, assign92000_e140839_d_n9, assign92000_e140839_d_n10, assign92000_e140839_d_n11, assign92000_e140839_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2145 != 0.0)) {
        let assign92000_e140834: f64 = (locals.var_fac1p2 * locals.var_beta);
        let assign92000_e140836: f64 = (assign92000_e140834 / 2.0);
        let assign92000_e140837: f64 = (locals.var_vgpld + assign92000_e140836);
        (assign92000_e140837, (((locals.var_fac1p2_dn0 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn0)) / 2.0), (locals.var_vgpld_dn2 + (((locals.var_fac1p2_dn2 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn2)) / 2.0)), (((locals.var_fac1p2_dn4 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn4)) / 2.0), (((locals.var_fac1p2_dn5 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn5)) / 2.0), (((locals.var_fac1p2_dn6 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn6)) / 2.0), (locals.var_vgpld_dn7 + (((locals.var_fac1p2_dn7 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn7)) / 2.0)), (locals.var_vgpld_dn8 + (((locals.var_fac1p2_dn8 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn8)) / 2.0)), (locals.var_vgpld_dn9 + (((locals.var_fac1p2_dn9 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn9)) / 2.0)), (((locals.var_fac1p2_dn10 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn10)) / 2.0), (((locals.var_fac1p2_dn11 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn11)) / 2.0), (((locals.var_fac1p2_dn14 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn14)) / 2.0),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn14,)
    }
};
        locals.var_ps0_inia = assign92000_e140839;
        locals.var_ps0_inia_dn0 = assign92000_e140839_d_n0;
        locals.var_ps0_inia_dn2 = assign92000_e140839_d_n2;
        locals.var_ps0_inia_dn4 = assign92000_e140839_d_n4;
        locals.var_ps0_inia_dn5 = assign92000_e140839_d_n5;
        locals.var_ps0_inia_dn6 = assign92000_e140839_d_n6;
        locals.var_ps0_inia_dn7 = assign92000_e140839_d_n7;
        locals.var_ps0_inia_dn8 = assign92000_e140839_d_n8;
        locals.var_ps0_inia_dn9 = assign92000_e140839_d_n9;
        locals.var_ps0_inia_dn10 = assign92000_e140839_d_n10;
        locals.var_ps0_inia_dn11 = assign92000_e140839_d_n11;
        locals.var_ps0_inia_dn14 = assign92000_e140839_d_n14;
        locals.var_ps0_inia_rv = 0.0;

        let (assign92010_e140865, assign92010_e140865_d_n0, assign92010_e140865_d_n2, assign92010_e140865_d_n4, assign92010_e140865_d_n5, assign92010_e140865_d_n6, assign92010_e140865_d_n7, assign92010_e140865_d_n8, assign92010_e140865_d_n9, assign92010_e140865_d_n10, assign92010_e140865_d_n11, assign92010_e140865_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2145 == 0.0)) {
        let assign92010_e140854: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign92010_e140855: f64 = (locals.var_beta * assign92010_e140854);
        let assign92010_e140857: f64 = (assign92010_e140855 - 1.0);
        let assign92010_e140858: f64 = (4.0 * assign92010_e140857);
        let assign92010_e140861: f64 = (locals.var_fac1p2 * locals.var_beta2);
        let assign92010_e140862: f64 = (assign92010_e140858 / assign92010_e140861);
        let assign92010_e140863: f64 = (1.0 + assign92010_e140862);
        (assign92010_e140863, ((((4.0 * ((locals.var_beta_dn0 * assign92010_e140854) + (locals.var_beta * locals.var_vxbgmtcl_dn0))) * assign92010_e140861) - (assign92010_e140858 * ((locals.var_fac1p2_dn0 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn0)))) / (assign92010_e140861 * assign92010_e140861)), ((((4.0 * ((locals.var_beta_dn2 * assign92010_e140854) + (locals.var_beta * (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2)))) * assign92010_e140861) - (assign92010_e140858 * ((locals.var_fac1p2_dn2 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn2)))) / (assign92010_e140861 * assign92010_e140861)), ((((4.0 * ((locals.var_beta_dn4 * assign92010_e140854) + (locals.var_beta * locals.var_vxbgmtcl_dn4))) * assign92010_e140861) - (assign92010_e140858 * ((locals.var_fac1p2_dn4 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn4)))) / (assign92010_e140861 * assign92010_e140861)), ((((4.0 * ((locals.var_beta_dn5 * assign92010_e140854) + (locals.var_beta * locals.var_vxbgmtcl_dn5))) * assign92010_e140861) - (assign92010_e140858 * ((locals.var_fac1p2_dn5 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn5)))) / (assign92010_e140861 * assign92010_e140861)), ((((4.0 * ((locals.var_beta_dn6 * assign92010_e140854) + (locals.var_beta * locals.var_vxbgmtcl_dn6))) * assign92010_e140861) - (assign92010_e140858 * ((locals.var_fac1p2_dn6 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn6)))) / (assign92010_e140861 * assign92010_e140861)), ((((4.0 * ((locals.var_beta_dn7 * assign92010_e140854) + (locals.var_beta * (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7)))) * assign92010_e140861) - (assign92010_e140858 * ((locals.var_fac1p2_dn7 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn7)))) / (assign92010_e140861 * assign92010_e140861)), ((((4.0 * ((locals.var_beta_dn8 * assign92010_e140854) + (locals.var_beta * (locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8)))) * assign92010_e140861) - (assign92010_e140858 * ((locals.var_fac1p2_dn8 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn8)))) / (assign92010_e140861 * assign92010_e140861)), ((((4.0 * ((locals.var_beta_dn9 * assign92010_e140854) + (locals.var_beta * (locals.var_vgpld_dn9 + locals.var_vxbgmtcl_dn9)))) * assign92010_e140861) - (assign92010_e140858 * ((locals.var_fac1p2_dn9 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn9)))) / (assign92010_e140861 * assign92010_e140861)), ((((4.0 * ((locals.var_beta_dn10 * assign92010_e140854) + (locals.var_beta * locals.var_vxbgmtcl_dn10))) * assign92010_e140861) - (assign92010_e140858 * ((locals.var_fac1p2_dn10 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn10)))) / (assign92010_e140861 * assign92010_e140861)), ((((4.0 * ((locals.var_beta_dn11 * assign92010_e140854) + (locals.var_beta * locals.var_vxbgmtcl_dn11))) * assign92010_e140861) - (assign92010_e140858 * ((locals.var_fac1p2_dn11 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn11)))) / (assign92010_e140861 * assign92010_e140861)), ((((4.0 * ((locals.var_beta_dn14 * assign92010_e140854) + (locals.var_beta * locals.var_vxbgmtcl_dn14))) * assign92010_e140861) - (assign92010_e140858 * ((locals.var_fac1p2_dn14 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn14)))) / (assign92010_e140861 * assign92010_e140861)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign92010_e140865;
        locals.var_tx_dn0 = assign92010_e140865_d_n0;
        locals.var_tx_dn2 = assign92010_e140865_d_n2;
        locals.var_tx_dn4 = assign92010_e140865_d_n4;
        locals.var_tx_dn5 = assign92010_e140865_d_n5;
        locals.var_tx_dn6 = assign92010_e140865_d_n6;
        locals.var_tx_dn7 = assign92010_e140865_d_n7;
        locals.var_tx_dn8 = assign92010_e140865_d_n8;
        locals.var_tx_dn9 = assign92010_e140865_d_n9;
        locals.var_tx_dn10 = assign92010_e140865_d_n10;
        locals.var_tx_dn11 = assign92010_e140865_d_n11;
        locals.var_tx_dn14 = assign92010_e140865_d_n14;
        locals.var_tx_rv = 0.0;

        let (assign92020_e140888, assign92020_e140888_d_n0, assign92020_e140888_d_n2, assign92020_e140888_d_n4, assign92020_e140888_d_n5, assign92020_e140888_d_n6, assign92020_e140888_d_n7, assign92020_e140888_d_n8, assign92020_e140888_d_n9, assign92020_e140888_d_n10, assign92020_e140888_d_n11, assign92020_e140888_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2145 == 0.0)) {
        let assign92020_e140878: f64 = (locals.var_fac1p2 * locals.var_beta);
        let assign92020_e140880: f64 = (assign92020_e140878 / 2.0);
        let assign92020_e140883: f64 = (locals.var_tx).sqrt();
        let assign92020_e140884: f64 = (1.0 - assign92020_e140883);
        let assign92020_e140885: f64 = (assign92020_e140880 * assign92020_e140884);
        let assign92020_e140886: f64 = (locals.var_vgpld + assign92020_e140885);
        (assign92020_e140886, (((((locals.var_fac1p2_dn0 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn0)) / 2.0) * assign92020_e140884) + (assign92020_e140880 * (-(locals.var_tx_dn0 / (2.0 * assign92020_e140883))))), (locals.var_vgpld_dn2 + (((((locals.var_fac1p2_dn2 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn2)) / 2.0) * assign92020_e140884) + (assign92020_e140880 * (-(locals.var_tx_dn2 / (2.0 * assign92020_e140883)))))), (((((locals.var_fac1p2_dn4 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn4)) / 2.0) * assign92020_e140884) + (assign92020_e140880 * (-(locals.var_tx_dn4 / (2.0 * assign92020_e140883))))), (((((locals.var_fac1p2_dn5 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn5)) / 2.0) * assign92020_e140884) + (assign92020_e140880 * (-(locals.var_tx_dn5 / (2.0 * assign92020_e140883))))), (((((locals.var_fac1p2_dn6 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn6)) / 2.0) * assign92020_e140884) + (assign92020_e140880 * (-(locals.var_tx_dn6 / (2.0 * assign92020_e140883))))), (locals.var_vgpld_dn7 + (((((locals.var_fac1p2_dn7 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn7)) / 2.0) * assign92020_e140884) + (assign92020_e140880 * (-(locals.var_tx_dn7 / (2.0 * assign92020_e140883)))))), (locals.var_vgpld_dn8 + (((((locals.var_fac1p2_dn8 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn8)) / 2.0) * assign92020_e140884) + (assign92020_e140880 * (-(locals.var_tx_dn8 / (2.0 * assign92020_e140883)))))), (locals.var_vgpld_dn9 + (((((locals.var_fac1p2_dn9 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn9)) / 2.0) * assign92020_e140884) + (assign92020_e140880 * (-(locals.var_tx_dn9 / (2.0 * assign92020_e140883)))))), (((((locals.var_fac1p2_dn10 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn10)) / 2.0) * assign92020_e140884) + (assign92020_e140880 * (-(locals.var_tx_dn10 / (2.0 * assign92020_e140883))))), (((((locals.var_fac1p2_dn11 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn11)) / 2.0) * assign92020_e140884) + (assign92020_e140880 * (-(locals.var_tx_dn11 / (2.0 * assign92020_e140883))))), (((((locals.var_fac1p2_dn14 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn14)) / 2.0) * assign92020_e140884) + (assign92020_e140880 * (-(locals.var_tx_dn14 / (2.0 * assign92020_e140883))))),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn14,)
    }
};
        locals.var_ps0_inia = assign92020_e140888;
        locals.var_ps0_inia_dn0 = assign92020_e140888_d_n0;
        locals.var_ps0_inia_dn2 = assign92020_e140888_d_n2;
        locals.var_ps0_inia_dn4 = assign92020_e140888_d_n4;
        locals.var_ps0_inia_dn5 = assign92020_e140888_d_n5;
        locals.var_ps0_inia_dn6 = assign92020_e140888_d_n6;
        locals.var_ps0_inia_dn7 = assign92020_e140888_d_n7;
        locals.var_ps0_inia_dn8 = assign92020_e140888_d_n8;
        locals.var_ps0_inia_dn9 = assign92020_e140888_d_n9;
        locals.var_ps0_inia_dn10 = assign92020_e140888_d_n10;
        locals.var_ps0_inia_dn11 = assign92020_e140888_d_n11;
        locals.var_ps0_inia_dn14 = assign92020_e140888_d_n14;
        locals.var_ps0_inia_rv = 0.0;

        let (assign92030_e140901, assign92030_e140901_d_n0, assign92030_e140901_d_n2, assign92030_e140901_d_n4, assign92030_e140901_d_n5, assign92030_e140901_d_n6, assign92030_e140901_d_n7, assign92030_e140901_d_n8, assign92030_e140901_d_n9, assign92030_e140901_d_n10, assign92030_e140901_d_n11, assign92030_e140901_d_n14,) = {
    if (((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) {
        let assign92030_e140898: f64 = (locals.var_ps0_inia + locals.var_vxbgmtcl);
        let assign92030_e140899: f64 = (locals.var_beta * assign92030_e140898);
        (assign92030_e140899, ((locals.var_beta_dn0 * assign92030_e140898) + (locals.var_beta * (locals.var_ps0_inia_dn0 + locals.var_vxbgmtcl_dn0))), ((locals.var_beta_dn2 * assign92030_e140898) + (locals.var_beta * (locals.var_ps0_inia_dn2 + locals.var_vxbgmtcl_dn2))), ((locals.var_beta_dn4 * assign92030_e140898) + (locals.var_beta * (locals.var_ps0_inia_dn4 + locals.var_vxbgmtcl_dn4))), ((locals.var_beta_dn5 * assign92030_e140898) + (locals.var_beta * (locals.var_ps0_inia_dn5 + locals.var_vxbgmtcl_dn5))), ((locals.var_beta_dn6 * assign92030_e140898) + (locals.var_beta * (locals.var_ps0_inia_dn6 + locals.var_vxbgmtcl_dn6))), ((locals.var_beta_dn7 * assign92030_e140898) + (locals.var_beta * (locals.var_ps0_inia_dn7 + locals.var_vxbgmtcl_dn7))), ((locals.var_beta_dn8 * assign92030_e140898) + (locals.var_beta * (locals.var_ps0_inia_dn8 + locals.var_vxbgmtcl_dn8))), ((locals.var_beta_dn9 * assign92030_e140898) + (locals.var_beta * (locals.var_ps0_inia_dn9 + locals.var_vxbgmtcl_dn9))), ((locals.var_beta_dn10 * assign92030_e140898) + (locals.var_beta * (locals.var_ps0_inia_dn10 + locals.var_vxbgmtcl_dn10))), ((locals.var_beta_dn11 * assign92030_e140898) + (locals.var_beta * (locals.var_ps0_inia_dn11 + locals.var_vxbgmtcl_dn11))), ((locals.var_beta_dn14 * assign92030_e140898) + (locals.var_beta * (locals.var_ps0_inia_dn14 + locals.var_vxbgmtcl_dn14))),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
        locals.var_chi = assign92030_e140901;
        locals.var_chi_dn0 = assign92030_e140901_d_n0;
        locals.var_chi_dn2 = assign92030_e140901_d_n2;
        locals.var_chi_dn4 = assign92030_e140901_d_n4;
        locals.var_chi_dn5 = assign92030_e140901_d_n5;
        locals.var_chi_dn6 = assign92030_e140901_d_n6;
        locals.var_chi_dn7 = assign92030_e140901_d_n7;
        locals.var_chi_dn8 = assign92030_e140901_d_n8;
        locals.var_chi_dn9 = assign92030_e140901_d_n9;
        locals.var_chi_dn10 = assign92030_e140901_d_n10;
        locals.var_chi_dn11 = assign92030_e140901_d_n11;
        locals.var_chi_dn14 = assign92030_e140901_d_n14;
        locals.var_chi_rv = 0.0;

        let assign92040_e140904: f64 = if locals.var_chi >= 3.0 { 1.0 } else { 0.0 };
        locals.var_guard2146 = assign92040_e140904;
        locals.var_guard2146_rv = 0.0;

        let (assign92060_e140928, assign92060_e140928_d_n0, assign92060_e140928_d_n2, assign92060_e140928_d_n4, assign92060_e140928_d_n5, assign92060_e140928_d_n6, assign92060_e140928_d_n7, assign92060_e140928_d_n8, assign92060_e140928_d_n9, assign92060_e140928_d_n10, assign92060_e140928_d_n11, assign92060_e140928_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2146 != 0.0)) {
        let assign92060_e140925: f64 = (-locals.var_chi);
        let assign92060_e140926: f64 = (assign92060_e140925).exp();
        (assign92060_e140926, (assign92060_e140926 * (-locals.var_chi_dn0)), (assign92060_e140926 * (-locals.var_chi_dn2)), (assign92060_e140926 * (-locals.var_chi_dn4)), (assign92060_e140926 * (-locals.var_chi_dn5)), (assign92060_e140926 * (-locals.var_chi_dn6)), (assign92060_e140926 * (-locals.var_chi_dn7)), (assign92060_e140926 * (-locals.var_chi_dn8)), (assign92060_e140926 * (-locals.var_chi_dn9)), (assign92060_e140926 * (-locals.var_chi_dn10)), (assign92060_e140926 * (-locals.var_chi_dn11)), (assign92060_e140926 * (-locals.var_chi_dn14)),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn14,)
    }
};
        locals.var_ty = assign92060_e140928;
        locals.var_ty_dn0 = assign92060_e140928_d_n0;
        locals.var_ty_dn2 = assign92060_e140928_d_n2;
        locals.var_ty_dn4 = assign92060_e140928_d_n4;
        locals.var_ty_dn5 = assign92060_e140928_d_n5;
        locals.var_ty_dn6 = assign92060_e140928_d_n6;
        locals.var_ty_dn7 = assign92060_e140928_d_n7;
        locals.var_ty_dn8 = assign92060_e140928_d_n8;
        locals.var_ty_dn9 = assign92060_e140928_d_n9;
        locals.var_ty_dn10 = assign92060_e140928_d_n10;
        locals.var_ty_dn11 = assign92060_e140928_d_n11;
        locals.var_ty_dn14 = assign92060_e140928_d_n14;
        locals.var_ty_rv = 0.0;

        let (assign92070_e140955, assign92070_e140955_d_n0, assign92070_e140955_d_n2, assign92070_e140955_d_n4, assign92070_e140955_d_n5, assign92070_e140955_d_n6, assign92070_e140955_d_n7, assign92070_e140955_d_n8, assign92070_e140955_d_n9, assign92070_e140955_d_n10, assign92070_e140955_d_n11, assign92070_e140955_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2146 != 0.0)) {
        let assign92070_e140942: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign92070_e140943: f64 = (locals.var_beta * assign92070_e140942);
        let assign92070_e140945: f64 = (assign92070_e140943 - 1.0);
        let assign92070_e140947: f64 = (assign92070_e140945 + locals.var_ty);
        let assign92070_e140948: f64 = (4.0 * assign92070_e140947);
        let assign92070_e140951: f64 = (locals.var_fac1p2 * locals.var_beta2);
        let assign92070_e140952: f64 = (assign92070_e140948 / assign92070_e140951);
        let assign92070_e140953: f64 = (1.0 + assign92070_e140952);
        (assign92070_e140953, ((((4.0 * (((locals.var_beta_dn0 * assign92070_e140942) + (locals.var_beta * locals.var_vxbgmtcl_dn0)) + locals.var_ty_dn0)) * assign92070_e140951) - (assign92070_e140948 * ((locals.var_fac1p2_dn0 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn0)))) / (assign92070_e140951 * assign92070_e140951)), ((((4.0 * (((locals.var_beta_dn2 * assign92070_e140942) + (locals.var_beta * (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2))) + locals.var_ty_dn2)) * assign92070_e140951) - (assign92070_e140948 * ((locals.var_fac1p2_dn2 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn2)))) / (assign92070_e140951 * assign92070_e140951)), ((((4.0 * (((locals.var_beta_dn4 * assign92070_e140942) + (locals.var_beta * locals.var_vxbgmtcl_dn4)) + locals.var_ty_dn4)) * assign92070_e140951) - (assign92070_e140948 * ((locals.var_fac1p2_dn4 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn4)))) / (assign92070_e140951 * assign92070_e140951)), ((((4.0 * (((locals.var_beta_dn5 * assign92070_e140942) + (locals.var_beta * locals.var_vxbgmtcl_dn5)) + locals.var_ty_dn5)) * assign92070_e140951) - (assign92070_e140948 * ((locals.var_fac1p2_dn5 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn5)))) / (assign92070_e140951 * assign92070_e140951)), ((((4.0 * (((locals.var_beta_dn6 * assign92070_e140942) + (locals.var_beta * locals.var_vxbgmtcl_dn6)) + locals.var_ty_dn6)) * assign92070_e140951) - (assign92070_e140948 * ((locals.var_fac1p2_dn6 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn6)))) / (assign92070_e140951 * assign92070_e140951)), ((((4.0 * (((locals.var_beta_dn7 * assign92070_e140942) + (locals.var_beta * (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7))) + locals.var_ty_dn7)) * assign92070_e140951) - (assign92070_e140948 * ((locals.var_fac1p2_dn7 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn7)))) / (assign92070_e140951 * assign92070_e140951)), ((((4.0 * (((locals.var_beta_dn8 * assign92070_e140942) + (locals.var_beta * (locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8))) + locals.var_ty_dn8)) * assign92070_e140951) - (assign92070_e140948 * ((locals.var_fac1p2_dn8 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn8)))) / (assign92070_e140951 * assign92070_e140951)), ((((4.0 * (((locals.var_beta_dn9 * assign92070_e140942) + (locals.var_beta * (locals.var_vgpld_dn9 + locals.var_vxbgmtcl_dn9))) + locals.var_ty_dn9)) * assign92070_e140951) - (assign92070_e140948 * ((locals.var_fac1p2_dn9 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn9)))) / (assign92070_e140951 * assign92070_e140951)), ((((4.0 * (((locals.var_beta_dn10 * assign92070_e140942) + (locals.var_beta * locals.var_vxbgmtcl_dn10)) + locals.var_ty_dn10)) * assign92070_e140951) - (assign92070_e140948 * ((locals.var_fac1p2_dn10 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn10)))) / (assign92070_e140951 * assign92070_e140951)), ((((4.0 * (((locals.var_beta_dn11 * assign92070_e140942) + (locals.var_beta * locals.var_vxbgmtcl_dn11)) + locals.var_ty_dn11)) * assign92070_e140951) - (assign92070_e140948 * ((locals.var_fac1p2_dn11 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn11)))) / (assign92070_e140951 * assign92070_e140951)), ((((4.0 * (((locals.var_beta_dn14 * assign92070_e140942) + (locals.var_beta * locals.var_vxbgmtcl_dn14)) + locals.var_ty_dn14)) * assign92070_e140951) - (assign92070_e140948 * ((locals.var_fac1p2_dn14 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn14)))) / (assign92070_e140951 * assign92070_e140951)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign92070_e140955;
        locals.var_tx_dn0 = assign92070_e140955_d_n0;
        locals.var_tx_dn2 = assign92070_e140955_d_n2;
        locals.var_tx_dn4 = assign92070_e140955_d_n4;
        locals.var_tx_dn5 = assign92070_e140955_d_n5;
        locals.var_tx_dn6 = assign92070_e140955_d_n6;
        locals.var_tx_dn7 = assign92070_e140955_d_n7;
        locals.var_tx_dn8 = assign92070_e140955_d_n8;
        locals.var_tx_dn9 = assign92070_e140955_d_n9;
        locals.var_tx_dn10 = assign92070_e140955_d_n10;
        locals.var_tx_dn11 = assign92070_e140955_d_n11;
        locals.var_tx_dn14 = assign92070_e140955_d_n14;
        locals.var_tx_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_355(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign92080_e140977, assign92080_e140977_d_n0, assign92080_e140977_d_n2, assign92080_e140977_d_n4, assign92080_e140977_d_n5, assign92080_e140977_d_n6, assign92080_e140977_d_n7, assign92080_e140977_d_n8, assign92080_e140977_d_n9, assign92080_e140977_d_n10, assign92080_e140977_d_n11, assign92080_e140977_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2146 != 0.0)) {
        let assign92080_e140967: f64 = (locals.var_fac1p2 * locals.var_beta);
        let assign92080_e140969: f64 = (assign92080_e140967 / 2.0);
        let assign92080_e140972: f64 = (locals.var_tx).sqrt();
        let assign92080_e140973: f64 = (1.0 - assign92080_e140972);
        let assign92080_e140974: f64 = (assign92080_e140969 * assign92080_e140973);
        let assign92080_e140975: f64 = (locals.var_vgpld + assign92080_e140974);
        (assign92080_e140975, (((((locals.var_fac1p2_dn0 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn0)) / 2.0) * assign92080_e140973) + (assign92080_e140969 * (-(locals.var_tx_dn0 / (2.0 * assign92080_e140972))))), (locals.var_vgpld_dn2 + (((((locals.var_fac1p2_dn2 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn2)) / 2.0) * assign92080_e140973) + (assign92080_e140969 * (-(locals.var_tx_dn2 / (2.0 * assign92080_e140972)))))), (((((locals.var_fac1p2_dn4 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn4)) / 2.0) * assign92080_e140973) + (assign92080_e140969 * (-(locals.var_tx_dn4 / (2.0 * assign92080_e140972))))), (((((locals.var_fac1p2_dn5 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn5)) / 2.0) * assign92080_e140973) + (assign92080_e140969 * (-(locals.var_tx_dn5 / (2.0 * assign92080_e140972))))), (((((locals.var_fac1p2_dn6 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn6)) / 2.0) * assign92080_e140973) + (assign92080_e140969 * (-(locals.var_tx_dn6 / (2.0 * assign92080_e140972))))), (locals.var_vgpld_dn7 + (((((locals.var_fac1p2_dn7 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn7)) / 2.0) * assign92080_e140973) + (assign92080_e140969 * (-(locals.var_tx_dn7 / (2.0 * assign92080_e140972)))))), (locals.var_vgpld_dn8 + (((((locals.var_fac1p2_dn8 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn8)) / 2.0) * assign92080_e140973) + (assign92080_e140969 * (-(locals.var_tx_dn8 / (2.0 * assign92080_e140972)))))), (locals.var_vgpld_dn9 + (((((locals.var_fac1p2_dn9 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn9)) / 2.0) * assign92080_e140973) + (assign92080_e140969 * (-(locals.var_tx_dn9 / (2.0 * assign92080_e140972)))))), (((((locals.var_fac1p2_dn10 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn10)) / 2.0) * assign92080_e140973) + (assign92080_e140969 * (-(locals.var_tx_dn10 / (2.0 * assign92080_e140972))))), (((((locals.var_fac1p2_dn11 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn11)) / 2.0) * assign92080_e140973) + (assign92080_e140969 * (-(locals.var_tx_dn11 / (2.0 * assign92080_e140972))))), (((((locals.var_fac1p2_dn14 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn14)) / 2.0) * assign92080_e140973) + (assign92080_e140969 * (-(locals.var_tx_dn14 / (2.0 * assign92080_e140972))))),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn14,)
    }
};
        locals.var_ps0_inia = assign92080_e140977;
        locals.var_ps0_inia_dn0 = assign92080_e140977_d_n0;
        locals.var_ps0_inia_dn2 = assign92080_e140977_d_n2;
        locals.var_ps0_inia_dn4 = assign92080_e140977_d_n4;
        locals.var_ps0_inia_dn5 = assign92080_e140977_d_n5;
        locals.var_ps0_inia_dn6 = assign92080_e140977_d_n6;
        locals.var_ps0_inia_dn7 = assign92080_e140977_d_n7;
        locals.var_ps0_inia_dn8 = assign92080_e140977_d_n8;
        locals.var_ps0_inia_dn9 = assign92080_e140977_d_n9;
        locals.var_ps0_inia_dn10 = assign92080_e140977_d_n10;
        locals.var_ps0_inia_dn11 = assign92080_e140977_d_n11;
        locals.var_ps0_inia_dn14 = assign92080_e140977_d_n14;
        locals.var_ps0_inia_rv = 0.0;

        let (assign92090_e140992, assign92090_e140992_d_n0, assign92090_e140992_d_n2, assign92090_e140992_d_n4, assign92090_e140992_d_n5, assign92090_e140992_d_n6, assign92090_e140992_d_n7, assign92090_e140992_d_n8, assign92090_e140992_d_n9, assign92090_e140992_d_n10, assign92090_e140992_d_n11, assign92090_e140992_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2146 != 0.0)) {
        let assign92090_e140989: f64 = (locals.var_ps0_inia + locals.var_vxbgmtcl);
        let assign92090_e140990: f64 = (locals.var_beta * assign92090_e140989);
        (assign92090_e140990, ((locals.var_beta_dn0 * assign92090_e140989) + (locals.var_beta * (locals.var_ps0_inia_dn0 + locals.var_vxbgmtcl_dn0))), ((locals.var_beta_dn2 * assign92090_e140989) + (locals.var_beta * (locals.var_ps0_inia_dn2 + locals.var_vxbgmtcl_dn2))), ((locals.var_beta_dn4 * assign92090_e140989) + (locals.var_beta * (locals.var_ps0_inia_dn4 + locals.var_vxbgmtcl_dn4))), ((locals.var_beta_dn5 * assign92090_e140989) + (locals.var_beta * (locals.var_ps0_inia_dn5 + locals.var_vxbgmtcl_dn5))), ((locals.var_beta_dn6 * assign92090_e140989) + (locals.var_beta * (locals.var_ps0_inia_dn6 + locals.var_vxbgmtcl_dn6))), ((locals.var_beta_dn7 * assign92090_e140989) + (locals.var_beta * (locals.var_ps0_inia_dn7 + locals.var_vxbgmtcl_dn7))), ((locals.var_beta_dn8 * assign92090_e140989) + (locals.var_beta * (locals.var_ps0_inia_dn8 + locals.var_vxbgmtcl_dn8))), ((locals.var_beta_dn9 * assign92090_e140989) + (locals.var_beta * (locals.var_ps0_inia_dn9 + locals.var_vxbgmtcl_dn9))), ((locals.var_beta_dn10 * assign92090_e140989) + (locals.var_beta * (locals.var_ps0_inia_dn10 + locals.var_vxbgmtcl_dn10))), ((locals.var_beta_dn11 * assign92090_e140989) + (locals.var_beta * (locals.var_ps0_inia_dn11 + locals.var_vxbgmtcl_dn11))), ((locals.var_beta_dn14 * assign92090_e140989) + (locals.var_beta * (locals.var_ps0_inia_dn14 + locals.var_vxbgmtcl_dn14))),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
        locals.var_chi = assign92090_e140992;
        locals.var_chi_dn0 = assign92090_e140992_d_n0;
        locals.var_chi_dn2 = assign92090_e140992_d_n2;
        locals.var_chi_dn4 = assign92090_e140992_d_n4;
        locals.var_chi_dn5 = assign92090_e140992_d_n5;
        locals.var_chi_dn6 = assign92090_e140992_d_n6;
        locals.var_chi_dn7 = assign92090_e140992_d_n7;
        locals.var_chi_dn8 = assign92090_e140992_d_n8;
        locals.var_chi_dn9 = assign92090_e140992_d_n9;
        locals.var_chi_dn10 = assign92090_e140992_d_n10;
        locals.var_chi_dn11 = assign92090_e140992_d_n11;
        locals.var_chi_dn14 = assign92090_e140992_d_n14;
        locals.var_chi_rv = 0.0;

        let (assign92100_e141005, assign92100_e141005_d_n0, assign92100_e141005_d_n2, assign92100_e141005_d_n4, assign92100_e141005_d_n5, assign92100_e141005_d_n6, assign92100_e141005_d_n7, assign92100_e141005_d_n8, assign92100_e141005_d_n9, assign92100_e141005_d_n10, assign92100_e141005_d_n11, assign92100_e141005_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2146 != 0.0)) {
        let assign92100_e141002: f64 = (-locals.var_chi);
        let assign92100_e141003: f64 = (assign92100_e141002).exp();
        (assign92100_e141003, (assign92100_e141003 * (-locals.var_chi_dn0)), (assign92100_e141003 * (-locals.var_chi_dn2)), (assign92100_e141003 * (-locals.var_chi_dn4)), (assign92100_e141003 * (-locals.var_chi_dn5)), (assign92100_e141003 * (-locals.var_chi_dn6)), (assign92100_e141003 * (-locals.var_chi_dn7)), (assign92100_e141003 * (-locals.var_chi_dn8)), (assign92100_e141003 * (-locals.var_chi_dn9)), (assign92100_e141003 * (-locals.var_chi_dn10)), (assign92100_e141003 * (-locals.var_chi_dn11)), (assign92100_e141003 * (-locals.var_chi_dn14)),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn14,)
    }
};
        locals.var_ty = assign92100_e141005;
        locals.var_ty_dn0 = assign92100_e141005_d_n0;
        locals.var_ty_dn2 = assign92100_e141005_d_n2;
        locals.var_ty_dn4 = assign92100_e141005_d_n4;
        locals.var_ty_dn5 = assign92100_e141005_d_n5;
        locals.var_ty_dn6 = assign92100_e141005_d_n6;
        locals.var_ty_dn7 = assign92100_e141005_d_n7;
        locals.var_ty_dn8 = assign92100_e141005_d_n8;
        locals.var_ty_dn9 = assign92100_e141005_d_n9;
        locals.var_ty_dn10 = assign92100_e141005_d_n10;
        locals.var_ty_dn11 = assign92100_e141005_d_n11;
        locals.var_ty_dn14 = assign92100_e141005_d_n14;
        locals.var_ty_rv = 0.0;

        let (assign92110_e141032, assign92110_e141032_d_n0, assign92110_e141032_d_n2, assign92110_e141032_d_n4, assign92110_e141032_d_n5, assign92110_e141032_d_n6, assign92110_e141032_d_n7, assign92110_e141032_d_n8, assign92110_e141032_d_n9, assign92110_e141032_d_n10, assign92110_e141032_d_n11, assign92110_e141032_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2146 != 0.0)) {
        let assign92110_e141019: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign92110_e141020: f64 = (locals.var_beta * assign92110_e141019);
        let assign92110_e141022: f64 = (assign92110_e141020 - 1.0);
        let assign92110_e141024: f64 = (assign92110_e141022 + locals.var_ty);
        let assign92110_e141025: f64 = (4.0 * assign92110_e141024);
        let assign92110_e141028: f64 = (locals.var_fac1p2 * locals.var_beta2);
        let assign92110_e141029: f64 = (assign92110_e141025 / assign92110_e141028);
        let assign92110_e141030: f64 = (1.0 + assign92110_e141029);
        (assign92110_e141030, ((((4.0 * (((locals.var_beta_dn0 * assign92110_e141019) + (locals.var_beta * locals.var_vxbgmtcl_dn0)) + locals.var_ty_dn0)) * assign92110_e141028) - (assign92110_e141025 * ((locals.var_fac1p2_dn0 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn0)))) / (assign92110_e141028 * assign92110_e141028)), ((((4.0 * (((locals.var_beta_dn2 * assign92110_e141019) + (locals.var_beta * (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2))) + locals.var_ty_dn2)) * assign92110_e141028) - (assign92110_e141025 * ((locals.var_fac1p2_dn2 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn2)))) / (assign92110_e141028 * assign92110_e141028)), ((((4.0 * (((locals.var_beta_dn4 * assign92110_e141019) + (locals.var_beta * locals.var_vxbgmtcl_dn4)) + locals.var_ty_dn4)) * assign92110_e141028) - (assign92110_e141025 * ((locals.var_fac1p2_dn4 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn4)))) / (assign92110_e141028 * assign92110_e141028)), ((((4.0 * (((locals.var_beta_dn5 * assign92110_e141019) + (locals.var_beta * locals.var_vxbgmtcl_dn5)) + locals.var_ty_dn5)) * assign92110_e141028) - (assign92110_e141025 * ((locals.var_fac1p2_dn5 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn5)))) / (assign92110_e141028 * assign92110_e141028)), ((((4.0 * (((locals.var_beta_dn6 * assign92110_e141019) + (locals.var_beta * locals.var_vxbgmtcl_dn6)) + locals.var_ty_dn6)) * assign92110_e141028) - (assign92110_e141025 * ((locals.var_fac1p2_dn6 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn6)))) / (assign92110_e141028 * assign92110_e141028)), ((((4.0 * (((locals.var_beta_dn7 * assign92110_e141019) + (locals.var_beta * (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7))) + locals.var_ty_dn7)) * assign92110_e141028) - (assign92110_e141025 * ((locals.var_fac1p2_dn7 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn7)))) / (assign92110_e141028 * assign92110_e141028)), ((((4.0 * (((locals.var_beta_dn8 * assign92110_e141019) + (locals.var_beta * (locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8))) + locals.var_ty_dn8)) * assign92110_e141028) - (assign92110_e141025 * ((locals.var_fac1p2_dn8 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn8)))) / (assign92110_e141028 * assign92110_e141028)), ((((4.0 * (((locals.var_beta_dn9 * assign92110_e141019) + (locals.var_beta * (locals.var_vgpld_dn9 + locals.var_vxbgmtcl_dn9))) + locals.var_ty_dn9)) * assign92110_e141028) - (assign92110_e141025 * ((locals.var_fac1p2_dn9 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn9)))) / (assign92110_e141028 * assign92110_e141028)), ((((4.0 * (((locals.var_beta_dn10 * assign92110_e141019) + (locals.var_beta * locals.var_vxbgmtcl_dn10)) + locals.var_ty_dn10)) * assign92110_e141028) - (assign92110_e141025 * ((locals.var_fac1p2_dn10 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn10)))) / (assign92110_e141028 * assign92110_e141028)), ((((4.0 * (((locals.var_beta_dn11 * assign92110_e141019) + (locals.var_beta * locals.var_vxbgmtcl_dn11)) + locals.var_ty_dn11)) * assign92110_e141028) - (assign92110_e141025 * ((locals.var_fac1p2_dn11 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn11)))) / (assign92110_e141028 * assign92110_e141028)), ((((4.0 * (((locals.var_beta_dn14 * assign92110_e141019) + (locals.var_beta * locals.var_vxbgmtcl_dn14)) + locals.var_ty_dn14)) * assign92110_e141028) - (assign92110_e141025 * ((locals.var_fac1p2_dn14 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn14)))) / (assign92110_e141028 * assign92110_e141028)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign92110_e141032;
        locals.var_tx_dn0 = assign92110_e141032_d_n0;
        locals.var_tx_dn2 = assign92110_e141032_d_n2;
        locals.var_tx_dn4 = assign92110_e141032_d_n4;
        locals.var_tx_dn5 = assign92110_e141032_d_n5;
        locals.var_tx_dn6 = assign92110_e141032_d_n6;
        locals.var_tx_dn7 = assign92110_e141032_d_n7;
        locals.var_tx_dn8 = assign92110_e141032_d_n8;
        locals.var_tx_dn9 = assign92110_e141032_d_n9;
        locals.var_tx_dn10 = assign92110_e141032_d_n10;
        locals.var_tx_dn11 = assign92110_e141032_d_n11;
        locals.var_tx_dn14 = assign92110_e141032_d_n14;
        locals.var_tx_rv = 0.0;

        let (assign92120_e141054, assign92120_e141054_d_n0, assign92120_e141054_d_n2, assign92120_e141054_d_n4, assign92120_e141054_d_n5, assign92120_e141054_d_n6, assign92120_e141054_d_n7, assign92120_e141054_d_n8, assign92120_e141054_d_n9, assign92120_e141054_d_n10, assign92120_e141054_d_n11, assign92120_e141054_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2146 != 0.0)) {
        let assign92120_e141044: f64 = (locals.var_fac1p2 * locals.var_beta);
        let assign92120_e141046: f64 = (assign92120_e141044 / 2.0);
        let assign92120_e141049: f64 = (locals.var_tx).sqrt();
        let assign92120_e141050: f64 = (1.0 - assign92120_e141049);
        let assign92120_e141051: f64 = (assign92120_e141046 * assign92120_e141050);
        let assign92120_e141052: f64 = (locals.var_vgpld + assign92120_e141051);
        (assign92120_e141052, (((((locals.var_fac1p2_dn0 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn0)) / 2.0) * assign92120_e141050) + (assign92120_e141046 * (-(locals.var_tx_dn0 / (2.0 * assign92120_e141049))))), (locals.var_vgpld_dn2 + (((((locals.var_fac1p2_dn2 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn2)) / 2.0) * assign92120_e141050) + (assign92120_e141046 * (-(locals.var_tx_dn2 / (2.0 * assign92120_e141049)))))), (((((locals.var_fac1p2_dn4 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn4)) / 2.0) * assign92120_e141050) + (assign92120_e141046 * (-(locals.var_tx_dn4 / (2.0 * assign92120_e141049))))), (((((locals.var_fac1p2_dn5 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn5)) / 2.0) * assign92120_e141050) + (assign92120_e141046 * (-(locals.var_tx_dn5 / (2.0 * assign92120_e141049))))), (((((locals.var_fac1p2_dn6 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn6)) / 2.0) * assign92120_e141050) + (assign92120_e141046 * (-(locals.var_tx_dn6 / (2.0 * assign92120_e141049))))), (locals.var_vgpld_dn7 + (((((locals.var_fac1p2_dn7 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn7)) / 2.0) * assign92120_e141050) + (assign92120_e141046 * (-(locals.var_tx_dn7 / (2.0 * assign92120_e141049)))))), (locals.var_vgpld_dn8 + (((((locals.var_fac1p2_dn8 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn8)) / 2.0) * assign92120_e141050) + (assign92120_e141046 * (-(locals.var_tx_dn8 / (2.0 * assign92120_e141049)))))), (locals.var_vgpld_dn9 + (((((locals.var_fac1p2_dn9 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn9)) / 2.0) * assign92120_e141050) + (assign92120_e141046 * (-(locals.var_tx_dn9 / (2.0 * assign92120_e141049)))))), (((((locals.var_fac1p2_dn10 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn10)) / 2.0) * assign92120_e141050) + (assign92120_e141046 * (-(locals.var_tx_dn10 / (2.0 * assign92120_e141049))))), (((((locals.var_fac1p2_dn11 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn11)) / 2.0) * assign92120_e141050) + (assign92120_e141046 * (-(locals.var_tx_dn11 / (2.0 * assign92120_e141049))))), (((((locals.var_fac1p2_dn14 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn14)) / 2.0) * assign92120_e141050) + (assign92120_e141046 * (-(locals.var_tx_dn14 / (2.0 * assign92120_e141049))))),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn14,)
    }
};
        locals.var_ps0_inia = assign92120_e141054;
        locals.var_ps0_inia_dn0 = assign92120_e141054_d_n0;
        locals.var_ps0_inia_dn2 = assign92120_e141054_d_n2;
        locals.var_ps0_inia_dn4 = assign92120_e141054_d_n4;
        locals.var_ps0_inia_dn5 = assign92120_e141054_d_n5;
        locals.var_ps0_inia_dn6 = assign92120_e141054_d_n6;
        locals.var_ps0_inia_dn7 = assign92120_e141054_d_n7;
        locals.var_ps0_inia_dn8 = assign92120_e141054_d_n8;
        locals.var_ps0_inia_dn9 = assign92120_e141054_d_n9;
        locals.var_ps0_inia_dn10 = assign92120_e141054_d_n10;
        locals.var_ps0_inia_dn11 = assign92120_e141054_d_n11;
        locals.var_ps0_inia_dn14 = assign92120_e141054_d_n14;
        locals.var_ps0_inia_rv = 0.0;

        let (assign92130_e141069, assign92130_e141069_d_n0, assign92130_e141069_d_n2, assign92130_e141069_d_n4, assign92130_e141069_d_n5, assign92130_e141069_d_n6, assign92130_e141069_d_n7, assign92130_e141069_d_n8, assign92130_e141069_d_n9, assign92130_e141069_d_n10, assign92130_e141069_d_n11, assign92130_e141069_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2146 != 0.0)) {
        let assign92130_e141066: f64 = (locals.var_ps0_inia + locals.var_vxbgmtcl);
        let assign92130_e141067: f64 = (locals.var_beta * assign92130_e141066);
        (assign92130_e141067, ((locals.var_beta_dn0 * assign92130_e141066) + (locals.var_beta * (locals.var_ps0_inia_dn0 + locals.var_vxbgmtcl_dn0))), ((locals.var_beta_dn2 * assign92130_e141066) + (locals.var_beta * (locals.var_ps0_inia_dn2 + locals.var_vxbgmtcl_dn2))), ((locals.var_beta_dn4 * assign92130_e141066) + (locals.var_beta * (locals.var_ps0_inia_dn4 + locals.var_vxbgmtcl_dn4))), ((locals.var_beta_dn5 * assign92130_e141066) + (locals.var_beta * (locals.var_ps0_inia_dn5 + locals.var_vxbgmtcl_dn5))), ((locals.var_beta_dn6 * assign92130_e141066) + (locals.var_beta * (locals.var_ps0_inia_dn6 + locals.var_vxbgmtcl_dn6))), ((locals.var_beta_dn7 * assign92130_e141066) + (locals.var_beta * (locals.var_ps0_inia_dn7 + locals.var_vxbgmtcl_dn7))), ((locals.var_beta_dn8 * assign92130_e141066) + (locals.var_beta * (locals.var_ps0_inia_dn8 + locals.var_vxbgmtcl_dn8))), ((locals.var_beta_dn9 * assign92130_e141066) + (locals.var_beta * (locals.var_ps0_inia_dn9 + locals.var_vxbgmtcl_dn9))), ((locals.var_beta_dn10 * assign92130_e141066) + (locals.var_beta * (locals.var_ps0_inia_dn10 + locals.var_vxbgmtcl_dn10))), ((locals.var_beta_dn11 * assign92130_e141066) + (locals.var_beta * (locals.var_ps0_inia_dn11 + locals.var_vxbgmtcl_dn11))), ((locals.var_beta_dn14 * assign92130_e141066) + (locals.var_beta * (locals.var_ps0_inia_dn14 + locals.var_vxbgmtcl_dn14))),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
        locals.var_chi = assign92130_e141069;
        locals.var_chi_dn0 = assign92130_e141069_d_n0;
        locals.var_chi_dn2 = assign92130_e141069_d_n2;
        locals.var_chi_dn4 = assign92130_e141069_d_n4;
        locals.var_chi_dn5 = assign92130_e141069_d_n5;
        locals.var_chi_dn6 = assign92130_e141069_d_n6;
        locals.var_chi_dn7 = assign92130_e141069_d_n7;
        locals.var_chi_dn8 = assign92130_e141069_d_n8;
        locals.var_chi_dn9 = assign92130_e141069_d_n9;
        locals.var_chi_dn10 = assign92130_e141069_d_n10;
        locals.var_chi_dn11 = assign92130_e141069_d_n11;
        locals.var_chi_dn14 = assign92130_e141069_d_n14;
        locals.var_chi_rv = 0.0;

        let (assign92150_e141115,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2146 == 0.0)) {
        let assign92150_e141094: f64 = (2.0_f64).sqrt();
        let assign92150_e141095: f64 = (9.0 * assign92150_e141094);
        let assign92150_e141096: f64 = (1.0 / assign92150_e141095);
        let assign92150_e141100: f64 = (-3.0);
        let assign92150_e141101: f64 = (assign92150_e141100).exp();
        let assign92150_e141102: f64 = (7.0 * assign92150_e141101);
        let assign92150_e141103: f64 = (5.0 + assign92150_e141102);
        let assign92150_e141107: f64 = (-3.0);
        let assign92150_e141108: f64 = (assign92150_e141107).exp();
        let assign92150_e141109: f64 = (2.0 + assign92150_e141108);
        let assign92150_e141110: f64 = (assign92150_e141109).sqrt();
        let assign92150_e141111: f64 = (54.0 * assign92150_e141110);
        let assign92150_e141112: f64 = (assign92150_e141103 / assign92150_e141111);
        let assign92150_e141113: f64 = (assign92150_e141096 - assign92150_e141112);
        (assign92150_e141113,)
    } else {
        (locals.var_ta,)
    }
};
        locals.var_ta = assign92150_e141115;
        locals.var_ta_rv = 0.0;

        let (assign92160_e141145,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2146 == 0.0)) {
        let assign92160_e141127: f64 = (-3.0);
        let assign92160_e141128: f64 = (assign92160_e141127).exp();
        let assign92160_e141129: f64 = (1.0 + assign92160_e141128);
        let assign92160_e141133: f64 = (-3.0);
        let assign92160_e141134: f64 = (assign92160_e141133).exp();
        let assign92160_e141135: f64 = (2.0 + assign92160_e141134);
        let assign92160_e141136: f64 = (assign92160_e141135).sqrt();
        let assign92160_e141137: f64 = (2.0 * assign92160_e141136);
        let assign92160_e141138: f64 = (assign92160_e141129 / assign92160_e141137);
        let assign92160_e141140: f64 = (2.0_f64).sqrt();
        let assign92160_e141142: f64 = (assign92160_e141140 / 3.0);
        let assign92160_e141143: f64 = (assign92160_e141138 - assign92160_e141142);
        (assign92160_e141143,)
    } else {
        (locals.var_tb,)
    }
};
        locals.var_tb = assign92160_e141145;
        locals.var_tb_rv = 0.0;

        let (assign92170_e141166, assign92170_e141166_d_n0, assign92170_e141166_d_n2, assign92170_e141166_d_n4, assign92170_e141166_d_n5, assign92170_e141166_d_n6, assign92170_e141166_d_n7, assign92170_e141166_d_n8, assign92170_e141166_d_n9, assign92170_e141166_d_n10, assign92170_e141166_d_n11, assign92170_e141166_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2146 == 0.0)) {
        let assign92170_e141157: f64 = (2.0_f64).sqrt();
        let assign92170_e141158: f64 = (1.0 / assign92170_e141157);
        let assign92170_e141162: f64 = (locals.var_beta * locals.var_fac1);
        let assign92170_e141163: f64 = (1.0 / assign92170_e141162);
        let assign92170_e141164: f64 = (assign92170_e141158 + assign92170_e141163);
        (assign92170_e141164, (-(((locals.var_beta_dn0 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn0)) / (assign92170_e141162 * assign92170_e141162))), (-(((locals.var_beta_dn2 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn2)) / (assign92170_e141162 * assign92170_e141162))), (-(((locals.var_beta_dn4 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn4)) / (assign92170_e141162 * assign92170_e141162))), (-(((locals.var_beta_dn5 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn5)) / (assign92170_e141162 * assign92170_e141162))), (-(((locals.var_beta_dn6 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn6)) / (assign92170_e141162 * assign92170_e141162))), (-(((locals.var_beta_dn7 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn7)) / (assign92170_e141162 * assign92170_e141162))), (-(((locals.var_beta_dn8 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn8)) / (assign92170_e141162 * assign92170_e141162))), (-(((locals.var_beta_dn9 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn9)) / (assign92170_e141162 * assign92170_e141162))), (-(((locals.var_beta_dn10 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn10)) / (assign92170_e141162 * assign92170_e141162))), (-(((locals.var_beta_dn11 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn11)) / (assign92170_e141162 * assign92170_e141162))), (-(((locals.var_beta_dn14 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn14)) / (assign92170_e141162 * assign92170_e141162))),)
    } else {
        (locals.var_tc, locals.var_tc_dn0, locals.var_tc_dn2, locals.var_tc_dn4, locals.var_tc_dn5, locals.var_tc_dn6, locals.var_tc_dn7, locals.var_tc_dn8, locals.var_tc_dn9, locals.var_tc_dn10, locals.var_tc_dn11, locals.var_tc_dn14,)
    }
};
        locals.var_tc = assign92170_e141166;
        locals.var_tc_dn0 = assign92170_e141166_d_n0;
        locals.var_tc_dn2 = assign92170_e141166_d_n2;
        locals.var_tc_dn4 = assign92170_e141166_d_n4;
        locals.var_tc_dn5 = assign92170_e141166_d_n5;
        locals.var_tc_dn6 = assign92170_e141166_d_n6;
        locals.var_tc_dn7 = assign92170_e141166_d_n7;
        locals.var_tc_dn8 = assign92170_e141166_d_n8;
        locals.var_tc_dn9 = assign92170_e141166_d_n9;
        locals.var_tc_dn10 = assign92170_e141166_d_n10;
        locals.var_tc_dn11 = assign92170_e141166_d_n11;
        locals.var_tc_dn14 = assign92170_e141166_d_n14;
        locals.var_tc_rv = 0.0;

        let (assign92180_e141183, assign92180_e141183_d_n0, assign92180_e141183_d_n2, assign92180_e141183_d_n4, assign92180_e141183_d_n5, assign92180_e141183_d_n6, assign92180_e141183_d_n7, assign92180_e141183_d_n8, assign92180_e141183_d_n9, assign92180_e141183_d_n10, assign92180_e141183_d_n11, assign92180_e141183_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2146 == 0.0)) {
        let assign92180_e141178: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign92180_e141179: f64 = (-assign92180_e141178);
        let assign92180_e141181: f64 = (assign92180_e141179 / locals.var_fac1);
        (assign92180_e141181, ((((-locals.var_vxbgmtcl_dn0) * locals.var_fac1) - (assign92180_e141179 * locals.var_fac1_dn0)) / (locals.var_fac1 * locals.var_fac1)), ((((-(locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2)) * locals.var_fac1) - (assign92180_e141179 * locals.var_fac1_dn2)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn4) * locals.var_fac1) - (assign92180_e141179 * locals.var_fac1_dn4)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn5) * locals.var_fac1) - (assign92180_e141179 * locals.var_fac1_dn5)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn6) * locals.var_fac1) - (assign92180_e141179 * locals.var_fac1_dn6)) / (locals.var_fac1 * locals.var_fac1)), ((((-(locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7)) * locals.var_fac1) - (assign92180_e141179 * locals.var_fac1_dn7)) / (locals.var_fac1 * locals.var_fac1)), ((((-(locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8)) * locals.var_fac1) - (assign92180_e141179 * locals.var_fac1_dn8)) / (locals.var_fac1 * locals.var_fac1)), ((((-(locals.var_vgpld_dn9 + locals.var_vxbgmtcl_dn9)) * locals.var_fac1) - (assign92180_e141179 * locals.var_fac1_dn9)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn10) * locals.var_fac1) - (assign92180_e141179 * locals.var_fac1_dn10)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn11) * locals.var_fac1) - (assign92180_e141179 * locals.var_fac1_dn11)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn14) * locals.var_fac1) - (assign92180_e141179 * locals.var_fac1_dn14)) / (locals.var_fac1 * locals.var_fac1)),)
    } else {
        (locals.var_td, locals.var_td_dn0, locals.var_td_dn2, locals.var_td_dn4, locals.var_td_dn5, locals.var_td_dn6, locals.var_td_dn7, locals.var_td_dn8, locals.var_td_dn9, locals.var_td_dn10, locals.var_td_dn11, locals.var_td_dn14,)
    }
};
        locals.var_td = assign92180_e141183;
        locals.var_td_dn0 = assign92180_e141183_d_n0;
        locals.var_td_dn2 = assign92180_e141183_d_n2;
        locals.var_td_dn4 = assign92180_e141183_d_n4;
        locals.var_td_dn5 = assign92180_e141183_d_n5;
        locals.var_td_dn6 = assign92180_e141183_d_n6;
        locals.var_td_dn7 = assign92180_e141183_d_n7;
        locals.var_td_dn8 = assign92180_e141183_d_n8;
        locals.var_td_dn9 = assign92180_e141183_d_n9;
        locals.var_td_dn10 = assign92180_e141183_d_n10;
        locals.var_td_dn11 = assign92180_e141183_d_n11;
        locals.var_td_dn14 = assign92180_e141183_d_n14;
        locals.var_td_rv = 0.0;

        let (assign92190_e141223, assign92190_e141223_d_n0, assign92190_e141223_d_n2, assign92190_e141223_d_n4, assign92190_e141223_d_n5, assign92190_e141223_d_n6, assign92190_e141223_d_n7, assign92190_e141223_d_n8, assign92190_e141223_d_n9, assign92190_e141223_d_n10, assign92190_e141223_d_n11, assign92190_e141223_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2146 == 0.0)) {
        let assign92190_e141195: f64 = (locals.var_tb * locals.var_tb);
        let assign92190_e141197: f64 = (assign92190_e141195 * locals.var_tb);
        let assign92190_e141200: f64 = (27.0 * locals.var_ta);
        let assign92190_e141202: f64 = (assign92190_e141200 * locals.var_ta);
        let assign92190_e141204: f64 = (assign92190_e141202 * locals.var_ta);
        let assign92190_e141205: f64 = (assign92190_e141197 / assign92190_e141204);
        let assign92190_e141208: f64 = (locals.var_tb * locals.var_tc);
        let assign92190_e141211: f64 = (6.0 * locals.var_ta);
        let assign92190_e141213: f64 = (assign92190_e141211 * locals.var_ta);
        let assign92190_e141214: f64 = (assign92190_e141208 / assign92190_e141213);
        let assign92190_e141215: f64 = (assign92190_e141205 - assign92190_e141214);
        let assign92190_e141219: f64 = (2.0 * locals.var_ta);
        let assign92190_e141220: f64 = (locals.var_td / assign92190_e141219);
        let assign92190_e141221: f64 = (assign92190_e141215 + assign92190_e141220);
        (assign92190_e141221, ((-((locals.var_tb * locals.var_tc_dn0) / assign92190_e141213)) + (locals.var_td_dn0 / assign92190_e141219)), ((-((locals.var_tb * locals.var_tc_dn2) / assign92190_e141213)) + (locals.var_td_dn2 / assign92190_e141219)), ((-((locals.var_tb * locals.var_tc_dn4) / assign92190_e141213)) + (locals.var_td_dn4 / assign92190_e141219)), ((-((locals.var_tb * locals.var_tc_dn5) / assign92190_e141213)) + (locals.var_td_dn5 / assign92190_e141219)), ((-((locals.var_tb * locals.var_tc_dn6) / assign92190_e141213)) + (locals.var_td_dn6 / assign92190_e141219)), ((-((locals.var_tb * locals.var_tc_dn7) / assign92190_e141213)) + (locals.var_td_dn7 / assign92190_e141219)), ((-((locals.var_tb * locals.var_tc_dn8) / assign92190_e141213)) + (locals.var_td_dn8 / assign92190_e141219)), ((-((locals.var_tb * locals.var_tc_dn9) / assign92190_e141213)) + (locals.var_td_dn9 / assign92190_e141219)), ((-((locals.var_tb * locals.var_tc_dn10) / assign92190_e141213)) + (locals.var_td_dn10 / assign92190_e141219)), ((-((locals.var_tb * locals.var_tc_dn11) / assign92190_e141213)) + (locals.var_td_dn11 / assign92190_e141219)), ((-((locals.var_tb * locals.var_tc_dn14) / assign92190_e141213)) + (locals.var_td_dn14 / assign92190_e141219)),)
    } else {
        (locals.var_tq, locals.var_tq_dn0, locals.var_tq_dn2, locals.var_tq_dn4, locals.var_tq_dn5, locals.var_tq_dn6, locals.var_tq_dn7, locals.var_tq_dn8, locals.var_tq_dn9, locals.var_tq_dn10, locals.var_tq_dn11, locals.var_tq_dn14,)
    }
};
        locals.var_tq = assign92190_e141223;
        locals.var_tq_dn0 = assign92190_e141223_d_n0;
        locals.var_tq_dn2 = assign92190_e141223_d_n2;
        locals.var_tq_dn4 = assign92190_e141223_d_n4;
        locals.var_tq_dn5 = assign92190_e141223_d_n5;
        locals.var_tq_dn6 = assign92190_e141223_d_n6;
        locals.var_tq_dn7 = assign92190_e141223_d_n7;
        locals.var_tq_dn8 = assign92190_e141223_d_n8;
        locals.var_tq_dn9 = assign92190_e141223_d_n9;
        locals.var_tq_dn10 = assign92190_e141223_d_n10;
        locals.var_tq_dn11 = assign92190_e141223_d_n11;
        locals.var_tq_dn14 = assign92190_e141223_d_n14;
        locals.var_tq_rv = 0.0;

        let (assign92200_e141249, assign92200_e141249_d_n0, assign92200_e141249_d_n2, assign92200_e141249_d_n4, assign92200_e141249_d_n5, assign92200_e141249_d_n6, assign92200_e141249_d_n7, assign92200_e141249_d_n8, assign92200_e141249_d_n9, assign92200_e141249_d_n10, assign92200_e141249_d_n11, assign92200_e141249_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2146 == 0.0)) {
        let assign92200_e141235: f64 = (3.0 * locals.var_ta);
        let assign92200_e141237: f64 = (assign92200_e141235 * locals.var_tc);
        let assign92200_e141240: f64 = (locals.var_tb * locals.var_tb);
        let assign92200_e141241: f64 = (assign92200_e141237 - assign92200_e141240);
        let assign92200_e141244: f64 = (9.0 * locals.var_ta);
        let assign92200_e141246: f64 = (assign92200_e141244 * locals.var_ta);
        let assign92200_e141247: f64 = (assign92200_e141241 / assign92200_e141246);
        (assign92200_e141247, ((assign92200_e141235 * locals.var_tc_dn0) / assign92200_e141246), ((assign92200_e141235 * locals.var_tc_dn2) / assign92200_e141246), ((assign92200_e141235 * locals.var_tc_dn4) / assign92200_e141246), ((assign92200_e141235 * locals.var_tc_dn5) / assign92200_e141246), ((assign92200_e141235 * locals.var_tc_dn6) / assign92200_e141246), ((assign92200_e141235 * locals.var_tc_dn7) / assign92200_e141246), ((assign92200_e141235 * locals.var_tc_dn8) / assign92200_e141246), ((assign92200_e141235 * locals.var_tc_dn9) / assign92200_e141246), ((assign92200_e141235 * locals.var_tc_dn10) / assign92200_e141246), ((assign92200_e141235 * locals.var_tc_dn11) / assign92200_e141246), ((assign92200_e141235 * locals.var_tc_dn14) / assign92200_e141246),)
    } else {
        (locals.var_tp, locals.var_tp_dn0, locals.var_tp_dn2, locals.var_tp_dn4, locals.var_tp_dn5, locals.var_tp_dn6, locals.var_tp_dn7, locals.var_tp_dn8, locals.var_tp_dn9, locals.var_tp_dn10, locals.var_tp_dn11, locals.var_tp_dn14,)
    }
};
        locals.var_tp = assign92200_e141249;
        locals.var_tp_dn0 = assign92200_e141249_d_n0;
        locals.var_tp_dn2 = assign92200_e141249_d_n2;
        locals.var_tp_dn4 = assign92200_e141249_d_n4;
        locals.var_tp_dn5 = assign92200_e141249_d_n5;
        locals.var_tp_dn6 = assign92200_e141249_d_n6;
        locals.var_tp_dn7 = assign92200_e141249_d_n7;
        locals.var_tp_dn8 = assign92200_e141249_d_n8;
        locals.var_tp_dn9 = assign92200_e141249_d_n9;
        locals.var_tp_dn10 = assign92200_e141249_d_n10;
        locals.var_tp_dn11 = assign92200_e141249_d_n11;
        locals.var_tp_dn14 = assign92200_e141249_d_n14;
        locals.var_tp_rv = 0.0;

        let (assign92210_e141270, assign92210_e141270_d_n0, assign92210_e141270_d_n2, assign92210_e141270_d_n4, assign92210_e141270_d_n5, assign92210_e141270_d_n6, assign92210_e141270_d_n7, assign92210_e141270_d_n8, assign92210_e141270_d_n9, assign92210_e141270_d_n10, assign92210_e141270_d_n11, assign92210_e141270_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2146 == 0.0)) {
        let assign92210_e141261: f64 = (locals.var_tq * locals.var_tq);
        let assign92210_e141264: f64 = (locals.var_tp * locals.var_tp);
        let assign92210_e141266: f64 = (assign92210_e141264 * locals.var_tp);
        let assign92210_e141267: f64 = (assign92210_e141261 + assign92210_e141266);
        let assign92210_e141268: f64 = (assign92210_e141267).sqrt();
        (assign92210_e141268, ((((locals.var_tq_dn0 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn0)) + ((((locals.var_tp_dn0 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn0)) * locals.var_tp) + (assign92210_e141264 * locals.var_tp_dn0))) / (2.0 * assign92210_e141268)), ((((locals.var_tq_dn2 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn2)) + ((((locals.var_tp_dn2 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn2)) * locals.var_tp) + (assign92210_e141264 * locals.var_tp_dn2))) / (2.0 * assign92210_e141268)), ((((locals.var_tq_dn4 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn4)) + ((((locals.var_tp_dn4 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn4)) * locals.var_tp) + (assign92210_e141264 * locals.var_tp_dn4))) / (2.0 * assign92210_e141268)), ((((locals.var_tq_dn5 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn5)) + ((((locals.var_tp_dn5 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn5)) * locals.var_tp) + (assign92210_e141264 * locals.var_tp_dn5))) / (2.0 * assign92210_e141268)), ((((locals.var_tq_dn6 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn6)) + ((((locals.var_tp_dn6 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn6)) * locals.var_tp) + (assign92210_e141264 * locals.var_tp_dn6))) / (2.0 * assign92210_e141268)), ((((locals.var_tq_dn7 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn7)) + ((((locals.var_tp_dn7 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn7)) * locals.var_tp) + (assign92210_e141264 * locals.var_tp_dn7))) / (2.0 * assign92210_e141268)), ((((locals.var_tq_dn8 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn8)) + ((((locals.var_tp_dn8 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn8)) * locals.var_tp) + (assign92210_e141264 * locals.var_tp_dn8))) / (2.0 * assign92210_e141268)), ((((locals.var_tq_dn9 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn9)) + ((((locals.var_tp_dn9 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn9)) * locals.var_tp) + (assign92210_e141264 * locals.var_tp_dn9))) / (2.0 * assign92210_e141268)), ((((locals.var_tq_dn10 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn10)) + ((((locals.var_tp_dn10 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn10)) * locals.var_tp) + (assign92210_e141264 * locals.var_tp_dn10))) / (2.0 * assign92210_e141268)), ((((locals.var_tq_dn11 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn11)) + ((((locals.var_tp_dn11 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn11)) * locals.var_tp) + (assign92210_e141264 * locals.var_tp_dn11))) / (2.0 * assign92210_e141268)), ((((locals.var_tq_dn14 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn14)) + ((((locals.var_tp_dn14 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn14)) * locals.var_tp) + (assign92210_e141264 * locals.var_tp_dn14))) / (2.0 * assign92210_e141268)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign92210_e141270;
        locals.var_t5_dn0 = assign92210_e141270_d_n0;
        locals.var_t5_dn2 = assign92210_e141270_d_n2;
        locals.var_t5_dn4 = assign92210_e141270_d_n4;
        locals.var_t5_dn5 = assign92210_e141270_d_n5;
        locals.var_t5_dn6 = assign92210_e141270_d_n6;
        locals.var_t5_dn7 = assign92210_e141270_d_n7;
        locals.var_t5_dn8 = assign92210_e141270_d_n8;
        locals.var_t5_dn9 = assign92210_e141270_d_n9;
        locals.var_t5_dn10 = assign92210_e141270_d_n10;
        locals.var_t5_dn11 = assign92210_e141270_d_n11;
        locals.var_t5_dn14 = assign92210_e141270_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign92220_e141287, assign92220_e141287_d_n0, assign92220_e141287_d_n2, assign92220_e141287_d_n4, assign92220_e141287_d_n5, assign92220_e141287_d_n6, assign92220_e141287_d_n7, assign92220_e141287_d_n8, assign92220_e141287_d_n9, assign92220_e141287_d_n10, assign92220_e141287_d_n11, assign92220_e141287_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2146 == 0.0)) {
        let assign92220_e141281: f64 = (-locals.var_tq);
        let assign92220_e141283: f64 = (assign92220_e141281 + locals.var_t5);
        let assign92220_e141285: f64 = (assign92220_e141283).powf(0.3333333333333333);
        (assign92220_e141285, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign92220_e141283).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn0) + locals.var_t5_dn0))) } } else { (assign92220_e141285 * (0.3333333333333333 * (((-locals.var_tq_dn0) + locals.var_t5_dn0) / assign92220_e141283))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign92220_e141283).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn2) + locals.var_t5_dn2))) } } else { (assign92220_e141285 * (0.3333333333333333 * (((-locals.var_tq_dn2) + locals.var_t5_dn2) / assign92220_e141283))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign92220_e141283).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn4) + locals.var_t5_dn4))) } } else { (assign92220_e141285 * (0.3333333333333333 * (((-locals.var_tq_dn4) + locals.var_t5_dn4) / assign92220_e141283))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign92220_e141283).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn5) + locals.var_t5_dn5))) } } else { (assign92220_e141285 * (0.3333333333333333 * (((-locals.var_tq_dn5) + locals.var_t5_dn5) / assign92220_e141283))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign92220_e141283).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn6) + locals.var_t5_dn6))) } } else { (assign92220_e141285 * (0.3333333333333333 * (((-locals.var_tq_dn6) + locals.var_t5_dn6) / assign92220_e141283))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign92220_e141283).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn7) + locals.var_t5_dn7))) } } else { (assign92220_e141285 * (0.3333333333333333 * (((-locals.var_tq_dn7) + locals.var_t5_dn7) / assign92220_e141283))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign92220_e141283).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn8) + locals.var_t5_dn8))) } } else { (assign92220_e141285 * (0.3333333333333333 * (((-locals.var_tq_dn8) + locals.var_t5_dn8) / assign92220_e141283))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign92220_e141283).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn9) + locals.var_t5_dn9))) } } else { (assign92220_e141285 * (0.3333333333333333 * (((-locals.var_tq_dn9) + locals.var_t5_dn9) / assign92220_e141283))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign92220_e141283).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn10) + locals.var_t5_dn10))) } } else { (assign92220_e141285 * (0.3333333333333333 * (((-locals.var_tq_dn10) + locals.var_t5_dn10) / assign92220_e141283))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign92220_e141283).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn11) + locals.var_t5_dn11))) } } else { (assign92220_e141285 * (0.3333333333333333 * (((-locals.var_tq_dn11) + locals.var_t5_dn11) / assign92220_e141283))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign92220_e141283).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn14) + locals.var_t5_dn14))) } } else { (assign92220_e141285 * (0.3333333333333333 * (((-locals.var_tq_dn14) + locals.var_t5_dn14) / assign92220_e141283))) },)
    } else {
        (locals.var_tu, locals.var_tu_dn0, locals.var_tu_dn2, locals.var_tu_dn4, locals.var_tu_dn5, locals.var_tu_dn6, locals.var_tu_dn7, locals.var_tu_dn8, locals.var_tu_dn9, locals.var_tu_dn10, locals.var_tu_dn11, locals.var_tu_dn14,)
    }
};
        locals.var_tu = assign92220_e141287;
        locals.var_tu_dn0 = assign92220_e141287_d_n0;
        locals.var_tu_dn2 = assign92220_e141287_d_n2;
        locals.var_tu_dn4 = assign92220_e141287_d_n4;
        locals.var_tu_dn5 = assign92220_e141287_d_n5;
        locals.var_tu_dn6 = assign92220_e141287_d_n6;
        locals.var_tu_dn7 = assign92220_e141287_d_n7;
        locals.var_tu_dn8 = assign92220_e141287_d_n8;
        locals.var_tu_dn9 = assign92220_e141287_d_n9;
        locals.var_tu_dn10 = assign92220_e141287_d_n10;
        locals.var_tu_dn11 = assign92220_e141287_d_n11;
        locals.var_tu_dn14 = assign92220_e141287_d_n14;
        locals.var_tu_rv = 0.0;

        let (assign92230_e141304, assign92230_e141304_d_n0, assign92230_e141304_d_n2, assign92230_e141304_d_n4, assign92230_e141304_d_n5, assign92230_e141304_d_n6, assign92230_e141304_d_n7, assign92230_e141304_d_n8, assign92230_e141304_d_n9, assign92230_e141304_d_n10, assign92230_e141304_d_n11, assign92230_e141304_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2146 == 0.0)) {
        let assign92230_e141299: f64 = (locals.var_tq + locals.var_t5);
        let assign92230_e141301: f64 = (assign92230_e141299).powf(0.3333333333333333);
        let assign92230_e141302: f64 = (-assign92230_e141301);
        (assign92230_e141302, (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign92230_e141299).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn0 + locals.var_t5_dn0))) } } else { (assign92230_e141301 * (0.3333333333333333 * ((locals.var_tq_dn0 + locals.var_t5_dn0) / assign92230_e141299))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign92230_e141299).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn2 + locals.var_t5_dn2))) } } else { (assign92230_e141301 * (0.3333333333333333 * ((locals.var_tq_dn2 + locals.var_t5_dn2) / assign92230_e141299))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign92230_e141299).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn4 + locals.var_t5_dn4))) } } else { (assign92230_e141301 * (0.3333333333333333 * ((locals.var_tq_dn4 + locals.var_t5_dn4) / assign92230_e141299))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign92230_e141299).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn5 + locals.var_t5_dn5))) } } else { (assign92230_e141301 * (0.3333333333333333 * ((locals.var_tq_dn5 + locals.var_t5_dn5) / assign92230_e141299))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign92230_e141299).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn6 + locals.var_t5_dn6))) } } else { (assign92230_e141301 * (0.3333333333333333 * ((locals.var_tq_dn6 + locals.var_t5_dn6) / assign92230_e141299))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign92230_e141299).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn7 + locals.var_t5_dn7))) } } else { (assign92230_e141301 * (0.3333333333333333 * ((locals.var_tq_dn7 + locals.var_t5_dn7) / assign92230_e141299))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign92230_e141299).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn8 + locals.var_t5_dn8))) } } else { (assign92230_e141301 * (0.3333333333333333 * ((locals.var_tq_dn8 + locals.var_t5_dn8) / assign92230_e141299))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign92230_e141299).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn9 + locals.var_t5_dn9))) } } else { (assign92230_e141301 * (0.3333333333333333 * ((locals.var_tq_dn9 + locals.var_t5_dn9) / assign92230_e141299))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign92230_e141299).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn10 + locals.var_t5_dn10))) } } else { (assign92230_e141301 * (0.3333333333333333 * ((locals.var_tq_dn10 + locals.var_t5_dn10) / assign92230_e141299))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign92230_e141299).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn11 + locals.var_t5_dn11))) } } else { (assign92230_e141301 * (0.3333333333333333 * ((locals.var_tq_dn11 + locals.var_t5_dn11) / assign92230_e141299))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign92230_e141299).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn14 + locals.var_t5_dn14))) } } else { (assign92230_e141301 * (0.3333333333333333 * ((locals.var_tq_dn14 + locals.var_t5_dn14) / assign92230_e141299))) }),)
    } else {
        (locals.var_tv, locals.var_tv_dn0, locals.var_tv_dn2, locals.var_tv_dn4, locals.var_tv_dn5, locals.var_tv_dn6, locals.var_tv_dn7, locals.var_tv_dn8, locals.var_tv_dn9, locals.var_tv_dn10, locals.var_tv_dn11, locals.var_tv_dn14,)
    }
};
        locals.var_tv = assign92230_e141304;
        locals.var_tv_dn0 = assign92230_e141304_d_n0;
        locals.var_tv_dn2 = assign92230_e141304_d_n2;
        locals.var_tv_dn4 = assign92230_e141304_d_n4;
        locals.var_tv_dn5 = assign92230_e141304_d_n5;
        locals.var_tv_dn6 = assign92230_e141304_d_n6;
        locals.var_tv_dn7 = assign92230_e141304_d_n7;
        locals.var_tv_dn8 = assign92230_e141304_d_n8;
        locals.var_tv_dn9 = assign92230_e141304_d_n9;
        locals.var_tv_dn10 = assign92230_e141304_d_n10;
        locals.var_tv_dn11 = assign92230_e141304_d_n11;
        locals.var_tv_dn14 = assign92230_e141304_d_n14;
        locals.var_tv_rv = 0.0;

        let (assign92240_e141324, assign92240_e141324_d_n0, assign92240_e141324_d_n2, assign92240_e141324_d_n4, assign92240_e141324_d_n5, assign92240_e141324_d_n6, assign92240_e141324_d_n7, assign92240_e141324_d_n8, assign92240_e141324_d_n9, assign92240_e141324_d_n10, assign92240_e141324_d_n11, assign92240_e141324_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2146 == 0.0)) {
        let assign92240_e141316: f64 = (locals.var_tu + locals.var_tv);
        let assign92240_e141320: f64 = (3.0 * locals.var_ta);
        let assign92240_e141321: f64 = (locals.var_tb / assign92240_e141320);
        let assign92240_e141322: f64 = (assign92240_e141316 - assign92240_e141321);
        (assign92240_e141322, (locals.var_tu_dn0 + locals.var_tv_dn0), (locals.var_tu_dn2 + locals.var_tv_dn2), (locals.var_tu_dn4 + locals.var_tv_dn4), (locals.var_tu_dn5 + locals.var_tv_dn5), (locals.var_tu_dn6 + locals.var_tv_dn6), (locals.var_tu_dn7 + locals.var_tv_dn7), (locals.var_tu_dn8 + locals.var_tv_dn8), (locals.var_tu_dn9 + locals.var_tv_dn9), (locals.var_tu_dn10 + locals.var_tv_dn10), (locals.var_tu_dn11 + locals.var_tv_dn11), (locals.var_tu_dn14 + locals.var_tv_dn14),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
        locals.var_chi = assign92240_e141324;
        locals.var_chi_dn0 = assign92240_e141324_d_n0;
        locals.var_chi_dn2 = assign92240_e141324_d_n2;
        locals.var_chi_dn4 = assign92240_e141324_d_n4;
        locals.var_chi_dn5 = assign92240_e141324_d_n5;
        locals.var_chi_dn6 = assign92240_e141324_d_n6;
        locals.var_chi_dn7 = assign92240_e141324_d_n7;
        locals.var_chi_dn8 = assign92240_e141324_d_n8;
        locals.var_chi_dn9 = assign92240_e141324_d_n9;
        locals.var_chi_dn10 = assign92240_e141324_d_n10;
        locals.var_chi_dn11 = assign92240_e141324_d_n11;
        locals.var_chi_dn14 = assign92240_e141324_d_n14;
        locals.var_chi_rv = 0.0;

        let (assign92250_e141340, assign92250_e141340_d_n0, assign92250_e141340_d_n2, assign92250_e141340_d_n4, assign92250_e141340_d_n5, assign92250_e141340_d_n6, assign92250_e141340_d_n7, assign92250_e141340_d_n8, assign92250_e141340_d_n9, assign92250_e141340_d_n10, assign92250_e141340_d_n11, assign92250_e141340_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2146 == 0.0)) {
        let assign92250_e141336: f64 = (locals.var_chi * locals.var_beta_inv);
        let assign92250_e141338: f64 = (assign92250_e141336 - locals.var_vxbgmtcl);
        (assign92250_e141338, (((locals.var_chi_dn0 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn0)) - locals.var_vxbgmtcl_dn0), (((locals.var_chi_dn2 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn2)) - locals.var_vxbgmtcl_dn2), (((locals.var_chi_dn4 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn4)) - locals.var_vxbgmtcl_dn4), (((locals.var_chi_dn5 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn5)) - locals.var_vxbgmtcl_dn5), (((locals.var_chi_dn6 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn6)) - locals.var_vxbgmtcl_dn6), (((locals.var_chi_dn7 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn7)) - locals.var_vxbgmtcl_dn7), (((locals.var_chi_dn8 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn8)) - locals.var_vxbgmtcl_dn8), (((locals.var_chi_dn9 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn9)) - locals.var_vxbgmtcl_dn9), (((locals.var_chi_dn10 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn10)) - locals.var_vxbgmtcl_dn10), (((locals.var_chi_dn11 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn11)) - locals.var_vxbgmtcl_dn11), (((locals.var_chi_dn14 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn14)) - locals.var_vxbgmtcl_dn14),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn14,)
    }
};
        locals.var_ps0_inia = assign92250_e141340;
        locals.var_ps0_inia_dn0 = assign92250_e141340_d_n0;
        locals.var_ps0_inia_dn2 = assign92250_e141340_d_n2;
        locals.var_ps0_inia_dn4 = assign92250_e141340_d_n4;
        locals.var_ps0_inia_dn5 = assign92250_e141340_d_n5;
        locals.var_ps0_inia_dn6 = assign92250_e141340_d_n6;
        locals.var_ps0_inia_dn7 = assign92250_e141340_d_n7;
        locals.var_ps0_inia_dn8 = assign92250_e141340_d_n8;
        locals.var_ps0_inia_dn9 = assign92250_e141340_d_n9;
        locals.var_ps0_inia_dn10 = assign92250_e141340_d_n10;
        locals.var_ps0_inia_dn11 = assign92250_e141340_d_n11;
        locals.var_ps0_inia_dn14 = assign92250_e141340_d_n14;
        locals.var_ps0_inia_rv = 0.0;

        let assign92260_e141343: f64 = if p.p33 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2147 = assign92260_e141343;
        locals.var_guard2147_rv = 0.0;

        let (assign92270_e141358, assign92270_e141358_d_n0, assign92270_e141358_d_n2, assign92270_e141358_d_n4, assign92270_e141358_d_n5, assign92270_e141358_d_n6, assign92270_e141358_d_n7, assign92270_e141358_d_n8, assign92270_e141358_d_n9, assign92270_e141358_d_n10, assign92270_e141358_d_n11, assign92270_e141358_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2147 != 0.0)) {
        let assign92270_e141354: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign92270_e141356: f64 = (assign92270_e141354 + 0.1);
        (assign92270_e141356, locals.var_vxbgmtcl_dn0, (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2), locals.var_vxbgmtcl_dn4, locals.var_vxbgmtcl_dn5, locals.var_vxbgmtcl_dn6, (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7), (locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8), (locals.var_vgpld_dn9 + locals.var_vxbgmtcl_dn9), locals.var_vxbgmtcl_dn10, locals.var_vxbgmtcl_dn11, locals.var_vxbgmtcl_dn14,)
    } else {
        (locals.var_vgpld_shift, locals.var_vgpld_shift_dn0, locals.var_vgpld_shift_dn2, locals.var_vgpld_shift_dn4, locals.var_vgpld_shift_dn5, locals.var_vgpld_shift_dn6, locals.var_vgpld_shift_dn7, locals.var_vgpld_shift_dn8, locals.var_vgpld_shift_dn9, locals.var_vgpld_shift_dn10, locals.var_vgpld_shift_dn11, locals.var_vgpld_shift_dn14,)
    }
};
        locals.var_vgpld_shift = assign92270_e141358;
        locals.var_vgpld_shift_dn0 = assign92270_e141358_d_n0;
        locals.var_vgpld_shift_dn2 = assign92270_e141358_d_n2;
        locals.var_vgpld_shift_dn4 = assign92270_e141358_d_n4;
        locals.var_vgpld_shift_dn5 = assign92270_e141358_d_n5;
        locals.var_vgpld_shift_dn6 = assign92270_e141358_d_n6;
        locals.var_vgpld_shift_dn7 = assign92270_e141358_d_n7;
        locals.var_vgpld_shift_dn8 = assign92270_e141358_d_n8;
        locals.var_vgpld_shift_dn9 = assign92270_e141358_d_n9;
        locals.var_vgpld_shift_dn10 = assign92270_e141358_d_n10;
        locals.var_vgpld_shift_dn11 = assign92270_e141358_d_n11;
        locals.var_vgpld_shift_dn14 = assign92270_e141358_d_n14;
        locals.var_vgpld_shift_rv = 0.0;

        let (assign92280_e141371, assign92280_e141371_d_n0, assign92280_e141371_d_n2, assign92280_e141371_d_n4, assign92280_e141371_d_n5, assign92280_e141371_d_n6, assign92280_e141371_d_n7, assign92280_e141371_d_n8, assign92280_e141371_d_n9, assign92280_e141371_d_n10, assign92280_e141371_d_n11, assign92280_e141371_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2147 != 0.0)) {
        let assign92280_e141369: f64 = (locals.var_cnst1over * locals.var_exp_bvbs);
        (assign92280_e141369, ((locals.var_cnst1over_dn0 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn0)), ((locals.var_cnst1over_dn2 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn2)), ((locals.var_cnst1over_dn4 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn4)), ((locals.var_cnst1over_dn5 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn5)), ((locals.var_cnst1over_dn6 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn6)), ((locals.var_cnst1over_dn7 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn7)), ((locals.var_cnst1over_dn8 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn8)), ((locals.var_cnst1over_dn9 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn9)), ((locals.var_cnst1over_dn10 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn10)), ((locals.var_cnst1over_dn11 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn11)), ((locals.var_cnst1over_dn14 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn14)),)
    } else {
        (locals.var_cfs1, locals.var_cfs1_dn0, locals.var_cfs1_dn2, locals.var_cfs1_dn4, locals.var_cfs1_dn5, locals.var_cfs1_dn6, locals.var_cfs1_dn7, locals.var_cfs1_dn8, locals.var_cfs1_dn9, locals.var_cfs1_dn10, locals.var_cfs1_dn11, locals.var_cfs1_dn14,)
    }
};
        locals.var_cfs1 = assign92280_e141371;
        locals.var_cfs1_dn0 = assign92280_e141371_d_n0;
        locals.var_cfs1_dn2 = assign92280_e141371_d_n2;
        locals.var_cfs1_dn4 = assign92280_e141371_d_n4;
        locals.var_cfs1_dn5 = assign92280_e141371_d_n5;
        locals.var_cfs1_dn6 = assign92280_e141371_d_n6;
        locals.var_cfs1_dn7 = assign92280_e141371_d_n7;
        locals.var_cfs1_dn8 = assign92280_e141371_d_n8;
        locals.var_cfs1_dn9 = assign92280_e141371_d_n9;
        locals.var_cfs1_dn10 = assign92280_e141371_d_n10;
        locals.var_cfs1_dn11 = assign92280_e141371_d_n11;
        locals.var_cfs1_dn14 = assign92280_e141371_d_n14;
        locals.var_cfs1_rv = 0.0;

        let (assign92290_e141384, assign92290_e141384_d_n0, assign92290_e141384_d_n2, assign92290_e141384_d_n4, assign92290_e141384_d_n5, assign92290_e141384_d_n6, assign92290_e141384_d_n7, assign92290_e141384_d_n8, assign92290_e141384_d_n9, assign92290_e141384_d_n10, assign92290_e141384_d_n11, assign92290_e141384_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2147 != 0.0)) {
        let assign92290_e141382: f64 = (locals.var_cnst1over * locals.var_exp_bvbs);
        (assign92290_e141382, ((locals.var_cnst1over_dn0 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn0)), ((locals.var_cnst1over_dn2 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn2)), ((locals.var_cnst1over_dn4 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn4)), ((locals.var_cnst1over_dn5 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn5)), ((locals.var_cnst1over_dn6 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn6)), ((locals.var_cnst1over_dn7 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn7)), ((locals.var_cnst1over_dn8 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn8)), ((locals.var_cnst1over_dn9 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn9)), ((locals.var_cnst1over_dn10 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn10)), ((locals.var_cnst1over_dn11 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn11)), ((locals.var_cnst1over_dn14 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn14)),)
    } else {
        (locals.var_gammachi, locals.var_gammachi_dn0, locals.var_gammachi_dn2, locals.var_gammachi_dn4, locals.var_gammachi_dn5, locals.var_gammachi_dn6, locals.var_gammachi_dn7, locals.var_gammachi_dn8, locals.var_gammachi_dn9, locals.var_gammachi_dn10, locals.var_gammachi_dn11, locals.var_gammachi_dn14,)
    }
};
        locals.var_gammachi = assign92290_e141384;
        locals.var_gammachi_dn0 = assign92290_e141384_d_n0;
        locals.var_gammachi_dn2 = assign92290_e141384_d_n2;
        locals.var_gammachi_dn4 = assign92290_e141384_d_n4;
        locals.var_gammachi_dn5 = assign92290_e141384_d_n5;
        locals.var_gammachi_dn6 = assign92290_e141384_d_n6;
        locals.var_gammachi_dn7 = assign92290_e141384_d_n7;
        locals.var_gammachi_dn8 = assign92290_e141384_d_n8;
        locals.var_gammachi_dn9 = assign92290_e141384_d_n9;
        locals.var_gammachi_dn10 = assign92290_e141384_d_n10;
        locals.var_gammachi_dn11 = assign92290_e141384_d_n11;
        locals.var_gammachi_dn14 = assign92290_e141384_d_n14;
        locals.var_gammachi_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_356(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign92300_e141397, assign92300_e141397_d_n0, assign92300_e141397_d_n2, assign92300_e141397_d_n4, assign92300_e141397_d_n5, assign92300_e141397_d_n6, assign92300_e141397_d_n7, assign92300_e141397_d_n8, assign92300_e141397_d_n9, assign92300_e141397_d_n10, assign92300_e141397_d_n11, assign92300_e141397_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2147 != 0.0)) {
        let assign92300_e141395: f64 = (locals.var_beta2 * locals.var_fac1p2);
        (assign92300_e141395, ((locals.var_beta2_dn0 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn0)), ((locals.var_beta2_dn2 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn2)), ((locals.var_beta2_dn4 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn4)), ((locals.var_beta2_dn5 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn5)), ((locals.var_beta2_dn6 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn6)), ((locals.var_beta2_dn7 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn7)), ((locals.var_beta2_dn8 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn8)), ((locals.var_beta2_dn9 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn9)), ((locals.var_beta2_dn10 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn10)), ((locals.var_beta2_dn11 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn11)), ((locals.var_beta2_dn14 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign92300_e141397;
        locals.var_t0_dn0 = assign92300_e141397_d_n0;
        locals.var_t0_dn2 = assign92300_e141397_d_n2;
        locals.var_t0_dn4 = assign92300_e141397_d_n4;
        locals.var_t0_dn5 = assign92300_e141397_d_n5;
        locals.var_t0_dn6 = assign92300_e141397_d_n6;
        locals.var_t0_dn7 = assign92300_e141397_d_n7;
        locals.var_t0_dn8 = assign92300_e141397_d_n8;
        locals.var_t0_dn9 = assign92300_e141397_d_n9;
        locals.var_t0_dn10 = assign92300_e141397_d_n10;
        locals.var_t0_dn11 = assign92300_e141397_d_n11;
        locals.var_t0_dn14 = assign92300_e141397_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign92310_e141410, assign92310_e141410_d_n0, assign92310_e141410_d_n2, assign92310_e141410_d_n4, assign92310_e141410_d_n5, assign92310_e141410_d_n6, assign92310_e141410_d_n7, assign92310_e141410_d_n8, assign92310_e141410_d_n9, assign92310_e141410_d_n10, assign92310_e141410_d_n11, assign92310_e141410_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2147 != 0.0)) {
        let assign92310_e141408: f64 = (locals.var_beta * locals.var_vgpld_shift);
        (assign92310_e141408, ((locals.var_beta_dn0 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn0)), ((locals.var_beta_dn2 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn2)), ((locals.var_beta_dn4 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn4)), ((locals.var_beta_dn5 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn5)), ((locals.var_beta_dn6 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn6)), ((locals.var_beta_dn7 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn7)), ((locals.var_beta_dn8 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn8)), ((locals.var_beta_dn9 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn9)), ((locals.var_beta_dn10 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn10)), ((locals.var_beta_dn11 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn11)), ((locals.var_beta_dn14 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn14)),)
    } else {
        (locals.var_psi, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn4, locals.var_psi_dn5, locals.var_psi_dn6, locals.var_psi_dn7, locals.var_psi_dn8, locals.var_psi_dn9, locals.var_psi_dn10, locals.var_psi_dn11, locals.var_psi_dn14,)
    }
};
        locals.var_psi = assign92310_e141410;
        locals.var_psi_dn0 = assign92310_e141410_d_n0;
        locals.var_psi_dn2 = assign92310_e141410_d_n2;
        locals.var_psi_dn4 = assign92310_e141410_d_n4;
        locals.var_psi_dn5 = assign92310_e141410_d_n5;
        locals.var_psi_dn6 = assign92310_e141410_d_n6;
        locals.var_psi_dn7 = assign92310_e141410_d_n7;
        locals.var_psi_dn8 = assign92310_e141410_d_n8;
        locals.var_psi_dn9 = assign92310_e141410_d_n9;
        locals.var_psi_dn10 = assign92310_e141410_d_n10;
        locals.var_psi_dn11 = assign92310_e141410_d_n11;
        locals.var_psi_dn14 = assign92310_e141410_d_n14;
        locals.var_psi_rv = 0.0;

        let (assign92320_e141437, assign92320_e141437_d_n0, assign92320_e141437_d_n2, assign92320_e141437_d_n4, assign92320_e141437_d_n5, assign92320_e141437_d_n6, assign92320_e141437_d_n7, assign92320_e141437_d_n8, assign92320_e141437_d_n9, assign92320_e141437_d_n10, assign92320_e141437_d_n11, assign92320_e141437_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2147 != 0.0)) {
        let assign92320_e141421: f64 = (locals.var_gammachi * locals.var_t0);
        let assign92320_e141424: f64 = (locals.var_psi * locals.var_psi);
        let assign92320_e141425: f64 = (assign92320_e141421 + assign92320_e141424);
        let assign92320_e141426: f64 = (assign92320_e141425).ln();
        let assign92320_e141429: f64 = (locals.var_cnst1over * locals.var_t0);
        let assign92320_e141430: f64 = (assign92320_e141429).ln();
        let assign92320_e141431: f64 = (assign92320_e141426 - assign92320_e141430);
        let assign92320_e141434: f64 = (locals.var_beta * locals.var_vxbgmtcl);
        let assign92320_e141435: f64 = (assign92320_e141431 + assign92320_e141434);
        (assign92320_e141435, ((((((locals.var_gammachi_dn0 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn0)) + ((locals.var_psi_dn0 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn0))) / assign92320_e141425) - (((locals.var_cnst1over_dn0 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn0)) / assign92320_e141429)) + ((locals.var_beta_dn0 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn0))), ((((((locals.var_gammachi_dn2 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn2)) + ((locals.var_psi_dn2 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn2))) / assign92320_e141425) - (((locals.var_cnst1over_dn2 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn2)) / assign92320_e141429)) + ((locals.var_beta_dn2 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn2))), ((((((locals.var_gammachi_dn4 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn4)) + ((locals.var_psi_dn4 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn4))) / assign92320_e141425) - (((locals.var_cnst1over_dn4 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn4)) / assign92320_e141429)) + ((locals.var_beta_dn4 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn4))), ((((((locals.var_gammachi_dn5 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn5)) + ((locals.var_psi_dn5 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn5))) / assign92320_e141425) - (((locals.var_cnst1over_dn5 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn5)) / assign92320_e141429)) + ((locals.var_beta_dn5 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn5))), ((((((locals.var_gammachi_dn6 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn6)) + ((locals.var_psi_dn6 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn6))) / assign92320_e141425) - (((locals.var_cnst1over_dn6 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn6)) / assign92320_e141429)) + ((locals.var_beta_dn6 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn6))), ((((((locals.var_gammachi_dn7 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn7)) + ((locals.var_psi_dn7 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn7))) / assign92320_e141425) - (((locals.var_cnst1over_dn7 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn7)) / assign92320_e141429)) + ((locals.var_beta_dn7 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn7))), ((((((locals.var_gammachi_dn8 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn8)) + ((locals.var_psi_dn8 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn8))) / assign92320_e141425) - (((locals.var_cnst1over_dn8 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn8)) / assign92320_e141429)) + ((locals.var_beta_dn8 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn8))), ((((((locals.var_gammachi_dn9 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn9)) + ((locals.var_psi_dn9 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn9))) / assign92320_e141425) - (((locals.var_cnst1over_dn9 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn9)) / assign92320_e141429)) + ((locals.var_beta_dn9 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn9))), ((((((locals.var_gammachi_dn10 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn10)) + ((locals.var_psi_dn10 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn10))) / assign92320_e141425) - (((locals.var_cnst1over_dn10 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn10)) / assign92320_e141429)) + ((locals.var_beta_dn10 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn10))), ((((((locals.var_gammachi_dn11 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn11)) + ((locals.var_psi_dn11 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn11))) / assign92320_e141425) - (((locals.var_cnst1over_dn11 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn11)) / assign92320_e141429)) + ((locals.var_beta_dn11 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn11))), ((((((locals.var_gammachi_dn14 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn14)) + ((locals.var_psi_dn14 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn14))) / assign92320_e141425) - (((locals.var_cnst1over_dn14 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn14)) / assign92320_e141429)) + ((locals.var_beta_dn14 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn14))),)
    } else {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn11, locals.var_chi_1_dn14,)
    }
};
        locals.var_chi_1 = assign92320_e141437;
        locals.var_chi_1_dn0 = assign92320_e141437_d_n0;
        locals.var_chi_1_dn2 = assign92320_e141437_d_n2;
        locals.var_chi_1_dn4 = assign92320_e141437_d_n4;
        locals.var_chi_1_dn5 = assign92320_e141437_d_n5;
        locals.var_chi_1_dn6 = assign92320_e141437_d_n6;
        locals.var_chi_1_dn7 = assign92320_e141437_d_n7;
        locals.var_chi_1_dn8 = assign92320_e141437_d_n8;
        locals.var_chi_1_dn9 = assign92320_e141437_d_n9;
        locals.var_chi_1_dn10 = assign92320_e141437_d_n10;
        locals.var_chi_1_dn11 = assign92320_e141437_d_n11;
        locals.var_chi_1_dn14 = assign92320_e141437_d_n14;
        locals.var_chi_1_rv = 0.0;

        let assign92330_e141440: f64 = if p.p33 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard2148 = assign92330_e141440;
        locals.var_guard2148_rv = 0.0;

        let (assign92340_e141457, assign92340_e141457_d_n0, assign92340_e141457_d_n2, assign92340_e141457_d_n4, assign92340_e141457_d_n5, assign92340_e141457_d_n6, assign92340_e141457_d_n7, assign92340_e141457_d_n8, assign92340_e141457_d_n9, assign92340_e141457_d_n10, assign92340_e141457_d_n11, assign92340_e141457_d_n14,) = {
    if (((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2147 != 0.0)) && (locals.var_guard2148 != 0.0)) {
        let assign92340_e141453: f64 = (locals.var_psi - locals.var_chi_1);
        let assign92340_e141455: f64 = (assign92340_e141453 - 1.0);
        (assign92340_e141455, (locals.var_psi_dn0 - locals.var_chi_1_dn0), (locals.var_psi_dn2 - locals.var_chi_1_dn2), (locals.var_psi_dn4 - locals.var_chi_1_dn4), (locals.var_psi_dn5 - locals.var_chi_1_dn5), (locals.var_psi_dn6 - locals.var_chi_1_dn6), (locals.var_psi_dn7 - locals.var_chi_1_dn7), (locals.var_psi_dn8 - locals.var_chi_1_dn8), (locals.var_psi_dn9 - locals.var_chi_1_dn9), (locals.var_psi_dn10 - locals.var_chi_1_dn10), (locals.var_psi_dn11 - locals.var_chi_1_dn11), (locals.var_psi_dn14 - locals.var_chi_1_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign92340_e141457;
        locals.var_tmf1_dn0 = assign92340_e141457_d_n0;
        locals.var_tmf1_dn2 = assign92340_e141457_d_n2;
        locals.var_tmf1_dn4 = assign92340_e141457_d_n4;
        locals.var_tmf1_dn5 = assign92340_e141457_d_n5;
        locals.var_tmf1_dn6 = assign92340_e141457_d_n6;
        locals.var_tmf1_dn7 = assign92340_e141457_d_n7;
        locals.var_tmf1_dn8 = assign92340_e141457_d_n8;
        locals.var_tmf1_dn9 = assign92340_e141457_d_n9;
        locals.var_tmf1_dn10 = assign92340_e141457_d_n10;
        locals.var_tmf1_dn11 = assign92340_e141457_d_n11;
        locals.var_tmf1_dn14 = assign92340_e141457_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign92350_e141474, assign92350_e141474_d_n0, assign92350_e141474_d_n2, assign92350_e141474_d_n4, assign92350_e141474_d_n5, assign92350_e141474_d_n6, assign92350_e141474_d_n7, assign92350_e141474_d_n8, assign92350_e141474_d_n9, assign92350_e141474_d_n10, assign92350_e141474_d_n11, assign92350_e141474_d_n14,) = {
    if (((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2147 != 0.0)) && (locals.var_guard2148 != 0.0)) {
        let assign92350_e141470: f64 = (4.0 * locals.var_psi);
        let assign92350_e141472: f64 = assign92350_e141470;
        (assign92350_e141472, (4.0 * locals.var_psi_dn0), (4.0 * locals.var_psi_dn2), (4.0 * locals.var_psi_dn4), (4.0 * locals.var_psi_dn5), (4.0 * locals.var_psi_dn6), (4.0 * locals.var_psi_dn7), (4.0 * locals.var_psi_dn8), (4.0 * locals.var_psi_dn9), (4.0 * locals.var_psi_dn10), (4.0 * locals.var_psi_dn11), (4.0 * locals.var_psi_dn14),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign92350_e141474;
        locals.var_tmf2_dn0 = assign92350_e141474_d_n0;
        locals.var_tmf2_dn2 = assign92350_e141474_d_n2;
        locals.var_tmf2_dn4 = assign92350_e141474_d_n4;
        locals.var_tmf2_dn5 = assign92350_e141474_d_n5;
        locals.var_tmf2_dn6 = assign92350_e141474_d_n6;
        locals.var_tmf2_dn7 = assign92350_e141474_d_n7;
        locals.var_tmf2_dn8 = assign92350_e141474_d_n8;
        locals.var_tmf2_dn9 = assign92350_e141474_d_n9;
        locals.var_tmf2_dn10 = assign92350_e141474_d_n10;
        locals.var_tmf2_dn11 = assign92350_e141474_d_n11;
        locals.var_tmf2_dn14 = assign92350_e141474_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign92360_e141493, assign92360_e141493_d_n0, assign92360_e141493_d_n2, assign92360_e141493_d_n4, assign92360_e141493_d_n5, assign92360_e141493_d_n6, assign92360_e141493_d_n7, assign92360_e141493_d_n8, assign92360_e141493_d_n9, assign92360_e141493_d_n10, assign92360_e141493_d_n11, assign92360_e141493_d_n14,) = {
    if (((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2147 != 0.0)) && (locals.var_guard2148 != 0.0)) {
        let (assign92360_e141491, assign92360_e141491_d_n0, assign92360_e141491_d_n2, assign92360_e141491_d_n4, assign92360_e141491_d_n5, assign92360_e141491_d_n6, assign92360_e141491_d_n7, assign92360_e141491_d_n8, assign92360_e141491_d_n9, assign92360_e141491_d_n10, assign92360_e141491_d_n11, assign92360_e141491_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign92360_e141490: f64 = (-locals.var_tmf2);
                (assign92360_e141490, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign92360_e141491, assign92360_e141491_d_n0, assign92360_e141491_d_n2, assign92360_e141491_d_n4, assign92360_e141491_d_n5, assign92360_e141491_d_n6, assign92360_e141491_d_n7, assign92360_e141491_d_n8, assign92360_e141491_d_n9, assign92360_e141491_d_n10, assign92360_e141491_d_n11, assign92360_e141491_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign92360_e141493;
        locals.var_tmf2_dn0 = assign92360_e141493_d_n0;
        locals.var_tmf2_dn2 = assign92360_e141493_d_n2;
        locals.var_tmf2_dn4 = assign92360_e141493_d_n4;
        locals.var_tmf2_dn5 = assign92360_e141493_d_n5;
        locals.var_tmf2_dn6 = assign92360_e141493_d_n6;
        locals.var_tmf2_dn7 = assign92360_e141493_d_n7;
        locals.var_tmf2_dn8 = assign92360_e141493_d_n8;
        locals.var_tmf2_dn9 = assign92360_e141493_d_n9;
        locals.var_tmf2_dn10 = assign92360_e141493_d_n10;
        locals.var_tmf2_dn11 = assign92360_e141493_d_n11;
        locals.var_tmf2_dn14 = assign92360_e141493_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign92370_e141511, assign92370_e141511_d_n0, assign92370_e141511_d_n2, assign92370_e141511_d_n4, assign92370_e141511_d_n5, assign92370_e141511_d_n6, assign92370_e141511_d_n7, assign92370_e141511_d_n8, assign92370_e141511_d_n9, assign92370_e141511_d_n10, assign92370_e141511_d_n11, assign92370_e141511_d_n14,) = {
    if (((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2147 != 0.0)) && (locals.var_guard2148 != 0.0)) {
        let assign92370_e141506: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign92370_e141508: f64 = (assign92370_e141506 + locals.var_tmf2);
        let assign92370_e141509: f64 = (assign92370_e141508).sqrt();
        (assign92370_e141509, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign92370_e141509)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign92370_e141509)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign92370_e141509)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign92370_e141509)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign92370_e141509)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign92370_e141509)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign92370_e141509)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign92370_e141509)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign92370_e141509)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign92370_e141509)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign92370_e141509)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign92370_e141511;
        locals.var_tmf2_dn0 = assign92370_e141511_d_n0;
        locals.var_tmf2_dn2 = assign92370_e141511_d_n2;
        locals.var_tmf2_dn4 = assign92370_e141511_d_n4;
        locals.var_tmf2_dn5 = assign92370_e141511_d_n5;
        locals.var_tmf2_dn6 = assign92370_e141511_d_n6;
        locals.var_tmf2_dn7 = assign92370_e141511_d_n7;
        locals.var_tmf2_dn8 = assign92370_e141511_d_n8;
        locals.var_tmf2_dn9 = assign92370_e141511_d_n9;
        locals.var_tmf2_dn10 = assign92370_e141511_d_n10;
        locals.var_tmf2_dn11 = assign92370_e141511_d_n11;
        locals.var_tmf2_dn14 = assign92370_e141511_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign92380_e141530, assign92380_e141530_d_n0, assign92380_e141530_d_n2, assign92380_e141530_d_n4, assign92380_e141530_d_n5, assign92380_e141530_d_n6, assign92380_e141530_d_n7, assign92380_e141530_d_n8, assign92380_e141530_d_n9, assign92380_e141530_d_n10, assign92380_e141530_d_n11, assign92380_e141530_d_n14,) = {
    if (((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2147 != 0.0)) && (locals.var_guard2148 != 0.0)) {
        let assign92380_e141526: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign92380_e141527: f64 = (1.0 + assign92380_e141526);
        let assign92380_e141528: f64 = (0.5 * assign92380_e141527);
        (assign92380_e141528, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign92380_e141530;
        locals.var_t1_dn0 = assign92380_e141530_d_n0;
        locals.var_t1_dn2 = assign92380_e141530_d_n2;
        locals.var_t1_dn4 = assign92380_e141530_d_n4;
        locals.var_t1_dn5 = assign92380_e141530_d_n5;
        locals.var_t1_dn6 = assign92380_e141530_d_n6;
        locals.var_t1_dn7 = assign92380_e141530_d_n7;
        locals.var_t1_dn8 = assign92380_e141530_d_n8;
        locals.var_t1_dn9 = assign92380_e141530_d_n9;
        locals.var_t1_dn10 = assign92380_e141530_d_n10;
        locals.var_t1_dn11 = assign92380_e141530_d_n11;
        locals.var_t1_dn14 = assign92380_e141530_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign92390_e141549, assign92390_e141549_d_n0, assign92390_e141549_d_n2, assign92390_e141549_d_n4, assign92390_e141549_d_n5, assign92390_e141549_d_n6, assign92390_e141549_d_n7, assign92390_e141549_d_n8, assign92390_e141549_d_n9, assign92390_e141549_d_n10, assign92390_e141549_d_n11, assign92390_e141549_d_n14,) = {
    if (((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2147 != 0.0)) && (locals.var_guard2148 != 0.0)) {
        let assign92390_e141545: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign92390_e141546: f64 = (0.5 * assign92390_e141545);
        let assign92390_e141547: f64 = (locals.var_psi - assign92390_e141546);
        (assign92390_e141547, (locals.var_psi_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_psi_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_psi_dn4 - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_psi_dn5 - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_psi_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_psi_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_psi_dn8 - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_psi_dn9 - (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (locals.var_psi_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_psi_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_psi_dn14 - (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn11, locals.var_chi_1_dn14,)
    }
};
        locals.var_chi_1 = assign92390_e141549;
        locals.var_chi_1_dn0 = assign92390_e141549_d_n0;
        locals.var_chi_1_dn2 = assign92390_e141549_d_n2;
        locals.var_chi_1_dn4 = assign92390_e141549_d_n4;
        locals.var_chi_1_dn5 = assign92390_e141549_d_n5;
        locals.var_chi_1_dn6 = assign92390_e141549_d_n6;
        locals.var_chi_1_dn7 = assign92390_e141549_d_n7;
        locals.var_chi_1_dn8 = assign92390_e141549_d_n8;
        locals.var_chi_1_dn9 = assign92390_e141549_d_n9;
        locals.var_chi_1_dn10 = assign92390_e141549_d_n10;
        locals.var_chi_1_dn11 = assign92390_e141549_d_n11;
        locals.var_chi_1_dn14 = assign92390_e141549_d_n14;
        locals.var_chi_1_rv = 0.0;

        let (assign92400_e141568, assign92400_e141568_d_n0, assign92400_e141568_d_n2, assign92400_e141568_d_n4, assign92400_e141568_d_n5, assign92400_e141568_d_n6, assign92400_e141568_d_n7, assign92400_e141568_d_n8, assign92400_e141568_d_n9, assign92400_e141568_d_n10, assign92400_e141568_d_n11, assign92400_e141568_d_n14,) = {
    if (((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2147 != 0.0)) && (locals.var_guard2148 == 0.0)) {
        let (assign92400_e141566, assign92400_e141566_d_n0, assign92400_e141566_d_n2, assign92400_e141566_d_n4, assign92400_e141566_d_n5, assign92400_e141566_d_n6, assign92400_e141566_d_n7, assign92400_e141566_d_n8, assign92400_e141566_d_n9, assign92400_e141566_d_n10, assign92400_e141566_d_n11, assign92400_e141566_d_n14,) = {
            if (locals.var_chi_1 <= locals.var_psi) {
                (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn11, locals.var_chi_1_dn14,)
            } else {
                (locals.var_psi, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn4, locals.var_psi_dn5, locals.var_psi_dn6, locals.var_psi_dn7, locals.var_psi_dn8, locals.var_psi_dn9, locals.var_psi_dn10, locals.var_psi_dn11, locals.var_psi_dn14,)
            }
        };
        (assign92400_e141566, assign92400_e141566_d_n0, assign92400_e141566_d_n2, assign92400_e141566_d_n4, assign92400_e141566_d_n5, assign92400_e141566_d_n6, assign92400_e141566_d_n7, assign92400_e141566_d_n8, assign92400_e141566_d_n9, assign92400_e141566_d_n10, assign92400_e141566_d_n11, assign92400_e141566_d_n14,)
    } else {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn11, locals.var_chi_1_dn14,)
    }
};
        locals.var_chi_1 = assign92400_e141568;
        locals.var_chi_1_dn0 = assign92400_e141568_d_n0;
        locals.var_chi_1_dn2 = assign92400_e141568_d_n2;
        locals.var_chi_1_dn4 = assign92400_e141568_d_n4;
        locals.var_chi_1_dn5 = assign92400_e141568_d_n5;
        locals.var_chi_1_dn6 = assign92400_e141568_d_n6;
        locals.var_chi_1_dn7 = assign92400_e141568_d_n7;
        locals.var_chi_1_dn8 = assign92400_e141568_d_n8;
        locals.var_chi_1_dn9 = assign92400_e141568_d_n9;
        locals.var_chi_1_dn10 = assign92400_e141568_d_n10;
        locals.var_chi_1_dn11 = assign92400_e141568_d_n11;
        locals.var_chi_1_dn14 = assign92400_e141568_d_n14;
        locals.var_chi_1_rv = 0.0;

        let (assign92410_e141584, assign92410_e141584_d_n0, assign92410_e141584_d_n2, assign92410_e141584_d_n4, assign92410_e141584_d_n5, assign92410_e141584_d_n6, assign92410_e141584_d_n7, assign92410_e141584_d_n8, assign92410_e141584_d_n9, assign92410_e141584_d_n10, assign92410_e141584_d_n11, assign92410_e141584_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2147 != 0.0)) {
        let (assign92410_e141582, assign92410_e141582_d_n0, assign92410_e141582_d_n2, assign92410_e141582_d_n4, assign92410_e141582_d_n5, assign92410_e141582_d_n6, assign92410_e141582_d_n7, assign92410_e141582_d_n8, assign92410_e141582_d_n9, assign92410_e141582_d_n10, assign92410_e141582_d_n11, assign92410_e141582_d_n14,) = {
            if (locals.var_chi_1 >= 0.0) {
                (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn11, locals.var_chi_1_dn14,)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign92410_e141582, assign92410_e141582_d_n0, assign92410_e141582_d_n2, assign92410_e141582_d_n4, assign92410_e141582_d_n5, assign92410_e141582_d_n6, assign92410_e141582_d_n7, assign92410_e141582_d_n8, assign92410_e141582_d_n9, assign92410_e141582_d_n10, assign92410_e141582_d_n11, assign92410_e141582_d_n14,)
    } else {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn11, locals.var_chi_1_dn14,)
    }
};
        locals.var_chi_1 = assign92410_e141584;
        locals.var_chi_1_dn0 = assign92410_e141584_d_n0;
        locals.var_chi_1_dn2 = assign92410_e141584_d_n2;
        locals.var_chi_1_dn4 = assign92410_e141584_d_n4;
        locals.var_chi_1_dn5 = assign92410_e141584_d_n5;
        locals.var_chi_1_dn6 = assign92410_e141584_d_n6;
        locals.var_chi_1_dn7 = assign92410_e141584_d_n7;
        locals.var_chi_1_dn8 = assign92410_e141584_d_n8;
        locals.var_chi_1_dn9 = assign92410_e141584_d_n9;
        locals.var_chi_1_dn10 = assign92410_e141584_d_n10;
        locals.var_chi_1_dn11 = assign92410_e141584_d_n11;
        locals.var_chi_1_dn14 = assign92410_e141584_d_n14;
        locals.var_chi_1_rv = 0.0;

        let (assign92420_e141597, assign92420_e141597_d_n0, assign92420_e141597_d_n2, assign92420_e141597_d_n4, assign92420_e141597_d_n5, assign92420_e141597_d_n6, assign92420_e141597_d_n7, assign92420_e141597_d_n8, assign92420_e141597_d_n9, assign92420_e141597_d_n10, assign92420_e141597_d_n11, assign92420_e141597_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2147 != 0.0)) {
        let assign92420_e141595: f64 = (locals.var_psi - locals.var_chi_1);
        (assign92420_e141595, (locals.var_psi_dn0 - locals.var_chi_1_dn0), (locals.var_psi_dn2 - locals.var_chi_1_dn2), (locals.var_psi_dn4 - locals.var_chi_1_dn4), (locals.var_psi_dn5 - locals.var_chi_1_dn5), (locals.var_psi_dn6 - locals.var_chi_1_dn6), (locals.var_psi_dn7 - locals.var_chi_1_dn7), (locals.var_psi_dn8 - locals.var_chi_1_dn8), (locals.var_psi_dn9 - locals.var_chi_1_dn9), (locals.var_psi_dn10 - locals.var_chi_1_dn10), (locals.var_psi_dn11 - locals.var_chi_1_dn11), (locals.var_psi_dn14 - locals.var_chi_1_dn14),)
    } else {
        (locals.var_psi, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn4, locals.var_psi_dn5, locals.var_psi_dn6, locals.var_psi_dn7, locals.var_psi_dn8, locals.var_psi_dn9, locals.var_psi_dn10, locals.var_psi_dn11, locals.var_psi_dn14,)
    }
};
        locals.var_psi = assign92420_e141597;
        locals.var_psi_dn0 = assign92420_e141597_d_n0;
        locals.var_psi_dn2 = assign92420_e141597_d_n2;
        locals.var_psi_dn4 = assign92420_e141597_d_n4;
        locals.var_psi_dn5 = assign92420_e141597_d_n5;
        locals.var_psi_dn6 = assign92420_e141597_d_n6;
        locals.var_psi_dn7 = assign92420_e141597_d_n7;
        locals.var_psi_dn8 = assign92420_e141597_d_n8;
        locals.var_psi_dn9 = assign92420_e141597_d_n9;
        locals.var_psi_dn10 = assign92420_e141597_d_n10;
        locals.var_psi_dn11 = assign92420_e141597_d_n11;
        locals.var_psi_dn14 = assign92420_e141597_d_n14;
        locals.var_psi_rv = 0.0;

        let (assign92430_e141612, assign92430_e141612_d_n0, assign92430_e141612_d_n2, assign92430_e141612_d_n4, assign92430_e141612_d_n5, assign92430_e141612_d_n6, assign92430_e141612_d_n7, assign92430_e141612_d_n8, assign92430_e141612_d_n9, assign92430_e141612_d_n10, assign92430_e141612_d_n11, assign92430_e141612_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2147 != 0.0)) {
        let assign92430_e141609: f64 = (locals.var_beta * 0.1);
        let assign92430_e141610: f64 = (locals.var_psi + assign92430_e141609);
        (assign92430_e141610, (locals.var_psi_dn0 + (locals.var_beta_dn0 * 0.1)), (locals.var_psi_dn2 + (locals.var_beta_dn2 * 0.1)), (locals.var_psi_dn4 + (locals.var_beta_dn4 * 0.1)), (locals.var_psi_dn5 + (locals.var_beta_dn5 * 0.1)), (locals.var_psi_dn6 + (locals.var_beta_dn6 * 0.1)), (locals.var_psi_dn7 + (locals.var_beta_dn7 * 0.1)), (locals.var_psi_dn8 + (locals.var_beta_dn8 * 0.1)), (locals.var_psi_dn9 + (locals.var_beta_dn9 * 0.1)), (locals.var_psi_dn10 + (locals.var_beta_dn10 * 0.1)), (locals.var_psi_dn11 + (locals.var_beta_dn11 * 0.1)), (locals.var_psi_dn14 + (locals.var_beta_dn14 * 0.1)),)
    } else {
        (locals.var_psi, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn4, locals.var_psi_dn5, locals.var_psi_dn6, locals.var_psi_dn7, locals.var_psi_dn8, locals.var_psi_dn9, locals.var_psi_dn10, locals.var_psi_dn11, locals.var_psi_dn14,)
    }
};
        locals.var_psi = assign92430_e141612;
        locals.var_psi_dn0 = assign92430_e141612_d_n0;
        locals.var_psi_dn2 = assign92430_e141612_d_n2;
        locals.var_psi_dn4 = assign92430_e141612_d_n4;
        locals.var_psi_dn5 = assign92430_e141612_d_n5;
        locals.var_psi_dn6 = assign92430_e141612_d_n6;
        locals.var_psi_dn7 = assign92430_e141612_d_n7;
        locals.var_psi_dn8 = assign92430_e141612_d_n8;
        locals.var_psi_dn9 = assign92430_e141612_d_n9;
        locals.var_psi_dn10 = assign92430_e141612_d_n10;
        locals.var_psi_dn11 = assign92430_e141612_d_n11;
        locals.var_psi_dn14 = assign92430_e141612_d_n14;
        locals.var_psi_rv = 0.0;

        let (assign92440_e141635, assign92440_e141635_d_n0, assign92440_e141635_d_n2, assign92440_e141635_d_n4, assign92440_e141635_d_n5, assign92440_e141635_d_n6, assign92440_e141635_d_n7, assign92440_e141635_d_n8, assign92440_e141635_d_n9, assign92440_e141635_d_n10, assign92440_e141635_d_n11, assign92440_e141635_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2147 != 0.0)) {
        let assign92440_e141623: f64 = (locals.var_gammachi * locals.var_t0);
        let assign92440_e141626: f64 = (locals.var_psi * locals.var_psi);
        let assign92440_e141627: f64 = (assign92440_e141623 + assign92440_e141626);
        let assign92440_e141628: f64 = (assign92440_e141627).ln();
        let assign92440_e141631: f64 = (locals.var_cnst1over * locals.var_t0);
        let assign92440_e141632: f64 = (assign92440_e141631).ln();
        let assign92440_e141633: f64 = (assign92440_e141628 - assign92440_e141632);
        (assign92440_e141633, (((((locals.var_gammachi_dn0 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn0)) + ((locals.var_psi_dn0 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn0))) / assign92440_e141627) - (((locals.var_cnst1over_dn0 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn0)) / assign92440_e141631)), (((((locals.var_gammachi_dn2 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn2)) + ((locals.var_psi_dn2 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn2))) / assign92440_e141627) - (((locals.var_cnst1over_dn2 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn2)) / assign92440_e141631)), (((((locals.var_gammachi_dn4 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn4)) + ((locals.var_psi_dn4 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn4))) / assign92440_e141627) - (((locals.var_cnst1over_dn4 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn4)) / assign92440_e141631)), (((((locals.var_gammachi_dn5 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn5)) + ((locals.var_psi_dn5 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn5))) / assign92440_e141627) - (((locals.var_cnst1over_dn5 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn5)) / assign92440_e141631)), (((((locals.var_gammachi_dn6 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn6)) + ((locals.var_psi_dn6 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn6))) / assign92440_e141627) - (((locals.var_cnst1over_dn6 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn6)) / assign92440_e141631)), (((((locals.var_gammachi_dn7 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn7)) + ((locals.var_psi_dn7 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn7))) / assign92440_e141627) - (((locals.var_cnst1over_dn7 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn7)) / assign92440_e141631)), (((((locals.var_gammachi_dn8 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn8)) + ((locals.var_psi_dn8 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn8))) / assign92440_e141627) - (((locals.var_cnst1over_dn8 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn8)) / assign92440_e141631)), (((((locals.var_gammachi_dn9 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn9)) + ((locals.var_psi_dn9 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn9))) / assign92440_e141627) - (((locals.var_cnst1over_dn9 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn9)) / assign92440_e141631)), (((((locals.var_gammachi_dn10 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn10)) + ((locals.var_psi_dn10 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn10))) / assign92440_e141627) - (((locals.var_cnst1over_dn10 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn10)) / assign92440_e141631)), (((((locals.var_gammachi_dn11 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn11)) + ((locals.var_psi_dn11 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn11))) / assign92440_e141627) - (((locals.var_cnst1over_dn11 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn11)) / assign92440_e141631)), (((((locals.var_gammachi_dn14 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn14)) + ((locals.var_psi_dn14 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn14))) / assign92440_e141627) - (((locals.var_cnst1over_dn14 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn14)) / assign92440_e141631)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign92440_e141635;
        locals.var_t1_dn0 = assign92440_e141635_d_n0;
        locals.var_t1_dn2 = assign92440_e141635_d_n2;
        locals.var_t1_dn4 = assign92440_e141635_d_n4;
        locals.var_t1_dn5 = assign92440_e141635_d_n5;
        locals.var_t1_dn6 = assign92440_e141635_d_n6;
        locals.var_t1_dn7 = assign92440_e141635_d_n7;
        locals.var_t1_dn8 = assign92440_e141635_d_n8;
        locals.var_t1_dn9 = assign92440_e141635_d_n9;
        locals.var_t1_dn10 = assign92440_e141635_d_n10;
        locals.var_t1_dn11 = assign92440_e141635_d_n11;
        locals.var_t1_dn14 = assign92440_e141635_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign92450_e141650, assign92450_e141650_d_n0, assign92450_e141650_d_n2, assign92450_e141650_d_n4, assign92450_e141650_d_n5, assign92450_e141650_d_n6, assign92450_e141650_d_n7, assign92450_e141650_d_n8, assign92450_e141650_d_n9, assign92450_e141650_d_n10, assign92450_e141650_d_n11, assign92450_e141650_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2147 != 0.0)) {
        let assign92450_e141647: f64 = (locals.var_beta * locals.var_vxbgmtcl);
        let assign92450_e141648: f64 = (locals.var_t1 + assign92450_e141647);
        (assign92450_e141648, (locals.var_t1_dn0 + ((locals.var_beta_dn0 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn0))), (locals.var_t1_dn2 + ((locals.var_beta_dn2 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn2))), (locals.var_t1_dn4 + ((locals.var_beta_dn4 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn4))), (locals.var_t1_dn5 + ((locals.var_beta_dn5 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn5))), (locals.var_t1_dn6 + ((locals.var_beta_dn6 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn6))), (locals.var_t1_dn7 + ((locals.var_beta_dn7 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn7))), (locals.var_t1_dn8 + ((locals.var_beta_dn8 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn8))), (locals.var_t1_dn9 + ((locals.var_beta_dn9 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn9))), (locals.var_t1_dn10 + ((locals.var_beta_dn10 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn10))), (locals.var_t1_dn11 + ((locals.var_beta_dn11 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn11))), (locals.var_t1_dn14 + ((locals.var_beta_dn14 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn14))),)
    } else {
        (locals.var_chi_b, locals.var_chi_b_dn0, locals.var_chi_b_dn2, locals.var_chi_b_dn4, locals.var_chi_b_dn5, locals.var_chi_b_dn6, locals.var_chi_b_dn7, locals.var_chi_b_dn8, locals.var_chi_b_dn9, locals.var_chi_b_dn10, locals.var_chi_b_dn11, locals.var_chi_b_dn14,)
    }
};
        locals.var_chi_b = assign92450_e141650;
        locals.var_chi_b_dn0 = assign92450_e141650_d_n0;
        locals.var_chi_b_dn2 = assign92450_e141650_d_n2;
        locals.var_chi_b_dn4 = assign92450_e141650_d_n4;
        locals.var_chi_b_dn5 = assign92450_e141650_d_n5;
        locals.var_chi_b_dn6 = assign92450_e141650_d_n6;
        locals.var_chi_b_dn7 = assign92450_e141650_d_n7;
        locals.var_chi_b_dn8 = assign92450_e141650_d_n8;
        locals.var_chi_b_dn9 = assign92450_e141650_d_n9;
        locals.var_chi_b_dn10 = assign92450_e141650_d_n10;
        locals.var_chi_b_dn11 = assign92450_e141650_d_n11;
        locals.var_chi_b_dn14 = assign92450_e141650_d_n14;
        locals.var_chi_b_rv = 0.0;

        let (assign92460_e141666, assign92460_e141666_d_n0, assign92460_e141666_d_n2, assign92460_e141666_d_n4, assign92460_e141666_d_n5, assign92460_e141666_d_n6, assign92460_e141666_d_n7, assign92460_e141666_d_n8, assign92460_e141666_d_n9, assign92460_e141666_d_n10, assign92460_e141666_d_n11, assign92460_e141666_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2147 != 0.0)) {
        let (assign92460_e141664, assign92460_e141664_d_n0, assign92460_e141664_d_n2, assign92460_e141664_d_n4, assign92460_e141664_d_n5, assign92460_e141664_d_n6, assign92460_e141664_d_n7, assign92460_e141664_d_n8, assign92460_e141664_d_n9, assign92460_e141664_d_n10, assign92460_e141664_d_n11, assign92460_e141664_d_n14,) = {
            if (locals.var_chi_b >= 0.0) {
                (locals.var_chi_b, locals.var_chi_b_dn0, locals.var_chi_b_dn2, locals.var_chi_b_dn4, locals.var_chi_b_dn5, locals.var_chi_b_dn6, locals.var_chi_b_dn7, locals.var_chi_b_dn8, locals.var_chi_b_dn9, locals.var_chi_b_dn10, locals.var_chi_b_dn11, locals.var_chi_b_dn14,)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign92460_e141664, assign92460_e141664_d_n0, assign92460_e141664_d_n2, assign92460_e141664_d_n4, assign92460_e141664_d_n5, assign92460_e141664_d_n6, assign92460_e141664_d_n7, assign92460_e141664_d_n8, assign92460_e141664_d_n9, assign92460_e141664_d_n10, assign92460_e141664_d_n11, assign92460_e141664_d_n14,)
    } else {
        (locals.var_chi_b, locals.var_chi_b_dn0, locals.var_chi_b_dn2, locals.var_chi_b_dn4, locals.var_chi_b_dn5, locals.var_chi_b_dn6, locals.var_chi_b_dn7, locals.var_chi_b_dn8, locals.var_chi_b_dn9, locals.var_chi_b_dn10, locals.var_chi_b_dn11, locals.var_chi_b_dn14,)
    }
};
        locals.var_chi_b = assign92460_e141666;
        locals.var_chi_b_dn0 = assign92460_e141666_d_n0;
        locals.var_chi_b_dn2 = assign92460_e141666_d_n2;
        locals.var_chi_b_dn4 = assign92460_e141666_d_n4;
        locals.var_chi_b_dn5 = assign92460_e141666_d_n5;
        locals.var_chi_b_dn6 = assign92460_e141666_d_n6;
        locals.var_chi_b_dn7 = assign92460_e141666_d_n7;
        locals.var_chi_b_dn8 = assign92460_e141666_d_n8;
        locals.var_chi_b_dn9 = assign92460_e141666_d_n9;
        locals.var_chi_b_dn10 = assign92460_e141666_d_n10;
        locals.var_chi_b_dn11 = assign92460_e141666_d_n11;
        locals.var_chi_b_dn14 = assign92460_e141666_d_n14;
        locals.var_chi_b_rv = 0.0;

        let (assign92470_e141677, assign92470_e141677_d_n0, assign92470_e141677_d_n2, assign92470_e141677_d_n4, assign92470_e141677_d_n5, assign92470_e141677_d_n6, assign92470_e141677_d_n7, assign92470_e141677_d_n8, assign92470_e141677_d_n9, assign92470_e141677_d_n10, assign92470_e141677_d_n11, assign92470_e141677_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2147 != 0.0)) {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    } else {
        (locals.var_chi_a, locals.var_chi_a_dn0, locals.var_chi_a_dn2, locals.var_chi_a_dn4, locals.var_chi_a_dn5, locals.var_chi_a_dn6, locals.var_chi_a_dn7, locals.var_chi_a_dn8, locals.var_chi_a_dn9, locals.var_chi_a_dn10, locals.var_chi_a_dn11, locals.var_chi_a_dn14,)
    }
};
        locals.var_chi_a = assign92470_e141677;
        locals.var_chi_a_dn0 = assign92470_e141677_d_n0;
        locals.var_chi_a_dn2 = assign92470_e141677_d_n2;
        locals.var_chi_a_dn4 = assign92470_e141677_d_n4;
        locals.var_chi_a_dn5 = assign92470_e141677_d_n5;
        locals.var_chi_a_dn6 = assign92470_e141677_d_n6;
        locals.var_chi_a_dn7 = assign92470_e141677_d_n7;
        locals.var_chi_a_dn8 = assign92470_e141677_d_n8;
        locals.var_chi_a_dn9 = assign92470_e141677_d_n9;
        locals.var_chi_a_dn10 = assign92470_e141677_d_n10;
        locals.var_chi_a_dn11 = assign92470_e141677_d_n11;
        locals.var_chi_a_dn14 = assign92470_e141677_d_n14;
        locals.var_chi_a_rv = 0.0;

        let assign92480_e141680: f64 = if p.p33 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard2149 = assign92480_e141680;
        locals.var_guard2149_rv = 0.0;

        let assign92490_e141685: f64 = (0.2 * locals.var_chi_b);
        let assign92490_e141686: f64 = (locals.var_chi_b - assign92490_e141685);
        let assign92490_e141690: f64 = (0.2 * locals.var_chi_b);
        let assign92490_e141693: f64 = if ((locals.var_chi_a > assign92490_e141686) && (assign92490_e141690 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2150 = assign92490_e141693;
        locals.var_guard2150_rv = 0.0;

        let (assign92500_e141714, assign92500_e141714_d_n0, assign92500_e141714_d_n2, assign92500_e141714_d_n4, assign92500_e141714_d_n5, assign92500_e141714_d_n6, assign92500_e141714_d_n7, assign92500_e141714_d_n8, assign92500_e141714_d_n9, assign92500_e141714_d_n10, assign92500_e141714_d_n11, assign92500_e141714_d_n14,) = {
    if ((((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2147 != 0.0)) && (locals.var_guard2149 != 0.0)) && (locals.var_guard2150 != 0.0)) {
        let assign92500_e141708: f64 = (locals.var_chi_a - locals.var_chi_b);
        let assign92500_e141711: f64 = (0.2 * locals.var_chi_b);
        let assign92500_e141712: f64 = (assign92500_e141708 + assign92500_e141711);
        (assign92500_e141712, ((locals.var_chi_a_dn0 - locals.var_chi_b_dn0) + (0.2 * locals.var_chi_b_dn0)), ((locals.var_chi_a_dn2 - locals.var_chi_b_dn2) + (0.2 * locals.var_chi_b_dn2)), ((locals.var_chi_a_dn4 - locals.var_chi_b_dn4) + (0.2 * locals.var_chi_b_dn4)), ((locals.var_chi_a_dn5 - locals.var_chi_b_dn5) + (0.2 * locals.var_chi_b_dn5)), ((locals.var_chi_a_dn6 - locals.var_chi_b_dn6) + (0.2 * locals.var_chi_b_dn6)), ((locals.var_chi_a_dn7 - locals.var_chi_b_dn7) + (0.2 * locals.var_chi_b_dn7)), ((locals.var_chi_a_dn8 - locals.var_chi_b_dn8) + (0.2 * locals.var_chi_b_dn8)), ((locals.var_chi_a_dn9 - locals.var_chi_b_dn9) + (0.2 * locals.var_chi_b_dn9)), ((locals.var_chi_a_dn10 - locals.var_chi_b_dn10) + (0.2 * locals.var_chi_b_dn10)), ((locals.var_chi_a_dn11 - locals.var_chi_b_dn11) + (0.2 * locals.var_chi_b_dn11)), ((locals.var_chi_a_dn14 - locals.var_chi_b_dn14) + (0.2 * locals.var_chi_b_dn14)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign92500_e141714;
        locals.var_tmf1_dn0 = assign92500_e141714_d_n0;
        locals.var_tmf1_dn2 = assign92500_e141714_d_n2;
        locals.var_tmf1_dn4 = assign92500_e141714_d_n4;
        locals.var_tmf1_dn5 = assign92500_e141714_d_n5;
        locals.var_tmf1_dn6 = assign92500_e141714_d_n6;
        locals.var_tmf1_dn7 = assign92500_e141714_d_n7;
        locals.var_tmf1_dn8 = assign92500_e141714_d_n8;
        locals.var_tmf1_dn9 = assign92500_e141714_d_n9;
        locals.var_tmf1_dn10 = assign92500_e141714_d_n10;
        locals.var_tmf1_dn11 = assign92500_e141714_d_n11;
        locals.var_tmf1_dn14 = assign92500_e141714_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign92510_e141731, assign92510_e141731_d_n0, assign92510_e141731_d_n2, assign92510_e141731_d_n4, assign92510_e141731_d_n5, assign92510_e141731_d_n6, assign92510_e141731_d_n7, assign92510_e141731_d_n8, assign92510_e141731_d_n9, assign92510_e141731_d_n10, assign92510_e141731_d_n11, assign92510_e141731_d_n14,) = {
    if ((((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2147 != 0.0)) && (locals.var_guard2149 != 0.0)) && (locals.var_guard2150 != 0.0)) {
        let assign92510_e141729: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign92510_e141729, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign92510_e141731;
        locals.var_x2_dn0 = assign92510_e141731_d_n0;
        locals.var_x2_dn2 = assign92510_e141731_d_n2;
        locals.var_x2_dn4 = assign92510_e141731_d_n4;
        locals.var_x2_dn5 = assign92510_e141731_d_n5;
        locals.var_x2_dn6 = assign92510_e141731_d_n6;
        locals.var_x2_dn7 = assign92510_e141731_d_n7;
        locals.var_x2_dn8 = assign92510_e141731_d_n8;
        locals.var_x2_dn9 = assign92510_e141731_d_n9;
        locals.var_x2_dn10 = assign92510_e141731_d_n10;
        locals.var_x2_dn11 = assign92510_e141731_d_n11;
        locals.var_x2_dn14 = assign92510_e141731_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign92520_e141752, assign92520_e141752_d_n0, assign92520_e141752_d_n2, assign92520_e141752_d_n4, assign92520_e141752_d_n5, assign92520_e141752_d_n6, assign92520_e141752_d_n7, assign92520_e141752_d_n8, assign92520_e141752_d_n9, assign92520_e141752_d_n10, assign92520_e141752_d_n11, assign92520_e141752_d_n14,) = {
    if ((((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2147 != 0.0)) && (locals.var_guard2149 != 0.0)) && (locals.var_guard2150 != 0.0)) {
        let assign92520_e141746: f64 = (0.2 * locals.var_chi_b);
        let assign92520_e141749: f64 = (0.2 * locals.var_chi_b);
        let assign92520_e141750: f64 = (assign92520_e141746 * assign92520_e141749);
        (assign92520_e141750, (((0.2 * locals.var_chi_b_dn0) * assign92520_e141749) + (assign92520_e141746 * (0.2 * locals.var_chi_b_dn0))), (((0.2 * locals.var_chi_b_dn2) * assign92520_e141749) + (assign92520_e141746 * (0.2 * locals.var_chi_b_dn2))), (((0.2 * locals.var_chi_b_dn4) * assign92520_e141749) + (assign92520_e141746 * (0.2 * locals.var_chi_b_dn4))), (((0.2 * locals.var_chi_b_dn5) * assign92520_e141749) + (assign92520_e141746 * (0.2 * locals.var_chi_b_dn5))), (((0.2 * locals.var_chi_b_dn6) * assign92520_e141749) + (assign92520_e141746 * (0.2 * locals.var_chi_b_dn6))), (((0.2 * locals.var_chi_b_dn7) * assign92520_e141749) + (assign92520_e141746 * (0.2 * locals.var_chi_b_dn7))), (((0.2 * locals.var_chi_b_dn8) * assign92520_e141749) + (assign92520_e141746 * (0.2 * locals.var_chi_b_dn8))), (((0.2 * locals.var_chi_b_dn9) * assign92520_e141749) + (assign92520_e141746 * (0.2 * locals.var_chi_b_dn9))), (((0.2 * locals.var_chi_b_dn10) * assign92520_e141749) + (assign92520_e141746 * (0.2 * locals.var_chi_b_dn10))), (((0.2 * locals.var_chi_b_dn11) * assign92520_e141749) + (assign92520_e141746 * (0.2 * locals.var_chi_b_dn11))), (((0.2 * locals.var_chi_b_dn14) * assign92520_e141749) + (assign92520_e141746 * (0.2 * locals.var_chi_b_dn14))),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign92520_e141752;
        locals.var_xmax2_dn0 = assign92520_e141752_d_n0;
        locals.var_xmax2_dn2 = assign92520_e141752_d_n2;
        locals.var_xmax2_dn4 = assign92520_e141752_d_n4;
        locals.var_xmax2_dn5 = assign92520_e141752_d_n5;
        locals.var_xmax2_dn6 = assign92520_e141752_d_n6;
        locals.var_xmax2_dn7 = assign92520_e141752_d_n7;
        locals.var_xmax2_dn8 = assign92520_e141752_d_n8;
        locals.var_xmax2_dn9 = assign92520_e141752_d_n9;
        locals.var_xmax2_dn10 = assign92520_e141752_d_n10;
        locals.var_xmax2_dn11 = assign92520_e141752_d_n11;
        locals.var_xmax2_dn14 = assign92520_e141752_d_n14;
        locals.var_xmax2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_357(
        locals: &mut StampLocals,
    ) {
        let (assign92530_e141767, assign92530_e141767_d_n0, assign92530_e141767_d_n2, assign92530_e141767_d_n4, assign92530_e141767_d_n5, assign92530_e141767_d_n6, assign92530_e141767_d_n7, assign92530_e141767_d_n8, assign92530_e141767_d_n9, assign92530_e141767_d_n10, assign92530_e141767_d_n11, assign92530_e141767_d_n14,) = {
    if ((((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2147 != 0.0)) && (locals.var_guard2149 != 0.0)) && (locals.var_guard2150 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign92530_e141767;
        locals.var_xp_dn0 = assign92530_e141767_d_n0;
        locals.var_xp_dn2 = assign92530_e141767_d_n2;
        locals.var_xp_dn4 = assign92530_e141767_d_n4;
        locals.var_xp_dn5 = assign92530_e141767_d_n5;
        locals.var_xp_dn6 = assign92530_e141767_d_n6;
        locals.var_xp_dn7 = assign92530_e141767_d_n7;
        locals.var_xp_dn8 = assign92530_e141767_d_n8;
        locals.var_xp_dn9 = assign92530_e141767_d_n9;
        locals.var_xp_dn10 = assign92530_e141767_d_n10;
        locals.var_xp_dn11 = assign92530_e141767_d_n11;
        locals.var_xp_dn14 = assign92530_e141767_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign92540_e141782, assign92540_e141782_d_n0, assign92540_e141782_d_n2, assign92540_e141782_d_n4, assign92540_e141782_d_n5, assign92540_e141782_d_n6, assign92540_e141782_d_n7, assign92540_e141782_d_n8, assign92540_e141782_d_n9, assign92540_e141782_d_n10, assign92540_e141782_d_n11, assign92540_e141782_d_n14,) = {
    if ((((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2147 != 0.0)) && (locals.var_guard2149 != 0.0)) && (locals.var_guard2150 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign92540_e141782;
        locals.var_xmp_dn0 = assign92540_e141782_d_n0;
        locals.var_xmp_dn2 = assign92540_e141782_d_n2;
        locals.var_xmp_dn4 = assign92540_e141782_d_n4;
        locals.var_xmp_dn5 = assign92540_e141782_d_n5;
        locals.var_xmp_dn6 = assign92540_e141782_d_n6;
        locals.var_xmp_dn7 = assign92540_e141782_d_n7;
        locals.var_xmp_dn8 = assign92540_e141782_d_n8;
        locals.var_xmp_dn9 = assign92540_e141782_d_n9;
        locals.var_xmp_dn10 = assign92540_e141782_d_n10;
        locals.var_xmp_dn11 = assign92540_e141782_d_n11;
        locals.var_xmp_dn14 = assign92540_e141782_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign92550_e141797,) = {
    if ((((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2147 != 0.0)) && (locals.var_guard2149 != 0.0)) && (locals.var_guard2150 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign92550_e141797;
        locals.var_m0_rv = 0.0;

        let (assign92560_e141812,) = {
    if ((((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2147 != 0.0)) && (locals.var_guard2149 != 0.0)) && (locals.var_guard2150 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign92560_e141812;
        locals.var_mm_rv = 0.0;

        let (assign92570_e141827, assign92570_e141827_d_n0, assign92570_e141827_d_n2, assign92570_e141827_d_n4, assign92570_e141827_d_n5, assign92570_e141827_d_n6, assign92570_e141827_d_n7, assign92570_e141827_d_n8, assign92570_e141827_d_n9, assign92570_e141827_d_n10, assign92570_e141827_d_n11, assign92570_e141827_d_n14,) = {
    if ((((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2147 != 0.0)) && (locals.var_guard2149 != 0.0)) && (locals.var_guard2150 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign92570_e141827;
        locals.var_arg_dn0 = assign92570_e141827_d_n0;
        locals.var_arg_dn2 = assign92570_e141827_d_n2;
        locals.var_arg_dn4 = assign92570_e141827_d_n4;
        locals.var_arg_dn5 = assign92570_e141827_d_n5;
        locals.var_arg_dn6 = assign92570_e141827_d_n6;
        locals.var_arg_dn7 = assign92570_e141827_d_n7;
        locals.var_arg_dn8 = assign92570_e141827_d_n8;
        locals.var_arg_dn9 = assign92570_e141827_d_n9;
        locals.var_arg_dn10 = assign92570_e141827_d_n10;
        locals.var_arg_dn11 = assign92570_e141827_d_n11;
        locals.var_arg_dn14 = assign92570_e141827_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign92580_e141842, assign92580_e141842_d_n0, assign92580_e141842_d_n2, assign92580_e141842_d_n4, assign92580_e141842_d_n5, assign92580_e141842_d_n6, assign92580_e141842_d_n7, assign92580_e141842_d_n8, assign92580_e141842_d_n9, assign92580_e141842_d_n10, assign92580_e141842_d_n11, assign92580_e141842_d_n14,) = {
    if ((((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2147 != 0.0)) && (locals.var_guard2149 != 0.0)) && (locals.var_guard2150 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign92580_e141842;
        locals.var_dnm_dn0 = assign92580_e141842_d_n0;
        locals.var_dnm_dn2 = assign92580_e141842_d_n2;
        locals.var_dnm_dn4 = assign92580_e141842_d_n4;
        locals.var_dnm_dn5 = assign92580_e141842_d_n5;
        locals.var_dnm_dn6 = assign92580_e141842_d_n6;
        locals.var_dnm_dn7 = assign92580_e141842_d_n7;
        locals.var_dnm_dn8 = assign92580_e141842_d_n8;
        locals.var_dnm_dn9 = assign92580_e141842_d_n9;
        locals.var_dnm_dn10 = assign92580_e141842_d_n10;
        locals.var_dnm_dn11 = assign92580_e141842_d_n11;
        locals.var_dnm_dn14 = assign92580_e141842_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign92590_e141859, assign92590_e141859_d_n0, assign92590_e141859_d_n2, assign92590_e141859_d_n4, assign92590_e141859_d_n5, assign92590_e141859_d_n6, assign92590_e141859_d_n7, assign92590_e141859_d_n8, assign92590_e141859_d_n9, assign92590_e141859_d_n10, assign92590_e141859_d_n11, assign92590_e141859_d_n14,) = {
    if ((((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2147 != 0.0)) && (locals.var_guard2149 != 0.0)) && (locals.var_guard2150 != 0.0)) {
        let assign92590_e141857: f64 = (locals.var_xp * locals.var_x2);
        (assign92590_e141857, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign92590_e141859;
        locals.var_xp_dn0 = assign92590_e141859_d_n0;
        locals.var_xp_dn2 = assign92590_e141859_d_n2;
        locals.var_xp_dn4 = assign92590_e141859_d_n4;
        locals.var_xp_dn5 = assign92590_e141859_d_n5;
        locals.var_xp_dn6 = assign92590_e141859_d_n6;
        locals.var_xp_dn7 = assign92590_e141859_d_n7;
        locals.var_xp_dn8 = assign92590_e141859_d_n8;
        locals.var_xp_dn9 = assign92590_e141859_d_n9;
        locals.var_xp_dn10 = assign92590_e141859_d_n10;
        locals.var_xp_dn11 = assign92590_e141859_d_n11;
        locals.var_xp_dn14 = assign92590_e141859_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign92600_e141876, assign92600_e141876_d_n0, assign92600_e141876_d_n2, assign92600_e141876_d_n4, assign92600_e141876_d_n5, assign92600_e141876_d_n6, assign92600_e141876_d_n7, assign92600_e141876_d_n8, assign92600_e141876_d_n9, assign92600_e141876_d_n10, assign92600_e141876_d_n11, assign92600_e141876_d_n14,) = {
    if ((((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2147 != 0.0)) && (locals.var_guard2149 != 0.0)) && (locals.var_guard2150 != 0.0)) {
        let assign92600_e141874: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign92600_e141874, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign92600_e141876;
        locals.var_xmp_dn0 = assign92600_e141876_d_n0;
        locals.var_xmp_dn2 = assign92600_e141876_d_n2;
        locals.var_xmp_dn4 = assign92600_e141876_d_n4;
        locals.var_xmp_dn5 = assign92600_e141876_d_n5;
        locals.var_xmp_dn6 = assign92600_e141876_d_n6;
        locals.var_xmp_dn7 = assign92600_e141876_d_n7;
        locals.var_xmp_dn8 = assign92600_e141876_d_n8;
        locals.var_xmp_dn9 = assign92600_e141876_d_n9;
        locals.var_xmp_dn10 = assign92600_e141876_d_n10;
        locals.var_xmp_dn11 = assign92600_e141876_d_n11;
        locals.var_xmp_dn14 = assign92600_e141876_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign92610_e141893, assign92610_e141893_d_n0, assign92610_e141893_d_n2, assign92610_e141893_d_n4, assign92610_e141893_d_n5, assign92610_e141893_d_n6, assign92610_e141893_d_n7, assign92610_e141893_d_n8, assign92610_e141893_d_n9, assign92610_e141893_d_n10, assign92610_e141893_d_n11, assign92610_e141893_d_n14,) = {
    if ((((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2147 != 0.0)) && (locals.var_guard2149 != 0.0)) && (locals.var_guard2150 != 0.0)) {
        let assign92610_e141891: f64 = (locals.var_xp * locals.var_x2);
        (assign92610_e141891, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign92610_e141893;
        locals.var_xp_dn0 = assign92610_e141893_d_n0;
        locals.var_xp_dn2 = assign92610_e141893_d_n2;
        locals.var_xp_dn4 = assign92610_e141893_d_n4;
        locals.var_xp_dn5 = assign92610_e141893_d_n5;
        locals.var_xp_dn6 = assign92610_e141893_d_n6;
        locals.var_xp_dn7 = assign92610_e141893_d_n7;
        locals.var_xp_dn8 = assign92610_e141893_d_n8;
        locals.var_xp_dn9 = assign92610_e141893_d_n9;
        locals.var_xp_dn10 = assign92610_e141893_d_n10;
        locals.var_xp_dn11 = assign92610_e141893_d_n11;
        locals.var_xp_dn14 = assign92610_e141893_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign92620_e141910, assign92620_e141910_d_n0, assign92620_e141910_d_n2, assign92620_e141910_d_n4, assign92620_e141910_d_n5, assign92620_e141910_d_n6, assign92620_e141910_d_n7, assign92620_e141910_d_n8, assign92620_e141910_d_n9, assign92620_e141910_d_n10, assign92620_e141910_d_n11, assign92620_e141910_d_n14,) = {
    if ((((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2147 != 0.0)) && (locals.var_guard2149 != 0.0)) && (locals.var_guard2150 != 0.0)) {
        let assign92620_e141908: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign92620_e141908, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign92620_e141910;
        locals.var_xmp_dn0 = assign92620_e141910_d_n0;
        locals.var_xmp_dn2 = assign92620_e141910_d_n2;
        locals.var_xmp_dn4 = assign92620_e141910_d_n4;
        locals.var_xmp_dn5 = assign92620_e141910_d_n5;
        locals.var_xmp_dn6 = assign92620_e141910_d_n6;
        locals.var_xmp_dn7 = assign92620_e141910_d_n7;
        locals.var_xmp_dn8 = assign92620_e141910_d_n8;
        locals.var_xmp_dn9 = assign92620_e141910_d_n9;
        locals.var_xmp_dn10 = assign92620_e141910_d_n10;
        locals.var_xmp_dn11 = assign92620_e141910_d_n11;
        locals.var_xmp_dn14 = assign92620_e141910_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign92630_e141927, assign92630_e141927_d_n0, assign92630_e141927_d_n2, assign92630_e141927_d_n4, assign92630_e141927_d_n5, assign92630_e141927_d_n6, assign92630_e141927_d_n7, assign92630_e141927_d_n8, assign92630_e141927_d_n9, assign92630_e141927_d_n10, assign92630_e141927_d_n11, assign92630_e141927_d_n14,) = {
    if ((((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2147 != 0.0)) && (locals.var_guard2149 != 0.0)) && (locals.var_guard2150 != 0.0)) {
        let assign92630_e141925: f64 = (locals.var_xp + locals.var_xmp);
        (assign92630_e141925, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign92630_e141927;
        locals.var_arg_dn0 = assign92630_e141927_d_n0;
        locals.var_arg_dn2 = assign92630_e141927_d_n2;
        locals.var_arg_dn4 = assign92630_e141927_d_n4;
        locals.var_arg_dn5 = assign92630_e141927_d_n5;
        locals.var_arg_dn6 = assign92630_e141927_d_n6;
        locals.var_arg_dn7 = assign92630_e141927_d_n7;
        locals.var_arg_dn8 = assign92630_e141927_d_n8;
        locals.var_arg_dn9 = assign92630_e141927_d_n9;
        locals.var_arg_dn10 = assign92630_e141927_d_n10;
        locals.var_arg_dn11 = assign92630_e141927_d_n11;
        locals.var_arg_dn14 = assign92630_e141927_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign92640_e141942, assign92640_e141942_d_n0, assign92640_e141942_d_n2, assign92640_e141942_d_n4, assign92640_e141942_d_n5, assign92640_e141942_d_n6, assign92640_e141942_d_n7, assign92640_e141942_d_n8, assign92640_e141942_d_n9, assign92640_e141942_d_n10, assign92640_e141942_d_n11, assign92640_e141942_d_n14,) = {
    if ((((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2147 != 0.0)) && (locals.var_guard2149 != 0.0)) && (locals.var_guard2150 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign92640_e141942;
        locals.var_dnm_dn0 = assign92640_e141942_d_n0;
        locals.var_dnm_dn2 = assign92640_e141942_d_n2;
        locals.var_dnm_dn4 = assign92640_e141942_d_n4;
        locals.var_dnm_dn5 = assign92640_e141942_d_n5;
        locals.var_dnm_dn6 = assign92640_e141942_d_n6;
        locals.var_dnm_dn7 = assign92640_e141942_d_n7;
        locals.var_dnm_dn8 = assign92640_e141942_d_n8;
        locals.var_dnm_dn9 = assign92640_e141942_d_n9;
        locals.var_dnm_dn10 = assign92640_e141942_d_n10;
        locals.var_dnm_dn11 = assign92640_e141942_d_n11;
        locals.var_dnm_dn14 = assign92640_e141942_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign92650_e141957: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard2151 = assign92650_e141957;
        locals.var_guard2151_rv = 0.0;

        let assign92660_e141960: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2152 = assign92660_e141960;
        locals.var_guard2152_rv = 0.0;

        let (assign92670_e141979,) = {
    if ((((((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2147 != 0.0)) && (locals.var_guard2149 != 0.0)) && (locals.var_guard2150 != 0.0)) && (locals.var_guard2151 != 0.0)) && (locals.var_guard2152 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign92670_e141979;
        locals.var_mm_rv = 0.0;

        let assign92680_e141982: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard2153 = assign92680_e141982;
        locals.var_guard2153_rv = 0.0;

        let (assign92690_e142004,) = {
    if (((((((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2147 != 0.0)) && (locals.var_guard2149 != 0.0)) && (locals.var_guard2150 != 0.0)) && (locals.var_guard2151 != 0.0)) && (locals.var_guard2152 == 0.0)) && (locals.var_guard2153 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign92690_e142004;
        locals.var_mm_rv = 0.0;

        let assign92700_e142007: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard2154 = assign92700_e142007;
        locals.var_guard2154_rv = 0.0;

        let (assign92710_e142032,) = {
    if ((((((((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2147 != 0.0)) && (locals.var_guard2149 != 0.0)) && (locals.var_guard2150 != 0.0)) && (locals.var_guard2151 != 0.0)) && (locals.var_guard2152 == 0.0)) && (locals.var_guard2153 == 0.0)) && (locals.var_guard2154 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign92710_e142032;
        locals.var_mm_rv = 0.0;

        let assign92720_e142035: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard2155 = assign92720_e142035;
        locals.var_guard2155_rv = 0.0;

        let (assign92730_e142063,) = {
    if (((((((((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2147 != 0.0)) && (locals.var_guard2149 != 0.0)) && (locals.var_guard2150 != 0.0)) && (locals.var_guard2151 != 0.0)) && (locals.var_guard2152 == 0.0)) && (locals.var_guard2153 == 0.0)) && (locals.var_guard2154 == 0.0)) && (locals.var_guard2155 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign92730_e142063;
        locals.var_mm_rv = 0.0;

        let (assign92740_e142080,) = {
    if (((((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2147 != 0.0)) && (locals.var_guard2149 != 0.0)) && (locals.var_guard2150 != 0.0)) && (locals.var_guard2151 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign92740_e142080;
        locals.var_m0_rv = 0.0;

        let mut assign92750_loop_guard: usize = 0;
        while {
            let assign92750_cond_e142098: f64 = if ((((((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2147 != 0.0)) && (locals.var_guard2149 != 0.0)) && (locals.var_guard2150 != 0.0)) && (locals.var_guard2151 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign92750_cond_e142098 != 0.0
        } {
            assign92750_loop_guard += 1;
            assert!(assign92750_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign92750_body0_e142116, assign92750_body0_e142116_d_n0, assign92750_body0_e142116_d_n2, assign92750_body0_e142116_d_n4, assign92750_body0_e142116_d_n5, assign92750_body0_e142116_d_n6, assign92750_body0_e142116_d_n7, assign92750_body0_e142116_d_n8, assign92750_body0_e142116_d_n9, assign92750_body0_e142116_d_n10, assign92750_body0_e142116_d_n11, assign92750_body0_e142116_d_n14,) = {
    if (((((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2147 != 0.0)) && (locals.var_guard2149 != 0.0)) && (locals.var_guard2150 != 0.0)) && (locals.var_guard2151 != 0.0)) {
        let assign92750_body0_e142114: f64 = (locals.var_dnm).sqrt();
        (assign92750_body0_e142114, (locals.var_dnm_dn0 / (2.0 * assign92750_body0_e142114)), (locals.var_dnm_dn2 / (2.0 * assign92750_body0_e142114)), (locals.var_dnm_dn4 / (2.0 * assign92750_body0_e142114)), (locals.var_dnm_dn5 / (2.0 * assign92750_body0_e142114)), (locals.var_dnm_dn6 / (2.0 * assign92750_body0_e142114)), (locals.var_dnm_dn7 / (2.0 * assign92750_body0_e142114)), (locals.var_dnm_dn8 / (2.0 * assign92750_body0_e142114)), (locals.var_dnm_dn9 / (2.0 * assign92750_body0_e142114)), (locals.var_dnm_dn10 / (2.0 * assign92750_body0_e142114)), (locals.var_dnm_dn11 / (2.0 * assign92750_body0_e142114)), (locals.var_dnm_dn14 / (2.0 * assign92750_body0_e142114)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign92750_body0_e142116;
            locals.var_dnm_dn0 = assign92750_body0_e142116_d_n0;
            locals.var_dnm_dn2 = assign92750_body0_e142116_d_n2;
            locals.var_dnm_dn4 = assign92750_body0_e142116_d_n4;
            locals.var_dnm_dn5 = assign92750_body0_e142116_d_n5;
            locals.var_dnm_dn6 = assign92750_body0_e142116_d_n6;
            locals.var_dnm_dn7 = assign92750_body0_e142116_d_n7;
            locals.var_dnm_dn8 = assign92750_body0_e142116_d_n8;
            locals.var_dnm_dn9 = assign92750_body0_e142116_d_n9;
            locals.var_dnm_dn10 = assign92750_body0_e142116_d_n10;
            locals.var_dnm_dn11 = assign92750_body0_e142116_d_n11;
            locals.var_dnm_dn14 = assign92750_body0_e142116_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign92750_body1_e142135,) = {
    if (((((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2147 != 0.0)) && (locals.var_guard2149 != 0.0)) && (locals.var_guard2150 != 0.0)) && (locals.var_guard2151 != 0.0)) {
        let assign92750_body1_e142133: f64 = (locals.var_m0 + 1.0);
        (assign92750_body1_e142133,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign92750_body1_e142135;
            locals.var_m0_rv = 0.0;
        }

        let (assign92760_e142164, assign92760_e142164_d_n0, assign92760_e142164_d_n2, assign92760_e142164_d_n4, assign92760_e142164_d_n5, assign92760_e142164_d_n6, assign92760_e142164_d_n7, assign92760_e142164_d_n8, assign92760_e142164_d_n9, assign92760_e142164_d_n10, assign92760_e142164_d_n11, assign92760_e142164_d_n14,) = {
    if (((((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2147 != 0.0)) && (locals.var_guard2149 != 0.0)) && (locals.var_guard2150 != 0.0)) && (locals.var_guard2151 == 0.0)) {
        let (assign92760_e142162, assign92760_e142162_d_n0, assign92760_e142162_d_n2, assign92760_e142162_d_n4, assign92760_e142162_d_n5, assign92760_e142162_d_n6, assign92760_e142162_d_n7, assign92760_e142162_d_n8, assign92760_e142162_d_n9, assign92760_e142162_d_n10, assign92760_e142162_d_n11, assign92760_e142162_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign92760_e142159: f64 = (2.0 * 2.0);
                let assign92760_e142160: f64 = (1.0 / assign92760_e142159);
                let assign92760_e142161: f64 = (locals.var_dnm).powf(assign92760_e142160);
                (assign92760_e142161, if 0.0 == 0.0 && ((assign92760_e142160) as f64).is_finite() && ((assign92760_e142160) as f64).fract() == 0.0 { if assign92760_e142160 == 0.0 { 0.0 } else { (assign92760_e142160 * ((locals.var_dnm).powf(assign92760_e142160 - 1.0) * locals.var_dnm_dn0)) } } else { (assign92760_e142161 * (assign92760_e142160 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign92760_e142160) as f64).is_finite() && ((assign92760_e142160) as f64).fract() == 0.0 { if assign92760_e142160 == 0.0 { 0.0 } else { (assign92760_e142160 * ((locals.var_dnm).powf(assign92760_e142160 - 1.0) * locals.var_dnm_dn2)) } } else { (assign92760_e142161 * (assign92760_e142160 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign92760_e142160) as f64).is_finite() && ((assign92760_e142160) as f64).fract() == 0.0 { if assign92760_e142160 == 0.0 { 0.0 } else { (assign92760_e142160 * ((locals.var_dnm).powf(assign92760_e142160 - 1.0) * locals.var_dnm_dn4)) } } else { (assign92760_e142161 * (assign92760_e142160 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign92760_e142160) as f64).is_finite() && ((assign92760_e142160) as f64).fract() == 0.0 { if assign92760_e142160 == 0.0 { 0.0 } else { (assign92760_e142160 * ((locals.var_dnm).powf(assign92760_e142160 - 1.0) * locals.var_dnm_dn5)) } } else { (assign92760_e142161 * (assign92760_e142160 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign92760_e142160) as f64).is_finite() && ((assign92760_e142160) as f64).fract() == 0.0 { if assign92760_e142160 == 0.0 { 0.0 } else { (assign92760_e142160 * ((locals.var_dnm).powf(assign92760_e142160 - 1.0) * locals.var_dnm_dn6)) } } else { (assign92760_e142161 * (assign92760_e142160 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign92760_e142160) as f64).is_finite() && ((assign92760_e142160) as f64).fract() == 0.0 { if assign92760_e142160 == 0.0 { 0.0 } else { (assign92760_e142160 * ((locals.var_dnm).powf(assign92760_e142160 - 1.0) * locals.var_dnm_dn7)) } } else { (assign92760_e142161 * (assign92760_e142160 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign92760_e142160) as f64).is_finite() && ((assign92760_e142160) as f64).fract() == 0.0 { if assign92760_e142160 == 0.0 { 0.0 } else { (assign92760_e142160 * ((locals.var_dnm).powf(assign92760_e142160 - 1.0) * locals.var_dnm_dn8)) } } else { (assign92760_e142161 * (assign92760_e142160 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign92760_e142160) as f64).is_finite() && ((assign92760_e142160) as f64).fract() == 0.0 { if assign92760_e142160 == 0.0 { 0.0 } else { (assign92760_e142160 * ((locals.var_dnm).powf(assign92760_e142160 - 1.0) * locals.var_dnm_dn9)) } } else { (assign92760_e142161 * (assign92760_e142160 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign92760_e142160) as f64).is_finite() && ((assign92760_e142160) as f64).fract() == 0.0 { if assign92760_e142160 == 0.0 { 0.0 } else { (assign92760_e142160 * ((locals.var_dnm).powf(assign92760_e142160 - 1.0) * locals.var_dnm_dn10)) } } else { (assign92760_e142161 * (assign92760_e142160 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign92760_e142160) as f64).is_finite() && ((assign92760_e142160) as f64).fract() == 0.0 { if assign92760_e142160 == 0.0 { 0.0 } else { (assign92760_e142160 * ((locals.var_dnm).powf(assign92760_e142160 - 1.0) * locals.var_dnm_dn11)) } } else { (assign92760_e142161 * (assign92760_e142160 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign92760_e142160) as f64).is_finite() && ((assign92760_e142160) as f64).fract() == 0.0 { if assign92760_e142160 == 0.0 { 0.0 } else { (assign92760_e142160 * ((locals.var_dnm).powf(assign92760_e142160 - 1.0) * locals.var_dnm_dn14)) } } else { (assign92760_e142161 * (assign92760_e142160 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign92760_e142162, assign92760_e142162_d_n0, assign92760_e142162_d_n2, assign92760_e142162_d_n4, assign92760_e142162_d_n5, assign92760_e142162_d_n6, assign92760_e142162_d_n7, assign92760_e142162_d_n8, assign92760_e142162_d_n9, assign92760_e142162_d_n10, assign92760_e142162_d_n11, assign92760_e142162_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign92760_e142164;
        locals.var_dnm_dn0 = assign92760_e142164_d_n0;
        locals.var_dnm_dn2 = assign92760_e142164_d_n2;
        locals.var_dnm_dn4 = assign92760_e142164_d_n4;
        locals.var_dnm_dn5 = assign92760_e142164_d_n5;
        locals.var_dnm_dn6 = assign92760_e142164_d_n6;
        locals.var_dnm_dn7 = assign92760_e142164_d_n7;
        locals.var_dnm_dn8 = assign92760_e142164_d_n8;
        locals.var_dnm_dn9 = assign92760_e142164_d_n9;
        locals.var_dnm_dn10 = assign92760_e142164_d_n10;
        locals.var_dnm_dn11 = assign92760_e142164_d_n11;
        locals.var_dnm_dn14 = assign92760_e142164_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign92770_e142181, assign92770_e142181_d_n0, assign92770_e142181_d_n2, assign92770_e142181_d_n4, assign92770_e142181_d_n5, assign92770_e142181_d_n6, assign92770_e142181_d_n7, assign92770_e142181_d_n8, assign92770_e142181_d_n9, assign92770_e142181_d_n10, assign92770_e142181_d_n11, assign92770_e142181_d_n14,) = {
    if ((((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2147 != 0.0)) && (locals.var_guard2149 != 0.0)) && (locals.var_guard2150 != 0.0)) {
        let assign92770_e142179: f64 = (1.0 / locals.var_dnm);
        (assign92770_e142179, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign92770_e142181;
        locals.var_dnm_dn0 = assign92770_e142181_d_n0;
        locals.var_dnm_dn2 = assign92770_e142181_d_n2;
        locals.var_dnm_dn4 = assign92770_e142181_d_n4;
        locals.var_dnm_dn5 = assign92770_e142181_d_n5;
        locals.var_dnm_dn6 = assign92770_e142181_d_n6;
        locals.var_dnm_dn7 = assign92770_e142181_d_n7;
        locals.var_dnm_dn8 = assign92770_e142181_d_n8;
        locals.var_dnm_dn9 = assign92770_e142181_d_n9;
        locals.var_dnm_dn10 = assign92770_e142181_d_n10;
        locals.var_dnm_dn11 = assign92770_e142181_d_n11;
        locals.var_dnm_dn14 = assign92770_e142181_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign92780_e142202, assign92780_e142202_d_n0, assign92780_e142202_d_n2, assign92780_e142202_d_n4, assign92780_e142202_d_n5, assign92780_e142202_d_n6, assign92780_e142202_d_n7, assign92780_e142202_d_n8, assign92780_e142202_d_n9, assign92780_e142202_d_n10, assign92780_e142202_d_n11, assign92780_e142202_d_n14,) = {
    if ((((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2147 != 0.0)) && (locals.var_guard2149 != 0.0)) && (locals.var_guard2150 != 0.0)) {
        let assign92780_e142197: f64 = (0.2 * locals.var_chi_b);
        let assign92780_e142198: f64 = (locals.var_tmf1 * assign92780_e142197);
        let assign92780_e142200: f64 = (assign92780_e142198 * locals.var_dnm);
        (assign92780_e142200, ((((locals.var_tmf1_dn0 * assign92780_e142197) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn0))) * locals.var_dnm) + (assign92780_e142198 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * assign92780_e142197) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn2))) * locals.var_dnm) + (assign92780_e142198 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn4 * assign92780_e142197) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn4))) * locals.var_dnm) + (assign92780_e142198 * locals.var_dnm_dn4)), ((((locals.var_tmf1_dn5 * assign92780_e142197) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn5))) * locals.var_dnm) + (assign92780_e142198 * locals.var_dnm_dn5)), ((((locals.var_tmf1_dn6 * assign92780_e142197) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn6))) * locals.var_dnm) + (assign92780_e142198 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * assign92780_e142197) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn7))) * locals.var_dnm) + (assign92780_e142198 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn8 * assign92780_e142197) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn8))) * locals.var_dnm) + (assign92780_e142198 * locals.var_dnm_dn8)), ((((locals.var_tmf1_dn9 * assign92780_e142197) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn9))) * locals.var_dnm) + (assign92780_e142198 * locals.var_dnm_dn9)), ((((locals.var_tmf1_dn10 * assign92780_e142197) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn10))) * locals.var_dnm) + (assign92780_e142198 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn11 * assign92780_e142197) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn11))) * locals.var_dnm) + (assign92780_e142198 * locals.var_dnm_dn11)), ((((locals.var_tmf1_dn14 * assign92780_e142197) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn14))) * locals.var_dnm) + (assign92780_e142198 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign92780_e142202;
        locals.var_tmf0_dn0 = assign92780_e142202_d_n0;
        locals.var_tmf0_dn2 = assign92780_e142202_d_n2;
        locals.var_tmf0_dn4 = assign92780_e142202_d_n4;
        locals.var_tmf0_dn5 = assign92780_e142202_d_n5;
        locals.var_tmf0_dn6 = assign92780_e142202_d_n6;
        locals.var_tmf0_dn7 = assign92780_e142202_d_n7;
        locals.var_tmf0_dn8 = assign92780_e142202_d_n8;
        locals.var_tmf0_dn9 = assign92780_e142202_d_n9;
        locals.var_tmf0_dn10 = assign92780_e142202_d_n10;
        locals.var_tmf0_dn11 = assign92780_e142202_d_n11;
        locals.var_tmf0_dn14 = assign92780_e142202_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign92790_e142225, assign92790_e142225_d_n0, assign92790_e142225_d_n2, assign92790_e142225_d_n4, assign92790_e142225_d_n5, assign92790_e142225_d_n6, assign92790_e142225_d_n7, assign92790_e142225_d_n8, assign92790_e142225_d_n9, assign92790_e142225_d_n10, assign92790_e142225_d_n11, assign92790_e142225_d_n14,) = {
    if ((((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2147 != 0.0)) && (locals.var_guard2149 != 0.0)) && (locals.var_guard2150 != 0.0)) {
        let assign92790_e142217: f64 = (0.2 * locals.var_chi_b);
        let assign92790_e142219: f64 = (assign92790_e142217 * locals.var_xmp);
        let assign92790_e142221: f64 = (assign92790_e142219 * locals.var_dnm);
        let assign92790_e142223: f64 = (assign92790_e142221 / locals.var_arg);
        (assign92790_e142223, ((((((((0.2 * locals.var_chi_b_dn0) * locals.var_xmp) + (assign92790_e142217 * locals.var_xmp_dn0)) * locals.var_dnm) + (assign92790_e142219 * locals.var_dnm_dn0)) * locals.var_arg) - (assign92790_e142221 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn2) * locals.var_xmp) + (assign92790_e142217 * locals.var_xmp_dn2)) * locals.var_dnm) + (assign92790_e142219 * locals.var_dnm_dn2)) * locals.var_arg) - (assign92790_e142221 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn4) * locals.var_xmp) + (assign92790_e142217 * locals.var_xmp_dn4)) * locals.var_dnm) + (assign92790_e142219 * locals.var_dnm_dn4)) * locals.var_arg) - (assign92790_e142221 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn5) * locals.var_xmp) + (assign92790_e142217 * locals.var_xmp_dn5)) * locals.var_dnm) + (assign92790_e142219 * locals.var_dnm_dn5)) * locals.var_arg) - (assign92790_e142221 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn6) * locals.var_xmp) + (assign92790_e142217 * locals.var_xmp_dn6)) * locals.var_dnm) + (assign92790_e142219 * locals.var_dnm_dn6)) * locals.var_arg) - (assign92790_e142221 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn7) * locals.var_xmp) + (assign92790_e142217 * locals.var_xmp_dn7)) * locals.var_dnm) + (assign92790_e142219 * locals.var_dnm_dn7)) * locals.var_arg) - (assign92790_e142221 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn8) * locals.var_xmp) + (assign92790_e142217 * locals.var_xmp_dn8)) * locals.var_dnm) + (assign92790_e142219 * locals.var_dnm_dn8)) * locals.var_arg) - (assign92790_e142221 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn9) * locals.var_xmp) + (assign92790_e142217 * locals.var_xmp_dn9)) * locals.var_dnm) + (assign92790_e142219 * locals.var_dnm_dn9)) * locals.var_arg) - (assign92790_e142221 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn10) * locals.var_xmp) + (assign92790_e142217 * locals.var_xmp_dn10)) * locals.var_dnm) + (assign92790_e142219 * locals.var_dnm_dn10)) * locals.var_arg) - (assign92790_e142221 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn11) * locals.var_xmp) + (assign92790_e142217 * locals.var_xmp_dn11)) * locals.var_dnm) + (assign92790_e142219 * locals.var_dnm_dn11)) * locals.var_arg) - (assign92790_e142221 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn14) * locals.var_xmp) + (assign92790_e142217 * locals.var_xmp_dn14)) * locals.var_dnm) + (assign92790_e142219 * locals.var_dnm_dn14)) * locals.var_arg) - (assign92790_e142221 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign92790_e142225;
        locals.var_t1_dn0 = assign92790_e142225_d_n0;
        locals.var_t1_dn2 = assign92790_e142225_d_n2;
        locals.var_t1_dn4 = assign92790_e142225_d_n4;
        locals.var_t1_dn5 = assign92790_e142225_d_n5;
        locals.var_t1_dn6 = assign92790_e142225_d_n6;
        locals.var_t1_dn7 = assign92790_e142225_d_n7;
        locals.var_t1_dn8 = assign92790_e142225_d_n8;
        locals.var_t1_dn9 = assign92790_e142225_d_n9;
        locals.var_t1_dn10 = assign92790_e142225_d_n10;
        locals.var_t1_dn11 = assign92790_e142225_d_n11;
        locals.var_t1_dn14 = assign92790_e142225_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign92800_e142246, assign92800_e142246_d_n0, assign92800_e142246_d_n2, assign92800_e142246_d_n4, assign92800_e142246_d_n5, assign92800_e142246_d_n6, assign92800_e142246_d_n7, assign92800_e142246_d_n8, assign92800_e142246_d_n9, assign92800_e142246_d_n10, assign92800_e142246_d_n11, assign92800_e142246_d_n14,) = {
    if ((((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2147 != 0.0)) && (locals.var_guard2149 != 0.0)) && (locals.var_guard2150 != 0.0)) {
        let assign92800_e142241: f64 = (0.2 * locals.var_chi_b);
        let assign92800_e142242: f64 = (locals.var_chi_b - assign92800_e142241);
        let assign92800_e142244: f64 = (assign92800_e142242 + locals.var_tmf0);
        (assign92800_e142244, ((locals.var_chi_b_dn0 - (0.2 * locals.var_chi_b_dn0)) + locals.var_tmf0_dn0), ((locals.var_chi_b_dn2 - (0.2 * locals.var_chi_b_dn2)) + locals.var_tmf0_dn2), ((locals.var_chi_b_dn4 - (0.2 * locals.var_chi_b_dn4)) + locals.var_tmf0_dn4), ((locals.var_chi_b_dn5 - (0.2 * locals.var_chi_b_dn5)) + locals.var_tmf0_dn5), ((locals.var_chi_b_dn6 - (0.2 * locals.var_chi_b_dn6)) + locals.var_tmf0_dn6), ((locals.var_chi_b_dn7 - (0.2 * locals.var_chi_b_dn7)) + locals.var_tmf0_dn7), ((locals.var_chi_b_dn8 - (0.2 * locals.var_chi_b_dn8)) + locals.var_tmf0_dn8), ((locals.var_chi_b_dn9 - (0.2 * locals.var_chi_b_dn9)) + locals.var_tmf0_dn9), ((locals.var_chi_b_dn10 - (0.2 * locals.var_chi_b_dn10)) + locals.var_tmf0_dn10), ((locals.var_chi_b_dn11 - (0.2 * locals.var_chi_b_dn11)) + locals.var_tmf0_dn11), ((locals.var_chi_b_dn14 - (0.2 * locals.var_chi_b_dn14)) + locals.var_tmf0_dn14),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
        locals.var_chi = assign92800_e142246;
        locals.var_chi_dn0 = assign92800_e142246_d_n0;
        locals.var_chi_dn2 = assign92800_e142246_d_n2;
        locals.var_chi_dn4 = assign92800_e142246_d_n4;
        locals.var_chi_dn5 = assign92800_e142246_d_n5;
        locals.var_chi_dn6 = assign92800_e142246_d_n6;
        locals.var_chi_dn7 = assign92800_e142246_d_n7;
        locals.var_chi_dn8 = assign92800_e142246_d_n8;
        locals.var_chi_dn9 = assign92800_e142246_d_n9;
        locals.var_chi_dn10 = assign92800_e142246_d_n10;
        locals.var_chi_dn11 = assign92800_e142246_d_n11;
        locals.var_chi_dn14 = assign92800_e142246_d_n14;
        locals.var_chi_rv = 0.0;

        let (assign92810_e142261, assign92810_e142261_d_n0, assign92810_e142261_d_n2, assign92810_e142261_d_n4, assign92810_e142261_d_n5, assign92810_e142261_d_n6, assign92810_e142261_d_n7, assign92810_e142261_d_n8, assign92810_e142261_d_n9, assign92810_e142261_d_n10, assign92810_e142261_d_n11, assign92810_e142261_d_n14,) = {
    if ((((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2147 != 0.0)) && (locals.var_guard2149 != 0.0)) && (locals.var_guard2150 != 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign92810_e142261;
        locals.var_t1_dn0 = assign92810_e142261_d_n0;
        locals.var_t1_dn2 = assign92810_e142261_d_n2;
        locals.var_t1_dn4 = assign92810_e142261_d_n4;
        locals.var_t1_dn5 = assign92810_e142261_d_n5;
        locals.var_t1_dn6 = assign92810_e142261_d_n6;
        locals.var_t1_dn7 = assign92810_e142261_d_n7;
        locals.var_t1_dn8 = assign92810_e142261_d_n8;
        locals.var_t1_dn9 = assign92810_e142261_d_n9;
        locals.var_t1_dn10 = assign92810_e142261_d_n10;
        locals.var_t1_dn11 = assign92810_e142261_d_n11;
        locals.var_t1_dn14 = assign92810_e142261_d_n14;
        locals.var_t1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_358(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign92820_e142277, assign92820_e142277_d_n0, assign92820_e142277_d_n2, assign92820_e142277_d_n4, assign92820_e142277_d_n5, assign92820_e142277_d_n6, assign92820_e142277_d_n7, assign92820_e142277_d_n8, assign92820_e142277_d_n9, assign92820_e142277_d_n10, assign92820_e142277_d_n11, assign92820_e142277_d_n14,) = {
    if ((((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2147 != 0.0)) && (locals.var_guard2149 != 0.0)) && (locals.var_guard2150 == 0.0)) {
        (locals.var_chi_a, locals.var_chi_a_dn0, locals.var_chi_a_dn2, locals.var_chi_a_dn4, locals.var_chi_a_dn5, locals.var_chi_a_dn6, locals.var_chi_a_dn7, locals.var_chi_a_dn8, locals.var_chi_a_dn9, locals.var_chi_a_dn10, locals.var_chi_a_dn11, locals.var_chi_a_dn14,)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
        locals.var_chi = assign92820_e142277;
        locals.var_chi_dn0 = assign92820_e142277_d_n0;
        locals.var_chi_dn2 = assign92820_e142277_d_n2;
        locals.var_chi_dn4 = assign92820_e142277_d_n4;
        locals.var_chi_dn5 = assign92820_e142277_d_n5;
        locals.var_chi_dn6 = assign92820_e142277_d_n6;
        locals.var_chi_dn7 = assign92820_e142277_d_n7;
        locals.var_chi_dn8 = assign92820_e142277_d_n8;
        locals.var_chi_dn9 = assign92820_e142277_d_n9;
        locals.var_chi_dn10 = assign92820_e142277_d_n10;
        locals.var_chi_dn11 = assign92820_e142277_d_n11;
        locals.var_chi_dn14 = assign92820_e142277_d_n14;
        locals.var_chi_rv = 0.0;

        let (assign92830_e142293, assign92830_e142293_d_n0, assign92830_e142293_d_n2, assign92830_e142293_d_n4, assign92830_e142293_d_n5, assign92830_e142293_d_n6, assign92830_e142293_d_n7, assign92830_e142293_d_n8, assign92830_e142293_d_n9, assign92830_e142293_d_n10, assign92830_e142293_d_n11, assign92830_e142293_d_n14,) = {
    if ((((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2147 != 0.0)) && (locals.var_guard2149 != 0.0)) && (locals.var_guard2150 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign92830_e142293;
        locals.var_t1_dn0 = assign92830_e142293_d_n0;
        locals.var_t1_dn2 = assign92830_e142293_d_n2;
        locals.var_t1_dn4 = assign92830_e142293_d_n4;
        locals.var_t1_dn5 = assign92830_e142293_d_n5;
        locals.var_t1_dn6 = assign92830_e142293_d_n6;
        locals.var_t1_dn7 = assign92830_e142293_d_n7;
        locals.var_t1_dn8 = assign92830_e142293_d_n8;
        locals.var_t1_dn9 = assign92830_e142293_d_n9;
        locals.var_t1_dn10 = assign92830_e142293_d_n10;
        locals.var_t1_dn11 = assign92830_e142293_d_n11;
        locals.var_t1_dn14 = assign92830_e142293_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign92840_e142312, assign92840_e142312_d_n0, assign92840_e142312_d_n2, assign92840_e142312_d_n4, assign92840_e142312_d_n5, assign92840_e142312_d_n6, assign92840_e142312_d_n7, assign92840_e142312_d_n8, assign92840_e142312_d_n9, assign92840_e142312_d_n10, assign92840_e142312_d_n11, assign92840_e142312_d_n14,) = {
    if (((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2147 != 0.0)) && (locals.var_guard2149 == 0.0)) {
        let (assign92840_e142310, assign92840_e142310_d_n0, assign92840_e142310_d_n2, assign92840_e142310_d_n4, assign92840_e142310_d_n5, assign92840_e142310_d_n6, assign92840_e142310_d_n7, assign92840_e142310_d_n8, assign92840_e142310_d_n9, assign92840_e142310_d_n10, assign92840_e142310_d_n11, assign92840_e142310_d_n14,) = {
            if (locals.var_chi_a <= locals.var_chi_b) {
                (locals.var_chi_a, locals.var_chi_a_dn0, locals.var_chi_a_dn2, locals.var_chi_a_dn4, locals.var_chi_a_dn5, locals.var_chi_a_dn6, locals.var_chi_a_dn7, locals.var_chi_a_dn8, locals.var_chi_a_dn9, locals.var_chi_a_dn10, locals.var_chi_a_dn11, locals.var_chi_a_dn14,)
            } else {
                (locals.var_chi_b, locals.var_chi_b_dn0, locals.var_chi_b_dn2, locals.var_chi_b_dn4, locals.var_chi_b_dn5, locals.var_chi_b_dn6, locals.var_chi_b_dn7, locals.var_chi_b_dn8, locals.var_chi_b_dn9, locals.var_chi_b_dn10, locals.var_chi_b_dn11, locals.var_chi_b_dn14,)
            }
        };
        (assign92840_e142310, assign92840_e142310_d_n0, assign92840_e142310_d_n2, assign92840_e142310_d_n4, assign92840_e142310_d_n5, assign92840_e142310_d_n6, assign92840_e142310_d_n7, assign92840_e142310_d_n8, assign92840_e142310_d_n9, assign92840_e142310_d_n10, assign92840_e142310_d_n11, assign92840_e142310_d_n14,)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
        locals.var_chi = assign92840_e142312;
        locals.var_chi_dn0 = assign92840_e142312_d_n0;
        locals.var_chi_dn2 = assign92840_e142312_d_n2;
        locals.var_chi_dn4 = assign92840_e142312_d_n4;
        locals.var_chi_dn5 = assign92840_e142312_d_n5;
        locals.var_chi_dn6 = assign92840_e142312_d_n6;
        locals.var_chi_dn7 = assign92840_e142312_d_n7;
        locals.var_chi_dn8 = assign92840_e142312_d_n8;
        locals.var_chi_dn9 = assign92840_e142312_d_n9;
        locals.var_chi_dn10 = assign92840_e142312_d_n10;
        locals.var_chi_dn11 = assign92840_e142312_d_n11;
        locals.var_chi_dn14 = assign92840_e142312_d_n14;
        locals.var_chi_rv = 0.0;

        let assign92850_e142315: f64 = if p.p33 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2156 = assign92850_e142315;
        locals.var_guard2156_rv = 0.0;

        let (assign92860_e142330, assign92860_e142330_d_n0, assign92860_e142330_d_n2, assign92860_e142330_d_n4, assign92860_e142330_d_n5, assign92860_e142330_d_n6, assign92860_e142330_d_n7, assign92860_e142330_d_n8, assign92860_e142330_d_n9, assign92860_e142330_d_n10, assign92860_e142330_d_n11, assign92860_e142330_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2156 != 0.0)) {
        let assign92860_e142326: f64 = (locals.var_chi * locals.var_beta_inv);
        let assign92860_e142328: f64 = (assign92860_e142326 - locals.var_vxbgmtcl);
        (assign92860_e142328, (((locals.var_chi_dn0 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn0)) - locals.var_vxbgmtcl_dn0), (((locals.var_chi_dn2 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn2)) - locals.var_vxbgmtcl_dn2), (((locals.var_chi_dn4 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn4)) - locals.var_vxbgmtcl_dn4), (((locals.var_chi_dn5 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn5)) - locals.var_vxbgmtcl_dn5), (((locals.var_chi_dn6 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn6)) - locals.var_vxbgmtcl_dn6), (((locals.var_chi_dn7 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn7)) - locals.var_vxbgmtcl_dn7), (((locals.var_chi_dn8 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn8)) - locals.var_vxbgmtcl_dn8), (((locals.var_chi_dn9 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn9)) - locals.var_vxbgmtcl_dn9), (((locals.var_chi_dn10 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn10)) - locals.var_vxbgmtcl_dn10), (((locals.var_chi_dn11 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn11)) - locals.var_vxbgmtcl_dn11), (((locals.var_chi_dn14 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn14)) - locals.var_vxbgmtcl_dn14),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn14,)
    }
};
        locals.var_ps0ld = assign92860_e142330;
        locals.var_ps0ld_dn0 = assign92860_e142330_d_n0;
        locals.var_ps0ld_dn2 = assign92860_e142330_d_n2;
        locals.var_ps0ld_dn4 = assign92860_e142330_d_n4;
        locals.var_ps0ld_dn5 = assign92860_e142330_d_n5;
        locals.var_ps0ld_dn6 = assign92860_e142330_d_n6;
        locals.var_ps0ld_dn7 = assign92860_e142330_d_n7;
        locals.var_ps0ld_dn8 = assign92860_e142330_d_n8;
        locals.var_ps0ld_dn9 = assign92860_e142330_d_n9;
        locals.var_ps0ld_dn10 = assign92860_e142330_d_n10;
        locals.var_ps0ld_dn11 = assign92860_e142330_d_n11;
        locals.var_ps0ld_dn14 = assign92860_e142330_d_n14;
        locals.var_ps0ld_rv = 0.0;

        let assign92870_e142333: f64 = if locals.var_wdep_func > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2157 = assign92870_e142333;
        locals.var_guard2157_rv = 0.0;

        let (assign92880_e142348, assign92880_e142348_d_n0, assign92880_e142348_d_n2, assign92880_e142348_d_n4, assign92880_e142348_d_n5, assign92880_e142348_d_n6, assign92880_e142348_d_n7, assign92880_e142348_d_n8, assign92880_e142348_d_n9, assign92880_e142348_d_n10, assign92880_e142348_d_n11, assign92880_e142348_d_n14,) = {
    if (((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2156 != 0.0)) && (locals.var_guard2157 != 0.0)) {
        let assign92880_e142346: f64 = (p.p334 - locals.var_wdep_func);
        (assign92880_e142346, (-locals.var_wdep_func_dn0), (-locals.var_wdep_func_dn2), (-locals.var_wdep_func_dn4), (-locals.var_wdep_func_dn5), (-locals.var_wdep_func_dn6), (-locals.var_wdep_func_dn7), (-locals.var_wdep_func_dn8), (-locals.var_wdep_func_dn9), (-locals.var_wdep_func_dn10), (-locals.var_wdep_func_dn11), (-locals.var_wdep_func_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign92880_e142348;
        locals.var_t2_dn0 = assign92880_e142348_d_n0;
        locals.var_t2_dn2 = assign92880_e142348_d_n2;
        locals.var_t2_dn4 = assign92880_e142348_d_n4;
        locals.var_t2_dn5 = assign92880_e142348_d_n5;
        locals.var_t2_dn6 = assign92880_e142348_d_n6;
        locals.var_t2_dn7 = assign92880_e142348_d_n7;
        locals.var_t2_dn8 = assign92880_e142348_d_n8;
        locals.var_t2_dn9 = assign92880_e142348_d_n9;
        locals.var_t2_dn10 = assign92880_e142348_d_n10;
        locals.var_t2_dn11 = assign92880_e142348_d_n11;
        locals.var_t2_dn14 = assign92880_e142348_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign92890_e142375, assign92890_e142375_d_n0, assign92890_e142375_d_n2, assign92890_e142375_d_n4, assign92890_e142375_d_n5, assign92890_e142375_d_n6, assign92890_e142375_d_n7, assign92890_e142375_d_n8, assign92890_e142375_d_n9, assign92890_e142375_d_n10, assign92890_e142375_d_n11, assign92890_e142375_d_n14,) = {
    if (((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2156 != 0.0)) && (locals.var_guard2157 == 0.0)) {
        let assign92890_e142362: f64 = (locals.var_vdsi + p.p137);
        let assign92890_e142365: f64 = (locals.var_vdsi + p.p137);
        let assign92890_e142366: f64 = (assign92890_e142362 * assign92890_e142365);
        let assign92890_e142369: f64 = (4.0 * 0.1);
        let assign92890_e142371: f64 = (assign92890_e142369 * 0.1);
        let assign92890_e142372: f64 = (assign92890_e142366 + assign92890_e142371);
        let assign92890_e142373: f64 = (assign92890_e142372).sqrt();
        (assign92890_e142373, 0.0, 0.0, 0.0, 0.0, (((locals.var_vdsi_dn6 * assign92890_e142365) + (assign92890_e142362 * locals.var_vdsi_dn6)) / (2.0 * assign92890_e142373)), 0.0, (((locals.var_vdsi_dn8 * assign92890_e142365) + (assign92890_e142362 * locals.var_vdsi_dn8)) / (2.0 * assign92890_e142373)), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign92890_e142375;
        locals.var_tmf2_dn0 = assign92890_e142375_d_n0;
        locals.var_tmf2_dn2 = assign92890_e142375_d_n2;
        locals.var_tmf2_dn4 = assign92890_e142375_d_n4;
        locals.var_tmf2_dn5 = assign92890_e142375_d_n5;
        locals.var_tmf2_dn6 = assign92890_e142375_d_n6;
        locals.var_tmf2_dn7 = assign92890_e142375_d_n7;
        locals.var_tmf2_dn8 = assign92890_e142375_d_n8;
        locals.var_tmf2_dn9 = assign92890_e142375_d_n9;
        locals.var_tmf2_dn10 = assign92890_e142375_d_n10;
        locals.var_tmf2_dn11 = assign92890_e142375_d_n11;
        locals.var_tmf2_dn14 = assign92890_e142375_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign92900_e142397, assign92900_e142397_d_n0, assign92900_e142397_d_n2, assign92900_e142397_d_n4, assign92900_e142397_d_n5, assign92900_e142397_d_n6, assign92900_e142397_d_n7, assign92900_e142397_d_n8, assign92900_e142397_d_n9, assign92900_e142397_d_n10, assign92900_e142397_d_n11, assign92900_e142397_d_n14,) = {
    if (((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2156 != 0.0)) && (locals.var_guard2157 == 0.0)) {
        let assign92900_e142391: f64 = (locals.var_vdsi + p.p137);
        let assign92900_e142393: f64 = (assign92900_e142391 / locals.var_tmf2);
        let assign92900_e142394: f64 = (1.0 + assign92900_e142393);
        let assign92900_e142395: f64 = (0.5 * assign92900_e142394);
        (assign92900_e142395, (0.5 * (-((assign92900_e142391 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign92900_e142391 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign92900_e142391 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign92900_e142391 * locals.var_tmf2_dn5) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (((locals.var_vdsi_dn6 * locals.var_tmf2) - (assign92900_e142391 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (-((assign92900_e142391 * locals.var_tmf2_dn7) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (((locals.var_vdsi_dn8 * locals.var_tmf2) - (assign92900_e142391 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (-((assign92900_e142391 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign92900_e142391 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign92900_e142391 * locals.var_tmf2_dn11) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign92900_e142391 * locals.var_tmf2_dn14) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign92900_e142397;
        locals.var_t9_dn0 = assign92900_e142397_d_n0;
        locals.var_t9_dn2 = assign92900_e142397_d_n2;
        locals.var_t9_dn4 = assign92900_e142397_d_n4;
        locals.var_t9_dn5 = assign92900_e142397_d_n5;
        locals.var_t9_dn6 = assign92900_e142397_d_n6;
        locals.var_t9_dn7 = assign92900_e142397_d_n7;
        locals.var_t9_dn8 = assign92900_e142397_d_n8;
        locals.var_t9_dn9 = assign92900_e142397_d_n9;
        locals.var_t9_dn10 = assign92900_e142397_d_n10;
        locals.var_t9_dn11 = assign92900_e142397_d_n11;
        locals.var_t9_dn14 = assign92900_e142397_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign92910_e142417, assign92910_e142417_d_n0, assign92910_e142417_d_n2, assign92910_e142417_d_n4, assign92910_e142417_d_n5, assign92910_e142417_d_n6, assign92910_e142417_d_n7, assign92910_e142417_d_n8, assign92910_e142417_d_n9, assign92910_e142417_d_n10, assign92910_e142417_d_n11, assign92910_e142417_d_n14,) = {
    if (((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2156 != 0.0)) && (locals.var_guard2157 == 0.0)) {
        let assign92910_e142412: f64 = (locals.var_vdsi + p.p137);
        let assign92910_e142414: f64 = (assign92910_e142412 + locals.var_tmf2);
        let assign92910_e142415: f64 = (0.5 * assign92910_e142414);
        (assign92910_e142415, (0.5 * locals.var_tmf2_dn0), (0.5 * locals.var_tmf2_dn2), (0.5 * locals.var_tmf2_dn4), (0.5 * locals.var_tmf2_dn5), (0.5 * (locals.var_vdsi_dn6 + locals.var_tmf2_dn6)), (0.5 * locals.var_tmf2_dn7), (0.5 * (locals.var_vdsi_dn8 + locals.var_tmf2_dn8)), (0.5 * locals.var_tmf2_dn9), (0.5 * locals.var_tmf2_dn10), (0.5 * locals.var_tmf2_dn11), (0.5 * locals.var_tmf2_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign92910_e142417;
        locals.var_t2_dn0 = assign92910_e142417_d_n0;
        locals.var_t2_dn2 = assign92910_e142417_d_n2;
        locals.var_t2_dn4 = assign92910_e142417_d_n4;
        locals.var_t2_dn5 = assign92910_e142417_d_n5;
        locals.var_t2_dn6 = assign92910_e142417_d_n6;
        locals.var_t2_dn7 = assign92910_e142417_d_n7;
        locals.var_t2_dn8 = assign92910_e142417_d_n8;
        locals.var_t2_dn9 = assign92910_e142417_d_n9;
        locals.var_t2_dn10 = assign92910_e142417_d_n10;
        locals.var_t2_dn11 = assign92910_e142417_d_n11;
        locals.var_t2_dn14 = assign92910_e142417_d_n14;
        locals.var_t2_rv = 0.0;

        let assign92920_e142420: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2158 = assign92920_e142420;
        locals.var_guard2158_rv = 0.0;

        let (assign92930_e142436, assign92930_e142436_d_n0, assign92930_e142436_d_n2, assign92930_e142436_d_n4, assign92930_e142436_d_n5, assign92930_e142436_d_n6, assign92930_e142436_d_n7, assign92930_e142436_d_n8, assign92930_e142436_d_n9, assign92930_e142436_d_n10, assign92930_e142436_d_n11, assign92930_e142436_d_n14,) = {
    if ((((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2156 != 0.0)) && (locals.var_guard2157 == 0.0)) && (locals.var_guard2158 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign92930_e142436;
        locals.var_t2_dn0 = assign92930_e142436_d_n0;
        locals.var_t2_dn2 = assign92930_e142436_d_n2;
        locals.var_t2_dn4 = assign92930_e142436_d_n4;
        locals.var_t2_dn5 = assign92930_e142436_d_n5;
        locals.var_t2_dn6 = assign92930_e142436_d_n6;
        locals.var_t2_dn7 = assign92930_e142436_d_n7;
        locals.var_t2_dn8 = assign92930_e142436_d_n8;
        locals.var_t2_dn9 = assign92930_e142436_d_n9;
        locals.var_t2_dn10 = assign92930_e142436_d_n10;
        locals.var_t2_dn11 = assign92930_e142436_d_n11;
        locals.var_t2_dn14 = assign92930_e142436_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign92940_e142452, assign92940_e142452_d_n0, assign92940_e142452_d_n2, assign92940_e142452_d_n4, assign92940_e142452_d_n5, assign92940_e142452_d_n6, assign92940_e142452_d_n7, assign92940_e142452_d_n8, assign92940_e142452_d_n9, assign92940_e142452_d_n10, assign92940_e142452_d_n11, assign92940_e142452_d_n14,) = {
    if ((((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2156 != 0.0)) && (locals.var_guard2157 == 0.0)) && (locals.var_guard2158 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign92940_e142452;
        locals.var_t9_dn0 = assign92940_e142452_d_n0;
        locals.var_t9_dn2 = assign92940_e142452_d_n2;
        locals.var_t9_dn4 = assign92940_e142452_d_n4;
        locals.var_t9_dn5 = assign92940_e142452_d_n5;
        locals.var_t9_dn6 = assign92940_e142452_d_n6;
        locals.var_t9_dn7 = assign92940_e142452_d_n7;
        locals.var_t9_dn8 = assign92940_e142452_d_n8;
        locals.var_t9_dn9 = assign92940_e142452_d_n9;
        locals.var_t9_dn10 = assign92940_e142452_d_n10;
        locals.var_t9_dn11 = assign92940_e142452_d_n11;
        locals.var_t9_dn14 = assign92940_e142452_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign92950_e142471, assign92950_e142471_d_n0, assign92950_e142471_d_n2, assign92950_e142471_d_n4, assign92950_e142471_d_n5, assign92950_e142471_d_n6, assign92950_e142471_d_n7, assign92950_e142471_d_n8, assign92950_e142471_d_n9, assign92950_e142471_d_n10, assign92950_e142471_d_n11, assign92950_e142471_d_n14,) = {
    if (((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2156 != 0.0)) && (locals.var_guard2157 == 0.0)) {
        let assign92950_e142466: f64 = (locals.var_kjunc * locals.var_t2);
        let assign92950_e142467: f64 = (assign92950_e142466).sqrt();
        let assign92950_e142469: f64 = (assign92950_e142467 * p.p432);
        (assign92950_e142469, ((((locals.var_kjunc_dn0 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn0)) / (2.0 * assign92950_e142467)) * p.p432), ((((locals.var_kjunc_dn2 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn2)) / (2.0 * assign92950_e142467)) * p.p432), ((((locals.var_kjunc_dn4 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn4)) / (2.0 * assign92950_e142467)) * p.p432), ((((locals.var_kjunc_dn5 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn5)) / (2.0 * assign92950_e142467)) * p.p432), ((((locals.var_kjunc_dn6 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn6)) / (2.0 * assign92950_e142467)) * p.p432), ((((locals.var_kjunc_dn7 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn7)) / (2.0 * assign92950_e142467)) * p.p432), ((((locals.var_kjunc_dn8 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn8)) / (2.0 * assign92950_e142467)) * p.p432), ((((locals.var_kjunc_dn9 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn9)) / (2.0 * assign92950_e142467)) * p.p432), ((((locals.var_kjunc_dn10 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn10)) / (2.0 * assign92950_e142467)) * p.p432), ((((locals.var_kjunc_dn11 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn11)) / (2.0 * assign92950_e142467)) * p.p432), ((((locals.var_kjunc_dn14 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn14)) / (2.0 * assign92950_e142467)) * p.p432),)
    } else {
        (locals.var_wjunc0, locals.var_wjunc0_dn0, locals.var_wjunc0_dn2, locals.var_wjunc0_dn4, locals.var_wjunc0_dn5, locals.var_wjunc0_dn6, locals.var_wjunc0_dn7, locals.var_wjunc0_dn8, locals.var_wjunc0_dn9, locals.var_wjunc0_dn10, locals.var_wjunc0_dn11, locals.var_wjunc0_dn14,)
    }
};
        locals.var_wjunc0 = assign92950_e142471;
        locals.var_wjunc0_dn0 = assign92950_e142471_d_n0;
        locals.var_wjunc0_dn2 = assign92950_e142471_d_n2;
        locals.var_wjunc0_dn4 = assign92950_e142471_d_n4;
        locals.var_wjunc0_dn5 = assign92950_e142471_d_n5;
        locals.var_wjunc0_dn6 = assign92950_e142471_d_n6;
        locals.var_wjunc0_dn7 = assign92950_e142471_d_n7;
        locals.var_wjunc0_dn8 = assign92950_e142471_d_n8;
        locals.var_wjunc0_dn9 = assign92950_e142471_d_n9;
        locals.var_wjunc0_dn10 = assign92950_e142471_d_n10;
        locals.var_wjunc0_dn11 = assign92950_e142471_d_n11;
        locals.var_wjunc0_dn14 = assign92950_e142471_d_n14;
        locals.var_wjunc0_rv = 0.0;

        let (assign92960_e142487, assign92960_e142487_d_n0, assign92960_e142487_d_n2, assign92960_e142487_d_n4, assign92960_e142487_d_n5, assign92960_e142487_d_n6, assign92960_e142487_d_n7, assign92960_e142487_d_n8, assign92960_e142487_d_n9, assign92960_e142487_d_n10, assign92960_e142487_d_n11, assign92960_e142487_d_n14,) = {
    if (((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2156 != 0.0)) && (locals.var_guard2157 == 0.0)) {
        let assign92960_e142485: f64 = (p.p334 - locals.var_wjunc0);
        (assign92960_e142485, (-locals.var_wjunc0_dn0), (-locals.var_wjunc0_dn2), (-locals.var_wjunc0_dn4), (-locals.var_wjunc0_dn5), (-locals.var_wjunc0_dn6), (-locals.var_wjunc0_dn7), (-locals.var_wjunc0_dn8), (-locals.var_wjunc0_dn9), (-locals.var_wjunc0_dn10), (-locals.var_wjunc0_dn11), (-locals.var_wjunc0_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign92960_e142487;
        locals.var_t2_dn0 = assign92960_e142487_d_n0;
        locals.var_t2_dn2 = assign92960_e142487_d_n2;
        locals.var_t2_dn4 = assign92960_e142487_d_n4;
        locals.var_t2_dn5 = assign92960_e142487_d_n5;
        locals.var_t2_dn6 = assign92960_e142487_d_n6;
        locals.var_t2_dn7 = assign92960_e142487_d_n7;
        locals.var_t2_dn8 = assign92960_e142487_d_n8;
        locals.var_t2_dn9 = assign92960_e142487_d_n9;
        locals.var_t2_dn10 = assign92960_e142487_d_n10;
        locals.var_t2_dn11 = assign92960_e142487_d_n11;
        locals.var_t2_dn14 = assign92960_e142487_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign92970_e142511, assign92970_e142511_d_n0, assign92970_e142511_d_n2, assign92970_e142511_d_n4, assign92970_e142511_d_n5, assign92970_e142511_d_n6, assign92970_e142511_d_n7, assign92970_e142511_d_n8, assign92970_e142511_d_n9, assign92970_e142511_d_n10, assign92970_e142511_d_n11, assign92970_e142511_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2156 != 0.0)) {
        let assign92970_e142498: f64 = (locals.var_t2 * locals.var_t2);
        let assign92970_e142502: f64 = (p.p334 * 0.01);
        let assign92970_e142503: f64 = (4.0 * assign92970_e142502);
        let assign92970_e142506: f64 = (p.p334 * 0.01);
        let assign92970_e142507: f64 = (assign92970_e142503 * assign92970_e142506);
        let assign92970_e142508: f64 = (assign92970_e142498 + assign92970_e142507);
        let assign92970_e142509: f64 = (assign92970_e142508).sqrt();
        (assign92970_e142509, (((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)) / (2.0 * assign92970_e142509)), (((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)) / (2.0 * assign92970_e142509)), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign92970_e142509)), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign92970_e142509)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign92970_e142509)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign92970_e142509)), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign92970_e142509)), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign92970_e142509)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign92970_e142509)), (((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign92970_e142509)), (((locals.var_t2_dn14 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn14)) / (2.0 * assign92970_e142509)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign92970_e142511;
        locals.var_tmf2_dn0 = assign92970_e142511_d_n0;
        locals.var_tmf2_dn2 = assign92970_e142511_d_n2;
        locals.var_tmf2_dn4 = assign92970_e142511_d_n4;
        locals.var_tmf2_dn5 = assign92970_e142511_d_n5;
        locals.var_tmf2_dn6 = assign92970_e142511_d_n6;
        locals.var_tmf2_dn7 = assign92970_e142511_d_n7;
        locals.var_tmf2_dn8 = assign92970_e142511_d_n8;
        locals.var_tmf2_dn9 = assign92970_e142511_d_n9;
        locals.var_tmf2_dn10 = assign92970_e142511_d_n10;
        locals.var_tmf2_dn11 = assign92970_e142511_d_n11;
        locals.var_tmf2_dn14 = assign92970_e142511_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign92980_e142528, assign92980_e142528_d_n0, assign92980_e142528_d_n2, assign92980_e142528_d_n4, assign92980_e142528_d_n5, assign92980_e142528_d_n6, assign92980_e142528_d_n7, assign92980_e142528_d_n8, assign92980_e142528_d_n9, assign92980_e142528_d_n10, assign92980_e142528_d_n11, assign92980_e142528_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2156 != 0.0)) {
        let assign92980_e142524: f64 = (locals.var_t2 / locals.var_tmf2);
        let assign92980_e142525: f64 = (1.0 + assign92980_e142524);
        let assign92980_e142526: f64 = (0.5 * assign92980_e142525);
        (assign92980_e142526, (0.5 * (((locals.var_t2_dn0 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn2 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn4 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn5 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn6 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn7 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn8 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn9 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn10 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn11 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn14 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign92980_e142528;
        locals.var_t9_dn0 = assign92980_e142528_d_n0;
        locals.var_t9_dn2 = assign92980_e142528_d_n2;
        locals.var_t9_dn4 = assign92980_e142528_d_n4;
        locals.var_t9_dn5 = assign92980_e142528_d_n5;
        locals.var_t9_dn6 = assign92980_e142528_d_n6;
        locals.var_t9_dn7 = assign92980_e142528_d_n7;
        locals.var_t9_dn8 = assign92980_e142528_d_n8;
        locals.var_t9_dn9 = assign92980_e142528_d_n9;
        locals.var_t9_dn10 = assign92980_e142528_d_n10;
        locals.var_t9_dn11 = assign92980_e142528_d_n11;
        locals.var_t9_dn14 = assign92980_e142528_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign92990_e142543, assign92990_e142543_d_n0, assign92990_e142543_d_n2, assign92990_e142543_d_n4, assign92990_e142543_d_n5, assign92990_e142543_d_n6, assign92990_e142543_d_n7, assign92990_e142543_d_n8, assign92990_e142543_d_n9, assign92990_e142543_d_n10, assign92990_e142543_d_n11, assign92990_e142543_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2156 != 0.0)) {
        let assign92990_e142540: f64 = (locals.var_t2 + locals.var_tmf2);
        let assign92990_e142541: f64 = (0.5 * assign92990_e142540);
        (assign92990_e142541, (0.5 * (locals.var_t2_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t2_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t2_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t2_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t2_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t2_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t2_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t2_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t2_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t2_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_t2_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign92990_e142543;
        locals.var_t2_dn0 = assign92990_e142543_d_n0;
        locals.var_t2_dn2 = assign92990_e142543_d_n2;
        locals.var_t2_dn4 = assign92990_e142543_d_n4;
        locals.var_t2_dn5 = assign92990_e142543_d_n5;
        locals.var_t2_dn6 = assign92990_e142543_d_n6;
        locals.var_t2_dn7 = assign92990_e142543_d_n7;
        locals.var_t2_dn8 = assign92990_e142543_d_n8;
        locals.var_t2_dn9 = assign92990_e142543_d_n9;
        locals.var_t2_dn10 = assign92990_e142543_d_n10;
        locals.var_t2_dn11 = assign92990_e142543_d_n11;
        locals.var_t2_dn14 = assign92990_e142543_d_n14;
        locals.var_t2_rv = 0.0;

        let assign93000_e142546: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2159 = assign93000_e142546;
        locals.var_guard2159_rv = 0.0;

        let (assign93010_e142559, assign93010_e142559_d_n0, assign93010_e142559_d_n2, assign93010_e142559_d_n4, assign93010_e142559_d_n5, assign93010_e142559_d_n6, assign93010_e142559_d_n7, assign93010_e142559_d_n8, assign93010_e142559_d_n9, assign93010_e142559_d_n10, assign93010_e142559_d_n11, assign93010_e142559_d_n14,) = {
    if (((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2156 != 0.0)) && (locals.var_guard2159 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign93010_e142559;
        locals.var_t2_dn0 = assign93010_e142559_d_n0;
        locals.var_t2_dn2 = assign93010_e142559_d_n2;
        locals.var_t2_dn4 = assign93010_e142559_d_n4;
        locals.var_t2_dn5 = assign93010_e142559_d_n5;
        locals.var_t2_dn6 = assign93010_e142559_d_n6;
        locals.var_t2_dn7 = assign93010_e142559_d_n7;
        locals.var_t2_dn8 = assign93010_e142559_d_n8;
        locals.var_t2_dn9 = assign93010_e142559_d_n9;
        locals.var_t2_dn10 = assign93010_e142559_d_n10;
        locals.var_t2_dn11 = assign93010_e142559_d_n11;
        locals.var_t2_dn14 = assign93010_e142559_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign93020_e142572, assign93020_e142572_d_n0, assign93020_e142572_d_n2, assign93020_e142572_d_n4, assign93020_e142572_d_n5, assign93020_e142572_d_n6, assign93020_e142572_d_n7, assign93020_e142572_d_n8, assign93020_e142572_d_n9, assign93020_e142572_d_n10, assign93020_e142572_d_n11, assign93020_e142572_d_n14,) = {
    if (((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2156 != 0.0)) && (locals.var_guard2159 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign93020_e142572;
        locals.var_t9_dn0 = assign93020_e142572_d_n0;
        locals.var_t9_dn2 = assign93020_e142572_d_n2;
        locals.var_t9_dn4 = assign93020_e142572_d_n4;
        locals.var_t9_dn5 = assign93020_e142572_d_n5;
        locals.var_t9_dn6 = assign93020_e142572_d_n6;
        locals.var_t9_dn7 = assign93020_e142572_d_n7;
        locals.var_t9_dn8 = assign93020_e142572_d_n8;
        locals.var_t9_dn9 = assign93020_e142572_d_n9;
        locals.var_t9_dn10 = assign93020_e142572_d_n10;
        locals.var_t9_dn11 = assign93020_e142572_d_n11;
        locals.var_t9_dn14 = assign93020_e142572_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign93030_e142583, assign93030_e142583_d_n0, assign93030_e142583_d_n2, assign93030_e142583_d_n4, assign93030_e142583_d_n5, assign93030_e142583_d_n6, assign93030_e142583_d_n7, assign93030_e142583_d_n8, assign93030_e142583_d_n9, assign93030_e142583_d_n10, assign93030_e142583_d_n11, assign93030_e142583_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2156 != 0.0)) {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    } else {
        (locals.var_ddriftldc, locals.var_ddriftldc_dn0, locals.var_ddriftldc_dn2, locals.var_ddriftldc_dn4, locals.var_ddriftldc_dn5, locals.var_ddriftldc_dn6, locals.var_ddriftldc_dn7, locals.var_ddriftldc_dn8, locals.var_ddriftldc_dn9, locals.var_ddriftldc_dn10, locals.var_ddriftldc_dn11, locals.var_ddriftldc_dn14,)
    }
};
        locals.var_ddriftldc = assign93030_e142583;
        locals.var_ddriftldc_dn0 = assign93030_e142583_d_n0;
        locals.var_ddriftldc_dn2 = assign93030_e142583_d_n2;
        locals.var_ddriftldc_dn4 = assign93030_e142583_d_n4;
        locals.var_ddriftldc_dn5 = assign93030_e142583_d_n5;
        locals.var_ddriftldc_dn6 = assign93030_e142583_d_n6;
        locals.var_ddriftldc_dn7 = assign93030_e142583_d_n7;
        locals.var_ddriftldc_dn8 = assign93030_e142583_d_n8;
        locals.var_ddriftldc_dn9 = assign93030_e142583_d_n9;
        locals.var_ddriftldc_dn10 = assign93030_e142583_d_n10;
        locals.var_ddriftldc_dn11 = assign93030_e142583_d_n11;
        locals.var_ddriftldc_dn14 = assign93030_e142583_d_n14;
        locals.var_ddriftldc_rv = 0.0;

        let (assign93040_e142602, assign93040_e142602_d_n0, assign93040_e142602_d_n2, assign93040_e142602_d_n4, assign93040_e142602_d_n5, assign93040_e142602_d_n6, assign93040_e142602_d_n7, assign93040_e142602_d_n8, assign93040_e142602_d_n9, assign93040_e142602_d_n10, assign93040_e142602_d_n11, assign93040_e142602_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2156 != 0.0)) {
        let assign93040_e142594: f64 = (locals.var_q_nsubld__blk2119 * locals.var_ddriftldc);
        let assign93040_e142596: f64 = (assign93040_e142594 * locals.var_ddriftldc);
        let assign93040_e142598: f64 = (assign93040_e142596 / 2.0);
        let assign93040_e142600: f64 = (assign93040_e142598 / 1.034943e-10);
        (assign93040_e142600, (((((locals.var_q_nsubld__blk2119 * locals.var_ddriftldc_dn0) * locals.var_ddriftldc) + (assign93040_e142594 * locals.var_ddriftldc_dn0)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2119 * locals.var_ddriftldc_dn2) * locals.var_ddriftldc) + (assign93040_e142594 * locals.var_ddriftldc_dn2)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2119 * locals.var_ddriftldc_dn4) * locals.var_ddriftldc) + (assign93040_e142594 * locals.var_ddriftldc_dn4)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2119 * locals.var_ddriftldc_dn5) * locals.var_ddriftldc) + (assign93040_e142594 * locals.var_ddriftldc_dn5)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2119 * locals.var_ddriftldc_dn6) * locals.var_ddriftldc) + (assign93040_e142594 * locals.var_ddriftldc_dn6)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2119 * locals.var_ddriftldc_dn7) * locals.var_ddriftldc) + (assign93040_e142594 * locals.var_ddriftldc_dn7)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2119 * locals.var_ddriftldc_dn8) * locals.var_ddriftldc) + (assign93040_e142594 * locals.var_ddriftldc_dn8)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2119 * locals.var_ddriftldc_dn9) * locals.var_ddriftldc) + (assign93040_e142594 * locals.var_ddriftldc_dn9)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2119 * locals.var_ddriftldc_dn10) * locals.var_ddriftldc) + (assign93040_e142594 * locals.var_ddriftldc_dn10)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2119 * locals.var_ddriftldc_dn11) * locals.var_ddriftldc) + (assign93040_e142594 * locals.var_ddriftldc_dn11)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2119 * locals.var_ddriftldc_dn14) * locals.var_ddriftldc) + (assign93040_e142594 * locals.var_ddriftldc_dn14)) / 2.0) / 1.034943e-10),)
    } else {
        (locals.var_dphi_sb, locals.var_dphi_sb_dn0, locals.var_dphi_sb_dn2, locals.var_dphi_sb_dn4, locals.var_dphi_sb_dn5, locals.var_dphi_sb_dn6, locals.var_dphi_sb_dn7, locals.var_dphi_sb_dn8, locals.var_dphi_sb_dn9, locals.var_dphi_sb_dn10, locals.var_dphi_sb_dn11, locals.var_dphi_sb_dn14,)
    }
};
        locals.var_dphi_sb = assign93040_e142602;
        locals.var_dphi_sb_dn0 = assign93040_e142602_d_n0;
        locals.var_dphi_sb_dn2 = assign93040_e142602_d_n2;
        locals.var_dphi_sb_dn4 = assign93040_e142602_d_n4;
        locals.var_dphi_sb_dn5 = assign93040_e142602_d_n5;
        locals.var_dphi_sb_dn6 = assign93040_e142602_d_n6;
        locals.var_dphi_sb_dn7 = assign93040_e142602_d_n7;
        locals.var_dphi_sb_dn8 = assign93040_e142602_d_n8;
        locals.var_dphi_sb_dn9 = assign93040_e142602_d_n9;
        locals.var_dphi_sb_dn10 = assign93040_e142602_d_n10;
        locals.var_dphi_sb_dn11 = assign93040_e142602_d_n11;
        locals.var_dphi_sb_dn14 = assign93040_e142602_d_n14;
        locals.var_dphi_sb_rv = 0.0;

        let (assign93050_e142618, assign93050_e142618_d_n0, assign93050_e142618_d_n2, assign93050_e142618_d_n4, assign93050_e142618_d_n5, assign93050_e142618_d_n6, assign93050_e142618_d_n7, assign93050_e142618_d_n8, assign93050_e142618_d_n9, assign93050_e142618_d_n10, assign93050_e142618_d_n11, assign93050_e142618_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2156 != 0.0)) {
        let assign93050_e142613: f64 = (2.0 * locals.var_beta);
        let assign93050_e142615: f64 = (assign93050_e142613 * locals.var_dphi_sb);
        let assign93050_e142616: f64 = (assign93050_e142615).sqrt();
        (assign93050_e142616, ((((2.0 * locals.var_beta_dn0) * locals.var_dphi_sb) + (assign93050_e142613 * locals.var_dphi_sb_dn0)) / (2.0 * assign93050_e142616)), ((((2.0 * locals.var_beta_dn2) * locals.var_dphi_sb) + (assign93050_e142613 * locals.var_dphi_sb_dn2)) / (2.0 * assign93050_e142616)), ((((2.0 * locals.var_beta_dn4) * locals.var_dphi_sb) + (assign93050_e142613 * locals.var_dphi_sb_dn4)) / (2.0 * assign93050_e142616)), ((((2.0 * locals.var_beta_dn5) * locals.var_dphi_sb) + (assign93050_e142613 * locals.var_dphi_sb_dn5)) / (2.0 * assign93050_e142616)), ((((2.0 * locals.var_beta_dn6) * locals.var_dphi_sb) + (assign93050_e142613 * locals.var_dphi_sb_dn6)) / (2.0 * assign93050_e142616)), ((((2.0 * locals.var_beta_dn7) * locals.var_dphi_sb) + (assign93050_e142613 * locals.var_dphi_sb_dn7)) / (2.0 * assign93050_e142616)), ((((2.0 * locals.var_beta_dn8) * locals.var_dphi_sb) + (assign93050_e142613 * locals.var_dphi_sb_dn8)) / (2.0 * assign93050_e142616)), ((((2.0 * locals.var_beta_dn9) * locals.var_dphi_sb) + (assign93050_e142613 * locals.var_dphi_sb_dn9)) / (2.0 * assign93050_e142616)), ((((2.0 * locals.var_beta_dn10) * locals.var_dphi_sb) + (assign93050_e142613 * locals.var_dphi_sb_dn10)) / (2.0 * assign93050_e142616)), ((((2.0 * locals.var_beta_dn11) * locals.var_dphi_sb) + (assign93050_e142613 * locals.var_dphi_sb_dn11)) / (2.0 * assign93050_e142616)), ((((2.0 * locals.var_beta_dn14) * locals.var_dphi_sb) + (assign93050_e142613 * locals.var_dphi_sb_dn14)) / (2.0 * assign93050_e142616)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign93050_e142618;
        locals.var_t0_dn0 = assign93050_e142618_d_n0;
        locals.var_t0_dn2 = assign93050_e142618_d_n2;
        locals.var_t0_dn4 = assign93050_e142618_d_n4;
        locals.var_t0_dn5 = assign93050_e142618_d_n5;
        locals.var_t0_dn6 = assign93050_e142618_d_n6;
        locals.var_t0_dn7 = assign93050_e142618_d_n7;
        locals.var_t0_dn8 = assign93050_e142618_d_n8;
        locals.var_t0_dn9 = assign93050_e142618_d_n9;
        locals.var_t0_dn10 = assign93050_e142618_d_n10;
        locals.var_t0_dn11 = assign93050_e142618_d_n11;
        locals.var_t0_dn14 = assign93050_e142618_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign93060_e142636, assign93060_e142636_d_n0, assign93060_e142636_d_n2, assign93060_e142636_d_n4, assign93060_e142636_d_n5, assign93060_e142636_d_n6, assign93060_e142636_d_n7, assign93060_e142636_d_n8, assign93060_e142636_d_n9, assign93060_e142636_d_n10, assign93060_e142636_d_n11, assign93060_e142636_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2156 != 0.0)) {
        let assign93060_e142628: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign93060_e142630: f64 = (-locals.var_t0);
        let assign93060_e142631: f64 = { let limited_exp_arg = assign93060_e142630; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign93060_e142632: f64 = (assign93060_e142628 + assign93060_e142631);
        let assign93060_e142634: f64 = (assign93060_e142632 / 2.0);
        (assign93060_e142634, ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn0) + ({ let limited_exp_arg = assign93060_e142630; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn0))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn2) + ({ let limited_exp_arg = assign93060_e142630; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn2))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4) + ({ let limited_exp_arg = assign93060_e142630; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn4))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5) + ({ let limited_exp_arg = assign93060_e142630; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn5))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6) + ({ let limited_exp_arg = assign93060_e142630; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn6))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7) + ({ let limited_exp_arg = assign93060_e142630; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn7))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8) + ({ let limited_exp_arg = assign93060_e142630; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn8))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9) + ({ let limited_exp_arg = assign93060_e142630; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn9))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10) + ({ let limited_exp_arg = assign93060_e142630; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn10))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11) + ({ let limited_exp_arg = assign93060_e142630; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn11))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn14) + ({ let limited_exp_arg = assign93060_e142630; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn14))) / 2.0),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign93060_e142636;
        locals.var_t1_dn0 = assign93060_e142636_d_n0;
        locals.var_t1_dn2 = assign93060_e142636_d_n2;
        locals.var_t1_dn4 = assign93060_e142636_d_n4;
        locals.var_t1_dn5 = assign93060_e142636_d_n5;
        locals.var_t1_dn6 = assign93060_e142636_d_n6;
        locals.var_t1_dn7 = assign93060_e142636_d_n7;
        locals.var_t1_dn8 = assign93060_e142636_d_n8;
        locals.var_t1_dn9 = assign93060_e142636_d_n9;
        locals.var_t1_dn10 = assign93060_e142636_d_n10;
        locals.var_t1_dn11 = assign93060_e142636_d_n11;
        locals.var_t1_dn14 = assign93060_e142636_d_n14;
        locals.var_t1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_359(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign93070_e142650, assign93070_e142650_d_n0, assign93070_e142650_d_n2, assign93070_e142650_d_n4, assign93070_e142650_d_n5, assign93070_e142650_d_n6, assign93070_e142650_d_n7, assign93070_e142650_d_n8, assign93070_e142650_d_n9, assign93070_e142650_d_n10, assign93070_e142650_d_n11, assign93070_e142650_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2156 != 0.0)) {
        let assign93070_e142646: f64 = (locals.var_t1).ln();
        let assign93070_e142648: f64 = (assign93070_e142646 / locals.var_dphi_sb);
        (assign93070_e142648, ((((locals.var_t1_dn0 / locals.var_t1) * locals.var_dphi_sb) - (assign93070_e142646 * locals.var_dphi_sb_dn0)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn2 / locals.var_t1) * locals.var_dphi_sb) - (assign93070_e142646 * locals.var_dphi_sb_dn2)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn4 / locals.var_t1) * locals.var_dphi_sb) - (assign93070_e142646 * locals.var_dphi_sb_dn4)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn5 / locals.var_t1) * locals.var_dphi_sb) - (assign93070_e142646 * locals.var_dphi_sb_dn5)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn6 / locals.var_t1) * locals.var_dphi_sb) - (assign93070_e142646 * locals.var_dphi_sb_dn6)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn7 / locals.var_t1) * locals.var_dphi_sb) - (assign93070_e142646 * locals.var_dphi_sb_dn7)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn8 / locals.var_t1) * locals.var_dphi_sb) - (assign93070_e142646 * locals.var_dphi_sb_dn8)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn9 / locals.var_t1) * locals.var_dphi_sb) - (assign93070_e142646 * locals.var_dphi_sb_dn9)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn10 / locals.var_t1) * locals.var_dphi_sb) - (assign93070_e142646 * locals.var_dphi_sb_dn10)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn11 / locals.var_t1) * locals.var_dphi_sb) - (assign93070_e142646 * locals.var_dphi_sb_dn11)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn14 / locals.var_t1) * locals.var_dphi_sb) - (assign93070_e142646 * locals.var_dphi_sb_dn14)) / (locals.var_dphi_sb * locals.var_dphi_sb)),)
    } else {
        (locals.var_c_sb, locals.var_c_sb_dn0, locals.var_c_sb_dn2, locals.var_c_sb_dn4, locals.var_c_sb_dn5, locals.var_c_sb_dn6, locals.var_c_sb_dn7, locals.var_c_sb_dn8, locals.var_c_sb_dn9, locals.var_c_sb_dn10, locals.var_c_sb_dn11, locals.var_c_sb_dn14,)
    }
};
        locals.var_c_sb = assign93070_e142650;
        locals.var_c_sb_dn0 = assign93070_e142650_d_n0;
        locals.var_c_sb_dn2 = assign93070_e142650_d_n2;
        locals.var_c_sb_dn4 = assign93070_e142650_d_n4;
        locals.var_c_sb_dn5 = assign93070_e142650_d_n5;
        locals.var_c_sb_dn6 = assign93070_e142650_d_n6;
        locals.var_c_sb_dn7 = assign93070_e142650_d_n7;
        locals.var_c_sb_dn8 = assign93070_e142650_d_n8;
        locals.var_c_sb_dn9 = assign93070_e142650_d_n9;
        locals.var_c_sb_dn10 = assign93070_e142650_d_n10;
        locals.var_c_sb_dn11 = assign93070_e142650_d_n11;
        locals.var_c_sb_dn14 = assign93070_e142650_d_n14;
        locals.var_c_sb_rv = 0.0;

        let (assign93080_e142663, assign93080_e142663_d_n0, assign93080_e142663_d_n2, assign93080_e142663_d_n4, assign93080_e142663_d_n5, assign93080_e142663_d_n6, assign93080_e142663_d_n7, assign93080_e142663_d_n8, assign93080_e142663_d_n9, assign93080_e142663_d_n10, assign93080_e142663_d_n11, assign93080_e142663_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2156 != 0.0)) {
        let assign93080_e142661: f64 = (locals.var_ps0ld + locals.var_vxbgmtcl);
        (assign93080_e142661, (locals.var_ps0ld_dn0 + locals.var_vxbgmtcl_dn0), (locals.var_ps0ld_dn2 + locals.var_vxbgmtcl_dn2), (locals.var_ps0ld_dn4 + locals.var_vxbgmtcl_dn4), (locals.var_ps0ld_dn5 + locals.var_vxbgmtcl_dn5), (locals.var_ps0ld_dn6 + locals.var_vxbgmtcl_dn6), (locals.var_ps0ld_dn7 + locals.var_vxbgmtcl_dn7), (locals.var_ps0ld_dn8 + locals.var_vxbgmtcl_dn8), (locals.var_ps0ld_dn9 + locals.var_vxbgmtcl_dn9), (locals.var_ps0ld_dn10 + locals.var_vxbgmtcl_dn10), (locals.var_ps0ld_dn11 + locals.var_vxbgmtcl_dn11), (locals.var_ps0ld_dn14 + locals.var_vxbgmtcl_dn14),)
    } else {
        (locals.var_ps0ld_vxb, locals.var_ps0ld_vxb_dn0, locals.var_ps0ld_vxb_dn2, locals.var_ps0ld_vxb_dn4, locals.var_ps0ld_vxb_dn5, locals.var_ps0ld_vxb_dn6, locals.var_ps0ld_vxb_dn7, locals.var_ps0ld_vxb_dn8, locals.var_ps0ld_vxb_dn9, locals.var_ps0ld_vxb_dn10, locals.var_ps0ld_vxb_dn11, locals.var_ps0ld_vxb_dn14,)
    }
};
        locals.var_ps0ld_vxb = assign93080_e142663;
        locals.var_ps0ld_vxb_dn0 = assign93080_e142663_d_n0;
        locals.var_ps0ld_vxb_dn2 = assign93080_e142663_d_n2;
        locals.var_ps0ld_vxb_dn4 = assign93080_e142663_d_n4;
        locals.var_ps0ld_vxb_dn5 = assign93080_e142663_d_n5;
        locals.var_ps0ld_vxb_dn6 = assign93080_e142663_d_n6;
        locals.var_ps0ld_vxb_dn7 = assign93080_e142663_d_n7;
        locals.var_ps0ld_vxb_dn8 = assign93080_e142663_d_n8;
        locals.var_ps0ld_vxb_dn9 = assign93080_e142663_d_n9;
        locals.var_ps0ld_vxb_dn10 = assign93080_e142663_d_n10;
        locals.var_ps0ld_vxb_dn11 = assign93080_e142663_d_n11;
        locals.var_ps0ld_vxb_dn14 = assign93080_e142663_d_n14;
        locals.var_ps0ld_vxb_rv = 0.0;

        let (assign93090_e142678, assign93090_e142678_d_n0, assign93090_e142678_d_n2, assign93090_e142678_d_n4, assign93090_e142678_d_n5, assign93090_e142678_d_n6, assign93090_e142678_d_n7, assign93090_e142678_d_n8, assign93090_e142678_d_n9, assign93090_e142678_d_n10, assign93090_e142678_d_n11, assign93090_e142678_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2156 != 0.0)) {
        let assign93090_e142675: f64 = (locals.var_ps0ld_vxb - locals.var_dphi_sb);
        let assign93090_e142676: f64 = (locals.var_c_sb * assign93090_e142675);
        (assign93090_e142676, ((locals.var_c_sb_dn0 * assign93090_e142675) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn0 - locals.var_dphi_sb_dn0))), ((locals.var_c_sb_dn2 * assign93090_e142675) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn2 - locals.var_dphi_sb_dn2))), ((locals.var_c_sb_dn4 * assign93090_e142675) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn4 - locals.var_dphi_sb_dn4))), ((locals.var_c_sb_dn5 * assign93090_e142675) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn5 - locals.var_dphi_sb_dn5))), ((locals.var_c_sb_dn6 * assign93090_e142675) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn6 - locals.var_dphi_sb_dn6))), ((locals.var_c_sb_dn7 * assign93090_e142675) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn7 - locals.var_dphi_sb_dn7))), ((locals.var_c_sb_dn8 * assign93090_e142675) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn8 - locals.var_dphi_sb_dn8))), ((locals.var_c_sb_dn9 * assign93090_e142675) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn9 - locals.var_dphi_sb_dn9))), ((locals.var_c_sb_dn10 * assign93090_e142675) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn10 - locals.var_dphi_sb_dn10))), ((locals.var_c_sb_dn11 * assign93090_e142675) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn11 - locals.var_dphi_sb_dn11))), ((locals.var_c_sb_dn14 * assign93090_e142675) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn14 - locals.var_dphi_sb_dn14))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn14,)
    }
};
        locals.var_ty = assign93090_e142678;
        locals.var_ty_dn0 = assign93090_e142678_d_n0;
        locals.var_ty_dn2 = assign93090_e142678_d_n2;
        locals.var_ty_dn4 = assign93090_e142678_d_n4;
        locals.var_ty_dn5 = assign93090_e142678_d_n5;
        locals.var_ty_dn6 = assign93090_e142678_d_n6;
        locals.var_ty_dn7 = assign93090_e142678_d_n7;
        locals.var_ty_dn8 = assign93090_e142678_d_n8;
        locals.var_ty_dn9 = assign93090_e142678_d_n9;
        locals.var_ty_dn10 = assign93090_e142678_d_n10;
        locals.var_ty_dn11 = assign93090_e142678_d_n11;
        locals.var_ty_dn14 = assign93090_e142678_d_n14;
        locals.var_ty_rv = 0.0;

        let assign93100_e142681: f64 = if locals.var_ty < 60.0 { 1.0 } else { 0.0 };
        locals.var_guard2160 = assign93100_e142681;
        locals.var_guard2160_rv = 0.0;

        let (assign93110_e142695, assign93110_e142695_d_n0, assign93110_e142695_d_n2, assign93110_e142695_d_n4, assign93110_e142695_d_n5, assign93110_e142695_d_n6, assign93110_e142695_d_n7, assign93110_e142695_d_n8, assign93110_e142695_d_n9, assign93110_e142695_d_n10, assign93110_e142695_d_n11, assign93110_e142695_d_n14,) = {
    if (((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2156 != 0.0)) && (locals.var_guard2160 != 0.0)) {
        let assign93110_e142693: f64 = (locals.var_ty).exp();
        (assign93110_e142693, (assign93110_e142693 * locals.var_ty_dn0), (assign93110_e142693 * locals.var_ty_dn2), (assign93110_e142693 * locals.var_ty_dn4), (assign93110_e142693 * locals.var_ty_dn5), (assign93110_e142693 * locals.var_ty_dn6), (assign93110_e142693 * locals.var_ty_dn7), (assign93110_e142693 * locals.var_ty_dn8), (assign93110_e142693 * locals.var_ty_dn9), (assign93110_e142693 * locals.var_ty_dn10), (assign93110_e142693 * locals.var_ty_dn11), (assign93110_e142693 * locals.var_ty_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign93110_e142695;
        locals.var_t1_dn0 = assign93110_e142695_d_n0;
        locals.var_t1_dn2 = assign93110_e142695_d_n2;
        locals.var_t1_dn4 = assign93110_e142695_d_n4;
        locals.var_t1_dn5 = assign93110_e142695_d_n5;
        locals.var_t1_dn6 = assign93110_e142695_d_n6;
        locals.var_t1_dn7 = assign93110_e142695_d_n7;
        locals.var_t1_dn8 = assign93110_e142695_d_n8;
        locals.var_t1_dn9 = assign93110_e142695_d_n9;
        locals.var_t1_dn10 = assign93110_e142695_d_n10;
        locals.var_t1_dn11 = assign93110_e142695_d_n11;
        locals.var_t1_dn14 = assign93110_e142695_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign93120_e142712, assign93120_e142712_d_n0, assign93120_e142712_d_n2, assign93120_e142712_d_n4, assign93120_e142712_d_n5, assign93120_e142712_d_n6, assign93120_e142712_d_n7, assign93120_e142712_d_n8, assign93120_e142712_d_n9, assign93120_e142712_d_n10, assign93120_e142712_d_n11, assign93120_e142712_d_n14,) = {
    if (((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2156 != 0.0)) && (locals.var_guard2160 != 0.0)) {
        let assign93120_e142707: f64 = (-locals.var_c_sb);
        let assign93120_e142709: f64 = (assign93120_e142707 * locals.var_dphi_sb);
        let assign93120_e142710: f64 = (assign93120_e142709).exp();
        (assign93120_e142710, (assign93120_e142710 * (((-locals.var_c_sb_dn0) * locals.var_dphi_sb) + (assign93120_e142707 * locals.var_dphi_sb_dn0))), (assign93120_e142710 * (((-locals.var_c_sb_dn2) * locals.var_dphi_sb) + (assign93120_e142707 * locals.var_dphi_sb_dn2))), (assign93120_e142710 * (((-locals.var_c_sb_dn4) * locals.var_dphi_sb) + (assign93120_e142707 * locals.var_dphi_sb_dn4))), (assign93120_e142710 * (((-locals.var_c_sb_dn5) * locals.var_dphi_sb) + (assign93120_e142707 * locals.var_dphi_sb_dn5))), (assign93120_e142710 * (((-locals.var_c_sb_dn6) * locals.var_dphi_sb) + (assign93120_e142707 * locals.var_dphi_sb_dn6))), (assign93120_e142710 * (((-locals.var_c_sb_dn7) * locals.var_dphi_sb) + (assign93120_e142707 * locals.var_dphi_sb_dn7))), (assign93120_e142710 * (((-locals.var_c_sb_dn8) * locals.var_dphi_sb) + (assign93120_e142707 * locals.var_dphi_sb_dn8))), (assign93120_e142710 * (((-locals.var_c_sb_dn9) * locals.var_dphi_sb) + (assign93120_e142707 * locals.var_dphi_sb_dn9))), (assign93120_e142710 * (((-locals.var_c_sb_dn10) * locals.var_dphi_sb) + (assign93120_e142707 * locals.var_dphi_sb_dn10))), (assign93120_e142710 * (((-locals.var_c_sb_dn11) * locals.var_dphi_sb) + (assign93120_e142707 * locals.var_dphi_sb_dn11))), (assign93120_e142710 * (((-locals.var_c_sb_dn14) * locals.var_dphi_sb) + (assign93120_e142707 * locals.var_dphi_sb_dn14))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign93120_e142712;
        locals.var_t0_dn0 = assign93120_e142712_d_n0;
        locals.var_t0_dn2 = assign93120_e142712_d_n2;
        locals.var_t0_dn4 = assign93120_e142712_d_n4;
        locals.var_t0_dn5 = assign93120_e142712_d_n5;
        locals.var_t0_dn6 = assign93120_e142712_d_n6;
        locals.var_t0_dn7 = assign93120_e142712_d_n7;
        locals.var_t0_dn8 = assign93120_e142712_d_n8;
        locals.var_t0_dn9 = assign93120_e142712_d_n9;
        locals.var_t0_dn10 = assign93120_e142712_d_n10;
        locals.var_t0_dn11 = assign93120_e142712_d_n11;
        locals.var_t0_dn14 = assign93120_e142712_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign93130_e142727, assign93130_e142727_d_n0, assign93130_e142727_d_n2, assign93130_e142727_d_n4, assign93130_e142727_d_n5, assign93130_e142727_d_n6, assign93130_e142727_d_n7, assign93130_e142727_d_n8, assign93130_e142727_d_n9, assign93130_e142727_d_n10, assign93130_e142727_d_n11, assign93130_e142727_d_n14,) = {
    if (((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2156 != 0.0)) && (locals.var_guard2160 != 0.0)) {
        let assign93130_e142725: f64 = (locals.var_t1 - locals.var_t0);
        (assign93130_e142725, (locals.var_t1_dn0 - locals.var_t0_dn0), (locals.var_t1_dn2 - locals.var_t0_dn2), (locals.var_t1_dn4 - locals.var_t0_dn4), (locals.var_t1_dn5 - locals.var_t0_dn5), (locals.var_t1_dn6 - locals.var_t0_dn6), (locals.var_t1_dn7 - locals.var_t0_dn7), (locals.var_t1_dn8 - locals.var_t0_dn8), (locals.var_t1_dn9 - locals.var_t0_dn9), (locals.var_t1_dn10 - locals.var_t0_dn10), (locals.var_t1_dn11 - locals.var_t0_dn11), (locals.var_t1_dn14 - locals.var_t0_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign93130_e142727;
        locals.var_t2_dn0 = assign93130_e142727_d_n0;
        locals.var_t2_dn2 = assign93130_e142727_d_n2;
        locals.var_t2_dn4 = assign93130_e142727_d_n4;
        locals.var_t2_dn5 = assign93130_e142727_d_n5;
        locals.var_t2_dn6 = assign93130_e142727_d_n6;
        locals.var_t2_dn7 = assign93130_e142727_d_n7;
        locals.var_t2_dn8 = assign93130_e142727_d_n8;
        locals.var_t2_dn9 = assign93130_e142727_d_n9;
        locals.var_t2_dn10 = assign93130_e142727_d_n10;
        locals.var_t2_dn11 = assign93130_e142727_d_n11;
        locals.var_t2_dn14 = assign93130_e142727_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign93140_e142745, assign93140_e142745_d_n0, assign93140_e142745_d_n2, assign93140_e142745_d_n4, assign93140_e142745_d_n5, assign93140_e142745_d_n6, assign93140_e142745_d_n7, assign93140_e142745_d_n8, assign93140_e142745_d_n9, assign93140_e142745_d_n10, assign93140_e142745_d_n11, assign93140_e142745_d_n14,) = {
    if (((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2156 != 0.0)) && (locals.var_guard2160 != 0.0)) {
        let assign93140_e142740: f64 = (1.0 + locals.var_t2);
        let assign93140_e142741: f64 = (assign93140_e142740).ln();
        let assign93140_e142743: f64 = (assign93140_e142741 / locals.var_c_sb);
        (assign93140_e142743, ((((locals.var_t2_dn0 / assign93140_e142740) * locals.var_c_sb) - (assign93140_e142741 * locals.var_c_sb_dn0)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn2 / assign93140_e142740) * locals.var_c_sb) - (assign93140_e142741 * locals.var_c_sb_dn2)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn4 / assign93140_e142740) * locals.var_c_sb) - (assign93140_e142741 * locals.var_c_sb_dn4)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn5 / assign93140_e142740) * locals.var_c_sb) - (assign93140_e142741 * locals.var_c_sb_dn5)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn6 / assign93140_e142740) * locals.var_c_sb) - (assign93140_e142741 * locals.var_c_sb_dn6)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn7 / assign93140_e142740) * locals.var_c_sb) - (assign93140_e142741 * locals.var_c_sb_dn7)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn8 / assign93140_e142740) * locals.var_c_sb) - (assign93140_e142741 * locals.var_c_sb_dn8)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn9 / assign93140_e142740) * locals.var_c_sb) - (assign93140_e142741 * locals.var_c_sb_dn9)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn10 / assign93140_e142740) * locals.var_c_sb) - (assign93140_e142741 * locals.var_c_sb_dn10)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn11 / assign93140_e142740) * locals.var_c_sb) - (assign93140_e142741 * locals.var_c_sb_dn11)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn14 / assign93140_e142740) * locals.var_c_sb) - (assign93140_e142741 * locals.var_c_sb_dn14)) / (locals.var_c_sb * locals.var_c_sb)),)
    } else {
        (locals.var_phi_b, locals.var_phi_b_dn0, locals.var_phi_b_dn2, locals.var_phi_b_dn4, locals.var_phi_b_dn5, locals.var_phi_b_dn6, locals.var_phi_b_dn7, locals.var_phi_b_dn8, locals.var_phi_b_dn9, locals.var_phi_b_dn10, locals.var_phi_b_dn11, locals.var_phi_b_dn14,)
    }
};
        locals.var_phi_b = assign93140_e142745;
        locals.var_phi_b_dn0 = assign93140_e142745_d_n0;
        locals.var_phi_b_dn2 = assign93140_e142745_d_n2;
        locals.var_phi_b_dn4 = assign93140_e142745_d_n4;
        locals.var_phi_b_dn5 = assign93140_e142745_d_n5;
        locals.var_phi_b_dn6 = assign93140_e142745_d_n6;
        locals.var_phi_b_dn7 = assign93140_e142745_d_n7;
        locals.var_phi_b_dn8 = assign93140_e142745_d_n8;
        locals.var_phi_b_dn9 = assign93140_e142745_d_n9;
        locals.var_phi_b_dn10 = assign93140_e142745_d_n10;
        locals.var_phi_b_dn11 = assign93140_e142745_d_n11;
        locals.var_phi_b_dn14 = assign93140_e142745_d_n14;
        locals.var_phi_b_rv = 0.0;

        let (assign93150_e142761, assign93150_e142761_d_n0, assign93150_e142761_d_n2, assign93150_e142761_d_n4, assign93150_e142761_d_n5, assign93150_e142761_d_n6, assign93150_e142761_d_n7, assign93150_e142761_d_n8, assign93150_e142761_d_n9, assign93150_e142761_d_n10, assign93150_e142761_d_n11, assign93150_e142761_d_n14,) = {
    if (((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2156 != 0.0)) && (locals.var_guard2160 == 0.0)) {
        let assign93150_e142759: f64 = (locals.var_ps0ld_vxb - locals.var_dphi_sb);
        (assign93150_e142759, (locals.var_ps0ld_vxb_dn0 - locals.var_dphi_sb_dn0), (locals.var_ps0ld_vxb_dn2 - locals.var_dphi_sb_dn2), (locals.var_ps0ld_vxb_dn4 - locals.var_dphi_sb_dn4), (locals.var_ps0ld_vxb_dn5 - locals.var_dphi_sb_dn5), (locals.var_ps0ld_vxb_dn6 - locals.var_dphi_sb_dn6), (locals.var_ps0ld_vxb_dn7 - locals.var_dphi_sb_dn7), (locals.var_ps0ld_vxb_dn8 - locals.var_dphi_sb_dn8), (locals.var_ps0ld_vxb_dn9 - locals.var_dphi_sb_dn9), (locals.var_ps0ld_vxb_dn10 - locals.var_dphi_sb_dn10), (locals.var_ps0ld_vxb_dn11 - locals.var_dphi_sb_dn11), (locals.var_ps0ld_vxb_dn14 - locals.var_dphi_sb_dn14),)
    } else {
        (locals.var_phi_b, locals.var_phi_b_dn0, locals.var_phi_b_dn2, locals.var_phi_b_dn4, locals.var_phi_b_dn5, locals.var_phi_b_dn6, locals.var_phi_b_dn7, locals.var_phi_b_dn8, locals.var_phi_b_dn9, locals.var_phi_b_dn10, locals.var_phi_b_dn11, locals.var_phi_b_dn14,)
    }
};
        locals.var_phi_b = assign93150_e142761;
        locals.var_phi_b_dn0 = assign93150_e142761_d_n0;
        locals.var_phi_b_dn2 = assign93150_e142761_d_n2;
        locals.var_phi_b_dn4 = assign93150_e142761_d_n4;
        locals.var_phi_b_dn5 = assign93150_e142761_d_n5;
        locals.var_phi_b_dn6 = assign93150_e142761_d_n6;
        locals.var_phi_b_dn7 = assign93150_e142761_d_n7;
        locals.var_phi_b_dn8 = assign93150_e142761_d_n8;
        locals.var_phi_b_dn9 = assign93150_e142761_d_n9;
        locals.var_phi_b_dn10 = assign93150_e142761_d_n10;
        locals.var_phi_b_dn11 = assign93150_e142761_d_n11;
        locals.var_phi_b_dn14 = assign93150_e142761_d_n14;
        locals.var_phi_b_rv = 0.0;

        let (assign93160_e142774, assign93160_e142774_d_n0, assign93160_e142774_d_n2, assign93160_e142774_d_n4, assign93160_e142774_d_n5, assign93160_e142774_d_n6, assign93160_e142774_d_n7, assign93160_e142774_d_n8, assign93160_e142774_d_n9, assign93160_e142774_d_n10, assign93160_e142774_d_n11, assign93160_e142774_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2156 != 0.0)) {
        let assign93160_e142772: f64 = (locals.var_beta * locals.var_phi_b);
        (assign93160_e142772, ((locals.var_beta_dn0 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn0)), ((locals.var_beta_dn2 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn2)), ((locals.var_beta_dn4 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn4)), ((locals.var_beta_dn5 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn5)), ((locals.var_beta_dn6 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn6)), ((locals.var_beta_dn7 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn7)), ((locals.var_beta_dn8 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn8)), ((locals.var_beta_dn9 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn9)), ((locals.var_beta_dn10 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn10)), ((locals.var_beta_dn11 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn11)), ((locals.var_beta_dn14 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn14)),)
    } else {
        (locals.var_chib, locals.var_chib_dn0, locals.var_chib_dn2, locals.var_chib_dn4, locals.var_chib_dn5, locals.var_chib_dn6, locals.var_chib_dn7, locals.var_chib_dn8, locals.var_chib_dn9, locals.var_chib_dn10, locals.var_chib_dn11, locals.var_chib_dn14,)
    }
};
        locals.var_chib = assign93160_e142774;
        locals.var_chib_dn0 = assign93160_e142774_d_n0;
        locals.var_chib_dn2 = assign93160_e142774_d_n2;
        locals.var_chib_dn4 = assign93160_e142774_d_n4;
        locals.var_chib_dn5 = assign93160_e142774_d_n5;
        locals.var_chib_dn6 = assign93160_e142774_d_n6;
        locals.var_chib_dn7 = assign93160_e142774_d_n7;
        locals.var_chib_dn8 = assign93160_e142774_d_n8;
        locals.var_chib_dn9 = assign93160_e142774_d_n9;
        locals.var_chib_dn10 = assign93160_e142774_d_n10;
        locals.var_chib_dn11 = assign93160_e142774_d_n11;
        locals.var_chib_dn14 = assign93160_e142774_d_n14;
        locals.var_chib_rv = 0.0;

        let assign93170_e142778: f64 = (locals.var_chi / 100.0);
        let assign93170_e142783: f64 = if ((locals.var_chib > assign93170_e142778) && (locals.var_chib > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2161 = assign93170_e142783;
        locals.var_guard2161_rv = 0.0;

        let (assign93180_e142798,) = {
    if (((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2156 != 0.0)) && (locals.var_guard2161 != 0.0)) {
        let assign93180_e142796: f64 = (locals.var_flg_fd_mode__blk2125 + 1.0);
        (assign93180_e142796,)
    } else {
        (locals.var_flg_fd_mode__blk2125,)
    }
};
        locals.var_flg_fd_mode__blk2125 = assign93180_e142798;
        locals.var_flg_fd_mode__blk2125_rv = 0.0;

        let (assign93190_e142811, assign93190_e142811_d_n0, assign93190_e142811_d_n2, assign93190_e142811_d_n4, assign93190_e142811_d_n5, assign93190_e142811_d_n6, assign93190_e142811_d_n7, assign93190_e142811_d_n8, assign93190_e142811_d_n9, assign93190_e142811_d_n10, assign93190_e142811_d_n11, assign93190_e142811_d_n14,) = {
    if (((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2156 != 0.0)) && (locals.var_guard2161 != 0.0)) {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn11, locals.var_chi_1_dn14,)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
        locals.var_chi = assign93190_e142811;
        locals.var_chi_dn0 = assign93190_e142811_d_n0;
        locals.var_chi_dn2 = assign93190_e142811_d_n2;
        locals.var_chi_dn4 = assign93190_e142811_d_n4;
        locals.var_chi_dn5 = assign93190_e142811_d_n5;
        locals.var_chi_dn6 = assign93190_e142811_d_n6;
        locals.var_chi_dn7 = assign93190_e142811_d_n7;
        locals.var_chi_dn8 = assign93190_e142811_d_n8;
        locals.var_chi_dn9 = assign93190_e142811_d_n9;
        locals.var_chi_dn10 = assign93190_e142811_d_n10;
        locals.var_chi_dn11 = assign93190_e142811_d_n11;
        locals.var_chi_dn14 = assign93190_e142811_d_n14;
        locals.var_chi_rv = 0.0;

        let (assign93200_e142824, assign93200_e142824_d_n0, assign93200_e142824_d_n2, assign93200_e142824_d_n4, assign93200_e142824_d_n5, assign93200_e142824_d_n6, assign93200_e142824_d_n7, assign93200_e142824_d_n8, assign93200_e142824_d_n9, assign93200_e142824_d_n10, assign93200_e142824_d_n11, assign93200_e142824_d_n14,) = {
    if (((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) {
        let assign93200_e142820: f64 = (locals.var_chi * locals.var_beta_inv);
        let assign93200_e142822: f64 = (assign93200_e142820 - locals.var_vxbgmtcl);
        (assign93200_e142822, (((locals.var_chi_dn0 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn0)) - locals.var_vxbgmtcl_dn0), (((locals.var_chi_dn2 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn2)) - locals.var_vxbgmtcl_dn2), (((locals.var_chi_dn4 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn4)) - locals.var_vxbgmtcl_dn4), (((locals.var_chi_dn5 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn5)) - locals.var_vxbgmtcl_dn5), (((locals.var_chi_dn6 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn6)) - locals.var_vxbgmtcl_dn6), (((locals.var_chi_dn7 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn7)) - locals.var_vxbgmtcl_dn7), (((locals.var_chi_dn8 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn8)) - locals.var_vxbgmtcl_dn8), (((locals.var_chi_dn9 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn9)) - locals.var_vxbgmtcl_dn9), (((locals.var_chi_dn10 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn10)) - locals.var_vxbgmtcl_dn10), (((locals.var_chi_dn11 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn11)) - locals.var_vxbgmtcl_dn11), (((locals.var_chi_dn14 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn14)) - locals.var_vxbgmtcl_dn14),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn14,)
    }
};
        locals.var_ps0ld = assign93200_e142824;
        locals.var_ps0ld_dn0 = assign93200_e142824_d_n0;
        locals.var_ps0ld_dn2 = assign93200_e142824_d_n2;
        locals.var_ps0ld_dn4 = assign93200_e142824_d_n4;
        locals.var_ps0ld_dn5 = assign93200_e142824_d_n5;
        locals.var_ps0ld_dn6 = assign93200_e142824_d_n6;
        locals.var_ps0ld_dn7 = assign93200_e142824_d_n7;
        locals.var_ps0ld_dn8 = assign93200_e142824_d_n8;
        locals.var_ps0ld_dn9 = assign93200_e142824_d_n9;
        locals.var_ps0ld_dn10 = assign93200_e142824_d_n10;
        locals.var_ps0ld_dn11 = assign93200_e142824_d_n11;
        locals.var_ps0ld_dn14 = assign93200_e142824_d_n14;
        locals.var_ps0ld_rv = 0.0;

        let assign93210_e142826: f64 = (locals.var_chi).abs();
        let assign93210_e142828: f64 = if assign93210_e142826 > 1e-6 { 1.0 } else { 0.0 };
        locals.var_guard2162 = assign93210_e142828;
        locals.var_guard2162_rv = 0.0;

        let (assign93220_e142845, assign93220_e142845_d_n0, assign93220_e142845_d_n2, assign93220_e142845_d_n4, assign93220_e142845_d_n5, assign93220_e142845_d_n6, assign93220_e142845_d_n7, assign93220_e142845_d_n8, assign93220_e142845_d_n9, assign93220_e142845_d_n10, assign93220_e142845_d_n11, assign93220_e142845_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2162 != 0.0)) {
        let assign93220_e142839: f64 = (locals.var_chi - 1.0);
        let assign93220_e142841: f64 = (-locals.var_chi);
        let assign93220_e142842: f64 = (assign93220_e142841).exp();
        let assign93220_e142843: f64 = (assign93220_e142839 + assign93220_e142842);
        (assign93220_e142843, (locals.var_chi_dn0 + (assign93220_e142842 * (-locals.var_chi_dn0))), (locals.var_chi_dn2 + (assign93220_e142842 * (-locals.var_chi_dn2))), (locals.var_chi_dn4 + (assign93220_e142842 * (-locals.var_chi_dn4))), (locals.var_chi_dn5 + (assign93220_e142842 * (-locals.var_chi_dn5))), (locals.var_chi_dn6 + (assign93220_e142842 * (-locals.var_chi_dn6))), (locals.var_chi_dn7 + (assign93220_e142842 * (-locals.var_chi_dn7))), (locals.var_chi_dn8 + (assign93220_e142842 * (-locals.var_chi_dn8))), (locals.var_chi_dn9 + (assign93220_e142842 * (-locals.var_chi_dn9))), (locals.var_chi_dn10 + (assign93220_e142842 * (-locals.var_chi_dn10))), (locals.var_chi_dn11 + (assign93220_e142842 * (-locals.var_chi_dn11))), (locals.var_chi_dn14 + (assign93220_e142842 * (-locals.var_chi_dn14))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign93220_e142845;
        locals.var_t1_dn0 = assign93220_e142845_d_n0;
        locals.var_t1_dn2 = assign93220_e142845_d_n2;
        locals.var_t1_dn4 = assign93220_e142845_d_n4;
        locals.var_t1_dn5 = assign93220_e142845_d_n5;
        locals.var_t1_dn6 = assign93220_e142845_d_n6;
        locals.var_t1_dn7 = assign93220_e142845_d_n7;
        locals.var_t1_dn8 = assign93220_e142845_d_n8;
        locals.var_t1_dn9 = assign93220_e142845_d_n9;
        locals.var_t1_dn10 = assign93220_e142845_d_n10;
        locals.var_t1_dn11 = assign93220_e142845_d_n11;
        locals.var_t1_dn14 = assign93220_e142845_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign93230_e142857, assign93230_e142857_d_n0, assign93230_e142857_d_n2, assign93230_e142857_d_n4, assign93230_e142857_d_n5, assign93230_e142857_d_n6, assign93230_e142857_d_n7, assign93230_e142857_d_n8, assign93230_e142857_d_n9, assign93230_e142857_d_n10, assign93230_e142857_d_n11, assign93230_e142857_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2162 != 0.0)) {
        let assign93230_e142855: f64 = (locals.var_t1).sqrt();
        (assign93230_e142855, (locals.var_t1_dn0 / (2.0 * assign93230_e142855)), (locals.var_t1_dn2 / (2.0 * assign93230_e142855)), (locals.var_t1_dn4 / (2.0 * assign93230_e142855)), (locals.var_t1_dn5 / (2.0 * assign93230_e142855)), (locals.var_t1_dn6 / (2.0 * assign93230_e142855)), (locals.var_t1_dn7 / (2.0 * assign93230_e142855)), (locals.var_t1_dn8 / (2.0 * assign93230_e142855)), (locals.var_t1_dn9 / (2.0 * assign93230_e142855)), (locals.var_t1_dn10 / (2.0 * assign93230_e142855)), (locals.var_t1_dn11 / (2.0 * assign93230_e142855)), (locals.var_t1_dn14 / (2.0 * assign93230_e142855)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign93230_e142857;
        locals.var_t2_dn0 = assign93230_e142857_d_n0;
        locals.var_t2_dn2 = assign93230_e142857_d_n2;
        locals.var_t2_dn4 = assign93230_e142857_d_n4;
        locals.var_t2_dn5 = assign93230_e142857_d_n5;
        locals.var_t2_dn6 = assign93230_e142857_d_n6;
        locals.var_t2_dn7 = assign93230_e142857_d_n7;
        locals.var_t2_dn8 = assign93230_e142857_d_n8;
        locals.var_t2_dn9 = assign93230_e142857_d_n9;
        locals.var_t2_dn10 = assign93230_e142857_d_n10;
        locals.var_t2_dn11 = assign93230_e142857_d_n11;
        locals.var_t2_dn14 = assign93230_e142857_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign93250_e142892, assign93250_e142892_d_n0, assign93250_e142892_d_n2, assign93250_e142892_d_n4, assign93250_e142892_d_n5, assign93250_e142892_d_n6, assign93250_e142892_d_n7, assign93250_e142892_d_n8, assign93250_e142892_d_n9, assign93250_e142892_d_n10, assign93250_e142892_d_n11, assign93250_e142892_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2162 == 0.0)) {
        let assign93250_e142883: f64 = (0.7071067811865475 * locals.var_chi);
        let assign93250_e142887: f64 = (locals.var_chi * 0.3333333333333333);
        let assign93250_e142888: f64 = (1.0 - assign93250_e142887);
        let assign93250_e142889: f64 = (assign93250_e142888).sqrt();
        let assign93250_e142890: f64 = (assign93250_e142883 * assign93250_e142889);
        (assign93250_e142890, (((0.7071067811865475 * locals.var_chi_dn0) * assign93250_e142889) + (assign93250_e142883 * ((-(locals.var_chi_dn0 * 0.3333333333333333)) / (2.0 * assign93250_e142889)))), (((0.7071067811865475 * locals.var_chi_dn2) * assign93250_e142889) + (assign93250_e142883 * ((-(locals.var_chi_dn2 * 0.3333333333333333)) / (2.0 * assign93250_e142889)))), (((0.7071067811865475 * locals.var_chi_dn4) * assign93250_e142889) + (assign93250_e142883 * ((-(locals.var_chi_dn4 * 0.3333333333333333)) / (2.0 * assign93250_e142889)))), (((0.7071067811865475 * locals.var_chi_dn5) * assign93250_e142889) + (assign93250_e142883 * ((-(locals.var_chi_dn5 * 0.3333333333333333)) / (2.0 * assign93250_e142889)))), (((0.7071067811865475 * locals.var_chi_dn6) * assign93250_e142889) + (assign93250_e142883 * ((-(locals.var_chi_dn6 * 0.3333333333333333)) / (2.0 * assign93250_e142889)))), (((0.7071067811865475 * locals.var_chi_dn7) * assign93250_e142889) + (assign93250_e142883 * ((-(locals.var_chi_dn7 * 0.3333333333333333)) / (2.0 * assign93250_e142889)))), (((0.7071067811865475 * locals.var_chi_dn8) * assign93250_e142889) + (assign93250_e142883 * ((-(locals.var_chi_dn8 * 0.3333333333333333)) / (2.0 * assign93250_e142889)))), (((0.7071067811865475 * locals.var_chi_dn9) * assign93250_e142889) + (assign93250_e142883 * ((-(locals.var_chi_dn9 * 0.3333333333333333)) / (2.0 * assign93250_e142889)))), (((0.7071067811865475 * locals.var_chi_dn10) * assign93250_e142889) + (assign93250_e142883 * ((-(locals.var_chi_dn10 * 0.3333333333333333)) / (2.0 * assign93250_e142889)))), (((0.7071067811865475 * locals.var_chi_dn11) * assign93250_e142889) + (assign93250_e142883 * ((-(locals.var_chi_dn11 * 0.3333333333333333)) / (2.0 * assign93250_e142889)))), (((0.7071067811865475 * locals.var_chi_dn14) * assign93250_e142889) + (assign93250_e142883 * ((-(locals.var_chi_dn14 * 0.3333333333333333)) / (2.0 * assign93250_e142889)))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign93250_e142892;
        locals.var_t2_dn0 = assign93250_e142892_d_n0;
        locals.var_t2_dn2 = assign93250_e142892_d_n2;
        locals.var_t2_dn4 = assign93250_e142892_d_n4;
        locals.var_t2_dn5 = assign93250_e142892_d_n5;
        locals.var_t2_dn6 = assign93250_e142892_d_n6;
        locals.var_t2_dn7 = assign93250_e142892_d_n7;
        locals.var_t2_dn8 = assign93250_e142892_d_n8;
        locals.var_t2_dn9 = assign93250_e142892_d_n9;
        locals.var_t2_dn10 = assign93250_e142892_d_n10;
        locals.var_t2_dn11 = assign93250_e142892_d_n11;
        locals.var_t2_dn14 = assign93250_e142892_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign93260_e142903, assign93260_e142903_d_n0, assign93260_e142903_d_n2, assign93260_e142903_d_n4, assign93260_e142903_d_n5, assign93260_e142903_d_n6, assign93260_e142903_d_n7, assign93260_e142903_d_n8, assign93260_e142903_d_n9, assign93260_e142903_d_n10, assign93260_e142903_d_n11, assign93260_e142903_d_n14,) = {
    if (((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) {
        let assign93260_e142901: f64 = (locals.var_cnst0over_func * locals.var_t2);
        (assign93260_e142901, ((locals.var_cnst0over_func_dn0 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn0)), ((locals.var_cnst0over_func_dn2 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn2)), ((locals.var_cnst0over_func_dn4 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn4)), ((locals.var_cnst0over_func_dn5 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn5)), ((locals.var_cnst0over_func_dn6 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn6)), ((locals.var_cnst0over_func_dn7 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn7)), ((locals.var_cnst0over_func_dn8 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn8)), ((locals.var_cnst0over_func_dn9 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn9)), ((locals.var_cnst0over_func_dn10 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn10)), ((locals.var_cnst0over_func_dn11 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn11)), ((locals.var_cnst0over_func_dn14 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn14)),)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn8, locals.var_qbuld_dn9, locals.var_qbuld_dn10, locals.var_qbuld_dn11, locals.var_qbuld_dn14,)
    }
};
        locals.var_qbuld = assign93260_e142903;
        locals.var_qbuld_dn0 = assign93260_e142903_d_n0;
        locals.var_qbuld_dn2 = assign93260_e142903_d_n2;
        locals.var_qbuld_dn4 = assign93260_e142903_d_n4;
        locals.var_qbuld_dn5 = assign93260_e142903_d_n5;
        locals.var_qbuld_dn6 = assign93260_e142903_d_n6;
        locals.var_qbuld_dn7 = assign93260_e142903_d_n7;
        locals.var_qbuld_dn8 = assign93260_e142903_d_n8;
        locals.var_qbuld_dn9 = assign93260_e142903_d_n9;
        locals.var_qbuld_dn10 = assign93260_e142903_d_n10;
        locals.var_qbuld_dn11 = assign93260_e142903_d_n11;
        locals.var_qbuld_dn14 = assign93260_e142903_d_n14;
        locals.var_qbuld_rv = 0.0;

        let (assign93270_e142916, assign93270_e142916_d_n0, assign93270_e142916_d_n2, assign93270_e142916_d_n4, assign93270_e142916_d_n5, assign93270_e142916_d_n6, assign93270_e142916_d_n7, assign93270_e142916_d_n8, assign93270_e142916_d_n9, assign93270_e142916_d_n10, assign93270_e142916_d_n11, assign93270_e142916_d_n14,) = {
    if (((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) {
        let assign93270_e142913: f64 = (locals.var_vgpld - locals.var_ps0ld);
        let assign93270_e142914: f64 = (locals.var_cox0_func * assign93270_e142913);
        (assign93270_e142914, (locals.var_cox0_func * (-locals.var_ps0ld_dn0)), (locals.var_cox0_func * (locals.var_vgpld_dn2 - locals.var_ps0ld_dn2)), (locals.var_cox0_func * (-locals.var_ps0ld_dn4)), (locals.var_cox0_func * (-locals.var_ps0ld_dn5)), (locals.var_cox0_func * (-locals.var_ps0ld_dn6)), (locals.var_cox0_func * (locals.var_vgpld_dn7 - locals.var_ps0ld_dn7)), (locals.var_cox0_func * (locals.var_vgpld_dn8 - locals.var_ps0ld_dn8)), (locals.var_cox0_func * (locals.var_vgpld_dn9 - locals.var_ps0ld_dn9)), (locals.var_cox0_func * (-locals.var_ps0ld_dn10)), (locals.var_cox0_func * (-locals.var_ps0ld_dn11)), (locals.var_cox0_func * (-locals.var_ps0ld_dn14)),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn4, locals.var_qsuld_dn5, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn8, locals.var_qsuld_dn9, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn14,)
    }
};
        locals.var_qsuld = assign93270_e142916;
        locals.var_qsuld_dn0 = assign93270_e142916_d_n0;
        locals.var_qsuld_dn2 = assign93270_e142916_d_n2;
        locals.var_qsuld_dn4 = assign93270_e142916_d_n4;
        locals.var_qsuld_dn5 = assign93270_e142916_d_n5;
        locals.var_qsuld_dn6 = assign93270_e142916_d_n6;
        locals.var_qsuld_dn7 = assign93270_e142916_d_n7;
        locals.var_qsuld_dn8 = assign93270_e142916_d_n8;
        locals.var_qsuld_dn9 = assign93270_e142916_d_n9;
        locals.var_qsuld_dn10 = assign93270_e142916_d_n10;
        locals.var_qsuld_dn11 = assign93270_e142916_d_n11;
        locals.var_qsuld_dn14 = assign93270_e142916_d_n14;
        locals.var_qsuld_rv = 0.0;

        let (assign93280_e142927, assign93280_e142927_d_n0, assign93280_e142927_d_n2, assign93280_e142927_d_n4, assign93280_e142927_d_n5, assign93280_e142927_d_n6, assign93280_e142927_d_n7, assign93280_e142927_d_n8, assign93280_e142927_d_n9, assign93280_e142927_d_n10, assign93280_e142927_d_n11, assign93280_e142927_d_n14,) = {
    if (((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) {
        let assign93280_e142925: f64 = (locals.var_qbuld / locals.var_q_nsubld__blk2119);
        (assign93280_e142925, (locals.var_qbuld_dn0 / locals.var_q_nsubld__blk2119), (locals.var_qbuld_dn2 / locals.var_q_nsubld__blk2119), (locals.var_qbuld_dn4 / locals.var_q_nsubld__blk2119), (locals.var_qbuld_dn5 / locals.var_q_nsubld__blk2119), (locals.var_qbuld_dn6 / locals.var_q_nsubld__blk2119), (locals.var_qbuld_dn7 / locals.var_q_nsubld__blk2119), (locals.var_qbuld_dn8 / locals.var_q_nsubld__blk2119), (locals.var_qbuld_dn9 / locals.var_q_nsubld__blk2119), (locals.var_qbuld_dn10 / locals.var_q_nsubld__blk2119), (locals.var_qbuld_dn11 / locals.var_q_nsubld__blk2119), (locals.var_qbuld_dn14 / locals.var_q_nsubld__blk2119),)
    } else {
        (locals.var_wdld0__blk2163, locals.var_wdld0__blk2163_dn0, locals.var_wdld0__blk2163_dn2, locals.var_wdld0__blk2163_dn4, locals.var_wdld0__blk2163_dn5, locals.var_wdld0__blk2163_dn6, locals.var_wdld0__blk2163_dn7, locals.var_wdld0__blk2163_dn8, locals.var_wdld0__blk2163_dn9, locals.var_wdld0__blk2163_dn10, locals.var_wdld0__blk2163_dn11, locals.var_wdld0__blk2163_dn14,)
    }
};
        locals.var_wdld0__blk2163 = assign93280_e142927;
        locals.var_wdld0__blk2163_dn0 = assign93280_e142927_d_n0;
        locals.var_wdld0__blk2163_dn2 = assign93280_e142927_d_n2;
        locals.var_wdld0__blk2163_dn4 = assign93280_e142927_d_n4;
        locals.var_wdld0__blk2163_dn5 = assign93280_e142927_d_n5;
        locals.var_wdld0__blk2163_dn6 = assign93280_e142927_d_n6;
        locals.var_wdld0__blk2163_dn7 = assign93280_e142927_d_n7;
        locals.var_wdld0__blk2163_dn8 = assign93280_e142927_d_n8;
        locals.var_wdld0__blk2163_dn9 = assign93280_e142927_d_n9;
        locals.var_wdld0__blk2163_dn10 = assign93280_e142927_d_n10;
        locals.var_wdld0__blk2163_dn11 = assign93280_e142927_d_n11;
        locals.var_wdld0__blk2163_dn14 = assign93280_e142927_d_n14;
        locals.var_wdld0__blk2163_rv = 0.0;

        let assign93290_e142930: f64 = if p.p33 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard2165 = assign93290_e142930;
        locals.var_guard2165_rv = 0.0;

        let assign93300_e142935: f64 = (locals.var_ddriftldc * 0.1);
        let assign93300_e142936: f64 = (locals.var_ddriftldc - assign93300_e142935);
        let assign93300_e142940: f64 = (locals.var_ddriftldc * 0.1);
        let assign93300_e142943: f64 = if ((locals.var_wdld0__blk2163 > assign93300_e142936) && (assign93300_e142940 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2166 = assign93300_e142943;
        locals.var_guard2166_rv = 0.0;

        let (assign93310_e142962, assign93310_e142962_d_n0, assign93310_e142962_d_n2, assign93310_e142962_d_n4, assign93310_e142962_d_n5, assign93310_e142962_d_n6, assign93310_e142962_d_n7, assign93310_e142962_d_n8, assign93310_e142962_d_n9, assign93310_e142962_d_n10, assign93310_e142962_d_n11, assign93310_e142962_d_n14,) = {
    if (((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2165 != 0.0)) && (locals.var_guard2166 != 0.0)) {
        let assign93310_e142956: f64 = (locals.var_wdld0__blk2163 - locals.var_ddriftldc);
        let assign93310_e142959: f64 = (locals.var_ddriftldc * 0.1);
        let assign93310_e142960: f64 = (assign93310_e142956 + assign93310_e142959);
        (assign93310_e142960, ((locals.var_wdld0__blk2163_dn0 - locals.var_ddriftldc_dn0) + (locals.var_ddriftldc_dn0 * 0.1)), ((locals.var_wdld0__blk2163_dn2 - locals.var_ddriftldc_dn2) + (locals.var_ddriftldc_dn2 * 0.1)), ((locals.var_wdld0__blk2163_dn4 - locals.var_ddriftldc_dn4) + (locals.var_ddriftldc_dn4 * 0.1)), ((locals.var_wdld0__blk2163_dn5 - locals.var_ddriftldc_dn5) + (locals.var_ddriftldc_dn5 * 0.1)), ((locals.var_wdld0__blk2163_dn6 - locals.var_ddriftldc_dn6) + (locals.var_ddriftldc_dn6 * 0.1)), ((locals.var_wdld0__blk2163_dn7 - locals.var_ddriftldc_dn7) + (locals.var_ddriftldc_dn7 * 0.1)), ((locals.var_wdld0__blk2163_dn8 - locals.var_ddriftldc_dn8) + (locals.var_ddriftldc_dn8 * 0.1)), ((locals.var_wdld0__blk2163_dn9 - locals.var_ddriftldc_dn9) + (locals.var_ddriftldc_dn9 * 0.1)), ((locals.var_wdld0__blk2163_dn10 - locals.var_ddriftldc_dn10) + (locals.var_ddriftldc_dn10 * 0.1)), ((locals.var_wdld0__blk2163_dn11 - locals.var_ddriftldc_dn11) + (locals.var_ddriftldc_dn11 * 0.1)), ((locals.var_wdld0__blk2163_dn14 - locals.var_ddriftldc_dn14) + (locals.var_ddriftldc_dn14 * 0.1)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign93310_e142962;
        locals.var_tmf1_dn0 = assign93310_e142962_d_n0;
        locals.var_tmf1_dn2 = assign93310_e142962_d_n2;
        locals.var_tmf1_dn4 = assign93310_e142962_d_n4;
        locals.var_tmf1_dn5 = assign93310_e142962_d_n5;
        locals.var_tmf1_dn6 = assign93310_e142962_d_n6;
        locals.var_tmf1_dn7 = assign93310_e142962_d_n7;
        locals.var_tmf1_dn8 = assign93310_e142962_d_n8;
        locals.var_tmf1_dn9 = assign93310_e142962_d_n9;
        locals.var_tmf1_dn10 = assign93310_e142962_d_n10;
        locals.var_tmf1_dn11 = assign93310_e142962_d_n11;
        locals.var_tmf1_dn14 = assign93310_e142962_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign93320_e142977, assign93320_e142977_d_n0, assign93320_e142977_d_n2, assign93320_e142977_d_n4, assign93320_e142977_d_n5, assign93320_e142977_d_n6, assign93320_e142977_d_n7, assign93320_e142977_d_n8, assign93320_e142977_d_n9, assign93320_e142977_d_n10, assign93320_e142977_d_n11, assign93320_e142977_d_n14,) = {
    if (((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2165 != 0.0)) && (locals.var_guard2166 != 0.0)) {
        let assign93320_e142975: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign93320_e142975, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign93320_e142977;
        locals.var_x2_dn0 = assign93320_e142977_d_n0;
        locals.var_x2_dn2 = assign93320_e142977_d_n2;
        locals.var_x2_dn4 = assign93320_e142977_d_n4;
        locals.var_x2_dn5 = assign93320_e142977_d_n5;
        locals.var_x2_dn6 = assign93320_e142977_d_n6;
        locals.var_x2_dn7 = assign93320_e142977_d_n7;
        locals.var_x2_dn8 = assign93320_e142977_d_n8;
        locals.var_x2_dn9 = assign93320_e142977_d_n9;
        locals.var_x2_dn10 = assign93320_e142977_d_n10;
        locals.var_x2_dn11 = assign93320_e142977_d_n11;
        locals.var_x2_dn14 = assign93320_e142977_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign93330_e142996, assign93330_e142996_d_n0, assign93330_e142996_d_n2, assign93330_e142996_d_n4, assign93330_e142996_d_n5, assign93330_e142996_d_n6, assign93330_e142996_d_n7, assign93330_e142996_d_n8, assign93330_e142996_d_n9, assign93330_e142996_d_n10, assign93330_e142996_d_n11, assign93330_e142996_d_n14,) = {
    if (((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2165 != 0.0)) && (locals.var_guard2166 != 0.0)) {
        let assign93330_e142990: f64 = (locals.var_ddriftldc * 0.1);
        let assign93330_e142993: f64 = (locals.var_ddriftldc * 0.1);
        let assign93330_e142994: f64 = (assign93330_e142990 * assign93330_e142993);
        (assign93330_e142994, (((locals.var_ddriftldc_dn0 * 0.1) * assign93330_e142993) + (assign93330_e142990 * (locals.var_ddriftldc_dn0 * 0.1))), (((locals.var_ddriftldc_dn2 * 0.1) * assign93330_e142993) + (assign93330_e142990 * (locals.var_ddriftldc_dn2 * 0.1))), (((locals.var_ddriftldc_dn4 * 0.1) * assign93330_e142993) + (assign93330_e142990 * (locals.var_ddriftldc_dn4 * 0.1))), (((locals.var_ddriftldc_dn5 * 0.1) * assign93330_e142993) + (assign93330_e142990 * (locals.var_ddriftldc_dn5 * 0.1))), (((locals.var_ddriftldc_dn6 * 0.1) * assign93330_e142993) + (assign93330_e142990 * (locals.var_ddriftldc_dn6 * 0.1))), (((locals.var_ddriftldc_dn7 * 0.1) * assign93330_e142993) + (assign93330_e142990 * (locals.var_ddriftldc_dn7 * 0.1))), (((locals.var_ddriftldc_dn8 * 0.1) * assign93330_e142993) + (assign93330_e142990 * (locals.var_ddriftldc_dn8 * 0.1))), (((locals.var_ddriftldc_dn9 * 0.1) * assign93330_e142993) + (assign93330_e142990 * (locals.var_ddriftldc_dn9 * 0.1))), (((locals.var_ddriftldc_dn10 * 0.1) * assign93330_e142993) + (assign93330_e142990 * (locals.var_ddriftldc_dn10 * 0.1))), (((locals.var_ddriftldc_dn11 * 0.1) * assign93330_e142993) + (assign93330_e142990 * (locals.var_ddriftldc_dn11 * 0.1))), (((locals.var_ddriftldc_dn14 * 0.1) * assign93330_e142993) + (assign93330_e142990 * (locals.var_ddriftldc_dn14 * 0.1))),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign93330_e142996;
        locals.var_xmax2_dn0 = assign93330_e142996_d_n0;
        locals.var_xmax2_dn2 = assign93330_e142996_d_n2;
        locals.var_xmax2_dn4 = assign93330_e142996_d_n4;
        locals.var_xmax2_dn5 = assign93330_e142996_d_n5;
        locals.var_xmax2_dn6 = assign93330_e142996_d_n6;
        locals.var_xmax2_dn7 = assign93330_e142996_d_n7;
        locals.var_xmax2_dn8 = assign93330_e142996_d_n8;
        locals.var_xmax2_dn9 = assign93330_e142996_d_n9;
        locals.var_xmax2_dn10 = assign93330_e142996_d_n10;
        locals.var_xmax2_dn11 = assign93330_e142996_d_n11;
        locals.var_xmax2_dn14 = assign93330_e142996_d_n14;
        locals.var_xmax2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_360(
        locals: &mut StampLocals,
    ) {
        let (assign93340_e143009, assign93340_e143009_d_n0, assign93340_e143009_d_n2, assign93340_e143009_d_n4, assign93340_e143009_d_n5, assign93340_e143009_d_n6, assign93340_e143009_d_n7, assign93340_e143009_d_n8, assign93340_e143009_d_n9, assign93340_e143009_d_n10, assign93340_e143009_d_n11, assign93340_e143009_d_n14,) = {
    if (((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2165 != 0.0)) && (locals.var_guard2166 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign93340_e143009;
        locals.var_xp_dn0 = assign93340_e143009_d_n0;
        locals.var_xp_dn2 = assign93340_e143009_d_n2;
        locals.var_xp_dn4 = assign93340_e143009_d_n4;
        locals.var_xp_dn5 = assign93340_e143009_d_n5;
        locals.var_xp_dn6 = assign93340_e143009_d_n6;
        locals.var_xp_dn7 = assign93340_e143009_d_n7;
        locals.var_xp_dn8 = assign93340_e143009_d_n8;
        locals.var_xp_dn9 = assign93340_e143009_d_n9;
        locals.var_xp_dn10 = assign93340_e143009_d_n10;
        locals.var_xp_dn11 = assign93340_e143009_d_n11;
        locals.var_xp_dn14 = assign93340_e143009_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign93350_e143022, assign93350_e143022_d_n0, assign93350_e143022_d_n2, assign93350_e143022_d_n4, assign93350_e143022_d_n5, assign93350_e143022_d_n6, assign93350_e143022_d_n7, assign93350_e143022_d_n8, assign93350_e143022_d_n9, assign93350_e143022_d_n10, assign93350_e143022_d_n11, assign93350_e143022_d_n14,) = {
    if (((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2165 != 0.0)) && (locals.var_guard2166 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign93350_e143022;
        locals.var_xmp_dn0 = assign93350_e143022_d_n0;
        locals.var_xmp_dn2 = assign93350_e143022_d_n2;
        locals.var_xmp_dn4 = assign93350_e143022_d_n4;
        locals.var_xmp_dn5 = assign93350_e143022_d_n5;
        locals.var_xmp_dn6 = assign93350_e143022_d_n6;
        locals.var_xmp_dn7 = assign93350_e143022_d_n7;
        locals.var_xmp_dn8 = assign93350_e143022_d_n8;
        locals.var_xmp_dn9 = assign93350_e143022_d_n9;
        locals.var_xmp_dn10 = assign93350_e143022_d_n10;
        locals.var_xmp_dn11 = assign93350_e143022_d_n11;
        locals.var_xmp_dn14 = assign93350_e143022_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign93360_e143035,) = {
    if (((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2165 != 0.0)) && (locals.var_guard2166 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign93360_e143035;
        locals.var_m0_rv = 0.0;

        let (assign93370_e143048,) = {
    if (((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2165 != 0.0)) && (locals.var_guard2166 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign93370_e143048;
        locals.var_mm_rv = 0.0;

        let (assign93380_e143061, assign93380_e143061_d_n0, assign93380_e143061_d_n2, assign93380_e143061_d_n4, assign93380_e143061_d_n5, assign93380_e143061_d_n6, assign93380_e143061_d_n7, assign93380_e143061_d_n8, assign93380_e143061_d_n9, assign93380_e143061_d_n10, assign93380_e143061_d_n11, assign93380_e143061_d_n14,) = {
    if (((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2165 != 0.0)) && (locals.var_guard2166 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign93380_e143061;
        locals.var_arg_dn0 = assign93380_e143061_d_n0;
        locals.var_arg_dn2 = assign93380_e143061_d_n2;
        locals.var_arg_dn4 = assign93380_e143061_d_n4;
        locals.var_arg_dn5 = assign93380_e143061_d_n5;
        locals.var_arg_dn6 = assign93380_e143061_d_n6;
        locals.var_arg_dn7 = assign93380_e143061_d_n7;
        locals.var_arg_dn8 = assign93380_e143061_d_n8;
        locals.var_arg_dn9 = assign93380_e143061_d_n9;
        locals.var_arg_dn10 = assign93380_e143061_d_n10;
        locals.var_arg_dn11 = assign93380_e143061_d_n11;
        locals.var_arg_dn14 = assign93380_e143061_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign93390_e143074, assign93390_e143074_d_n0, assign93390_e143074_d_n2, assign93390_e143074_d_n4, assign93390_e143074_d_n5, assign93390_e143074_d_n6, assign93390_e143074_d_n7, assign93390_e143074_d_n8, assign93390_e143074_d_n9, assign93390_e143074_d_n10, assign93390_e143074_d_n11, assign93390_e143074_d_n14,) = {
    if (((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2165 != 0.0)) && (locals.var_guard2166 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign93390_e143074;
        locals.var_dnm_dn0 = assign93390_e143074_d_n0;
        locals.var_dnm_dn2 = assign93390_e143074_d_n2;
        locals.var_dnm_dn4 = assign93390_e143074_d_n4;
        locals.var_dnm_dn5 = assign93390_e143074_d_n5;
        locals.var_dnm_dn6 = assign93390_e143074_d_n6;
        locals.var_dnm_dn7 = assign93390_e143074_d_n7;
        locals.var_dnm_dn8 = assign93390_e143074_d_n8;
        locals.var_dnm_dn9 = assign93390_e143074_d_n9;
        locals.var_dnm_dn10 = assign93390_e143074_d_n10;
        locals.var_dnm_dn11 = assign93390_e143074_d_n11;
        locals.var_dnm_dn14 = assign93390_e143074_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign93400_e143089, assign93400_e143089_d_n0, assign93400_e143089_d_n2, assign93400_e143089_d_n4, assign93400_e143089_d_n5, assign93400_e143089_d_n6, assign93400_e143089_d_n7, assign93400_e143089_d_n8, assign93400_e143089_d_n9, assign93400_e143089_d_n10, assign93400_e143089_d_n11, assign93400_e143089_d_n14,) = {
    if (((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2165 != 0.0)) && (locals.var_guard2166 != 0.0)) {
        let assign93400_e143087: f64 = (locals.var_xp * locals.var_x2);
        (assign93400_e143087, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign93400_e143089;
        locals.var_xp_dn0 = assign93400_e143089_d_n0;
        locals.var_xp_dn2 = assign93400_e143089_d_n2;
        locals.var_xp_dn4 = assign93400_e143089_d_n4;
        locals.var_xp_dn5 = assign93400_e143089_d_n5;
        locals.var_xp_dn6 = assign93400_e143089_d_n6;
        locals.var_xp_dn7 = assign93400_e143089_d_n7;
        locals.var_xp_dn8 = assign93400_e143089_d_n8;
        locals.var_xp_dn9 = assign93400_e143089_d_n9;
        locals.var_xp_dn10 = assign93400_e143089_d_n10;
        locals.var_xp_dn11 = assign93400_e143089_d_n11;
        locals.var_xp_dn14 = assign93400_e143089_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign93410_e143104, assign93410_e143104_d_n0, assign93410_e143104_d_n2, assign93410_e143104_d_n4, assign93410_e143104_d_n5, assign93410_e143104_d_n6, assign93410_e143104_d_n7, assign93410_e143104_d_n8, assign93410_e143104_d_n9, assign93410_e143104_d_n10, assign93410_e143104_d_n11, assign93410_e143104_d_n14,) = {
    if (((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2165 != 0.0)) && (locals.var_guard2166 != 0.0)) {
        let assign93410_e143102: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign93410_e143102, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign93410_e143104;
        locals.var_xmp_dn0 = assign93410_e143104_d_n0;
        locals.var_xmp_dn2 = assign93410_e143104_d_n2;
        locals.var_xmp_dn4 = assign93410_e143104_d_n4;
        locals.var_xmp_dn5 = assign93410_e143104_d_n5;
        locals.var_xmp_dn6 = assign93410_e143104_d_n6;
        locals.var_xmp_dn7 = assign93410_e143104_d_n7;
        locals.var_xmp_dn8 = assign93410_e143104_d_n8;
        locals.var_xmp_dn9 = assign93410_e143104_d_n9;
        locals.var_xmp_dn10 = assign93410_e143104_d_n10;
        locals.var_xmp_dn11 = assign93410_e143104_d_n11;
        locals.var_xmp_dn14 = assign93410_e143104_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign93420_e143119, assign93420_e143119_d_n0, assign93420_e143119_d_n2, assign93420_e143119_d_n4, assign93420_e143119_d_n5, assign93420_e143119_d_n6, assign93420_e143119_d_n7, assign93420_e143119_d_n8, assign93420_e143119_d_n9, assign93420_e143119_d_n10, assign93420_e143119_d_n11, assign93420_e143119_d_n14,) = {
    if (((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2165 != 0.0)) && (locals.var_guard2166 != 0.0)) {
        let assign93420_e143117: f64 = (locals.var_xp * locals.var_x2);
        (assign93420_e143117, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign93420_e143119;
        locals.var_xp_dn0 = assign93420_e143119_d_n0;
        locals.var_xp_dn2 = assign93420_e143119_d_n2;
        locals.var_xp_dn4 = assign93420_e143119_d_n4;
        locals.var_xp_dn5 = assign93420_e143119_d_n5;
        locals.var_xp_dn6 = assign93420_e143119_d_n6;
        locals.var_xp_dn7 = assign93420_e143119_d_n7;
        locals.var_xp_dn8 = assign93420_e143119_d_n8;
        locals.var_xp_dn9 = assign93420_e143119_d_n9;
        locals.var_xp_dn10 = assign93420_e143119_d_n10;
        locals.var_xp_dn11 = assign93420_e143119_d_n11;
        locals.var_xp_dn14 = assign93420_e143119_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign93430_e143134, assign93430_e143134_d_n0, assign93430_e143134_d_n2, assign93430_e143134_d_n4, assign93430_e143134_d_n5, assign93430_e143134_d_n6, assign93430_e143134_d_n7, assign93430_e143134_d_n8, assign93430_e143134_d_n9, assign93430_e143134_d_n10, assign93430_e143134_d_n11, assign93430_e143134_d_n14,) = {
    if (((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2165 != 0.0)) && (locals.var_guard2166 != 0.0)) {
        let assign93430_e143132: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign93430_e143132, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign93430_e143134;
        locals.var_xmp_dn0 = assign93430_e143134_d_n0;
        locals.var_xmp_dn2 = assign93430_e143134_d_n2;
        locals.var_xmp_dn4 = assign93430_e143134_d_n4;
        locals.var_xmp_dn5 = assign93430_e143134_d_n5;
        locals.var_xmp_dn6 = assign93430_e143134_d_n6;
        locals.var_xmp_dn7 = assign93430_e143134_d_n7;
        locals.var_xmp_dn8 = assign93430_e143134_d_n8;
        locals.var_xmp_dn9 = assign93430_e143134_d_n9;
        locals.var_xmp_dn10 = assign93430_e143134_d_n10;
        locals.var_xmp_dn11 = assign93430_e143134_d_n11;
        locals.var_xmp_dn14 = assign93430_e143134_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign93440_e143149, assign93440_e143149_d_n0, assign93440_e143149_d_n2, assign93440_e143149_d_n4, assign93440_e143149_d_n5, assign93440_e143149_d_n6, assign93440_e143149_d_n7, assign93440_e143149_d_n8, assign93440_e143149_d_n9, assign93440_e143149_d_n10, assign93440_e143149_d_n11, assign93440_e143149_d_n14,) = {
    if (((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2165 != 0.0)) && (locals.var_guard2166 != 0.0)) {
        let assign93440_e143147: f64 = (locals.var_xp + locals.var_xmp);
        (assign93440_e143147, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign93440_e143149;
        locals.var_arg_dn0 = assign93440_e143149_d_n0;
        locals.var_arg_dn2 = assign93440_e143149_d_n2;
        locals.var_arg_dn4 = assign93440_e143149_d_n4;
        locals.var_arg_dn5 = assign93440_e143149_d_n5;
        locals.var_arg_dn6 = assign93440_e143149_d_n6;
        locals.var_arg_dn7 = assign93440_e143149_d_n7;
        locals.var_arg_dn8 = assign93440_e143149_d_n8;
        locals.var_arg_dn9 = assign93440_e143149_d_n9;
        locals.var_arg_dn10 = assign93440_e143149_d_n10;
        locals.var_arg_dn11 = assign93440_e143149_d_n11;
        locals.var_arg_dn14 = assign93440_e143149_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign93450_e143162, assign93450_e143162_d_n0, assign93450_e143162_d_n2, assign93450_e143162_d_n4, assign93450_e143162_d_n5, assign93450_e143162_d_n6, assign93450_e143162_d_n7, assign93450_e143162_d_n8, assign93450_e143162_d_n9, assign93450_e143162_d_n10, assign93450_e143162_d_n11, assign93450_e143162_d_n14,) = {
    if (((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2165 != 0.0)) && (locals.var_guard2166 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign93450_e143162;
        locals.var_dnm_dn0 = assign93450_e143162_d_n0;
        locals.var_dnm_dn2 = assign93450_e143162_d_n2;
        locals.var_dnm_dn4 = assign93450_e143162_d_n4;
        locals.var_dnm_dn5 = assign93450_e143162_d_n5;
        locals.var_dnm_dn6 = assign93450_e143162_d_n6;
        locals.var_dnm_dn7 = assign93450_e143162_d_n7;
        locals.var_dnm_dn8 = assign93450_e143162_d_n8;
        locals.var_dnm_dn9 = assign93450_e143162_d_n9;
        locals.var_dnm_dn10 = assign93450_e143162_d_n10;
        locals.var_dnm_dn11 = assign93450_e143162_d_n11;
        locals.var_dnm_dn14 = assign93450_e143162_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign93460_e143177: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard2167 = assign93460_e143177;
        locals.var_guard2167_rv = 0.0;

        let assign93470_e143180: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2168 = assign93470_e143180;
        locals.var_guard2168_rv = 0.0;

        let (assign93480_e143197,) = {
    if (((((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2165 != 0.0)) && (locals.var_guard2166 != 0.0)) && (locals.var_guard2167 != 0.0)) && (locals.var_guard2168 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign93480_e143197;
        locals.var_mm_rv = 0.0;

        let assign93490_e143200: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard2169 = assign93490_e143200;
        locals.var_guard2169_rv = 0.0;

        let (assign93500_e143220,) = {
    if ((((((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2165 != 0.0)) && (locals.var_guard2166 != 0.0)) && (locals.var_guard2167 != 0.0)) && (locals.var_guard2168 == 0.0)) && (locals.var_guard2169 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign93500_e143220;
        locals.var_mm_rv = 0.0;

        let assign93510_e143223: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard2170 = assign93510_e143223;
        locals.var_guard2170_rv = 0.0;

        let (assign93520_e143246,) = {
    if (((((((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2165 != 0.0)) && (locals.var_guard2166 != 0.0)) && (locals.var_guard2167 != 0.0)) && (locals.var_guard2168 == 0.0)) && (locals.var_guard2169 == 0.0)) && (locals.var_guard2170 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign93520_e143246;
        locals.var_mm_rv = 0.0;

        let assign93530_e143249: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard2171 = assign93530_e143249;
        locals.var_guard2171_rv = 0.0;

        let (assign93540_e143275,) = {
    if ((((((((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2165 != 0.0)) && (locals.var_guard2166 != 0.0)) && (locals.var_guard2167 != 0.0)) && (locals.var_guard2168 == 0.0)) && (locals.var_guard2169 == 0.0)) && (locals.var_guard2170 == 0.0)) && (locals.var_guard2171 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign93540_e143275;
        locals.var_mm_rv = 0.0;

        let (assign93550_e143290,) = {
    if ((((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2165 != 0.0)) && (locals.var_guard2166 != 0.0)) && (locals.var_guard2167 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign93550_e143290;
        locals.var_m0_rv = 0.0;

        let mut assign93560_loop_guard: usize = 0;
        while {
            let assign93560_cond_e143306: f64 = if (((((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2165 != 0.0)) && (locals.var_guard2166 != 0.0)) && (locals.var_guard2167 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign93560_cond_e143306 != 0.0
        } {
            assign93560_loop_guard += 1;
            assert!(assign93560_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign93560_body0_e143322, assign93560_body0_e143322_d_n0, assign93560_body0_e143322_d_n2, assign93560_body0_e143322_d_n4, assign93560_body0_e143322_d_n5, assign93560_body0_e143322_d_n6, assign93560_body0_e143322_d_n7, assign93560_body0_e143322_d_n8, assign93560_body0_e143322_d_n9, assign93560_body0_e143322_d_n10, assign93560_body0_e143322_d_n11, assign93560_body0_e143322_d_n14,) = {
    if ((((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2165 != 0.0)) && (locals.var_guard2166 != 0.0)) && (locals.var_guard2167 != 0.0)) {
        let assign93560_body0_e143320: f64 = (locals.var_dnm).sqrt();
        (assign93560_body0_e143320, (locals.var_dnm_dn0 / (2.0 * assign93560_body0_e143320)), (locals.var_dnm_dn2 / (2.0 * assign93560_body0_e143320)), (locals.var_dnm_dn4 / (2.0 * assign93560_body0_e143320)), (locals.var_dnm_dn5 / (2.0 * assign93560_body0_e143320)), (locals.var_dnm_dn6 / (2.0 * assign93560_body0_e143320)), (locals.var_dnm_dn7 / (2.0 * assign93560_body0_e143320)), (locals.var_dnm_dn8 / (2.0 * assign93560_body0_e143320)), (locals.var_dnm_dn9 / (2.0 * assign93560_body0_e143320)), (locals.var_dnm_dn10 / (2.0 * assign93560_body0_e143320)), (locals.var_dnm_dn11 / (2.0 * assign93560_body0_e143320)), (locals.var_dnm_dn14 / (2.0 * assign93560_body0_e143320)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign93560_body0_e143322;
            locals.var_dnm_dn0 = assign93560_body0_e143322_d_n0;
            locals.var_dnm_dn2 = assign93560_body0_e143322_d_n2;
            locals.var_dnm_dn4 = assign93560_body0_e143322_d_n4;
            locals.var_dnm_dn5 = assign93560_body0_e143322_d_n5;
            locals.var_dnm_dn6 = assign93560_body0_e143322_d_n6;
            locals.var_dnm_dn7 = assign93560_body0_e143322_d_n7;
            locals.var_dnm_dn8 = assign93560_body0_e143322_d_n8;
            locals.var_dnm_dn9 = assign93560_body0_e143322_d_n9;
            locals.var_dnm_dn10 = assign93560_body0_e143322_d_n10;
            locals.var_dnm_dn11 = assign93560_body0_e143322_d_n11;
            locals.var_dnm_dn14 = assign93560_body0_e143322_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign93560_body1_e143339,) = {
    if ((((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2165 != 0.0)) && (locals.var_guard2166 != 0.0)) && (locals.var_guard2167 != 0.0)) {
        let assign93560_body1_e143337: f64 = (locals.var_m0 + 1.0);
        (assign93560_body1_e143337,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign93560_body1_e143339;
            locals.var_m0_rv = 0.0;
        }

        let (assign93570_e143366, assign93570_e143366_d_n0, assign93570_e143366_d_n2, assign93570_e143366_d_n4, assign93570_e143366_d_n5, assign93570_e143366_d_n6, assign93570_e143366_d_n7, assign93570_e143366_d_n8, assign93570_e143366_d_n9, assign93570_e143366_d_n10, assign93570_e143366_d_n11, assign93570_e143366_d_n14,) = {
    if ((((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2165 != 0.0)) && (locals.var_guard2166 != 0.0)) && (locals.var_guard2167 == 0.0)) {
        let (assign93570_e143364, assign93570_e143364_d_n0, assign93570_e143364_d_n2, assign93570_e143364_d_n4, assign93570_e143364_d_n5, assign93570_e143364_d_n6, assign93570_e143364_d_n7, assign93570_e143364_d_n8, assign93570_e143364_d_n9, assign93570_e143364_d_n10, assign93570_e143364_d_n11, assign93570_e143364_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign93570_e143361: f64 = (2.0 * 2.0);
                let assign93570_e143362: f64 = (1.0 / assign93570_e143361);
                let assign93570_e143363: f64 = (locals.var_dnm).powf(assign93570_e143362);
                (assign93570_e143363, if 0.0 == 0.0 && ((assign93570_e143362) as f64).is_finite() && ((assign93570_e143362) as f64).fract() == 0.0 { if assign93570_e143362 == 0.0 { 0.0 } else { (assign93570_e143362 * ((locals.var_dnm).powf(assign93570_e143362 - 1.0) * locals.var_dnm_dn0)) } } else { (assign93570_e143363 * (assign93570_e143362 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign93570_e143362) as f64).is_finite() && ((assign93570_e143362) as f64).fract() == 0.0 { if assign93570_e143362 == 0.0 { 0.0 } else { (assign93570_e143362 * ((locals.var_dnm).powf(assign93570_e143362 - 1.0) * locals.var_dnm_dn2)) } } else { (assign93570_e143363 * (assign93570_e143362 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign93570_e143362) as f64).is_finite() && ((assign93570_e143362) as f64).fract() == 0.0 { if assign93570_e143362 == 0.0 { 0.0 } else { (assign93570_e143362 * ((locals.var_dnm).powf(assign93570_e143362 - 1.0) * locals.var_dnm_dn4)) } } else { (assign93570_e143363 * (assign93570_e143362 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign93570_e143362) as f64).is_finite() && ((assign93570_e143362) as f64).fract() == 0.0 { if assign93570_e143362 == 0.0 { 0.0 } else { (assign93570_e143362 * ((locals.var_dnm).powf(assign93570_e143362 - 1.0) * locals.var_dnm_dn5)) } } else { (assign93570_e143363 * (assign93570_e143362 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign93570_e143362) as f64).is_finite() && ((assign93570_e143362) as f64).fract() == 0.0 { if assign93570_e143362 == 0.0 { 0.0 } else { (assign93570_e143362 * ((locals.var_dnm).powf(assign93570_e143362 - 1.0) * locals.var_dnm_dn6)) } } else { (assign93570_e143363 * (assign93570_e143362 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign93570_e143362) as f64).is_finite() && ((assign93570_e143362) as f64).fract() == 0.0 { if assign93570_e143362 == 0.0 { 0.0 } else { (assign93570_e143362 * ((locals.var_dnm).powf(assign93570_e143362 - 1.0) * locals.var_dnm_dn7)) } } else { (assign93570_e143363 * (assign93570_e143362 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign93570_e143362) as f64).is_finite() && ((assign93570_e143362) as f64).fract() == 0.0 { if assign93570_e143362 == 0.0 { 0.0 } else { (assign93570_e143362 * ((locals.var_dnm).powf(assign93570_e143362 - 1.0) * locals.var_dnm_dn8)) } } else { (assign93570_e143363 * (assign93570_e143362 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign93570_e143362) as f64).is_finite() && ((assign93570_e143362) as f64).fract() == 0.0 { if assign93570_e143362 == 0.0 { 0.0 } else { (assign93570_e143362 * ((locals.var_dnm).powf(assign93570_e143362 - 1.0) * locals.var_dnm_dn9)) } } else { (assign93570_e143363 * (assign93570_e143362 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign93570_e143362) as f64).is_finite() && ((assign93570_e143362) as f64).fract() == 0.0 { if assign93570_e143362 == 0.0 { 0.0 } else { (assign93570_e143362 * ((locals.var_dnm).powf(assign93570_e143362 - 1.0) * locals.var_dnm_dn10)) } } else { (assign93570_e143363 * (assign93570_e143362 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign93570_e143362) as f64).is_finite() && ((assign93570_e143362) as f64).fract() == 0.0 { if assign93570_e143362 == 0.0 { 0.0 } else { (assign93570_e143362 * ((locals.var_dnm).powf(assign93570_e143362 - 1.0) * locals.var_dnm_dn11)) } } else { (assign93570_e143363 * (assign93570_e143362 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign93570_e143362) as f64).is_finite() && ((assign93570_e143362) as f64).fract() == 0.0 { if assign93570_e143362 == 0.0 { 0.0 } else { (assign93570_e143362 * ((locals.var_dnm).powf(assign93570_e143362 - 1.0) * locals.var_dnm_dn14)) } } else { (assign93570_e143363 * (assign93570_e143362 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign93570_e143364, assign93570_e143364_d_n0, assign93570_e143364_d_n2, assign93570_e143364_d_n4, assign93570_e143364_d_n5, assign93570_e143364_d_n6, assign93570_e143364_d_n7, assign93570_e143364_d_n8, assign93570_e143364_d_n9, assign93570_e143364_d_n10, assign93570_e143364_d_n11, assign93570_e143364_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign93570_e143366;
        locals.var_dnm_dn0 = assign93570_e143366_d_n0;
        locals.var_dnm_dn2 = assign93570_e143366_d_n2;
        locals.var_dnm_dn4 = assign93570_e143366_d_n4;
        locals.var_dnm_dn5 = assign93570_e143366_d_n5;
        locals.var_dnm_dn6 = assign93570_e143366_d_n6;
        locals.var_dnm_dn7 = assign93570_e143366_d_n7;
        locals.var_dnm_dn8 = assign93570_e143366_d_n8;
        locals.var_dnm_dn9 = assign93570_e143366_d_n9;
        locals.var_dnm_dn10 = assign93570_e143366_d_n10;
        locals.var_dnm_dn11 = assign93570_e143366_d_n11;
        locals.var_dnm_dn14 = assign93570_e143366_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign93580_e143381, assign93580_e143381_d_n0, assign93580_e143381_d_n2, assign93580_e143381_d_n4, assign93580_e143381_d_n5, assign93580_e143381_d_n6, assign93580_e143381_d_n7, assign93580_e143381_d_n8, assign93580_e143381_d_n9, assign93580_e143381_d_n10, assign93580_e143381_d_n11, assign93580_e143381_d_n14,) = {
    if (((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2165 != 0.0)) && (locals.var_guard2166 != 0.0)) {
        let assign93580_e143379: f64 = (1.0 / locals.var_dnm);
        (assign93580_e143379, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign93580_e143381;
        locals.var_dnm_dn0 = assign93580_e143381_d_n0;
        locals.var_dnm_dn2 = assign93580_e143381_d_n2;
        locals.var_dnm_dn4 = assign93580_e143381_d_n4;
        locals.var_dnm_dn5 = assign93580_e143381_d_n5;
        locals.var_dnm_dn6 = assign93580_e143381_d_n6;
        locals.var_dnm_dn7 = assign93580_e143381_d_n7;
        locals.var_dnm_dn8 = assign93580_e143381_d_n8;
        locals.var_dnm_dn9 = assign93580_e143381_d_n9;
        locals.var_dnm_dn10 = assign93580_e143381_d_n10;
        locals.var_dnm_dn11 = assign93580_e143381_d_n11;
        locals.var_dnm_dn14 = assign93580_e143381_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign93590_e143400, assign93590_e143400_d_n0, assign93590_e143400_d_n2, assign93590_e143400_d_n4, assign93590_e143400_d_n5, assign93590_e143400_d_n6, assign93590_e143400_d_n7, assign93590_e143400_d_n8, assign93590_e143400_d_n9, assign93590_e143400_d_n10, assign93590_e143400_d_n11, assign93590_e143400_d_n14,) = {
    if (((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2165 != 0.0)) && (locals.var_guard2166 != 0.0)) {
        let assign93590_e143395: f64 = (locals.var_ddriftldc * 0.1);
        let assign93590_e143396: f64 = (locals.var_tmf1 * assign93590_e143395);
        let assign93590_e143398: f64 = (assign93590_e143396 * locals.var_dnm);
        (assign93590_e143398, ((((locals.var_tmf1_dn0 * assign93590_e143395) + (locals.var_tmf1 * (locals.var_ddriftldc_dn0 * 0.1))) * locals.var_dnm) + (assign93590_e143396 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * assign93590_e143395) + (locals.var_tmf1 * (locals.var_ddriftldc_dn2 * 0.1))) * locals.var_dnm) + (assign93590_e143396 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn4 * assign93590_e143395) + (locals.var_tmf1 * (locals.var_ddriftldc_dn4 * 0.1))) * locals.var_dnm) + (assign93590_e143396 * locals.var_dnm_dn4)), ((((locals.var_tmf1_dn5 * assign93590_e143395) + (locals.var_tmf1 * (locals.var_ddriftldc_dn5 * 0.1))) * locals.var_dnm) + (assign93590_e143396 * locals.var_dnm_dn5)), ((((locals.var_tmf1_dn6 * assign93590_e143395) + (locals.var_tmf1 * (locals.var_ddriftldc_dn6 * 0.1))) * locals.var_dnm) + (assign93590_e143396 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * assign93590_e143395) + (locals.var_tmf1 * (locals.var_ddriftldc_dn7 * 0.1))) * locals.var_dnm) + (assign93590_e143396 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn8 * assign93590_e143395) + (locals.var_tmf1 * (locals.var_ddriftldc_dn8 * 0.1))) * locals.var_dnm) + (assign93590_e143396 * locals.var_dnm_dn8)), ((((locals.var_tmf1_dn9 * assign93590_e143395) + (locals.var_tmf1 * (locals.var_ddriftldc_dn9 * 0.1))) * locals.var_dnm) + (assign93590_e143396 * locals.var_dnm_dn9)), ((((locals.var_tmf1_dn10 * assign93590_e143395) + (locals.var_tmf1 * (locals.var_ddriftldc_dn10 * 0.1))) * locals.var_dnm) + (assign93590_e143396 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn11 * assign93590_e143395) + (locals.var_tmf1 * (locals.var_ddriftldc_dn11 * 0.1))) * locals.var_dnm) + (assign93590_e143396 * locals.var_dnm_dn11)), ((((locals.var_tmf1_dn14 * assign93590_e143395) + (locals.var_tmf1 * (locals.var_ddriftldc_dn14 * 0.1))) * locals.var_dnm) + (assign93590_e143396 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign93590_e143400;
        locals.var_tmf0_dn0 = assign93590_e143400_d_n0;
        locals.var_tmf0_dn2 = assign93590_e143400_d_n2;
        locals.var_tmf0_dn4 = assign93590_e143400_d_n4;
        locals.var_tmf0_dn5 = assign93590_e143400_d_n5;
        locals.var_tmf0_dn6 = assign93590_e143400_d_n6;
        locals.var_tmf0_dn7 = assign93590_e143400_d_n7;
        locals.var_tmf0_dn8 = assign93590_e143400_d_n8;
        locals.var_tmf0_dn9 = assign93590_e143400_d_n9;
        locals.var_tmf0_dn10 = assign93590_e143400_d_n10;
        locals.var_tmf0_dn11 = assign93590_e143400_d_n11;
        locals.var_tmf0_dn14 = assign93590_e143400_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign93600_e143421, assign93600_e143421_d_n0, assign93600_e143421_d_n2, assign93600_e143421_d_n4, assign93600_e143421_d_n5, assign93600_e143421_d_n6, assign93600_e143421_d_n7, assign93600_e143421_d_n8, assign93600_e143421_d_n9, assign93600_e143421_d_n10, assign93600_e143421_d_n11, assign93600_e143421_d_n14,) = {
    if (((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2165 != 0.0)) && (locals.var_guard2166 != 0.0)) {
        let assign93600_e143413: f64 = (locals.var_ddriftldc * 0.1);
        let assign93600_e143415: f64 = (assign93600_e143413 * locals.var_xmp);
        let assign93600_e143417: f64 = (assign93600_e143415 * locals.var_dnm);
        let assign93600_e143419: f64 = (assign93600_e143417 / locals.var_arg);
        (assign93600_e143419, ((((((((locals.var_ddriftldc_dn0 * 0.1) * locals.var_xmp) + (assign93600_e143413 * locals.var_xmp_dn0)) * locals.var_dnm) + (assign93600_e143415 * locals.var_dnm_dn0)) * locals.var_arg) - (assign93600_e143417 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn2 * 0.1) * locals.var_xmp) + (assign93600_e143413 * locals.var_xmp_dn2)) * locals.var_dnm) + (assign93600_e143415 * locals.var_dnm_dn2)) * locals.var_arg) - (assign93600_e143417 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn4 * 0.1) * locals.var_xmp) + (assign93600_e143413 * locals.var_xmp_dn4)) * locals.var_dnm) + (assign93600_e143415 * locals.var_dnm_dn4)) * locals.var_arg) - (assign93600_e143417 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn5 * 0.1) * locals.var_xmp) + (assign93600_e143413 * locals.var_xmp_dn5)) * locals.var_dnm) + (assign93600_e143415 * locals.var_dnm_dn5)) * locals.var_arg) - (assign93600_e143417 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn6 * 0.1) * locals.var_xmp) + (assign93600_e143413 * locals.var_xmp_dn6)) * locals.var_dnm) + (assign93600_e143415 * locals.var_dnm_dn6)) * locals.var_arg) - (assign93600_e143417 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn7 * 0.1) * locals.var_xmp) + (assign93600_e143413 * locals.var_xmp_dn7)) * locals.var_dnm) + (assign93600_e143415 * locals.var_dnm_dn7)) * locals.var_arg) - (assign93600_e143417 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn8 * 0.1) * locals.var_xmp) + (assign93600_e143413 * locals.var_xmp_dn8)) * locals.var_dnm) + (assign93600_e143415 * locals.var_dnm_dn8)) * locals.var_arg) - (assign93600_e143417 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn9 * 0.1) * locals.var_xmp) + (assign93600_e143413 * locals.var_xmp_dn9)) * locals.var_dnm) + (assign93600_e143415 * locals.var_dnm_dn9)) * locals.var_arg) - (assign93600_e143417 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn10 * 0.1) * locals.var_xmp) + (assign93600_e143413 * locals.var_xmp_dn10)) * locals.var_dnm) + (assign93600_e143415 * locals.var_dnm_dn10)) * locals.var_arg) - (assign93600_e143417 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn11 * 0.1) * locals.var_xmp) + (assign93600_e143413 * locals.var_xmp_dn11)) * locals.var_dnm) + (assign93600_e143415 * locals.var_dnm_dn11)) * locals.var_arg) - (assign93600_e143417 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn14 * 0.1) * locals.var_xmp) + (assign93600_e143413 * locals.var_xmp_dn14)) * locals.var_dnm) + (assign93600_e143415 * locals.var_dnm_dn14)) * locals.var_arg) - (assign93600_e143417 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign93600_e143421;
        locals.var_t0_dn0 = assign93600_e143421_d_n0;
        locals.var_t0_dn2 = assign93600_e143421_d_n2;
        locals.var_t0_dn4 = assign93600_e143421_d_n4;
        locals.var_t0_dn5 = assign93600_e143421_d_n5;
        locals.var_t0_dn6 = assign93600_e143421_d_n6;
        locals.var_t0_dn7 = assign93600_e143421_d_n7;
        locals.var_t0_dn8 = assign93600_e143421_d_n8;
        locals.var_t0_dn9 = assign93600_e143421_d_n9;
        locals.var_t0_dn10 = assign93600_e143421_d_n10;
        locals.var_t0_dn11 = assign93600_e143421_d_n11;
        locals.var_t0_dn14 = assign93600_e143421_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign93610_e143440, assign93610_e143440_d_n0, assign93610_e143440_d_n2, assign93610_e143440_d_n4, assign93610_e143440_d_n5, assign93610_e143440_d_n6, assign93610_e143440_d_n7, assign93610_e143440_d_n8, assign93610_e143440_d_n9, assign93610_e143440_d_n10, assign93610_e143440_d_n11, assign93610_e143440_d_n14,) = {
    if (((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2165 != 0.0)) && (locals.var_guard2166 != 0.0)) {
        let assign93610_e143435: f64 = (locals.var_ddriftldc * 0.1);
        let assign93610_e143436: f64 = (locals.var_ddriftldc - assign93610_e143435);
        let assign93610_e143438: f64 = (assign93610_e143436 + locals.var_tmf0);
        (assign93610_e143438, ((locals.var_ddriftldc_dn0 - (locals.var_ddriftldc_dn0 * 0.1)) + locals.var_tmf0_dn0), ((locals.var_ddriftldc_dn2 - (locals.var_ddriftldc_dn2 * 0.1)) + locals.var_tmf0_dn2), ((locals.var_ddriftldc_dn4 - (locals.var_ddriftldc_dn4 * 0.1)) + locals.var_tmf0_dn4), ((locals.var_ddriftldc_dn5 - (locals.var_ddriftldc_dn5 * 0.1)) + locals.var_tmf0_dn5), ((locals.var_ddriftldc_dn6 - (locals.var_ddriftldc_dn6 * 0.1)) + locals.var_tmf0_dn6), ((locals.var_ddriftldc_dn7 - (locals.var_ddriftldc_dn7 * 0.1)) + locals.var_tmf0_dn7), ((locals.var_ddriftldc_dn8 - (locals.var_ddriftldc_dn8 * 0.1)) + locals.var_tmf0_dn8), ((locals.var_ddriftldc_dn9 - (locals.var_ddriftldc_dn9 * 0.1)) + locals.var_tmf0_dn9), ((locals.var_ddriftldc_dn10 - (locals.var_ddriftldc_dn10 * 0.1)) + locals.var_tmf0_dn10), ((locals.var_ddriftldc_dn11 - (locals.var_ddriftldc_dn11 * 0.1)) + locals.var_tmf0_dn11), ((locals.var_ddriftldc_dn14 - (locals.var_ddriftldc_dn14 * 0.1)) + locals.var_tmf0_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign93610_e143440;
        locals.var_t1_dn0 = assign93610_e143440_d_n0;
        locals.var_t1_dn2 = assign93610_e143440_d_n2;
        locals.var_t1_dn4 = assign93610_e143440_d_n4;
        locals.var_t1_dn5 = assign93610_e143440_d_n5;
        locals.var_t1_dn6 = assign93610_e143440_d_n6;
        locals.var_t1_dn7 = assign93610_e143440_d_n7;
        locals.var_t1_dn8 = assign93610_e143440_d_n8;
        locals.var_t1_dn9 = assign93610_e143440_d_n9;
        locals.var_t1_dn10 = assign93610_e143440_d_n10;
        locals.var_t1_dn11 = assign93610_e143440_d_n11;
        locals.var_t1_dn14 = assign93610_e143440_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign93620_e143453, assign93620_e143453_d_n0, assign93620_e143453_d_n2, assign93620_e143453_d_n4, assign93620_e143453_d_n5, assign93620_e143453_d_n6, assign93620_e143453_d_n7, assign93620_e143453_d_n8, assign93620_e143453_d_n9, assign93620_e143453_d_n10, assign93620_e143453_d_n11, assign93620_e143453_d_n14,) = {
    if (((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2165 != 0.0)) && (locals.var_guard2166 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign93620_e143453;
        locals.var_t0_dn0 = assign93620_e143453_d_n0;
        locals.var_t0_dn2 = assign93620_e143453_d_n2;
        locals.var_t0_dn4 = assign93620_e143453_d_n4;
        locals.var_t0_dn5 = assign93620_e143453_d_n5;
        locals.var_t0_dn6 = assign93620_e143453_d_n6;
        locals.var_t0_dn7 = assign93620_e143453_d_n7;
        locals.var_t0_dn8 = assign93620_e143453_d_n8;
        locals.var_t0_dn9 = assign93620_e143453_d_n9;
        locals.var_t0_dn10 = assign93620_e143453_d_n10;
        locals.var_t0_dn11 = assign93620_e143453_d_n11;
        locals.var_t0_dn14 = assign93620_e143453_d_n14;
        locals.var_t0_rv = 0.0;

    }
}
