#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_48(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign20300_e15083, assign20300_e15083_d_n0, assign20300_e15083_d_n2, assign20300_e15083_d_n4, assign20300_e15083_d_n5, assign20300_e15083_d_n6, assign20300_e15083_d_n7, assign20300_e15083_d_n8, assign20300_e15083_d_n9, assign20300_e15083_d_n10, assign20300_e15083_d_n11, assign20300_e15083_d_n14,) = {
    if (locals.var_guard409 != 0.0) {
        let assign20300_e15079: f64 = (locals.var_mks_nsubsub + locals.var_uc_nover);
        let assign20300_e15080: f64 = (locals.var_uc_nover * assign20300_e15079);
        let assign20300_e15081: f64 = (locals.var_mks_nsubsub / assign20300_e15080);
        (assign20300_e15081, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign20300_e15083;
        locals.var_t0_dn0 = assign20300_e15083_d_n0;
        locals.var_t0_dn2 = assign20300_e15083_d_n2;
        locals.var_t0_dn4 = assign20300_e15083_d_n4;
        locals.var_t0_dn5 = assign20300_e15083_d_n5;
        locals.var_t0_dn6 = assign20300_e15083_d_n6;
        locals.var_t0_dn7 = assign20300_e15083_d_n7;
        locals.var_t0_dn8 = assign20300_e15083_d_n8;
        locals.var_t0_dn9 = assign20300_e15083_d_n9;
        locals.var_t0_dn10 = assign20300_e15083_d_n10;
        locals.var_t0_dn11 = assign20300_e15083_d_n11;
        locals.var_t0_dn14 = assign20300_e15083_d_n14;

        let (assign20310_e15093, assign20310_e15093_d_n0, assign20310_e15093_d_n2, assign20310_e15093_d_n4, assign20310_e15093_d_n5, assign20310_e15093_d_n6, assign20310_e15093_d_n7, assign20310_e15093_d_n8, assign20310_e15093_d_n9, assign20310_e15093_d_n10, assign20310_e15093_d_n11, assign20310_e15093_d_n14,) = {
    if (locals.var_guard409 != 0.0) {
        let assign20310_e15087: f64 = (2.0 * 1.034943e-10);
        let assign20310_e15089: f64 = (assign20310_e15087 / 1.6021918e-19);
        let assign20310_e15091: f64 = (assign20310_e15089 * locals.var_t0);
        (assign20310_e15091, (assign20310_e15089 * locals.var_t0_dn0), (assign20310_e15089 * locals.var_t0_dn2), (assign20310_e15089 * locals.var_t0_dn4), (assign20310_e15089 * locals.var_t0_dn5), (assign20310_e15089 * locals.var_t0_dn6), (assign20310_e15089 * locals.var_t0_dn7), (assign20310_e15089 * locals.var_t0_dn8), (assign20310_e15089 * locals.var_t0_dn9), (assign20310_e15089 * locals.var_t0_dn10), (assign20310_e15089 * locals.var_t0_dn11), (assign20310_e15089 * locals.var_t0_dn14),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign20310_e15093;
        locals.var_t4_dn0 = assign20310_e15093_d_n0;
        locals.var_t4_dn2 = assign20310_e15093_d_n2;
        locals.var_t4_dn4 = assign20310_e15093_d_n4;
        locals.var_t4_dn5 = assign20310_e15093_d_n5;
        locals.var_t4_dn6 = assign20310_e15093_d_n6;
        locals.var_t4_dn7 = assign20310_e15093_d_n7;
        locals.var_t4_dn8 = assign20310_e15093_d_n8;
        locals.var_t4_dn9 = assign20310_e15093_d_n9;
        locals.var_t4_dn10 = assign20310_e15093_d_n10;
        locals.var_t4_dn11 = assign20310_e15093_d_n11;
        locals.var_t4_dn14 = assign20310_e15093_d_n14;

        let (assign20320_e15102, assign20320_e15102_d_n0, assign20320_e15102_d_n2, assign20320_e15102_d_n4, assign20320_e15102_d_n5, assign20320_e15102_d_n6, assign20320_e15102_d_n7, assign20320_e15102_d_n8, assign20320_e15102_d_n9, assign20320_e15102_d_n10, assign20320_e15102_d_n11, assign20320_e15102_d_n14,) = {
    if (locals.var_guard409 != 0.0) {
        let assign20320_e15097: f64 = (locals.var_t4 * locals.var_t1);
        let assign20320_e15098: f64 = (assign20320_e15097).sqrt();
        let assign20320_e15100: f64 = (assign20320_e15098 + 1e-25);
        (assign20320_e15100, (((locals.var_t4_dn0 * locals.var_t1) + (locals.var_t4 * locals.var_t1_dn0)) / (2.0 * assign20320_e15098)), (((locals.var_t4_dn2 * locals.var_t1) + (locals.var_t4 * locals.var_t1_dn2)) / (2.0 * assign20320_e15098)), (((locals.var_t4_dn4 * locals.var_t1) + (locals.var_t4 * locals.var_t1_dn4)) / (2.0 * assign20320_e15098)), (((locals.var_t4_dn5 * locals.var_t1) + (locals.var_t4 * locals.var_t1_dn5)) / (2.0 * assign20320_e15098)), (((locals.var_t4_dn6 * locals.var_t1) + (locals.var_t4 * locals.var_t1_dn6)) / (2.0 * assign20320_e15098)), (((locals.var_t4_dn7 * locals.var_t1) + (locals.var_t4 * locals.var_t1_dn7)) / (2.0 * assign20320_e15098)), (((locals.var_t4_dn8 * locals.var_t1) + (locals.var_t4 * locals.var_t1_dn8)) / (2.0 * assign20320_e15098)), (((locals.var_t4_dn9 * locals.var_t1) + (locals.var_t4 * locals.var_t1_dn9)) / (2.0 * assign20320_e15098)), (((locals.var_t4_dn10 * locals.var_t1) + (locals.var_t4 * locals.var_t1_dn10)) / (2.0 * assign20320_e15098)), (((locals.var_t4_dn11 * locals.var_t1) + (locals.var_t4 * locals.var_t1_dn11)) / (2.0 * assign20320_e15098)), (((locals.var_t4_dn14 * locals.var_t1) + (locals.var_t4 * locals.var_t1_dn14)) / (2.0 * assign20320_e15098)),)
    } else {
        (locals.var_wdep, locals.var_wdep_dn0, locals.var_wdep_dn2, locals.var_wdep_dn4, locals.var_wdep_dn5, locals.var_wdep_dn6, locals.var_wdep_dn7, locals.var_wdep_dn8, locals.var_wdep_dn9, locals.var_wdep_dn10, locals.var_wdep_dn11, locals.var_wdep_dn14,)
    }
};
        locals.var_wdep = assign20320_e15102;
        locals.var_wdep_dn0 = assign20320_e15102_d_n0;
        locals.var_wdep_dn2 = assign20320_e15102_d_n2;
        locals.var_wdep_dn4 = assign20320_e15102_d_n4;
        locals.var_wdep_dn5 = assign20320_e15102_d_n5;
        locals.var_wdep_dn6 = assign20320_e15102_d_n6;
        locals.var_wdep_dn7 = assign20320_e15102_d_n7;
        locals.var_wdep_dn8 = assign20320_e15102_d_n8;
        locals.var_wdep_dn9 = assign20320_e15102_d_n9;
        locals.var_wdep_dn10 = assign20320_e15102_d_n10;
        locals.var_wdep_dn11 = assign20320_e15102_d_n11;
        locals.var_wdep_dn14 = assign20320_e15102_d_n14;

        let (assign20330_e15112, assign20330_e15112_d_n0, assign20330_e15112_d_n2, assign20330_e15112_d_n4, assign20330_e15112_d_n5, assign20330_e15112_d_n6, assign20330_e15112_d_n7, assign20330_e15112_d_n8, assign20330_e15112_d_n9, assign20330_e15112_d_n10, assign20330_e15112_d_n11, assign20330_e15112_d_n14,) = {
    if (locals.var_guard409 != 0.0) {
        let assign20330_e15106: f64 = (p.p334 - locals.var_wdep);
        let assign20330_e15109: f64 = (0.1 * p.p334);
        let assign20330_e15110: f64 = (assign20330_e15106 - assign20330_e15109);
        (assign20330_e15110, (-locals.var_wdep_dn0), (-locals.var_wdep_dn2), (-locals.var_wdep_dn4), (-locals.var_wdep_dn5), (-locals.var_wdep_dn6), (-locals.var_wdep_dn7), (-locals.var_wdep_dn8), (-locals.var_wdep_dn9), (-locals.var_wdep_dn10), (-locals.var_wdep_dn11), (-locals.var_wdep_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign20330_e15112;
        locals.var_tmf1_dn0 = assign20330_e15112_d_n0;
        locals.var_tmf1_dn2 = assign20330_e15112_d_n2;
        locals.var_tmf1_dn4 = assign20330_e15112_d_n4;
        locals.var_tmf1_dn5 = assign20330_e15112_d_n5;
        locals.var_tmf1_dn6 = assign20330_e15112_d_n6;
        locals.var_tmf1_dn7 = assign20330_e15112_d_n7;
        locals.var_tmf1_dn8 = assign20330_e15112_d_n8;
        locals.var_tmf1_dn9 = assign20330_e15112_d_n9;
        locals.var_tmf1_dn10 = assign20330_e15112_d_n10;
        locals.var_tmf1_dn11 = assign20330_e15112_d_n11;
        locals.var_tmf1_dn14 = assign20330_e15112_d_n14;

        let (assign20340_e15122, assign20340_e15122_d_n0, assign20340_e15122_d_n2, assign20340_e15122_d_n4, assign20340_e15122_d_n5, assign20340_e15122_d_n6, assign20340_e15122_d_n7, assign20340_e15122_d_n8, assign20340_e15122_d_n9, assign20340_e15122_d_n10, assign20340_e15122_d_n11, assign20340_e15122_d_n14,) = {
    if (locals.var_guard409 != 0.0) {
        let assign20340_e15116: f64 = (4.0 * p.p334);
        let assign20340_e15119: f64 = (0.1 * p.p334);
        let assign20340_e15120: f64 = (assign20340_e15116 * assign20340_e15119);
        (assign20340_e15120, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign20340_e15122;
        locals.var_tmf2_dn0 = assign20340_e15122_d_n0;
        locals.var_tmf2_dn2 = assign20340_e15122_d_n2;
        locals.var_tmf2_dn4 = assign20340_e15122_d_n4;
        locals.var_tmf2_dn5 = assign20340_e15122_d_n5;
        locals.var_tmf2_dn6 = assign20340_e15122_d_n6;
        locals.var_tmf2_dn7 = assign20340_e15122_d_n7;
        locals.var_tmf2_dn8 = assign20340_e15122_d_n8;
        locals.var_tmf2_dn9 = assign20340_e15122_d_n9;
        locals.var_tmf2_dn10 = assign20340_e15122_d_n10;
        locals.var_tmf2_dn11 = assign20340_e15122_d_n11;
        locals.var_tmf2_dn14 = assign20340_e15122_d_n14;

        let (assign20350_e15132, assign20350_e15132_d_n0, assign20350_e15132_d_n2, assign20350_e15132_d_n4, assign20350_e15132_d_n5, assign20350_e15132_d_n6, assign20350_e15132_d_n7, assign20350_e15132_d_n8, assign20350_e15132_d_n9, assign20350_e15132_d_n10, assign20350_e15132_d_n11, assign20350_e15132_d_n14,) = {
    if (locals.var_guard409 != 0.0) {
        let (assign20350_e15130, assign20350_e15130_d_n0, assign20350_e15130_d_n2, assign20350_e15130_d_n4, assign20350_e15130_d_n5, assign20350_e15130_d_n6, assign20350_e15130_d_n7, assign20350_e15130_d_n8, assign20350_e15130_d_n9, assign20350_e15130_d_n10, assign20350_e15130_d_n11, assign20350_e15130_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign20350_e15129: f64 = (-locals.var_tmf2);
                (assign20350_e15129, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign20350_e15130, assign20350_e15130_d_n0, assign20350_e15130_d_n2, assign20350_e15130_d_n4, assign20350_e15130_d_n5, assign20350_e15130_d_n6, assign20350_e15130_d_n7, assign20350_e15130_d_n8, assign20350_e15130_d_n9, assign20350_e15130_d_n10, assign20350_e15130_d_n11, assign20350_e15130_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign20350_e15132;
        locals.var_tmf2_dn0 = assign20350_e15132_d_n0;
        locals.var_tmf2_dn2 = assign20350_e15132_d_n2;
        locals.var_tmf2_dn4 = assign20350_e15132_d_n4;
        locals.var_tmf2_dn5 = assign20350_e15132_d_n5;
        locals.var_tmf2_dn6 = assign20350_e15132_d_n6;
        locals.var_tmf2_dn7 = assign20350_e15132_d_n7;
        locals.var_tmf2_dn8 = assign20350_e15132_d_n8;
        locals.var_tmf2_dn9 = assign20350_e15132_d_n9;
        locals.var_tmf2_dn10 = assign20350_e15132_d_n10;
        locals.var_tmf2_dn11 = assign20350_e15132_d_n11;
        locals.var_tmf2_dn14 = assign20350_e15132_d_n14;

        let (assign20360_e15141, assign20360_e15141_d_n0, assign20360_e15141_d_n2, assign20360_e15141_d_n4, assign20360_e15141_d_n5, assign20360_e15141_d_n6, assign20360_e15141_d_n7, assign20360_e15141_d_n8, assign20360_e15141_d_n9, assign20360_e15141_d_n10, assign20360_e15141_d_n11, assign20360_e15141_d_n14,) = {
    if (locals.var_guard409 != 0.0) {
        let assign20360_e15136: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign20360_e15138: f64 = (assign20360_e15136 + locals.var_tmf2);
        let assign20360_e15139: f64 = (assign20360_e15138).sqrt();
        (assign20360_e15139, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign20360_e15139)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign20360_e15139)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign20360_e15139)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign20360_e15139)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign20360_e15139)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign20360_e15139)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign20360_e15139)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign20360_e15139)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign20360_e15139)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign20360_e15139)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign20360_e15139)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign20360_e15141;
        locals.var_tmf2_dn0 = assign20360_e15141_d_n0;
        locals.var_tmf2_dn2 = assign20360_e15141_d_n2;
        locals.var_tmf2_dn4 = assign20360_e15141_d_n4;
        locals.var_tmf2_dn5 = assign20360_e15141_d_n5;
        locals.var_tmf2_dn6 = assign20360_e15141_d_n6;
        locals.var_tmf2_dn7 = assign20360_e15141_d_n7;
        locals.var_tmf2_dn8 = assign20360_e15141_d_n8;
        locals.var_tmf2_dn9 = assign20360_e15141_d_n9;
        locals.var_tmf2_dn10 = assign20360_e15141_d_n10;
        locals.var_tmf2_dn11 = assign20360_e15141_d_n11;
        locals.var_tmf2_dn14 = assign20360_e15141_d_n14;

        let (assign20370_e15151, assign20370_e15151_d_n0, assign20370_e15151_d_n2, assign20370_e15151_d_n4, assign20370_e15151_d_n5, assign20370_e15151_d_n6, assign20370_e15151_d_n7, assign20370_e15151_d_n8, assign20370_e15151_d_n9, assign20370_e15151_d_n10, assign20370_e15151_d_n11, assign20370_e15151_d_n14,) = {
    if (locals.var_guard409 != 0.0) {
        let assign20370_e15147: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign20370_e15148: f64 = (1.0 + assign20370_e15147);
        let assign20370_e15149: f64 = (0.5 * assign20370_e15148);
        (assign20370_e15149, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign20370_e15151;
        locals.var_t0_dn0 = assign20370_e15151_d_n0;
        locals.var_t0_dn2 = assign20370_e15151_d_n2;
        locals.var_t0_dn4 = assign20370_e15151_d_n4;
        locals.var_t0_dn5 = assign20370_e15151_d_n5;
        locals.var_t0_dn6 = assign20370_e15151_d_n6;
        locals.var_t0_dn7 = assign20370_e15151_d_n7;
        locals.var_t0_dn8 = assign20370_e15151_d_n8;
        locals.var_t0_dn9 = assign20370_e15151_d_n9;
        locals.var_t0_dn10 = assign20370_e15151_d_n10;
        locals.var_t0_dn11 = assign20370_e15151_d_n11;
        locals.var_t0_dn14 = assign20370_e15151_d_n14;

        let (assign20380_e15161, assign20380_e15161_d_n0, assign20380_e15161_d_n2, assign20380_e15161_d_n4, assign20380_e15161_d_n5, assign20380_e15161_d_n6, assign20380_e15161_d_n7, assign20380_e15161_d_n8, assign20380_e15161_d_n9, assign20380_e15161_d_n10, assign20380_e15161_d_n11, assign20380_e15161_d_n14,) = {
    if (locals.var_guard409 != 0.0) {
        let assign20380_e15157: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign20380_e15158: f64 = (0.5 * assign20380_e15157);
        let assign20380_e15159: f64 = (p.p334 - assign20380_e15158);
        (assign20380_e15159, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (-(0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (-(0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (-(0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (-(0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (-(0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (-(0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (-(0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (-(0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (-(0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_wdep, locals.var_wdep_dn0, locals.var_wdep_dn2, locals.var_wdep_dn4, locals.var_wdep_dn5, locals.var_wdep_dn6, locals.var_wdep_dn7, locals.var_wdep_dn8, locals.var_wdep_dn9, locals.var_wdep_dn10, locals.var_wdep_dn11, locals.var_wdep_dn14,)
    }
};
        locals.var_wdep = assign20380_e15161;
        locals.var_wdep_dn0 = assign20380_e15161_d_n0;
        locals.var_wdep_dn2 = assign20380_e15161_d_n2;
        locals.var_wdep_dn4 = assign20380_e15161_d_n4;
        locals.var_wdep_dn5 = assign20380_e15161_d_n5;
        locals.var_wdep_dn6 = assign20380_e15161_d_n6;
        locals.var_wdep_dn7 = assign20380_e15161_d_n7;
        locals.var_wdep_dn8 = assign20380_e15161_d_n8;
        locals.var_wdep_dn9 = assign20380_e15161_d_n9;
        locals.var_wdep_dn10 = assign20380_e15161_d_n10;
        locals.var_wdep_dn11 = assign20380_e15161_d_n11;
        locals.var_wdep_dn14 = assign20380_e15161_d_n14;

        let (assign20390_e15166, assign20390_e15166_d_n0, assign20390_e15166_d_n2, assign20390_e15166_d_n4, assign20390_e15166_d_n5, assign20390_e15166_d_n6, assign20390_e15166_d_n7, assign20390_e15166_d_n8, assign20390_e15166_d_n9, assign20390_e15166_d_n10, assign20390_e15166_d_n11, assign20390_e15166_d_n14,) = {
    if (locals.var_guard409 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_wdep, locals.var_wdep_dn0, locals.var_wdep_dn2, locals.var_wdep_dn4, locals.var_wdep_dn5, locals.var_wdep_dn6, locals.var_wdep_dn7, locals.var_wdep_dn8, locals.var_wdep_dn9, locals.var_wdep_dn10, locals.var_wdep_dn11, locals.var_wdep_dn14,)
    }
};
        locals.var_wdep = assign20390_e15166;
        locals.var_wdep_dn0 = assign20390_e15166_d_n0;
        locals.var_wdep_dn2 = assign20390_e15166_d_n2;
        locals.var_wdep_dn4 = assign20390_e15166_d_n4;
        locals.var_wdep_dn5 = assign20390_e15166_d_n5;
        locals.var_wdep_dn6 = assign20390_e15166_d_n6;
        locals.var_wdep_dn7 = assign20390_e15166_d_n7;
        locals.var_wdep_dn8 = assign20390_e15166_d_n8;
        locals.var_wdep_dn9 = assign20390_e15166_d_n9;
        locals.var_wdep_dn10 = assign20390_e15166_d_n10;
        locals.var_wdep_dn11 = assign20390_e15166_d_n11;
        locals.var_wdep_dn14 = assign20390_e15166_d_n14;

        let assign20400_e15173: f64 = if ((locals.var_flg_rsrd == 1.0) || (locals.var_flg_rsrd == 3.0)) { 1.0 } else { 0.0 };
        locals.var_guard413 = assign20400_e15173;

        let (assign20410_e15177, assign20410_e15177_d_n0, assign20410_e15177_d_n2,) = {
    if (locals.var_guard413 != 0.0) {
        (locals.var_vdsei, locals.var_vdsei_dn0, locals.var_vdsei_dn2,)
    } else {
        (locals.var_vdsegmt, locals.var_vdsegmt_dn0, locals.var_vdsegmt_dn2,)
    }
};
        locals.var_vdsegmt = assign20410_e15177;
        locals.var_vdsegmt_dn0 = assign20410_e15177_d_n0;
        locals.var_vdsegmt_dn2 = assign20410_e15177_d_n2;

        let (assign20420_e15181, assign20420_e15181_d_n2, assign20420_e15181_d_n7,) = {
    if (locals.var_guard413 != 0.0) {
        (locals.var_vgsei, locals.var_vgsei_dn2, locals.var_vgsei_dn7,)
    } else {
        (locals.var_vgsegmt, locals.var_vgsegmt_dn2, locals.var_vgsegmt_dn7,)
    }
};
        locals.var_vgsegmt = assign20420_e15181;
        locals.var_vgsegmt_dn2 = assign20420_e15181_d_n2;
        locals.var_vgsegmt_dn7 = assign20420_e15181_d_n7;

        let (assign20430_e15185, assign20430_e15185_d_n2, assign20430_e15185_d_n9,) = {
    if (locals.var_guard413 != 0.0) {
        (locals.var_vbsei, locals.var_vbsei_dn2, locals.var_vbsei_dn9,)
    } else {
        (locals.var_vbsegmt, locals.var_vbsegmt_dn2, locals.var_vbsegmt_dn9,)
    }
};
        locals.var_vbsegmt = assign20430_e15185;
        locals.var_vbsegmt_dn2 = assign20430_e15185_d_n2;
        locals.var_vbsegmt_dn9 = assign20430_e15185_d_n9;

        let assign20440_e15188: f64 = if locals.var_vdsegmt >= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard414 = assign20440_e15188;

        let (assign20450_e15194,) = {
    if ((locals.var_guard413 != 0.0) && (locals.var_guard414 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_vdsemodenml,)
    }
};
        locals.var_vdsemodenml = assign20450_e15194;

        let (assign20460_e15200,) = {
    if ((locals.var_guard413 != 0.0) && (locals.var_guard414 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_vdsemodervs,)
    }
};
        locals.var_vdsemodervs = assign20460_e15200;

        let (assign20470_e15206, assign20470_e15206_d_n0, assign20470_e15206_d_n2,) = {
    if ((locals.var_guard413 != 0.0) && (locals.var_guard414 != 0.0)) {
        (locals.var_vdsegmt, locals.var_vdsegmt_dn0, locals.var_vdsegmt_dn2,)
    } else {
        (locals.var_vdserev, locals.var_vdserev_dn0, locals.var_vdserev_dn2,)
    }
};
        locals.var_vdserev = assign20470_e15206;
        locals.var_vdserev_dn0 = assign20470_e15206_d_n0;
        locals.var_vdserev_dn2 = assign20470_e15206_d_n2;

        let (assign20480_e15212, assign20480_e15212_d_n0, assign20480_e15212_d_n2, assign20480_e15212_d_n7,) = {
    if ((locals.var_guard413 != 0.0) && (locals.var_guard414 != 0.0)) {
        (locals.var_vgsegmt, 0.0, locals.var_vgsegmt_dn2, locals.var_vgsegmt_dn7,)
    } else {
        (locals.var_vgserev, locals.var_vgserev_dn0, locals.var_vgserev_dn2, locals.var_vgserev_dn7,)
    }
};
        locals.var_vgserev = assign20480_e15212;
        locals.var_vgserev_dn0 = assign20480_e15212_d_n0;
        locals.var_vgserev_dn2 = assign20480_e15212_d_n2;
        locals.var_vgserev_dn7 = assign20480_e15212_d_n7;

        let (assign20490_e15218, assign20490_e15218_d_n0, assign20490_e15218_d_n2, assign20490_e15218_d_n9,) = {
    if ((locals.var_guard413 != 0.0) && (locals.var_guard414 != 0.0)) {
        (locals.var_vbsegmt, 0.0, locals.var_vbsegmt_dn2, locals.var_vbsegmt_dn9,)
    } else {
        (locals.var_vbserev, locals.var_vbserev_dn0, locals.var_vbserev_dn2, locals.var_vbserev_dn9,)
    }
};
        locals.var_vbserev = assign20490_e15218;
        locals.var_vbserev_dn0 = assign20490_e15218_d_n0;
        locals.var_vbserev_dn2 = assign20490_e15218_d_n2;
        locals.var_vbserev_dn9 = assign20490_e15218_d_n9;

        let (assign20500_e15224, assign20500_e15224_d_n0, assign20500_e15224_d_n2, assign20500_e15224_d_n4,) = {
    if ((locals.var_guard413 != 0.0) && (locals.var_guard414 != 0.0)) {
        (locals.var_vsubs, 0.0, locals.var_vsubs_dn2, locals.var_vsubs_dn4,)
    } else {
        (locals.var_vsubsrev, locals.var_vsubsrev_dn0, locals.var_vsubsrev_dn2, locals.var_vsubsrev_dn4,)
    }
};
        locals.var_vsubsrev = assign20500_e15224;
        locals.var_vsubsrev_dn0 = assign20500_e15224_d_n0;
        locals.var_vsubsrev_dn2 = assign20500_e15224_d_n2;
        locals.var_vsubsrev_dn4 = assign20500_e15224_d_n4;

        let (assign20510_e15231,) = {
    if ((locals.var_guard413 != 0.0) && (locals.var_guard414 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_vdsemodenml,)
    }
};
        locals.var_vdsemodenml = assign20510_e15231;

        let (assign20520_e15238,) = {
    if ((locals.var_guard413 != 0.0) && (locals.var_guard414 == 0.0)) {
        (1.0,)
    } else {
        (locals.var_vdsemodervs,)
    }
};
        locals.var_vdsemodervs = assign20520_e15238;

        let (assign20530_e15246, assign20530_e15246_d_n0, assign20530_e15246_d_n2,) = {
    if ((locals.var_guard413 != 0.0) && (locals.var_guard414 == 0.0)) {
        let assign20530_e15244: f64 = (-locals.var_vdsegmt);
        (assign20530_e15244, (-locals.var_vdsegmt_dn0), (-locals.var_vdsegmt_dn2),)
    } else {
        (locals.var_vdserev, locals.var_vdserev_dn0, locals.var_vdserev_dn2,)
    }
};
        locals.var_vdserev = assign20530_e15246;
        locals.var_vdserev_dn0 = assign20530_e15246_d_n0;
        locals.var_vdserev_dn2 = assign20530_e15246_d_n2;

        let (assign20540_e15255, assign20540_e15255_d_n0, assign20540_e15255_d_n2, assign20540_e15255_d_n7,) = {
    if ((locals.var_guard413 != 0.0) && (locals.var_guard414 == 0.0)) {
        let assign20540_e15253: f64 = (locals.var_vgsegmt - locals.var_vdsegmt);
        (assign20540_e15253, (-locals.var_vdsegmt_dn0), (locals.var_vgsegmt_dn2 - locals.var_vdsegmt_dn2), locals.var_vgsegmt_dn7,)
    } else {
        (locals.var_vgserev, locals.var_vgserev_dn0, locals.var_vgserev_dn2, locals.var_vgserev_dn7,)
    }
};
        locals.var_vgserev = assign20540_e15255;
        locals.var_vgserev_dn0 = assign20540_e15255_d_n0;
        locals.var_vgserev_dn2 = assign20540_e15255_d_n2;
        locals.var_vgserev_dn7 = assign20540_e15255_d_n7;

        let (assign20550_e15264, assign20550_e15264_d_n0, assign20550_e15264_d_n2, assign20550_e15264_d_n9,) = {
    if ((locals.var_guard413 != 0.0) && (locals.var_guard414 == 0.0)) {
        let assign20550_e15262: f64 = (locals.var_vbsegmt - locals.var_vdsegmt);
        (assign20550_e15262, (-locals.var_vdsegmt_dn0), (locals.var_vbsegmt_dn2 - locals.var_vdsegmt_dn2), locals.var_vbsegmt_dn9,)
    } else {
        (locals.var_vbserev, locals.var_vbserev_dn0, locals.var_vbserev_dn2, locals.var_vbserev_dn9,)
    }
};
        locals.var_vbserev = assign20550_e15264;
        locals.var_vbserev_dn0 = assign20550_e15264_d_n0;
        locals.var_vbserev_dn2 = assign20550_e15264_d_n2;
        locals.var_vbserev_dn9 = assign20550_e15264_d_n9;

        let (assign20560_e15273, assign20560_e15273_d_n0, assign20560_e15273_d_n2, assign20560_e15273_d_n4,) = {
    if ((locals.var_guard413 != 0.0) && (locals.var_guard414 == 0.0)) {
        let assign20560_e15271: f64 = (locals.var_vsubs - locals.var_vdsegmt);
        (assign20560_e15271, (-locals.var_vdsegmt_dn0), (locals.var_vsubs_dn2 - locals.var_vdsegmt_dn2), locals.var_vsubs_dn4,)
    } else {
        (locals.var_vsubsrev, locals.var_vsubsrev_dn0, locals.var_vsubsrev_dn2, locals.var_vsubsrev_dn4,)
    }
};
        locals.var_vsubsrev = assign20560_e15273;
        locals.var_vsubsrev_dn0 = assign20560_e15273_d_n0;
        locals.var_vsubsrev_dn2 = assign20560_e15273_d_n2;
        locals.var_vsubsrev_dn4 = assign20560_e15273_d_n4;

        let assign20570_e15292: f64 = if (((((locals.var_rdvde > 0.0) || (locals.var_rsvde > 0.0)) || (locals.var_uc_rdvg11 > 0.0)) || (locals.var_uc_rdvb > 0.0)) || (p.p54 == 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard415 = assign20570_e15292;

        let (assign20580_e15304, assign20580_e15304_d_n0, assign20580_e15304_d_n2, assign20580_e15304_d_n4, assign20580_e15304_d_n5, assign20580_e15304_d_n6, assign20580_e15304_d_n7, assign20580_e15304_d_n8, assign20580_e15304_d_n9, assign20580_e15304_d_n10, assign20580_e15304_d_n11, assign20580_e15304_d_n14,) = {
    if ((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) {
        let assign20580_e15299: f64 = (locals.var_vdserev / 2.0);
        let assign20580_e15300: f64 = (2.0 * assign20580_e15299);
        let assign20580_e15302: f64 = (assign20580_e15300 / p.p262);
        (assign20580_e15302, ((2.0 * (locals.var_vdserev_dn0 / 2.0)) / p.p262), ((2.0 * (locals.var_vdserev_dn2 / 2.0)) / p.p262), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign20580_e15304;
        locals.var_tmf1_dn0 = assign20580_e15304_d_n0;
        locals.var_tmf1_dn2 = assign20580_e15304_d_n2;
        locals.var_tmf1_dn4 = assign20580_e15304_d_n4;
        locals.var_tmf1_dn5 = assign20580_e15304_d_n5;
        locals.var_tmf1_dn6 = assign20580_e15304_d_n6;
        locals.var_tmf1_dn7 = assign20580_e15304_d_n7;
        locals.var_tmf1_dn8 = assign20580_e15304_d_n8;
        locals.var_tmf1_dn9 = assign20580_e15304_d_n9;
        locals.var_tmf1_dn10 = assign20580_e15304_d_n10;
        locals.var_tmf1_dn11 = assign20580_e15304_d_n11;
        locals.var_tmf1_dn14 = assign20580_e15304_d_n14;

        let (assign20590_e15346, assign20590_e15346_d_n0, assign20590_e15346_d_n2, assign20590_e15346_d_n4, assign20590_e15346_d_n5, assign20590_e15346_d_n6, assign20590_e15346_d_n7, assign20590_e15346_d_n8, assign20590_e15346_d_n9, assign20590_e15346_d_n10, assign20590_e15346_d_n11, assign20590_e15346_d_n14,) = {
    if ((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) {
        let assign20590_e15312: f64 = (1.0 / 2.0);
        let assign20590_e15316: f64 = (1.0 / 6.0);
        let assign20590_e15320: f64 = (1.0 / 24.0);
        let assign20590_e15324: f64 = (1.0 / 120.0);
        let assign20590_e15328: f64 = (1.0 / 720.0);
        let assign20590_e15332: f64 = (1.0 / 5040.0);
        let assign20590_e15333: f64 = (locals.var_tmf1 * assign20590_e15332);
        let assign20590_e15334: f64 = (assign20590_e15328 + assign20590_e15333);
        let assign20590_e15335: f64 = (locals.var_tmf1 * assign20590_e15334);
        let assign20590_e15336: f64 = (assign20590_e15324 + assign20590_e15335);
        let assign20590_e15337: f64 = (locals.var_tmf1 * assign20590_e15336);
        let assign20590_e15338: f64 = (assign20590_e15320 + assign20590_e15337);
        let assign20590_e15339: f64 = (locals.var_tmf1 * assign20590_e15338);
        let assign20590_e15340: f64 = (assign20590_e15316 + assign20590_e15339);
        let assign20590_e15341: f64 = (locals.var_tmf1 * assign20590_e15340);
        let assign20590_e15342: f64 = (assign20590_e15312 + assign20590_e15341);
        let assign20590_e15343: f64 = (locals.var_tmf1 * assign20590_e15342);
        let assign20590_e15344: f64 = (1.0 + assign20590_e15343);
        (assign20590_e15344, ((locals.var_tmf1_dn0 * assign20590_e15342) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign20590_e15340) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign20590_e15338) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign20590_e15336) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign20590_e15334) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign20590_e15332))))))))))), ((locals.var_tmf1_dn2 * assign20590_e15342) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign20590_e15340) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign20590_e15338) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign20590_e15336) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign20590_e15334) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign20590_e15332))))))))))), ((locals.var_tmf1_dn4 * assign20590_e15342) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign20590_e15340) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign20590_e15338) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign20590_e15336) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign20590_e15334) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign20590_e15332))))))))))), ((locals.var_tmf1_dn5 * assign20590_e15342) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign20590_e15340) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign20590_e15338) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign20590_e15336) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign20590_e15334) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign20590_e15332))))))))))), ((locals.var_tmf1_dn6 * assign20590_e15342) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign20590_e15340) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign20590_e15338) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign20590_e15336) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign20590_e15334) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign20590_e15332))))))))))), ((locals.var_tmf1_dn7 * assign20590_e15342) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign20590_e15340) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign20590_e15338) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign20590_e15336) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign20590_e15334) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign20590_e15332))))))))))), ((locals.var_tmf1_dn8 * assign20590_e15342) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign20590_e15340) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign20590_e15338) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign20590_e15336) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign20590_e15334) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign20590_e15332))))))))))), ((locals.var_tmf1_dn9 * assign20590_e15342) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign20590_e15340) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign20590_e15338) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign20590_e15336) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign20590_e15334) + (locals.var_tmf1 * (locals.var_tmf1_dn9 * assign20590_e15332))))))))))), ((locals.var_tmf1_dn10 * assign20590_e15342) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign20590_e15340) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign20590_e15338) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign20590_e15336) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign20590_e15334) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign20590_e15332))))))))))), ((locals.var_tmf1_dn11 * assign20590_e15342) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign20590_e15340) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign20590_e15338) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign20590_e15336) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign20590_e15334) + (locals.var_tmf1 * (locals.var_tmf1_dn11 * assign20590_e15332))))))))))), ((locals.var_tmf1_dn14 * assign20590_e15342) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign20590_e15340) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign20590_e15338) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign20590_e15336) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign20590_e15334) + (locals.var_tmf1 * (locals.var_tmf1_dn14 * assign20590_e15332))))))))))),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign20590_e15346;
        locals.var_tmf2_dn0 = assign20590_e15346_d_n0;
        locals.var_tmf2_dn2 = assign20590_e15346_d_n2;
        locals.var_tmf2_dn4 = assign20590_e15346_d_n4;
        locals.var_tmf2_dn5 = assign20590_e15346_d_n5;
        locals.var_tmf2_dn6 = assign20590_e15346_d_n6;
        locals.var_tmf2_dn7 = assign20590_e15346_d_n7;
        locals.var_tmf2_dn8 = assign20590_e15346_d_n8;
        locals.var_tmf2_dn9 = assign20590_e15346_d_n9;
        locals.var_tmf2_dn10 = assign20590_e15346_d_n10;
        locals.var_tmf2_dn11 = assign20590_e15346_d_n11;
        locals.var_tmf2_dn14 = assign20590_e15346_d_n14;

        let (assign20600_e15384, assign20600_e15384_d_n0, assign20600_e15384_d_n2, assign20600_e15384_d_n4, assign20600_e15384_d_n5, assign20600_e15384_d_n6, assign20600_e15384_d_n7, assign20600_e15384_d_n8, assign20600_e15384_d_n9, assign20600_e15384_d_n10, assign20600_e15384_d_n11, assign20600_e15384_d_n14,) = {
    if ((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) {
        let assign20600_e15352: f64 = (1.0 / 2.0);
        let assign20600_e15356: f64 = (1.0 / 3.0);
        let assign20600_e15360: f64 = (1.0 / 8.0);
        let assign20600_e15364: f64 = (1.0 / 30.0);
        let assign20600_e15368: f64 = (1.0 / 144.0);
        let assign20600_e15372: f64 = (1.0 / 840.0);
        let assign20600_e15373: f64 = (locals.var_tmf1 * assign20600_e15372);
        let assign20600_e15374: f64 = (assign20600_e15368 + assign20600_e15373);
        let assign20600_e15375: f64 = (locals.var_tmf1 * assign20600_e15374);
        let assign20600_e15376: f64 = (assign20600_e15364 + assign20600_e15375);
        let assign20600_e15377: f64 = (locals.var_tmf1 * assign20600_e15376);
        let assign20600_e15378: f64 = (assign20600_e15360 + assign20600_e15377);
        let assign20600_e15379: f64 = (locals.var_tmf1 * assign20600_e15378);
        let assign20600_e15380: f64 = (assign20600_e15356 + assign20600_e15379);
        let assign20600_e15381: f64 = (locals.var_tmf1 * assign20600_e15380);
        let assign20600_e15382: f64 = (assign20600_e15352 + assign20600_e15381);
        (assign20600_e15382, ((locals.var_tmf1_dn0 * assign20600_e15380) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign20600_e15378) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign20600_e15376) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign20600_e15374) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign20600_e15372))))))))), ((locals.var_tmf1_dn2 * assign20600_e15380) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign20600_e15378) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign20600_e15376) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign20600_e15374) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign20600_e15372))))))))), ((locals.var_tmf1_dn4 * assign20600_e15380) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign20600_e15378) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign20600_e15376) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign20600_e15374) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign20600_e15372))))))))), ((locals.var_tmf1_dn5 * assign20600_e15380) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign20600_e15378) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign20600_e15376) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign20600_e15374) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign20600_e15372))))))))), ((locals.var_tmf1_dn6 * assign20600_e15380) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign20600_e15378) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign20600_e15376) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign20600_e15374) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign20600_e15372))))))))), ((locals.var_tmf1_dn7 * assign20600_e15380) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign20600_e15378) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign20600_e15376) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign20600_e15374) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign20600_e15372))))))))), ((locals.var_tmf1_dn8 * assign20600_e15380) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign20600_e15378) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign20600_e15376) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign20600_e15374) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign20600_e15372))))))))), ((locals.var_tmf1_dn9 * assign20600_e15380) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign20600_e15378) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign20600_e15376) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign20600_e15374) + (locals.var_tmf1 * (locals.var_tmf1_dn9 * assign20600_e15372))))))))), ((locals.var_tmf1_dn10 * assign20600_e15380) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign20600_e15378) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign20600_e15376) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign20600_e15374) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign20600_e15372))))))))), ((locals.var_tmf1_dn11 * assign20600_e15380) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign20600_e15378) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign20600_e15376) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign20600_e15374) + (locals.var_tmf1 * (locals.var_tmf1_dn11 * assign20600_e15372))))))))), ((locals.var_tmf1_dn14 * assign20600_e15380) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign20600_e15378) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign20600_e15376) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign20600_e15374) + (locals.var_tmf1 * (locals.var_tmf1_dn14 * assign20600_e15372))))))))),)
    } else {
        (locals.var_tmf3, locals.var_tmf3_dn0, locals.var_tmf3_dn2, locals.var_tmf3_dn4, locals.var_tmf3_dn5, locals.var_tmf3_dn6, locals.var_tmf3_dn7, locals.var_tmf3_dn8, locals.var_tmf3_dn9, locals.var_tmf3_dn10, locals.var_tmf3_dn11, locals.var_tmf3_dn14,)
    }
};
        locals.var_tmf3 = assign20600_e15384;
        locals.var_tmf3_dn0 = assign20600_e15384_d_n0;
        locals.var_tmf3_dn2 = assign20600_e15384_d_n2;
        locals.var_tmf3_dn4 = assign20600_e15384_d_n4;
        locals.var_tmf3_dn5 = assign20600_e15384_d_n5;
        locals.var_tmf3_dn6 = assign20600_e15384_d_n6;
        locals.var_tmf3_dn7 = assign20600_e15384_d_n7;
        locals.var_tmf3_dn8 = assign20600_e15384_d_n8;
        locals.var_tmf3_dn9 = assign20600_e15384_d_n9;
        locals.var_tmf3_dn10 = assign20600_e15384_d_n10;
        locals.var_tmf3_dn11 = assign20600_e15384_d_n11;
        locals.var_tmf3_dn14 = assign20600_e15384_d_n14;

    }

    pub(super) fn stamp_transient_block_49(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign20610_e15392, assign20610_e15392_d_n0, assign20610_e15392_d_n2, assign20610_e15392_d_n4, assign20610_e15392_d_n5, assign20610_e15392_d_n6, assign20610_e15392_d_n7, assign20610_e15392_d_n8, assign20610_e15392_d_n9, assign20610_e15392_d_n10, assign20610_e15392_d_n11, assign20610_e15392_d_n14,) = {
    if ((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) {
        let assign20610_e15390: f64 = (p.p262 / locals.var_tmf2);
        (assign20610_e15390, (-((p.p262 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn5) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn7) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn8) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn11) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn14) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_vzadd, locals.var_vzadd_dn0, locals.var_vzadd_dn2, locals.var_vzadd_dn4, locals.var_vzadd_dn5, locals.var_vzadd_dn6, locals.var_vzadd_dn7, locals.var_vzadd_dn8, locals.var_vzadd_dn9, locals.var_vzadd_dn10, locals.var_vzadd_dn11, locals.var_vzadd_dn14,)
    }
};
        locals.var_vzadd = assign20610_e15392;
        locals.var_vzadd_dn0 = assign20610_e15392_d_n0;
        locals.var_vzadd_dn2 = assign20610_e15392_d_n2;
        locals.var_vzadd_dn4 = assign20610_e15392_d_n4;
        locals.var_vzadd_dn5 = assign20610_e15392_d_n5;
        locals.var_vzadd_dn6 = assign20610_e15392_d_n6;
        locals.var_vzadd_dn7 = assign20610_e15392_d_n7;
        locals.var_vzadd_dn8 = assign20610_e15392_d_n8;
        locals.var_vzadd_dn9 = assign20610_e15392_d_n9;
        locals.var_vzadd_dn10 = assign20610_e15392_d_n10;
        locals.var_vzadd_dn11 = assign20610_e15392_d_n11;
        locals.var_vzadd_dn14 = assign20610_e15392_d_n14;

        let (assign20620_e15405, assign20620_e15405_d_n0, assign20620_e15405_d_n2, assign20620_e15405_d_n4, assign20620_e15405_d_n5, assign20620_e15405_d_n6, assign20620_e15405_d_n7, assign20620_e15405_d_n8, assign20620_e15405_d_n9, assign20620_e15405_d_n10, assign20620_e15405_d_n11, assign20620_e15405_d_n14,) = {
    if ((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) {
        let assign20620_e15397: f64 = (-2.0);
        let assign20620_e15399: f64 = (assign20620_e15397 * locals.var_tmf3);
        let assign20620_e15402: f64 = (locals.var_tmf2 * locals.var_tmf2);
        let assign20620_e15403: f64 = (assign20620_e15399 / assign20620_e15402);
        (assign20620_e15403, ((((assign20620_e15397 * locals.var_tmf3_dn0) * assign20620_e15402) - (assign20620_e15399 * ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0)))) / (assign20620_e15402 * assign20620_e15402)), ((((assign20620_e15397 * locals.var_tmf3_dn2) * assign20620_e15402) - (assign20620_e15399 * ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2)))) / (assign20620_e15402 * assign20620_e15402)), ((((assign20620_e15397 * locals.var_tmf3_dn4) * assign20620_e15402) - (assign20620_e15399 * ((locals.var_tmf2_dn4 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn4)))) / (assign20620_e15402 * assign20620_e15402)), ((((assign20620_e15397 * locals.var_tmf3_dn5) * assign20620_e15402) - (assign20620_e15399 * ((locals.var_tmf2_dn5 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn5)))) / (assign20620_e15402 * assign20620_e15402)), ((((assign20620_e15397 * locals.var_tmf3_dn6) * assign20620_e15402) - (assign20620_e15399 * ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6)))) / (assign20620_e15402 * assign20620_e15402)), ((((assign20620_e15397 * locals.var_tmf3_dn7) * assign20620_e15402) - (assign20620_e15399 * ((locals.var_tmf2_dn7 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn7)))) / (assign20620_e15402 * assign20620_e15402)), ((((assign20620_e15397 * locals.var_tmf3_dn8) * assign20620_e15402) - (assign20620_e15399 * ((locals.var_tmf2_dn8 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn8)))) / (assign20620_e15402 * assign20620_e15402)), ((((assign20620_e15397 * locals.var_tmf3_dn9) * assign20620_e15402) - (assign20620_e15399 * ((locals.var_tmf2_dn9 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn9)))) / (assign20620_e15402 * assign20620_e15402)), ((((assign20620_e15397 * locals.var_tmf3_dn10) * assign20620_e15402) - (assign20620_e15399 * ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10)))) / (assign20620_e15402 * assign20620_e15402)), ((((assign20620_e15397 * locals.var_tmf3_dn11) * assign20620_e15402) - (assign20620_e15399 * ((locals.var_tmf2_dn11 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn11)))) / (assign20620_e15402 * assign20620_e15402)), ((((assign20620_e15397 * locals.var_tmf3_dn14) * assign20620_e15402) - (assign20620_e15399 * ((locals.var_tmf2_dn14 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn14)))) / (assign20620_e15402 * assign20620_e15402)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign20620_e15405;
        locals.var_t2_dn0 = assign20620_e15405_d_n0;
        locals.var_t2_dn2 = assign20620_e15405_d_n2;
        locals.var_t2_dn4 = assign20620_e15405_d_n4;
        locals.var_t2_dn5 = assign20620_e15405_d_n5;
        locals.var_t2_dn6 = assign20620_e15405_d_n6;
        locals.var_t2_dn7 = assign20620_e15405_d_n7;
        locals.var_t2_dn8 = assign20620_e15405_d_n8;
        locals.var_t2_dn9 = assign20620_e15405_d_n9;
        locals.var_t2_dn10 = assign20620_e15405_d_n10;
        locals.var_t2_dn11 = assign20620_e15405_d_n11;
        locals.var_t2_dn14 = assign20620_e15405_d_n14;

        let assign20630_e15408: f64 = if locals.var_vzadd < 1e-12 { 1.0 } else { 0.0 };
        locals.var_guard416 = assign20630_e15408;

        let (assign20640_e15416, assign20640_e15416_d_n0, assign20640_e15416_d_n2, assign20640_e15416_d_n4, assign20640_e15416_d_n5, assign20640_e15416_d_n6, assign20640_e15416_d_n7, assign20640_e15416_d_n8, assign20640_e15416_d_n9, assign20640_e15416_d_n10, assign20640_e15416_d_n11, assign20640_e15416_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard416 != 0.0)) {
        (1e-12, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vzadd, locals.var_vzadd_dn0, locals.var_vzadd_dn2, locals.var_vzadd_dn4, locals.var_vzadd_dn5, locals.var_vzadd_dn6, locals.var_vzadd_dn7, locals.var_vzadd_dn8, locals.var_vzadd_dn9, locals.var_vzadd_dn10, locals.var_vzadd_dn11, locals.var_vzadd_dn14,)
    }
};
        locals.var_vzadd = assign20640_e15416;
        locals.var_vzadd_dn0 = assign20640_e15416_d_n0;
        locals.var_vzadd_dn2 = assign20640_e15416_d_n2;
        locals.var_vzadd_dn4 = assign20640_e15416_d_n4;
        locals.var_vzadd_dn5 = assign20640_e15416_d_n5;
        locals.var_vzadd_dn6 = assign20640_e15416_d_n6;
        locals.var_vzadd_dn7 = assign20640_e15416_d_n7;
        locals.var_vzadd_dn8 = assign20640_e15416_d_n8;
        locals.var_vzadd_dn9 = assign20640_e15416_d_n9;
        locals.var_vzadd_dn10 = assign20640_e15416_d_n10;
        locals.var_vzadd_dn11 = assign20640_e15416_d_n11;
        locals.var_vzadd_dn14 = assign20640_e15416_d_n14;

        let (assign20650_e15426, assign20650_e15426_d_n0, assign20650_e15426_d_n2, assign20650_e15426_d_n4, assign20650_e15426_d_n5, assign20650_e15426_d_n6, assign20650_e15426_d_n7, assign20650_e15426_d_n8, assign20650_e15426_d_n9, assign20650_e15426_d_n10, assign20650_e15426_d_n11, assign20650_e15426_d_n14,) = {
    if ((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) {
        let assign20650_e15423: f64 = (2.0 * locals.var_vzadd);
        let assign20650_e15424: f64 = (locals.var_vdserev + assign20650_e15423);
        (assign20650_e15424, (locals.var_vdserev_dn0 + (2.0 * locals.var_vzadd_dn0)), (locals.var_vdserev_dn2 + (2.0 * locals.var_vzadd_dn2)), (2.0 * locals.var_vzadd_dn4), (2.0 * locals.var_vzadd_dn5), (2.0 * locals.var_vzadd_dn6), (2.0 * locals.var_vzadd_dn7), (2.0 * locals.var_vzadd_dn8), (2.0 * locals.var_vzadd_dn9), (2.0 * locals.var_vzadd_dn10), (2.0 * locals.var_vzadd_dn11), (2.0 * locals.var_vzadd_dn14),)
    } else {
        (locals.var_vdserevz, locals.var_vdserevz_dn0, locals.var_vdserevz_dn2, locals.var_vdserevz_dn4, locals.var_vdserevz_dn5, locals.var_vdserevz_dn6, locals.var_vdserevz_dn7, locals.var_vdserevz_dn8, locals.var_vdserevz_dn9, locals.var_vdserevz_dn10, locals.var_vdserevz_dn11, locals.var_vdserevz_dn14,)
    }
};
        locals.var_vdserevz = assign20650_e15426;
        locals.var_vdserevz_dn0 = assign20650_e15426_d_n0;
        locals.var_vdserevz_dn2 = assign20650_e15426_d_n2;
        locals.var_vdserevz_dn4 = assign20650_e15426_d_n4;
        locals.var_vdserevz_dn5 = assign20650_e15426_d_n5;
        locals.var_vdserevz_dn6 = assign20650_e15426_d_n6;
        locals.var_vdserevz_dn7 = assign20650_e15426_d_n7;
        locals.var_vdserevz_dn8 = assign20650_e15426_d_n8;
        locals.var_vdserevz_dn9 = assign20650_e15426_d_n9;
        locals.var_vdserevz_dn10 = assign20650_e15426_d_n10;
        locals.var_vdserevz_dn11 = assign20650_e15426_d_n11;
        locals.var_vdserevz_dn14 = assign20650_e15426_d_n14;

        let (assign20660_e15434, assign20660_e15434_d_n0, assign20660_e15434_d_n2, assign20660_e15434_d_n4, assign20660_e15434_d_n5, assign20660_e15434_d_n6, assign20660_e15434_d_n7, assign20660_e15434_d_n8, assign20660_e15434_d_n9, assign20660_e15434_d_n10, assign20660_e15434_d_n11, assign20660_e15434_d_n14,) = {
    if ((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) {
        let assign20660_e15432: f64 = (locals.var_vgserev + locals.var_vzadd);
        (assign20660_e15432, (locals.var_vgserev_dn0 + locals.var_vzadd_dn0), (locals.var_vgserev_dn2 + locals.var_vzadd_dn2), locals.var_vzadd_dn4, locals.var_vzadd_dn5, locals.var_vzadd_dn6, (locals.var_vgserev_dn7 + locals.var_vzadd_dn7), locals.var_vzadd_dn8, locals.var_vzadd_dn9, locals.var_vzadd_dn10, locals.var_vzadd_dn11, locals.var_vzadd_dn14,)
    } else {
        (locals.var_vgserevz, locals.var_vgserevz_dn0, locals.var_vgserevz_dn2, locals.var_vgserevz_dn4, locals.var_vgserevz_dn5, locals.var_vgserevz_dn6, locals.var_vgserevz_dn7, locals.var_vgserevz_dn8, locals.var_vgserevz_dn9, locals.var_vgserevz_dn10, locals.var_vgserevz_dn11, locals.var_vgserevz_dn14,)
    }
};
        locals.var_vgserevz = assign20660_e15434;
        locals.var_vgserevz_dn0 = assign20660_e15434_d_n0;
        locals.var_vgserevz_dn2 = assign20660_e15434_d_n2;
        locals.var_vgserevz_dn4 = assign20660_e15434_d_n4;
        locals.var_vgserevz_dn5 = assign20660_e15434_d_n5;
        locals.var_vgserevz_dn6 = assign20660_e15434_d_n6;
        locals.var_vgserevz_dn7 = assign20660_e15434_d_n7;
        locals.var_vgserevz_dn8 = assign20660_e15434_d_n8;
        locals.var_vgserevz_dn9 = assign20660_e15434_d_n9;
        locals.var_vgserevz_dn10 = assign20660_e15434_d_n10;
        locals.var_vgserevz_dn11 = assign20660_e15434_d_n11;
        locals.var_vgserevz_dn14 = assign20660_e15434_d_n14;

        let (assign20670_e15442, assign20670_e15442_d_n0, assign20670_e15442_d_n2, assign20670_e15442_d_n4, assign20670_e15442_d_n5, assign20670_e15442_d_n6, assign20670_e15442_d_n7, assign20670_e15442_d_n8, assign20670_e15442_d_n9, assign20670_e15442_d_n10, assign20670_e15442_d_n11, assign20670_e15442_d_n14,) = {
    if ((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) {
        let assign20670_e15440: f64 = (locals.var_vbserev + locals.var_vzadd);
        (assign20670_e15440, (locals.var_vbserev_dn0 + locals.var_vzadd_dn0), (locals.var_vbserev_dn2 + locals.var_vzadd_dn2), locals.var_vzadd_dn4, locals.var_vzadd_dn5, locals.var_vzadd_dn6, locals.var_vzadd_dn7, locals.var_vzadd_dn8, (locals.var_vbserev_dn9 + locals.var_vzadd_dn9), locals.var_vzadd_dn10, locals.var_vzadd_dn11, locals.var_vzadd_dn14,)
    } else {
        (locals.var_vbserevz, locals.var_vbserevz_dn0, locals.var_vbserevz_dn2, locals.var_vbserevz_dn4, locals.var_vbserevz_dn5, locals.var_vbserevz_dn6, locals.var_vbserevz_dn7, locals.var_vbserevz_dn8, locals.var_vbserevz_dn9, locals.var_vbserevz_dn10, locals.var_vbserevz_dn11, locals.var_vbserevz_dn14,)
    }
};
        locals.var_vbserevz = assign20670_e15442;
        locals.var_vbserevz_dn0 = assign20670_e15442_d_n0;
        locals.var_vbserevz_dn2 = assign20670_e15442_d_n2;
        locals.var_vbserevz_dn4 = assign20670_e15442_d_n4;
        locals.var_vbserevz_dn5 = assign20670_e15442_d_n5;
        locals.var_vbserevz_dn6 = assign20670_e15442_d_n6;
        locals.var_vbserevz_dn7 = assign20670_e15442_d_n7;
        locals.var_vbserevz_dn8 = assign20670_e15442_d_n8;
        locals.var_vbserevz_dn9 = assign20670_e15442_d_n9;
        locals.var_vbserevz_dn10 = assign20670_e15442_d_n10;
        locals.var_vbserevz_dn11 = assign20670_e15442_d_n11;
        locals.var_vbserevz_dn14 = assign20670_e15442_d_n14;

        let assign20680_e15449: f64 = if ((p.p34 == 1.0) || (locals.var_vdsemodenml == 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard417 = assign20680_e15449;

        let (assign20690_e15463, assign20690_e15463_d_n0, assign20690_e15463_d_n2, assign20690_e15463_d_n4, assign20690_e15463_d_n5, assign20690_e15463_d_n6, assign20690_e15463_d_n7, assign20690_e15463_d_n8, assign20690_e15463_d_n9, assign20690_e15463_d_n10, assign20690_e15463_d_n11, assign20690_e15463_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard417 != 0.0)) {
        let assign20690_e15457: f64 = (locals.var_vdsemodenml * locals.var_rde);
        let assign20690_e15460: f64 = (locals.var_vdsemodervs * locals.var_rse);
        let assign20690_e15461: f64 = (assign20690_e15457 + assign20690_e15460);
        (assign20690_e15461, ((locals.var_vdsemodenml * locals.var_rde_dn0) + (locals.var_vdsemodervs * locals.var_rse_dn0)), ((locals.var_vdsemodenml * locals.var_rde_dn2) + (locals.var_vdsemodervs * locals.var_rse_dn2)), ((locals.var_vdsemodenml * locals.var_rde_dn4) + (locals.var_vdsemodervs * locals.var_rse_dn4)), ((locals.var_vdsemodenml * locals.var_rde_dn5) + (locals.var_vdsemodervs * locals.var_rse_dn5)), ((locals.var_vdsemodenml * locals.var_rde_dn6) + (locals.var_vdsemodervs * locals.var_rse_dn6)), ((locals.var_vdsemodenml * locals.var_rde_dn7) + (locals.var_vdsemodervs * locals.var_rse_dn7)), ((locals.var_vdsemodenml * locals.var_rde_dn8) + (locals.var_vdsemodervs * locals.var_rse_dn8)), ((locals.var_vdsemodenml * locals.var_rde_dn9) + (locals.var_vdsemodervs * locals.var_rse_dn9)), ((locals.var_vdsemodenml * locals.var_rde_dn10) + (locals.var_vdsemodervs * locals.var_rse_dn10)), ((locals.var_vdsemodenml * locals.var_rde_dn11) + (locals.var_vdsemodervs * locals.var_rse_dn11)), ((locals.var_vdsemodenml * locals.var_rde_dn14) + (locals.var_vdsemodervs * locals.var_rse_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign20690_e15463;
        locals.var_t1_dn0 = assign20690_e15463_d_n0;
        locals.var_t1_dn2 = assign20690_e15463_d_n2;
        locals.var_t1_dn4 = assign20690_e15463_d_n4;
        locals.var_t1_dn5 = assign20690_e15463_d_n5;
        locals.var_t1_dn6 = assign20690_e15463_d_n6;
        locals.var_t1_dn7 = assign20690_e15463_d_n7;
        locals.var_t1_dn8 = assign20690_e15463_d_n8;
        locals.var_t1_dn9 = assign20690_e15463_d_n9;
        locals.var_t1_dn10 = assign20690_e15463_d_n10;
        locals.var_t1_dn11 = assign20690_e15463_d_n11;
        locals.var_t1_dn14 = assign20690_e15463_d_n14;

        let (assign20700_e15477, assign20700_e15477_d_n0, assign20700_e15477_d_n2, assign20700_e15477_d_n4, assign20700_e15477_d_n5, assign20700_e15477_d_n6, assign20700_e15477_d_n7, assign20700_e15477_d_n8, assign20700_e15477_d_n9, assign20700_e15477_d_n10, assign20700_e15477_d_n11, assign20700_e15477_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard417 != 0.0)) {
        let assign20700_e15471: f64 = (locals.var_vdsemodenml * locals.var_rdvde);
        let assign20700_e15474: f64 = (locals.var_vdsemodervs * locals.var_rsvde);
        let assign20700_e15475: f64 = (assign20700_e15471 + assign20700_e15474);
        (assign20700_e15475, ((locals.var_vdsemodenml * locals.var_rdvde_dn0) + (locals.var_vdsemodervs * locals.var_rsvde_dn0)), ((locals.var_vdsemodenml * locals.var_rdvde_dn2) + (locals.var_vdsemodervs * locals.var_rsvde_dn2)), ((locals.var_vdsemodenml * locals.var_rdvde_dn4) + (locals.var_vdsemodervs * locals.var_rsvde_dn4)), ((locals.var_vdsemodenml * locals.var_rdvde_dn5) + (locals.var_vdsemodervs * locals.var_rsvde_dn5)), ((locals.var_vdsemodenml * locals.var_rdvde_dn6) + (locals.var_vdsemodervs * locals.var_rsvde_dn6)), ((locals.var_vdsemodenml * locals.var_rdvde_dn7) + (locals.var_vdsemodervs * locals.var_rsvde_dn7)), ((locals.var_vdsemodenml * locals.var_rdvde_dn8) + (locals.var_vdsemodervs * locals.var_rsvde_dn8)), ((locals.var_vdsemodenml * locals.var_rdvde_dn9) + (locals.var_vdsemodervs * locals.var_rsvde_dn9)), ((locals.var_vdsemodenml * locals.var_rdvde_dn10) + (locals.var_vdsemodervs * locals.var_rsvde_dn10)), ((locals.var_vdsemodenml * locals.var_rdvde_dn11) + (locals.var_vdsemodervs * locals.var_rsvde_dn11)), ((locals.var_vdsemodenml * locals.var_rdvde_dn14) + (locals.var_vdsemodervs * locals.var_rsvde_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign20700_e15477;
        locals.var_t0_dn0 = assign20700_e15477_d_n0;
        locals.var_t0_dn2 = assign20700_e15477_d_n2;
        locals.var_t0_dn4 = assign20700_e15477_d_n4;
        locals.var_t0_dn5 = assign20700_e15477_d_n5;
        locals.var_t0_dn6 = assign20700_e15477_d_n6;
        locals.var_t0_dn7 = assign20700_e15477_d_n7;
        locals.var_t0_dn8 = assign20700_e15477_d_n8;
        locals.var_t0_dn9 = assign20700_e15477_d_n9;
        locals.var_t0_dn10 = assign20700_e15477_d_n10;
        locals.var_t0_dn11 = assign20700_e15477_d_n11;
        locals.var_t0_dn14 = assign20700_e15477_d_n14;

        let (assign20710_e15489, assign20710_e15489_d_n0, assign20710_e15489_d_n2, assign20710_e15489_d_n4, assign20710_e15489_d_n5, assign20710_e15489_d_n6, assign20710_e15489_d_n7, assign20710_e15489_d_n8, assign20710_e15489_d_n9, assign20710_e15489_d_n10, assign20710_e15489_d_n11, assign20710_e15489_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard417 != 0.0)) {
        let assign20710_e15486: f64 = (locals.var_t0 * locals.var_vdserevz);
        let assign20710_e15487: f64 = (locals.var_t1 + assign20710_e15486);
        (assign20710_e15487, (locals.var_t1_dn0 + ((locals.var_t0_dn0 * locals.var_vdserevz) + (locals.var_t0 * locals.var_vdserevz_dn0))), (locals.var_t1_dn2 + ((locals.var_t0_dn2 * locals.var_vdserevz) + (locals.var_t0 * locals.var_vdserevz_dn2))), (locals.var_t1_dn4 + ((locals.var_t0_dn4 * locals.var_vdserevz) + (locals.var_t0 * locals.var_vdserevz_dn4))), (locals.var_t1_dn5 + ((locals.var_t0_dn5 * locals.var_vdserevz) + (locals.var_t0 * locals.var_vdserevz_dn5))), (locals.var_t1_dn6 + ((locals.var_t0_dn6 * locals.var_vdserevz) + (locals.var_t0 * locals.var_vdserevz_dn6))), (locals.var_t1_dn7 + ((locals.var_t0_dn7 * locals.var_vdserevz) + (locals.var_t0 * locals.var_vdserevz_dn7))), (locals.var_t1_dn8 + ((locals.var_t0_dn8 * locals.var_vdserevz) + (locals.var_t0 * locals.var_vdserevz_dn8))), (locals.var_t1_dn9 + ((locals.var_t0_dn9 * locals.var_vdserevz) + (locals.var_t0 * locals.var_vdserevz_dn9))), (locals.var_t1_dn10 + ((locals.var_t0_dn10 * locals.var_vdserevz) + (locals.var_t0 * locals.var_vdserevz_dn10))), (locals.var_t1_dn11 + ((locals.var_t0_dn11 * locals.var_vdserevz) + (locals.var_t0 * locals.var_vdserevz_dn11))), (locals.var_t1_dn14 + ((locals.var_t0_dn14 * locals.var_vdserevz) + (locals.var_t0 * locals.var_vdserevz_dn14))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign20710_e15489;
        locals.var_t4_dn0 = assign20710_e15489_d_n0;
        locals.var_t4_dn2 = assign20710_e15489_d_n2;
        locals.var_t4_dn4 = assign20710_e15489_d_n4;
        locals.var_t4_dn5 = assign20710_e15489_d_n5;
        locals.var_t4_dn6 = assign20710_e15489_d_n6;
        locals.var_t4_dn7 = assign20710_e15489_d_n7;
        locals.var_t4_dn8 = assign20710_e15489_d_n8;
        locals.var_t4_dn9 = assign20710_e15489_d_n9;
        locals.var_t4_dn10 = assign20710_e15489_d_n10;
        locals.var_t4_dn11 = assign20710_e15489_d_n11;
        locals.var_t4_dn14 = assign20710_e15489_d_n14;

        let (assign20720_e15510, assign20720_e15510_d_n0, assign20720_e15510_d_n2, assign20720_e15510_d_n4, assign20720_e15510_d_n5, assign20720_e15510_d_n6, assign20720_e15510_d_n7, assign20720_e15510_d_n8, assign20720_e15510_d_n9, assign20720_e15510_d_n10, assign20720_e15510_d_n11, assign20720_e15510_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard417 != 0.0)) {
        let assign20720_e15497: f64 = (p.p292 * p.p292);
        let assign20720_e15501: f64 = (0.0001 * 0.01);
        let assign20720_e15502: f64 = (4.0 * assign20720_e15501);
        let assign20720_e15505: f64 = (0.0001 * 0.01);
        let assign20720_e15506: f64 = (assign20720_e15502 * assign20720_e15505);
        let assign20720_e15507: f64 = (assign20720_e15497 + assign20720_e15506);
        let assign20720_e15508: f64 = (assign20720_e15507).sqrt();
        (assign20720_e15508, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign20720_e15510;
        locals.var_tmf2_dn0 = assign20720_e15510_d_n0;
        locals.var_tmf2_dn2 = assign20720_e15510_d_n2;
        locals.var_tmf2_dn4 = assign20720_e15510_d_n4;
        locals.var_tmf2_dn5 = assign20720_e15510_d_n5;
        locals.var_tmf2_dn6 = assign20720_e15510_d_n6;
        locals.var_tmf2_dn7 = assign20720_e15510_d_n7;
        locals.var_tmf2_dn8 = assign20720_e15510_d_n8;
        locals.var_tmf2_dn9 = assign20720_e15510_d_n9;
        locals.var_tmf2_dn10 = assign20720_e15510_d_n10;
        locals.var_tmf2_dn11 = assign20720_e15510_d_n11;
        locals.var_tmf2_dn14 = assign20720_e15510_d_n14;

        let (assign20730_e15524, assign20730_e15524_d_n0, assign20730_e15524_d_n2, assign20730_e15524_d_n4, assign20730_e15524_d_n5, assign20730_e15524_d_n6, assign20730_e15524_d_n7, assign20730_e15524_d_n8, assign20730_e15524_d_n9, assign20730_e15524_d_n10, assign20730_e15524_d_n11, assign20730_e15524_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard417 != 0.0)) {
        let assign20730_e15520: f64 = (p.p292 / locals.var_tmf2);
        let assign20730_e15521: f64 = (1.0 + assign20730_e15520);
        let assign20730_e15522: f64 = (0.5 * assign20730_e15521);
        (assign20730_e15522, (0.5 * (-((p.p292 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p292 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p292 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p292 * locals.var_tmf2_dn5) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p292 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p292 * locals.var_tmf2_dn7) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p292 * locals.var_tmf2_dn8) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p292 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p292 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p292 * locals.var_tmf2_dn11) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p292 * locals.var_tmf2_dn14) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign20730_e15524;
        locals.var_t0_dn0 = assign20730_e15524_d_n0;
        locals.var_t0_dn2 = assign20730_e15524_d_n2;
        locals.var_t0_dn4 = assign20730_e15524_d_n4;
        locals.var_t0_dn5 = assign20730_e15524_d_n5;
        locals.var_t0_dn6 = assign20730_e15524_d_n6;
        locals.var_t0_dn7 = assign20730_e15524_d_n7;
        locals.var_t0_dn8 = assign20730_e15524_d_n8;
        locals.var_t0_dn9 = assign20730_e15524_d_n9;
        locals.var_t0_dn10 = assign20730_e15524_d_n10;
        locals.var_t0_dn11 = assign20730_e15524_d_n11;
        locals.var_t0_dn14 = assign20730_e15524_d_n14;

        let (assign20740_e15536, assign20740_e15536_d_n0, assign20740_e15536_d_n2, assign20740_e15536_d_n4, assign20740_e15536_d_n5, assign20740_e15536_d_n6, assign20740_e15536_d_n7, assign20740_e15536_d_n8, assign20740_e15536_d_n9, assign20740_e15536_d_n10, assign20740_e15536_d_n11, assign20740_e15536_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard417 != 0.0)) {
        let assign20740_e15533: f64 = (p.p292 + locals.var_tmf2);
        let assign20740_e15534: f64 = (0.5 * assign20740_e15533);
        (assign20740_e15534, (0.5 * locals.var_tmf2_dn0), (0.5 * locals.var_tmf2_dn2), (0.5 * locals.var_tmf2_dn4), (0.5 * locals.var_tmf2_dn5), (0.5 * locals.var_tmf2_dn6), (0.5 * locals.var_tmf2_dn7), (0.5 * locals.var_tmf2_dn8), (0.5 * locals.var_tmf2_dn9), (0.5 * locals.var_tmf2_dn10), (0.5 * locals.var_tmf2_dn11), (0.5 * locals.var_tmf2_dn14),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign20740_e15536;
        locals.var_t10_dn0 = assign20740_e15536_d_n0;
        locals.var_t10_dn2 = assign20740_e15536_d_n2;
        locals.var_t10_dn4 = assign20740_e15536_d_n4;
        locals.var_t10_dn5 = assign20740_e15536_d_n5;
        locals.var_t10_dn6 = assign20740_e15536_d_n6;
        locals.var_t10_dn7 = assign20740_e15536_d_n7;
        locals.var_t10_dn8 = assign20740_e15536_d_n8;
        locals.var_t10_dn9 = assign20740_e15536_d_n9;
        locals.var_t10_dn10 = assign20740_e15536_d_n10;
        locals.var_t10_dn11 = assign20740_e15536_d_n11;
        locals.var_t10_dn14 = assign20740_e15536_d_n14;

        let assign20750_e15539: f64 = if locals.var_t10 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard418 = assign20750_e15539;

        let (assign20760_e15549, assign20760_e15549_d_n0, assign20760_e15549_d_n2, assign20760_e15549_d_n4, assign20760_e15549_d_n5, assign20760_e15549_d_n6, assign20760_e15549_d_n7, assign20760_e15549_d_n8, assign20760_e15549_d_n9, assign20760_e15549_d_n10, assign20760_e15549_d_n11, assign20760_e15549_d_n14,) = {
    if ((((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard417 != 0.0)) && (locals.var_guard418 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign20760_e15549;
        locals.var_t10_dn0 = assign20760_e15549_d_n0;
        locals.var_t10_dn2 = assign20760_e15549_d_n2;
        locals.var_t10_dn4 = assign20760_e15549_d_n4;
        locals.var_t10_dn5 = assign20760_e15549_d_n5;
        locals.var_t10_dn6 = assign20760_e15549_d_n6;
        locals.var_t10_dn7 = assign20760_e15549_d_n7;
        locals.var_t10_dn8 = assign20760_e15549_d_n8;
        locals.var_t10_dn9 = assign20760_e15549_d_n9;
        locals.var_t10_dn10 = assign20760_e15549_d_n10;
        locals.var_t10_dn11 = assign20760_e15549_d_n11;
        locals.var_t10_dn14 = assign20760_e15549_d_n14;

        let (assign20770_e15559, assign20770_e15559_d_n0, assign20770_e15559_d_n2, assign20770_e15559_d_n4, assign20770_e15559_d_n5, assign20770_e15559_d_n6, assign20770_e15559_d_n7, assign20770_e15559_d_n8, assign20770_e15559_d_n9, assign20770_e15559_d_n10, assign20770_e15559_d_n11, assign20770_e15559_d_n14,) = {
    if ((((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard417 != 0.0)) && (locals.var_guard418 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign20770_e15559;
        locals.var_t0_dn0 = assign20770_e15559_d_n0;
        locals.var_t0_dn2 = assign20770_e15559_d_n2;
        locals.var_t0_dn4 = assign20770_e15559_d_n4;
        locals.var_t0_dn5 = assign20770_e15559_d_n5;
        locals.var_t0_dn6 = assign20770_e15559_d_n6;
        locals.var_t0_dn7 = assign20770_e15559_d_n7;
        locals.var_t0_dn8 = assign20770_e15559_d_n8;
        locals.var_t0_dn9 = assign20770_e15559_d_n9;
        locals.var_t0_dn10 = assign20770_e15559_d_n10;
        locals.var_t0_dn11 = assign20770_e15559_d_n11;
        locals.var_t0_dn14 = assign20770_e15559_d_n14;

        let (assign20780_e15577, assign20780_e15577_d_n0, assign20780_e15577_d_n2, assign20780_e15577_d_n4, assign20780_e15577_d_n5, assign20780_e15577_d_n6, assign20780_e15577_d_n7, assign20780_e15577_d_n8, assign20780_e15577_d_n9, assign20780_e15577_d_n10, assign20780_e15577_d_n11, assign20780_e15577_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard417 != 0.0)) {
        let assign20780_e15571: f64 = (locals.var_vgserevz / locals.var_t10);
        let assign20780_e15572: f64 = (1.0 - assign20780_e15571);
        let assign20780_e15573: f64 = (locals.var_uc_rdvg11 * assign20780_e15572);
        let assign20780_e15574: f64 = (1.0 + assign20780_e15573);
        let assign20780_e15575: f64 = (locals.var_t4 * assign20780_e15574);
        (assign20780_e15575, ((locals.var_t4_dn0 * assign20780_e15574) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn0 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn0)) / (locals.var_t10 * locals.var_t10)))))), ((locals.var_t4_dn2 * assign20780_e15574) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn2 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn2)) / (locals.var_t10 * locals.var_t10)))))), ((locals.var_t4_dn4 * assign20780_e15574) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn4 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn4)) / (locals.var_t10 * locals.var_t10)))))), ((locals.var_t4_dn5 * assign20780_e15574) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn5 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn5)) / (locals.var_t10 * locals.var_t10)))))), ((locals.var_t4_dn6 * assign20780_e15574) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn6 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn6)) / (locals.var_t10 * locals.var_t10)))))), ((locals.var_t4_dn7 * assign20780_e15574) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn7 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn7)) / (locals.var_t10 * locals.var_t10)))))), ((locals.var_t4_dn8 * assign20780_e15574) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn8 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn8)) / (locals.var_t10 * locals.var_t10)))))), ((locals.var_t4_dn9 * assign20780_e15574) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn9 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn9)) / (locals.var_t10 * locals.var_t10)))))), ((locals.var_t4_dn10 * assign20780_e15574) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn10 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn10)) / (locals.var_t10 * locals.var_t10)))))), ((locals.var_t4_dn11 * assign20780_e15574) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn11 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn11)) / (locals.var_t10 * locals.var_t10)))))), ((locals.var_t4_dn14 * assign20780_e15574) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn14 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn14)) / (locals.var_t10 * locals.var_t10)))))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign20780_e15577;
        locals.var_t1_dn0 = assign20780_e15577_d_n0;
        locals.var_t1_dn2 = assign20780_e15577_d_n2;
        locals.var_t1_dn4 = assign20780_e15577_d_n4;
        locals.var_t1_dn5 = assign20780_e15577_d_n5;
        locals.var_t1_dn6 = assign20780_e15577_d_n6;
        locals.var_t1_dn7 = assign20780_e15577_d_n7;
        locals.var_t1_dn8 = assign20780_e15577_d_n8;
        locals.var_t1_dn9 = assign20780_e15577_d_n9;
        locals.var_t1_dn10 = assign20780_e15577_d_n10;
        locals.var_t1_dn11 = assign20780_e15577_d_n11;
        locals.var_t1_dn14 = assign20780_e15577_d_n14;

        let (assign20790_e15591, assign20790_e15591_d_n0, assign20790_e15591_d_n2, assign20790_e15591_d_n4, assign20790_e15591_d_n5, assign20790_e15591_d_n6, assign20790_e15591_d_n7, assign20790_e15591_d_n8, assign20790_e15591_d_n9, assign20790_e15591_d_n10, assign20790_e15591_d_n11, assign20790_e15591_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard417 != 0.0)) {
        let assign20790_e15585: f64 = (locals.var_t1 - locals.var_t4);
        let assign20790_e15588: f64 = (0.01 * 0.01);
        let assign20790_e15589: f64 = (assign20790_e15585 - assign20790_e15588);
        (assign20790_e15589, (locals.var_t1_dn0 - locals.var_t4_dn0), (locals.var_t1_dn2 - locals.var_t4_dn2), (locals.var_t1_dn4 - locals.var_t4_dn4), (locals.var_t1_dn5 - locals.var_t4_dn5), (locals.var_t1_dn6 - locals.var_t4_dn6), (locals.var_t1_dn7 - locals.var_t4_dn7), (locals.var_t1_dn8 - locals.var_t4_dn8), (locals.var_t1_dn9 - locals.var_t4_dn9), (locals.var_t1_dn10 - locals.var_t4_dn10), (locals.var_t1_dn11 - locals.var_t4_dn11), (locals.var_t1_dn14 - locals.var_t4_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign20790_e15591;
        locals.var_tmf1_dn0 = assign20790_e15591_d_n0;
        locals.var_tmf1_dn2 = assign20790_e15591_d_n2;
        locals.var_tmf1_dn4 = assign20790_e15591_d_n4;
        locals.var_tmf1_dn5 = assign20790_e15591_d_n5;
        locals.var_tmf1_dn6 = assign20790_e15591_d_n6;
        locals.var_tmf1_dn7 = assign20790_e15591_d_n7;
        locals.var_tmf1_dn8 = assign20790_e15591_d_n8;
        locals.var_tmf1_dn9 = assign20790_e15591_d_n9;
        locals.var_tmf1_dn10 = assign20790_e15591_d_n10;
        locals.var_tmf1_dn11 = assign20790_e15591_d_n11;
        locals.var_tmf1_dn14 = assign20790_e15591_d_n14;

        let (assign20800_e15605, assign20800_e15605_d_n0, assign20800_e15605_d_n2, assign20800_e15605_d_n4, assign20800_e15605_d_n5, assign20800_e15605_d_n6, assign20800_e15605_d_n7, assign20800_e15605_d_n8, assign20800_e15605_d_n9, assign20800_e15605_d_n10, assign20800_e15605_d_n11, assign20800_e15605_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard417 != 0.0)) {
        let assign20800_e15599: f64 = (4.0 * locals.var_t4);
        let assign20800_e15602: f64 = (0.01 * 0.01);
        let assign20800_e15603: f64 = (assign20800_e15599 * assign20800_e15602);
        (assign20800_e15603, ((4.0 * locals.var_t4_dn0) * assign20800_e15602), ((4.0 * locals.var_t4_dn2) * assign20800_e15602), ((4.0 * locals.var_t4_dn4) * assign20800_e15602), ((4.0 * locals.var_t4_dn5) * assign20800_e15602), ((4.0 * locals.var_t4_dn6) * assign20800_e15602), ((4.0 * locals.var_t4_dn7) * assign20800_e15602), ((4.0 * locals.var_t4_dn8) * assign20800_e15602), ((4.0 * locals.var_t4_dn9) * assign20800_e15602), ((4.0 * locals.var_t4_dn10) * assign20800_e15602), ((4.0 * locals.var_t4_dn11) * assign20800_e15602), ((4.0 * locals.var_t4_dn14) * assign20800_e15602),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign20800_e15605;
        locals.var_tmf2_dn0 = assign20800_e15605_d_n0;
        locals.var_tmf2_dn2 = assign20800_e15605_d_n2;
        locals.var_tmf2_dn4 = assign20800_e15605_d_n4;
        locals.var_tmf2_dn5 = assign20800_e15605_d_n5;
        locals.var_tmf2_dn6 = assign20800_e15605_d_n6;
        locals.var_tmf2_dn7 = assign20800_e15605_d_n7;
        locals.var_tmf2_dn8 = assign20800_e15605_d_n8;
        locals.var_tmf2_dn9 = assign20800_e15605_d_n9;
        locals.var_tmf2_dn10 = assign20800_e15605_d_n10;
        locals.var_tmf2_dn11 = assign20800_e15605_d_n11;
        locals.var_tmf2_dn14 = assign20800_e15605_d_n14;

        let (assign20810_e15619, assign20810_e15619_d_n0, assign20810_e15619_d_n2, assign20810_e15619_d_n4, assign20810_e15619_d_n5, assign20810_e15619_d_n6, assign20810_e15619_d_n7, assign20810_e15619_d_n8, assign20810_e15619_d_n9, assign20810_e15619_d_n10, assign20810_e15619_d_n11, assign20810_e15619_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard417 != 0.0)) {
        let (assign20810_e15617, assign20810_e15617_d_n0, assign20810_e15617_d_n2, assign20810_e15617_d_n4, assign20810_e15617_d_n5, assign20810_e15617_d_n6, assign20810_e15617_d_n7, assign20810_e15617_d_n8, assign20810_e15617_d_n9, assign20810_e15617_d_n10, assign20810_e15617_d_n11, assign20810_e15617_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign20810_e15616: f64 = (-locals.var_tmf2);
                (assign20810_e15616, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign20810_e15617, assign20810_e15617_d_n0, assign20810_e15617_d_n2, assign20810_e15617_d_n4, assign20810_e15617_d_n5, assign20810_e15617_d_n6, assign20810_e15617_d_n7, assign20810_e15617_d_n8, assign20810_e15617_d_n9, assign20810_e15617_d_n10, assign20810_e15617_d_n11, assign20810_e15617_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign20810_e15619;
        locals.var_tmf2_dn0 = assign20810_e15619_d_n0;
        locals.var_tmf2_dn2 = assign20810_e15619_d_n2;
        locals.var_tmf2_dn4 = assign20810_e15619_d_n4;
        locals.var_tmf2_dn5 = assign20810_e15619_d_n5;
        locals.var_tmf2_dn6 = assign20810_e15619_d_n6;
        locals.var_tmf2_dn7 = assign20810_e15619_d_n7;
        locals.var_tmf2_dn8 = assign20810_e15619_d_n8;
        locals.var_tmf2_dn9 = assign20810_e15619_d_n9;
        locals.var_tmf2_dn10 = assign20810_e15619_d_n10;
        locals.var_tmf2_dn11 = assign20810_e15619_d_n11;
        locals.var_tmf2_dn14 = assign20810_e15619_d_n14;

        let (assign20820_e15632, assign20820_e15632_d_n0, assign20820_e15632_d_n2, assign20820_e15632_d_n4, assign20820_e15632_d_n5, assign20820_e15632_d_n6, assign20820_e15632_d_n7, assign20820_e15632_d_n8, assign20820_e15632_d_n9, assign20820_e15632_d_n10, assign20820_e15632_d_n11, assign20820_e15632_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard417 != 0.0)) {
        let assign20820_e15627: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign20820_e15629: f64 = (assign20820_e15627 + locals.var_tmf2);
        let assign20820_e15630: f64 = (assign20820_e15629).sqrt();
        (assign20820_e15630, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign20820_e15630)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign20820_e15630)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign20820_e15630)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign20820_e15630)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign20820_e15630)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign20820_e15630)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign20820_e15630)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign20820_e15630)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign20820_e15630)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign20820_e15630)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign20820_e15630)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign20820_e15632;
        locals.var_tmf2_dn0 = assign20820_e15632_d_n0;
        locals.var_tmf2_dn2 = assign20820_e15632_d_n2;
        locals.var_tmf2_dn4 = assign20820_e15632_d_n4;
        locals.var_tmf2_dn5 = assign20820_e15632_d_n5;
        locals.var_tmf2_dn6 = assign20820_e15632_d_n6;
        locals.var_tmf2_dn7 = assign20820_e15632_d_n7;
        locals.var_tmf2_dn8 = assign20820_e15632_d_n8;
        locals.var_tmf2_dn9 = assign20820_e15632_d_n9;
        locals.var_tmf2_dn10 = assign20820_e15632_d_n10;
        locals.var_tmf2_dn11 = assign20820_e15632_d_n11;
        locals.var_tmf2_dn14 = assign20820_e15632_d_n14;

        let (assign20830_e15646, assign20830_e15646_d_n0, assign20830_e15646_d_n2, assign20830_e15646_d_n4, assign20830_e15646_d_n5, assign20830_e15646_d_n6, assign20830_e15646_d_n7, assign20830_e15646_d_n8, assign20830_e15646_d_n9, assign20830_e15646_d_n10, assign20830_e15646_d_n11, assign20830_e15646_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard417 != 0.0)) {
        let assign20830_e15642: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign20830_e15643: f64 = (1.0 + assign20830_e15642);
        let assign20830_e15644: f64 = (0.5 * assign20830_e15643);
        (assign20830_e15644, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign20830_e15646;
        locals.var_t0_dn0 = assign20830_e15646_d_n0;
        locals.var_t0_dn2 = assign20830_e15646_d_n2;
        locals.var_t0_dn4 = assign20830_e15646_d_n4;
        locals.var_t0_dn5 = assign20830_e15646_d_n5;
        locals.var_t0_dn6 = assign20830_e15646_d_n6;
        locals.var_t0_dn7 = assign20830_e15646_d_n7;
        locals.var_t0_dn8 = assign20830_e15646_d_n8;
        locals.var_t0_dn9 = assign20830_e15646_d_n9;
        locals.var_t0_dn10 = assign20830_e15646_d_n10;
        locals.var_t0_dn11 = assign20830_e15646_d_n11;
        locals.var_t0_dn14 = assign20830_e15646_d_n14;

        let (assign20840_e15666, assign20840_e15666_d_n0, assign20840_e15666_d_n2, assign20840_e15666_d_n4, assign20840_e15666_d_n5, assign20840_e15666_d_n6, assign20840_e15666_d_n7, assign20840_e15666_d_n8, assign20840_e15666_d_n9, assign20840_e15666_d_n10, assign20840_e15666_d_n11, assign20840_e15666_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard417 != 0.0)) {
        let assign20840_e15657: f64 = (2.0 * 0.01);
        let assign20840_e15659: f64 = (assign20840_e15657 * 0.01);
        let assign20840_e15660: f64 = (locals.var_tmf1 - assign20840_e15659);
        let assign20840_e15662: f64 = (assign20840_e15660 / locals.var_tmf2);
        let assign20840_e15663: f64 = (1.0 - assign20840_e15662);
        let assign20840_e15664: f64 = (0.5 * assign20840_e15663);
        (assign20840_e15664, (0.5 * (-(((locals.var_tmf1_dn0 * locals.var_tmf2) - (assign20840_e15660 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn2 * locals.var_tmf2) - (assign20840_e15660 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn4 * locals.var_tmf2) - (assign20840_e15660 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn5 * locals.var_tmf2) - (assign20840_e15660 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn6 * locals.var_tmf2) - (assign20840_e15660 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn7 * locals.var_tmf2) - (assign20840_e15660 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn8 * locals.var_tmf2) - (assign20840_e15660 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn9 * locals.var_tmf2) - (assign20840_e15660 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn10 * locals.var_tmf2) - (assign20840_e15660 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn11 * locals.var_tmf2) - (assign20840_e15660 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn14 * locals.var_tmf2) - (assign20840_e15660 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign20840_e15666;
        locals.var_t5_dn0 = assign20840_e15666_d_n0;
        locals.var_t5_dn2 = assign20840_e15666_d_n2;
        locals.var_t5_dn4 = assign20840_e15666_d_n4;
        locals.var_t5_dn5 = assign20840_e15666_d_n5;
        locals.var_t5_dn6 = assign20840_e15666_d_n6;
        locals.var_t5_dn7 = assign20840_e15666_d_n7;
        locals.var_t5_dn8 = assign20840_e15666_d_n8;
        locals.var_t5_dn9 = assign20840_e15666_d_n9;
        locals.var_t5_dn10 = assign20840_e15666_d_n10;
        locals.var_t5_dn11 = assign20840_e15666_d_n11;
        locals.var_t5_dn14 = assign20840_e15666_d_n14;

        let (assign20850_e15680, assign20850_e15680_d_n0, assign20850_e15680_d_n2, assign20850_e15680_d_n4, assign20850_e15680_d_n5, assign20850_e15680_d_n6, assign20850_e15680_d_n7, assign20850_e15680_d_n8, assign20850_e15680_d_n9, assign20850_e15680_d_n10, assign20850_e15680_d_n11, assign20850_e15680_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard417 != 0.0)) {
        let assign20850_e15676: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign20850_e15677: f64 = (0.5 * assign20850_e15676);
        let assign20850_e15678: f64 = (locals.var_t4 + assign20850_e15677);
        (assign20850_e15678, (locals.var_t4_dn0 + (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_t4_dn2 + (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_t4_dn4 + (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_t4_dn5 + (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_t4_dn6 + (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_t4_dn7 + (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_t4_dn8 + (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_t4_dn9 + (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (locals.var_t4_dn10 + (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_t4_dn11 + (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_t4_dn14 + (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign20850_e15680;
        locals.var_t2_dn0 = assign20850_e15680_d_n0;
        locals.var_t2_dn2 = assign20850_e15680_d_n2;
        locals.var_t2_dn4 = assign20850_e15680_d_n4;
        locals.var_t2_dn5 = assign20850_e15680_d_n5;
        locals.var_t2_dn6 = assign20850_e15680_d_n6;
        locals.var_t2_dn7 = assign20850_e15680_d_n7;
        locals.var_t2_dn8 = assign20850_e15680_d_n8;
        locals.var_t2_dn9 = assign20850_e15680_d_n9;
        locals.var_t2_dn10 = assign20850_e15680_d_n10;
        locals.var_t2_dn11 = assign20850_e15680_d_n11;
        locals.var_t2_dn14 = assign20850_e15680_d_n14;

    }

    pub(super) fn stamp_transient_block_50(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign20860_e15692, assign20860_e15692_d_n0, assign20860_e15692_d_n2, assign20860_e15692_d_n4, assign20860_e15692_d_n5, assign20860_e15692_d_n6, assign20860_e15692_d_n7, assign20860_e15692_d_n8, assign20860_e15692_d_n9, assign20860_e15692_d_n10, assign20860_e15692_d_n11, assign20860_e15692_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard417 != 0.0)) {
        let assign20860_e15689: f64 = (1.0 + locals.var_uc_rdvg11);
        let assign20860_e15690: f64 = (locals.var_t4 * assign20860_e15689);
        (assign20860_e15690, (locals.var_t4_dn0 * assign20860_e15689), (locals.var_t4_dn2 * assign20860_e15689), (locals.var_t4_dn4 * assign20860_e15689), (locals.var_t4_dn5 * assign20860_e15689), (locals.var_t4_dn6 * assign20860_e15689), (locals.var_t4_dn7 * assign20860_e15689), (locals.var_t4_dn8 * assign20860_e15689), (locals.var_t4_dn9 * assign20860_e15689), (locals.var_t4_dn10 * assign20860_e15689), (locals.var_t4_dn11 * assign20860_e15689), (locals.var_t4_dn14 * assign20860_e15689),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign20860_e15692;
        locals.var_t3_dn0 = assign20860_e15692_d_n0;
        locals.var_t3_dn2 = assign20860_e15692_d_n2;
        locals.var_t3_dn4 = assign20860_e15692_d_n4;
        locals.var_t3_dn5 = assign20860_e15692_d_n5;
        locals.var_t3_dn6 = assign20860_e15692_d_n6;
        locals.var_t3_dn7 = assign20860_e15692_d_n7;
        locals.var_t3_dn8 = assign20860_e15692_d_n8;
        locals.var_t3_dn9 = assign20860_e15692_d_n9;
        locals.var_t3_dn10 = assign20860_e15692_d_n10;
        locals.var_t3_dn11 = assign20860_e15692_d_n11;
        locals.var_t3_dn14 = assign20860_e15692_d_n14;

        let (assign20870_e15706, assign20870_e15706_d_n0, assign20870_e15706_d_n2, assign20870_e15706_d_n4, assign20870_e15706_d_n5, assign20870_e15706_d_n6, assign20870_e15706_d_n7, assign20870_e15706_d_n8, assign20870_e15706_d_n9, assign20870_e15706_d_n10, assign20870_e15706_d_n11, assign20870_e15706_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard417 != 0.0)) {
        let assign20870_e15700: f64 = (locals.var_t3 - locals.var_t2);
        let assign20870_e15703: f64 = (5e-5 * 0.01);
        let assign20870_e15704: f64 = (assign20870_e15700 - assign20870_e15703);
        (assign20870_e15704, (locals.var_t3_dn0 - locals.var_t2_dn0), (locals.var_t3_dn2 - locals.var_t2_dn2), (locals.var_t3_dn4 - locals.var_t2_dn4), (locals.var_t3_dn5 - locals.var_t2_dn5), (locals.var_t3_dn6 - locals.var_t2_dn6), (locals.var_t3_dn7 - locals.var_t2_dn7), (locals.var_t3_dn8 - locals.var_t2_dn8), (locals.var_t3_dn9 - locals.var_t2_dn9), (locals.var_t3_dn10 - locals.var_t2_dn10), (locals.var_t3_dn11 - locals.var_t2_dn11), (locals.var_t3_dn14 - locals.var_t2_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign20870_e15706;
        locals.var_tmf1_dn0 = assign20870_e15706_d_n0;
        locals.var_tmf1_dn2 = assign20870_e15706_d_n2;
        locals.var_tmf1_dn4 = assign20870_e15706_d_n4;
        locals.var_tmf1_dn5 = assign20870_e15706_d_n5;
        locals.var_tmf1_dn6 = assign20870_e15706_d_n6;
        locals.var_tmf1_dn7 = assign20870_e15706_d_n7;
        locals.var_tmf1_dn8 = assign20870_e15706_d_n8;
        locals.var_tmf1_dn9 = assign20870_e15706_d_n9;
        locals.var_tmf1_dn10 = assign20870_e15706_d_n10;
        locals.var_tmf1_dn11 = assign20870_e15706_d_n11;
        locals.var_tmf1_dn14 = assign20870_e15706_d_n14;

        let (assign20880_e15720, assign20880_e15720_d_n0, assign20880_e15720_d_n2, assign20880_e15720_d_n4, assign20880_e15720_d_n5, assign20880_e15720_d_n6, assign20880_e15720_d_n7, assign20880_e15720_d_n8, assign20880_e15720_d_n9, assign20880_e15720_d_n10, assign20880_e15720_d_n11, assign20880_e15720_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard417 != 0.0)) {
        let assign20880_e15714: f64 = (4.0 * locals.var_t3);
        let assign20880_e15717: f64 = (5e-5 * 0.01);
        let assign20880_e15718: f64 = (assign20880_e15714 * assign20880_e15717);
        (assign20880_e15718, ((4.0 * locals.var_t3_dn0) * assign20880_e15717), ((4.0 * locals.var_t3_dn2) * assign20880_e15717), ((4.0 * locals.var_t3_dn4) * assign20880_e15717), ((4.0 * locals.var_t3_dn5) * assign20880_e15717), ((4.0 * locals.var_t3_dn6) * assign20880_e15717), ((4.0 * locals.var_t3_dn7) * assign20880_e15717), ((4.0 * locals.var_t3_dn8) * assign20880_e15717), ((4.0 * locals.var_t3_dn9) * assign20880_e15717), ((4.0 * locals.var_t3_dn10) * assign20880_e15717), ((4.0 * locals.var_t3_dn11) * assign20880_e15717), ((4.0 * locals.var_t3_dn14) * assign20880_e15717),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign20880_e15720;
        locals.var_tmf2_dn0 = assign20880_e15720_d_n0;
        locals.var_tmf2_dn2 = assign20880_e15720_d_n2;
        locals.var_tmf2_dn4 = assign20880_e15720_d_n4;
        locals.var_tmf2_dn5 = assign20880_e15720_d_n5;
        locals.var_tmf2_dn6 = assign20880_e15720_d_n6;
        locals.var_tmf2_dn7 = assign20880_e15720_d_n7;
        locals.var_tmf2_dn8 = assign20880_e15720_d_n8;
        locals.var_tmf2_dn9 = assign20880_e15720_d_n9;
        locals.var_tmf2_dn10 = assign20880_e15720_d_n10;
        locals.var_tmf2_dn11 = assign20880_e15720_d_n11;
        locals.var_tmf2_dn14 = assign20880_e15720_d_n14;

        let (assign20890_e15734, assign20890_e15734_d_n0, assign20890_e15734_d_n2, assign20890_e15734_d_n4, assign20890_e15734_d_n5, assign20890_e15734_d_n6, assign20890_e15734_d_n7, assign20890_e15734_d_n8, assign20890_e15734_d_n9, assign20890_e15734_d_n10, assign20890_e15734_d_n11, assign20890_e15734_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard417 != 0.0)) {
        let (assign20890_e15732, assign20890_e15732_d_n0, assign20890_e15732_d_n2, assign20890_e15732_d_n4, assign20890_e15732_d_n5, assign20890_e15732_d_n6, assign20890_e15732_d_n7, assign20890_e15732_d_n8, assign20890_e15732_d_n9, assign20890_e15732_d_n10, assign20890_e15732_d_n11, assign20890_e15732_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign20890_e15731: f64 = (-locals.var_tmf2);
                (assign20890_e15731, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign20890_e15732, assign20890_e15732_d_n0, assign20890_e15732_d_n2, assign20890_e15732_d_n4, assign20890_e15732_d_n5, assign20890_e15732_d_n6, assign20890_e15732_d_n7, assign20890_e15732_d_n8, assign20890_e15732_d_n9, assign20890_e15732_d_n10, assign20890_e15732_d_n11, assign20890_e15732_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign20890_e15734;
        locals.var_tmf2_dn0 = assign20890_e15734_d_n0;
        locals.var_tmf2_dn2 = assign20890_e15734_d_n2;
        locals.var_tmf2_dn4 = assign20890_e15734_d_n4;
        locals.var_tmf2_dn5 = assign20890_e15734_d_n5;
        locals.var_tmf2_dn6 = assign20890_e15734_d_n6;
        locals.var_tmf2_dn7 = assign20890_e15734_d_n7;
        locals.var_tmf2_dn8 = assign20890_e15734_d_n8;
        locals.var_tmf2_dn9 = assign20890_e15734_d_n9;
        locals.var_tmf2_dn10 = assign20890_e15734_d_n10;
        locals.var_tmf2_dn11 = assign20890_e15734_d_n11;
        locals.var_tmf2_dn14 = assign20890_e15734_d_n14;

        let (assign20900_e15747, assign20900_e15747_d_n0, assign20900_e15747_d_n2, assign20900_e15747_d_n4, assign20900_e15747_d_n5, assign20900_e15747_d_n6, assign20900_e15747_d_n7, assign20900_e15747_d_n8, assign20900_e15747_d_n9, assign20900_e15747_d_n10, assign20900_e15747_d_n11, assign20900_e15747_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard417 != 0.0)) {
        let assign20900_e15742: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign20900_e15744: f64 = (assign20900_e15742 + locals.var_tmf2);
        let assign20900_e15745: f64 = (assign20900_e15744).sqrt();
        (assign20900_e15745, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign20900_e15745)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign20900_e15745)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign20900_e15745)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign20900_e15745)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign20900_e15745)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign20900_e15745)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign20900_e15745)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign20900_e15745)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign20900_e15745)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign20900_e15745)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign20900_e15745)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign20900_e15747;
        locals.var_tmf2_dn0 = assign20900_e15747_d_n0;
        locals.var_tmf2_dn2 = assign20900_e15747_d_n2;
        locals.var_tmf2_dn4 = assign20900_e15747_d_n4;
        locals.var_tmf2_dn5 = assign20900_e15747_d_n5;
        locals.var_tmf2_dn6 = assign20900_e15747_d_n6;
        locals.var_tmf2_dn7 = assign20900_e15747_d_n7;
        locals.var_tmf2_dn8 = assign20900_e15747_d_n8;
        locals.var_tmf2_dn9 = assign20900_e15747_d_n9;
        locals.var_tmf2_dn10 = assign20900_e15747_d_n10;
        locals.var_tmf2_dn11 = assign20900_e15747_d_n11;
        locals.var_tmf2_dn14 = assign20900_e15747_d_n14;

        let (assign20910_e15761, assign20910_e15761_d_n0, assign20910_e15761_d_n2, assign20910_e15761_d_n4, assign20910_e15761_d_n5, assign20910_e15761_d_n6, assign20910_e15761_d_n7, assign20910_e15761_d_n8, assign20910_e15761_d_n9, assign20910_e15761_d_n10, assign20910_e15761_d_n11, assign20910_e15761_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard417 != 0.0)) {
        let assign20910_e15757: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign20910_e15758: f64 = (1.0 + assign20910_e15757);
        let assign20910_e15759: f64 = (0.5 * assign20910_e15758);
        (assign20910_e15759, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign20910_e15761;
        locals.var_t0_dn0 = assign20910_e15761_d_n0;
        locals.var_t0_dn2 = assign20910_e15761_d_n2;
        locals.var_t0_dn4 = assign20910_e15761_d_n4;
        locals.var_t0_dn5 = assign20910_e15761_d_n5;
        locals.var_t0_dn6 = assign20910_e15761_d_n6;
        locals.var_t0_dn7 = assign20910_e15761_d_n7;
        locals.var_t0_dn8 = assign20910_e15761_d_n8;
        locals.var_t0_dn9 = assign20910_e15761_d_n9;
        locals.var_t0_dn10 = assign20910_e15761_d_n10;
        locals.var_t0_dn11 = assign20910_e15761_d_n11;
        locals.var_t0_dn14 = assign20910_e15761_d_n14;

        let (assign20920_e15781, assign20920_e15781_d_n0, assign20920_e15781_d_n2, assign20920_e15781_d_n4, assign20920_e15781_d_n5, assign20920_e15781_d_n6, assign20920_e15781_d_n7, assign20920_e15781_d_n8, assign20920_e15781_d_n9, assign20920_e15781_d_n10, assign20920_e15781_d_n11, assign20920_e15781_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard417 != 0.0)) {
        let assign20920_e15772: f64 = (2.0 * 5e-5);
        let assign20920_e15774: f64 = (assign20920_e15772 * 0.01);
        let assign20920_e15775: f64 = (locals.var_tmf1 + assign20920_e15774);
        let assign20920_e15777: f64 = (assign20920_e15775 / locals.var_tmf2);
        let assign20920_e15778: f64 = (1.0 - assign20920_e15777);
        let assign20920_e15779: f64 = (0.5 * assign20920_e15778);
        (assign20920_e15779, (0.5 * (-(((locals.var_tmf1_dn0 * locals.var_tmf2) - (assign20920_e15775 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn2 * locals.var_tmf2) - (assign20920_e15775 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn4 * locals.var_tmf2) - (assign20920_e15775 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn5 * locals.var_tmf2) - (assign20920_e15775 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn6 * locals.var_tmf2) - (assign20920_e15775 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn7 * locals.var_tmf2) - (assign20920_e15775 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn8 * locals.var_tmf2) - (assign20920_e15775 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn9 * locals.var_tmf2) - (assign20920_e15775 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn10 * locals.var_tmf2) - (assign20920_e15775 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn11 * locals.var_tmf2) - (assign20920_e15775 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn14 * locals.var_tmf2) - (assign20920_e15775 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign20920_e15781;
        locals.var_t5_dn0 = assign20920_e15781_d_n0;
        locals.var_t5_dn2 = assign20920_e15781_d_n2;
        locals.var_t5_dn4 = assign20920_e15781_d_n4;
        locals.var_t5_dn5 = assign20920_e15781_d_n5;
        locals.var_t5_dn6 = assign20920_e15781_d_n6;
        locals.var_t5_dn7 = assign20920_e15781_d_n7;
        locals.var_t5_dn8 = assign20920_e15781_d_n8;
        locals.var_t5_dn9 = assign20920_e15781_d_n9;
        locals.var_t5_dn10 = assign20920_e15781_d_n10;
        locals.var_t5_dn11 = assign20920_e15781_d_n11;
        locals.var_t5_dn14 = assign20920_e15781_d_n14;

        let (assign20930_e15795, assign20930_e15795_d_n0, assign20930_e15795_d_n2, assign20930_e15795_d_n4, assign20930_e15795_d_n5, assign20930_e15795_d_n6, assign20930_e15795_d_n7, assign20930_e15795_d_n8, assign20930_e15795_d_n9, assign20930_e15795_d_n10, assign20930_e15795_d_n11, assign20930_e15795_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard417 != 0.0)) {
        let assign20930_e15791: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign20930_e15792: f64 = (0.5 * assign20930_e15791);
        let assign20930_e15793: f64 = (locals.var_t3 - assign20930_e15792);
        (assign20930_e15793, (locals.var_t3_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_t3_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_t3_dn4 - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_t3_dn5 - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_t3_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_t3_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_t3_dn8 - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_t3_dn9 - (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (locals.var_t3_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_t3_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_t3_dn14 - (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_rdrift, locals.var_rdrift_dn0, locals.var_rdrift_dn2, locals.var_rdrift_dn4, locals.var_rdrift_dn5, locals.var_rdrift_dn6, locals.var_rdrift_dn7, locals.var_rdrift_dn8, locals.var_rdrift_dn9, locals.var_rdrift_dn10, locals.var_rdrift_dn11, locals.var_rdrift_dn14,)
    }
};
        locals.var_rdrift = assign20930_e15795;
        locals.var_rdrift_dn0 = assign20930_e15795_d_n0;
        locals.var_rdrift_dn2 = assign20930_e15795_d_n2;
        locals.var_rdrift_dn4 = assign20930_e15795_d_n4;
        locals.var_rdrift_dn5 = assign20930_e15795_d_n5;
        locals.var_rdrift_dn6 = assign20930_e15795_d_n6;
        locals.var_rdrift_dn7 = assign20930_e15795_d_n7;
        locals.var_rdrift_dn8 = assign20930_e15795_d_n8;
        locals.var_rdrift_dn9 = assign20930_e15795_d_n9;
        locals.var_rdrift_dn10 = assign20930_e15795_d_n10;
        locals.var_rdrift_dn11 = assign20930_e15795_d_n11;
        locals.var_rdrift_dn14 = assign20930_e15795_d_n14;

        let (assign20940_e15807, assign20940_e15807_d_n0, assign20940_e15807_d_n2, assign20940_e15807_d_n4, assign20940_e15807_d_n5, assign20940_e15807_d_n6, assign20940_e15807_d_n7, assign20940_e15807_d_n8, assign20940_e15807_d_n9, assign20940_e15807_d_n10, assign20940_e15807_d_n11, assign20940_e15807_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard417 != 0.0)) {
        let assign20940_e15804: f64 = (locals.var_uc_rdvb * locals.var_vbserevz);
        let assign20940_e15805: f64 = (1.0 - assign20940_e15804);
        (assign20940_e15805, (-(locals.var_uc_rdvb * locals.var_vbserevz_dn0)), (-(locals.var_uc_rdvb * locals.var_vbserevz_dn2)), (-(locals.var_uc_rdvb * locals.var_vbserevz_dn4)), (-(locals.var_uc_rdvb * locals.var_vbserevz_dn5)), (-(locals.var_uc_rdvb * locals.var_vbserevz_dn6)), (-(locals.var_uc_rdvb * locals.var_vbserevz_dn7)), (-(locals.var_uc_rdvb * locals.var_vbserevz_dn8)), (-(locals.var_uc_rdvb * locals.var_vbserevz_dn9)), (-(locals.var_uc_rdvb * locals.var_vbserevz_dn10)), (-(locals.var_uc_rdvb * locals.var_vbserevz_dn11)), (-(locals.var_uc_rdvb * locals.var_vbserevz_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign20940_e15807;
        locals.var_t1_dn0 = assign20940_e15807_d_n0;
        locals.var_t1_dn2 = assign20940_e15807_d_n2;
        locals.var_t1_dn4 = assign20940_e15807_d_n4;
        locals.var_t1_dn5 = assign20940_e15807_d_n5;
        locals.var_t1_dn6 = assign20940_e15807_d_n6;
        locals.var_t1_dn7 = assign20940_e15807_d_n7;
        locals.var_t1_dn8 = assign20940_e15807_d_n8;
        locals.var_t1_dn9 = assign20940_e15807_d_n9;
        locals.var_t1_dn10 = assign20940_e15807_d_n10;
        locals.var_t1_dn11 = assign20940_e15807_d_n11;
        locals.var_t1_dn14 = assign20940_e15807_d_n14;

        let (assign20950_e15828, assign20950_e15828_d_n0, assign20950_e15828_d_n2, assign20950_e15828_d_n4, assign20950_e15828_d_n5, assign20950_e15828_d_n6, assign20950_e15828_d_n7, assign20950_e15828_d_n8, assign20950_e15828_d_n9, assign20950_e15828_d_n10, assign20950_e15828_d_n11, assign20950_e15828_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard417 != 0.0)) {
        let assign20950_e15815: f64 = (locals.var_t1 * locals.var_t1);
        let assign20950_e15819: f64 = (0.0001 * 0.01);
        let assign20950_e15820: f64 = (4.0 * assign20950_e15819);
        let assign20950_e15823: f64 = (0.0001 * 0.01);
        let assign20950_e15824: f64 = (assign20950_e15820 * assign20950_e15823);
        let assign20950_e15825: f64 = (assign20950_e15815 + assign20950_e15824);
        let assign20950_e15826: f64 = (assign20950_e15825).sqrt();
        (assign20950_e15826, (((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) / (2.0 * assign20950_e15826)), (((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) / (2.0 * assign20950_e15826)), (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) / (2.0 * assign20950_e15826)), (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) / (2.0 * assign20950_e15826)), (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) / (2.0 * assign20950_e15826)), (((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) / (2.0 * assign20950_e15826)), (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) / (2.0 * assign20950_e15826)), (((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) / (2.0 * assign20950_e15826)), (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) / (2.0 * assign20950_e15826)), (((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) / (2.0 * assign20950_e15826)), (((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14)) / (2.0 * assign20950_e15826)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign20950_e15828;
        locals.var_tmf2_dn0 = assign20950_e15828_d_n0;
        locals.var_tmf2_dn2 = assign20950_e15828_d_n2;
        locals.var_tmf2_dn4 = assign20950_e15828_d_n4;
        locals.var_tmf2_dn5 = assign20950_e15828_d_n5;
        locals.var_tmf2_dn6 = assign20950_e15828_d_n6;
        locals.var_tmf2_dn7 = assign20950_e15828_d_n7;
        locals.var_tmf2_dn8 = assign20950_e15828_d_n8;
        locals.var_tmf2_dn9 = assign20950_e15828_d_n9;
        locals.var_tmf2_dn10 = assign20950_e15828_d_n10;
        locals.var_tmf2_dn11 = assign20950_e15828_d_n11;
        locals.var_tmf2_dn14 = assign20950_e15828_d_n14;

        let (assign20960_e15842, assign20960_e15842_d_n0, assign20960_e15842_d_n2, assign20960_e15842_d_n4, assign20960_e15842_d_n5, assign20960_e15842_d_n6, assign20960_e15842_d_n7, assign20960_e15842_d_n8, assign20960_e15842_d_n9, assign20960_e15842_d_n10, assign20960_e15842_d_n11, assign20960_e15842_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard417 != 0.0)) {
        let assign20960_e15838: f64 = (locals.var_t1 / locals.var_tmf2);
        let assign20960_e15839: f64 = (1.0 + assign20960_e15838);
        let assign20960_e15840: f64 = (0.5 * assign20960_e15839);
        (assign20960_e15840, (0.5 * (((locals.var_t1_dn0 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn2 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn4 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn5 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn6 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn7 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn8 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn9 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn10 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn11 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn14 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign20960_e15842;
        locals.var_t4_dn0 = assign20960_e15842_d_n0;
        locals.var_t4_dn2 = assign20960_e15842_d_n2;
        locals.var_t4_dn4 = assign20960_e15842_d_n4;
        locals.var_t4_dn5 = assign20960_e15842_d_n5;
        locals.var_t4_dn6 = assign20960_e15842_d_n6;
        locals.var_t4_dn7 = assign20960_e15842_d_n7;
        locals.var_t4_dn8 = assign20960_e15842_d_n8;
        locals.var_t4_dn9 = assign20960_e15842_d_n9;
        locals.var_t4_dn10 = assign20960_e15842_d_n10;
        locals.var_t4_dn11 = assign20960_e15842_d_n11;
        locals.var_t4_dn14 = assign20960_e15842_d_n14;

        let (assign20970_e15854, assign20970_e15854_d_n0, assign20970_e15854_d_n2, assign20970_e15854_d_n4, assign20970_e15854_d_n5, assign20970_e15854_d_n6, assign20970_e15854_d_n7, assign20970_e15854_d_n8, assign20970_e15854_d_n9, assign20970_e15854_d_n10, assign20970_e15854_d_n11, assign20970_e15854_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard417 != 0.0)) {
        let assign20970_e15851: f64 = (locals.var_t1 + locals.var_tmf2);
        let assign20970_e15852: f64 = (0.5 * assign20970_e15851);
        (assign20970_e15852, (0.5 * (locals.var_t1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_t1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign20970_e15854;
        locals.var_t3_dn0 = assign20970_e15854_d_n0;
        locals.var_t3_dn2 = assign20970_e15854_d_n2;
        locals.var_t3_dn4 = assign20970_e15854_d_n4;
        locals.var_t3_dn5 = assign20970_e15854_d_n5;
        locals.var_t3_dn6 = assign20970_e15854_d_n6;
        locals.var_t3_dn7 = assign20970_e15854_d_n7;
        locals.var_t3_dn8 = assign20970_e15854_d_n8;
        locals.var_t3_dn9 = assign20970_e15854_d_n9;
        locals.var_t3_dn10 = assign20970_e15854_d_n10;
        locals.var_t3_dn11 = assign20970_e15854_d_n11;
        locals.var_t3_dn14 = assign20970_e15854_d_n14;

        let assign20980_e15857: f64 = if locals.var_t3 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard419 = assign20980_e15857;

        let (assign20990_e15867, assign20990_e15867_d_n0, assign20990_e15867_d_n2, assign20990_e15867_d_n4, assign20990_e15867_d_n5, assign20990_e15867_d_n6, assign20990_e15867_d_n7, assign20990_e15867_d_n8, assign20990_e15867_d_n9, assign20990_e15867_d_n10, assign20990_e15867_d_n11, assign20990_e15867_d_n14,) = {
    if ((((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard417 != 0.0)) && (locals.var_guard419 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign20990_e15867;
        locals.var_t3_dn0 = assign20990_e15867_d_n0;
        locals.var_t3_dn2 = assign20990_e15867_d_n2;
        locals.var_t3_dn4 = assign20990_e15867_d_n4;
        locals.var_t3_dn5 = assign20990_e15867_d_n5;
        locals.var_t3_dn6 = assign20990_e15867_d_n6;
        locals.var_t3_dn7 = assign20990_e15867_d_n7;
        locals.var_t3_dn8 = assign20990_e15867_d_n8;
        locals.var_t3_dn9 = assign20990_e15867_d_n9;
        locals.var_t3_dn10 = assign20990_e15867_d_n10;
        locals.var_t3_dn11 = assign20990_e15867_d_n11;
        locals.var_t3_dn14 = assign20990_e15867_d_n14;

        let (assign21000_e15877, assign21000_e15877_d_n0, assign21000_e15877_d_n2, assign21000_e15877_d_n4, assign21000_e15877_d_n5, assign21000_e15877_d_n6, assign21000_e15877_d_n7, assign21000_e15877_d_n8, assign21000_e15877_d_n9, assign21000_e15877_d_n10, assign21000_e15877_d_n11, assign21000_e15877_d_n14,) = {
    if ((((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard417 != 0.0)) && (locals.var_guard419 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign21000_e15877;
        locals.var_t4_dn0 = assign21000_e15877_d_n0;
        locals.var_t4_dn2 = assign21000_e15877_d_n2;
        locals.var_t4_dn4 = assign21000_e15877_d_n4;
        locals.var_t4_dn5 = assign21000_e15877_d_n5;
        locals.var_t4_dn6 = assign21000_e15877_d_n6;
        locals.var_t4_dn7 = assign21000_e15877_d_n7;
        locals.var_t4_dn8 = assign21000_e15877_d_n8;
        locals.var_t4_dn9 = assign21000_e15877_d_n9;
        locals.var_t4_dn10 = assign21000_e15877_d_n10;
        locals.var_t4_dn11 = assign21000_e15877_d_n11;
        locals.var_t4_dn14 = assign21000_e15877_d_n14;

        let (assign21010_e15887, assign21010_e15887_d_n0, assign21010_e15887_d_n2, assign21010_e15887_d_n4, assign21010_e15887_d_n5, assign21010_e15887_d_n6, assign21010_e15887_d_n7, assign21010_e15887_d_n8, assign21010_e15887_d_n9, assign21010_e15887_d_n10, assign21010_e15887_d_n11, assign21010_e15887_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard417 != 0.0)) {
        let assign21010_e15885: f64 = (locals.var_t3 + 1e-25);
        (assign21010_e15885, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign21010_e15887;
        locals.var_t3_dn0 = assign21010_e15887_d_n0;
        locals.var_t3_dn2 = assign21010_e15887_d_n2;
        locals.var_t3_dn4 = assign21010_e15887_d_n4;
        locals.var_t3_dn5 = assign21010_e15887_d_n5;
        locals.var_t3_dn6 = assign21010_e15887_d_n6;
        locals.var_t3_dn7 = assign21010_e15887_d_n7;
        locals.var_t3_dn8 = assign21010_e15887_d_n8;
        locals.var_t3_dn9 = assign21010_e15887_d_n9;
        locals.var_t3_dn10 = assign21010_e15887_d_n10;
        locals.var_t3_dn11 = assign21010_e15887_d_n11;
        locals.var_t3_dn14 = assign21010_e15887_d_n14;

        let (assign21020_e15895, assign21020_e15895_d_n0, assign21020_e15895_d_n2, assign21020_e15895_d_n4, assign21020_e15895_d_n5, assign21020_e15895_d_n6, assign21020_e15895_d_n7, assign21020_e15895_d_n8, assign21020_e15895_d_n9, assign21020_e15895_d_n10, assign21020_e15895_d_n11, assign21020_e15895_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard417 != 0.0)) {
        (locals.var_rdrift, locals.var_rdrift_dn0, locals.var_rdrift_dn2, locals.var_rdrift_dn4, locals.var_rdrift_dn5, locals.var_rdrift_dn6, locals.var_rdrift_dn7, locals.var_rdrift_dn8, locals.var_rdrift_dn9, locals.var_rdrift_dn10, locals.var_rdrift_dn11, locals.var_rdrift_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign21020_e15895;
        locals.var_t0_dn0 = assign21020_e15895_d_n0;
        locals.var_t0_dn2 = assign21020_e15895_d_n2;
        locals.var_t0_dn4 = assign21020_e15895_d_n4;
        locals.var_t0_dn5 = assign21020_e15895_d_n5;
        locals.var_t0_dn6 = assign21020_e15895_d_n6;
        locals.var_t0_dn7 = assign21020_e15895_d_n7;
        locals.var_t0_dn8 = assign21020_e15895_d_n8;
        locals.var_t0_dn9 = assign21020_e15895_d_n9;
        locals.var_t0_dn10 = assign21020_e15895_d_n10;
        locals.var_t0_dn11 = assign21020_e15895_d_n11;
        locals.var_t0_dn14 = assign21020_e15895_d_n14;

        let (assign21030_e15905, assign21030_e15905_d_n0, assign21030_e15905_d_n2, assign21030_e15905_d_n4, assign21030_e15905_d_n5, assign21030_e15905_d_n6, assign21030_e15905_d_n7, assign21030_e15905_d_n8, assign21030_e15905_d_n9, assign21030_e15905_d_n10, assign21030_e15905_d_n11, assign21030_e15905_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard417 != 0.0)) {
        let assign21030_e15903: f64 = (locals.var_rdrift * locals.var_t3);
        (assign21030_e15903, ((locals.var_rdrift_dn0 * locals.var_t3) + (locals.var_rdrift * locals.var_t3_dn0)), ((locals.var_rdrift_dn2 * locals.var_t3) + (locals.var_rdrift * locals.var_t3_dn2)), ((locals.var_rdrift_dn4 * locals.var_t3) + (locals.var_rdrift * locals.var_t3_dn4)), ((locals.var_rdrift_dn5 * locals.var_t3) + (locals.var_rdrift * locals.var_t3_dn5)), ((locals.var_rdrift_dn6 * locals.var_t3) + (locals.var_rdrift * locals.var_t3_dn6)), ((locals.var_rdrift_dn7 * locals.var_t3) + (locals.var_rdrift * locals.var_t3_dn7)), ((locals.var_rdrift_dn8 * locals.var_t3) + (locals.var_rdrift * locals.var_t3_dn8)), ((locals.var_rdrift_dn9 * locals.var_t3) + (locals.var_rdrift * locals.var_t3_dn9)), ((locals.var_rdrift_dn10 * locals.var_t3) + (locals.var_rdrift * locals.var_t3_dn10)), ((locals.var_rdrift_dn11 * locals.var_t3) + (locals.var_rdrift * locals.var_t3_dn11)), ((locals.var_rdrift_dn14 * locals.var_t3) + (locals.var_rdrift * locals.var_t3_dn14)),)
    } else {
        (locals.var_rdrift, locals.var_rdrift_dn0, locals.var_rdrift_dn2, locals.var_rdrift_dn4, locals.var_rdrift_dn5, locals.var_rdrift_dn6, locals.var_rdrift_dn7, locals.var_rdrift_dn8, locals.var_rdrift_dn9, locals.var_rdrift_dn10, locals.var_rdrift_dn11, locals.var_rdrift_dn14,)
    }
};
        locals.var_rdrift = assign21030_e15905;
        locals.var_rdrift_dn0 = assign21030_e15905_d_n0;
        locals.var_rdrift_dn2 = assign21030_e15905_d_n2;
        locals.var_rdrift_dn4 = assign21030_e15905_d_n4;
        locals.var_rdrift_dn5 = assign21030_e15905_d_n5;
        locals.var_rdrift_dn6 = assign21030_e15905_d_n6;
        locals.var_rdrift_dn7 = assign21030_e15905_d_n7;
        locals.var_rdrift_dn8 = assign21030_e15905_d_n8;
        locals.var_rdrift_dn9 = assign21030_e15905_d_n9;
        locals.var_rdrift_dn10 = assign21030_e15905_d_n10;
        locals.var_rdrift_dn11 = assign21030_e15905_d_n11;
        locals.var_rdrift_dn14 = assign21030_e15905_d_n14;

        let (assign21040_e15914, assign21040_e15914_d_n0, assign21040_e15914_d_n2, assign21040_e15914_d_n4, assign21040_e15914_d_n5, assign21040_e15914_d_n6, assign21040_e15914_d_n7, assign21040_e15914_d_n8, assign21040_e15914_d_n9, assign21040_e15914_d_n10, assign21040_e15914_d_n11, assign21040_e15914_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard417 == 0.0)) {
        (locals.var_rse, locals.var_rse_dn0, locals.var_rse_dn2, locals.var_rse_dn4, locals.var_rse_dn5, locals.var_rse_dn6, locals.var_rse_dn7, locals.var_rse_dn8, locals.var_rse_dn9, locals.var_rse_dn10, locals.var_rse_dn11, locals.var_rse_dn14,)
    } else {
        (locals.var_rdrift, locals.var_rdrift_dn0, locals.var_rdrift_dn2, locals.var_rdrift_dn4, locals.var_rdrift_dn5, locals.var_rdrift_dn6, locals.var_rdrift_dn7, locals.var_rdrift_dn8, locals.var_rdrift_dn9, locals.var_rdrift_dn10, locals.var_rdrift_dn11, locals.var_rdrift_dn14,)
    }
};
        locals.var_rdrift = assign21040_e15914;
        locals.var_rdrift_dn0 = assign21040_e15914_d_n0;
        locals.var_rdrift_dn2 = assign21040_e15914_d_n2;
        locals.var_rdrift_dn4 = assign21040_e15914_d_n4;
        locals.var_rdrift_dn5 = assign21040_e15914_d_n5;
        locals.var_rdrift_dn6 = assign21040_e15914_d_n6;
        locals.var_rdrift_dn7 = assign21040_e15914_d_n7;
        locals.var_rdrift_dn8 = assign21040_e15914_d_n8;
        locals.var_rdrift_dn9 = assign21040_e15914_d_n9;
        locals.var_rdrift_dn10 = assign21040_e15914_d_n10;
        locals.var_rdrift_dn11 = assign21040_e15914_d_n11;
        locals.var_rdrift_dn14 = assign21040_e15914_d_n14;

        let (assign21050_e15926, assign21050_e15926_d_n0, assign21050_e15926_d_n2, assign21050_e15926_d_n4, assign21050_e15926_d_n5, assign21050_e15926_d_n6, assign21050_e15926_d_n7, assign21050_e15926_d_n8, assign21050_e15926_d_n9, assign21050_e15926_d_n10, assign21050_e15926_d_n11, assign21050_e15926_d_n14,) = {
    if ((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) {
        let assign21050_e15920: f64 = (locals.var_vdsemodenml * locals.var_rse);
        let assign21050_e15923: f64 = (locals.var_vdsemodervs * locals.var_rde);
        let assign21050_e15924: f64 = (assign21050_e15920 + assign21050_e15923);
        (assign21050_e15924, ((locals.var_vdsemodenml * locals.var_rse_dn0) + (locals.var_vdsemodervs * locals.var_rde_dn0)), ((locals.var_vdsemodenml * locals.var_rse_dn2) + (locals.var_vdsemodervs * locals.var_rde_dn2)), ((locals.var_vdsemodenml * locals.var_rse_dn4) + (locals.var_vdsemodervs * locals.var_rde_dn4)), ((locals.var_vdsemodenml * locals.var_rse_dn5) + (locals.var_vdsemodervs * locals.var_rde_dn5)), ((locals.var_vdsemodenml * locals.var_rse_dn6) + (locals.var_vdsemodervs * locals.var_rde_dn6)), ((locals.var_vdsemodenml * locals.var_rse_dn7) + (locals.var_vdsemodervs * locals.var_rde_dn7)), ((locals.var_vdsemodenml * locals.var_rse_dn8) + (locals.var_vdsemodervs * locals.var_rde_dn8)), ((locals.var_vdsemodenml * locals.var_rse_dn9) + (locals.var_vdsemodervs * locals.var_rde_dn9)), ((locals.var_vdsemodenml * locals.var_rse_dn10) + (locals.var_vdsemodervs * locals.var_rde_dn10)), ((locals.var_vdsemodenml * locals.var_rse_dn11) + (locals.var_vdsemodervs * locals.var_rde_dn11)), ((locals.var_vdsemodenml * locals.var_rse_dn14) + (locals.var_vdsemodervs * locals.var_rde_dn14)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign21050_e15926;
        locals.var_t4_dn0 = assign21050_e15926_d_n0;
        locals.var_t4_dn2 = assign21050_e15926_d_n2;
        locals.var_t4_dn4 = assign21050_e15926_d_n4;
        locals.var_t4_dn5 = assign21050_e15926_d_n5;
        locals.var_t4_dn6 = assign21050_e15926_d_n6;
        locals.var_t4_dn7 = assign21050_e15926_d_n7;
        locals.var_t4_dn8 = assign21050_e15926_d_n8;
        locals.var_t4_dn9 = assign21050_e15926_d_n9;
        locals.var_t4_dn10 = assign21050_e15926_d_n10;
        locals.var_t4_dn11 = assign21050_e15926_d_n11;
        locals.var_t4_dn14 = assign21050_e15926_d_n14;

        let assign21060_e15933: f64 = if ((p.p34 == 1.0) || (locals.var_vdsemodervs == 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard420 = assign21060_e15933;

        let (assign21070_e15947, assign21070_e15947_d_n0, assign21070_e15947_d_n2, assign21070_e15947_d_n4, assign21070_e15947_d_n5, assign21070_e15947_d_n6, assign21070_e15947_d_n7, assign21070_e15947_d_n8, assign21070_e15947_d_n9, assign21070_e15947_d_n10, assign21070_e15947_d_n11, assign21070_e15947_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard420 != 0.0)) {
        let assign21070_e15941: f64 = (locals.var_vdsemodenml * locals.var_rsvde);
        let assign21070_e15944: f64 = (locals.var_vdsemodervs * locals.var_rdvde);
        let assign21070_e15945: f64 = (assign21070_e15941 + assign21070_e15944);
        (assign21070_e15945, ((locals.var_vdsemodenml * locals.var_rsvde_dn0) + (locals.var_vdsemodervs * locals.var_rdvde_dn0)), ((locals.var_vdsemodenml * locals.var_rsvde_dn2) + (locals.var_vdsemodervs * locals.var_rdvde_dn2)), ((locals.var_vdsemodenml * locals.var_rsvde_dn4) + (locals.var_vdsemodervs * locals.var_rdvde_dn4)), ((locals.var_vdsemodenml * locals.var_rsvde_dn5) + (locals.var_vdsemodervs * locals.var_rdvde_dn5)), ((locals.var_vdsemodenml * locals.var_rsvde_dn6) + (locals.var_vdsemodervs * locals.var_rdvde_dn6)), ((locals.var_vdsemodenml * locals.var_rsvde_dn7) + (locals.var_vdsemodervs * locals.var_rdvde_dn7)), ((locals.var_vdsemodenml * locals.var_rsvde_dn8) + (locals.var_vdsemodervs * locals.var_rdvde_dn8)), ((locals.var_vdsemodenml * locals.var_rsvde_dn9) + (locals.var_vdsemodervs * locals.var_rdvde_dn9)), ((locals.var_vdsemodenml * locals.var_rsvde_dn10) + (locals.var_vdsemodervs * locals.var_rdvde_dn10)), ((locals.var_vdsemodenml * locals.var_rsvde_dn11) + (locals.var_vdsemodervs * locals.var_rdvde_dn11)), ((locals.var_vdsemodenml * locals.var_rsvde_dn14) + (locals.var_vdsemodervs * locals.var_rdvde_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign21070_e15947;
        locals.var_t0_dn0 = assign21070_e15947_d_n0;
        locals.var_t0_dn2 = assign21070_e15947_d_n2;
        locals.var_t0_dn4 = assign21070_e15947_d_n4;
        locals.var_t0_dn5 = assign21070_e15947_d_n5;
        locals.var_t0_dn6 = assign21070_e15947_d_n6;
        locals.var_t0_dn7 = assign21070_e15947_d_n7;
        locals.var_t0_dn8 = assign21070_e15947_d_n8;
        locals.var_t0_dn9 = assign21070_e15947_d_n9;
        locals.var_t0_dn10 = assign21070_e15947_d_n10;
        locals.var_t0_dn11 = assign21070_e15947_d_n11;
        locals.var_t0_dn14 = assign21070_e15947_d_n14;

        let (assign21080_e15961, assign21080_e15961_d_n0, assign21080_e15961_d_n2, assign21080_e15961_d_n4, assign21080_e15961_d_n5, assign21080_e15961_d_n6, assign21080_e15961_d_n7, assign21080_e15961_d_n8, assign21080_e15961_d_n9, assign21080_e15961_d_n10, assign21080_e15961_d_n11, assign21080_e15961_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard420 != 0.0)) {
        let assign21080_e15957: f64 = (2.0 * p.p262);
        let assign21080_e15958: f64 = (locals.var_t0 * assign21080_e15957);
        let assign21080_e15959: f64 = (locals.var_t4 + assign21080_e15958);
        (assign21080_e15959, (locals.var_t4_dn0 + (locals.var_t0_dn0 * assign21080_e15957)), (locals.var_t4_dn2 + (locals.var_t0_dn2 * assign21080_e15957)), (locals.var_t4_dn4 + (locals.var_t0_dn4 * assign21080_e15957)), (locals.var_t4_dn5 + (locals.var_t0_dn5 * assign21080_e15957)), (locals.var_t4_dn6 + (locals.var_t0_dn6 * assign21080_e15957)), (locals.var_t4_dn7 + (locals.var_t0_dn7 * assign21080_e15957)), (locals.var_t4_dn8 + (locals.var_t0_dn8 * assign21080_e15957)), (locals.var_t4_dn9 + (locals.var_t0_dn9 * assign21080_e15957)), (locals.var_t4_dn10 + (locals.var_t0_dn10 * assign21080_e15957)), (locals.var_t4_dn11 + (locals.var_t0_dn11 * assign21080_e15957)), (locals.var_t4_dn14 + (locals.var_t0_dn14 * assign21080_e15957)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign21080_e15961;
        locals.var_t4_dn0 = assign21080_e15961_d_n0;
        locals.var_t4_dn2 = assign21080_e15961_d_n2;
        locals.var_t4_dn4 = assign21080_e15961_d_n4;
        locals.var_t4_dn5 = assign21080_e15961_d_n5;
        locals.var_t4_dn6 = assign21080_e15961_d_n6;
        locals.var_t4_dn7 = assign21080_e15961_d_n7;
        locals.var_t4_dn8 = assign21080_e15961_d_n8;
        locals.var_t4_dn9 = assign21080_e15961_d_n9;
        locals.var_t4_dn10 = assign21080_e15961_d_n10;
        locals.var_t4_dn11 = assign21080_e15961_d_n11;
        locals.var_t4_dn14 = assign21080_e15961_d_n14;

        let (assign21090_e15971, assign21090_e15971_d_n0, assign21090_e15971_d_n2, assign21090_e15971_d_n4, assign21090_e15971_d_n5, assign21090_e15971_d_n6, assign21090_e15971_d_n7, assign21090_e15971_d_n8, assign21090_e15971_d_n9, assign21090_e15971_d_n10, assign21090_e15971_d_n11, assign21090_e15971_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard420 != 0.0)) {
        let assign21090_e15969: f64 = (p.p292 + 1e-25);
        (assign21090_e15969, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign21090_e15971;
        locals.var_t10_dn0 = assign21090_e15971_d_n0;
        locals.var_t10_dn2 = assign21090_e15971_d_n2;
        locals.var_t10_dn4 = assign21090_e15971_d_n4;
        locals.var_t10_dn5 = assign21090_e15971_d_n5;
        locals.var_t10_dn6 = assign21090_e15971_d_n6;
        locals.var_t10_dn7 = assign21090_e15971_d_n7;
        locals.var_t10_dn8 = assign21090_e15971_d_n8;
        locals.var_t10_dn9 = assign21090_e15971_d_n9;
        locals.var_t10_dn10 = assign21090_e15971_d_n10;
        locals.var_t10_dn11 = assign21090_e15971_d_n11;
        locals.var_t10_dn14 = assign21090_e15971_d_n14;

    }

    pub(super) fn stamp_transient_block_51(
        locals: &mut StampLocals,
    ) {
        let (assign21100_e15989, assign21100_e15989_d_n0, assign21100_e15989_d_n2, assign21100_e15989_d_n4, assign21100_e15989_d_n5, assign21100_e15989_d_n6, assign21100_e15989_d_n7, assign21100_e15989_d_n8, assign21100_e15989_d_n9, assign21100_e15989_d_n10, assign21100_e15989_d_n11, assign21100_e15989_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard420 != 0.0)) {
        let assign21100_e15983: f64 = (locals.var_vgserevz / locals.var_t10);
        let assign21100_e15984: f64 = (1.0 - assign21100_e15983);
        let assign21100_e15985: f64 = (locals.var_uc_rdvg11 * assign21100_e15984);
        let assign21100_e15986: f64 = (1.0 + assign21100_e15985);
        let assign21100_e15987: f64 = (locals.var_t4 * assign21100_e15986);
        (assign21100_e15987, ((locals.var_t4_dn0 * assign21100_e15986) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn0 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn0)) / (locals.var_t10 * locals.var_t10)))))), ((locals.var_t4_dn2 * assign21100_e15986) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn2 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn2)) / (locals.var_t10 * locals.var_t10)))))), ((locals.var_t4_dn4 * assign21100_e15986) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn4 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn4)) / (locals.var_t10 * locals.var_t10)))))), ((locals.var_t4_dn5 * assign21100_e15986) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn5 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn5)) / (locals.var_t10 * locals.var_t10)))))), ((locals.var_t4_dn6 * assign21100_e15986) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn6 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn6)) / (locals.var_t10 * locals.var_t10)))))), ((locals.var_t4_dn7 * assign21100_e15986) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn7 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn7)) / (locals.var_t10 * locals.var_t10)))))), ((locals.var_t4_dn8 * assign21100_e15986) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn8 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn8)) / (locals.var_t10 * locals.var_t10)))))), ((locals.var_t4_dn9 * assign21100_e15986) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn9 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn9)) / (locals.var_t10 * locals.var_t10)))))), ((locals.var_t4_dn10 * assign21100_e15986) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn10 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn10)) / (locals.var_t10 * locals.var_t10)))))), ((locals.var_t4_dn11 * assign21100_e15986) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn11 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn11)) / (locals.var_t10 * locals.var_t10)))))), ((locals.var_t4_dn14 * assign21100_e15986) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn14 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn14)) / (locals.var_t10 * locals.var_t10)))))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign21100_e15989;
        locals.var_t1_dn0 = assign21100_e15989_d_n0;
        locals.var_t1_dn2 = assign21100_e15989_d_n2;
        locals.var_t1_dn4 = assign21100_e15989_d_n4;
        locals.var_t1_dn5 = assign21100_e15989_d_n5;
        locals.var_t1_dn6 = assign21100_e15989_d_n6;
        locals.var_t1_dn7 = assign21100_e15989_d_n7;
        locals.var_t1_dn8 = assign21100_e15989_d_n8;
        locals.var_t1_dn9 = assign21100_e15989_d_n9;
        locals.var_t1_dn10 = assign21100_e15989_d_n10;
        locals.var_t1_dn11 = assign21100_e15989_d_n11;
        locals.var_t1_dn14 = assign21100_e15989_d_n14;

        let (assign21110_e16003, assign21110_e16003_d_n0, assign21110_e16003_d_n2, assign21110_e16003_d_n4, assign21110_e16003_d_n5, assign21110_e16003_d_n6, assign21110_e16003_d_n7, assign21110_e16003_d_n8, assign21110_e16003_d_n9, assign21110_e16003_d_n10, assign21110_e16003_d_n11, assign21110_e16003_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard420 != 0.0)) {
        let assign21110_e15997: f64 = (locals.var_t1 - locals.var_t4);
        let assign21110_e16000: f64 = (0.01 * 0.01);
        let assign21110_e16001: f64 = (assign21110_e15997 - assign21110_e16000);
        (assign21110_e16001, (locals.var_t1_dn0 - locals.var_t4_dn0), (locals.var_t1_dn2 - locals.var_t4_dn2), (locals.var_t1_dn4 - locals.var_t4_dn4), (locals.var_t1_dn5 - locals.var_t4_dn5), (locals.var_t1_dn6 - locals.var_t4_dn6), (locals.var_t1_dn7 - locals.var_t4_dn7), (locals.var_t1_dn8 - locals.var_t4_dn8), (locals.var_t1_dn9 - locals.var_t4_dn9), (locals.var_t1_dn10 - locals.var_t4_dn10), (locals.var_t1_dn11 - locals.var_t4_dn11), (locals.var_t1_dn14 - locals.var_t4_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign21110_e16003;
        locals.var_tmf1_dn0 = assign21110_e16003_d_n0;
        locals.var_tmf1_dn2 = assign21110_e16003_d_n2;
        locals.var_tmf1_dn4 = assign21110_e16003_d_n4;
        locals.var_tmf1_dn5 = assign21110_e16003_d_n5;
        locals.var_tmf1_dn6 = assign21110_e16003_d_n6;
        locals.var_tmf1_dn7 = assign21110_e16003_d_n7;
        locals.var_tmf1_dn8 = assign21110_e16003_d_n8;
        locals.var_tmf1_dn9 = assign21110_e16003_d_n9;
        locals.var_tmf1_dn10 = assign21110_e16003_d_n10;
        locals.var_tmf1_dn11 = assign21110_e16003_d_n11;
        locals.var_tmf1_dn14 = assign21110_e16003_d_n14;

        let (assign21120_e16017, assign21120_e16017_d_n0, assign21120_e16017_d_n2, assign21120_e16017_d_n4, assign21120_e16017_d_n5, assign21120_e16017_d_n6, assign21120_e16017_d_n7, assign21120_e16017_d_n8, assign21120_e16017_d_n9, assign21120_e16017_d_n10, assign21120_e16017_d_n11, assign21120_e16017_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard420 != 0.0)) {
        let assign21120_e16011: f64 = (4.0 * locals.var_t4);
        let assign21120_e16014: f64 = (0.01 * 0.01);
        let assign21120_e16015: f64 = (assign21120_e16011 * assign21120_e16014);
        (assign21120_e16015, ((4.0 * locals.var_t4_dn0) * assign21120_e16014), ((4.0 * locals.var_t4_dn2) * assign21120_e16014), ((4.0 * locals.var_t4_dn4) * assign21120_e16014), ((4.0 * locals.var_t4_dn5) * assign21120_e16014), ((4.0 * locals.var_t4_dn6) * assign21120_e16014), ((4.0 * locals.var_t4_dn7) * assign21120_e16014), ((4.0 * locals.var_t4_dn8) * assign21120_e16014), ((4.0 * locals.var_t4_dn9) * assign21120_e16014), ((4.0 * locals.var_t4_dn10) * assign21120_e16014), ((4.0 * locals.var_t4_dn11) * assign21120_e16014), ((4.0 * locals.var_t4_dn14) * assign21120_e16014),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign21120_e16017;
        locals.var_tmf2_dn0 = assign21120_e16017_d_n0;
        locals.var_tmf2_dn2 = assign21120_e16017_d_n2;
        locals.var_tmf2_dn4 = assign21120_e16017_d_n4;
        locals.var_tmf2_dn5 = assign21120_e16017_d_n5;
        locals.var_tmf2_dn6 = assign21120_e16017_d_n6;
        locals.var_tmf2_dn7 = assign21120_e16017_d_n7;
        locals.var_tmf2_dn8 = assign21120_e16017_d_n8;
        locals.var_tmf2_dn9 = assign21120_e16017_d_n9;
        locals.var_tmf2_dn10 = assign21120_e16017_d_n10;
        locals.var_tmf2_dn11 = assign21120_e16017_d_n11;
        locals.var_tmf2_dn14 = assign21120_e16017_d_n14;

        let (assign21130_e16031, assign21130_e16031_d_n0, assign21130_e16031_d_n2, assign21130_e16031_d_n4, assign21130_e16031_d_n5, assign21130_e16031_d_n6, assign21130_e16031_d_n7, assign21130_e16031_d_n8, assign21130_e16031_d_n9, assign21130_e16031_d_n10, assign21130_e16031_d_n11, assign21130_e16031_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard420 != 0.0)) {
        let (assign21130_e16029, assign21130_e16029_d_n0, assign21130_e16029_d_n2, assign21130_e16029_d_n4, assign21130_e16029_d_n5, assign21130_e16029_d_n6, assign21130_e16029_d_n7, assign21130_e16029_d_n8, assign21130_e16029_d_n9, assign21130_e16029_d_n10, assign21130_e16029_d_n11, assign21130_e16029_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign21130_e16028: f64 = (-locals.var_tmf2);
                (assign21130_e16028, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign21130_e16029, assign21130_e16029_d_n0, assign21130_e16029_d_n2, assign21130_e16029_d_n4, assign21130_e16029_d_n5, assign21130_e16029_d_n6, assign21130_e16029_d_n7, assign21130_e16029_d_n8, assign21130_e16029_d_n9, assign21130_e16029_d_n10, assign21130_e16029_d_n11, assign21130_e16029_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign21130_e16031;
        locals.var_tmf2_dn0 = assign21130_e16031_d_n0;
        locals.var_tmf2_dn2 = assign21130_e16031_d_n2;
        locals.var_tmf2_dn4 = assign21130_e16031_d_n4;
        locals.var_tmf2_dn5 = assign21130_e16031_d_n5;
        locals.var_tmf2_dn6 = assign21130_e16031_d_n6;
        locals.var_tmf2_dn7 = assign21130_e16031_d_n7;
        locals.var_tmf2_dn8 = assign21130_e16031_d_n8;
        locals.var_tmf2_dn9 = assign21130_e16031_d_n9;
        locals.var_tmf2_dn10 = assign21130_e16031_d_n10;
        locals.var_tmf2_dn11 = assign21130_e16031_d_n11;
        locals.var_tmf2_dn14 = assign21130_e16031_d_n14;

        let (assign21140_e16044, assign21140_e16044_d_n0, assign21140_e16044_d_n2, assign21140_e16044_d_n4, assign21140_e16044_d_n5, assign21140_e16044_d_n6, assign21140_e16044_d_n7, assign21140_e16044_d_n8, assign21140_e16044_d_n9, assign21140_e16044_d_n10, assign21140_e16044_d_n11, assign21140_e16044_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard420 != 0.0)) {
        let assign21140_e16039: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign21140_e16041: f64 = (assign21140_e16039 + locals.var_tmf2);
        let assign21140_e16042: f64 = (assign21140_e16041).sqrt();
        (assign21140_e16042, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign21140_e16042)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign21140_e16042)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign21140_e16042)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign21140_e16042)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign21140_e16042)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign21140_e16042)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign21140_e16042)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign21140_e16042)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign21140_e16042)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign21140_e16042)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign21140_e16042)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign21140_e16044;
        locals.var_tmf2_dn0 = assign21140_e16044_d_n0;
        locals.var_tmf2_dn2 = assign21140_e16044_d_n2;
        locals.var_tmf2_dn4 = assign21140_e16044_d_n4;
        locals.var_tmf2_dn5 = assign21140_e16044_d_n5;
        locals.var_tmf2_dn6 = assign21140_e16044_d_n6;
        locals.var_tmf2_dn7 = assign21140_e16044_d_n7;
        locals.var_tmf2_dn8 = assign21140_e16044_d_n8;
        locals.var_tmf2_dn9 = assign21140_e16044_d_n9;
        locals.var_tmf2_dn10 = assign21140_e16044_d_n10;
        locals.var_tmf2_dn11 = assign21140_e16044_d_n11;
        locals.var_tmf2_dn14 = assign21140_e16044_d_n14;

        let (assign21150_e16058, assign21150_e16058_d_n0, assign21150_e16058_d_n2, assign21150_e16058_d_n4, assign21150_e16058_d_n5, assign21150_e16058_d_n6, assign21150_e16058_d_n7, assign21150_e16058_d_n8, assign21150_e16058_d_n9, assign21150_e16058_d_n10, assign21150_e16058_d_n11, assign21150_e16058_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard420 != 0.0)) {
        let assign21150_e16054: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign21150_e16055: f64 = (1.0 + assign21150_e16054);
        let assign21150_e16056: f64 = (0.5 * assign21150_e16055);
        (assign21150_e16056, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign21150_e16058;
        locals.var_t0_dn0 = assign21150_e16058_d_n0;
        locals.var_t0_dn2 = assign21150_e16058_d_n2;
        locals.var_t0_dn4 = assign21150_e16058_d_n4;
        locals.var_t0_dn5 = assign21150_e16058_d_n5;
        locals.var_t0_dn6 = assign21150_e16058_d_n6;
        locals.var_t0_dn7 = assign21150_e16058_d_n7;
        locals.var_t0_dn8 = assign21150_e16058_d_n8;
        locals.var_t0_dn9 = assign21150_e16058_d_n9;
        locals.var_t0_dn10 = assign21150_e16058_d_n10;
        locals.var_t0_dn11 = assign21150_e16058_d_n11;
        locals.var_t0_dn14 = assign21150_e16058_d_n14;

        let (assign21160_e16078, assign21160_e16078_d_n0, assign21160_e16078_d_n2, assign21160_e16078_d_n4, assign21160_e16078_d_n5, assign21160_e16078_d_n6, assign21160_e16078_d_n7, assign21160_e16078_d_n8, assign21160_e16078_d_n9, assign21160_e16078_d_n10, assign21160_e16078_d_n11, assign21160_e16078_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard420 != 0.0)) {
        let assign21160_e16069: f64 = (2.0 * 0.01);
        let assign21160_e16071: f64 = (assign21160_e16069 * 0.01);
        let assign21160_e16072: f64 = (locals.var_tmf1 - assign21160_e16071);
        let assign21160_e16074: f64 = (assign21160_e16072 / locals.var_tmf2);
        let assign21160_e16075: f64 = (1.0 - assign21160_e16074);
        let assign21160_e16076: f64 = (0.5 * assign21160_e16075);
        (assign21160_e16076, (0.5 * (-(((locals.var_tmf1_dn0 * locals.var_tmf2) - (assign21160_e16072 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn2 * locals.var_tmf2) - (assign21160_e16072 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn4 * locals.var_tmf2) - (assign21160_e16072 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn5 * locals.var_tmf2) - (assign21160_e16072 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn6 * locals.var_tmf2) - (assign21160_e16072 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn7 * locals.var_tmf2) - (assign21160_e16072 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn8 * locals.var_tmf2) - (assign21160_e16072 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn9 * locals.var_tmf2) - (assign21160_e16072 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn10 * locals.var_tmf2) - (assign21160_e16072 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn11 * locals.var_tmf2) - (assign21160_e16072 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn14 * locals.var_tmf2) - (assign21160_e16072 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign21160_e16078;
        locals.var_t5_dn0 = assign21160_e16078_d_n0;
        locals.var_t5_dn2 = assign21160_e16078_d_n2;
        locals.var_t5_dn4 = assign21160_e16078_d_n4;
        locals.var_t5_dn5 = assign21160_e16078_d_n5;
        locals.var_t5_dn6 = assign21160_e16078_d_n6;
        locals.var_t5_dn7 = assign21160_e16078_d_n7;
        locals.var_t5_dn8 = assign21160_e16078_d_n8;
        locals.var_t5_dn9 = assign21160_e16078_d_n9;
        locals.var_t5_dn10 = assign21160_e16078_d_n10;
        locals.var_t5_dn11 = assign21160_e16078_d_n11;
        locals.var_t5_dn14 = assign21160_e16078_d_n14;

        let (assign21170_e16092, assign21170_e16092_d_n0, assign21170_e16092_d_n2, assign21170_e16092_d_n4, assign21170_e16092_d_n5, assign21170_e16092_d_n6, assign21170_e16092_d_n7, assign21170_e16092_d_n8, assign21170_e16092_d_n9, assign21170_e16092_d_n10, assign21170_e16092_d_n11, assign21170_e16092_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard420 != 0.0)) {
        let assign21170_e16088: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign21170_e16089: f64 = (0.5 * assign21170_e16088);
        let assign21170_e16090: f64 = (locals.var_t4 + assign21170_e16089);
        (assign21170_e16090, (locals.var_t4_dn0 + (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_t4_dn2 + (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_t4_dn4 + (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_t4_dn5 + (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_t4_dn6 + (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_t4_dn7 + (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_t4_dn8 + (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_t4_dn9 + (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (locals.var_t4_dn10 + (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_t4_dn11 + (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_t4_dn14 + (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign21170_e16092;
        locals.var_t2_dn0 = assign21170_e16092_d_n0;
        locals.var_t2_dn2 = assign21170_e16092_d_n2;
        locals.var_t2_dn4 = assign21170_e16092_d_n4;
        locals.var_t2_dn5 = assign21170_e16092_d_n5;
        locals.var_t2_dn6 = assign21170_e16092_d_n6;
        locals.var_t2_dn7 = assign21170_e16092_d_n7;
        locals.var_t2_dn8 = assign21170_e16092_d_n8;
        locals.var_t2_dn9 = assign21170_e16092_d_n9;
        locals.var_t2_dn10 = assign21170_e16092_d_n10;
        locals.var_t2_dn11 = assign21170_e16092_d_n11;
        locals.var_t2_dn14 = assign21170_e16092_d_n14;

        let (assign21180_e16104, assign21180_e16104_d_n0, assign21180_e16104_d_n2, assign21180_e16104_d_n4, assign21180_e16104_d_n5, assign21180_e16104_d_n6, assign21180_e16104_d_n7, assign21180_e16104_d_n8, assign21180_e16104_d_n9, assign21180_e16104_d_n10, assign21180_e16104_d_n11, assign21180_e16104_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard420 != 0.0)) {
        let assign21180_e16101: f64 = (1.0 + locals.var_uc_rdvg11);
        let assign21180_e16102: f64 = (locals.var_t4 * assign21180_e16101);
        (assign21180_e16102, (locals.var_t4_dn0 * assign21180_e16101), (locals.var_t4_dn2 * assign21180_e16101), (locals.var_t4_dn4 * assign21180_e16101), (locals.var_t4_dn5 * assign21180_e16101), (locals.var_t4_dn6 * assign21180_e16101), (locals.var_t4_dn7 * assign21180_e16101), (locals.var_t4_dn8 * assign21180_e16101), (locals.var_t4_dn9 * assign21180_e16101), (locals.var_t4_dn10 * assign21180_e16101), (locals.var_t4_dn11 * assign21180_e16101), (locals.var_t4_dn14 * assign21180_e16101),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign21180_e16104;
        locals.var_t3_dn0 = assign21180_e16104_d_n0;
        locals.var_t3_dn2 = assign21180_e16104_d_n2;
        locals.var_t3_dn4 = assign21180_e16104_d_n4;
        locals.var_t3_dn5 = assign21180_e16104_d_n5;
        locals.var_t3_dn6 = assign21180_e16104_d_n6;
        locals.var_t3_dn7 = assign21180_e16104_d_n7;
        locals.var_t3_dn8 = assign21180_e16104_d_n8;
        locals.var_t3_dn9 = assign21180_e16104_d_n9;
        locals.var_t3_dn10 = assign21180_e16104_d_n10;
        locals.var_t3_dn11 = assign21180_e16104_d_n11;
        locals.var_t3_dn14 = assign21180_e16104_d_n14;

        let (assign21190_e16118, assign21190_e16118_d_n0, assign21190_e16118_d_n2, assign21190_e16118_d_n4, assign21190_e16118_d_n5, assign21190_e16118_d_n6, assign21190_e16118_d_n7, assign21190_e16118_d_n8, assign21190_e16118_d_n9, assign21190_e16118_d_n10, assign21190_e16118_d_n11, assign21190_e16118_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard420 != 0.0)) {
        let assign21190_e16112: f64 = (locals.var_t3 - locals.var_t2);
        let assign21190_e16115: f64 = (5e-5 * 0.01);
        let assign21190_e16116: f64 = (assign21190_e16112 - assign21190_e16115);
        (assign21190_e16116, (locals.var_t3_dn0 - locals.var_t2_dn0), (locals.var_t3_dn2 - locals.var_t2_dn2), (locals.var_t3_dn4 - locals.var_t2_dn4), (locals.var_t3_dn5 - locals.var_t2_dn5), (locals.var_t3_dn6 - locals.var_t2_dn6), (locals.var_t3_dn7 - locals.var_t2_dn7), (locals.var_t3_dn8 - locals.var_t2_dn8), (locals.var_t3_dn9 - locals.var_t2_dn9), (locals.var_t3_dn10 - locals.var_t2_dn10), (locals.var_t3_dn11 - locals.var_t2_dn11), (locals.var_t3_dn14 - locals.var_t2_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign21190_e16118;
        locals.var_tmf1_dn0 = assign21190_e16118_d_n0;
        locals.var_tmf1_dn2 = assign21190_e16118_d_n2;
        locals.var_tmf1_dn4 = assign21190_e16118_d_n4;
        locals.var_tmf1_dn5 = assign21190_e16118_d_n5;
        locals.var_tmf1_dn6 = assign21190_e16118_d_n6;
        locals.var_tmf1_dn7 = assign21190_e16118_d_n7;
        locals.var_tmf1_dn8 = assign21190_e16118_d_n8;
        locals.var_tmf1_dn9 = assign21190_e16118_d_n9;
        locals.var_tmf1_dn10 = assign21190_e16118_d_n10;
        locals.var_tmf1_dn11 = assign21190_e16118_d_n11;
        locals.var_tmf1_dn14 = assign21190_e16118_d_n14;

        let (assign21200_e16132, assign21200_e16132_d_n0, assign21200_e16132_d_n2, assign21200_e16132_d_n4, assign21200_e16132_d_n5, assign21200_e16132_d_n6, assign21200_e16132_d_n7, assign21200_e16132_d_n8, assign21200_e16132_d_n9, assign21200_e16132_d_n10, assign21200_e16132_d_n11, assign21200_e16132_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard420 != 0.0)) {
        let assign21200_e16126: f64 = (4.0 * locals.var_t3);
        let assign21200_e16129: f64 = (5e-5 * 0.01);
        let assign21200_e16130: f64 = (assign21200_e16126 * assign21200_e16129);
        (assign21200_e16130, ((4.0 * locals.var_t3_dn0) * assign21200_e16129), ((4.0 * locals.var_t3_dn2) * assign21200_e16129), ((4.0 * locals.var_t3_dn4) * assign21200_e16129), ((4.0 * locals.var_t3_dn5) * assign21200_e16129), ((4.0 * locals.var_t3_dn6) * assign21200_e16129), ((4.0 * locals.var_t3_dn7) * assign21200_e16129), ((4.0 * locals.var_t3_dn8) * assign21200_e16129), ((4.0 * locals.var_t3_dn9) * assign21200_e16129), ((4.0 * locals.var_t3_dn10) * assign21200_e16129), ((4.0 * locals.var_t3_dn11) * assign21200_e16129), ((4.0 * locals.var_t3_dn14) * assign21200_e16129),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign21200_e16132;
        locals.var_tmf2_dn0 = assign21200_e16132_d_n0;
        locals.var_tmf2_dn2 = assign21200_e16132_d_n2;
        locals.var_tmf2_dn4 = assign21200_e16132_d_n4;
        locals.var_tmf2_dn5 = assign21200_e16132_d_n5;
        locals.var_tmf2_dn6 = assign21200_e16132_d_n6;
        locals.var_tmf2_dn7 = assign21200_e16132_d_n7;
        locals.var_tmf2_dn8 = assign21200_e16132_d_n8;
        locals.var_tmf2_dn9 = assign21200_e16132_d_n9;
        locals.var_tmf2_dn10 = assign21200_e16132_d_n10;
        locals.var_tmf2_dn11 = assign21200_e16132_d_n11;
        locals.var_tmf2_dn14 = assign21200_e16132_d_n14;

        let (assign21210_e16146, assign21210_e16146_d_n0, assign21210_e16146_d_n2, assign21210_e16146_d_n4, assign21210_e16146_d_n5, assign21210_e16146_d_n6, assign21210_e16146_d_n7, assign21210_e16146_d_n8, assign21210_e16146_d_n9, assign21210_e16146_d_n10, assign21210_e16146_d_n11, assign21210_e16146_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard420 != 0.0)) {
        let (assign21210_e16144, assign21210_e16144_d_n0, assign21210_e16144_d_n2, assign21210_e16144_d_n4, assign21210_e16144_d_n5, assign21210_e16144_d_n6, assign21210_e16144_d_n7, assign21210_e16144_d_n8, assign21210_e16144_d_n9, assign21210_e16144_d_n10, assign21210_e16144_d_n11, assign21210_e16144_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign21210_e16143: f64 = (-locals.var_tmf2);
                (assign21210_e16143, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign21210_e16144, assign21210_e16144_d_n0, assign21210_e16144_d_n2, assign21210_e16144_d_n4, assign21210_e16144_d_n5, assign21210_e16144_d_n6, assign21210_e16144_d_n7, assign21210_e16144_d_n8, assign21210_e16144_d_n9, assign21210_e16144_d_n10, assign21210_e16144_d_n11, assign21210_e16144_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign21210_e16146;
        locals.var_tmf2_dn0 = assign21210_e16146_d_n0;
        locals.var_tmf2_dn2 = assign21210_e16146_d_n2;
        locals.var_tmf2_dn4 = assign21210_e16146_d_n4;
        locals.var_tmf2_dn5 = assign21210_e16146_d_n5;
        locals.var_tmf2_dn6 = assign21210_e16146_d_n6;
        locals.var_tmf2_dn7 = assign21210_e16146_d_n7;
        locals.var_tmf2_dn8 = assign21210_e16146_d_n8;
        locals.var_tmf2_dn9 = assign21210_e16146_d_n9;
        locals.var_tmf2_dn10 = assign21210_e16146_d_n10;
        locals.var_tmf2_dn11 = assign21210_e16146_d_n11;
        locals.var_tmf2_dn14 = assign21210_e16146_d_n14;

        let (assign21220_e16159, assign21220_e16159_d_n0, assign21220_e16159_d_n2, assign21220_e16159_d_n4, assign21220_e16159_d_n5, assign21220_e16159_d_n6, assign21220_e16159_d_n7, assign21220_e16159_d_n8, assign21220_e16159_d_n9, assign21220_e16159_d_n10, assign21220_e16159_d_n11, assign21220_e16159_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard420 != 0.0)) {
        let assign21220_e16154: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign21220_e16156: f64 = (assign21220_e16154 + locals.var_tmf2);
        let assign21220_e16157: f64 = (assign21220_e16156).sqrt();
        (assign21220_e16157, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign21220_e16157)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign21220_e16157)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign21220_e16157)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign21220_e16157)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign21220_e16157)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign21220_e16157)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign21220_e16157)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign21220_e16157)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign21220_e16157)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign21220_e16157)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign21220_e16157)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign21220_e16159;
        locals.var_tmf2_dn0 = assign21220_e16159_d_n0;
        locals.var_tmf2_dn2 = assign21220_e16159_d_n2;
        locals.var_tmf2_dn4 = assign21220_e16159_d_n4;
        locals.var_tmf2_dn5 = assign21220_e16159_d_n5;
        locals.var_tmf2_dn6 = assign21220_e16159_d_n6;
        locals.var_tmf2_dn7 = assign21220_e16159_d_n7;
        locals.var_tmf2_dn8 = assign21220_e16159_d_n8;
        locals.var_tmf2_dn9 = assign21220_e16159_d_n9;
        locals.var_tmf2_dn10 = assign21220_e16159_d_n10;
        locals.var_tmf2_dn11 = assign21220_e16159_d_n11;
        locals.var_tmf2_dn14 = assign21220_e16159_d_n14;

        let (assign21230_e16173, assign21230_e16173_d_n0, assign21230_e16173_d_n2, assign21230_e16173_d_n4, assign21230_e16173_d_n5, assign21230_e16173_d_n6, assign21230_e16173_d_n7, assign21230_e16173_d_n8, assign21230_e16173_d_n9, assign21230_e16173_d_n10, assign21230_e16173_d_n11, assign21230_e16173_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard420 != 0.0)) {
        let assign21230_e16169: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign21230_e16170: f64 = (1.0 + assign21230_e16169);
        let assign21230_e16171: f64 = (0.5 * assign21230_e16170);
        (assign21230_e16171, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign21230_e16173;
        locals.var_t0_dn0 = assign21230_e16173_d_n0;
        locals.var_t0_dn2 = assign21230_e16173_d_n2;
        locals.var_t0_dn4 = assign21230_e16173_d_n4;
        locals.var_t0_dn5 = assign21230_e16173_d_n5;
        locals.var_t0_dn6 = assign21230_e16173_d_n6;
        locals.var_t0_dn7 = assign21230_e16173_d_n7;
        locals.var_t0_dn8 = assign21230_e16173_d_n8;
        locals.var_t0_dn9 = assign21230_e16173_d_n9;
        locals.var_t0_dn10 = assign21230_e16173_d_n10;
        locals.var_t0_dn11 = assign21230_e16173_d_n11;
        locals.var_t0_dn14 = assign21230_e16173_d_n14;

        let (assign21240_e16193, assign21240_e16193_d_n0, assign21240_e16193_d_n2, assign21240_e16193_d_n4, assign21240_e16193_d_n5, assign21240_e16193_d_n6, assign21240_e16193_d_n7, assign21240_e16193_d_n8, assign21240_e16193_d_n9, assign21240_e16193_d_n10, assign21240_e16193_d_n11, assign21240_e16193_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard420 != 0.0)) {
        let assign21240_e16184: f64 = (2.0 * 5e-5);
        let assign21240_e16186: f64 = (assign21240_e16184 * 0.01);
        let assign21240_e16187: f64 = (locals.var_tmf1 + assign21240_e16186);
        let assign21240_e16189: f64 = (assign21240_e16187 / locals.var_tmf2);
        let assign21240_e16190: f64 = (1.0 - assign21240_e16189);
        let assign21240_e16191: f64 = (0.5 * assign21240_e16190);
        (assign21240_e16191, (0.5 * (-(((locals.var_tmf1_dn0 * locals.var_tmf2) - (assign21240_e16187 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn2 * locals.var_tmf2) - (assign21240_e16187 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn4 * locals.var_tmf2) - (assign21240_e16187 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn5 * locals.var_tmf2) - (assign21240_e16187 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn6 * locals.var_tmf2) - (assign21240_e16187 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn7 * locals.var_tmf2) - (assign21240_e16187 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn8 * locals.var_tmf2) - (assign21240_e16187 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn9 * locals.var_tmf2) - (assign21240_e16187 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn10 * locals.var_tmf2) - (assign21240_e16187 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn11 * locals.var_tmf2) - (assign21240_e16187 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn14 * locals.var_tmf2) - (assign21240_e16187 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign21240_e16193;
        locals.var_t5_dn0 = assign21240_e16193_d_n0;
        locals.var_t5_dn2 = assign21240_e16193_d_n2;
        locals.var_t5_dn4 = assign21240_e16193_d_n4;
        locals.var_t5_dn5 = assign21240_e16193_d_n5;
        locals.var_t5_dn6 = assign21240_e16193_d_n6;
        locals.var_t5_dn7 = assign21240_e16193_d_n7;
        locals.var_t5_dn8 = assign21240_e16193_d_n8;
        locals.var_t5_dn9 = assign21240_e16193_d_n9;
        locals.var_t5_dn10 = assign21240_e16193_d_n10;
        locals.var_t5_dn11 = assign21240_e16193_d_n11;
        locals.var_t5_dn14 = assign21240_e16193_d_n14;

        let (assign21250_e16207, assign21250_e16207_d_n0, assign21250_e16207_d_n2, assign21250_e16207_d_n4, assign21250_e16207_d_n5, assign21250_e16207_d_n6, assign21250_e16207_d_n7, assign21250_e16207_d_n8, assign21250_e16207_d_n9, assign21250_e16207_d_n10, assign21250_e16207_d_n11, assign21250_e16207_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard420 != 0.0)) {
        let assign21250_e16203: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign21250_e16204: f64 = (0.5 * assign21250_e16203);
        let assign21250_e16205: f64 = (locals.var_t3 - assign21250_e16204);
        (assign21250_e16205, (locals.var_t3_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_t3_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_t3_dn4 - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_t3_dn5 - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_t3_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_t3_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_t3_dn8 - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_t3_dn9 - (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (locals.var_t3_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_t3_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_t3_dn14 - (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_rsdrift, locals.var_rsdrift_dn0, locals.var_rsdrift_dn2, locals.var_rsdrift_dn4, locals.var_rsdrift_dn5, locals.var_rsdrift_dn6, locals.var_rsdrift_dn7, locals.var_rsdrift_dn8, locals.var_rsdrift_dn9, locals.var_rsdrift_dn10, locals.var_rsdrift_dn11, locals.var_rsdrift_dn14,)
    }
};
        locals.var_rsdrift = assign21250_e16207;
        locals.var_rsdrift_dn0 = assign21250_e16207_d_n0;
        locals.var_rsdrift_dn2 = assign21250_e16207_d_n2;
        locals.var_rsdrift_dn4 = assign21250_e16207_d_n4;
        locals.var_rsdrift_dn5 = assign21250_e16207_d_n5;
        locals.var_rsdrift_dn6 = assign21250_e16207_d_n6;
        locals.var_rsdrift_dn7 = assign21250_e16207_d_n7;
        locals.var_rsdrift_dn8 = assign21250_e16207_d_n8;
        locals.var_rsdrift_dn9 = assign21250_e16207_d_n9;
        locals.var_rsdrift_dn10 = assign21250_e16207_d_n10;
        locals.var_rsdrift_dn11 = assign21250_e16207_d_n11;
        locals.var_rsdrift_dn14 = assign21250_e16207_d_n14;

        let (assign21260_e16219, assign21260_e16219_d_n0, assign21260_e16219_d_n2, assign21260_e16219_d_n4, assign21260_e16219_d_n5, assign21260_e16219_d_n6, assign21260_e16219_d_n7, assign21260_e16219_d_n8, assign21260_e16219_d_n9, assign21260_e16219_d_n10, assign21260_e16219_d_n11, assign21260_e16219_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard420 != 0.0)) {
        let assign21260_e16216: f64 = (locals.var_uc_rdvb * locals.var_vbserevz);
        let assign21260_e16217: f64 = (1.0 - assign21260_e16216);
        (assign21260_e16217, (-(locals.var_uc_rdvb * locals.var_vbserevz_dn0)), (-(locals.var_uc_rdvb * locals.var_vbserevz_dn2)), (-(locals.var_uc_rdvb * locals.var_vbserevz_dn4)), (-(locals.var_uc_rdvb * locals.var_vbserevz_dn5)), (-(locals.var_uc_rdvb * locals.var_vbserevz_dn6)), (-(locals.var_uc_rdvb * locals.var_vbserevz_dn7)), (-(locals.var_uc_rdvb * locals.var_vbserevz_dn8)), (-(locals.var_uc_rdvb * locals.var_vbserevz_dn9)), (-(locals.var_uc_rdvb * locals.var_vbserevz_dn10)), (-(locals.var_uc_rdvb * locals.var_vbserevz_dn11)), (-(locals.var_uc_rdvb * locals.var_vbserevz_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign21260_e16219;
        locals.var_t1_dn0 = assign21260_e16219_d_n0;
        locals.var_t1_dn2 = assign21260_e16219_d_n2;
        locals.var_t1_dn4 = assign21260_e16219_d_n4;
        locals.var_t1_dn5 = assign21260_e16219_d_n5;
        locals.var_t1_dn6 = assign21260_e16219_d_n6;
        locals.var_t1_dn7 = assign21260_e16219_d_n7;
        locals.var_t1_dn8 = assign21260_e16219_d_n8;
        locals.var_t1_dn9 = assign21260_e16219_d_n9;
        locals.var_t1_dn10 = assign21260_e16219_d_n10;
        locals.var_t1_dn11 = assign21260_e16219_d_n11;
        locals.var_t1_dn14 = assign21260_e16219_d_n14;

        let (assign21270_e16240, assign21270_e16240_d_n0, assign21270_e16240_d_n2, assign21270_e16240_d_n4, assign21270_e16240_d_n5, assign21270_e16240_d_n6, assign21270_e16240_d_n7, assign21270_e16240_d_n8, assign21270_e16240_d_n9, assign21270_e16240_d_n10, assign21270_e16240_d_n11, assign21270_e16240_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard420 != 0.0)) {
        let assign21270_e16227: f64 = (locals.var_t1 * locals.var_t1);
        let assign21270_e16231: f64 = (0.0001 * 0.01);
        let assign21270_e16232: f64 = (4.0 * assign21270_e16231);
        let assign21270_e16235: f64 = (0.0001 * 0.01);
        let assign21270_e16236: f64 = (assign21270_e16232 * assign21270_e16235);
        let assign21270_e16237: f64 = (assign21270_e16227 + assign21270_e16236);
        let assign21270_e16238: f64 = (assign21270_e16237).sqrt();
        (assign21270_e16238, (((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) / (2.0 * assign21270_e16238)), (((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) / (2.0 * assign21270_e16238)), (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) / (2.0 * assign21270_e16238)), (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) / (2.0 * assign21270_e16238)), (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) / (2.0 * assign21270_e16238)), (((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) / (2.0 * assign21270_e16238)), (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) / (2.0 * assign21270_e16238)), (((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) / (2.0 * assign21270_e16238)), (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) / (2.0 * assign21270_e16238)), (((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) / (2.0 * assign21270_e16238)), (((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14)) / (2.0 * assign21270_e16238)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign21270_e16240;
        locals.var_tmf2_dn0 = assign21270_e16240_d_n0;
        locals.var_tmf2_dn2 = assign21270_e16240_d_n2;
        locals.var_tmf2_dn4 = assign21270_e16240_d_n4;
        locals.var_tmf2_dn5 = assign21270_e16240_d_n5;
        locals.var_tmf2_dn6 = assign21270_e16240_d_n6;
        locals.var_tmf2_dn7 = assign21270_e16240_d_n7;
        locals.var_tmf2_dn8 = assign21270_e16240_d_n8;
        locals.var_tmf2_dn9 = assign21270_e16240_d_n9;
        locals.var_tmf2_dn10 = assign21270_e16240_d_n10;
        locals.var_tmf2_dn11 = assign21270_e16240_d_n11;
        locals.var_tmf2_dn14 = assign21270_e16240_d_n14;

        let (assign21280_e16254, assign21280_e16254_d_n0, assign21280_e16254_d_n2, assign21280_e16254_d_n4, assign21280_e16254_d_n5, assign21280_e16254_d_n6, assign21280_e16254_d_n7, assign21280_e16254_d_n8, assign21280_e16254_d_n9, assign21280_e16254_d_n10, assign21280_e16254_d_n11, assign21280_e16254_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard420 != 0.0)) {
        let assign21280_e16250: f64 = (locals.var_t1 / locals.var_tmf2);
        let assign21280_e16251: f64 = (1.0 + assign21280_e16250);
        let assign21280_e16252: f64 = (0.5 * assign21280_e16251);
        (assign21280_e16252, (0.5 * (((locals.var_t1_dn0 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn2 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn4 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn5 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn6 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn7 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn8 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn9 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn10 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn11 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn14 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign21280_e16254;
        locals.var_t4_dn0 = assign21280_e16254_d_n0;
        locals.var_t4_dn2 = assign21280_e16254_d_n2;
        locals.var_t4_dn4 = assign21280_e16254_d_n4;
        locals.var_t4_dn5 = assign21280_e16254_d_n5;
        locals.var_t4_dn6 = assign21280_e16254_d_n6;
        locals.var_t4_dn7 = assign21280_e16254_d_n7;
        locals.var_t4_dn8 = assign21280_e16254_d_n8;
        locals.var_t4_dn9 = assign21280_e16254_d_n9;
        locals.var_t4_dn10 = assign21280_e16254_d_n10;
        locals.var_t4_dn11 = assign21280_e16254_d_n11;
        locals.var_t4_dn14 = assign21280_e16254_d_n14;

        let (assign21290_e16266, assign21290_e16266_d_n0, assign21290_e16266_d_n2, assign21290_e16266_d_n4, assign21290_e16266_d_n5, assign21290_e16266_d_n6, assign21290_e16266_d_n7, assign21290_e16266_d_n8, assign21290_e16266_d_n9, assign21290_e16266_d_n10, assign21290_e16266_d_n11, assign21290_e16266_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard420 != 0.0)) {
        let assign21290_e16263: f64 = (locals.var_t1 + locals.var_tmf2);
        let assign21290_e16264: f64 = (0.5 * assign21290_e16263);
        (assign21290_e16264, (0.5 * (locals.var_t1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_t1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign21290_e16266;
        locals.var_t3_dn0 = assign21290_e16266_d_n0;
        locals.var_t3_dn2 = assign21290_e16266_d_n2;
        locals.var_t3_dn4 = assign21290_e16266_d_n4;
        locals.var_t3_dn5 = assign21290_e16266_d_n5;
        locals.var_t3_dn6 = assign21290_e16266_d_n6;
        locals.var_t3_dn7 = assign21290_e16266_d_n7;
        locals.var_t3_dn8 = assign21290_e16266_d_n8;
        locals.var_t3_dn9 = assign21290_e16266_d_n9;
        locals.var_t3_dn10 = assign21290_e16266_d_n10;
        locals.var_t3_dn11 = assign21290_e16266_d_n11;
        locals.var_t3_dn14 = assign21290_e16266_d_n14;

        let assign21300_e16269: f64 = if locals.var_t3 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard421 = assign21300_e16269;

        let (assign21310_e16279, assign21310_e16279_d_n0, assign21310_e16279_d_n2, assign21310_e16279_d_n4, assign21310_e16279_d_n5, assign21310_e16279_d_n6, assign21310_e16279_d_n7, assign21310_e16279_d_n8, assign21310_e16279_d_n9, assign21310_e16279_d_n10, assign21310_e16279_d_n11, assign21310_e16279_d_n14,) = {
    if ((((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard420 != 0.0)) && (locals.var_guard421 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign21310_e16279;
        locals.var_t3_dn0 = assign21310_e16279_d_n0;
        locals.var_t3_dn2 = assign21310_e16279_d_n2;
        locals.var_t3_dn4 = assign21310_e16279_d_n4;
        locals.var_t3_dn5 = assign21310_e16279_d_n5;
        locals.var_t3_dn6 = assign21310_e16279_d_n6;
        locals.var_t3_dn7 = assign21310_e16279_d_n7;
        locals.var_t3_dn8 = assign21310_e16279_d_n8;
        locals.var_t3_dn9 = assign21310_e16279_d_n9;
        locals.var_t3_dn10 = assign21310_e16279_d_n10;
        locals.var_t3_dn11 = assign21310_e16279_d_n11;
        locals.var_t3_dn14 = assign21310_e16279_d_n14;

    }

    pub(super) fn stamp_transient_block_52(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign21320_e16289, assign21320_e16289_d_n0, assign21320_e16289_d_n2, assign21320_e16289_d_n4, assign21320_e16289_d_n5, assign21320_e16289_d_n6, assign21320_e16289_d_n7, assign21320_e16289_d_n8, assign21320_e16289_d_n9, assign21320_e16289_d_n10, assign21320_e16289_d_n11, assign21320_e16289_d_n14,) = {
    if ((((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard420 != 0.0)) && (locals.var_guard421 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign21320_e16289;
        locals.var_t4_dn0 = assign21320_e16289_d_n0;
        locals.var_t4_dn2 = assign21320_e16289_d_n2;
        locals.var_t4_dn4 = assign21320_e16289_d_n4;
        locals.var_t4_dn5 = assign21320_e16289_d_n5;
        locals.var_t4_dn6 = assign21320_e16289_d_n6;
        locals.var_t4_dn7 = assign21320_e16289_d_n7;
        locals.var_t4_dn8 = assign21320_e16289_d_n8;
        locals.var_t4_dn9 = assign21320_e16289_d_n9;
        locals.var_t4_dn10 = assign21320_e16289_d_n10;
        locals.var_t4_dn11 = assign21320_e16289_d_n11;
        locals.var_t4_dn14 = assign21320_e16289_d_n14;

        let (assign21330_e16299, assign21330_e16299_d_n0, assign21330_e16299_d_n2, assign21330_e16299_d_n4, assign21330_e16299_d_n5, assign21330_e16299_d_n6, assign21330_e16299_d_n7, assign21330_e16299_d_n8, assign21330_e16299_d_n9, assign21330_e16299_d_n10, assign21330_e16299_d_n11, assign21330_e16299_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard420 != 0.0)) {
        let assign21330_e16297: f64 = (locals.var_t3 + 1e-25);
        (assign21330_e16297, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign21330_e16299;
        locals.var_t3_dn0 = assign21330_e16299_d_n0;
        locals.var_t3_dn2 = assign21330_e16299_d_n2;
        locals.var_t3_dn4 = assign21330_e16299_d_n4;
        locals.var_t3_dn5 = assign21330_e16299_d_n5;
        locals.var_t3_dn6 = assign21330_e16299_d_n6;
        locals.var_t3_dn7 = assign21330_e16299_d_n7;
        locals.var_t3_dn8 = assign21330_e16299_d_n8;
        locals.var_t3_dn9 = assign21330_e16299_d_n9;
        locals.var_t3_dn10 = assign21330_e16299_d_n10;
        locals.var_t3_dn11 = assign21330_e16299_d_n11;
        locals.var_t3_dn14 = assign21330_e16299_d_n14;

        let (assign21340_e16307, assign21340_e16307_d_n0, assign21340_e16307_d_n2, assign21340_e16307_d_n4, assign21340_e16307_d_n5, assign21340_e16307_d_n6, assign21340_e16307_d_n7, assign21340_e16307_d_n8, assign21340_e16307_d_n9, assign21340_e16307_d_n10, assign21340_e16307_d_n11, assign21340_e16307_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard420 != 0.0)) {
        (locals.var_rsdrift, locals.var_rsdrift_dn0, locals.var_rsdrift_dn2, locals.var_rsdrift_dn4, locals.var_rsdrift_dn5, locals.var_rsdrift_dn6, locals.var_rsdrift_dn7, locals.var_rsdrift_dn8, locals.var_rsdrift_dn9, locals.var_rsdrift_dn10, locals.var_rsdrift_dn11, locals.var_rsdrift_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign21340_e16307;
        locals.var_t0_dn0 = assign21340_e16307_d_n0;
        locals.var_t0_dn2 = assign21340_e16307_d_n2;
        locals.var_t0_dn4 = assign21340_e16307_d_n4;
        locals.var_t0_dn5 = assign21340_e16307_d_n5;
        locals.var_t0_dn6 = assign21340_e16307_d_n6;
        locals.var_t0_dn7 = assign21340_e16307_d_n7;
        locals.var_t0_dn8 = assign21340_e16307_d_n8;
        locals.var_t0_dn9 = assign21340_e16307_d_n9;
        locals.var_t0_dn10 = assign21340_e16307_d_n10;
        locals.var_t0_dn11 = assign21340_e16307_d_n11;
        locals.var_t0_dn14 = assign21340_e16307_d_n14;

        let (assign21350_e16317, assign21350_e16317_d_n0, assign21350_e16317_d_n2, assign21350_e16317_d_n4, assign21350_e16317_d_n5, assign21350_e16317_d_n6, assign21350_e16317_d_n7, assign21350_e16317_d_n8, assign21350_e16317_d_n9, assign21350_e16317_d_n10, assign21350_e16317_d_n11, assign21350_e16317_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard420 != 0.0)) {
        let assign21350_e16315: f64 = (locals.var_rsdrift * locals.var_t3);
        (assign21350_e16315, ((locals.var_rsdrift_dn0 * locals.var_t3) + (locals.var_rsdrift * locals.var_t3_dn0)), ((locals.var_rsdrift_dn2 * locals.var_t3) + (locals.var_rsdrift * locals.var_t3_dn2)), ((locals.var_rsdrift_dn4 * locals.var_t3) + (locals.var_rsdrift * locals.var_t3_dn4)), ((locals.var_rsdrift_dn5 * locals.var_t3) + (locals.var_rsdrift * locals.var_t3_dn5)), ((locals.var_rsdrift_dn6 * locals.var_t3) + (locals.var_rsdrift * locals.var_t3_dn6)), ((locals.var_rsdrift_dn7 * locals.var_t3) + (locals.var_rsdrift * locals.var_t3_dn7)), ((locals.var_rsdrift_dn8 * locals.var_t3) + (locals.var_rsdrift * locals.var_t3_dn8)), ((locals.var_rsdrift_dn9 * locals.var_t3) + (locals.var_rsdrift * locals.var_t3_dn9)), ((locals.var_rsdrift_dn10 * locals.var_t3) + (locals.var_rsdrift * locals.var_t3_dn10)), ((locals.var_rsdrift_dn11 * locals.var_t3) + (locals.var_rsdrift * locals.var_t3_dn11)), ((locals.var_rsdrift_dn14 * locals.var_t3) + (locals.var_rsdrift * locals.var_t3_dn14)),)
    } else {
        (locals.var_rsdrift, locals.var_rsdrift_dn0, locals.var_rsdrift_dn2, locals.var_rsdrift_dn4, locals.var_rsdrift_dn5, locals.var_rsdrift_dn6, locals.var_rsdrift_dn7, locals.var_rsdrift_dn8, locals.var_rsdrift_dn9, locals.var_rsdrift_dn10, locals.var_rsdrift_dn11, locals.var_rsdrift_dn14,)
    }
};
        locals.var_rsdrift = assign21350_e16317;
        locals.var_rsdrift_dn0 = assign21350_e16317_d_n0;
        locals.var_rsdrift_dn2 = assign21350_e16317_d_n2;
        locals.var_rsdrift_dn4 = assign21350_e16317_d_n4;
        locals.var_rsdrift_dn5 = assign21350_e16317_d_n5;
        locals.var_rsdrift_dn6 = assign21350_e16317_d_n6;
        locals.var_rsdrift_dn7 = assign21350_e16317_d_n7;
        locals.var_rsdrift_dn8 = assign21350_e16317_d_n8;
        locals.var_rsdrift_dn9 = assign21350_e16317_d_n9;
        locals.var_rsdrift_dn10 = assign21350_e16317_d_n10;
        locals.var_rsdrift_dn11 = assign21350_e16317_d_n11;
        locals.var_rsdrift_dn14 = assign21350_e16317_d_n14;

        let (assign21360_e16326, assign21360_e16326_d_n0, assign21360_e16326_d_n2, assign21360_e16326_d_n4, assign21360_e16326_d_n5, assign21360_e16326_d_n6, assign21360_e16326_d_n7, assign21360_e16326_d_n8, assign21360_e16326_d_n9, assign21360_e16326_d_n10, assign21360_e16326_d_n11, assign21360_e16326_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard420 == 0.0)) {
        (locals.var_rse, locals.var_rse_dn0, locals.var_rse_dn2, locals.var_rse_dn4, locals.var_rse_dn5, locals.var_rse_dn6, locals.var_rse_dn7, locals.var_rse_dn8, locals.var_rse_dn9, locals.var_rse_dn10, locals.var_rse_dn11, locals.var_rse_dn14,)
    } else {
        (locals.var_rsdrift, locals.var_rsdrift_dn0, locals.var_rsdrift_dn2, locals.var_rsdrift_dn4, locals.var_rsdrift_dn5, locals.var_rsdrift_dn6, locals.var_rsdrift_dn7, locals.var_rsdrift_dn8, locals.var_rsdrift_dn9, locals.var_rsdrift_dn10, locals.var_rsdrift_dn11, locals.var_rsdrift_dn14,)
    }
};
        locals.var_rsdrift = assign21360_e16326;
        locals.var_rsdrift_dn0 = assign21360_e16326_d_n0;
        locals.var_rsdrift_dn2 = assign21360_e16326_d_n2;
        locals.var_rsdrift_dn4 = assign21360_e16326_d_n4;
        locals.var_rsdrift_dn5 = assign21360_e16326_d_n5;
        locals.var_rsdrift_dn6 = assign21360_e16326_d_n6;
        locals.var_rsdrift_dn7 = assign21360_e16326_d_n7;
        locals.var_rsdrift_dn8 = assign21360_e16326_d_n8;
        locals.var_rsdrift_dn9 = assign21360_e16326_d_n9;
        locals.var_rsdrift_dn10 = assign21360_e16326_d_n10;
        locals.var_rsdrift_dn11 = assign21360_e16326_d_n11;
        locals.var_rsdrift_dn14 = assign21360_e16326_d_n14;

        let assign21370_e16337: f64 = (locals.var_mks_nsubsub + locals.var_uc_nover);
        let assign21370_e16338: f64 = (locals.var_uc_nover * assign21370_e16337);
        let assign21370_e16341: f64 = if (((p.p54 == 1.0) && (p.p34 == 0.0)) && (assign21370_e16338 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard422 = assign21370_e16341;

        let (assign21380_e16357, assign21380_e16357_d_n0, assign21380_e16357_d_n2, assign21380_e16357_d_n4, assign21380_e16357_d_n5, assign21380_e16357_d_n6, assign21380_e16357_d_n7, assign21380_e16357_d_n8, assign21380_e16357_d_n9, assign21380_e16357_d_n10, assign21380_e16357_d_n11, assign21380_e16357_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard422 != 0.0)) {
        let assign21380_e16350: f64 = (p.p333 * locals.var_vdserevz);
        let assign21380_e16351: f64 = (p.p335 - assign21380_e16350);
        let assign21380_e16354: f64 = (p.p332 * locals.var_vsubsrev);
        let assign21380_e16355: f64 = (assign21380_e16351 - assign21380_e16354);
        (assign21380_e16355, ((-(p.p333 * locals.var_vdserevz_dn0)) - (p.p332 * locals.var_vsubsrev_dn0)), ((-(p.p333 * locals.var_vdserevz_dn2)) - (p.p332 * locals.var_vsubsrev_dn2)), ((-(p.p333 * locals.var_vdserevz_dn4)) - (p.p332 * locals.var_vsubsrev_dn4)), (-(p.p333 * locals.var_vdserevz_dn5)), (-(p.p333 * locals.var_vdserevz_dn6)), (-(p.p333 * locals.var_vdserevz_dn7)), (-(p.p333 * locals.var_vdserevz_dn8)), (-(p.p333 * locals.var_vdserevz_dn9)), (-(p.p333 * locals.var_vdserevz_dn10)), (-(p.p333 * locals.var_vdserevz_dn11)), (-(p.p333 * locals.var_vdserevz_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign21380_e16357;
        locals.var_t0_dn0 = assign21380_e16357_d_n0;
        locals.var_t0_dn2 = assign21380_e16357_d_n2;
        locals.var_t0_dn4 = assign21380_e16357_d_n4;
        locals.var_t0_dn5 = assign21380_e16357_d_n5;
        locals.var_t0_dn6 = assign21380_e16357_d_n6;
        locals.var_t0_dn7 = assign21380_e16357_d_n7;
        locals.var_t0_dn8 = assign21380_e16357_d_n8;
        locals.var_t0_dn9 = assign21380_e16357_d_n9;
        locals.var_t0_dn10 = assign21380_e16357_d_n10;
        locals.var_t0_dn11 = assign21380_e16357_d_n11;
        locals.var_t0_dn14 = assign21380_e16357_d_n14;

        let (assign21390_e16374, assign21390_e16374_d_n0, assign21390_e16374_d_n2, assign21390_e16374_d_n4, assign21390_e16374_d_n5, assign21390_e16374_d_n6, assign21390_e16374_d_n7, assign21390_e16374_d_n8, assign21390_e16374_d_n9, assign21390_e16374_d_n10, assign21390_e16374_d_n11, assign21390_e16374_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard422 != 0.0)) {
        let assign21390_e16365: f64 = (locals.var_t0 * locals.var_t0);
        let assign21390_e16368: f64 = (4.0 * 10.0);
        let assign21390_e16370: f64 = (assign21390_e16368 * 10.0);
        let assign21390_e16371: f64 = (assign21390_e16365 + assign21390_e16370);
        let assign21390_e16372: f64 = (assign21390_e16371).sqrt();
        (assign21390_e16372, (((locals.var_t0_dn0 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn0)) / (2.0 * assign21390_e16372)), (((locals.var_t0_dn2 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn2)) / (2.0 * assign21390_e16372)), (((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)) / (2.0 * assign21390_e16372)), (((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)) / (2.0 * assign21390_e16372)), (((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)) / (2.0 * assign21390_e16372)), (((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)) / (2.0 * assign21390_e16372)), (((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)) / (2.0 * assign21390_e16372)), (((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)) / (2.0 * assign21390_e16372)), (((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)) / (2.0 * assign21390_e16372)), (((locals.var_t0_dn11 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn11)) / (2.0 * assign21390_e16372)), (((locals.var_t0_dn14 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn14)) / (2.0 * assign21390_e16372)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign21390_e16374;
        locals.var_tmf2_dn0 = assign21390_e16374_d_n0;
        locals.var_tmf2_dn2 = assign21390_e16374_d_n2;
        locals.var_tmf2_dn4 = assign21390_e16374_d_n4;
        locals.var_tmf2_dn5 = assign21390_e16374_d_n5;
        locals.var_tmf2_dn6 = assign21390_e16374_d_n6;
        locals.var_tmf2_dn7 = assign21390_e16374_d_n7;
        locals.var_tmf2_dn8 = assign21390_e16374_d_n8;
        locals.var_tmf2_dn9 = assign21390_e16374_d_n9;
        locals.var_tmf2_dn10 = assign21390_e16374_d_n10;
        locals.var_tmf2_dn11 = assign21390_e16374_d_n11;
        locals.var_tmf2_dn14 = assign21390_e16374_d_n14;

        let (assign21400_e16388, assign21400_e16388_d_n0, assign21400_e16388_d_n2, assign21400_e16388_d_n4, assign21400_e16388_d_n5, assign21400_e16388_d_n6, assign21400_e16388_d_n7, assign21400_e16388_d_n8, assign21400_e16388_d_n9, assign21400_e16388_d_n10, assign21400_e16388_d_n11, assign21400_e16388_d_n14,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) && (locals.var_guard422 != 0.0)) {
        let assign21400_e16384: f64 = (locals.var_t0 / locals.var_tmf2);
        let assign21400_e16385: f64 = (1.0 + assign21400_e16384);
        let assign21400_e16386: f64 = (0.5 * assign21400_e16385);
        (assign21400_e16386, (0.5 * (((locals.var_t0_dn0 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn2 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn4 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn5 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn6 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn7 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn8 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn9 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn10 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn11 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn14 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign21400_e16388;
        locals.var_t2_dn0 = assign21400_e16388_d_n0;
        locals.var_t2_dn2 = assign21400_e16388_d_n2;
        locals.var_t2_dn4 = assign21400_e16388_d_n4;
        locals.var_t2_dn5 = assign21400_e16388_d_n5;
        locals.var_t2_dn6 = assign21400_e16388_d_n6;
        locals.var_t2_dn7 = assign21400_e16388_d_n7;
        locals.var_t2_dn8 = assign21400_e16388_d_n8;
        locals.var_t2_dn9 = assign21400_e16388_d_n9;
        locals.var_t2_dn10 = assign21400_e16388_d_n10;
        locals.var_t2_dn11 = assign21400_e16388_d_n11;
        locals.var_t2_dn14 = assign21400_e16388_d_n14;

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

        let assign21420_e16403: f64 = if locals.var_t1 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard423 = assign21420_e16403;

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

    }

    pub(super) fn stamp_transient_block_53(
        locals: &mut StampLocals,
    ) {
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

        let assign21770_e16769: f64 = if locals.var_vbs > locals.var_vbs_bnd { 1.0 } else { 0.0 };
        locals.var_guard426 = assign21770_e16769;

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

    }

    pub(super) fn stamp_transient_block_54(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
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

        let assign22030_e17027: f64 = if locals.var_vzadd < 1e-12 { 1.0 } else { 0.0 };
        locals.var_guard427 = assign22030_e17027;

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

    }

    pub(super) fn stamp_transient_block_55(
        locals: &mut StampLocals,
    ) {
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

        let assign22140_e17088: f64 = if locals.var_t4 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard428 = assign22140_e17088;

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

        let assign22370_e17205: f64 = if locals.var_flg_qmetemp == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard429 = assign22370_e17205;

        let (assign22380_e17209,) = {
    if (locals.var_guard429 != 0.0) {
        (0.0,)
    } else {
        (locals.var_flg_qme,)
    }
};
        locals.var_flg_qme = assign22380_e17209;

        let (assign22390_e17214,) = {
    if (locals.var_guard429 == 0.0) {
        (1.0,)
    } else {
        (locals.var_flg_qme,)
    }
};
        locals.var_flg_qme = assign22390_e17214;

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

        let assign22430_e17229: f64 = if locals.var_flg_qme == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard430 = assign22430_e17229;

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

    }

    pub(super) fn stamp_transient_block_56(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
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

        let assign22530_e17307: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard431 = assign22530_e17307;

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

        let assign22610_e17365: f64 = if locals.var_t6 > locals.var_t4 { 1.0 } else { 0.0 };
        locals.var_guard432 = assign22610_e17365;

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

    }

    pub(super) fn stamp_transient_block_57(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
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

        let assign22710_e17460: f64 = (locals.var_dtox * 1000000000000.0);
        let assign22710_e17462: f64 = if assign22710_e17460 < locals.var_tox0 { 1.0 } else { 0.0 };
        locals.var_guard433 = assign22710_e17462;

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

        let (assign22730_e17476,) = {
    if ((locals.var_guard430 == 0.0) && (locals.var_guard433 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_qme,)
    }
};
        locals.var_flg_qme = assign22730_e17476;

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

        let assign22950_e17608: f64 = if p.p140 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard434 = assign22950_e17608;

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

    }

    pub(super) fn stamp_transient_block_58(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
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

    }

    pub(super) fn stamp_transient_block_59(
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

    }

    pub(super) fn stamp_transient_block_60(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
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

        let assign23550_e18050: f64 = (locals.var_pb2 + locals.var_vfb);
        let assign23550_e18053: f64 = (locals.var_t2 * locals.var_cox0_inv);
        let assign23550_e18054: f64 = (assign23550_e18050 + assign23550_e18053);
        let assign23550_e18056: f64 = (assign23550_e18054 - locals.var_dvth);
        locals.var_vth = assign23550_e18056;

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

        let assign23590_e18066: f64 = if locals.var_flg_pgd == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard435 = assign23590_e18066;

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

        let assign23630_e18083: f64 = (-3.0);
        let assign23630_e18084: f64 = if locals.var_t3 < assign23630_e18083 { 1.0 } else { 0.0 };
        locals.var_guard436 = assign23630_e18084;

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

        let assign23660_e18099: f64 = if locals.var_t3 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard437 = assign23660_e18099;

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

        let assign23740_e18249: f64 = if locals.var_dppg < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard438 = assign23740_e18249;

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

    }

    pub(super) fn stamp_transient_block_61(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
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

        let assign23840_e18325: f64 = if locals.var_vbs > locals.var_vbs_bnd_local { 1.0 } else { 0.0 };
        locals.var_guard445 = assign23840_e18325;

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

    }

    pub(super) fn stamp_transient_block_62(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
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

        let assign24120_e18631: f64 = if locals.var_vzadd__blk441 < 1e-12 { 1.0 } else { 0.0 };
        locals.var_guard446 = assign24120_e18631;

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

        let assign24180_e18656: f64 = (locals.var_vfb - locals.var_dvth);
        let assign24180_e18658: f64 = (assign24180_e18656 + locals.var_dppg);
        let assign24180_e18660: f64 = (assign24180_e18658 + locals.var_vbscl__blk439);
        locals.var_vgs_fb = assign24180_e18660;

        let assign24190_e18663: f64 = if locals.var_uc_codep != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard447 = assign24190_e18663;

        let assign24200_e18666: f64 = if p.p42 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard448 = assign24200_e18666;

        let assign24210_e18669: f64 = if p.p42 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard449 = assign24210_e18669;

        let assign24220_e18672: f64 = if p.p42 == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard450 = assign24220_e18672;

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

        let (assign24280_e18720,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign24280_e18718: f64 = (1.6021918e-19 * 1.6021918e-19);
        (assign24280_e18718,)
    } else {
        (locals.var_c_qe2,)
    }
};
        locals.var_c_qe2 = assign24280_e18720;

        let (assign24290_e18728,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign24290_e18726: f64 = (1.034943e-10 * 1.034943e-10);
        (assign24290_e18726,)
    } else {
        (locals.var_c_esi2,)
    }
};
        locals.var_c_esi2 = assign24290_e18728;

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

    }

    pub(super) fn stamp_transient_block_63(
        locals: &mut StampLocals,
    ) {
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

        let (assign24380_e18812,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign24380_e18810: f64 = (1e-12 * 1000.0);
        (assign24380_e18810,)
    } else {
        (locals.var_ps_conv3,)
    }
};
        locals.var_ps_conv3 = assign24380_e18812;

        let (assign24390_e18820,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign24390_e18818: f64 = (1e-10 * 1000.0);
        (assign24390_e18818,)
    } else {
        (locals.var_ps_conv23,)
    }
};
        locals.var_ps_conv23 = assign24390_e18820;

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

        let assign24550_e18943: f64 = if locals.var_w_bsub0 > locals.var_uc_depthn { 1.0 } else { 0.0 };
        locals.var_guard563 = assign24550_e18943;

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

    }
}
