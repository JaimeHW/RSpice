#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_160(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign47360_e80247, assign47360_e80247_d_n3, assign47360_e80247_d_n4, assign47360_e80247_d_n5, assign47360_e80247_d_n6, assign47360_e80247_d_n7, assign47360_e80247_d_n8, assign47360_e80247_d_n9, assign47360_e80247_d_n10, assign47360_e80247_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard734 != 0.0)) {
        let assign47360_e80244: f64 = (p.p433 * locals.var_leff);
        let assign47360_e80245: f64 = (1.0 + assign47360_e80244);
        (assign47360_e80245, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign47360_e80247;
        locals.var_t2_dn3 = assign47360_e80247_d_n3;
        locals.var_t2_dn4 = assign47360_e80247_d_n4;
        locals.var_t2_dn5 = assign47360_e80247_d_n5;
        locals.var_t2_dn6 = assign47360_e80247_d_n6;
        locals.var_t2_dn7 = assign47360_e80247_d_n7;
        locals.var_t2_dn8 = assign47360_e80247_d_n8;
        locals.var_t2_dn9 = assign47360_e80247_d_n9;
        locals.var_t2_dn10 = assign47360_e80247_d_n10;
        locals.var_t2_dn11 = assign47360_e80247_d_n11;

        let (assign47370_e80260, assign47370_e80260_d_n3, assign47370_e80260_d_n4, assign47370_e80260_d_n5, assign47370_e80260_d_n6, assign47370_e80260_d_n7, assign47370_e80260_d_n8, assign47370_e80260_d_n9, assign47370_e80260_d_n10, assign47370_e80260_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard734 != 0.0)) {
        let assign47370_e80255: f64 = (locals.var_t2 * locals.var_t1);
        let assign47370_e80256: f64 = (1.0 + assign47370_e80255);
        let assign47370_e80258: f64 = (assign47370_e80256 / locals.var_pdits_i);
        (assign47370_e80258, (((locals.var_t2_dn3 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn3)) / locals.var_pdits_i), (((locals.var_t2_dn4 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn4)) / locals.var_pdits_i), (((locals.var_t2_dn5 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn5)) / locals.var_pdits_i), (((locals.var_t2_dn6 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn6)) / locals.var_pdits_i), (((locals.var_t2_dn7 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn7)) / locals.var_pdits_i), (((locals.var_t2_dn8 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn8)) / locals.var_pdits_i), (((locals.var_t2_dn9 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn9)) / locals.var_pdits_i), (((locals.var_t2_dn10 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn10)) / locals.var_pdits_i), (((locals.var_t2_dn11 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn11)) / locals.var_pdits_i),)
    } else {
        (locals.var_vadits, locals.var_vadits_dn3, locals.var_vadits_dn4, locals.var_vadits_dn5, locals.var_vadits_dn6, locals.var_vadits_dn7, locals.var_vadits_dn8, locals.var_vadits_dn9, locals.var_vadits_dn10, locals.var_vadits_dn11,)
    }
};
        locals.var_vadits = assign47370_e80260;
        locals.var_vadits_dn3 = assign47370_e80260_d_n3;
        locals.var_vadits_dn4 = assign47370_e80260_d_n4;
        locals.var_vadits_dn5 = assign47370_e80260_d_n5;
        locals.var_vadits_dn6 = assign47370_e80260_d_n6;
        locals.var_vadits_dn7 = assign47370_e80260_d_n7;
        locals.var_vadits_dn8 = assign47370_e80260_d_n8;
        locals.var_vadits_dn9 = assign47370_e80260_d_n9;
        locals.var_vadits_dn10 = assign47370_e80260_d_n10;
        locals.var_vadits_dn11 = assign47370_e80260_d_n11;

        let (assign47380_e80269, assign47380_e80269_d_n3, assign47380_e80269_d_n4, assign47380_e80269_d_n5, assign47380_e80269_d_n6, assign47380_e80269_d_n7, assign47380_e80269_d_n8, assign47380_e80269_d_n9, assign47380_e80269_d_n10, assign47380_e80269_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard734 != 0.0)) {
        let assign47380_e80267: f64 = (locals.var_vadits * locals.var_fp);
        (assign47380_e80267, ((locals.var_vadits_dn3 * locals.var_fp) + (locals.var_vadits * locals.var_fp_dn3)), ((locals.var_vadits_dn4 * locals.var_fp) + (locals.var_vadits * locals.var_fp_dn4)), ((locals.var_vadits_dn5 * locals.var_fp) + (locals.var_vadits * locals.var_fp_dn5)), ((locals.var_vadits_dn6 * locals.var_fp) + (locals.var_vadits * locals.var_fp_dn6)), ((locals.var_vadits_dn7 * locals.var_fp) + (locals.var_vadits * locals.var_fp_dn7)), ((locals.var_vadits_dn8 * locals.var_fp) + (locals.var_vadits * locals.var_fp_dn8)), ((locals.var_vadits_dn9 * locals.var_fp) + (locals.var_vadits * locals.var_fp_dn9)), ((locals.var_vadits_dn10 * locals.var_fp) + (locals.var_vadits * locals.var_fp_dn10)), ((locals.var_vadits_dn11 * locals.var_fp) + (locals.var_vadits * locals.var_fp_dn11)),)
    } else {
        (locals.var_vadits, locals.var_vadits_dn3, locals.var_vadits_dn4, locals.var_vadits_dn5, locals.var_vadits_dn6, locals.var_vadits_dn7, locals.var_vadits_dn8, locals.var_vadits_dn9, locals.var_vadits_dn10, locals.var_vadits_dn11,)
    }
};
        locals.var_vadits = assign47380_e80269;
        locals.var_vadits_dn3 = assign47380_e80269_d_n3;
        locals.var_vadits_dn4 = assign47380_e80269_d_n4;
        locals.var_vadits_dn5 = assign47380_e80269_d_n5;
        locals.var_vadits_dn6 = assign47380_e80269_d_n6;
        locals.var_vadits_dn7 = assign47380_e80269_d_n7;
        locals.var_vadits_dn8 = assign47380_e80269_d_n8;
        locals.var_vadits_dn9 = assign47380_e80269_d_n9;
        locals.var_vadits_dn10 = assign47380_e80269_d_n10;
        locals.var_vadits_dn11 = assign47380_e80269_d_n11;

        let (assign47390_e80277, assign47390_e80277_d_n3, assign47390_e80277_d_n4, assign47390_e80277_d_n5, assign47390_e80277_d_n6, assign47390_e80277_d_n7, assign47390_e80277_d_n8, assign47390_e80277_d_n9, assign47390_e80277_d_n10, assign47390_e80277_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard734 == 0.0)) {
        (5.540622384e34, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vadits, locals.var_vadits_dn3, locals.var_vadits_dn4, locals.var_vadits_dn5, locals.var_vadits_dn6, locals.var_vadits_dn7, locals.var_vadits_dn8, locals.var_vadits_dn9, locals.var_vadits_dn10, locals.var_vadits_dn11,)
    }
};
        locals.var_vadits = assign47390_e80277;
        locals.var_vadits_dn3 = assign47390_e80277_d_n3;
        locals.var_vadits_dn4 = assign47390_e80277_d_n4;
        locals.var_vadits_dn5 = assign47390_e80277_d_n5;
        locals.var_vadits_dn6 = assign47390_e80277_d_n6;
        locals.var_vadits_dn7 = assign47390_e80277_d_n7;
        locals.var_vadits_dn8 = assign47390_e80277_d_n8;
        locals.var_vadits_dn9 = assign47390_e80277_d_n9;
        locals.var_vadits_dn10 = assign47390_e80277_d_n10;
        locals.var_vadits_dn11 = assign47390_e80277_d_n11;

        let (assign47400_e80284, assign47400_e80284_d_n3, assign47400_e80284_d_n4, assign47400_e80284_d_n5, assign47400_e80284_d_n6, assign47400_e80284_d_n7, assign47400_e80284_d_n8, assign47400_e80284_d_n9, assign47400_e80284_d_n10, assign47400_e80284_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign47400_e80282: f64 = (locals.var_diffvds / locals.var_vadits);
        (assign47400_e80282, (((locals.var_diffvds_dn3 * locals.var_vadits) - (locals.var_diffvds * locals.var_vadits_dn3)) / (locals.var_vadits * locals.var_vadits)), (((locals.var_diffvds_dn4 * locals.var_vadits) - (locals.var_diffvds * locals.var_vadits_dn4)) / (locals.var_vadits * locals.var_vadits)), (((locals.var_diffvds_dn5 * locals.var_vadits) - (locals.var_diffvds * locals.var_vadits_dn5)) / (locals.var_vadits * locals.var_vadits)), (((locals.var_diffvds_dn6 * locals.var_vadits) - (locals.var_diffvds * locals.var_vadits_dn6)) / (locals.var_vadits * locals.var_vadits)), (((locals.var_diffvds_dn7 * locals.var_vadits) - (locals.var_diffvds * locals.var_vadits_dn7)) / (locals.var_vadits * locals.var_vadits)), (((locals.var_diffvds_dn8 * locals.var_vadits) - (locals.var_diffvds * locals.var_vadits_dn8)) / (locals.var_vadits * locals.var_vadits)), (((locals.var_diffvds_dn9 * locals.var_vadits) - (locals.var_diffvds * locals.var_vadits_dn9)) / (locals.var_vadits * locals.var_vadits)), (((locals.var_diffvds_dn10 * locals.var_vadits) - (locals.var_diffvds * locals.var_vadits_dn10)) / (locals.var_vadits * locals.var_vadits)), (((locals.var_diffvds_dn11 * locals.var_vadits) - (locals.var_diffvds * locals.var_vadits_dn11)) / (locals.var_vadits * locals.var_vadits)),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign47400_e80284;
        locals.var_t4_dn3 = assign47400_e80284_d_n3;
        locals.var_t4_dn4 = assign47400_e80284_d_n4;
        locals.var_t4_dn5 = assign47400_e80284_d_n5;
        locals.var_t4_dn6 = assign47400_e80284_d_n6;
        locals.var_t4_dn7 = assign47400_e80284_d_n7;
        locals.var_t4_dn8 = assign47400_e80284_d_n8;
        locals.var_t4_dn9 = assign47400_e80284_d_n9;
        locals.var_t4_dn10 = assign47400_e80284_d_n10;
        locals.var_t4_dn11 = assign47400_e80284_d_n11;

        let (assign47410_e80291, assign47410_e80291_d_n3, assign47410_e80291_d_n4, assign47410_e80291_d_n5, assign47410_e80291_d_n6, assign47410_e80291_d_n7, assign47410_e80291_d_n8, assign47410_e80291_d_n9, assign47410_e80291_d_n10, assign47410_e80291_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign47410_e80289: f64 = (1.0 + locals.var_t4);
        (assign47410_e80289, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign47410_e80291;
        locals.var_t0_dn3 = assign47410_e80291_d_n3;
        locals.var_t0_dn4 = assign47410_e80291_d_n4;
        locals.var_t0_dn5 = assign47410_e80291_d_n5;
        locals.var_t0_dn6 = assign47410_e80291_d_n6;
        locals.var_t0_dn7 = assign47410_e80291_d_n7;
        locals.var_t0_dn8 = assign47410_e80291_d_n8;
        locals.var_t0_dn9 = assign47410_e80291_d_n9;
        locals.var_t0_dn10 = assign47410_e80291_d_n10;
        locals.var_t0_dn11 = assign47410_e80291_d_n11;

        let (assign47420_e80298, assign47420_e80298_d_n3, assign47420_e80298_d_n4, assign47420_e80298_d_n5, assign47420_e80298_d_n6, assign47420_e80298_d_n7, assign47420_e80298_d_n8, assign47420_e80298_d_n9, assign47420_e80298_d_n10, assign47420_e80298_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign47420_e80296: f64 = (locals.var_moc * locals.var_t0);
        (assign47420_e80296, ((locals.var_moc_dn3 * locals.var_t0) + (locals.var_moc * locals.var_t0_dn3)), ((locals.var_moc_dn4 * locals.var_t0) + (locals.var_moc * locals.var_t0_dn4)), ((locals.var_moc_dn5 * locals.var_t0) + (locals.var_moc * locals.var_t0_dn5)), ((locals.var_moc_dn6 * locals.var_t0) + (locals.var_moc * locals.var_t0_dn6)), ((locals.var_moc_dn7 * locals.var_t0) + (locals.var_moc * locals.var_t0_dn7)), ((locals.var_moc_dn8 * locals.var_t0) + (locals.var_moc * locals.var_t0_dn8)), ((locals.var_moc_dn9 * locals.var_t0) + (locals.var_moc * locals.var_t0_dn9)), ((locals.var_moc_dn10 * locals.var_t0) + (locals.var_moc * locals.var_t0_dn10)), ((locals.var_moc_dn11 * locals.var_t0) + (locals.var_moc * locals.var_t0_dn11)),)
    } else {
        (locals.var_moc, locals.var_moc_dn3, locals.var_moc_dn4, locals.var_moc_dn5, locals.var_moc_dn6, locals.var_moc_dn7, locals.var_moc_dn8, locals.var_moc_dn9, locals.var_moc_dn10, locals.var_moc_dn11,)
    }
};
        locals.var_moc = assign47420_e80298;
        locals.var_moc_dn3 = assign47420_e80298_d_n3;
        locals.var_moc_dn4 = assign47420_e80298_d_n4;
        locals.var_moc_dn5 = assign47420_e80298_d_n5;
        locals.var_moc_dn6 = assign47420_e80298_d_n6;
        locals.var_moc_dn7 = assign47420_e80298_d_n7;
        locals.var_moc_dn8 = assign47420_e80298_d_n8;
        locals.var_moc_dn9 = assign47420_e80298_d_n9;
        locals.var_moc_dn10 = assign47420_e80298_d_n10;
        locals.var_moc_dn11 = assign47420_e80298_d_n11;

        let assign47430_e80301: f64 = if locals.var_pscbe2_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard735 = assign47430_e80301;

        let assign47440_e80305: f64 = (locals.var_pscbe1_i * locals.var_litl);
        let assign47440_e80307: f64 = (assign47440_e80305 / 80.0);
        let assign47440_e80308: f64 = if locals.var_diffvds > assign47440_e80307 { 1.0 } else { 0.0 };
        locals.var_guard736 = assign47440_e80308;

        let (assign47450_e80321, assign47450_e80321_d_n3, assign47450_e80321_d_n4, assign47450_e80321_d_n5, assign47450_e80321_d_n6, assign47450_e80321_d_n7, assign47450_e80321_d_n8, assign47450_e80321_d_n9, assign47450_e80321_d_n10, assign47450_e80321_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard735 != 0.0)) && (locals.var_guard736 != 0.0)) {
        let assign47450_e80317: f64 = (locals.var_pscbe1_i * locals.var_litl);
        let assign47450_e80319: f64 = (assign47450_e80317 / locals.var_diffvds);
        (assign47450_e80319, (-((assign47450_e80317 * locals.var_diffvds_dn3) / (locals.var_diffvds * locals.var_diffvds))), (-((assign47450_e80317 * locals.var_diffvds_dn4) / (locals.var_diffvds * locals.var_diffvds))), (-((assign47450_e80317 * locals.var_diffvds_dn5) / (locals.var_diffvds * locals.var_diffvds))), (-((assign47450_e80317 * locals.var_diffvds_dn6) / (locals.var_diffvds * locals.var_diffvds))), (-((assign47450_e80317 * locals.var_diffvds_dn7) / (locals.var_diffvds * locals.var_diffvds))), (-((assign47450_e80317 * locals.var_diffvds_dn8) / (locals.var_diffvds * locals.var_diffvds))), (-((assign47450_e80317 * locals.var_diffvds_dn9) / (locals.var_diffvds * locals.var_diffvds))), (-((assign47450_e80317 * locals.var_diffvds_dn10) / (locals.var_diffvds * locals.var_diffvds))), (-((assign47450_e80317 * locals.var_diffvds_dn11) / (locals.var_diffvds * locals.var_diffvds))),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign47450_e80321;
        locals.var_t0_dn3 = assign47450_e80321_d_n3;
        locals.var_t0_dn4 = assign47450_e80321_d_n4;
        locals.var_t0_dn5 = assign47450_e80321_d_n5;
        locals.var_t0_dn6 = assign47450_e80321_d_n6;
        locals.var_t0_dn7 = assign47450_e80321_d_n7;
        locals.var_t0_dn8 = assign47450_e80321_d_n8;
        locals.var_t0_dn9 = assign47450_e80321_d_n9;
        locals.var_t0_dn10 = assign47450_e80321_d_n10;
        locals.var_t0_dn11 = assign47450_e80321_d_n11;

        let (assign47460_e80335, assign47460_e80335_d_n3, assign47460_e80335_d_n4, assign47460_e80335_d_n5, assign47460_e80335_d_n6, assign47460_e80335_d_n7, assign47460_e80335_d_n8, assign47460_e80335_d_n9, assign47460_e80335_d_n10, assign47460_e80335_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard735 != 0.0)) && (locals.var_guard736 != 0.0)) {
        let assign47460_e80330: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign47460_e80331: f64 = (locals.var_leff * assign47460_e80330);
        let assign47460_e80333: f64 = (assign47460_e80331 / locals.var_pscbe2_i);
        (assign47460_e80333, ((locals.var_leff * ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn3)) / locals.var_pscbe2_i), ((locals.var_leff * ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4)) / locals.var_pscbe2_i), ((locals.var_leff * ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5)) / locals.var_pscbe2_i), ((locals.var_leff * ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6)) / locals.var_pscbe2_i), ((locals.var_leff * ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7)) / locals.var_pscbe2_i), ((locals.var_leff * ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8)) / locals.var_pscbe2_i), ((locals.var_leff * ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9)) / locals.var_pscbe2_i), ((locals.var_leff * ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10)) / locals.var_pscbe2_i), ((locals.var_leff * ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11)) / locals.var_pscbe2_i),)
    } else {
        (locals.var_vascbe, locals.var_vascbe_dn3, locals.var_vascbe_dn4, locals.var_vascbe_dn5, locals.var_vascbe_dn6, locals.var_vascbe_dn7, locals.var_vascbe_dn8, locals.var_vascbe_dn9, locals.var_vascbe_dn10, locals.var_vascbe_dn11,)
    }
};
        locals.var_vascbe = assign47460_e80335;
        locals.var_vascbe_dn3 = assign47460_e80335_d_n3;
        locals.var_vascbe_dn4 = assign47460_e80335_d_n4;
        locals.var_vascbe_dn5 = assign47460_e80335_d_n5;
        locals.var_vascbe_dn6 = assign47460_e80335_d_n6;
        locals.var_vascbe_dn7 = assign47460_e80335_d_n7;
        locals.var_vascbe_dn8 = assign47460_e80335_d_n8;
        locals.var_vascbe_dn9 = assign47460_e80335_d_n9;
        locals.var_vascbe_dn10 = assign47460_e80335_d_n10;
        locals.var_vascbe_dn11 = assign47460_e80335_d_n11;

        let (assign47470_e80349, assign47470_e80349_d_n3, assign47470_e80349_d_n4, assign47470_e80349_d_n5, assign47470_e80349_d_n6, assign47470_e80349_d_n7, assign47470_e80349_d_n8, assign47470_e80349_d_n9, assign47470_e80349_d_n10, assign47470_e80349_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard735 != 0.0)) && (locals.var_guard736 == 0.0)) {
        let assign47470_e80345: f64 = (5.540622384e34 * locals.var_leff);
        let assign47470_e80347: f64 = (assign47470_e80345 / locals.var_pscbe2_i);
        (assign47470_e80347, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vascbe, locals.var_vascbe_dn3, locals.var_vascbe_dn4, locals.var_vascbe_dn5, locals.var_vascbe_dn6, locals.var_vascbe_dn7, locals.var_vascbe_dn8, locals.var_vascbe_dn9, locals.var_vascbe_dn10, locals.var_vascbe_dn11,)
    }
};
        locals.var_vascbe = assign47470_e80349;
        locals.var_vascbe_dn3 = assign47470_e80349_d_n3;
        locals.var_vascbe_dn4 = assign47470_e80349_d_n4;
        locals.var_vascbe_dn5 = assign47470_e80349_d_n5;
        locals.var_vascbe_dn6 = assign47470_e80349_d_n6;
        locals.var_vascbe_dn7 = assign47470_e80349_d_n7;
        locals.var_vascbe_dn8 = assign47470_e80349_d_n8;
        locals.var_vascbe_dn9 = assign47470_e80349_d_n9;
        locals.var_vascbe_dn10 = assign47470_e80349_d_n10;
        locals.var_vascbe_dn11 = assign47470_e80349_d_n11;

        let (assign47480_e80357, assign47480_e80357_d_n3, assign47480_e80357_d_n4, assign47480_e80357_d_n5, assign47480_e80357_d_n6, assign47480_e80357_d_n7, assign47480_e80357_d_n8, assign47480_e80357_d_n9, assign47480_e80357_d_n10, assign47480_e80357_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard735 == 0.0)) {
        (5.540622384e34, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vascbe, locals.var_vascbe_dn3, locals.var_vascbe_dn4, locals.var_vascbe_dn5, locals.var_vascbe_dn6, locals.var_vascbe_dn7, locals.var_vascbe_dn8, locals.var_vascbe_dn9, locals.var_vascbe_dn10, locals.var_vascbe_dn11,)
    }
};
        locals.var_vascbe = assign47480_e80357;
        locals.var_vascbe_dn3 = assign47480_e80357_d_n3;
        locals.var_vascbe_dn4 = assign47480_e80357_d_n4;
        locals.var_vascbe_dn5 = assign47480_e80357_d_n5;
        locals.var_vascbe_dn6 = assign47480_e80357_d_n6;
        locals.var_vascbe_dn7 = assign47480_e80357_d_n7;
        locals.var_vascbe_dn8 = assign47480_e80357_d_n8;
        locals.var_vascbe_dn9 = assign47480_e80357_d_n9;
        locals.var_vascbe_dn10 = assign47480_e80357_d_n10;
        locals.var_vascbe_dn11 = assign47480_e80357_d_n11;

        let (assign47490_e80366, assign47490_e80366_d_n3, assign47490_e80366_d_n4, assign47490_e80366_d_n5, assign47490_e80366_d_n6, assign47490_e80366_d_n7, assign47490_e80366_d_n8, assign47490_e80366_d_n9, assign47490_e80366_d_n10, assign47490_e80366_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign47490_e80363: f64 = (locals.var_diffvds / locals.var_vascbe);
        let assign47490_e80364: f64 = (1.0 + assign47490_e80363);
        (assign47490_e80364, (((locals.var_diffvds_dn3 * locals.var_vascbe) - (locals.var_diffvds * locals.var_vascbe_dn3)) / (locals.var_vascbe * locals.var_vascbe)), (((locals.var_diffvds_dn4 * locals.var_vascbe) - (locals.var_diffvds * locals.var_vascbe_dn4)) / (locals.var_vascbe * locals.var_vascbe)), (((locals.var_diffvds_dn5 * locals.var_vascbe) - (locals.var_diffvds * locals.var_vascbe_dn5)) / (locals.var_vascbe * locals.var_vascbe)), (((locals.var_diffvds_dn6 * locals.var_vascbe) - (locals.var_diffvds * locals.var_vascbe_dn6)) / (locals.var_vascbe * locals.var_vascbe)), (((locals.var_diffvds_dn7 * locals.var_vascbe) - (locals.var_diffvds * locals.var_vascbe_dn7)) / (locals.var_vascbe * locals.var_vascbe)), (((locals.var_diffvds_dn8 * locals.var_vascbe) - (locals.var_diffvds * locals.var_vascbe_dn8)) / (locals.var_vascbe * locals.var_vascbe)), (((locals.var_diffvds_dn9 * locals.var_vascbe) - (locals.var_diffvds * locals.var_vascbe_dn9)) / (locals.var_vascbe * locals.var_vascbe)), (((locals.var_diffvds_dn10 * locals.var_vascbe) - (locals.var_diffvds * locals.var_vascbe_dn10)) / (locals.var_vascbe * locals.var_vascbe)), (((locals.var_diffvds_dn11 * locals.var_vascbe) - (locals.var_diffvds * locals.var_vascbe_dn11)) / (locals.var_vascbe * locals.var_vascbe)),)
    } else {
        (locals.var_mscbe, locals.var_mscbe_dn3, locals.var_mscbe_dn4, locals.var_mscbe_dn5, locals.var_mscbe_dn6, locals.var_mscbe_dn7, locals.var_mscbe_dn8, locals.var_mscbe_dn9, locals.var_mscbe_dn10, locals.var_mscbe_dn11,)
    }
};
        locals.var_mscbe = assign47490_e80366;
        locals.var_mscbe_dn3 = assign47490_e80366_d_n3;
        locals.var_mscbe_dn4 = assign47490_e80366_d_n4;
        locals.var_mscbe_dn5 = assign47490_e80366_d_n5;
        locals.var_mscbe_dn6 = assign47490_e80366_d_n6;
        locals.var_mscbe_dn7 = assign47490_e80366_d_n7;
        locals.var_mscbe_dn8 = assign47490_e80366_d_n8;
        locals.var_mscbe_dn9 = assign47490_e80366_d_n9;
        locals.var_mscbe_dn10 = assign47490_e80366_d_n10;
        locals.var_mscbe_dn11 = assign47490_e80366_d_n11;

        let (assign47500_e80373, assign47500_e80373_d_n3, assign47500_e80373_d_n4, assign47500_e80373_d_n5, assign47500_e80373_d_n6, assign47500_e80373_d_n7, assign47500_e80373_d_n8, assign47500_e80373_d_n9, assign47500_e80373_d_n10, assign47500_e80373_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign47500_e80371: f64 = (locals.var_moc * locals.var_mscbe);
        (assign47500_e80371, ((locals.var_moc_dn3 * locals.var_mscbe) + (locals.var_moc * locals.var_mscbe_dn3)), ((locals.var_moc_dn4 * locals.var_mscbe) + (locals.var_moc * locals.var_mscbe_dn4)), ((locals.var_moc_dn5 * locals.var_mscbe) + (locals.var_moc * locals.var_mscbe_dn5)), ((locals.var_moc_dn6 * locals.var_mscbe) + (locals.var_moc * locals.var_mscbe_dn6)), ((locals.var_moc_dn7 * locals.var_mscbe) + (locals.var_moc * locals.var_mscbe_dn7)), ((locals.var_moc_dn8 * locals.var_mscbe) + (locals.var_moc * locals.var_mscbe_dn8)), ((locals.var_moc_dn9 * locals.var_mscbe) + (locals.var_moc * locals.var_mscbe_dn9)), ((locals.var_moc_dn10 * locals.var_mscbe) + (locals.var_moc * locals.var_mscbe_dn10)), ((locals.var_moc_dn11 * locals.var_mscbe) + (locals.var_moc * locals.var_mscbe_dn11)),)
    } else {
        (locals.var_moc, locals.var_moc_dn3, locals.var_moc_dn4, locals.var_moc_dn5, locals.var_moc_dn6, locals.var_moc_dn7, locals.var_moc_dn8, locals.var_moc_dn9, locals.var_moc_dn10, locals.var_moc_dn11,)
    }
};
        locals.var_moc = assign47500_e80373;
        locals.var_moc_dn3 = assign47500_e80373_d_n3;
        locals.var_moc_dn4 = assign47500_e80373_d_n4;
        locals.var_moc_dn5 = assign47500_e80373_d_n5;
        locals.var_moc_dn6 = assign47500_e80373_d_n6;
        locals.var_moc_dn7 = assign47500_e80373_d_n7;
        locals.var_moc_dn8 = assign47500_e80373_d_n8;
        locals.var_moc_dn9 = assign47500_e80373_d_n9;
        locals.var_moc_dn10 = assign47500_e80373_d_n10;
        locals.var_moc_dn11 = assign47500_e80373_d_n11;

        let (assign47510_e80382, assign47510_e80382_d_n3, assign47510_e80382_d_n4, assign47510_e80382_d_n5, assign47510_e80382_d_n6, assign47510_e80382_d_n7, assign47510_e80382_d_n8, assign47510_e80382_d_n9, assign47510_e80382_d_n10, assign47510_e80382_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign47510_e80379: f64 = (1.0 / locals.var_psat_a);
        let assign47510_e80380: f64 = (locals.var_dmob).powf(assign47510_e80379);
        (assign47510_e80380, if (-(locals.var_psat_a_dn3 / (locals.var_psat_a * locals.var_psat_a))) == 0.0 && ((assign47510_e80379) as f64).is_finite() && ((assign47510_e80379) as f64).fract() == 0.0 { if assign47510_e80379 == 0.0 { 0.0 } else { (assign47510_e80379 * ((locals.var_dmob).powf(assign47510_e80379 - 1.0) * locals.var_dmob_dn3)) } } else { (assign47510_e80380 * (((-(locals.var_psat_a_dn3 / (locals.var_psat_a * locals.var_psat_a))) * (locals.var_dmob).ln()) + (assign47510_e80379 * (locals.var_dmob_dn3 / locals.var_dmob)))) }, if (-(locals.var_psat_a_dn4 / (locals.var_psat_a * locals.var_psat_a))) == 0.0 && ((assign47510_e80379) as f64).is_finite() && ((assign47510_e80379) as f64).fract() == 0.0 { if assign47510_e80379 == 0.0 { 0.0 } else { (assign47510_e80379 * ((locals.var_dmob).powf(assign47510_e80379 - 1.0) * locals.var_dmob_dn4)) } } else { (assign47510_e80380 * (((-(locals.var_psat_a_dn4 / (locals.var_psat_a * locals.var_psat_a))) * (locals.var_dmob).ln()) + (assign47510_e80379 * (locals.var_dmob_dn4 / locals.var_dmob)))) }, if (-(locals.var_psat_a_dn5 / (locals.var_psat_a * locals.var_psat_a))) == 0.0 && ((assign47510_e80379) as f64).is_finite() && ((assign47510_e80379) as f64).fract() == 0.0 { if assign47510_e80379 == 0.0 { 0.0 } else { (assign47510_e80379 * ((locals.var_dmob).powf(assign47510_e80379 - 1.0) * locals.var_dmob_dn5)) } } else { (assign47510_e80380 * (((-(locals.var_psat_a_dn5 / (locals.var_psat_a * locals.var_psat_a))) * (locals.var_dmob).ln()) + (assign47510_e80379 * (locals.var_dmob_dn5 / locals.var_dmob)))) }, if (-(locals.var_psat_a_dn6 / (locals.var_psat_a * locals.var_psat_a))) == 0.0 && ((assign47510_e80379) as f64).is_finite() && ((assign47510_e80379) as f64).fract() == 0.0 { if assign47510_e80379 == 0.0 { 0.0 } else { (assign47510_e80379 * ((locals.var_dmob).powf(assign47510_e80379 - 1.0) * locals.var_dmob_dn6)) } } else { (assign47510_e80380 * (((-(locals.var_psat_a_dn6 / (locals.var_psat_a * locals.var_psat_a))) * (locals.var_dmob).ln()) + (assign47510_e80379 * (locals.var_dmob_dn6 / locals.var_dmob)))) }, if (-(locals.var_psat_a_dn7 / (locals.var_psat_a * locals.var_psat_a))) == 0.0 && ((assign47510_e80379) as f64).is_finite() && ((assign47510_e80379) as f64).fract() == 0.0 { if assign47510_e80379 == 0.0 { 0.0 } else { (assign47510_e80379 * ((locals.var_dmob).powf(assign47510_e80379 - 1.0) * locals.var_dmob_dn7)) } } else { (assign47510_e80380 * (((-(locals.var_psat_a_dn7 / (locals.var_psat_a * locals.var_psat_a))) * (locals.var_dmob).ln()) + (assign47510_e80379 * (locals.var_dmob_dn7 / locals.var_dmob)))) }, if (-(locals.var_psat_a_dn8 / (locals.var_psat_a * locals.var_psat_a))) == 0.0 && ((assign47510_e80379) as f64).is_finite() && ((assign47510_e80379) as f64).fract() == 0.0 { if assign47510_e80379 == 0.0 { 0.0 } else { (assign47510_e80379 * ((locals.var_dmob).powf(assign47510_e80379 - 1.0) * locals.var_dmob_dn8)) } } else { (assign47510_e80380 * (((-(locals.var_psat_a_dn8 / (locals.var_psat_a * locals.var_psat_a))) * (locals.var_dmob).ln()) + (assign47510_e80379 * (locals.var_dmob_dn8 / locals.var_dmob)))) }, if (-(locals.var_psat_a_dn9 / (locals.var_psat_a * locals.var_psat_a))) == 0.0 && ((assign47510_e80379) as f64).is_finite() && ((assign47510_e80379) as f64).fract() == 0.0 { if assign47510_e80379 == 0.0 { 0.0 } else { (assign47510_e80379 * ((locals.var_dmob).powf(assign47510_e80379 - 1.0) * locals.var_dmob_dn9)) } } else { (assign47510_e80380 * (((-(locals.var_psat_a_dn9 / (locals.var_psat_a * locals.var_psat_a))) * (locals.var_dmob).ln()) + (assign47510_e80379 * (locals.var_dmob_dn9 / locals.var_dmob)))) }, if (-(locals.var_psat_a_dn10 / (locals.var_psat_a * locals.var_psat_a))) == 0.0 && ((assign47510_e80379) as f64).is_finite() && ((assign47510_e80379) as f64).fract() == 0.0 { if assign47510_e80379 == 0.0 { 0.0 } else { (assign47510_e80379 * ((locals.var_dmob).powf(assign47510_e80379 - 1.0) * locals.var_dmob_dn10)) } } else { (assign47510_e80380 * (((-(locals.var_psat_a_dn10 / (locals.var_psat_a * locals.var_psat_a))) * (locals.var_dmob).ln()) + (assign47510_e80379 * (locals.var_dmob_dn10 / locals.var_dmob)))) }, if (-(locals.var_psat_a_dn11 / (locals.var_psat_a * locals.var_psat_a))) == 0.0 && ((assign47510_e80379) as f64).is_finite() && ((assign47510_e80379) as f64).fract() == 0.0 { if assign47510_e80379 == 0.0 { 0.0 } else { (assign47510_e80379 * ((locals.var_dmob).powf(assign47510_e80379 - 1.0) * locals.var_dmob_dn11)) } } else { (assign47510_e80380 * (((-(locals.var_psat_a_dn11 / (locals.var_psat_a * locals.var_psat_a))) * (locals.var_dmob).ln()) + (assign47510_e80379 * (locals.var_dmob_dn11 / locals.var_dmob)))) },)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign47510_e80382;
        locals.var_t0_dn3 = assign47510_e80382_d_n3;
        locals.var_t0_dn4 = assign47510_e80382_d_n4;
        locals.var_t0_dn5 = assign47510_e80382_d_n5;
        locals.var_t0_dn6 = assign47510_e80382_d_n6;
        locals.var_t0_dn7 = assign47510_e80382_d_n7;
        locals.var_t0_dn8 = assign47510_e80382_d_n8;
        locals.var_t0_dn9 = assign47510_e80382_d_n9;
        locals.var_t0_dn10 = assign47510_e80382_d_n10;
        locals.var_t0_dn11 = assign47510_e80382_d_n11;

        let (assign47520_e80389, assign47520_e80389_d_n3, assign47520_e80389_d_n4, assign47520_e80389_d_n5, assign47520_e80389_d_n6, assign47520_e80389_d_n7, assign47520_e80389_d_n8, assign47520_e80389_d_n9, assign47520_e80389_d_n10, assign47520_e80389_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign47520_e80387: f64 = (locals.var_psatb_i * locals.var_vbsx);
        (assign47520_e80387, (locals.var_psatb_i * locals.var_vbsx_dn3), (locals.var_psatb_i * locals.var_vbsx_dn4), (locals.var_psatb_i * locals.var_vbsx_dn5), (locals.var_psatb_i * locals.var_vbsx_dn6), (locals.var_psatb_i * locals.var_vbsx_dn7), (locals.var_psatb_i * locals.var_vbsx_dn8), (locals.var_psatb_i * locals.var_vbsx_dn9), (locals.var_psatb_i * locals.var_vbsx_dn10), (locals.var_psatb_i * locals.var_vbsx_dn11),)
    } else {
        (locals.var_t11, locals.var_t11_dn3, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11,)
    }
};
        locals.var_t11 = assign47520_e80389;
        locals.var_t11_dn3 = assign47520_e80389_d_n3;
        locals.var_t11_dn4 = assign47520_e80389_d_n4;
        locals.var_t11_dn5 = assign47520_e80389_d_n5;
        locals.var_t11_dn6 = assign47520_e80389_d_n6;
        locals.var_t11_dn7 = assign47520_e80389_d_n7;
        locals.var_t11_dn8 = assign47520_e80389_d_n8;
        locals.var_t11_dn9 = assign47520_e80389_d_n9;
        locals.var_t11_dn10 = assign47520_e80389_d_n10;
        locals.var_t11_dn11 = assign47520_e80389_d_n11;

        let (assign47530_e80399, assign47530_e80399_d_n3, assign47530_e80399_d_n4, assign47530_e80399_d_n5, assign47530_e80399_d_n6, assign47530_e80399_d_n7, assign47530_e80399_d_n8, assign47530_e80399_d_n9, assign47530_e80399_d_n10, assign47530_e80399_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign47530_e80395: f64 = (locals.var_t11 * locals.var_t11);
        let assign47530_e80396: f64 = (0.1 + assign47530_e80395);
        let assign47530_e80397: f64 = (assign47530_e80396).sqrt();
        (assign47530_e80397, (((locals.var_t11_dn3 * locals.var_t11) + (locals.var_t11 * locals.var_t11_dn3)) / (2.0 * assign47530_e80397)), (((locals.var_t11_dn4 * locals.var_t11) + (locals.var_t11 * locals.var_t11_dn4)) / (2.0 * assign47530_e80397)), (((locals.var_t11_dn5 * locals.var_t11) + (locals.var_t11 * locals.var_t11_dn5)) / (2.0 * assign47530_e80397)), (((locals.var_t11_dn6 * locals.var_t11) + (locals.var_t11 * locals.var_t11_dn6)) / (2.0 * assign47530_e80397)), (((locals.var_t11_dn7 * locals.var_t11) + (locals.var_t11 * locals.var_t11_dn7)) / (2.0 * assign47530_e80397)), (((locals.var_t11_dn8 * locals.var_t11) + (locals.var_t11 * locals.var_t11_dn8)) / (2.0 * assign47530_e80397)), (((locals.var_t11_dn9 * locals.var_t11) + (locals.var_t11 * locals.var_t11_dn9)) / (2.0 * assign47530_e80397)), (((locals.var_t11_dn10 * locals.var_t11) + (locals.var_t11 * locals.var_t11_dn10)) / (2.0 * assign47530_e80397)), (((locals.var_t11_dn11 * locals.var_t11) + (locals.var_t11 * locals.var_t11_dn11)) / (2.0 * assign47530_e80397)),)
    } else {
        (locals.var_t12, locals.var_t12_dn3, locals.var_t12_dn4, locals.var_t12_dn5, locals.var_t12_dn6, locals.var_t12_dn7, locals.var_t12_dn8, locals.var_t12_dn9, locals.var_t12_dn10, locals.var_t12_dn11,)
    }
};
        locals.var_t12 = assign47530_e80399;
        locals.var_t12_dn3 = assign47530_e80399_d_n3;
        locals.var_t12_dn4 = assign47530_e80399_d_n4;
        locals.var_t12_dn5 = assign47530_e80399_d_n5;
        locals.var_t12_dn6 = assign47530_e80399_d_n6;
        locals.var_t12_dn7 = assign47530_e80399_d_n7;
        locals.var_t12_dn8 = assign47530_e80399_d_n8;
        locals.var_t12_dn9 = assign47530_e80399_d_n9;
        locals.var_t12_dn10 = assign47530_e80399_d_n10;
        locals.var_t12_dn11 = assign47530_e80399_d_n11;

        let (assign47540_e80419, assign47540_e80419_d_n3, assign47540_e80419_d_n4, assign47540_e80419_d_n5, assign47540_e80419_d_n6, assign47540_e80419_d_n7, assign47540_e80419_d_n8, assign47540_e80419_d_n9, assign47540_e80419_d_n10, assign47540_e80419_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign47540_e80405: f64 = (1.0 - locals.var_t11);
        let assign47540_e80408: f64 = (1.0 - locals.var_t11);
        let assign47540_e80411: f64 = (1.0 - locals.var_t11);
        let assign47540_e80412: f64 = (assign47540_e80408 * assign47540_e80411);
        let assign47540_e80414: f64 = (assign47540_e80412 + locals.var_t12);
        let assign47540_e80415: f64 = (assign47540_e80414).sqrt();
        let assign47540_e80416: f64 = (assign47540_e80405 + assign47540_e80415);
        let assign47540_e80417: f64 = (0.5 * assign47540_e80416);
        (assign47540_e80417, (0.5 * ((-locals.var_t11_dn3) + (((((-locals.var_t11_dn3) * assign47540_e80411) + (assign47540_e80408 * (-locals.var_t11_dn3))) + locals.var_t12_dn3) / (2.0 * assign47540_e80415)))), (0.5 * ((-locals.var_t11_dn4) + (((((-locals.var_t11_dn4) * assign47540_e80411) + (assign47540_e80408 * (-locals.var_t11_dn4))) + locals.var_t12_dn4) / (2.0 * assign47540_e80415)))), (0.5 * ((-locals.var_t11_dn5) + (((((-locals.var_t11_dn5) * assign47540_e80411) + (assign47540_e80408 * (-locals.var_t11_dn5))) + locals.var_t12_dn5) / (2.0 * assign47540_e80415)))), (0.5 * ((-locals.var_t11_dn6) + (((((-locals.var_t11_dn6) * assign47540_e80411) + (assign47540_e80408 * (-locals.var_t11_dn6))) + locals.var_t12_dn6) / (2.0 * assign47540_e80415)))), (0.5 * ((-locals.var_t11_dn7) + (((((-locals.var_t11_dn7) * assign47540_e80411) + (assign47540_e80408 * (-locals.var_t11_dn7))) + locals.var_t12_dn7) / (2.0 * assign47540_e80415)))), (0.5 * ((-locals.var_t11_dn8) + (((((-locals.var_t11_dn8) * assign47540_e80411) + (assign47540_e80408 * (-locals.var_t11_dn8))) + locals.var_t12_dn8) / (2.0 * assign47540_e80415)))), (0.5 * ((-locals.var_t11_dn9) + (((((-locals.var_t11_dn9) * assign47540_e80411) + (assign47540_e80408 * (-locals.var_t11_dn9))) + locals.var_t12_dn9) / (2.0 * assign47540_e80415)))), (0.5 * ((-locals.var_t11_dn10) + (((((-locals.var_t11_dn10) * assign47540_e80411) + (assign47540_e80408 * (-locals.var_t11_dn10))) + locals.var_t12_dn10) / (2.0 * assign47540_e80415)))), (0.5 * ((-locals.var_t11_dn11) + (((((-locals.var_t11_dn11) * assign47540_e80411) + (assign47540_e80408 * (-locals.var_t11_dn11))) + locals.var_t12_dn11) / (2.0 * assign47540_e80415)))),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign47540_e80419;
        locals.var_t1_dn3 = assign47540_e80419_d_n3;
        locals.var_t1_dn4 = assign47540_e80419_d_n4;
        locals.var_t1_dn5 = assign47540_e80419_d_n5;
        locals.var_t1_dn6 = assign47540_e80419_d_n6;
        locals.var_t1_dn7 = assign47540_e80419_d_n7;
        locals.var_t1_dn8 = assign47540_e80419_d_n8;
        locals.var_t1_dn9 = assign47540_e80419_d_n9;
        locals.var_t1_dn10 = assign47540_e80419_d_n10;
        locals.var_t1_dn11 = assign47540_e80419_d_n11;

        let (assign47550_e80438, assign47550_e80438_d_n3, assign47550_e80438_d_n4, assign47550_e80438_d_n5, assign47550_e80438_d_n6, assign47550_e80438_d_n7, assign47550_e80438_d_n8, assign47550_e80438_d_n9, assign47550_e80438_d_n10, assign47550_e80438_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign47550_e80424: f64 = (10.0 * p.p497);
        let assign47550_e80426: f64 = (assign47550_e80424 * locals.var_qia);
        let assign47550_e80428: f64 = (assign47550_e80426 * locals.var_t1);
        let assign47550_e80431: f64 = (10.0 * p.p497);
        let assign47550_e80434: f64 = (locals.var_qia * locals.var_t1);
        let assign47550_e80435: f64 = (assign47550_e80431 + assign47550_e80434);
        let assign47550_e80436: f64 = (assign47550_e80428 / assign47550_e80435);
        (assign47550_e80436, ((((((assign47550_e80424 * locals.var_qia_dn3) * locals.var_t1) + (assign47550_e80426 * locals.var_t1_dn3)) * assign47550_e80435) - (assign47550_e80428 * ((locals.var_qia_dn3 * locals.var_t1) + (locals.var_qia * locals.var_t1_dn3)))) / (assign47550_e80435 * assign47550_e80435)), ((((((assign47550_e80424 * locals.var_qia_dn4) * locals.var_t1) + (assign47550_e80426 * locals.var_t1_dn4)) * assign47550_e80435) - (assign47550_e80428 * ((locals.var_qia_dn4 * locals.var_t1) + (locals.var_qia * locals.var_t1_dn4)))) / (assign47550_e80435 * assign47550_e80435)), ((((((assign47550_e80424 * locals.var_qia_dn5) * locals.var_t1) + (assign47550_e80426 * locals.var_t1_dn5)) * assign47550_e80435) - (assign47550_e80428 * ((locals.var_qia_dn5 * locals.var_t1) + (locals.var_qia * locals.var_t1_dn5)))) / (assign47550_e80435 * assign47550_e80435)), ((((((assign47550_e80424 * locals.var_qia_dn6) * locals.var_t1) + (assign47550_e80426 * locals.var_t1_dn6)) * assign47550_e80435) - (assign47550_e80428 * ((locals.var_qia_dn6 * locals.var_t1) + (locals.var_qia * locals.var_t1_dn6)))) / (assign47550_e80435 * assign47550_e80435)), ((((((assign47550_e80424 * locals.var_qia_dn7) * locals.var_t1) + (assign47550_e80426 * locals.var_t1_dn7)) * assign47550_e80435) - (assign47550_e80428 * ((locals.var_qia_dn7 * locals.var_t1) + (locals.var_qia * locals.var_t1_dn7)))) / (assign47550_e80435 * assign47550_e80435)), ((((((assign47550_e80424 * locals.var_qia_dn8) * locals.var_t1) + (assign47550_e80426 * locals.var_t1_dn8)) * assign47550_e80435) - (assign47550_e80428 * ((locals.var_qia_dn8 * locals.var_t1) + (locals.var_qia * locals.var_t1_dn8)))) / (assign47550_e80435 * assign47550_e80435)), ((((((assign47550_e80424 * locals.var_qia_dn9) * locals.var_t1) + (assign47550_e80426 * locals.var_t1_dn9)) * assign47550_e80435) - (assign47550_e80428 * ((locals.var_qia_dn9 * locals.var_t1) + (locals.var_qia * locals.var_t1_dn9)))) / (assign47550_e80435 * assign47550_e80435)), ((((((assign47550_e80424 * locals.var_qia_dn10) * locals.var_t1) + (assign47550_e80426 * locals.var_t1_dn10)) * assign47550_e80435) - (assign47550_e80428 * ((locals.var_qia_dn10 * locals.var_t1) + (locals.var_qia * locals.var_t1_dn10)))) / (assign47550_e80435 * assign47550_e80435)), ((((((assign47550_e80424 * locals.var_qia_dn11) * locals.var_t1) + (assign47550_e80426 * locals.var_t1_dn11)) * assign47550_e80435) - (assign47550_e80428 * ((locals.var_qia_dn11 * locals.var_t1) + (locals.var_qia * locals.var_t1_dn11)))) / (assign47550_e80435 * assign47550_e80435)),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign47550_e80438;
        locals.var_t2_dn3 = assign47550_e80438_d_n3;
        locals.var_t2_dn4 = assign47550_e80438_d_n4;
        locals.var_t2_dn5 = assign47550_e80438_d_n5;
        locals.var_t2_dn6 = assign47550_e80438_d_n6;
        locals.var_t2_dn7 = assign47550_e80438_d_n7;
        locals.var_t2_dn8 = assign47550_e80438_d_n8;
        locals.var_t2_dn9 = assign47550_e80438_d_n9;
        locals.var_t2_dn10 = assign47550_e80438_d_n10;
        locals.var_t2_dn11 = assign47550_e80438_d_n11;

        let assign47560_e80441: f64 = if locals.var_ptwg_a < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard737 = assign47560_e80441;

        let (assign47570_e80466, assign47570_e80466_d_n3, assign47570_e80466_d_n4, assign47570_e80466_d_n5, assign47570_e80466_d_n6, assign47570_e80466_d_n7, assign47570_e80466_d_n8, assign47570_e80466_d_n9, assign47570_e80466_d_n10, assign47570_e80466_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard737 != 0.0)) {
        let assign47570_e80449: f64 = (locals.var_u0_a / locals.var_t0);
        let assign47570_e80451: f64 = (assign47570_e80449 * locals.var_nvt);
        let assign47570_e80454: f64 = (locals.var_vsat_a * locals.var_leff);
        let assign47570_e80455: f64 = (assign47570_e80451 / assign47570_e80454);
        let assign47570_e80456: f64 = (2.0 * assign47570_e80455);
        let assign47570_e80461: f64 = (locals.var_ptwg_a * locals.var_t2);
        let assign47570_e80462: f64 = (1.0 - assign47570_e80461);
        let assign47570_e80463: f64 = (1.0 / assign47570_e80462);
        let assign47570_e80464: f64 = (assign47570_e80456 * assign47570_e80463);
        (assign47570_e80464, (((2.0 * ((((((((locals.var_u0_a_dn3 * locals.var_t0) - (locals.var_u0_a * locals.var_t0_dn3)) / (locals.var_t0 * locals.var_t0)) * locals.var_nvt) + (assign47570_e80449 * locals.var_nvt_dn3)) * assign47570_e80454) - (assign47570_e80451 * (locals.var_vsat_a_dn3 * locals.var_leff))) / (assign47570_e80454 * assign47570_e80454))) * assign47570_e80463) + (assign47570_e80456 * (-((-((locals.var_ptwg_a_dn3 * locals.var_t2) + (locals.var_ptwg_a * locals.var_t2_dn3))) / (assign47570_e80462 * assign47570_e80462))))), (((2.0 * ((((((((locals.var_u0_a_dn4 * locals.var_t0) - (locals.var_u0_a * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)) * locals.var_nvt) + (assign47570_e80449 * locals.var_nvt_dn4)) * assign47570_e80454) - (assign47570_e80451 * (locals.var_vsat_a_dn4 * locals.var_leff))) / (assign47570_e80454 * assign47570_e80454))) * assign47570_e80463) + (assign47570_e80456 * (-((-((locals.var_ptwg_a_dn4 * locals.var_t2) + (locals.var_ptwg_a * locals.var_t2_dn4))) / (assign47570_e80462 * assign47570_e80462))))), (((2.0 * ((((((((locals.var_u0_a_dn5 * locals.var_t0) - (locals.var_u0_a * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)) * locals.var_nvt) + (assign47570_e80449 * locals.var_nvt_dn5)) * assign47570_e80454) - (assign47570_e80451 * (locals.var_vsat_a_dn5 * locals.var_leff))) / (assign47570_e80454 * assign47570_e80454))) * assign47570_e80463) + (assign47570_e80456 * (-((-((locals.var_ptwg_a_dn5 * locals.var_t2) + (locals.var_ptwg_a * locals.var_t2_dn5))) / (assign47570_e80462 * assign47570_e80462))))), (((2.0 * ((((((((locals.var_u0_a_dn6 * locals.var_t0) - (locals.var_u0_a * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)) * locals.var_nvt) + (assign47570_e80449 * locals.var_nvt_dn6)) * assign47570_e80454) - (assign47570_e80451 * (locals.var_vsat_a_dn6 * locals.var_leff))) / (assign47570_e80454 * assign47570_e80454))) * assign47570_e80463) + (assign47570_e80456 * (-((-((locals.var_ptwg_a_dn6 * locals.var_t2) + (locals.var_ptwg_a * locals.var_t2_dn6))) / (assign47570_e80462 * assign47570_e80462))))), (((2.0 * ((((((((locals.var_u0_a_dn7 * locals.var_t0) - (locals.var_u0_a * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)) * locals.var_nvt) + (assign47570_e80449 * locals.var_nvt_dn7)) * assign47570_e80454) - (assign47570_e80451 * (locals.var_vsat_a_dn7 * locals.var_leff))) / (assign47570_e80454 * assign47570_e80454))) * assign47570_e80463) + (assign47570_e80456 * (-((-((locals.var_ptwg_a_dn7 * locals.var_t2) + (locals.var_ptwg_a * locals.var_t2_dn7))) / (assign47570_e80462 * assign47570_e80462))))), (((2.0 * ((((((((locals.var_u0_a_dn8 * locals.var_t0) - (locals.var_u0_a * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)) * locals.var_nvt) + (assign47570_e80449 * locals.var_nvt_dn8)) * assign47570_e80454) - (assign47570_e80451 * (locals.var_vsat_a_dn8 * locals.var_leff))) / (assign47570_e80454 * assign47570_e80454))) * assign47570_e80463) + (assign47570_e80456 * (-((-((locals.var_ptwg_a_dn8 * locals.var_t2) + (locals.var_ptwg_a * locals.var_t2_dn8))) / (assign47570_e80462 * assign47570_e80462))))), (((2.0 * ((((((((locals.var_u0_a_dn9 * locals.var_t0) - (locals.var_u0_a * locals.var_t0_dn9)) / (locals.var_t0 * locals.var_t0)) * locals.var_nvt) + (assign47570_e80449 * locals.var_nvt_dn9)) * assign47570_e80454) - (assign47570_e80451 * (locals.var_vsat_a_dn9 * locals.var_leff))) / (assign47570_e80454 * assign47570_e80454))) * assign47570_e80463) + (assign47570_e80456 * (-((-((locals.var_ptwg_a_dn9 * locals.var_t2) + (locals.var_ptwg_a * locals.var_t2_dn9))) / (assign47570_e80462 * assign47570_e80462))))), (((2.0 * ((((((((locals.var_u0_a_dn10 * locals.var_t0) - (locals.var_u0_a * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)) * locals.var_nvt) + (assign47570_e80449 * locals.var_nvt_dn10)) * assign47570_e80454) - (assign47570_e80451 * (locals.var_vsat_a_dn10 * locals.var_leff))) / (assign47570_e80454 * assign47570_e80454))) * assign47570_e80463) + (assign47570_e80456 * (-((-((locals.var_ptwg_a_dn10 * locals.var_t2) + (locals.var_ptwg_a * locals.var_t2_dn10))) / (assign47570_e80462 * assign47570_e80462))))), (((2.0 * ((((((((locals.var_u0_a_dn11 * locals.var_t0) - (locals.var_u0_a * locals.var_t0_dn11)) / (locals.var_t0 * locals.var_t0)) * locals.var_nvt) + (assign47570_e80449 * locals.var_nvt_dn11)) * assign47570_e80454) - (assign47570_e80451 * (locals.var_vsat_a_dn11 * locals.var_leff))) / (assign47570_e80454 * assign47570_e80454))) * assign47570_e80463) + (assign47570_e80456 * (-((-((locals.var_ptwg_a_dn11 * locals.var_t2) + (locals.var_ptwg_a * locals.var_t2_dn11))) / (assign47570_e80462 * assign47570_e80462))))),)
    } else {
        (locals.var_lambdac, locals.var_lambdac_dn3, locals.var_lambdac_dn4, locals.var_lambdac_dn5, locals.var_lambdac_dn6, locals.var_lambdac_dn7, locals.var_lambdac_dn8, locals.var_lambdac_dn9, locals.var_lambdac_dn10, locals.var_lambdac_dn11,)
    }
};
        locals.var_lambdac = assign47570_e80466;
        locals.var_lambdac_dn3 = assign47570_e80466_d_n3;
        locals.var_lambdac_dn4 = assign47570_e80466_d_n4;
        locals.var_lambdac_dn5 = assign47570_e80466_d_n5;
        locals.var_lambdac_dn6 = assign47570_e80466_d_n6;
        locals.var_lambdac_dn7 = assign47570_e80466_d_n7;
        locals.var_lambdac_dn8 = assign47570_e80466_d_n8;
        locals.var_lambdac_dn9 = assign47570_e80466_d_n9;
        locals.var_lambdac_dn10 = assign47570_e80466_d_n10;
        locals.var_lambdac_dn11 = assign47570_e80466_d_n11;

        let (assign47580_e80490, assign47580_e80490_d_n3, assign47580_e80490_d_n4, assign47580_e80490_d_n5, assign47580_e80490_d_n6, assign47580_e80490_d_n7, assign47580_e80490_d_n8, assign47580_e80490_d_n9, assign47580_e80490_d_n10, assign47580_e80490_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard737 == 0.0)) {
        let assign47580_e80475: f64 = (locals.var_u0_a / locals.var_t0);
        let assign47580_e80477: f64 = (assign47580_e80475 * locals.var_nvt);
        let assign47580_e80480: f64 = (locals.var_vsat_a * locals.var_leff);
        let assign47580_e80481: f64 = (assign47580_e80477 / assign47580_e80480);
        let assign47580_e80482: f64 = (2.0 * assign47580_e80481);
        let assign47580_e80486: f64 = (locals.var_ptwg_a * locals.var_t2);
        let assign47580_e80487: f64 = (1.0 + assign47580_e80486);
        let assign47580_e80488: f64 = (assign47580_e80482 * assign47580_e80487);
        (assign47580_e80488, (((2.0 * ((((((((locals.var_u0_a_dn3 * locals.var_t0) - (locals.var_u0_a * locals.var_t0_dn3)) / (locals.var_t0 * locals.var_t0)) * locals.var_nvt) + (assign47580_e80475 * locals.var_nvt_dn3)) * assign47580_e80480) - (assign47580_e80477 * (locals.var_vsat_a_dn3 * locals.var_leff))) / (assign47580_e80480 * assign47580_e80480))) * assign47580_e80487) + (assign47580_e80482 * ((locals.var_ptwg_a_dn3 * locals.var_t2) + (locals.var_ptwg_a * locals.var_t2_dn3)))), (((2.0 * ((((((((locals.var_u0_a_dn4 * locals.var_t0) - (locals.var_u0_a * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)) * locals.var_nvt) + (assign47580_e80475 * locals.var_nvt_dn4)) * assign47580_e80480) - (assign47580_e80477 * (locals.var_vsat_a_dn4 * locals.var_leff))) / (assign47580_e80480 * assign47580_e80480))) * assign47580_e80487) + (assign47580_e80482 * ((locals.var_ptwg_a_dn4 * locals.var_t2) + (locals.var_ptwg_a * locals.var_t2_dn4)))), (((2.0 * ((((((((locals.var_u0_a_dn5 * locals.var_t0) - (locals.var_u0_a * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)) * locals.var_nvt) + (assign47580_e80475 * locals.var_nvt_dn5)) * assign47580_e80480) - (assign47580_e80477 * (locals.var_vsat_a_dn5 * locals.var_leff))) / (assign47580_e80480 * assign47580_e80480))) * assign47580_e80487) + (assign47580_e80482 * ((locals.var_ptwg_a_dn5 * locals.var_t2) + (locals.var_ptwg_a * locals.var_t2_dn5)))), (((2.0 * ((((((((locals.var_u0_a_dn6 * locals.var_t0) - (locals.var_u0_a * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)) * locals.var_nvt) + (assign47580_e80475 * locals.var_nvt_dn6)) * assign47580_e80480) - (assign47580_e80477 * (locals.var_vsat_a_dn6 * locals.var_leff))) / (assign47580_e80480 * assign47580_e80480))) * assign47580_e80487) + (assign47580_e80482 * ((locals.var_ptwg_a_dn6 * locals.var_t2) + (locals.var_ptwg_a * locals.var_t2_dn6)))), (((2.0 * ((((((((locals.var_u0_a_dn7 * locals.var_t0) - (locals.var_u0_a * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)) * locals.var_nvt) + (assign47580_e80475 * locals.var_nvt_dn7)) * assign47580_e80480) - (assign47580_e80477 * (locals.var_vsat_a_dn7 * locals.var_leff))) / (assign47580_e80480 * assign47580_e80480))) * assign47580_e80487) + (assign47580_e80482 * ((locals.var_ptwg_a_dn7 * locals.var_t2) + (locals.var_ptwg_a * locals.var_t2_dn7)))), (((2.0 * ((((((((locals.var_u0_a_dn8 * locals.var_t0) - (locals.var_u0_a * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)) * locals.var_nvt) + (assign47580_e80475 * locals.var_nvt_dn8)) * assign47580_e80480) - (assign47580_e80477 * (locals.var_vsat_a_dn8 * locals.var_leff))) / (assign47580_e80480 * assign47580_e80480))) * assign47580_e80487) + (assign47580_e80482 * ((locals.var_ptwg_a_dn8 * locals.var_t2) + (locals.var_ptwg_a * locals.var_t2_dn8)))), (((2.0 * ((((((((locals.var_u0_a_dn9 * locals.var_t0) - (locals.var_u0_a * locals.var_t0_dn9)) / (locals.var_t0 * locals.var_t0)) * locals.var_nvt) + (assign47580_e80475 * locals.var_nvt_dn9)) * assign47580_e80480) - (assign47580_e80477 * (locals.var_vsat_a_dn9 * locals.var_leff))) / (assign47580_e80480 * assign47580_e80480))) * assign47580_e80487) + (assign47580_e80482 * ((locals.var_ptwg_a_dn9 * locals.var_t2) + (locals.var_ptwg_a * locals.var_t2_dn9)))), (((2.0 * ((((((((locals.var_u0_a_dn10 * locals.var_t0) - (locals.var_u0_a * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)) * locals.var_nvt) + (assign47580_e80475 * locals.var_nvt_dn10)) * assign47580_e80480) - (assign47580_e80477 * (locals.var_vsat_a_dn10 * locals.var_leff))) / (assign47580_e80480 * assign47580_e80480))) * assign47580_e80487) + (assign47580_e80482 * ((locals.var_ptwg_a_dn10 * locals.var_t2) + (locals.var_ptwg_a * locals.var_t2_dn10)))), (((2.0 * ((((((((locals.var_u0_a_dn11 * locals.var_t0) - (locals.var_u0_a * locals.var_t0_dn11)) / (locals.var_t0 * locals.var_t0)) * locals.var_nvt) + (assign47580_e80475 * locals.var_nvt_dn11)) * assign47580_e80480) - (assign47580_e80477 * (locals.var_vsat_a_dn11 * locals.var_leff))) / (assign47580_e80480 * assign47580_e80480))) * assign47580_e80487) + (assign47580_e80482 * ((locals.var_ptwg_a_dn11 * locals.var_t2) + (locals.var_ptwg_a * locals.var_t2_dn11)))),)
    } else {
        (locals.var_lambdac, locals.var_lambdac_dn3, locals.var_lambdac_dn4, locals.var_lambdac_dn5, locals.var_lambdac_dn6, locals.var_lambdac_dn7, locals.var_lambdac_dn8, locals.var_lambdac_dn9, locals.var_lambdac_dn10, locals.var_lambdac_dn11,)
    }
};
        locals.var_lambdac = assign47580_e80490;
        locals.var_lambdac_dn3 = assign47580_e80490_d_n3;
        locals.var_lambdac_dn4 = assign47580_e80490_d_n4;
        locals.var_lambdac_dn5 = assign47580_e80490_d_n5;
        locals.var_lambdac_dn6 = assign47580_e80490_d_n6;
        locals.var_lambdac_dn7 = assign47580_e80490_d_n7;
        locals.var_lambdac_dn8 = assign47580_e80490_d_n8;
        locals.var_lambdac_dn9 = assign47580_e80490_d_n9;
        locals.var_lambdac_dn10 = assign47580_e80490_d_n10;
        locals.var_lambdac_dn11 = assign47580_e80490_d_n11;

        let (assign47590_e80501, assign47590_e80501_d_n3, assign47590_e80501_d_n4, assign47590_e80501_d_n5, assign47590_e80501_d_n6, assign47590_e80501_d_n7, assign47590_e80501_d_n8, assign47590_e80501_d_n9, assign47590_e80501_d_n10, assign47590_e80501_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign47590_e80495: f64 = (2.0 * locals.var_lambdac);
        let assign47590_e80498: f64 = (locals.var_qs_1 - locals.var_qdeff);
        let assign47590_e80499: f64 = (assign47590_e80495 * assign47590_e80498);
        (assign47590_e80499, (((2.0 * locals.var_lambdac_dn3) * assign47590_e80498) + (assign47590_e80495 * (locals.var_qs_1_dn3 - locals.var_qdeff_dn3))), (((2.0 * locals.var_lambdac_dn4) * assign47590_e80498) + (assign47590_e80495 * (locals.var_qs_1_dn4 - locals.var_qdeff_dn4))), (((2.0 * locals.var_lambdac_dn5) * assign47590_e80498) + (assign47590_e80495 * (locals.var_qs_1_dn5 - locals.var_qdeff_dn5))), (((2.0 * locals.var_lambdac_dn6) * assign47590_e80498) + (assign47590_e80495 * (locals.var_qs_1_dn6 - locals.var_qdeff_dn6))), (((2.0 * locals.var_lambdac_dn7) * assign47590_e80498) + (assign47590_e80495 * (locals.var_qs_1_dn7 - locals.var_qdeff_dn7))), (((2.0 * locals.var_lambdac_dn8) * assign47590_e80498) + (assign47590_e80495 * (locals.var_qs_1_dn8 - locals.var_qdeff_dn8))), (((2.0 * locals.var_lambdac_dn9) * assign47590_e80498) + (assign47590_e80495 * (locals.var_qs_1_dn9 - locals.var_qdeff_dn9))), (((2.0 * locals.var_lambdac_dn10) * assign47590_e80498) + (assign47590_e80495 * (locals.var_qs_1_dn10 - locals.var_qdeff_dn10))), (((2.0 * locals.var_lambdac_dn11) * assign47590_e80498) + (assign47590_e80495 * (locals.var_qs_1_dn11 - locals.var_qdeff_dn11))),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign47590_e80501;
        locals.var_t1_dn3 = assign47590_e80501_d_n3;
        locals.var_t1_dn4 = assign47590_e80501_d_n4;
        locals.var_t1_dn5 = assign47590_e80501_d_n5;
        locals.var_t1_dn6 = assign47590_e80501_d_n6;
        locals.var_t1_dn7 = assign47590_e80501_d_n7;
        locals.var_t1_dn8 = assign47590_e80501_d_n8;
        locals.var_t1_dn9 = assign47590_e80501_d_n9;
        locals.var_t1_dn10 = assign47590_e80501_d_n10;
        locals.var_t1_dn11 = assign47590_e80501_d_n11;

        let (assign47600_e80511, assign47600_e80511_d_n3, assign47600_e80511_d_n4, assign47600_e80511_d_n5, assign47600_e80511_d_n6, assign47600_e80511_d_n7, assign47600_e80511_d_n8, assign47600_e80511_d_n9, assign47600_e80511_d_n10, assign47600_e80511_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign47600_e80507: f64 = (locals.var_t1 * locals.var_t1);
        let assign47600_e80508: f64 = (1.0 + assign47600_e80507);
        let assign47600_e80509: f64 = (assign47600_e80508).sqrt();
        (assign47600_e80509, (((locals.var_t1_dn3 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn3)) / (2.0 * assign47600_e80509)), (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) / (2.0 * assign47600_e80509)), (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) / (2.0 * assign47600_e80509)), (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) / (2.0 * assign47600_e80509)), (((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) / (2.0 * assign47600_e80509)), (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) / (2.0 * assign47600_e80509)), (((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) / (2.0 * assign47600_e80509)), (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) / (2.0 * assign47600_e80509)), (((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) / (2.0 * assign47600_e80509)),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign47600_e80511;
        locals.var_t2_dn3 = assign47600_e80511_d_n3;
        locals.var_t2_dn4 = assign47600_e80511_d_n4;
        locals.var_t2_dn5 = assign47600_e80511_d_n5;
        locals.var_t2_dn6 = assign47600_e80511_d_n6;
        locals.var_t2_dn7 = assign47600_e80511_d_n7;
        locals.var_t2_dn8 = assign47600_e80511_d_n8;
        locals.var_t2_dn9 = assign47600_e80511_d_n9;
        locals.var_t2_dn10 = assign47600_e80511_d_n10;
        locals.var_t2_dn11 = assign47600_e80511_d_n11;

        let assign47610_e80514: f64 = if locals.var_t1 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard738 = assign47610_e80514;

        let (assign47620_e80530, assign47620_e80530_d_n3, assign47620_e80530_d_n4, assign47620_e80530_d_n5, assign47620_e80530_d_n6, assign47620_e80530_d_n7, assign47620_e80530_d_n8, assign47620_e80530_d_n9, assign47620_e80530_d_n10, assign47620_e80530_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard738 != 0.0)) {
        let assign47620_e80523: f64 = (1.0 / locals.var_t1);
        let assign47620_e80525: f64 = (locals.var_t1).asinh();
        let assign47620_e80526: f64 = (assign47620_e80523 * assign47620_e80525);
        let assign47620_e80527: f64 = (locals.var_t2 + assign47620_e80526);
        let assign47620_e80528: f64 = (0.5 * assign47620_e80527);
        (assign47620_e80528, (0.5 * (locals.var_t2_dn3 + (((-(locals.var_t1_dn3 / (locals.var_t1 * locals.var_t1))) * assign47620_e80525) + (assign47620_e80523 * (locals.var_t1_dn3 / ((locals.var_t1 * locals.var_t1) + 1.0).sqrt()))))), (0.5 * (locals.var_t2_dn4 + (((-(locals.var_t1_dn4 / (locals.var_t1 * locals.var_t1))) * assign47620_e80525) + (assign47620_e80523 * (locals.var_t1_dn4 / ((locals.var_t1 * locals.var_t1) + 1.0).sqrt()))))), (0.5 * (locals.var_t2_dn5 + (((-(locals.var_t1_dn5 / (locals.var_t1 * locals.var_t1))) * assign47620_e80525) + (assign47620_e80523 * (locals.var_t1_dn5 / ((locals.var_t1 * locals.var_t1) + 1.0).sqrt()))))), (0.5 * (locals.var_t2_dn6 + (((-(locals.var_t1_dn6 / (locals.var_t1 * locals.var_t1))) * assign47620_e80525) + (assign47620_e80523 * (locals.var_t1_dn6 / ((locals.var_t1 * locals.var_t1) + 1.0).sqrt()))))), (0.5 * (locals.var_t2_dn7 + (((-(locals.var_t1_dn7 / (locals.var_t1 * locals.var_t1))) * assign47620_e80525) + (assign47620_e80523 * (locals.var_t1_dn7 / ((locals.var_t1 * locals.var_t1) + 1.0).sqrt()))))), (0.5 * (locals.var_t2_dn8 + (((-(locals.var_t1_dn8 / (locals.var_t1 * locals.var_t1))) * assign47620_e80525) + (assign47620_e80523 * (locals.var_t1_dn8 / ((locals.var_t1 * locals.var_t1) + 1.0).sqrt()))))), (0.5 * (locals.var_t2_dn9 + (((-(locals.var_t1_dn9 / (locals.var_t1 * locals.var_t1))) * assign47620_e80525) + (assign47620_e80523 * (locals.var_t1_dn9 / ((locals.var_t1 * locals.var_t1) + 1.0).sqrt()))))), (0.5 * (locals.var_t2_dn10 + (((-(locals.var_t1_dn10 / (locals.var_t1 * locals.var_t1))) * assign47620_e80525) + (assign47620_e80523 * (locals.var_t1_dn10 / ((locals.var_t1 * locals.var_t1) + 1.0).sqrt()))))), (0.5 * (locals.var_t2_dn11 + (((-(locals.var_t1_dn11 / (locals.var_t1 * locals.var_t1))) * assign47620_e80525) + (assign47620_e80523 * (locals.var_t1_dn11 / ((locals.var_t1 * locals.var_t1) + 1.0).sqrt()))))),)
    } else {
        (locals.var_dvsat, locals.var_dvsat_dn3, locals.var_dvsat_dn4, locals.var_dvsat_dn5, locals.var_dvsat_dn6, locals.var_dvsat_dn7, locals.var_dvsat_dn8, locals.var_dvsat_dn9, locals.var_dvsat_dn10, locals.var_dvsat_dn11,)
    }
};
        locals.var_dvsat = assign47620_e80530;
        locals.var_dvsat_dn3 = assign47620_e80530_d_n3;
        locals.var_dvsat_dn4 = assign47620_e80530_d_n4;
        locals.var_dvsat_dn5 = assign47620_e80530_d_n5;
        locals.var_dvsat_dn6 = assign47620_e80530_d_n6;
        locals.var_dvsat_dn7 = assign47620_e80530_d_n7;
        locals.var_dvsat_dn8 = assign47620_e80530_d_n8;
        locals.var_dvsat_dn9 = assign47620_e80530_d_n9;
        locals.var_dvsat_dn10 = assign47620_e80530_d_n10;
        locals.var_dvsat_dn11 = assign47620_e80530_d_n11;

    }

    pub(super) fn stamp_transient_block_161(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign47630_e80544, assign47630_e80544_d_n3, assign47630_e80544_d_n4, assign47630_e80544_d_n5, assign47630_e80544_d_n6, assign47630_e80544_d_n7, assign47630_e80544_d_n8, assign47630_e80544_d_n9, assign47630_e80544_d_n10, assign47630_e80544_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard738 == 0.0)) {
        let assign47630_e80540: f64 = (1.0 / locals.var_t2);
        let assign47630_e80541: f64 = (locals.var_t2 + assign47630_e80540);
        let assign47630_e80542: f64 = (0.5 * assign47630_e80541);
        (assign47630_e80542, (0.5 * (locals.var_t2_dn3 + (-(locals.var_t2_dn3 / (locals.var_t2 * locals.var_t2))))), (0.5 * (locals.var_t2_dn4 + (-(locals.var_t2_dn4 / (locals.var_t2 * locals.var_t2))))), (0.5 * (locals.var_t2_dn5 + (-(locals.var_t2_dn5 / (locals.var_t2 * locals.var_t2))))), (0.5 * (locals.var_t2_dn6 + (-(locals.var_t2_dn6 / (locals.var_t2 * locals.var_t2))))), (0.5 * (locals.var_t2_dn7 + (-(locals.var_t2_dn7 / (locals.var_t2 * locals.var_t2))))), (0.5 * (locals.var_t2_dn8 + (-(locals.var_t2_dn8 / (locals.var_t2 * locals.var_t2))))), (0.5 * (locals.var_t2_dn9 + (-(locals.var_t2_dn9 / (locals.var_t2 * locals.var_t2))))), (0.5 * (locals.var_t2_dn10 + (-(locals.var_t2_dn10 / (locals.var_t2 * locals.var_t2))))), (0.5 * (locals.var_t2_dn11 + (-(locals.var_t2_dn11 / (locals.var_t2 * locals.var_t2))))),)
    } else {
        (locals.var_dvsat, locals.var_dvsat_dn3, locals.var_dvsat_dn4, locals.var_dvsat_dn5, locals.var_dvsat_dn6, locals.var_dvsat_dn7, locals.var_dvsat_dn8, locals.var_dvsat_dn9, locals.var_dvsat_dn10, locals.var_dvsat_dn11,)
    }
};
        locals.var_dvsat = assign47630_e80544;
        locals.var_dvsat_dn3 = assign47630_e80544_d_n3;
        locals.var_dvsat_dn4 = assign47630_e80544_d_n4;
        locals.var_dvsat_dn5 = assign47630_e80544_d_n5;
        locals.var_dvsat_dn6 = assign47630_e80544_d_n6;
        locals.var_dvsat_dn7 = assign47630_e80544_d_n7;
        locals.var_dvsat_dn8 = assign47630_e80544_d_n8;
        locals.var_dvsat_dn9 = assign47630_e80544_d_n9;
        locals.var_dvsat_dn10 = assign47630_e80544_d_n10;
        locals.var_dvsat_dn11 = assign47630_e80544_d_n11;

        let (assign47640_e80549, assign47640_e80549_d_n3, assign47640_e80549_d_n4, assign47640_e80549_d_n5, assign47640_e80549_d_n6, assign47640_e80549_d_n7, assign47640_e80549_d_n8, assign47640_e80549_d_n9, assign47640_e80549_d_n10, assign47640_e80549_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        (locals.var_dvsat, locals.var_dvsat_dn3, locals.var_dvsat_dn4, locals.var_dvsat_dn5, locals.var_dvsat_dn6, locals.var_dvsat_dn7, locals.var_dvsat_dn8, locals.var_dvsat_dn9, locals.var_dvsat_dn10, locals.var_dvsat_dn11,)
    } else {
        (locals.var_dptwg, locals.var_dptwg_dn3, locals.var_dptwg_dn4, locals.var_dptwg_dn5, locals.var_dptwg_dn6, locals.var_dptwg_dn7, locals.var_dptwg_dn8, locals.var_dptwg_dn9, locals.var_dptwg_dn10, locals.var_dptwg_dn11,)
    }
};
        locals.var_dptwg = assign47640_e80549;
        locals.var_dptwg_dn3 = assign47640_e80549_d_n3;
        locals.var_dptwg_dn4 = assign47640_e80549_d_n4;
        locals.var_dptwg_dn5 = assign47640_e80549_d_n5;
        locals.var_dptwg_dn6 = assign47640_e80549_d_n6;
        locals.var_dptwg_dn7 = assign47640_e80549_d_n7;
        locals.var_dptwg_dn8 = assign47640_e80549_d_n8;
        locals.var_dptwg_dn9 = assign47640_e80549_d_n9;
        locals.var_dptwg_dn10 = assign47640_e80549_d_n10;
        locals.var_dptwg_dn11 = assign47640_e80549_d_n11;

        let (assign47650_e80554, assign47650_e80554_d_n3, assign47650_e80554_d_n4, assign47650_e80554_d_n5, assign47650_e80554_d_n6, assign47650_e80554_d_n7, assign47650_e80554_d_n8, assign47650_e80554_d_n9, assign47650_e80554_d_n10, assign47650_e80554_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rsource, locals.var_rsource_dn3, locals.var_rsource_dn4, locals.var_rsource_dn5, locals.var_rsource_dn6, locals.var_rsource_dn7, locals.var_rsource_dn8, locals.var_rsource_dn9, locals.var_rsource_dn10, locals.var_rsource_dn11,)
    }
};
        locals.var_rsource = assign47650_e80554;
        locals.var_rsource_dn3 = assign47650_e80554_d_n3;
        locals.var_rsource_dn4 = assign47650_e80554_d_n4;
        locals.var_rsource_dn5 = assign47650_e80554_d_n5;
        locals.var_rsource_dn6 = assign47650_e80554_d_n6;
        locals.var_rsource_dn7 = assign47650_e80554_d_n7;
        locals.var_rsource_dn8 = assign47650_e80554_d_n8;
        locals.var_rsource_dn9 = assign47650_e80554_d_n9;
        locals.var_rsource_dn10 = assign47650_e80554_d_n10;
        locals.var_rsource_dn11 = assign47650_e80554_d_n11;

        let (assign47660_e80559, assign47660_e80559_d_n3, assign47660_e80559_d_n4, assign47660_e80559_d_n5, assign47660_e80559_d_n6, assign47660_e80559_d_n7, assign47660_e80559_d_n8, assign47660_e80559_d_n9, assign47660_e80559_d_n10, assign47660_e80559_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rdrain, locals.var_rdrain_dn3, locals.var_rdrain_dn4, locals.var_rdrain_dn5, locals.var_rdrain_dn6, locals.var_rdrain_dn7, locals.var_rdrain_dn8, locals.var_rdrain_dn9, locals.var_rdrain_dn10, locals.var_rdrain_dn11,)
    }
};
        locals.var_rdrain = assign47660_e80559;
        locals.var_rdrain_dn3 = assign47660_e80559_d_n3;
        locals.var_rdrain_dn4 = assign47660_e80559_d_n4;
        locals.var_rdrain_dn5 = assign47660_e80559_d_n5;
        locals.var_rdrain_dn6 = assign47660_e80559_d_n6;
        locals.var_rdrain_dn7 = assign47660_e80559_d_n7;
        locals.var_rdrain_dn8 = assign47660_e80559_d_n8;
        locals.var_rdrain_dn9 = assign47660_e80559_d_n9;
        locals.var_rdrain_dn10 = assign47660_e80559_d_n10;
        locals.var_rdrain_dn11 = assign47660_e80559_d_n11;

        let assign47670_e80562: f64 = if p.p33 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard739 = assign47670_e80562;

        let (assign47680_e80569, assign47680_e80569_d_n3, assign47680_e80569_d_n4, assign47680_e80569_d_n5, assign47680_e80569_d_n6, assign47680_e80569_d_n7, assign47680_e80569_d_n8, assign47680_e80569_d_n9, assign47680_e80569_d_n10, assign47680_e80569_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard739 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rdsi, locals.var_rdsi_dn3, locals.var_rdsi_dn4, locals.var_rdsi_dn5, locals.var_rdsi_dn6, locals.var_rdsi_dn7, locals.var_rdsi_dn8, locals.var_rdsi_dn9, locals.var_rdsi_dn10, locals.var_rdsi_dn11,)
    }
};
        locals.var_rdsi = assign47680_e80569;
        locals.var_rdsi_dn3 = assign47680_e80569_d_n3;
        locals.var_rdsi_dn4 = assign47680_e80569_d_n4;
        locals.var_rdsi_dn5 = assign47680_e80569_d_n5;
        locals.var_rdsi_dn6 = assign47680_e80569_d_n6;
        locals.var_rdsi_dn7 = assign47680_e80569_d_n7;
        locals.var_rdsi_dn8 = assign47680_e80569_d_n8;
        locals.var_rdsi_dn9 = assign47680_e80569_d_n9;
        locals.var_rdsi_dn10 = assign47680_e80569_d_n10;
        locals.var_rdsi_dn11 = assign47680_e80569_d_n11;

        let (assign47690_e80576, assign47690_e80576_d_n3, assign47690_e80576_d_n4, assign47690_e80576_d_n5, assign47690_e80576_d_n6, assign47690_e80576_d_n7, assign47690_e80576_d_n8, assign47690_e80576_d_n9, assign47690_e80576_d_n10, assign47690_e80576_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard739 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dr, locals.var_dr_dn3, locals.var_dr_dn4, locals.var_dr_dn5, locals.var_dr_dn6, locals.var_dr_dn7, locals.var_dr_dn8, locals.var_dr_dn9, locals.var_dr_dn10, locals.var_dr_dn11,)
    }
};
        locals.var_dr = assign47690_e80576;
        locals.var_dr_dn3 = assign47690_e80576_d_n3;
        locals.var_dr_dn4 = assign47690_e80576_d_n4;
        locals.var_dr_dn5 = assign47690_e80576_d_n5;
        locals.var_dr_dn6 = assign47690_e80576_d_n6;
        locals.var_dr_dn7 = assign47690_e80576_d_n7;
        locals.var_dr_dn8 = assign47690_e80576_d_n8;
        locals.var_dr_dn9 = assign47690_e80576_d_n9;
        locals.var_dr_dn10 = assign47690_e80576_d_n10;
        locals.var_dr_dn11 = assign47690_e80576_d_n11;

        let (assign47700_e80585, assign47700_e80585_d_n3, assign47700_e80585_d_n4, assign47700_e80585_d_n5, assign47700_e80585_d_n6, assign47700_e80585_d_n7, assign47700_e80585_d_n8, assign47700_e80585_d_n9, assign47700_e80585_d_n10, assign47700_e80585_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard739 != 0.0)) {
        let assign47700_e80583: f64 = (locals.var_vgs_noswap - locals.var_vfbsdr);
        (assign47700_e80583, 0.0, (-locals.var_vfbsdr_dn4), (-locals.var_vfbsdr_dn5), locals.var_vgs_noswap_dn6, locals.var_vgs_noswap_dn7, locals.var_vgs_noswap_dn8, 0.0, locals.var_vgs_noswap_dn10, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign47700_e80585;
        locals.var_t2_dn3 = assign47700_e80585_d_n3;
        locals.var_t2_dn4 = assign47700_e80585_d_n4;
        locals.var_t2_dn5 = assign47700_e80585_d_n5;
        locals.var_t2_dn6 = assign47700_e80585_d_n6;
        locals.var_t2_dn7 = assign47700_e80585_d_n7;
        locals.var_t2_dn8 = assign47700_e80585_d_n8;
        locals.var_t2_dn9 = assign47700_e80585_d_n9;
        locals.var_t2_dn10 = assign47700_e80585_d_n10;
        locals.var_t2_dn11 = assign47700_e80585_d_n11;

        let (assign47710_e80597, assign47710_e80597_d_n3, assign47710_e80597_d_n4, assign47710_e80597_d_n5, assign47710_e80597_d_n6, assign47710_e80597_d_n7, assign47710_e80597_d_n8, assign47710_e80597_d_n9, assign47710_e80597_d_n10, assign47710_e80597_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard739 != 0.0)) {
        let assign47710_e80592: f64 = (locals.var_t2 * locals.var_t2);
        let assign47710_e80594: f64 = (assign47710_e80592 + 0.01);
        let assign47710_e80595: f64 = (assign47710_e80594).sqrt();
        (assign47710_e80595, (((locals.var_t2_dn3 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn3)) / (2.0 * assign47710_e80595)), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign47710_e80595)), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign47710_e80595)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign47710_e80595)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign47710_e80595)), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign47710_e80595)), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign47710_e80595)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign47710_e80595)), (((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign47710_e80595)),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign47710_e80597;
        locals.var_t3_dn3 = assign47710_e80597_d_n3;
        locals.var_t3_dn4 = assign47710_e80597_d_n4;
        locals.var_t3_dn5 = assign47710_e80597_d_n5;
        locals.var_t3_dn6 = assign47710_e80597_d_n6;
        locals.var_t3_dn7 = assign47710_e80597_d_n7;
        locals.var_t3_dn8 = assign47710_e80597_d_n8;
        locals.var_t3_dn9 = assign47710_e80597_d_n9;
        locals.var_t3_dn10 = assign47710_e80597_d_n10;
        locals.var_t3_dn11 = assign47710_e80597_d_n11;

        let (assign47720_e80608, assign47720_e80608_d_n3, assign47720_e80608_d_n4, assign47720_e80608_d_n5, assign47720_e80608_d_n6, assign47720_e80608_d_n7, assign47720_e80608_d_n8, assign47720_e80608_d_n9, assign47720_e80608_d_n10, assign47720_e80608_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard739 != 0.0)) {
        let assign47720_e80605: f64 = (locals.var_t2 + locals.var_t3);
        let assign47720_e80606: f64 = (0.5 * assign47720_e80605);
        (assign47720_e80606, (0.5 * (locals.var_t2_dn3 + locals.var_t3_dn3)), (0.5 * (locals.var_t2_dn4 + locals.var_t3_dn4)), (0.5 * (locals.var_t2_dn5 + locals.var_t3_dn5)), (0.5 * (locals.var_t2_dn6 + locals.var_t3_dn6)), (0.5 * (locals.var_t2_dn7 + locals.var_t3_dn7)), (0.5 * (locals.var_t2_dn8 + locals.var_t3_dn8)), (0.5 * (locals.var_t2_dn9 + locals.var_t3_dn9)), (0.5 * (locals.var_t2_dn10 + locals.var_t3_dn10)), (0.5 * (locals.var_t2_dn11 + locals.var_t3_dn11)),)
    } else {
        (locals.var_vgs_eff, locals.var_vgs_eff_dn3, locals.var_vgs_eff_dn4, locals.var_vgs_eff_dn5, locals.var_vgs_eff_dn6, locals.var_vgs_eff_dn7, locals.var_vgs_eff_dn8, locals.var_vgs_eff_dn9, locals.var_vgs_eff_dn10, locals.var_vgs_eff_dn11,)
    }
};
        locals.var_vgs_eff = assign47720_e80608;
        locals.var_vgs_eff_dn3 = assign47720_e80608_d_n3;
        locals.var_vgs_eff_dn4 = assign47720_e80608_d_n4;
        locals.var_vgs_eff_dn5 = assign47720_e80608_d_n5;
        locals.var_vgs_eff_dn6 = assign47720_e80608_d_n6;
        locals.var_vgs_eff_dn7 = assign47720_e80608_d_n7;
        locals.var_vgs_eff_dn8 = assign47720_e80608_d_n8;
        locals.var_vgs_eff_dn9 = assign47720_e80608_d_n9;
        locals.var_vgs_eff_dn10 = assign47720_e80608_d_n10;
        locals.var_vgs_eff_dn11 = assign47720_e80608_d_n11;

        let (assign47730_e80619, assign47730_e80619_d_n3, assign47730_e80619_d_n4, assign47730_e80619_d_n5, assign47730_e80619_d_n6, assign47730_e80619_d_n7, assign47730_e80619_d_n8, assign47730_e80619_d_n9, assign47730_e80619_d_n10, assign47730_e80619_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard739 != 0.0)) {
        let assign47730_e80616: f64 = (locals.var_prwg_i * locals.var_vgs_eff);
        let assign47730_e80617: f64 = (1.0 + assign47730_e80616);
        (assign47730_e80617, (locals.var_prwg_i * locals.var_vgs_eff_dn3), (locals.var_prwg_i * locals.var_vgs_eff_dn4), (locals.var_prwg_i * locals.var_vgs_eff_dn5), (locals.var_prwg_i * locals.var_vgs_eff_dn6), (locals.var_prwg_i * locals.var_vgs_eff_dn7), (locals.var_prwg_i * locals.var_vgs_eff_dn8), (locals.var_prwg_i * locals.var_vgs_eff_dn9), (locals.var_prwg_i * locals.var_vgs_eff_dn10), (locals.var_prwg_i * locals.var_vgs_eff_dn11),)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign47730_e80619;
        locals.var_t5_dn3 = assign47730_e80619_d_n3;
        locals.var_t5_dn4 = assign47730_e80619_d_n4;
        locals.var_t5_dn5 = assign47730_e80619_d_n5;
        locals.var_t5_dn6 = assign47730_e80619_d_n6;
        locals.var_t5_dn7 = assign47730_e80619_d_n7;
        locals.var_t5_dn8 = assign47730_e80619_d_n8;
        locals.var_t5_dn9 = assign47730_e80619_d_n9;
        locals.var_t5_dn10 = assign47730_e80619_d_n10;
        locals.var_t5_dn11 = assign47730_e80619_d_n11;

        let (assign47740_e80632, assign47740_e80632_d_n3, assign47740_e80632_d_n4, assign47740_e80632_d_n5, assign47740_e80632_d_n6, assign47740_e80632_d_n7, assign47740_e80632_d_n8, assign47740_e80632_d_n9, assign47740_e80632_d_n10, assign47740_e80632_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard739 != 0.0)) {
        let assign47740_e80626: f64 = (1.0 / locals.var_t5);
        let assign47740_e80629: f64 = (locals.var_prwb_i * locals.var_vsb_noswap);
        let assign47740_e80630: f64 = (assign47740_e80626 + assign47740_e80629);
        (assign47740_e80630, (-(locals.var_t5_dn3 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn4 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn5 / (locals.var_t5 * locals.var_t5))), ((-(locals.var_t5_dn6 / (locals.var_t5 * locals.var_t5))) + (locals.var_prwb_i * locals.var_vsb_noswap_dn6)), ((-(locals.var_t5_dn7 / (locals.var_t5 * locals.var_t5))) + (locals.var_prwb_i * locals.var_vsb_noswap_dn7)), (-(locals.var_t5_dn8 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn9 / (locals.var_t5 * locals.var_t5))), ((-(locals.var_t5_dn10 / (locals.var_t5 * locals.var_t5))) + (locals.var_prwb_i * locals.var_vsb_noswap_dn10)), (-(locals.var_t5_dn11 / (locals.var_t5 * locals.var_t5))),)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign47740_e80632;
        locals.var_t6_dn3 = assign47740_e80632_d_n3;
        locals.var_t6_dn4 = assign47740_e80632_d_n4;
        locals.var_t6_dn5 = assign47740_e80632_d_n5;
        locals.var_t6_dn6 = assign47740_e80632_d_n6;
        locals.var_t6_dn7 = assign47740_e80632_d_n7;
        locals.var_t6_dn8 = assign47740_e80632_d_n8;
        locals.var_t6_dn9 = assign47740_e80632_d_n9;
        locals.var_t6_dn10 = assign47740_e80632_d_n10;
        locals.var_t6_dn11 = assign47740_e80632_d_n11;

        let (assign47750_e80648, assign47750_e80648_d_n3, assign47750_e80648_d_n4, assign47750_e80648_d_n5, assign47750_e80648_d_n6, assign47750_e80648_d_n7, assign47750_e80648_d_n8, assign47750_e80648_d_n9, assign47750_e80648_d_n10, assign47750_e80648_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard739 != 0.0)) {
        let assign47750_e80641: f64 = (locals.var_t6 * locals.var_t6);
        let assign47750_e80643: f64 = (assign47750_e80641 + 0.01);
        let assign47750_e80644: f64 = (assign47750_e80643).sqrt();
        let assign47750_e80645: f64 = (locals.var_t6 + assign47750_e80644);
        let assign47750_e80646: f64 = (0.5 * assign47750_e80645);
        (assign47750_e80646, (0.5 * (locals.var_t6_dn3 + (((locals.var_t6_dn3 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn3)) / (2.0 * assign47750_e80644)))), (0.5 * (locals.var_t6_dn4 + (((locals.var_t6_dn4 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn4)) / (2.0 * assign47750_e80644)))), (0.5 * (locals.var_t6_dn5 + (((locals.var_t6_dn5 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn5)) / (2.0 * assign47750_e80644)))), (0.5 * (locals.var_t6_dn6 + (((locals.var_t6_dn6 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn6)) / (2.0 * assign47750_e80644)))), (0.5 * (locals.var_t6_dn7 + (((locals.var_t6_dn7 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn7)) / (2.0 * assign47750_e80644)))), (0.5 * (locals.var_t6_dn8 + (((locals.var_t6_dn8 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn8)) / (2.0 * assign47750_e80644)))), (0.5 * (locals.var_t6_dn9 + (((locals.var_t6_dn9 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn9)) / (2.0 * assign47750_e80644)))), (0.5 * (locals.var_t6_dn10 + (((locals.var_t6_dn10 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn10)) / (2.0 * assign47750_e80644)))), (0.5 * (locals.var_t6_dn11 + (((locals.var_t6_dn11 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn11)) / (2.0 * assign47750_e80644)))),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign47750_e80648;
        locals.var_t4_dn3 = assign47750_e80648_d_n3;
        locals.var_t4_dn4 = assign47750_e80648_d_n4;
        locals.var_t4_dn5 = assign47750_e80648_d_n5;
        locals.var_t4_dn6 = assign47750_e80648_d_n6;
        locals.var_t4_dn7 = assign47750_e80648_d_n7;
        locals.var_t4_dn8 = assign47750_e80648_d_n8;
        locals.var_t4_dn9 = assign47750_e80648_d_n9;
        locals.var_t4_dn10 = assign47750_e80648_d_n10;
        locals.var_t4_dn11 = assign47750_e80648_d_n11;

        let (assign47760_e80665, assign47760_e80665_d_n3, assign47760_e80665_d_n4, assign47760_e80665_d_n5, assign47760_e80665_d_n6, assign47760_e80665_d_n7, assign47760_e80665_d_n8, assign47760_e80665_d_n9, assign47760_e80665_d_n10, assign47760_e80665_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard739 != 0.0)) {
        let assign47760_e80658: f64 = (locals.var_rsw_i * locals.var_t4);
        let assign47760_e80659: f64 = (locals.var_rswmin_i + assign47760_e80658);
        let assign47760_e80661: f64 = (assign47760_e80659 * locals.var_weffwrfactor);
        let assign47760_e80662: f64 = (locals.var_rsourcegeo + assign47760_e80661);
        let assign47760_e80663: f64 = (locals.var_rdstemp * assign47760_e80662);
        (assign47760_e80663, (locals.var_rdstemp * ((locals.var_rsw_i * locals.var_t4_dn3) * locals.var_weffwrfactor)), ((locals.var_rdstemp_dn4 * assign47760_e80662) + (locals.var_rdstemp * ((locals.var_rsw_i * locals.var_t4_dn4) * locals.var_weffwrfactor))), ((locals.var_rdstemp_dn5 * assign47760_e80662) + (locals.var_rdstemp * ((locals.var_rsw_i * locals.var_t4_dn5) * locals.var_weffwrfactor))), (locals.var_rdstemp * ((locals.var_rsw_i * locals.var_t4_dn6) * locals.var_weffwrfactor)), (locals.var_rdstemp * ((locals.var_rsw_i * locals.var_t4_dn7) * locals.var_weffwrfactor)), (locals.var_rdstemp * ((locals.var_rsw_i * locals.var_t4_dn8) * locals.var_weffwrfactor)), (locals.var_rdstemp * ((locals.var_rsw_i * locals.var_t4_dn9) * locals.var_weffwrfactor)), (locals.var_rdstemp * ((locals.var_rsw_i * locals.var_t4_dn10) * locals.var_weffwrfactor)), (locals.var_rdstemp * ((locals.var_rsw_i * locals.var_t4_dn11) * locals.var_weffwrfactor)),)
    } else {
        (locals.var_rsource, locals.var_rsource_dn3, locals.var_rsource_dn4, locals.var_rsource_dn5, locals.var_rsource_dn6, locals.var_rsource_dn7, locals.var_rsource_dn8, locals.var_rsource_dn9, locals.var_rsource_dn10, locals.var_rsource_dn11,)
    }
};
        locals.var_rsource = assign47760_e80665;
        locals.var_rsource_dn3 = assign47760_e80665_d_n3;
        locals.var_rsource_dn4 = assign47760_e80665_d_n4;
        locals.var_rsource_dn5 = assign47760_e80665_d_n5;
        locals.var_rsource_dn6 = assign47760_e80665_d_n6;
        locals.var_rsource_dn7 = assign47760_e80665_d_n7;
        locals.var_rsource_dn8 = assign47760_e80665_d_n8;
        locals.var_rsource_dn9 = assign47760_e80665_d_n9;
        locals.var_rsource_dn10 = assign47760_e80665_d_n10;
        locals.var_rsource_dn11 = assign47760_e80665_d_n11;

        let (assign47770_e80674, assign47770_e80674_d_n3, assign47770_e80674_d_n4, assign47770_e80674_d_n5, assign47770_e80674_d_n6, assign47770_e80674_d_n7, assign47770_e80674_d_n8, assign47770_e80674_d_n9, assign47770_e80674_d_n10, assign47770_e80674_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard739 != 0.0)) {
        let assign47770_e80672: f64 = (locals.var_vgd_noswap - locals.var_vfbsdr);
        (assign47770_e80672, 0.0, (-locals.var_vfbsdr_dn4), (-locals.var_vfbsdr_dn5), locals.var_vgd_noswap_dn6, locals.var_vgd_noswap_dn7, locals.var_vgd_noswap_dn8, 0.0, locals.var_vgd_noswap_dn10, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign47770_e80674;
        locals.var_t2_dn3 = assign47770_e80674_d_n3;
        locals.var_t2_dn4 = assign47770_e80674_d_n4;
        locals.var_t2_dn5 = assign47770_e80674_d_n5;
        locals.var_t2_dn6 = assign47770_e80674_d_n6;
        locals.var_t2_dn7 = assign47770_e80674_d_n7;
        locals.var_t2_dn8 = assign47770_e80674_d_n8;
        locals.var_t2_dn9 = assign47770_e80674_d_n9;
        locals.var_t2_dn10 = assign47770_e80674_d_n10;
        locals.var_t2_dn11 = assign47770_e80674_d_n11;

        let (assign47780_e80686, assign47780_e80686_d_n3, assign47780_e80686_d_n4, assign47780_e80686_d_n5, assign47780_e80686_d_n6, assign47780_e80686_d_n7, assign47780_e80686_d_n8, assign47780_e80686_d_n9, assign47780_e80686_d_n10, assign47780_e80686_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard739 != 0.0)) {
        let assign47780_e80681: f64 = (locals.var_t2 * locals.var_t2);
        let assign47780_e80683: f64 = (assign47780_e80681 + 0.01);
        let assign47780_e80684: f64 = (assign47780_e80683).sqrt();
        (assign47780_e80684, (((locals.var_t2_dn3 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn3)) / (2.0 * assign47780_e80684)), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign47780_e80684)), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign47780_e80684)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign47780_e80684)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign47780_e80684)), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign47780_e80684)), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign47780_e80684)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign47780_e80684)), (((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign47780_e80684)),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign47780_e80686;
        locals.var_t3_dn3 = assign47780_e80686_d_n3;
        locals.var_t3_dn4 = assign47780_e80686_d_n4;
        locals.var_t3_dn5 = assign47780_e80686_d_n5;
        locals.var_t3_dn6 = assign47780_e80686_d_n6;
        locals.var_t3_dn7 = assign47780_e80686_d_n7;
        locals.var_t3_dn8 = assign47780_e80686_d_n8;
        locals.var_t3_dn9 = assign47780_e80686_d_n9;
        locals.var_t3_dn10 = assign47780_e80686_d_n10;
        locals.var_t3_dn11 = assign47780_e80686_d_n11;

        let (assign47790_e80697, assign47790_e80697_d_n3, assign47790_e80697_d_n4, assign47790_e80697_d_n5, assign47790_e80697_d_n6, assign47790_e80697_d_n7, assign47790_e80697_d_n8, assign47790_e80697_d_n9, assign47790_e80697_d_n10, assign47790_e80697_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard739 != 0.0)) {
        let assign47790_e80694: f64 = (locals.var_t2 + locals.var_t3);
        let assign47790_e80695: f64 = (0.5 * assign47790_e80694);
        (assign47790_e80695, (0.5 * (locals.var_t2_dn3 + locals.var_t3_dn3)), (0.5 * (locals.var_t2_dn4 + locals.var_t3_dn4)), (0.5 * (locals.var_t2_dn5 + locals.var_t3_dn5)), (0.5 * (locals.var_t2_dn6 + locals.var_t3_dn6)), (0.5 * (locals.var_t2_dn7 + locals.var_t3_dn7)), (0.5 * (locals.var_t2_dn8 + locals.var_t3_dn8)), (0.5 * (locals.var_t2_dn9 + locals.var_t3_dn9)), (0.5 * (locals.var_t2_dn10 + locals.var_t3_dn10)), (0.5 * (locals.var_t2_dn11 + locals.var_t3_dn11)),)
    } else {
        (locals.var_vgd_eff, locals.var_vgd_eff_dn3, locals.var_vgd_eff_dn4, locals.var_vgd_eff_dn5, locals.var_vgd_eff_dn6, locals.var_vgd_eff_dn7, locals.var_vgd_eff_dn8, locals.var_vgd_eff_dn9, locals.var_vgd_eff_dn10, locals.var_vgd_eff_dn11,)
    }
};
        locals.var_vgd_eff = assign47790_e80697;
        locals.var_vgd_eff_dn3 = assign47790_e80697_d_n3;
        locals.var_vgd_eff_dn4 = assign47790_e80697_d_n4;
        locals.var_vgd_eff_dn5 = assign47790_e80697_d_n5;
        locals.var_vgd_eff_dn6 = assign47790_e80697_d_n6;
        locals.var_vgd_eff_dn7 = assign47790_e80697_d_n7;
        locals.var_vgd_eff_dn8 = assign47790_e80697_d_n8;
        locals.var_vgd_eff_dn9 = assign47790_e80697_d_n9;
        locals.var_vgd_eff_dn10 = assign47790_e80697_d_n10;
        locals.var_vgd_eff_dn11 = assign47790_e80697_d_n11;

        let (assign47800_e80708, assign47800_e80708_d_n3, assign47800_e80708_d_n4, assign47800_e80708_d_n5, assign47800_e80708_d_n6, assign47800_e80708_d_n7, assign47800_e80708_d_n8, assign47800_e80708_d_n9, assign47800_e80708_d_n10, assign47800_e80708_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard739 != 0.0)) {
        let assign47800_e80705: f64 = (locals.var_prwg_i * locals.var_vgd_eff);
        let assign47800_e80706: f64 = (1.0 + assign47800_e80705);
        (assign47800_e80706, (locals.var_prwg_i * locals.var_vgd_eff_dn3), (locals.var_prwg_i * locals.var_vgd_eff_dn4), (locals.var_prwg_i * locals.var_vgd_eff_dn5), (locals.var_prwg_i * locals.var_vgd_eff_dn6), (locals.var_prwg_i * locals.var_vgd_eff_dn7), (locals.var_prwg_i * locals.var_vgd_eff_dn8), (locals.var_prwg_i * locals.var_vgd_eff_dn9), (locals.var_prwg_i * locals.var_vgd_eff_dn10), (locals.var_prwg_i * locals.var_vgd_eff_dn11),)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign47800_e80708;
        locals.var_t5_dn3 = assign47800_e80708_d_n3;
        locals.var_t5_dn4 = assign47800_e80708_d_n4;
        locals.var_t5_dn5 = assign47800_e80708_d_n5;
        locals.var_t5_dn6 = assign47800_e80708_d_n6;
        locals.var_t5_dn7 = assign47800_e80708_d_n7;
        locals.var_t5_dn8 = assign47800_e80708_d_n8;
        locals.var_t5_dn9 = assign47800_e80708_d_n9;
        locals.var_t5_dn10 = assign47800_e80708_d_n10;
        locals.var_t5_dn11 = assign47800_e80708_d_n11;

        let (assign47810_e80721, assign47810_e80721_d_n3, assign47810_e80721_d_n4, assign47810_e80721_d_n5, assign47810_e80721_d_n6, assign47810_e80721_d_n7, assign47810_e80721_d_n8, assign47810_e80721_d_n9, assign47810_e80721_d_n10, assign47810_e80721_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard739 != 0.0)) {
        let assign47810_e80715: f64 = (1.0 / locals.var_t5);
        let assign47810_e80718: f64 = (locals.var_prwb_i * locals.var_vdb_noswap);
        let assign47810_e80719: f64 = (assign47810_e80715 + assign47810_e80718);
        (assign47810_e80719, (-(locals.var_t5_dn3 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn4 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn5 / (locals.var_t5 * locals.var_t5))), ((-(locals.var_t5_dn6 / (locals.var_t5 * locals.var_t5))) + (locals.var_prwb_i * locals.var_vdb_noswap_dn6)), ((-(locals.var_t5_dn7 / (locals.var_t5 * locals.var_t5))) + (locals.var_prwb_i * locals.var_vdb_noswap_dn7)), (-(locals.var_t5_dn8 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn9 / (locals.var_t5 * locals.var_t5))), ((-(locals.var_t5_dn10 / (locals.var_t5 * locals.var_t5))) + (locals.var_prwb_i * locals.var_vdb_noswap_dn10)), (-(locals.var_t5_dn11 / (locals.var_t5 * locals.var_t5))),)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign47810_e80721;
        locals.var_t6_dn3 = assign47810_e80721_d_n3;
        locals.var_t6_dn4 = assign47810_e80721_d_n4;
        locals.var_t6_dn5 = assign47810_e80721_d_n5;
        locals.var_t6_dn6 = assign47810_e80721_d_n6;
        locals.var_t6_dn7 = assign47810_e80721_d_n7;
        locals.var_t6_dn8 = assign47810_e80721_d_n8;
        locals.var_t6_dn9 = assign47810_e80721_d_n9;
        locals.var_t6_dn10 = assign47810_e80721_d_n10;
        locals.var_t6_dn11 = assign47810_e80721_d_n11;

        let (assign47820_e80737, assign47820_e80737_d_n3, assign47820_e80737_d_n4, assign47820_e80737_d_n5, assign47820_e80737_d_n6, assign47820_e80737_d_n7, assign47820_e80737_d_n8, assign47820_e80737_d_n9, assign47820_e80737_d_n10, assign47820_e80737_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard739 != 0.0)) {
        let assign47820_e80730: f64 = (locals.var_t6 * locals.var_t6);
        let assign47820_e80732: f64 = (assign47820_e80730 + 0.01);
        let assign47820_e80733: f64 = (assign47820_e80732).sqrt();
        let assign47820_e80734: f64 = (locals.var_t6 + assign47820_e80733);
        let assign47820_e80735: f64 = (0.5 * assign47820_e80734);
        (assign47820_e80735, (0.5 * (locals.var_t6_dn3 + (((locals.var_t6_dn3 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn3)) / (2.0 * assign47820_e80733)))), (0.5 * (locals.var_t6_dn4 + (((locals.var_t6_dn4 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn4)) / (2.0 * assign47820_e80733)))), (0.5 * (locals.var_t6_dn5 + (((locals.var_t6_dn5 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn5)) / (2.0 * assign47820_e80733)))), (0.5 * (locals.var_t6_dn6 + (((locals.var_t6_dn6 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn6)) / (2.0 * assign47820_e80733)))), (0.5 * (locals.var_t6_dn7 + (((locals.var_t6_dn7 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn7)) / (2.0 * assign47820_e80733)))), (0.5 * (locals.var_t6_dn8 + (((locals.var_t6_dn8 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn8)) / (2.0 * assign47820_e80733)))), (0.5 * (locals.var_t6_dn9 + (((locals.var_t6_dn9 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn9)) / (2.0 * assign47820_e80733)))), (0.5 * (locals.var_t6_dn10 + (((locals.var_t6_dn10 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn10)) / (2.0 * assign47820_e80733)))), (0.5 * (locals.var_t6_dn11 + (((locals.var_t6_dn11 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn11)) / (2.0 * assign47820_e80733)))),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign47820_e80737;
        locals.var_t4_dn3 = assign47820_e80737_d_n3;
        locals.var_t4_dn4 = assign47820_e80737_d_n4;
        locals.var_t4_dn5 = assign47820_e80737_d_n5;
        locals.var_t4_dn6 = assign47820_e80737_d_n6;
        locals.var_t4_dn7 = assign47820_e80737_d_n7;
        locals.var_t4_dn8 = assign47820_e80737_d_n8;
        locals.var_t4_dn9 = assign47820_e80737_d_n9;
        locals.var_t4_dn10 = assign47820_e80737_d_n10;
        locals.var_t4_dn11 = assign47820_e80737_d_n11;

        let (assign47830_e80754, assign47830_e80754_d_n3, assign47830_e80754_d_n4, assign47830_e80754_d_n5, assign47830_e80754_d_n6, assign47830_e80754_d_n7, assign47830_e80754_d_n8, assign47830_e80754_d_n9, assign47830_e80754_d_n10, assign47830_e80754_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard739 != 0.0)) {
        let assign47830_e80747: f64 = (locals.var_rdw_i * locals.var_t4);
        let assign47830_e80748: f64 = (locals.var_rdwmin_i + assign47830_e80747);
        let assign47830_e80750: f64 = (assign47830_e80748 * locals.var_weffwrfactor);
        let assign47830_e80751: f64 = (locals.var_rdraingeo + assign47830_e80750);
        let assign47830_e80752: f64 = (locals.var_rdstemp * assign47830_e80751);
        (assign47830_e80752, (locals.var_rdstemp * ((locals.var_rdw_i * locals.var_t4_dn3) * locals.var_weffwrfactor)), ((locals.var_rdstemp_dn4 * assign47830_e80751) + (locals.var_rdstemp * ((locals.var_rdw_i * locals.var_t4_dn4) * locals.var_weffwrfactor))), ((locals.var_rdstemp_dn5 * assign47830_e80751) + (locals.var_rdstemp * ((locals.var_rdw_i * locals.var_t4_dn5) * locals.var_weffwrfactor))), (locals.var_rdstemp * ((locals.var_rdw_i * locals.var_t4_dn6) * locals.var_weffwrfactor)), (locals.var_rdstemp * ((locals.var_rdw_i * locals.var_t4_dn7) * locals.var_weffwrfactor)), (locals.var_rdstemp * ((locals.var_rdw_i * locals.var_t4_dn8) * locals.var_weffwrfactor)), (locals.var_rdstemp * ((locals.var_rdw_i * locals.var_t4_dn9) * locals.var_weffwrfactor)), (locals.var_rdstemp * ((locals.var_rdw_i * locals.var_t4_dn10) * locals.var_weffwrfactor)), (locals.var_rdstemp * ((locals.var_rdw_i * locals.var_t4_dn11) * locals.var_weffwrfactor)),)
    } else {
        (locals.var_rdrain, locals.var_rdrain_dn3, locals.var_rdrain_dn4, locals.var_rdrain_dn5, locals.var_rdrain_dn6, locals.var_rdrain_dn7, locals.var_rdrain_dn8, locals.var_rdrain_dn9, locals.var_rdrain_dn10, locals.var_rdrain_dn11,)
    }
};
        locals.var_rdrain = assign47830_e80754;
        locals.var_rdrain_dn3 = assign47830_e80754_d_n3;
        locals.var_rdrain_dn4 = assign47830_e80754_d_n4;
        locals.var_rdrain_dn5 = assign47830_e80754_d_n5;
        locals.var_rdrain_dn6 = assign47830_e80754_d_n6;
        locals.var_rdrain_dn7 = assign47830_e80754_d_n7;
        locals.var_rdrain_dn8 = assign47830_e80754_d_n8;
        locals.var_rdrain_dn9 = assign47830_e80754_d_n9;
        locals.var_rdrain_dn10 = assign47830_e80754_d_n10;
        locals.var_rdrain_dn11 = assign47830_e80754_d_n11;

        let (assign47840_e80766, assign47840_e80766_d_n3, assign47840_e80766_d_n4, assign47840_e80766_d_n5, assign47840_e80766_d_n6, assign47840_e80766_d_n7, assign47840_e80766_d_n8, assign47840_e80766_d_n9, assign47840_e80766_d_n10, assign47840_e80766_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard739 == 0.0)) {
        let assign47840_e80763: f64 = (locals.var_prwg_i * locals.var_qia);
        let assign47840_e80764: f64 = (1.0 + assign47840_e80763);
        (assign47840_e80764, (locals.var_prwg_i * locals.var_qia_dn3), (locals.var_prwg_i * locals.var_qia_dn4), (locals.var_prwg_i * locals.var_qia_dn5), (locals.var_prwg_i * locals.var_qia_dn6), (locals.var_prwg_i * locals.var_qia_dn7), (locals.var_prwg_i * locals.var_qia_dn8), (locals.var_prwg_i * locals.var_qia_dn9), (locals.var_prwg_i * locals.var_qia_dn10), (locals.var_prwg_i * locals.var_qia_dn11),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign47840_e80766;
        locals.var_t0_dn3 = assign47840_e80766_d_n3;
        locals.var_t0_dn4 = assign47840_e80766_d_n4;
        locals.var_t0_dn5 = assign47840_e80766_d_n5;
        locals.var_t0_dn6 = assign47840_e80766_d_n6;
        locals.var_t0_dn7 = assign47840_e80766_d_n7;
        locals.var_t0_dn8 = assign47840_e80766_d_n8;
        locals.var_t0_dn9 = assign47840_e80766_d_n9;
        locals.var_t0_dn10 = assign47840_e80766_d_n10;
        locals.var_t0_dn11 = assign47840_e80766_d_n11;

        let (assign47850_e80778, assign47850_e80778_d_n3, assign47850_e80778_d_n4, assign47850_e80778_d_n5, assign47850_e80778_d_n6, assign47850_e80778_d_n7, assign47850_e80778_d_n8, assign47850_e80778_d_n9, assign47850_e80778_d_n10, assign47850_e80778_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard739 == 0.0)) {
        let assign47850_e80775: f64 = (locals.var_sqrtphistvbs - locals.var_sqrtphist);
        let assign47850_e80776: f64 = (locals.var_prwb_i * assign47850_e80775);
        (assign47850_e80776, (locals.var_prwb_i * (locals.var_sqrtphistvbs_dn3 - locals.var_sqrtphist_dn3)), (locals.var_prwb_i * (locals.var_sqrtphistvbs_dn4 - locals.var_sqrtphist_dn4)), (locals.var_prwb_i * (locals.var_sqrtphistvbs_dn5 - locals.var_sqrtphist_dn5)), (locals.var_prwb_i * (locals.var_sqrtphistvbs_dn6 - locals.var_sqrtphist_dn6)), (locals.var_prwb_i * (locals.var_sqrtphistvbs_dn7 - locals.var_sqrtphist_dn7)), (locals.var_prwb_i * (locals.var_sqrtphistvbs_dn8 - locals.var_sqrtphist_dn8)), (locals.var_prwb_i * (locals.var_sqrtphistvbs_dn9 - locals.var_sqrtphist_dn9)), (locals.var_prwb_i * (locals.var_sqrtphistvbs_dn10 - locals.var_sqrtphist_dn10)), (locals.var_prwb_i * (locals.var_sqrtphistvbs_dn11 - locals.var_sqrtphist_dn11)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign47850_e80778;
        locals.var_t1_dn3 = assign47850_e80778_d_n3;
        locals.var_t1_dn4 = assign47850_e80778_d_n4;
        locals.var_t1_dn5 = assign47850_e80778_d_n5;
        locals.var_t1_dn6 = assign47850_e80778_d_n6;
        locals.var_t1_dn7 = assign47850_e80778_d_n7;
        locals.var_t1_dn8 = assign47850_e80778_d_n8;
        locals.var_t1_dn9 = assign47850_e80778_d_n9;
        locals.var_t1_dn10 = assign47850_e80778_d_n10;
        locals.var_t1_dn11 = assign47850_e80778_d_n11;

        let (assign47860_e80790, assign47860_e80790_d_n3, assign47860_e80790_d_n4, assign47860_e80790_d_n5, assign47860_e80790_d_n6, assign47860_e80790_d_n7, assign47860_e80790_d_n8, assign47860_e80790_d_n9, assign47860_e80790_d_n10, assign47860_e80790_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard739 == 0.0)) {
        let assign47860_e80786: f64 = (1.0 / locals.var_t0);
        let assign47860_e80788: f64 = (assign47860_e80786 + locals.var_t1);
        (assign47860_e80788, ((-(locals.var_t0_dn3 / (locals.var_t0 * locals.var_t0))) + locals.var_t1_dn3), ((-(locals.var_t0_dn4 / (locals.var_t0 * locals.var_t0))) + locals.var_t1_dn4), ((-(locals.var_t0_dn5 / (locals.var_t0 * locals.var_t0))) + locals.var_t1_dn5), ((-(locals.var_t0_dn6 / (locals.var_t0 * locals.var_t0))) + locals.var_t1_dn6), ((-(locals.var_t0_dn7 / (locals.var_t0 * locals.var_t0))) + locals.var_t1_dn7), ((-(locals.var_t0_dn8 / (locals.var_t0 * locals.var_t0))) + locals.var_t1_dn8), ((-(locals.var_t0_dn9 / (locals.var_t0 * locals.var_t0))) + locals.var_t1_dn9), ((-(locals.var_t0_dn10 / (locals.var_t0 * locals.var_t0))) + locals.var_t1_dn10), ((-(locals.var_t0_dn11 / (locals.var_t0 * locals.var_t0))) + locals.var_t1_dn11),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign47860_e80790;
        locals.var_t2_dn3 = assign47860_e80790_d_n3;
        locals.var_t2_dn4 = assign47860_e80790_d_n4;
        locals.var_t2_dn5 = assign47860_e80790_d_n5;
        locals.var_t2_dn6 = assign47860_e80790_d_n6;
        locals.var_t2_dn7 = assign47860_e80790_d_n7;
        locals.var_t2_dn8 = assign47860_e80790_d_n8;
        locals.var_t2_dn9 = assign47860_e80790_d_n9;
        locals.var_t2_dn10 = assign47860_e80790_d_n10;
        locals.var_t2_dn11 = assign47860_e80790_d_n11;

        let (assign47870_e80807, assign47870_e80807_d_n3, assign47870_e80807_d_n4, assign47870_e80807_d_n5, assign47870_e80807_d_n6, assign47870_e80807_d_n7, assign47870_e80807_d_n8, assign47870_e80807_d_n9, assign47870_e80807_d_n10, assign47870_e80807_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard739 == 0.0)) {
        let assign47870_e80800: f64 = (locals.var_t2 * locals.var_t2);
        let assign47870_e80802: f64 = (assign47870_e80800 + 0.01);
        let assign47870_e80803: f64 = (assign47870_e80802).sqrt();
        let assign47870_e80804: f64 = (locals.var_t2 + assign47870_e80803);
        let assign47870_e80805: f64 = (0.5 * assign47870_e80804);
        (assign47870_e80805, (0.5 * (locals.var_t2_dn3 + (((locals.var_t2_dn3 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn3)) / (2.0 * assign47870_e80803)))), (0.5 * (locals.var_t2_dn4 + (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign47870_e80803)))), (0.5 * (locals.var_t2_dn5 + (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign47870_e80803)))), (0.5 * (locals.var_t2_dn6 + (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign47870_e80803)))), (0.5 * (locals.var_t2_dn7 + (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign47870_e80803)))), (0.5 * (locals.var_t2_dn8 + (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign47870_e80803)))), (0.5 * (locals.var_t2_dn9 + (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign47870_e80803)))), (0.5 * (locals.var_t2_dn10 + (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign47870_e80803)))), (0.5 * (locals.var_t2_dn11 + (((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign47870_e80803)))),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign47870_e80807;
        locals.var_t3_dn3 = assign47870_e80807_d_n3;
        locals.var_t3_dn4 = assign47870_e80807_d_n4;
        locals.var_t3_dn5 = assign47870_e80807_d_n5;
        locals.var_t3_dn6 = assign47870_e80807_d_n6;
        locals.var_t3_dn7 = assign47870_e80807_d_n7;
        locals.var_t3_dn8 = assign47870_e80807_d_n8;
        locals.var_t3_dn9 = assign47870_e80807_d_n9;
        locals.var_t3_dn10 = assign47870_e80807_d_n10;
        locals.var_t3_dn11 = assign47870_e80807_d_n11;

    }

    pub(super) fn stamp_transient_block_162(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign47880_e80825, assign47880_e80825_d_n3, assign47880_e80825_d_n4, assign47880_e80825_d_n5, assign47880_e80825_d_n6, assign47880_e80825_d_n7, assign47880_e80825_d_n8, assign47880_e80825_d_n9, assign47880_e80825_d_n10, assign47880_e80825_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard739 == 0.0)) {
        let assign47880_e80817: f64 = (locals.var_rdsw_i * locals.var_t3);
        let assign47880_e80818: f64 = (locals.var_rdswmin_i + assign47880_e80817);
        let assign47880_e80819: f64 = (locals.var_rdstemp * assign47880_e80818);
        let assign47880_e80821: f64 = (assign47880_e80819 * locals.var_weffwrfactor);
        let assign47880_e80823: f64 = (assign47880_e80821 * p.p2);
        (assign47880_e80823, (((locals.var_rdstemp * (locals.var_rdsw_i * locals.var_t3_dn3)) * locals.var_weffwrfactor) * p.p2), ((((locals.var_rdstemp_dn4 * assign47880_e80818) + (locals.var_rdstemp * (locals.var_rdsw_i * locals.var_t3_dn4))) * locals.var_weffwrfactor) * p.p2), ((((locals.var_rdstemp_dn5 * assign47880_e80818) + (locals.var_rdstemp * (locals.var_rdsw_i * locals.var_t3_dn5))) * locals.var_weffwrfactor) * p.p2), (((locals.var_rdstemp * (locals.var_rdsw_i * locals.var_t3_dn6)) * locals.var_weffwrfactor) * p.p2), (((locals.var_rdstemp * (locals.var_rdsw_i * locals.var_t3_dn7)) * locals.var_weffwrfactor) * p.p2), (((locals.var_rdstemp * (locals.var_rdsw_i * locals.var_t3_dn8)) * locals.var_weffwrfactor) * p.p2), (((locals.var_rdstemp * (locals.var_rdsw_i * locals.var_t3_dn9)) * locals.var_weffwrfactor) * p.p2), (((locals.var_rdstemp * (locals.var_rdsw_i * locals.var_t3_dn10)) * locals.var_weffwrfactor) * p.p2), (((locals.var_rdstemp * (locals.var_rdsw_i * locals.var_t3_dn11)) * locals.var_weffwrfactor) * p.p2),)
    } else {
        (locals.var_rdsi, locals.var_rdsi_dn3, locals.var_rdsi_dn4, locals.var_rdsi_dn5, locals.var_rdsi_dn6, locals.var_rdsi_dn7, locals.var_rdsi_dn8, locals.var_rdsi_dn9, locals.var_rdsi_dn10, locals.var_rdsi_dn11,)
    }
};
        locals.var_rdsi = assign47880_e80825;
        locals.var_rdsi_dn3 = assign47880_e80825_d_n3;
        locals.var_rdsi_dn4 = assign47880_e80825_d_n4;
        locals.var_rdsi_dn5 = assign47880_e80825_d_n5;
        locals.var_rdsi_dn6 = assign47880_e80825_d_n6;
        locals.var_rdsi_dn7 = assign47880_e80825_d_n7;
        locals.var_rdsi_dn8 = assign47880_e80825_d_n8;
        locals.var_rdsi_dn9 = assign47880_e80825_d_n9;
        locals.var_rdsi_dn10 = assign47880_e80825_d_n10;
        locals.var_rdsi_dn11 = assign47880_e80825_d_n11;

        let (assign47890_e80833, assign47890_e80833_d_n3, assign47890_e80833_d_n4, assign47890_e80833_d_n5, assign47890_e80833_d_n6, assign47890_e80833_d_n7, assign47890_e80833_d_n8, assign47890_e80833_d_n9, assign47890_e80833_d_n10, assign47890_e80833_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard739 == 0.0)) {
        (locals.var_rdraingeo, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rdrain, locals.var_rdrain_dn3, locals.var_rdrain_dn4, locals.var_rdrain_dn5, locals.var_rdrain_dn6, locals.var_rdrain_dn7, locals.var_rdrain_dn8, locals.var_rdrain_dn9, locals.var_rdrain_dn10, locals.var_rdrain_dn11,)
    }
};
        locals.var_rdrain = assign47890_e80833;
        locals.var_rdrain_dn3 = assign47890_e80833_d_n3;
        locals.var_rdrain_dn4 = assign47890_e80833_d_n4;
        locals.var_rdrain_dn5 = assign47890_e80833_d_n5;
        locals.var_rdrain_dn6 = assign47890_e80833_d_n6;
        locals.var_rdrain_dn7 = assign47890_e80833_d_n7;
        locals.var_rdrain_dn8 = assign47890_e80833_d_n8;
        locals.var_rdrain_dn9 = assign47890_e80833_d_n9;
        locals.var_rdrain_dn10 = assign47890_e80833_d_n10;
        locals.var_rdrain_dn11 = assign47890_e80833_d_n11;

        let (assign47900_e80841, assign47900_e80841_d_n3, assign47900_e80841_d_n4, assign47900_e80841_d_n5, assign47900_e80841_d_n6, assign47900_e80841_d_n7, assign47900_e80841_d_n8, assign47900_e80841_d_n9, assign47900_e80841_d_n10, assign47900_e80841_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard739 == 0.0)) {
        (locals.var_rsourcegeo, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rsource, locals.var_rsource_dn3, locals.var_rsource_dn4, locals.var_rsource_dn5, locals.var_rsource_dn6, locals.var_rsource_dn7, locals.var_rsource_dn8, locals.var_rsource_dn9, locals.var_rsource_dn10, locals.var_rsource_dn11,)
    }
};
        locals.var_rsource = assign47900_e80841;
        locals.var_rsource_dn3 = assign47900_e80841_d_n3;
        locals.var_rsource_dn4 = assign47900_e80841_d_n4;
        locals.var_rsource_dn5 = assign47900_e80841_d_n5;
        locals.var_rsource_dn6 = assign47900_e80841_d_n6;
        locals.var_rsource_dn7 = assign47900_e80841_d_n7;
        locals.var_rsource_dn8 = assign47900_e80841_d_n8;
        locals.var_rsource_dn9 = assign47900_e80841_d_n9;
        locals.var_rsource_dn10 = assign47900_e80841_d_n10;
        locals.var_rsource_dn11 = assign47900_e80841_d_n11;

        let (assign47910_e80865, assign47910_e80865_d_n3, assign47910_e80865_d_n4, assign47910_e80865_d_n5, assign47910_e80865_d_n6, assign47910_e80865_d_n7, assign47910_e80865_d_n8, assign47910_e80865_d_n9, assign47910_e80865_d_n10, assign47910_e80865_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard739 == 0.0)) {
        let assign47910_e80851: f64 = (locals.var_dvsat * locals.var_dmob);
        let assign47910_e80852: f64 = (locals.var_u0_a / assign47910_e80851);
        let assign47910_e80854: f64 = (assign47910_e80852 * locals.var_cox);
        let assign47910_e80856: f64 = (assign47910_e80854 * locals.var_weff);
        let assign47910_e80858: f64 = (assign47910_e80856 / locals.var_leff);
        let assign47910_e80860: f64 = (assign47910_e80858 * locals.var_qia);
        let assign47910_e80862: f64 = (assign47910_e80860 * locals.var_rdsi);
        let assign47910_e80863: f64 = (1.0 + assign47910_e80862);
        (assign47910_e80863, ((((((((((locals.var_u0_a_dn3 * assign47910_e80851) - (locals.var_u0_a * ((locals.var_dvsat_dn3 * locals.var_dmob) + (locals.var_dvsat * locals.var_dmob_dn3)))) / (assign47910_e80851 * assign47910_e80851)) * locals.var_cox) * locals.var_weff) / locals.var_leff) * locals.var_qia) + (assign47910_e80858 * locals.var_qia_dn3)) * locals.var_rdsi) + (assign47910_e80860 * locals.var_rdsi_dn3)), ((((((((((locals.var_u0_a_dn4 * assign47910_e80851) - (locals.var_u0_a * ((locals.var_dvsat_dn4 * locals.var_dmob) + (locals.var_dvsat * locals.var_dmob_dn4)))) / (assign47910_e80851 * assign47910_e80851)) * locals.var_cox) * locals.var_weff) / locals.var_leff) * locals.var_qia) + (assign47910_e80858 * locals.var_qia_dn4)) * locals.var_rdsi) + (assign47910_e80860 * locals.var_rdsi_dn4)), ((((((((((locals.var_u0_a_dn5 * assign47910_e80851) - (locals.var_u0_a * ((locals.var_dvsat_dn5 * locals.var_dmob) + (locals.var_dvsat * locals.var_dmob_dn5)))) / (assign47910_e80851 * assign47910_e80851)) * locals.var_cox) * locals.var_weff) / locals.var_leff) * locals.var_qia) + (assign47910_e80858 * locals.var_qia_dn5)) * locals.var_rdsi) + (assign47910_e80860 * locals.var_rdsi_dn5)), ((((((((((locals.var_u0_a_dn6 * assign47910_e80851) - (locals.var_u0_a * ((locals.var_dvsat_dn6 * locals.var_dmob) + (locals.var_dvsat * locals.var_dmob_dn6)))) / (assign47910_e80851 * assign47910_e80851)) * locals.var_cox) * locals.var_weff) / locals.var_leff) * locals.var_qia) + (assign47910_e80858 * locals.var_qia_dn6)) * locals.var_rdsi) + (assign47910_e80860 * locals.var_rdsi_dn6)), ((((((((((locals.var_u0_a_dn7 * assign47910_e80851) - (locals.var_u0_a * ((locals.var_dvsat_dn7 * locals.var_dmob) + (locals.var_dvsat * locals.var_dmob_dn7)))) / (assign47910_e80851 * assign47910_e80851)) * locals.var_cox) * locals.var_weff) / locals.var_leff) * locals.var_qia) + (assign47910_e80858 * locals.var_qia_dn7)) * locals.var_rdsi) + (assign47910_e80860 * locals.var_rdsi_dn7)), ((((((((((locals.var_u0_a_dn8 * assign47910_e80851) - (locals.var_u0_a * ((locals.var_dvsat_dn8 * locals.var_dmob) + (locals.var_dvsat * locals.var_dmob_dn8)))) / (assign47910_e80851 * assign47910_e80851)) * locals.var_cox) * locals.var_weff) / locals.var_leff) * locals.var_qia) + (assign47910_e80858 * locals.var_qia_dn8)) * locals.var_rdsi) + (assign47910_e80860 * locals.var_rdsi_dn8)), ((((((((((locals.var_u0_a_dn9 * assign47910_e80851) - (locals.var_u0_a * ((locals.var_dvsat_dn9 * locals.var_dmob) + (locals.var_dvsat * locals.var_dmob_dn9)))) / (assign47910_e80851 * assign47910_e80851)) * locals.var_cox) * locals.var_weff) / locals.var_leff) * locals.var_qia) + (assign47910_e80858 * locals.var_qia_dn9)) * locals.var_rdsi) + (assign47910_e80860 * locals.var_rdsi_dn9)), ((((((((((locals.var_u0_a_dn10 * assign47910_e80851) - (locals.var_u0_a * ((locals.var_dvsat_dn10 * locals.var_dmob) + (locals.var_dvsat * locals.var_dmob_dn10)))) / (assign47910_e80851 * assign47910_e80851)) * locals.var_cox) * locals.var_weff) / locals.var_leff) * locals.var_qia) + (assign47910_e80858 * locals.var_qia_dn10)) * locals.var_rdsi) + (assign47910_e80860 * locals.var_rdsi_dn10)), ((((((((((locals.var_u0_a_dn11 * assign47910_e80851) - (locals.var_u0_a * ((locals.var_dvsat_dn11 * locals.var_dmob) + (locals.var_dvsat * locals.var_dmob_dn11)))) / (assign47910_e80851 * assign47910_e80851)) * locals.var_cox) * locals.var_weff) / locals.var_leff) * locals.var_qia) + (assign47910_e80858 * locals.var_qia_dn11)) * locals.var_rdsi) + (assign47910_e80860 * locals.var_rdsi_dn11)),)
    } else {
        (locals.var_dr, locals.var_dr_dn3, locals.var_dr_dn4, locals.var_dr_dn5, locals.var_dr_dn6, locals.var_dr_dn7, locals.var_dr_dn8, locals.var_dr_dn9, locals.var_dr_dn10, locals.var_dr_dn11,)
    }
};
        locals.var_dr = assign47910_e80865;
        locals.var_dr_dn3 = assign47910_e80865_d_n3;
        locals.var_dr_dn4 = assign47910_e80865_d_n4;
        locals.var_dr_dn5 = assign47910_e80865_d_n5;
        locals.var_dr_dn6 = assign47910_e80865_d_n6;
        locals.var_dr_dn7 = assign47910_e80865_d_n7;
        locals.var_dr_dn8 = assign47910_e80865_d_n8;
        locals.var_dr_dn9 = assign47910_e80865_d_n9;
        locals.var_dr_dn10 = assign47910_e80865_d_n10;
        locals.var_dr_dn11 = assign47910_e80865_d_n11;

        let assign47920_e80868: f64 = if p.p33 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard740 = assign47920_e80868;

        let (assign47930_e80892, assign47930_e80892_d_n3, assign47930_e80892_d_n4, assign47930_e80892_d_n5, assign47930_e80892_d_n6, assign47930_e80892_d_n7, assign47930_e80892_d_n8, assign47930_e80892_d_n9, assign47930_e80892_d_n10, assign47930_e80892_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard739 == 0.0)) && (locals.var_guard740 != 0.0)) {
        let assign47930_e80881: f64 = (locals.var_rdsw_i * locals.var_t3);
        let assign47930_e80882: f64 = (locals.var_rdswmin_i + assign47930_e80881);
        let assign47930_e80884: f64 = (assign47930_e80882 * locals.var_weffwrfactor);
        let assign47930_e80886: f64 = (assign47930_e80884 * p.p2);
        let assign47930_e80887: f64 = (locals.var_rsourcegeo + assign47930_e80886);
        let assign47930_e80889: f64 = (assign47930_e80887 + locals.var_rdraingeo);
        let assign47930_e80890: f64 = (locals.var_rdstemp * assign47930_e80889);
        (assign47930_e80890, (locals.var_rdstemp * (((locals.var_rdsw_i * locals.var_t3_dn3) * locals.var_weffwrfactor) * p.p2)), ((locals.var_rdstemp_dn4 * assign47930_e80889) + (locals.var_rdstemp * (((locals.var_rdsw_i * locals.var_t3_dn4) * locals.var_weffwrfactor) * p.p2))), ((locals.var_rdstemp_dn5 * assign47930_e80889) + (locals.var_rdstemp * (((locals.var_rdsw_i * locals.var_t3_dn5) * locals.var_weffwrfactor) * p.p2))), (locals.var_rdstemp * (((locals.var_rdsw_i * locals.var_t3_dn6) * locals.var_weffwrfactor) * p.p2)), (locals.var_rdstemp * (((locals.var_rdsw_i * locals.var_t3_dn7) * locals.var_weffwrfactor) * p.p2)), (locals.var_rdstemp * (((locals.var_rdsw_i * locals.var_t3_dn8) * locals.var_weffwrfactor) * p.p2)), (locals.var_rdstemp * (((locals.var_rdsw_i * locals.var_t3_dn9) * locals.var_weffwrfactor) * p.p2)), (locals.var_rdstemp * (((locals.var_rdsw_i * locals.var_t3_dn10) * locals.var_weffwrfactor) * p.p2)), (locals.var_rdstemp * (((locals.var_rdsw_i * locals.var_t3_dn11) * locals.var_weffwrfactor) * p.p2)),)
    } else {
        (locals.var_rdsi, locals.var_rdsi_dn3, locals.var_rdsi_dn4, locals.var_rdsi_dn5, locals.var_rdsi_dn6, locals.var_rdsi_dn7, locals.var_rdsi_dn8, locals.var_rdsi_dn9, locals.var_rdsi_dn10, locals.var_rdsi_dn11,)
    }
};
        locals.var_rdsi = assign47930_e80892;
        locals.var_rdsi_dn3 = assign47930_e80892_d_n3;
        locals.var_rdsi_dn4 = assign47930_e80892_d_n4;
        locals.var_rdsi_dn5 = assign47930_e80892_d_n5;
        locals.var_rdsi_dn6 = assign47930_e80892_d_n6;
        locals.var_rdsi_dn7 = assign47930_e80892_d_n7;
        locals.var_rdsi_dn8 = assign47930_e80892_d_n8;
        locals.var_rdsi_dn9 = assign47930_e80892_d_n9;
        locals.var_rdsi_dn10 = assign47930_e80892_d_n10;
        locals.var_rdsi_dn11 = assign47930_e80892_d_n11;

        let (assign47940_e80902, assign47940_e80902_d_n3, assign47940_e80902_d_n4, assign47940_e80902_d_n5, assign47940_e80902_d_n6, assign47940_e80902_d_n7, assign47940_e80902_d_n8, assign47940_e80902_d_n9, assign47940_e80902_d_n10, assign47940_e80902_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard739 == 0.0)) && (locals.var_guard740 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rdrain, locals.var_rdrain_dn3, locals.var_rdrain_dn4, locals.var_rdrain_dn5, locals.var_rdrain_dn6, locals.var_rdrain_dn7, locals.var_rdrain_dn8, locals.var_rdrain_dn9, locals.var_rdrain_dn10, locals.var_rdrain_dn11,)
    }
};
        locals.var_rdrain = assign47940_e80902;
        locals.var_rdrain_dn3 = assign47940_e80902_d_n3;
        locals.var_rdrain_dn4 = assign47940_e80902_d_n4;
        locals.var_rdrain_dn5 = assign47940_e80902_d_n5;
        locals.var_rdrain_dn6 = assign47940_e80902_d_n6;
        locals.var_rdrain_dn7 = assign47940_e80902_d_n7;
        locals.var_rdrain_dn8 = assign47940_e80902_d_n8;
        locals.var_rdrain_dn9 = assign47940_e80902_d_n9;
        locals.var_rdrain_dn10 = assign47940_e80902_d_n10;
        locals.var_rdrain_dn11 = assign47940_e80902_d_n11;

        let (assign47950_e80912, assign47950_e80912_d_n3, assign47950_e80912_d_n4, assign47950_e80912_d_n5, assign47950_e80912_d_n6, assign47950_e80912_d_n7, assign47950_e80912_d_n8, assign47950_e80912_d_n9, assign47950_e80912_d_n10, assign47950_e80912_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard739 == 0.0)) && (locals.var_guard740 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rsource, locals.var_rsource_dn3, locals.var_rsource_dn4, locals.var_rsource_dn5, locals.var_rsource_dn6, locals.var_rsource_dn7, locals.var_rsource_dn8, locals.var_rsource_dn9, locals.var_rsource_dn10, locals.var_rsource_dn11,)
    }
};
        locals.var_rsource = assign47950_e80912;
        locals.var_rsource_dn3 = assign47950_e80912_d_n3;
        locals.var_rsource_dn4 = assign47950_e80912_d_n4;
        locals.var_rsource_dn5 = assign47950_e80912_d_n5;
        locals.var_rsource_dn6 = assign47950_e80912_d_n6;
        locals.var_rsource_dn7 = assign47950_e80912_d_n7;
        locals.var_rsource_dn8 = assign47950_e80912_d_n8;
        locals.var_rsource_dn9 = assign47950_e80912_d_n9;
        locals.var_rsource_dn10 = assign47950_e80912_d_n10;
        locals.var_rsource_dn11 = assign47950_e80912_d_n11;

        let (assign47960_e80938, assign47960_e80938_d_n3, assign47960_e80938_d_n4, assign47960_e80938_d_n5, assign47960_e80938_d_n6, assign47960_e80938_d_n7, assign47960_e80938_d_n8, assign47960_e80938_d_n9, assign47960_e80938_d_n10, assign47960_e80938_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard739 == 0.0)) && (locals.var_guard740 != 0.0)) {
        let assign47960_e80924: f64 = (locals.var_dvsat * locals.var_dmob);
        let assign47960_e80925: f64 = (locals.var_u0_a / assign47960_e80924);
        let assign47960_e80927: f64 = (assign47960_e80925 * locals.var_cox);
        let assign47960_e80929: f64 = (assign47960_e80927 * locals.var_weff);
        let assign47960_e80931: f64 = (assign47960_e80929 / locals.var_leff);
        let assign47960_e80933: f64 = (assign47960_e80931 * locals.var_qia);
        let assign47960_e80935: f64 = (assign47960_e80933 * locals.var_rdsi);
        let assign47960_e80936: f64 = (1.0 + assign47960_e80935);
        (assign47960_e80936, ((((((((((locals.var_u0_a_dn3 * assign47960_e80924) - (locals.var_u0_a * ((locals.var_dvsat_dn3 * locals.var_dmob) + (locals.var_dvsat * locals.var_dmob_dn3)))) / (assign47960_e80924 * assign47960_e80924)) * locals.var_cox) * locals.var_weff) / locals.var_leff) * locals.var_qia) + (assign47960_e80931 * locals.var_qia_dn3)) * locals.var_rdsi) + (assign47960_e80933 * locals.var_rdsi_dn3)), ((((((((((locals.var_u0_a_dn4 * assign47960_e80924) - (locals.var_u0_a * ((locals.var_dvsat_dn4 * locals.var_dmob) + (locals.var_dvsat * locals.var_dmob_dn4)))) / (assign47960_e80924 * assign47960_e80924)) * locals.var_cox) * locals.var_weff) / locals.var_leff) * locals.var_qia) + (assign47960_e80931 * locals.var_qia_dn4)) * locals.var_rdsi) + (assign47960_e80933 * locals.var_rdsi_dn4)), ((((((((((locals.var_u0_a_dn5 * assign47960_e80924) - (locals.var_u0_a * ((locals.var_dvsat_dn5 * locals.var_dmob) + (locals.var_dvsat * locals.var_dmob_dn5)))) / (assign47960_e80924 * assign47960_e80924)) * locals.var_cox) * locals.var_weff) / locals.var_leff) * locals.var_qia) + (assign47960_e80931 * locals.var_qia_dn5)) * locals.var_rdsi) + (assign47960_e80933 * locals.var_rdsi_dn5)), ((((((((((locals.var_u0_a_dn6 * assign47960_e80924) - (locals.var_u0_a * ((locals.var_dvsat_dn6 * locals.var_dmob) + (locals.var_dvsat * locals.var_dmob_dn6)))) / (assign47960_e80924 * assign47960_e80924)) * locals.var_cox) * locals.var_weff) / locals.var_leff) * locals.var_qia) + (assign47960_e80931 * locals.var_qia_dn6)) * locals.var_rdsi) + (assign47960_e80933 * locals.var_rdsi_dn6)), ((((((((((locals.var_u0_a_dn7 * assign47960_e80924) - (locals.var_u0_a * ((locals.var_dvsat_dn7 * locals.var_dmob) + (locals.var_dvsat * locals.var_dmob_dn7)))) / (assign47960_e80924 * assign47960_e80924)) * locals.var_cox) * locals.var_weff) / locals.var_leff) * locals.var_qia) + (assign47960_e80931 * locals.var_qia_dn7)) * locals.var_rdsi) + (assign47960_e80933 * locals.var_rdsi_dn7)), ((((((((((locals.var_u0_a_dn8 * assign47960_e80924) - (locals.var_u0_a * ((locals.var_dvsat_dn8 * locals.var_dmob) + (locals.var_dvsat * locals.var_dmob_dn8)))) / (assign47960_e80924 * assign47960_e80924)) * locals.var_cox) * locals.var_weff) / locals.var_leff) * locals.var_qia) + (assign47960_e80931 * locals.var_qia_dn8)) * locals.var_rdsi) + (assign47960_e80933 * locals.var_rdsi_dn8)), ((((((((((locals.var_u0_a_dn9 * assign47960_e80924) - (locals.var_u0_a * ((locals.var_dvsat_dn9 * locals.var_dmob) + (locals.var_dvsat * locals.var_dmob_dn9)))) / (assign47960_e80924 * assign47960_e80924)) * locals.var_cox) * locals.var_weff) / locals.var_leff) * locals.var_qia) + (assign47960_e80931 * locals.var_qia_dn9)) * locals.var_rdsi) + (assign47960_e80933 * locals.var_rdsi_dn9)), ((((((((((locals.var_u0_a_dn10 * assign47960_e80924) - (locals.var_u0_a * ((locals.var_dvsat_dn10 * locals.var_dmob) + (locals.var_dvsat * locals.var_dmob_dn10)))) / (assign47960_e80924 * assign47960_e80924)) * locals.var_cox) * locals.var_weff) / locals.var_leff) * locals.var_qia) + (assign47960_e80931 * locals.var_qia_dn10)) * locals.var_rdsi) + (assign47960_e80933 * locals.var_rdsi_dn10)), ((((((((((locals.var_u0_a_dn11 * assign47960_e80924) - (locals.var_u0_a * ((locals.var_dvsat_dn11 * locals.var_dmob) + (locals.var_dvsat * locals.var_dmob_dn11)))) / (assign47960_e80924 * assign47960_e80924)) * locals.var_cox) * locals.var_weff) / locals.var_leff) * locals.var_qia) + (assign47960_e80931 * locals.var_qia_dn11)) * locals.var_rdsi) + (assign47960_e80933 * locals.var_rdsi_dn11)),)
    } else {
        (locals.var_dr, locals.var_dr_dn3, locals.var_dr_dn4, locals.var_dr_dn5, locals.var_dr_dn6, locals.var_dr_dn7, locals.var_dr_dn8, locals.var_dr_dn9, locals.var_dr_dn10, locals.var_dr_dn11,)
    }
};
        locals.var_dr = assign47960_e80938;
        locals.var_dr_dn3 = assign47960_e80938_d_n3;
        locals.var_dr_dn4 = assign47960_e80938_d_n4;
        locals.var_dr_dn5 = assign47960_e80938_d_n5;
        locals.var_dr_dn6 = assign47960_e80938_d_n6;
        locals.var_dr_dn7 = assign47960_e80938_d_n7;
        locals.var_dr_dn8 = assign47960_e80938_d_n8;
        locals.var_dr_dn9 = assign47960_e80938_d_n9;
        locals.var_dr_dn10 = assign47960_e80938_d_n10;
        locals.var_dr_dn11 = assign47960_e80938_d_n11;

        let (assign47970_e80953, assign47970_e80953_d_n3, assign47970_e80953_d_n4, assign47970_e80953_d_n5, assign47970_e80953_d_n6, assign47970_e80953_d_n7, assign47970_e80953_d_n8, assign47970_e80953_d_n9, assign47970_e80953_d_n10, assign47970_e80953_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign47970_e80946: f64 = (2.0 * locals.var_n);
        let assign47970_e80948: f64 = (assign47970_e80946 * locals.var_vtm);
        let assign47970_e80949: f64 = (locals.var_qia + assign47970_e80948);
        let assign47970_e80950: f64 = (locals.var_a2_t / assign47970_e80949);
        let assign47970_e80951: f64 = (locals.var_a1_t + assign47970_e80950);
        (assign47970_e80951, (-((locals.var_a2_t * (locals.var_qia_dn3 + ((2.0 * locals.var_n_dn3) * locals.var_vtm))) / (assign47970_e80949 * assign47970_e80949))), (locals.var_a1_t_dn4 + (((locals.var_a2_t_dn4 * assign47970_e80949) - (locals.var_a2_t * (locals.var_qia_dn4 + (((2.0 * locals.var_n_dn4) * locals.var_vtm) + (assign47970_e80946 * locals.var_vtm_dn4))))) / (assign47970_e80949 * assign47970_e80949))), (locals.var_a1_t_dn5 + (((locals.var_a2_t_dn5 * assign47970_e80949) - (locals.var_a2_t * (locals.var_qia_dn5 + (((2.0 * locals.var_n_dn5) * locals.var_vtm) + (assign47970_e80946 * locals.var_vtm_dn5))))) / (assign47970_e80949 * assign47970_e80949))), (-((locals.var_a2_t * (locals.var_qia_dn6 + ((2.0 * locals.var_n_dn6) * locals.var_vtm))) / (assign47970_e80949 * assign47970_e80949))), (-((locals.var_a2_t * (locals.var_qia_dn7 + ((2.0 * locals.var_n_dn7) * locals.var_vtm))) / (assign47970_e80949 * assign47970_e80949))), (-((locals.var_a2_t * (locals.var_qia_dn8 + ((2.0 * locals.var_n_dn8) * locals.var_vtm))) / (assign47970_e80949 * assign47970_e80949))), (-((locals.var_a2_t * (locals.var_qia_dn9 + ((2.0 * locals.var_n_dn9) * locals.var_vtm))) / (assign47970_e80949 * assign47970_e80949))), (-((locals.var_a2_t * (locals.var_qia_dn10 + ((2.0 * locals.var_n_dn10) * locals.var_vtm))) / (assign47970_e80949 * assign47970_e80949))), (-((locals.var_a2_t * (locals.var_qia_dn11 + ((2.0 * locals.var_n_dn11) * locals.var_vtm))) / (assign47970_e80949 * assign47970_e80949))),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign47970_e80953;
        locals.var_t0_dn3 = assign47970_e80953_d_n3;
        locals.var_t0_dn4 = assign47970_e80953_d_n4;
        locals.var_t0_dn5 = assign47970_e80953_d_n5;
        locals.var_t0_dn6 = assign47970_e80953_d_n6;
        locals.var_t0_dn7 = assign47970_e80953_d_n7;
        locals.var_t0_dn8 = assign47970_e80953_d_n8;
        locals.var_t0_dn9 = assign47970_e80953_d_n9;
        locals.var_t0_dn10 = assign47970_e80953_d_n10;
        locals.var_t0_dn11 = assign47970_e80953_d_n11;

        let (assign47980_e80960, assign47980_e80960_d_n3, assign47980_e80960_d_n4, assign47980_e80960_d_n5, assign47980_e80960_d_n6, assign47980_e80960_d_n7, assign47980_e80960_d_n8, assign47980_e80960_d_n9, assign47980_e80960_d_n10, assign47980_e80960_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign47980_e80958: f64 = (locals.var_qs_1 - locals.var_qdeff);
        (assign47980_e80958, (locals.var_qs_1_dn3 - locals.var_qdeff_dn3), (locals.var_qs_1_dn4 - locals.var_qdeff_dn4), (locals.var_qs_1_dn5 - locals.var_qdeff_dn5), (locals.var_qs_1_dn6 - locals.var_qdeff_dn6), (locals.var_qs_1_dn7 - locals.var_qdeff_dn7), (locals.var_qs_1_dn8 - locals.var_qdeff_dn8), (locals.var_qs_1_dn9 - locals.var_qdeff_dn9), (locals.var_qs_1_dn10 - locals.var_qdeff_dn10), (locals.var_qs_1_dn11 - locals.var_qdeff_dn11),)
    } else {
        (locals.var_dqsd, locals.var_dqsd_dn3, locals.var_dqsd_dn4, locals.var_dqsd_dn5, locals.var_dqsd_dn6, locals.var_dqsd_dn7, locals.var_dqsd_dn8, locals.var_dqsd_dn9, locals.var_dqsd_dn10, locals.var_dqsd_dn11,)
    }
};
        locals.var_dqsd = assign47980_e80960;
        locals.var_dqsd_dn3 = assign47980_e80960_d_n3;
        locals.var_dqsd_dn4 = assign47980_e80960_d_n4;
        locals.var_dqsd_dn5 = assign47980_e80960_d_n5;
        locals.var_dqsd_dn6 = assign47980_e80960_d_n6;
        locals.var_dqsd_dn7 = assign47980_e80960_d_n7;
        locals.var_dqsd_dn8 = assign47980_e80960_d_n8;
        locals.var_dqsd_dn9 = assign47980_e80960_d_n9;
        locals.var_dqsd_dn10 = assign47980_e80960_d_n10;
        locals.var_dqsd_dn11 = assign47980_e80960_d_n11;

        let (assign47990_e80969, assign47990_e80969_d_n3, assign47990_e80969_d_n4, assign47990_e80969_d_n5, assign47990_e80969_d_n6, assign47990_e80969_d_n7, assign47990_e80969_d_n8, assign47990_e80969_d_n9, assign47990_e80969_d_n10, assign47990_e80969_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign47990_e80965: f64 = (locals.var_t0 * locals.var_dqsd);
        let assign47990_e80967: f64 = (assign47990_e80965 * locals.var_dqsd);
        (assign47990_e80967, ((((locals.var_t0_dn3 * locals.var_dqsd) + (locals.var_t0 * locals.var_dqsd_dn3)) * locals.var_dqsd) + (assign47990_e80965 * locals.var_dqsd_dn3)), ((((locals.var_t0_dn4 * locals.var_dqsd) + (locals.var_t0 * locals.var_dqsd_dn4)) * locals.var_dqsd) + (assign47990_e80965 * locals.var_dqsd_dn4)), ((((locals.var_t0_dn5 * locals.var_dqsd) + (locals.var_t0 * locals.var_dqsd_dn5)) * locals.var_dqsd) + (assign47990_e80965 * locals.var_dqsd_dn5)), ((((locals.var_t0_dn6 * locals.var_dqsd) + (locals.var_t0 * locals.var_dqsd_dn6)) * locals.var_dqsd) + (assign47990_e80965 * locals.var_dqsd_dn6)), ((((locals.var_t0_dn7 * locals.var_dqsd) + (locals.var_t0 * locals.var_dqsd_dn7)) * locals.var_dqsd) + (assign47990_e80965 * locals.var_dqsd_dn7)), ((((locals.var_t0_dn8 * locals.var_dqsd) + (locals.var_t0 * locals.var_dqsd_dn8)) * locals.var_dqsd) + (assign47990_e80965 * locals.var_dqsd_dn8)), ((((locals.var_t0_dn9 * locals.var_dqsd) + (locals.var_t0 * locals.var_dqsd_dn9)) * locals.var_dqsd) + (assign47990_e80965 * locals.var_dqsd_dn9)), ((((locals.var_t0_dn10 * locals.var_dqsd) + (locals.var_t0 * locals.var_dqsd_dn10)) * locals.var_dqsd) + (assign47990_e80965 * locals.var_dqsd_dn10)), ((((locals.var_t0_dn11 * locals.var_dqsd) + (locals.var_t0 * locals.var_dqsd_dn11)) * locals.var_dqsd) + (assign47990_e80965 * locals.var_dqsd_dn11)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign47990_e80969;
        locals.var_t1_dn3 = assign47990_e80969_d_n3;
        locals.var_t1_dn4 = assign47990_e80969_d_n4;
        locals.var_t1_dn5 = assign47990_e80969_d_n5;
        locals.var_t1_dn6 = assign47990_e80969_d_n6;
        locals.var_t1_dn7 = assign47990_e80969_d_n7;
        locals.var_t1_dn8 = assign47990_e80969_d_n8;
        locals.var_t1_dn9 = assign47990_e80969_d_n9;
        locals.var_t1_dn10 = assign47990_e80969_d_n10;
        locals.var_t1_dn11 = assign47990_e80969_d_n11;

        let (assign48000_e80978, assign48000_e80978_d_n3, assign48000_e80978_d_n4, assign48000_e80978_d_n5, assign48000_e80978_d_n6, assign48000_e80978_d_n7, assign48000_e80978_d_n8, assign48000_e80978_d_n9, assign48000_e80978_d_n10, assign48000_e80978_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign48000_e80974: f64 = (locals.var_t1 + 1.0);
        let assign48000_e80976: f64 = (assign48000_e80974 - 0.001);
        (assign48000_e80976, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign48000_e80978;
        locals.var_t2_dn3 = assign48000_e80978_d_n3;
        locals.var_t2_dn4 = assign48000_e80978_d_n4;
        locals.var_t2_dn5 = assign48000_e80978_d_n5;
        locals.var_t2_dn6 = assign48000_e80978_d_n6;
        locals.var_t2_dn7 = assign48000_e80978_d_n7;
        locals.var_t2_dn8 = assign48000_e80978_d_n8;
        locals.var_t2_dn9 = assign48000_e80978_d_n9;
        locals.var_t2_dn10 = assign48000_e80978_d_n10;
        locals.var_t2_dn11 = assign48000_e80978_d_n11;

        let (assign48010_e80995, assign48010_e80995_d_n3, assign48010_e80995_d_n4, assign48010_e80995_d_n5, assign48010_e80995_d_n6, assign48010_e80995_d_n7, assign48010_e80995_d_n8, assign48010_e80995_d_n9, assign48010_e80995_d_n10, assign48010_e80995_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign48010_e80982: f64 = (-1.0);
        let assign48010_e80987: f64 = (locals.var_t2 * locals.var_t2);
        let assign48010_e80989: f64 = (assign48010_e80987 + 0.004);
        let assign48010_e80990: f64 = (assign48010_e80989).sqrt();
        let assign48010_e80991: f64 = (locals.var_t2 + assign48010_e80990);
        let assign48010_e80992: f64 = (0.5 * assign48010_e80991);
        let assign48010_e80993: f64 = (assign48010_e80982 + assign48010_e80992);
        (assign48010_e80993, (0.5 * (locals.var_t2_dn3 + (((locals.var_t2_dn3 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn3)) / (2.0 * assign48010_e80990)))), (0.5 * (locals.var_t2_dn4 + (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign48010_e80990)))), (0.5 * (locals.var_t2_dn5 + (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign48010_e80990)))), (0.5 * (locals.var_t2_dn6 + (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign48010_e80990)))), (0.5 * (locals.var_t2_dn7 + (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign48010_e80990)))), (0.5 * (locals.var_t2_dn8 + (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign48010_e80990)))), (0.5 * (locals.var_t2_dn9 + (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign48010_e80990)))), (0.5 * (locals.var_t2_dn10 + (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign48010_e80990)))), (0.5 * (locals.var_t2_dn11 + (((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign48010_e80990)))),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign48010_e80995;
        locals.var_t3_dn3 = assign48010_e80995_d_n3;
        locals.var_t3_dn4 = assign48010_e80995_d_n4;
        locals.var_t3_dn5 = assign48010_e80995_d_n5;
        locals.var_t3_dn6 = assign48010_e80995_d_n6;
        locals.var_t3_dn7 = assign48010_e80995_d_n7;
        locals.var_t3_dn8 = assign48010_e80995_d_n8;
        locals.var_t3_dn9 = assign48010_e80995_d_n9;
        locals.var_t3_dn10 = assign48010_e80995_d_n10;
        locals.var_t3_dn11 = assign48010_e80995_d_n11;

        let (assign48020_e81007, assign48020_e81007_d_n3, assign48020_e81007_d_n4, assign48020_e81007_d_n5, assign48020_e81007_d_n6, assign48020_e81007_d_n7, assign48020_e81007_d_n8, assign48020_e81007_d_n9, assign48020_e81007_d_n10, assign48020_e81007_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign48020_e81002: f64 = (1.0 + locals.var_t3);
        let assign48020_e81003: f64 = (assign48020_e81002).sqrt();
        let assign48020_e81004: f64 = (1.0 + assign48020_e81003);
        let assign48020_e81005: f64 = (0.5 * assign48020_e81004);
        (assign48020_e81005, (0.5 * (locals.var_t3_dn3 / (2.0 * assign48020_e81003))), (0.5 * (locals.var_t3_dn4 / (2.0 * assign48020_e81003))), (0.5 * (locals.var_t3_dn5 / (2.0 * assign48020_e81003))), (0.5 * (locals.var_t3_dn6 / (2.0 * assign48020_e81003))), (0.5 * (locals.var_t3_dn7 / (2.0 * assign48020_e81003))), (0.5 * (locals.var_t3_dn8 / (2.0 * assign48020_e81003))), (0.5 * (locals.var_t3_dn9 / (2.0 * assign48020_e81003))), (0.5 * (locals.var_t3_dn10 / (2.0 * assign48020_e81003))), (0.5 * (locals.var_t3_dn11 / (2.0 * assign48020_e81003))),)
    } else {
        (locals.var_nsat, locals.var_nsat_dn3, locals.var_nsat_dn4, locals.var_nsat_dn5, locals.var_nsat_dn6, locals.var_nsat_dn7, locals.var_nsat_dn8, locals.var_nsat_dn9, locals.var_nsat_dn10, locals.var_nsat_dn11,)
    }
};
        locals.var_nsat = assign48020_e81007;
        locals.var_nsat_dn3 = assign48020_e81007_d_n3;
        locals.var_nsat_dn4 = assign48020_e81007_d_n4;
        locals.var_nsat_dn5 = assign48020_e81007_d_n5;
        locals.var_nsat_dn6 = assign48020_e81007_d_n6;
        locals.var_nsat_dn7 = assign48020_e81007_d_n7;
        locals.var_nsat_dn8 = assign48020_e81007_d_n8;
        locals.var_nsat_dn9 = assign48020_e81007_d_n9;
        locals.var_nsat_dn10 = assign48020_e81007_d_n10;
        locals.var_nsat_dn11 = assign48020_e81007_d_n11;

        let (assign48030_e81035, assign48030_e81035_d_n3, assign48030_e81035_d_n4, assign48030_e81035_d_n5, assign48030_e81035_d_n6, assign48030_e81035_d_n7, assign48030_e81035_d_n8, assign48030_e81035_d_n9, assign48030_e81035_d_n10, assign48030_e81035_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign48030_e81013: f64 = (locals.var_nsat + 1.0);
        let assign48030_e81016: f64 = (locals.var_nsat - 1.0);
        let assign48030_e81019: f64 = (locals.var_nsat - 1.0);
        let assign48030_e81020: f64 = (assign48030_e81016 * assign48030_e81019);
        let assign48030_e81023: f64 = (0.25 * 0.01);
        let assign48030_e81025: f64 = (assign48030_e81023 * 0.01);
        let assign48030_e81026: f64 = (assign48030_e81020 + assign48030_e81025);
        let assign48030_e81027: f64 = (assign48030_e81026).sqrt();
        let assign48030_e81028: f64 = (assign48030_e81013 - assign48030_e81027);
        let assign48030_e81029: f64 = (0.5 * assign48030_e81028);
        let assign48030_e81032: f64 = (0.25 * 0.01);
        let assign48030_e81033: f64 = (assign48030_e81029 + assign48030_e81032);
        (assign48030_e81033, (0.5 * (locals.var_nsat_dn3 - (((locals.var_nsat_dn3 * assign48030_e81019) + (assign48030_e81016 * locals.var_nsat_dn3)) / (2.0 * assign48030_e81027)))), (0.5 * (locals.var_nsat_dn4 - (((locals.var_nsat_dn4 * assign48030_e81019) + (assign48030_e81016 * locals.var_nsat_dn4)) / (2.0 * assign48030_e81027)))), (0.5 * (locals.var_nsat_dn5 - (((locals.var_nsat_dn5 * assign48030_e81019) + (assign48030_e81016 * locals.var_nsat_dn5)) / (2.0 * assign48030_e81027)))), (0.5 * (locals.var_nsat_dn6 - (((locals.var_nsat_dn6 * assign48030_e81019) + (assign48030_e81016 * locals.var_nsat_dn6)) / (2.0 * assign48030_e81027)))), (0.5 * (locals.var_nsat_dn7 - (((locals.var_nsat_dn7 * assign48030_e81019) + (assign48030_e81016 * locals.var_nsat_dn7)) / (2.0 * assign48030_e81027)))), (0.5 * (locals.var_nsat_dn8 - (((locals.var_nsat_dn8 * assign48030_e81019) + (assign48030_e81016 * locals.var_nsat_dn8)) / (2.0 * assign48030_e81027)))), (0.5 * (locals.var_nsat_dn9 - (((locals.var_nsat_dn9 * assign48030_e81019) + (assign48030_e81016 * locals.var_nsat_dn9)) / (2.0 * assign48030_e81027)))), (0.5 * (locals.var_nsat_dn10 - (((locals.var_nsat_dn10 * assign48030_e81019) + (assign48030_e81016 * locals.var_nsat_dn10)) / (2.0 * assign48030_e81027)))), (0.5 * (locals.var_nsat_dn11 - (((locals.var_nsat_dn11 * assign48030_e81019) + (assign48030_e81016 * locals.var_nsat_dn11)) / (2.0 * assign48030_e81027)))),)
    } else {
        (locals.var_nsat, locals.var_nsat_dn3, locals.var_nsat_dn4, locals.var_nsat_dn5, locals.var_nsat_dn6, locals.var_nsat_dn7, locals.var_nsat_dn8, locals.var_nsat_dn9, locals.var_nsat_dn10, locals.var_nsat_dn11,)
    }
};
        locals.var_nsat = assign48030_e81035;
        locals.var_nsat_dn3 = assign48030_e81035_d_n3;
        locals.var_nsat_dn4 = assign48030_e81035_d_n4;
        locals.var_nsat_dn5 = assign48030_e81035_d_n5;
        locals.var_nsat_dn6 = assign48030_e81035_d_n6;
        locals.var_nsat_dn7 = assign48030_e81035_d_n7;
        locals.var_nsat_dn8 = assign48030_e81035_d_n8;
        locals.var_nsat_dn9 = assign48030_e81035_d_n9;
        locals.var_nsat_dn10 = assign48030_e81035_d_n10;
        locals.var_nsat_dn11 = assign48030_e81035_d_n11;

        let (assign48040_e81042, assign48040_e81042_d_n3, assign48040_e81042_d_n4, assign48040_e81042_d_n5, assign48040_e81042_d_n6, assign48040_e81042_d_n7, assign48040_e81042_d_n8, assign48040_e81042_d_n9, assign48040_e81042_d_n10, assign48040_e81042_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign48040_e81040: f64 = (locals.var_qs_1 + locals.var_qdeff);
        (assign48040_e81040, (locals.var_qs_1_dn3 + locals.var_qdeff_dn3), (locals.var_qs_1_dn4 + locals.var_qdeff_dn4), (locals.var_qs_1_dn5 + locals.var_qdeff_dn5), (locals.var_qs_1_dn6 + locals.var_qdeff_dn6), (locals.var_qs_1_dn7 + locals.var_qdeff_dn7), (locals.var_qs_1_dn8 + locals.var_qdeff_dn8), (locals.var_qs_1_dn9 + locals.var_qdeff_dn9), (locals.var_qs_1_dn10 + locals.var_qdeff_dn10), (locals.var_qs_1_dn11 + locals.var_qdeff_dn11),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign48040_e81042;
        locals.var_t0_dn3 = assign48040_e81042_d_n3;
        locals.var_t0_dn4 = assign48040_e81042_d_n4;
        locals.var_t0_dn5 = assign48040_e81042_d_n5;
        locals.var_t0_dn6 = assign48040_e81042_d_n6;
        locals.var_t0_dn7 = assign48040_e81042_d_n7;
        locals.var_t0_dn8 = assign48040_e81042_d_n8;
        locals.var_t0_dn9 = assign48040_e81042_d_n9;
        locals.var_t0_dn10 = assign48040_e81042_d_n10;
        locals.var_t0_dn11 = assign48040_e81042_d_n11;

        let (assign48050_e81049, assign48050_e81049_d_n3, assign48050_e81049_d_n4, assign48050_e81049_d_n5, assign48050_e81049_d_n6, assign48050_e81049_d_n7, assign48050_e81049_d_n8, assign48050_e81049_d_n9, assign48050_e81049_d_n10, assign48050_e81049_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign48050_e81047: f64 = (locals.var_qs_1 - locals.var_qdeff);
        (assign48050_e81047, (locals.var_qs_1_dn3 - locals.var_qdeff_dn3), (locals.var_qs_1_dn4 - locals.var_qdeff_dn4), (locals.var_qs_1_dn5 - locals.var_qdeff_dn5), (locals.var_qs_1_dn6 - locals.var_qdeff_dn6), (locals.var_qs_1_dn7 - locals.var_qdeff_dn7), (locals.var_qs_1_dn8 - locals.var_qdeff_dn8), (locals.var_qs_1_dn9 - locals.var_qdeff_dn9), (locals.var_qs_1_dn10 - locals.var_qdeff_dn10), (locals.var_qs_1_dn11 - locals.var_qdeff_dn11),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign48050_e81049;
        locals.var_t1_dn3 = assign48050_e81049_d_n3;
        locals.var_t1_dn4 = assign48050_e81049_d_n4;
        locals.var_t1_dn5 = assign48050_e81049_d_n5;
        locals.var_t1_dn6 = assign48050_e81049_d_n6;
        locals.var_t1_dn7 = assign48050_e81049_d_n7;
        locals.var_t1_dn8 = assign48050_e81049_d_n8;
        locals.var_t1_dn9 = assign48050_e81049_d_n9;
        locals.var_t1_dn10 = assign48050_e81049_d_n10;
        locals.var_t1_dn11 = assign48050_e81049_d_n11;

        let (assign48060_e81058, assign48060_e81058_d_n3, assign48060_e81058_d_n4, assign48060_e81058_d_n5, assign48060_e81058_d_n6, assign48060_e81058_d_n7, assign48060_e81058_d_n8, assign48060_e81058_d_n9, assign48060_e81058_d_n10, assign48060_e81058_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign48060_e81055: f64 = (locals.var_t0 + locals.var_m0_t);
        let assign48060_e81056: f64 = (locals.var_t1 / assign48060_e81055);
        (assign48060_e81056, (((locals.var_t1_dn3 * assign48060_e81055) - (locals.var_t1 * locals.var_t0_dn3)) / (assign48060_e81055 * assign48060_e81055)), (((locals.var_t1_dn4 * assign48060_e81055) - (locals.var_t1 * (locals.var_t0_dn4 + locals.var_m0_t_dn4))) / (assign48060_e81055 * assign48060_e81055)), (((locals.var_t1_dn5 * assign48060_e81055) - (locals.var_t1 * (locals.var_t0_dn5 + locals.var_m0_t_dn5))) / (assign48060_e81055 * assign48060_e81055)), (((locals.var_t1_dn6 * assign48060_e81055) - (locals.var_t1 * locals.var_t0_dn6)) / (assign48060_e81055 * assign48060_e81055)), (((locals.var_t1_dn7 * assign48060_e81055) - (locals.var_t1 * locals.var_t0_dn7)) / (assign48060_e81055 * assign48060_e81055)), (((locals.var_t1_dn8 * assign48060_e81055) - (locals.var_t1 * locals.var_t0_dn8)) / (assign48060_e81055 * assign48060_e81055)), (((locals.var_t1_dn9 * assign48060_e81055) - (locals.var_t1 * locals.var_t0_dn9)) / (assign48060_e81055 * assign48060_e81055)), (((locals.var_t1_dn10 * assign48060_e81055) - (locals.var_t1 * locals.var_t0_dn10)) / (assign48060_e81055 * assign48060_e81055)), (((locals.var_t1_dn11 * assign48060_e81055) - (locals.var_t1 * locals.var_t0_dn11)) / (assign48060_e81055 * assign48060_e81055)),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign48060_e81058;
        locals.var_t2_dn3 = assign48060_e81058_d_n3;
        locals.var_t2_dn4 = assign48060_e81058_d_n4;
        locals.var_t2_dn5 = assign48060_e81058_d_n5;
        locals.var_t2_dn6 = assign48060_e81058_d_n6;
        locals.var_t2_dn7 = assign48060_e81058_d_n7;
        locals.var_t2_dn8 = assign48060_e81058_d_n8;
        locals.var_t2_dn9 = assign48060_e81058_d_n9;
        locals.var_t2_dn10 = assign48060_e81058_d_n10;
        locals.var_t2_dn11 = assign48060_e81058_d_n11;

        let (assign48070_e81067, assign48070_e81067_d_n3, assign48070_e81067_d_n4, assign48070_e81067_d_n5, assign48070_e81067_d_n6, assign48070_e81067_d_n7, assign48070_e81067_d_n8, assign48070_e81067_d_n9, assign48070_e81067_d_n10, assign48070_e81067_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign48070_e81063: f64 = (locals.var_k0_t * locals.var_t2);
        let assign48070_e81065: f64 = (assign48070_e81063 * locals.var_t2);
        (assign48070_e81065, (((locals.var_k0_t * locals.var_t2_dn3) * locals.var_t2) + (assign48070_e81063 * locals.var_t2_dn3)), ((((locals.var_k0_t_dn4 * locals.var_t2) + (locals.var_k0_t * locals.var_t2_dn4)) * locals.var_t2) + (assign48070_e81063 * locals.var_t2_dn4)), ((((locals.var_k0_t_dn5 * locals.var_t2) + (locals.var_k0_t * locals.var_t2_dn5)) * locals.var_t2) + (assign48070_e81063 * locals.var_t2_dn5)), (((locals.var_k0_t * locals.var_t2_dn6) * locals.var_t2) + (assign48070_e81063 * locals.var_t2_dn6)), (((locals.var_k0_t * locals.var_t2_dn7) * locals.var_t2) + (assign48070_e81063 * locals.var_t2_dn7)), (((locals.var_k0_t * locals.var_t2_dn8) * locals.var_t2) + (assign48070_e81063 * locals.var_t2_dn8)), (((locals.var_k0_t * locals.var_t2_dn9) * locals.var_t2) + (assign48070_e81063 * locals.var_t2_dn9)), (((locals.var_k0_t * locals.var_t2_dn10) * locals.var_t2) + (assign48070_e81063 * locals.var_t2_dn10)), (((locals.var_k0_t * locals.var_t2_dn11) * locals.var_t2) + (assign48070_e81063 * locals.var_t2_dn11)),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign48070_e81067;
        locals.var_t3_dn3 = assign48070_e81067_d_n3;
        locals.var_t3_dn4 = assign48070_e81067_d_n4;
        locals.var_t3_dn5 = assign48070_e81067_d_n5;
        locals.var_t3_dn6 = assign48070_e81067_d_n6;
        locals.var_t3_dn7 = assign48070_e81067_d_n7;
        locals.var_t3_dn8 = assign48070_e81067_d_n8;
        locals.var_t3_dn9 = assign48070_e81067_d_n9;
        locals.var_t3_dn10 = assign48070_e81067_d_n10;
        locals.var_t3_dn11 = assign48070_e81067_d_n11;

        let (assign48080_e81074, assign48080_e81074_d_n3, assign48080_e81074_d_n4, assign48080_e81074_d_n5, assign48080_e81074_d_n6, assign48080_e81074_d_n7, assign48080_e81074_d_n8, assign48080_e81074_d_n9, assign48080_e81074_d_n10, assign48080_e81074_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign48080_e81072: f64 = (1.0 + locals.var_t3);
        (assign48080_e81072, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    } else {
        (locals.var_mnud, locals.var_mnud_dn3, locals.var_mnud_dn4, locals.var_mnud_dn5, locals.var_mnud_dn6, locals.var_mnud_dn7, locals.var_mnud_dn8, locals.var_mnud_dn9, locals.var_mnud_dn10, locals.var_mnud_dn11,)
    }
};
        locals.var_mnud = assign48080_e81074;
        locals.var_mnud_dn3 = assign48080_e81074_d_n3;
        locals.var_mnud_dn4 = assign48080_e81074_d_n4;
        locals.var_mnud_dn5 = assign48080_e81074_d_n5;
        locals.var_mnud_dn6 = assign48080_e81074_d_n6;
        locals.var_mnud_dn7 = assign48080_e81074_d_n7;
        locals.var_mnud_dn8 = assign48080_e81074_d_n8;
        locals.var_mnud_dn9 = assign48080_e81074_d_n9;
        locals.var_mnud_dn10 = assign48080_e81074_d_n10;
        locals.var_mnud_dn11 = assign48080_e81074_d_n11;

        let (assign48090_e81097, assign48090_e81097_d_n3, assign48090_e81097_d_n4, assign48090_e81097_d_n5, assign48090_e81097_d_n6, assign48090_e81097_d_n7, assign48090_e81097_d_n8, assign48090_e81097_d_n9, assign48090_e81097_d_n10, assign48090_e81097_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign48090_e81082: f64 = (locals.var_c0sisat_t * locals.var_t1);
        let assign48090_e81084: f64 = (assign48090_e81082 * locals.var_t1);
        let assign48090_e81085: f64 = (locals.var_c0si_t + assign48090_e81084);
        let assign48090_e81086: f64 = (0.0_f64).max(assign48090_e81085);
        let assign48090_e81088: f64 = (assign48090_e81086 * locals.var_t0);
        let assign48090_e81091: f64 = (2.0 * locals.var_n);
        let assign48090_e81093: f64 = (assign48090_e81091 * locals.var_vtm);
        let assign48090_e81094: f64 = (assign48090_e81088 + assign48090_e81093);
        let assign48090_e81095: f64 = (locals.var_c0_t / assign48090_e81094);
        (assign48090_e81095, (-((locals.var_c0_t * (((if 0.0 >= assign48090_e81085 { 0.0 } else { (((locals.var_c0sisat_t * locals.var_t1_dn3) * locals.var_t1) + (assign48090_e81082 * locals.var_t1_dn3)) } * locals.var_t0) + (assign48090_e81086 * locals.var_t0_dn3)) + ((2.0 * locals.var_n_dn3) * locals.var_vtm))) / (assign48090_e81094 * assign48090_e81094))), (((locals.var_c0_t_dn4 * assign48090_e81094) - (locals.var_c0_t * (((if 0.0 >= assign48090_e81085 { 0.0 } else { (locals.var_c0si_t_dn4 + ((((locals.var_c0sisat_t_dn4 * locals.var_t1) + (locals.var_c0sisat_t * locals.var_t1_dn4)) * locals.var_t1) + (assign48090_e81082 * locals.var_t1_dn4))) } * locals.var_t0) + (assign48090_e81086 * locals.var_t0_dn4)) + (((2.0 * locals.var_n_dn4) * locals.var_vtm) + (assign48090_e81091 * locals.var_vtm_dn4))))) / (assign48090_e81094 * assign48090_e81094)), (((locals.var_c0_t_dn5 * assign48090_e81094) - (locals.var_c0_t * (((if 0.0 >= assign48090_e81085 { 0.0 } else { (locals.var_c0si_t_dn5 + ((((locals.var_c0sisat_t_dn5 * locals.var_t1) + (locals.var_c0sisat_t * locals.var_t1_dn5)) * locals.var_t1) + (assign48090_e81082 * locals.var_t1_dn5))) } * locals.var_t0) + (assign48090_e81086 * locals.var_t0_dn5)) + (((2.0 * locals.var_n_dn5) * locals.var_vtm) + (assign48090_e81091 * locals.var_vtm_dn5))))) / (assign48090_e81094 * assign48090_e81094)), (-((locals.var_c0_t * (((if 0.0 >= assign48090_e81085 { 0.0 } else { (((locals.var_c0sisat_t * locals.var_t1_dn6) * locals.var_t1) + (assign48090_e81082 * locals.var_t1_dn6)) } * locals.var_t0) + (assign48090_e81086 * locals.var_t0_dn6)) + ((2.0 * locals.var_n_dn6) * locals.var_vtm))) / (assign48090_e81094 * assign48090_e81094))), (-((locals.var_c0_t * (((if 0.0 >= assign48090_e81085 { 0.0 } else { (((locals.var_c0sisat_t * locals.var_t1_dn7) * locals.var_t1) + (assign48090_e81082 * locals.var_t1_dn7)) } * locals.var_t0) + (assign48090_e81086 * locals.var_t0_dn7)) + ((2.0 * locals.var_n_dn7) * locals.var_vtm))) / (assign48090_e81094 * assign48090_e81094))), (-((locals.var_c0_t * (((if 0.0 >= assign48090_e81085 { 0.0 } else { (((locals.var_c0sisat_t * locals.var_t1_dn8) * locals.var_t1) + (assign48090_e81082 * locals.var_t1_dn8)) } * locals.var_t0) + (assign48090_e81086 * locals.var_t0_dn8)) + ((2.0 * locals.var_n_dn8) * locals.var_vtm))) / (assign48090_e81094 * assign48090_e81094))), (-((locals.var_c0_t * (((if 0.0 >= assign48090_e81085 { 0.0 } else { (((locals.var_c0sisat_t * locals.var_t1_dn9) * locals.var_t1) + (assign48090_e81082 * locals.var_t1_dn9)) } * locals.var_t0) + (assign48090_e81086 * locals.var_t0_dn9)) + ((2.0 * locals.var_n_dn9) * locals.var_vtm))) / (assign48090_e81094 * assign48090_e81094))), (-((locals.var_c0_t * (((if 0.0 >= assign48090_e81085 { 0.0 } else { (((locals.var_c0sisat_t * locals.var_t1_dn10) * locals.var_t1) + (assign48090_e81082 * locals.var_t1_dn10)) } * locals.var_t0) + (assign48090_e81086 * locals.var_t0_dn10)) + ((2.0 * locals.var_n_dn10) * locals.var_vtm))) / (assign48090_e81094 * assign48090_e81094))), (-((locals.var_c0_t * (((if 0.0 >= assign48090_e81085 { 0.0 } else { (((locals.var_c0sisat_t * locals.var_t1_dn11) * locals.var_t1) + (assign48090_e81082 * locals.var_t1_dn11)) } * locals.var_t0) + (assign48090_e81086 * locals.var_t0_dn11)) + ((2.0 * locals.var_n_dn11) * locals.var_vtm))) / (assign48090_e81094 * assign48090_e81094))),)
    } else {
        (locals.var_t9, locals.var_t9_dn3, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11,)
    }
};
        locals.var_t9 = assign48090_e81097;
        locals.var_t9_dn3 = assign48090_e81097_d_n3;
        locals.var_t9_dn4 = assign48090_e81097_d_n4;
        locals.var_t9_dn5 = assign48090_e81097_d_n5;
        locals.var_t9_dn6 = assign48090_e81097_d_n6;
        locals.var_t9_dn7 = assign48090_e81097_d_n7;
        locals.var_t9_dn8 = assign48090_e81097_d_n8;
        locals.var_t9_dn9 = assign48090_e81097_d_n9;
        locals.var_t9_dn10 = assign48090_e81097_d_n10;
        locals.var_t9_dn11 = assign48090_e81097_d_n11;

        let (assign48100_e81104, assign48100_e81104_d_n3, assign48100_e81104_d_n4, assign48100_e81104_d_n5, assign48100_e81104_d_n6, assign48100_e81104_d_n7, assign48100_e81104_d_n8, assign48100_e81104_d_n9, assign48100_e81104_d_n10, assign48100_e81104_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign48100_e81101: f64 = (-locals.var_t9);
        let assign48100_e81102: f64 = { let limited_exp_arg = assign48100_e81101; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign48100_e81102, ({ let limited_exp_arg = assign48100_e81101; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t9_dn3)), ({ let limited_exp_arg = assign48100_e81101; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t9_dn4)), ({ let limited_exp_arg = assign48100_e81101; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t9_dn5)), ({ let limited_exp_arg = assign48100_e81101; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t9_dn6)), ({ let limited_exp_arg = assign48100_e81101; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t9_dn7)), ({ let limited_exp_arg = assign48100_e81101; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t9_dn8)), ({ let limited_exp_arg = assign48100_e81101; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t9_dn9)), ({ let limited_exp_arg = assign48100_e81101; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t9_dn10)), ({ let limited_exp_arg = assign48100_e81101; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t9_dn11)),)
    } else {
        (locals.var_mnud1, locals.var_mnud1_dn3, locals.var_mnud1_dn4, locals.var_mnud1_dn5, locals.var_mnud1_dn6, locals.var_mnud1_dn7, locals.var_mnud1_dn8, locals.var_mnud1_dn9, locals.var_mnud1_dn10, locals.var_mnud1_dn11,)
    }
};
        locals.var_mnud1 = assign48100_e81104;
        locals.var_mnud1_dn3 = assign48100_e81104_d_n3;
        locals.var_mnud1_dn4 = assign48100_e81104_d_n4;
        locals.var_mnud1_dn5 = assign48100_e81104_d_n5;
        locals.var_mnud1_dn6 = assign48100_e81104_d_n6;
        locals.var_mnud1_dn7 = assign48100_e81104_d_n7;
        locals.var_mnud1_dn8 = assign48100_e81104_d_n8;
        locals.var_mnud1_dn9 = assign48100_e81104_d_n9;
        locals.var_mnud1_dn10 = assign48100_e81104_d_n10;
        locals.var_mnud1_dn11 = assign48100_e81104_d_n11;

        let (assign48110_e81113, assign48110_e81113_d_n3, assign48110_e81113_d_n4, assign48110_e81113_d_n5, assign48110_e81113_d_n6, assign48110_e81113_d_n7, assign48110_e81113_d_n8, assign48110_e81113_d_n9, assign48110_e81113_d_n10, assign48110_e81113_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign48110_e81109: f64 = (locals.var_dmob * locals.var_dvsat);
        let assign48110_e81111: f64 = (assign48110_e81109 * locals.var_dr);
        (assign48110_e81111, ((((locals.var_dmob_dn3 * locals.var_dvsat) + (locals.var_dmob * locals.var_dvsat_dn3)) * locals.var_dr) + (assign48110_e81109 * locals.var_dr_dn3)), ((((locals.var_dmob_dn4 * locals.var_dvsat) + (locals.var_dmob * locals.var_dvsat_dn4)) * locals.var_dr) + (assign48110_e81109 * locals.var_dr_dn4)), ((((locals.var_dmob_dn5 * locals.var_dvsat) + (locals.var_dmob * locals.var_dvsat_dn5)) * locals.var_dr) + (assign48110_e81109 * locals.var_dr_dn5)), ((((locals.var_dmob_dn6 * locals.var_dvsat) + (locals.var_dmob * locals.var_dvsat_dn6)) * locals.var_dr) + (assign48110_e81109 * locals.var_dr_dn6)), ((((locals.var_dmob_dn7 * locals.var_dvsat) + (locals.var_dmob * locals.var_dvsat_dn7)) * locals.var_dr) + (assign48110_e81109 * locals.var_dr_dn7)), ((((locals.var_dmob_dn8 * locals.var_dvsat) + (locals.var_dmob * locals.var_dvsat_dn8)) * locals.var_dr) + (assign48110_e81109 * locals.var_dr_dn8)), ((((locals.var_dmob_dn9 * locals.var_dvsat) + (locals.var_dmob * locals.var_dvsat_dn9)) * locals.var_dr) + (assign48110_e81109 * locals.var_dr_dn9)), ((((locals.var_dmob_dn10 * locals.var_dvsat) + (locals.var_dmob * locals.var_dvsat_dn10)) * locals.var_dr) + (assign48110_e81109 * locals.var_dr_dn10)), ((((locals.var_dmob_dn11 * locals.var_dvsat) + (locals.var_dmob * locals.var_dvsat_dn11)) * locals.var_dr) + (assign48110_e81109 * locals.var_dr_dn11)),)
    } else {
        (locals.var_dtot, locals.var_dtot_dn3, locals.var_dtot_dn4, locals.var_dtot_dn5, locals.var_dtot_dn6, locals.var_dtot_dn7, locals.var_dtot_dn8, locals.var_dtot_dn9, locals.var_dtot_dn10, locals.var_dtot_dn11,)
    }
};
        locals.var_dtot = assign48110_e81113;
        locals.var_dtot_dn3 = assign48110_e81113_d_n3;
        locals.var_dtot_dn4 = assign48110_e81113_d_n4;
        locals.var_dtot_dn5 = assign48110_e81113_d_n5;
        locals.var_dtot_dn6 = assign48110_e81113_d_n6;
        locals.var_dtot_dn7 = assign48110_e81113_d_n7;
        locals.var_dtot_dn8 = assign48110_e81113_d_n8;
        locals.var_dtot_dn9 = assign48110_e81113_d_n9;
        locals.var_dtot_dn10 = assign48110_e81113_d_n10;
        locals.var_dtot_dn11 = assign48110_e81113_d_n11;

    }

    pub(super) fn stamp_transient_block_163(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign48120_e81120, assign48120_e81120_d_n3, assign48120_e81120_d_n4, assign48120_e81120_d_n5, assign48120_e81120_d_n6, assign48120_e81120_d_n7, assign48120_e81120_d_n8, assign48120_e81120_d_n9, assign48120_e81120_d_n10, assign48120_e81120_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign48120_e81118: f64 = (locals.var_u0_a / locals.var_dtot);
        (assign48120_e81118, (((locals.var_u0_a_dn3 * locals.var_dtot) - (locals.var_u0_a * locals.var_dtot_dn3)) / (locals.var_dtot * locals.var_dtot)), (((locals.var_u0_a_dn4 * locals.var_dtot) - (locals.var_u0_a * locals.var_dtot_dn4)) / (locals.var_dtot * locals.var_dtot)), (((locals.var_u0_a_dn5 * locals.var_dtot) - (locals.var_u0_a * locals.var_dtot_dn5)) / (locals.var_dtot * locals.var_dtot)), (((locals.var_u0_a_dn6 * locals.var_dtot) - (locals.var_u0_a * locals.var_dtot_dn6)) / (locals.var_dtot * locals.var_dtot)), (((locals.var_u0_a_dn7 * locals.var_dtot) - (locals.var_u0_a * locals.var_dtot_dn7)) / (locals.var_dtot * locals.var_dtot)), (((locals.var_u0_a_dn8 * locals.var_dtot) - (locals.var_u0_a * locals.var_dtot_dn8)) / (locals.var_dtot * locals.var_dtot)), (((locals.var_u0_a_dn9 * locals.var_dtot) - (locals.var_u0_a * locals.var_dtot_dn9)) / (locals.var_dtot * locals.var_dtot)), (((locals.var_u0_a_dn10 * locals.var_dtot) - (locals.var_u0_a * locals.var_dtot_dn10)) / (locals.var_dtot * locals.var_dtot)), (((locals.var_u0_a_dn11 * locals.var_dtot) - (locals.var_u0_a * locals.var_dtot_dn11)) / (locals.var_dtot * locals.var_dtot)),)
    } else {
        (locals.var_ueff, locals.var_ueff_dn3, locals.var_ueff_dn4, locals.var_ueff_dn5, locals.var_ueff_dn6, locals.var_ueff_dn7, locals.var_ueff_dn8, locals.var_ueff_dn9, locals.var_ueff_dn10, locals.var_ueff_dn11,)
    }
};
        locals.var_ueff = assign48120_e81120;
        locals.var_ueff_dn3 = assign48120_e81120_d_n3;
        locals.var_ueff_dn4 = assign48120_e81120_d_n4;
        locals.var_ueff_dn5 = assign48120_e81120_d_n5;
        locals.var_ueff_dn6 = assign48120_e81120_d_n6;
        locals.var_ueff_dn7 = assign48120_e81120_d_n7;
        locals.var_ueff_dn8 = assign48120_e81120_d_n8;
        locals.var_ueff_dn9 = assign48120_e81120_d_n9;
        locals.var_ueff_dn10 = assign48120_e81120_d_n10;
        locals.var_ueff_dn11 = assign48120_e81120_d_n11;

        let (assign48130_e81159, assign48130_e81159_d_n3, assign48130_e81159_d_n4, assign48130_e81159_d_n5, assign48130_e81159_d_n6, assign48130_e81159_d_n7, assign48130_e81159_d_n8, assign48130_e81159_d_n9, assign48130_e81159_d_n10, assign48130_e81159_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign48130_e81125: f64 = (2.0 * p.p2);
        let assign48130_e81127: f64 = (assign48130_e81125 * locals.var_nq);
        let assign48130_e81129: f64 = (assign48130_e81127 * locals.var_ueff);
        let assign48130_e81131: f64 = (assign48130_e81129 * locals.var_weff);
        let assign48130_e81133: f64 = (assign48130_e81131 / locals.var_leff);
        let assign48130_e81135: f64 = (assign48130_e81133 * locals.var_cox);
        let assign48130_e81137: f64 = (assign48130_e81135 * locals.var_nvt);
        let assign48130_e81139: f64 = (assign48130_e81137 * locals.var_nvt);
        let assign48130_e81142: f64 = (locals.var_qs_1 - locals.var_qdeff);
        let assign48130_e81145: f64 = (1.0 + locals.var_qs_1);
        let assign48130_e81147: f64 = (assign48130_e81145 + locals.var_qdeff);
        let assign48130_e81148: f64 = (assign48130_e81142 * assign48130_e81147);
        let assign48130_e81149: f64 = (assign48130_e81139 * assign48130_e81148);
        let assign48130_e81151: f64 = (assign48130_e81149 * locals.var_moc);
        let assign48130_e81153: f64 = (assign48130_e81151 / locals.var_nsat);
        let assign48130_e81155: f64 = (assign48130_e81153 * locals.var_mnud);
        let assign48130_e81157: f64 = (assign48130_e81155 * locals.var_mnud1);
        (assign48130_e81157, (((((((((((((((((((((assign48130_e81125 * locals.var_nq_dn3) * locals.var_ueff) + (assign48130_e81127 * locals.var_ueff_dn3)) * locals.var_weff) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign48130_e81135 * locals.var_nvt_dn3)) * locals.var_nvt) + (assign48130_e81137 * locals.var_nvt_dn3)) * assign48130_e81148) + (assign48130_e81139 * (((locals.var_qs_1_dn3 - locals.var_qdeff_dn3) * assign48130_e81147) + (assign48130_e81142 * (locals.var_qs_1_dn3 + locals.var_qdeff_dn3))))) * locals.var_moc) + (assign48130_e81149 * locals.var_moc_dn3)) * locals.var_nsat) - (assign48130_e81151 * locals.var_nsat_dn3)) / (locals.var_nsat * locals.var_nsat)) * locals.var_mnud) + (assign48130_e81153 * locals.var_mnud_dn3)) * locals.var_mnud1) + (assign48130_e81155 * locals.var_mnud1_dn3)), (((((((((((((((((((((assign48130_e81125 * locals.var_nq_dn4) * locals.var_ueff) + (assign48130_e81127 * locals.var_ueff_dn4)) * locals.var_weff) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign48130_e81135 * locals.var_nvt_dn4)) * locals.var_nvt) + (assign48130_e81137 * locals.var_nvt_dn4)) * assign48130_e81148) + (assign48130_e81139 * (((locals.var_qs_1_dn4 - locals.var_qdeff_dn4) * assign48130_e81147) + (assign48130_e81142 * (locals.var_qs_1_dn4 + locals.var_qdeff_dn4))))) * locals.var_moc) + (assign48130_e81149 * locals.var_moc_dn4)) * locals.var_nsat) - (assign48130_e81151 * locals.var_nsat_dn4)) / (locals.var_nsat * locals.var_nsat)) * locals.var_mnud) + (assign48130_e81153 * locals.var_mnud_dn4)) * locals.var_mnud1) + (assign48130_e81155 * locals.var_mnud1_dn4)), (((((((((((((((((((((assign48130_e81125 * locals.var_nq_dn5) * locals.var_ueff) + (assign48130_e81127 * locals.var_ueff_dn5)) * locals.var_weff) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign48130_e81135 * locals.var_nvt_dn5)) * locals.var_nvt) + (assign48130_e81137 * locals.var_nvt_dn5)) * assign48130_e81148) + (assign48130_e81139 * (((locals.var_qs_1_dn5 - locals.var_qdeff_dn5) * assign48130_e81147) + (assign48130_e81142 * (locals.var_qs_1_dn5 + locals.var_qdeff_dn5))))) * locals.var_moc) + (assign48130_e81149 * locals.var_moc_dn5)) * locals.var_nsat) - (assign48130_e81151 * locals.var_nsat_dn5)) / (locals.var_nsat * locals.var_nsat)) * locals.var_mnud) + (assign48130_e81153 * locals.var_mnud_dn5)) * locals.var_mnud1) + (assign48130_e81155 * locals.var_mnud1_dn5)), (((((((((((((((((((((assign48130_e81125 * locals.var_nq_dn6) * locals.var_ueff) + (assign48130_e81127 * locals.var_ueff_dn6)) * locals.var_weff) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign48130_e81135 * locals.var_nvt_dn6)) * locals.var_nvt) + (assign48130_e81137 * locals.var_nvt_dn6)) * assign48130_e81148) + (assign48130_e81139 * (((locals.var_qs_1_dn6 - locals.var_qdeff_dn6) * assign48130_e81147) + (assign48130_e81142 * (locals.var_qs_1_dn6 + locals.var_qdeff_dn6))))) * locals.var_moc) + (assign48130_e81149 * locals.var_moc_dn6)) * locals.var_nsat) - (assign48130_e81151 * locals.var_nsat_dn6)) / (locals.var_nsat * locals.var_nsat)) * locals.var_mnud) + (assign48130_e81153 * locals.var_mnud_dn6)) * locals.var_mnud1) + (assign48130_e81155 * locals.var_mnud1_dn6)), (((((((((((((((((((((assign48130_e81125 * locals.var_nq_dn7) * locals.var_ueff) + (assign48130_e81127 * locals.var_ueff_dn7)) * locals.var_weff) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign48130_e81135 * locals.var_nvt_dn7)) * locals.var_nvt) + (assign48130_e81137 * locals.var_nvt_dn7)) * assign48130_e81148) + (assign48130_e81139 * (((locals.var_qs_1_dn7 - locals.var_qdeff_dn7) * assign48130_e81147) + (assign48130_e81142 * (locals.var_qs_1_dn7 + locals.var_qdeff_dn7))))) * locals.var_moc) + (assign48130_e81149 * locals.var_moc_dn7)) * locals.var_nsat) - (assign48130_e81151 * locals.var_nsat_dn7)) / (locals.var_nsat * locals.var_nsat)) * locals.var_mnud) + (assign48130_e81153 * locals.var_mnud_dn7)) * locals.var_mnud1) + (assign48130_e81155 * locals.var_mnud1_dn7)), (((((((((((((((((((((assign48130_e81125 * locals.var_nq_dn8) * locals.var_ueff) + (assign48130_e81127 * locals.var_ueff_dn8)) * locals.var_weff) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign48130_e81135 * locals.var_nvt_dn8)) * locals.var_nvt) + (assign48130_e81137 * locals.var_nvt_dn8)) * assign48130_e81148) + (assign48130_e81139 * (((locals.var_qs_1_dn8 - locals.var_qdeff_dn8) * assign48130_e81147) + (assign48130_e81142 * (locals.var_qs_1_dn8 + locals.var_qdeff_dn8))))) * locals.var_moc) + (assign48130_e81149 * locals.var_moc_dn8)) * locals.var_nsat) - (assign48130_e81151 * locals.var_nsat_dn8)) / (locals.var_nsat * locals.var_nsat)) * locals.var_mnud) + (assign48130_e81153 * locals.var_mnud_dn8)) * locals.var_mnud1) + (assign48130_e81155 * locals.var_mnud1_dn8)), (((((((((((((((((((((assign48130_e81125 * locals.var_nq_dn9) * locals.var_ueff) + (assign48130_e81127 * locals.var_ueff_dn9)) * locals.var_weff) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign48130_e81135 * locals.var_nvt_dn9)) * locals.var_nvt) + (assign48130_e81137 * locals.var_nvt_dn9)) * assign48130_e81148) + (assign48130_e81139 * (((locals.var_qs_1_dn9 - locals.var_qdeff_dn9) * assign48130_e81147) + (assign48130_e81142 * (locals.var_qs_1_dn9 + locals.var_qdeff_dn9))))) * locals.var_moc) + (assign48130_e81149 * locals.var_moc_dn9)) * locals.var_nsat) - (assign48130_e81151 * locals.var_nsat_dn9)) / (locals.var_nsat * locals.var_nsat)) * locals.var_mnud) + (assign48130_e81153 * locals.var_mnud_dn9)) * locals.var_mnud1) + (assign48130_e81155 * locals.var_mnud1_dn9)), (((((((((((((((((((((assign48130_e81125 * locals.var_nq_dn10) * locals.var_ueff) + (assign48130_e81127 * locals.var_ueff_dn10)) * locals.var_weff) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign48130_e81135 * locals.var_nvt_dn10)) * locals.var_nvt) + (assign48130_e81137 * locals.var_nvt_dn10)) * assign48130_e81148) + (assign48130_e81139 * (((locals.var_qs_1_dn10 - locals.var_qdeff_dn10) * assign48130_e81147) + (assign48130_e81142 * (locals.var_qs_1_dn10 + locals.var_qdeff_dn10))))) * locals.var_moc) + (assign48130_e81149 * locals.var_moc_dn10)) * locals.var_nsat) - (assign48130_e81151 * locals.var_nsat_dn10)) / (locals.var_nsat * locals.var_nsat)) * locals.var_mnud) + (assign48130_e81153 * locals.var_mnud_dn10)) * locals.var_mnud1) + (assign48130_e81155 * locals.var_mnud1_dn10)), (((((((((((((((((((((assign48130_e81125 * locals.var_nq_dn11) * locals.var_ueff) + (assign48130_e81127 * locals.var_ueff_dn11)) * locals.var_weff) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign48130_e81135 * locals.var_nvt_dn11)) * locals.var_nvt) + (assign48130_e81137 * locals.var_nvt_dn11)) * assign48130_e81148) + (assign48130_e81139 * (((locals.var_qs_1_dn11 - locals.var_qdeff_dn11) * assign48130_e81147) + (assign48130_e81142 * (locals.var_qs_1_dn11 + locals.var_qdeff_dn11))))) * locals.var_moc) + (assign48130_e81149 * locals.var_moc_dn11)) * locals.var_nsat) - (assign48130_e81151 * locals.var_nsat_dn11)) / (locals.var_nsat * locals.var_nsat)) * locals.var_mnud) + (assign48130_e81153 * locals.var_mnud_dn11)) * locals.var_mnud1) + (assign48130_e81155 * locals.var_mnud1_dn11)),)
    } else {
        (locals.var_ids, locals.var_ids_dn3, locals.var_ids_dn4, locals.var_ids_dn5, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn8, locals.var_ids_dn9, locals.var_ids_dn10, locals.var_ids_dn11,)
    }
};
        locals.var_ids = assign48130_e81159;
        locals.var_ids_dn3 = assign48130_e81159_d_n3;
        locals.var_ids_dn4 = assign48130_e81159_d_n4;
        locals.var_ids_dn5 = assign48130_e81159_d_n5;
        locals.var_ids_dn6 = assign48130_e81159_d_n6;
        locals.var_ids_dn7 = assign48130_e81159_d_n7;
        locals.var_ids_dn8 = assign48130_e81159_d_n8;
        locals.var_ids_dn9 = assign48130_e81159_d_n9;
        locals.var_ids_dn10 = assign48130_e81159_d_n10;
        locals.var_ids_dn11 = assign48130_e81159_d_n11;

        let (assign48140_e81166, assign48140_e81166_d_n3, assign48140_e81166_d_n4, assign48140_e81166_d_n5, assign48140_e81166_d_n6, assign48140_e81166_d_n7, assign48140_e81166_d_n8, assign48140_e81166_d_n9, assign48140_e81166_d_n10, assign48140_e81166_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign48140_e81164: f64 = (locals.var_ids * p.p26);
        (assign48140_e81164, (locals.var_ids_dn3 * p.p26), (locals.var_ids_dn4 * p.p26), (locals.var_ids_dn5 * p.p26), (locals.var_ids_dn6 * p.p26), (locals.var_ids_dn7 * p.p26), (locals.var_ids_dn8 * p.p26), (locals.var_ids_dn9 * p.p26), (locals.var_ids_dn10 * p.p26), (locals.var_ids_dn11 * p.p26),)
    } else {
        (locals.var_ids, locals.var_ids_dn3, locals.var_ids_dn4, locals.var_ids_dn5, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn8, locals.var_ids_dn9, locals.var_ids_dn10, locals.var_ids_dn11,)
    }
};
        locals.var_ids = assign48140_e81166;
        locals.var_ids_dn3 = assign48140_e81166_d_n3;
        locals.var_ids_dn4 = assign48140_e81166_d_n4;
        locals.var_ids_dn5 = assign48140_e81166_d_n5;
        locals.var_ids_dn6 = assign48140_e81166_d_n6;
        locals.var_ids_dn7 = assign48140_e81166_d_n7;
        locals.var_ids_dn8 = assign48140_e81166_d_n8;
        locals.var_ids_dn9 = assign48140_e81166_d_n9;
        locals.var_ids_dn10 = assign48140_e81166_d_n10;
        locals.var_ids_dn11 = assign48140_e81166_d_n11;

        let (assign48150_e81171, assign48150_e81171_d_n3, assign48150_e81171_d_n4, assign48150_e81171_d_n5, assign48150_e81171_d_n6, assign48150_e81171_d_n7, assign48150_e81171_d_n8, assign48150_e81171_d_n9, assign48150_e81171_d_n10, assign48150_e81171_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_gcrg, locals.var_gcrg_dn3, locals.var_gcrg_dn4, locals.var_gcrg_dn5, locals.var_gcrg_dn6, locals.var_gcrg_dn7, locals.var_gcrg_dn8, locals.var_gcrg_dn9, locals.var_gcrg_dn10, locals.var_gcrg_dn11,)
    }
};
        locals.var_gcrg = assign48150_e81171;
        locals.var_gcrg_dn3 = assign48150_e81171_d_n3;
        locals.var_gcrg_dn4 = assign48150_e81171_d_n4;
        locals.var_gcrg_dn5 = assign48150_e81171_d_n5;
        locals.var_gcrg_dn6 = assign48150_e81171_d_n6;
        locals.var_gcrg_dn7 = assign48150_e81171_d_n7;
        locals.var_gcrg_dn8 = assign48150_e81171_d_n8;
        locals.var_gcrg_dn9 = assign48150_e81171_d_n9;
        locals.var_gcrg_dn10 = assign48150_e81171_d_n10;
        locals.var_gcrg_dn11 = assign48150_e81171_d_n11;

        let assign48160_e81174: f64 = if p.p7 > 1.0 { 1.0 } else { 0.0 };
        locals.var_guard741 = assign48160_e81174;

        let (assign48170_e81189, assign48170_e81189_d_n3, assign48170_e81189_d_n4, assign48170_e81189_d_n5, assign48170_e81189_d_n6, assign48170_e81189_d_n7, assign48170_e81189_d_n8, assign48170_e81189_d_n9, assign48170_e81189_d_n10, assign48170_e81189_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard741 != 0.0)) {
        let assign48170_e81181: f64 = (locals.var_ueff * locals.var_weff);
        let assign48170_e81183: f64 = (assign48170_e81181 / locals.var_leff);
        let assign48170_e81185: f64 = (assign48170_e81183 * locals.var_cox);
        let assign48170_e81187: f64 = (assign48170_e81185 * locals.var_qia);
        (assign48170_e81187, (((((locals.var_ueff_dn3 * locals.var_weff) / locals.var_leff) * locals.var_cox) * locals.var_qia) + (assign48170_e81185 * locals.var_qia_dn3)), (((((locals.var_ueff_dn4 * locals.var_weff) / locals.var_leff) * locals.var_cox) * locals.var_qia) + (assign48170_e81185 * locals.var_qia_dn4)), (((((locals.var_ueff_dn5 * locals.var_weff) / locals.var_leff) * locals.var_cox) * locals.var_qia) + (assign48170_e81185 * locals.var_qia_dn5)), (((((locals.var_ueff_dn6 * locals.var_weff) / locals.var_leff) * locals.var_cox) * locals.var_qia) + (assign48170_e81185 * locals.var_qia_dn6)), (((((locals.var_ueff_dn7 * locals.var_weff) / locals.var_leff) * locals.var_cox) * locals.var_qia) + (assign48170_e81185 * locals.var_qia_dn7)), (((((locals.var_ueff_dn8 * locals.var_weff) / locals.var_leff) * locals.var_cox) * locals.var_qia) + (assign48170_e81185 * locals.var_qia_dn8)), (((((locals.var_ueff_dn9 * locals.var_weff) / locals.var_leff) * locals.var_cox) * locals.var_qia) + (assign48170_e81185 * locals.var_qia_dn9)), (((((locals.var_ueff_dn10 * locals.var_weff) / locals.var_leff) * locals.var_cox) * locals.var_qia) + (assign48170_e81185 * locals.var_qia_dn10)), (((((locals.var_ueff_dn11 * locals.var_weff) / locals.var_leff) * locals.var_cox) * locals.var_qia) + (assign48170_e81185 * locals.var_qia_dn11)),)
    } else {
        (locals.var_idsovvds, locals.var_idsovvds_dn3, locals.var_idsovvds_dn4, locals.var_idsovvds_dn5, locals.var_idsovvds_dn6, locals.var_idsovvds_dn7, locals.var_idsovvds_dn8, locals.var_idsovvds_dn9, locals.var_idsovvds_dn10, locals.var_idsovvds_dn11,)
    }
};
        locals.var_idsovvds = assign48170_e81189;
        locals.var_idsovvds_dn3 = assign48170_e81189_d_n3;
        locals.var_idsovvds_dn4 = assign48170_e81189_d_n4;
        locals.var_idsovvds_dn5 = assign48170_e81189_d_n5;
        locals.var_idsovvds_dn6 = assign48170_e81189_d_n6;
        locals.var_idsovvds_dn7 = assign48170_e81189_d_n7;
        locals.var_idsovvds_dn8 = assign48170_e81189_d_n8;
        locals.var_idsovvds_dn9 = assign48170_e81189_d_n9;
        locals.var_idsovvds_dn10 = assign48170_e81189_d_n10;
        locals.var_idsovvds_dn11 = assign48170_e81189_d_n11;

        let (assign48180_e81198, assign48180_e81198_d_n3, assign48180_e81198_d_n4, assign48180_e81198_d_n5, assign48180_e81198_d_n6, assign48180_e81198_d_n7, assign48180_e81198_d_n8, assign48180_e81198_d_n9, assign48180_e81198_d_n10, assign48180_e81198_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard741 != 0.0)) {
        let assign48180_e81196: f64 = (p.p1009 * locals.var_vt);
        (assign48180_e81196, 0.0, (p.p1009 * locals.var_vt_dn4), (p.p1009 * locals.var_vt_dn5), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn3, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11,)
    }
};
        locals.var_t9 = assign48180_e81198;
        locals.var_t9_dn3 = assign48180_e81198_d_n3;
        locals.var_t9_dn4 = assign48180_e81198_d_n4;
        locals.var_t9_dn5 = assign48180_e81198_d_n5;
        locals.var_t9_dn6 = assign48180_e81198_d_n6;
        locals.var_t9_dn7 = assign48180_e81198_d_n7;
        locals.var_t9_dn8 = assign48180_e81198_d_n8;
        locals.var_t9_dn9 = assign48180_e81198_d_n9;
        locals.var_t9_dn10 = assign48180_e81198_d_n10;
        locals.var_t9_dn11 = assign48180_e81198_d_n11;

        let (assign48190_e81213, assign48190_e81213_d_n3, assign48190_e81213_d_n4, assign48190_e81213_d_n5, assign48190_e81213_d_n6, assign48190_e81213_d_n7, assign48190_e81213_d_n8, assign48190_e81213_d_n9, assign48190_e81213_d_n10, assign48190_e81213_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard741 != 0.0)) {
        let assign48190_e81205: f64 = (locals.var_t9 * locals.var_ueff);
        let assign48190_e81207: f64 = (assign48190_e81205 * locals.var_weff);
        let assign48190_e81209: f64 = (assign48190_e81207 / locals.var_leff);
        let assign48190_e81211: f64 = (assign48190_e81209 * locals.var_cox);
        (assign48190_e81211, (((((locals.var_t9_dn3 * locals.var_ueff) + (locals.var_t9 * locals.var_ueff_dn3)) * locals.var_weff) / locals.var_leff) * locals.var_cox), (((((locals.var_t9_dn4 * locals.var_ueff) + (locals.var_t9 * locals.var_ueff_dn4)) * locals.var_weff) / locals.var_leff) * locals.var_cox), (((((locals.var_t9_dn5 * locals.var_ueff) + (locals.var_t9 * locals.var_ueff_dn5)) * locals.var_weff) / locals.var_leff) * locals.var_cox), (((((locals.var_t9_dn6 * locals.var_ueff) + (locals.var_t9 * locals.var_ueff_dn6)) * locals.var_weff) / locals.var_leff) * locals.var_cox), (((((locals.var_t9_dn7 * locals.var_ueff) + (locals.var_t9 * locals.var_ueff_dn7)) * locals.var_weff) / locals.var_leff) * locals.var_cox), (((((locals.var_t9_dn8 * locals.var_ueff) + (locals.var_t9 * locals.var_ueff_dn8)) * locals.var_weff) / locals.var_leff) * locals.var_cox), (((((locals.var_t9_dn9 * locals.var_ueff) + (locals.var_t9 * locals.var_ueff_dn9)) * locals.var_weff) / locals.var_leff) * locals.var_cox), (((((locals.var_t9_dn10 * locals.var_ueff) + (locals.var_t9 * locals.var_ueff_dn10)) * locals.var_weff) / locals.var_leff) * locals.var_cox), (((((locals.var_t9_dn11 * locals.var_ueff) + (locals.var_t9 * locals.var_ueff_dn11)) * locals.var_weff) / locals.var_leff) * locals.var_cox),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign48190_e81213;
        locals.var_t0_dn3 = assign48190_e81213_d_n3;
        locals.var_t0_dn4 = assign48190_e81213_d_n4;
        locals.var_t0_dn5 = assign48190_e81213_d_n5;
        locals.var_t0_dn6 = assign48190_e81213_d_n6;
        locals.var_t0_dn7 = assign48190_e81213_d_n7;
        locals.var_t0_dn8 = assign48190_e81213_d_n8;
        locals.var_t0_dn9 = assign48190_e81213_d_n9;
        locals.var_t0_dn10 = assign48190_e81213_d_n10;
        locals.var_t0_dn11 = assign48190_e81213_d_n11;

        let (assign48200_e81226, assign48200_e81226_d_n3, assign48200_e81226_d_n4, assign48200_e81226_d_n5, assign48200_e81226_d_n6, assign48200_e81226_d_n7, assign48200_e81226_d_n8, assign48200_e81226_d_n9, assign48200_e81226_d_n10, assign48200_e81226_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard741 != 0.0)) {
        let assign48200_e81220: f64 = (p.p1008 * p.p2);
        let assign48200_e81223: f64 = (locals.var_t0 + locals.var_idsovvds);
        let assign48200_e81224: f64 = (assign48200_e81220 * assign48200_e81223);
        (assign48200_e81224, (assign48200_e81220 * (locals.var_t0_dn3 + locals.var_idsovvds_dn3)), (assign48200_e81220 * (locals.var_t0_dn4 + locals.var_idsovvds_dn4)), (assign48200_e81220 * (locals.var_t0_dn5 + locals.var_idsovvds_dn5)), (assign48200_e81220 * (locals.var_t0_dn6 + locals.var_idsovvds_dn6)), (assign48200_e81220 * (locals.var_t0_dn7 + locals.var_idsovvds_dn7)), (assign48200_e81220 * (locals.var_t0_dn8 + locals.var_idsovvds_dn8)), (assign48200_e81220 * (locals.var_t0_dn9 + locals.var_idsovvds_dn9)), (assign48200_e81220 * (locals.var_t0_dn10 + locals.var_idsovvds_dn10)), (assign48200_e81220 * (locals.var_t0_dn11 + locals.var_idsovvds_dn11)),)
    } else {
        (locals.var_gcrg, locals.var_gcrg_dn3, locals.var_gcrg_dn4, locals.var_gcrg_dn5, locals.var_gcrg_dn6, locals.var_gcrg_dn7, locals.var_gcrg_dn8, locals.var_gcrg_dn9, locals.var_gcrg_dn10, locals.var_gcrg_dn11,)
    }
};
        locals.var_gcrg = assign48200_e81226;
        locals.var_gcrg_dn3 = assign48200_e81226_d_n3;
        locals.var_gcrg_dn4 = assign48200_e81226_d_n4;
        locals.var_gcrg_dn5 = assign48200_e81226_d_n5;
        locals.var_gcrg_dn6 = assign48200_e81226_d_n6;
        locals.var_gcrg_dn7 = assign48200_e81226_d_n7;
        locals.var_gcrg_dn8 = assign48200_e81226_d_n8;
        locals.var_gcrg_dn9 = assign48200_e81226_d_n9;
        locals.var_gcrg_dn10 = assign48200_e81226_d_n10;
        locals.var_gcrg_dn11 = assign48200_e81226_d_n11;

        let assign48210_e81229: f64 = if p.p7 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard742 = assign48210_e81229;

        let (assign48220_e81240,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard741 != 0.0)) && (locals.var_guard742 != 0.0)) {
        let assign48220_e81238: f64 = (1.0 / locals.var_grgeltd);
        (assign48220_e81238,)
    } else {
        (locals.var_rgeltd,)
    }
};
        locals.var_rgeltd = assign48220_e81240;

        let assign48230_e81243: f64 = if locals.var_rgeltd < p.p1347 { 1.0 } else { 0.0 };
        locals.var_guard743 = assign48230_e81243;

        let (assign48240_e81254,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard741 != 0.0)) && (locals.var_guard742 != 0.0)) && (locals.var_guard743 != 0.0)) {
        (p.p1347,)
    } else {
        (locals.var_rgeltd,)
    }
};
        locals.var_rgeltd = assign48240_e81254;

        let (assign48250_e81267,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard741 != 0.0)) && (locals.var_guard742 != 0.0)) && (locals.var_guard743 != 0.0)) {
        let assign48250_e81265: f64 = (1.0 / locals.var_rgeltd);
        (assign48250_e81265,)
    } else {
        (locals.var_grgeltd,)
    }
};
        locals.var_grgeltd = assign48250_e81267;

        let (assign48260_e81278, assign48260_e81278_d_n3, assign48260_e81278_d_n4, assign48260_e81278_d_n5, assign48260_e81278_d_n6, assign48260_e81278_d_n7, assign48260_e81278_d_n8, assign48260_e81278_d_n9, assign48260_e81278_d_n10, assign48260_e81278_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard741 != 0.0)) && (locals.var_guard742 != 0.0)) {
        let assign48260_e81276: f64 = (locals.var_grgeltd + locals.var_gcrg);
        (assign48260_e81276, locals.var_gcrg_dn3, locals.var_gcrg_dn4, locals.var_gcrg_dn5, locals.var_gcrg_dn6, locals.var_gcrg_dn7, locals.var_gcrg_dn8, locals.var_gcrg_dn9, locals.var_gcrg_dn10, locals.var_gcrg_dn11,)
    } else {
        (locals.var_t11, locals.var_t11_dn3, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11,)
    }
};
        locals.var_t11 = assign48260_e81278;
        locals.var_t11_dn3 = assign48260_e81278_d_n3;
        locals.var_t11_dn4 = assign48260_e81278_d_n4;
        locals.var_t11_dn5 = assign48260_e81278_d_n5;
        locals.var_t11_dn6 = assign48260_e81278_d_n6;
        locals.var_t11_dn7 = assign48260_e81278_d_n7;
        locals.var_t11_dn8 = assign48260_e81278_d_n8;
        locals.var_t11_dn9 = assign48260_e81278_d_n9;
        locals.var_t11_dn10 = assign48260_e81278_d_n10;
        locals.var_t11_dn11 = assign48260_e81278_d_n11;

        let (assign48270_e81291, assign48270_e81291_d_n3, assign48270_e81291_d_n4, assign48270_e81291_d_n5, assign48270_e81291_d_n6, assign48270_e81291_d_n7, assign48270_e81291_d_n8, assign48270_e81291_d_n9, assign48270_e81291_d_n10, assign48270_e81291_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard741 != 0.0)) && (locals.var_guard742 != 0.0)) {
        let assign48270_e81287: f64 = (locals.var_grgeltd * locals.var_gcrg);
        let assign48270_e81289: f64 = (assign48270_e81287 / locals.var_t11);
        (assign48270_e81289, ((((locals.var_grgeltd * locals.var_gcrg_dn3) * locals.var_t11) - (assign48270_e81287 * locals.var_t11_dn3)) / (locals.var_t11 * locals.var_t11)), ((((locals.var_grgeltd * locals.var_gcrg_dn4) * locals.var_t11) - (assign48270_e81287 * locals.var_t11_dn4)) / (locals.var_t11 * locals.var_t11)), ((((locals.var_grgeltd * locals.var_gcrg_dn5) * locals.var_t11) - (assign48270_e81287 * locals.var_t11_dn5)) / (locals.var_t11 * locals.var_t11)), ((((locals.var_grgeltd * locals.var_gcrg_dn6) * locals.var_t11) - (assign48270_e81287 * locals.var_t11_dn6)) / (locals.var_t11 * locals.var_t11)), ((((locals.var_grgeltd * locals.var_gcrg_dn7) * locals.var_t11) - (assign48270_e81287 * locals.var_t11_dn7)) / (locals.var_t11 * locals.var_t11)), ((((locals.var_grgeltd * locals.var_gcrg_dn8) * locals.var_t11) - (assign48270_e81287 * locals.var_t11_dn8)) / (locals.var_t11 * locals.var_t11)), ((((locals.var_grgeltd * locals.var_gcrg_dn9) * locals.var_t11) - (assign48270_e81287 * locals.var_t11_dn9)) / (locals.var_t11 * locals.var_t11)), ((((locals.var_grgeltd * locals.var_gcrg_dn10) * locals.var_t11) - (assign48270_e81287 * locals.var_t11_dn10)) / (locals.var_t11 * locals.var_t11)), ((((locals.var_grgeltd * locals.var_gcrg_dn11) * locals.var_t11) - (assign48270_e81287 * locals.var_t11_dn11)) / (locals.var_t11 * locals.var_t11)),)
    } else {
        (locals.var_gcrg, locals.var_gcrg_dn3, locals.var_gcrg_dn4, locals.var_gcrg_dn5, locals.var_gcrg_dn6, locals.var_gcrg_dn7, locals.var_gcrg_dn8, locals.var_gcrg_dn9, locals.var_gcrg_dn10, locals.var_gcrg_dn11,)
    }
};
        locals.var_gcrg = assign48270_e81291;
        locals.var_gcrg_dn3 = assign48270_e81291_d_n3;
        locals.var_gcrg_dn4 = assign48270_e81291_d_n4;
        locals.var_gcrg_dn5 = assign48270_e81291_d_n5;
        locals.var_gcrg_dn6 = assign48270_e81291_d_n6;
        locals.var_gcrg_dn7 = assign48270_e81291_d_n7;
        locals.var_gcrg_dn8 = assign48270_e81291_d_n8;
        locals.var_gcrg_dn9 = assign48270_e81291_d_n9;
        locals.var_gcrg_dn10 = assign48270_e81291_d_n10;
        locals.var_gcrg_dn11 = assign48270_e81291_d_n11;

        let (assign48280_e81300,) = {
    if (locals.var_guard492 == 0.0) {
        let assign48280_e81296: f64 = (locals.var_weff / p.p1373);
        let assign48280_e81298: f64 = (assign48280_e81296 + p.p1377);
        (assign48280_e81298,)
    } else {
        (locals.var_wdiod,)
    }
};
        locals.var_wdiod = assign48280_e81300;

        let (assign48290_e81309,) = {
    if (locals.var_guard492 == 0.0) {
        let assign48290_e81305: f64 = (locals.var_weff / p.p1373);
        let assign48290_e81307: f64 = (assign48290_e81305 + p.p1378);
        (assign48290_e81307,)
    } else {
        (locals.var_wdios,)
    }
};
        locals.var_wdios = assign48290_e81309;

        let (assign48300_e81316,) = {
    if (locals.var_guard492 == 0.0) {
        let assign48300_e81314: f64 = (locals.var_wdios * p.p74);
        (assign48300_e81314,)
    } else {
        (locals.var_wstsi,)
    }
};
        locals.var_wstsi = assign48300_e81316;

        let (assign48310_e81323,) = {
    if (locals.var_guard492 == 0.0) {
        let assign48310_e81321: f64 = (locals.var_wdiod * p.p74);
        (assign48310_e81321,)
    } else {
        (locals.var_wdtsi,)
    }
};
        locals.var_wdtsi = assign48310_e81323;

        let (assign48320_e81330, assign48320_e81330_d_n4, assign48320_e81330_d_n5,) = {
    if (locals.var_guard492 == 0.0) {
        let assign48320_e81328: f64 = (locals.var_vtm * locals.var_ndiode_i);
        (assign48320_e81328, (locals.var_vtm_dn4 * locals.var_ndiode_i), (locals.var_vtm_dn5 * locals.var_ndiode_i),)
    } else {
        (locals.var_nvtm1, locals.var_nvtm1_dn4, locals.var_nvtm1_dn5,)
    }
};
        locals.var_nvtm1 = assign48320_e81330;
        locals.var_nvtm1_dn4 = assign48320_e81330_d_n4;
        locals.var_nvtm1_dn5 = assign48320_e81330_d_n5;

        let (assign48330_e81337, assign48330_e81337_d_n3, assign48330_e81337_d_n4, assign48330_e81337_d_n5, assign48330_e81337_d_n6, assign48330_e81337_d_n7, assign48330_e81337_d_n8, assign48330_e81337_d_n9, assign48330_e81337_d_n10, assign48330_e81337_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign48330_e81335: f64 = (locals.var_vbs_jct / locals.var_nvtm1);
        (assign48330_e81335, 0.0, (-((locals.var_vbs_jct * locals.var_nvtm1_dn4) / (locals.var_nvtm1 * locals.var_nvtm1))), (-((locals.var_vbs_jct * locals.var_nvtm1_dn5) / (locals.var_nvtm1 * locals.var_nvtm1))), 0.0, (locals.var_vbs_jct_dn7 / locals.var_nvtm1), 0.0, 0.0, (locals.var_vbs_jct_dn10 / locals.var_nvtm1), 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign48330_e81337;
        locals.var_t0_dn3 = assign48330_e81337_d_n3;
        locals.var_t0_dn4 = assign48330_e81337_d_n4;
        locals.var_t0_dn5 = assign48330_e81337_d_n5;
        locals.var_t0_dn6 = assign48330_e81337_d_n6;
        locals.var_t0_dn7 = assign48330_e81337_d_n7;
        locals.var_t0_dn8 = assign48330_e81337_d_n8;
        locals.var_t0_dn9 = assign48330_e81337_d_n9;
        locals.var_t0_dn10 = assign48330_e81337_d_n10;
        locals.var_t0_dn11 = assign48330_e81337_d_n11;

        let (assign48340_e81343, assign48340_e81343_d_n3, assign48340_e81343_d_n4, assign48340_e81343_d_n5, assign48340_e81343_d_n6, assign48340_e81343_d_n7, assign48340_e81343_d_n8, assign48340_e81343_d_n9, assign48340_e81343_d_n10, assign48340_e81343_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign48340_e81341: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign48340_e81341, ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn3), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11),)
    } else {
        (locals.var_expvbsnvtm, locals.var_expvbsnvtm_dn3, locals.var_expvbsnvtm_dn4, locals.var_expvbsnvtm_dn5, locals.var_expvbsnvtm_dn6, locals.var_expvbsnvtm_dn7, locals.var_expvbsnvtm_dn8, locals.var_expvbsnvtm_dn9, locals.var_expvbsnvtm_dn10, locals.var_expvbsnvtm_dn11,)
    }
};
        locals.var_expvbsnvtm = assign48340_e81343;
        locals.var_expvbsnvtm_dn3 = assign48340_e81343_d_n3;
        locals.var_expvbsnvtm_dn4 = assign48340_e81343_d_n4;
        locals.var_expvbsnvtm_dn5 = assign48340_e81343_d_n5;
        locals.var_expvbsnvtm_dn6 = assign48340_e81343_d_n6;
        locals.var_expvbsnvtm_dn7 = assign48340_e81343_d_n7;
        locals.var_expvbsnvtm_dn8 = assign48340_e81343_d_n8;
        locals.var_expvbsnvtm_dn9 = assign48340_e81343_d_n9;
        locals.var_expvbsnvtm_dn10 = assign48340_e81343_d_n10;
        locals.var_expvbsnvtm_dn11 = assign48340_e81343_d_n11;

        let (assign48350_e81350, assign48350_e81350_d_n4, assign48350_e81350_d_n5,) = {
    if (locals.var_guard492 == 0.0) {
        let assign48350_e81348: f64 = (locals.var_vtm * locals.var_ndiode_i);
        (assign48350_e81348, (locals.var_vtm_dn4 * locals.var_ndiode_i), (locals.var_vtm_dn5 * locals.var_ndiode_i),)
    } else {
        (locals.var_nvtm2, locals.var_nvtm2_dn4, locals.var_nvtm2_dn5,)
    }
};
        locals.var_nvtm2 = assign48350_e81350;
        locals.var_nvtm2_dn4 = assign48350_e81350_d_n4;
        locals.var_nvtm2_dn5 = assign48350_e81350_d_n5;

        let (assign48360_e81357, assign48360_e81357_d_n3, assign48360_e81357_d_n4, assign48360_e81357_d_n5, assign48360_e81357_d_n6, assign48360_e81357_d_n7, assign48360_e81357_d_n8, assign48360_e81357_d_n9, assign48360_e81357_d_n10, assign48360_e81357_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign48360_e81355: f64 = (locals.var_vbd_jct / locals.var_nvtm2);
        (assign48360_e81355, 0.0, (-((locals.var_vbd_jct * locals.var_nvtm2_dn4) / (locals.var_nvtm2 * locals.var_nvtm2))), (-((locals.var_vbd_jct * locals.var_nvtm2_dn5) / (locals.var_nvtm2 * locals.var_nvtm2))), (locals.var_vbd_jct_dn6 / locals.var_nvtm2), 0.0, 0.0, 0.0, (locals.var_vbd_jct_dn10 / locals.var_nvtm2), 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign48360_e81357;
        locals.var_t0_dn3 = assign48360_e81357_d_n3;
        locals.var_t0_dn4 = assign48360_e81357_d_n4;
        locals.var_t0_dn5 = assign48360_e81357_d_n5;
        locals.var_t0_dn6 = assign48360_e81357_d_n6;
        locals.var_t0_dn7 = assign48360_e81357_d_n7;
        locals.var_t0_dn8 = assign48360_e81357_d_n8;
        locals.var_t0_dn9 = assign48360_e81357_d_n9;
        locals.var_t0_dn10 = assign48360_e81357_d_n10;
        locals.var_t0_dn11 = assign48360_e81357_d_n11;

        let (assign48370_e81363, assign48370_e81363_d_n3, assign48370_e81363_d_n4, assign48370_e81363_d_n5, assign48370_e81363_d_n6, assign48370_e81363_d_n7, assign48370_e81363_d_n8, assign48370_e81363_d_n9, assign48370_e81363_d_n10, assign48370_e81363_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign48370_e81361: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign48370_e81361, ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn3), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11),)
    } else {
        (locals.var_expvbdnvtm, locals.var_expvbdnvtm_dn3, locals.var_expvbdnvtm_dn4, locals.var_expvbdnvtm_dn5, locals.var_expvbdnvtm_dn6, locals.var_expvbdnvtm_dn7, locals.var_expvbdnvtm_dn8, locals.var_expvbdnvtm_dn9, locals.var_expvbdnvtm_dn10, locals.var_expvbdnvtm_dn11,)
    }
};
        locals.var_expvbdnvtm = assign48370_e81363;
        locals.var_expvbdnvtm_dn3 = assign48370_e81363_d_n3;
        locals.var_expvbdnvtm_dn4 = assign48370_e81363_d_n4;
        locals.var_expvbdnvtm_dn5 = assign48370_e81363_d_n5;
        locals.var_expvbdnvtm_dn6 = assign48370_e81363_d_n6;
        locals.var_expvbdnvtm_dn7 = assign48370_e81363_d_n7;
        locals.var_expvbdnvtm_dn8 = assign48370_e81363_d_n8;
        locals.var_expvbdnvtm_dn9 = assign48370_e81363_d_n9;
        locals.var_expvbdnvtm_dn10 = assign48370_e81363_d_n10;
        locals.var_expvbdnvtm_dn11 = assign48370_e81363_d_n11;

        let (assign48380_e81374, assign48380_e81374_d_n3, assign48380_e81374_d_n4, assign48380_e81374_d_n5, assign48380_e81374_d_n6, assign48380_e81374_d_n7, assign48380_e81374_d_n8, assign48380_e81374_d_n9, assign48380_e81374_d_n10, assign48380_e81374_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign48380_e81368: f64 = (1.115 / locals.var_vtm);
        let assign48380_e81371: f64 = (locals.var_tratio - 1.0);
        let assign48380_e81372: f64 = (assign48380_e81368 * assign48380_e81371);
        (assign48380_e81372, 0.0, (((-((1.115 * locals.var_vtm_dn4) / (locals.var_vtm * locals.var_vtm))) * assign48380_e81371) + (assign48380_e81368 * locals.var_tratio_dn4)), (((-((1.115 * locals.var_vtm_dn5) / (locals.var_vtm * locals.var_vtm))) * assign48380_e81371) + (assign48380_e81368 * locals.var_tratio_dn5)), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign48380_e81374;
        locals.var_t4_dn3 = assign48380_e81374_d_n3;
        locals.var_t4_dn4 = assign48380_e81374_d_n4;
        locals.var_t4_dn5 = assign48380_e81374_d_n5;
        locals.var_t4_dn6 = assign48380_e81374_d_n6;
        locals.var_t4_dn7 = assign48380_e81374_d_n7;
        locals.var_t4_dn8 = assign48380_e81374_d_n8;
        locals.var_t4_dn9 = assign48380_e81374_d_n9;
        locals.var_t4_dn10 = assign48380_e81374_d_n10;
        locals.var_t4_dn11 = assign48380_e81374_d_n11;

        let assign48390_e81377: f64 = if locals.var_isdif_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard744 = assign48390_e81377;

        let (assign48400_e81384, assign48400_e81384_d_n3, assign48400_e81384_d_n4, assign48400_e81384_d_n5, assign48400_e81384_d_n6, assign48400_e81384_d_n7, assign48400_e81384_d_n8, assign48400_e81384_d_n9, assign48400_e81384_d_n10, assign48400_e81384_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard744 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ibs1, locals.var_ibs1_dn3, locals.var_ibs1_dn4, locals.var_ibs1_dn5, locals.var_ibs1_dn6, locals.var_ibs1_dn7, locals.var_ibs1_dn8, locals.var_ibs1_dn9, locals.var_ibs1_dn10, locals.var_ibs1_dn11,)
    }
};
        locals.var_ibs1 = assign48400_e81384;
        locals.var_ibs1_dn3 = assign48400_e81384_d_n3;
        locals.var_ibs1_dn4 = assign48400_e81384_d_n4;
        locals.var_ibs1_dn5 = assign48400_e81384_d_n5;
        locals.var_ibs1_dn6 = assign48400_e81384_d_n6;
        locals.var_ibs1_dn7 = assign48400_e81384_d_n7;
        locals.var_ibs1_dn8 = assign48400_e81384_d_n8;
        locals.var_ibs1_dn9 = assign48400_e81384_d_n9;
        locals.var_ibs1_dn10 = assign48400_e81384_d_n10;
        locals.var_ibs1_dn11 = assign48400_e81384_d_n11;

        let (assign48410_e81396, assign48410_e81396_d_n3, assign48410_e81396_d_n4, assign48410_e81396_d_n5, assign48410_e81396_d_n6, assign48410_e81396_d_n7, assign48410_e81396_d_n8, assign48410_e81396_d_n9, assign48410_e81396_d_n10, assign48410_e81396_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard744 == 0.0)) {
        let assign48410_e81392: f64 = (locals.var_xdif_i * locals.var_t4);
        let assign48410_e81394: f64 = (assign48410_e81392 / locals.var_ndiode_i);
        (assign48410_e81394, ((locals.var_xdif_i * locals.var_t4_dn3) / locals.var_ndiode_i), ((locals.var_xdif_i * locals.var_t4_dn4) / locals.var_ndiode_i), ((locals.var_xdif_i * locals.var_t4_dn5) / locals.var_ndiode_i), ((locals.var_xdif_i * locals.var_t4_dn6) / locals.var_ndiode_i), ((locals.var_xdif_i * locals.var_t4_dn7) / locals.var_ndiode_i), ((locals.var_xdif_i * locals.var_t4_dn8) / locals.var_ndiode_i), ((locals.var_xdif_i * locals.var_t4_dn9) / locals.var_ndiode_i), ((locals.var_xdif_i * locals.var_t4_dn10) / locals.var_ndiode_i), ((locals.var_xdif_i * locals.var_t4_dn11) / locals.var_ndiode_i),)
    } else {
        (locals.var_t7, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11,)
    }
};
        locals.var_t7 = assign48410_e81396;
        locals.var_t7_dn3 = assign48410_e81396_d_n3;
        locals.var_t7_dn4 = assign48410_e81396_d_n4;
        locals.var_t7_dn5 = assign48410_e81396_d_n5;
        locals.var_t7_dn6 = assign48410_e81396_d_n6;
        locals.var_t7_dn7 = assign48410_e81396_d_n7;
        locals.var_t7_dn8 = assign48410_e81396_d_n8;
        locals.var_t7_dn9 = assign48410_e81396_d_n9;
        locals.var_t7_dn10 = assign48410_e81396_d_n10;
        locals.var_t7_dn11 = assign48410_e81396_d_n11;

        let (assign48420_e81405, assign48420_e81405_d_n3, assign48420_e81405_d_n4, assign48420_e81405_d_n5, assign48420_e81405_d_n6, assign48420_e81405_d_n7, assign48420_e81405_d_n8, assign48420_e81405_d_n9, assign48420_e81405_d_n10, assign48420_e81405_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard744 == 0.0)) {
        let assign48420_e81403: f64 = { let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign48420_e81403, ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn3), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn4), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn5), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn6), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn7), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn8), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn9), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn10), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn11),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign48420_e81405;
        locals.var_t1_dn3 = assign48420_e81405_d_n3;
        locals.var_t1_dn4 = assign48420_e81405_d_n4;
        locals.var_t1_dn5 = assign48420_e81405_d_n5;
        locals.var_t1_dn6 = assign48420_e81405_d_n6;
        locals.var_t1_dn7 = assign48420_e81405_d_n7;
        locals.var_t1_dn8 = assign48420_e81405_d_n8;
        locals.var_t1_dn9 = assign48420_e81405_d_n9;
        locals.var_t1_dn10 = assign48420_e81405_d_n10;
        locals.var_t1_dn11 = assign48420_e81405_d_n11;

        let (assign48430_e81415, assign48430_e81415_d_n3, assign48430_e81415_d_n4, assign48430_e81415_d_n5, assign48430_e81415_d_n6, assign48430_e81415_d_n7, assign48430_e81415_d_n8, assign48430_e81415_d_n9, assign48430_e81415_d_n10, assign48430_e81415_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard744 == 0.0)) {
        let assign48430_e81413: f64 = (locals.var_isdif_i * locals.var_t1);
        (assign48430_e81413, (locals.var_isdif_i * locals.var_t1_dn3), (locals.var_isdif_i * locals.var_t1_dn4), (locals.var_isdif_i * locals.var_t1_dn5), (locals.var_isdif_i * locals.var_t1_dn6), (locals.var_isdif_i * locals.var_t1_dn7), (locals.var_isdif_i * locals.var_t1_dn8), (locals.var_isdif_i * locals.var_t1_dn9), (locals.var_isdif_i * locals.var_t1_dn10), (locals.var_isdif_i * locals.var_t1_dn11),)
    } else {
        (locals.var_jdifs, locals.var_jdifs_dn3, locals.var_jdifs_dn4, locals.var_jdifs_dn5, locals.var_jdifs_dn6, locals.var_jdifs_dn7, locals.var_jdifs_dn8, locals.var_jdifs_dn9, locals.var_jdifs_dn10, locals.var_jdifs_dn11,)
    }
};
        locals.var_jdifs = assign48430_e81415;
        locals.var_jdifs_dn3 = assign48430_e81415_d_n3;
        locals.var_jdifs_dn4 = assign48430_e81415_d_n4;
        locals.var_jdifs_dn5 = assign48430_e81415_d_n5;
        locals.var_jdifs_dn6 = assign48430_e81415_d_n6;
        locals.var_jdifs_dn7 = assign48430_e81415_d_n7;
        locals.var_jdifs_dn8 = assign48430_e81415_d_n8;
        locals.var_jdifs_dn9 = assign48430_e81415_d_n9;
        locals.var_jdifs_dn10 = assign48430_e81415_d_n10;
        locals.var_jdifs_dn11 = assign48430_e81415_d_n11;

    }

    pub(super) fn stamp_transient_block_164(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign48440_e81425, assign48440_e81425_d_n3, assign48440_e81425_d_n4, assign48440_e81425_d_n5, assign48440_e81425_d_n6, assign48440_e81425_d_n7, assign48440_e81425_d_n8, assign48440_e81425_d_n9, assign48440_e81425_d_n10, assign48440_e81425_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard744 == 0.0)) {
        let assign48440_e81423: f64 = (locals.var_wstsi * locals.var_jdifs);
        (assign48440_e81423, (locals.var_wstsi * locals.var_jdifs_dn3), (locals.var_wstsi * locals.var_jdifs_dn4), (locals.var_wstsi * locals.var_jdifs_dn5), (locals.var_wstsi * locals.var_jdifs_dn6), (locals.var_wstsi * locals.var_jdifs_dn7), (locals.var_wstsi * locals.var_jdifs_dn8), (locals.var_wstsi * locals.var_jdifs_dn9), (locals.var_wstsi * locals.var_jdifs_dn10), (locals.var_wstsi * locals.var_jdifs_dn11),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign48440_e81425;
        locals.var_t0_dn3 = assign48440_e81425_d_n3;
        locals.var_t0_dn4 = assign48440_e81425_d_n4;
        locals.var_t0_dn5 = assign48440_e81425_d_n5;
        locals.var_t0_dn6 = assign48440_e81425_d_n6;
        locals.var_t0_dn7 = assign48440_e81425_d_n7;
        locals.var_t0_dn8 = assign48440_e81425_d_n8;
        locals.var_t0_dn9 = assign48440_e81425_d_n9;
        locals.var_t0_dn10 = assign48440_e81425_d_n10;
        locals.var_t0_dn11 = assign48440_e81425_d_n11;

        let (assign48450_e81437, assign48450_e81437_d_n3, assign48450_e81437_d_n4, assign48450_e81437_d_n5, assign48450_e81437_d_n6, assign48450_e81437_d_n7, assign48450_e81437_d_n8, assign48450_e81437_d_n9, assign48450_e81437_d_n10, assign48450_e81437_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard744 == 0.0)) {
        let assign48450_e81434: f64 = (locals.var_expvbsnvtm - 1.0);
        let assign48450_e81435: f64 = (locals.var_t0 * assign48450_e81434);
        (assign48450_e81435, ((locals.var_t0_dn3 * assign48450_e81434) + (locals.var_t0 * locals.var_expvbsnvtm_dn3)), ((locals.var_t0_dn4 * assign48450_e81434) + (locals.var_t0 * locals.var_expvbsnvtm_dn4)), ((locals.var_t0_dn5 * assign48450_e81434) + (locals.var_t0 * locals.var_expvbsnvtm_dn5)), ((locals.var_t0_dn6 * assign48450_e81434) + (locals.var_t0 * locals.var_expvbsnvtm_dn6)), ((locals.var_t0_dn7 * assign48450_e81434) + (locals.var_t0 * locals.var_expvbsnvtm_dn7)), ((locals.var_t0_dn8 * assign48450_e81434) + (locals.var_t0 * locals.var_expvbsnvtm_dn8)), ((locals.var_t0_dn9 * assign48450_e81434) + (locals.var_t0 * locals.var_expvbsnvtm_dn9)), ((locals.var_t0_dn10 * assign48450_e81434) + (locals.var_t0 * locals.var_expvbsnvtm_dn10)), ((locals.var_t0_dn11 * assign48450_e81434) + (locals.var_t0 * locals.var_expvbsnvtm_dn11)),)
    } else {
        (locals.var_ibs1, locals.var_ibs1_dn3, locals.var_ibs1_dn4, locals.var_ibs1_dn5, locals.var_ibs1_dn6, locals.var_ibs1_dn7, locals.var_ibs1_dn8, locals.var_ibs1_dn9, locals.var_ibs1_dn10, locals.var_ibs1_dn11,)
    }
};
        locals.var_ibs1 = assign48450_e81437;
        locals.var_ibs1_dn3 = assign48450_e81437_d_n3;
        locals.var_ibs1_dn4 = assign48450_e81437_d_n4;
        locals.var_ibs1_dn5 = assign48450_e81437_d_n5;
        locals.var_ibs1_dn6 = assign48450_e81437_d_n6;
        locals.var_ibs1_dn7 = assign48450_e81437_d_n7;
        locals.var_ibs1_dn8 = assign48450_e81437_d_n8;
        locals.var_ibs1_dn9 = assign48450_e81437_d_n9;
        locals.var_ibs1_dn10 = assign48450_e81437_d_n10;
        locals.var_ibs1_dn11 = assign48450_e81437_d_n11;

        let assign48460_e81440: f64 = if locals.var_iddif_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard745 = assign48460_e81440;

        let (assign48470_e81447, assign48470_e81447_d_n3, assign48470_e81447_d_n4, assign48470_e81447_d_n5, assign48470_e81447_d_n6, assign48470_e81447_d_n7, assign48470_e81447_d_n8, assign48470_e81447_d_n9, assign48470_e81447_d_n10, assign48470_e81447_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard745 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ibd1, locals.var_ibd1_dn3, locals.var_ibd1_dn4, locals.var_ibd1_dn5, locals.var_ibd1_dn6, locals.var_ibd1_dn7, locals.var_ibd1_dn8, locals.var_ibd1_dn9, locals.var_ibd1_dn10, locals.var_ibd1_dn11,)
    }
};
        locals.var_ibd1 = assign48470_e81447;
        locals.var_ibd1_dn3 = assign48470_e81447_d_n3;
        locals.var_ibd1_dn4 = assign48470_e81447_d_n4;
        locals.var_ibd1_dn5 = assign48470_e81447_d_n5;
        locals.var_ibd1_dn6 = assign48470_e81447_d_n6;
        locals.var_ibd1_dn7 = assign48470_e81447_d_n7;
        locals.var_ibd1_dn8 = assign48470_e81447_d_n8;
        locals.var_ibd1_dn9 = assign48470_e81447_d_n9;
        locals.var_ibd1_dn10 = assign48470_e81447_d_n10;
        locals.var_ibd1_dn11 = assign48470_e81447_d_n11;

        let (assign48480_e81459, assign48480_e81459_d_n3, assign48480_e81459_d_n4, assign48480_e81459_d_n5, assign48480_e81459_d_n6, assign48480_e81459_d_n7, assign48480_e81459_d_n8, assign48480_e81459_d_n9, assign48480_e81459_d_n10, assign48480_e81459_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard745 == 0.0)) {
        let assign48480_e81455: f64 = (locals.var_xdif_i * locals.var_t4);
        let assign48480_e81457: f64 = (assign48480_e81455 / locals.var_ndiode_i);
        (assign48480_e81457, ((locals.var_xdif_i * locals.var_t4_dn3) / locals.var_ndiode_i), ((locals.var_xdif_i * locals.var_t4_dn4) / locals.var_ndiode_i), ((locals.var_xdif_i * locals.var_t4_dn5) / locals.var_ndiode_i), ((locals.var_xdif_i * locals.var_t4_dn6) / locals.var_ndiode_i), ((locals.var_xdif_i * locals.var_t4_dn7) / locals.var_ndiode_i), ((locals.var_xdif_i * locals.var_t4_dn8) / locals.var_ndiode_i), ((locals.var_xdif_i * locals.var_t4_dn9) / locals.var_ndiode_i), ((locals.var_xdif_i * locals.var_t4_dn10) / locals.var_ndiode_i), ((locals.var_xdif_i * locals.var_t4_dn11) / locals.var_ndiode_i),)
    } else {
        (locals.var_t7, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11,)
    }
};
        locals.var_t7 = assign48480_e81459;
        locals.var_t7_dn3 = assign48480_e81459_d_n3;
        locals.var_t7_dn4 = assign48480_e81459_d_n4;
        locals.var_t7_dn5 = assign48480_e81459_d_n5;
        locals.var_t7_dn6 = assign48480_e81459_d_n6;
        locals.var_t7_dn7 = assign48480_e81459_d_n7;
        locals.var_t7_dn8 = assign48480_e81459_d_n8;
        locals.var_t7_dn9 = assign48480_e81459_d_n9;
        locals.var_t7_dn10 = assign48480_e81459_d_n10;
        locals.var_t7_dn11 = assign48480_e81459_d_n11;

        let (assign48490_e81468, assign48490_e81468_d_n3, assign48490_e81468_d_n4, assign48490_e81468_d_n5, assign48490_e81468_d_n6, assign48490_e81468_d_n7, assign48490_e81468_d_n8, assign48490_e81468_d_n9, assign48490_e81468_d_n10, assign48490_e81468_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard745 == 0.0)) {
        let assign48490_e81466: f64 = { let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign48490_e81466, ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn3), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn4), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn5), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn6), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn7), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn8), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn9), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn10), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn11),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign48490_e81468;
        locals.var_t1_dn3 = assign48490_e81468_d_n3;
        locals.var_t1_dn4 = assign48490_e81468_d_n4;
        locals.var_t1_dn5 = assign48490_e81468_d_n5;
        locals.var_t1_dn6 = assign48490_e81468_d_n6;
        locals.var_t1_dn7 = assign48490_e81468_d_n7;
        locals.var_t1_dn8 = assign48490_e81468_d_n8;
        locals.var_t1_dn9 = assign48490_e81468_d_n9;
        locals.var_t1_dn10 = assign48490_e81468_d_n10;
        locals.var_t1_dn11 = assign48490_e81468_d_n11;

        let (assign48500_e81478, assign48500_e81478_d_n3, assign48500_e81478_d_n4, assign48500_e81478_d_n5, assign48500_e81478_d_n6, assign48500_e81478_d_n7, assign48500_e81478_d_n8, assign48500_e81478_d_n9, assign48500_e81478_d_n10, assign48500_e81478_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard745 == 0.0)) {
        let assign48500_e81476: f64 = (locals.var_iddif_i * locals.var_t1);
        (assign48500_e81476, (locals.var_iddif_i * locals.var_t1_dn3), (locals.var_iddif_i * locals.var_t1_dn4), (locals.var_iddif_i * locals.var_t1_dn5), (locals.var_iddif_i * locals.var_t1_dn6), (locals.var_iddif_i * locals.var_t1_dn7), (locals.var_iddif_i * locals.var_t1_dn8), (locals.var_iddif_i * locals.var_t1_dn9), (locals.var_iddif_i * locals.var_t1_dn10), (locals.var_iddif_i * locals.var_t1_dn11),)
    } else {
        (locals.var_jdifd, locals.var_jdifd_dn3, locals.var_jdifd_dn4, locals.var_jdifd_dn5, locals.var_jdifd_dn6, locals.var_jdifd_dn7, locals.var_jdifd_dn8, locals.var_jdifd_dn9, locals.var_jdifd_dn10, locals.var_jdifd_dn11,)
    }
};
        locals.var_jdifd = assign48500_e81478;
        locals.var_jdifd_dn3 = assign48500_e81478_d_n3;
        locals.var_jdifd_dn4 = assign48500_e81478_d_n4;
        locals.var_jdifd_dn5 = assign48500_e81478_d_n5;
        locals.var_jdifd_dn6 = assign48500_e81478_d_n6;
        locals.var_jdifd_dn7 = assign48500_e81478_d_n7;
        locals.var_jdifd_dn8 = assign48500_e81478_d_n8;
        locals.var_jdifd_dn9 = assign48500_e81478_d_n9;
        locals.var_jdifd_dn10 = assign48500_e81478_d_n10;
        locals.var_jdifd_dn11 = assign48500_e81478_d_n11;

        let (assign48510_e81488, assign48510_e81488_d_n3, assign48510_e81488_d_n4, assign48510_e81488_d_n5, assign48510_e81488_d_n6, assign48510_e81488_d_n7, assign48510_e81488_d_n8, assign48510_e81488_d_n9, assign48510_e81488_d_n10, assign48510_e81488_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard745 == 0.0)) {
        let assign48510_e81486: f64 = (locals.var_wdtsi * locals.var_jdifd);
        (assign48510_e81486, (locals.var_wdtsi * locals.var_jdifd_dn3), (locals.var_wdtsi * locals.var_jdifd_dn4), (locals.var_wdtsi * locals.var_jdifd_dn5), (locals.var_wdtsi * locals.var_jdifd_dn6), (locals.var_wdtsi * locals.var_jdifd_dn7), (locals.var_wdtsi * locals.var_jdifd_dn8), (locals.var_wdtsi * locals.var_jdifd_dn9), (locals.var_wdtsi * locals.var_jdifd_dn10), (locals.var_wdtsi * locals.var_jdifd_dn11),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign48510_e81488;
        locals.var_t0_dn3 = assign48510_e81488_d_n3;
        locals.var_t0_dn4 = assign48510_e81488_d_n4;
        locals.var_t0_dn5 = assign48510_e81488_d_n5;
        locals.var_t0_dn6 = assign48510_e81488_d_n6;
        locals.var_t0_dn7 = assign48510_e81488_d_n7;
        locals.var_t0_dn8 = assign48510_e81488_d_n8;
        locals.var_t0_dn9 = assign48510_e81488_d_n9;
        locals.var_t0_dn10 = assign48510_e81488_d_n10;
        locals.var_t0_dn11 = assign48510_e81488_d_n11;

        let (assign48520_e81500, assign48520_e81500_d_n3, assign48520_e81500_d_n4, assign48520_e81500_d_n5, assign48520_e81500_d_n6, assign48520_e81500_d_n7, assign48520_e81500_d_n8, assign48520_e81500_d_n9, assign48520_e81500_d_n10, assign48520_e81500_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard745 == 0.0)) {
        let assign48520_e81497: f64 = (locals.var_expvbdnvtm - 1.0);
        let assign48520_e81498: f64 = (locals.var_t0 * assign48520_e81497);
        (assign48520_e81498, ((locals.var_t0_dn3 * assign48520_e81497) + (locals.var_t0 * locals.var_expvbdnvtm_dn3)), ((locals.var_t0_dn4 * assign48520_e81497) + (locals.var_t0 * locals.var_expvbdnvtm_dn4)), ((locals.var_t0_dn5 * assign48520_e81497) + (locals.var_t0 * locals.var_expvbdnvtm_dn5)), ((locals.var_t0_dn6 * assign48520_e81497) + (locals.var_t0 * locals.var_expvbdnvtm_dn6)), ((locals.var_t0_dn7 * assign48520_e81497) + (locals.var_t0 * locals.var_expvbdnvtm_dn7)), ((locals.var_t0_dn8 * assign48520_e81497) + (locals.var_t0 * locals.var_expvbdnvtm_dn8)), ((locals.var_t0_dn9 * assign48520_e81497) + (locals.var_t0 * locals.var_expvbdnvtm_dn9)), ((locals.var_t0_dn10 * assign48520_e81497) + (locals.var_t0 * locals.var_expvbdnvtm_dn10)), ((locals.var_t0_dn11 * assign48520_e81497) + (locals.var_t0 * locals.var_expvbdnvtm_dn11)),)
    } else {
        (locals.var_ibd1, locals.var_ibd1_dn3, locals.var_ibd1_dn4, locals.var_ibd1_dn5, locals.var_ibd1_dn6, locals.var_ibd1_dn7, locals.var_ibd1_dn8, locals.var_ibd1_dn9, locals.var_ibd1_dn10, locals.var_ibd1_dn11,)
    }
};
        locals.var_ibd1 = assign48520_e81500;
        locals.var_ibd1_dn3 = assign48520_e81500_d_n3;
        locals.var_ibd1_dn4 = assign48520_e81500_d_n4;
        locals.var_ibd1_dn5 = assign48520_e81500_d_n5;
        locals.var_ibd1_dn6 = assign48520_e81500_d_n6;
        locals.var_ibd1_dn7 = assign48520_e81500_d_n7;
        locals.var_ibd1_dn8 = assign48520_e81500_d_n8;
        locals.var_ibd1_dn9 = assign48520_e81500_d_n9;
        locals.var_ibd1_dn10 = assign48520_e81500_d_n10;
        locals.var_ibd1_dn11 = assign48520_e81500_d_n11;

        let assign48530_e81503: f64 = if locals.var_isrec_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard746 = assign48530_e81503;

        let (assign48540_e81510, assign48540_e81510_d_n3, assign48540_e81510_d_n4, assign48540_e81510_d_n5, assign48540_e81510_d_n6, assign48540_e81510_d_n7, assign48540_e81510_d_n8, assign48540_e81510_d_n9, assign48540_e81510_d_n10, assign48540_e81510_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard746 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ibs2, locals.var_ibs2_dn3, locals.var_ibs2_dn4, locals.var_ibs2_dn5, locals.var_ibs2_dn6, locals.var_ibs2_dn7, locals.var_ibs2_dn8, locals.var_ibs2_dn9, locals.var_ibs2_dn10, locals.var_ibs2_dn11,)
    }
};
        locals.var_ibs2 = assign48540_e81510;
        locals.var_ibs2_dn3 = assign48540_e81510_d_n3;
        locals.var_ibs2_dn4 = assign48540_e81510_d_n4;
        locals.var_ibs2_dn5 = assign48540_e81510_d_n5;
        locals.var_ibs2_dn6 = assign48540_e81510_d_n6;
        locals.var_ibs2_dn7 = assign48540_e81510_d_n7;
        locals.var_ibs2_dn8 = assign48540_e81510_d_n8;
        locals.var_ibs2_dn9 = assign48540_e81510_d_n9;
        locals.var_ibs2_dn10 = assign48540_e81510_d_n10;
        locals.var_ibs2_dn11 = assign48540_e81510_d_n11;

        let (assign48550_e81522, assign48550_e81522_d_n3, assign48550_e81522_d_n4, assign48550_e81522_d_n5, assign48550_e81522_d_n6, assign48550_e81522_d_n7, assign48550_e81522_d_n8, assign48550_e81522_d_n9, assign48550_e81522_d_n10, assign48550_e81522_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard746 == 0.0)) {
        let assign48550_e81518: f64 = (locals.var_xrec_i * locals.var_t4);
        let assign48550_e81520: f64 = (assign48550_e81518 / locals.var_nrecf0_i);
        (assign48550_e81520, ((locals.var_xrec_i * locals.var_t4_dn3) / locals.var_nrecf0_i), ((locals.var_xrec_i * locals.var_t4_dn4) / locals.var_nrecf0_i), ((locals.var_xrec_i * locals.var_t4_dn5) / locals.var_nrecf0_i), ((locals.var_xrec_i * locals.var_t4_dn6) / locals.var_nrecf0_i), ((locals.var_xrec_i * locals.var_t4_dn7) / locals.var_nrecf0_i), ((locals.var_xrec_i * locals.var_t4_dn8) / locals.var_nrecf0_i), ((locals.var_xrec_i * locals.var_t4_dn9) / locals.var_nrecf0_i), ((locals.var_xrec_i * locals.var_t4_dn10) / locals.var_nrecf0_i), ((locals.var_xrec_i * locals.var_t4_dn11) / locals.var_nrecf0_i),)
    } else {
        (locals.var_t7, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11,)
    }
};
        locals.var_t7 = assign48550_e81522;
        locals.var_t7_dn3 = assign48550_e81522_d_n3;
        locals.var_t7_dn4 = assign48550_e81522_d_n4;
        locals.var_t7_dn5 = assign48550_e81522_d_n5;
        locals.var_t7_dn6 = assign48550_e81522_d_n6;
        locals.var_t7_dn7 = assign48550_e81522_d_n7;
        locals.var_t7_dn8 = assign48550_e81522_d_n8;
        locals.var_t7_dn9 = assign48550_e81522_d_n9;
        locals.var_t7_dn10 = assign48550_e81522_d_n10;
        locals.var_t7_dn11 = assign48550_e81522_d_n11;

        let (assign48560_e81531, assign48560_e81531_d_n3, assign48560_e81531_d_n4, assign48560_e81531_d_n5, assign48560_e81531_d_n6, assign48560_e81531_d_n7, assign48560_e81531_d_n8, assign48560_e81531_d_n9, assign48560_e81531_d_n10, assign48560_e81531_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard746 == 0.0)) {
        let assign48560_e81529: f64 = { let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign48560_e81529, ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn3), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn4), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn5), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn6), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn7), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn8), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn9), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn10), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn11),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign48560_e81531;
        locals.var_t2_dn3 = assign48560_e81531_d_n3;
        locals.var_t2_dn4 = assign48560_e81531_d_n4;
        locals.var_t2_dn5 = assign48560_e81531_d_n5;
        locals.var_t2_dn6 = assign48560_e81531_d_n6;
        locals.var_t2_dn7 = assign48560_e81531_d_n7;
        locals.var_t2_dn8 = assign48560_e81531_d_n8;
        locals.var_t2_dn9 = assign48560_e81531_d_n9;
        locals.var_t2_dn10 = assign48560_e81531_d_n10;
        locals.var_t2_dn11 = assign48560_e81531_d_n11;

        let (assign48570_e81541, assign48570_e81541_d_n3, assign48570_e81541_d_n4, assign48570_e81541_d_n5, assign48570_e81541_d_n6, assign48570_e81541_d_n7, assign48570_e81541_d_n8, assign48570_e81541_d_n9, assign48570_e81541_d_n10, assign48570_e81541_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard746 == 0.0)) {
        let assign48570_e81539: f64 = (locals.var_isrec_i * locals.var_t2);
        (assign48570_e81539, (locals.var_isrec_i * locals.var_t2_dn3), (locals.var_isrec_i * locals.var_t2_dn4), (locals.var_isrec_i * locals.var_t2_dn5), (locals.var_isrec_i * locals.var_t2_dn6), (locals.var_isrec_i * locals.var_t2_dn7), (locals.var_isrec_i * locals.var_t2_dn8), (locals.var_isrec_i * locals.var_t2_dn9), (locals.var_isrec_i * locals.var_t2_dn10), (locals.var_isrec_i * locals.var_t2_dn11),)
    } else {
        (locals.var_jrecs, locals.var_jrecs_dn3, locals.var_jrecs_dn4, locals.var_jrecs_dn5, locals.var_jrecs_dn6, locals.var_jrecs_dn7, locals.var_jrecs_dn8, locals.var_jrecs_dn9, locals.var_jrecs_dn10, locals.var_jrecs_dn11,)
    }
};
        locals.var_jrecs = assign48570_e81541;
        locals.var_jrecs_dn3 = assign48570_e81541_d_n3;
        locals.var_jrecs_dn4 = assign48570_e81541_d_n4;
        locals.var_jrecs_dn5 = assign48570_e81541_d_n5;
        locals.var_jrecs_dn6 = assign48570_e81541_d_n6;
        locals.var_jrecs_dn7 = assign48570_e81541_d_n7;
        locals.var_jrecs_dn8 = assign48570_e81541_d_n8;
        locals.var_jrecs_dn9 = assign48570_e81541_d_n9;
        locals.var_jrecs_dn10 = assign48570_e81541_d_n10;
        locals.var_jrecs_dn11 = assign48570_e81541_d_n11;

        let (assign48580_e81559, assign48580_e81559_d_n4, assign48580_e81559_d_n5,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard746 == 0.0)) {
        let assign48580_e81549: f64 = (p.p925 * locals.var_nrecf0_i);
        let assign48580_e81554: f64 = (locals.var_tratio - 1.0);
        let assign48580_e81555: f64 = (locals.var_ntrecf_i * assign48580_e81554);
        let assign48580_e81556: f64 = (1.0 + assign48580_e81555);
        let assign48580_e81557: f64 = (assign48580_e81549 * assign48580_e81556);
        (assign48580_e81557, (assign48580_e81549 * (locals.var_ntrecf_i * locals.var_tratio_dn4)), (assign48580_e81549 * (locals.var_ntrecf_i * locals.var_tratio_dn5)),)
    } else {
        (locals.var_nvtmf, locals.var_nvtmf_dn4, locals.var_nvtmf_dn5,)
    }
};
        locals.var_nvtmf = assign48580_e81559;
        locals.var_nvtmf_dn4 = assign48580_e81559_d_n4;
        locals.var_nvtmf_dn5 = assign48580_e81559_d_n5;

        let (assign48590_e81577, assign48590_e81577_d_n4, assign48590_e81577_d_n5,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard746 == 0.0)) {
        let assign48590_e81567: f64 = (p.p925 * locals.var_nrecr0_i);
        let assign48590_e81572: f64 = (locals.var_tratio - 1.0);
        let assign48590_e81573: f64 = (locals.var_ntrecr_i * assign48590_e81572);
        let assign48590_e81574: f64 = (1.0 + assign48590_e81573);
        let assign48590_e81575: f64 = (assign48590_e81567 * assign48590_e81574);
        (assign48590_e81575, (assign48590_e81567 * (locals.var_ntrecr_i * locals.var_tratio_dn4)), (assign48590_e81567 * (locals.var_ntrecr_i * locals.var_tratio_dn5)),)
    } else {
        (locals.var_nvtmr, locals.var_nvtmr_dn4, locals.var_nvtmr_dn5,)
    }
};
        locals.var_nvtmr = assign48590_e81577;
        locals.var_nvtmr_dn4 = assign48590_e81577_d_n4;
        locals.var_nvtmr_dn5 = assign48590_e81577_d_n5;

        let (assign48600_e81587, assign48600_e81587_d_n3, assign48600_e81587_d_n4, assign48600_e81587_d_n5, assign48600_e81587_d_n6, assign48600_e81587_d_n7, assign48600_e81587_d_n8, assign48600_e81587_d_n9, assign48600_e81587_d_n10, assign48600_e81587_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard746 == 0.0)) {
        let assign48600_e81585: f64 = (locals.var_vbs_jct / locals.var_nvtmf);
        (assign48600_e81585, 0.0, (-((locals.var_vbs_jct * locals.var_nvtmf_dn4) / (locals.var_nvtmf * locals.var_nvtmf))), (-((locals.var_vbs_jct * locals.var_nvtmf_dn5) / (locals.var_nvtmf * locals.var_nvtmf))), 0.0, (locals.var_vbs_jct_dn7 / locals.var_nvtmf), 0.0, 0.0, (locals.var_vbs_jct_dn10 / locals.var_nvtmf), 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign48600_e81587;
        locals.var_t0_dn3 = assign48600_e81587_d_n3;
        locals.var_t0_dn4 = assign48600_e81587_d_n4;
        locals.var_t0_dn5 = assign48600_e81587_d_n5;
        locals.var_t0_dn6 = assign48600_e81587_d_n6;
        locals.var_t0_dn7 = assign48600_e81587_d_n7;
        locals.var_t0_dn8 = assign48600_e81587_d_n8;
        locals.var_t0_dn9 = assign48600_e81587_d_n9;
        locals.var_t0_dn10 = assign48600_e81587_d_n10;
        locals.var_t0_dn11 = assign48600_e81587_d_n11;

        let (assign48610_e81596, assign48610_e81596_d_n3, assign48610_e81596_d_n4, assign48610_e81596_d_n5, assign48610_e81596_d_n6, assign48610_e81596_d_n7, assign48610_e81596_d_n8, assign48610_e81596_d_n9, assign48610_e81596_d_n10, assign48610_e81596_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard746 == 0.0)) {
        let assign48610_e81594: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign48610_e81594, ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn3), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11),)
    } else {
        (locals.var_t10, locals.var_t10_dn3, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11,)
    }
};
        locals.var_t10 = assign48610_e81596;
        locals.var_t10_dn3 = assign48610_e81596_d_n3;
        locals.var_t10_dn4 = assign48610_e81596_d_n4;
        locals.var_t10_dn5 = assign48610_e81596_d_n5;
        locals.var_t10_dn6 = assign48610_e81596_d_n6;
        locals.var_t10_dn7 = assign48610_e81596_d_n7;
        locals.var_t10_dn8 = assign48610_e81596_d_n8;
        locals.var_t10_dn9 = assign48610_e81596_d_n9;
        locals.var_t10_dn10 = assign48610_e81596_d_n10;
        locals.var_t10_dn11 = assign48610_e81596_d_n11;

        let assign48620_e81599: f64 = (locals.var_vrec0_i - locals.var_vbs_jct);
        let assign48620_e81601: f64 = if assign48620_e81599 < 0.001 { 1.0 } else { 0.0 };
        locals.var_guard747 = assign48620_e81601;

        let (assign48630_e81611, assign48630_e81611_d_n3, assign48630_e81611_d_n4, assign48630_e81611_d_n5, assign48630_e81611_d_n6, assign48630_e81611_d_n7, assign48630_e81611_d_n8, assign48630_e81611_d_n9, assign48630_e81611_d_n10, assign48630_e81611_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard746 == 0.0)) && (locals.var_guard747 != 0.0)) {
        (1000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign48630_e81611;
        locals.var_t1_dn3 = assign48630_e81611_d_n3;
        locals.var_t1_dn4 = assign48630_e81611_d_n4;
        locals.var_t1_dn5 = assign48630_e81611_d_n5;
        locals.var_t1_dn6 = assign48630_e81611_d_n6;
        locals.var_t1_dn7 = assign48630_e81611_d_n7;
        locals.var_t1_dn8 = assign48630_e81611_d_n8;
        locals.var_t1_dn9 = assign48630_e81611_d_n9;
        locals.var_t1_dn10 = assign48630_e81611_d_n10;
        locals.var_t1_dn11 = assign48630_e81611_d_n11;

        let (assign48640_e81628, assign48640_e81628_d_n3, assign48640_e81628_d_n4, assign48640_e81628_d_n5, assign48640_e81628_d_n6, assign48640_e81628_d_n7, assign48640_e81628_d_n8, assign48640_e81628_d_n9, assign48640_e81628_d_n10, assign48640_e81628_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard746 == 0.0)) && (locals.var_guard747 != 0.0)) {
        let assign48640_e81620: f64 = (-locals.var_vbs_jct);
        let assign48640_e81622: f64 = (assign48640_e81620 / locals.var_nvtmr);
        let assign48640_e81624: f64 = (assign48640_e81622 * locals.var_vrec0_i);
        let assign48640_e81626: f64 = (assign48640_e81624 * locals.var_t1);
        (assign48640_e81626, (assign48640_e81624 * locals.var_t1_dn3), ((((-((assign48640_e81620 * locals.var_nvtmr_dn4) / (locals.var_nvtmr * locals.var_nvtmr))) * locals.var_vrec0_i) * locals.var_t1) + (assign48640_e81624 * locals.var_t1_dn4)), ((((-((assign48640_e81620 * locals.var_nvtmr_dn5) / (locals.var_nvtmr * locals.var_nvtmr))) * locals.var_vrec0_i) * locals.var_t1) + (assign48640_e81624 * locals.var_t1_dn5)), (assign48640_e81624 * locals.var_t1_dn6), (((((-locals.var_vbs_jct_dn7) / locals.var_nvtmr) * locals.var_vrec0_i) * locals.var_t1) + (assign48640_e81624 * locals.var_t1_dn7)), (assign48640_e81624 * locals.var_t1_dn8), (assign48640_e81624 * locals.var_t1_dn9), (((((-locals.var_vbs_jct_dn10) / locals.var_nvtmr) * locals.var_vrec0_i) * locals.var_t1) + (assign48640_e81624 * locals.var_t1_dn10)), (assign48640_e81624 * locals.var_t1_dn11),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign48640_e81628;
        locals.var_t0_dn3 = assign48640_e81628_d_n3;
        locals.var_t0_dn4 = assign48640_e81628_d_n4;
        locals.var_t0_dn5 = assign48640_e81628_d_n5;
        locals.var_t0_dn6 = assign48640_e81628_d_n6;
        locals.var_t0_dn7 = assign48640_e81628_d_n7;
        locals.var_t0_dn8 = assign48640_e81628_d_n8;
        locals.var_t0_dn9 = assign48640_e81628_d_n9;
        locals.var_t0_dn10 = assign48640_e81628_d_n10;
        locals.var_t0_dn11 = assign48640_e81628_d_n11;

        let (assign48650_e81639, assign48650_e81639_d_n3, assign48650_e81639_d_n4, assign48650_e81639_d_n5, assign48650_e81639_d_n6, assign48650_e81639_d_n7, assign48650_e81639_d_n8, assign48650_e81639_d_n9, assign48650_e81639_d_n10, assign48650_e81639_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard746 == 0.0)) && (locals.var_guard747 != 0.0)) {
        let assign48650_e81637: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign48650_e81637, ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn3), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11),)
    } else {
        (locals.var_t11, locals.var_t11_dn3, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11,)
    }
};
        locals.var_t11 = assign48650_e81639;
        locals.var_t11_dn3 = assign48650_e81639_d_n3;
        locals.var_t11_dn4 = assign48650_e81639_d_n4;
        locals.var_t11_dn5 = assign48650_e81639_d_n5;
        locals.var_t11_dn6 = assign48650_e81639_d_n6;
        locals.var_t11_dn7 = assign48650_e81639_d_n7;
        locals.var_t11_dn8 = assign48650_e81639_d_n8;
        locals.var_t11_dn9 = assign48650_e81639_d_n9;
        locals.var_t11_dn10 = assign48650_e81639_d_n10;
        locals.var_t11_dn11 = assign48650_e81639_d_n11;

        let (assign48660_e81650, assign48660_e81650_d_n3, assign48660_e81650_d_n4, assign48660_e81650_d_n5, assign48660_e81650_d_n6, assign48660_e81650_d_n7, assign48660_e81650_d_n8, assign48660_e81650_d_n9, assign48660_e81650_d_n10, assign48660_e81650_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard746 == 0.0)) && (locals.var_guard747 != 0.0)) {
        let assign48660_e81648: f64 = (-locals.var_t11);
        (assign48660_e81648, (-locals.var_t11_dn3), (-locals.var_t11_dn4), (-locals.var_t11_dn5), (-locals.var_t11_dn6), (-locals.var_t11_dn7), (-locals.var_t11_dn8), (-locals.var_t11_dn9), (-locals.var_t11_dn10), (-locals.var_t11_dn11),)
    } else {
        (locals.var_t11, locals.var_t11_dn3, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11,)
    }
};
        locals.var_t11 = assign48660_e81650;
        locals.var_t11_dn3 = assign48660_e81650_d_n3;
        locals.var_t11_dn4 = assign48660_e81650_d_n4;
        locals.var_t11_dn5 = assign48660_e81650_d_n5;
        locals.var_t11_dn6 = assign48660_e81650_d_n6;
        locals.var_t11_dn7 = assign48660_e81650_d_n7;
        locals.var_t11_dn8 = assign48660_e81650_d_n8;
        locals.var_t11_dn9 = assign48660_e81650_d_n9;
        locals.var_t11_dn10 = assign48660_e81650_d_n10;
        locals.var_t11_dn11 = assign48660_e81650_d_n11;

        let (assign48670_e81665, assign48670_e81665_d_n3, assign48670_e81665_d_n4, assign48670_e81665_d_n5, assign48670_e81665_d_n6, assign48670_e81665_d_n7, assign48670_e81665_d_n8, assign48670_e81665_d_n9, assign48670_e81665_d_n10, assign48670_e81665_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard746 == 0.0)) && (locals.var_guard747 == 0.0)) {
        let assign48670_e81662: f64 = (locals.var_vrec0_i - locals.var_vbs_jct);
        let assign48670_e81663: f64 = (1.0 / assign48670_e81662);
        (assign48670_e81663, 0.0, 0.0, 0.0, 0.0, (-((-locals.var_vbs_jct_dn7) / (assign48670_e81662 * assign48670_e81662))), 0.0, 0.0, (-((-locals.var_vbs_jct_dn10) / (assign48670_e81662 * assign48670_e81662))), 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign48670_e81665;
        locals.var_t1_dn3 = assign48670_e81665_d_n3;
        locals.var_t1_dn4 = assign48670_e81665_d_n4;
        locals.var_t1_dn5 = assign48670_e81665_d_n5;
        locals.var_t1_dn6 = assign48670_e81665_d_n6;
        locals.var_t1_dn7 = assign48670_e81665_d_n7;
        locals.var_t1_dn8 = assign48670_e81665_d_n8;
        locals.var_t1_dn9 = assign48670_e81665_d_n9;
        locals.var_t1_dn10 = assign48670_e81665_d_n10;
        locals.var_t1_dn11 = assign48670_e81665_d_n11;

        let (assign48680_e81683, assign48680_e81683_d_n3, assign48680_e81683_d_n4, assign48680_e81683_d_n5, assign48680_e81683_d_n6, assign48680_e81683_d_n7, assign48680_e81683_d_n8, assign48680_e81683_d_n9, assign48680_e81683_d_n10, assign48680_e81683_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard746 == 0.0)) && (locals.var_guard747 == 0.0)) {
        let assign48680_e81675: f64 = (-locals.var_vbs_jct);
        let assign48680_e81677: f64 = (assign48680_e81675 / locals.var_nvtmr);
        let assign48680_e81679: f64 = (assign48680_e81677 * locals.var_vrec0_i);
        let assign48680_e81681: f64 = (assign48680_e81679 * locals.var_t1);
        (assign48680_e81681, (assign48680_e81679 * locals.var_t1_dn3), ((((-((assign48680_e81675 * locals.var_nvtmr_dn4) / (locals.var_nvtmr * locals.var_nvtmr))) * locals.var_vrec0_i) * locals.var_t1) + (assign48680_e81679 * locals.var_t1_dn4)), ((((-((assign48680_e81675 * locals.var_nvtmr_dn5) / (locals.var_nvtmr * locals.var_nvtmr))) * locals.var_vrec0_i) * locals.var_t1) + (assign48680_e81679 * locals.var_t1_dn5)), (assign48680_e81679 * locals.var_t1_dn6), (((((-locals.var_vbs_jct_dn7) / locals.var_nvtmr) * locals.var_vrec0_i) * locals.var_t1) + (assign48680_e81679 * locals.var_t1_dn7)), (assign48680_e81679 * locals.var_t1_dn8), (assign48680_e81679 * locals.var_t1_dn9), (((((-locals.var_vbs_jct_dn10) / locals.var_nvtmr) * locals.var_vrec0_i) * locals.var_t1) + (assign48680_e81679 * locals.var_t1_dn10)), (assign48680_e81679 * locals.var_t1_dn11),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign48680_e81683;
        locals.var_t0_dn3 = assign48680_e81683_d_n3;
        locals.var_t0_dn4 = assign48680_e81683_d_n4;
        locals.var_t0_dn5 = assign48680_e81683_d_n5;
        locals.var_t0_dn6 = assign48680_e81683_d_n6;
        locals.var_t0_dn7 = assign48680_e81683_d_n7;
        locals.var_t0_dn8 = assign48680_e81683_d_n8;
        locals.var_t0_dn9 = assign48680_e81683_d_n9;
        locals.var_t0_dn10 = assign48680_e81683_d_n10;
        locals.var_t0_dn11 = assign48680_e81683_d_n11;

        let (assign48690_e81695, assign48690_e81695_d_n3, assign48690_e81695_d_n4, assign48690_e81695_d_n5, assign48690_e81695_d_n6, assign48690_e81695_d_n7, assign48690_e81695_d_n8, assign48690_e81695_d_n9, assign48690_e81695_d_n10, assign48690_e81695_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard746 == 0.0)) && (locals.var_guard747 == 0.0)) {
        let assign48690_e81693: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign48690_e81693, ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn3), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11),)
    } else {
        (locals.var_t11, locals.var_t11_dn3, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11,)
    }
};
        locals.var_t11 = assign48690_e81695;
        locals.var_t11_dn3 = assign48690_e81695_d_n3;
        locals.var_t11_dn4 = assign48690_e81695_d_n4;
        locals.var_t11_dn5 = assign48690_e81695_d_n5;
        locals.var_t11_dn6 = assign48690_e81695_d_n6;
        locals.var_t11_dn7 = assign48690_e81695_d_n7;
        locals.var_t11_dn8 = assign48690_e81695_d_n8;
        locals.var_t11_dn9 = assign48690_e81695_d_n9;
        locals.var_t11_dn10 = assign48690_e81695_d_n10;
        locals.var_t11_dn11 = assign48690_e81695_d_n11;

        let (assign48700_e81707, assign48700_e81707_d_n3, assign48700_e81707_d_n4, assign48700_e81707_d_n5, assign48700_e81707_d_n6, assign48700_e81707_d_n7, assign48700_e81707_d_n8, assign48700_e81707_d_n9, assign48700_e81707_d_n10, assign48700_e81707_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard746 == 0.0)) && (locals.var_guard747 == 0.0)) {
        let assign48700_e81705: f64 = (-locals.var_t11);
        (assign48700_e81705, (-locals.var_t11_dn3), (-locals.var_t11_dn4), (-locals.var_t11_dn5), (-locals.var_t11_dn6), (-locals.var_t11_dn7), (-locals.var_t11_dn8), (-locals.var_t11_dn9), (-locals.var_t11_dn10), (-locals.var_t11_dn11),)
    } else {
        (locals.var_t11, locals.var_t11_dn3, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11,)
    }
};
        locals.var_t11 = assign48700_e81707;
        locals.var_t11_dn3 = assign48700_e81707_d_n3;
        locals.var_t11_dn4 = assign48700_e81707_d_n4;
        locals.var_t11_dn5 = assign48700_e81707_d_n5;
        locals.var_t11_dn6 = assign48700_e81707_d_n6;
        locals.var_t11_dn7 = assign48700_e81707_d_n7;
        locals.var_t11_dn8 = assign48700_e81707_d_n8;
        locals.var_t11_dn9 = assign48700_e81707_d_n9;
        locals.var_t11_dn10 = assign48700_e81707_d_n10;
        locals.var_t11_dn11 = assign48700_e81707_d_n11;

        let (assign48710_e81717, assign48710_e81717_d_n3, assign48710_e81717_d_n4, assign48710_e81717_d_n5, assign48710_e81717_d_n6, assign48710_e81717_d_n7, assign48710_e81717_d_n8, assign48710_e81717_d_n9, assign48710_e81717_d_n10, assign48710_e81717_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard746 == 0.0)) {
        let assign48710_e81715: f64 = (locals.var_wstsi * locals.var_jrecs);
        (assign48710_e81715, (locals.var_wstsi * locals.var_jrecs_dn3), (locals.var_wstsi * locals.var_jrecs_dn4), (locals.var_wstsi * locals.var_jrecs_dn5), (locals.var_wstsi * locals.var_jrecs_dn6), (locals.var_wstsi * locals.var_jrecs_dn7), (locals.var_wstsi * locals.var_jrecs_dn8), (locals.var_wstsi * locals.var_jrecs_dn9), (locals.var_wstsi * locals.var_jrecs_dn10), (locals.var_wstsi * locals.var_jrecs_dn11),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign48710_e81717;
        locals.var_t3_dn3 = assign48710_e81717_d_n3;
        locals.var_t3_dn4 = assign48710_e81717_d_n4;
        locals.var_t3_dn5 = assign48710_e81717_d_n5;
        locals.var_t3_dn6 = assign48710_e81717_d_n6;
        locals.var_t3_dn7 = assign48710_e81717_d_n7;
        locals.var_t3_dn8 = assign48710_e81717_d_n8;
        locals.var_t3_dn9 = assign48710_e81717_d_n9;
        locals.var_t3_dn10 = assign48710_e81717_d_n10;
        locals.var_t3_dn11 = assign48710_e81717_d_n11;

        let (assign48720_e81729, assign48720_e81729_d_n3, assign48720_e81729_d_n4, assign48720_e81729_d_n5, assign48720_e81729_d_n6, assign48720_e81729_d_n7, assign48720_e81729_d_n8, assign48720_e81729_d_n9, assign48720_e81729_d_n10, assign48720_e81729_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard746 == 0.0)) {
        let assign48720_e81726: f64 = (locals.var_t10 + locals.var_t11);
        let assign48720_e81727: f64 = (locals.var_t3 * assign48720_e81726);
        (assign48720_e81727, ((locals.var_t3_dn3 * assign48720_e81726) + (locals.var_t3 * (locals.var_t10_dn3 + locals.var_t11_dn3))), ((locals.var_t3_dn4 * assign48720_e81726) + (locals.var_t3 * (locals.var_t10_dn4 + locals.var_t11_dn4))), ((locals.var_t3_dn5 * assign48720_e81726) + (locals.var_t3 * (locals.var_t10_dn5 + locals.var_t11_dn5))), ((locals.var_t3_dn6 * assign48720_e81726) + (locals.var_t3 * (locals.var_t10_dn6 + locals.var_t11_dn6))), ((locals.var_t3_dn7 * assign48720_e81726) + (locals.var_t3 * (locals.var_t10_dn7 + locals.var_t11_dn7))), ((locals.var_t3_dn8 * assign48720_e81726) + (locals.var_t3 * (locals.var_t10_dn8 + locals.var_t11_dn8))), ((locals.var_t3_dn9 * assign48720_e81726) + (locals.var_t3 * (locals.var_t10_dn9 + locals.var_t11_dn9))), ((locals.var_t3_dn10 * assign48720_e81726) + (locals.var_t3 * (locals.var_t10_dn10 + locals.var_t11_dn10))), ((locals.var_t3_dn11 * assign48720_e81726) + (locals.var_t3 * (locals.var_t10_dn11 + locals.var_t11_dn11))),)
    } else {
        (locals.var_ibs2, locals.var_ibs2_dn3, locals.var_ibs2_dn4, locals.var_ibs2_dn5, locals.var_ibs2_dn6, locals.var_ibs2_dn7, locals.var_ibs2_dn8, locals.var_ibs2_dn9, locals.var_ibs2_dn10, locals.var_ibs2_dn11,)
    }
};
        locals.var_ibs2 = assign48720_e81729;
        locals.var_ibs2_dn3 = assign48720_e81729_d_n3;
        locals.var_ibs2_dn4 = assign48720_e81729_d_n4;
        locals.var_ibs2_dn5 = assign48720_e81729_d_n5;
        locals.var_ibs2_dn6 = assign48720_e81729_d_n6;
        locals.var_ibs2_dn7 = assign48720_e81729_d_n7;
        locals.var_ibs2_dn8 = assign48720_e81729_d_n8;
        locals.var_ibs2_dn9 = assign48720_e81729_d_n9;
        locals.var_ibs2_dn10 = assign48720_e81729_d_n10;
        locals.var_ibs2_dn11 = assign48720_e81729_d_n11;

        let assign48730_e81732: f64 = if locals.var_idrec_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard748 = assign48730_e81732;

    }

    pub(super) fn stamp_transient_block_165(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign48740_e81739, assign48740_e81739_d_n3, assign48740_e81739_d_n4, assign48740_e81739_d_n5, assign48740_e81739_d_n6, assign48740_e81739_d_n7, assign48740_e81739_d_n8, assign48740_e81739_d_n9, assign48740_e81739_d_n10, assign48740_e81739_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard748 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ibd2, locals.var_ibd2_dn3, locals.var_ibd2_dn4, locals.var_ibd2_dn5, locals.var_ibd2_dn6, locals.var_ibd2_dn7, locals.var_ibd2_dn8, locals.var_ibd2_dn9, locals.var_ibd2_dn10, locals.var_ibd2_dn11,)
    }
};
        locals.var_ibd2 = assign48740_e81739;
        locals.var_ibd2_dn3 = assign48740_e81739_d_n3;
        locals.var_ibd2_dn4 = assign48740_e81739_d_n4;
        locals.var_ibd2_dn5 = assign48740_e81739_d_n5;
        locals.var_ibd2_dn6 = assign48740_e81739_d_n6;
        locals.var_ibd2_dn7 = assign48740_e81739_d_n7;
        locals.var_ibd2_dn8 = assign48740_e81739_d_n8;
        locals.var_ibd2_dn9 = assign48740_e81739_d_n9;
        locals.var_ibd2_dn10 = assign48740_e81739_d_n10;
        locals.var_ibd2_dn11 = assign48740_e81739_d_n11;

        let (assign48750_e81751, assign48750_e81751_d_n3, assign48750_e81751_d_n4, assign48750_e81751_d_n5, assign48750_e81751_d_n6, assign48750_e81751_d_n7, assign48750_e81751_d_n8, assign48750_e81751_d_n9, assign48750_e81751_d_n10, assign48750_e81751_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard748 == 0.0)) {
        let assign48750_e81747: f64 = (locals.var_xrec_i * locals.var_t4);
        let assign48750_e81749: f64 = (assign48750_e81747 / locals.var_nrecf0_i);
        (assign48750_e81749, ((locals.var_xrec_i * locals.var_t4_dn3) / locals.var_nrecf0_i), ((locals.var_xrec_i * locals.var_t4_dn4) / locals.var_nrecf0_i), ((locals.var_xrec_i * locals.var_t4_dn5) / locals.var_nrecf0_i), ((locals.var_xrec_i * locals.var_t4_dn6) / locals.var_nrecf0_i), ((locals.var_xrec_i * locals.var_t4_dn7) / locals.var_nrecf0_i), ((locals.var_xrec_i * locals.var_t4_dn8) / locals.var_nrecf0_i), ((locals.var_xrec_i * locals.var_t4_dn9) / locals.var_nrecf0_i), ((locals.var_xrec_i * locals.var_t4_dn10) / locals.var_nrecf0_i), ((locals.var_xrec_i * locals.var_t4_dn11) / locals.var_nrecf0_i),)
    } else {
        (locals.var_t7, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11,)
    }
};
        locals.var_t7 = assign48750_e81751;
        locals.var_t7_dn3 = assign48750_e81751_d_n3;
        locals.var_t7_dn4 = assign48750_e81751_d_n4;
        locals.var_t7_dn5 = assign48750_e81751_d_n5;
        locals.var_t7_dn6 = assign48750_e81751_d_n6;
        locals.var_t7_dn7 = assign48750_e81751_d_n7;
        locals.var_t7_dn8 = assign48750_e81751_d_n8;
        locals.var_t7_dn9 = assign48750_e81751_d_n9;
        locals.var_t7_dn10 = assign48750_e81751_d_n10;
        locals.var_t7_dn11 = assign48750_e81751_d_n11;

        let (assign48760_e81760, assign48760_e81760_d_n3, assign48760_e81760_d_n4, assign48760_e81760_d_n5, assign48760_e81760_d_n6, assign48760_e81760_d_n7, assign48760_e81760_d_n8, assign48760_e81760_d_n9, assign48760_e81760_d_n10, assign48760_e81760_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard748 == 0.0)) {
        let assign48760_e81758: f64 = { let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign48760_e81758, ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn3), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn4), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn5), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn6), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn7), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn8), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn9), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn10), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn11),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign48760_e81760;
        locals.var_t2_dn3 = assign48760_e81760_d_n3;
        locals.var_t2_dn4 = assign48760_e81760_d_n4;
        locals.var_t2_dn5 = assign48760_e81760_d_n5;
        locals.var_t2_dn6 = assign48760_e81760_d_n6;
        locals.var_t2_dn7 = assign48760_e81760_d_n7;
        locals.var_t2_dn8 = assign48760_e81760_d_n8;
        locals.var_t2_dn9 = assign48760_e81760_d_n9;
        locals.var_t2_dn10 = assign48760_e81760_d_n10;
        locals.var_t2_dn11 = assign48760_e81760_d_n11;

        let (assign48770_e81770, assign48770_e81770_d_n3, assign48770_e81770_d_n4, assign48770_e81770_d_n5, assign48770_e81770_d_n6, assign48770_e81770_d_n7, assign48770_e81770_d_n8, assign48770_e81770_d_n9, assign48770_e81770_d_n10, assign48770_e81770_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard748 == 0.0)) {
        let assign48770_e81768: f64 = (locals.var_idrec_i * locals.var_t2);
        (assign48770_e81768, (locals.var_idrec_i * locals.var_t2_dn3), (locals.var_idrec_i * locals.var_t2_dn4), (locals.var_idrec_i * locals.var_t2_dn5), (locals.var_idrec_i * locals.var_t2_dn6), (locals.var_idrec_i * locals.var_t2_dn7), (locals.var_idrec_i * locals.var_t2_dn8), (locals.var_idrec_i * locals.var_t2_dn9), (locals.var_idrec_i * locals.var_t2_dn10), (locals.var_idrec_i * locals.var_t2_dn11),)
    } else {
        (locals.var_jrecd, locals.var_jrecd_dn3, locals.var_jrecd_dn4, locals.var_jrecd_dn5, locals.var_jrecd_dn6, locals.var_jrecd_dn7, locals.var_jrecd_dn8, locals.var_jrecd_dn9, locals.var_jrecd_dn10, locals.var_jrecd_dn11,)
    }
};
        locals.var_jrecd = assign48770_e81770;
        locals.var_jrecd_dn3 = assign48770_e81770_d_n3;
        locals.var_jrecd_dn4 = assign48770_e81770_d_n4;
        locals.var_jrecd_dn5 = assign48770_e81770_d_n5;
        locals.var_jrecd_dn6 = assign48770_e81770_d_n6;
        locals.var_jrecd_dn7 = assign48770_e81770_d_n7;
        locals.var_jrecd_dn8 = assign48770_e81770_d_n8;
        locals.var_jrecd_dn9 = assign48770_e81770_d_n9;
        locals.var_jrecd_dn10 = assign48770_e81770_d_n10;
        locals.var_jrecd_dn11 = assign48770_e81770_d_n11;

        let (assign48780_e81788, assign48780_e81788_d_n4, assign48780_e81788_d_n5,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard748 == 0.0)) {
        let assign48780_e81778: f64 = (p.p925 * locals.var_nrecf0_i);
        let assign48780_e81783: f64 = (locals.var_tratio - 1.0);
        let assign48780_e81784: f64 = (locals.var_ntrecf_i * assign48780_e81783);
        let assign48780_e81785: f64 = (1.0 + assign48780_e81784);
        let assign48780_e81786: f64 = (assign48780_e81778 * assign48780_e81785);
        (assign48780_e81786, (assign48780_e81778 * (locals.var_ntrecf_i * locals.var_tratio_dn4)), (assign48780_e81778 * (locals.var_ntrecf_i * locals.var_tratio_dn5)),)
    } else {
        (locals.var_nvtmf, locals.var_nvtmf_dn4, locals.var_nvtmf_dn5,)
    }
};
        locals.var_nvtmf = assign48780_e81788;
        locals.var_nvtmf_dn4 = assign48780_e81788_d_n4;
        locals.var_nvtmf_dn5 = assign48780_e81788_d_n5;

        let (assign48790_e81806, assign48790_e81806_d_n4, assign48790_e81806_d_n5,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard748 == 0.0)) {
        let assign48790_e81796: f64 = (p.p925 * locals.var_nrecr0_i);
        let assign48790_e81801: f64 = (locals.var_tratio - 1.0);
        let assign48790_e81802: f64 = (locals.var_ntrecr_i * assign48790_e81801);
        let assign48790_e81803: f64 = (1.0 + assign48790_e81802);
        let assign48790_e81804: f64 = (assign48790_e81796 * assign48790_e81803);
        (assign48790_e81804, (assign48790_e81796 * (locals.var_ntrecr_i * locals.var_tratio_dn4)), (assign48790_e81796 * (locals.var_ntrecr_i * locals.var_tratio_dn5)),)
    } else {
        (locals.var_nvtmr, locals.var_nvtmr_dn4, locals.var_nvtmr_dn5,)
    }
};
        locals.var_nvtmr = assign48790_e81806;
        locals.var_nvtmr_dn4 = assign48790_e81806_d_n4;
        locals.var_nvtmr_dn5 = assign48790_e81806_d_n5;

        let (assign48800_e81816, assign48800_e81816_d_n3, assign48800_e81816_d_n4, assign48800_e81816_d_n5, assign48800_e81816_d_n6, assign48800_e81816_d_n7, assign48800_e81816_d_n8, assign48800_e81816_d_n9, assign48800_e81816_d_n10, assign48800_e81816_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard748 == 0.0)) {
        let assign48800_e81814: f64 = (locals.var_vbd_jct / locals.var_nvtmf);
        (assign48800_e81814, 0.0, (-((locals.var_vbd_jct * locals.var_nvtmf_dn4) / (locals.var_nvtmf * locals.var_nvtmf))), (-((locals.var_vbd_jct * locals.var_nvtmf_dn5) / (locals.var_nvtmf * locals.var_nvtmf))), (locals.var_vbd_jct_dn6 / locals.var_nvtmf), 0.0, 0.0, 0.0, (locals.var_vbd_jct_dn10 / locals.var_nvtmf), 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign48800_e81816;
        locals.var_t0_dn3 = assign48800_e81816_d_n3;
        locals.var_t0_dn4 = assign48800_e81816_d_n4;
        locals.var_t0_dn5 = assign48800_e81816_d_n5;
        locals.var_t0_dn6 = assign48800_e81816_d_n6;
        locals.var_t0_dn7 = assign48800_e81816_d_n7;
        locals.var_t0_dn8 = assign48800_e81816_d_n8;
        locals.var_t0_dn9 = assign48800_e81816_d_n9;
        locals.var_t0_dn10 = assign48800_e81816_d_n10;
        locals.var_t0_dn11 = assign48800_e81816_d_n11;

        let (assign48810_e81825, assign48810_e81825_d_n3, assign48810_e81825_d_n4, assign48810_e81825_d_n5, assign48810_e81825_d_n6, assign48810_e81825_d_n7, assign48810_e81825_d_n8, assign48810_e81825_d_n9, assign48810_e81825_d_n10, assign48810_e81825_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard748 == 0.0)) {
        let assign48810_e81823: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign48810_e81823, ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn3), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11),)
    } else {
        (locals.var_t10, locals.var_t10_dn3, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11,)
    }
};
        locals.var_t10 = assign48810_e81825;
        locals.var_t10_dn3 = assign48810_e81825_d_n3;
        locals.var_t10_dn4 = assign48810_e81825_d_n4;
        locals.var_t10_dn5 = assign48810_e81825_d_n5;
        locals.var_t10_dn6 = assign48810_e81825_d_n6;
        locals.var_t10_dn7 = assign48810_e81825_d_n7;
        locals.var_t10_dn8 = assign48810_e81825_d_n8;
        locals.var_t10_dn9 = assign48810_e81825_d_n9;
        locals.var_t10_dn10 = assign48810_e81825_d_n10;
        locals.var_t10_dn11 = assign48810_e81825_d_n11;

        let assign48820_e81828: f64 = (locals.var_vrec0d_i - locals.var_vbd_jct);
        let assign48820_e81830: f64 = if assign48820_e81828 < 0.001 { 1.0 } else { 0.0 };
        locals.var_guard749 = assign48820_e81830;

        let (assign48830_e81840, assign48830_e81840_d_n3, assign48830_e81840_d_n4, assign48830_e81840_d_n5, assign48830_e81840_d_n6, assign48830_e81840_d_n7, assign48830_e81840_d_n8, assign48830_e81840_d_n9, assign48830_e81840_d_n10, assign48830_e81840_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard748 == 0.0)) && (locals.var_guard749 != 0.0)) {
        (1000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign48830_e81840;
        locals.var_t1_dn3 = assign48830_e81840_d_n3;
        locals.var_t1_dn4 = assign48830_e81840_d_n4;
        locals.var_t1_dn5 = assign48830_e81840_d_n5;
        locals.var_t1_dn6 = assign48830_e81840_d_n6;
        locals.var_t1_dn7 = assign48830_e81840_d_n7;
        locals.var_t1_dn8 = assign48830_e81840_d_n8;
        locals.var_t1_dn9 = assign48830_e81840_d_n9;
        locals.var_t1_dn10 = assign48830_e81840_d_n10;
        locals.var_t1_dn11 = assign48830_e81840_d_n11;

        let (assign48840_e81857, assign48840_e81857_d_n3, assign48840_e81857_d_n4, assign48840_e81857_d_n5, assign48840_e81857_d_n6, assign48840_e81857_d_n7, assign48840_e81857_d_n8, assign48840_e81857_d_n9, assign48840_e81857_d_n10, assign48840_e81857_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard748 == 0.0)) && (locals.var_guard749 != 0.0)) {
        let assign48840_e81849: f64 = (-locals.var_vbd_jct);
        let assign48840_e81851: f64 = (assign48840_e81849 / locals.var_nvtmr);
        let assign48840_e81853: f64 = (assign48840_e81851 * locals.var_vrec0d_i);
        let assign48840_e81855: f64 = (assign48840_e81853 * locals.var_t1);
        (assign48840_e81855, (assign48840_e81853 * locals.var_t1_dn3), ((((-((assign48840_e81849 * locals.var_nvtmr_dn4) / (locals.var_nvtmr * locals.var_nvtmr))) * locals.var_vrec0d_i) * locals.var_t1) + (assign48840_e81853 * locals.var_t1_dn4)), ((((-((assign48840_e81849 * locals.var_nvtmr_dn5) / (locals.var_nvtmr * locals.var_nvtmr))) * locals.var_vrec0d_i) * locals.var_t1) + (assign48840_e81853 * locals.var_t1_dn5)), (((((-locals.var_vbd_jct_dn6) / locals.var_nvtmr) * locals.var_vrec0d_i) * locals.var_t1) + (assign48840_e81853 * locals.var_t1_dn6)), (assign48840_e81853 * locals.var_t1_dn7), (assign48840_e81853 * locals.var_t1_dn8), (assign48840_e81853 * locals.var_t1_dn9), (((((-locals.var_vbd_jct_dn10) / locals.var_nvtmr) * locals.var_vrec0d_i) * locals.var_t1) + (assign48840_e81853 * locals.var_t1_dn10)), (assign48840_e81853 * locals.var_t1_dn11),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign48840_e81857;
        locals.var_t0_dn3 = assign48840_e81857_d_n3;
        locals.var_t0_dn4 = assign48840_e81857_d_n4;
        locals.var_t0_dn5 = assign48840_e81857_d_n5;
        locals.var_t0_dn6 = assign48840_e81857_d_n6;
        locals.var_t0_dn7 = assign48840_e81857_d_n7;
        locals.var_t0_dn8 = assign48840_e81857_d_n8;
        locals.var_t0_dn9 = assign48840_e81857_d_n9;
        locals.var_t0_dn10 = assign48840_e81857_d_n10;
        locals.var_t0_dn11 = assign48840_e81857_d_n11;

        let (assign48850_e81868, assign48850_e81868_d_n3, assign48850_e81868_d_n4, assign48850_e81868_d_n5, assign48850_e81868_d_n6, assign48850_e81868_d_n7, assign48850_e81868_d_n8, assign48850_e81868_d_n9, assign48850_e81868_d_n10, assign48850_e81868_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard748 == 0.0)) && (locals.var_guard749 != 0.0)) {
        let assign48850_e81866: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign48850_e81866, ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn3), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11),)
    } else {
        (locals.var_t11, locals.var_t11_dn3, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11,)
    }
};
        locals.var_t11 = assign48850_e81868;
        locals.var_t11_dn3 = assign48850_e81868_d_n3;
        locals.var_t11_dn4 = assign48850_e81868_d_n4;
        locals.var_t11_dn5 = assign48850_e81868_d_n5;
        locals.var_t11_dn6 = assign48850_e81868_d_n6;
        locals.var_t11_dn7 = assign48850_e81868_d_n7;
        locals.var_t11_dn8 = assign48850_e81868_d_n8;
        locals.var_t11_dn9 = assign48850_e81868_d_n9;
        locals.var_t11_dn10 = assign48850_e81868_d_n10;
        locals.var_t11_dn11 = assign48850_e81868_d_n11;

        let (assign48860_e81879, assign48860_e81879_d_n3, assign48860_e81879_d_n4, assign48860_e81879_d_n5, assign48860_e81879_d_n6, assign48860_e81879_d_n7, assign48860_e81879_d_n8, assign48860_e81879_d_n9, assign48860_e81879_d_n10, assign48860_e81879_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard748 == 0.0)) && (locals.var_guard749 != 0.0)) {
        let assign48860_e81877: f64 = (-locals.var_t11);
        (assign48860_e81877, (-locals.var_t11_dn3), (-locals.var_t11_dn4), (-locals.var_t11_dn5), (-locals.var_t11_dn6), (-locals.var_t11_dn7), (-locals.var_t11_dn8), (-locals.var_t11_dn9), (-locals.var_t11_dn10), (-locals.var_t11_dn11),)
    } else {
        (locals.var_t11, locals.var_t11_dn3, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11,)
    }
};
        locals.var_t11 = assign48860_e81879;
        locals.var_t11_dn3 = assign48860_e81879_d_n3;
        locals.var_t11_dn4 = assign48860_e81879_d_n4;
        locals.var_t11_dn5 = assign48860_e81879_d_n5;
        locals.var_t11_dn6 = assign48860_e81879_d_n6;
        locals.var_t11_dn7 = assign48860_e81879_d_n7;
        locals.var_t11_dn8 = assign48860_e81879_d_n8;
        locals.var_t11_dn9 = assign48860_e81879_d_n9;
        locals.var_t11_dn10 = assign48860_e81879_d_n10;
        locals.var_t11_dn11 = assign48860_e81879_d_n11;

        let (assign48870_e81894, assign48870_e81894_d_n3, assign48870_e81894_d_n4, assign48870_e81894_d_n5, assign48870_e81894_d_n6, assign48870_e81894_d_n7, assign48870_e81894_d_n8, assign48870_e81894_d_n9, assign48870_e81894_d_n10, assign48870_e81894_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard748 == 0.0)) && (locals.var_guard749 == 0.0)) {
        let assign48870_e81891: f64 = (locals.var_vrec0d_i - locals.var_vbd_jct);
        let assign48870_e81892: f64 = (1.0 / assign48870_e81891);
        (assign48870_e81892, 0.0, 0.0, 0.0, (-((-locals.var_vbd_jct_dn6) / (assign48870_e81891 * assign48870_e81891))), 0.0, 0.0, 0.0, (-((-locals.var_vbd_jct_dn10) / (assign48870_e81891 * assign48870_e81891))), 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign48870_e81894;
        locals.var_t1_dn3 = assign48870_e81894_d_n3;
        locals.var_t1_dn4 = assign48870_e81894_d_n4;
        locals.var_t1_dn5 = assign48870_e81894_d_n5;
        locals.var_t1_dn6 = assign48870_e81894_d_n6;
        locals.var_t1_dn7 = assign48870_e81894_d_n7;
        locals.var_t1_dn8 = assign48870_e81894_d_n8;
        locals.var_t1_dn9 = assign48870_e81894_d_n9;
        locals.var_t1_dn10 = assign48870_e81894_d_n10;
        locals.var_t1_dn11 = assign48870_e81894_d_n11;

        let (assign48880_e81912, assign48880_e81912_d_n3, assign48880_e81912_d_n4, assign48880_e81912_d_n5, assign48880_e81912_d_n6, assign48880_e81912_d_n7, assign48880_e81912_d_n8, assign48880_e81912_d_n9, assign48880_e81912_d_n10, assign48880_e81912_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard748 == 0.0)) && (locals.var_guard749 == 0.0)) {
        let assign48880_e81904: f64 = (-locals.var_vbd_jct);
        let assign48880_e81906: f64 = (assign48880_e81904 / locals.var_nvtmr);
        let assign48880_e81908: f64 = (assign48880_e81906 * locals.var_vrec0d_i);
        let assign48880_e81910: f64 = (assign48880_e81908 * locals.var_t1);
        (assign48880_e81910, (assign48880_e81908 * locals.var_t1_dn3), ((((-((assign48880_e81904 * locals.var_nvtmr_dn4) / (locals.var_nvtmr * locals.var_nvtmr))) * locals.var_vrec0d_i) * locals.var_t1) + (assign48880_e81908 * locals.var_t1_dn4)), ((((-((assign48880_e81904 * locals.var_nvtmr_dn5) / (locals.var_nvtmr * locals.var_nvtmr))) * locals.var_vrec0d_i) * locals.var_t1) + (assign48880_e81908 * locals.var_t1_dn5)), (((((-locals.var_vbd_jct_dn6) / locals.var_nvtmr) * locals.var_vrec0d_i) * locals.var_t1) + (assign48880_e81908 * locals.var_t1_dn6)), (assign48880_e81908 * locals.var_t1_dn7), (assign48880_e81908 * locals.var_t1_dn8), (assign48880_e81908 * locals.var_t1_dn9), (((((-locals.var_vbd_jct_dn10) / locals.var_nvtmr) * locals.var_vrec0d_i) * locals.var_t1) + (assign48880_e81908 * locals.var_t1_dn10)), (assign48880_e81908 * locals.var_t1_dn11),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign48880_e81912;
        locals.var_t0_dn3 = assign48880_e81912_d_n3;
        locals.var_t0_dn4 = assign48880_e81912_d_n4;
        locals.var_t0_dn5 = assign48880_e81912_d_n5;
        locals.var_t0_dn6 = assign48880_e81912_d_n6;
        locals.var_t0_dn7 = assign48880_e81912_d_n7;
        locals.var_t0_dn8 = assign48880_e81912_d_n8;
        locals.var_t0_dn9 = assign48880_e81912_d_n9;
        locals.var_t0_dn10 = assign48880_e81912_d_n10;
        locals.var_t0_dn11 = assign48880_e81912_d_n11;

        let (assign48890_e81924, assign48890_e81924_d_n3, assign48890_e81924_d_n4, assign48890_e81924_d_n5, assign48890_e81924_d_n6, assign48890_e81924_d_n7, assign48890_e81924_d_n8, assign48890_e81924_d_n9, assign48890_e81924_d_n10, assign48890_e81924_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard748 == 0.0)) && (locals.var_guard749 == 0.0)) {
        let assign48890_e81922: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign48890_e81922, ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn3), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11),)
    } else {
        (locals.var_t11, locals.var_t11_dn3, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11,)
    }
};
        locals.var_t11 = assign48890_e81924;
        locals.var_t11_dn3 = assign48890_e81924_d_n3;
        locals.var_t11_dn4 = assign48890_e81924_d_n4;
        locals.var_t11_dn5 = assign48890_e81924_d_n5;
        locals.var_t11_dn6 = assign48890_e81924_d_n6;
        locals.var_t11_dn7 = assign48890_e81924_d_n7;
        locals.var_t11_dn8 = assign48890_e81924_d_n8;
        locals.var_t11_dn9 = assign48890_e81924_d_n9;
        locals.var_t11_dn10 = assign48890_e81924_d_n10;
        locals.var_t11_dn11 = assign48890_e81924_d_n11;

        let (assign48900_e81936, assign48900_e81936_d_n3, assign48900_e81936_d_n4, assign48900_e81936_d_n5, assign48900_e81936_d_n6, assign48900_e81936_d_n7, assign48900_e81936_d_n8, assign48900_e81936_d_n9, assign48900_e81936_d_n10, assign48900_e81936_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard748 == 0.0)) && (locals.var_guard749 == 0.0)) {
        let assign48900_e81934: f64 = (-locals.var_t11);
        (assign48900_e81934, (-locals.var_t11_dn3), (-locals.var_t11_dn4), (-locals.var_t11_dn5), (-locals.var_t11_dn6), (-locals.var_t11_dn7), (-locals.var_t11_dn8), (-locals.var_t11_dn9), (-locals.var_t11_dn10), (-locals.var_t11_dn11),)
    } else {
        (locals.var_t11, locals.var_t11_dn3, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11,)
    }
};
        locals.var_t11 = assign48900_e81936;
        locals.var_t11_dn3 = assign48900_e81936_d_n3;
        locals.var_t11_dn4 = assign48900_e81936_d_n4;
        locals.var_t11_dn5 = assign48900_e81936_d_n5;
        locals.var_t11_dn6 = assign48900_e81936_d_n6;
        locals.var_t11_dn7 = assign48900_e81936_d_n7;
        locals.var_t11_dn8 = assign48900_e81936_d_n8;
        locals.var_t11_dn9 = assign48900_e81936_d_n9;
        locals.var_t11_dn10 = assign48900_e81936_d_n10;
        locals.var_t11_dn11 = assign48900_e81936_d_n11;

        let (assign48910_e81946, assign48910_e81946_d_n3, assign48910_e81946_d_n4, assign48910_e81946_d_n5, assign48910_e81946_d_n6, assign48910_e81946_d_n7, assign48910_e81946_d_n8, assign48910_e81946_d_n9, assign48910_e81946_d_n10, assign48910_e81946_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard748 == 0.0)) {
        let assign48910_e81944: f64 = (locals.var_wdtsi * locals.var_jrecd);
        (assign48910_e81944, (locals.var_wdtsi * locals.var_jrecd_dn3), (locals.var_wdtsi * locals.var_jrecd_dn4), (locals.var_wdtsi * locals.var_jrecd_dn5), (locals.var_wdtsi * locals.var_jrecd_dn6), (locals.var_wdtsi * locals.var_jrecd_dn7), (locals.var_wdtsi * locals.var_jrecd_dn8), (locals.var_wdtsi * locals.var_jrecd_dn9), (locals.var_wdtsi * locals.var_jrecd_dn10), (locals.var_wdtsi * locals.var_jrecd_dn11),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign48910_e81946;
        locals.var_t3_dn3 = assign48910_e81946_d_n3;
        locals.var_t3_dn4 = assign48910_e81946_d_n4;
        locals.var_t3_dn5 = assign48910_e81946_d_n5;
        locals.var_t3_dn6 = assign48910_e81946_d_n6;
        locals.var_t3_dn7 = assign48910_e81946_d_n7;
        locals.var_t3_dn8 = assign48910_e81946_d_n8;
        locals.var_t3_dn9 = assign48910_e81946_d_n9;
        locals.var_t3_dn10 = assign48910_e81946_d_n10;
        locals.var_t3_dn11 = assign48910_e81946_d_n11;

        let (assign48920_e81958, assign48920_e81958_d_n3, assign48920_e81958_d_n4, assign48920_e81958_d_n5, assign48920_e81958_d_n6, assign48920_e81958_d_n7, assign48920_e81958_d_n8, assign48920_e81958_d_n9, assign48920_e81958_d_n10, assign48920_e81958_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard748 == 0.0)) {
        let assign48920_e81955: f64 = (locals.var_t10 + locals.var_t11);
        let assign48920_e81956: f64 = (locals.var_t3 * assign48920_e81955);
        (assign48920_e81956, ((locals.var_t3_dn3 * assign48920_e81955) + (locals.var_t3 * (locals.var_t10_dn3 + locals.var_t11_dn3))), ((locals.var_t3_dn4 * assign48920_e81955) + (locals.var_t3 * (locals.var_t10_dn4 + locals.var_t11_dn4))), ((locals.var_t3_dn5 * assign48920_e81955) + (locals.var_t3 * (locals.var_t10_dn5 + locals.var_t11_dn5))), ((locals.var_t3_dn6 * assign48920_e81955) + (locals.var_t3 * (locals.var_t10_dn6 + locals.var_t11_dn6))), ((locals.var_t3_dn7 * assign48920_e81955) + (locals.var_t3 * (locals.var_t10_dn7 + locals.var_t11_dn7))), ((locals.var_t3_dn8 * assign48920_e81955) + (locals.var_t3 * (locals.var_t10_dn8 + locals.var_t11_dn8))), ((locals.var_t3_dn9 * assign48920_e81955) + (locals.var_t3 * (locals.var_t10_dn9 + locals.var_t11_dn9))), ((locals.var_t3_dn10 * assign48920_e81955) + (locals.var_t3 * (locals.var_t10_dn10 + locals.var_t11_dn10))), ((locals.var_t3_dn11 * assign48920_e81955) + (locals.var_t3 * (locals.var_t10_dn11 + locals.var_t11_dn11))),)
    } else {
        (locals.var_ibd2, locals.var_ibd2_dn3, locals.var_ibd2_dn4, locals.var_ibd2_dn5, locals.var_ibd2_dn6, locals.var_ibd2_dn7, locals.var_ibd2_dn8, locals.var_ibd2_dn9, locals.var_ibd2_dn10, locals.var_ibd2_dn11,)
    }
};
        locals.var_ibd2 = assign48920_e81958;
        locals.var_ibd2_dn3 = assign48920_e81958_d_n3;
        locals.var_ibd2_dn4 = assign48920_e81958_d_n4;
        locals.var_ibd2_dn5 = assign48920_e81958_d_n5;
        locals.var_ibd2_dn6 = assign48920_e81958_d_n6;
        locals.var_ibd2_dn7 = assign48920_e81958_d_n7;
        locals.var_ibd2_dn8 = assign48920_e81958_d_n8;
        locals.var_ibd2_dn9 = assign48920_e81958_d_n9;
        locals.var_ibd2_dn10 = assign48920_e81958_d_n10;
        locals.var_ibd2_dn11 = assign48920_e81958_d_n11;

        let (assign48930_e81967,) = {
    if (locals.var_guard492 == 0.0) {
        let assign48930_e81963: f64 = (locals.var_weff / p.p1373);
        let assign48930_e81965: f64 = (assign48930_e81963 * p.p74);
        (assign48930_e81965,)
    } else {
        (locals.var_wtsi,)
    }
};
        locals.var_wtsi = assign48930_e81967;

        let assign48940_e81974: f64 = if ((locals.var_isbjt_i == 0.0) && (locals.var_idbjt_i == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard750 = assign48940_e81974;

        let (assign48950_e81981, assign48950_e81981_d_n3, assign48950_e81981_d_n4, assign48950_e81981_d_n5, assign48950_e81981_d_n6, assign48950_e81981_d_n7, assign48950_e81981_d_n8, assign48950_e81981_d_n9, assign48950_e81981_d_n10, assign48950_e81981_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard750 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ibs3, locals.var_ibs3_dn3, locals.var_ibs3_dn4, locals.var_ibs3_dn5, locals.var_ibs3_dn6, locals.var_ibs3_dn7, locals.var_ibs3_dn8, locals.var_ibs3_dn9, locals.var_ibs3_dn10, locals.var_ibs3_dn11,)
    }
};
        locals.var_ibs3 = assign48950_e81981;
        locals.var_ibs3_dn3 = assign48950_e81981_d_n3;
        locals.var_ibs3_dn4 = assign48950_e81981_d_n4;
        locals.var_ibs3_dn5 = assign48950_e81981_d_n5;
        locals.var_ibs3_dn6 = assign48950_e81981_d_n6;
        locals.var_ibs3_dn7 = assign48950_e81981_d_n7;
        locals.var_ibs3_dn8 = assign48950_e81981_d_n8;
        locals.var_ibs3_dn9 = assign48950_e81981_d_n9;
        locals.var_ibs3_dn10 = assign48950_e81981_d_n10;
        locals.var_ibs3_dn11 = assign48950_e81981_d_n11;

        let (assign48960_e81988, assign48960_e81988_d_n3, assign48960_e81988_d_n4, assign48960_e81988_d_n5, assign48960_e81988_d_n6, assign48960_e81988_d_n7, assign48960_e81988_d_n8, assign48960_e81988_d_n9, assign48960_e81988_d_n10, assign48960_e81988_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard750 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ibd3, locals.var_ibd3_dn3, locals.var_ibd3_dn4, locals.var_ibd3_dn5, locals.var_ibd3_dn6, locals.var_ibd3_dn7, locals.var_ibd3_dn8, locals.var_ibd3_dn9, locals.var_ibd3_dn10, locals.var_ibd3_dn11,)
    }
};
        locals.var_ibd3 = assign48960_e81988;
        locals.var_ibd3_dn3 = assign48960_e81988_d_n3;
        locals.var_ibd3_dn4 = assign48960_e81988_d_n4;
        locals.var_ibd3_dn5 = assign48960_e81988_d_n5;
        locals.var_ibd3_dn6 = assign48960_e81988_d_n6;
        locals.var_ibd3_dn7 = assign48960_e81988_d_n7;
        locals.var_ibd3_dn8 = assign48960_e81988_d_n8;
        locals.var_ibd3_dn9 = assign48960_e81988_d_n9;
        locals.var_ibd3_dn10 = assign48960_e81988_d_n10;
        locals.var_ibd3_dn11 = assign48960_e81988_d_n11;

        let (assign48970_e81995, assign48970_e81995_d_n3, assign48970_e81995_d_n4, assign48970_e81995_d_n5, assign48970_e81995_d_n6, assign48970_e81995_d_n7, assign48970_e81995_d_n8, assign48970_e81995_d_n9, assign48970_e81995_d_n10, assign48970_e81995_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard750 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ic, locals.var_ic_dn3, locals.var_ic_dn4, locals.var_ic_dn5, locals.var_ic_dn6, locals.var_ic_dn7, locals.var_ic_dn8, locals.var_ic_dn9, locals.var_ic_dn10, locals.var_ic_dn11,)
    }
};
        locals.var_ic = assign48970_e81995;
        locals.var_ic_dn3 = assign48970_e81995_d_n3;
        locals.var_ic_dn4 = assign48970_e81995_d_n4;
        locals.var_ic_dn5 = assign48970_e81995_d_n5;
        locals.var_ic_dn6 = assign48970_e81995_d_n6;
        locals.var_ic_dn7 = assign48970_e81995_d_n7;
        locals.var_ic_dn8 = assign48970_e81995_d_n8;
        locals.var_ic_dn9 = assign48970_e81995_d_n9;
        locals.var_ic_dn10 = assign48970_e81995_d_n10;
        locals.var_ic_dn11 = assign48970_e81995_d_n11;

        let (assign48980_e82007, assign48980_e82007_d_n3, assign48980_e82007_d_n4, assign48980_e82007_d_n5, assign48980_e82007_d_n6, assign48980_e82007_d_n7, assign48980_e82007_d_n8, assign48980_e82007_d_n9, assign48980_e82007_d_n10, assign48980_e82007_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) {
        let assign48980_e82003: f64 = (locals.var_xbjt_i * locals.var_t4);
        let assign48980_e82005: f64 = (assign48980_e82003 / locals.var_ndiode_i);
        (assign48980_e82005, ((locals.var_xbjt_i * locals.var_t4_dn3) / locals.var_ndiode_i), ((locals.var_xbjt_i * locals.var_t4_dn4) / locals.var_ndiode_i), ((locals.var_xbjt_i * locals.var_t4_dn5) / locals.var_ndiode_i), ((locals.var_xbjt_i * locals.var_t4_dn6) / locals.var_ndiode_i), ((locals.var_xbjt_i * locals.var_t4_dn7) / locals.var_ndiode_i), ((locals.var_xbjt_i * locals.var_t4_dn8) / locals.var_ndiode_i), ((locals.var_xbjt_i * locals.var_t4_dn9) / locals.var_ndiode_i), ((locals.var_xbjt_i * locals.var_t4_dn10) / locals.var_ndiode_i), ((locals.var_xbjt_i * locals.var_t4_dn11) / locals.var_ndiode_i),)
    } else {
        (locals.var_t7, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11,)
    }
};
        locals.var_t7 = assign48980_e82007;
        locals.var_t7_dn3 = assign48980_e82007_d_n3;
        locals.var_t7_dn4 = assign48980_e82007_d_n4;
        locals.var_t7_dn5 = assign48980_e82007_d_n5;
        locals.var_t7_dn6 = assign48980_e82007_d_n6;
        locals.var_t7_dn7 = assign48980_e82007_d_n7;
        locals.var_t7_dn8 = assign48980_e82007_d_n8;
        locals.var_t7_dn9 = assign48980_e82007_d_n9;
        locals.var_t7_dn10 = assign48980_e82007_d_n10;
        locals.var_t7_dn11 = assign48980_e82007_d_n11;

        let (assign48990_e82016, assign48990_e82016_d_n3, assign48990_e82016_d_n4, assign48990_e82016_d_n5, assign48990_e82016_d_n6, assign48990_e82016_d_n7, assign48990_e82016_d_n8, assign48990_e82016_d_n9, assign48990_e82016_d_n10, assign48990_e82016_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) {
        let assign48990_e82014: f64 = { let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign48990_e82014, ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn3), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn4), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn5), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn6), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn7), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn8), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn9), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn10), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn11),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign48990_e82016;
        locals.var_t0_dn3 = assign48990_e82016_d_n3;
        locals.var_t0_dn4 = assign48990_e82016_d_n4;
        locals.var_t0_dn5 = assign48990_e82016_d_n5;
        locals.var_t0_dn6 = assign48990_e82016_d_n6;
        locals.var_t0_dn7 = assign48990_e82016_d_n7;
        locals.var_t0_dn8 = assign48990_e82016_d_n8;
        locals.var_t0_dn9 = assign48990_e82016_d_n9;
        locals.var_t0_dn10 = assign48990_e82016_d_n10;
        locals.var_t0_dn11 = assign48990_e82016_d_n11;

        let (assign49000_e82026, assign49000_e82026_d_n3, assign49000_e82026_d_n4, assign49000_e82026_d_n5, assign49000_e82026_d_n6, assign49000_e82026_d_n7, assign49000_e82026_d_n8, assign49000_e82026_d_n9, assign49000_e82026_d_n10, assign49000_e82026_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) {
        let assign49000_e82024: f64 = (locals.var_ahli_i * locals.var_t0);
        (assign49000_e82024, (locals.var_ahli_i * locals.var_t0_dn3), (locals.var_ahli_i * locals.var_t0_dn4), (locals.var_ahli_i * locals.var_t0_dn5), (locals.var_ahli_i * locals.var_t0_dn6), (locals.var_ahli_i * locals.var_t0_dn7), (locals.var_ahli_i * locals.var_t0_dn8), (locals.var_ahli_i * locals.var_t0_dn9), (locals.var_ahli_i * locals.var_t0_dn10), (locals.var_ahli_i * locals.var_t0_dn11),)
    } else {
        (locals.var_ahlis, locals.var_ahlis_dn3, locals.var_ahlis_dn4, locals.var_ahlis_dn5, locals.var_ahlis_dn6, locals.var_ahlis_dn7, locals.var_ahlis_dn8, locals.var_ahlis_dn9, locals.var_ahlis_dn10, locals.var_ahlis_dn11,)
    }
};
        locals.var_ahlis = assign49000_e82026;
        locals.var_ahlis_dn3 = assign49000_e82026_d_n3;
        locals.var_ahlis_dn4 = assign49000_e82026_d_n4;
        locals.var_ahlis_dn5 = assign49000_e82026_d_n5;
        locals.var_ahlis_dn6 = assign49000_e82026_d_n6;
        locals.var_ahlis_dn7 = assign49000_e82026_d_n7;
        locals.var_ahlis_dn8 = assign49000_e82026_d_n8;
        locals.var_ahlis_dn9 = assign49000_e82026_d_n9;
        locals.var_ahlis_dn10 = assign49000_e82026_d_n10;
        locals.var_ahlis_dn11 = assign49000_e82026_d_n11;

        let (assign49010_e82036, assign49010_e82036_d_n3, assign49010_e82036_d_n4, assign49010_e82036_d_n5, assign49010_e82036_d_n6, assign49010_e82036_d_n7, assign49010_e82036_d_n8, assign49010_e82036_d_n9, assign49010_e82036_d_n10, assign49010_e82036_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) {
        let assign49010_e82034: f64 = (locals.var_isbjt_i * locals.var_t0);
        (assign49010_e82034, (locals.var_isbjt_i * locals.var_t0_dn3), (locals.var_isbjt_i * locals.var_t0_dn4), (locals.var_isbjt_i * locals.var_t0_dn5), (locals.var_isbjt_i * locals.var_t0_dn6), (locals.var_isbjt_i * locals.var_t0_dn7), (locals.var_isbjt_i * locals.var_t0_dn8), (locals.var_isbjt_i * locals.var_t0_dn9), (locals.var_isbjt_i * locals.var_t0_dn10), (locals.var_isbjt_i * locals.var_t0_dn11),)
    } else {
        (locals.var_jbjts, locals.var_jbjts_dn3, locals.var_jbjts_dn4, locals.var_jbjts_dn5, locals.var_jbjts_dn6, locals.var_jbjts_dn7, locals.var_jbjts_dn8, locals.var_jbjts_dn9, locals.var_jbjts_dn10, locals.var_jbjts_dn11,)
    }
};
        locals.var_jbjts = assign49010_e82036;
        locals.var_jbjts_dn3 = assign49010_e82036_d_n3;
        locals.var_jbjts_dn4 = assign49010_e82036_d_n4;
        locals.var_jbjts_dn5 = assign49010_e82036_d_n5;
        locals.var_jbjts_dn6 = assign49010_e82036_d_n6;
        locals.var_jbjts_dn7 = assign49010_e82036_d_n7;
        locals.var_jbjts_dn8 = assign49010_e82036_d_n8;
        locals.var_jbjts_dn9 = assign49010_e82036_d_n9;
        locals.var_jbjts_dn10 = assign49010_e82036_d_n10;
        locals.var_jbjts_dn11 = assign49010_e82036_d_n11;

        let (assign49020_e82048, assign49020_e82048_d_n3, assign49020_e82048_d_n4, assign49020_e82048_d_n5, assign49020_e82048_d_n6, assign49020_e82048_d_n7, assign49020_e82048_d_n8, assign49020_e82048_d_n9, assign49020_e82048_d_n10, assign49020_e82048_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) {
        let assign49020_e82044: f64 = (locals.var_xbjt_i * locals.var_t4);
        let assign49020_e82046: f64 = (assign49020_e82044 / locals.var_ndiode_i);
        (assign49020_e82046, ((locals.var_xbjt_i * locals.var_t4_dn3) / locals.var_ndiode_i), ((locals.var_xbjt_i * locals.var_t4_dn4) / locals.var_ndiode_i), ((locals.var_xbjt_i * locals.var_t4_dn5) / locals.var_ndiode_i), ((locals.var_xbjt_i * locals.var_t4_dn6) / locals.var_ndiode_i), ((locals.var_xbjt_i * locals.var_t4_dn7) / locals.var_ndiode_i), ((locals.var_xbjt_i * locals.var_t4_dn8) / locals.var_ndiode_i), ((locals.var_xbjt_i * locals.var_t4_dn9) / locals.var_ndiode_i), ((locals.var_xbjt_i * locals.var_t4_dn10) / locals.var_ndiode_i), ((locals.var_xbjt_i * locals.var_t4_dn11) / locals.var_ndiode_i),)
    } else {
        (locals.var_t7, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11,)
    }
};
        locals.var_t7 = assign49020_e82048;
        locals.var_t7_dn3 = assign49020_e82048_d_n3;
        locals.var_t7_dn4 = assign49020_e82048_d_n4;
        locals.var_t7_dn5 = assign49020_e82048_d_n5;
        locals.var_t7_dn6 = assign49020_e82048_d_n6;
        locals.var_t7_dn7 = assign49020_e82048_d_n7;
        locals.var_t7_dn8 = assign49020_e82048_d_n8;
        locals.var_t7_dn9 = assign49020_e82048_d_n9;
        locals.var_t7_dn10 = assign49020_e82048_d_n10;
        locals.var_t7_dn11 = assign49020_e82048_d_n11;

    }

    pub(super) fn stamp_transient_block_166(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign49030_e82057, assign49030_e82057_d_n3, assign49030_e82057_d_n4, assign49030_e82057_d_n5, assign49030_e82057_d_n6, assign49030_e82057_d_n7, assign49030_e82057_d_n8, assign49030_e82057_d_n9, assign49030_e82057_d_n10, assign49030_e82057_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) {
        let assign49030_e82055: f64 = { let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign49030_e82055, ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn3), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn4), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn5), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn6), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn7), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn8), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn9), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn10), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn11),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign49030_e82057;
        locals.var_t0_dn3 = assign49030_e82057_d_n3;
        locals.var_t0_dn4 = assign49030_e82057_d_n4;
        locals.var_t0_dn5 = assign49030_e82057_d_n5;
        locals.var_t0_dn6 = assign49030_e82057_d_n6;
        locals.var_t0_dn7 = assign49030_e82057_d_n7;
        locals.var_t0_dn8 = assign49030_e82057_d_n8;
        locals.var_t0_dn9 = assign49030_e82057_d_n9;
        locals.var_t0_dn10 = assign49030_e82057_d_n10;
        locals.var_t0_dn11 = assign49030_e82057_d_n11;

        let (assign49040_e82067, assign49040_e82067_d_n3, assign49040_e82067_d_n4, assign49040_e82067_d_n5, assign49040_e82067_d_n6, assign49040_e82067_d_n7, assign49040_e82067_d_n8, assign49040_e82067_d_n9, assign49040_e82067_d_n10, assign49040_e82067_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) {
        let assign49040_e82065: f64 = (locals.var_ahlid_i * locals.var_t0);
        (assign49040_e82065, (locals.var_ahlid_i * locals.var_t0_dn3), (locals.var_ahlid_i * locals.var_t0_dn4), (locals.var_ahlid_i * locals.var_t0_dn5), (locals.var_ahlid_i * locals.var_t0_dn6), (locals.var_ahlid_i * locals.var_t0_dn7), (locals.var_ahlid_i * locals.var_t0_dn8), (locals.var_ahlid_i * locals.var_t0_dn9), (locals.var_ahlid_i * locals.var_t0_dn10), (locals.var_ahlid_i * locals.var_t0_dn11),)
    } else {
        (locals.var_ahlid, locals.var_ahlid_dn3, locals.var_ahlid_dn4, locals.var_ahlid_dn5, locals.var_ahlid_dn6, locals.var_ahlid_dn7, locals.var_ahlid_dn8, locals.var_ahlid_dn9, locals.var_ahlid_dn10, locals.var_ahlid_dn11,)
    }
};
        locals.var_ahlid = assign49040_e82067;
        locals.var_ahlid_dn3 = assign49040_e82067_d_n3;
        locals.var_ahlid_dn4 = assign49040_e82067_d_n4;
        locals.var_ahlid_dn5 = assign49040_e82067_d_n5;
        locals.var_ahlid_dn6 = assign49040_e82067_d_n6;
        locals.var_ahlid_dn7 = assign49040_e82067_d_n7;
        locals.var_ahlid_dn8 = assign49040_e82067_d_n8;
        locals.var_ahlid_dn9 = assign49040_e82067_d_n9;
        locals.var_ahlid_dn10 = assign49040_e82067_d_n10;
        locals.var_ahlid_dn11 = assign49040_e82067_d_n11;

        let (assign49050_e82077, assign49050_e82077_d_n3, assign49050_e82077_d_n4, assign49050_e82077_d_n5, assign49050_e82077_d_n6, assign49050_e82077_d_n7, assign49050_e82077_d_n8, assign49050_e82077_d_n9, assign49050_e82077_d_n10, assign49050_e82077_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) {
        let assign49050_e82075: f64 = (locals.var_idbjt_i * locals.var_t0);
        (assign49050_e82075, (locals.var_idbjt_i * locals.var_t0_dn3), (locals.var_idbjt_i * locals.var_t0_dn4), (locals.var_idbjt_i * locals.var_t0_dn5), (locals.var_idbjt_i * locals.var_t0_dn6), (locals.var_idbjt_i * locals.var_t0_dn7), (locals.var_idbjt_i * locals.var_t0_dn8), (locals.var_idbjt_i * locals.var_t0_dn9), (locals.var_idbjt_i * locals.var_t0_dn10), (locals.var_idbjt_i * locals.var_t0_dn11),)
    } else {
        (locals.var_jbjtd, locals.var_jbjtd_dn3, locals.var_jbjtd_dn4, locals.var_jbjtd_dn5, locals.var_jbjtd_dn6, locals.var_jbjtd_dn7, locals.var_jbjtd_dn8, locals.var_jbjtd_dn9, locals.var_jbjtd_dn10, locals.var_jbjtd_dn11,)
    }
};
        locals.var_jbjtd = assign49050_e82077;
        locals.var_jbjtd_dn3 = assign49050_e82077_d_n3;
        locals.var_jbjtd_dn4 = assign49050_e82077_d_n4;
        locals.var_jbjtd_dn5 = assign49050_e82077_d_n5;
        locals.var_jbjtd_dn6 = assign49050_e82077_d_n6;
        locals.var_jbjtd_dn7 = assign49050_e82077_d_n7;
        locals.var_jbjtd_dn8 = assign49050_e82077_d_n8;
        locals.var_jbjtd_dn9 = assign49050_e82077_d_n9;
        locals.var_jbjtd_dn10 = assign49050_e82077_d_n10;
        locals.var_jbjtd_dn11 = assign49050_e82077_d_n11;

        let (assign49060_e82089, assign49060_e82089_d_n3, assign49060_e82089_d_n4, assign49060_e82089_d_n5, assign49060_e82089_d_n6, assign49060_e82089_d_n7, assign49060_e82089_d_n8, assign49060_e82089_d_n9, assign49060_e82089_d_n10, assign49060_e82089_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) {
        let assign49060_e82086: f64 = (locals.var_expvbsnvtm - 1.0);
        let assign49060_e82087: f64 = (locals.var_ahlis * assign49060_e82086);
        (assign49060_e82087, ((locals.var_ahlis_dn3 * assign49060_e82086) + (locals.var_ahlis * locals.var_expvbsnvtm_dn3)), ((locals.var_ahlis_dn4 * assign49060_e82086) + (locals.var_ahlis * locals.var_expvbsnvtm_dn4)), ((locals.var_ahlis_dn5 * assign49060_e82086) + (locals.var_ahlis * locals.var_expvbsnvtm_dn5)), ((locals.var_ahlis_dn6 * assign49060_e82086) + (locals.var_ahlis * locals.var_expvbsnvtm_dn6)), ((locals.var_ahlis_dn7 * assign49060_e82086) + (locals.var_ahlis * locals.var_expvbsnvtm_dn7)), ((locals.var_ahlis_dn8 * assign49060_e82086) + (locals.var_ahlis * locals.var_expvbsnvtm_dn8)), ((locals.var_ahlis_dn9 * assign49060_e82086) + (locals.var_ahlis * locals.var_expvbsnvtm_dn9)), ((locals.var_ahlis_dn10 * assign49060_e82086) + (locals.var_ahlis * locals.var_expvbsnvtm_dn10)), ((locals.var_ahlis_dn11 * assign49060_e82086) + (locals.var_ahlis * locals.var_expvbsnvtm_dn11)),)
    } else {
        (locals.var_ehlis, locals.var_ehlis_dn3, locals.var_ehlis_dn4, locals.var_ehlis_dn5, locals.var_ehlis_dn6, locals.var_ehlis_dn7, locals.var_ehlis_dn8, locals.var_ehlis_dn9, locals.var_ehlis_dn10, locals.var_ehlis_dn11,)
    }
};
        locals.var_ehlis = assign49060_e82089;
        locals.var_ehlis_dn3 = assign49060_e82089_d_n3;
        locals.var_ehlis_dn4 = assign49060_e82089_d_n4;
        locals.var_ehlis_dn5 = assign49060_e82089_d_n5;
        locals.var_ehlis_dn6 = assign49060_e82089_d_n6;
        locals.var_ehlis_dn7 = assign49060_e82089_d_n7;
        locals.var_ehlis_dn8 = assign49060_e82089_d_n8;
        locals.var_ehlis_dn9 = assign49060_e82089_d_n9;
        locals.var_ehlis_dn10 = assign49060_e82089_d_n10;
        locals.var_ehlis_dn11 = assign49060_e82089_d_n11;

        let assign49070_e82092: f64 = if locals.var_ehlis < 1e-5 { 1.0 } else { 0.0 };
        locals.var_guard751 = assign49070_e82092;

        let (assign49080_e82102, assign49080_e82102_d_n3, assign49080_e82102_d_n4, assign49080_e82102_d_n5, assign49080_e82102_d_n6, assign49080_e82102_d_n7, assign49080_e82102_d_n8, assign49080_e82102_d_n9, assign49080_e82102_d_n10, assign49080_e82102_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) && (locals.var_guard751 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ehlis, locals.var_ehlis_dn3, locals.var_ehlis_dn4, locals.var_ehlis_dn5, locals.var_ehlis_dn6, locals.var_ehlis_dn7, locals.var_ehlis_dn8, locals.var_ehlis_dn9, locals.var_ehlis_dn10, locals.var_ehlis_dn11,)
    }
};
        locals.var_ehlis = assign49080_e82102;
        locals.var_ehlis_dn3 = assign49080_e82102_d_n3;
        locals.var_ehlis_dn4 = assign49080_e82102_d_n4;
        locals.var_ehlis_dn5 = assign49080_e82102_d_n5;
        locals.var_ehlis_dn6 = assign49080_e82102_d_n6;
        locals.var_ehlis_dn7 = assign49080_e82102_d_n7;
        locals.var_ehlis_dn8 = assign49080_e82102_d_n8;
        locals.var_ehlis_dn9 = assign49080_e82102_d_n9;
        locals.var_ehlis_dn10 = assign49080_e82102_d_n10;
        locals.var_ehlis_dn11 = assign49080_e82102_d_n11;

        let (assign49090_e82112, assign49090_e82112_d_n3, assign49090_e82112_d_n4, assign49090_e82112_d_n5, assign49090_e82112_d_n6, assign49090_e82112_d_n7, assign49090_e82112_d_n8, assign49090_e82112_d_n9, assign49090_e82112_d_n10, assign49090_e82112_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) && (locals.var_guard751 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ehlisfactor, locals.var_ehlisfactor_dn3, locals.var_ehlisfactor_dn4, locals.var_ehlisfactor_dn5, locals.var_ehlisfactor_dn6, locals.var_ehlisfactor_dn7, locals.var_ehlisfactor_dn8, locals.var_ehlisfactor_dn9, locals.var_ehlisfactor_dn10, locals.var_ehlisfactor_dn11,)
    }
};
        locals.var_ehlisfactor = assign49090_e82112;
        locals.var_ehlisfactor_dn3 = assign49090_e82112_d_n3;
        locals.var_ehlisfactor_dn4 = assign49090_e82112_d_n4;
        locals.var_ehlisfactor_dn5 = assign49090_e82112_d_n5;
        locals.var_ehlisfactor_dn6 = assign49090_e82112_d_n6;
        locals.var_ehlisfactor_dn7 = assign49090_e82112_d_n7;
        locals.var_ehlisfactor_dn8 = assign49090_e82112_d_n8;
        locals.var_ehlisfactor_dn9 = assign49090_e82112_d_n9;
        locals.var_ehlisfactor_dn10 = assign49090_e82112_d_n10;
        locals.var_ehlisfactor_dn11 = assign49090_e82112_d_n11;

        let (assign49100_e82128, assign49100_e82128_d_n3, assign49100_e82128_d_n4, assign49100_e82128_d_n5, assign49100_e82128_d_n6, assign49100_e82128_d_n7, assign49100_e82128_d_n8, assign49100_e82128_d_n9, assign49100_e82128_d_n10, assign49100_e82128_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) && (locals.var_guard751 == 0.0)) {
        let assign49100_e82124: f64 = (1.0 + locals.var_ehlis);
        let assign49100_e82125: f64 = (assign49100_e82124).sqrt();
        let assign49100_e82126: f64 = (1.0 / assign49100_e82125);
        (assign49100_e82126, (-((locals.var_ehlis_dn3 / (2.0 * assign49100_e82125)) / (assign49100_e82125 * assign49100_e82125))), (-((locals.var_ehlis_dn4 / (2.0 * assign49100_e82125)) / (assign49100_e82125 * assign49100_e82125))), (-((locals.var_ehlis_dn5 / (2.0 * assign49100_e82125)) / (assign49100_e82125 * assign49100_e82125))), (-((locals.var_ehlis_dn6 / (2.0 * assign49100_e82125)) / (assign49100_e82125 * assign49100_e82125))), (-((locals.var_ehlis_dn7 / (2.0 * assign49100_e82125)) / (assign49100_e82125 * assign49100_e82125))), (-((locals.var_ehlis_dn8 / (2.0 * assign49100_e82125)) / (assign49100_e82125 * assign49100_e82125))), (-((locals.var_ehlis_dn9 / (2.0 * assign49100_e82125)) / (assign49100_e82125 * assign49100_e82125))), (-((locals.var_ehlis_dn10 / (2.0 * assign49100_e82125)) / (assign49100_e82125 * assign49100_e82125))), (-((locals.var_ehlis_dn11 / (2.0 * assign49100_e82125)) / (assign49100_e82125 * assign49100_e82125))),)
    } else {
        (locals.var_ehlisfactor, locals.var_ehlisfactor_dn3, locals.var_ehlisfactor_dn4, locals.var_ehlisfactor_dn5, locals.var_ehlisfactor_dn6, locals.var_ehlisfactor_dn7, locals.var_ehlisfactor_dn8, locals.var_ehlisfactor_dn9, locals.var_ehlisfactor_dn10, locals.var_ehlisfactor_dn11,)
    }
};
        locals.var_ehlisfactor = assign49100_e82128;
        locals.var_ehlisfactor_dn3 = assign49100_e82128_d_n3;
        locals.var_ehlisfactor_dn4 = assign49100_e82128_d_n4;
        locals.var_ehlisfactor_dn5 = assign49100_e82128_d_n5;
        locals.var_ehlisfactor_dn6 = assign49100_e82128_d_n6;
        locals.var_ehlisfactor_dn7 = assign49100_e82128_d_n7;
        locals.var_ehlisfactor_dn8 = assign49100_e82128_d_n8;
        locals.var_ehlisfactor_dn9 = assign49100_e82128_d_n9;
        locals.var_ehlisfactor_dn10 = assign49100_e82128_d_n10;
        locals.var_ehlisfactor_dn11 = assign49100_e82128_d_n11;

        let (assign49110_e82140, assign49110_e82140_d_n3, assign49110_e82140_d_n4, assign49110_e82140_d_n5, assign49110_e82140_d_n6, assign49110_e82140_d_n7, assign49110_e82140_d_n8, assign49110_e82140_d_n9, assign49110_e82140_d_n10, assign49110_e82140_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) {
        let assign49110_e82137: f64 = (locals.var_expvbdnvtm - 1.0);
        let assign49110_e82138: f64 = (locals.var_ahlid * assign49110_e82137);
        (assign49110_e82138, ((locals.var_ahlid_dn3 * assign49110_e82137) + (locals.var_ahlid * locals.var_expvbdnvtm_dn3)), ((locals.var_ahlid_dn4 * assign49110_e82137) + (locals.var_ahlid * locals.var_expvbdnvtm_dn4)), ((locals.var_ahlid_dn5 * assign49110_e82137) + (locals.var_ahlid * locals.var_expvbdnvtm_dn5)), ((locals.var_ahlid_dn6 * assign49110_e82137) + (locals.var_ahlid * locals.var_expvbdnvtm_dn6)), ((locals.var_ahlid_dn7 * assign49110_e82137) + (locals.var_ahlid * locals.var_expvbdnvtm_dn7)), ((locals.var_ahlid_dn8 * assign49110_e82137) + (locals.var_ahlid * locals.var_expvbdnvtm_dn8)), ((locals.var_ahlid_dn9 * assign49110_e82137) + (locals.var_ahlid * locals.var_expvbdnvtm_dn9)), ((locals.var_ahlid_dn10 * assign49110_e82137) + (locals.var_ahlid * locals.var_expvbdnvtm_dn10)), ((locals.var_ahlid_dn11 * assign49110_e82137) + (locals.var_ahlid * locals.var_expvbdnvtm_dn11)),)
    } else {
        (locals.var_ehlid, locals.var_ehlid_dn3, locals.var_ehlid_dn4, locals.var_ehlid_dn5, locals.var_ehlid_dn6, locals.var_ehlid_dn7, locals.var_ehlid_dn8, locals.var_ehlid_dn9, locals.var_ehlid_dn10, locals.var_ehlid_dn11,)
    }
};
        locals.var_ehlid = assign49110_e82140;
        locals.var_ehlid_dn3 = assign49110_e82140_d_n3;
        locals.var_ehlid_dn4 = assign49110_e82140_d_n4;
        locals.var_ehlid_dn5 = assign49110_e82140_d_n5;
        locals.var_ehlid_dn6 = assign49110_e82140_d_n6;
        locals.var_ehlid_dn7 = assign49110_e82140_d_n7;
        locals.var_ehlid_dn8 = assign49110_e82140_d_n8;
        locals.var_ehlid_dn9 = assign49110_e82140_d_n9;
        locals.var_ehlid_dn10 = assign49110_e82140_d_n10;
        locals.var_ehlid_dn11 = assign49110_e82140_d_n11;

        let assign49120_e82143: f64 = if locals.var_ehlid < 1e-5 { 1.0 } else { 0.0 };
        locals.var_guard752 = assign49120_e82143;

        let (assign49130_e82153, assign49130_e82153_d_n3, assign49130_e82153_d_n4, assign49130_e82153_d_n5, assign49130_e82153_d_n6, assign49130_e82153_d_n7, assign49130_e82153_d_n8, assign49130_e82153_d_n9, assign49130_e82153_d_n10, assign49130_e82153_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) && (locals.var_guard752 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ehlid, locals.var_ehlid_dn3, locals.var_ehlid_dn4, locals.var_ehlid_dn5, locals.var_ehlid_dn6, locals.var_ehlid_dn7, locals.var_ehlid_dn8, locals.var_ehlid_dn9, locals.var_ehlid_dn10, locals.var_ehlid_dn11,)
    }
};
        locals.var_ehlid = assign49130_e82153;
        locals.var_ehlid_dn3 = assign49130_e82153_d_n3;
        locals.var_ehlid_dn4 = assign49130_e82153_d_n4;
        locals.var_ehlid_dn5 = assign49130_e82153_d_n5;
        locals.var_ehlid_dn6 = assign49130_e82153_d_n6;
        locals.var_ehlid_dn7 = assign49130_e82153_d_n7;
        locals.var_ehlid_dn8 = assign49130_e82153_d_n8;
        locals.var_ehlid_dn9 = assign49130_e82153_d_n9;
        locals.var_ehlid_dn10 = assign49130_e82153_d_n10;
        locals.var_ehlid_dn11 = assign49130_e82153_d_n11;

        let (assign49140_e82163, assign49140_e82163_d_n3, assign49140_e82163_d_n4, assign49140_e82163_d_n5, assign49140_e82163_d_n6, assign49140_e82163_d_n7, assign49140_e82163_d_n8, assign49140_e82163_d_n9, assign49140_e82163_d_n10, assign49140_e82163_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) && (locals.var_guard752 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ehlidfactor, locals.var_ehlidfactor_dn3, locals.var_ehlidfactor_dn4, locals.var_ehlidfactor_dn5, locals.var_ehlidfactor_dn6, locals.var_ehlidfactor_dn7, locals.var_ehlidfactor_dn8, locals.var_ehlidfactor_dn9, locals.var_ehlidfactor_dn10, locals.var_ehlidfactor_dn11,)
    }
};
        locals.var_ehlidfactor = assign49140_e82163;
        locals.var_ehlidfactor_dn3 = assign49140_e82163_d_n3;
        locals.var_ehlidfactor_dn4 = assign49140_e82163_d_n4;
        locals.var_ehlidfactor_dn5 = assign49140_e82163_d_n5;
        locals.var_ehlidfactor_dn6 = assign49140_e82163_d_n6;
        locals.var_ehlidfactor_dn7 = assign49140_e82163_d_n7;
        locals.var_ehlidfactor_dn8 = assign49140_e82163_d_n8;
        locals.var_ehlidfactor_dn9 = assign49140_e82163_d_n9;
        locals.var_ehlidfactor_dn10 = assign49140_e82163_d_n10;
        locals.var_ehlidfactor_dn11 = assign49140_e82163_d_n11;

        let (assign49150_e82179, assign49150_e82179_d_n3, assign49150_e82179_d_n4, assign49150_e82179_d_n5, assign49150_e82179_d_n6, assign49150_e82179_d_n7, assign49150_e82179_d_n8, assign49150_e82179_d_n9, assign49150_e82179_d_n10, assign49150_e82179_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) && (locals.var_guard752 == 0.0)) {
        let assign49150_e82175: f64 = (1.0 + locals.var_ehlid);
        let assign49150_e82176: f64 = (assign49150_e82175).sqrt();
        let assign49150_e82177: f64 = (1.0 / assign49150_e82176);
        (assign49150_e82177, (-((locals.var_ehlid_dn3 / (2.0 * assign49150_e82176)) / (assign49150_e82176 * assign49150_e82176))), (-((locals.var_ehlid_dn4 / (2.0 * assign49150_e82176)) / (assign49150_e82176 * assign49150_e82176))), (-((locals.var_ehlid_dn5 / (2.0 * assign49150_e82176)) / (assign49150_e82176 * assign49150_e82176))), (-((locals.var_ehlid_dn6 / (2.0 * assign49150_e82176)) / (assign49150_e82176 * assign49150_e82176))), (-((locals.var_ehlid_dn7 / (2.0 * assign49150_e82176)) / (assign49150_e82176 * assign49150_e82176))), (-((locals.var_ehlid_dn8 / (2.0 * assign49150_e82176)) / (assign49150_e82176 * assign49150_e82176))), (-((locals.var_ehlid_dn9 / (2.0 * assign49150_e82176)) / (assign49150_e82176 * assign49150_e82176))), (-((locals.var_ehlid_dn10 / (2.0 * assign49150_e82176)) / (assign49150_e82176 * assign49150_e82176))), (-((locals.var_ehlid_dn11 / (2.0 * assign49150_e82176)) / (assign49150_e82176 * assign49150_e82176))),)
    } else {
        (locals.var_ehlidfactor, locals.var_ehlidfactor_dn3, locals.var_ehlidfactor_dn4, locals.var_ehlidfactor_dn5, locals.var_ehlidfactor_dn6, locals.var_ehlidfactor_dn7, locals.var_ehlidfactor_dn8, locals.var_ehlidfactor_dn9, locals.var_ehlidfactor_dn10, locals.var_ehlidfactor_dn11,)
    }
};
        locals.var_ehlidfactor = assign49150_e82179;
        locals.var_ehlidfactor_dn3 = assign49150_e82179_d_n3;
        locals.var_ehlidfactor_dn4 = assign49150_e82179_d_n4;
        locals.var_ehlidfactor_dn5 = assign49150_e82179_d_n5;
        locals.var_ehlidfactor_dn6 = assign49150_e82179_d_n6;
        locals.var_ehlidfactor_dn7 = assign49150_e82179_d_n7;
        locals.var_ehlidfactor_dn8 = assign49150_e82179_d_n8;
        locals.var_ehlidfactor_dn9 = assign49150_e82179_d_n9;
        locals.var_ehlidfactor_dn10 = assign49150_e82179_d_n10;
        locals.var_ehlidfactor_dn11 = assign49150_e82179_d_n11;

        let (assign49160_e82196, assign49160_e82196_d_n3, assign49160_e82196_d_n4, assign49160_e82196_d_n5, assign49160_e82196_d_n6, assign49160_e82196_d_n7, assign49160_e82196_d_n8, assign49160_e82196_d_n9, assign49160_e82196_d_n10, assign49160_e82196_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) {
        let assign49160_e82186: f64 = (-0.5);
        let assign49160_e82188: f64 = (assign49160_e82186 * locals.var_leff);
        let assign49160_e82190: f64 = (assign49160_e82188 * locals.var_leff);
        let __rspice_inv_cse_0: f64 = 1.0 / p.p595;
        let assign49160_e82192: f64 = (assign49160_e82190 * __rspice_inv_cse_0);
        let assign49160_e82194: f64 = (assign49160_e82192 * __rspice_inv_cse_0);
        (assign49160_e82194, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign49160_e82196;
        locals.var_t0_dn3 = assign49160_e82196_d_n3;
        locals.var_t0_dn4 = assign49160_e82196_d_n4;
        locals.var_t0_dn5 = assign49160_e82196_d_n5;
        locals.var_t0_dn6 = assign49160_e82196_d_n6;
        locals.var_t0_dn7 = assign49160_e82196_d_n7;
        locals.var_t0_dn8 = assign49160_e82196_d_n8;
        locals.var_t0_dn9 = assign49160_e82196_d_n9;
        locals.var_t0_dn10 = assign49160_e82196_d_n10;
        locals.var_t0_dn11 = assign49160_e82196_d_n11;

        let (assign49170_e82205, assign49170_e82205_d_n3, assign49170_e82205_d_n4, assign49170_e82205_d_n5, assign49170_e82205_d_n6, assign49170_e82205_d_n7, assign49170_e82205_d_n8, assign49170_e82205_d_n9, assign49170_e82205_d_n10, assign49170_e82205_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) {
        let assign49170_e82203: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign49170_e82203, ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn3), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11),)
    } else {
        (locals.var_alphabjt, locals.var_alphabjt_dn3, locals.var_alphabjt_dn4, locals.var_alphabjt_dn5, locals.var_alphabjt_dn6, locals.var_alphabjt_dn7, locals.var_alphabjt_dn8, locals.var_alphabjt_dn9, locals.var_alphabjt_dn10, locals.var_alphabjt_dn11,)
    }
};
        locals.var_alphabjt = assign49170_e82205;
        locals.var_alphabjt_dn3 = assign49170_e82205_d_n3;
        locals.var_alphabjt_dn4 = assign49170_e82205_d_n4;
        locals.var_alphabjt_dn5 = assign49170_e82205_d_n5;
        locals.var_alphabjt_dn6 = assign49170_e82205_d_n6;
        locals.var_alphabjt_dn7 = assign49170_e82205_d_n7;
        locals.var_alphabjt_dn8 = assign49170_e82205_d_n8;
        locals.var_alphabjt_dn9 = assign49170_e82205_d_n9;
        locals.var_alphabjt_dn10 = assign49170_e82205_d_n10;
        locals.var_alphabjt_dn11 = assign49170_e82205_d_n11;

        let (assign49180_e82215, assign49180_e82215_d_n3, assign49180_e82215_d_n4, assign49180_e82215_d_n5, assign49180_e82215_d_n6, assign49180_e82215_d_n7, assign49180_e82215_d_n8, assign49180_e82215_d_n9, assign49180_e82215_d_n10, assign49180_e82215_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) {
        let assign49180_e82213: f64 = (1.0 - locals.var_alphabjt);
        (assign49180_e82213, (-locals.var_alphabjt_dn3), (-locals.var_alphabjt_dn4), (-locals.var_alphabjt_dn5), (-locals.var_alphabjt_dn6), (-locals.var_alphabjt_dn7), (-locals.var_alphabjt_dn8), (-locals.var_alphabjt_dn9), (-locals.var_alphabjt_dn10), (-locals.var_alphabjt_dn11),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign49180_e82215;
        locals.var_t2_dn3 = assign49180_e82215_d_n3;
        locals.var_t2_dn4 = assign49180_e82215_d_n4;
        locals.var_t2_dn5 = assign49180_e82215_d_n5;
        locals.var_t2_dn6 = assign49180_e82215_d_n6;
        locals.var_t2_dn7 = assign49180_e82215_d_n7;
        locals.var_t2_dn8 = assign49180_e82215_d_n8;
        locals.var_t2_dn9 = assign49180_e82215_d_n9;
        locals.var_t2_dn10 = assign49180_e82215_d_n10;
        locals.var_t2_dn11 = assign49180_e82215_d_n11;

        let (assign49190_e82231, assign49190_e82231_d_n3, assign49190_e82231_d_n4, assign49190_e82231_d_n5, assign49190_e82231_d_n6, assign49190_e82231_d_n7, assign49190_e82231_d_n8, assign49190_e82231_d_n9, assign49190_e82231_d_n10, assign49190_e82231_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) {
        let assign49190_e82224: f64 = (1.0 / locals.var_leff);
        let assign49190_e82227: f64 = (1.0 / p.p595);
        let assign49190_e82228: f64 = (assign49190_e82224 + assign49190_e82227);
        let assign49190_e82229: f64 = (locals.var_lbjt0_i * assign49190_e82228);
        (assign49190_e82229, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign49190_e82231;
        locals.var_t0_dn3 = assign49190_e82231_d_n3;
        locals.var_t0_dn4 = assign49190_e82231_d_n4;
        locals.var_t0_dn5 = assign49190_e82231_d_n5;
        locals.var_t0_dn6 = assign49190_e82231_d_n6;
        locals.var_t0_dn7 = assign49190_e82231_d_n7;
        locals.var_t0_dn8 = assign49190_e82231_d_n8;
        locals.var_t0_dn9 = assign49190_e82231_d_n9;
        locals.var_t0_dn10 = assign49190_e82231_d_n10;
        locals.var_t0_dn11 = assign49190_e82231_d_n11;

        let (assign49200_e82241, assign49200_e82241_d_n3, assign49200_e82241_d_n4, assign49200_e82241_d_n5, assign49200_e82241_d_n6, assign49200_e82241_d_n7, assign49200_e82241_d_n8, assign49200_e82241_d_n9, assign49200_e82241_d_n10, assign49200_e82241_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) {
        let assign49200_e82239: f64 = (locals.var_t0).powf(locals.var_nbjt_i);
        (assign49200_e82239, if 0.0 == 0.0 && ((locals.var_nbjt_i) as f64).is_finite() && ((locals.var_nbjt_i) as f64).fract() == 0.0 { if locals.var_nbjt_i == 0.0 { 0.0 } else { (locals.var_nbjt_i * ((locals.var_t0).powf(locals.var_nbjt_i - 1.0) * locals.var_t0_dn3)) } } else { (assign49200_e82239 * (locals.var_nbjt_i * (locals.var_t0_dn3 / locals.var_t0))) }, if 0.0 == 0.0 && ((locals.var_nbjt_i) as f64).is_finite() && ((locals.var_nbjt_i) as f64).fract() == 0.0 { if locals.var_nbjt_i == 0.0 { 0.0 } else { (locals.var_nbjt_i * ((locals.var_t0).powf(locals.var_nbjt_i - 1.0) * locals.var_t0_dn4)) } } else { (assign49200_e82239 * (locals.var_nbjt_i * (locals.var_t0_dn4 / locals.var_t0))) }, if 0.0 == 0.0 && ((locals.var_nbjt_i) as f64).is_finite() && ((locals.var_nbjt_i) as f64).fract() == 0.0 { if locals.var_nbjt_i == 0.0 { 0.0 } else { (locals.var_nbjt_i * ((locals.var_t0).powf(locals.var_nbjt_i - 1.0) * locals.var_t0_dn5)) } } else { (assign49200_e82239 * (locals.var_nbjt_i * (locals.var_t0_dn5 / locals.var_t0))) }, if 0.0 == 0.0 && ((locals.var_nbjt_i) as f64).is_finite() && ((locals.var_nbjt_i) as f64).fract() == 0.0 { if locals.var_nbjt_i == 0.0 { 0.0 } else { (locals.var_nbjt_i * ((locals.var_t0).powf(locals.var_nbjt_i - 1.0) * locals.var_t0_dn6)) } } else { (assign49200_e82239 * (locals.var_nbjt_i * (locals.var_t0_dn6 / locals.var_t0))) }, if 0.0 == 0.0 && ((locals.var_nbjt_i) as f64).is_finite() && ((locals.var_nbjt_i) as f64).fract() == 0.0 { if locals.var_nbjt_i == 0.0 { 0.0 } else { (locals.var_nbjt_i * ((locals.var_t0).powf(locals.var_nbjt_i - 1.0) * locals.var_t0_dn7)) } } else { (assign49200_e82239 * (locals.var_nbjt_i * (locals.var_t0_dn7 / locals.var_t0))) }, if 0.0 == 0.0 && ((locals.var_nbjt_i) as f64).is_finite() && ((locals.var_nbjt_i) as f64).fract() == 0.0 { if locals.var_nbjt_i == 0.0 { 0.0 } else { (locals.var_nbjt_i * ((locals.var_t0).powf(locals.var_nbjt_i - 1.0) * locals.var_t0_dn8)) } } else { (assign49200_e82239 * (locals.var_nbjt_i * (locals.var_t0_dn8 / locals.var_t0))) }, if 0.0 == 0.0 && ((locals.var_nbjt_i) as f64).is_finite() && ((locals.var_nbjt_i) as f64).fract() == 0.0 { if locals.var_nbjt_i == 0.0 { 0.0 } else { (locals.var_nbjt_i * ((locals.var_t0).powf(locals.var_nbjt_i - 1.0) * locals.var_t0_dn9)) } } else { (assign49200_e82239 * (locals.var_nbjt_i * (locals.var_t0_dn9 / locals.var_t0))) }, if 0.0 == 0.0 && ((locals.var_nbjt_i) as f64).is_finite() && ((locals.var_nbjt_i) as f64).fract() == 0.0 { if locals.var_nbjt_i == 0.0 { 0.0 } else { (locals.var_nbjt_i * ((locals.var_t0).powf(locals.var_nbjt_i - 1.0) * locals.var_t0_dn10)) } } else { (assign49200_e82239 * (locals.var_nbjt_i * (locals.var_t0_dn10 / locals.var_t0))) }, if 0.0 == 0.0 && ((locals.var_nbjt_i) as f64).is_finite() && ((locals.var_nbjt_i) as f64).fract() == 0.0 { if locals.var_nbjt_i == 0.0 { 0.0 } else { (locals.var_nbjt_i * ((locals.var_t0).powf(locals.var_nbjt_i - 1.0) * locals.var_t0_dn11)) } } else { (assign49200_e82239 * (locals.var_nbjt_i * (locals.var_t0_dn11 / locals.var_t0))) },)
    } else {
        (locals.var_lratio, locals.var_lratio_dn3, locals.var_lratio_dn4, locals.var_lratio_dn5, locals.var_lratio_dn6, locals.var_lratio_dn7, locals.var_lratio_dn8, locals.var_lratio_dn9, locals.var_lratio_dn10, locals.var_lratio_dn11,)
    }
};
        locals.var_lratio = assign49200_e82241;
        locals.var_lratio_dn3 = assign49200_e82241_d_n3;
        locals.var_lratio_dn4 = assign49200_e82241_d_n4;
        locals.var_lratio_dn5 = assign49200_e82241_d_n5;
        locals.var_lratio_dn6 = assign49200_e82241_d_n6;
        locals.var_lratio_dn7 = assign49200_e82241_d_n7;
        locals.var_lratio_dn8 = assign49200_e82241_d_n8;
        locals.var_lratio_dn9 = assign49200_e82241_d_n9;
        locals.var_lratio_dn10 = assign49200_e82241_d_n10;
        locals.var_lratio_dn11 = assign49200_e82241_d_n11;

        let (assign49210_e82253, assign49210_e82253_d_n3, assign49210_e82253_d_n4, assign49210_e82253_d_n5, assign49210_e82253_d_n6, assign49210_e82253_d_n7, assign49210_e82253_d_n8, assign49210_e82253_d_n9, assign49210_e82253_d_n10, assign49210_e82253_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) {
        let assign49210_e82249: f64 = (locals.var_wtsi * locals.var_jbjts);
        let assign49210_e82251: f64 = (assign49210_e82249 * locals.var_lratio);
        (assign49210_e82251, (((locals.var_wtsi * locals.var_jbjts_dn3) * locals.var_lratio) + (assign49210_e82249 * locals.var_lratio_dn3)), (((locals.var_wtsi * locals.var_jbjts_dn4) * locals.var_lratio) + (assign49210_e82249 * locals.var_lratio_dn4)), (((locals.var_wtsi * locals.var_jbjts_dn5) * locals.var_lratio) + (assign49210_e82249 * locals.var_lratio_dn5)), (((locals.var_wtsi * locals.var_jbjts_dn6) * locals.var_lratio) + (assign49210_e82249 * locals.var_lratio_dn6)), (((locals.var_wtsi * locals.var_jbjts_dn7) * locals.var_lratio) + (assign49210_e82249 * locals.var_lratio_dn7)), (((locals.var_wtsi * locals.var_jbjts_dn8) * locals.var_lratio) + (assign49210_e82249 * locals.var_lratio_dn8)), (((locals.var_wtsi * locals.var_jbjts_dn9) * locals.var_lratio) + (assign49210_e82249 * locals.var_lratio_dn9)), (((locals.var_wtsi * locals.var_jbjts_dn10) * locals.var_lratio) + (assign49210_e82249 * locals.var_lratio_dn10)), (((locals.var_wtsi * locals.var_jbjts_dn11) * locals.var_lratio) + (assign49210_e82249 * locals.var_lratio_dn11)),)
    } else {
        (locals.var_ien, locals.var_ien_dn3, locals.var_ien_dn4, locals.var_ien_dn5, locals.var_ien_dn6, locals.var_ien_dn7, locals.var_ien_dn8, locals.var_ien_dn9, locals.var_ien_dn10, locals.var_ien_dn11,)
    }
};
        locals.var_ien = assign49210_e82253;
        locals.var_ien_dn3 = assign49210_e82253_d_n3;
        locals.var_ien_dn4 = assign49210_e82253_d_n4;
        locals.var_ien_dn5 = assign49210_e82253_d_n5;
        locals.var_ien_dn6 = assign49210_e82253_d_n6;
        locals.var_ien_dn7 = assign49210_e82253_d_n7;
        locals.var_ien_dn8 = assign49210_e82253_d_n8;
        locals.var_ien_dn9 = assign49210_e82253_d_n9;
        locals.var_ien_dn10 = assign49210_e82253_d_n10;
        locals.var_ien_dn11 = assign49210_e82253_d_n11;

        let (assign49220_e82263, assign49220_e82263_d_n3, assign49220_e82263_d_n4, assign49220_e82263_d_n5, assign49220_e82263_d_n6, assign49220_e82263_d_n7, assign49220_e82263_d_n8, assign49220_e82263_d_n9, assign49220_e82263_d_n10, assign49220_e82263_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) {
        let assign49220_e82261: f64 = (locals.var_t0 * locals.var_ien);
        (assign49220_e82261, ((locals.var_t0_dn3 * locals.var_ien) + (locals.var_t0 * locals.var_ien_dn3)), ((locals.var_t0_dn4 * locals.var_ien) + (locals.var_t0 * locals.var_ien_dn4)), ((locals.var_t0_dn5 * locals.var_ien) + (locals.var_t0 * locals.var_ien_dn5)), ((locals.var_t0_dn6 * locals.var_ien) + (locals.var_t0 * locals.var_ien_dn6)), ((locals.var_t0_dn7 * locals.var_ien) + (locals.var_t0 * locals.var_ien_dn7)), ((locals.var_t0_dn8 * locals.var_ien) + (locals.var_t0 * locals.var_ien_dn8)), ((locals.var_t0_dn9 * locals.var_ien) + (locals.var_t0 * locals.var_ien_dn9)), ((locals.var_t0_dn10 * locals.var_ien) + (locals.var_t0 * locals.var_ien_dn10)), ((locals.var_t0_dn11 * locals.var_ien) + (locals.var_t0 * locals.var_ien_dn11)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign49220_e82263;
        locals.var_t1_dn3 = assign49220_e82263_d_n3;
        locals.var_t1_dn4 = assign49220_e82263_d_n4;
        locals.var_t1_dn5 = assign49220_e82263_d_n5;
        locals.var_t1_dn6 = assign49220_e82263_d_n6;
        locals.var_t1_dn7 = assign49220_e82263_d_n7;
        locals.var_t1_dn8 = assign49220_e82263_d_n8;
        locals.var_t1_dn9 = assign49220_e82263_d_n9;
        locals.var_t1_dn10 = assign49220_e82263_d_n10;
        locals.var_t1_dn11 = assign49220_e82263_d_n11;

        let (assign49230_e82277, assign49230_e82277_d_n3, assign49230_e82277_d_n4, assign49230_e82277_d_n5, assign49230_e82277_d_n6, assign49230_e82277_d_n7, assign49230_e82277_d_n8, assign49230_e82277_d_n9, assign49230_e82277_d_n10, assign49230_e82277_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) {
        let assign49230_e82272: f64 = (locals.var_expvbsnvtm - 1.0);
        let assign49230_e82273: f64 = (locals.var_t1 * assign49230_e82272);
        let assign49230_e82275: f64 = (assign49230_e82273 * locals.var_ehlisfactor);
        (assign49230_e82275, ((((locals.var_t1_dn3 * assign49230_e82272) + (locals.var_t1 * locals.var_expvbsnvtm_dn3)) * locals.var_ehlisfactor) + (assign49230_e82273 * locals.var_ehlisfactor_dn3)), ((((locals.var_t1_dn4 * assign49230_e82272) + (locals.var_t1 * locals.var_expvbsnvtm_dn4)) * locals.var_ehlisfactor) + (assign49230_e82273 * locals.var_ehlisfactor_dn4)), ((((locals.var_t1_dn5 * assign49230_e82272) + (locals.var_t1 * locals.var_expvbsnvtm_dn5)) * locals.var_ehlisfactor) + (assign49230_e82273 * locals.var_ehlisfactor_dn5)), ((((locals.var_t1_dn6 * assign49230_e82272) + (locals.var_t1 * locals.var_expvbsnvtm_dn6)) * locals.var_ehlisfactor) + (assign49230_e82273 * locals.var_ehlisfactor_dn6)), ((((locals.var_t1_dn7 * assign49230_e82272) + (locals.var_t1 * locals.var_expvbsnvtm_dn7)) * locals.var_ehlisfactor) + (assign49230_e82273 * locals.var_ehlisfactor_dn7)), ((((locals.var_t1_dn8 * assign49230_e82272) + (locals.var_t1 * locals.var_expvbsnvtm_dn8)) * locals.var_ehlisfactor) + (assign49230_e82273 * locals.var_ehlisfactor_dn8)), ((((locals.var_t1_dn9 * assign49230_e82272) + (locals.var_t1 * locals.var_expvbsnvtm_dn9)) * locals.var_ehlisfactor) + (assign49230_e82273 * locals.var_ehlisfactor_dn9)), ((((locals.var_t1_dn10 * assign49230_e82272) + (locals.var_t1 * locals.var_expvbsnvtm_dn10)) * locals.var_ehlisfactor) + (assign49230_e82273 * locals.var_ehlisfactor_dn10)), ((((locals.var_t1_dn11 * assign49230_e82272) + (locals.var_t1 * locals.var_expvbsnvtm_dn11)) * locals.var_ehlisfactor) + (assign49230_e82273 * locals.var_ehlisfactor_dn11)),)
    } else {
        (locals.var_ibs3, locals.var_ibs3_dn3, locals.var_ibs3_dn4, locals.var_ibs3_dn5, locals.var_ibs3_dn6, locals.var_ibs3_dn7, locals.var_ibs3_dn8, locals.var_ibs3_dn9, locals.var_ibs3_dn10, locals.var_ibs3_dn11,)
    }
};
        locals.var_ibs3 = assign49230_e82277;
        locals.var_ibs3_dn3 = assign49230_e82277_d_n3;
        locals.var_ibs3_dn4 = assign49230_e82277_d_n4;
        locals.var_ibs3_dn5 = assign49230_e82277_d_n5;
        locals.var_ibs3_dn6 = assign49230_e82277_d_n6;
        locals.var_ibs3_dn7 = assign49230_e82277_d_n7;
        locals.var_ibs3_dn8 = assign49230_e82277_d_n8;
        locals.var_ibs3_dn9 = assign49230_e82277_d_n9;
        locals.var_ibs3_dn10 = assign49230_e82277_d_n10;
        locals.var_ibs3_dn11 = assign49230_e82277_d_n11;

        let (assign49240_e82289, assign49240_e82289_d_n3, assign49240_e82289_d_n4, assign49240_e82289_d_n5, assign49240_e82289_d_n6, assign49240_e82289_d_n7, assign49240_e82289_d_n8, assign49240_e82289_d_n9, assign49240_e82289_d_n10, assign49240_e82289_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) {
        let assign49240_e82285: f64 = (locals.var_wtsi * locals.var_jbjtd);
        let assign49240_e82287: f64 = (assign49240_e82285 * locals.var_lratio);
        (assign49240_e82287, (((locals.var_wtsi * locals.var_jbjtd_dn3) * locals.var_lratio) + (assign49240_e82285 * locals.var_lratio_dn3)), (((locals.var_wtsi * locals.var_jbjtd_dn4) * locals.var_lratio) + (assign49240_e82285 * locals.var_lratio_dn4)), (((locals.var_wtsi * locals.var_jbjtd_dn5) * locals.var_lratio) + (assign49240_e82285 * locals.var_lratio_dn5)), (((locals.var_wtsi * locals.var_jbjtd_dn6) * locals.var_lratio) + (assign49240_e82285 * locals.var_lratio_dn6)), (((locals.var_wtsi * locals.var_jbjtd_dn7) * locals.var_lratio) + (assign49240_e82285 * locals.var_lratio_dn7)), (((locals.var_wtsi * locals.var_jbjtd_dn8) * locals.var_lratio) + (assign49240_e82285 * locals.var_lratio_dn8)), (((locals.var_wtsi * locals.var_jbjtd_dn9) * locals.var_lratio) + (assign49240_e82285 * locals.var_lratio_dn9)), (((locals.var_wtsi * locals.var_jbjtd_dn10) * locals.var_lratio) + (assign49240_e82285 * locals.var_lratio_dn10)), (((locals.var_wtsi * locals.var_jbjtd_dn11) * locals.var_lratio) + (assign49240_e82285 * locals.var_lratio_dn11)),)
    } else {
        (locals.var_ien, locals.var_ien_dn3, locals.var_ien_dn4, locals.var_ien_dn5, locals.var_ien_dn6, locals.var_ien_dn7, locals.var_ien_dn8, locals.var_ien_dn9, locals.var_ien_dn10, locals.var_ien_dn11,)
    }
};
        locals.var_ien = assign49240_e82289;
        locals.var_ien_dn3 = assign49240_e82289_d_n3;
        locals.var_ien_dn4 = assign49240_e82289_d_n4;
        locals.var_ien_dn5 = assign49240_e82289_d_n5;
        locals.var_ien_dn6 = assign49240_e82289_d_n6;
        locals.var_ien_dn7 = assign49240_e82289_d_n7;
        locals.var_ien_dn8 = assign49240_e82289_d_n8;
        locals.var_ien_dn9 = assign49240_e82289_d_n9;
        locals.var_ien_dn10 = assign49240_e82289_d_n10;
        locals.var_ien_dn11 = assign49240_e82289_d_n11;

        let (assign49250_e82299, assign49250_e82299_d_n3, assign49250_e82299_d_n4, assign49250_e82299_d_n5, assign49250_e82299_d_n6, assign49250_e82299_d_n7, assign49250_e82299_d_n8, assign49250_e82299_d_n9, assign49250_e82299_d_n10, assign49250_e82299_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) {
        let assign49250_e82297: f64 = (locals.var_t0 * locals.var_ien);
        (assign49250_e82297, ((locals.var_t0_dn3 * locals.var_ien) + (locals.var_t0 * locals.var_ien_dn3)), ((locals.var_t0_dn4 * locals.var_ien) + (locals.var_t0 * locals.var_ien_dn4)), ((locals.var_t0_dn5 * locals.var_ien) + (locals.var_t0 * locals.var_ien_dn5)), ((locals.var_t0_dn6 * locals.var_ien) + (locals.var_t0 * locals.var_ien_dn6)), ((locals.var_t0_dn7 * locals.var_ien) + (locals.var_t0 * locals.var_ien_dn7)), ((locals.var_t0_dn8 * locals.var_ien) + (locals.var_t0 * locals.var_ien_dn8)), ((locals.var_t0_dn9 * locals.var_ien) + (locals.var_t0 * locals.var_ien_dn9)), ((locals.var_t0_dn10 * locals.var_ien) + (locals.var_t0 * locals.var_ien_dn10)), ((locals.var_t0_dn11 * locals.var_ien) + (locals.var_t0 * locals.var_ien_dn11)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign49250_e82299;
        locals.var_t1_dn3 = assign49250_e82299_d_n3;
        locals.var_t1_dn4 = assign49250_e82299_d_n4;
        locals.var_t1_dn5 = assign49250_e82299_d_n5;
        locals.var_t1_dn6 = assign49250_e82299_d_n6;
        locals.var_t1_dn7 = assign49250_e82299_d_n7;
        locals.var_t1_dn8 = assign49250_e82299_d_n8;
        locals.var_t1_dn9 = assign49250_e82299_d_n9;
        locals.var_t1_dn10 = assign49250_e82299_d_n10;
        locals.var_t1_dn11 = assign49250_e82299_d_n11;

        let (assign49260_e82313, assign49260_e82313_d_n3, assign49260_e82313_d_n4, assign49260_e82313_d_n5, assign49260_e82313_d_n6, assign49260_e82313_d_n7, assign49260_e82313_d_n8, assign49260_e82313_d_n9, assign49260_e82313_d_n10, assign49260_e82313_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) {
        let assign49260_e82308: f64 = (locals.var_expvbdnvtm - 1.0);
        let assign49260_e82309: f64 = (locals.var_t1 * assign49260_e82308);
        let assign49260_e82311: f64 = (assign49260_e82309 * locals.var_ehlidfactor);
        (assign49260_e82311, ((((locals.var_t1_dn3 * assign49260_e82308) + (locals.var_t1 * locals.var_expvbdnvtm_dn3)) * locals.var_ehlidfactor) + (assign49260_e82309 * locals.var_ehlidfactor_dn3)), ((((locals.var_t1_dn4 * assign49260_e82308) + (locals.var_t1 * locals.var_expvbdnvtm_dn4)) * locals.var_ehlidfactor) + (assign49260_e82309 * locals.var_ehlidfactor_dn4)), ((((locals.var_t1_dn5 * assign49260_e82308) + (locals.var_t1 * locals.var_expvbdnvtm_dn5)) * locals.var_ehlidfactor) + (assign49260_e82309 * locals.var_ehlidfactor_dn5)), ((((locals.var_t1_dn6 * assign49260_e82308) + (locals.var_t1 * locals.var_expvbdnvtm_dn6)) * locals.var_ehlidfactor) + (assign49260_e82309 * locals.var_ehlidfactor_dn6)), ((((locals.var_t1_dn7 * assign49260_e82308) + (locals.var_t1 * locals.var_expvbdnvtm_dn7)) * locals.var_ehlidfactor) + (assign49260_e82309 * locals.var_ehlidfactor_dn7)), ((((locals.var_t1_dn8 * assign49260_e82308) + (locals.var_t1 * locals.var_expvbdnvtm_dn8)) * locals.var_ehlidfactor) + (assign49260_e82309 * locals.var_ehlidfactor_dn8)), ((((locals.var_t1_dn9 * assign49260_e82308) + (locals.var_t1 * locals.var_expvbdnvtm_dn9)) * locals.var_ehlidfactor) + (assign49260_e82309 * locals.var_ehlidfactor_dn9)), ((((locals.var_t1_dn10 * assign49260_e82308) + (locals.var_t1 * locals.var_expvbdnvtm_dn10)) * locals.var_ehlidfactor) + (assign49260_e82309 * locals.var_ehlidfactor_dn10)), ((((locals.var_t1_dn11 * assign49260_e82308) + (locals.var_t1 * locals.var_expvbdnvtm_dn11)) * locals.var_ehlidfactor) + (assign49260_e82309 * locals.var_ehlidfactor_dn11)),)
    } else {
        (locals.var_ibd3, locals.var_ibd3_dn3, locals.var_ibd3_dn4, locals.var_ibd3_dn5, locals.var_ibd3_dn6, locals.var_ibd3_dn7, locals.var_ibd3_dn8, locals.var_ibd3_dn9, locals.var_ibd3_dn10, locals.var_ibd3_dn11,)
    }
};
        locals.var_ibd3 = assign49260_e82313;
        locals.var_ibd3_dn3 = assign49260_e82313_d_n3;
        locals.var_ibd3_dn4 = assign49260_e82313_d_n4;
        locals.var_ibd3_dn5 = assign49260_e82313_d_n5;
        locals.var_ibd3_dn6 = assign49260_e82313_d_n6;
        locals.var_ibd3_dn7 = assign49260_e82313_d_n7;
        locals.var_ibd3_dn8 = assign49260_e82313_d_n8;
        locals.var_ibd3_dn9 = assign49260_e82313_d_n9;
        locals.var_ibd3_dn10 = assign49260_e82313_d_n10;
        locals.var_ibd3_dn11 = assign49260_e82313_d_n11;

        let (assign49270_e82327, assign49270_e82327_d_n3, assign49270_e82327_d_n4, assign49270_e82327_d_n5, assign49270_e82327_d_n6, assign49270_e82327_d_n7, assign49270_e82327_d_n8, assign49270_e82327_d_n9, assign49270_e82327_d_n10, assign49270_e82327_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) {
        let assign49270_e82323: f64 = (locals.var_t0).powf(locals.var_ndif_i);
        let assign49270_e82324: f64 = (p.p920 * assign49270_e82323);
        let assign49270_e82325: f64 = (1.0 + assign49270_e82324);
        (assign49270_e82325, (p.p920 * if 0.0 == 0.0 && ((locals.var_ndif_i) as f64).is_finite() && ((locals.var_ndif_i) as f64).fract() == 0.0 { if locals.var_ndif_i == 0.0 { 0.0 } else { (locals.var_ndif_i * ((locals.var_t0).powf(locals.var_ndif_i - 1.0) * locals.var_t0_dn3)) } } else { (assign49270_e82323 * (locals.var_ndif_i * (locals.var_t0_dn3 / locals.var_t0))) }), (p.p920 * if 0.0 == 0.0 && ((locals.var_ndif_i) as f64).is_finite() && ((locals.var_ndif_i) as f64).fract() == 0.0 { if locals.var_ndif_i == 0.0 { 0.0 } else { (locals.var_ndif_i * ((locals.var_t0).powf(locals.var_ndif_i - 1.0) * locals.var_t0_dn4)) } } else { (assign49270_e82323 * (locals.var_ndif_i * (locals.var_t0_dn4 / locals.var_t0))) }), (p.p920 * if 0.0 == 0.0 && ((locals.var_ndif_i) as f64).is_finite() && ((locals.var_ndif_i) as f64).fract() == 0.0 { if locals.var_ndif_i == 0.0 { 0.0 } else { (locals.var_ndif_i * ((locals.var_t0).powf(locals.var_ndif_i - 1.0) * locals.var_t0_dn5)) } } else { (assign49270_e82323 * (locals.var_ndif_i * (locals.var_t0_dn5 / locals.var_t0))) }), (p.p920 * if 0.0 == 0.0 && ((locals.var_ndif_i) as f64).is_finite() && ((locals.var_ndif_i) as f64).fract() == 0.0 { if locals.var_ndif_i == 0.0 { 0.0 } else { (locals.var_ndif_i * ((locals.var_t0).powf(locals.var_ndif_i - 1.0) * locals.var_t0_dn6)) } } else { (assign49270_e82323 * (locals.var_ndif_i * (locals.var_t0_dn6 / locals.var_t0))) }), (p.p920 * if 0.0 == 0.0 && ((locals.var_ndif_i) as f64).is_finite() && ((locals.var_ndif_i) as f64).fract() == 0.0 { if locals.var_ndif_i == 0.0 { 0.0 } else { (locals.var_ndif_i * ((locals.var_t0).powf(locals.var_ndif_i - 1.0) * locals.var_t0_dn7)) } } else { (assign49270_e82323 * (locals.var_ndif_i * (locals.var_t0_dn7 / locals.var_t0))) }), (p.p920 * if 0.0 == 0.0 && ((locals.var_ndif_i) as f64).is_finite() && ((locals.var_ndif_i) as f64).fract() == 0.0 { if locals.var_ndif_i == 0.0 { 0.0 } else { (locals.var_ndif_i * ((locals.var_t0).powf(locals.var_ndif_i - 1.0) * locals.var_t0_dn8)) } } else { (assign49270_e82323 * (locals.var_ndif_i * (locals.var_t0_dn8 / locals.var_t0))) }), (p.p920 * if 0.0 == 0.0 && ((locals.var_ndif_i) as f64).is_finite() && ((locals.var_ndif_i) as f64).fract() == 0.0 { if locals.var_ndif_i == 0.0 { 0.0 } else { (locals.var_ndif_i * ((locals.var_t0).powf(locals.var_ndif_i - 1.0) * locals.var_t0_dn9)) } } else { (assign49270_e82323 * (locals.var_ndif_i * (locals.var_t0_dn9 / locals.var_t0))) }), (p.p920 * if 0.0 == 0.0 && ((locals.var_ndif_i) as f64).is_finite() && ((locals.var_ndif_i) as f64).fract() == 0.0 { if locals.var_ndif_i == 0.0 { 0.0 } else { (locals.var_ndif_i * ((locals.var_t0).powf(locals.var_ndif_i - 1.0) * locals.var_t0_dn10)) } } else { (assign49270_e82323 * (locals.var_ndif_i * (locals.var_t0_dn10 / locals.var_t0))) }), (p.p920 * if 0.0 == 0.0 && ((locals.var_ndif_i) as f64).is_finite() && ((locals.var_ndif_i) as f64).fract() == 0.0 { if locals.var_ndif_i == 0.0 { 0.0 } else { (locals.var_ndif_i * ((locals.var_t0).powf(locals.var_ndif_i - 1.0) * locals.var_t0_dn11)) } } else { (assign49270_e82323 * (locals.var_ndif_i * (locals.var_t0_dn11 / locals.var_t0))) }),)
    } else {
        (locals.var_lratiodif, locals.var_lratiodif_dn3, locals.var_lratiodif_dn4, locals.var_lratiodif_dn5, locals.var_lratiodif_dn6, locals.var_lratiodif_dn7, locals.var_lratiodif_dn8, locals.var_lratiodif_dn9, locals.var_lratiodif_dn10, locals.var_lratiodif_dn11,)
    }
};
        locals.var_lratiodif = assign49270_e82327;
        locals.var_lratiodif_dn3 = assign49270_e82327_d_n3;
        locals.var_lratiodif_dn4 = assign49270_e82327_d_n4;
        locals.var_lratiodif_dn5 = assign49270_e82327_d_n5;
        locals.var_lratiodif_dn6 = assign49270_e82327_d_n6;
        locals.var_lratiodif_dn7 = assign49270_e82327_d_n7;
        locals.var_lratiodif_dn8 = assign49270_e82327_d_n8;
        locals.var_lratiodif_dn9 = assign49270_e82327_d_n9;
        locals.var_lratiodif_dn10 = assign49270_e82327_d_n10;
        locals.var_lratiodif_dn11 = assign49270_e82327_d_n11;

        let (assign49280_e82339, assign49280_e82339_d_n3, assign49280_e82339_d_n4, assign49280_e82339_d_n5, assign49280_e82339_d_n6, assign49280_e82339_d_n7, assign49280_e82339_d_n8, assign49280_e82339_d_n9, assign49280_e82339_d_n10, assign49280_e82339_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) {
        let assign49280_e82335: f64 = (locals.var_wtsi * locals.var_jbjts);
        let assign49280_e82337: f64 = (assign49280_e82335 * locals.var_lratiodif);
        (assign49280_e82337, (((locals.var_wtsi * locals.var_jbjts_dn3) * locals.var_lratiodif) + (assign49280_e82335 * locals.var_lratiodif_dn3)), (((locals.var_wtsi * locals.var_jbjts_dn4) * locals.var_lratiodif) + (assign49280_e82335 * locals.var_lratiodif_dn4)), (((locals.var_wtsi * locals.var_jbjts_dn5) * locals.var_lratiodif) + (assign49280_e82335 * locals.var_lratiodif_dn5)), (((locals.var_wtsi * locals.var_jbjts_dn6) * locals.var_lratiodif) + (assign49280_e82335 * locals.var_lratiodif_dn6)), (((locals.var_wtsi * locals.var_jbjts_dn7) * locals.var_lratiodif) + (assign49280_e82335 * locals.var_lratiodif_dn7)), (((locals.var_wtsi * locals.var_jbjts_dn8) * locals.var_lratiodif) + (assign49280_e82335 * locals.var_lratiodif_dn8)), (((locals.var_wtsi * locals.var_jbjts_dn9) * locals.var_lratiodif) + (assign49280_e82335 * locals.var_lratiodif_dn9)), (((locals.var_wtsi * locals.var_jbjts_dn10) * locals.var_lratiodif) + (assign49280_e82335 * locals.var_lratiodif_dn10)), (((locals.var_wtsi * locals.var_jbjts_dn11) * locals.var_lratiodif) + (assign49280_e82335 * locals.var_lratiodif_dn11)),)
    } else {
        (locals.var_iendif, locals.var_iendif_dn3, locals.var_iendif_dn4, locals.var_iendif_dn5, locals.var_iendif_dn6, locals.var_iendif_dn7, locals.var_iendif_dn8, locals.var_iendif_dn9, locals.var_iendif_dn10, locals.var_iendif_dn11,)
    }
};
        locals.var_iendif = assign49280_e82339;
        locals.var_iendif_dn3 = assign49280_e82339_d_n3;
        locals.var_iendif_dn4 = assign49280_e82339_d_n4;
        locals.var_iendif_dn5 = assign49280_e82339_d_n5;
        locals.var_iendif_dn6 = assign49280_e82339_d_n6;
        locals.var_iendif_dn7 = assign49280_e82339_d_n7;
        locals.var_iendif_dn8 = assign49280_e82339_d_n8;
        locals.var_iendif_dn9 = assign49280_e82339_d_n9;
        locals.var_iendif_dn10 = assign49280_e82339_d_n10;
        locals.var_iendif_dn11 = assign49280_e82339_d_n11;

        let (assign49290_e82353, assign49290_e82353_d_n3, assign49290_e82353_d_n4, assign49290_e82353_d_n5, assign49290_e82353_d_n6, assign49290_e82353_d_n7, assign49290_e82353_d_n8, assign49290_e82353_d_n9, assign49290_e82353_d_n10, assign49290_e82353_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) {
        let assign49290_e82348: f64 = (locals.var_expvbsnvtm - 1.0);
        let assign49290_e82349: f64 = (locals.var_iendif * assign49290_e82348);
        let assign49290_e82351: f64 = (assign49290_e82349 * locals.var_ehlisfactor);
        (assign49290_e82351, ((((locals.var_iendif_dn3 * assign49290_e82348) + (locals.var_iendif * locals.var_expvbsnvtm_dn3)) * locals.var_ehlisfactor) + (assign49290_e82349 * locals.var_ehlisfactor_dn3)), ((((locals.var_iendif_dn4 * assign49290_e82348) + (locals.var_iendif * locals.var_expvbsnvtm_dn4)) * locals.var_ehlisfactor) + (assign49290_e82349 * locals.var_ehlisfactor_dn4)), ((((locals.var_iendif_dn5 * assign49290_e82348) + (locals.var_iendif * locals.var_expvbsnvtm_dn5)) * locals.var_ehlisfactor) + (assign49290_e82349 * locals.var_ehlisfactor_dn5)), ((((locals.var_iendif_dn6 * assign49290_e82348) + (locals.var_iendif * locals.var_expvbsnvtm_dn6)) * locals.var_ehlisfactor) + (assign49290_e82349 * locals.var_ehlisfactor_dn6)), ((((locals.var_iendif_dn7 * assign49290_e82348) + (locals.var_iendif * locals.var_expvbsnvtm_dn7)) * locals.var_ehlisfactor) + (assign49290_e82349 * locals.var_ehlisfactor_dn7)), ((((locals.var_iendif_dn8 * assign49290_e82348) + (locals.var_iendif * locals.var_expvbsnvtm_dn8)) * locals.var_ehlisfactor) + (assign49290_e82349 * locals.var_ehlisfactor_dn8)), ((((locals.var_iendif_dn9 * assign49290_e82348) + (locals.var_iendif * locals.var_expvbsnvtm_dn9)) * locals.var_ehlisfactor) + (assign49290_e82349 * locals.var_ehlisfactor_dn9)), ((((locals.var_iendif_dn10 * assign49290_e82348) + (locals.var_iendif * locals.var_expvbsnvtm_dn10)) * locals.var_ehlisfactor) + (assign49290_e82349 * locals.var_ehlisfactor_dn10)), ((((locals.var_iendif_dn11 * assign49290_e82348) + (locals.var_iendif * locals.var_expvbsnvtm_dn11)) * locals.var_ehlisfactor) + (assign49290_e82349 * locals.var_ehlisfactor_dn11)),)
    } else {
        (locals.var_ibsdif, locals.var_ibsdif_dn3, locals.var_ibsdif_dn4, locals.var_ibsdif_dn5, locals.var_ibsdif_dn6, locals.var_ibsdif_dn7, locals.var_ibsdif_dn8, locals.var_ibsdif_dn9, locals.var_ibsdif_dn10, locals.var_ibsdif_dn11,)
    }
};
        locals.var_ibsdif = assign49290_e82353;
        locals.var_ibsdif_dn3 = assign49290_e82353_d_n3;
        locals.var_ibsdif_dn4 = assign49290_e82353_d_n4;
        locals.var_ibsdif_dn5 = assign49290_e82353_d_n5;
        locals.var_ibsdif_dn6 = assign49290_e82353_d_n6;
        locals.var_ibsdif_dn7 = assign49290_e82353_d_n7;
        locals.var_ibsdif_dn8 = assign49290_e82353_d_n8;
        locals.var_ibsdif_dn9 = assign49290_e82353_d_n9;
        locals.var_ibsdif_dn10 = assign49290_e82353_d_n10;
        locals.var_ibsdif_dn11 = assign49290_e82353_d_n11;

    }

    pub(super) fn stamp_transient_block_167(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign49300_e82365, assign49300_e82365_d_n3, assign49300_e82365_d_n4, assign49300_e82365_d_n5, assign49300_e82365_d_n6, assign49300_e82365_d_n7, assign49300_e82365_d_n8, assign49300_e82365_d_n9, assign49300_e82365_d_n10, assign49300_e82365_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) {
        let assign49300_e82361: f64 = (locals.var_wtsi * locals.var_jbjtd);
        let assign49300_e82363: f64 = (assign49300_e82361 * locals.var_lratiodif);
        (assign49300_e82363, (((locals.var_wtsi * locals.var_jbjtd_dn3) * locals.var_lratiodif) + (assign49300_e82361 * locals.var_lratiodif_dn3)), (((locals.var_wtsi * locals.var_jbjtd_dn4) * locals.var_lratiodif) + (assign49300_e82361 * locals.var_lratiodif_dn4)), (((locals.var_wtsi * locals.var_jbjtd_dn5) * locals.var_lratiodif) + (assign49300_e82361 * locals.var_lratiodif_dn5)), (((locals.var_wtsi * locals.var_jbjtd_dn6) * locals.var_lratiodif) + (assign49300_e82361 * locals.var_lratiodif_dn6)), (((locals.var_wtsi * locals.var_jbjtd_dn7) * locals.var_lratiodif) + (assign49300_e82361 * locals.var_lratiodif_dn7)), (((locals.var_wtsi * locals.var_jbjtd_dn8) * locals.var_lratiodif) + (assign49300_e82361 * locals.var_lratiodif_dn8)), (((locals.var_wtsi * locals.var_jbjtd_dn9) * locals.var_lratiodif) + (assign49300_e82361 * locals.var_lratiodif_dn9)), (((locals.var_wtsi * locals.var_jbjtd_dn10) * locals.var_lratiodif) + (assign49300_e82361 * locals.var_lratiodif_dn10)), (((locals.var_wtsi * locals.var_jbjtd_dn11) * locals.var_lratiodif) + (assign49300_e82361 * locals.var_lratiodif_dn11)),)
    } else {
        (locals.var_iendif, locals.var_iendif_dn3, locals.var_iendif_dn4, locals.var_iendif_dn5, locals.var_iendif_dn6, locals.var_iendif_dn7, locals.var_iendif_dn8, locals.var_iendif_dn9, locals.var_iendif_dn10, locals.var_iendif_dn11,)
    }
};
        locals.var_iendif = assign49300_e82365;
        locals.var_iendif_dn3 = assign49300_e82365_d_n3;
        locals.var_iendif_dn4 = assign49300_e82365_d_n4;
        locals.var_iendif_dn5 = assign49300_e82365_d_n5;
        locals.var_iendif_dn6 = assign49300_e82365_d_n6;
        locals.var_iendif_dn7 = assign49300_e82365_d_n7;
        locals.var_iendif_dn8 = assign49300_e82365_d_n8;
        locals.var_iendif_dn9 = assign49300_e82365_d_n9;
        locals.var_iendif_dn10 = assign49300_e82365_d_n10;
        locals.var_iendif_dn11 = assign49300_e82365_d_n11;

        let (assign49310_e82379, assign49310_e82379_d_n3, assign49310_e82379_d_n4, assign49310_e82379_d_n5, assign49310_e82379_d_n6, assign49310_e82379_d_n7, assign49310_e82379_d_n8, assign49310_e82379_d_n9, assign49310_e82379_d_n10, assign49310_e82379_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) {
        let assign49310_e82374: f64 = (locals.var_expvbdnvtm - 1.0);
        let assign49310_e82375: f64 = (locals.var_iendif * assign49310_e82374);
        let assign49310_e82377: f64 = (assign49310_e82375 * locals.var_ehlidfactor);
        (assign49310_e82377, ((((locals.var_iendif_dn3 * assign49310_e82374) + (locals.var_iendif * locals.var_expvbdnvtm_dn3)) * locals.var_ehlidfactor) + (assign49310_e82375 * locals.var_ehlidfactor_dn3)), ((((locals.var_iendif_dn4 * assign49310_e82374) + (locals.var_iendif * locals.var_expvbdnvtm_dn4)) * locals.var_ehlidfactor) + (assign49310_e82375 * locals.var_ehlidfactor_dn4)), ((((locals.var_iendif_dn5 * assign49310_e82374) + (locals.var_iendif * locals.var_expvbdnvtm_dn5)) * locals.var_ehlidfactor) + (assign49310_e82375 * locals.var_ehlidfactor_dn5)), ((((locals.var_iendif_dn6 * assign49310_e82374) + (locals.var_iendif * locals.var_expvbdnvtm_dn6)) * locals.var_ehlidfactor) + (assign49310_e82375 * locals.var_ehlidfactor_dn6)), ((((locals.var_iendif_dn7 * assign49310_e82374) + (locals.var_iendif * locals.var_expvbdnvtm_dn7)) * locals.var_ehlidfactor) + (assign49310_e82375 * locals.var_ehlidfactor_dn7)), ((((locals.var_iendif_dn8 * assign49310_e82374) + (locals.var_iendif * locals.var_expvbdnvtm_dn8)) * locals.var_ehlidfactor) + (assign49310_e82375 * locals.var_ehlidfactor_dn8)), ((((locals.var_iendif_dn9 * assign49310_e82374) + (locals.var_iendif * locals.var_expvbdnvtm_dn9)) * locals.var_ehlidfactor) + (assign49310_e82375 * locals.var_ehlidfactor_dn9)), ((((locals.var_iendif_dn10 * assign49310_e82374) + (locals.var_iendif * locals.var_expvbdnvtm_dn10)) * locals.var_ehlidfactor) + (assign49310_e82375 * locals.var_ehlidfactor_dn10)), ((((locals.var_iendif_dn11 * assign49310_e82374) + (locals.var_iendif * locals.var_expvbdnvtm_dn11)) * locals.var_ehlidfactor) + (assign49310_e82375 * locals.var_ehlidfactor_dn11)),)
    } else {
        (locals.var_ibddif, locals.var_ibddif_dn3, locals.var_ibddif_dn4, locals.var_ibddif_dn5, locals.var_ibddif_dn6, locals.var_ibddif_dn7, locals.var_ibddif_dn8, locals.var_ibddif_dn9, locals.var_ibddif_dn10, locals.var_ibddif_dn11,)
    }
};
        locals.var_ibddif = assign49310_e82379;
        locals.var_ibddif_dn3 = assign49310_e82379_d_n3;
        locals.var_ibddif_dn4 = assign49310_e82379_d_n4;
        locals.var_ibddif_dn5 = assign49310_e82379_d_n5;
        locals.var_ibddif_dn6 = assign49310_e82379_d_n6;
        locals.var_ibddif_dn7 = assign49310_e82379_d_n7;
        locals.var_ibddif_dn8 = assign49310_e82379_d_n8;
        locals.var_ibddif_dn9 = assign49310_e82379_d_n9;
        locals.var_ibddif_dn10 = assign49310_e82379_d_n10;
        locals.var_ibddif_dn11 = assign49310_e82379_d_n11;

        let (assign49320_e82391,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) {
        let assign49320_e82388: f64 = (locals.var_aely_i * locals.var_leff);
        let assign49320_e82389: f64 = (locals.var_vabjt_i + assign49320_e82388);
        (assign49320_e82389,)
    } else {
        (locals.var_vearly,)
    }
};
        locals.var_vearly = assign49320_e82391;

        let assign49330_e82394: f64 = if locals.var_vearly < 1.0 { 1.0 } else { 0.0 };
        locals.var_guard753 = assign49330_e82394;

        let (assign49340_e82404,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) && (locals.var_guard753 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_vearly,)
    }
};
        locals.var_vearly = assign49340_e82404;

        let assign49350_e82407: f64 = if p.p554 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard754 = assign49350_e82407;

        let (assign49360_e82417, assign49360_e82417_d_n3, assign49360_e82417_d_n4, assign49360_e82417_d_n5, assign49360_e82417_d_n6, assign49360_e82417_d_n7, assign49360_e82417_d_n8, assign49360_e82417_d_n9, assign49360_e82417_d_n10, assign49360_e82417_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) && (locals.var_guard754 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ic, locals.var_ic_dn3, locals.var_ic_dn4, locals.var_ic_dn5, locals.var_ic_dn6, locals.var_ic_dn7, locals.var_ic_dn8, locals.var_ic_dn9, locals.var_ic_dn10, locals.var_ic_dn11,)
    }
};
        locals.var_ic = assign49360_e82417;
        locals.var_ic_dn3 = assign49360_e82417_d_n3;
        locals.var_ic_dn4 = assign49360_e82417_d_n4;
        locals.var_ic_dn5 = assign49360_e82417_d_n5;
        locals.var_ic_dn6 = assign49360_e82417_d_n6;
        locals.var_ic_dn7 = assign49360_e82417_d_n7;
        locals.var_ic_dn8 = assign49360_e82417_d_n8;
        locals.var_ic_dn9 = assign49360_e82417_d_n9;
        locals.var_ic_dn10 = assign49360_e82417_d_n10;
        locals.var_ic_dn11 = assign49360_e82417_d_n11;

        let (assign49370_e82434, assign49370_e82434_d_n3, assign49370_e82434_d_n4, assign49370_e82434_d_n5, assign49370_e82434_d_n6, assign49370_e82434_d_n7, assign49370_e82434_d_n8, assign49370_e82434_d_n9, assign49370_e82434_d_n10, assign49370_e82434_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) && (locals.var_guard754 == 0.0)) {
        let assign49370_e82429: f64 = (locals.var_vbs_jct + locals.var_vbd_jct);
        let assign49370_e82431: f64 = (assign49370_e82429 / locals.var_vearly);
        let assign49370_e82432: f64 = (1.0 + assign49370_e82431);
        (assign49370_e82432, 0.0, 0.0, 0.0, (locals.var_vbd_jct_dn6 / locals.var_vearly), (locals.var_vbs_jct_dn7 / locals.var_vearly), 0.0, 0.0, ((locals.var_vbs_jct_dn10 + locals.var_vbd_jct_dn10) / locals.var_vearly), 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign49370_e82434;
        locals.var_t0_dn3 = assign49370_e82434_d_n3;
        locals.var_t0_dn4 = assign49370_e82434_d_n4;
        locals.var_t0_dn5 = assign49370_e82434_d_n5;
        locals.var_t0_dn6 = assign49370_e82434_d_n6;
        locals.var_t0_dn7 = assign49370_e82434_d_n7;
        locals.var_t0_dn8 = assign49370_e82434_d_n8;
        locals.var_t0_dn9 = assign49370_e82434_d_n9;
        locals.var_t0_dn10 = assign49370_e82434_d_n10;
        locals.var_t0_dn11 = assign49370_e82434_d_n11;

        let (assign49380_e82447, assign49380_e82447_d_n3, assign49380_e82447_d_n4, assign49380_e82447_d_n5, assign49380_e82447_d_n6, assign49380_e82447_d_n7, assign49380_e82447_d_n8, assign49380_e82447_d_n9, assign49380_e82447_d_n10, assign49380_e82447_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) && (locals.var_guard754 == 0.0)) {
        let assign49380_e82445: f64 = (locals.var_ehlis + locals.var_ehlid);
        (assign49380_e82445, (locals.var_ehlis_dn3 + locals.var_ehlid_dn3), (locals.var_ehlis_dn4 + locals.var_ehlid_dn4), (locals.var_ehlis_dn5 + locals.var_ehlid_dn5), (locals.var_ehlis_dn6 + locals.var_ehlid_dn6), (locals.var_ehlis_dn7 + locals.var_ehlid_dn7), (locals.var_ehlis_dn8 + locals.var_ehlid_dn8), (locals.var_ehlis_dn9 + locals.var_ehlid_dn9), (locals.var_ehlis_dn10 + locals.var_ehlid_dn10), (locals.var_ehlis_dn11 + locals.var_ehlid_dn11),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign49380_e82447;
        locals.var_t1_dn3 = assign49380_e82447_d_n3;
        locals.var_t1_dn4 = assign49380_e82447_d_n4;
        locals.var_t1_dn5 = assign49380_e82447_d_n5;
        locals.var_t1_dn6 = assign49380_e82447_d_n6;
        locals.var_t1_dn7 = assign49380_e82447_d_n7;
        locals.var_t1_dn8 = assign49380_e82447_d_n8;
        locals.var_t1_dn9 = assign49380_e82447_d_n9;
        locals.var_t1_dn10 = assign49380_e82447_d_n10;
        locals.var_t1_dn11 = assign49380_e82447_d_n11;

        let (assign49390_e82465, assign49390_e82465_d_n3, assign49390_e82465_d_n4, assign49390_e82465_d_n5, assign49390_e82465_d_n6, assign49390_e82465_d_n7, assign49390_e82465_d_n8, assign49390_e82465_d_n9, assign49390_e82465_d_n10, assign49390_e82465_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) && (locals.var_guard754 == 0.0)) {
        let assign49390_e82458: f64 = (locals.var_t0 * locals.var_t0);
        let assign49390_e82461: f64 = (4.0 * locals.var_t1);
        let assign49390_e82462: f64 = (assign49390_e82458 + assign49390_e82461);
        let assign49390_e82463: f64 = (assign49390_e82462).sqrt();
        (assign49390_e82463, ((((locals.var_t0_dn3 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn3)) + (4.0 * locals.var_t1_dn3)) / (2.0 * assign49390_e82463)), ((((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)) + (4.0 * locals.var_t1_dn4)) / (2.0 * assign49390_e82463)), ((((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)) + (4.0 * locals.var_t1_dn5)) / (2.0 * assign49390_e82463)), ((((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)) + (4.0 * locals.var_t1_dn6)) / (2.0 * assign49390_e82463)), ((((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)) + (4.0 * locals.var_t1_dn7)) / (2.0 * assign49390_e82463)), ((((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)) + (4.0 * locals.var_t1_dn8)) / (2.0 * assign49390_e82463)), ((((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)) + (4.0 * locals.var_t1_dn9)) / (2.0 * assign49390_e82463)), ((((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)) + (4.0 * locals.var_t1_dn10)) / (2.0 * assign49390_e82463)), ((((locals.var_t0_dn11 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn11)) + (4.0 * locals.var_t1_dn11)) / (2.0 * assign49390_e82463)),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign49390_e82465;
        locals.var_t3_dn3 = assign49390_e82465_d_n3;
        locals.var_t3_dn4 = assign49390_e82465_d_n4;
        locals.var_t3_dn5 = assign49390_e82465_d_n5;
        locals.var_t3_dn6 = assign49390_e82465_d_n6;
        locals.var_t3_dn7 = assign49390_e82465_d_n7;
        locals.var_t3_dn8 = assign49390_e82465_d_n8;
        locals.var_t3_dn9 = assign49390_e82465_d_n9;
        locals.var_t3_dn10 = assign49390_e82465_d_n10;
        locals.var_t3_dn11 = assign49390_e82465_d_n11;

        let (assign49400_e82480, assign49400_e82480_d_n3, assign49400_e82480_d_n4, assign49400_e82480_d_n5, assign49400_e82480_d_n6, assign49400_e82480_d_n7, assign49400_e82480_d_n8, assign49400_e82480_d_n9, assign49400_e82480_d_n10, assign49400_e82480_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) && (locals.var_guard754 == 0.0)) {
        let assign49400_e82476: f64 = (locals.var_t0 + locals.var_t3);
        let assign49400_e82478: f64 = (assign49400_e82476 / 2.0);
        (assign49400_e82478, ((locals.var_t0_dn3 + locals.var_t3_dn3) / 2.0), ((locals.var_t0_dn4 + locals.var_t3_dn4) / 2.0), ((locals.var_t0_dn5 + locals.var_t3_dn5) / 2.0), ((locals.var_t0_dn6 + locals.var_t3_dn6) / 2.0), ((locals.var_t0_dn7 + locals.var_t3_dn7) / 2.0), ((locals.var_t0_dn8 + locals.var_t3_dn8) / 2.0), ((locals.var_t0_dn9 + locals.var_t3_dn9) / 2.0), ((locals.var_t0_dn10 + locals.var_t3_dn10) / 2.0), ((locals.var_t0_dn11 + locals.var_t3_dn11) / 2.0),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign49400_e82480;
        locals.var_t2_dn3 = assign49400_e82480_d_n3;
        locals.var_t2_dn4 = assign49400_e82480_d_n4;
        locals.var_t2_dn5 = assign49400_e82480_d_n5;
        locals.var_t2_dn6 = assign49400_e82480_d_n6;
        locals.var_t2_dn7 = assign49400_e82480_d_n7;
        locals.var_t2_dn8 = assign49400_e82480_d_n8;
        locals.var_t2_dn9 = assign49400_e82480_d_n9;
        locals.var_t2_dn10 = assign49400_e82480_d_n10;
        locals.var_t2_dn11 = assign49400_e82480_d_n11;

        let assign49410_e82483: f64 = if locals.var_t2 < 0.1 { 1.0 } else { 0.0 };
        locals.var_guard755 = assign49410_e82483;

        let (assign49420_e82496, assign49420_e82496_d_n3, assign49420_e82496_d_n4, assign49420_e82496_d_n5, assign49420_e82496_d_n6, assign49420_e82496_d_n7, assign49420_e82496_d_n8, assign49420_e82496_d_n9, assign49420_e82496_d_n10, assign49420_e82496_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) && (locals.var_guard754 == 0.0)) && (locals.var_guard755 != 0.0)) {
        (10.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_e2ndfactor, locals.var_e2ndfactor_dn3, locals.var_e2ndfactor_dn4, locals.var_e2ndfactor_dn5, locals.var_e2ndfactor_dn6, locals.var_e2ndfactor_dn7, locals.var_e2ndfactor_dn8, locals.var_e2ndfactor_dn9, locals.var_e2ndfactor_dn10, locals.var_e2ndfactor_dn11,)
    }
};
        locals.var_e2ndfactor = assign49420_e82496;
        locals.var_e2ndfactor_dn3 = assign49420_e82496_d_n3;
        locals.var_e2ndfactor_dn4 = assign49420_e82496_d_n4;
        locals.var_e2ndfactor_dn5 = assign49420_e82496_d_n5;
        locals.var_e2ndfactor_dn6 = assign49420_e82496_d_n6;
        locals.var_e2ndfactor_dn7 = assign49420_e82496_d_n7;
        locals.var_e2ndfactor_dn8 = assign49420_e82496_d_n8;
        locals.var_e2ndfactor_dn9 = assign49420_e82496_d_n9;
        locals.var_e2ndfactor_dn10 = assign49420_e82496_d_n10;
        locals.var_e2ndfactor_dn11 = assign49420_e82496_d_n11;

        let (assign49430_e82512, assign49430_e82512_d_n3, assign49430_e82512_d_n4, assign49430_e82512_d_n5, assign49430_e82512_d_n6, assign49430_e82512_d_n7, assign49430_e82512_d_n8, assign49430_e82512_d_n9, assign49430_e82512_d_n10, assign49430_e82512_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) && (locals.var_guard754 == 0.0)) && (locals.var_guard755 == 0.0)) {
        let assign49430_e82510: f64 = (1.0 / locals.var_t2);
        (assign49430_e82510, (-(locals.var_t2_dn3 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn4 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn5 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn6 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn7 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn8 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn9 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn10 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn11 / (locals.var_t2 * locals.var_t2))),)
    } else {
        (locals.var_e2ndfactor, locals.var_e2ndfactor_dn3, locals.var_e2ndfactor_dn4, locals.var_e2ndfactor_dn5, locals.var_e2ndfactor_dn6, locals.var_e2ndfactor_dn7, locals.var_e2ndfactor_dn8, locals.var_e2ndfactor_dn9, locals.var_e2ndfactor_dn10, locals.var_e2ndfactor_dn11,)
    }
};
        locals.var_e2ndfactor = assign49430_e82512;
        locals.var_e2ndfactor_dn3 = assign49430_e82512_d_n3;
        locals.var_e2ndfactor_dn4 = assign49430_e82512_d_n4;
        locals.var_e2ndfactor_dn5 = assign49430_e82512_d_n5;
        locals.var_e2ndfactor_dn6 = assign49430_e82512_d_n6;
        locals.var_e2ndfactor_dn7 = assign49430_e82512_d_n7;
        locals.var_e2ndfactor_dn8 = assign49430_e82512_d_n8;
        locals.var_e2ndfactor_dn9 = assign49430_e82512_d_n9;
        locals.var_e2ndfactor_dn10 = assign49430_e82512_d_n10;
        locals.var_e2ndfactor_dn11 = assign49430_e82512_d_n11;

        let (assign49440_e82525, assign49440_e82525_d_n3, assign49440_e82525_d_n4, assign49440_e82525_d_n5, assign49440_e82525_d_n6, assign49440_e82525_d_n7, assign49440_e82525_d_n8, assign49440_e82525_d_n9, assign49440_e82525_d_n10, assign49440_e82525_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) && (locals.var_guard754 == 0.0)) {
        let assign49440_e82523: f64 = (locals.var_alphabjt * locals.var_ien);
        (assign49440_e82523, ((locals.var_alphabjt_dn3 * locals.var_ien) + (locals.var_alphabjt * locals.var_ien_dn3)), ((locals.var_alphabjt_dn4 * locals.var_ien) + (locals.var_alphabjt * locals.var_ien_dn4)), ((locals.var_alphabjt_dn5 * locals.var_ien) + (locals.var_alphabjt * locals.var_ien_dn5)), ((locals.var_alphabjt_dn6 * locals.var_ien) + (locals.var_alphabjt * locals.var_ien_dn6)), ((locals.var_alphabjt_dn7 * locals.var_ien) + (locals.var_alphabjt * locals.var_ien_dn7)), ((locals.var_alphabjt_dn8 * locals.var_ien) + (locals.var_alphabjt * locals.var_ien_dn8)), ((locals.var_alphabjt_dn9 * locals.var_ien) + (locals.var_alphabjt * locals.var_ien_dn9)), ((locals.var_alphabjt_dn10 * locals.var_ien) + (locals.var_alphabjt * locals.var_ien_dn10)), ((locals.var_alphabjt_dn11 * locals.var_ien) + (locals.var_alphabjt * locals.var_ien_dn11)),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign49440_e82525;
        locals.var_t0_dn3 = assign49440_e82525_d_n3;
        locals.var_t0_dn4 = assign49440_e82525_d_n4;
        locals.var_t0_dn5 = assign49440_e82525_d_n5;
        locals.var_t0_dn6 = assign49440_e82525_d_n6;
        locals.var_t0_dn7 = assign49440_e82525_d_n7;
        locals.var_t0_dn8 = assign49440_e82525_d_n8;
        locals.var_t0_dn9 = assign49440_e82525_d_n9;
        locals.var_t0_dn10 = assign49440_e82525_d_n10;
        locals.var_t0_dn11 = assign49440_e82525_d_n11;

        let (assign49450_e82544, assign49450_e82544_d_n3, assign49450_e82544_d_n4, assign49450_e82544_d_n5, assign49450_e82544_d_n6, assign49450_e82544_d_n7, assign49450_e82544_d_n8, assign49450_e82544_d_n9, assign49450_e82544_d_n10, assign49450_e82544_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) && (locals.var_guard754 == 0.0)) {
        let assign49450_e82536: f64 = (p.p2 * locals.var_t0);
        let assign49450_e82539: f64 = (locals.var_expvbsnvtm - locals.var_expvbdnvtm);
        let assign49450_e82540: f64 = (assign49450_e82536 * assign49450_e82539);
        let assign49450_e82542: f64 = (assign49450_e82540 * locals.var_e2ndfactor);
        (assign49450_e82542, (((((p.p2 * locals.var_t0_dn3) * assign49450_e82539) + (assign49450_e82536 * (locals.var_expvbsnvtm_dn3 - locals.var_expvbdnvtm_dn3))) * locals.var_e2ndfactor) + (assign49450_e82540 * locals.var_e2ndfactor_dn3)), (((((p.p2 * locals.var_t0_dn4) * assign49450_e82539) + (assign49450_e82536 * (locals.var_expvbsnvtm_dn4 - locals.var_expvbdnvtm_dn4))) * locals.var_e2ndfactor) + (assign49450_e82540 * locals.var_e2ndfactor_dn4)), (((((p.p2 * locals.var_t0_dn5) * assign49450_e82539) + (assign49450_e82536 * (locals.var_expvbsnvtm_dn5 - locals.var_expvbdnvtm_dn5))) * locals.var_e2ndfactor) + (assign49450_e82540 * locals.var_e2ndfactor_dn5)), (((((p.p2 * locals.var_t0_dn6) * assign49450_e82539) + (assign49450_e82536 * (locals.var_expvbsnvtm_dn6 - locals.var_expvbdnvtm_dn6))) * locals.var_e2ndfactor) + (assign49450_e82540 * locals.var_e2ndfactor_dn6)), (((((p.p2 * locals.var_t0_dn7) * assign49450_e82539) + (assign49450_e82536 * (locals.var_expvbsnvtm_dn7 - locals.var_expvbdnvtm_dn7))) * locals.var_e2ndfactor) + (assign49450_e82540 * locals.var_e2ndfactor_dn7)), (((((p.p2 * locals.var_t0_dn8) * assign49450_e82539) + (assign49450_e82536 * (locals.var_expvbsnvtm_dn8 - locals.var_expvbdnvtm_dn8))) * locals.var_e2ndfactor) + (assign49450_e82540 * locals.var_e2ndfactor_dn8)), (((((p.p2 * locals.var_t0_dn9) * assign49450_e82539) + (assign49450_e82536 * (locals.var_expvbsnvtm_dn9 - locals.var_expvbdnvtm_dn9))) * locals.var_e2ndfactor) + (assign49450_e82540 * locals.var_e2ndfactor_dn9)), (((((p.p2 * locals.var_t0_dn10) * assign49450_e82539) + (assign49450_e82536 * (locals.var_expvbsnvtm_dn10 - locals.var_expvbdnvtm_dn10))) * locals.var_e2ndfactor) + (assign49450_e82540 * locals.var_e2ndfactor_dn10)), (((((p.p2 * locals.var_t0_dn11) * assign49450_e82539) + (assign49450_e82536 * (locals.var_expvbsnvtm_dn11 - locals.var_expvbdnvtm_dn11))) * locals.var_e2ndfactor) + (assign49450_e82540 * locals.var_e2ndfactor_dn11)),)
    } else {
        (locals.var_ic, locals.var_ic_dn3, locals.var_ic_dn4, locals.var_ic_dn5, locals.var_ic_dn6, locals.var_ic_dn7, locals.var_ic_dn8, locals.var_ic_dn9, locals.var_ic_dn10, locals.var_ic_dn11,)
    }
};
        locals.var_ic = assign49450_e82544;
        locals.var_ic_dn3 = assign49450_e82544_d_n3;
        locals.var_ic_dn4 = assign49450_e82544_d_n4;
        locals.var_ic_dn5 = assign49450_e82544_d_n5;
        locals.var_ic_dn6 = assign49450_e82544_d_n6;
        locals.var_ic_dn7 = assign49450_e82544_d_n7;
        locals.var_ic_dn8 = assign49450_e82544_d_n8;
        locals.var_ic_dn9 = assign49450_e82544_d_n9;
        locals.var_ic_dn10 = assign49450_e82544_d_n10;
        locals.var_ic_dn11 = assign49450_e82544_d_n11;

        let assign49460_e82551: f64 = if ((locals.var_istun_i == 0.0) && (locals.var_idtun_i == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard756 = assign49460_e82551;

        let (assign49470_e82558, assign49470_e82558_d_n3, assign49470_e82558_d_n4, assign49470_e82558_d_n5, assign49470_e82558_d_n6, assign49470_e82558_d_n7, assign49470_e82558_d_n8, assign49470_e82558_d_n9, assign49470_e82558_d_n10, assign49470_e82558_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard756 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ibs4, locals.var_ibs4_dn3, locals.var_ibs4_dn4, locals.var_ibs4_dn5, locals.var_ibs4_dn6, locals.var_ibs4_dn7, locals.var_ibs4_dn8, locals.var_ibs4_dn9, locals.var_ibs4_dn10, locals.var_ibs4_dn11,)
    }
};
        locals.var_ibs4 = assign49470_e82558;
        locals.var_ibs4_dn3 = assign49470_e82558_d_n3;
        locals.var_ibs4_dn4 = assign49470_e82558_d_n4;
        locals.var_ibs4_dn5 = assign49470_e82558_d_n5;
        locals.var_ibs4_dn6 = assign49470_e82558_d_n6;
        locals.var_ibs4_dn7 = assign49470_e82558_d_n7;
        locals.var_ibs4_dn8 = assign49470_e82558_d_n8;
        locals.var_ibs4_dn9 = assign49470_e82558_d_n9;
        locals.var_ibs4_dn10 = assign49470_e82558_d_n10;
        locals.var_ibs4_dn11 = assign49470_e82558_d_n11;

        let (assign49480_e82565, assign49480_e82565_d_n3, assign49480_e82565_d_n4, assign49480_e82565_d_n5, assign49480_e82565_d_n6, assign49480_e82565_d_n7, assign49480_e82565_d_n8, assign49480_e82565_d_n9, assign49480_e82565_d_n10, assign49480_e82565_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard756 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ibd4, locals.var_ibd4_dn3, locals.var_ibd4_dn4, locals.var_ibd4_dn5, locals.var_ibd4_dn6, locals.var_ibd4_dn7, locals.var_ibd4_dn8, locals.var_ibd4_dn9, locals.var_ibd4_dn10, locals.var_ibd4_dn11,)
    }
};
        locals.var_ibd4 = assign49480_e82565;
        locals.var_ibd4_dn3 = assign49480_e82565_d_n3;
        locals.var_ibd4_dn4 = assign49480_e82565_d_n4;
        locals.var_ibd4_dn5 = assign49480_e82565_d_n5;
        locals.var_ibd4_dn6 = assign49480_e82565_d_n6;
        locals.var_ibd4_dn7 = assign49480_e82565_d_n7;
        locals.var_ibd4_dn8 = assign49480_e82565_d_n8;
        locals.var_ibd4_dn9 = assign49480_e82565_d_n9;
        locals.var_ibd4_dn10 = assign49480_e82565_d_n10;
        locals.var_ibd4_dn11 = assign49480_e82565_d_n11;

        let (assign49490_e82577, assign49490_e82577_d_n3, assign49490_e82577_d_n4, assign49490_e82577_d_n5, assign49490_e82577_d_n6, assign49490_e82577_d_n7, assign49490_e82577_d_n8, assign49490_e82577_d_n9, assign49490_e82577_d_n10, assign49490_e82577_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard756 == 0.0)) {
        let assign49490_e82574: f64 = (locals.var_tratio - 1.0);
        let assign49490_e82575: f64 = (locals.var_xtun_i * assign49490_e82574);
        (assign49490_e82575, 0.0, (locals.var_xtun_i * locals.var_tratio_dn4), (locals.var_xtun_i * locals.var_tratio_dn5), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t7, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11,)
    }
};
        locals.var_t7 = assign49490_e82577;
        locals.var_t7_dn3 = assign49490_e82577_d_n3;
        locals.var_t7_dn4 = assign49490_e82577_d_n4;
        locals.var_t7_dn5 = assign49490_e82577_d_n5;
        locals.var_t7_dn6 = assign49490_e82577_d_n6;
        locals.var_t7_dn7 = assign49490_e82577_d_n7;
        locals.var_t7_dn8 = assign49490_e82577_d_n8;
        locals.var_t7_dn9 = assign49490_e82577_d_n9;
        locals.var_t7_dn10 = assign49490_e82577_d_n10;
        locals.var_t7_dn11 = assign49490_e82577_d_n11;

        let (assign49500_e82586, assign49500_e82586_d_n3, assign49500_e82586_d_n4, assign49500_e82586_d_n5, assign49500_e82586_d_n6, assign49500_e82586_d_n7, assign49500_e82586_d_n8, assign49500_e82586_d_n9, assign49500_e82586_d_n10, assign49500_e82586_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard756 == 0.0)) {
        let assign49500_e82584: f64 = { let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign49500_e82584, ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn3), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn4), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn5), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn6), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn7), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn8), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn9), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn10), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn11),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign49500_e82586;
        locals.var_t0_dn3 = assign49500_e82586_d_n3;
        locals.var_t0_dn4 = assign49500_e82586_d_n4;
        locals.var_t0_dn5 = assign49500_e82586_d_n5;
        locals.var_t0_dn6 = assign49500_e82586_d_n6;
        locals.var_t0_dn7 = assign49500_e82586_d_n7;
        locals.var_t0_dn8 = assign49500_e82586_d_n8;
        locals.var_t0_dn9 = assign49500_e82586_d_n9;
        locals.var_t0_dn10 = assign49500_e82586_d_n10;
        locals.var_t0_dn11 = assign49500_e82586_d_n11;

        let (assign49510_e82596, assign49510_e82596_d_n3, assign49510_e82596_d_n4, assign49510_e82596_d_n5, assign49510_e82596_d_n6, assign49510_e82596_d_n7, assign49510_e82596_d_n8, assign49510_e82596_d_n9, assign49510_e82596_d_n10, assign49510_e82596_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard756 == 0.0)) {
        let assign49510_e82594: f64 = (locals.var_istun_i * locals.var_t0);
        (assign49510_e82594, (locals.var_istun_i * locals.var_t0_dn3), (locals.var_istun_i * locals.var_t0_dn4), (locals.var_istun_i * locals.var_t0_dn5), (locals.var_istun_i * locals.var_t0_dn6), (locals.var_istun_i * locals.var_t0_dn7), (locals.var_istun_i * locals.var_t0_dn8), (locals.var_istun_i * locals.var_t0_dn9), (locals.var_istun_i * locals.var_t0_dn10), (locals.var_istun_i * locals.var_t0_dn11),)
    } else {
        (locals.var_jtuns, locals.var_jtuns_dn3, locals.var_jtuns_dn4, locals.var_jtuns_dn5, locals.var_jtuns_dn6, locals.var_jtuns_dn7, locals.var_jtuns_dn8, locals.var_jtuns_dn9, locals.var_jtuns_dn10, locals.var_jtuns_dn11,)
    }
};
        locals.var_jtuns = assign49510_e82596;
        locals.var_jtuns_dn3 = assign49510_e82596_d_n3;
        locals.var_jtuns_dn4 = assign49510_e82596_d_n4;
        locals.var_jtuns_dn5 = assign49510_e82596_d_n5;
        locals.var_jtuns_dn6 = assign49510_e82596_d_n6;
        locals.var_jtuns_dn7 = assign49510_e82596_d_n7;
        locals.var_jtuns_dn8 = assign49510_e82596_d_n8;
        locals.var_jtuns_dn9 = assign49510_e82596_d_n9;
        locals.var_jtuns_dn10 = assign49510_e82596_d_n10;
        locals.var_jtuns_dn11 = assign49510_e82596_d_n11;

        let (assign49520_e82608, assign49520_e82608_d_n3, assign49520_e82608_d_n4, assign49520_e82608_d_n5, assign49520_e82608_d_n6, assign49520_e82608_d_n7, assign49520_e82608_d_n8, assign49520_e82608_d_n9, assign49520_e82608_d_n10, assign49520_e82608_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard756 == 0.0)) {
        let assign49520_e82605: f64 = (locals.var_tratio - 1.0);
        let assign49520_e82606: f64 = (locals.var_xtund_i * assign49520_e82605);
        (assign49520_e82606, 0.0, (locals.var_xtund_i * locals.var_tratio_dn4), (locals.var_xtund_i * locals.var_tratio_dn5), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t7, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11,)
    }
};
        locals.var_t7 = assign49520_e82608;
        locals.var_t7_dn3 = assign49520_e82608_d_n3;
        locals.var_t7_dn4 = assign49520_e82608_d_n4;
        locals.var_t7_dn5 = assign49520_e82608_d_n5;
        locals.var_t7_dn6 = assign49520_e82608_d_n6;
        locals.var_t7_dn7 = assign49520_e82608_d_n7;
        locals.var_t7_dn8 = assign49520_e82608_d_n8;
        locals.var_t7_dn9 = assign49520_e82608_d_n9;
        locals.var_t7_dn10 = assign49520_e82608_d_n10;
        locals.var_t7_dn11 = assign49520_e82608_d_n11;

        let (assign49530_e82617, assign49530_e82617_d_n3, assign49530_e82617_d_n4, assign49530_e82617_d_n5, assign49530_e82617_d_n6, assign49530_e82617_d_n7, assign49530_e82617_d_n8, assign49530_e82617_d_n9, assign49530_e82617_d_n10, assign49530_e82617_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard756 == 0.0)) {
        let assign49530_e82615: f64 = { let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign49530_e82615, ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn3), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn4), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn5), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn6), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn7), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn8), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn9), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn10), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn11),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign49530_e82617;
        locals.var_t0_dn3 = assign49530_e82617_d_n3;
        locals.var_t0_dn4 = assign49530_e82617_d_n4;
        locals.var_t0_dn5 = assign49530_e82617_d_n5;
        locals.var_t0_dn6 = assign49530_e82617_d_n6;
        locals.var_t0_dn7 = assign49530_e82617_d_n7;
        locals.var_t0_dn8 = assign49530_e82617_d_n8;
        locals.var_t0_dn9 = assign49530_e82617_d_n9;
        locals.var_t0_dn10 = assign49530_e82617_d_n10;
        locals.var_t0_dn11 = assign49530_e82617_d_n11;

        let (assign49540_e82627, assign49540_e82627_d_n3, assign49540_e82627_d_n4, assign49540_e82627_d_n5, assign49540_e82627_d_n6, assign49540_e82627_d_n7, assign49540_e82627_d_n8, assign49540_e82627_d_n9, assign49540_e82627_d_n10, assign49540_e82627_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard756 == 0.0)) {
        let assign49540_e82625: f64 = (locals.var_idtun_i * locals.var_t0);
        (assign49540_e82625, (locals.var_idtun_i * locals.var_t0_dn3), (locals.var_idtun_i * locals.var_t0_dn4), (locals.var_idtun_i * locals.var_t0_dn5), (locals.var_idtun_i * locals.var_t0_dn6), (locals.var_idtun_i * locals.var_t0_dn7), (locals.var_idtun_i * locals.var_t0_dn8), (locals.var_idtun_i * locals.var_t0_dn9), (locals.var_idtun_i * locals.var_t0_dn10), (locals.var_idtun_i * locals.var_t0_dn11),)
    } else {
        (locals.var_jtund, locals.var_jtund_dn3, locals.var_jtund_dn4, locals.var_jtund_dn5, locals.var_jtund_dn6, locals.var_jtund_dn7, locals.var_jtund_dn8, locals.var_jtund_dn9, locals.var_jtund_dn10, locals.var_jtund_dn11,)
    }
};
        locals.var_jtund = assign49540_e82627;
        locals.var_jtund_dn3 = assign49540_e82627_d_n3;
        locals.var_jtund_dn4 = assign49540_e82627_d_n4;
        locals.var_jtund_dn5 = assign49540_e82627_d_n5;
        locals.var_jtund_dn6 = assign49540_e82627_d_n6;
        locals.var_jtund_dn7 = assign49540_e82627_d_n7;
        locals.var_jtund_dn8 = assign49540_e82627_d_n8;
        locals.var_jtund_dn9 = assign49540_e82627_d_n9;
        locals.var_jtund_dn10 = assign49540_e82627_d_n10;
        locals.var_jtund_dn11 = assign49540_e82627_d_n11;

        let (assign49550_e82637, assign49550_e82637_d_n4, assign49550_e82637_d_n5,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard756 == 0.0)) {
        let assign49550_e82635: f64 = (p.p925 * locals.var_ntun_i);
        (assign49550_e82635, 0.0, 0.0,)
    } else {
        (locals.var_nvtm2, locals.var_nvtm2_dn4, locals.var_nvtm2_dn5,)
    }
};
        locals.var_nvtm2 = assign49550_e82637;
        locals.var_nvtm2_dn4 = assign49550_e82637_d_n4;
        locals.var_nvtm2_dn5 = assign49550_e82637_d_n5;

        let assign49560_e82640: f64 = (locals.var_vtun0_i - locals.var_vbs_jct);
        let assign49560_e82642: f64 = if assign49560_e82640 < 0.001 { 1.0 } else { 0.0 };
        locals.var_guard757 = assign49560_e82642;

        let (assign49570_e82652, assign49570_e82652_d_n3, assign49570_e82652_d_n4, assign49570_e82652_d_n5, assign49570_e82652_d_n6, assign49570_e82652_d_n7, assign49570_e82652_d_n8, assign49570_e82652_d_n9, assign49570_e82652_d_n10, assign49570_e82652_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard756 == 0.0)) && (locals.var_guard757 != 0.0)) {
        (1000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign49570_e82652;
        locals.var_t1_dn3 = assign49570_e82652_d_n3;
        locals.var_t1_dn4 = assign49570_e82652_d_n4;
        locals.var_t1_dn5 = assign49570_e82652_d_n5;
        locals.var_t1_dn6 = assign49570_e82652_d_n6;
        locals.var_t1_dn7 = assign49570_e82652_d_n7;
        locals.var_t1_dn8 = assign49570_e82652_d_n8;
        locals.var_t1_dn9 = assign49570_e82652_d_n9;
        locals.var_t1_dn10 = assign49570_e82652_d_n10;
        locals.var_t1_dn11 = assign49570_e82652_d_n11;

        let (assign49580_e82669, assign49580_e82669_d_n3, assign49580_e82669_d_n4, assign49580_e82669_d_n5, assign49580_e82669_d_n6, assign49580_e82669_d_n7, assign49580_e82669_d_n8, assign49580_e82669_d_n9, assign49580_e82669_d_n10, assign49580_e82669_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard756 == 0.0)) && (locals.var_guard757 != 0.0)) {
        let assign49580_e82661: f64 = (-locals.var_vbs_jct);
        let assign49580_e82663: f64 = (assign49580_e82661 / locals.var_nvtm2);
        let assign49580_e82665: f64 = (assign49580_e82663 * locals.var_vtun0_i);
        let assign49580_e82667: f64 = (assign49580_e82665 * locals.var_t1);
        (assign49580_e82667, (assign49580_e82665 * locals.var_t1_dn3), ((((-((assign49580_e82661 * locals.var_nvtm2_dn4) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_vtun0_i) * locals.var_t1) + (assign49580_e82665 * locals.var_t1_dn4)), ((((-((assign49580_e82661 * locals.var_nvtm2_dn5) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_vtun0_i) * locals.var_t1) + (assign49580_e82665 * locals.var_t1_dn5)), (assign49580_e82665 * locals.var_t1_dn6), (((((-locals.var_vbs_jct_dn7) / locals.var_nvtm2) * locals.var_vtun0_i) * locals.var_t1) + (assign49580_e82665 * locals.var_t1_dn7)), (assign49580_e82665 * locals.var_t1_dn8), (assign49580_e82665 * locals.var_t1_dn9), (((((-locals.var_vbs_jct_dn10) / locals.var_nvtm2) * locals.var_vtun0_i) * locals.var_t1) + (assign49580_e82665 * locals.var_t1_dn10)), (assign49580_e82665 * locals.var_t1_dn11),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign49580_e82669;
        locals.var_t0_dn3 = assign49580_e82669_d_n3;
        locals.var_t0_dn4 = assign49580_e82669_d_n4;
        locals.var_t0_dn5 = assign49580_e82669_d_n5;
        locals.var_t0_dn6 = assign49580_e82669_d_n6;
        locals.var_t0_dn7 = assign49580_e82669_d_n7;
        locals.var_t0_dn8 = assign49580_e82669_d_n8;
        locals.var_t0_dn9 = assign49580_e82669_d_n9;
        locals.var_t0_dn10 = assign49580_e82669_d_n10;
        locals.var_t0_dn11 = assign49580_e82669_d_n11;

        let (assign49590_e82680, assign49590_e82680_d_n3, assign49590_e82680_d_n4, assign49590_e82680_d_n5, assign49590_e82680_d_n6, assign49590_e82680_d_n7, assign49590_e82680_d_n8, assign49590_e82680_d_n9, assign49590_e82680_d_n10, assign49590_e82680_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard756 == 0.0)) && (locals.var_guard757 != 0.0)) {
        let assign49590_e82678: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign49590_e82678, ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn3), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign49590_e82680;
        locals.var_t1_dn3 = assign49590_e82680_d_n3;
        locals.var_t1_dn4 = assign49590_e82680_d_n4;
        locals.var_t1_dn5 = assign49590_e82680_d_n5;
        locals.var_t1_dn6 = assign49590_e82680_d_n6;
        locals.var_t1_dn7 = assign49590_e82680_d_n7;
        locals.var_t1_dn8 = assign49590_e82680_d_n8;
        locals.var_t1_dn9 = assign49590_e82680_d_n9;
        locals.var_t1_dn10 = assign49590_e82680_d_n10;
        locals.var_t1_dn11 = assign49590_e82680_d_n11;

        let (assign49600_e82692, assign49600_e82692_d_n3, assign49600_e82692_d_n4, assign49600_e82692_d_n5, assign49600_e82692_d_n6, assign49600_e82692_d_n7, assign49600_e82692_d_n8, assign49600_e82692_d_n9, assign49600_e82692_d_n10, assign49600_e82692_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard756 == 0.0)) && (locals.var_guard757 != 0.0)) {
        let assign49600_e82690: f64 = (locals.var_wstsi * locals.var_jtuns);
        (assign49600_e82690, (locals.var_wstsi * locals.var_jtuns_dn3), (locals.var_wstsi * locals.var_jtuns_dn4), (locals.var_wstsi * locals.var_jtuns_dn5), (locals.var_wstsi * locals.var_jtuns_dn6), (locals.var_wstsi * locals.var_jtuns_dn7), (locals.var_wstsi * locals.var_jtuns_dn8), (locals.var_wstsi * locals.var_jtuns_dn9), (locals.var_wstsi * locals.var_jtuns_dn10), (locals.var_wstsi * locals.var_jtuns_dn11),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign49600_e82692;
        locals.var_t3_dn3 = assign49600_e82692_d_n3;
        locals.var_t3_dn4 = assign49600_e82692_d_n4;
        locals.var_t3_dn5 = assign49600_e82692_d_n5;
        locals.var_t3_dn6 = assign49600_e82692_d_n6;
        locals.var_t3_dn7 = assign49600_e82692_d_n7;
        locals.var_t3_dn8 = assign49600_e82692_d_n8;
        locals.var_t3_dn9 = assign49600_e82692_d_n9;
        locals.var_t3_dn10 = assign49600_e82692_d_n10;
        locals.var_t3_dn11 = assign49600_e82692_d_n11;

    }

    pub(super) fn stamp_transient_block_168(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign49610_e82706, assign49610_e82706_d_n3, assign49610_e82706_d_n4, assign49610_e82706_d_n5, assign49610_e82706_d_n6, assign49610_e82706_d_n7, assign49610_e82706_d_n8, assign49610_e82706_d_n9, assign49610_e82706_d_n10, assign49610_e82706_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard756 == 0.0)) && (locals.var_guard757 != 0.0)) {
        let assign49610_e82703: f64 = (1.0 - locals.var_t1);
        let assign49610_e82704: f64 = (locals.var_t3 * assign49610_e82703);
        (assign49610_e82704, ((locals.var_t3_dn3 * assign49610_e82703) + (locals.var_t3 * (-locals.var_t1_dn3))), ((locals.var_t3_dn4 * assign49610_e82703) + (locals.var_t3 * (-locals.var_t1_dn4))), ((locals.var_t3_dn5 * assign49610_e82703) + (locals.var_t3 * (-locals.var_t1_dn5))), ((locals.var_t3_dn6 * assign49610_e82703) + (locals.var_t3 * (-locals.var_t1_dn6))), ((locals.var_t3_dn7 * assign49610_e82703) + (locals.var_t3 * (-locals.var_t1_dn7))), ((locals.var_t3_dn8 * assign49610_e82703) + (locals.var_t3 * (-locals.var_t1_dn8))), ((locals.var_t3_dn9 * assign49610_e82703) + (locals.var_t3 * (-locals.var_t1_dn9))), ((locals.var_t3_dn10 * assign49610_e82703) + (locals.var_t3 * (-locals.var_t1_dn10))), ((locals.var_t3_dn11 * assign49610_e82703) + (locals.var_t3 * (-locals.var_t1_dn11))),)
    } else {
        (locals.var_ibs4, locals.var_ibs4_dn3, locals.var_ibs4_dn4, locals.var_ibs4_dn5, locals.var_ibs4_dn6, locals.var_ibs4_dn7, locals.var_ibs4_dn8, locals.var_ibs4_dn9, locals.var_ibs4_dn10, locals.var_ibs4_dn11,)
    }
};
        locals.var_ibs4 = assign49610_e82706;
        locals.var_ibs4_dn3 = assign49610_e82706_d_n3;
        locals.var_ibs4_dn4 = assign49610_e82706_d_n4;
        locals.var_ibs4_dn5 = assign49610_e82706_d_n5;
        locals.var_ibs4_dn6 = assign49610_e82706_d_n6;
        locals.var_ibs4_dn7 = assign49610_e82706_d_n7;
        locals.var_ibs4_dn8 = assign49610_e82706_d_n8;
        locals.var_ibs4_dn9 = assign49610_e82706_d_n9;
        locals.var_ibs4_dn10 = assign49610_e82706_d_n10;
        locals.var_ibs4_dn11 = assign49610_e82706_d_n11;

        let (assign49620_e82721, assign49620_e82721_d_n3, assign49620_e82721_d_n4, assign49620_e82721_d_n5, assign49620_e82721_d_n6, assign49620_e82721_d_n7, assign49620_e82721_d_n8, assign49620_e82721_d_n9, assign49620_e82721_d_n10, assign49620_e82721_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard756 == 0.0)) && (locals.var_guard757 == 0.0)) {
        let assign49620_e82718: f64 = (locals.var_vtun0_i - locals.var_vbs_jct);
        let assign49620_e82719: f64 = (1.0 / assign49620_e82718);
        (assign49620_e82719, 0.0, 0.0, 0.0, 0.0, (-((-locals.var_vbs_jct_dn7) / (assign49620_e82718 * assign49620_e82718))), 0.0, 0.0, (-((-locals.var_vbs_jct_dn10) / (assign49620_e82718 * assign49620_e82718))), 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign49620_e82721;
        locals.var_t1_dn3 = assign49620_e82721_d_n3;
        locals.var_t1_dn4 = assign49620_e82721_d_n4;
        locals.var_t1_dn5 = assign49620_e82721_d_n5;
        locals.var_t1_dn6 = assign49620_e82721_d_n6;
        locals.var_t1_dn7 = assign49620_e82721_d_n7;
        locals.var_t1_dn8 = assign49620_e82721_d_n8;
        locals.var_t1_dn9 = assign49620_e82721_d_n9;
        locals.var_t1_dn10 = assign49620_e82721_d_n10;
        locals.var_t1_dn11 = assign49620_e82721_d_n11;

        let (assign49630_e82739, assign49630_e82739_d_n3, assign49630_e82739_d_n4, assign49630_e82739_d_n5, assign49630_e82739_d_n6, assign49630_e82739_d_n7, assign49630_e82739_d_n8, assign49630_e82739_d_n9, assign49630_e82739_d_n10, assign49630_e82739_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard756 == 0.0)) && (locals.var_guard757 == 0.0)) {
        let assign49630_e82731: f64 = (-locals.var_vbs_jct);
        let assign49630_e82733: f64 = (assign49630_e82731 / locals.var_nvtm2);
        let assign49630_e82735: f64 = (assign49630_e82733 * locals.var_vtun0_i);
        let assign49630_e82737: f64 = (assign49630_e82735 * locals.var_t1);
        (assign49630_e82737, (assign49630_e82735 * locals.var_t1_dn3), ((((-((assign49630_e82731 * locals.var_nvtm2_dn4) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_vtun0_i) * locals.var_t1) + (assign49630_e82735 * locals.var_t1_dn4)), ((((-((assign49630_e82731 * locals.var_nvtm2_dn5) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_vtun0_i) * locals.var_t1) + (assign49630_e82735 * locals.var_t1_dn5)), (assign49630_e82735 * locals.var_t1_dn6), (((((-locals.var_vbs_jct_dn7) / locals.var_nvtm2) * locals.var_vtun0_i) * locals.var_t1) + (assign49630_e82735 * locals.var_t1_dn7)), (assign49630_e82735 * locals.var_t1_dn8), (assign49630_e82735 * locals.var_t1_dn9), (((((-locals.var_vbs_jct_dn10) / locals.var_nvtm2) * locals.var_vtun0_i) * locals.var_t1) + (assign49630_e82735 * locals.var_t1_dn10)), (assign49630_e82735 * locals.var_t1_dn11),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign49630_e82739;
        locals.var_t0_dn3 = assign49630_e82739_d_n3;
        locals.var_t0_dn4 = assign49630_e82739_d_n4;
        locals.var_t0_dn5 = assign49630_e82739_d_n5;
        locals.var_t0_dn6 = assign49630_e82739_d_n6;
        locals.var_t0_dn7 = assign49630_e82739_d_n7;
        locals.var_t0_dn8 = assign49630_e82739_d_n8;
        locals.var_t0_dn9 = assign49630_e82739_d_n9;
        locals.var_t0_dn10 = assign49630_e82739_d_n10;
        locals.var_t0_dn11 = assign49630_e82739_d_n11;

        let (assign49640_e82751, assign49640_e82751_d_n3, assign49640_e82751_d_n4, assign49640_e82751_d_n5, assign49640_e82751_d_n6, assign49640_e82751_d_n7, assign49640_e82751_d_n8, assign49640_e82751_d_n9, assign49640_e82751_d_n10, assign49640_e82751_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard756 == 0.0)) && (locals.var_guard757 == 0.0)) {
        let assign49640_e82749: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign49640_e82749, ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn3), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign49640_e82751;
        locals.var_t1_dn3 = assign49640_e82751_d_n3;
        locals.var_t1_dn4 = assign49640_e82751_d_n4;
        locals.var_t1_dn5 = assign49640_e82751_d_n5;
        locals.var_t1_dn6 = assign49640_e82751_d_n6;
        locals.var_t1_dn7 = assign49640_e82751_d_n7;
        locals.var_t1_dn8 = assign49640_e82751_d_n8;
        locals.var_t1_dn9 = assign49640_e82751_d_n9;
        locals.var_t1_dn10 = assign49640_e82751_d_n10;
        locals.var_t1_dn11 = assign49640_e82751_d_n11;

        let (assign49650_e82764, assign49650_e82764_d_n3, assign49650_e82764_d_n4, assign49650_e82764_d_n5, assign49650_e82764_d_n6, assign49650_e82764_d_n7, assign49650_e82764_d_n8, assign49650_e82764_d_n9, assign49650_e82764_d_n10, assign49650_e82764_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard756 == 0.0)) && (locals.var_guard757 == 0.0)) {
        let assign49650_e82762: f64 = (locals.var_wstsi * locals.var_jtuns);
        (assign49650_e82762, (locals.var_wstsi * locals.var_jtuns_dn3), (locals.var_wstsi * locals.var_jtuns_dn4), (locals.var_wstsi * locals.var_jtuns_dn5), (locals.var_wstsi * locals.var_jtuns_dn6), (locals.var_wstsi * locals.var_jtuns_dn7), (locals.var_wstsi * locals.var_jtuns_dn8), (locals.var_wstsi * locals.var_jtuns_dn9), (locals.var_wstsi * locals.var_jtuns_dn10), (locals.var_wstsi * locals.var_jtuns_dn11),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign49650_e82764;
        locals.var_t3_dn3 = assign49650_e82764_d_n3;
        locals.var_t3_dn4 = assign49650_e82764_d_n4;
        locals.var_t3_dn5 = assign49650_e82764_d_n5;
        locals.var_t3_dn6 = assign49650_e82764_d_n6;
        locals.var_t3_dn7 = assign49650_e82764_d_n7;
        locals.var_t3_dn8 = assign49650_e82764_d_n8;
        locals.var_t3_dn9 = assign49650_e82764_d_n9;
        locals.var_t3_dn10 = assign49650_e82764_d_n10;
        locals.var_t3_dn11 = assign49650_e82764_d_n11;

        let (assign49660_e82779, assign49660_e82779_d_n3, assign49660_e82779_d_n4, assign49660_e82779_d_n5, assign49660_e82779_d_n6, assign49660_e82779_d_n7, assign49660_e82779_d_n8, assign49660_e82779_d_n9, assign49660_e82779_d_n10, assign49660_e82779_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard756 == 0.0)) && (locals.var_guard757 == 0.0)) {
        let assign49660_e82776: f64 = (1.0 - locals.var_t1);
        let assign49660_e82777: f64 = (locals.var_t3 * assign49660_e82776);
        (assign49660_e82777, ((locals.var_t3_dn3 * assign49660_e82776) + (locals.var_t3 * (-locals.var_t1_dn3))), ((locals.var_t3_dn4 * assign49660_e82776) + (locals.var_t3 * (-locals.var_t1_dn4))), ((locals.var_t3_dn5 * assign49660_e82776) + (locals.var_t3 * (-locals.var_t1_dn5))), ((locals.var_t3_dn6 * assign49660_e82776) + (locals.var_t3 * (-locals.var_t1_dn6))), ((locals.var_t3_dn7 * assign49660_e82776) + (locals.var_t3 * (-locals.var_t1_dn7))), ((locals.var_t3_dn8 * assign49660_e82776) + (locals.var_t3 * (-locals.var_t1_dn8))), ((locals.var_t3_dn9 * assign49660_e82776) + (locals.var_t3 * (-locals.var_t1_dn9))), ((locals.var_t3_dn10 * assign49660_e82776) + (locals.var_t3 * (-locals.var_t1_dn10))), ((locals.var_t3_dn11 * assign49660_e82776) + (locals.var_t3 * (-locals.var_t1_dn11))),)
    } else {
        (locals.var_ibs4, locals.var_ibs4_dn3, locals.var_ibs4_dn4, locals.var_ibs4_dn5, locals.var_ibs4_dn6, locals.var_ibs4_dn7, locals.var_ibs4_dn8, locals.var_ibs4_dn9, locals.var_ibs4_dn10, locals.var_ibs4_dn11,)
    }
};
        locals.var_ibs4 = assign49660_e82779;
        locals.var_ibs4_dn3 = assign49660_e82779_d_n3;
        locals.var_ibs4_dn4 = assign49660_e82779_d_n4;
        locals.var_ibs4_dn5 = assign49660_e82779_d_n5;
        locals.var_ibs4_dn6 = assign49660_e82779_d_n6;
        locals.var_ibs4_dn7 = assign49660_e82779_d_n7;
        locals.var_ibs4_dn8 = assign49660_e82779_d_n8;
        locals.var_ibs4_dn9 = assign49660_e82779_d_n9;
        locals.var_ibs4_dn10 = assign49660_e82779_d_n10;
        locals.var_ibs4_dn11 = assign49660_e82779_d_n11;

        let (assign49670_e82789, assign49670_e82789_d_n4, assign49670_e82789_d_n5,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard756 == 0.0)) {
        let assign49670_e82787: f64 = (p.p925 * locals.var_ntund_i);
        (assign49670_e82787, 0.0, 0.0,)
    } else {
        (locals.var_nvtm2, locals.var_nvtm2_dn4, locals.var_nvtm2_dn5,)
    }
};
        locals.var_nvtm2 = assign49670_e82789;
        locals.var_nvtm2_dn4 = assign49670_e82789_d_n4;
        locals.var_nvtm2_dn5 = assign49670_e82789_d_n5;

        let assign49680_e82792: f64 = (locals.var_vtun0d_i - locals.var_vbd_jct);
        let assign49680_e82794: f64 = if assign49680_e82792 < 0.001 { 1.0 } else { 0.0 };
        locals.var_guard758 = assign49680_e82794;

        let (assign49690_e82804, assign49690_e82804_d_n3, assign49690_e82804_d_n4, assign49690_e82804_d_n5, assign49690_e82804_d_n6, assign49690_e82804_d_n7, assign49690_e82804_d_n8, assign49690_e82804_d_n9, assign49690_e82804_d_n10, assign49690_e82804_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard756 == 0.0)) && (locals.var_guard758 != 0.0)) {
        (1000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign49690_e82804;
        locals.var_t1_dn3 = assign49690_e82804_d_n3;
        locals.var_t1_dn4 = assign49690_e82804_d_n4;
        locals.var_t1_dn5 = assign49690_e82804_d_n5;
        locals.var_t1_dn6 = assign49690_e82804_d_n6;
        locals.var_t1_dn7 = assign49690_e82804_d_n7;
        locals.var_t1_dn8 = assign49690_e82804_d_n8;
        locals.var_t1_dn9 = assign49690_e82804_d_n9;
        locals.var_t1_dn10 = assign49690_e82804_d_n10;
        locals.var_t1_dn11 = assign49690_e82804_d_n11;

        let (assign49700_e82821, assign49700_e82821_d_n3, assign49700_e82821_d_n4, assign49700_e82821_d_n5, assign49700_e82821_d_n6, assign49700_e82821_d_n7, assign49700_e82821_d_n8, assign49700_e82821_d_n9, assign49700_e82821_d_n10, assign49700_e82821_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard756 == 0.0)) && (locals.var_guard758 != 0.0)) {
        let assign49700_e82813: f64 = (-locals.var_vbd_jct);
        let assign49700_e82815: f64 = (assign49700_e82813 / locals.var_nvtm2);
        let assign49700_e82817: f64 = (assign49700_e82815 * locals.var_vtun0d_i);
        let assign49700_e82819: f64 = (assign49700_e82817 * locals.var_t1);
        (assign49700_e82819, (assign49700_e82817 * locals.var_t1_dn3), ((((-((assign49700_e82813 * locals.var_nvtm2_dn4) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_vtun0d_i) * locals.var_t1) + (assign49700_e82817 * locals.var_t1_dn4)), ((((-((assign49700_e82813 * locals.var_nvtm2_dn5) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_vtun0d_i) * locals.var_t1) + (assign49700_e82817 * locals.var_t1_dn5)), (((((-locals.var_vbd_jct_dn6) / locals.var_nvtm2) * locals.var_vtun0d_i) * locals.var_t1) + (assign49700_e82817 * locals.var_t1_dn6)), (assign49700_e82817 * locals.var_t1_dn7), (assign49700_e82817 * locals.var_t1_dn8), (assign49700_e82817 * locals.var_t1_dn9), (((((-locals.var_vbd_jct_dn10) / locals.var_nvtm2) * locals.var_vtun0d_i) * locals.var_t1) + (assign49700_e82817 * locals.var_t1_dn10)), (assign49700_e82817 * locals.var_t1_dn11),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign49700_e82821;
        locals.var_t0_dn3 = assign49700_e82821_d_n3;
        locals.var_t0_dn4 = assign49700_e82821_d_n4;
        locals.var_t0_dn5 = assign49700_e82821_d_n5;
        locals.var_t0_dn6 = assign49700_e82821_d_n6;
        locals.var_t0_dn7 = assign49700_e82821_d_n7;
        locals.var_t0_dn8 = assign49700_e82821_d_n8;
        locals.var_t0_dn9 = assign49700_e82821_d_n9;
        locals.var_t0_dn10 = assign49700_e82821_d_n10;
        locals.var_t0_dn11 = assign49700_e82821_d_n11;

        let (assign49710_e82832, assign49710_e82832_d_n3, assign49710_e82832_d_n4, assign49710_e82832_d_n5, assign49710_e82832_d_n6, assign49710_e82832_d_n7, assign49710_e82832_d_n8, assign49710_e82832_d_n9, assign49710_e82832_d_n10, assign49710_e82832_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard756 == 0.0)) && (locals.var_guard758 != 0.0)) {
        let assign49710_e82830: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign49710_e82830, ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn3), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign49710_e82832;
        locals.var_t1_dn3 = assign49710_e82832_d_n3;
        locals.var_t1_dn4 = assign49710_e82832_d_n4;
        locals.var_t1_dn5 = assign49710_e82832_d_n5;
        locals.var_t1_dn6 = assign49710_e82832_d_n6;
        locals.var_t1_dn7 = assign49710_e82832_d_n7;
        locals.var_t1_dn8 = assign49710_e82832_d_n8;
        locals.var_t1_dn9 = assign49710_e82832_d_n9;
        locals.var_t1_dn10 = assign49710_e82832_d_n10;
        locals.var_t1_dn11 = assign49710_e82832_d_n11;

        let (assign49720_e82844, assign49720_e82844_d_n3, assign49720_e82844_d_n4, assign49720_e82844_d_n5, assign49720_e82844_d_n6, assign49720_e82844_d_n7, assign49720_e82844_d_n8, assign49720_e82844_d_n9, assign49720_e82844_d_n10, assign49720_e82844_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard756 == 0.0)) && (locals.var_guard758 != 0.0)) {
        let assign49720_e82842: f64 = (locals.var_wstsi * locals.var_jtund);
        (assign49720_e82842, (locals.var_wstsi * locals.var_jtund_dn3), (locals.var_wstsi * locals.var_jtund_dn4), (locals.var_wstsi * locals.var_jtund_dn5), (locals.var_wstsi * locals.var_jtund_dn6), (locals.var_wstsi * locals.var_jtund_dn7), (locals.var_wstsi * locals.var_jtund_dn8), (locals.var_wstsi * locals.var_jtund_dn9), (locals.var_wstsi * locals.var_jtund_dn10), (locals.var_wstsi * locals.var_jtund_dn11),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign49720_e82844;
        locals.var_t3_dn3 = assign49720_e82844_d_n3;
        locals.var_t3_dn4 = assign49720_e82844_d_n4;
        locals.var_t3_dn5 = assign49720_e82844_d_n5;
        locals.var_t3_dn6 = assign49720_e82844_d_n6;
        locals.var_t3_dn7 = assign49720_e82844_d_n7;
        locals.var_t3_dn8 = assign49720_e82844_d_n8;
        locals.var_t3_dn9 = assign49720_e82844_d_n9;
        locals.var_t3_dn10 = assign49720_e82844_d_n10;
        locals.var_t3_dn11 = assign49720_e82844_d_n11;

        let (assign49730_e82858, assign49730_e82858_d_n3, assign49730_e82858_d_n4, assign49730_e82858_d_n5, assign49730_e82858_d_n6, assign49730_e82858_d_n7, assign49730_e82858_d_n8, assign49730_e82858_d_n9, assign49730_e82858_d_n10, assign49730_e82858_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard756 == 0.0)) && (locals.var_guard758 != 0.0)) {
        let assign49730_e82855: f64 = (1.0 - locals.var_t1);
        let assign49730_e82856: f64 = (locals.var_t3 * assign49730_e82855);
        (assign49730_e82856, ((locals.var_t3_dn3 * assign49730_e82855) + (locals.var_t3 * (-locals.var_t1_dn3))), ((locals.var_t3_dn4 * assign49730_e82855) + (locals.var_t3 * (-locals.var_t1_dn4))), ((locals.var_t3_dn5 * assign49730_e82855) + (locals.var_t3 * (-locals.var_t1_dn5))), ((locals.var_t3_dn6 * assign49730_e82855) + (locals.var_t3 * (-locals.var_t1_dn6))), ((locals.var_t3_dn7 * assign49730_e82855) + (locals.var_t3 * (-locals.var_t1_dn7))), ((locals.var_t3_dn8 * assign49730_e82855) + (locals.var_t3 * (-locals.var_t1_dn8))), ((locals.var_t3_dn9 * assign49730_e82855) + (locals.var_t3 * (-locals.var_t1_dn9))), ((locals.var_t3_dn10 * assign49730_e82855) + (locals.var_t3 * (-locals.var_t1_dn10))), ((locals.var_t3_dn11 * assign49730_e82855) + (locals.var_t3 * (-locals.var_t1_dn11))),)
    } else {
        (locals.var_ibd4, locals.var_ibd4_dn3, locals.var_ibd4_dn4, locals.var_ibd4_dn5, locals.var_ibd4_dn6, locals.var_ibd4_dn7, locals.var_ibd4_dn8, locals.var_ibd4_dn9, locals.var_ibd4_dn10, locals.var_ibd4_dn11,)
    }
};
        locals.var_ibd4 = assign49730_e82858;
        locals.var_ibd4_dn3 = assign49730_e82858_d_n3;
        locals.var_ibd4_dn4 = assign49730_e82858_d_n4;
        locals.var_ibd4_dn5 = assign49730_e82858_d_n5;
        locals.var_ibd4_dn6 = assign49730_e82858_d_n6;
        locals.var_ibd4_dn7 = assign49730_e82858_d_n7;
        locals.var_ibd4_dn8 = assign49730_e82858_d_n8;
        locals.var_ibd4_dn9 = assign49730_e82858_d_n9;
        locals.var_ibd4_dn10 = assign49730_e82858_d_n10;
        locals.var_ibd4_dn11 = assign49730_e82858_d_n11;

        let (assign49740_e82873, assign49740_e82873_d_n3, assign49740_e82873_d_n4, assign49740_e82873_d_n5, assign49740_e82873_d_n6, assign49740_e82873_d_n7, assign49740_e82873_d_n8, assign49740_e82873_d_n9, assign49740_e82873_d_n10, assign49740_e82873_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard756 == 0.0)) && (locals.var_guard758 == 0.0)) {
        let assign49740_e82870: f64 = (locals.var_vtun0d_i - locals.var_vbd_jct);
        let assign49740_e82871: f64 = (1.0 / assign49740_e82870);
        (assign49740_e82871, 0.0, 0.0, 0.0, (-((-locals.var_vbd_jct_dn6) / (assign49740_e82870 * assign49740_e82870))), 0.0, 0.0, 0.0, (-((-locals.var_vbd_jct_dn10) / (assign49740_e82870 * assign49740_e82870))), 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign49740_e82873;
        locals.var_t1_dn3 = assign49740_e82873_d_n3;
        locals.var_t1_dn4 = assign49740_e82873_d_n4;
        locals.var_t1_dn5 = assign49740_e82873_d_n5;
        locals.var_t1_dn6 = assign49740_e82873_d_n6;
        locals.var_t1_dn7 = assign49740_e82873_d_n7;
        locals.var_t1_dn8 = assign49740_e82873_d_n8;
        locals.var_t1_dn9 = assign49740_e82873_d_n9;
        locals.var_t1_dn10 = assign49740_e82873_d_n10;
        locals.var_t1_dn11 = assign49740_e82873_d_n11;

        let (assign49750_e82891, assign49750_e82891_d_n3, assign49750_e82891_d_n4, assign49750_e82891_d_n5, assign49750_e82891_d_n6, assign49750_e82891_d_n7, assign49750_e82891_d_n8, assign49750_e82891_d_n9, assign49750_e82891_d_n10, assign49750_e82891_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard756 == 0.0)) && (locals.var_guard758 == 0.0)) {
        let assign49750_e82883: f64 = (-locals.var_vbd_jct);
        let assign49750_e82885: f64 = (assign49750_e82883 / locals.var_nvtm2);
        let assign49750_e82887: f64 = (assign49750_e82885 * locals.var_vtun0d_i);
        let assign49750_e82889: f64 = (assign49750_e82887 * locals.var_t1);
        (assign49750_e82889, (assign49750_e82887 * locals.var_t1_dn3), ((((-((assign49750_e82883 * locals.var_nvtm2_dn4) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_vtun0d_i) * locals.var_t1) + (assign49750_e82887 * locals.var_t1_dn4)), ((((-((assign49750_e82883 * locals.var_nvtm2_dn5) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_vtun0d_i) * locals.var_t1) + (assign49750_e82887 * locals.var_t1_dn5)), (((((-locals.var_vbd_jct_dn6) / locals.var_nvtm2) * locals.var_vtun0d_i) * locals.var_t1) + (assign49750_e82887 * locals.var_t1_dn6)), (assign49750_e82887 * locals.var_t1_dn7), (assign49750_e82887 * locals.var_t1_dn8), (assign49750_e82887 * locals.var_t1_dn9), (((((-locals.var_vbd_jct_dn10) / locals.var_nvtm2) * locals.var_vtun0d_i) * locals.var_t1) + (assign49750_e82887 * locals.var_t1_dn10)), (assign49750_e82887 * locals.var_t1_dn11),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign49750_e82891;
        locals.var_t0_dn3 = assign49750_e82891_d_n3;
        locals.var_t0_dn4 = assign49750_e82891_d_n4;
        locals.var_t0_dn5 = assign49750_e82891_d_n5;
        locals.var_t0_dn6 = assign49750_e82891_d_n6;
        locals.var_t0_dn7 = assign49750_e82891_d_n7;
        locals.var_t0_dn8 = assign49750_e82891_d_n8;
        locals.var_t0_dn9 = assign49750_e82891_d_n9;
        locals.var_t0_dn10 = assign49750_e82891_d_n10;
        locals.var_t0_dn11 = assign49750_e82891_d_n11;

        let (assign49760_e82903, assign49760_e82903_d_n3, assign49760_e82903_d_n4, assign49760_e82903_d_n5, assign49760_e82903_d_n6, assign49760_e82903_d_n7, assign49760_e82903_d_n8, assign49760_e82903_d_n9, assign49760_e82903_d_n10, assign49760_e82903_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard756 == 0.0)) && (locals.var_guard758 == 0.0)) {
        let assign49760_e82901: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign49760_e82901, ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn3), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign49760_e82903;
        locals.var_t1_dn3 = assign49760_e82903_d_n3;
        locals.var_t1_dn4 = assign49760_e82903_d_n4;
        locals.var_t1_dn5 = assign49760_e82903_d_n5;
        locals.var_t1_dn6 = assign49760_e82903_d_n6;
        locals.var_t1_dn7 = assign49760_e82903_d_n7;
        locals.var_t1_dn8 = assign49760_e82903_d_n8;
        locals.var_t1_dn9 = assign49760_e82903_d_n9;
        locals.var_t1_dn10 = assign49760_e82903_d_n10;
        locals.var_t1_dn11 = assign49760_e82903_d_n11;

        let (assign49770_e82916, assign49770_e82916_d_n3, assign49770_e82916_d_n4, assign49770_e82916_d_n5, assign49770_e82916_d_n6, assign49770_e82916_d_n7, assign49770_e82916_d_n8, assign49770_e82916_d_n9, assign49770_e82916_d_n10, assign49770_e82916_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard756 == 0.0)) && (locals.var_guard758 == 0.0)) {
        let assign49770_e82914: f64 = (locals.var_wstsi * locals.var_jtund);
        (assign49770_e82914, (locals.var_wstsi * locals.var_jtund_dn3), (locals.var_wstsi * locals.var_jtund_dn4), (locals.var_wstsi * locals.var_jtund_dn5), (locals.var_wstsi * locals.var_jtund_dn6), (locals.var_wstsi * locals.var_jtund_dn7), (locals.var_wstsi * locals.var_jtund_dn8), (locals.var_wstsi * locals.var_jtund_dn9), (locals.var_wstsi * locals.var_jtund_dn10), (locals.var_wstsi * locals.var_jtund_dn11),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign49770_e82916;
        locals.var_t3_dn3 = assign49770_e82916_d_n3;
        locals.var_t3_dn4 = assign49770_e82916_d_n4;
        locals.var_t3_dn5 = assign49770_e82916_d_n5;
        locals.var_t3_dn6 = assign49770_e82916_d_n6;
        locals.var_t3_dn7 = assign49770_e82916_d_n7;
        locals.var_t3_dn8 = assign49770_e82916_d_n8;
        locals.var_t3_dn9 = assign49770_e82916_d_n9;
        locals.var_t3_dn10 = assign49770_e82916_d_n10;
        locals.var_t3_dn11 = assign49770_e82916_d_n11;

        let (assign49780_e82931, assign49780_e82931_d_n3, assign49780_e82931_d_n4, assign49780_e82931_d_n5, assign49780_e82931_d_n6, assign49780_e82931_d_n7, assign49780_e82931_d_n8, assign49780_e82931_d_n9, assign49780_e82931_d_n10, assign49780_e82931_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard756 == 0.0)) && (locals.var_guard758 == 0.0)) {
        let assign49780_e82928: f64 = (1.0 - locals.var_t1);
        let assign49780_e82929: f64 = (locals.var_t3 * assign49780_e82928);
        (assign49780_e82929, ((locals.var_t3_dn3 * assign49780_e82928) + (locals.var_t3 * (-locals.var_t1_dn3))), ((locals.var_t3_dn4 * assign49780_e82928) + (locals.var_t3 * (-locals.var_t1_dn4))), ((locals.var_t3_dn5 * assign49780_e82928) + (locals.var_t3 * (-locals.var_t1_dn5))), ((locals.var_t3_dn6 * assign49780_e82928) + (locals.var_t3 * (-locals.var_t1_dn6))), ((locals.var_t3_dn7 * assign49780_e82928) + (locals.var_t3 * (-locals.var_t1_dn7))), ((locals.var_t3_dn8 * assign49780_e82928) + (locals.var_t3 * (-locals.var_t1_dn8))), ((locals.var_t3_dn9 * assign49780_e82928) + (locals.var_t3 * (-locals.var_t1_dn9))), ((locals.var_t3_dn10 * assign49780_e82928) + (locals.var_t3 * (-locals.var_t1_dn10))), ((locals.var_t3_dn11 * assign49780_e82928) + (locals.var_t3 * (-locals.var_t1_dn11))),)
    } else {
        (locals.var_ibd4, locals.var_ibd4_dn3, locals.var_ibd4_dn4, locals.var_ibd4_dn5, locals.var_ibd4_dn6, locals.var_ibd4_dn7, locals.var_ibd4_dn8, locals.var_ibd4_dn9, locals.var_ibd4_dn10, locals.var_ibd4_dn11,)
    }
};
        locals.var_ibd4 = assign49780_e82931;
        locals.var_ibd4_dn3 = assign49780_e82931_d_n3;
        locals.var_ibd4_dn4 = assign49780_e82931_d_n4;
        locals.var_ibd4_dn5 = assign49780_e82931_d_n5;
        locals.var_ibd4_dn6 = assign49780_e82931_d_n6;
        locals.var_ibd4_dn7 = assign49780_e82931_d_n7;
        locals.var_ibd4_dn8 = assign49780_e82931_d_n8;
        locals.var_ibd4_dn9 = assign49780_e82931_d_n9;
        locals.var_ibd4_dn10 = assign49780_e82931_d_n10;
        locals.var_ibd4_dn11 = assign49780_e82931_d_n11;

        let (assign49790_e82944, assign49790_e82944_d_n3, assign49790_e82944_d_n4, assign49790_e82944_d_n5, assign49790_e82944_d_n6, assign49790_e82944_d_n7, assign49790_e82944_d_n8, assign49790_e82944_d_n9, assign49790_e82944_d_n10, assign49790_e82944_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign49790_e82937: f64 = (locals.var_ibs1 + locals.var_ibs2);
        let assign49790_e82939: f64 = (assign49790_e82937 + locals.var_ibs3);
        let assign49790_e82941: f64 = (assign49790_e82939 + locals.var_ibs4);
        let assign49790_e82942: f64 = (p.p2 * assign49790_e82941);
        (assign49790_e82942, (p.p2 * (((locals.var_ibs1_dn3 + locals.var_ibs2_dn3) + locals.var_ibs3_dn3) + locals.var_ibs4_dn3)), (p.p2 * (((locals.var_ibs1_dn4 + locals.var_ibs2_dn4) + locals.var_ibs3_dn4) + locals.var_ibs4_dn4)), (p.p2 * (((locals.var_ibs1_dn5 + locals.var_ibs2_dn5) + locals.var_ibs3_dn5) + locals.var_ibs4_dn5)), (p.p2 * (((locals.var_ibs1_dn6 + locals.var_ibs2_dn6) + locals.var_ibs3_dn6) + locals.var_ibs4_dn6)), (p.p2 * (((locals.var_ibs1_dn7 + locals.var_ibs2_dn7) + locals.var_ibs3_dn7) + locals.var_ibs4_dn7)), (p.p2 * (((locals.var_ibs1_dn8 + locals.var_ibs2_dn8) + locals.var_ibs3_dn8) + locals.var_ibs4_dn8)), (p.p2 * (((locals.var_ibs1_dn9 + locals.var_ibs2_dn9) + locals.var_ibs3_dn9) + locals.var_ibs4_dn9)), (p.p2 * (((locals.var_ibs1_dn10 + locals.var_ibs2_dn10) + locals.var_ibs3_dn10) + locals.var_ibs4_dn10)), (p.p2 * (((locals.var_ibs1_dn11 + locals.var_ibs2_dn11) + locals.var_ibs3_dn11) + locals.var_ibs4_dn11)),)
    } else {
        (locals.var_ibs, locals.var_ibs_dn3, locals.var_ibs_dn4, locals.var_ibs_dn5, locals.var_ibs_dn6, locals.var_ibs_dn7, locals.var_ibs_dn8, locals.var_ibs_dn9, locals.var_ibs_dn10, locals.var_ibs_dn11,)
    }
};
        locals.var_ibs = assign49790_e82944;
        locals.var_ibs_dn3 = assign49790_e82944_d_n3;
        locals.var_ibs_dn4 = assign49790_e82944_d_n4;
        locals.var_ibs_dn5 = assign49790_e82944_d_n5;
        locals.var_ibs_dn6 = assign49790_e82944_d_n6;
        locals.var_ibs_dn7 = assign49790_e82944_d_n7;
        locals.var_ibs_dn8 = assign49790_e82944_d_n8;
        locals.var_ibs_dn9 = assign49790_e82944_d_n9;
        locals.var_ibs_dn10 = assign49790_e82944_d_n10;
        locals.var_ibs_dn11 = assign49790_e82944_d_n11;

        let (assign49800_e82957, assign49800_e82957_d_n3, assign49800_e82957_d_n4, assign49800_e82957_d_n5, assign49800_e82957_d_n6, assign49800_e82957_d_n7, assign49800_e82957_d_n8, assign49800_e82957_d_n9, assign49800_e82957_d_n10, assign49800_e82957_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign49800_e82950: f64 = (locals.var_ibd1 + locals.var_ibd2);
        let assign49800_e82952: f64 = (assign49800_e82950 + locals.var_ibd3);
        let assign49800_e82954: f64 = (assign49800_e82952 + locals.var_ibd4);
        let assign49800_e82955: f64 = (p.p2 * assign49800_e82954);
        (assign49800_e82955, (p.p2 * (((locals.var_ibd1_dn3 + locals.var_ibd2_dn3) + locals.var_ibd3_dn3) + locals.var_ibd4_dn3)), (p.p2 * (((locals.var_ibd1_dn4 + locals.var_ibd2_dn4) + locals.var_ibd3_dn4) + locals.var_ibd4_dn4)), (p.p2 * (((locals.var_ibd1_dn5 + locals.var_ibd2_dn5) + locals.var_ibd3_dn5) + locals.var_ibd4_dn5)), (p.p2 * (((locals.var_ibd1_dn6 + locals.var_ibd2_dn6) + locals.var_ibd3_dn6) + locals.var_ibd4_dn6)), (p.p2 * (((locals.var_ibd1_dn7 + locals.var_ibd2_dn7) + locals.var_ibd3_dn7) + locals.var_ibd4_dn7)), (p.p2 * (((locals.var_ibd1_dn8 + locals.var_ibd2_dn8) + locals.var_ibd3_dn8) + locals.var_ibd4_dn8)), (p.p2 * (((locals.var_ibd1_dn9 + locals.var_ibd2_dn9) + locals.var_ibd3_dn9) + locals.var_ibd4_dn9)), (p.p2 * (((locals.var_ibd1_dn10 + locals.var_ibd2_dn10) + locals.var_ibd3_dn10) + locals.var_ibd4_dn10)), (p.p2 * (((locals.var_ibd1_dn11 + locals.var_ibd2_dn11) + locals.var_ibd3_dn11) + locals.var_ibd4_dn11)),)
    } else {
        (locals.var_ibd, locals.var_ibd_dn3, locals.var_ibd_dn4, locals.var_ibd_dn5, locals.var_ibd_dn6, locals.var_ibd_dn7, locals.var_ibd_dn8, locals.var_ibd_dn9, locals.var_ibd_dn10, locals.var_ibd_dn11,)
    }
};
        locals.var_ibd = assign49800_e82957;
        locals.var_ibd_dn3 = assign49800_e82957_d_n3;
        locals.var_ibd_dn4 = assign49800_e82957_d_n4;
        locals.var_ibd_dn5 = assign49800_e82957_d_n5;
        locals.var_ibd_dn6 = assign49800_e82957_d_n6;
        locals.var_ibd_dn7 = assign49800_e82957_d_n7;
        locals.var_ibd_dn8 = assign49800_e82957_d_n8;
        locals.var_ibd_dn9 = assign49800_e82957_d_n9;
        locals.var_ibd_dn10 = assign49800_e82957_d_n10;
        locals.var_ibd_dn11 = assign49800_e82957_d_n11;

        let (assign49810_e82962, assign49810_e82962_d_n3, assign49810_e82962_d_n4, assign49810_e82962_d_n5, assign49810_e82962_d_n6, assign49810_e82962_d_n7, assign49810_e82962_d_n8, assign49810_e82962_d_n9, assign49810_e82962_d_n10, assign49810_e82962_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_igisl, locals.var_igisl_dn3, locals.var_igisl_dn4, locals.var_igisl_dn5, locals.var_igisl_dn6, locals.var_igisl_dn7, locals.var_igisl_dn8, locals.var_igisl_dn9, locals.var_igisl_dn10, locals.var_igisl_dn11,)
    }
};
        locals.var_igisl = assign49810_e82962;
        locals.var_igisl_dn3 = assign49810_e82962_d_n3;
        locals.var_igisl_dn4 = assign49810_e82962_d_n4;
        locals.var_igisl_dn5 = assign49810_e82962_d_n5;
        locals.var_igisl_dn6 = assign49810_e82962_d_n6;
        locals.var_igisl_dn7 = assign49810_e82962_d_n7;
        locals.var_igisl_dn8 = assign49810_e82962_d_n8;
        locals.var_igisl_dn9 = assign49810_e82962_d_n9;
        locals.var_igisl_dn10 = assign49810_e82962_d_n10;
        locals.var_igisl_dn11 = assign49810_e82962_d_n11;

        let (assign49820_e82967, assign49820_e82967_d_n3, assign49820_e82967_d_n4, assign49820_e82967_d_n5, assign49820_e82967_d_n6, assign49820_e82967_d_n7, assign49820_e82967_d_n8, assign49820_e82967_d_n9, assign49820_e82967_d_n10, assign49820_e82967_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_igidl, locals.var_igidl_dn3, locals.var_igidl_dn4, locals.var_igidl_dn5, locals.var_igidl_dn6, locals.var_igidl_dn7, locals.var_igidl_dn8, locals.var_igidl_dn9, locals.var_igidl_dn10, locals.var_igidl_dn11,)
    }
};
        locals.var_igidl = assign49820_e82967;
        locals.var_igidl_dn3 = assign49820_e82967_d_n3;
        locals.var_igidl_dn4 = assign49820_e82967_d_n4;
        locals.var_igidl_dn5 = assign49820_e82967_d_n5;
        locals.var_igidl_dn6 = assign49820_e82967_d_n6;
        locals.var_igidl_dn7 = assign49820_e82967_d_n7;
        locals.var_igidl_dn8 = assign49820_e82967_d_n8;
        locals.var_igidl_dn9 = assign49820_e82967_d_n9;
        locals.var_igidl_dn10 = assign49820_e82967_d_n10;
        locals.var_igidl_dn11 = assign49820_e82967_d_n11;

        let assign49830_e82970: f64 = if p.p36 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard759 = assign49830_e82970;

        let (assign49840_e82979, assign49840_e82979_d_n3, assign49840_e82979_d_n4, assign49840_e82979_d_n5, assign49840_e82979_d_n6, assign49840_e82979_d_n7, assign49840_e82979_d_n8, assign49840_e82979_d_n9, assign49840_e82979_d_n10, assign49840_e82979_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard759 != 0.0)) {
        let assign49840_e82977: f64 = (locals.var_epsratio * p.p76);
        (assign49840_e82977, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign49840_e82979;
        locals.var_t0_dn3 = assign49840_e82979_d_n3;
        locals.var_t0_dn4 = assign49840_e82979_d_n4;
        locals.var_t0_dn5 = assign49840_e82979_d_n5;
        locals.var_t0_dn6 = assign49840_e82979_d_n6;
        locals.var_t0_dn7 = assign49840_e82979_d_n7;
        locals.var_t0_dn8 = assign49840_e82979_d_n8;
        locals.var_t0_dn9 = assign49840_e82979_d_n9;
        locals.var_t0_dn10 = assign49840_e82979_d_n10;
        locals.var_t0_dn11 = assign49840_e82979_d_n11;

        let assign49850_e82990: f64 = if (((locals.var_agidl_i <= 0.0) || (locals.var_bgidl_t <= 0.0)) || (locals.var_cgidl_i < 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard760 = assign49850_e82990;

        let (assign49860_e82999, assign49860_e82999_d_n3, assign49860_e82999_d_n4, assign49860_e82999_d_n5, assign49860_e82999_d_n6, assign49860_e82999_d_n7, assign49860_e82999_d_n8, assign49860_e82999_d_n9, assign49860_e82999_d_n10, assign49860_e82999_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard759 != 0.0)) && (locals.var_guard760 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign49860_e82999;
        locals.var_t6_dn3 = assign49860_e82999_d_n3;
        locals.var_t6_dn4 = assign49860_e82999_d_n4;
        locals.var_t6_dn5 = assign49860_e82999_d_n5;
        locals.var_t6_dn6 = assign49860_e82999_d_n6;
        locals.var_t6_dn7 = assign49860_e82999_d_n7;
        locals.var_t6_dn8 = assign49860_e82999_d_n8;
        locals.var_t6_dn9 = assign49860_e82999_d_n9;
        locals.var_t6_dn10 = assign49860_e82999_d_n10;
        locals.var_t6_dn11 = assign49860_e82999_d_n11;

        let (assign49870_e83016, assign49870_e83016_d_n3, assign49870_e83016_d_n4, assign49870_e83016_d_n5, assign49870_e83016_d_n6, assign49870_e83016_d_n7, assign49870_e83016_d_n8, assign49870_e83016_d_n9, assign49870_e83016_d_n10, assign49870_e83016_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard759 != 0.0)) && (locals.var_guard760 == 0.0)) {
        let assign49870_e83008: f64 = (-locals.var_vgd_noswap);
        let assign49870_e83010: f64 = (assign49870_e83008 - locals.var_egidl_i);
        let assign49870_e83012: f64 = (assign49870_e83010 + locals.var_vfbsdr);
        let assign49870_e83014: f64 = (assign49870_e83012 / locals.var_t0);
        (assign49870_e83014, (-((assign49870_e83012 * locals.var_t0_dn3) / (locals.var_t0 * locals.var_t0))), (((locals.var_vfbsdr_dn4 * locals.var_t0) - (assign49870_e83012 * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)), (((locals.var_vfbsdr_dn5 * locals.var_t0) - (assign49870_e83012 * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)), ((((-locals.var_vgd_noswap_dn6) * locals.var_t0) - (assign49870_e83012 * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)), ((((-locals.var_vgd_noswap_dn7) * locals.var_t0) - (assign49870_e83012 * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)), ((((-locals.var_vgd_noswap_dn8) * locals.var_t0) - (assign49870_e83012 * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)), (-((assign49870_e83012 * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0))), ((((-locals.var_vgd_noswap_dn10) * locals.var_t0) - (assign49870_e83012 * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)), (-((assign49870_e83012 * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0))),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign49870_e83016;
        locals.var_t1_dn3 = assign49870_e83016_d_n3;
        locals.var_t1_dn4 = assign49870_e83016_d_n4;
        locals.var_t1_dn5 = assign49870_e83016_d_n5;
        locals.var_t1_dn6 = assign49870_e83016_d_n6;
        locals.var_t1_dn7 = assign49870_e83016_d_n7;
        locals.var_t1_dn8 = assign49870_e83016_d_n8;
        locals.var_t1_dn9 = assign49870_e83016_d_n9;
        locals.var_t1_dn10 = assign49870_e83016_d_n10;
        locals.var_t1_dn11 = assign49870_e83016_d_n11;

        let (assign49880_e83039, assign49880_e83039_d_n3, assign49880_e83039_d_n4, assign49880_e83039_d_n5, assign49880_e83039_d_n6, assign49880_e83039_d_n7, assign49880_e83039_d_n8, assign49880_e83039_d_n9, assign49880_e83039_d_n10, assign49880_e83039_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard759 != 0.0)) && (locals.var_guard760 == 0.0)) {
        let assign49880_e83028: f64 = (locals.var_t1 * locals.var_t1);
        let assign49880_e83031: f64 = (4.0 * 0.01);
        let assign49880_e83033: f64 = (assign49880_e83031 * 0.01);
        let assign49880_e83034: f64 = (assign49880_e83028 + assign49880_e83033);
        let assign49880_e83035: f64 = (assign49880_e83034).sqrt();
        let assign49880_e83036: f64 = (locals.var_t1 + assign49880_e83035);
        let assign49880_e83037: f64 = (0.5 * assign49880_e83036);
        (assign49880_e83037, (0.5 * (locals.var_t1_dn3 + (((locals.var_t1_dn3 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn3)) / (2.0 * assign49880_e83035)))), (0.5 * (locals.var_t1_dn4 + (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) / (2.0 * assign49880_e83035)))), (0.5 * (locals.var_t1_dn5 + (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) / (2.0 * assign49880_e83035)))), (0.5 * (locals.var_t1_dn6 + (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) / (2.0 * assign49880_e83035)))), (0.5 * (locals.var_t1_dn7 + (((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) / (2.0 * assign49880_e83035)))), (0.5 * (locals.var_t1_dn8 + (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) / (2.0 * assign49880_e83035)))), (0.5 * (locals.var_t1_dn9 + (((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) / (2.0 * assign49880_e83035)))), (0.5 * (locals.var_t1_dn10 + (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) / (2.0 * assign49880_e83035)))), (0.5 * (locals.var_t1_dn11 + (((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) / (2.0 * assign49880_e83035)))),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign49880_e83039;
        locals.var_t1_dn3 = assign49880_e83039_d_n3;
        locals.var_t1_dn4 = assign49880_e83039_d_n4;
        locals.var_t1_dn5 = assign49880_e83039_d_n5;
        locals.var_t1_dn6 = assign49880_e83039_d_n6;
        locals.var_t1_dn7 = assign49880_e83039_d_n7;
        locals.var_t1_dn8 = assign49880_e83039_d_n8;
        locals.var_t1_dn9 = assign49880_e83039_d_n9;
        locals.var_t1_dn10 = assign49880_e83039_d_n10;
        locals.var_t1_dn11 = assign49880_e83039_d_n11;

    }

    pub(super) fn stamp_transient_block_169(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign49890_e83053, assign49890_e83053_d_n3, assign49890_e83053_d_n4, assign49890_e83053_d_n5, assign49890_e83053_d_n6, assign49890_e83053_d_n7, assign49890_e83053_d_n8, assign49890_e83053_d_n9, assign49890_e83053_d_n10, assign49890_e83053_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard759 != 0.0)) && (locals.var_guard760 == 0.0)) {
        let assign49890_e83050: f64 = (locals.var_t1 + 0.001);
        let assign49890_e83051: f64 = (locals.var_bgidl_t / assign49890_e83050);
        (assign49890_e83051, (-((locals.var_bgidl_t * locals.var_t1_dn3) / (assign49890_e83050 * assign49890_e83050))), (((locals.var_bgidl_t_dn4 * assign49890_e83050) - (locals.var_bgidl_t * locals.var_t1_dn4)) / (assign49890_e83050 * assign49890_e83050)), (((locals.var_bgidl_t_dn5 * assign49890_e83050) - (locals.var_bgidl_t * locals.var_t1_dn5)) / (assign49890_e83050 * assign49890_e83050)), (-((locals.var_bgidl_t * locals.var_t1_dn6) / (assign49890_e83050 * assign49890_e83050))), (-((locals.var_bgidl_t * locals.var_t1_dn7) / (assign49890_e83050 * assign49890_e83050))), (-((locals.var_bgidl_t * locals.var_t1_dn8) / (assign49890_e83050 * assign49890_e83050))), (-((locals.var_bgidl_t * locals.var_t1_dn9) / (assign49890_e83050 * assign49890_e83050))), (-((locals.var_bgidl_t * locals.var_t1_dn10) / (assign49890_e83050 * assign49890_e83050))), (-((locals.var_bgidl_t * locals.var_t1_dn11) / (assign49890_e83050 * assign49890_e83050))),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign49890_e83053;
        locals.var_t2_dn3 = assign49890_e83053_d_n3;
        locals.var_t2_dn4 = assign49890_e83053_d_n4;
        locals.var_t2_dn5 = assign49890_e83053_d_n5;
        locals.var_t2_dn6 = assign49890_e83053_d_n6;
        locals.var_t2_dn7 = assign49890_e83053_d_n7;
        locals.var_t2_dn8 = assign49890_e83053_d_n8;
        locals.var_t2_dn9 = assign49890_e83053_d_n9;
        locals.var_t2_dn10 = assign49890_e83053_d_n10;
        locals.var_t2_dn11 = assign49890_e83053_d_n11;

        let assign49900_e83056: f64 = if locals.var_cgidl_i != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard761 = assign49900_e83056;

        let (assign49910_e83072, assign49910_e83072_d_n3, assign49910_e83072_d_n4, assign49910_e83072_d_n5, assign49910_e83072_d_n6, assign49910_e83072_d_n7, assign49910_e83072_d_n8, assign49910_e83072_d_n9, assign49910_e83072_d_n10, assign49910_e83072_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard759 != 0.0)) && (locals.var_guard760 == 0.0)) && (locals.var_guard761 != 0.0)) {
        let assign49910_e83068: f64 = (locals.var_vdb_noswap * locals.var_vdb_noswap);
        let assign49910_e83070: f64 = (assign49910_e83068 * locals.var_vdb_noswap);
        (assign49910_e83070, 0.0, 0.0, 0.0, ((((locals.var_vdb_noswap_dn6 * locals.var_vdb_noswap) + (locals.var_vdb_noswap * locals.var_vdb_noswap_dn6)) * locals.var_vdb_noswap) + (assign49910_e83068 * locals.var_vdb_noswap_dn6)), ((((locals.var_vdb_noswap_dn7 * locals.var_vdb_noswap) + (locals.var_vdb_noswap * locals.var_vdb_noswap_dn7)) * locals.var_vdb_noswap) + (assign49910_e83068 * locals.var_vdb_noswap_dn7)), 0.0, 0.0, ((((locals.var_vdb_noswap_dn10 * locals.var_vdb_noswap) + (locals.var_vdb_noswap * locals.var_vdb_noswap_dn10)) * locals.var_vdb_noswap) + (assign49910_e83068 * locals.var_vdb_noswap_dn10)), 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign49910_e83072;
        locals.var_t3_dn3 = assign49910_e83072_d_n3;
        locals.var_t3_dn4 = assign49910_e83072_d_n4;
        locals.var_t3_dn5 = assign49910_e83072_d_n5;
        locals.var_t3_dn6 = assign49910_e83072_d_n6;
        locals.var_t3_dn7 = assign49910_e83072_d_n7;
        locals.var_t3_dn8 = assign49910_e83072_d_n8;
        locals.var_t3_dn9 = assign49910_e83072_d_n9;
        locals.var_t3_dn10 = assign49910_e83072_d_n10;
        locals.var_t3_dn11 = assign49910_e83072_d_n11;

        let (assign49920_e83089, assign49920_e83089_d_n3, assign49920_e83089_d_n4, assign49920_e83089_d_n5, assign49920_e83089_d_n6, assign49920_e83089_d_n7, assign49920_e83089_d_n8, assign49920_e83089_d_n9, assign49920_e83089_d_n10, assign49920_e83089_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard759 != 0.0)) && (locals.var_guard760 == 0.0)) && (locals.var_guard761 != 0.0)) {
        let assign49920_e83084: f64 = (locals.var_t3).abs();
        let assign49920_e83085: f64 = (locals.var_cgidl_i + assign49920_e83084);
        let assign49920_e83087: f64 = (assign49920_e83085 + 0.0001);
        (assign49920_e83087, if locals.var_t3 >= 0.0 { locals.var_t3_dn3 } else { (-locals.var_t3_dn3) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn4 } else { (-locals.var_t3_dn4) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn5 } else { (-locals.var_t3_dn5) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn6 } else { (-locals.var_t3_dn6) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn7 } else { (-locals.var_t3_dn7) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn8 } else { (-locals.var_t3_dn8) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn9 } else { (-locals.var_t3_dn9) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn10 } else { (-locals.var_t3_dn10) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn11 } else { (-locals.var_t3_dn11) },)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign49920_e83089;
        locals.var_t4_dn3 = assign49920_e83089_d_n3;
        locals.var_t4_dn4 = assign49920_e83089_d_n4;
        locals.var_t4_dn5 = assign49920_e83089_d_n5;
        locals.var_t4_dn6 = assign49920_e83089_d_n6;
        locals.var_t4_dn7 = assign49920_e83089_d_n7;
        locals.var_t4_dn8 = assign49920_e83089_d_n8;
        locals.var_t4_dn9 = assign49920_e83089_d_n9;
        locals.var_t4_dn10 = assign49920_e83089_d_n10;
        locals.var_t4_dn11 = assign49920_e83089_d_n11;

        let (assign49930_e83122, assign49930_e83122_d_n3, assign49930_e83122_d_n4, assign49930_e83122_d_n5, assign49930_e83122_d_n6, assign49930_e83122_d_n7, assign49930_e83122_d_n8, assign49930_e83122_d_n9, assign49930_e83122_d_n10, assign49930_e83122_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard759 != 0.0)) && (locals.var_guard760 == 0.0)) && (locals.var_guard761 != 0.0)) {
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_t4;
        let assign49930_e83102: f64 = (locals.var_t3 * __rspice_inv_cse_0);
        let assign49930_e83105: f64 = (locals.var_t3 * __rspice_inv_cse_0);
        let assign49930_e83108: f64 = (locals.var_t3 * __rspice_inv_cse_0);
        let assign49930_e83109: f64 = (assign49930_e83105 * assign49930_e83108);
        let assign49930_e83112: f64 = (4.0 * 1e-6);
        let assign49930_e83114: f64 = (assign49930_e83112 * 1e-6);
        let assign49930_e83115: f64 = (assign49930_e83109 + assign49930_e83114);
        let assign49930_e83116: f64 = (assign49930_e83115).sqrt();
        let assign49930_e83117: f64 = (assign49930_e83102 + assign49930_e83116);
        let assign49930_e83118: f64 = (0.5 * assign49930_e83117);
        let assign49930_e83120: f64 = (assign49930_e83118 - 1e-6);
        (assign49930_e83120, (0.5 * ((((locals.var_t3_dn3 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn3)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn3 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn3)) / (locals.var_t4 * locals.var_t4)) * assign49930_e83108) + (assign49930_e83105 * (((locals.var_t3_dn3 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn3)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign49930_e83116)))), (0.5 * ((((locals.var_t3_dn4 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn4 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4)) * assign49930_e83108) + (assign49930_e83105 * (((locals.var_t3_dn4 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign49930_e83116)))), (0.5 * ((((locals.var_t3_dn5 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn5 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4)) * assign49930_e83108) + (assign49930_e83105 * (((locals.var_t3_dn5 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign49930_e83116)))), (0.5 * ((((locals.var_t3_dn6 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn6 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4)) * assign49930_e83108) + (assign49930_e83105 * (((locals.var_t3_dn6 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign49930_e83116)))), (0.5 * ((((locals.var_t3_dn7 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn7)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn7 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn7)) / (locals.var_t4 * locals.var_t4)) * assign49930_e83108) + (assign49930_e83105 * (((locals.var_t3_dn7 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn7)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign49930_e83116)))), (0.5 * ((((locals.var_t3_dn8 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn8 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4)) * assign49930_e83108) + (assign49930_e83105 * (((locals.var_t3_dn8 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign49930_e83116)))), (0.5 * ((((locals.var_t3_dn9 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn9)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn9 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn9)) / (locals.var_t4 * locals.var_t4)) * assign49930_e83108) + (assign49930_e83105 * (((locals.var_t3_dn9 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn9)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign49930_e83116)))), (0.5 * ((((locals.var_t3_dn10 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn10 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4)) * assign49930_e83108) + (assign49930_e83105 * (((locals.var_t3_dn10 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign49930_e83116)))), (0.5 * ((((locals.var_t3_dn11 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn11)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn11 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn11)) / (locals.var_t4 * locals.var_t4)) * assign49930_e83108) + (assign49930_e83105 * (((locals.var_t3_dn11 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn11)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign49930_e83116)))),)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign49930_e83122;
        locals.var_t5_dn3 = assign49930_e83122_d_n3;
        locals.var_t5_dn4 = assign49930_e83122_d_n4;
        locals.var_t5_dn5 = assign49930_e83122_d_n5;
        locals.var_t5_dn6 = assign49930_e83122_d_n6;
        locals.var_t5_dn7 = assign49930_e83122_d_n7;
        locals.var_t5_dn8 = assign49930_e83122_d_n8;
        locals.var_t5_dn9 = assign49930_e83122_d_n9;
        locals.var_t5_dn10 = assign49930_e83122_d_n10;
        locals.var_t5_dn11 = assign49930_e83122_d_n11;

        let (assign49940_e83135, assign49940_e83135_d_n3, assign49940_e83135_d_n4, assign49940_e83135_d_n5, assign49940_e83135_d_n6, assign49940_e83135_d_n7, assign49940_e83135_d_n8, assign49940_e83135_d_n9, assign49940_e83135_d_n10, assign49940_e83135_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard759 != 0.0)) && (locals.var_guard760 == 0.0)) && (locals.var_guard761 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign49940_e83135;
        locals.var_t5_dn3 = assign49940_e83135_d_n3;
        locals.var_t5_dn4 = assign49940_e83135_d_n4;
        locals.var_t5_dn5 = assign49940_e83135_d_n5;
        locals.var_t5_dn6 = assign49940_e83135_d_n6;
        locals.var_t5_dn7 = assign49940_e83135_d_n7;
        locals.var_t5_dn8 = assign49940_e83135_d_n8;
        locals.var_t5_dn9 = assign49940_e83135_d_n9;
        locals.var_t5_dn10 = assign49940_e83135_d_n10;
        locals.var_t5_dn11 = assign49940_e83135_d_n11;

        let (assign49950_e83155, assign49950_e83155_d_n3, assign49950_e83155_d_n4, assign49950_e83155_d_n5, assign49950_e83155_d_n6, assign49950_e83155_d_n7, assign49950_e83155_d_n8, assign49950_e83155_d_n9, assign49950_e83155_d_n10, assign49950_e83155_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard759 != 0.0)) && (locals.var_guard760 == 0.0)) {
        let assign49950_e83145: f64 = (locals.var_agidl_i * locals.var_wdiod);
        let assign49950_e83147: f64 = (assign49950_e83145 * locals.var_t1);
        let assign49950_e83149: f64 = (-locals.var_t2);
        let assign49950_e83150: f64 = { let limited_exp_arg = assign49950_e83149; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign49950_e83151: f64 = (assign49950_e83147 * assign49950_e83150);
        let assign49950_e83153: f64 = (assign49950_e83151 * locals.var_t5);
        (assign49950_e83153, (((((assign49950_e83145 * locals.var_t1_dn3) * assign49950_e83150) + (assign49950_e83147 * ({ let limited_exp_arg = assign49950_e83149; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn3)))) * locals.var_t5) + (assign49950_e83151 * locals.var_t5_dn3)), (((((assign49950_e83145 * locals.var_t1_dn4) * assign49950_e83150) + (assign49950_e83147 * ({ let limited_exp_arg = assign49950_e83149; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn4)))) * locals.var_t5) + (assign49950_e83151 * locals.var_t5_dn4)), (((((assign49950_e83145 * locals.var_t1_dn5) * assign49950_e83150) + (assign49950_e83147 * ({ let limited_exp_arg = assign49950_e83149; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn5)))) * locals.var_t5) + (assign49950_e83151 * locals.var_t5_dn5)), (((((assign49950_e83145 * locals.var_t1_dn6) * assign49950_e83150) + (assign49950_e83147 * ({ let limited_exp_arg = assign49950_e83149; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn6)))) * locals.var_t5) + (assign49950_e83151 * locals.var_t5_dn6)), (((((assign49950_e83145 * locals.var_t1_dn7) * assign49950_e83150) + (assign49950_e83147 * ({ let limited_exp_arg = assign49950_e83149; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn7)))) * locals.var_t5) + (assign49950_e83151 * locals.var_t5_dn7)), (((((assign49950_e83145 * locals.var_t1_dn8) * assign49950_e83150) + (assign49950_e83147 * ({ let limited_exp_arg = assign49950_e83149; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn8)))) * locals.var_t5) + (assign49950_e83151 * locals.var_t5_dn8)), (((((assign49950_e83145 * locals.var_t1_dn9) * assign49950_e83150) + (assign49950_e83147 * ({ let limited_exp_arg = assign49950_e83149; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn9)))) * locals.var_t5) + (assign49950_e83151 * locals.var_t5_dn9)), (((((assign49950_e83145 * locals.var_t1_dn10) * assign49950_e83150) + (assign49950_e83147 * ({ let limited_exp_arg = assign49950_e83149; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn10)))) * locals.var_t5) + (assign49950_e83151 * locals.var_t5_dn10)), (((((assign49950_e83145 * locals.var_t1_dn11) * assign49950_e83150) + (assign49950_e83147 * ({ let limited_exp_arg = assign49950_e83149; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn11)))) * locals.var_t5) + (assign49950_e83151 * locals.var_t5_dn11)),)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign49950_e83155;
        locals.var_t6_dn3 = assign49950_e83155_d_n3;
        locals.var_t6_dn4 = assign49950_e83155_d_n4;
        locals.var_t6_dn5 = assign49950_e83155_d_n5;
        locals.var_t6_dn6 = assign49950_e83155_d_n6;
        locals.var_t6_dn7 = assign49950_e83155_d_n7;
        locals.var_t6_dn8 = assign49950_e83155_d_n8;
        locals.var_t6_dn9 = assign49950_e83155_d_n9;
        locals.var_t6_dn10 = assign49950_e83155_d_n10;
        locals.var_t6_dn11 = assign49950_e83155_d_n11;

        let (assign49960_e83162, assign49960_e83162_d_n3, assign49960_e83162_d_n4, assign49960_e83162_d_n5, assign49960_e83162_d_n6, assign49960_e83162_d_n7, assign49960_e83162_d_n8, assign49960_e83162_d_n9, assign49960_e83162_d_n10, assign49960_e83162_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard759 != 0.0)) {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    } else {
        (locals.var_igidl, locals.var_igidl_dn3, locals.var_igidl_dn4, locals.var_igidl_dn5, locals.var_igidl_dn6, locals.var_igidl_dn7, locals.var_igidl_dn8, locals.var_igidl_dn9, locals.var_igidl_dn10, locals.var_igidl_dn11,)
    }
};
        locals.var_igidl = assign49960_e83162;
        locals.var_igidl_dn3 = assign49960_e83162_d_n3;
        locals.var_igidl_dn4 = assign49960_e83162_d_n4;
        locals.var_igidl_dn5 = assign49960_e83162_d_n5;
        locals.var_igidl_dn6 = assign49960_e83162_d_n6;
        locals.var_igidl_dn7 = assign49960_e83162_d_n7;
        locals.var_igidl_dn8 = assign49960_e83162_d_n8;
        locals.var_igidl_dn9 = assign49960_e83162_d_n9;
        locals.var_igidl_dn10 = assign49960_e83162_d_n10;
        locals.var_igidl_dn11 = assign49960_e83162_d_n11;

        let assign49970_e83173: f64 = if (((locals.var_agisl_i <= 0.0) || (locals.var_bgisl_t <= 0.0)) || (locals.var_cgisl_i < 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard762 = assign49970_e83173;

        let (assign49980_e83182, assign49980_e83182_d_n3, assign49980_e83182_d_n4, assign49980_e83182_d_n5, assign49980_e83182_d_n6, assign49980_e83182_d_n7, assign49980_e83182_d_n8, assign49980_e83182_d_n9, assign49980_e83182_d_n10, assign49980_e83182_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard759 != 0.0)) && (locals.var_guard762 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign49980_e83182;
        locals.var_t6_dn3 = assign49980_e83182_d_n3;
        locals.var_t6_dn4 = assign49980_e83182_d_n4;
        locals.var_t6_dn5 = assign49980_e83182_d_n5;
        locals.var_t6_dn6 = assign49980_e83182_d_n6;
        locals.var_t6_dn7 = assign49980_e83182_d_n7;
        locals.var_t6_dn8 = assign49980_e83182_d_n8;
        locals.var_t6_dn9 = assign49980_e83182_d_n9;
        locals.var_t6_dn10 = assign49980_e83182_d_n10;
        locals.var_t6_dn11 = assign49980_e83182_d_n11;

        let (assign49990_e83199, assign49990_e83199_d_n3, assign49990_e83199_d_n4, assign49990_e83199_d_n5, assign49990_e83199_d_n6, assign49990_e83199_d_n7, assign49990_e83199_d_n8, assign49990_e83199_d_n9, assign49990_e83199_d_n10, assign49990_e83199_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard759 != 0.0)) && (locals.var_guard762 == 0.0)) {
        let assign49990_e83191: f64 = (-locals.var_vgs_noswap);
        let assign49990_e83193: f64 = (assign49990_e83191 - locals.var_egisl_i);
        let assign49990_e83195: f64 = (assign49990_e83193 + locals.var_vfbsdr);
        let assign49990_e83197: f64 = (assign49990_e83195 / locals.var_t0);
        (assign49990_e83197, (-((assign49990_e83195 * locals.var_t0_dn3) / (locals.var_t0 * locals.var_t0))), (((locals.var_vfbsdr_dn4 * locals.var_t0) - (assign49990_e83195 * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)), (((locals.var_vfbsdr_dn5 * locals.var_t0) - (assign49990_e83195 * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)), ((((-locals.var_vgs_noswap_dn6) * locals.var_t0) - (assign49990_e83195 * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)), ((((-locals.var_vgs_noswap_dn7) * locals.var_t0) - (assign49990_e83195 * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)), ((((-locals.var_vgs_noswap_dn8) * locals.var_t0) - (assign49990_e83195 * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)), (-((assign49990_e83195 * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0))), ((((-locals.var_vgs_noswap_dn10) * locals.var_t0) - (assign49990_e83195 * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)), (-((assign49990_e83195 * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0))),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign49990_e83199;
        locals.var_t1_dn3 = assign49990_e83199_d_n3;
        locals.var_t1_dn4 = assign49990_e83199_d_n4;
        locals.var_t1_dn5 = assign49990_e83199_d_n5;
        locals.var_t1_dn6 = assign49990_e83199_d_n6;
        locals.var_t1_dn7 = assign49990_e83199_d_n7;
        locals.var_t1_dn8 = assign49990_e83199_d_n8;
        locals.var_t1_dn9 = assign49990_e83199_d_n9;
        locals.var_t1_dn10 = assign49990_e83199_d_n10;
        locals.var_t1_dn11 = assign49990_e83199_d_n11;

        let (assign50000_e83222, assign50000_e83222_d_n3, assign50000_e83222_d_n4, assign50000_e83222_d_n5, assign50000_e83222_d_n6, assign50000_e83222_d_n7, assign50000_e83222_d_n8, assign50000_e83222_d_n9, assign50000_e83222_d_n10, assign50000_e83222_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard759 != 0.0)) && (locals.var_guard762 == 0.0)) {
        let assign50000_e83211: f64 = (locals.var_t1 * locals.var_t1);
        let assign50000_e83214: f64 = (4.0 * 0.01);
        let assign50000_e83216: f64 = (assign50000_e83214 * 0.01);
        let assign50000_e83217: f64 = (assign50000_e83211 + assign50000_e83216);
        let assign50000_e83218: f64 = (assign50000_e83217).sqrt();
        let assign50000_e83219: f64 = (locals.var_t1 + assign50000_e83218);
        let assign50000_e83220: f64 = (0.5 * assign50000_e83219);
        (assign50000_e83220, (0.5 * (locals.var_t1_dn3 + (((locals.var_t1_dn3 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn3)) / (2.0 * assign50000_e83218)))), (0.5 * (locals.var_t1_dn4 + (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) / (2.0 * assign50000_e83218)))), (0.5 * (locals.var_t1_dn5 + (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) / (2.0 * assign50000_e83218)))), (0.5 * (locals.var_t1_dn6 + (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) / (2.0 * assign50000_e83218)))), (0.5 * (locals.var_t1_dn7 + (((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) / (2.0 * assign50000_e83218)))), (0.5 * (locals.var_t1_dn8 + (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) / (2.0 * assign50000_e83218)))), (0.5 * (locals.var_t1_dn9 + (((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) / (2.0 * assign50000_e83218)))), (0.5 * (locals.var_t1_dn10 + (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) / (2.0 * assign50000_e83218)))), (0.5 * (locals.var_t1_dn11 + (((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) / (2.0 * assign50000_e83218)))),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign50000_e83222;
        locals.var_t1_dn3 = assign50000_e83222_d_n3;
        locals.var_t1_dn4 = assign50000_e83222_d_n4;
        locals.var_t1_dn5 = assign50000_e83222_d_n5;
        locals.var_t1_dn6 = assign50000_e83222_d_n6;
        locals.var_t1_dn7 = assign50000_e83222_d_n7;
        locals.var_t1_dn8 = assign50000_e83222_d_n8;
        locals.var_t1_dn9 = assign50000_e83222_d_n9;
        locals.var_t1_dn10 = assign50000_e83222_d_n10;
        locals.var_t1_dn11 = assign50000_e83222_d_n11;

        let (assign50010_e83236, assign50010_e83236_d_n3, assign50010_e83236_d_n4, assign50010_e83236_d_n5, assign50010_e83236_d_n6, assign50010_e83236_d_n7, assign50010_e83236_d_n8, assign50010_e83236_d_n9, assign50010_e83236_d_n10, assign50010_e83236_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard759 != 0.0)) && (locals.var_guard762 == 0.0)) {
        let assign50010_e83233: f64 = (locals.var_t1 + 0.001);
        let assign50010_e83234: f64 = (locals.var_bgisl_t / assign50010_e83233);
        (assign50010_e83234, (-((locals.var_bgisl_t * locals.var_t1_dn3) / (assign50010_e83233 * assign50010_e83233))), (((locals.var_bgisl_t_dn4 * assign50010_e83233) - (locals.var_bgisl_t * locals.var_t1_dn4)) / (assign50010_e83233 * assign50010_e83233)), (((locals.var_bgisl_t_dn5 * assign50010_e83233) - (locals.var_bgisl_t * locals.var_t1_dn5)) / (assign50010_e83233 * assign50010_e83233)), (-((locals.var_bgisl_t * locals.var_t1_dn6) / (assign50010_e83233 * assign50010_e83233))), (-((locals.var_bgisl_t * locals.var_t1_dn7) / (assign50010_e83233 * assign50010_e83233))), (-((locals.var_bgisl_t * locals.var_t1_dn8) / (assign50010_e83233 * assign50010_e83233))), (-((locals.var_bgisl_t * locals.var_t1_dn9) / (assign50010_e83233 * assign50010_e83233))), (-((locals.var_bgisl_t * locals.var_t1_dn10) / (assign50010_e83233 * assign50010_e83233))), (-((locals.var_bgisl_t * locals.var_t1_dn11) / (assign50010_e83233 * assign50010_e83233))),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign50010_e83236;
        locals.var_t2_dn3 = assign50010_e83236_d_n3;
        locals.var_t2_dn4 = assign50010_e83236_d_n4;
        locals.var_t2_dn5 = assign50010_e83236_d_n5;
        locals.var_t2_dn6 = assign50010_e83236_d_n6;
        locals.var_t2_dn7 = assign50010_e83236_d_n7;
        locals.var_t2_dn8 = assign50010_e83236_d_n8;
        locals.var_t2_dn9 = assign50010_e83236_d_n9;
        locals.var_t2_dn10 = assign50010_e83236_d_n10;
        locals.var_t2_dn11 = assign50010_e83236_d_n11;

        let assign50020_e83239: f64 = if locals.var_cgisl_i != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard763 = assign50020_e83239;

        let (assign50030_e83255, assign50030_e83255_d_n3, assign50030_e83255_d_n4, assign50030_e83255_d_n5, assign50030_e83255_d_n6, assign50030_e83255_d_n7, assign50030_e83255_d_n8, assign50030_e83255_d_n9, assign50030_e83255_d_n10, assign50030_e83255_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard759 != 0.0)) && (locals.var_guard762 == 0.0)) && (locals.var_guard763 != 0.0)) {
        let assign50030_e83251: f64 = (locals.var_vsb_noswap * locals.var_vsb_noswap);
        let assign50030_e83253: f64 = (assign50030_e83251 * locals.var_vsb_noswap);
        (assign50030_e83253, 0.0, 0.0, 0.0, ((((locals.var_vsb_noswap_dn6 * locals.var_vsb_noswap) + (locals.var_vsb_noswap * locals.var_vsb_noswap_dn6)) * locals.var_vsb_noswap) + (assign50030_e83251 * locals.var_vsb_noswap_dn6)), ((((locals.var_vsb_noswap_dn7 * locals.var_vsb_noswap) + (locals.var_vsb_noswap * locals.var_vsb_noswap_dn7)) * locals.var_vsb_noswap) + (assign50030_e83251 * locals.var_vsb_noswap_dn7)), 0.0, 0.0, ((((locals.var_vsb_noswap_dn10 * locals.var_vsb_noswap) + (locals.var_vsb_noswap * locals.var_vsb_noswap_dn10)) * locals.var_vsb_noswap) + (assign50030_e83251 * locals.var_vsb_noswap_dn10)), 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign50030_e83255;
        locals.var_t3_dn3 = assign50030_e83255_d_n3;
        locals.var_t3_dn4 = assign50030_e83255_d_n4;
        locals.var_t3_dn5 = assign50030_e83255_d_n5;
        locals.var_t3_dn6 = assign50030_e83255_d_n6;
        locals.var_t3_dn7 = assign50030_e83255_d_n7;
        locals.var_t3_dn8 = assign50030_e83255_d_n8;
        locals.var_t3_dn9 = assign50030_e83255_d_n9;
        locals.var_t3_dn10 = assign50030_e83255_d_n10;
        locals.var_t3_dn11 = assign50030_e83255_d_n11;

        let (assign50040_e83272, assign50040_e83272_d_n3, assign50040_e83272_d_n4, assign50040_e83272_d_n5, assign50040_e83272_d_n6, assign50040_e83272_d_n7, assign50040_e83272_d_n8, assign50040_e83272_d_n9, assign50040_e83272_d_n10, assign50040_e83272_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard759 != 0.0)) && (locals.var_guard762 == 0.0)) && (locals.var_guard763 != 0.0)) {
        let assign50040_e83267: f64 = (locals.var_t3).abs();
        let assign50040_e83268: f64 = (locals.var_cgisl_i + assign50040_e83267);
        let assign50040_e83270: f64 = (assign50040_e83268 + 0.0001);
        (assign50040_e83270, if locals.var_t3 >= 0.0 { locals.var_t3_dn3 } else { (-locals.var_t3_dn3) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn4 } else { (-locals.var_t3_dn4) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn5 } else { (-locals.var_t3_dn5) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn6 } else { (-locals.var_t3_dn6) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn7 } else { (-locals.var_t3_dn7) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn8 } else { (-locals.var_t3_dn8) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn9 } else { (-locals.var_t3_dn9) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn10 } else { (-locals.var_t3_dn10) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn11 } else { (-locals.var_t3_dn11) },)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign50040_e83272;
        locals.var_t4_dn3 = assign50040_e83272_d_n3;
        locals.var_t4_dn4 = assign50040_e83272_d_n4;
        locals.var_t4_dn5 = assign50040_e83272_d_n5;
        locals.var_t4_dn6 = assign50040_e83272_d_n6;
        locals.var_t4_dn7 = assign50040_e83272_d_n7;
        locals.var_t4_dn8 = assign50040_e83272_d_n8;
        locals.var_t4_dn9 = assign50040_e83272_d_n9;
        locals.var_t4_dn10 = assign50040_e83272_d_n10;
        locals.var_t4_dn11 = assign50040_e83272_d_n11;

        let (assign50050_e83305, assign50050_e83305_d_n3, assign50050_e83305_d_n4, assign50050_e83305_d_n5, assign50050_e83305_d_n6, assign50050_e83305_d_n7, assign50050_e83305_d_n8, assign50050_e83305_d_n9, assign50050_e83305_d_n10, assign50050_e83305_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard759 != 0.0)) && (locals.var_guard762 == 0.0)) && (locals.var_guard763 != 0.0)) {
        let __rspice_inv_cse_1: f64 = 1.0 / locals.var_t4;
        let assign50050_e83285: f64 = (locals.var_t3 * __rspice_inv_cse_1);
        let assign50050_e83288: f64 = (locals.var_t3 * __rspice_inv_cse_1);
        let assign50050_e83291: f64 = (locals.var_t3 * __rspice_inv_cse_1);
        let assign50050_e83292: f64 = (assign50050_e83288 * assign50050_e83291);
        let assign50050_e83295: f64 = (4.0 * 1e-6);
        let assign50050_e83297: f64 = (assign50050_e83295 * 1e-6);
        let assign50050_e83298: f64 = (assign50050_e83292 + assign50050_e83297);
        let assign50050_e83299: f64 = (assign50050_e83298).sqrt();
        let assign50050_e83300: f64 = (assign50050_e83285 + assign50050_e83299);
        let assign50050_e83301: f64 = (0.5 * assign50050_e83300);
        let assign50050_e83303: f64 = (assign50050_e83301 - 1e-6);
        (assign50050_e83303, (0.5 * ((((locals.var_t3_dn3 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn3)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn3 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn3)) / (locals.var_t4 * locals.var_t4)) * assign50050_e83291) + (assign50050_e83288 * (((locals.var_t3_dn3 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn3)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign50050_e83299)))), (0.5 * ((((locals.var_t3_dn4 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn4 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4)) * assign50050_e83291) + (assign50050_e83288 * (((locals.var_t3_dn4 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign50050_e83299)))), (0.5 * ((((locals.var_t3_dn5 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn5 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4)) * assign50050_e83291) + (assign50050_e83288 * (((locals.var_t3_dn5 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign50050_e83299)))), (0.5 * ((((locals.var_t3_dn6 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn6 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4)) * assign50050_e83291) + (assign50050_e83288 * (((locals.var_t3_dn6 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign50050_e83299)))), (0.5 * ((((locals.var_t3_dn7 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn7)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn7 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn7)) / (locals.var_t4 * locals.var_t4)) * assign50050_e83291) + (assign50050_e83288 * (((locals.var_t3_dn7 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn7)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign50050_e83299)))), (0.5 * ((((locals.var_t3_dn8 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn8 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4)) * assign50050_e83291) + (assign50050_e83288 * (((locals.var_t3_dn8 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign50050_e83299)))), (0.5 * ((((locals.var_t3_dn9 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn9)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn9 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn9)) / (locals.var_t4 * locals.var_t4)) * assign50050_e83291) + (assign50050_e83288 * (((locals.var_t3_dn9 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn9)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign50050_e83299)))), (0.5 * ((((locals.var_t3_dn10 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn10 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4)) * assign50050_e83291) + (assign50050_e83288 * (((locals.var_t3_dn10 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign50050_e83299)))), (0.5 * ((((locals.var_t3_dn11 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn11)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn11 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn11)) / (locals.var_t4 * locals.var_t4)) * assign50050_e83291) + (assign50050_e83288 * (((locals.var_t3_dn11 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn11)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign50050_e83299)))),)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign50050_e83305;
        locals.var_t5_dn3 = assign50050_e83305_d_n3;
        locals.var_t5_dn4 = assign50050_e83305_d_n4;
        locals.var_t5_dn5 = assign50050_e83305_d_n5;
        locals.var_t5_dn6 = assign50050_e83305_d_n6;
        locals.var_t5_dn7 = assign50050_e83305_d_n7;
        locals.var_t5_dn8 = assign50050_e83305_d_n8;
        locals.var_t5_dn9 = assign50050_e83305_d_n9;
        locals.var_t5_dn10 = assign50050_e83305_d_n10;
        locals.var_t5_dn11 = assign50050_e83305_d_n11;

        let (assign50060_e83318, assign50060_e83318_d_n3, assign50060_e83318_d_n4, assign50060_e83318_d_n5, assign50060_e83318_d_n6, assign50060_e83318_d_n7, assign50060_e83318_d_n8, assign50060_e83318_d_n9, assign50060_e83318_d_n10, assign50060_e83318_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard759 != 0.0)) && (locals.var_guard762 == 0.0)) && (locals.var_guard763 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign50060_e83318;
        locals.var_t5_dn3 = assign50060_e83318_d_n3;
        locals.var_t5_dn4 = assign50060_e83318_d_n4;
        locals.var_t5_dn5 = assign50060_e83318_d_n5;
        locals.var_t5_dn6 = assign50060_e83318_d_n6;
        locals.var_t5_dn7 = assign50060_e83318_d_n7;
        locals.var_t5_dn8 = assign50060_e83318_d_n8;
        locals.var_t5_dn9 = assign50060_e83318_d_n9;
        locals.var_t5_dn10 = assign50060_e83318_d_n10;
        locals.var_t5_dn11 = assign50060_e83318_d_n11;

        let (assign50070_e83338, assign50070_e83338_d_n3, assign50070_e83338_d_n4, assign50070_e83338_d_n5, assign50070_e83338_d_n6, assign50070_e83338_d_n7, assign50070_e83338_d_n8, assign50070_e83338_d_n9, assign50070_e83338_d_n10, assign50070_e83338_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard759 != 0.0)) && (locals.var_guard762 == 0.0)) {
        let assign50070_e83328: f64 = (locals.var_agisl_i * locals.var_wdios);
        let assign50070_e83330: f64 = (assign50070_e83328 * locals.var_t1);
        let assign50070_e83332: f64 = (-locals.var_t2);
        let assign50070_e83333: f64 = { let limited_exp_arg = assign50070_e83332; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign50070_e83334: f64 = (assign50070_e83330 * assign50070_e83333);
        let assign50070_e83336: f64 = (assign50070_e83334 * locals.var_t5);
        (assign50070_e83336, (((((assign50070_e83328 * locals.var_t1_dn3) * assign50070_e83333) + (assign50070_e83330 * ({ let limited_exp_arg = assign50070_e83332; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn3)))) * locals.var_t5) + (assign50070_e83334 * locals.var_t5_dn3)), (((((assign50070_e83328 * locals.var_t1_dn4) * assign50070_e83333) + (assign50070_e83330 * ({ let limited_exp_arg = assign50070_e83332; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn4)))) * locals.var_t5) + (assign50070_e83334 * locals.var_t5_dn4)), (((((assign50070_e83328 * locals.var_t1_dn5) * assign50070_e83333) + (assign50070_e83330 * ({ let limited_exp_arg = assign50070_e83332; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn5)))) * locals.var_t5) + (assign50070_e83334 * locals.var_t5_dn5)), (((((assign50070_e83328 * locals.var_t1_dn6) * assign50070_e83333) + (assign50070_e83330 * ({ let limited_exp_arg = assign50070_e83332; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn6)))) * locals.var_t5) + (assign50070_e83334 * locals.var_t5_dn6)), (((((assign50070_e83328 * locals.var_t1_dn7) * assign50070_e83333) + (assign50070_e83330 * ({ let limited_exp_arg = assign50070_e83332; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn7)))) * locals.var_t5) + (assign50070_e83334 * locals.var_t5_dn7)), (((((assign50070_e83328 * locals.var_t1_dn8) * assign50070_e83333) + (assign50070_e83330 * ({ let limited_exp_arg = assign50070_e83332; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn8)))) * locals.var_t5) + (assign50070_e83334 * locals.var_t5_dn8)), (((((assign50070_e83328 * locals.var_t1_dn9) * assign50070_e83333) + (assign50070_e83330 * ({ let limited_exp_arg = assign50070_e83332; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn9)))) * locals.var_t5) + (assign50070_e83334 * locals.var_t5_dn9)), (((((assign50070_e83328 * locals.var_t1_dn10) * assign50070_e83333) + (assign50070_e83330 * ({ let limited_exp_arg = assign50070_e83332; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn10)))) * locals.var_t5) + (assign50070_e83334 * locals.var_t5_dn10)), (((((assign50070_e83328 * locals.var_t1_dn11) * assign50070_e83333) + (assign50070_e83330 * ({ let limited_exp_arg = assign50070_e83332; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn11)))) * locals.var_t5) + (assign50070_e83334 * locals.var_t5_dn11)),)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign50070_e83338;
        locals.var_t6_dn3 = assign50070_e83338_d_n3;
        locals.var_t6_dn4 = assign50070_e83338_d_n4;
        locals.var_t6_dn5 = assign50070_e83338_d_n5;
        locals.var_t6_dn6 = assign50070_e83338_d_n6;
        locals.var_t6_dn7 = assign50070_e83338_d_n7;
        locals.var_t6_dn8 = assign50070_e83338_d_n8;
        locals.var_t6_dn9 = assign50070_e83338_d_n9;
        locals.var_t6_dn10 = assign50070_e83338_d_n10;
        locals.var_t6_dn11 = assign50070_e83338_d_n11;

        let (assign50080_e83345, assign50080_e83345_d_n3, assign50080_e83345_d_n4, assign50080_e83345_d_n5, assign50080_e83345_d_n6, assign50080_e83345_d_n7, assign50080_e83345_d_n8, assign50080_e83345_d_n9, assign50080_e83345_d_n10, assign50080_e83345_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard759 != 0.0)) {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    } else {
        (locals.var_igisl, locals.var_igisl_dn3, locals.var_igisl_dn4, locals.var_igisl_dn5, locals.var_igisl_dn6, locals.var_igisl_dn7, locals.var_igisl_dn8, locals.var_igisl_dn9, locals.var_igisl_dn10, locals.var_igisl_dn11,)
    }
};
        locals.var_igisl = assign50080_e83345;
        locals.var_igisl_dn3 = assign50080_e83345_d_n3;
        locals.var_igisl_dn4 = assign50080_e83345_d_n4;
        locals.var_igisl_dn5 = assign50080_e83345_d_n5;
        locals.var_igisl_dn6 = assign50080_e83345_d_n6;
        locals.var_igisl_dn7 = assign50080_e83345_d_n7;
        locals.var_igisl_dn8 = assign50080_e83345_d_n8;
        locals.var_igisl_dn9 = assign50080_e83345_d_n9;
        locals.var_igisl_dn10 = assign50080_e83345_d_n10;
        locals.var_igisl_dn11 = assign50080_e83345_d_n11;

        let (assign50090_e83355, assign50090_e83355_d_n3, assign50090_e83355_d_n4, assign50090_e83355_d_n5, assign50090_e83355_d_n6, assign50090_e83355_d_n7, assign50090_e83355_d_n8, assign50090_e83355_d_n9, assign50090_e83355_d_n10, assign50090_e83355_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard759 == 0.0)) {
        let assign50090_e83353: f64 = (locals.var_epsratio * p.p76);
        (assign50090_e83353, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign50090_e83355;
        locals.var_t0_dn3 = assign50090_e83355_d_n3;
        locals.var_t0_dn4 = assign50090_e83355_d_n4;
        locals.var_t0_dn5 = assign50090_e83355_d_n5;
        locals.var_t0_dn6 = assign50090_e83355_d_n6;
        locals.var_t0_dn7 = assign50090_e83355_d_n7;
        locals.var_t0_dn8 = assign50090_e83355_d_n8;
        locals.var_t0_dn9 = assign50090_e83355_d_n9;
        locals.var_t0_dn10 = assign50090_e83355_d_n10;
        locals.var_t0_dn11 = assign50090_e83355_d_n11;

        let (assign50100_e83367, assign50100_e83367_d_n6, assign50100_e83367_d_n7, assign50100_e83367_d_n8, assign50100_e83367_d_n10,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard759 == 0.0)) {
        let assign50100_e83363: f64 = (locals.var_rgisl_i * locals.var_vg);
        let assign50100_e83365: f64 = (assign50100_e83363 - locals.var_vd);
        (assign50100_e83365, (-locals.var_vd_dn6), (-locals.var_vd_dn7), (locals.var_rgisl_i * locals.var_vg_dn8), ((locals.var_rgisl_i * locals.var_vg_dn10) - locals.var_vd_dn10),)
    } else {
        (locals.var_vgd_noswap_1, locals.var_vgd_noswap_1_dn6, locals.var_vgd_noswap_1_dn7, locals.var_vgd_noswap_1_dn8, locals.var_vgd_noswap_1_dn10,)
    }
};
        locals.var_vgd_noswap_1 = assign50100_e83367;
        locals.var_vgd_noswap_1_dn6 = assign50100_e83367_d_n6;
        locals.var_vgd_noswap_1_dn7 = assign50100_e83367_d_n7;
        locals.var_vgd_noswap_1_dn8 = assign50100_e83367_d_n8;
        locals.var_vgd_noswap_1_dn10 = assign50100_e83367_d_n10;

        let (assign50110_e83379, assign50110_e83379_d_n6, assign50110_e83379_d_n7, assign50110_e83379_d_n8, assign50110_e83379_d_n10,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard759 == 0.0)) {
        let assign50110_e83375: f64 = (locals.var_rgidl_i * locals.var_vg);
        let assign50110_e83377: f64 = (assign50110_e83375 - locals.var_vs);
        (assign50110_e83377, (-locals.var_vs_dn6), (-locals.var_vs_dn7), (locals.var_rgidl_i * locals.var_vg_dn8), ((locals.var_rgidl_i * locals.var_vg_dn10) - locals.var_vs_dn10),)
    } else {
        (locals.var_vgs_noswap_1, locals.var_vgs_noswap_1_dn6, locals.var_vgs_noswap_1_dn7, locals.var_vgs_noswap_1_dn8, locals.var_vgs_noswap_1_dn10,)
    }
};
        locals.var_vgs_noswap_1 = assign50110_e83379;
        locals.var_vgs_noswap_1_dn6 = assign50110_e83379_d_n6;
        locals.var_vgs_noswap_1_dn7 = assign50110_e83379_d_n7;
        locals.var_vgs_noswap_1_dn8 = assign50110_e83379_d_n8;
        locals.var_vgs_noswap_1_dn10 = assign50110_e83379_d_n10;

        let (assign50120_e83389, assign50120_e83389_d_n3, assign50120_e83389_d_n4, assign50120_e83389_d_n5, assign50120_e83389_d_n6, assign50120_e83389_d_n7, assign50120_e83389_d_n8, assign50120_e83389_d_n9, assign50120_e83389_d_n10, assign50120_e83389_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard759 == 0.0)) {
        let assign50120_e83387: f64 = (locals.var_vgs_noswap - locals.var_vfbsdr);
        (assign50120_e83387, 0.0, (-locals.var_vfbsdr_dn4), (-locals.var_vfbsdr_dn5), locals.var_vgs_noswap_dn6, locals.var_vgs_noswap_dn7, locals.var_vgs_noswap_dn8, 0.0, locals.var_vgs_noswap_dn10, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign50120_e83389;
        locals.var_t2_dn3 = assign50120_e83389_d_n3;
        locals.var_t2_dn4 = assign50120_e83389_d_n4;
        locals.var_t2_dn5 = assign50120_e83389_d_n5;
        locals.var_t2_dn6 = assign50120_e83389_d_n6;
        locals.var_t2_dn7 = assign50120_e83389_d_n7;
        locals.var_t2_dn8 = assign50120_e83389_d_n8;
        locals.var_t2_dn9 = assign50120_e83389_d_n9;
        locals.var_t2_dn10 = assign50120_e83389_d_n10;
        locals.var_t2_dn11 = assign50120_e83389_d_n11;

        let (assign50130_e83402, assign50130_e83402_d_n3, assign50130_e83402_d_n4, assign50130_e83402_d_n5, assign50130_e83402_d_n6, assign50130_e83402_d_n7, assign50130_e83402_d_n8, assign50130_e83402_d_n9, assign50130_e83402_d_n10, assign50130_e83402_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard759 == 0.0)) {
        let assign50130_e83397: f64 = (locals.var_t2 * locals.var_t2);
        let assign50130_e83399: f64 = (assign50130_e83397 + 0.0001);
        let assign50130_e83400: f64 = (assign50130_e83399).sqrt();
        (assign50130_e83400, (((locals.var_t2_dn3 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn3)) / (2.0 * assign50130_e83400)), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign50130_e83400)), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign50130_e83400)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign50130_e83400)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign50130_e83400)), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign50130_e83400)), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign50130_e83400)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign50130_e83400)), (((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign50130_e83400)),)
    } else {
        (locals.var_vgs_eff, locals.var_vgs_eff_dn3, locals.var_vgs_eff_dn4, locals.var_vgs_eff_dn5, locals.var_vgs_eff_dn6, locals.var_vgs_eff_dn7, locals.var_vgs_eff_dn8, locals.var_vgs_eff_dn9, locals.var_vgs_eff_dn10, locals.var_vgs_eff_dn11,)
    }
};
        locals.var_vgs_eff = assign50130_e83402;
        locals.var_vgs_eff_dn3 = assign50130_e83402_d_n3;
        locals.var_vgs_eff_dn4 = assign50130_e83402_d_n4;
        locals.var_vgs_eff_dn5 = assign50130_e83402_d_n5;
        locals.var_vgs_eff_dn6 = assign50130_e83402_d_n6;
        locals.var_vgs_eff_dn7 = assign50130_e83402_d_n7;
        locals.var_vgs_eff_dn8 = assign50130_e83402_d_n8;
        locals.var_vgs_eff_dn9 = assign50130_e83402_d_n9;
        locals.var_vgs_eff_dn10 = assign50130_e83402_d_n10;
        locals.var_vgs_eff_dn11 = assign50130_e83402_d_n11;

        let assign50140_e83409: f64 = if ((locals.var_agidl_i <= 0.0) || (locals.var_bgidl_t <= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard764 = assign50140_e83409;

        let (assign50150_e83419, assign50150_e83419_d_n3, assign50150_e83419_d_n4, assign50150_e83419_d_n5, assign50150_e83419_d_n6, assign50150_e83419_d_n7, assign50150_e83419_d_n8, assign50150_e83419_d_n9, assign50150_e83419_d_n10, assign50150_e83419_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard759 == 0.0)) && (locals.var_guard764 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign50150_e83419;
        locals.var_t6_dn3 = assign50150_e83419_d_n3;
        locals.var_t6_dn4 = assign50150_e83419_d_n4;
        locals.var_t6_dn5 = assign50150_e83419_d_n5;
        locals.var_t6_dn6 = assign50150_e83419_d_n6;
        locals.var_t6_dn7 = assign50150_e83419_d_n7;
        locals.var_t6_dn8 = assign50150_e83419_d_n8;
        locals.var_t6_dn9 = assign50150_e83419_d_n9;
        locals.var_t6_dn10 = assign50150_e83419_d_n10;
        locals.var_t6_dn11 = assign50150_e83419_d_n11;

        let (assign50160_e83437, assign50160_e83437_d_n3, assign50160_e83437_d_n4, assign50160_e83437_d_n5, assign50160_e83437_d_n6, assign50160_e83437_d_n7, assign50160_e83437_d_n8, assign50160_e83437_d_n9, assign50160_e83437_d_n10, assign50160_e83437_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard759 == 0.0)) && (locals.var_guard764 == 0.0)) {
        let assign50160_e83429: f64 = (-locals.var_vgd_noswap_1);
        let assign50160_e83431: f64 = (assign50160_e83429 - locals.var_egidl_i);
        let assign50160_e83433: f64 = (assign50160_e83431 + locals.var_vfbsdr);
        let assign50160_e83435: f64 = (assign50160_e83433 / locals.var_t0);
        (assign50160_e83435, (-((assign50160_e83433 * locals.var_t0_dn3) / (locals.var_t0 * locals.var_t0))), (((locals.var_vfbsdr_dn4 * locals.var_t0) - (assign50160_e83433 * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)), (((locals.var_vfbsdr_dn5 * locals.var_t0) - (assign50160_e83433 * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)), ((((-locals.var_vgd_noswap_1_dn6) * locals.var_t0) - (assign50160_e83433 * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)), ((((-locals.var_vgd_noswap_1_dn7) * locals.var_t0) - (assign50160_e83433 * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)), ((((-locals.var_vgd_noswap_1_dn8) * locals.var_t0) - (assign50160_e83433 * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)), (-((assign50160_e83433 * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0))), ((((-locals.var_vgd_noswap_1_dn10) * locals.var_t0) - (assign50160_e83433 * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)), (-((assign50160_e83433 * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0))),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign50160_e83437;
        locals.var_t1_dn3 = assign50160_e83437_d_n3;
        locals.var_t1_dn4 = assign50160_e83437_d_n4;
        locals.var_t1_dn5 = assign50160_e83437_d_n5;
        locals.var_t1_dn6 = assign50160_e83437_d_n6;
        locals.var_t1_dn7 = assign50160_e83437_d_n7;
        locals.var_t1_dn8 = assign50160_e83437_d_n8;
        locals.var_t1_dn9 = assign50160_e83437_d_n9;
        locals.var_t1_dn10 = assign50160_e83437_d_n10;
        locals.var_t1_dn11 = assign50160_e83437_d_n11;

    }

    pub(super) fn stamp_transient_block_170(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign50170_e83461, assign50170_e83461_d_n3, assign50170_e83461_d_n4, assign50170_e83461_d_n5, assign50170_e83461_d_n6, assign50170_e83461_d_n7, assign50170_e83461_d_n8, assign50170_e83461_d_n9, assign50170_e83461_d_n10, assign50170_e83461_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard759 == 0.0)) && (locals.var_guard764 == 0.0)) {
        let assign50170_e83450: f64 = (locals.var_t1 * locals.var_t1);
        let assign50170_e83453: f64 = (4.0 * 0.01);
        let assign50170_e83455: f64 = (assign50170_e83453 * 0.01);
        let assign50170_e83456: f64 = (assign50170_e83450 + assign50170_e83455);
        let assign50170_e83457: f64 = (assign50170_e83456).sqrt();
        let assign50170_e83458: f64 = (locals.var_t1 + assign50170_e83457);
        let assign50170_e83459: f64 = (0.5 * assign50170_e83458);
        (assign50170_e83459, (0.5 * (locals.var_t1_dn3 + (((locals.var_t1_dn3 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn3)) / (2.0 * assign50170_e83457)))), (0.5 * (locals.var_t1_dn4 + (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) / (2.0 * assign50170_e83457)))), (0.5 * (locals.var_t1_dn5 + (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) / (2.0 * assign50170_e83457)))), (0.5 * (locals.var_t1_dn6 + (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) / (2.0 * assign50170_e83457)))), (0.5 * (locals.var_t1_dn7 + (((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) / (2.0 * assign50170_e83457)))), (0.5 * (locals.var_t1_dn8 + (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) / (2.0 * assign50170_e83457)))), (0.5 * (locals.var_t1_dn9 + (((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) / (2.0 * assign50170_e83457)))), (0.5 * (locals.var_t1_dn10 + (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) / (2.0 * assign50170_e83457)))), (0.5 * (locals.var_t1_dn11 + (((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) / (2.0 * assign50170_e83457)))),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign50170_e83461;
        locals.var_t1_dn3 = assign50170_e83461_d_n3;
        locals.var_t1_dn4 = assign50170_e83461_d_n4;
        locals.var_t1_dn5 = assign50170_e83461_d_n5;
        locals.var_t1_dn6 = assign50170_e83461_d_n6;
        locals.var_t1_dn7 = assign50170_e83461_d_n7;
        locals.var_t1_dn8 = assign50170_e83461_d_n8;
        locals.var_t1_dn9 = assign50170_e83461_d_n9;
        locals.var_t1_dn10 = assign50170_e83461_d_n10;
        locals.var_t1_dn11 = assign50170_e83461_d_n11;

        let (assign50180_e83476, assign50180_e83476_d_n3, assign50180_e83476_d_n4, assign50180_e83476_d_n5, assign50180_e83476_d_n6, assign50180_e83476_d_n7, assign50180_e83476_d_n8, assign50180_e83476_d_n9, assign50180_e83476_d_n10, assign50180_e83476_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard759 == 0.0)) && (locals.var_guard764 == 0.0)) {
        let assign50180_e83473: f64 = (locals.var_t1 + 0.001);
        let assign50180_e83474: f64 = (locals.var_bgidl_t / assign50180_e83473);
        (assign50180_e83474, (-((locals.var_bgidl_t * locals.var_t1_dn3) / (assign50180_e83473 * assign50180_e83473))), (((locals.var_bgidl_t_dn4 * assign50180_e83473) - (locals.var_bgidl_t * locals.var_t1_dn4)) / (assign50180_e83473 * assign50180_e83473)), (((locals.var_bgidl_t_dn5 * assign50180_e83473) - (locals.var_bgidl_t * locals.var_t1_dn5)) / (assign50180_e83473 * assign50180_e83473)), (-((locals.var_bgidl_t * locals.var_t1_dn6) / (assign50180_e83473 * assign50180_e83473))), (-((locals.var_bgidl_t * locals.var_t1_dn7) / (assign50180_e83473 * assign50180_e83473))), (-((locals.var_bgidl_t * locals.var_t1_dn8) / (assign50180_e83473 * assign50180_e83473))), (-((locals.var_bgidl_t * locals.var_t1_dn9) / (assign50180_e83473 * assign50180_e83473))), (-((locals.var_bgidl_t * locals.var_t1_dn10) / (assign50180_e83473 * assign50180_e83473))), (-((locals.var_bgidl_t * locals.var_t1_dn11) / (assign50180_e83473 * assign50180_e83473))),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign50180_e83476;
        locals.var_t2_dn3 = assign50180_e83476_d_n3;
        locals.var_t2_dn4 = assign50180_e83476_d_n4;
        locals.var_t2_dn5 = assign50180_e83476_d_n5;
        locals.var_t2_dn6 = assign50180_e83476_d_n6;
        locals.var_t2_dn7 = assign50180_e83476_d_n7;
        locals.var_t2_dn8 = assign50180_e83476_d_n8;
        locals.var_t2_dn9 = assign50180_e83476_d_n9;
        locals.var_t2_dn10 = assign50180_e83476_d_n10;
        locals.var_t2_dn11 = assign50180_e83476_d_n11;

        let assign50190_e83479: f64 = if locals.var_kgidl_i != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard765 = assign50190_e83479;

        let (assign50200_e83495, assign50200_e83495_d_n3, assign50200_e83495_d_n4, assign50200_e83495_d_n5, assign50200_e83495_d_n6, assign50200_e83495_d_n7, assign50200_e83495_d_n8, assign50200_e83495_d_n9, assign50200_e83495_d_n10, assign50200_e83495_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard759 == 0.0)) && (locals.var_guard764 == 0.0)) && (locals.var_guard765 != 0.0)) {
        let assign50200_e83491: f64 = (-locals.var_vdb_noswap);
        let assign50200_e83493: f64 = (assign50200_e83491 - locals.var_fgidl_i);
        (assign50200_e83493, 0.0, 0.0, 0.0, (-locals.var_vdb_noswap_dn6), (-locals.var_vdb_noswap_dn7), 0.0, 0.0, (-locals.var_vdb_noswap_dn10), 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign50200_e83495;
        locals.var_t3_dn3 = assign50200_e83495_d_n3;
        locals.var_t3_dn4 = assign50200_e83495_d_n4;
        locals.var_t3_dn5 = assign50200_e83495_d_n5;
        locals.var_t3_dn6 = assign50200_e83495_d_n6;
        locals.var_t3_dn7 = assign50200_e83495_d_n7;
        locals.var_t3_dn8 = assign50200_e83495_d_n8;
        locals.var_t3_dn9 = assign50200_e83495_d_n9;
        locals.var_t3_dn10 = assign50200_e83495_d_n10;
        locals.var_t3_dn11 = assign50200_e83495_d_n11;

        let (assign50210_e83510, assign50210_e83510_d_n3, assign50210_e83510_d_n4, assign50210_e83510_d_n5, assign50210_e83510_d_n6, assign50210_e83510_d_n7, assign50210_e83510_d_n8, assign50210_e83510_d_n9, assign50210_e83510_d_n10, assign50210_e83510_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard759 == 0.0)) && (locals.var_guard764 == 0.0)) && (locals.var_guard765 != 0.0)) {
        let assign50210_e83508: f64 = (locals.var_t3 + 0.0001);
        (assign50210_e83508, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign50210_e83510;
        locals.var_t4_dn3 = assign50210_e83510_d_n3;
        locals.var_t4_dn4 = assign50210_e83510_d_n4;
        locals.var_t4_dn5 = assign50210_e83510_d_n5;
        locals.var_t4_dn6 = assign50210_e83510_d_n6;
        locals.var_t4_dn7 = assign50210_e83510_d_n7;
        locals.var_t4_dn8 = assign50210_e83510_d_n8;
        locals.var_t4_dn9 = assign50210_e83510_d_n9;
        locals.var_t4_dn10 = assign50210_e83510_d_n10;
        locals.var_t4_dn11 = assign50210_e83510_d_n11;

        let (assign50220_e83544, assign50220_e83544_d_n3, assign50220_e83544_d_n4, assign50220_e83544_d_n5, assign50220_e83544_d_n6, assign50220_e83544_d_n7, assign50220_e83544_d_n8, assign50220_e83544_d_n9, assign50220_e83544_d_n10, assign50220_e83544_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard759 == 0.0)) && (locals.var_guard764 == 0.0)) && (locals.var_guard765 != 0.0)) {
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_t4;
        let assign50220_e83524: f64 = (locals.var_kgidl_i * __rspice_inv_cse_0);
        let assign50220_e83527: f64 = (locals.var_kgidl_i * __rspice_inv_cse_0);
        let assign50220_e83530: f64 = (locals.var_kgidl_i * __rspice_inv_cse_0);
        let assign50220_e83531: f64 = (assign50220_e83527 * assign50220_e83530);
        let assign50220_e83534: f64 = (4.0 * 1e-6);
        let assign50220_e83536: f64 = (assign50220_e83534 * 1e-6);
        let assign50220_e83537: f64 = (assign50220_e83531 + assign50220_e83536);
        let assign50220_e83538: f64 = (assign50220_e83537).sqrt();
        let assign50220_e83539: f64 = (assign50220_e83524 + assign50220_e83538);
        let assign50220_e83540: f64 = (0.5 * assign50220_e83539);
        let assign50220_e83542: f64 = (assign50220_e83540 - 1e-6);
        (assign50220_e83542, (0.5 * ((-((locals.var_kgidl_i * locals.var_t4_dn3) / (locals.var_t4 * locals.var_t4))) + ((((-((locals.var_kgidl_i * locals.var_t4_dn3) / (locals.var_t4 * locals.var_t4))) * assign50220_e83530) + (assign50220_e83527 * (-((locals.var_kgidl_i * locals.var_t4_dn3) / (locals.var_t4 * locals.var_t4))))) / (2.0 * assign50220_e83538)))), (0.5 * ((-((locals.var_kgidl_i * locals.var_t4_dn4) / (locals.var_t4 * locals.var_t4))) + ((((-((locals.var_kgidl_i * locals.var_t4_dn4) / (locals.var_t4 * locals.var_t4))) * assign50220_e83530) + (assign50220_e83527 * (-((locals.var_kgidl_i * locals.var_t4_dn4) / (locals.var_t4 * locals.var_t4))))) / (2.0 * assign50220_e83538)))), (0.5 * ((-((locals.var_kgidl_i * locals.var_t4_dn5) / (locals.var_t4 * locals.var_t4))) + ((((-((locals.var_kgidl_i * locals.var_t4_dn5) / (locals.var_t4 * locals.var_t4))) * assign50220_e83530) + (assign50220_e83527 * (-((locals.var_kgidl_i * locals.var_t4_dn5) / (locals.var_t4 * locals.var_t4))))) / (2.0 * assign50220_e83538)))), (0.5 * ((-((locals.var_kgidl_i * locals.var_t4_dn6) / (locals.var_t4 * locals.var_t4))) + ((((-((locals.var_kgidl_i * locals.var_t4_dn6) / (locals.var_t4 * locals.var_t4))) * assign50220_e83530) + (assign50220_e83527 * (-((locals.var_kgidl_i * locals.var_t4_dn6) / (locals.var_t4 * locals.var_t4))))) / (2.0 * assign50220_e83538)))), (0.5 * ((-((locals.var_kgidl_i * locals.var_t4_dn7) / (locals.var_t4 * locals.var_t4))) + ((((-((locals.var_kgidl_i * locals.var_t4_dn7) / (locals.var_t4 * locals.var_t4))) * assign50220_e83530) + (assign50220_e83527 * (-((locals.var_kgidl_i * locals.var_t4_dn7) / (locals.var_t4 * locals.var_t4))))) / (2.0 * assign50220_e83538)))), (0.5 * ((-((locals.var_kgidl_i * locals.var_t4_dn8) / (locals.var_t4 * locals.var_t4))) + ((((-((locals.var_kgidl_i * locals.var_t4_dn8) / (locals.var_t4 * locals.var_t4))) * assign50220_e83530) + (assign50220_e83527 * (-((locals.var_kgidl_i * locals.var_t4_dn8) / (locals.var_t4 * locals.var_t4))))) / (2.0 * assign50220_e83538)))), (0.5 * ((-((locals.var_kgidl_i * locals.var_t4_dn9) / (locals.var_t4 * locals.var_t4))) + ((((-((locals.var_kgidl_i * locals.var_t4_dn9) / (locals.var_t4 * locals.var_t4))) * assign50220_e83530) + (assign50220_e83527 * (-((locals.var_kgidl_i * locals.var_t4_dn9) / (locals.var_t4 * locals.var_t4))))) / (2.0 * assign50220_e83538)))), (0.5 * ((-((locals.var_kgidl_i * locals.var_t4_dn10) / (locals.var_t4 * locals.var_t4))) + ((((-((locals.var_kgidl_i * locals.var_t4_dn10) / (locals.var_t4 * locals.var_t4))) * assign50220_e83530) + (assign50220_e83527 * (-((locals.var_kgidl_i * locals.var_t4_dn10) / (locals.var_t4 * locals.var_t4))))) / (2.0 * assign50220_e83538)))), (0.5 * ((-((locals.var_kgidl_i * locals.var_t4_dn11) / (locals.var_t4 * locals.var_t4))) + ((((-((locals.var_kgidl_i * locals.var_t4_dn11) / (locals.var_t4 * locals.var_t4))) * assign50220_e83530) + (assign50220_e83527 * (-((locals.var_kgidl_i * locals.var_t4_dn11) / (locals.var_t4 * locals.var_t4))))) / (2.0 * assign50220_e83538)))),)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign50220_e83544;
        locals.var_t5_dn3 = assign50220_e83544_d_n3;
        locals.var_t5_dn4 = assign50220_e83544_d_n4;
        locals.var_t5_dn5 = assign50220_e83544_d_n5;
        locals.var_t5_dn6 = assign50220_e83544_d_n6;
        locals.var_t5_dn7 = assign50220_e83544_d_n7;
        locals.var_t5_dn8 = assign50220_e83544_d_n8;
        locals.var_t5_dn9 = assign50220_e83544_d_n9;
        locals.var_t5_dn10 = assign50220_e83544_d_n10;
        locals.var_t5_dn11 = assign50220_e83544_d_n11;

        let (assign50230_e83558, assign50230_e83558_d_n3, assign50230_e83558_d_n4, assign50230_e83558_d_n5, assign50230_e83558_d_n6, assign50230_e83558_d_n7, assign50230_e83558_d_n8, assign50230_e83558_d_n9, assign50230_e83558_d_n10, assign50230_e83558_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard759 == 0.0)) && (locals.var_guard764 == 0.0)) && (locals.var_guard765 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign50230_e83558;
        locals.var_t5_dn3 = assign50230_e83558_d_n3;
        locals.var_t5_dn4 = assign50230_e83558_d_n4;
        locals.var_t5_dn5 = assign50230_e83558_d_n5;
        locals.var_t5_dn6 = assign50230_e83558_d_n6;
        locals.var_t5_dn7 = assign50230_e83558_d_n7;
        locals.var_t5_dn8 = assign50230_e83558_d_n8;
        locals.var_t5_dn9 = assign50230_e83558_d_n9;
        locals.var_t5_dn10 = assign50230_e83558_d_n10;
        locals.var_t5_dn11 = assign50230_e83558_d_n11;

        let (assign50240_e83580, assign50240_e83580_d_n3, assign50240_e83580_d_n4, assign50240_e83580_d_n5, assign50240_e83580_d_n6, assign50240_e83580_d_n7, assign50240_e83580_d_n8, assign50240_e83580_d_n9, assign50240_e83580_d_n10, assign50240_e83580_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard759 == 0.0)) && (locals.var_guard764 == 0.0)) {
        let assign50240_e83569: f64 = (locals.var_agidl_i * locals.var_wdiod);
        let assign50240_e83571: f64 = (assign50240_e83569 * locals.var_t1);
        let assign50240_e83573: f64 = (-locals.var_t2);
        let assign50240_e83574: f64 = { let limited_exp_arg = assign50240_e83573; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign50240_e83575: f64 = (assign50240_e83571 * assign50240_e83574);
        let assign50240_e83577: f64 = { let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign50240_e83578: f64 = (assign50240_e83575 * assign50240_e83577);
        (assign50240_e83578, (((((assign50240_e83569 * locals.var_t1_dn3) * assign50240_e83574) + (assign50240_e83571 * ({ let limited_exp_arg = assign50240_e83573; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn3)))) * assign50240_e83577) + (assign50240_e83575 * ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn3))), (((((assign50240_e83569 * locals.var_t1_dn4) * assign50240_e83574) + (assign50240_e83571 * ({ let limited_exp_arg = assign50240_e83573; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn4)))) * assign50240_e83577) + (assign50240_e83575 * ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn4))), (((((assign50240_e83569 * locals.var_t1_dn5) * assign50240_e83574) + (assign50240_e83571 * ({ let limited_exp_arg = assign50240_e83573; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn5)))) * assign50240_e83577) + (assign50240_e83575 * ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn5))), (((((assign50240_e83569 * locals.var_t1_dn6) * assign50240_e83574) + (assign50240_e83571 * ({ let limited_exp_arg = assign50240_e83573; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn6)))) * assign50240_e83577) + (assign50240_e83575 * ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn6))), (((((assign50240_e83569 * locals.var_t1_dn7) * assign50240_e83574) + (assign50240_e83571 * ({ let limited_exp_arg = assign50240_e83573; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn7)))) * assign50240_e83577) + (assign50240_e83575 * ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn7))), (((((assign50240_e83569 * locals.var_t1_dn8) * assign50240_e83574) + (assign50240_e83571 * ({ let limited_exp_arg = assign50240_e83573; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn8)))) * assign50240_e83577) + (assign50240_e83575 * ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn8))), (((((assign50240_e83569 * locals.var_t1_dn9) * assign50240_e83574) + (assign50240_e83571 * ({ let limited_exp_arg = assign50240_e83573; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn9)))) * assign50240_e83577) + (assign50240_e83575 * ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn9))), (((((assign50240_e83569 * locals.var_t1_dn10) * assign50240_e83574) + (assign50240_e83571 * ({ let limited_exp_arg = assign50240_e83573; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn10)))) * assign50240_e83577) + (assign50240_e83575 * ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn10))), (((((assign50240_e83569 * locals.var_t1_dn11) * assign50240_e83574) + (assign50240_e83571 * ({ let limited_exp_arg = assign50240_e83573; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn11)))) * assign50240_e83577) + (assign50240_e83575 * ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn11))),)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign50240_e83580;
        locals.var_t6_dn3 = assign50240_e83580_d_n3;
        locals.var_t6_dn4 = assign50240_e83580_d_n4;
        locals.var_t6_dn5 = assign50240_e83580_d_n5;
        locals.var_t6_dn6 = assign50240_e83580_d_n6;
        locals.var_t6_dn7 = assign50240_e83580_d_n7;
        locals.var_t6_dn8 = assign50240_e83580_d_n8;
        locals.var_t6_dn9 = assign50240_e83580_d_n9;
        locals.var_t6_dn10 = assign50240_e83580_d_n10;
        locals.var_t6_dn11 = assign50240_e83580_d_n11;

        let (assign50250_e83588, assign50250_e83588_d_n3, assign50250_e83588_d_n4, assign50250_e83588_d_n5, assign50250_e83588_d_n6, assign50250_e83588_d_n7, assign50250_e83588_d_n8, assign50250_e83588_d_n9, assign50250_e83588_d_n10, assign50250_e83588_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard759 == 0.0)) {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    } else {
        (locals.var_igidl, locals.var_igidl_dn3, locals.var_igidl_dn4, locals.var_igidl_dn5, locals.var_igidl_dn6, locals.var_igidl_dn7, locals.var_igidl_dn8, locals.var_igidl_dn9, locals.var_igidl_dn10, locals.var_igidl_dn11,)
    }
};
        locals.var_igidl = assign50250_e83588;
        locals.var_igidl_dn3 = assign50250_e83588_d_n3;
        locals.var_igidl_dn4 = assign50250_e83588_d_n4;
        locals.var_igidl_dn5 = assign50250_e83588_d_n5;
        locals.var_igidl_dn6 = assign50250_e83588_d_n6;
        locals.var_igidl_dn7 = assign50250_e83588_d_n7;
        locals.var_igidl_dn8 = assign50250_e83588_d_n8;
        locals.var_igidl_dn9 = assign50250_e83588_d_n9;
        locals.var_igidl_dn10 = assign50250_e83588_d_n10;
        locals.var_igidl_dn11 = assign50250_e83588_d_n11;

        let assign50260_e83595: f64 = if ((locals.var_agisl_i <= 0.0) || (locals.var_bgisl_t <= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard766 = assign50260_e83595;

        let (assign50270_e83605, assign50270_e83605_d_n3, assign50270_e83605_d_n4, assign50270_e83605_d_n5, assign50270_e83605_d_n6, assign50270_e83605_d_n7, assign50270_e83605_d_n8, assign50270_e83605_d_n9, assign50270_e83605_d_n10, assign50270_e83605_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard759 == 0.0)) && (locals.var_guard766 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign50270_e83605;
        locals.var_t6_dn3 = assign50270_e83605_d_n3;
        locals.var_t6_dn4 = assign50270_e83605_d_n4;
        locals.var_t6_dn5 = assign50270_e83605_d_n5;
        locals.var_t6_dn6 = assign50270_e83605_d_n6;
        locals.var_t6_dn7 = assign50270_e83605_d_n7;
        locals.var_t6_dn8 = assign50270_e83605_d_n8;
        locals.var_t6_dn9 = assign50270_e83605_d_n9;
        locals.var_t6_dn10 = assign50270_e83605_d_n10;
        locals.var_t6_dn11 = assign50270_e83605_d_n11;

        let (assign50280_e83623, assign50280_e83623_d_n3, assign50280_e83623_d_n4, assign50280_e83623_d_n5, assign50280_e83623_d_n6, assign50280_e83623_d_n7, assign50280_e83623_d_n8, assign50280_e83623_d_n9, assign50280_e83623_d_n10, assign50280_e83623_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard759 == 0.0)) && (locals.var_guard766 == 0.0)) {
        let assign50280_e83615: f64 = (-locals.var_vgs_noswap_1);
        let assign50280_e83617: f64 = (assign50280_e83615 - locals.var_egisl_i);
        let assign50280_e83619: f64 = (assign50280_e83617 + locals.var_vfbsdr);
        let assign50280_e83621: f64 = (assign50280_e83619 / locals.var_t0);
        (assign50280_e83621, (-((assign50280_e83619 * locals.var_t0_dn3) / (locals.var_t0 * locals.var_t0))), (((locals.var_vfbsdr_dn4 * locals.var_t0) - (assign50280_e83619 * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)), (((locals.var_vfbsdr_dn5 * locals.var_t0) - (assign50280_e83619 * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)), ((((-locals.var_vgs_noswap_1_dn6) * locals.var_t0) - (assign50280_e83619 * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)), ((((-locals.var_vgs_noswap_1_dn7) * locals.var_t0) - (assign50280_e83619 * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)), ((((-locals.var_vgs_noswap_1_dn8) * locals.var_t0) - (assign50280_e83619 * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)), (-((assign50280_e83619 * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0))), ((((-locals.var_vgs_noswap_1_dn10) * locals.var_t0) - (assign50280_e83619 * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)), (-((assign50280_e83619 * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0))),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign50280_e83623;
        locals.var_t1_dn3 = assign50280_e83623_d_n3;
        locals.var_t1_dn4 = assign50280_e83623_d_n4;
        locals.var_t1_dn5 = assign50280_e83623_d_n5;
        locals.var_t1_dn6 = assign50280_e83623_d_n6;
        locals.var_t1_dn7 = assign50280_e83623_d_n7;
        locals.var_t1_dn8 = assign50280_e83623_d_n8;
        locals.var_t1_dn9 = assign50280_e83623_d_n9;
        locals.var_t1_dn10 = assign50280_e83623_d_n10;
        locals.var_t1_dn11 = assign50280_e83623_d_n11;

        let (assign50290_e83647, assign50290_e83647_d_n3, assign50290_e83647_d_n4, assign50290_e83647_d_n5, assign50290_e83647_d_n6, assign50290_e83647_d_n7, assign50290_e83647_d_n8, assign50290_e83647_d_n9, assign50290_e83647_d_n10, assign50290_e83647_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard759 == 0.0)) && (locals.var_guard766 == 0.0)) {
        let assign50290_e83636: f64 = (locals.var_t1 * locals.var_t1);
        let assign50290_e83639: f64 = (4.0 * 0.01);
        let assign50290_e83641: f64 = (assign50290_e83639 * 0.01);
        let assign50290_e83642: f64 = (assign50290_e83636 + assign50290_e83641);
        let assign50290_e83643: f64 = (assign50290_e83642).sqrt();
        let assign50290_e83644: f64 = (locals.var_t1 + assign50290_e83643);
        let assign50290_e83645: f64 = (0.5 * assign50290_e83644);
        (assign50290_e83645, (0.5 * (locals.var_t1_dn3 + (((locals.var_t1_dn3 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn3)) / (2.0 * assign50290_e83643)))), (0.5 * (locals.var_t1_dn4 + (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) / (2.0 * assign50290_e83643)))), (0.5 * (locals.var_t1_dn5 + (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) / (2.0 * assign50290_e83643)))), (0.5 * (locals.var_t1_dn6 + (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) / (2.0 * assign50290_e83643)))), (0.5 * (locals.var_t1_dn7 + (((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) / (2.0 * assign50290_e83643)))), (0.5 * (locals.var_t1_dn8 + (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) / (2.0 * assign50290_e83643)))), (0.5 * (locals.var_t1_dn9 + (((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) / (2.0 * assign50290_e83643)))), (0.5 * (locals.var_t1_dn10 + (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) / (2.0 * assign50290_e83643)))), (0.5 * (locals.var_t1_dn11 + (((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) / (2.0 * assign50290_e83643)))),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign50290_e83647;
        locals.var_t1_dn3 = assign50290_e83647_d_n3;
        locals.var_t1_dn4 = assign50290_e83647_d_n4;
        locals.var_t1_dn5 = assign50290_e83647_d_n5;
        locals.var_t1_dn6 = assign50290_e83647_d_n6;
        locals.var_t1_dn7 = assign50290_e83647_d_n7;
        locals.var_t1_dn8 = assign50290_e83647_d_n8;
        locals.var_t1_dn9 = assign50290_e83647_d_n9;
        locals.var_t1_dn10 = assign50290_e83647_d_n10;
        locals.var_t1_dn11 = assign50290_e83647_d_n11;

        let (assign50300_e83662, assign50300_e83662_d_n3, assign50300_e83662_d_n4, assign50300_e83662_d_n5, assign50300_e83662_d_n6, assign50300_e83662_d_n7, assign50300_e83662_d_n8, assign50300_e83662_d_n9, assign50300_e83662_d_n10, assign50300_e83662_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard759 == 0.0)) && (locals.var_guard766 == 0.0)) {
        let assign50300_e83659: f64 = (locals.var_t1 + 0.001);
        let assign50300_e83660: f64 = (locals.var_bgisl_t / assign50300_e83659);
        (assign50300_e83660, (-((locals.var_bgisl_t * locals.var_t1_dn3) / (assign50300_e83659 * assign50300_e83659))), (((locals.var_bgisl_t_dn4 * assign50300_e83659) - (locals.var_bgisl_t * locals.var_t1_dn4)) / (assign50300_e83659 * assign50300_e83659)), (((locals.var_bgisl_t_dn5 * assign50300_e83659) - (locals.var_bgisl_t * locals.var_t1_dn5)) / (assign50300_e83659 * assign50300_e83659)), (-((locals.var_bgisl_t * locals.var_t1_dn6) / (assign50300_e83659 * assign50300_e83659))), (-((locals.var_bgisl_t * locals.var_t1_dn7) / (assign50300_e83659 * assign50300_e83659))), (-((locals.var_bgisl_t * locals.var_t1_dn8) / (assign50300_e83659 * assign50300_e83659))), (-((locals.var_bgisl_t * locals.var_t1_dn9) / (assign50300_e83659 * assign50300_e83659))), (-((locals.var_bgisl_t * locals.var_t1_dn10) / (assign50300_e83659 * assign50300_e83659))), (-((locals.var_bgisl_t * locals.var_t1_dn11) / (assign50300_e83659 * assign50300_e83659))),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign50300_e83662;
        locals.var_t2_dn3 = assign50300_e83662_d_n3;
        locals.var_t2_dn4 = assign50300_e83662_d_n4;
        locals.var_t2_dn5 = assign50300_e83662_d_n5;
        locals.var_t2_dn6 = assign50300_e83662_d_n6;
        locals.var_t2_dn7 = assign50300_e83662_d_n7;
        locals.var_t2_dn8 = assign50300_e83662_d_n8;
        locals.var_t2_dn9 = assign50300_e83662_d_n9;
        locals.var_t2_dn10 = assign50300_e83662_d_n10;
        locals.var_t2_dn11 = assign50300_e83662_d_n11;

        let assign50310_e83665: f64 = if locals.var_kgisl_i != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard767 = assign50310_e83665;

        let (assign50320_e83681, assign50320_e83681_d_n3, assign50320_e83681_d_n4, assign50320_e83681_d_n5, assign50320_e83681_d_n6, assign50320_e83681_d_n7, assign50320_e83681_d_n8, assign50320_e83681_d_n9, assign50320_e83681_d_n10, assign50320_e83681_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard759 == 0.0)) && (locals.var_guard766 == 0.0)) && (locals.var_guard767 != 0.0)) {
        let assign50320_e83677: f64 = (-locals.var_vsb_noswap);
        let assign50320_e83679: f64 = (assign50320_e83677 - locals.var_fgisl_i);
        (assign50320_e83679, 0.0, 0.0, 0.0, (-locals.var_vsb_noswap_dn6), (-locals.var_vsb_noswap_dn7), 0.0, 0.0, (-locals.var_vsb_noswap_dn10), 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign50320_e83681;
        locals.var_t3_dn3 = assign50320_e83681_d_n3;
        locals.var_t3_dn4 = assign50320_e83681_d_n4;
        locals.var_t3_dn5 = assign50320_e83681_d_n5;
        locals.var_t3_dn6 = assign50320_e83681_d_n6;
        locals.var_t3_dn7 = assign50320_e83681_d_n7;
        locals.var_t3_dn8 = assign50320_e83681_d_n8;
        locals.var_t3_dn9 = assign50320_e83681_d_n9;
        locals.var_t3_dn10 = assign50320_e83681_d_n10;
        locals.var_t3_dn11 = assign50320_e83681_d_n11;

        let (assign50330_e83696, assign50330_e83696_d_n3, assign50330_e83696_d_n4, assign50330_e83696_d_n5, assign50330_e83696_d_n6, assign50330_e83696_d_n7, assign50330_e83696_d_n8, assign50330_e83696_d_n9, assign50330_e83696_d_n10, assign50330_e83696_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard759 == 0.0)) && (locals.var_guard766 == 0.0)) && (locals.var_guard767 != 0.0)) {
        let assign50330_e83694: f64 = (locals.var_t3 + 0.0001);
        (assign50330_e83694, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign50330_e83696;
        locals.var_t4_dn3 = assign50330_e83696_d_n3;
        locals.var_t4_dn4 = assign50330_e83696_d_n4;
        locals.var_t4_dn5 = assign50330_e83696_d_n5;
        locals.var_t4_dn6 = assign50330_e83696_d_n6;
        locals.var_t4_dn7 = assign50330_e83696_d_n7;
        locals.var_t4_dn8 = assign50330_e83696_d_n8;
        locals.var_t4_dn9 = assign50330_e83696_d_n9;
        locals.var_t4_dn10 = assign50330_e83696_d_n10;
        locals.var_t4_dn11 = assign50330_e83696_d_n11;

        let (assign50340_e83730, assign50340_e83730_d_n3, assign50340_e83730_d_n4, assign50340_e83730_d_n5, assign50340_e83730_d_n6, assign50340_e83730_d_n7, assign50340_e83730_d_n8, assign50340_e83730_d_n9, assign50340_e83730_d_n10, assign50340_e83730_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard759 == 0.0)) && (locals.var_guard766 == 0.0)) && (locals.var_guard767 != 0.0)) {
        let __rspice_inv_cse_1: f64 = 1.0 / locals.var_t4;
        let assign50340_e83710: f64 = (locals.var_kgisl_i * __rspice_inv_cse_1);
        let assign50340_e83713: f64 = (locals.var_kgisl_i * __rspice_inv_cse_1);
        let assign50340_e83716: f64 = (locals.var_kgisl_i * __rspice_inv_cse_1);
        let assign50340_e83717: f64 = (assign50340_e83713 * assign50340_e83716);
        let assign50340_e83720: f64 = (4.0 * 1e-6);
        let assign50340_e83722: f64 = (assign50340_e83720 * 1e-6);
        let assign50340_e83723: f64 = (assign50340_e83717 + assign50340_e83722);
        let assign50340_e83724: f64 = (assign50340_e83723).sqrt();
        let assign50340_e83725: f64 = (assign50340_e83710 + assign50340_e83724);
        let assign50340_e83726: f64 = (0.5 * assign50340_e83725);
        let assign50340_e83728: f64 = (assign50340_e83726 - 1e-6);
        (assign50340_e83728, (0.5 * ((-((locals.var_kgisl_i * locals.var_t4_dn3) / (locals.var_t4 * locals.var_t4))) + ((((-((locals.var_kgisl_i * locals.var_t4_dn3) / (locals.var_t4 * locals.var_t4))) * assign50340_e83716) + (assign50340_e83713 * (-((locals.var_kgisl_i * locals.var_t4_dn3) / (locals.var_t4 * locals.var_t4))))) / (2.0 * assign50340_e83724)))), (0.5 * ((-((locals.var_kgisl_i * locals.var_t4_dn4) / (locals.var_t4 * locals.var_t4))) + ((((-((locals.var_kgisl_i * locals.var_t4_dn4) / (locals.var_t4 * locals.var_t4))) * assign50340_e83716) + (assign50340_e83713 * (-((locals.var_kgisl_i * locals.var_t4_dn4) / (locals.var_t4 * locals.var_t4))))) / (2.0 * assign50340_e83724)))), (0.5 * ((-((locals.var_kgisl_i * locals.var_t4_dn5) / (locals.var_t4 * locals.var_t4))) + ((((-((locals.var_kgisl_i * locals.var_t4_dn5) / (locals.var_t4 * locals.var_t4))) * assign50340_e83716) + (assign50340_e83713 * (-((locals.var_kgisl_i * locals.var_t4_dn5) / (locals.var_t4 * locals.var_t4))))) / (2.0 * assign50340_e83724)))), (0.5 * ((-((locals.var_kgisl_i * locals.var_t4_dn6) / (locals.var_t4 * locals.var_t4))) + ((((-((locals.var_kgisl_i * locals.var_t4_dn6) / (locals.var_t4 * locals.var_t4))) * assign50340_e83716) + (assign50340_e83713 * (-((locals.var_kgisl_i * locals.var_t4_dn6) / (locals.var_t4 * locals.var_t4))))) / (2.0 * assign50340_e83724)))), (0.5 * ((-((locals.var_kgisl_i * locals.var_t4_dn7) / (locals.var_t4 * locals.var_t4))) + ((((-((locals.var_kgisl_i * locals.var_t4_dn7) / (locals.var_t4 * locals.var_t4))) * assign50340_e83716) + (assign50340_e83713 * (-((locals.var_kgisl_i * locals.var_t4_dn7) / (locals.var_t4 * locals.var_t4))))) / (2.0 * assign50340_e83724)))), (0.5 * ((-((locals.var_kgisl_i * locals.var_t4_dn8) / (locals.var_t4 * locals.var_t4))) + ((((-((locals.var_kgisl_i * locals.var_t4_dn8) / (locals.var_t4 * locals.var_t4))) * assign50340_e83716) + (assign50340_e83713 * (-((locals.var_kgisl_i * locals.var_t4_dn8) / (locals.var_t4 * locals.var_t4))))) / (2.0 * assign50340_e83724)))), (0.5 * ((-((locals.var_kgisl_i * locals.var_t4_dn9) / (locals.var_t4 * locals.var_t4))) + ((((-((locals.var_kgisl_i * locals.var_t4_dn9) / (locals.var_t4 * locals.var_t4))) * assign50340_e83716) + (assign50340_e83713 * (-((locals.var_kgisl_i * locals.var_t4_dn9) / (locals.var_t4 * locals.var_t4))))) / (2.0 * assign50340_e83724)))), (0.5 * ((-((locals.var_kgisl_i * locals.var_t4_dn10) / (locals.var_t4 * locals.var_t4))) + ((((-((locals.var_kgisl_i * locals.var_t4_dn10) / (locals.var_t4 * locals.var_t4))) * assign50340_e83716) + (assign50340_e83713 * (-((locals.var_kgisl_i * locals.var_t4_dn10) / (locals.var_t4 * locals.var_t4))))) / (2.0 * assign50340_e83724)))), (0.5 * ((-((locals.var_kgisl_i * locals.var_t4_dn11) / (locals.var_t4 * locals.var_t4))) + ((((-((locals.var_kgisl_i * locals.var_t4_dn11) / (locals.var_t4 * locals.var_t4))) * assign50340_e83716) + (assign50340_e83713 * (-((locals.var_kgisl_i * locals.var_t4_dn11) / (locals.var_t4 * locals.var_t4))))) / (2.0 * assign50340_e83724)))),)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign50340_e83730;
        locals.var_t5_dn3 = assign50340_e83730_d_n3;
        locals.var_t5_dn4 = assign50340_e83730_d_n4;
        locals.var_t5_dn5 = assign50340_e83730_d_n5;
        locals.var_t5_dn6 = assign50340_e83730_d_n6;
        locals.var_t5_dn7 = assign50340_e83730_d_n7;
        locals.var_t5_dn8 = assign50340_e83730_d_n8;
        locals.var_t5_dn9 = assign50340_e83730_d_n9;
        locals.var_t5_dn10 = assign50340_e83730_d_n10;
        locals.var_t5_dn11 = assign50340_e83730_d_n11;

        let (assign50350_e83744, assign50350_e83744_d_n3, assign50350_e83744_d_n4, assign50350_e83744_d_n5, assign50350_e83744_d_n6, assign50350_e83744_d_n7, assign50350_e83744_d_n8, assign50350_e83744_d_n9, assign50350_e83744_d_n10, assign50350_e83744_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard759 == 0.0)) && (locals.var_guard766 == 0.0)) && (locals.var_guard767 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign50350_e83744;
        locals.var_t5_dn3 = assign50350_e83744_d_n3;
        locals.var_t5_dn4 = assign50350_e83744_d_n4;
        locals.var_t5_dn5 = assign50350_e83744_d_n5;
        locals.var_t5_dn6 = assign50350_e83744_d_n6;
        locals.var_t5_dn7 = assign50350_e83744_d_n7;
        locals.var_t5_dn8 = assign50350_e83744_d_n8;
        locals.var_t5_dn9 = assign50350_e83744_d_n9;
        locals.var_t5_dn10 = assign50350_e83744_d_n10;
        locals.var_t5_dn11 = assign50350_e83744_d_n11;

        let (assign50360_e83766, assign50360_e83766_d_n3, assign50360_e83766_d_n4, assign50360_e83766_d_n5, assign50360_e83766_d_n6, assign50360_e83766_d_n7, assign50360_e83766_d_n8, assign50360_e83766_d_n9, assign50360_e83766_d_n10, assign50360_e83766_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard759 == 0.0)) && (locals.var_guard766 == 0.0)) {
        let assign50360_e83755: f64 = (locals.var_agisl_i * locals.var_wdios);
        let assign50360_e83757: f64 = (assign50360_e83755 * locals.var_t1);
        let assign50360_e83759: f64 = (-locals.var_t2);
        let assign50360_e83760: f64 = { let limited_exp_arg = assign50360_e83759; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign50360_e83761: f64 = (assign50360_e83757 * assign50360_e83760);
        let assign50360_e83763: f64 = { let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign50360_e83764: f64 = (assign50360_e83761 * assign50360_e83763);
        (assign50360_e83764, (((((assign50360_e83755 * locals.var_t1_dn3) * assign50360_e83760) + (assign50360_e83757 * ({ let limited_exp_arg = assign50360_e83759; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn3)))) * assign50360_e83763) + (assign50360_e83761 * ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn3))), (((((assign50360_e83755 * locals.var_t1_dn4) * assign50360_e83760) + (assign50360_e83757 * ({ let limited_exp_arg = assign50360_e83759; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn4)))) * assign50360_e83763) + (assign50360_e83761 * ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn4))), (((((assign50360_e83755 * locals.var_t1_dn5) * assign50360_e83760) + (assign50360_e83757 * ({ let limited_exp_arg = assign50360_e83759; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn5)))) * assign50360_e83763) + (assign50360_e83761 * ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn5))), (((((assign50360_e83755 * locals.var_t1_dn6) * assign50360_e83760) + (assign50360_e83757 * ({ let limited_exp_arg = assign50360_e83759; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn6)))) * assign50360_e83763) + (assign50360_e83761 * ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn6))), (((((assign50360_e83755 * locals.var_t1_dn7) * assign50360_e83760) + (assign50360_e83757 * ({ let limited_exp_arg = assign50360_e83759; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn7)))) * assign50360_e83763) + (assign50360_e83761 * ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn7))), (((((assign50360_e83755 * locals.var_t1_dn8) * assign50360_e83760) + (assign50360_e83757 * ({ let limited_exp_arg = assign50360_e83759; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn8)))) * assign50360_e83763) + (assign50360_e83761 * ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn8))), (((((assign50360_e83755 * locals.var_t1_dn9) * assign50360_e83760) + (assign50360_e83757 * ({ let limited_exp_arg = assign50360_e83759; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn9)))) * assign50360_e83763) + (assign50360_e83761 * ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn9))), (((((assign50360_e83755 * locals.var_t1_dn10) * assign50360_e83760) + (assign50360_e83757 * ({ let limited_exp_arg = assign50360_e83759; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn10)))) * assign50360_e83763) + (assign50360_e83761 * ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn10))), (((((assign50360_e83755 * locals.var_t1_dn11) * assign50360_e83760) + (assign50360_e83757 * ({ let limited_exp_arg = assign50360_e83759; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn11)))) * assign50360_e83763) + (assign50360_e83761 * ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn11))),)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign50360_e83766;
        locals.var_t6_dn3 = assign50360_e83766_d_n3;
        locals.var_t6_dn4 = assign50360_e83766_d_n4;
        locals.var_t6_dn5 = assign50360_e83766_d_n5;
        locals.var_t6_dn6 = assign50360_e83766_d_n6;
        locals.var_t6_dn7 = assign50360_e83766_d_n7;
        locals.var_t6_dn8 = assign50360_e83766_d_n8;
        locals.var_t6_dn9 = assign50360_e83766_d_n9;
        locals.var_t6_dn10 = assign50360_e83766_d_n10;
        locals.var_t6_dn11 = assign50360_e83766_d_n11;

        let (assign50370_e83774, assign50370_e83774_d_n3, assign50370_e83774_d_n4, assign50370_e83774_d_n5, assign50370_e83774_d_n6, assign50370_e83774_d_n7, assign50370_e83774_d_n8, assign50370_e83774_d_n9, assign50370_e83774_d_n10, assign50370_e83774_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard759 == 0.0)) {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    } else {
        (locals.var_igisl, locals.var_igisl_dn3, locals.var_igisl_dn4, locals.var_igisl_dn5, locals.var_igisl_dn6, locals.var_igisl_dn7, locals.var_igisl_dn8, locals.var_igisl_dn9, locals.var_igisl_dn10, locals.var_igisl_dn11,)
    }
};
        locals.var_igisl = assign50370_e83774;
        locals.var_igisl_dn3 = assign50370_e83774_d_n3;
        locals.var_igisl_dn4 = assign50370_e83774_d_n4;
        locals.var_igisl_dn5 = assign50370_e83774_d_n5;
        locals.var_igisl_dn6 = assign50370_e83774_d_n6;
        locals.var_igisl_dn7 = assign50370_e83774_d_n7;
        locals.var_igisl_dn8 = assign50370_e83774_d_n8;
        locals.var_igisl_dn9 = assign50370_e83774_d_n9;
        locals.var_igisl_dn10 = assign50370_e83774_d_n10;
        locals.var_igisl_dn11 = assign50370_e83774_d_n11;

        let (assign50380_e83783, assign50380_e83783_d_n3, assign50380_e83783_d_n4, assign50380_e83783_d_n5, assign50380_e83783_d_n6, assign50380_e83783_d_n7, assign50380_e83783_d_n8, assign50380_e83783_d_n9, assign50380_e83783_d_n10, assign50380_e83783_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign50380_e83779: f64 = (locals.var_devsign * p.p2);
        let assign50380_e83781: f64 = (assign50380_e83779 * locals.var_igidl);
        (assign50380_e83781, (assign50380_e83779 * locals.var_igidl_dn3), (assign50380_e83779 * locals.var_igidl_dn4), (assign50380_e83779 * locals.var_igidl_dn5), (assign50380_e83779 * locals.var_igidl_dn6), (assign50380_e83779 * locals.var_igidl_dn7), (assign50380_e83779 * locals.var_igidl_dn8), (assign50380_e83779 * locals.var_igidl_dn9), (assign50380_e83779 * locals.var_igidl_dn10), (assign50380_e83779 * locals.var_igidl_dn11),)
    } else {
        (locals.var_igidl_1, locals.var_igidl_1_dn3, locals.var_igidl_1_dn4, locals.var_igidl_1_dn5, locals.var_igidl_1_dn6, locals.var_igidl_1_dn7, locals.var_igidl_1_dn8, locals.var_igidl_1_dn9, locals.var_igidl_1_dn10, locals.var_igidl_1_dn11,)
    }
};
        locals.var_igidl_1 = assign50380_e83783;
        locals.var_igidl_1_dn3 = assign50380_e83783_d_n3;
        locals.var_igidl_1_dn4 = assign50380_e83783_d_n4;
        locals.var_igidl_1_dn5 = assign50380_e83783_d_n5;
        locals.var_igidl_1_dn6 = assign50380_e83783_d_n6;
        locals.var_igidl_1_dn7 = assign50380_e83783_d_n7;
        locals.var_igidl_1_dn8 = assign50380_e83783_d_n8;
        locals.var_igidl_1_dn9 = assign50380_e83783_d_n9;
        locals.var_igidl_1_dn10 = assign50380_e83783_d_n10;
        locals.var_igidl_1_dn11 = assign50380_e83783_d_n11;

        let (assign50390_e83792, assign50390_e83792_d_n3, assign50390_e83792_d_n4, assign50390_e83792_d_n5, assign50390_e83792_d_n6, assign50390_e83792_d_n7, assign50390_e83792_d_n8, assign50390_e83792_d_n9, assign50390_e83792_d_n10, assign50390_e83792_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign50390_e83788: f64 = (locals.var_devsign * p.p2);
        let assign50390_e83790: f64 = (assign50390_e83788 * locals.var_igisl);
        (assign50390_e83790, (assign50390_e83788 * locals.var_igisl_dn3), (assign50390_e83788 * locals.var_igisl_dn4), (assign50390_e83788 * locals.var_igisl_dn5), (assign50390_e83788 * locals.var_igisl_dn6), (assign50390_e83788 * locals.var_igisl_dn7), (assign50390_e83788 * locals.var_igisl_dn8), (assign50390_e83788 * locals.var_igisl_dn9), (assign50390_e83788 * locals.var_igisl_dn10), (assign50390_e83788 * locals.var_igisl_dn11),)
    } else {
        (locals.var_igisl_1, locals.var_igisl_1_dn3, locals.var_igisl_1_dn4, locals.var_igisl_1_dn5, locals.var_igisl_1_dn6, locals.var_igisl_1_dn7, locals.var_igisl_1_dn8, locals.var_igisl_1_dn9, locals.var_igisl_1_dn10, locals.var_igisl_1_dn11,)
    }
};
        locals.var_igisl_1 = assign50390_e83792;
        locals.var_igisl_1_dn3 = assign50390_e83792_d_n3;
        locals.var_igisl_1_dn4 = assign50390_e83792_d_n4;
        locals.var_igisl_1_dn5 = assign50390_e83792_d_n5;
        locals.var_igisl_1_dn6 = assign50390_e83792_d_n6;
        locals.var_igisl_1_dn7 = assign50390_e83792_d_n7;
        locals.var_igisl_1_dn8 = assign50390_e83792_d_n8;
        locals.var_igisl_1_dn9 = assign50390_e83792_d_n9;
        locals.var_igisl_1_dn10 = assign50390_e83792_d_n10;
        locals.var_igisl_1_dn11 = assign50390_e83792_d_n11;

        let assign50400_e83795: f64 = if p.p44 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard768 = assign50400_e83795;

        let assign50410_e83802: f64 = if ((locals.var_alpha0_i <= 0.0) || (locals.var_beta0_t <= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard769 = assign50410_e83802;

        let (assign50420_e83811, assign50420_e83811_d_n3, assign50420_e83811_d_n4, assign50420_e83811_d_n5, assign50420_e83811_d_n6, assign50420_e83811_d_n7, assign50420_e83811_d_n8, assign50420_e83811_d_n9, assign50420_e83811_d_n10, assign50420_e83811_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard768 != 0.0)) && (locals.var_guard769 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_iii, locals.var_iii_dn3, locals.var_iii_dn4, locals.var_iii_dn5, locals.var_iii_dn6, locals.var_iii_dn7, locals.var_iii_dn8, locals.var_iii_dn9, locals.var_iii_dn10, locals.var_iii_dn11,)
    }
};
        locals.var_iii = assign50420_e83811;
        locals.var_iii_dn3 = assign50420_e83811_d_n3;
        locals.var_iii_dn4 = assign50420_e83811_d_n4;
        locals.var_iii_dn5 = assign50420_e83811_d_n5;
        locals.var_iii_dn6 = assign50420_e83811_d_n6;
        locals.var_iii_dn7 = assign50420_e83811_d_n7;
        locals.var_iii_dn8 = assign50420_e83811_d_n8;
        locals.var_iii_dn9 = assign50420_e83811_d_n9;
        locals.var_iii_dn10 = assign50420_e83811_d_n10;
        locals.var_iii_dn11 = assign50420_e83811_d_n11;

        let assign50430_e83815: f64 = (locals.var_beta0_t / 80.0);
        let assign50430_e83816: f64 = if locals.var_diffvds > assign50430_e83815 { 1.0 } else { 0.0 };
        locals.var_guard770 = assign50430_e83816;

        let (assign50440_e83831, assign50440_e83831_d_n3, assign50440_e83831_d_n4, assign50440_e83831_d_n5, assign50440_e83831_d_n6, assign50440_e83831_d_n7, assign50440_e83831_d_n8, assign50440_e83831_d_n9, assign50440_e83831_d_n10, assign50440_e83831_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 != 0.0)) && (locals.var_guard769 == 0.0)) && (locals.var_guard770 != 0.0)) {
        let assign50440_e83827: f64 = (-locals.var_beta0_t);
        let assign50440_e83829: f64 = (assign50440_e83827 / locals.var_diffvds);
        (assign50440_e83829, (-((assign50440_e83827 * locals.var_diffvds_dn3) / (locals.var_diffvds * locals.var_diffvds))), ((((-locals.var_beta0_t_dn4) * locals.var_diffvds) - (assign50440_e83827 * locals.var_diffvds_dn4)) / (locals.var_diffvds * locals.var_diffvds)), ((((-locals.var_beta0_t_dn5) * locals.var_diffvds) - (assign50440_e83827 * locals.var_diffvds_dn5)) / (locals.var_diffvds * locals.var_diffvds)), (-((assign50440_e83827 * locals.var_diffvds_dn6) / (locals.var_diffvds * locals.var_diffvds))), (-((assign50440_e83827 * locals.var_diffvds_dn7) / (locals.var_diffvds * locals.var_diffvds))), (-((assign50440_e83827 * locals.var_diffvds_dn8) / (locals.var_diffvds * locals.var_diffvds))), (-((assign50440_e83827 * locals.var_diffvds_dn9) / (locals.var_diffvds * locals.var_diffvds))), (-((assign50440_e83827 * locals.var_diffvds_dn10) / (locals.var_diffvds * locals.var_diffvds))), (-((assign50440_e83827 * locals.var_diffvds_dn11) / (locals.var_diffvds * locals.var_diffvds))),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign50440_e83831;
        locals.var_t1_dn3 = assign50440_e83831_d_n3;
        locals.var_t1_dn4 = assign50440_e83831_d_n4;
        locals.var_t1_dn5 = assign50440_e83831_d_n5;
        locals.var_t1_dn6 = assign50440_e83831_d_n6;
        locals.var_t1_dn7 = assign50440_e83831_d_n7;
        locals.var_t1_dn8 = assign50440_e83831_d_n8;
        locals.var_t1_dn9 = assign50440_e83831_d_n9;
        locals.var_t1_dn10 = assign50440_e83831_d_n10;
        locals.var_t1_dn11 = assign50440_e83831_d_n11;

        let (assign50450_e83852, assign50450_e83852_d_n3, assign50450_e83852_d_n4, assign50450_e83852_d_n5, assign50450_e83852_d_n6, assign50450_e83852_d_n7, assign50450_e83852_d_n8, assign50450_e83852_d_n9, assign50450_e83852_d_n10, assign50450_e83852_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 != 0.0)) && (locals.var_guard769 == 0.0)) && (locals.var_guard770 != 0.0)) {
        let assign50450_e83843: f64 = (locals.var_alpha0_i * locals.var_diffvds);
        let assign50450_e83845: f64 = (assign50450_e83843 * locals.var_ids);
        let assign50450_e83847: f64 = { let limited_exp_arg = locals.var_t1; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign50450_e83848: f64 = (assign50450_e83845 * assign50450_e83847);
        let assign50450_e83850: f64 = (assign50450_e83848 / locals.var_mscbe);
        (assign50450_e83850, ((((((((locals.var_alpha0_i * locals.var_diffvds_dn3) * locals.var_ids) + (assign50450_e83843 * locals.var_ids_dn3)) * assign50450_e83847) + (assign50450_e83845 * ({ let limited_exp_arg = locals.var_t1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t1_dn3))) * locals.var_mscbe) - (assign50450_e83848 * locals.var_mscbe_dn3)) / (locals.var_mscbe * locals.var_mscbe)), ((((((((locals.var_alpha0_i * locals.var_diffvds_dn4) * locals.var_ids) + (assign50450_e83843 * locals.var_ids_dn4)) * assign50450_e83847) + (assign50450_e83845 * ({ let limited_exp_arg = locals.var_t1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t1_dn4))) * locals.var_mscbe) - (assign50450_e83848 * locals.var_mscbe_dn4)) / (locals.var_mscbe * locals.var_mscbe)), ((((((((locals.var_alpha0_i * locals.var_diffvds_dn5) * locals.var_ids) + (assign50450_e83843 * locals.var_ids_dn5)) * assign50450_e83847) + (assign50450_e83845 * ({ let limited_exp_arg = locals.var_t1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t1_dn5))) * locals.var_mscbe) - (assign50450_e83848 * locals.var_mscbe_dn5)) / (locals.var_mscbe * locals.var_mscbe)), ((((((((locals.var_alpha0_i * locals.var_diffvds_dn6) * locals.var_ids) + (assign50450_e83843 * locals.var_ids_dn6)) * assign50450_e83847) + (assign50450_e83845 * ({ let limited_exp_arg = locals.var_t1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t1_dn6))) * locals.var_mscbe) - (assign50450_e83848 * locals.var_mscbe_dn6)) / (locals.var_mscbe * locals.var_mscbe)), ((((((((locals.var_alpha0_i * locals.var_diffvds_dn7) * locals.var_ids) + (assign50450_e83843 * locals.var_ids_dn7)) * assign50450_e83847) + (assign50450_e83845 * ({ let limited_exp_arg = locals.var_t1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t1_dn7))) * locals.var_mscbe) - (assign50450_e83848 * locals.var_mscbe_dn7)) / (locals.var_mscbe * locals.var_mscbe)), ((((((((locals.var_alpha0_i * locals.var_diffvds_dn8) * locals.var_ids) + (assign50450_e83843 * locals.var_ids_dn8)) * assign50450_e83847) + (assign50450_e83845 * ({ let limited_exp_arg = locals.var_t1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t1_dn8))) * locals.var_mscbe) - (assign50450_e83848 * locals.var_mscbe_dn8)) / (locals.var_mscbe * locals.var_mscbe)), ((((((((locals.var_alpha0_i * locals.var_diffvds_dn9) * locals.var_ids) + (assign50450_e83843 * locals.var_ids_dn9)) * assign50450_e83847) + (assign50450_e83845 * ({ let limited_exp_arg = locals.var_t1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t1_dn9))) * locals.var_mscbe) - (assign50450_e83848 * locals.var_mscbe_dn9)) / (locals.var_mscbe * locals.var_mscbe)), ((((((((locals.var_alpha0_i * locals.var_diffvds_dn10) * locals.var_ids) + (assign50450_e83843 * locals.var_ids_dn10)) * assign50450_e83847) + (assign50450_e83845 * ({ let limited_exp_arg = locals.var_t1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t1_dn10))) * locals.var_mscbe) - (assign50450_e83848 * locals.var_mscbe_dn10)) / (locals.var_mscbe * locals.var_mscbe)), ((((((((locals.var_alpha0_i * locals.var_diffvds_dn11) * locals.var_ids) + (assign50450_e83843 * locals.var_ids_dn11)) * assign50450_e83847) + (assign50450_e83845 * ({ let limited_exp_arg = locals.var_t1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t1_dn11))) * locals.var_mscbe) - (assign50450_e83848 * locals.var_mscbe_dn11)) / (locals.var_mscbe * locals.var_mscbe)),)
    } else {
        (locals.var_iii, locals.var_iii_dn3, locals.var_iii_dn4, locals.var_iii_dn5, locals.var_iii_dn6, locals.var_iii_dn7, locals.var_iii_dn8, locals.var_iii_dn9, locals.var_iii_dn10, locals.var_iii_dn11,)
    }
};
        locals.var_iii = assign50450_e83852;
        locals.var_iii_dn3 = assign50450_e83852_d_n3;
        locals.var_iii_dn4 = assign50450_e83852_d_n4;
        locals.var_iii_dn5 = assign50450_e83852_d_n5;
        locals.var_iii_dn6 = assign50450_e83852_d_n6;
        locals.var_iii_dn7 = assign50450_e83852_d_n7;
        locals.var_iii_dn8 = assign50450_e83852_d_n8;
        locals.var_iii_dn9 = assign50450_e83852_d_n9;
        locals.var_iii_dn10 = assign50450_e83852_d_n10;
        locals.var_iii_dn11 = assign50450_e83852_d_n11;

    }

    pub(super) fn stamp_transient_block_171(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign50460_e83873, assign50460_e83873_d_n3, assign50460_e83873_d_n4, assign50460_e83873_d_n5, assign50460_e83873_d_n6, assign50460_e83873_d_n7, assign50460_e83873_d_n8, assign50460_e83873_d_n9, assign50460_e83873_d_n10, assign50460_e83873_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 != 0.0)) && (locals.var_guard769 == 0.0)) && (locals.var_guard770 == 0.0)) {
        let assign50460_e83865: f64 = (locals.var_alpha0_i * locals.var_diffvds);
        let assign50460_e83867: f64 = (assign50460_e83865 * locals.var_ids);
        let assign50460_e83869: f64 = (assign50460_e83867 * 1.804851387e-35);
        let assign50460_e83871: f64 = (assign50460_e83869 / locals.var_mscbe);
        (assign50460_e83871, (((((((locals.var_alpha0_i * locals.var_diffvds_dn3) * locals.var_ids) + (assign50460_e83865 * locals.var_ids_dn3)) * 1.804851387e-35) * locals.var_mscbe) - (assign50460_e83869 * locals.var_mscbe_dn3)) / (locals.var_mscbe * locals.var_mscbe)), (((((((locals.var_alpha0_i * locals.var_diffvds_dn4) * locals.var_ids) + (assign50460_e83865 * locals.var_ids_dn4)) * 1.804851387e-35) * locals.var_mscbe) - (assign50460_e83869 * locals.var_mscbe_dn4)) / (locals.var_mscbe * locals.var_mscbe)), (((((((locals.var_alpha0_i * locals.var_diffvds_dn5) * locals.var_ids) + (assign50460_e83865 * locals.var_ids_dn5)) * 1.804851387e-35) * locals.var_mscbe) - (assign50460_e83869 * locals.var_mscbe_dn5)) / (locals.var_mscbe * locals.var_mscbe)), (((((((locals.var_alpha0_i * locals.var_diffvds_dn6) * locals.var_ids) + (assign50460_e83865 * locals.var_ids_dn6)) * 1.804851387e-35) * locals.var_mscbe) - (assign50460_e83869 * locals.var_mscbe_dn6)) / (locals.var_mscbe * locals.var_mscbe)), (((((((locals.var_alpha0_i * locals.var_diffvds_dn7) * locals.var_ids) + (assign50460_e83865 * locals.var_ids_dn7)) * 1.804851387e-35) * locals.var_mscbe) - (assign50460_e83869 * locals.var_mscbe_dn7)) / (locals.var_mscbe * locals.var_mscbe)), (((((((locals.var_alpha0_i * locals.var_diffvds_dn8) * locals.var_ids) + (assign50460_e83865 * locals.var_ids_dn8)) * 1.804851387e-35) * locals.var_mscbe) - (assign50460_e83869 * locals.var_mscbe_dn8)) / (locals.var_mscbe * locals.var_mscbe)), (((((((locals.var_alpha0_i * locals.var_diffvds_dn9) * locals.var_ids) + (assign50460_e83865 * locals.var_ids_dn9)) * 1.804851387e-35) * locals.var_mscbe) - (assign50460_e83869 * locals.var_mscbe_dn9)) / (locals.var_mscbe * locals.var_mscbe)), (((((((locals.var_alpha0_i * locals.var_diffvds_dn10) * locals.var_ids) + (assign50460_e83865 * locals.var_ids_dn10)) * 1.804851387e-35) * locals.var_mscbe) - (assign50460_e83869 * locals.var_mscbe_dn10)) / (locals.var_mscbe * locals.var_mscbe)), (((((((locals.var_alpha0_i * locals.var_diffvds_dn11) * locals.var_ids) + (assign50460_e83865 * locals.var_ids_dn11)) * 1.804851387e-35) * locals.var_mscbe) - (assign50460_e83869 * locals.var_mscbe_dn11)) / (locals.var_mscbe * locals.var_mscbe)),)
    } else {
        (locals.var_iii, locals.var_iii_dn3, locals.var_iii_dn4, locals.var_iii_dn5, locals.var_iii_dn6, locals.var_iii_dn7, locals.var_iii_dn8, locals.var_iii_dn9, locals.var_iii_dn10, locals.var_iii_dn11,)
    }
};
        locals.var_iii = assign50460_e83873;
        locals.var_iii_dn3 = assign50460_e83873_d_n3;
        locals.var_iii_dn4 = assign50460_e83873_d_n4;
        locals.var_iii_dn5 = assign50460_e83873_d_n5;
        locals.var_iii_dn6 = assign50460_e83873_d_n6;
        locals.var_iii_dn7 = assign50460_e83873_d_n7;
        locals.var_iii_dn8 = assign50460_e83873_d_n8;
        locals.var_iii_dn9 = assign50460_e83873_d_n9;
        locals.var_iii_dn10 = assign50460_e83873_d_n10;
        locals.var_iii_dn11 = assign50460_e83873_d_n11;

        let assign50470_e83876: f64 = if p.p44 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard771 = assign50470_e83876;

        let assign50480_e83891: f64 = if ((locals.var_alpha0_i <= 0.0) || (((locals.var_beta2_i == 0.0) && (locals.var_beta1_i == 0.0)) && (locals.var_beta0_t == 0.0))) { 1.0 } else { 0.0 };
        locals.var_guard772 = assign50480_e83891;

        let (assign50490_e83903, assign50490_e83903_d_n3, assign50490_e83903_d_n4, assign50490_e83903_d_n5, assign50490_e83903_d_n6, assign50490_e83903_d_n7, assign50490_e83903_d_n8, assign50490_e83903_d_n9, assign50490_e83903_d_n10, assign50490_e83903_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 != 0.0)) && (locals.var_guard772 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_iii, locals.var_iii_dn3, locals.var_iii_dn4, locals.var_iii_dn5, locals.var_iii_dn6, locals.var_iii_dn7, locals.var_iii_dn8, locals.var_iii_dn9, locals.var_iii_dn10, locals.var_iii_dn11,)
    }
};
        locals.var_iii = assign50490_e83903;
        locals.var_iii_dn3 = assign50490_e83903_d_n3;
        locals.var_iii_dn4 = assign50490_e83903_d_n4;
        locals.var_iii_dn5 = assign50490_e83903_d_n5;
        locals.var_iii_dn6 = assign50490_e83903_d_n6;
        locals.var_iii_dn7 = assign50490_e83903_d_n7;
        locals.var_iii_dn8 = assign50490_e83903_d_n8;
        locals.var_iii_dn9 = assign50490_e83903_d_n9;
        locals.var_iii_dn10 = assign50490_e83903_d_n10;
        locals.var_iii_dn11 = assign50490_e83903_d_n11;

        let (assign50500_e83928, assign50500_e83928_d_n4, assign50500_e83928_d_n5,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 != 0.0)) && (locals.var_guard772 == 0.0)) {
        let assign50500_e83919: f64 = (locals.var_tratio - 1.0);
        let assign50500_e83920: f64 = (p.p600 * assign50500_e83919);
        let assign50500_e83921: f64 = (1.0 + assign50500_e83920);
        let assign50500_e83922: f64 = (locals.var_vdsatii0_i * assign50500_e83921);
        let assign50500_e83925: f64 = (locals.var_lii_i / locals.var_leff);
        let assign50500_e83926: f64 = (assign50500_e83922 - assign50500_e83925);
        (assign50500_e83926, (locals.var_vdsatii0_i * (p.p600 * locals.var_tratio_dn4)), (locals.var_vdsatii0_i * (p.p600 * locals.var_tratio_dn5)),)
    } else {
        (locals.var_vdsatii0, locals.var_vdsatii0_dn4, locals.var_vdsatii0_dn5,)
    }
};
        locals.var_vdsatii0 = assign50500_e83928;
        locals.var_vdsatii0_dn4 = assign50500_e83928_d_n4;
        locals.var_vdsatii0_dn5 = assign50500_e83928_d_n5;

        let (assign50510_e83943, assign50510_e83943_d_n3, assign50510_e83943_d_n4, assign50510_e83943_d_n5, assign50510_e83943_d_n6, assign50510_e83943_d_n7, assign50510_e83943_d_n8, assign50510_e83943_d_n9, assign50510_e83943_d_n10, assign50510_e83943_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 != 0.0)) && (locals.var_guard772 == 0.0)) {
        let assign50510_e83941: f64 = (locals.var_esatii_i * locals.var_leff);
        (assign50510_e83941, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign50510_e83943;
        locals.var_t0_dn3 = assign50510_e83943_d_n3;
        locals.var_t0_dn4 = assign50510_e83943_d_n4;
        locals.var_t0_dn5 = assign50510_e83943_d_n5;
        locals.var_t0_dn6 = assign50510_e83943_d_n6;
        locals.var_t0_dn7 = assign50510_e83943_d_n7;
        locals.var_t0_dn8 = assign50510_e83943_d_n8;
        locals.var_t0_dn9 = assign50510_e83943_d_n9;
        locals.var_t0_dn10 = assign50510_e83943_d_n10;
        locals.var_t0_dn11 = assign50510_e83943_d_n11;

        let (assign50520_e83962, assign50520_e83962_d_n3, assign50520_e83962_d_n4, assign50520_e83962_d_n5, assign50520_e83962_d_n6, assign50520_e83962_d_n7, assign50520_e83962_d_n8, assign50520_e83962_d_n9, assign50520_e83962_d_n10, assign50520_e83962_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 != 0.0)) && (locals.var_guard772 == 0.0)) {
        let assign50520_e83956: f64 = (locals.var_sii0_i * locals.var_t0);
        let assign50520_e83959: f64 = (1.0 + locals.var_t0);
        let assign50520_e83960: f64 = (assign50520_e83956 / assign50520_e83959);
        (assign50520_e83960, ((((locals.var_sii0_i * locals.var_t0_dn3) * assign50520_e83959) - (assign50520_e83956 * locals.var_t0_dn3)) / (assign50520_e83959 * assign50520_e83959)), ((((locals.var_sii0_i * locals.var_t0_dn4) * assign50520_e83959) - (assign50520_e83956 * locals.var_t0_dn4)) / (assign50520_e83959 * assign50520_e83959)), ((((locals.var_sii0_i * locals.var_t0_dn5) * assign50520_e83959) - (assign50520_e83956 * locals.var_t0_dn5)) / (assign50520_e83959 * assign50520_e83959)), ((((locals.var_sii0_i * locals.var_t0_dn6) * assign50520_e83959) - (assign50520_e83956 * locals.var_t0_dn6)) / (assign50520_e83959 * assign50520_e83959)), ((((locals.var_sii0_i * locals.var_t0_dn7) * assign50520_e83959) - (assign50520_e83956 * locals.var_t0_dn7)) / (assign50520_e83959 * assign50520_e83959)), ((((locals.var_sii0_i * locals.var_t0_dn8) * assign50520_e83959) - (assign50520_e83956 * locals.var_t0_dn8)) / (assign50520_e83959 * assign50520_e83959)), ((((locals.var_sii0_i * locals.var_t0_dn9) * assign50520_e83959) - (assign50520_e83956 * locals.var_t0_dn9)) / (assign50520_e83959 * assign50520_e83959)), ((((locals.var_sii0_i * locals.var_t0_dn10) * assign50520_e83959) - (assign50520_e83956 * locals.var_t0_dn10)) / (assign50520_e83959 * assign50520_e83959)), ((((locals.var_sii0_i * locals.var_t0_dn11) * assign50520_e83959) - (assign50520_e83956 * locals.var_t0_dn11)) / (assign50520_e83959 * assign50520_e83959)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign50520_e83962;
        locals.var_t1_dn3 = assign50520_e83962_d_n3;
        locals.var_t1_dn4 = assign50520_e83962_d_n4;
        locals.var_t1_dn5 = assign50520_e83962_d_n5;
        locals.var_t1_dn6 = assign50520_e83962_d_n6;
        locals.var_t1_dn7 = assign50520_e83962_d_n7;
        locals.var_t1_dn8 = assign50520_e83962_d_n8;
        locals.var_t1_dn9 = assign50520_e83962_d_n9;
        locals.var_t1_dn10 = assign50520_e83962_d_n10;
        locals.var_t1_dn11 = assign50520_e83962_d_n11;

        let (assign50530_e84004, assign50530_e84004_d_n3, assign50530_e84004_d_n4, assign50530_e84004_d_n5, assign50530_e84004_d_n6, assign50530_e84004_d_n7, assign50530_e84004_d_n8, assign50530_e84004_d_n9, assign50530_e84004_d_n10, assign50530_e84004_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 != 0.0)) && (locals.var_guard772 == 0.0)) {
        let assign50530_e83978: f64 = (locals.var_sii1_i * locals.var_vgsfb);
        let assign50530_e83980: f64 = (assign50530_e83978 * locals.var_nvt);
        let assign50530_e83983: f64 = (locals.var_sii1_i * locals.var_vgsfb);
        let assign50530_e83985: f64 = (assign50530_e83983 * locals.var_nvt);
        let assign50530_e83988: f64 = (locals.var_sii1_i * locals.var_vgsfb);
        let assign50530_e83990: f64 = (assign50530_e83988 * locals.var_nvt);
        let assign50530_e83991: f64 = (assign50530_e83985 * assign50530_e83990);
        let assign50530_e83994: f64 = (4.0 * p.p643);
        let assign50530_e83996: f64 = (assign50530_e83994 * p.p643);
        let assign50530_e83997: f64 = (assign50530_e83991 + assign50530_e83996);
        let assign50530_e83998: f64 = (assign50530_e83997).sqrt();
        let assign50530_e83999: f64 = (assign50530_e83980 + assign50530_e83998);
        let assign50530_e84000: f64 = (0.5 * assign50530_e83999);
        let assign50530_e84001: f64 = (1.0 + assign50530_e84000);
        let assign50530_e84002: f64 = (1.0 / assign50530_e84001);
        (assign50530_e84002, (-((0.5 * ((((locals.var_sii1_i * locals.var_vgsfb_dn3) * locals.var_nvt) + (assign50530_e83978 * locals.var_nvt_dn3)) + ((((((locals.var_sii1_i * locals.var_vgsfb_dn3) * locals.var_nvt) + (assign50530_e83983 * locals.var_nvt_dn3)) * assign50530_e83990) + (assign50530_e83985 * (((locals.var_sii1_i * locals.var_vgsfb_dn3) * locals.var_nvt) + (assign50530_e83988 * locals.var_nvt_dn3)))) / (2.0 * assign50530_e83998)))) / (assign50530_e84001 * assign50530_e84001))), (-((0.5 * ((((locals.var_sii1_i * locals.var_vgsfb_dn4) * locals.var_nvt) + (assign50530_e83978 * locals.var_nvt_dn4)) + ((((((locals.var_sii1_i * locals.var_vgsfb_dn4) * locals.var_nvt) + (assign50530_e83983 * locals.var_nvt_dn4)) * assign50530_e83990) + (assign50530_e83985 * (((locals.var_sii1_i * locals.var_vgsfb_dn4) * locals.var_nvt) + (assign50530_e83988 * locals.var_nvt_dn4)))) / (2.0 * assign50530_e83998)))) / (assign50530_e84001 * assign50530_e84001))), (-((0.5 * ((((locals.var_sii1_i * locals.var_vgsfb_dn5) * locals.var_nvt) + (assign50530_e83978 * locals.var_nvt_dn5)) + ((((((locals.var_sii1_i * locals.var_vgsfb_dn5) * locals.var_nvt) + (assign50530_e83983 * locals.var_nvt_dn5)) * assign50530_e83990) + (assign50530_e83985 * (((locals.var_sii1_i * locals.var_vgsfb_dn5) * locals.var_nvt) + (assign50530_e83988 * locals.var_nvt_dn5)))) / (2.0 * assign50530_e83998)))) / (assign50530_e84001 * assign50530_e84001))), (-((0.5 * ((((locals.var_sii1_i * locals.var_vgsfb_dn6) * locals.var_nvt) + (assign50530_e83978 * locals.var_nvt_dn6)) + ((((((locals.var_sii1_i * locals.var_vgsfb_dn6) * locals.var_nvt) + (assign50530_e83983 * locals.var_nvt_dn6)) * assign50530_e83990) + (assign50530_e83985 * (((locals.var_sii1_i * locals.var_vgsfb_dn6) * locals.var_nvt) + (assign50530_e83988 * locals.var_nvt_dn6)))) / (2.0 * assign50530_e83998)))) / (assign50530_e84001 * assign50530_e84001))), (-((0.5 * ((((locals.var_sii1_i * locals.var_vgsfb_dn7) * locals.var_nvt) + (assign50530_e83978 * locals.var_nvt_dn7)) + ((((((locals.var_sii1_i * locals.var_vgsfb_dn7) * locals.var_nvt) + (assign50530_e83983 * locals.var_nvt_dn7)) * assign50530_e83990) + (assign50530_e83985 * (((locals.var_sii1_i * locals.var_vgsfb_dn7) * locals.var_nvt) + (assign50530_e83988 * locals.var_nvt_dn7)))) / (2.0 * assign50530_e83998)))) / (assign50530_e84001 * assign50530_e84001))), (-((0.5 * ((((locals.var_sii1_i * locals.var_vgsfb_dn8) * locals.var_nvt) + (assign50530_e83978 * locals.var_nvt_dn8)) + ((((((locals.var_sii1_i * locals.var_vgsfb_dn8) * locals.var_nvt) + (assign50530_e83983 * locals.var_nvt_dn8)) * assign50530_e83990) + (assign50530_e83985 * (((locals.var_sii1_i * locals.var_vgsfb_dn8) * locals.var_nvt) + (assign50530_e83988 * locals.var_nvt_dn8)))) / (2.0 * assign50530_e83998)))) / (assign50530_e84001 * assign50530_e84001))), (-((0.5 * ((((locals.var_sii1_i * locals.var_vgsfb_dn9) * locals.var_nvt) + (assign50530_e83978 * locals.var_nvt_dn9)) + ((((((locals.var_sii1_i * locals.var_vgsfb_dn9) * locals.var_nvt) + (assign50530_e83983 * locals.var_nvt_dn9)) * assign50530_e83990) + (assign50530_e83985 * (((locals.var_sii1_i * locals.var_vgsfb_dn9) * locals.var_nvt) + (assign50530_e83988 * locals.var_nvt_dn9)))) / (2.0 * assign50530_e83998)))) / (assign50530_e84001 * assign50530_e84001))), (-((0.5 * ((((locals.var_sii1_i * locals.var_vgsfb_dn10) * locals.var_nvt) + (assign50530_e83978 * locals.var_nvt_dn10)) + ((((((locals.var_sii1_i * locals.var_vgsfb_dn10) * locals.var_nvt) + (assign50530_e83983 * locals.var_nvt_dn10)) * assign50530_e83990) + (assign50530_e83985 * (((locals.var_sii1_i * locals.var_vgsfb_dn10) * locals.var_nvt) + (assign50530_e83988 * locals.var_nvt_dn10)))) / (2.0 * assign50530_e83998)))) / (assign50530_e84001 * assign50530_e84001))), (-((0.5 * ((((locals.var_sii1_i * locals.var_vgsfb_dn11) * locals.var_nvt) + (assign50530_e83978 * locals.var_nvt_dn11)) + ((((((locals.var_sii1_i * locals.var_vgsfb_dn11) * locals.var_nvt) + (assign50530_e83983 * locals.var_nvt_dn11)) * assign50530_e83990) + (assign50530_e83985 * (((locals.var_sii1_i * locals.var_vgsfb_dn11) * locals.var_nvt) + (assign50530_e83988 * locals.var_nvt_dn11)))) / (2.0 * assign50530_e83998)))) / (assign50530_e84001 * assign50530_e84001))),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign50530_e84004;
        locals.var_t0_dn3 = assign50530_e84004_d_n3;
        locals.var_t0_dn4 = assign50530_e84004_d_n4;
        locals.var_t0_dn5 = assign50530_e84004_d_n5;
        locals.var_t0_dn6 = assign50530_e84004_d_n6;
        locals.var_t0_dn7 = assign50530_e84004_d_n7;
        locals.var_t0_dn8 = assign50530_e84004_d_n8;
        locals.var_t0_dn9 = assign50530_e84004_d_n9;
        locals.var_t0_dn10 = assign50530_e84004_d_n10;
        locals.var_t0_dn11 = assign50530_e84004_d_n11;

        let (assign50540_e84019, assign50540_e84019_d_n3, assign50540_e84019_d_n4, assign50540_e84019_d_n5, assign50540_e84019_d_n6, assign50540_e84019_d_n7, assign50540_e84019_d_n8, assign50540_e84019_d_n9, assign50540_e84019_d_n10, assign50540_e84019_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 != 0.0)) && (locals.var_guard772 == 0.0)) {
        let assign50540_e84017: f64 = (locals.var_t0 + locals.var_sii2_i);
        (assign50540_e84017, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign50540_e84019;
        locals.var_t3_dn3 = assign50540_e84019_d_n3;
        locals.var_t3_dn4 = assign50540_e84019_d_n4;
        locals.var_t3_dn5 = assign50540_e84019_d_n5;
        locals.var_t3_dn6 = assign50540_e84019_d_n6;
        locals.var_t3_dn7 = assign50540_e84019_d_n7;
        locals.var_t3_dn8 = assign50540_e84019_d_n8;
        locals.var_t3_dn9 = assign50540_e84019_d_n9;
        locals.var_t3_dn10 = assign50540_e84019_d_n10;
        locals.var_t3_dn11 = assign50540_e84019_d_n11;

        let (assign50550_e84057, assign50550_e84057_d_n3, assign50550_e84057_d_n4, assign50550_e84057_d_n5, assign50550_e84057_d_n6, assign50550_e84057_d_n7, assign50550_e84057_d_n8, assign50550_e84057_d_n9, assign50550_e84057_d_n10, assign50550_e84057_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 != 0.0)) && (locals.var_guard772 == 0.0)) {
        let assign50550_e84033: f64 = (locals.var_vgsfb * locals.var_nvt);
        let assign50550_e84035: f64 = (assign50550_e84033 * locals.var_t3);
        let assign50550_e84038: f64 = (locals.var_vgsfb * locals.var_nvt);
        let assign50550_e84040: f64 = (assign50550_e84038 * locals.var_t3);
        let assign50550_e84043: f64 = (locals.var_vgsfb * locals.var_nvt);
        let assign50550_e84045: f64 = (assign50550_e84043 * locals.var_t3);
        let assign50550_e84046: f64 = (assign50550_e84040 * assign50550_e84045);
        let assign50550_e84049: f64 = (4.0 * p.p644);
        let assign50550_e84051: f64 = (assign50550_e84049 * p.p644);
        let assign50550_e84052: f64 = (assign50550_e84046 + assign50550_e84051);
        let assign50550_e84053: f64 = (assign50550_e84052).sqrt();
        let assign50550_e84054: f64 = (assign50550_e84035 + assign50550_e84053);
        let assign50550_e84055: f64 = (0.5 * assign50550_e84054);
        (assign50550_e84055, (0.5 * (((((locals.var_vgsfb_dn3 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn3)) * locals.var_t3) + (assign50550_e84033 * locals.var_t3_dn3)) + (((((((locals.var_vgsfb_dn3 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn3)) * locals.var_t3) + (assign50550_e84038 * locals.var_t3_dn3)) * assign50550_e84045) + (assign50550_e84040 * ((((locals.var_vgsfb_dn3 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn3)) * locals.var_t3) + (assign50550_e84043 * locals.var_t3_dn3)))) / (2.0 * assign50550_e84053)))), (0.5 * (((((locals.var_vgsfb_dn4 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn4)) * locals.var_t3) + (assign50550_e84033 * locals.var_t3_dn4)) + (((((((locals.var_vgsfb_dn4 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn4)) * locals.var_t3) + (assign50550_e84038 * locals.var_t3_dn4)) * assign50550_e84045) + (assign50550_e84040 * ((((locals.var_vgsfb_dn4 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn4)) * locals.var_t3) + (assign50550_e84043 * locals.var_t3_dn4)))) / (2.0 * assign50550_e84053)))), (0.5 * (((((locals.var_vgsfb_dn5 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn5)) * locals.var_t3) + (assign50550_e84033 * locals.var_t3_dn5)) + (((((((locals.var_vgsfb_dn5 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn5)) * locals.var_t3) + (assign50550_e84038 * locals.var_t3_dn5)) * assign50550_e84045) + (assign50550_e84040 * ((((locals.var_vgsfb_dn5 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn5)) * locals.var_t3) + (assign50550_e84043 * locals.var_t3_dn5)))) / (2.0 * assign50550_e84053)))), (0.5 * (((((locals.var_vgsfb_dn6 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn6)) * locals.var_t3) + (assign50550_e84033 * locals.var_t3_dn6)) + (((((((locals.var_vgsfb_dn6 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn6)) * locals.var_t3) + (assign50550_e84038 * locals.var_t3_dn6)) * assign50550_e84045) + (assign50550_e84040 * ((((locals.var_vgsfb_dn6 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn6)) * locals.var_t3) + (assign50550_e84043 * locals.var_t3_dn6)))) / (2.0 * assign50550_e84053)))), (0.5 * (((((locals.var_vgsfb_dn7 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn7)) * locals.var_t3) + (assign50550_e84033 * locals.var_t3_dn7)) + (((((((locals.var_vgsfb_dn7 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn7)) * locals.var_t3) + (assign50550_e84038 * locals.var_t3_dn7)) * assign50550_e84045) + (assign50550_e84040 * ((((locals.var_vgsfb_dn7 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn7)) * locals.var_t3) + (assign50550_e84043 * locals.var_t3_dn7)))) / (2.0 * assign50550_e84053)))), (0.5 * (((((locals.var_vgsfb_dn8 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn8)) * locals.var_t3) + (assign50550_e84033 * locals.var_t3_dn8)) + (((((((locals.var_vgsfb_dn8 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn8)) * locals.var_t3) + (assign50550_e84038 * locals.var_t3_dn8)) * assign50550_e84045) + (assign50550_e84040 * ((((locals.var_vgsfb_dn8 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn8)) * locals.var_t3) + (assign50550_e84043 * locals.var_t3_dn8)))) / (2.0 * assign50550_e84053)))), (0.5 * (((((locals.var_vgsfb_dn9 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn9)) * locals.var_t3) + (assign50550_e84033 * locals.var_t3_dn9)) + (((((((locals.var_vgsfb_dn9 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn9)) * locals.var_t3) + (assign50550_e84038 * locals.var_t3_dn9)) * assign50550_e84045) + (assign50550_e84040 * ((((locals.var_vgsfb_dn9 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn9)) * locals.var_t3) + (assign50550_e84043 * locals.var_t3_dn9)))) / (2.0 * assign50550_e84053)))), (0.5 * (((((locals.var_vgsfb_dn10 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn10)) * locals.var_t3) + (assign50550_e84033 * locals.var_t3_dn10)) + (((((((locals.var_vgsfb_dn10 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn10)) * locals.var_t3) + (assign50550_e84038 * locals.var_t3_dn10)) * assign50550_e84045) + (assign50550_e84040 * ((((locals.var_vgsfb_dn10 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn10)) * locals.var_t3) + (assign50550_e84043 * locals.var_t3_dn10)))) / (2.0 * assign50550_e84053)))), (0.5 * (((((locals.var_vgsfb_dn11 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn11)) * locals.var_t3) + (assign50550_e84033 * locals.var_t3_dn11)) + (((((((locals.var_vgsfb_dn11 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn11)) * locals.var_t3) + (assign50550_e84038 * locals.var_t3_dn11)) * assign50550_e84045) + (assign50550_e84040 * ((((locals.var_vgsfb_dn11 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn11)) * locals.var_t3) + (assign50550_e84043 * locals.var_t3_dn11)))) / (2.0 * assign50550_e84053)))),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign50550_e84057;
        locals.var_t2_dn3 = assign50550_e84057_d_n3;
        locals.var_t2_dn4 = assign50550_e84057_d_n4;
        locals.var_t2_dn5 = assign50550_e84057_d_n5;
        locals.var_t2_dn6 = assign50550_e84057_d_n6;
        locals.var_t2_dn7 = assign50550_e84057_d_n7;
        locals.var_t2_dn8 = assign50550_e84057_d_n8;
        locals.var_t2_dn9 = assign50550_e84057_d_n9;
        locals.var_t2_dn10 = assign50550_e84057_d_n10;
        locals.var_t2_dn11 = assign50550_e84057_d_n11;

        let (assign50560_e84076, assign50560_e84076_d_n3, assign50560_e84076_d_n4, assign50560_e84076_d_n5, assign50560_e84076_d_n6, assign50560_e84076_d_n7, assign50560_e84076_d_n8, assign50560_e84076_d_n9, assign50560_e84076_d_n10, assign50560_e84076_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 != 0.0)) && (locals.var_guard772 == 0.0)) {
        let assign50560_e84072: f64 = (locals.var_siid_i * locals.var_vdsx);
        let assign50560_e84073: f64 = (1.0 + assign50560_e84072);
        let assign50560_e84074: f64 = (1.0 / assign50560_e84073);
        (assign50560_e84074, (-((locals.var_siid_i * locals.var_vdsx_dn3) / (assign50560_e84073 * assign50560_e84073))), (-((locals.var_siid_i * locals.var_vdsx_dn4) / (assign50560_e84073 * assign50560_e84073))), (-((locals.var_siid_i * locals.var_vdsx_dn5) / (assign50560_e84073 * assign50560_e84073))), (-((locals.var_siid_i * locals.var_vdsx_dn6) / (assign50560_e84073 * assign50560_e84073))), (-((locals.var_siid_i * locals.var_vdsx_dn7) / (assign50560_e84073 * assign50560_e84073))), (-((locals.var_siid_i * locals.var_vdsx_dn8) / (assign50560_e84073 * assign50560_e84073))), (-((locals.var_siid_i * locals.var_vdsx_dn9) / (assign50560_e84073 * assign50560_e84073))), (-((locals.var_siid_i * locals.var_vdsx_dn10) / (assign50560_e84073 * assign50560_e84073))), (-((locals.var_siid_i * locals.var_vdsx_dn11) / (assign50560_e84073 * assign50560_e84073))),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign50560_e84076;
        locals.var_t3_dn3 = assign50560_e84076_d_n3;
        locals.var_t3_dn4 = assign50560_e84076_d_n4;
        locals.var_t3_dn5 = assign50560_e84076_d_n5;
        locals.var_t3_dn6 = assign50560_e84076_d_n6;
        locals.var_t3_dn7 = assign50560_e84076_d_n7;
        locals.var_t3_dn8 = assign50560_e84076_d_n8;
        locals.var_t3_dn9 = assign50560_e84076_d_n9;
        locals.var_t3_dn10 = assign50560_e84076_d_n10;
        locals.var_t3_dn11 = assign50560_e84076_d_n11;

        let (assign50570_e84093, assign50570_e84093_d_n3, assign50570_e84093_d_n4, assign50570_e84093_d_n5, assign50570_e84093_d_n6, assign50570_e84093_d_n7, assign50570_e84093_d_n8, assign50570_e84093_d_n9, assign50570_e84093_d_n10, assign50570_e84093_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 != 0.0)) && (locals.var_guard772 == 0.0)) {
        let assign50570_e84089: f64 = (locals.var_t1 * locals.var_t2);
        let assign50570_e84091: f64 = (assign50570_e84089 * locals.var_t3);
        (assign50570_e84091, ((((locals.var_t1_dn3 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn3)) * locals.var_t3) + (assign50570_e84089 * locals.var_t3_dn3)), ((((locals.var_t1_dn4 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn4)) * locals.var_t3) + (assign50570_e84089 * locals.var_t3_dn4)), ((((locals.var_t1_dn5 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn5)) * locals.var_t3) + (assign50570_e84089 * locals.var_t3_dn5)), ((((locals.var_t1_dn6 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn6)) * locals.var_t3) + (assign50570_e84089 * locals.var_t3_dn6)), ((((locals.var_t1_dn7 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn7)) * locals.var_t3) + (assign50570_e84089 * locals.var_t3_dn7)), ((((locals.var_t1_dn8 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn8)) * locals.var_t3) + (assign50570_e84089 * locals.var_t3_dn8)), ((((locals.var_t1_dn9 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn9)) * locals.var_t3) + (assign50570_e84089 * locals.var_t3_dn9)), ((((locals.var_t1_dn10 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn10)) * locals.var_t3) + (assign50570_e84089 * locals.var_t3_dn10)), ((((locals.var_t1_dn11 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn11)) * locals.var_t3) + (assign50570_e84089 * locals.var_t3_dn11)),)
    } else {
        (locals.var_vgsstep, locals.var_vgsstep_dn3, locals.var_vgsstep_dn4, locals.var_vgsstep_dn5, locals.var_vgsstep_dn6, locals.var_vgsstep_dn7, locals.var_vgsstep_dn8, locals.var_vgsstep_dn9, locals.var_vgsstep_dn10, locals.var_vgsstep_dn11,)
    }
};
        locals.var_vgsstep = assign50570_e84093;
        locals.var_vgsstep_dn3 = assign50570_e84093_d_n3;
        locals.var_vgsstep_dn4 = assign50570_e84093_d_n4;
        locals.var_vgsstep_dn5 = assign50570_e84093_d_n5;
        locals.var_vgsstep_dn6 = assign50570_e84093_d_n6;
        locals.var_vgsstep_dn7 = assign50570_e84093_d_n7;
        locals.var_vgsstep_dn8 = assign50570_e84093_d_n8;
        locals.var_vgsstep_dn9 = assign50570_e84093_d_n9;
        locals.var_vgsstep_dn10 = assign50570_e84093_d_n10;
        locals.var_vgsstep_dn11 = assign50570_e84093_d_n11;

        let (assign50580_e84108, assign50580_e84108_d_n3, assign50580_e84108_d_n4, assign50580_e84108_d_n5, assign50580_e84108_d_n6, assign50580_e84108_d_n7, assign50580_e84108_d_n8, assign50580_e84108_d_n9, assign50580_e84108_d_n10, assign50580_e84108_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 != 0.0)) && (locals.var_guard772 == 0.0)) {
        let assign50580_e84106: f64 = (locals.var_vdsatii0 + locals.var_vgsstep);
        (assign50580_e84106, locals.var_vgsstep_dn3, (locals.var_vdsatii0_dn4 + locals.var_vgsstep_dn4), (locals.var_vdsatii0_dn5 + locals.var_vgsstep_dn5), locals.var_vgsstep_dn6, locals.var_vgsstep_dn7, locals.var_vgsstep_dn8, locals.var_vgsstep_dn9, locals.var_vgsstep_dn10, locals.var_vgsstep_dn11,)
    } else {
        (locals.var_vdsatii, locals.var_vdsatii_dn3, locals.var_vdsatii_dn4, locals.var_vdsatii_dn5, locals.var_vdsatii_dn6, locals.var_vdsatii_dn7, locals.var_vdsatii_dn8, locals.var_vdsatii_dn9, locals.var_vdsatii_dn10, locals.var_vdsatii_dn11,)
    }
};
        locals.var_vdsatii = assign50580_e84108;
        locals.var_vdsatii_dn3 = assign50580_e84108_d_n3;
        locals.var_vdsatii_dn4 = assign50580_e84108_d_n4;
        locals.var_vdsatii_dn5 = assign50580_e84108_d_n5;
        locals.var_vdsatii_dn6 = assign50580_e84108_d_n6;
        locals.var_vdsatii_dn7 = assign50580_e84108_d_n7;
        locals.var_vdsatii_dn8 = assign50580_e84108_d_n8;
        locals.var_vdsatii_dn9 = assign50580_e84108_d_n9;
        locals.var_vdsatii_dn10 = assign50580_e84108_d_n10;
        locals.var_vdsatii_dn11 = assign50580_e84108_d_n11;

        let (assign50590_e84123, assign50590_e84123_d_n3, assign50590_e84123_d_n4, assign50590_e84123_d_n5, assign50590_e84123_d_n6, assign50590_e84123_d_n7, assign50590_e84123_d_n8, assign50590_e84123_d_n9, assign50590_e84123_d_n10, assign50590_e84123_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 != 0.0)) && (locals.var_guard772 == 0.0)) {
        let assign50590_e84121: f64 = (locals.var_vdsx - locals.var_vdsatii);
        (assign50590_e84121, (locals.var_vdsx_dn3 - locals.var_vdsatii_dn3), (locals.var_vdsx_dn4 - locals.var_vdsatii_dn4), (locals.var_vdsx_dn5 - locals.var_vdsatii_dn5), (locals.var_vdsx_dn6 - locals.var_vdsatii_dn6), (locals.var_vdsx_dn7 - locals.var_vdsatii_dn7), (locals.var_vdsx_dn8 - locals.var_vdsatii_dn8), (locals.var_vdsx_dn9 - locals.var_vdsatii_dn9), (locals.var_vdsx_dn10 - locals.var_vdsatii_dn10), (locals.var_vdsx_dn11 - locals.var_vdsatii_dn11),)
    } else {
        (locals.var_vdiff, locals.var_vdiff_dn3, locals.var_vdiff_dn4, locals.var_vdiff_dn5, locals.var_vdiff_dn6, locals.var_vdiff_dn7, locals.var_vdiff_dn8, locals.var_vdiff_dn9, locals.var_vdiff_dn10, locals.var_vdiff_dn11,)
    }
};
        locals.var_vdiff = assign50590_e84123;
        locals.var_vdiff_dn3 = assign50590_e84123_d_n3;
        locals.var_vdiff_dn4 = assign50590_e84123_d_n4;
        locals.var_vdiff_dn5 = assign50590_e84123_d_n5;
        locals.var_vdiff_dn6 = assign50590_e84123_d_n6;
        locals.var_vdiff_dn7 = assign50590_e84123_d_n7;
        locals.var_vdiff_dn8 = assign50590_e84123_d_n8;
        locals.var_vdiff_dn9 = assign50590_e84123_d_n9;
        locals.var_vdiff_dn10 = assign50590_e84123_d_n10;
        locals.var_vdiff_dn11 = assign50590_e84123_d_n11;

        let (assign50600_e84146, assign50600_e84146_d_n3, assign50600_e84146_d_n4, assign50600_e84146_d_n5, assign50600_e84146_d_n6, assign50600_e84146_d_n7, assign50600_e84146_d_n8, assign50600_e84146_d_n9, assign50600_e84146_d_n10, assign50600_e84146_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 != 0.0)) && (locals.var_guard772 == 0.0)) {
        let assign50600_e84137: f64 = (locals.var_beta1_i * locals.var_vdiff);
        let assign50600_e84138: f64 = (locals.var_beta2_i + assign50600_e84137);
        let assign50600_e84141: f64 = (locals.var_beta0_t * locals.var_vdiff);
        let assign50600_e84143: f64 = (assign50600_e84141 * locals.var_vdiff);
        let assign50600_e84144: f64 = (assign50600_e84138 + assign50600_e84143);
        (assign50600_e84144, ((locals.var_beta1_i * locals.var_vdiff_dn3) + (((locals.var_beta0_t * locals.var_vdiff_dn3) * locals.var_vdiff) + (assign50600_e84141 * locals.var_vdiff_dn3))), ((locals.var_beta1_i * locals.var_vdiff_dn4) + ((((locals.var_beta0_t_dn4 * locals.var_vdiff) + (locals.var_beta0_t * locals.var_vdiff_dn4)) * locals.var_vdiff) + (assign50600_e84141 * locals.var_vdiff_dn4))), ((locals.var_beta1_i * locals.var_vdiff_dn5) + ((((locals.var_beta0_t_dn5 * locals.var_vdiff) + (locals.var_beta0_t * locals.var_vdiff_dn5)) * locals.var_vdiff) + (assign50600_e84141 * locals.var_vdiff_dn5))), ((locals.var_beta1_i * locals.var_vdiff_dn6) + (((locals.var_beta0_t * locals.var_vdiff_dn6) * locals.var_vdiff) + (assign50600_e84141 * locals.var_vdiff_dn6))), ((locals.var_beta1_i * locals.var_vdiff_dn7) + (((locals.var_beta0_t * locals.var_vdiff_dn7) * locals.var_vdiff) + (assign50600_e84141 * locals.var_vdiff_dn7))), ((locals.var_beta1_i * locals.var_vdiff_dn8) + (((locals.var_beta0_t * locals.var_vdiff_dn8) * locals.var_vdiff) + (assign50600_e84141 * locals.var_vdiff_dn8))), ((locals.var_beta1_i * locals.var_vdiff_dn9) + (((locals.var_beta0_t * locals.var_vdiff_dn9) * locals.var_vdiff) + (assign50600_e84141 * locals.var_vdiff_dn9))), ((locals.var_beta1_i * locals.var_vdiff_dn10) + (((locals.var_beta0_t * locals.var_vdiff_dn10) * locals.var_vdiff) + (assign50600_e84141 * locals.var_vdiff_dn10))), ((locals.var_beta1_i * locals.var_vdiff_dn11) + (((locals.var_beta0_t * locals.var_vdiff_dn11) * locals.var_vdiff) + (assign50600_e84141 * locals.var_vdiff_dn11))),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign50600_e84146;
        locals.var_t0_dn3 = assign50600_e84146_d_n3;
        locals.var_t0_dn4 = assign50600_e84146_d_n4;
        locals.var_t0_dn5 = assign50600_e84146_d_n5;
        locals.var_t0_dn6 = assign50600_e84146_d_n6;
        locals.var_t0_dn7 = assign50600_e84146_d_n7;
        locals.var_t0_dn8 = assign50600_e84146_d_n8;
        locals.var_t0_dn9 = assign50600_e84146_d_n9;
        locals.var_t0_dn10 = assign50600_e84146_d_n10;
        locals.var_t0_dn11 = assign50600_e84146_d_n11;

        let (assign50610_e84164, assign50610_e84164_d_n3, assign50610_e84164_d_n4, assign50610_e84164_d_n5, assign50610_e84164_d_n6, assign50610_e84164_d_n7, assign50610_e84164_d_n8, assign50610_e84164_d_n9, assign50610_e84164_d_n10, assign50610_e84164_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 != 0.0)) && (locals.var_guard772 == 0.0)) {
        let assign50610_e84159: f64 = (locals.var_t0 * locals.var_t0);
        let assign50610_e84161: f64 = (assign50610_e84159 + 1e-10);
        let assign50610_e84162: f64 = (assign50610_e84161).sqrt();
        (assign50610_e84162, (((locals.var_t0_dn3 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn3)) / (2.0 * assign50610_e84162)), (((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)) / (2.0 * assign50610_e84162)), (((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)) / (2.0 * assign50610_e84162)), (((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)) / (2.0 * assign50610_e84162)), (((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)) / (2.0 * assign50610_e84162)), (((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)) / (2.0 * assign50610_e84162)), (((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)) / (2.0 * assign50610_e84162)), (((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)) / (2.0 * assign50610_e84162)), (((locals.var_t0_dn11 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn11)) / (2.0 * assign50610_e84162)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign50610_e84164;
        locals.var_t1_dn3 = assign50610_e84164_d_n3;
        locals.var_t1_dn4 = assign50610_e84164_d_n4;
        locals.var_t1_dn5 = assign50610_e84164_d_n5;
        locals.var_t1_dn6 = assign50610_e84164_d_n6;
        locals.var_t1_dn7 = assign50610_e84164_d_n7;
        locals.var_t1_dn8 = assign50610_e84164_d_n8;
        locals.var_t1_dn9 = assign50610_e84164_d_n9;
        locals.var_t1_dn10 = assign50610_e84164_d_n10;
        locals.var_t1_dn11 = assign50610_e84164_d_n11;

        let (assign50620_e84228, assign50620_e84228_d_n3, assign50620_e84228_d_n4, assign50620_e84228_d_n5, assign50620_e84228_d_n6, assign50620_e84228_d_n7, assign50620_e84228_d_n8, assign50620_e84228_d_n9, assign50620_e84228_d_n10, assign50620_e84228_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 != 0.0)) && (locals.var_guard772 == 0.0)) {
        let assign50620_e84176: f64 = (-10.0);
        let assign50620_e84179: f64 = (-locals.var_alpha0_i);
        let assign50620_e84182: f64 = (locals.var_vdiff / locals.var_t1);
        let assign50620_e84183: f64 = { let limited_exp_arg = assign50620_e84182; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign50620_e84184: f64 = (assign50620_e84179 * assign50620_e84183);
        let assign50620_e84186: f64 = (-10.0);
        let assign50620_e84187: f64 = (assign50620_e84184 - assign50620_e84186);
        let assign50620_e84189: f64 = (assign50620_e84187 - p.p645);
        let assign50620_e84191: f64 = (-locals.var_alpha0_i);
        let assign50620_e84194: f64 = (locals.var_vdiff / locals.var_t1);
        let assign50620_e84195: f64 = { let limited_exp_arg = assign50620_e84194; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign50620_e84196: f64 = (assign50620_e84191 * assign50620_e84195);
        let assign50620_e84198: f64 = (-10.0);
        let assign50620_e84199: f64 = (assign50620_e84196 - assign50620_e84198);
        let assign50620_e84201: f64 = (assign50620_e84199 - p.p645);
        let assign50620_e84203: f64 = (-locals.var_alpha0_i);
        let assign50620_e84206: f64 = (locals.var_vdiff / locals.var_t1);
        let assign50620_e84207: f64 = { let limited_exp_arg = assign50620_e84206; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign50620_e84208: f64 = (assign50620_e84203 * assign50620_e84207);
        let assign50620_e84210: f64 = (-10.0);
        let assign50620_e84211: f64 = (assign50620_e84208 - assign50620_e84210);
        let assign50620_e84213: f64 = (assign50620_e84211 - p.p645);
        let assign50620_e84214: f64 = (assign50620_e84201 * assign50620_e84213);
        let assign50620_e84217: f64 = (-10.0);
        let assign50620_e84218: f64 = (4.0 * assign50620_e84217);
        let assign50620_e84220: f64 = (assign50620_e84218 * p.p645);
        let assign50620_e84221: f64 = (assign50620_e84214 - assign50620_e84220);
        let assign50620_e84222: f64 = (assign50620_e84221).sqrt();
        let assign50620_e84223: f64 = (assign50620_e84189 + assign50620_e84222);
        let assign50620_e84224: f64 = (0.5 * assign50620_e84223);
        let assign50620_e84225: f64 = (assign50620_e84176 + assign50620_e84224);
        let assign50620_e84226: f64 = (-assign50620_e84225);
        (assign50620_e84226, (-(0.5 * ((assign50620_e84179 * ({ let limited_exp_arg = assign50620_e84182; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((locals.var_vdiff_dn3 * locals.var_t1) - (locals.var_vdiff * locals.var_t1_dn3)) / (locals.var_t1 * locals.var_t1)))) + ((((assign50620_e84191 * ({ let limited_exp_arg = assign50620_e84194; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((locals.var_vdiff_dn3 * locals.var_t1) - (locals.var_vdiff * locals.var_t1_dn3)) / (locals.var_t1 * locals.var_t1)))) * assign50620_e84213) + (assign50620_e84201 * (assign50620_e84203 * ({ let limited_exp_arg = assign50620_e84206; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((locals.var_vdiff_dn3 * locals.var_t1) - (locals.var_vdiff * locals.var_t1_dn3)) / (locals.var_t1 * locals.var_t1)))))) / (2.0 * assign50620_e84222))))), (-(0.5 * ((assign50620_e84179 * ({ let limited_exp_arg = assign50620_e84182; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((locals.var_vdiff_dn4 * locals.var_t1) - (locals.var_vdiff * locals.var_t1_dn4)) / (locals.var_t1 * locals.var_t1)))) + ((((assign50620_e84191 * ({ let limited_exp_arg = assign50620_e84194; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((locals.var_vdiff_dn4 * locals.var_t1) - (locals.var_vdiff * locals.var_t1_dn4)) / (locals.var_t1 * locals.var_t1)))) * assign50620_e84213) + (assign50620_e84201 * (assign50620_e84203 * ({ let limited_exp_arg = assign50620_e84206; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((locals.var_vdiff_dn4 * locals.var_t1) - (locals.var_vdiff * locals.var_t1_dn4)) / (locals.var_t1 * locals.var_t1)))))) / (2.0 * assign50620_e84222))))), (-(0.5 * ((assign50620_e84179 * ({ let limited_exp_arg = assign50620_e84182; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((locals.var_vdiff_dn5 * locals.var_t1) - (locals.var_vdiff * locals.var_t1_dn5)) / (locals.var_t1 * locals.var_t1)))) + ((((assign50620_e84191 * ({ let limited_exp_arg = assign50620_e84194; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((locals.var_vdiff_dn5 * locals.var_t1) - (locals.var_vdiff * locals.var_t1_dn5)) / (locals.var_t1 * locals.var_t1)))) * assign50620_e84213) + (assign50620_e84201 * (assign50620_e84203 * ({ let limited_exp_arg = assign50620_e84206; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((locals.var_vdiff_dn5 * locals.var_t1) - (locals.var_vdiff * locals.var_t1_dn5)) / (locals.var_t1 * locals.var_t1)))))) / (2.0 * assign50620_e84222))))), (-(0.5 * ((assign50620_e84179 * ({ let limited_exp_arg = assign50620_e84182; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((locals.var_vdiff_dn6 * locals.var_t1) - (locals.var_vdiff * locals.var_t1_dn6)) / (locals.var_t1 * locals.var_t1)))) + ((((assign50620_e84191 * ({ let limited_exp_arg = assign50620_e84194; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((locals.var_vdiff_dn6 * locals.var_t1) - (locals.var_vdiff * locals.var_t1_dn6)) / (locals.var_t1 * locals.var_t1)))) * assign50620_e84213) + (assign50620_e84201 * (assign50620_e84203 * ({ let limited_exp_arg = assign50620_e84206; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((locals.var_vdiff_dn6 * locals.var_t1) - (locals.var_vdiff * locals.var_t1_dn6)) / (locals.var_t1 * locals.var_t1)))))) / (2.0 * assign50620_e84222))))), (-(0.5 * ((assign50620_e84179 * ({ let limited_exp_arg = assign50620_e84182; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((locals.var_vdiff_dn7 * locals.var_t1) - (locals.var_vdiff * locals.var_t1_dn7)) / (locals.var_t1 * locals.var_t1)))) + ((((assign50620_e84191 * ({ let limited_exp_arg = assign50620_e84194; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((locals.var_vdiff_dn7 * locals.var_t1) - (locals.var_vdiff * locals.var_t1_dn7)) / (locals.var_t1 * locals.var_t1)))) * assign50620_e84213) + (assign50620_e84201 * (assign50620_e84203 * ({ let limited_exp_arg = assign50620_e84206; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((locals.var_vdiff_dn7 * locals.var_t1) - (locals.var_vdiff * locals.var_t1_dn7)) / (locals.var_t1 * locals.var_t1)))))) / (2.0 * assign50620_e84222))))), (-(0.5 * ((assign50620_e84179 * ({ let limited_exp_arg = assign50620_e84182; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((locals.var_vdiff_dn8 * locals.var_t1) - (locals.var_vdiff * locals.var_t1_dn8)) / (locals.var_t1 * locals.var_t1)))) + ((((assign50620_e84191 * ({ let limited_exp_arg = assign50620_e84194; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((locals.var_vdiff_dn8 * locals.var_t1) - (locals.var_vdiff * locals.var_t1_dn8)) / (locals.var_t1 * locals.var_t1)))) * assign50620_e84213) + (assign50620_e84201 * (assign50620_e84203 * ({ let limited_exp_arg = assign50620_e84206; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((locals.var_vdiff_dn8 * locals.var_t1) - (locals.var_vdiff * locals.var_t1_dn8)) / (locals.var_t1 * locals.var_t1)))))) / (2.0 * assign50620_e84222))))), (-(0.5 * ((assign50620_e84179 * ({ let limited_exp_arg = assign50620_e84182; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((locals.var_vdiff_dn9 * locals.var_t1) - (locals.var_vdiff * locals.var_t1_dn9)) / (locals.var_t1 * locals.var_t1)))) + ((((assign50620_e84191 * ({ let limited_exp_arg = assign50620_e84194; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((locals.var_vdiff_dn9 * locals.var_t1) - (locals.var_vdiff * locals.var_t1_dn9)) / (locals.var_t1 * locals.var_t1)))) * assign50620_e84213) + (assign50620_e84201 * (assign50620_e84203 * ({ let limited_exp_arg = assign50620_e84206; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((locals.var_vdiff_dn9 * locals.var_t1) - (locals.var_vdiff * locals.var_t1_dn9)) / (locals.var_t1 * locals.var_t1)))))) / (2.0 * assign50620_e84222))))), (-(0.5 * ((assign50620_e84179 * ({ let limited_exp_arg = assign50620_e84182; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((locals.var_vdiff_dn10 * locals.var_t1) - (locals.var_vdiff * locals.var_t1_dn10)) / (locals.var_t1 * locals.var_t1)))) + ((((assign50620_e84191 * ({ let limited_exp_arg = assign50620_e84194; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((locals.var_vdiff_dn10 * locals.var_t1) - (locals.var_vdiff * locals.var_t1_dn10)) / (locals.var_t1 * locals.var_t1)))) * assign50620_e84213) + (assign50620_e84201 * (assign50620_e84203 * ({ let limited_exp_arg = assign50620_e84206; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((locals.var_vdiff_dn10 * locals.var_t1) - (locals.var_vdiff * locals.var_t1_dn10)) / (locals.var_t1 * locals.var_t1)))))) / (2.0 * assign50620_e84222))))), (-(0.5 * ((assign50620_e84179 * ({ let limited_exp_arg = assign50620_e84182; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((locals.var_vdiff_dn11 * locals.var_t1) - (locals.var_vdiff * locals.var_t1_dn11)) / (locals.var_t1 * locals.var_t1)))) + ((((assign50620_e84191 * ({ let limited_exp_arg = assign50620_e84194; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((locals.var_vdiff_dn11 * locals.var_t1) - (locals.var_vdiff * locals.var_t1_dn11)) / (locals.var_t1 * locals.var_t1)))) * assign50620_e84213) + (assign50620_e84201 * (assign50620_e84203 * ({ let limited_exp_arg = assign50620_e84206; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((locals.var_vdiff_dn11 * locals.var_t1) - (locals.var_vdiff * locals.var_t1_dn11)) / (locals.var_t1 * locals.var_t1)))))) / (2.0 * assign50620_e84222))))),)
    } else {
        (locals.var_ratio, locals.var_ratio_dn3, locals.var_ratio_dn4, locals.var_ratio_dn5, locals.var_ratio_dn6, locals.var_ratio_dn7, locals.var_ratio_dn8, locals.var_ratio_dn9, locals.var_ratio_dn10, locals.var_ratio_dn11,)
    }
};
        locals.var_ratio = assign50620_e84228;
        locals.var_ratio_dn3 = assign50620_e84228_d_n3;
        locals.var_ratio_dn4 = assign50620_e84228_d_n4;
        locals.var_ratio_dn5 = assign50620_e84228_d_n5;
        locals.var_ratio_dn6 = assign50620_e84228_d_n6;
        locals.var_ratio_dn7 = assign50620_e84228_d_n7;
        locals.var_ratio_dn8 = assign50620_e84228_d_n8;
        locals.var_ratio_dn9 = assign50620_e84228_d_n9;
        locals.var_ratio_dn10 = assign50620_e84228_d_n10;
        locals.var_ratio_dn11 = assign50620_e84228_d_n11;

        let (assign50630_e84249, assign50630_e84249_d_n3, assign50630_e84249_d_n4, assign50630_e84249_d_n5, assign50630_e84249_d_n6, assign50630_e84249_d_n7, assign50630_e84249_d_n8, assign50630_e84249_d_n9, assign50630_e84249_d_n10, assign50630_e84249_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 != 0.0)) && (locals.var_guard772 == 0.0)) {
        let assign50630_e84243: f64 = (locals.var_fbjtii_i * locals.var_sigvds);
        let assign50630_e84245: f64 = (assign50630_e84243 * locals.var_ic);
        let assign50630_e84246: f64 = (locals.var_ids + assign50630_e84245);
        let assign50630_e84247: f64 = (locals.var_ratio * assign50630_e84246);
        (assign50630_e84247, ((locals.var_ratio_dn3 * assign50630_e84246) + (locals.var_ratio * (locals.var_ids_dn3 + (assign50630_e84243 * locals.var_ic_dn3)))), ((locals.var_ratio_dn4 * assign50630_e84246) + (locals.var_ratio * (locals.var_ids_dn4 + (assign50630_e84243 * locals.var_ic_dn4)))), ((locals.var_ratio_dn5 * assign50630_e84246) + (locals.var_ratio * (locals.var_ids_dn5 + (assign50630_e84243 * locals.var_ic_dn5)))), ((locals.var_ratio_dn6 * assign50630_e84246) + (locals.var_ratio * (locals.var_ids_dn6 + (assign50630_e84243 * locals.var_ic_dn6)))), ((locals.var_ratio_dn7 * assign50630_e84246) + (locals.var_ratio * (locals.var_ids_dn7 + (assign50630_e84243 * locals.var_ic_dn7)))), ((locals.var_ratio_dn8 * assign50630_e84246) + (locals.var_ratio * (locals.var_ids_dn8 + (assign50630_e84243 * locals.var_ic_dn8)))), ((locals.var_ratio_dn9 * assign50630_e84246) + (locals.var_ratio * (locals.var_ids_dn9 + (assign50630_e84243 * locals.var_ic_dn9)))), ((locals.var_ratio_dn10 * assign50630_e84246) + (locals.var_ratio * (locals.var_ids_dn10 + (assign50630_e84243 * locals.var_ic_dn10)))), ((locals.var_ratio_dn11 * assign50630_e84246) + (locals.var_ratio * (locals.var_ids_dn11 + (assign50630_e84243 * locals.var_ic_dn11)))),)
    } else {
        (locals.var_iii, locals.var_iii_dn3, locals.var_iii_dn4, locals.var_iii_dn5, locals.var_iii_dn6, locals.var_iii_dn7, locals.var_iii_dn8, locals.var_iii_dn9, locals.var_iii_dn10, locals.var_iii_dn11,)
    }
};
        locals.var_iii = assign50630_e84249;
        locals.var_iii_dn3 = assign50630_e84249_d_n3;
        locals.var_iii_dn4 = assign50630_e84249_d_n4;
        locals.var_iii_dn5 = assign50630_e84249_d_n5;
        locals.var_iii_dn6 = assign50630_e84249_d_n6;
        locals.var_iii_dn7 = assign50630_e84249_d_n7;
        locals.var_iii_dn8 = assign50630_e84249_d_n8;
        locals.var_iii_dn9 = assign50630_e84249_d_n9;
        locals.var_iii_dn10 = assign50630_e84249_d_n10;
        locals.var_iii_dn11 = assign50630_e84249_d_n11;

        let assign50640_e84264: f64 = if ((locals.var_alpha0_i <= 0.0) || (((locals.var_beta2_i == 0.0) && (locals.var_beta1_i == 0.0)) && (locals.var_beta0_t == 0.0))) { 1.0 } else { 0.0 };
        locals.var_guard773 = assign50640_e84264;

        let (assign50650_e84277, assign50650_e84277_d_n3, assign50650_e84277_d_n4, assign50650_e84277_d_n5, assign50650_e84277_d_n6, assign50650_e84277_d_n7, assign50650_e84277_d_n8, assign50650_e84277_d_n9, assign50650_e84277_d_n10, assign50650_e84277_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 == 0.0)) && (locals.var_guard773 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_iii, locals.var_iii_dn3, locals.var_iii_dn4, locals.var_iii_dn5, locals.var_iii_dn6, locals.var_iii_dn7, locals.var_iii_dn8, locals.var_iii_dn9, locals.var_iii_dn10, locals.var_iii_dn11,)
    }
};
        locals.var_iii = assign50650_e84277;
        locals.var_iii_dn3 = assign50650_e84277_d_n3;
        locals.var_iii_dn4 = assign50650_e84277_d_n4;
        locals.var_iii_dn5 = assign50650_e84277_d_n5;
        locals.var_iii_dn6 = assign50650_e84277_d_n6;
        locals.var_iii_dn7 = assign50650_e84277_d_n7;
        locals.var_iii_dn8 = assign50650_e84277_d_n8;
        locals.var_iii_dn9 = assign50650_e84277_d_n9;
        locals.var_iii_dn10 = assign50650_e84277_d_n10;
        locals.var_iii_dn11 = assign50650_e84277_d_n11;

        let (assign50660_e84303, assign50660_e84303_d_n4, assign50660_e84303_d_n5,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 == 0.0)) && (locals.var_guard773 == 0.0)) {
        let assign50660_e84294: f64 = (locals.var_tratio - 1.0);
        let assign50660_e84295: f64 = (p.p600 * assign50660_e84294);
        let assign50660_e84296: f64 = (1.0 + assign50660_e84295);
        let assign50660_e84297: f64 = (locals.var_vdsatii0_i * assign50660_e84296);
        let assign50660_e84300: f64 = (locals.var_lii_i / locals.var_leff);
        let assign50660_e84301: f64 = (assign50660_e84297 - assign50660_e84300);
        (assign50660_e84301, (locals.var_vdsatii0_i * (p.p600 * locals.var_tratio_dn4)), (locals.var_vdsatii0_i * (p.p600 * locals.var_tratio_dn5)),)
    } else {
        (locals.var_vdsatii0, locals.var_vdsatii0_dn4, locals.var_vdsatii0_dn5,)
    }
};
        locals.var_vdsatii0 = assign50660_e84303;
        locals.var_vdsatii0_dn4 = assign50660_e84303_d_n4;
        locals.var_vdsatii0_dn5 = assign50660_e84303_d_n5;

        let (assign50670_e84319, assign50670_e84319_d_n3, assign50670_e84319_d_n4, assign50670_e84319_d_n5, assign50670_e84319_d_n6, assign50670_e84319_d_n7, assign50670_e84319_d_n8, assign50670_e84319_d_n9, assign50670_e84319_d_n10, assign50670_e84319_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 == 0.0)) && (locals.var_guard773 == 0.0)) {
        let assign50670_e84317: f64 = (locals.var_esatii_i * locals.var_leff);
        (assign50670_e84317, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign50670_e84319;
        locals.var_t0_dn3 = assign50670_e84319_d_n3;
        locals.var_t0_dn4 = assign50670_e84319_d_n4;
        locals.var_t0_dn5 = assign50670_e84319_d_n5;
        locals.var_t0_dn6 = assign50670_e84319_d_n6;
        locals.var_t0_dn7 = assign50670_e84319_d_n7;
        locals.var_t0_dn8 = assign50670_e84319_d_n8;
        locals.var_t0_dn9 = assign50670_e84319_d_n9;
        locals.var_t0_dn10 = assign50670_e84319_d_n10;
        locals.var_t0_dn11 = assign50670_e84319_d_n11;

        let (assign50680_e84339, assign50680_e84339_d_n3, assign50680_e84339_d_n4, assign50680_e84339_d_n5, assign50680_e84339_d_n6, assign50680_e84339_d_n7, assign50680_e84339_d_n8, assign50680_e84339_d_n9, assign50680_e84339_d_n10, assign50680_e84339_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 == 0.0)) && (locals.var_guard773 == 0.0)) {
        let assign50680_e84333: f64 = (locals.var_sii0_i * locals.var_t0);
        let assign50680_e84336: f64 = (1.0 + locals.var_t0);
        let assign50680_e84337: f64 = (assign50680_e84333 / assign50680_e84336);
        (assign50680_e84337, ((((locals.var_sii0_i * locals.var_t0_dn3) * assign50680_e84336) - (assign50680_e84333 * locals.var_t0_dn3)) / (assign50680_e84336 * assign50680_e84336)), ((((locals.var_sii0_i * locals.var_t0_dn4) * assign50680_e84336) - (assign50680_e84333 * locals.var_t0_dn4)) / (assign50680_e84336 * assign50680_e84336)), ((((locals.var_sii0_i * locals.var_t0_dn5) * assign50680_e84336) - (assign50680_e84333 * locals.var_t0_dn5)) / (assign50680_e84336 * assign50680_e84336)), ((((locals.var_sii0_i * locals.var_t0_dn6) * assign50680_e84336) - (assign50680_e84333 * locals.var_t0_dn6)) / (assign50680_e84336 * assign50680_e84336)), ((((locals.var_sii0_i * locals.var_t0_dn7) * assign50680_e84336) - (assign50680_e84333 * locals.var_t0_dn7)) / (assign50680_e84336 * assign50680_e84336)), ((((locals.var_sii0_i * locals.var_t0_dn8) * assign50680_e84336) - (assign50680_e84333 * locals.var_t0_dn8)) / (assign50680_e84336 * assign50680_e84336)), ((((locals.var_sii0_i * locals.var_t0_dn9) * assign50680_e84336) - (assign50680_e84333 * locals.var_t0_dn9)) / (assign50680_e84336 * assign50680_e84336)), ((((locals.var_sii0_i * locals.var_t0_dn10) * assign50680_e84336) - (assign50680_e84333 * locals.var_t0_dn10)) / (assign50680_e84336 * assign50680_e84336)), ((((locals.var_sii0_i * locals.var_t0_dn11) * assign50680_e84336) - (assign50680_e84333 * locals.var_t0_dn11)) / (assign50680_e84336 * assign50680_e84336)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign50680_e84339;
        locals.var_t1_dn3 = assign50680_e84339_d_n3;
        locals.var_t1_dn4 = assign50680_e84339_d_n4;
        locals.var_t1_dn5 = assign50680_e84339_d_n5;
        locals.var_t1_dn6 = assign50680_e84339_d_n6;
        locals.var_t1_dn7 = assign50680_e84339_d_n7;
        locals.var_t1_dn8 = assign50680_e84339_d_n8;
        locals.var_t1_dn9 = assign50680_e84339_d_n9;
        locals.var_t1_dn10 = assign50680_e84339_d_n10;
        locals.var_t1_dn11 = assign50680_e84339_d_n11;

        let (assign50690_e84382, assign50690_e84382_d_n3, assign50690_e84382_d_n4, assign50690_e84382_d_n5, assign50690_e84382_d_n6, assign50690_e84382_d_n7, assign50690_e84382_d_n8, assign50690_e84382_d_n9, assign50690_e84382_d_n10, assign50690_e84382_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 == 0.0)) && (locals.var_guard773 == 0.0)) {
        let assign50690_e84356: f64 = (locals.var_sii1_i * locals.var_vgsfb);
        let assign50690_e84358: f64 = (assign50690_e84356 * locals.var_nvt);
        let assign50690_e84361: f64 = (locals.var_sii1_i * locals.var_vgsfb);
        let assign50690_e84363: f64 = (assign50690_e84361 * locals.var_nvt);
        let assign50690_e84366: f64 = (locals.var_sii1_i * locals.var_vgsfb);
        let assign50690_e84368: f64 = (assign50690_e84366 * locals.var_nvt);
        let assign50690_e84369: f64 = (assign50690_e84363 * assign50690_e84368);
        let assign50690_e84372: f64 = (4.0 * p.p643);
        let assign50690_e84374: f64 = (assign50690_e84372 * p.p643);
        let assign50690_e84375: f64 = (assign50690_e84369 + assign50690_e84374);
        let assign50690_e84376: f64 = (assign50690_e84375).sqrt();
        let assign50690_e84377: f64 = (assign50690_e84358 + assign50690_e84376);
        let assign50690_e84378: f64 = (0.5 * assign50690_e84377);
        let assign50690_e84379: f64 = (1.0 + assign50690_e84378);
        let assign50690_e84380: f64 = (1.0 / assign50690_e84379);
        (assign50690_e84380, (-((0.5 * ((((locals.var_sii1_i * locals.var_vgsfb_dn3) * locals.var_nvt) + (assign50690_e84356 * locals.var_nvt_dn3)) + ((((((locals.var_sii1_i * locals.var_vgsfb_dn3) * locals.var_nvt) + (assign50690_e84361 * locals.var_nvt_dn3)) * assign50690_e84368) + (assign50690_e84363 * (((locals.var_sii1_i * locals.var_vgsfb_dn3) * locals.var_nvt) + (assign50690_e84366 * locals.var_nvt_dn3)))) / (2.0 * assign50690_e84376)))) / (assign50690_e84379 * assign50690_e84379))), (-((0.5 * ((((locals.var_sii1_i * locals.var_vgsfb_dn4) * locals.var_nvt) + (assign50690_e84356 * locals.var_nvt_dn4)) + ((((((locals.var_sii1_i * locals.var_vgsfb_dn4) * locals.var_nvt) + (assign50690_e84361 * locals.var_nvt_dn4)) * assign50690_e84368) + (assign50690_e84363 * (((locals.var_sii1_i * locals.var_vgsfb_dn4) * locals.var_nvt) + (assign50690_e84366 * locals.var_nvt_dn4)))) / (2.0 * assign50690_e84376)))) / (assign50690_e84379 * assign50690_e84379))), (-((0.5 * ((((locals.var_sii1_i * locals.var_vgsfb_dn5) * locals.var_nvt) + (assign50690_e84356 * locals.var_nvt_dn5)) + ((((((locals.var_sii1_i * locals.var_vgsfb_dn5) * locals.var_nvt) + (assign50690_e84361 * locals.var_nvt_dn5)) * assign50690_e84368) + (assign50690_e84363 * (((locals.var_sii1_i * locals.var_vgsfb_dn5) * locals.var_nvt) + (assign50690_e84366 * locals.var_nvt_dn5)))) / (2.0 * assign50690_e84376)))) / (assign50690_e84379 * assign50690_e84379))), (-((0.5 * ((((locals.var_sii1_i * locals.var_vgsfb_dn6) * locals.var_nvt) + (assign50690_e84356 * locals.var_nvt_dn6)) + ((((((locals.var_sii1_i * locals.var_vgsfb_dn6) * locals.var_nvt) + (assign50690_e84361 * locals.var_nvt_dn6)) * assign50690_e84368) + (assign50690_e84363 * (((locals.var_sii1_i * locals.var_vgsfb_dn6) * locals.var_nvt) + (assign50690_e84366 * locals.var_nvt_dn6)))) / (2.0 * assign50690_e84376)))) / (assign50690_e84379 * assign50690_e84379))), (-((0.5 * ((((locals.var_sii1_i * locals.var_vgsfb_dn7) * locals.var_nvt) + (assign50690_e84356 * locals.var_nvt_dn7)) + ((((((locals.var_sii1_i * locals.var_vgsfb_dn7) * locals.var_nvt) + (assign50690_e84361 * locals.var_nvt_dn7)) * assign50690_e84368) + (assign50690_e84363 * (((locals.var_sii1_i * locals.var_vgsfb_dn7) * locals.var_nvt) + (assign50690_e84366 * locals.var_nvt_dn7)))) / (2.0 * assign50690_e84376)))) / (assign50690_e84379 * assign50690_e84379))), (-((0.5 * ((((locals.var_sii1_i * locals.var_vgsfb_dn8) * locals.var_nvt) + (assign50690_e84356 * locals.var_nvt_dn8)) + ((((((locals.var_sii1_i * locals.var_vgsfb_dn8) * locals.var_nvt) + (assign50690_e84361 * locals.var_nvt_dn8)) * assign50690_e84368) + (assign50690_e84363 * (((locals.var_sii1_i * locals.var_vgsfb_dn8) * locals.var_nvt) + (assign50690_e84366 * locals.var_nvt_dn8)))) / (2.0 * assign50690_e84376)))) / (assign50690_e84379 * assign50690_e84379))), (-((0.5 * ((((locals.var_sii1_i * locals.var_vgsfb_dn9) * locals.var_nvt) + (assign50690_e84356 * locals.var_nvt_dn9)) + ((((((locals.var_sii1_i * locals.var_vgsfb_dn9) * locals.var_nvt) + (assign50690_e84361 * locals.var_nvt_dn9)) * assign50690_e84368) + (assign50690_e84363 * (((locals.var_sii1_i * locals.var_vgsfb_dn9) * locals.var_nvt) + (assign50690_e84366 * locals.var_nvt_dn9)))) / (2.0 * assign50690_e84376)))) / (assign50690_e84379 * assign50690_e84379))), (-((0.5 * ((((locals.var_sii1_i * locals.var_vgsfb_dn10) * locals.var_nvt) + (assign50690_e84356 * locals.var_nvt_dn10)) + ((((((locals.var_sii1_i * locals.var_vgsfb_dn10) * locals.var_nvt) + (assign50690_e84361 * locals.var_nvt_dn10)) * assign50690_e84368) + (assign50690_e84363 * (((locals.var_sii1_i * locals.var_vgsfb_dn10) * locals.var_nvt) + (assign50690_e84366 * locals.var_nvt_dn10)))) / (2.0 * assign50690_e84376)))) / (assign50690_e84379 * assign50690_e84379))), (-((0.5 * ((((locals.var_sii1_i * locals.var_vgsfb_dn11) * locals.var_nvt) + (assign50690_e84356 * locals.var_nvt_dn11)) + ((((((locals.var_sii1_i * locals.var_vgsfb_dn11) * locals.var_nvt) + (assign50690_e84361 * locals.var_nvt_dn11)) * assign50690_e84368) + (assign50690_e84363 * (((locals.var_sii1_i * locals.var_vgsfb_dn11) * locals.var_nvt) + (assign50690_e84366 * locals.var_nvt_dn11)))) / (2.0 * assign50690_e84376)))) / (assign50690_e84379 * assign50690_e84379))),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign50690_e84382;
        locals.var_t0_dn3 = assign50690_e84382_d_n3;
        locals.var_t0_dn4 = assign50690_e84382_d_n4;
        locals.var_t0_dn5 = assign50690_e84382_d_n5;
        locals.var_t0_dn6 = assign50690_e84382_d_n6;
        locals.var_t0_dn7 = assign50690_e84382_d_n7;
        locals.var_t0_dn8 = assign50690_e84382_d_n8;
        locals.var_t0_dn9 = assign50690_e84382_d_n9;
        locals.var_t0_dn10 = assign50690_e84382_d_n10;
        locals.var_t0_dn11 = assign50690_e84382_d_n11;

        let (assign50700_e84398, assign50700_e84398_d_n3, assign50700_e84398_d_n4, assign50700_e84398_d_n5, assign50700_e84398_d_n6, assign50700_e84398_d_n7, assign50700_e84398_d_n8, assign50700_e84398_d_n9, assign50700_e84398_d_n10, assign50700_e84398_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 == 0.0)) && (locals.var_guard773 == 0.0)) {
        let assign50700_e84396: f64 = (locals.var_t0 + locals.var_sii2_i);
        (assign50700_e84396, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign50700_e84398;
        locals.var_t3_dn3 = assign50700_e84398_d_n3;
        locals.var_t3_dn4 = assign50700_e84398_d_n4;
        locals.var_t3_dn5 = assign50700_e84398_d_n5;
        locals.var_t3_dn6 = assign50700_e84398_d_n6;
        locals.var_t3_dn7 = assign50700_e84398_d_n7;
        locals.var_t3_dn8 = assign50700_e84398_d_n8;
        locals.var_t3_dn9 = assign50700_e84398_d_n9;
        locals.var_t3_dn10 = assign50700_e84398_d_n10;
        locals.var_t3_dn11 = assign50700_e84398_d_n11;

    }

    pub(super) fn stamp_transient_block_172(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign50710_e84437, assign50710_e84437_d_n3, assign50710_e84437_d_n4, assign50710_e84437_d_n5, assign50710_e84437_d_n6, assign50710_e84437_d_n7, assign50710_e84437_d_n8, assign50710_e84437_d_n9, assign50710_e84437_d_n10, assign50710_e84437_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 == 0.0)) && (locals.var_guard773 == 0.0)) {
        let assign50710_e84413: f64 = (locals.var_vgsfb * locals.var_nvt);
        let assign50710_e84415: f64 = (assign50710_e84413 * locals.var_t3);
        let assign50710_e84418: f64 = (locals.var_vgsfb * locals.var_nvt);
        let assign50710_e84420: f64 = (assign50710_e84418 * locals.var_t3);
        let assign50710_e84423: f64 = (locals.var_vgsfb * locals.var_nvt);
        let assign50710_e84425: f64 = (assign50710_e84423 * locals.var_t3);
        let assign50710_e84426: f64 = (assign50710_e84420 * assign50710_e84425);
        let assign50710_e84429: f64 = (4.0 * p.p644);
        let assign50710_e84431: f64 = (assign50710_e84429 * p.p644);
        let assign50710_e84432: f64 = (assign50710_e84426 + assign50710_e84431);
        let assign50710_e84433: f64 = (assign50710_e84432).sqrt();
        let assign50710_e84434: f64 = (assign50710_e84415 + assign50710_e84433);
        let assign50710_e84435: f64 = (0.5 * assign50710_e84434);
        (assign50710_e84435, (0.5 * (((((locals.var_vgsfb_dn3 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn3)) * locals.var_t3) + (assign50710_e84413 * locals.var_t3_dn3)) + (((((((locals.var_vgsfb_dn3 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn3)) * locals.var_t3) + (assign50710_e84418 * locals.var_t3_dn3)) * assign50710_e84425) + (assign50710_e84420 * ((((locals.var_vgsfb_dn3 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn3)) * locals.var_t3) + (assign50710_e84423 * locals.var_t3_dn3)))) / (2.0 * assign50710_e84433)))), (0.5 * (((((locals.var_vgsfb_dn4 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn4)) * locals.var_t3) + (assign50710_e84413 * locals.var_t3_dn4)) + (((((((locals.var_vgsfb_dn4 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn4)) * locals.var_t3) + (assign50710_e84418 * locals.var_t3_dn4)) * assign50710_e84425) + (assign50710_e84420 * ((((locals.var_vgsfb_dn4 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn4)) * locals.var_t3) + (assign50710_e84423 * locals.var_t3_dn4)))) / (2.0 * assign50710_e84433)))), (0.5 * (((((locals.var_vgsfb_dn5 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn5)) * locals.var_t3) + (assign50710_e84413 * locals.var_t3_dn5)) + (((((((locals.var_vgsfb_dn5 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn5)) * locals.var_t3) + (assign50710_e84418 * locals.var_t3_dn5)) * assign50710_e84425) + (assign50710_e84420 * ((((locals.var_vgsfb_dn5 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn5)) * locals.var_t3) + (assign50710_e84423 * locals.var_t3_dn5)))) / (2.0 * assign50710_e84433)))), (0.5 * (((((locals.var_vgsfb_dn6 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn6)) * locals.var_t3) + (assign50710_e84413 * locals.var_t3_dn6)) + (((((((locals.var_vgsfb_dn6 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn6)) * locals.var_t3) + (assign50710_e84418 * locals.var_t3_dn6)) * assign50710_e84425) + (assign50710_e84420 * ((((locals.var_vgsfb_dn6 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn6)) * locals.var_t3) + (assign50710_e84423 * locals.var_t3_dn6)))) / (2.0 * assign50710_e84433)))), (0.5 * (((((locals.var_vgsfb_dn7 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn7)) * locals.var_t3) + (assign50710_e84413 * locals.var_t3_dn7)) + (((((((locals.var_vgsfb_dn7 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn7)) * locals.var_t3) + (assign50710_e84418 * locals.var_t3_dn7)) * assign50710_e84425) + (assign50710_e84420 * ((((locals.var_vgsfb_dn7 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn7)) * locals.var_t3) + (assign50710_e84423 * locals.var_t3_dn7)))) / (2.0 * assign50710_e84433)))), (0.5 * (((((locals.var_vgsfb_dn8 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn8)) * locals.var_t3) + (assign50710_e84413 * locals.var_t3_dn8)) + (((((((locals.var_vgsfb_dn8 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn8)) * locals.var_t3) + (assign50710_e84418 * locals.var_t3_dn8)) * assign50710_e84425) + (assign50710_e84420 * ((((locals.var_vgsfb_dn8 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn8)) * locals.var_t3) + (assign50710_e84423 * locals.var_t3_dn8)))) / (2.0 * assign50710_e84433)))), (0.5 * (((((locals.var_vgsfb_dn9 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn9)) * locals.var_t3) + (assign50710_e84413 * locals.var_t3_dn9)) + (((((((locals.var_vgsfb_dn9 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn9)) * locals.var_t3) + (assign50710_e84418 * locals.var_t3_dn9)) * assign50710_e84425) + (assign50710_e84420 * ((((locals.var_vgsfb_dn9 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn9)) * locals.var_t3) + (assign50710_e84423 * locals.var_t3_dn9)))) / (2.0 * assign50710_e84433)))), (0.5 * (((((locals.var_vgsfb_dn10 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn10)) * locals.var_t3) + (assign50710_e84413 * locals.var_t3_dn10)) + (((((((locals.var_vgsfb_dn10 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn10)) * locals.var_t3) + (assign50710_e84418 * locals.var_t3_dn10)) * assign50710_e84425) + (assign50710_e84420 * ((((locals.var_vgsfb_dn10 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn10)) * locals.var_t3) + (assign50710_e84423 * locals.var_t3_dn10)))) / (2.0 * assign50710_e84433)))), (0.5 * (((((locals.var_vgsfb_dn11 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn11)) * locals.var_t3) + (assign50710_e84413 * locals.var_t3_dn11)) + (((((((locals.var_vgsfb_dn11 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn11)) * locals.var_t3) + (assign50710_e84418 * locals.var_t3_dn11)) * assign50710_e84425) + (assign50710_e84420 * ((((locals.var_vgsfb_dn11 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn11)) * locals.var_t3) + (assign50710_e84423 * locals.var_t3_dn11)))) / (2.0 * assign50710_e84433)))),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign50710_e84437;
        locals.var_t2_dn3 = assign50710_e84437_d_n3;
        locals.var_t2_dn4 = assign50710_e84437_d_n4;
        locals.var_t2_dn5 = assign50710_e84437_d_n5;
        locals.var_t2_dn6 = assign50710_e84437_d_n6;
        locals.var_t2_dn7 = assign50710_e84437_d_n7;
        locals.var_t2_dn8 = assign50710_e84437_d_n8;
        locals.var_t2_dn9 = assign50710_e84437_d_n9;
        locals.var_t2_dn10 = assign50710_e84437_d_n10;
        locals.var_t2_dn11 = assign50710_e84437_d_n11;

        let (assign50720_e84457, assign50720_e84457_d_n3, assign50720_e84457_d_n4, assign50720_e84457_d_n5, assign50720_e84457_d_n6, assign50720_e84457_d_n7, assign50720_e84457_d_n8, assign50720_e84457_d_n9, assign50720_e84457_d_n10, assign50720_e84457_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 == 0.0)) && (locals.var_guard773 == 0.0)) {
        let assign50720_e84453: f64 = (locals.var_siid_i * locals.var_vdsx);
        let assign50720_e84454: f64 = (1.0 + assign50720_e84453);
        let assign50720_e84455: f64 = (1.0 / assign50720_e84454);
        (assign50720_e84455, (-((locals.var_siid_i * locals.var_vdsx_dn3) / (assign50720_e84454 * assign50720_e84454))), (-((locals.var_siid_i * locals.var_vdsx_dn4) / (assign50720_e84454 * assign50720_e84454))), (-((locals.var_siid_i * locals.var_vdsx_dn5) / (assign50720_e84454 * assign50720_e84454))), (-((locals.var_siid_i * locals.var_vdsx_dn6) / (assign50720_e84454 * assign50720_e84454))), (-((locals.var_siid_i * locals.var_vdsx_dn7) / (assign50720_e84454 * assign50720_e84454))), (-((locals.var_siid_i * locals.var_vdsx_dn8) / (assign50720_e84454 * assign50720_e84454))), (-((locals.var_siid_i * locals.var_vdsx_dn9) / (assign50720_e84454 * assign50720_e84454))), (-((locals.var_siid_i * locals.var_vdsx_dn10) / (assign50720_e84454 * assign50720_e84454))), (-((locals.var_siid_i * locals.var_vdsx_dn11) / (assign50720_e84454 * assign50720_e84454))),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign50720_e84457;
        locals.var_t3_dn3 = assign50720_e84457_d_n3;
        locals.var_t3_dn4 = assign50720_e84457_d_n4;
        locals.var_t3_dn5 = assign50720_e84457_d_n5;
        locals.var_t3_dn6 = assign50720_e84457_d_n6;
        locals.var_t3_dn7 = assign50720_e84457_d_n7;
        locals.var_t3_dn8 = assign50720_e84457_d_n8;
        locals.var_t3_dn9 = assign50720_e84457_d_n9;
        locals.var_t3_dn10 = assign50720_e84457_d_n10;
        locals.var_t3_dn11 = assign50720_e84457_d_n11;

        let (assign50730_e84475, assign50730_e84475_d_n3, assign50730_e84475_d_n4, assign50730_e84475_d_n5, assign50730_e84475_d_n6, assign50730_e84475_d_n7, assign50730_e84475_d_n8, assign50730_e84475_d_n9, assign50730_e84475_d_n10, assign50730_e84475_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 == 0.0)) && (locals.var_guard773 == 0.0)) {
        let assign50730_e84471: f64 = (locals.var_t1 * locals.var_t2);
        let assign50730_e84473: f64 = (assign50730_e84471 * locals.var_t3);
        (assign50730_e84473, ((((locals.var_t1_dn3 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn3)) * locals.var_t3) + (assign50730_e84471 * locals.var_t3_dn3)), ((((locals.var_t1_dn4 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn4)) * locals.var_t3) + (assign50730_e84471 * locals.var_t3_dn4)), ((((locals.var_t1_dn5 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn5)) * locals.var_t3) + (assign50730_e84471 * locals.var_t3_dn5)), ((((locals.var_t1_dn6 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn6)) * locals.var_t3) + (assign50730_e84471 * locals.var_t3_dn6)), ((((locals.var_t1_dn7 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn7)) * locals.var_t3) + (assign50730_e84471 * locals.var_t3_dn7)), ((((locals.var_t1_dn8 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn8)) * locals.var_t3) + (assign50730_e84471 * locals.var_t3_dn8)), ((((locals.var_t1_dn9 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn9)) * locals.var_t3) + (assign50730_e84471 * locals.var_t3_dn9)), ((((locals.var_t1_dn10 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn10)) * locals.var_t3) + (assign50730_e84471 * locals.var_t3_dn10)), ((((locals.var_t1_dn11 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn11)) * locals.var_t3) + (assign50730_e84471 * locals.var_t3_dn11)),)
    } else {
        (locals.var_vgsstep, locals.var_vgsstep_dn3, locals.var_vgsstep_dn4, locals.var_vgsstep_dn5, locals.var_vgsstep_dn6, locals.var_vgsstep_dn7, locals.var_vgsstep_dn8, locals.var_vgsstep_dn9, locals.var_vgsstep_dn10, locals.var_vgsstep_dn11,)
    }
};
        locals.var_vgsstep = assign50730_e84475;
        locals.var_vgsstep_dn3 = assign50730_e84475_d_n3;
        locals.var_vgsstep_dn4 = assign50730_e84475_d_n4;
        locals.var_vgsstep_dn5 = assign50730_e84475_d_n5;
        locals.var_vgsstep_dn6 = assign50730_e84475_d_n6;
        locals.var_vgsstep_dn7 = assign50730_e84475_d_n7;
        locals.var_vgsstep_dn8 = assign50730_e84475_d_n8;
        locals.var_vgsstep_dn9 = assign50730_e84475_d_n9;
        locals.var_vgsstep_dn10 = assign50730_e84475_d_n10;
        locals.var_vgsstep_dn11 = assign50730_e84475_d_n11;

        let (assign50740_e84491, assign50740_e84491_d_n3, assign50740_e84491_d_n4, assign50740_e84491_d_n5, assign50740_e84491_d_n6, assign50740_e84491_d_n7, assign50740_e84491_d_n8, assign50740_e84491_d_n9, assign50740_e84491_d_n10, assign50740_e84491_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 == 0.0)) && (locals.var_guard773 == 0.0)) {
        let assign50740_e84489: f64 = (locals.var_vdsatii0 + locals.var_vgsstep);
        (assign50740_e84489, locals.var_vgsstep_dn3, (locals.var_vdsatii0_dn4 + locals.var_vgsstep_dn4), (locals.var_vdsatii0_dn5 + locals.var_vgsstep_dn5), locals.var_vgsstep_dn6, locals.var_vgsstep_dn7, locals.var_vgsstep_dn8, locals.var_vgsstep_dn9, locals.var_vgsstep_dn10, locals.var_vgsstep_dn11,)
    } else {
        (locals.var_vdsatii, locals.var_vdsatii_dn3, locals.var_vdsatii_dn4, locals.var_vdsatii_dn5, locals.var_vdsatii_dn6, locals.var_vdsatii_dn7, locals.var_vdsatii_dn8, locals.var_vdsatii_dn9, locals.var_vdsatii_dn10, locals.var_vdsatii_dn11,)
    }
};
        locals.var_vdsatii = assign50740_e84491;
        locals.var_vdsatii_dn3 = assign50740_e84491_d_n3;
        locals.var_vdsatii_dn4 = assign50740_e84491_d_n4;
        locals.var_vdsatii_dn5 = assign50740_e84491_d_n5;
        locals.var_vdsatii_dn6 = assign50740_e84491_d_n6;
        locals.var_vdsatii_dn7 = assign50740_e84491_d_n7;
        locals.var_vdsatii_dn8 = assign50740_e84491_d_n8;
        locals.var_vdsatii_dn9 = assign50740_e84491_d_n9;
        locals.var_vdsatii_dn10 = assign50740_e84491_d_n10;
        locals.var_vdsatii_dn11 = assign50740_e84491_d_n11;

        let (assign50750_e84507, assign50750_e84507_d_n3, assign50750_e84507_d_n4, assign50750_e84507_d_n5, assign50750_e84507_d_n6, assign50750_e84507_d_n7, assign50750_e84507_d_n8, assign50750_e84507_d_n9, assign50750_e84507_d_n10, assign50750_e84507_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 == 0.0)) && (locals.var_guard773 == 0.0)) {
        let assign50750_e84505: f64 = (locals.var_vdsx - locals.var_vdsatii);
        (assign50750_e84505, (locals.var_vdsx_dn3 - locals.var_vdsatii_dn3), (locals.var_vdsx_dn4 - locals.var_vdsatii_dn4), (locals.var_vdsx_dn5 - locals.var_vdsatii_dn5), (locals.var_vdsx_dn6 - locals.var_vdsatii_dn6), (locals.var_vdsx_dn7 - locals.var_vdsatii_dn7), (locals.var_vdsx_dn8 - locals.var_vdsatii_dn8), (locals.var_vdsx_dn9 - locals.var_vdsatii_dn9), (locals.var_vdsx_dn10 - locals.var_vdsatii_dn10), (locals.var_vdsx_dn11 - locals.var_vdsatii_dn11),)
    } else {
        (locals.var_vdiff, locals.var_vdiff_dn3, locals.var_vdiff_dn4, locals.var_vdiff_dn5, locals.var_vdiff_dn6, locals.var_vdiff_dn7, locals.var_vdiff_dn8, locals.var_vdiff_dn9, locals.var_vdiff_dn10, locals.var_vdiff_dn11,)
    }
};
        locals.var_vdiff = assign50750_e84507;
        locals.var_vdiff_dn3 = assign50750_e84507_d_n3;
        locals.var_vdiff_dn4 = assign50750_e84507_d_n4;
        locals.var_vdiff_dn5 = assign50750_e84507_d_n5;
        locals.var_vdiff_dn6 = assign50750_e84507_d_n6;
        locals.var_vdiff_dn7 = assign50750_e84507_d_n7;
        locals.var_vdiff_dn8 = assign50750_e84507_d_n8;
        locals.var_vdiff_dn9 = assign50750_e84507_d_n9;
        locals.var_vdiff_dn10 = assign50750_e84507_d_n10;
        locals.var_vdiff_dn11 = assign50750_e84507_d_n11;

        let (assign50760_e84531, assign50760_e84531_d_n3, assign50760_e84531_d_n4, assign50760_e84531_d_n5, assign50760_e84531_d_n6, assign50760_e84531_d_n7, assign50760_e84531_d_n8, assign50760_e84531_d_n9, assign50760_e84531_d_n10, assign50760_e84531_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 == 0.0)) && (locals.var_guard773 == 0.0)) {
        let assign50760_e84522: f64 = (locals.var_beta1_i * locals.var_vdiff);
        let assign50760_e84523: f64 = (locals.var_beta2_i + assign50760_e84522);
        let assign50760_e84526: f64 = (locals.var_beta0_t * locals.var_vdiff);
        let assign50760_e84528: f64 = (assign50760_e84526 * locals.var_vdiff);
        let assign50760_e84529: f64 = (assign50760_e84523 + assign50760_e84528);
        (assign50760_e84529, ((locals.var_beta1_i * locals.var_vdiff_dn3) + (((locals.var_beta0_t * locals.var_vdiff_dn3) * locals.var_vdiff) + (assign50760_e84526 * locals.var_vdiff_dn3))), ((locals.var_beta1_i * locals.var_vdiff_dn4) + ((((locals.var_beta0_t_dn4 * locals.var_vdiff) + (locals.var_beta0_t * locals.var_vdiff_dn4)) * locals.var_vdiff) + (assign50760_e84526 * locals.var_vdiff_dn4))), ((locals.var_beta1_i * locals.var_vdiff_dn5) + ((((locals.var_beta0_t_dn5 * locals.var_vdiff) + (locals.var_beta0_t * locals.var_vdiff_dn5)) * locals.var_vdiff) + (assign50760_e84526 * locals.var_vdiff_dn5))), ((locals.var_beta1_i * locals.var_vdiff_dn6) + (((locals.var_beta0_t * locals.var_vdiff_dn6) * locals.var_vdiff) + (assign50760_e84526 * locals.var_vdiff_dn6))), ((locals.var_beta1_i * locals.var_vdiff_dn7) + (((locals.var_beta0_t * locals.var_vdiff_dn7) * locals.var_vdiff) + (assign50760_e84526 * locals.var_vdiff_dn7))), ((locals.var_beta1_i * locals.var_vdiff_dn8) + (((locals.var_beta0_t * locals.var_vdiff_dn8) * locals.var_vdiff) + (assign50760_e84526 * locals.var_vdiff_dn8))), ((locals.var_beta1_i * locals.var_vdiff_dn9) + (((locals.var_beta0_t * locals.var_vdiff_dn9) * locals.var_vdiff) + (assign50760_e84526 * locals.var_vdiff_dn9))), ((locals.var_beta1_i * locals.var_vdiff_dn10) + (((locals.var_beta0_t * locals.var_vdiff_dn10) * locals.var_vdiff) + (assign50760_e84526 * locals.var_vdiff_dn10))), ((locals.var_beta1_i * locals.var_vdiff_dn11) + (((locals.var_beta0_t * locals.var_vdiff_dn11) * locals.var_vdiff) + (assign50760_e84526 * locals.var_vdiff_dn11))),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign50760_e84531;
        locals.var_t0_dn3 = assign50760_e84531_d_n3;
        locals.var_t0_dn4 = assign50760_e84531_d_n4;
        locals.var_t0_dn5 = assign50760_e84531_d_n5;
        locals.var_t0_dn6 = assign50760_e84531_d_n6;
        locals.var_t0_dn7 = assign50760_e84531_d_n7;
        locals.var_t0_dn8 = assign50760_e84531_d_n8;
        locals.var_t0_dn9 = assign50760_e84531_d_n9;
        locals.var_t0_dn10 = assign50760_e84531_d_n10;
        locals.var_t0_dn11 = assign50760_e84531_d_n11;

        let (assign50770_e84550, assign50770_e84550_d_n3, assign50770_e84550_d_n4, assign50770_e84550_d_n5, assign50770_e84550_d_n6, assign50770_e84550_d_n7, assign50770_e84550_d_n8, assign50770_e84550_d_n9, assign50770_e84550_d_n10, assign50770_e84550_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 == 0.0)) && (locals.var_guard773 == 0.0)) {
        let assign50770_e84545: f64 = (locals.var_t0 * locals.var_t0);
        let assign50770_e84547: f64 = (assign50770_e84545 + 1e-10);
        let assign50770_e84548: f64 = (assign50770_e84547).sqrt();
        (assign50770_e84548, (((locals.var_t0_dn3 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn3)) / (2.0 * assign50770_e84548)), (((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)) / (2.0 * assign50770_e84548)), (((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)) / (2.0 * assign50770_e84548)), (((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)) / (2.0 * assign50770_e84548)), (((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)) / (2.0 * assign50770_e84548)), (((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)) / (2.0 * assign50770_e84548)), (((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)) / (2.0 * assign50770_e84548)), (((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)) / (2.0 * assign50770_e84548)), (((locals.var_t0_dn11 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn11)) / (2.0 * assign50770_e84548)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign50770_e84550;
        locals.var_t1_dn3 = assign50770_e84550_d_n3;
        locals.var_t1_dn4 = assign50770_e84550_d_n4;
        locals.var_t1_dn5 = assign50770_e84550_d_n5;
        locals.var_t1_dn6 = assign50770_e84550_d_n6;
        locals.var_t1_dn7 = assign50770_e84550_d_n7;
        locals.var_t1_dn8 = assign50770_e84550_d_n8;
        locals.var_t1_dn9 = assign50770_e84550_d_n9;
        locals.var_t1_dn10 = assign50770_e84550_d_n10;
        locals.var_t1_dn11 = assign50770_e84550_d_n11;

        let (assign50780_e84615, assign50780_e84615_d_n3, assign50780_e84615_d_n4, assign50780_e84615_d_n5, assign50780_e84615_d_n6, assign50780_e84615_d_n7, assign50780_e84615_d_n8, assign50780_e84615_d_n9, assign50780_e84615_d_n10, assign50780_e84615_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 == 0.0)) && (locals.var_guard773 == 0.0)) {
        let assign50780_e84563: f64 = (-10.0);
        let assign50780_e84566: f64 = (-locals.var_alpha0_i);
        let assign50780_e84569: f64 = (locals.var_vdiff / locals.var_t1);
        let assign50780_e84570: f64 = { let limited_exp_arg = assign50780_e84569; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign50780_e84571: f64 = (assign50780_e84566 * assign50780_e84570);
        let assign50780_e84573: f64 = (-10.0);
        let assign50780_e84574: f64 = (assign50780_e84571 - assign50780_e84573);
        let assign50780_e84576: f64 = (assign50780_e84574 - p.p645);
        let assign50780_e84578: f64 = (-locals.var_alpha0_i);
        let assign50780_e84581: f64 = (locals.var_vdiff / locals.var_t1);
        let assign50780_e84582: f64 = { let limited_exp_arg = assign50780_e84581; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign50780_e84583: f64 = (assign50780_e84578 * assign50780_e84582);
        let assign50780_e84585: f64 = (-10.0);
        let assign50780_e84586: f64 = (assign50780_e84583 - assign50780_e84585);
        let assign50780_e84588: f64 = (assign50780_e84586 - p.p645);
        let assign50780_e84590: f64 = (-locals.var_alpha0_i);
        let assign50780_e84593: f64 = (locals.var_vdiff / locals.var_t1);
        let assign50780_e84594: f64 = { let limited_exp_arg = assign50780_e84593; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign50780_e84595: f64 = (assign50780_e84590 * assign50780_e84594);
        let assign50780_e84597: f64 = (-10.0);
        let assign50780_e84598: f64 = (assign50780_e84595 - assign50780_e84597);
        let assign50780_e84600: f64 = (assign50780_e84598 - p.p645);
        let assign50780_e84601: f64 = (assign50780_e84588 * assign50780_e84600);
        let assign50780_e84604: f64 = (-10.0);
        let assign50780_e84605: f64 = (4.0 * assign50780_e84604);
        let assign50780_e84607: f64 = (assign50780_e84605 * p.p645);
        let assign50780_e84608: f64 = (assign50780_e84601 - assign50780_e84607);
        let assign50780_e84609: f64 = (assign50780_e84608).sqrt();
        let assign50780_e84610: f64 = (assign50780_e84576 + assign50780_e84609);
        let assign50780_e84611: f64 = (0.5 * assign50780_e84610);
        let assign50780_e84612: f64 = (assign50780_e84563 + assign50780_e84611);
        let assign50780_e84613: f64 = (-assign50780_e84612);
        (assign50780_e84613, (-(0.5 * ((assign50780_e84566 * ({ let limited_exp_arg = assign50780_e84569; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((locals.var_vdiff_dn3 * locals.var_t1) - (locals.var_vdiff * locals.var_t1_dn3)) / (locals.var_t1 * locals.var_t1)))) + ((((assign50780_e84578 * ({ let limited_exp_arg = assign50780_e84581; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((locals.var_vdiff_dn3 * locals.var_t1) - (locals.var_vdiff * locals.var_t1_dn3)) / (locals.var_t1 * locals.var_t1)))) * assign50780_e84600) + (assign50780_e84588 * (assign50780_e84590 * ({ let limited_exp_arg = assign50780_e84593; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((locals.var_vdiff_dn3 * locals.var_t1) - (locals.var_vdiff * locals.var_t1_dn3)) / (locals.var_t1 * locals.var_t1)))))) / (2.0 * assign50780_e84609))))), (-(0.5 * ((assign50780_e84566 * ({ let limited_exp_arg = assign50780_e84569; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((locals.var_vdiff_dn4 * locals.var_t1) - (locals.var_vdiff * locals.var_t1_dn4)) / (locals.var_t1 * locals.var_t1)))) + ((((assign50780_e84578 * ({ let limited_exp_arg = assign50780_e84581; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((locals.var_vdiff_dn4 * locals.var_t1) - (locals.var_vdiff * locals.var_t1_dn4)) / (locals.var_t1 * locals.var_t1)))) * assign50780_e84600) + (assign50780_e84588 * (assign50780_e84590 * ({ let limited_exp_arg = assign50780_e84593; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((locals.var_vdiff_dn4 * locals.var_t1) - (locals.var_vdiff * locals.var_t1_dn4)) / (locals.var_t1 * locals.var_t1)))))) / (2.0 * assign50780_e84609))))), (-(0.5 * ((assign50780_e84566 * ({ let limited_exp_arg = assign50780_e84569; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((locals.var_vdiff_dn5 * locals.var_t1) - (locals.var_vdiff * locals.var_t1_dn5)) / (locals.var_t1 * locals.var_t1)))) + ((((assign50780_e84578 * ({ let limited_exp_arg = assign50780_e84581; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((locals.var_vdiff_dn5 * locals.var_t1) - (locals.var_vdiff * locals.var_t1_dn5)) / (locals.var_t1 * locals.var_t1)))) * assign50780_e84600) + (assign50780_e84588 * (assign50780_e84590 * ({ let limited_exp_arg = assign50780_e84593; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((locals.var_vdiff_dn5 * locals.var_t1) - (locals.var_vdiff * locals.var_t1_dn5)) / (locals.var_t1 * locals.var_t1)))))) / (2.0 * assign50780_e84609))))), (-(0.5 * ((assign50780_e84566 * ({ let limited_exp_arg = assign50780_e84569; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((locals.var_vdiff_dn6 * locals.var_t1) - (locals.var_vdiff * locals.var_t1_dn6)) / (locals.var_t1 * locals.var_t1)))) + ((((assign50780_e84578 * ({ let limited_exp_arg = assign50780_e84581; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((locals.var_vdiff_dn6 * locals.var_t1) - (locals.var_vdiff * locals.var_t1_dn6)) / (locals.var_t1 * locals.var_t1)))) * assign50780_e84600) + (assign50780_e84588 * (assign50780_e84590 * ({ let limited_exp_arg = assign50780_e84593; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((locals.var_vdiff_dn6 * locals.var_t1) - (locals.var_vdiff * locals.var_t1_dn6)) / (locals.var_t1 * locals.var_t1)))))) / (2.0 * assign50780_e84609))))), (-(0.5 * ((assign50780_e84566 * ({ let limited_exp_arg = assign50780_e84569; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((locals.var_vdiff_dn7 * locals.var_t1) - (locals.var_vdiff * locals.var_t1_dn7)) / (locals.var_t1 * locals.var_t1)))) + ((((assign50780_e84578 * ({ let limited_exp_arg = assign50780_e84581; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((locals.var_vdiff_dn7 * locals.var_t1) - (locals.var_vdiff * locals.var_t1_dn7)) / (locals.var_t1 * locals.var_t1)))) * assign50780_e84600) + (assign50780_e84588 * (assign50780_e84590 * ({ let limited_exp_arg = assign50780_e84593; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((locals.var_vdiff_dn7 * locals.var_t1) - (locals.var_vdiff * locals.var_t1_dn7)) / (locals.var_t1 * locals.var_t1)))))) / (2.0 * assign50780_e84609))))), (-(0.5 * ((assign50780_e84566 * ({ let limited_exp_arg = assign50780_e84569; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((locals.var_vdiff_dn8 * locals.var_t1) - (locals.var_vdiff * locals.var_t1_dn8)) / (locals.var_t1 * locals.var_t1)))) + ((((assign50780_e84578 * ({ let limited_exp_arg = assign50780_e84581; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((locals.var_vdiff_dn8 * locals.var_t1) - (locals.var_vdiff * locals.var_t1_dn8)) / (locals.var_t1 * locals.var_t1)))) * assign50780_e84600) + (assign50780_e84588 * (assign50780_e84590 * ({ let limited_exp_arg = assign50780_e84593; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((locals.var_vdiff_dn8 * locals.var_t1) - (locals.var_vdiff * locals.var_t1_dn8)) / (locals.var_t1 * locals.var_t1)))))) / (2.0 * assign50780_e84609))))), (-(0.5 * ((assign50780_e84566 * ({ let limited_exp_arg = assign50780_e84569; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((locals.var_vdiff_dn9 * locals.var_t1) - (locals.var_vdiff * locals.var_t1_dn9)) / (locals.var_t1 * locals.var_t1)))) + ((((assign50780_e84578 * ({ let limited_exp_arg = assign50780_e84581; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((locals.var_vdiff_dn9 * locals.var_t1) - (locals.var_vdiff * locals.var_t1_dn9)) / (locals.var_t1 * locals.var_t1)))) * assign50780_e84600) + (assign50780_e84588 * (assign50780_e84590 * ({ let limited_exp_arg = assign50780_e84593; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((locals.var_vdiff_dn9 * locals.var_t1) - (locals.var_vdiff * locals.var_t1_dn9)) / (locals.var_t1 * locals.var_t1)))))) / (2.0 * assign50780_e84609))))), (-(0.5 * ((assign50780_e84566 * ({ let limited_exp_arg = assign50780_e84569; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((locals.var_vdiff_dn10 * locals.var_t1) - (locals.var_vdiff * locals.var_t1_dn10)) / (locals.var_t1 * locals.var_t1)))) + ((((assign50780_e84578 * ({ let limited_exp_arg = assign50780_e84581; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((locals.var_vdiff_dn10 * locals.var_t1) - (locals.var_vdiff * locals.var_t1_dn10)) / (locals.var_t1 * locals.var_t1)))) * assign50780_e84600) + (assign50780_e84588 * (assign50780_e84590 * ({ let limited_exp_arg = assign50780_e84593; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((locals.var_vdiff_dn10 * locals.var_t1) - (locals.var_vdiff * locals.var_t1_dn10)) / (locals.var_t1 * locals.var_t1)))))) / (2.0 * assign50780_e84609))))), (-(0.5 * ((assign50780_e84566 * ({ let limited_exp_arg = assign50780_e84569; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((locals.var_vdiff_dn11 * locals.var_t1) - (locals.var_vdiff * locals.var_t1_dn11)) / (locals.var_t1 * locals.var_t1)))) + ((((assign50780_e84578 * ({ let limited_exp_arg = assign50780_e84581; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((locals.var_vdiff_dn11 * locals.var_t1) - (locals.var_vdiff * locals.var_t1_dn11)) / (locals.var_t1 * locals.var_t1)))) * assign50780_e84600) + (assign50780_e84588 * (assign50780_e84590 * ({ let limited_exp_arg = assign50780_e84593; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((locals.var_vdiff_dn11 * locals.var_t1) - (locals.var_vdiff * locals.var_t1_dn11)) / (locals.var_t1 * locals.var_t1)))))) / (2.0 * assign50780_e84609))))),)
    } else {
        (locals.var_ratio, locals.var_ratio_dn3, locals.var_ratio_dn4, locals.var_ratio_dn5, locals.var_ratio_dn6, locals.var_ratio_dn7, locals.var_ratio_dn8, locals.var_ratio_dn9, locals.var_ratio_dn10, locals.var_ratio_dn11,)
    }
};
        locals.var_ratio = assign50780_e84615;
        locals.var_ratio_dn3 = assign50780_e84615_d_n3;
        locals.var_ratio_dn4 = assign50780_e84615_d_n4;
        locals.var_ratio_dn5 = assign50780_e84615_d_n5;
        locals.var_ratio_dn6 = assign50780_e84615_d_n6;
        locals.var_ratio_dn7 = assign50780_e84615_d_n7;
        locals.var_ratio_dn8 = assign50780_e84615_d_n8;
        locals.var_ratio_dn9 = assign50780_e84615_d_n9;
        locals.var_ratio_dn10 = assign50780_e84615_d_n10;
        locals.var_ratio_dn11 = assign50780_e84615_d_n11;

        let (assign50790_e84631, assign50790_e84631_d_n3, assign50790_e84631_d_n4, assign50790_e84631_d_n5, assign50790_e84631_d_n6, assign50790_e84631_d_n7, assign50790_e84631_d_n8, assign50790_e84631_d_n9, assign50790_e84631_d_n10, assign50790_e84631_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 == 0.0)) && (locals.var_guard773 == 0.0)) {
        let assign50790_e84629: f64 = (locals.var_ratio * locals.var_ids);
        (assign50790_e84629, ((locals.var_ratio_dn3 * locals.var_ids) + (locals.var_ratio * locals.var_ids_dn3)), ((locals.var_ratio_dn4 * locals.var_ids) + (locals.var_ratio * locals.var_ids_dn4)), ((locals.var_ratio_dn5 * locals.var_ids) + (locals.var_ratio * locals.var_ids_dn5)), ((locals.var_ratio_dn6 * locals.var_ids) + (locals.var_ratio * locals.var_ids_dn6)), ((locals.var_ratio_dn7 * locals.var_ids) + (locals.var_ratio * locals.var_ids_dn7)), ((locals.var_ratio_dn8 * locals.var_ids) + (locals.var_ratio * locals.var_ids_dn8)), ((locals.var_ratio_dn9 * locals.var_ids) + (locals.var_ratio * locals.var_ids_dn9)), ((locals.var_ratio_dn10 * locals.var_ids) + (locals.var_ratio * locals.var_ids_dn10)), ((locals.var_ratio_dn11 * locals.var_ids) + (locals.var_ratio * locals.var_ids_dn11)),)
    } else {
        (locals.var_idsmosfet, locals.var_idsmosfet_dn3, locals.var_idsmosfet_dn4, locals.var_idsmosfet_dn5, locals.var_idsmosfet_dn6, locals.var_idsmosfet_dn7, locals.var_idsmosfet_dn8, locals.var_idsmosfet_dn9, locals.var_idsmosfet_dn10, locals.var_idsmosfet_dn11,)
    }
};
        locals.var_idsmosfet = assign50790_e84631;
        locals.var_idsmosfet_dn3 = assign50790_e84631_d_n3;
        locals.var_idsmosfet_dn4 = assign50790_e84631_d_n4;
        locals.var_idsmosfet_dn5 = assign50790_e84631_d_n5;
        locals.var_idsmosfet_dn6 = assign50790_e84631_d_n6;
        locals.var_idsmosfet_dn7 = assign50790_e84631_d_n7;
        locals.var_idsmosfet_dn8 = assign50790_e84631_d_n8;
        locals.var_idsmosfet_dn9 = assign50790_e84631_d_n9;
        locals.var_idsmosfet_dn10 = assign50790_e84631_d_n10;
        locals.var_idsmosfet_dn11 = assign50790_e84631_d_n11;

        let (assign50800_e84648, assign50800_e84648_d_n3, assign50800_e84648_d_n4, assign50800_e84648_d_n5, assign50800_e84648_d_n6, assign50800_e84648_d_n7, assign50800_e84648_d_n8, assign50800_e84648_d_n9, assign50800_e84648_d_n10, assign50800_e84648_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 == 0.0)) {
        let assign50800_e84643: f64 = (locals.var_ebjtii_i * locals.var_leff);
        let assign50800_e84644: f64 = (locals.var_cbjtii_i + assign50800_e84643);
        let assign50800_e84646: f64 = (assign50800_e84644 / locals.var_leff);
        (assign50800_e84646, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign50800_e84648;
        locals.var_t0_dn3 = assign50800_e84648_d_n3;
        locals.var_t0_dn4 = assign50800_e84648_d_n4;
        locals.var_t0_dn5 = assign50800_e84648_d_n5;
        locals.var_t0_dn6 = assign50800_e84648_d_n6;
        locals.var_t0_dn7 = assign50800_e84648_d_n7;
        locals.var_t0_dn8 = assign50800_e84648_d_n8;
        locals.var_t0_dn9 = assign50800_e84648_d_n9;
        locals.var_t0_dn10 = assign50800_e84648_d_n10;
        locals.var_t0_dn11 = assign50800_e84648_d_n11;

        let (assign50810_e84667, assign50810_e84667_d_n4, assign50810_e84667_d_n5,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 == 0.0)) {
        let assign50810_e84662: f64 = (locals.var_tratio - 1.0);
        let assign50810_e84663: f64 = (p.p666 * assign50810_e84662);
        let assign50810_e84664: f64 = (1.0 + assign50810_e84663);
        let assign50810_e84665: f64 = (locals.var_vbci_i * assign50810_e84664);
        (assign50810_e84665, (locals.var_vbci_i * (p.p666 * locals.var_tratio_dn4)), (locals.var_vbci_i * (p.p666 * locals.var_tratio_dn5)),)
    } else {
        (locals.var_vbc, locals.var_vbc_dn4, locals.var_vbc_dn5,)
    }
};
        locals.var_vbc = assign50810_e84667;
        locals.var_vbc_dn4 = assign50810_e84667_d_n4;
        locals.var_vbc_dn5 = assign50810_e84667_d_n5;

        let assign50820_e84670: f64 = if locals.var_sigvds > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard774 = assign50820_e84670;

        let (assign50830_e84685, assign50830_e84685_d_n3, assign50830_e84685_d_n4, assign50830_e84685_d_n5, assign50830_e84685_d_n6, assign50830_e84685_d_n7, assign50830_e84685_d_n8, assign50830_e84685_d_n9, assign50830_e84685_d_n10, assign50830_e84685_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 == 0.0)) && (locals.var_guard774 != 0.0)) {
        let assign50830_e84683: f64 = (locals.var_vbc - locals.var_vbd_jct);
        (assign50830_e84683, 0.0, locals.var_vbc_dn4, locals.var_vbc_dn5, (-locals.var_vbd_jct_dn6), 0.0, 0.0, 0.0, (-locals.var_vbd_jct_dn10), 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign50830_e84685;
        locals.var_t1_dn3 = assign50830_e84685_d_n3;
        locals.var_t1_dn4 = assign50830_e84685_d_n4;
        locals.var_t1_dn5 = assign50830_e84685_d_n5;
        locals.var_t1_dn6 = assign50830_e84685_d_n6;
        locals.var_t1_dn7 = assign50830_e84685_d_n7;
        locals.var_t1_dn8 = assign50830_e84685_d_n8;
        locals.var_t1_dn9 = assign50830_e84685_d_n9;
        locals.var_t1_dn10 = assign50830_e84685_d_n10;
        locals.var_t1_dn11 = assign50830_e84685_d_n11;

        let (assign50840_e84701, assign50840_e84701_d_n3, assign50840_e84701_d_n4, assign50840_e84701_d_n5, assign50840_e84701_d_n6, assign50840_e84701_d_n7, assign50840_e84701_d_n8, assign50840_e84701_d_n9, assign50840_e84701_d_n10, assign50840_e84701_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 == 0.0)) && (locals.var_guard774 == 0.0)) {
        let assign50840_e84699: f64 = (locals.var_vbc - locals.var_vbs_jct);
        (assign50840_e84699, 0.0, locals.var_vbc_dn4, locals.var_vbc_dn5, 0.0, (-locals.var_vbs_jct_dn7), 0.0, 0.0, (-locals.var_vbs_jct_dn10), 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign50840_e84701;
        locals.var_t1_dn3 = assign50840_e84701_d_n3;
        locals.var_t1_dn4 = assign50840_e84701_d_n4;
        locals.var_t1_dn5 = assign50840_e84701_d_n5;
        locals.var_t1_dn6 = assign50840_e84701_d_n6;
        locals.var_t1_dn7 = assign50840_e84701_d_n7;
        locals.var_t1_dn8 = assign50840_e84701_d_n8;
        locals.var_t1_dn9 = assign50840_e84701_d_n9;
        locals.var_t1_dn10 = assign50840_e84701_d_n10;
        locals.var_t1_dn11 = assign50840_e84701_d_n11;

        let (assign50850_e84714, assign50850_e84714_d_n3, assign50850_e84714_d_n4, assign50850_e84714_d_n5, assign50850_e84714_d_n6, assign50850_e84714_d_n7, assign50850_e84714_d_n8, assign50850_e84714_d_n9, assign50850_e84714_d_n10, assign50850_e84714_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 == 0.0)) {
        let assign50850_e84712: f64 = (locals.var_mbjtii_i - 1.0);
        (assign50850_e84712, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign50850_e84714;
        locals.var_t2_dn3 = assign50850_e84714_d_n3;
        locals.var_t2_dn4 = assign50850_e84714_d_n4;
        locals.var_t2_dn5 = assign50850_e84714_d_n5;
        locals.var_t2_dn6 = assign50850_e84714_d_n6;
        locals.var_t2_dn7 = assign50850_e84714_d_n7;
        locals.var_t2_dn8 = assign50850_e84714_d_n8;
        locals.var_t2_dn9 = assign50850_e84714_d_n9;
        locals.var_t2_dn10 = assign50850_e84714_d_n10;
        locals.var_t2_dn11 = assign50850_e84714_d_n11;

        let assign50860_e84717: f64 = if locals.var_t1 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard775 = assign50860_e84717;

        let (assign50870_e84735, assign50870_e84735_d_n3, assign50870_e84735_d_n4, assign50870_e84735_d_n5, assign50870_e84735_d_n6, assign50870_e84735_d_n7, assign50870_e84735_d_n8, assign50870_e84735_d_n9, assign50870_e84735_d_n10, assign50870_e84735_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 == 0.0)) && (locals.var_guard775 != 0.0)) {
        let assign50870_e84729: f64 = (-locals.var_abjtii_i);
        let assign50870_e84732: f64 = (locals.var_t1).powf(locals.var_t2);
        let assign50870_e84733: f64 = (assign50870_e84729 * assign50870_e84732);
        (assign50870_e84733, (assign50870_e84729 * if locals.var_t2_dn3 == 0.0 && ((locals.var_t2) as f64).is_finite() && ((locals.var_t2) as f64).fract() == 0.0 { if locals.var_t2 == 0.0 { 0.0 } else { (locals.var_t2 * ((locals.var_t1).powf(locals.var_t2 - 1.0) * locals.var_t1_dn3)) } } else { (assign50870_e84732 * ((locals.var_t2_dn3 * (locals.var_t1).ln()) + (locals.var_t2 * (locals.var_t1_dn3 / locals.var_t1)))) }), (assign50870_e84729 * if locals.var_t2_dn4 == 0.0 && ((locals.var_t2) as f64).is_finite() && ((locals.var_t2) as f64).fract() == 0.0 { if locals.var_t2 == 0.0 { 0.0 } else { (locals.var_t2 * ((locals.var_t1).powf(locals.var_t2 - 1.0) * locals.var_t1_dn4)) } } else { (assign50870_e84732 * ((locals.var_t2_dn4 * (locals.var_t1).ln()) + (locals.var_t2 * (locals.var_t1_dn4 / locals.var_t1)))) }), (assign50870_e84729 * if locals.var_t2_dn5 == 0.0 && ((locals.var_t2) as f64).is_finite() && ((locals.var_t2) as f64).fract() == 0.0 { if locals.var_t2 == 0.0 { 0.0 } else { (locals.var_t2 * ((locals.var_t1).powf(locals.var_t2 - 1.0) * locals.var_t1_dn5)) } } else { (assign50870_e84732 * ((locals.var_t2_dn5 * (locals.var_t1).ln()) + (locals.var_t2 * (locals.var_t1_dn5 / locals.var_t1)))) }), (assign50870_e84729 * if locals.var_t2_dn6 == 0.0 && ((locals.var_t2) as f64).is_finite() && ((locals.var_t2) as f64).fract() == 0.0 { if locals.var_t2 == 0.0 { 0.0 } else { (locals.var_t2 * ((locals.var_t1).powf(locals.var_t2 - 1.0) * locals.var_t1_dn6)) } } else { (assign50870_e84732 * ((locals.var_t2_dn6 * (locals.var_t1).ln()) + (locals.var_t2 * (locals.var_t1_dn6 / locals.var_t1)))) }), (assign50870_e84729 * if locals.var_t2_dn7 == 0.0 && ((locals.var_t2) as f64).is_finite() && ((locals.var_t2) as f64).fract() == 0.0 { if locals.var_t2 == 0.0 { 0.0 } else { (locals.var_t2 * ((locals.var_t1).powf(locals.var_t2 - 1.0) * locals.var_t1_dn7)) } } else { (assign50870_e84732 * ((locals.var_t2_dn7 * (locals.var_t1).ln()) + (locals.var_t2 * (locals.var_t1_dn7 / locals.var_t1)))) }), (assign50870_e84729 * if locals.var_t2_dn8 == 0.0 && ((locals.var_t2) as f64).is_finite() && ((locals.var_t2) as f64).fract() == 0.0 { if locals.var_t2 == 0.0 { 0.0 } else { (locals.var_t2 * ((locals.var_t1).powf(locals.var_t2 - 1.0) * locals.var_t1_dn8)) } } else { (assign50870_e84732 * ((locals.var_t2_dn8 * (locals.var_t1).ln()) + (locals.var_t2 * (locals.var_t1_dn8 / locals.var_t1)))) }), (assign50870_e84729 * if locals.var_t2_dn9 == 0.0 && ((locals.var_t2) as f64).is_finite() && ((locals.var_t2) as f64).fract() == 0.0 { if locals.var_t2 == 0.0 { 0.0 } else { (locals.var_t2 * ((locals.var_t1).powf(locals.var_t2 - 1.0) * locals.var_t1_dn9)) } } else { (assign50870_e84732 * ((locals.var_t2_dn9 * (locals.var_t1).ln()) + (locals.var_t2 * (locals.var_t1_dn9 / locals.var_t1)))) }), (assign50870_e84729 * if locals.var_t2_dn10 == 0.0 && ((locals.var_t2) as f64).is_finite() && ((locals.var_t2) as f64).fract() == 0.0 { if locals.var_t2 == 0.0 { 0.0 } else { (locals.var_t2 * ((locals.var_t1).powf(locals.var_t2 - 1.0) * locals.var_t1_dn10)) } } else { (assign50870_e84732 * ((locals.var_t2_dn10 * (locals.var_t1).ln()) + (locals.var_t2 * (locals.var_t1_dn10 / locals.var_t1)))) }), (assign50870_e84729 * if locals.var_t2_dn11 == 0.0 && ((locals.var_t2) as f64).is_finite() && ((locals.var_t2) as f64).fract() == 0.0 { if locals.var_t2 == 0.0 { 0.0 } else { (locals.var_t2 * ((locals.var_t1).powf(locals.var_t2 - 1.0) * locals.var_t1_dn11)) } } else { (assign50870_e84732 * ((locals.var_t2_dn11 * (locals.var_t1).ln()) + (locals.var_t2 * (locals.var_t1_dn11 / locals.var_t1)))) }),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign50870_e84735;
        locals.var_t3_dn3 = assign50870_e84735_d_n3;
        locals.var_t3_dn4 = assign50870_e84735_d_n4;
        locals.var_t3_dn5 = assign50870_e84735_d_n5;
        locals.var_t3_dn6 = assign50870_e84735_d_n6;
        locals.var_t3_dn7 = assign50870_e84735_d_n7;
        locals.var_t3_dn8 = assign50870_e84735_d_n8;
        locals.var_t3_dn9 = assign50870_e84735_d_n9;
        locals.var_t3_dn10 = assign50870_e84735_d_n10;
        locals.var_t3_dn11 = assign50870_e84735_d_n11;

        let (assign50880_e84749, assign50880_e84749_d_n3, assign50880_e84749_d_n4, assign50880_e84749_d_n5, assign50880_e84749_d_n6, assign50880_e84749_d_n7, assign50880_e84749_d_n8, assign50880_e84749_d_n9, assign50880_e84749_d_n10, assign50880_e84749_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 == 0.0)) && (locals.var_guard775 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign50880_e84749;
        locals.var_t3_dn3 = assign50880_e84749_d_n3;
        locals.var_t3_dn4 = assign50880_e84749_d_n4;
        locals.var_t3_dn5 = assign50880_e84749_d_n5;
        locals.var_t3_dn6 = assign50880_e84749_d_n6;
        locals.var_t3_dn7 = assign50880_e84749_d_n7;
        locals.var_t3_dn8 = assign50880_e84749_d_n8;
        locals.var_t3_dn9 = assign50880_e84749_d_n9;
        locals.var_t3_dn10 = assign50880_e84749_d_n10;
        locals.var_t3_dn11 = assign50880_e84749_d_n11;

        let (assign50890_e84761, assign50890_e84761_d_n3, assign50890_e84761_d_n4, assign50890_e84761_d_n5, assign50890_e84761_d_n6, assign50890_e84761_d_n7, assign50890_e84761_d_n8, assign50890_e84761_d_n9, assign50890_e84761_d_n10, assign50890_e84761_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 == 0.0)) {
        let assign50890_e84759: f64 = { let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign50890_e84759, ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn3), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn4), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn5), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn6), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn7), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn8), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn9), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn10), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn11),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign50890_e84761;
        locals.var_t4_dn3 = assign50890_e84761_d_n3;
        locals.var_t4_dn4 = assign50890_e84761_d_n4;
        locals.var_t4_dn5 = assign50890_e84761_d_n5;
        locals.var_t4_dn6 = assign50890_e84761_d_n6;
        locals.var_t4_dn7 = assign50890_e84761_d_n7;
        locals.var_t4_dn8 = assign50890_e84761_d_n8;
        locals.var_t4_dn9 = assign50890_e84761_d_n9;
        locals.var_t4_dn10 = assign50890_e84761_d_n10;
        locals.var_t4_dn11 = assign50890_e84761_d_n11;

        let (assign50900_e84780, assign50900_e84780_d_n3, assign50900_e84780_d_n4, assign50900_e84780_d_n5, assign50900_e84780_d_n6, assign50900_e84780_d_n7, assign50900_e84780_d_n8, assign50900_e84780_d_n9, assign50900_e84780_d_n10, assign50900_e84780_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 == 0.0)) {
        let assign50900_e84772: f64 = (locals.var_t0 * locals.var_sigvds);
        let assign50900_e84774: f64 = (assign50900_e84772 * locals.var_ic);
        let assign50900_e84776: f64 = (assign50900_e84774 * locals.var_t1);
        let assign50900_e84778: f64 = (assign50900_e84776 * locals.var_t4);
        (assign50900_e84778, (((((((locals.var_t0_dn3 * locals.var_sigvds) * locals.var_ic) + (assign50900_e84772 * locals.var_ic_dn3)) * locals.var_t1) + (assign50900_e84774 * locals.var_t1_dn3)) * locals.var_t4) + (assign50900_e84776 * locals.var_t4_dn3)), (((((((locals.var_t0_dn4 * locals.var_sigvds) * locals.var_ic) + (assign50900_e84772 * locals.var_ic_dn4)) * locals.var_t1) + (assign50900_e84774 * locals.var_t1_dn4)) * locals.var_t4) + (assign50900_e84776 * locals.var_t4_dn4)), (((((((locals.var_t0_dn5 * locals.var_sigvds) * locals.var_ic) + (assign50900_e84772 * locals.var_ic_dn5)) * locals.var_t1) + (assign50900_e84774 * locals.var_t1_dn5)) * locals.var_t4) + (assign50900_e84776 * locals.var_t4_dn5)), (((((((locals.var_t0_dn6 * locals.var_sigvds) * locals.var_ic) + (assign50900_e84772 * locals.var_ic_dn6)) * locals.var_t1) + (assign50900_e84774 * locals.var_t1_dn6)) * locals.var_t4) + (assign50900_e84776 * locals.var_t4_dn6)), (((((((locals.var_t0_dn7 * locals.var_sigvds) * locals.var_ic) + (assign50900_e84772 * locals.var_ic_dn7)) * locals.var_t1) + (assign50900_e84774 * locals.var_t1_dn7)) * locals.var_t4) + (assign50900_e84776 * locals.var_t4_dn7)), (((((((locals.var_t0_dn8 * locals.var_sigvds) * locals.var_ic) + (assign50900_e84772 * locals.var_ic_dn8)) * locals.var_t1) + (assign50900_e84774 * locals.var_t1_dn8)) * locals.var_t4) + (assign50900_e84776 * locals.var_t4_dn8)), (((((((locals.var_t0_dn9 * locals.var_sigvds) * locals.var_ic) + (assign50900_e84772 * locals.var_ic_dn9)) * locals.var_t1) + (assign50900_e84774 * locals.var_t1_dn9)) * locals.var_t4) + (assign50900_e84776 * locals.var_t4_dn9)), (((((((locals.var_t0_dn10 * locals.var_sigvds) * locals.var_ic) + (assign50900_e84772 * locals.var_ic_dn10)) * locals.var_t1) + (assign50900_e84774 * locals.var_t1_dn10)) * locals.var_t4) + (assign50900_e84776 * locals.var_t4_dn10)), (((((((locals.var_t0_dn11 * locals.var_sigvds) * locals.var_ic) + (assign50900_e84772 * locals.var_ic_dn11)) * locals.var_t1) + (assign50900_e84774 * locals.var_t1_dn11)) * locals.var_t4) + (assign50900_e84776 * locals.var_t4_dn11)),)
    } else {
        (locals.var_iiibjt, locals.var_iiibjt_dn3, locals.var_iiibjt_dn4, locals.var_iiibjt_dn5, locals.var_iiibjt_dn6, locals.var_iiibjt_dn7, locals.var_iiibjt_dn8, locals.var_iiibjt_dn9, locals.var_iiibjt_dn10, locals.var_iiibjt_dn11,)
    }
};
        locals.var_iiibjt = assign50900_e84780;
        locals.var_iiibjt_dn3 = assign50900_e84780_d_n3;
        locals.var_iiibjt_dn4 = assign50900_e84780_d_n4;
        locals.var_iiibjt_dn5 = assign50900_e84780_d_n5;
        locals.var_iiibjt_dn6 = assign50900_e84780_d_n6;
        locals.var_iiibjt_dn7 = assign50900_e84780_d_n7;
        locals.var_iiibjt_dn8 = assign50900_e84780_d_n8;
        locals.var_iiibjt_dn9 = assign50900_e84780_d_n9;
        locals.var_iiibjt_dn10 = assign50900_e84780_d_n10;
        locals.var_iiibjt_dn11 = assign50900_e84780_d_n11;

        let (assign50910_e84793, assign50910_e84793_d_n3, assign50910_e84793_d_n4, assign50910_e84793_d_n5, assign50910_e84793_d_n6, assign50910_e84793_d_n7, assign50910_e84793_d_n8, assign50910_e84793_d_n9, assign50910_e84793_d_n10, assign50910_e84793_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 == 0.0)) {
        let assign50910_e84791: f64 = (locals.var_idsmosfet + locals.var_iiibjt);
        (assign50910_e84791, (locals.var_idsmosfet_dn3 + locals.var_iiibjt_dn3), (locals.var_idsmosfet_dn4 + locals.var_iiibjt_dn4), (locals.var_idsmosfet_dn5 + locals.var_iiibjt_dn5), (locals.var_idsmosfet_dn6 + locals.var_iiibjt_dn6), (locals.var_idsmosfet_dn7 + locals.var_iiibjt_dn7), (locals.var_idsmosfet_dn8 + locals.var_iiibjt_dn8), (locals.var_idsmosfet_dn9 + locals.var_iiibjt_dn9), (locals.var_idsmosfet_dn10 + locals.var_iiibjt_dn10), (locals.var_idsmosfet_dn11 + locals.var_iiibjt_dn11),)
    } else {
        (locals.var_iii, locals.var_iii_dn3, locals.var_iii_dn4, locals.var_iii_dn5, locals.var_iii_dn6, locals.var_iii_dn7, locals.var_iii_dn8, locals.var_iii_dn9, locals.var_iii_dn10, locals.var_iii_dn11,)
    }
};
        locals.var_iii = assign50910_e84793;
        locals.var_iii_dn3 = assign50910_e84793_d_n3;
        locals.var_iii_dn4 = assign50910_e84793_d_n4;
        locals.var_iii_dn5 = assign50910_e84793_d_n5;
        locals.var_iii_dn6 = assign50910_e84793_d_n6;
        locals.var_iii_dn7 = assign50910_e84793_d_n7;
        locals.var_iii_dn8 = assign50910_e84793_d_n8;
        locals.var_iii_dn9 = assign50910_e84793_d_n9;
        locals.var_iii_dn10 = assign50910_e84793_d_n10;
        locals.var_iii_dn11 = assign50910_e84793_d_n11;

        let (assign50920_e84800, assign50920_e84800_d_n3, assign50920_e84800_d_n4, assign50920_e84800_d_n5, assign50920_e84800_d_n6, assign50920_e84800_d_n7, assign50920_e84800_d_n8, assign50920_e84800_d_n9, assign50920_e84800_d_n10, assign50920_e84800_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign50920_e84798: f64 = (locals.var_iii * locals.var_devsign);
        (assign50920_e84798, (locals.var_iii_dn3 * locals.var_devsign), (locals.var_iii_dn4 * locals.var_devsign), (locals.var_iii_dn5 * locals.var_devsign), (locals.var_iii_dn6 * locals.var_devsign), (locals.var_iii_dn7 * locals.var_devsign), (locals.var_iii_dn8 * locals.var_devsign), (locals.var_iii_dn9 * locals.var_devsign), (locals.var_iii_dn10 * locals.var_devsign), (locals.var_iii_dn11 * locals.var_devsign),)
    } else {
        (locals.var_isub, locals.var_isub_dn3, locals.var_isub_dn4, locals.var_isub_dn5, locals.var_isub_dn6, locals.var_isub_dn7, locals.var_isub_dn8, locals.var_isub_dn9, locals.var_isub_dn10, locals.var_isub_dn11,)
    }
};
        locals.var_isub = assign50920_e84800;
        locals.var_isub_dn3 = assign50920_e84800_d_n3;
        locals.var_isub_dn4 = assign50920_e84800_d_n4;
        locals.var_isub_dn5 = assign50920_e84800_d_n5;
        locals.var_isub_dn6 = assign50920_e84800_d_n6;
        locals.var_isub_dn7 = assign50920_e84800_d_n7;
        locals.var_isub_dn8 = assign50920_e84800_d_n8;
        locals.var_isub_dn9 = assign50920_e84800_d_n9;
        locals.var_isub_dn10 = assign50920_e84800_d_n10;
        locals.var_isub_dn11 = assign50920_e84800_d_n11;

        let (assign50930_e84811, assign50930_e84811_d_n4, assign50930_e84811_d_n5,) = {
    if (locals.var_guard492 == 0.0) {
        let assign50930_e84807: f64 = (locals.var_tratio - 1.0);
        let assign50930_e84808: f64 = (locals.var_aigc1_i * assign50930_e84807);
        let assign50930_e84809: f64 = (locals.var_aigc_i + assign50930_e84808);
        (assign50930_e84809, (locals.var_aigc_i_dn4 + (locals.var_aigc1_i * locals.var_tratio_dn4)), (locals.var_aigc_i_dn5 + (locals.var_aigc1_i * locals.var_tratio_dn5)),)
    } else {
        (locals.var_aigc_i, locals.var_aigc_i_dn4, locals.var_aigc_i_dn5,)
    }
};
        locals.var_aigc_i = assign50930_e84811;
        locals.var_aigc_i_dn4 = assign50930_e84811_d_n4;
        locals.var_aigc_i_dn5 = assign50930_e84811_d_n5;

        let (assign50940_e84822, assign50940_e84822_d_n4, assign50940_e84822_d_n5,) = {
    if (locals.var_guard492 == 0.0) {
        let assign50940_e84818: f64 = (locals.var_tratio - 1.0);
        let assign50940_e84819: f64 = (locals.var_aigs1_i * assign50940_e84818);
        let assign50940_e84820: f64 = (locals.var_aigs_i + assign50940_e84819);
        (assign50940_e84820, (locals.var_aigs_i_dn4 + (locals.var_aigs1_i * locals.var_tratio_dn4)), (locals.var_aigs_i_dn5 + (locals.var_aigs1_i * locals.var_tratio_dn5)),)
    } else {
        (locals.var_aigs_i, locals.var_aigs_i_dn4, locals.var_aigs_i_dn5,)
    }
};
        locals.var_aigs_i = assign50940_e84822;
        locals.var_aigs_i_dn4 = assign50940_e84822_d_n4;
        locals.var_aigs_i_dn5 = assign50940_e84822_d_n5;

        let (assign50950_e84833, assign50950_e84833_d_n4, assign50950_e84833_d_n5,) = {
    if (locals.var_guard492 == 0.0) {
        let assign50950_e84829: f64 = (locals.var_tratio - 1.0);
        let assign50950_e84830: f64 = (locals.var_aigd1_i * assign50950_e84829);
        let assign50950_e84831: f64 = (locals.var_aigd_i + assign50950_e84830);
        (assign50950_e84831, (locals.var_aigd_i_dn4 + (locals.var_aigd1_i * locals.var_tratio_dn4)), (locals.var_aigd_i_dn5 + (locals.var_aigd1_i * locals.var_tratio_dn5)),)
    } else {
        (locals.var_aigd_i, locals.var_aigd_i_dn4, locals.var_aigd_i_dn5,)
    }
};
        locals.var_aigd_i = assign50950_e84833;
        locals.var_aigd_i_dn4 = assign50950_e84833_d_n4;
        locals.var_aigd_i_dn5 = assign50950_e84833_d_n5;

        let (assign50960_e84844, assign50960_e84844_d_n4, assign50960_e84844_d_n5,) = {
    if (locals.var_guard492 == 0.0) {
        let assign50960_e84840: f64 = (locals.var_tratio - 1.0);
        let assign50960_e84841: f64 = (locals.var_alphagb1_t_i * assign50960_e84840);
        let assign50960_e84842: f64 = (locals.var_alphagb1_i + assign50960_e84841);
        (assign50960_e84842, (locals.var_alphagb1_i_dn4 + (locals.var_alphagb1_t_i * locals.var_tratio_dn4)), (locals.var_alphagb1_i_dn5 + (locals.var_alphagb1_t_i * locals.var_tratio_dn5)),)
    } else {
        (locals.var_alphagb1_i, locals.var_alphagb1_i_dn4, locals.var_alphagb1_i_dn5,)
    }
};
        locals.var_alphagb1_i = assign50960_e84844;
        locals.var_alphagb1_i_dn4 = assign50960_e84844_d_n4;
        locals.var_alphagb1_i_dn5 = assign50960_e84844_d_n5;

        let (assign50970_e84855, assign50970_e84855_d_n4, assign50970_e84855_d_n5,) = {
    if (locals.var_guard492 == 0.0) {
        let assign50970_e84851: f64 = (locals.var_tratio - 1.0);
        let assign50970_e84852: f64 = (locals.var_alphagb2_t_i * assign50970_e84851);
        let assign50970_e84853: f64 = (locals.var_alphagb2_i + assign50970_e84852);
        (assign50970_e84853, (locals.var_alphagb2_i_dn4 + (locals.var_alphagb2_t_i * locals.var_tratio_dn4)), (locals.var_alphagb2_i_dn5 + (locals.var_alphagb2_t_i * locals.var_tratio_dn5)),)
    } else {
        (locals.var_alphagb2_i, locals.var_alphagb2_i_dn4, locals.var_alphagb2_i_dn5,)
    }
};
        locals.var_alphagb2_i = assign50970_e84855;
        locals.var_alphagb2_i_dn4 = assign50970_e84855_d_n4;
        locals.var_alphagb2_i_dn5 = assign50970_e84855_d_n5;

    }

    pub(super) fn stamp_transient_block_173(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign50980_e84866, assign50980_e84866_d_n4, assign50980_e84866_d_n5,) = {
    if (locals.var_guard492 == 0.0) {
        let assign50980_e84862: f64 = (locals.var_tratio - 1.0);
        let assign50980_e84863: f64 = (locals.var_aigbcp2_t_i * assign50980_e84862);
        let assign50980_e84864: f64 = (locals.var_aigbcp2_i + assign50980_e84863);
        (assign50980_e84864, (locals.var_aigbcp2_i_dn4 + (locals.var_aigbcp2_t_i * locals.var_tratio_dn4)), (locals.var_aigbcp2_i_dn5 + (locals.var_aigbcp2_t_i * locals.var_tratio_dn5)),)
    } else {
        (locals.var_aigbcp2_i, locals.var_aigbcp2_i_dn4, locals.var_aigbcp2_i_dn5,)
    }
};
        locals.var_aigbcp2_i = assign50980_e84866;
        locals.var_aigbcp2_i_dn4 = assign50980_e84866_d_n4;
        locals.var_aigbcp2_i_dn5 = assign50980_e84866_d_n5;

        let (assign50990_e84871, assign50990_e84871_d_n3, assign50990_e84871_d_n4, assign50990_e84871_d_n5, assign50990_e84871_d_n6, assign50990_e84871_d_n7, assign50990_e84871_d_n8, assign50990_e84871_d_n9, assign50990_e84871_d_n10, assign50990_e84871_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_igb, locals.var_igb_dn3, locals.var_igb_dn4, locals.var_igb_dn5, locals.var_igb_dn6, locals.var_igb_dn7, locals.var_igb_dn8, locals.var_igb_dn9, locals.var_igb_dn10, locals.var_igb_dn11,)
    }
};
        locals.var_igb = assign50990_e84871;
        locals.var_igb_dn3 = assign50990_e84871_d_n3;
        locals.var_igb_dn4 = assign50990_e84871_d_n4;
        locals.var_igb_dn5 = assign50990_e84871_d_n5;
        locals.var_igb_dn6 = assign50990_e84871_d_n6;
        locals.var_igb_dn7 = assign50990_e84871_d_n7;
        locals.var_igb_dn8 = assign50990_e84871_d_n8;
        locals.var_igb_dn9 = assign50990_e84871_d_n9;
        locals.var_igb_dn10 = assign50990_e84871_d_n10;
        locals.var_igb_dn11 = assign50990_e84871_d_n11;

        let (assign51000_e84876, assign51000_e84876_d_n3, assign51000_e84876_d_n4, assign51000_e84876_d_n5, assign51000_e84876_d_n6, assign51000_e84876_d_n7, assign51000_e84876_d_n8, assign51000_e84876_d_n9, assign51000_e84876_d_n10, assign51000_e84876_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_igcs, locals.var_igcs_dn3, locals.var_igcs_dn4, locals.var_igcs_dn5, locals.var_igcs_dn6, locals.var_igcs_dn7, locals.var_igcs_dn8, locals.var_igcs_dn9, locals.var_igcs_dn10, locals.var_igcs_dn11,)
    }
};
        locals.var_igcs = assign51000_e84876;
        locals.var_igcs_dn3 = assign51000_e84876_d_n3;
        locals.var_igcs_dn4 = assign51000_e84876_d_n4;
        locals.var_igcs_dn5 = assign51000_e84876_d_n5;
        locals.var_igcs_dn6 = assign51000_e84876_d_n6;
        locals.var_igcs_dn7 = assign51000_e84876_d_n7;
        locals.var_igcs_dn8 = assign51000_e84876_d_n8;
        locals.var_igcs_dn9 = assign51000_e84876_d_n9;
        locals.var_igcs_dn10 = assign51000_e84876_d_n10;
        locals.var_igcs_dn11 = assign51000_e84876_d_n11;

        let (assign51010_e84881, assign51010_e84881_d_n3, assign51010_e84881_d_n4, assign51010_e84881_d_n5, assign51010_e84881_d_n6, assign51010_e84881_d_n7, assign51010_e84881_d_n8, assign51010_e84881_d_n9, assign51010_e84881_d_n10, assign51010_e84881_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_igcd, locals.var_igcd_dn3, locals.var_igcd_dn4, locals.var_igcd_dn5, locals.var_igcd_dn6, locals.var_igcd_dn7, locals.var_igcd_dn8, locals.var_igcd_dn9, locals.var_igcd_dn10, locals.var_igcd_dn11,)
    }
};
        locals.var_igcd = assign51010_e84881;
        locals.var_igcd_dn3 = assign51010_e84881_d_n3;
        locals.var_igcd_dn4 = assign51010_e84881_d_n4;
        locals.var_igcd_dn5 = assign51010_e84881_d_n5;
        locals.var_igcd_dn6 = assign51010_e84881_d_n6;
        locals.var_igcd_dn7 = assign51010_e84881_d_n7;
        locals.var_igcd_dn8 = assign51010_e84881_d_n8;
        locals.var_igcd_dn9 = assign51010_e84881_d_n9;
        locals.var_igcd_dn10 = assign51010_e84881_d_n10;
        locals.var_igcd_dn11 = assign51010_e84881_d_n11;

        let (assign51020_e84886, assign51020_e84886_d_n3, assign51020_e84886_d_n4, assign51020_e84886_d_n5, assign51020_e84886_d_n6, assign51020_e84886_d_n7, assign51020_e84886_d_n8, assign51020_e84886_d_n9, assign51020_e84886_d_n10, assign51020_e84886_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_igs, locals.var_igs_dn3, locals.var_igs_dn4, locals.var_igs_dn5, locals.var_igs_dn6, locals.var_igs_dn7, locals.var_igs_dn8, locals.var_igs_dn9, locals.var_igs_dn10, locals.var_igs_dn11,)
    }
};
        locals.var_igs = assign51020_e84886;
        locals.var_igs_dn3 = assign51020_e84886_d_n3;
        locals.var_igs_dn4 = assign51020_e84886_d_n4;
        locals.var_igs_dn5 = assign51020_e84886_d_n5;
        locals.var_igs_dn6 = assign51020_e84886_d_n6;
        locals.var_igs_dn7 = assign51020_e84886_d_n7;
        locals.var_igs_dn8 = assign51020_e84886_d_n8;
        locals.var_igs_dn9 = assign51020_e84886_d_n9;
        locals.var_igs_dn10 = assign51020_e84886_d_n10;
        locals.var_igs_dn11 = assign51020_e84886_d_n11;

        let (assign51030_e84891, assign51030_e84891_d_n3, assign51030_e84891_d_n4, assign51030_e84891_d_n5, assign51030_e84891_d_n6, assign51030_e84891_d_n7, assign51030_e84891_d_n8, assign51030_e84891_d_n9, assign51030_e84891_d_n10, assign51030_e84891_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_igd, locals.var_igd_dn3, locals.var_igd_dn4, locals.var_igd_dn5, locals.var_igd_dn6, locals.var_igd_dn7, locals.var_igd_dn8, locals.var_igd_dn9, locals.var_igd_dn10, locals.var_igd_dn11,)
    }
};
        locals.var_igd = assign51030_e84891;
        locals.var_igd_dn3 = assign51030_e84891_d_n3;
        locals.var_igd_dn4 = assign51030_e84891_d_n4;
        locals.var_igd_dn5 = assign51030_e84891_d_n5;
        locals.var_igd_dn6 = assign51030_e84891_d_n6;
        locals.var_igd_dn7 = assign51030_e84891_d_n7;
        locals.var_igd_dn8 = assign51030_e84891_d_n8;
        locals.var_igd_dn9 = assign51030_e84891_d_n9;
        locals.var_igd_dn10 = assign51030_e84891_d_n10;
        locals.var_igd_dn11 = assign51030_e84891_d_n11;

        let assign51040_e84898: f64 = if ((p.p37 != 0.0) || (p.p38 != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard776 = assign51040_e84898;

        let (assign51050_e84913, assign51050_e84913_d_n3, assign51050_e84913_d_n4, assign51050_e84913_d_n5, assign51050_e84913_d_n6, assign51050_e84913_d_n7, assign51050_e84913_d_n8, assign51050_e84913_d_n9, assign51050_e84913_d_n10, assign51050_e84913_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) {
        let assign51050_e84906: f64 = (locals.var_vgfb - locals.var_psip);
        let assign51050_e84908: f64 = (assign51050_e84906 + locals.var_qs_1);
        let assign51050_e84910: f64 = (assign51050_e84908 + locals.var_qdeff);
        let assign51050_e84911: f64 = (locals.var_nvt * assign51050_e84910);
        (assign51050_e84911, ((locals.var_nvt_dn3 * assign51050_e84910) + (locals.var_nvt * (((locals.var_vgfb_dn3 - locals.var_psip_dn3) + locals.var_qs_1_dn3) + locals.var_qdeff_dn3))), ((locals.var_nvt_dn4 * assign51050_e84910) + (locals.var_nvt * (((locals.var_vgfb_dn4 - locals.var_psip_dn4) + locals.var_qs_1_dn4) + locals.var_qdeff_dn4))), ((locals.var_nvt_dn5 * assign51050_e84910) + (locals.var_nvt * (((locals.var_vgfb_dn5 - locals.var_psip_dn5) + locals.var_qs_1_dn5) + locals.var_qdeff_dn5))), ((locals.var_nvt_dn6 * assign51050_e84910) + (locals.var_nvt * (((locals.var_vgfb_dn6 - locals.var_psip_dn6) + locals.var_qs_1_dn6) + locals.var_qdeff_dn6))), ((locals.var_nvt_dn7 * assign51050_e84910) + (locals.var_nvt * (((locals.var_vgfb_dn7 - locals.var_psip_dn7) + locals.var_qs_1_dn7) + locals.var_qdeff_dn7))), ((locals.var_nvt_dn8 * assign51050_e84910) + (locals.var_nvt * (((locals.var_vgfb_dn8 - locals.var_psip_dn8) + locals.var_qs_1_dn8) + locals.var_qdeff_dn8))), ((locals.var_nvt_dn9 * assign51050_e84910) + (locals.var_nvt * (((locals.var_vgfb_dn9 - locals.var_psip_dn9) + locals.var_qs_1_dn9) + locals.var_qdeff_dn9))), ((locals.var_nvt_dn10 * assign51050_e84910) + (locals.var_nvt * (((locals.var_vgfb_dn10 - locals.var_psip_dn10) + locals.var_qs_1_dn10) + locals.var_qdeff_dn10))), ((locals.var_nvt_dn11 * assign51050_e84910) + (locals.var_nvt * (((locals.var_vgfb_dn11 - locals.var_psip_dn11) + locals.var_qs_1_dn11) + locals.var_qdeff_dn11))),)
    } else {
        (locals.var_voxm1, locals.var_voxm1_dn3, locals.var_voxm1_dn4, locals.var_voxm1_dn5, locals.var_voxm1_dn6, locals.var_voxm1_dn7, locals.var_voxm1_dn8, locals.var_voxm1_dn9, locals.var_voxm1_dn10, locals.var_voxm1_dn11,)
    }
};
        locals.var_voxm1 = assign51050_e84913;
        locals.var_voxm1_dn3 = assign51050_e84913_d_n3;
        locals.var_voxm1_dn4 = assign51050_e84913_d_n4;
        locals.var_voxm1_dn5 = assign51050_e84913_d_n5;
        locals.var_voxm1_dn6 = assign51050_e84913_d_n6;
        locals.var_voxm1_dn7 = assign51050_e84913_d_n7;
        locals.var_voxm1_dn8 = assign51050_e84913_d_n8;
        locals.var_voxm1_dn9 = assign51050_e84913_d_n9;
        locals.var_voxm1_dn10 = assign51050_e84913_d_n10;
        locals.var_voxm1_dn11 = assign51050_e84913_d_n11;

        let (assign51060_e84925, assign51060_e84925_d_n3, assign51060_e84925_d_n4, assign51060_e84925_d_n5, assign51060_e84925_d_n6, assign51060_e84925_d_n7, assign51060_e84925_d_n8, assign51060_e84925_d_n9, assign51060_e84925_d_n10, assign51060_e84925_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) {
        let assign51060_e84920: f64 = (locals.var_voxm1 * locals.var_voxm1);
        let assign51060_e84922: f64 = (assign51060_e84920 + 0.0001);
        let assign51060_e84923: f64 = (assign51060_e84922).sqrt();
        (assign51060_e84923, (((locals.var_voxm1_dn3 * locals.var_voxm1) + (locals.var_voxm1 * locals.var_voxm1_dn3)) / (2.0 * assign51060_e84923)), (((locals.var_voxm1_dn4 * locals.var_voxm1) + (locals.var_voxm1 * locals.var_voxm1_dn4)) / (2.0 * assign51060_e84923)), (((locals.var_voxm1_dn5 * locals.var_voxm1) + (locals.var_voxm1 * locals.var_voxm1_dn5)) / (2.0 * assign51060_e84923)), (((locals.var_voxm1_dn6 * locals.var_voxm1) + (locals.var_voxm1 * locals.var_voxm1_dn6)) / (2.0 * assign51060_e84923)), (((locals.var_voxm1_dn7 * locals.var_voxm1) + (locals.var_voxm1 * locals.var_voxm1_dn7)) / (2.0 * assign51060_e84923)), (((locals.var_voxm1_dn8 * locals.var_voxm1) + (locals.var_voxm1 * locals.var_voxm1_dn8)) / (2.0 * assign51060_e84923)), (((locals.var_voxm1_dn9 * locals.var_voxm1) + (locals.var_voxm1 * locals.var_voxm1_dn9)) / (2.0 * assign51060_e84923)), (((locals.var_voxm1_dn10 * locals.var_voxm1) + (locals.var_voxm1 * locals.var_voxm1_dn10)) / (2.0 * assign51060_e84923)), (((locals.var_voxm1_dn11 * locals.var_voxm1) + (locals.var_voxm1 * locals.var_voxm1_dn11)) / (2.0 * assign51060_e84923)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign51060_e84925;
        locals.var_t1_dn3 = assign51060_e84925_d_n3;
        locals.var_t1_dn4 = assign51060_e84925_d_n4;
        locals.var_t1_dn5 = assign51060_e84925_d_n5;
        locals.var_t1_dn6 = assign51060_e84925_d_n6;
        locals.var_t1_dn7 = assign51060_e84925_d_n7;
        locals.var_t1_dn8 = assign51060_e84925_d_n8;
        locals.var_t1_dn9 = assign51060_e84925_d_n9;
        locals.var_t1_dn10 = assign51060_e84925_d_n10;
        locals.var_t1_dn11 = assign51060_e84925_d_n11;

        let (assign51070_e84937, assign51070_e84937_d_n3, assign51070_e84937_d_n4, assign51070_e84937_d_n5, assign51070_e84937_d_n6, assign51070_e84937_d_n7, assign51070_e84937_d_n8, assign51070_e84937_d_n9, assign51070_e84937_d_n10, assign51070_e84937_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) {
        let assign51070_e84932: f64 = (-locals.var_voxm1);
        let assign51070_e84934: f64 = (assign51070_e84932 + locals.var_t1);
        let assign51070_e84935: f64 = (0.5 * assign51070_e84934);
        (assign51070_e84935, (0.5 * ((-locals.var_voxm1_dn3) + locals.var_t1_dn3)), (0.5 * ((-locals.var_voxm1_dn4) + locals.var_t1_dn4)), (0.5 * ((-locals.var_voxm1_dn5) + locals.var_t1_dn5)), (0.5 * ((-locals.var_voxm1_dn6) + locals.var_t1_dn6)), (0.5 * ((-locals.var_voxm1_dn7) + locals.var_t1_dn7)), (0.5 * ((-locals.var_voxm1_dn8) + locals.var_t1_dn8)), (0.5 * ((-locals.var_voxm1_dn9) + locals.var_t1_dn9)), (0.5 * ((-locals.var_voxm1_dn10) + locals.var_t1_dn10)), (0.5 * ((-locals.var_voxm1_dn11) + locals.var_t1_dn11)),)
    } else {
        (locals.var_voxmacc, locals.var_voxmacc_dn3, locals.var_voxmacc_dn4, locals.var_voxmacc_dn5, locals.var_voxmacc_dn6, locals.var_voxmacc_dn7, locals.var_voxmacc_dn8, locals.var_voxmacc_dn9, locals.var_voxmacc_dn10, locals.var_voxmacc_dn11,)
    }
};
        locals.var_voxmacc = assign51070_e84937;
        locals.var_voxmacc_dn3 = assign51070_e84937_d_n3;
        locals.var_voxmacc_dn4 = assign51070_e84937_d_n4;
        locals.var_voxmacc_dn5 = assign51070_e84937_d_n5;
        locals.var_voxmacc_dn6 = assign51070_e84937_d_n6;
        locals.var_voxmacc_dn7 = assign51070_e84937_d_n7;
        locals.var_voxmacc_dn8 = assign51070_e84937_d_n8;
        locals.var_voxmacc_dn9 = assign51070_e84937_d_n9;
        locals.var_voxmacc_dn10 = assign51070_e84937_d_n10;
        locals.var_voxmacc_dn11 = assign51070_e84937_d_n11;

        let (assign51080_e84948, assign51080_e84948_d_n3, assign51080_e84948_d_n4, assign51080_e84948_d_n5, assign51080_e84948_d_n6, assign51080_e84948_d_n7, assign51080_e84948_d_n8, assign51080_e84948_d_n9, assign51080_e84948_d_n10, assign51080_e84948_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) {
        let assign51080_e84945: f64 = (locals.var_voxm1 + locals.var_t1);
        let assign51080_e84946: f64 = (0.5 * assign51080_e84945);
        (assign51080_e84946, (0.5 * (locals.var_voxm1_dn3 + locals.var_t1_dn3)), (0.5 * (locals.var_voxm1_dn4 + locals.var_t1_dn4)), (0.5 * (locals.var_voxm1_dn5 + locals.var_t1_dn5)), (0.5 * (locals.var_voxm1_dn6 + locals.var_t1_dn6)), (0.5 * (locals.var_voxm1_dn7 + locals.var_t1_dn7)), (0.5 * (locals.var_voxm1_dn8 + locals.var_t1_dn8)), (0.5 * (locals.var_voxm1_dn9 + locals.var_t1_dn9)), (0.5 * (locals.var_voxm1_dn10 + locals.var_t1_dn10)), (0.5 * (locals.var_voxm1_dn11 + locals.var_t1_dn11)),)
    } else {
        (locals.var_voxminv, locals.var_voxminv_dn3, locals.var_voxminv_dn4, locals.var_voxminv_dn5, locals.var_voxminv_dn6, locals.var_voxminv_dn7, locals.var_voxminv_dn8, locals.var_voxminv_dn9, locals.var_voxminv_dn10, locals.var_voxminv_dn11,)
    }
};
        locals.var_voxminv = assign51080_e84948;
        locals.var_voxminv_dn3 = assign51080_e84948_d_n3;
        locals.var_voxminv_dn4 = assign51080_e84948_d_n4;
        locals.var_voxminv_dn5 = assign51080_e84948_d_n5;
        locals.var_voxminv_dn6 = assign51080_e84948_d_n6;
        locals.var_voxminv_dn7 = assign51080_e84948_d_n7;
        locals.var_voxminv_dn8 = assign51080_e84948_d_n8;
        locals.var_voxminv_dn9 = assign51080_e84948_d_n9;
        locals.var_voxminv_dn10 = assign51080_e84948_d_n10;
        locals.var_voxminv_dn11 = assign51080_e84948_d_n11;

        let assign51090_e84951: f64 = if p.p38 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard777 = assign51090_e84951;

        let (assign51100_e84962, assign51100_e84962_d_n3, assign51100_e84962_d_n4, assign51100_e84962_d_n5, assign51100_e84962_d_n6, assign51100_e84962_d_n7, assign51100_e84962_d_n8, assign51100_e84962_d_n9, assign51100_e84962_d_n10, assign51100_e84962_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 != 0.0)) {
        let assign51100_e84960: f64 = (locals.var_voxm1 / p.p671);
        (assign51100_e84960, (locals.var_voxm1_dn3 / p.p671), (locals.var_voxm1_dn4 / p.p671), (locals.var_voxm1_dn5 / p.p671), (locals.var_voxm1_dn6 / p.p671), (locals.var_voxm1_dn7 / p.p671), (locals.var_voxm1_dn8 / p.p671), (locals.var_voxm1_dn9 / p.p671), (locals.var_voxm1_dn10 / p.p671), (locals.var_voxm1_dn11 / p.p671),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign51100_e84962;
        locals.var_t1_dn3 = assign51100_e84962_d_n3;
        locals.var_t1_dn4 = assign51100_e84962_d_n4;
        locals.var_t1_dn5 = assign51100_e84962_d_n5;
        locals.var_t1_dn6 = assign51100_e84962_d_n6;
        locals.var_t1_dn7 = assign51100_e84962_d_n7;
        locals.var_t1_dn8 = assign51100_e84962_d_n8;
        locals.var_t1_dn9 = assign51100_e84962_d_n9;
        locals.var_t1_dn10 = assign51100_e84962_d_n10;
        locals.var_t1_dn11 = assign51100_e84962_d_n11;

        let (assign51110_e85014, assign51110_e85014_d_n3, assign51110_e85014_d_n4, assign51110_e85014_d_n5, assign51110_e85014_d_n6, assign51110_e85014_d_n7, assign51110_e85014_d_n8, assign51110_e85014_d_n9, assign51110_e85014_d_n10, assign51110_e85014_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 != 0.0)) {
        let assign51110_e84971: f64 = (-locals.var_t1);
        let assign51110_e84976: f64 = (-locals.var_t1);
        let assign51110_e84978: f64 = (-37.0);
        let (assign51110_e85011, assign51110_e85011_d_n3, assign51110_e85011_d_n4, assign51110_e85011_d_n5, assign51110_e85011_d_n6, assign51110_e85011_d_n7, assign51110_e85011_d_n8, assign51110_e85011_d_n9, assign51110_e85011_d_n10, assign51110_e85011_d_n11,) = {
            if ((!(assign51110_e84971 > 37.0)) && (!(assign51110_e84976 < assign51110_e84978))) {
                let assign51110_e84984: f64 = (-locals.var_t1);
                let assign51110_e84985: f64 = (assign51110_e84984).exp();
                let assign51110_e84986: f64 = (1.0 + assign51110_e84985);
                let assign51110_e84987: f64 = (assign51110_e84986).ln();
                (assign51110_e84987, ((assign51110_e84985 * (-locals.var_t1_dn3)) / assign51110_e84986), ((assign51110_e84985 * (-locals.var_t1_dn4)) / assign51110_e84986), ((assign51110_e84985 * (-locals.var_t1_dn5)) / assign51110_e84986), ((assign51110_e84985 * (-locals.var_t1_dn6)) / assign51110_e84986), ((assign51110_e84985 * (-locals.var_t1_dn7)) / assign51110_e84986), ((assign51110_e84985 * (-locals.var_t1_dn8)) / assign51110_e84986), ((assign51110_e84985 * (-locals.var_t1_dn9)) / assign51110_e84986), ((assign51110_e84985 * (-locals.var_t1_dn10)) / assign51110_e84986), ((assign51110_e84985 * (-locals.var_t1_dn11)) / assign51110_e84986),)
            } else {
                let assign51110_e84989: f64 = (-locals.var_t1);
                let assign51110_e84994: f64 = (-locals.var_t1);
                let assign51110_e84996: f64 = (-37.0);
                let (assign51110_e85010, assign51110_e85010_d_n3, assign51110_e85010_d_n4, assign51110_e85010_d_n5, assign51110_e85010_d_n6, assign51110_e85010_d_n7, assign51110_e85010_d_n8, assign51110_e85010_d_n9, assign51110_e85010_d_n10, assign51110_e85010_d_n11,) = {
                    if ((!(assign51110_e84989 > 37.0)) && (assign51110_e84994 < assign51110_e84996)) {
                        let assign51110_e85000: f64 = (-locals.var_t1);
                        let assign51110_e85001: f64 = (assign51110_e85000).exp();
                        (assign51110_e85001, (assign51110_e85001 * (-locals.var_t1_dn3)), (assign51110_e85001 * (-locals.var_t1_dn4)), (assign51110_e85001 * (-locals.var_t1_dn5)), (assign51110_e85001 * (-locals.var_t1_dn6)), (assign51110_e85001 * (-locals.var_t1_dn7)), (assign51110_e85001 * (-locals.var_t1_dn8)), (assign51110_e85001 * (-locals.var_t1_dn9)), (assign51110_e85001 * (-locals.var_t1_dn10)), (assign51110_e85001 * (-locals.var_t1_dn11)),)
                    } else {
                        let assign51110_e85003: f64 = (-locals.var_t1);
                        let (assign51110_e85009, assign51110_e85009_d_n3, assign51110_e85009_d_n4, assign51110_e85009_d_n5, assign51110_e85009_d_n6, assign51110_e85009_d_n7, assign51110_e85009_d_n8, assign51110_e85009_d_n9, assign51110_e85009_d_n10, assign51110_e85009_d_n11,) = {
                            if (assign51110_e85003 > 37.0) {
                                let assign51110_e85007: f64 = (-locals.var_t1);
                                (assign51110_e85007, (-locals.var_t1_dn3), (-locals.var_t1_dn4), (-locals.var_t1_dn5), (-locals.var_t1_dn6), (-locals.var_t1_dn7), (-locals.var_t1_dn8), (-locals.var_t1_dn9), (-locals.var_t1_dn10), (-locals.var_t1_dn11),)
                            } else {
                                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign51110_e85009, assign51110_e85009_d_n3, assign51110_e85009_d_n4, assign51110_e85009_d_n5, assign51110_e85009_d_n6, assign51110_e85009_d_n7, assign51110_e85009_d_n8, assign51110_e85009_d_n9, assign51110_e85009_d_n10, assign51110_e85009_d_n11,)
                    }
                };
                (assign51110_e85010, assign51110_e85010_d_n3, assign51110_e85010_d_n4, assign51110_e85010_d_n5, assign51110_e85010_d_n6, assign51110_e85010_d_n7, assign51110_e85010_d_n8, assign51110_e85010_d_n9, assign51110_e85010_d_n10, assign51110_e85010_d_n11,)
            }
        };
        let assign51110_e85012: f64 = (p.p671 * assign51110_e85011);
        (assign51110_e85012, (p.p671 * assign51110_e85011_d_n3), (p.p671 * assign51110_e85011_d_n4), (p.p671 * assign51110_e85011_d_n5), (p.p671 * assign51110_e85011_d_n6), (p.p671 * assign51110_e85011_d_n7), (p.p671 * assign51110_e85011_d_n8), (p.p671 * assign51110_e85011_d_n9), (p.p671 * assign51110_e85011_d_n10), (p.p671 * assign51110_e85011_d_n11),)
    } else {
        (locals.var_vaux_igbacc, locals.var_vaux_igbacc_dn3, locals.var_vaux_igbacc_dn4, locals.var_vaux_igbacc_dn5, locals.var_vaux_igbacc_dn6, locals.var_vaux_igbacc_dn7, locals.var_vaux_igbacc_dn8, locals.var_vaux_igbacc_dn9, locals.var_vaux_igbacc_dn10, locals.var_vaux_igbacc_dn11,)
    }
};
        locals.var_vaux_igbacc = assign51110_e85014;
        locals.var_vaux_igbacc_dn3 = assign51110_e85014_d_n3;
        locals.var_vaux_igbacc_dn4 = assign51110_e85014_d_n4;
        locals.var_vaux_igbacc_dn5 = assign51110_e85014_d_n5;
        locals.var_vaux_igbacc_dn6 = assign51110_e85014_d_n6;
        locals.var_vaux_igbacc_dn7 = assign51110_e85014_d_n7;
        locals.var_vaux_igbacc_dn8 = assign51110_e85014_d_n8;
        locals.var_vaux_igbacc_dn9 = assign51110_e85014_d_n9;
        locals.var_vaux_igbacc_dn10 = assign51110_e85014_d_n10;
        locals.var_vaux_igbacc_dn11 = assign51110_e85014_d_n11;

        let assign51120_e85017: f64 = if p.p696 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard778 = assign51120_e85017;

        let (assign51130_e85032, assign51130_e85032_d_n3, assign51130_e85032_d_n4, assign51130_e85032_d_n5, assign51130_e85032_d_n6, assign51130_e85032_d_n7, assign51130_e85032_d_n8, assign51130_e85032_d_n9, assign51130_e85032_d_n10, assign51130_e85032_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 != 0.0)) && (locals.var_guard778 != 0.0)) {
        let assign51130_e85029: f64 = (locals.var_voxmacc / p.p696);
        let assign51130_e85030: f64 = (1.0 - assign51130_e85029);
        (assign51130_e85030, (-(locals.var_voxmacc_dn3 / p.p696)), (-(locals.var_voxmacc_dn4 / p.p696)), (-(locals.var_voxmacc_dn5 / p.p696)), (-(locals.var_voxmacc_dn6 / p.p696)), (-(locals.var_voxmacc_dn7 / p.p696)), (-(locals.var_voxmacc_dn8 / p.p696)), (-(locals.var_voxmacc_dn9 / p.p696)), (-(locals.var_voxmacc_dn10 / p.p696)), (-(locals.var_voxmacc_dn11 / p.p696)),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign51130_e85032;
        locals.var_t0_dn3 = assign51130_e85032_d_n3;
        locals.var_t0_dn4 = assign51130_e85032_d_n4;
        locals.var_t0_dn5 = assign51130_e85032_d_n5;
        locals.var_t0_dn6 = assign51130_e85032_d_n6;
        locals.var_t0_dn7 = assign51130_e85032_d_n7;
        locals.var_t0_dn8 = assign51130_e85032_d_n8;
        locals.var_t0_dn9 = assign51130_e85032_d_n9;
        locals.var_t0_dn10 = assign51130_e85032_d_n10;
        locals.var_t0_dn11 = assign51130_e85032_d_n11;

        let (assign51140_e85044, assign51140_e85044_d_n3, assign51140_e85044_d_n4, assign51140_e85044_d_n5, assign51140_e85044_d_n6, assign51140_e85044_d_n7, assign51140_e85044_d_n8, assign51140_e85044_d_n9, assign51140_e85044_d_n10, assign51140_e85044_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 != 0.0)) && (locals.var_guard778 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign51140_e85044;
        locals.var_t0_dn3 = assign51140_e85044_d_n3;
        locals.var_t0_dn4 = assign51140_e85044_d_n4;
        locals.var_t0_dn5 = assign51140_e85044_d_n5;
        locals.var_t0_dn6 = assign51140_e85044_d_n6;
        locals.var_t0_dn7 = assign51140_e85044_d_n7;
        locals.var_t0_dn8 = assign51140_e85044_d_n8;
        locals.var_t0_dn9 = assign51140_e85044_d_n9;
        locals.var_t0_dn10 = assign51140_e85044_d_n10;
        locals.var_t0_dn11 = assign51140_e85044_d_n11;

        let assign51150_e85047: f64 = if locals.var_t0 < 0.01 { 1.0 } else { 0.0 };
        locals.var_guard779 = assign51150_e85047;

        let (assign51160_e85058, assign51160_e85058_d_n3, assign51160_e85058_d_n4, assign51160_e85058_d_n5, assign51160_e85058_d_n6, assign51160_e85058_d_n7, assign51160_e85058_d_n8, assign51160_e85058_d_n9, assign51160_e85058_d_n10, assign51160_e85058_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 != 0.0)) && (locals.var_guard779 != 0.0)) {
        (0.01, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign51160_e85058;
        locals.var_t0_dn3 = assign51160_e85058_d_n3;
        locals.var_t0_dn4 = assign51160_e85058_d_n4;
        locals.var_t0_dn5 = assign51160_e85058_d_n5;
        locals.var_t0_dn6 = assign51160_e85058_d_n6;
        locals.var_t0_dn7 = assign51160_e85058_d_n7;
        locals.var_t0_dn8 = assign51160_e85058_d_n8;
        locals.var_t0_dn9 = assign51160_e85058_d_n9;
        locals.var_t0_dn10 = assign51160_e85058_d_n10;
        locals.var_t0_dn11 = assign51160_e85058_d_n11;

        let (assign51170_e85079, assign51170_e85079_d_n3, assign51170_e85079_d_n4, assign51170_e85079_d_n5, assign51170_e85079_d_n6, assign51170_e85079_d_n7, assign51170_e85079_d_n8, assign51170_e85079_d_n9, assign51170_e85079_d_n10, assign51170_e85079_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 != 0.0)) {
        let assign51170_e85067: f64 = (locals.var_leff * locals.var_weff);
        let assign51170_e85069: f64 = (assign51170_e85067 / p.p1373);
        let assign51170_e85072: f64 = (p.p1381 / p.p2);
        let assign51170_e85073: f64 = (assign51170_e85069 + assign51170_e85072);
        let assign51170_e85075: f64 = (assign51170_e85073 * p.p700);
        let assign51170_e85077: f64 = (assign51170_e85075 * locals.var_toxratio);
        (assign51170_e85077, (assign51170_e85075 * locals.var_toxratio_dn3), (assign51170_e85075 * locals.var_toxratio_dn4), (assign51170_e85075 * locals.var_toxratio_dn5), (assign51170_e85075 * locals.var_toxratio_dn6), (assign51170_e85075 * locals.var_toxratio_dn7), (assign51170_e85075 * locals.var_toxratio_dn8), (assign51170_e85075 * locals.var_toxratio_dn9), (assign51170_e85075 * locals.var_toxratio_dn10), (assign51170_e85075 * locals.var_toxratio_dn11),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign51170_e85079;
        locals.var_t1_dn3 = assign51170_e85079_d_n3;
        locals.var_t1_dn4 = assign51170_e85079_d_n4;
        locals.var_t1_dn5 = assign51170_e85079_d_n5;
        locals.var_t1_dn6 = assign51170_e85079_d_n6;
        locals.var_t1_dn7 = assign51170_e85079_d_n7;
        locals.var_t1_dn8 = assign51170_e85079_d_n8;
        locals.var_t1_dn9 = assign51170_e85079_d_n9;
        locals.var_t1_dn10 = assign51170_e85079_d_n10;
        locals.var_t1_dn11 = assign51170_e85079_d_n11;

        let (assign51180_e85090, assign51180_e85090_d_n3, assign51180_e85090_d_n4, assign51180_e85090_d_n5, assign51180_e85090_d_n6, assign51180_e85090_d_n7, assign51180_e85090_d_n8, assign51180_e85090_d_n9, assign51180_e85090_d_n10, assign51180_e85090_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 != 0.0)) {
        let assign51180_e85088: f64 = (p.p701 * p.p76);
        (assign51180_e85088, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign51180_e85090;
        locals.var_t2_dn3 = assign51180_e85090_d_n3;
        locals.var_t2_dn4 = assign51180_e85090_d_n4;
        locals.var_t2_dn5 = assign51180_e85090_d_n5;
        locals.var_t2_dn6 = assign51180_e85090_d_n6;
        locals.var_t2_dn7 = assign51180_e85090_d_n7;
        locals.var_t2_dn8 = assign51180_e85090_d_n8;
        locals.var_t2_dn9 = assign51180_e85090_d_n9;
        locals.var_t2_dn10 = assign51180_e85090_d_n10;
        locals.var_t2_dn11 = assign51180_e85090_d_n11;

        let (assign51190_e85107, assign51190_e85107_d_n3, assign51190_e85107_d_n4, assign51190_e85107_d_n5, assign51190_e85107_d_n6, assign51190_e85107_d_n7, assign51190_e85107_d_n8, assign51190_e85107_d_n9, assign51190_e85107_d_n10, assign51190_e85107_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 != 0.0)) {
        let assign51190_e85101: f64 = (locals.var_betagb2_i * locals.var_voxmacc);
        let assign51190_e85102: f64 = (locals.var_alphagb2_i - assign51190_e85101);
        let assign51190_e85103: f64 = (locals.var_t2 * assign51190_e85102);
        let assign51190_e85105: f64 = (assign51190_e85103 / locals.var_t0);
        (assign51190_e85105, (((((locals.var_t2_dn3 * assign51190_e85102) + (locals.var_t2 * (-(locals.var_betagb2_i * locals.var_voxmacc_dn3)))) * locals.var_t0) - (assign51190_e85103 * locals.var_t0_dn3)) / (locals.var_t0 * locals.var_t0)), (((((locals.var_t2_dn4 * assign51190_e85102) + (locals.var_t2 * (locals.var_alphagb2_i_dn4 - (locals.var_betagb2_i * locals.var_voxmacc_dn4)))) * locals.var_t0) - (assign51190_e85103 * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)), (((((locals.var_t2_dn5 * assign51190_e85102) + (locals.var_t2 * (locals.var_alphagb2_i_dn5 - (locals.var_betagb2_i * locals.var_voxmacc_dn5)))) * locals.var_t0) - (assign51190_e85103 * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)), (((((locals.var_t2_dn6 * assign51190_e85102) + (locals.var_t2 * (-(locals.var_betagb2_i * locals.var_voxmacc_dn6)))) * locals.var_t0) - (assign51190_e85103 * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)), (((((locals.var_t2_dn7 * assign51190_e85102) + (locals.var_t2 * (-(locals.var_betagb2_i * locals.var_voxmacc_dn7)))) * locals.var_t0) - (assign51190_e85103 * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)), (((((locals.var_t2_dn8 * assign51190_e85102) + (locals.var_t2 * (-(locals.var_betagb2_i * locals.var_voxmacc_dn8)))) * locals.var_t0) - (assign51190_e85103 * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)), (((((locals.var_t2_dn9 * assign51190_e85102) + (locals.var_t2 * (-(locals.var_betagb2_i * locals.var_voxmacc_dn9)))) * locals.var_t0) - (assign51190_e85103 * locals.var_t0_dn9)) / (locals.var_t0 * locals.var_t0)), (((((locals.var_t2_dn10 * assign51190_e85102) + (locals.var_t2 * (-(locals.var_betagb2_i * locals.var_voxmacc_dn10)))) * locals.var_t0) - (assign51190_e85103 * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)), (((((locals.var_t2_dn11 * assign51190_e85102) + (locals.var_t2 * (-(locals.var_betagb2_i * locals.var_voxmacc_dn11)))) * locals.var_t0) - (assign51190_e85103 * locals.var_t0_dn11)) / (locals.var_t0 * locals.var_t0)),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign51190_e85107;
        locals.var_t3_dn3 = assign51190_e85107_d_n3;
        locals.var_t3_dn4 = assign51190_e85107_d_n4;
        locals.var_t3_dn5 = assign51190_e85107_d_n5;
        locals.var_t3_dn6 = assign51190_e85107_d_n6;
        locals.var_t3_dn7 = assign51190_e85107_d_n7;
        locals.var_t3_dn8 = assign51190_e85107_d_n8;
        locals.var_t3_dn9 = assign51190_e85107_d_n9;
        locals.var_t3_dn10 = assign51190_e85107_d_n10;
        locals.var_t3_dn11 = assign51190_e85107_d_n11;

        let (assign51200_e85117, assign51200_e85117_d_n3, assign51200_e85117_d_n4, assign51200_e85117_d_n5, assign51200_e85117_d_n6, assign51200_e85117_d_n7, assign51200_e85117_d_n8, assign51200_e85117_d_n9, assign51200_e85117_d_n10, assign51200_e85117_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 != 0.0)) {
        let assign51200_e85115: f64 = { let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign51200_e85115, ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn3), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn4), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn5), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn6), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn7), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn8), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn9), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn10), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn11),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign51200_e85117;
        locals.var_t4_dn3 = assign51200_e85117_d_n3;
        locals.var_t4_dn4 = assign51200_e85117_d_n4;
        locals.var_t4_dn5 = assign51200_e85117_d_n5;
        locals.var_t4_dn6 = assign51200_e85117_d_n6;
        locals.var_t4_dn7 = assign51200_e85117_d_n7;
        locals.var_t4_dn8 = assign51200_e85117_d_n8;
        locals.var_t4_dn9 = assign51200_e85117_d_n9;
        locals.var_t4_dn10 = assign51200_e85117_d_n10;
        locals.var_t4_dn11 = assign51200_e85117_d_n11;

        let (assign51210_e85132, assign51210_e85132_d_n3, assign51210_e85132_d_n4, assign51210_e85132_d_n5, assign51210_e85132_d_n6, assign51210_e85132_d_n7, assign51210_e85132_d_n8, assign51210_e85132_d_n9, assign51210_e85132_d_n10, assign51210_e85132_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 != 0.0)) {
        let assign51210_e85126: f64 = (locals.var_t1 * locals.var_vg);
        let assign51210_e85128: f64 = (assign51210_e85126 * locals.var_vaux_igbacc);
        let assign51210_e85130: f64 = (assign51210_e85128 * locals.var_t4);
        (assign51210_e85130, (((((locals.var_t1_dn3 * locals.var_vg) * locals.var_vaux_igbacc) + (assign51210_e85126 * locals.var_vaux_igbacc_dn3)) * locals.var_t4) + (assign51210_e85128 * locals.var_t4_dn3)), (((((locals.var_t1_dn4 * locals.var_vg) * locals.var_vaux_igbacc) + (assign51210_e85126 * locals.var_vaux_igbacc_dn4)) * locals.var_t4) + (assign51210_e85128 * locals.var_t4_dn4)), (((((locals.var_t1_dn5 * locals.var_vg) * locals.var_vaux_igbacc) + (assign51210_e85126 * locals.var_vaux_igbacc_dn5)) * locals.var_t4) + (assign51210_e85128 * locals.var_t4_dn5)), (((((locals.var_t1_dn6 * locals.var_vg) * locals.var_vaux_igbacc) + (assign51210_e85126 * locals.var_vaux_igbacc_dn6)) * locals.var_t4) + (assign51210_e85128 * locals.var_t4_dn6)), (((((locals.var_t1_dn7 * locals.var_vg) * locals.var_vaux_igbacc) + (assign51210_e85126 * locals.var_vaux_igbacc_dn7)) * locals.var_t4) + (assign51210_e85128 * locals.var_t4_dn7)), ((((((locals.var_t1_dn8 * locals.var_vg) + (locals.var_t1 * locals.var_vg_dn8)) * locals.var_vaux_igbacc) + (assign51210_e85126 * locals.var_vaux_igbacc_dn8)) * locals.var_t4) + (assign51210_e85128 * locals.var_t4_dn8)), (((((locals.var_t1_dn9 * locals.var_vg) * locals.var_vaux_igbacc) + (assign51210_e85126 * locals.var_vaux_igbacc_dn9)) * locals.var_t4) + (assign51210_e85128 * locals.var_t4_dn9)), ((((((locals.var_t1_dn10 * locals.var_vg) + (locals.var_t1 * locals.var_vg_dn10)) * locals.var_vaux_igbacc) + (assign51210_e85126 * locals.var_vaux_igbacc_dn10)) * locals.var_t4) + (assign51210_e85128 * locals.var_t4_dn10)), (((((locals.var_t1_dn11 * locals.var_vg) * locals.var_vaux_igbacc) + (assign51210_e85126 * locals.var_vaux_igbacc_dn11)) * locals.var_t4) + (assign51210_e85128 * locals.var_t4_dn11)),)
    } else {
        (locals.var_igbacc, locals.var_igbacc_dn3, locals.var_igbacc_dn4, locals.var_igbacc_dn5, locals.var_igbacc_dn6, locals.var_igbacc_dn7, locals.var_igbacc_dn8, locals.var_igbacc_dn9, locals.var_igbacc_dn10, locals.var_igbacc_dn11,)
    }
};
        locals.var_igbacc = assign51210_e85132;
        locals.var_igbacc_dn3 = assign51210_e85132_d_n3;
        locals.var_igbacc_dn4 = assign51210_e85132_d_n4;
        locals.var_igbacc_dn5 = assign51210_e85132_d_n5;
        locals.var_igbacc_dn6 = assign51210_e85132_d_n6;
        locals.var_igbacc_dn7 = assign51210_e85132_d_n7;
        locals.var_igbacc_dn8 = assign51210_e85132_d_n8;
        locals.var_igbacc_dn9 = assign51210_e85132_d_n9;
        locals.var_igbacc_dn10 = assign51210_e85132_d_n10;
        locals.var_igbacc_dn11 = assign51210_e85132_d_n11;

        let (assign51220_e85143, assign51220_e85143_d_n3, assign51220_e85143_d_n4, assign51220_e85143_d_n5, assign51220_e85143_d_n6, assign51220_e85143_d_n7, assign51220_e85143_d_n8, assign51220_e85143_d_n9, assign51220_e85143_d_n10, assign51220_e85143_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 != 0.0)) {
        let assign51220_e85141: f64 = (locals.var_igbacc * locals.var_igtemp);
        (assign51220_e85141, (locals.var_igbacc_dn3 * locals.var_igtemp), ((locals.var_igbacc_dn4 * locals.var_igtemp) + (locals.var_igbacc * locals.var_igtemp_dn4)), ((locals.var_igbacc_dn5 * locals.var_igtemp) + (locals.var_igbacc * locals.var_igtemp_dn5)), (locals.var_igbacc_dn6 * locals.var_igtemp), (locals.var_igbacc_dn7 * locals.var_igtemp), (locals.var_igbacc_dn8 * locals.var_igtemp), (locals.var_igbacc_dn9 * locals.var_igtemp), (locals.var_igbacc_dn10 * locals.var_igtemp), (locals.var_igbacc_dn11 * locals.var_igtemp),)
    } else {
        (locals.var_igbacc, locals.var_igbacc_dn3, locals.var_igbacc_dn4, locals.var_igbacc_dn5, locals.var_igbacc_dn6, locals.var_igbacc_dn7, locals.var_igbacc_dn8, locals.var_igbacc_dn9, locals.var_igbacc_dn10, locals.var_igbacc_dn11,)
    }
};
        locals.var_igbacc = assign51220_e85143;
        locals.var_igbacc_dn3 = assign51220_e85143_d_n3;
        locals.var_igbacc_dn4 = assign51220_e85143_d_n4;
        locals.var_igbacc_dn5 = assign51220_e85143_d_n5;
        locals.var_igbacc_dn6 = assign51220_e85143_d_n6;
        locals.var_igbacc_dn7 = assign51220_e85143_d_n7;
        locals.var_igbacc_dn8 = assign51220_e85143_d_n8;
        locals.var_igbacc_dn9 = assign51220_e85143_d_n9;
        locals.var_igbacc_dn10 = assign51220_e85143_d_n10;
        locals.var_igbacc_dn11 = assign51220_e85143_d_n11;

        let (assign51230_e85156, assign51230_e85156_d_n3, assign51230_e85156_d_n4, assign51230_e85156_d_n5, assign51230_e85156_d_n6, assign51230_e85156_d_n7, assign51230_e85156_d_n8, assign51230_e85156_d_n9, assign51230_e85156_d_n10, assign51230_e85156_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 != 0.0)) {
        let assign51230_e85152: f64 = (locals.var_voxm1 - locals.var_eigbinv_i);
        let assign51230_e85154: f64 = (assign51230_e85152 / p.p671);
        (assign51230_e85154, (locals.var_voxm1_dn3 / p.p671), (locals.var_voxm1_dn4 / p.p671), (locals.var_voxm1_dn5 / p.p671), (locals.var_voxm1_dn6 / p.p671), (locals.var_voxm1_dn7 / p.p671), (locals.var_voxm1_dn8 / p.p671), (locals.var_voxm1_dn9 / p.p671), (locals.var_voxm1_dn10 / p.p671), (locals.var_voxm1_dn11 / p.p671),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign51230_e85156;
        locals.var_t1_dn3 = assign51230_e85156_d_n3;
        locals.var_t1_dn4 = assign51230_e85156_d_n4;
        locals.var_t1_dn5 = assign51230_e85156_d_n5;
        locals.var_t1_dn6 = assign51230_e85156_d_n6;
        locals.var_t1_dn7 = assign51230_e85156_d_n7;
        locals.var_t1_dn8 = assign51230_e85156_d_n8;
        locals.var_t1_dn9 = assign51230_e85156_d_n9;
        locals.var_t1_dn10 = assign51230_e85156_d_n10;
        locals.var_t1_dn11 = assign51230_e85156_d_n11;

    }

    pub(super) fn stamp_transient_block_174(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let (assign51240_e85200, assign51240_e85200_d_n3, assign51240_e85200_d_n4, assign51240_e85200_d_n5, assign51240_e85200_d_n6, assign51240_e85200_d_n7, assign51240_e85200_d_n8, assign51240_e85200_d_n9, assign51240_e85200_d_n10, assign51240_e85200_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 != 0.0)) {
        let assign51240_e85170: f64 = (-37.0);
        let (assign51240_e85197, assign51240_e85197_d_n3, assign51240_e85197_d_n4, assign51240_e85197_d_n5, assign51240_e85197_d_n6, assign51240_e85197_d_n7, assign51240_e85197_d_n8, assign51240_e85197_d_n9, assign51240_e85197_d_n10, assign51240_e85197_d_n11,) = {
            if ((!(locals.var_t1 > 37.0)) && (!(locals.var_t1 < assign51240_e85170))) {
                let assign51240_e85176: f64 = (locals.var_t1).exp();
                let assign51240_e85177: f64 = (1.0 + assign51240_e85176);
                let assign51240_e85178: f64 = (assign51240_e85177).ln();
                (assign51240_e85178, ((assign51240_e85176 * locals.var_t1_dn3) / assign51240_e85177), ((assign51240_e85176 * locals.var_t1_dn4) / assign51240_e85177), ((assign51240_e85176 * locals.var_t1_dn5) / assign51240_e85177), ((assign51240_e85176 * locals.var_t1_dn6) / assign51240_e85177), ((assign51240_e85176 * locals.var_t1_dn7) / assign51240_e85177), ((assign51240_e85176 * locals.var_t1_dn8) / assign51240_e85177), ((assign51240_e85176 * locals.var_t1_dn9) / assign51240_e85177), ((assign51240_e85176 * locals.var_t1_dn10) / assign51240_e85177), ((assign51240_e85176 * locals.var_t1_dn11) / assign51240_e85177),)
            } else {
                let assign51240_e85185: f64 = (-37.0);
                let (assign51240_e85196, assign51240_e85196_d_n3, assign51240_e85196_d_n4, assign51240_e85196_d_n5, assign51240_e85196_d_n6, assign51240_e85196_d_n7, assign51240_e85196_d_n8, assign51240_e85196_d_n9, assign51240_e85196_d_n10, assign51240_e85196_d_n11,) = {
                    if ((!(locals.var_t1 > 37.0)) && (locals.var_t1 < assign51240_e85185)) {
                        let assign51240_e85189: f64 = (locals.var_t1).exp();
                        (assign51240_e85189, (assign51240_e85189 * locals.var_t1_dn3), (assign51240_e85189 * locals.var_t1_dn4), (assign51240_e85189 * locals.var_t1_dn5), (assign51240_e85189 * locals.var_t1_dn6), (assign51240_e85189 * locals.var_t1_dn7), (assign51240_e85189 * locals.var_t1_dn8), (assign51240_e85189 * locals.var_t1_dn9), (assign51240_e85189 * locals.var_t1_dn10), (assign51240_e85189 * locals.var_t1_dn11),)
                    } else {
                        let (assign51240_e85195, assign51240_e85195_d_n3, assign51240_e85195_d_n4, assign51240_e85195_d_n5, assign51240_e85195_d_n6, assign51240_e85195_d_n7, assign51240_e85195_d_n8, assign51240_e85195_d_n9, assign51240_e85195_d_n10, assign51240_e85195_d_n11,) = {
                            if (locals.var_t1 > 37.0) {
                                (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
                            } else {
                                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign51240_e85195, assign51240_e85195_d_n3, assign51240_e85195_d_n4, assign51240_e85195_d_n5, assign51240_e85195_d_n6, assign51240_e85195_d_n7, assign51240_e85195_d_n8, assign51240_e85195_d_n9, assign51240_e85195_d_n10, assign51240_e85195_d_n11,)
                    }
                };
                (assign51240_e85196, assign51240_e85196_d_n3, assign51240_e85196_d_n4, assign51240_e85196_d_n5, assign51240_e85196_d_n6, assign51240_e85196_d_n7, assign51240_e85196_d_n8, assign51240_e85196_d_n9, assign51240_e85196_d_n10, assign51240_e85196_d_n11,)
            }
        };
        let assign51240_e85198: f64 = (p.p671 * assign51240_e85197);
        (assign51240_e85198, (p.p671 * assign51240_e85197_d_n3), (p.p671 * assign51240_e85197_d_n4), (p.p671 * assign51240_e85197_d_n5), (p.p671 * assign51240_e85197_d_n6), (p.p671 * assign51240_e85197_d_n7), (p.p671 * assign51240_e85197_d_n8), (p.p671 * assign51240_e85197_d_n9), (p.p671 * assign51240_e85197_d_n10), (p.p671 * assign51240_e85197_d_n11),)
    } else {
        (locals.var_vaux_igbinv, locals.var_vaux_igbinv_dn3, locals.var_vaux_igbinv_dn4, locals.var_vaux_igbinv_dn5, locals.var_vaux_igbinv_dn6, locals.var_vaux_igbinv_dn7, locals.var_vaux_igbinv_dn8, locals.var_vaux_igbinv_dn9, locals.var_vaux_igbinv_dn10, locals.var_vaux_igbinv_dn11,)
    }
};
        locals.var_vaux_igbinv = assign51240_e85200;
        locals.var_vaux_igbinv_dn3 = assign51240_e85200_d_n3;
        locals.var_vaux_igbinv_dn4 = assign51240_e85200_d_n4;
        locals.var_vaux_igbinv_dn5 = assign51240_e85200_d_n5;
        locals.var_vaux_igbinv_dn6 = assign51240_e85200_d_n6;
        locals.var_vaux_igbinv_dn7 = assign51240_e85200_d_n7;
        locals.var_vaux_igbinv_dn8 = assign51240_e85200_d_n8;
        locals.var_vaux_igbinv_dn9 = assign51240_e85200_d_n9;
        locals.var_vaux_igbinv_dn10 = assign51240_e85200_d_n10;
        locals.var_vaux_igbinv_dn11 = assign51240_e85200_d_n11;

        let assign51250_e85203: f64 = if p.p697 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard780 = assign51250_e85203;

        let (assign51260_e85218, assign51260_e85218_d_n3, assign51260_e85218_d_n4, assign51260_e85218_d_n5, assign51260_e85218_d_n6, assign51260_e85218_d_n7, assign51260_e85218_d_n8, assign51260_e85218_d_n9, assign51260_e85218_d_n10, assign51260_e85218_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 != 0.0)) && (locals.var_guard780 != 0.0)) {
        let assign51260_e85215: f64 = (locals.var_voxminv / p.p697);
        let assign51260_e85216: f64 = (1.0 - assign51260_e85215);
        (assign51260_e85216, (-(locals.var_voxminv_dn3 / p.p697)), (-(locals.var_voxminv_dn4 / p.p697)), (-(locals.var_voxminv_dn5 / p.p697)), (-(locals.var_voxminv_dn6 / p.p697)), (-(locals.var_voxminv_dn7 / p.p697)), (-(locals.var_voxminv_dn8 / p.p697)), (-(locals.var_voxminv_dn9 / p.p697)), (-(locals.var_voxminv_dn10 / p.p697)), (-(locals.var_voxminv_dn11 / p.p697)),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign51260_e85218;
        locals.var_t0_dn3 = assign51260_e85218_d_n3;
        locals.var_t0_dn4 = assign51260_e85218_d_n4;
        locals.var_t0_dn5 = assign51260_e85218_d_n5;
        locals.var_t0_dn6 = assign51260_e85218_d_n6;
        locals.var_t0_dn7 = assign51260_e85218_d_n7;
        locals.var_t0_dn8 = assign51260_e85218_d_n8;
        locals.var_t0_dn9 = assign51260_e85218_d_n9;
        locals.var_t0_dn10 = assign51260_e85218_d_n10;
        locals.var_t0_dn11 = assign51260_e85218_d_n11;

        let (assign51270_e85230, assign51270_e85230_d_n3, assign51270_e85230_d_n4, assign51270_e85230_d_n5, assign51270_e85230_d_n6, assign51270_e85230_d_n7, assign51270_e85230_d_n8, assign51270_e85230_d_n9, assign51270_e85230_d_n10, assign51270_e85230_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 != 0.0)) && (locals.var_guard780 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign51270_e85230;
        locals.var_t0_dn3 = assign51270_e85230_d_n3;
        locals.var_t0_dn4 = assign51270_e85230_d_n4;
        locals.var_t0_dn5 = assign51270_e85230_d_n5;
        locals.var_t0_dn6 = assign51270_e85230_d_n6;
        locals.var_t0_dn7 = assign51270_e85230_d_n7;
        locals.var_t0_dn8 = assign51270_e85230_d_n8;
        locals.var_t0_dn9 = assign51270_e85230_d_n9;
        locals.var_t0_dn10 = assign51270_e85230_d_n10;
        locals.var_t0_dn11 = assign51270_e85230_d_n11;

        let assign51280_e85233: f64 = if locals.var_t0 < 0.01 { 1.0 } else { 0.0 };
        locals.var_guard781 = assign51280_e85233;

        let (assign51290_e85244, assign51290_e85244_d_n3, assign51290_e85244_d_n4, assign51290_e85244_d_n5, assign51290_e85244_d_n6, assign51290_e85244_d_n7, assign51290_e85244_d_n8, assign51290_e85244_d_n9, assign51290_e85244_d_n10, assign51290_e85244_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 != 0.0)) && (locals.var_guard781 != 0.0)) {
        (0.01, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign51290_e85244;
        locals.var_t0_dn3 = assign51290_e85244_d_n3;
        locals.var_t0_dn4 = assign51290_e85244_d_n4;
        locals.var_t0_dn5 = assign51290_e85244_d_n5;
        locals.var_t0_dn6 = assign51290_e85244_d_n6;
        locals.var_t0_dn7 = assign51290_e85244_d_n7;
        locals.var_t0_dn8 = assign51290_e85244_d_n8;
        locals.var_t0_dn9 = assign51290_e85244_d_n9;
        locals.var_t0_dn10 = assign51290_e85244_d_n10;
        locals.var_t0_dn11 = assign51290_e85244_d_n11;

        let (assign51300_e85265, assign51300_e85265_d_n3, assign51300_e85265_d_n4, assign51300_e85265_d_n5, assign51300_e85265_d_n6, assign51300_e85265_d_n7, assign51300_e85265_d_n8, assign51300_e85265_d_n9, assign51300_e85265_d_n10, assign51300_e85265_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 != 0.0)) {
        let assign51300_e85253: f64 = (locals.var_leff * locals.var_weff);
        let assign51300_e85255: f64 = (assign51300_e85253 / p.p1373);
        let assign51300_e85258: f64 = (p.p1381 / p.p2);
        let assign51300_e85259: f64 = (assign51300_e85255 + assign51300_e85258);
        let assign51300_e85261: f64 = (assign51300_e85259 * p.p698);
        let assign51300_e85263: f64 = (assign51300_e85261 * locals.var_toxratio);
        (assign51300_e85263, (assign51300_e85261 * locals.var_toxratio_dn3), (assign51300_e85261 * locals.var_toxratio_dn4), (assign51300_e85261 * locals.var_toxratio_dn5), (assign51300_e85261 * locals.var_toxratio_dn6), (assign51300_e85261 * locals.var_toxratio_dn7), (assign51300_e85261 * locals.var_toxratio_dn8), (assign51300_e85261 * locals.var_toxratio_dn9), (assign51300_e85261 * locals.var_toxratio_dn10), (assign51300_e85261 * locals.var_toxratio_dn11),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign51300_e85265;
        locals.var_t1_dn3 = assign51300_e85265_d_n3;
        locals.var_t1_dn4 = assign51300_e85265_d_n4;
        locals.var_t1_dn5 = assign51300_e85265_d_n5;
        locals.var_t1_dn6 = assign51300_e85265_d_n6;
        locals.var_t1_dn7 = assign51300_e85265_d_n7;
        locals.var_t1_dn8 = assign51300_e85265_d_n8;
        locals.var_t1_dn9 = assign51300_e85265_d_n9;
        locals.var_t1_dn10 = assign51300_e85265_d_n10;
        locals.var_t1_dn11 = assign51300_e85265_d_n11;

        let (assign51310_e85276, assign51310_e85276_d_n3, assign51310_e85276_d_n4, assign51310_e85276_d_n5, assign51310_e85276_d_n6, assign51310_e85276_d_n7, assign51310_e85276_d_n8, assign51310_e85276_d_n9, assign51310_e85276_d_n10, assign51310_e85276_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 != 0.0)) {
        let assign51310_e85274: f64 = (p.p699 * p.p76);
        (assign51310_e85274, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign51310_e85276;
        locals.var_t2_dn3 = assign51310_e85276_d_n3;
        locals.var_t2_dn4 = assign51310_e85276_d_n4;
        locals.var_t2_dn5 = assign51310_e85276_d_n5;
        locals.var_t2_dn6 = assign51310_e85276_d_n6;
        locals.var_t2_dn7 = assign51310_e85276_d_n7;
        locals.var_t2_dn8 = assign51310_e85276_d_n8;
        locals.var_t2_dn9 = assign51310_e85276_d_n9;
        locals.var_t2_dn10 = assign51310_e85276_d_n10;
        locals.var_t2_dn11 = assign51310_e85276_d_n11;

        let (assign51320_e85293, assign51320_e85293_d_n3, assign51320_e85293_d_n4, assign51320_e85293_d_n5, assign51320_e85293_d_n6, assign51320_e85293_d_n7, assign51320_e85293_d_n8, assign51320_e85293_d_n9, assign51320_e85293_d_n10, assign51320_e85293_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 != 0.0)) {
        let assign51320_e85287: f64 = (locals.var_betagb1_i * locals.var_voxminv);
        let assign51320_e85288: f64 = (locals.var_alphagb1_i - assign51320_e85287);
        let assign51320_e85289: f64 = (locals.var_t2 * assign51320_e85288);
        let assign51320_e85291: f64 = (assign51320_e85289 / locals.var_t0);
        (assign51320_e85291, (((((locals.var_t2_dn3 * assign51320_e85288) + (locals.var_t2 * (-(locals.var_betagb1_i * locals.var_voxminv_dn3)))) * locals.var_t0) - (assign51320_e85289 * locals.var_t0_dn3)) / (locals.var_t0 * locals.var_t0)), (((((locals.var_t2_dn4 * assign51320_e85288) + (locals.var_t2 * (locals.var_alphagb1_i_dn4 - (locals.var_betagb1_i * locals.var_voxminv_dn4)))) * locals.var_t0) - (assign51320_e85289 * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)), (((((locals.var_t2_dn5 * assign51320_e85288) + (locals.var_t2 * (locals.var_alphagb1_i_dn5 - (locals.var_betagb1_i * locals.var_voxminv_dn5)))) * locals.var_t0) - (assign51320_e85289 * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)), (((((locals.var_t2_dn6 * assign51320_e85288) + (locals.var_t2 * (-(locals.var_betagb1_i * locals.var_voxminv_dn6)))) * locals.var_t0) - (assign51320_e85289 * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)), (((((locals.var_t2_dn7 * assign51320_e85288) + (locals.var_t2 * (-(locals.var_betagb1_i * locals.var_voxminv_dn7)))) * locals.var_t0) - (assign51320_e85289 * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)), (((((locals.var_t2_dn8 * assign51320_e85288) + (locals.var_t2 * (-(locals.var_betagb1_i * locals.var_voxminv_dn8)))) * locals.var_t0) - (assign51320_e85289 * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)), (((((locals.var_t2_dn9 * assign51320_e85288) + (locals.var_t2 * (-(locals.var_betagb1_i * locals.var_voxminv_dn9)))) * locals.var_t0) - (assign51320_e85289 * locals.var_t0_dn9)) / (locals.var_t0 * locals.var_t0)), (((((locals.var_t2_dn10 * assign51320_e85288) + (locals.var_t2 * (-(locals.var_betagb1_i * locals.var_voxminv_dn10)))) * locals.var_t0) - (assign51320_e85289 * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)), (((((locals.var_t2_dn11 * assign51320_e85288) + (locals.var_t2 * (-(locals.var_betagb1_i * locals.var_voxminv_dn11)))) * locals.var_t0) - (assign51320_e85289 * locals.var_t0_dn11)) / (locals.var_t0 * locals.var_t0)),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign51320_e85293;
        locals.var_t3_dn3 = assign51320_e85293_d_n3;
        locals.var_t3_dn4 = assign51320_e85293_d_n4;
        locals.var_t3_dn5 = assign51320_e85293_d_n5;
        locals.var_t3_dn6 = assign51320_e85293_d_n6;
        locals.var_t3_dn7 = assign51320_e85293_d_n7;
        locals.var_t3_dn8 = assign51320_e85293_d_n8;
        locals.var_t3_dn9 = assign51320_e85293_d_n9;
        locals.var_t3_dn10 = assign51320_e85293_d_n10;
        locals.var_t3_dn11 = assign51320_e85293_d_n11;

        let (assign51330_e85303, assign51330_e85303_d_n3, assign51330_e85303_d_n4, assign51330_e85303_d_n5, assign51330_e85303_d_n6, assign51330_e85303_d_n7, assign51330_e85303_d_n8, assign51330_e85303_d_n9, assign51330_e85303_d_n10, assign51330_e85303_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 != 0.0)) {
        let assign51330_e85301: f64 = { let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign51330_e85301, ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn3), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn4), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn5), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn6), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn7), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn8), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn9), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn10), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn11),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign51330_e85303;
        locals.var_t4_dn3 = assign51330_e85303_d_n3;
        locals.var_t4_dn4 = assign51330_e85303_d_n4;
        locals.var_t4_dn5 = assign51330_e85303_d_n5;
        locals.var_t4_dn6 = assign51330_e85303_d_n6;
        locals.var_t4_dn7 = assign51330_e85303_d_n7;
        locals.var_t4_dn8 = assign51330_e85303_d_n8;
        locals.var_t4_dn9 = assign51330_e85303_d_n9;
        locals.var_t4_dn10 = assign51330_e85303_d_n10;
        locals.var_t4_dn11 = assign51330_e85303_d_n11;

        let (assign51340_e85318, assign51340_e85318_d_n3, assign51340_e85318_d_n4, assign51340_e85318_d_n5, assign51340_e85318_d_n6, assign51340_e85318_d_n7, assign51340_e85318_d_n8, assign51340_e85318_d_n9, assign51340_e85318_d_n10, assign51340_e85318_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 != 0.0)) {
        let assign51340_e85312: f64 = (locals.var_t1 * locals.var_vg);
        let assign51340_e85314: f64 = (assign51340_e85312 * locals.var_vaux_igbinv);
        let assign51340_e85316: f64 = (assign51340_e85314 * locals.var_t4);
        (assign51340_e85316, (((((locals.var_t1_dn3 * locals.var_vg) * locals.var_vaux_igbinv) + (assign51340_e85312 * locals.var_vaux_igbinv_dn3)) * locals.var_t4) + (assign51340_e85314 * locals.var_t4_dn3)), (((((locals.var_t1_dn4 * locals.var_vg) * locals.var_vaux_igbinv) + (assign51340_e85312 * locals.var_vaux_igbinv_dn4)) * locals.var_t4) + (assign51340_e85314 * locals.var_t4_dn4)), (((((locals.var_t1_dn5 * locals.var_vg) * locals.var_vaux_igbinv) + (assign51340_e85312 * locals.var_vaux_igbinv_dn5)) * locals.var_t4) + (assign51340_e85314 * locals.var_t4_dn5)), (((((locals.var_t1_dn6 * locals.var_vg) * locals.var_vaux_igbinv) + (assign51340_e85312 * locals.var_vaux_igbinv_dn6)) * locals.var_t4) + (assign51340_e85314 * locals.var_t4_dn6)), (((((locals.var_t1_dn7 * locals.var_vg) * locals.var_vaux_igbinv) + (assign51340_e85312 * locals.var_vaux_igbinv_dn7)) * locals.var_t4) + (assign51340_e85314 * locals.var_t4_dn7)), ((((((locals.var_t1_dn8 * locals.var_vg) + (locals.var_t1 * locals.var_vg_dn8)) * locals.var_vaux_igbinv) + (assign51340_e85312 * locals.var_vaux_igbinv_dn8)) * locals.var_t4) + (assign51340_e85314 * locals.var_t4_dn8)), (((((locals.var_t1_dn9 * locals.var_vg) * locals.var_vaux_igbinv) + (assign51340_e85312 * locals.var_vaux_igbinv_dn9)) * locals.var_t4) + (assign51340_e85314 * locals.var_t4_dn9)), ((((((locals.var_t1_dn10 * locals.var_vg) + (locals.var_t1 * locals.var_vg_dn10)) * locals.var_vaux_igbinv) + (assign51340_e85312 * locals.var_vaux_igbinv_dn10)) * locals.var_t4) + (assign51340_e85314 * locals.var_t4_dn10)), (((((locals.var_t1_dn11 * locals.var_vg) * locals.var_vaux_igbinv) + (assign51340_e85312 * locals.var_vaux_igbinv_dn11)) * locals.var_t4) + (assign51340_e85314 * locals.var_t4_dn11)),)
    } else {
        (locals.var_igbinv, locals.var_igbinv_dn3, locals.var_igbinv_dn4, locals.var_igbinv_dn5, locals.var_igbinv_dn6, locals.var_igbinv_dn7, locals.var_igbinv_dn8, locals.var_igbinv_dn9, locals.var_igbinv_dn10, locals.var_igbinv_dn11,)
    }
};
        locals.var_igbinv = assign51340_e85318;
        locals.var_igbinv_dn3 = assign51340_e85318_d_n3;
        locals.var_igbinv_dn4 = assign51340_e85318_d_n4;
        locals.var_igbinv_dn5 = assign51340_e85318_d_n5;
        locals.var_igbinv_dn6 = assign51340_e85318_d_n6;
        locals.var_igbinv_dn7 = assign51340_e85318_d_n7;
        locals.var_igbinv_dn8 = assign51340_e85318_d_n8;
        locals.var_igbinv_dn9 = assign51340_e85318_d_n9;
        locals.var_igbinv_dn10 = assign51340_e85318_d_n10;
        locals.var_igbinv_dn11 = assign51340_e85318_d_n11;

        let (assign51350_e85329, assign51350_e85329_d_n3, assign51350_e85329_d_n4, assign51350_e85329_d_n5, assign51350_e85329_d_n6, assign51350_e85329_d_n7, assign51350_e85329_d_n8, assign51350_e85329_d_n9, assign51350_e85329_d_n10, assign51350_e85329_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 != 0.0)) {
        let assign51350_e85327: f64 = (locals.var_igbinv * locals.var_igtemp);
        (assign51350_e85327, (locals.var_igbinv_dn3 * locals.var_igtemp), ((locals.var_igbinv_dn4 * locals.var_igtemp) + (locals.var_igbinv * locals.var_igtemp_dn4)), ((locals.var_igbinv_dn5 * locals.var_igtemp) + (locals.var_igbinv * locals.var_igtemp_dn5)), (locals.var_igbinv_dn6 * locals.var_igtemp), (locals.var_igbinv_dn7 * locals.var_igtemp), (locals.var_igbinv_dn8 * locals.var_igtemp), (locals.var_igbinv_dn9 * locals.var_igtemp), (locals.var_igbinv_dn10 * locals.var_igtemp), (locals.var_igbinv_dn11 * locals.var_igtemp),)
    } else {
        (locals.var_igbinv, locals.var_igbinv_dn3, locals.var_igbinv_dn4, locals.var_igbinv_dn5, locals.var_igbinv_dn6, locals.var_igbinv_dn7, locals.var_igbinv_dn8, locals.var_igbinv_dn9, locals.var_igbinv_dn10, locals.var_igbinv_dn11,)
    }
};
        locals.var_igbinv = assign51350_e85329;
        locals.var_igbinv_dn3 = assign51350_e85329_d_n3;
        locals.var_igbinv_dn4 = assign51350_e85329_d_n4;
        locals.var_igbinv_dn5 = assign51350_e85329_d_n5;
        locals.var_igbinv_dn6 = assign51350_e85329_d_n6;
        locals.var_igbinv_dn7 = assign51350_e85329_d_n7;
        locals.var_igbinv_dn8 = assign51350_e85329_d_n8;
        locals.var_igbinv_dn9 = assign51350_e85329_d_n9;
        locals.var_igbinv_dn10 = assign51350_e85329_d_n10;
        locals.var_igbinv_dn11 = assign51350_e85329_d_n11;

        let (assign51360_e85342, assign51360_e85342_d_n3, assign51360_e85342_d_n4, assign51360_e85342_d_n5, assign51360_e85342_d_n6, assign51360_e85342_d_n7, assign51360_e85342_d_n8, assign51360_e85342_d_n9, assign51360_e85342_d_n10, assign51360_e85342_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 != 0.0)) {
        let assign51360_e85339: f64 = (locals.var_igbacc + locals.var_igbinv);
        let assign51360_e85340: f64 = (p.p2 * assign51360_e85339);
        (assign51360_e85340, (p.p2 * (locals.var_igbacc_dn3 + locals.var_igbinv_dn3)), (p.p2 * (locals.var_igbacc_dn4 + locals.var_igbinv_dn4)), (p.p2 * (locals.var_igbacc_dn5 + locals.var_igbinv_dn5)), (p.p2 * (locals.var_igbacc_dn6 + locals.var_igbinv_dn6)), (p.p2 * (locals.var_igbacc_dn7 + locals.var_igbinv_dn7)), (p.p2 * (locals.var_igbacc_dn8 + locals.var_igbinv_dn8)), (p.p2 * (locals.var_igbacc_dn9 + locals.var_igbinv_dn9)), (p.p2 * (locals.var_igbacc_dn10 + locals.var_igbinv_dn10)), (p.p2 * (locals.var_igbacc_dn11 + locals.var_igbinv_dn11)),)
    } else {
        (locals.var_igb, locals.var_igb_dn3, locals.var_igb_dn4, locals.var_igb_dn5, locals.var_igb_dn6, locals.var_igb_dn7, locals.var_igb_dn8, locals.var_igb_dn9, locals.var_igb_dn10, locals.var_igb_dn11,)
    }
};
        locals.var_igb = assign51360_e85342;
        locals.var_igb_dn3 = assign51360_e85342_d_n3;
        locals.var_igb_dn4 = assign51360_e85342_d_n4;
        locals.var_igb_dn5 = assign51360_e85342_d_n5;
        locals.var_igb_dn6 = assign51360_e85342_d_n6;
        locals.var_igb_dn7 = assign51360_e85342_d_n7;
        locals.var_igb_dn8 = assign51360_e85342_d_n8;
        locals.var_igb_dn9 = assign51360_e85342_d_n9;
        locals.var_igb_dn10 = assign51360_e85342_d_n10;
        locals.var_igb_dn11 = assign51360_e85342_d_n11;

        let (assign51370_e85355, assign51370_e85355_d_n3, assign51370_e85355_d_n4, assign51370_e85355_d_n5, assign51370_e85355_d_n6, assign51370_e85355_d_n7, assign51370_e85355_d_n8, assign51370_e85355_d_n9, assign51370_e85355_d_n10, assign51370_e85355_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 != 0.0)) {
        let assign51370_e85351: f64 = (locals.var_vfb * locals.var_nvt);
        let assign51370_e85353: f64 = (assign51370_e85351 + p.p1383);
        (assign51370_e85353, ((locals.var_vfb_dn3 * locals.var_nvt) + (locals.var_vfb * locals.var_nvt_dn3)), ((locals.var_vfb_dn4 * locals.var_nvt) + (locals.var_vfb * locals.var_nvt_dn4)), ((locals.var_vfb_dn5 * locals.var_nvt) + (locals.var_vfb * locals.var_nvt_dn5)), ((locals.var_vfb_dn6 * locals.var_nvt) + (locals.var_vfb * locals.var_nvt_dn6)), ((locals.var_vfb_dn7 * locals.var_nvt) + (locals.var_vfb * locals.var_nvt_dn7)), ((locals.var_vfb_dn8 * locals.var_nvt) + (locals.var_vfb * locals.var_nvt_dn8)), ((locals.var_vfb_dn9 * locals.var_nvt) + (locals.var_vfb * locals.var_nvt_dn9)), ((locals.var_vfb_dn10 * locals.var_nvt) + (locals.var_vfb * locals.var_nvt_dn10)), ((locals.var_vfb_dn11 * locals.var_nvt) + (locals.var_vfb * locals.var_nvt_dn11)),)
    } else {
        (locals.var_vfb2, locals.var_vfb2_dn3, locals.var_vfb2_dn4, locals.var_vfb2_dn5, locals.var_vfb2_dn6, locals.var_vfb2_dn7, locals.var_vfb2_dn8, locals.var_vfb2_dn9, locals.var_vfb2_dn10, locals.var_vfb2_dn11,)
    }
};
        locals.var_vfb2 = assign51370_e85355;
        locals.var_vfb2_dn3 = assign51370_e85355_d_n3;
        locals.var_vfb2_dn4 = assign51370_e85355_d_n4;
        locals.var_vfb2_dn5 = assign51370_e85355_d_n5;
        locals.var_vfb2_dn6 = assign51370_e85355_d_n6;
        locals.var_vfb2_dn7 = assign51370_e85355_d_n7;
        locals.var_vfb2_dn8 = assign51370_e85355_d_n8;
        locals.var_vfb2_dn9 = assign51370_e85355_d_n9;
        locals.var_vfb2_dn10 = assign51370_e85355_d_n10;
        locals.var_vfb2_dn11 = assign51370_e85355_d_n11;

        let assign51380_e85378: f64 = if (((((p.p43 != 0.0) && true) && (!((p.p40 != 0.0) && (!true)))) && (p.p45 == 1.0)) && (p.p1380 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard782 = assign51380_e85378;

        let (assign51390_e85389, assign51390_e85389_d_n8, assign51390_e85389_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard782 != 0.0)) {
        let assign51390_e85387: f64 = (locals.var_devsign * (nv8 - nv11));
        (assign51390_e85387, locals.var_devsign, (-locals.var_devsign),)
    } else {
        (locals.var_vgb, locals.var_vgb_dn8, locals.var_vgb_dn11,)
    }
};
        locals.var_vgb = assign51390_e85389;
        locals.var_vgb_dn8 = assign51390_e85389_d_n8;
        locals.var_vgb_dn11 = assign51390_e85389_d_n11;

        let (assign51400_e85400, assign51400_e85400_d_n3, assign51400_e85400_d_n4, assign51400_e85400_d_n5, assign51400_e85400_d_n6, assign51400_e85400_d_n7, assign51400_e85400_d_n8, assign51400_e85400_d_n9, assign51400_e85400_d_n10, assign51400_e85400_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard782 != 0.0)) {
        let assign51400_e85398: f64 = (locals.var_vgb - locals.var_vfb2);
        (assign51400_e85398, (-locals.var_vfb2_dn3), (-locals.var_vfb2_dn4), (-locals.var_vfb2_dn5), (-locals.var_vfb2_dn6), (-locals.var_vfb2_dn7), (locals.var_vgb_dn8 - locals.var_vfb2_dn8), (-locals.var_vfb2_dn9), (-locals.var_vfb2_dn10), (locals.var_vgb_dn11 - locals.var_vfb2_dn11),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign51400_e85400;
        locals.var_t0_dn3 = assign51400_e85400_d_n3;
        locals.var_t0_dn4 = assign51400_e85400_d_n4;
        locals.var_t0_dn5 = assign51400_e85400_d_n5;
        locals.var_t0_dn6 = assign51400_e85400_d_n6;
        locals.var_t0_dn7 = assign51400_e85400_d_n7;
        locals.var_t0_dn8 = assign51400_e85400_d_n8;
        locals.var_t0_dn9 = assign51400_e85400_d_n9;
        locals.var_t0_dn10 = assign51400_e85400_d_n10;
        locals.var_t0_dn11 = assign51400_e85400_d_n11;

        let (assign51410_e85414, assign51410_e85414_d_n3, assign51410_e85414_d_n4, assign51410_e85414_d_n5, assign51410_e85414_d_n6, assign51410_e85414_d_n7, assign51410_e85414_d_n8, assign51410_e85414_d_n9, assign51410_e85414_d_n10, assign51410_e85414_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard782 != 0.0)) {
        let assign51410_e85409: f64 = (locals.var_t0 * locals.var_t0);
        let assign51410_e85411: f64 = (assign51410_e85409 + 0.0001);
        let assign51410_e85412: f64 = (assign51410_e85411).sqrt();
        (assign51410_e85412, (((locals.var_t0_dn3 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn3)) / (2.0 * assign51410_e85412)), (((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)) / (2.0 * assign51410_e85412)), (((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)) / (2.0 * assign51410_e85412)), (((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)) / (2.0 * assign51410_e85412)), (((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)) / (2.0 * assign51410_e85412)), (((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)) / (2.0 * assign51410_e85412)), (((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)) / (2.0 * assign51410_e85412)), (((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)) / (2.0 * assign51410_e85412)), (((locals.var_t0_dn11 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn11)) / (2.0 * assign51410_e85412)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign51410_e85414;
        locals.var_t1_dn3 = assign51410_e85414_d_n3;
        locals.var_t1_dn4 = assign51410_e85414_d_n4;
        locals.var_t1_dn5 = assign51410_e85414_d_n5;
        locals.var_t1_dn6 = assign51410_e85414_d_n6;
        locals.var_t1_dn7 = assign51410_e85414_d_n7;
        locals.var_t1_dn8 = assign51410_e85414_d_n8;
        locals.var_t1_dn9 = assign51410_e85414_d_n9;
        locals.var_t1_dn10 = assign51410_e85414_d_n10;
        locals.var_t1_dn11 = assign51410_e85414_d_n11;

        let (assign51420_e85430, assign51420_e85430_d_n3, assign51420_e85430_d_n4, assign51420_e85430_d_n5, assign51420_e85430_d_n6, assign51420_e85430_d_n7, assign51420_e85430_d_n8, assign51420_e85430_d_n9, assign51420_e85430_d_n10, assign51420_e85430_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard782 != 0.0)) {
        let assign51420_e85423: f64 = (-locals.var_t0);
        let assign51420_e85425: f64 = (assign51420_e85423 + locals.var_t1);
        let assign51420_e85427: f64 = (assign51420_e85425 - 0.01);
        let assign51420_e85428: f64 = (0.5 * assign51420_e85427);
        (assign51420_e85428, (0.5 * ((-locals.var_t0_dn3) + locals.var_t1_dn3)), (0.5 * ((-locals.var_t0_dn4) + locals.var_t1_dn4)), (0.5 * ((-locals.var_t0_dn5) + locals.var_t1_dn5)), (0.5 * ((-locals.var_t0_dn6) + locals.var_t1_dn6)), (0.5 * ((-locals.var_t0_dn7) + locals.var_t1_dn7)), (0.5 * ((-locals.var_t0_dn8) + locals.var_t1_dn8)), (0.5 * ((-locals.var_t0_dn9) + locals.var_t1_dn9)), (0.5 * ((-locals.var_t0_dn10) + locals.var_t1_dn10)), (0.5 * ((-locals.var_t0_dn11) + locals.var_t1_dn11)),)
    } else {
        (locals.var_vgp_eff, locals.var_vgp_eff_dn3, locals.var_vgp_eff_dn4, locals.var_vgp_eff_dn5, locals.var_vgp_eff_dn6, locals.var_vgp_eff_dn7, locals.var_vgp_eff_dn8, locals.var_vgp_eff_dn9, locals.var_vgp_eff_dn10, locals.var_vgp_eff_dn11,)
    }
};
        locals.var_vgp_eff = assign51420_e85430;
        locals.var_vgp_eff_dn3 = assign51420_e85430_d_n3;
        locals.var_vgp_eff_dn4 = assign51420_e85430_d_n4;
        locals.var_vgp_eff_dn5 = assign51420_e85430_d_n5;
        locals.var_vgp_eff_dn6 = assign51420_e85430_d_n6;
        locals.var_vgp_eff_dn7 = assign51420_e85430_d_n7;
        locals.var_vgp_eff_dn8 = assign51420_e85430_d_n8;
        locals.var_vgp_eff_dn9 = assign51420_e85430_d_n9;
        locals.var_vgp_eff_dn10 = assign51420_e85430_d_n10;
        locals.var_vgp_eff_dn11 = assign51420_e85430_d_n11;

        let (assign51430_e85444, assign51430_e85444_d_n3, assign51430_e85444_d_n4, assign51430_e85444_d_n5, assign51430_e85444_d_n6, assign51430_e85444_d_n7, assign51430_e85444_d_n8, assign51430_e85444_d_n9, assign51430_e85444_d_n10, assign51430_e85444_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard782 != 0.0)) {
        let (assign51430_e85442,) = {
            if (p.p30 == 1.0) {
                (p.p702,)
            } else {
                (p.p703,)
            }
        };
        (assign51430_e85442, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t11, locals.var_t11_dn3, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11,)
    }
};
        locals.var_t11 = assign51430_e85444;
        locals.var_t11_dn3 = assign51430_e85444_d_n3;
        locals.var_t11_dn4 = assign51430_e85444_d_n4;
        locals.var_t11_dn5 = assign51430_e85444_d_n5;
        locals.var_t11_dn6 = assign51430_e85444_d_n6;
        locals.var_t11_dn7 = assign51430_e85444_d_n7;
        locals.var_t11_dn8 = assign51430_e85444_d_n8;
        locals.var_t11_dn9 = assign51430_e85444_d_n9;
        locals.var_t11_dn10 = assign51430_e85444_d_n10;
        locals.var_t11_dn11 = assign51430_e85444_d_n11;

        let (assign51440_e85458, assign51440_e85458_d_n3, assign51440_e85458_d_n4, assign51440_e85458_d_n5, assign51440_e85458_d_n6, assign51440_e85458_d_n7, assign51440_e85458_d_n8, assign51440_e85458_d_n9, assign51440_e85458_d_n10, assign51440_e85458_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard782 != 0.0)) {
        let (assign51440_e85456,) = {
            if (p.p30 == 1.0) {
                (p.p704,)
            } else {
                (p.p705,)
            }
        };
        (assign51440_e85456, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t12, locals.var_t12_dn3, locals.var_t12_dn4, locals.var_t12_dn5, locals.var_t12_dn6, locals.var_t12_dn7, locals.var_t12_dn8, locals.var_t12_dn9, locals.var_t12_dn10, locals.var_t12_dn11,)
    }
};
        locals.var_t12 = assign51440_e85458;
        locals.var_t12_dn3 = assign51440_e85458_d_n3;
        locals.var_t12_dn4 = assign51440_e85458_d_n4;
        locals.var_t12_dn5 = assign51440_e85458_d_n5;
        locals.var_t12_dn6 = assign51440_e85458_d_n6;
        locals.var_t12_dn7 = assign51440_e85458_d_n7;
        locals.var_t12_dn8 = assign51440_e85458_d_n8;
        locals.var_t12_dn9 = assign51440_e85458_d_n9;
        locals.var_t12_dn10 = assign51440_e85458_d_n10;
        locals.var_t12_dn11 = assign51440_e85458_d_n11;

        let (assign51450_e85469, assign51450_e85469_d_n3, assign51450_e85469_d_n4, assign51450_e85469_d_n5, assign51450_e85469_d_n6, assign51450_e85469_d_n7, assign51450_e85469_d_n8, assign51450_e85469_d_n9, assign51450_e85469_d_n10, assign51450_e85469_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard782 != 0.0)) {
        let assign51450_e85467: f64 = (locals.var_vgb * locals.var_vgp_eff);
        (assign51450_e85467, (locals.var_vgb * locals.var_vgp_eff_dn3), (locals.var_vgb * locals.var_vgp_eff_dn4), (locals.var_vgb * locals.var_vgp_eff_dn5), (locals.var_vgb * locals.var_vgp_eff_dn6), (locals.var_vgb * locals.var_vgp_eff_dn7), ((locals.var_vgb_dn8 * locals.var_vgp_eff) + (locals.var_vgb * locals.var_vgp_eff_dn8)), (locals.var_vgb * locals.var_vgp_eff_dn9), (locals.var_vgb * locals.var_vgp_eff_dn10), ((locals.var_vgb_dn11 * locals.var_vgp_eff) + (locals.var_vgb * locals.var_vgp_eff_dn11)),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign51450_e85469;
        locals.var_t2_dn3 = assign51450_e85469_d_n3;
        locals.var_t2_dn4 = assign51450_e85469_d_n4;
        locals.var_t2_dn5 = assign51450_e85469_d_n5;
        locals.var_t2_dn6 = assign51450_e85469_d_n6;
        locals.var_t2_dn7 = assign51450_e85469_d_n7;
        locals.var_t2_dn8 = assign51450_e85469_d_n8;
        locals.var_t2_dn9 = assign51450_e85469_d_n9;
        locals.var_t2_dn10 = assign51450_e85469_d_n10;
        locals.var_t2_dn11 = assign51450_e85469_d_n11;

        let (assign51460_e85482, assign51460_e85482_d_n3, assign51460_e85482_d_n4, assign51460_e85482_d_n5, assign51460_e85482_d_n6, assign51460_e85482_d_n7, assign51460_e85482_d_n8, assign51460_e85482_d_n9, assign51460_e85482_d_n10, assign51460_e85482_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard782 != 0.0)) {
        let assign51460_e85478: f64 = (locals.var_aigbcp2_i * locals.var_cigbcp2_i);
        let assign51460_e85480: f64 = (assign51460_e85478 - locals.var_bigbcp2_i);
        (assign51460_e85480, 0.0, (locals.var_aigbcp2_i_dn4 * locals.var_cigbcp2_i), (locals.var_aigbcp2_i_dn5 * locals.var_cigbcp2_i), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign51460_e85482;
        locals.var_t3_dn3 = assign51460_e85482_d_n3;
        locals.var_t3_dn4 = assign51460_e85482_d_n4;
        locals.var_t3_dn5 = assign51460_e85482_d_n5;
        locals.var_t3_dn6 = assign51460_e85482_d_n6;
        locals.var_t3_dn7 = assign51460_e85482_d_n7;
        locals.var_t3_dn8 = assign51460_e85482_d_n8;
        locals.var_t3_dn9 = assign51460_e85482_d_n9;
        locals.var_t3_dn10 = assign51460_e85482_d_n10;
        locals.var_t3_dn11 = assign51460_e85482_d_n11;

        let (assign51470_e85493, assign51470_e85493_d_n3, assign51470_e85493_d_n4, assign51470_e85493_d_n5, assign51470_e85493_d_n6, assign51470_e85493_d_n7, assign51470_e85493_d_n8, assign51470_e85493_d_n9, assign51470_e85493_d_n10, assign51470_e85493_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard782 != 0.0)) {
        let assign51470_e85491: f64 = (locals.var_bigbcp2_i * locals.var_cigbcp2_i);
        (assign51470_e85491, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign51470_e85493;
        locals.var_t4_dn3 = assign51470_e85493_d_n3;
        locals.var_t4_dn4 = assign51470_e85493_d_n4;
        locals.var_t4_dn5 = assign51470_e85493_d_n5;
        locals.var_t4_dn6 = assign51470_e85493_d_n6;
        locals.var_t4_dn7 = assign51470_e85493_d_n7;
        locals.var_t4_dn8 = assign51470_e85493_d_n8;
        locals.var_t4_dn9 = assign51470_e85493_d_n9;
        locals.var_t4_dn10 = assign51470_e85493_d_n10;
        locals.var_t4_dn11 = assign51470_e85493_d_n11;

        let (assign51480_e85517, assign51480_e85517_d_n3, assign51480_e85517_d_n4, assign51480_e85517_d_n5, assign51480_e85517_d_n6, assign51480_e85517_d_n7, assign51480_e85517_d_n8, assign51480_e85517_d_n9, assign51480_e85517_d_n10, assign51480_e85517_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard782 != 0.0)) {
        let assign51480_e85501: f64 = (-locals.var_t12);
        let assign51480_e85503: f64 = (assign51480_e85501 * p.p76);
        let assign51480_e85507: f64 = (locals.var_t3 * locals.var_vgp_eff);
        let assign51480_e85508: f64 = (locals.var_aigbcp2_i + assign51480_e85507);
        let assign51480_e85511: f64 = (locals.var_t4 * locals.var_vgp_eff);
        let assign51480_e85513: f64 = (assign51480_e85511 * locals.var_vgp_eff);
        let assign51480_e85514: f64 = (assign51480_e85508 - assign51480_e85513);
        let assign51480_e85515: f64 = (assign51480_e85503 * assign51480_e85514);
        (assign51480_e85515, ((((-locals.var_t12_dn3) * p.p76) * assign51480_e85514) + (assign51480_e85503 * (((locals.var_t3_dn3 * locals.var_vgp_eff) + (locals.var_t3 * locals.var_vgp_eff_dn3)) - ((((locals.var_t4_dn3 * locals.var_vgp_eff) + (locals.var_t4 * locals.var_vgp_eff_dn3)) * locals.var_vgp_eff) + (assign51480_e85511 * locals.var_vgp_eff_dn3))))), ((((-locals.var_t12_dn4) * p.p76) * assign51480_e85514) + (assign51480_e85503 * ((locals.var_aigbcp2_i_dn4 + ((locals.var_t3_dn4 * locals.var_vgp_eff) + (locals.var_t3 * locals.var_vgp_eff_dn4))) - ((((locals.var_t4_dn4 * locals.var_vgp_eff) + (locals.var_t4 * locals.var_vgp_eff_dn4)) * locals.var_vgp_eff) + (assign51480_e85511 * locals.var_vgp_eff_dn4))))), ((((-locals.var_t12_dn5) * p.p76) * assign51480_e85514) + (assign51480_e85503 * ((locals.var_aigbcp2_i_dn5 + ((locals.var_t3_dn5 * locals.var_vgp_eff) + (locals.var_t3 * locals.var_vgp_eff_dn5))) - ((((locals.var_t4_dn5 * locals.var_vgp_eff) + (locals.var_t4 * locals.var_vgp_eff_dn5)) * locals.var_vgp_eff) + (assign51480_e85511 * locals.var_vgp_eff_dn5))))), ((((-locals.var_t12_dn6) * p.p76) * assign51480_e85514) + (assign51480_e85503 * (((locals.var_t3_dn6 * locals.var_vgp_eff) + (locals.var_t3 * locals.var_vgp_eff_dn6)) - ((((locals.var_t4_dn6 * locals.var_vgp_eff) + (locals.var_t4 * locals.var_vgp_eff_dn6)) * locals.var_vgp_eff) + (assign51480_e85511 * locals.var_vgp_eff_dn6))))), ((((-locals.var_t12_dn7) * p.p76) * assign51480_e85514) + (assign51480_e85503 * (((locals.var_t3_dn7 * locals.var_vgp_eff) + (locals.var_t3 * locals.var_vgp_eff_dn7)) - ((((locals.var_t4_dn7 * locals.var_vgp_eff) + (locals.var_t4 * locals.var_vgp_eff_dn7)) * locals.var_vgp_eff) + (assign51480_e85511 * locals.var_vgp_eff_dn7))))), ((((-locals.var_t12_dn8) * p.p76) * assign51480_e85514) + (assign51480_e85503 * (((locals.var_t3_dn8 * locals.var_vgp_eff) + (locals.var_t3 * locals.var_vgp_eff_dn8)) - ((((locals.var_t4_dn8 * locals.var_vgp_eff) + (locals.var_t4 * locals.var_vgp_eff_dn8)) * locals.var_vgp_eff) + (assign51480_e85511 * locals.var_vgp_eff_dn8))))), ((((-locals.var_t12_dn9) * p.p76) * assign51480_e85514) + (assign51480_e85503 * (((locals.var_t3_dn9 * locals.var_vgp_eff) + (locals.var_t3 * locals.var_vgp_eff_dn9)) - ((((locals.var_t4_dn9 * locals.var_vgp_eff) + (locals.var_t4 * locals.var_vgp_eff_dn9)) * locals.var_vgp_eff) + (assign51480_e85511 * locals.var_vgp_eff_dn9))))), ((((-locals.var_t12_dn10) * p.p76) * assign51480_e85514) + (assign51480_e85503 * (((locals.var_t3_dn10 * locals.var_vgp_eff) + (locals.var_t3 * locals.var_vgp_eff_dn10)) - ((((locals.var_t4_dn10 * locals.var_vgp_eff) + (locals.var_t4 * locals.var_vgp_eff_dn10)) * locals.var_vgp_eff) + (assign51480_e85511 * locals.var_vgp_eff_dn10))))), ((((-locals.var_t12_dn11) * p.p76) * assign51480_e85514) + (assign51480_e85503 * (((locals.var_t3_dn11 * locals.var_vgp_eff) + (locals.var_t3 * locals.var_vgp_eff_dn11)) - ((((locals.var_t4_dn11 * locals.var_vgp_eff) + (locals.var_t4 * locals.var_vgp_eff_dn11)) * locals.var_vgp_eff) + (assign51480_e85511 * locals.var_vgp_eff_dn11))))),)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign51480_e85517;
        locals.var_t5_dn3 = assign51480_e85517_d_n3;
        locals.var_t5_dn4 = assign51480_e85517_d_n4;
        locals.var_t5_dn5 = assign51480_e85517_d_n5;
        locals.var_t5_dn6 = assign51480_e85517_d_n6;
        locals.var_t5_dn7 = assign51480_e85517_d_n7;
        locals.var_t5_dn8 = assign51480_e85517_d_n8;
        locals.var_t5_dn9 = assign51480_e85517_d_n9;
        locals.var_t5_dn10 = assign51480_e85517_d_n10;
        locals.var_t5_dn11 = assign51480_e85517_d_n11;

        let (assign51490_e85527, assign51490_e85527_d_n3, assign51490_e85527_d_n4, assign51490_e85527_d_n5, assign51490_e85527_d_n6, assign51490_e85527_d_n7, assign51490_e85527_d_n8, assign51490_e85527_d_n9, assign51490_e85527_d_n10, assign51490_e85527_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard782 != 0.0)) {
        let assign51490_e85525: f64 = { let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign51490_e85525, ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn3), ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn4), ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn5), ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn6), ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn7), ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn8), ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn9), ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn10), ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn11),)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign51490_e85527;
        locals.var_t6_dn3 = assign51490_e85527_d_n3;
        locals.var_t6_dn4 = assign51490_e85527_d_n4;
        locals.var_t6_dn5 = assign51490_e85527_d_n5;
        locals.var_t6_dn6 = assign51490_e85527_d_n6;
        locals.var_t6_dn7 = assign51490_e85527_d_n7;
        locals.var_t6_dn8 = assign51490_e85527_d_n8;
        locals.var_t6_dn9 = assign51490_e85527_d_n9;
        locals.var_t6_dn10 = assign51490_e85527_d_n10;
        locals.var_t6_dn11 = assign51490_e85527_d_n11;

    }

    pub(super) fn stamp_transient_block_175(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign51500_e85540, assign51500_e85540_d_n3, assign51500_e85540_d_n4, assign51500_e85540_d_n5, assign51500_e85540_d_n6, assign51500_e85540_d_n7, assign51500_e85540_d_n8, assign51500_e85540_d_n9, assign51500_e85540_d_n10, assign51500_e85540_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard782 != 0.0)) {
        let assign51500_e85536: f64 = (locals.var_t11 * p.p1380);
        let assign51500_e85538: f64 = (assign51500_e85536 * locals.var_toxratio);
        (assign51500_e85538, (((locals.var_t11_dn3 * p.p1380) * locals.var_toxratio) + (assign51500_e85536 * locals.var_toxratio_dn3)), (((locals.var_t11_dn4 * p.p1380) * locals.var_toxratio) + (assign51500_e85536 * locals.var_toxratio_dn4)), (((locals.var_t11_dn5 * p.p1380) * locals.var_toxratio) + (assign51500_e85536 * locals.var_toxratio_dn5)), (((locals.var_t11_dn6 * p.p1380) * locals.var_toxratio) + (assign51500_e85536 * locals.var_toxratio_dn6)), (((locals.var_t11_dn7 * p.p1380) * locals.var_toxratio) + (assign51500_e85536 * locals.var_toxratio_dn7)), (((locals.var_t11_dn8 * p.p1380) * locals.var_toxratio) + (assign51500_e85536 * locals.var_toxratio_dn8)), (((locals.var_t11_dn9 * p.p1380) * locals.var_toxratio) + (assign51500_e85536 * locals.var_toxratio_dn9)), (((locals.var_t11_dn10 * p.p1380) * locals.var_toxratio) + (assign51500_e85536 * locals.var_toxratio_dn10)), (((locals.var_t11_dn11 * p.p1380) * locals.var_toxratio) + (assign51500_e85536 * locals.var_toxratio_dn11)),)
    } else {
        (locals.var_t11, locals.var_t11_dn3, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11,)
    }
};
        locals.var_t11 = assign51500_e85540;
        locals.var_t11_dn3 = assign51500_e85540_d_n3;
        locals.var_t11_dn4 = assign51500_e85540_d_n4;
        locals.var_t11_dn5 = assign51500_e85540_d_n5;
        locals.var_t11_dn6 = assign51500_e85540_d_n6;
        locals.var_t11_dn7 = assign51500_e85540_d_n7;
        locals.var_t11_dn8 = assign51500_e85540_d_n8;
        locals.var_t11_dn9 = assign51500_e85540_d_n9;
        locals.var_t11_dn10 = assign51500_e85540_d_n10;
        locals.var_t11_dn11 = assign51500_e85540_d_n11;

        let (assign51510_e85555, assign51510_e85555_d_n3, assign51510_e85555_d_n4, assign51510_e85555_d_n5, assign51510_e85555_d_n6, assign51510_e85555_d_n7, assign51510_e85555_d_n8, assign51510_e85555_d_n9, assign51510_e85555_d_n10, assign51510_e85555_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard782 != 0.0)) {
        let assign51510_e85549: f64 = (locals.var_t11 * locals.var_t2);
        let assign51510_e85551: f64 = (assign51510_e85549 * locals.var_t6);
        let assign51510_e85553: f64 = (assign51510_e85551 * locals.var_igtemp);
        (assign51510_e85553, (((((locals.var_t11_dn3 * locals.var_t2) + (locals.var_t11 * locals.var_t2_dn3)) * locals.var_t6) + (assign51510_e85549 * locals.var_t6_dn3)) * locals.var_igtemp), ((((((locals.var_t11_dn4 * locals.var_t2) + (locals.var_t11 * locals.var_t2_dn4)) * locals.var_t6) + (assign51510_e85549 * locals.var_t6_dn4)) * locals.var_igtemp) + (assign51510_e85551 * locals.var_igtemp_dn4)), ((((((locals.var_t11_dn5 * locals.var_t2) + (locals.var_t11 * locals.var_t2_dn5)) * locals.var_t6) + (assign51510_e85549 * locals.var_t6_dn5)) * locals.var_igtemp) + (assign51510_e85551 * locals.var_igtemp_dn5)), (((((locals.var_t11_dn6 * locals.var_t2) + (locals.var_t11 * locals.var_t2_dn6)) * locals.var_t6) + (assign51510_e85549 * locals.var_t6_dn6)) * locals.var_igtemp), (((((locals.var_t11_dn7 * locals.var_t2) + (locals.var_t11 * locals.var_t2_dn7)) * locals.var_t6) + (assign51510_e85549 * locals.var_t6_dn7)) * locals.var_igtemp), (((((locals.var_t11_dn8 * locals.var_t2) + (locals.var_t11 * locals.var_t2_dn8)) * locals.var_t6) + (assign51510_e85549 * locals.var_t6_dn8)) * locals.var_igtemp), (((((locals.var_t11_dn9 * locals.var_t2) + (locals.var_t11 * locals.var_t2_dn9)) * locals.var_t6) + (assign51510_e85549 * locals.var_t6_dn9)) * locals.var_igtemp), (((((locals.var_t11_dn10 * locals.var_t2) + (locals.var_t11 * locals.var_t2_dn10)) * locals.var_t6) + (assign51510_e85549 * locals.var_t6_dn10)) * locals.var_igtemp), (((((locals.var_t11_dn11 * locals.var_t2) + (locals.var_t11 * locals.var_t2_dn11)) * locals.var_t6) + (assign51510_e85549 * locals.var_t6_dn11)) * locals.var_igtemp),)
    } else {
        (locals.var_ig_agbcp2, locals.var_ig_agbcp2_dn3, locals.var_ig_agbcp2_dn4, locals.var_ig_agbcp2_dn5, locals.var_ig_agbcp2_dn6, locals.var_ig_agbcp2_dn7, locals.var_ig_agbcp2_dn8, locals.var_ig_agbcp2_dn9, locals.var_ig_agbcp2_dn10, locals.var_ig_agbcp2_dn11,)
    }
};
        locals.var_ig_agbcp2 = assign51510_e85555;
        locals.var_ig_agbcp2_dn3 = assign51510_e85555_d_n3;
        locals.var_ig_agbcp2_dn4 = assign51510_e85555_d_n4;
        locals.var_ig_agbcp2_dn5 = assign51510_e85555_d_n5;
        locals.var_ig_agbcp2_dn6 = assign51510_e85555_d_n6;
        locals.var_ig_agbcp2_dn7 = assign51510_e85555_d_n7;
        locals.var_ig_agbcp2_dn8 = assign51510_e85555_d_n8;
        locals.var_ig_agbcp2_dn9 = assign51510_e85555_d_n9;
        locals.var_ig_agbcp2_dn10 = assign51510_e85555_d_n10;
        locals.var_ig_agbcp2_dn11 = assign51510_e85555_d_n11;

        let (assign51520_e85565, assign51520_e85565_d_n3, assign51520_e85565_d_n4, assign51520_e85565_d_n5, assign51520_e85565_d_n6, assign51520_e85565_d_n7, assign51520_e85565_d_n8, assign51520_e85565_d_n9, assign51520_e85565_d_n10, assign51520_e85565_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard782 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ig_agbcp2, locals.var_ig_agbcp2_dn3, locals.var_ig_agbcp2_dn4, locals.var_ig_agbcp2_dn5, locals.var_ig_agbcp2_dn6, locals.var_ig_agbcp2_dn7, locals.var_ig_agbcp2_dn8, locals.var_ig_agbcp2_dn9, locals.var_ig_agbcp2_dn10, locals.var_ig_agbcp2_dn11,)
    }
};
        locals.var_ig_agbcp2 = assign51520_e85565;
        locals.var_ig_agbcp2_dn3 = assign51520_e85565_d_n3;
        locals.var_ig_agbcp2_dn4 = assign51520_e85565_d_n4;
        locals.var_ig_agbcp2_dn5 = assign51520_e85565_d_n5;
        locals.var_ig_agbcp2_dn6 = assign51520_e85565_d_n6;
        locals.var_ig_agbcp2_dn7 = assign51520_e85565_d_n7;
        locals.var_ig_agbcp2_dn8 = assign51520_e85565_d_n8;
        locals.var_ig_agbcp2_dn9 = assign51520_e85565_d_n9;
        locals.var_ig_agbcp2_dn10 = assign51520_e85565_d_n10;
        locals.var_ig_agbcp2_dn11 = assign51520_e85565_d_n11;

        let assign51530_e85568: f64 = if p.p37 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard783 = assign51530_e85568;

        let (assign51540_e85581, assign51540_e85581_d_n3, assign51540_e85581_d_n4, assign51540_e85581_d_n5, assign51540_e85581_d_n6, assign51540_e85581_d_n7, assign51540_e85581_d_n8, assign51540_e85581_d_n9, assign51540_e85581_d_n10, assign51540_e85581_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard783 != 0.0)) {
        let assign51540_e85578: f64 = (locals.var_bigc_i * locals.var_voxminv);
        let assign51540_e85579: f64 = (locals.var_aigc_i - assign51540_e85578);
        (assign51540_e85579, (-(locals.var_bigc_i * locals.var_voxminv_dn3)), (locals.var_aigc_i_dn4 - (locals.var_bigc_i * locals.var_voxminv_dn4)), (locals.var_aigc_i_dn5 - (locals.var_bigc_i * locals.var_voxminv_dn5)), (-(locals.var_bigc_i * locals.var_voxminv_dn6)), (-(locals.var_bigc_i * locals.var_voxminv_dn7)), (-(locals.var_bigc_i * locals.var_voxminv_dn8)), (-(locals.var_bigc_i * locals.var_voxminv_dn9)), (-(locals.var_bigc_i * locals.var_voxminv_dn10)), (-(locals.var_bigc_i * locals.var_voxminv_dn11)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign51540_e85581;
        locals.var_t1_dn3 = assign51540_e85581_d_n3;
        locals.var_t1_dn4 = assign51540_e85581_d_n4;
        locals.var_t1_dn5 = assign51540_e85581_d_n5;
        locals.var_t1_dn6 = assign51540_e85581_d_n6;
        locals.var_t1_dn7 = assign51540_e85581_d_n7;
        locals.var_t1_dn8 = assign51540_e85581_d_n8;
        locals.var_t1_dn9 = assign51540_e85581_d_n9;
        locals.var_t1_dn10 = assign51540_e85581_d_n10;
        locals.var_t1_dn11 = assign51540_e85581_d_n11;

        let (assign51550_e85594, assign51550_e85594_d_n3, assign51550_e85594_d_n4, assign51550_e85594_d_n5, assign51550_e85594_d_n6, assign51550_e85594_d_n7, assign51550_e85594_d_n8, assign51550_e85594_d_n9, assign51550_e85594_d_n10, assign51550_e85594_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard783 != 0.0)) {
        let assign51550_e85591: f64 = (locals.var_cigc_i * locals.var_voxminv);
        let assign51550_e85592: f64 = (1.0 + assign51550_e85591);
        (assign51550_e85592, (locals.var_cigc_i * locals.var_voxminv_dn3), (locals.var_cigc_i * locals.var_voxminv_dn4), (locals.var_cigc_i * locals.var_voxminv_dn5), (locals.var_cigc_i * locals.var_voxminv_dn6), (locals.var_cigc_i * locals.var_voxminv_dn7), (locals.var_cigc_i * locals.var_voxminv_dn8), (locals.var_cigc_i * locals.var_voxminv_dn9), (locals.var_cigc_i * locals.var_voxminv_dn10), (locals.var_cigc_i * locals.var_voxminv_dn11),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign51550_e85594;
        locals.var_t2_dn3 = assign51550_e85594_d_n3;
        locals.var_t2_dn4 = assign51550_e85594_d_n4;
        locals.var_t2_dn5 = assign51550_e85594_d_n5;
        locals.var_t2_dn6 = assign51550_e85594_d_n6;
        locals.var_t2_dn7 = assign51550_e85594_d_n7;
        locals.var_t2_dn8 = assign51550_e85594_d_n8;
        locals.var_t2_dn9 = assign51550_e85594_d_n9;
        locals.var_t2_dn10 = assign51550_e85594_d_n10;
        locals.var_t2_dn11 = assign51550_e85594_d_n11;

        let (assign51560_e85607, assign51560_e85607_d_n3, assign51560_e85607_d_n4, assign51560_e85607_d_n5, assign51560_e85607_d_n6, assign51560_e85607_d_n7, assign51560_e85607_d_n8, assign51560_e85607_d_n9, assign51560_e85607_d_n10, assign51560_e85607_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard783 != 0.0)) {
        let assign51560_e85603: f64 = (locals.var_bechvb * locals.var_t1);
        let assign51560_e85605: f64 = (assign51560_e85603 * locals.var_t2);
        (assign51560_e85605, (((locals.var_bechvb * locals.var_t1_dn3) * locals.var_t2) + (assign51560_e85603 * locals.var_t2_dn3)), (((locals.var_bechvb * locals.var_t1_dn4) * locals.var_t2) + (assign51560_e85603 * locals.var_t2_dn4)), (((locals.var_bechvb * locals.var_t1_dn5) * locals.var_t2) + (assign51560_e85603 * locals.var_t2_dn5)), (((locals.var_bechvb * locals.var_t1_dn6) * locals.var_t2) + (assign51560_e85603 * locals.var_t2_dn6)), (((locals.var_bechvb * locals.var_t1_dn7) * locals.var_t2) + (assign51560_e85603 * locals.var_t2_dn7)), (((locals.var_bechvb * locals.var_t1_dn8) * locals.var_t2) + (assign51560_e85603 * locals.var_t2_dn8)), (((locals.var_bechvb * locals.var_t1_dn9) * locals.var_t2) + (assign51560_e85603 * locals.var_t2_dn9)), (((locals.var_bechvb * locals.var_t1_dn10) * locals.var_t2) + (assign51560_e85603 * locals.var_t2_dn10)), (((locals.var_bechvb * locals.var_t1_dn11) * locals.var_t2) + (assign51560_e85603 * locals.var_t2_dn11)),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign51560_e85607;
        locals.var_t3_dn3 = assign51560_e85607_d_n3;
        locals.var_t3_dn4 = assign51560_e85607_d_n4;
        locals.var_t3_dn5 = assign51560_e85607_d_n5;
        locals.var_t3_dn6 = assign51560_e85607_d_n6;
        locals.var_t3_dn7 = assign51560_e85607_d_n7;
        locals.var_t3_dn8 = assign51560_e85607_d_n8;
        locals.var_t3_dn9 = assign51560_e85607_d_n9;
        locals.var_t3_dn10 = assign51560_e85607_d_n10;
        locals.var_t3_dn11 = assign51560_e85607_d_n11;

        let (assign51570_e85625, assign51570_e85625_d_n3, assign51570_e85625_d_n4, assign51570_e85625_d_n5, assign51570_e85625_d_n6, assign51570_e85625_d_n7, assign51570_e85625_d_n8, assign51570_e85625_d_n9, assign51570_e85625_d_n10, assign51570_e85625_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard783 != 0.0)) {
        let assign51570_e85616: f64 = (locals.var_nq * locals.var_nvt);
        let assign51570_e85619: f64 = (locals.var_qs_1 + locals.var_qdeff);
        let assign51570_e85620: f64 = (assign51570_e85616 * assign51570_e85619);
        let assign51570_e85622: f64 = { let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign51570_e85623: f64 = (assign51570_e85620 * assign51570_e85622);
        (assign51570_e85623, ((((((locals.var_nq_dn3 * locals.var_nvt) + (locals.var_nq * locals.var_nvt_dn3)) * assign51570_e85619) + (assign51570_e85616 * (locals.var_qs_1_dn3 + locals.var_qdeff_dn3))) * assign51570_e85622) + (assign51570_e85620 * ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn3))), ((((((locals.var_nq_dn4 * locals.var_nvt) + (locals.var_nq * locals.var_nvt_dn4)) * assign51570_e85619) + (assign51570_e85616 * (locals.var_qs_1_dn4 + locals.var_qdeff_dn4))) * assign51570_e85622) + (assign51570_e85620 * ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn4))), ((((((locals.var_nq_dn5 * locals.var_nvt) + (locals.var_nq * locals.var_nvt_dn5)) * assign51570_e85619) + (assign51570_e85616 * (locals.var_qs_1_dn5 + locals.var_qdeff_dn5))) * assign51570_e85622) + (assign51570_e85620 * ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn5))), ((((((locals.var_nq_dn6 * locals.var_nvt) + (locals.var_nq * locals.var_nvt_dn6)) * assign51570_e85619) + (assign51570_e85616 * (locals.var_qs_1_dn6 + locals.var_qdeff_dn6))) * assign51570_e85622) + (assign51570_e85620 * ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn6))), ((((((locals.var_nq_dn7 * locals.var_nvt) + (locals.var_nq * locals.var_nvt_dn7)) * assign51570_e85619) + (assign51570_e85616 * (locals.var_qs_1_dn7 + locals.var_qdeff_dn7))) * assign51570_e85622) + (assign51570_e85620 * ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn7))), ((((((locals.var_nq_dn8 * locals.var_nvt) + (locals.var_nq * locals.var_nvt_dn8)) * assign51570_e85619) + (assign51570_e85616 * (locals.var_qs_1_dn8 + locals.var_qdeff_dn8))) * assign51570_e85622) + (assign51570_e85620 * ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn8))), ((((((locals.var_nq_dn9 * locals.var_nvt) + (locals.var_nq * locals.var_nvt_dn9)) * assign51570_e85619) + (assign51570_e85616 * (locals.var_qs_1_dn9 + locals.var_qdeff_dn9))) * assign51570_e85622) + (assign51570_e85620 * ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn9))), ((((((locals.var_nq_dn10 * locals.var_nvt) + (locals.var_nq * locals.var_nvt_dn10)) * assign51570_e85619) + (assign51570_e85616 * (locals.var_qs_1_dn10 + locals.var_qdeff_dn10))) * assign51570_e85622) + (assign51570_e85620 * ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn10))), ((((((locals.var_nq_dn11 * locals.var_nvt) + (locals.var_nq * locals.var_nvt_dn11)) * assign51570_e85619) + (assign51570_e85616 * (locals.var_qs_1_dn11 + locals.var_qdeff_dn11))) * assign51570_e85622) + (assign51570_e85620 * ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn11))),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign51570_e85625;
        locals.var_t4_dn3 = assign51570_e85625_d_n3;
        locals.var_t4_dn4 = assign51570_e85625_d_n4;
        locals.var_t4_dn5 = assign51570_e85625_d_n5;
        locals.var_t4_dn6 = assign51570_e85625_d_n6;
        locals.var_t4_dn7 = assign51570_e85625_d_n7;
        locals.var_t4_dn8 = assign51570_e85625_d_n8;
        locals.var_t4_dn9 = assign51570_e85625_d_n9;
        locals.var_t4_dn10 = assign51570_e85625_d_n10;
        locals.var_t4_dn11 = assign51570_e85625_d_n11;

        let (assign51580_e85652, assign51580_e85652_d_n3, assign51580_e85652_d_n4, assign51580_e85652_d_n5, assign51580_e85652_d_n6, assign51580_e85652_d_n7, assign51580_e85652_d_n8, assign51580_e85652_d_n9, assign51580_e85652_d_n10, assign51580_e85652_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard783 != 0.0)) {
        let assign51580_e85634: f64 = (p.p2 * locals.var_aechvb);
        let assign51580_e85636: f64 = (assign51580_e85634 * locals.var_t4);
        let assign51580_e85640: f64 = (0.5 * locals.var_vdsx);
        let assign51580_e85641: f64 = (locals.var_vg + assign51580_e85640);
        let assign51580_e85645: f64 = (locals.var_vs + locals.var_vd);
        let assign51580_e85646: f64 = (0.5 * assign51580_e85645);
        let assign51580_e85647: f64 = (assign51580_e85641 - assign51580_e85646);
        let assign51580_e85648: f64 = (assign51580_e85636 * assign51580_e85647);
        let assign51580_e85650: f64 = (assign51580_e85648 * locals.var_igtemp);
        (assign51580_e85650, ((((((p.p2 * locals.var_aechvb_dn3) * locals.var_t4) + (assign51580_e85634 * locals.var_t4_dn3)) * assign51580_e85647) + (assign51580_e85636 * (0.5 * locals.var_vdsx_dn3))) * locals.var_igtemp), (((((((p.p2 * locals.var_aechvb_dn4) * locals.var_t4) + (assign51580_e85634 * locals.var_t4_dn4)) * assign51580_e85647) + (assign51580_e85636 * (0.5 * locals.var_vdsx_dn4))) * locals.var_igtemp) + (assign51580_e85648 * locals.var_igtemp_dn4)), (((((((p.p2 * locals.var_aechvb_dn5) * locals.var_t4) + (assign51580_e85634 * locals.var_t4_dn5)) * assign51580_e85647) + (assign51580_e85636 * (0.5 * locals.var_vdsx_dn5))) * locals.var_igtemp) + (assign51580_e85648 * locals.var_igtemp_dn5)), ((((((p.p2 * locals.var_aechvb_dn6) * locals.var_t4) + (assign51580_e85634 * locals.var_t4_dn6)) * assign51580_e85647) + (assign51580_e85636 * ((0.5 * locals.var_vdsx_dn6) - (0.5 * (locals.var_vs_dn6 + locals.var_vd_dn6))))) * locals.var_igtemp), ((((((p.p2 * locals.var_aechvb_dn7) * locals.var_t4) + (assign51580_e85634 * locals.var_t4_dn7)) * assign51580_e85647) + (assign51580_e85636 * ((0.5 * locals.var_vdsx_dn7) - (0.5 * (locals.var_vs_dn7 + locals.var_vd_dn7))))) * locals.var_igtemp), ((((((p.p2 * locals.var_aechvb_dn8) * locals.var_t4) + (assign51580_e85634 * locals.var_t4_dn8)) * assign51580_e85647) + (assign51580_e85636 * (locals.var_vg_dn8 + (0.5 * locals.var_vdsx_dn8)))) * locals.var_igtemp), ((((((p.p2 * locals.var_aechvb_dn9) * locals.var_t4) + (assign51580_e85634 * locals.var_t4_dn9)) * assign51580_e85647) + (assign51580_e85636 * (0.5 * locals.var_vdsx_dn9))) * locals.var_igtemp), ((((((p.p2 * locals.var_aechvb_dn10) * locals.var_t4) + (assign51580_e85634 * locals.var_t4_dn10)) * assign51580_e85647) + (assign51580_e85636 * ((locals.var_vg_dn10 + (0.5 * locals.var_vdsx_dn10)) - (0.5 * (locals.var_vs_dn10 + locals.var_vd_dn10))))) * locals.var_igtemp), ((((((p.p2 * locals.var_aechvb_dn11) * locals.var_t4) + (assign51580_e85634 * locals.var_t4_dn11)) * assign51580_e85647) + (assign51580_e85636 * (0.5 * locals.var_vdsx_dn11))) * locals.var_igtemp),)
    } else {
        (locals.var_igc0, locals.var_igc0_dn3, locals.var_igc0_dn4, locals.var_igc0_dn5, locals.var_igc0_dn6, locals.var_igc0_dn7, locals.var_igc0_dn8, locals.var_igc0_dn9, locals.var_igc0_dn10, locals.var_igc0_dn11,)
    }
};
        locals.var_igc0 = assign51580_e85652;
        locals.var_igc0_dn3 = assign51580_e85652_d_n3;
        locals.var_igc0_dn4 = assign51580_e85652_d_n4;
        locals.var_igc0_dn5 = assign51580_e85652_d_n5;
        locals.var_igc0_dn6 = assign51580_e85652_d_n6;
        locals.var_igc0_dn7 = assign51580_e85652_d_n7;
        locals.var_igc0_dn8 = assign51580_e85652_d_n8;
        locals.var_igc0_dn9 = assign51580_e85652_d_n9;
        locals.var_igc0_dn10 = assign51580_e85652_d_n10;
        locals.var_igc0_dn11 = assign51580_e85652_d_n11;

        let (assign51590_e85668, assign51590_e85668_d_n3, assign51590_e85668_d_n4, assign51590_e85668_d_n5, assign51590_e85668_d_n6, assign51590_e85668_d_n7, assign51590_e85668_d_n8, assign51590_e85668_d_n9, assign51590_e85668_d_n10, assign51590_e85668_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard783 != 0.0)) {
        let assign51590_e85661: f64 = (locals.var_vdseff * locals.var_vdseff);
        let assign51590_e85663: f64 = (assign51590_e85661 + 0.01);
        let assign51590_e85664: f64 = (assign51590_e85663).sqrt();
        let assign51590_e85666: f64 = (assign51590_e85664 - 0.1);
        (assign51590_e85666, (((locals.var_vdseff_dn3 * locals.var_vdseff) + (locals.var_vdseff * locals.var_vdseff_dn3)) / (2.0 * assign51590_e85664)), (((locals.var_vdseff_dn4 * locals.var_vdseff) + (locals.var_vdseff * locals.var_vdseff_dn4)) / (2.0 * assign51590_e85664)), (((locals.var_vdseff_dn5 * locals.var_vdseff) + (locals.var_vdseff * locals.var_vdseff_dn5)) / (2.0 * assign51590_e85664)), (((locals.var_vdseff_dn6 * locals.var_vdseff) + (locals.var_vdseff * locals.var_vdseff_dn6)) / (2.0 * assign51590_e85664)), (((locals.var_vdseff_dn7 * locals.var_vdseff) + (locals.var_vdseff * locals.var_vdseff_dn7)) / (2.0 * assign51590_e85664)), (((locals.var_vdseff_dn8 * locals.var_vdseff) + (locals.var_vdseff * locals.var_vdseff_dn8)) / (2.0 * assign51590_e85664)), (((locals.var_vdseff_dn9 * locals.var_vdseff) + (locals.var_vdseff * locals.var_vdseff_dn9)) / (2.0 * assign51590_e85664)), (((locals.var_vdseff_dn10 * locals.var_vdseff) + (locals.var_vdseff * locals.var_vdseff_dn10)) / (2.0 * assign51590_e85664)), (((locals.var_vdseff_dn11 * locals.var_vdseff) + (locals.var_vdseff * locals.var_vdseff_dn11)) / (2.0 * assign51590_e85664)),)
    } else {
        (locals.var_vdseffx, locals.var_vdseffx_dn3, locals.var_vdseffx_dn4, locals.var_vdseffx_dn5, locals.var_vdseffx_dn6, locals.var_vdseffx_dn7, locals.var_vdseffx_dn8, locals.var_vdseffx_dn9, locals.var_vdseffx_dn10, locals.var_vdseffx_dn11,)
    }
};
        locals.var_vdseffx = assign51590_e85668;
        locals.var_vdseffx_dn3 = assign51590_e85668_d_n3;
        locals.var_vdseffx_dn4 = assign51590_e85668_d_n4;
        locals.var_vdseffx_dn5 = assign51590_e85668_d_n5;
        locals.var_vdseffx_dn6 = assign51590_e85668_d_n6;
        locals.var_vdseffx_dn7 = assign51590_e85668_d_n7;
        locals.var_vdseffx_dn8 = assign51590_e85668_d_n8;
        locals.var_vdseffx_dn9 = assign51590_e85668_d_n9;
        locals.var_vdseffx_dn10 = assign51590_e85668_d_n10;
        locals.var_vdseffx_dn11 = assign51590_e85668_d_n11;

        let (assign51600_e85679, assign51600_e85679_d_n3, assign51600_e85679_d_n4, assign51600_e85679_d_n5, assign51600_e85679_d_n6, assign51600_e85679_d_n7, assign51600_e85679_d_n8, assign51600_e85679_d_n9, assign51600_e85679_d_n10, assign51600_e85679_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard783 != 0.0)) {
        let assign51600_e85677: f64 = (locals.var_pigcd_i * locals.var_vdseffx);
        (assign51600_e85677, (locals.var_pigcd_i * locals.var_vdseffx_dn3), (locals.var_pigcd_i * locals.var_vdseffx_dn4), (locals.var_pigcd_i * locals.var_vdseffx_dn5), (locals.var_pigcd_i * locals.var_vdseffx_dn6), (locals.var_pigcd_i * locals.var_vdseffx_dn7), (locals.var_pigcd_i * locals.var_vdseffx_dn8), (locals.var_pigcd_i * locals.var_vdseffx_dn9), (locals.var_pigcd_i * locals.var_vdseffx_dn10), (locals.var_pigcd_i * locals.var_vdseffx_dn11),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign51600_e85679;
        locals.var_t1_dn3 = assign51600_e85679_d_n3;
        locals.var_t1_dn4 = assign51600_e85679_d_n4;
        locals.var_t1_dn5 = assign51600_e85679_d_n5;
        locals.var_t1_dn6 = assign51600_e85679_d_n6;
        locals.var_t1_dn7 = assign51600_e85679_d_n7;
        locals.var_t1_dn8 = assign51600_e85679_d_n8;
        locals.var_t1_dn9 = assign51600_e85679_d_n9;
        locals.var_t1_dn10 = assign51600_e85679_d_n10;
        locals.var_t1_dn11 = assign51600_e85679_d_n11;

        let (assign51610_e85690, assign51610_e85690_d_n3, assign51610_e85690_d_n4, assign51610_e85690_d_n5, assign51610_e85690_d_n6, assign51610_e85690_d_n7, assign51610_e85690_d_n8, assign51610_e85690_d_n9, assign51610_e85690_d_n10, assign51610_e85690_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard783 != 0.0)) {
        let assign51610_e85687: f64 = (-locals.var_t1);
        let assign51610_e85688: f64 = { let limited_exp_arg = assign51610_e85687; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign51610_e85688, ({ let limited_exp_arg = assign51610_e85687; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn3)), ({ let limited_exp_arg = assign51610_e85687; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn4)), ({ let limited_exp_arg = assign51610_e85687; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn5)), ({ let limited_exp_arg = assign51610_e85687; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn6)), ({ let limited_exp_arg = assign51610_e85687; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn7)), ({ let limited_exp_arg = assign51610_e85687; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn8)), ({ let limited_exp_arg = assign51610_e85687; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn9)), ({ let limited_exp_arg = assign51610_e85687; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn10)), ({ let limited_exp_arg = assign51610_e85687; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn11)),)
    } else {
        (locals.var_t1_exp, locals.var_t1_exp_dn3, locals.var_t1_exp_dn4, locals.var_t1_exp_dn5, locals.var_t1_exp_dn6, locals.var_t1_exp_dn7, locals.var_t1_exp_dn8, locals.var_t1_exp_dn9, locals.var_t1_exp_dn10, locals.var_t1_exp_dn11,)
    }
};
        locals.var_t1_exp = assign51610_e85690;
        locals.var_t1_exp_dn3 = assign51610_e85690_d_n3;
        locals.var_t1_exp_dn4 = assign51610_e85690_d_n4;
        locals.var_t1_exp_dn5 = assign51610_e85690_d_n5;
        locals.var_t1_exp_dn6 = assign51610_e85690_d_n6;
        locals.var_t1_exp_dn7 = assign51610_e85690_d_n7;
        locals.var_t1_exp_dn8 = assign51610_e85690_d_n8;
        locals.var_t1_exp_dn9 = assign51610_e85690_d_n9;
        locals.var_t1_exp_dn10 = assign51610_e85690_d_n10;
        locals.var_t1_exp_dn11 = assign51610_e85690_d_n11;

        let (assign51620_e85705, assign51620_e85705_d_n3, assign51620_e85705_d_n4, assign51620_e85705_d_n5, assign51620_e85705_d_n6, assign51620_e85705_d_n7, assign51620_e85705_d_n8, assign51620_e85705_d_n9, assign51620_e85705_d_n10, assign51620_e85705_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard783 != 0.0)) {
        let assign51620_e85699: f64 = (locals.var_t1 + locals.var_t1_exp);
        let assign51620_e85701: f64 = (assign51620_e85699 - 1.0);
        let assign51620_e85703: f64 = (assign51620_e85701 + 0.0001);
        (assign51620_e85703, (locals.var_t1_dn3 + locals.var_t1_exp_dn3), (locals.var_t1_dn4 + locals.var_t1_exp_dn4), (locals.var_t1_dn5 + locals.var_t1_exp_dn5), (locals.var_t1_dn6 + locals.var_t1_exp_dn6), (locals.var_t1_dn7 + locals.var_t1_exp_dn7), (locals.var_t1_dn8 + locals.var_t1_exp_dn8), (locals.var_t1_dn9 + locals.var_t1_exp_dn9), (locals.var_t1_dn10 + locals.var_t1_exp_dn10), (locals.var_t1_dn11 + locals.var_t1_exp_dn11),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign51620_e85705;
        locals.var_t3_dn3 = assign51620_e85705_d_n3;
        locals.var_t3_dn4 = assign51620_e85705_d_n4;
        locals.var_t3_dn5 = assign51620_e85705_d_n5;
        locals.var_t3_dn6 = assign51620_e85705_d_n6;
        locals.var_t3_dn7 = assign51620_e85705_d_n7;
        locals.var_t3_dn8 = assign51620_e85705_d_n8;
        locals.var_t3_dn9 = assign51620_e85705_d_n9;
        locals.var_t3_dn10 = assign51620_e85705_d_n10;
        locals.var_t3_dn11 = assign51620_e85705_d_n11;

        let (assign51630_e85722, assign51630_e85722_d_n3, assign51630_e85722_d_n4, assign51630_e85722_d_n5, assign51630_e85722_d_n6, assign51630_e85722_d_n7, assign51630_e85722_d_n8, assign51630_e85722_d_n9, assign51630_e85722_d_n10, assign51630_e85722_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard783 != 0.0)) {
        let assign51630_e85715: f64 = (locals.var_t1 + 1.0);
        let assign51630_e85717: f64 = (assign51630_e85715 * locals.var_t1_exp);
        let assign51630_e85718: f64 = (1.0 - assign51630_e85717);
        let assign51630_e85720: f64 = (assign51630_e85718 + 0.0001);
        (assign51630_e85720, (-((locals.var_t1_dn3 * locals.var_t1_exp) + (assign51630_e85715 * locals.var_t1_exp_dn3))), (-((locals.var_t1_dn4 * locals.var_t1_exp) + (assign51630_e85715 * locals.var_t1_exp_dn4))), (-((locals.var_t1_dn5 * locals.var_t1_exp) + (assign51630_e85715 * locals.var_t1_exp_dn5))), (-((locals.var_t1_dn6 * locals.var_t1_exp) + (assign51630_e85715 * locals.var_t1_exp_dn6))), (-((locals.var_t1_dn7 * locals.var_t1_exp) + (assign51630_e85715 * locals.var_t1_exp_dn7))), (-((locals.var_t1_dn8 * locals.var_t1_exp) + (assign51630_e85715 * locals.var_t1_exp_dn8))), (-((locals.var_t1_dn9 * locals.var_t1_exp) + (assign51630_e85715 * locals.var_t1_exp_dn9))), (-((locals.var_t1_dn10 * locals.var_t1_exp) + (assign51630_e85715 * locals.var_t1_exp_dn10))), (-((locals.var_t1_dn11 * locals.var_t1_exp) + (assign51630_e85715 * locals.var_t1_exp_dn11))),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign51630_e85722;
        locals.var_t4_dn3 = assign51630_e85722_d_n3;
        locals.var_t4_dn4 = assign51630_e85722_d_n4;
        locals.var_t4_dn5 = assign51630_e85722_d_n5;
        locals.var_t4_dn6 = assign51630_e85722_d_n6;
        locals.var_t4_dn7 = assign51630_e85722_d_n7;
        locals.var_t4_dn8 = assign51630_e85722_d_n8;
        locals.var_t4_dn9 = assign51630_e85722_d_n9;
        locals.var_t4_dn10 = assign51630_e85722_d_n10;
        locals.var_t4_dn11 = assign51630_e85722_d_n11;

        let (assign51640_e85735, assign51640_e85735_d_n3, assign51640_e85735_d_n4, assign51640_e85735_d_n5, assign51640_e85735_d_n6, assign51640_e85735_d_n7, assign51640_e85735_d_n8, assign51640_e85735_d_n9, assign51640_e85735_d_n10, assign51640_e85735_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard783 != 0.0)) {
        let assign51640_e85731: f64 = (locals.var_t1 * locals.var_t1);
        let assign51640_e85733: f64 = (assign51640_e85731 + 0.0002);
        (assign51640_e85733, ((locals.var_t1_dn3 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn3)), ((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)), ((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)), ((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)), ((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)), ((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)), ((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)), ((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)), ((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)),)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign51640_e85735;
        locals.var_t5_dn3 = assign51640_e85735_d_n3;
        locals.var_t5_dn4 = assign51640_e85735_d_n4;
        locals.var_t5_dn5 = assign51640_e85735_d_n5;
        locals.var_t5_dn6 = assign51640_e85735_d_n6;
        locals.var_t5_dn7 = assign51640_e85735_d_n7;
        locals.var_t5_dn8 = assign51640_e85735_d_n8;
        locals.var_t5_dn9 = assign51640_e85735_d_n9;
        locals.var_t5_dn10 = assign51640_e85735_d_n10;
        locals.var_t5_dn11 = assign51640_e85735_d_n11;

        let assign51650_e85738: f64 = if locals.var_sigvds > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard784 = assign51650_e85738;

        let (assign51660_e85753, assign51660_e85753_d_n3, assign51660_e85753_d_n4, assign51660_e85753_d_n5, assign51660_e85753_d_n6, assign51660_e85753_d_n7, assign51660_e85753_d_n8, assign51660_e85753_d_n9, assign51660_e85753_d_n10, assign51660_e85753_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard783 != 0.0)) && (locals.var_guard784 != 0.0)) {
        let assign51660_e85749: f64 = (locals.var_igc0 * locals.var_t4);
        let assign51660_e85751: f64 = (assign51660_e85749 / locals.var_t5);
        (assign51660_e85751, (((((locals.var_igc0_dn3 * locals.var_t4) + (locals.var_igc0 * locals.var_t4_dn3)) * locals.var_t5) - (assign51660_e85749 * locals.var_t5_dn3)) / (locals.var_t5 * locals.var_t5)), (((((locals.var_igc0_dn4 * locals.var_t4) + (locals.var_igc0 * locals.var_t4_dn4)) * locals.var_t5) - (assign51660_e85749 * locals.var_t5_dn4)) / (locals.var_t5 * locals.var_t5)), (((((locals.var_igc0_dn5 * locals.var_t4) + (locals.var_igc0 * locals.var_t4_dn5)) * locals.var_t5) - (assign51660_e85749 * locals.var_t5_dn5)) / (locals.var_t5 * locals.var_t5)), (((((locals.var_igc0_dn6 * locals.var_t4) + (locals.var_igc0 * locals.var_t4_dn6)) * locals.var_t5) - (assign51660_e85749 * locals.var_t5_dn6)) / (locals.var_t5 * locals.var_t5)), (((((locals.var_igc0_dn7 * locals.var_t4) + (locals.var_igc0 * locals.var_t4_dn7)) * locals.var_t5) - (assign51660_e85749 * locals.var_t5_dn7)) / (locals.var_t5 * locals.var_t5)), (((((locals.var_igc0_dn8 * locals.var_t4) + (locals.var_igc0 * locals.var_t4_dn8)) * locals.var_t5) - (assign51660_e85749 * locals.var_t5_dn8)) / (locals.var_t5 * locals.var_t5)), (((((locals.var_igc0_dn9 * locals.var_t4) + (locals.var_igc0 * locals.var_t4_dn9)) * locals.var_t5) - (assign51660_e85749 * locals.var_t5_dn9)) / (locals.var_t5 * locals.var_t5)), (((((locals.var_igc0_dn10 * locals.var_t4) + (locals.var_igc0 * locals.var_t4_dn10)) * locals.var_t5) - (assign51660_e85749 * locals.var_t5_dn10)) / (locals.var_t5 * locals.var_t5)), (((((locals.var_igc0_dn11 * locals.var_t4) + (locals.var_igc0 * locals.var_t4_dn11)) * locals.var_t5) - (assign51660_e85749 * locals.var_t5_dn11)) / (locals.var_t5 * locals.var_t5)),)
    } else {
        (locals.var_igcd, locals.var_igcd_dn3, locals.var_igcd_dn4, locals.var_igcd_dn5, locals.var_igcd_dn6, locals.var_igcd_dn7, locals.var_igcd_dn8, locals.var_igcd_dn9, locals.var_igcd_dn10, locals.var_igcd_dn11,)
    }
};
        locals.var_igcd = assign51660_e85753;
        locals.var_igcd_dn3 = assign51660_e85753_d_n3;
        locals.var_igcd_dn4 = assign51660_e85753_d_n4;
        locals.var_igcd_dn5 = assign51660_e85753_d_n5;
        locals.var_igcd_dn6 = assign51660_e85753_d_n6;
        locals.var_igcd_dn7 = assign51660_e85753_d_n7;
        locals.var_igcd_dn8 = assign51660_e85753_d_n8;
        locals.var_igcd_dn9 = assign51660_e85753_d_n9;
        locals.var_igcd_dn10 = assign51660_e85753_d_n10;
        locals.var_igcd_dn11 = assign51660_e85753_d_n11;

        let (assign51670_e85768, assign51670_e85768_d_n3, assign51670_e85768_d_n4, assign51670_e85768_d_n5, assign51670_e85768_d_n6, assign51670_e85768_d_n7, assign51670_e85768_d_n8, assign51670_e85768_d_n9, assign51670_e85768_d_n10, assign51670_e85768_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard783 != 0.0)) && (locals.var_guard784 != 0.0)) {
        let assign51670_e85764: f64 = (locals.var_igc0 * locals.var_t3);
        let assign51670_e85766: f64 = (assign51670_e85764 / locals.var_t5);
        (assign51670_e85766, (((((locals.var_igc0_dn3 * locals.var_t3) + (locals.var_igc0 * locals.var_t3_dn3)) * locals.var_t5) - (assign51670_e85764 * locals.var_t5_dn3)) / (locals.var_t5 * locals.var_t5)), (((((locals.var_igc0_dn4 * locals.var_t3) + (locals.var_igc0 * locals.var_t3_dn4)) * locals.var_t5) - (assign51670_e85764 * locals.var_t5_dn4)) / (locals.var_t5 * locals.var_t5)), (((((locals.var_igc0_dn5 * locals.var_t3) + (locals.var_igc0 * locals.var_t3_dn5)) * locals.var_t5) - (assign51670_e85764 * locals.var_t5_dn5)) / (locals.var_t5 * locals.var_t5)), (((((locals.var_igc0_dn6 * locals.var_t3) + (locals.var_igc0 * locals.var_t3_dn6)) * locals.var_t5) - (assign51670_e85764 * locals.var_t5_dn6)) / (locals.var_t5 * locals.var_t5)), (((((locals.var_igc0_dn7 * locals.var_t3) + (locals.var_igc0 * locals.var_t3_dn7)) * locals.var_t5) - (assign51670_e85764 * locals.var_t5_dn7)) / (locals.var_t5 * locals.var_t5)), (((((locals.var_igc0_dn8 * locals.var_t3) + (locals.var_igc0 * locals.var_t3_dn8)) * locals.var_t5) - (assign51670_e85764 * locals.var_t5_dn8)) / (locals.var_t5 * locals.var_t5)), (((((locals.var_igc0_dn9 * locals.var_t3) + (locals.var_igc0 * locals.var_t3_dn9)) * locals.var_t5) - (assign51670_e85764 * locals.var_t5_dn9)) / (locals.var_t5 * locals.var_t5)), (((((locals.var_igc0_dn10 * locals.var_t3) + (locals.var_igc0 * locals.var_t3_dn10)) * locals.var_t5) - (assign51670_e85764 * locals.var_t5_dn10)) / (locals.var_t5 * locals.var_t5)), (((((locals.var_igc0_dn11 * locals.var_t3) + (locals.var_igc0 * locals.var_t3_dn11)) * locals.var_t5) - (assign51670_e85764 * locals.var_t5_dn11)) / (locals.var_t5 * locals.var_t5)),)
    } else {
        (locals.var_igcs, locals.var_igcs_dn3, locals.var_igcs_dn4, locals.var_igcs_dn5, locals.var_igcs_dn6, locals.var_igcs_dn7, locals.var_igcs_dn8, locals.var_igcs_dn9, locals.var_igcs_dn10, locals.var_igcs_dn11,)
    }
};
        locals.var_igcs = assign51670_e85768;
        locals.var_igcs_dn3 = assign51670_e85768_d_n3;
        locals.var_igcs_dn4 = assign51670_e85768_d_n4;
        locals.var_igcs_dn5 = assign51670_e85768_d_n5;
        locals.var_igcs_dn6 = assign51670_e85768_d_n6;
        locals.var_igcs_dn7 = assign51670_e85768_d_n7;
        locals.var_igcs_dn8 = assign51670_e85768_d_n8;
        locals.var_igcs_dn9 = assign51670_e85768_d_n9;
        locals.var_igcs_dn10 = assign51670_e85768_d_n10;
        locals.var_igcs_dn11 = assign51670_e85768_d_n11;

        let (assign51680_e85784, assign51680_e85784_d_n3, assign51680_e85784_d_n4, assign51680_e85784_d_n5, assign51680_e85784_d_n6, assign51680_e85784_d_n7, assign51680_e85784_d_n8, assign51680_e85784_d_n9, assign51680_e85784_d_n10, assign51680_e85784_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard783 != 0.0)) && (locals.var_guard784 == 0.0)) {
        let assign51680_e85780: f64 = (locals.var_igc0 * locals.var_t4);
        let assign51680_e85782: f64 = (assign51680_e85780 / locals.var_t5);
        (assign51680_e85782, (((((locals.var_igc0_dn3 * locals.var_t4) + (locals.var_igc0 * locals.var_t4_dn3)) * locals.var_t5) - (assign51680_e85780 * locals.var_t5_dn3)) / (locals.var_t5 * locals.var_t5)), (((((locals.var_igc0_dn4 * locals.var_t4) + (locals.var_igc0 * locals.var_t4_dn4)) * locals.var_t5) - (assign51680_e85780 * locals.var_t5_dn4)) / (locals.var_t5 * locals.var_t5)), (((((locals.var_igc0_dn5 * locals.var_t4) + (locals.var_igc0 * locals.var_t4_dn5)) * locals.var_t5) - (assign51680_e85780 * locals.var_t5_dn5)) / (locals.var_t5 * locals.var_t5)), (((((locals.var_igc0_dn6 * locals.var_t4) + (locals.var_igc0 * locals.var_t4_dn6)) * locals.var_t5) - (assign51680_e85780 * locals.var_t5_dn6)) / (locals.var_t5 * locals.var_t5)), (((((locals.var_igc0_dn7 * locals.var_t4) + (locals.var_igc0 * locals.var_t4_dn7)) * locals.var_t5) - (assign51680_e85780 * locals.var_t5_dn7)) / (locals.var_t5 * locals.var_t5)), (((((locals.var_igc0_dn8 * locals.var_t4) + (locals.var_igc0 * locals.var_t4_dn8)) * locals.var_t5) - (assign51680_e85780 * locals.var_t5_dn8)) / (locals.var_t5 * locals.var_t5)), (((((locals.var_igc0_dn9 * locals.var_t4) + (locals.var_igc0 * locals.var_t4_dn9)) * locals.var_t5) - (assign51680_e85780 * locals.var_t5_dn9)) / (locals.var_t5 * locals.var_t5)), (((((locals.var_igc0_dn10 * locals.var_t4) + (locals.var_igc0 * locals.var_t4_dn10)) * locals.var_t5) - (assign51680_e85780 * locals.var_t5_dn10)) / (locals.var_t5 * locals.var_t5)), (((((locals.var_igc0_dn11 * locals.var_t4) + (locals.var_igc0 * locals.var_t4_dn11)) * locals.var_t5) - (assign51680_e85780 * locals.var_t5_dn11)) / (locals.var_t5 * locals.var_t5)),)
    } else {
        (locals.var_igcs, locals.var_igcs_dn3, locals.var_igcs_dn4, locals.var_igcs_dn5, locals.var_igcs_dn6, locals.var_igcs_dn7, locals.var_igcs_dn8, locals.var_igcs_dn9, locals.var_igcs_dn10, locals.var_igcs_dn11,)
    }
};
        locals.var_igcs = assign51680_e85784;
        locals.var_igcs_dn3 = assign51680_e85784_d_n3;
        locals.var_igcs_dn4 = assign51680_e85784_d_n4;
        locals.var_igcs_dn5 = assign51680_e85784_d_n5;
        locals.var_igcs_dn6 = assign51680_e85784_d_n6;
        locals.var_igcs_dn7 = assign51680_e85784_d_n7;
        locals.var_igcs_dn8 = assign51680_e85784_d_n8;
        locals.var_igcs_dn9 = assign51680_e85784_d_n9;
        locals.var_igcs_dn10 = assign51680_e85784_d_n10;
        locals.var_igcs_dn11 = assign51680_e85784_d_n11;

        let (assign51690_e85800, assign51690_e85800_d_n3, assign51690_e85800_d_n4, assign51690_e85800_d_n5, assign51690_e85800_d_n6, assign51690_e85800_d_n7, assign51690_e85800_d_n8, assign51690_e85800_d_n9, assign51690_e85800_d_n10, assign51690_e85800_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard783 != 0.0)) && (locals.var_guard784 == 0.0)) {
        let assign51690_e85796: f64 = (locals.var_igc0 * locals.var_t3);
        let assign51690_e85798: f64 = (assign51690_e85796 / locals.var_t5);
        (assign51690_e85798, (((((locals.var_igc0_dn3 * locals.var_t3) + (locals.var_igc0 * locals.var_t3_dn3)) * locals.var_t5) - (assign51690_e85796 * locals.var_t5_dn3)) / (locals.var_t5 * locals.var_t5)), (((((locals.var_igc0_dn4 * locals.var_t3) + (locals.var_igc0 * locals.var_t3_dn4)) * locals.var_t5) - (assign51690_e85796 * locals.var_t5_dn4)) / (locals.var_t5 * locals.var_t5)), (((((locals.var_igc0_dn5 * locals.var_t3) + (locals.var_igc0 * locals.var_t3_dn5)) * locals.var_t5) - (assign51690_e85796 * locals.var_t5_dn5)) / (locals.var_t5 * locals.var_t5)), (((((locals.var_igc0_dn6 * locals.var_t3) + (locals.var_igc0 * locals.var_t3_dn6)) * locals.var_t5) - (assign51690_e85796 * locals.var_t5_dn6)) / (locals.var_t5 * locals.var_t5)), (((((locals.var_igc0_dn7 * locals.var_t3) + (locals.var_igc0 * locals.var_t3_dn7)) * locals.var_t5) - (assign51690_e85796 * locals.var_t5_dn7)) / (locals.var_t5 * locals.var_t5)), (((((locals.var_igc0_dn8 * locals.var_t3) + (locals.var_igc0 * locals.var_t3_dn8)) * locals.var_t5) - (assign51690_e85796 * locals.var_t5_dn8)) / (locals.var_t5 * locals.var_t5)), (((((locals.var_igc0_dn9 * locals.var_t3) + (locals.var_igc0 * locals.var_t3_dn9)) * locals.var_t5) - (assign51690_e85796 * locals.var_t5_dn9)) / (locals.var_t5 * locals.var_t5)), (((((locals.var_igc0_dn10 * locals.var_t3) + (locals.var_igc0 * locals.var_t3_dn10)) * locals.var_t5) - (assign51690_e85796 * locals.var_t5_dn10)) / (locals.var_t5 * locals.var_t5)), (((((locals.var_igc0_dn11 * locals.var_t3) + (locals.var_igc0 * locals.var_t3_dn11)) * locals.var_t5) - (assign51690_e85796 * locals.var_t5_dn11)) / (locals.var_t5 * locals.var_t5)),)
    } else {
        (locals.var_igcd, locals.var_igcd_dn3, locals.var_igcd_dn4, locals.var_igcd_dn5, locals.var_igcd_dn6, locals.var_igcd_dn7, locals.var_igcd_dn8, locals.var_igcd_dn9, locals.var_igcd_dn10, locals.var_igcd_dn11,)
    }
};
        locals.var_igcd = assign51690_e85800;
        locals.var_igcd_dn3 = assign51690_e85800_d_n3;
        locals.var_igcd_dn4 = assign51690_e85800_d_n4;
        locals.var_igcd_dn5 = assign51690_e85800_d_n5;
        locals.var_igcd_dn6 = assign51690_e85800_d_n6;
        locals.var_igcd_dn7 = assign51690_e85800_d_n7;
        locals.var_igcd_dn8 = assign51690_e85800_d_n8;
        locals.var_igcd_dn9 = assign51690_e85800_d_n9;
        locals.var_igcd_dn10 = assign51690_e85800_d_n10;
        locals.var_igcd_dn11 = assign51690_e85800_d_n11;

        let (assign51700_e85811, assign51700_e85811_d_n3, assign51700_e85811_d_n4, assign51700_e85811_d_n5, assign51700_e85811_d_n6, assign51700_e85811_d_n7, assign51700_e85811_d_n8, assign51700_e85811_d_n9, assign51700_e85811_d_n10, assign51700_e85811_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard783 != 0.0)) {
        let assign51700_e85809: f64 = (locals.var_vgs_noswap - locals.var_vfbsdr);
        (assign51700_e85809, 0.0, (-locals.var_vfbsdr_dn4), (-locals.var_vfbsdr_dn5), locals.var_vgs_noswap_dn6, locals.var_vgs_noswap_dn7, locals.var_vgs_noswap_dn8, 0.0, locals.var_vgs_noswap_dn10, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign51700_e85811;
        locals.var_t2_dn3 = assign51700_e85811_d_n3;
        locals.var_t2_dn4 = assign51700_e85811_d_n4;
        locals.var_t2_dn5 = assign51700_e85811_d_n5;
        locals.var_t2_dn6 = assign51700_e85811_d_n6;
        locals.var_t2_dn7 = assign51700_e85811_d_n7;
        locals.var_t2_dn8 = assign51700_e85811_d_n8;
        locals.var_t2_dn9 = assign51700_e85811_d_n9;
        locals.var_t2_dn10 = assign51700_e85811_d_n10;
        locals.var_t2_dn11 = assign51700_e85811_d_n11;

        let (assign51710_e85825, assign51710_e85825_d_n3, assign51710_e85825_d_n4, assign51710_e85825_d_n5, assign51710_e85825_d_n6, assign51710_e85825_d_n7, assign51710_e85825_d_n8, assign51710_e85825_d_n9, assign51710_e85825_d_n10, assign51710_e85825_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard783 != 0.0)) {
        let assign51710_e85820: f64 = (locals.var_t2 * locals.var_t2);
        let assign51710_e85822: f64 = (assign51710_e85820 + 0.0001);
        let assign51710_e85823: f64 = (assign51710_e85822).sqrt();
        (assign51710_e85823, (((locals.var_t2_dn3 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn3)) / (2.0 * assign51710_e85823)), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign51710_e85823)), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign51710_e85823)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign51710_e85823)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign51710_e85823)), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign51710_e85823)), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign51710_e85823)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign51710_e85823)), (((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign51710_e85823)),)
    } else {
        (locals.var_vgs_eff, locals.var_vgs_eff_dn3, locals.var_vgs_eff_dn4, locals.var_vgs_eff_dn5, locals.var_vgs_eff_dn6, locals.var_vgs_eff_dn7, locals.var_vgs_eff_dn8, locals.var_vgs_eff_dn9, locals.var_vgs_eff_dn10, locals.var_vgs_eff_dn11,)
    }
};
        locals.var_vgs_eff = assign51710_e85825;
        locals.var_vgs_eff_dn3 = assign51710_e85825_d_n3;
        locals.var_vgs_eff_dn4 = assign51710_e85825_d_n4;
        locals.var_vgs_eff_dn5 = assign51710_e85825_d_n5;
        locals.var_vgs_eff_dn6 = assign51710_e85825_d_n6;
        locals.var_vgs_eff_dn7 = assign51710_e85825_d_n7;
        locals.var_vgs_eff_dn8 = assign51710_e85825_d_n8;
        locals.var_vgs_eff_dn9 = assign51710_e85825_d_n9;
        locals.var_vgs_eff_dn10 = assign51710_e85825_d_n10;
        locals.var_vgs_eff_dn11 = assign51710_e85825_d_n11;

        let assign51720_e85828: f64 = if p.p1295 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard785 = assign51720_e85828;

        let (assign51730_e85864, assign51730_e85864_d_n3, assign51730_e85864_d_n4, assign51730_e85864_d_n5, assign51730_e85864_d_n6, assign51730_e85864_d_n7, assign51730_e85864_d_n8, assign51730_e85864_d_n9, assign51730_e85864_d_n10, assign51730_e85864_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard783 != 0.0)) && (locals.var_guard785 != 0.0)) {
        let assign51730_e85841: f64 = (locals.var_bigs_i * locals.var_vgs_eff);
        let assign51730_e85842: f64 = (locals.var_aigs_i - assign51730_e85841);
        let assign51730_e85846: f64 = (locals.var_bigs_i * locals.var_vgs_eff);
        let assign51730_e85847: f64 = (locals.var_aigs_i - assign51730_e85846);
        let assign51730_e85851: f64 = (locals.var_bigs_i * locals.var_vgs_eff);
        let assign51730_e85852: f64 = (locals.var_aigs_i - assign51730_e85851);
        let assign51730_e85853: f64 = (assign51730_e85847 * assign51730_e85852);
        let assign51730_e85856: f64 = (4.0 * 1e-6);
        let assign51730_e85858: f64 = (assign51730_e85856 * 1e-6);
        let assign51730_e85859: f64 = (assign51730_e85853 + assign51730_e85858);
        let assign51730_e85860: f64 = (assign51730_e85859).sqrt();
        let assign51730_e85861: f64 = (assign51730_e85842 + assign51730_e85860);
        let assign51730_e85862: f64 = (0.5 * assign51730_e85861);
        (assign51730_e85862, (0.5 * ((-(locals.var_bigs_i * locals.var_vgs_eff_dn3)) + ((((-(locals.var_bigs_i * locals.var_vgs_eff_dn3)) * assign51730_e85852) + (assign51730_e85847 * (-(locals.var_bigs_i * locals.var_vgs_eff_dn3)))) / (2.0 * assign51730_e85860)))), (0.5 * ((locals.var_aigs_i_dn4 - (locals.var_bigs_i * locals.var_vgs_eff_dn4)) + ((((locals.var_aigs_i_dn4 - (locals.var_bigs_i * locals.var_vgs_eff_dn4)) * assign51730_e85852) + (assign51730_e85847 * (locals.var_aigs_i_dn4 - (locals.var_bigs_i * locals.var_vgs_eff_dn4)))) / (2.0 * assign51730_e85860)))), (0.5 * ((locals.var_aigs_i_dn5 - (locals.var_bigs_i * locals.var_vgs_eff_dn5)) + ((((locals.var_aigs_i_dn5 - (locals.var_bigs_i * locals.var_vgs_eff_dn5)) * assign51730_e85852) + (assign51730_e85847 * (locals.var_aigs_i_dn5 - (locals.var_bigs_i * locals.var_vgs_eff_dn5)))) / (2.0 * assign51730_e85860)))), (0.5 * ((-(locals.var_bigs_i * locals.var_vgs_eff_dn6)) + ((((-(locals.var_bigs_i * locals.var_vgs_eff_dn6)) * assign51730_e85852) + (assign51730_e85847 * (-(locals.var_bigs_i * locals.var_vgs_eff_dn6)))) / (2.0 * assign51730_e85860)))), (0.5 * ((-(locals.var_bigs_i * locals.var_vgs_eff_dn7)) + ((((-(locals.var_bigs_i * locals.var_vgs_eff_dn7)) * assign51730_e85852) + (assign51730_e85847 * (-(locals.var_bigs_i * locals.var_vgs_eff_dn7)))) / (2.0 * assign51730_e85860)))), (0.5 * ((-(locals.var_bigs_i * locals.var_vgs_eff_dn8)) + ((((-(locals.var_bigs_i * locals.var_vgs_eff_dn8)) * assign51730_e85852) + (assign51730_e85847 * (-(locals.var_bigs_i * locals.var_vgs_eff_dn8)))) / (2.0 * assign51730_e85860)))), (0.5 * ((-(locals.var_bigs_i * locals.var_vgs_eff_dn9)) + ((((-(locals.var_bigs_i * locals.var_vgs_eff_dn9)) * assign51730_e85852) + (assign51730_e85847 * (-(locals.var_bigs_i * locals.var_vgs_eff_dn9)))) / (2.0 * assign51730_e85860)))), (0.5 * ((-(locals.var_bigs_i * locals.var_vgs_eff_dn10)) + ((((-(locals.var_bigs_i * locals.var_vgs_eff_dn10)) * assign51730_e85852) + (assign51730_e85847 * (-(locals.var_bigs_i * locals.var_vgs_eff_dn10)))) / (2.0 * assign51730_e85860)))), (0.5 * ((-(locals.var_bigs_i * locals.var_vgs_eff_dn11)) + ((((-(locals.var_bigs_i * locals.var_vgs_eff_dn11)) * assign51730_e85852) + (assign51730_e85847 * (-(locals.var_bigs_i * locals.var_vgs_eff_dn11)))) / (2.0 * assign51730_e85860)))),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign51730_e85864;
        locals.var_t1_dn3 = assign51730_e85864_d_n3;
        locals.var_t1_dn4 = assign51730_e85864_d_n4;
        locals.var_t1_dn5 = assign51730_e85864_d_n5;
        locals.var_t1_dn6 = assign51730_e85864_d_n6;
        locals.var_t1_dn7 = assign51730_e85864_d_n7;
        locals.var_t1_dn8 = assign51730_e85864_d_n8;
        locals.var_t1_dn9 = assign51730_e85864_d_n9;
        locals.var_t1_dn10 = assign51730_e85864_d_n10;
        locals.var_t1_dn11 = assign51730_e85864_d_n11;

        let assign51740_e85867: f64 = if locals.var_cigs_i < 0.01 { 1.0 } else { 0.0 };
        locals.var_guard786 = assign51740_e85867;

        let (assign51750_e85880,) = {
    if (((((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard783 != 0.0)) && (locals.var_guard785 != 0.0)) && (locals.var_guard786 != 0.0)) {
        (0.01,)
    } else {
        (locals.var_cigs_i,)
    }
};
        locals.var_cigs_i = assign51750_e85880;

        let (assign51760_e85896, assign51760_e85896_d_n3, assign51760_e85896_d_n4, assign51760_e85896_d_n5, assign51760_e85896_d_n6, assign51760_e85896_d_n7, assign51760_e85896_d_n8, assign51760_e85896_d_n9, assign51760_e85896_d_n10, assign51760_e85896_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard783 != 0.0)) && (locals.var_guard785 == 0.0)) {
        let assign51760_e85893: f64 = (locals.var_bigs_i * locals.var_vgs_eff);
        let assign51760_e85894: f64 = (locals.var_aigs_i - assign51760_e85893);
        (assign51760_e85894, (-(locals.var_bigs_i * locals.var_vgs_eff_dn3)), (locals.var_aigs_i_dn4 - (locals.var_bigs_i * locals.var_vgs_eff_dn4)), (locals.var_aigs_i_dn5 - (locals.var_bigs_i * locals.var_vgs_eff_dn5)), (-(locals.var_bigs_i * locals.var_vgs_eff_dn6)), (-(locals.var_bigs_i * locals.var_vgs_eff_dn7)), (-(locals.var_bigs_i * locals.var_vgs_eff_dn8)), (-(locals.var_bigs_i * locals.var_vgs_eff_dn9)), (-(locals.var_bigs_i * locals.var_vgs_eff_dn10)), (-(locals.var_bigs_i * locals.var_vgs_eff_dn11)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign51760_e85896;
        locals.var_t1_dn3 = assign51760_e85896_d_n3;
        locals.var_t1_dn4 = assign51760_e85896_d_n4;
        locals.var_t1_dn5 = assign51760_e85896_d_n5;
        locals.var_t1_dn6 = assign51760_e85896_d_n6;
        locals.var_t1_dn7 = assign51760_e85896_d_n7;
        locals.var_t1_dn8 = assign51760_e85896_d_n8;
        locals.var_t1_dn9 = assign51760_e85896_d_n9;
        locals.var_t1_dn10 = assign51760_e85896_d_n10;
        locals.var_t1_dn11 = assign51760_e85896_d_n11;

        let (assign51770_e85909, assign51770_e85909_d_n3, assign51770_e85909_d_n4, assign51770_e85909_d_n5, assign51770_e85909_d_n6, assign51770_e85909_d_n7, assign51770_e85909_d_n8, assign51770_e85909_d_n9, assign51770_e85909_d_n10, assign51770_e85909_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard783 != 0.0)) {
        let assign51770_e85906: f64 = (locals.var_cigs_i * locals.var_vgs_eff);
        let assign51770_e85907: f64 = (1.0 + assign51770_e85906);
        (assign51770_e85907, (locals.var_cigs_i * locals.var_vgs_eff_dn3), (locals.var_cigs_i * locals.var_vgs_eff_dn4), (locals.var_cigs_i * locals.var_vgs_eff_dn5), (locals.var_cigs_i * locals.var_vgs_eff_dn6), (locals.var_cigs_i * locals.var_vgs_eff_dn7), (locals.var_cigs_i * locals.var_vgs_eff_dn8), (locals.var_cigs_i * locals.var_vgs_eff_dn9), (locals.var_cigs_i * locals.var_vgs_eff_dn10), (locals.var_cigs_i * locals.var_vgs_eff_dn11),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign51770_e85909;
        locals.var_t2_dn3 = assign51770_e85909_d_n3;
        locals.var_t2_dn4 = assign51770_e85909_d_n4;
        locals.var_t2_dn5 = assign51770_e85909_d_n5;
        locals.var_t2_dn6 = assign51770_e85909_d_n6;
        locals.var_t2_dn7 = assign51770_e85909_d_n7;
        locals.var_t2_dn8 = assign51770_e85909_d_n8;
        locals.var_t2_dn9 = assign51770_e85909_d_n9;
        locals.var_t2_dn10 = assign51770_e85909_d_n10;
        locals.var_t2_dn11 = assign51770_e85909_d_n11;

    }
}
