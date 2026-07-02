#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_240(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign67290_e104026, assign67290_e104026_d_n0, assign67290_e104026_d_n2, assign67290_e104026_d_n4, assign67290_e104026_d_n5, assign67290_e104026_d_n6, assign67290_e104026_d_n7, assign67290_e104026_d_n8, assign67290_e104026_d_n9, assign67290_e104026_d_n10, assign67290_e104026_d_n11, assign67290_e104026_d_n14,) = {
    if ((locals.var_guard1584 == 0.0) && (locals.var_guard1593 != 0.0)) {
        let assign67290_e104024: f64 = (locals.var_t4 + 1e-25);
        (assign67290_e104024, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign67290_e104026;
        locals.var_t4_dn0 = assign67290_e104026_d_n0;
        locals.var_t4_dn2 = assign67290_e104026_d_n2;
        locals.var_t4_dn4 = assign67290_e104026_d_n4;
        locals.var_t4_dn5 = assign67290_e104026_d_n5;
        locals.var_t4_dn6 = assign67290_e104026_d_n6;
        locals.var_t4_dn7 = assign67290_e104026_d_n7;
        locals.var_t4_dn8 = assign67290_e104026_d_n8;
        locals.var_t4_dn9 = assign67290_e104026_d_n9;
        locals.var_t4_dn10 = assign67290_e104026_d_n10;
        locals.var_t4_dn11 = assign67290_e104026_d_n11;
        locals.var_t4_dn14 = assign67290_e104026_d_n14;

        let (assign67300_e104041, assign67300_e104041_d_n0, assign67300_e104041_d_n2, assign67300_e104041_d_n4, assign67300_e104041_d_n5, assign67300_e104041_d_n6, assign67300_e104041_d_n7, assign67300_e104041_d_n8, assign67300_e104041_d_n9, assign67300_e104041_d_n10, assign67300_e104041_d_n11, assign67300_e104041_d_n14,) = {
    if ((locals.var_guard1584 == 0.0) && (locals.var_guard1593 != 0.0)) {
        let assign67300_e104033: f64 = (locals.var_uc_xpdv * locals.var_uc_xldld);
        let assign67300_e104035: f64 = (-1.0);
        let assign67300_e104037: f64 = (assign67300_e104035 / locals.var_t4);
        let assign67300_e104038: f64 = (assign67300_e104037).exp();
        let assign67300_e104039: f64 = (assign67300_e104033 * assign67300_e104038);
        (assign67300_e104039, (assign67300_e104033 * (assign67300_e104038 * (-((assign67300_e104035 * locals.var_t4_dn0) / (locals.var_t4 * locals.var_t4))))), (assign67300_e104033 * (assign67300_e104038 * (-((assign67300_e104035 * locals.var_t4_dn2) / (locals.var_t4 * locals.var_t4))))), (assign67300_e104033 * (assign67300_e104038 * (-((assign67300_e104035 * locals.var_t4_dn4) / (locals.var_t4 * locals.var_t4))))), (assign67300_e104033 * (assign67300_e104038 * (-((assign67300_e104035 * locals.var_t4_dn5) / (locals.var_t4 * locals.var_t4))))), (assign67300_e104033 * (assign67300_e104038 * (-((assign67300_e104035 * locals.var_t4_dn6) / (locals.var_t4 * locals.var_t4))))), (assign67300_e104033 * (assign67300_e104038 * (-((assign67300_e104035 * locals.var_t4_dn7) / (locals.var_t4 * locals.var_t4))))), (assign67300_e104033 * (assign67300_e104038 * (-((assign67300_e104035 * locals.var_t4_dn8) / (locals.var_t4 * locals.var_t4))))), (assign67300_e104033 * (assign67300_e104038 * (-((assign67300_e104035 * locals.var_t4_dn9) / (locals.var_t4 * locals.var_t4))))), (assign67300_e104033 * (assign67300_e104038 * (-((assign67300_e104035 * locals.var_t4_dn10) / (locals.var_t4 * locals.var_t4))))), (assign67300_e104033 * (assign67300_e104038 * (-((assign67300_e104035 * locals.var_t4_dn11) / (locals.var_t4 * locals.var_t4))))), (assign67300_e104033 * (assign67300_e104038 * (-((assign67300_e104035 * locals.var_t4_dn14) / (locals.var_t4 * locals.var_t4))))),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign67300_e104041;
        locals.var_t10_dn0 = assign67300_e104041_d_n0;
        locals.var_t10_dn2 = assign67300_e104041_d_n2;
        locals.var_t10_dn4 = assign67300_e104041_d_n4;
        locals.var_t10_dn5 = assign67300_e104041_d_n5;
        locals.var_t10_dn6 = assign67300_e104041_d_n6;
        locals.var_t10_dn7 = assign67300_e104041_d_n7;
        locals.var_t10_dn8 = assign67300_e104041_d_n8;
        locals.var_t10_dn9 = assign67300_e104041_d_n9;
        locals.var_t10_dn10 = assign67300_e104041_d_n10;
        locals.var_t10_dn11 = assign67300_e104041_d_n11;
        locals.var_t10_dn14 = assign67300_e104041_d_n14;

        let (assign67310_e104054, assign67310_e104054_d_n0, assign67310_e104054_d_n2, assign67310_e104054_d_n4, assign67310_e104054_d_n5, assign67310_e104054_d_n6, assign67310_e104054_d_n7, assign67310_e104054_d_n8, assign67310_e104054_d_n9, assign67310_e104054_d_n10, assign67310_e104054_d_n11, assign67310_e104054_d_n14,) = {
    if ((locals.var_guard1584 == 0.0) && (locals.var_guard1593 != 0.0)) {
        let assign67310_e104050: f64 = (1.0 / locals.var_t4);
        let assign67310_e104051: f64 = (1.0 + assign67310_e104050);
        let assign67310_e104052: f64 = (locals.var_t10 * assign67310_e104051);
        (assign67310_e104052, ((locals.var_t10_dn0 * assign67310_e104051) + (locals.var_t10 * (-(locals.var_t4_dn0 / (locals.var_t4 * locals.var_t4))))), ((locals.var_t10_dn2 * assign67310_e104051) + (locals.var_t10 * (-(locals.var_t4_dn2 / (locals.var_t4 * locals.var_t4))))), ((locals.var_t10_dn4 * assign67310_e104051) + (locals.var_t10 * (-(locals.var_t4_dn4 / (locals.var_t4 * locals.var_t4))))), ((locals.var_t10_dn5 * assign67310_e104051) + (locals.var_t10 * (-(locals.var_t4_dn5 / (locals.var_t4 * locals.var_t4))))), ((locals.var_t10_dn6 * assign67310_e104051) + (locals.var_t10 * (-(locals.var_t4_dn6 / (locals.var_t4 * locals.var_t4))))), ((locals.var_t10_dn7 * assign67310_e104051) + (locals.var_t10 * (-(locals.var_t4_dn7 / (locals.var_t4 * locals.var_t4))))), ((locals.var_t10_dn8 * assign67310_e104051) + (locals.var_t10 * (-(locals.var_t4_dn8 / (locals.var_t4 * locals.var_t4))))), ((locals.var_t10_dn9 * assign67310_e104051) + (locals.var_t10 * (-(locals.var_t4_dn9 / (locals.var_t4 * locals.var_t4))))), ((locals.var_t10_dn10 * assign67310_e104051) + (locals.var_t10 * (-(locals.var_t4_dn10 / (locals.var_t4 * locals.var_t4))))), ((locals.var_t10_dn11 * assign67310_e104051) + (locals.var_t10 * (-(locals.var_t4_dn11 / (locals.var_t4 * locals.var_t4))))), ((locals.var_t10_dn14 * assign67310_e104051) + (locals.var_t10 * (-(locals.var_t4_dn14 / (locals.var_t4 * locals.var_t4))))),)
    } else {
        (locals.var_t11, locals.var_t11_dn0, locals.var_t11_dn2, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11, locals.var_t11_dn14,)
    }
};
        locals.var_t11 = assign67310_e104054;
        locals.var_t11_dn0 = assign67310_e104054_d_n0;
        locals.var_t11_dn2 = assign67310_e104054_d_n2;
        locals.var_t11_dn4 = assign67310_e104054_d_n4;
        locals.var_t11_dn5 = assign67310_e104054_d_n5;
        locals.var_t11_dn6 = assign67310_e104054_d_n6;
        locals.var_t11_dn7 = assign67310_e104054_d_n7;
        locals.var_t11_dn8 = assign67310_e104054_d_n8;
        locals.var_t11_dn9 = assign67310_e104054_d_n9;
        locals.var_t11_dn10 = assign67310_e104054_d_n10;
        locals.var_t11_dn11 = assign67310_e104054_d_n11;
        locals.var_t11_dn14 = assign67310_e104054_d_n14;

        let (assign67320_e104063, assign67320_e104063_d_n0, assign67320_e104063_d_n2, assign67320_e104063_d_n4, assign67320_e104063_d_n5, assign67320_e104063_d_n6, assign67320_e104063_d_n7, assign67320_e104063_d_n8, assign67320_e104063_d_n9, assign67320_e104063_d_n10, assign67320_e104063_d_n11, assign67320_e104063_d_n14,) = {
    if ((locals.var_guard1584 == 0.0) && (locals.var_guard1593 != 0.0)) {
        let assign67320_e104061: f64 = (locals.var_t4 * locals.var_t10);
        (assign67320_e104061, ((locals.var_t4_dn0 * locals.var_t10) + (locals.var_t4 * locals.var_t10_dn0)), ((locals.var_t4_dn2 * locals.var_t10) + (locals.var_t4 * locals.var_t10_dn2)), ((locals.var_t4_dn4 * locals.var_t10) + (locals.var_t4 * locals.var_t10_dn4)), ((locals.var_t4_dn5 * locals.var_t10) + (locals.var_t4 * locals.var_t10_dn5)), ((locals.var_t4_dn6 * locals.var_t10) + (locals.var_t4 * locals.var_t10_dn6)), ((locals.var_t4_dn7 * locals.var_t10) + (locals.var_t4 * locals.var_t10_dn7)), ((locals.var_t4_dn8 * locals.var_t10) + (locals.var_t4 * locals.var_t10_dn8)), ((locals.var_t4_dn9 * locals.var_t10) + (locals.var_t4 * locals.var_t10_dn9)), ((locals.var_t4_dn10 * locals.var_t10) + (locals.var_t4 * locals.var_t10_dn10)), ((locals.var_t4_dn11 * locals.var_t10) + (locals.var_t4 * locals.var_t10_dn11)), ((locals.var_t4_dn14 * locals.var_t10) + (locals.var_t4 * locals.var_t10_dn14)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign67320_e104063;
        locals.var_t3_dn0 = assign67320_e104063_d_n0;
        locals.var_t3_dn2 = assign67320_e104063_d_n2;
        locals.var_t3_dn4 = assign67320_e104063_d_n4;
        locals.var_t3_dn5 = assign67320_e104063_d_n5;
        locals.var_t3_dn6 = assign67320_e104063_d_n6;
        locals.var_t3_dn7 = assign67320_e104063_d_n7;
        locals.var_t3_dn8 = assign67320_e104063_d_n8;
        locals.var_t3_dn9 = assign67320_e104063_d_n9;
        locals.var_t3_dn10 = assign67320_e104063_d_n10;
        locals.var_t3_dn11 = assign67320_e104063_d_n11;
        locals.var_t3_dn14 = assign67320_e104063_d_n14;

        let (assign67330_e104072, assign67330_e104072_d_n0, assign67330_e104072_d_n2, assign67330_e104072_d_n4, assign67330_e104072_d_n5, assign67330_e104072_d_n6, assign67330_e104072_d_n7, assign67330_e104072_d_n8, assign67330_e104072_d_n9, assign67330_e104072_d_n10, assign67330_e104072_d_n11, assign67330_e104072_d_n14,) = {
    if ((locals.var_guard1584 == 0.0) && (locals.var_guard1593 != 0.0)) {
        let assign67330_e104070: f64 = (locals.var_t0 - locals.var_t3);
        (assign67330_e104070, (locals.var_t0_dn0 - locals.var_t3_dn0), (locals.var_t0_dn2 - locals.var_t3_dn2), (locals.var_t0_dn4 - locals.var_t3_dn4), (locals.var_t0_dn5 - locals.var_t3_dn5), (locals.var_t0_dn6 - locals.var_t3_dn6), (locals.var_t0_dn7 - locals.var_t3_dn7), (locals.var_t0_dn8 - locals.var_t3_dn8), (locals.var_t0_dn9 - locals.var_t3_dn9), (locals.var_t0_dn10 - locals.var_t3_dn10), (locals.var_t0_dn11 - locals.var_t3_dn11), (locals.var_t0_dn14 - locals.var_t3_dn14),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign67330_e104072;
        locals.var_t0_dn0 = assign67330_e104072_d_n0;
        locals.var_t0_dn2 = assign67330_e104072_d_n2;
        locals.var_t0_dn4 = assign67330_e104072_d_n4;
        locals.var_t0_dn5 = assign67330_e104072_d_n5;
        locals.var_t0_dn6 = assign67330_e104072_d_n6;
        locals.var_t0_dn7 = assign67330_e104072_d_n7;
        locals.var_t0_dn8 = assign67330_e104072_d_n8;
        locals.var_t0_dn9 = assign67330_e104072_d_n9;
        locals.var_t0_dn10 = assign67330_e104072_d_n10;
        locals.var_t0_dn11 = assign67330_e104072_d_n11;
        locals.var_t0_dn14 = assign67330_e104072_d_n14;

        let (assign67340_e104088, assign67340_e104088_d_n0, assign67340_e104088_d_n2, assign67340_e104088_d_n4, assign67340_e104088_d_n5, assign67340_e104088_d_n6, assign67340_e104088_d_n7, assign67340_e104088_d_n8, assign67340_e104088_d_n9, assign67340_e104088_d_n10, assign67340_e104088_d_n11, assign67340_e104088_d_n14,) = {
    if ((locals.var_guard1584 == 0.0) && (locals.var_guard1593 != 0.0)) {
        let assign67340_e104079: f64 = (locals.var_t0 * locals.var_t0);
        let assign67340_e104082: f64 = (4.0 * 0.01);
        let assign67340_e104084: f64 = (assign67340_e104082 * 0.01);
        let assign67340_e104085: f64 = (assign67340_e104079 + assign67340_e104084);
        let assign67340_e104086: f64 = (assign67340_e104085).sqrt();
        (assign67340_e104086, (((locals.var_t0_dn0 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn0)) / (2.0 * assign67340_e104086)), (((locals.var_t0_dn2 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn2)) / (2.0 * assign67340_e104086)), (((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)) / (2.0 * assign67340_e104086)), (((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)) / (2.0 * assign67340_e104086)), (((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)) / (2.0 * assign67340_e104086)), (((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)) / (2.0 * assign67340_e104086)), (((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)) / (2.0 * assign67340_e104086)), (((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)) / (2.0 * assign67340_e104086)), (((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)) / (2.0 * assign67340_e104086)), (((locals.var_t0_dn11 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn11)) / (2.0 * assign67340_e104086)), (((locals.var_t0_dn14 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn14)) / (2.0 * assign67340_e104086)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign67340_e104088;
        locals.var_tmf2_dn0 = assign67340_e104088_d_n0;
        locals.var_tmf2_dn2 = assign67340_e104088_d_n2;
        locals.var_tmf2_dn4 = assign67340_e104088_d_n4;
        locals.var_tmf2_dn5 = assign67340_e104088_d_n5;
        locals.var_tmf2_dn6 = assign67340_e104088_d_n6;
        locals.var_tmf2_dn7 = assign67340_e104088_d_n7;
        locals.var_tmf2_dn8 = assign67340_e104088_d_n8;
        locals.var_tmf2_dn9 = assign67340_e104088_d_n9;
        locals.var_tmf2_dn10 = assign67340_e104088_d_n10;
        locals.var_tmf2_dn11 = assign67340_e104088_d_n11;
        locals.var_tmf2_dn14 = assign67340_e104088_d_n14;

        let (assign67350_e104101, assign67350_e104101_d_n0, assign67350_e104101_d_n2, assign67350_e104101_d_n4, assign67350_e104101_d_n5, assign67350_e104101_d_n6, assign67350_e104101_d_n7, assign67350_e104101_d_n8, assign67350_e104101_d_n9, assign67350_e104101_d_n10, assign67350_e104101_d_n11, assign67350_e104101_d_n14,) = {
    if ((locals.var_guard1584 == 0.0) && (locals.var_guard1593 != 0.0)) {
        let assign67350_e104097: f64 = (locals.var_t0 / locals.var_tmf2);
        let assign67350_e104098: f64 = (1.0 + assign67350_e104097);
        let assign67350_e104099: f64 = (0.5 * assign67350_e104098);
        (assign67350_e104099, (0.5 * (((locals.var_t0_dn0 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn2 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn4 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn5 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn6 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn7 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn8 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn9 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn10 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn11 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn14 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign67350_e104101;
        locals.var_t9_dn0 = assign67350_e104101_d_n0;
        locals.var_t9_dn2 = assign67350_e104101_d_n2;
        locals.var_t9_dn4 = assign67350_e104101_d_n4;
        locals.var_t9_dn5 = assign67350_e104101_d_n5;
        locals.var_t9_dn6 = assign67350_e104101_d_n6;
        locals.var_t9_dn7 = assign67350_e104101_d_n7;
        locals.var_t9_dn8 = assign67350_e104101_d_n8;
        locals.var_t9_dn9 = assign67350_e104101_d_n9;
        locals.var_t9_dn10 = assign67350_e104101_d_n10;
        locals.var_t9_dn11 = assign67350_e104101_d_n11;
        locals.var_t9_dn14 = assign67350_e104101_d_n14;

        let (assign67360_e104112, assign67360_e104112_d_n0, assign67360_e104112_d_n2, assign67360_e104112_d_n4, assign67360_e104112_d_n5, assign67360_e104112_d_n6, assign67360_e104112_d_n7, assign67360_e104112_d_n8, assign67360_e104112_d_n9, assign67360_e104112_d_n10, assign67360_e104112_d_n11, assign67360_e104112_d_n14,) = {
    if ((locals.var_guard1584 == 0.0) && (locals.var_guard1593 != 0.0)) {
        let assign67360_e104109: f64 = (locals.var_t0 + locals.var_tmf2);
        let assign67360_e104110: f64 = (0.5 * assign67360_e104109);
        (assign67360_e104110, (0.5 * (locals.var_t0_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t0_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t0_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t0_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t0_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t0_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t0_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t0_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t0_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t0_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_t0_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign67360_e104112;
        locals.var_t0_dn0 = assign67360_e104112_d_n0;
        locals.var_t0_dn2 = assign67360_e104112_d_n2;
        locals.var_t0_dn4 = assign67360_e104112_d_n4;
        locals.var_t0_dn5 = assign67360_e104112_d_n5;
        locals.var_t0_dn6 = assign67360_e104112_d_n6;
        locals.var_t0_dn7 = assign67360_e104112_d_n7;
        locals.var_t0_dn8 = assign67360_e104112_d_n8;
        locals.var_t0_dn9 = assign67360_e104112_d_n9;
        locals.var_t0_dn10 = assign67360_e104112_d_n10;
        locals.var_t0_dn11 = assign67360_e104112_d_n11;
        locals.var_t0_dn14 = assign67360_e104112_d_n14;

        let assign67370_e104115: f64 = if locals.var_t0 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1596 = assign67370_e104115;

        let (assign67380_e104124, assign67380_e104124_d_n0, assign67380_e104124_d_n2, assign67380_e104124_d_n4, assign67380_e104124_d_n5, assign67380_e104124_d_n6, assign67380_e104124_d_n7, assign67380_e104124_d_n8, assign67380_e104124_d_n9, assign67380_e104124_d_n10, assign67380_e104124_d_n11, assign67380_e104124_d_n14,) = {
    if (((locals.var_guard1584 == 0.0) && (locals.var_guard1593 != 0.0)) && (locals.var_guard1596 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign67380_e104124;
        locals.var_t0_dn0 = assign67380_e104124_d_n0;
        locals.var_t0_dn2 = assign67380_e104124_d_n2;
        locals.var_t0_dn4 = assign67380_e104124_d_n4;
        locals.var_t0_dn5 = assign67380_e104124_d_n5;
        locals.var_t0_dn6 = assign67380_e104124_d_n6;
        locals.var_t0_dn7 = assign67380_e104124_d_n7;
        locals.var_t0_dn8 = assign67380_e104124_d_n8;
        locals.var_t0_dn9 = assign67380_e104124_d_n9;
        locals.var_t0_dn10 = assign67380_e104124_d_n10;
        locals.var_t0_dn11 = assign67380_e104124_d_n11;
        locals.var_t0_dn14 = assign67380_e104124_d_n14;

        let (assign67390_e104133, assign67390_e104133_d_n0, assign67390_e104133_d_n2, assign67390_e104133_d_n4, assign67390_e104133_d_n5, assign67390_e104133_d_n6, assign67390_e104133_d_n7, assign67390_e104133_d_n8, assign67390_e104133_d_n9, assign67390_e104133_d_n10, assign67390_e104133_d_n11, assign67390_e104133_d_n14,) = {
    if (((locals.var_guard1584 == 0.0) && (locals.var_guard1593 != 0.0)) && (locals.var_guard1596 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign67390_e104133;
        locals.var_t9_dn0 = assign67390_e104133_d_n0;
        locals.var_t9_dn2 = assign67390_e104133_d_n2;
        locals.var_t9_dn4 = assign67390_e104133_d_n4;
        locals.var_t9_dn5 = assign67390_e104133_d_n5;
        locals.var_t9_dn6 = assign67390_e104133_d_n6;
        locals.var_t9_dn7 = assign67390_e104133_d_n7;
        locals.var_t9_dn8 = assign67390_e104133_d_n8;
        locals.var_t9_dn9 = assign67390_e104133_d_n9;
        locals.var_t9_dn10 = assign67390_e104133_d_n10;
        locals.var_t9_dn11 = assign67390_e104133_d_n11;
        locals.var_t9_dn14 = assign67390_e104133_d_n14;

        let (assign67400_e104142, assign67400_e104142_d_n0, assign67400_e104142_d_n2, assign67400_e104142_d_n4, assign67400_e104142_d_n5, assign67400_e104142_d_n6, assign67400_e104142_d_n7, assign67400_e104142_d_n8, assign67400_e104142_d_n9, assign67400_e104142_d_n10, assign67400_e104142_d_n11, assign67400_e104142_d_n14,) = {
    if ((locals.var_guard1584 == 0.0) && (locals.var_guard1593 != 0.0)) {
        let assign67400_e104140: f64 = (locals.var_t0 + 1e-25);
        (assign67400_e104140, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign67400_e104142;
        locals.var_t0_dn0 = assign67400_e104142_d_n0;
        locals.var_t0_dn2 = assign67400_e104142_d_n2;
        locals.var_t0_dn4 = assign67400_e104142_d_n4;
        locals.var_t0_dn5 = assign67400_e104142_d_n5;
        locals.var_t0_dn6 = assign67400_e104142_d_n6;
        locals.var_t0_dn7 = assign67400_e104142_d_n7;
        locals.var_t0_dn8 = assign67400_e104142_d_n8;
        locals.var_t0_dn9 = assign67400_e104142_d_n9;
        locals.var_t0_dn10 = assign67400_e104142_d_n10;
        locals.var_t0_dn11 = assign67400_e104142_d_n11;
        locals.var_t0_dn14 = assign67400_e104142_d_n14;

        let (assign67410_e104153, assign67410_e104153_d_n0, assign67410_e104153_d_n2, assign67410_e104153_d_n4, assign67410_e104153_d_n5, assign67410_e104153_d_n6, assign67410_e104153_d_n7, assign67410_e104153_d_n8, assign67410_e104153_d_n9, assign67410_e104153_d_n10, assign67410_e104153_d_n11, assign67410_e104153_d_n14,) = {
    if ((locals.var_guard1584 == 0.0) && (locals.var_guard1593 != 0.0)) {
        let assign67410_e104150: f64 = (locals.var_t0 * locals.var_t1);
        let assign67410_e104151: f64 = (1.0 / assign67410_e104150);
        (assign67410_e104151, (-(((locals.var_t0_dn0 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn0)) / (assign67410_e104150 * assign67410_e104150))), (-(((locals.var_t0_dn2 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn2)) / (assign67410_e104150 * assign67410_e104150))), (-(((locals.var_t0_dn4 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn4)) / (assign67410_e104150 * assign67410_e104150))), (-(((locals.var_t0_dn5 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn5)) / (assign67410_e104150 * assign67410_e104150))), (-(((locals.var_t0_dn6 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn6)) / (assign67410_e104150 * assign67410_e104150))), (-(((locals.var_t0_dn7 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn7)) / (assign67410_e104150 * assign67410_e104150))), (-(((locals.var_t0_dn8 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn8)) / (assign67410_e104150 * assign67410_e104150))), (-(((locals.var_t0_dn9 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn9)) / (assign67410_e104150 * assign67410_e104150))), (-(((locals.var_t0_dn10 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn10)) / (assign67410_e104150 * assign67410_e104150))), (-(((locals.var_t0_dn11 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn11)) / (assign67410_e104150 * assign67410_e104150))), (-(((locals.var_t0_dn14 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn14)) / (assign67410_e104150 * assign67410_e104150))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign67410_e104153;
        locals.var_t4_dn0 = assign67410_e104153_d_n0;
        locals.var_t4_dn2 = assign67410_e104153_d_n2;
        locals.var_t4_dn4 = assign67410_e104153_d_n4;
        locals.var_t4_dn5 = assign67410_e104153_d_n5;
        locals.var_t4_dn6 = assign67410_e104153_d_n6;
        locals.var_t4_dn7 = assign67410_e104153_d_n7;
        locals.var_t4_dn8 = assign67410_e104153_d_n8;
        locals.var_t4_dn9 = assign67410_e104153_d_n9;
        locals.var_t4_dn10 = assign67410_e104153_d_n10;
        locals.var_t4_dn11 = assign67410_e104153_d_n11;
        locals.var_t4_dn14 = assign67410_e104153_d_n14;

        let (assign67420_e104162, assign67420_e104162_d_n0, assign67420_e104162_d_n2, assign67420_e104162_d_n4, assign67420_e104162_d_n5, assign67420_e104162_d_n6, assign67420_e104162_d_n7, assign67420_e104162_d_n8, assign67420_e104162_d_n9, assign67420_e104162_d_n10, assign67420_e104162_d_n11, assign67420_e104162_d_n14,) = {
    if ((locals.var_guard1584 == 0.0) && (locals.var_guard1593 != 0.0)) {
        let assign67420_e104160: f64 = (locals.var_ldrift0 * locals.var_mks_subld2);
        (assign67420_e104160, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign67420_e104162;
        locals.var_t7_dn0 = assign67420_e104162_d_n0;
        locals.var_t7_dn2 = assign67420_e104162_d_n2;
        locals.var_t7_dn4 = assign67420_e104162_d_n4;
        locals.var_t7_dn5 = assign67420_e104162_d_n5;
        locals.var_t7_dn6 = assign67420_e104162_d_n6;
        locals.var_t7_dn7 = assign67420_e104162_d_n7;
        locals.var_t7_dn8 = assign67420_e104162_d_n8;
        locals.var_t7_dn9 = assign67420_e104162_d_n9;
        locals.var_t7_dn10 = assign67420_e104162_d_n10;
        locals.var_t7_dn11 = assign67420_e104162_d_n11;
        locals.var_t7_dn14 = assign67420_e104162_d_n14;

        let (assign67430_e104173, assign67430_e104173_d_n0, assign67430_e104173_d_n2, assign67430_e104173_d_n4, assign67430_e104173_d_n5, assign67430_e104173_d_n6, assign67430_e104173_d_n7, assign67430_e104173_d_n8, assign67430_e104173_d_n9, assign67430_e104173_d_n10, assign67430_e104173_d_n11, assign67430_e104173_d_n14,) = {
    if ((locals.var_guard1584 == 0.0) && (locals.var_guard1593 != 0.0)) {
        let assign67430_e104168: f64 = (-locals.var_t7);
        let assign67430_e104170: f64 = (assign67430_e104168 * locals.var_t4);
        let assign67430_e104171: f64 = (assign67430_e104170).exp();
        (assign67430_e104171, (assign67430_e104171 * (((-locals.var_t7_dn0) * locals.var_t4) + (assign67430_e104168 * locals.var_t4_dn0))), (assign67430_e104171 * (((-locals.var_t7_dn2) * locals.var_t4) + (assign67430_e104168 * locals.var_t4_dn2))), (assign67430_e104171 * (((-locals.var_t7_dn4) * locals.var_t4) + (assign67430_e104168 * locals.var_t4_dn4))), (assign67430_e104171 * (((-locals.var_t7_dn5) * locals.var_t4) + (assign67430_e104168 * locals.var_t4_dn5))), (assign67430_e104171 * (((-locals.var_t7_dn6) * locals.var_t4) + (assign67430_e104168 * locals.var_t4_dn6))), (assign67430_e104171 * (((-locals.var_t7_dn7) * locals.var_t4) + (assign67430_e104168 * locals.var_t4_dn7))), (assign67430_e104171 * (((-locals.var_t7_dn8) * locals.var_t4) + (assign67430_e104168 * locals.var_t4_dn8))), (assign67430_e104171 * (((-locals.var_t7_dn9) * locals.var_t4) + (assign67430_e104168 * locals.var_t4_dn9))), (assign67430_e104171 * (((-locals.var_t7_dn10) * locals.var_t4) + (assign67430_e104168 * locals.var_t4_dn10))), (assign67430_e104171 * (((-locals.var_t7_dn11) * locals.var_t4) + (assign67430_e104168 * locals.var_t4_dn11))), (assign67430_e104171 * (((-locals.var_t7_dn14) * locals.var_t4) + (assign67430_e104168 * locals.var_t4_dn14))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign67430_e104173;
        locals.var_t2_dn0 = assign67430_e104173_d_n0;
        locals.var_t2_dn2 = assign67430_e104173_d_n2;
        locals.var_t2_dn4 = assign67430_e104173_d_n4;
        locals.var_t2_dn5 = assign67430_e104173_d_n5;
        locals.var_t2_dn6 = assign67430_e104173_d_n6;
        locals.var_t2_dn7 = assign67430_e104173_d_n7;
        locals.var_t2_dn8 = assign67430_e104173_d_n8;
        locals.var_t2_dn9 = assign67430_e104173_d_n9;
        locals.var_t2_dn10 = assign67430_e104173_d_n10;
        locals.var_t2_dn11 = assign67430_e104173_d_n11;
        locals.var_t2_dn14 = assign67430_e104173_d_n14;

        let (assign67440_e104186, assign67440_e104186_d_n0, assign67440_e104186_d_n2, assign67440_e104186_d_n4, assign67440_e104186_d_n5, assign67440_e104186_d_n6, assign67440_e104186_d_n7, assign67440_e104186_d_n8, assign67440_e104186_d_n9, assign67440_e104186_d_n10, assign67440_e104186_d_n11, assign67440_e104186_d_n14,) = {
    if ((locals.var_guard1584 == 0.0) && (locals.var_guard1593 != 0.0)) {
        let assign67440_e104180: f64 = (locals.var_t7 * locals.var_t2);
        let assign67440_e104182: f64 = (assign67440_e104180 * locals.var_t4);
        let assign67440_e104184: f64 = (assign67440_e104182 * locals.var_t4);
        (assign67440_e104184, ((((((locals.var_t7_dn0 * locals.var_t2) + (locals.var_t7 * locals.var_t2_dn0)) * locals.var_t4) + (assign67440_e104180 * locals.var_t4_dn0)) * locals.var_t4) + (assign67440_e104182 * locals.var_t4_dn0)), ((((((locals.var_t7_dn2 * locals.var_t2) + (locals.var_t7 * locals.var_t2_dn2)) * locals.var_t4) + (assign67440_e104180 * locals.var_t4_dn2)) * locals.var_t4) + (assign67440_e104182 * locals.var_t4_dn2)), ((((((locals.var_t7_dn4 * locals.var_t2) + (locals.var_t7 * locals.var_t2_dn4)) * locals.var_t4) + (assign67440_e104180 * locals.var_t4_dn4)) * locals.var_t4) + (assign67440_e104182 * locals.var_t4_dn4)), ((((((locals.var_t7_dn5 * locals.var_t2) + (locals.var_t7 * locals.var_t2_dn5)) * locals.var_t4) + (assign67440_e104180 * locals.var_t4_dn5)) * locals.var_t4) + (assign67440_e104182 * locals.var_t4_dn5)), ((((((locals.var_t7_dn6 * locals.var_t2) + (locals.var_t7 * locals.var_t2_dn6)) * locals.var_t4) + (assign67440_e104180 * locals.var_t4_dn6)) * locals.var_t4) + (assign67440_e104182 * locals.var_t4_dn6)), ((((((locals.var_t7_dn7 * locals.var_t2) + (locals.var_t7 * locals.var_t2_dn7)) * locals.var_t4) + (assign67440_e104180 * locals.var_t4_dn7)) * locals.var_t4) + (assign67440_e104182 * locals.var_t4_dn7)), ((((((locals.var_t7_dn8 * locals.var_t2) + (locals.var_t7 * locals.var_t2_dn8)) * locals.var_t4) + (assign67440_e104180 * locals.var_t4_dn8)) * locals.var_t4) + (assign67440_e104182 * locals.var_t4_dn8)), ((((((locals.var_t7_dn9 * locals.var_t2) + (locals.var_t7 * locals.var_t2_dn9)) * locals.var_t4) + (assign67440_e104180 * locals.var_t4_dn9)) * locals.var_t4) + (assign67440_e104182 * locals.var_t4_dn9)), ((((((locals.var_t7_dn10 * locals.var_t2) + (locals.var_t7 * locals.var_t2_dn10)) * locals.var_t4) + (assign67440_e104180 * locals.var_t4_dn10)) * locals.var_t4) + (assign67440_e104182 * locals.var_t4_dn10)), ((((((locals.var_t7_dn11 * locals.var_t2) + (locals.var_t7 * locals.var_t2_dn11)) * locals.var_t4) + (assign67440_e104180 * locals.var_t4_dn11)) * locals.var_t4) + (assign67440_e104182 * locals.var_t4_dn11)), ((((((locals.var_t7_dn14 * locals.var_t2) + (locals.var_t7 * locals.var_t2_dn14)) * locals.var_t4) + (assign67440_e104180 * locals.var_t4_dn14)) * locals.var_t4) + (assign67440_e104182 * locals.var_t4_dn14)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign67440_e104186;
        locals.var_t6_dn0 = assign67440_e104186_d_n0;
        locals.var_t6_dn2 = assign67440_e104186_d_n2;
        locals.var_t6_dn4 = assign67440_e104186_d_n4;
        locals.var_t6_dn5 = assign67440_e104186_d_n5;
        locals.var_t6_dn6 = assign67440_e104186_d_n6;
        locals.var_t6_dn7 = assign67440_e104186_d_n7;
        locals.var_t6_dn8 = assign67440_e104186_d_n8;
        locals.var_t6_dn9 = assign67440_e104186_d_n9;
        locals.var_t6_dn10 = assign67440_e104186_d_n10;
        locals.var_t6_dn11 = assign67440_e104186_d_n11;
        locals.var_t6_dn14 = assign67440_e104186_d_n14;

        let (assign67450_e104199, assign67450_e104199_d_n0, assign67450_e104199_d_n2, assign67450_e104199_d_n4, assign67450_e104199_d_n5, assign67450_e104199_d_n6, assign67450_e104199_d_n7, assign67450_e104199_d_n8, assign67450_e104199_d_n9, assign67450_e104199_d_n10, assign67450_e104199_d_n11, assign67450_e104199_d_n14,) = {
    if ((locals.var_guard1584 == 0.0) && (locals.var_guard1593 != 0.0)) {
        let assign67450_e104193: f64 = (locals.var_uc_subld1 * locals.var_ids);
        let assign67450_e104195: f64 = (assign67450_e104193 * locals.var_t0);
        let assign67450_e104197: f64 = (assign67450_e104195 * locals.var_t2);
        (assign67450_e104197, (((((locals.var_uc_subld1 * locals.var_ids_dn0) * locals.var_t0) + (assign67450_e104193 * locals.var_t0_dn0)) * locals.var_t2) + (assign67450_e104195 * locals.var_t2_dn0)), (((((locals.var_uc_subld1 * locals.var_ids_dn2) * locals.var_t0) + (assign67450_e104193 * locals.var_t0_dn2)) * locals.var_t2) + (assign67450_e104195 * locals.var_t2_dn2)), (((((locals.var_uc_subld1 * locals.var_ids_dn4) * locals.var_t0) + (assign67450_e104193 * locals.var_t0_dn4)) * locals.var_t2) + (assign67450_e104195 * locals.var_t2_dn4)), (((((locals.var_uc_subld1 * locals.var_ids_dn5) * locals.var_t0) + (assign67450_e104193 * locals.var_t0_dn5)) * locals.var_t2) + (assign67450_e104195 * locals.var_t2_dn5)), (((((locals.var_uc_subld1 * locals.var_ids_dn6) * locals.var_t0) + (assign67450_e104193 * locals.var_t0_dn6)) * locals.var_t2) + (assign67450_e104195 * locals.var_t2_dn6)), (((((locals.var_uc_subld1 * locals.var_ids_dn7) * locals.var_t0) + (assign67450_e104193 * locals.var_t0_dn7)) * locals.var_t2) + (assign67450_e104195 * locals.var_t2_dn7)), (((((locals.var_uc_subld1 * locals.var_ids_dn8) * locals.var_t0) + (assign67450_e104193 * locals.var_t0_dn8)) * locals.var_t2) + (assign67450_e104195 * locals.var_t2_dn8)), (((((locals.var_uc_subld1 * locals.var_ids_dn9) * locals.var_t0) + (assign67450_e104193 * locals.var_t0_dn9)) * locals.var_t2) + (assign67450_e104195 * locals.var_t2_dn9)), (((((locals.var_uc_subld1 * locals.var_ids_dn10) * locals.var_t0) + (assign67450_e104193 * locals.var_t0_dn10)) * locals.var_t2) + (assign67450_e104195 * locals.var_t2_dn10)), (((((locals.var_uc_subld1 * locals.var_ids_dn11) * locals.var_t0) + (assign67450_e104193 * locals.var_t0_dn11)) * locals.var_t2) + (assign67450_e104195 * locals.var_t2_dn11)), (((((locals.var_uc_subld1 * locals.var_ids_dn14) * locals.var_t0) + (assign67450_e104193 * locals.var_t0_dn14)) * locals.var_t2) + (assign67450_e104195 * locals.var_t2_dn14)),)
    } else {
        (locals.var_isubld, locals.var_isubld_dn0, locals.var_isubld_dn2, locals.var_isubld_dn4, locals.var_isubld_dn5, locals.var_isubld_dn6, locals.var_isubld_dn7, locals.var_isubld_dn8, locals.var_isubld_dn9, locals.var_isubld_dn10, locals.var_isubld_dn11, locals.var_isubld_dn14,)
    }
};
        locals.var_isubld = assign67450_e104199;
        locals.var_isubld_dn0 = assign67450_e104199_d_n0;
        locals.var_isubld_dn2 = assign67450_e104199_d_n2;
        locals.var_isubld_dn4 = assign67450_e104199_d_n4;
        locals.var_isubld_dn5 = assign67450_e104199_d_n5;
        locals.var_isubld_dn6 = assign67450_e104199_d_n6;
        locals.var_isubld_dn7 = assign67450_e104199_d_n7;
        locals.var_isubld_dn8 = assign67450_e104199_d_n8;
        locals.var_isubld_dn9 = assign67450_e104199_d_n9;
        locals.var_isubld_dn10 = assign67450_e104199_d_n10;
        locals.var_isubld_dn11 = assign67450_e104199_d_n11;
        locals.var_isubld_dn14 = assign67450_e104199_d_n14;

        let assign67460_e104202: f64 = if p.p45 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1597 = assign67460_e104202;

        let (assign67470_e104206, assign67470_e104206_d_n0, assign67470_e104206_d_n2, assign67470_e104206_d_n4, assign67470_e104206_d_n5, assign67470_e104206_d_n6, assign67470_e104206_d_n7, assign67470_e104206_d_n8, assign67470_e104206_d_n9, assign67470_e104206_d_n10, assign67470_e104206_d_n11, assign67470_e104206_d_n14,) = {
    if (locals.var_guard1597 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ibreakhe, locals.var_ibreakhe_dn0, locals.var_ibreakhe_dn2, locals.var_ibreakhe_dn4, locals.var_ibreakhe_dn5, locals.var_ibreakhe_dn6, locals.var_ibreakhe_dn7, locals.var_ibreakhe_dn8, locals.var_ibreakhe_dn9, locals.var_ibreakhe_dn10, locals.var_ibreakhe_dn11, locals.var_ibreakhe_dn14,)
    }
};
        locals.var_ibreakhe = assign67470_e104206;
        locals.var_ibreakhe_dn0 = assign67470_e104206_d_n0;
        locals.var_ibreakhe_dn2 = assign67470_e104206_d_n2;
        locals.var_ibreakhe_dn4 = assign67470_e104206_d_n4;
        locals.var_ibreakhe_dn5 = assign67470_e104206_d_n5;
        locals.var_ibreakhe_dn6 = assign67470_e104206_d_n6;
        locals.var_ibreakhe_dn7 = assign67470_e104206_d_n7;
        locals.var_ibreakhe_dn8 = assign67470_e104206_d_n8;
        locals.var_ibreakhe_dn9 = assign67470_e104206_d_n9;
        locals.var_ibreakhe_dn10 = assign67470_e104206_d_n10;
        locals.var_ibreakhe_dn11 = assign67470_e104206_d_n11;
        locals.var_ibreakhe_dn14 = assign67470_e104206_d_n14;

        let assign67480_e104210: f64 = (locals.var_vgse - p.p446);
        let assign67480_e104211: f64 = (p.p45 * assign67480_e104210);
        let assign67480_e104213: f64 = if assign67480_e104211 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1598 = assign67480_e104213;

        let (assign67490_e104220, assign67490_e104220_d_n0, assign67490_e104220_d_n2, assign67490_e104220_d_n4, assign67490_e104220_d_n5, assign67490_e104220_d_n6, assign67490_e104220_d_n7, assign67490_e104220_d_n8, assign67490_e104220_d_n9, assign67490_e104220_d_n10, assign67490_e104220_d_n11, assign67490_e104220_d_n14,) = {
    if ((locals.var_guard1597 == 0.0) && (locals.var_guard1598 != 0.0)) {
        (locals.var_hbdceff, locals.var_hbdceff_dn0, locals.var_hbdceff_dn2, locals.var_hbdceff_dn4, locals.var_hbdceff_dn5, locals.var_hbdceff_dn6, locals.var_hbdceff_dn7, locals.var_hbdceff_dn8, locals.var_hbdceff_dn9, locals.var_hbdceff_dn10, locals.var_hbdceff_dn11, locals.var_hbdceff_dn14,)
    } else {
        (locals.var_hbdv, locals.var_hbdv_dn0, locals.var_hbdv_dn2, locals.var_hbdv_dn4, locals.var_hbdv_dn5, locals.var_hbdv_dn6, locals.var_hbdv_dn7, locals.var_hbdv_dn8, locals.var_hbdv_dn9, locals.var_hbdv_dn10, locals.var_hbdv_dn11, locals.var_hbdv_dn14,)
    }
};
        locals.var_hbdv = assign67490_e104220;
        locals.var_hbdv_dn0 = assign67490_e104220_d_n0;
        locals.var_hbdv_dn2 = assign67490_e104220_d_n2;
        locals.var_hbdv_dn4 = assign67490_e104220_d_n4;
        locals.var_hbdv_dn5 = assign67490_e104220_d_n5;
        locals.var_hbdv_dn6 = assign67490_e104220_d_n6;
        locals.var_hbdv_dn7 = assign67490_e104220_d_n7;
        locals.var_hbdv_dn8 = assign67490_e104220_d_n8;
        locals.var_hbdv_dn9 = assign67490_e104220_d_n9;
        locals.var_hbdv_dn10 = assign67490_e104220_d_n10;
        locals.var_hbdv_dn11 = assign67490_e104220_d_n11;
        locals.var_hbdv_dn14 = assign67490_e104220_d_n14;

        let (assign67500_e104236, assign67500_e104236_d_n0, assign67500_e104236_d_n2, assign67500_e104236_d_n4, assign67500_e104236_d_n5, assign67500_e104236_d_n6, assign67500_e104236_d_n7, assign67500_e104236_d_n8, assign67500_e104236_d_n9, assign67500_e104236_d_n10, assign67500_e104236_d_n11, assign67500_e104236_d_n14,) = {
    if ((locals.var_guard1597 == 0.0) && (locals.var_guard1598 == 0.0)) {
        let assign67500_e104229: f64 = (locals.var_vgse - p.p446);
        let assign67500_e104231: f64 = (assign67500_e104229).powf(2.0);
        let assign67500_e104232: f64 = (p.p445 * assign67500_e104231);
        let assign67500_e104234: f64 = (assign67500_e104232 + locals.var_hbdceff);
        (assign67500_e104234, ((p.p445 * if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign67500_e104229).powf(2.0 - 1.0) * locals.var_vgse_dn0)) } } else { (assign67500_e104231 * (2.0 * (locals.var_vgse_dn0 / assign67500_e104229))) }) + locals.var_hbdceff_dn0), ((p.p445 * if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign67500_e104229).powf(2.0 - 1.0) * locals.var_vgse_dn2)) } } else { (assign67500_e104231 * (2.0 * (locals.var_vgse_dn2 / assign67500_e104229))) }) + locals.var_hbdceff_dn2), locals.var_hbdceff_dn4, locals.var_hbdceff_dn5, locals.var_hbdceff_dn6, ((p.p445 * if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign67500_e104229).powf(2.0 - 1.0) * locals.var_vgse_dn7)) } } else { (assign67500_e104231 * (2.0 * (locals.var_vgse_dn7 / assign67500_e104229))) }) + locals.var_hbdceff_dn7), locals.var_hbdceff_dn8, locals.var_hbdceff_dn9, locals.var_hbdceff_dn10, locals.var_hbdceff_dn11, locals.var_hbdceff_dn14,)
    } else {
        (locals.var_hbdv, locals.var_hbdv_dn0, locals.var_hbdv_dn2, locals.var_hbdv_dn4, locals.var_hbdv_dn5, locals.var_hbdv_dn6, locals.var_hbdv_dn7, locals.var_hbdv_dn8, locals.var_hbdv_dn9, locals.var_hbdv_dn10, locals.var_hbdv_dn11, locals.var_hbdv_dn14,)
    }
};
        locals.var_hbdv = assign67500_e104236;
        locals.var_hbdv_dn0 = assign67500_e104236_d_n0;
        locals.var_hbdv_dn2 = assign67500_e104236_d_n2;
        locals.var_hbdv_dn4 = assign67500_e104236_d_n4;
        locals.var_hbdv_dn5 = assign67500_e104236_d_n5;
        locals.var_hbdv_dn6 = assign67500_e104236_d_n6;
        locals.var_hbdv_dn7 = assign67500_e104236_d_n7;
        locals.var_hbdv_dn8 = assign67500_e104236_d_n8;
        locals.var_hbdv_dn9 = assign67500_e104236_d_n9;
        locals.var_hbdv_dn10 = assign67500_e104236_d_n10;
        locals.var_hbdv_dn11 = assign67500_e104236_d_n11;
        locals.var_hbdv_dn14 = assign67500_e104236_d_n14;

        let (assign67510_e104248, assign67510_e104248_d_n0, assign67510_e104248_d_n2, assign67510_e104248_d_n4, assign67510_e104248_d_n5, assign67510_e104248_d_n6, assign67510_e104248_d_n7, assign67510_e104248_d_n8, assign67510_e104248_d_n9, assign67510_e104248_d_n10, assign67510_e104248_d_n11, assign67510_e104248_d_n14,) = {
    if (locals.var_guard1597 == 0.0) {
        let assign67510_e104243: f64 = (locals.var_vdse - locals.var_hbdv);
        let assign67510_e104244: f64 = (locals.var_beta * assign67510_e104243);
        let assign67510_e104245: f64 = { let limited_exp_arg = assign67510_e104244; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign67510_e104246: f64 = (p.p449 * assign67510_e104245);
        (assign67510_e104246, (p.p449 * ({ let limited_exp_arg = assign67510_e104244; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_beta_dn0 * assign67510_e104243) + (locals.var_beta * (locals.var_vdse_dn0 - locals.var_hbdv_dn0))))), (p.p449 * ({ let limited_exp_arg = assign67510_e104244; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_beta_dn2 * assign67510_e104243) + (locals.var_beta * (locals.var_vdse_dn2 - locals.var_hbdv_dn2))))), (p.p449 * ({ let limited_exp_arg = assign67510_e104244; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_beta_dn4 * assign67510_e104243) + (locals.var_beta * (-locals.var_hbdv_dn4))))), (p.p449 * ({ let limited_exp_arg = assign67510_e104244; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_beta_dn5 * assign67510_e104243) + (locals.var_beta * (-locals.var_hbdv_dn5))))), (p.p449 * ({ let limited_exp_arg = assign67510_e104244; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_beta_dn6 * assign67510_e104243) + (locals.var_beta * (-locals.var_hbdv_dn6))))), (p.p449 * ({ let limited_exp_arg = assign67510_e104244; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_beta_dn7 * assign67510_e104243) + (locals.var_beta * (-locals.var_hbdv_dn7))))), (p.p449 * ({ let limited_exp_arg = assign67510_e104244; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_beta_dn8 * assign67510_e104243) + (locals.var_beta * (-locals.var_hbdv_dn8))))), (p.p449 * ({ let limited_exp_arg = assign67510_e104244; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_beta_dn9 * assign67510_e104243) + (locals.var_beta * (-locals.var_hbdv_dn9))))), (p.p449 * ({ let limited_exp_arg = assign67510_e104244; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_beta_dn10 * assign67510_e104243) + (locals.var_beta * (-locals.var_hbdv_dn10))))), (p.p449 * ({ let limited_exp_arg = assign67510_e104244; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_beta_dn11 * assign67510_e104243) + (locals.var_beta * (-locals.var_hbdv_dn11))))), (p.p449 * ({ let limited_exp_arg = assign67510_e104244; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_beta_dn14 * assign67510_e104243) + (locals.var_beta * (-locals.var_hbdv_dn14))))),)
    } else {
        (locals.var_ibreakhe, locals.var_ibreakhe_dn0, locals.var_ibreakhe_dn2, locals.var_ibreakhe_dn4, locals.var_ibreakhe_dn5, locals.var_ibreakhe_dn6, locals.var_ibreakhe_dn7, locals.var_ibreakhe_dn8, locals.var_ibreakhe_dn9, locals.var_ibreakhe_dn10, locals.var_ibreakhe_dn11, locals.var_ibreakhe_dn14,)
    }
};
        locals.var_ibreakhe = assign67510_e104248;
        locals.var_ibreakhe_dn0 = assign67510_e104248_d_n0;
        locals.var_ibreakhe_dn2 = assign67510_e104248_d_n2;
        locals.var_ibreakhe_dn4 = assign67510_e104248_d_n4;
        locals.var_ibreakhe_dn5 = assign67510_e104248_d_n5;
        locals.var_ibreakhe_dn6 = assign67510_e104248_d_n6;
        locals.var_ibreakhe_dn7 = assign67510_e104248_d_n7;
        locals.var_ibreakhe_dn8 = assign67510_e104248_d_n8;
        locals.var_ibreakhe_dn9 = assign67510_e104248_d_n9;
        locals.var_ibreakhe_dn10 = assign67510_e104248_d_n10;
        locals.var_ibreakhe_dn11 = assign67510_e104248_d_n11;
        locals.var_ibreakhe_dn14 = assign67510_e104248_d_n14;

        let assign67520_e104251: f64 = if locals.var_ibreakhe > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1599 = assign67520_e104251;

        let assign67530_e104255: f64 = (100000.0 - 50000.0);
        let assign67530_e104260: f64 = if ((locals.var_ibreakhe > assign67530_e104255) && (50000.0 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1600 = assign67530_e104260;

        let (assign67540_e104270, assign67540_e104270_d_n0, assign67540_e104270_d_n2, assign67540_e104270_d_n4, assign67540_e104270_d_n5, assign67540_e104270_d_n6, assign67540_e104270_d_n7, assign67540_e104270_d_n8, assign67540_e104270_d_n9, assign67540_e104270_d_n10, assign67540_e104270_d_n11, assign67540_e104270_d_n14,) = {
    if ((locals.var_guard1599 != 0.0) && (locals.var_guard1600 != 0.0)) {
        let assign67540_e104266: f64 = (locals.var_ibreakhe - 100000.0);
        let assign67540_e104268: f64 = (assign67540_e104266 + 50000.0);
        (assign67540_e104268, locals.var_ibreakhe_dn0, locals.var_ibreakhe_dn2, locals.var_ibreakhe_dn4, locals.var_ibreakhe_dn5, locals.var_ibreakhe_dn6, locals.var_ibreakhe_dn7, locals.var_ibreakhe_dn8, locals.var_ibreakhe_dn9, locals.var_ibreakhe_dn10, locals.var_ibreakhe_dn11, locals.var_ibreakhe_dn14,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign67540_e104270;
        locals.var_tmf1_dn0 = assign67540_e104270_d_n0;
        locals.var_tmf1_dn2 = assign67540_e104270_d_n2;
        locals.var_tmf1_dn4 = assign67540_e104270_d_n4;
        locals.var_tmf1_dn5 = assign67540_e104270_d_n5;
        locals.var_tmf1_dn6 = assign67540_e104270_d_n6;
        locals.var_tmf1_dn7 = assign67540_e104270_d_n7;
        locals.var_tmf1_dn8 = assign67540_e104270_d_n8;
        locals.var_tmf1_dn9 = assign67540_e104270_d_n9;
        locals.var_tmf1_dn10 = assign67540_e104270_d_n10;
        locals.var_tmf1_dn11 = assign67540_e104270_d_n11;
        locals.var_tmf1_dn14 = assign67540_e104270_d_n14;

        let (assign67550_e104278, assign67550_e104278_d_n0, assign67550_e104278_d_n2, assign67550_e104278_d_n4, assign67550_e104278_d_n5, assign67550_e104278_d_n6, assign67550_e104278_d_n7, assign67550_e104278_d_n8, assign67550_e104278_d_n9, assign67550_e104278_d_n10, assign67550_e104278_d_n11, assign67550_e104278_d_n14,) = {
    if ((locals.var_guard1599 != 0.0) && (locals.var_guard1600 != 0.0)) {
        let assign67550_e104276: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign67550_e104276, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign67550_e104278;
        locals.var_x2_dn0 = assign67550_e104278_d_n0;
        locals.var_x2_dn2 = assign67550_e104278_d_n2;
        locals.var_x2_dn4 = assign67550_e104278_d_n4;
        locals.var_x2_dn5 = assign67550_e104278_d_n5;
        locals.var_x2_dn6 = assign67550_e104278_d_n6;
        locals.var_x2_dn7 = assign67550_e104278_d_n7;
        locals.var_x2_dn8 = assign67550_e104278_d_n8;
        locals.var_x2_dn9 = assign67550_e104278_d_n9;
        locals.var_x2_dn10 = assign67550_e104278_d_n10;
        locals.var_x2_dn11 = assign67550_e104278_d_n11;
        locals.var_x2_dn14 = assign67550_e104278_d_n14;

    }

    pub(super) fn stamp_transient_block_241(
        locals: &mut StampLocals,
    ) {
        let (assign67560_e104286, assign67560_e104286_d_n0, assign67560_e104286_d_n2, assign67560_e104286_d_n4, assign67560_e104286_d_n5, assign67560_e104286_d_n6, assign67560_e104286_d_n7, assign67560_e104286_d_n8, assign67560_e104286_d_n9, assign67560_e104286_d_n10, assign67560_e104286_d_n11, assign67560_e104286_d_n14,) = {
    if ((locals.var_guard1599 != 0.0) && (locals.var_guard1600 != 0.0)) {
        let assign67560_e104284: f64 = (50000.0 * 50000.0);
        (assign67560_e104284, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign67560_e104286;
        locals.var_xmax2_dn0 = assign67560_e104286_d_n0;
        locals.var_xmax2_dn2 = assign67560_e104286_d_n2;
        locals.var_xmax2_dn4 = assign67560_e104286_d_n4;
        locals.var_xmax2_dn5 = assign67560_e104286_d_n5;
        locals.var_xmax2_dn6 = assign67560_e104286_d_n6;
        locals.var_xmax2_dn7 = assign67560_e104286_d_n7;
        locals.var_xmax2_dn8 = assign67560_e104286_d_n8;
        locals.var_xmax2_dn9 = assign67560_e104286_d_n9;
        locals.var_xmax2_dn10 = assign67560_e104286_d_n10;
        locals.var_xmax2_dn11 = assign67560_e104286_d_n11;
        locals.var_xmax2_dn14 = assign67560_e104286_d_n14;

        let (assign67570_e104292, assign67570_e104292_d_n0, assign67570_e104292_d_n2, assign67570_e104292_d_n4, assign67570_e104292_d_n5, assign67570_e104292_d_n6, assign67570_e104292_d_n7, assign67570_e104292_d_n8, assign67570_e104292_d_n9, assign67570_e104292_d_n10, assign67570_e104292_d_n11, assign67570_e104292_d_n14,) = {
    if ((locals.var_guard1599 != 0.0) && (locals.var_guard1600 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign67570_e104292;
        locals.var_xp_dn0 = assign67570_e104292_d_n0;
        locals.var_xp_dn2 = assign67570_e104292_d_n2;
        locals.var_xp_dn4 = assign67570_e104292_d_n4;
        locals.var_xp_dn5 = assign67570_e104292_d_n5;
        locals.var_xp_dn6 = assign67570_e104292_d_n6;
        locals.var_xp_dn7 = assign67570_e104292_d_n7;
        locals.var_xp_dn8 = assign67570_e104292_d_n8;
        locals.var_xp_dn9 = assign67570_e104292_d_n9;
        locals.var_xp_dn10 = assign67570_e104292_d_n10;
        locals.var_xp_dn11 = assign67570_e104292_d_n11;
        locals.var_xp_dn14 = assign67570_e104292_d_n14;

        let (assign67580_e104298, assign67580_e104298_d_n0, assign67580_e104298_d_n2, assign67580_e104298_d_n4, assign67580_e104298_d_n5, assign67580_e104298_d_n6, assign67580_e104298_d_n7, assign67580_e104298_d_n8, assign67580_e104298_d_n9, assign67580_e104298_d_n10, assign67580_e104298_d_n11, assign67580_e104298_d_n14,) = {
    if ((locals.var_guard1599 != 0.0) && (locals.var_guard1600 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign67580_e104298;
        locals.var_xmp_dn0 = assign67580_e104298_d_n0;
        locals.var_xmp_dn2 = assign67580_e104298_d_n2;
        locals.var_xmp_dn4 = assign67580_e104298_d_n4;
        locals.var_xmp_dn5 = assign67580_e104298_d_n5;
        locals.var_xmp_dn6 = assign67580_e104298_d_n6;
        locals.var_xmp_dn7 = assign67580_e104298_d_n7;
        locals.var_xmp_dn8 = assign67580_e104298_d_n8;
        locals.var_xmp_dn9 = assign67580_e104298_d_n9;
        locals.var_xmp_dn10 = assign67580_e104298_d_n10;
        locals.var_xmp_dn11 = assign67580_e104298_d_n11;
        locals.var_xmp_dn14 = assign67580_e104298_d_n14;

        let (assign67590_e104304,) = {
    if ((locals.var_guard1599 != 0.0) && (locals.var_guard1600 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign67590_e104304;

        let (assign67600_e104310,) = {
    if ((locals.var_guard1599 != 0.0) && (locals.var_guard1600 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign67600_e104310;

        let (assign67610_e104316, assign67610_e104316_d_n0, assign67610_e104316_d_n2, assign67610_e104316_d_n4, assign67610_e104316_d_n5, assign67610_e104316_d_n6, assign67610_e104316_d_n7, assign67610_e104316_d_n8, assign67610_e104316_d_n9, assign67610_e104316_d_n10, assign67610_e104316_d_n11, assign67610_e104316_d_n14,) = {
    if ((locals.var_guard1599 != 0.0) && (locals.var_guard1600 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign67610_e104316;
        locals.var_arg_dn0 = assign67610_e104316_d_n0;
        locals.var_arg_dn2 = assign67610_e104316_d_n2;
        locals.var_arg_dn4 = assign67610_e104316_d_n4;
        locals.var_arg_dn5 = assign67610_e104316_d_n5;
        locals.var_arg_dn6 = assign67610_e104316_d_n6;
        locals.var_arg_dn7 = assign67610_e104316_d_n7;
        locals.var_arg_dn8 = assign67610_e104316_d_n8;
        locals.var_arg_dn9 = assign67610_e104316_d_n9;
        locals.var_arg_dn10 = assign67610_e104316_d_n10;
        locals.var_arg_dn11 = assign67610_e104316_d_n11;
        locals.var_arg_dn14 = assign67610_e104316_d_n14;

        let (assign67620_e104322, assign67620_e104322_d_n0, assign67620_e104322_d_n2, assign67620_e104322_d_n4, assign67620_e104322_d_n5, assign67620_e104322_d_n6, assign67620_e104322_d_n7, assign67620_e104322_d_n8, assign67620_e104322_d_n9, assign67620_e104322_d_n10, assign67620_e104322_d_n11, assign67620_e104322_d_n14,) = {
    if ((locals.var_guard1599 != 0.0) && (locals.var_guard1600 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign67620_e104322;
        locals.var_dnm_dn0 = assign67620_e104322_d_n0;
        locals.var_dnm_dn2 = assign67620_e104322_d_n2;
        locals.var_dnm_dn4 = assign67620_e104322_d_n4;
        locals.var_dnm_dn5 = assign67620_e104322_d_n5;
        locals.var_dnm_dn6 = assign67620_e104322_d_n6;
        locals.var_dnm_dn7 = assign67620_e104322_d_n7;
        locals.var_dnm_dn8 = assign67620_e104322_d_n8;
        locals.var_dnm_dn9 = assign67620_e104322_d_n9;
        locals.var_dnm_dn10 = assign67620_e104322_d_n10;
        locals.var_dnm_dn11 = assign67620_e104322_d_n11;
        locals.var_dnm_dn14 = assign67620_e104322_d_n14;

        let (assign67630_e104330, assign67630_e104330_d_n0, assign67630_e104330_d_n2, assign67630_e104330_d_n4, assign67630_e104330_d_n5, assign67630_e104330_d_n6, assign67630_e104330_d_n7, assign67630_e104330_d_n8, assign67630_e104330_d_n9, assign67630_e104330_d_n10, assign67630_e104330_d_n11, assign67630_e104330_d_n14,) = {
    if ((locals.var_guard1599 != 0.0) && (locals.var_guard1600 != 0.0)) {
        let assign67630_e104328: f64 = (locals.var_xp * locals.var_x2);
        (assign67630_e104328, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign67630_e104330;
        locals.var_xp_dn0 = assign67630_e104330_d_n0;
        locals.var_xp_dn2 = assign67630_e104330_d_n2;
        locals.var_xp_dn4 = assign67630_e104330_d_n4;
        locals.var_xp_dn5 = assign67630_e104330_d_n5;
        locals.var_xp_dn6 = assign67630_e104330_d_n6;
        locals.var_xp_dn7 = assign67630_e104330_d_n7;
        locals.var_xp_dn8 = assign67630_e104330_d_n8;
        locals.var_xp_dn9 = assign67630_e104330_d_n9;
        locals.var_xp_dn10 = assign67630_e104330_d_n10;
        locals.var_xp_dn11 = assign67630_e104330_d_n11;
        locals.var_xp_dn14 = assign67630_e104330_d_n14;

        let (assign67640_e104338, assign67640_e104338_d_n0, assign67640_e104338_d_n2, assign67640_e104338_d_n4, assign67640_e104338_d_n5, assign67640_e104338_d_n6, assign67640_e104338_d_n7, assign67640_e104338_d_n8, assign67640_e104338_d_n9, assign67640_e104338_d_n10, assign67640_e104338_d_n11, assign67640_e104338_d_n14,) = {
    if ((locals.var_guard1599 != 0.0) && (locals.var_guard1600 != 0.0)) {
        let assign67640_e104336: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign67640_e104336, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign67640_e104338;
        locals.var_xmp_dn0 = assign67640_e104338_d_n0;
        locals.var_xmp_dn2 = assign67640_e104338_d_n2;
        locals.var_xmp_dn4 = assign67640_e104338_d_n4;
        locals.var_xmp_dn5 = assign67640_e104338_d_n5;
        locals.var_xmp_dn6 = assign67640_e104338_d_n6;
        locals.var_xmp_dn7 = assign67640_e104338_d_n7;
        locals.var_xmp_dn8 = assign67640_e104338_d_n8;
        locals.var_xmp_dn9 = assign67640_e104338_d_n9;
        locals.var_xmp_dn10 = assign67640_e104338_d_n10;
        locals.var_xmp_dn11 = assign67640_e104338_d_n11;
        locals.var_xmp_dn14 = assign67640_e104338_d_n14;

        let (assign67650_e104346, assign67650_e104346_d_n0, assign67650_e104346_d_n2, assign67650_e104346_d_n4, assign67650_e104346_d_n5, assign67650_e104346_d_n6, assign67650_e104346_d_n7, assign67650_e104346_d_n8, assign67650_e104346_d_n9, assign67650_e104346_d_n10, assign67650_e104346_d_n11, assign67650_e104346_d_n14,) = {
    if ((locals.var_guard1599 != 0.0) && (locals.var_guard1600 != 0.0)) {
        let assign67650_e104344: f64 = (locals.var_xp + locals.var_xmp);
        (assign67650_e104344, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign67650_e104346;
        locals.var_arg_dn0 = assign67650_e104346_d_n0;
        locals.var_arg_dn2 = assign67650_e104346_d_n2;
        locals.var_arg_dn4 = assign67650_e104346_d_n4;
        locals.var_arg_dn5 = assign67650_e104346_d_n5;
        locals.var_arg_dn6 = assign67650_e104346_d_n6;
        locals.var_arg_dn7 = assign67650_e104346_d_n7;
        locals.var_arg_dn8 = assign67650_e104346_d_n8;
        locals.var_arg_dn9 = assign67650_e104346_d_n9;
        locals.var_arg_dn10 = assign67650_e104346_d_n10;
        locals.var_arg_dn11 = assign67650_e104346_d_n11;
        locals.var_arg_dn14 = assign67650_e104346_d_n14;

        let (assign67660_e104352, assign67660_e104352_d_n0, assign67660_e104352_d_n2, assign67660_e104352_d_n4, assign67660_e104352_d_n5, assign67660_e104352_d_n6, assign67660_e104352_d_n7, assign67660_e104352_d_n8, assign67660_e104352_d_n9, assign67660_e104352_d_n10, assign67660_e104352_d_n11, assign67660_e104352_d_n14,) = {
    if ((locals.var_guard1599 != 0.0) && (locals.var_guard1600 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign67660_e104352;
        locals.var_dnm_dn0 = assign67660_e104352_d_n0;
        locals.var_dnm_dn2 = assign67660_e104352_d_n2;
        locals.var_dnm_dn4 = assign67660_e104352_d_n4;
        locals.var_dnm_dn5 = assign67660_e104352_d_n5;
        locals.var_dnm_dn6 = assign67660_e104352_d_n6;
        locals.var_dnm_dn7 = assign67660_e104352_d_n7;
        locals.var_dnm_dn8 = assign67660_e104352_d_n8;
        locals.var_dnm_dn9 = assign67660_e104352_d_n9;
        locals.var_dnm_dn10 = assign67660_e104352_d_n10;
        locals.var_dnm_dn11 = assign67660_e104352_d_n11;
        locals.var_dnm_dn14 = assign67660_e104352_d_n14;

        let assign67670_e104367: f64 = if ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1601 = assign67670_e104367;

        let assign67680_e104370: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1602 = assign67680_e104370;

        let (assign67690_e104380,) = {
    if ((((locals.var_guard1599 != 0.0) && (locals.var_guard1600 != 0.0)) && (locals.var_guard1601 != 0.0)) && (locals.var_guard1602 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign67690_e104380;

        let assign67700_e104383: f64 = if 1.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1603 = assign67700_e104383;

        let (assign67710_e104396,) = {
    if (((((locals.var_guard1599 != 0.0) && (locals.var_guard1600 != 0.0)) && (locals.var_guard1601 != 0.0)) && (locals.var_guard1602 == 0.0)) && (locals.var_guard1603 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign67710_e104396;

        let assign67720_e104399: f64 = if 1.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1604 = assign67720_e104399;

        let (assign67730_e104415,) = {
    if ((((((locals.var_guard1599 != 0.0) && (locals.var_guard1600 != 0.0)) && (locals.var_guard1601 != 0.0)) && (locals.var_guard1602 == 0.0)) && (locals.var_guard1603 == 0.0)) && (locals.var_guard1604 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign67730_e104415;

        let assign67740_e104418: f64 = if 1.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1605 = assign67740_e104418;

        let (assign67750_e104437,) = {
    if (((((((locals.var_guard1599 != 0.0) && (locals.var_guard1600 != 0.0)) && (locals.var_guard1601 != 0.0)) && (locals.var_guard1602 == 0.0)) && (locals.var_guard1603 == 0.0)) && (locals.var_guard1604 == 0.0)) && (locals.var_guard1605 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign67750_e104437;

        let (assign67760_e104445,) = {
    if (((locals.var_guard1599 != 0.0) && (locals.var_guard1600 != 0.0)) && (locals.var_guard1601 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign67760_e104445;

        let mut assign67770_loop_guard: usize = 0;
        while {
            let assign67770_cond_e104454: f64 = if ((((locals.var_guard1599 != 0.0) && (locals.var_guard1600 != 0.0)) && (locals.var_guard1601 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign67770_cond_e104454 != 0.0
        } {
            assign67770_loop_guard += 1;
            assert!(assign67770_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign67770_body0_e104463, assign67770_body0_e104463_d_n0, assign67770_body0_e104463_d_n2, assign67770_body0_e104463_d_n4, assign67770_body0_e104463_d_n5, assign67770_body0_e104463_d_n6, assign67770_body0_e104463_d_n7, assign67770_body0_e104463_d_n8, assign67770_body0_e104463_d_n9, assign67770_body0_e104463_d_n10, assign67770_body0_e104463_d_n11, assign67770_body0_e104463_d_n14,) = {
    if (((locals.var_guard1599 != 0.0) && (locals.var_guard1600 != 0.0)) && (locals.var_guard1601 != 0.0)) {
        let assign67770_body0_e104461: f64 = (locals.var_dnm).sqrt();
        (assign67770_body0_e104461, (locals.var_dnm_dn0 / (2.0 * assign67770_body0_e104461)), (locals.var_dnm_dn2 / (2.0 * assign67770_body0_e104461)), (locals.var_dnm_dn4 / (2.0 * assign67770_body0_e104461)), (locals.var_dnm_dn5 / (2.0 * assign67770_body0_e104461)), (locals.var_dnm_dn6 / (2.0 * assign67770_body0_e104461)), (locals.var_dnm_dn7 / (2.0 * assign67770_body0_e104461)), (locals.var_dnm_dn8 / (2.0 * assign67770_body0_e104461)), (locals.var_dnm_dn9 / (2.0 * assign67770_body0_e104461)), (locals.var_dnm_dn10 / (2.0 * assign67770_body0_e104461)), (locals.var_dnm_dn11 / (2.0 * assign67770_body0_e104461)), (locals.var_dnm_dn14 / (2.0 * assign67770_body0_e104461)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign67770_body0_e104463;
            locals.var_dnm_dn0 = assign67770_body0_e104463_d_n0;
            locals.var_dnm_dn2 = assign67770_body0_e104463_d_n2;
            locals.var_dnm_dn4 = assign67770_body0_e104463_d_n4;
            locals.var_dnm_dn5 = assign67770_body0_e104463_d_n5;
            locals.var_dnm_dn6 = assign67770_body0_e104463_d_n6;
            locals.var_dnm_dn7 = assign67770_body0_e104463_d_n7;
            locals.var_dnm_dn8 = assign67770_body0_e104463_d_n8;
            locals.var_dnm_dn9 = assign67770_body0_e104463_d_n9;
            locals.var_dnm_dn10 = assign67770_body0_e104463_d_n10;
            locals.var_dnm_dn11 = assign67770_body0_e104463_d_n11;
            locals.var_dnm_dn14 = assign67770_body0_e104463_d_n14;
            let (assign67770_body1_e104473,) = {
    if (((locals.var_guard1599 != 0.0) && (locals.var_guard1600 != 0.0)) && (locals.var_guard1601 != 0.0)) {
        let assign67770_body1_e104471: f64 = (locals.var_m0 + 1.0);
        (assign67770_body1_e104471,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign67770_body1_e104473;
        }

        let (assign67780_e104493, assign67780_e104493_d_n0, assign67780_e104493_d_n2, assign67780_e104493_d_n4, assign67780_e104493_d_n5, assign67780_e104493_d_n6, assign67780_e104493_d_n7, assign67780_e104493_d_n8, assign67780_e104493_d_n9, assign67780_e104493_d_n10, assign67780_e104493_d_n11, assign67780_e104493_d_n14,) = {
    if (((locals.var_guard1599 != 0.0) && (locals.var_guard1600 != 0.0)) && (locals.var_guard1601 == 0.0)) {
        let (assign67780_e104491, assign67780_e104491_d_n0, assign67780_e104491_d_n2, assign67780_e104491_d_n4, assign67780_e104491_d_n5, assign67780_e104491_d_n6, assign67780_e104491_d_n7, assign67780_e104491_d_n8, assign67780_e104491_d_n9, assign67780_e104491_d_n10, assign67780_e104491_d_n11, assign67780_e104491_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign67780_e104488: f64 = 2.0;
                let assign67780_e104489: f64 = (1.0 / assign67780_e104488);
                let assign67780_e104490: f64 = (locals.var_dnm).powf(assign67780_e104489);
                (assign67780_e104490, if 0.0 == 0.0 && ((assign67780_e104489) as f64).is_finite() && ((assign67780_e104489) as f64).fract() == 0.0 { if assign67780_e104489 == 0.0 { 0.0 } else { (assign67780_e104489 * ((locals.var_dnm).powf(assign67780_e104489 - 1.0) * locals.var_dnm_dn0)) } } else { (assign67780_e104490 * (assign67780_e104489 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign67780_e104489) as f64).is_finite() && ((assign67780_e104489) as f64).fract() == 0.0 { if assign67780_e104489 == 0.0 { 0.0 } else { (assign67780_e104489 * ((locals.var_dnm).powf(assign67780_e104489 - 1.0) * locals.var_dnm_dn2)) } } else { (assign67780_e104490 * (assign67780_e104489 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign67780_e104489) as f64).is_finite() && ((assign67780_e104489) as f64).fract() == 0.0 { if assign67780_e104489 == 0.0 { 0.0 } else { (assign67780_e104489 * ((locals.var_dnm).powf(assign67780_e104489 - 1.0) * locals.var_dnm_dn4)) } } else { (assign67780_e104490 * (assign67780_e104489 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign67780_e104489) as f64).is_finite() && ((assign67780_e104489) as f64).fract() == 0.0 { if assign67780_e104489 == 0.0 { 0.0 } else { (assign67780_e104489 * ((locals.var_dnm).powf(assign67780_e104489 - 1.0) * locals.var_dnm_dn5)) } } else { (assign67780_e104490 * (assign67780_e104489 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign67780_e104489) as f64).is_finite() && ((assign67780_e104489) as f64).fract() == 0.0 { if assign67780_e104489 == 0.0 { 0.0 } else { (assign67780_e104489 * ((locals.var_dnm).powf(assign67780_e104489 - 1.0) * locals.var_dnm_dn6)) } } else { (assign67780_e104490 * (assign67780_e104489 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign67780_e104489) as f64).is_finite() && ((assign67780_e104489) as f64).fract() == 0.0 { if assign67780_e104489 == 0.0 { 0.0 } else { (assign67780_e104489 * ((locals.var_dnm).powf(assign67780_e104489 - 1.0) * locals.var_dnm_dn7)) } } else { (assign67780_e104490 * (assign67780_e104489 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign67780_e104489) as f64).is_finite() && ((assign67780_e104489) as f64).fract() == 0.0 { if assign67780_e104489 == 0.0 { 0.0 } else { (assign67780_e104489 * ((locals.var_dnm).powf(assign67780_e104489 - 1.0) * locals.var_dnm_dn8)) } } else { (assign67780_e104490 * (assign67780_e104489 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign67780_e104489) as f64).is_finite() && ((assign67780_e104489) as f64).fract() == 0.0 { if assign67780_e104489 == 0.0 { 0.0 } else { (assign67780_e104489 * ((locals.var_dnm).powf(assign67780_e104489 - 1.0) * locals.var_dnm_dn9)) } } else { (assign67780_e104490 * (assign67780_e104489 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign67780_e104489) as f64).is_finite() && ((assign67780_e104489) as f64).fract() == 0.0 { if assign67780_e104489 == 0.0 { 0.0 } else { (assign67780_e104489 * ((locals.var_dnm).powf(assign67780_e104489 - 1.0) * locals.var_dnm_dn10)) } } else { (assign67780_e104490 * (assign67780_e104489 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign67780_e104489) as f64).is_finite() && ((assign67780_e104489) as f64).fract() == 0.0 { if assign67780_e104489 == 0.0 { 0.0 } else { (assign67780_e104489 * ((locals.var_dnm).powf(assign67780_e104489 - 1.0) * locals.var_dnm_dn11)) } } else { (assign67780_e104490 * (assign67780_e104489 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign67780_e104489) as f64).is_finite() && ((assign67780_e104489) as f64).fract() == 0.0 { if assign67780_e104489 == 0.0 { 0.0 } else { (assign67780_e104489 * ((locals.var_dnm).powf(assign67780_e104489 - 1.0) * locals.var_dnm_dn14)) } } else { (assign67780_e104490 * (assign67780_e104489 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign67780_e104491, assign67780_e104491_d_n0, assign67780_e104491_d_n2, assign67780_e104491_d_n4, assign67780_e104491_d_n5, assign67780_e104491_d_n6, assign67780_e104491_d_n7, assign67780_e104491_d_n8, assign67780_e104491_d_n9, assign67780_e104491_d_n10, assign67780_e104491_d_n11, assign67780_e104491_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign67780_e104493;
        locals.var_dnm_dn0 = assign67780_e104493_d_n0;
        locals.var_dnm_dn2 = assign67780_e104493_d_n2;
        locals.var_dnm_dn4 = assign67780_e104493_d_n4;
        locals.var_dnm_dn5 = assign67780_e104493_d_n5;
        locals.var_dnm_dn6 = assign67780_e104493_d_n6;
        locals.var_dnm_dn7 = assign67780_e104493_d_n7;
        locals.var_dnm_dn8 = assign67780_e104493_d_n8;
        locals.var_dnm_dn9 = assign67780_e104493_d_n9;
        locals.var_dnm_dn10 = assign67780_e104493_d_n10;
        locals.var_dnm_dn11 = assign67780_e104493_d_n11;
        locals.var_dnm_dn14 = assign67780_e104493_d_n14;

        let (assign67790_e104501, assign67790_e104501_d_n0, assign67790_e104501_d_n2, assign67790_e104501_d_n4, assign67790_e104501_d_n5, assign67790_e104501_d_n6, assign67790_e104501_d_n7, assign67790_e104501_d_n8, assign67790_e104501_d_n9, assign67790_e104501_d_n10, assign67790_e104501_d_n11, assign67790_e104501_d_n14,) = {
    if ((locals.var_guard1599 != 0.0) && (locals.var_guard1600 != 0.0)) {
        let assign67790_e104499: f64 = (1.0 / locals.var_dnm);
        (assign67790_e104499, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign67790_e104501;
        locals.var_dnm_dn0 = assign67790_e104501_d_n0;
        locals.var_dnm_dn2 = assign67790_e104501_d_n2;
        locals.var_dnm_dn4 = assign67790_e104501_d_n4;
        locals.var_dnm_dn5 = assign67790_e104501_d_n5;
        locals.var_dnm_dn6 = assign67790_e104501_d_n6;
        locals.var_dnm_dn7 = assign67790_e104501_d_n7;
        locals.var_dnm_dn8 = assign67790_e104501_d_n8;
        locals.var_dnm_dn9 = assign67790_e104501_d_n9;
        locals.var_dnm_dn10 = assign67790_e104501_d_n10;
        locals.var_dnm_dn11 = assign67790_e104501_d_n11;
        locals.var_dnm_dn14 = assign67790_e104501_d_n14;

        let (assign67800_e104511, assign67800_e104511_d_n0, assign67800_e104511_d_n2, assign67800_e104511_d_n4, assign67800_e104511_d_n5, assign67800_e104511_d_n6, assign67800_e104511_d_n7, assign67800_e104511_d_n8, assign67800_e104511_d_n9, assign67800_e104511_d_n10, assign67800_e104511_d_n11, assign67800_e104511_d_n14,) = {
    if ((locals.var_guard1599 != 0.0) && (locals.var_guard1600 != 0.0)) {
        let assign67800_e104507: f64 = (locals.var_tmf1 * 50000.0);
        let assign67800_e104509: f64 = (assign67800_e104507 * locals.var_dnm);
        (assign67800_e104509, (((locals.var_tmf1_dn0 * 50000.0) * locals.var_dnm) + (assign67800_e104507 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 50000.0) * locals.var_dnm) + (assign67800_e104507 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 50000.0) * locals.var_dnm) + (assign67800_e104507 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 50000.0) * locals.var_dnm) + (assign67800_e104507 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 50000.0) * locals.var_dnm) + (assign67800_e104507 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 50000.0) * locals.var_dnm) + (assign67800_e104507 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 50000.0) * locals.var_dnm) + (assign67800_e104507 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 50000.0) * locals.var_dnm) + (assign67800_e104507 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 50000.0) * locals.var_dnm) + (assign67800_e104507 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * 50000.0) * locals.var_dnm) + (assign67800_e104507 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * 50000.0) * locals.var_dnm) + (assign67800_e104507 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign67800_e104511;
        locals.var_tmf0_dn0 = assign67800_e104511_d_n0;
        locals.var_tmf0_dn2 = assign67800_e104511_d_n2;
        locals.var_tmf0_dn4 = assign67800_e104511_d_n4;
        locals.var_tmf0_dn5 = assign67800_e104511_d_n5;
        locals.var_tmf0_dn6 = assign67800_e104511_d_n6;
        locals.var_tmf0_dn7 = assign67800_e104511_d_n7;
        locals.var_tmf0_dn8 = assign67800_e104511_d_n8;
        locals.var_tmf0_dn9 = assign67800_e104511_d_n9;
        locals.var_tmf0_dn10 = assign67800_e104511_d_n10;
        locals.var_tmf0_dn11 = assign67800_e104511_d_n11;
        locals.var_tmf0_dn14 = assign67800_e104511_d_n14;

        let (assign67810_e104523, assign67810_e104523_d_n0, assign67810_e104523_d_n2, assign67810_e104523_d_n4, assign67810_e104523_d_n5, assign67810_e104523_d_n6, assign67810_e104523_d_n7, assign67810_e104523_d_n8, assign67810_e104523_d_n9, assign67810_e104523_d_n10, assign67810_e104523_d_n11, assign67810_e104523_d_n14,) = {
    if ((locals.var_guard1599 != 0.0) && (locals.var_guard1600 != 0.0)) {
        let assign67810_e104517: f64 = (50000.0 * locals.var_xmp);
        let assign67810_e104519: f64 = (assign67810_e104517 * locals.var_dnm);
        let assign67810_e104521: f64 = (assign67810_e104519 / locals.var_arg);
        (assign67810_e104521, ((((((50000.0 * locals.var_xmp_dn0) * locals.var_dnm) + (assign67810_e104517 * locals.var_dnm_dn0)) * locals.var_arg) - (assign67810_e104519 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((50000.0 * locals.var_xmp_dn2) * locals.var_dnm) + (assign67810_e104517 * locals.var_dnm_dn2)) * locals.var_arg) - (assign67810_e104519 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((50000.0 * locals.var_xmp_dn4) * locals.var_dnm) + (assign67810_e104517 * locals.var_dnm_dn4)) * locals.var_arg) - (assign67810_e104519 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((50000.0 * locals.var_xmp_dn5) * locals.var_dnm) + (assign67810_e104517 * locals.var_dnm_dn5)) * locals.var_arg) - (assign67810_e104519 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((50000.0 * locals.var_xmp_dn6) * locals.var_dnm) + (assign67810_e104517 * locals.var_dnm_dn6)) * locals.var_arg) - (assign67810_e104519 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((50000.0 * locals.var_xmp_dn7) * locals.var_dnm) + (assign67810_e104517 * locals.var_dnm_dn7)) * locals.var_arg) - (assign67810_e104519 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((50000.0 * locals.var_xmp_dn8) * locals.var_dnm) + (assign67810_e104517 * locals.var_dnm_dn8)) * locals.var_arg) - (assign67810_e104519 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((50000.0 * locals.var_xmp_dn9) * locals.var_dnm) + (assign67810_e104517 * locals.var_dnm_dn9)) * locals.var_arg) - (assign67810_e104519 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((50000.0 * locals.var_xmp_dn10) * locals.var_dnm) + (assign67810_e104517 * locals.var_dnm_dn10)) * locals.var_arg) - (assign67810_e104519 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((50000.0 * locals.var_xmp_dn11) * locals.var_dnm) + (assign67810_e104517 * locals.var_dnm_dn11)) * locals.var_arg) - (assign67810_e104519 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((50000.0 * locals.var_xmp_dn14) * locals.var_dnm) + (assign67810_e104517 * locals.var_dnm_dn14)) * locals.var_arg) - (assign67810_e104519 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign67810_e104523;
        locals.var_t0_dn0 = assign67810_e104523_d_n0;
        locals.var_t0_dn2 = assign67810_e104523_d_n2;
        locals.var_t0_dn4 = assign67810_e104523_d_n4;
        locals.var_t0_dn5 = assign67810_e104523_d_n5;
        locals.var_t0_dn6 = assign67810_e104523_d_n6;
        locals.var_t0_dn7 = assign67810_e104523_d_n7;
        locals.var_t0_dn8 = assign67810_e104523_d_n8;
        locals.var_t0_dn9 = assign67810_e104523_d_n9;
        locals.var_t0_dn10 = assign67810_e104523_d_n10;
        locals.var_t0_dn11 = assign67810_e104523_d_n11;
        locals.var_t0_dn14 = assign67810_e104523_d_n14;

        let (assign67820_e104533, assign67820_e104533_d_n0, assign67820_e104533_d_n2, assign67820_e104533_d_n4, assign67820_e104533_d_n5, assign67820_e104533_d_n6, assign67820_e104533_d_n7, assign67820_e104533_d_n8, assign67820_e104533_d_n9, assign67820_e104533_d_n10, assign67820_e104533_d_n11, assign67820_e104533_d_n14,) = {
    if ((locals.var_guard1599 != 0.0) && (locals.var_guard1600 != 0.0)) {
        let assign67820_e104529: f64 = (100000.0 - 50000.0);
        let assign67820_e104531: f64 = (assign67820_e104529 + locals.var_tmf0);
        (assign67820_e104531, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign67820_e104533;
        locals.var_t2_dn0 = assign67820_e104533_d_n0;
        locals.var_t2_dn2 = assign67820_e104533_d_n2;
        locals.var_t2_dn4 = assign67820_e104533_d_n4;
        locals.var_t2_dn5 = assign67820_e104533_d_n5;
        locals.var_t2_dn6 = assign67820_e104533_d_n6;
        locals.var_t2_dn7 = assign67820_e104533_d_n7;
        locals.var_t2_dn8 = assign67820_e104533_d_n8;
        locals.var_t2_dn9 = assign67820_e104533_d_n9;
        locals.var_t2_dn10 = assign67820_e104533_d_n10;
        locals.var_t2_dn11 = assign67820_e104533_d_n11;
        locals.var_t2_dn14 = assign67820_e104533_d_n14;

        let (assign67830_e104539, assign67830_e104539_d_n0, assign67830_e104539_d_n2, assign67830_e104539_d_n4, assign67830_e104539_d_n5, assign67830_e104539_d_n6, assign67830_e104539_d_n7, assign67830_e104539_d_n8, assign67830_e104539_d_n9, assign67830_e104539_d_n10, assign67830_e104539_d_n11, assign67830_e104539_d_n14,) = {
    if ((locals.var_guard1599 != 0.0) && (locals.var_guard1600 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign67830_e104539;
        locals.var_t0_dn0 = assign67830_e104539_d_n0;
        locals.var_t0_dn2 = assign67830_e104539_d_n2;
        locals.var_t0_dn4 = assign67830_e104539_d_n4;
        locals.var_t0_dn5 = assign67830_e104539_d_n5;
        locals.var_t0_dn6 = assign67830_e104539_d_n6;
        locals.var_t0_dn7 = assign67830_e104539_d_n7;
        locals.var_t0_dn8 = assign67830_e104539_d_n8;
        locals.var_t0_dn9 = assign67830_e104539_d_n9;
        locals.var_t0_dn10 = assign67830_e104539_d_n10;
        locals.var_t0_dn11 = assign67830_e104539_d_n11;
        locals.var_t0_dn14 = assign67830_e104539_d_n14;

        let (assign67840_e104546, assign67840_e104546_d_n0, assign67840_e104546_d_n2, assign67840_e104546_d_n4, assign67840_e104546_d_n5, assign67840_e104546_d_n6, assign67840_e104546_d_n7, assign67840_e104546_d_n8, assign67840_e104546_d_n9, assign67840_e104546_d_n10, assign67840_e104546_d_n11, assign67840_e104546_d_n14,) = {
    if ((locals.var_guard1599 != 0.0) && (locals.var_guard1600 == 0.0)) {
        (locals.var_ibreakhe, locals.var_ibreakhe_dn0, locals.var_ibreakhe_dn2, locals.var_ibreakhe_dn4, locals.var_ibreakhe_dn5, locals.var_ibreakhe_dn6, locals.var_ibreakhe_dn7, locals.var_ibreakhe_dn8, locals.var_ibreakhe_dn9, locals.var_ibreakhe_dn10, locals.var_ibreakhe_dn11, locals.var_ibreakhe_dn14,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign67840_e104546;
        locals.var_t2_dn0 = assign67840_e104546_d_n0;
        locals.var_t2_dn2 = assign67840_e104546_d_n2;
        locals.var_t2_dn4 = assign67840_e104546_d_n4;
        locals.var_t2_dn5 = assign67840_e104546_d_n5;
        locals.var_t2_dn6 = assign67840_e104546_d_n6;
        locals.var_t2_dn7 = assign67840_e104546_d_n7;
        locals.var_t2_dn8 = assign67840_e104546_d_n8;
        locals.var_t2_dn9 = assign67840_e104546_d_n9;
        locals.var_t2_dn10 = assign67840_e104546_d_n10;
        locals.var_t2_dn11 = assign67840_e104546_d_n11;
        locals.var_t2_dn14 = assign67840_e104546_d_n14;

        let (assign67850_e104553, assign67850_e104553_d_n0, assign67850_e104553_d_n2, assign67850_e104553_d_n4, assign67850_e104553_d_n5, assign67850_e104553_d_n6, assign67850_e104553_d_n7, assign67850_e104553_d_n8, assign67850_e104553_d_n9, assign67850_e104553_d_n10, assign67850_e104553_d_n11, assign67850_e104553_d_n14,) = {
    if ((locals.var_guard1599 != 0.0) && (locals.var_guard1600 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign67850_e104553;
        locals.var_t0_dn0 = assign67850_e104553_d_n0;
        locals.var_t0_dn2 = assign67850_e104553_d_n2;
        locals.var_t0_dn4 = assign67850_e104553_d_n4;
        locals.var_t0_dn5 = assign67850_e104553_d_n5;
        locals.var_t0_dn6 = assign67850_e104553_d_n6;
        locals.var_t0_dn7 = assign67850_e104553_d_n7;
        locals.var_t0_dn8 = assign67850_e104553_d_n8;
        locals.var_t0_dn9 = assign67850_e104553_d_n9;
        locals.var_t0_dn10 = assign67850_e104553_d_n10;
        locals.var_t0_dn11 = assign67850_e104553_d_n11;
        locals.var_t0_dn14 = assign67850_e104553_d_n14;

        let (assign67860_e104561, assign67860_e104561_d_n0, assign67860_e104561_d_n2, assign67860_e104561_d_n4, assign67860_e104561_d_n5, assign67860_e104561_d_n6, assign67860_e104561_d_n7, assign67860_e104561_d_n8, assign67860_e104561_d_n9, assign67860_e104561_d_n10, assign67860_e104561_d_n11, assign67860_e104561_d_n14,) = {
    if (locals.var_guard1599 != 0.0) {
        let assign67860_e104557: f64 = (locals.var_mfactor * locals.var_weff_nf);
        let assign67860_e104559: f64 = (assign67860_e104557 * locals.var_t2);
        (assign67860_e104559, (assign67860_e104557 * locals.var_t2_dn0), (assign67860_e104557 * locals.var_t2_dn2), (assign67860_e104557 * locals.var_t2_dn4), (assign67860_e104557 * locals.var_t2_dn5), (assign67860_e104557 * locals.var_t2_dn6), (assign67860_e104557 * locals.var_t2_dn7), (assign67860_e104557 * locals.var_t2_dn8), (assign67860_e104557 * locals.var_t2_dn9), (assign67860_e104557 * locals.var_t2_dn10), (assign67860_e104557 * locals.var_t2_dn11), (assign67860_e104557 * locals.var_t2_dn14),)
    } else {
        (locals.var_ibreake, locals.var_ibreake_dn0, locals.var_ibreake_dn2, locals.var_ibreake_dn4, locals.var_ibreake_dn5, locals.var_ibreake_dn6, locals.var_ibreake_dn7, locals.var_ibreake_dn8, locals.var_ibreake_dn9, locals.var_ibreake_dn10, locals.var_ibreake_dn11, locals.var_ibreake_dn14,)
    }
};
        locals.var_ibreake = assign67860_e104561;
        locals.var_ibreake_dn0 = assign67860_e104561_d_n0;
        locals.var_ibreake_dn2 = assign67860_e104561_d_n2;
        locals.var_ibreake_dn4 = assign67860_e104561_d_n4;
        locals.var_ibreake_dn5 = assign67860_e104561_d_n5;
        locals.var_ibreake_dn6 = assign67860_e104561_d_n6;
        locals.var_ibreake_dn7 = assign67860_e104561_d_n7;
        locals.var_ibreake_dn8 = assign67860_e104561_d_n8;
        locals.var_ibreake_dn9 = assign67860_e104561_d_n9;
        locals.var_ibreake_dn10 = assign67860_e104561_d_n10;
        locals.var_ibreake_dn11 = assign67860_e104561_d_n11;
        locals.var_ibreake_dn14 = assign67860_e104561_d_n14;

    }

    pub(super) fn stamp_transient_block_242(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign67870_e104566, assign67870_e104566_d_n0, assign67870_e104566_d_n2, assign67870_e104566_d_n4, assign67870_e104566_d_n5, assign67870_e104566_d_n6, assign67870_e104566_d_n7, assign67870_e104566_d_n8, assign67870_e104566_d_n9, assign67870_e104566_d_n10, assign67870_e104566_d_n11, assign67870_e104566_d_n14,) = {
    if (locals.var_guard1599 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ibreake, locals.var_ibreake_dn0, locals.var_ibreake_dn2, locals.var_ibreake_dn4, locals.var_ibreake_dn5, locals.var_ibreake_dn6, locals.var_ibreake_dn7, locals.var_ibreake_dn8, locals.var_ibreake_dn9, locals.var_ibreake_dn10, locals.var_ibreake_dn11, locals.var_ibreake_dn14,)
    }
};
        locals.var_ibreake = assign67870_e104566;
        locals.var_ibreake_dn0 = assign67870_e104566_d_n0;
        locals.var_ibreake_dn2 = assign67870_e104566_d_n2;
        locals.var_ibreake_dn4 = assign67870_e104566_d_n4;
        locals.var_ibreake_dn5 = assign67870_e104566_d_n5;
        locals.var_ibreake_dn6 = assign67870_e104566_d_n6;
        locals.var_ibreake_dn7 = assign67870_e104566_d_n7;
        locals.var_ibreake_dn8 = assign67870_e104566_d_n8;
        locals.var_ibreake_dn9 = assign67870_e104566_d_n9;
        locals.var_ibreake_dn10 = assign67870_e104566_d_n10;
        locals.var_ibreake_dn11 = assign67870_e104566_d_n11;
        locals.var_ibreake_dn14 = assign67870_e104566_d_n14;

        let assign67880_e104569: f64 = (locals.var_isub + locals.var_isubld);
        let assign67880_e104579: f64 = if (((assign67880_e104569 > 0.0) && (locals.var_uc_ibpc1 != 0.0)) && (locals.var_uc_codep == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1606 = assign67880_e104579;

        let (assign67890_e104587, assign67890_e104587_d_n0, assign67890_e104587_d_n2, assign67890_e104587_d_n4, assign67890_e104587_d_n5, assign67890_e104587_d_n6, assign67890_e104587_d_n7, assign67890_e104587_d_n8, assign67890_e104587_d_n9, assign67890_e104587_d_n10, assign67890_e104587_d_n11, assign67890_e104587_d_n14,) = {
    if (locals.var_guard1606 != 0.0) {
        let assign67890_e104584: f64 = (locals.var_uc_ibpc2 * locals.var_dvth);
        let assign67890_e104585: f64 = (1.0 + assign67890_e104584);
        (assign67890_e104585, (locals.var_uc_ibpc2 * locals.var_dvth_dn0), (locals.var_uc_ibpc2 * locals.var_dvth_dn2), (locals.var_uc_ibpc2 * locals.var_dvth_dn4), (locals.var_uc_ibpc2 * locals.var_dvth_dn5), (locals.var_uc_ibpc2 * locals.var_dvth_dn6), (locals.var_uc_ibpc2 * locals.var_dvth_dn7), (locals.var_uc_ibpc2 * locals.var_dvth_dn8), (locals.var_uc_ibpc2 * locals.var_dvth_dn9), (locals.var_uc_ibpc2 * locals.var_dvth_dn10), (locals.var_uc_ibpc2 * locals.var_dvth_dn11), (locals.var_uc_ibpc2 * locals.var_dvth_dn14),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign67890_e104587;
        locals.var_t0_dn0 = assign67890_e104587_d_n0;
        locals.var_t0_dn2 = assign67890_e104587_d_n2;
        locals.var_t0_dn4 = assign67890_e104587_d_n4;
        locals.var_t0_dn5 = assign67890_e104587_d_n5;
        locals.var_t0_dn6 = assign67890_e104587_d_n6;
        locals.var_t0_dn7 = assign67890_e104587_d_n7;
        locals.var_t0_dn8 = assign67890_e104587_d_n8;
        locals.var_t0_dn9 = assign67890_e104587_d_n9;
        locals.var_t0_dn10 = assign67890_e104587_d_n10;
        locals.var_t0_dn11 = assign67890_e104587_d_n11;
        locals.var_t0_dn14 = assign67890_e104587_d_n14;

        let (assign67900_e104593, assign67900_e104593_d_n0, assign67900_e104593_d_n2, assign67900_e104593_d_n4, assign67900_e104593_d_n5, assign67900_e104593_d_n6, assign67900_e104593_d_n7, assign67900_e104593_d_n8, assign67900_e104593_d_n9, assign67900_e104593_d_n10, assign67900_e104593_d_n11, assign67900_e104593_d_n14,) = {
    if (locals.var_guard1606 != 0.0) {
        let assign67900_e104591: f64 = (locals.var_isub + locals.var_isubld);
        (assign67900_e104591, (locals.var_isub_dn0 + locals.var_isubld_dn0), (locals.var_isub_dn2 + locals.var_isubld_dn2), (locals.var_isub_dn4 + locals.var_isubld_dn4), (locals.var_isub_dn5 + locals.var_isubld_dn5), (locals.var_isub_dn6 + locals.var_isubld_dn6), (locals.var_isub_dn7 + locals.var_isubld_dn7), (locals.var_isub_dn8 + locals.var_isubld_dn8), (locals.var_isub_dn9 + locals.var_isubld_dn9), (locals.var_isub_dn10 + locals.var_isubld_dn10), (locals.var_isub_dn11 + locals.var_isubld_dn11), (locals.var_isub_dn14 + locals.var_isubld_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign67900_e104593;
        locals.var_t1_dn0 = assign67900_e104593_d_n0;
        locals.var_t1_dn2 = assign67900_e104593_d_n2;
        locals.var_t1_dn4 = assign67900_e104593_d_n4;
        locals.var_t1_dn5 = assign67900_e104593_d_n5;
        locals.var_t1_dn6 = assign67900_e104593_d_n6;
        locals.var_t1_dn7 = assign67900_e104593_d_n7;
        locals.var_t1_dn8 = assign67900_e104593_d_n8;
        locals.var_t1_dn9 = assign67900_e104593_d_n9;
        locals.var_t1_dn10 = assign67900_e104593_d_n10;
        locals.var_t1_dn11 = assign67900_e104593_d_n11;
        locals.var_t1_dn14 = assign67900_e104593_d_n14;

        let (assign67910_e104601, assign67910_e104601_d_n0, assign67910_e104601_d_n2, assign67910_e104601_d_n4, assign67910_e104601_d_n5, assign67910_e104601_d_n6, assign67910_e104601_d_n7, assign67910_e104601_d_n8, assign67910_e104601_d_n9, assign67910_e104601_d_n10, assign67910_e104601_d_n11, assign67910_e104601_d_n14,) = {
    if (locals.var_guard1606 != 0.0) {
        let assign67910_e104597: f64 = (locals.var_uc_ibpc1 * locals.var_t0);
        let assign67910_e104599: f64 = (assign67910_e104597 * locals.var_t1);
        (assign67910_e104599, (((locals.var_uc_ibpc1 * locals.var_t0_dn0) * locals.var_t1) + (assign67910_e104597 * locals.var_t1_dn0)), (((locals.var_uc_ibpc1 * locals.var_t0_dn2) * locals.var_t1) + (assign67910_e104597 * locals.var_t1_dn2)), (((locals.var_uc_ibpc1 * locals.var_t0_dn4) * locals.var_t1) + (assign67910_e104597 * locals.var_t1_dn4)), (((locals.var_uc_ibpc1 * locals.var_t0_dn5) * locals.var_t1) + (assign67910_e104597 * locals.var_t1_dn5)), (((locals.var_uc_ibpc1 * locals.var_t0_dn6) * locals.var_t1) + (assign67910_e104597 * locals.var_t1_dn6)), (((locals.var_uc_ibpc1 * locals.var_t0_dn7) * locals.var_t1) + (assign67910_e104597 * locals.var_t1_dn7)), (((locals.var_uc_ibpc1 * locals.var_t0_dn8) * locals.var_t1) + (assign67910_e104597 * locals.var_t1_dn8)), (((locals.var_uc_ibpc1 * locals.var_t0_dn9) * locals.var_t1) + (assign67910_e104597 * locals.var_t1_dn9)), (((locals.var_uc_ibpc1 * locals.var_t0_dn10) * locals.var_t1) + (assign67910_e104597 * locals.var_t1_dn10)), (((locals.var_uc_ibpc1 * locals.var_t0_dn11) * locals.var_t1) + (assign67910_e104597 * locals.var_t1_dn11)), (((locals.var_uc_ibpc1 * locals.var_t0_dn14) * locals.var_t1) + (assign67910_e104597 * locals.var_t1_dn14)),)
    } else {
        (locals.var_dvbsibpc, locals.var_dvbsibpc_dn0, locals.var_dvbsibpc_dn2, locals.var_dvbsibpc_dn4, locals.var_dvbsibpc_dn5, locals.var_dvbsibpc_dn6, locals.var_dvbsibpc_dn7, locals.var_dvbsibpc_dn8, locals.var_dvbsibpc_dn9, locals.var_dvbsibpc_dn10, locals.var_dvbsibpc_dn11, locals.var_dvbsibpc_dn14,)
    }
};
        locals.var_dvbsibpc = assign67910_e104601;
        locals.var_dvbsibpc_dn0 = assign67910_e104601_d_n0;
        locals.var_dvbsibpc_dn2 = assign67910_e104601_d_n2;
        locals.var_dvbsibpc_dn4 = assign67910_e104601_d_n4;
        locals.var_dvbsibpc_dn5 = assign67910_e104601_d_n5;
        locals.var_dvbsibpc_dn6 = assign67910_e104601_d_n6;
        locals.var_dvbsibpc_dn7 = assign67910_e104601_d_n7;
        locals.var_dvbsibpc_dn8 = assign67910_e104601_d_n8;
        locals.var_dvbsibpc_dn9 = assign67910_e104601_d_n9;
        locals.var_dvbsibpc_dn10 = assign67910_e104601_d_n10;
        locals.var_dvbsibpc_dn11 = assign67910_e104601_d_n11;
        locals.var_dvbsibpc_dn14 = assign67910_e104601_d_n14;

        let (assign67920_e104607, assign67920_e104607_d_n0, assign67920_e104607_d_n2, assign67920_e104607_d_n4, assign67920_e104607_d_n5, assign67920_e104607_d_n6, assign67920_e104607_d_n7, assign67920_e104607_d_n8, assign67920_e104607_d_n9, assign67920_e104607_d_n10, assign67920_e104607_d_n11, assign67920_e104607_d_n14,) = {
    if (locals.var_guard1606 != 0.0) {
        let assign67920_e104605: f64 = (1.0 / locals.var_xi0);
        (assign67920_e104605, (-(locals.var_xi0_dn0 / (locals.var_xi0 * locals.var_xi0))), (-(locals.var_xi0_dn2 / (locals.var_xi0 * locals.var_xi0))), (-(locals.var_xi0_dn4 / (locals.var_xi0 * locals.var_xi0))), (-(locals.var_xi0_dn5 / (locals.var_xi0 * locals.var_xi0))), (-(locals.var_xi0_dn6 / (locals.var_xi0 * locals.var_xi0))), (-(locals.var_xi0_dn7 / (locals.var_xi0 * locals.var_xi0))), (-(locals.var_xi0_dn8 / (locals.var_xi0 * locals.var_xi0))), (-(locals.var_xi0_dn9 / (locals.var_xi0 * locals.var_xi0))), (-(locals.var_xi0_dn10 / (locals.var_xi0 * locals.var_xi0))), (-(locals.var_xi0_dn11 / (locals.var_xi0 * locals.var_xi0))), (-(locals.var_xi0_dn14 / (locals.var_xi0 * locals.var_xi0))),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign67920_e104607;
        locals.var_t10_dn0 = assign67920_e104607_d_n0;
        locals.var_t10_dn2 = assign67920_e104607_d_n2;
        locals.var_t10_dn4 = assign67920_e104607_d_n4;
        locals.var_t10_dn5 = assign67920_e104607_d_n5;
        locals.var_t10_dn6 = assign67920_e104607_d_n6;
        locals.var_t10_dn7 = assign67920_e104607_d_n7;
        locals.var_t10_dn8 = assign67920_e104607_d_n8;
        locals.var_t10_dn9 = assign67920_e104607_d_n9;
        locals.var_t10_dn10 = assign67920_e104607_d_n10;
        locals.var_t10_dn11 = assign67920_e104607_d_n11;
        locals.var_t10_dn14 = assign67920_e104607_d_n14;

        let (assign67930_e104615, assign67930_e104615_d_n0, assign67930_e104615_d_n2, assign67930_e104615_d_n4, assign67930_e104615_d_n5, assign67930_e104615_d_n6, assign67930_e104615_d_n7, assign67930_e104615_d_n8, assign67930_e104615_d_n9, assign67930_e104615_d_n10, assign67930_e104615_d_n11, assign67930_e104615_d_n14,) = {
    if (locals.var_guard1606 != 0.0) {
        let assign67930_e104611: f64 = (locals.var_beta * locals.var_dvbsibpc);
        let assign67930_e104613: f64 = (assign67930_e104611 * locals.var_t10);
        (assign67930_e104613, ((((locals.var_beta_dn0 * locals.var_dvbsibpc) + (locals.var_beta * locals.var_dvbsibpc_dn0)) * locals.var_t10) + (assign67930_e104611 * locals.var_t10_dn0)), ((((locals.var_beta_dn2 * locals.var_dvbsibpc) + (locals.var_beta * locals.var_dvbsibpc_dn2)) * locals.var_t10) + (assign67930_e104611 * locals.var_t10_dn2)), ((((locals.var_beta_dn4 * locals.var_dvbsibpc) + (locals.var_beta * locals.var_dvbsibpc_dn4)) * locals.var_t10) + (assign67930_e104611 * locals.var_t10_dn4)), ((((locals.var_beta_dn5 * locals.var_dvbsibpc) + (locals.var_beta * locals.var_dvbsibpc_dn5)) * locals.var_t10) + (assign67930_e104611 * locals.var_t10_dn5)), ((((locals.var_beta_dn6 * locals.var_dvbsibpc) + (locals.var_beta * locals.var_dvbsibpc_dn6)) * locals.var_t10) + (assign67930_e104611 * locals.var_t10_dn6)), ((((locals.var_beta_dn7 * locals.var_dvbsibpc) + (locals.var_beta * locals.var_dvbsibpc_dn7)) * locals.var_t10) + (assign67930_e104611 * locals.var_t10_dn7)), ((((locals.var_beta_dn8 * locals.var_dvbsibpc) + (locals.var_beta * locals.var_dvbsibpc_dn8)) * locals.var_t10) + (assign67930_e104611 * locals.var_t10_dn8)), ((((locals.var_beta_dn9 * locals.var_dvbsibpc) + (locals.var_beta * locals.var_dvbsibpc_dn9)) * locals.var_t10) + (assign67930_e104611 * locals.var_t10_dn9)), ((((locals.var_beta_dn10 * locals.var_dvbsibpc) + (locals.var_beta * locals.var_dvbsibpc_dn10)) * locals.var_t10) + (assign67930_e104611 * locals.var_t10_dn10)), ((((locals.var_beta_dn11 * locals.var_dvbsibpc) + (locals.var_beta * locals.var_dvbsibpc_dn11)) * locals.var_t10) + (assign67930_e104611 * locals.var_t10_dn11)), ((((locals.var_beta_dn14 * locals.var_dvbsibpc) + (locals.var_beta * locals.var_dvbsibpc_dn14)) * locals.var_t10) + (assign67930_e104611 * locals.var_t10_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign67930_e104615;
        locals.var_t1_dn0 = assign67930_e104615_d_n0;
        locals.var_t1_dn2 = assign67930_e104615_d_n2;
        locals.var_t1_dn4 = assign67930_e104615_d_n4;
        locals.var_t1_dn5 = assign67930_e104615_d_n5;
        locals.var_t1_dn6 = assign67930_e104615_d_n6;
        locals.var_t1_dn7 = assign67930_e104615_d_n7;
        locals.var_t1_dn8 = assign67930_e104615_d_n8;
        locals.var_t1_dn9 = assign67930_e104615_d_n9;
        locals.var_t1_dn10 = assign67930_e104615_d_n10;
        locals.var_t1_dn11 = assign67930_e104615_d_n11;
        locals.var_t1_dn14 = assign67930_e104615_d_n14;

        let (assign67940_e104621, assign67940_e104621_d_n0, assign67940_e104621_d_n2, assign67940_e104621_d_n4, assign67940_e104621_d_n5, assign67940_e104621_d_n6, assign67940_e104621_d_n7, assign67940_e104621_d_n8, assign67940_e104621_d_n9, assign67940_e104621_d_n10, assign67940_e104621_d_n11, assign67940_e104621_d_n14,) = {
    if (locals.var_guard1606 != 0.0) {
        let assign67940_e104619: f64 = (locals.var_t10 * locals.var_t10);
        (assign67940_e104619, ((locals.var_t10_dn0 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn0)), ((locals.var_t10_dn2 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn2)), ((locals.var_t10_dn4 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn4)), ((locals.var_t10_dn5 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn5)), ((locals.var_t10_dn6 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn6)), ((locals.var_t10_dn7 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn7)), ((locals.var_t10_dn8 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn8)), ((locals.var_t10_dn9 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn9)), ((locals.var_t10_dn10 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn10)), ((locals.var_t10_dn11 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn11)), ((locals.var_t10_dn14 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn14)),)
    } else {
        (locals.var_t11, locals.var_t11_dn0, locals.var_t11_dn2, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11, locals.var_t11_dn14,)
    }
};
        locals.var_t11 = assign67940_e104621;
        locals.var_t11_dn0 = assign67940_e104621_d_n0;
        locals.var_t11_dn2 = assign67940_e104621_d_n2;
        locals.var_t11_dn4 = assign67940_e104621_d_n4;
        locals.var_t11_dn5 = assign67940_e104621_d_n5;
        locals.var_t11_dn6 = assign67940_e104621_d_n6;
        locals.var_t11_dn7 = assign67940_e104621_d_n7;
        locals.var_t11_dn8 = assign67940_e104621_d_n8;
        locals.var_t11_dn9 = assign67940_e104621_d_n9;
        locals.var_t11_dn10 = assign67940_e104621_d_n10;
        locals.var_t11_dn11 = assign67940_e104621_d_n11;
        locals.var_t11_dn14 = assign67940_e104621_d_n14;

        let (assign67950_e104627, assign67950_e104627_d_n0, assign67950_e104627_d_n2, assign67950_e104627_d_n4, assign67950_e104627_d_n5, assign67950_e104627_d_n6, assign67950_e104627_d_n7, assign67950_e104627_d_n8, assign67950_e104627_d_n9, assign67950_e104627_d_n10, assign67950_e104627_d_n11, assign67950_e104627_d_n14,) = {
    if (locals.var_guard1606 != 0.0) {
        let assign67950_e104625: f64 = (1.0 / locals.var_xil);
        (assign67950_e104625, (-(locals.var_xil_dn0 / (locals.var_xil * locals.var_xil))), (-(locals.var_xil_dn2 / (locals.var_xil * locals.var_xil))), (-(locals.var_xil_dn4 / (locals.var_xil * locals.var_xil))), (-(locals.var_xil_dn5 / (locals.var_xil * locals.var_xil))), (-(locals.var_xil_dn6 / (locals.var_xil * locals.var_xil))), (-(locals.var_xil_dn7 / (locals.var_xil * locals.var_xil))), (-(locals.var_xil_dn8 / (locals.var_xil * locals.var_xil))), (-(locals.var_xil_dn9 / (locals.var_xil * locals.var_xil))), (-(locals.var_xil_dn10 / (locals.var_xil * locals.var_xil))), (-(locals.var_xil_dn11 / (locals.var_xil * locals.var_xil))), (-(locals.var_xil_dn14 / (locals.var_xil * locals.var_xil))),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign67950_e104627;
        locals.var_t10_dn0 = assign67950_e104627_d_n0;
        locals.var_t10_dn2 = assign67950_e104627_d_n2;
        locals.var_t10_dn4 = assign67950_e104627_d_n4;
        locals.var_t10_dn5 = assign67950_e104627_d_n5;
        locals.var_t10_dn6 = assign67950_e104627_d_n6;
        locals.var_t10_dn7 = assign67950_e104627_d_n7;
        locals.var_t10_dn8 = assign67950_e104627_d_n8;
        locals.var_t10_dn9 = assign67950_e104627_d_n9;
        locals.var_t10_dn10 = assign67950_e104627_d_n10;
        locals.var_t10_dn11 = assign67950_e104627_d_n11;
        locals.var_t10_dn14 = assign67950_e104627_d_n14;

        let (assign67960_e104635, assign67960_e104635_d_n0, assign67960_e104635_d_n2, assign67960_e104635_d_n4, assign67960_e104635_d_n5, assign67960_e104635_d_n6, assign67960_e104635_d_n7, assign67960_e104635_d_n8, assign67960_e104635_d_n9, assign67960_e104635_d_n10, assign67960_e104635_d_n11, assign67960_e104635_d_n14,) = {
    if (locals.var_guard1606 != 0.0) {
        let assign67960_e104631: f64 = (locals.var_beta * locals.var_dvbsibpc);
        let assign67960_e104633: f64 = (assign67960_e104631 * locals.var_t10);
        (assign67960_e104633, ((((locals.var_beta_dn0 * locals.var_dvbsibpc) + (locals.var_beta * locals.var_dvbsibpc_dn0)) * locals.var_t10) + (assign67960_e104631 * locals.var_t10_dn0)), ((((locals.var_beta_dn2 * locals.var_dvbsibpc) + (locals.var_beta * locals.var_dvbsibpc_dn2)) * locals.var_t10) + (assign67960_e104631 * locals.var_t10_dn2)), ((((locals.var_beta_dn4 * locals.var_dvbsibpc) + (locals.var_beta * locals.var_dvbsibpc_dn4)) * locals.var_t10) + (assign67960_e104631 * locals.var_t10_dn4)), ((((locals.var_beta_dn5 * locals.var_dvbsibpc) + (locals.var_beta * locals.var_dvbsibpc_dn5)) * locals.var_t10) + (assign67960_e104631 * locals.var_t10_dn5)), ((((locals.var_beta_dn6 * locals.var_dvbsibpc) + (locals.var_beta * locals.var_dvbsibpc_dn6)) * locals.var_t10) + (assign67960_e104631 * locals.var_t10_dn6)), ((((locals.var_beta_dn7 * locals.var_dvbsibpc) + (locals.var_beta * locals.var_dvbsibpc_dn7)) * locals.var_t10) + (assign67960_e104631 * locals.var_t10_dn7)), ((((locals.var_beta_dn8 * locals.var_dvbsibpc) + (locals.var_beta * locals.var_dvbsibpc_dn8)) * locals.var_t10) + (assign67960_e104631 * locals.var_t10_dn8)), ((((locals.var_beta_dn9 * locals.var_dvbsibpc) + (locals.var_beta * locals.var_dvbsibpc_dn9)) * locals.var_t10) + (assign67960_e104631 * locals.var_t10_dn9)), ((((locals.var_beta_dn10 * locals.var_dvbsibpc) + (locals.var_beta * locals.var_dvbsibpc_dn10)) * locals.var_t10) + (assign67960_e104631 * locals.var_t10_dn10)), ((((locals.var_beta_dn11 * locals.var_dvbsibpc) + (locals.var_beta * locals.var_dvbsibpc_dn11)) * locals.var_t10) + (assign67960_e104631 * locals.var_t10_dn11)), ((((locals.var_beta_dn14 * locals.var_dvbsibpc) + (locals.var_beta * locals.var_dvbsibpc_dn14)) * locals.var_t10) + (assign67960_e104631 * locals.var_t10_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign67960_e104635;
        locals.var_t2_dn0 = assign67960_e104635_d_n0;
        locals.var_t2_dn2 = assign67960_e104635_d_n2;
        locals.var_t2_dn4 = assign67960_e104635_d_n4;
        locals.var_t2_dn5 = assign67960_e104635_d_n5;
        locals.var_t2_dn6 = assign67960_e104635_d_n6;
        locals.var_t2_dn7 = assign67960_e104635_d_n7;
        locals.var_t2_dn8 = assign67960_e104635_d_n8;
        locals.var_t2_dn9 = assign67960_e104635_d_n9;
        locals.var_t2_dn10 = assign67960_e104635_d_n10;
        locals.var_t2_dn11 = assign67960_e104635_d_n11;
        locals.var_t2_dn14 = assign67960_e104635_d_n14;

        let (assign67970_e104641, assign67970_e104641_d_n0, assign67970_e104641_d_n2, assign67970_e104641_d_n4, assign67970_e104641_d_n5, assign67970_e104641_d_n6, assign67970_e104641_d_n7, assign67970_e104641_d_n8, assign67970_e104641_d_n9, assign67970_e104641_d_n10, assign67970_e104641_d_n11, assign67970_e104641_d_n14,) = {
    if (locals.var_guard1606 != 0.0) {
        let assign67970_e104639: f64 = (locals.var_t10 * locals.var_t10);
        (assign67970_e104639, ((locals.var_t10_dn0 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn0)), ((locals.var_t10_dn2 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn2)), ((locals.var_t10_dn4 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn4)), ((locals.var_t10_dn5 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn5)), ((locals.var_t10_dn6 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn6)), ((locals.var_t10_dn7 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn7)), ((locals.var_t10_dn8 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn8)), ((locals.var_t10_dn9 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn9)), ((locals.var_t10_dn10 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn10)), ((locals.var_t10_dn11 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn11)), ((locals.var_t10_dn14 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn14)),)
    } else {
        (locals.var_t11, locals.var_t11_dn0, locals.var_t11_dn2, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11, locals.var_t11_dn14,)
    }
};
        locals.var_t11 = assign67970_e104641;
        locals.var_t11_dn0 = assign67970_e104641_d_n0;
        locals.var_t11_dn2 = assign67970_e104641_d_n2;
        locals.var_t11_dn4 = assign67970_e104641_d_n4;
        locals.var_t11_dn5 = assign67970_e104641_d_n5;
        locals.var_t11_dn6 = assign67970_e104641_d_n6;
        locals.var_t11_dn7 = assign67970_e104641_d_n7;
        locals.var_t11_dn8 = assign67970_e104641_d_n8;
        locals.var_t11_dn9 = assign67970_e104641_d_n9;
        locals.var_t11_dn10 = assign67970_e104641_d_n10;
        locals.var_t11_dn11 = assign67970_e104641_d_n11;
        locals.var_t11_dn14 = assign67970_e104641_d_n14;

        let (assign67980_e104653, assign67980_e104653_d_n0, assign67980_e104653_d_n2, assign67980_e104653_d_n4, assign67980_e104653_d_n5, assign67980_e104653_d_n6, assign67980_e104653_d_n7, assign67980_e104653_d_n8, assign67980_e104653_d_n9, assign67980_e104653_d_n10, assign67980_e104653_d_n11, assign67980_e104653_d_n14,) = {
    if (locals.var_guard1606 != 0.0) {
        let assign67980_e104646: f64 = (locals.var_xilp32 * locals.var_t2);
        let assign67980_e104649: f64 = (locals.var_xi0p32 * locals.var_t1);
        let assign67980_e104650: f64 = (assign67980_e104646 - assign67980_e104649);
        let assign67980_e104651: f64 = (locals.var_cnst0 * assign67980_e104650);
        (assign67980_e104651, ((locals.var_cnst0_dn0 * assign67980_e104650) + (locals.var_cnst0 * (((locals.var_xilp32_dn0 * locals.var_t2) + (locals.var_xilp32 * locals.var_t2_dn0)) - ((locals.var_xi0p32_dn0 * locals.var_t1) + (locals.var_xi0p32 * locals.var_t1_dn0))))), ((locals.var_cnst0_dn2 * assign67980_e104650) + (locals.var_cnst0 * (((locals.var_xilp32_dn2 * locals.var_t2) + (locals.var_xilp32 * locals.var_t2_dn2)) - ((locals.var_xi0p32_dn2 * locals.var_t1) + (locals.var_xi0p32 * locals.var_t1_dn2))))), ((locals.var_cnst0_dn4 * assign67980_e104650) + (locals.var_cnst0 * (((locals.var_xilp32_dn4 * locals.var_t2) + (locals.var_xilp32 * locals.var_t2_dn4)) - ((locals.var_xi0p32_dn4 * locals.var_t1) + (locals.var_xi0p32 * locals.var_t1_dn4))))), ((locals.var_cnst0_dn5 * assign67980_e104650) + (locals.var_cnst0 * (((locals.var_xilp32_dn5 * locals.var_t2) + (locals.var_xilp32 * locals.var_t2_dn5)) - ((locals.var_xi0p32_dn5 * locals.var_t1) + (locals.var_xi0p32 * locals.var_t1_dn5))))), ((locals.var_cnst0_dn6 * assign67980_e104650) + (locals.var_cnst0 * (((locals.var_xilp32_dn6 * locals.var_t2) + (locals.var_xilp32 * locals.var_t2_dn6)) - ((locals.var_xi0p32_dn6 * locals.var_t1) + (locals.var_xi0p32 * locals.var_t1_dn6))))), ((locals.var_cnst0_dn7 * assign67980_e104650) + (locals.var_cnst0 * (((locals.var_xilp32_dn7 * locals.var_t2) + (locals.var_xilp32 * locals.var_t2_dn7)) - ((locals.var_xi0p32_dn7 * locals.var_t1) + (locals.var_xi0p32 * locals.var_t1_dn7))))), ((locals.var_cnst0_dn8 * assign67980_e104650) + (locals.var_cnst0 * (((locals.var_xilp32_dn8 * locals.var_t2) + (locals.var_xilp32 * locals.var_t2_dn8)) - ((locals.var_xi0p32_dn8 * locals.var_t1) + (locals.var_xi0p32 * locals.var_t1_dn8))))), ((locals.var_cnst0_dn9 * assign67980_e104650) + (locals.var_cnst0 * (((locals.var_xilp32_dn9 * locals.var_t2) + (locals.var_xilp32 * locals.var_t2_dn9)) - ((locals.var_xi0p32_dn9 * locals.var_t1) + (locals.var_xi0p32 * locals.var_t1_dn9))))), ((locals.var_cnst0_dn10 * assign67980_e104650) + (locals.var_cnst0 * (((locals.var_xilp32_dn10 * locals.var_t2) + (locals.var_xilp32 * locals.var_t2_dn10)) - ((locals.var_xi0p32_dn10 * locals.var_t1) + (locals.var_xi0p32 * locals.var_t1_dn10))))), ((locals.var_cnst0_dn11 * assign67980_e104650) + (locals.var_cnst0 * (((locals.var_xilp32_dn11 * locals.var_t2) + (locals.var_xilp32 * locals.var_t2_dn11)) - ((locals.var_xi0p32_dn11 * locals.var_t1) + (locals.var_xi0p32 * locals.var_t1_dn11))))), ((locals.var_cnst0_dn14 * assign67980_e104650) + (locals.var_cnst0 * (((locals.var_xilp32_dn14 * locals.var_t2) + (locals.var_xilp32 * locals.var_t2_dn14)) - ((locals.var_xi0p32_dn14 * locals.var_t1) + (locals.var_xi0p32 * locals.var_t1_dn14))))),)
    } else {
        (locals.var_dg3, locals.var_dg3_dn0, locals.var_dg3_dn2, locals.var_dg3_dn4, locals.var_dg3_dn5, locals.var_dg3_dn6, locals.var_dg3_dn7, locals.var_dg3_dn8, locals.var_dg3_dn9, locals.var_dg3_dn10, locals.var_dg3_dn11, locals.var_dg3_dn14,)
    }
};
        locals.var_dg3 = assign67980_e104653;
        locals.var_dg3_dn0 = assign67980_e104653_d_n0;
        locals.var_dg3_dn2 = assign67980_e104653_d_n2;
        locals.var_dg3_dn4 = assign67980_e104653_d_n4;
        locals.var_dg3_dn5 = assign67980_e104653_d_n5;
        locals.var_dg3_dn6 = assign67980_e104653_d_n6;
        locals.var_dg3_dn7 = assign67980_e104653_d_n7;
        locals.var_dg3_dn8 = assign67980_e104653_d_n8;
        locals.var_dg3_dn9 = assign67980_e104653_d_n9;
        locals.var_dg3_dn10 = assign67980_e104653_d_n10;
        locals.var_dg3_dn11 = assign67980_e104653_d_n11;
        locals.var_dg3_dn14 = assign67980_e104653_d_n14;

        let (assign67990_e104668, assign67990_e104668_d_n0, assign67990_e104668_d_n2, assign67990_e104668_d_n4, assign67990_e104668_d_n5, assign67990_e104668_d_n6, assign67990_e104668_d_n7, assign67990_e104668_d_n8, assign67990_e104668_d_n9, assign67990_e104668_d_n10, assign67990_e104668_d_n11, assign67990_e104668_d_n14,) = {
    if (locals.var_guard1606 != 0.0) {
        let assign67990_e104657: f64 = (locals.var_cnst0 * 0.5);
        let assign67990_e104659: f64 = (-locals.var_xilp12);
        let assign67990_e104661: f64 = (assign67990_e104659 * locals.var_t2);
        let assign67990_e104664: f64 = (locals.var_xi0p12 * locals.var_t1);
        let assign67990_e104665: f64 = (assign67990_e104661 + assign67990_e104664);
        let assign67990_e104666: f64 = (assign67990_e104657 * assign67990_e104665);
        (assign67990_e104666, (((locals.var_cnst0_dn0 * 0.5) * assign67990_e104665) + (assign67990_e104657 * ((((-locals.var_xilp12_dn0) * locals.var_t2) + (assign67990_e104659 * locals.var_t2_dn0)) + ((locals.var_xi0p12_dn0 * locals.var_t1) + (locals.var_xi0p12 * locals.var_t1_dn0))))), (((locals.var_cnst0_dn2 * 0.5) * assign67990_e104665) + (assign67990_e104657 * ((((-locals.var_xilp12_dn2) * locals.var_t2) + (assign67990_e104659 * locals.var_t2_dn2)) + ((locals.var_xi0p12_dn2 * locals.var_t1) + (locals.var_xi0p12 * locals.var_t1_dn2))))), (((locals.var_cnst0_dn4 * 0.5) * assign67990_e104665) + (assign67990_e104657 * ((((-locals.var_xilp12_dn4) * locals.var_t2) + (assign67990_e104659 * locals.var_t2_dn4)) + ((locals.var_xi0p12_dn4 * locals.var_t1) + (locals.var_xi0p12 * locals.var_t1_dn4))))), (((locals.var_cnst0_dn5 * 0.5) * assign67990_e104665) + (assign67990_e104657 * ((((-locals.var_xilp12_dn5) * locals.var_t2) + (assign67990_e104659 * locals.var_t2_dn5)) + ((locals.var_xi0p12_dn5 * locals.var_t1) + (locals.var_xi0p12 * locals.var_t1_dn5))))), (((locals.var_cnst0_dn6 * 0.5) * assign67990_e104665) + (assign67990_e104657 * ((((-locals.var_xilp12_dn6) * locals.var_t2) + (assign67990_e104659 * locals.var_t2_dn6)) + ((locals.var_xi0p12_dn6 * locals.var_t1) + (locals.var_xi0p12 * locals.var_t1_dn6))))), (((locals.var_cnst0_dn7 * 0.5) * assign67990_e104665) + (assign67990_e104657 * ((((-locals.var_xilp12_dn7) * locals.var_t2) + (assign67990_e104659 * locals.var_t2_dn7)) + ((locals.var_xi0p12_dn7 * locals.var_t1) + (locals.var_xi0p12 * locals.var_t1_dn7))))), (((locals.var_cnst0_dn8 * 0.5) * assign67990_e104665) + (assign67990_e104657 * ((((-locals.var_xilp12_dn8) * locals.var_t2) + (assign67990_e104659 * locals.var_t2_dn8)) + ((locals.var_xi0p12_dn8 * locals.var_t1) + (locals.var_xi0p12 * locals.var_t1_dn8))))), (((locals.var_cnst0_dn9 * 0.5) * assign67990_e104665) + (assign67990_e104657 * ((((-locals.var_xilp12_dn9) * locals.var_t2) + (assign67990_e104659 * locals.var_t2_dn9)) + ((locals.var_xi0p12_dn9 * locals.var_t1) + (locals.var_xi0p12 * locals.var_t1_dn9))))), (((locals.var_cnst0_dn10 * 0.5) * assign67990_e104665) + (assign67990_e104657 * ((((-locals.var_xilp12_dn10) * locals.var_t2) + (assign67990_e104659 * locals.var_t2_dn10)) + ((locals.var_xi0p12_dn10 * locals.var_t1) + (locals.var_xi0p12 * locals.var_t1_dn10))))), (((locals.var_cnst0_dn11 * 0.5) * assign67990_e104665) + (assign67990_e104657 * ((((-locals.var_xilp12_dn11) * locals.var_t2) + (assign67990_e104659 * locals.var_t2_dn11)) + ((locals.var_xi0p12_dn11 * locals.var_t1) + (locals.var_xi0p12 * locals.var_t1_dn11))))), (((locals.var_cnst0_dn14 * 0.5) * assign67990_e104665) + (assign67990_e104657 * ((((-locals.var_xilp12_dn14) * locals.var_t2) + (assign67990_e104659 * locals.var_t2_dn14)) + ((locals.var_xi0p12_dn14 * locals.var_t1) + (locals.var_xi0p12 * locals.var_t1_dn14))))),)
    } else {
        (locals.var_dg4, locals.var_dg4_dn0, locals.var_dg4_dn2, locals.var_dg4_dn4, locals.var_dg4_dn5, locals.var_dg4_dn6, locals.var_dg4_dn7, locals.var_dg4_dn8, locals.var_dg4_dn9, locals.var_dg4_dn10, locals.var_dg4_dn11, locals.var_dg4_dn14,)
    }
};
        locals.var_dg4 = assign67990_e104668;
        locals.var_dg4_dn0 = assign67990_e104668_d_n0;
        locals.var_dg4_dn2 = assign67990_e104668_d_n2;
        locals.var_dg4_dn4 = assign67990_e104668_d_n4;
        locals.var_dg4_dn5 = assign67990_e104668_d_n5;
        locals.var_dg4_dn6 = assign67990_e104668_d_n6;
        locals.var_dg4_dn7 = assign67990_e104668_d_n7;
        locals.var_dg4_dn8 = assign67990_e104668_d_n8;
        locals.var_dg4_dn9 = assign67990_e104668_d_n9;
        locals.var_dg4_dn10 = assign67990_e104668_d_n10;
        locals.var_dg4_dn11 = assign67990_e104668_d_n11;
        locals.var_dg4_dn14 = assign67990_e104668_d_n14;

        let (assign68000_e104674, assign68000_e104674_d_n0, assign68000_e104674_d_n2, assign68000_e104674_d_n4, assign68000_e104674_d_n5, assign68000_e104674_d_n6, assign68000_e104674_d_n7, assign68000_e104674_d_n8, assign68000_e104674_d_n9, assign68000_e104674_d_n10, assign68000_e104674_d_n11, assign68000_e104674_d_n14,) = {
    if (locals.var_guard1606 != 0.0) {
        let assign68000_e104672: f64 = (locals.var_dg3 + locals.var_dg4);
        (assign68000_e104672, (locals.var_dg3_dn0 + locals.var_dg4_dn0), (locals.var_dg3_dn2 + locals.var_dg4_dn2), (locals.var_dg3_dn4 + locals.var_dg4_dn4), (locals.var_dg3_dn5 + locals.var_dg4_dn5), (locals.var_dg3_dn6 + locals.var_dg4_dn6), (locals.var_dg3_dn7 + locals.var_dg4_dn7), (locals.var_dg3_dn8 + locals.var_dg4_dn8), (locals.var_dg3_dn9 + locals.var_dg4_dn9), (locals.var_dg3_dn10 + locals.var_dg4_dn10), (locals.var_dg3_dn11 + locals.var_dg4_dn11), (locals.var_dg3_dn14 + locals.var_dg4_dn14),)
    } else {
        (locals.var_didd, locals.var_didd_dn0, locals.var_didd_dn2, locals.var_didd_dn4, locals.var_didd_dn5, locals.var_didd_dn6, locals.var_didd_dn7, locals.var_didd_dn8, locals.var_didd_dn9, locals.var_didd_dn10, locals.var_didd_dn11, locals.var_didd_dn14,)
    }
};
        locals.var_didd = assign68000_e104674;
        locals.var_didd_dn0 = assign68000_e104674_d_n0;
        locals.var_didd_dn2 = assign68000_e104674_d_n2;
        locals.var_didd_dn4 = assign68000_e104674_d_n4;
        locals.var_didd_dn5 = assign68000_e104674_d_n5;
        locals.var_didd_dn6 = assign68000_e104674_d_n6;
        locals.var_didd_dn7 = assign68000_e104674_d_n7;
        locals.var_didd_dn8 = assign68000_e104674_d_n8;
        locals.var_didd_dn9 = assign68000_e104674_d_n9;
        locals.var_didd_dn10 = assign68000_e104674_d_n10;
        locals.var_didd_dn11 = assign68000_e104674_d_n11;
        locals.var_didd_dn14 = assign68000_e104674_d_n14;

        let (assign68010_e104682, assign68010_e104682_d_n0, assign68010_e104682_d_n2, assign68010_e104682_d_n4, assign68010_e104682_d_n5, assign68010_e104682_d_n6, assign68010_e104682_d_n7, assign68010_e104682_d_n8, assign68010_e104682_d_n9, assign68010_e104682_d_n10, assign68010_e104682_d_n11, assign68010_e104682_d_n14,) = {
    if (locals.var_guard1606 != 0.0) {
        let assign68010_e104678: f64 = (locals.var_betawl * locals.var_didd);
        let assign68010_e104680: f64 = (assign68010_e104678 * locals.var_mu);
        (assign68010_e104680, ((((locals.var_betawl_dn0 * locals.var_didd) + (locals.var_betawl * locals.var_didd_dn0)) * locals.var_mu) + (assign68010_e104678 * locals.var_mu_dn0)), ((((locals.var_betawl_dn2 * locals.var_didd) + (locals.var_betawl * locals.var_didd_dn2)) * locals.var_mu) + (assign68010_e104678 * locals.var_mu_dn2)), ((((locals.var_betawl_dn4 * locals.var_didd) + (locals.var_betawl * locals.var_didd_dn4)) * locals.var_mu) + (assign68010_e104678 * locals.var_mu_dn4)), ((((locals.var_betawl_dn5 * locals.var_didd) + (locals.var_betawl * locals.var_didd_dn5)) * locals.var_mu) + (assign68010_e104678 * locals.var_mu_dn5)), ((((locals.var_betawl_dn6 * locals.var_didd) + (locals.var_betawl * locals.var_didd_dn6)) * locals.var_mu) + (assign68010_e104678 * locals.var_mu_dn6)), ((((locals.var_betawl_dn7 * locals.var_didd) + (locals.var_betawl * locals.var_didd_dn7)) * locals.var_mu) + (assign68010_e104678 * locals.var_mu_dn7)), ((((locals.var_betawl_dn8 * locals.var_didd) + (locals.var_betawl * locals.var_didd_dn8)) * locals.var_mu) + (assign68010_e104678 * locals.var_mu_dn8)), ((((locals.var_betawl_dn9 * locals.var_didd) + (locals.var_betawl * locals.var_didd_dn9)) * locals.var_mu) + (assign68010_e104678 * locals.var_mu_dn9)), ((((locals.var_betawl_dn10 * locals.var_didd) + (locals.var_betawl * locals.var_didd_dn10)) * locals.var_mu) + (assign68010_e104678 * locals.var_mu_dn10)), ((((locals.var_betawl_dn11 * locals.var_didd) + (locals.var_betawl * locals.var_didd_dn11)) * locals.var_mu) + (assign68010_e104678 * locals.var_mu_dn11)), ((((locals.var_betawl_dn14 * locals.var_didd) + (locals.var_betawl * locals.var_didd_dn14)) * locals.var_mu) + (assign68010_e104678 * locals.var_mu_dn14)),)
    } else {
        (locals.var_idsibpc, locals.var_idsibpc_dn0, locals.var_idsibpc_dn2, locals.var_idsibpc_dn4, locals.var_idsibpc_dn5, locals.var_idsibpc_dn6, locals.var_idsibpc_dn7, locals.var_idsibpc_dn8, locals.var_idsibpc_dn9, locals.var_idsibpc_dn10, locals.var_idsibpc_dn11, locals.var_idsibpc_dn14,)
    }
};
        locals.var_idsibpc = assign68010_e104682;
        locals.var_idsibpc_dn0 = assign68010_e104682_d_n0;
        locals.var_idsibpc_dn2 = assign68010_e104682_d_n2;
        locals.var_idsibpc_dn4 = assign68010_e104682_d_n4;
        locals.var_idsibpc_dn5 = assign68010_e104682_d_n5;
        locals.var_idsibpc_dn6 = assign68010_e104682_d_n6;
        locals.var_idsibpc_dn7 = assign68010_e104682_d_n7;
        locals.var_idsibpc_dn8 = assign68010_e104682_d_n8;
        locals.var_idsibpc_dn9 = assign68010_e104682_d_n9;
        locals.var_idsibpc_dn10 = assign68010_e104682_d_n10;
        locals.var_idsibpc_dn11 = assign68010_e104682_d_n11;
        locals.var_idsibpc_dn14 = assign68010_e104682_d_n14;

        let (assign68020_e104688, assign68020_e104688_d_n0, assign68020_e104688_d_n2, assign68020_e104688_d_n4, assign68020_e104688_d_n5, assign68020_e104688_d_n6, assign68020_e104688_d_n7, assign68020_e104688_d_n8, assign68020_e104688_d_n9, assign68020_e104688_d_n10, assign68020_e104688_d_n11, assign68020_e104688_d_n14,) = {
    if (locals.var_guard1606 != 0.0) {
        let assign68020_e104686: f64 = (locals.var_wk_ii * locals.var_idsibpc);
        (assign68020_e104686, ((locals.var_wk_ii_dn0 * locals.var_idsibpc) + (locals.var_wk_ii * locals.var_idsibpc_dn0)), ((locals.var_wk_ii_dn2 * locals.var_idsibpc) + (locals.var_wk_ii * locals.var_idsibpc_dn2)), ((locals.var_wk_ii_dn4 * locals.var_idsibpc) + (locals.var_wk_ii * locals.var_idsibpc_dn4)), ((locals.var_wk_ii_dn5 * locals.var_idsibpc) + (locals.var_wk_ii * locals.var_idsibpc_dn5)), ((locals.var_wk_ii_dn6 * locals.var_idsibpc) + (locals.var_wk_ii * locals.var_idsibpc_dn6)), ((locals.var_wk_ii_dn7 * locals.var_idsibpc) + (locals.var_wk_ii * locals.var_idsibpc_dn7)), ((locals.var_wk_ii_dn8 * locals.var_idsibpc) + (locals.var_wk_ii * locals.var_idsibpc_dn8)), ((locals.var_wk_ii_dn9 * locals.var_idsibpc) + (locals.var_wk_ii * locals.var_idsibpc_dn9)), ((locals.var_wk_ii_dn10 * locals.var_idsibpc) + (locals.var_wk_ii * locals.var_idsibpc_dn10)), ((locals.var_wk_ii_dn11 * locals.var_idsibpc) + (locals.var_wk_ii * locals.var_idsibpc_dn11)), ((locals.var_wk_ii_dn14 * locals.var_idsibpc) + (locals.var_wk_ii * locals.var_idsibpc_dn14)),)
    } else {
        (locals.var_isubibpc, locals.var_isubibpc_dn0, locals.var_isubibpc_dn2, locals.var_isubibpc_dn4, locals.var_isubibpc_dn5, locals.var_isubibpc_dn6, locals.var_isubibpc_dn7, locals.var_isubibpc_dn8, locals.var_isubibpc_dn9, locals.var_isubibpc_dn10, locals.var_isubibpc_dn11, locals.var_isubibpc_dn14,)
    }
};
        locals.var_isubibpc = assign68020_e104688;
        locals.var_isubibpc_dn0 = assign68020_e104688_d_n0;
        locals.var_isubibpc_dn2 = assign68020_e104688_d_n2;
        locals.var_isubibpc_dn4 = assign68020_e104688_d_n4;
        locals.var_isubibpc_dn5 = assign68020_e104688_d_n5;
        locals.var_isubibpc_dn6 = assign68020_e104688_d_n6;
        locals.var_isubibpc_dn7 = assign68020_e104688_d_n7;
        locals.var_isubibpc_dn8 = assign68020_e104688_d_n8;
        locals.var_isubibpc_dn9 = assign68020_e104688_d_n9;
        locals.var_isubibpc_dn10 = assign68020_e104688_d_n10;
        locals.var_isubibpc_dn11 = assign68020_e104688_d_n11;
        locals.var_isubibpc_dn14 = assign68020_e104688_d_n14;

        let assign68030_e104691: f64 = if p.p24 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1607 = assign68030_e104691;

        let assign68040_e104694: f64 = if locals.var_flg_noqi == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1608 = assign68040_e104694;

        let (assign68050_e104706, assign68050_e104706_d_n0, assign68050_e104706_d_n2, assign68050_e104706_d_n4, assign68050_e104706_d_n5, assign68050_e104706_d_n6, assign68050_e104706_d_n7, assign68050_e104706_d_n8, assign68050_e104706_d_n9, assign68050_e104706_d_n10, assign68050_e104706_d_n11, assign68050_e104706_d_n14,) = {
    if ((locals.var_guard1607 != 0.0) && (locals.var_guard1608 != 0.0)) {
        let assign68050_e104700: f64 = (locals.var_ps0z + locals.var_vdsz__blk443);
        let assign68050_e104703: f64 = (10.0 * 2.220446049250313e-16);
        let assign68050_e104704: f64 = (assign68050_e104700 - assign68050_e104703);
        (assign68050_e104704, (locals.var_ps0z_dn0 + locals.var_vdsz__blk443_dn0), (locals.var_ps0z_dn2 + locals.var_vdsz__blk443_dn2), (locals.var_ps0z_dn4 + locals.var_vdsz__blk443_dn4), (locals.var_ps0z_dn5 + locals.var_vdsz__blk443_dn5), (locals.var_ps0z_dn6 + locals.var_vdsz__blk443_dn6), (locals.var_ps0z_dn7 + locals.var_vdsz__blk443_dn7), (locals.var_ps0z_dn8 + locals.var_vdsz__blk443_dn8), (locals.var_ps0z_dn9 + locals.var_vdsz__blk443_dn9), (locals.var_ps0z_dn10 + locals.var_vdsz__blk443_dn10), (locals.var_ps0z_dn11 + locals.var_vdsz__blk443_dn11), (locals.var_ps0z_dn14 + locals.var_vdsz__blk443_dn14),)
    } else {
        (locals.var_psdlz, locals.var_psdlz_dn0, locals.var_psdlz_dn2, locals.var_psdlz_dn4, locals.var_psdlz_dn5, locals.var_psdlz_dn6, locals.var_psdlz_dn7, locals.var_psdlz_dn8, locals.var_psdlz_dn9, locals.var_psdlz_dn10, locals.var_psdlz_dn11, locals.var_psdlz_dn14,)
    }
};
        locals.var_psdlz = assign68050_e104706;
        locals.var_psdlz_dn0 = assign68050_e104706_d_n0;
        locals.var_psdlz_dn2 = assign68050_e104706_d_n2;
        locals.var_psdlz_dn4 = assign68050_e104706_d_n4;
        locals.var_psdlz_dn5 = assign68050_e104706_d_n5;
        locals.var_psdlz_dn6 = assign68050_e104706_d_n6;
        locals.var_psdlz_dn7 = assign68050_e104706_d_n7;
        locals.var_psdlz_dn8 = assign68050_e104706_d_n8;
        locals.var_psdlz_dn9 = assign68050_e104706_d_n9;
        locals.var_psdlz_dn10 = assign68050_e104706_d_n10;
        locals.var_psdlz_dn11 = assign68050_e104706_d_n11;
        locals.var_psdlz_dn14 = assign68050_e104706_d_n14;

        let (assign68060_e104726, assign68060_e104726_d_n0, assign68060_e104726_d_n2, assign68060_e104726_d_n4, assign68060_e104726_d_n5, assign68060_e104726_d_n6, assign68060_e104726_d_n7, assign68060_e104726_d_n8, assign68060_e104726_d_n9, assign68060_e104726_d_n10, assign68060_e104726_d_n11, assign68060_e104726_d_n14,) = {
    if ((locals.var_guard1607 != 0.0) && (locals.var_guard1608 != 0.0)) {
        let assign68060_e104712: f64 = (locals.var_vgsz__blk444 - locals.var_vfb);
        let assign68060_e104716: f64 = (locals.var_dvth - locals.var_dppg);
        let assign68060_e104717: f64 = (locals.var_mks_gleak4 * assign68060_e104716);
        let assign68060_e104719: f64 = (assign68060_e104717 * locals.var_leff);
        let assign68060_e104720: f64 = (assign68060_e104712 + assign68060_e104719);
        let assign68060_e104723: f64 = (locals.var_psdlz * locals.var_uc_gleak3);
        let assign68060_e104724: f64 = (assign68060_e104720 - assign68060_e104723);
        (assign68060_e104724, ((locals.var_vgsz__blk444_dn0 + ((locals.var_mks_gleak4 * (locals.var_dvth_dn0 - locals.var_dppg_dn0)) * locals.var_leff)) - (locals.var_psdlz_dn0 * locals.var_uc_gleak3)), ((locals.var_vgsz__blk444_dn2 + ((locals.var_mks_gleak4 * (locals.var_dvth_dn2 - locals.var_dppg_dn2)) * locals.var_leff)) - (locals.var_psdlz_dn2 * locals.var_uc_gleak3)), ((locals.var_vgsz__blk444_dn4 + ((locals.var_mks_gleak4 * (locals.var_dvth_dn4 - locals.var_dppg_dn4)) * locals.var_leff)) - (locals.var_psdlz_dn4 * locals.var_uc_gleak3)), ((locals.var_vgsz__blk444_dn5 + ((locals.var_mks_gleak4 * (locals.var_dvth_dn5 - locals.var_dppg_dn5)) * locals.var_leff)) - (locals.var_psdlz_dn5 * locals.var_uc_gleak3)), ((locals.var_vgsz__blk444_dn6 + ((locals.var_mks_gleak4 * (locals.var_dvth_dn6 - locals.var_dppg_dn6)) * locals.var_leff)) - (locals.var_psdlz_dn6 * locals.var_uc_gleak3)), ((locals.var_vgsz__blk444_dn7 + ((locals.var_mks_gleak4 * (locals.var_dvth_dn7 - locals.var_dppg_dn7)) * locals.var_leff)) - (locals.var_psdlz_dn7 * locals.var_uc_gleak3)), ((locals.var_vgsz__blk444_dn8 + ((locals.var_mks_gleak4 * (locals.var_dvth_dn8 - locals.var_dppg_dn8)) * locals.var_leff)) - (locals.var_psdlz_dn8 * locals.var_uc_gleak3)), ((locals.var_vgsz__blk444_dn9 + ((locals.var_mks_gleak4 * (locals.var_dvth_dn9 - locals.var_dppg_dn9)) * locals.var_leff)) - (locals.var_psdlz_dn9 * locals.var_uc_gleak3)), ((locals.var_vgsz__blk444_dn10 + ((locals.var_mks_gleak4 * (locals.var_dvth_dn10 - locals.var_dppg_dn10)) * locals.var_leff)) - (locals.var_psdlz_dn10 * locals.var_uc_gleak3)), ((locals.var_vgsz__blk444_dn11 + ((locals.var_mks_gleak4 * (locals.var_dvth_dn11 - locals.var_dppg_dn11)) * locals.var_leff)) - (locals.var_psdlz_dn11 * locals.var_uc_gleak3)), ((locals.var_vgsz__blk444_dn14 + ((locals.var_mks_gleak4 * (locals.var_dvth_dn14 - locals.var_dppg_dn14)) * locals.var_leff)) - (locals.var_psdlz_dn14 * locals.var_uc_gleak3)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign68060_e104726;
        locals.var_t1_dn0 = assign68060_e104726_d_n0;
        locals.var_t1_dn2 = assign68060_e104726_d_n2;
        locals.var_t1_dn4 = assign68060_e104726_d_n4;
        locals.var_t1_dn5 = assign68060_e104726_d_n5;
        locals.var_t1_dn6 = assign68060_e104726_d_n6;
        locals.var_t1_dn7 = assign68060_e104726_d_n7;
        locals.var_t1_dn8 = assign68060_e104726_d_n8;
        locals.var_t1_dn9 = assign68060_e104726_d_n9;
        locals.var_t1_dn10 = assign68060_e104726_d_n10;
        locals.var_t1_dn11 = assign68060_e104726_d_n11;
        locals.var_t1_dn14 = assign68060_e104726_d_n14;

        let (assign68070_e104734, assign68070_e104734_d_n0, assign68070_e104734_d_n2, assign68070_e104734_d_n4, assign68070_e104734_d_n5, assign68070_e104734_d_n6, assign68070_e104734_d_n7, assign68070_e104734_d_n8, assign68070_e104734_d_n9, assign68070_e104734_d_n10, assign68070_e104734_d_n11, assign68070_e104734_d_n14,) = {
    if ((locals.var_guard1607 != 0.0) && (locals.var_guard1608 != 0.0)) {
        let assign68070_e104732: f64 = (locals.var_t1 * locals.var_t1);
        (assign68070_e104732, ((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)), ((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)), ((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)), ((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)), ((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)), ((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)), ((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)), ((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)), ((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)), ((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)), ((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign68070_e104734;
        locals.var_t1_dn0 = assign68070_e104734_d_n0;
        locals.var_t1_dn2 = assign68070_e104734_d_n2;
        locals.var_t1_dn4 = assign68070_e104734_d_n4;
        locals.var_t1_dn5 = assign68070_e104734_d_n5;
        locals.var_t1_dn6 = assign68070_e104734_d_n6;
        locals.var_t1_dn7 = assign68070_e104734_d_n7;
        locals.var_t1_dn8 = assign68070_e104734_d_n8;
        locals.var_t1_dn9 = assign68070_e104734_d_n9;
        locals.var_t1_dn10 = assign68070_e104734_d_n10;
        locals.var_t1_dn11 = assign68070_e104734_d_n11;
        locals.var_t1_dn14 = assign68070_e104734_d_n14;

        let (assign68080_e104742, assign68080_e104742_d_n0, assign68080_e104742_d_n2, assign68080_e104742_d_n4, assign68080_e104742_d_n5, assign68080_e104742_d_n6, assign68080_e104742_d_n7, assign68080_e104742_d_n8, assign68080_e104742_d_n9, assign68080_e104742_d_n10, assign68080_e104742_d_n11, assign68080_e104742_d_n14,) = {
    if ((locals.var_guard1607 != 0.0) && (locals.var_guard1608 != 0.0)) {
        let assign68080_e104740: f64 = (1.0 / locals.var_tox0);
        (assign68080_e104740, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign68080_e104742;
        locals.var_t3_dn0 = assign68080_e104742_d_n0;
        locals.var_t3_dn2 = assign68080_e104742_d_n2;
        locals.var_t3_dn4 = assign68080_e104742_d_n4;
        locals.var_t3_dn5 = assign68080_e104742_d_n5;
        locals.var_t3_dn6 = assign68080_e104742_d_n6;
        locals.var_t3_dn7 = assign68080_e104742_d_n7;
        locals.var_t3_dn8 = assign68080_e104742_d_n8;
        locals.var_t3_dn9 = assign68080_e104742_d_n9;
        locals.var_t3_dn10 = assign68080_e104742_d_n10;
        locals.var_t3_dn11 = assign68080_e104742_d_n11;
        locals.var_t3_dn14 = assign68080_e104742_d_n14;

        let (assign68090_e104750, assign68090_e104750_d_n0, assign68090_e104750_d_n2, assign68090_e104750_d_n4, assign68090_e104750_d_n5, assign68090_e104750_d_n6, assign68090_e104750_d_n7, assign68090_e104750_d_n8, assign68090_e104750_d_n9, assign68090_e104750_d_n10, assign68090_e104750_d_n11, assign68090_e104750_d_n14,) = {
    if ((locals.var_guard1607 != 0.0) && (locals.var_guard1608 != 0.0)) {
        let assign68090_e104748: f64 = (locals.var_t1 * locals.var_t3);
        (assign68090_e104748, ((locals.var_t1_dn0 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn0)), ((locals.var_t1_dn2 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn2)), ((locals.var_t1_dn4 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn4)), ((locals.var_t1_dn5 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn5)), ((locals.var_t1_dn6 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn6)), ((locals.var_t1_dn7 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn7)), ((locals.var_t1_dn8 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn8)), ((locals.var_t1_dn9 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn9)), ((locals.var_t1_dn10 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn10)), ((locals.var_t1_dn11 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn11)), ((locals.var_t1_dn14 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign68090_e104750;
        locals.var_t2_dn0 = assign68090_e104750_d_n0;
        locals.var_t2_dn2 = assign68090_e104750_d_n2;
        locals.var_t2_dn4 = assign68090_e104750_d_n4;
        locals.var_t2_dn5 = assign68090_e104750_d_n5;
        locals.var_t2_dn6 = assign68090_e104750_d_n6;
        locals.var_t2_dn7 = assign68090_e104750_d_n7;
        locals.var_t2_dn8 = assign68090_e104750_d_n8;
        locals.var_t2_dn9 = assign68090_e104750_d_n9;
        locals.var_t2_dn10 = assign68090_e104750_d_n10;
        locals.var_t2_dn11 = assign68090_e104750_d_n11;
        locals.var_t2_dn14 = assign68090_e104750_d_n14;

        let (assign68100_e104758, assign68100_e104758_d_n0, assign68100_e104758_d_n2, assign68100_e104758_d_n4, assign68100_e104758_d_n5, assign68100_e104758_d_n6, assign68100_e104758_d_n7, assign68100_e104758_d_n8, assign68100_e104758_d_n9, assign68100_e104758_d_n10, assign68100_e104758_d_n11, assign68100_e104758_d_n14,) = {
    if ((locals.var_guard1607 != 0.0) && (locals.var_guard1608 != 0.0)) {
        let assign68100_e104756: f64 = (1.0 / locals.var_mks_gleak5);
        (assign68100_e104756, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign68100_e104758;
        locals.var_t3_dn0 = assign68100_e104758_d_n0;
        locals.var_t3_dn2 = assign68100_e104758_d_n2;
        locals.var_t3_dn4 = assign68100_e104758_d_n4;
        locals.var_t3_dn5 = assign68100_e104758_d_n5;
        locals.var_t3_dn6 = assign68100_e104758_d_n6;
        locals.var_t3_dn7 = assign68100_e104758_d_n7;
        locals.var_t3_dn8 = assign68100_e104758_d_n8;
        locals.var_t3_dn9 = assign68100_e104758_d_n9;
        locals.var_t3_dn10 = assign68100_e104758_d_n10;
        locals.var_t3_dn11 = assign68100_e104758_d_n11;
        locals.var_t3_dn14 = assign68100_e104758_d_n14;

        let (assign68110_e104768, assign68110_e104768_d_n0, assign68110_e104768_d_n2, assign68110_e104768_d_n4, assign68110_e104768_d_n5, assign68110_e104768_d_n6, assign68110_e104768_d_n7, assign68110_e104768_d_n8, assign68110_e104768_d_n9, assign68110_e104768_d_n10, assign68110_e104768_d_n11, assign68110_e104768_d_n14,) = {
    if ((locals.var_guard1607 != 0.0) && (locals.var_guard1608 != 0.0)) {
        let assign68110_e104765: f64 = (locals.var_ey * locals.var_t3);
        let assign68110_e104766: f64 = (1.0 + assign68110_e104765);
        (assign68110_e104766, ((locals.var_ey_dn0 * locals.var_t3) + (locals.var_ey * locals.var_t3_dn0)), ((locals.var_ey_dn2 * locals.var_t3) + (locals.var_ey * locals.var_t3_dn2)), ((locals.var_ey_dn4 * locals.var_t3) + (locals.var_ey * locals.var_t3_dn4)), ((locals.var_ey_dn5 * locals.var_t3) + (locals.var_ey * locals.var_t3_dn5)), ((locals.var_ey_dn6 * locals.var_t3) + (locals.var_ey * locals.var_t3_dn6)), ((locals.var_ey_dn7 * locals.var_t3) + (locals.var_ey * locals.var_t3_dn7)), ((locals.var_ey_dn8 * locals.var_t3) + (locals.var_ey * locals.var_t3_dn8)), ((locals.var_ey_dn9 * locals.var_t3) + (locals.var_ey * locals.var_t3_dn9)), ((locals.var_ey_dn10 * locals.var_t3) + (locals.var_ey * locals.var_t3_dn10)), ((locals.var_ey_dn11 * locals.var_t3) + (locals.var_ey * locals.var_t3_dn11)), ((locals.var_ey_dn14 * locals.var_t3) + (locals.var_ey * locals.var_t3_dn14)),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign68110_e104768;
        locals.var_t7_dn0 = assign68110_e104768_d_n0;
        locals.var_t7_dn2 = assign68110_e104768_d_n2;
        locals.var_t7_dn4 = assign68110_e104768_d_n4;
        locals.var_t7_dn5 = assign68110_e104768_d_n5;
        locals.var_t7_dn6 = assign68110_e104768_d_n6;
        locals.var_t7_dn7 = assign68110_e104768_d_n7;
        locals.var_t7_dn8 = assign68110_e104768_d_n8;
        locals.var_t7_dn9 = assign68110_e104768_d_n9;
        locals.var_t7_dn10 = assign68110_e104768_d_n10;
        locals.var_t7_dn11 = assign68110_e104768_d_n11;
        locals.var_t7_dn14 = assign68110_e104768_d_n14;

    }

    pub(super) fn stamp_transient_block_243(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign68120_e104776, assign68120_e104776_d_n0, assign68120_e104776_d_n2, assign68120_e104776_d_n4, assign68120_e104776_d_n5, assign68120_e104776_d_n6, assign68120_e104776_d_n7, assign68120_e104776_d_n8, assign68120_e104776_d_n9, assign68120_e104776_d_n10, assign68120_e104776_d_n11, assign68120_e104776_d_n14,) = {
    if ((locals.var_guard1607 != 0.0) && (locals.var_guard1608 != 0.0)) {
        let assign68120_e104774: f64 = (locals.var_t2 * locals.var_t7);
        (assign68120_e104774, ((locals.var_t2_dn0 * locals.var_t7) + (locals.var_t2 * locals.var_t7_dn0)), ((locals.var_t2_dn2 * locals.var_t7) + (locals.var_t2 * locals.var_t7_dn2)), ((locals.var_t2_dn4 * locals.var_t7) + (locals.var_t2 * locals.var_t7_dn4)), ((locals.var_t2_dn5 * locals.var_t7) + (locals.var_t2 * locals.var_t7_dn5)), ((locals.var_t2_dn6 * locals.var_t7) + (locals.var_t2 * locals.var_t7_dn6)), ((locals.var_t2_dn7 * locals.var_t7) + (locals.var_t2 * locals.var_t7_dn7)), ((locals.var_t2_dn8 * locals.var_t7) + (locals.var_t2 * locals.var_t7_dn8)), ((locals.var_t2_dn9 * locals.var_t7) + (locals.var_t2 * locals.var_t7_dn9)), ((locals.var_t2_dn10 * locals.var_t7) + (locals.var_t2 * locals.var_t7_dn10)), ((locals.var_t2_dn11 * locals.var_t7) + (locals.var_t2 * locals.var_t7_dn11)), ((locals.var_t2_dn14 * locals.var_t7) + (locals.var_t2 * locals.var_t7_dn14)),)
    } else {
        (locals.var_etun, locals.var_etun_dn0, locals.var_etun_dn2, locals.var_etun_dn4, locals.var_etun_dn5, locals.var_etun_dn6, locals.var_etun_dn7, locals.var_etun_dn8, locals.var_etun_dn9, locals.var_etun_dn10, locals.var_etun_dn11, locals.var_etun_dn14,)
    }
};
        locals.var_etun = assign68120_e104776;
        locals.var_etun_dn0 = assign68120_e104776_d_n0;
        locals.var_etun_dn2 = assign68120_e104776_d_n2;
        locals.var_etun_dn4 = assign68120_e104776_d_n4;
        locals.var_etun_dn5 = assign68120_e104776_d_n5;
        locals.var_etun_dn6 = assign68120_e104776_d_n6;
        locals.var_etun_dn7 = assign68120_e104776_d_n7;
        locals.var_etun_dn8 = assign68120_e104776_d_n8;
        locals.var_etun_dn9 = assign68120_e104776_d_n9;
        locals.var_etun_dn10 = assign68120_e104776_d_n10;
        locals.var_etun_dn11 = assign68120_e104776_d_n11;
        locals.var_etun_dn14 = assign68120_e104776_d_n14;

        let (assign68130_e104795, assign68130_e104795_d_n0, assign68130_e104795_d_n2, assign68130_e104795_d_n4, assign68130_e104795_d_n5, assign68130_e104795_d_n6, assign68130_e104795_d_n7, assign68130_e104795_d_n8, assign68130_e104795_d_n9, assign68130_e104795_d_n10, assign68130_e104795_d_n11, assign68130_e104795_d_n14,) = {
    if ((locals.var_guard1607 != 0.0) && (locals.var_guard1608 != 0.0)) {
        let assign68130_e104782: f64 = (locals.var_etun * locals.var_etun);
        let assign68130_e104786: f64 = (0.01 / 0.01);
        let assign68130_e104787: f64 = (4.0 * assign68130_e104786);
        let assign68130_e104790: f64 = (0.01 / 0.01);
        let assign68130_e104791: f64 = (assign68130_e104787 * assign68130_e104790);
        let assign68130_e104792: f64 = (assign68130_e104782 + assign68130_e104791);
        let assign68130_e104793: f64 = (assign68130_e104792).sqrt();
        (assign68130_e104793, (((locals.var_etun_dn0 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn0)) / (2.0 * assign68130_e104793)), (((locals.var_etun_dn2 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn2)) / (2.0 * assign68130_e104793)), (((locals.var_etun_dn4 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn4)) / (2.0 * assign68130_e104793)), (((locals.var_etun_dn5 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn5)) / (2.0 * assign68130_e104793)), (((locals.var_etun_dn6 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn6)) / (2.0 * assign68130_e104793)), (((locals.var_etun_dn7 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn7)) / (2.0 * assign68130_e104793)), (((locals.var_etun_dn8 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn8)) / (2.0 * assign68130_e104793)), (((locals.var_etun_dn9 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn9)) / (2.0 * assign68130_e104793)), (((locals.var_etun_dn10 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn10)) / (2.0 * assign68130_e104793)), (((locals.var_etun_dn11 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn11)) / (2.0 * assign68130_e104793)), (((locals.var_etun_dn14 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn14)) / (2.0 * assign68130_e104793)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign68130_e104795;
        locals.var_tmf2_dn0 = assign68130_e104795_d_n0;
        locals.var_tmf2_dn2 = assign68130_e104795_d_n2;
        locals.var_tmf2_dn4 = assign68130_e104795_d_n4;
        locals.var_tmf2_dn5 = assign68130_e104795_d_n5;
        locals.var_tmf2_dn6 = assign68130_e104795_d_n6;
        locals.var_tmf2_dn7 = assign68130_e104795_d_n7;
        locals.var_tmf2_dn8 = assign68130_e104795_d_n8;
        locals.var_tmf2_dn9 = assign68130_e104795_d_n9;
        locals.var_tmf2_dn10 = assign68130_e104795_d_n10;
        locals.var_tmf2_dn11 = assign68130_e104795_d_n11;
        locals.var_tmf2_dn14 = assign68130_e104795_d_n14;

        let (assign68140_e104807, assign68140_e104807_d_n0, assign68140_e104807_d_n2, assign68140_e104807_d_n4, assign68140_e104807_d_n5, assign68140_e104807_d_n6, assign68140_e104807_d_n7, assign68140_e104807_d_n8, assign68140_e104807_d_n9, assign68140_e104807_d_n10, assign68140_e104807_d_n11, assign68140_e104807_d_n14,) = {
    if ((locals.var_guard1607 != 0.0) && (locals.var_guard1608 != 0.0)) {
        let assign68140_e104803: f64 = (locals.var_etun / locals.var_tmf2);
        let assign68140_e104804: f64 = (1.0 + assign68140_e104803);
        let assign68140_e104805: f64 = (0.5 * assign68140_e104804);
        (assign68140_e104805, (0.5 * (((locals.var_etun_dn0 * locals.var_tmf2) - (locals.var_etun * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_etun_dn2 * locals.var_tmf2) - (locals.var_etun * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_etun_dn4 * locals.var_tmf2) - (locals.var_etun * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_etun_dn5 * locals.var_tmf2) - (locals.var_etun * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_etun_dn6 * locals.var_tmf2) - (locals.var_etun * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_etun_dn7 * locals.var_tmf2) - (locals.var_etun * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_etun_dn8 * locals.var_tmf2) - (locals.var_etun * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_etun_dn9 * locals.var_tmf2) - (locals.var_etun * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_etun_dn10 * locals.var_tmf2) - (locals.var_etun * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_etun_dn11 * locals.var_tmf2) - (locals.var_etun * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_etun_dn14 * locals.var_tmf2) - (locals.var_etun * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign68140_e104807;
        locals.var_t5_dn0 = assign68140_e104807_d_n0;
        locals.var_t5_dn2 = assign68140_e104807_d_n2;
        locals.var_t5_dn4 = assign68140_e104807_d_n4;
        locals.var_t5_dn5 = assign68140_e104807_d_n5;
        locals.var_t5_dn6 = assign68140_e104807_d_n6;
        locals.var_t5_dn7 = assign68140_e104807_d_n7;
        locals.var_t5_dn8 = assign68140_e104807_d_n8;
        locals.var_t5_dn9 = assign68140_e104807_d_n9;
        locals.var_t5_dn10 = assign68140_e104807_d_n10;
        locals.var_t5_dn11 = assign68140_e104807_d_n11;
        locals.var_t5_dn14 = assign68140_e104807_d_n14;

        let (assign68150_e104817, assign68150_e104817_d_n0, assign68150_e104817_d_n2, assign68150_e104817_d_n4, assign68150_e104817_d_n5, assign68150_e104817_d_n6, assign68150_e104817_d_n7, assign68150_e104817_d_n8, assign68150_e104817_d_n9, assign68150_e104817_d_n10, assign68150_e104817_d_n11, assign68150_e104817_d_n14,) = {
    if ((locals.var_guard1607 != 0.0) && (locals.var_guard1608 != 0.0)) {
        let assign68150_e104814: f64 = (locals.var_etun + locals.var_tmf2);
        let assign68150_e104815: f64 = (0.5 * assign68150_e104814);
        (assign68150_e104815, (0.5 * (locals.var_etun_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_etun_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_etun_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_etun_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_etun_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_etun_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_etun_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_etun_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_etun_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_etun_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_etun_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_etun, locals.var_etun_dn0, locals.var_etun_dn2, locals.var_etun_dn4, locals.var_etun_dn5, locals.var_etun_dn6, locals.var_etun_dn7, locals.var_etun_dn8, locals.var_etun_dn9, locals.var_etun_dn10, locals.var_etun_dn11, locals.var_etun_dn14,)
    }
};
        locals.var_etun = assign68150_e104817;
        locals.var_etun_dn0 = assign68150_e104817_d_n0;
        locals.var_etun_dn2 = assign68150_e104817_d_n2;
        locals.var_etun_dn4 = assign68150_e104817_d_n4;
        locals.var_etun_dn5 = assign68150_e104817_d_n5;
        locals.var_etun_dn6 = assign68150_e104817_d_n6;
        locals.var_etun_dn7 = assign68150_e104817_d_n7;
        locals.var_etun_dn8 = assign68150_e104817_d_n8;
        locals.var_etun_dn9 = assign68150_e104817_d_n9;
        locals.var_etun_dn10 = assign68150_e104817_d_n10;
        locals.var_etun_dn11 = assign68150_e104817_d_n11;
        locals.var_etun_dn14 = assign68150_e104817_d_n14;

        let assign68160_e104820: f64 = if locals.var_etun < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1609 = assign68160_e104820;

        let (assign68170_e104828, assign68170_e104828_d_n0, assign68170_e104828_d_n2, assign68170_e104828_d_n4, assign68170_e104828_d_n5, assign68170_e104828_d_n6, assign68170_e104828_d_n7, assign68170_e104828_d_n8, assign68170_e104828_d_n9, assign68170_e104828_d_n10, assign68170_e104828_d_n11, assign68170_e104828_d_n14,) = {
    if (((locals.var_guard1607 != 0.0) && (locals.var_guard1608 != 0.0)) && (locals.var_guard1609 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_etun, locals.var_etun_dn0, locals.var_etun_dn2, locals.var_etun_dn4, locals.var_etun_dn5, locals.var_etun_dn6, locals.var_etun_dn7, locals.var_etun_dn8, locals.var_etun_dn9, locals.var_etun_dn10, locals.var_etun_dn11, locals.var_etun_dn14,)
    }
};
        locals.var_etun = assign68170_e104828;
        locals.var_etun_dn0 = assign68170_e104828_d_n0;
        locals.var_etun_dn2 = assign68170_e104828_d_n2;
        locals.var_etun_dn4 = assign68170_e104828_d_n4;
        locals.var_etun_dn5 = assign68170_e104828_d_n5;
        locals.var_etun_dn6 = assign68170_e104828_d_n6;
        locals.var_etun_dn7 = assign68170_e104828_d_n7;
        locals.var_etun_dn8 = assign68170_e104828_d_n8;
        locals.var_etun_dn9 = assign68170_e104828_d_n9;
        locals.var_etun_dn10 = assign68170_e104828_d_n10;
        locals.var_etun_dn11 = assign68170_e104828_d_n11;
        locals.var_etun_dn14 = assign68170_e104828_d_n14;

        let (assign68180_e104836, assign68180_e104836_d_n0, assign68180_e104836_d_n2, assign68180_e104836_d_n4, assign68180_e104836_d_n5, assign68180_e104836_d_n6, assign68180_e104836_d_n7, assign68180_e104836_d_n8, assign68180_e104836_d_n9, assign68180_e104836_d_n10, assign68180_e104836_d_n11, assign68180_e104836_d_n14,) = {
    if (((locals.var_guard1607 != 0.0) && (locals.var_guard1608 != 0.0)) && (locals.var_guard1609 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign68180_e104836;
        locals.var_t5_dn0 = assign68180_e104836_d_n0;
        locals.var_t5_dn2 = assign68180_e104836_d_n2;
        locals.var_t5_dn4 = assign68180_e104836_d_n4;
        locals.var_t5_dn5 = assign68180_e104836_d_n5;
        locals.var_t5_dn6 = assign68180_e104836_d_n6;
        locals.var_t5_dn7 = assign68180_e104836_d_n7;
        locals.var_t5_dn8 = assign68180_e104836_d_n8;
        locals.var_t5_dn9 = assign68180_e104836_d_n9;
        locals.var_t5_dn10 = assign68180_e104836_d_n10;
        locals.var_t5_dn11 = assign68180_e104836_d_n11;
        locals.var_t5_dn14 = assign68180_e104836_d_n14;

        let (assign68190_e104851, assign68190_e104851_d_n0, assign68190_e104851_d_n2, assign68190_e104851_d_n4, assign68190_e104851_d_n5, assign68190_e104851_d_n6, assign68190_e104851_d_n7, assign68190_e104851_d_n8, assign68190_e104851_d_n9, assign68190_e104851_d_n10, assign68190_e104851_d_n11, assign68190_e104851_d_n14,) = {
    if ((locals.var_guard1607 != 0.0) && (locals.var_guard1608 != 0.0)) {
        let assign68190_e104842: f64 = (locals.var_vgsz__blk444 * locals.var_vgsz__blk444);
        let assign68190_e104845: f64 = (4.0 * 0.001);
        let assign68190_e104847: f64 = (assign68190_e104845 * 0.001);
        let assign68190_e104848: f64 = (assign68190_e104842 + assign68190_e104847);
        let assign68190_e104849: f64 = (assign68190_e104848).sqrt();
        (assign68190_e104849, (((locals.var_vgsz__blk444_dn0 * locals.var_vgsz__blk444) + (locals.var_vgsz__blk444 * locals.var_vgsz__blk444_dn0)) / (2.0 * assign68190_e104849)), (((locals.var_vgsz__blk444_dn2 * locals.var_vgsz__blk444) + (locals.var_vgsz__blk444 * locals.var_vgsz__blk444_dn2)) / (2.0 * assign68190_e104849)), (((locals.var_vgsz__blk444_dn4 * locals.var_vgsz__blk444) + (locals.var_vgsz__blk444 * locals.var_vgsz__blk444_dn4)) / (2.0 * assign68190_e104849)), (((locals.var_vgsz__blk444_dn5 * locals.var_vgsz__blk444) + (locals.var_vgsz__blk444 * locals.var_vgsz__blk444_dn5)) / (2.0 * assign68190_e104849)), (((locals.var_vgsz__blk444_dn6 * locals.var_vgsz__blk444) + (locals.var_vgsz__blk444 * locals.var_vgsz__blk444_dn6)) / (2.0 * assign68190_e104849)), (((locals.var_vgsz__blk444_dn7 * locals.var_vgsz__blk444) + (locals.var_vgsz__blk444 * locals.var_vgsz__blk444_dn7)) / (2.0 * assign68190_e104849)), (((locals.var_vgsz__blk444_dn8 * locals.var_vgsz__blk444) + (locals.var_vgsz__blk444 * locals.var_vgsz__blk444_dn8)) / (2.0 * assign68190_e104849)), (((locals.var_vgsz__blk444_dn9 * locals.var_vgsz__blk444) + (locals.var_vgsz__blk444 * locals.var_vgsz__blk444_dn9)) / (2.0 * assign68190_e104849)), (((locals.var_vgsz__blk444_dn10 * locals.var_vgsz__blk444) + (locals.var_vgsz__blk444 * locals.var_vgsz__blk444_dn10)) / (2.0 * assign68190_e104849)), (((locals.var_vgsz__blk444_dn11 * locals.var_vgsz__blk444) + (locals.var_vgsz__blk444 * locals.var_vgsz__blk444_dn11)) / (2.0 * assign68190_e104849)), (((locals.var_vgsz__blk444_dn14 * locals.var_vgsz__blk444) + (locals.var_vgsz__blk444 * locals.var_vgsz__blk444_dn14)) / (2.0 * assign68190_e104849)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign68190_e104851;
        locals.var_tmf2_dn0 = assign68190_e104851_d_n0;
        locals.var_tmf2_dn2 = assign68190_e104851_d_n2;
        locals.var_tmf2_dn4 = assign68190_e104851_d_n4;
        locals.var_tmf2_dn5 = assign68190_e104851_d_n5;
        locals.var_tmf2_dn6 = assign68190_e104851_d_n6;
        locals.var_tmf2_dn7 = assign68190_e104851_d_n7;
        locals.var_tmf2_dn8 = assign68190_e104851_d_n8;
        locals.var_tmf2_dn9 = assign68190_e104851_d_n9;
        locals.var_tmf2_dn10 = assign68190_e104851_d_n10;
        locals.var_tmf2_dn11 = assign68190_e104851_d_n11;
        locals.var_tmf2_dn14 = assign68190_e104851_d_n14;

        let (assign68200_e104863, assign68200_e104863_d_n0, assign68200_e104863_d_n2, assign68200_e104863_d_n4, assign68200_e104863_d_n5, assign68200_e104863_d_n6, assign68200_e104863_d_n7, assign68200_e104863_d_n8, assign68200_e104863_d_n9, assign68200_e104863_d_n10, assign68200_e104863_d_n11, assign68200_e104863_d_n14,) = {
    if ((locals.var_guard1607 != 0.0) && (locals.var_guard1608 != 0.0)) {
        let assign68200_e104859: f64 = (locals.var_vgsz__blk444 / locals.var_tmf2);
        let assign68200_e104860: f64 = (1.0 + assign68200_e104859);
        let assign68200_e104861: f64 = (0.5 * assign68200_e104860);
        (assign68200_e104861, (0.5 * (((locals.var_vgsz__blk444_dn0 * locals.var_tmf2) - (locals.var_vgsz__blk444 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vgsz__blk444_dn2 * locals.var_tmf2) - (locals.var_vgsz__blk444 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vgsz__blk444_dn4 * locals.var_tmf2) - (locals.var_vgsz__blk444 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vgsz__blk444_dn5 * locals.var_tmf2) - (locals.var_vgsz__blk444 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vgsz__blk444_dn6 * locals.var_tmf2) - (locals.var_vgsz__blk444 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vgsz__blk444_dn7 * locals.var_tmf2) - (locals.var_vgsz__blk444 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vgsz__blk444_dn8 * locals.var_tmf2) - (locals.var_vgsz__blk444 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vgsz__blk444_dn9 * locals.var_tmf2) - (locals.var_vgsz__blk444 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vgsz__blk444_dn10 * locals.var_tmf2) - (locals.var_vgsz__blk444 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vgsz__blk444_dn11 * locals.var_tmf2) - (locals.var_vgsz__blk444 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vgsz__blk444_dn14 * locals.var_tmf2) - (locals.var_vgsz__blk444 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign68200_e104863;
        locals.var_t4_dn0 = assign68200_e104863_d_n0;
        locals.var_t4_dn2 = assign68200_e104863_d_n2;
        locals.var_t4_dn4 = assign68200_e104863_d_n4;
        locals.var_t4_dn5 = assign68200_e104863_d_n5;
        locals.var_t4_dn6 = assign68200_e104863_d_n6;
        locals.var_t4_dn7 = assign68200_e104863_d_n7;
        locals.var_t4_dn8 = assign68200_e104863_d_n8;
        locals.var_t4_dn9 = assign68200_e104863_d_n9;
        locals.var_t4_dn10 = assign68200_e104863_d_n10;
        locals.var_t4_dn11 = assign68200_e104863_d_n11;
        locals.var_t4_dn14 = assign68200_e104863_d_n14;

        let (assign68210_e104873, assign68210_e104873_d_n0, assign68210_e104873_d_n2, assign68210_e104873_d_n4, assign68210_e104873_d_n5, assign68210_e104873_d_n6, assign68210_e104873_d_n7, assign68210_e104873_d_n8, assign68210_e104873_d_n9, assign68210_e104873_d_n10, assign68210_e104873_d_n11, assign68210_e104873_d_n14,) = {
    if ((locals.var_guard1607 != 0.0) && (locals.var_guard1608 != 0.0)) {
        let assign68210_e104870: f64 = (locals.var_vgsz__blk444 + locals.var_tmf2);
        let assign68210_e104871: f64 = (0.5 * assign68210_e104870);
        (assign68210_e104871, (0.5 * (locals.var_vgsz__blk444_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_vgsz__blk444_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_vgsz__blk444_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_vgsz__blk444_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_vgsz__blk444_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_vgsz__blk444_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_vgsz__blk444_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_vgsz__blk444_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_vgsz__blk444_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_vgsz__blk444_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_vgsz__blk444_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign68210_e104873;
        locals.var_t3_dn0 = assign68210_e104873_d_n0;
        locals.var_t3_dn2 = assign68210_e104873_d_n2;
        locals.var_t3_dn4 = assign68210_e104873_d_n4;
        locals.var_t3_dn5 = assign68210_e104873_d_n5;
        locals.var_t3_dn6 = assign68210_e104873_d_n6;
        locals.var_t3_dn7 = assign68210_e104873_d_n7;
        locals.var_t3_dn8 = assign68210_e104873_d_n8;
        locals.var_t3_dn9 = assign68210_e104873_d_n9;
        locals.var_t3_dn10 = assign68210_e104873_d_n10;
        locals.var_t3_dn11 = assign68210_e104873_d_n11;
        locals.var_t3_dn14 = assign68210_e104873_d_n14;

        let assign68220_e104876: f64 = if locals.var_t3 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1610 = assign68220_e104876;

        let (assign68230_e104884, assign68230_e104884_d_n0, assign68230_e104884_d_n2, assign68230_e104884_d_n4, assign68230_e104884_d_n5, assign68230_e104884_d_n6, assign68230_e104884_d_n7, assign68230_e104884_d_n8, assign68230_e104884_d_n9, assign68230_e104884_d_n10, assign68230_e104884_d_n11, assign68230_e104884_d_n14,) = {
    if (((locals.var_guard1607 != 0.0) && (locals.var_guard1608 != 0.0)) && (locals.var_guard1610 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign68230_e104884;
        locals.var_t3_dn0 = assign68230_e104884_d_n0;
        locals.var_t3_dn2 = assign68230_e104884_d_n2;
        locals.var_t3_dn4 = assign68230_e104884_d_n4;
        locals.var_t3_dn5 = assign68230_e104884_d_n5;
        locals.var_t3_dn6 = assign68230_e104884_d_n6;
        locals.var_t3_dn7 = assign68230_e104884_d_n7;
        locals.var_t3_dn8 = assign68230_e104884_d_n8;
        locals.var_t3_dn9 = assign68230_e104884_d_n9;
        locals.var_t3_dn10 = assign68230_e104884_d_n10;
        locals.var_t3_dn11 = assign68230_e104884_d_n11;
        locals.var_t3_dn14 = assign68230_e104884_d_n14;

        let (assign68240_e104892, assign68240_e104892_d_n0, assign68240_e104892_d_n2, assign68240_e104892_d_n4, assign68240_e104892_d_n5, assign68240_e104892_d_n6, assign68240_e104892_d_n7, assign68240_e104892_d_n8, assign68240_e104892_d_n9, assign68240_e104892_d_n10, assign68240_e104892_d_n11, assign68240_e104892_d_n14,) = {
    if (((locals.var_guard1607 != 0.0) && (locals.var_guard1608 != 0.0)) && (locals.var_guard1610 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign68240_e104892;
        locals.var_t4_dn0 = assign68240_e104892_d_n0;
        locals.var_t4_dn2 = assign68240_e104892_d_n2;
        locals.var_t4_dn4 = assign68240_e104892_d_n4;
        locals.var_t4_dn5 = assign68240_e104892_d_n5;
        locals.var_t4_dn6 = assign68240_e104892_d_n6;
        locals.var_t4_dn7 = assign68240_e104892_d_n7;
        locals.var_t4_dn8 = assign68240_e104892_d_n8;
        locals.var_t4_dn9 = assign68240_e104892_d_n9;
        locals.var_t4_dn10 = assign68240_e104892_d_n10;
        locals.var_t4_dn11 = assign68240_e104892_d_n11;
        locals.var_t4_dn14 = assign68240_e104892_d_n14;

        let (assign68250_e104900, assign68250_e104900_d_n0, assign68250_e104900_d_n2, assign68250_e104900_d_n4, assign68250_e104900_d_n5, assign68250_e104900_d_n6, assign68250_e104900_d_n7, assign68250_e104900_d_n8, assign68250_e104900_d_n9, assign68250_e104900_d_n10, assign68250_e104900_d_n11, assign68250_e104900_d_n14,) = {
    if ((locals.var_guard1607 != 0.0) && (locals.var_guard1608 != 0.0)) {
        let assign68250_e104898: f64 = (locals.var_t3 - p.p262);
        (assign68250_e104898, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign68250_e104900;
        locals.var_t3_dn0 = assign68250_e104900_d_n0;
        locals.var_t3_dn2 = assign68250_e104900_d_n2;
        locals.var_t3_dn4 = assign68250_e104900_d_n4;
        locals.var_t3_dn5 = assign68250_e104900_d_n5;
        locals.var_t3_dn6 = assign68250_e104900_d_n6;
        locals.var_t3_dn7 = assign68250_e104900_d_n7;
        locals.var_t3_dn8 = assign68250_e104900_d_n8;
        locals.var_t3_dn9 = assign68250_e104900_d_n9;
        locals.var_t3_dn10 = assign68250_e104900_d_n10;
        locals.var_t3_dn11 = assign68250_e104900_d_n11;
        locals.var_t3_dn14 = assign68250_e104900_d_n14;

        let (assign68260_e104908, assign68260_e104908_d_n0, assign68260_e104908_d_n2, assign68260_e104908_d_n4, assign68260_e104908_d_n5, assign68260_e104908_d_n6, assign68260_e104908_d_n7, assign68260_e104908_d_n8, assign68260_e104908_d_n9, assign68260_e104908_d_n10, assign68260_e104908_d_n11, assign68260_e104908_d_n14,) = {
    if ((locals.var_guard1607 != 0.0) && (locals.var_guard1608 != 0.0)) {
        let assign68260_e104906: f64 = (locals.var_t3 / 0.1);
        (assign68260_e104906, (locals.var_t3_dn0 / 0.1), (locals.var_t3_dn2 / 0.1), (locals.var_t3_dn4 / 0.1), (locals.var_t3_dn5 / 0.1), (locals.var_t3_dn6 / 0.1), (locals.var_t3_dn7 / 0.1), (locals.var_t3_dn8 / 0.1), (locals.var_t3_dn9 / 0.1), (locals.var_t3_dn10 / 0.1), (locals.var_t3_dn11 / 0.1), (locals.var_t3_dn14 / 0.1),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign68260_e104908;
        locals.var_tx_dn0 = assign68260_e104908_d_n0;
        locals.var_tx_dn2 = assign68260_e104908_d_n2;
        locals.var_tx_dn4 = assign68260_e104908_d_n4;
        locals.var_tx_dn5 = assign68260_e104908_d_n5;
        locals.var_tx_dn6 = assign68260_e104908_d_n6;
        locals.var_tx_dn7 = assign68260_e104908_d_n7;
        locals.var_tx_dn8 = assign68260_e104908_d_n8;
        locals.var_tx_dn9 = assign68260_e104908_d_n9;
        locals.var_tx_dn10 = assign68260_e104908_d_n10;
        locals.var_tx_dn11 = assign68260_e104908_d_n11;
        locals.var_tx_dn14 = assign68260_e104908_d_n14;

        let (assign68270_e104918, assign68270_e104918_d_n0, assign68270_e104918_d_n2, assign68270_e104918_d_n4, assign68270_e104918_d_n5, assign68270_e104918_d_n6, assign68270_e104918_d_n7, assign68270_e104918_d_n8, assign68270_e104918_d_n9, assign68270_e104918_d_n10, assign68270_e104918_d_n11, assign68270_e104918_d_n14,) = {
    if ((locals.var_guard1607 != 0.0) && (locals.var_guard1608 != 0.0)) {
        let assign68270_e104915: f64 = (locals.var_tx * locals.var_tx);
        let assign68270_e104916: f64 = (1.0 + assign68270_e104915);
        (assign68270_e104916, ((locals.var_tx_dn0 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn0)), ((locals.var_tx_dn2 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn2)), ((locals.var_tx_dn4 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn4)), ((locals.var_tx_dn5 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn5)), ((locals.var_tx_dn6 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn6)), ((locals.var_tx_dn7 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn7)), ((locals.var_tx_dn8 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn8)), ((locals.var_tx_dn9 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn9)), ((locals.var_tx_dn10 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn10)), ((locals.var_tx_dn11 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn11)), ((locals.var_tx_dn14 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign68270_e104918;
        locals.var_t2_dn0 = assign68270_e104918_d_n0;
        locals.var_t2_dn2 = assign68270_e104918_d_n2;
        locals.var_t2_dn4 = assign68270_e104918_d_n4;
        locals.var_t2_dn5 = assign68270_e104918_d_n5;
        locals.var_t2_dn6 = assign68270_e104918_d_n6;
        locals.var_t2_dn7 = assign68270_e104918_d_n7;
        locals.var_t2_dn8 = assign68270_e104918_d_n8;
        locals.var_t2_dn9 = assign68270_e104918_d_n9;
        locals.var_t2_dn10 = assign68270_e104918_d_n10;
        locals.var_t2_dn11 = assign68270_e104918_d_n11;
        locals.var_t2_dn14 = assign68270_e104918_d_n14;

        let (assign68280_e104928, assign68280_e104928_d_n0, assign68280_e104928_d_n2, assign68280_e104928_d_n4, assign68280_e104928_d_n5, assign68280_e104928_d_n6, assign68280_e104928_d_n7, assign68280_e104928_d_n8, assign68280_e104928_d_n9, assign68280_e104928_d_n10, assign68280_e104928_d_n11, assign68280_e104928_d_n14,) = {
    if ((locals.var_guard1607 != 0.0) && (locals.var_guard1608 != 0.0)) {
        let assign68280_e104925: f64 = (1.0 / locals.var_t2);
        let assign68280_e104926: f64 = (1.0 - assign68280_e104925);
        (assign68280_e104926, (-(-(locals.var_t2_dn0 / (locals.var_t2 * locals.var_t2)))), (-(-(locals.var_t2_dn2 / (locals.var_t2 * locals.var_t2)))), (-(-(locals.var_t2_dn4 / (locals.var_t2 * locals.var_t2)))), (-(-(locals.var_t2_dn5 / (locals.var_t2 * locals.var_t2)))), (-(-(locals.var_t2_dn6 / (locals.var_t2 * locals.var_t2)))), (-(-(locals.var_t2_dn7 / (locals.var_t2 * locals.var_t2)))), (-(-(locals.var_t2_dn8 / (locals.var_t2 * locals.var_t2)))), (-(-(locals.var_t2_dn9 / (locals.var_t2 * locals.var_t2)))), (-(-(locals.var_t2_dn10 / (locals.var_t2 * locals.var_t2)))), (-(-(locals.var_t2_dn11 / (locals.var_t2 * locals.var_t2)))), (-(-(locals.var_t2_dn14 / (locals.var_t2 * locals.var_t2)))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign68280_e104928;
        locals.var_t1_dn0 = assign68280_e104928_d_n0;
        locals.var_t1_dn2 = assign68280_e104928_d_n2;
        locals.var_t1_dn4 = assign68280_e104928_d_n4;
        locals.var_t1_dn5 = assign68280_e104928_d_n5;
        locals.var_t1_dn6 = assign68280_e104928_d_n6;
        locals.var_t1_dn7 = assign68280_e104928_d_n7;
        locals.var_t1_dn8 = assign68280_e104928_d_n8;
        locals.var_t1_dn9 = assign68280_e104928_d_n9;
        locals.var_t1_dn10 = assign68280_e104928_d_n10;
        locals.var_t1_dn11 = assign68280_e104928_d_n11;
        locals.var_t1_dn14 = assign68280_e104928_d_n14;

        let (assign68290_e104936, assign68290_e104936_d_n0, assign68290_e104936_d_n2, assign68290_e104936_d_n4, assign68290_e104936_d_n5, assign68290_e104936_d_n6, assign68290_e104936_d_n7, assign68290_e104936_d_n8, assign68290_e104936_d_n9, assign68290_e104936_d_n10, assign68290_e104936_d_n11, assign68290_e104936_d_n14,) = {
    if ((locals.var_guard1607 != 0.0) && (locals.var_guard1608 != 0.0)) {
        let assign68290_e104934: f64 = (locals.var_etun * locals.var_t1);
        (assign68290_e104934, ((locals.var_etun_dn0 * locals.var_t1) + (locals.var_etun * locals.var_t1_dn0)), ((locals.var_etun_dn2 * locals.var_t1) + (locals.var_etun * locals.var_t1_dn2)), ((locals.var_etun_dn4 * locals.var_t1) + (locals.var_etun * locals.var_t1_dn4)), ((locals.var_etun_dn5 * locals.var_t1) + (locals.var_etun * locals.var_t1_dn5)), ((locals.var_etun_dn6 * locals.var_t1) + (locals.var_etun * locals.var_t1_dn6)), ((locals.var_etun_dn7 * locals.var_t1) + (locals.var_etun * locals.var_t1_dn7)), ((locals.var_etun_dn8 * locals.var_t1) + (locals.var_etun * locals.var_t1_dn8)), ((locals.var_etun_dn9 * locals.var_t1) + (locals.var_etun * locals.var_t1_dn9)), ((locals.var_etun_dn10 * locals.var_t1) + (locals.var_etun * locals.var_t1_dn10)), ((locals.var_etun_dn11 * locals.var_t1) + (locals.var_etun * locals.var_t1_dn11)), ((locals.var_etun_dn14 * locals.var_t1) + (locals.var_etun * locals.var_t1_dn14)),)
    } else {
        (locals.var_etun, locals.var_etun_dn0, locals.var_etun_dn2, locals.var_etun_dn4, locals.var_etun_dn5, locals.var_etun_dn6, locals.var_etun_dn7, locals.var_etun_dn8, locals.var_etun_dn9, locals.var_etun_dn10, locals.var_etun_dn11, locals.var_etun_dn14,)
    }
};
        locals.var_etun = assign68290_e104936;
        locals.var_etun_dn0 = assign68290_e104936_d_n0;
        locals.var_etun_dn2 = assign68290_e104936_d_n2;
        locals.var_etun_dn4 = assign68290_e104936_d_n4;
        locals.var_etun_dn5 = assign68290_e104936_d_n5;
        locals.var_etun_dn6 = assign68290_e104936_d_n6;
        locals.var_etun_dn7 = assign68290_e104936_d_n7;
        locals.var_etun_dn8 = assign68290_e104936_d_n8;
        locals.var_etun_dn9 = assign68290_e104936_d_n9;
        locals.var_etun_dn10 = assign68290_e104936_d_n10;
        locals.var_etun_dn11 = assign68290_e104936_d_n11;
        locals.var_etun_dn14 = assign68290_e104936_d_n14;

        let (assign68300_e104944, assign68300_e104944_d_n0, assign68300_e104944_d_n2, assign68300_e104944_d_n4, assign68300_e104944_d_n5, assign68300_e104944_d_n6, assign68300_e104944_d_n7, assign68300_e104944_d_n8, assign68300_e104944_d_n9, assign68300_e104944_d_n10, assign68300_e104944_d_n11, assign68300_e104944_d_n14,) = {
    if ((locals.var_guard1607 != 0.0) && (locals.var_guard1608 != 0.0)) {
        let assign68300_e104942: f64 = (locals.var_leff * locals.var_weff_nf);
        (assign68300_e104942, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign68300_e104944;
        locals.var_t0_dn0 = assign68300_e104944_d_n0;
        locals.var_t0_dn2 = assign68300_e104944_d_n2;
        locals.var_t0_dn4 = assign68300_e104944_d_n4;
        locals.var_t0_dn5 = assign68300_e104944_d_n5;
        locals.var_t0_dn6 = assign68300_e104944_d_n6;
        locals.var_t0_dn7 = assign68300_e104944_d_n7;
        locals.var_t0_dn8 = assign68300_e104944_d_n8;
        locals.var_t0_dn9 = assign68300_e104944_d_n9;
        locals.var_t0_dn10 = assign68300_e104944_d_n10;
        locals.var_t0_dn11 = assign68300_e104944_d_n11;
        locals.var_t0_dn14 = assign68300_e104944_d_n14;

        let (assign68310_e104954, assign68310_e104954_d_n0, assign68310_e104954_d_n2, assign68310_e104954_d_n4, assign68310_e104954_d_n5, assign68310_e104954_d_n6, assign68310_e104954_d_n7, assign68310_e104954_d_n8, assign68310_e104954_d_n9, assign68310_e104954_d_n10, assign68310_e104954_d_n11, assign68310_e104954_d_n14,) = {
    if ((locals.var_guard1607 != 0.0) && (locals.var_guard1608 != 0.0)) {
        let assign68310_e104951: f64 = (locals.var_mks_gleak7 + locals.var_t0);
        let assign68310_e104952: f64 = (locals.var_mks_gleak7 / assign68310_e104951);
        (assign68310_e104952, (-((locals.var_mks_gleak7 * locals.var_t0_dn0) / (assign68310_e104951 * assign68310_e104951))), (-((locals.var_mks_gleak7 * locals.var_t0_dn2) / (assign68310_e104951 * assign68310_e104951))), (-((locals.var_mks_gleak7 * locals.var_t0_dn4) / (assign68310_e104951 * assign68310_e104951))), (-((locals.var_mks_gleak7 * locals.var_t0_dn5) / (assign68310_e104951 * assign68310_e104951))), (-((locals.var_mks_gleak7 * locals.var_t0_dn6) / (assign68310_e104951 * assign68310_e104951))), (-((locals.var_mks_gleak7 * locals.var_t0_dn7) / (assign68310_e104951 * assign68310_e104951))), (-((locals.var_mks_gleak7 * locals.var_t0_dn8) / (assign68310_e104951 * assign68310_e104951))), (-((locals.var_mks_gleak7 * locals.var_t0_dn9) / (assign68310_e104951 * assign68310_e104951))), (-((locals.var_mks_gleak7 * locals.var_t0_dn10) / (assign68310_e104951 * assign68310_e104951))), (-((locals.var_mks_gleak7 * locals.var_t0_dn11) / (assign68310_e104951 * assign68310_e104951))), (-((locals.var_mks_gleak7 * locals.var_t0_dn14) / (assign68310_e104951 * assign68310_e104951))),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign68310_e104954;
        locals.var_t7_dn0 = assign68310_e104954_d_n0;
        locals.var_t7_dn2 = assign68310_e104954_d_n2;
        locals.var_t7_dn4 = assign68310_e104954_d_n4;
        locals.var_t7_dn5 = assign68310_e104954_d_n5;
        locals.var_t7_dn6 = assign68310_e104954_d_n6;
        locals.var_t7_dn7 = assign68310_e104954_d_n7;
        locals.var_t7_dn8 = assign68310_e104954_d_n8;
        locals.var_t7_dn9 = assign68310_e104954_d_n9;
        locals.var_t7_dn10 = assign68310_e104954_d_n10;
        locals.var_t7_dn11 = assign68310_e104954_d_n11;
        locals.var_t7_dn14 = assign68310_e104954_d_n14;

        let (assign68320_e104960, assign68320_e104960_d_n0, assign68320_e104960_d_n2, assign68320_e104960_d_n4, assign68320_e104960_d_n5, assign68320_e104960_d_n6, assign68320_e104960_d_n7, assign68320_e104960_d_n8, assign68320_e104960_d_n9, assign68320_e104960_d_n10, assign68320_e104960_d_n11, assign68320_e104960_d_n14,) = {
    if ((locals.var_guard1607 != 0.0) && (locals.var_guard1608 != 0.0)) {
        (locals.var_uc_gleak6, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign68320_e104960;
        locals.var_t6_dn0 = assign68320_e104960_d_n0;
        locals.var_t6_dn2 = assign68320_e104960_d_n2;
        locals.var_t6_dn4 = assign68320_e104960_d_n4;
        locals.var_t6_dn5 = assign68320_e104960_d_n5;
        locals.var_t6_dn6 = assign68320_e104960_d_n6;
        locals.var_t6_dn7 = assign68320_e104960_d_n7;
        locals.var_t6_dn8 = assign68320_e104960_d_n8;
        locals.var_t6_dn9 = assign68320_e104960_d_n9;
        locals.var_t6_dn10 = assign68320_e104960_d_n10;
        locals.var_t6_dn11 = assign68320_e104960_d_n11;
        locals.var_t6_dn14 = assign68320_e104960_d_n14;

        let (assign68330_e104970, assign68330_e104970_d_n0, assign68330_e104970_d_n2, assign68330_e104970_d_n4, assign68330_e104970_d_n5, assign68330_e104970_d_n6, assign68330_e104970_d_n7, assign68330_e104970_d_n8, assign68330_e104970_d_n9, assign68330_e104970_d_n10, assign68330_e104970_d_n11, assign68330_e104970_d_n14,) = {
    if ((locals.var_guard1607 != 0.0) && (locals.var_guard1608 != 0.0)) {
        let assign68330_e104967: f64 = (locals.var_t6 + locals.var_vdsz__blk443);
        let assign68330_e104968: f64 = (locals.var_t6 / assign68330_e104967);
        (assign68330_e104968, (((locals.var_t6_dn0 * assign68330_e104967) - (locals.var_t6 * (locals.var_t6_dn0 + locals.var_vdsz__blk443_dn0))) / (assign68330_e104967 * assign68330_e104967)), (((locals.var_t6_dn2 * assign68330_e104967) - (locals.var_t6 * (locals.var_t6_dn2 + locals.var_vdsz__blk443_dn2))) / (assign68330_e104967 * assign68330_e104967)), (((locals.var_t6_dn4 * assign68330_e104967) - (locals.var_t6 * (locals.var_t6_dn4 + locals.var_vdsz__blk443_dn4))) / (assign68330_e104967 * assign68330_e104967)), (((locals.var_t6_dn5 * assign68330_e104967) - (locals.var_t6 * (locals.var_t6_dn5 + locals.var_vdsz__blk443_dn5))) / (assign68330_e104967 * assign68330_e104967)), (((locals.var_t6_dn6 * assign68330_e104967) - (locals.var_t6 * (locals.var_t6_dn6 + locals.var_vdsz__blk443_dn6))) / (assign68330_e104967 * assign68330_e104967)), (((locals.var_t6_dn7 * assign68330_e104967) - (locals.var_t6 * (locals.var_t6_dn7 + locals.var_vdsz__blk443_dn7))) / (assign68330_e104967 * assign68330_e104967)), (((locals.var_t6_dn8 * assign68330_e104967) - (locals.var_t6 * (locals.var_t6_dn8 + locals.var_vdsz__blk443_dn8))) / (assign68330_e104967 * assign68330_e104967)), (((locals.var_t6_dn9 * assign68330_e104967) - (locals.var_t6 * (locals.var_t6_dn9 + locals.var_vdsz__blk443_dn9))) / (assign68330_e104967 * assign68330_e104967)), (((locals.var_t6_dn10 * assign68330_e104967) - (locals.var_t6 * (locals.var_t6_dn10 + locals.var_vdsz__blk443_dn10))) / (assign68330_e104967 * assign68330_e104967)), (((locals.var_t6_dn11 * assign68330_e104967) - (locals.var_t6 * (locals.var_t6_dn11 + locals.var_vdsz__blk443_dn11))) / (assign68330_e104967 * assign68330_e104967)), (((locals.var_t6_dn14 * assign68330_e104967) - (locals.var_t6 * (locals.var_t6_dn14 + locals.var_vdsz__blk443_dn14))) / (assign68330_e104967 * assign68330_e104967)),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign68330_e104970;
        locals.var_t9_dn0 = assign68330_e104970_d_n0;
        locals.var_t9_dn2 = assign68330_e104970_d_n2;
        locals.var_t9_dn4 = assign68330_e104970_d_n4;
        locals.var_t9_dn5 = assign68330_e104970_d_n5;
        locals.var_t9_dn6 = assign68330_e104970_d_n6;
        locals.var_t9_dn7 = assign68330_e104970_d_n7;
        locals.var_t9_dn8 = assign68330_e104970_d_n8;
        locals.var_t9_dn9 = assign68330_e104970_d_n9;
        locals.var_t9_dn10 = assign68330_e104970_d_n10;
        locals.var_t9_dn11 = assign68330_e104970_d_n11;
        locals.var_t9_dn14 = assign68330_e104970_d_n14;

        let (assign68340_e104980, assign68340_e104980_d_n0, assign68340_e104980_d_n2, assign68340_e104980_d_n4, assign68340_e104980_d_n5, assign68340_e104980_d_n6, assign68340_e104980_d_n7, assign68340_e104980_d_n8, assign68340_e104980_d_n9, assign68340_e104980_d_n10, assign68340_e104980_d_n11, assign68340_e104980_d_n14,) = {
    if ((locals.var_guard1607 != 0.0) && (locals.var_guard1608 != 0.0)) {
        let assign68340_e104977: f64 = (locals.var_etun + 1e-25);
        let assign68340_e104978: f64 = (1.0 / assign68340_e104977);
        (assign68340_e104978, (-(locals.var_etun_dn0 / (assign68340_e104977 * assign68340_e104977))), (-(locals.var_etun_dn2 / (assign68340_e104977 * assign68340_e104977))), (-(locals.var_etun_dn4 / (assign68340_e104977 * assign68340_e104977))), (-(locals.var_etun_dn5 / (assign68340_e104977 * assign68340_e104977))), (-(locals.var_etun_dn6 / (assign68340_e104977 * assign68340_e104977))), (-(locals.var_etun_dn7 / (assign68340_e104977 * assign68340_e104977))), (-(locals.var_etun_dn8 / (assign68340_e104977 * assign68340_e104977))), (-(locals.var_etun_dn9 / (assign68340_e104977 * assign68340_e104977))), (-(locals.var_etun_dn10 / (assign68340_e104977 * assign68340_e104977))), (-(locals.var_etun_dn11 / (assign68340_e104977 * assign68340_e104977))), (-(locals.var_etun_dn14 / (assign68340_e104977 * assign68340_e104977))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign68340_e104980;
        locals.var_t4_dn0 = assign68340_e104980_d_n0;
        locals.var_t4_dn2 = assign68340_e104980_d_n2;
        locals.var_t4_dn4 = assign68340_e104980_d_n4;
        locals.var_t4_dn5 = assign68340_e104980_d_n5;
        locals.var_t4_dn6 = assign68340_e104980_d_n6;
        locals.var_t4_dn7 = assign68340_e104980_d_n7;
        locals.var_t4_dn8 = assign68340_e104980_d_n8;
        locals.var_t4_dn9 = assign68340_e104980_d_n9;
        locals.var_t4_dn10 = assign68340_e104980_d_n10;
        locals.var_t4_dn11 = assign68340_e104980_d_n11;
        locals.var_t4_dn14 = assign68340_e104980_d_n14;

        let (assign68350_e104991, assign68350_e104991_d_n0, assign68350_e104991_d_n2, assign68350_e104991_d_n4, assign68350_e104991_d_n5, assign68350_e104991_d_n6, assign68350_e104991_d_n7, assign68350_e104991_d_n8, assign68350_e104991_d_n9, assign68350_e104991_d_n10, assign68350_e104991_d_n11, assign68350_e104991_d_n14,) = {
    if ((locals.var_guard1607 != 0.0) && (locals.var_guard1608 != 0.0)) {
        let assign68350_e104985: f64 = (-locals.var_uc_gleak2);
        let assign68350_e104987: f64 = (assign68350_e104985 * locals.var_egp32);
        let assign68350_e104989: f64 = (assign68350_e104987 * locals.var_t4);
        (assign68350_e104989, (((assign68350_e104985 * locals.var_egp32_dn0) * locals.var_t4) + (assign68350_e104987 * locals.var_t4_dn0)), (((assign68350_e104985 * locals.var_egp32_dn2) * locals.var_t4) + (assign68350_e104987 * locals.var_t4_dn2)), (((assign68350_e104985 * locals.var_egp32_dn4) * locals.var_t4) + (assign68350_e104987 * locals.var_t4_dn4)), (((assign68350_e104985 * locals.var_egp32_dn5) * locals.var_t4) + (assign68350_e104987 * locals.var_t4_dn5)), (((assign68350_e104985 * locals.var_egp32_dn6) * locals.var_t4) + (assign68350_e104987 * locals.var_t4_dn6)), (((assign68350_e104985 * locals.var_egp32_dn7) * locals.var_t4) + (assign68350_e104987 * locals.var_t4_dn7)), (((assign68350_e104985 * locals.var_egp32_dn8) * locals.var_t4) + (assign68350_e104987 * locals.var_t4_dn8)), (((assign68350_e104985 * locals.var_egp32_dn9) * locals.var_t4) + (assign68350_e104987 * locals.var_t4_dn9)), (((assign68350_e104985 * locals.var_egp32_dn10) * locals.var_t4) + (assign68350_e104987 * locals.var_t4_dn10)), (((assign68350_e104985 * locals.var_egp32_dn11) * locals.var_t4) + (assign68350_e104987 * locals.var_t4_dn11)), (((assign68350_e104985 * locals.var_egp32_dn14) * locals.var_t4) + (assign68350_e104987 * locals.var_t4_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign68350_e104991;
        locals.var_t1_dn0 = assign68350_e104991_d_n0;
        locals.var_t1_dn2 = assign68350_e104991_d_n2;
        locals.var_t1_dn4 = assign68350_e104991_d_n4;
        locals.var_t1_dn5 = assign68350_e104991_d_n5;
        locals.var_t1_dn6 = assign68350_e104991_d_n6;
        locals.var_t1_dn7 = assign68350_e104991_d_n7;
        locals.var_t1_dn8 = assign68350_e104991_d_n8;
        locals.var_t1_dn9 = assign68350_e104991_d_n9;
        locals.var_t1_dn10 = assign68350_e104991_d_n10;
        locals.var_t1_dn11 = assign68350_e104991_d_n11;
        locals.var_t1_dn14 = assign68350_e104991_d_n14;

        let (assign68360_e105001, assign68360_e105001_d_n0, assign68360_e105001_d_n2, assign68360_e105001_d_n4, assign68360_e105001_d_n5, assign68360_e105001_d_n6, assign68360_e105001_d_n7, assign68360_e105001_d_n8, assign68360_e105001_d_n9, assign68360_e105001_d_n10, assign68360_e105001_d_n11, assign68360_e105001_d_n14,) = {
    if ((locals.var_guard1607 != 0.0) && (locals.var_guard1608 != 0.0)) {
        let assign68360_e104997: f64 = (locals.var_uc_gleak2 * locals.var_t4);
        let assign68360_e104999: f64 = (assign68360_e104997 * locals.var_t4);
        (assign68360_e104999, (((locals.var_uc_gleak2 * locals.var_t4_dn0) * locals.var_t4) + (assign68360_e104997 * locals.var_t4_dn0)), (((locals.var_uc_gleak2 * locals.var_t4_dn2) * locals.var_t4) + (assign68360_e104997 * locals.var_t4_dn2)), (((locals.var_uc_gleak2 * locals.var_t4_dn4) * locals.var_t4) + (assign68360_e104997 * locals.var_t4_dn4)), (((locals.var_uc_gleak2 * locals.var_t4_dn5) * locals.var_t4) + (assign68360_e104997 * locals.var_t4_dn5)), (((locals.var_uc_gleak2 * locals.var_t4_dn6) * locals.var_t4) + (assign68360_e104997 * locals.var_t4_dn6)), (((locals.var_uc_gleak2 * locals.var_t4_dn7) * locals.var_t4) + (assign68360_e104997 * locals.var_t4_dn7)), (((locals.var_uc_gleak2 * locals.var_t4_dn8) * locals.var_t4) + (assign68360_e104997 * locals.var_t4_dn8)), (((locals.var_uc_gleak2 * locals.var_t4_dn9) * locals.var_t4) + (assign68360_e104997 * locals.var_t4_dn9)), (((locals.var_uc_gleak2 * locals.var_t4_dn10) * locals.var_t4) + (assign68360_e104997 * locals.var_t4_dn10)), (((locals.var_uc_gleak2 * locals.var_t4_dn11) * locals.var_t4) + (assign68360_e104997 * locals.var_t4_dn11)), (((locals.var_uc_gleak2 * locals.var_t4_dn14) * locals.var_t4) + (assign68360_e104997 * locals.var_t4_dn14)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign68360_e105001;
        locals.var_t3_dn0 = assign68360_e105001_d_n0;
        locals.var_t3_dn2 = assign68360_e105001_d_n2;
        locals.var_t3_dn4 = assign68360_e105001_d_n4;
        locals.var_t3_dn5 = assign68360_e105001_d_n5;
        locals.var_t3_dn6 = assign68360_e105001_d_n6;
        locals.var_t3_dn7 = assign68360_e105001_d_n7;
        locals.var_t3_dn8 = assign68360_e105001_d_n8;
        locals.var_t3_dn9 = assign68360_e105001_d_n9;
        locals.var_t3_dn10 = assign68360_e105001_d_n10;
        locals.var_t3_dn11 = assign68360_e105001_d_n11;
        locals.var_t3_dn14 = assign68360_e105001_d_n14;

        let assign68370_e105004: f64 = (-34.0);
        let assign68370_e105005: f64 = if locals.var_t1 < assign68370_e105004 { 1.0 } else { 0.0 };
        locals.var_guard1611 = assign68370_e105005;

    }

    pub(super) fn stamp_transient_block_244(
        locals: &mut StampLocals,
    ) {
        let (assign68380_e105013, assign68380_e105013_d_n0, assign68380_e105013_d_n2, assign68380_e105013_d_n4, assign68380_e105013_d_n5, assign68380_e105013_d_n6, assign68380_e105013_d_n7, assign68380_e105013_d_n8, assign68380_e105013_d_n9, assign68380_e105013_d_n10, assign68380_e105013_d_n11, assign68380_e105013_d_n14,) = {
    if (((locals.var_guard1607 != 0.0) && (locals.var_guard1608 != 0.0)) && (locals.var_guard1611 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_igate, locals.var_igate_dn0, locals.var_igate_dn2, locals.var_igate_dn4, locals.var_igate_dn5, locals.var_igate_dn6, locals.var_igate_dn7, locals.var_igate_dn8, locals.var_igate_dn9, locals.var_igate_dn10, locals.var_igate_dn11, locals.var_igate_dn14,)
    }
};
        locals.var_igate = assign68380_e105013;
        locals.var_igate_dn0 = assign68380_e105013_d_n0;
        locals.var_igate_dn2 = assign68380_e105013_d_n2;
        locals.var_igate_dn4 = assign68380_e105013_d_n4;
        locals.var_igate_dn5 = assign68380_e105013_d_n5;
        locals.var_igate_dn6 = assign68380_e105013_d_n6;
        locals.var_igate_dn7 = assign68380_e105013_d_n7;
        locals.var_igate_dn8 = assign68380_e105013_d_n8;
        locals.var_igate_dn9 = assign68380_e105013_d_n9;
        locals.var_igate_dn10 = assign68380_e105013_d_n10;
        locals.var_igate_dn11 = assign68380_e105013_d_n11;
        locals.var_igate_dn14 = assign68380_e105013_d_n14;

        let (assign68390_e105023, assign68390_e105023_d_n0, assign68390_e105023_d_n2, assign68390_e105023_d_n4, assign68390_e105023_d_n5, assign68390_e105023_d_n6, assign68390_e105023_d_n7, assign68390_e105023_d_n8, assign68390_e105023_d_n9, assign68390_e105023_d_n10, assign68390_e105023_d_n11, assign68390_e105023_d_n14,) = {
    if (((locals.var_guard1607 != 0.0) && (locals.var_guard1608 != 0.0)) && (locals.var_guard1611 == 0.0)) {
        let assign68390_e105021: f64 = (locals.var_t1).exp();
        (assign68390_e105021, (assign68390_e105021 * locals.var_t1_dn0), (assign68390_e105021 * locals.var_t1_dn2), (assign68390_e105021 * locals.var_t1_dn4), (assign68390_e105021 * locals.var_t1_dn5), (assign68390_e105021 * locals.var_t1_dn6), (assign68390_e105021 * locals.var_t1_dn7), (assign68390_e105021 * locals.var_t1_dn8), (assign68390_e105021 * locals.var_t1_dn9), (assign68390_e105021 * locals.var_t1_dn10), (assign68390_e105021 * locals.var_t1_dn11), (assign68390_e105021 * locals.var_t1_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign68390_e105023;
        locals.var_t2_dn0 = assign68390_e105023_d_n0;
        locals.var_t2_dn2 = assign68390_e105023_d_n2;
        locals.var_t2_dn4 = assign68390_e105023_d_n4;
        locals.var_t2_dn5 = assign68390_e105023_d_n5;
        locals.var_t2_dn6 = assign68390_e105023_d_n6;
        locals.var_t2_dn7 = assign68390_e105023_d_n7;
        locals.var_t2_dn8 = assign68390_e105023_d_n8;
        locals.var_t2_dn9 = assign68390_e105023_d_n9;
        locals.var_t2_dn10 = assign68390_e105023_d_n10;
        locals.var_t2_dn11 = assign68390_e105023_d_n11;
        locals.var_t2_dn14 = assign68390_e105023_d_n14;

        let (assign68400_e105038, assign68400_e105038_d_n0, assign68400_e105038_d_n2, assign68400_e105038_d_n4, assign68400_e105038_d_n5, assign68400_e105038_d_n6, assign68400_e105038_d_n7, assign68400_e105038_d_n8, assign68400_e105038_d_n9, assign68400_e105038_d_n10, assign68400_e105038_d_n11, assign68400_e105038_d_n14,) = {
    if (((locals.var_guard1607 != 0.0) && (locals.var_guard1608 != 0.0)) && (locals.var_guard1611 == 0.0)) {
        let assign68400_e105032: f64 = (locals.var_uc_gleak1 / locals.var_egp12);
        let assign68400_e105034: f64 = (assign68400_e105032 * 1.6021918e-19);
        let assign68400_e105036: f64 = (assign68400_e105034 * locals.var_t0);
        (assign68400_e105036, ((((-((locals.var_uc_gleak1 * locals.var_egp12_dn0) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_t0) + (assign68400_e105034 * locals.var_t0_dn0)), ((((-((locals.var_uc_gleak1 * locals.var_egp12_dn2) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_t0) + (assign68400_e105034 * locals.var_t0_dn2)), ((((-((locals.var_uc_gleak1 * locals.var_egp12_dn4) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_t0) + (assign68400_e105034 * locals.var_t0_dn4)), ((((-((locals.var_uc_gleak1 * locals.var_egp12_dn5) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_t0) + (assign68400_e105034 * locals.var_t0_dn5)), ((((-((locals.var_uc_gleak1 * locals.var_egp12_dn6) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_t0) + (assign68400_e105034 * locals.var_t0_dn6)), ((((-((locals.var_uc_gleak1 * locals.var_egp12_dn7) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_t0) + (assign68400_e105034 * locals.var_t0_dn7)), ((((-((locals.var_uc_gleak1 * locals.var_egp12_dn8) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_t0) + (assign68400_e105034 * locals.var_t0_dn8)), ((((-((locals.var_uc_gleak1 * locals.var_egp12_dn9) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_t0) + (assign68400_e105034 * locals.var_t0_dn9)), ((((-((locals.var_uc_gleak1 * locals.var_egp12_dn10) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_t0) + (assign68400_e105034 * locals.var_t0_dn10)), ((((-((locals.var_uc_gleak1 * locals.var_egp12_dn11) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_t0) + (assign68400_e105034 * locals.var_t0_dn11)), ((((-((locals.var_uc_gleak1 * locals.var_egp12_dn14) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_t0) + (assign68400_e105034 * locals.var_t0_dn14)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign68400_e105038;
        locals.var_t3_dn0 = assign68400_e105038_d_n0;
        locals.var_t3_dn2 = assign68400_e105038_d_n2;
        locals.var_t3_dn4 = assign68400_e105038_d_n4;
        locals.var_t3_dn5 = assign68400_e105038_d_n5;
        locals.var_t3_dn6 = assign68400_e105038_d_n6;
        locals.var_t3_dn7 = assign68400_e105038_d_n7;
        locals.var_t3_dn8 = assign68400_e105038_d_n8;
        locals.var_t3_dn9 = assign68400_e105038_d_n9;
        locals.var_t3_dn10 = assign68400_e105038_d_n10;
        locals.var_t3_dn11 = assign68400_e105038_d_n11;
        locals.var_t3_dn14 = assign68400_e105038_d_n14;

        let (assign68410_e105049, assign68410_e105049_d_n0, assign68410_e105049_d_n2, assign68410_e105049_d_n4, assign68410_e105049_d_n5, assign68410_e105049_d_n6, assign68410_e105049_d_n7, assign68410_e105049_d_n8, assign68410_e105049_d_n9, assign68410_e105049_d_n10, assign68410_e105049_d_n11, assign68410_e105049_d_n14,) = {
    if (((locals.var_guard1607 != 0.0) && (locals.var_guard1608 != 0.0)) && (locals.var_guard1611 == 0.0)) {
        let assign68410_e105047: f64 = (1.0 / locals.var_cnst0);
        (assign68410_e105047, (-(locals.var_cnst0_dn0 / (locals.var_cnst0 * locals.var_cnst0))), (-(locals.var_cnst0_dn2 / (locals.var_cnst0 * locals.var_cnst0))), (-(locals.var_cnst0_dn4 / (locals.var_cnst0 * locals.var_cnst0))), (-(locals.var_cnst0_dn5 / (locals.var_cnst0 * locals.var_cnst0))), (-(locals.var_cnst0_dn6 / (locals.var_cnst0 * locals.var_cnst0))), (-(locals.var_cnst0_dn7 / (locals.var_cnst0 * locals.var_cnst0))), (-(locals.var_cnst0_dn8 / (locals.var_cnst0 * locals.var_cnst0))), (-(locals.var_cnst0_dn9 / (locals.var_cnst0 * locals.var_cnst0))), (-(locals.var_cnst0_dn10 / (locals.var_cnst0 * locals.var_cnst0))), (-(locals.var_cnst0_dn11 / (locals.var_cnst0 * locals.var_cnst0))), (-(locals.var_cnst0_dn14 / (locals.var_cnst0 * locals.var_cnst0))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign68410_e105049;
        locals.var_t5_dn0 = assign68410_e105049_d_n0;
        locals.var_t5_dn2 = assign68410_e105049_d_n2;
        locals.var_t5_dn4 = assign68410_e105049_d_n4;
        locals.var_t5_dn5 = assign68410_e105049_d_n5;
        locals.var_t5_dn6 = assign68410_e105049_d_n6;
        locals.var_t5_dn7 = assign68410_e105049_d_n7;
        locals.var_t5_dn8 = assign68410_e105049_d_n8;
        locals.var_t5_dn9 = assign68410_e105049_d_n9;
        locals.var_t5_dn10 = assign68410_e105049_d_n10;
        locals.var_t5_dn11 = assign68410_e105049_d_n11;
        locals.var_t5_dn14 = assign68410_e105049_d_n14;

        let (assign68420_e105065, assign68420_e105065_d_n0, assign68420_e105065_d_n2, assign68420_e105065_d_n4, assign68420_e105065_d_n5, assign68420_e105065_d_n6, assign68420_e105065_d_n7, assign68420_e105065_d_n8, assign68420_e105065_d_n9, assign68420_e105065_d_n10, assign68420_e105065_d_n11, assign68420_e105065_d_n14,) = {
    if (((locals.var_guard1607 != 0.0) && (locals.var_guard1608 != 0.0)) && (locals.var_guard1611 == 0.0)) {
        let assign68420_e105059: f64 = (locals.var_cox0 * 1e-12);
        let assign68420_e105060: f64 = (locals.var_qiu_noi + assign68420_e105059);
        let assign68420_e105062: f64 = (assign68420_e105060 * locals.var_t5);
        let assign68420_e105063: f64 = (assign68420_e105062).sqrt();
        (assign68420_e105063, (((locals.var_qiu_noi_dn0 * locals.var_t5) + (assign68420_e105060 * locals.var_t5_dn0)) / (2.0 * assign68420_e105063)), (((locals.var_qiu_noi_dn2 * locals.var_t5) + (assign68420_e105060 * locals.var_t5_dn2)) / (2.0 * assign68420_e105063)), (((locals.var_qiu_noi_dn4 * locals.var_t5) + (assign68420_e105060 * locals.var_t5_dn4)) / (2.0 * assign68420_e105063)), (((locals.var_qiu_noi_dn5 * locals.var_t5) + (assign68420_e105060 * locals.var_t5_dn5)) / (2.0 * assign68420_e105063)), (((locals.var_qiu_noi_dn6 * locals.var_t5) + (assign68420_e105060 * locals.var_t5_dn6)) / (2.0 * assign68420_e105063)), (((locals.var_qiu_noi_dn7 * locals.var_t5) + (assign68420_e105060 * locals.var_t5_dn7)) / (2.0 * assign68420_e105063)), (((locals.var_qiu_noi_dn8 * locals.var_t5) + (assign68420_e105060 * locals.var_t5_dn8)) / (2.0 * assign68420_e105063)), (((locals.var_qiu_noi_dn9 * locals.var_t5) + (assign68420_e105060 * locals.var_t5_dn9)) / (2.0 * assign68420_e105063)), (((locals.var_qiu_noi_dn10 * locals.var_t5) + (assign68420_e105060 * locals.var_t5_dn10)) / (2.0 * assign68420_e105063)), (((locals.var_qiu_noi_dn11 * locals.var_t5) + (assign68420_e105060 * locals.var_t5_dn11)) / (2.0 * assign68420_e105063)), (((locals.var_qiu_noi_dn14 * locals.var_t5) + (assign68420_e105060 * locals.var_t5_dn14)) / (2.0 * assign68420_e105063)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign68420_e105065;
        locals.var_t6_dn0 = assign68420_e105065_d_n0;
        locals.var_t6_dn2 = assign68420_e105065_d_n2;
        locals.var_t6_dn4 = assign68420_e105065_d_n4;
        locals.var_t6_dn5 = assign68420_e105065_d_n5;
        locals.var_t6_dn6 = assign68420_e105065_d_n6;
        locals.var_t6_dn7 = assign68420_e105065_d_n7;
        locals.var_t6_dn8 = assign68420_e105065_d_n8;
        locals.var_t6_dn9 = assign68420_e105065_d_n9;
        locals.var_t6_dn10 = assign68420_e105065_d_n10;
        locals.var_t6_dn11 = assign68420_e105065_d_n11;
        locals.var_t6_dn14 = assign68420_e105065_d_n14;

        let (assign68430_e105078, assign68430_e105078_d_n0, assign68430_e105078_d_n2, assign68430_e105078_d_n4, assign68430_e105078_d_n5, assign68430_e105078_d_n6, assign68430_e105078_d_n7, assign68430_e105078_d_n8, assign68430_e105078_d_n9, assign68430_e105078_d_n10, assign68430_e105078_d_n11, assign68430_e105078_d_n14,) = {
    if (((locals.var_guard1607 != 0.0) && (locals.var_guard1608 != 0.0)) && (locals.var_guard1611 == 0.0)) {
        let assign68430_e105074: f64 = (locals.var_t2 * locals.var_t3);
        let assign68430_e105076: f64 = (assign68430_e105074 * locals.var_t6);
        (assign68430_e105076, ((((locals.var_t2_dn0 * locals.var_t3) + (locals.var_t2 * locals.var_t3_dn0)) * locals.var_t6) + (assign68430_e105074 * locals.var_t6_dn0)), ((((locals.var_t2_dn2 * locals.var_t3) + (locals.var_t2 * locals.var_t3_dn2)) * locals.var_t6) + (assign68430_e105074 * locals.var_t6_dn2)), ((((locals.var_t2_dn4 * locals.var_t3) + (locals.var_t2 * locals.var_t3_dn4)) * locals.var_t6) + (assign68430_e105074 * locals.var_t6_dn4)), ((((locals.var_t2_dn5 * locals.var_t3) + (locals.var_t2 * locals.var_t3_dn5)) * locals.var_t6) + (assign68430_e105074 * locals.var_t6_dn5)), ((((locals.var_t2_dn6 * locals.var_t3) + (locals.var_t2 * locals.var_t3_dn6)) * locals.var_t6) + (assign68430_e105074 * locals.var_t6_dn6)), ((((locals.var_t2_dn7 * locals.var_t3) + (locals.var_t2 * locals.var_t3_dn7)) * locals.var_t6) + (assign68430_e105074 * locals.var_t6_dn7)), ((((locals.var_t2_dn8 * locals.var_t3) + (locals.var_t2 * locals.var_t3_dn8)) * locals.var_t6) + (assign68430_e105074 * locals.var_t6_dn8)), ((((locals.var_t2_dn9 * locals.var_t3) + (locals.var_t2 * locals.var_t3_dn9)) * locals.var_t6) + (assign68430_e105074 * locals.var_t6_dn9)), ((((locals.var_t2_dn10 * locals.var_t3) + (locals.var_t2 * locals.var_t3_dn10)) * locals.var_t6) + (assign68430_e105074 * locals.var_t6_dn10)), ((((locals.var_t2_dn11 * locals.var_t3) + (locals.var_t2 * locals.var_t3_dn11)) * locals.var_t6) + (assign68430_e105074 * locals.var_t6_dn11)), ((((locals.var_t2_dn14 * locals.var_t3) + (locals.var_t2 * locals.var_t3_dn14)) * locals.var_t6) + (assign68430_e105074 * locals.var_t6_dn14)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign68430_e105078;
        locals.var_t4_dn0 = assign68430_e105078_d_n0;
        locals.var_t4_dn2 = assign68430_e105078_d_n2;
        locals.var_t4_dn4 = assign68430_e105078_d_n4;
        locals.var_t4_dn5 = assign68430_e105078_d_n5;
        locals.var_t4_dn6 = assign68430_e105078_d_n6;
        locals.var_t4_dn7 = assign68430_e105078_d_n7;
        locals.var_t4_dn8 = assign68430_e105078_d_n8;
        locals.var_t4_dn9 = assign68430_e105078_d_n9;
        locals.var_t4_dn10 = assign68430_e105078_d_n10;
        locals.var_t4_dn11 = assign68430_e105078_d_n11;
        locals.var_t4_dn14 = assign68430_e105078_d_n14;

        let (assign68440_e105089, assign68440_e105089_d_n0, assign68440_e105089_d_n2, assign68440_e105089_d_n4, assign68440_e105089_d_n5, assign68440_e105089_d_n6, assign68440_e105089_d_n7, assign68440_e105089_d_n8, assign68440_e105089_d_n9, assign68440_e105089_d_n10, assign68440_e105089_d_n11, assign68440_e105089_d_n14,) = {
    if (((locals.var_guard1607 != 0.0) && (locals.var_guard1608 != 0.0)) && (locals.var_guard1611 == 0.0)) {
        let assign68440_e105087: f64 = (locals.var_t4 * locals.var_etun);
        (assign68440_e105087, ((locals.var_t4_dn0 * locals.var_etun) + (locals.var_t4 * locals.var_etun_dn0)), ((locals.var_t4_dn2 * locals.var_etun) + (locals.var_t4 * locals.var_etun_dn2)), ((locals.var_t4_dn4 * locals.var_etun) + (locals.var_t4 * locals.var_etun_dn4)), ((locals.var_t4_dn5 * locals.var_etun) + (locals.var_t4 * locals.var_etun_dn5)), ((locals.var_t4_dn6 * locals.var_etun) + (locals.var_t4 * locals.var_etun_dn6)), ((locals.var_t4_dn7 * locals.var_etun) + (locals.var_t4 * locals.var_etun_dn7)), ((locals.var_t4_dn8 * locals.var_etun) + (locals.var_t4 * locals.var_etun_dn8)), ((locals.var_t4_dn9 * locals.var_etun) + (locals.var_t4 * locals.var_etun_dn9)), ((locals.var_t4_dn10 * locals.var_etun) + (locals.var_t4 * locals.var_etun_dn10)), ((locals.var_t4_dn11 * locals.var_etun) + (locals.var_t4 * locals.var_etun_dn11)), ((locals.var_t4_dn14 * locals.var_etun) + (locals.var_t4 * locals.var_etun_dn14)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign68440_e105089;
        locals.var_t5_dn0 = assign68440_e105089_d_n0;
        locals.var_t5_dn2 = assign68440_e105089_d_n2;
        locals.var_t5_dn4 = assign68440_e105089_d_n4;
        locals.var_t5_dn5 = assign68440_e105089_d_n5;
        locals.var_t5_dn6 = assign68440_e105089_d_n6;
        locals.var_t5_dn7 = assign68440_e105089_d_n7;
        locals.var_t5_dn8 = assign68440_e105089_d_n8;
        locals.var_t5_dn9 = assign68440_e105089_d_n9;
        locals.var_t5_dn10 = assign68440_e105089_d_n10;
        locals.var_t5_dn11 = assign68440_e105089_d_n11;
        locals.var_t5_dn14 = assign68440_e105089_d_n14;

        let (assign68450_e105100, assign68450_e105100_d_n0, assign68450_e105100_d_n2, assign68450_e105100_d_n4, assign68450_e105100_d_n5, assign68450_e105100_d_n6, assign68450_e105100_d_n7, assign68450_e105100_d_n8, assign68450_e105100_d_n9, assign68450_e105100_d_n10, assign68450_e105100_d_n11, assign68450_e105100_d_n14,) = {
    if (((locals.var_guard1607 != 0.0) && (locals.var_guard1608 != 0.0)) && (locals.var_guard1611 == 0.0)) {
        let assign68450_e105098: f64 = (locals.var_t5 * locals.var_etun);
        (assign68450_e105098, ((locals.var_t5_dn0 * locals.var_etun) + (locals.var_t5 * locals.var_etun_dn0)), ((locals.var_t5_dn2 * locals.var_etun) + (locals.var_t5 * locals.var_etun_dn2)), ((locals.var_t5_dn4 * locals.var_etun) + (locals.var_t5 * locals.var_etun_dn4)), ((locals.var_t5_dn5 * locals.var_etun) + (locals.var_t5 * locals.var_etun_dn5)), ((locals.var_t5_dn6 * locals.var_etun) + (locals.var_t5 * locals.var_etun_dn6)), ((locals.var_t5_dn7 * locals.var_etun) + (locals.var_t5 * locals.var_etun_dn7)), ((locals.var_t5_dn8 * locals.var_etun) + (locals.var_t5 * locals.var_etun_dn8)), ((locals.var_t5_dn9 * locals.var_etun) + (locals.var_t5 * locals.var_etun_dn9)), ((locals.var_t5_dn10 * locals.var_etun) + (locals.var_t5 * locals.var_etun_dn10)), ((locals.var_t5_dn11 * locals.var_etun) + (locals.var_t5 * locals.var_etun_dn11)), ((locals.var_t5_dn14 * locals.var_etun) + (locals.var_t5 * locals.var_etun_dn14)),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign68450_e105100;
        locals.var_t10_dn0 = assign68450_e105100_d_n0;
        locals.var_t10_dn2 = assign68450_e105100_d_n2;
        locals.var_t10_dn4 = assign68450_e105100_d_n4;
        locals.var_t10_dn5 = assign68450_e105100_d_n5;
        locals.var_t10_dn6 = assign68450_e105100_d_n6;
        locals.var_t10_dn7 = assign68450_e105100_d_n7;
        locals.var_t10_dn8 = assign68450_e105100_d_n8;
        locals.var_t10_dn9 = assign68450_e105100_d_n9;
        locals.var_t10_dn10 = assign68450_e105100_d_n10;
        locals.var_t10_dn11 = assign68450_e105100_d_n11;
        locals.var_t10_dn14 = assign68450_e105100_d_n14;

        let (assign68460_e105113, assign68460_e105113_d_n0, assign68460_e105113_d_n2, assign68460_e105113_d_n4, assign68460_e105113_d_n5, assign68460_e105113_d_n6, assign68460_e105113_d_n7, assign68460_e105113_d_n8, assign68460_e105113_d_n9, assign68460_e105113_d_n10, assign68460_e105113_d_n11, assign68460_e105113_d_n14,) = {
    if (((locals.var_guard1607 != 0.0) && (locals.var_guard1608 != 0.0)) && (locals.var_guard1611 == 0.0)) {
        let assign68460_e105109: f64 = (locals.var_t7 * locals.var_t9);
        let assign68460_e105111: f64 = (assign68460_e105109 * locals.var_t10);
        (assign68460_e105111, ((((locals.var_t7_dn0 * locals.var_t9) + (locals.var_t7 * locals.var_t9_dn0)) * locals.var_t10) + (assign68460_e105109 * locals.var_t10_dn0)), ((((locals.var_t7_dn2 * locals.var_t9) + (locals.var_t7 * locals.var_t9_dn2)) * locals.var_t10) + (assign68460_e105109 * locals.var_t10_dn2)), ((((locals.var_t7_dn4 * locals.var_t9) + (locals.var_t7 * locals.var_t9_dn4)) * locals.var_t10) + (assign68460_e105109 * locals.var_t10_dn4)), ((((locals.var_t7_dn5 * locals.var_t9) + (locals.var_t7 * locals.var_t9_dn5)) * locals.var_t10) + (assign68460_e105109 * locals.var_t10_dn5)), ((((locals.var_t7_dn6 * locals.var_t9) + (locals.var_t7 * locals.var_t9_dn6)) * locals.var_t10) + (assign68460_e105109 * locals.var_t10_dn6)), ((((locals.var_t7_dn7 * locals.var_t9) + (locals.var_t7 * locals.var_t9_dn7)) * locals.var_t10) + (assign68460_e105109 * locals.var_t10_dn7)), ((((locals.var_t7_dn8 * locals.var_t9) + (locals.var_t7 * locals.var_t9_dn8)) * locals.var_t10) + (assign68460_e105109 * locals.var_t10_dn8)), ((((locals.var_t7_dn9 * locals.var_t9) + (locals.var_t7 * locals.var_t9_dn9)) * locals.var_t10) + (assign68460_e105109 * locals.var_t10_dn9)), ((((locals.var_t7_dn10 * locals.var_t9) + (locals.var_t7 * locals.var_t9_dn10)) * locals.var_t10) + (assign68460_e105109 * locals.var_t10_dn10)), ((((locals.var_t7_dn11 * locals.var_t9) + (locals.var_t7 * locals.var_t9_dn11)) * locals.var_t10) + (assign68460_e105109 * locals.var_t10_dn11)), ((((locals.var_t7_dn14 * locals.var_t9) + (locals.var_t7 * locals.var_t9_dn14)) * locals.var_t10) + (assign68460_e105109 * locals.var_t10_dn14)),)
    } else {
        (locals.var_igate, locals.var_igate_dn0, locals.var_igate_dn2, locals.var_igate_dn4, locals.var_igate_dn5, locals.var_igate_dn6, locals.var_igate_dn7, locals.var_igate_dn8, locals.var_igate_dn9, locals.var_igate_dn10, locals.var_igate_dn11, locals.var_igate_dn14,)
    }
};
        locals.var_igate = assign68460_e105113;
        locals.var_igate_dn0 = assign68460_e105113_d_n0;
        locals.var_igate_dn2 = assign68460_e105113_d_n2;
        locals.var_igate_dn4 = assign68460_e105113_d_n4;
        locals.var_igate_dn5 = assign68460_e105113_d_n5;
        locals.var_igate_dn6 = assign68460_e105113_d_n6;
        locals.var_igate_dn7 = assign68460_e105113_d_n7;
        locals.var_igate_dn8 = assign68460_e105113_d_n8;
        locals.var_igate_dn9 = assign68460_e105113_d_n9;
        locals.var_igate_dn10 = assign68460_e105113_d_n10;
        locals.var_igate_dn11 = assign68460_e105113_d_n11;
        locals.var_igate_dn14 = assign68460_e105113_d_n14;

        let (assign68470_e105122, assign68470_e105122_d_n0, assign68470_e105122_d_n2, assign68470_e105122_d_n4, assign68470_e105122_d_n5, assign68470_e105122_d_n6, assign68470_e105122_d_n7, assign68470_e105122_d_n8, assign68470_e105122_d_n9, assign68470_e105122_d_n10, assign68470_e105122_d_n11, assign68470_e105122_d_n14,) = {
    if (locals.var_guard1607 != 0.0) {
        let assign68470_e105116: f64 = (-locals.var_uc_glksd2);
        let assign68470_e105118: f64 = (assign68470_e105116 * locals.var_vgs);
        let assign68470_e105120: f64 = (assign68470_e105118 + locals.var_mks_glksd3);
        (assign68470_e105120, 0.0, 0.0, 0.0, 0.0, (assign68470_e105116 * locals.var_vgs_dn6), (assign68470_e105116 * locals.var_vgs_dn7), (assign68470_e105116 * locals.var_vgs_dn8), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign68470_e105122;
        locals.var_t0_dn0 = assign68470_e105122_d_n0;
        locals.var_t0_dn2 = assign68470_e105122_d_n2;
        locals.var_t0_dn4 = assign68470_e105122_d_n4;
        locals.var_t0_dn5 = assign68470_e105122_d_n5;
        locals.var_t0_dn6 = assign68470_e105122_d_n6;
        locals.var_t0_dn7 = assign68470_e105122_d_n7;
        locals.var_t0_dn8 = assign68470_e105122_d_n8;
        locals.var_t0_dn9 = assign68470_e105122_d_n9;
        locals.var_t0_dn10 = assign68470_e105122_d_n10;
        locals.var_t0_dn11 = assign68470_e105122_d_n11;
        locals.var_t0_dn14 = assign68470_e105122_d_n14;

        let (assign68480_e105129, assign68480_e105129_d_n0, assign68480_e105129_d_n2, assign68480_e105129_d_n4, assign68480_e105129_d_n5, assign68480_e105129_d_n6, assign68480_e105129_d_n7, assign68480_e105129_d_n8, assign68480_e105129_d_n9, assign68480_e105129_d_n10, assign68480_e105129_d_n11, assign68480_e105129_d_n14,) = {
    if (locals.var_guard1607 != 0.0) {
        let assign68480_e105126: f64 = (locals.var_tox0 * locals.var_t0);
        let assign68480_e105127: f64 = (assign68480_e105126).exp();
        (assign68480_e105127, (assign68480_e105127 * (locals.var_tox0 * locals.var_t0_dn0)), (assign68480_e105127 * (locals.var_tox0 * locals.var_t0_dn2)), (assign68480_e105127 * (locals.var_tox0 * locals.var_t0_dn4)), (assign68480_e105127 * (locals.var_tox0 * locals.var_t0_dn5)), (assign68480_e105127 * (locals.var_tox0 * locals.var_t0_dn6)), (assign68480_e105127 * (locals.var_tox0 * locals.var_t0_dn7)), (assign68480_e105127 * (locals.var_tox0 * locals.var_t0_dn8)), (assign68480_e105127 * (locals.var_tox0 * locals.var_t0_dn9)), (assign68480_e105127 * (locals.var_tox0 * locals.var_t0_dn10)), (assign68480_e105127 * (locals.var_tox0 * locals.var_t0_dn11)), (assign68480_e105127 * (locals.var_tox0 * locals.var_t0_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign68480_e105129;
        locals.var_t2_dn0 = assign68480_e105129_d_n0;
        locals.var_t2_dn2 = assign68480_e105129_d_n2;
        locals.var_t2_dn4 = assign68480_e105129_d_n4;
        locals.var_t2_dn5 = assign68480_e105129_d_n5;
        locals.var_t2_dn6 = assign68480_e105129_d_n6;
        locals.var_t2_dn7 = assign68480_e105129_d_n7;
        locals.var_t2_dn8 = assign68480_e105129_d_n8;
        locals.var_t2_dn9 = assign68480_e105129_d_n9;
        locals.var_t2_dn10 = assign68480_e105129_d_n10;
        locals.var_t2_dn11 = assign68480_e105129_d_n11;
        locals.var_t2_dn14 = assign68480_e105129_d_n14;

        let (assign68490_e105137, assign68490_e105137_d_n0, assign68490_e105137_d_n2, assign68490_e105137_d_n4, assign68490_e105137_d_n5, assign68490_e105137_d_n6, assign68490_e105137_d_n7, assign68490_e105137_d_n8, assign68490_e105137_d_n9, assign68490_e105137_d_n10, assign68490_e105137_d_n11, assign68490_e105137_d_n14,) = {
    if (locals.var_guard1607 != 0.0) {
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_tox0;
        let assign68490_e105133: f64 = (locals.var_vgs * __rspice_inv_cse_0);
        let assign68490_e105135: f64 = (assign68490_e105133 * __rspice_inv_cse_0);
        (assign68490_e105135, 0.0, 0.0, 0.0, 0.0, ((locals.var_vgs_dn6 / locals.var_tox0) / locals.var_tox0), ((locals.var_vgs_dn7 / locals.var_tox0) / locals.var_tox0), ((locals.var_vgs_dn8 / locals.var_tox0) / locals.var_tox0), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign68490_e105137;
        locals.var_t0_dn0 = assign68490_e105137_d_n0;
        locals.var_t0_dn2 = assign68490_e105137_d_n2;
        locals.var_t0_dn4 = assign68490_e105137_d_n4;
        locals.var_t0_dn5 = assign68490_e105137_d_n5;
        locals.var_t0_dn6 = assign68490_e105137_d_n6;
        locals.var_t0_dn7 = assign68490_e105137_d_n7;
        locals.var_t0_dn8 = assign68490_e105137_d_n8;
        locals.var_t0_dn9 = assign68490_e105137_d_n9;
        locals.var_t0_dn10 = assign68490_e105137_d_n10;
        locals.var_t0_dn11 = assign68490_e105137_d_n11;
        locals.var_t0_dn14 = assign68490_e105137_d_n14;

        let (assign68500_e105143, assign68500_e105143_d_n0, assign68500_e105143_d_n2, assign68500_e105143_d_n4, assign68500_e105143_d_n5, assign68500_e105143_d_n6, assign68500_e105143_d_n7, assign68500_e105143_d_n8, assign68500_e105143_d_n9, assign68500_e105143_d_n10, assign68500_e105143_d_n11, assign68500_e105143_d_n14,) = {
    if (locals.var_guard1607 != 0.0) {
        let assign68500_e105141: f64 = (locals.var_vgs * locals.var_t0);
        (assign68500_e105141, (locals.var_vgs * locals.var_t0_dn0), (locals.var_vgs * locals.var_t0_dn2), (locals.var_vgs * locals.var_t0_dn4), (locals.var_vgs * locals.var_t0_dn5), ((locals.var_vgs_dn6 * locals.var_t0) + (locals.var_vgs * locals.var_t0_dn6)), ((locals.var_vgs_dn7 * locals.var_t0) + (locals.var_vgs * locals.var_t0_dn7)), ((locals.var_vgs_dn8 * locals.var_t0) + (locals.var_vgs * locals.var_t0_dn8)), (locals.var_vgs * locals.var_t0_dn9), (locals.var_vgs * locals.var_t0_dn10), (locals.var_vgs * locals.var_t0_dn11), (locals.var_vgs * locals.var_t0_dn14),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign68500_e105143;
        locals.var_t3_dn0 = assign68500_e105143_d_n0;
        locals.var_t3_dn2 = assign68500_e105143_d_n2;
        locals.var_t3_dn4 = assign68500_e105143_d_n4;
        locals.var_t3_dn5 = assign68500_e105143_d_n5;
        locals.var_t3_dn6 = assign68500_e105143_d_n6;
        locals.var_t3_dn7 = assign68500_e105143_d_n7;
        locals.var_t3_dn8 = assign68500_e105143_d_n8;
        locals.var_t3_dn9 = assign68500_e105143_d_n9;
        locals.var_t3_dn10 = assign68500_e105143_d_n10;
        locals.var_t3_dn11 = assign68500_e105143_d_n11;
        locals.var_t3_dn14 = assign68500_e105143_d_n14;

        let (assign68510_e105151, assign68510_e105151_d_n0, assign68510_e105151_d_n2, assign68510_e105151_d_n4, assign68510_e105151_d_n5, assign68510_e105151_d_n6, assign68510_e105151_d_n7, assign68510_e105151_d_n8, assign68510_e105151_d_n9, assign68510_e105151_d_n10, assign68510_e105151_d_n11, assign68510_e105151_d_n14,) = {
    if (locals.var_guard1607 != 0.0) {
        let assign68510_e105147: f64 = (locals.var_uc_glksd1 / 1000000.0);
        let assign68510_e105149: f64 = (assign68510_e105147 * locals.var_weff_nf);
        (assign68510_e105149, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign68510_e105151;
        locals.var_t4_dn0 = assign68510_e105151_d_n0;
        locals.var_t4_dn2 = assign68510_e105151_d_n2;
        locals.var_t4_dn4 = assign68510_e105151_d_n4;
        locals.var_t4_dn5 = assign68510_e105151_d_n5;
        locals.var_t4_dn6 = assign68510_e105151_d_n6;
        locals.var_t4_dn7 = assign68510_e105151_d_n7;
        locals.var_t4_dn8 = assign68510_e105151_d_n8;
        locals.var_t4_dn9 = assign68510_e105151_d_n9;
        locals.var_t4_dn10 = assign68510_e105151_d_n10;
        locals.var_t4_dn11 = assign68510_e105151_d_n11;
        locals.var_t4_dn14 = assign68510_e105151_d_n14;

        let (assign68520_e105159, assign68520_e105159_d_n0, assign68520_e105159_d_n2, assign68520_e105159_d_n4, assign68520_e105159_d_n5, assign68520_e105159_d_n6, assign68520_e105159_d_n7, assign68520_e105159_d_n8, assign68520_e105159_d_n9, assign68520_e105159_d_n10, assign68520_e105159_d_n11, assign68520_e105159_d_n14,) = {
    if (locals.var_guard1607 != 0.0) {
        let assign68520_e105155: f64 = (locals.var_t4 * locals.var_t2);
        let assign68520_e105157: f64 = (assign68520_e105155 * locals.var_t3);
        (assign68520_e105157, ((((locals.var_t4_dn0 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn0)) * locals.var_t3) + (assign68520_e105155 * locals.var_t3_dn0)), ((((locals.var_t4_dn2 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn2)) * locals.var_t3) + (assign68520_e105155 * locals.var_t3_dn2)), ((((locals.var_t4_dn4 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn4)) * locals.var_t3) + (assign68520_e105155 * locals.var_t3_dn4)), ((((locals.var_t4_dn5 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn5)) * locals.var_t3) + (assign68520_e105155 * locals.var_t3_dn5)), ((((locals.var_t4_dn6 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn6)) * locals.var_t3) + (assign68520_e105155 * locals.var_t3_dn6)), ((((locals.var_t4_dn7 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn7)) * locals.var_t3) + (assign68520_e105155 * locals.var_t3_dn7)), ((((locals.var_t4_dn8 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn8)) * locals.var_t3) + (assign68520_e105155 * locals.var_t3_dn8)), ((((locals.var_t4_dn9 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn9)) * locals.var_t3) + (assign68520_e105155 * locals.var_t3_dn9)), ((((locals.var_t4_dn10 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn10)) * locals.var_t3) + (assign68520_e105155 * locals.var_t3_dn10)), ((((locals.var_t4_dn11 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn11)) * locals.var_t3) + (assign68520_e105155 * locals.var_t3_dn11)), ((((locals.var_t4_dn14 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn14)) * locals.var_t3) + (assign68520_e105155 * locals.var_t3_dn14)),)
    } else {
        (locals.var_igs, locals.var_igs_dn0, locals.var_igs_dn2, locals.var_igs_dn4, locals.var_igs_dn5, locals.var_igs_dn6, locals.var_igs_dn7, locals.var_igs_dn8, locals.var_igs_dn9, locals.var_igs_dn10, locals.var_igs_dn11, locals.var_igs_dn14,)
    }
};
        locals.var_igs = assign68520_e105159;
        locals.var_igs_dn0 = assign68520_e105159_d_n0;
        locals.var_igs_dn2 = assign68520_e105159_d_n2;
        locals.var_igs_dn4 = assign68520_e105159_d_n4;
        locals.var_igs_dn5 = assign68520_e105159_d_n5;
        locals.var_igs_dn6 = assign68520_e105159_d_n6;
        locals.var_igs_dn7 = assign68520_e105159_d_n7;
        locals.var_igs_dn8 = assign68520_e105159_d_n8;
        locals.var_igs_dn9 = assign68520_e105159_d_n9;
        locals.var_igs_dn10 = assign68520_e105159_d_n10;
        locals.var_igs_dn11 = assign68520_e105159_d_n11;
        locals.var_igs_dn14 = assign68520_e105159_d_n14;

        let assign68530_e105162: f64 = if locals.var_vgs >= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1612 = assign68530_e105162;

        let (assign68540_e105171, assign68540_e105171_d_n0, assign68540_e105171_d_n2, assign68540_e105171_d_n4, assign68540_e105171_d_n5, assign68540_e105171_d_n6, assign68540_e105171_d_n7, assign68540_e105171_d_n8, assign68540_e105171_d_n9, assign68540_e105171_d_n10, assign68540_e105171_d_n11, assign68540_e105171_d_n14,) = {
    if ((locals.var_guard1607 != 0.0) && (locals.var_guard1612 != 0.0)) {
        let assign68540_e105168: f64 = (-1.0);
        let assign68540_e105169: f64 = (locals.var_igs * assign68540_e105168);
        (assign68540_e105169, (locals.var_igs_dn0 * assign68540_e105168), (locals.var_igs_dn2 * assign68540_e105168), (locals.var_igs_dn4 * assign68540_e105168), (locals.var_igs_dn5 * assign68540_e105168), (locals.var_igs_dn6 * assign68540_e105168), (locals.var_igs_dn7 * assign68540_e105168), (locals.var_igs_dn8 * assign68540_e105168), (locals.var_igs_dn9 * assign68540_e105168), (locals.var_igs_dn10 * assign68540_e105168), (locals.var_igs_dn11 * assign68540_e105168), (locals.var_igs_dn14 * assign68540_e105168),)
    } else {
        (locals.var_igs, locals.var_igs_dn0, locals.var_igs_dn2, locals.var_igs_dn4, locals.var_igs_dn5, locals.var_igs_dn6, locals.var_igs_dn7, locals.var_igs_dn8, locals.var_igs_dn9, locals.var_igs_dn10, locals.var_igs_dn11, locals.var_igs_dn14,)
    }
};
        locals.var_igs = assign68540_e105171;
        locals.var_igs_dn0 = assign68540_e105171_d_n0;
        locals.var_igs_dn2 = assign68540_e105171_d_n2;
        locals.var_igs_dn4 = assign68540_e105171_d_n4;
        locals.var_igs_dn5 = assign68540_e105171_d_n5;
        locals.var_igs_dn6 = assign68540_e105171_d_n6;
        locals.var_igs_dn7 = assign68540_e105171_d_n7;
        locals.var_igs_dn8 = assign68540_e105171_d_n8;
        locals.var_igs_dn9 = assign68540_e105171_d_n9;
        locals.var_igs_dn10 = assign68540_e105171_d_n10;
        locals.var_igs_dn11 = assign68540_e105171_d_n11;
        locals.var_igs_dn14 = assign68540_e105171_d_n14;

        let (assign68550_e105177, assign68550_e105177_d_n0, assign68550_e105177_d_n2, assign68550_e105177_d_n4, assign68550_e105177_d_n5, assign68550_e105177_d_n6, assign68550_e105177_d_n7, assign68550_e105177_d_n8, assign68550_e105177_d_n9, assign68550_e105177_d_n10, assign68550_e105177_d_n11, assign68550_e105177_d_n14,) = {
    if (locals.var_guard1607 != 0.0) {
        let assign68550_e105175: f64 = (locals.var_vgs - locals.var_vds);
        (assign68550_e105175, (-locals.var_vds_dn0), (-locals.var_vds_dn2), (-locals.var_vds_dn4), (-locals.var_vds_dn5), (locals.var_vgs_dn6 - locals.var_vds_dn6), (locals.var_vgs_dn7 - locals.var_vds_dn7), (locals.var_vgs_dn8 - locals.var_vds_dn8), (-locals.var_vds_dn9), (-locals.var_vds_dn10), (-locals.var_vds_dn11), (-locals.var_vds_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign68550_e105177;
        locals.var_t1_dn0 = assign68550_e105177_d_n0;
        locals.var_t1_dn2 = assign68550_e105177_d_n2;
        locals.var_t1_dn4 = assign68550_e105177_d_n4;
        locals.var_t1_dn5 = assign68550_e105177_d_n5;
        locals.var_t1_dn6 = assign68550_e105177_d_n6;
        locals.var_t1_dn7 = assign68550_e105177_d_n7;
        locals.var_t1_dn8 = assign68550_e105177_d_n8;
        locals.var_t1_dn9 = assign68550_e105177_d_n9;
        locals.var_t1_dn10 = assign68550_e105177_d_n10;
        locals.var_t1_dn11 = assign68550_e105177_d_n11;
        locals.var_t1_dn14 = assign68550_e105177_d_n14;

        let (assign68560_e105186, assign68560_e105186_d_n0, assign68560_e105186_d_n2, assign68560_e105186_d_n4, assign68560_e105186_d_n5, assign68560_e105186_d_n6, assign68560_e105186_d_n7, assign68560_e105186_d_n8, assign68560_e105186_d_n9, assign68560_e105186_d_n10, assign68560_e105186_d_n11, assign68560_e105186_d_n14,) = {
    if (locals.var_guard1607 != 0.0) {
        let assign68560_e105180: f64 = (-locals.var_uc_glksd2);
        let assign68560_e105182: f64 = (assign68560_e105180 * locals.var_t1);
        let assign68560_e105184: f64 = (assign68560_e105182 + locals.var_mks_glksd3);
        (assign68560_e105184, (assign68560_e105180 * locals.var_t1_dn0), (assign68560_e105180 * locals.var_t1_dn2), (assign68560_e105180 * locals.var_t1_dn4), (assign68560_e105180 * locals.var_t1_dn5), (assign68560_e105180 * locals.var_t1_dn6), (assign68560_e105180 * locals.var_t1_dn7), (assign68560_e105180 * locals.var_t1_dn8), (assign68560_e105180 * locals.var_t1_dn9), (assign68560_e105180 * locals.var_t1_dn10), (assign68560_e105180 * locals.var_t1_dn11), (assign68560_e105180 * locals.var_t1_dn14),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign68560_e105186;
        locals.var_t0_dn0 = assign68560_e105186_d_n0;
        locals.var_t0_dn2 = assign68560_e105186_d_n2;
        locals.var_t0_dn4 = assign68560_e105186_d_n4;
        locals.var_t0_dn5 = assign68560_e105186_d_n5;
        locals.var_t0_dn6 = assign68560_e105186_d_n6;
        locals.var_t0_dn7 = assign68560_e105186_d_n7;
        locals.var_t0_dn8 = assign68560_e105186_d_n8;
        locals.var_t0_dn9 = assign68560_e105186_d_n9;
        locals.var_t0_dn10 = assign68560_e105186_d_n10;
        locals.var_t0_dn11 = assign68560_e105186_d_n11;
        locals.var_t0_dn14 = assign68560_e105186_d_n14;

        let (assign68570_e105193, assign68570_e105193_d_n0, assign68570_e105193_d_n2, assign68570_e105193_d_n4, assign68570_e105193_d_n5, assign68570_e105193_d_n6, assign68570_e105193_d_n7, assign68570_e105193_d_n8, assign68570_e105193_d_n9, assign68570_e105193_d_n10, assign68570_e105193_d_n11, assign68570_e105193_d_n14,) = {
    if (locals.var_guard1607 != 0.0) {
        let assign68570_e105190: f64 = (locals.var_tox0 * locals.var_t0);
        let assign68570_e105191: f64 = (assign68570_e105190).exp();
        (assign68570_e105191, (assign68570_e105191 * (locals.var_tox0 * locals.var_t0_dn0)), (assign68570_e105191 * (locals.var_tox0 * locals.var_t0_dn2)), (assign68570_e105191 * (locals.var_tox0 * locals.var_t0_dn4)), (assign68570_e105191 * (locals.var_tox0 * locals.var_t0_dn5)), (assign68570_e105191 * (locals.var_tox0 * locals.var_t0_dn6)), (assign68570_e105191 * (locals.var_tox0 * locals.var_t0_dn7)), (assign68570_e105191 * (locals.var_tox0 * locals.var_t0_dn8)), (assign68570_e105191 * (locals.var_tox0 * locals.var_t0_dn9)), (assign68570_e105191 * (locals.var_tox0 * locals.var_t0_dn10)), (assign68570_e105191 * (locals.var_tox0 * locals.var_t0_dn11)), (assign68570_e105191 * (locals.var_tox0 * locals.var_t0_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign68570_e105193;
        locals.var_t2_dn0 = assign68570_e105193_d_n0;
        locals.var_t2_dn2 = assign68570_e105193_d_n2;
        locals.var_t2_dn4 = assign68570_e105193_d_n4;
        locals.var_t2_dn5 = assign68570_e105193_d_n5;
        locals.var_t2_dn6 = assign68570_e105193_d_n6;
        locals.var_t2_dn7 = assign68570_e105193_d_n7;
        locals.var_t2_dn8 = assign68570_e105193_d_n8;
        locals.var_t2_dn9 = assign68570_e105193_d_n9;
        locals.var_t2_dn10 = assign68570_e105193_d_n10;
        locals.var_t2_dn11 = assign68570_e105193_d_n11;
        locals.var_t2_dn14 = assign68570_e105193_d_n14;

        let (assign68580_e105201, assign68580_e105201_d_n0, assign68580_e105201_d_n2, assign68580_e105201_d_n4, assign68580_e105201_d_n5, assign68580_e105201_d_n6, assign68580_e105201_d_n7, assign68580_e105201_d_n8, assign68580_e105201_d_n9, assign68580_e105201_d_n10, assign68580_e105201_d_n11, assign68580_e105201_d_n14,) = {
    if (locals.var_guard1607 != 0.0) {
        let __rspice_inv_cse_1: f64 = 1.0 / locals.var_tox0;
        let assign68580_e105197: f64 = (locals.var_t1 * __rspice_inv_cse_1);
        let assign68580_e105199: f64 = (assign68580_e105197 * __rspice_inv_cse_1);
        (assign68580_e105199, ((locals.var_t1_dn0 / locals.var_tox0) / locals.var_tox0), ((locals.var_t1_dn2 / locals.var_tox0) / locals.var_tox0), ((locals.var_t1_dn4 / locals.var_tox0) / locals.var_tox0), ((locals.var_t1_dn5 / locals.var_tox0) / locals.var_tox0), ((locals.var_t1_dn6 / locals.var_tox0) / locals.var_tox0), ((locals.var_t1_dn7 / locals.var_tox0) / locals.var_tox0), ((locals.var_t1_dn8 / locals.var_tox0) / locals.var_tox0), ((locals.var_t1_dn9 / locals.var_tox0) / locals.var_tox0), ((locals.var_t1_dn10 / locals.var_tox0) / locals.var_tox0), ((locals.var_t1_dn11 / locals.var_tox0) / locals.var_tox0), ((locals.var_t1_dn14 / locals.var_tox0) / locals.var_tox0),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign68580_e105201;
        locals.var_t0_dn0 = assign68580_e105201_d_n0;
        locals.var_t0_dn2 = assign68580_e105201_d_n2;
        locals.var_t0_dn4 = assign68580_e105201_d_n4;
        locals.var_t0_dn5 = assign68580_e105201_d_n5;
        locals.var_t0_dn6 = assign68580_e105201_d_n6;
        locals.var_t0_dn7 = assign68580_e105201_d_n7;
        locals.var_t0_dn8 = assign68580_e105201_d_n8;
        locals.var_t0_dn9 = assign68580_e105201_d_n9;
        locals.var_t0_dn10 = assign68580_e105201_d_n10;
        locals.var_t0_dn11 = assign68580_e105201_d_n11;
        locals.var_t0_dn14 = assign68580_e105201_d_n14;

        let (assign68590_e105207, assign68590_e105207_d_n0, assign68590_e105207_d_n2, assign68590_e105207_d_n4, assign68590_e105207_d_n5, assign68590_e105207_d_n6, assign68590_e105207_d_n7, assign68590_e105207_d_n8, assign68590_e105207_d_n9, assign68590_e105207_d_n10, assign68590_e105207_d_n11, assign68590_e105207_d_n14,) = {
    if (locals.var_guard1607 != 0.0) {
        let assign68590_e105205: f64 = (locals.var_t1 * locals.var_t0);
        (assign68590_e105205, ((locals.var_t1_dn0 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn0)), ((locals.var_t1_dn2 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn2)), ((locals.var_t1_dn4 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn4)), ((locals.var_t1_dn5 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn5)), ((locals.var_t1_dn6 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn6)), ((locals.var_t1_dn7 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn7)), ((locals.var_t1_dn8 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn8)), ((locals.var_t1_dn9 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn9)), ((locals.var_t1_dn10 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn10)), ((locals.var_t1_dn11 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn11)), ((locals.var_t1_dn14 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn14)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign68590_e105207;
        locals.var_t3_dn0 = assign68590_e105207_d_n0;
        locals.var_t3_dn2 = assign68590_e105207_d_n2;
        locals.var_t3_dn4 = assign68590_e105207_d_n4;
        locals.var_t3_dn5 = assign68590_e105207_d_n5;
        locals.var_t3_dn6 = assign68590_e105207_d_n6;
        locals.var_t3_dn7 = assign68590_e105207_d_n7;
        locals.var_t3_dn8 = assign68590_e105207_d_n8;
        locals.var_t3_dn9 = assign68590_e105207_d_n9;
        locals.var_t3_dn10 = assign68590_e105207_d_n10;
        locals.var_t3_dn11 = assign68590_e105207_d_n11;
        locals.var_t3_dn14 = assign68590_e105207_d_n14;

        let (assign68600_e105215, assign68600_e105215_d_n0, assign68600_e105215_d_n2, assign68600_e105215_d_n4, assign68600_e105215_d_n5, assign68600_e105215_d_n6, assign68600_e105215_d_n7, assign68600_e105215_d_n8, assign68600_e105215_d_n9, assign68600_e105215_d_n10, assign68600_e105215_d_n11, assign68600_e105215_d_n14,) = {
    if (locals.var_guard1607 != 0.0) {
        let assign68600_e105211: f64 = (locals.var_uc_glksd1 / 1000000.0);
        let assign68600_e105213: f64 = (assign68600_e105211 * locals.var_weff_nf);
        (assign68600_e105213, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign68600_e105215;
        locals.var_t4_dn0 = assign68600_e105215_d_n0;
        locals.var_t4_dn2 = assign68600_e105215_d_n2;
        locals.var_t4_dn4 = assign68600_e105215_d_n4;
        locals.var_t4_dn5 = assign68600_e105215_d_n5;
        locals.var_t4_dn6 = assign68600_e105215_d_n6;
        locals.var_t4_dn7 = assign68600_e105215_d_n7;
        locals.var_t4_dn8 = assign68600_e105215_d_n8;
        locals.var_t4_dn9 = assign68600_e105215_d_n9;
        locals.var_t4_dn10 = assign68600_e105215_d_n10;
        locals.var_t4_dn11 = assign68600_e105215_d_n11;
        locals.var_t4_dn14 = assign68600_e105215_d_n14;

        let (assign68610_e105223, assign68610_e105223_d_n0, assign68610_e105223_d_n2, assign68610_e105223_d_n4, assign68610_e105223_d_n5, assign68610_e105223_d_n6, assign68610_e105223_d_n7, assign68610_e105223_d_n8, assign68610_e105223_d_n9, assign68610_e105223_d_n10, assign68610_e105223_d_n11, assign68610_e105223_d_n14,) = {
    if (locals.var_guard1607 != 0.0) {
        let assign68610_e105219: f64 = (locals.var_t4 * locals.var_t2);
        let assign68610_e105221: f64 = (assign68610_e105219 * locals.var_t3);
        (assign68610_e105221, ((((locals.var_t4_dn0 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn0)) * locals.var_t3) + (assign68610_e105219 * locals.var_t3_dn0)), ((((locals.var_t4_dn2 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn2)) * locals.var_t3) + (assign68610_e105219 * locals.var_t3_dn2)), ((((locals.var_t4_dn4 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn4)) * locals.var_t3) + (assign68610_e105219 * locals.var_t3_dn4)), ((((locals.var_t4_dn5 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn5)) * locals.var_t3) + (assign68610_e105219 * locals.var_t3_dn5)), ((((locals.var_t4_dn6 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn6)) * locals.var_t3) + (assign68610_e105219 * locals.var_t3_dn6)), ((((locals.var_t4_dn7 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn7)) * locals.var_t3) + (assign68610_e105219 * locals.var_t3_dn7)), ((((locals.var_t4_dn8 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn8)) * locals.var_t3) + (assign68610_e105219 * locals.var_t3_dn8)), ((((locals.var_t4_dn9 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn9)) * locals.var_t3) + (assign68610_e105219 * locals.var_t3_dn9)), ((((locals.var_t4_dn10 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn10)) * locals.var_t3) + (assign68610_e105219 * locals.var_t3_dn10)), ((((locals.var_t4_dn11 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn11)) * locals.var_t3) + (assign68610_e105219 * locals.var_t3_dn11)), ((((locals.var_t4_dn14 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn14)) * locals.var_t3) + (assign68610_e105219 * locals.var_t3_dn14)),)
    } else {
        (locals.var_igd, locals.var_igd_dn0, locals.var_igd_dn2, locals.var_igd_dn4, locals.var_igd_dn5, locals.var_igd_dn6, locals.var_igd_dn7, locals.var_igd_dn8, locals.var_igd_dn9, locals.var_igd_dn10, locals.var_igd_dn11, locals.var_igd_dn14,)
    }
};
        locals.var_igd = assign68610_e105223;
        locals.var_igd_dn0 = assign68610_e105223_d_n0;
        locals.var_igd_dn2 = assign68610_e105223_d_n2;
        locals.var_igd_dn4 = assign68610_e105223_d_n4;
        locals.var_igd_dn5 = assign68610_e105223_d_n5;
        locals.var_igd_dn6 = assign68610_e105223_d_n6;
        locals.var_igd_dn7 = assign68610_e105223_d_n7;
        locals.var_igd_dn8 = assign68610_e105223_d_n8;
        locals.var_igd_dn9 = assign68610_e105223_d_n9;
        locals.var_igd_dn10 = assign68610_e105223_d_n10;
        locals.var_igd_dn11 = assign68610_e105223_d_n11;
        locals.var_igd_dn14 = assign68610_e105223_d_n14;

        let assign68620_e105226: f64 = if locals.var_t1 >= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1613 = assign68620_e105226;

    }

    pub(super) fn stamp_transient_block_245(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign68630_e105235, assign68630_e105235_d_n0, assign68630_e105235_d_n2, assign68630_e105235_d_n4, assign68630_e105235_d_n5, assign68630_e105235_d_n6, assign68630_e105235_d_n7, assign68630_e105235_d_n8, assign68630_e105235_d_n9, assign68630_e105235_d_n10, assign68630_e105235_d_n11, assign68630_e105235_d_n14,) = {
    if ((locals.var_guard1607 != 0.0) && (locals.var_guard1613 != 0.0)) {
        let assign68630_e105232: f64 = (-1.0);
        let assign68630_e105233: f64 = (locals.var_igd * assign68630_e105232);
        (assign68630_e105233, (locals.var_igd_dn0 * assign68630_e105232), (locals.var_igd_dn2 * assign68630_e105232), (locals.var_igd_dn4 * assign68630_e105232), (locals.var_igd_dn5 * assign68630_e105232), (locals.var_igd_dn6 * assign68630_e105232), (locals.var_igd_dn7 * assign68630_e105232), (locals.var_igd_dn8 * assign68630_e105232), (locals.var_igd_dn9 * assign68630_e105232), (locals.var_igd_dn10 * assign68630_e105232), (locals.var_igd_dn11 * assign68630_e105232), (locals.var_igd_dn14 * assign68630_e105232),)
    } else {
        (locals.var_igd, locals.var_igd_dn0, locals.var_igd_dn2, locals.var_igd_dn4, locals.var_igd_dn5, locals.var_igd_dn6, locals.var_igd_dn7, locals.var_igd_dn8, locals.var_igd_dn9, locals.var_igd_dn10, locals.var_igd_dn11, locals.var_igd_dn14,)
    }
};
        locals.var_igd = assign68630_e105235;
        locals.var_igd_dn0 = assign68630_e105235_d_n0;
        locals.var_igd_dn2 = assign68630_e105235_d_n2;
        locals.var_igd_dn4 = assign68630_e105235_d_n4;
        locals.var_igd_dn5 = assign68630_e105235_d_n5;
        locals.var_igd_dn6 = assign68630_e105235_d_n6;
        locals.var_igd_dn7 = assign68630_e105235_d_n7;
        locals.var_igd_dn8 = assign68630_e105235_d_n8;
        locals.var_igd_dn9 = assign68630_e105235_d_n9;
        locals.var_igd_dn10 = assign68630_e105235_d_n10;
        locals.var_igd_dn11 = assign68630_e105235_d_n11;
        locals.var_igd_dn14 = assign68630_e105235_d_n14;

        let (assign68640_e105248, assign68640_e105248_d_n0, assign68640_e105248_d_n2, assign68640_e105248_d_n4, assign68640_e105248_d_n5, assign68640_e105248_d_n6, assign68640_e105248_d_n7, assign68640_e105248_d_n8, assign68640_e105248_d_n9, assign68640_e105248_d_n10, assign68640_e105248_d_n11, assign68640_e105248_d_n14,) = {
    if (locals.var_guard1607 != 0.0) {
        let assign68640_e105239: f64 = (locals.var_vgs - locals.var_vbs);
        let assign68640_e105240: f64 = (-assign68640_e105239);
        let assign68640_e105242: f64 = (assign68640_e105240 + locals.var_vfb);
        let assign68640_e105244: f64 = (assign68640_e105242 + p.p258);
        let assign68640_e105246: f64 = (assign68640_e105244 / locals.var_tox0);
        (assign68640_e105246, 0.0, 0.0, 0.0, 0.0, ((-(locals.var_vgs_dn6 - locals.var_vbs_dn6)) / locals.var_tox0), ((-locals.var_vgs_dn7) / locals.var_tox0), ((-(locals.var_vgs_dn8 - locals.var_vbs_dn8)) / locals.var_tox0), ((-(-locals.var_vbs_dn9)) / locals.var_tox0), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_etun, locals.var_etun_dn0, locals.var_etun_dn2, locals.var_etun_dn4, locals.var_etun_dn5, locals.var_etun_dn6, locals.var_etun_dn7, locals.var_etun_dn8, locals.var_etun_dn9, locals.var_etun_dn10, locals.var_etun_dn11, locals.var_etun_dn14,)
    }
};
        locals.var_etun = assign68640_e105248;
        locals.var_etun_dn0 = assign68640_e105248_d_n0;
        locals.var_etun_dn2 = assign68640_e105248_d_n2;
        locals.var_etun_dn4 = assign68640_e105248_d_n4;
        locals.var_etun_dn5 = assign68640_e105248_d_n5;
        locals.var_etun_dn6 = assign68640_e105248_d_n6;
        locals.var_etun_dn7 = assign68640_e105248_d_n7;
        locals.var_etun_dn8 = assign68640_e105248_d_n8;
        locals.var_etun_dn9 = assign68640_e105248_d_n9;
        locals.var_etun_dn10 = assign68640_e105248_d_n10;
        locals.var_etun_dn11 = assign68640_e105248_d_n11;
        locals.var_etun_dn14 = assign68640_e105248_d_n14;

        let (assign68650_e105265, assign68650_e105265_d_n0, assign68650_e105265_d_n2, assign68650_e105265_d_n4, assign68650_e105265_d_n5, assign68650_e105265_d_n6, assign68650_e105265_d_n7, assign68650_e105265_d_n8, assign68650_e105265_d_n9, assign68650_e105265_d_n10, assign68650_e105265_d_n11, assign68650_e105265_d_n14,) = {
    if (locals.var_guard1607 != 0.0) {
        let assign68650_e105252: f64 = (locals.var_etun * locals.var_etun);
        let assign68650_e105256: f64 = (0.01 / 0.01);
        let assign68650_e105257: f64 = (4.0 * assign68650_e105256);
        let assign68650_e105260: f64 = (0.01 / 0.01);
        let assign68650_e105261: f64 = (assign68650_e105257 * assign68650_e105260);
        let assign68650_e105262: f64 = (assign68650_e105252 + assign68650_e105261);
        let assign68650_e105263: f64 = (assign68650_e105262).sqrt();
        (assign68650_e105263, (((locals.var_etun_dn0 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn0)) / (2.0 * assign68650_e105263)), (((locals.var_etun_dn2 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn2)) / (2.0 * assign68650_e105263)), (((locals.var_etun_dn4 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn4)) / (2.0 * assign68650_e105263)), (((locals.var_etun_dn5 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn5)) / (2.0 * assign68650_e105263)), (((locals.var_etun_dn6 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn6)) / (2.0 * assign68650_e105263)), (((locals.var_etun_dn7 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn7)) / (2.0 * assign68650_e105263)), (((locals.var_etun_dn8 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn8)) / (2.0 * assign68650_e105263)), (((locals.var_etun_dn9 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn9)) / (2.0 * assign68650_e105263)), (((locals.var_etun_dn10 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn10)) / (2.0 * assign68650_e105263)), (((locals.var_etun_dn11 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn11)) / (2.0 * assign68650_e105263)), (((locals.var_etun_dn14 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn14)) / (2.0 * assign68650_e105263)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign68650_e105265;
        locals.var_tmf2_dn0 = assign68650_e105265_d_n0;
        locals.var_tmf2_dn2 = assign68650_e105265_d_n2;
        locals.var_tmf2_dn4 = assign68650_e105265_d_n4;
        locals.var_tmf2_dn5 = assign68650_e105265_d_n5;
        locals.var_tmf2_dn6 = assign68650_e105265_d_n6;
        locals.var_tmf2_dn7 = assign68650_e105265_d_n7;
        locals.var_tmf2_dn8 = assign68650_e105265_d_n8;
        locals.var_tmf2_dn9 = assign68650_e105265_d_n9;
        locals.var_tmf2_dn10 = assign68650_e105265_d_n10;
        locals.var_tmf2_dn11 = assign68650_e105265_d_n11;
        locals.var_tmf2_dn14 = assign68650_e105265_d_n14;

        let (assign68660_e105275, assign68660_e105275_d_n0, assign68660_e105275_d_n2, assign68660_e105275_d_n4, assign68660_e105275_d_n5, assign68660_e105275_d_n6, assign68660_e105275_d_n7, assign68660_e105275_d_n8, assign68660_e105275_d_n9, assign68660_e105275_d_n10, assign68660_e105275_d_n11, assign68660_e105275_d_n14,) = {
    if (locals.var_guard1607 != 0.0) {
        let assign68660_e105271: f64 = (locals.var_etun / locals.var_tmf2);
        let assign68660_e105272: f64 = (1.0 + assign68660_e105271);
        let assign68660_e105273: f64 = (0.5 * assign68660_e105272);
        (assign68660_e105273, (0.5 * (((locals.var_etun_dn0 * locals.var_tmf2) - (locals.var_etun * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_etun_dn2 * locals.var_tmf2) - (locals.var_etun * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_etun_dn4 * locals.var_tmf2) - (locals.var_etun * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_etun_dn5 * locals.var_tmf2) - (locals.var_etun * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_etun_dn6 * locals.var_tmf2) - (locals.var_etun * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_etun_dn7 * locals.var_tmf2) - (locals.var_etun * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_etun_dn8 * locals.var_tmf2) - (locals.var_etun * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_etun_dn9 * locals.var_tmf2) - (locals.var_etun * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_etun_dn10 * locals.var_tmf2) - (locals.var_etun * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_etun_dn11 * locals.var_tmf2) - (locals.var_etun * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_etun_dn14 * locals.var_tmf2) - (locals.var_etun * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign68660_e105275;
        locals.var_t5_dn0 = assign68660_e105275_d_n0;
        locals.var_t5_dn2 = assign68660_e105275_d_n2;
        locals.var_t5_dn4 = assign68660_e105275_d_n4;
        locals.var_t5_dn5 = assign68660_e105275_d_n5;
        locals.var_t5_dn6 = assign68660_e105275_d_n6;
        locals.var_t5_dn7 = assign68660_e105275_d_n7;
        locals.var_t5_dn8 = assign68660_e105275_d_n8;
        locals.var_t5_dn9 = assign68660_e105275_d_n9;
        locals.var_t5_dn10 = assign68660_e105275_d_n10;
        locals.var_t5_dn11 = assign68660_e105275_d_n11;
        locals.var_t5_dn14 = assign68660_e105275_d_n14;

        let (assign68670_e105283, assign68670_e105283_d_n0, assign68670_e105283_d_n2, assign68670_e105283_d_n4, assign68670_e105283_d_n5, assign68670_e105283_d_n6, assign68670_e105283_d_n7, assign68670_e105283_d_n8, assign68670_e105283_d_n9, assign68670_e105283_d_n10, assign68670_e105283_d_n11, assign68670_e105283_d_n14,) = {
    if (locals.var_guard1607 != 0.0) {
        let assign68670_e105280: f64 = (locals.var_etun + locals.var_tmf2);
        let assign68670_e105281: f64 = (0.5 * assign68670_e105280);
        (assign68670_e105281, (0.5 * (locals.var_etun_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_etun_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_etun_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_etun_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_etun_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_etun_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_etun_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_etun_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_etun_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_etun_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_etun_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_etun, locals.var_etun_dn0, locals.var_etun_dn2, locals.var_etun_dn4, locals.var_etun_dn5, locals.var_etun_dn6, locals.var_etun_dn7, locals.var_etun_dn8, locals.var_etun_dn9, locals.var_etun_dn10, locals.var_etun_dn11, locals.var_etun_dn14,)
    }
};
        locals.var_etun = assign68670_e105283;
        locals.var_etun_dn0 = assign68670_e105283_d_n0;
        locals.var_etun_dn2 = assign68670_e105283_d_n2;
        locals.var_etun_dn4 = assign68670_e105283_d_n4;
        locals.var_etun_dn5 = assign68670_e105283_d_n5;
        locals.var_etun_dn6 = assign68670_e105283_d_n6;
        locals.var_etun_dn7 = assign68670_e105283_d_n7;
        locals.var_etun_dn8 = assign68670_e105283_d_n8;
        locals.var_etun_dn9 = assign68670_e105283_d_n9;
        locals.var_etun_dn10 = assign68670_e105283_d_n10;
        locals.var_etun_dn11 = assign68670_e105283_d_n11;
        locals.var_etun_dn14 = assign68670_e105283_d_n14;

        let assign68680_e105286: f64 = if locals.var_etun < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1614 = assign68680_e105286;

        let (assign68690_e105292, assign68690_e105292_d_n0, assign68690_e105292_d_n2, assign68690_e105292_d_n4, assign68690_e105292_d_n5, assign68690_e105292_d_n6, assign68690_e105292_d_n7, assign68690_e105292_d_n8, assign68690_e105292_d_n9, assign68690_e105292_d_n10, assign68690_e105292_d_n11, assign68690_e105292_d_n14,) = {
    if ((locals.var_guard1607 != 0.0) && (locals.var_guard1614 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_etun, locals.var_etun_dn0, locals.var_etun_dn2, locals.var_etun_dn4, locals.var_etun_dn5, locals.var_etun_dn6, locals.var_etun_dn7, locals.var_etun_dn8, locals.var_etun_dn9, locals.var_etun_dn10, locals.var_etun_dn11, locals.var_etun_dn14,)
    }
};
        locals.var_etun = assign68690_e105292;
        locals.var_etun_dn0 = assign68690_e105292_d_n0;
        locals.var_etun_dn2 = assign68690_e105292_d_n2;
        locals.var_etun_dn4 = assign68690_e105292_d_n4;
        locals.var_etun_dn5 = assign68690_e105292_d_n5;
        locals.var_etun_dn6 = assign68690_e105292_d_n6;
        locals.var_etun_dn7 = assign68690_e105292_d_n7;
        locals.var_etun_dn8 = assign68690_e105292_d_n8;
        locals.var_etun_dn9 = assign68690_e105292_d_n9;
        locals.var_etun_dn10 = assign68690_e105292_d_n10;
        locals.var_etun_dn11 = assign68690_e105292_d_n11;
        locals.var_etun_dn14 = assign68690_e105292_d_n14;

        let (assign68700_e105298, assign68700_e105298_d_n0, assign68700_e105298_d_n2, assign68700_e105298_d_n4, assign68700_e105298_d_n5, assign68700_e105298_d_n6, assign68700_e105298_d_n7, assign68700_e105298_d_n8, assign68700_e105298_d_n9, assign68700_e105298_d_n10, assign68700_e105298_d_n11, assign68700_e105298_d_n14,) = {
    if ((locals.var_guard1607 != 0.0) && (locals.var_guard1614 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign68700_e105298;
        locals.var_t5_dn0 = assign68700_e105298_d_n0;
        locals.var_t5_dn2 = assign68700_e105298_d_n2;
        locals.var_t5_dn4 = assign68700_e105298_d_n4;
        locals.var_t5_dn5 = assign68700_e105298_d_n5;
        locals.var_t5_dn6 = assign68700_e105298_d_n6;
        locals.var_t5_dn7 = assign68700_e105298_d_n7;
        locals.var_t5_dn8 = assign68700_e105298_d_n8;
        locals.var_t5_dn9 = assign68700_e105298_d_n9;
        locals.var_t5_dn10 = assign68700_e105298_d_n10;
        locals.var_t5_dn11 = assign68700_e105298_d_n11;
        locals.var_t5_dn14 = assign68700_e105298_d_n14;

        let (assign68710_e105304, assign68710_e105304_d_n0, assign68710_e105304_d_n2, assign68710_e105304_d_n4, assign68710_e105304_d_n5, assign68710_e105304_d_n6, assign68710_e105304_d_n7, assign68710_e105304_d_n8, assign68710_e105304_d_n9, assign68710_e105304_d_n10, assign68710_e105304_d_n11, assign68710_e105304_d_n14,) = {
    if (locals.var_guard1607 != 0.0) {
        let assign68710_e105302: f64 = (locals.var_etun + 1e-25);
        (assign68710_e105302, locals.var_etun_dn0, locals.var_etun_dn2, locals.var_etun_dn4, locals.var_etun_dn5, locals.var_etun_dn6, locals.var_etun_dn7, locals.var_etun_dn8, locals.var_etun_dn9, locals.var_etun_dn10, locals.var_etun_dn11, locals.var_etun_dn14,)
    } else {
        (locals.var_etun, locals.var_etun_dn0, locals.var_etun_dn2, locals.var_etun_dn4, locals.var_etun_dn5, locals.var_etun_dn6, locals.var_etun_dn7, locals.var_etun_dn8, locals.var_etun_dn9, locals.var_etun_dn10, locals.var_etun_dn11, locals.var_etun_dn14,)
    }
};
        locals.var_etun = assign68710_e105304;
        locals.var_etun_dn0 = assign68710_e105304_d_n0;
        locals.var_etun_dn2 = assign68710_e105304_d_n2;
        locals.var_etun_dn4 = assign68710_e105304_d_n4;
        locals.var_etun_dn5 = assign68710_e105304_d_n5;
        locals.var_etun_dn6 = assign68710_e105304_d_n6;
        locals.var_etun_dn7 = assign68710_e105304_d_n7;
        locals.var_etun_dn8 = assign68710_e105304_d_n8;
        locals.var_etun_dn9 = assign68710_e105304_d_n9;
        locals.var_etun_dn10 = assign68710_e105304_d_n10;
        locals.var_etun_dn11 = assign68710_e105304_d_n11;
        locals.var_etun_dn14 = assign68710_e105304_d_n14;

        let (assign68720_e105311, assign68720_e105311_d_n0, assign68720_e105311_d_n2, assign68720_e105311_d_n4, assign68720_e105311_d_n5, assign68720_e105311_d_n6, assign68720_e105311_d_n7, assign68720_e105311_d_n8, assign68720_e105311_d_n9, assign68720_e105311_d_n10, assign68720_e105311_d_n11, assign68720_e105311_d_n14,) = {
    if (locals.var_guard1607 != 0.0) {
        let assign68720_e105307: f64 = (-locals.var_uc_glkb2);
        let assign68720_e105309: f64 = (assign68720_e105307 / locals.var_etun);
        (assign68720_e105309, (-((assign68720_e105307 * locals.var_etun_dn0) / (locals.var_etun * locals.var_etun))), (-((assign68720_e105307 * locals.var_etun_dn2) / (locals.var_etun * locals.var_etun))), (-((assign68720_e105307 * locals.var_etun_dn4) / (locals.var_etun * locals.var_etun))), (-((assign68720_e105307 * locals.var_etun_dn5) / (locals.var_etun * locals.var_etun))), (-((assign68720_e105307 * locals.var_etun_dn6) / (locals.var_etun * locals.var_etun))), (-((assign68720_e105307 * locals.var_etun_dn7) / (locals.var_etun * locals.var_etun))), (-((assign68720_e105307 * locals.var_etun_dn8) / (locals.var_etun * locals.var_etun))), (-((assign68720_e105307 * locals.var_etun_dn9) / (locals.var_etun * locals.var_etun))), (-((assign68720_e105307 * locals.var_etun_dn10) / (locals.var_etun * locals.var_etun))), (-((assign68720_e105307 * locals.var_etun_dn11) / (locals.var_etun * locals.var_etun))), (-((assign68720_e105307 * locals.var_etun_dn14) / (locals.var_etun * locals.var_etun))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign68720_e105311;
        locals.var_t1_dn0 = assign68720_e105311_d_n0;
        locals.var_t1_dn2 = assign68720_e105311_d_n2;
        locals.var_t1_dn4 = assign68720_e105311_d_n4;
        locals.var_t1_dn5 = assign68720_e105311_d_n5;
        locals.var_t1_dn6 = assign68720_e105311_d_n6;
        locals.var_t1_dn7 = assign68720_e105311_d_n7;
        locals.var_t1_dn8 = assign68720_e105311_d_n8;
        locals.var_t1_dn9 = assign68720_e105311_d_n9;
        locals.var_t1_dn10 = assign68720_e105311_d_n10;
        locals.var_t1_dn11 = assign68720_e105311_d_n11;
        locals.var_t1_dn14 = assign68720_e105311_d_n14;

        let assign68730_e105314: f64 = (-34.0);
        let assign68730_e105315: f64 = if locals.var_t1 < assign68730_e105314 { 1.0 } else { 0.0 };
        locals.var_guard1615 = assign68730_e105315;

        let (assign68740_e105321, assign68740_e105321_d_n0, assign68740_e105321_d_n2, assign68740_e105321_d_n4, assign68740_e105321_d_n5, assign68740_e105321_d_n6, assign68740_e105321_d_n7, assign68740_e105321_d_n8, assign68740_e105321_d_n9, assign68740_e105321_d_n10, assign68740_e105321_d_n11, assign68740_e105321_d_n14,) = {
    if ((locals.var_guard1607 != 0.0) && (locals.var_guard1615 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_igb, locals.var_igb_dn0, locals.var_igb_dn2, locals.var_igb_dn4, locals.var_igb_dn5, locals.var_igb_dn6, locals.var_igb_dn7, locals.var_igb_dn8, locals.var_igb_dn9, locals.var_igb_dn10, locals.var_igb_dn11, locals.var_igb_dn14,)
    }
};
        locals.var_igb = assign68740_e105321;
        locals.var_igb_dn0 = assign68740_e105321_d_n0;
        locals.var_igb_dn2 = assign68740_e105321_d_n2;
        locals.var_igb_dn4 = assign68740_e105321_d_n4;
        locals.var_igb_dn5 = assign68740_e105321_d_n5;
        locals.var_igb_dn6 = assign68740_e105321_d_n6;
        locals.var_igb_dn7 = assign68740_e105321_d_n7;
        locals.var_igb_dn8 = assign68740_e105321_d_n8;
        locals.var_igb_dn9 = assign68740_e105321_d_n9;
        locals.var_igb_dn10 = assign68740_e105321_d_n10;
        locals.var_igb_dn11 = assign68740_e105321_d_n11;
        locals.var_igb_dn14 = assign68740_e105321_d_n14;

        let (assign68750_e105329, assign68750_e105329_d_n0, assign68750_e105329_d_n2, assign68750_e105329_d_n4, assign68750_e105329_d_n5, assign68750_e105329_d_n6, assign68750_e105329_d_n7, assign68750_e105329_d_n8, assign68750_e105329_d_n9, assign68750_e105329_d_n10, assign68750_e105329_d_n11, assign68750_e105329_d_n14,) = {
    if ((locals.var_guard1607 != 0.0) && (locals.var_guard1615 == 0.0)) {
        let assign68750_e105327: f64 = (locals.var_t1).exp();
        (assign68750_e105327, (assign68750_e105327 * locals.var_t1_dn0), (assign68750_e105327 * locals.var_t1_dn2), (assign68750_e105327 * locals.var_t1_dn4), (assign68750_e105327 * locals.var_t1_dn5), (assign68750_e105327 * locals.var_t1_dn6), (assign68750_e105327 * locals.var_t1_dn7), (assign68750_e105327 * locals.var_t1_dn8), (assign68750_e105327 * locals.var_t1_dn9), (assign68750_e105327 * locals.var_t1_dn10), (assign68750_e105327 * locals.var_t1_dn11), (assign68750_e105327 * locals.var_t1_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign68750_e105329;
        locals.var_t2_dn0 = assign68750_e105329_d_n0;
        locals.var_t2_dn2 = assign68750_e105329_d_n2;
        locals.var_t2_dn4 = assign68750_e105329_d_n4;
        locals.var_t2_dn5 = assign68750_e105329_d_n5;
        locals.var_t2_dn6 = assign68750_e105329_d_n6;
        locals.var_t2_dn7 = assign68750_e105329_d_n7;
        locals.var_t2_dn8 = assign68750_e105329_d_n8;
        locals.var_t2_dn9 = assign68750_e105329_d_n9;
        locals.var_t2_dn10 = assign68750_e105329_d_n10;
        locals.var_t2_dn11 = assign68750_e105329_d_n11;
        locals.var_t2_dn14 = assign68750_e105329_d_n14;

        let (assign68760_e105342, assign68760_e105342_d_n0, assign68760_e105342_d_n2, assign68760_e105342_d_n4, assign68760_e105342_d_n5, assign68760_e105342_d_n6, assign68760_e105342_d_n7, assign68760_e105342_d_n8, assign68760_e105342_d_n9, assign68760_e105342_d_n10, assign68760_e105342_d_n11, assign68760_e105342_d_n14,) = {
    if ((locals.var_guard1607 != 0.0) && (locals.var_guard1615 == 0.0)) {
        let assign68760_e105337: f64 = (locals.var_etun * locals.var_etun);
        let assign68760_e105338: f64 = (locals.var_uc_glkb2 / assign68760_e105337);
        let assign68760_e105340: f64 = (assign68760_e105338 * locals.var_t2);
        (assign68760_e105340, (((-((locals.var_uc_glkb2 * ((locals.var_etun_dn0 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn0))) / (assign68760_e105337 * assign68760_e105337))) * locals.var_t2) + (assign68760_e105338 * locals.var_t2_dn0)), (((-((locals.var_uc_glkb2 * ((locals.var_etun_dn2 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn2))) / (assign68760_e105337 * assign68760_e105337))) * locals.var_t2) + (assign68760_e105338 * locals.var_t2_dn2)), (((-((locals.var_uc_glkb2 * ((locals.var_etun_dn4 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn4))) / (assign68760_e105337 * assign68760_e105337))) * locals.var_t2) + (assign68760_e105338 * locals.var_t2_dn4)), (((-((locals.var_uc_glkb2 * ((locals.var_etun_dn5 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn5))) / (assign68760_e105337 * assign68760_e105337))) * locals.var_t2) + (assign68760_e105338 * locals.var_t2_dn5)), (((-((locals.var_uc_glkb2 * ((locals.var_etun_dn6 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn6))) / (assign68760_e105337 * assign68760_e105337))) * locals.var_t2) + (assign68760_e105338 * locals.var_t2_dn6)), (((-((locals.var_uc_glkb2 * ((locals.var_etun_dn7 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn7))) / (assign68760_e105337 * assign68760_e105337))) * locals.var_t2) + (assign68760_e105338 * locals.var_t2_dn7)), (((-((locals.var_uc_glkb2 * ((locals.var_etun_dn8 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn8))) / (assign68760_e105337 * assign68760_e105337))) * locals.var_t2) + (assign68760_e105338 * locals.var_t2_dn8)), (((-((locals.var_uc_glkb2 * ((locals.var_etun_dn9 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn9))) / (assign68760_e105337 * assign68760_e105337))) * locals.var_t2) + (assign68760_e105338 * locals.var_t2_dn9)), (((-((locals.var_uc_glkb2 * ((locals.var_etun_dn10 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn10))) / (assign68760_e105337 * assign68760_e105337))) * locals.var_t2) + (assign68760_e105338 * locals.var_t2_dn10)), (((-((locals.var_uc_glkb2 * ((locals.var_etun_dn11 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn11))) / (assign68760_e105337 * assign68760_e105337))) * locals.var_t2) + (assign68760_e105338 * locals.var_t2_dn11)), (((-((locals.var_uc_glkb2 * ((locals.var_etun_dn14 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn14))) / (assign68760_e105337 * assign68760_e105337))) * locals.var_t2) + (assign68760_e105338 * locals.var_t2_dn14)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign68760_e105342;
        locals.var_t3_dn0 = assign68760_e105342_d_n0;
        locals.var_t3_dn2 = assign68760_e105342_d_n2;
        locals.var_t3_dn4 = assign68760_e105342_d_n4;
        locals.var_t3_dn5 = assign68760_e105342_d_n5;
        locals.var_t3_dn6 = assign68760_e105342_d_n6;
        locals.var_t3_dn7 = assign68760_e105342_d_n7;
        locals.var_t3_dn8 = assign68760_e105342_d_n8;
        locals.var_t3_dn9 = assign68760_e105342_d_n9;
        locals.var_t3_dn10 = assign68760_e105342_d_n10;
        locals.var_t3_dn11 = assign68760_e105342_d_n11;
        locals.var_t3_dn14 = assign68760_e105342_d_n14;

        let (assign68770_e105353, assign68770_e105353_d_n0, assign68770_e105353_d_n2, assign68770_e105353_d_n4, assign68770_e105353_d_n5, assign68770_e105353_d_n6, assign68770_e105353_d_n7, assign68770_e105353_d_n8, assign68770_e105353_d_n9, assign68770_e105353_d_n10, assign68770_e105353_d_n11, assign68770_e105353_d_n14,) = {
    if ((locals.var_guard1607 != 0.0) && (locals.var_guard1615 == 0.0)) {
        let assign68770_e105349: f64 = (locals.var_uc_glkb1 * locals.var_weff_nf);
        let assign68770_e105351: f64 = (assign68770_e105349 * locals.var_leff);
        (assign68770_e105351, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign68770_e105353;
        locals.var_t3_dn0 = assign68770_e105353_d_n0;
        locals.var_t3_dn2 = assign68770_e105353_d_n2;
        locals.var_t3_dn4 = assign68770_e105353_d_n4;
        locals.var_t3_dn5 = assign68770_e105353_d_n5;
        locals.var_t3_dn6 = assign68770_e105353_d_n6;
        locals.var_t3_dn7 = assign68770_e105353_d_n7;
        locals.var_t3_dn8 = assign68770_e105353_d_n8;
        locals.var_t3_dn9 = assign68770_e105353_d_n9;
        locals.var_t3_dn10 = assign68770_e105353_d_n10;
        locals.var_t3_dn11 = assign68770_e105353_d_n11;
        locals.var_t3_dn14 = assign68770_e105353_d_n14;

        let (assign68780_e105366, assign68780_e105366_d_n0, assign68780_e105366_d_n2, assign68780_e105366_d_n4, assign68780_e105366_d_n5, assign68780_e105366_d_n6, assign68780_e105366_d_n7, assign68780_e105366_d_n8, assign68780_e105366_d_n9, assign68780_e105366_d_n10, assign68780_e105366_d_n11, assign68780_e105366_d_n14,) = {
    if ((locals.var_guard1607 != 0.0) && (locals.var_guard1615 == 0.0)) {
        let assign68780_e105360: f64 = (locals.var_t3 * locals.var_etun);
        let assign68780_e105362: f64 = (assign68780_e105360 * locals.var_etun);
        let assign68780_e105364: f64 = (assign68780_e105362 * locals.var_t2);
        (assign68780_e105364, ((((((locals.var_t3_dn0 * locals.var_etun) + (locals.var_t3 * locals.var_etun_dn0)) * locals.var_etun) + (assign68780_e105360 * locals.var_etun_dn0)) * locals.var_t2) + (assign68780_e105362 * locals.var_t2_dn0)), ((((((locals.var_t3_dn2 * locals.var_etun) + (locals.var_t3 * locals.var_etun_dn2)) * locals.var_etun) + (assign68780_e105360 * locals.var_etun_dn2)) * locals.var_t2) + (assign68780_e105362 * locals.var_t2_dn2)), ((((((locals.var_t3_dn4 * locals.var_etun) + (locals.var_t3 * locals.var_etun_dn4)) * locals.var_etun) + (assign68780_e105360 * locals.var_etun_dn4)) * locals.var_t2) + (assign68780_e105362 * locals.var_t2_dn4)), ((((((locals.var_t3_dn5 * locals.var_etun) + (locals.var_t3 * locals.var_etun_dn5)) * locals.var_etun) + (assign68780_e105360 * locals.var_etun_dn5)) * locals.var_t2) + (assign68780_e105362 * locals.var_t2_dn5)), ((((((locals.var_t3_dn6 * locals.var_etun) + (locals.var_t3 * locals.var_etun_dn6)) * locals.var_etun) + (assign68780_e105360 * locals.var_etun_dn6)) * locals.var_t2) + (assign68780_e105362 * locals.var_t2_dn6)), ((((((locals.var_t3_dn7 * locals.var_etun) + (locals.var_t3 * locals.var_etun_dn7)) * locals.var_etun) + (assign68780_e105360 * locals.var_etun_dn7)) * locals.var_t2) + (assign68780_e105362 * locals.var_t2_dn7)), ((((((locals.var_t3_dn8 * locals.var_etun) + (locals.var_t3 * locals.var_etun_dn8)) * locals.var_etun) + (assign68780_e105360 * locals.var_etun_dn8)) * locals.var_t2) + (assign68780_e105362 * locals.var_t2_dn8)), ((((((locals.var_t3_dn9 * locals.var_etun) + (locals.var_t3 * locals.var_etun_dn9)) * locals.var_etun) + (assign68780_e105360 * locals.var_etun_dn9)) * locals.var_t2) + (assign68780_e105362 * locals.var_t2_dn9)), ((((((locals.var_t3_dn10 * locals.var_etun) + (locals.var_t3 * locals.var_etun_dn10)) * locals.var_etun) + (assign68780_e105360 * locals.var_etun_dn10)) * locals.var_t2) + (assign68780_e105362 * locals.var_t2_dn10)), ((((((locals.var_t3_dn11 * locals.var_etun) + (locals.var_t3 * locals.var_etun_dn11)) * locals.var_etun) + (assign68780_e105360 * locals.var_etun_dn11)) * locals.var_t2) + (assign68780_e105362 * locals.var_t2_dn11)), ((((((locals.var_t3_dn14 * locals.var_etun) + (locals.var_t3 * locals.var_etun_dn14)) * locals.var_etun) + (assign68780_e105360 * locals.var_etun_dn14)) * locals.var_t2) + (assign68780_e105362 * locals.var_t2_dn14)),)
    } else {
        (locals.var_igb, locals.var_igb_dn0, locals.var_igb_dn2, locals.var_igb_dn4, locals.var_igb_dn5, locals.var_igb_dn6, locals.var_igb_dn7, locals.var_igb_dn8, locals.var_igb_dn9, locals.var_igb_dn10, locals.var_igb_dn11, locals.var_igb_dn14,)
    }
};
        locals.var_igb = assign68780_e105366;
        locals.var_igb_dn0 = assign68780_e105366_d_n0;
        locals.var_igb_dn2 = assign68780_e105366_d_n2;
        locals.var_igb_dn4 = assign68780_e105366_d_n4;
        locals.var_igb_dn5 = assign68780_e105366_d_n5;
        locals.var_igb_dn6 = assign68780_e105366_d_n6;
        locals.var_igb_dn7 = assign68780_e105366_d_n7;
        locals.var_igb_dn8 = assign68780_e105366_d_n8;
        locals.var_igb_dn9 = assign68780_e105366_d_n9;
        locals.var_igb_dn10 = assign68780_e105366_d_n10;
        locals.var_igb_dn11 = assign68780_e105366_d_n11;
        locals.var_igb_dn14 = assign68780_e105366_d_n14;

        let (assign68790_e105370, assign68790_e105370_d_n0, assign68790_e105370_d_n2, assign68790_e105370_d_n4, assign68790_e105370_d_n5, assign68790_e105370_d_n6, assign68790_e105370_d_n7, assign68790_e105370_d_n8, assign68790_e105370_d_n9, assign68790_e105370_d_n10, assign68790_e105370_d_n11, assign68790_e105370_d_n14,) = {
    if (locals.var_guard1607 != 0.0) {
        (locals.var_sqrt_eg, locals.var_sqrt_eg_dn0, locals.var_sqrt_eg_dn2, locals.var_sqrt_eg_dn4, locals.var_sqrt_eg_dn5, locals.var_sqrt_eg_dn6, locals.var_sqrt_eg_dn7, locals.var_sqrt_eg_dn8, locals.var_sqrt_eg_dn9, locals.var_sqrt_eg_dn10, locals.var_sqrt_eg_dn11, locals.var_sqrt_eg_dn14,)
    } else {
        (locals.var_eg12, locals.var_eg12_dn0, locals.var_eg12_dn2, locals.var_eg12_dn4, locals.var_eg12_dn5, locals.var_eg12_dn6, locals.var_eg12_dn7, locals.var_eg12_dn8, locals.var_eg12_dn9, locals.var_eg12_dn10, locals.var_eg12_dn11, locals.var_eg12_dn14,)
    }
};
        locals.var_eg12 = assign68790_e105370;
        locals.var_eg12_dn0 = assign68790_e105370_d_n0;
        locals.var_eg12_dn2 = assign68790_e105370_d_n2;
        locals.var_eg12_dn4 = assign68790_e105370_d_n4;
        locals.var_eg12_dn5 = assign68790_e105370_d_n5;
        locals.var_eg12_dn6 = assign68790_e105370_d_n6;
        locals.var_eg12_dn7 = assign68790_e105370_d_n7;
        locals.var_eg12_dn8 = assign68790_e105370_d_n8;
        locals.var_eg12_dn9 = assign68790_e105370_d_n9;
        locals.var_eg12_dn10 = assign68790_e105370_d_n10;
        locals.var_eg12_dn11 = assign68790_e105370_d_n11;
        locals.var_eg12_dn14 = assign68790_e105370_d_n14;

        let (assign68800_e105376, assign68800_e105376_d_n0, assign68800_e105376_d_n2, assign68800_e105376_d_n4, assign68800_e105376_d_n5, assign68800_e105376_d_n6, assign68800_e105376_d_n7, assign68800_e105376_d_n8, assign68800_e105376_d_n9, assign68800_e105376_d_n10, assign68800_e105376_d_n11, assign68800_e105376_d_n14,) = {
    if (locals.var_guard1607 != 0.0) {
        let assign68800_e105374: f64 = (locals.var_eg * locals.var_eg12);
        (assign68800_e105374, ((locals.var_eg_dn0 * locals.var_eg12) + (locals.var_eg * locals.var_eg12_dn0)), ((locals.var_eg_dn2 * locals.var_eg12) + (locals.var_eg * locals.var_eg12_dn2)), ((locals.var_eg_dn4 * locals.var_eg12) + (locals.var_eg * locals.var_eg12_dn4)), ((locals.var_eg_dn5 * locals.var_eg12) + (locals.var_eg * locals.var_eg12_dn5)), ((locals.var_eg_dn6 * locals.var_eg12) + (locals.var_eg * locals.var_eg12_dn6)), ((locals.var_eg_dn7 * locals.var_eg12) + (locals.var_eg * locals.var_eg12_dn7)), ((locals.var_eg_dn8 * locals.var_eg12) + (locals.var_eg * locals.var_eg12_dn8)), ((locals.var_eg_dn9 * locals.var_eg12) + (locals.var_eg * locals.var_eg12_dn9)), ((locals.var_eg_dn10 * locals.var_eg12) + (locals.var_eg * locals.var_eg12_dn10)), ((locals.var_eg_dn11 * locals.var_eg12) + (locals.var_eg * locals.var_eg12_dn11)), ((locals.var_eg_dn14 * locals.var_eg12) + (locals.var_eg * locals.var_eg12_dn14)),)
    } else {
        (locals.var_eg32, locals.var_eg32_dn0, locals.var_eg32_dn2, locals.var_eg32_dn4, locals.var_eg32_dn5, locals.var_eg32_dn6, locals.var_eg32_dn7, locals.var_eg32_dn8, locals.var_eg32_dn9, locals.var_eg32_dn10, locals.var_eg32_dn11, locals.var_eg32_dn14,)
    }
};
        locals.var_eg32 = assign68800_e105376;
        locals.var_eg32_dn0 = assign68800_e105376_d_n0;
        locals.var_eg32_dn2 = assign68800_e105376_d_n2;
        locals.var_eg32_dn4 = assign68800_e105376_d_n4;
        locals.var_eg32_dn5 = assign68800_e105376_d_n5;
        locals.var_eg32_dn6 = assign68800_e105376_d_n6;
        locals.var_eg32_dn7 = assign68800_e105376_d_n7;
        locals.var_eg32_dn8 = assign68800_e105376_d_n8;
        locals.var_eg32_dn9 = assign68800_e105376_d_n9;
        locals.var_eg32_dn10 = assign68800_e105376_d_n10;
        locals.var_eg32_dn11 = assign68800_e105376_d_n11;
        locals.var_eg32_dn14 = assign68800_e105376_d_n14;

        let (assign68810_e105393, assign68810_e105393_d_n0, assign68810_e105393_d_n2, assign68810_e105393_d_n4, assign68810_e105393_d_n5, assign68810_e105393_d_n6, assign68810_e105393_d_n7, assign68810_e105393_d_n8, assign68810_e105393_d_n9, assign68810_e105393_d_n10, assign68810_e105393_d_n11, assign68810_e105393_d_n14,) = {
    if (locals.var_guard1607 != 0.0) {
        let assign68810_e105380: f64 = (locals.var_uc_fvbs * locals.var_vbsz__blk442);
        let assign68810_e105382: f64 = (assign68810_e105380 - locals.var_vgsz__blk444);
        let assign68810_e105384: f64 = (assign68810_e105382 + locals.var_dvthsc);
        let assign68810_e105386: f64 = (assign68810_e105384 + locals.var_dvthlp);
        let assign68810_e105388: f64 = (assign68810_e105386 - locals.var_uc_fn3);
        let assign68810_e105389: f64 = (-assign68810_e105388);
        let assign68810_e105391: f64 = (assign68810_e105389 / locals.var_tox0);
        (assign68810_e105391, ((-((((locals.var_uc_fvbs * locals.var_vbsz__blk442_dn0) - locals.var_vgsz__blk444_dn0) + locals.var_dvthsc_dn0) + locals.var_dvthlp_dn0)) / locals.var_tox0), ((-((((locals.var_uc_fvbs * locals.var_vbsz__blk442_dn2) - locals.var_vgsz__blk444_dn2) + locals.var_dvthsc_dn2) + locals.var_dvthlp_dn2)) / locals.var_tox0), ((-((((locals.var_uc_fvbs * locals.var_vbsz__blk442_dn4) - locals.var_vgsz__blk444_dn4) + locals.var_dvthsc_dn4) + locals.var_dvthlp_dn4)) / locals.var_tox0), ((-((((locals.var_uc_fvbs * locals.var_vbsz__blk442_dn5) - locals.var_vgsz__blk444_dn5) + locals.var_dvthsc_dn5) + locals.var_dvthlp_dn5)) / locals.var_tox0), ((-((((locals.var_uc_fvbs * locals.var_vbsz__blk442_dn6) - locals.var_vgsz__blk444_dn6) + locals.var_dvthsc_dn6) + locals.var_dvthlp_dn6)) / locals.var_tox0), ((-((((locals.var_uc_fvbs * locals.var_vbsz__blk442_dn7) - locals.var_vgsz__blk444_dn7) + locals.var_dvthsc_dn7) + locals.var_dvthlp_dn7)) / locals.var_tox0), ((-((((locals.var_uc_fvbs * locals.var_vbsz__blk442_dn8) - locals.var_vgsz__blk444_dn8) + locals.var_dvthsc_dn8) + locals.var_dvthlp_dn8)) / locals.var_tox0), ((-((((locals.var_uc_fvbs * locals.var_vbsz__blk442_dn9) - locals.var_vgsz__blk444_dn9) + locals.var_dvthsc_dn9) + locals.var_dvthlp_dn9)) / locals.var_tox0), ((-((((locals.var_uc_fvbs * locals.var_vbsz__blk442_dn10) - locals.var_vgsz__blk444_dn10) + locals.var_dvthsc_dn10) + locals.var_dvthlp_dn10)) / locals.var_tox0), ((-((((locals.var_uc_fvbs * locals.var_vbsz__blk442_dn11) - locals.var_vgsz__blk444_dn11) + locals.var_dvthsc_dn11) + locals.var_dvthlp_dn11)) / locals.var_tox0), ((-((((locals.var_uc_fvbs * locals.var_vbsz__blk442_dn14) - locals.var_vgsz__blk444_dn14) + locals.var_dvthsc_dn14) + locals.var_dvthlp_dn14)) / locals.var_tox0),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign68810_e105393;
        locals.var_t2_dn0 = assign68810_e105393_d_n0;
        locals.var_t2_dn2 = assign68810_e105393_d_n2;
        locals.var_t2_dn4 = assign68810_e105393_d_n4;
        locals.var_t2_dn5 = assign68810_e105393_d_n5;
        locals.var_t2_dn6 = assign68810_e105393_d_n6;
        locals.var_t2_dn7 = assign68810_e105393_d_n7;
        locals.var_t2_dn8 = assign68810_e105393_d_n8;
        locals.var_t2_dn9 = assign68810_e105393_d_n9;
        locals.var_t2_dn10 = assign68810_e105393_d_n10;
        locals.var_t2_dn11 = assign68810_e105393_d_n11;
        locals.var_t2_dn14 = assign68810_e105393_d_n14;

        let (assign68820_e105399, assign68820_e105399_d_n0, assign68820_e105399_d_n2, assign68820_e105399_d_n4, assign68820_e105399_d_n5, assign68820_e105399_d_n6, assign68820_e105399_d_n7, assign68820_e105399_d_n8, assign68820_e105399_d_n9, assign68820_e105399_d_n10, assign68820_e105399_d_n11, assign68820_e105399_d_n14,) = {
    if (locals.var_guard1607 != 0.0) {
        let assign68820_e105397: f64 = (locals.var_t2 * locals.var_t2);
        (assign68820_e105397, ((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)), ((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)), ((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)), ((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)), ((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)), ((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)), ((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)), ((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)), ((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)), ((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)), ((locals.var_t2_dn14 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign68820_e105399;
        locals.var_t0_dn0 = assign68820_e105399_d_n0;
        locals.var_t0_dn2 = assign68820_e105399_d_n2;
        locals.var_t0_dn4 = assign68820_e105399_d_n4;
        locals.var_t0_dn5 = assign68820_e105399_d_n5;
        locals.var_t0_dn6 = assign68820_e105399_d_n6;
        locals.var_t0_dn7 = assign68820_e105399_d_n7;
        locals.var_t0_dn8 = assign68820_e105399_d_n8;
        locals.var_t0_dn9 = assign68820_e105399_d_n9;
        locals.var_t0_dn10 = assign68820_e105399_d_n10;
        locals.var_t0_dn11 = assign68820_e105399_d_n11;
        locals.var_t0_dn14 = assign68820_e105399_d_n14;

        let (assign68830_e105405, assign68830_e105405_d_n0, assign68830_e105405_d_n2, assign68830_e105405_d_n4, assign68830_e105405_d_n5, assign68830_e105405_d_n6, assign68830_e105405_d_n7, assign68830_e105405_d_n8, assign68830_e105405_d_n9, assign68830_e105405_d_n10, assign68830_e105405_d_n11, assign68830_e105405_d_n14,) = {
    if (locals.var_guard1607 != 0.0) {
        let assign68830_e105403: f64 = (locals.var_uc_fn2 * locals.var_eg32);
        (assign68830_e105403, (locals.var_uc_fn2 * locals.var_eg32_dn0), (locals.var_uc_fn2 * locals.var_eg32_dn2), (locals.var_uc_fn2 * locals.var_eg32_dn4), (locals.var_uc_fn2 * locals.var_eg32_dn5), (locals.var_uc_fn2 * locals.var_eg32_dn6), (locals.var_uc_fn2 * locals.var_eg32_dn7), (locals.var_uc_fn2 * locals.var_eg32_dn8), (locals.var_uc_fn2 * locals.var_eg32_dn9), (locals.var_uc_fn2 * locals.var_eg32_dn10), (locals.var_uc_fn2 * locals.var_eg32_dn11), (locals.var_uc_fn2 * locals.var_eg32_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign68830_e105405;
        locals.var_t1_dn0 = assign68830_e105405_d_n0;
        locals.var_t1_dn2 = assign68830_e105405_d_n2;
        locals.var_t1_dn4 = assign68830_e105405_d_n4;
        locals.var_t1_dn5 = assign68830_e105405_d_n5;
        locals.var_t1_dn6 = assign68830_e105405_d_n6;
        locals.var_t1_dn7 = assign68830_e105405_d_n7;
        locals.var_t1_dn8 = assign68830_e105405_d_n8;
        locals.var_t1_dn9 = assign68830_e105405_d_n9;
        locals.var_t1_dn10 = assign68830_e105405_d_n10;
        locals.var_t1_dn11 = assign68830_e105405_d_n11;
        locals.var_t1_dn14 = assign68830_e105405_d_n14;

        let (assign68840_e105412, assign68840_e105412_d_n0, assign68840_e105412_d_n2, assign68840_e105412_d_n4, assign68840_e105412_d_n5, assign68840_e105412_d_n6, assign68840_e105412_d_n7, assign68840_e105412_d_n8, assign68840_e105412_d_n9, assign68840_e105412_d_n10, assign68840_e105412_d_n11, assign68840_e105412_d_n14,) = {
    if (locals.var_guard1607 != 0.0) {
        let assign68840_e105408: f64 = (-locals.var_t1);
        let assign68840_e105410: f64 = (assign68840_e105408 / locals.var_t2);
        (assign68840_e105410, ((((-locals.var_t1_dn0) * locals.var_t2) - (assign68840_e105408 * locals.var_t2_dn0)) / (locals.var_t2 * locals.var_t2)), ((((-locals.var_t1_dn2) * locals.var_t2) - (assign68840_e105408 * locals.var_t2_dn2)) / (locals.var_t2 * locals.var_t2)), ((((-locals.var_t1_dn4) * locals.var_t2) - (assign68840_e105408 * locals.var_t2_dn4)) / (locals.var_t2 * locals.var_t2)), ((((-locals.var_t1_dn5) * locals.var_t2) - (assign68840_e105408 * locals.var_t2_dn5)) / (locals.var_t2 * locals.var_t2)), ((((-locals.var_t1_dn6) * locals.var_t2) - (assign68840_e105408 * locals.var_t2_dn6)) / (locals.var_t2 * locals.var_t2)), ((((-locals.var_t1_dn7) * locals.var_t2) - (assign68840_e105408 * locals.var_t2_dn7)) / (locals.var_t2 * locals.var_t2)), ((((-locals.var_t1_dn8) * locals.var_t2) - (assign68840_e105408 * locals.var_t2_dn8)) / (locals.var_t2 * locals.var_t2)), ((((-locals.var_t1_dn9) * locals.var_t2) - (assign68840_e105408 * locals.var_t2_dn9)) / (locals.var_t2 * locals.var_t2)), ((((-locals.var_t1_dn10) * locals.var_t2) - (assign68840_e105408 * locals.var_t2_dn10)) / (locals.var_t2 * locals.var_t2)), ((((-locals.var_t1_dn11) * locals.var_t2) - (assign68840_e105408 * locals.var_t2_dn11)) / (locals.var_t2 * locals.var_t2)), ((((-locals.var_t1_dn14) * locals.var_t2) - (assign68840_e105408 * locals.var_t2_dn14)) / (locals.var_t2 * locals.var_t2)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign68840_e105412;
        locals.var_t3_dn0 = assign68840_e105412_d_n0;
        locals.var_t3_dn2 = assign68840_e105412_d_n2;
        locals.var_t3_dn4 = assign68840_e105412_d_n4;
        locals.var_t3_dn5 = assign68840_e105412_d_n5;
        locals.var_t3_dn6 = assign68840_e105412_d_n6;
        locals.var_t3_dn7 = assign68840_e105412_d_n7;
        locals.var_t3_dn8 = assign68840_e105412_d_n8;
        locals.var_t3_dn9 = assign68840_e105412_d_n9;
        locals.var_t3_dn10 = assign68840_e105412_d_n10;
        locals.var_t3_dn11 = assign68840_e105412_d_n11;
        locals.var_t3_dn14 = assign68840_e105412_d_n14;

        let assign68850_e105415: f64 = (-34.0);
        let assign68850_e105416: f64 = if locals.var_t3 < assign68850_e105415 { 1.0 } else { 0.0 };
        locals.var_guard1616 = assign68850_e105416;

        let (assign68860_e105422, assign68860_e105422_d_n0, assign68860_e105422_d_n2, assign68860_e105422_d_n4, assign68860_e105422_d_n5, assign68860_e105422_d_n6, assign68860_e105422_d_n7, assign68860_e105422_d_n8, assign68860_e105422_d_n9, assign68860_e105422_d_n10, assign68860_e105422_d_n11, assign68860_e105422_d_n14,) = {
    if ((locals.var_guard1607 != 0.0) && (locals.var_guard1616 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign68860_e105422;
        locals.var_t5_dn0 = assign68860_e105422_d_n0;
        locals.var_t5_dn2 = assign68860_e105422_d_n2;
        locals.var_t5_dn4 = assign68860_e105422_d_n4;
        locals.var_t5_dn5 = assign68860_e105422_d_n5;
        locals.var_t5_dn6 = assign68860_e105422_d_n6;
        locals.var_t5_dn7 = assign68860_e105422_d_n7;
        locals.var_t5_dn8 = assign68860_e105422_d_n8;
        locals.var_t5_dn9 = assign68860_e105422_d_n9;
        locals.var_t5_dn10 = assign68860_e105422_d_n10;
        locals.var_t5_dn11 = assign68860_e105422_d_n11;
        locals.var_t5_dn14 = assign68860_e105422_d_n14;

        let (assign68870_e105430, assign68870_e105430_d_n0, assign68870_e105430_d_n2, assign68870_e105430_d_n4, assign68870_e105430_d_n5, assign68870_e105430_d_n6, assign68870_e105430_d_n7, assign68870_e105430_d_n8, assign68870_e105430_d_n9, assign68870_e105430_d_n10, assign68870_e105430_d_n11, assign68870_e105430_d_n14,) = {
    if ((locals.var_guard1607 != 0.0) && (locals.var_guard1616 == 0.0)) {
        let assign68870_e105428: f64 = (locals.var_t3).exp();
        (assign68870_e105428, (assign68870_e105428 * locals.var_t3_dn0), (assign68870_e105428 * locals.var_t3_dn2), (assign68870_e105428 * locals.var_t3_dn4), (assign68870_e105428 * locals.var_t3_dn5), (assign68870_e105428 * locals.var_t3_dn6), (assign68870_e105428 * locals.var_t3_dn7), (assign68870_e105428 * locals.var_t3_dn8), (assign68870_e105428 * locals.var_t3_dn9), (assign68870_e105428 * locals.var_t3_dn10), (assign68870_e105428 * locals.var_t3_dn11), (assign68870_e105428 * locals.var_t3_dn14),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign68870_e105430;
        locals.var_t5_dn0 = assign68870_e105430_d_n0;
        locals.var_t5_dn2 = assign68870_e105430_d_n2;
        locals.var_t5_dn4 = assign68870_e105430_d_n4;
        locals.var_t5_dn5 = assign68870_e105430_d_n5;
        locals.var_t5_dn6 = assign68870_e105430_d_n6;
        locals.var_t5_dn7 = assign68870_e105430_d_n7;
        locals.var_t5_dn8 = assign68870_e105430_d_n8;
        locals.var_t5_dn9 = assign68870_e105430_d_n9;
        locals.var_t5_dn10 = assign68870_e105430_d_n10;
        locals.var_t5_dn11 = assign68870_e105430_d_n11;
        locals.var_t5_dn14 = assign68870_e105430_d_n14;

    }

    pub(super) fn stamp_transient_block_246(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign68880_e105442, assign68880_e105442_d_n0, assign68880_e105442_d_n2, assign68880_e105442_d_n4, assign68880_e105442_d_n5, assign68880_e105442_d_n6, assign68880_e105442_d_n7, assign68880_e105442_d_n8, assign68880_e105442_d_n9, assign68880_e105442_d_n10, assign68880_e105442_d_n11, assign68880_e105442_d_n14,) = {
    if (locals.var_guard1607 != 0.0) {
        let assign68880_e105434: f64 = (1.6021918e-19 * locals.var_uc_fn1);
        let assign68880_e105436: f64 = (assign68880_e105434 * locals.var_weff_nf);
        let assign68880_e105438: f64 = (assign68880_e105436 * locals.var_lgate);
        let assign68880_e105440: f64 = (assign68880_e105438 / locals.var_eg12);
        (assign68880_e105440, (-((assign68880_e105438 * locals.var_eg12_dn0) / (locals.var_eg12 * locals.var_eg12))), (-((assign68880_e105438 * locals.var_eg12_dn2) / (locals.var_eg12 * locals.var_eg12))), (-((assign68880_e105438 * locals.var_eg12_dn4) / (locals.var_eg12 * locals.var_eg12))), (-((assign68880_e105438 * locals.var_eg12_dn5) / (locals.var_eg12 * locals.var_eg12))), (-((assign68880_e105438 * locals.var_eg12_dn6) / (locals.var_eg12 * locals.var_eg12))), (-((assign68880_e105438 * locals.var_eg12_dn7) / (locals.var_eg12 * locals.var_eg12))), (-((assign68880_e105438 * locals.var_eg12_dn8) / (locals.var_eg12 * locals.var_eg12))), (-((assign68880_e105438 * locals.var_eg12_dn9) / (locals.var_eg12 * locals.var_eg12))), (-((assign68880_e105438 * locals.var_eg12_dn10) / (locals.var_eg12 * locals.var_eg12))), (-((assign68880_e105438 * locals.var_eg12_dn11) / (locals.var_eg12 * locals.var_eg12))), (-((assign68880_e105438 * locals.var_eg12_dn14) / (locals.var_eg12 * locals.var_eg12))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign68880_e105442;
        locals.var_t4_dn0 = assign68880_e105442_d_n0;
        locals.var_t4_dn2 = assign68880_e105442_d_n2;
        locals.var_t4_dn4 = assign68880_e105442_d_n4;
        locals.var_t4_dn5 = assign68880_e105442_d_n5;
        locals.var_t4_dn6 = assign68880_e105442_d_n6;
        locals.var_t4_dn7 = assign68880_e105442_d_n7;
        locals.var_t4_dn8 = assign68880_e105442_d_n8;
        locals.var_t4_dn9 = assign68880_e105442_d_n9;
        locals.var_t4_dn10 = assign68880_e105442_d_n10;
        locals.var_t4_dn11 = assign68880_e105442_d_n11;
        locals.var_t4_dn14 = assign68880_e105442_d_n14;

        let assign68890_e105445: f64 = (2.0 * locals.var_t2);
        let assign68890_e105447: f64 = (assign68890_e105445 + locals.var_t1);
        let assign68890_e105449: f64 = if assign68890_e105447 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1617 = assign68890_e105449;

        let (assign68900_e105463, assign68900_e105463_d_n0, assign68900_e105463_d_n2, assign68900_e105463_d_n4, assign68900_e105463_d_n5, assign68900_e105463_d_n6, assign68900_e105463_d_n7, assign68900_e105463_d_n8, assign68900_e105463_d_n9, assign68900_e105463_d_n10, assign68900_e105463_d_n11, assign68900_e105463_d_n14,) = {
    if ((locals.var_guard1607 != 0.0) && (locals.var_guard1617 != 0.0)) {
        let assign68900_e105455: f64 = (0.25 * locals.var_t4);
        let assign68900_e105457: f64 = (assign68900_e105455 * locals.var_t1);
        let assign68900_e105459: f64 = (assign68900_e105457 * locals.var_t1);
        let assign68900_e105461: f64 = (assign68900_e105459 * 7.38905609893065);
        (assign68900_e105461, ((((((0.25 * locals.var_t4_dn0) * locals.var_t1) + (assign68900_e105455 * locals.var_t1_dn0)) * locals.var_t1) + (assign68900_e105457 * locals.var_t1_dn0)) * 7.38905609893065), ((((((0.25 * locals.var_t4_dn2) * locals.var_t1) + (assign68900_e105455 * locals.var_t1_dn2)) * locals.var_t1) + (assign68900_e105457 * locals.var_t1_dn2)) * 7.38905609893065), ((((((0.25 * locals.var_t4_dn4) * locals.var_t1) + (assign68900_e105455 * locals.var_t1_dn4)) * locals.var_t1) + (assign68900_e105457 * locals.var_t1_dn4)) * 7.38905609893065), ((((((0.25 * locals.var_t4_dn5) * locals.var_t1) + (assign68900_e105455 * locals.var_t1_dn5)) * locals.var_t1) + (assign68900_e105457 * locals.var_t1_dn5)) * 7.38905609893065), ((((((0.25 * locals.var_t4_dn6) * locals.var_t1) + (assign68900_e105455 * locals.var_t1_dn6)) * locals.var_t1) + (assign68900_e105457 * locals.var_t1_dn6)) * 7.38905609893065), ((((((0.25 * locals.var_t4_dn7) * locals.var_t1) + (assign68900_e105455 * locals.var_t1_dn7)) * locals.var_t1) + (assign68900_e105457 * locals.var_t1_dn7)) * 7.38905609893065), ((((((0.25 * locals.var_t4_dn8) * locals.var_t1) + (assign68900_e105455 * locals.var_t1_dn8)) * locals.var_t1) + (assign68900_e105457 * locals.var_t1_dn8)) * 7.38905609893065), ((((((0.25 * locals.var_t4_dn9) * locals.var_t1) + (assign68900_e105455 * locals.var_t1_dn9)) * locals.var_t1) + (assign68900_e105457 * locals.var_t1_dn9)) * 7.38905609893065), ((((((0.25 * locals.var_t4_dn10) * locals.var_t1) + (assign68900_e105455 * locals.var_t1_dn10)) * locals.var_t1) + (assign68900_e105457 * locals.var_t1_dn10)) * 7.38905609893065), ((((((0.25 * locals.var_t4_dn11) * locals.var_t1) + (assign68900_e105455 * locals.var_t1_dn11)) * locals.var_t1) + (assign68900_e105457 * locals.var_t1_dn11)) * 7.38905609893065), ((((((0.25 * locals.var_t4_dn14) * locals.var_t1) + (assign68900_e105455 * locals.var_t1_dn14)) * locals.var_t1) + (assign68900_e105457 * locals.var_t1_dn14)) * 7.38905609893065),)
    } else {
        (locals.var_ifn, locals.var_ifn_dn0, locals.var_ifn_dn2, locals.var_ifn_dn4, locals.var_ifn_dn5, locals.var_ifn_dn6, locals.var_ifn_dn7, locals.var_ifn_dn8, locals.var_ifn_dn9, locals.var_ifn_dn10, locals.var_ifn_dn11, locals.var_ifn_dn14,)
    }
};
        locals.var_ifn = assign68900_e105463;
        locals.var_ifn_dn0 = assign68900_e105463_d_n0;
        locals.var_ifn_dn2 = assign68900_e105463_d_n2;
        locals.var_ifn_dn4 = assign68900_e105463_d_n4;
        locals.var_ifn_dn5 = assign68900_e105463_d_n5;
        locals.var_ifn_dn6 = assign68900_e105463_d_n6;
        locals.var_ifn_dn7 = assign68900_e105463_d_n7;
        locals.var_ifn_dn8 = assign68900_e105463_d_n8;
        locals.var_ifn_dn9 = assign68900_e105463_d_n9;
        locals.var_ifn_dn10 = assign68900_e105463_d_n10;
        locals.var_ifn_dn11 = assign68900_e105463_d_n11;
        locals.var_ifn_dn14 = assign68900_e105463_d_n14;

        let (assign68910_e105474, assign68910_e105474_d_n0, assign68910_e105474_d_n2, assign68910_e105474_d_n4, assign68910_e105474_d_n5, assign68910_e105474_d_n6, assign68910_e105474_d_n7, assign68910_e105474_d_n8, assign68910_e105474_d_n9, assign68910_e105474_d_n10, assign68910_e105474_d_n11, assign68910_e105474_d_n14,) = {
    if ((locals.var_guard1607 != 0.0) && (locals.var_guard1617 == 0.0)) {
        let assign68910_e105470: f64 = (locals.var_t4 * locals.var_t0);
        let assign68910_e105472: f64 = (assign68910_e105470 * locals.var_t5);
        (assign68910_e105472, ((((locals.var_t4_dn0 * locals.var_t0) + (locals.var_t4 * locals.var_t0_dn0)) * locals.var_t5) + (assign68910_e105470 * locals.var_t5_dn0)), ((((locals.var_t4_dn2 * locals.var_t0) + (locals.var_t4 * locals.var_t0_dn2)) * locals.var_t5) + (assign68910_e105470 * locals.var_t5_dn2)), ((((locals.var_t4_dn4 * locals.var_t0) + (locals.var_t4 * locals.var_t0_dn4)) * locals.var_t5) + (assign68910_e105470 * locals.var_t5_dn4)), ((((locals.var_t4_dn5 * locals.var_t0) + (locals.var_t4 * locals.var_t0_dn5)) * locals.var_t5) + (assign68910_e105470 * locals.var_t5_dn5)), ((((locals.var_t4_dn6 * locals.var_t0) + (locals.var_t4 * locals.var_t0_dn6)) * locals.var_t5) + (assign68910_e105470 * locals.var_t5_dn6)), ((((locals.var_t4_dn7 * locals.var_t0) + (locals.var_t4 * locals.var_t0_dn7)) * locals.var_t5) + (assign68910_e105470 * locals.var_t5_dn7)), ((((locals.var_t4_dn8 * locals.var_t0) + (locals.var_t4 * locals.var_t0_dn8)) * locals.var_t5) + (assign68910_e105470 * locals.var_t5_dn8)), ((((locals.var_t4_dn9 * locals.var_t0) + (locals.var_t4 * locals.var_t0_dn9)) * locals.var_t5) + (assign68910_e105470 * locals.var_t5_dn9)), ((((locals.var_t4_dn10 * locals.var_t0) + (locals.var_t4 * locals.var_t0_dn10)) * locals.var_t5) + (assign68910_e105470 * locals.var_t5_dn10)), ((((locals.var_t4_dn11 * locals.var_t0) + (locals.var_t4 * locals.var_t0_dn11)) * locals.var_t5) + (assign68910_e105470 * locals.var_t5_dn11)), ((((locals.var_t4_dn14 * locals.var_t0) + (locals.var_t4 * locals.var_t0_dn14)) * locals.var_t5) + (assign68910_e105470 * locals.var_t5_dn14)),)
    } else {
        (locals.var_ifn, locals.var_ifn_dn0, locals.var_ifn_dn2, locals.var_ifn_dn4, locals.var_ifn_dn5, locals.var_ifn_dn6, locals.var_ifn_dn7, locals.var_ifn_dn8, locals.var_ifn_dn9, locals.var_ifn_dn10, locals.var_ifn_dn11, locals.var_ifn_dn14,)
    }
};
        locals.var_ifn = assign68910_e105474;
        locals.var_ifn_dn0 = assign68910_e105474_d_n0;
        locals.var_ifn_dn2 = assign68910_e105474_d_n2;
        locals.var_ifn_dn4 = assign68910_e105474_d_n4;
        locals.var_ifn_dn5 = assign68910_e105474_d_n5;
        locals.var_ifn_dn6 = assign68910_e105474_d_n6;
        locals.var_ifn_dn7 = assign68910_e105474_d_n7;
        locals.var_ifn_dn8 = assign68910_e105474_d_n8;
        locals.var_ifn_dn9 = assign68910_e105474_d_n9;
        locals.var_ifn_dn10 = assign68910_e105474_d_n10;
        locals.var_ifn_dn11 = assign68910_e105474_d_n11;
        locals.var_ifn_dn14 = assign68910_e105474_d_n14;

        let (assign68920_e105480, assign68920_e105480_d_n0, assign68920_e105480_d_n2, assign68920_e105480_d_n4, assign68920_e105480_d_n5, assign68920_e105480_d_n6, assign68920_e105480_d_n7, assign68920_e105480_d_n8, assign68920_e105480_d_n9, assign68920_e105480_d_n10, assign68920_e105480_d_n11, assign68920_e105480_d_n14,) = {
    if (locals.var_guard1607 != 0.0) {
        let assign68920_e105478: f64 = (locals.var_igb - locals.var_ifn);
        (assign68920_e105478, (locals.var_igb_dn0 - locals.var_ifn_dn0), (locals.var_igb_dn2 - locals.var_ifn_dn2), (locals.var_igb_dn4 - locals.var_ifn_dn4), (locals.var_igb_dn5 - locals.var_ifn_dn5), (locals.var_igb_dn6 - locals.var_ifn_dn6), (locals.var_igb_dn7 - locals.var_ifn_dn7), (locals.var_igb_dn8 - locals.var_ifn_dn8), (locals.var_igb_dn9 - locals.var_ifn_dn9), (locals.var_igb_dn10 - locals.var_ifn_dn10), (locals.var_igb_dn11 - locals.var_ifn_dn11), (locals.var_igb_dn14 - locals.var_ifn_dn14),)
    } else {
        (locals.var_igb, locals.var_igb_dn0, locals.var_igb_dn2, locals.var_igb_dn4, locals.var_igb_dn5, locals.var_igb_dn6, locals.var_igb_dn7, locals.var_igb_dn8, locals.var_igb_dn9, locals.var_igb_dn10, locals.var_igb_dn11, locals.var_igb_dn14,)
    }
};
        locals.var_igb = assign68920_e105480;
        locals.var_igb_dn0 = assign68920_e105480_d_n0;
        locals.var_igb_dn2 = assign68920_e105480_d_n2;
        locals.var_igb_dn4 = assign68920_e105480_d_n4;
        locals.var_igb_dn5 = assign68920_e105480_d_n5;
        locals.var_igb_dn6 = assign68920_e105480_d_n6;
        locals.var_igb_dn7 = assign68920_e105480_d_n7;
        locals.var_igb_dn8 = assign68920_e105480_d_n8;
        locals.var_igb_dn9 = assign68920_e105480_d_n9;
        locals.var_igb_dn10 = assign68920_e105480_d_n10;
        locals.var_igb_dn11 = assign68920_e105480_d_n11;
        locals.var_igb_dn14 = assign68920_e105480_d_n14;

        let assign68930_e105483: f64 = if p.p25 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1618 = assign68930_e105483;

        let (assign68940_e105495, assign68940_e105495_d_n0, assign68940_e105495_d_n2, assign68940_e105495_d_n4, assign68940_e105495_d_n5, assign68940_e105495_d_n6, assign68940_e105495_d_n7, assign68940_e105495_d_n8, assign68940_e105495_d_n9, assign68940_e105495_d_n10, assign68940_e105495_d_n11, assign68940_e105495_d_n14,) = {
    if (locals.var_guard1618 != 0.0) {
        let assign68940_e105489: f64 = (100.0 * locals.var_vds);
        let assign68940_e105490: f64 = (1.0 - assign68940_e105489);
        let assign68940_e105491: f64 = (locals.var_vds * assign68940_e105490);
        let assign68940_e105493: f64 = (assign68940_e105491 - 1e-5);
        (assign68940_e105493, ((locals.var_vds_dn0 * assign68940_e105490) + (locals.var_vds * (-(100.0 * locals.var_vds_dn0)))), ((locals.var_vds_dn2 * assign68940_e105490) + (locals.var_vds * (-(100.0 * locals.var_vds_dn2)))), ((locals.var_vds_dn4 * assign68940_e105490) + (locals.var_vds * (-(100.0 * locals.var_vds_dn4)))), ((locals.var_vds_dn5 * assign68940_e105490) + (locals.var_vds * (-(100.0 * locals.var_vds_dn5)))), ((locals.var_vds_dn6 * assign68940_e105490) + (locals.var_vds * (-(100.0 * locals.var_vds_dn6)))), ((locals.var_vds_dn7 * assign68940_e105490) + (locals.var_vds * (-(100.0 * locals.var_vds_dn7)))), ((locals.var_vds_dn8 * assign68940_e105490) + (locals.var_vds * (-(100.0 * locals.var_vds_dn8)))), ((locals.var_vds_dn9 * assign68940_e105490) + (locals.var_vds * (-(100.0 * locals.var_vds_dn9)))), ((locals.var_vds_dn10 * assign68940_e105490) + (locals.var_vds * (-(100.0 * locals.var_vds_dn10)))), ((locals.var_vds_dn11 * assign68940_e105490) + (locals.var_vds * (-(100.0 * locals.var_vds_dn11)))), ((locals.var_vds_dn14 * assign68940_e105490) + (locals.var_vds * (-(100.0 * locals.var_vds_dn14)))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign68940_e105495;
        locals.var_t1_dn0 = assign68940_e105495_d_n0;
        locals.var_t1_dn2 = assign68940_e105495_d_n2;
        locals.var_t1_dn4 = assign68940_e105495_d_n4;
        locals.var_t1_dn5 = assign68940_e105495_d_n5;
        locals.var_t1_dn6 = assign68940_e105495_d_n6;
        locals.var_t1_dn7 = assign68940_e105495_d_n7;
        locals.var_t1_dn8 = assign68940_e105495_d_n8;
        locals.var_t1_dn9 = assign68940_e105495_d_n9;
        locals.var_t1_dn10 = assign68940_e105495_d_n10;
        locals.var_t1_dn11 = assign68940_e105495_d_n11;
        locals.var_t1_dn14 = assign68940_e105495_d_n14;

        let (assign68950_e105508, assign68950_e105508_d_n0, assign68950_e105508_d_n2, assign68950_e105508_d_n4, assign68950_e105508_d_n5, assign68950_e105508_d_n6, assign68950_e105508_d_n7, assign68950_e105508_d_n8, assign68950_e105508_d_n9, assign68950_e105508_d_n10, assign68950_e105508_d_n11, assign68950_e105508_d_n14,) = {
    if (locals.var_guard1618 != 0.0) {
        let assign68950_e105499: f64 = (locals.var_t1 * locals.var_t1);
        let assign68950_e105502: f64 = (4.0 * 1e-5);
        let assign68950_e105504: f64 = (assign68950_e105502 * locals.var_vds);
        let assign68950_e105505: f64 = (assign68950_e105499 + assign68950_e105504);
        let assign68950_e105506: f64 = (assign68950_e105505).sqrt();
        (assign68950_e105506, ((((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) + (assign68950_e105502 * locals.var_vds_dn0)) / (2.0 * assign68950_e105506)), ((((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) + (assign68950_e105502 * locals.var_vds_dn2)) / (2.0 * assign68950_e105506)), ((((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) + (assign68950_e105502 * locals.var_vds_dn4)) / (2.0 * assign68950_e105506)), ((((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) + (assign68950_e105502 * locals.var_vds_dn5)) / (2.0 * assign68950_e105506)), ((((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) + (assign68950_e105502 * locals.var_vds_dn6)) / (2.0 * assign68950_e105506)), ((((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) + (assign68950_e105502 * locals.var_vds_dn7)) / (2.0 * assign68950_e105506)), ((((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) + (assign68950_e105502 * locals.var_vds_dn8)) / (2.0 * assign68950_e105506)), ((((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) + (assign68950_e105502 * locals.var_vds_dn9)) / (2.0 * assign68950_e105506)), ((((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) + (assign68950_e105502 * locals.var_vds_dn10)) / (2.0 * assign68950_e105506)), ((((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) + (assign68950_e105502 * locals.var_vds_dn11)) / (2.0 * assign68950_e105506)), ((((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14)) + (assign68950_e105502 * locals.var_vds_dn14)) / (2.0 * assign68950_e105506)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign68950_e105508;
        locals.var_t2_dn0 = assign68950_e105508_d_n0;
        locals.var_t2_dn2 = assign68950_e105508_d_n2;
        locals.var_t2_dn4 = assign68950_e105508_d_n4;
        locals.var_t2_dn5 = assign68950_e105508_d_n5;
        locals.var_t2_dn6 = assign68950_e105508_d_n6;
        locals.var_t2_dn7 = assign68950_e105508_d_n7;
        locals.var_t2_dn8 = assign68950_e105508_d_n8;
        locals.var_t2_dn9 = assign68950_e105508_d_n9;
        locals.var_t2_dn10 = assign68950_e105508_d_n10;
        locals.var_t2_dn11 = assign68950_e105508_d_n11;
        locals.var_t2_dn14 = assign68950_e105508_d_n14;

        let (assign68960_e105518, assign68960_e105518_d_n0, assign68960_e105518_d_n2, assign68960_e105518_d_n4, assign68960_e105518_d_n5, assign68960_e105518_d_n6, assign68960_e105518_d_n7, assign68960_e105518_d_n8, assign68960_e105518_d_n9, assign68960_e105518_d_n10, assign68960_e105518_d_n11, assign68960_e105518_d_n14,) = {
    if (locals.var_guard1618 != 0.0) {
        let assign68960_e105514: f64 = (locals.var_t1 + locals.var_t2);
        let assign68960_e105515: f64 = (0.5 * assign68960_e105514);
        let assign68960_e105516: f64 = (locals.var_vds - assign68960_e105515);
        (assign68960_e105516, (locals.var_vds_dn0 - (0.5 * (locals.var_t1_dn0 + locals.var_t2_dn0))), (locals.var_vds_dn2 - (0.5 * (locals.var_t1_dn2 + locals.var_t2_dn2))), (locals.var_vds_dn4 - (0.5 * (locals.var_t1_dn4 + locals.var_t2_dn4))), (locals.var_vds_dn5 - (0.5 * (locals.var_t1_dn5 + locals.var_t2_dn5))), (locals.var_vds_dn6 - (0.5 * (locals.var_t1_dn6 + locals.var_t2_dn6))), (locals.var_vds_dn7 - (0.5 * (locals.var_t1_dn7 + locals.var_t2_dn7))), (locals.var_vds_dn8 - (0.5 * (locals.var_t1_dn8 + locals.var_t2_dn8))), (locals.var_vds_dn9 - (0.5 * (locals.var_t1_dn9 + locals.var_t2_dn9))), (locals.var_vds_dn10 - (0.5 * (locals.var_t1_dn10 + locals.var_t2_dn10))), (locals.var_vds_dn11 - (0.5 * (locals.var_t1_dn11 + locals.var_t2_dn11))), (locals.var_vds_dn14 - (0.5 * (locals.var_t1_dn14 + locals.var_t2_dn14))),)
    } else {
        (locals.var_vdsp, locals.var_vdsp_dn0, locals.var_vdsp_dn2, locals.var_vdsp_dn4, locals.var_vdsp_dn5, locals.var_vdsp_dn6, locals.var_vdsp_dn7, locals.var_vdsp_dn8, locals.var_vdsp_dn9, locals.var_vdsp_dn10, locals.var_vdsp_dn11, locals.var_vdsp_dn14,)
    }
};
        locals.var_vdsp = assign68960_e105518;
        locals.var_vdsp_dn0 = assign68960_e105518_d_n0;
        locals.var_vdsp_dn2 = assign68960_e105518_d_n2;
        locals.var_vdsp_dn4 = assign68960_e105518_d_n4;
        locals.var_vdsp_dn5 = assign68960_e105518_d_n5;
        locals.var_vdsp_dn6 = assign68960_e105518_d_n6;
        locals.var_vdsp_dn7 = assign68960_e105518_d_n7;
        locals.var_vdsp_dn8 = assign68960_e105518_d_n8;
        locals.var_vdsp_dn9 = assign68960_e105518_d_n9;
        locals.var_vdsp_dn10 = assign68960_e105518_d_n10;
        locals.var_vdsp_dn11 = assign68960_e105518_d_n11;
        locals.var_vdsp_dn14 = assign68960_e105518_d_n14;

        let assign68970_e105521: f64 = if p.p25 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1619 = assign68970_e105521;

        let (assign68980_e105525, assign68980_e105525_d_n0, assign68980_e105525_d_n2, assign68980_e105525_d_n4, assign68980_e105525_d_n5, assign68980_e105525_d_n6, assign68980_e105525_d_n7, assign68980_e105525_d_n8, assign68980_e105525_d_n9, assign68980_e105525_d_n10, assign68980_e105525_d_n11, assign68980_e105525_d_n14,) = {
    if (locals.var_guard1619 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_igidl, locals.var_igidl_dn0, locals.var_igidl_dn2, locals.var_igidl_dn4, locals.var_igidl_dn5, locals.var_igidl_dn6, locals.var_igidl_dn7, locals.var_igidl_dn8, locals.var_igidl_dn9, locals.var_igidl_dn10, locals.var_igidl_dn11, locals.var_igidl_dn14,)
    }
};
        locals.var_igidl = assign68980_e105525;
        locals.var_igidl_dn0 = assign68980_e105525_d_n0;
        locals.var_igidl_dn2 = assign68980_e105525_d_n2;
        locals.var_igidl_dn4 = assign68980_e105525_d_n4;
        locals.var_igidl_dn5 = assign68980_e105525_d_n5;
        locals.var_igidl_dn6 = assign68980_e105525_d_n6;
        locals.var_igidl_dn7 = assign68980_e105525_d_n7;
        locals.var_igidl_dn8 = assign68980_e105525_d_n8;
        locals.var_igidl_dn9 = assign68980_e105525_d_n9;
        locals.var_igidl_dn10 = assign68980_e105525_d_n10;
        locals.var_igidl_dn11 = assign68980_e105525_d_n11;
        locals.var_igidl_dn14 = assign68980_e105525_d_n14;

        let (assign68990_e105542, assign68990_e105542_d_n0, assign68990_e105542_d_n2, assign68990_e105542_d_n4, assign68990_e105542_d_n5, assign68990_e105542_d_n6, assign68990_e105542_d_n7, assign68990_e105542_d_n8, assign68990_e105542_d_n9, assign68990_e105542_d_n10, assign68990_e105542_d_n11, assign68990_e105542_d_n14,) = {
    if (locals.var_guard1619 == 0.0) {
        let assign68990_e105531: f64 = (locals.var_vdsp + p.p243);
        let assign68990_e105532: f64 = (p.p242 * assign68990_e105531);
        let assign68990_e105534: f64 = (assign68990_e105532 - locals.var_vgs);
        let assign68990_e105537: f64 = (locals.var_dvthsc + locals.var_dvthlp);
        let assign68990_e105539: f64 = (assign68990_e105537 * p.p244);
        let assign68990_e105540: f64 = (assign68990_e105534 + assign68990_e105539);
        (assign68990_e105540, ((p.p242 * locals.var_vdsp_dn0) + ((locals.var_dvthsc_dn0 + locals.var_dvthlp_dn0) * p.p244)), ((p.p242 * locals.var_vdsp_dn2) + ((locals.var_dvthsc_dn2 + locals.var_dvthlp_dn2) * p.p244)), ((p.p242 * locals.var_vdsp_dn4) + ((locals.var_dvthsc_dn4 + locals.var_dvthlp_dn4) * p.p244)), ((p.p242 * locals.var_vdsp_dn5) + ((locals.var_dvthsc_dn5 + locals.var_dvthlp_dn5) * p.p244)), (((p.p242 * locals.var_vdsp_dn6) - locals.var_vgs_dn6) + ((locals.var_dvthsc_dn6 + locals.var_dvthlp_dn6) * p.p244)), (((p.p242 * locals.var_vdsp_dn7) - locals.var_vgs_dn7) + ((locals.var_dvthsc_dn7 + locals.var_dvthlp_dn7) * p.p244)), (((p.p242 * locals.var_vdsp_dn8) - locals.var_vgs_dn8) + ((locals.var_dvthsc_dn8 + locals.var_dvthlp_dn8) * p.p244)), ((p.p242 * locals.var_vdsp_dn9) + ((locals.var_dvthsc_dn9 + locals.var_dvthlp_dn9) * p.p244)), ((p.p242 * locals.var_vdsp_dn10) + ((locals.var_dvthsc_dn10 + locals.var_dvthlp_dn10) * p.p244)), ((p.p242 * locals.var_vdsp_dn11) + ((locals.var_dvthsc_dn11 + locals.var_dvthlp_dn11) * p.p244)), ((p.p242 * locals.var_vdsp_dn14) + ((locals.var_dvthsc_dn14 + locals.var_dvthlp_dn14) * p.p244)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign68990_e105542;
        locals.var_t1_dn0 = assign68990_e105542_d_n0;
        locals.var_t1_dn2 = assign68990_e105542_d_n2;
        locals.var_t1_dn4 = assign68990_e105542_d_n4;
        locals.var_t1_dn5 = assign68990_e105542_d_n5;
        locals.var_t1_dn6 = assign68990_e105542_d_n6;
        locals.var_t1_dn7 = assign68990_e105542_d_n7;
        locals.var_t1_dn8 = assign68990_e105542_d_n8;
        locals.var_t1_dn9 = assign68990_e105542_d_n9;
        locals.var_t1_dn10 = assign68990_e105542_d_n10;
        locals.var_t1_dn11 = assign68990_e105542_d_n11;
        locals.var_t1_dn14 = assign68990_e105542_d_n14;

        let (assign69000_e105549, assign69000_e105549_d_n0, assign69000_e105549_d_n2, assign69000_e105549_d_n4, assign69000_e105549_d_n5, assign69000_e105549_d_n6, assign69000_e105549_d_n7, assign69000_e105549_d_n8, assign69000_e105549_d_n9, assign69000_e105549_d_n10, assign69000_e105549_d_n11, assign69000_e105549_d_n14,) = {
    if (locals.var_guard1619 == 0.0) {
        let assign69000_e105547: f64 = (1.0 / locals.var_tox0);
        (assign69000_e105547, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign69000_e105549;
        locals.var_t2_dn0 = assign69000_e105549_d_n0;
        locals.var_t2_dn2 = assign69000_e105549_d_n2;
        locals.var_t2_dn4 = assign69000_e105549_d_n4;
        locals.var_t2_dn5 = assign69000_e105549_d_n5;
        locals.var_t2_dn6 = assign69000_e105549_d_n6;
        locals.var_t2_dn7 = assign69000_e105549_d_n7;
        locals.var_t2_dn8 = assign69000_e105549_d_n8;
        locals.var_t2_dn9 = assign69000_e105549_d_n9;
        locals.var_t2_dn10 = assign69000_e105549_d_n10;
        locals.var_t2_dn11 = assign69000_e105549_d_n11;
        locals.var_t2_dn14 = assign69000_e105549_d_n14;

        let (assign69010_e105556, assign69010_e105556_d_n0, assign69010_e105556_d_n2, assign69010_e105556_d_n4, assign69010_e105556_d_n5, assign69010_e105556_d_n6, assign69010_e105556_d_n7, assign69010_e105556_d_n8, assign69010_e105556_d_n9, assign69010_e105556_d_n10, assign69010_e105556_d_n11, assign69010_e105556_d_n14,) = {
    if (locals.var_guard1619 == 0.0) {
        let assign69010_e105554: f64 = (locals.var_t1 * locals.var_t2);
        (assign69010_e105554, ((locals.var_t1_dn0 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn0)), ((locals.var_t1_dn2 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn2)), ((locals.var_t1_dn4 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn4)), ((locals.var_t1_dn5 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn5)), ((locals.var_t1_dn6 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn6)), ((locals.var_t1_dn7 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn7)), ((locals.var_t1_dn8 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn8)), ((locals.var_t1_dn9 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn9)), ((locals.var_t1_dn10 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn10)), ((locals.var_t1_dn11 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn11)), ((locals.var_t1_dn14 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn14)),)
    } else {
        (locals.var_e1, locals.var_e1_dn0, locals.var_e1_dn2, locals.var_e1_dn4, locals.var_e1_dn5, locals.var_e1_dn6, locals.var_e1_dn7, locals.var_e1_dn8, locals.var_e1_dn9, locals.var_e1_dn10, locals.var_e1_dn11, locals.var_e1_dn14,)
    }
};
        locals.var_e1 = assign69010_e105556;
        locals.var_e1_dn0 = assign69010_e105556_d_n0;
        locals.var_e1_dn2 = assign69010_e105556_d_n2;
        locals.var_e1_dn4 = assign69010_e105556_d_n4;
        locals.var_e1_dn5 = assign69010_e105556_d_n5;
        locals.var_e1_dn6 = assign69010_e105556_d_n6;
        locals.var_e1_dn7 = assign69010_e105556_d_n7;
        locals.var_e1_dn8 = assign69010_e105556_d_n8;
        locals.var_e1_dn9 = assign69010_e105556_d_n9;
        locals.var_e1_dn10 = assign69010_e105556_d_n10;
        locals.var_e1_dn11 = assign69010_e105556_d_n11;
        locals.var_e1_dn14 = assign69010_e105556_d_n14;

        let (assign69020_e105574, assign69020_e105574_d_n0, assign69020_e105574_d_n2, assign69020_e105574_d_n4, assign69020_e105574_d_n5, assign69020_e105574_d_n6, assign69020_e105574_d_n7, assign69020_e105574_d_n8, assign69020_e105574_d_n9, assign69020_e105574_d_n10, assign69020_e105574_d_n11, assign69020_e105574_d_n14,) = {
    if (locals.var_guard1619 == 0.0) {
        let assign69020_e105561: f64 = (locals.var_e1 * locals.var_e1);
        let assign69020_e105565: f64 = (0.01 / 0.01);
        let assign69020_e105566: f64 = (4.0 * assign69020_e105565);
        let assign69020_e105569: f64 = (0.01 / 0.01);
        let assign69020_e105570: f64 = (assign69020_e105566 * assign69020_e105569);
        let assign69020_e105571: f64 = (assign69020_e105561 + assign69020_e105570);
        let assign69020_e105572: f64 = (assign69020_e105571).sqrt();
        (assign69020_e105572, (((locals.var_e1_dn0 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn0)) / (2.0 * assign69020_e105572)), (((locals.var_e1_dn2 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn2)) / (2.0 * assign69020_e105572)), (((locals.var_e1_dn4 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn4)) / (2.0 * assign69020_e105572)), (((locals.var_e1_dn5 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn5)) / (2.0 * assign69020_e105572)), (((locals.var_e1_dn6 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn6)) / (2.0 * assign69020_e105572)), (((locals.var_e1_dn7 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn7)) / (2.0 * assign69020_e105572)), (((locals.var_e1_dn8 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn8)) / (2.0 * assign69020_e105572)), (((locals.var_e1_dn9 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn9)) / (2.0 * assign69020_e105572)), (((locals.var_e1_dn10 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn10)) / (2.0 * assign69020_e105572)), (((locals.var_e1_dn11 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn11)) / (2.0 * assign69020_e105572)), (((locals.var_e1_dn14 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn14)) / (2.0 * assign69020_e105572)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign69020_e105574;
        locals.var_tmf2_dn0 = assign69020_e105574_d_n0;
        locals.var_tmf2_dn2 = assign69020_e105574_d_n2;
        locals.var_tmf2_dn4 = assign69020_e105574_d_n4;
        locals.var_tmf2_dn5 = assign69020_e105574_d_n5;
        locals.var_tmf2_dn6 = assign69020_e105574_d_n6;
        locals.var_tmf2_dn7 = assign69020_e105574_d_n7;
        locals.var_tmf2_dn8 = assign69020_e105574_d_n8;
        locals.var_tmf2_dn9 = assign69020_e105574_d_n9;
        locals.var_tmf2_dn10 = assign69020_e105574_d_n10;
        locals.var_tmf2_dn11 = assign69020_e105574_d_n11;
        locals.var_tmf2_dn14 = assign69020_e105574_d_n14;

        let (assign69030_e105585, assign69030_e105585_d_n0, assign69030_e105585_d_n2, assign69030_e105585_d_n4, assign69030_e105585_d_n5, assign69030_e105585_d_n6, assign69030_e105585_d_n7, assign69030_e105585_d_n8, assign69030_e105585_d_n9, assign69030_e105585_d_n10, assign69030_e105585_d_n11, assign69030_e105585_d_n14,) = {
    if (locals.var_guard1619 == 0.0) {
        let assign69030_e105581: f64 = (locals.var_e1 / locals.var_tmf2);
        let assign69030_e105582: f64 = (1.0 + assign69030_e105581);
        let assign69030_e105583: f64 = (0.5 * assign69030_e105582);
        (assign69030_e105583, (0.5 * (((locals.var_e1_dn0 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_e1_dn2 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_e1_dn4 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_e1_dn5 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_e1_dn6 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_e1_dn7 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_e1_dn8 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_e1_dn9 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_e1_dn10 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_e1_dn11 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_e1_dn14 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign69030_e105585;
        locals.var_t5_dn0 = assign69030_e105585_d_n0;
        locals.var_t5_dn2 = assign69030_e105585_d_n2;
        locals.var_t5_dn4 = assign69030_e105585_d_n4;
        locals.var_t5_dn5 = assign69030_e105585_d_n5;
        locals.var_t5_dn6 = assign69030_e105585_d_n6;
        locals.var_t5_dn7 = assign69030_e105585_d_n7;
        locals.var_t5_dn8 = assign69030_e105585_d_n8;
        locals.var_t5_dn9 = assign69030_e105585_d_n9;
        locals.var_t5_dn10 = assign69030_e105585_d_n10;
        locals.var_t5_dn11 = assign69030_e105585_d_n11;
        locals.var_t5_dn14 = assign69030_e105585_d_n14;

        let (assign69040_e105594, assign69040_e105594_d_n0, assign69040_e105594_d_n2, assign69040_e105594_d_n4, assign69040_e105594_d_n5, assign69040_e105594_d_n6, assign69040_e105594_d_n7, assign69040_e105594_d_n8, assign69040_e105594_d_n9, assign69040_e105594_d_n10, assign69040_e105594_d_n11, assign69040_e105594_d_n14,) = {
    if (locals.var_guard1619 == 0.0) {
        let assign69040_e105591: f64 = (locals.var_e1 + locals.var_tmf2);
        let assign69040_e105592: f64 = (0.5 * assign69040_e105591);
        (assign69040_e105592, (0.5 * (locals.var_e1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_e1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_e1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_e1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_e1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_e1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_e1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_e1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_e1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_e1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_e1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_egidl, locals.var_egidl_dn0, locals.var_egidl_dn2, locals.var_egidl_dn4, locals.var_egidl_dn5, locals.var_egidl_dn6, locals.var_egidl_dn7, locals.var_egidl_dn8, locals.var_egidl_dn9, locals.var_egidl_dn10, locals.var_egidl_dn11, locals.var_egidl_dn14,)
    }
};
        locals.var_egidl = assign69040_e105594;
        locals.var_egidl_dn0 = assign69040_e105594_d_n0;
        locals.var_egidl_dn2 = assign69040_e105594_d_n2;
        locals.var_egidl_dn4 = assign69040_e105594_d_n4;
        locals.var_egidl_dn5 = assign69040_e105594_d_n5;
        locals.var_egidl_dn6 = assign69040_e105594_d_n6;
        locals.var_egidl_dn7 = assign69040_e105594_d_n7;
        locals.var_egidl_dn8 = assign69040_e105594_d_n8;
        locals.var_egidl_dn9 = assign69040_e105594_d_n9;
        locals.var_egidl_dn10 = assign69040_e105594_d_n10;
        locals.var_egidl_dn11 = assign69040_e105594_d_n11;
        locals.var_egidl_dn14 = assign69040_e105594_d_n14;

        let assign69050_e105597: f64 = if locals.var_egidl < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1620 = assign69050_e105597;

        let (assign69060_e105604, assign69060_e105604_d_n0, assign69060_e105604_d_n2, assign69060_e105604_d_n4, assign69060_e105604_d_n5, assign69060_e105604_d_n6, assign69060_e105604_d_n7, assign69060_e105604_d_n8, assign69060_e105604_d_n9, assign69060_e105604_d_n10, assign69060_e105604_d_n11, assign69060_e105604_d_n14,) = {
    if ((locals.var_guard1619 == 0.0) && (locals.var_guard1620 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_egidl, locals.var_egidl_dn0, locals.var_egidl_dn2, locals.var_egidl_dn4, locals.var_egidl_dn5, locals.var_egidl_dn6, locals.var_egidl_dn7, locals.var_egidl_dn8, locals.var_egidl_dn9, locals.var_egidl_dn10, locals.var_egidl_dn11, locals.var_egidl_dn14,)
    }
};
        locals.var_egidl = assign69060_e105604;
        locals.var_egidl_dn0 = assign69060_e105604_d_n0;
        locals.var_egidl_dn2 = assign69060_e105604_d_n2;
        locals.var_egidl_dn4 = assign69060_e105604_d_n4;
        locals.var_egidl_dn5 = assign69060_e105604_d_n5;
        locals.var_egidl_dn6 = assign69060_e105604_d_n6;
        locals.var_egidl_dn7 = assign69060_e105604_d_n7;
        locals.var_egidl_dn8 = assign69060_e105604_d_n8;
        locals.var_egidl_dn9 = assign69060_e105604_d_n9;
        locals.var_egidl_dn10 = assign69060_e105604_d_n10;
        locals.var_egidl_dn11 = assign69060_e105604_d_n11;
        locals.var_egidl_dn14 = assign69060_e105604_d_n14;

        let (assign69070_e105611, assign69070_e105611_d_n0, assign69070_e105611_d_n2, assign69070_e105611_d_n4, assign69070_e105611_d_n5, assign69070_e105611_d_n6, assign69070_e105611_d_n7, assign69070_e105611_d_n8, assign69070_e105611_d_n9, assign69070_e105611_d_n10, assign69070_e105611_d_n11, assign69070_e105611_d_n14,) = {
    if ((locals.var_guard1619 == 0.0) && (locals.var_guard1620 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign69070_e105611;
        locals.var_t5_dn0 = assign69070_e105611_d_n0;
        locals.var_t5_dn2 = assign69070_e105611_d_n2;
        locals.var_t5_dn4 = assign69070_e105611_d_n4;
        locals.var_t5_dn5 = assign69070_e105611_d_n5;
        locals.var_t5_dn6 = assign69070_e105611_d_n6;
        locals.var_t5_dn7 = assign69070_e105611_d_n7;
        locals.var_t5_dn8 = assign69070_e105611_d_n8;
        locals.var_t5_dn9 = assign69070_e105611_d_n9;
        locals.var_t5_dn10 = assign69070_e105611_d_n10;
        locals.var_t5_dn11 = assign69070_e105611_d_n11;
        locals.var_t5_dn14 = assign69070_e105611_d_n14;

        let (assign69080_e105620, assign69080_e105620_d_n0, assign69080_e105620_d_n2, assign69080_e105620_d_n4, assign69080_e105620_d_n5, assign69080_e105620_d_n6, assign69080_e105620_d_n7, assign69080_e105620_d_n8, assign69080_e105620_d_n9, assign69080_e105620_d_n10, assign69080_e105620_d_n11, assign69080_e105620_d_n14,) = {
    if (locals.var_guard1619 == 0.0) {
        let assign69080_e105617: f64 = (locals.var_egidl + 1e-25);
        let assign69080_e105618: f64 = (1.0 / assign69080_e105617);
        (assign69080_e105618, (-(locals.var_egidl_dn0 / (assign69080_e105617 * assign69080_e105617))), (-(locals.var_egidl_dn2 / (assign69080_e105617 * assign69080_e105617))), (-(locals.var_egidl_dn4 / (assign69080_e105617 * assign69080_e105617))), (-(locals.var_egidl_dn5 / (assign69080_e105617 * assign69080_e105617))), (-(locals.var_egidl_dn6 / (assign69080_e105617 * assign69080_e105617))), (-(locals.var_egidl_dn7 / (assign69080_e105617 * assign69080_e105617))), (-(locals.var_egidl_dn8 / (assign69080_e105617 * assign69080_e105617))), (-(locals.var_egidl_dn9 / (assign69080_e105617 * assign69080_e105617))), (-(locals.var_egidl_dn10 / (assign69080_e105617 * assign69080_e105617))), (-(locals.var_egidl_dn11 / (assign69080_e105617 * assign69080_e105617))), (-(locals.var_egidl_dn14 / (assign69080_e105617 * assign69080_e105617))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign69080_e105620;
        locals.var_t3_dn0 = assign69080_e105620_d_n0;
        locals.var_t3_dn2 = assign69080_e105620_d_n2;
        locals.var_t3_dn4 = assign69080_e105620_d_n4;
        locals.var_t3_dn5 = assign69080_e105620_d_n5;
        locals.var_t3_dn6 = assign69080_e105620_d_n6;
        locals.var_t3_dn7 = assign69080_e105620_d_n7;
        locals.var_t3_dn8 = assign69080_e105620_d_n8;
        locals.var_t3_dn9 = assign69080_e105620_d_n9;
        locals.var_t3_dn10 = assign69080_e105620_d_n10;
        locals.var_t3_dn11 = assign69080_e105620_d_n11;
        locals.var_t3_dn14 = assign69080_e105620_d_n14;

        let (assign69090_e105630, assign69090_e105630_d_n0, assign69090_e105630_d_n2, assign69090_e105630_d_n4, assign69090_e105630_d_n5, assign69090_e105630_d_n6, assign69090_e105630_d_n7, assign69090_e105630_d_n8, assign69090_e105630_d_n9, assign69090_e105630_d_n10, assign69090_e105630_d_n11, assign69090_e105630_d_n14,) = {
    if (locals.var_guard1619 == 0.0) {
        let assign69090_e105624: f64 = (-locals.var_uc_gidl2);
        let assign69090_e105626: f64 = (assign69090_e105624 * locals.var_egp32);
        let assign69090_e105628: f64 = (assign69090_e105626 * locals.var_t3);
        (assign69090_e105628, (((assign69090_e105624 * locals.var_egp32_dn0) * locals.var_t3) + (assign69090_e105626 * locals.var_t3_dn0)), (((assign69090_e105624 * locals.var_egp32_dn2) * locals.var_t3) + (assign69090_e105626 * locals.var_t3_dn2)), (((assign69090_e105624 * locals.var_egp32_dn4) * locals.var_t3) + (assign69090_e105626 * locals.var_t3_dn4)), (((assign69090_e105624 * locals.var_egp32_dn5) * locals.var_t3) + (assign69090_e105626 * locals.var_t3_dn5)), (((assign69090_e105624 * locals.var_egp32_dn6) * locals.var_t3) + (assign69090_e105626 * locals.var_t3_dn6)), (((assign69090_e105624 * locals.var_egp32_dn7) * locals.var_t3) + (assign69090_e105626 * locals.var_t3_dn7)), (((assign69090_e105624 * locals.var_egp32_dn8) * locals.var_t3) + (assign69090_e105626 * locals.var_t3_dn8)), (((assign69090_e105624 * locals.var_egp32_dn9) * locals.var_t3) + (assign69090_e105626 * locals.var_t3_dn9)), (((assign69090_e105624 * locals.var_egp32_dn10) * locals.var_t3) + (assign69090_e105626 * locals.var_t3_dn10)), (((assign69090_e105624 * locals.var_egp32_dn11) * locals.var_t3) + (assign69090_e105626 * locals.var_t3_dn11)), (((assign69090_e105624 * locals.var_egp32_dn14) * locals.var_t3) + (assign69090_e105626 * locals.var_t3_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign69090_e105630;
        locals.var_t0_dn0 = assign69090_e105630_d_n0;
        locals.var_t0_dn2 = assign69090_e105630_d_n2;
        locals.var_t0_dn4 = assign69090_e105630_d_n4;
        locals.var_t0_dn5 = assign69090_e105630_d_n5;
        locals.var_t0_dn6 = assign69090_e105630_d_n6;
        locals.var_t0_dn7 = assign69090_e105630_d_n7;
        locals.var_t0_dn8 = assign69090_e105630_d_n8;
        locals.var_t0_dn9 = assign69090_e105630_d_n9;
        locals.var_t0_dn10 = assign69090_e105630_d_n10;
        locals.var_t0_dn11 = assign69090_e105630_d_n11;
        locals.var_t0_dn14 = assign69090_e105630_d_n14;

        let assign69100_e105633: f64 = (-34.0);
        let assign69100_e105634: f64 = if locals.var_t0 < assign69100_e105633 { 1.0 } else { 0.0 };
        locals.var_guard1621 = assign69100_e105634;

        let (assign69110_e105641, assign69110_e105641_d_n0, assign69110_e105641_d_n2, assign69110_e105641_d_n4, assign69110_e105641_d_n5, assign69110_e105641_d_n6, assign69110_e105641_d_n7, assign69110_e105641_d_n8, assign69110_e105641_d_n9, assign69110_e105641_d_n10, assign69110_e105641_d_n11, assign69110_e105641_d_n14,) = {
    if ((locals.var_guard1619 == 0.0) && (locals.var_guard1621 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_igidl, locals.var_igidl_dn0, locals.var_igidl_dn2, locals.var_igidl_dn4, locals.var_igidl_dn5, locals.var_igidl_dn6, locals.var_igidl_dn7, locals.var_igidl_dn8, locals.var_igidl_dn9, locals.var_igidl_dn10, locals.var_igidl_dn11, locals.var_igidl_dn14,)
    }
};
        locals.var_igidl = assign69110_e105641;
        locals.var_igidl_dn0 = assign69110_e105641_d_n0;
        locals.var_igidl_dn2 = assign69110_e105641_d_n2;
        locals.var_igidl_dn4 = assign69110_e105641_d_n4;
        locals.var_igidl_dn5 = assign69110_e105641_d_n5;
        locals.var_igidl_dn6 = assign69110_e105641_d_n6;
        locals.var_igidl_dn7 = assign69110_e105641_d_n7;
        locals.var_igidl_dn8 = assign69110_e105641_d_n8;
        locals.var_igidl_dn9 = assign69110_e105641_d_n9;
        locals.var_igidl_dn10 = assign69110_e105641_d_n10;
        locals.var_igidl_dn11 = assign69110_e105641_d_n11;
        locals.var_igidl_dn14 = assign69110_e105641_d_n14;

        let (assign69120_e105650, assign69120_e105650_d_n0, assign69120_e105650_d_n2, assign69120_e105650_d_n4, assign69120_e105650_d_n5, assign69120_e105650_d_n6, assign69120_e105650_d_n7, assign69120_e105650_d_n8, assign69120_e105650_d_n9, assign69120_e105650_d_n10, assign69120_e105650_d_n11, assign69120_e105650_d_n14,) = {
    if ((locals.var_guard1619 == 0.0) && (locals.var_guard1621 == 0.0)) {
        let assign69120_e105648: f64 = (locals.var_t0).exp();
        (assign69120_e105648, (assign69120_e105648 * locals.var_t0_dn0), (assign69120_e105648 * locals.var_t0_dn2), (assign69120_e105648 * locals.var_t0_dn4), (assign69120_e105648 * locals.var_t0_dn5), (assign69120_e105648 * locals.var_t0_dn6), (assign69120_e105648 * locals.var_t0_dn7), (assign69120_e105648 * locals.var_t0_dn8), (assign69120_e105648 * locals.var_t0_dn9), (assign69120_e105648 * locals.var_t0_dn10), (assign69120_e105648 * locals.var_t0_dn11), (assign69120_e105648 * locals.var_t0_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign69120_e105650;
        locals.var_t1_dn0 = assign69120_e105650_d_n0;
        locals.var_t1_dn2 = assign69120_e105650_d_n2;
        locals.var_t1_dn4 = assign69120_e105650_d_n4;
        locals.var_t1_dn5 = assign69120_e105650_d_n5;
        locals.var_t1_dn6 = assign69120_e105650_d_n6;
        locals.var_t1_dn7 = assign69120_e105650_d_n7;
        locals.var_t1_dn8 = assign69120_e105650_d_n8;
        locals.var_t1_dn9 = assign69120_e105650_d_n9;
        locals.var_t1_dn10 = assign69120_e105650_d_n10;
        locals.var_t1_dn11 = assign69120_e105650_d_n11;
        locals.var_t1_dn14 = assign69120_e105650_d_n14;

        let (assign69130_e105664, assign69130_e105664_d_n0, assign69130_e105664_d_n2, assign69130_e105664_d_n4, assign69130_e105664_d_n5, assign69130_e105664_d_n6, assign69130_e105664_d_n7, assign69130_e105664_d_n8, assign69130_e105664_d_n9, assign69130_e105664_d_n10, assign69130_e105664_d_n11, assign69130_e105664_d_n14,) = {
    if ((locals.var_guard1619 == 0.0) && (locals.var_guard1621 == 0.0)) {
        let assign69130_e105658: f64 = (locals.var_uc_gidl1 / locals.var_egp12);
        let assign69130_e105660: f64 = (assign69130_e105658 * 1.6021918e-19);
        let assign69130_e105662: f64 = (assign69130_e105660 * locals.var_weff_nf);
        (assign69130_e105662, (((-((locals.var_uc_gidl1 * locals.var_egp12_dn0) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_weff_nf), (((-((locals.var_uc_gidl1 * locals.var_egp12_dn2) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_weff_nf), (((-((locals.var_uc_gidl1 * locals.var_egp12_dn4) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_weff_nf), (((-((locals.var_uc_gidl1 * locals.var_egp12_dn5) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_weff_nf), (((-((locals.var_uc_gidl1 * locals.var_egp12_dn6) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_weff_nf), (((-((locals.var_uc_gidl1 * locals.var_egp12_dn7) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_weff_nf), (((-((locals.var_uc_gidl1 * locals.var_egp12_dn8) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_weff_nf), (((-((locals.var_uc_gidl1 * locals.var_egp12_dn9) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_weff_nf), (((-((locals.var_uc_gidl1 * locals.var_egp12_dn10) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_weff_nf), (((-((locals.var_uc_gidl1 * locals.var_egp12_dn11) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_weff_nf), (((-((locals.var_uc_gidl1 * locals.var_egp12_dn14) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_weff_nf),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign69130_e105664;
        locals.var_t2_dn0 = assign69130_e105664_d_n0;
        locals.var_t2_dn2 = assign69130_e105664_d_n2;
        locals.var_t2_dn4 = assign69130_e105664_d_n4;
        locals.var_t2_dn5 = assign69130_e105664_d_n5;
        locals.var_t2_dn6 = assign69130_e105664_d_n6;
        locals.var_t2_dn7 = assign69130_e105664_d_n7;
        locals.var_t2_dn8 = assign69130_e105664_d_n8;
        locals.var_t2_dn9 = assign69130_e105664_d_n9;
        locals.var_t2_dn10 = assign69130_e105664_d_n10;
        locals.var_t2_dn11 = assign69130_e105664_d_n11;
        locals.var_t2_dn14 = assign69130_e105664_d_n14;

    }

    pub(super) fn stamp_transient_block_247(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign69140_e105678, assign69140_e105678_d_n0, assign69140_e105678_d_n2, assign69140_e105678_d_n4, assign69140_e105678_d_n5, assign69140_e105678_d_n6, assign69140_e105678_d_n7, assign69140_e105678_d_n8, assign69140_e105678_d_n9, assign69140_e105678_d_n10, assign69140_e105678_d_n11, assign69140_e105678_d_n14,) = {
    if ((locals.var_guard1619 == 0.0) && (locals.var_guard1621 == 0.0)) {
        let assign69140_e105672: f64 = (locals.var_t2 * locals.var_egidl);
        let assign69140_e105674: f64 = (assign69140_e105672 * locals.var_egidl);
        let assign69140_e105676: f64 = (assign69140_e105674 * locals.var_t1);
        (assign69140_e105676, ((((((locals.var_t2_dn0 * locals.var_egidl) + (locals.var_t2 * locals.var_egidl_dn0)) * locals.var_egidl) + (assign69140_e105672 * locals.var_egidl_dn0)) * locals.var_t1) + (assign69140_e105674 * locals.var_t1_dn0)), ((((((locals.var_t2_dn2 * locals.var_egidl) + (locals.var_t2 * locals.var_egidl_dn2)) * locals.var_egidl) + (assign69140_e105672 * locals.var_egidl_dn2)) * locals.var_t1) + (assign69140_e105674 * locals.var_t1_dn2)), ((((((locals.var_t2_dn4 * locals.var_egidl) + (locals.var_t2 * locals.var_egidl_dn4)) * locals.var_egidl) + (assign69140_e105672 * locals.var_egidl_dn4)) * locals.var_t1) + (assign69140_e105674 * locals.var_t1_dn4)), ((((((locals.var_t2_dn5 * locals.var_egidl) + (locals.var_t2 * locals.var_egidl_dn5)) * locals.var_egidl) + (assign69140_e105672 * locals.var_egidl_dn5)) * locals.var_t1) + (assign69140_e105674 * locals.var_t1_dn5)), ((((((locals.var_t2_dn6 * locals.var_egidl) + (locals.var_t2 * locals.var_egidl_dn6)) * locals.var_egidl) + (assign69140_e105672 * locals.var_egidl_dn6)) * locals.var_t1) + (assign69140_e105674 * locals.var_t1_dn6)), ((((((locals.var_t2_dn7 * locals.var_egidl) + (locals.var_t2 * locals.var_egidl_dn7)) * locals.var_egidl) + (assign69140_e105672 * locals.var_egidl_dn7)) * locals.var_t1) + (assign69140_e105674 * locals.var_t1_dn7)), ((((((locals.var_t2_dn8 * locals.var_egidl) + (locals.var_t2 * locals.var_egidl_dn8)) * locals.var_egidl) + (assign69140_e105672 * locals.var_egidl_dn8)) * locals.var_t1) + (assign69140_e105674 * locals.var_t1_dn8)), ((((((locals.var_t2_dn9 * locals.var_egidl) + (locals.var_t2 * locals.var_egidl_dn9)) * locals.var_egidl) + (assign69140_e105672 * locals.var_egidl_dn9)) * locals.var_t1) + (assign69140_e105674 * locals.var_t1_dn9)), ((((((locals.var_t2_dn10 * locals.var_egidl) + (locals.var_t2 * locals.var_egidl_dn10)) * locals.var_egidl) + (assign69140_e105672 * locals.var_egidl_dn10)) * locals.var_t1) + (assign69140_e105674 * locals.var_t1_dn10)), ((((((locals.var_t2_dn11 * locals.var_egidl) + (locals.var_t2 * locals.var_egidl_dn11)) * locals.var_egidl) + (assign69140_e105672 * locals.var_egidl_dn11)) * locals.var_t1) + (assign69140_e105674 * locals.var_t1_dn11)), ((((((locals.var_t2_dn14 * locals.var_egidl) + (locals.var_t2 * locals.var_egidl_dn14)) * locals.var_egidl) + (assign69140_e105672 * locals.var_egidl_dn14)) * locals.var_t1) + (assign69140_e105674 * locals.var_t1_dn14)),)
    } else {
        (locals.var_igidl, locals.var_igidl_dn0, locals.var_igidl_dn2, locals.var_igidl_dn4, locals.var_igidl_dn5, locals.var_igidl_dn6, locals.var_igidl_dn7, locals.var_igidl_dn8, locals.var_igidl_dn9, locals.var_igidl_dn10, locals.var_igidl_dn11, locals.var_igidl_dn14,)
    }
};
        locals.var_igidl = assign69140_e105678;
        locals.var_igidl_dn0 = assign69140_e105678_d_n0;
        locals.var_igidl_dn2 = assign69140_e105678_d_n2;
        locals.var_igidl_dn4 = assign69140_e105678_d_n4;
        locals.var_igidl_dn5 = assign69140_e105678_d_n5;
        locals.var_igidl_dn6 = assign69140_e105678_d_n6;
        locals.var_igidl_dn7 = assign69140_e105678_d_n7;
        locals.var_igidl_dn8 = assign69140_e105678_d_n8;
        locals.var_igidl_dn9 = assign69140_e105678_d_n9;
        locals.var_igidl_dn10 = assign69140_e105678_d_n10;
        locals.var_igidl_dn11 = assign69140_e105678_d_n11;
        locals.var_igidl_dn14 = assign69140_e105678_d_n14;

        let (assign69150_e105685, assign69150_e105685_d_n0, assign69150_e105685_d_n2, assign69150_e105685_d_n4, assign69150_e105685_d_n5, assign69150_e105685_d_n6, assign69150_e105685_d_n7, assign69150_e105685_d_n8, assign69150_e105685_d_n9, assign69150_e105685_d_n10, assign69150_e105685_d_n11, assign69150_e105685_d_n14,) = {
    if (locals.var_guard1619 == 0.0) {
        let assign69150_e105683: f64 = (locals.var_vds - locals.var_vbs);
        (assign69150_e105683, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn4, locals.var_vds_dn5, (locals.var_vds_dn6 - locals.var_vbs_dn6), locals.var_vds_dn7, (locals.var_vds_dn8 - locals.var_vbs_dn8), (locals.var_vds_dn9 - locals.var_vbs_dn9), locals.var_vds_dn10, locals.var_vds_dn11, locals.var_vds_dn14,)
    } else {
        (locals.var_vdb, locals.var_vdb_dn0, locals.var_vdb_dn2, locals.var_vdb_dn4, locals.var_vdb_dn5, locals.var_vdb_dn6, locals.var_vdb_dn7, locals.var_vdb_dn8, locals.var_vdb_dn9, locals.var_vdb_dn10, locals.var_vdb_dn11, locals.var_vdb_dn14,)
    }
};
        locals.var_vdb = assign69150_e105685;
        locals.var_vdb_dn0 = assign69150_e105685_d_n0;
        locals.var_vdb_dn2 = assign69150_e105685_d_n2;
        locals.var_vdb_dn4 = assign69150_e105685_d_n4;
        locals.var_vdb_dn5 = assign69150_e105685_d_n5;
        locals.var_vdb_dn6 = assign69150_e105685_d_n6;
        locals.var_vdb_dn7 = assign69150_e105685_d_n7;
        locals.var_vdb_dn8 = assign69150_e105685_d_n8;
        locals.var_vdb_dn9 = assign69150_e105685_d_n9;
        locals.var_vdb_dn10 = assign69150_e105685_d_n10;
        locals.var_vdb_dn11 = assign69150_e105685_d_n11;
        locals.var_vdb_dn14 = assign69150_e105685_d_n14;

        let assign69160_e105688: f64 = if locals.var_vdb > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1622 = assign69160_e105688;

        let (assign69170_e105697, assign69170_e105697_d_n0, assign69170_e105697_d_n2, assign69170_e105697_d_n4, assign69170_e105697_d_n5, assign69170_e105697_d_n6, assign69170_e105697_d_n7, assign69170_e105697_d_n8, assign69170_e105697_d_n9, assign69170_e105697_d_n10, assign69170_e105697_d_n11, assign69170_e105697_d_n14,) = {
    if ((locals.var_guard1619 == 0.0) && (locals.var_guard1622 != 0.0)) {
        let assign69170_e105695: f64 = (locals.var_vdb * locals.var_vdb);
        (assign69170_e105695, ((locals.var_vdb_dn0 * locals.var_vdb) + (locals.var_vdb * locals.var_vdb_dn0)), ((locals.var_vdb_dn2 * locals.var_vdb) + (locals.var_vdb * locals.var_vdb_dn2)), ((locals.var_vdb_dn4 * locals.var_vdb) + (locals.var_vdb * locals.var_vdb_dn4)), ((locals.var_vdb_dn5 * locals.var_vdb) + (locals.var_vdb * locals.var_vdb_dn5)), ((locals.var_vdb_dn6 * locals.var_vdb) + (locals.var_vdb * locals.var_vdb_dn6)), ((locals.var_vdb_dn7 * locals.var_vdb) + (locals.var_vdb * locals.var_vdb_dn7)), ((locals.var_vdb_dn8 * locals.var_vdb) + (locals.var_vdb * locals.var_vdb_dn8)), ((locals.var_vdb_dn9 * locals.var_vdb) + (locals.var_vdb * locals.var_vdb_dn9)), ((locals.var_vdb_dn10 * locals.var_vdb) + (locals.var_vdb * locals.var_vdb_dn10)), ((locals.var_vdb_dn11 * locals.var_vdb) + (locals.var_vdb * locals.var_vdb_dn11)), ((locals.var_vdb_dn14 * locals.var_vdb) + (locals.var_vdb * locals.var_vdb_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign69170_e105697;
        locals.var_t2_dn0 = assign69170_e105697_d_n0;
        locals.var_t2_dn2 = assign69170_e105697_d_n2;
        locals.var_t2_dn4 = assign69170_e105697_d_n4;
        locals.var_t2_dn5 = assign69170_e105697_d_n5;
        locals.var_t2_dn6 = assign69170_e105697_d_n6;
        locals.var_t2_dn7 = assign69170_e105697_d_n7;
        locals.var_t2_dn8 = assign69170_e105697_d_n8;
        locals.var_t2_dn9 = assign69170_e105697_d_n9;
        locals.var_t2_dn10 = assign69170_e105697_d_n10;
        locals.var_t2_dn11 = assign69170_e105697_d_n11;
        locals.var_t2_dn14 = assign69170_e105697_d_n14;

        let (assign69180_e105706, assign69180_e105706_d_n0, assign69180_e105706_d_n2, assign69180_e105706_d_n4, assign69180_e105706_d_n5, assign69180_e105706_d_n6, assign69180_e105706_d_n7, assign69180_e105706_d_n8, assign69180_e105706_d_n9, assign69180_e105706_d_n10, assign69180_e105706_d_n11, assign69180_e105706_d_n14,) = {
    if ((locals.var_guard1619 == 0.0) && (locals.var_guard1622 != 0.0)) {
        let assign69180_e105704: f64 = (locals.var_t2 * locals.var_vdb);
        (assign69180_e105704, ((locals.var_t2_dn0 * locals.var_vdb) + (locals.var_t2 * locals.var_vdb_dn0)), ((locals.var_t2_dn2 * locals.var_vdb) + (locals.var_t2 * locals.var_vdb_dn2)), ((locals.var_t2_dn4 * locals.var_vdb) + (locals.var_t2 * locals.var_vdb_dn4)), ((locals.var_t2_dn5 * locals.var_vdb) + (locals.var_t2 * locals.var_vdb_dn5)), ((locals.var_t2_dn6 * locals.var_vdb) + (locals.var_t2 * locals.var_vdb_dn6)), ((locals.var_t2_dn7 * locals.var_vdb) + (locals.var_t2 * locals.var_vdb_dn7)), ((locals.var_t2_dn8 * locals.var_vdb) + (locals.var_t2 * locals.var_vdb_dn8)), ((locals.var_t2_dn9 * locals.var_vdb) + (locals.var_t2 * locals.var_vdb_dn9)), ((locals.var_t2_dn10 * locals.var_vdb) + (locals.var_t2 * locals.var_vdb_dn10)), ((locals.var_t2_dn11 * locals.var_vdb) + (locals.var_t2 * locals.var_vdb_dn11)), ((locals.var_t2_dn14 * locals.var_vdb) + (locals.var_t2 * locals.var_vdb_dn14)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign69180_e105706;
        locals.var_t4_dn0 = assign69180_e105706_d_n0;
        locals.var_t4_dn2 = assign69180_e105706_d_n2;
        locals.var_t4_dn4 = assign69180_e105706_d_n4;
        locals.var_t4_dn5 = assign69180_e105706_d_n5;
        locals.var_t4_dn6 = assign69180_e105706_d_n6;
        locals.var_t4_dn7 = assign69180_e105706_d_n7;
        locals.var_t4_dn8 = assign69180_e105706_d_n8;
        locals.var_t4_dn9 = assign69180_e105706_d_n9;
        locals.var_t4_dn10 = assign69180_e105706_d_n10;
        locals.var_t4_dn11 = assign69180_e105706_d_n11;
        locals.var_t4_dn14 = assign69180_e105706_d_n14;

        let (assign69190_e105715, assign69190_e105715_d_n0, assign69190_e105715_d_n2, assign69190_e105715_d_n4, assign69190_e105715_d_n5, assign69190_e105715_d_n6, assign69190_e105715_d_n7, assign69190_e105715_d_n8, assign69190_e105715_d_n9, assign69190_e105715_d_n10, assign69190_e105715_d_n11, assign69190_e105715_d_n14,) = {
    if ((locals.var_guard1619 == 0.0) && (locals.var_guard1622 != 0.0)) {
        let assign69190_e105713: f64 = (locals.var_t4 + 0.5);
        (assign69190_e105713, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign69190_e105715;
        locals.var_t0_dn0 = assign69190_e105715_d_n0;
        locals.var_t0_dn2 = assign69190_e105715_d_n2;
        locals.var_t0_dn4 = assign69190_e105715_d_n4;
        locals.var_t0_dn5 = assign69190_e105715_d_n5;
        locals.var_t0_dn6 = assign69190_e105715_d_n6;
        locals.var_t0_dn7 = assign69190_e105715_d_n7;
        locals.var_t0_dn8 = assign69190_e105715_d_n8;
        locals.var_t0_dn9 = assign69190_e105715_d_n9;
        locals.var_t0_dn10 = assign69190_e105715_d_n10;
        locals.var_t0_dn11 = assign69190_e105715_d_n11;
        locals.var_t0_dn14 = assign69190_e105715_d_n14;

        let (assign69200_e105724, assign69200_e105724_d_n0, assign69200_e105724_d_n2, assign69200_e105724_d_n4, assign69200_e105724_d_n5, assign69200_e105724_d_n6, assign69200_e105724_d_n7, assign69200_e105724_d_n8, assign69200_e105724_d_n9, assign69200_e105724_d_n10, assign69200_e105724_d_n11, assign69200_e105724_d_n14,) = {
    if ((locals.var_guard1619 == 0.0) && (locals.var_guard1622 != 0.0)) {
        let assign69200_e105722: f64 = (locals.var_t4 / locals.var_t0);
        (assign69200_e105722, (((locals.var_t4_dn0 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn0)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn2 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn2)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn4 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn5 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn6 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn7 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn8 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn9 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn9)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn10 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn11 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn11)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn14 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn14)) / (locals.var_t0 * locals.var_t0)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign69200_e105724;
        locals.var_t5_dn0 = assign69200_e105724_d_n0;
        locals.var_t5_dn2 = assign69200_e105724_d_n2;
        locals.var_t5_dn4 = assign69200_e105724_d_n4;
        locals.var_t5_dn5 = assign69200_e105724_d_n5;
        locals.var_t5_dn6 = assign69200_e105724_d_n6;
        locals.var_t5_dn7 = assign69200_e105724_d_n7;
        locals.var_t5_dn8 = assign69200_e105724_d_n8;
        locals.var_t5_dn9 = assign69200_e105724_d_n9;
        locals.var_t5_dn10 = assign69200_e105724_d_n10;
        locals.var_t5_dn11 = assign69200_e105724_d_n11;
        locals.var_t5_dn14 = assign69200_e105724_d_n14;

        let (assign69210_e105745, assign69210_e105745_d_n0, assign69210_e105745_d_n2, assign69210_e105745_d_n4, assign69210_e105745_d_n5, assign69210_e105745_d_n6, assign69210_e105745_d_n7, assign69210_e105745_d_n8, assign69210_e105745_d_n9, assign69210_e105745_d_n10, assign69210_e105745_d_n11, assign69210_e105745_d_n14,) = {
    if ((locals.var_guard1619 == 0.0) && (locals.var_guard1622 != 0.0)) {
        let assign69210_e105731: f64 = (3.0 * locals.var_t2);
        let assign69210_e105733: f64 = (assign69210_e105731 * locals.var_t0);
        let assign69210_e105736: f64 = (locals.var_t4 * 3.0);
        let assign69210_e105738: f64 = (assign69210_e105736 * locals.var_t2);
        let assign69210_e105739: f64 = (assign69210_e105733 - assign69210_e105738);
        let assign69210_e105742: f64 = (locals.var_t0 * locals.var_t0);
        let assign69210_e105743: f64 = (assign69210_e105739 / assign69210_e105742);
        (assign69210_e105743, (((((((3.0 * locals.var_t2_dn0) * locals.var_t0) + (assign69210_e105731 * locals.var_t0_dn0)) - (((locals.var_t4_dn0 * 3.0) * locals.var_t2) + (assign69210_e105736 * locals.var_t2_dn0))) * assign69210_e105742) - (assign69210_e105739 * ((locals.var_t0_dn0 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn0)))) / (assign69210_e105742 * assign69210_e105742)), (((((((3.0 * locals.var_t2_dn2) * locals.var_t0) + (assign69210_e105731 * locals.var_t0_dn2)) - (((locals.var_t4_dn2 * 3.0) * locals.var_t2) + (assign69210_e105736 * locals.var_t2_dn2))) * assign69210_e105742) - (assign69210_e105739 * ((locals.var_t0_dn2 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn2)))) / (assign69210_e105742 * assign69210_e105742)), (((((((3.0 * locals.var_t2_dn4) * locals.var_t0) + (assign69210_e105731 * locals.var_t0_dn4)) - (((locals.var_t4_dn4 * 3.0) * locals.var_t2) + (assign69210_e105736 * locals.var_t2_dn4))) * assign69210_e105742) - (assign69210_e105739 * ((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)))) / (assign69210_e105742 * assign69210_e105742)), (((((((3.0 * locals.var_t2_dn5) * locals.var_t0) + (assign69210_e105731 * locals.var_t0_dn5)) - (((locals.var_t4_dn5 * 3.0) * locals.var_t2) + (assign69210_e105736 * locals.var_t2_dn5))) * assign69210_e105742) - (assign69210_e105739 * ((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)))) / (assign69210_e105742 * assign69210_e105742)), (((((((3.0 * locals.var_t2_dn6) * locals.var_t0) + (assign69210_e105731 * locals.var_t0_dn6)) - (((locals.var_t4_dn6 * 3.0) * locals.var_t2) + (assign69210_e105736 * locals.var_t2_dn6))) * assign69210_e105742) - (assign69210_e105739 * ((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)))) / (assign69210_e105742 * assign69210_e105742)), (((((((3.0 * locals.var_t2_dn7) * locals.var_t0) + (assign69210_e105731 * locals.var_t0_dn7)) - (((locals.var_t4_dn7 * 3.0) * locals.var_t2) + (assign69210_e105736 * locals.var_t2_dn7))) * assign69210_e105742) - (assign69210_e105739 * ((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)))) / (assign69210_e105742 * assign69210_e105742)), (((((((3.0 * locals.var_t2_dn8) * locals.var_t0) + (assign69210_e105731 * locals.var_t0_dn8)) - (((locals.var_t4_dn8 * 3.0) * locals.var_t2) + (assign69210_e105736 * locals.var_t2_dn8))) * assign69210_e105742) - (assign69210_e105739 * ((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)))) / (assign69210_e105742 * assign69210_e105742)), (((((((3.0 * locals.var_t2_dn9) * locals.var_t0) + (assign69210_e105731 * locals.var_t0_dn9)) - (((locals.var_t4_dn9 * 3.0) * locals.var_t2) + (assign69210_e105736 * locals.var_t2_dn9))) * assign69210_e105742) - (assign69210_e105739 * ((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)))) / (assign69210_e105742 * assign69210_e105742)), (((((((3.0 * locals.var_t2_dn10) * locals.var_t0) + (assign69210_e105731 * locals.var_t0_dn10)) - (((locals.var_t4_dn10 * 3.0) * locals.var_t2) + (assign69210_e105736 * locals.var_t2_dn10))) * assign69210_e105742) - (assign69210_e105739 * ((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)))) / (assign69210_e105742 * assign69210_e105742)), (((((((3.0 * locals.var_t2_dn11) * locals.var_t0) + (assign69210_e105731 * locals.var_t0_dn11)) - (((locals.var_t4_dn11 * 3.0) * locals.var_t2) + (assign69210_e105736 * locals.var_t2_dn11))) * assign69210_e105742) - (assign69210_e105739 * ((locals.var_t0_dn11 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn11)))) / (assign69210_e105742 * assign69210_e105742)), (((((((3.0 * locals.var_t2_dn14) * locals.var_t0) + (assign69210_e105731 * locals.var_t0_dn14)) - (((locals.var_t4_dn14 * 3.0) * locals.var_t2) + (assign69210_e105736 * locals.var_t2_dn14))) * assign69210_e105742) - (assign69210_e105739 * ((locals.var_t0_dn14 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn14)))) / (assign69210_e105742 * assign69210_e105742)),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign69210_e105745;
        locals.var_t7_dn0 = assign69210_e105745_d_n0;
        locals.var_t7_dn2 = assign69210_e105745_d_n2;
        locals.var_t7_dn4 = assign69210_e105745_d_n4;
        locals.var_t7_dn5 = assign69210_e105745_d_n5;
        locals.var_t7_dn6 = assign69210_e105745_d_n6;
        locals.var_t7_dn7 = assign69210_e105745_d_n7;
        locals.var_t7_dn8 = assign69210_e105745_d_n8;
        locals.var_t7_dn9 = assign69210_e105745_d_n9;
        locals.var_t7_dn10 = assign69210_e105745_d_n10;
        locals.var_t7_dn11 = assign69210_e105745_d_n11;
        locals.var_t7_dn14 = assign69210_e105745_d_n14;

        let (assign69220_e105754, assign69220_e105754_d_n0, assign69220_e105754_d_n2, assign69220_e105754_d_n4, assign69220_e105754_d_n5, assign69220_e105754_d_n6, assign69220_e105754_d_n7, assign69220_e105754_d_n8, assign69220_e105754_d_n9, assign69220_e105754_d_n10, assign69220_e105754_d_n11, assign69220_e105754_d_n14,) = {
    if ((locals.var_guard1619 == 0.0) && (locals.var_guard1622 != 0.0)) {
        let assign69220_e105752: f64 = (locals.var_igidl * locals.var_t5);
        (assign69220_e105752, ((locals.var_igidl_dn0 * locals.var_t5) + (locals.var_igidl * locals.var_t5_dn0)), ((locals.var_igidl_dn2 * locals.var_t5) + (locals.var_igidl * locals.var_t5_dn2)), ((locals.var_igidl_dn4 * locals.var_t5) + (locals.var_igidl * locals.var_t5_dn4)), ((locals.var_igidl_dn5 * locals.var_t5) + (locals.var_igidl * locals.var_t5_dn5)), ((locals.var_igidl_dn6 * locals.var_t5) + (locals.var_igidl * locals.var_t5_dn6)), ((locals.var_igidl_dn7 * locals.var_t5) + (locals.var_igidl * locals.var_t5_dn7)), ((locals.var_igidl_dn8 * locals.var_t5) + (locals.var_igidl * locals.var_t5_dn8)), ((locals.var_igidl_dn9 * locals.var_t5) + (locals.var_igidl * locals.var_t5_dn9)), ((locals.var_igidl_dn10 * locals.var_t5) + (locals.var_igidl * locals.var_t5_dn10)), ((locals.var_igidl_dn11 * locals.var_t5) + (locals.var_igidl * locals.var_t5_dn11)), ((locals.var_igidl_dn14 * locals.var_t5) + (locals.var_igidl * locals.var_t5_dn14)),)
    } else {
        (locals.var_igidl, locals.var_igidl_dn0, locals.var_igidl_dn2, locals.var_igidl_dn4, locals.var_igidl_dn5, locals.var_igidl_dn6, locals.var_igidl_dn7, locals.var_igidl_dn8, locals.var_igidl_dn9, locals.var_igidl_dn10, locals.var_igidl_dn11, locals.var_igidl_dn14,)
    }
};
        locals.var_igidl = assign69220_e105754;
        locals.var_igidl_dn0 = assign69220_e105754_d_n0;
        locals.var_igidl_dn2 = assign69220_e105754_d_n2;
        locals.var_igidl_dn4 = assign69220_e105754_d_n4;
        locals.var_igidl_dn5 = assign69220_e105754_d_n5;
        locals.var_igidl_dn6 = assign69220_e105754_d_n6;
        locals.var_igidl_dn7 = assign69220_e105754_d_n7;
        locals.var_igidl_dn8 = assign69220_e105754_d_n8;
        locals.var_igidl_dn9 = assign69220_e105754_d_n9;
        locals.var_igidl_dn10 = assign69220_e105754_d_n10;
        locals.var_igidl_dn11 = assign69220_e105754_d_n11;
        locals.var_igidl_dn14 = assign69220_e105754_d_n14;

        let (assign69230_e105762, assign69230_e105762_d_n0, assign69230_e105762_d_n2, assign69230_e105762_d_n4, assign69230_e105762_d_n5, assign69230_e105762_d_n6, assign69230_e105762_d_n7, assign69230_e105762_d_n8, assign69230_e105762_d_n9, assign69230_e105762_d_n10, assign69230_e105762_d_n11, assign69230_e105762_d_n14,) = {
    if ((locals.var_guard1619 == 0.0) && (locals.var_guard1622 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_igidl, locals.var_igidl_dn0, locals.var_igidl_dn2, locals.var_igidl_dn4, locals.var_igidl_dn5, locals.var_igidl_dn6, locals.var_igidl_dn7, locals.var_igidl_dn8, locals.var_igidl_dn9, locals.var_igidl_dn10, locals.var_igidl_dn11, locals.var_igidl_dn14,)
    }
};
        locals.var_igidl = assign69230_e105762;
        locals.var_igidl_dn0 = assign69230_e105762_d_n0;
        locals.var_igidl_dn2 = assign69230_e105762_d_n2;
        locals.var_igidl_dn4 = assign69230_e105762_d_n4;
        locals.var_igidl_dn5 = assign69230_e105762_d_n5;
        locals.var_igidl_dn6 = assign69230_e105762_d_n6;
        locals.var_igidl_dn7 = assign69230_e105762_d_n7;
        locals.var_igidl_dn8 = assign69230_e105762_d_n8;
        locals.var_igidl_dn9 = assign69230_e105762_d_n9;
        locals.var_igidl_dn10 = assign69230_e105762_d_n10;
        locals.var_igidl_dn11 = assign69230_e105762_d_n11;
        locals.var_igidl_dn14 = assign69230_e105762_d_n14;

        let assign69240_e105765: f64 = if p.p25 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1623 = assign69240_e105765;

        let (assign69250_e105769, assign69250_e105769_d_n0, assign69250_e105769_d_n2, assign69250_e105769_d_n4, assign69250_e105769_d_n5, assign69250_e105769_d_n6, assign69250_e105769_d_n7, assign69250_e105769_d_n8, assign69250_e105769_d_n9, assign69250_e105769_d_n10, assign69250_e105769_d_n11, assign69250_e105769_d_n14,) = {
    if (locals.var_guard1623 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_igisl, locals.var_igisl_dn0, locals.var_igisl_dn2, locals.var_igisl_dn4, locals.var_igisl_dn5, locals.var_igisl_dn6, locals.var_igisl_dn7, locals.var_igisl_dn8, locals.var_igisl_dn9, locals.var_igisl_dn10, locals.var_igisl_dn11, locals.var_igisl_dn14,)
    }
};
        locals.var_igisl = assign69250_e105769;
        locals.var_igisl_dn0 = assign69250_e105769_d_n0;
        locals.var_igisl_dn2 = assign69250_e105769_d_n2;
        locals.var_igisl_dn4 = assign69250_e105769_d_n4;
        locals.var_igisl_dn5 = assign69250_e105769_d_n5;
        locals.var_igisl_dn6 = assign69250_e105769_d_n6;
        locals.var_igisl_dn7 = assign69250_e105769_d_n7;
        locals.var_igisl_dn8 = assign69250_e105769_d_n8;
        locals.var_igisl_dn9 = assign69250_e105769_d_n9;
        locals.var_igisl_dn10 = assign69250_e105769_d_n10;
        locals.var_igisl_dn11 = assign69250_e105769_d_n11;
        locals.var_igisl_dn14 = assign69250_e105769_d_n14;

        let (assign69260_e105789, assign69260_e105789_d_n0, assign69260_e105789_d_n2, assign69260_e105789_d_n4, assign69260_e105789_d_n5, assign69260_e105789_d_n6, assign69260_e105789_d_n7, assign69260_e105789_d_n8, assign69260_e105789_d_n9, assign69260_e105789_d_n10, assign69260_e105789_d_n11, assign69260_e105789_d_n14,) = {
    if (locals.var_guard1623 == 0.0) {
        let assign69260_e105774: f64 = (-locals.var_vdsp);
        let assign69260_e105776: f64 = (assign69260_e105774 + p.p243);
        let assign69260_e105777: f64 = (p.p242 * assign69260_e105776);
        let assign69260_e105780: f64 = (locals.var_vgs - locals.var_vdsp);
        let assign69260_e105781: f64 = (assign69260_e105777 - assign69260_e105780);
        let assign69260_e105784: f64 = (locals.var_dvthsc + locals.var_dvthlp);
        let assign69260_e105786: f64 = (assign69260_e105784 * p.p244);
        let assign69260_e105787: f64 = (assign69260_e105781 + assign69260_e105786);
        (assign69260_e105787, (((p.p242 * (-locals.var_vdsp_dn0)) - (-locals.var_vdsp_dn0)) + ((locals.var_dvthsc_dn0 + locals.var_dvthlp_dn0) * p.p244)), (((p.p242 * (-locals.var_vdsp_dn2)) - (-locals.var_vdsp_dn2)) + ((locals.var_dvthsc_dn2 + locals.var_dvthlp_dn2) * p.p244)), (((p.p242 * (-locals.var_vdsp_dn4)) - (-locals.var_vdsp_dn4)) + ((locals.var_dvthsc_dn4 + locals.var_dvthlp_dn4) * p.p244)), (((p.p242 * (-locals.var_vdsp_dn5)) - (-locals.var_vdsp_dn5)) + ((locals.var_dvthsc_dn5 + locals.var_dvthlp_dn5) * p.p244)), (((p.p242 * (-locals.var_vdsp_dn6)) - (locals.var_vgs_dn6 - locals.var_vdsp_dn6)) + ((locals.var_dvthsc_dn6 + locals.var_dvthlp_dn6) * p.p244)), (((p.p242 * (-locals.var_vdsp_dn7)) - (locals.var_vgs_dn7 - locals.var_vdsp_dn7)) + ((locals.var_dvthsc_dn7 + locals.var_dvthlp_dn7) * p.p244)), (((p.p242 * (-locals.var_vdsp_dn8)) - (locals.var_vgs_dn8 - locals.var_vdsp_dn8)) + ((locals.var_dvthsc_dn8 + locals.var_dvthlp_dn8) * p.p244)), (((p.p242 * (-locals.var_vdsp_dn9)) - (-locals.var_vdsp_dn9)) + ((locals.var_dvthsc_dn9 + locals.var_dvthlp_dn9) * p.p244)), (((p.p242 * (-locals.var_vdsp_dn10)) - (-locals.var_vdsp_dn10)) + ((locals.var_dvthsc_dn10 + locals.var_dvthlp_dn10) * p.p244)), (((p.p242 * (-locals.var_vdsp_dn11)) - (-locals.var_vdsp_dn11)) + ((locals.var_dvthsc_dn11 + locals.var_dvthlp_dn11) * p.p244)), (((p.p242 * (-locals.var_vdsp_dn14)) - (-locals.var_vdsp_dn14)) + ((locals.var_dvthsc_dn14 + locals.var_dvthlp_dn14) * p.p244)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign69260_e105789;
        locals.var_t1_dn0 = assign69260_e105789_d_n0;
        locals.var_t1_dn2 = assign69260_e105789_d_n2;
        locals.var_t1_dn4 = assign69260_e105789_d_n4;
        locals.var_t1_dn5 = assign69260_e105789_d_n5;
        locals.var_t1_dn6 = assign69260_e105789_d_n6;
        locals.var_t1_dn7 = assign69260_e105789_d_n7;
        locals.var_t1_dn8 = assign69260_e105789_d_n8;
        locals.var_t1_dn9 = assign69260_e105789_d_n9;
        locals.var_t1_dn10 = assign69260_e105789_d_n10;
        locals.var_t1_dn11 = assign69260_e105789_d_n11;
        locals.var_t1_dn14 = assign69260_e105789_d_n14;

        let (assign69270_e105796, assign69270_e105796_d_n0, assign69270_e105796_d_n2, assign69270_e105796_d_n4, assign69270_e105796_d_n5, assign69270_e105796_d_n6, assign69270_e105796_d_n7, assign69270_e105796_d_n8, assign69270_e105796_d_n9, assign69270_e105796_d_n10, assign69270_e105796_d_n11, assign69270_e105796_d_n14,) = {
    if (locals.var_guard1623 == 0.0) {
        let assign69270_e105794: f64 = (1.0 / locals.var_tox0);
        (assign69270_e105794, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign69270_e105796;
        locals.var_t2_dn0 = assign69270_e105796_d_n0;
        locals.var_t2_dn2 = assign69270_e105796_d_n2;
        locals.var_t2_dn4 = assign69270_e105796_d_n4;
        locals.var_t2_dn5 = assign69270_e105796_d_n5;
        locals.var_t2_dn6 = assign69270_e105796_d_n6;
        locals.var_t2_dn7 = assign69270_e105796_d_n7;
        locals.var_t2_dn8 = assign69270_e105796_d_n8;
        locals.var_t2_dn9 = assign69270_e105796_d_n9;
        locals.var_t2_dn10 = assign69270_e105796_d_n10;
        locals.var_t2_dn11 = assign69270_e105796_d_n11;
        locals.var_t2_dn14 = assign69270_e105796_d_n14;

        let (assign69280_e105803, assign69280_e105803_d_n0, assign69280_e105803_d_n2, assign69280_e105803_d_n4, assign69280_e105803_d_n5, assign69280_e105803_d_n6, assign69280_e105803_d_n7, assign69280_e105803_d_n8, assign69280_e105803_d_n9, assign69280_e105803_d_n10, assign69280_e105803_d_n11, assign69280_e105803_d_n14,) = {
    if (locals.var_guard1623 == 0.0) {
        let assign69280_e105801: f64 = (locals.var_t1 * locals.var_t2);
        (assign69280_e105801, ((locals.var_t1_dn0 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn0)), ((locals.var_t1_dn2 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn2)), ((locals.var_t1_dn4 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn4)), ((locals.var_t1_dn5 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn5)), ((locals.var_t1_dn6 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn6)), ((locals.var_t1_dn7 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn7)), ((locals.var_t1_dn8 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn8)), ((locals.var_t1_dn9 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn9)), ((locals.var_t1_dn10 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn10)), ((locals.var_t1_dn11 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn11)), ((locals.var_t1_dn14 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn14)),)
    } else {
        (locals.var_e1, locals.var_e1_dn0, locals.var_e1_dn2, locals.var_e1_dn4, locals.var_e1_dn5, locals.var_e1_dn6, locals.var_e1_dn7, locals.var_e1_dn8, locals.var_e1_dn9, locals.var_e1_dn10, locals.var_e1_dn11, locals.var_e1_dn14,)
    }
};
        locals.var_e1 = assign69280_e105803;
        locals.var_e1_dn0 = assign69280_e105803_d_n0;
        locals.var_e1_dn2 = assign69280_e105803_d_n2;
        locals.var_e1_dn4 = assign69280_e105803_d_n4;
        locals.var_e1_dn5 = assign69280_e105803_d_n5;
        locals.var_e1_dn6 = assign69280_e105803_d_n6;
        locals.var_e1_dn7 = assign69280_e105803_d_n7;
        locals.var_e1_dn8 = assign69280_e105803_d_n8;
        locals.var_e1_dn9 = assign69280_e105803_d_n9;
        locals.var_e1_dn10 = assign69280_e105803_d_n10;
        locals.var_e1_dn11 = assign69280_e105803_d_n11;
        locals.var_e1_dn14 = assign69280_e105803_d_n14;

        let (assign69290_e105821, assign69290_e105821_d_n0, assign69290_e105821_d_n2, assign69290_e105821_d_n4, assign69290_e105821_d_n5, assign69290_e105821_d_n6, assign69290_e105821_d_n7, assign69290_e105821_d_n8, assign69290_e105821_d_n9, assign69290_e105821_d_n10, assign69290_e105821_d_n11, assign69290_e105821_d_n14,) = {
    if (locals.var_guard1623 == 0.0) {
        let assign69290_e105808: f64 = (locals.var_e1 * locals.var_e1);
        let assign69290_e105812: f64 = (0.01 / 0.01);
        let assign69290_e105813: f64 = (4.0 * assign69290_e105812);
        let assign69290_e105816: f64 = (0.01 / 0.01);
        let assign69290_e105817: f64 = (assign69290_e105813 * assign69290_e105816);
        let assign69290_e105818: f64 = (assign69290_e105808 + assign69290_e105817);
        let assign69290_e105819: f64 = (assign69290_e105818).sqrt();
        (assign69290_e105819, (((locals.var_e1_dn0 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn0)) / (2.0 * assign69290_e105819)), (((locals.var_e1_dn2 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn2)) / (2.0 * assign69290_e105819)), (((locals.var_e1_dn4 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn4)) / (2.0 * assign69290_e105819)), (((locals.var_e1_dn5 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn5)) / (2.0 * assign69290_e105819)), (((locals.var_e1_dn6 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn6)) / (2.0 * assign69290_e105819)), (((locals.var_e1_dn7 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn7)) / (2.0 * assign69290_e105819)), (((locals.var_e1_dn8 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn8)) / (2.0 * assign69290_e105819)), (((locals.var_e1_dn9 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn9)) / (2.0 * assign69290_e105819)), (((locals.var_e1_dn10 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn10)) / (2.0 * assign69290_e105819)), (((locals.var_e1_dn11 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn11)) / (2.0 * assign69290_e105819)), (((locals.var_e1_dn14 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn14)) / (2.0 * assign69290_e105819)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign69290_e105821;
        locals.var_tmf2_dn0 = assign69290_e105821_d_n0;
        locals.var_tmf2_dn2 = assign69290_e105821_d_n2;
        locals.var_tmf2_dn4 = assign69290_e105821_d_n4;
        locals.var_tmf2_dn5 = assign69290_e105821_d_n5;
        locals.var_tmf2_dn6 = assign69290_e105821_d_n6;
        locals.var_tmf2_dn7 = assign69290_e105821_d_n7;
        locals.var_tmf2_dn8 = assign69290_e105821_d_n8;
        locals.var_tmf2_dn9 = assign69290_e105821_d_n9;
        locals.var_tmf2_dn10 = assign69290_e105821_d_n10;
        locals.var_tmf2_dn11 = assign69290_e105821_d_n11;
        locals.var_tmf2_dn14 = assign69290_e105821_d_n14;

        let (assign69300_e105832, assign69300_e105832_d_n0, assign69300_e105832_d_n2, assign69300_e105832_d_n4, assign69300_e105832_d_n5, assign69300_e105832_d_n6, assign69300_e105832_d_n7, assign69300_e105832_d_n8, assign69300_e105832_d_n9, assign69300_e105832_d_n10, assign69300_e105832_d_n11, assign69300_e105832_d_n14,) = {
    if (locals.var_guard1623 == 0.0) {
        let assign69300_e105828: f64 = (locals.var_e1 / locals.var_tmf2);
        let assign69300_e105829: f64 = (1.0 + assign69300_e105828);
        let assign69300_e105830: f64 = (0.5 * assign69300_e105829);
        (assign69300_e105830, (0.5 * (((locals.var_e1_dn0 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_e1_dn2 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_e1_dn4 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_e1_dn5 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_e1_dn6 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_e1_dn7 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_e1_dn8 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_e1_dn9 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_e1_dn10 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_e1_dn11 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_e1_dn14 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign69300_e105832;
        locals.var_t5_dn0 = assign69300_e105832_d_n0;
        locals.var_t5_dn2 = assign69300_e105832_d_n2;
        locals.var_t5_dn4 = assign69300_e105832_d_n4;
        locals.var_t5_dn5 = assign69300_e105832_d_n5;
        locals.var_t5_dn6 = assign69300_e105832_d_n6;
        locals.var_t5_dn7 = assign69300_e105832_d_n7;
        locals.var_t5_dn8 = assign69300_e105832_d_n8;
        locals.var_t5_dn9 = assign69300_e105832_d_n9;
        locals.var_t5_dn10 = assign69300_e105832_d_n10;
        locals.var_t5_dn11 = assign69300_e105832_d_n11;
        locals.var_t5_dn14 = assign69300_e105832_d_n14;

        let (assign69310_e105841, assign69310_e105841_d_n0, assign69310_e105841_d_n2, assign69310_e105841_d_n4, assign69310_e105841_d_n5, assign69310_e105841_d_n6, assign69310_e105841_d_n7, assign69310_e105841_d_n8, assign69310_e105841_d_n9, assign69310_e105841_d_n10, assign69310_e105841_d_n11, assign69310_e105841_d_n14,) = {
    if (locals.var_guard1623 == 0.0) {
        let assign69310_e105838: f64 = (locals.var_e1 + locals.var_tmf2);
        let assign69310_e105839: f64 = (0.5 * assign69310_e105838);
        (assign69310_e105839, (0.5 * (locals.var_e1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_e1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_e1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_e1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_e1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_e1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_e1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_e1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_e1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_e1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_e1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_egisl, locals.var_egisl_dn0, locals.var_egisl_dn2, locals.var_egisl_dn4, locals.var_egisl_dn5, locals.var_egisl_dn6, locals.var_egisl_dn7, locals.var_egisl_dn8, locals.var_egisl_dn9, locals.var_egisl_dn10, locals.var_egisl_dn11, locals.var_egisl_dn14,)
    }
};
        locals.var_egisl = assign69310_e105841;
        locals.var_egisl_dn0 = assign69310_e105841_d_n0;
        locals.var_egisl_dn2 = assign69310_e105841_d_n2;
        locals.var_egisl_dn4 = assign69310_e105841_d_n4;
        locals.var_egisl_dn5 = assign69310_e105841_d_n5;
        locals.var_egisl_dn6 = assign69310_e105841_d_n6;
        locals.var_egisl_dn7 = assign69310_e105841_d_n7;
        locals.var_egisl_dn8 = assign69310_e105841_d_n8;
        locals.var_egisl_dn9 = assign69310_e105841_d_n9;
        locals.var_egisl_dn10 = assign69310_e105841_d_n10;
        locals.var_egisl_dn11 = assign69310_e105841_d_n11;
        locals.var_egisl_dn14 = assign69310_e105841_d_n14;

        let assign69320_e105844: f64 = if locals.var_egisl < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1624 = assign69320_e105844;

        let (assign69330_e105851, assign69330_e105851_d_n0, assign69330_e105851_d_n2, assign69330_e105851_d_n4, assign69330_e105851_d_n5, assign69330_e105851_d_n6, assign69330_e105851_d_n7, assign69330_e105851_d_n8, assign69330_e105851_d_n9, assign69330_e105851_d_n10, assign69330_e105851_d_n11, assign69330_e105851_d_n14,) = {
    if ((locals.var_guard1623 == 0.0) && (locals.var_guard1624 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_egisl, locals.var_egisl_dn0, locals.var_egisl_dn2, locals.var_egisl_dn4, locals.var_egisl_dn5, locals.var_egisl_dn6, locals.var_egisl_dn7, locals.var_egisl_dn8, locals.var_egisl_dn9, locals.var_egisl_dn10, locals.var_egisl_dn11, locals.var_egisl_dn14,)
    }
};
        locals.var_egisl = assign69330_e105851;
        locals.var_egisl_dn0 = assign69330_e105851_d_n0;
        locals.var_egisl_dn2 = assign69330_e105851_d_n2;
        locals.var_egisl_dn4 = assign69330_e105851_d_n4;
        locals.var_egisl_dn5 = assign69330_e105851_d_n5;
        locals.var_egisl_dn6 = assign69330_e105851_d_n6;
        locals.var_egisl_dn7 = assign69330_e105851_d_n7;
        locals.var_egisl_dn8 = assign69330_e105851_d_n8;
        locals.var_egisl_dn9 = assign69330_e105851_d_n9;
        locals.var_egisl_dn10 = assign69330_e105851_d_n10;
        locals.var_egisl_dn11 = assign69330_e105851_d_n11;
        locals.var_egisl_dn14 = assign69330_e105851_d_n14;

        let (assign69340_e105858, assign69340_e105858_d_n0, assign69340_e105858_d_n2, assign69340_e105858_d_n4, assign69340_e105858_d_n5, assign69340_e105858_d_n6, assign69340_e105858_d_n7, assign69340_e105858_d_n8, assign69340_e105858_d_n9, assign69340_e105858_d_n10, assign69340_e105858_d_n11, assign69340_e105858_d_n14,) = {
    if ((locals.var_guard1623 == 0.0) && (locals.var_guard1624 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign69340_e105858;
        locals.var_t5_dn0 = assign69340_e105858_d_n0;
        locals.var_t5_dn2 = assign69340_e105858_d_n2;
        locals.var_t5_dn4 = assign69340_e105858_d_n4;
        locals.var_t5_dn5 = assign69340_e105858_d_n5;
        locals.var_t5_dn6 = assign69340_e105858_d_n6;
        locals.var_t5_dn7 = assign69340_e105858_d_n7;
        locals.var_t5_dn8 = assign69340_e105858_d_n8;
        locals.var_t5_dn9 = assign69340_e105858_d_n9;
        locals.var_t5_dn10 = assign69340_e105858_d_n10;
        locals.var_t5_dn11 = assign69340_e105858_d_n11;
        locals.var_t5_dn14 = assign69340_e105858_d_n14;

        let (assign69350_e105867, assign69350_e105867_d_n0, assign69350_e105867_d_n2, assign69350_e105867_d_n4, assign69350_e105867_d_n5, assign69350_e105867_d_n6, assign69350_e105867_d_n7, assign69350_e105867_d_n8, assign69350_e105867_d_n9, assign69350_e105867_d_n10, assign69350_e105867_d_n11, assign69350_e105867_d_n14,) = {
    if (locals.var_guard1623 == 0.0) {
        let assign69350_e105864: f64 = (locals.var_egisl + 1e-25);
        let assign69350_e105865: f64 = (1.0 / assign69350_e105864);
        (assign69350_e105865, (-(locals.var_egisl_dn0 / (assign69350_e105864 * assign69350_e105864))), (-(locals.var_egisl_dn2 / (assign69350_e105864 * assign69350_e105864))), (-(locals.var_egisl_dn4 / (assign69350_e105864 * assign69350_e105864))), (-(locals.var_egisl_dn5 / (assign69350_e105864 * assign69350_e105864))), (-(locals.var_egisl_dn6 / (assign69350_e105864 * assign69350_e105864))), (-(locals.var_egisl_dn7 / (assign69350_e105864 * assign69350_e105864))), (-(locals.var_egisl_dn8 / (assign69350_e105864 * assign69350_e105864))), (-(locals.var_egisl_dn9 / (assign69350_e105864 * assign69350_e105864))), (-(locals.var_egisl_dn10 / (assign69350_e105864 * assign69350_e105864))), (-(locals.var_egisl_dn11 / (assign69350_e105864 * assign69350_e105864))), (-(locals.var_egisl_dn14 / (assign69350_e105864 * assign69350_e105864))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign69350_e105867;
        locals.var_t3_dn0 = assign69350_e105867_d_n0;
        locals.var_t3_dn2 = assign69350_e105867_d_n2;
        locals.var_t3_dn4 = assign69350_e105867_d_n4;
        locals.var_t3_dn5 = assign69350_e105867_d_n5;
        locals.var_t3_dn6 = assign69350_e105867_d_n6;
        locals.var_t3_dn7 = assign69350_e105867_d_n7;
        locals.var_t3_dn8 = assign69350_e105867_d_n8;
        locals.var_t3_dn9 = assign69350_e105867_d_n9;
        locals.var_t3_dn10 = assign69350_e105867_d_n10;
        locals.var_t3_dn11 = assign69350_e105867_d_n11;
        locals.var_t3_dn14 = assign69350_e105867_d_n14;

        let (assign69360_e105877, assign69360_e105877_d_n0, assign69360_e105877_d_n2, assign69360_e105877_d_n4, assign69360_e105877_d_n5, assign69360_e105877_d_n6, assign69360_e105877_d_n7, assign69360_e105877_d_n8, assign69360_e105877_d_n9, assign69360_e105877_d_n10, assign69360_e105877_d_n11, assign69360_e105877_d_n14,) = {
    if (locals.var_guard1623 == 0.0) {
        let assign69360_e105871: f64 = (-locals.var_uc_gidl2);
        let assign69360_e105873: f64 = (assign69360_e105871 * locals.var_egp32);
        let assign69360_e105875: f64 = (assign69360_e105873 * locals.var_t3);
        (assign69360_e105875, (((assign69360_e105871 * locals.var_egp32_dn0) * locals.var_t3) + (assign69360_e105873 * locals.var_t3_dn0)), (((assign69360_e105871 * locals.var_egp32_dn2) * locals.var_t3) + (assign69360_e105873 * locals.var_t3_dn2)), (((assign69360_e105871 * locals.var_egp32_dn4) * locals.var_t3) + (assign69360_e105873 * locals.var_t3_dn4)), (((assign69360_e105871 * locals.var_egp32_dn5) * locals.var_t3) + (assign69360_e105873 * locals.var_t3_dn5)), (((assign69360_e105871 * locals.var_egp32_dn6) * locals.var_t3) + (assign69360_e105873 * locals.var_t3_dn6)), (((assign69360_e105871 * locals.var_egp32_dn7) * locals.var_t3) + (assign69360_e105873 * locals.var_t3_dn7)), (((assign69360_e105871 * locals.var_egp32_dn8) * locals.var_t3) + (assign69360_e105873 * locals.var_t3_dn8)), (((assign69360_e105871 * locals.var_egp32_dn9) * locals.var_t3) + (assign69360_e105873 * locals.var_t3_dn9)), (((assign69360_e105871 * locals.var_egp32_dn10) * locals.var_t3) + (assign69360_e105873 * locals.var_t3_dn10)), (((assign69360_e105871 * locals.var_egp32_dn11) * locals.var_t3) + (assign69360_e105873 * locals.var_t3_dn11)), (((assign69360_e105871 * locals.var_egp32_dn14) * locals.var_t3) + (assign69360_e105873 * locals.var_t3_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign69360_e105877;
        locals.var_t0_dn0 = assign69360_e105877_d_n0;
        locals.var_t0_dn2 = assign69360_e105877_d_n2;
        locals.var_t0_dn4 = assign69360_e105877_d_n4;
        locals.var_t0_dn5 = assign69360_e105877_d_n5;
        locals.var_t0_dn6 = assign69360_e105877_d_n6;
        locals.var_t0_dn7 = assign69360_e105877_d_n7;
        locals.var_t0_dn8 = assign69360_e105877_d_n8;
        locals.var_t0_dn9 = assign69360_e105877_d_n9;
        locals.var_t0_dn10 = assign69360_e105877_d_n10;
        locals.var_t0_dn11 = assign69360_e105877_d_n11;
        locals.var_t0_dn14 = assign69360_e105877_d_n14;

        let assign69370_e105880: f64 = (-34.0);
        let assign69370_e105881: f64 = if locals.var_t0 < assign69370_e105880 { 1.0 } else { 0.0 };
        locals.var_guard1625 = assign69370_e105881;

        let (assign69380_e105888, assign69380_e105888_d_n0, assign69380_e105888_d_n2, assign69380_e105888_d_n4, assign69380_e105888_d_n5, assign69380_e105888_d_n6, assign69380_e105888_d_n7, assign69380_e105888_d_n8, assign69380_e105888_d_n9, assign69380_e105888_d_n10, assign69380_e105888_d_n11, assign69380_e105888_d_n14,) = {
    if ((locals.var_guard1623 == 0.0) && (locals.var_guard1625 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_igisl, locals.var_igisl_dn0, locals.var_igisl_dn2, locals.var_igisl_dn4, locals.var_igisl_dn5, locals.var_igisl_dn6, locals.var_igisl_dn7, locals.var_igisl_dn8, locals.var_igisl_dn9, locals.var_igisl_dn10, locals.var_igisl_dn11, locals.var_igisl_dn14,)
    }
};
        locals.var_igisl = assign69380_e105888;
        locals.var_igisl_dn0 = assign69380_e105888_d_n0;
        locals.var_igisl_dn2 = assign69380_e105888_d_n2;
        locals.var_igisl_dn4 = assign69380_e105888_d_n4;
        locals.var_igisl_dn5 = assign69380_e105888_d_n5;
        locals.var_igisl_dn6 = assign69380_e105888_d_n6;
        locals.var_igisl_dn7 = assign69380_e105888_d_n7;
        locals.var_igisl_dn8 = assign69380_e105888_d_n8;
        locals.var_igisl_dn9 = assign69380_e105888_d_n9;
        locals.var_igisl_dn10 = assign69380_e105888_d_n10;
        locals.var_igisl_dn11 = assign69380_e105888_d_n11;
        locals.var_igisl_dn14 = assign69380_e105888_d_n14;

        let (assign69390_e105897, assign69390_e105897_d_n0, assign69390_e105897_d_n2, assign69390_e105897_d_n4, assign69390_e105897_d_n5, assign69390_e105897_d_n6, assign69390_e105897_d_n7, assign69390_e105897_d_n8, assign69390_e105897_d_n9, assign69390_e105897_d_n10, assign69390_e105897_d_n11, assign69390_e105897_d_n14,) = {
    if ((locals.var_guard1623 == 0.0) && (locals.var_guard1625 == 0.0)) {
        let assign69390_e105895: f64 = (locals.var_t0).exp();
        (assign69390_e105895, (assign69390_e105895 * locals.var_t0_dn0), (assign69390_e105895 * locals.var_t0_dn2), (assign69390_e105895 * locals.var_t0_dn4), (assign69390_e105895 * locals.var_t0_dn5), (assign69390_e105895 * locals.var_t0_dn6), (assign69390_e105895 * locals.var_t0_dn7), (assign69390_e105895 * locals.var_t0_dn8), (assign69390_e105895 * locals.var_t0_dn9), (assign69390_e105895 * locals.var_t0_dn10), (assign69390_e105895 * locals.var_t0_dn11), (assign69390_e105895 * locals.var_t0_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign69390_e105897;
        locals.var_t1_dn0 = assign69390_e105897_d_n0;
        locals.var_t1_dn2 = assign69390_e105897_d_n2;
        locals.var_t1_dn4 = assign69390_e105897_d_n4;
        locals.var_t1_dn5 = assign69390_e105897_d_n5;
        locals.var_t1_dn6 = assign69390_e105897_d_n6;
        locals.var_t1_dn7 = assign69390_e105897_d_n7;
        locals.var_t1_dn8 = assign69390_e105897_d_n8;
        locals.var_t1_dn9 = assign69390_e105897_d_n9;
        locals.var_t1_dn10 = assign69390_e105897_d_n10;
        locals.var_t1_dn11 = assign69390_e105897_d_n11;
        locals.var_t1_dn14 = assign69390_e105897_d_n14;

    }

    pub(super) fn stamp_transient_block_248(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign69400_e105907, assign69400_e105907_d_n0, assign69400_e105907_d_n2, assign69400_e105907_d_n4, assign69400_e105907_d_n5, assign69400_e105907_d_n6, assign69400_e105907_d_n7, assign69400_e105907_d_n8, assign69400_e105907_d_n9, assign69400_e105907_d_n10, assign69400_e105907_d_n11, assign69400_e105907_d_n14,) = {
    if ((locals.var_guard1623 == 0.0) && (locals.var_guard1625 == 0.0)) {
        let assign69400_e105905: f64 = (1.0 / locals.var_egp12);
        (assign69400_e105905, (-(locals.var_egp12_dn0 / (locals.var_egp12 * locals.var_egp12))), (-(locals.var_egp12_dn2 / (locals.var_egp12 * locals.var_egp12))), (-(locals.var_egp12_dn4 / (locals.var_egp12 * locals.var_egp12))), (-(locals.var_egp12_dn5 / (locals.var_egp12 * locals.var_egp12))), (-(locals.var_egp12_dn6 / (locals.var_egp12 * locals.var_egp12))), (-(locals.var_egp12_dn7 / (locals.var_egp12 * locals.var_egp12))), (-(locals.var_egp12_dn8 / (locals.var_egp12 * locals.var_egp12))), (-(locals.var_egp12_dn9 / (locals.var_egp12 * locals.var_egp12))), (-(locals.var_egp12_dn10 / (locals.var_egp12 * locals.var_egp12))), (-(locals.var_egp12_dn11 / (locals.var_egp12 * locals.var_egp12))), (-(locals.var_egp12_dn14 / (locals.var_egp12 * locals.var_egp12))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign69400_e105907;
        locals.var_t3_dn0 = assign69400_e105907_d_n0;
        locals.var_t3_dn2 = assign69400_e105907_d_n2;
        locals.var_t3_dn4 = assign69400_e105907_d_n4;
        locals.var_t3_dn5 = assign69400_e105907_d_n5;
        locals.var_t3_dn6 = assign69400_e105907_d_n6;
        locals.var_t3_dn7 = assign69400_e105907_d_n7;
        locals.var_t3_dn8 = assign69400_e105907_d_n8;
        locals.var_t3_dn9 = assign69400_e105907_d_n9;
        locals.var_t3_dn10 = assign69400_e105907_d_n10;
        locals.var_t3_dn11 = assign69400_e105907_d_n11;
        locals.var_t3_dn14 = assign69400_e105907_d_n14;

        let (assign69410_e105921, assign69410_e105921_d_n0, assign69410_e105921_d_n2, assign69410_e105921_d_n4, assign69410_e105921_d_n5, assign69410_e105921_d_n6, assign69410_e105921_d_n7, assign69410_e105921_d_n8, assign69410_e105921_d_n9, assign69410_e105921_d_n10, assign69410_e105921_d_n11, assign69410_e105921_d_n14,) = {
    if ((locals.var_guard1623 == 0.0) && (locals.var_guard1625 == 0.0)) {
        let assign69410_e105915: f64 = (locals.var_uc_gidl1 * locals.var_t3);
        let assign69410_e105917: f64 = (assign69410_e105915 * 1.6021918e-19);
        let assign69410_e105919: f64 = (assign69410_e105917 * locals.var_weff_nf);
        (assign69410_e105919, (((locals.var_uc_gidl1 * locals.var_t3_dn0) * 1.6021918e-19) * locals.var_weff_nf), (((locals.var_uc_gidl1 * locals.var_t3_dn2) * 1.6021918e-19) * locals.var_weff_nf), (((locals.var_uc_gidl1 * locals.var_t3_dn4) * 1.6021918e-19) * locals.var_weff_nf), (((locals.var_uc_gidl1 * locals.var_t3_dn5) * 1.6021918e-19) * locals.var_weff_nf), (((locals.var_uc_gidl1 * locals.var_t3_dn6) * 1.6021918e-19) * locals.var_weff_nf), (((locals.var_uc_gidl1 * locals.var_t3_dn7) * 1.6021918e-19) * locals.var_weff_nf), (((locals.var_uc_gidl1 * locals.var_t3_dn8) * 1.6021918e-19) * locals.var_weff_nf), (((locals.var_uc_gidl1 * locals.var_t3_dn9) * 1.6021918e-19) * locals.var_weff_nf), (((locals.var_uc_gidl1 * locals.var_t3_dn10) * 1.6021918e-19) * locals.var_weff_nf), (((locals.var_uc_gidl1 * locals.var_t3_dn11) * 1.6021918e-19) * locals.var_weff_nf), (((locals.var_uc_gidl1 * locals.var_t3_dn14) * 1.6021918e-19) * locals.var_weff_nf),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign69410_e105921;
        locals.var_t2_dn0 = assign69410_e105921_d_n0;
        locals.var_t2_dn2 = assign69410_e105921_d_n2;
        locals.var_t2_dn4 = assign69410_e105921_d_n4;
        locals.var_t2_dn5 = assign69410_e105921_d_n5;
        locals.var_t2_dn6 = assign69410_e105921_d_n6;
        locals.var_t2_dn7 = assign69410_e105921_d_n7;
        locals.var_t2_dn8 = assign69410_e105921_d_n8;
        locals.var_t2_dn9 = assign69410_e105921_d_n9;
        locals.var_t2_dn10 = assign69410_e105921_d_n10;
        locals.var_t2_dn11 = assign69410_e105921_d_n11;
        locals.var_t2_dn14 = assign69410_e105921_d_n14;

        let (assign69420_e105935, assign69420_e105935_d_n0, assign69420_e105935_d_n2, assign69420_e105935_d_n4, assign69420_e105935_d_n5, assign69420_e105935_d_n6, assign69420_e105935_d_n7, assign69420_e105935_d_n8, assign69420_e105935_d_n9, assign69420_e105935_d_n10, assign69420_e105935_d_n11, assign69420_e105935_d_n14,) = {
    if ((locals.var_guard1623 == 0.0) && (locals.var_guard1625 == 0.0)) {
        let assign69420_e105929: f64 = (locals.var_t2 * locals.var_egisl);
        let assign69420_e105931: f64 = (assign69420_e105929 * locals.var_egisl);
        let assign69420_e105933: f64 = (assign69420_e105931 * locals.var_t1);
        (assign69420_e105933, ((((((locals.var_t2_dn0 * locals.var_egisl) + (locals.var_t2 * locals.var_egisl_dn0)) * locals.var_egisl) + (assign69420_e105929 * locals.var_egisl_dn0)) * locals.var_t1) + (assign69420_e105931 * locals.var_t1_dn0)), ((((((locals.var_t2_dn2 * locals.var_egisl) + (locals.var_t2 * locals.var_egisl_dn2)) * locals.var_egisl) + (assign69420_e105929 * locals.var_egisl_dn2)) * locals.var_t1) + (assign69420_e105931 * locals.var_t1_dn2)), ((((((locals.var_t2_dn4 * locals.var_egisl) + (locals.var_t2 * locals.var_egisl_dn4)) * locals.var_egisl) + (assign69420_e105929 * locals.var_egisl_dn4)) * locals.var_t1) + (assign69420_e105931 * locals.var_t1_dn4)), ((((((locals.var_t2_dn5 * locals.var_egisl) + (locals.var_t2 * locals.var_egisl_dn5)) * locals.var_egisl) + (assign69420_e105929 * locals.var_egisl_dn5)) * locals.var_t1) + (assign69420_e105931 * locals.var_t1_dn5)), ((((((locals.var_t2_dn6 * locals.var_egisl) + (locals.var_t2 * locals.var_egisl_dn6)) * locals.var_egisl) + (assign69420_e105929 * locals.var_egisl_dn6)) * locals.var_t1) + (assign69420_e105931 * locals.var_t1_dn6)), ((((((locals.var_t2_dn7 * locals.var_egisl) + (locals.var_t2 * locals.var_egisl_dn7)) * locals.var_egisl) + (assign69420_e105929 * locals.var_egisl_dn7)) * locals.var_t1) + (assign69420_e105931 * locals.var_t1_dn7)), ((((((locals.var_t2_dn8 * locals.var_egisl) + (locals.var_t2 * locals.var_egisl_dn8)) * locals.var_egisl) + (assign69420_e105929 * locals.var_egisl_dn8)) * locals.var_t1) + (assign69420_e105931 * locals.var_t1_dn8)), ((((((locals.var_t2_dn9 * locals.var_egisl) + (locals.var_t2 * locals.var_egisl_dn9)) * locals.var_egisl) + (assign69420_e105929 * locals.var_egisl_dn9)) * locals.var_t1) + (assign69420_e105931 * locals.var_t1_dn9)), ((((((locals.var_t2_dn10 * locals.var_egisl) + (locals.var_t2 * locals.var_egisl_dn10)) * locals.var_egisl) + (assign69420_e105929 * locals.var_egisl_dn10)) * locals.var_t1) + (assign69420_e105931 * locals.var_t1_dn10)), ((((((locals.var_t2_dn11 * locals.var_egisl) + (locals.var_t2 * locals.var_egisl_dn11)) * locals.var_egisl) + (assign69420_e105929 * locals.var_egisl_dn11)) * locals.var_t1) + (assign69420_e105931 * locals.var_t1_dn11)), ((((((locals.var_t2_dn14 * locals.var_egisl) + (locals.var_t2 * locals.var_egisl_dn14)) * locals.var_egisl) + (assign69420_e105929 * locals.var_egisl_dn14)) * locals.var_t1) + (assign69420_e105931 * locals.var_t1_dn14)),)
    } else {
        (locals.var_igisl, locals.var_igisl_dn0, locals.var_igisl_dn2, locals.var_igisl_dn4, locals.var_igisl_dn5, locals.var_igisl_dn6, locals.var_igisl_dn7, locals.var_igisl_dn8, locals.var_igisl_dn9, locals.var_igisl_dn10, locals.var_igisl_dn11, locals.var_igisl_dn14,)
    }
};
        locals.var_igisl = assign69420_e105935;
        locals.var_igisl_dn0 = assign69420_e105935_d_n0;
        locals.var_igisl_dn2 = assign69420_e105935_d_n2;
        locals.var_igisl_dn4 = assign69420_e105935_d_n4;
        locals.var_igisl_dn5 = assign69420_e105935_d_n5;
        locals.var_igisl_dn6 = assign69420_e105935_d_n6;
        locals.var_igisl_dn7 = assign69420_e105935_d_n7;
        locals.var_igisl_dn8 = assign69420_e105935_d_n8;
        locals.var_igisl_dn9 = assign69420_e105935_d_n9;
        locals.var_igisl_dn10 = assign69420_e105935_d_n10;
        locals.var_igisl_dn11 = assign69420_e105935_d_n11;
        locals.var_igisl_dn14 = assign69420_e105935_d_n14;

        let (assign69430_e105941, assign69430_e105941_d_n6, assign69430_e105941_d_n8, assign69430_e105941_d_n9,) = {
    if (locals.var_guard1623 == 0.0) {
        let assign69430_e105939: f64 = (-locals.var_vbs);
        (assign69430_e105939, (-locals.var_vbs_dn6), (-locals.var_vbs_dn8), (-locals.var_vbs_dn9),)
    } else {
        (locals.var_vsb, locals.var_vsb_dn6, locals.var_vsb_dn8, locals.var_vsb_dn9,)
    }
};
        locals.var_vsb = assign69430_e105941;
        locals.var_vsb_dn6 = assign69430_e105941_d_n6;
        locals.var_vsb_dn8 = assign69430_e105941_d_n8;
        locals.var_vsb_dn9 = assign69430_e105941_d_n9;

        let assign69440_e105944: f64 = if locals.var_vsb > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1626 = assign69440_e105944;

        let (assign69450_e105953, assign69450_e105953_d_n0, assign69450_e105953_d_n2, assign69450_e105953_d_n4, assign69450_e105953_d_n5, assign69450_e105953_d_n6, assign69450_e105953_d_n7, assign69450_e105953_d_n8, assign69450_e105953_d_n9, assign69450_e105953_d_n10, assign69450_e105953_d_n11, assign69450_e105953_d_n14,) = {
    if ((locals.var_guard1623 == 0.0) && (locals.var_guard1626 != 0.0)) {
        let assign69450_e105951: f64 = (locals.var_vsb * locals.var_vsb);
        (assign69450_e105951, 0.0, 0.0, 0.0, 0.0, ((locals.var_vsb_dn6 * locals.var_vsb) + (locals.var_vsb * locals.var_vsb_dn6)), 0.0, ((locals.var_vsb_dn8 * locals.var_vsb) + (locals.var_vsb * locals.var_vsb_dn8)), ((locals.var_vsb_dn9 * locals.var_vsb) + (locals.var_vsb * locals.var_vsb_dn9)), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign69450_e105953;
        locals.var_t2_dn0 = assign69450_e105953_d_n0;
        locals.var_t2_dn2 = assign69450_e105953_d_n2;
        locals.var_t2_dn4 = assign69450_e105953_d_n4;
        locals.var_t2_dn5 = assign69450_e105953_d_n5;
        locals.var_t2_dn6 = assign69450_e105953_d_n6;
        locals.var_t2_dn7 = assign69450_e105953_d_n7;
        locals.var_t2_dn8 = assign69450_e105953_d_n8;
        locals.var_t2_dn9 = assign69450_e105953_d_n9;
        locals.var_t2_dn10 = assign69450_e105953_d_n10;
        locals.var_t2_dn11 = assign69450_e105953_d_n11;
        locals.var_t2_dn14 = assign69450_e105953_d_n14;

        let (assign69460_e105962, assign69460_e105962_d_n0, assign69460_e105962_d_n2, assign69460_e105962_d_n4, assign69460_e105962_d_n5, assign69460_e105962_d_n6, assign69460_e105962_d_n7, assign69460_e105962_d_n8, assign69460_e105962_d_n9, assign69460_e105962_d_n10, assign69460_e105962_d_n11, assign69460_e105962_d_n14,) = {
    if ((locals.var_guard1623 == 0.0) && (locals.var_guard1626 != 0.0)) {
        let assign69460_e105960: f64 = (locals.var_t2 * locals.var_vsb);
        (assign69460_e105960, (locals.var_t2_dn0 * locals.var_vsb), (locals.var_t2_dn2 * locals.var_vsb), (locals.var_t2_dn4 * locals.var_vsb), (locals.var_t2_dn5 * locals.var_vsb), ((locals.var_t2_dn6 * locals.var_vsb) + (locals.var_t2 * locals.var_vsb_dn6)), (locals.var_t2_dn7 * locals.var_vsb), ((locals.var_t2_dn8 * locals.var_vsb) + (locals.var_t2 * locals.var_vsb_dn8)), ((locals.var_t2_dn9 * locals.var_vsb) + (locals.var_t2 * locals.var_vsb_dn9)), (locals.var_t2_dn10 * locals.var_vsb), (locals.var_t2_dn11 * locals.var_vsb), (locals.var_t2_dn14 * locals.var_vsb),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign69460_e105962;
        locals.var_t4_dn0 = assign69460_e105962_d_n0;
        locals.var_t4_dn2 = assign69460_e105962_d_n2;
        locals.var_t4_dn4 = assign69460_e105962_d_n4;
        locals.var_t4_dn5 = assign69460_e105962_d_n5;
        locals.var_t4_dn6 = assign69460_e105962_d_n6;
        locals.var_t4_dn7 = assign69460_e105962_d_n7;
        locals.var_t4_dn8 = assign69460_e105962_d_n8;
        locals.var_t4_dn9 = assign69460_e105962_d_n9;
        locals.var_t4_dn10 = assign69460_e105962_d_n10;
        locals.var_t4_dn11 = assign69460_e105962_d_n11;
        locals.var_t4_dn14 = assign69460_e105962_d_n14;

        let (assign69470_e105971, assign69470_e105971_d_n0, assign69470_e105971_d_n2, assign69470_e105971_d_n4, assign69470_e105971_d_n5, assign69470_e105971_d_n6, assign69470_e105971_d_n7, assign69470_e105971_d_n8, assign69470_e105971_d_n9, assign69470_e105971_d_n10, assign69470_e105971_d_n11, assign69470_e105971_d_n14,) = {
    if ((locals.var_guard1623 == 0.0) && (locals.var_guard1626 != 0.0)) {
        let assign69470_e105969: f64 = (locals.var_t4 + 0.5);
        (assign69470_e105969, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign69470_e105971;
        locals.var_t0_dn0 = assign69470_e105971_d_n0;
        locals.var_t0_dn2 = assign69470_e105971_d_n2;
        locals.var_t0_dn4 = assign69470_e105971_d_n4;
        locals.var_t0_dn5 = assign69470_e105971_d_n5;
        locals.var_t0_dn6 = assign69470_e105971_d_n6;
        locals.var_t0_dn7 = assign69470_e105971_d_n7;
        locals.var_t0_dn8 = assign69470_e105971_d_n8;
        locals.var_t0_dn9 = assign69470_e105971_d_n9;
        locals.var_t0_dn10 = assign69470_e105971_d_n10;
        locals.var_t0_dn11 = assign69470_e105971_d_n11;
        locals.var_t0_dn14 = assign69470_e105971_d_n14;

        let (assign69480_e105980, assign69480_e105980_d_n0, assign69480_e105980_d_n2, assign69480_e105980_d_n4, assign69480_e105980_d_n5, assign69480_e105980_d_n6, assign69480_e105980_d_n7, assign69480_e105980_d_n8, assign69480_e105980_d_n9, assign69480_e105980_d_n10, assign69480_e105980_d_n11, assign69480_e105980_d_n14,) = {
    if ((locals.var_guard1623 == 0.0) && (locals.var_guard1626 != 0.0)) {
        let assign69480_e105978: f64 = (locals.var_t4 / locals.var_t0);
        (assign69480_e105978, (((locals.var_t4_dn0 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn0)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn2 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn2)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn4 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn5 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn6 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn7 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn8 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn9 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn9)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn10 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn11 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn11)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn14 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn14)) / (locals.var_t0 * locals.var_t0)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign69480_e105980;
        locals.var_t5_dn0 = assign69480_e105980_d_n0;
        locals.var_t5_dn2 = assign69480_e105980_d_n2;
        locals.var_t5_dn4 = assign69480_e105980_d_n4;
        locals.var_t5_dn5 = assign69480_e105980_d_n5;
        locals.var_t5_dn6 = assign69480_e105980_d_n6;
        locals.var_t5_dn7 = assign69480_e105980_d_n7;
        locals.var_t5_dn8 = assign69480_e105980_d_n8;
        locals.var_t5_dn9 = assign69480_e105980_d_n9;
        locals.var_t5_dn10 = assign69480_e105980_d_n10;
        locals.var_t5_dn11 = assign69480_e105980_d_n11;
        locals.var_t5_dn14 = assign69480_e105980_d_n14;

        let (assign69490_e106001, assign69490_e106001_d_n0, assign69490_e106001_d_n2, assign69490_e106001_d_n4, assign69490_e106001_d_n5, assign69490_e106001_d_n6, assign69490_e106001_d_n7, assign69490_e106001_d_n8, assign69490_e106001_d_n9, assign69490_e106001_d_n10, assign69490_e106001_d_n11, assign69490_e106001_d_n14,) = {
    if ((locals.var_guard1623 == 0.0) && (locals.var_guard1626 != 0.0)) {
        let assign69490_e105987: f64 = (3.0 * locals.var_t2);
        let assign69490_e105989: f64 = (assign69490_e105987 * locals.var_t0);
        let assign69490_e105992: f64 = (locals.var_t4 * 3.0);
        let assign69490_e105994: f64 = (assign69490_e105992 * locals.var_t2);
        let assign69490_e105995: f64 = (assign69490_e105989 - assign69490_e105994);
        let assign69490_e105998: f64 = (locals.var_t0 * locals.var_t0);
        let assign69490_e105999: f64 = (assign69490_e105995 / assign69490_e105998);
        (assign69490_e105999, (((((((3.0 * locals.var_t2_dn0) * locals.var_t0) + (assign69490_e105987 * locals.var_t0_dn0)) - (((locals.var_t4_dn0 * 3.0) * locals.var_t2) + (assign69490_e105992 * locals.var_t2_dn0))) * assign69490_e105998) - (assign69490_e105995 * ((locals.var_t0_dn0 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn0)))) / (assign69490_e105998 * assign69490_e105998)), (((((((3.0 * locals.var_t2_dn2) * locals.var_t0) + (assign69490_e105987 * locals.var_t0_dn2)) - (((locals.var_t4_dn2 * 3.0) * locals.var_t2) + (assign69490_e105992 * locals.var_t2_dn2))) * assign69490_e105998) - (assign69490_e105995 * ((locals.var_t0_dn2 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn2)))) / (assign69490_e105998 * assign69490_e105998)), (((((((3.0 * locals.var_t2_dn4) * locals.var_t0) + (assign69490_e105987 * locals.var_t0_dn4)) - (((locals.var_t4_dn4 * 3.0) * locals.var_t2) + (assign69490_e105992 * locals.var_t2_dn4))) * assign69490_e105998) - (assign69490_e105995 * ((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)))) / (assign69490_e105998 * assign69490_e105998)), (((((((3.0 * locals.var_t2_dn5) * locals.var_t0) + (assign69490_e105987 * locals.var_t0_dn5)) - (((locals.var_t4_dn5 * 3.0) * locals.var_t2) + (assign69490_e105992 * locals.var_t2_dn5))) * assign69490_e105998) - (assign69490_e105995 * ((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)))) / (assign69490_e105998 * assign69490_e105998)), (((((((3.0 * locals.var_t2_dn6) * locals.var_t0) + (assign69490_e105987 * locals.var_t0_dn6)) - (((locals.var_t4_dn6 * 3.0) * locals.var_t2) + (assign69490_e105992 * locals.var_t2_dn6))) * assign69490_e105998) - (assign69490_e105995 * ((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)))) / (assign69490_e105998 * assign69490_e105998)), (((((((3.0 * locals.var_t2_dn7) * locals.var_t0) + (assign69490_e105987 * locals.var_t0_dn7)) - (((locals.var_t4_dn7 * 3.0) * locals.var_t2) + (assign69490_e105992 * locals.var_t2_dn7))) * assign69490_e105998) - (assign69490_e105995 * ((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)))) / (assign69490_e105998 * assign69490_e105998)), (((((((3.0 * locals.var_t2_dn8) * locals.var_t0) + (assign69490_e105987 * locals.var_t0_dn8)) - (((locals.var_t4_dn8 * 3.0) * locals.var_t2) + (assign69490_e105992 * locals.var_t2_dn8))) * assign69490_e105998) - (assign69490_e105995 * ((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)))) / (assign69490_e105998 * assign69490_e105998)), (((((((3.0 * locals.var_t2_dn9) * locals.var_t0) + (assign69490_e105987 * locals.var_t0_dn9)) - (((locals.var_t4_dn9 * 3.0) * locals.var_t2) + (assign69490_e105992 * locals.var_t2_dn9))) * assign69490_e105998) - (assign69490_e105995 * ((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)))) / (assign69490_e105998 * assign69490_e105998)), (((((((3.0 * locals.var_t2_dn10) * locals.var_t0) + (assign69490_e105987 * locals.var_t0_dn10)) - (((locals.var_t4_dn10 * 3.0) * locals.var_t2) + (assign69490_e105992 * locals.var_t2_dn10))) * assign69490_e105998) - (assign69490_e105995 * ((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)))) / (assign69490_e105998 * assign69490_e105998)), (((((((3.0 * locals.var_t2_dn11) * locals.var_t0) + (assign69490_e105987 * locals.var_t0_dn11)) - (((locals.var_t4_dn11 * 3.0) * locals.var_t2) + (assign69490_e105992 * locals.var_t2_dn11))) * assign69490_e105998) - (assign69490_e105995 * ((locals.var_t0_dn11 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn11)))) / (assign69490_e105998 * assign69490_e105998)), (((((((3.0 * locals.var_t2_dn14) * locals.var_t0) + (assign69490_e105987 * locals.var_t0_dn14)) - (((locals.var_t4_dn14 * 3.0) * locals.var_t2) + (assign69490_e105992 * locals.var_t2_dn14))) * assign69490_e105998) - (assign69490_e105995 * ((locals.var_t0_dn14 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn14)))) / (assign69490_e105998 * assign69490_e105998)),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign69490_e106001;
        locals.var_t7_dn0 = assign69490_e106001_d_n0;
        locals.var_t7_dn2 = assign69490_e106001_d_n2;
        locals.var_t7_dn4 = assign69490_e106001_d_n4;
        locals.var_t7_dn5 = assign69490_e106001_d_n5;
        locals.var_t7_dn6 = assign69490_e106001_d_n6;
        locals.var_t7_dn7 = assign69490_e106001_d_n7;
        locals.var_t7_dn8 = assign69490_e106001_d_n8;
        locals.var_t7_dn9 = assign69490_e106001_d_n9;
        locals.var_t7_dn10 = assign69490_e106001_d_n10;
        locals.var_t7_dn11 = assign69490_e106001_d_n11;
        locals.var_t7_dn14 = assign69490_e106001_d_n14;

        let (assign69500_e106010, assign69500_e106010_d_n0, assign69500_e106010_d_n2, assign69500_e106010_d_n4, assign69500_e106010_d_n5, assign69500_e106010_d_n6, assign69500_e106010_d_n7, assign69500_e106010_d_n8, assign69500_e106010_d_n9, assign69500_e106010_d_n10, assign69500_e106010_d_n11, assign69500_e106010_d_n14,) = {
    if ((locals.var_guard1623 == 0.0) && (locals.var_guard1626 != 0.0)) {
        let assign69500_e106008: f64 = (locals.var_igisl * locals.var_t5);
        (assign69500_e106008, ((locals.var_igisl_dn0 * locals.var_t5) + (locals.var_igisl * locals.var_t5_dn0)), ((locals.var_igisl_dn2 * locals.var_t5) + (locals.var_igisl * locals.var_t5_dn2)), ((locals.var_igisl_dn4 * locals.var_t5) + (locals.var_igisl * locals.var_t5_dn4)), ((locals.var_igisl_dn5 * locals.var_t5) + (locals.var_igisl * locals.var_t5_dn5)), ((locals.var_igisl_dn6 * locals.var_t5) + (locals.var_igisl * locals.var_t5_dn6)), ((locals.var_igisl_dn7 * locals.var_t5) + (locals.var_igisl * locals.var_t5_dn7)), ((locals.var_igisl_dn8 * locals.var_t5) + (locals.var_igisl * locals.var_t5_dn8)), ((locals.var_igisl_dn9 * locals.var_t5) + (locals.var_igisl * locals.var_t5_dn9)), ((locals.var_igisl_dn10 * locals.var_t5) + (locals.var_igisl * locals.var_t5_dn10)), ((locals.var_igisl_dn11 * locals.var_t5) + (locals.var_igisl * locals.var_t5_dn11)), ((locals.var_igisl_dn14 * locals.var_t5) + (locals.var_igisl * locals.var_t5_dn14)),)
    } else {
        (locals.var_igisl, locals.var_igisl_dn0, locals.var_igisl_dn2, locals.var_igisl_dn4, locals.var_igisl_dn5, locals.var_igisl_dn6, locals.var_igisl_dn7, locals.var_igisl_dn8, locals.var_igisl_dn9, locals.var_igisl_dn10, locals.var_igisl_dn11, locals.var_igisl_dn14,)
    }
};
        locals.var_igisl = assign69500_e106010;
        locals.var_igisl_dn0 = assign69500_e106010_d_n0;
        locals.var_igisl_dn2 = assign69500_e106010_d_n2;
        locals.var_igisl_dn4 = assign69500_e106010_d_n4;
        locals.var_igisl_dn5 = assign69500_e106010_d_n5;
        locals.var_igisl_dn6 = assign69500_e106010_d_n6;
        locals.var_igisl_dn7 = assign69500_e106010_d_n7;
        locals.var_igisl_dn8 = assign69500_e106010_d_n8;
        locals.var_igisl_dn9 = assign69500_e106010_d_n9;
        locals.var_igisl_dn10 = assign69500_e106010_d_n10;
        locals.var_igisl_dn11 = assign69500_e106010_d_n11;
        locals.var_igisl_dn14 = assign69500_e106010_d_n14;

        let (assign69510_e106018, assign69510_e106018_d_n0, assign69510_e106018_d_n2, assign69510_e106018_d_n4, assign69510_e106018_d_n5, assign69510_e106018_d_n6, assign69510_e106018_d_n7, assign69510_e106018_d_n8, assign69510_e106018_d_n9, assign69510_e106018_d_n10, assign69510_e106018_d_n11, assign69510_e106018_d_n14,) = {
    if ((locals.var_guard1623 == 0.0) && (locals.var_guard1626 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_igisl, locals.var_igisl_dn0, locals.var_igisl_dn2, locals.var_igisl_dn4, locals.var_igisl_dn5, locals.var_igisl_dn6, locals.var_igisl_dn7, locals.var_igisl_dn8, locals.var_igisl_dn9, locals.var_igisl_dn10, locals.var_igisl_dn11, locals.var_igisl_dn14,)
    }
};
        locals.var_igisl = assign69510_e106018;
        locals.var_igisl_dn0 = assign69510_e106018_d_n0;
        locals.var_igisl_dn2 = assign69510_e106018_d_n2;
        locals.var_igisl_dn4 = assign69510_e106018_d_n4;
        locals.var_igisl_dn5 = assign69510_e106018_d_n5;
        locals.var_igisl_dn6 = assign69510_e106018_d_n6;
        locals.var_igisl_dn7 = assign69510_e106018_d_n7;
        locals.var_igisl_dn8 = assign69510_e106018_d_n8;
        locals.var_igisl_dn9 = assign69510_e106018_d_n9;
        locals.var_igisl_dn10 = assign69510_e106018_d_n10;
        locals.var_igisl_dn11 = assign69510_e106018_d_n11;
        locals.var_igisl_dn14 = assign69510_e106018_d_n14;

        locals.var_flg_coovlps = 0.0;

        locals.var_flg_coovlp = 0.0;

        locals.var_flg_calcqover = 0.0;

        locals.var_flg_never_reach_vfbover = 0.0;

        locals.var_flg_calcqover = 0.0;

        let assign69580_e106027: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1631 = assign69580_e106027;

        let assign69590_e106030: f64 = if 1.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1632 = assign69590_e106030;

        let assign69600_e106033: f64 = if 1.0 == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard1633 = assign69600_e106033;

        let assign69610_e106036: f64 = if 1.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1634 = assign69610_e106036;

        let assign69620_e106047: f64 = if (((p.p36 == 1.0) && (p.p66 > 0.0)) && (locals.var_uc_novers > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1635 = assign69620_e106047;

        let (assign69630_e106053,) = {
    if ((locals.var_guard1631 != 0.0) && (locals.var_guard1635 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_calcqover,)
    }
};
        locals.var_flg_calcqover = assign69630_e106053;

        let (assign69640_e106059,) = {
    if ((locals.var_guard1631 != 0.0) && (locals.var_guard1635 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_coovlps,)
    }
};
        locals.var_flg_coovlps = assign69640_e106059;

        let (assign69650_e106067, assign69650_e106067_d_n2, assign69650_e106067_d_n7, assign69650_e106067_d_n8, assign69650_e106067_d_n9,) = {
    if ((locals.var_guard1631 != 0.0) && (locals.var_guard1635 != 0.0)) {
        let assign69650_e106065: f64 = (locals.var_vgsi - locals.var_vbsi);
        (assign69650_e106065, 0.0, locals.var_vgsi_dn7, (locals.var_vgsi_dn8 - locals.var_vbsi_dn8), (-locals.var_vbsi_dn9),)
    } else {
        (locals.var_vgbgmt, locals.var_vgbgmt_dn2, locals.var_vgbgmt_dn7, locals.var_vgbgmt_dn8, locals.var_vgbgmt_dn9,)
    }
};
        locals.var_vgbgmt = assign69650_e106067;
        locals.var_vgbgmt_dn2 = assign69650_e106067_d_n2;
        locals.var_vgbgmt_dn7 = assign69650_e106067_d_n7;
        locals.var_vgbgmt_dn8 = assign69650_e106067_d_n8;
        locals.var_vgbgmt_dn9 = assign69650_e106067_d_n9;

        let (assign69660_e106074, assign69660_e106074_d_n0, assign69660_e106074_d_n2, assign69660_e106074_d_n4, assign69660_e106074_d_n5, assign69660_e106074_d_n6, assign69660_e106074_d_n7, assign69660_e106074_d_n8, assign69660_e106074_d_n9, assign69660_e106074_d_n10, assign69660_e106074_d_n11, assign69660_e106074_d_n14,) = {
    if ((locals.var_guard1631 != 0.0) && (locals.var_guard1635 != 0.0)) {
        let assign69660_e106072: f64 = (-locals.var_vbsi);
        (assign69660_e106072, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, (-locals.var_vbsi_dn8), (-locals.var_vbsi_dn9), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn11, locals.var_vxbgmt_dn14,)
    }
};
        locals.var_vxbgmt = assign69660_e106074;
        locals.var_vxbgmt_dn0 = assign69660_e106074_d_n0;
        locals.var_vxbgmt_dn2 = assign69660_e106074_d_n2;
        locals.var_vxbgmt_dn4 = assign69660_e106074_d_n4;
        locals.var_vxbgmt_dn5 = assign69660_e106074_d_n5;
        locals.var_vxbgmt_dn6 = assign69660_e106074_d_n6;
        locals.var_vxbgmt_dn7 = assign69660_e106074_d_n7;
        locals.var_vxbgmt_dn8 = assign69660_e106074_d_n8;
        locals.var_vxbgmt_dn9 = assign69660_e106074_d_n9;
        locals.var_vxbgmt_dn10 = assign69660_e106074_d_n10;
        locals.var_vxbgmt_dn11 = assign69660_e106074_d_n11;
        locals.var_vxbgmt_dn14 = assign69660_e106074_d_n14;

        let (assign69670_e106080,) = {
    if ((locals.var_guard1631 != 0.0) && (locals.var_guard1635 != 0.0)) {
        (locals.var_uc_novers,)
    } else {
        (locals.var_nover_func,)
    }
};
        locals.var_nover_func = assign69670_e106080;

        let (assign69680_e106086, assign69680_e106086_d_n0, assign69680_e106086_d_n2, assign69680_e106086_d_n4, assign69680_e106086_d_n5, assign69680_e106086_d_n6, assign69680_e106086_d_n7, assign69680_e106086_d_n8, assign69680_e106086_d_n9, assign69680_e106086_d_n10, assign69680_e106086_d_n11, assign69680_e106086_d_n14,) = {
    if ((locals.var_guard1631 != 0.0) && (locals.var_guard1635 != 0.0)) {
        (p.p66, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_lover_func, locals.var_lover_func_dn0, locals.var_lover_func_dn2, locals.var_lover_func_dn4, locals.var_lover_func_dn5, locals.var_lover_func_dn6, locals.var_lover_func_dn7, locals.var_lover_func_dn8, locals.var_lover_func_dn9, locals.var_lover_func_dn10, locals.var_lover_func_dn11, locals.var_lover_func_dn14,)
    }
};
        locals.var_lover_func = assign69680_e106086;
        locals.var_lover_func_dn0 = assign69680_e106086_d_n0;
        locals.var_lover_func_dn2 = assign69680_e106086_d_n2;
        locals.var_lover_func_dn4 = assign69680_e106086_d_n4;
        locals.var_lover_func_dn5 = assign69680_e106086_d_n5;
        locals.var_lover_func_dn6 = assign69680_e106086_d_n6;
        locals.var_lover_func_dn7 = assign69680_e106086_d_n7;
        locals.var_lover_func_dn8 = assign69680_e106086_d_n8;
        locals.var_lover_func_dn9 = assign69680_e106086_d_n9;
        locals.var_lover_func_dn10 = assign69680_e106086_d_n10;
        locals.var_lover_func_dn11 = assign69680_e106086_d_n11;
        locals.var_lover_func_dn14 = assign69680_e106086_d_n14;

        let (assign69690_e106092, assign69690_e106092_d_n0, assign69690_e106092_d_n2, assign69690_e106092_d_n4, assign69690_e106092_d_n5, assign69690_e106092_d_n6, assign69690_e106092_d_n7, assign69690_e106092_d_n8, assign69690_e106092_d_n9, assign69690_e106092_d_n10, assign69690_e106092_d_n11, assign69690_e106092_d_n14,) = {
    if ((locals.var_guard1631 != 0.0) && (locals.var_guard1635 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_wdep_func, locals.var_wdep_func_dn0, locals.var_wdep_func_dn2, locals.var_wdep_func_dn4, locals.var_wdep_func_dn5, locals.var_wdep_func_dn6, locals.var_wdep_func_dn7, locals.var_wdep_func_dn8, locals.var_wdep_func_dn9, locals.var_wdep_func_dn10, locals.var_wdep_func_dn11, locals.var_wdep_func_dn14,)
    }
};
        locals.var_wdep_func = assign69690_e106092;
        locals.var_wdep_func_dn0 = assign69690_e106092_d_n0;
        locals.var_wdep_func_dn2 = assign69690_e106092_d_n2;
        locals.var_wdep_func_dn4 = assign69690_e106092_d_n4;
        locals.var_wdep_func_dn5 = assign69690_e106092_d_n5;
        locals.var_wdep_func_dn6 = assign69690_e106092_d_n6;
        locals.var_wdep_func_dn7 = assign69690_e106092_d_n7;
        locals.var_wdep_func_dn8 = assign69690_e106092_d_n8;
        locals.var_wdep_func_dn9 = assign69690_e106092_d_n9;
        locals.var_wdep_func_dn10 = assign69690_e106092_d_n10;
        locals.var_wdep_func_dn11 = assign69690_e106092_d_n11;
        locals.var_wdep_func_dn14 = assign69690_e106092_d_n14;

        let (assign69700_e106098, assign69700_e106098_d_n0, assign69700_e106098_d_n2, assign69700_e106098_d_n4, assign69700_e106098_d_n5, assign69700_e106098_d_n6, assign69700_e106098_d_n7, assign69700_e106098_d_n8, assign69700_e106098_d_n9, assign69700_e106098_d_n10, assign69700_e106098_d_n11, assign69700_e106098_d_n14,) = {
    if ((locals.var_guard1631 != 0.0) && (locals.var_guard1635 != 0.0)) {
        (locals.var_cnst0overs, locals.var_cnst0overs_dn0, locals.var_cnst0overs_dn2, locals.var_cnst0overs_dn4, locals.var_cnst0overs_dn5, locals.var_cnst0overs_dn6, locals.var_cnst0overs_dn7, locals.var_cnst0overs_dn8, locals.var_cnst0overs_dn9, locals.var_cnst0overs_dn10, locals.var_cnst0overs_dn11, locals.var_cnst0overs_dn14,)
    } else {
        (locals.var_cnst0over_func, locals.var_cnst0over_func_dn0, locals.var_cnst0over_func_dn2, locals.var_cnst0over_func_dn4, locals.var_cnst0over_func_dn5, locals.var_cnst0over_func_dn6, locals.var_cnst0over_func_dn7, locals.var_cnst0over_func_dn8, locals.var_cnst0over_func_dn9, locals.var_cnst0over_func_dn10, locals.var_cnst0over_func_dn11, locals.var_cnst0over_func_dn14,)
    }
};
        locals.var_cnst0over_func = assign69700_e106098;
        locals.var_cnst0over_func_dn0 = assign69700_e106098_d_n0;
        locals.var_cnst0over_func_dn2 = assign69700_e106098_d_n2;
        locals.var_cnst0over_func_dn4 = assign69700_e106098_d_n4;
        locals.var_cnst0over_func_dn5 = assign69700_e106098_d_n5;
        locals.var_cnst0over_func_dn6 = assign69700_e106098_d_n6;
        locals.var_cnst0over_func_dn7 = assign69700_e106098_d_n7;
        locals.var_cnst0over_func_dn8 = assign69700_e106098_d_n8;
        locals.var_cnst0over_func_dn9 = assign69700_e106098_d_n9;
        locals.var_cnst0over_func_dn10 = assign69700_e106098_d_n10;
        locals.var_cnst0over_func_dn11 = assign69700_e106098_d_n11;
        locals.var_cnst0over_func_dn14 = assign69700_e106098_d_n14;

        let (assign69710_e106104,) = {
    if ((locals.var_guard1631 != 0.0) && (locals.var_guard1635 != 0.0)) {
        (locals.var_cox0,)
    } else {
        (locals.var_cox0_func,)
    }
};
        locals.var_cox0_func = assign69710_e106104;

        let assign69720_e106123: f64 = if (((((p.p36 == 1.0) && (p.p66 > 0.0)) && (locals.var_uc_novers > 0.0)) && (locals.var_uc_cvdsover != 0.0)) && (p.p55 != 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard1636 = assign69720_e106123;

        let (assign69730_e106132,) = {
    if (((locals.var_guard1632 != 0.0) && (locals.var_guard1631 == 0.0)) && (locals.var_guard1636 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_calcqover,)
    }
};
        locals.var_flg_calcqover = assign69730_e106132;

        let (assign69740_e106143, assign69740_e106143_d_n2, assign69740_e106143_d_n7, assign69740_e106143_d_n8, assign69740_e106143_d_n9,) = {
    if (((locals.var_guard1632 != 0.0) && (locals.var_guard1631 == 0.0)) && (locals.var_guard1636 != 0.0)) {
        let assign69740_e106141: f64 = (locals.var_vgsei - locals.var_vbsei);
        (assign69740_e106141, (locals.var_vgsei_dn2 - locals.var_vbsei_dn2), locals.var_vgsei_dn7, 0.0, (-locals.var_vbsei_dn9),)
    } else {
        (locals.var_vgbgmt, locals.var_vgbgmt_dn2, locals.var_vgbgmt_dn7, locals.var_vgbgmt_dn8, locals.var_vgbgmt_dn9,)
    }
};
        locals.var_vgbgmt = assign69740_e106143;
        locals.var_vgbgmt_dn2 = assign69740_e106143_d_n2;
        locals.var_vgbgmt_dn7 = assign69740_e106143_d_n7;
        locals.var_vgbgmt_dn8 = assign69740_e106143_d_n8;
        locals.var_vgbgmt_dn9 = assign69740_e106143_d_n9;

        let (assign69750_e106153, assign69750_e106153_d_n0, assign69750_e106153_d_n2, assign69750_e106153_d_n4, assign69750_e106153_d_n5, assign69750_e106153_d_n6, assign69750_e106153_d_n7, assign69750_e106153_d_n8, assign69750_e106153_d_n9, assign69750_e106153_d_n10, assign69750_e106153_d_n11, assign69750_e106153_d_n14,) = {
    if (((locals.var_guard1632 != 0.0) && (locals.var_guard1631 == 0.0)) && (locals.var_guard1636 != 0.0)) {
        let assign69750_e106151: f64 = (-locals.var_vbsei);
        (assign69750_e106151, 0.0, (-locals.var_vbsei_dn2), 0.0, 0.0, 0.0, 0.0, 0.0, (-locals.var_vbsei_dn9), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn11, locals.var_vxbgmt_dn14,)
    }
};
        locals.var_vxbgmt = assign69750_e106153;
        locals.var_vxbgmt_dn0 = assign69750_e106153_d_n0;
        locals.var_vxbgmt_dn2 = assign69750_e106153_d_n2;
        locals.var_vxbgmt_dn4 = assign69750_e106153_d_n4;
        locals.var_vxbgmt_dn5 = assign69750_e106153_d_n5;
        locals.var_vxbgmt_dn6 = assign69750_e106153_d_n6;
        locals.var_vxbgmt_dn7 = assign69750_e106153_d_n7;
        locals.var_vxbgmt_dn8 = assign69750_e106153_d_n8;
        locals.var_vxbgmt_dn9 = assign69750_e106153_d_n9;
        locals.var_vxbgmt_dn10 = assign69750_e106153_d_n10;
        locals.var_vxbgmt_dn11 = assign69750_e106153_d_n11;
        locals.var_vxbgmt_dn14 = assign69750_e106153_d_n14;

        let assign69760_e106164: f64 = if (((p.p35 == 1.0) && (p.p63 > 0.0)) && (locals.var_uc_nover > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1637 = assign69760_e106164;

        let (assign69770_e106175,) = {
    if (((locals.var_guard1633 != 0.0) && (!((locals.var_guard1631 != 0.0) || (locals.var_guard1632 != 0.0)))) && (locals.var_guard1637 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_calcqover,)
    }
};
        locals.var_flg_calcqover = assign69770_e106175;

        let (assign69780_e106186,) = {
    if (((locals.var_guard1633 != 0.0) && (!((locals.var_guard1631 != 0.0) || (locals.var_guard1632 != 0.0)))) && (locals.var_guard1637 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_coovlp,)
    }
};
        locals.var_flg_coovlp = assign69780_e106186;

        let (assign69790_e106199, assign69790_e106199_d_n2, assign69790_e106199_d_n7, assign69790_e106199_d_n8, assign69790_e106199_d_n9,) = {
    if (((locals.var_guard1633 != 0.0) && (!((locals.var_guard1631 != 0.0) || (locals.var_guard1632 != 0.0)))) && (locals.var_guard1637 != 0.0)) {
        let assign69790_e106197: f64 = (locals.var_vgsi - locals.var_vbsi);
        (assign69790_e106197, 0.0, locals.var_vgsi_dn7, (locals.var_vgsi_dn8 - locals.var_vbsi_dn8), (-locals.var_vbsi_dn9),)
    } else {
        (locals.var_vgbgmt, locals.var_vgbgmt_dn2, locals.var_vgbgmt_dn7, locals.var_vgbgmt_dn8, locals.var_vgbgmt_dn9,)
    }
};
        locals.var_vgbgmt = assign69790_e106199;
        locals.var_vgbgmt_dn2 = assign69790_e106199_d_n2;
        locals.var_vgbgmt_dn7 = assign69790_e106199_d_n7;
        locals.var_vgbgmt_dn8 = assign69790_e106199_d_n8;
        locals.var_vgbgmt_dn9 = assign69790_e106199_d_n9;

        let (assign69800_e106212, assign69800_e106212_d_n0, assign69800_e106212_d_n2, assign69800_e106212_d_n4, assign69800_e106212_d_n5, assign69800_e106212_d_n6, assign69800_e106212_d_n7, assign69800_e106212_d_n8, assign69800_e106212_d_n9, assign69800_e106212_d_n10, assign69800_e106212_d_n11, assign69800_e106212_d_n14,) = {
    if (((locals.var_guard1633 != 0.0) && (!((locals.var_guard1631 != 0.0) || (locals.var_guard1632 != 0.0)))) && (locals.var_guard1637 != 0.0)) {
        let assign69800_e106210: f64 = (locals.var_vdsi - locals.var_vbsi);
        (assign69800_e106210, 0.0, 0.0, 0.0, 0.0, locals.var_vdsi_dn6, 0.0, (locals.var_vdsi_dn8 - locals.var_vbsi_dn8), (-locals.var_vbsi_dn9), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn11, locals.var_vxbgmt_dn14,)
    }
};
        locals.var_vxbgmt = assign69800_e106212;
        locals.var_vxbgmt_dn0 = assign69800_e106212_d_n0;
        locals.var_vxbgmt_dn2 = assign69800_e106212_d_n2;
        locals.var_vxbgmt_dn4 = assign69800_e106212_d_n4;
        locals.var_vxbgmt_dn5 = assign69800_e106212_d_n5;
        locals.var_vxbgmt_dn6 = assign69800_e106212_d_n6;
        locals.var_vxbgmt_dn7 = assign69800_e106212_d_n7;
        locals.var_vxbgmt_dn8 = assign69800_e106212_d_n8;
        locals.var_vxbgmt_dn9 = assign69800_e106212_d_n9;
        locals.var_vxbgmt_dn10 = assign69800_e106212_d_n10;
        locals.var_vxbgmt_dn11 = assign69800_e106212_d_n11;
        locals.var_vxbgmt_dn14 = assign69800_e106212_d_n14;

        let (assign69810_e106223,) = {
    if (((locals.var_guard1633 != 0.0) && (!((locals.var_guard1631 != 0.0) || (locals.var_guard1632 != 0.0)))) && (locals.var_guard1637 != 0.0)) {
        (locals.var_uc_nover,)
    } else {
        (locals.var_nover_func,)
    }
};
        locals.var_nover_func = assign69810_e106223;

    }

    pub(super) fn stamp_transient_block_249(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign69820_e106238, assign69820_e106238_d_n0, assign69820_e106238_d_n2, assign69820_e106238_d_n4, assign69820_e106238_d_n5, assign69820_e106238_d_n6, assign69820_e106238_d_n7, assign69820_e106238_d_n8, assign69820_e106238_d_n9, assign69820_e106238_d_n10, assign69820_e106238_d_n11, assign69820_e106238_d_n14,) = {
    if (((locals.var_guard1633 != 0.0) && (!((locals.var_guard1631 != 0.0) || (locals.var_guard1632 != 0.0)))) && (locals.var_guard1637 != 0.0)) {
        let assign69820_e106235: f64 = (p.p64 * p.p55);
        let assign69820_e106236: f64 = (p.p63 + assign69820_e106235);
        (assign69820_e106236, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_lover_func, locals.var_lover_func_dn0, locals.var_lover_func_dn2, locals.var_lover_func_dn4, locals.var_lover_func_dn5, locals.var_lover_func_dn6, locals.var_lover_func_dn7, locals.var_lover_func_dn8, locals.var_lover_func_dn9, locals.var_lover_func_dn10, locals.var_lover_func_dn11, locals.var_lover_func_dn14,)
    }
};
        locals.var_lover_func = assign69820_e106238;
        locals.var_lover_func_dn0 = assign69820_e106238_d_n0;
        locals.var_lover_func_dn2 = assign69820_e106238_d_n2;
        locals.var_lover_func_dn4 = assign69820_e106238_d_n4;
        locals.var_lover_func_dn5 = assign69820_e106238_d_n5;
        locals.var_lover_func_dn6 = assign69820_e106238_d_n6;
        locals.var_lover_func_dn7 = assign69820_e106238_d_n7;
        locals.var_lover_func_dn8 = assign69820_e106238_d_n8;
        locals.var_lover_func_dn9 = assign69820_e106238_d_n9;
        locals.var_lover_func_dn10 = assign69820_e106238_d_n10;
        locals.var_lover_func_dn11 = assign69820_e106238_d_n11;
        locals.var_lover_func_dn14 = assign69820_e106238_d_n14;

        let (assign69830_e106249, assign69830_e106249_d_n0, assign69830_e106249_d_n2, assign69830_e106249_d_n4, assign69830_e106249_d_n5, assign69830_e106249_d_n6, assign69830_e106249_d_n7, assign69830_e106249_d_n8, assign69830_e106249_d_n9, assign69830_e106249_d_n10, assign69830_e106249_d_n11, assign69830_e106249_d_n14,) = {
    if (((locals.var_guard1633 != 0.0) && (!((locals.var_guard1631 != 0.0) || (locals.var_guard1632 != 0.0)))) && (locals.var_guard1637 != 0.0)) {
        (locals.var_wdep, locals.var_wdep_dn0, locals.var_wdep_dn2, locals.var_wdep_dn4, locals.var_wdep_dn5, locals.var_wdep_dn6, locals.var_wdep_dn7, locals.var_wdep_dn8, locals.var_wdep_dn9, locals.var_wdep_dn10, locals.var_wdep_dn11, locals.var_wdep_dn14,)
    } else {
        (locals.var_wdep_func, locals.var_wdep_func_dn0, locals.var_wdep_func_dn2, locals.var_wdep_func_dn4, locals.var_wdep_func_dn5, locals.var_wdep_func_dn6, locals.var_wdep_func_dn7, locals.var_wdep_func_dn8, locals.var_wdep_func_dn9, locals.var_wdep_func_dn10, locals.var_wdep_func_dn11, locals.var_wdep_func_dn14,)
    }
};
        locals.var_wdep_func = assign69830_e106249;
        locals.var_wdep_func_dn0 = assign69830_e106249_d_n0;
        locals.var_wdep_func_dn2 = assign69830_e106249_d_n2;
        locals.var_wdep_func_dn4 = assign69830_e106249_d_n4;
        locals.var_wdep_func_dn5 = assign69830_e106249_d_n5;
        locals.var_wdep_func_dn6 = assign69830_e106249_d_n6;
        locals.var_wdep_func_dn7 = assign69830_e106249_d_n7;
        locals.var_wdep_func_dn8 = assign69830_e106249_d_n8;
        locals.var_wdep_func_dn9 = assign69830_e106249_d_n9;
        locals.var_wdep_func_dn10 = assign69830_e106249_d_n10;
        locals.var_wdep_func_dn11 = assign69830_e106249_d_n11;
        locals.var_wdep_func_dn14 = assign69830_e106249_d_n14;

        let (assign69840_e106260, assign69840_e106260_d_n0, assign69840_e106260_d_n2, assign69840_e106260_d_n4, assign69840_e106260_d_n5, assign69840_e106260_d_n6, assign69840_e106260_d_n7, assign69840_e106260_d_n8, assign69840_e106260_d_n9, assign69840_e106260_d_n10, assign69840_e106260_d_n11, assign69840_e106260_d_n14,) = {
    if (((locals.var_guard1633 != 0.0) && (!((locals.var_guard1631 != 0.0) || (locals.var_guard1632 != 0.0)))) && (locals.var_guard1637 != 0.0)) {
        (locals.var_cnst0over, locals.var_cnst0over_dn0, locals.var_cnst0over_dn2, locals.var_cnst0over_dn4, locals.var_cnst0over_dn5, locals.var_cnst0over_dn6, locals.var_cnst0over_dn7, locals.var_cnst0over_dn8, locals.var_cnst0over_dn9, locals.var_cnst0over_dn10, locals.var_cnst0over_dn11, locals.var_cnst0over_dn14,)
    } else {
        (locals.var_cnst0over_func, locals.var_cnst0over_func_dn0, locals.var_cnst0over_func_dn2, locals.var_cnst0over_func_dn4, locals.var_cnst0over_func_dn5, locals.var_cnst0over_func_dn6, locals.var_cnst0over_func_dn7, locals.var_cnst0over_func_dn8, locals.var_cnst0over_func_dn9, locals.var_cnst0over_func_dn10, locals.var_cnst0over_func_dn11, locals.var_cnst0over_func_dn14,)
    }
};
        locals.var_cnst0over_func = assign69840_e106260;
        locals.var_cnst0over_func_dn0 = assign69840_e106260_d_n0;
        locals.var_cnst0over_func_dn2 = assign69840_e106260_d_n2;
        locals.var_cnst0over_func_dn4 = assign69840_e106260_d_n4;
        locals.var_cnst0over_func_dn5 = assign69840_e106260_d_n5;
        locals.var_cnst0over_func_dn6 = assign69840_e106260_d_n6;
        locals.var_cnst0over_func_dn7 = assign69840_e106260_d_n7;
        locals.var_cnst0over_func_dn8 = assign69840_e106260_d_n8;
        locals.var_cnst0over_func_dn9 = assign69840_e106260_d_n9;
        locals.var_cnst0over_func_dn10 = assign69840_e106260_d_n10;
        locals.var_cnst0over_func_dn11 = assign69840_e106260_d_n11;
        locals.var_cnst0over_func_dn14 = assign69840_e106260_d_n14;

        let (assign69850_e106271,) = {
    if (((locals.var_guard1633 != 0.0) && (!((locals.var_guard1631 != 0.0) || (locals.var_guard1632 != 0.0)))) && (locals.var_guard1637 != 0.0)) {
        (locals.var_coxb0,)
    } else {
        (locals.var_cox0_func,)
    }
};
        locals.var_cox0_func = assign69850_e106271;

        let (assign69860_e106283, assign69860_e106283_d_n0, assign69860_e106283_d_n2, assign69860_e106283_d_n4, assign69860_e106283_d_n5, assign69860_e106283_d_n6, assign69860_e106283_d_n7, assign69860_e106283_d_n8, assign69860_e106283_d_n9, assign69860_e106283_d_n10, assign69860_e106283_d_n11, assign69860_e106283_d_n14,) = {
    if (((locals.var_guard1633 != 0.0) && (!((locals.var_guard1631 != 0.0) || (locals.var_guard1632 != 0.0)))) && (locals.var_guard1637 != 0.0)) {
        let assign69860_e106281: f64 = (-locals.var_lover_func);
        (assign69860_e106281, (-locals.var_lover_func_dn0), (-locals.var_lover_func_dn2), (-locals.var_lover_func_dn4), (-locals.var_lover_func_dn5), (-locals.var_lover_func_dn6), (-locals.var_lover_func_dn7), (-locals.var_lover_func_dn8), (-locals.var_lover_func_dn9), (-locals.var_lover_func_dn10), (-locals.var_lover_func_dn11), (-locals.var_lover_func_dn14),)
    } else {
        (locals.var_lover_func, locals.var_lover_func_dn0, locals.var_lover_func_dn2, locals.var_lover_func_dn4, locals.var_lover_func_dn5, locals.var_lover_func_dn6, locals.var_lover_func_dn7, locals.var_lover_func_dn8, locals.var_lover_func_dn9, locals.var_lover_func_dn10, locals.var_lover_func_dn11, locals.var_lover_func_dn14,)
    }
};
        locals.var_lover_func = assign69860_e106283;
        locals.var_lover_func_dn0 = assign69860_e106283_d_n0;
        locals.var_lover_func_dn2 = assign69860_e106283_d_n2;
        locals.var_lover_func_dn4 = assign69860_e106283_d_n4;
        locals.var_lover_func_dn5 = assign69860_e106283_d_n5;
        locals.var_lover_func_dn6 = assign69860_e106283_d_n6;
        locals.var_lover_func_dn7 = assign69860_e106283_d_n7;
        locals.var_lover_func_dn8 = assign69860_e106283_d_n8;
        locals.var_lover_func_dn9 = assign69860_e106283_d_n9;
        locals.var_lover_func_dn10 = assign69860_e106283_d_n10;
        locals.var_lover_func_dn11 = assign69860_e106283_d_n11;
        locals.var_lover_func_dn14 = assign69860_e106283_d_n14;

        let assign69870_e106294: f64 = if (((locals.var_lover_func < 0.0) && (p.p432 > 0.0)) && (p.p55 == 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard1638 = assign69870_e106294;

        let (assign69880_e106308, assign69880_e106308_d_n0, assign69880_e106308_d_n2, assign69880_e106308_d_n4, assign69880_e106308_d_n5, assign69880_e106308_d_n6, assign69880_e106308_d_n7, assign69880_e106308_d_n8, assign69880_e106308_d_n9, assign69880_e106308_d_n10, assign69880_e106308_d_n11, assign69880_e106308_d_n14,) = {
    if ((((locals.var_guard1633 != 0.0) && (!((locals.var_guard1631 != 0.0) || (locals.var_guard1632 != 0.0)))) && (locals.var_guard1637 != 0.0)) && (locals.var_guard1638 != 0.0)) {
        let assign69880_e106306: f64 = (-locals.var_lover_func);
        (assign69880_e106306, (-locals.var_lover_func_dn0), (-locals.var_lover_func_dn2), (-locals.var_lover_func_dn4), (-locals.var_lover_func_dn5), (-locals.var_lover_func_dn6), (-locals.var_lover_func_dn7), (-locals.var_lover_func_dn8), (-locals.var_lover_func_dn9), (-locals.var_lover_func_dn10), (-locals.var_lover_func_dn11), (-locals.var_lover_func_dn14),)
    } else {
        (locals.var_lover_func, locals.var_lover_func_dn0, locals.var_lover_func_dn2, locals.var_lover_func_dn4, locals.var_lover_func_dn5, locals.var_lover_func_dn6, locals.var_lover_func_dn7, locals.var_lover_func_dn8, locals.var_lover_func_dn9, locals.var_lover_func_dn10, locals.var_lover_func_dn11, locals.var_lover_func_dn14,)
    }
};
        locals.var_lover_func = assign69880_e106308;
        locals.var_lover_func_dn0 = assign69880_e106308_d_n0;
        locals.var_lover_func_dn2 = assign69880_e106308_d_n2;
        locals.var_lover_func_dn4 = assign69880_e106308_d_n4;
        locals.var_lover_func_dn5 = assign69880_e106308_d_n5;
        locals.var_lover_func_dn6 = assign69880_e106308_d_n6;
        locals.var_lover_func_dn7 = assign69880_e106308_d_n7;
        locals.var_lover_func_dn8 = assign69880_e106308_d_n8;
        locals.var_lover_func_dn9 = assign69880_e106308_d_n9;
        locals.var_lover_func_dn10 = assign69880_e106308_d_n10;
        locals.var_lover_func_dn11 = assign69880_e106308_d_n11;
        locals.var_lover_func_dn14 = assign69880_e106308_d_n14;

        let (assign69890_e106321, assign69890_e106321_d_n0, assign69890_e106321_d_n2, assign69890_e106321_d_n4, assign69890_e106321_d_n5, assign69890_e106321_d_n6, assign69890_e106321_d_n7, assign69890_e106321_d_n8, assign69890_e106321_d_n9, assign69890_e106321_d_n10, assign69890_e106321_d_n11, assign69890_e106321_d_n14,) = {
    if ((((locals.var_guard1633 != 0.0) && (!((locals.var_guard1631 != 0.0) || (locals.var_guard1632 != 0.0)))) && (locals.var_guard1637 != 0.0)) && (locals.var_guard1638 != 0.0)) {
        (p.p63, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign69890_e106321;
        locals.var_t1_dn0 = assign69890_e106321_d_n0;
        locals.var_t1_dn2 = assign69890_e106321_d_n2;
        locals.var_t1_dn4 = assign69890_e106321_d_n4;
        locals.var_t1_dn5 = assign69890_e106321_d_n5;
        locals.var_t1_dn6 = assign69890_e106321_d_n6;
        locals.var_t1_dn7 = assign69890_e106321_d_n7;
        locals.var_t1_dn8 = assign69890_e106321_d_n8;
        locals.var_t1_dn9 = assign69890_e106321_d_n9;
        locals.var_t1_dn10 = assign69890_e106321_d_n10;
        locals.var_t1_dn11 = assign69890_e106321_d_n11;
        locals.var_t1_dn14 = assign69890_e106321_d_n14;

        let (assign69900_e106340, assign69900_e106340_d_n0, assign69900_e106340_d_n2, assign69900_e106340_d_n4, assign69900_e106340_d_n5, assign69900_e106340_d_n6, assign69900_e106340_d_n7, assign69900_e106340_d_n8, assign69900_e106340_d_n9, assign69900_e106340_d_n10, assign69900_e106340_d_n11, assign69900_e106340_d_n14,) = {
    if ((((locals.var_guard1633 != 0.0) && (!((locals.var_guard1631 != 0.0) || (locals.var_guard1632 != 0.0)))) && (locals.var_guard1637 != 0.0)) && (locals.var_guard1638 != 0.0)) {
        let assign69900_e106334: f64 = (locals.var_t1 * locals.var_t1);
        let assign69900_e106336: f64 = (assign69900_e106334 / locals.var_kjunc);
        let assign69900_e106338: f64 = (assign69900_e106336 - p.p137);
        (assign69900_e106338, (((((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) * locals.var_kjunc) - (assign69900_e106334 * locals.var_kjunc_dn0)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) * locals.var_kjunc) - (assign69900_e106334 * locals.var_kjunc_dn2)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) * locals.var_kjunc) - (assign69900_e106334 * locals.var_kjunc_dn4)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) * locals.var_kjunc) - (assign69900_e106334 * locals.var_kjunc_dn5)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) * locals.var_kjunc) - (assign69900_e106334 * locals.var_kjunc_dn6)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) * locals.var_kjunc) - (assign69900_e106334 * locals.var_kjunc_dn7)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) * locals.var_kjunc) - (assign69900_e106334 * locals.var_kjunc_dn8)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) * locals.var_kjunc) - (assign69900_e106334 * locals.var_kjunc_dn9)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) * locals.var_kjunc) - (assign69900_e106334 * locals.var_kjunc_dn10)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) * locals.var_kjunc) - (assign69900_e106334 * locals.var_kjunc_dn11)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14)) * locals.var_kjunc) - (assign69900_e106334 * locals.var_kjunc_dn14)) / (locals.var_kjunc * locals.var_kjunc)),)
    } else {
        (locals.var_vxb_lim, locals.var_vxb_lim_dn0, locals.var_vxb_lim_dn2, locals.var_vxb_lim_dn4, locals.var_vxb_lim_dn5, locals.var_vxb_lim_dn6, locals.var_vxb_lim_dn7, locals.var_vxb_lim_dn8, locals.var_vxb_lim_dn9, locals.var_vxb_lim_dn10, locals.var_vxb_lim_dn11, locals.var_vxb_lim_dn14,)
    }
};
        locals.var_vxb_lim = assign69900_e106340;
        locals.var_vxb_lim_dn0 = assign69900_e106340_d_n0;
        locals.var_vxb_lim_dn2 = assign69900_e106340_d_n2;
        locals.var_vxb_lim_dn4 = assign69900_e106340_d_n4;
        locals.var_vxb_lim_dn5 = assign69900_e106340_d_n5;
        locals.var_vxb_lim_dn6 = assign69900_e106340_d_n6;
        locals.var_vxb_lim_dn7 = assign69900_e106340_d_n7;
        locals.var_vxb_lim_dn8 = assign69900_e106340_d_n8;
        locals.var_vxb_lim_dn9 = assign69900_e106340_d_n9;
        locals.var_vxb_lim_dn10 = assign69900_e106340_d_n10;
        locals.var_vxb_lim_dn11 = assign69900_e106340_d_n11;
        locals.var_vxb_lim_dn14 = assign69900_e106340_d_n14;

        let assign69910_e106343: f64 = if p.p113 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1639 = assign69910_e106343;

        let assign69920_e106350: f64 = if ((locals.var_vxbgmt == 0.0) || (p.p113 <= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1640 = assign69920_e106350;

        let (assign69930_e106367, assign69930_e106367_d_n0, assign69930_e106367_d_n2, assign69930_e106367_d_n4, assign69930_e106367_d_n5, assign69930_e106367_d_n6, assign69930_e106367_d_n7, assign69930_e106367_d_n8, assign69930_e106367_d_n9, assign69930_e106367_d_n10, assign69930_e106367_d_n11, assign69930_e106367_d_n14,) = {
    if ((((((locals.var_guard1633 != 0.0) && (!((locals.var_guard1631 != 0.0) || (locals.var_guard1632 != 0.0)))) && (locals.var_guard1637 != 0.0)) && (locals.var_guard1638 != 0.0)) && (locals.var_guard1639 != 0.0)) && (locals.var_guard1640 != 0.0)) {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn11, locals.var_vxbgmt_dn14,)
    } else {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn11, locals.var_vxbgmt_dn14,)
    }
};
        locals.var_vxbgmt = assign69930_e106367;
        locals.var_vxbgmt_dn0 = assign69930_e106367_d_n0;
        locals.var_vxbgmt_dn2 = assign69930_e106367_d_n2;
        locals.var_vxbgmt_dn4 = assign69930_e106367_d_n4;
        locals.var_vxbgmt_dn5 = assign69930_e106367_d_n5;
        locals.var_vxbgmt_dn6 = assign69930_e106367_d_n6;
        locals.var_vxbgmt_dn7 = assign69930_e106367_d_n7;
        locals.var_vxbgmt_dn8 = assign69930_e106367_d_n8;
        locals.var_vxbgmt_dn9 = assign69930_e106367_d_n9;
        locals.var_vxbgmt_dn10 = assign69930_e106367_d_n10;
        locals.var_vxbgmt_dn11 = assign69930_e106367_d_n11;
        locals.var_vxbgmt_dn14 = assign69930_e106367_d_n14;

        let (assign69940_e106391, assign69940_e106391_d_n0, assign69940_e106391_d_n2, assign69940_e106391_d_n4, assign69940_e106391_d_n5, assign69940_e106391_d_n6, assign69940_e106391_d_n7, assign69940_e106391_d_n8, assign69940_e106391_d_n9, assign69940_e106391_d_n10, assign69940_e106391_d_n11, assign69940_e106391_d_n14,) = {
    if ((((((locals.var_guard1633 != 0.0) && (!((locals.var_guard1631 != 0.0) || (locals.var_guard1632 != 0.0)))) && (locals.var_guard1637 != 0.0)) && (locals.var_guard1638 != 0.0)) && (locals.var_guard1639 != 0.0)) && (locals.var_guard1640 == 0.0)) {
        let (assign69940_e106389,) = {
            if (locals.var_vxbgmt < 0.0) {
                let assign69940_e106387: f64 = (-1.0);
                (assign69940_e106387,)
            } else {
                (1.0,)
            }
        };
        (assign69940_e106389, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf3, locals.var_tmf3_dn0, locals.var_tmf3_dn2, locals.var_tmf3_dn4, locals.var_tmf3_dn5, locals.var_tmf3_dn6, locals.var_tmf3_dn7, locals.var_tmf3_dn8, locals.var_tmf3_dn9, locals.var_tmf3_dn10, locals.var_tmf3_dn11, locals.var_tmf3_dn14,)
    }
};
        locals.var_tmf3 = assign69940_e106391;
        locals.var_tmf3_dn0 = assign69940_e106391_d_n0;
        locals.var_tmf3_dn2 = assign69940_e106391_d_n2;
        locals.var_tmf3_dn4 = assign69940_e106391_d_n4;
        locals.var_tmf3_dn5 = assign69940_e106391_d_n5;
        locals.var_tmf3_dn6 = assign69940_e106391_d_n6;
        locals.var_tmf3_dn7 = assign69940_e106391_d_n7;
        locals.var_tmf3_dn8 = assign69940_e106391_d_n8;
        locals.var_tmf3_dn9 = assign69940_e106391_d_n9;
        locals.var_tmf3_dn10 = assign69940_e106391_d_n10;
        locals.var_tmf3_dn11 = assign69940_e106391_d_n11;
        locals.var_tmf3_dn14 = assign69940_e106391_d_n14;

        let (assign69950_e106411, assign69950_e106411_d_n0, assign69950_e106411_d_n2, assign69950_e106411_d_n4, assign69950_e106411_d_n5, assign69950_e106411_d_n6, assign69950_e106411_d_n7, assign69950_e106411_d_n8, assign69950_e106411_d_n9, assign69950_e106411_d_n10, assign69950_e106411_d_n11, assign69950_e106411_d_n14,) = {
    if ((((((locals.var_guard1633 != 0.0) && (!((locals.var_guard1631 != 0.0) || (locals.var_guard1632 != 0.0)))) && (locals.var_guard1637 != 0.0)) && (locals.var_guard1638 != 0.0)) && (locals.var_guard1639 != 0.0)) && (locals.var_guard1640 == 0.0)) {
        let assign69950_e106409: f64 = (locals.var_tmf3 * locals.var_vxbgmt);
        (assign69950_e106409, ((locals.var_tmf3_dn0 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn0)), ((locals.var_tmf3_dn2 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn2)), ((locals.var_tmf3_dn4 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn4)), ((locals.var_tmf3_dn5 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn5)), ((locals.var_tmf3_dn6 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn6)), ((locals.var_tmf3_dn7 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn7)), ((locals.var_tmf3_dn8 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn8)), ((locals.var_tmf3_dn9 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn9)), ((locals.var_tmf3_dn10 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn10)), ((locals.var_tmf3_dn11 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn11)), ((locals.var_tmf3_dn14 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn14)),)
    } else {
        (locals.var_tmf4, locals.var_tmf4_dn0, locals.var_tmf4_dn2, locals.var_tmf4_dn4, locals.var_tmf4_dn5, locals.var_tmf4_dn6, locals.var_tmf4_dn7, locals.var_tmf4_dn8, locals.var_tmf4_dn9, locals.var_tmf4_dn10, locals.var_tmf4_dn11, locals.var_tmf4_dn14,)
    }
};
        locals.var_tmf4 = assign69950_e106411;
        locals.var_tmf4_dn0 = assign69950_e106411_d_n0;
        locals.var_tmf4_dn2 = assign69950_e106411_d_n2;
        locals.var_tmf4_dn4 = assign69950_e106411_d_n4;
        locals.var_tmf4_dn5 = assign69950_e106411_d_n5;
        locals.var_tmf4_dn6 = assign69950_e106411_d_n6;
        locals.var_tmf4_dn7 = assign69950_e106411_d_n7;
        locals.var_tmf4_dn8 = assign69950_e106411_d_n8;
        locals.var_tmf4_dn9 = assign69950_e106411_d_n9;
        locals.var_tmf4_dn10 = assign69950_e106411_d_n10;
        locals.var_tmf4_dn11 = assign69950_e106411_d_n11;
        locals.var_tmf4_dn14 = assign69950_e106411_d_n14;

        let (assign69960_e106435, assign69960_e106435_d_n0, assign69960_e106435_d_n2, assign69960_e106435_d_n4, assign69960_e106435_d_n5, assign69960_e106435_d_n6, assign69960_e106435_d_n7, assign69960_e106435_d_n8, assign69960_e106435_d_n9, assign69960_e106435_d_n10, assign69960_e106435_d_n11, assign69960_e106435_d_n14,) = {
    if ((((((locals.var_guard1633 != 0.0) && (!((locals.var_guard1631 != 0.0) || (locals.var_guard1632 != 0.0)))) && (locals.var_guard1637 != 0.0)) && (locals.var_guard1638 != 0.0)) && (locals.var_guard1639 != 0.0)) && (locals.var_guard1640 == 0.0)) {
        let assign69960_e106430: f64 = (locals.var_tmf4 / locals.var_vxb_lim);
        let assign69960_e106432: f64 = (assign69960_e106430).powf(p.p113);
        let assign69960_e106433: f64 = (1.0 + assign69960_e106432);
        (assign69960_e106433, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign69960_e106430).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn0 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn0)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign69960_e106432 * (p.p113 * ((((locals.var_tmf4_dn0 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn0)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign69960_e106430))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign69960_e106430).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn2 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn2)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign69960_e106432 * (p.p113 * ((((locals.var_tmf4_dn2 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn2)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign69960_e106430))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign69960_e106430).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn4 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn4)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign69960_e106432 * (p.p113 * ((((locals.var_tmf4_dn4 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn4)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign69960_e106430))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign69960_e106430).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn5 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn5)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign69960_e106432 * (p.p113 * ((((locals.var_tmf4_dn5 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn5)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign69960_e106430))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign69960_e106430).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn6 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn6)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign69960_e106432 * (p.p113 * ((((locals.var_tmf4_dn6 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn6)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign69960_e106430))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign69960_e106430).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn7 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn7)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign69960_e106432 * (p.p113 * ((((locals.var_tmf4_dn7 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn7)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign69960_e106430))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign69960_e106430).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn8 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn8)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign69960_e106432 * (p.p113 * ((((locals.var_tmf4_dn8 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn8)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign69960_e106430))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign69960_e106430).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn9 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn9)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign69960_e106432 * (p.p113 * ((((locals.var_tmf4_dn9 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn9)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign69960_e106430))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign69960_e106430).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn10 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn10)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign69960_e106432 * (p.p113 * ((((locals.var_tmf4_dn10 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn10)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign69960_e106430))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign69960_e106430).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn11 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn11)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign69960_e106432 * (p.p113 * ((((locals.var_tmf4_dn11 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn11)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign69960_e106430))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign69960_e106430).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn14 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn14)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign69960_e106432 * (p.p113 * ((((locals.var_tmf4_dn14 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn14)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign69960_e106430))) },)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign69960_e106435;
        locals.var_tmf1_dn0 = assign69960_e106435_d_n0;
        locals.var_tmf1_dn2 = assign69960_e106435_d_n2;
        locals.var_tmf1_dn4 = assign69960_e106435_d_n4;
        locals.var_tmf1_dn5 = assign69960_e106435_d_n5;
        locals.var_tmf1_dn6 = assign69960_e106435_d_n6;
        locals.var_tmf1_dn7 = assign69960_e106435_d_n7;
        locals.var_tmf1_dn8 = assign69960_e106435_d_n8;
        locals.var_tmf1_dn9 = assign69960_e106435_d_n9;
        locals.var_tmf1_dn10 = assign69960_e106435_d_n10;
        locals.var_tmf1_dn11 = assign69960_e106435_d_n11;
        locals.var_tmf1_dn14 = assign69960_e106435_d_n14;

        let (assign69970_e106457, assign69970_e106457_d_n0, assign69970_e106457_d_n2, assign69970_e106457_d_n4, assign69970_e106457_d_n5, assign69970_e106457_d_n6, assign69970_e106457_d_n7, assign69970_e106457_d_n8, assign69970_e106457_d_n9, assign69970_e106457_d_n10, assign69970_e106457_d_n11, assign69970_e106457_d_n14,) = {
    if ((((((locals.var_guard1633 != 0.0) && (!((locals.var_guard1631 != 0.0) || (locals.var_guard1632 != 0.0)))) && (locals.var_guard1637 != 0.0)) && (locals.var_guard1638 != 0.0)) && (locals.var_guard1639 != 0.0)) && (locals.var_guard1640 == 0.0)) {
        let assign69970_e106454: f64 = (1.0 / p.p113);
        let assign69970_e106455: f64 = (locals.var_tmf1).powf(assign69970_e106454);
        (assign69970_e106455, if 0.0 == 0.0 && ((assign69970_e106454) as f64).is_finite() && ((assign69970_e106454) as f64).fract() == 0.0 { if assign69970_e106454 == 0.0 { 0.0 } else { (assign69970_e106454 * ((locals.var_tmf1).powf(assign69970_e106454 - 1.0) * locals.var_tmf1_dn0)) } } else { (assign69970_e106455 * (assign69970_e106454 * (locals.var_tmf1_dn0 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign69970_e106454) as f64).is_finite() && ((assign69970_e106454) as f64).fract() == 0.0 { if assign69970_e106454 == 0.0 { 0.0 } else { (assign69970_e106454 * ((locals.var_tmf1).powf(assign69970_e106454 - 1.0) * locals.var_tmf1_dn2)) } } else { (assign69970_e106455 * (assign69970_e106454 * (locals.var_tmf1_dn2 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign69970_e106454) as f64).is_finite() && ((assign69970_e106454) as f64).fract() == 0.0 { if assign69970_e106454 == 0.0 { 0.0 } else { (assign69970_e106454 * ((locals.var_tmf1).powf(assign69970_e106454 - 1.0) * locals.var_tmf1_dn4)) } } else { (assign69970_e106455 * (assign69970_e106454 * (locals.var_tmf1_dn4 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign69970_e106454) as f64).is_finite() && ((assign69970_e106454) as f64).fract() == 0.0 { if assign69970_e106454 == 0.0 { 0.0 } else { (assign69970_e106454 * ((locals.var_tmf1).powf(assign69970_e106454 - 1.0) * locals.var_tmf1_dn5)) } } else { (assign69970_e106455 * (assign69970_e106454 * (locals.var_tmf1_dn5 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign69970_e106454) as f64).is_finite() && ((assign69970_e106454) as f64).fract() == 0.0 { if assign69970_e106454 == 0.0 { 0.0 } else { (assign69970_e106454 * ((locals.var_tmf1).powf(assign69970_e106454 - 1.0) * locals.var_tmf1_dn6)) } } else { (assign69970_e106455 * (assign69970_e106454 * (locals.var_tmf1_dn6 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign69970_e106454) as f64).is_finite() && ((assign69970_e106454) as f64).fract() == 0.0 { if assign69970_e106454 == 0.0 { 0.0 } else { (assign69970_e106454 * ((locals.var_tmf1).powf(assign69970_e106454 - 1.0) * locals.var_tmf1_dn7)) } } else { (assign69970_e106455 * (assign69970_e106454 * (locals.var_tmf1_dn7 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign69970_e106454) as f64).is_finite() && ((assign69970_e106454) as f64).fract() == 0.0 { if assign69970_e106454 == 0.0 { 0.0 } else { (assign69970_e106454 * ((locals.var_tmf1).powf(assign69970_e106454 - 1.0) * locals.var_tmf1_dn8)) } } else { (assign69970_e106455 * (assign69970_e106454 * (locals.var_tmf1_dn8 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign69970_e106454) as f64).is_finite() && ((assign69970_e106454) as f64).fract() == 0.0 { if assign69970_e106454 == 0.0 { 0.0 } else { (assign69970_e106454 * ((locals.var_tmf1).powf(assign69970_e106454 - 1.0) * locals.var_tmf1_dn9)) } } else { (assign69970_e106455 * (assign69970_e106454 * (locals.var_tmf1_dn9 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign69970_e106454) as f64).is_finite() && ((assign69970_e106454) as f64).fract() == 0.0 { if assign69970_e106454 == 0.0 { 0.0 } else { (assign69970_e106454 * ((locals.var_tmf1).powf(assign69970_e106454 - 1.0) * locals.var_tmf1_dn10)) } } else { (assign69970_e106455 * (assign69970_e106454 * (locals.var_tmf1_dn10 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign69970_e106454) as f64).is_finite() && ((assign69970_e106454) as f64).fract() == 0.0 { if assign69970_e106454 == 0.0 { 0.0 } else { (assign69970_e106454 * ((locals.var_tmf1).powf(assign69970_e106454 - 1.0) * locals.var_tmf1_dn11)) } } else { (assign69970_e106455 * (assign69970_e106454 * (locals.var_tmf1_dn11 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign69970_e106454) as f64).is_finite() && ((assign69970_e106454) as f64).fract() == 0.0 { if assign69970_e106454 == 0.0 { 0.0 } else { (assign69970_e106454 * ((locals.var_tmf1).powf(assign69970_e106454 - 1.0) * locals.var_tmf1_dn14)) } } else { (assign69970_e106455 * (assign69970_e106454 * (locals.var_tmf1_dn14 / locals.var_tmf1))) },)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign69970_e106457;
        locals.var_tmf2_dn0 = assign69970_e106457_d_n0;
        locals.var_tmf2_dn2 = assign69970_e106457_d_n2;
        locals.var_tmf2_dn4 = assign69970_e106457_d_n4;
        locals.var_tmf2_dn5 = assign69970_e106457_d_n5;
        locals.var_tmf2_dn6 = assign69970_e106457_d_n6;
        locals.var_tmf2_dn7 = assign69970_e106457_d_n7;
        locals.var_tmf2_dn8 = assign69970_e106457_d_n8;
        locals.var_tmf2_dn9 = assign69970_e106457_d_n9;
        locals.var_tmf2_dn10 = assign69970_e106457_d_n10;
        locals.var_tmf2_dn11 = assign69970_e106457_d_n11;
        locals.var_tmf2_dn14 = assign69970_e106457_d_n14;

        let (assign69980_e106479, assign69980_e106479_d_n0, assign69980_e106479_d_n2, assign69980_e106479_d_n4, assign69980_e106479_d_n5, assign69980_e106479_d_n6, assign69980_e106479_d_n7, assign69980_e106479_d_n8, assign69980_e106479_d_n9, assign69980_e106479_d_n10, assign69980_e106479_d_n11, assign69980_e106479_d_n14,) = {
    if ((((((locals.var_guard1633 != 0.0) && (!((locals.var_guard1631 != 0.0) || (locals.var_guard1632 != 0.0)))) && (locals.var_guard1637 != 0.0)) && (locals.var_guard1638 != 0.0)) && (locals.var_guard1639 != 0.0)) && (locals.var_guard1640 == 0.0)) {
        let assign69980_e106475: f64 = (locals.var_tmf3 * locals.var_tmf4);
        let assign69980_e106477: f64 = (assign69980_e106475 / locals.var_tmf2);
        (assign69980_e106477, (((((locals.var_tmf3_dn0 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn0)) * locals.var_tmf2) - (assign69980_e106475 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn2 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn2)) * locals.var_tmf2) - (assign69980_e106475 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn4 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn4)) * locals.var_tmf2) - (assign69980_e106475 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn5 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn5)) * locals.var_tmf2) - (assign69980_e106475 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn6 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn6)) * locals.var_tmf2) - (assign69980_e106475 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn7 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn7)) * locals.var_tmf2) - (assign69980_e106475 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn8 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn8)) * locals.var_tmf2) - (assign69980_e106475 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn9 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn9)) * locals.var_tmf2) - (assign69980_e106475 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn10 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn10)) * locals.var_tmf2) - (assign69980_e106475 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn11 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn11)) * locals.var_tmf2) - (assign69980_e106475 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn14 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn14)) * locals.var_tmf2) - (assign69980_e106475 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2)),)
    } else {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn11, locals.var_vxbgmt_dn14,)
    }
};
        locals.var_vxbgmt = assign69980_e106479;
        locals.var_vxbgmt_dn0 = assign69980_e106479_d_n0;
        locals.var_vxbgmt_dn2 = assign69980_e106479_d_n2;
        locals.var_vxbgmt_dn4 = assign69980_e106479_d_n4;
        locals.var_vxbgmt_dn5 = assign69980_e106479_d_n5;
        locals.var_vxbgmt_dn6 = assign69980_e106479_d_n6;
        locals.var_vxbgmt_dn7 = assign69980_e106479_d_n7;
        locals.var_vxbgmt_dn8 = assign69980_e106479_d_n8;
        locals.var_vxbgmt_dn9 = assign69980_e106479_d_n9;
        locals.var_vxbgmt_dn10 = assign69980_e106479_d_n10;
        locals.var_vxbgmt_dn11 = assign69980_e106479_d_n11;
        locals.var_vxbgmt_dn14 = assign69980_e106479_d_n14;

        let (assign69990_e106507, assign69990_e106507_d_n0, assign69990_e106507_d_n2, assign69990_e106507_d_n4, assign69990_e106507_d_n5, assign69990_e106507_d_n6, assign69990_e106507_d_n7, assign69990_e106507_d_n8, assign69990_e106507_d_n9, assign69990_e106507_d_n10, assign69990_e106507_d_n11, assign69990_e106507_d_n14,) = {
    if (((((locals.var_guard1633 != 0.0) && (!((locals.var_guard1631 != 0.0) || (locals.var_guard1632 != 0.0)))) && (locals.var_guard1637 != 0.0)) && (locals.var_guard1638 != 0.0)) && (locals.var_guard1639 != 0.0)) {
        let assign69990_e106494: f64 = (locals.var_vxbgmt + p.p137);
        let assign69990_e106497: f64 = (locals.var_vxbgmt + p.p137);
        let assign69990_e106498: f64 = (assign69990_e106494 * assign69990_e106497);
        let assign69990_e106501: f64 = (4.0 * 0.1);
        let assign69990_e106503: f64 = (assign69990_e106501 * 0.1);
        let assign69990_e106504: f64 = (assign69990_e106498 + assign69990_e106503);
        let assign69990_e106505: f64 = (assign69990_e106504).sqrt();
        (assign69990_e106505, (((locals.var_vxbgmt_dn0 * assign69990_e106497) + (assign69990_e106494 * locals.var_vxbgmt_dn0)) / (2.0 * assign69990_e106505)), (((locals.var_vxbgmt_dn2 * assign69990_e106497) + (assign69990_e106494 * locals.var_vxbgmt_dn2)) / (2.0 * assign69990_e106505)), (((locals.var_vxbgmt_dn4 * assign69990_e106497) + (assign69990_e106494 * locals.var_vxbgmt_dn4)) / (2.0 * assign69990_e106505)), (((locals.var_vxbgmt_dn5 * assign69990_e106497) + (assign69990_e106494 * locals.var_vxbgmt_dn5)) / (2.0 * assign69990_e106505)), (((locals.var_vxbgmt_dn6 * assign69990_e106497) + (assign69990_e106494 * locals.var_vxbgmt_dn6)) / (2.0 * assign69990_e106505)), (((locals.var_vxbgmt_dn7 * assign69990_e106497) + (assign69990_e106494 * locals.var_vxbgmt_dn7)) / (2.0 * assign69990_e106505)), (((locals.var_vxbgmt_dn8 * assign69990_e106497) + (assign69990_e106494 * locals.var_vxbgmt_dn8)) / (2.0 * assign69990_e106505)), (((locals.var_vxbgmt_dn9 * assign69990_e106497) + (assign69990_e106494 * locals.var_vxbgmt_dn9)) / (2.0 * assign69990_e106505)), (((locals.var_vxbgmt_dn10 * assign69990_e106497) + (assign69990_e106494 * locals.var_vxbgmt_dn10)) / (2.0 * assign69990_e106505)), (((locals.var_vxbgmt_dn11 * assign69990_e106497) + (assign69990_e106494 * locals.var_vxbgmt_dn11)) / (2.0 * assign69990_e106505)), (((locals.var_vxbgmt_dn14 * assign69990_e106497) + (assign69990_e106494 * locals.var_vxbgmt_dn14)) / (2.0 * assign69990_e106505)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign69990_e106507;
        locals.var_tmf2_dn0 = assign69990_e106507_d_n0;
        locals.var_tmf2_dn2 = assign69990_e106507_d_n2;
        locals.var_tmf2_dn4 = assign69990_e106507_d_n4;
        locals.var_tmf2_dn5 = assign69990_e106507_d_n5;
        locals.var_tmf2_dn6 = assign69990_e106507_d_n6;
        locals.var_tmf2_dn7 = assign69990_e106507_d_n7;
        locals.var_tmf2_dn8 = assign69990_e106507_d_n8;
        locals.var_tmf2_dn9 = assign69990_e106507_d_n9;
        locals.var_tmf2_dn10 = assign69990_e106507_d_n10;
        locals.var_tmf2_dn11 = assign69990_e106507_d_n11;
        locals.var_tmf2_dn14 = assign69990_e106507_d_n14;

        let (assign70000_e106530, assign70000_e106530_d_n0, assign70000_e106530_d_n2, assign70000_e106530_d_n4, assign70000_e106530_d_n5, assign70000_e106530_d_n6, assign70000_e106530_d_n7, assign70000_e106530_d_n8, assign70000_e106530_d_n9, assign70000_e106530_d_n10, assign70000_e106530_d_n11, assign70000_e106530_d_n14,) = {
    if (((((locals.var_guard1633 != 0.0) && (!((locals.var_guard1631 != 0.0) || (locals.var_guard1632 != 0.0)))) && (locals.var_guard1637 != 0.0)) && (locals.var_guard1638 != 0.0)) && (locals.var_guard1639 != 0.0)) {
        let assign70000_e106524: f64 = (locals.var_vxbgmt + p.p137);
        let assign70000_e106526: f64 = (assign70000_e106524 / locals.var_tmf2);
        let assign70000_e106527: f64 = (1.0 + assign70000_e106526);
        let assign70000_e106528: f64 = (0.5 * assign70000_e106527);
        (assign70000_e106528, (0.5 * (((locals.var_vxbgmt_dn0 * locals.var_tmf2) - (assign70000_e106524 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn2 * locals.var_tmf2) - (assign70000_e106524 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn4 * locals.var_tmf2) - (assign70000_e106524 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn5 * locals.var_tmf2) - (assign70000_e106524 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn6 * locals.var_tmf2) - (assign70000_e106524 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn7 * locals.var_tmf2) - (assign70000_e106524 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn8 * locals.var_tmf2) - (assign70000_e106524 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn9 * locals.var_tmf2) - (assign70000_e106524 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn10 * locals.var_tmf2) - (assign70000_e106524 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn11 * locals.var_tmf2) - (assign70000_e106524 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn14 * locals.var_tmf2) - (assign70000_e106524 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign70000_e106530;
        locals.var_t9_dn0 = assign70000_e106530_d_n0;
        locals.var_t9_dn2 = assign70000_e106530_d_n2;
        locals.var_t9_dn4 = assign70000_e106530_d_n4;
        locals.var_t9_dn5 = assign70000_e106530_d_n5;
        locals.var_t9_dn6 = assign70000_e106530_d_n6;
        locals.var_t9_dn7 = assign70000_e106530_d_n7;
        locals.var_t9_dn8 = assign70000_e106530_d_n8;
        locals.var_t9_dn9 = assign70000_e106530_d_n9;
        locals.var_t9_dn10 = assign70000_e106530_d_n10;
        locals.var_t9_dn11 = assign70000_e106530_d_n11;
        locals.var_t9_dn14 = assign70000_e106530_d_n14;

        let (assign70010_e106551, assign70010_e106551_d_n0, assign70010_e106551_d_n2, assign70010_e106551_d_n4, assign70010_e106551_d_n5, assign70010_e106551_d_n6, assign70010_e106551_d_n7, assign70010_e106551_d_n8, assign70010_e106551_d_n9, assign70010_e106551_d_n10, assign70010_e106551_d_n11, assign70010_e106551_d_n14,) = {
    if (((((locals.var_guard1633 != 0.0) && (!((locals.var_guard1631 != 0.0) || (locals.var_guard1632 != 0.0)))) && (locals.var_guard1637 != 0.0)) && (locals.var_guard1638 != 0.0)) && (locals.var_guard1639 != 0.0)) {
        let assign70010_e106546: f64 = (locals.var_vxbgmt + p.p137);
        let assign70010_e106548: f64 = (assign70010_e106546 + locals.var_tmf2);
        let assign70010_e106549: f64 = (0.5 * assign70010_e106548);
        (assign70010_e106549, (0.5 * (locals.var_vxbgmt_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_vxbgmt_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_vxbgmt_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_vxbgmt_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_vxbgmt_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_vxbgmt_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_vxbgmt_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_vxbgmt_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_vxbgmt_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_vxbgmt_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_vxbgmt_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign70010_e106551;
        locals.var_t2_dn0 = assign70010_e106551_d_n0;
        locals.var_t2_dn2 = assign70010_e106551_d_n2;
        locals.var_t2_dn4 = assign70010_e106551_d_n4;
        locals.var_t2_dn5 = assign70010_e106551_d_n5;
        locals.var_t2_dn6 = assign70010_e106551_d_n6;
        locals.var_t2_dn7 = assign70010_e106551_d_n7;
        locals.var_t2_dn8 = assign70010_e106551_d_n8;
        locals.var_t2_dn9 = assign70010_e106551_d_n9;
        locals.var_t2_dn10 = assign70010_e106551_d_n10;
        locals.var_t2_dn11 = assign70010_e106551_d_n11;
        locals.var_t2_dn14 = assign70010_e106551_d_n14;

        let assign70020_e106554: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1641 = assign70020_e106554;

        let (assign70030_e106571, assign70030_e106571_d_n0, assign70030_e106571_d_n2, assign70030_e106571_d_n4, assign70030_e106571_d_n5, assign70030_e106571_d_n6, assign70030_e106571_d_n7, assign70030_e106571_d_n8, assign70030_e106571_d_n9, assign70030_e106571_d_n10, assign70030_e106571_d_n11, assign70030_e106571_d_n14,) = {
    if ((((((locals.var_guard1633 != 0.0) && (!((locals.var_guard1631 != 0.0) || (locals.var_guard1632 != 0.0)))) && (locals.var_guard1637 != 0.0)) && (locals.var_guard1638 != 0.0)) && (locals.var_guard1639 != 0.0)) && (locals.var_guard1641 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign70030_e106571;
        locals.var_t2_dn0 = assign70030_e106571_d_n0;
        locals.var_t2_dn2 = assign70030_e106571_d_n2;
        locals.var_t2_dn4 = assign70030_e106571_d_n4;
        locals.var_t2_dn5 = assign70030_e106571_d_n5;
        locals.var_t2_dn6 = assign70030_e106571_d_n6;
        locals.var_t2_dn7 = assign70030_e106571_d_n7;
        locals.var_t2_dn8 = assign70030_e106571_d_n8;
        locals.var_t2_dn9 = assign70030_e106571_d_n9;
        locals.var_t2_dn10 = assign70030_e106571_d_n10;
        locals.var_t2_dn11 = assign70030_e106571_d_n11;
        locals.var_t2_dn14 = assign70030_e106571_d_n14;

        let (assign70040_e106588, assign70040_e106588_d_n0, assign70040_e106588_d_n2, assign70040_e106588_d_n4, assign70040_e106588_d_n5, assign70040_e106588_d_n6, assign70040_e106588_d_n7, assign70040_e106588_d_n8, assign70040_e106588_d_n9, assign70040_e106588_d_n10, assign70040_e106588_d_n11, assign70040_e106588_d_n14,) = {
    if ((((((locals.var_guard1633 != 0.0) && (!((locals.var_guard1631 != 0.0) || (locals.var_guard1632 != 0.0)))) && (locals.var_guard1637 != 0.0)) && (locals.var_guard1638 != 0.0)) && (locals.var_guard1639 != 0.0)) && (locals.var_guard1641 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign70040_e106588;
        locals.var_t9_dn0 = assign70040_e106588_d_n0;
        locals.var_t9_dn2 = assign70040_e106588_d_n2;
        locals.var_t9_dn4 = assign70040_e106588_d_n4;
        locals.var_t9_dn5 = assign70040_e106588_d_n5;
        locals.var_t9_dn6 = assign70040_e106588_d_n6;
        locals.var_t9_dn7 = assign70040_e106588_d_n7;
        locals.var_t9_dn8 = assign70040_e106588_d_n8;
        locals.var_t9_dn9 = assign70040_e106588_d_n9;
        locals.var_t9_dn10 = assign70040_e106588_d_n10;
        locals.var_t9_dn11 = assign70040_e106588_d_n11;
        locals.var_t9_dn14 = assign70040_e106588_d_n14;

        let (assign70050_e106608, assign70050_e106608_d_n0, assign70050_e106608_d_n2, assign70050_e106608_d_n4, assign70050_e106608_d_n5, assign70050_e106608_d_n6, assign70050_e106608_d_n7, assign70050_e106608_d_n8, assign70050_e106608_d_n9, assign70050_e106608_d_n10, assign70050_e106608_d_n11, assign70050_e106608_d_n14,) = {
    if (((((locals.var_guard1633 != 0.0) && (!((locals.var_guard1631 != 0.0) || (locals.var_guard1632 != 0.0)))) && (locals.var_guard1637 != 0.0)) && (locals.var_guard1638 != 0.0)) && (locals.var_guard1639 != 0.0)) {
        let assign70050_e106603: f64 = (locals.var_kjunc * locals.var_t2);
        let assign70050_e106604: f64 = (assign70050_e106603).sqrt();
        let assign70050_e106606: f64 = (assign70050_e106604 * p.p432);
        (assign70050_e106606, ((((locals.var_kjunc_dn0 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn0)) / (2.0 * assign70050_e106604)) * p.p432), ((((locals.var_kjunc_dn2 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn2)) / (2.0 * assign70050_e106604)) * p.p432), ((((locals.var_kjunc_dn4 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn4)) / (2.0 * assign70050_e106604)) * p.p432), ((((locals.var_kjunc_dn5 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn5)) / (2.0 * assign70050_e106604)) * p.p432), ((((locals.var_kjunc_dn6 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn6)) / (2.0 * assign70050_e106604)) * p.p432), ((((locals.var_kjunc_dn7 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn7)) / (2.0 * assign70050_e106604)) * p.p432), ((((locals.var_kjunc_dn8 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn8)) / (2.0 * assign70050_e106604)) * p.p432), ((((locals.var_kjunc_dn9 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn9)) / (2.0 * assign70050_e106604)) * p.p432), ((((locals.var_kjunc_dn10 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn10)) / (2.0 * assign70050_e106604)) * p.p432), ((((locals.var_kjunc_dn11 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn11)) / (2.0 * assign70050_e106604)) * p.p432), ((((locals.var_kjunc_dn14 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn14)) / (2.0 * assign70050_e106604)) * p.p432),)
    } else {
        (locals.var_wjunc0, locals.var_wjunc0_dn0, locals.var_wjunc0_dn2, locals.var_wjunc0_dn4, locals.var_wjunc0_dn5, locals.var_wjunc0_dn6, locals.var_wjunc0_dn7, locals.var_wjunc0_dn8, locals.var_wjunc0_dn9, locals.var_wjunc0_dn10, locals.var_wjunc0_dn11, locals.var_wjunc0_dn14,)
    }
};
        locals.var_wjunc0 = assign70050_e106608;
        locals.var_wjunc0_dn0 = assign70050_e106608_d_n0;
        locals.var_wjunc0_dn2 = assign70050_e106608_d_n2;
        locals.var_wjunc0_dn4 = assign70050_e106608_d_n4;
        locals.var_wjunc0_dn5 = assign70050_e106608_d_n5;
        locals.var_wjunc0_dn6 = assign70050_e106608_d_n6;
        locals.var_wjunc0_dn7 = assign70050_e106608_d_n7;
        locals.var_wjunc0_dn8 = assign70050_e106608_d_n8;
        locals.var_wjunc0_dn9 = assign70050_e106608_d_n9;
        locals.var_wjunc0_dn10 = assign70050_e106608_d_n10;
        locals.var_wjunc0_dn11 = assign70050_e106608_d_n11;
        locals.var_wjunc0_dn14 = assign70050_e106608_d_n14;

        let (assign70060_e106625, assign70060_e106625_d_n0, assign70060_e106625_d_n2, assign70060_e106625_d_n4, assign70060_e106625_d_n5, assign70060_e106625_d_n6, assign70060_e106625_d_n7, assign70060_e106625_d_n8, assign70060_e106625_d_n9, assign70060_e106625_d_n10, assign70060_e106625_d_n11, assign70060_e106625_d_n14,) = {
    if (((((locals.var_guard1633 != 0.0) && (!((locals.var_guard1631 != 0.0) || (locals.var_guard1632 != 0.0)))) && (locals.var_guard1637 != 0.0)) && (locals.var_guard1638 != 0.0)) && (locals.var_guard1639 != 0.0)) {
        let assign70060_e106623: f64 = (locals.var_lover_func - locals.var_wjunc0);
        (assign70060_e106623, (locals.var_lover_func_dn0 - locals.var_wjunc0_dn0), (locals.var_lover_func_dn2 - locals.var_wjunc0_dn2), (locals.var_lover_func_dn4 - locals.var_wjunc0_dn4), (locals.var_lover_func_dn5 - locals.var_wjunc0_dn5), (locals.var_lover_func_dn6 - locals.var_wjunc0_dn6), (locals.var_lover_func_dn7 - locals.var_wjunc0_dn7), (locals.var_lover_func_dn8 - locals.var_wjunc0_dn8), (locals.var_lover_func_dn9 - locals.var_wjunc0_dn9), (locals.var_lover_func_dn10 - locals.var_wjunc0_dn10), (locals.var_lover_func_dn11 - locals.var_wjunc0_dn11), (locals.var_lover_func_dn14 - locals.var_wjunc0_dn14),)
    } else {
        (locals.var_lover_func, locals.var_lover_func_dn0, locals.var_lover_func_dn2, locals.var_lover_func_dn4, locals.var_lover_func_dn5, locals.var_lover_func_dn6, locals.var_lover_func_dn7, locals.var_lover_func_dn8, locals.var_lover_func_dn9, locals.var_lover_func_dn10, locals.var_lover_func_dn11, locals.var_lover_func_dn14,)
    }
};
        locals.var_lover_func = assign70060_e106625;
        locals.var_lover_func_dn0 = assign70060_e106625_d_n0;
        locals.var_lover_func_dn2 = assign70060_e106625_d_n2;
        locals.var_lover_func_dn4 = assign70060_e106625_d_n4;
        locals.var_lover_func_dn5 = assign70060_e106625_d_n5;
        locals.var_lover_func_dn6 = assign70060_e106625_d_n6;
        locals.var_lover_func_dn7 = assign70060_e106625_d_n7;
        locals.var_lover_func_dn8 = assign70060_e106625_d_n8;
        locals.var_lover_func_dn9 = assign70060_e106625_d_n9;
        locals.var_lover_func_dn10 = assign70060_e106625_d_n10;
        locals.var_lover_func_dn11 = assign70060_e106625_d_n11;
        locals.var_lover_func_dn14 = assign70060_e106625_d_n14;

        let assign70070_e106644: f64 = if (((((p.p35 == 1.0) && (p.p63 > 0.0)) && (locals.var_uc_nover > 0.0)) && (locals.var_uc_cvdsover != 0.0)) && (p.p55 != 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard1642 = assign70070_e106644;

        let (assign70080_e106657,) = {
    if (((locals.var_guard1634 != 0.0) && (!(((locals.var_guard1631 != 0.0) || (locals.var_guard1632 != 0.0)) || (locals.var_guard1633 != 0.0)))) && (locals.var_guard1642 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_calcqover,)
    }
};
        locals.var_flg_calcqover = assign70080_e106657;

        let (assign70090_e106672, assign70090_e106672_d_n2, assign70090_e106672_d_n7, assign70090_e106672_d_n8, assign70090_e106672_d_n9,) = {
    if (((locals.var_guard1634 != 0.0) && (!(((locals.var_guard1631 != 0.0) || (locals.var_guard1632 != 0.0)) || (locals.var_guard1633 != 0.0)))) && (locals.var_guard1642 != 0.0)) {
        let assign70090_e106670: f64 = (locals.var_vgsei - locals.var_vbsei);
        (assign70090_e106670, (locals.var_vgsei_dn2 - locals.var_vbsei_dn2), locals.var_vgsei_dn7, 0.0, (-locals.var_vbsei_dn9),)
    } else {
        (locals.var_vgbgmt, locals.var_vgbgmt_dn2, locals.var_vgbgmt_dn7, locals.var_vgbgmt_dn8, locals.var_vgbgmt_dn9,)
    }
};
        locals.var_vgbgmt = assign70090_e106672;
        locals.var_vgbgmt_dn2 = assign70090_e106672_d_n2;
        locals.var_vgbgmt_dn7 = assign70090_e106672_d_n7;
        locals.var_vgbgmt_dn8 = assign70090_e106672_d_n8;
        locals.var_vgbgmt_dn9 = assign70090_e106672_d_n9;

        let (assign70100_e106687, assign70100_e106687_d_n0, assign70100_e106687_d_n2, assign70100_e106687_d_n4, assign70100_e106687_d_n5, assign70100_e106687_d_n6, assign70100_e106687_d_n7, assign70100_e106687_d_n8, assign70100_e106687_d_n9, assign70100_e106687_d_n10, assign70100_e106687_d_n11, assign70100_e106687_d_n14,) = {
    if (((locals.var_guard1634 != 0.0) && (!(((locals.var_guard1631 != 0.0) || (locals.var_guard1632 != 0.0)) || (locals.var_guard1633 != 0.0)))) && (locals.var_guard1642 != 0.0)) {
        let assign70100_e106685: f64 = (locals.var_vdsei - locals.var_vbsei);
        (assign70100_e106685, locals.var_vdsei_dn0, (locals.var_vdsei_dn2 - locals.var_vbsei_dn2), 0.0, 0.0, 0.0, 0.0, 0.0, (-locals.var_vbsei_dn9), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn11, locals.var_vxbgmt_dn14,)
    }
};
        locals.var_vxbgmt = assign70100_e106687;
        locals.var_vxbgmt_dn0 = assign70100_e106687_d_n0;
        locals.var_vxbgmt_dn2 = assign70100_e106687_d_n2;
        locals.var_vxbgmt_dn4 = assign70100_e106687_d_n4;
        locals.var_vxbgmt_dn5 = assign70100_e106687_d_n5;
        locals.var_vxbgmt_dn6 = assign70100_e106687_d_n6;
        locals.var_vxbgmt_dn7 = assign70100_e106687_d_n7;
        locals.var_vxbgmt_dn8 = assign70100_e106687_d_n8;
        locals.var_vxbgmt_dn9 = assign70100_e106687_d_n9;
        locals.var_vxbgmt_dn10 = assign70100_e106687_d_n10;
        locals.var_vxbgmt_dn11 = assign70100_e106687_d_n11;
        locals.var_vxbgmt_dn14 = assign70100_e106687_d_n14;

    }

    pub(super) fn stamp_transient_block_250(
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign70110_e106691, assign70110_e106691_d_n0, assign70110_e106691_d_n2, assign70110_e106691_d_n4, assign70110_e106691_d_n5, assign70110_e106691_d_n6, assign70110_e106691_d_n7, assign70110_e106691_d_n8, assign70110_e106691_d_n9, assign70110_e106691_d_n10, assign70110_e106691_d_n11, assign70110_e106691_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.4, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbs_bnd_over, locals.var_vbs_bnd_over_dn0, locals.var_vbs_bnd_over_dn2, locals.var_vbs_bnd_over_dn4, locals.var_vbs_bnd_over_dn5, locals.var_vbs_bnd_over_dn6, locals.var_vbs_bnd_over_dn7, locals.var_vbs_bnd_over_dn8, locals.var_vbs_bnd_over_dn9, locals.var_vbs_bnd_over_dn10, locals.var_vbs_bnd_over_dn11, locals.var_vbs_bnd_over_dn14,)
    }
};
        locals.var_vbs_bnd_over = assign70110_e106691;
        locals.var_vbs_bnd_over_dn0 = assign70110_e106691_d_n0;
        locals.var_vbs_bnd_over_dn2 = assign70110_e106691_d_n2;
        locals.var_vbs_bnd_over_dn4 = assign70110_e106691_d_n4;
        locals.var_vbs_bnd_over_dn5 = assign70110_e106691_d_n5;
        locals.var_vbs_bnd_over_dn6 = assign70110_e106691_d_n6;
        locals.var_vbs_bnd_over_dn7 = assign70110_e106691_d_n7;
        locals.var_vbs_bnd_over_dn8 = assign70110_e106691_d_n8;
        locals.var_vbs_bnd_over_dn9 = assign70110_e106691_d_n9;
        locals.var_vbs_bnd_over_dn10 = assign70110_e106691_d_n10;
        locals.var_vbs_bnd_over_dn11 = assign70110_e106691_d_n11;
        locals.var_vbs_bnd_over_dn14 = assign70110_e106691_d_n14;

        let (assign70130_e106699,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0,)
    } else {
        (locals.var_flg_fd_mode,)
    }
};
        locals.var_flg_fd_mode = assign70130_e106699;

        let (assign70140_e106703, assign70140_e106703_d_n0, assign70140_e106703_d_n2, assign70140_e106703_d_n4, assign70140_e106703_d_n5, assign70140_e106703_d_n6, assign70140_e106703_d_n7, assign70140_e106703_d_n8, assign70140_e106703_d_n9, assign70140_e106703_d_n10, assign70140_e106703_d_n11, assign70140_e106703_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn14,)
    }
};
        locals.var_fb = assign70140_e106703;
        locals.var_fb_dn0 = assign70140_e106703_d_n0;
        locals.var_fb_dn2 = assign70140_e106703_d_n2;
        locals.var_fb_dn4 = assign70140_e106703_d_n4;
        locals.var_fb_dn5 = assign70140_e106703_d_n5;
        locals.var_fb_dn6 = assign70140_e106703_d_n6;
        locals.var_fb_dn7 = assign70140_e106703_d_n7;
        locals.var_fb_dn8 = assign70140_e106703_d_n8;
        locals.var_fb_dn9 = assign70140_e106703_d_n9;
        locals.var_fb_dn10 = assign70140_e106703_d_n10;
        locals.var_fb_dn11 = assign70140_e106703_d_n11;
        locals.var_fb_dn14 = assign70140_e106703_d_n14;

        let (assign70150_e106707, assign70150_e106707_d_n0, assign70150_e106707_d_n2, assign70150_e106707_d_n4, assign70150_e106707_d_n5, assign70150_e106707_d_n6, assign70150_e106707_d_n7, assign70150_e106707_d_n8, assign70150_e106707_d_n9, assign70150_e106707_d_n10, assign70150_e106707_d_n11, assign70150_e106707_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn11, locals.var_fs01_dn14,)
    }
};
        locals.var_fs01 = assign70150_e106707;
        locals.var_fs01_dn0 = assign70150_e106707_d_n0;
        locals.var_fs01_dn2 = assign70150_e106707_d_n2;
        locals.var_fs01_dn4 = assign70150_e106707_d_n4;
        locals.var_fs01_dn5 = assign70150_e106707_d_n5;
        locals.var_fs01_dn6 = assign70150_e106707_d_n6;
        locals.var_fs01_dn7 = assign70150_e106707_d_n7;
        locals.var_fs01_dn8 = assign70150_e106707_d_n8;
        locals.var_fs01_dn9 = assign70150_e106707_d_n9;
        locals.var_fs01_dn10 = assign70150_e106707_d_n10;
        locals.var_fs01_dn11 = assign70150_e106707_d_n11;
        locals.var_fs01_dn14 = assign70150_e106707_d_n14;

        let (assign70160_e106711, assign70160_e106711_d_n0, assign70160_e106711_d_n2, assign70160_e106711_d_n4, assign70160_e106711_d_n5, assign70160_e106711_d_n6, assign70160_e106711_d_n7, assign70160_e106711_d_n8, assign70160_e106711_d_n9, assign70160_e106711_d_n10, assign70160_e106711_d_n11, assign70160_e106711_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn11, locals.var_fs02_dn14,)
    }
};
        locals.var_fs02 = assign70160_e106711;
        locals.var_fs02_dn0 = assign70160_e106711_d_n0;
        locals.var_fs02_dn2 = assign70160_e106711_d_n2;
        locals.var_fs02_dn4 = assign70160_e106711_d_n4;
        locals.var_fs02_dn5 = assign70160_e106711_d_n5;
        locals.var_fs02_dn6 = assign70160_e106711_d_n6;
        locals.var_fs02_dn7 = assign70160_e106711_d_n7;
        locals.var_fs02_dn8 = assign70160_e106711_d_n8;
        locals.var_fs02_dn9 = assign70160_e106711_d_n9;
        locals.var_fs02_dn10 = assign70160_e106711_d_n10;
        locals.var_fs02_dn11 = assign70160_e106711_d_n11;
        locals.var_fs02_dn14 = assign70160_e106711_d_n14;

        let (assign70170_e106715, assign70170_e106715_d_n0, assign70170_e106715_d_n2, assign70170_e106715_d_n4, assign70170_e106715_d_n5, assign70170_e106715_d_n6, assign70170_e106715_d_n7, assign70170_e106715_d_n8, assign70170_e106715_d_n9, assign70170_e106715_d_n10, assign70170_e106715_d_n11, assign70170_e106715_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs0, locals.var_fs0_dn0, locals.var_fs0_dn2, locals.var_fs0_dn4, locals.var_fs0_dn5, locals.var_fs0_dn6, locals.var_fs0_dn7, locals.var_fs0_dn8, locals.var_fs0_dn9, locals.var_fs0_dn10, locals.var_fs0_dn11, locals.var_fs0_dn14,)
    }
};
        locals.var_fs0 = assign70170_e106715;
        locals.var_fs0_dn0 = assign70170_e106715_d_n0;
        locals.var_fs0_dn2 = assign70170_e106715_d_n2;
        locals.var_fs0_dn4 = assign70170_e106715_d_n4;
        locals.var_fs0_dn5 = assign70170_e106715_d_n5;
        locals.var_fs0_dn6 = assign70170_e106715_d_n6;
        locals.var_fs0_dn7 = assign70170_e106715_d_n7;
        locals.var_fs0_dn8 = assign70170_e106715_d_n8;
        locals.var_fs0_dn9 = assign70170_e106715_d_n9;
        locals.var_fs0_dn10 = assign70170_e106715_d_n10;
        locals.var_fs0_dn11 = assign70170_e106715_d_n11;
        locals.var_fs0_dn14 = assign70170_e106715_d_n14;

        let (assign70180_e106719, assign70180_e106719_d_n0, assign70180_e106719_d_n2, assign70180_e106719_d_n4, assign70180_e106719_d_n5, assign70180_e106719_d_n6, assign70180_e106719_d_n7, assign70180_e106719_d_n8, assign70180_e106719_d_n9, assign70180_e106719_d_n10, assign70180_e106719_d_n11, assign70180_e106719_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn4, locals.var_dps0_dn5, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn8, locals.var_dps0_dn9, locals.var_dps0_dn10, locals.var_dps0_dn11, locals.var_dps0_dn14,)
    }
};
        locals.var_dps0 = assign70180_e106719;
        locals.var_dps0_dn0 = assign70180_e106719_d_n0;
        locals.var_dps0_dn2 = assign70180_e106719_d_n2;
        locals.var_dps0_dn4 = assign70180_e106719_d_n4;
        locals.var_dps0_dn5 = assign70180_e106719_d_n5;
        locals.var_dps0_dn6 = assign70180_e106719_d_n6;
        locals.var_dps0_dn7 = assign70180_e106719_d_n7;
        locals.var_dps0_dn8 = assign70180_e106719_d_n8;
        locals.var_dps0_dn9 = assign70180_e106719_d_n9;
        locals.var_dps0_dn10 = assign70180_e106719_d_n10;
        locals.var_dps0_dn11 = assign70180_e106719_d_n11;
        locals.var_dps0_dn14 = assign70180_e106719_d_n14;

        let (assign70190_e106723, assign70190_e106723_d_n0, assign70190_e106723_d_n2, assign70190_e106723_d_n4, assign70190_e106723_d_n5, assign70190_e106723_d_n6, assign70190_e106723_d_n7, assign70190_e106723_d_n8, assign70190_e106723_d_n9, assign70190_e106723_d_n10, assign70190_e106723_d_n11, assign70190_e106723_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs0_dps0, locals.var_fs0_dps0_dn0, locals.var_fs0_dps0_dn2, locals.var_fs0_dps0_dn4, locals.var_fs0_dps0_dn5, locals.var_fs0_dps0_dn6, locals.var_fs0_dps0_dn7, locals.var_fs0_dps0_dn8, locals.var_fs0_dps0_dn9, locals.var_fs0_dps0_dn10, locals.var_fs0_dps0_dn11, locals.var_fs0_dps0_dn14,)
    }
};
        locals.var_fs0_dps0 = assign70190_e106723;
        locals.var_fs0_dps0_dn0 = assign70190_e106723_d_n0;
        locals.var_fs0_dps0_dn2 = assign70190_e106723_d_n2;
        locals.var_fs0_dps0_dn4 = assign70190_e106723_d_n4;
        locals.var_fs0_dps0_dn5 = assign70190_e106723_d_n5;
        locals.var_fs0_dps0_dn6 = assign70190_e106723_d_n6;
        locals.var_fs0_dps0_dn7 = assign70190_e106723_d_n7;
        locals.var_fs0_dps0_dn8 = assign70190_e106723_d_n8;
        locals.var_fs0_dps0_dn9 = assign70190_e106723_d_n9;
        locals.var_fs0_dps0_dn10 = assign70190_e106723_d_n10;
        locals.var_fs0_dps0_dn11 = assign70190_e106723_d_n11;
        locals.var_fs0_dps0_dn14 = assign70190_e106723_d_n14;

        let (assign70200_e106727, assign70200_e106727_d_n0, assign70200_e106727_d_n2, assign70200_e106727_d_n4, assign70200_e106727_d_n5, assign70200_e106727_d_n6, assign70200_e106727_d_n7, assign70200_e106727_d_n8, assign70200_e106727_d_n9, assign70200_e106727_d_n10, assign70200_e106727_d_n11, assign70200_e106727_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn11, locals.var_fs02_dps0_dn14,)
    }
};
        locals.var_fs02_dps0 = assign70200_e106727;
        locals.var_fs02_dps0_dn0 = assign70200_e106727_d_n0;
        locals.var_fs02_dps0_dn2 = assign70200_e106727_d_n2;
        locals.var_fs02_dps0_dn4 = assign70200_e106727_d_n4;
        locals.var_fs02_dps0_dn5 = assign70200_e106727_d_n5;
        locals.var_fs02_dps0_dn6 = assign70200_e106727_d_n6;
        locals.var_fs02_dps0_dn7 = assign70200_e106727_d_n7;
        locals.var_fs02_dps0_dn8 = assign70200_e106727_d_n8;
        locals.var_fs02_dps0_dn9 = assign70200_e106727_d_n9;
        locals.var_fs02_dps0_dn10 = assign70200_e106727_d_n10;
        locals.var_fs02_dps0_dn11 = assign70200_e106727_d_n11;
        locals.var_fs02_dps0_dn14 = assign70200_e106727_d_n14;

        let (assign70210_e106731, assign70210_e106731_d_n0, assign70210_e106731_d_n2, assign70210_e106731_d_n4, assign70210_e106731_d_n5, assign70210_e106731_d_n6, assign70210_e106731_d_n7, assign70210_e106731_d_n8, assign70210_e106731_d_n9, assign70210_e106731_d_n10, assign70210_e106731_d_n11, assign70210_e106731_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn14,)
    }
};
        locals.var_fb_dpss = assign70210_e106731;
        locals.var_fb_dpss_dn0 = assign70210_e106731_d_n0;
        locals.var_fb_dpss_dn2 = assign70210_e106731_d_n2;
        locals.var_fb_dpss_dn4 = assign70210_e106731_d_n4;
        locals.var_fb_dpss_dn5 = assign70210_e106731_d_n5;
        locals.var_fb_dpss_dn6 = assign70210_e106731_d_n6;
        locals.var_fb_dpss_dn7 = assign70210_e106731_d_n7;
        locals.var_fb_dpss_dn8 = assign70210_e106731_d_n8;
        locals.var_fb_dpss_dn9 = assign70210_e106731_d_n9;
        locals.var_fb_dpss_dn10 = assign70210_e106731_d_n10;
        locals.var_fb_dpss_dn11 = assign70210_e106731_d_n11;
        locals.var_fb_dpss_dn14 = assign70210_e106731_d_n14;

        let (assign70220_e106735, assign70220_e106735_d_n0, assign70220_e106735_d_n2, assign70220_e106735_d_n4, assign70220_e106735_d_n5, assign70220_e106735_d_n6, assign70220_e106735_d_n7, assign70220_e106735_d_n8, assign70220_e106735_d_n9, assign70220_e106735_d_n10, assign70220_e106735_d_n11, assign70220_e106735_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn11, locals.var_fs01_dps0_dn14,)
    }
};
        locals.var_fs01_dps0 = assign70220_e106735;
        locals.var_fs01_dps0_dn0 = assign70220_e106735_d_n0;
        locals.var_fs01_dps0_dn2 = assign70220_e106735_d_n2;
        locals.var_fs01_dps0_dn4 = assign70220_e106735_d_n4;
        locals.var_fs01_dps0_dn5 = assign70220_e106735_d_n5;
        locals.var_fs01_dps0_dn6 = assign70220_e106735_d_n6;
        locals.var_fs01_dps0_dn7 = assign70220_e106735_d_n7;
        locals.var_fs01_dps0_dn8 = assign70220_e106735_d_n8;
        locals.var_fs01_dps0_dn9 = assign70220_e106735_d_n9;
        locals.var_fs01_dps0_dn10 = assign70220_e106735_d_n10;
        locals.var_fs01_dps0_dn11 = assign70220_e106735_d_n11;
        locals.var_fs01_dps0_dn14 = assign70220_e106735_d_n14;

        let (assign70230_e106739, assign70230_e106739_d_n0, assign70230_e106739_d_n2, assign70230_e106739_d_n4, assign70230_e106739_d_n5, assign70230_e106739_d_n6, assign70230_e106739_d_n7, assign70230_e106739_d_n8, assign70230_e106739_d_n9, assign70230_e106739_d_n10, assign70230_e106739_d_n11, assign70230_e106739_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn11, locals.var_chi_1_dn14,)
    }
};
        locals.var_chi_1 = assign70230_e106739;
        locals.var_chi_1_dn0 = assign70230_e106739_d_n0;
        locals.var_chi_1_dn2 = assign70230_e106739_d_n2;
        locals.var_chi_1_dn4 = assign70230_e106739_d_n4;
        locals.var_chi_1_dn5 = assign70230_e106739_d_n5;
        locals.var_chi_1_dn6 = assign70230_e106739_d_n6;
        locals.var_chi_1_dn7 = assign70230_e106739_d_n7;
        locals.var_chi_1_dn8 = assign70230_e106739_d_n8;
        locals.var_chi_1_dn9 = assign70230_e106739_d_n9;
        locals.var_chi_1_dn10 = assign70230_e106739_d_n10;
        locals.var_chi_1_dn11 = assign70230_e106739_d_n11;
        locals.var_chi_1_dn14 = assign70230_e106739_d_n14;

        let (assign70240_e106743, assign70240_e106743_d_n0, assign70240_e106743_d_n2, assign70240_e106743_d_n4, assign70240_e106743_d_n5, assign70240_e106743_d_n6, assign70240_e106743_d_n7, assign70240_e106743_d_n8, assign70240_e106743_d_n9, assign70240_e106743_d_n10, assign70240_e106743_d_n11, assign70240_e106743_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_chi_a, locals.var_chi_a_dn0, locals.var_chi_a_dn2, locals.var_chi_a_dn4, locals.var_chi_a_dn5, locals.var_chi_a_dn6, locals.var_chi_a_dn7, locals.var_chi_a_dn8, locals.var_chi_a_dn9, locals.var_chi_a_dn10, locals.var_chi_a_dn11, locals.var_chi_a_dn14,)
    }
};
        locals.var_chi_a = assign70240_e106743;
        locals.var_chi_a_dn0 = assign70240_e106743_d_n0;
        locals.var_chi_a_dn2 = assign70240_e106743_d_n2;
        locals.var_chi_a_dn4 = assign70240_e106743_d_n4;
        locals.var_chi_a_dn5 = assign70240_e106743_d_n5;
        locals.var_chi_a_dn6 = assign70240_e106743_d_n6;
        locals.var_chi_a_dn7 = assign70240_e106743_d_n7;
        locals.var_chi_a_dn8 = assign70240_e106743_d_n8;
        locals.var_chi_a_dn9 = assign70240_e106743_d_n9;
        locals.var_chi_a_dn10 = assign70240_e106743_d_n10;
        locals.var_chi_a_dn11 = assign70240_e106743_d_n11;
        locals.var_chi_a_dn14 = assign70240_e106743_d_n14;

        let (assign70250_e106747, assign70250_e106747_d_n0, assign70250_e106747_d_n2, assign70250_e106747_d_n4, assign70250_e106747_d_n5, assign70250_e106747_d_n6, assign70250_e106747_d_n7, assign70250_e106747_d_n8, assign70250_e106747_d_n9, assign70250_e106747_d_n10, assign70250_e106747_d_n11, assign70250_e106747_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_chi_b, locals.var_chi_b_dn0, locals.var_chi_b_dn2, locals.var_chi_b_dn4, locals.var_chi_b_dn5, locals.var_chi_b_dn6, locals.var_chi_b_dn7, locals.var_chi_b_dn8, locals.var_chi_b_dn9, locals.var_chi_b_dn10, locals.var_chi_b_dn11, locals.var_chi_b_dn14,)
    }
};
        locals.var_chi_b = assign70250_e106747;
        locals.var_chi_b_dn0 = assign70250_e106747_d_n0;
        locals.var_chi_b_dn2 = assign70250_e106747_d_n2;
        locals.var_chi_b_dn4 = assign70250_e106747_d_n4;
        locals.var_chi_b_dn5 = assign70250_e106747_d_n5;
        locals.var_chi_b_dn6 = assign70250_e106747_d_n6;
        locals.var_chi_b_dn7 = assign70250_e106747_d_n7;
        locals.var_chi_b_dn8 = assign70250_e106747_d_n8;
        locals.var_chi_b_dn9 = assign70250_e106747_d_n9;
        locals.var_chi_b_dn10 = assign70250_e106747_d_n10;
        locals.var_chi_b_dn11 = assign70250_e106747_d_n11;
        locals.var_chi_b_dn14 = assign70250_e106747_d_n14;

        let (assign70260_e106752,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign70260_e106750: f64 = (-1.0);
        (assign70260_e106750,)
    } else {
        (locals.var_flg_conv,)
    }
};
        locals.var_flg_conv = assign70260_e106752;

        let (assign70270_e106756, assign70270_e106756_d_n0, assign70270_e106756_d_n2, assign70270_e106756_d_n4, assign70270_e106756_d_n5, assign70270_e106756_d_n6, assign70270_e106756_d_n7, assign70270_e106756_d_n8, assign70270_e106756_d_n9, assign70270_e106756_d_n10, assign70270_e106756_d_n11, assign70270_e106756_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ps0ld_ini, locals.var_ps0ld_ini_dn0, locals.var_ps0ld_ini_dn2, locals.var_ps0ld_ini_dn4, locals.var_ps0ld_ini_dn5, locals.var_ps0ld_ini_dn6, locals.var_ps0ld_ini_dn7, locals.var_ps0ld_ini_dn8, locals.var_ps0ld_ini_dn9, locals.var_ps0ld_ini_dn10, locals.var_ps0ld_ini_dn11, locals.var_ps0ld_ini_dn14,)
    }
};
        locals.var_ps0ld_ini = assign70270_e106756;
        locals.var_ps0ld_ini_dn0 = assign70270_e106756_d_n0;
        locals.var_ps0ld_ini_dn2 = assign70270_e106756_d_n2;
        locals.var_ps0ld_ini_dn4 = assign70270_e106756_d_n4;
        locals.var_ps0ld_ini_dn5 = assign70270_e106756_d_n5;
        locals.var_ps0ld_ini_dn6 = assign70270_e106756_d_n6;
        locals.var_ps0ld_ini_dn7 = assign70270_e106756_d_n7;
        locals.var_ps0ld_ini_dn8 = assign70270_e106756_d_n8;
        locals.var_ps0ld_ini_dn9 = assign70270_e106756_d_n9;
        locals.var_ps0ld_ini_dn10 = assign70270_e106756_d_n10;
        locals.var_ps0ld_ini_dn11 = assign70270_e106756_d_n11;
        locals.var_ps0ld_ini_dn14 = assign70270_e106756_d_n14;

        let (assign70280_e106760, assign70280_e106760_d_n0, assign70280_e106760_d_n2, assign70280_e106760_d_n4, assign70280_e106760_d_n5, assign70280_e106760_d_n6, assign70280_e106760_d_n7, assign70280_e106760_d_n8, assign70280_e106760_d_n9, assign70280_e106760_d_n10, assign70280_e106760_d_n11, assign70280_e106760_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fbsq, locals.var_fbsq_dn0, locals.var_fbsq_dn2, locals.var_fbsq_dn4, locals.var_fbsq_dn5, locals.var_fbsq_dn6, locals.var_fbsq_dn7, locals.var_fbsq_dn8, locals.var_fbsq_dn9, locals.var_fbsq_dn10, locals.var_fbsq_dn11, locals.var_fbsq_dn14,)
    }
};
        locals.var_fbsq = assign70280_e106760;
        locals.var_fbsq_dn0 = assign70280_e106760_d_n0;
        locals.var_fbsq_dn2 = assign70280_e106760_d_n2;
        locals.var_fbsq_dn4 = assign70280_e106760_d_n4;
        locals.var_fbsq_dn5 = assign70280_e106760_d_n5;
        locals.var_fbsq_dn6 = assign70280_e106760_d_n6;
        locals.var_fbsq_dn7 = assign70280_e106760_d_n7;
        locals.var_fbsq_dn8 = assign70280_e106760_d_n8;
        locals.var_fbsq_dn9 = assign70280_e106760_d_n9;
        locals.var_fbsq_dn10 = assign70280_e106760_d_n10;
        locals.var_fbsq_dn11 = assign70280_e106760_d_n11;
        locals.var_fbsq_dn14 = assign70280_e106760_d_n14;

        let (assign70290_e106771, assign70290_e106771_d_n0, assign70290_e106771_d_n2, assign70290_e106771_d_n4, assign70290_e106771_d_n5, assign70290_e106771_d_n6, assign70290_e106771_d_n7, assign70290_e106771_d_n8, assign70290_e106771_d_n9, assign70290_e106771_d_n10, assign70290_e106771_d_n11, assign70290_e106771_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign70290_e106764: f64 = (2.0 * locals.var_beta_inv);
        let assign70290_e106767: f64 = (locals.var_nover_func / locals.var_nin);
        let assign70290_e106768: f64 = (assign70290_e106767).ln();
        let assign70290_e106769: f64 = (assign70290_e106764 * assign70290_e106768);
        (assign70290_e106769, (((2.0 * locals.var_beta_inv_dn0) * assign70290_e106768) + (assign70290_e106764 * ((-((locals.var_nover_func * locals.var_nin_dn0) / (locals.var_nin * locals.var_nin))) / assign70290_e106767))), (((2.0 * locals.var_beta_inv_dn2) * assign70290_e106768) + (assign70290_e106764 * ((-((locals.var_nover_func * locals.var_nin_dn2) / (locals.var_nin * locals.var_nin))) / assign70290_e106767))), (((2.0 * locals.var_beta_inv_dn4) * assign70290_e106768) + (assign70290_e106764 * ((-((locals.var_nover_func * locals.var_nin_dn4) / (locals.var_nin * locals.var_nin))) / assign70290_e106767))), (((2.0 * locals.var_beta_inv_dn5) * assign70290_e106768) + (assign70290_e106764 * ((-((locals.var_nover_func * locals.var_nin_dn5) / (locals.var_nin * locals.var_nin))) / assign70290_e106767))), (((2.0 * locals.var_beta_inv_dn6) * assign70290_e106768) + (assign70290_e106764 * ((-((locals.var_nover_func * locals.var_nin_dn6) / (locals.var_nin * locals.var_nin))) / assign70290_e106767))), (((2.0 * locals.var_beta_inv_dn7) * assign70290_e106768) + (assign70290_e106764 * ((-((locals.var_nover_func * locals.var_nin_dn7) / (locals.var_nin * locals.var_nin))) / assign70290_e106767))), (((2.0 * locals.var_beta_inv_dn8) * assign70290_e106768) + (assign70290_e106764 * ((-((locals.var_nover_func * locals.var_nin_dn8) / (locals.var_nin * locals.var_nin))) / assign70290_e106767))), (((2.0 * locals.var_beta_inv_dn9) * assign70290_e106768) + (assign70290_e106764 * ((-((locals.var_nover_func * locals.var_nin_dn9) / (locals.var_nin * locals.var_nin))) / assign70290_e106767))), (((2.0 * locals.var_beta_inv_dn10) * assign70290_e106768) + (assign70290_e106764 * ((-((locals.var_nover_func * locals.var_nin_dn10) / (locals.var_nin * locals.var_nin))) / assign70290_e106767))), (((2.0 * locals.var_beta_inv_dn11) * assign70290_e106768) + (assign70290_e106764 * ((-((locals.var_nover_func * locals.var_nin_dn11) / (locals.var_nin * locals.var_nin))) / assign70290_e106767))), (((2.0 * locals.var_beta_inv_dn14) * assign70290_e106768) + (assign70290_e106764 * ((-((locals.var_nover_func * locals.var_nin_dn14) / (locals.var_nin * locals.var_nin))) / assign70290_e106767))),)
    } else {
        (locals.var_pb2over, locals.var_pb2over_dn0, locals.var_pb2over_dn2, locals.var_pb2over_dn4, locals.var_pb2over_dn5, locals.var_pb2over_dn6, locals.var_pb2over_dn7, locals.var_pb2over_dn8, locals.var_pb2over_dn9, locals.var_pb2over_dn10, locals.var_pb2over_dn11, locals.var_pb2over_dn14,)
    }
};
        locals.var_pb2over = assign70290_e106771;
        locals.var_pb2over_dn0 = assign70290_e106771_d_n0;
        locals.var_pb2over_dn2 = assign70290_e106771_d_n2;
        locals.var_pb2over_dn4 = assign70290_e106771_d_n4;
        locals.var_pb2over_dn5 = assign70290_e106771_d_n5;
        locals.var_pb2over_dn6 = assign70290_e106771_d_n6;
        locals.var_pb2over_dn7 = assign70290_e106771_d_n7;
        locals.var_pb2over_dn8 = assign70290_e106771_d_n8;
        locals.var_pb2over_dn9 = assign70290_e106771_d_n9;
        locals.var_pb2over_dn10 = assign70290_e106771_d_n10;
        locals.var_pb2over_dn11 = assign70290_e106771_d_n11;
        locals.var_pb2over_dn14 = assign70290_e106771_d_n14;

        let (assign70300_e106779, assign70300_e106779_d_n0, assign70300_e106779_d_n2, assign70300_e106779_d_n4, assign70300_e106779_d_n5, assign70300_e106779_d_n6, assign70300_e106779_d_n7, assign70300_e106779_d_n8, assign70300_e106779_d_n9, assign70300_e106779_d_n10, assign70300_e106779_d_n11, assign70300_e106779_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign70300_e106775: f64 = (0.8 - locals.var_pb2over);
        let assign70300_e106777: f64 = (assign70300_e106775 - 0.1);
        (assign70300_e106777, (-locals.var_pb2over_dn0), (-locals.var_pb2over_dn2), (-locals.var_pb2over_dn4), (-locals.var_pb2over_dn5), (-locals.var_pb2over_dn6), (-locals.var_pb2over_dn7), (-locals.var_pb2over_dn8), (-locals.var_pb2over_dn9), (-locals.var_pb2over_dn10), (-locals.var_pb2over_dn11), (-locals.var_pb2over_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign70300_e106779;
        locals.var_tmf1_dn0 = assign70300_e106779_d_n0;
        locals.var_tmf1_dn2 = assign70300_e106779_d_n2;
        locals.var_tmf1_dn4 = assign70300_e106779_d_n4;
        locals.var_tmf1_dn5 = assign70300_e106779_d_n5;
        locals.var_tmf1_dn6 = assign70300_e106779_d_n6;
        locals.var_tmf1_dn7 = assign70300_e106779_d_n7;
        locals.var_tmf1_dn8 = assign70300_e106779_d_n8;
        locals.var_tmf1_dn9 = assign70300_e106779_d_n9;
        locals.var_tmf1_dn10 = assign70300_e106779_d_n10;
        locals.var_tmf1_dn11 = assign70300_e106779_d_n11;
        locals.var_tmf1_dn14 = assign70300_e106779_d_n14;

        let (assign70310_e106787, assign70310_e106787_d_n0, assign70310_e106787_d_n2, assign70310_e106787_d_n4, assign70310_e106787_d_n5, assign70310_e106787_d_n6, assign70310_e106787_d_n7, assign70310_e106787_d_n8, assign70310_e106787_d_n9, assign70310_e106787_d_n10, assign70310_e106787_d_n11, assign70310_e106787_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign70310_e106783: f64 = (4.0 * 0.8);
        let assign70310_e106785: f64 = (assign70310_e106783 * 0.1);
        (assign70310_e106785, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign70310_e106787;
        locals.var_tmf2_dn0 = assign70310_e106787_d_n0;
        locals.var_tmf2_dn2 = assign70310_e106787_d_n2;
        locals.var_tmf2_dn4 = assign70310_e106787_d_n4;
        locals.var_tmf2_dn5 = assign70310_e106787_d_n5;
        locals.var_tmf2_dn6 = assign70310_e106787_d_n6;
        locals.var_tmf2_dn7 = assign70310_e106787_d_n7;
        locals.var_tmf2_dn8 = assign70310_e106787_d_n8;
        locals.var_tmf2_dn9 = assign70310_e106787_d_n9;
        locals.var_tmf2_dn10 = assign70310_e106787_d_n10;
        locals.var_tmf2_dn11 = assign70310_e106787_d_n11;
        locals.var_tmf2_dn14 = assign70310_e106787_d_n14;

        let (assign70320_e106797, assign70320_e106797_d_n0, assign70320_e106797_d_n2, assign70320_e106797_d_n4, assign70320_e106797_d_n5, assign70320_e106797_d_n6, assign70320_e106797_d_n7, assign70320_e106797_d_n8, assign70320_e106797_d_n9, assign70320_e106797_d_n10, assign70320_e106797_d_n11, assign70320_e106797_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let (assign70320_e106795, assign70320_e106795_d_n0, assign70320_e106795_d_n2, assign70320_e106795_d_n4, assign70320_e106795_d_n5, assign70320_e106795_d_n6, assign70320_e106795_d_n7, assign70320_e106795_d_n8, assign70320_e106795_d_n9, assign70320_e106795_d_n10, assign70320_e106795_d_n11, assign70320_e106795_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign70320_e106794: f64 = (-locals.var_tmf2);
                (assign70320_e106794, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign70320_e106795, assign70320_e106795_d_n0, assign70320_e106795_d_n2, assign70320_e106795_d_n4, assign70320_e106795_d_n5, assign70320_e106795_d_n6, assign70320_e106795_d_n7, assign70320_e106795_d_n8, assign70320_e106795_d_n9, assign70320_e106795_d_n10, assign70320_e106795_d_n11, assign70320_e106795_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign70320_e106797;
        locals.var_tmf2_dn0 = assign70320_e106797_d_n0;
        locals.var_tmf2_dn2 = assign70320_e106797_d_n2;
        locals.var_tmf2_dn4 = assign70320_e106797_d_n4;
        locals.var_tmf2_dn5 = assign70320_e106797_d_n5;
        locals.var_tmf2_dn6 = assign70320_e106797_d_n6;
        locals.var_tmf2_dn7 = assign70320_e106797_d_n7;
        locals.var_tmf2_dn8 = assign70320_e106797_d_n8;
        locals.var_tmf2_dn9 = assign70320_e106797_d_n9;
        locals.var_tmf2_dn10 = assign70320_e106797_d_n10;
        locals.var_tmf2_dn11 = assign70320_e106797_d_n11;
        locals.var_tmf2_dn14 = assign70320_e106797_d_n14;

        let (assign70330_e106806, assign70330_e106806_d_n0, assign70330_e106806_d_n2, assign70330_e106806_d_n4, assign70330_e106806_d_n5, assign70330_e106806_d_n6, assign70330_e106806_d_n7, assign70330_e106806_d_n8, assign70330_e106806_d_n9, assign70330_e106806_d_n10, assign70330_e106806_d_n11, assign70330_e106806_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign70330_e106801: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign70330_e106803: f64 = (assign70330_e106801 + locals.var_tmf2);
        let assign70330_e106804: f64 = (assign70330_e106803).sqrt();
        (assign70330_e106804, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign70330_e106804)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign70330_e106804)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign70330_e106804)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign70330_e106804)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign70330_e106804)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign70330_e106804)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign70330_e106804)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign70330_e106804)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign70330_e106804)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign70330_e106804)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign70330_e106804)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign70330_e106806;
        locals.var_tmf2_dn0 = assign70330_e106806_d_n0;
        locals.var_tmf2_dn2 = assign70330_e106806_d_n2;
        locals.var_tmf2_dn4 = assign70330_e106806_d_n4;
        locals.var_tmf2_dn5 = assign70330_e106806_d_n5;
        locals.var_tmf2_dn6 = assign70330_e106806_d_n6;
        locals.var_tmf2_dn7 = assign70330_e106806_d_n7;
        locals.var_tmf2_dn8 = assign70330_e106806_d_n8;
        locals.var_tmf2_dn9 = assign70330_e106806_d_n9;
        locals.var_tmf2_dn10 = assign70330_e106806_d_n10;
        locals.var_tmf2_dn11 = assign70330_e106806_d_n11;
        locals.var_tmf2_dn14 = assign70330_e106806_d_n14;

        let (assign70340_e106816, assign70340_e106816_d_n0, assign70340_e106816_d_n2, assign70340_e106816_d_n4, assign70340_e106816_d_n5, assign70340_e106816_d_n6, assign70340_e106816_d_n7, assign70340_e106816_d_n8, assign70340_e106816_d_n9, assign70340_e106816_d_n10, assign70340_e106816_d_n11, assign70340_e106816_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign70340_e106812: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign70340_e106813: f64 = (1.0 + assign70340_e106812);
        let assign70340_e106814: f64 = (0.5 * assign70340_e106813);
        (assign70340_e106814, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign70340_e106816;
        locals.var_t0_dn0 = assign70340_e106816_d_n0;
        locals.var_t0_dn2 = assign70340_e106816_d_n2;
        locals.var_t0_dn4 = assign70340_e106816_d_n4;
        locals.var_t0_dn5 = assign70340_e106816_d_n5;
        locals.var_t0_dn6 = assign70340_e106816_d_n6;
        locals.var_t0_dn7 = assign70340_e106816_d_n7;
        locals.var_t0_dn8 = assign70340_e106816_d_n8;
        locals.var_t0_dn9 = assign70340_e106816_d_n9;
        locals.var_t0_dn10 = assign70340_e106816_d_n10;
        locals.var_t0_dn11 = assign70340_e106816_d_n11;
        locals.var_t0_dn14 = assign70340_e106816_d_n14;

        let (assign70350_e106826, assign70350_e106826_d_n0, assign70350_e106826_d_n2, assign70350_e106826_d_n4, assign70350_e106826_d_n5, assign70350_e106826_d_n6, assign70350_e106826_d_n7, assign70350_e106826_d_n8, assign70350_e106826_d_n9, assign70350_e106826_d_n10, assign70350_e106826_d_n11, assign70350_e106826_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign70350_e106822: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign70350_e106823: f64 = (0.5 * assign70350_e106822);
        let assign70350_e106824: f64 = (0.8 - assign70350_e106823);
        (assign70350_e106824, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (-(0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (-(0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (-(0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (-(0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (-(0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (-(0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (-(0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (-(0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (-(0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_vbs_max_over, locals.var_vbs_max_over_dn0, locals.var_vbs_max_over_dn2, locals.var_vbs_max_over_dn4, locals.var_vbs_max_over_dn5, locals.var_vbs_max_over_dn6, locals.var_vbs_max_over_dn7, locals.var_vbs_max_over_dn8, locals.var_vbs_max_over_dn9, locals.var_vbs_max_over_dn10, locals.var_vbs_max_over_dn11, locals.var_vbs_max_over_dn14,)
    }
};
        locals.var_vbs_max_over = assign70350_e106826;
        locals.var_vbs_max_over_dn0 = assign70350_e106826_d_n0;
        locals.var_vbs_max_over_dn2 = assign70350_e106826_d_n2;
        locals.var_vbs_max_over_dn4 = assign70350_e106826_d_n4;
        locals.var_vbs_max_over_dn5 = assign70350_e106826_d_n5;
        locals.var_vbs_max_over_dn6 = assign70350_e106826_d_n6;
        locals.var_vbs_max_over_dn7 = assign70350_e106826_d_n7;
        locals.var_vbs_max_over_dn8 = assign70350_e106826_d_n8;
        locals.var_vbs_max_over_dn9 = assign70350_e106826_d_n9;
        locals.var_vbs_max_over_dn10 = assign70350_e106826_d_n10;
        locals.var_vbs_max_over_dn11 = assign70350_e106826_d_n11;
        locals.var_vbs_max_over_dn14 = assign70350_e106826_d_n14;

        let assign70360_e106830: f64 = (locals.var_vbs_max_over * 0.5);
        let assign70360_e106831: f64 = if locals.var_vbs_bnd_over > assign70360_e106830 { 1.0 } else { 0.0 };
        locals.var_guard1655 = assign70360_e106831;

        let (assign70370_e106839, assign70370_e106839_d_n0, assign70370_e106839_d_n2, assign70370_e106839_d_n4, assign70370_e106839_d_n5, assign70370_e106839_d_n6, assign70370_e106839_d_n7, assign70370_e106839_d_n8, assign70370_e106839_d_n9, assign70370_e106839_d_n10, assign70370_e106839_d_n11, assign70370_e106839_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1655 != 0.0)) {
        let assign70370_e106837: f64 = (0.5 * locals.var_vbs_max_over);
        (assign70370_e106837, (0.5 * locals.var_vbs_max_over_dn0), (0.5 * locals.var_vbs_max_over_dn2), (0.5 * locals.var_vbs_max_over_dn4), (0.5 * locals.var_vbs_max_over_dn5), (0.5 * locals.var_vbs_max_over_dn6), (0.5 * locals.var_vbs_max_over_dn7), (0.5 * locals.var_vbs_max_over_dn8), (0.5 * locals.var_vbs_max_over_dn9), (0.5 * locals.var_vbs_max_over_dn10), (0.5 * locals.var_vbs_max_over_dn11), (0.5 * locals.var_vbs_max_over_dn14),)
    } else {
        (locals.var_vbs_bnd_over, locals.var_vbs_bnd_over_dn0, locals.var_vbs_bnd_over_dn2, locals.var_vbs_bnd_over_dn4, locals.var_vbs_bnd_over_dn5, locals.var_vbs_bnd_over_dn6, locals.var_vbs_bnd_over_dn7, locals.var_vbs_bnd_over_dn8, locals.var_vbs_bnd_over_dn9, locals.var_vbs_bnd_over_dn10, locals.var_vbs_bnd_over_dn11, locals.var_vbs_bnd_over_dn14,)
    }
};
        locals.var_vbs_bnd_over = assign70370_e106839;
        locals.var_vbs_bnd_over_dn0 = assign70370_e106839_d_n0;
        locals.var_vbs_bnd_over_dn2 = assign70370_e106839_d_n2;
        locals.var_vbs_bnd_over_dn4 = assign70370_e106839_d_n4;
        locals.var_vbs_bnd_over_dn5 = assign70370_e106839_d_n5;
        locals.var_vbs_bnd_over_dn6 = assign70370_e106839_d_n6;
        locals.var_vbs_bnd_over_dn7 = assign70370_e106839_d_n7;
        locals.var_vbs_bnd_over_dn8 = assign70370_e106839_d_n8;
        locals.var_vbs_bnd_over_dn9 = assign70370_e106839_d_n9;
        locals.var_vbs_bnd_over_dn10 = assign70370_e106839_d_n10;
        locals.var_vbs_bnd_over_dn11 = assign70370_e106839_d_n11;
        locals.var_vbs_bnd_over_dn14 = assign70370_e106839_d_n14;

        let assign70380_e106841: f64 = if param_given[338] { 1.0 } else { 0.0 };
        locals.var_guard1656 = assign70380_e106841;

    }

    pub(super) fn stamp_transient_block_251(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign70390_e106847, assign70390_e106847_d_n0, assign70390_e106847_d_n2, assign70390_e106847_d_n4, assign70390_e106847_d_n5, assign70390_e106847_d_n6, assign70390_e106847_d_n7, assign70390_e106847_d_n8, assign70390_e106847_d_n9, assign70390_e106847_d_n10, assign70390_e106847_d_n11, assign70390_e106847_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1656 != 0.0)) {
        (p.p338, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbs_max_over, locals.var_vbs_max_over_dn0, locals.var_vbs_max_over_dn2, locals.var_vbs_max_over_dn4, locals.var_vbs_max_over_dn5, locals.var_vbs_max_over_dn6, locals.var_vbs_max_over_dn7, locals.var_vbs_max_over_dn8, locals.var_vbs_max_over_dn9, locals.var_vbs_max_over_dn10, locals.var_vbs_max_over_dn11, locals.var_vbs_max_over_dn14,)
    }
};
        locals.var_vbs_max_over = assign70390_e106847;
        locals.var_vbs_max_over_dn0 = assign70390_e106847_d_n0;
        locals.var_vbs_max_over_dn2 = assign70390_e106847_d_n2;
        locals.var_vbs_max_over_dn4 = assign70390_e106847_d_n4;
        locals.var_vbs_max_over_dn5 = assign70390_e106847_d_n5;
        locals.var_vbs_max_over_dn6 = assign70390_e106847_d_n6;
        locals.var_vbs_max_over_dn7 = assign70390_e106847_d_n7;
        locals.var_vbs_max_over_dn8 = assign70390_e106847_d_n8;
        locals.var_vbs_max_over_dn9 = assign70390_e106847_d_n9;
        locals.var_vbs_max_over_dn10 = assign70390_e106847_d_n10;
        locals.var_vbs_max_over_dn11 = assign70390_e106847_d_n11;
        locals.var_vbs_max_over_dn14 = assign70390_e106847_d_n14;

        let assign70400_e106849: f64 = if param_given[339] { 1.0 } else { 0.0 };
        locals.var_guard1657 = assign70400_e106849;

        let (assign70410_e106855, assign70410_e106855_d_n0, assign70410_e106855_d_n2, assign70410_e106855_d_n4, assign70410_e106855_d_n5, assign70410_e106855_d_n6, assign70410_e106855_d_n7, assign70410_e106855_d_n8, assign70410_e106855_d_n9, assign70410_e106855_d_n10, assign70410_e106855_d_n11, assign70410_e106855_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1657 != 0.0)) {
        (p.p339, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbs_bnd_over, locals.var_vbs_bnd_over_dn0, locals.var_vbs_bnd_over_dn2, locals.var_vbs_bnd_over_dn4, locals.var_vbs_bnd_over_dn5, locals.var_vbs_bnd_over_dn6, locals.var_vbs_bnd_over_dn7, locals.var_vbs_bnd_over_dn8, locals.var_vbs_bnd_over_dn9, locals.var_vbs_bnd_over_dn10, locals.var_vbs_bnd_over_dn11, locals.var_vbs_bnd_over_dn14,)
    }
};
        locals.var_vbs_bnd_over = assign70410_e106855;
        locals.var_vbs_bnd_over_dn0 = assign70410_e106855_d_n0;
        locals.var_vbs_bnd_over_dn2 = assign70410_e106855_d_n2;
        locals.var_vbs_bnd_over_dn4 = assign70410_e106855_d_n4;
        locals.var_vbs_bnd_over_dn5 = assign70410_e106855_d_n5;
        locals.var_vbs_bnd_over_dn6 = assign70410_e106855_d_n6;
        locals.var_vbs_bnd_over_dn7 = assign70410_e106855_d_n7;
        locals.var_vbs_bnd_over_dn8 = assign70410_e106855_d_n8;
        locals.var_vbs_bnd_over_dn9 = assign70410_e106855_d_n9;
        locals.var_vbs_bnd_over_dn10 = assign70410_e106855_d_n10;
        locals.var_vbs_bnd_over_dn11 = assign70410_e106855_d_n11;
        locals.var_vbs_bnd_over_dn14 = assign70410_e106855_d_n14;

        let assign70420_e106857: f64 = if param_given[338] { 1.0 } else { 0.0 };
        locals.var_guard1658 = assign70420_e106857;

        let (assign70430_e106868, assign70430_e106868_d_n0, assign70430_e106868_d_n2, assign70430_e106868_d_n4, assign70430_e106868_d_n5, assign70430_e106868_d_n6, assign70430_e106868_d_n7, assign70430_e106868_d_n8, assign70430_e106868_d_n9, assign70430_e106868_d_n10, assign70430_e106868_d_n11, assign70430_e106868_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1657 == 0.0)) && (locals.var_guard1658 != 0.0)) {
        let assign70430_e106866: f64 = (0.5 * locals.var_vbs_max_over);
        (assign70430_e106866, (0.5 * locals.var_vbs_max_over_dn0), (0.5 * locals.var_vbs_max_over_dn2), (0.5 * locals.var_vbs_max_over_dn4), (0.5 * locals.var_vbs_max_over_dn5), (0.5 * locals.var_vbs_max_over_dn6), (0.5 * locals.var_vbs_max_over_dn7), (0.5 * locals.var_vbs_max_over_dn8), (0.5 * locals.var_vbs_max_over_dn9), (0.5 * locals.var_vbs_max_over_dn10), (0.5 * locals.var_vbs_max_over_dn11), (0.5 * locals.var_vbs_max_over_dn14),)
    } else {
        (locals.var_vbs_bnd_over, locals.var_vbs_bnd_over_dn0, locals.var_vbs_bnd_over_dn2, locals.var_vbs_bnd_over_dn4, locals.var_vbs_bnd_over_dn5, locals.var_vbs_bnd_over_dn6, locals.var_vbs_bnd_over_dn7, locals.var_vbs_bnd_over_dn8, locals.var_vbs_bnd_over_dn9, locals.var_vbs_bnd_over_dn10, locals.var_vbs_bnd_over_dn11, locals.var_vbs_bnd_over_dn14,)
    }
};
        locals.var_vbs_bnd_over = assign70430_e106868;
        locals.var_vbs_bnd_over_dn0 = assign70430_e106868_d_n0;
        locals.var_vbs_bnd_over_dn2 = assign70430_e106868_d_n2;
        locals.var_vbs_bnd_over_dn4 = assign70430_e106868_d_n4;
        locals.var_vbs_bnd_over_dn5 = assign70430_e106868_d_n5;
        locals.var_vbs_bnd_over_dn6 = assign70430_e106868_d_n6;
        locals.var_vbs_bnd_over_dn7 = assign70430_e106868_d_n7;
        locals.var_vbs_bnd_over_dn8 = assign70430_e106868_d_n8;
        locals.var_vbs_bnd_over_dn9 = assign70430_e106868_d_n9;
        locals.var_vbs_bnd_over_dn10 = assign70430_e106868_d_n10;
        locals.var_vbs_bnd_over_dn11 = assign70430_e106868_d_n11;
        locals.var_vbs_bnd_over_dn14 = assign70430_e106868_d_n14;

        let assign70440_e106872: f64 = (locals.var_vbs_max_over * 0.5);
        let assign70440_e106873: f64 = if locals.var_vbs_bnd_over > assign70440_e106872 { 1.0 } else { 0.0 };
        locals.var_guard1659 = assign70440_e106873;

        let (assign70450_e106881, assign70450_e106881_d_n0, assign70450_e106881_d_n2, assign70450_e106881_d_n4, assign70450_e106881_d_n5, assign70450_e106881_d_n6, assign70450_e106881_d_n7, assign70450_e106881_d_n8, assign70450_e106881_d_n9, assign70450_e106881_d_n10, assign70450_e106881_d_n11, assign70450_e106881_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1659 != 0.0)) {
        let assign70450_e106879: f64 = (0.5 * locals.var_vbs_max_over);
        (assign70450_e106879, (0.5 * locals.var_vbs_max_over_dn0), (0.5 * locals.var_vbs_max_over_dn2), (0.5 * locals.var_vbs_max_over_dn4), (0.5 * locals.var_vbs_max_over_dn5), (0.5 * locals.var_vbs_max_over_dn6), (0.5 * locals.var_vbs_max_over_dn7), (0.5 * locals.var_vbs_max_over_dn8), (0.5 * locals.var_vbs_max_over_dn9), (0.5 * locals.var_vbs_max_over_dn10), (0.5 * locals.var_vbs_max_over_dn11), (0.5 * locals.var_vbs_max_over_dn14),)
    } else {
        (locals.var_vbs_bnd_over, locals.var_vbs_bnd_over_dn0, locals.var_vbs_bnd_over_dn2, locals.var_vbs_bnd_over_dn4, locals.var_vbs_bnd_over_dn5, locals.var_vbs_bnd_over_dn6, locals.var_vbs_bnd_over_dn7, locals.var_vbs_bnd_over_dn8, locals.var_vbs_bnd_over_dn9, locals.var_vbs_bnd_over_dn10, locals.var_vbs_bnd_over_dn11, locals.var_vbs_bnd_over_dn14,)
    }
};
        locals.var_vbs_bnd_over = assign70450_e106881;
        locals.var_vbs_bnd_over_dn0 = assign70450_e106881_d_n0;
        locals.var_vbs_bnd_over_dn2 = assign70450_e106881_d_n2;
        locals.var_vbs_bnd_over_dn4 = assign70450_e106881_d_n4;
        locals.var_vbs_bnd_over_dn5 = assign70450_e106881_d_n5;
        locals.var_vbs_bnd_over_dn6 = assign70450_e106881_d_n6;
        locals.var_vbs_bnd_over_dn7 = assign70450_e106881_d_n7;
        locals.var_vbs_bnd_over_dn8 = assign70450_e106881_d_n8;
        locals.var_vbs_bnd_over_dn9 = assign70450_e106881_d_n9;
        locals.var_vbs_bnd_over_dn10 = assign70450_e106881_d_n10;
        locals.var_vbs_bnd_over_dn11 = assign70450_e106881_d_n11;
        locals.var_vbs_bnd_over_dn14 = assign70450_e106881_d_n14;

        let assign70460_e106884: f64 = if p.p38 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1660 = assign70460_e106884;

        let (assign70470_e106891, assign70470_e106891_d_n0, assign70470_e106891_d_n2, assign70470_e106891_d_n4, assign70470_e106891_d_n5, assign70470_e106891_d_n6, assign70470_e106891_d_n7, assign70470_e106891_d_n8, assign70470_e106891_d_n9, assign70470_e106891_d_n10, assign70470_e106891_d_n11, assign70470_e106891_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1660 != 0.0)) {
        let assign70470_e106889: f64 = (-locals.var_vxbgmt);
        (assign70470_e106889, (-locals.var_vxbgmt_dn0), (-locals.var_vxbgmt_dn2), (-locals.var_vxbgmt_dn4), (-locals.var_vxbgmt_dn5), (-locals.var_vxbgmt_dn6), (-locals.var_vxbgmt_dn7), (-locals.var_vxbgmt_dn8), (-locals.var_vxbgmt_dn9), (-locals.var_vxbgmt_dn10), (-locals.var_vxbgmt_dn11), (-locals.var_vxbgmt_dn14),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign70470_e106891;
        locals.var_t0_dn0 = assign70470_e106891_d_n0;
        locals.var_t0_dn2 = assign70470_e106891_d_n2;
        locals.var_t0_dn4 = assign70470_e106891_d_n4;
        locals.var_t0_dn5 = assign70470_e106891_d_n5;
        locals.var_t0_dn6 = assign70470_e106891_d_n6;
        locals.var_t0_dn7 = assign70470_e106891_d_n7;
        locals.var_t0_dn8 = assign70470_e106891_d_n8;
        locals.var_t0_dn9 = assign70470_e106891_d_n9;
        locals.var_t0_dn10 = assign70470_e106891_d_n10;
        locals.var_t0_dn11 = assign70470_e106891_d_n11;
        locals.var_t0_dn14 = assign70470_e106891_d_n14;

        let assign70480_e106894: f64 = if locals.var_t0 > locals.var_vbs_bnd_over { 1.0 } else { 0.0 };
        locals.var_guard1661 = assign70480_e106894;

        let (assign70490_e106904, assign70490_e106904_d_n0, assign70490_e106904_d_n2, assign70490_e106904_d_n4, assign70490_e106904_d_n5, assign70490_e106904_d_n6, assign70490_e106904_d_n7, assign70490_e106904_d_n8, assign70490_e106904_d_n9, assign70490_e106904_d_n10, assign70490_e106904_d_n11, assign70490_e106904_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1660 != 0.0)) && (locals.var_guard1661 != 0.0)) {
        let assign70490_e106902: f64 = (locals.var_t0 - locals.var_vbs_bnd_over);
        (assign70490_e106902, (locals.var_t0_dn0 - locals.var_vbs_bnd_over_dn0), (locals.var_t0_dn2 - locals.var_vbs_bnd_over_dn2), (locals.var_t0_dn4 - locals.var_vbs_bnd_over_dn4), (locals.var_t0_dn5 - locals.var_vbs_bnd_over_dn5), (locals.var_t0_dn6 - locals.var_vbs_bnd_over_dn6), (locals.var_t0_dn7 - locals.var_vbs_bnd_over_dn7), (locals.var_t0_dn8 - locals.var_vbs_bnd_over_dn8), (locals.var_t0_dn9 - locals.var_vbs_bnd_over_dn9), (locals.var_t0_dn10 - locals.var_vbs_bnd_over_dn10), (locals.var_t0_dn11 - locals.var_vbs_bnd_over_dn11), (locals.var_t0_dn14 - locals.var_vbs_bnd_over_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign70490_e106904;
        locals.var_t1_dn0 = assign70490_e106904_d_n0;
        locals.var_t1_dn2 = assign70490_e106904_d_n2;
        locals.var_t1_dn4 = assign70490_e106904_d_n4;
        locals.var_t1_dn5 = assign70490_e106904_d_n5;
        locals.var_t1_dn6 = assign70490_e106904_d_n6;
        locals.var_t1_dn7 = assign70490_e106904_d_n7;
        locals.var_t1_dn8 = assign70490_e106904_d_n8;
        locals.var_t1_dn9 = assign70490_e106904_d_n9;
        locals.var_t1_dn10 = assign70490_e106904_d_n10;
        locals.var_t1_dn11 = assign70490_e106904_d_n11;
        locals.var_t1_dn14 = assign70490_e106904_d_n14;

        let (assign70500_e106914, assign70500_e106914_d_n0, assign70500_e106914_d_n2, assign70500_e106914_d_n4, assign70500_e106914_d_n5, assign70500_e106914_d_n6, assign70500_e106914_d_n7, assign70500_e106914_d_n8, assign70500_e106914_d_n9, assign70500_e106914_d_n10, assign70500_e106914_d_n11, assign70500_e106914_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1660 != 0.0)) && (locals.var_guard1661 != 0.0)) {
        let assign70500_e106912: f64 = (locals.var_vbs_max_over - locals.var_vbs_bnd_over);
        (assign70500_e106912, (locals.var_vbs_max_over_dn0 - locals.var_vbs_bnd_over_dn0), (locals.var_vbs_max_over_dn2 - locals.var_vbs_bnd_over_dn2), (locals.var_vbs_max_over_dn4 - locals.var_vbs_bnd_over_dn4), (locals.var_vbs_max_over_dn5 - locals.var_vbs_bnd_over_dn5), (locals.var_vbs_max_over_dn6 - locals.var_vbs_bnd_over_dn6), (locals.var_vbs_max_over_dn7 - locals.var_vbs_bnd_over_dn7), (locals.var_vbs_max_over_dn8 - locals.var_vbs_bnd_over_dn8), (locals.var_vbs_max_over_dn9 - locals.var_vbs_bnd_over_dn9), (locals.var_vbs_max_over_dn10 - locals.var_vbs_bnd_over_dn10), (locals.var_vbs_max_over_dn11 - locals.var_vbs_bnd_over_dn11), (locals.var_vbs_max_over_dn14 - locals.var_vbs_bnd_over_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign70500_e106914;
        locals.var_t2_dn0 = assign70500_e106914_d_n0;
        locals.var_t2_dn2 = assign70500_e106914_d_n2;
        locals.var_t2_dn4 = assign70500_e106914_d_n4;
        locals.var_t2_dn5 = assign70500_e106914_d_n5;
        locals.var_t2_dn6 = assign70500_e106914_d_n6;
        locals.var_t2_dn7 = assign70500_e106914_d_n7;
        locals.var_t2_dn8 = assign70500_e106914_d_n8;
        locals.var_t2_dn9 = assign70500_e106914_d_n9;
        locals.var_t2_dn10 = assign70500_e106914_d_n10;
        locals.var_t2_dn11 = assign70500_e106914_d_n11;
        locals.var_t2_dn14 = assign70500_e106914_d_n14;

        let (assign70510_e106924, assign70510_e106924_d_n0, assign70510_e106924_d_n2, assign70510_e106924_d_n4, assign70510_e106924_d_n5, assign70510_e106924_d_n6, assign70510_e106924_d_n7, assign70510_e106924_d_n8, assign70510_e106924_d_n9, assign70510_e106924_d_n10, assign70510_e106924_d_n11, assign70510_e106924_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1660 != 0.0)) && (locals.var_guard1661 != 0.0)) {
        let assign70510_e106922: f64 = (locals.var_t1 / locals.var_t2);
        (assign70510_e106922, (((locals.var_t1_dn0 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn0)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn2 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn2)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn4 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn4)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn5 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn5)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn6 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn6)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn7 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn7)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn8 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn8)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn9 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn9)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn10 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn10)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn11 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn11)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn14 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn14)) / (locals.var_t2 * locals.var_t2)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign70510_e106924;
        locals.var_tmf1_dn0 = assign70510_e106924_d_n0;
        locals.var_tmf1_dn2 = assign70510_e106924_d_n2;
        locals.var_tmf1_dn4 = assign70510_e106924_d_n4;
        locals.var_tmf1_dn5 = assign70510_e106924_d_n5;
        locals.var_tmf1_dn6 = assign70510_e106924_d_n6;
        locals.var_tmf1_dn7 = assign70510_e106924_d_n7;
        locals.var_tmf1_dn8 = assign70510_e106924_d_n8;
        locals.var_tmf1_dn9 = assign70510_e106924_d_n9;
        locals.var_tmf1_dn10 = assign70510_e106924_d_n10;
        locals.var_tmf1_dn11 = assign70510_e106924_d_n11;
        locals.var_tmf1_dn14 = assign70510_e106924_d_n14;

        let (assign70520_e106934, assign70520_e106934_d_n0, assign70520_e106934_d_n2, assign70520_e106934_d_n4, assign70520_e106934_d_n5, assign70520_e106934_d_n6, assign70520_e106934_d_n7, assign70520_e106934_d_n8, assign70520_e106934_d_n9, assign70520_e106934_d_n10, assign70520_e106934_d_n11, assign70520_e106934_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1660 != 0.0)) && (locals.var_guard1661 != 0.0)) {
        let assign70520_e106932: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign70520_e106932, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign70520_e106934;
        locals.var_tmf2_dn0 = assign70520_e106934_d_n0;
        locals.var_tmf2_dn2 = assign70520_e106934_d_n2;
        locals.var_tmf2_dn4 = assign70520_e106934_d_n4;
        locals.var_tmf2_dn5 = assign70520_e106934_d_n5;
        locals.var_tmf2_dn6 = assign70520_e106934_d_n6;
        locals.var_tmf2_dn7 = assign70520_e106934_d_n7;
        locals.var_tmf2_dn8 = assign70520_e106934_d_n8;
        locals.var_tmf2_dn9 = assign70520_e106934_d_n9;
        locals.var_tmf2_dn10 = assign70520_e106934_d_n10;
        locals.var_tmf2_dn11 = assign70520_e106934_d_n11;
        locals.var_tmf2_dn14 = assign70520_e106934_d_n14;

        let (assign70530_e106944, assign70530_e106944_d_n0, assign70530_e106944_d_n2, assign70530_e106944_d_n4, assign70530_e106944_d_n5, assign70530_e106944_d_n6, assign70530_e106944_d_n7, assign70530_e106944_d_n8, assign70530_e106944_d_n9, assign70530_e106944_d_n10, assign70530_e106944_d_n11, assign70530_e106944_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1660 != 0.0)) && (locals.var_guard1661 != 0.0)) {
        let assign70530_e106942: f64 = (locals.var_tmf2 * locals.var_tmf1);
        (assign70530_e106942, ((locals.var_tmf2_dn0 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn0)), ((locals.var_tmf2_dn2 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn2)), ((locals.var_tmf2_dn4 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn4)), ((locals.var_tmf2_dn5 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn5)), ((locals.var_tmf2_dn6 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn6)), ((locals.var_tmf2_dn7 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn7)), ((locals.var_tmf2_dn8 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn8)), ((locals.var_tmf2_dn9 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn9)), ((locals.var_tmf2_dn10 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn10)), ((locals.var_tmf2_dn11 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn11)), ((locals.var_tmf2_dn14 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_tmf3, locals.var_tmf3_dn0, locals.var_tmf3_dn2, locals.var_tmf3_dn4, locals.var_tmf3_dn5, locals.var_tmf3_dn6, locals.var_tmf3_dn7, locals.var_tmf3_dn8, locals.var_tmf3_dn9, locals.var_tmf3_dn10, locals.var_tmf3_dn11, locals.var_tmf3_dn14,)
    }
};
        locals.var_tmf3 = assign70530_e106944;
        locals.var_tmf3_dn0 = assign70530_e106944_d_n0;
        locals.var_tmf3_dn2 = assign70530_e106944_d_n2;
        locals.var_tmf3_dn4 = assign70530_e106944_d_n4;
        locals.var_tmf3_dn5 = assign70530_e106944_d_n5;
        locals.var_tmf3_dn6 = assign70530_e106944_d_n6;
        locals.var_tmf3_dn7 = assign70530_e106944_d_n7;
        locals.var_tmf3_dn8 = assign70530_e106944_d_n8;
        locals.var_tmf3_dn9 = assign70530_e106944_d_n9;
        locals.var_tmf3_dn10 = assign70530_e106944_d_n10;
        locals.var_tmf3_dn11 = assign70530_e106944_d_n11;
        locals.var_tmf3_dn14 = assign70530_e106944_d_n14;

        let (assign70540_e106954, assign70540_e106954_d_n0, assign70540_e106954_d_n2, assign70540_e106954_d_n4, assign70540_e106954_d_n5, assign70540_e106954_d_n6, assign70540_e106954_d_n7, assign70540_e106954_d_n8, assign70540_e106954_d_n9, assign70540_e106954_d_n10, assign70540_e106954_d_n11, assign70540_e106954_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1660 != 0.0)) && (locals.var_guard1661 != 0.0)) {
        let assign70540_e106952: f64 = (locals.var_tmf2 * locals.var_tmf2);
        (assign70540_e106952, ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0)), ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2)), ((locals.var_tmf2_dn4 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn4)), ((locals.var_tmf2_dn5 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn5)), ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6)), ((locals.var_tmf2_dn7 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn7)), ((locals.var_tmf2_dn8 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn8)), ((locals.var_tmf2_dn9 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn9)), ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10)), ((locals.var_tmf2_dn11 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn11)), ((locals.var_tmf2_dn14 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn14)),)
    } else {
        (locals.var_tmf4, locals.var_tmf4_dn0, locals.var_tmf4_dn2, locals.var_tmf4_dn4, locals.var_tmf4_dn5, locals.var_tmf4_dn6, locals.var_tmf4_dn7, locals.var_tmf4_dn8, locals.var_tmf4_dn9, locals.var_tmf4_dn10, locals.var_tmf4_dn11, locals.var_tmf4_dn14,)
    }
};
        locals.var_tmf4 = assign70540_e106954;
        locals.var_tmf4_dn0 = assign70540_e106954_d_n0;
        locals.var_tmf4_dn2 = assign70540_e106954_d_n2;
        locals.var_tmf4_dn4 = assign70540_e106954_d_n4;
        locals.var_tmf4_dn5 = assign70540_e106954_d_n5;
        locals.var_tmf4_dn6 = assign70540_e106954_d_n6;
        locals.var_tmf4_dn7 = assign70540_e106954_d_n7;
        locals.var_tmf4_dn8 = assign70540_e106954_d_n8;
        locals.var_tmf4_dn9 = assign70540_e106954_d_n9;
        locals.var_tmf4_dn10 = assign70540_e106954_d_n10;
        locals.var_tmf4_dn11 = assign70540_e106954_d_n11;
        locals.var_tmf4_dn14 = assign70540_e106954_d_n14;

        let (assign70550_e106972, assign70550_e106972_d_n0, assign70550_e106972_d_n2, assign70550_e106972_d_n4, assign70550_e106972_d_n5, assign70550_e106972_d_n6, assign70550_e106972_d_n7, assign70550_e106972_d_n8, assign70550_e106972_d_n9, assign70550_e106972_d_n10, assign70550_e106972_d_n11, assign70550_e106972_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1660 != 0.0)) && (locals.var_guard1661 != 0.0)) {
        let assign70550_e106963: f64 = (1.0 + locals.var_tmf1);
        let assign70550_e106965: f64 = (assign70550_e106963 + locals.var_tmf2);
        let assign70550_e106967: f64 = (assign70550_e106965 + locals.var_tmf3);
        let assign70550_e106969: f64 = (assign70550_e106967 + locals.var_tmf4);
        let assign70550_e106970: f64 = (1.0 / assign70550_e106969);
        (assign70550_e106970, (-((((locals.var_tmf1_dn0 + locals.var_tmf2_dn0) + locals.var_tmf3_dn0) + locals.var_tmf4_dn0) / (assign70550_e106969 * assign70550_e106969))), (-((((locals.var_tmf1_dn2 + locals.var_tmf2_dn2) + locals.var_tmf3_dn2) + locals.var_tmf4_dn2) / (assign70550_e106969 * assign70550_e106969))), (-((((locals.var_tmf1_dn4 + locals.var_tmf2_dn4) + locals.var_tmf3_dn4) + locals.var_tmf4_dn4) / (assign70550_e106969 * assign70550_e106969))), (-((((locals.var_tmf1_dn5 + locals.var_tmf2_dn5) + locals.var_tmf3_dn5) + locals.var_tmf4_dn5) / (assign70550_e106969 * assign70550_e106969))), (-((((locals.var_tmf1_dn6 + locals.var_tmf2_dn6) + locals.var_tmf3_dn6) + locals.var_tmf4_dn6) / (assign70550_e106969 * assign70550_e106969))), (-((((locals.var_tmf1_dn7 + locals.var_tmf2_dn7) + locals.var_tmf3_dn7) + locals.var_tmf4_dn7) / (assign70550_e106969 * assign70550_e106969))), (-((((locals.var_tmf1_dn8 + locals.var_tmf2_dn8) + locals.var_tmf3_dn8) + locals.var_tmf4_dn8) / (assign70550_e106969 * assign70550_e106969))), (-((((locals.var_tmf1_dn9 + locals.var_tmf2_dn9) + locals.var_tmf3_dn9) + locals.var_tmf4_dn9) / (assign70550_e106969 * assign70550_e106969))), (-((((locals.var_tmf1_dn10 + locals.var_tmf2_dn10) + locals.var_tmf3_dn10) + locals.var_tmf4_dn10) / (assign70550_e106969 * assign70550_e106969))), (-((((locals.var_tmf1_dn11 + locals.var_tmf2_dn11) + locals.var_tmf3_dn11) + locals.var_tmf4_dn11) / (assign70550_e106969 * assign70550_e106969))), (-((((locals.var_tmf1_dn14 + locals.var_tmf2_dn14) + locals.var_tmf3_dn14) + locals.var_tmf4_dn14) / (assign70550_e106969 * assign70550_e106969))),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign70550_e106972;
        locals.var_tmf0_dn0 = assign70550_e106972_d_n0;
        locals.var_tmf0_dn2 = assign70550_e106972_d_n2;
        locals.var_tmf0_dn4 = assign70550_e106972_d_n4;
        locals.var_tmf0_dn5 = assign70550_e106972_d_n5;
        locals.var_tmf0_dn6 = assign70550_e106972_d_n6;
        locals.var_tmf0_dn7 = assign70550_e106972_d_n7;
        locals.var_tmf0_dn8 = assign70550_e106972_d_n8;
        locals.var_tmf0_dn9 = assign70550_e106972_d_n9;
        locals.var_tmf0_dn10 = assign70550_e106972_d_n10;
        locals.var_tmf0_dn11 = assign70550_e106972_d_n11;
        locals.var_tmf0_dn14 = assign70550_e106972_d_n14;

        let (assign70560_e106997, assign70560_e106997_d_n0, assign70560_e106997_d_n2, assign70560_e106997_d_n4, assign70560_e106997_d_n5, assign70560_e106997_d_n6, assign70560_e106997_d_n7, assign70560_e106997_d_n8, assign70560_e106997_d_n9, assign70560_e106997_d_n10, assign70560_e106997_d_n11, assign70560_e106997_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1660 != 0.0)) && (locals.var_guard1661 != 0.0)) {
        let assign70560_e106981: f64 = (2.0 * locals.var_tmf1);
        let assign70560_e106982: f64 = (1.0 + assign70560_e106981);
        let assign70560_e106985: f64 = (3.0 * locals.var_tmf2);
        let assign70560_e106986: f64 = (assign70560_e106982 + assign70560_e106985);
        let assign70560_e106989: f64 = (4.0 * locals.var_tmf3);
        let assign70560_e106990: f64 = (assign70560_e106986 + assign70560_e106989);
        let assign70560_e106991: f64 = (-assign70560_e106990);
        let assign70560_e106993: f64 = (assign70560_e106991 * locals.var_tmf0);
        let assign70560_e106995: f64 = (assign70560_e106993 * locals.var_tmf0);
        (assign70560_e106995, (((((-(((2.0 * locals.var_tmf1_dn0) + (3.0 * locals.var_tmf2_dn0)) + (4.0 * locals.var_tmf3_dn0))) * locals.var_tmf0) + (assign70560_e106991 * locals.var_tmf0_dn0)) * locals.var_tmf0) + (assign70560_e106993 * locals.var_tmf0_dn0)), (((((-(((2.0 * locals.var_tmf1_dn2) + (3.0 * locals.var_tmf2_dn2)) + (4.0 * locals.var_tmf3_dn2))) * locals.var_tmf0) + (assign70560_e106991 * locals.var_tmf0_dn2)) * locals.var_tmf0) + (assign70560_e106993 * locals.var_tmf0_dn2)), (((((-(((2.0 * locals.var_tmf1_dn4) + (3.0 * locals.var_tmf2_dn4)) + (4.0 * locals.var_tmf3_dn4))) * locals.var_tmf0) + (assign70560_e106991 * locals.var_tmf0_dn4)) * locals.var_tmf0) + (assign70560_e106993 * locals.var_tmf0_dn4)), (((((-(((2.0 * locals.var_tmf1_dn5) + (3.0 * locals.var_tmf2_dn5)) + (4.0 * locals.var_tmf3_dn5))) * locals.var_tmf0) + (assign70560_e106991 * locals.var_tmf0_dn5)) * locals.var_tmf0) + (assign70560_e106993 * locals.var_tmf0_dn5)), (((((-(((2.0 * locals.var_tmf1_dn6) + (3.0 * locals.var_tmf2_dn6)) + (4.0 * locals.var_tmf3_dn6))) * locals.var_tmf0) + (assign70560_e106991 * locals.var_tmf0_dn6)) * locals.var_tmf0) + (assign70560_e106993 * locals.var_tmf0_dn6)), (((((-(((2.0 * locals.var_tmf1_dn7) + (3.0 * locals.var_tmf2_dn7)) + (4.0 * locals.var_tmf3_dn7))) * locals.var_tmf0) + (assign70560_e106991 * locals.var_tmf0_dn7)) * locals.var_tmf0) + (assign70560_e106993 * locals.var_tmf0_dn7)), (((((-(((2.0 * locals.var_tmf1_dn8) + (3.0 * locals.var_tmf2_dn8)) + (4.0 * locals.var_tmf3_dn8))) * locals.var_tmf0) + (assign70560_e106991 * locals.var_tmf0_dn8)) * locals.var_tmf0) + (assign70560_e106993 * locals.var_tmf0_dn8)), (((((-(((2.0 * locals.var_tmf1_dn9) + (3.0 * locals.var_tmf2_dn9)) + (4.0 * locals.var_tmf3_dn9))) * locals.var_tmf0) + (assign70560_e106991 * locals.var_tmf0_dn9)) * locals.var_tmf0) + (assign70560_e106993 * locals.var_tmf0_dn9)), (((((-(((2.0 * locals.var_tmf1_dn10) + (3.0 * locals.var_tmf2_dn10)) + (4.0 * locals.var_tmf3_dn10))) * locals.var_tmf0) + (assign70560_e106991 * locals.var_tmf0_dn10)) * locals.var_tmf0) + (assign70560_e106993 * locals.var_tmf0_dn10)), (((((-(((2.0 * locals.var_tmf1_dn11) + (3.0 * locals.var_tmf2_dn11)) + (4.0 * locals.var_tmf3_dn11))) * locals.var_tmf0) + (assign70560_e106991 * locals.var_tmf0_dn11)) * locals.var_tmf0) + (assign70560_e106993 * locals.var_tmf0_dn11)), (((((-(((2.0 * locals.var_tmf1_dn14) + (3.0 * locals.var_tmf2_dn14)) + (4.0 * locals.var_tmf3_dn14))) * locals.var_tmf0) + (assign70560_e106991 * locals.var_tmf0_dn14)) * locals.var_tmf0) + (assign70560_e106993 * locals.var_tmf0_dn14)),)
    } else {
        (locals.var_t11, locals.var_t11_dn0, locals.var_t11_dn2, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11, locals.var_t11_dn14,)
    }
};
        locals.var_t11 = assign70560_e106997;
        locals.var_t11_dn0 = assign70560_e106997_d_n0;
        locals.var_t11_dn2 = assign70560_e106997_d_n2;
        locals.var_t11_dn4 = assign70560_e106997_d_n4;
        locals.var_t11_dn5 = assign70560_e106997_d_n5;
        locals.var_t11_dn6 = assign70560_e106997_d_n6;
        locals.var_t11_dn7 = assign70560_e106997_d_n7;
        locals.var_t11_dn8 = assign70560_e106997_d_n8;
        locals.var_t11_dn9 = assign70560_e106997_d_n9;
        locals.var_t11_dn10 = assign70560_e106997_d_n10;
        locals.var_t11_dn11 = assign70560_e106997_d_n11;
        locals.var_t11_dn14 = assign70560_e106997_d_n14;

        let (assign70570_e107009, assign70570_e107009_d_n0, assign70570_e107009_d_n2, assign70570_e107009_d_n4, assign70570_e107009_d_n5, assign70570_e107009_d_n6, assign70570_e107009_d_n7, assign70570_e107009_d_n8, assign70570_e107009_d_n9, assign70570_e107009_d_n10, assign70570_e107009_d_n11, assign70570_e107009_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1660 != 0.0)) && (locals.var_guard1661 != 0.0)) {
        let assign70570_e107006: f64 = (1.0 - locals.var_tmf0);
        let assign70570_e107007: f64 = (locals.var_t2 * assign70570_e107006);
        (assign70570_e107007, ((locals.var_t2_dn0 * assign70570_e107006) + (locals.var_t2 * (-locals.var_tmf0_dn0))), ((locals.var_t2_dn2 * assign70570_e107006) + (locals.var_t2 * (-locals.var_tmf0_dn2))), ((locals.var_t2_dn4 * assign70570_e107006) + (locals.var_t2 * (-locals.var_tmf0_dn4))), ((locals.var_t2_dn5 * assign70570_e107006) + (locals.var_t2 * (-locals.var_tmf0_dn5))), ((locals.var_t2_dn6 * assign70570_e107006) + (locals.var_t2 * (-locals.var_tmf0_dn6))), ((locals.var_t2_dn7 * assign70570_e107006) + (locals.var_t2 * (-locals.var_tmf0_dn7))), ((locals.var_t2_dn8 * assign70570_e107006) + (locals.var_t2 * (-locals.var_tmf0_dn8))), ((locals.var_t2_dn9 * assign70570_e107006) + (locals.var_t2 * (-locals.var_tmf0_dn9))), ((locals.var_t2_dn10 * assign70570_e107006) + (locals.var_t2 * (-locals.var_tmf0_dn10))), ((locals.var_t2_dn11 * assign70570_e107006) + (locals.var_t2 * (-locals.var_tmf0_dn11))), ((locals.var_t2_dn14 * assign70570_e107006) + (locals.var_t2 * (-locals.var_tmf0_dn14))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn14,)
    }
};
        locals.var_ty = assign70570_e107009;
        locals.var_ty_dn0 = assign70570_e107009_d_n0;
        locals.var_ty_dn2 = assign70570_e107009_d_n2;
        locals.var_ty_dn4 = assign70570_e107009_d_n4;
        locals.var_ty_dn5 = assign70570_e107009_d_n5;
        locals.var_ty_dn6 = assign70570_e107009_d_n6;
        locals.var_ty_dn7 = assign70570_e107009_d_n7;
        locals.var_ty_dn8 = assign70570_e107009_d_n8;
        locals.var_ty_dn9 = assign70570_e107009_d_n9;
        locals.var_ty_dn10 = assign70570_e107009_d_n10;
        locals.var_ty_dn11 = assign70570_e107009_d_n11;
        locals.var_ty_dn14 = assign70570_e107009_d_n14;

        let (assign70580_e107023, assign70580_e107023_d_n0, assign70580_e107023_d_n2, assign70580_e107023_d_n4, assign70580_e107023_d_n5, assign70580_e107023_d_n6, assign70580_e107023_d_n7, assign70580_e107023_d_n8, assign70580_e107023_d_n9, assign70580_e107023_d_n10, assign70580_e107023_d_n11, assign70580_e107023_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1660 != 0.0)) && (locals.var_guard1661 != 0.0)) {
        let assign70580_e107017: f64 = (1.0 - locals.var_tmf0);
        let assign70580_e107020: f64 = (locals.var_tmf1 * locals.var_t11);
        let assign70580_e107021: f64 = (assign70580_e107017 + assign70580_e107020);
        (assign70580_e107021, ((-locals.var_tmf0_dn0) + ((locals.var_tmf1_dn0 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn0))), ((-locals.var_tmf0_dn2) + ((locals.var_tmf1_dn2 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn2))), ((-locals.var_tmf0_dn4) + ((locals.var_tmf1_dn4 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn4))), ((-locals.var_tmf0_dn5) + ((locals.var_tmf1_dn5 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn5))), ((-locals.var_tmf0_dn6) + ((locals.var_tmf1_dn6 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn6))), ((-locals.var_tmf0_dn7) + ((locals.var_tmf1_dn7 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn7))), ((-locals.var_tmf0_dn8) + ((locals.var_tmf1_dn8 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn8))), ((-locals.var_tmf0_dn9) + ((locals.var_tmf1_dn9 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn9))), ((-locals.var_tmf0_dn10) + ((locals.var_tmf1_dn10 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn10))), ((-locals.var_tmf0_dn11) + ((locals.var_tmf1_dn11 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn11))), ((-locals.var_tmf0_dn14) + ((locals.var_tmf1_dn14 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn14))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign70580_e107023;
        locals.var_t0_dn0 = assign70580_e107023_d_n0;
        locals.var_t0_dn2 = assign70580_e107023_d_n2;
        locals.var_t0_dn4 = assign70580_e107023_d_n4;
        locals.var_t0_dn5 = assign70580_e107023_d_n5;
        locals.var_t0_dn6 = assign70580_e107023_d_n6;
        locals.var_t0_dn7 = assign70580_e107023_d_n7;
        locals.var_t0_dn8 = assign70580_e107023_d_n8;
        locals.var_t0_dn9 = assign70580_e107023_d_n9;
        locals.var_t0_dn10 = assign70580_e107023_d_n10;
        locals.var_t0_dn11 = assign70580_e107023_d_n11;
        locals.var_t0_dn14 = assign70580_e107023_d_n14;

        let (assign70590_e107032, assign70590_e107032_d_n0, assign70590_e107032_d_n2, assign70590_e107032_d_n4, assign70590_e107032_d_n5, assign70590_e107032_d_n6, assign70590_e107032_d_n7, assign70590_e107032_d_n8, assign70590_e107032_d_n9, assign70590_e107032_d_n10, assign70590_e107032_d_n11, assign70590_e107032_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1660 != 0.0)) && (locals.var_guard1661 != 0.0)) {
        let assign70590_e107030: f64 = (-locals.var_t11);
        (assign70590_e107030, (-locals.var_t11_dn0), (-locals.var_t11_dn2), (-locals.var_t11_dn4), (-locals.var_t11_dn5), (-locals.var_t11_dn6), (-locals.var_t11_dn7), (-locals.var_t11_dn8), (-locals.var_t11_dn9), (-locals.var_t11_dn10), (-locals.var_t11_dn11), (-locals.var_t11_dn14),)
    } else {
        (locals.var_t11, locals.var_t11_dn0, locals.var_t11_dn2, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11, locals.var_t11_dn14,)
    }
};
        locals.var_t11 = assign70590_e107032;
        locals.var_t11_dn0 = assign70590_e107032_d_n0;
        locals.var_t11_dn2 = assign70590_e107032_d_n2;
        locals.var_t11_dn4 = assign70590_e107032_d_n4;
        locals.var_t11_dn5 = assign70590_e107032_d_n5;
        locals.var_t11_dn6 = assign70590_e107032_d_n6;
        locals.var_t11_dn7 = assign70590_e107032_d_n7;
        locals.var_t11_dn8 = assign70590_e107032_d_n8;
        locals.var_t11_dn9 = assign70590_e107032_d_n9;
        locals.var_t11_dn10 = assign70590_e107032_d_n10;
        locals.var_t11_dn11 = assign70590_e107032_d_n11;
        locals.var_t11_dn14 = assign70590_e107032_d_n14;

        let (assign70600_e107042, assign70600_e107042_d_n0, assign70600_e107042_d_n2, assign70600_e107042_d_n4, assign70600_e107042_d_n5, assign70600_e107042_d_n6, assign70600_e107042_d_n7, assign70600_e107042_d_n8, assign70600_e107042_d_n9, assign70600_e107042_d_n10, assign70600_e107042_d_n11, assign70600_e107042_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1660 != 0.0)) && (locals.var_guard1661 != 0.0)) {
        let assign70600_e107040: f64 = (locals.var_vbs_bnd_over + locals.var_ty);
        (assign70600_e107040, (locals.var_vbs_bnd_over_dn0 + locals.var_ty_dn0), (locals.var_vbs_bnd_over_dn2 + locals.var_ty_dn2), (locals.var_vbs_bnd_over_dn4 + locals.var_ty_dn4), (locals.var_vbs_bnd_over_dn5 + locals.var_ty_dn5), (locals.var_vbs_bnd_over_dn6 + locals.var_ty_dn6), (locals.var_vbs_bnd_over_dn7 + locals.var_ty_dn7), (locals.var_vbs_bnd_over_dn8 + locals.var_ty_dn8), (locals.var_vbs_bnd_over_dn9 + locals.var_ty_dn9), (locals.var_vbs_bnd_over_dn10 + locals.var_ty_dn10), (locals.var_vbs_bnd_over_dn11 + locals.var_ty_dn11), (locals.var_vbs_bnd_over_dn14 + locals.var_ty_dn14),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign70600_e107042;
        locals.var_t10_dn0 = assign70600_e107042_d_n0;
        locals.var_t10_dn2 = assign70600_e107042_d_n2;
        locals.var_t10_dn4 = assign70600_e107042_d_n4;
        locals.var_t10_dn5 = assign70600_e107042_d_n5;
        locals.var_t10_dn6 = assign70600_e107042_d_n6;
        locals.var_t10_dn7 = assign70600_e107042_d_n7;
        locals.var_t10_dn8 = assign70600_e107042_d_n8;
        locals.var_t10_dn9 = assign70600_e107042_d_n9;
        locals.var_t10_dn10 = assign70600_e107042_d_n10;
        locals.var_t10_dn11 = assign70600_e107042_d_n11;
        locals.var_t10_dn14 = assign70600_e107042_d_n14;

        let (assign70610_e107051, assign70610_e107051_d_n0, assign70610_e107051_d_n2, assign70610_e107051_d_n4, assign70610_e107051_d_n5, assign70610_e107051_d_n6, assign70610_e107051_d_n7, assign70610_e107051_d_n8, assign70610_e107051_d_n9, assign70610_e107051_d_n10, assign70610_e107051_d_n11, assign70610_e107051_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1660 != 0.0)) && (locals.var_guard1661 == 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign70610_e107051;
        locals.var_t10_dn0 = assign70610_e107051_d_n0;
        locals.var_t10_dn2 = assign70610_e107051_d_n2;
        locals.var_t10_dn4 = assign70610_e107051_d_n4;
        locals.var_t10_dn5 = assign70610_e107051_d_n5;
        locals.var_t10_dn6 = assign70610_e107051_d_n6;
        locals.var_t10_dn7 = assign70610_e107051_d_n7;
        locals.var_t10_dn8 = assign70610_e107051_d_n8;
        locals.var_t10_dn9 = assign70610_e107051_d_n9;
        locals.var_t10_dn10 = assign70610_e107051_d_n10;
        locals.var_t10_dn11 = assign70610_e107051_d_n11;
        locals.var_t10_dn14 = assign70610_e107051_d_n14;

        let (assign70620_e107058, assign70620_e107058_d_n0, assign70620_e107058_d_n2, assign70620_e107058_d_n4, assign70620_e107058_d_n5, assign70620_e107058_d_n6, assign70620_e107058_d_n7, assign70620_e107058_d_n8, assign70620_e107058_d_n9, assign70620_e107058_d_n10, assign70620_e107058_d_n11, assign70620_e107058_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1660 != 0.0)) {
        let assign70620_e107056: f64 = (-locals.var_t10);
        (assign70620_e107056, (-locals.var_t10_dn0), (-locals.var_t10_dn2), (-locals.var_t10_dn4), (-locals.var_t10_dn5), (-locals.var_t10_dn6), (-locals.var_t10_dn7), (-locals.var_t10_dn8), (-locals.var_t10_dn9), (-locals.var_t10_dn10), (-locals.var_t10_dn11), (-locals.var_t10_dn14),)
    } else {
        (locals.var_vxbgmtcl, locals.var_vxbgmtcl_dn0, locals.var_vxbgmtcl_dn2, locals.var_vxbgmtcl_dn4, locals.var_vxbgmtcl_dn5, locals.var_vxbgmtcl_dn6, locals.var_vxbgmtcl_dn7, locals.var_vxbgmtcl_dn8, locals.var_vxbgmtcl_dn9, locals.var_vxbgmtcl_dn10, locals.var_vxbgmtcl_dn11, locals.var_vxbgmtcl_dn14,)
    }
};
        locals.var_vxbgmtcl = assign70620_e107058;
        locals.var_vxbgmtcl_dn0 = assign70620_e107058_d_n0;
        locals.var_vxbgmtcl_dn2 = assign70620_e107058_d_n2;
        locals.var_vxbgmtcl_dn4 = assign70620_e107058_d_n4;
        locals.var_vxbgmtcl_dn5 = assign70620_e107058_d_n5;
        locals.var_vxbgmtcl_dn6 = assign70620_e107058_d_n6;
        locals.var_vxbgmtcl_dn7 = assign70620_e107058_d_n7;
        locals.var_vxbgmtcl_dn8 = assign70620_e107058_d_n8;
        locals.var_vxbgmtcl_dn9 = assign70620_e107058_d_n9;
        locals.var_vxbgmtcl_dn10 = assign70620_e107058_d_n10;
        locals.var_vxbgmtcl_dn11 = assign70620_e107058_d_n11;
        locals.var_vxbgmtcl_dn14 = assign70620_e107058_d_n14;

        let (assign70630_e107065, assign70630_e107065_d_n0, assign70630_e107065_d_n2, assign70630_e107065_d_n4, assign70630_e107065_d_n5, assign70630_e107065_d_n6, assign70630_e107065_d_n7, assign70630_e107065_d_n8, assign70630_e107065_d_n9, assign70630_e107065_d_n10, assign70630_e107065_d_n11, assign70630_e107065_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1660 == 0.0)) {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn11, locals.var_vxbgmt_dn14,)
    } else {
        (locals.var_vxbgmtcl, locals.var_vxbgmtcl_dn0, locals.var_vxbgmtcl_dn2, locals.var_vxbgmtcl_dn4, locals.var_vxbgmtcl_dn5, locals.var_vxbgmtcl_dn6, locals.var_vxbgmtcl_dn7, locals.var_vxbgmtcl_dn8, locals.var_vxbgmtcl_dn9, locals.var_vxbgmtcl_dn10, locals.var_vxbgmtcl_dn11, locals.var_vxbgmtcl_dn14,)
    }
};
        locals.var_vxbgmtcl = assign70630_e107065;
        locals.var_vxbgmtcl_dn0 = assign70630_e107065_d_n0;
        locals.var_vxbgmtcl_dn2 = assign70630_e107065_d_n2;
        locals.var_vxbgmtcl_dn4 = assign70630_e107065_d_n4;
        locals.var_vxbgmtcl_dn5 = assign70630_e107065_d_n5;
        locals.var_vxbgmtcl_dn6 = assign70630_e107065_d_n6;
        locals.var_vxbgmtcl_dn7 = assign70630_e107065_d_n7;
        locals.var_vxbgmtcl_dn8 = assign70630_e107065_d_n8;
        locals.var_vxbgmtcl_dn9 = assign70630_e107065_d_n9;
        locals.var_vxbgmtcl_dn10 = assign70630_e107065_d_n10;
        locals.var_vxbgmtcl_dn11 = assign70630_e107065_d_n11;
        locals.var_vxbgmtcl_dn14 = assign70630_e107065_d_n14;

        let (assign70640_e107071, assign70640_e107071_d_n0, assign70640_e107071_d_n2, assign70640_e107071_d_n4, assign70640_e107071_d_n5, assign70640_e107071_d_n6, assign70640_e107071_d_n7, assign70640_e107071_d_n8, assign70640_e107071_d_n9, assign70640_e107071_d_n10, assign70640_e107071_d_n11, assign70640_e107071_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign70640_e107069: f64 = (locals.var_cnst0over_func / locals.var_cox0_func);
        (assign70640_e107069, (locals.var_cnst0over_func_dn0 / locals.var_cox0_func), (locals.var_cnst0over_func_dn2 / locals.var_cox0_func), (locals.var_cnst0over_func_dn4 / locals.var_cox0_func), (locals.var_cnst0over_func_dn5 / locals.var_cox0_func), (locals.var_cnst0over_func_dn6 / locals.var_cox0_func), (locals.var_cnst0over_func_dn7 / locals.var_cox0_func), (locals.var_cnst0over_func_dn8 / locals.var_cox0_func), (locals.var_cnst0over_func_dn9 / locals.var_cox0_func), (locals.var_cnst0over_func_dn10 / locals.var_cox0_func), (locals.var_cnst0over_func_dn11 / locals.var_cox0_func), (locals.var_cnst0over_func_dn14 / locals.var_cox0_func),)
    } else {
        (locals.var_fac1, locals.var_fac1_dn0, locals.var_fac1_dn2, locals.var_fac1_dn4, locals.var_fac1_dn5, locals.var_fac1_dn6, locals.var_fac1_dn7, locals.var_fac1_dn8, locals.var_fac1_dn9, locals.var_fac1_dn10, locals.var_fac1_dn11, locals.var_fac1_dn14,)
    }
};
        locals.var_fac1 = assign70640_e107071;
        locals.var_fac1_dn0 = assign70640_e107071_d_n0;
        locals.var_fac1_dn2 = assign70640_e107071_d_n2;
        locals.var_fac1_dn4 = assign70640_e107071_d_n4;
        locals.var_fac1_dn5 = assign70640_e107071_d_n5;
        locals.var_fac1_dn6 = assign70640_e107071_d_n6;
        locals.var_fac1_dn7 = assign70640_e107071_d_n7;
        locals.var_fac1_dn8 = assign70640_e107071_d_n8;
        locals.var_fac1_dn9 = assign70640_e107071_d_n9;
        locals.var_fac1_dn10 = assign70640_e107071_d_n10;
        locals.var_fac1_dn11 = assign70640_e107071_d_n11;
        locals.var_fac1_dn14 = assign70640_e107071_d_n14;

        let (assign70650_e107077, assign70650_e107077_d_n0, assign70650_e107077_d_n2, assign70650_e107077_d_n4, assign70650_e107077_d_n5, assign70650_e107077_d_n6, assign70650_e107077_d_n7, assign70650_e107077_d_n8, assign70650_e107077_d_n9, assign70650_e107077_d_n10, assign70650_e107077_d_n11, assign70650_e107077_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign70650_e107075: f64 = (locals.var_fac1 * locals.var_fac1);
        (assign70650_e107075, ((locals.var_fac1_dn0 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn0)), ((locals.var_fac1_dn2 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn2)), ((locals.var_fac1_dn4 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn4)), ((locals.var_fac1_dn5 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn5)), ((locals.var_fac1_dn6 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn6)), ((locals.var_fac1_dn7 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn7)), ((locals.var_fac1_dn8 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn8)), ((locals.var_fac1_dn9 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn9)), ((locals.var_fac1_dn10 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn10)), ((locals.var_fac1_dn11 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn11)), ((locals.var_fac1_dn14 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn14)),)
    } else {
        (locals.var_fac1p2, locals.var_fac1p2_dn0, locals.var_fac1p2_dn2, locals.var_fac1p2_dn4, locals.var_fac1p2_dn5, locals.var_fac1p2_dn6, locals.var_fac1p2_dn7, locals.var_fac1p2_dn8, locals.var_fac1p2_dn9, locals.var_fac1p2_dn10, locals.var_fac1p2_dn11, locals.var_fac1p2_dn14,)
    }
};
        locals.var_fac1p2 = assign70650_e107077;
        locals.var_fac1p2_dn0 = assign70650_e107077_d_n0;
        locals.var_fac1p2_dn2 = assign70650_e107077_d_n2;
        locals.var_fac1p2_dn4 = assign70650_e107077_d_n4;
        locals.var_fac1p2_dn5 = assign70650_e107077_d_n5;
        locals.var_fac1p2_dn6 = assign70650_e107077_d_n6;
        locals.var_fac1p2_dn7 = assign70650_e107077_d_n7;
        locals.var_fac1p2_dn8 = assign70650_e107077_d_n8;
        locals.var_fac1p2_dn9 = assign70650_e107077_d_n9;
        locals.var_fac1p2_dn10 = assign70650_e107077_d_n10;
        locals.var_fac1p2_dn11 = assign70650_e107077_d_n11;
        locals.var_fac1p2_dn14 = assign70650_e107077_d_n14;

        let (assign70660_e107084, assign70660_e107084_d_n2, assign70660_e107084_d_n7, assign70660_e107084_d_n8, assign70660_e107084_d_n9,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign70660_e107080: f64 = (-locals.var_vgbgmt);
        let assign70660_e107082: f64 = (assign70660_e107080 + locals.var_uc_vfbover);
        (assign70660_e107082, (-locals.var_vgbgmt_dn2), (-locals.var_vgbgmt_dn7), (-locals.var_vgbgmt_dn8), (-locals.var_vgbgmt_dn9),)
    } else {
        (locals.var_vgpld, locals.var_vgpld_dn2, locals.var_vgpld_dn7, locals.var_vgpld_dn8, locals.var_vgpld_dn9,)
    }
};
        locals.var_vgpld = assign70660_e107084;
        locals.var_vgpld_dn2 = assign70660_e107084_d_n2;
        locals.var_vgpld_dn7 = assign70660_e107084_d_n7;
        locals.var_vgpld_dn8 = assign70660_e107084_d_n8;
        locals.var_vgpld_dn9 = assign70660_e107084_d_n9;

    }

    pub(super) fn stamp_transient_block_252(
        locals: &mut StampLocals,
    ) {
        let (assign70670_e107093,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign70670_e107087: f64 = (-locals.var_vxbgmtcl);
        let assign70670_e107090: f64 = (10.0 * 2.220446049250313e-16);
        let assign70670_e107091: f64 = (assign70670_e107087 + assign70670_e107090);
        (assign70670_e107091,)
    } else {
        (locals.var_vgb_fb_ld,)
    }
};
        locals.var_vgb_fb_ld = assign70670_e107093;

        let (assign70680_e107097, assign70680_e107097_d_n0, assign70680_e107097_d_n2, assign70680_e107097_d_n4, assign70680_e107097_d_n5, assign70680_e107097_d_n6, assign70680_e107097_d_n7, assign70680_e107097_d_n8, assign70680_e107097_d_n9, assign70680_e107097_d_n10, assign70680_e107097_d_n11, assign70680_e107097_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_dep_ld, locals.var_q_dep_ld_dn0, locals.var_q_dep_ld_dn2, locals.var_q_dep_ld_dn4, locals.var_q_dep_ld_dn5, locals.var_q_dep_ld_dn6, locals.var_q_dep_ld_dn7, locals.var_q_dep_ld_dn8, locals.var_q_dep_ld_dn9, locals.var_q_dep_ld_dn10, locals.var_q_dep_ld_dn11, locals.var_q_dep_ld_dn14,)
    }
};
        locals.var_q_dep_ld = assign70680_e107097;
        locals.var_q_dep_ld_dn0 = assign70680_e107097_d_n0;
        locals.var_q_dep_ld_dn2 = assign70680_e107097_d_n2;
        locals.var_q_dep_ld_dn4 = assign70680_e107097_d_n4;
        locals.var_q_dep_ld_dn5 = assign70680_e107097_d_n5;
        locals.var_q_dep_ld_dn6 = assign70680_e107097_d_n6;
        locals.var_q_dep_ld_dn7 = assign70680_e107097_d_n7;
        locals.var_q_dep_ld_dn8 = assign70680_e107097_d_n8;
        locals.var_q_dep_ld_dn9 = assign70680_e107097_d_n9;
        locals.var_q_dep_ld_dn10 = assign70680_e107097_d_n10;
        locals.var_q_dep_ld_dn11 = assign70680_e107097_d_n11;
        locals.var_q_dep_ld_dn14 = assign70680_e107097_d_n14;

        let (assign70690_e107103,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign70690_e107101: f64 = (1.6021918e-19 * locals.var_nover_func);
        (assign70690_e107101,)
    } else {
        (locals.var_q_nsubld,)
    }
};
        locals.var_q_nsubld = assign70690_e107103;

        let (assign70700_e107109, assign70700_e107109_d_n0, assign70700_e107109_d_n2, assign70700_e107109_d_n4, assign70700_e107109_d_n5, assign70700_e107109_d_n6, assign70700_e107109_d_n7, assign70700_e107109_d_n8, assign70700_e107109_d_n9, assign70700_e107109_d_n10, assign70700_e107109_d_n11, assign70700_e107109_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign70700_e107107: f64 = (locals.var_nin / locals.var_nover_func);
        (assign70700_e107107, (locals.var_nin_dn0 / locals.var_nover_func), (locals.var_nin_dn2 / locals.var_nover_func), (locals.var_nin_dn4 / locals.var_nover_func), (locals.var_nin_dn5 / locals.var_nover_func), (locals.var_nin_dn6 / locals.var_nover_func), (locals.var_nin_dn7 / locals.var_nover_func), (locals.var_nin_dn8 / locals.var_nover_func), (locals.var_nin_dn9 / locals.var_nover_func), (locals.var_nin_dn10 / locals.var_nover_func), (locals.var_nin_dn11 / locals.var_nover_func), (locals.var_nin_dn14 / locals.var_nover_func),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign70700_e107109;
        locals.var_t0_dn0 = assign70700_e107109_d_n0;
        locals.var_t0_dn2 = assign70700_e107109_d_n2;
        locals.var_t0_dn4 = assign70700_e107109_d_n4;
        locals.var_t0_dn5 = assign70700_e107109_d_n5;
        locals.var_t0_dn6 = assign70700_e107109_d_n6;
        locals.var_t0_dn7 = assign70700_e107109_d_n7;
        locals.var_t0_dn8 = assign70700_e107109_d_n8;
        locals.var_t0_dn9 = assign70700_e107109_d_n9;
        locals.var_t0_dn10 = assign70700_e107109_d_n10;
        locals.var_t0_dn11 = assign70700_e107109_d_n11;
        locals.var_t0_dn14 = assign70700_e107109_d_n14;

        let (assign70710_e107115, assign70710_e107115_d_n0, assign70710_e107115_d_n2, assign70710_e107115_d_n4, assign70710_e107115_d_n5, assign70710_e107115_d_n6, assign70710_e107115_d_n7, assign70710_e107115_d_n8, assign70710_e107115_d_n9, assign70710_e107115_d_n10, assign70710_e107115_d_n11, assign70710_e107115_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign70710_e107113: f64 = (locals.var_t0 * locals.var_t0);
        (assign70710_e107113, ((locals.var_t0_dn0 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn0)), ((locals.var_t0_dn2 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn2)), ((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)), ((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)), ((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)), ((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)), ((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)), ((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)), ((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)), ((locals.var_t0_dn11 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn11)), ((locals.var_t0_dn14 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn14)),)
    } else {
        (locals.var_cnst1over, locals.var_cnst1over_dn0, locals.var_cnst1over_dn2, locals.var_cnst1over_dn4, locals.var_cnst1over_dn5, locals.var_cnst1over_dn6, locals.var_cnst1over_dn7, locals.var_cnst1over_dn8, locals.var_cnst1over_dn9, locals.var_cnst1over_dn10, locals.var_cnst1over_dn11, locals.var_cnst1over_dn14,)
    }
};
        locals.var_cnst1over = assign70710_e107115;
        locals.var_cnst1over_dn0 = assign70710_e107115_d_n0;
        locals.var_cnst1over_dn2 = assign70710_e107115_d_n2;
        locals.var_cnst1over_dn4 = assign70710_e107115_d_n4;
        locals.var_cnst1over_dn5 = assign70710_e107115_d_n5;
        locals.var_cnst1over_dn6 = assign70710_e107115_d_n6;
        locals.var_cnst1over_dn7 = assign70710_e107115_d_n7;
        locals.var_cnst1over_dn8 = assign70710_e107115_d_n8;
        locals.var_cnst1over_dn9 = assign70710_e107115_d_n9;
        locals.var_cnst1over_dn10 = assign70710_e107115_d_n10;
        locals.var_cnst1over_dn11 = assign70710_e107115_d_n11;
        locals.var_cnst1over_dn14 = assign70710_e107115_d_n14;

        let assign70720_e107118: f64 = (-locals.var_vxbgmtcl);
        let assign70720_e107119: f64 = (locals.var_beta * assign70720_e107118);
        let assign70720_e107121: f64 = if assign70720_e107119 >= 500.0 { 1.0 } else { 0.0 };
        locals.var_guard1662 = assign70720_e107121;

        let (assign70730_e107136, assign70730_e107136_d_n0, assign70730_e107136_d_n2, assign70730_e107136_d_n4, assign70730_e107136_d_n5, assign70730_e107136_d_n6, assign70730_e107136_d_n7, assign70730_e107136_d_n8, assign70730_e107136_d_n9, assign70730_e107136_d_n10, assign70730_e107136_d_n11, assign70730_e107136_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1662 != 0.0)) {
        let assign70730_e107129: f64 = (-locals.var_vxbgmtcl);
        let assign70730_e107130: f64 = (locals.var_beta * assign70730_e107129);
        let assign70730_e107131: f64 = (1.0 + assign70730_e107130);
        let assign70730_e107133: f64 = (assign70730_e107131 - 500.0);
        let assign70730_e107134: f64 = (1.403592217853e217 * assign70730_e107133);
        (assign70730_e107134, (1.403592217853e217 * ((locals.var_beta_dn0 * assign70730_e107129) + (locals.var_beta * (-locals.var_vxbgmtcl_dn0)))), (1.403592217853e217 * ((locals.var_beta_dn2 * assign70730_e107129) + (locals.var_beta * (-locals.var_vxbgmtcl_dn2)))), (1.403592217853e217 * ((locals.var_beta_dn4 * assign70730_e107129) + (locals.var_beta * (-locals.var_vxbgmtcl_dn4)))), (1.403592217853e217 * ((locals.var_beta_dn5 * assign70730_e107129) + (locals.var_beta * (-locals.var_vxbgmtcl_dn5)))), (1.403592217853e217 * ((locals.var_beta_dn6 * assign70730_e107129) + (locals.var_beta * (-locals.var_vxbgmtcl_dn6)))), (1.403592217853e217 * ((locals.var_beta_dn7 * assign70730_e107129) + (locals.var_beta * (-locals.var_vxbgmtcl_dn7)))), (1.403592217853e217 * ((locals.var_beta_dn8 * assign70730_e107129) + (locals.var_beta * (-locals.var_vxbgmtcl_dn8)))), (1.403592217853e217 * ((locals.var_beta_dn9 * assign70730_e107129) + (locals.var_beta * (-locals.var_vxbgmtcl_dn9)))), (1.403592217853e217 * ((locals.var_beta_dn10 * assign70730_e107129) + (locals.var_beta * (-locals.var_vxbgmtcl_dn10)))), (1.403592217853e217 * ((locals.var_beta_dn11 * assign70730_e107129) + (locals.var_beta * (-locals.var_vxbgmtcl_dn11)))), (1.403592217853e217 * ((locals.var_beta_dn14 * assign70730_e107129) + (locals.var_beta * (-locals.var_vxbgmtcl_dn14)))),)
    } else {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn11, locals.var_exp_bvbs_dn14,)
    }
};
        locals.var_exp_bvbs = assign70730_e107136;
        locals.var_exp_bvbs_dn0 = assign70730_e107136_d_n0;
        locals.var_exp_bvbs_dn2 = assign70730_e107136_d_n2;
        locals.var_exp_bvbs_dn4 = assign70730_e107136_d_n4;
        locals.var_exp_bvbs_dn5 = assign70730_e107136_d_n5;
        locals.var_exp_bvbs_dn6 = assign70730_e107136_d_n6;
        locals.var_exp_bvbs_dn7 = assign70730_e107136_d_n7;
        locals.var_exp_bvbs_dn8 = assign70730_e107136_d_n8;
        locals.var_exp_bvbs_dn9 = assign70730_e107136_d_n9;
        locals.var_exp_bvbs_dn10 = assign70730_e107136_d_n10;
        locals.var_exp_bvbs_dn11 = assign70730_e107136_d_n11;
        locals.var_exp_bvbs_dn14 = assign70730_e107136_d_n14;

        let (assign70740_e107142, assign70740_e107142_d_n0, assign70740_e107142_d_n2, assign70740_e107142_d_n4, assign70740_e107142_d_n5, assign70740_e107142_d_n6, assign70740_e107142_d_n7, assign70740_e107142_d_n8, assign70740_e107142_d_n9, assign70740_e107142_d_n10, assign70740_e107142_d_n11, assign70740_e107142_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1662 != 0.0)) {
        (1.403592217853e217, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign70740_e107142;
        locals.var_t0_dn0 = assign70740_e107142_d_n0;
        locals.var_t0_dn2 = assign70740_e107142_d_n2;
        locals.var_t0_dn4 = assign70740_e107142_d_n4;
        locals.var_t0_dn5 = assign70740_e107142_d_n5;
        locals.var_t0_dn6 = assign70740_e107142_d_n6;
        locals.var_t0_dn7 = assign70740_e107142_d_n7;
        locals.var_t0_dn8 = assign70740_e107142_d_n8;
        locals.var_t0_dn9 = assign70740_e107142_d_n9;
        locals.var_t0_dn10 = assign70740_e107142_d_n10;
        locals.var_t0_dn11 = assign70740_e107142_d_n11;
        locals.var_t0_dn14 = assign70740_e107142_d_n14;

        let (assign70750_e107152, assign70750_e107152_d_n0, assign70750_e107152_d_n2, assign70750_e107152_d_n4, assign70750_e107152_d_n5, assign70750_e107152_d_n6, assign70750_e107152_d_n7, assign70750_e107152_d_n8, assign70750_e107152_d_n9, assign70750_e107152_d_n10, assign70750_e107152_d_n11, assign70750_e107152_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1662 == 0.0)) {
        let assign70750_e107149: f64 = (-locals.var_vxbgmtcl);
        let assign70750_e107150: f64 = (locals.var_beta * assign70750_e107149);
        (assign70750_e107150, ((locals.var_beta_dn0 * assign70750_e107149) + (locals.var_beta * (-locals.var_vxbgmtcl_dn0))), ((locals.var_beta_dn2 * assign70750_e107149) + (locals.var_beta * (-locals.var_vxbgmtcl_dn2))), ((locals.var_beta_dn4 * assign70750_e107149) + (locals.var_beta * (-locals.var_vxbgmtcl_dn4))), ((locals.var_beta_dn5 * assign70750_e107149) + (locals.var_beta * (-locals.var_vxbgmtcl_dn5))), ((locals.var_beta_dn6 * assign70750_e107149) + (locals.var_beta * (-locals.var_vxbgmtcl_dn6))), ((locals.var_beta_dn7 * assign70750_e107149) + (locals.var_beta * (-locals.var_vxbgmtcl_dn7))), ((locals.var_beta_dn8 * assign70750_e107149) + (locals.var_beta * (-locals.var_vxbgmtcl_dn8))), ((locals.var_beta_dn9 * assign70750_e107149) + (locals.var_beta * (-locals.var_vxbgmtcl_dn9))), ((locals.var_beta_dn10 * assign70750_e107149) + (locals.var_beta * (-locals.var_vxbgmtcl_dn10))), ((locals.var_beta_dn11 * assign70750_e107149) + (locals.var_beta * (-locals.var_vxbgmtcl_dn11))), ((locals.var_beta_dn14 * assign70750_e107149) + (locals.var_beta * (-locals.var_vxbgmtcl_dn14))),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign70750_e107152;
        locals.var_tmf1_dn0 = assign70750_e107152_d_n0;
        locals.var_tmf1_dn2 = assign70750_e107152_d_n2;
        locals.var_tmf1_dn4 = assign70750_e107152_d_n4;
        locals.var_tmf1_dn5 = assign70750_e107152_d_n5;
        locals.var_tmf1_dn6 = assign70750_e107152_d_n6;
        locals.var_tmf1_dn7 = assign70750_e107152_d_n7;
        locals.var_tmf1_dn8 = assign70750_e107152_d_n8;
        locals.var_tmf1_dn9 = assign70750_e107152_d_n9;
        locals.var_tmf1_dn10 = assign70750_e107152_d_n10;
        locals.var_tmf1_dn11 = assign70750_e107152_d_n11;
        locals.var_tmf1_dn14 = assign70750_e107152_d_n14;

        let (assign70760_e107159, assign70760_e107159_d_n0, assign70760_e107159_d_n2, assign70760_e107159_d_n4, assign70760_e107159_d_n5, assign70760_e107159_d_n6, assign70760_e107159_d_n7, assign70760_e107159_d_n8, assign70760_e107159_d_n9, assign70760_e107159_d_n10, assign70760_e107159_d_n11, assign70760_e107159_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1662 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn11, locals.var_exp_bvbs_dn14,)
    }
};
        locals.var_exp_bvbs = assign70760_e107159;
        locals.var_exp_bvbs_dn0 = assign70760_e107159_d_n0;
        locals.var_exp_bvbs_dn2 = assign70760_e107159_d_n2;
        locals.var_exp_bvbs_dn4 = assign70760_e107159_d_n4;
        locals.var_exp_bvbs_dn5 = assign70760_e107159_d_n5;
        locals.var_exp_bvbs_dn6 = assign70760_e107159_d_n6;
        locals.var_exp_bvbs_dn7 = assign70760_e107159_d_n7;
        locals.var_exp_bvbs_dn8 = assign70760_e107159_d_n8;
        locals.var_exp_bvbs_dn9 = assign70760_e107159_d_n9;
        locals.var_exp_bvbs_dn10 = assign70760_e107159_d_n10;
        locals.var_exp_bvbs_dn11 = assign70760_e107159_d_n11;
        locals.var_exp_bvbs_dn14 = assign70760_e107159_d_n14;

        let mut assign70770_loop_guard: usize = 0;
        while {
            let assign70770_cond_e107167: f64 = if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1662 == 0.0)) && (locals.var_tmf1 >= 60.0)) { 1.0 } else { 0.0 };
            assign70770_cond_e107167 != 0.0
        } {
            assign70770_loop_guard += 1;
            assert!(assign70770_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign70770_body0_e107176, assign70770_body0_e107176_d_n0, assign70770_body0_e107176_d_n2, assign70770_body0_e107176_d_n4, assign70770_body0_e107176_d_n5, assign70770_body0_e107176_d_n6, assign70770_body0_e107176_d_n7, assign70770_body0_e107176_d_n8, assign70770_body0_e107176_d_n9, assign70770_body0_e107176_d_n10, assign70770_body0_e107176_d_n11, assign70770_body0_e107176_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1662 == 0.0)) {
        let assign70770_body0_e107174: f64 = (locals.var_exp_bvbs * 1.14200738981568e26);
        (assign70770_body0_e107174, (locals.var_exp_bvbs_dn0 * 1.14200738981568e26), (locals.var_exp_bvbs_dn2 * 1.14200738981568e26), (locals.var_exp_bvbs_dn4 * 1.14200738981568e26), (locals.var_exp_bvbs_dn5 * 1.14200738981568e26), (locals.var_exp_bvbs_dn6 * 1.14200738981568e26), (locals.var_exp_bvbs_dn7 * 1.14200738981568e26), (locals.var_exp_bvbs_dn8 * 1.14200738981568e26), (locals.var_exp_bvbs_dn9 * 1.14200738981568e26), (locals.var_exp_bvbs_dn10 * 1.14200738981568e26), (locals.var_exp_bvbs_dn11 * 1.14200738981568e26), (locals.var_exp_bvbs_dn14 * 1.14200738981568e26),)
    } else {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn11, locals.var_exp_bvbs_dn14,)
    }
};
            locals.var_exp_bvbs = assign70770_body0_e107176;
            locals.var_exp_bvbs_dn0 = assign70770_body0_e107176_d_n0;
            locals.var_exp_bvbs_dn2 = assign70770_body0_e107176_d_n2;
            locals.var_exp_bvbs_dn4 = assign70770_body0_e107176_d_n4;
            locals.var_exp_bvbs_dn5 = assign70770_body0_e107176_d_n5;
            locals.var_exp_bvbs_dn6 = assign70770_body0_e107176_d_n6;
            locals.var_exp_bvbs_dn7 = assign70770_body0_e107176_d_n7;
            locals.var_exp_bvbs_dn8 = assign70770_body0_e107176_d_n8;
            locals.var_exp_bvbs_dn9 = assign70770_body0_e107176_d_n9;
            locals.var_exp_bvbs_dn10 = assign70770_body0_e107176_d_n10;
            locals.var_exp_bvbs_dn11 = assign70770_body0_e107176_d_n11;
            locals.var_exp_bvbs_dn14 = assign70770_body0_e107176_d_n14;
            let (assign70770_body1_e107185, assign70770_body1_e107185_d_n0, assign70770_body1_e107185_d_n2, assign70770_body1_e107185_d_n4, assign70770_body1_e107185_d_n5, assign70770_body1_e107185_d_n6, assign70770_body1_e107185_d_n7, assign70770_body1_e107185_d_n8, assign70770_body1_e107185_d_n9, assign70770_body1_e107185_d_n10, assign70770_body1_e107185_d_n11, assign70770_body1_e107185_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1662 == 0.0)) {
        let assign70770_body1_e107183: f64 = (locals.var_tmf1 - 60.0);
        (assign70770_body1_e107183, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
            locals.var_tmf1 = assign70770_body1_e107185;
            locals.var_tmf1_dn0 = assign70770_body1_e107185_d_n0;
            locals.var_tmf1_dn2 = assign70770_body1_e107185_d_n2;
            locals.var_tmf1_dn4 = assign70770_body1_e107185_d_n4;
            locals.var_tmf1_dn5 = assign70770_body1_e107185_d_n5;
            locals.var_tmf1_dn6 = assign70770_body1_e107185_d_n6;
            locals.var_tmf1_dn7 = assign70770_body1_e107185_d_n7;
            locals.var_tmf1_dn8 = assign70770_body1_e107185_d_n8;
            locals.var_tmf1_dn9 = assign70770_body1_e107185_d_n9;
            locals.var_tmf1_dn10 = assign70770_body1_e107185_d_n10;
            locals.var_tmf1_dn11 = assign70770_body1_e107185_d_n11;
            locals.var_tmf1_dn14 = assign70770_body1_e107185_d_n14;
        }

        let (assign70780_e107195, assign70780_e107195_d_n0, assign70780_e107195_d_n2, assign70780_e107195_d_n4, assign70780_e107195_d_n5, assign70780_e107195_d_n6, assign70780_e107195_d_n7, assign70780_e107195_d_n8, assign70780_e107195_d_n9, assign70780_e107195_d_n10, assign70780_e107195_d_n11, assign70780_e107195_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1662 == 0.0)) {
        let assign70780_e107192: f64 = (locals.var_tmf1).exp();
        let assign70780_e107193: f64 = (locals.var_exp_bvbs * assign70780_e107192);
        (assign70780_e107193, ((locals.var_exp_bvbs_dn0 * assign70780_e107192) + (locals.var_exp_bvbs * (assign70780_e107192 * locals.var_tmf1_dn0))), ((locals.var_exp_bvbs_dn2 * assign70780_e107192) + (locals.var_exp_bvbs * (assign70780_e107192 * locals.var_tmf1_dn2))), ((locals.var_exp_bvbs_dn4 * assign70780_e107192) + (locals.var_exp_bvbs * (assign70780_e107192 * locals.var_tmf1_dn4))), ((locals.var_exp_bvbs_dn5 * assign70780_e107192) + (locals.var_exp_bvbs * (assign70780_e107192 * locals.var_tmf1_dn5))), ((locals.var_exp_bvbs_dn6 * assign70780_e107192) + (locals.var_exp_bvbs * (assign70780_e107192 * locals.var_tmf1_dn6))), ((locals.var_exp_bvbs_dn7 * assign70780_e107192) + (locals.var_exp_bvbs * (assign70780_e107192 * locals.var_tmf1_dn7))), ((locals.var_exp_bvbs_dn8 * assign70780_e107192) + (locals.var_exp_bvbs * (assign70780_e107192 * locals.var_tmf1_dn8))), ((locals.var_exp_bvbs_dn9 * assign70780_e107192) + (locals.var_exp_bvbs * (assign70780_e107192 * locals.var_tmf1_dn9))), ((locals.var_exp_bvbs_dn10 * assign70780_e107192) + (locals.var_exp_bvbs * (assign70780_e107192 * locals.var_tmf1_dn10))), ((locals.var_exp_bvbs_dn11 * assign70780_e107192) + (locals.var_exp_bvbs * (assign70780_e107192 * locals.var_tmf1_dn11))), ((locals.var_exp_bvbs_dn14 * assign70780_e107192) + (locals.var_exp_bvbs * (assign70780_e107192 * locals.var_tmf1_dn14))),)
    } else {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn11, locals.var_exp_bvbs_dn14,)
    }
};
        locals.var_exp_bvbs = assign70780_e107195;
        locals.var_exp_bvbs_dn0 = assign70780_e107195_d_n0;
        locals.var_exp_bvbs_dn2 = assign70780_e107195_d_n2;
        locals.var_exp_bvbs_dn4 = assign70780_e107195_d_n4;
        locals.var_exp_bvbs_dn5 = assign70780_e107195_d_n5;
        locals.var_exp_bvbs_dn6 = assign70780_e107195_d_n6;
        locals.var_exp_bvbs_dn7 = assign70780_e107195_d_n7;
        locals.var_exp_bvbs_dn8 = assign70780_e107195_d_n8;
        locals.var_exp_bvbs_dn9 = assign70780_e107195_d_n9;
        locals.var_exp_bvbs_dn10 = assign70780_e107195_d_n10;
        locals.var_exp_bvbs_dn11 = assign70780_e107195_d_n11;
        locals.var_exp_bvbs_dn14 = assign70780_e107195_d_n14;

        let (assign70790_e107202, assign70790_e107202_d_n0, assign70790_e107202_d_n2, assign70790_e107202_d_n4, assign70790_e107202_d_n5, assign70790_e107202_d_n6, assign70790_e107202_d_n7, assign70790_e107202_d_n8, assign70790_e107202_d_n9, assign70790_e107202_d_n10, assign70790_e107202_d_n11, assign70790_e107202_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1662 == 0.0)) {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn11, locals.var_exp_bvbs_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign70790_e107202;
        locals.var_t0_dn0 = assign70790_e107202_d_n0;
        locals.var_t0_dn2 = assign70790_e107202_d_n2;
        locals.var_t0_dn4 = assign70790_e107202_d_n4;
        locals.var_t0_dn5 = assign70790_e107202_d_n5;
        locals.var_t0_dn6 = assign70790_e107202_d_n6;
        locals.var_t0_dn7 = assign70790_e107202_d_n7;
        locals.var_t0_dn8 = assign70790_e107202_d_n8;
        locals.var_t0_dn9 = assign70790_e107202_d_n9;
        locals.var_t0_dn10 = assign70790_e107202_d_n10;
        locals.var_t0_dn11 = assign70790_e107202_d_n11;
        locals.var_t0_dn14 = assign70790_e107202_d_n14;

        let (assign70800_e107215, assign70800_e107215_d_n0, assign70800_e107215_d_n2, assign70800_e107215_d_n4, assign70800_e107215_d_n5, assign70800_e107215_d_n6, assign70800_e107215_d_n7, assign70800_e107215_d_n8, assign70800_e107215_d_n9, assign70800_e107215_d_n10, assign70800_e107215_d_n11, assign70800_e107215_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign70800_e107207: f64 = (-locals.var_vgpld);
        let assign70800_e107209: f64 = (assign70800_e107207 * 0.5);
        let assign70800_e107211: f64 = (assign70800_e107209 - 0.5);
        let assign70800_e107213: f64 = (assign70800_e107211 - 1.0);
        (assign70800_e107213, 0.0, ((-locals.var_vgpld_dn2) * 0.5), 0.0, 0.0, 0.0, ((-locals.var_vgpld_dn7) * 0.5), ((-locals.var_vgpld_dn8) * 0.5), ((-locals.var_vgpld_dn9) * 0.5), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign70800_e107215;
        locals.var_tmf1_dn0 = assign70800_e107215_d_n0;
        locals.var_tmf1_dn2 = assign70800_e107215_d_n2;
        locals.var_tmf1_dn4 = assign70800_e107215_d_n4;
        locals.var_tmf1_dn5 = assign70800_e107215_d_n5;
        locals.var_tmf1_dn6 = assign70800_e107215_d_n6;
        locals.var_tmf1_dn7 = assign70800_e107215_d_n7;
        locals.var_tmf1_dn8 = assign70800_e107215_d_n8;
        locals.var_tmf1_dn9 = assign70800_e107215_d_n9;
        locals.var_tmf1_dn10 = assign70800_e107215_d_n10;
        locals.var_tmf1_dn11 = assign70800_e107215_d_n11;
        locals.var_tmf1_dn14 = assign70800_e107215_d_n14;

        let (assign70810_e107225, assign70810_e107225_d_n0, assign70810_e107225_d_n2, assign70810_e107225_d_n4, assign70810_e107225_d_n5, assign70810_e107225_d_n6, assign70810_e107225_d_n7, assign70810_e107225_d_n8, assign70810_e107225_d_n9, assign70810_e107225_d_n10, assign70810_e107225_d_n11, assign70810_e107225_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign70810_e107221: f64 = (4.0 * 0.5);
        let assign70810_e107223: f64 = assign70810_e107221;
        (assign70810_e107223, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign70810_e107225;
        locals.var_tmf2_dn0 = assign70810_e107225_d_n0;
        locals.var_tmf2_dn2 = assign70810_e107225_d_n2;
        locals.var_tmf2_dn4 = assign70810_e107225_d_n4;
        locals.var_tmf2_dn5 = assign70810_e107225_d_n5;
        locals.var_tmf2_dn6 = assign70810_e107225_d_n6;
        locals.var_tmf2_dn7 = assign70810_e107225_d_n7;
        locals.var_tmf2_dn8 = assign70810_e107225_d_n8;
        locals.var_tmf2_dn9 = assign70810_e107225_d_n9;
        locals.var_tmf2_dn10 = assign70810_e107225_d_n10;
        locals.var_tmf2_dn11 = assign70810_e107225_d_n11;
        locals.var_tmf2_dn14 = assign70810_e107225_d_n14;

        let (assign70820_e107237, assign70820_e107237_d_n0, assign70820_e107237_d_n2, assign70820_e107237_d_n4, assign70820_e107237_d_n5, assign70820_e107237_d_n6, assign70820_e107237_d_n7, assign70820_e107237_d_n8, assign70820_e107237_d_n9, assign70820_e107237_d_n10, assign70820_e107237_d_n11, assign70820_e107237_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let (assign70820_e107235, assign70820_e107235_d_n0, assign70820_e107235_d_n2, assign70820_e107235_d_n4, assign70820_e107235_d_n5, assign70820_e107235_d_n6, assign70820_e107235_d_n7, assign70820_e107235_d_n8, assign70820_e107235_d_n9, assign70820_e107235_d_n10, assign70820_e107235_d_n11, assign70820_e107235_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign70820_e107234: f64 = (-locals.var_tmf2);
                (assign70820_e107234, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign70820_e107235, assign70820_e107235_d_n0, assign70820_e107235_d_n2, assign70820_e107235_d_n4, assign70820_e107235_d_n5, assign70820_e107235_d_n6, assign70820_e107235_d_n7, assign70820_e107235_d_n8, assign70820_e107235_d_n9, assign70820_e107235_d_n10, assign70820_e107235_d_n11, assign70820_e107235_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign70820_e107237;
        locals.var_tmf2_dn0 = assign70820_e107237_d_n0;
        locals.var_tmf2_dn2 = assign70820_e107237_d_n2;
        locals.var_tmf2_dn4 = assign70820_e107237_d_n4;
        locals.var_tmf2_dn5 = assign70820_e107237_d_n5;
        locals.var_tmf2_dn6 = assign70820_e107237_d_n6;
        locals.var_tmf2_dn7 = assign70820_e107237_d_n7;
        locals.var_tmf2_dn8 = assign70820_e107237_d_n8;
        locals.var_tmf2_dn9 = assign70820_e107237_d_n9;
        locals.var_tmf2_dn10 = assign70820_e107237_d_n10;
        locals.var_tmf2_dn11 = assign70820_e107237_d_n11;
        locals.var_tmf2_dn14 = assign70820_e107237_d_n14;

        let (assign70830_e107248, assign70830_e107248_d_n0, assign70830_e107248_d_n2, assign70830_e107248_d_n4, assign70830_e107248_d_n5, assign70830_e107248_d_n6, assign70830_e107248_d_n7, assign70830_e107248_d_n8, assign70830_e107248_d_n9, assign70830_e107248_d_n10, assign70830_e107248_d_n11, assign70830_e107248_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign70830_e107243: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign70830_e107245: f64 = (assign70830_e107243 + locals.var_tmf2);
        let assign70830_e107246: f64 = (assign70830_e107245).sqrt();
        (assign70830_e107246, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign70830_e107246)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign70830_e107246)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign70830_e107246)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign70830_e107246)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign70830_e107246)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign70830_e107246)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign70830_e107246)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign70830_e107246)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign70830_e107246)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign70830_e107246)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign70830_e107246)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign70830_e107248;
        locals.var_tmf2_dn0 = assign70830_e107248_d_n0;
        locals.var_tmf2_dn2 = assign70830_e107248_d_n2;
        locals.var_tmf2_dn4 = assign70830_e107248_d_n4;
        locals.var_tmf2_dn5 = assign70830_e107248_d_n5;
        locals.var_tmf2_dn6 = assign70830_e107248_d_n6;
        locals.var_tmf2_dn7 = assign70830_e107248_d_n7;
        locals.var_tmf2_dn8 = assign70830_e107248_d_n8;
        locals.var_tmf2_dn9 = assign70830_e107248_d_n9;
        locals.var_tmf2_dn10 = assign70830_e107248_d_n10;
        locals.var_tmf2_dn11 = assign70830_e107248_d_n11;
        locals.var_tmf2_dn14 = assign70830_e107248_d_n14;

        let (assign70840_e107260, assign70840_e107260_d_n0, assign70840_e107260_d_n2, assign70840_e107260_d_n4, assign70840_e107260_d_n5, assign70840_e107260_d_n6, assign70840_e107260_d_n7, assign70840_e107260_d_n8, assign70840_e107260_d_n9, assign70840_e107260_d_n10, assign70840_e107260_d_n11, assign70840_e107260_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign70840_e107256: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign70840_e107257: f64 = (1.0 + assign70840_e107256);
        let assign70840_e107258: f64 = (0.5 * assign70840_e107257);
        (assign70840_e107258, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign70840_e107260;
        locals.var_t0_dn0 = assign70840_e107260_d_n0;
        locals.var_t0_dn2 = assign70840_e107260_d_n2;
        locals.var_t0_dn4 = assign70840_e107260_d_n4;
        locals.var_t0_dn5 = assign70840_e107260_d_n5;
        locals.var_t0_dn6 = assign70840_e107260_d_n6;
        locals.var_t0_dn7 = assign70840_e107260_d_n7;
        locals.var_t0_dn8 = assign70840_e107260_d_n8;
        locals.var_t0_dn9 = assign70840_e107260_d_n9;
        locals.var_t0_dn10 = assign70840_e107260_d_n10;
        locals.var_t0_dn11 = assign70840_e107260_d_n11;
        locals.var_t0_dn14 = assign70840_e107260_d_n14;

        let (assign70850_e107272, assign70850_e107272_d_n0, assign70850_e107272_d_n2, assign70850_e107272_d_n4, assign70850_e107272_d_n5, assign70850_e107272_d_n6, assign70850_e107272_d_n7, assign70850_e107272_d_n8, assign70850_e107272_d_n9, assign70850_e107272_d_n10, assign70850_e107272_d_n11, assign70850_e107272_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign70850_e107268: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign70850_e107269: f64 = (0.5 * assign70850_e107268);
        let assign70850_e107270: f64 = (0.5 + assign70850_e107269);
        (assign70850_e107270, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign70850_e107272;
        locals.var_t1_dn0 = assign70850_e107272_d_n0;
        locals.var_t1_dn2 = assign70850_e107272_d_n2;
        locals.var_t1_dn4 = assign70850_e107272_d_n4;
        locals.var_t1_dn5 = assign70850_e107272_d_n5;
        locals.var_t1_dn6 = assign70850_e107272_d_n6;
        locals.var_t1_dn7 = assign70850_e107272_d_n7;
        locals.var_t1_dn8 = assign70850_e107272_d_n8;
        locals.var_t1_dn9 = assign70850_e107272_d_n9;
        locals.var_t1_dn10 = assign70850_e107272_d_n10;
        locals.var_t1_dn11 = assign70850_e107272_d_n11;
        locals.var_t1_dn14 = assign70850_e107272_d_n14;

        let assign70860_e107275: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign70860_e107278: f64 = (-locals.var_t1);
        let assign70860_e107283: f64 = if ((assign70860_e107275 > assign70860_e107278) && (locals.var_t1 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1663 = assign70860_e107283;

        let (assign70870_e107297, assign70870_e107297_d_n0, assign70870_e107297_d_n2, assign70870_e107297_d_n4, assign70870_e107297_d_n5, assign70870_e107297_d_n6, assign70870_e107297_d_n7, assign70870_e107297_d_n8, assign70870_e107297_d_n9, assign70870_e107297_d_n10, assign70870_e107297_d_n11, assign70870_e107297_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1663 != 0.0)) {
        let assign70870_e107291: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign70870_e107293: f64 = assign70870_e107291;
        let assign70870_e107295: f64 = (assign70870_e107293 + locals.var_t1);
        (assign70870_e107295, (locals.var_vxbgmtcl_dn0 + locals.var_t1_dn0), ((locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2) + locals.var_t1_dn2), (locals.var_vxbgmtcl_dn4 + locals.var_t1_dn4), (locals.var_vxbgmtcl_dn5 + locals.var_t1_dn5), (locals.var_vxbgmtcl_dn6 + locals.var_t1_dn6), ((locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7) + locals.var_t1_dn7), ((locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8) + locals.var_t1_dn8), ((locals.var_vgpld_dn9 + locals.var_vxbgmtcl_dn9) + locals.var_t1_dn9), (locals.var_vxbgmtcl_dn10 + locals.var_t1_dn10), (locals.var_vxbgmtcl_dn11 + locals.var_t1_dn11), (locals.var_vxbgmtcl_dn14 + locals.var_t1_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign70870_e107297;
        locals.var_tmf1_dn0 = assign70870_e107297_d_n0;
        locals.var_tmf1_dn2 = assign70870_e107297_d_n2;
        locals.var_tmf1_dn4 = assign70870_e107297_d_n4;
        locals.var_tmf1_dn5 = assign70870_e107297_d_n5;
        locals.var_tmf1_dn6 = assign70870_e107297_d_n6;
        locals.var_tmf1_dn7 = assign70870_e107297_d_n7;
        locals.var_tmf1_dn8 = assign70870_e107297_d_n8;
        locals.var_tmf1_dn9 = assign70870_e107297_d_n9;
        locals.var_tmf1_dn10 = assign70870_e107297_d_n10;
        locals.var_tmf1_dn11 = assign70870_e107297_d_n11;
        locals.var_tmf1_dn14 = assign70870_e107297_d_n14;

        let (assign70880_e107307, assign70880_e107307_d_n0, assign70880_e107307_d_n2, assign70880_e107307_d_n4, assign70880_e107307_d_n5, assign70880_e107307_d_n6, assign70880_e107307_d_n7, assign70880_e107307_d_n8, assign70880_e107307_d_n9, assign70880_e107307_d_n10, assign70880_e107307_d_n11, assign70880_e107307_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1663 != 0.0)) {
        let assign70880_e107305: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign70880_e107305, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign70880_e107307;
        locals.var_x2_dn0 = assign70880_e107307_d_n0;
        locals.var_x2_dn2 = assign70880_e107307_d_n2;
        locals.var_x2_dn4 = assign70880_e107307_d_n4;
        locals.var_x2_dn5 = assign70880_e107307_d_n5;
        locals.var_x2_dn6 = assign70880_e107307_d_n6;
        locals.var_x2_dn7 = assign70880_e107307_d_n7;
        locals.var_x2_dn8 = assign70880_e107307_d_n8;
        locals.var_x2_dn9 = assign70880_e107307_d_n9;
        locals.var_x2_dn10 = assign70880_e107307_d_n10;
        locals.var_x2_dn11 = assign70880_e107307_d_n11;
        locals.var_x2_dn14 = assign70880_e107307_d_n14;

        let (assign70890_e107317, assign70890_e107317_d_n0, assign70890_e107317_d_n2, assign70890_e107317_d_n4, assign70890_e107317_d_n5, assign70890_e107317_d_n6, assign70890_e107317_d_n7, assign70890_e107317_d_n8, assign70890_e107317_d_n9, assign70890_e107317_d_n10, assign70890_e107317_d_n11, assign70890_e107317_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1663 != 0.0)) {
        let assign70890_e107315: f64 = (locals.var_t1 * locals.var_t1);
        (assign70890_e107315, ((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)), ((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)), ((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)), ((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)), ((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)), ((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)), ((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)), ((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)), ((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)), ((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)), ((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14)),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign70890_e107317;
        locals.var_xmax2_dn0 = assign70890_e107317_d_n0;
        locals.var_xmax2_dn2 = assign70890_e107317_d_n2;
        locals.var_xmax2_dn4 = assign70890_e107317_d_n4;
        locals.var_xmax2_dn5 = assign70890_e107317_d_n5;
        locals.var_xmax2_dn6 = assign70890_e107317_d_n6;
        locals.var_xmax2_dn7 = assign70890_e107317_d_n7;
        locals.var_xmax2_dn8 = assign70890_e107317_d_n8;
        locals.var_xmax2_dn9 = assign70890_e107317_d_n9;
        locals.var_xmax2_dn10 = assign70890_e107317_d_n10;
        locals.var_xmax2_dn11 = assign70890_e107317_d_n11;
        locals.var_xmax2_dn14 = assign70890_e107317_d_n14;

        let (assign70900_e107325, assign70900_e107325_d_n0, assign70900_e107325_d_n2, assign70900_e107325_d_n4, assign70900_e107325_d_n5, assign70900_e107325_d_n6, assign70900_e107325_d_n7, assign70900_e107325_d_n8, assign70900_e107325_d_n9, assign70900_e107325_d_n10, assign70900_e107325_d_n11, assign70900_e107325_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1663 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign70900_e107325;
        locals.var_xp_dn0 = assign70900_e107325_d_n0;
        locals.var_xp_dn2 = assign70900_e107325_d_n2;
        locals.var_xp_dn4 = assign70900_e107325_d_n4;
        locals.var_xp_dn5 = assign70900_e107325_d_n5;
        locals.var_xp_dn6 = assign70900_e107325_d_n6;
        locals.var_xp_dn7 = assign70900_e107325_d_n7;
        locals.var_xp_dn8 = assign70900_e107325_d_n8;
        locals.var_xp_dn9 = assign70900_e107325_d_n9;
        locals.var_xp_dn10 = assign70900_e107325_d_n10;
        locals.var_xp_dn11 = assign70900_e107325_d_n11;
        locals.var_xp_dn14 = assign70900_e107325_d_n14;

    }

    pub(super) fn stamp_transient_block_253(
        locals: &mut StampLocals,
    ) {
        let (assign70910_e107333, assign70910_e107333_d_n0, assign70910_e107333_d_n2, assign70910_e107333_d_n4, assign70910_e107333_d_n5, assign70910_e107333_d_n6, assign70910_e107333_d_n7, assign70910_e107333_d_n8, assign70910_e107333_d_n9, assign70910_e107333_d_n10, assign70910_e107333_d_n11, assign70910_e107333_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1663 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign70910_e107333;
        locals.var_xmp_dn0 = assign70910_e107333_d_n0;
        locals.var_xmp_dn2 = assign70910_e107333_d_n2;
        locals.var_xmp_dn4 = assign70910_e107333_d_n4;
        locals.var_xmp_dn5 = assign70910_e107333_d_n5;
        locals.var_xmp_dn6 = assign70910_e107333_d_n6;
        locals.var_xmp_dn7 = assign70910_e107333_d_n7;
        locals.var_xmp_dn8 = assign70910_e107333_d_n8;
        locals.var_xmp_dn9 = assign70910_e107333_d_n9;
        locals.var_xmp_dn10 = assign70910_e107333_d_n10;
        locals.var_xmp_dn11 = assign70910_e107333_d_n11;
        locals.var_xmp_dn14 = assign70910_e107333_d_n14;

        let (assign70920_e107341,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1663 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign70920_e107341;

        let (assign70930_e107349,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1663 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign70930_e107349;

        let (assign70940_e107357, assign70940_e107357_d_n0, assign70940_e107357_d_n2, assign70940_e107357_d_n4, assign70940_e107357_d_n5, assign70940_e107357_d_n6, assign70940_e107357_d_n7, assign70940_e107357_d_n8, assign70940_e107357_d_n9, assign70940_e107357_d_n10, assign70940_e107357_d_n11, assign70940_e107357_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1663 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign70940_e107357;
        locals.var_arg_dn0 = assign70940_e107357_d_n0;
        locals.var_arg_dn2 = assign70940_e107357_d_n2;
        locals.var_arg_dn4 = assign70940_e107357_d_n4;
        locals.var_arg_dn5 = assign70940_e107357_d_n5;
        locals.var_arg_dn6 = assign70940_e107357_d_n6;
        locals.var_arg_dn7 = assign70940_e107357_d_n7;
        locals.var_arg_dn8 = assign70940_e107357_d_n8;
        locals.var_arg_dn9 = assign70940_e107357_d_n9;
        locals.var_arg_dn10 = assign70940_e107357_d_n10;
        locals.var_arg_dn11 = assign70940_e107357_d_n11;
        locals.var_arg_dn14 = assign70940_e107357_d_n14;

        let (assign70950_e107365, assign70950_e107365_d_n0, assign70950_e107365_d_n2, assign70950_e107365_d_n4, assign70950_e107365_d_n5, assign70950_e107365_d_n6, assign70950_e107365_d_n7, assign70950_e107365_d_n8, assign70950_e107365_d_n9, assign70950_e107365_d_n10, assign70950_e107365_d_n11, assign70950_e107365_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1663 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign70950_e107365;
        locals.var_dnm_dn0 = assign70950_e107365_d_n0;
        locals.var_dnm_dn2 = assign70950_e107365_d_n2;
        locals.var_dnm_dn4 = assign70950_e107365_d_n4;
        locals.var_dnm_dn5 = assign70950_e107365_d_n5;
        locals.var_dnm_dn6 = assign70950_e107365_d_n6;
        locals.var_dnm_dn7 = assign70950_e107365_d_n7;
        locals.var_dnm_dn8 = assign70950_e107365_d_n8;
        locals.var_dnm_dn9 = assign70950_e107365_d_n9;
        locals.var_dnm_dn10 = assign70950_e107365_d_n10;
        locals.var_dnm_dn11 = assign70950_e107365_d_n11;
        locals.var_dnm_dn14 = assign70950_e107365_d_n14;

        let (assign70960_e107375, assign70960_e107375_d_n0, assign70960_e107375_d_n2, assign70960_e107375_d_n4, assign70960_e107375_d_n5, assign70960_e107375_d_n6, assign70960_e107375_d_n7, assign70960_e107375_d_n8, assign70960_e107375_d_n9, assign70960_e107375_d_n10, assign70960_e107375_d_n11, assign70960_e107375_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1663 != 0.0)) {
        let assign70960_e107373: f64 = (locals.var_xp * locals.var_x2);
        (assign70960_e107373, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign70960_e107375;
        locals.var_xp_dn0 = assign70960_e107375_d_n0;
        locals.var_xp_dn2 = assign70960_e107375_d_n2;
        locals.var_xp_dn4 = assign70960_e107375_d_n4;
        locals.var_xp_dn5 = assign70960_e107375_d_n5;
        locals.var_xp_dn6 = assign70960_e107375_d_n6;
        locals.var_xp_dn7 = assign70960_e107375_d_n7;
        locals.var_xp_dn8 = assign70960_e107375_d_n8;
        locals.var_xp_dn9 = assign70960_e107375_d_n9;
        locals.var_xp_dn10 = assign70960_e107375_d_n10;
        locals.var_xp_dn11 = assign70960_e107375_d_n11;
        locals.var_xp_dn14 = assign70960_e107375_d_n14;

        let (assign70970_e107385, assign70970_e107385_d_n0, assign70970_e107385_d_n2, assign70970_e107385_d_n4, assign70970_e107385_d_n5, assign70970_e107385_d_n6, assign70970_e107385_d_n7, assign70970_e107385_d_n8, assign70970_e107385_d_n9, assign70970_e107385_d_n10, assign70970_e107385_d_n11, assign70970_e107385_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1663 != 0.0)) {
        let assign70970_e107383: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign70970_e107383, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign70970_e107385;
        locals.var_xmp_dn0 = assign70970_e107385_d_n0;
        locals.var_xmp_dn2 = assign70970_e107385_d_n2;
        locals.var_xmp_dn4 = assign70970_e107385_d_n4;
        locals.var_xmp_dn5 = assign70970_e107385_d_n5;
        locals.var_xmp_dn6 = assign70970_e107385_d_n6;
        locals.var_xmp_dn7 = assign70970_e107385_d_n7;
        locals.var_xmp_dn8 = assign70970_e107385_d_n8;
        locals.var_xmp_dn9 = assign70970_e107385_d_n9;
        locals.var_xmp_dn10 = assign70970_e107385_d_n10;
        locals.var_xmp_dn11 = assign70970_e107385_d_n11;
        locals.var_xmp_dn14 = assign70970_e107385_d_n14;

        let (assign70980_e107395, assign70980_e107395_d_n0, assign70980_e107395_d_n2, assign70980_e107395_d_n4, assign70980_e107395_d_n5, assign70980_e107395_d_n6, assign70980_e107395_d_n7, assign70980_e107395_d_n8, assign70980_e107395_d_n9, assign70980_e107395_d_n10, assign70980_e107395_d_n11, assign70980_e107395_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1663 != 0.0)) {
        let assign70980_e107393: f64 = (locals.var_xp + locals.var_xmp);
        (assign70980_e107393, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign70980_e107395;
        locals.var_arg_dn0 = assign70980_e107395_d_n0;
        locals.var_arg_dn2 = assign70980_e107395_d_n2;
        locals.var_arg_dn4 = assign70980_e107395_d_n4;
        locals.var_arg_dn5 = assign70980_e107395_d_n5;
        locals.var_arg_dn6 = assign70980_e107395_d_n6;
        locals.var_arg_dn7 = assign70980_e107395_d_n7;
        locals.var_arg_dn8 = assign70980_e107395_d_n8;
        locals.var_arg_dn9 = assign70980_e107395_d_n9;
        locals.var_arg_dn10 = assign70980_e107395_d_n10;
        locals.var_arg_dn11 = assign70980_e107395_d_n11;
        locals.var_arg_dn14 = assign70980_e107395_d_n14;

        let (assign70990_e107403, assign70990_e107403_d_n0, assign70990_e107403_d_n2, assign70990_e107403_d_n4, assign70990_e107403_d_n5, assign70990_e107403_d_n6, assign70990_e107403_d_n7, assign70990_e107403_d_n8, assign70990_e107403_d_n9, assign70990_e107403_d_n10, assign70990_e107403_d_n11, assign70990_e107403_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1663 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign70990_e107403;
        locals.var_dnm_dn0 = assign70990_e107403_d_n0;
        locals.var_dnm_dn2 = assign70990_e107403_d_n2;
        locals.var_dnm_dn4 = assign70990_e107403_d_n4;
        locals.var_dnm_dn5 = assign70990_e107403_d_n5;
        locals.var_dnm_dn6 = assign70990_e107403_d_n6;
        locals.var_dnm_dn7 = assign70990_e107403_d_n7;
        locals.var_dnm_dn8 = assign70990_e107403_d_n8;
        locals.var_dnm_dn9 = assign70990_e107403_d_n9;
        locals.var_dnm_dn10 = assign70990_e107403_d_n10;
        locals.var_dnm_dn11 = assign70990_e107403_d_n11;
        locals.var_dnm_dn14 = assign70990_e107403_d_n14;

        let assign71000_e107418: f64 = if ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1664 = assign71000_e107418;

        let assign71010_e107421: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1665 = assign71010_e107421;

        let (assign71020_e107433,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1663 != 0.0)) && (locals.var_guard1664 != 0.0)) && (locals.var_guard1665 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign71020_e107433;

        let assign71030_e107436: f64 = if 1.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1666 = assign71030_e107436;

        let (assign71040_e107451,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1663 != 0.0)) && (locals.var_guard1664 != 0.0)) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1666 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign71040_e107451;

        let assign71050_e107454: f64 = if 1.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1667 = assign71050_e107454;

        let (assign71060_e107472,) = {
    if (((((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1663 != 0.0)) && (locals.var_guard1664 != 0.0)) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1666 == 0.0)) && (locals.var_guard1667 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign71060_e107472;

        let assign71070_e107475: f64 = if 1.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1668 = assign71070_e107475;

        let (assign71080_e107496,) = {
    if ((((((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1663 != 0.0)) && (locals.var_guard1664 != 0.0)) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1666 == 0.0)) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1668 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign71080_e107496;

        let (assign71090_e107506,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1663 != 0.0)) && (locals.var_guard1664 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign71090_e107506;

        let mut assign71100_loop_guard: usize = 0;
        while {
            let assign71100_cond_e107517: f64 = if (((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1663 != 0.0)) && (locals.var_guard1664 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign71100_cond_e107517 != 0.0
        } {
            assign71100_loop_guard += 1;
            assert!(assign71100_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign71100_body0_e107528, assign71100_body0_e107528_d_n0, assign71100_body0_e107528_d_n2, assign71100_body0_e107528_d_n4, assign71100_body0_e107528_d_n5, assign71100_body0_e107528_d_n6, assign71100_body0_e107528_d_n7, assign71100_body0_e107528_d_n8, assign71100_body0_e107528_d_n9, assign71100_body0_e107528_d_n10, assign71100_body0_e107528_d_n11, assign71100_body0_e107528_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1663 != 0.0)) && (locals.var_guard1664 != 0.0)) {
        let assign71100_body0_e107526: f64 = (locals.var_dnm).sqrt();
        (assign71100_body0_e107526, (locals.var_dnm_dn0 / (2.0 * assign71100_body0_e107526)), (locals.var_dnm_dn2 / (2.0 * assign71100_body0_e107526)), (locals.var_dnm_dn4 / (2.0 * assign71100_body0_e107526)), (locals.var_dnm_dn5 / (2.0 * assign71100_body0_e107526)), (locals.var_dnm_dn6 / (2.0 * assign71100_body0_e107526)), (locals.var_dnm_dn7 / (2.0 * assign71100_body0_e107526)), (locals.var_dnm_dn8 / (2.0 * assign71100_body0_e107526)), (locals.var_dnm_dn9 / (2.0 * assign71100_body0_e107526)), (locals.var_dnm_dn10 / (2.0 * assign71100_body0_e107526)), (locals.var_dnm_dn11 / (2.0 * assign71100_body0_e107526)), (locals.var_dnm_dn14 / (2.0 * assign71100_body0_e107526)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign71100_body0_e107528;
            locals.var_dnm_dn0 = assign71100_body0_e107528_d_n0;
            locals.var_dnm_dn2 = assign71100_body0_e107528_d_n2;
            locals.var_dnm_dn4 = assign71100_body0_e107528_d_n4;
            locals.var_dnm_dn5 = assign71100_body0_e107528_d_n5;
            locals.var_dnm_dn6 = assign71100_body0_e107528_d_n6;
            locals.var_dnm_dn7 = assign71100_body0_e107528_d_n7;
            locals.var_dnm_dn8 = assign71100_body0_e107528_d_n8;
            locals.var_dnm_dn9 = assign71100_body0_e107528_d_n9;
            locals.var_dnm_dn10 = assign71100_body0_e107528_d_n10;
            locals.var_dnm_dn11 = assign71100_body0_e107528_d_n11;
            locals.var_dnm_dn14 = assign71100_body0_e107528_d_n14;
            let (assign71100_body1_e107540,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1663 != 0.0)) && (locals.var_guard1664 != 0.0)) {
        let assign71100_body1_e107538: f64 = (locals.var_m0 + 1.0);
        (assign71100_body1_e107538,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign71100_body1_e107540;
        }

        let (assign71110_e107562, assign71110_e107562_d_n0, assign71110_e107562_d_n2, assign71110_e107562_d_n4, assign71110_e107562_d_n5, assign71110_e107562_d_n6, assign71110_e107562_d_n7, assign71110_e107562_d_n8, assign71110_e107562_d_n9, assign71110_e107562_d_n10, assign71110_e107562_d_n11, assign71110_e107562_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1663 != 0.0)) && (locals.var_guard1664 == 0.0)) {
        let (assign71110_e107560, assign71110_e107560_d_n0, assign71110_e107560_d_n2, assign71110_e107560_d_n4, assign71110_e107560_d_n5, assign71110_e107560_d_n6, assign71110_e107560_d_n7, assign71110_e107560_d_n8, assign71110_e107560_d_n9, assign71110_e107560_d_n10, assign71110_e107560_d_n11, assign71110_e107560_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign71110_e107557: f64 = 2.0;
                let assign71110_e107558: f64 = (1.0 / assign71110_e107557);
                let assign71110_e107559: f64 = (locals.var_dnm).powf(assign71110_e107558);
                (assign71110_e107559, if 0.0 == 0.0 && ((assign71110_e107558) as f64).is_finite() && ((assign71110_e107558) as f64).fract() == 0.0 { if assign71110_e107558 == 0.0 { 0.0 } else { (assign71110_e107558 * ((locals.var_dnm).powf(assign71110_e107558 - 1.0) * locals.var_dnm_dn0)) } } else { (assign71110_e107559 * (assign71110_e107558 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign71110_e107558) as f64).is_finite() && ((assign71110_e107558) as f64).fract() == 0.0 { if assign71110_e107558 == 0.0 { 0.0 } else { (assign71110_e107558 * ((locals.var_dnm).powf(assign71110_e107558 - 1.0) * locals.var_dnm_dn2)) } } else { (assign71110_e107559 * (assign71110_e107558 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign71110_e107558) as f64).is_finite() && ((assign71110_e107558) as f64).fract() == 0.0 { if assign71110_e107558 == 0.0 { 0.0 } else { (assign71110_e107558 * ((locals.var_dnm).powf(assign71110_e107558 - 1.0) * locals.var_dnm_dn4)) } } else { (assign71110_e107559 * (assign71110_e107558 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign71110_e107558) as f64).is_finite() && ((assign71110_e107558) as f64).fract() == 0.0 { if assign71110_e107558 == 0.0 { 0.0 } else { (assign71110_e107558 * ((locals.var_dnm).powf(assign71110_e107558 - 1.0) * locals.var_dnm_dn5)) } } else { (assign71110_e107559 * (assign71110_e107558 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign71110_e107558) as f64).is_finite() && ((assign71110_e107558) as f64).fract() == 0.0 { if assign71110_e107558 == 0.0 { 0.0 } else { (assign71110_e107558 * ((locals.var_dnm).powf(assign71110_e107558 - 1.0) * locals.var_dnm_dn6)) } } else { (assign71110_e107559 * (assign71110_e107558 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign71110_e107558) as f64).is_finite() && ((assign71110_e107558) as f64).fract() == 0.0 { if assign71110_e107558 == 0.0 { 0.0 } else { (assign71110_e107558 * ((locals.var_dnm).powf(assign71110_e107558 - 1.0) * locals.var_dnm_dn7)) } } else { (assign71110_e107559 * (assign71110_e107558 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign71110_e107558) as f64).is_finite() && ((assign71110_e107558) as f64).fract() == 0.0 { if assign71110_e107558 == 0.0 { 0.0 } else { (assign71110_e107558 * ((locals.var_dnm).powf(assign71110_e107558 - 1.0) * locals.var_dnm_dn8)) } } else { (assign71110_e107559 * (assign71110_e107558 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign71110_e107558) as f64).is_finite() && ((assign71110_e107558) as f64).fract() == 0.0 { if assign71110_e107558 == 0.0 { 0.0 } else { (assign71110_e107558 * ((locals.var_dnm).powf(assign71110_e107558 - 1.0) * locals.var_dnm_dn9)) } } else { (assign71110_e107559 * (assign71110_e107558 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign71110_e107558) as f64).is_finite() && ((assign71110_e107558) as f64).fract() == 0.0 { if assign71110_e107558 == 0.0 { 0.0 } else { (assign71110_e107558 * ((locals.var_dnm).powf(assign71110_e107558 - 1.0) * locals.var_dnm_dn10)) } } else { (assign71110_e107559 * (assign71110_e107558 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign71110_e107558) as f64).is_finite() && ((assign71110_e107558) as f64).fract() == 0.0 { if assign71110_e107558 == 0.0 { 0.0 } else { (assign71110_e107558 * ((locals.var_dnm).powf(assign71110_e107558 - 1.0) * locals.var_dnm_dn11)) } } else { (assign71110_e107559 * (assign71110_e107558 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign71110_e107558) as f64).is_finite() && ((assign71110_e107558) as f64).fract() == 0.0 { if assign71110_e107558 == 0.0 { 0.0 } else { (assign71110_e107558 * ((locals.var_dnm).powf(assign71110_e107558 - 1.0) * locals.var_dnm_dn14)) } } else { (assign71110_e107559 * (assign71110_e107558 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign71110_e107560, assign71110_e107560_d_n0, assign71110_e107560_d_n2, assign71110_e107560_d_n4, assign71110_e107560_d_n5, assign71110_e107560_d_n6, assign71110_e107560_d_n7, assign71110_e107560_d_n8, assign71110_e107560_d_n9, assign71110_e107560_d_n10, assign71110_e107560_d_n11, assign71110_e107560_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign71110_e107562;
        locals.var_dnm_dn0 = assign71110_e107562_d_n0;
        locals.var_dnm_dn2 = assign71110_e107562_d_n2;
        locals.var_dnm_dn4 = assign71110_e107562_d_n4;
        locals.var_dnm_dn5 = assign71110_e107562_d_n5;
        locals.var_dnm_dn6 = assign71110_e107562_d_n6;
        locals.var_dnm_dn7 = assign71110_e107562_d_n7;
        locals.var_dnm_dn8 = assign71110_e107562_d_n8;
        locals.var_dnm_dn9 = assign71110_e107562_d_n9;
        locals.var_dnm_dn10 = assign71110_e107562_d_n10;
        locals.var_dnm_dn11 = assign71110_e107562_d_n11;
        locals.var_dnm_dn14 = assign71110_e107562_d_n14;

        let (assign71120_e107572, assign71120_e107572_d_n0, assign71120_e107572_d_n2, assign71120_e107572_d_n4, assign71120_e107572_d_n5, assign71120_e107572_d_n6, assign71120_e107572_d_n7, assign71120_e107572_d_n8, assign71120_e107572_d_n9, assign71120_e107572_d_n10, assign71120_e107572_d_n11, assign71120_e107572_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1663 != 0.0)) {
        let assign71120_e107570: f64 = (1.0 / locals.var_dnm);
        (assign71120_e107570, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign71120_e107572;
        locals.var_dnm_dn0 = assign71120_e107572_d_n0;
        locals.var_dnm_dn2 = assign71120_e107572_d_n2;
        locals.var_dnm_dn4 = assign71120_e107572_d_n4;
        locals.var_dnm_dn5 = assign71120_e107572_d_n5;
        locals.var_dnm_dn6 = assign71120_e107572_d_n6;
        locals.var_dnm_dn7 = assign71120_e107572_d_n7;
        locals.var_dnm_dn8 = assign71120_e107572_d_n8;
        locals.var_dnm_dn9 = assign71120_e107572_d_n9;
        locals.var_dnm_dn10 = assign71120_e107572_d_n10;
        locals.var_dnm_dn11 = assign71120_e107572_d_n11;
        locals.var_dnm_dn14 = assign71120_e107572_d_n14;

        let (assign71130_e107584, assign71130_e107584_d_n0, assign71130_e107584_d_n2, assign71130_e107584_d_n4, assign71130_e107584_d_n5, assign71130_e107584_d_n6, assign71130_e107584_d_n7, assign71130_e107584_d_n8, assign71130_e107584_d_n9, assign71130_e107584_d_n10, assign71130_e107584_d_n11, assign71130_e107584_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1663 != 0.0)) {
        let assign71130_e107580: f64 = (locals.var_tmf1 * locals.var_t1);
        let assign71130_e107582: f64 = (assign71130_e107580 * locals.var_dnm);
        (assign71130_e107582, ((((locals.var_tmf1_dn0 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn0)) * locals.var_dnm) + (assign71130_e107580 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn2)) * locals.var_dnm) + (assign71130_e107580 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn4 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn4)) * locals.var_dnm) + (assign71130_e107580 * locals.var_dnm_dn4)), ((((locals.var_tmf1_dn5 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn5)) * locals.var_dnm) + (assign71130_e107580 * locals.var_dnm_dn5)), ((((locals.var_tmf1_dn6 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn6)) * locals.var_dnm) + (assign71130_e107580 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn7)) * locals.var_dnm) + (assign71130_e107580 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn8 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn8)) * locals.var_dnm) + (assign71130_e107580 * locals.var_dnm_dn8)), ((((locals.var_tmf1_dn9 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn9)) * locals.var_dnm) + (assign71130_e107580 * locals.var_dnm_dn9)), ((((locals.var_tmf1_dn10 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn10)) * locals.var_dnm) + (assign71130_e107580 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn11 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn11)) * locals.var_dnm) + (assign71130_e107580 * locals.var_dnm_dn11)), ((((locals.var_tmf1_dn14 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn14)) * locals.var_dnm) + (assign71130_e107580 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign71130_e107584;
        locals.var_tmf0_dn0 = assign71130_e107584_d_n0;
        locals.var_tmf0_dn2 = assign71130_e107584_d_n2;
        locals.var_tmf0_dn4 = assign71130_e107584_d_n4;
        locals.var_tmf0_dn5 = assign71130_e107584_d_n5;
        locals.var_tmf0_dn6 = assign71130_e107584_d_n6;
        locals.var_tmf0_dn7 = assign71130_e107584_d_n7;
        locals.var_tmf0_dn8 = assign71130_e107584_d_n8;
        locals.var_tmf0_dn9 = assign71130_e107584_d_n9;
        locals.var_tmf0_dn10 = assign71130_e107584_d_n10;
        locals.var_tmf0_dn11 = assign71130_e107584_d_n11;
        locals.var_tmf0_dn14 = assign71130_e107584_d_n14;

        let (assign71140_e107598, assign71140_e107598_d_n0, assign71140_e107598_d_n2, assign71140_e107598_d_n4, assign71140_e107598_d_n5, assign71140_e107598_d_n6, assign71140_e107598_d_n7, assign71140_e107598_d_n8, assign71140_e107598_d_n9, assign71140_e107598_d_n10, assign71140_e107598_d_n11, assign71140_e107598_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1663 != 0.0)) {
        let assign71140_e107592: f64 = (locals.var_t1 * locals.var_xmp);
        let assign71140_e107594: f64 = (assign71140_e107592 * locals.var_dnm);
        let assign71140_e107596: f64 = (assign71140_e107594 / locals.var_arg);
        (assign71140_e107596, (((((((locals.var_t1_dn0 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn0)) * locals.var_dnm) + (assign71140_e107592 * locals.var_dnm_dn0)) * locals.var_arg) - (assign71140_e107594 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn2 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn2)) * locals.var_dnm) + (assign71140_e107592 * locals.var_dnm_dn2)) * locals.var_arg) - (assign71140_e107594 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn4 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn4)) * locals.var_dnm) + (assign71140_e107592 * locals.var_dnm_dn4)) * locals.var_arg) - (assign71140_e107594 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn5 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn5)) * locals.var_dnm) + (assign71140_e107592 * locals.var_dnm_dn5)) * locals.var_arg) - (assign71140_e107594 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn6 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn6)) * locals.var_dnm) + (assign71140_e107592 * locals.var_dnm_dn6)) * locals.var_arg) - (assign71140_e107594 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn7 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn7)) * locals.var_dnm) + (assign71140_e107592 * locals.var_dnm_dn7)) * locals.var_arg) - (assign71140_e107594 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn8 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn8)) * locals.var_dnm) + (assign71140_e107592 * locals.var_dnm_dn8)) * locals.var_arg) - (assign71140_e107594 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn9 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn9)) * locals.var_dnm) + (assign71140_e107592 * locals.var_dnm_dn9)) * locals.var_arg) - (assign71140_e107594 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn10 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn10)) * locals.var_dnm) + (assign71140_e107592 * locals.var_dnm_dn10)) * locals.var_arg) - (assign71140_e107594 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn11 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn11)) * locals.var_dnm) + (assign71140_e107592 * locals.var_dnm_dn11)) * locals.var_arg) - (assign71140_e107594 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn14 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn14)) * locals.var_dnm) + (assign71140_e107592 * locals.var_dnm_dn14)) * locals.var_arg) - (assign71140_e107594 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign71140_e107598;
        locals.var_t0_dn0 = assign71140_e107598_d_n0;
        locals.var_t0_dn2 = assign71140_e107598_d_n2;
        locals.var_t0_dn4 = assign71140_e107598_d_n4;
        locals.var_t0_dn5 = assign71140_e107598_d_n5;
        locals.var_t0_dn6 = assign71140_e107598_d_n6;
        locals.var_t0_dn7 = assign71140_e107598_d_n7;
        locals.var_t0_dn8 = assign71140_e107598_d_n8;
        locals.var_t0_dn9 = assign71140_e107598_d_n9;
        locals.var_t0_dn10 = assign71140_e107598_d_n10;
        locals.var_t0_dn11 = assign71140_e107598_d_n11;
        locals.var_t0_dn14 = assign71140_e107598_d_n14;

        let (assign71150_e107610, assign71150_e107610_d_n0, assign71150_e107610_d_n2, assign71150_e107610_d_n4, assign71150_e107610_d_n5, assign71150_e107610_d_n6, assign71150_e107610_d_n7, assign71150_e107610_d_n8, assign71150_e107610_d_n9, assign71150_e107610_d_n10, assign71150_e107610_d_n11, assign71150_e107610_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1663 != 0.0)) {
        let assign71150_e107606: f64 = (-locals.var_t1);
        let assign71150_e107608: f64 = (assign71150_e107606 + locals.var_tmf0);
        (assign71150_e107608, ((-locals.var_t1_dn0) + locals.var_tmf0_dn0), ((-locals.var_t1_dn2) + locals.var_tmf0_dn2), ((-locals.var_t1_dn4) + locals.var_tmf0_dn4), ((-locals.var_t1_dn5) + locals.var_tmf0_dn5), ((-locals.var_t1_dn6) + locals.var_tmf0_dn6), ((-locals.var_t1_dn7) + locals.var_tmf0_dn7), ((-locals.var_t1_dn8) + locals.var_tmf0_dn8), ((-locals.var_t1_dn9) + locals.var_tmf0_dn9), ((-locals.var_t1_dn10) + locals.var_tmf0_dn10), ((-locals.var_t1_dn11) + locals.var_tmf0_dn11), ((-locals.var_t1_dn14) + locals.var_tmf0_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign71150_e107610;
        locals.var_t1_dn0 = assign71150_e107610_d_n0;
        locals.var_t1_dn2 = assign71150_e107610_d_n2;
        locals.var_t1_dn4 = assign71150_e107610_d_n4;
        locals.var_t1_dn5 = assign71150_e107610_d_n5;
        locals.var_t1_dn6 = assign71150_e107610_d_n6;
        locals.var_t1_dn7 = assign71150_e107610_d_n7;
        locals.var_t1_dn8 = assign71150_e107610_d_n8;
        locals.var_t1_dn9 = assign71150_e107610_d_n9;
        locals.var_t1_dn10 = assign71150_e107610_d_n10;
        locals.var_t1_dn11 = assign71150_e107610_d_n11;
        locals.var_t1_dn14 = assign71150_e107610_d_n14;

        let (assign71160_e107618, assign71160_e107618_d_n0, assign71160_e107618_d_n2, assign71160_e107618_d_n4, assign71160_e107618_d_n5, assign71160_e107618_d_n6, assign71160_e107618_d_n7, assign71160_e107618_d_n8, assign71160_e107618_d_n9, assign71160_e107618_d_n10, assign71160_e107618_d_n11, assign71160_e107618_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1663 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign71160_e107618;
        locals.var_t0_dn0 = assign71160_e107618_d_n0;
        locals.var_t0_dn2 = assign71160_e107618_d_n2;
        locals.var_t0_dn4 = assign71160_e107618_d_n4;
        locals.var_t0_dn5 = assign71160_e107618_d_n5;
        locals.var_t0_dn6 = assign71160_e107618_d_n6;
        locals.var_t0_dn7 = assign71160_e107618_d_n7;
        locals.var_t0_dn8 = assign71160_e107618_d_n8;
        locals.var_t0_dn9 = assign71160_e107618_d_n9;
        locals.var_t0_dn10 = assign71160_e107618_d_n10;
        locals.var_t0_dn11 = assign71160_e107618_d_n11;
        locals.var_t0_dn14 = assign71160_e107618_d_n14;

        let (assign71170_e107629, assign71170_e107629_d_n0, assign71170_e107629_d_n2, assign71170_e107629_d_n4, assign71170_e107629_d_n5, assign71170_e107629_d_n6, assign71170_e107629_d_n7, assign71170_e107629_d_n8, assign71170_e107629_d_n9, assign71170_e107629_d_n10, assign71170_e107629_d_n11, assign71170_e107629_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1663 == 0.0)) {
        let assign71170_e107627: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        (assign71170_e107627, locals.var_vxbgmtcl_dn0, (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2), locals.var_vxbgmtcl_dn4, locals.var_vxbgmtcl_dn5, locals.var_vxbgmtcl_dn6, (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7), (locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8), (locals.var_vgpld_dn9 + locals.var_vxbgmtcl_dn9), locals.var_vxbgmtcl_dn10, locals.var_vxbgmtcl_dn11, locals.var_vxbgmtcl_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign71170_e107629;
        locals.var_t1_dn0 = assign71170_e107629_d_n0;
        locals.var_t1_dn2 = assign71170_e107629_d_n2;
        locals.var_t1_dn4 = assign71170_e107629_d_n4;
        locals.var_t1_dn5 = assign71170_e107629_d_n5;
        locals.var_t1_dn6 = assign71170_e107629_d_n6;
        locals.var_t1_dn7 = assign71170_e107629_d_n7;
        locals.var_t1_dn8 = assign71170_e107629_d_n8;
        locals.var_t1_dn9 = assign71170_e107629_d_n9;
        locals.var_t1_dn10 = assign71170_e107629_d_n10;
        locals.var_t1_dn11 = assign71170_e107629_d_n11;
        locals.var_t1_dn14 = assign71170_e107629_d_n14;

        let (assign71180_e107638, assign71180_e107638_d_n0, assign71180_e107638_d_n2, assign71180_e107638_d_n4, assign71180_e107638_d_n5, assign71180_e107638_d_n6, assign71180_e107638_d_n7, assign71180_e107638_d_n8, assign71180_e107638_d_n9, assign71180_e107638_d_n10, assign71180_e107638_d_n11, assign71180_e107638_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1663 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign71180_e107638;
        locals.var_t0_dn0 = assign71180_e107638_d_n0;
        locals.var_t0_dn2 = assign71180_e107638_d_n2;
        locals.var_t0_dn4 = assign71180_e107638_d_n4;
        locals.var_t0_dn5 = assign71180_e107638_d_n5;
        locals.var_t0_dn6 = assign71180_e107638_d_n6;
        locals.var_t0_dn7 = assign71180_e107638_d_n7;
        locals.var_t0_dn8 = assign71180_e107638_d_n8;
        locals.var_t0_dn9 = assign71180_e107638_d_n9;
        locals.var_t0_dn10 = assign71180_e107638_d_n10;
        locals.var_t0_dn11 = assign71180_e107638_d_n11;
        locals.var_t0_dn14 = assign71180_e107638_d_n14;

        let (assign71190_e107646, assign71190_e107646_d_n0, assign71190_e107646_d_n2, assign71190_e107646_d_n4, assign71190_e107646_d_n5, assign71190_e107646_d_n6, assign71190_e107646_d_n7, assign71190_e107646_d_n8, assign71190_e107646_d_n9, assign71190_e107646_d_n10, assign71190_e107646_d_n11, assign71190_e107646_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign71190_e107644: f64 = (locals.var_t1 - locals.var_vgpld);
        (assign71190_e107644, locals.var_t1_dn0, (locals.var_t1_dn2 - locals.var_vgpld_dn2), locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, (locals.var_t1_dn7 - locals.var_vgpld_dn7), (locals.var_t1_dn8 - locals.var_vgpld_dn8), (locals.var_t1_dn9 - locals.var_vgpld_dn9), locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    } else {
        (locals.var_vxbgmtcl, locals.var_vxbgmtcl_dn0, locals.var_vxbgmtcl_dn2, locals.var_vxbgmtcl_dn4, locals.var_vxbgmtcl_dn5, locals.var_vxbgmtcl_dn6, locals.var_vxbgmtcl_dn7, locals.var_vxbgmtcl_dn8, locals.var_vxbgmtcl_dn9, locals.var_vxbgmtcl_dn10, locals.var_vxbgmtcl_dn11, locals.var_vxbgmtcl_dn14,)
    }
};
        locals.var_vxbgmtcl = assign71190_e107646;
        locals.var_vxbgmtcl_dn0 = assign71190_e107646_d_n0;
        locals.var_vxbgmtcl_dn2 = assign71190_e107646_d_n2;
        locals.var_vxbgmtcl_dn4 = assign71190_e107646_d_n4;
        locals.var_vxbgmtcl_dn5 = assign71190_e107646_d_n5;
        locals.var_vxbgmtcl_dn6 = assign71190_e107646_d_n6;
        locals.var_vxbgmtcl_dn7 = assign71190_e107646_d_n7;
        locals.var_vxbgmtcl_dn8 = assign71190_e107646_d_n8;
        locals.var_vxbgmtcl_dn9 = assign71190_e107646_d_n9;
        locals.var_vxbgmtcl_dn10 = assign71190_e107646_d_n10;
        locals.var_vxbgmtcl_dn11 = assign71190_e107646_d_n11;
        locals.var_vxbgmtcl_dn14 = assign71190_e107646_d_n14;

        let (assign71200_e107657,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign71200_e107651: f64 = (-locals.var_vxbgmtcl);
        let assign71200_e107654: f64 = (10.0 * 2.220446049250313e-16);
        let assign71200_e107655: f64 = (assign71200_e107651 + assign71200_e107654);
        (assign71200_e107655,)
    } else {
        (locals.var_vgb_fb_ld,)
    }
};
        locals.var_vgb_fb_ld = assign71200_e107657;

        let assign71210_e107660: f64 = if locals.var_vgpld < locals.var_vgb_fb_ld { 1.0 } else { 0.0 };
        locals.var_guard1669 = assign71210_e107660;

        let (assign71230_e107681, assign71230_e107681_d_n0, assign71230_e107681_d_n2, assign71230_e107681_d_n4, assign71230_e107681_d_n5, assign71230_e107681_d_n6, assign71230_e107681_d_n7, assign71230_e107681_d_n8, assign71230_e107681_d_n9, assign71230_e107681_d_n10, assign71230_e107681_d_n11, assign71230_e107681_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1669 != 0.0)) {
        let assign71230_e107673: f64 = (2.0 * locals.var_beta_inv);
        let assign71230_e107675: f64 = (-locals.var_vgs_min);
        let assign71230_e107677: f64 = (assign71230_e107675 / locals.var_fac1);
        let assign71230_e107678: f64 = (assign71230_e107677).ln();
        let assign71230_e107679: f64 = (assign71230_e107673 * assign71230_e107678);
        (assign71230_e107679, (((2.0 * locals.var_beta_inv_dn0) * assign71230_e107678) + (assign71230_e107673 * ((-((assign71230_e107675 * locals.var_fac1_dn0) / (locals.var_fac1 * locals.var_fac1))) / assign71230_e107677))), (((2.0 * locals.var_beta_inv_dn2) * assign71230_e107678) + (assign71230_e107673 * ((-((assign71230_e107675 * locals.var_fac1_dn2) / (locals.var_fac1 * locals.var_fac1))) / assign71230_e107677))), (((2.0 * locals.var_beta_inv_dn4) * assign71230_e107678) + (assign71230_e107673 * ((-((assign71230_e107675 * locals.var_fac1_dn4) / (locals.var_fac1 * locals.var_fac1))) / assign71230_e107677))), (((2.0 * locals.var_beta_inv_dn5) * assign71230_e107678) + (assign71230_e107673 * ((-((assign71230_e107675 * locals.var_fac1_dn5) / (locals.var_fac1 * locals.var_fac1))) / assign71230_e107677))), (((2.0 * locals.var_beta_inv_dn6) * assign71230_e107678) + (assign71230_e107673 * ((-((assign71230_e107675 * locals.var_fac1_dn6) / (locals.var_fac1 * locals.var_fac1))) / assign71230_e107677))), (((2.0 * locals.var_beta_inv_dn7) * assign71230_e107678) + (assign71230_e107673 * ((-((assign71230_e107675 * locals.var_fac1_dn7) / (locals.var_fac1 * locals.var_fac1))) / assign71230_e107677))), (((2.0 * locals.var_beta_inv_dn8) * assign71230_e107678) + (assign71230_e107673 * ((-((assign71230_e107675 * locals.var_fac1_dn8) / (locals.var_fac1 * locals.var_fac1))) / assign71230_e107677))), (((2.0 * locals.var_beta_inv_dn9) * assign71230_e107678) + (assign71230_e107673 * ((-((assign71230_e107675 * locals.var_fac1_dn9) / (locals.var_fac1 * locals.var_fac1))) / assign71230_e107677))), (((2.0 * locals.var_beta_inv_dn10) * assign71230_e107678) + (assign71230_e107673 * ((-((assign71230_e107675 * locals.var_fac1_dn10) / (locals.var_fac1 * locals.var_fac1))) / assign71230_e107677))), (((2.0 * locals.var_beta_inv_dn11) * assign71230_e107678) + (assign71230_e107673 * ((-((assign71230_e107675 * locals.var_fac1_dn11) / (locals.var_fac1 * locals.var_fac1))) / assign71230_e107677))), (((2.0 * locals.var_beta_inv_dn14) * assign71230_e107678) + (assign71230_e107673 * ((-((assign71230_e107675 * locals.var_fac1_dn14) / (locals.var_fac1 * locals.var_fac1))) / assign71230_e107677))),)
    } else {
        (locals.var_ps0_min, locals.var_ps0_min_dn0, locals.var_ps0_min_dn2, locals.var_ps0_min_dn4, locals.var_ps0_min_dn5, locals.var_ps0_min_dn6, locals.var_ps0_min_dn7, locals.var_ps0_min_dn8, locals.var_ps0_min_dn9, locals.var_ps0_min_dn10, locals.var_ps0_min_dn11, locals.var_ps0_min_dn14,)
    }
};
        locals.var_ps0_min = assign71230_e107681;
        locals.var_ps0_min_dn0 = assign71230_e107681_d_n0;
        locals.var_ps0_min_dn2 = assign71230_e107681_d_n2;
        locals.var_ps0_min_dn4 = assign71230_e107681_d_n4;
        locals.var_ps0_min_dn5 = assign71230_e107681_d_n5;
        locals.var_ps0_min_dn6 = assign71230_e107681_d_n6;
        locals.var_ps0_min_dn7 = assign71230_e107681_d_n7;
        locals.var_ps0_min_dn8 = assign71230_e107681_d_n8;
        locals.var_ps0_min_dn9 = assign71230_e107681_d_n9;
        locals.var_ps0_min_dn10 = assign71230_e107681_d_n10;
        locals.var_ps0_min_dn11 = assign71230_e107681_d_n11;
        locals.var_ps0_min_dn14 = assign71230_e107681_d_n14;

    }

    pub(super) fn stamp_transient_block_254(
        locals: &mut StampLocals,
    ) {
        let (assign71240_e107691, assign71240_e107691_d_n0, assign71240_e107691_d_n2, assign71240_e107691_d_n4, assign71240_e107691_d_n5, assign71240_e107691_d_n6, assign71240_e107691_d_n7, assign71240_e107691_d_n8, assign71240_e107691_d_n9, assign71240_e107691_d_n10, assign71240_e107691_d_n11, assign71240_e107691_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1669 != 0.0)) {
        let assign71240_e107688: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign71240_e107689: f64 = (locals.var_beta * assign71240_e107688);
        (assign71240_e107689, ((locals.var_beta_dn0 * assign71240_e107688) + (locals.var_beta * locals.var_vxbgmtcl_dn0)), ((locals.var_beta_dn2 * assign71240_e107688) + (locals.var_beta * (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2))), ((locals.var_beta_dn4 * assign71240_e107688) + (locals.var_beta * locals.var_vxbgmtcl_dn4)), ((locals.var_beta_dn5 * assign71240_e107688) + (locals.var_beta * locals.var_vxbgmtcl_dn5)), ((locals.var_beta_dn6 * assign71240_e107688) + (locals.var_beta * locals.var_vxbgmtcl_dn6)), ((locals.var_beta_dn7 * assign71240_e107688) + (locals.var_beta * (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7))), ((locals.var_beta_dn8 * assign71240_e107688) + (locals.var_beta * (locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8))), ((locals.var_beta_dn9 * assign71240_e107688) + (locals.var_beta * (locals.var_vgpld_dn9 + locals.var_vxbgmtcl_dn9))), ((locals.var_beta_dn10 * assign71240_e107688) + (locals.var_beta * locals.var_vxbgmtcl_dn10)), ((locals.var_beta_dn11 * assign71240_e107688) + (locals.var_beta * locals.var_vxbgmtcl_dn11)), ((locals.var_beta_dn14 * assign71240_e107688) + (locals.var_beta * locals.var_vxbgmtcl_dn14)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign71240_e107691;
        locals.var_tx_dn0 = assign71240_e107691_d_n0;
        locals.var_tx_dn2 = assign71240_e107691_d_n2;
        locals.var_tx_dn4 = assign71240_e107691_d_n4;
        locals.var_tx_dn5 = assign71240_e107691_d_n5;
        locals.var_tx_dn6 = assign71240_e107691_d_n6;
        locals.var_tx_dn7 = assign71240_e107691_d_n7;
        locals.var_tx_dn8 = assign71240_e107691_d_n8;
        locals.var_tx_dn9 = assign71240_e107691_d_n9;
        locals.var_tx_dn10 = assign71240_e107691_d_n10;
        locals.var_tx_dn11 = assign71240_e107691_d_n11;
        locals.var_tx_dn14 = assign71240_e107691_d_n14;

        let (assign71250_e107701, assign71250_e107701_d_n0, assign71250_e107701_d_n2, assign71250_e107701_d_n4, assign71250_e107701_d_n5, assign71250_e107701_d_n6, assign71250_e107701_d_n7, assign71250_e107701_d_n8, assign71250_e107701_d_n9, assign71250_e107701_d_n10, assign71250_e107701_d_n11, assign71250_e107701_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1669 != 0.0)) {
        let assign71250_e107698: f64 = (locals.var_beta * locals.var_cnst0over_func);
        let assign71250_e107699: f64 = (1.0 / assign71250_e107698);
        (assign71250_e107699, (-(((locals.var_beta_dn0 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn0)) / (assign71250_e107698 * assign71250_e107698))), (-(((locals.var_beta_dn2 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn2)) / (assign71250_e107698 * assign71250_e107698))), (-(((locals.var_beta_dn4 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn4)) / (assign71250_e107698 * assign71250_e107698))), (-(((locals.var_beta_dn5 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn5)) / (assign71250_e107698 * assign71250_e107698))), (-(((locals.var_beta_dn6 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn6)) / (assign71250_e107698 * assign71250_e107698))), (-(((locals.var_beta_dn7 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn7)) / (assign71250_e107698 * assign71250_e107698))), (-(((locals.var_beta_dn8 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn8)) / (assign71250_e107698 * assign71250_e107698))), (-(((locals.var_beta_dn9 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn9)) / (assign71250_e107698 * assign71250_e107698))), (-(((locals.var_beta_dn10 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn10)) / (assign71250_e107698 * assign71250_e107698))), (-(((locals.var_beta_dn11 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn11)) / (assign71250_e107698 * assign71250_e107698))), (-(((locals.var_beta_dn14 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn14)) / (assign71250_e107698 * assign71250_e107698))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign71250_e107701;
        locals.var_t1_dn0 = assign71250_e107701_d_n0;
        locals.var_t1_dn2 = assign71250_e107701_d_n2;
        locals.var_t1_dn4 = assign71250_e107701_d_n4;
        locals.var_t1_dn5 = assign71250_e107701_d_n5;
        locals.var_t1_dn6 = assign71250_e107701_d_n6;
        locals.var_t1_dn7 = assign71250_e107701_d_n7;
        locals.var_t1_dn8 = assign71250_e107701_d_n8;
        locals.var_t1_dn9 = assign71250_e107701_d_n9;
        locals.var_t1_dn10 = assign71250_e107701_d_n10;
        locals.var_t1_dn11 = assign71250_e107701_d_n11;
        locals.var_t1_dn14 = assign71250_e107701_d_n14;

        let (assign71260_e107709, assign71260_e107709_d_n0, assign71260_e107709_d_n2, assign71260_e107709_d_n4, assign71260_e107709_d_n5, assign71260_e107709_d_n6, assign71260_e107709_d_n7, assign71260_e107709_d_n8, assign71260_e107709_d_n9, assign71260_e107709_d_n10, assign71260_e107709_d_n11, assign71260_e107709_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1669 != 0.0)) {
        let assign71260_e107707: f64 = (locals.var_t1 * locals.var_cox0_func);
        (assign71260_e107707, (locals.var_t1_dn0 * locals.var_cox0_func), (locals.var_t1_dn2 * locals.var_cox0_func), (locals.var_t1_dn4 * locals.var_cox0_func), (locals.var_t1_dn5 * locals.var_cox0_func), (locals.var_t1_dn6 * locals.var_cox0_func), (locals.var_t1_dn7 * locals.var_cox0_func), (locals.var_t1_dn8 * locals.var_cox0_func), (locals.var_t1_dn9 * locals.var_cox0_func), (locals.var_t1_dn10 * locals.var_cox0_func), (locals.var_t1_dn11 * locals.var_cox0_func), (locals.var_t1_dn14 * locals.var_cox0_func),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn14,)
    }
};
        locals.var_ty = assign71260_e107709;
        locals.var_ty_dn0 = assign71260_e107709_d_n0;
        locals.var_ty_dn2 = assign71260_e107709_d_n2;
        locals.var_ty_dn4 = assign71260_e107709_d_n4;
        locals.var_ty_dn5 = assign71260_e107709_d_n5;
        locals.var_ty_dn6 = assign71260_e107709_d_n6;
        locals.var_ty_dn7 = assign71260_e107709_d_n7;
        locals.var_ty_dn8 = assign71260_e107709_d_n8;
        locals.var_ty_dn9 = assign71260_e107709_d_n9;
        locals.var_ty_dn10 = assign71260_e107709_d_n10;
        locals.var_ty_dn11 = assign71260_e107709_d_n11;
        locals.var_ty_dn14 = assign71260_e107709_d_n14;

        let (assign71270_e107721, assign71270_e107721_d_n0, assign71270_e107721_d_n2, assign71270_e107721_d_n4, assign71270_e107721_d_n5, assign71270_e107721_d_n6, assign71270_e107721_d_n7, assign71270_e107721_d_n8, assign71270_e107721_d_n9, assign71270_e107721_d_n10, assign71270_e107721_d_n11, assign71270_e107721_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1669 != 0.0)) {
        let assign71270_e107716: f64 = (3.0 * 1.414213562373095);
        let assign71270_e107718: f64 = (assign71270_e107716 * locals.var_ty);
        let assign71270_e107719: f64 = (2.0 + assign71270_e107718);
        (assign71270_e107719, (assign71270_e107716 * locals.var_ty_dn0), (assign71270_e107716 * locals.var_ty_dn2), (assign71270_e107716 * locals.var_ty_dn4), (assign71270_e107716 * locals.var_ty_dn5), (assign71270_e107716 * locals.var_ty_dn6), (assign71270_e107716 * locals.var_ty_dn7), (assign71270_e107716 * locals.var_ty_dn8), (assign71270_e107716 * locals.var_ty_dn9), (assign71270_e107716 * locals.var_ty_dn10), (assign71270_e107716 * locals.var_ty_dn11), (assign71270_e107716 * locals.var_ty_dn14),)
    } else {
        (locals.var_ac41, locals.var_ac41_dn0, locals.var_ac41_dn2, locals.var_ac41_dn4, locals.var_ac41_dn5, locals.var_ac41_dn6, locals.var_ac41_dn7, locals.var_ac41_dn8, locals.var_ac41_dn9, locals.var_ac41_dn10, locals.var_ac41_dn11, locals.var_ac41_dn14,)
    }
};
        locals.var_ac41 = assign71270_e107721;
        locals.var_ac41_dn0 = assign71270_e107721_d_n0;
        locals.var_ac41_dn2 = assign71270_e107721_d_n2;
        locals.var_ac41_dn4 = assign71270_e107721_d_n4;
        locals.var_ac41_dn5 = assign71270_e107721_d_n5;
        locals.var_ac41_dn6 = assign71270_e107721_d_n6;
        locals.var_ac41_dn7 = assign71270_e107721_d_n7;
        locals.var_ac41_dn8 = assign71270_e107721_d_n8;
        locals.var_ac41_dn9 = assign71270_e107721_d_n9;
        locals.var_ac41_dn10 = assign71270_e107721_d_n10;
        locals.var_ac41_dn11 = assign71270_e107721_d_n11;
        locals.var_ac41_dn14 = assign71270_e107721_d_n14;

        let (assign71280_e107733, assign71280_e107733_d_n0, assign71280_e107733_d_n2, assign71280_e107733_d_n4, assign71280_e107733_d_n5, assign71280_e107733_d_n6, assign71280_e107733_d_n7, assign71280_e107733_d_n8, assign71280_e107733_d_n9, assign71280_e107733_d_n10, assign71280_e107733_d_n11, assign71280_e107733_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1669 != 0.0)) {
        let assign71280_e107727: f64 = (8.0 * locals.var_ac41);
        let assign71280_e107729: f64 = (assign71280_e107727 * locals.var_ac41);
        let assign71280_e107731: f64 = (assign71280_e107729 * locals.var_ac41);
        (assign71280_e107731, (((((8.0 * locals.var_ac41_dn0) * locals.var_ac41) + (assign71280_e107727 * locals.var_ac41_dn0)) * locals.var_ac41) + (assign71280_e107729 * locals.var_ac41_dn0)), (((((8.0 * locals.var_ac41_dn2) * locals.var_ac41) + (assign71280_e107727 * locals.var_ac41_dn2)) * locals.var_ac41) + (assign71280_e107729 * locals.var_ac41_dn2)), (((((8.0 * locals.var_ac41_dn4) * locals.var_ac41) + (assign71280_e107727 * locals.var_ac41_dn4)) * locals.var_ac41) + (assign71280_e107729 * locals.var_ac41_dn4)), (((((8.0 * locals.var_ac41_dn5) * locals.var_ac41) + (assign71280_e107727 * locals.var_ac41_dn5)) * locals.var_ac41) + (assign71280_e107729 * locals.var_ac41_dn5)), (((((8.0 * locals.var_ac41_dn6) * locals.var_ac41) + (assign71280_e107727 * locals.var_ac41_dn6)) * locals.var_ac41) + (assign71280_e107729 * locals.var_ac41_dn6)), (((((8.0 * locals.var_ac41_dn7) * locals.var_ac41) + (assign71280_e107727 * locals.var_ac41_dn7)) * locals.var_ac41) + (assign71280_e107729 * locals.var_ac41_dn7)), (((((8.0 * locals.var_ac41_dn8) * locals.var_ac41) + (assign71280_e107727 * locals.var_ac41_dn8)) * locals.var_ac41) + (assign71280_e107729 * locals.var_ac41_dn8)), (((((8.0 * locals.var_ac41_dn9) * locals.var_ac41) + (assign71280_e107727 * locals.var_ac41_dn9)) * locals.var_ac41) + (assign71280_e107729 * locals.var_ac41_dn9)), (((((8.0 * locals.var_ac41_dn10) * locals.var_ac41) + (assign71280_e107727 * locals.var_ac41_dn10)) * locals.var_ac41) + (assign71280_e107729 * locals.var_ac41_dn10)), (((((8.0 * locals.var_ac41_dn11) * locals.var_ac41) + (assign71280_e107727 * locals.var_ac41_dn11)) * locals.var_ac41) + (assign71280_e107729 * locals.var_ac41_dn11)), (((((8.0 * locals.var_ac41_dn14) * locals.var_ac41) + (assign71280_e107727 * locals.var_ac41_dn14)) * locals.var_ac41) + (assign71280_e107729 * locals.var_ac41_dn14)),)
    } else {
        (locals.var_ac4, locals.var_ac4_dn0, locals.var_ac4_dn2, locals.var_ac4_dn4, locals.var_ac4_dn5, locals.var_ac4_dn6, locals.var_ac4_dn7, locals.var_ac4_dn8, locals.var_ac4_dn9, locals.var_ac4_dn10, locals.var_ac4_dn11, locals.var_ac4_dn14,)
    }
};
        locals.var_ac4 = assign71280_e107733;
        locals.var_ac4_dn0 = assign71280_e107733_d_n0;
        locals.var_ac4_dn2 = assign71280_e107733_d_n2;
        locals.var_ac4_dn4 = assign71280_e107733_d_n4;
        locals.var_ac4_dn5 = assign71280_e107733_d_n5;
        locals.var_ac4_dn6 = assign71280_e107733_d_n6;
        locals.var_ac4_dn7 = assign71280_e107733_d_n7;
        locals.var_ac4_dn8 = assign71280_e107733_d_n8;
        locals.var_ac4_dn9 = assign71280_e107733_d_n9;
        locals.var_ac4_dn10 = assign71280_e107733_d_n10;
        locals.var_ac4_dn11 = assign71280_e107733_d_n11;
        locals.var_ac4_dn14 = assign71280_e107733_d_n14;

        let (assign71290_e107749, assign71290_e107749_d_n0, assign71290_e107749_d_n2, assign71290_e107749_d_n4, assign71290_e107749_d_n5, assign71290_e107749_d_n6, assign71290_e107749_d_n7, assign71290_e107749_d_n8, assign71290_e107749_d_n9, assign71290_e107749_d_n10, assign71290_e107749_d_n11, assign71290_e107749_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1669 != 0.0)) {
        let assign71290_e107739: f64 = (7.0 * 1.414213562373095);
        let assign71290_e107742: f64 = (9.0 * locals.var_ty);
        let assign71290_e107745: f64 = (locals.var_tx - 2.0);
        let assign71290_e107746: f64 = (assign71290_e107742 * assign71290_e107745);
        let assign71290_e107747: f64 = (assign71290_e107739 - assign71290_e107746);
        (assign71290_e107747, (-(((9.0 * locals.var_ty_dn0) * assign71290_e107745) + (assign71290_e107742 * locals.var_tx_dn0))), (-(((9.0 * locals.var_ty_dn2) * assign71290_e107745) + (assign71290_e107742 * locals.var_tx_dn2))), (-(((9.0 * locals.var_ty_dn4) * assign71290_e107745) + (assign71290_e107742 * locals.var_tx_dn4))), (-(((9.0 * locals.var_ty_dn5) * assign71290_e107745) + (assign71290_e107742 * locals.var_tx_dn5))), (-(((9.0 * locals.var_ty_dn6) * assign71290_e107745) + (assign71290_e107742 * locals.var_tx_dn6))), (-(((9.0 * locals.var_ty_dn7) * assign71290_e107745) + (assign71290_e107742 * locals.var_tx_dn7))), (-(((9.0 * locals.var_ty_dn8) * assign71290_e107745) + (assign71290_e107742 * locals.var_tx_dn8))), (-(((9.0 * locals.var_ty_dn9) * assign71290_e107745) + (assign71290_e107742 * locals.var_tx_dn9))), (-(((9.0 * locals.var_ty_dn10) * assign71290_e107745) + (assign71290_e107742 * locals.var_tx_dn10))), (-(((9.0 * locals.var_ty_dn11) * assign71290_e107745) + (assign71290_e107742 * locals.var_tx_dn11))), (-(((9.0 * locals.var_ty_dn14) * assign71290_e107745) + (assign71290_e107742 * locals.var_tx_dn14))),)
    } else {
        (locals.var_ac31, locals.var_ac31_dn0, locals.var_ac31_dn2, locals.var_ac31_dn4, locals.var_ac31_dn5, locals.var_ac31_dn6, locals.var_ac31_dn7, locals.var_ac31_dn8, locals.var_ac31_dn9, locals.var_ac31_dn10, locals.var_ac31_dn11, locals.var_ac31_dn14,)
    }
};
        locals.var_ac31 = assign71290_e107749;
        locals.var_ac31_dn0 = assign71290_e107749_d_n0;
        locals.var_ac31_dn2 = assign71290_e107749_d_n2;
        locals.var_ac31_dn4 = assign71290_e107749_d_n4;
        locals.var_ac31_dn5 = assign71290_e107749_d_n5;
        locals.var_ac31_dn6 = assign71290_e107749_d_n6;
        locals.var_ac31_dn7 = assign71290_e107749_d_n7;
        locals.var_ac31_dn8 = assign71290_e107749_d_n8;
        locals.var_ac31_dn9 = assign71290_e107749_d_n9;
        locals.var_ac31_dn10 = assign71290_e107749_d_n10;
        locals.var_ac31_dn11 = assign71290_e107749_d_n11;
        locals.var_ac31_dn14 = assign71290_e107749_d_n14;

        let (assign71300_e107757, assign71300_e107757_d_n0, assign71300_e107757_d_n2, assign71300_e107757_d_n4, assign71300_e107757_d_n5, assign71300_e107757_d_n6, assign71300_e107757_d_n7, assign71300_e107757_d_n8, assign71300_e107757_d_n9, assign71300_e107757_d_n10, assign71300_e107757_d_n11, assign71300_e107757_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1669 != 0.0)) {
        let assign71300_e107755: f64 = (locals.var_ac31 * locals.var_ac31);
        (assign71300_e107755, ((locals.var_ac31_dn0 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn0)), ((locals.var_ac31_dn2 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn2)), ((locals.var_ac31_dn4 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn4)), ((locals.var_ac31_dn5 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn5)), ((locals.var_ac31_dn6 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn6)), ((locals.var_ac31_dn7 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn7)), ((locals.var_ac31_dn8 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn8)), ((locals.var_ac31_dn9 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn9)), ((locals.var_ac31_dn10 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn10)), ((locals.var_ac31_dn11 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn11)), ((locals.var_ac31_dn14 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn14)),)
    } else {
        (locals.var_ac3, locals.var_ac3_dn0, locals.var_ac3_dn2, locals.var_ac3_dn4, locals.var_ac3_dn5, locals.var_ac3_dn6, locals.var_ac3_dn7, locals.var_ac3_dn8, locals.var_ac3_dn9, locals.var_ac3_dn10, locals.var_ac3_dn11, locals.var_ac3_dn14,)
    }
};
        locals.var_ac3 = assign71300_e107757;
        locals.var_ac3_dn0 = assign71300_e107757_d_n0;
        locals.var_ac3_dn2 = assign71300_e107757_d_n2;
        locals.var_ac3_dn4 = assign71300_e107757_d_n4;
        locals.var_ac3_dn5 = assign71300_e107757_d_n5;
        locals.var_ac3_dn6 = assign71300_e107757_d_n6;
        locals.var_ac3_dn7 = assign71300_e107757_d_n7;
        locals.var_ac3_dn8 = assign71300_e107757_d_n8;
        locals.var_ac3_dn9 = assign71300_e107757_d_n9;
        locals.var_ac3_dn10 = assign71300_e107757_d_n10;
        locals.var_ac3_dn11 = assign71300_e107757_d_n11;
        locals.var_ac3_dn14 = assign71300_e107757_d_n14;

        let assign71310_e107761: f64 = (locals.var_ac3 * 1e-8);
        let assign71310_e107762: f64 = if locals.var_ac4 < assign71310_e107761 { 1.0 } else { 0.0 };
        locals.var_guard1670 = assign71310_e107762;

        let (assign71330_e107783, assign71330_e107783_d_n0, assign71330_e107783_d_n2, assign71330_e107783_d_n4, assign71330_e107783_d_n5, assign71330_e107783_d_n6, assign71330_e107783_d_n7, assign71330_e107783_d_n8, assign71330_e107783_d_n9, assign71330_e107783_d_n10, assign71330_e107783_d_n11, assign71330_e107783_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1669 != 0.0)) && (locals.var_guard1670 != 0.0)) {
        let assign71330_e107779: f64 = (0.5 * locals.var_ac4);
        let assign71330_e107781: f64 = (assign71330_e107779 / locals.var_ac31);
        (assign71330_e107781, ((((0.5 * locals.var_ac4_dn0) * locals.var_ac31) - (assign71330_e107779 * locals.var_ac31_dn0)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn2) * locals.var_ac31) - (assign71330_e107779 * locals.var_ac31_dn2)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn4) * locals.var_ac31) - (assign71330_e107779 * locals.var_ac31_dn4)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn5) * locals.var_ac31) - (assign71330_e107779 * locals.var_ac31_dn5)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn6) * locals.var_ac31) - (assign71330_e107779 * locals.var_ac31_dn6)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn7) * locals.var_ac31) - (assign71330_e107779 * locals.var_ac31_dn7)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn8) * locals.var_ac31) - (assign71330_e107779 * locals.var_ac31_dn8)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn9) * locals.var_ac31) - (assign71330_e107779 * locals.var_ac31_dn9)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn10) * locals.var_ac31) - (assign71330_e107779 * locals.var_ac31_dn10)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn11) * locals.var_ac31) - (assign71330_e107779 * locals.var_ac31_dn11)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn14) * locals.var_ac31) - (assign71330_e107779 * locals.var_ac31_dn14)) / (locals.var_ac31 * locals.var_ac31)),)
    } else {
        (locals.var_ac1, locals.var_ac1_dn0, locals.var_ac1_dn2, locals.var_ac1_dn4, locals.var_ac1_dn5, locals.var_ac1_dn6, locals.var_ac1_dn7, locals.var_ac1_dn8, locals.var_ac1_dn9, locals.var_ac1_dn10, locals.var_ac1_dn11, locals.var_ac1_dn14,)
    }
};
        locals.var_ac1 = assign71330_e107783;
        locals.var_ac1_dn0 = assign71330_e107783_d_n0;
        locals.var_ac1_dn2 = assign71330_e107783_d_n2;
        locals.var_ac1_dn4 = assign71330_e107783_d_n4;
        locals.var_ac1_dn5 = assign71330_e107783_d_n5;
        locals.var_ac1_dn6 = assign71330_e107783_d_n6;
        locals.var_ac1_dn7 = assign71330_e107783_d_n7;
        locals.var_ac1_dn8 = assign71330_e107783_d_n8;
        locals.var_ac1_dn9 = assign71330_e107783_d_n9;
        locals.var_ac1_dn10 = assign71330_e107783_d_n10;
        locals.var_ac1_dn11 = assign71330_e107783_d_n11;
        locals.var_ac1_dn14 = assign71330_e107783_d_n14;

        let (assign71340_e107795, assign71340_e107795_d_n0, assign71340_e107795_d_n2, assign71340_e107795_d_n4, assign71340_e107795_d_n5, assign71340_e107795_d_n6, assign71340_e107795_d_n7, assign71340_e107795_d_n8, assign71340_e107795_d_n9, assign71340_e107795_d_n10, assign71340_e107795_d_n11, assign71340_e107795_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1669 != 0.0)) && (locals.var_guard1670 == 0.0)) {
        let assign71340_e107792: f64 = (locals.var_ac4 + locals.var_ac3);
        let assign71340_e107793: f64 = (assign71340_e107792).sqrt();
        (assign71340_e107793, ((locals.var_ac4_dn0 + locals.var_ac3_dn0) / (2.0 * assign71340_e107793)), ((locals.var_ac4_dn2 + locals.var_ac3_dn2) / (2.0 * assign71340_e107793)), ((locals.var_ac4_dn4 + locals.var_ac3_dn4) / (2.0 * assign71340_e107793)), ((locals.var_ac4_dn5 + locals.var_ac3_dn5) / (2.0 * assign71340_e107793)), ((locals.var_ac4_dn6 + locals.var_ac3_dn6) / (2.0 * assign71340_e107793)), ((locals.var_ac4_dn7 + locals.var_ac3_dn7) / (2.0 * assign71340_e107793)), ((locals.var_ac4_dn8 + locals.var_ac3_dn8) / (2.0 * assign71340_e107793)), ((locals.var_ac4_dn9 + locals.var_ac3_dn9) / (2.0 * assign71340_e107793)), ((locals.var_ac4_dn10 + locals.var_ac3_dn10) / (2.0 * assign71340_e107793)), ((locals.var_ac4_dn11 + locals.var_ac3_dn11) / (2.0 * assign71340_e107793)), ((locals.var_ac4_dn14 + locals.var_ac3_dn14) / (2.0 * assign71340_e107793)),)
    } else {
        (locals.var_ac2, locals.var_ac2_dn0, locals.var_ac2_dn2, locals.var_ac2_dn4, locals.var_ac2_dn5, locals.var_ac2_dn6, locals.var_ac2_dn7, locals.var_ac2_dn8, locals.var_ac2_dn9, locals.var_ac2_dn10, locals.var_ac2_dn11, locals.var_ac2_dn14,)
    }
};
        locals.var_ac2 = assign71340_e107795;
        locals.var_ac2_dn0 = assign71340_e107795_d_n0;
        locals.var_ac2_dn2 = assign71340_e107795_d_n2;
        locals.var_ac2_dn4 = assign71340_e107795_d_n4;
        locals.var_ac2_dn5 = assign71340_e107795_d_n5;
        locals.var_ac2_dn6 = assign71340_e107795_d_n6;
        locals.var_ac2_dn7 = assign71340_e107795_d_n7;
        locals.var_ac2_dn8 = assign71340_e107795_d_n8;
        locals.var_ac2_dn9 = assign71340_e107795_d_n9;
        locals.var_ac2_dn10 = assign71340_e107795_d_n10;
        locals.var_ac2_dn11 = assign71340_e107795_d_n11;
        locals.var_ac2_dn14 = assign71340_e107795_d_n14;

        let (assign71350_e107807, assign71350_e107807_d_n0, assign71350_e107807_d_n2, assign71350_e107807_d_n4, assign71350_e107807_d_n5, assign71350_e107807_d_n6, assign71350_e107807_d_n7, assign71350_e107807_d_n8, assign71350_e107807_d_n9, assign71350_e107807_d_n10, assign71350_e107807_d_n11, assign71350_e107807_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1669 != 0.0)) && (locals.var_guard1670 == 0.0)) {
        let assign71350_e107803: f64 = (-locals.var_ac31);
        let assign71350_e107805: f64 = (assign71350_e107803 + locals.var_ac2);
        (assign71350_e107805, ((-locals.var_ac31_dn0) + locals.var_ac2_dn0), ((-locals.var_ac31_dn2) + locals.var_ac2_dn2), ((-locals.var_ac31_dn4) + locals.var_ac2_dn4), ((-locals.var_ac31_dn5) + locals.var_ac2_dn5), ((-locals.var_ac31_dn6) + locals.var_ac2_dn6), ((-locals.var_ac31_dn7) + locals.var_ac2_dn7), ((-locals.var_ac31_dn8) + locals.var_ac2_dn8), ((-locals.var_ac31_dn9) + locals.var_ac2_dn9), ((-locals.var_ac31_dn10) + locals.var_ac2_dn10), ((-locals.var_ac31_dn11) + locals.var_ac2_dn11), ((-locals.var_ac31_dn14) + locals.var_ac2_dn14),)
    } else {
        (locals.var_ac1, locals.var_ac1_dn0, locals.var_ac1_dn2, locals.var_ac1_dn4, locals.var_ac1_dn5, locals.var_ac1_dn6, locals.var_ac1_dn7, locals.var_ac1_dn8, locals.var_ac1_dn9, locals.var_ac1_dn10, locals.var_ac1_dn11, locals.var_ac1_dn14,)
    }
};
        locals.var_ac1 = assign71350_e107807;
        locals.var_ac1_dn0 = assign71350_e107807_d_n0;
        locals.var_ac1_dn2 = assign71350_e107807_d_n2;
        locals.var_ac1_dn4 = assign71350_e107807_d_n4;
        locals.var_ac1_dn5 = assign71350_e107807_d_n5;
        locals.var_ac1_dn6 = assign71350_e107807_d_n6;
        locals.var_ac1_dn7 = assign71350_e107807_d_n7;
        locals.var_ac1_dn8 = assign71350_e107807_d_n8;
        locals.var_ac1_dn9 = assign71350_e107807_d_n9;
        locals.var_ac1_dn10 = assign71350_e107807_d_n10;
        locals.var_ac1_dn11 = assign71350_e107807_d_n11;
        locals.var_ac1_dn14 = assign71350_e107807_d_n14;

        let (assign71360_e107815, assign71360_e107815_d_n0, assign71360_e107815_d_n2, assign71360_e107815_d_n4, assign71360_e107815_d_n5, assign71360_e107815_d_n6, assign71360_e107815_d_n7, assign71360_e107815_d_n8, assign71360_e107815_d_n9, assign71360_e107815_d_n10, assign71360_e107815_d_n11, assign71360_e107815_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1669 != 0.0)) {
        let assign71360_e107813: f64 = (locals.var_ac1).powf(0.3333333333333333);
        (assign71360_e107813, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn0)) } } else { (assign71360_e107813 * (0.3333333333333333 * (locals.var_ac1_dn0 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn2)) } } else { (assign71360_e107813 * (0.3333333333333333 * (locals.var_ac1_dn2 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn4)) } } else { (assign71360_e107813 * (0.3333333333333333 * (locals.var_ac1_dn4 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn5)) } } else { (assign71360_e107813 * (0.3333333333333333 * (locals.var_ac1_dn5 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn6)) } } else { (assign71360_e107813 * (0.3333333333333333 * (locals.var_ac1_dn6 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn7)) } } else { (assign71360_e107813 * (0.3333333333333333 * (locals.var_ac1_dn7 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn8)) } } else { (assign71360_e107813 * (0.3333333333333333 * (locals.var_ac1_dn8 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn9)) } } else { (assign71360_e107813 * (0.3333333333333333 * (locals.var_ac1_dn9 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn10)) } } else { (assign71360_e107813 * (0.3333333333333333 * (locals.var_ac1_dn10 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn11)) } } else { (assign71360_e107813 * (0.3333333333333333 * (locals.var_ac1_dn11 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn14)) } } else { (assign71360_e107813 * (0.3333333333333333 * (locals.var_ac1_dn14 / locals.var_ac1))) },)
    } else {
        (locals.var_acd, locals.var_acd_dn0, locals.var_acd_dn2, locals.var_acd_dn4, locals.var_acd_dn5, locals.var_acd_dn6, locals.var_acd_dn7, locals.var_acd_dn8, locals.var_acd_dn9, locals.var_acd_dn10, locals.var_acd_dn11, locals.var_acd_dn14,)
    }
};
        locals.var_acd = assign71360_e107815;
        locals.var_acd_dn0 = assign71360_e107815_d_n0;
        locals.var_acd_dn2 = assign71360_e107815_d_n2;
        locals.var_acd_dn4 = assign71360_e107815_d_n4;
        locals.var_acd_dn5 = assign71360_e107815_d_n5;
        locals.var_acd_dn6 = assign71360_e107815_d_n6;
        locals.var_acd_dn7 = assign71360_e107815_d_n7;
        locals.var_acd_dn8 = assign71360_e107815_d_n8;
        locals.var_acd_dn9 = assign71360_e107815_d_n9;
        locals.var_acd_dn10 = assign71360_e107815_d_n10;
        locals.var_acd_dn11 = assign71360_e107815_d_n11;
        locals.var_acd_dn14 = assign71360_e107815_d_n14;

        let (assign71370_e107838, assign71370_e107838_d_n0, assign71370_e107838_d_n2, assign71370_e107838_d_n4, assign71370_e107838_d_n5, assign71370_e107838_d_n6, assign71370_e107838_d_n7, assign71370_e107838_d_n8, assign71370_e107838_d_n9, assign71370_e107838_d_n10, assign71370_e107838_d_n11, assign71370_e107838_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1669 != 0.0)) {
        let assign71370_e107820: f64 = (-4.0);
        let assign71370_e107822: f64 = (assign71370_e107820 * 1.414213562373095);
        let assign71370_e107825: f64 = (12.0 * locals.var_ty);
        let assign71370_e107826: f64 = (assign71370_e107822 - assign71370_e107825);
        let assign71370_e107829: f64 = (2.0 * locals.var_acd);
        let assign71370_e107830: f64 = (assign71370_e107826 + assign71370_e107829);
        let assign71370_e107833: f64 = (1.414213562373095 * locals.var_acd);
        let assign71370_e107835: f64 = (assign71370_e107833 * locals.var_acd);
        let assign71370_e107836: f64 = (assign71370_e107830 + assign71370_e107835);
        (assign71370_e107836, (((-(12.0 * locals.var_ty_dn0)) + (2.0 * locals.var_acd_dn0)) + (((1.414213562373095 * locals.var_acd_dn0) * locals.var_acd) + (assign71370_e107833 * locals.var_acd_dn0))), (((-(12.0 * locals.var_ty_dn2)) + (2.0 * locals.var_acd_dn2)) + (((1.414213562373095 * locals.var_acd_dn2) * locals.var_acd) + (assign71370_e107833 * locals.var_acd_dn2))), (((-(12.0 * locals.var_ty_dn4)) + (2.0 * locals.var_acd_dn4)) + (((1.414213562373095 * locals.var_acd_dn4) * locals.var_acd) + (assign71370_e107833 * locals.var_acd_dn4))), (((-(12.0 * locals.var_ty_dn5)) + (2.0 * locals.var_acd_dn5)) + (((1.414213562373095 * locals.var_acd_dn5) * locals.var_acd) + (assign71370_e107833 * locals.var_acd_dn5))), (((-(12.0 * locals.var_ty_dn6)) + (2.0 * locals.var_acd_dn6)) + (((1.414213562373095 * locals.var_acd_dn6) * locals.var_acd) + (assign71370_e107833 * locals.var_acd_dn6))), (((-(12.0 * locals.var_ty_dn7)) + (2.0 * locals.var_acd_dn7)) + (((1.414213562373095 * locals.var_acd_dn7) * locals.var_acd) + (assign71370_e107833 * locals.var_acd_dn7))), (((-(12.0 * locals.var_ty_dn8)) + (2.0 * locals.var_acd_dn8)) + (((1.414213562373095 * locals.var_acd_dn8) * locals.var_acd) + (assign71370_e107833 * locals.var_acd_dn8))), (((-(12.0 * locals.var_ty_dn9)) + (2.0 * locals.var_acd_dn9)) + (((1.414213562373095 * locals.var_acd_dn9) * locals.var_acd) + (assign71370_e107833 * locals.var_acd_dn9))), (((-(12.0 * locals.var_ty_dn10)) + (2.0 * locals.var_acd_dn10)) + (((1.414213562373095 * locals.var_acd_dn10) * locals.var_acd) + (assign71370_e107833 * locals.var_acd_dn10))), (((-(12.0 * locals.var_ty_dn11)) + (2.0 * locals.var_acd_dn11)) + (((1.414213562373095 * locals.var_acd_dn11) * locals.var_acd) + (assign71370_e107833 * locals.var_acd_dn11))), (((-(12.0 * locals.var_ty_dn14)) + (2.0 * locals.var_acd_dn14)) + (((1.414213562373095 * locals.var_acd_dn14) * locals.var_acd) + (assign71370_e107833 * locals.var_acd_dn14))),)
    } else {
        (locals.var_acn, locals.var_acn_dn0, locals.var_acn_dn2, locals.var_acn_dn4, locals.var_acn_dn5, locals.var_acn_dn6, locals.var_acn_dn7, locals.var_acn_dn8, locals.var_acn_dn9, locals.var_acn_dn10, locals.var_acn_dn11, locals.var_acn_dn14,)
    }
};
        locals.var_acn = assign71370_e107838;
        locals.var_acn_dn0 = assign71370_e107838_d_n0;
        locals.var_acn_dn2 = assign71370_e107838_d_n2;
        locals.var_acn_dn4 = assign71370_e107838_d_n4;
        locals.var_acn_dn5 = assign71370_e107838_d_n5;
        locals.var_acn_dn6 = assign71370_e107838_d_n6;
        locals.var_acn_dn7 = assign71370_e107838_d_n7;
        locals.var_acn_dn8 = assign71370_e107838_d_n8;
        locals.var_acn_dn9 = assign71370_e107838_d_n9;
        locals.var_acn_dn10 = assign71370_e107838_d_n10;
        locals.var_acn_dn11 = assign71370_e107838_d_n11;
        locals.var_acn_dn14 = assign71370_e107838_d_n14;

        let (assign71380_e107846, assign71380_e107846_d_n0, assign71380_e107846_d_n2, assign71380_e107846_d_n4, assign71380_e107846_d_n5, assign71380_e107846_d_n6, assign71380_e107846_d_n7, assign71380_e107846_d_n8, assign71380_e107846_d_n9, assign71380_e107846_d_n10, assign71380_e107846_d_n11, assign71380_e107846_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1669 != 0.0)) {
        let assign71380_e107844: f64 = (locals.var_acn / locals.var_acd);
        (assign71380_e107844, (((locals.var_acn_dn0 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn0)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn2 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn2)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn4 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn4)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn5 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn5)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn6 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn6)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn7 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn7)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn8 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn8)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn9 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn9)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn10 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn10)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn11 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn11)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn14 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn14)) / (locals.var_acd * locals.var_acd)),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
        locals.var_chi = assign71380_e107846;
        locals.var_chi_dn0 = assign71380_e107846_d_n0;
        locals.var_chi_dn2 = assign71380_e107846_d_n2;
        locals.var_chi_dn4 = assign71380_e107846_d_n4;
        locals.var_chi_dn5 = assign71380_e107846_d_n5;
        locals.var_chi_dn6 = assign71380_e107846_d_n6;
        locals.var_chi_dn7 = assign71380_e107846_d_n7;
        locals.var_chi_dn8 = assign71380_e107846_d_n8;
        locals.var_chi_dn9 = assign71380_e107846_d_n9;
        locals.var_chi_dn10 = assign71380_e107846_d_n10;
        locals.var_chi_dn11 = assign71380_e107846_d_n11;
        locals.var_chi_dn14 = assign71380_e107846_d_n14;

        let (assign71390_e107854, assign71390_e107854_d_n0, assign71390_e107854_d_n2, assign71390_e107854_d_n4, assign71390_e107854_d_n5, assign71390_e107854_d_n6, assign71390_e107854_d_n7, assign71390_e107854_d_n8, assign71390_e107854_d_n9, assign71390_e107854_d_n10, assign71390_e107854_d_n11, assign71390_e107854_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1669 != 0.0)) {
        let assign71390_e107852: f64 = (locals.var_chi * locals.var_beta_inv);
        (assign71390_e107852, ((locals.var_chi_dn0 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn0)), ((locals.var_chi_dn2 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn2)), ((locals.var_chi_dn4 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn4)), ((locals.var_chi_dn5 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn5)), ((locals.var_chi_dn6 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn6)), ((locals.var_chi_dn7 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn7)), ((locals.var_chi_dn8 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn8)), ((locals.var_chi_dn9 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn9)), ((locals.var_chi_dn10 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn10)), ((locals.var_chi_dn11 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn11)), ((locals.var_chi_dn14 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign71390_e107854;
        locals.var_t1_dn0 = assign71390_e107854_d_n0;
        locals.var_t1_dn2 = assign71390_e107854_d_n2;
        locals.var_t1_dn4 = assign71390_e107854_d_n4;
        locals.var_t1_dn5 = assign71390_e107854_d_n5;
        locals.var_t1_dn6 = assign71390_e107854_d_n6;
        locals.var_t1_dn7 = assign71390_e107854_d_n7;
        locals.var_t1_dn8 = assign71390_e107854_d_n8;
        locals.var_t1_dn9 = assign71390_e107854_d_n9;
        locals.var_t1_dn10 = assign71390_e107854_d_n10;
        locals.var_t1_dn11 = assign71390_e107854_d_n11;
        locals.var_t1_dn14 = assign71390_e107854_d_n14;

        let (assign71400_e107862, assign71400_e107862_d_n0, assign71400_e107862_d_n2, assign71400_e107862_d_n4, assign71400_e107862_d_n5, assign71400_e107862_d_n6, assign71400_e107862_d_n7, assign71400_e107862_d_n8, assign71400_e107862_d_n9, assign71400_e107862_d_n10, assign71400_e107862_d_n11, assign71400_e107862_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1669 != 0.0)) {
        let assign71400_e107860: f64 = (locals.var_t1 / locals.var_ps0_min);
        (assign71400_e107860, (((locals.var_t1_dn0 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn0)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn2 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn2)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn4 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn4)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn5 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn5)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn6 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn6)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn7 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn7)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn8 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn8)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn9 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn9)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn10 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn10)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn11 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn11)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn14 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn14)) / (locals.var_ps0_min * locals.var_ps0_min)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign71400_e107862;
        locals.var_t2_dn0 = assign71400_e107862_d_n0;
        locals.var_t2_dn2 = assign71400_e107862_d_n2;
        locals.var_t2_dn4 = assign71400_e107862_d_n4;
        locals.var_t2_dn5 = assign71400_e107862_d_n5;
        locals.var_t2_dn6 = assign71400_e107862_d_n6;
        locals.var_t2_dn7 = assign71400_e107862_d_n7;
        locals.var_t2_dn8 = assign71400_e107862_d_n8;
        locals.var_t2_dn9 = assign71400_e107862_d_n9;
        locals.var_t2_dn10 = assign71400_e107862_d_n10;
        locals.var_t2_dn11 = assign71400_e107862_d_n11;
        locals.var_t2_dn14 = assign71400_e107862_d_n14;

        let (assign71410_e107873, assign71410_e107873_d_n0, assign71410_e107873_d_n2, assign71410_e107873_d_n4, assign71410_e107873_d_n5, assign71410_e107873_d_n6, assign71410_e107873_d_n7, assign71410_e107873_d_n8, assign71410_e107873_d_n9, assign71410_e107873_d_n10, assign71410_e107873_d_n11, assign71410_e107873_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1669 != 0.0)) {
        let assign71410_e107869: f64 = (locals.var_t2 * locals.var_t2);
        let assign71410_e107870: f64 = (1.0 + assign71410_e107869);
        let assign71410_e107871: f64 = (assign71410_e107870).sqrt();
        (assign71410_e107871, (((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)) / (2.0 * assign71410_e107871)), (((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)) / (2.0 * assign71410_e107871)), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign71410_e107871)), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign71410_e107871)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign71410_e107871)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign71410_e107871)), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign71410_e107871)), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign71410_e107871)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign71410_e107871)), (((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign71410_e107871)), (((locals.var_t2_dn14 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn14)) / (2.0 * assign71410_e107871)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign71410_e107873;
        locals.var_t3_dn0 = assign71410_e107873_d_n0;
        locals.var_t3_dn2 = assign71410_e107873_d_n2;
        locals.var_t3_dn4 = assign71410_e107873_d_n4;
        locals.var_t3_dn5 = assign71410_e107873_d_n5;
        locals.var_t3_dn6 = assign71410_e107873_d_n6;
        locals.var_t3_dn7 = assign71410_e107873_d_n7;
        locals.var_t3_dn8 = assign71410_e107873_d_n8;
        locals.var_t3_dn9 = assign71410_e107873_d_n9;
        locals.var_t3_dn10 = assign71410_e107873_d_n10;
        locals.var_t3_dn11 = assign71410_e107873_d_n11;
        locals.var_t3_dn14 = assign71410_e107873_d_n14;

        let (assign71420_e107883, assign71420_e107883_d_n0, assign71420_e107883_d_n2, assign71420_e107883_d_n4, assign71420_e107883_d_n5, assign71420_e107883_d_n6, assign71420_e107883_d_n7, assign71420_e107883_d_n8, assign71420_e107883_d_n9, assign71420_e107883_d_n10, assign71420_e107883_d_n11, assign71420_e107883_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1669 != 0.0)) {
        let assign71420_e107879: f64 = (locals.var_t1 / locals.var_t3);
        let assign71420_e107881: f64 = (assign71420_e107879 - locals.var_vxbgmtcl);
        (assign71420_e107881, ((((locals.var_t1_dn0 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn0)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn0), ((((locals.var_t1_dn2 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn2)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn2), ((((locals.var_t1_dn4 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn4)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn4), ((((locals.var_t1_dn5 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn5)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn5), ((((locals.var_t1_dn6 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn6)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn6), ((((locals.var_t1_dn7 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn7)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn7), ((((locals.var_t1_dn8 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn8)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn8), ((((locals.var_t1_dn9 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn9)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn9), ((((locals.var_t1_dn10 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn10)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn10), ((((locals.var_t1_dn11 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn11)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn11), ((((locals.var_t1_dn14 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn14)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn14),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn14,)
    }
};
        locals.var_ps0ld = assign71420_e107883;
        locals.var_ps0ld_dn0 = assign71420_e107883_d_n0;
        locals.var_ps0ld_dn2 = assign71420_e107883_d_n2;
        locals.var_ps0ld_dn4 = assign71420_e107883_d_n4;
        locals.var_ps0ld_dn5 = assign71420_e107883_d_n5;
        locals.var_ps0ld_dn6 = assign71420_e107883_d_n6;
        locals.var_ps0ld_dn7 = assign71420_e107883_d_n7;
        locals.var_ps0ld_dn8 = assign71420_e107883_d_n8;
        locals.var_ps0ld_dn9 = assign71420_e107883_d_n9;
        locals.var_ps0ld_dn10 = assign71420_e107883_d_n10;
        locals.var_ps0ld_dn11 = assign71420_e107883_d_n11;
        locals.var_ps0ld_dn14 = assign71420_e107883_d_n14;

        let (assign71430_e107891, assign71430_e107891_d_n0, assign71430_e107891_d_n2, assign71430_e107891_d_n4, assign71430_e107891_d_n5, assign71430_e107891_d_n6, assign71430_e107891_d_n7, assign71430_e107891_d_n8, assign71430_e107891_d_n9, assign71430_e107891_d_n10, assign71430_e107891_d_n11, assign71430_e107891_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1669 != 0.0)) {
        let assign71430_e107889: f64 = (locals.var_vgpld - locals.var_ps0ld);
        (assign71430_e107889, (-locals.var_ps0ld_dn0), (locals.var_vgpld_dn2 - locals.var_ps0ld_dn2), (-locals.var_ps0ld_dn4), (-locals.var_ps0ld_dn5), (-locals.var_ps0ld_dn6), (locals.var_vgpld_dn7 - locals.var_ps0ld_dn7), (locals.var_vgpld_dn8 - locals.var_ps0ld_dn8), (locals.var_vgpld_dn9 - locals.var_ps0ld_dn9), (-locals.var_ps0ld_dn10), (-locals.var_ps0ld_dn11), (-locals.var_ps0ld_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign71430_e107891;
        locals.var_t2_dn0 = assign71430_e107891_d_n0;
        locals.var_t2_dn2 = assign71430_e107891_d_n2;
        locals.var_t2_dn4 = assign71430_e107891_d_n4;
        locals.var_t2_dn5 = assign71430_e107891_d_n5;
        locals.var_t2_dn6 = assign71430_e107891_d_n6;
        locals.var_t2_dn7 = assign71430_e107891_d_n7;
        locals.var_t2_dn8 = assign71430_e107891_d_n8;
        locals.var_t2_dn9 = assign71430_e107891_d_n9;
        locals.var_t2_dn10 = assign71430_e107891_d_n10;
        locals.var_t2_dn11 = assign71430_e107891_d_n11;
        locals.var_t2_dn14 = assign71430_e107891_d_n14;

        let (assign71440_e107899, assign71440_e107899_d_n0, assign71440_e107899_d_n2, assign71440_e107899_d_n4, assign71440_e107899_d_n5, assign71440_e107899_d_n6, assign71440_e107899_d_n7, assign71440_e107899_d_n8, assign71440_e107899_d_n9, assign71440_e107899_d_n10, assign71440_e107899_d_n11, assign71440_e107899_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1669 != 0.0)) {
        let assign71440_e107897: f64 = (locals.var_cox0_func * locals.var_t2);
        (assign71440_e107897, (locals.var_cox0_func * locals.var_t2_dn0), (locals.var_cox0_func * locals.var_t2_dn2), (locals.var_cox0_func * locals.var_t2_dn4), (locals.var_cox0_func * locals.var_t2_dn5), (locals.var_cox0_func * locals.var_t2_dn6), (locals.var_cox0_func * locals.var_t2_dn7), (locals.var_cox0_func * locals.var_t2_dn8), (locals.var_cox0_func * locals.var_t2_dn9), (locals.var_cox0_func * locals.var_t2_dn10), (locals.var_cox0_func * locals.var_t2_dn11), (locals.var_cox0_func * locals.var_t2_dn14),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn4, locals.var_qsuld_dn5, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn8, locals.var_qsuld_dn9, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn14,)
    }
};
        locals.var_qsuld = assign71440_e107899;
        locals.var_qsuld_dn0 = assign71440_e107899_d_n0;
        locals.var_qsuld_dn2 = assign71440_e107899_d_n2;
        locals.var_qsuld_dn4 = assign71440_e107899_d_n4;
        locals.var_qsuld_dn5 = assign71440_e107899_d_n5;
        locals.var_qsuld_dn6 = assign71440_e107899_d_n6;
        locals.var_qsuld_dn7 = assign71440_e107899_d_n7;
        locals.var_qsuld_dn8 = assign71440_e107899_d_n8;
        locals.var_qsuld_dn9 = assign71440_e107899_d_n9;
        locals.var_qsuld_dn10 = assign71440_e107899_d_n10;
        locals.var_qsuld_dn11 = assign71440_e107899_d_n11;
        locals.var_qsuld_dn14 = assign71440_e107899_d_n14;

        let (assign71450_e107905, assign71450_e107905_d_n0, assign71450_e107905_d_n2, assign71450_e107905_d_n4, assign71450_e107905_d_n5, assign71450_e107905_d_n6, assign71450_e107905_d_n7, assign71450_e107905_d_n8, assign71450_e107905_d_n9, assign71450_e107905_d_n10, assign71450_e107905_d_n11, assign71450_e107905_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1669 != 0.0)) {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn4, locals.var_qsuld_dn5, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn8, locals.var_qsuld_dn9, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn14,)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn8, locals.var_qbuld_dn9, locals.var_qbuld_dn10, locals.var_qbuld_dn11, locals.var_qbuld_dn14,)
    }
};
        locals.var_qbuld = assign71450_e107905;
        locals.var_qbuld_dn0 = assign71450_e107905_d_n0;
        locals.var_qbuld_dn2 = assign71450_e107905_d_n2;
        locals.var_qbuld_dn4 = assign71450_e107905_d_n4;
        locals.var_qbuld_dn5 = assign71450_e107905_d_n5;
        locals.var_qbuld_dn6 = assign71450_e107905_d_n6;
        locals.var_qbuld_dn7 = assign71450_e107905_d_n7;
        locals.var_qbuld_dn8 = assign71450_e107905_d_n8;
        locals.var_qbuld_dn9 = assign71450_e107905_d_n9;
        locals.var_qbuld_dn10 = assign71450_e107905_d_n10;
        locals.var_qbuld_dn11 = assign71450_e107905_d_n11;
        locals.var_qbuld_dn14 = assign71450_e107905_d_n14;

        let (assign71460_e107911, assign71460_e107911_d_n0, assign71460_e107911_d_n2, assign71460_e107911_d_n4, assign71460_e107911_d_n5, assign71460_e107911_d_n6, assign71460_e107911_d_n7, assign71460_e107911_d_n8, assign71460_e107911_d_n9, assign71460_e107911_d_n10, assign71460_e107911_d_n11, assign71460_e107911_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1669 != 0.0)) {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn14,)
    } else {
        (locals.var_ps0ld_ini, locals.var_ps0ld_ini_dn0, locals.var_ps0ld_ini_dn2, locals.var_ps0ld_ini_dn4, locals.var_ps0ld_ini_dn5, locals.var_ps0ld_ini_dn6, locals.var_ps0ld_ini_dn7, locals.var_ps0ld_ini_dn8, locals.var_ps0ld_ini_dn9, locals.var_ps0ld_ini_dn10, locals.var_ps0ld_ini_dn11, locals.var_ps0ld_ini_dn14,)
    }
};
        locals.var_ps0ld_ini = assign71460_e107911;
        locals.var_ps0ld_ini_dn0 = assign71460_e107911_d_n0;
        locals.var_ps0ld_ini_dn2 = assign71460_e107911_d_n2;
        locals.var_ps0ld_ini_dn4 = assign71460_e107911_d_n4;
        locals.var_ps0ld_ini_dn5 = assign71460_e107911_d_n5;
        locals.var_ps0ld_ini_dn6 = assign71460_e107911_d_n6;
        locals.var_ps0ld_ini_dn7 = assign71460_e107911_d_n7;
        locals.var_ps0ld_ini_dn8 = assign71460_e107911_d_n8;
        locals.var_ps0ld_ini_dn9 = assign71460_e107911_d_n9;
        locals.var_ps0ld_ini_dn10 = assign71460_e107911_d_n10;
        locals.var_ps0ld_ini_dn11 = assign71460_e107911_d_n11;
        locals.var_ps0ld_ini_dn14 = assign71460_e107911_d_n14;

        let assign71470_e107915: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign71470_e107916: f64 = (locals.var_beta * assign71470_e107915);
        let assign71470_e107920: f64 = (10.0 * 2.220446049250313e-16);
        let assign71470_e107922: f64 = (assign71470_e107920 - 1.0);
        let assign71470_e107924: f64 = (assign71470_e107922 * locals.var_fac1p2);
        let assign71470_e107926: f64 = (assign71470_e107924 * locals.var_beta2);
        let assign71470_e107928: f64 = (assign71470_e107926 / 4.0);
        let assign71470_e107929: f64 = (1.0 + assign71470_e107928);
        let assign71470_e107930: f64 = if assign71470_e107916 < assign71470_e107929 { 1.0 } else { 0.0 };
        locals.var_guard1671 = assign71470_e107930;

        let (assign71480_e107945, assign71480_e107945_d_n0, assign71480_e107945_d_n2, assign71480_e107945_d_n4, assign71480_e107945_d_n5, assign71480_e107945_d_n6, assign71480_e107945_d_n7, assign71480_e107945_d_n8, assign71480_e107945_d_n9, assign71480_e107945_d_n10, assign71480_e107945_d_n11, assign71480_e107945_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1669 == 0.0)) && (locals.var_guard1671 != 0.0)) {
        let assign71480_e107940: f64 = (locals.var_fac1p2 * locals.var_beta);
        let assign71480_e107942: f64 = (assign71480_e107940 / 2.0);
        let assign71480_e107943: f64 = (locals.var_vgpld + assign71480_e107942);
        (assign71480_e107943, (((locals.var_fac1p2_dn0 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn0)) / 2.0), (locals.var_vgpld_dn2 + (((locals.var_fac1p2_dn2 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn2)) / 2.0)), (((locals.var_fac1p2_dn4 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn4)) / 2.0), (((locals.var_fac1p2_dn5 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn5)) / 2.0), (((locals.var_fac1p2_dn6 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn6)) / 2.0), (locals.var_vgpld_dn7 + (((locals.var_fac1p2_dn7 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn7)) / 2.0)), (locals.var_vgpld_dn8 + (((locals.var_fac1p2_dn8 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn8)) / 2.0)), (locals.var_vgpld_dn9 + (((locals.var_fac1p2_dn9 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn9)) / 2.0)), (((locals.var_fac1p2_dn10 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn10)) / 2.0), (((locals.var_fac1p2_dn11 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn11)) / 2.0), (((locals.var_fac1p2_dn14 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn14)) / 2.0),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn14,)
    }
};
        locals.var_ps0_inia = assign71480_e107945;
        locals.var_ps0_inia_dn0 = assign71480_e107945_d_n0;
        locals.var_ps0_inia_dn2 = assign71480_e107945_d_n2;
        locals.var_ps0_inia_dn4 = assign71480_e107945_d_n4;
        locals.var_ps0_inia_dn5 = assign71480_e107945_d_n5;
        locals.var_ps0_inia_dn6 = assign71480_e107945_d_n6;
        locals.var_ps0_inia_dn7 = assign71480_e107945_d_n7;
        locals.var_ps0_inia_dn8 = assign71480_e107945_d_n8;
        locals.var_ps0_inia_dn9 = assign71480_e107945_d_n9;
        locals.var_ps0_inia_dn10 = assign71480_e107945_d_n10;
        locals.var_ps0_inia_dn11 = assign71480_e107945_d_n11;
        locals.var_ps0_inia_dn14 = assign71480_e107945_d_n14;

    }

    pub(super) fn stamp_transient_block_255(
        locals: &mut StampLocals,
    ) {
        let (assign71490_e107969, assign71490_e107969_d_n0, assign71490_e107969_d_n2, assign71490_e107969_d_n4, assign71490_e107969_d_n5, assign71490_e107969_d_n6, assign71490_e107969_d_n7, assign71490_e107969_d_n8, assign71490_e107969_d_n9, assign71490_e107969_d_n10, assign71490_e107969_d_n11, assign71490_e107969_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1669 == 0.0)) && (locals.var_guard1671 == 0.0)) {
        let assign71490_e107958: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign71490_e107959: f64 = (locals.var_beta * assign71490_e107958);
        let assign71490_e107961: f64 = (assign71490_e107959 - 1.0);
        let assign71490_e107962: f64 = (4.0 * assign71490_e107961);
        let assign71490_e107965: f64 = (locals.var_fac1p2 * locals.var_beta2);
        let assign71490_e107966: f64 = (assign71490_e107962 / assign71490_e107965);
        let assign71490_e107967: f64 = (1.0 + assign71490_e107966);
        (assign71490_e107967, ((((4.0 * ((locals.var_beta_dn0 * assign71490_e107958) + (locals.var_beta * locals.var_vxbgmtcl_dn0))) * assign71490_e107965) - (assign71490_e107962 * ((locals.var_fac1p2_dn0 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn0)))) / (assign71490_e107965 * assign71490_e107965)), ((((4.0 * ((locals.var_beta_dn2 * assign71490_e107958) + (locals.var_beta * (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2)))) * assign71490_e107965) - (assign71490_e107962 * ((locals.var_fac1p2_dn2 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn2)))) / (assign71490_e107965 * assign71490_e107965)), ((((4.0 * ((locals.var_beta_dn4 * assign71490_e107958) + (locals.var_beta * locals.var_vxbgmtcl_dn4))) * assign71490_e107965) - (assign71490_e107962 * ((locals.var_fac1p2_dn4 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn4)))) / (assign71490_e107965 * assign71490_e107965)), ((((4.0 * ((locals.var_beta_dn5 * assign71490_e107958) + (locals.var_beta * locals.var_vxbgmtcl_dn5))) * assign71490_e107965) - (assign71490_e107962 * ((locals.var_fac1p2_dn5 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn5)))) / (assign71490_e107965 * assign71490_e107965)), ((((4.0 * ((locals.var_beta_dn6 * assign71490_e107958) + (locals.var_beta * locals.var_vxbgmtcl_dn6))) * assign71490_e107965) - (assign71490_e107962 * ((locals.var_fac1p2_dn6 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn6)))) / (assign71490_e107965 * assign71490_e107965)), ((((4.0 * ((locals.var_beta_dn7 * assign71490_e107958) + (locals.var_beta * (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7)))) * assign71490_e107965) - (assign71490_e107962 * ((locals.var_fac1p2_dn7 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn7)))) / (assign71490_e107965 * assign71490_e107965)), ((((4.0 * ((locals.var_beta_dn8 * assign71490_e107958) + (locals.var_beta * (locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8)))) * assign71490_e107965) - (assign71490_e107962 * ((locals.var_fac1p2_dn8 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn8)))) / (assign71490_e107965 * assign71490_e107965)), ((((4.0 * ((locals.var_beta_dn9 * assign71490_e107958) + (locals.var_beta * (locals.var_vgpld_dn9 + locals.var_vxbgmtcl_dn9)))) * assign71490_e107965) - (assign71490_e107962 * ((locals.var_fac1p2_dn9 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn9)))) / (assign71490_e107965 * assign71490_e107965)), ((((4.0 * ((locals.var_beta_dn10 * assign71490_e107958) + (locals.var_beta * locals.var_vxbgmtcl_dn10))) * assign71490_e107965) - (assign71490_e107962 * ((locals.var_fac1p2_dn10 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn10)))) / (assign71490_e107965 * assign71490_e107965)), ((((4.0 * ((locals.var_beta_dn11 * assign71490_e107958) + (locals.var_beta * locals.var_vxbgmtcl_dn11))) * assign71490_e107965) - (assign71490_e107962 * ((locals.var_fac1p2_dn11 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn11)))) / (assign71490_e107965 * assign71490_e107965)), ((((4.0 * ((locals.var_beta_dn14 * assign71490_e107958) + (locals.var_beta * locals.var_vxbgmtcl_dn14))) * assign71490_e107965) - (assign71490_e107962 * ((locals.var_fac1p2_dn14 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn14)))) / (assign71490_e107965 * assign71490_e107965)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign71490_e107969;
        locals.var_tx_dn0 = assign71490_e107969_d_n0;
        locals.var_tx_dn2 = assign71490_e107969_d_n2;
        locals.var_tx_dn4 = assign71490_e107969_d_n4;
        locals.var_tx_dn5 = assign71490_e107969_d_n5;
        locals.var_tx_dn6 = assign71490_e107969_d_n6;
        locals.var_tx_dn7 = assign71490_e107969_d_n7;
        locals.var_tx_dn8 = assign71490_e107969_d_n8;
        locals.var_tx_dn9 = assign71490_e107969_d_n9;
        locals.var_tx_dn10 = assign71490_e107969_d_n10;
        locals.var_tx_dn11 = assign71490_e107969_d_n11;
        locals.var_tx_dn14 = assign71490_e107969_d_n14;

        let (assign71500_e107990, assign71500_e107990_d_n0, assign71500_e107990_d_n2, assign71500_e107990_d_n4, assign71500_e107990_d_n5, assign71500_e107990_d_n6, assign71500_e107990_d_n7, assign71500_e107990_d_n8, assign71500_e107990_d_n9, assign71500_e107990_d_n10, assign71500_e107990_d_n11, assign71500_e107990_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1669 == 0.0)) && (locals.var_guard1671 == 0.0)) {
        let assign71500_e107980: f64 = (locals.var_fac1p2 * locals.var_beta);
        let assign71500_e107982: f64 = (assign71500_e107980 / 2.0);
        let assign71500_e107985: f64 = (locals.var_tx).sqrt();
        let assign71500_e107986: f64 = (1.0 - assign71500_e107985);
        let assign71500_e107987: f64 = (assign71500_e107982 * assign71500_e107986);
        let assign71500_e107988: f64 = (locals.var_vgpld + assign71500_e107987);
        (assign71500_e107988, (((((locals.var_fac1p2_dn0 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn0)) / 2.0) * assign71500_e107986) + (assign71500_e107982 * (-(locals.var_tx_dn0 / (2.0 * assign71500_e107985))))), (locals.var_vgpld_dn2 + (((((locals.var_fac1p2_dn2 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn2)) / 2.0) * assign71500_e107986) + (assign71500_e107982 * (-(locals.var_tx_dn2 / (2.0 * assign71500_e107985)))))), (((((locals.var_fac1p2_dn4 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn4)) / 2.0) * assign71500_e107986) + (assign71500_e107982 * (-(locals.var_tx_dn4 / (2.0 * assign71500_e107985))))), (((((locals.var_fac1p2_dn5 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn5)) / 2.0) * assign71500_e107986) + (assign71500_e107982 * (-(locals.var_tx_dn5 / (2.0 * assign71500_e107985))))), (((((locals.var_fac1p2_dn6 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn6)) / 2.0) * assign71500_e107986) + (assign71500_e107982 * (-(locals.var_tx_dn6 / (2.0 * assign71500_e107985))))), (locals.var_vgpld_dn7 + (((((locals.var_fac1p2_dn7 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn7)) / 2.0) * assign71500_e107986) + (assign71500_e107982 * (-(locals.var_tx_dn7 / (2.0 * assign71500_e107985)))))), (locals.var_vgpld_dn8 + (((((locals.var_fac1p2_dn8 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn8)) / 2.0) * assign71500_e107986) + (assign71500_e107982 * (-(locals.var_tx_dn8 / (2.0 * assign71500_e107985)))))), (locals.var_vgpld_dn9 + (((((locals.var_fac1p2_dn9 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn9)) / 2.0) * assign71500_e107986) + (assign71500_e107982 * (-(locals.var_tx_dn9 / (2.0 * assign71500_e107985)))))), (((((locals.var_fac1p2_dn10 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn10)) / 2.0) * assign71500_e107986) + (assign71500_e107982 * (-(locals.var_tx_dn10 / (2.0 * assign71500_e107985))))), (((((locals.var_fac1p2_dn11 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn11)) / 2.0) * assign71500_e107986) + (assign71500_e107982 * (-(locals.var_tx_dn11 / (2.0 * assign71500_e107985))))), (((((locals.var_fac1p2_dn14 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn14)) / 2.0) * assign71500_e107986) + (assign71500_e107982 * (-(locals.var_tx_dn14 / (2.0 * assign71500_e107985))))),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn14,)
    }
};
        locals.var_ps0_inia = assign71500_e107990;
        locals.var_ps0_inia_dn0 = assign71500_e107990_d_n0;
        locals.var_ps0_inia_dn2 = assign71500_e107990_d_n2;
        locals.var_ps0_inia_dn4 = assign71500_e107990_d_n4;
        locals.var_ps0_inia_dn5 = assign71500_e107990_d_n5;
        locals.var_ps0_inia_dn6 = assign71500_e107990_d_n6;
        locals.var_ps0_inia_dn7 = assign71500_e107990_d_n7;
        locals.var_ps0_inia_dn8 = assign71500_e107990_d_n8;
        locals.var_ps0_inia_dn9 = assign71500_e107990_d_n9;
        locals.var_ps0_inia_dn10 = assign71500_e107990_d_n10;
        locals.var_ps0_inia_dn11 = assign71500_e107990_d_n11;
        locals.var_ps0_inia_dn14 = assign71500_e107990_d_n14;

        let (assign71510_e108001, assign71510_e108001_d_n0, assign71510_e108001_d_n2, assign71510_e108001_d_n4, assign71510_e108001_d_n5, assign71510_e108001_d_n6, assign71510_e108001_d_n7, assign71510_e108001_d_n8, assign71510_e108001_d_n9, assign71510_e108001_d_n10, assign71510_e108001_d_n11, assign71510_e108001_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1669 == 0.0)) {
        let assign71510_e107998: f64 = (locals.var_ps0_inia + locals.var_vxbgmtcl);
        let assign71510_e107999: f64 = (locals.var_beta * assign71510_e107998);
        (assign71510_e107999, ((locals.var_beta_dn0 * assign71510_e107998) + (locals.var_beta * (locals.var_ps0_inia_dn0 + locals.var_vxbgmtcl_dn0))), ((locals.var_beta_dn2 * assign71510_e107998) + (locals.var_beta * (locals.var_ps0_inia_dn2 + locals.var_vxbgmtcl_dn2))), ((locals.var_beta_dn4 * assign71510_e107998) + (locals.var_beta * (locals.var_ps0_inia_dn4 + locals.var_vxbgmtcl_dn4))), ((locals.var_beta_dn5 * assign71510_e107998) + (locals.var_beta * (locals.var_ps0_inia_dn5 + locals.var_vxbgmtcl_dn5))), ((locals.var_beta_dn6 * assign71510_e107998) + (locals.var_beta * (locals.var_ps0_inia_dn6 + locals.var_vxbgmtcl_dn6))), ((locals.var_beta_dn7 * assign71510_e107998) + (locals.var_beta * (locals.var_ps0_inia_dn7 + locals.var_vxbgmtcl_dn7))), ((locals.var_beta_dn8 * assign71510_e107998) + (locals.var_beta * (locals.var_ps0_inia_dn8 + locals.var_vxbgmtcl_dn8))), ((locals.var_beta_dn9 * assign71510_e107998) + (locals.var_beta * (locals.var_ps0_inia_dn9 + locals.var_vxbgmtcl_dn9))), ((locals.var_beta_dn10 * assign71510_e107998) + (locals.var_beta * (locals.var_ps0_inia_dn10 + locals.var_vxbgmtcl_dn10))), ((locals.var_beta_dn11 * assign71510_e107998) + (locals.var_beta * (locals.var_ps0_inia_dn11 + locals.var_vxbgmtcl_dn11))), ((locals.var_beta_dn14 * assign71510_e107998) + (locals.var_beta * (locals.var_ps0_inia_dn14 + locals.var_vxbgmtcl_dn14))),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
        locals.var_chi = assign71510_e108001;
        locals.var_chi_dn0 = assign71510_e108001_d_n0;
        locals.var_chi_dn2 = assign71510_e108001_d_n2;
        locals.var_chi_dn4 = assign71510_e108001_d_n4;
        locals.var_chi_dn5 = assign71510_e108001_d_n5;
        locals.var_chi_dn6 = assign71510_e108001_d_n6;
        locals.var_chi_dn7 = assign71510_e108001_d_n7;
        locals.var_chi_dn8 = assign71510_e108001_d_n8;
        locals.var_chi_dn9 = assign71510_e108001_d_n9;
        locals.var_chi_dn10 = assign71510_e108001_d_n10;
        locals.var_chi_dn11 = assign71510_e108001_d_n11;
        locals.var_chi_dn14 = assign71510_e108001_d_n14;

        let assign71520_e108004: f64 = if locals.var_chi >= 3.0 { 1.0 } else { 0.0 };
        locals.var_guard1672 = assign71520_e108004;

        let (assign71540_e108024, assign71540_e108024_d_n0, assign71540_e108024_d_n2, assign71540_e108024_d_n4, assign71540_e108024_d_n5, assign71540_e108024_d_n6, assign71540_e108024_d_n7, assign71540_e108024_d_n8, assign71540_e108024_d_n9, assign71540_e108024_d_n10, assign71540_e108024_d_n11, assign71540_e108024_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1669 == 0.0)) && (locals.var_guard1672 != 0.0)) {
        let assign71540_e108021: f64 = (-locals.var_chi);
        let assign71540_e108022: f64 = (assign71540_e108021).exp();
        (assign71540_e108022, (assign71540_e108022 * (-locals.var_chi_dn0)), (assign71540_e108022 * (-locals.var_chi_dn2)), (assign71540_e108022 * (-locals.var_chi_dn4)), (assign71540_e108022 * (-locals.var_chi_dn5)), (assign71540_e108022 * (-locals.var_chi_dn6)), (assign71540_e108022 * (-locals.var_chi_dn7)), (assign71540_e108022 * (-locals.var_chi_dn8)), (assign71540_e108022 * (-locals.var_chi_dn9)), (assign71540_e108022 * (-locals.var_chi_dn10)), (assign71540_e108022 * (-locals.var_chi_dn11)), (assign71540_e108022 * (-locals.var_chi_dn14)),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn14,)
    }
};
        locals.var_ty = assign71540_e108024;
        locals.var_ty_dn0 = assign71540_e108024_d_n0;
        locals.var_ty_dn2 = assign71540_e108024_d_n2;
        locals.var_ty_dn4 = assign71540_e108024_d_n4;
        locals.var_ty_dn5 = assign71540_e108024_d_n5;
        locals.var_ty_dn6 = assign71540_e108024_d_n6;
        locals.var_ty_dn7 = assign71540_e108024_d_n7;
        locals.var_ty_dn8 = assign71540_e108024_d_n8;
        locals.var_ty_dn9 = assign71540_e108024_d_n9;
        locals.var_ty_dn10 = assign71540_e108024_d_n10;
        locals.var_ty_dn11 = assign71540_e108024_d_n11;
        locals.var_ty_dn14 = assign71540_e108024_d_n14;

        let (assign71550_e108049, assign71550_e108049_d_n0, assign71550_e108049_d_n2, assign71550_e108049_d_n4, assign71550_e108049_d_n5, assign71550_e108049_d_n6, assign71550_e108049_d_n7, assign71550_e108049_d_n8, assign71550_e108049_d_n9, assign71550_e108049_d_n10, assign71550_e108049_d_n11, assign71550_e108049_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1669 == 0.0)) && (locals.var_guard1672 != 0.0)) {
        let assign71550_e108036: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign71550_e108037: f64 = (locals.var_beta * assign71550_e108036);
        let assign71550_e108039: f64 = (assign71550_e108037 - 1.0);
        let assign71550_e108041: f64 = (assign71550_e108039 + locals.var_ty);
        let assign71550_e108042: f64 = (4.0 * assign71550_e108041);
        let assign71550_e108045: f64 = (locals.var_fac1p2 * locals.var_beta2);
        let assign71550_e108046: f64 = (assign71550_e108042 / assign71550_e108045);
        let assign71550_e108047: f64 = (1.0 + assign71550_e108046);
        (assign71550_e108047, ((((4.0 * (((locals.var_beta_dn0 * assign71550_e108036) + (locals.var_beta * locals.var_vxbgmtcl_dn0)) + locals.var_ty_dn0)) * assign71550_e108045) - (assign71550_e108042 * ((locals.var_fac1p2_dn0 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn0)))) / (assign71550_e108045 * assign71550_e108045)), ((((4.0 * (((locals.var_beta_dn2 * assign71550_e108036) + (locals.var_beta * (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2))) + locals.var_ty_dn2)) * assign71550_e108045) - (assign71550_e108042 * ((locals.var_fac1p2_dn2 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn2)))) / (assign71550_e108045 * assign71550_e108045)), ((((4.0 * (((locals.var_beta_dn4 * assign71550_e108036) + (locals.var_beta * locals.var_vxbgmtcl_dn4)) + locals.var_ty_dn4)) * assign71550_e108045) - (assign71550_e108042 * ((locals.var_fac1p2_dn4 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn4)))) / (assign71550_e108045 * assign71550_e108045)), ((((4.0 * (((locals.var_beta_dn5 * assign71550_e108036) + (locals.var_beta * locals.var_vxbgmtcl_dn5)) + locals.var_ty_dn5)) * assign71550_e108045) - (assign71550_e108042 * ((locals.var_fac1p2_dn5 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn5)))) / (assign71550_e108045 * assign71550_e108045)), ((((4.0 * (((locals.var_beta_dn6 * assign71550_e108036) + (locals.var_beta * locals.var_vxbgmtcl_dn6)) + locals.var_ty_dn6)) * assign71550_e108045) - (assign71550_e108042 * ((locals.var_fac1p2_dn6 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn6)))) / (assign71550_e108045 * assign71550_e108045)), ((((4.0 * (((locals.var_beta_dn7 * assign71550_e108036) + (locals.var_beta * (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7))) + locals.var_ty_dn7)) * assign71550_e108045) - (assign71550_e108042 * ((locals.var_fac1p2_dn7 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn7)))) / (assign71550_e108045 * assign71550_e108045)), ((((4.0 * (((locals.var_beta_dn8 * assign71550_e108036) + (locals.var_beta * (locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8))) + locals.var_ty_dn8)) * assign71550_e108045) - (assign71550_e108042 * ((locals.var_fac1p2_dn8 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn8)))) / (assign71550_e108045 * assign71550_e108045)), ((((4.0 * (((locals.var_beta_dn9 * assign71550_e108036) + (locals.var_beta * (locals.var_vgpld_dn9 + locals.var_vxbgmtcl_dn9))) + locals.var_ty_dn9)) * assign71550_e108045) - (assign71550_e108042 * ((locals.var_fac1p2_dn9 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn9)))) / (assign71550_e108045 * assign71550_e108045)), ((((4.0 * (((locals.var_beta_dn10 * assign71550_e108036) + (locals.var_beta * locals.var_vxbgmtcl_dn10)) + locals.var_ty_dn10)) * assign71550_e108045) - (assign71550_e108042 * ((locals.var_fac1p2_dn10 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn10)))) / (assign71550_e108045 * assign71550_e108045)), ((((4.0 * (((locals.var_beta_dn11 * assign71550_e108036) + (locals.var_beta * locals.var_vxbgmtcl_dn11)) + locals.var_ty_dn11)) * assign71550_e108045) - (assign71550_e108042 * ((locals.var_fac1p2_dn11 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn11)))) / (assign71550_e108045 * assign71550_e108045)), ((((4.0 * (((locals.var_beta_dn14 * assign71550_e108036) + (locals.var_beta * locals.var_vxbgmtcl_dn14)) + locals.var_ty_dn14)) * assign71550_e108045) - (assign71550_e108042 * ((locals.var_fac1p2_dn14 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn14)))) / (assign71550_e108045 * assign71550_e108045)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign71550_e108049;
        locals.var_tx_dn0 = assign71550_e108049_d_n0;
        locals.var_tx_dn2 = assign71550_e108049_d_n2;
        locals.var_tx_dn4 = assign71550_e108049_d_n4;
        locals.var_tx_dn5 = assign71550_e108049_d_n5;
        locals.var_tx_dn6 = assign71550_e108049_d_n6;
        locals.var_tx_dn7 = assign71550_e108049_d_n7;
        locals.var_tx_dn8 = assign71550_e108049_d_n8;
        locals.var_tx_dn9 = assign71550_e108049_d_n9;
        locals.var_tx_dn10 = assign71550_e108049_d_n10;
        locals.var_tx_dn11 = assign71550_e108049_d_n11;
        locals.var_tx_dn14 = assign71550_e108049_d_n14;

        let (assign71560_e108069, assign71560_e108069_d_n0, assign71560_e108069_d_n2, assign71560_e108069_d_n4, assign71560_e108069_d_n5, assign71560_e108069_d_n6, assign71560_e108069_d_n7, assign71560_e108069_d_n8, assign71560_e108069_d_n9, assign71560_e108069_d_n10, assign71560_e108069_d_n11, assign71560_e108069_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1669 == 0.0)) && (locals.var_guard1672 != 0.0)) {
        let assign71560_e108059: f64 = (locals.var_fac1p2 * locals.var_beta);
        let assign71560_e108061: f64 = (assign71560_e108059 / 2.0);
        let assign71560_e108064: f64 = (locals.var_tx).sqrt();
        let assign71560_e108065: f64 = (1.0 - assign71560_e108064);
        let assign71560_e108066: f64 = (assign71560_e108061 * assign71560_e108065);
        let assign71560_e108067: f64 = (locals.var_vgpld + assign71560_e108066);
        (assign71560_e108067, (((((locals.var_fac1p2_dn0 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn0)) / 2.0) * assign71560_e108065) + (assign71560_e108061 * (-(locals.var_tx_dn0 / (2.0 * assign71560_e108064))))), (locals.var_vgpld_dn2 + (((((locals.var_fac1p2_dn2 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn2)) / 2.0) * assign71560_e108065) + (assign71560_e108061 * (-(locals.var_tx_dn2 / (2.0 * assign71560_e108064)))))), (((((locals.var_fac1p2_dn4 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn4)) / 2.0) * assign71560_e108065) + (assign71560_e108061 * (-(locals.var_tx_dn4 / (2.0 * assign71560_e108064))))), (((((locals.var_fac1p2_dn5 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn5)) / 2.0) * assign71560_e108065) + (assign71560_e108061 * (-(locals.var_tx_dn5 / (2.0 * assign71560_e108064))))), (((((locals.var_fac1p2_dn6 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn6)) / 2.0) * assign71560_e108065) + (assign71560_e108061 * (-(locals.var_tx_dn6 / (2.0 * assign71560_e108064))))), (locals.var_vgpld_dn7 + (((((locals.var_fac1p2_dn7 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn7)) / 2.0) * assign71560_e108065) + (assign71560_e108061 * (-(locals.var_tx_dn7 / (2.0 * assign71560_e108064)))))), (locals.var_vgpld_dn8 + (((((locals.var_fac1p2_dn8 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn8)) / 2.0) * assign71560_e108065) + (assign71560_e108061 * (-(locals.var_tx_dn8 / (2.0 * assign71560_e108064)))))), (locals.var_vgpld_dn9 + (((((locals.var_fac1p2_dn9 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn9)) / 2.0) * assign71560_e108065) + (assign71560_e108061 * (-(locals.var_tx_dn9 / (2.0 * assign71560_e108064)))))), (((((locals.var_fac1p2_dn10 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn10)) / 2.0) * assign71560_e108065) + (assign71560_e108061 * (-(locals.var_tx_dn10 / (2.0 * assign71560_e108064))))), (((((locals.var_fac1p2_dn11 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn11)) / 2.0) * assign71560_e108065) + (assign71560_e108061 * (-(locals.var_tx_dn11 / (2.0 * assign71560_e108064))))), (((((locals.var_fac1p2_dn14 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn14)) / 2.0) * assign71560_e108065) + (assign71560_e108061 * (-(locals.var_tx_dn14 / (2.0 * assign71560_e108064))))),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn14,)
    }
};
        locals.var_ps0_inia = assign71560_e108069;
        locals.var_ps0_inia_dn0 = assign71560_e108069_d_n0;
        locals.var_ps0_inia_dn2 = assign71560_e108069_d_n2;
        locals.var_ps0_inia_dn4 = assign71560_e108069_d_n4;
        locals.var_ps0_inia_dn5 = assign71560_e108069_d_n5;
        locals.var_ps0_inia_dn6 = assign71560_e108069_d_n6;
        locals.var_ps0_inia_dn7 = assign71560_e108069_d_n7;
        locals.var_ps0_inia_dn8 = assign71560_e108069_d_n8;
        locals.var_ps0_inia_dn9 = assign71560_e108069_d_n9;
        locals.var_ps0_inia_dn10 = assign71560_e108069_d_n10;
        locals.var_ps0_inia_dn11 = assign71560_e108069_d_n11;
        locals.var_ps0_inia_dn14 = assign71560_e108069_d_n14;

        let (assign71570_e108082, assign71570_e108082_d_n0, assign71570_e108082_d_n2, assign71570_e108082_d_n4, assign71570_e108082_d_n5, assign71570_e108082_d_n6, assign71570_e108082_d_n7, assign71570_e108082_d_n8, assign71570_e108082_d_n9, assign71570_e108082_d_n10, assign71570_e108082_d_n11, assign71570_e108082_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1669 == 0.0)) && (locals.var_guard1672 != 0.0)) {
        let assign71570_e108079: f64 = (locals.var_ps0_inia + locals.var_vxbgmtcl);
        let assign71570_e108080: f64 = (locals.var_beta * assign71570_e108079);
        (assign71570_e108080, ((locals.var_beta_dn0 * assign71570_e108079) + (locals.var_beta * (locals.var_ps0_inia_dn0 + locals.var_vxbgmtcl_dn0))), ((locals.var_beta_dn2 * assign71570_e108079) + (locals.var_beta * (locals.var_ps0_inia_dn2 + locals.var_vxbgmtcl_dn2))), ((locals.var_beta_dn4 * assign71570_e108079) + (locals.var_beta * (locals.var_ps0_inia_dn4 + locals.var_vxbgmtcl_dn4))), ((locals.var_beta_dn5 * assign71570_e108079) + (locals.var_beta * (locals.var_ps0_inia_dn5 + locals.var_vxbgmtcl_dn5))), ((locals.var_beta_dn6 * assign71570_e108079) + (locals.var_beta * (locals.var_ps0_inia_dn6 + locals.var_vxbgmtcl_dn6))), ((locals.var_beta_dn7 * assign71570_e108079) + (locals.var_beta * (locals.var_ps0_inia_dn7 + locals.var_vxbgmtcl_dn7))), ((locals.var_beta_dn8 * assign71570_e108079) + (locals.var_beta * (locals.var_ps0_inia_dn8 + locals.var_vxbgmtcl_dn8))), ((locals.var_beta_dn9 * assign71570_e108079) + (locals.var_beta * (locals.var_ps0_inia_dn9 + locals.var_vxbgmtcl_dn9))), ((locals.var_beta_dn10 * assign71570_e108079) + (locals.var_beta * (locals.var_ps0_inia_dn10 + locals.var_vxbgmtcl_dn10))), ((locals.var_beta_dn11 * assign71570_e108079) + (locals.var_beta * (locals.var_ps0_inia_dn11 + locals.var_vxbgmtcl_dn11))), ((locals.var_beta_dn14 * assign71570_e108079) + (locals.var_beta * (locals.var_ps0_inia_dn14 + locals.var_vxbgmtcl_dn14))),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
        locals.var_chi = assign71570_e108082;
        locals.var_chi_dn0 = assign71570_e108082_d_n0;
        locals.var_chi_dn2 = assign71570_e108082_d_n2;
        locals.var_chi_dn4 = assign71570_e108082_d_n4;
        locals.var_chi_dn5 = assign71570_e108082_d_n5;
        locals.var_chi_dn6 = assign71570_e108082_d_n6;
        locals.var_chi_dn7 = assign71570_e108082_d_n7;
        locals.var_chi_dn8 = assign71570_e108082_d_n8;
        locals.var_chi_dn9 = assign71570_e108082_d_n9;
        locals.var_chi_dn10 = assign71570_e108082_d_n10;
        locals.var_chi_dn11 = assign71570_e108082_d_n11;
        locals.var_chi_dn14 = assign71570_e108082_d_n14;

        let (assign71580_e108093, assign71580_e108093_d_n0, assign71580_e108093_d_n2, assign71580_e108093_d_n4, assign71580_e108093_d_n5, assign71580_e108093_d_n6, assign71580_e108093_d_n7, assign71580_e108093_d_n8, assign71580_e108093_d_n9, assign71580_e108093_d_n10, assign71580_e108093_d_n11, assign71580_e108093_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1669 == 0.0)) && (locals.var_guard1672 != 0.0)) {
        let assign71580_e108090: f64 = (-locals.var_chi);
        let assign71580_e108091: f64 = (assign71580_e108090).exp();
        (assign71580_e108091, (assign71580_e108091 * (-locals.var_chi_dn0)), (assign71580_e108091 * (-locals.var_chi_dn2)), (assign71580_e108091 * (-locals.var_chi_dn4)), (assign71580_e108091 * (-locals.var_chi_dn5)), (assign71580_e108091 * (-locals.var_chi_dn6)), (assign71580_e108091 * (-locals.var_chi_dn7)), (assign71580_e108091 * (-locals.var_chi_dn8)), (assign71580_e108091 * (-locals.var_chi_dn9)), (assign71580_e108091 * (-locals.var_chi_dn10)), (assign71580_e108091 * (-locals.var_chi_dn11)), (assign71580_e108091 * (-locals.var_chi_dn14)),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn14,)
    }
};
        locals.var_ty = assign71580_e108093;
        locals.var_ty_dn0 = assign71580_e108093_d_n0;
        locals.var_ty_dn2 = assign71580_e108093_d_n2;
        locals.var_ty_dn4 = assign71580_e108093_d_n4;
        locals.var_ty_dn5 = assign71580_e108093_d_n5;
        locals.var_ty_dn6 = assign71580_e108093_d_n6;
        locals.var_ty_dn7 = assign71580_e108093_d_n7;
        locals.var_ty_dn8 = assign71580_e108093_d_n8;
        locals.var_ty_dn9 = assign71580_e108093_d_n9;
        locals.var_ty_dn10 = assign71580_e108093_d_n10;
        locals.var_ty_dn11 = assign71580_e108093_d_n11;
        locals.var_ty_dn14 = assign71580_e108093_d_n14;

        let (assign71590_e108118, assign71590_e108118_d_n0, assign71590_e108118_d_n2, assign71590_e108118_d_n4, assign71590_e108118_d_n5, assign71590_e108118_d_n6, assign71590_e108118_d_n7, assign71590_e108118_d_n8, assign71590_e108118_d_n9, assign71590_e108118_d_n10, assign71590_e108118_d_n11, assign71590_e108118_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1669 == 0.0)) && (locals.var_guard1672 != 0.0)) {
        let assign71590_e108105: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign71590_e108106: f64 = (locals.var_beta * assign71590_e108105);
        let assign71590_e108108: f64 = (assign71590_e108106 - 1.0);
        let assign71590_e108110: f64 = (assign71590_e108108 + locals.var_ty);
        let assign71590_e108111: f64 = (4.0 * assign71590_e108110);
        let assign71590_e108114: f64 = (locals.var_fac1p2 * locals.var_beta2);
        let assign71590_e108115: f64 = (assign71590_e108111 / assign71590_e108114);
        let assign71590_e108116: f64 = (1.0 + assign71590_e108115);
        (assign71590_e108116, ((((4.0 * (((locals.var_beta_dn0 * assign71590_e108105) + (locals.var_beta * locals.var_vxbgmtcl_dn0)) + locals.var_ty_dn0)) * assign71590_e108114) - (assign71590_e108111 * ((locals.var_fac1p2_dn0 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn0)))) / (assign71590_e108114 * assign71590_e108114)), ((((4.0 * (((locals.var_beta_dn2 * assign71590_e108105) + (locals.var_beta * (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2))) + locals.var_ty_dn2)) * assign71590_e108114) - (assign71590_e108111 * ((locals.var_fac1p2_dn2 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn2)))) / (assign71590_e108114 * assign71590_e108114)), ((((4.0 * (((locals.var_beta_dn4 * assign71590_e108105) + (locals.var_beta * locals.var_vxbgmtcl_dn4)) + locals.var_ty_dn4)) * assign71590_e108114) - (assign71590_e108111 * ((locals.var_fac1p2_dn4 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn4)))) / (assign71590_e108114 * assign71590_e108114)), ((((4.0 * (((locals.var_beta_dn5 * assign71590_e108105) + (locals.var_beta * locals.var_vxbgmtcl_dn5)) + locals.var_ty_dn5)) * assign71590_e108114) - (assign71590_e108111 * ((locals.var_fac1p2_dn5 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn5)))) / (assign71590_e108114 * assign71590_e108114)), ((((4.0 * (((locals.var_beta_dn6 * assign71590_e108105) + (locals.var_beta * locals.var_vxbgmtcl_dn6)) + locals.var_ty_dn6)) * assign71590_e108114) - (assign71590_e108111 * ((locals.var_fac1p2_dn6 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn6)))) / (assign71590_e108114 * assign71590_e108114)), ((((4.0 * (((locals.var_beta_dn7 * assign71590_e108105) + (locals.var_beta * (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7))) + locals.var_ty_dn7)) * assign71590_e108114) - (assign71590_e108111 * ((locals.var_fac1p2_dn7 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn7)))) / (assign71590_e108114 * assign71590_e108114)), ((((4.0 * (((locals.var_beta_dn8 * assign71590_e108105) + (locals.var_beta * (locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8))) + locals.var_ty_dn8)) * assign71590_e108114) - (assign71590_e108111 * ((locals.var_fac1p2_dn8 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn8)))) / (assign71590_e108114 * assign71590_e108114)), ((((4.0 * (((locals.var_beta_dn9 * assign71590_e108105) + (locals.var_beta * (locals.var_vgpld_dn9 + locals.var_vxbgmtcl_dn9))) + locals.var_ty_dn9)) * assign71590_e108114) - (assign71590_e108111 * ((locals.var_fac1p2_dn9 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn9)))) / (assign71590_e108114 * assign71590_e108114)), ((((4.0 * (((locals.var_beta_dn10 * assign71590_e108105) + (locals.var_beta * locals.var_vxbgmtcl_dn10)) + locals.var_ty_dn10)) * assign71590_e108114) - (assign71590_e108111 * ((locals.var_fac1p2_dn10 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn10)))) / (assign71590_e108114 * assign71590_e108114)), ((((4.0 * (((locals.var_beta_dn11 * assign71590_e108105) + (locals.var_beta * locals.var_vxbgmtcl_dn11)) + locals.var_ty_dn11)) * assign71590_e108114) - (assign71590_e108111 * ((locals.var_fac1p2_dn11 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn11)))) / (assign71590_e108114 * assign71590_e108114)), ((((4.0 * (((locals.var_beta_dn14 * assign71590_e108105) + (locals.var_beta * locals.var_vxbgmtcl_dn14)) + locals.var_ty_dn14)) * assign71590_e108114) - (assign71590_e108111 * ((locals.var_fac1p2_dn14 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn14)))) / (assign71590_e108114 * assign71590_e108114)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign71590_e108118;
        locals.var_tx_dn0 = assign71590_e108118_d_n0;
        locals.var_tx_dn2 = assign71590_e108118_d_n2;
        locals.var_tx_dn4 = assign71590_e108118_d_n4;
        locals.var_tx_dn5 = assign71590_e108118_d_n5;
        locals.var_tx_dn6 = assign71590_e108118_d_n6;
        locals.var_tx_dn7 = assign71590_e108118_d_n7;
        locals.var_tx_dn8 = assign71590_e108118_d_n8;
        locals.var_tx_dn9 = assign71590_e108118_d_n9;
        locals.var_tx_dn10 = assign71590_e108118_d_n10;
        locals.var_tx_dn11 = assign71590_e108118_d_n11;
        locals.var_tx_dn14 = assign71590_e108118_d_n14;

        let (assign71600_e108138, assign71600_e108138_d_n0, assign71600_e108138_d_n2, assign71600_e108138_d_n4, assign71600_e108138_d_n5, assign71600_e108138_d_n6, assign71600_e108138_d_n7, assign71600_e108138_d_n8, assign71600_e108138_d_n9, assign71600_e108138_d_n10, assign71600_e108138_d_n11, assign71600_e108138_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1669 == 0.0)) && (locals.var_guard1672 != 0.0)) {
        let assign71600_e108128: f64 = (locals.var_fac1p2 * locals.var_beta);
        let assign71600_e108130: f64 = (assign71600_e108128 / 2.0);
        let assign71600_e108133: f64 = (locals.var_tx).sqrt();
        let assign71600_e108134: f64 = (1.0 - assign71600_e108133);
        let assign71600_e108135: f64 = (assign71600_e108130 * assign71600_e108134);
        let assign71600_e108136: f64 = (locals.var_vgpld + assign71600_e108135);
        (assign71600_e108136, (((((locals.var_fac1p2_dn0 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn0)) / 2.0) * assign71600_e108134) + (assign71600_e108130 * (-(locals.var_tx_dn0 / (2.0 * assign71600_e108133))))), (locals.var_vgpld_dn2 + (((((locals.var_fac1p2_dn2 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn2)) / 2.0) * assign71600_e108134) + (assign71600_e108130 * (-(locals.var_tx_dn2 / (2.0 * assign71600_e108133)))))), (((((locals.var_fac1p2_dn4 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn4)) / 2.0) * assign71600_e108134) + (assign71600_e108130 * (-(locals.var_tx_dn4 / (2.0 * assign71600_e108133))))), (((((locals.var_fac1p2_dn5 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn5)) / 2.0) * assign71600_e108134) + (assign71600_e108130 * (-(locals.var_tx_dn5 / (2.0 * assign71600_e108133))))), (((((locals.var_fac1p2_dn6 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn6)) / 2.0) * assign71600_e108134) + (assign71600_e108130 * (-(locals.var_tx_dn6 / (2.0 * assign71600_e108133))))), (locals.var_vgpld_dn7 + (((((locals.var_fac1p2_dn7 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn7)) / 2.0) * assign71600_e108134) + (assign71600_e108130 * (-(locals.var_tx_dn7 / (2.0 * assign71600_e108133)))))), (locals.var_vgpld_dn8 + (((((locals.var_fac1p2_dn8 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn8)) / 2.0) * assign71600_e108134) + (assign71600_e108130 * (-(locals.var_tx_dn8 / (2.0 * assign71600_e108133)))))), (locals.var_vgpld_dn9 + (((((locals.var_fac1p2_dn9 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn9)) / 2.0) * assign71600_e108134) + (assign71600_e108130 * (-(locals.var_tx_dn9 / (2.0 * assign71600_e108133)))))), (((((locals.var_fac1p2_dn10 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn10)) / 2.0) * assign71600_e108134) + (assign71600_e108130 * (-(locals.var_tx_dn10 / (2.0 * assign71600_e108133))))), (((((locals.var_fac1p2_dn11 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn11)) / 2.0) * assign71600_e108134) + (assign71600_e108130 * (-(locals.var_tx_dn11 / (2.0 * assign71600_e108133))))), (((((locals.var_fac1p2_dn14 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn14)) / 2.0) * assign71600_e108134) + (assign71600_e108130 * (-(locals.var_tx_dn14 / (2.0 * assign71600_e108133))))),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn14,)
    }
};
        locals.var_ps0_inia = assign71600_e108138;
        locals.var_ps0_inia_dn0 = assign71600_e108138_d_n0;
        locals.var_ps0_inia_dn2 = assign71600_e108138_d_n2;
        locals.var_ps0_inia_dn4 = assign71600_e108138_d_n4;
        locals.var_ps0_inia_dn5 = assign71600_e108138_d_n5;
        locals.var_ps0_inia_dn6 = assign71600_e108138_d_n6;
        locals.var_ps0_inia_dn7 = assign71600_e108138_d_n7;
        locals.var_ps0_inia_dn8 = assign71600_e108138_d_n8;
        locals.var_ps0_inia_dn9 = assign71600_e108138_d_n9;
        locals.var_ps0_inia_dn10 = assign71600_e108138_d_n10;
        locals.var_ps0_inia_dn11 = assign71600_e108138_d_n11;
        locals.var_ps0_inia_dn14 = assign71600_e108138_d_n14;

        let (assign71610_e108151, assign71610_e108151_d_n0, assign71610_e108151_d_n2, assign71610_e108151_d_n4, assign71610_e108151_d_n5, assign71610_e108151_d_n6, assign71610_e108151_d_n7, assign71610_e108151_d_n8, assign71610_e108151_d_n9, assign71610_e108151_d_n10, assign71610_e108151_d_n11, assign71610_e108151_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1669 == 0.0)) && (locals.var_guard1672 != 0.0)) {
        let assign71610_e108148: f64 = (locals.var_ps0_inia + locals.var_vxbgmtcl);
        let assign71610_e108149: f64 = (locals.var_beta * assign71610_e108148);
        (assign71610_e108149, ((locals.var_beta_dn0 * assign71610_e108148) + (locals.var_beta * (locals.var_ps0_inia_dn0 + locals.var_vxbgmtcl_dn0))), ((locals.var_beta_dn2 * assign71610_e108148) + (locals.var_beta * (locals.var_ps0_inia_dn2 + locals.var_vxbgmtcl_dn2))), ((locals.var_beta_dn4 * assign71610_e108148) + (locals.var_beta * (locals.var_ps0_inia_dn4 + locals.var_vxbgmtcl_dn4))), ((locals.var_beta_dn5 * assign71610_e108148) + (locals.var_beta * (locals.var_ps0_inia_dn5 + locals.var_vxbgmtcl_dn5))), ((locals.var_beta_dn6 * assign71610_e108148) + (locals.var_beta * (locals.var_ps0_inia_dn6 + locals.var_vxbgmtcl_dn6))), ((locals.var_beta_dn7 * assign71610_e108148) + (locals.var_beta * (locals.var_ps0_inia_dn7 + locals.var_vxbgmtcl_dn7))), ((locals.var_beta_dn8 * assign71610_e108148) + (locals.var_beta * (locals.var_ps0_inia_dn8 + locals.var_vxbgmtcl_dn8))), ((locals.var_beta_dn9 * assign71610_e108148) + (locals.var_beta * (locals.var_ps0_inia_dn9 + locals.var_vxbgmtcl_dn9))), ((locals.var_beta_dn10 * assign71610_e108148) + (locals.var_beta * (locals.var_ps0_inia_dn10 + locals.var_vxbgmtcl_dn10))), ((locals.var_beta_dn11 * assign71610_e108148) + (locals.var_beta * (locals.var_ps0_inia_dn11 + locals.var_vxbgmtcl_dn11))), ((locals.var_beta_dn14 * assign71610_e108148) + (locals.var_beta * (locals.var_ps0_inia_dn14 + locals.var_vxbgmtcl_dn14))),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
        locals.var_chi = assign71610_e108151;
        locals.var_chi_dn0 = assign71610_e108151_d_n0;
        locals.var_chi_dn2 = assign71610_e108151_d_n2;
        locals.var_chi_dn4 = assign71610_e108151_d_n4;
        locals.var_chi_dn5 = assign71610_e108151_d_n5;
        locals.var_chi_dn6 = assign71610_e108151_d_n6;
        locals.var_chi_dn7 = assign71610_e108151_d_n7;
        locals.var_chi_dn8 = assign71610_e108151_d_n8;
        locals.var_chi_dn9 = assign71610_e108151_d_n9;
        locals.var_chi_dn10 = assign71610_e108151_d_n10;
        locals.var_chi_dn11 = assign71610_e108151_d_n11;
        locals.var_chi_dn14 = assign71610_e108151_d_n14;

        let (assign71630_e108193,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1669 == 0.0)) && (locals.var_guard1672 == 0.0)) {
        let assign71630_e108172: f64 = (2.0_f64).sqrt();
        let assign71630_e108173: f64 = (9.0 * assign71630_e108172);
        let assign71630_e108174: f64 = (1.0 / assign71630_e108173);
        let assign71630_e108178: f64 = (-3.0);
        let assign71630_e108179: f64 = (assign71630_e108178).exp();
        let assign71630_e108180: f64 = (7.0 * assign71630_e108179);
        let assign71630_e108181: f64 = (5.0 + assign71630_e108180);
        let assign71630_e108185: f64 = (-3.0);
        let assign71630_e108186: f64 = (assign71630_e108185).exp();
        let assign71630_e108187: f64 = (2.0 + assign71630_e108186);
        let assign71630_e108188: f64 = (assign71630_e108187).sqrt();
        let assign71630_e108189: f64 = (54.0 * assign71630_e108188);
        let assign71630_e108190: f64 = (assign71630_e108181 / assign71630_e108189);
        let assign71630_e108191: f64 = (assign71630_e108174 - assign71630_e108190);
        (assign71630_e108191,)
    } else {
        (locals.var_ta,)
    }
};
        locals.var_ta = assign71630_e108193;

        let (assign71640_e108221,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1669 == 0.0)) && (locals.var_guard1672 == 0.0)) {
        let assign71640_e108203: f64 = (-3.0);
        let assign71640_e108204: f64 = (assign71640_e108203).exp();
        let assign71640_e108205: f64 = (1.0 + assign71640_e108204);
        let assign71640_e108209: f64 = (-3.0);
        let assign71640_e108210: f64 = (assign71640_e108209).exp();
        let assign71640_e108211: f64 = (2.0 + assign71640_e108210);
        let assign71640_e108212: f64 = (assign71640_e108211).sqrt();
        let assign71640_e108213: f64 = (2.0 * assign71640_e108212);
        let assign71640_e108214: f64 = (assign71640_e108205 / assign71640_e108213);
        let assign71640_e108216: f64 = (2.0_f64).sqrt();
        let assign71640_e108218: f64 = (assign71640_e108216 / 3.0);
        let assign71640_e108219: f64 = (assign71640_e108214 - assign71640_e108218);
        (assign71640_e108219,)
    } else {
        (locals.var_tb,)
    }
};
        locals.var_tb = assign71640_e108221;

        let (assign71650_e108240, assign71650_e108240_d_n0, assign71650_e108240_d_n2, assign71650_e108240_d_n4, assign71650_e108240_d_n5, assign71650_e108240_d_n6, assign71650_e108240_d_n7, assign71650_e108240_d_n8, assign71650_e108240_d_n9, assign71650_e108240_d_n10, assign71650_e108240_d_n11, assign71650_e108240_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1669 == 0.0)) && (locals.var_guard1672 == 0.0)) {
        let assign71650_e108231: f64 = (2.0_f64).sqrt();
        let assign71650_e108232: f64 = (1.0 / assign71650_e108231);
        let assign71650_e108236: f64 = (locals.var_beta * locals.var_fac1);
        let assign71650_e108237: f64 = (1.0 / assign71650_e108236);
        let assign71650_e108238: f64 = (assign71650_e108232 + assign71650_e108237);
        (assign71650_e108238, (-(((locals.var_beta_dn0 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn0)) / (assign71650_e108236 * assign71650_e108236))), (-(((locals.var_beta_dn2 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn2)) / (assign71650_e108236 * assign71650_e108236))), (-(((locals.var_beta_dn4 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn4)) / (assign71650_e108236 * assign71650_e108236))), (-(((locals.var_beta_dn5 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn5)) / (assign71650_e108236 * assign71650_e108236))), (-(((locals.var_beta_dn6 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn6)) / (assign71650_e108236 * assign71650_e108236))), (-(((locals.var_beta_dn7 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn7)) / (assign71650_e108236 * assign71650_e108236))), (-(((locals.var_beta_dn8 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn8)) / (assign71650_e108236 * assign71650_e108236))), (-(((locals.var_beta_dn9 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn9)) / (assign71650_e108236 * assign71650_e108236))), (-(((locals.var_beta_dn10 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn10)) / (assign71650_e108236 * assign71650_e108236))), (-(((locals.var_beta_dn11 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn11)) / (assign71650_e108236 * assign71650_e108236))), (-(((locals.var_beta_dn14 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn14)) / (assign71650_e108236 * assign71650_e108236))),)
    } else {
        (locals.var_tc, locals.var_tc_dn0, locals.var_tc_dn2, locals.var_tc_dn4, locals.var_tc_dn5, locals.var_tc_dn6, locals.var_tc_dn7, locals.var_tc_dn8, locals.var_tc_dn9, locals.var_tc_dn10, locals.var_tc_dn11, locals.var_tc_dn14,)
    }
};
        locals.var_tc = assign71650_e108240;
        locals.var_tc_dn0 = assign71650_e108240_d_n0;
        locals.var_tc_dn2 = assign71650_e108240_d_n2;
        locals.var_tc_dn4 = assign71650_e108240_d_n4;
        locals.var_tc_dn5 = assign71650_e108240_d_n5;
        locals.var_tc_dn6 = assign71650_e108240_d_n6;
        locals.var_tc_dn7 = assign71650_e108240_d_n7;
        locals.var_tc_dn8 = assign71650_e108240_d_n8;
        locals.var_tc_dn9 = assign71650_e108240_d_n9;
        locals.var_tc_dn10 = assign71650_e108240_d_n10;
        locals.var_tc_dn11 = assign71650_e108240_d_n11;
        locals.var_tc_dn14 = assign71650_e108240_d_n14;

        let (assign71660_e108255, assign71660_e108255_d_n0, assign71660_e108255_d_n2, assign71660_e108255_d_n4, assign71660_e108255_d_n5, assign71660_e108255_d_n6, assign71660_e108255_d_n7, assign71660_e108255_d_n8, assign71660_e108255_d_n9, assign71660_e108255_d_n10, assign71660_e108255_d_n11, assign71660_e108255_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1669 == 0.0)) && (locals.var_guard1672 == 0.0)) {
        let assign71660_e108250: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign71660_e108251: f64 = (-assign71660_e108250);
        let assign71660_e108253: f64 = (assign71660_e108251 / locals.var_fac1);
        (assign71660_e108253, ((((-locals.var_vxbgmtcl_dn0) * locals.var_fac1) - (assign71660_e108251 * locals.var_fac1_dn0)) / (locals.var_fac1 * locals.var_fac1)), ((((-(locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2)) * locals.var_fac1) - (assign71660_e108251 * locals.var_fac1_dn2)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn4) * locals.var_fac1) - (assign71660_e108251 * locals.var_fac1_dn4)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn5) * locals.var_fac1) - (assign71660_e108251 * locals.var_fac1_dn5)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn6) * locals.var_fac1) - (assign71660_e108251 * locals.var_fac1_dn6)) / (locals.var_fac1 * locals.var_fac1)), ((((-(locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7)) * locals.var_fac1) - (assign71660_e108251 * locals.var_fac1_dn7)) / (locals.var_fac1 * locals.var_fac1)), ((((-(locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8)) * locals.var_fac1) - (assign71660_e108251 * locals.var_fac1_dn8)) / (locals.var_fac1 * locals.var_fac1)), ((((-(locals.var_vgpld_dn9 + locals.var_vxbgmtcl_dn9)) * locals.var_fac1) - (assign71660_e108251 * locals.var_fac1_dn9)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn10) * locals.var_fac1) - (assign71660_e108251 * locals.var_fac1_dn10)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn11) * locals.var_fac1) - (assign71660_e108251 * locals.var_fac1_dn11)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn14) * locals.var_fac1) - (assign71660_e108251 * locals.var_fac1_dn14)) / (locals.var_fac1 * locals.var_fac1)),)
    } else {
        (locals.var_td, locals.var_td_dn0, locals.var_td_dn2, locals.var_td_dn4, locals.var_td_dn5, locals.var_td_dn6, locals.var_td_dn7, locals.var_td_dn8, locals.var_td_dn9, locals.var_td_dn10, locals.var_td_dn11, locals.var_td_dn14,)
    }
};
        locals.var_td = assign71660_e108255;
        locals.var_td_dn0 = assign71660_e108255_d_n0;
        locals.var_td_dn2 = assign71660_e108255_d_n2;
        locals.var_td_dn4 = assign71660_e108255_d_n4;
        locals.var_td_dn5 = assign71660_e108255_d_n5;
        locals.var_td_dn6 = assign71660_e108255_d_n6;
        locals.var_td_dn7 = assign71660_e108255_d_n7;
        locals.var_td_dn8 = assign71660_e108255_d_n8;
        locals.var_td_dn9 = assign71660_e108255_d_n9;
        locals.var_td_dn10 = assign71660_e108255_d_n10;
        locals.var_td_dn11 = assign71660_e108255_d_n11;
        locals.var_td_dn14 = assign71660_e108255_d_n14;

        let (assign71670_e108293, assign71670_e108293_d_n0, assign71670_e108293_d_n2, assign71670_e108293_d_n4, assign71670_e108293_d_n5, assign71670_e108293_d_n6, assign71670_e108293_d_n7, assign71670_e108293_d_n8, assign71670_e108293_d_n9, assign71670_e108293_d_n10, assign71670_e108293_d_n11, assign71670_e108293_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1669 == 0.0)) && (locals.var_guard1672 == 0.0)) {
        let assign71670_e108265: f64 = (locals.var_tb * locals.var_tb);
        let assign71670_e108267: f64 = (assign71670_e108265 * locals.var_tb);
        let assign71670_e108270: f64 = (27.0 * locals.var_ta);
        let assign71670_e108272: f64 = (assign71670_e108270 * locals.var_ta);
        let assign71670_e108274: f64 = (assign71670_e108272 * locals.var_ta);
        let assign71670_e108275: f64 = (assign71670_e108267 / assign71670_e108274);
        let assign71670_e108278: f64 = (locals.var_tb * locals.var_tc);
        let assign71670_e108281: f64 = (6.0 * locals.var_ta);
        let assign71670_e108283: f64 = (assign71670_e108281 * locals.var_ta);
        let assign71670_e108284: f64 = (assign71670_e108278 / assign71670_e108283);
        let assign71670_e108285: f64 = (assign71670_e108275 - assign71670_e108284);
        let assign71670_e108289: f64 = (2.0 * locals.var_ta);
        let assign71670_e108290: f64 = (locals.var_td / assign71670_e108289);
        let assign71670_e108291: f64 = (assign71670_e108285 + assign71670_e108290);
        (assign71670_e108291, ((-((locals.var_tb * locals.var_tc_dn0) / assign71670_e108283)) + (locals.var_td_dn0 / assign71670_e108289)), ((-((locals.var_tb * locals.var_tc_dn2) / assign71670_e108283)) + (locals.var_td_dn2 / assign71670_e108289)), ((-((locals.var_tb * locals.var_tc_dn4) / assign71670_e108283)) + (locals.var_td_dn4 / assign71670_e108289)), ((-((locals.var_tb * locals.var_tc_dn5) / assign71670_e108283)) + (locals.var_td_dn5 / assign71670_e108289)), ((-((locals.var_tb * locals.var_tc_dn6) / assign71670_e108283)) + (locals.var_td_dn6 / assign71670_e108289)), ((-((locals.var_tb * locals.var_tc_dn7) / assign71670_e108283)) + (locals.var_td_dn7 / assign71670_e108289)), ((-((locals.var_tb * locals.var_tc_dn8) / assign71670_e108283)) + (locals.var_td_dn8 / assign71670_e108289)), ((-((locals.var_tb * locals.var_tc_dn9) / assign71670_e108283)) + (locals.var_td_dn9 / assign71670_e108289)), ((-((locals.var_tb * locals.var_tc_dn10) / assign71670_e108283)) + (locals.var_td_dn10 / assign71670_e108289)), ((-((locals.var_tb * locals.var_tc_dn11) / assign71670_e108283)) + (locals.var_td_dn11 / assign71670_e108289)), ((-((locals.var_tb * locals.var_tc_dn14) / assign71670_e108283)) + (locals.var_td_dn14 / assign71670_e108289)),)
    } else {
        (locals.var_tq, locals.var_tq_dn0, locals.var_tq_dn2, locals.var_tq_dn4, locals.var_tq_dn5, locals.var_tq_dn6, locals.var_tq_dn7, locals.var_tq_dn8, locals.var_tq_dn9, locals.var_tq_dn10, locals.var_tq_dn11, locals.var_tq_dn14,)
    }
};
        locals.var_tq = assign71670_e108293;
        locals.var_tq_dn0 = assign71670_e108293_d_n0;
        locals.var_tq_dn2 = assign71670_e108293_d_n2;
        locals.var_tq_dn4 = assign71670_e108293_d_n4;
        locals.var_tq_dn5 = assign71670_e108293_d_n5;
        locals.var_tq_dn6 = assign71670_e108293_d_n6;
        locals.var_tq_dn7 = assign71670_e108293_d_n7;
        locals.var_tq_dn8 = assign71670_e108293_d_n8;
        locals.var_tq_dn9 = assign71670_e108293_d_n9;
        locals.var_tq_dn10 = assign71670_e108293_d_n10;
        locals.var_tq_dn11 = assign71670_e108293_d_n11;
        locals.var_tq_dn14 = assign71670_e108293_d_n14;

        let (assign71680_e108317, assign71680_e108317_d_n0, assign71680_e108317_d_n2, assign71680_e108317_d_n4, assign71680_e108317_d_n5, assign71680_e108317_d_n6, assign71680_e108317_d_n7, assign71680_e108317_d_n8, assign71680_e108317_d_n9, assign71680_e108317_d_n10, assign71680_e108317_d_n11, assign71680_e108317_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1669 == 0.0)) && (locals.var_guard1672 == 0.0)) {
        let assign71680_e108303: f64 = (3.0 * locals.var_ta);
        let assign71680_e108305: f64 = (assign71680_e108303 * locals.var_tc);
        let assign71680_e108308: f64 = (locals.var_tb * locals.var_tb);
        let assign71680_e108309: f64 = (assign71680_e108305 - assign71680_e108308);
        let assign71680_e108312: f64 = (9.0 * locals.var_ta);
        let assign71680_e108314: f64 = (assign71680_e108312 * locals.var_ta);
        let assign71680_e108315: f64 = (assign71680_e108309 / assign71680_e108314);
        (assign71680_e108315, ((assign71680_e108303 * locals.var_tc_dn0) / assign71680_e108314), ((assign71680_e108303 * locals.var_tc_dn2) / assign71680_e108314), ((assign71680_e108303 * locals.var_tc_dn4) / assign71680_e108314), ((assign71680_e108303 * locals.var_tc_dn5) / assign71680_e108314), ((assign71680_e108303 * locals.var_tc_dn6) / assign71680_e108314), ((assign71680_e108303 * locals.var_tc_dn7) / assign71680_e108314), ((assign71680_e108303 * locals.var_tc_dn8) / assign71680_e108314), ((assign71680_e108303 * locals.var_tc_dn9) / assign71680_e108314), ((assign71680_e108303 * locals.var_tc_dn10) / assign71680_e108314), ((assign71680_e108303 * locals.var_tc_dn11) / assign71680_e108314), ((assign71680_e108303 * locals.var_tc_dn14) / assign71680_e108314),)
    } else {
        (locals.var_tp, locals.var_tp_dn0, locals.var_tp_dn2, locals.var_tp_dn4, locals.var_tp_dn5, locals.var_tp_dn6, locals.var_tp_dn7, locals.var_tp_dn8, locals.var_tp_dn9, locals.var_tp_dn10, locals.var_tp_dn11, locals.var_tp_dn14,)
    }
};
        locals.var_tp = assign71680_e108317;
        locals.var_tp_dn0 = assign71680_e108317_d_n0;
        locals.var_tp_dn2 = assign71680_e108317_d_n2;
        locals.var_tp_dn4 = assign71680_e108317_d_n4;
        locals.var_tp_dn5 = assign71680_e108317_d_n5;
        locals.var_tp_dn6 = assign71680_e108317_d_n6;
        locals.var_tp_dn7 = assign71680_e108317_d_n7;
        locals.var_tp_dn8 = assign71680_e108317_d_n8;
        locals.var_tp_dn9 = assign71680_e108317_d_n9;
        locals.var_tp_dn10 = assign71680_e108317_d_n10;
        locals.var_tp_dn11 = assign71680_e108317_d_n11;
        locals.var_tp_dn14 = assign71680_e108317_d_n14;

        let (assign71690_e108336, assign71690_e108336_d_n0, assign71690_e108336_d_n2, assign71690_e108336_d_n4, assign71690_e108336_d_n5, assign71690_e108336_d_n6, assign71690_e108336_d_n7, assign71690_e108336_d_n8, assign71690_e108336_d_n9, assign71690_e108336_d_n10, assign71690_e108336_d_n11, assign71690_e108336_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1669 == 0.0)) && (locals.var_guard1672 == 0.0)) {
        let assign71690_e108327: f64 = (locals.var_tq * locals.var_tq);
        let assign71690_e108330: f64 = (locals.var_tp * locals.var_tp);
        let assign71690_e108332: f64 = (assign71690_e108330 * locals.var_tp);
        let assign71690_e108333: f64 = (assign71690_e108327 + assign71690_e108332);
        let assign71690_e108334: f64 = (assign71690_e108333).sqrt();
        (assign71690_e108334, ((((locals.var_tq_dn0 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn0)) + ((((locals.var_tp_dn0 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn0)) * locals.var_tp) + (assign71690_e108330 * locals.var_tp_dn0))) / (2.0 * assign71690_e108334)), ((((locals.var_tq_dn2 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn2)) + ((((locals.var_tp_dn2 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn2)) * locals.var_tp) + (assign71690_e108330 * locals.var_tp_dn2))) / (2.0 * assign71690_e108334)), ((((locals.var_tq_dn4 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn4)) + ((((locals.var_tp_dn4 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn4)) * locals.var_tp) + (assign71690_e108330 * locals.var_tp_dn4))) / (2.0 * assign71690_e108334)), ((((locals.var_tq_dn5 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn5)) + ((((locals.var_tp_dn5 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn5)) * locals.var_tp) + (assign71690_e108330 * locals.var_tp_dn5))) / (2.0 * assign71690_e108334)), ((((locals.var_tq_dn6 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn6)) + ((((locals.var_tp_dn6 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn6)) * locals.var_tp) + (assign71690_e108330 * locals.var_tp_dn6))) / (2.0 * assign71690_e108334)), ((((locals.var_tq_dn7 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn7)) + ((((locals.var_tp_dn7 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn7)) * locals.var_tp) + (assign71690_e108330 * locals.var_tp_dn7))) / (2.0 * assign71690_e108334)), ((((locals.var_tq_dn8 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn8)) + ((((locals.var_tp_dn8 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn8)) * locals.var_tp) + (assign71690_e108330 * locals.var_tp_dn8))) / (2.0 * assign71690_e108334)), ((((locals.var_tq_dn9 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn9)) + ((((locals.var_tp_dn9 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn9)) * locals.var_tp) + (assign71690_e108330 * locals.var_tp_dn9))) / (2.0 * assign71690_e108334)), ((((locals.var_tq_dn10 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn10)) + ((((locals.var_tp_dn10 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn10)) * locals.var_tp) + (assign71690_e108330 * locals.var_tp_dn10))) / (2.0 * assign71690_e108334)), ((((locals.var_tq_dn11 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn11)) + ((((locals.var_tp_dn11 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn11)) * locals.var_tp) + (assign71690_e108330 * locals.var_tp_dn11))) / (2.0 * assign71690_e108334)), ((((locals.var_tq_dn14 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn14)) + ((((locals.var_tp_dn14 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn14)) * locals.var_tp) + (assign71690_e108330 * locals.var_tp_dn14))) / (2.0 * assign71690_e108334)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign71690_e108336;
        locals.var_t5_dn0 = assign71690_e108336_d_n0;
        locals.var_t5_dn2 = assign71690_e108336_d_n2;
        locals.var_t5_dn4 = assign71690_e108336_d_n4;
        locals.var_t5_dn5 = assign71690_e108336_d_n5;
        locals.var_t5_dn6 = assign71690_e108336_d_n6;
        locals.var_t5_dn7 = assign71690_e108336_d_n7;
        locals.var_t5_dn8 = assign71690_e108336_d_n8;
        locals.var_t5_dn9 = assign71690_e108336_d_n9;
        locals.var_t5_dn10 = assign71690_e108336_d_n10;
        locals.var_t5_dn11 = assign71690_e108336_d_n11;
        locals.var_t5_dn14 = assign71690_e108336_d_n14;

        let (assign71700_e108351, assign71700_e108351_d_n0, assign71700_e108351_d_n2, assign71700_e108351_d_n4, assign71700_e108351_d_n5, assign71700_e108351_d_n6, assign71700_e108351_d_n7, assign71700_e108351_d_n8, assign71700_e108351_d_n9, assign71700_e108351_d_n10, assign71700_e108351_d_n11, assign71700_e108351_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1669 == 0.0)) && (locals.var_guard1672 == 0.0)) {
        let assign71700_e108345: f64 = (-locals.var_tq);
        let assign71700_e108347: f64 = (assign71700_e108345 + locals.var_t5);
        let assign71700_e108349: f64 = (assign71700_e108347).powf(0.3333333333333333);
        (assign71700_e108349, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign71700_e108347).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn0) + locals.var_t5_dn0))) } } else { (assign71700_e108349 * (0.3333333333333333 * (((-locals.var_tq_dn0) + locals.var_t5_dn0) / assign71700_e108347))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign71700_e108347).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn2) + locals.var_t5_dn2))) } } else { (assign71700_e108349 * (0.3333333333333333 * (((-locals.var_tq_dn2) + locals.var_t5_dn2) / assign71700_e108347))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign71700_e108347).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn4) + locals.var_t5_dn4))) } } else { (assign71700_e108349 * (0.3333333333333333 * (((-locals.var_tq_dn4) + locals.var_t5_dn4) / assign71700_e108347))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign71700_e108347).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn5) + locals.var_t5_dn5))) } } else { (assign71700_e108349 * (0.3333333333333333 * (((-locals.var_tq_dn5) + locals.var_t5_dn5) / assign71700_e108347))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign71700_e108347).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn6) + locals.var_t5_dn6))) } } else { (assign71700_e108349 * (0.3333333333333333 * (((-locals.var_tq_dn6) + locals.var_t5_dn6) / assign71700_e108347))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign71700_e108347).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn7) + locals.var_t5_dn7))) } } else { (assign71700_e108349 * (0.3333333333333333 * (((-locals.var_tq_dn7) + locals.var_t5_dn7) / assign71700_e108347))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign71700_e108347).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn8) + locals.var_t5_dn8))) } } else { (assign71700_e108349 * (0.3333333333333333 * (((-locals.var_tq_dn8) + locals.var_t5_dn8) / assign71700_e108347))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign71700_e108347).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn9) + locals.var_t5_dn9))) } } else { (assign71700_e108349 * (0.3333333333333333 * (((-locals.var_tq_dn9) + locals.var_t5_dn9) / assign71700_e108347))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign71700_e108347).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn10) + locals.var_t5_dn10))) } } else { (assign71700_e108349 * (0.3333333333333333 * (((-locals.var_tq_dn10) + locals.var_t5_dn10) / assign71700_e108347))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign71700_e108347).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn11) + locals.var_t5_dn11))) } } else { (assign71700_e108349 * (0.3333333333333333 * (((-locals.var_tq_dn11) + locals.var_t5_dn11) / assign71700_e108347))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign71700_e108347).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn14) + locals.var_t5_dn14))) } } else { (assign71700_e108349 * (0.3333333333333333 * (((-locals.var_tq_dn14) + locals.var_t5_dn14) / assign71700_e108347))) },)
    } else {
        (locals.var_tu, locals.var_tu_dn0, locals.var_tu_dn2, locals.var_tu_dn4, locals.var_tu_dn5, locals.var_tu_dn6, locals.var_tu_dn7, locals.var_tu_dn8, locals.var_tu_dn9, locals.var_tu_dn10, locals.var_tu_dn11, locals.var_tu_dn14,)
    }
};
        locals.var_tu = assign71700_e108351;
        locals.var_tu_dn0 = assign71700_e108351_d_n0;
        locals.var_tu_dn2 = assign71700_e108351_d_n2;
        locals.var_tu_dn4 = assign71700_e108351_d_n4;
        locals.var_tu_dn5 = assign71700_e108351_d_n5;
        locals.var_tu_dn6 = assign71700_e108351_d_n6;
        locals.var_tu_dn7 = assign71700_e108351_d_n7;
        locals.var_tu_dn8 = assign71700_e108351_d_n8;
        locals.var_tu_dn9 = assign71700_e108351_d_n9;
        locals.var_tu_dn10 = assign71700_e108351_d_n10;
        locals.var_tu_dn11 = assign71700_e108351_d_n11;
        locals.var_tu_dn14 = assign71700_e108351_d_n14;

        let (assign71710_e108366, assign71710_e108366_d_n0, assign71710_e108366_d_n2, assign71710_e108366_d_n4, assign71710_e108366_d_n5, assign71710_e108366_d_n6, assign71710_e108366_d_n7, assign71710_e108366_d_n8, assign71710_e108366_d_n9, assign71710_e108366_d_n10, assign71710_e108366_d_n11, assign71710_e108366_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1669 == 0.0)) && (locals.var_guard1672 == 0.0)) {
        let assign71710_e108361: f64 = (locals.var_tq + locals.var_t5);
        let assign71710_e108363: f64 = (assign71710_e108361).powf(0.3333333333333333);
        let assign71710_e108364: f64 = (-assign71710_e108363);
        (assign71710_e108364, (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign71710_e108361).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn0 + locals.var_t5_dn0))) } } else { (assign71710_e108363 * (0.3333333333333333 * ((locals.var_tq_dn0 + locals.var_t5_dn0) / assign71710_e108361))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign71710_e108361).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn2 + locals.var_t5_dn2))) } } else { (assign71710_e108363 * (0.3333333333333333 * ((locals.var_tq_dn2 + locals.var_t5_dn2) / assign71710_e108361))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign71710_e108361).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn4 + locals.var_t5_dn4))) } } else { (assign71710_e108363 * (0.3333333333333333 * ((locals.var_tq_dn4 + locals.var_t5_dn4) / assign71710_e108361))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign71710_e108361).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn5 + locals.var_t5_dn5))) } } else { (assign71710_e108363 * (0.3333333333333333 * ((locals.var_tq_dn5 + locals.var_t5_dn5) / assign71710_e108361))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign71710_e108361).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn6 + locals.var_t5_dn6))) } } else { (assign71710_e108363 * (0.3333333333333333 * ((locals.var_tq_dn6 + locals.var_t5_dn6) / assign71710_e108361))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign71710_e108361).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn7 + locals.var_t5_dn7))) } } else { (assign71710_e108363 * (0.3333333333333333 * ((locals.var_tq_dn7 + locals.var_t5_dn7) / assign71710_e108361))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign71710_e108361).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn8 + locals.var_t5_dn8))) } } else { (assign71710_e108363 * (0.3333333333333333 * ((locals.var_tq_dn8 + locals.var_t5_dn8) / assign71710_e108361))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign71710_e108361).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn9 + locals.var_t5_dn9))) } } else { (assign71710_e108363 * (0.3333333333333333 * ((locals.var_tq_dn9 + locals.var_t5_dn9) / assign71710_e108361))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign71710_e108361).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn10 + locals.var_t5_dn10))) } } else { (assign71710_e108363 * (0.3333333333333333 * ((locals.var_tq_dn10 + locals.var_t5_dn10) / assign71710_e108361))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign71710_e108361).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn11 + locals.var_t5_dn11))) } } else { (assign71710_e108363 * (0.3333333333333333 * ((locals.var_tq_dn11 + locals.var_t5_dn11) / assign71710_e108361))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign71710_e108361).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn14 + locals.var_t5_dn14))) } } else { (assign71710_e108363 * (0.3333333333333333 * ((locals.var_tq_dn14 + locals.var_t5_dn14) / assign71710_e108361))) }),)
    } else {
        (locals.var_tv, locals.var_tv_dn0, locals.var_tv_dn2, locals.var_tv_dn4, locals.var_tv_dn5, locals.var_tv_dn6, locals.var_tv_dn7, locals.var_tv_dn8, locals.var_tv_dn9, locals.var_tv_dn10, locals.var_tv_dn11, locals.var_tv_dn14,)
    }
};
        locals.var_tv = assign71710_e108366;
        locals.var_tv_dn0 = assign71710_e108366_d_n0;
        locals.var_tv_dn2 = assign71710_e108366_d_n2;
        locals.var_tv_dn4 = assign71710_e108366_d_n4;
        locals.var_tv_dn5 = assign71710_e108366_d_n5;
        locals.var_tv_dn6 = assign71710_e108366_d_n6;
        locals.var_tv_dn7 = assign71710_e108366_d_n7;
        locals.var_tv_dn8 = assign71710_e108366_d_n8;
        locals.var_tv_dn9 = assign71710_e108366_d_n9;
        locals.var_tv_dn10 = assign71710_e108366_d_n10;
        locals.var_tv_dn11 = assign71710_e108366_d_n11;
        locals.var_tv_dn14 = assign71710_e108366_d_n14;

    }
}
