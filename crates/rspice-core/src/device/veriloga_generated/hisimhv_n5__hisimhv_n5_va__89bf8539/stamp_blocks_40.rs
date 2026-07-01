#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_251(
        locals: &mut StampLocals,
    ) {
        let (assign66650_e103326, assign66650_e103326_d_n0, assign66650_e103326_d_n2, assign66650_e103326_d_n4, assign66650_e103326_d_n5, assign66650_e103326_d_n6, assign66650_e103326_d_n7, assign66650_e103326_d_n8, assign66650_e103326_d_n9, assign66650_e103326_d_n10, assign66650_e103326_d_n11, assign66650_e103326_d_n14,) = {
    if (((locals.var_guard1582 == 0.0) && (locals.var_guard1583 != 0.0)) && (locals.var_guard1584 != 0.0)) {
        let assign66650_e103324: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign66650_e103324, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign66650_e103326;
        locals.var_xmp_dn0 = assign66650_e103326_d_n0;
        locals.var_xmp_dn2 = assign66650_e103326_d_n2;
        locals.var_xmp_dn4 = assign66650_e103326_d_n4;
        locals.var_xmp_dn5 = assign66650_e103326_d_n5;
        locals.var_xmp_dn6 = assign66650_e103326_d_n6;
        locals.var_xmp_dn7 = assign66650_e103326_d_n7;
        locals.var_xmp_dn8 = assign66650_e103326_d_n8;
        locals.var_xmp_dn9 = assign66650_e103326_d_n9;
        locals.var_xmp_dn10 = assign66650_e103326_d_n10;
        locals.var_xmp_dn11 = assign66650_e103326_d_n11;
        locals.var_xmp_dn14 = assign66650_e103326_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign66660_e103337, assign66660_e103337_d_n0, assign66660_e103337_d_n2, assign66660_e103337_d_n4, assign66660_e103337_d_n5, assign66660_e103337_d_n6, assign66660_e103337_d_n7, assign66660_e103337_d_n8, assign66660_e103337_d_n9, assign66660_e103337_d_n10, assign66660_e103337_d_n11, assign66660_e103337_d_n14,) = {
    if (((locals.var_guard1582 == 0.0) && (locals.var_guard1583 != 0.0)) && (locals.var_guard1584 != 0.0)) {
        let assign66660_e103335: f64 = (locals.var_xp * locals.var_x2);
        (assign66660_e103335, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign66660_e103337;
        locals.var_xp_dn0 = assign66660_e103337_d_n0;
        locals.var_xp_dn2 = assign66660_e103337_d_n2;
        locals.var_xp_dn4 = assign66660_e103337_d_n4;
        locals.var_xp_dn5 = assign66660_e103337_d_n5;
        locals.var_xp_dn6 = assign66660_e103337_d_n6;
        locals.var_xp_dn7 = assign66660_e103337_d_n7;
        locals.var_xp_dn8 = assign66660_e103337_d_n8;
        locals.var_xp_dn9 = assign66660_e103337_d_n9;
        locals.var_xp_dn10 = assign66660_e103337_d_n10;
        locals.var_xp_dn11 = assign66660_e103337_d_n11;
        locals.var_xp_dn14 = assign66660_e103337_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign66670_e103348, assign66670_e103348_d_n0, assign66670_e103348_d_n2, assign66670_e103348_d_n4, assign66670_e103348_d_n5, assign66670_e103348_d_n6, assign66670_e103348_d_n7, assign66670_e103348_d_n8, assign66670_e103348_d_n9, assign66670_e103348_d_n10, assign66670_e103348_d_n11, assign66670_e103348_d_n14,) = {
    if (((locals.var_guard1582 == 0.0) && (locals.var_guard1583 != 0.0)) && (locals.var_guard1584 != 0.0)) {
        let assign66670_e103346: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign66670_e103346, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign66670_e103348;
        locals.var_xmp_dn0 = assign66670_e103348_d_n0;
        locals.var_xmp_dn2 = assign66670_e103348_d_n2;
        locals.var_xmp_dn4 = assign66670_e103348_d_n4;
        locals.var_xmp_dn5 = assign66670_e103348_d_n5;
        locals.var_xmp_dn6 = assign66670_e103348_d_n6;
        locals.var_xmp_dn7 = assign66670_e103348_d_n7;
        locals.var_xmp_dn8 = assign66670_e103348_d_n8;
        locals.var_xmp_dn9 = assign66670_e103348_d_n9;
        locals.var_xmp_dn10 = assign66670_e103348_d_n10;
        locals.var_xmp_dn11 = assign66670_e103348_d_n11;
        locals.var_xmp_dn14 = assign66670_e103348_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign66680_e103359, assign66680_e103359_d_n0, assign66680_e103359_d_n2, assign66680_e103359_d_n4, assign66680_e103359_d_n5, assign66680_e103359_d_n6, assign66680_e103359_d_n7, assign66680_e103359_d_n8, assign66680_e103359_d_n9, assign66680_e103359_d_n10, assign66680_e103359_d_n11, assign66680_e103359_d_n14,) = {
    if (((locals.var_guard1582 == 0.0) && (locals.var_guard1583 != 0.0)) && (locals.var_guard1584 != 0.0)) {
        let assign66680_e103357: f64 = (locals.var_xp * locals.var_x2);
        (assign66680_e103357, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign66680_e103359;
        locals.var_xp_dn0 = assign66680_e103359_d_n0;
        locals.var_xp_dn2 = assign66680_e103359_d_n2;
        locals.var_xp_dn4 = assign66680_e103359_d_n4;
        locals.var_xp_dn5 = assign66680_e103359_d_n5;
        locals.var_xp_dn6 = assign66680_e103359_d_n6;
        locals.var_xp_dn7 = assign66680_e103359_d_n7;
        locals.var_xp_dn8 = assign66680_e103359_d_n8;
        locals.var_xp_dn9 = assign66680_e103359_d_n9;
        locals.var_xp_dn10 = assign66680_e103359_d_n10;
        locals.var_xp_dn11 = assign66680_e103359_d_n11;
        locals.var_xp_dn14 = assign66680_e103359_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign66690_e103370, assign66690_e103370_d_n0, assign66690_e103370_d_n2, assign66690_e103370_d_n4, assign66690_e103370_d_n5, assign66690_e103370_d_n6, assign66690_e103370_d_n7, assign66690_e103370_d_n8, assign66690_e103370_d_n9, assign66690_e103370_d_n10, assign66690_e103370_d_n11, assign66690_e103370_d_n14,) = {
    if (((locals.var_guard1582 == 0.0) && (locals.var_guard1583 != 0.0)) && (locals.var_guard1584 != 0.0)) {
        let assign66690_e103368: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign66690_e103368, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign66690_e103370;
        locals.var_xmp_dn0 = assign66690_e103370_d_n0;
        locals.var_xmp_dn2 = assign66690_e103370_d_n2;
        locals.var_xmp_dn4 = assign66690_e103370_d_n4;
        locals.var_xmp_dn5 = assign66690_e103370_d_n5;
        locals.var_xmp_dn6 = assign66690_e103370_d_n6;
        locals.var_xmp_dn7 = assign66690_e103370_d_n7;
        locals.var_xmp_dn8 = assign66690_e103370_d_n8;
        locals.var_xmp_dn9 = assign66690_e103370_d_n9;
        locals.var_xmp_dn10 = assign66690_e103370_d_n10;
        locals.var_xmp_dn11 = assign66690_e103370_d_n11;
        locals.var_xmp_dn14 = assign66690_e103370_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign66700_e103381, assign66700_e103381_d_n0, assign66700_e103381_d_n2, assign66700_e103381_d_n4, assign66700_e103381_d_n5, assign66700_e103381_d_n6, assign66700_e103381_d_n7, assign66700_e103381_d_n8, assign66700_e103381_d_n9, assign66700_e103381_d_n10, assign66700_e103381_d_n11, assign66700_e103381_d_n14,) = {
    if (((locals.var_guard1582 == 0.0) && (locals.var_guard1583 != 0.0)) && (locals.var_guard1584 != 0.0)) {
        let assign66700_e103379: f64 = (locals.var_xp + locals.var_xmp);
        (assign66700_e103379, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign66700_e103381;
        locals.var_arg_dn0 = assign66700_e103381_d_n0;
        locals.var_arg_dn2 = assign66700_e103381_d_n2;
        locals.var_arg_dn4 = assign66700_e103381_d_n4;
        locals.var_arg_dn5 = assign66700_e103381_d_n5;
        locals.var_arg_dn6 = assign66700_e103381_d_n6;
        locals.var_arg_dn7 = assign66700_e103381_d_n7;
        locals.var_arg_dn8 = assign66700_e103381_d_n8;
        locals.var_arg_dn9 = assign66700_e103381_d_n9;
        locals.var_arg_dn10 = assign66700_e103381_d_n10;
        locals.var_arg_dn11 = assign66700_e103381_d_n11;
        locals.var_arg_dn14 = assign66700_e103381_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign66710_e103390, assign66710_e103390_d_n0, assign66710_e103390_d_n2, assign66710_e103390_d_n4, assign66710_e103390_d_n5, assign66710_e103390_d_n6, assign66710_e103390_d_n7, assign66710_e103390_d_n8, assign66710_e103390_d_n9, assign66710_e103390_d_n10, assign66710_e103390_d_n11, assign66710_e103390_d_n14,) = {
    if (((locals.var_guard1582 == 0.0) && (locals.var_guard1583 != 0.0)) && (locals.var_guard1584 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign66710_e103390;
        locals.var_dnm_dn0 = assign66710_e103390_d_n0;
        locals.var_dnm_dn2 = assign66710_e103390_d_n2;
        locals.var_dnm_dn4 = assign66710_e103390_d_n4;
        locals.var_dnm_dn5 = assign66710_e103390_d_n5;
        locals.var_dnm_dn6 = assign66710_e103390_d_n6;
        locals.var_dnm_dn7 = assign66710_e103390_d_n7;
        locals.var_dnm_dn8 = assign66710_e103390_d_n8;
        locals.var_dnm_dn9 = assign66710_e103390_d_n9;
        locals.var_dnm_dn10 = assign66710_e103390_d_n10;
        locals.var_dnm_dn11 = assign66710_e103390_d_n11;
        locals.var_dnm_dn14 = assign66710_e103390_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign66720_e103405: f64 = if ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1585 = assign66720_e103405;
        locals.var_guard1585_rv = 0.0;

        let assign66730_e103408: f64 = if 4.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1586 = assign66730_e103408;
        locals.var_guard1586_rv = 0.0;

        let (assign66740_e103421,) = {
    if (((((locals.var_guard1582 == 0.0) && (locals.var_guard1583 != 0.0)) && (locals.var_guard1584 != 0.0)) && (locals.var_guard1585 != 0.0)) && (locals.var_guard1586 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign66740_e103421;
        locals.var_mm_rv = 0.0;

        let assign66750_e103424: f64 = if 4.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1587 = assign66750_e103424;
        locals.var_guard1587_rv = 0.0;

        let (assign66760_e103440,) = {
    if ((((((locals.var_guard1582 == 0.0) && (locals.var_guard1583 != 0.0)) && (locals.var_guard1584 != 0.0)) && (locals.var_guard1585 != 0.0)) && (locals.var_guard1586 == 0.0)) && (locals.var_guard1587 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign66760_e103440;
        locals.var_mm_rv = 0.0;

        let assign66770_e103443: f64 = if 4.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1588 = assign66770_e103443;
        locals.var_guard1588_rv = 0.0;

        let (assign66780_e103462,) = {
    if (((((((locals.var_guard1582 == 0.0) && (locals.var_guard1583 != 0.0)) && (locals.var_guard1584 != 0.0)) && (locals.var_guard1585 != 0.0)) && (locals.var_guard1586 == 0.0)) && (locals.var_guard1587 == 0.0)) && (locals.var_guard1588 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign66780_e103462;
        locals.var_mm_rv = 0.0;

        let assign66790_e103465: f64 = if 4.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1589 = assign66790_e103465;
        locals.var_guard1589_rv = 0.0;

        let (assign66800_e103487,) = {
    if ((((((((locals.var_guard1582 == 0.0) && (locals.var_guard1583 != 0.0)) && (locals.var_guard1584 != 0.0)) && (locals.var_guard1585 != 0.0)) && (locals.var_guard1586 == 0.0)) && (locals.var_guard1587 == 0.0)) && (locals.var_guard1588 == 0.0)) && (locals.var_guard1589 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign66800_e103487;
        locals.var_mm_rv = 0.0;

        let (assign66810_e103498,) = {
    if ((((locals.var_guard1582 == 0.0) && (locals.var_guard1583 != 0.0)) && (locals.var_guard1584 != 0.0)) && (locals.var_guard1585 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign66810_e103498;
        locals.var_m0_rv = 0.0;

        let mut assign66820_loop_guard: usize = 0;
        while {
            let assign66820_cond_e103510: f64 = if (((((locals.var_guard1582 == 0.0) && (locals.var_guard1583 != 0.0)) && (locals.var_guard1584 != 0.0)) && (locals.var_guard1585 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign66820_cond_e103510 != 0.0
        } {
            assign66820_loop_guard += 1;
            assert!(assign66820_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign66820_body0_e103522, assign66820_body0_e103522_d_n0, assign66820_body0_e103522_d_n2, assign66820_body0_e103522_d_n4, assign66820_body0_e103522_d_n5, assign66820_body0_e103522_d_n6, assign66820_body0_e103522_d_n7, assign66820_body0_e103522_d_n8, assign66820_body0_e103522_d_n9, assign66820_body0_e103522_d_n10, assign66820_body0_e103522_d_n11, assign66820_body0_e103522_d_n14,) = {
    if ((((locals.var_guard1582 == 0.0) && (locals.var_guard1583 != 0.0)) && (locals.var_guard1584 != 0.0)) && (locals.var_guard1585 != 0.0)) {
        let assign66820_body0_e103520: f64 = (locals.var_dnm).sqrt();
        (assign66820_body0_e103520, (locals.var_dnm_dn0 / (2.0 * assign66820_body0_e103520)), (locals.var_dnm_dn2 / (2.0 * assign66820_body0_e103520)), (locals.var_dnm_dn4 / (2.0 * assign66820_body0_e103520)), (locals.var_dnm_dn5 / (2.0 * assign66820_body0_e103520)), (locals.var_dnm_dn6 / (2.0 * assign66820_body0_e103520)), (locals.var_dnm_dn7 / (2.0 * assign66820_body0_e103520)), (locals.var_dnm_dn8 / (2.0 * assign66820_body0_e103520)), (locals.var_dnm_dn9 / (2.0 * assign66820_body0_e103520)), (locals.var_dnm_dn10 / (2.0 * assign66820_body0_e103520)), (locals.var_dnm_dn11 / (2.0 * assign66820_body0_e103520)), (locals.var_dnm_dn14 / (2.0 * assign66820_body0_e103520)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign66820_body0_e103522;
            locals.var_dnm_dn0 = assign66820_body0_e103522_d_n0;
            locals.var_dnm_dn2 = assign66820_body0_e103522_d_n2;
            locals.var_dnm_dn4 = assign66820_body0_e103522_d_n4;
            locals.var_dnm_dn5 = assign66820_body0_e103522_d_n5;
            locals.var_dnm_dn6 = assign66820_body0_e103522_d_n6;
            locals.var_dnm_dn7 = assign66820_body0_e103522_d_n7;
            locals.var_dnm_dn8 = assign66820_body0_e103522_d_n8;
            locals.var_dnm_dn9 = assign66820_body0_e103522_d_n9;
            locals.var_dnm_dn10 = assign66820_body0_e103522_d_n10;
            locals.var_dnm_dn11 = assign66820_body0_e103522_d_n11;
            locals.var_dnm_dn14 = assign66820_body0_e103522_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign66820_body1_e103535,) = {
    if ((((locals.var_guard1582 == 0.0) && (locals.var_guard1583 != 0.0)) && (locals.var_guard1584 != 0.0)) && (locals.var_guard1585 != 0.0)) {
        let assign66820_body1_e103533: f64 = (locals.var_m0 + 1.0);
        (assign66820_body1_e103533,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign66820_body1_e103535;
            locals.var_m0_rv = 0.0;
        }

        let (assign66830_e103558, assign66830_e103558_d_n0, assign66830_e103558_d_n2, assign66830_e103558_d_n4, assign66830_e103558_d_n5, assign66830_e103558_d_n6, assign66830_e103558_d_n7, assign66830_e103558_d_n8, assign66830_e103558_d_n9, assign66830_e103558_d_n10, assign66830_e103558_d_n11, assign66830_e103558_d_n14,) = {
    if ((((locals.var_guard1582 == 0.0) && (locals.var_guard1583 != 0.0)) && (locals.var_guard1584 != 0.0)) && (locals.var_guard1585 == 0.0)) {
        let (assign66830_e103556, assign66830_e103556_d_n0, assign66830_e103556_d_n2, assign66830_e103556_d_n4, assign66830_e103556_d_n5, assign66830_e103556_d_n6, assign66830_e103556_d_n7, assign66830_e103556_d_n8, assign66830_e103556_d_n9, assign66830_e103556_d_n10, assign66830_e103556_d_n11, assign66830_e103556_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign66830_e103553: f64 = (2.0 * 4.0);
                let assign66830_e103554: f64 = (1.0 / assign66830_e103553);
                let assign66830_e103555: f64 = (locals.var_dnm).powf(assign66830_e103554);
                (assign66830_e103555, if 0.0 == 0.0 && ((assign66830_e103554) as f64).is_finite() && ((assign66830_e103554) as f64).fract() == 0.0 { if assign66830_e103554 == 0.0 { 0.0 } else { (assign66830_e103554 * ((locals.var_dnm).powf(assign66830_e103554 - 1.0) * locals.var_dnm_dn0)) } } else { (assign66830_e103555 * (assign66830_e103554 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign66830_e103554) as f64).is_finite() && ((assign66830_e103554) as f64).fract() == 0.0 { if assign66830_e103554 == 0.0 { 0.0 } else { (assign66830_e103554 * ((locals.var_dnm).powf(assign66830_e103554 - 1.0) * locals.var_dnm_dn2)) } } else { (assign66830_e103555 * (assign66830_e103554 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign66830_e103554) as f64).is_finite() && ((assign66830_e103554) as f64).fract() == 0.0 { if assign66830_e103554 == 0.0 { 0.0 } else { (assign66830_e103554 * ((locals.var_dnm).powf(assign66830_e103554 - 1.0) * locals.var_dnm_dn4)) } } else { (assign66830_e103555 * (assign66830_e103554 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign66830_e103554) as f64).is_finite() && ((assign66830_e103554) as f64).fract() == 0.0 { if assign66830_e103554 == 0.0 { 0.0 } else { (assign66830_e103554 * ((locals.var_dnm).powf(assign66830_e103554 - 1.0) * locals.var_dnm_dn5)) } } else { (assign66830_e103555 * (assign66830_e103554 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign66830_e103554) as f64).is_finite() && ((assign66830_e103554) as f64).fract() == 0.0 { if assign66830_e103554 == 0.0 { 0.0 } else { (assign66830_e103554 * ((locals.var_dnm).powf(assign66830_e103554 - 1.0) * locals.var_dnm_dn6)) } } else { (assign66830_e103555 * (assign66830_e103554 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign66830_e103554) as f64).is_finite() && ((assign66830_e103554) as f64).fract() == 0.0 { if assign66830_e103554 == 0.0 { 0.0 } else { (assign66830_e103554 * ((locals.var_dnm).powf(assign66830_e103554 - 1.0) * locals.var_dnm_dn7)) } } else { (assign66830_e103555 * (assign66830_e103554 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign66830_e103554) as f64).is_finite() && ((assign66830_e103554) as f64).fract() == 0.0 { if assign66830_e103554 == 0.0 { 0.0 } else { (assign66830_e103554 * ((locals.var_dnm).powf(assign66830_e103554 - 1.0) * locals.var_dnm_dn8)) } } else { (assign66830_e103555 * (assign66830_e103554 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign66830_e103554) as f64).is_finite() && ((assign66830_e103554) as f64).fract() == 0.0 { if assign66830_e103554 == 0.0 { 0.0 } else { (assign66830_e103554 * ((locals.var_dnm).powf(assign66830_e103554 - 1.0) * locals.var_dnm_dn9)) } } else { (assign66830_e103555 * (assign66830_e103554 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign66830_e103554) as f64).is_finite() && ((assign66830_e103554) as f64).fract() == 0.0 { if assign66830_e103554 == 0.0 { 0.0 } else { (assign66830_e103554 * ((locals.var_dnm).powf(assign66830_e103554 - 1.0) * locals.var_dnm_dn10)) } } else { (assign66830_e103555 * (assign66830_e103554 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign66830_e103554) as f64).is_finite() && ((assign66830_e103554) as f64).fract() == 0.0 { if assign66830_e103554 == 0.0 { 0.0 } else { (assign66830_e103554 * ((locals.var_dnm).powf(assign66830_e103554 - 1.0) * locals.var_dnm_dn11)) } } else { (assign66830_e103555 * (assign66830_e103554 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign66830_e103554) as f64).is_finite() && ((assign66830_e103554) as f64).fract() == 0.0 { if assign66830_e103554 == 0.0 { 0.0 } else { (assign66830_e103554 * ((locals.var_dnm).powf(assign66830_e103554 - 1.0) * locals.var_dnm_dn14)) } } else { (assign66830_e103555 * (assign66830_e103554 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign66830_e103556, assign66830_e103556_d_n0, assign66830_e103556_d_n2, assign66830_e103556_d_n4, assign66830_e103556_d_n5, assign66830_e103556_d_n6, assign66830_e103556_d_n7, assign66830_e103556_d_n8, assign66830_e103556_d_n9, assign66830_e103556_d_n10, assign66830_e103556_d_n11, assign66830_e103556_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign66830_e103558;
        locals.var_dnm_dn0 = assign66830_e103558_d_n0;
        locals.var_dnm_dn2 = assign66830_e103558_d_n2;
        locals.var_dnm_dn4 = assign66830_e103558_d_n4;
        locals.var_dnm_dn5 = assign66830_e103558_d_n5;
        locals.var_dnm_dn6 = assign66830_e103558_d_n6;
        locals.var_dnm_dn7 = assign66830_e103558_d_n7;
        locals.var_dnm_dn8 = assign66830_e103558_d_n8;
        locals.var_dnm_dn9 = assign66830_e103558_d_n9;
        locals.var_dnm_dn10 = assign66830_e103558_d_n10;
        locals.var_dnm_dn11 = assign66830_e103558_d_n11;
        locals.var_dnm_dn14 = assign66830_e103558_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign66840_e103569, assign66840_e103569_d_n0, assign66840_e103569_d_n2, assign66840_e103569_d_n4, assign66840_e103569_d_n5, assign66840_e103569_d_n6, assign66840_e103569_d_n7, assign66840_e103569_d_n8, assign66840_e103569_d_n9, assign66840_e103569_d_n10, assign66840_e103569_d_n11, assign66840_e103569_d_n14,) = {
    if (((locals.var_guard1582 == 0.0) && (locals.var_guard1583 != 0.0)) && (locals.var_guard1584 != 0.0)) {
        let assign66840_e103567: f64 = (1.0 / locals.var_dnm);
        (assign66840_e103567, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign66840_e103569;
        locals.var_dnm_dn0 = assign66840_e103569_d_n0;
        locals.var_dnm_dn2 = assign66840_e103569_d_n2;
        locals.var_dnm_dn4 = assign66840_e103569_d_n4;
        locals.var_dnm_dn5 = assign66840_e103569_d_n5;
        locals.var_dnm_dn6 = assign66840_e103569_d_n6;
        locals.var_dnm_dn7 = assign66840_e103569_d_n7;
        locals.var_dnm_dn8 = assign66840_e103569_d_n8;
        locals.var_dnm_dn9 = assign66840_e103569_d_n9;
        locals.var_dnm_dn10 = assign66840_e103569_d_n10;
        locals.var_dnm_dn11 = assign66840_e103569_d_n11;
        locals.var_dnm_dn14 = assign66840_e103569_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign66850_e103582, assign66850_e103582_d_n0, assign66850_e103582_d_n2, assign66850_e103582_d_n4, assign66850_e103582_d_n5, assign66850_e103582_d_n6, assign66850_e103582_d_n7, assign66850_e103582_d_n8, assign66850_e103582_d_n9, assign66850_e103582_d_n10, assign66850_e103582_d_n11, assign66850_e103582_d_n14,) = {
    if (((locals.var_guard1582 == 0.0) && (locals.var_guard1583 != 0.0)) && (locals.var_guard1584 != 0.0)) {
        let assign66850_e103578: f64 = (locals.var_tmf1 * locals.var_t7);
        let assign66850_e103580: f64 = (assign66850_e103578 * locals.var_dnm);
        (assign66850_e103580, ((((locals.var_tmf1_dn0 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn0)) * locals.var_dnm) + (assign66850_e103578 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn2)) * locals.var_dnm) + (assign66850_e103578 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn4 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn4)) * locals.var_dnm) + (assign66850_e103578 * locals.var_dnm_dn4)), ((((locals.var_tmf1_dn5 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn5)) * locals.var_dnm) + (assign66850_e103578 * locals.var_dnm_dn5)), ((((locals.var_tmf1_dn6 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn6)) * locals.var_dnm) + (assign66850_e103578 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn7)) * locals.var_dnm) + (assign66850_e103578 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn8 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn8)) * locals.var_dnm) + (assign66850_e103578 * locals.var_dnm_dn8)), ((((locals.var_tmf1_dn9 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn9)) * locals.var_dnm) + (assign66850_e103578 * locals.var_dnm_dn9)), ((((locals.var_tmf1_dn10 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn10)) * locals.var_dnm) + (assign66850_e103578 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn11 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn11)) * locals.var_dnm) + (assign66850_e103578 * locals.var_dnm_dn11)), ((((locals.var_tmf1_dn14 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn14)) * locals.var_dnm) + (assign66850_e103578 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign66850_e103582;
        locals.var_tmf0_dn0 = assign66850_e103582_d_n0;
        locals.var_tmf0_dn2 = assign66850_e103582_d_n2;
        locals.var_tmf0_dn4 = assign66850_e103582_d_n4;
        locals.var_tmf0_dn5 = assign66850_e103582_d_n5;
        locals.var_tmf0_dn6 = assign66850_e103582_d_n6;
        locals.var_tmf0_dn7 = assign66850_e103582_d_n7;
        locals.var_tmf0_dn8 = assign66850_e103582_d_n8;
        locals.var_tmf0_dn9 = assign66850_e103582_d_n9;
        locals.var_tmf0_dn10 = assign66850_e103582_d_n10;
        locals.var_tmf0_dn11 = assign66850_e103582_d_n11;
        locals.var_tmf0_dn14 = assign66850_e103582_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign66860_e103597, assign66860_e103597_d_n0, assign66860_e103597_d_n2, assign66860_e103597_d_n4, assign66860_e103597_d_n5, assign66860_e103597_d_n6, assign66860_e103597_d_n7, assign66860_e103597_d_n8, assign66860_e103597_d_n9, assign66860_e103597_d_n10, assign66860_e103597_d_n11, assign66860_e103597_d_n14,) = {
    if (((locals.var_guard1582 == 0.0) && (locals.var_guard1583 != 0.0)) && (locals.var_guard1584 != 0.0)) {
        let assign66860_e103591: f64 = (locals.var_t7 * locals.var_xmp);
        let assign66860_e103593: f64 = (assign66860_e103591 * locals.var_dnm);
        let assign66860_e103595: f64 = (assign66860_e103593 / locals.var_arg);
        (assign66860_e103595, (((((((locals.var_t7_dn0 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn0)) * locals.var_dnm) + (assign66860_e103591 * locals.var_dnm_dn0)) * locals.var_arg) - (assign66860_e103593 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t7_dn2 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn2)) * locals.var_dnm) + (assign66860_e103591 * locals.var_dnm_dn2)) * locals.var_arg) - (assign66860_e103593 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t7_dn4 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn4)) * locals.var_dnm) + (assign66860_e103591 * locals.var_dnm_dn4)) * locals.var_arg) - (assign66860_e103593 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t7_dn5 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn5)) * locals.var_dnm) + (assign66860_e103591 * locals.var_dnm_dn5)) * locals.var_arg) - (assign66860_e103593 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t7_dn6 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn6)) * locals.var_dnm) + (assign66860_e103591 * locals.var_dnm_dn6)) * locals.var_arg) - (assign66860_e103593 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t7_dn7 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn7)) * locals.var_dnm) + (assign66860_e103591 * locals.var_dnm_dn7)) * locals.var_arg) - (assign66860_e103593 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t7_dn8 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn8)) * locals.var_dnm) + (assign66860_e103591 * locals.var_dnm_dn8)) * locals.var_arg) - (assign66860_e103593 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t7_dn9 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn9)) * locals.var_dnm) + (assign66860_e103591 * locals.var_dnm_dn9)) * locals.var_arg) - (assign66860_e103593 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t7_dn10 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn10)) * locals.var_dnm) + (assign66860_e103591 * locals.var_dnm_dn10)) * locals.var_arg) - (assign66860_e103593 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t7_dn11 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn11)) * locals.var_dnm) + (assign66860_e103591 * locals.var_dnm_dn11)) * locals.var_arg) - (assign66860_e103593 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t7_dn14 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn14)) * locals.var_dnm) + (assign66860_e103591 * locals.var_dnm_dn14)) * locals.var_arg) - (assign66860_e103593 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign66860_e103597;
        locals.var_t0_dn0 = assign66860_e103597_d_n0;
        locals.var_t0_dn2 = assign66860_e103597_d_n2;
        locals.var_t0_dn4 = assign66860_e103597_d_n4;
        locals.var_t0_dn5 = assign66860_e103597_d_n5;
        locals.var_t0_dn6 = assign66860_e103597_d_n6;
        locals.var_t0_dn7 = assign66860_e103597_d_n7;
        locals.var_t0_dn8 = assign66860_e103597_d_n8;
        locals.var_t0_dn9 = assign66860_e103597_d_n9;
        locals.var_t0_dn10 = assign66860_e103597_d_n10;
        locals.var_t0_dn11 = assign66860_e103597_d_n11;
        locals.var_t0_dn14 = assign66860_e103597_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign66870_e103610, assign66870_e103610_d_n0, assign66870_e103610_d_n2, assign66870_e103610_d_n4, assign66870_e103610_d_n5, assign66870_e103610_d_n6, assign66870_e103610_d_n7, assign66870_e103610_d_n8, assign66870_e103610_d_n9, assign66870_e103610_d_n10, assign66870_e103610_d_n11, assign66870_e103610_d_n14,) = {
    if (((locals.var_guard1582 == 0.0) && (locals.var_guard1583 != 0.0)) && (locals.var_guard1584 != 0.0)) {
        let assign66870_e103606: f64 = (1e-6 + locals.var_t7);
        let assign66870_e103608: f64 = (assign66870_e103606 - locals.var_tmf0);
        (assign66870_e103608, (locals.var_t7_dn0 - locals.var_tmf0_dn0), (locals.var_t7_dn2 - locals.var_tmf0_dn2), (locals.var_t7_dn4 - locals.var_tmf0_dn4), (locals.var_t7_dn5 - locals.var_tmf0_dn5), (locals.var_t7_dn6 - locals.var_tmf0_dn6), (locals.var_t7_dn7 - locals.var_tmf0_dn7), (locals.var_t7_dn8 - locals.var_tmf0_dn8), (locals.var_t7_dn9 - locals.var_tmf0_dn9), (locals.var_t7_dn10 - locals.var_tmf0_dn10), (locals.var_t7_dn11 - locals.var_tmf0_dn11), (locals.var_t7_dn14 - locals.var_tmf0_dn14),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign66870_e103610;
        locals.var_t6_dn0 = assign66870_e103610_d_n0;
        locals.var_t6_dn2 = assign66870_e103610_d_n2;
        locals.var_t6_dn4 = assign66870_e103610_d_n4;
        locals.var_t6_dn5 = assign66870_e103610_d_n5;
        locals.var_t6_dn6 = assign66870_e103610_d_n6;
        locals.var_t6_dn7 = assign66870_e103610_d_n7;
        locals.var_t6_dn8 = assign66870_e103610_d_n8;
        locals.var_t6_dn9 = assign66870_e103610_d_n9;
        locals.var_t6_dn10 = assign66870_e103610_d_n10;
        locals.var_t6_dn11 = assign66870_e103610_d_n11;
        locals.var_t6_dn14 = assign66870_e103610_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign66880_e103619, assign66880_e103619_d_n0, assign66880_e103619_d_n2, assign66880_e103619_d_n4, assign66880_e103619_d_n5, assign66880_e103619_d_n6, assign66880_e103619_d_n7, assign66880_e103619_d_n8, assign66880_e103619_d_n9, assign66880_e103619_d_n10, assign66880_e103619_d_n11, assign66880_e103619_d_n14,) = {
    if (((locals.var_guard1582 == 0.0) && (locals.var_guard1583 != 0.0)) && (locals.var_guard1584 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign66880_e103619;
        locals.var_t0_dn0 = assign66880_e103619_d_n0;
        locals.var_t0_dn2 = assign66880_e103619_d_n2;
        locals.var_t0_dn4 = assign66880_e103619_d_n4;
        locals.var_t0_dn5 = assign66880_e103619_d_n5;
        locals.var_t0_dn6 = assign66880_e103619_d_n6;
        locals.var_t0_dn7 = assign66880_e103619_d_n7;
        locals.var_t0_dn8 = assign66880_e103619_d_n8;
        locals.var_t0_dn9 = assign66880_e103619_d_n9;
        locals.var_t0_dn10 = assign66880_e103619_d_n10;
        locals.var_t0_dn11 = assign66880_e103619_d_n11;
        locals.var_t0_dn14 = assign66880_e103619_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign66890_e103629, assign66890_e103629_d_n0, assign66890_e103629_d_n2, assign66890_e103629_d_n4, assign66890_e103629_d_n5, assign66890_e103629_d_n6, assign66890_e103629_d_n7, assign66890_e103629_d_n8, assign66890_e103629_d_n9, assign66890_e103629_d_n10, assign66890_e103629_d_n11, assign66890_e103629_d_n14,) = {
    if (((locals.var_guard1582 == 0.0) && (locals.var_guard1583 != 0.0)) && (locals.var_guard1584 == 0.0)) {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign66890_e103629;
        locals.var_t6_dn0 = assign66890_e103629_d_n0;
        locals.var_t6_dn2 = assign66890_e103629_d_n2;
        locals.var_t6_dn4 = assign66890_e103629_d_n4;
        locals.var_t6_dn5 = assign66890_e103629_d_n5;
        locals.var_t6_dn6 = assign66890_e103629_d_n6;
        locals.var_t6_dn7 = assign66890_e103629_d_n7;
        locals.var_t6_dn8 = assign66890_e103629_d_n8;
        locals.var_t6_dn9 = assign66890_e103629_d_n9;
        locals.var_t6_dn10 = assign66890_e103629_d_n10;
        locals.var_t6_dn11 = assign66890_e103629_d_n11;
        locals.var_t6_dn14 = assign66890_e103629_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign66900_e103639, assign66900_e103639_d_n0, assign66900_e103639_d_n2, assign66900_e103639_d_n4, assign66900_e103639_d_n5, assign66900_e103639_d_n6, assign66900_e103639_d_n7, assign66900_e103639_d_n8, assign66900_e103639_d_n9, assign66900_e103639_d_n10, assign66900_e103639_d_n11, assign66900_e103639_d_n14,) = {
    if (((locals.var_guard1582 == 0.0) && (locals.var_guard1583 != 0.0)) && (locals.var_guard1584 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign66900_e103639;
        locals.var_t0_dn0 = assign66900_e103639_d_n0;
        locals.var_t0_dn2 = assign66900_e103639_d_n2;
        locals.var_t0_dn4 = assign66900_e103639_d_n4;
        locals.var_t0_dn5 = assign66900_e103639_d_n5;
        locals.var_t0_dn6 = assign66900_e103639_d_n6;
        locals.var_t0_dn7 = assign66900_e103639_d_n7;
        locals.var_t0_dn8 = assign66900_e103639_d_n8;
        locals.var_t0_dn9 = assign66900_e103639_d_n9;
        locals.var_t0_dn10 = assign66900_e103639_d_n10;
        locals.var_t0_dn11 = assign66900_e103639_d_n11;
        locals.var_t0_dn14 = assign66900_e103639_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign66910_e103647, assign66910_e103647_d_n0, assign66910_e103647_d_n2, assign66910_e103647_d_n4, assign66910_e103647_d_n5, assign66910_e103647_d_n6, assign66910_e103647_d_n7, assign66910_e103647_d_n8, assign66910_e103647_d_n9, assign66910_e103647_d_n10, assign66910_e103647_d_n11, assign66910_e103647_d_n14,) = {
    if ((locals.var_guard1582 == 0.0) && (locals.var_guard1583 != 0.0)) {
        let assign66910_e103645: f64 = (locals.var_t6).sqrt();
        (assign66910_e103645, (locals.var_t6_dn0 / (2.0 * assign66910_e103645)), (locals.var_t6_dn2 / (2.0 * assign66910_e103645)), (locals.var_t6_dn4 / (2.0 * assign66910_e103645)), (locals.var_t6_dn5 / (2.0 * assign66910_e103645)), (locals.var_t6_dn6 / (2.0 * assign66910_e103645)), (locals.var_t6_dn7 / (2.0 * assign66910_e103645)), (locals.var_t6_dn8 / (2.0 * assign66910_e103645)), (locals.var_t6_dn9 / (2.0 * assign66910_e103645)), (locals.var_t6_dn10 / (2.0 * assign66910_e103645)), (locals.var_t6_dn11 / (2.0 * assign66910_e103645)), (locals.var_t6_dn14 / (2.0 * assign66910_e103645)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign66910_e103647;
        locals.var_t6_dn0 = assign66910_e103647_d_n0;
        locals.var_t6_dn2 = assign66910_e103647_d_n2;
        locals.var_t6_dn4 = assign66910_e103647_d_n4;
        locals.var_t6_dn5 = assign66910_e103647_d_n5;
        locals.var_t6_dn6 = assign66910_e103647_d_n6;
        locals.var_t6_dn7 = assign66910_e103647_d_n7;
        locals.var_t6_dn8 = assign66910_e103647_d_n8;
        locals.var_t6_dn9 = assign66910_e103647_d_n9;
        locals.var_t6_dn10 = assign66910_e103647_d_n10;
        locals.var_t6_dn11 = assign66910_e103647_d_n11;
        locals.var_t6_dn14 = assign66910_e103647_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign66920_e103660, assign66920_e103660_d_n0, assign66920_e103660_d_n2, assign66920_e103660_d_n4, assign66920_e103660_d_n5, assign66920_e103660_d_n6, assign66920_e103660_d_n7, assign66920_e103660_d_n8, assign66920_e103660_d_n9, assign66920_e103660_d_n10, assign66920_e103660_d_n11, assign66920_e103660_d_n14,) = {
    if ((locals.var_guard1582 == 0.0) && (locals.var_guard1583 != 0.0)) {
        let assign66920_e103656: f64 = (1.0 - locals.var_t6);
        let assign66920_e103657: f64 = (locals.var_t3 * assign66920_e103656);
        let assign66920_e103658: f64 = (locals.var_t1 + assign66920_e103657);
        (assign66920_e103658, (locals.var_t1_dn0 + ((locals.var_t3_dn0 * assign66920_e103656) + (locals.var_t3 * (-locals.var_t6_dn0)))), (locals.var_t1_dn2 + ((locals.var_t3_dn2 * assign66920_e103656) + (locals.var_t3 * (-locals.var_t6_dn2)))), (locals.var_t1_dn4 + ((locals.var_t3_dn4 * assign66920_e103656) + (locals.var_t3 * (-locals.var_t6_dn4)))), (locals.var_t1_dn5 + ((locals.var_t3_dn5 * assign66920_e103656) + (locals.var_t3 * (-locals.var_t6_dn5)))), (locals.var_t1_dn6 + ((locals.var_t3_dn6 * assign66920_e103656) + (locals.var_t3 * (-locals.var_t6_dn6)))), (locals.var_t1_dn7 + ((locals.var_t3_dn7 * assign66920_e103656) + (locals.var_t3 * (-locals.var_t6_dn7)))), (locals.var_t1_dn8 + ((locals.var_t3_dn8 * assign66920_e103656) + (locals.var_t3 * (-locals.var_t6_dn8)))), (locals.var_t1_dn9 + ((locals.var_t3_dn9 * assign66920_e103656) + (locals.var_t3 * (-locals.var_t6_dn9)))), (locals.var_t1_dn10 + ((locals.var_t3_dn10 * assign66920_e103656) + (locals.var_t3 * (-locals.var_t6_dn10)))), (locals.var_t1_dn11 + ((locals.var_t3_dn11 * assign66920_e103656) + (locals.var_t3 * (-locals.var_t6_dn11)))), (locals.var_t1_dn14 + ((locals.var_t3_dn14 * assign66920_e103656) + (locals.var_t3 * (-locals.var_t6_dn14)))),)
    } else {
        (locals.var_psislsat, locals.var_psislsat_dn0, locals.var_psislsat_dn2, locals.var_psislsat_dn4, locals.var_psislsat_dn5, locals.var_psislsat_dn6, locals.var_psislsat_dn7, locals.var_psislsat_dn8, locals.var_psislsat_dn9, locals.var_psislsat_dn10, locals.var_psislsat_dn11, locals.var_psislsat_dn14,)
    }
};
        locals.var_psislsat = assign66920_e103660;
        locals.var_psislsat_dn0 = assign66920_e103660_d_n0;
        locals.var_psislsat_dn2 = assign66920_e103660_d_n2;
        locals.var_psislsat_dn4 = assign66920_e103660_d_n4;
        locals.var_psislsat_dn5 = assign66920_e103660_d_n5;
        locals.var_psislsat_dn6 = assign66920_e103660_d_n6;
        locals.var_psislsat_dn7 = assign66920_e103660_d_n7;
        locals.var_psislsat_dn8 = assign66920_e103660_d_n8;
        locals.var_psislsat_dn9 = assign66920_e103660_d_n9;
        locals.var_psislsat_dn10 = assign66920_e103660_d_n10;
        locals.var_psislsat_dn11 = assign66920_e103660_d_n11;
        locals.var_psislsat_dn14 = assign66920_e103660_d_n14;
        locals.var_psislsat_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_252(
        locals: &mut StampLocals,
    ) {
        let (assign66930_e103671, assign66930_e103671_d_n0, assign66930_e103671_d_n2, assign66930_e103671_d_n4, assign66930_e103671_d_n5, assign66930_e103671_d_n6, assign66930_e103671_d_n7, assign66930_e103671_d_n8, assign66930_e103671_d_n9, assign66930_e103671_d_n10, assign66930_e103671_d_n11, assign66930_e103671_d_n14,) = {
    if ((locals.var_guard1582 == 0.0) && (locals.var_guard1583 != 0.0)) {
        let assign66930_e103668: f64 = (locals.var_xgate + locals.var_lgate);
        let assign66930_e103669: f64 = (locals.var_lgate / assign66930_e103668);
        (assign66930_e103669, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign66930_e103671;
        locals.var_t2_dn0 = assign66930_e103671_d_n0;
        locals.var_t2_dn2 = assign66930_e103671_d_n2;
        locals.var_t2_dn4 = assign66930_e103671_d_n4;
        locals.var_t2_dn5 = assign66930_e103671_d_n5;
        locals.var_t2_dn6 = assign66930_e103671_d_n6;
        locals.var_t2_dn7 = assign66930_e103671_d_n7;
        locals.var_t2_dn8 = assign66930_e103671_d_n8;
        locals.var_t2_dn9 = assign66930_e103671_d_n9;
        locals.var_t2_dn10 = assign66930_e103671_d_n10;
        locals.var_t2_dn11 = assign66930_e103671_d_n11;
        locals.var_t2_dn14 = assign66930_e103671_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign66940_e103686, assign66940_e103686_d_n0, assign66940_e103686_d_n2, assign66940_e103686_d_n4, assign66940_e103686_d_n5, assign66940_e103686_d_n6, assign66940_e103686_d_n7, assign66940_e103686_d_n8, assign66940_e103686_d_n9, assign66940_e103686_d_n10, assign66940_e103686_d_n11, assign66940_e103686_d_n14,) = {
    if ((locals.var_guard1582 == 0.0) && (locals.var_guard1583 != 0.0)) {
        let assign66940_e103678: f64 = (locals.var_uc_svds * locals.var_vdsz__blk441);
        let assign66940_e103680: f64 = (assign66940_e103678 + locals.var_ps0z);
        let assign66940_e103683: f64 = (locals.var_t2 * locals.var_psislsat);
        let assign66940_e103684: f64 = (assign66940_e103680 - assign66940_e103683);
        (assign66940_e103684, (((locals.var_uc_svds * locals.var_vdsz__blk441_dn0) + locals.var_ps0z_dn0) - ((locals.var_t2_dn0 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn0))), (((locals.var_uc_svds * locals.var_vdsz__blk441_dn2) + locals.var_ps0z_dn2) - ((locals.var_t2_dn2 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn2))), (((locals.var_uc_svds * locals.var_vdsz__blk441_dn4) + locals.var_ps0z_dn4) - ((locals.var_t2_dn4 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn4))), (((locals.var_uc_svds * locals.var_vdsz__blk441_dn5) + locals.var_ps0z_dn5) - ((locals.var_t2_dn5 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn5))), (((locals.var_uc_svds * locals.var_vdsz__blk441_dn6) + locals.var_ps0z_dn6) - ((locals.var_t2_dn6 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn6))), (((locals.var_uc_svds * locals.var_vdsz__blk441_dn7) + locals.var_ps0z_dn7) - ((locals.var_t2_dn7 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn7))), (((locals.var_uc_svds * locals.var_vdsz__blk441_dn8) + locals.var_ps0z_dn8) - ((locals.var_t2_dn8 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn8))), (((locals.var_uc_svds * locals.var_vdsz__blk441_dn9) + locals.var_ps0z_dn9) - ((locals.var_t2_dn9 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn9))), (((locals.var_uc_svds * locals.var_vdsz__blk441_dn10) + locals.var_ps0z_dn10) - ((locals.var_t2_dn10 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn10))), (((locals.var_uc_svds * locals.var_vdsz__blk441_dn11) + locals.var_ps0z_dn11) - ((locals.var_t2_dn11 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn11))), (((locals.var_uc_svds * locals.var_vdsz__blk441_dn14) + locals.var_ps0z_dn14) - ((locals.var_t2_dn14 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn14))),)
    } else {
        (locals.var_psisubsat, locals.var_psisubsat_dn0, locals.var_psisubsat_dn2, locals.var_psisubsat_dn4, locals.var_psisubsat_dn5, locals.var_psisubsat_dn6, locals.var_psisubsat_dn7, locals.var_psisubsat_dn8, locals.var_psisubsat_dn9, locals.var_psisubsat_dn10, locals.var_psisubsat_dn11, locals.var_psisubsat_dn14,)
    }
};
        locals.var_psisubsat = assign66940_e103686;
        locals.var_psisubsat_dn0 = assign66940_e103686_d_n0;
        locals.var_psisubsat_dn2 = assign66940_e103686_d_n2;
        locals.var_psisubsat_dn4 = assign66940_e103686_d_n4;
        locals.var_psisubsat_dn5 = assign66940_e103686_d_n5;
        locals.var_psisubsat_dn6 = assign66940_e103686_d_n6;
        locals.var_psisubsat_dn7 = assign66940_e103686_d_n7;
        locals.var_psisubsat_dn8 = assign66940_e103686_d_n8;
        locals.var_psisubsat_dn9 = assign66940_e103686_d_n9;
        locals.var_psisubsat_dn10 = assign66940_e103686_d_n10;
        locals.var_psisubsat_dn11 = assign66940_e103686_d_n11;
        locals.var_psisubsat_dn14 = assign66940_e103686_d_n14;
        locals.var_psisubsat_rv = 0.0;

        let (assign66950_e103702, assign66950_e103702_d_n0, assign66950_e103702_d_n2, assign66950_e103702_d_n4, assign66950_e103702_d_n5, assign66950_e103702_d_n6, assign66950_e103702_d_n7, assign66950_e103702_d_n8, assign66950_e103702_d_n9, assign66950_e103702_d_n10, assign66950_e103702_d_n11, assign66950_e103702_d_n14,) = {
    if ((locals.var_guard1582 == 0.0) && (locals.var_guard1583 != 0.0)) {
        let assign66950_e103693: f64 = (locals.var_psisubsat * locals.var_psisubsat);
        let assign66950_e103696: f64 = (4.0 * 0.001);
        let assign66950_e103698: f64 = (assign66950_e103696 * 0.001);
        let assign66950_e103699: f64 = (assign66950_e103693 + assign66950_e103698);
        let assign66950_e103700: f64 = (assign66950_e103699).sqrt();
        (assign66950_e103700, (((locals.var_psisubsat_dn0 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn0)) / (2.0 * assign66950_e103700)), (((locals.var_psisubsat_dn2 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn2)) / (2.0 * assign66950_e103700)), (((locals.var_psisubsat_dn4 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn4)) / (2.0 * assign66950_e103700)), (((locals.var_psisubsat_dn5 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn5)) / (2.0 * assign66950_e103700)), (((locals.var_psisubsat_dn6 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn6)) / (2.0 * assign66950_e103700)), (((locals.var_psisubsat_dn7 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn7)) / (2.0 * assign66950_e103700)), (((locals.var_psisubsat_dn8 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn8)) / (2.0 * assign66950_e103700)), (((locals.var_psisubsat_dn9 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn9)) / (2.0 * assign66950_e103700)), (((locals.var_psisubsat_dn10 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn10)) / (2.0 * assign66950_e103700)), (((locals.var_psisubsat_dn11 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn11)) / (2.0 * assign66950_e103700)), (((locals.var_psisubsat_dn14 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn14)) / (2.0 * assign66950_e103700)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign66950_e103702;
        locals.var_tmf2_dn0 = assign66950_e103702_d_n0;
        locals.var_tmf2_dn2 = assign66950_e103702_d_n2;
        locals.var_tmf2_dn4 = assign66950_e103702_d_n4;
        locals.var_tmf2_dn5 = assign66950_e103702_d_n5;
        locals.var_tmf2_dn6 = assign66950_e103702_d_n6;
        locals.var_tmf2_dn7 = assign66950_e103702_d_n7;
        locals.var_tmf2_dn8 = assign66950_e103702_d_n8;
        locals.var_tmf2_dn9 = assign66950_e103702_d_n9;
        locals.var_tmf2_dn10 = assign66950_e103702_d_n10;
        locals.var_tmf2_dn11 = assign66950_e103702_d_n11;
        locals.var_tmf2_dn14 = assign66950_e103702_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign66960_e103715, assign66960_e103715_d_n0, assign66960_e103715_d_n2, assign66960_e103715_d_n4, assign66960_e103715_d_n5, assign66960_e103715_d_n6, assign66960_e103715_d_n7, assign66960_e103715_d_n8, assign66960_e103715_d_n9, assign66960_e103715_d_n10, assign66960_e103715_d_n11, assign66960_e103715_d_n14,) = {
    if ((locals.var_guard1582 == 0.0) && (locals.var_guard1583 != 0.0)) {
        let assign66960_e103711: f64 = (locals.var_psisubsat / locals.var_tmf2);
        let assign66960_e103712: f64 = (1.0 + assign66960_e103711);
        let assign66960_e103713: f64 = (0.5 * assign66960_e103712);
        (assign66960_e103713, (0.5 * (((locals.var_psisubsat_dn0 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn2 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn4 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn5 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn6 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn7 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn8 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn9 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn10 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn11 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn14 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign66960_e103715;
        locals.var_t9_dn0 = assign66960_e103715_d_n0;
        locals.var_t9_dn2 = assign66960_e103715_d_n2;
        locals.var_t9_dn4 = assign66960_e103715_d_n4;
        locals.var_t9_dn5 = assign66960_e103715_d_n5;
        locals.var_t9_dn6 = assign66960_e103715_d_n6;
        locals.var_t9_dn7 = assign66960_e103715_d_n7;
        locals.var_t9_dn8 = assign66960_e103715_d_n8;
        locals.var_t9_dn9 = assign66960_e103715_d_n9;
        locals.var_t9_dn10 = assign66960_e103715_d_n10;
        locals.var_t9_dn11 = assign66960_e103715_d_n11;
        locals.var_t9_dn14 = assign66960_e103715_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign66970_e103726, assign66970_e103726_d_n0, assign66970_e103726_d_n2, assign66970_e103726_d_n4, assign66970_e103726_d_n5, assign66970_e103726_d_n6, assign66970_e103726_d_n7, assign66970_e103726_d_n8, assign66970_e103726_d_n9, assign66970_e103726_d_n10, assign66970_e103726_d_n11, assign66970_e103726_d_n14,) = {
    if ((locals.var_guard1582 == 0.0) && (locals.var_guard1583 != 0.0)) {
        let assign66970_e103723: f64 = (locals.var_psisubsat + locals.var_tmf2);
        let assign66970_e103724: f64 = (0.5 * assign66970_e103723);
        (assign66970_e103724, (0.5 * (locals.var_psisubsat_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_psisubsat_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_psisubsat_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_psisubsat_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_psisubsat_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_psisubsat_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_psisubsat_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_psisubsat_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_psisubsat_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_psisubsat_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_psisubsat_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_psisubsat, locals.var_psisubsat_dn0, locals.var_psisubsat_dn2, locals.var_psisubsat_dn4, locals.var_psisubsat_dn5, locals.var_psisubsat_dn6, locals.var_psisubsat_dn7, locals.var_psisubsat_dn8, locals.var_psisubsat_dn9, locals.var_psisubsat_dn10, locals.var_psisubsat_dn11, locals.var_psisubsat_dn14,)
    }
};
        locals.var_psisubsat = assign66970_e103726;
        locals.var_psisubsat_dn0 = assign66970_e103726_d_n0;
        locals.var_psisubsat_dn2 = assign66970_e103726_d_n2;
        locals.var_psisubsat_dn4 = assign66970_e103726_d_n4;
        locals.var_psisubsat_dn5 = assign66970_e103726_d_n5;
        locals.var_psisubsat_dn6 = assign66970_e103726_d_n6;
        locals.var_psisubsat_dn7 = assign66970_e103726_d_n7;
        locals.var_psisubsat_dn8 = assign66970_e103726_d_n8;
        locals.var_psisubsat_dn9 = assign66970_e103726_d_n9;
        locals.var_psisubsat_dn10 = assign66970_e103726_d_n10;
        locals.var_psisubsat_dn11 = assign66970_e103726_d_n11;
        locals.var_psisubsat_dn14 = assign66970_e103726_d_n14;
        locals.var_psisubsat_rv = 0.0;

        let assign66980_e103729: f64 = if locals.var_psisubsat < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1590 = assign66980_e103729;
        locals.var_guard1590_rv = 0.0;

        let (assign66990_e103738, assign66990_e103738_d_n0, assign66990_e103738_d_n2, assign66990_e103738_d_n4, assign66990_e103738_d_n5, assign66990_e103738_d_n6, assign66990_e103738_d_n7, assign66990_e103738_d_n8, assign66990_e103738_d_n9, assign66990_e103738_d_n10, assign66990_e103738_d_n11, assign66990_e103738_d_n14,) = {
    if (((locals.var_guard1582 == 0.0) && (locals.var_guard1583 != 0.0)) && (locals.var_guard1590 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_psisubsat, locals.var_psisubsat_dn0, locals.var_psisubsat_dn2, locals.var_psisubsat_dn4, locals.var_psisubsat_dn5, locals.var_psisubsat_dn6, locals.var_psisubsat_dn7, locals.var_psisubsat_dn8, locals.var_psisubsat_dn9, locals.var_psisubsat_dn10, locals.var_psisubsat_dn11, locals.var_psisubsat_dn14,)
    }
};
        locals.var_psisubsat = assign66990_e103738;
        locals.var_psisubsat_dn0 = assign66990_e103738_d_n0;
        locals.var_psisubsat_dn2 = assign66990_e103738_d_n2;
        locals.var_psisubsat_dn4 = assign66990_e103738_d_n4;
        locals.var_psisubsat_dn5 = assign66990_e103738_d_n5;
        locals.var_psisubsat_dn6 = assign66990_e103738_d_n6;
        locals.var_psisubsat_dn7 = assign66990_e103738_d_n7;
        locals.var_psisubsat_dn8 = assign66990_e103738_d_n8;
        locals.var_psisubsat_dn9 = assign66990_e103738_d_n9;
        locals.var_psisubsat_dn10 = assign66990_e103738_d_n10;
        locals.var_psisubsat_dn11 = assign66990_e103738_d_n11;
        locals.var_psisubsat_dn14 = assign66990_e103738_d_n14;
        locals.var_psisubsat_rv = 0.0;

        let (assign67000_e103747, assign67000_e103747_d_n0, assign67000_e103747_d_n2, assign67000_e103747_d_n4, assign67000_e103747_d_n5, assign67000_e103747_d_n6, assign67000_e103747_d_n7, assign67000_e103747_d_n8, assign67000_e103747_d_n9, assign67000_e103747_d_n10, assign67000_e103747_d_n11, assign67000_e103747_d_n14,) = {
    if (((locals.var_guard1582 == 0.0) && (locals.var_guard1583 != 0.0)) && (locals.var_guard1590 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign67000_e103747;
        locals.var_t9_dn0 = assign67000_e103747_d_n0;
        locals.var_t9_dn2 = assign67000_e103747_d_n2;
        locals.var_t9_dn4 = assign67000_e103747_d_n4;
        locals.var_t9_dn5 = assign67000_e103747_d_n5;
        locals.var_t9_dn6 = assign67000_e103747_d_n6;
        locals.var_t9_dn7 = assign67000_e103747_d_n7;
        locals.var_t9_dn8 = assign67000_e103747_d_n8;
        locals.var_t9_dn9 = assign67000_e103747_d_n9;
        locals.var_t9_dn10 = assign67000_e103747_d_n10;
        locals.var_t9_dn11 = assign67000_e103747_d_n11;
        locals.var_t9_dn14 = assign67000_e103747_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign67010_e103756, assign67010_e103756_d_n0, assign67010_e103756_d_n2, assign67010_e103756_d_n4, assign67010_e103756_d_n5, assign67010_e103756_d_n6, assign67010_e103756_d_n7, assign67010_e103756_d_n8, assign67010_e103756_d_n9, assign67010_e103756_d_n10, assign67010_e103756_d_n11, assign67010_e103756_d_n14,) = {
    if ((locals.var_guard1582 == 0.0) && (locals.var_guard1583 != 0.0)) {
        let assign67010_e103754: f64 = (locals.var_psisubsat + 1e-25);
        (assign67010_e103754, locals.var_psisubsat_dn0, locals.var_psisubsat_dn2, locals.var_psisubsat_dn4, locals.var_psisubsat_dn5, locals.var_psisubsat_dn6, locals.var_psisubsat_dn7, locals.var_psisubsat_dn8, locals.var_psisubsat_dn9, locals.var_psisubsat_dn10, locals.var_psisubsat_dn11, locals.var_psisubsat_dn14,)
    } else {
        (locals.var_psisubsat, locals.var_psisubsat_dn0, locals.var_psisubsat_dn2, locals.var_psisubsat_dn4, locals.var_psisubsat_dn5, locals.var_psisubsat_dn6, locals.var_psisubsat_dn7, locals.var_psisubsat_dn8, locals.var_psisubsat_dn9, locals.var_psisubsat_dn10, locals.var_psisubsat_dn11, locals.var_psisubsat_dn14,)
    }
};
        locals.var_psisubsat = assign67010_e103756;
        locals.var_psisubsat_dn0 = assign67010_e103756_d_n0;
        locals.var_psisubsat_dn2 = assign67010_e103756_d_n2;
        locals.var_psisubsat_dn4 = assign67010_e103756_d_n4;
        locals.var_psisubsat_dn5 = assign67010_e103756_d_n5;
        locals.var_psisubsat_dn6 = assign67010_e103756_d_n6;
        locals.var_psisubsat_dn7 = assign67010_e103756_d_n7;
        locals.var_psisubsat_dn8 = assign67010_e103756_d_n8;
        locals.var_psisubsat_dn9 = assign67010_e103756_d_n9;
        locals.var_psisubsat_dn10 = assign67010_e103756_d_n10;
        locals.var_psisubsat_dn11 = assign67010_e103756_d_n11;
        locals.var_psisubsat_dn14 = assign67010_e103756_d_n14;
        locals.var_psisubsat_rv = 0.0;

        let (assign67020_e103769, assign67020_e103769_d_n0, assign67020_e103769_d_n2, assign67020_e103769_d_n4, assign67020_e103769_d_n5, assign67020_e103769_d_n6, assign67020_e103769_d_n7, assign67020_e103769_d_n8, assign67020_e103769_d_n9, assign67020_e103769_d_n10, assign67020_e103769_d_n11, assign67020_e103769_d_n14,) = {
    if ((locals.var_guard1582 == 0.0) && (locals.var_guard1583 != 0.0)) {
        let assign67020_e103765: f64 = (locals.var_ttemp - locals.var_ktnom);
        let assign67020_e103766: f64 = (locals.var_uc_subtmp * assign67020_e103765);
        let assign67020_e103767: f64 = (1.0 + assign67020_e103766);
        (assign67020_e103767, (locals.var_uc_subtmp * locals.var_ttemp_dn0), (locals.var_uc_subtmp * locals.var_ttemp_dn2), (locals.var_uc_subtmp * locals.var_ttemp_dn4), (locals.var_uc_subtmp * locals.var_ttemp_dn5), (locals.var_uc_subtmp * locals.var_ttemp_dn6), (locals.var_uc_subtmp * locals.var_ttemp_dn7), (locals.var_uc_subtmp * locals.var_ttemp_dn8), (locals.var_uc_subtmp * locals.var_ttemp_dn9), (locals.var_uc_subtmp * locals.var_ttemp_dn10), (locals.var_uc_subtmp * locals.var_ttemp_dn11), (locals.var_uc_subtmp * locals.var_ttemp_dn14),)
    } else {
        (locals.var_xsubtmp, locals.var_xsubtmp_dn0, locals.var_xsubtmp_dn2, locals.var_xsubtmp_dn4, locals.var_xsubtmp_dn5, locals.var_xsubtmp_dn6, locals.var_xsubtmp_dn7, locals.var_xsubtmp_dn8, locals.var_xsubtmp_dn9, locals.var_xsubtmp_dn10, locals.var_xsubtmp_dn11, locals.var_xsubtmp_dn14,)
    }
};
        locals.var_xsubtmp = assign67020_e103769;
        locals.var_xsubtmp_dn0 = assign67020_e103769_d_n0;
        locals.var_xsubtmp_dn2 = assign67020_e103769_d_n2;
        locals.var_xsubtmp_dn4 = assign67020_e103769_d_n4;
        locals.var_xsubtmp_dn5 = assign67020_e103769_d_n5;
        locals.var_xsubtmp_dn6 = assign67020_e103769_d_n6;
        locals.var_xsubtmp_dn7 = assign67020_e103769_d_n7;
        locals.var_xsubtmp_dn8 = assign67020_e103769_d_n8;
        locals.var_xsubtmp_dn9 = assign67020_e103769_d_n9;
        locals.var_xsubtmp_dn10 = assign67020_e103769_d_n10;
        locals.var_xsubtmp_dn11 = assign67020_e103769_d_n11;
        locals.var_xsubtmp_dn14 = assign67020_e103769_d_n14;
        locals.var_xsubtmp_rv = 0.0;

        let (assign67030_e103781, assign67030_e103781_d_n0, assign67030_e103781_d_n2, assign67030_e103781_d_n4, assign67030_e103781_d_n5, assign67030_e103781_d_n6, assign67030_e103781_d_n7, assign67030_e103781_d_n8, assign67030_e103781_d_n9, assign67030_e103781_d_n10, assign67030_e103781_d_n11, assign67030_e103781_d_n14,) = {
    if ((locals.var_guard1582 == 0.0) && (locals.var_guard1583 != 0.0)) {
        let (assign67030_e103779, assign67030_e103779_d_n0, assign67030_e103779_d_n2, assign67030_e103779_d_n4, assign67030_e103779_d_n5, assign67030_e103779_d_n6, assign67030_e103779_d_n7, assign67030_e103779_d_n8, assign67030_e103779_d_n9, assign67030_e103779_d_n10, assign67030_e103779_d_n11, assign67030_e103779_d_n14,) = {
            if (locals.var_xsubtmp <= 0.001) {
                (0.001, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                (locals.var_xsubtmp, locals.var_xsubtmp_dn0, locals.var_xsubtmp_dn2, locals.var_xsubtmp_dn4, locals.var_xsubtmp_dn5, locals.var_xsubtmp_dn6, locals.var_xsubtmp_dn7, locals.var_xsubtmp_dn8, locals.var_xsubtmp_dn9, locals.var_xsubtmp_dn10, locals.var_xsubtmp_dn11, locals.var_xsubtmp_dn14,)
            }
        };
        (assign67030_e103779, assign67030_e103779_d_n0, assign67030_e103779_d_n2, assign67030_e103779_d_n4, assign67030_e103779_d_n5, assign67030_e103779_d_n6, assign67030_e103779_d_n7, assign67030_e103779_d_n8, assign67030_e103779_d_n9, assign67030_e103779_d_n10, assign67030_e103779_d_n11, assign67030_e103779_d_n14,)
    } else {
        (locals.var_xsubtmp, locals.var_xsubtmp_dn0, locals.var_xsubtmp_dn2, locals.var_xsubtmp_dn4, locals.var_xsubtmp_dn5, locals.var_xsubtmp_dn6, locals.var_xsubtmp_dn7, locals.var_xsubtmp_dn8, locals.var_xsubtmp_dn9, locals.var_xsubtmp_dn10, locals.var_xsubtmp_dn11, locals.var_xsubtmp_dn14,)
    }
};
        locals.var_xsubtmp = assign67030_e103781;
        locals.var_xsubtmp_dn0 = assign67030_e103781_d_n0;
        locals.var_xsubtmp_dn2 = assign67030_e103781_d_n2;
        locals.var_xsubtmp_dn4 = assign67030_e103781_d_n4;
        locals.var_xsubtmp_dn5 = assign67030_e103781_d_n5;
        locals.var_xsubtmp_dn6 = assign67030_e103781_d_n6;
        locals.var_xsubtmp_dn7 = assign67030_e103781_d_n7;
        locals.var_xsubtmp_dn8 = assign67030_e103781_d_n8;
        locals.var_xsubtmp_dn9 = assign67030_e103781_d_n9;
        locals.var_xsubtmp_dn10 = assign67030_e103781_d_n10;
        locals.var_xsubtmp_dn11 = assign67030_e103781_d_n11;
        locals.var_xsubtmp_dn14 = assign67030_e103781_d_n14;
        locals.var_xsubtmp_rv = 0.0;

        let (assign67040_e103790, assign67040_e103790_d_n0, assign67040_e103790_d_n2, assign67040_e103790_d_n4, assign67040_e103790_d_n5, assign67040_e103790_d_n6, assign67040_e103790_d_n7, assign67040_e103790_d_n8, assign67040_e103790_d_n9, assign67040_e103790_d_n10, assign67040_e103790_d_n11, assign67040_e103790_d_n14,) = {
    if ((locals.var_guard1582 == 0.0) && (locals.var_guard1583 != 0.0)) {
        let assign67040_e103788: f64 = (locals.var_xsub1 / locals.var_xsubtmp);
        (assign67040_e103788, (-((locals.var_xsub1 * locals.var_xsubtmp_dn0) / (locals.var_xsubtmp * locals.var_xsubtmp))), (-((locals.var_xsub1 * locals.var_xsubtmp_dn2) / (locals.var_xsubtmp * locals.var_xsubtmp))), (-((locals.var_xsub1 * locals.var_xsubtmp_dn4) / (locals.var_xsubtmp * locals.var_xsubtmp))), (-((locals.var_xsub1 * locals.var_xsubtmp_dn5) / (locals.var_xsubtmp * locals.var_xsubtmp))), (-((locals.var_xsub1 * locals.var_xsubtmp_dn6) / (locals.var_xsubtmp * locals.var_xsubtmp))), (-((locals.var_xsub1 * locals.var_xsubtmp_dn7) / (locals.var_xsubtmp * locals.var_xsubtmp))), (-((locals.var_xsub1 * locals.var_xsubtmp_dn8) / (locals.var_xsubtmp * locals.var_xsubtmp))), (-((locals.var_xsub1 * locals.var_xsubtmp_dn9) / (locals.var_xsubtmp * locals.var_xsubtmp))), (-((locals.var_xsub1 * locals.var_xsubtmp_dn10) / (locals.var_xsubtmp * locals.var_xsubtmp))), (-((locals.var_xsub1 * locals.var_xsubtmp_dn11) / (locals.var_xsubtmp * locals.var_xsubtmp))), (-((locals.var_xsub1 * locals.var_xsubtmp_dn14) / (locals.var_xsubtmp * locals.var_xsubtmp))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign67040_e103790;
        locals.var_t5_dn0 = assign67040_e103790_d_n0;
        locals.var_t5_dn2 = assign67040_e103790_d_n2;
        locals.var_t5_dn4 = assign67040_e103790_d_n4;
        locals.var_t5_dn5 = assign67040_e103790_d_n5;
        locals.var_t5_dn6 = assign67040_e103790_d_n6;
        locals.var_t5_dn7 = assign67040_e103790_d_n7;
        locals.var_t5_dn8 = assign67040_e103790_d_n8;
        locals.var_t5_dn9 = assign67040_e103790_d_n9;
        locals.var_t5_dn10 = assign67040_e103790_d_n10;
        locals.var_t5_dn11 = assign67040_e103790_d_n11;
        locals.var_t5_dn14 = assign67040_e103790_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign67050_e103799, assign67050_e103799_d_n0, assign67050_e103799_d_n2, assign67050_e103799_d_n4, assign67050_e103799_d_n5, assign67050_e103799_d_n6, assign67050_e103799_d_n7, assign67050_e103799_d_n8, assign67050_e103799_d_n9, assign67050_e103799_d_n10, assign67050_e103799_d_n11, assign67050_e103799_d_n14,) = {
    if ((locals.var_guard1582 == 0.0) && (locals.var_guard1583 != 0.0)) {
        let assign67050_e103797: f64 = (locals.var_xsub2 * locals.var_xsubtmp);
        (assign67050_e103797, (locals.var_xsub2 * locals.var_xsubtmp_dn0), (locals.var_xsub2 * locals.var_xsubtmp_dn2), (locals.var_xsub2 * locals.var_xsubtmp_dn4), (locals.var_xsub2 * locals.var_xsubtmp_dn5), (locals.var_xsub2 * locals.var_xsubtmp_dn6), (locals.var_xsub2 * locals.var_xsubtmp_dn7), (locals.var_xsub2 * locals.var_xsubtmp_dn8), (locals.var_xsub2 * locals.var_xsubtmp_dn9), (locals.var_xsub2 * locals.var_xsubtmp_dn10), (locals.var_xsub2 * locals.var_xsubtmp_dn11), (locals.var_xsub2 * locals.var_xsubtmp_dn14),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign67050_e103799;
        locals.var_t6_dn0 = assign67050_e103799_d_n0;
        locals.var_t6_dn2 = assign67050_e103799_d_n2;
        locals.var_t6_dn4 = assign67050_e103799_d_n4;
        locals.var_t6_dn5 = assign67050_e103799_d_n5;
        locals.var_t6_dn6 = assign67050_e103799_d_n6;
        locals.var_t6_dn7 = assign67050_e103799_d_n7;
        locals.var_t6_dn8 = assign67050_e103799_d_n8;
        locals.var_t6_dn9 = assign67050_e103799_d_n9;
        locals.var_t6_dn10 = assign67050_e103799_d_n10;
        locals.var_t6_dn11 = assign67050_e103799_d_n11;
        locals.var_t6_dn14 = assign67050_e103799_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign67060_e103810, assign67060_e103810_d_n0, assign67060_e103810_d_n2, assign67060_e103810_d_n4, assign67060_e103810_d_n5, assign67060_e103810_d_n6, assign67060_e103810_d_n7, assign67060_e103810_d_n8, assign67060_e103810_d_n9, assign67060_e103810_d_n10, assign67060_e103810_d_n11, assign67060_e103810_d_n14,) = {
    if ((locals.var_guard1582 == 0.0) && (locals.var_guard1583 != 0.0)) {
        let assign67060_e103805: f64 = (-locals.var_t6);
        let assign67060_e103807: f64 = (assign67060_e103805 / locals.var_psisubsat);
        let assign67060_e103808: f64 = (assign67060_e103807).exp();
        (assign67060_e103808, (assign67060_e103808 * ((((-locals.var_t6_dn0) * locals.var_psisubsat) - (assign67060_e103805 * locals.var_psisubsat_dn0)) / (locals.var_psisubsat * locals.var_psisubsat))), (assign67060_e103808 * ((((-locals.var_t6_dn2) * locals.var_psisubsat) - (assign67060_e103805 * locals.var_psisubsat_dn2)) / (locals.var_psisubsat * locals.var_psisubsat))), (assign67060_e103808 * ((((-locals.var_t6_dn4) * locals.var_psisubsat) - (assign67060_e103805 * locals.var_psisubsat_dn4)) / (locals.var_psisubsat * locals.var_psisubsat))), (assign67060_e103808 * ((((-locals.var_t6_dn5) * locals.var_psisubsat) - (assign67060_e103805 * locals.var_psisubsat_dn5)) / (locals.var_psisubsat * locals.var_psisubsat))), (assign67060_e103808 * ((((-locals.var_t6_dn6) * locals.var_psisubsat) - (assign67060_e103805 * locals.var_psisubsat_dn6)) / (locals.var_psisubsat * locals.var_psisubsat))), (assign67060_e103808 * ((((-locals.var_t6_dn7) * locals.var_psisubsat) - (assign67060_e103805 * locals.var_psisubsat_dn7)) / (locals.var_psisubsat * locals.var_psisubsat))), (assign67060_e103808 * ((((-locals.var_t6_dn8) * locals.var_psisubsat) - (assign67060_e103805 * locals.var_psisubsat_dn8)) / (locals.var_psisubsat * locals.var_psisubsat))), (assign67060_e103808 * ((((-locals.var_t6_dn9) * locals.var_psisubsat) - (assign67060_e103805 * locals.var_psisubsat_dn9)) / (locals.var_psisubsat * locals.var_psisubsat))), (assign67060_e103808 * ((((-locals.var_t6_dn10) * locals.var_psisubsat) - (assign67060_e103805 * locals.var_psisubsat_dn10)) / (locals.var_psisubsat * locals.var_psisubsat))), (assign67060_e103808 * ((((-locals.var_t6_dn11) * locals.var_psisubsat) - (assign67060_e103805 * locals.var_psisubsat_dn11)) / (locals.var_psisubsat * locals.var_psisubsat))), (assign67060_e103808 * ((((-locals.var_t6_dn14) * locals.var_psisubsat) - (assign67060_e103805 * locals.var_psisubsat_dn14)) / (locals.var_psisubsat * locals.var_psisubsat))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign67060_e103810;
        locals.var_t2_dn0 = assign67060_e103810_d_n0;
        locals.var_t2_dn2 = assign67060_e103810_d_n2;
        locals.var_t2_dn4 = assign67060_e103810_d_n4;
        locals.var_t2_dn5 = assign67060_e103810_d_n5;
        locals.var_t2_dn6 = assign67060_e103810_d_n6;
        locals.var_t2_dn7 = assign67060_e103810_d_n7;
        locals.var_t2_dn8 = assign67060_e103810_d_n8;
        locals.var_t2_dn9 = assign67060_e103810_d_n9;
        locals.var_t2_dn10 = assign67060_e103810_d_n10;
        locals.var_t2_dn11 = assign67060_e103810_d_n11;
        locals.var_t2_dn14 = assign67060_e103810_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign67070_e103823, assign67070_e103823_d_n0, assign67070_e103823_d_n2, assign67070_e103823_d_n4, assign67070_e103823_d_n5, assign67070_e103823_d_n6, assign67070_e103823_d_n7, assign67070_e103823_d_n8, assign67070_e103823_d_n9, assign67070_e103823_d_n10, assign67070_e103823_d_n11, assign67070_e103823_d_n14,) = {
    if ((locals.var_guard1582 == 0.0) && (locals.var_guard1583 != 0.0)) {
        let assign67070_e103817: f64 = (locals.var_t5 * locals.var_psisubsat);
        let assign67070_e103819: f64 = (assign67070_e103817 * locals.var_ids);
        let assign67070_e103821: f64 = (assign67070_e103819 * locals.var_t2);
        (assign67070_e103821, ((((((locals.var_t5_dn0 * locals.var_psisubsat) + (locals.var_t5 * locals.var_psisubsat_dn0)) * locals.var_ids) + (assign67070_e103817 * locals.var_ids_dn0)) * locals.var_t2) + (assign67070_e103819 * locals.var_t2_dn0)), ((((((locals.var_t5_dn2 * locals.var_psisubsat) + (locals.var_t5 * locals.var_psisubsat_dn2)) * locals.var_ids) + (assign67070_e103817 * locals.var_ids_dn2)) * locals.var_t2) + (assign67070_e103819 * locals.var_t2_dn2)), ((((((locals.var_t5_dn4 * locals.var_psisubsat) + (locals.var_t5 * locals.var_psisubsat_dn4)) * locals.var_ids) + (assign67070_e103817 * locals.var_ids_dn4)) * locals.var_t2) + (assign67070_e103819 * locals.var_t2_dn4)), ((((((locals.var_t5_dn5 * locals.var_psisubsat) + (locals.var_t5 * locals.var_psisubsat_dn5)) * locals.var_ids) + (assign67070_e103817 * locals.var_ids_dn5)) * locals.var_t2) + (assign67070_e103819 * locals.var_t2_dn5)), ((((((locals.var_t5_dn6 * locals.var_psisubsat) + (locals.var_t5 * locals.var_psisubsat_dn6)) * locals.var_ids) + (assign67070_e103817 * locals.var_ids_dn6)) * locals.var_t2) + (assign67070_e103819 * locals.var_t2_dn6)), ((((((locals.var_t5_dn7 * locals.var_psisubsat) + (locals.var_t5 * locals.var_psisubsat_dn7)) * locals.var_ids) + (assign67070_e103817 * locals.var_ids_dn7)) * locals.var_t2) + (assign67070_e103819 * locals.var_t2_dn7)), ((((((locals.var_t5_dn8 * locals.var_psisubsat) + (locals.var_t5 * locals.var_psisubsat_dn8)) * locals.var_ids) + (assign67070_e103817 * locals.var_ids_dn8)) * locals.var_t2) + (assign67070_e103819 * locals.var_t2_dn8)), ((((((locals.var_t5_dn9 * locals.var_psisubsat) + (locals.var_t5 * locals.var_psisubsat_dn9)) * locals.var_ids) + (assign67070_e103817 * locals.var_ids_dn9)) * locals.var_t2) + (assign67070_e103819 * locals.var_t2_dn9)), ((((((locals.var_t5_dn10 * locals.var_psisubsat) + (locals.var_t5 * locals.var_psisubsat_dn10)) * locals.var_ids) + (assign67070_e103817 * locals.var_ids_dn10)) * locals.var_t2) + (assign67070_e103819 * locals.var_t2_dn10)), ((((((locals.var_t5_dn11 * locals.var_psisubsat) + (locals.var_t5 * locals.var_psisubsat_dn11)) * locals.var_ids) + (assign67070_e103817 * locals.var_ids_dn11)) * locals.var_t2) + (assign67070_e103819 * locals.var_t2_dn11)), ((((((locals.var_t5_dn14 * locals.var_psisubsat) + (locals.var_t5 * locals.var_psisubsat_dn14)) * locals.var_ids) + (assign67070_e103817 * locals.var_ids_dn14)) * locals.var_t2) + (assign67070_e103819 * locals.var_t2_dn14)),)
    } else {
        (locals.var_isub, locals.var_isub_dn0, locals.var_isub_dn2, locals.var_isub_dn4, locals.var_isub_dn5, locals.var_isub_dn6, locals.var_isub_dn7, locals.var_isub_dn8, locals.var_isub_dn9, locals.var_isub_dn10, locals.var_isub_dn11, locals.var_isub_dn14,)
    }
};
        locals.var_isub = assign67070_e103823;
        locals.var_isub_dn0 = assign67070_e103823_d_n0;
        locals.var_isub_dn2 = assign67070_e103823_d_n2;
        locals.var_isub_dn4 = assign67070_e103823_d_n4;
        locals.var_isub_dn5 = assign67070_e103823_d_n5;
        locals.var_isub_dn6 = assign67070_e103823_d_n6;
        locals.var_isub_dn7 = assign67070_e103823_d_n7;
        locals.var_isub_dn8 = assign67070_e103823_d_n8;
        locals.var_isub_dn9 = assign67070_e103823_d_n9;
        locals.var_isub_dn10 = assign67070_e103823_d_n10;
        locals.var_isub_dn11 = assign67070_e103823_d_n11;
        locals.var_isub_dn14 = assign67070_e103823_d_n14;
        locals.var_isub_rv = 0.0;

        let (assign67080_e103834, assign67080_e103834_d_n0, assign67080_e103834_d_n2, assign67080_e103834_d_n4, assign67080_e103834_d_n5, assign67080_e103834_d_n6, assign67080_e103834_d_n7, assign67080_e103834_d_n8, assign67080_e103834_d_n9, assign67080_e103834_d_n10, assign67080_e103834_d_n11, assign67080_e103834_d_n14,) = {
    if ((locals.var_guard1582 == 0.0) && (locals.var_guard1583 != 0.0)) {
        let assign67080_e103830: f64 = (locals.var_t5 * locals.var_psisubsat);
        let assign67080_e103832: f64 = (assign67080_e103830 * locals.var_t2);
        (assign67080_e103832, ((((locals.var_t5_dn0 * locals.var_psisubsat) + (locals.var_t5 * locals.var_psisubsat_dn0)) * locals.var_t2) + (assign67080_e103830 * locals.var_t2_dn0)), ((((locals.var_t5_dn2 * locals.var_psisubsat) + (locals.var_t5 * locals.var_psisubsat_dn2)) * locals.var_t2) + (assign67080_e103830 * locals.var_t2_dn2)), ((((locals.var_t5_dn4 * locals.var_psisubsat) + (locals.var_t5 * locals.var_psisubsat_dn4)) * locals.var_t2) + (assign67080_e103830 * locals.var_t2_dn4)), ((((locals.var_t5_dn5 * locals.var_psisubsat) + (locals.var_t5 * locals.var_psisubsat_dn5)) * locals.var_t2) + (assign67080_e103830 * locals.var_t2_dn5)), ((((locals.var_t5_dn6 * locals.var_psisubsat) + (locals.var_t5 * locals.var_psisubsat_dn6)) * locals.var_t2) + (assign67080_e103830 * locals.var_t2_dn6)), ((((locals.var_t5_dn7 * locals.var_psisubsat) + (locals.var_t5 * locals.var_psisubsat_dn7)) * locals.var_t2) + (assign67080_e103830 * locals.var_t2_dn7)), ((((locals.var_t5_dn8 * locals.var_psisubsat) + (locals.var_t5 * locals.var_psisubsat_dn8)) * locals.var_t2) + (assign67080_e103830 * locals.var_t2_dn8)), ((((locals.var_t5_dn9 * locals.var_psisubsat) + (locals.var_t5 * locals.var_psisubsat_dn9)) * locals.var_t2) + (assign67080_e103830 * locals.var_t2_dn9)), ((((locals.var_t5_dn10 * locals.var_psisubsat) + (locals.var_t5 * locals.var_psisubsat_dn10)) * locals.var_t2) + (assign67080_e103830 * locals.var_t2_dn10)), ((((locals.var_t5_dn11 * locals.var_psisubsat) + (locals.var_t5 * locals.var_psisubsat_dn11)) * locals.var_t2) + (assign67080_e103830 * locals.var_t2_dn11)), ((((locals.var_t5_dn14 * locals.var_psisubsat) + (locals.var_t5 * locals.var_psisubsat_dn14)) * locals.var_t2) + (assign67080_e103830 * locals.var_t2_dn14)),)
    } else {
        (locals.var_wk_ii, locals.var_wk_ii_dn0, locals.var_wk_ii_dn2, locals.var_wk_ii_dn4, locals.var_wk_ii_dn5, locals.var_wk_ii_dn6, locals.var_wk_ii_dn7, locals.var_wk_ii_dn8, locals.var_wk_ii_dn9, locals.var_wk_ii_dn10, locals.var_wk_ii_dn11, locals.var_wk_ii_dn14,)
    }
};
        locals.var_wk_ii = assign67080_e103834;
        locals.var_wk_ii_dn0 = assign67080_e103834_d_n0;
        locals.var_wk_ii_dn2 = assign67080_e103834_d_n2;
        locals.var_wk_ii_dn4 = assign67080_e103834_d_n4;
        locals.var_wk_ii_dn5 = assign67080_e103834_d_n5;
        locals.var_wk_ii_dn6 = assign67080_e103834_d_n6;
        locals.var_wk_ii_dn7 = assign67080_e103834_d_n7;
        locals.var_wk_ii_dn8 = assign67080_e103834_d_n8;
        locals.var_wk_ii_dn9 = assign67080_e103834_d_n9;
        locals.var_wk_ii_dn10 = assign67080_e103834_d_n10;
        locals.var_wk_ii_dn11 = assign67080_e103834_d_n11;
        locals.var_wk_ii_dn14 = assign67080_e103834_d_n14;
        locals.var_wk_ii_rv = 0.0;

        let (assign67090_e103842, assign67090_e103842_d_n0, assign67090_e103842_d_n2, assign67090_e103842_d_n4, assign67090_e103842_d_n5, assign67090_e103842_d_n6, assign67090_e103842_d_n7, assign67090_e103842_d_n8, assign67090_e103842_d_n9, assign67090_e103842_d_n10, assign67090_e103842_d_n11, assign67090_e103842_d_n14,) = {
    if ((locals.var_guard1582 == 0.0) && (locals.var_guard1583 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isub, locals.var_isub_dn0, locals.var_isub_dn2, locals.var_isub_dn4, locals.var_isub_dn5, locals.var_isub_dn6, locals.var_isub_dn7, locals.var_isub_dn8, locals.var_isub_dn9, locals.var_isub_dn10, locals.var_isub_dn11, locals.var_isub_dn14,)
    }
};
        locals.var_isub = assign67090_e103842;
        locals.var_isub_dn0 = assign67090_e103842_d_n0;
        locals.var_isub_dn2 = assign67090_e103842_d_n2;
        locals.var_isub_dn4 = assign67090_e103842_d_n4;
        locals.var_isub_dn5 = assign67090_e103842_d_n5;
        locals.var_isub_dn6 = assign67090_e103842_d_n6;
        locals.var_isub_dn7 = assign67090_e103842_d_n7;
        locals.var_isub_dn8 = assign67090_e103842_d_n8;
        locals.var_isub_dn9 = assign67090_e103842_d_n9;
        locals.var_isub_dn10 = assign67090_e103842_d_n10;
        locals.var_isub_dn11 = assign67090_e103842_d_n11;
        locals.var_isub_dn14 = assign67090_e103842_d_n14;
        locals.var_isub_rv = 0.0;

        let assign67100_e103845: f64 = if locals.var_uc_subld1 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1591 = assign67100_e103845;
        locals.var_guard1591_rv = 0.0;

        let (assign67110_e103852, assign67110_e103852_d_n0, assign67110_e103852_d_n2, assign67110_e103852_d_n4, assign67110_e103852_d_n5, assign67110_e103852_d_n6, assign67110_e103852_d_n7, assign67110_e103852_d_n8, assign67110_e103852_d_n9, assign67110_e103852_d_n10, assign67110_e103852_d_n11, assign67110_e103852_d_n14,) = {
    if ((locals.var_guard1582 == 0.0) && (locals.var_guard1591 != 0.0)) {
        (locals.var_vddp, locals.var_vddp_dn0, 0.0, 0.0, 0.0, locals.var_vddp_dn6, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign67110_e103852;
        locals.var_t0_dn0 = assign67110_e103852_d_n0;
        locals.var_t0_dn2 = assign67110_e103852_d_n2;
        locals.var_t0_dn4 = assign67110_e103852_d_n4;
        locals.var_t0_dn5 = assign67110_e103852_d_n5;
        locals.var_t0_dn6 = assign67110_e103852_d_n6;
        locals.var_t0_dn7 = assign67110_e103852_d_n7;
        locals.var_t0_dn8 = assign67110_e103852_d_n8;
        locals.var_t0_dn9 = assign67110_e103852_d_n9;
        locals.var_t0_dn10 = assign67110_e103852_d_n10;
        locals.var_t0_dn11 = assign67110_e103852_d_n11;
        locals.var_t0_dn14 = assign67110_e103852_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign67120_e103868, assign67120_e103868_d_n0, assign67120_e103868_d_n2, assign67120_e103868_d_n4, assign67120_e103868_d_n5, assign67120_e103868_d_n6, assign67120_e103868_d_n7, assign67120_e103868_d_n8, assign67120_e103868_d_n9, assign67120_e103868_d_n10, assign67120_e103868_d_n11, assign67120_e103868_d_n14,) = {
    if ((locals.var_guard1582 == 0.0) && (locals.var_guard1591 != 0.0)) {
        let assign67120_e103859: f64 = (locals.var_t0 * locals.var_t0);
        let assign67120_e103862: f64 = (4.0 * 1e-6);
        let assign67120_e103864: f64 = (assign67120_e103862 * 1e-6);
        let assign67120_e103865: f64 = (assign67120_e103859 + assign67120_e103864);
        let assign67120_e103866: f64 = (assign67120_e103865).sqrt();
        (assign67120_e103866, (((locals.var_t0_dn0 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn0)) / (2.0 * assign67120_e103866)), (((locals.var_t0_dn2 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn2)) / (2.0 * assign67120_e103866)), (((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)) / (2.0 * assign67120_e103866)), (((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)) / (2.0 * assign67120_e103866)), (((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)) / (2.0 * assign67120_e103866)), (((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)) / (2.0 * assign67120_e103866)), (((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)) / (2.0 * assign67120_e103866)), (((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)) / (2.0 * assign67120_e103866)), (((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)) / (2.0 * assign67120_e103866)), (((locals.var_t0_dn11 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn11)) / (2.0 * assign67120_e103866)), (((locals.var_t0_dn14 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn14)) / (2.0 * assign67120_e103866)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign67120_e103868;
        locals.var_tmf2_dn0 = assign67120_e103868_d_n0;
        locals.var_tmf2_dn2 = assign67120_e103868_d_n2;
        locals.var_tmf2_dn4 = assign67120_e103868_d_n4;
        locals.var_tmf2_dn5 = assign67120_e103868_d_n5;
        locals.var_tmf2_dn6 = assign67120_e103868_d_n6;
        locals.var_tmf2_dn7 = assign67120_e103868_d_n7;
        locals.var_tmf2_dn8 = assign67120_e103868_d_n8;
        locals.var_tmf2_dn9 = assign67120_e103868_d_n9;
        locals.var_tmf2_dn10 = assign67120_e103868_d_n10;
        locals.var_tmf2_dn11 = assign67120_e103868_d_n11;
        locals.var_tmf2_dn14 = assign67120_e103868_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign67130_e103881, assign67130_e103881_d_n0, assign67130_e103881_d_n2, assign67130_e103881_d_n4, assign67130_e103881_d_n5, assign67130_e103881_d_n6, assign67130_e103881_d_n7, assign67130_e103881_d_n8, assign67130_e103881_d_n9, assign67130_e103881_d_n10, assign67130_e103881_d_n11, assign67130_e103881_d_n14,) = {
    if ((locals.var_guard1582 == 0.0) && (locals.var_guard1591 != 0.0)) {
        let assign67130_e103877: f64 = (locals.var_t0 / locals.var_tmf2);
        let assign67130_e103878: f64 = (1.0 + assign67130_e103877);
        let assign67130_e103879: f64 = (0.5 * assign67130_e103878);
        (assign67130_e103879, (0.5 * (((locals.var_t0_dn0 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn2 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn4 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn5 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn6 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn7 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn8 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn9 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn10 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn11 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn14 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign67130_e103881;
        locals.var_t1_dn0 = assign67130_e103881_d_n0;
        locals.var_t1_dn2 = assign67130_e103881_d_n2;
        locals.var_t1_dn4 = assign67130_e103881_d_n4;
        locals.var_t1_dn5 = assign67130_e103881_d_n5;
        locals.var_t1_dn6 = assign67130_e103881_d_n6;
        locals.var_t1_dn7 = assign67130_e103881_d_n7;
        locals.var_t1_dn8 = assign67130_e103881_d_n8;
        locals.var_t1_dn9 = assign67130_e103881_d_n9;
        locals.var_t1_dn10 = assign67130_e103881_d_n10;
        locals.var_t1_dn11 = assign67130_e103881_d_n11;
        locals.var_t1_dn14 = assign67130_e103881_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign67140_e103892, assign67140_e103892_d_n0, assign67140_e103892_d_n2, assign67140_e103892_d_n4, assign67140_e103892_d_n5, assign67140_e103892_d_n6, assign67140_e103892_d_n7, assign67140_e103892_d_n8, assign67140_e103892_d_n9, assign67140_e103892_d_n10, assign67140_e103892_d_n11, assign67140_e103892_d_n14,) = {
    if ((locals.var_guard1582 == 0.0) && (locals.var_guard1591 != 0.0)) {
        let assign67140_e103889: f64 = (locals.var_t0 + locals.var_tmf2);
        let assign67140_e103890: f64 = (0.5 * assign67140_e103889);
        (assign67140_e103890, (0.5 * (locals.var_t0_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t0_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t0_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t0_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t0_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t0_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t0_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t0_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t0_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t0_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_t0_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign67140_e103892;
        locals.var_t0_dn0 = assign67140_e103892_d_n0;
        locals.var_t0_dn2 = assign67140_e103892_d_n2;
        locals.var_t0_dn4 = assign67140_e103892_d_n4;
        locals.var_t0_dn5 = assign67140_e103892_d_n5;
        locals.var_t0_dn6 = assign67140_e103892_d_n6;
        locals.var_t0_dn7 = assign67140_e103892_d_n7;
        locals.var_t0_dn8 = assign67140_e103892_d_n8;
        locals.var_t0_dn9 = assign67140_e103892_d_n9;
        locals.var_t0_dn10 = assign67140_e103892_d_n10;
        locals.var_t0_dn11 = assign67140_e103892_d_n11;
        locals.var_t0_dn14 = assign67140_e103892_d_n14;
        locals.var_t0_rv = 0.0;

        let assign67150_e103895: f64 = if locals.var_t0 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1592 = assign67150_e103895;
        locals.var_guard1592_rv = 0.0;

        let (assign67160_e103904, assign67160_e103904_d_n0, assign67160_e103904_d_n2, assign67160_e103904_d_n4, assign67160_e103904_d_n5, assign67160_e103904_d_n6, assign67160_e103904_d_n7, assign67160_e103904_d_n8, assign67160_e103904_d_n9, assign67160_e103904_d_n10, assign67160_e103904_d_n11, assign67160_e103904_d_n14,) = {
    if (((locals.var_guard1582 == 0.0) && (locals.var_guard1591 != 0.0)) && (locals.var_guard1592 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign67160_e103904;
        locals.var_t0_dn0 = assign67160_e103904_d_n0;
        locals.var_t0_dn2 = assign67160_e103904_d_n2;
        locals.var_t0_dn4 = assign67160_e103904_d_n4;
        locals.var_t0_dn5 = assign67160_e103904_d_n5;
        locals.var_t0_dn6 = assign67160_e103904_d_n6;
        locals.var_t0_dn7 = assign67160_e103904_d_n7;
        locals.var_t0_dn8 = assign67160_e103904_d_n8;
        locals.var_t0_dn9 = assign67160_e103904_d_n9;
        locals.var_t0_dn10 = assign67160_e103904_d_n10;
        locals.var_t0_dn11 = assign67160_e103904_d_n11;
        locals.var_t0_dn14 = assign67160_e103904_d_n14;
        locals.var_t0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_253(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign67170_e103913, assign67170_e103913_d_n0, assign67170_e103913_d_n2, assign67170_e103913_d_n4, assign67170_e103913_d_n5, assign67170_e103913_d_n6, assign67170_e103913_d_n7, assign67170_e103913_d_n8, assign67170_e103913_d_n9, assign67170_e103913_d_n10, assign67170_e103913_d_n11, assign67170_e103913_d_n14,) = {
    if (((locals.var_guard1582 == 0.0) && (locals.var_guard1591 != 0.0)) && (locals.var_guard1592 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign67170_e103913;
        locals.var_t1_dn0 = assign67170_e103913_d_n0;
        locals.var_t1_dn2 = assign67170_e103913_d_n2;
        locals.var_t1_dn4 = assign67170_e103913_d_n4;
        locals.var_t1_dn5 = assign67170_e103913_d_n5;
        locals.var_t1_dn6 = assign67170_e103913_d_n6;
        locals.var_t1_dn7 = assign67170_e103913_d_n7;
        locals.var_t1_dn8 = assign67170_e103913_d_n8;
        locals.var_t1_dn9 = assign67170_e103913_d_n9;
        locals.var_t1_dn10 = assign67170_e103913_d_n10;
        locals.var_t1_dn11 = assign67170_e103913_d_n11;
        locals.var_t1_dn14 = assign67170_e103913_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign67180_e103923, assign67180_e103923_d_n0, assign67180_e103923_d_n2, assign67180_e103923_d_n4, assign67180_e103923_d_n5, assign67180_e103923_d_n6, assign67180_e103923_d_n7, assign67180_e103923_d_n8, assign67180_e103923_d_n9, assign67180_e103923_d_n10, assign67180_e103923_d_n11, assign67180_e103923_d_n14,) = {
    if ((locals.var_guard1582 == 0.0) && (locals.var_guard1591 != 0.0)) {
        let assign67180_e103920: f64 = (locals.var_vgvt + 1e-25);
        let assign67180_e103921: f64 = (assign67180_e103920).sqrt();
        (assign67180_e103921, (locals.var_vgvt_dn0 / (2.0 * assign67180_e103921)), (locals.var_vgvt_dn2 / (2.0 * assign67180_e103921)), (locals.var_vgvt_dn4 / (2.0 * assign67180_e103921)), (locals.var_vgvt_dn5 / (2.0 * assign67180_e103921)), (locals.var_vgvt_dn6 / (2.0 * assign67180_e103921)), (locals.var_vgvt_dn7 / (2.0 * assign67180_e103921)), (locals.var_vgvt_dn8 / (2.0 * assign67180_e103921)), (locals.var_vgvt_dn9 / (2.0 * assign67180_e103921)), (locals.var_vgvt_dn10 / (2.0 * assign67180_e103921)), (locals.var_vgvt_dn11 / (2.0 * assign67180_e103921)), (locals.var_vgvt_dn14 / (2.0 * assign67180_e103921)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign67180_e103923;
        locals.var_t1_dn0 = assign67180_e103923_d_n0;
        locals.var_t1_dn2 = assign67180_e103923_d_n2;
        locals.var_t1_dn4 = assign67180_e103923_d_n4;
        locals.var_t1_dn5 = assign67180_e103923_d_n5;
        locals.var_t1_dn6 = assign67180_e103923_d_n6;
        locals.var_t1_dn7 = assign67180_e103923_d_n7;
        locals.var_t1_dn8 = assign67180_e103923_d_n8;
        locals.var_t1_dn9 = assign67180_e103923_d_n9;
        locals.var_t1_dn10 = assign67180_e103923_d_n10;
        locals.var_t1_dn11 = assign67180_e103923_d_n11;
        locals.var_t1_dn14 = assign67180_e103923_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign67190_e103934, assign67190_e103934_d_n0, assign67190_e103934_d_n2, assign67190_e103934_d_n4, assign67190_e103934_d_n5, assign67190_e103934_d_n6, assign67190_e103934_d_n7, assign67190_e103934_d_n8, assign67190_e103934_d_n9, assign67190_e103934_d_n10, assign67190_e103934_d_n11, assign67190_e103934_d_n14,) = {
    if ((locals.var_guard1582 == 0.0) && (locals.var_guard1591 != 0.0)) {
        let assign67190_e103931: f64 = (2.0 * locals.var_t1);
        let assign67190_e103932: f64 = (1.0 / assign67190_e103931);
        (assign67190_e103932, (-((2.0 * locals.var_t1_dn0) / (assign67190_e103931 * assign67190_e103931))), (-((2.0 * locals.var_t1_dn2) / (assign67190_e103931 * assign67190_e103931))), (-((2.0 * locals.var_t1_dn4) / (assign67190_e103931 * assign67190_e103931))), (-((2.0 * locals.var_t1_dn5) / (assign67190_e103931 * assign67190_e103931))), (-((2.0 * locals.var_t1_dn6) / (assign67190_e103931 * assign67190_e103931))), (-((2.0 * locals.var_t1_dn7) / (assign67190_e103931 * assign67190_e103931))), (-((2.0 * locals.var_t1_dn8) / (assign67190_e103931 * assign67190_e103931))), (-((2.0 * locals.var_t1_dn9) / (assign67190_e103931 * assign67190_e103931))), (-((2.0 * locals.var_t1_dn10) / (assign67190_e103931 * assign67190_e103931))), (-((2.0 * locals.var_t1_dn11) / (assign67190_e103931 * assign67190_e103931))), (-((2.0 * locals.var_t1_dn14) / (assign67190_e103931 * assign67190_e103931))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign67190_e103934;
        locals.var_t3_dn0 = assign67190_e103934_d_n0;
        locals.var_t3_dn2 = assign67190_e103934_d_n2;
        locals.var_t3_dn4 = assign67190_e103934_d_n4;
        locals.var_t3_dn5 = assign67190_e103934_d_n5;
        locals.var_t3_dn6 = assign67190_e103934_d_n6;
        locals.var_t3_dn7 = assign67190_e103934_d_n7;
        locals.var_t3_dn8 = assign67190_e103934_d_n8;
        locals.var_t3_dn9 = assign67190_e103934_d_n9;
        locals.var_t3_dn10 = assign67190_e103934_d_n10;
        locals.var_t3_dn11 = assign67190_e103934_d_n11;
        locals.var_t3_dn14 = assign67190_e103934_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign67200_e103949, assign67200_e103949_d_n0, assign67200_e103949_d_n2, assign67200_e103949_d_n4, assign67200_e103949_d_n5, assign67200_e103949_d_n6, assign67200_e103949_d_n7, assign67200_e103949_d_n8, assign67200_e103949_d_n9, assign67200_e103949_d_n10, assign67200_e103949_d_n11, assign67200_e103949_d_n14,) = {
    if ((locals.var_guard1582 == 0.0) && (locals.var_guard1591 != 0.0)) {
        let assign67200_e103944: f64 = (p.p106 * locals.var_vgs);
        let assign67200_e103945: f64 = (1.0 + assign67200_e103944);
        let assign67200_e103946: f64 = (p.p105 * assign67200_e103945);
        let assign67200_e103947: f64 = (locals.var_t0 - assign67200_e103946);
        (assign67200_e103947, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, (locals.var_t0_dn6 - (p.p105 * (p.p106 * locals.var_vgs_dn6))), (locals.var_t0_dn7 - (p.p105 * (p.p106 * locals.var_vgs_dn7))), (locals.var_t0_dn8 - (p.p105 * (p.p106 * locals.var_vgs_dn8))), locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign67200_e103949;
        locals.var_t4_dn0 = assign67200_e103949_d_n0;
        locals.var_t4_dn2 = assign67200_e103949_d_n2;
        locals.var_t4_dn4 = assign67200_e103949_d_n4;
        locals.var_t4_dn5 = assign67200_e103949_d_n5;
        locals.var_t4_dn6 = assign67200_e103949_d_n6;
        locals.var_t4_dn7 = assign67200_e103949_d_n7;
        locals.var_t4_dn8 = assign67200_e103949_d_n8;
        locals.var_t4_dn9 = assign67200_e103949_d_n9;
        locals.var_t4_dn10 = assign67200_e103949_d_n10;
        locals.var_t4_dn11 = assign67200_e103949_d_n11;
        locals.var_t4_dn14 = assign67200_e103949_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign67210_e103965, assign67210_e103965_d_n0, assign67210_e103965_d_n2, assign67210_e103965_d_n4, assign67210_e103965_d_n5, assign67210_e103965_d_n6, assign67210_e103965_d_n7, assign67210_e103965_d_n8, assign67210_e103965_d_n9, assign67210_e103965_d_n10, assign67210_e103965_d_n11, assign67210_e103965_d_n14,) = {
    if ((locals.var_guard1582 == 0.0) && (locals.var_guard1591 != 0.0)) {
        let assign67210_e103956: f64 = (locals.var_t4 * locals.var_t4);
        let assign67210_e103959: f64 = (4.0 * 0.01);
        let assign67210_e103961: f64 = (assign67210_e103959 * 0.01);
        let assign67210_e103962: f64 = (assign67210_e103956 + assign67210_e103961);
        let assign67210_e103963: f64 = (assign67210_e103962).sqrt();
        (assign67210_e103963, (((locals.var_t4_dn0 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn0)) / (2.0 * assign67210_e103963)), (((locals.var_t4_dn2 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn2)) / (2.0 * assign67210_e103963)), (((locals.var_t4_dn4 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn4)) / (2.0 * assign67210_e103963)), (((locals.var_t4_dn5 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn5)) / (2.0 * assign67210_e103963)), (((locals.var_t4_dn6 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn6)) / (2.0 * assign67210_e103963)), (((locals.var_t4_dn7 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn7)) / (2.0 * assign67210_e103963)), (((locals.var_t4_dn8 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn8)) / (2.0 * assign67210_e103963)), (((locals.var_t4_dn9 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn9)) / (2.0 * assign67210_e103963)), (((locals.var_t4_dn10 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn10)) / (2.0 * assign67210_e103963)), (((locals.var_t4_dn11 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn11)) / (2.0 * assign67210_e103963)), (((locals.var_t4_dn14 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn14)) / (2.0 * assign67210_e103963)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign67210_e103965;
        locals.var_tmf2_dn0 = assign67210_e103965_d_n0;
        locals.var_tmf2_dn2 = assign67210_e103965_d_n2;
        locals.var_tmf2_dn4 = assign67210_e103965_d_n4;
        locals.var_tmf2_dn5 = assign67210_e103965_d_n5;
        locals.var_tmf2_dn6 = assign67210_e103965_d_n6;
        locals.var_tmf2_dn7 = assign67210_e103965_d_n7;
        locals.var_tmf2_dn8 = assign67210_e103965_d_n8;
        locals.var_tmf2_dn9 = assign67210_e103965_d_n9;
        locals.var_tmf2_dn10 = assign67210_e103965_d_n10;
        locals.var_tmf2_dn11 = assign67210_e103965_d_n11;
        locals.var_tmf2_dn14 = assign67210_e103965_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign67220_e103978, assign67220_e103978_d_n0, assign67220_e103978_d_n2, assign67220_e103978_d_n4, assign67220_e103978_d_n5, assign67220_e103978_d_n6, assign67220_e103978_d_n7, assign67220_e103978_d_n8, assign67220_e103978_d_n9, assign67220_e103978_d_n10, assign67220_e103978_d_n11, assign67220_e103978_d_n14,) = {
    if ((locals.var_guard1582 == 0.0) && (locals.var_guard1591 != 0.0)) {
        let assign67220_e103974: f64 = (locals.var_t4 / locals.var_tmf2);
        let assign67220_e103975: f64 = (1.0 + assign67220_e103974);
        let assign67220_e103976: f64 = (0.5 * assign67220_e103975);
        (assign67220_e103976, (0.5 * (((locals.var_t4_dn0 * locals.var_tmf2) - (locals.var_t4 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4_dn2 * locals.var_tmf2) - (locals.var_t4 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4_dn4 * locals.var_tmf2) - (locals.var_t4 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4_dn5 * locals.var_tmf2) - (locals.var_t4 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4_dn6 * locals.var_tmf2) - (locals.var_t4 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4_dn7 * locals.var_tmf2) - (locals.var_t4 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4_dn8 * locals.var_tmf2) - (locals.var_t4 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4_dn9 * locals.var_tmf2) - (locals.var_t4 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4_dn10 * locals.var_tmf2) - (locals.var_t4 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4_dn11 * locals.var_tmf2) - (locals.var_t4 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4_dn14 * locals.var_tmf2) - (locals.var_t4 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign67220_e103978;
        locals.var_t9_dn0 = assign67220_e103978_d_n0;
        locals.var_t9_dn2 = assign67220_e103978_d_n2;
        locals.var_t9_dn4 = assign67220_e103978_d_n4;
        locals.var_t9_dn5 = assign67220_e103978_d_n5;
        locals.var_t9_dn6 = assign67220_e103978_d_n6;
        locals.var_t9_dn7 = assign67220_e103978_d_n7;
        locals.var_t9_dn8 = assign67220_e103978_d_n8;
        locals.var_t9_dn9 = assign67220_e103978_d_n9;
        locals.var_t9_dn10 = assign67220_e103978_d_n10;
        locals.var_t9_dn11 = assign67220_e103978_d_n11;
        locals.var_t9_dn14 = assign67220_e103978_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign67230_e103989, assign67230_e103989_d_n0, assign67230_e103989_d_n2, assign67230_e103989_d_n4, assign67230_e103989_d_n5, assign67230_e103989_d_n6, assign67230_e103989_d_n7, assign67230_e103989_d_n8, assign67230_e103989_d_n9, assign67230_e103989_d_n10, assign67230_e103989_d_n11, assign67230_e103989_d_n14,) = {
    if ((locals.var_guard1582 == 0.0) && (locals.var_guard1591 != 0.0)) {
        let assign67230_e103986: f64 = (locals.var_t4 + locals.var_tmf2);
        let assign67230_e103987: f64 = (0.5 * assign67230_e103986);
        (assign67230_e103987, (0.5 * (locals.var_t4_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t4_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t4_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t4_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t4_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t4_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t4_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t4_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t4_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t4_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_t4_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign67230_e103989;
        locals.var_t4_dn0 = assign67230_e103989_d_n0;
        locals.var_t4_dn2 = assign67230_e103989_d_n2;
        locals.var_t4_dn4 = assign67230_e103989_d_n4;
        locals.var_t4_dn5 = assign67230_e103989_d_n5;
        locals.var_t4_dn6 = assign67230_e103989_d_n6;
        locals.var_t4_dn7 = assign67230_e103989_d_n7;
        locals.var_t4_dn8 = assign67230_e103989_d_n8;
        locals.var_t4_dn9 = assign67230_e103989_d_n9;
        locals.var_t4_dn10 = assign67230_e103989_d_n10;
        locals.var_t4_dn11 = assign67230_e103989_d_n11;
        locals.var_t4_dn14 = assign67230_e103989_d_n14;
        locals.var_t4_rv = 0.0;

        let assign67240_e103992: f64 = if locals.var_t4 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1593 = assign67240_e103992;
        locals.var_guard1593_rv = 0.0;

        let (assign67250_e104001, assign67250_e104001_d_n0, assign67250_e104001_d_n2, assign67250_e104001_d_n4, assign67250_e104001_d_n5, assign67250_e104001_d_n6, assign67250_e104001_d_n7, assign67250_e104001_d_n8, assign67250_e104001_d_n9, assign67250_e104001_d_n10, assign67250_e104001_d_n11, assign67250_e104001_d_n14,) = {
    if (((locals.var_guard1582 == 0.0) && (locals.var_guard1591 != 0.0)) && (locals.var_guard1593 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign67250_e104001;
        locals.var_t4_dn0 = assign67250_e104001_d_n0;
        locals.var_t4_dn2 = assign67250_e104001_d_n2;
        locals.var_t4_dn4 = assign67250_e104001_d_n4;
        locals.var_t4_dn5 = assign67250_e104001_d_n5;
        locals.var_t4_dn6 = assign67250_e104001_d_n6;
        locals.var_t4_dn7 = assign67250_e104001_d_n7;
        locals.var_t4_dn8 = assign67250_e104001_d_n8;
        locals.var_t4_dn9 = assign67250_e104001_d_n9;
        locals.var_t4_dn10 = assign67250_e104001_d_n10;
        locals.var_t4_dn11 = assign67250_e104001_d_n11;
        locals.var_t4_dn14 = assign67250_e104001_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign67260_e104010, assign67260_e104010_d_n0, assign67260_e104010_d_n2, assign67260_e104010_d_n4, assign67260_e104010_d_n5, assign67260_e104010_d_n6, assign67260_e104010_d_n7, assign67260_e104010_d_n8, assign67260_e104010_d_n9, assign67260_e104010_d_n10, assign67260_e104010_d_n11, assign67260_e104010_d_n14,) = {
    if (((locals.var_guard1582 == 0.0) && (locals.var_guard1591 != 0.0)) && (locals.var_guard1593 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign67260_e104010;
        locals.var_t9_dn0 = assign67260_e104010_d_n0;
        locals.var_t9_dn2 = assign67260_e104010_d_n2;
        locals.var_t9_dn4 = assign67260_e104010_d_n4;
        locals.var_t9_dn5 = assign67260_e104010_d_n5;
        locals.var_t9_dn6 = assign67260_e104010_d_n6;
        locals.var_t9_dn7 = assign67260_e104010_d_n7;
        locals.var_t9_dn8 = assign67260_e104010_d_n8;
        locals.var_t9_dn9 = assign67260_e104010_d_n9;
        locals.var_t9_dn10 = assign67260_e104010_d_n10;
        locals.var_t9_dn11 = assign67260_e104010_d_n11;
        locals.var_t9_dn14 = assign67260_e104010_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign67270_e104019, assign67270_e104019_d_n0, assign67270_e104019_d_n2, assign67270_e104019_d_n4, assign67270_e104019_d_n5, assign67270_e104019_d_n6, assign67270_e104019_d_n7, assign67270_e104019_d_n8, assign67270_e104019_d_n9, assign67270_e104019_d_n10, assign67270_e104019_d_n11, assign67270_e104019_d_n14,) = {
    if ((locals.var_guard1582 == 0.0) && (locals.var_guard1591 != 0.0)) {
        let assign67270_e104017: f64 = (locals.var_t4 + 1e-25);
        (assign67270_e104017, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign67270_e104019;
        locals.var_t4_dn0 = assign67270_e104019_d_n0;
        locals.var_t4_dn2 = assign67270_e104019_d_n2;
        locals.var_t4_dn4 = assign67270_e104019_d_n4;
        locals.var_t4_dn5 = assign67270_e104019_d_n5;
        locals.var_t4_dn6 = assign67270_e104019_d_n6;
        locals.var_t4_dn7 = assign67270_e104019_d_n7;
        locals.var_t4_dn8 = assign67270_e104019_d_n8;
        locals.var_t4_dn9 = assign67270_e104019_d_n9;
        locals.var_t4_dn10 = assign67270_e104019_d_n10;
        locals.var_t4_dn11 = assign67270_e104019_d_n11;
        locals.var_t4_dn14 = assign67270_e104019_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign67280_e104034, assign67280_e104034_d_n0, assign67280_e104034_d_n2, assign67280_e104034_d_n4, assign67280_e104034_d_n5, assign67280_e104034_d_n6, assign67280_e104034_d_n7, assign67280_e104034_d_n8, assign67280_e104034_d_n9, assign67280_e104034_d_n10, assign67280_e104034_d_n11, assign67280_e104034_d_n14,) = {
    if ((locals.var_guard1582 == 0.0) && (locals.var_guard1591 != 0.0)) {
        let assign67280_e104026: f64 = (locals.var_uc_xpdv * locals.var_uc_xldld);
        let assign67280_e104028: f64 = (-1.0);
        let assign67280_e104030: f64 = (assign67280_e104028 / locals.var_t4);
        let assign67280_e104031: f64 = (assign67280_e104030).exp();
        let assign67280_e104032: f64 = (assign67280_e104026 * assign67280_e104031);
        (assign67280_e104032, (assign67280_e104026 * (assign67280_e104031 * (-((assign67280_e104028 * locals.var_t4_dn0) / (locals.var_t4 * locals.var_t4))))), (assign67280_e104026 * (assign67280_e104031 * (-((assign67280_e104028 * locals.var_t4_dn2) / (locals.var_t4 * locals.var_t4))))), (assign67280_e104026 * (assign67280_e104031 * (-((assign67280_e104028 * locals.var_t4_dn4) / (locals.var_t4 * locals.var_t4))))), (assign67280_e104026 * (assign67280_e104031 * (-((assign67280_e104028 * locals.var_t4_dn5) / (locals.var_t4 * locals.var_t4))))), (assign67280_e104026 * (assign67280_e104031 * (-((assign67280_e104028 * locals.var_t4_dn6) / (locals.var_t4 * locals.var_t4))))), (assign67280_e104026 * (assign67280_e104031 * (-((assign67280_e104028 * locals.var_t4_dn7) / (locals.var_t4 * locals.var_t4))))), (assign67280_e104026 * (assign67280_e104031 * (-((assign67280_e104028 * locals.var_t4_dn8) / (locals.var_t4 * locals.var_t4))))), (assign67280_e104026 * (assign67280_e104031 * (-((assign67280_e104028 * locals.var_t4_dn9) / (locals.var_t4 * locals.var_t4))))), (assign67280_e104026 * (assign67280_e104031 * (-((assign67280_e104028 * locals.var_t4_dn10) / (locals.var_t4 * locals.var_t4))))), (assign67280_e104026 * (assign67280_e104031 * (-((assign67280_e104028 * locals.var_t4_dn11) / (locals.var_t4 * locals.var_t4))))), (assign67280_e104026 * (assign67280_e104031 * (-((assign67280_e104028 * locals.var_t4_dn14) / (locals.var_t4 * locals.var_t4))))),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign67280_e104034;
        locals.var_t10_dn0 = assign67280_e104034_d_n0;
        locals.var_t10_dn2 = assign67280_e104034_d_n2;
        locals.var_t10_dn4 = assign67280_e104034_d_n4;
        locals.var_t10_dn5 = assign67280_e104034_d_n5;
        locals.var_t10_dn6 = assign67280_e104034_d_n6;
        locals.var_t10_dn7 = assign67280_e104034_d_n7;
        locals.var_t10_dn8 = assign67280_e104034_d_n8;
        locals.var_t10_dn9 = assign67280_e104034_d_n9;
        locals.var_t10_dn10 = assign67280_e104034_d_n10;
        locals.var_t10_dn11 = assign67280_e104034_d_n11;
        locals.var_t10_dn14 = assign67280_e104034_d_n14;
        locals.var_t10_rv = 0.0;

        let (assign67290_e104047, assign67290_e104047_d_n0, assign67290_e104047_d_n2, assign67290_e104047_d_n4, assign67290_e104047_d_n5, assign67290_e104047_d_n6, assign67290_e104047_d_n7, assign67290_e104047_d_n8, assign67290_e104047_d_n9, assign67290_e104047_d_n10, assign67290_e104047_d_n11, assign67290_e104047_d_n14,) = {
    if ((locals.var_guard1582 == 0.0) && (locals.var_guard1591 != 0.0)) {
        let assign67290_e104043: f64 = (1.0 / locals.var_t4);
        let assign67290_e104044: f64 = (1.0 + assign67290_e104043);
        let assign67290_e104045: f64 = (locals.var_t10 * assign67290_e104044);
        (assign67290_e104045, ((locals.var_t10_dn0 * assign67290_e104044) + (locals.var_t10 * (-(locals.var_t4_dn0 / (locals.var_t4 * locals.var_t4))))), ((locals.var_t10_dn2 * assign67290_e104044) + (locals.var_t10 * (-(locals.var_t4_dn2 / (locals.var_t4 * locals.var_t4))))), ((locals.var_t10_dn4 * assign67290_e104044) + (locals.var_t10 * (-(locals.var_t4_dn4 / (locals.var_t4 * locals.var_t4))))), ((locals.var_t10_dn5 * assign67290_e104044) + (locals.var_t10 * (-(locals.var_t4_dn5 / (locals.var_t4 * locals.var_t4))))), ((locals.var_t10_dn6 * assign67290_e104044) + (locals.var_t10 * (-(locals.var_t4_dn6 / (locals.var_t4 * locals.var_t4))))), ((locals.var_t10_dn7 * assign67290_e104044) + (locals.var_t10 * (-(locals.var_t4_dn7 / (locals.var_t4 * locals.var_t4))))), ((locals.var_t10_dn8 * assign67290_e104044) + (locals.var_t10 * (-(locals.var_t4_dn8 / (locals.var_t4 * locals.var_t4))))), ((locals.var_t10_dn9 * assign67290_e104044) + (locals.var_t10 * (-(locals.var_t4_dn9 / (locals.var_t4 * locals.var_t4))))), ((locals.var_t10_dn10 * assign67290_e104044) + (locals.var_t10 * (-(locals.var_t4_dn10 / (locals.var_t4 * locals.var_t4))))), ((locals.var_t10_dn11 * assign67290_e104044) + (locals.var_t10 * (-(locals.var_t4_dn11 / (locals.var_t4 * locals.var_t4))))), ((locals.var_t10_dn14 * assign67290_e104044) + (locals.var_t10 * (-(locals.var_t4_dn14 / (locals.var_t4 * locals.var_t4))))),)
    } else {
        (locals.var_t11, locals.var_t11_dn0, locals.var_t11_dn2, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11, locals.var_t11_dn14,)
    }
};
        locals.var_t11 = assign67290_e104047;
        locals.var_t11_dn0 = assign67290_e104047_d_n0;
        locals.var_t11_dn2 = assign67290_e104047_d_n2;
        locals.var_t11_dn4 = assign67290_e104047_d_n4;
        locals.var_t11_dn5 = assign67290_e104047_d_n5;
        locals.var_t11_dn6 = assign67290_e104047_d_n6;
        locals.var_t11_dn7 = assign67290_e104047_d_n7;
        locals.var_t11_dn8 = assign67290_e104047_d_n8;
        locals.var_t11_dn9 = assign67290_e104047_d_n9;
        locals.var_t11_dn10 = assign67290_e104047_d_n10;
        locals.var_t11_dn11 = assign67290_e104047_d_n11;
        locals.var_t11_dn14 = assign67290_e104047_d_n14;
        locals.var_t11_rv = 0.0;

        let (assign67300_e104056, assign67300_e104056_d_n0, assign67300_e104056_d_n2, assign67300_e104056_d_n4, assign67300_e104056_d_n5, assign67300_e104056_d_n6, assign67300_e104056_d_n7, assign67300_e104056_d_n8, assign67300_e104056_d_n9, assign67300_e104056_d_n10, assign67300_e104056_d_n11, assign67300_e104056_d_n14,) = {
    if ((locals.var_guard1582 == 0.0) && (locals.var_guard1591 != 0.0)) {
        let assign67300_e104054: f64 = (locals.var_t4 * locals.var_t10);
        (assign67300_e104054, ((locals.var_t4_dn0 * locals.var_t10) + (locals.var_t4 * locals.var_t10_dn0)), ((locals.var_t4_dn2 * locals.var_t10) + (locals.var_t4 * locals.var_t10_dn2)), ((locals.var_t4_dn4 * locals.var_t10) + (locals.var_t4 * locals.var_t10_dn4)), ((locals.var_t4_dn5 * locals.var_t10) + (locals.var_t4 * locals.var_t10_dn5)), ((locals.var_t4_dn6 * locals.var_t10) + (locals.var_t4 * locals.var_t10_dn6)), ((locals.var_t4_dn7 * locals.var_t10) + (locals.var_t4 * locals.var_t10_dn7)), ((locals.var_t4_dn8 * locals.var_t10) + (locals.var_t4 * locals.var_t10_dn8)), ((locals.var_t4_dn9 * locals.var_t10) + (locals.var_t4 * locals.var_t10_dn9)), ((locals.var_t4_dn10 * locals.var_t10) + (locals.var_t4 * locals.var_t10_dn10)), ((locals.var_t4_dn11 * locals.var_t10) + (locals.var_t4 * locals.var_t10_dn11)), ((locals.var_t4_dn14 * locals.var_t10) + (locals.var_t4 * locals.var_t10_dn14)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign67300_e104056;
        locals.var_t3_dn0 = assign67300_e104056_d_n0;
        locals.var_t3_dn2 = assign67300_e104056_d_n2;
        locals.var_t3_dn4 = assign67300_e104056_d_n4;
        locals.var_t3_dn5 = assign67300_e104056_d_n5;
        locals.var_t3_dn6 = assign67300_e104056_d_n6;
        locals.var_t3_dn7 = assign67300_e104056_d_n7;
        locals.var_t3_dn8 = assign67300_e104056_d_n8;
        locals.var_t3_dn9 = assign67300_e104056_d_n9;
        locals.var_t3_dn10 = assign67300_e104056_d_n10;
        locals.var_t3_dn11 = assign67300_e104056_d_n11;
        locals.var_t3_dn14 = assign67300_e104056_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign67310_e104065, assign67310_e104065_d_n0, assign67310_e104065_d_n2, assign67310_e104065_d_n4, assign67310_e104065_d_n5, assign67310_e104065_d_n6, assign67310_e104065_d_n7, assign67310_e104065_d_n8, assign67310_e104065_d_n9, assign67310_e104065_d_n10, assign67310_e104065_d_n11, assign67310_e104065_d_n14,) = {
    if ((locals.var_guard1582 == 0.0) && (locals.var_guard1591 != 0.0)) {
        let assign67310_e104063: f64 = (locals.var_t0 - locals.var_t3);
        (assign67310_e104063, (locals.var_t0_dn0 - locals.var_t3_dn0), (locals.var_t0_dn2 - locals.var_t3_dn2), (locals.var_t0_dn4 - locals.var_t3_dn4), (locals.var_t0_dn5 - locals.var_t3_dn5), (locals.var_t0_dn6 - locals.var_t3_dn6), (locals.var_t0_dn7 - locals.var_t3_dn7), (locals.var_t0_dn8 - locals.var_t3_dn8), (locals.var_t0_dn9 - locals.var_t3_dn9), (locals.var_t0_dn10 - locals.var_t3_dn10), (locals.var_t0_dn11 - locals.var_t3_dn11), (locals.var_t0_dn14 - locals.var_t3_dn14),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign67310_e104065;
        locals.var_t0_dn0 = assign67310_e104065_d_n0;
        locals.var_t0_dn2 = assign67310_e104065_d_n2;
        locals.var_t0_dn4 = assign67310_e104065_d_n4;
        locals.var_t0_dn5 = assign67310_e104065_d_n5;
        locals.var_t0_dn6 = assign67310_e104065_d_n6;
        locals.var_t0_dn7 = assign67310_e104065_d_n7;
        locals.var_t0_dn8 = assign67310_e104065_d_n8;
        locals.var_t0_dn9 = assign67310_e104065_d_n9;
        locals.var_t0_dn10 = assign67310_e104065_d_n10;
        locals.var_t0_dn11 = assign67310_e104065_d_n11;
        locals.var_t0_dn14 = assign67310_e104065_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign67320_e104081, assign67320_e104081_d_n0, assign67320_e104081_d_n2, assign67320_e104081_d_n4, assign67320_e104081_d_n5, assign67320_e104081_d_n6, assign67320_e104081_d_n7, assign67320_e104081_d_n8, assign67320_e104081_d_n9, assign67320_e104081_d_n10, assign67320_e104081_d_n11, assign67320_e104081_d_n14,) = {
    if ((locals.var_guard1582 == 0.0) && (locals.var_guard1591 != 0.0)) {
        let assign67320_e104072: f64 = (locals.var_t0 * locals.var_t0);
        let assign67320_e104075: f64 = (4.0 * 0.01);
        let assign67320_e104077: f64 = (assign67320_e104075 * 0.01);
        let assign67320_e104078: f64 = (assign67320_e104072 + assign67320_e104077);
        let assign67320_e104079: f64 = (assign67320_e104078).sqrt();
        (assign67320_e104079, (((locals.var_t0_dn0 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn0)) / (2.0 * assign67320_e104079)), (((locals.var_t0_dn2 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn2)) / (2.0 * assign67320_e104079)), (((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)) / (2.0 * assign67320_e104079)), (((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)) / (2.0 * assign67320_e104079)), (((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)) / (2.0 * assign67320_e104079)), (((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)) / (2.0 * assign67320_e104079)), (((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)) / (2.0 * assign67320_e104079)), (((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)) / (2.0 * assign67320_e104079)), (((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)) / (2.0 * assign67320_e104079)), (((locals.var_t0_dn11 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn11)) / (2.0 * assign67320_e104079)), (((locals.var_t0_dn14 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn14)) / (2.0 * assign67320_e104079)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign67320_e104081;
        locals.var_tmf2_dn0 = assign67320_e104081_d_n0;
        locals.var_tmf2_dn2 = assign67320_e104081_d_n2;
        locals.var_tmf2_dn4 = assign67320_e104081_d_n4;
        locals.var_tmf2_dn5 = assign67320_e104081_d_n5;
        locals.var_tmf2_dn6 = assign67320_e104081_d_n6;
        locals.var_tmf2_dn7 = assign67320_e104081_d_n7;
        locals.var_tmf2_dn8 = assign67320_e104081_d_n8;
        locals.var_tmf2_dn9 = assign67320_e104081_d_n9;
        locals.var_tmf2_dn10 = assign67320_e104081_d_n10;
        locals.var_tmf2_dn11 = assign67320_e104081_d_n11;
        locals.var_tmf2_dn14 = assign67320_e104081_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign67330_e104094, assign67330_e104094_d_n0, assign67330_e104094_d_n2, assign67330_e104094_d_n4, assign67330_e104094_d_n5, assign67330_e104094_d_n6, assign67330_e104094_d_n7, assign67330_e104094_d_n8, assign67330_e104094_d_n9, assign67330_e104094_d_n10, assign67330_e104094_d_n11, assign67330_e104094_d_n14,) = {
    if ((locals.var_guard1582 == 0.0) && (locals.var_guard1591 != 0.0)) {
        let assign67330_e104090: f64 = (locals.var_t0 / locals.var_tmf2);
        let assign67330_e104091: f64 = (1.0 + assign67330_e104090);
        let assign67330_e104092: f64 = (0.5 * assign67330_e104091);
        (assign67330_e104092, (0.5 * (((locals.var_t0_dn0 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn2 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn4 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn5 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn6 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn7 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn8 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn9 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn10 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn11 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn14 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign67330_e104094;
        locals.var_t9_dn0 = assign67330_e104094_d_n0;
        locals.var_t9_dn2 = assign67330_e104094_d_n2;
        locals.var_t9_dn4 = assign67330_e104094_d_n4;
        locals.var_t9_dn5 = assign67330_e104094_d_n5;
        locals.var_t9_dn6 = assign67330_e104094_d_n6;
        locals.var_t9_dn7 = assign67330_e104094_d_n7;
        locals.var_t9_dn8 = assign67330_e104094_d_n8;
        locals.var_t9_dn9 = assign67330_e104094_d_n9;
        locals.var_t9_dn10 = assign67330_e104094_d_n10;
        locals.var_t9_dn11 = assign67330_e104094_d_n11;
        locals.var_t9_dn14 = assign67330_e104094_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign67340_e104105, assign67340_e104105_d_n0, assign67340_e104105_d_n2, assign67340_e104105_d_n4, assign67340_e104105_d_n5, assign67340_e104105_d_n6, assign67340_e104105_d_n7, assign67340_e104105_d_n8, assign67340_e104105_d_n9, assign67340_e104105_d_n10, assign67340_e104105_d_n11, assign67340_e104105_d_n14,) = {
    if ((locals.var_guard1582 == 0.0) && (locals.var_guard1591 != 0.0)) {
        let assign67340_e104102: f64 = (locals.var_t0 + locals.var_tmf2);
        let assign67340_e104103: f64 = (0.5 * assign67340_e104102);
        (assign67340_e104103, (0.5 * (locals.var_t0_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t0_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t0_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t0_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t0_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t0_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t0_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t0_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t0_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t0_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_t0_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign67340_e104105;
        locals.var_t0_dn0 = assign67340_e104105_d_n0;
        locals.var_t0_dn2 = assign67340_e104105_d_n2;
        locals.var_t0_dn4 = assign67340_e104105_d_n4;
        locals.var_t0_dn5 = assign67340_e104105_d_n5;
        locals.var_t0_dn6 = assign67340_e104105_d_n6;
        locals.var_t0_dn7 = assign67340_e104105_d_n7;
        locals.var_t0_dn8 = assign67340_e104105_d_n8;
        locals.var_t0_dn9 = assign67340_e104105_d_n9;
        locals.var_t0_dn10 = assign67340_e104105_d_n10;
        locals.var_t0_dn11 = assign67340_e104105_d_n11;
        locals.var_t0_dn14 = assign67340_e104105_d_n14;
        locals.var_t0_rv = 0.0;

        let assign67350_e104108: f64 = if locals.var_t0 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1594 = assign67350_e104108;
        locals.var_guard1594_rv = 0.0;

        let (assign67360_e104117, assign67360_e104117_d_n0, assign67360_e104117_d_n2, assign67360_e104117_d_n4, assign67360_e104117_d_n5, assign67360_e104117_d_n6, assign67360_e104117_d_n7, assign67360_e104117_d_n8, assign67360_e104117_d_n9, assign67360_e104117_d_n10, assign67360_e104117_d_n11, assign67360_e104117_d_n14,) = {
    if (((locals.var_guard1582 == 0.0) && (locals.var_guard1591 != 0.0)) && (locals.var_guard1594 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign67360_e104117;
        locals.var_t0_dn0 = assign67360_e104117_d_n0;
        locals.var_t0_dn2 = assign67360_e104117_d_n2;
        locals.var_t0_dn4 = assign67360_e104117_d_n4;
        locals.var_t0_dn5 = assign67360_e104117_d_n5;
        locals.var_t0_dn6 = assign67360_e104117_d_n6;
        locals.var_t0_dn7 = assign67360_e104117_d_n7;
        locals.var_t0_dn8 = assign67360_e104117_d_n8;
        locals.var_t0_dn9 = assign67360_e104117_d_n9;
        locals.var_t0_dn10 = assign67360_e104117_d_n10;
        locals.var_t0_dn11 = assign67360_e104117_d_n11;
        locals.var_t0_dn14 = assign67360_e104117_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign67370_e104126, assign67370_e104126_d_n0, assign67370_e104126_d_n2, assign67370_e104126_d_n4, assign67370_e104126_d_n5, assign67370_e104126_d_n6, assign67370_e104126_d_n7, assign67370_e104126_d_n8, assign67370_e104126_d_n9, assign67370_e104126_d_n10, assign67370_e104126_d_n11, assign67370_e104126_d_n14,) = {
    if (((locals.var_guard1582 == 0.0) && (locals.var_guard1591 != 0.0)) && (locals.var_guard1594 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign67370_e104126;
        locals.var_t9_dn0 = assign67370_e104126_d_n0;
        locals.var_t9_dn2 = assign67370_e104126_d_n2;
        locals.var_t9_dn4 = assign67370_e104126_d_n4;
        locals.var_t9_dn5 = assign67370_e104126_d_n5;
        locals.var_t9_dn6 = assign67370_e104126_d_n6;
        locals.var_t9_dn7 = assign67370_e104126_d_n7;
        locals.var_t9_dn8 = assign67370_e104126_d_n8;
        locals.var_t9_dn9 = assign67370_e104126_d_n9;
        locals.var_t9_dn10 = assign67370_e104126_d_n10;
        locals.var_t9_dn11 = assign67370_e104126_d_n11;
        locals.var_t9_dn14 = assign67370_e104126_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign67380_e104135, assign67380_e104135_d_n0, assign67380_e104135_d_n2, assign67380_e104135_d_n4, assign67380_e104135_d_n5, assign67380_e104135_d_n6, assign67380_e104135_d_n7, assign67380_e104135_d_n8, assign67380_e104135_d_n9, assign67380_e104135_d_n10, assign67380_e104135_d_n11, assign67380_e104135_d_n14,) = {
    if ((locals.var_guard1582 == 0.0) && (locals.var_guard1591 != 0.0)) {
        let assign67380_e104133: f64 = (locals.var_t0 + 1e-25);
        (assign67380_e104133, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign67380_e104135;
        locals.var_t0_dn0 = assign67380_e104135_d_n0;
        locals.var_t0_dn2 = assign67380_e104135_d_n2;
        locals.var_t0_dn4 = assign67380_e104135_d_n4;
        locals.var_t0_dn5 = assign67380_e104135_d_n5;
        locals.var_t0_dn6 = assign67380_e104135_d_n6;
        locals.var_t0_dn7 = assign67380_e104135_d_n7;
        locals.var_t0_dn8 = assign67380_e104135_d_n8;
        locals.var_t0_dn9 = assign67380_e104135_d_n9;
        locals.var_t0_dn10 = assign67380_e104135_d_n10;
        locals.var_t0_dn11 = assign67380_e104135_d_n11;
        locals.var_t0_dn14 = assign67380_e104135_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign67390_e104146, assign67390_e104146_d_n0, assign67390_e104146_d_n2, assign67390_e104146_d_n4, assign67390_e104146_d_n5, assign67390_e104146_d_n6, assign67390_e104146_d_n7, assign67390_e104146_d_n8, assign67390_e104146_d_n9, assign67390_e104146_d_n10, assign67390_e104146_d_n11, assign67390_e104146_d_n14,) = {
    if ((locals.var_guard1582 == 0.0) && (locals.var_guard1591 != 0.0)) {
        let assign67390_e104143: f64 = (locals.var_t0 * locals.var_t1);
        let assign67390_e104144: f64 = (1.0 / assign67390_e104143);
        (assign67390_e104144, (-(((locals.var_t0_dn0 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn0)) / (assign67390_e104143 * assign67390_e104143))), (-(((locals.var_t0_dn2 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn2)) / (assign67390_e104143 * assign67390_e104143))), (-(((locals.var_t0_dn4 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn4)) / (assign67390_e104143 * assign67390_e104143))), (-(((locals.var_t0_dn5 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn5)) / (assign67390_e104143 * assign67390_e104143))), (-(((locals.var_t0_dn6 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn6)) / (assign67390_e104143 * assign67390_e104143))), (-(((locals.var_t0_dn7 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn7)) / (assign67390_e104143 * assign67390_e104143))), (-(((locals.var_t0_dn8 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn8)) / (assign67390_e104143 * assign67390_e104143))), (-(((locals.var_t0_dn9 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn9)) / (assign67390_e104143 * assign67390_e104143))), (-(((locals.var_t0_dn10 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn10)) / (assign67390_e104143 * assign67390_e104143))), (-(((locals.var_t0_dn11 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn11)) / (assign67390_e104143 * assign67390_e104143))), (-(((locals.var_t0_dn14 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn14)) / (assign67390_e104143 * assign67390_e104143))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign67390_e104146;
        locals.var_t4_dn0 = assign67390_e104146_d_n0;
        locals.var_t4_dn2 = assign67390_e104146_d_n2;
        locals.var_t4_dn4 = assign67390_e104146_d_n4;
        locals.var_t4_dn5 = assign67390_e104146_d_n5;
        locals.var_t4_dn6 = assign67390_e104146_d_n6;
        locals.var_t4_dn7 = assign67390_e104146_d_n7;
        locals.var_t4_dn8 = assign67390_e104146_d_n8;
        locals.var_t4_dn9 = assign67390_e104146_d_n9;
        locals.var_t4_dn10 = assign67390_e104146_d_n10;
        locals.var_t4_dn11 = assign67390_e104146_d_n11;
        locals.var_t4_dn14 = assign67390_e104146_d_n14;
        locals.var_t4_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_254(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign67400_e104155, assign67400_e104155_d_n0, assign67400_e104155_d_n2, assign67400_e104155_d_n4, assign67400_e104155_d_n5, assign67400_e104155_d_n6, assign67400_e104155_d_n7, assign67400_e104155_d_n8, assign67400_e104155_d_n9, assign67400_e104155_d_n10, assign67400_e104155_d_n11, assign67400_e104155_d_n14,) = {
    if ((locals.var_guard1582 == 0.0) && (locals.var_guard1591 != 0.0)) {
        let assign67400_e104153: f64 = (locals.var_ldrift0 * locals.var_mks_subld2);
        (assign67400_e104153, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign67400_e104155;
        locals.var_t7_dn0 = assign67400_e104155_d_n0;
        locals.var_t7_dn2 = assign67400_e104155_d_n2;
        locals.var_t7_dn4 = assign67400_e104155_d_n4;
        locals.var_t7_dn5 = assign67400_e104155_d_n5;
        locals.var_t7_dn6 = assign67400_e104155_d_n6;
        locals.var_t7_dn7 = assign67400_e104155_d_n7;
        locals.var_t7_dn8 = assign67400_e104155_d_n8;
        locals.var_t7_dn9 = assign67400_e104155_d_n9;
        locals.var_t7_dn10 = assign67400_e104155_d_n10;
        locals.var_t7_dn11 = assign67400_e104155_d_n11;
        locals.var_t7_dn14 = assign67400_e104155_d_n14;
        locals.var_t7_rv = 0.0;

        let (assign67410_e104166, assign67410_e104166_d_n0, assign67410_e104166_d_n2, assign67410_e104166_d_n4, assign67410_e104166_d_n5, assign67410_e104166_d_n6, assign67410_e104166_d_n7, assign67410_e104166_d_n8, assign67410_e104166_d_n9, assign67410_e104166_d_n10, assign67410_e104166_d_n11, assign67410_e104166_d_n14,) = {
    if ((locals.var_guard1582 == 0.0) && (locals.var_guard1591 != 0.0)) {
        let assign67410_e104161: f64 = (-locals.var_t7);
        let assign67410_e104163: f64 = (assign67410_e104161 * locals.var_t4);
        let assign67410_e104164: f64 = (assign67410_e104163).exp();
        (assign67410_e104164, (assign67410_e104164 * (((-locals.var_t7_dn0) * locals.var_t4) + (assign67410_e104161 * locals.var_t4_dn0))), (assign67410_e104164 * (((-locals.var_t7_dn2) * locals.var_t4) + (assign67410_e104161 * locals.var_t4_dn2))), (assign67410_e104164 * (((-locals.var_t7_dn4) * locals.var_t4) + (assign67410_e104161 * locals.var_t4_dn4))), (assign67410_e104164 * (((-locals.var_t7_dn5) * locals.var_t4) + (assign67410_e104161 * locals.var_t4_dn5))), (assign67410_e104164 * (((-locals.var_t7_dn6) * locals.var_t4) + (assign67410_e104161 * locals.var_t4_dn6))), (assign67410_e104164 * (((-locals.var_t7_dn7) * locals.var_t4) + (assign67410_e104161 * locals.var_t4_dn7))), (assign67410_e104164 * (((-locals.var_t7_dn8) * locals.var_t4) + (assign67410_e104161 * locals.var_t4_dn8))), (assign67410_e104164 * (((-locals.var_t7_dn9) * locals.var_t4) + (assign67410_e104161 * locals.var_t4_dn9))), (assign67410_e104164 * (((-locals.var_t7_dn10) * locals.var_t4) + (assign67410_e104161 * locals.var_t4_dn10))), (assign67410_e104164 * (((-locals.var_t7_dn11) * locals.var_t4) + (assign67410_e104161 * locals.var_t4_dn11))), (assign67410_e104164 * (((-locals.var_t7_dn14) * locals.var_t4) + (assign67410_e104161 * locals.var_t4_dn14))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign67410_e104166;
        locals.var_t2_dn0 = assign67410_e104166_d_n0;
        locals.var_t2_dn2 = assign67410_e104166_d_n2;
        locals.var_t2_dn4 = assign67410_e104166_d_n4;
        locals.var_t2_dn5 = assign67410_e104166_d_n5;
        locals.var_t2_dn6 = assign67410_e104166_d_n6;
        locals.var_t2_dn7 = assign67410_e104166_d_n7;
        locals.var_t2_dn8 = assign67410_e104166_d_n8;
        locals.var_t2_dn9 = assign67410_e104166_d_n9;
        locals.var_t2_dn10 = assign67410_e104166_d_n10;
        locals.var_t2_dn11 = assign67410_e104166_d_n11;
        locals.var_t2_dn14 = assign67410_e104166_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign67420_e104179, assign67420_e104179_d_n0, assign67420_e104179_d_n2, assign67420_e104179_d_n4, assign67420_e104179_d_n5, assign67420_e104179_d_n6, assign67420_e104179_d_n7, assign67420_e104179_d_n8, assign67420_e104179_d_n9, assign67420_e104179_d_n10, assign67420_e104179_d_n11, assign67420_e104179_d_n14,) = {
    if ((locals.var_guard1582 == 0.0) && (locals.var_guard1591 != 0.0)) {
        let assign67420_e104173: f64 = (locals.var_t7 * locals.var_t2);
        let assign67420_e104175: f64 = (assign67420_e104173 * locals.var_t4);
        let assign67420_e104177: f64 = (assign67420_e104175 * locals.var_t4);
        (assign67420_e104177, ((((((locals.var_t7_dn0 * locals.var_t2) + (locals.var_t7 * locals.var_t2_dn0)) * locals.var_t4) + (assign67420_e104173 * locals.var_t4_dn0)) * locals.var_t4) + (assign67420_e104175 * locals.var_t4_dn0)), ((((((locals.var_t7_dn2 * locals.var_t2) + (locals.var_t7 * locals.var_t2_dn2)) * locals.var_t4) + (assign67420_e104173 * locals.var_t4_dn2)) * locals.var_t4) + (assign67420_e104175 * locals.var_t4_dn2)), ((((((locals.var_t7_dn4 * locals.var_t2) + (locals.var_t7 * locals.var_t2_dn4)) * locals.var_t4) + (assign67420_e104173 * locals.var_t4_dn4)) * locals.var_t4) + (assign67420_e104175 * locals.var_t4_dn4)), ((((((locals.var_t7_dn5 * locals.var_t2) + (locals.var_t7 * locals.var_t2_dn5)) * locals.var_t4) + (assign67420_e104173 * locals.var_t4_dn5)) * locals.var_t4) + (assign67420_e104175 * locals.var_t4_dn5)), ((((((locals.var_t7_dn6 * locals.var_t2) + (locals.var_t7 * locals.var_t2_dn6)) * locals.var_t4) + (assign67420_e104173 * locals.var_t4_dn6)) * locals.var_t4) + (assign67420_e104175 * locals.var_t4_dn6)), ((((((locals.var_t7_dn7 * locals.var_t2) + (locals.var_t7 * locals.var_t2_dn7)) * locals.var_t4) + (assign67420_e104173 * locals.var_t4_dn7)) * locals.var_t4) + (assign67420_e104175 * locals.var_t4_dn7)), ((((((locals.var_t7_dn8 * locals.var_t2) + (locals.var_t7 * locals.var_t2_dn8)) * locals.var_t4) + (assign67420_e104173 * locals.var_t4_dn8)) * locals.var_t4) + (assign67420_e104175 * locals.var_t4_dn8)), ((((((locals.var_t7_dn9 * locals.var_t2) + (locals.var_t7 * locals.var_t2_dn9)) * locals.var_t4) + (assign67420_e104173 * locals.var_t4_dn9)) * locals.var_t4) + (assign67420_e104175 * locals.var_t4_dn9)), ((((((locals.var_t7_dn10 * locals.var_t2) + (locals.var_t7 * locals.var_t2_dn10)) * locals.var_t4) + (assign67420_e104173 * locals.var_t4_dn10)) * locals.var_t4) + (assign67420_e104175 * locals.var_t4_dn10)), ((((((locals.var_t7_dn11 * locals.var_t2) + (locals.var_t7 * locals.var_t2_dn11)) * locals.var_t4) + (assign67420_e104173 * locals.var_t4_dn11)) * locals.var_t4) + (assign67420_e104175 * locals.var_t4_dn11)), ((((((locals.var_t7_dn14 * locals.var_t2) + (locals.var_t7 * locals.var_t2_dn14)) * locals.var_t4) + (assign67420_e104173 * locals.var_t4_dn14)) * locals.var_t4) + (assign67420_e104175 * locals.var_t4_dn14)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign67420_e104179;
        locals.var_t6_dn0 = assign67420_e104179_d_n0;
        locals.var_t6_dn2 = assign67420_e104179_d_n2;
        locals.var_t6_dn4 = assign67420_e104179_d_n4;
        locals.var_t6_dn5 = assign67420_e104179_d_n5;
        locals.var_t6_dn6 = assign67420_e104179_d_n6;
        locals.var_t6_dn7 = assign67420_e104179_d_n7;
        locals.var_t6_dn8 = assign67420_e104179_d_n8;
        locals.var_t6_dn9 = assign67420_e104179_d_n9;
        locals.var_t6_dn10 = assign67420_e104179_d_n10;
        locals.var_t6_dn11 = assign67420_e104179_d_n11;
        locals.var_t6_dn14 = assign67420_e104179_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign67430_e104192, assign67430_e104192_d_n0, assign67430_e104192_d_n2, assign67430_e104192_d_n4, assign67430_e104192_d_n5, assign67430_e104192_d_n6, assign67430_e104192_d_n7, assign67430_e104192_d_n8, assign67430_e104192_d_n9, assign67430_e104192_d_n10, assign67430_e104192_d_n11, assign67430_e104192_d_n14,) = {
    if ((locals.var_guard1582 == 0.0) && (locals.var_guard1591 != 0.0)) {
        let assign67430_e104186: f64 = (locals.var_uc_subld1 * locals.var_ids);
        let assign67430_e104188: f64 = (assign67430_e104186 * locals.var_t0);
        let assign67430_e104190: f64 = (assign67430_e104188 * locals.var_t2);
        (assign67430_e104190, (((((locals.var_uc_subld1 * locals.var_ids_dn0) * locals.var_t0) + (assign67430_e104186 * locals.var_t0_dn0)) * locals.var_t2) + (assign67430_e104188 * locals.var_t2_dn0)), (((((locals.var_uc_subld1 * locals.var_ids_dn2) * locals.var_t0) + (assign67430_e104186 * locals.var_t0_dn2)) * locals.var_t2) + (assign67430_e104188 * locals.var_t2_dn2)), (((((locals.var_uc_subld1 * locals.var_ids_dn4) * locals.var_t0) + (assign67430_e104186 * locals.var_t0_dn4)) * locals.var_t2) + (assign67430_e104188 * locals.var_t2_dn4)), (((((locals.var_uc_subld1 * locals.var_ids_dn5) * locals.var_t0) + (assign67430_e104186 * locals.var_t0_dn5)) * locals.var_t2) + (assign67430_e104188 * locals.var_t2_dn5)), (((((locals.var_uc_subld1 * locals.var_ids_dn6) * locals.var_t0) + (assign67430_e104186 * locals.var_t0_dn6)) * locals.var_t2) + (assign67430_e104188 * locals.var_t2_dn6)), (((((locals.var_uc_subld1 * locals.var_ids_dn7) * locals.var_t0) + (assign67430_e104186 * locals.var_t0_dn7)) * locals.var_t2) + (assign67430_e104188 * locals.var_t2_dn7)), (((((locals.var_uc_subld1 * locals.var_ids_dn8) * locals.var_t0) + (assign67430_e104186 * locals.var_t0_dn8)) * locals.var_t2) + (assign67430_e104188 * locals.var_t2_dn8)), (((((locals.var_uc_subld1 * locals.var_ids_dn9) * locals.var_t0) + (assign67430_e104186 * locals.var_t0_dn9)) * locals.var_t2) + (assign67430_e104188 * locals.var_t2_dn9)), (((((locals.var_uc_subld1 * locals.var_ids_dn10) * locals.var_t0) + (assign67430_e104186 * locals.var_t0_dn10)) * locals.var_t2) + (assign67430_e104188 * locals.var_t2_dn10)), (((((locals.var_uc_subld1 * locals.var_ids_dn11) * locals.var_t0) + (assign67430_e104186 * locals.var_t0_dn11)) * locals.var_t2) + (assign67430_e104188 * locals.var_t2_dn11)), (((((locals.var_uc_subld1 * locals.var_ids_dn14) * locals.var_t0) + (assign67430_e104186 * locals.var_t0_dn14)) * locals.var_t2) + (assign67430_e104188 * locals.var_t2_dn14)),)
    } else {
        (locals.var_isubld, locals.var_isubld_dn0, locals.var_isubld_dn2, locals.var_isubld_dn4, locals.var_isubld_dn5, locals.var_isubld_dn6, locals.var_isubld_dn7, locals.var_isubld_dn8, locals.var_isubld_dn9, locals.var_isubld_dn10, locals.var_isubld_dn11, locals.var_isubld_dn14,)
    }
};
        locals.var_isubld = assign67430_e104192;
        locals.var_isubld_dn0 = assign67430_e104192_d_n0;
        locals.var_isubld_dn2 = assign67430_e104192_d_n2;
        locals.var_isubld_dn4 = assign67430_e104192_d_n4;
        locals.var_isubld_dn5 = assign67430_e104192_d_n5;
        locals.var_isubld_dn6 = assign67430_e104192_d_n6;
        locals.var_isubld_dn7 = assign67430_e104192_d_n7;
        locals.var_isubld_dn8 = assign67430_e104192_d_n8;
        locals.var_isubld_dn9 = assign67430_e104192_d_n9;
        locals.var_isubld_dn10 = assign67430_e104192_d_n10;
        locals.var_isubld_dn11 = assign67430_e104192_d_n11;
        locals.var_isubld_dn14 = assign67430_e104192_d_n14;
        locals.var_isubld_rv = 0.0;

        let assign67440_e104195: f64 = if p.p45 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1595 = assign67440_e104195;
        locals.var_guard1595_rv = 0.0;

        let (assign67450_e104199, assign67450_e104199_d_n0, assign67450_e104199_d_n2, assign67450_e104199_d_n4, assign67450_e104199_d_n5, assign67450_e104199_d_n6, assign67450_e104199_d_n7, assign67450_e104199_d_n8, assign67450_e104199_d_n9, assign67450_e104199_d_n10, assign67450_e104199_d_n11, assign67450_e104199_d_n14,) = {
    if (locals.var_guard1595 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ibreakhe, locals.var_ibreakhe_dn0, locals.var_ibreakhe_dn2, locals.var_ibreakhe_dn4, locals.var_ibreakhe_dn5, locals.var_ibreakhe_dn6, locals.var_ibreakhe_dn7, locals.var_ibreakhe_dn8, locals.var_ibreakhe_dn9, locals.var_ibreakhe_dn10, locals.var_ibreakhe_dn11, locals.var_ibreakhe_dn14,)
    }
};
        locals.var_ibreakhe = assign67450_e104199;
        locals.var_ibreakhe_dn0 = assign67450_e104199_d_n0;
        locals.var_ibreakhe_dn2 = assign67450_e104199_d_n2;
        locals.var_ibreakhe_dn4 = assign67450_e104199_d_n4;
        locals.var_ibreakhe_dn5 = assign67450_e104199_d_n5;
        locals.var_ibreakhe_dn6 = assign67450_e104199_d_n6;
        locals.var_ibreakhe_dn7 = assign67450_e104199_d_n7;
        locals.var_ibreakhe_dn8 = assign67450_e104199_d_n8;
        locals.var_ibreakhe_dn9 = assign67450_e104199_d_n9;
        locals.var_ibreakhe_dn10 = assign67450_e104199_d_n10;
        locals.var_ibreakhe_dn11 = assign67450_e104199_d_n11;
        locals.var_ibreakhe_dn14 = assign67450_e104199_d_n14;
        locals.var_ibreakhe_rv = 0.0;

        let assign67460_e104203: f64 = (locals.var_vgse - p.p446);
        let assign67460_e104204: f64 = (p.p45 * assign67460_e104203);
        let assign67460_e104206: f64 = if assign67460_e104204 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1596 = assign67460_e104206;
        locals.var_guard1596_rv = 0.0;

        let (assign67470_e104213, assign67470_e104213_d_n0, assign67470_e104213_d_n2, assign67470_e104213_d_n4, assign67470_e104213_d_n5, assign67470_e104213_d_n6, assign67470_e104213_d_n7, assign67470_e104213_d_n8, assign67470_e104213_d_n9, assign67470_e104213_d_n10, assign67470_e104213_d_n11, assign67470_e104213_d_n14,) = {
    if ((locals.var_guard1595 == 0.0) && (locals.var_guard1596 != 0.0)) {
        (locals.var_hbdceff, locals.var_hbdceff_dn0, locals.var_hbdceff_dn2, locals.var_hbdceff_dn4, locals.var_hbdceff_dn5, locals.var_hbdceff_dn6, locals.var_hbdceff_dn7, locals.var_hbdceff_dn8, locals.var_hbdceff_dn9, locals.var_hbdceff_dn10, locals.var_hbdceff_dn11, locals.var_hbdceff_dn14,)
    } else {
        (locals.var_hbdv, locals.var_hbdv_dn0, locals.var_hbdv_dn2, locals.var_hbdv_dn4, locals.var_hbdv_dn5, locals.var_hbdv_dn6, locals.var_hbdv_dn7, locals.var_hbdv_dn8, locals.var_hbdv_dn9, locals.var_hbdv_dn10, locals.var_hbdv_dn11, locals.var_hbdv_dn14,)
    }
};
        locals.var_hbdv = assign67470_e104213;
        locals.var_hbdv_dn0 = assign67470_e104213_d_n0;
        locals.var_hbdv_dn2 = assign67470_e104213_d_n2;
        locals.var_hbdv_dn4 = assign67470_e104213_d_n4;
        locals.var_hbdv_dn5 = assign67470_e104213_d_n5;
        locals.var_hbdv_dn6 = assign67470_e104213_d_n6;
        locals.var_hbdv_dn7 = assign67470_e104213_d_n7;
        locals.var_hbdv_dn8 = assign67470_e104213_d_n8;
        locals.var_hbdv_dn9 = assign67470_e104213_d_n9;
        locals.var_hbdv_dn10 = assign67470_e104213_d_n10;
        locals.var_hbdv_dn11 = assign67470_e104213_d_n11;
        locals.var_hbdv_dn14 = assign67470_e104213_d_n14;
        locals.var_hbdv_rv = 0.0;

        let (assign67480_e104229, assign67480_e104229_d_n0, assign67480_e104229_d_n2, assign67480_e104229_d_n4, assign67480_e104229_d_n5, assign67480_e104229_d_n6, assign67480_e104229_d_n7, assign67480_e104229_d_n8, assign67480_e104229_d_n9, assign67480_e104229_d_n10, assign67480_e104229_d_n11, assign67480_e104229_d_n14,) = {
    if ((locals.var_guard1595 == 0.0) && (locals.var_guard1596 == 0.0)) {
        let assign67480_e104222: f64 = (locals.var_vgse - p.p446);
        let assign67480_e104224: f64 = (assign67480_e104222).powf(2.0);
        let assign67480_e104225: f64 = (p.p445 * assign67480_e104224);
        let assign67480_e104227: f64 = (assign67480_e104225 + locals.var_hbdceff);
        (assign67480_e104227, ((p.p445 * if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign67480_e104222).powf(2.0 - 1.0) * locals.var_vgse_dn0)) } } else { (assign67480_e104224 * (2.0 * (locals.var_vgse_dn0 / assign67480_e104222))) }) + locals.var_hbdceff_dn0), ((p.p445 * if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign67480_e104222).powf(2.0 - 1.0) * locals.var_vgse_dn2)) } } else { (assign67480_e104224 * (2.0 * (locals.var_vgse_dn2 / assign67480_e104222))) }) + locals.var_hbdceff_dn2), locals.var_hbdceff_dn4, locals.var_hbdceff_dn5, locals.var_hbdceff_dn6, ((p.p445 * if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign67480_e104222).powf(2.0 - 1.0) * locals.var_vgse_dn7)) } } else { (assign67480_e104224 * (2.0 * (locals.var_vgse_dn7 / assign67480_e104222))) }) + locals.var_hbdceff_dn7), locals.var_hbdceff_dn8, locals.var_hbdceff_dn9, locals.var_hbdceff_dn10, locals.var_hbdceff_dn11, locals.var_hbdceff_dn14,)
    } else {
        (locals.var_hbdv, locals.var_hbdv_dn0, locals.var_hbdv_dn2, locals.var_hbdv_dn4, locals.var_hbdv_dn5, locals.var_hbdv_dn6, locals.var_hbdv_dn7, locals.var_hbdv_dn8, locals.var_hbdv_dn9, locals.var_hbdv_dn10, locals.var_hbdv_dn11, locals.var_hbdv_dn14,)
    }
};
        locals.var_hbdv = assign67480_e104229;
        locals.var_hbdv_dn0 = assign67480_e104229_d_n0;
        locals.var_hbdv_dn2 = assign67480_e104229_d_n2;
        locals.var_hbdv_dn4 = assign67480_e104229_d_n4;
        locals.var_hbdv_dn5 = assign67480_e104229_d_n5;
        locals.var_hbdv_dn6 = assign67480_e104229_d_n6;
        locals.var_hbdv_dn7 = assign67480_e104229_d_n7;
        locals.var_hbdv_dn8 = assign67480_e104229_d_n8;
        locals.var_hbdv_dn9 = assign67480_e104229_d_n9;
        locals.var_hbdv_dn10 = assign67480_e104229_d_n10;
        locals.var_hbdv_dn11 = assign67480_e104229_d_n11;
        locals.var_hbdv_dn14 = assign67480_e104229_d_n14;
        locals.var_hbdv_rv = 0.0;

        let (assign67490_e104241, assign67490_e104241_d_n0, assign67490_e104241_d_n2, assign67490_e104241_d_n4, assign67490_e104241_d_n5, assign67490_e104241_d_n6, assign67490_e104241_d_n7, assign67490_e104241_d_n8, assign67490_e104241_d_n9, assign67490_e104241_d_n10, assign67490_e104241_d_n11, assign67490_e104241_d_n14,) = {
    if (locals.var_guard1595 == 0.0) {
        let assign67490_e104236: f64 = (locals.var_vdse - locals.var_hbdv);
        let assign67490_e104237: f64 = (locals.var_beta * assign67490_e104236);
        let assign67490_e104238: f64 = { let limited_exp_arg = assign67490_e104237; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign67490_e104239: f64 = (p.p449 * assign67490_e104238);
        (assign67490_e104239, (p.p449 * ({ let limited_exp_arg = assign67490_e104237; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_beta_dn0 * assign67490_e104236) + (locals.var_beta * (locals.var_vdse_dn0 - locals.var_hbdv_dn0))))), (p.p449 * ({ let limited_exp_arg = assign67490_e104237; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_beta_dn2 * assign67490_e104236) + (locals.var_beta * (locals.var_vdse_dn2 - locals.var_hbdv_dn2))))), (p.p449 * ({ let limited_exp_arg = assign67490_e104237; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_beta_dn4 * assign67490_e104236) + (locals.var_beta * (-locals.var_hbdv_dn4))))), (p.p449 * ({ let limited_exp_arg = assign67490_e104237; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_beta_dn5 * assign67490_e104236) + (locals.var_beta * (-locals.var_hbdv_dn5))))), (p.p449 * ({ let limited_exp_arg = assign67490_e104237; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_beta_dn6 * assign67490_e104236) + (locals.var_beta * (-locals.var_hbdv_dn6))))), (p.p449 * ({ let limited_exp_arg = assign67490_e104237; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_beta_dn7 * assign67490_e104236) + (locals.var_beta * (-locals.var_hbdv_dn7))))), (p.p449 * ({ let limited_exp_arg = assign67490_e104237; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_beta_dn8 * assign67490_e104236) + (locals.var_beta * (-locals.var_hbdv_dn8))))), (p.p449 * ({ let limited_exp_arg = assign67490_e104237; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_beta_dn9 * assign67490_e104236) + (locals.var_beta * (-locals.var_hbdv_dn9))))), (p.p449 * ({ let limited_exp_arg = assign67490_e104237; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_beta_dn10 * assign67490_e104236) + (locals.var_beta * (-locals.var_hbdv_dn10))))), (p.p449 * ({ let limited_exp_arg = assign67490_e104237; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_beta_dn11 * assign67490_e104236) + (locals.var_beta * (-locals.var_hbdv_dn11))))), (p.p449 * ({ let limited_exp_arg = assign67490_e104237; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_beta_dn14 * assign67490_e104236) + (locals.var_beta * (-locals.var_hbdv_dn14))))),)
    } else {
        (locals.var_ibreakhe, locals.var_ibreakhe_dn0, locals.var_ibreakhe_dn2, locals.var_ibreakhe_dn4, locals.var_ibreakhe_dn5, locals.var_ibreakhe_dn6, locals.var_ibreakhe_dn7, locals.var_ibreakhe_dn8, locals.var_ibreakhe_dn9, locals.var_ibreakhe_dn10, locals.var_ibreakhe_dn11, locals.var_ibreakhe_dn14,)
    }
};
        locals.var_ibreakhe = assign67490_e104241;
        locals.var_ibreakhe_dn0 = assign67490_e104241_d_n0;
        locals.var_ibreakhe_dn2 = assign67490_e104241_d_n2;
        locals.var_ibreakhe_dn4 = assign67490_e104241_d_n4;
        locals.var_ibreakhe_dn5 = assign67490_e104241_d_n5;
        locals.var_ibreakhe_dn6 = assign67490_e104241_d_n6;
        locals.var_ibreakhe_dn7 = assign67490_e104241_d_n7;
        locals.var_ibreakhe_dn8 = assign67490_e104241_d_n8;
        locals.var_ibreakhe_dn9 = assign67490_e104241_d_n9;
        locals.var_ibreakhe_dn10 = assign67490_e104241_d_n10;
        locals.var_ibreakhe_dn11 = assign67490_e104241_d_n11;
        locals.var_ibreakhe_dn14 = assign67490_e104241_d_n14;
        locals.var_ibreakhe_rv = 0.0;

        let assign67500_e104244: f64 = if locals.var_ibreakhe > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1597 = assign67500_e104244;
        locals.var_guard1597_rv = 0.0;

        let assign67510_e104248: f64 = (100000.0 - 50000.0);
        let assign67510_e104253: f64 = if ((locals.var_ibreakhe > assign67510_e104248) && (50000.0 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1598 = assign67510_e104253;
        locals.var_guard1598_rv = 0.0;

        let (assign67520_e104263, assign67520_e104263_d_n0, assign67520_e104263_d_n2, assign67520_e104263_d_n4, assign67520_e104263_d_n5, assign67520_e104263_d_n6, assign67520_e104263_d_n7, assign67520_e104263_d_n8, assign67520_e104263_d_n9, assign67520_e104263_d_n10, assign67520_e104263_d_n11, assign67520_e104263_d_n14,) = {
    if ((locals.var_guard1597 != 0.0) && (locals.var_guard1598 != 0.0)) {
        let assign67520_e104259: f64 = (locals.var_ibreakhe - 100000.0);
        let assign67520_e104261: f64 = (assign67520_e104259 + 50000.0);
        (assign67520_e104261, locals.var_ibreakhe_dn0, locals.var_ibreakhe_dn2, locals.var_ibreakhe_dn4, locals.var_ibreakhe_dn5, locals.var_ibreakhe_dn6, locals.var_ibreakhe_dn7, locals.var_ibreakhe_dn8, locals.var_ibreakhe_dn9, locals.var_ibreakhe_dn10, locals.var_ibreakhe_dn11, locals.var_ibreakhe_dn14,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign67520_e104263;
        locals.var_tmf1_dn0 = assign67520_e104263_d_n0;
        locals.var_tmf1_dn2 = assign67520_e104263_d_n2;
        locals.var_tmf1_dn4 = assign67520_e104263_d_n4;
        locals.var_tmf1_dn5 = assign67520_e104263_d_n5;
        locals.var_tmf1_dn6 = assign67520_e104263_d_n6;
        locals.var_tmf1_dn7 = assign67520_e104263_d_n7;
        locals.var_tmf1_dn8 = assign67520_e104263_d_n8;
        locals.var_tmf1_dn9 = assign67520_e104263_d_n9;
        locals.var_tmf1_dn10 = assign67520_e104263_d_n10;
        locals.var_tmf1_dn11 = assign67520_e104263_d_n11;
        locals.var_tmf1_dn14 = assign67520_e104263_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign67530_e104271, assign67530_e104271_d_n0, assign67530_e104271_d_n2, assign67530_e104271_d_n4, assign67530_e104271_d_n5, assign67530_e104271_d_n6, assign67530_e104271_d_n7, assign67530_e104271_d_n8, assign67530_e104271_d_n9, assign67530_e104271_d_n10, assign67530_e104271_d_n11, assign67530_e104271_d_n14,) = {
    if ((locals.var_guard1597 != 0.0) && (locals.var_guard1598 != 0.0)) {
        let assign67530_e104269: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign67530_e104269, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign67530_e104271;
        locals.var_x2_dn0 = assign67530_e104271_d_n0;
        locals.var_x2_dn2 = assign67530_e104271_d_n2;
        locals.var_x2_dn4 = assign67530_e104271_d_n4;
        locals.var_x2_dn5 = assign67530_e104271_d_n5;
        locals.var_x2_dn6 = assign67530_e104271_d_n6;
        locals.var_x2_dn7 = assign67530_e104271_d_n7;
        locals.var_x2_dn8 = assign67530_e104271_d_n8;
        locals.var_x2_dn9 = assign67530_e104271_d_n9;
        locals.var_x2_dn10 = assign67530_e104271_d_n10;
        locals.var_x2_dn11 = assign67530_e104271_d_n11;
        locals.var_x2_dn14 = assign67530_e104271_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign67540_e104279, assign67540_e104279_d_n0, assign67540_e104279_d_n2, assign67540_e104279_d_n4, assign67540_e104279_d_n5, assign67540_e104279_d_n6, assign67540_e104279_d_n7, assign67540_e104279_d_n8, assign67540_e104279_d_n9, assign67540_e104279_d_n10, assign67540_e104279_d_n11, assign67540_e104279_d_n14,) = {
    if ((locals.var_guard1597 != 0.0) && (locals.var_guard1598 != 0.0)) {
        let assign67540_e104277: f64 = (50000.0 * 50000.0);
        (assign67540_e104277, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign67540_e104279;
        locals.var_xmax2_dn0 = assign67540_e104279_d_n0;
        locals.var_xmax2_dn2 = assign67540_e104279_d_n2;
        locals.var_xmax2_dn4 = assign67540_e104279_d_n4;
        locals.var_xmax2_dn5 = assign67540_e104279_d_n5;
        locals.var_xmax2_dn6 = assign67540_e104279_d_n6;
        locals.var_xmax2_dn7 = assign67540_e104279_d_n7;
        locals.var_xmax2_dn8 = assign67540_e104279_d_n8;
        locals.var_xmax2_dn9 = assign67540_e104279_d_n9;
        locals.var_xmax2_dn10 = assign67540_e104279_d_n10;
        locals.var_xmax2_dn11 = assign67540_e104279_d_n11;
        locals.var_xmax2_dn14 = assign67540_e104279_d_n14;
        locals.var_xmax2_rv = 0.0;

        let (assign67550_e104285, assign67550_e104285_d_n0, assign67550_e104285_d_n2, assign67550_e104285_d_n4, assign67550_e104285_d_n5, assign67550_e104285_d_n6, assign67550_e104285_d_n7, assign67550_e104285_d_n8, assign67550_e104285_d_n9, assign67550_e104285_d_n10, assign67550_e104285_d_n11, assign67550_e104285_d_n14,) = {
    if ((locals.var_guard1597 != 0.0) && (locals.var_guard1598 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign67550_e104285;
        locals.var_xp_dn0 = assign67550_e104285_d_n0;
        locals.var_xp_dn2 = assign67550_e104285_d_n2;
        locals.var_xp_dn4 = assign67550_e104285_d_n4;
        locals.var_xp_dn5 = assign67550_e104285_d_n5;
        locals.var_xp_dn6 = assign67550_e104285_d_n6;
        locals.var_xp_dn7 = assign67550_e104285_d_n7;
        locals.var_xp_dn8 = assign67550_e104285_d_n8;
        locals.var_xp_dn9 = assign67550_e104285_d_n9;
        locals.var_xp_dn10 = assign67550_e104285_d_n10;
        locals.var_xp_dn11 = assign67550_e104285_d_n11;
        locals.var_xp_dn14 = assign67550_e104285_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign67560_e104291, assign67560_e104291_d_n0, assign67560_e104291_d_n2, assign67560_e104291_d_n4, assign67560_e104291_d_n5, assign67560_e104291_d_n6, assign67560_e104291_d_n7, assign67560_e104291_d_n8, assign67560_e104291_d_n9, assign67560_e104291_d_n10, assign67560_e104291_d_n11, assign67560_e104291_d_n14,) = {
    if ((locals.var_guard1597 != 0.0) && (locals.var_guard1598 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign67560_e104291;
        locals.var_xmp_dn0 = assign67560_e104291_d_n0;
        locals.var_xmp_dn2 = assign67560_e104291_d_n2;
        locals.var_xmp_dn4 = assign67560_e104291_d_n4;
        locals.var_xmp_dn5 = assign67560_e104291_d_n5;
        locals.var_xmp_dn6 = assign67560_e104291_d_n6;
        locals.var_xmp_dn7 = assign67560_e104291_d_n7;
        locals.var_xmp_dn8 = assign67560_e104291_d_n8;
        locals.var_xmp_dn9 = assign67560_e104291_d_n9;
        locals.var_xmp_dn10 = assign67560_e104291_d_n10;
        locals.var_xmp_dn11 = assign67560_e104291_d_n11;
        locals.var_xmp_dn14 = assign67560_e104291_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign67570_e104297,) = {
    if ((locals.var_guard1597 != 0.0) && (locals.var_guard1598 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign67570_e104297;
        locals.var_m0_rv = 0.0;

        let (assign67580_e104303,) = {
    if ((locals.var_guard1597 != 0.0) && (locals.var_guard1598 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign67580_e104303;
        locals.var_mm_rv = 0.0;

        let (assign67590_e104309, assign67590_e104309_d_n0, assign67590_e104309_d_n2, assign67590_e104309_d_n4, assign67590_e104309_d_n5, assign67590_e104309_d_n6, assign67590_e104309_d_n7, assign67590_e104309_d_n8, assign67590_e104309_d_n9, assign67590_e104309_d_n10, assign67590_e104309_d_n11, assign67590_e104309_d_n14,) = {
    if ((locals.var_guard1597 != 0.0) && (locals.var_guard1598 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign67590_e104309;
        locals.var_arg_dn0 = assign67590_e104309_d_n0;
        locals.var_arg_dn2 = assign67590_e104309_d_n2;
        locals.var_arg_dn4 = assign67590_e104309_d_n4;
        locals.var_arg_dn5 = assign67590_e104309_d_n5;
        locals.var_arg_dn6 = assign67590_e104309_d_n6;
        locals.var_arg_dn7 = assign67590_e104309_d_n7;
        locals.var_arg_dn8 = assign67590_e104309_d_n8;
        locals.var_arg_dn9 = assign67590_e104309_d_n9;
        locals.var_arg_dn10 = assign67590_e104309_d_n10;
        locals.var_arg_dn11 = assign67590_e104309_d_n11;
        locals.var_arg_dn14 = assign67590_e104309_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign67600_e104315, assign67600_e104315_d_n0, assign67600_e104315_d_n2, assign67600_e104315_d_n4, assign67600_e104315_d_n5, assign67600_e104315_d_n6, assign67600_e104315_d_n7, assign67600_e104315_d_n8, assign67600_e104315_d_n9, assign67600_e104315_d_n10, assign67600_e104315_d_n11, assign67600_e104315_d_n14,) = {
    if ((locals.var_guard1597 != 0.0) && (locals.var_guard1598 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign67600_e104315;
        locals.var_dnm_dn0 = assign67600_e104315_d_n0;
        locals.var_dnm_dn2 = assign67600_e104315_d_n2;
        locals.var_dnm_dn4 = assign67600_e104315_d_n4;
        locals.var_dnm_dn5 = assign67600_e104315_d_n5;
        locals.var_dnm_dn6 = assign67600_e104315_d_n6;
        locals.var_dnm_dn7 = assign67600_e104315_d_n7;
        locals.var_dnm_dn8 = assign67600_e104315_d_n8;
        locals.var_dnm_dn9 = assign67600_e104315_d_n9;
        locals.var_dnm_dn10 = assign67600_e104315_d_n10;
        locals.var_dnm_dn11 = assign67600_e104315_d_n11;
        locals.var_dnm_dn14 = assign67600_e104315_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign67610_e104323, assign67610_e104323_d_n0, assign67610_e104323_d_n2, assign67610_e104323_d_n4, assign67610_e104323_d_n5, assign67610_e104323_d_n6, assign67610_e104323_d_n7, assign67610_e104323_d_n8, assign67610_e104323_d_n9, assign67610_e104323_d_n10, assign67610_e104323_d_n11, assign67610_e104323_d_n14,) = {
    if ((locals.var_guard1597 != 0.0) && (locals.var_guard1598 != 0.0)) {
        let assign67610_e104321: f64 = (locals.var_xp * locals.var_x2);
        (assign67610_e104321, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign67610_e104323;
        locals.var_xp_dn0 = assign67610_e104323_d_n0;
        locals.var_xp_dn2 = assign67610_e104323_d_n2;
        locals.var_xp_dn4 = assign67610_e104323_d_n4;
        locals.var_xp_dn5 = assign67610_e104323_d_n5;
        locals.var_xp_dn6 = assign67610_e104323_d_n6;
        locals.var_xp_dn7 = assign67610_e104323_d_n7;
        locals.var_xp_dn8 = assign67610_e104323_d_n8;
        locals.var_xp_dn9 = assign67610_e104323_d_n9;
        locals.var_xp_dn10 = assign67610_e104323_d_n10;
        locals.var_xp_dn11 = assign67610_e104323_d_n11;
        locals.var_xp_dn14 = assign67610_e104323_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign67620_e104331, assign67620_e104331_d_n0, assign67620_e104331_d_n2, assign67620_e104331_d_n4, assign67620_e104331_d_n5, assign67620_e104331_d_n6, assign67620_e104331_d_n7, assign67620_e104331_d_n8, assign67620_e104331_d_n9, assign67620_e104331_d_n10, assign67620_e104331_d_n11, assign67620_e104331_d_n14,) = {
    if ((locals.var_guard1597 != 0.0) && (locals.var_guard1598 != 0.0)) {
        let assign67620_e104329: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign67620_e104329, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign67620_e104331;
        locals.var_xmp_dn0 = assign67620_e104331_d_n0;
        locals.var_xmp_dn2 = assign67620_e104331_d_n2;
        locals.var_xmp_dn4 = assign67620_e104331_d_n4;
        locals.var_xmp_dn5 = assign67620_e104331_d_n5;
        locals.var_xmp_dn6 = assign67620_e104331_d_n6;
        locals.var_xmp_dn7 = assign67620_e104331_d_n7;
        locals.var_xmp_dn8 = assign67620_e104331_d_n8;
        locals.var_xmp_dn9 = assign67620_e104331_d_n9;
        locals.var_xmp_dn10 = assign67620_e104331_d_n10;
        locals.var_xmp_dn11 = assign67620_e104331_d_n11;
        locals.var_xmp_dn14 = assign67620_e104331_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign67630_e104339, assign67630_e104339_d_n0, assign67630_e104339_d_n2, assign67630_e104339_d_n4, assign67630_e104339_d_n5, assign67630_e104339_d_n6, assign67630_e104339_d_n7, assign67630_e104339_d_n8, assign67630_e104339_d_n9, assign67630_e104339_d_n10, assign67630_e104339_d_n11, assign67630_e104339_d_n14,) = {
    if ((locals.var_guard1597 != 0.0) && (locals.var_guard1598 != 0.0)) {
        let assign67630_e104337: f64 = (locals.var_xp + locals.var_xmp);
        (assign67630_e104337, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign67630_e104339;
        locals.var_arg_dn0 = assign67630_e104339_d_n0;
        locals.var_arg_dn2 = assign67630_e104339_d_n2;
        locals.var_arg_dn4 = assign67630_e104339_d_n4;
        locals.var_arg_dn5 = assign67630_e104339_d_n5;
        locals.var_arg_dn6 = assign67630_e104339_d_n6;
        locals.var_arg_dn7 = assign67630_e104339_d_n7;
        locals.var_arg_dn8 = assign67630_e104339_d_n8;
        locals.var_arg_dn9 = assign67630_e104339_d_n9;
        locals.var_arg_dn10 = assign67630_e104339_d_n10;
        locals.var_arg_dn11 = assign67630_e104339_d_n11;
        locals.var_arg_dn14 = assign67630_e104339_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign67640_e104345, assign67640_e104345_d_n0, assign67640_e104345_d_n2, assign67640_e104345_d_n4, assign67640_e104345_d_n5, assign67640_e104345_d_n6, assign67640_e104345_d_n7, assign67640_e104345_d_n8, assign67640_e104345_d_n9, assign67640_e104345_d_n10, assign67640_e104345_d_n11, assign67640_e104345_d_n14,) = {
    if ((locals.var_guard1597 != 0.0) && (locals.var_guard1598 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign67640_e104345;
        locals.var_dnm_dn0 = assign67640_e104345_d_n0;
        locals.var_dnm_dn2 = assign67640_e104345_d_n2;
        locals.var_dnm_dn4 = assign67640_e104345_d_n4;
        locals.var_dnm_dn5 = assign67640_e104345_d_n5;
        locals.var_dnm_dn6 = assign67640_e104345_d_n6;
        locals.var_dnm_dn7 = assign67640_e104345_d_n7;
        locals.var_dnm_dn8 = assign67640_e104345_d_n8;
        locals.var_dnm_dn9 = assign67640_e104345_d_n9;
        locals.var_dnm_dn10 = assign67640_e104345_d_n10;
        locals.var_dnm_dn11 = assign67640_e104345_d_n11;
        locals.var_dnm_dn14 = assign67640_e104345_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign67650_e104360: f64 = if ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1599 = assign67650_e104360;
        locals.var_guard1599_rv = 0.0;

        let assign67660_e104363: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1600 = assign67660_e104363;
        locals.var_guard1600_rv = 0.0;

        let (assign67670_e104373,) = {
    if ((((locals.var_guard1597 != 0.0) && (locals.var_guard1598 != 0.0)) && (locals.var_guard1599 != 0.0)) && (locals.var_guard1600 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign67670_e104373;
        locals.var_mm_rv = 0.0;

        let assign67680_e104376: f64 = if 1.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1601 = assign67680_e104376;
        locals.var_guard1601_rv = 0.0;

        let (assign67690_e104389,) = {
    if (((((locals.var_guard1597 != 0.0) && (locals.var_guard1598 != 0.0)) && (locals.var_guard1599 != 0.0)) && (locals.var_guard1600 == 0.0)) && (locals.var_guard1601 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign67690_e104389;
        locals.var_mm_rv = 0.0;

        let assign67700_e104392: f64 = if 1.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1602 = assign67700_e104392;
        locals.var_guard1602_rv = 0.0;

        let (assign67710_e104408,) = {
    if ((((((locals.var_guard1597 != 0.0) && (locals.var_guard1598 != 0.0)) && (locals.var_guard1599 != 0.0)) && (locals.var_guard1600 == 0.0)) && (locals.var_guard1601 == 0.0)) && (locals.var_guard1602 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign67710_e104408;
        locals.var_mm_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_255(
        locals: &mut StampLocals,
    ) {
        let assign67720_e104411: f64 = if 1.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1603 = assign67720_e104411;
        locals.var_guard1603_rv = 0.0;

        let (assign67730_e104430,) = {
    if (((((((locals.var_guard1597 != 0.0) && (locals.var_guard1598 != 0.0)) && (locals.var_guard1599 != 0.0)) && (locals.var_guard1600 == 0.0)) && (locals.var_guard1601 == 0.0)) && (locals.var_guard1602 == 0.0)) && (locals.var_guard1603 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign67730_e104430;
        locals.var_mm_rv = 0.0;

        let (assign67740_e104438,) = {
    if (((locals.var_guard1597 != 0.0) && (locals.var_guard1598 != 0.0)) && (locals.var_guard1599 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign67740_e104438;
        locals.var_m0_rv = 0.0;

        let mut assign67750_loop_guard: usize = 0;
        while {
            let assign67750_cond_e104447: f64 = if ((((locals.var_guard1597 != 0.0) && (locals.var_guard1598 != 0.0)) && (locals.var_guard1599 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign67750_cond_e104447 != 0.0
        } {
            assign67750_loop_guard += 1;
            assert!(assign67750_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign67750_body0_e104456, assign67750_body0_e104456_d_n0, assign67750_body0_e104456_d_n2, assign67750_body0_e104456_d_n4, assign67750_body0_e104456_d_n5, assign67750_body0_e104456_d_n6, assign67750_body0_e104456_d_n7, assign67750_body0_e104456_d_n8, assign67750_body0_e104456_d_n9, assign67750_body0_e104456_d_n10, assign67750_body0_e104456_d_n11, assign67750_body0_e104456_d_n14,) = {
    if (((locals.var_guard1597 != 0.0) && (locals.var_guard1598 != 0.0)) && (locals.var_guard1599 != 0.0)) {
        let assign67750_body0_e104454: f64 = (locals.var_dnm).sqrt();
        (assign67750_body0_e104454, (locals.var_dnm_dn0 / (2.0 * assign67750_body0_e104454)), (locals.var_dnm_dn2 / (2.0 * assign67750_body0_e104454)), (locals.var_dnm_dn4 / (2.0 * assign67750_body0_e104454)), (locals.var_dnm_dn5 / (2.0 * assign67750_body0_e104454)), (locals.var_dnm_dn6 / (2.0 * assign67750_body0_e104454)), (locals.var_dnm_dn7 / (2.0 * assign67750_body0_e104454)), (locals.var_dnm_dn8 / (2.0 * assign67750_body0_e104454)), (locals.var_dnm_dn9 / (2.0 * assign67750_body0_e104454)), (locals.var_dnm_dn10 / (2.0 * assign67750_body0_e104454)), (locals.var_dnm_dn11 / (2.0 * assign67750_body0_e104454)), (locals.var_dnm_dn14 / (2.0 * assign67750_body0_e104454)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign67750_body0_e104456;
            locals.var_dnm_dn0 = assign67750_body0_e104456_d_n0;
            locals.var_dnm_dn2 = assign67750_body0_e104456_d_n2;
            locals.var_dnm_dn4 = assign67750_body0_e104456_d_n4;
            locals.var_dnm_dn5 = assign67750_body0_e104456_d_n5;
            locals.var_dnm_dn6 = assign67750_body0_e104456_d_n6;
            locals.var_dnm_dn7 = assign67750_body0_e104456_d_n7;
            locals.var_dnm_dn8 = assign67750_body0_e104456_d_n8;
            locals.var_dnm_dn9 = assign67750_body0_e104456_d_n9;
            locals.var_dnm_dn10 = assign67750_body0_e104456_d_n10;
            locals.var_dnm_dn11 = assign67750_body0_e104456_d_n11;
            locals.var_dnm_dn14 = assign67750_body0_e104456_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign67750_body1_e104466,) = {
    if (((locals.var_guard1597 != 0.0) && (locals.var_guard1598 != 0.0)) && (locals.var_guard1599 != 0.0)) {
        let assign67750_body1_e104464: f64 = (locals.var_m0 + 1.0);
        (assign67750_body1_e104464,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign67750_body1_e104466;
            locals.var_m0_rv = 0.0;
        }

        let (assign67760_e104486, assign67760_e104486_d_n0, assign67760_e104486_d_n2, assign67760_e104486_d_n4, assign67760_e104486_d_n5, assign67760_e104486_d_n6, assign67760_e104486_d_n7, assign67760_e104486_d_n8, assign67760_e104486_d_n9, assign67760_e104486_d_n10, assign67760_e104486_d_n11, assign67760_e104486_d_n14,) = {
    if (((locals.var_guard1597 != 0.0) && (locals.var_guard1598 != 0.0)) && (locals.var_guard1599 == 0.0)) {
        let (assign67760_e104484, assign67760_e104484_d_n0, assign67760_e104484_d_n2, assign67760_e104484_d_n4, assign67760_e104484_d_n5, assign67760_e104484_d_n6, assign67760_e104484_d_n7, assign67760_e104484_d_n8, assign67760_e104484_d_n9, assign67760_e104484_d_n10, assign67760_e104484_d_n11, assign67760_e104484_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign67760_e104481: f64 = 2.0;
                let assign67760_e104482: f64 = (1.0 / assign67760_e104481);
                let assign67760_e104483: f64 = (locals.var_dnm).powf(assign67760_e104482);
                (assign67760_e104483, if 0.0 == 0.0 && ((assign67760_e104482) as f64).is_finite() && ((assign67760_e104482) as f64).fract() == 0.0 { if assign67760_e104482 == 0.0 { 0.0 } else { (assign67760_e104482 * ((locals.var_dnm).powf(assign67760_e104482 - 1.0) * locals.var_dnm_dn0)) } } else { (assign67760_e104483 * (assign67760_e104482 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign67760_e104482) as f64).is_finite() && ((assign67760_e104482) as f64).fract() == 0.0 { if assign67760_e104482 == 0.0 { 0.0 } else { (assign67760_e104482 * ((locals.var_dnm).powf(assign67760_e104482 - 1.0) * locals.var_dnm_dn2)) } } else { (assign67760_e104483 * (assign67760_e104482 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign67760_e104482) as f64).is_finite() && ((assign67760_e104482) as f64).fract() == 0.0 { if assign67760_e104482 == 0.0 { 0.0 } else { (assign67760_e104482 * ((locals.var_dnm).powf(assign67760_e104482 - 1.0) * locals.var_dnm_dn4)) } } else { (assign67760_e104483 * (assign67760_e104482 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign67760_e104482) as f64).is_finite() && ((assign67760_e104482) as f64).fract() == 0.0 { if assign67760_e104482 == 0.0 { 0.0 } else { (assign67760_e104482 * ((locals.var_dnm).powf(assign67760_e104482 - 1.0) * locals.var_dnm_dn5)) } } else { (assign67760_e104483 * (assign67760_e104482 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign67760_e104482) as f64).is_finite() && ((assign67760_e104482) as f64).fract() == 0.0 { if assign67760_e104482 == 0.0 { 0.0 } else { (assign67760_e104482 * ((locals.var_dnm).powf(assign67760_e104482 - 1.0) * locals.var_dnm_dn6)) } } else { (assign67760_e104483 * (assign67760_e104482 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign67760_e104482) as f64).is_finite() && ((assign67760_e104482) as f64).fract() == 0.0 { if assign67760_e104482 == 0.0 { 0.0 } else { (assign67760_e104482 * ((locals.var_dnm).powf(assign67760_e104482 - 1.0) * locals.var_dnm_dn7)) } } else { (assign67760_e104483 * (assign67760_e104482 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign67760_e104482) as f64).is_finite() && ((assign67760_e104482) as f64).fract() == 0.0 { if assign67760_e104482 == 0.0 { 0.0 } else { (assign67760_e104482 * ((locals.var_dnm).powf(assign67760_e104482 - 1.0) * locals.var_dnm_dn8)) } } else { (assign67760_e104483 * (assign67760_e104482 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign67760_e104482) as f64).is_finite() && ((assign67760_e104482) as f64).fract() == 0.0 { if assign67760_e104482 == 0.0 { 0.0 } else { (assign67760_e104482 * ((locals.var_dnm).powf(assign67760_e104482 - 1.0) * locals.var_dnm_dn9)) } } else { (assign67760_e104483 * (assign67760_e104482 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign67760_e104482) as f64).is_finite() && ((assign67760_e104482) as f64).fract() == 0.0 { if assign67760_e104482 == 0.0 { 0.0 } else { (assign67760_e104482 * ((locals.var_dnm).powf(assign67760_e104482 - 1.0) * locals.var_dnm_dn10)) } } else { (assign67760_e104483 * (assign67760_e104482 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign67760_e104482) as f64).is_finite() && ((assign67760_e104482) as f64).fract() == 0.0 { if assign67760_e104482 == 0.0 { 0.0 } else { (assign67760_e104482 * ((locals.var_dnm).powf(assign67760_e104482 - 1.0) * locals.var_dnm_dn11)) } } else { (assign67760_e104483 * (assign67760_e104482 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign67760_e104482) as f64).is_finite() && ((assign67760_e104482) as f64).fract() == 0.0 { if assign67760_e104482 == 0.0 { 0.0 } else { (assign67760_e104482 * ((locals.var_dnm).powf(assign67760_e104482 - 1.0) * locals.var_dnm_dn14)) } } else { (assign67760_e104483 * (assign67760_e104482 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign67760_e104484, assign67760_e104484_d_n0, assign67760_e104484_d_n2, assign67760_e104484_d_n4, assign67760_e104484_d_n5, assign67760_e104484_d_n6, assign67760_e104484_d_n7, assign67760_e104484_d_n8, assign67760_e104484_d_n9, assign67760_e104484_d_n10, assign67760_e104484_d_n11, assign67760_e104484_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign67760_e104486;
        locals.var_dnm_dn0 = assign67760_e104486_d_n0;
        locals.var_dnm_dn2 = assign67760_e104486_d_n2;
        locals.var_dnm_dn4 = assign67760_e104486_d_n4;
        locals.var_dnm_dn5 = assign67760_e104486_d_n5;
        locals.var_dnm_dn6 = assign67760_e104486_d_n6;
        locals.var_dnm_dn7 = assign67760_e104486_d_n7;
        locals.var_dnm_dn8 = assign67760_e104486_d_n8;
        locals.var_dnm_dn9 = assign67760_e104486_d_n9;
        locals.var_dnm_dn10 = assign67760_e104486_d_n10;
        locals.var_dnm_dn11 = assign67760_e104486_d_n11;
        locals.var_dnm_dn14 = assign67760_e104486_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign67770_e104494, assign67770_e104494_d_n0, assign67770_e104494_d_n2, assign67770_e104494_d_n4, assign67770_e104494_d_n5, assign67770_e104494_d_n6, assign67770_e104494_d_n7, assign67770_e104494_d_n8, assign67770_e104494_d_n9, assign67770_e104494_d_n10, assign67770_e104494_d_n11, assign67770_e104494_d_n14,) = {
    if ((locals.var_guard1597 != 0.0) && (locals.var_guard1598 != 0.0)) {
        let assign67770_e104492: f64 = (1.0 / locals.var_dnm);
        (assign67770_e104492, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign67770_e104494;
        locals.var_dnm_dn0 = assign67770_e104494_d_n0;
        locals.var_dnm_dn2 = assign67770_e104494_d_n2;
        locals.var_dnm_dn4 = assign67770_e104494_d_n4;
        locals.var_dnm_dn5 = assign67770_e104494_d_n5;
        locals.var_dnm_dn6 = assign67770_e104494_d_n6;
        locals.var_dnm_dn7 = assign67770_e104494_d_n7;
        locals.var_dnm_dn8 = assign67770_e104494_d_n8;
        locals.var_dnm_dn9 = assign67770_e104494_d_n9;
        locals.var_dnm_dn10 = assign67770_e104494_d_n10;
        locals.var_dnm_dn11 = assign67770_e104494_d_n11;
        locals.var_dnm_dn14 = assign67770_e104494_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign67780_e104504, assign67780_e104504_d_n0, assign67780_e104504_d_n2, assign67780_e104504_d_n4, assign67780_e104504_d_n5, assign67780_e104504_d_n6, assign67780_e104504_d_n7, assign67780_e104504_d_n8, assign67780_e104504_d_n9, assign67780_e104504_d_n10, assign67780_e104504_d_n11, assign67780_e104504_d_n14,) = {
    if ((locals.var_guard1597 != 0.0) && (locals.var_guard1598 != 0.0)) {
        let assign67780_e104500: f64 = (locals.var_tmf1 * 50000.0);
        let assign67780_e104502: f64 = (assign67780_e104500 * locals.var_dnm);
        (assign67780_e104502, (((locals.var_tmf1_dn0 * 50000.0) * locals.var_dnm) + (assign67780_e104500 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 50000.0) * locals.var_dnm) + (assign67780_e104500 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 50000.0) * locals.var_dnm) + (assign67780_e104500 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 50000.0) * locals.var_dnm) + (assign67780_e104500 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 50000.0) * locals.var_dnm) + (assign67780_e104500 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 50000.0) * locals.var_dnm) + (assign67780_e104500 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 50000.0) * locals.var_dnm) + (assign67780_e104500 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 50000.0) * locals.var_dnm) + (assign67780_e104500 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 50000.0) * locals.var_dnm) + (assign67780_e104500 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * 50000.0) * locals.var_dnm) + (assign67780_e104500 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * 50000.0) * locals.var_dnm) + (assign67780_e104500 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign67780_e104504;
        locals.var_tmf0_dn0 = assign67780_e104504_d_n0;
        locals.var_tmf0_dn2 = assign67780_e104504_d_n2;
        locals.var_tmf0_dn4 = assign67780_e104504_d_n4;
        locals.var_tmf0_dn5 = assign67780_e104504_d_n5;
        locals.var_tmf0_dn6 = assign67780_e104504_d_n6;
        locals.var_tmf0_dn7 = assign67780_e104504_d_n7;
        locals.var_tmf0_dn8 = assign67780_e104504_d_n8;
        locals.var_tmf0_dn9 = assign67780_e104504_d_n9;
        locals.var_tmf0_dn10 = assign67780_e104504_d_n10;
        locals.var_tmf0_dn11 = assign67780_e104504_d_n11;
        locals.var_tmf0_dn14 = assign67780_e104504_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign67790_e104516, assign67790_e104516_d_n0, assign67790_e104516_d_n2, assign67790_e104516_d_n4, assign67790_e104516_d_n5, assign67790_e104516_d_n6, assign67790_e104516_d_n7, assign67790_e104516_d_n8, assign67790_e104516_d_n9, assign67790_e104516_d_n10, assign67790_e104516_d_n11, assign67790_e104516_d_n14,) = {
    if ((locals.var_guard1597 != 0.0) && (locals.var_guard1598 != 0.0)) {
        let assign67790_e104510: f64 = (50000.0 * locals.var_xmp);
        let assign67790_e104512: f64 = (assign67790_e104510 * locals.var_dnm);
        let assign67790_e104514: f64 = (assign67790_e104512 / locals.var_arg);
        (assign67790_e104514, ((((((50000.0 * locals.var_xmp_dn0) * locals.var_dnm) + (assign67790_e104510 * locals.var_dnm_dn0)) * locals.var_arg) - (assign67790_e104512 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((50000.0 * locals.var_xmp_dn2) * locals.var_dnm) + (assign67790_e104510 * locals.var_dnm_dn2)) * locals.var_arg) - (assign67790_e104512 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((50000.0 * locals.var_xmp_dn4) * locals.var_dnm) + (assign67790_e104510 * locals.var_dnm_dn4)) * locals.var_arg) - (assign67790_e104512 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((50000.0 * locals.var_xmp_dn5) * locals.var_dnm) + (assign67790_e104510 * locals.var_dnm_dn5)) * locals.var_arg) - (assign67790_e104512 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((50000.0 * locals.var_xmp_dn6) * locals.var_dnm) + (assign67790_e104510 * locals.var_dnm_dn6)) * locals.var_arg) - (assign67790_e104512 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((50000.0 * locals.var_xmp_dn7) * locals.var_dnm) + (assign67790_e104510 * locals.var_dnm_dn7)) * locals.var_arg) - (assign67790_e104512 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((50000.0 * locals.var_xmp_dn8) * locals.var_dnm) + (assign67790_e104510 * locals.var_dnm_dn8)) * locals.var_arg) - (assign67790_e104512 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((50000.0 * locals.var_xmp_dn9) * locals.var_dnm) + (assign67790_e104510 * locals.var_dnm_dn9)) * locals.var_arg) - (assign67790_e104512 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((50000.0 * locals.var_xmp_dn10) * locals.var_dnm) + (assign67790_e104510 * locals.var_dnm_dn10)) * locals.var_arg) - (assign67790_e104512 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((50000.0 * locals.var_xmp_dn11) * locals.var_dnm) + (assign67790_e104510 * locals.var_dnm_dn11)) * locals.var_arg) - (assign67790_e104512 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((50000.0 * locals.var_xmp_dn14) * locals.var_dnm) + (assign67790_e104510 * locals.var_dnm_dn14)) * locals.var_arg) - (assign67790_e104512 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign67790_e104516;
        locals.var_t0_dn0 = assign67790_e104516_d_n0;
        locals.var_t0_dn2 = assign67790_e104516_d_n2;
        locals.var_t0_dn4 = assign67790_e104516_d_n4;
        locals.var_t0_dn5 = assign67790_e104516_d_n5;
        locals.var_t0_dn6 = assign67790_e104516_d_n6;
        locals.var_t0_dn7 = assign67790_e104516_d_n7;
        locals.var_t0_dn8 = assign67790_e104516_d_n8;
        locals.var_t0_dn9 = assign67790_e104516_d_n9;
        locals.var_t0_dn10 = assign67790_e104516_d_n10;
        locals.var_t0_dn11 = assign67790_e104516_d_n11;
        locals.var_t0_dn14 = assign67790_e104516_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign67800_e104526, assign67800_e104526_d_n0, assign67800_e104526_d_n2, assign67800_e104526_d_n4, assign67800_e104526_d_n5, assign67800_e104526_d_n6, assign67800_e104526_d_n7, assign67800_e104526_d_n8, assign67800_e104526_d_n9, assign67800_e104526_d_n10, assign67800_e104526_d_n11, assign67800_e104526_d_n14,) = {
    if ((locals.var_guard1597 != 0.0) && (locals.var_guard1598 != 0.0)) {
        let assign67800_e104522: f64 = (100000.0 - 50000.0);
        let assign67800_e104524: f64 = (assign67800_e104522 + locals.var_tmf0);
        (assign67800_e104524, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign67800_e104526;
        locals.var_t2_dn0 = assign67800_e104526_d_n0;
        locals.var_t2_dn2 = assign67800_e104526_d_n2;
        locals.var_t2_dn4 = assign67800_e104526_d_n4;
        locals.var_t2_dn5 = assign67800_e104526_d_n5;
        locals.var_t2_dn6 = assign67800_e104526_d_n6;
        locals.var_t2_dn7 = assign67800_e104526_d_n7;
        locals.var_t2_dn8 = assign67800_e104526_d_n8;
        locals.var_t2_dn9 = assign67800_e104526_d_n9;
        locals.var_t2_dn10 = assign67800_e104526_d_n10;
        locals.var_t2_dn11 = assign67800_e104526_d_n11;
        locals.var_t2_dn14 = assign67800_e104526_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign67810_e104532, assign67810_e104532_d_n0, assign67810_e104532_d_n2, assign67810_e104532_d_n4, assign67810_e104532_d_n5, assign67810_e104532_d_n6, assign67810_e104532_d_n7, assign67810_e104532_d_n8, assign67810_e104532_d_n9, assign67810_e104532_d_n10, assign67810_e104532_d_n11, assign67810_e104532_d_n14,) = {
    if ((locals.var_guard1597 != 0.0) && (locals.var_guard1598 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign67810_e104532;
        locals.var_t0_dn0 = assign67810_e104532_d_n0;
        locals.var_t0_dn2 = assign67810_e104532_d_n2;
        locals.var_t0_dn4 = assign67810_e104532_d_n4;
        locals.var_t0_dn5 = assign67810_e104532_d_n5;
        locals.var_t0_dn6 = assign67810_e104532_d_n6;
        locals.var_t0_dn7 = assign67810_e104532_d_n7;
        locals.var_t0_dn8 = assign67810_e104532_d_n8;
        locals.var_t0_dn9 = assign67810_e104532_d_n9;
        locals.var_t0_dn10 = assign67810_e104532_d_n10;
        locals.var_t0_dn11 = assign67810_e104532_d_n11;
        locals.var_t0_dn14 = assign67810_e104532_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign67820_e104539, assign67820_e104539_d_n0, assign67820_e104539_d_n2, assign67820_e104539_d_n4, assign67820_e104539_d_n5, assign67820_e104539_d_n6, assign67820_e104539_d_n7, assign67820_e104539_d_n8, assign67820_e104539_d_n9, assign67820_e104539_d_n10, assign67820_e104539_d_n11, assign67820_e104539_d_n14,) = {
    if ((locals.var_guard1597 != 0.0) && (locals.var_guard1598 == 0.0)) {
        (locals.var_ibreakhe, locals.var_ibreakhe_dn0, locals.var_ibreakhe_dn2, locals.var_ibreakhe_dn4, locals.var_ibreakhe_dn5, locals.var_ibreakhe_dn6, locals.var_ibreakhe_dn7, locals.var_ibreakhe_dn8, locals.var_ibreakhe_dn9, locals.var_ibreakhe_dn10, locals.var_ibreakhe_dn11, locals.var_ibreakhe_dn14,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign67820_e104539;
        locals.var_t2_dn0 = assign67820_e104539_d_n0;
        locals.var_t2_dn2 = assign67820_e104539_d_n2;
        locals.var_t2_dn4 = assign67820_e104539_d_n4;
        locals.var_t2_dn5 = assign67820_e104539_d_n5;
        locals.var_t2_dn6 = assign67820_e104539_d_n6;
        locals.var_t2_dn7 = assign67820_e104539_d_n7;
        locals.var_t2_dn8 = assign67820_e104539_d_n8;
        locals.var_t2_dn9 = assign67820_e104539_d_n9;
        locals.var_t2_dn10 = assign67820_e104539_d_n10;
        locals.var_t2_dn11 = assign67820_e104539_d_n11;
        locals.var_t2_dn14 = assign67820_e104539_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign67830_e104546, assign67830_e104546_d_n0, assign67830_e104546_d_n2, assign67830_e104546_d_n4, assign67830_e104546_d_n5, assign67830_e104546_d_n6, assign67830_e104546_d_n7, assign67830_e104546_d_n8, assign67830_e104546_d_n9, assign67830_e104546_d_n10, assign67830_e104546_d_n11, assign67830_e104546_d_n14,) = {
    if ((locals.var_guard1597 != 0.0) && (locals.var_guard1598 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign67830_e104546;
        locals.var_t0_dn0 = assign67830_e104546_d_n0;
        locals.var_t0_dn2 = assign67830_e104546_d_n2;
        locals.var_t0_dn4 = assign67830_e104546_d_n4;
        locals.var_t0_dn5 = assign67830_e104546_d_n5;
        locals.var_t0_dn6 = assign67830_e104546_d_n6;
        locals.var_t0_dn7 = assign67830_e104546_d_n7;
        locals.var_t0_dn8 = assign67830_e104546_d_n8;
        locals.var_t0_dn9 = assign67830_e104546_d_n9;
        locals.var_t0_dn10 = assign67830_e104546_d_n10;
        locals.var_t0_dn11 = assign67830_e104546_d_n11;
        locals.var_t0_dn14 = assign67830_e104546_d_n14;
        locals.var_t0_rv = 0.0;

        let assign67860_e104562: f64 = (locals.var_isub + locals.var_isubld);
        let assign67860_e104572: f64 = if (((assign67860_e104562 > 0.0) && (locals.var_uc_ibpc1 != 0.0)) && (locals.var_uc_codep == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1604 = assign67860_e104572;
        locals.var_guard1604_rv = 0.0;

        let (assign67870_e104580, assign67870_e104580_d_n0, assign67870_e104580_d_n2, assign67870_e104580_d_n4, assign67870_e104580_d_n5, assign67870_e104580_d_n6, assign67870_e104580_d_n7, assign67870_e104580_d_n8, assign67870_e104580_d_n9, assign67870_e104580_d_n10, assign67870_e104580_d_n11, assign67870_e104580_d_n14,) = {
    if (locals.var_guard1604 != 0.0) {
        let assign67870_e104577: f64 = (locals.var_uc_ibpc2 * locals.var_dvth);
        let assign67870_e104578: f64 = (1.0 + assign67870_e104577);
        (assign67870_e104578, (locals.var_uc_ibpc2 * locals.var_dvth_dn0), (locals.var_uc_ibpc2 * locals.var_dvth_dn2), (locals.var_uc_ibpc2 * locals.var_dvth_dn4), (locals.var_uc_ibpc2 * locals.var_dvth_dn5), (locals.var_uc_ibpc2 * locals.var_dvth_dn6), (locals.var_uc_ibpc2 * locals.var_dvth_dn7), (locals.var_uc_ibpc2 * locals.var_dvth_dn8), (locals.var_uc_ibpc2 * locals.var_dvth_dn9), (locals.var_uc_ibpc2 * locals.var_dvth_dn10), (locals.var_uc_ibpc2 * locals.var_dvth_dn11), (locals.var_uc_ibpc2 * locals.var_dvth_dn14),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign67870_e104580;
        locals.var_t0_dn0 = assign67870_e104580_d_n0;
        locals.var_t0_dn2 = assign67870_e104580_d_n2;
        locals.var_t0_dn4 = assign67870_e104580_d_n4;
        locals.var_t0_dn5 = assign67870_e104580_d_n5;
        locals.var_t0_dn6 = assign67870_e104580_d_n6;
        locals.var_t0_dn7 = assign67870_e104580_d_n7;
        locals.var_t0_dn8 = assign67870_e104580_d_n8;
        locals.var_t0_dn9 = assign67870_e104580_d_n9;
        locals.var_t0_dn10 = assign67870_e104580_d_n10;
        locals.var_t0_dn11 = assign67870_e104580_d_n11;
        locals.var_t0_dn14 = assign67870_e104580_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign67880_e104586, assign67880_e104586_d_n0, assign67880_e104586_d_n2, assign67880_e104586_d_n4, assign67880_e104586_d_n5, assign67880_e104586_d_n6, assign67880_e104586_d_n7, assign67880_e104586_d_n8, assign67880_e104586_d_n9, assign67880_e104586_d_n10, assign67880_e104586_d_n11, assign67880_e104586_d_n14,) = {
    if (locals.var_guard1604 != 0.0) {
        let assign67880_e104584: f64 = (locals.var_isub + locals.var_isubld);
        (assign67880_e104584, (locals.var_isub_dn0 + locals.var_isubld_dn0), (locals.var_isub_dn2 + locals.var_isubld_dn2), (locals.var_isub_dn4 + locals.var_isubld_dn4), (locals.var_isub_dn5 + locals.var_isubld_dn5), (locals.var_isub_dn6 + locals.var_isubld_dn6), (locals.var_isub_dn7 + locals.var_isubld_dn7), (locals.var_isub_dn8 + locals.var_isubld_dn8), (locals.var_isub_dn9 + locals.var_isubld_dn9), (locals.var_isub_dn10 + locals.var_isubld_dn10), (locals.var_isub_dn11 + locals.var_isubld_dn11), (locals.var_isub_dn14 + locals.var_isubld_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign67880_e104586;
        locals.var_t1_dn0 = assign67880_e104586_d_n0;
        locals.var_t1_dn2 = assign67880_e104586_d_n2;
        locals.var_t1_dn4 = assign67880_e104586_d_n4;
        locals.var_t1_dn5 = assign67880_e104586_d_n5;
        locals.var_t1_dn6 = assign67880_e104586_d_n6;
        locals.var_t1_dn7 = assign67880_e104586_d_n7;
        locals.var_t1_dn8 = assign67880_e104586_d_n8;
        locals.var_t1_dn9 = assign67880_e104586_d_n9;
        locals.var_t1_dn10 = assign67880_e104586_d_n10;
        locals.var_t1_dn11 = assign67880_e104586_d_n11;
        locals.var_t1_dn14 = assign67880_e104586_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign67890_e104594, assign67890_e104594_d_n0, assign67890_e104594_d_n2, assign67890_e104594_d_n4, assign67890_e104594_d_n5, assign67890_e104594_d_n6, assign67890_e104594_d_n7, assign67890_e104594_d_n8, assign67890_e104594_d_n9, assign67890_e104594_d_n10, assign67890_e104594_d_n11, assign67890_e104594_d_n14,) = {
    if (locals.var_guard1604 != 0.0) {
        let assign67890_e104590: f64 = (locals.var_uc_ibpc1 * locals.var_t0);
        let assign67890_e104592: f64 = (assign67890_e104590 * locals.var_t1);
        (assign67890_e104592, (((locals.var_uc_ibpc1 * locals.var_t0_dn0) * locals.var_t1) + (assign67890_e104590 * locals.var_t1_dn0)), (((locals.var_uc_ibpc1 * locals.var_t0_dn2) * locals.var_t1) + (assign67890_e104590 * locals.var_t1_dn2)), (((locals.var_uc_ibpc1 * locals.var_t0_dn4) * locals.var_t1) + (assign67890_e104590 * locals.var_t1_dn4)), (((locals.var_uc_ibpc1 * locals.var_t0_dn5) * locals.var_t1) + (assign67890_e104590 * locals.var_t1_dn5)), (((locals.var_uc_ibpc1 * locals.var_t0_dn6) * locals.var_t1) + (assign67890_e104590 * locals.var_t1_dn6)), (((locals.var_uc_ibpc1 * locals.var_t0_dn7) * locals.var_t1) + (assign67890_e104590 * locals.var_t1_dn7)), (((locals.var_uc_ibpc1 * locals.var_t0_dn8) * locals.var_t1) + (assign67890_e104590 * locals.var_t1_dn8)), (((locals.var_uc_ibpc1 * locals.var_t0_dn9) * locals.var_t1) + (assign67890_e104590 * locals.var_t1_dn9)), (((locals.var_uc_ibpc1 * locals.var_t0_dn10) * locals.var_t1) + (assign67890_e104590 * locals.var_t1_dn10)), (((locals.var_uc_ibpc1 * locals.var_t0_dn11) * locals.var_t1) + (assign67890_e104590 * locals.var_t1_dn11)), (((locals.var_uc_ibpc1 * locals.var_t0_dn14) * locals.var_t1) + (assign67890_e104590 * locals.var_t1_dn14)),)
    } else {
        (locals.var_dvbsibpc, locals.var_dvbsibpc_dn0, locals.var_dvbsibpc_dn2, locals.var_dvbsibpc_dn4, locals.var_dvbsibpc_dn5, locals.var_dvbsibpc_dn6, locals.var_dvbsibpc_dn7, locals.var_dvbsibpc_dn8, locals.var_dvbsibpc_dn9, locals.var_dvbsibpc_dn10, locals.var_dvbsibpc_dn11, locals.var_dvbsibpc_dn14,)
    }
};
        locals.var_dvbsibpc = assign67890_e104594;
        locals.var_dvbsibpc_dn0 = assign67890_e104594_d_n0;
        locals.var_dvbsibpc_dn2 = assign67890_e104594_d_n2;
        locals.var_dvbsibpc_dn4 = assign67890_e104594_d_n4;
        locals.var_dvbsibpc_dn5 = assign67890_e104594_d_n5;
        locals.var_dvbsibpc_dn6 = assign67890_e104594_d_n6;
        locals.var_dvbsibpc_dn7 = assign67890_e104594_d_n7;
        locals.var_dvbsibpc_dn8 = assign67890_e104594_d_n8;
        locals.var_dvbsibpc_dn9 = assign67890_e104594_d_n9;
        locals.var_dvbsibpc_dn10 = assign67890_e104594_d_n10;
        locals.var_dvbsibpc_dn11 = assign67890_e104594_d_n11;
        locals.var_dvbsibpc_dn14 = assign67890_e104594_d_n14;
        locals.var_dvbsibpc_rv = 0.0;

        let (assign67900_e104600, assign67900_e104600_d_n0, assign67900_e104600_d_n2, assign67900_e104600_d_n4, assign67900_e104600_d_n5, assign67900_e104600_d_n6, assign67900_e104600_d_n7, assign67900_e104600_d_n8, assign67900_e104600_d_n9, assign67900_e104600_d_n10, assign67900_e104600_d_n11, assign67900_e104600_d_n14,) = {
    if (locals.var_guard1604 != 0.0) {
        let assign67900_e104598: f64 = (1.0 / locals.var_xi0);
        (assign67900_e104598, (-(locals.var_xi0_dn0 / (locals.var_xi0 * locals.var_xi0))), (-(locals.var_xi0_dn2 / (locals.var_xi0 * locals.var_xi0))), (-(locals.var_xi0_dn4 / (locals.var_xi0 * locals.var_xi0))), (-(locals.var_xi0_dn5 / (locals.var_xi0 * locals.var_xi0))), (-(locals.var_xi0_dn6 / (locals.var_xi0 * locals.var_xi0))), (-(locals.var_xi0_dn7 / (locals.var_xi0 * locals.var_xi0))), (-(locals.var_xi0_dn8 / (locals.var_xi0 * locals.var_xi0))), (-(locals.var_xi0_dn9 / (locals.var_xi0 * locals.var_xi0))), (-(locals.var_xi0_dn10 / (locals.var_xi0 * locals.var_xi0))), (-(locals.var_xi0_dn11 / (locals.var_xi0 * locals.var_xi0))), (-(locals.var_xi0_dn14 / (locals.var_xi0 * locals.var_xi0))),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign67900_e104600;
        locals.var_t10_dn0 = assign67900_e104600_d_n0;
        locals.var_t10_dn2 = assign67900_e104600_d_n2;
        locals.var_t10_dn4 = assign67900_e104600_d_n4;
        locals.var_t10_dn5 = assign67900_e104600_d_n5;
        locals.var_t10_dn6 = assign67900_e104600_d_n6;
        locals.var_t10_dn7 = assign67900_e104600_d_n7;
        locals.var_t10_dn8 = assign67900_e104600_d_n8;
        locals.var_t10_dn9 = assign67900_e104600_d_n9;
        locals.var_t10_dn10 = assign67900_e104600_d_n10;
        locals.var_t10_dn11 = assign67900_e104600_d_n11;
        locals.var_t10_dn14 = assign67900_e104600_d_n14;
        locals.var_t10_rv = 0.0;

        let (assign67910_e104608, assign67910_e104608_d_n0, assign67910_e104608_d_n2, assign67910_e104608_d_n4, assign67910_e104608_d_n5, assign67910_e104608_d_n6, assign67910_e104608_d_n7, assign67910_e104608_d_n8, assign67910_e104608_d_n9, assign67910_e104608_d_n10, assign67910_e104608_d_n11, assign67910_e104608_d_n14,) = {
    if (locals.var_guard1604 != 0.0) {
        let assign67910_e104604: f64 = (locals.var_beta * locals.var_dvbsibpc);
        let assign67910_e104606: f64 = (assign67910_e104604 * locals.var_t10);
        (assign67910_e104606, ((((locals.var_beta_dn0 * locals.var_dvbsibpc) + (locals.var_beta * locals.var_dvbsibpc_dn0)) * locals.var_t10) + (assign67910_e104604 * locals.var_t10_dn0)), ((((locals.var_beta_dn2 * locals.var_dvbsibpc) + (locals.var_beta * locals.var_dvbsibpc_dn2)) * locals.var_t10) + (assign67910_e104604 * locals.var_t10_dn2)), ((((locals.var_beta_dn4 * locals.var_dvbsibpc) + (locals.var_beta * locals.var_dvbsibpc_dn4)) * locals.var_t10) + (assign67910_e104604 * locals.var_t10_dn4)), ((((locals.var_beta_dn5 * locals.var_dvbsibpc) + (locals.var_beta * locals.var_dvbsibpc_dn5)) * locals.var_t10) + (assign67910_e104604 * locals.var_t10_dn5)), ((((locals.var_beta_dn6 * locals.var_dvbsibpc) + (locals.var_beta * locals.var_dvbsibpc_dn6)) * locals.var_t10) + (assign67910_e104604 * locals.var_t10_dn6)), ((((locals.var_beta_dn7 * locals.var_dvbsibpc) + (locals.var_beta * locals.var_dvbsibpc_dn7)) * locals.var_t10) + (assign67910_e104604 * locals.var_t10_dn7)), ((((locals.var_beta_dn8 * locals.var_dvbsibpc) + (locals.var_beta * locals.var_dvbsibpc_dn8)) * locals.var_t10) + (assign67910_e104604 * locals.var_t10_dn8)), ((((locals.var_beta_dn9 * locals.var_dvbsibpc) + (locals.var_beta * locals.var_dvbsibpc_dn9)) * locals.var_t10) + (assign67910_e104604 * locals.var_t10_dn9)), ((((locals.var_beta_dn10 * locals.var_dvbsibpc) + (locals.var_beta * locals.var_dvbsibpc_dn10)) * locals.var_t10) + (assign67910_e104604 * locals.var_t10_dn10)), ((((locals.var_beta_dn11 * locals.var_dvbsibpc) + (locals.var_beta * locals.var_dvbsibpc_dn11)) * locals.var_t10) + (assign67910_e104604 * locals.var_t10_dn11)), ((((locals.var_beta_dn14 * locals.var_dvbsibpc) + (locals.var_beta * locals.var_dvbsibpc_dn14)) * locals.var_t10) + (assign67910_e104604 * locals.var_t10_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign67910_e104608;
        locals.var_t1_dn0 = assign67910_e104608_d_n0;
        locals.var_t1_dn2 = assign67910_e104608_d_n2;
        locals.var_t1_dn4 = assign67910_e104608_d_n4;
        locals.var_t1_dn5 = assign67910_e104608_d_n5;
        locals.var_t1_dn6 = assign67910_e104608_d_n6;
        locals.var_t1_dn7 = assign67910_e104608_d_n7;
        locals.var_t1_dn8 = assign67910_e104608_d_n8;
        locals.var_t1_dn9 = assign67910_e104608_d_n9;
        locals.var_t1_dn10 = assign67910_e104608_d_n10;
        locals.var_t1_dn11 = assign67910_e104608_d_n11;
        locals.var_t1_dn14 = assign67910_e104608_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign67920_e104614, assign67920_e104614_d_n0, assign67920_e104614_d_n2, assign67920_e104614_d_n4, assign67920_e104614_d_n5, assign67920_e104614_d_n6, assign67920_e104614_d_n7, assign67920_e104614_d_n8, assign67920_e104614_d_n9, assign67920_e104614_d_n10, assign67920_e104614_d_n11, assign67920_e104614_d_n14,) = {
    if (locals.var_guard1604 != 0.0) {
        let assign67920_e104612: f64 = (locals.var_t10 * locals.var_t10);
        (assign67920_e104612, ((locals.var_t10_dn0 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn0)), ((locals.var_t10_dn2 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn2)), ((locals.var_t10_dn4 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn4)), ((locals.var_t10_dn5 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn5)), ((locals.var_t10_dn6 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn6)), ((locals.var_t10_dn7 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn7)), ((locals.var_t10_dn8 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn8)), ((locals.var_t10_dn9 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn9)), ((locals.var_t10_dn10 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn10)), ((locals.var_t10_dn11 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn11)), ((locals.var_t10_dn14 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn14)),)
    } else {
        (locals.var_t11, locals.var_t11_dn0, locals.var_t11_dn2, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11, locals.var_t11_dn14,)
    }
};
        locals.var_t11 = assign67920_e104614;
        locals.var_t11_dn0 = assign67920_e104614_d_n0;
        locals.var_t11_dn2 = assign67920_e104614_d_n2;
        locals.var_t11_dn4 = assign67920_e104614_d_n4;
        locals.var_t11_dn5 = assign67920_e104614_d_n5;
        locals.var_t11_dn6 = assign67920_e104614_d_n6;
        locals.var_t11_dn7 = assign67920_e104614_d_n7;
        locals.var_t11_dn8 = assign67920_e104614_d_n8;
        locals.var_t11_dn9 = assign67920_e104614_d_n9;
        locals.var_t11_dn10 = assign67920_e104614_d_n10;
        locals.var_t11_dn11 = assign67920_e104614_d_n11;
        locals.var_t11_dn14 = assign67920_e104614_d_n14;
        locals.var_t11_rv = 0.0;

        let (assign67930_e104620, assign67930_e104620_d_n0, assign67930_e104620_d_n2, assign67930_e104620_d_n4, assign67930_e104620_d_n5, assign67930_e104620_d_n6, assign67930_e104620_d_n7, assign67930_e104620_d_n8, assign67930_e104620_d_n9, assign67930_e104620_d_n10, assign67930_e104620_d_n11, assign67930_e104620_d_n14,) = {
    if (locals.var_guard1604 != 0.0) {
        let assign67930_e104618: f64 = (1.0 / locals.var_xil);
        (assign67930_e104618, (-(locals.var_xil_dn0 / (locals.var_xil * locals.var_xil))), (-(locals.var_xil_dn2 / (locals.var_xil * locals.var_xil))), (-(locals.var_xil_dn4 / (locals.var_xil * locals.var_xil))), (-(locals.var_xil_dn5 / (locals.var_xil * locals.var_xil))), (-(locals.var_xil_dn6 / (locals.var_xil * locals.var_xil))), (-(locals.var_xil_dn7 / (locals.var_xil * locals.var_xil))), (-(locals.var_xil_dn8 / (locals.var_xil * locals.var_xil))), (-(locals.var_xil_dn9 / (locals.var_xil * locals.var_xil))), (-(locals.var_xil_dn10 / (locals.var_xil * locals.var_xil))), (-(locals.var_xil_dn11 / (locals.var_xil * locals.var_xil))), (-(locals.var_xil_dn14 / (locals.var_xil * locals.var_xil))),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign67930_e104620;
        locals.var_t10_dn0 = assign67930_e104620_d_n0;
        locals.var_t10_dn2 = assign67930_e104620_d_n2;
        locals.var_t10_dn4 = assign67930_e104620_d_n4;
        locals.var_t10_dn5 = assign67930_e104620_d_n5;
        locals.var_t10_dn6 = assign67930_e104620_d_n6;
        locals.var_t10_dn7 = assign67930_e104620_d_n7;
        locals.var_t10_dn8 = assign67930_e104620_d_n8;
        locals.var_t10_dn9 = assign67930_e104620_d_n9;
        locals.var_t10_dn10 = assign67930_e104620_d_n10;
        locals.var_t10_dn11 = assign67930_e104620_d_n11;
        locals.var_t10_dn14 = assign67930_e104620_d_n14;
        locals.var_t10_rv = 0.0;

        let (assign67940_e104628, assign67940_e104628_d_n0, assign67940_e104628_d_n2, assign67940_e104628_d_n4, assign67940_e104628_d_n5, assign67940_e104628_d_n6, assign67940_e104628_d_n7, assign67940_e104628_d_n8, assign67940_e104628_d_n9, assign67940_e104628_d_n10, assign67940_e104628_d_n11, assign67940_e104628_d_n14,) = {
    if (locals.var_guard1604 != 0.0) {
        let assign67940_e104624: f64 = (locals.var_beta * locals.var_dvbsibpc);
        let assign67940_e104626: f64 = (assign67940_e104624 * locals.var_t10);
        (assign67940_e104626, ((((locals.var_beta_dn0 * locals.var_dvbsibpc) + (locals.var_beta * locals.var_dvbsibpc_dn0)) * locals.var_t10) + (assign67940_e104624 * locals.var_t10_dn0)), ((((locals.var_beta_dn2 * locals.var_dvbsibpc) + (locals.var_beta * locals.var_dvbsibpc_dn2)) * locals.var_t10) + (assign67940_e104624 * locals.var_t10_dn2)), ((((locals.var_beta_dn4 * locals.var_dvbsibpc) + (locals.var_beta * locals.var_dvbsibpc_dn4)) * locals.var_t10) + (assign67940_e104624 * locals.var_t10_dn4)), ((((locals.var_beta_dn5 * locals.var_dvbsibpc) + (locals.var_beta * locals.var_dvbsibpc_dn5)) * locals.var_t10) + (assign67940_e104624 * locals.var_t10_dn5)), ((((locals.var_beta_dn6 * locals.var_dvbsibpc) + (locals.var_beta * locals.var_dvbsibpc_dn6)) * locals.var_t10) + (assign67940_e104624 * locals.var_t10_dn6)), ((((locals.var_beta_dn7 * locals.var_dvbsibpc) + (locals.var_beta * locals.var_dvbsibpc_dn7)) * locals.var_t10) + (assign67940_e104624 * locals.var_t10_dn7)), ((((locals.var_beta_dn8 * locals.var_dvbsibpc) + (locals.var_beta * locals.var_dvbsibpc_dn8)) * locals.var_t10) + (assign67940_e104624 * locals.var_t10_dn8)), ((((locals.var_beta_dn9 * locals.var_dvbsibpc) + (locals.var_beta * locals.var_dvbsibpc_dn9)) * locals.var_t10) + (assign67940_e104624 * locals.var_t10_dn9)), ((((locals.var_beta_dn10 * locals.var_dvbsibpc) + (locals.var_beta * locals.var_dvbsibpc_dn10)) * locals.var_t10) + (assign67940_e104624 * locals.var_t10_dn10)), ((((locals.var_beta_dn11 * locals.var_dvbsibpc) + (locals.var_beta * locals.var_dvbsibpc_dn11)) * locals.var_t10) + (assign67940_e104624 * locals.var_t10_dn11)), ((((locals.var_beta_dn14 * locals.var_dvbsibpc) + (locals.var_beta * locals.var_dvbsibpc_dn14)) * locals.var_t10) + (assign67940_e104624 * locals.var_t10_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign67940_e104628;
        locals.var_t2_dn0 = assign67940_e104628_d_n0;
        locals.var_t2_dn2 = assign67940_e104628_d_n2;
        locals.var_t2_dn4 = assign67940_e104628_d_n4;
        locals.var_t2_dn5 = assign67940_e104628_d_n5;
        locals.var_t2_dn6 = assign67940_e104628_d_n6;
        locals.var_t2_dn7 = assign67940_e104628_d_n7;
        locals.var_t2_dn8 = assign67940_e104628_d_n8;
        locals.var_t2_dn9 = assign67940_e104628_d_n9;
        locals.var_t2_dn10 = assign67940_e104628_d_n10;
        locals.var_t2_dn11 = assign67940_e104628_d_n11;
        locals.var_t2_dn14 = assign67940_e104628_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign67950_e104634, assign67950_e104634_d_n0, assign67950_e104634_d_n2, assign67950_e104634_d_n4, assign67950_e104634_d_n5, assign67950_e104634_d_n6, assign67950_e104634_d_n7, assign67950_e104634_d_n8, assign67950_e104634_d_n9, assign67950_e104634_d_n10, assign67950_e104634_d_n11, assign67950_e104634_d_n14,) = {
    if (locals.var_guard1604 != 0.0) {
        let assign67950_e104632: f64 = (locals.var_t10 * locals.var_t10);
        (assign67950_e104632, ((locals.var_t10_dn0 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn0)), ((locals.var_t10_dn2 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn2)), ((locals.var_t10_dn4 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn4)), ((locals.var_t10_dn5 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn5)), ((locals.var_t10_dn6 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn6)), ((locals.var_t10_dn7 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn7)), ((locals.var_t10_dn8 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn8)), ((locals.var_t10_dn9 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn9)), ((locals.var_t10_dn10 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn10)), ((locals.var_t10_dn11 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn11)), ((locals.var_t10_dn14 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn14)),)
    } else {
        (locals.var_t11, locals.var_t11_dn0, locals.var_t11_dn2, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11, locals.var_t11_dn14,)
    }
};
        locals.var_t11 = assign67950_e104634;
        locals.var_t11_dn0 = assign67950_e104634_d_n0;
        locals.var_t11_dn2 = assign67950_e104634_d_n2;
        locals.var_t11_dn4 = assign67950_e104634_d_n4;
        locals.var_t11_dn5 = assign67950_e104634_d_n5;
        locals.var_t11_dn6 = assign67950_e104634_d_n6;
        locals.var_t11_dn7 = assign67950_e104634_d_n7;
        locals.var_t11_dn8 = assign67950_e104634_d_n8;
        locals.var_t11_dn9 = assign67950_e104634_d_n9;
        locals.var_t11_dn10 = assign67950_e104634_d_n10;
        locals.var_t11_dn11 = assign67950_e104634_d_n11;
        locals.var_t11_dn14 = assign67950_e104634_d_n14;
        locals.var_t11_rv = 0.0;

        let (assign67960_e104646, assign67960_e104646_d_n0, assign67960_e104646_d_n2, assign67960_e104646_d_n4, assign67960_e104646_d_n5, assign67960_e104646_d_n6, assign67960_e104646_d_n7, assign67960_e104646_d_n8, assign67960_e104646_d_n9, assign67960_e104646_d_n10, assign67960_e104646_d_n11, assign67960_e104646_d_n14,) = {
    if (locals.var_guard1604 != 0.0) {
        let assign67960_e104639: f64 = (locals.var_xilp32 * locals.var_t2);
        let assign67960_e104642: f64 = (locals.var_xi0p32 * locals.var_t1);
        let assign67960_e104643: f64 = (assign67960_e104639 - assign67960_e104642);
        let assign67960_e104644: f64 = (locals.var_cnst0 * assign67960_e104643);
        (assign67960_e104644, ((locals.var_cnst0_dn0 * assign67960_e104643) + (locals.var_cnst0 * (((locals.var_xilp32_dn0 * locals.var_t2) + (locals.var_xilp32 * locals.var_t2_dn0)) - ((locals.var_xi0p32_dn0 * locals.var_t1) + (locals.var_xi0p32 * locals.var_t1_dn0))))), ((locals.var_cnst0_dn2 * assign67960_e104643) + (locals.var_cnst0 * (((locals.var_xilp32_dn2 * locals.var_t2) + (locals.var_xilp32 * locals.var_t2_dn2)) - ((locals.var_xi0p32_dn2 * locals.var_t1) + (locals.var_xi0p32 * locals.var_t1_dn2))))), ((locals.var_cnst0_dn4 * assign67960_e104643) + (locals.var_cnst0 * (((locals.var_xilp32_dn4 * locals.var_t2) + (locals.var_xilp32 * locals.var_t2_dn4)) - ((locals.var_xi0p32_dn4 * locals.var_t1) + (locals.var_xi0p32 * locals.var_t1_dn4))))), ((locals.var_cnst0_dn5 * assign67960_e104643) + (locals.var_cnst0 * (((locals.var_xilp32_dn5 * locals.var_t2) + (locals.var_xilp32 * locals.var_t2_dn5)) - ((locals.var_xi0p32_dn5 * locals.var_t1) + (locals.var_xi0p32 * locals.var_t1_dn5))))), ((locals.var_cnst0_dn6 * assign67960_e104643) + (locals.var_cnst0 * (((locals.var_xilp32_dn6 * locals.var_t2) + (locals.var_xilp32 * locals.var_t2_dn6)) - ((locals.var_xi0p32_dn6 * locals.var_t1) + (locals.var_xi0p32 * locals.var_t1_dn6))))), ((locals.var_cnst0_dn7 * assign67960_e104643) + (locals.var_cnst0 * (((locals.var_xilp32_dn7 * locals.var_t2) + (locals.var_xilp32 * locals.var_t2_dn7)) - ((locals.var_xi0p32_dn7 * locals.var_t1) + (locals.var_xi0p32 * locals.var_t1_dn7))))), ((locals.var_cnst0_dn8 * assign67960_e104643) + (locals.var_cnst0 * (((locals.var_xilp32_dn8 * locals.var_t2) + (locals.var_xilp32 * locals.var_t2_dn8)) - ((locals.var_xi0p32_dn8 * locals.var_t1) + (locals.var_xi0p32 * locals.var_t1_dn8))))), ((locals.var_cnst0_dn9 * assign67960_e104643) + (locals.var_cnst0 * (((locals.var_xilp32_dn9 * locals.var_t2) + (locals.var_xilp32 * locals.var_t2_dn9)) - ((locals.var_xi0p32_dn9 * locals.var_t1) + (locals.var_xi0p32 * locals.var_t1_dn9))))), ((locals.var_cnst0_dn10 * assign67960_e104643) + (locals.var_cnst0 * (((locals.var_xilp32_dn10 * locals.var_t2) + (locals.var_xilp32 * locals.var_t2_dn10)) - ((locals.var_xi0p32_dn10 * locals.var_t1) + (locals.var_xi0p32 * locals.var_t1_dn10))))), ((locals.var_cnst0_dn11 * assign67960_e104643) + (locals.var_cnst0 * (((locals.var_xilp32_dn11 * locals.var_t2) + (locals.var_xilp32 * locals.var_t2_dn11)) - ((locals.var_xi0p32_dn11 * locals.var_t1) + (locals.var_xi0p32 * locals.var_t1_dn11))))), ((locals.var_cnst0_dn14 * assign67960_e104643) + (locals.var_cnst0 * (((locals.var_xilp32_dn14 * locals.var_t2) + (locals.var_xilp32 * locals.var_t2_dn14)) - ((locals.var_xi0p32_dn14 * locals.var_t1) + (locals.var_xi0p32 * locals.var_t1_dn14))))),)
    } else {
        (locals.var_dg3, locals.var_dg3_dn0, locals.var_dg3_dn2, locals.var_dg3_dn4, locals.var_dg3_dn5, locals.var_dg3_dn6, locals.var_dg3_dn7, locals.var_dg3_dn8, locals.var_dg3_dn9, locals.var_dg3_dn10, locals.var_dg3_dn11, locals.var_dg3_dn14,)
    }
};
        locals.var_dg3 = assign67960_e104646;
        locals.var_dg3_dn0 = assign67960_e104646_d_n0;
        locals.var_dg3_dn2 = assign67960_e104646_d_n2;
        locals.var_dg3_dn4 = assign67960_e104646_d_n4;
        locals.var_dg3_dn5 = assign67960_e104646_d_n5;
        locals.var_dg3_dn6 = assign67960_e104646_d_n6;
        locals.var_dg3_dn7 = assign67960_e104646_d_n7;
        locals.var_dg3_dn8 = assign67960_e104646_d_n8;
        locals.var_dg3_dn9 = assign67960_e104646_d_n9;
        locals.var_dg3_dn10 = assign67960_e104646_d_n10;
        locals.var_dg3_dn11 = assign67960_e104646_d_n11;
        locals.var_dg3_dn14 = assign67960_e104646_d_n14;
        locals.var_dg3_rv = 0.0;

        let (assign67970_e104661, assign67970_e104661_d_n0, assign67970_e104661_d_n2, assign67970_e104661_d_n4, assign67970_e104661_d_n5, assign67970_e104661_d_n6, assign67970_e104661_d_n7, assign67970_e104661_d_n8, assign67970_e104661_d_n9, assign67970_e104661_d_n10, assign67970_e104661_d_n11, assign67970_e104661_d_n14,) = {
    if (locals.var_guard1604 != 0.0) {
        let assign67970_e104650: f64 = (locals.var_cnst0 * 0.5);
        let assign67970_e104652: f64 = (-locals.var_xilp12);
        let assign67970_e104654: f64 = (assign67970_e104652 * locals.var_t2);
        let assign67970_e104657: f64 = (locals.var_xi0p12 * locals.var_t1);
        let assign67970_e104658: f64 = (assign67970_e104654 + assign67970_e104657);
        let assign67970_e104659: f64 = (assign67970_e104650 * assign67970_e104658);
        (assign67970_e104659, (((locals.var_cnst0_dn0 * 0.5) * assign67970_e104658) + (assign67970_e104650 * ((((-locals.var_xilp12_dn0) * locals.var_t2) + (assign67970_e104652 * locals.var_t2_dn0)) + ((locals.var_xi0p12_dn0 * locals.var_t1) + (locals.var_xi0p12 * locals.var_t1_dn0))))), (((locals.var_cnst0_dn2 * 0.5) * assign67970_e104658) + (assign67970_e104650 * ((((-locals.var_xilp12_dn2) * locals.var_t2) + (assign67970_e104652 * locals.var_t2_dn2)) + ((locals.var_xi0p12_dn2 * locals.var_t1) + (locals.var_xi0p12 * locals.var_t1_dn2))))), (((locals.var_cnst0_dn4 * 0.5) * assign67970_e104658) + (assign67970_e104650 * ((((-locals.var_xilp12_dn4) * locals.var_t2) + (assign67970_e104652 * locals.var_t2_dn4)) + ((locals.var_xi0p12_dn4 * locals.var_t1) + (locals.var_xi0p12 * locals.var_t1_dn4))))), (((locals.var_cnst0_dn5 * 0.5) * assign67970_e104658) + (assign67970_e104650 * ((((-locals.var_xilp12_dn5) * locals.var_t2) + (assign67970_e104652 * locals.var_t2_dn5)) + ((locals.var_xi0p12_dn5 * locals.var_t1) + (locals.var_xi0p12 * locals.var_t1_dn5))))), (((locals.var_cnst0_dn6 * 0.5) * assign67970_e104658) + (assign67970_e104650 * ((((-locals.var_xilp12_dn6) * locals.var_t2) + (assign67970_e104652 * locals.var_t2_dn6)) + ((locals.var_xi0p12_dn6 * locals.var_t1) + (locals.var_xi0p12 * locals.var_t1_dn6))))), (((locals.var_cnst0_dn7 * 0.5) * assign67970_e104658) + (assign67970_e104650 * ((((-locals.var_xilp12_dn7) * locals.var_t2) + (assign67970_e104652 * locals.var_t2_dn7)) + ((locals.var_xi0p12_dn7 * locals.var_t1) + (locals.var_xi0p12 * locals.var_t1_dn7))))), (((locals.var_cnst0_dn8 * 0.5) * assign67970_e104658) + (assign67970_e104650 * ((((-locals.var_xilp12_dn8) * locals.var_t2) + (assign67970_e104652 * locals.var_t2_dn8)) + ((locals.var_xi0p12_dn8 * locals.var_t1) + (locals.var_xi0p12 * locals.var_t1_dn8))))), (((locals.var_cnst0_dn9 * 0.5) * assign67970_e104658) + (assign67970_e104650 * ((((-locals.var_xilp12_dn9) * locals.var_t2) + (assign67970_e104652 * locals.var_t2_dn9)) + ((locals.var_xi0p12_dn9 * locals.var_t1) + (locals.var_xi0p12 * locals.var_t1_dn9))))), (((locals.var_cnst0_dn10 * 0.5) * assign67970_e104658) + (assign67970_e104650 * ((((-locals.var_xilp12_dn10) * locals.var_t2) + (assign67970_e104652 * locals.var_t2_dn10)) + ((locals.var_xi0p12_dn10 * locals.var_t1) + (locals.var_xi0p12 * locals.var_t1_dn10))))), (((locals.var_cnst0_dn11 * 0.5) * assign67970_e104658) + (assign67970_e104650 * ((((-locals.var_xilp12_dn11) * locals.var_t2) + (assign67970_e104652 * locals.var_t2_dn11)) + ((locals.var_xi0p12_dn11 * locals.var_t1) + (locals.var_xi0p12 * locals.var_t1_dn11))))), (((locals.var_cnst0_dn14 * 0.5) * assign67970_e104658) + (assign67970_e104650 * ((((-locals.var_xilp12_dn14) * locals.var_t2) + (assign67970_e104652 * locals.var_t2_dn14)) + ((locals.var_xi0p12_dn14 * locals.var_t1) + (locals.var_xi0p12 * locals.var_t1_dn14))))),)
    } else {
        (locals.var_dg4, locals.var_dg4_dn0, locals.var_dg4_dn2, locals.var_dg4_dn4, locals.var_dg4_dn5, locals.var_dg4_dn6, locals.var_dg4_dn7, locals.var_dg4_dn8, locals.var_dg4_dn9, locals.var_dg4_dn10, locals.var_dg4_dn11, locals.var_dg4_dn14,)
    }
};
        locals.var_dg4 = assign67970_e104661;
        locals.var_dg4_dn0 = assign67970_e104661_d_n0;
        locals.var_dg4_dn2 = assign67970_e104661_d_n2;
        locals.var_dg4_dn4 = assign67970_e104661_d_n4;
        locals.var_dg4_dn5 = assign67970_e104661_d_n5;
        locals.var_dg4_dn6 = assign67970_e104661_d_n6;
        locals.var_dg4_dn7 = assign67970_e104661_d_n7;
        locals.var_dg4_dn8 = assign67970_e104661_d_n8;
        locals.var_dg4_dn9 = assign67970_e104661_d_n9;
        locals.var_dg4_dn10 = assign67970_e104661_d_n10;
        locals.var_dg4_dn11 = assign67970_e104661_d_n11;
        locals.var_dg4_dn14 = assign67970_e104661_d_n14;
        locals.var_dg4_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_256(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign67980_e104667, assign67980_e104667_d_n0, assign67980_e104667_d_n2, assign67980_e104667_d_n4, assign67980_e104667_d_n5, assign67980_e104667_d_n6, assign67980_e104667_d_n7, assign67980_e104667_d_n8, assign67980_e104667_d_n9, assign67980_e104667_d_n10, assign67980_e104667_d_n11, assign67980_e104667_d_n14,) = {
    if (locals.var_guard1604 != 0.0) {
        let assign67980_e104665: f64 = (locals.var_dg3 + locals.var_dg4);
        (assign67980_e104665, (locals.var_dg3_dn0 + locals.var_dg4_dn0), (locals.var_dg3_dn2 + locals.var_dg4_dn2), (locals.var_dg3_dn4 + locals.var_dg4_dn4), (locals.var_dg3_dn5 + locals.var_dg4_dn5), (locals.var_dg3_dn6 + locals.var_dg4_dn6), (locals.var_dg3_dn7 + locals.var_dg4_dn7), (locals.var_dg3_dn8 + locals.var_dg4_dn8), (locals.var_dg3_dn9 + locals.var_dg4_dn9), (locals.var_dg3_dn10 + locals.var_dg4_dn10), (locals.var_dg3_dn11 + locals.var_dg4_dn11), (locals.var_dg3_dn14 + locals.var_dg4_dn14),)
    } else {
        (locals.var_didd, locals.var_didd_dn0, locals.var_didd_dn2, locals.var_didd_dn4, locals.var_didd_dn5, locals.var_didd_dn6, locals.var_didd_dn7, locals.var_didd_dn8, locals.var_didd_dn9, locals.var_didd_dn10, locals.var_didd_dn11, locals.var_didd_dn14,)
    }
};
        locals.var_didd = assign67980_e104667;
        locals.var_didd_dn0 = assign67980_e104667_d_n0;
        locals.var_didd_dn2 = assign67980_e104667_d_n2;
        locals.var_didd_dn4 = assign67980_e104667_d_n4;
        locals.var_didd_dn5 = assign67980_e104667_d_n5;
        locals.var_didd_dn6 = assign67980_e104667_d_n6;
        locals.var_didd_dn7 = assign67980_e104667_d_n7;
        locals.var_didd_dn8 = assign67980_e104667_d_n8;
        locals.var_didd_dn9 = assign67980_e104667_d_n9;
        locals.var_didd_dn10 = assign67980_e104667_d_n10;
        locals.var_didd_dn11 = assign67980_e104667_d_n11;
        locals.var_didd_dn14 = assign67980_e104667_d_n14;
        locals.var_didd_rv = 0.0;

        let (assign67990_e104675, assign67990_e104675_d_n0, assign67990_e104675_d_n2, assign67990_e104675_d_n4, assign67990_e104675_d_n5, assign67990_e104675_d_n6, assign67990_e104675_d_n7, assign67990_e104675_d_n8, assign67990_e104675_d_n9, assign67990_e104675_d_n10, assign67990_e104675_d_n11, assign67990_e104675_d_n14,) = {
    if (locals.var_guard1604 != 0.0) {
        let assign67990_e104671: f64 = (locals.var_betawl * locals.var_didd);
        let assign67990_e104673: f64 = (assign67990_e104671 * locals.var_mu);
        (assign67990_e104673, ((((locals.var_betawl_dn0 * locals.var_didd) + (locals.var_betawl * locals.var_didd_dn0)) * locals.var_mu) + (assign67990_e104671 * locals.var_mu_dn0)), ((((locals.var_betawl_dn2 * locals.var_didd) + (locals.var_betawl * locals.var_didd_dn2)) * locals.var_mu) + (assign67990_e104671 * locals.var_mu_dn2)), ((((locals.var_betawl_dn4 * locals.var_didd) + (locals.var_betawl * locals.var_didd_dn4)) * locals.var_mu) + (assign67990_e104671 * locals.var_mu_dn4)), ((((locals.var_betawl_dn5 * locals.var_didd) + (locals.var_betawl * locals.var_didd_dn5)) * locals.var_mu) + (assign67990_e104671 * locals.var_mu_dn5)), ((((locals.var_betawl_dn6 * locals.var_didd) + (locals.var_betawl * locals.var_didd_dn6)) * locals.var_mu) + (assign67990_e104671 * locals.var_mu_dn6)), ((((locals.var_betawl_dn7 * locals.var_didd) + (locals.var_betawl * locals.var_didd_dn7)) * locals.var_mu) + (assign67990_e104671 * locals.var_mu_dn7)), ((((locals.var_betawl_dn8 * locals.var_didd) + (locals.var_betawl * locals.var_didd_dn8)) * locals.var_mu) + (assign67990_e104671 * locals.var_mu_dn8)), ((((locals.var_betawl_dn9 * locals.var_didd) + (locals.var_betawl * locals.var_didd_dn9)) * locals.var_mu) + (assign67990_e104671 * locals.var_mu_dn9)), ((((locals.var_betawl_dn10 * locals.var_didd) + (locals.var_betawl * locals.var_didd_dn10)) * locals.var_mu) + (assign67990_e104671 * locals.var_mu_dn10)), ((((locals.var_betawl_dn11 * locals.var_didd) + (locals.var_betawl * locals.var_didd_dn11)) * locals.var_mu) + (assign67990_e104671 * locals.var_mu_dn11)), ((((locals.var_betawl_dn14 * locals.var_didd) + (locals.var_betawl * locals.var_didd_dn14)) * locals.var_mu) + (assign67990_e104671 * locals.var_mu_dn14)),)
    } else {
        (locals.var_idsibpc, locals.var_idsibpc_dn0, locals.var_idsibpc_dn2, locals.var_idsibpc_dn4, locals.var_idsibpc_dn5, locals.var_idsibpc_dn6, locals.var_idsibpc_dn7, locals.var_idsibpc_dn8, locals.var_idsibpc_dn9, locals.var_idsibpc_dn10, locals.var_idsibpc_dn11, locals.var_idsibpc_dn14,)
    }
};
        locals.var_idsibpc = assign67990_e104675;
        locals.var_idsibpc_dn0 = assign67990_e104675_d_n0;
        locals.var_idsibpc_dn2 = assign67990_e104675_d_n2;
        locals.var_idsibpc_dn4 = assign67990_e104675_d_n4;
        locals.var_idsibpc_dn5 = assign67990_e104675_d_n5;
        locals.var_idsibpc_dn6 = assign67990_e104675_d_n6;
        locals.var_idsibpc_dn7 = assign67990_e104675_d_n7;
        locals.var_idsibpc_dn8 = assign67990_e104675_d_n8;
        locals.var_idsibpc_dn9 = assign67990_e104675_d_n9;
        locals.var_idsibpc_dn10 = assign67990_e104675_d_n10;
        locals.var_idsibpc_dn11 = assign67990_e104675_d_n11;
        locals.var_idsibpc_dn14 = assign67990_e104675_d_n14;
        locals.var_idsibpc_rv = 0.0;

        let (assign68000_e104681, assign68000_e104681_d_n0, assign68000_e104681_d_n2, assign68000_e104681_d_n4, assign68000_e104681_d_n5, assign68000_e104681_d_n6, assign68000_e104681_d_n7, assign68000_e104681_d_n8, assign68000_e104681_d_n9, assign68000_e104681_d_n10, assign68000_e104681_d_n11, assign68000_e104681_d_n14,) = {
    if (locals.var_guard1604 != 0.0) {
        let assign68000_e104679: f64 = (locals.var_wk_ii * locals.var_idsibpc);
        (assign68000_e104679, ((locals.var_wk_ii_dn0 * locals.var_idsibpc) + (locals.var_wk_ii * locals.var_idsibpc_dn0)), ((locals.var_wk_ii_dn2 * locals.var_idsibpc) + (locals.var_wk_ii * locals.var_idsibpc_dn2)), ((locals.var_wk_ii_dn4 * locals.var_idsibpc) + (locals.var_wk_ii * locals.var_idsibpc_dn4)), ((locals.var_wk_ii_dn5 * locals.var_idsibpc) + (locals.var_wk_ii * locals.var_idsibpc_dn5)), ((locals.var_wk_ii_dn6 * locals.var_idsibpc) + (locals.var_wk_ii * locals.var_idsibpc_dn6)), ((locals.var_wk_ii_dn7 * locals.var_idsibpc) + (locals.var_wk_ii * locals.var_idsibpc_dn7)), ((locals.var_wk_ii_dn8 * locals.var_idsibpc) + (locals.var_wk_ii * locals.var_idsibpc_dn8)), ((locals.var_wk_ii_dn9 * locals.var_idsibpc) + (locals.var_wk_ii * locals.var_idsibpc_dn9)), ((locals.var_wk_ii_dn10 * locals.var_idsibpc) + (locals.var_wk_ii * locals.var_idsibpc_dn10)), ((locals.var_wk_ii_dn11 * locals.var_idsibpc) + (locals.var_wk_ii * locals.var_idsibpc_dn11)), ((locals.var_wk_ii_dn14 * locals.var_idsibpc) + (locals.var_wk_ii * locals.var_idsibpc_dn14)),)
    } else {
        (locals.var_isubibpc, locals.var_isubibpc_dn0, locals.var_isubibpc_dn2, locals.var_isubibpc_dn4, locals.var_isubibpc_dn5, locals.var_isubibpc_dn6, locals.var_isubibpc_dn7, locals.var_isubibpc_dn8, locals.var_isubibpc_dn9, locals.var_isubibpc_dn10, locals.var_isubibpc_dn11, locals.var_isubibpc_dn14,)
    }
};
        locals.var_isubibpc = assign68000_e104681;
        locals.var_isubibpc_dn0 = assign68000_e104681_d_n0;
        locals.var_isubibpc_dn2 = assign68000_e104681_d_n2;
        locals.var_isubibpc_dn4 = assign68000_e104681_d_n4;
        locals.var_isubibpc_dn5 = assign68000_e104681_d_n5;
        locals.var_isubibpc_dn6 = assign68000_e104681_d_n6;
        locals.var_isubibpc_dn7 = assign68000_e104681_d_n7;
        locals.var_isubibpc_dn8 = assign68000_e104681_d_n8;
        locals.var_isubibpc_dn9 = assign68000_e104681_d_n9;
        locals.var_isubibpc_dn10 = assign68000_e104681_d_n10;
        locals.var_isubibpc_dn11 = assign68000_e104681_d_n11;
        locals.var_isubibpc_dn14 = assign68000_e104681_d_n14;
        locals.var_isubibpc_rv = 0.0;

        let assign68010_e104684: f64 = if p.p24 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1605 = assign68010_e104684;
        locals.var_guard1605_rv = 0.0;

        let assign68020_e104687: f64 = if locals.var_flg_noqi == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1606 = assign68020_e104687;
        locals.var_guard1606_rv = 0.0;

        let (assign68030_e104699, assign68030_e104699_d_n0, assign68030_e104699_d_n2, assign68030_e104699_d_n4, assign68030_e104699_d_n5, assign68030_e104699_d_n6, assign68030_e104699_d_n7, assign68030_e104699_d_n8, assign68030_e104699_d_n9, assign68030_e104699_d_n10, assign68030_e104699_d_n11, assign68030_e104699_d_n14,) = {
    if ((locals.var_guard1605 != 0.0) && (locals.var_guard1606 != 0.0)) {
        let assign68030_e104693: f64 = (locals.var_ps0z + locals.var_vdsz__blk441);
        let assign68030_e104696: f64 = (10.0 * 2.220446049250313e-16);
        let assign68030_e104697: f64 = (assign68030_e104693 - assign68030_e104696);
        (assign68030_e104697, (locals.var_ps0z_dn0 + locals.var_vdsz__blk441_dn0), (locals.var_ps0z_dn2 + locals.var_vdsz__blk441_dn2), (locals.var_ps0z_dn4 + locals.var_vdsz__blk441_dn4), (locals.var_ps0z_dn5 + locals.var_vdsz__blk441_dn5), (locals.var_ps0z_dn6 + locals.var_vdsz__blk441_dn6), (locals.var_ps0z_dn7 + locals.var_vdsz__blk441_dn7), (locals.var_ps0z_dn8 + locals.var_vdsz__blk441_dn8), (locals.var_ps0z_dn9 + locals.var_vdsz__blk441_dn9), (locals.var_ps0z_dn10 + locals.var_vdsz__blk441_dn10), (locals.var_ps0z_dn11 + locals.var_vdsz__blk441_dn11), (locals.var_ps0z_dn14 + locals.var_vdsz__blk441_dn14),)
    } else {
        (locals.var_psdlz, locals.var_psdlz_dn0, locals.var_psdlz_dn2, locals.var_psdlz_dn4, locals.var_psdlz_dn5, locals.var_psdlz_dn6, locals.var_psdlz_dn7, locals.var_psdlz_dn8, locals.var_psdlz_dn9, locals.var_psdlz_dn10, locals.var_psdlz_dn11, locals.var_psdlz_dn14,)
    }
};
        locals.var_psdlz = assign68030_e104699;
        locals.var_psdlz_dn0 = assign68030_e104699_d_n0;
        locals.var_psdlz_dn2 = assign68030_e104699_d_n2;
        locals.var_psdlz_dn4 = assign68030_e104699_d_n4;
        locals.var_psdlz_dn5 = assign68030_e104699_d_n5;
        locals.var_psdlz_dn6 = assign68030_e104699_d_n6;
        locals.var_psdlz_dn7 = assign68030_e104699_d_n7;
        locals.var_psdlz_dn8 = assign68030_e104699_d_n8;
        locals.var_psdlz_dn9 = assign68030_e104699_d_n9;
        locals.var_psdlz_dn10 = assign68030_e104699_d_n10;
        locals.var_psdlz_dn11 = assign68030_e104699_d_n11;
        locals.var_psdlz_dn14 = assign68030_e104699_d_n14;
        locals.var_psdlz_rv = 0.0;

        let (assign68040_e104719, assign68040_e104719_d_n0, assign68040_e104719_d_n2, assign68040_e104719_d_n4, assign68040_e104719_d_n5, assign68040_e104719_d_n6, assign68040_e104719_d_n7, assign68040_e104719_d_n8, assign68040_e104719_d_n9, assign68040_e104719_d_n10, assign68040_e104719_d_n11, assign68040_e104719_d_n14,) = {
    if ((locals.var_guard1605 != 0.0) && (locals.var_guard1606 != 0.0)) {
        let assign68040_e104705: f64 = (locals.var_vgsz__blk442 - locals.var_vfb);
        let assign68040_e104709: f64 = (locals.var_dvth - locals.var_dppg);
        let assign68040_e104710: f64 = (locals.var_mks_gleak4 * assign68040_e104709);
        let assign68040_e104712: f64 = (assign68040_e104710 * locals.var_leff);
        let assign68040_e104713: f64 = (assign68040_e104705 + assign68040_e104712);
        let assign68040_e104716: f64 = (locals.var_psdlz * locals.var_uc_gleak3);
        let assign68040_e104717: f64 = (assign68040_e104713 - assign68040_e104716);
        (assign68040_e104717, ((locals.var_vgsz__blk442_dn0 + ((locals.var_mks_gleak4 * (locals.var_dvth_dn0 - locals.var_dppg_dn0)) * locals.var_leff)) - (locals.var_psdlz_dn0 * locals.var_uc_gleak3)), ((locals.var_vgsz__blk442_dn2 + ((locals.var_mks_gleak4 * (locals.var_dvth_dn2 - locals.var_dppg_dn2)) * locals.var_leff)) - (locals.var_psdlz_dn2 * locals.var_uc_gleak3)), ((locals.var_vgsz__blk442_dn4 + ((locals.var_mks_gleak4 * (locals.var_dvth_dn4 - locals.var_dppg_dn4)) * locals.var_leff)) - (locals.var_psdlz_dn4 * locals.var_uc_gleak3)), ((locals.var_vgsz__blk442_dn5 + ((locals.var_mks_gleak4 * (locals.var_dvth_dn5 - locals.var_dppg_dn5)) * locals.var_leff)) - (locals.var_psdlz_dn5 * locals.var_uc_gleak3)), ((locals.var_vgsz__blk442_dn6 + ((locals.var_mks_gleak4 * (locals.var_dvth_dn6 - locals.var_dppg_dn6)) * locals.var_leff)) - (locals.var_psdlz_dn6 * locals.var_uc_gleak3)), ((locals.var_vgsz__blk442_dn7 + ((locals.var_mks_gleak4 * (locals.var_dvth_dn7 - locals.var_dppg_dn7)) * locals.var_leff)) - (locals.var_psdlz_dn7 * locals.var_uc_gleak3)), ((locals.var_vgsz__blk442_dn8 + ((locals.var_mks_gleak4 * (locals.var_dvth_dn8 - locals.var_dppg_dn8)) * locals.var_leff)) - (locals.var_psdlz_dn8 * locals.var_uc_gleak3)), ((locals.var_vgsz__blk442_dn9 + ((locals.var_mks_gleak4 * (locals.var_dvth_dn9 - locals.var_dppg_dn9)) * locals.var_leff)) - (locals.var_psdlz_dn9 * locals.var_uc_gleak3)), ((locals.var_vgsz__blk442_dn10 + ((locals.var_mks_gleak4 * (locals.var_dvth_dn10 - locals.var_dppg_dn10)) * locals.var_leff)) - (locals.var_psdlz_dn10 * locals.var_uc_gleak3)), ((locals.var_vgsz__blk442_dn11 + ((locals.var_mks_gleak4 * (locals.var_dvth_dn11 - locals.var_dppg_dn11)) * locals.var_leff)) - (locals.var_psdlz_dn11 * locals.var_uc_gleak3)), ((locals.var_vgsz__blk442_dn14 + ((locals.var_mks_gleak4 * (locals.var_dvth_dn14 - locals.var_dppg_dn14)) * locals.var_leff)) - (locals.var_psdlz_dn14 * locals.var_uc_gleak3)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign68040_e104719;
        locals.var_t1_dn0 = assign68040_e104719_d_n0;
        locals.var_t1_dn2 = assign68040_e104719_d_n2;
        locals.var_t1_dn4 = assign68040_e104719_d_n4;
        locals.var_t1_dn5 = assign68040_e104719_d_n5;
        locals.var_t1_dn6 = assign68040_e104719_d_n6;
        locals.var_t1_dn7 = assign68040_e104719_d_n7;
        locals.var_t1_dn8 = assign68040_e104719_d_n8;
        locals.var_t1_dn9 = assign68040_e104719_d_n9;
        locals.var_t1_dn10 = assign68040_e104719_d_n10;
        locals.var_t1_dn11 = assign68040_e104719_d_n11;
        locals.var_t1_dn14 = assign68040_e104719_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign68050_e104727, assign68050_e104727_d_n0, assign68050_e104727_d_n2, assign68050_e104727_d_n4, assign68050_e104727_d_n5, assign68050_e104727_d_n6, assign68050_e104727_d_n7, assign68050_e104727_d_n8, assign68050_e104727_d_n9, assign68050_e104727_d_n10, assign68050_e104727_d_n11, assign68050_e104727_d_n14,) = {
    if ((locals.var_guard1605 != 0.0) && (locals.var_guard1606 != 0.0)) {
        let assign68050_e104725: f64 = (locals.var_t1 * locals.var_t1);
        (assign68050_e104725, ((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)), ((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)), ((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)), ((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)), ((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)), ((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)), ((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)), ((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)), ((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)), ((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)), ((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign68050_e104727;
        locals.var_t1_dn0 = assign68050_e104727_d_n0;
        locals.var_t1_dn2 = assign68050_e104727_d_n2;
        locals.var_t1_dn4 = assign68050_e104727_d_n4;
        locals.var_t1_dn5 = assign68050_e104727_d_n5;
        locals.var_t1_dn6 = assign68050_e104727_d_n6;
        locals.var_t1_dn7 = assign68050_e104727_d_n7;
        locals.var_t1_dn8 = assign68050_e104727_d_n8;
        locals.var_t1_dn9 = assign68050_e104727_d_n9;
        locals.var_t1_dn10 = assign68050_e104727_d_n10;
        locals.var_t1_dn11 = assign68050_e104727_d_n11;
        locals.var_t1_dn14 = assign68050_e104727_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign68060_e104735, assign68060_e104735_d_n0, assign68060_e104735_d_n2, assign68060_e104735_d_n4, assign68060_e104735_d_n5, assign68060_e104735_d_n6, assign68060_e104735_d_n7, assign68060_e104735_d_n8, assign68060_e104735_d_n9, assign68060_e104735_d_n10, assign68060_e104735_d_n11, assign68060_e104735_d_n14,) = {
    if ((locals.var_guard1605 != 0.0) && (locals.var_guard1606 != 0.0)) {
        let assign68060_e104733: f64 = (1.0 / locals.var_tox0);
        (assign68060_e104733, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign68060_e104735;
        locals.var_t3_dn0 = assign68060_e104735_d_n0;
        locals.var_t3_dn2 = assign68060_e104735_d_n2;
        locals.var_t3_dn4 = assign68060_e104735_d_n4;
        locals.var_t3_dn5 = assign68060_e104735_d_n5;
        locals.var_t3_dn6 = assign68060_e104735_d_n6;
        locals.var_t3_dn7 = assign68060_e104735_d_n7;
        locals.var_t3_dn8 = assign68060_e104735_d_n8;
        locals.var_t3_dn9 = assign68060_e104735_d_n9;
        locals.var_t3_dn10 = assign68060_e104735_d_n10;
        locals.var_t3_dn11 = assign68060_e104735_d_n11;
        locals.var_t3_dn14 = assign68060_e104735_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign68070_e104743, assign68070_e104743_d_n0, assign68070_e104743_d_n2, assign68070_e104743_d_n4, assign68070_e104743_d_n5, assign68070_e104743_d_n6, assign68070_e104743_d_n7, assign68070_e104743_d_n8, assign68070_e104743_d_n9, assign68070_e104743_d_n10, assign68070_e104743_d_n11, assign68070_e104743_d_n14,) = {
    if ((locals.var_guard1605 != 0.0) && (locals.var_guard1606 != 0.0)) {
        let assign68070_e104741: f64 = (locals.var_t1 * locals.var_t3);
        (assign68070_e104741, ((locals.var_t1_dn0 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn0)), ((locals.var_t1_dn2 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn2)), ((locals.var_t1_dn4 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn4)), ((locals.var_t1_dn5 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn5)), ((locals.var_t1_dn6 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn6)), ((locals.var_t1_dn7 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn7)), ((locals.var_t1_dn8 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn8)), ((locals.var_t1_dn9 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn9)), ((locals.var_t1_dn10 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn10)), ((locals.var_t1_dn11 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn11)), ((locals.var_t1_dn14 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign68070_e104743;
        locals.var_t2_dn0 = assign68070_e104743_d_n0;
        locals.var_t2_dn2 = assign68070_e104743_d_n2;
        locals.var_t2_dn4 = assign68070_e104743_d_n4;
        locals.var_t2_dn5 = assign68070_e104743_d_n5;
        locals.var_t2_dn6 = assign68070_e104743_d_n6;
        locals.var_t2_dn7 = assign68070_e104743_d_n7;
        locals.var_t2_dn8 = assign68070_e104743_d_n8;
        locals.var_t2_dn9 = assign68070_e104743_d_n9;
        locals.var_t2_dn10 = assign68070_e104743_d_n10;
        locals.var_t2_dn11 = assign68070_e104743_d_n11;
        locals.var_t2_dn14 = assign68070_e104743_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign68080_e104751, assign68080_e104751_d_n0, assign68080_e104751_d_n2, assign68080_e104751_d_n4, assign68080_e104751_d_n5, assign68080_e104751_d_n6, assign68080_e104751_d_n7, assign68080_e104751_d_n8, assign68080_e104751_d_n9, assign68080_e104751_d_n10, assign68080_e104751_d_n11, assign68080_e104751_d_n14,) = {
    if ((locals.var_guard1605 != 0.0) && (locals.var_guard1606 != 0.0)) {
        let assign68080_e104749: f64 = (1.0 / locals.var_mks_gleak5);
        (assign68080_e104749, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign68080_e104751;
        locals.var_t3_dn0 = assign68080_e104751_d_n0;
        locals.var_t3_dn2 = assign68080_e104751_d_n2;
        locals.var_t3_dn4 = assign68080_e104751_d_n4;
        locals.var_t3_dn5 = assign68080_e104751_d_n5;
        locals.var_t3_dn6 = assign68080_e104751_d_n6;
        locals.var_t3_dn7 = assign68080_e104751_d_n7;
        locals.var_t3_dn8 = assign68080_e104751_d_n8;
        locals.var_t3_dn9 = assign68080_e104751_d_n9;
        locals.var_t3_dn10 = assign68080_e104751_d_n10;
        locals.var_t3_dn11 = assign68080_e104751_d_n11;
        locals.var_t3_dn14 = assign68080_e104751_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign68090_e104761, assign68090_e104761_d_n0, assign68090_e104761_d_n2, assign68090_e104761_d_n4, assign68090_e104761_d_n5, assign68090_e104761_d_n6, assign68090_e104761_d_n7, assign68090_e104761_d_n8, assign68090_e104761_d_n9, assign68090_e104761_d_n10, assign68090_e104761_d_n11, assign68090_e104761_d_n14,) = {
    if ((locals.var_guard1605 != 0.0) && (locals.var_guard1606 != 0.0)) {
        let assign68090_e104758: f64 = (locals.var_ey * locals.var_t3);
        let assign68090_e104759: f64 = (1.0 + assign68090_e104758);
        (assign68090_e104759, ((locals.var_ey_dn0 * locals.var_t3) + (locals.var_ey * locals.var_t3_dn0)), ((locals.var_ey_dn2 * locals.var_t3) + (locals.var_ey * locals.var_t3_dn2)), ((locals.var_ey_dn4 * locals.var_t3) + (locals.var_ey * locals.var_t3_dn4)), ((locals.var_ey_dn5 * locals.var_t3) + (locals.var_ey * locals.var_t3_dn5)), ((locals.var_ey_dn6 * locals.var_t3) + (locals.var_ey * locals.var_t3_dn6)), ((locals.var_ey_dn7 * locals.var_t3) + (locals.var_ey * locals.var_t3_dn7)), ((locals.var_ey_dn8 * locals.var_t3) + (locals.var_ey * locals.var_t3_dn8)), ((locals.var_ey_dn9 * locals.var_t3) + (locals.var_ey * locals.var_t3_dn9)), ((locals.var_ey_dn10 * locals.var_t3) + (locals.var_ey * locals.var_t3_dn10)), ((locals.var_ey_dn11 * locals.var_t3) + (locals.var_ey * locals.var_t3_dn11)), ((locals.var_ey_dn14 * locals.var_t3) + (locals.var_ey * locals.var_t3_dn14)),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign68090_e104761;
        locals.var_t7_dn0 = assign68090_e104761_d_n0;
        locals.var_t7_dn2 = assign68090_e104761_d_n2;
        locals.var_t7_dn4 = assign68090_e104761_d_n4;
        locals.var_t7_dn5 = assign68090_e104761_d_n5;
        locals.var_t7_dn6 = assign68090_e104761_d_n6;
        locals.var_t7_dn7 = assign68090_e104761_d_n7;
        locals.var_t7_dn8 = assign68090_e104761_d_n8;
        locals.var_t7_dn9 = assign68090_e104761_d_n9;
        locals.var_t7_dn10 = assign68090_e104761_d_n10;
        locals.var_t7_dn11 = assign68090_e104761_d_n11;
        locals.var_t7_dn14 = assign68090_e104761_d_n14;
        locals.var_t7_rv = 0.0;

        let (assign68100_e104769, assign68100_e104769_d_n0, assign68100_e104769_d_n2, assign68100_e104769_d_n4, assign68100_e104769_d_n5, assign68100_e104769_d_n6, assign68100_e104769_d_n7, assign68100_e104769_d_n8, assign68100_e104769_d_n9, assign68100_e104769_d_n10, assign68100_e104769_d_n11, assign68100_e104769_d_n14,) = {
    if ((locals.var_guard1605 != 0.0) && (locals.var_guard1606 != 0.0)) {
        let assign68100_e104767: f64 = (locals.var_t2 * locals.var_t7);
        (assign68100_e104767, ((locals.var_t2_dn0 * locals.var_t7) + (locals.var_t2 * locals.var_t7_dn0)), ((locals.var_t2_dn2 * locals.var_t7) + (locals.var_t2 * locals.var_t7_dn2)), ((locals.var_t2_dn4 * locals.var_t7) + (locals.var_t2 * locals.var_t7_dn4)), ((locals.var_t2_dn5 * locals.var_t7) + (locals.var_t2 * locals.var_t7_dn5)), ((locals.var_t2_dn6 * locals.var_t7) + (locals.var_t2 * locals.var_t7_dn6)), ((locals.var_t2_dn7 * locals.var_t7) + (locals.var_t2 * locals.var_t7_dn7)), ((locals.var_t2_dn8 * locals.var_t7) + (locals.var_t2 * locals.var_t7_dn8)), ((locals.var_t2_dn9 * locals.var_t7) + (locals.var_t2 * locals.var_t7_dn9)), ((locals.var_t2_dn10 * locals.var_t7) + (locals.var_t2 * locals.var_t7_dn10)), ((locals.var_t2_dn11 * locals.var_t7) + (locals.var_t2 * locals.var_t7_dn11)), ((locals.var_t2_dn14 * locals.var_t7) + (locals.var_t2 * locals.var_t7_dn14)),)
    } else {
        (locals.var_etun, locals.var_etun_dn0, locals.var_etun_dn2, locals.var_etun_dn4, locals.var_etun_dn5, locals.var_etun_dn6, locals.var_etun_dn7, locals.var_etun_dn8, locals.var_etun_dn9, locals.var_etun_dn10, locals.var_etun_dn11, locals.var_etun_dn14,)
    }
};
        locals.var_etun = assign68100_e104769;
        locals.var_etun_dn0 = assign68100_e104769_d_n0;
        locals.var_etun_dn2 = assign68100_e104769_d_n2;
        locals.var_etun_dn4 = assign68100_e104769_d_n4;
        locals.var_etun_dn5 = assign68100_e104769_d_n5;
        locals.var_etun_dn6 = assign68100_e104769_d_n6;
        locals.var_etun_dn7 = assign68100_e104769_d_n7;
        locals.var_etun_dn8 = assign68100_e104769_d_n8;
        locals.var_etun_dn9 = assign68100_e104769_d_n9;
        locals.var_etun_dn10 = assign68100_e104769_d_n10;
        locals.var_etun_dn11 = assign68100_e104769_d_n11;
        locals.var_etun_dn14 = assign68100_e104769_d_n14;
        locals.var_etun_rv = 0.0;

        let (assign68110_e104788, assign68110_e104788_d_n0, assign68110_e104788_d_n2, assign68110_e104788_d_n4, assign68110_e104788_d_n5, assign68110_e104788_d_n6, assign68110_e104788_d_n7, assign68110_e104788_d_n8, assign68110_e104788_d_n9, assign68110_e104788_d_n10, assign68110_e104788_d_n11, assign68110_e104788_d_n14,) = {
    if ((locals.var_guard1605 != 0.0) && (locals.var_guard1606 != 0.0)) {
        let assign68110_e104775: f64 = (locals.var_etun * locals.var_etun);
        let assign68110_e104779: f64 = (0.01 / 0.01);
        let assign68110_e104780: f64 = (4.0 * assign68110_e104779);
        let assign68110_e104783: f64 = (0.01 / 0.01);
        let assign68110_e104784: f64 = (assign68110_e104780 * assign68110_e104783);
        let assign68110_e104785: f64 = (assign68110_e104775 + assign68110_e104784);
        let assign68110_e104786: f64 = (assign68110_e104785).sqrt();
        (assign68110_e104786, (((locals.var_etun_dn0 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn0)) / (2.0 * assign68110_e104786)), (((locals.var_etun_dn2 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn2)) / (2.0 * assign68110_e104786)), (((locals.var_etun_dn4 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn4)) / (2.0 * assign68110_e104786)), (((locals.var_etun_dn5 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn5)) / (2.0 * assign68110_e104786)), (((locals.var_etun_dn6 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn6)) / (2.0 * assign68110_e104786)), (((locals.var_etun_dn7 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn7)) / (2.0 * assign68110_e104786)), (((locals.var_etun_dn8 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn8)) / (2.0 * assign68110_e104786)), (((locals.var_etun_dn9 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn9)) / (2.0 * assign68110_e104786)), (((locals.var_etun_dn10 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn10)) / (2.0 * assign68110_e104786)), (((locals.var_etun_dn11 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn11)) / (2.0 * assign68110_e104786)), (((locals.var_etun_dn14 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn14)) / (2.0 * assign68110_e104786)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign68110_e104788;
        locals.var_tmf2_dn0 = assign68110_e104788_d_n0;
        locals.var_tmf2_dn2 = assign68110_e104788_d_n2;
        locals.var_tmf2_dn4 = assign68110_e104788_d_n4;
        locals.var_tmf2_dn5 = assign68110_e104788_d_n5;
        locals.var_tmf2_dn6 = assign68110_e104788_d_n6;
        locals.var_tmf2_dn7 = assign68110_e104788_d_n7;
        locals.var_tmf2_dn8 = assign68110_e104788_d_n8;
        locals.var_tmf2_dn9 = assign68110_e104788_d_n9;
        locals.var_tmf2_dn10 = assign68110_e104788_d_n10;
        locals.var_tmf2_dn11 = assign68110_e104788_d_n11;
        locals.var_tmf2_dn14 = assign68110_e104788_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign68120_e104800, assign68120_e104800_d_n0, assign68120_e104800_d_n2, assign68120_e104800_d_n4, assign68120_e104800_d_n5, assign68120_e104800_d_n6, assign68120_e104800_d_n7, assign68120_e104800_d_n8, assign68120_e104800_d_n9, assign68120_e104800_d_n10, assign68120_e104800_d_n11, assign68120_e104800_d_n14,) = {
    if ((locals.var_guard1605 != 0.0) && (locals.var_guard1606 != 0.0)) {
        let assign68120_e104796: f64 = (locals.var_etun / locals.var_tmf2);
        let assign68120_e104797: f64 = (1.0 + assign68120_e104796);
        let assign68120_e104798: f64 = (0.5 * assign68120_e104797);
        (assign68120_e104798, (0.5 * (((locals.var_etun_dn0 * locals.var_tmf2) - (locals.var_etun * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_etun_dn2 * locals.var_tmf2) - (locals.var_etun * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_etun_dn4 * locals.var_tmf2) - (locals.var_etun * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_etun_dn5 * locals.var_tmf2) - (locals.var_etun * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_etun_dn6 * locals.var_tmf2) - (locals.var_etun * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_etun_dn7 * locals.var_tmf2) - (locals.var_etun * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_etun_dn8 * locals.var_tmf2) - (locals.var_etun * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_etun_dn9 * locals.var_tmf2) - (locals.var_etun * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_etun_dn10 * locals.var_tmf2) - (locals.var_etun * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_etun_dn11 * locals.var_tmf2) - (locals.var_etun * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_etun_dn14 * locals.var_tmf2) - (locals.var_etun * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign68120_e104800;
        locals.var_t5_dn0 = assign68120_e104800_d_n0;
        locals.var_t5_dn2 = assign68120_e104800_d_n2;
        locals.var_t5_dn4 = assign68120_e104800_d_n4;
        locals.var_t5_dn5 = assign68120_e104800_d_n5;
        locals.var_t5_dn6 = assign68120_e104800_d_n6;
        locals.var_t5_dn7 = assign68120_e104800_d_n7;
        locals.var_t5_dn8 = assign68120_e104800_d_n8;
        locals.var_t5_dn9 = assign68120_e104800_d_n9;
        locals.var_t5_dn10 = assign68120_e104800_d_n10;
        locals.var_t5_dn11 = assign68120_e104800_d_n11;
        locals.var_t5_dn14 = assign68120_e104800_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign68130_e104810, assign68130_e104810_d_n0, assign68130_e104810_d_n2, assign68130_e104810_d_n4, assign68130_e104810_d_n5, assign68130_e104810_d_n6, assign68130_e104810_d_n7, assign68130_e104810_d_n8, assign68130_e104810_d_n9, assign68130_e104810_d_n10, assign68130_e104810_d_n11, assign68130_e104810_d_n14,) = {
    if ((locals.var_guard1605 != 0.0) && (locals.var_guard1606 != 0.0)) {
        let assign68130_e104807: f64 = (locals.var_etun + locals.var_tmf2);
        let assign68130_e104808: f64 = (0.5 * assign68130_e104807);
        (assign68130_e104808, (0.5 * (locals.var_etun_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_etun_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_etun_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_etun_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_etun_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_etun_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_etun_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_etun_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_etun_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_etun_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_etun_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_etun, locals.var_etun_dn0, locals.var_etun_dn2, locals.var_etun_dn4, locals.var_etun_dn5, locals.var_etun_dn6, locals.var_etun_dn7, locals.var_etun_dn8, locals.var_etun_dn9, locals.var_etun_dn10, locals.var_etun_dn11, locals.var_etun_dn14,)
    }
};
        locals.var_etun = assign68130_e104810;
        locals.var_etun_dn0 = assign68130_e104810_d_n0;
        locals.var_etun_dn2 = assign68130_e104810_d_n2;
        locals.var_etun_dn4 = assign68130_e104810_d_n4;
        locals.var_etun_dn5 = assign68130_e104810_d_n5;
        locals.var_etun_dn6 = assign68130_e104810_d_n6;
        locals.var_etun_dn7 = assign68130_e104810_d_n7;
        locals.var_etun_dn8 = assign68130_e104810_d_n8;
        locals.var_etun_dn9 = assign68130_e104810_d_n9;
        locals.var_etun_dn10 = assign68130_e104810_d_n10;
        locals.var_etun_dn11 = assign68130_e104810_d_n11;
        locals.var_etun_dn14 = assign68130_e104810_d_n14;
        locals.var_etun_rv = 0.0;

        let assign68140_e104813: f64 = if locals.var_etun < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1607 = assign68140_e104813;
        locals.var_guard1607_rv = 0.0;

        let (assign68150_e104821, assign68150_e104821_d_n0, assign68150_e104821_d_n2, assign68150_e104821_d_n4, assign68150_e104821_d_n5, assign68150_e104821_d_n6, assign68150_e104821_d_n7, assign68150_e104821_d_n8, assign68150_e104821_d_n9, assign68150_e104821_d_n10, assign68150_e104821_d_n11, assign68150_e104821_d_n14,) = {
    if (((locals.var_guard1605 != 0.0) && (locals.var_guard1606 != 0.0)) && (locals.var_guard1607 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_etun, locals.var_etun_dn0, locals.var_etun_dn2, locals.var_etun_dn4, locals.var_etun_dn5, locals.var_etun_dn6, locals.var_etun_dn7, locals.var_etun_dn8, locals.var_etun_dn9, locals.var_etun_dn10, locals.var_etun_dn11, locals.var_etun_dn14,)
    }
};
        locals.var_etun = assign68150_e104821;
        locals.var_etun_dn0 = assign68150_e104821_d_n0;
        locals.var_etun_dn2 = assign68150_e104821_d_n2;
        locals.var_etun_dn4 = assign68150_e104821_d_n4;
        locals.var_etun_dn5 = assign68150_e104821_d_n5;
        locals.var_etun_dn6 = assign68150_e104821_d_n6;
        locals.var_etun_dn7 = assign68150_e104821_d_n7;
        locals.var_etun_dn8 = assign68150_e104821_d_n8;
        locals.var_etun_dn9 = assign68150_e104821_d_n9;
        locals.var_etun_dn10 = assign68150_e104821_d_n10;
        locals.var_etun_dn11 = assign68150_e104821_d_n11;
        locals.var_etun_dn14 = assign68150_e104821_d_n14;
        locals.var_etun_rv = 0.0;

        let (assign68160_e104829, assign68160_e104829_d_n0, assign68160_e104829_d_n2, assign68160_e104829_d_n4, assign68160_e104829_d_n5, assign68160_e104829_d_n6, assign68160_e104829_d_n7, assign68160_e104829_d_n8, assign68160_e104829_d_n9, assign68160_e104829_d_n10, assign68160_e104829_d_n11, assign68160_e104829_d_n14,) = {
    if (((locals.var_guard1605 != 0.0) && (locals.var_guard1606 != 0.0)) && (locals.var_guard1607 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign68160_e104829;
        locals.var_t5_dn0 = assign68160_e104829_d_n0;
        locals.var_t5_dn2 = assign68160_e104829_d_n2;
        locals.var_t5_dn4 = assign68160_e104829_d_n4;
        locals.var_t5_dn5 = assign68160_e104829_d_n5;
        locals.var_t5_dn6 = assign68160_e104829_d_n6;
        locals.var_t5_dn7 = assign68160_e104829_d_n7;
        locals.var_t5_dn8 = assign68160_e104829_d_n8;
        locals.var_t5_dn9 = assign68160_e104829_d_n9;
        locals.var_t5_dn10 = assign68160_e104829_d_n10;
        locals.var_t5_dn11 = assign68160_e104829_d_n11;
        locals.var_t5_dn14 = assign68160_e104829_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign68170_e104844, assign68170_e104844_d_n0, assign68170_e104844_d_n2, assign68170_e104844_d_n4, assign68170_e104844_d_n5, assign68170_e104844_d_n6, assign68170_e104844_d_n7, assign68170_e104844_d_n8, assign68170_e104844_d_n9, assign68170_e104844_d_n10, assign68170_e104844_d_n11, assign68170_e104844_d_n14,) = {
    if ((locals.var_guard1605 != 0.0) && (locals.var_guard1606 != 0.0)) {
        let assign68170_e104835: f64 = (locals.var_vgsz__blk442 * locals.var_vgsz__blk442);
        let assign68170_e104838: f64 = (4.0 * 0.001);
        let assign68170_e104840: f64 = (assign68170_e104838 * 0.001);
        let assign68170_e104841: f64 = (assign68170_e104835 + assign68170_e104840);
        let assign68170_e104842: f64 = (assign68170_e104841).sqrt();
        (assign68170_e104842, (((locals.var_vgsz__blk442_dn0 * locals.var_vgsz__blk442) + (locals.var_vgsz__blk442 * locals.var_vgsz__blk442_dn0)) / (2.0 * assign68170_e104842)), (((locals.var_vgsz__blk442_dn2 * locals.var_vgsz__blk442) + (locals.var_vgsz__blk442 * locals.var_vgsz__blk442_dn2)) / (2.0 * assign68170_e104842)), (((locals.var_vgsz__blk442_dn4 * locals.var_vgsz__blk442) + (locals.var_vgsz__blk442 * locals.var_vgsz__blk442_dn4)) / (2.0 * assign68170_e104842)), (((locals.var_vgsz__blk442_dn5 * locals.var_vgsz__blk442) + (locals.var_vgsz__blk442 * locals.var_vgsz__blk442_dn5)) / (2.0 * assign68170_e104842)), (((locals.var_vgsz__blk442_dn6 * locals.var_vgsz__blk442) + (locals.var_vgsz__blk442 * locals.var_vgsz__blk442_dn6)) / (2.0 * assign68170_e104842)), (((locals.var_vgsz__blk442_dn7 * locals.var_vgsz__blk442) + (locals.var_vgsz__blk442 * locals.var_vgsz__blk442_dn7)) / (2.0 * assign68170_e104842)), (((locals.var_vgsz__blk442_dn8 * locals.var_vgsz__blk442) + (locals.var_vgsz__blk442 * locals.var_vgsz__blk442_dn8)) / (2.0 * assign68170_e104842)), (((locals.var_vgsz__blk442_dn9 * locals.var_vgsz__blk442) + (locals.var_vgsz__blk442 * locals.var_vgsz__blk442_dn9)) / (2.0 * assign68170_e104842)), (((locals.var_vgsz__blk442_dn10 * locals.var_vgsz__blk442) + (locals.var_vgsz__blk442 * locals.var_vgsz__blk442_dn10)) / (2.0 * assign68170_e104842)), (((locals.var_vgsz__blk442_dn11 * locals.var_vgsz__blk442) + (locals.var_vgsz__blk442 * locals.var_vgsz__blk442_dn11)) / (2.0 * assign68170_e104842)), (((locals.var_vgsz__blk442_dn14 * locals.var_vgsz__blk442) + (locals.var_vgsz__blk442 * locals.var_vgsz__blk442_dn14)) / (2.0 * assign68170_e104842)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign68170_e104844;
        locals.var_tmf2_dn0 = assign68170_e104844_d_n0;
        locals.var_tmf2_dn2 = assign68170_e104844_d_n2;
        locals.var_tmf2_dn4 = assign68170_e104844_d_n4;
        locals.var_tmf2_dn5 = assign68170_e104844_d_n5;
        locals.var_tmf2_dn6 = assign68170_e104844_d_n6;
        locals.var_tmf2_dn7 = assign68170_e104844_d_n7;
        locals.var_tmf2_dn8 = assign68170_e104844_d_n8;
        locals.var_tmf2_dn9 = assign68170_e104844_d_n9;
        locals.var_tmf2_dn10 = assign68170_e104844_d_n10;
        locals.var_tmf2_dn11 = assign68170_e104844_d_n11;
        locals.var_tmf2_dn14 = assign68170_e104844_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign68180_e104856, assign68180_e104856_d_n0, assign68180_e104856_d_n2, assign68180_e104856_d_n4, assign68180_e104856_d_n5, assign68180_e104856_d_n6, assign68180_e104856_d_n7, assign68180_e104856_d_n8, assign68180_e104856_d_n9, assign68180_e104856_d_n10, assign68180_e104856_d_n11, assign68180_e104856_d_n14,) = {
    if ((locals.var_guard1605 != 0.0) && (locals.var_guard1606 != 0.0)) {
        let assign68180_e104852: f64 = (locals.var_vgsz__blk442 / locals.var_tmf2);
        let assign68180_e104853: f64 = (1.0 + assign68180_e104852);
        let assign68180_e104854: f64 = (0.5 * assign68180_e104853);
        (assign68180_e104854, (0.5 * (((locals.var_vgsz__blk442_dn0 * locals.var_tmf2) - (locals.var_vgsz__blk442 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vgsz__blk442_dn2 * locals.var_tmf2) - (locals.var_vgsz__blk442 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vgsz__blk442_dn4 * locals.var_tmf2) - (locals.var_vgsz__blk442 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vgsz__blk442_dn5 * locals.var_tmf2) - (locals.var_vgsz__blk442 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vgsz__blk442_dn6 * locals.var_tmf2) - (locals.var_vgsz__blk442 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vgsz__blk442_dn7 * locals.var_tmf2) - (locals.var_vgsz__blk442 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vgsz__blk442_dn8 * locals.var_tmf2) - (locals.var_vgsz__blk442 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vgsz__blk442_dn9 * locals.var_tmf2) - (locals.var_vgsz__blk442 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vgsz__blk442_dn10 * locals.var_tmf2) - (locals.var_vgsz__blk442 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vgsz__blk442_dn11 * locals.var_tmf2) - (locals.var_vgsz__blk442 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vgsz__blk442_dn14 * locals.var_tmf2) - (locals.var_vgsz__blk442 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign68180_e104856;
        locals.var_t4_dn0 = assign68180_e104856_d_n0;
        locals.var_t4_dn2 = assign68180_e104856_d_n2;
        locals.var_t4_dn4 = assign68180_e104856_d_n4;
        locals.var_t4_dn5 = assign68180_e104856_d_n5;
        locals.var_t4_dn6 = assign68180_e104856_d_n6;
        locals.var_t4_dn7 = assign68180_e104856_d_n7;
        locals.var_t4_dn8 = assign68180_e104856_d_n8;
        locals.var_t4_dn9 = assign68180_e104856_d_n9;
        locals.var_t4_dn10 = assign68180_e104856_d_n10;
        locals.var_t4_dn11 = assign68180_e104856_d_n11;
        locals.var_t4_dn14 = assign68180_e104856_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign68190_e104866, assign68190_e104866_d_n0, assign68190_e104866_d_n2, assign68190_e104866_d_n4, assign68190_e104866_d_n5, assign68190_e104866_d_n6, assign68190_e104866_d_n7, assign68190_e104866_d_n8, assign68190_e104866_d_n9, assign68190_e104866_d_n10, assign68190_e104866_d_n11, assign68190_e104866_d_n14,) = {
    if ((locals.var_guard1605 != 0.0) && (locals.var_guard1606 != 0.0)) {
        let assign68190_e104863: f64 = (locals.var_vgsz__blk442 + locals.var_tmf2);
        let assign68190_e104864: f64 = (0.5 * assign68190_e104863);
        (assign68190_e104864, (0.5 * (locals.var_vgsz__blk442_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_vgsz__blk442_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_vgsz__blk442_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_vgsz__blk442_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_vgsz__blk442_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_vgsz__blk442_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_vgsz__blk442_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_vgsz__blk442_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_vgsz__blk442_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_vgsz__blk442_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_vgsz__blk442_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign68190_e104866;
        locals.var_t3_dn0 = assign68190_e104866_d_n0;
        locals.var_t3_dn2 = assign68190_e104866_d_n2;
        locals.var_t3_dn4 = assign68190_e104866_d_n4;
        locals.var_t3_dn5 = assign68190_e104866_d_n5;
        locals.var_t3_dn6 = assign68190_e104866_d_n6;
        locals.var_t3_dn7 = assign68190_e104866_d_n7;
        locals.var_t3_dn8 = assign68190_e104866_d_n8;
        locals.var_t3_dn9 = assign68190_e104866_d_n9;
        locals.var_t3_dn10 = assign68190_e104866_d_n10;
        locals.var_t3_dn11 = assign68190_e104866_d_n11;
        locals.var_t3_dn14 = assign68190_e104866_d_n14;
        locals.var_t3_rv = 0.0;

        let assign68200_e104869: f64 = if locals.var_t3 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1608 = assign68200_e104869;
        locals.var_guard1608_rv = 0.0;

        let (assign68210_e104877, assign68210_e104877_d_n0, assign68210_e104877_d_n2, assign68210_e104877_d_n4, assign68210_e104877_d_n5, assign68210_e104877_d_n6, assign68210_e104877_d_n7, assign68210_e104877_d_n8, assign68210_e104877_d_n9, assign68210_e104877_d_n10, assign68210_e104877_d_n11, assign68210_e104877_d_n14,) = {
    if (((locals.var_guard1605 != 0.0) && (locals.var_guard1606 != 0.0)) && (locals.var_guard1608 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign68210_e104877;
        locals.var_t3_dn0 = assign68210_e104877_d_n0;
        locals.var_t3_dn2 = assign68210_e104877_d_n2;
        locals.var_t3_dn4 = assign68210_e104877_d_n4;
        locals.var_t3_dn5 = assign68210_e104877_d_n5;
        locals.var_t3_dn6 = assign68210_e104877_d_n6;
        locals.var_t3_dn7 = assign68210_e104877_d_n7;
        locals.var_t3_dn8 = assign68210_e104877_d_n8;
        locals.var_t3_dn9 = assign68210_e104877_d_n9;
        locals.var_t3_dn10 = assign68210_e104877_d_n10;
        locals.var_t3_dn11 = assign68210_e104877_d_n11;
        locals.var_t3_dn14 = assign68210_e104877_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign68220_e104885, assign68220_e104885_d_n0, assign68220_e104885_d_n2, assign68220_e104885_d_n4, assign68220_e104885_d_n5, assign68220_e104885_d_n6, assign68220_e104885_d_n7, assign68220_e104885_d_n8, assign68220_e104885_d_n9, assign68220_e104885_d_n10, assign68220_e104885_d_n11, assign68220_e104885_d_n14,) = {
    if (((locals.var_guard1605 != 0.0) && (locals.var_guard1606 != 0.0)) && (locals.var_guard1608 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign68220_e104885;
        locals.var_t4_dn0 = assign68220_e104885_d_n0;
        locals.var_t4_dn2 = assign68220_e104885_d_n2;
        locals.var_t4_dn4 = assign68220_e104885_d_n4;
        locals.var_t4_dn5 = assign68220_e104885_d_n5;
        locals.var_t4_dn6 = assign68220_e104885_d_n6;
        locals.var_t4_dn7 = assign68220_e104885_d_n7;
        locals.var_t4_dn8 = assign68220_e104885_d_n8;
        locals.var_t4_dn9 = assign68220_e104885_d_n9;
        locals.var_t4_dn10 = assign68220_e104885_d_n10;
        locals.var_t4_dn11 = assign68220_e104885_d_n11;
        locals.var_t4_dn14 = assign68220_e104885_d_n14;
        locals.var_t4_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_257(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign68230_e104893, assign68230_e104893_d_n0, assign68230_e104893_d_n2, assign68230_e104893_d_n4, assign68230_e104893_d_n5, assign68230_e104893_d_n6, assign68230_e104893_d_n7, assign68230_e104893_d_n8, assign68230_e104893_d_n9, assign68230_e104893_d_n10, assign68230_e104893_d_n11, assign68230_e104893_d_n14,) = {
    if ((locals.var_guard1605 != 0.0) && (locals.var_guard1606 != 0.0)) {
        let assign68230_e104891: f64 = (locals.var_t3 - p.p262);
        (assign68230_e104891, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign68230_e104893;
        locals.var_t3_dn0 = assign68230_e104893_d_n0;
        locals.var_t3_dn2 = assign68230_e104893_d_n2;
        locals.var_t3_dn4 = assign68230_e104893_d_n4;
        locals.var_t3_dn5 = assign68230_e104893_d_n5;
        locals.var_t3_dn6 = assign68230_e104893_d_n6;
        locals.var_t3_dn7 = assign68230_e104893_d_n7;
        locals.var_t3_dn8 = assign68230_e104893_d_n8;
        locals.var_t3_dn9 = assign68230_e104893_d_n9;
        locals.var_t3_dn10 = assign68230_e104893_d_n10;
        locals.var_t3_dn11 = assign68230_e104893_d_n11;
        locals.var_t3_dn14 = assign68230_e104893_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign68240_e104901, assign68240_e104901_d_n0, assign68240_e104901_d_n2, assign68240_e104901_d_n4, assign68240_e104901_d_n5, assign68240_e104901_d_n6, assign68240_e104901_d_n7, assign68240_e104901_d_n8, assign68240_e104901_d_n9, assign68240_e104901_d_n10, assign68240_e104901_d_n11, assign68240_e104901_d_n14,) = {
    if ((locals.var_guard1605 != 0.0) && (locals.var_guard1606 != 0.0)) {
        let assign68240_e104899: f64 = (locals.var_t3 / 0.1);
        (assign68240_e104899, (locals.var_t3_dn0 / 0.1), (locals.var_t3_dn2 / 0.1), (locals.var_t3_dn4 / 0.1), (locals.var_t3_dn5 / 0.1), (locals.var_t3_dn6 / 0.1), (locals.var_t3_dn7 / 0.1), (locals.var_t3_dn8 / 0.1), (locals.var_t3_dn9 / 0.1), (locals.var_t3_dn10 / 0.1), (locals.var_t3_dn11 / 0.1), (locals.var_t3_dn14 / 0.1),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign68240_e104901;
        locals.var_tx_dn0 = assign68240_e104901_d_n0;
        locals.var_tx_dn2 = assign68240_e104901_d_n2;
        locals.var_tx_dn4 = assign68240_e104901_d_n4;
        locals.var_tx_dn5 = assign68240_e104901_d_n5;
        locals.var_tx_dn6 = assign68240_e104901_d_n6;
        locals.var_tx_dn7 = assign68240_e104901_d_n7;
        locals.var_tx_dn8 = assign68240_e104901_d_n8;
        locals.var_tx_dn9 = assign68240_e104901_d_n9;
        locals.var_tx_dn10 = assign68240_e104901_d_n10;
        locals.var_tx_dn11 = assign68240_e104901_d_n11;
        locals.var_tx_dn14 = assign68240_e104901_d_n14;
        locals.var_tx_rv = 0.0;

        let (assign68250_e104911, assign68250_e104911_d_n0, assign68250_e104911_d_n2, assign68250_e104911_d_n4, assign68250_e104911_d_n5, assign68250_e104911_d_n6, assign68250_e104911_d_n7, assign68250_e104911_d_n8, assign68250_e104911_d_n9, assign68250_e104911_d_n10, assign68250_e104911_d_n11, assign68250_e104911_d_n14,) = {
    if ((locals.var_guard1605 != 0.0) && (locals.var_guard1606 != 0.0)) {
        let assign68250_e104908: f64 = (locals.var_tx * locals.var_tx);
        let assign68250_e104909: f64 = (1.0 + assign68250_e104908);
        (assign68250_e104909, ((locals.var_tx_dn0 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn0)), ((locals.var_tx_dn2 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn2)), ((locals.var_tx_dn4 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn4)), ((locals.var_tx_dn5 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn5)), ((locals.var_tx_dn6 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn6)), ((locals.var_tx_dn7 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn7)), ((locals.var_tx_dn8 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn8)), ((locals.var_tx_dn9 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn9)), ((locals.var_tx_dn10 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn10)), ((locals.var_tx_dn11 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn11)), ((locals.var_tx_dn14 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign68250_e104911;
        locals.var_t2_dn0 = assign68250_e104911_d_n0;
        locals.var_t2_dn2 = assign68250_e104911_d_n2;
        locals.var_t2_dn4 = assign68250_e104911_d_n4;
        locals.var_t2_dn5 = assign68250_e104911_d_n5;
        locals.var_t2_dn6 = assign68250_e104911_d_n6;
        locals.var_t2_dn7 = assign68250_e104911_d_n7;
        locals.var_t2_dn8 = assign68250_e104911_d_n8;
        locals.var_t2_dn9 = assign68250_e104911_d_n9;
        locals.var_t2_dn10 = assign68250_e104911_d_n10;
        locals.var_t2_dn11 = assign68250_e104911_d_n11;
        locals.var_t2_dn14 = assign68250_e104911_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign68260_e104921, assign68260_e104921_d_n0, assign68260_e104921_d_n2, assign68260_e104921_d_n4, assign68260_e104921_d_n5, assign68260_e104921_d_n6, assign68260_e104921_d_n7, assign68260_e104921_d_n8, assign68260_e104921_d_n9, assign68260_e104921_d_n10, assign68260_e104921_d_n11, assign68260_e104921_d_n14,) = {
    if ((locals.var_guard1605 != 0.0) && (locals.var_guard1606 != 0.0)) {
        let assign68260_e104918: f64 = (1.0 / locals.var_t2);
        let assign68260_e104919: f64 = (1.0 - assign68260_e104918);
        (assign68260_e104919, (-(-(locals.var_t2_dn0 / (locals.var_t2 * locals.var_t2)))), (-(-(locals.var_t2_dn2 / (locals.var_t2 * locals.var_t2)))), (-(-(locals.var_t2_dn4 / (locals.var_t2 * locals.var_t2)))), (-(-(locals.var_t2_dn5 / (locals.var_t2 * locals.var_t2)))), (-(-(locals.var_t2_dn6 / (locals.var_t2 * locals.var_t2)))), (-(-(locals.var_t2_dn7 / (locals.var_t2 * locals.var_t2)))), (-(-(locals.var_t2_dn8 / (locals.var_t2 * locals.var_t2)))), (-(-(locals.var_t2_dn9 / (locals.var_t2 * locals.var_t2)))), (-(-(locals.var_t2_dn10 / (locals.var_t2 * locals.var_t2)))), (-(-(locals.var_t2_dn11 / (locals.var_t2 * locals.var_t2)))), (-(-(locals.var_t2_dn14 / (locals.var_t2 * locals.var_t2)))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign68260_e104921;
        locals.var_t1_dn0 = assign68260_e104921_d_n0;
        locals.var_t1_dn2 = assign68260_e104921_d_n2;
        locals.var_t1_dn4 = assign68260_e104921_d_n4;
        locals.var_t1_dn5 = assign68260_e104921_d_n5;
        locals.var_t1_dn6 = assign68260_e104921_d_n6;
        locals.var_t1_dn7 = assign68260_e104921_d_n7;
        locals.var_t1_dn8 = assign68260_e104921_d_n8;
        locals.var_t1_dn9 = assign68260_e104921_d_n9;
        locals.var_t1_dn10 = assign68260_e104921_d_n10;
        locals.var_t1_dn11 = assign68260_e104921_d_n11;
        locals.var_t1_dn14 = assign68260_e104921_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign68270_e104929, assign68270_e104929_d_n0, assign68270_e104929_d_n2, assign68270_e104929_d_n4, assign68270_e104929_d_n5, assign68270_e104929_d_n6, assign68270_e104929_d_n7, assign68270_e104929_d_n8, assign68270_e104929_d_n9, assign68270_e104929_d_n10, assign68270_e104929_d_n11, assign68270_e104929_d_n14,) = {
    if ((locals.var_guard1605 != 0.0) && (locals.var_guard1606 != 0.0)) {
        let assign68270_e104927: f64 = (locals.var_etun * locals.var_t1);
        (assign68270_e104927, ((locals.var_etun_dn0 * locals.var_t1) + (locals.var_etun * locals.var_t1_dn0)), ((locals.var_etun_dn2 * locals.var_t1) + (locals.var_etun * locals.var_t1_dn2)), ((locals.var_etun_dn4 * locals.var_t1) + (locals.var_etun * locals.var_t1_dn4)), ((locals.var_etun_dn5 * locals.var_t1) + (locals.var_etun * locals.var_t1_dn5)), ((locals.var_etun_dn6 * locals.var_t1) + (locals.var_etun * locals.var_t1_dn6)), ((locals.var_etun_dn7 * locals.var_t1) + (locals.var_etun * locals.var_t1_dn7)), ((locals.var_etun_dn8 * locals.var_t1) + (locals.var_etun * locals.var_t1_dn8)), ((locals.var_etun_dn9 * locals.var_t1) + (locals.var_etun * locals.var_t1_dn9)), ((locals.var_etun_dn10 * locals.var_t1) + (locals.var_etun * locals.var_t1_dn10)), ((locals.var_etun_dn11 * locals.var_t1) + (locals.var_etun * locals.var_t1_dn11)), ((locals.var_etun_dn14 * locals.var_t1) + (locals.var_etun * locals.var_t1_dn14)),)
    } else {
        (locals.var_etun, locals.var_etun_dn0, locals.var_etun_dn2, locals.var_etun_dn4, locals.var_etun_dn5, locals.var_etun_dn6, locals.var_etun_dn7, locals.var_etun_dn8, locals.var_etun_dn9, locals.var_etun_dn10, locals.var_etun_dn11, locals.var_etun_dn14,)
    }
};
        locals.var_etun = assign68270_e104929;
        locals.var_etun_dn0 = assign68270_e104929_d_n0;
        locals.var_etun_dn2 = assign68270_e104929_d_n2;
        locals.var_etun_dn4 = assign68270_e104929_d_n4;
        locals.var_etun_dn5 = assign68270_e104929_d_n5;
        locals.var_etun_dn6 = assign68270_e104929_d_n6;
        locals.var_etun_dn7 = assign68270_e104929_d_n7;
        locals.var_etun_dn8 = assign68270_e104929_d_n8;
        locals.var_etun_dn9 = assign68270_e104929_d_n9;
        locals.var_etun_dn10 = assign68270_e104929_d_n10;
        locals.var_etun_dn11 = assign68270_e104929_d_n11;
        locals.var_etun_dn14 = assign68270_e104929_d_n14;
        locals.var_etun_rv = 0.0;

        let (assign68280_e104937, assign68280_e104937_d_n0, assign68280_e104937_d_n2, assign68280_e104937_d_n4, assign68280_e104937_d_n5, assign68280_e104937_d_n6, assign68280_e104937_d_n7, assign68280_e104937_d_n8, assign68280_e104937_d_n9, assign68280_e104937_d_n10, assign68280_e104937_d_n11, assign68280_e104937_d_n14,) = {
    if ((locals.var_guard1605 != 0.0) && (locals.var_guard1606 != 0.0)) {
        let assign68280_e104935: f64 = (locals.var_leff * locals.var_weff_nf);
        (assign68280_e104935, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign68280_e104937;
        locals.var_t0_dn0 = assign68280_e104937_d_n0;
        locals.var_t0_dn2 = assign68280_e104937_d_n2;
        locals.var_t0_dn4 = assign68280_e104937_d_n4;
        locals.var_t0_dn5 = assign68280_e104937_d_n5;
        locals.var_t0_dn6 = assign68280_e104937_d_n6;
        locals.var_t0_dn7 = assign68280_e104937_d_n7;
        locals.var_t0_dn8 = assign68280_e104937_d_n8;
        locals.var_t0_dn9 = assign68280_e104937_d_n9;
        locals.var_t0_dn10 = assign68280_e104937_d_n10;
        locals.var_t0_dn11 = assign68280_e104937_d_n11;
        locals.var_t0_dn14 = assign68280_e104937_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign68290_e104947, assign68290_e104947_d_n0, assign68290_e104947_d_n2, assign68290_e104947_d_n4, assign68290_e104947_d_n5, assign68290_e104947_d_n6, assign68290_e104947_d_n7, assign68290_e104947_d_n8, assign68290_e104947_d_n9, assign68290_e104947_d_n10, assign68290_e104947_d_n11, assign68290_e104947_d_n14,) = {
    if ((locals.var_guard1605 != 0.0) && (locals.var_guard1606 != 0.0)) {
        let assign68290_e104944: f64 = (locals.var_mks_gleak7 + locals.var_t0);
        let assign68290_e104945: f64 = (locals.var_mks_gleak7 / assign68290_e104944);
        (assign68290_e104945, (-((locals.var_mks_gleak7 * locals.var_t0_dn0) / (assign68290_e104944 * assign68290_e104944))), (-((locals.var_mks_gleak7 * locals.var_t0_dn2) / (assign68290_e104944 * assign68290_e104944))), (-((locals.var_mks_gleak7 * locals.var_t0_dn4) / (assign68290_e104944 * assign68290_e104944))), (-((locals.var_mks_gleak7 * locals.var_t0_dn5) / (assign68290_e104944 * assign68290_e104944))), (-((locals.var_mks_gleak7 * locals.var_t0_dn6) / (assign68290_e104944 * assign68290_e104944))), (-((locals.var_mks_gleak7 * locals.var_t0_dn7) / (assign68290_e104944 * assign68290_e104944))), (-((locals.var_mks_gleak7 * locals.var_t0_dn8) / (assign68290_e104944 * assign68290_e104944))), (-((locals.var_mks_gleak7 * locals.var_t0_dn9) / (assign68290_e104944 * assign68290_e104944))), (-((locals.var_mks_gleak7 * locals.var_t0_dn10) / (assign68290_e104944 * assign68290_e104944))), (-((locals.var_mks_gleak7 * locals.var_t0_dn11) / (assign68290_e104944 * assign68290_e104944))), (-((locals.var_mks_gleak7 * locals.var_t0_dn14) / (assign68290_e104944 * assign68290_e104944))),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign68290_e104947;
        locals.var_t7_dn0 = assign68290_e104947_d_n0;
        locals.var_t7_dn2 = assign68290_e104947_d_n2;
        locals.var_t7_dn4 = assign68290_e104947_d_n4;
        locals.var_t7_dn5 = assign68290_e104947_d_n5;
        locals.var_t7_dn6 = assign68290_e104947_d_n6;
        locals.var_t7_dn7 = assign68290_e104947_d_n7;
        locals.var_t7_dn8 = assign68290_e104947_d_n8;
        locals.var_t7_dn9 = assign68290_e104947_d_n9;
        locals.var_t7_dn10 = assign68290_e104947_d_n10;
        locals.var_t7_dn11 = assign68290_e104947_d_n11;
        locals.var_t7_dn14 = assign68290_e104947_d_n14;
        locals.var_t7_rv = 0.0;

        let (assign68300_e104953, assign68300_e104953_d_n0, assign68300_e104953_d_n2, assign68300_e104953_d_n4, assign68300_e104953_d_n5, assign68300_e104953_d_n6, assign68300_e104953_d_n7, assign68300_e104953_d_n8, assign68300_e104953_d_n9, assign68300_e104953_d_n10, assign68300_e104953_d_n11, assign68300_e104953_d_n14,) = {
    if ((locals.var_guard1605 != 0.0) && (locals.var_guard1606 != 0.0)) {
        (locals.var_uc_gleak6, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign68300_e104953;
        locals.var_t6_dn0 = assign68300_e104953_d_n0;
        locals.var_t6_dn2 = assign68300_e104953_d_n2;
        locals.var_t6_dn4 = assign68300_e104953_d_n4;
        locals.var_t6_dn5 = assign68300_e104953_d_n5;
        locals.var_t6_dn6 = assign68300_e104953_d_n6;
        locals.var_t6_dn7 = assign68300_e104953_d_n7;
        locals.var_t6_dn8 = assign68300_e104953_d_n8;
        locals.var_t6_dn9 = assign68300_e104953_d_n9;
        locals.var_t6_dn10 = assign68300_e104953_d_n10;
        locals.var_t6_dn11 = assign68300_e104953_d_n11;
        locals.var_t6_dn14 = assign68300_e104953_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign68310_e104963, assign68310_e104963_d_n0, assign68310_e104963_d_n2, assign68310_e104963_d_n4, assign68310_e104963_d_n5, assign68310_e104963_d_n6, assign68310_e104963_d_n7, assign68310_e104963_d_n8, assign68310_e104963_d_n9, assign68310_e104963_d_n10, assign68310_e104963_d_n11, assign68310_e104963_d_n14,) = {
    if ((locals.var_guard1605 != 0.0) && (locals.var_guard1606 != 0.0)) {
        let assign68310_e104960: f64 = (locals.var_t6 + locals.var_vdsz__blk441);
        let assign68310_e104961: f64 = (locals.var_t6 / assign68310_e104960);
        (assign68310_e104961, (((locals.var_t6_dn0 * assign68310_e104960) - (locals.var_t6 * (locals.var_t6_dn0 + locals.var_vdsz__blk441_dn0))) / (assign68310_e104960 * assign68310_e104960)), (((locals.var_t6_dn2 * assign68310_e104960) - (locals.var_t6 * (locals.var_t6_dn2 + locals.var_vdsz__blk441_dn2))) / (assign68310_e104960 * assign68310_e104960)), (((locals.var_t6_dn4 * assign68310_e104960) - (locals.var_t6 * (locals.var_t6_dn4 + locals.var_vdsz__blk441_dn4))) / (assign68310_e104960 * assign68310_e104960)), (((locals.var_t6_dn5 * assign68310_e104960) - (locals.var_t6 * (locals.var_t6_dn5 + locals.var_vdsz__blk441_dn5))) / (assign68310_e104960 * assign68310_e104960)), (((locals.var_t6_dn6 * assign68310_e104960) - (locals.var_t6 * (locals.var_t6_dn6 + locals.var_vdsz__blk441_dn6))) / (assign68310_e104960 * assign68310_e104960)), (((locals.var_t6_dn7 * assign68310_e104960) - (locals.var_t6 * (locals.var_t6_dn7 + locals.var_vdsz__blk441_dn7))) / (assign68310_e104960 * assign68310_e104960)), (((locals.var_t6_dn8 * assign68310_e104960) - (locals.var_t6 * (locals.var_t6_dn8 + locals.var_vdsz__blk441_dn8))) / (assign68310_e104960 * assign68310_e104960)), (((locals.var_t6_dn9 * assign68310_e104960) - (locals.var_t6 * (locals.var_t6_dn9 + locals.var_vdsz__blk441_dn9))) / (assign68310_e104960 * assign68310_e104960)), (((locals.var_t6_dn10 * assign68310_e104960) - (locals.var_t6 * (locals.var_t6_dn10 + locals.var_vdsz__blk441_dn10))) / (assign68310_e104960 * assign68310_e104960)), (((locals.var_t6_dn11 * assign68310_e104960) - (locals.var_t6 * (locals.var_t6_dn11 + locals.var_vdsz__blk441_dn11))) / (assign68310_e104960 * assign68310_e104960)), (((locals.var_t6_dn14 * assign68310_e104960) - (locals.var_t6 * (locals.var_t6_dn14 + locals.var_vdsz__blk441_dn14))) / (assign68310_e104960 * assign68310_e104960)),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign68310_e104963;
        locals.var_t9_dn0 = assign68310_e104963_d_n0;
        locals.var_t9_dn2 = assign68310_e104963_d_n2;
        locals.var_t9_dn4 = assign68310_e104963_d_n4;
        locals.var_t9_dn5 = assign68310_e104963_d_n5;
        locals.var_t9_dn6 = assign68310_e104963_d_n6;
        locals.var_t9_dn7 = assign68310_e104963_d_n7;
        locals.var_t9_dn8 = assign68310_e104963_d_n8;
        locals.var_t9_dn9 = assign68310_e104963_d_n9;
        locals.var_t9_dn10 = assign68310_e104963_d_n10;
        locals.var_t9_dn11 = assign68310_e104963_d_n11;
        locals.var_t9_dn14 = assign68310_e104963_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign68320_e104973, assign68320_e104973_d_n0, assign68320_e104973_d_n2, assign68320_e104973_d_n4, assign68320_e104973_d_n5, assign68320_e104973_d_n6, assign68320_e104973_d_n7, assign68320_e104973_d_n8, assign68320_e104973_d_n9, assign68320_e104973_d_n10, assign68320_e104973_d_n11, assign68320_e104973_d_n14,) = {
    if ((locals.var_guard1605 != 0.0) && (locals.var_guard1606 != 0.0)) {
        let assign68320_e104970: f64 = (locals.var_etun + 1e-25);
        let assign68320_e104971: f64 = (1.0 / assign68320_e104970);
        (assign68320_e104971, (-(locals.var_etun_dn0 / (assign68320_e104970 * assign68320_e104970))), (-(locals.var_etun_dn2 / (assign68320_e104970 * assign68320_e104970))), (-(locals.var_etun_dn4 / (assign68320_e104970 * assign68320_e104970))), (-(locals.var_etun_dn5 / (assign68320_e104970 * assign68320_e104970))), (-(locals.var_etun_dn6 / (assign68320_e104970 * assign68320_e104970))), (-(locals.var_etun_dn7 / (assign68320_e104970 * assign68320_e104970))), (-(locals.var_etun_dn8 / (assign68320_e104970 * assign68320_e104970))), (-(locals.var_etun_dn9 / (assign68320_e104970 * assign68320_e104970))), (-(locals.var_etun_dn10 / (assign68320_e104970 * assign68320_e104970))), (-(locals.var_etun_dn11 / (assign68320_e104970 * assign68320_e104970))), (-(locals.var_etun_dn14 / (assign68320_e104970 * assign68320_e104970))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign68320_e104973;
        locals.var_t4_dn0 = assign68320_e104973_d_n0;
        locals.var_t4_dn2 = assign68320_e104973_d_n2;
        locals.var_t4_dn4 = assign68320_e104973_d_n4;
        locals.var_t4_dn5 = assign68320_e104973_d_n5;
        locals.var_t4_dn6 = assign68320_e104973_d_n6;
        locals.var_t4_dn7 = assign68320_e104973_d_n7;
        locals.var_t4_dn8 = assign68320_e104973_d_n8;
        locals.var_t4_dn9 = assign68320_e104973_d_n9;
        locals.var_t4_dn10 = assign68320_e104973_d_n10;
        locals.var_t4_dn11 = assign68320_e104973_d_n11;
        locals.var_t4_dn14 = assign68320_e104973_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign68330_e104984, assign68330_e104984_d_n0, assign68330_e104984_d_n2, assign68330_e104984_d_n4, assign68330_e104984_d_n5, assign68330_e104984_d_n6, assign68330_e104984_d_n7, assign68330_e104984_d_n8, assign68330_e104984_d_n9, assign68330_e104984_d_n10, assign68330_e104984_d_n11, assign68330_e104984_d_n14,) = {
    if ((locals.var_guard1605 != 0.0) && (locals.var_guard1606 != 0.0)) {
        let assign68330_e104978: f64 = (-locals.var_uc_gleak2);
        let assign68330_e104980: f64 = (assign68330_e104978 * locals.var_egp32);
        let assign68330_e104982: f64 = (assign68330_e104980 * locals.var_t4);
        (assign68330_e104982, (((assign68330_e104978 * locals.var_egp32_dn0) * locals.var_t4) + (assign68330_e104980 * locals.var_t4_dn0)), (((assign68330_e104978 * locals.var_egp32_dn2) * locals.var_t4) + (assign68330_e104980 * locals.var_t4_dn2)), (((assign68330_e104978 * locals.var_egp32_dn4) * locals.var_t4) + (assign68330_e104980 * locals.var_t4_dn4)), (((assign68330_e104978 * locals.var_egp32_dn5) * locals.var_t4) + (assign68330_e104980 * locals.var_t4_dn5)), (((assign68330_e104978 * locals.var_egp32_dn6) * locals.var_t4) + (assign68330_e104980 * locals.var_t4_dn6)), (((assign68330_e104978 * locals.var_egp32_dn7) * locals.var_t4) + (assign68330_e104980 * locals.var_t4_dn7)), (((assign68330_e104978 * locals.var_egp32_dn8) * locals.var_t4) + (assign68330_e104980 * locals.var_t4_dn8)), (((assign68330_e104978 * locals.var_egp32_dn9) * locals.var_t4) + (assign68330_e104980 * locals.var_t4_dn9)), (((assign68330_e104978 * locals.var_egp32_dn10) * locals.var_t4) + (assign68330_e104980 * locals.var_t4_dn10)), (((assign68330_e104978 * locals.var_egp32_dn11) * locals.var_t4) + (assign68330_e104980 * locals.var_t4_dn11)), (((assign68330_e104978 * locals.var_egp32_dn14) * locals.var_t4) + (assign68330_e104980 * locals.var_t4_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign68330_e104984;
        locals.var_t1_dn0 = assign68330_e104984_d_n0;
        locals.var_t1_dn2 = assign68330_e104984_d_n2;
        locals.var_t1_dn4 = assign68330_e104984_d_n4;
        locals.var_t1_dn5 = assign68330_e104984_d_n5;
        locals.var_t1_dn6 = assign68330_e104984_d_n6;
        locals.var_t1_dn7 = assign68330_e104984_d_n7;
        locals.var_t1_dn8 = assign68330_e104984_d_n8;
        locals.var_t1_dn9 = assign68330_e104984_d_n9;
        locals.var_t1_dn10 = assign68330_e104984_d_n10;
        locals.var_t1_dn11 = assign68330_e104984_d_n11;
        locals.var_t1_dn14 = assign68330_e104984_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign68340_e104994, assign68340_e104994_d_n0, assign68340_e104994_d_n2, assign68340_e104994_d_n4, assign68340_e104994_d_n5, assign68340_e104994_d_n6, assign68340_e104994_d_n7, assign68340_e104994_d_n8, assign68340_e104994_d_n9, assign68340_e104994_d_n10, assign68340_e104994_d_n11, assign68340_e104994_d_n14,) = {
    if ((locals.var_guard1605 != 0.0) && (locals.var_guard1606 != 0.0)) {
        let assign68340_e104990: f64 = (locals.var_uc_gleak2 * locals.var_t4);
        let assign68340_e104992: f64 = (assign68340_e104990 * locals.var_t4);
        (assign68340_e104992, (((locals.var_uc_gleak2 * locals.var_t4_dn0) * locals.var_t4) + (assign68340_e104990 * locals.var_t4_dn0)), (((locals.var_uc_gleak2 * locals.var_t4_dn2) * locals.var_t4) + (assign68340_e104990 * locals.var_t4_dn2)), (((locals.var_uc_gleak2 * locals.var_t4_dn4) * locals.var_t4) + (assign68340_e104990 * locals.var_t4_dn4)), (((locals.var_uc_gleak2 * locals.var_t4_dn5) * locals.var_t4) + (assign68340_e104990 * locals.var_t4_dn5)), (((locals.var_uc_gleak2 * locals.var_t4_dn6) * locals.var_t4) + (assign68340_e104990 * locals.var_t4_dn6)), (((locals.var_uc_gleak2 * locals.var_t4_dn7) * locals.var_t4) + (assign68340_e104990 * locals.var_t4_dn7)), (((locals.var_uc_gleak2 * locals.var_t4_dn8) * locals.var_t4) + (assign68340_e104990 * locals.var_t4_dn8)), (((locals.var_uc_gleak2 * locals.var_t4_dn9) * locals.var_t4) + (assign68340_e104990 * locals.var_t4_dn9)), (((locals.var_uc_gleak2 * locals.var_t4_dn10) * locals.var_t4) + (assign68340_e104990 * locals.var_t4_dn10)), (((locals.var_uc_gleak2 * locals.var_t4_dn11) * locals.var_t4) + (assign68340_e104990 * locals.var_t4_dn11)), (((locals.var_uc_gleak2 * locals.var_t4_dn14) * locals.var_t4) + (assign68340_e104990 * locals.var_t4_dn14)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign68340_e104994;
        locals.var_t3_dn0 = assign68340_e104994_d_n0;
        locals.var_t3_dn2 = assign68340_e104994_d_n2;
        locals.var_t3_dn4 = assign68340_e104994_d_n4;
        locals.var_t3_dn5 = assign68340_e104994_d_n5;
        locals.var_t3_dn6 = assign68340_e104994_d_n6;
        locals.var_t3_dn7 = assign68340_e104994_d_n7;
        locals.var_t3_dn8 = assign68340_e104994_d_n8;
        locals.var_t3_dn9 = assign68340_e104994_d_n9;
        locals.var_t3_dn10 = assign68340_e104994_d_n10;
        locals.var_t3_dn11 = assign68340_e104994_d_n11;
        locals.var_t3_dn14 = assign68340_e104994_d_n14;
        locals.var_t3_rv = 0.0;

        let assign68350_e104997: f64 = (-34.0);
        let assign68350_e104998: f64 = if locals.var_t1 < assign68350_e104997 { 1.0 } else { 0.0 };
        locals.var_guard1609 = assign68350_e104998;
        locals.var_guard1609_rv = 0.0;

        let (assign68370_e105016, assign68370_e105016_d_n0, assign68370_e105016_d_n2, assign68370_e105016_d_n4, assign68370_e105016_d_n5, assign68370_e105016_d_n6, assign68370_e105016_d_n7, assign68370_e105016_d_n8, assign68370_e105016_d_n9, assign68370_e105016_d_n10, assign68370_e105016_d_n11, assign68370_e105016_d_n14,) = {
    if (((locals.var_guard1605 != 0.0) && (locals.var_guard1606 != 0.0)) && (locals.var_guard1609 == 0.0)) {
        let assign68370_e105014: f64 = (locals.var_t1).exp();
        (assign68370_e105014, (assign68370_e105014 * locals.var_t1_dn0), (assign68370_e105014 * locals.var_t1_dn2), (assign68370_e105014 * locals.var_t1_dn4), (assign68370_e105014 * locals.var_t1_dn5), (assign68370_e105014 * locals.var_t1_dn6), (assign68370_e105014 * locals.var_t1_dn7), (assign68370_e105014 * locals.var_t1_dn8), (assign68370_e105014 * locals.var_t1_dn9), (assign68370_e105014 * locals.var_t1_dn10), (assign68370_e105014 * locals.var_t1_dn11), (assign68370_e105014 * locals.var_t1_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign68370_e105016;
        locals.var_t2_dn0 = assign68370_e105016_d_n0;
        locals.var_t2_dn2 = assign68370_e105016_d_n2;
        locals.var_t2_dn4 = assign68370_e105016_d_n4;
        locals.var_t2_dn5 = assign68370_e105016_d_n5;
        locals.var_t2_dn6 = assign68370_e105016_d_n6;
        locals.var_t2_dn7 = assign68370_e105016_d_n7;
        locals.var_t2_dn8 = assign68370_e105016_d_n8;
        locals.var_t2_dn9 = assign68370_e105016_d_n9;
        locals.var_t2_dn10 = assign68370_e105016_d_n10;
        locals.var_t2_dn11 = assign68370_e105016_d_n11;
        locals.var_t2_dn14 = assign68370_e105016_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign68380_e105031, assign68380_e105031_d_n0, assign68380_e105031_d_n2, assign68380_e105031_d_n4, assign68380_e105031_d_n5, assign68380_e105031_d_n6, assign68380_e105031_d_n7, assign68380_e105031_d_n8, assign68380_e105031_d_n9, assign68380_e105031_d_n10, assign68380_e105031_d_n11, assign68380_e105031_d_n14,) = {
    if (((locals.var_guard1605 != 0.0) && (locals.var_guard1606 != 0.0)) && (locals.var_guard1609 == 0.0)) {
        let assign68380_e105025: f64 = (locals.var_uc_gleak1 / locals.var_egp12);
        let assign68380_e105027: f64 = (assign68380_e105025 * 1.6021918e-19);
        let assign68380_e105029: f64 = (assign68380_e105027 * locals.var_t0);
        (assign68380_e105029, ((((-((locals.var_uc_gleak1 * locals.var_egp12_dn0) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_t0) + (assign68380_e105027 * locals.var_t0_dn0)), ((((-((locals.var_uc_gleak1 * locals.var_egp12_dn2) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_t0) + (assign68380_e105027 * locals.var_t0_dn2)), ((((-((locals.var_uc_gleak1 * locals.var_egp12_dn4) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_t0) + (assign68380_e105027 * locals.var_t0_dn4)), ((((-((locals.var_uc_gleak1 * locals.var_egp12_dn5) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_t0) + (assign68380_e105027 * locals.var_t0_dn5)), ((((-((locals.var_uc_gleak1 * locals.var_egp12_dn6) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_t0) + (assign68380_e105027 * locals.var_t0_dn6)), ((((-((locals.var_uc_gleak1 * locals.var_egp12_dn7) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_t0) + (assign68380_e105027 * locals.var_t0_dn7)), ((((-((locals.var_uc_gleak1 * locals.var_egp12_dn8) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_t0) + (assign68380_e105027 * locals.var_t0_dn8)), ((((-((locals.var_uc_gleak1 * locals.var_egp12_dn9) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_t0) + (assign68380_e105027 * locals.var_t0_dn9)), ((((-((locals.var_uc_gleak1 * locals.var_egp12_dn10) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_t0) + (assign68380_e105027 * locals.var_t0_dn10)), ((((-((locals.var_uc_gleak1 * locals.var_egp12_dn11) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_t0) + (assign68380_e105027 * locals.var_t0_dn11)), ((((-((locals.var_uc_gleak1 * locals.var_egp12_dn14) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_t0) + (assign68380_e105027 * locals.var_t0_dn14)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign68380_e105031;
        locals.var_t3_dn0 = assign68380_e105031_d_n0;
        locals.var_t3_dn2 = assign68380_e105031_d_n2;
        locals.var_t3_dn4 = assign68380_e105031_d_n4;
        locals.var_t3_dn5 = assign68380_e105031_d_n5;
        locals.var_t3_dn6 = assign68380_e105031_d_n6;
        locals.var_t3_dn7 = assign68380_e105031_d_n7;
        locals.var_t3_dn8 = assign68380_e105031_d_n8;
        locals.var_t3_dn9 = assign68380_e105031_d_n9;
        locals.var_t3_dn10 = assign68380_e105031_d_n10;
        locals.var_t3_dn11 = assign68380_e105031_d_n11;
        locals.var_t3_dn14 = assign68380_e105031_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign68390_e105042, assign68390_e105042_d_n0, assign68390_e105042_d_n2, assign68390_e105042_d_n4, assign68390_e105042_d_n5, assign68390_e105042_d_n6, assign68390_e105042_d_n7, assign68390_e105042_d_n8, assign68390_e105042_d_n9, assign68390_e105042_d_n10, assign68390_e105042_d_n11, assign68390_e105042_d_n14,) = {
    if (((locals.var_guard1605 != 0.0) && (locals.var_guard1606 != 0.0)) && (locals.var_guard1609 == 0.0)) {
        let assign68390_e105040: f64 = (1.0 / locals.var_cnst0);
        (assign68390_e105040, (-(locals.var_cnst0_dn0 / (locals.var_cnst0 * locals.var_cnst0))), (-(locals.var_cnst0_dn2 / (locals.var_cnst0 * locals.var_cnst0))), (-(locals.var_cnst0_dn4 / (locals.var_cnst0 * locals.var_cnst0))), (-(locals.var_cnst0_dn5 / (locals.var_cnst0 * locals.var_cnst0))), (-(locals.var_cnst0_dn6 / (locals.var_cnst0 * locals.var_cnst0))), (-(locals.var_cnst0_dn7 / (locals.var_cnst0 * locals.var_cnst0))), (-(locals.var_cnst0_dn8 / (locals.var_cnst0 * locals.var_cnst0))), (-(locals.var_cnst0_dn9 / (locals.var_cnst0 * locals.var_cnst0))), (-(locals.var_cnst0_dn10 / (locals.var_cnst0 * locals.var_cnst0))), (-(locals.var_cnst0_dn11 / (locals.var_cnst0 * locals.var_cnst0))), (-(locals.var_cnst0_dn14 / (locals.var_cnst0 * locals.var_cnst0))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign68390_e105042;
        locals.var_t5_dn0 = assign68390_e105042_d_n0;
        locals.var_t5_dn2 = assign68390_e105042_d_n2;
        locals.var_t5_dn4 = assign68390_e105042_d_n4;
        locals.var_t5_dn5 = assign68390_e105042_d_n5;
        locals.var_t5_dn6 = assign68390_e105042_d_n6;
        locals.var_t5_dn7 = assign68390_e105042_d_n7;
        locals.var_t5_dn8 = assign68390_e105042_d_n8;
        locals.var_t5_dn9 = assign68390_e105042_d_n9;
        locals.var_t5_dn10 = assign68390_e105042_d_n10;
        locals.var_t5_dn11 = assign68390_e105042_d_n11;
        locals.var_t5_dn14 = assign68390_e105042_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign68400_e105058, assign68400_e105058_d_n0, assign68400_e105058_d_n2, assign68400_e105058_d_n4, assign68400_e105058_d_n5, assign68400_e105058_d_n6, assign68400_e105058_d_n7, assign68400_e105058_d_n8, assign68400_e105058_d_n9, assign68400_e105058_d_n10, assign68400_e105058_d_n11, assign68400_e105058_d_n14,) = {
    if (((locals.var_guard1605 != 0.0) && (locals.var_guard1606 != 0.0)) && (locals.var_guard1609 == 0.0)) {
        let assign68400_e105052: f64 = (locals.var_cox0 * 1e-12);
        let assign68400_e105053: f64 = (locals.var_qiu_noi + assign68400_e105052);
        let assign68400_e105055: f64 = (assign68400_e105053 * locals.var_t5);
        let assign68400_e105056: f64 = (assign68400_e105055).sqrt();
        (assign68400_e105056, (((locals.var_qiu_noi_dn0 * locals.var_t5) + (assign68400_e105053 * locals.var_t5_dn0)) / (2.0 * assign68400_e105056)), (((locals.var_qiu_noi_dn2 * locals.var_t5) + (assign68400_e105053 * locals.var_t5_dn2)) / (2.0 * assign68400_e105056)), (((locals.var_qiu_noi_dn4 * locals.var_t5) + (assign68400_e105053 * locals.var_t5_dn4)) / (2.0 * assign68400_e105056)), (((locals.var_qiu_noi_dn5 * locals.var_t5) + (assign68400_e105053 * locals.var_t5_dn5)) / (2.0 * assign68400_e105056)), (((locals.var_qiu_noi_dn6 * locals.var_t5) + (assign68400_e105053 * locals.var_t5_dn6)) / (2.0 * assign68400_e105056)), (((locals.var_qiu_noi_dn7 * locals.var_t5) + (assign68400_e105053 * locals.var_t5_dn7)) / (2.0 * assign68400_e105056)), (((locals.var_qiu_noi_dn8 * locals.var_t5) + (assign68400_e105053 * locals.var_t5_dn8)) / (2.0 * assign68400_e105056)), (((locals.var_qiu_noi_dn9 * locals.var_t5) + (assign68400_e105053 * locals.var_t5_dn9)) / (2.0 * assign68400_e105056)), (((locals.var_qiu_noi_dn10 * locals.var_t5) + (assign68400_e105053 * locals.var_t5_dn10)) / (2.0 * assign68400_e105056)), (((locals.var_qiu_noi_dn11 * locals.var_t5) + (assign68400_e105053 * locals.var_t5_dn11)) / (2.0 * assign68400_e105056)), (((locals.var_qiu_noi_dn14 * locals.var_t5) + (assign68400_e105053 * locals.var_t5_dn14)) / (2.0 * assign68400_e105056)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign68400_e105058;
        locals.var_t6_dn0 = assign68400_e105058_d_n0;
        locals.var_t6_dn2 = assign68400_e105058_d_n2;
        locals.var_t6_dn4 = assign68400_e105058_d_n4;
        locals.var_t6_dn5 = assign68400_e105058_d_n5;
        locals.var_t6_dn6 = assign68400_e105058_d_n6;
        locals.var_t6_dn7 = assign68400_e105058_d_n7;
        locals.var_t6_dn8 = assign68400_e105058_d_n8;
        locals.var_t6_dn9 = assign68400_e105058_d_n9;
        locals.var_t6_dn10 = assign68400_e105058_d_n10;
        locals.var_t6_dn11 = assign68400_e105058_d_n11;
        locals.var_t6_dn14 = assign68400_e105058_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign68410_e105071, assign68410_e105071_d_n0, assign68410_e105071_d_n2, assign68410_e105071_d_n4, assign68410_e105071_d_n5, assign68410_e105071_d_n6, assign68410_e105071_d_n7, assign68410_e105071_d_n8, assign68410_e105071_d_n9, assign68410_e105071_d_n10, assign68410_e105071_d_n11, assign68410_e105071_d_n14,) = {
    if (((locals.var_guard1605 != 0.0) && (locals.var_guard1606 != 0.0)) && (locals.var_guard1609 == 0.0)) {
        let assign68410_e105067: f64 = (locals.var_t2 * locals.var_t3);
        let assign68410_e105069: f64 = (assign68410_e105067 * locals.var_t6);
        (assign68410_e105069, ((((locals.var_t2_dn0 * locals.var_t3) + (locals.var_t2 * locals.var_t3_dn0)) * locals.var_t6) + (assign68410_e105067 * locals.var_t6_dn0)), ((((locals.var_t2_dn2 * locals.var_t3) + (locals.var_t2 * locals.var_t3_dn2)) * locals.var_t6) + (assign68410_e105067 * locals.var_t6_dn2)), ((((locals.var_t2_dn4 * locals.var_t3) + (locals.var_t2 * locals.var_t3_dn4)) * locals.var_t6) + (assign68410_e105067 * locals.var_t6_dn4)), ((((locals.var_t2_dn5 * locals.var_t3) + (locals.var_t2 * locals.var_t3_dn5)) * locals.var_t6) + (assign68410_e105067 * locals.var_t6_dn5)), ((((locals.var_t2_dn6 * locals.var_t3) + (locals.var_t2 * locals.var_t3_dn6)) * locals.var_t6) + (assign68410_e105067 * locals.var_t6_dn6)), ((((locals.var_t2_dn7 * locals.var_t3) + (locals.var_t2 * locals.var_t3_dn7)) * locals.var_t6) + (assign68410_e105067 * locals.var_t6_dn7)), ((((locals.var_t2_dn8 * locals.var_t3) + (locals.var_t2 * locals.var_t3_dn8)) * locals.var_t6) + (assign68410_e105067 * locals.var_t6_dn8)), ((((locals.var_t2_dn9 * locals.var_t3) + (locals.var_t2 * locals.var_t3_dn9)) * locals.var_t6) + (assign68410_e105067 * locals.var_t6_dn9)), ((((locals.var_t2_dn10 * locals.var_t3) + (locals.var_t2 * locals.var_t3_dn10)) * locals.var_t6) + (assign68410_e105067 * locals.var_t6_dn10)), ((((locals.var_t2_dn11 * locals.var_t3) + (locals.var_t2 * locals.var_t3_dn11)) * locals.var_t6) + (assign68410_e105067 * locals.var_t6_dn11)), ((((locals.var_t2_dn14 * locals.var_t3) + (locals.var_t2 * locals.var_t3_dn14)) * locals.var_t6) + (assign68410_e105067 * locals.var_t6_dn14)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign68410_e105071;
        locals.var_t4_dn0 = assign68410_e105071_d_n0;
        locals.var_t4_dn2 = assign68410_e105071_d_n2;
        locals.var_t4_dn4 = assign68410_e105071_d_n4;
        locals.var_t4_dn5 = assign68410_e105071_d_n5;
        locals.var_t4_dn6 = assign68410_e105071_d_n6;
        locals.var_t4_dn7 = assign68410_e105071_d_n7;
        locals.var_t4_dn8 = assign68410_e105071_d_n8;
        locals.var_t4_dn9 = assign68410_e105071_d_n9;
        locals.var_t4_dn10 = assign68410_e105071_d_n10;
        locals.var_t4_dn11 = assign68410_e105071_d_n11;
        locals.var_t4_dn14 = assign68410_e105071_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign68420_e105082, assign68420_e105082_d_n0, assign68420_e105082_d_n2, assign68420_e105082_d_n4, assign68420_e105082_d_n5, assign68420_e105082_d_n6, assign68420_e105082_d_n7, assign68420_e105082_d_n8, assign68420_e105082_d_n9, assign68420_e105082_d_n10, assign68420_e105082_d_n11, assign68420_e105082_d_n14,) = {
    if (((locals.var_guard1605 != 0.0) && (locals.var_guard1606 != 0.0)) && (locals.var_guard1609 == 0.0)) {
        let assign68420_e105080: f64 = (locals.var_t4 * locals.var_etun);
        (assign68420_e105080, ((locals.var_t4_dn0 * locals.var_etun) + (locals.var_t4 * locals.var_etun_dn0)), ((locals.var_t4_dn2 * locals.var_etun) + (locals.var_t4 * locals.var_etun_dn2)), ((locals.var_t4_dn4 * locals.var_etun) + (locals.var_t4 * locals.var_etun_dn4)), ((locals.var_t4_dn5 * locals.var_etun) + (locals.var_t4 * locals.var_etun_dn5)), ((locals.var_t4_dn6 * locals.var_etun) + (locals.var_t4 * locals.var_etun_dn6)), ((locals.var_t4_dn7 * locals.var_etun) + (locals.var_t4 * locals.var_etun_dn7)), ((locals.var_t4_dn8 * locals.var_etun) + (locals.var_t4 * locals.var_etun_dn8)), ((locals.var_t4_dn9 * locals.var_etun) + (locals.var_t4 * locals.var_etun_dn9)), ((locals.var_t4_dn10 * locals.var_etun) + (locals.var_t4 * locals.var_etun_dn10)), ((locals.var_t4_dn11 * locals.var_etun) + (locals.var_t4 * locals.var_etun_dn11)), ((locals.var_t4_dn14 * locals.var_etun) + (locals.var_t4 * locals.var_etun_dn14)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign68420_e105082;
        locals.var_t5_dn0 = assign68420_e105082_d_n0;
        locals.var_t5_dn2 = assign68420_e105082_d_n2;
        locals.var_t5_dn4 = assign68420_e105082_d_n4;
        locals.var_t5_dn5 = assign68420_e105082_d_n5;
        locals.var_t5_dn6 = assign68420_e105082_d_n6;
        locals.var_t5_dn7 = assign68420_e105082_d_n7;
        locals.var_t5_dn8 = assign68420_e105082_d_n8;
        locals.var_t5_dn9 = assign68420_e105082_d_n9;
        locals.var_t5_dn10 = assign68420_e105082_d_n10;
        locals.var_t5_dn11 = assign68420_e105082_d_n11;
        locals.var_t5_dn14 = assign68420_e105082_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign68430_e105093, assign68430_e105093_d_n0, assign68430_e105093_d_n2, assign68430_e105093_d_n4, assign68430_e105093_d_n5, assign68430_e105093_d_n6, assign68430_e105093_d_n7, assign68430_e105093_d_n8, assign68430_e105093_d_n9, assign68430_e105093_d_n10, assign68430_e105093_d_n11, assign68430_e105093_d_n14,) = {
    if (((locals.var_guard1605 != 0.0) && (locals.var_guard1606 != 0.0)) && (locals.var_guard1609 == 0.0)) {
        let assign68430_e105091: f64 = (locals.var_t5 * locals.var_etun);
        (assign68430_e105091, ((locals.var_t5_dn0 * locals.var_etun) + (locals.var_t5 * locals.var_etun_dn0)), ((locals.var_t5_dn2 * locals.var_etun) + (locals.var_t5 * locals.var_etun_dn2)), ((locals.var_t5_dn4 * locals.var_etun) + (locals.var_t5 * locals.var_etun_dn4)), ((locals.var_t5_dn5 * locals.var_etun) + (locals.var_t5 * locals.var_etun_dn5)), ((locals.var_t5_dn6 * locals.var_etun) + (locals.var_t5 * locals.var_etun_dn6)), ((locals.var_t5_dn7 * locals.var_etun) + (locals.var_t5 * locals.var_etun_dn7)), ((locals.var_t5_dn8 * locals.var_etun) + (locals.var_t5 * locals.var_etun_dn8)), ((locals.var_t5_dn9 * locals.var_etun) + (locals.var_t5 * locals.var_etun_dn9)), ((locals.var_t5_dn10 * locals.var_etun) + (locals.var_t5 * locals.var_etun_dn10)), ((locals.var_t5_dn11 * locals.var_etun) + (locals.var_t5 * locals.var_etun_dn11)), ((locals.var_t5_dn14 * locals.var_etun) + (locals.var_t5 * locals.var_etun_dn14)),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign68430_e105093;
        locals.var_t10_dn0 = assign68430_e105093_d_n0;
        locals.var_t10_dn2 = assign68430_e105093_d_n2;
        locals.var_t10_dn4 = assign68430_e105093_d_n4;
        locals.var_t10_dn5 = assign68430_e105093_d_n5;
        locals.var_t10_dn6 = assign68430_e105093_d_n6;
        locals.var_t10_dn7 = assign68430_e105093_d_n7;
        locals.var_t10_dn8 = assign68430_e105093_d_n8;
        locals.var_t10_dn9 = assign68430_e105093_d_n9;
        locals.var_t10_dn10 = assign68430_e105093_d_n10;
        locals.var_t10_dn11 = assign68430_e105093_d_n11;
        locals.var_t10_dn14 = assign68430_e105093_d_n14;
        locals.var_t10_rv = 0.0;

        let (assign68450_e105115, assign68450_e105115_d_n0, assign68450_e105115_d_n2, assign68450_e105115_d_n4, assign68450_e105115_d_n5, assign68450_e105115_d_n6, assign68450_e105115_d_n7, assign68450_e105115_d_n8, assign68450_e105115_d_n9, assign68450_e105115_d_n10, assign68450_e105115_d_n11, assign68450_e105115_d_n14,) = {
    if (locals.var_guard1605 != 0.0) {
        let assign68450_e105109: f64 = (-locals.var_uc_glksd2);
        let assign68450_e105111: f64 = (assign68450_e105109 * locals.var_vgs);
        let assign68450_e105113: f64 = (assign68450_e105111 + locals.var_mks_glksd3);
        (assign68450_e105113, 0.0, 0.0, 0.0, 0.0, (assign68450_e105109 * locals.var_vgs_dn6), (assign68450_e105109 * locals.var_vgs_dn7), (assign68450_e105109 * locals.var_vgs_dn8), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign68450_e105115;
        locals.var_t0_dn0 = assign68450_e105115_d_n0;
        locals.var_t0_dn2 = assign68450_e105115_d_n2;
        locals.var_t0_dn4 = assign68450_e105115_d_n4;
        locals.var_t0_dn5 = assign68450_e105115_d_n5;
        locals.var_t0_dn6 = assign68450_e105115_d_n6;
        locals.var_t0_dn7 = assign68450_e105115_d_n7;
        locals.var_t0_dn8 = assign68450_e105115_d_n8;
        locals.var_t0_dn9 = assign68450_e105115_d_n9;
        locals.var_t0_dn10 = assign68450_e105115_d_n10;
        locals.var_t0_dn11 = assign68450_e105115_d_n11;
        locals.var_t0_dn14 = assign68450_e105115_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign68460_e105122, assign68460_e105122_d_n0, assign68460_e105122_d_n2, assign68460_e105122_d_n4, assign68460_e105122_d_n5, assign68460_e105122_d_n6, assign68460_e105122_d_n7, assign68460_e105122_d_n8, assign68460_e105122_d_n9, assign68460_e105122_d_n10, assign68460_e105122_d_n11, assign68460_e105122_d_n14,) = {
    if (locals.var_guard1605 != 0.0) {
        let assign68460_e105119: f64 = (locals.var_tox0 * locals.var_t0);
        let assign68460_e105120: f64 = (assign68460_e105119).exp();
        (assign68460_e105120, (assign68460_e105120 * (locals.var_tox0 * locals.var_t0_dn0)), (assign68460_e105120 * (locals.var_tox0 * locals.var_t0_dn2)), (assign68460_e105120 * (locals.var_tox0 * locals.var_t0_dn4)), (assign68460_e105120 * (locals.var_tox0 * locals.var_t0_dn5)), (assign68460_e105120 * (locals.var_tox0 * locals.var_t0_dn6)), (assign68460_e105120 * (locals.var_tox0 * locals.var_t0_dn7)), (assign68460_e105120 * (locals.var_tox0 * locals.var_t0_dn8)), (assign68460_e105120 * (locals.var_tox0 * locals.var_t0_dn9)), (assign68460_e105120 * (locals.var_tox0 * locals.var_t0_dn10)), (assign68460_e105120 * (locals.var_tox0 * locals.var_t0_dn11)), (assign68460_e105120 * (locals.var_tox0 * locals.var_t0_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign68460_e105122;
        locals.var_t2_dn0 = assign68460_e105122_d_n0;
        locals.var_t2_dn2 = assign68460_e105122_d_n2;
        locals.var_t2_dn4 = assign68460_e105122_d_n4;
        locals.var_t2_dn5 = assign68460_e105122_d_n5;
        locals.var_t2_dn6 = assign68460_e105122_d_n6;
        locals.var_t2_dn7 = assign68460_e105122_d_n7;
        locals.var_t2_dn8 = assign68460_e105122_d_n8;
        locals.var_t2_dn9 = assign68460_e105122_d_n9;
        locals.var_t2_dn10 = assign68460_e105122_d_n10;
        locals.var_t2_dn11 = assign68460_e105122_d_n11;
        locals.var_t2_dn14 = assign68460_e105122_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign68470_e105130, assign68470_e105130_d_n0, assign68470_e105130_d_n2, assign68470_e105130_d_n4, assign68470_e105130_d_n5, assign68470_e105130_d_n6, assign68470_e105130_d_n7, assign68470_e105130_d_n8, assign68470_e105130_d_n9, assign68470_e105130_d_n10, assign68470_e105130_d_n11, assign68470_e105130_d_n14,) = {
    if (locals.var_guard1605 != 0.0) {
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_tox0;
        let assign68470_e105126: f64 = (locals.var_vgs * __rspice_inv_cse_0);
        let assign68470_e105128: f64 = (assign68470_e105126 * __rspice_inv_cse_0);
        (assign68470_e105128, 0.0, 0.0, 0.0, 0.0, ((locals.var_vgs_dn6 / locals.var_tox0) / locals.var_tox0), ((locals.var_vgs_dn7 / locals.var_tox0) / locals.var_tox0), ((locals.var_vgs_dn8 / locals.var_tox0) / locals.var_tox0), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign68470_e105130;
        locals.var_t0_dn0 = assign68470_e105130_d_n0;
        locals.var_t0_dn2 = assign68470_e105130_d_n2;
        locals.var_t0_dn4 = assign68470_e105130_d_n4;
        locals.var_t0_dn5 = assign68470_e105130_d_n5;
        locals.var_t0_dn6 = assign68470_e105130_d_n6;
        locals.var_t0_dn7 = assign68470_e105130_d_n7;
        locals.var_t0_dn8 = assign68470_e105130_d_n8;
        locals.var_t0_dn9 = assign68470_e105130_d_n9;
        locals.var_t0_dn10 = assign68470_e105130_d_n10;
        locals.var_t0_dn11 = assign68470_e105130_d_n11;
        locals.var_t0_dn14 = assign68470_e105130_d_n14;
        locals.var_t0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_258(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign68480_e105136, assign68480_e105136_d_n0, assign68480_e105136_d_n2, assign68480_e105136_d_n4, assign68480_e105136_d_n5, assign68480_e105136_d_n6, assign68480_e105136_d_n7, assign68480_e105136_d_n8, assign68480_e105136_d_n9, assign68480_e105136_d_n10, assign68480_e105136_d_n11, assign68480_e105136_d_n14,) = {
    if (locals.var_guard1605 != 0.0) {
        let assign68480_e105134: f64 = (locals.var_vgs * locals.var_t0);
        (assign68480_e105134, (locals.var_vgs * locals.var_t0_dn0), (locals.var_vgs * locals.var_t0_dn2), (locals.var_vgs * locals.var_t0_dn4), (locals.var_vgs * locals.var_t0_dn5), ((locals.var_vgs_dn6 * locals.var_t0) + (locals.var_vgs * locals.var_t0_dn6)), ((locals.var_vgs_dn7 * locals.var_t0) + (locals.var_vgs * locals.var_t0_dn7)), ((locals.var_vgs_dn8 * locals.var_t0) + (locals.var_vgs * locals.var_t0_dn8)), (locals.var_vgs * locals.var_t0_dn9), (locals.var_vgs * locals.var_t0_dn10), (locals.var_vgs * locals.var_t0_dn11), (locals.var_vgs * locals.var_t0_dn14),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign68480_e105136;
        locals.var_t3_dn0 = assign68480_e105136_d_n0;
        locals.var_t3_dn2 = assign68480_e105136_d_n2;
        locals.var_t3_dn4 = assign68480_e105136_d_n4;
        locals.var_t3_dn5 = assign68480_e105136_d_n5;
        locals.var_t3_dn6 = assign68480_e105136_d_n6;
        locals.var_t3_dn7 = assign68480_e105136_d_n7;
        locals.var_t3_dn8 = assign68480_e105136_d_n8;
        locals.var_t3_dn9 = assign68480_e105136_d_n9;
        locals.var_t3_dn10 = assign68480_e105136_d_n10;
        locals.var_t3_dn11 = assign68480_e105136_d_n11;
        locals.var_t3_dn14 = assign68480_e105136_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign68490_e105144, assign68490_e105144_d_n0, assign68490_e105144_d_n2, assign68490_e105144_d_n4, assign68490_e105144_d_n5, assign68490_e105144_d_n6, assign68490_e105144_d_n7, assign68490_e105144_d_n8, assign68490_e105144_d_n9, assign68490_e105144_d_n10, assign68490_e105144_d_n11, assign68490_e105144_d_n14,) = {
    if (locals.var_guard1605 != 0.0) {
        let assign68490_e105140: f64 = (locals.var_uc_glksd1 / 1000000.0);
        let assign68490_e105142: f64 = (assign68490_e105140 * locals.var_weff_nf);
        (assign68490_e105142, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign68490_e105144;
        locals.var_t4_dn0 = assign68490_e105144_d_n0;
        locals.var_t4_dn2 = assign68490_e105144_d_n2;
        locals.var_t4_dn4 = assign68490_e105144_d_n4;
        locals.var_t4_dn5 = assign68490_e105144_d_n5;
        locals.var_t4_dn6 = assign68490_e105144_d_n6;
        locals.var_t4_dn7 = assign68490_e105144_d_n7;
        locals.var_t4_dn8 = assign68490_e105144_d_n8;
        locals.var_t4_dn9 = assign68490_e105144_d_n9;
        locals.var_t4_dn10 = assign68490_e105144_d_n10;
        locals.var_t4_dn11 = assign68490_e105144_d_n11;
        locals.var_t4_dn14 = assign68490_e105144_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign68530_e105170, assign68530_e105170_d_n0, assign68530_e105170_d_n2, assign68530_e105170_d_n4, assign68530_e105170_d_n5, assign68530_e105170_d_n6, assign68530_e105170_d_n7, assign68530_e105170_d_n8, assign68530_e105170_d_n9, assign68530_e105170_d_n10, assign68530_e105170_d_n11, assign68530_e105170_d_n14,) = {
    if (locals.var_guard1605 != 0.0) {
        let assign68530_e105168: f64 = (locals.var_vgs - locals.var_vds);
        (assign68530_e105168, (-locals.var_vds_dn0), (-locals.var_vds_dn2), (-locals.var_vds_dn4), (-locals.var_vds_dn5), (locals.var_vgs_dn6 - locals.var_vds_dn6), (locals.var_vgs_dn7 - locals.var_vds_dn7), (locals.var_vgs_dn8 - locals.var_vds_dn8), (-locals.var_vds_dn9), (-locals.var_vds_dn10), (-locals.var_vds_dn11), (-locals.var_vds_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign68530_e105170;
        locals.var_t1_dn0 = assign68530_e105170_d_n0;
        locals.var_t1_dn2 = assign68530_e105170_d_n2;
        locals.var_t1_dn4 = assign68530_e105170_d_n4;
        locals.var_t1_dn5 = assign68530_e105170_d_n5;
        locals.var_t1_dn6 = assign68530_e105170_d_n6;
        locals.var_t1_dn7 = assign68530_e105170_d_n7;
        locals.var_t1_dn8 = assign68530_e105170_d_n8;
        locals.var_t1_dn9 = assign68530_e105170_d_n9;
        locals.var_t1_dn10 = assign68530_e105170_d_n10;
        locals.var_t1_dn11 = assign68530_e105170_d_n11;
        locals.var_t1_dn14 = assign68530_e105170_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign68540_e105179, assign68540_e105179_d_n0, assign68540_e105179_d_n2, assign68540_e105179_d_n4, assign68540_e105179_d_n5, assign68540_e105179_d_n6, assign68540_e105179_d_n7, assign68540_e105179_d_n8, assign68540_e105179_d_n9, assign68540_e105179_d_n10, assign68540_e105179_d_n11, assign68540_e105179_d_n14,) = {
    if (locals.var_guard1605 != 0.0) {
        let assign68540_e105173: f64 = (-locals.var_uc_glksd2);
        let assign68540_e105175: f64 = (assign68540_e105173 * locals.var_t1);
        let assign68540_e105177: f64 = (assign68540_e105175 + locals.var_mks_glksd3);
        (assign68540_e105177, (assign68540_e105173 * locals.var_t1_dn0), (assign68540_e105173 * locals.var_t1_dn2), (assign68540_e105173 * locals.var_t1_dn4), (assign68540_e105173 * locals.var_t1_dn5), (assign68540_e105173 * locals.var_t1_dn6), (assign68540_e105173 * locals.var_t1_dn7), (assign68540_e105173 * locals.var_t1_dn8), (assign68540_e105173 * locals.var_t1_dn9), (assign68540_e105173 * locals.var_t1_dn10), (assign68540_e105173 * locals.var_t1_dn11), (assign68540_e105173 * locals.var_t1_dn14),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign68540_e105179;
        locals.var_t0_dn0 = assign68540_e105179_d_n0;
        locals.var_t0_dn2 = assign68540_e105179_d_n2;
        locals.var_t0_dn4 = assign68540_e105179_d_n4;
        locals.var_t0_dn5 = assign68540_e105179_d_n5;
        locals.var_t0_dn6 = assign68540_e105179_d_n6;
        locals.var_t0_dn7 = assign68540_e105179_d_n7;
        locals.var_t0_dn8 = assign68540_e105179_d_n8;
        locals.var_t0_dn9 = assign68540_e105179_d_n9;
        locals.var_t0_dn10 = assign68540_e105179_d_n10;
        locals.var_t0_dn11 = assign68540_e105179_d_n11;
        locals.var_t0_dn14 = assign68540_e105179_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign68550_e105186, assign68550_e105186_d_n0, assign68550_e105186_d_n2, assign68550_e105186_d_n4, assign68550_e105186_d_n5, assign68550_e105186_d_n6, assign68550_e105186_d_n7, assign68550_e105186_d_n8, assign68550_e105186_d_n9, assign68550_e105186_d_n10, assign68550_e105186_d_n11, assign68550_e105186_d_n14,) = {
    if (locals.var_guard1605 != 0.0) {
        let assign68550_e105183: f64 = (locals.var_tox0 * locals.var_t0);
        let assign68550_e105184: f64 = (assign68550_e105183).exp();
        (assign68550_e105184, (assign68550_e105184 * (locals.var_tox0 * locals.var_t0_dn0)), (assign68550_e105184 * (locals.var_tox0 * locals.var_t0_dn2)), (assign68550_e105184 * (locals.var_tox0 * locals.var_t0_dn4)), (assign68550_e105184 * (locals.var_tox0 * locals.var_t0_dn5)), (assign68550_e105184 * (locals.var_tox0 * locals.var_t0_dn6)), (assign68550_e105184 * (locals.var_tox0 * locals.var_t0_dn7)), (assign68550_e105184 * (locals.var_tox0 * locals.var_t0_dn8)), (assign68550_e105184 * (locals.var_tox0 * locals.var_t0_dn9)), (assign68550_e105184 * (locals.var_tox0 * locals.var_t0_dn10)), (assign68550_e105184 * (locals.var_tox0 * locals.var_t0_dn11)), (assign68550_e105184 * (locals.var_tox0 * locals.var_t0_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign68550_e105186;
        locals.var_t2_dn0 = assign68550_e105186_d_n0;
        locals.var_t2_dn2 = assign68550_e105186_d_n2;
        locals.var_t2_dn4 = assign68550_e105186_d_n4;
        locals.var_t2_dn5 = assign68550_e105186_d_n5;
        locals.var_t2_dn6 = assign68550_e105186_d_n6;
        locals.var_t2_dn7 = assign68550_e105186_d_n7;
        locals.var_t2_dn8 = assign68550_e105186_d_n8;
        locals.var_t2_dn9 = assign68550_e105186_d_n9;
        locals.var_t2_dn10 = assign68550_e105186_d_n10;
        locals.var_t2_dn11 = assign68550_e105186_d_n11;
        locals.var_t2_dn14 = assign68550_e105186_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign68560_e105194, assign68560_e105194_d_n0, assign68560_e105194_d_n2, assign68560_e105194_d_n4, assign68560_e105194_d_n5, assign68560_e105194_d_n6, assign68560_e105194_d_n7, assign68560_e105194_d_n8, assign68560_e105194_d_n9, assign68560_e105194_d_n10, assign68560_e105194_d_n11, assign68560_e105194_d_n14,) = {
    if (locals.var_guard1605 != 0.0) {
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_tox0;
        let assign68560_e105190: f64 = (locals.var_t1 * __rspice_inv_cse_0);
        let assign68560_e105192: f64 = (assign68560_e105190 * __rspice_inv_cse_0);
        (assign68560_e105192, ((locals.var_t1_dn0 / locals.var_tox0) / locals.var_tox0), ((locals.var_t1_dn2 / locals.var_tox0) / locals.var_tox0), ((locals.var_t1_dn4 / locals.var_tox0) / locals.var_tox0), ((locals.var_t1_dn5 / locals.var_tox0) / locals.var_tox0), ((locals.var_t1_dn6 / locals.var_tox0) / locals.var_tox0), ((locals.var_t1_dn7 / locals.var_tox0) / locals.var_tox0), ((locals.var_t1_dn8 / locals.var_tox0) / locals.var_tox0), ((locals.var_t1_dn9 / locals.var_tox0) / locals.var_tox0), ((locals.var_t1_dn10 / locals.var_tox0) / locals.var_tox0), ((locals.var_t1_dn11 / locals.var_tox0) / locals.var_tox0), ((locals.var_t1_dn14 / locals.var_tox0) / locals.var_tox0),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign68560_e105194;
        locals.var_t0_dn0 = assign68560_e105194_d_n0;
        locals.var_t0_dn2 = assign68560_e105194_d_n2;
        locals.var_t0_dn4 = assign68560_e105194_d_n4;
        locals.var_t0_dn5 = assign68560_e105194_d_n5;
        locals.var_t0_dn6 = assign68560_e105194_d_n6;
        locals.var_t0_dn7 = assign68560_e105194_d_n7;
        locals.var_t0_dn8 = assign68560_e105194_d_n8;
        locals.var_t0_dn9 = assign68560_e105194_d_n9;
        locals.var_t0_dn10 = assign68560_e105194_d_n10;
        locals.var_t0_dn11 = assign68560_e105194_d_n11;
        locals.var_t0_dn14 = assign68560_e105194_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign68570_e105200, assign68570_e105200_d_n0, assign68570_e105200_d_n2, assign68570_e105200_d_n4, assign68570_e105200_d_n5, assign68570_e105200_d_n6, assign68570_e105200_d_n7, assign68570_e105200_d_n8, assign68570_e105200_d_n9, assign68570_e105200_d_n10, assign68570_e105200_d_n11, assign68570_e105200_d_n14,) = {
    if (locals.var_guard1605 != 0.0) {
        let assign68570_e105198: f64 = (locals.var_t1 * locals.var_t0);
        (assign68570_e105198, ((locals.var_t1_dn0 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn0)), ((locals.var_t1_dn2 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn2)), ((locals.var_t1_dn4 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn4)), ((locals.var_t1_dn5 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn5)), ((locals.var_t1_dn6 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn6)), ((locals.var_t1_dn7 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn7)), ((locals.var_t1_dn8 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn8)), ((locals.var_t1_dn9 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn9)), ((locals.var_t1_dn10 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn10)), ((locals.var_t1_dn11 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn11)), ((locals.var_t1_dn14 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn14)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign68570_e105200;
        locals.var_t3_dn0 = assign68570_e105200_d_n0;
        locals.var_t3_dn2 = assign68570_e105200_d_n2;
        locals.var_t3_dn4 = assign68570_e105200_d_n4;
        locals.var_t3_dn5 = assign68570_e105200_d_n5;
        locals.var_t3_dn6 = assign68570_e105200_d_n6;
        locals.var_t3_dn7 = assign68570_e105200_d_n7;
        locals.var_t3_dn8 = assign68570_e105200_d_n8;
        locals.var_t3_dn9 = assign68570_e105200_d_n9;
        locals.var_t3_dn10 = assign68570_e105200_d_n10;
        locals.var_t3_dn11 = assign68570_e105200_d_n11;
        locals.var_t3_dn14 = assign68570_e105200_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign68580_e105208, assign68580_e105208_d_n0, assign68580_e105208_d_n2, assign68580_e105208_d_n4, assign68580_e105208_d_n5, assign68580_e105208_d_n6, assign68580_e105208_d_n7, assign68580_e105208_d_n8, assign68580_e105208_d_n9, assign68580_e105208_d_n10, assign68580_e105208_d_n11, assign68580_e105208_d_n14,) = {
    if (locals.var_guard1605 != 0.0) {
        let assign68580_e105204: f64 = (locals.var_uc_glksd1 / 1000000.0);
        let assign68580_e105206: f64 = (assign68580_e105204 * locals.var_weff_nf);
        (assign68580_e105206, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign68580_e105208;
        locals.var_t4_dn0 = assign68580_e105208_d_n0;
        locals.var_t4_dn2 = assign68580_e105208_d_n2;
        locals.var_t4_dn4 = assign68580_e105208_d_n4;
        locals.var_t4_dn5 = assign68580_e105208_d_n5;
        locals.var_t4_dn6 = assign68580_e105208_d_n6;
        locals.var_t4_dn7 = assign68580_e105208_d_n7;
        locals.var_t4_dn8 = assign68580_e105208_d_n8;
        locals.var_t4_dn9 = assign68580_e105208_d_n9;
        locals.var_t4_dn10 = assign68580_e105208_d_n10;
        locals.var_t4_dn11 = assign68580_e105208_d_n11;
        locals.var_t4_dn14 = assign68580_e105208_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign68620_e105241, assign68620_e105241_d_n0, assign68620_e105241_d_n2, assign68620_e105241_d_n4, assign68620_e105241_d_n5, assign68620_e105241_d_n6, assign68620_e105241_d_n7, assign68620_e105241_d_n8, assign68620_e105241_d_n9, assign68620_e105241_d_n10, assign68620_e105241_d_n11, assign68620_e105241_d_n14,) = {
    if (locals.var_guard1605 != 0.0) {
        let assign68620_e105232: f64 = (locals.var_vgs - locals.var_vbs);
        let assign68620_e105233: f64 = (-assign68620_e105232);
        let assign68620_e105235: f64 = (assign68620_e105233 + locals.var_vfb);
        let assign68620_e105237: f64 = (assign68620_e105235 + p.p258);
        let assign68620_e105239: f64 = (assign68620_e105237 / locals.var_tox0);
        (assign68620_e105239, 0.0, 0.0, 0.0, 0.0, ((-(locals.var_vgs_dn6 - locals.var_vbs_dn6)) / locals.var_tox0), ((-locals.var_vgs_dn7) / locals.var_tox0), ((-(locals.var_vgs_dn8 - locals.var_vbs_dn8)) / locals.var_tox0), ((-(-locals.var_vbs_dn9)) / locals.var_tox0), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_etun, locals.var_etun_dn0, locals.var_etun_dn2, locals.var_etun_dn4, locals.var_etun_dn5, locals.var_etun_dn6, locals.var_etun_dn7, locals.var_etun_dn8, locals.var_etun_dn9, locals.var_etun_dn10, locals.var_etun_dn11, locals.var_etun_dn14,)
    }
};
        locals.var_etun = assign68620_e105241;
        locals.var_etun_dn0 = assign68620_e105241_d_n0;
        locals.var_etun_dn2 = assign68620_e105241_d_n2;
        locals.var_etun_dn4 = assign68620_e105241_d_n4;
        locals.var_etun_dn5 = assign68620_e105241_d_n5;
        locals.var_etun_dn6 = assign68620_e105241_d_n6;
        locals.var_etun_dn7 = assign68620_e105241_d_n7;
        locals.var_etun_dn8 = assign68620_e105241_d_n8;
        locals.var_etun_dn9 = assign68620_e105241_d_n9;
        locals.var_etun_dn10 = assign68620_e105241_d_n10;
        locals.var_etun_dn11 = assign68620_e105241_d_n11;
        locals.var_etun_dn14 = assign68620_e105241_d_n14;
        locals.var_etun_rv = 0.0;

        let (assign68630_e105258, assign68630_e105258_d_n0, assign68630_e105258_d_n2, assign68630_e105258_d_n4, assign68630_e105258_d_n5, assign68630_e105258_d_n6, assign68630_e105258_d_n7, assign68630_e105258_d_n8, assign68630_e105258_d_n9, assign68630_e105258_d_n10, assign68630_e105258_d_n11, assign68630_e105258_d_n14,) = {
    if (locals.var_guard1605 != 0.0) {
        let assign68630_e105245: f64 = (locals.var_etun * locals.var_etun);
        let assign68630_e105249: f64 = (0.01 / 0.01);
        let assign68630_e105250: f64 = (4.0 * assign68630_e105249);
        let assign68630_e105253: f64 = (0.01 / 0.01);
        let assign68630_e105254: f64 = (assign68630_e105250 * assign68630_e105253);
        let assign68630_e105255: f64 = (assign68630_e105245 + assign68630_e105254);
        let assign68630_e105256: f64 = (assign68630_e105255).sqrt();
        (assign68630_e105256, (((locals.var_etun_dn0 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn0)) / (2.0 * assign68630_e105256)), (((locals.var_etun_dn2 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn2)) / (2.0 * assign68630_e105256)), (((locals.var_etun_dn4 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn4)) / (2.0 * assign68630_e105256)), (((locals.var_etun_dn5 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn5)) / (2.0 * assign68630_e105256)), (((locals.var_etun_dn6 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn6)) / (2.0 * assign68630_e105256)), (((locals.var_etun_dn7 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn7)) / (2.0 * assign68630_e105256)), (((locals.var_etun_dn8 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn8)) / (2.0 * assign68630_e105256)), (((locals.var_etun_dn9 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn9)) / (2.0 * assign68630_e105256)), (((locals.var_etun_dn10 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn10)) / (2.0 * assign68630_e105256)), (((locals.var_etun_dn11 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn11)) / (2.0 * assign68630_e105256)), (((locals.var_etun_dn14 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn14)) / (2.0 * assign68630_e105256)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign68630_e105258;
        locals.var_tmf2_dn0 = assign68630_e105258_d_n0;
        locals.var_tmf2_dn2 = assign68630_e105258_d_n2;
        locals.var_tmf2_dn4 = assign68630_e105258_d_n4;
        locals.var_tmf2_dn5 = assign68630_e105258_d_n5;
        locals.var_tmf2_dn6 = assign68630_e105258_d_n6;
        locals.var_tmf2_dn7 = assign68630_e105258_d_n7;
        locals.var_tmf2_dn8 = assign68630_e105258_d_n8;
        locals.var_tmf2_dn9 = assign68630_e105258_d_n9;
        locals.var_tmf2_dn10 = assign68630_e105258_d_n10;
        locals.var_tmf2_dn11 = assign68630_e105258_d_n11;
        locals.var_tmf2_dn14 = assign68630_e105258_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign68640_e105268, assign68640_e105268_d_n0, assign68640_e105268_d_n2, assign68640_e105268_d_n4, assign68640_e105268_d_n5, assign68640_e105268_d_n6, assign68640_e105268_d_n7, assign68640_e105268_d_n8, assign68640_e105268_d_n9, assign68640_e105268_d_n10, assign68640_e105268_d_n11, assign68640_e105268_d_n14,) = {
    if (locals.var_guard1605 != 0.0) {
        let assign68640_e105264: f64 = (locals.var_etun / locals.var_tmf2);
        let assign68640_e105265: f64 = (1.0 + assign68640_e105264);
        let assign68640_e105266: f64 = (0.5 * assign68640_e105265);
        (assign68640_e105266, (0.5 * (((locals.var_etun_dn0 * locals.var_tmf2) - (locals.var_etun * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_etun_dn2 * locals.var_tmf2) - (locals.var_etun * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_etun_dn4 * locals.var_tmf2) - (locals.var_etun * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_etun_dn5 * locals.var_tmf2) - (locals.var_etun * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_etun_dn6 * locals.var_tmf2) - (locals.var_etun * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_etun_dn7 * locals.var_tmf2) - (locals.var_etun * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_etun_dn8 * locals.var_tmf2) - (locals.var_etun * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_etun_dn9 * locals.var_tmf2) - (locals.var_etun * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_etun_dn10 * locals.var_tmf2) - (locals.var_etun * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_etun_dn11 * locals.var_tmf2) - (locals.var_etun * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_etun_dn14 * locals.var_tmf2) - (locals.var_etun * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign68640_e105268;
        locals.var_t5_dn0 = assign68640_e105268_d_n0;
        locals.var_t5_dn2 = assign68640_e105268_d_n2;
        locals.var_t5_dn4 = assign68640_e105268_d_n4;
        locals.var_t5_dn5 = assign68640_e105268_d_n5;
        locals.var_t5_dn6 = assign68640_e105268_d_n6;
        locals.var_t5_dn7 = assign68640_e105268_d_n7;
        locals.var_t5_dn8 = assign68640_e105268_d_n8;
        locals.var_t5_dn9 = assign68640_e105268_d_n9;
        locals.var_t5_dn10 = assign68640_e105268_d_n10;
        locals.var_t5_dn11 = assign68640_e105268_d_n11;
        locals.var_t5_dn14 = assign68640_e105268_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign68650_e105276, assign68650_e105276_d_n0, assign68650_e105276_d_n2, assign68650_e105276_d_n4, assign68650_e105276_d_n5, assign68650_e105276_d_n6, assign68650_e105276_d_n7, assign68650_e105276_d_n8, assign68650_e105276_d_n9, assign68650_e105276_d_n10, assign68650_e105276_d_n11, assign68650_e105276_d_n14,) = {
    if (locals.var_guard1605 != 0.0) {
        let assign68650_e105273: f64 = (locals.var_etun + locals.var_tmf2);
        let assign68650_e105274: f64 = (0.5 * assign68650_e105273);
        (assign68650_e105274, (0.5 * (locals.var_etun_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_etun_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_etun_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_etun_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_etun_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_etun_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_etun_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_etun_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_etun_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_etun_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_etun_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_etun, locals.var_etun_dn0, locals.var_etun_dn2, locals.var_etun_dn4, locals.var_etun_dn5, locals.var_etun_dn6, locals.var_etun_dn7, locals.var_etun_dn8, locals.var_etun_dn9, locals.var_etun_dn10, locals.var_etun_dn11, locals.var_etun_dn14,)
    }
};
        locals.var_etun = assign68650_e105276;
        locals.var_etun_dn0 = assign68650_e105276_d_n0;
        locals.var_etun_dn2 = assign68650_e105276_d_n2;
        locals.var_etun_dn4 = assign68650_e105276_d_n4;
        locals.var_etun_dn5 = assign68650_e105276_d_n5;
        locals.var_etun_dn6 = assign68650_e105276_d_n6;
        locals.var_etun_dn7 = assign68650_e105276_d_n7;
        locals.var_etun_dn8 = assign68650_e105276_d_n8;
        locals.var_etun_dn9 = assign68650_e105276_d_n9;
        locals.var_etun_dn10 = assign68650_e105276_d_n10;
        locals.var_etun_dn11 = assign68650_e105276_d_n11;
        locals.var_etun_dn14 = assign68650_e105276_d_n14;
        locals.var_etun_rv = 0.0;

        let assign68660_e105279: f64 = if locals.var_etun < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1612 = assign68660_e105279;
        locals.var_guard1612_rv = 0.0;

        let (assign68670_e105285, assign68670_e105285_d_n0, assign68670_e105285_d_n2, assign68670_e105285_d_n4, assign68670_e105285_d_n5, assign68670_e105285_d_n6, assign68670_e105285_d_n7, assign68670_e105285_d_n8, assign68670_e105285_d_n9, assign68670_e105285_d_n10, assign68670_e105285_d_n11, assign68670_e105285_d_n14,) = {
    if ((locals.var_guard1605 != 0.0) && (locals.var_guard1612 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_etun, locals.var_etun_dn0, locals.var_etun_dn2, locals.var_etun_dn4, locals.var_etun_dn5, locals.var_etun_dn6, locals.var_etun_dn7, locals.var_etun_dn8, locals.var_etun_dn9, locals.var_etun_dn10, locals.var_etun_dn11, locals.var_etun_dn14,)
    }
};
        locals.var_etun = assign68670_e105285;
        locals.var_etun_dn0 = assign68670_e105285_d_n0;
        locals.var_etun_dn2 = assign68670_e105285_d_n2;
        locals.var_etun_dn4 = assign68670_e105285_d_n4;
        locals.var_etun_dn5 = assign68670_e105285_d_n5;
        locals.var_etun_dn6 = assign68670_e105285_d_n6;
        locals.var_etun_dn7 = assign68670_e105285_d_n7;
        locals.var_etun_dn8 = assign68670_e105285_d_n8;
        locals.var_etun_dn9 = assign68670_e105285_d_n9;
        locals.var_etun_dn10 = assign68670_e105285_d_n10;
        locals.var_etun_dn11 = assign68670_e105285_d_n11;
        locals.var_etun_dn14 = assign68670_e105285_d_n14;
        locals.var_etun_rv = 0.0;

        let (assign68680_e105291, assign68680_e105291_d_n0, assign68680_e105291_d_n2, assign68680_e105291_d_n4, assign68680_e105291_d_n5, assign68680_e105291_d_n6, assign68680_e105291_d_n7, assign68680_e105291_d_n8, assign68680_e105291_d_n9, assign68680_e105291_d_n10, assign68680_e105291_d_n11, assign68680_e105291_d_n14,) = {
    if ((locals.var_guard1605 != 0.0) && (locals.var_guard1612 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign68680_e105291;
        locals.var_t5_dn0 = assign68680_e105291_d_n0;
        locals.var_t5_dn2 = assign68680_e105291_d_n2;
        locals.var_t5_dn4 = assign68680_e105291_d_n4;
        locals.var_t5_dn5 = assign68680_e105291_d_n5;
        locals.var_t5_dn6 = assign68680_e105291_d_n6;
        locals.var_t5_dn7 = assign68680_e105291_d_n7;
        locals.var_t5_dn8 = assign68680_e105291_d_n8;
        locals.var_t5_dn9 = assign68680_e105291_d_n9;
        locals.var_t5_dn10 = assign68680_e105291_d_n10;
        locals.var_t5_dn11 = assign68680_e105291_d_n11;
        locals.var_t5_dn14 = assign68680_e105291_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign68690_e105297, assign68690_e105297_d_n0, assign68690_e105297_d_n2, assign68690_e105297_d_n4, assign68690_e105297_d_n5, assign68690_e105297_d_n6, assign68690_e105297_d_n7, assign68690_e105297_d_n8, assign68690_e105297_d_n9, assign68690_e105297_d_n10, assign68690_e105297_d_n11, assign68690_e105297_d_n14,) = {
    if (locals.var_guard1605 != 0.0) {
        let assign68690_e105295: f64 = (locals.var_etun + 1e-25);
        (assign68690_e105295, locals.var_etun_dn0, locals.var_etun_dn2, locals.var_etun_dn4, locals.var_etun_dn5, locals.var_etun_dn6, locals.var_etun_dn7, locals.var_etun_dn8, locals.var_etun_dn9, locals.var_etun_dn10, locals.var_etun_dn11, locals.var_etun_dn14,)
    } else {
        (locals.var_etun, locals.var_etun_dn0, locals.var_etun_dn2, locals.var_etun_dn4, locals.var_etun_dn5, locals.var_etun_dn6, locals.var_etun_dn7, locals.var_etun_dn8, locals.var_etun_dn9, locals.var_etun_dn10, locals.var_etun_dn11, locals.var_etun_dn14,)
    }
};
        locals.var_etun = assign68690_e105297;
        locals.var_etun_dn0 = assign68690_e105297_d_n0;
        locals.var_etun_dn2 = assign68690_e105297_d_n2;
        locals.var_etun_dn4 = assign68690_e105297_d_n4;
        locals.var_etun_dn5 = assign68690_e105297_d_n5;
        locals.var_etun_dn6 = assign68690_e105297_d_n6;
        locals.var_etun_dn7 = assign68690_e105297_d_n7;
        locals.var_etun_dn8 = assign68690_e105297_d_n8;
        locals.var_etun_dn9 = assign68690_e105297_d_n9;
        locals.var_etun_dn10 = assign68690_e105297_d_n10;
        locals.var_etun_dn11 = assign68690_e105297_d_n11;
        locals.var_etun_dn14 = assign68690_e105297_d_n14;
        locals.var_etun_rv = 0.0;

        let (assign68700_e105304, assign68700_e105304_d_n0, assign68700_e105304_d_n2, assign68700_e105304_d_n4, assign68700_e105304_d_n5, assign68700_e105304_d_n6, assign68700_e105304_d_n7, assign68700_e105304_d_n8, assign68700_e105304_d_n9, assign68700_e105304_d_n10, assign68700_e105304_d_n11, assign68700_e105304_d_n14,) = {
    if (locals.var_guard1605 != 0.0) {
        let assign68700_e105300: f64 = (-locals.var_uc_glkb2);
        let assign68700_e105302: f64 = (assign68700_e105300 / locals.var_etun);
        (assign68700_e105302, (-((assign68700_e105300 * locals.var_etun_dn0) / (locals.var_etun * locals.var_etun))), (-((assign68700_e105300 * locals.var_etun_dn2) / (locals.var_etun * locals.var_etun))), (-((assign68700_e105300 * locals.var_etun_dn4) / (locals.var_etun * locals.var_etun))), (-((assign68700_e105300 * locals.var_etun_dn5) / (locals.var_etun * locals.var_etun))), (-((assign68700_e105300 * locals.var_etun_dn6) / (locals.var_etun * locals.var_etun))), (-((assign68700_e105300 * locals.var_etun_dn7) / (locals.var_etun * locals.var_etun))), (-((assign68700_e105300 * locals.var_etun_dn8) / (locals.var_etun * locals.var_etun))), (-((assign68700_e105300 * locals.var_etun_dn9) / (locals.var_etun * locals.var_etun))), (-((assign68700_e105300 * locals.var_etun_dn10) / (locals.var_etun * locals.var_etun))), (-((assign68700_e105300 * locals.var_etun_dn11) / (locals.var_etun * locals.var_etun))), (-((assign68700_e105300 * locals.var_etun_dn14) / (locals.var_etun * locals.var_etun))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign68700_e105304;
        locals.var_t1_dn0 = assign68700_e105304_d_n0;
        locals.var_t1_dn2 = assign68700_e105304_d_n2;
        locals.var_t1_dn4 = assign68700_e105304_d_n4;
        locals.var_t1_dn5 = assign68700_e105304_d_n5;
        locals.var_t1_dn6 = assign68700_e105304_d_n6;
        locals.var_t1_dn7 = assign68700_e105304_d_n7;
        locals.var_t1_dn8 = assign68700_e105304_d_n8;
        locals.var_t1_dn9 = assign68700_e105304_d_n9;
        locals.var_t1_dn10 = assign68700_e105304_d_n10;
        locals.var_t1_dn11 = assign68700_e105304_d_n11;
        locals.var_t1_dn14 = assign68700_e105304_d_n14;
        locals.var_t1_rv = 0.0;

        let assign68710_e105307: f64 = (-34.0);
        let assign68710_e105308: f64 = if locals.var_t1 < assign68710_e105307 { 1.0 } else { 0.0 };
        locals.var_guard1613 = assign68710_e105308;
        locals.var_guard1613_rv = 0.0;

        let (assign68730_e105322, assign68730_e105322_d_n0, assign68730_e105322_d_n2, assign68730_e105322_d_n4, assign68730_e105322_d_n5, assign68730_e105322_d_n6, assign68730_e105322_d_n7, assign68730_e105322_d_n8, assign68730_e105322_d_n9, assign68730_e105322_d_n10, assign68730_e105322_d_n11, assign68730_e105322_d_n14,) = {
    if ((locals.var_guard1605 != 0.0) && (locals.var_guard1613 == 0.0)) {
        let assign68730_e105320: f64 = (locals.var_t1).exp();
        (assign68730_e105320, (assign68730_e105320 * locals.var_t1_dn0), (assign68730_e105320 * locals.var_t1_dn2), (assign68730_e105320 * locals.var_t1_dn4), (assign68730_e105320 * locals.var_t1_dn5), (assign68730_e105320 * locals.var_t1_dn6), (assign68730_e105320 * locals.var_t1_dn7), (assign68730_e105320 * locals.var_t1_dn8), (assign68730_e105320 * locals.var_t1_dn9), (assign68730_e105320 * locals.var_t1_dn10), (assign68730_e105320 * locals.var_t1_dn11), (assign68730_e105320 * locals.var_t1_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign68730_e105322;
        locals.var_t2_dn0 = assign68730_e105322_d_n0;
        locals.var_t2_dn2 = assign68730_e105322_d_n2;
        locals.var_t2_dn4 = assign68730_e105322_d_n4;
        locals.var_t2_dn5 = assign68730_e105322_d_n5;
        locals.var_t2_dn6 = assign68730_e105322_d_n6;
        locals.var_t2_dn7 = assign68730_e105322_d_n7;
        locals.var_t2_dn8 = assign68730_e105322_d_n8;
        locals.var_t2_dn9 = assign68730_e105322_d_n9;
        locals.var_t2_dn10 = assign68730_e105322_d_n10;
        locals.var_t2_dn11 = assign68730_e105322_d_n11;
        locals.var_t2_dn14 = assign68730_e105322_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign68740_e105335, assign68740_e105335_d_n0, assign68740_e105335_d_n2, assign68740_e105335_d_n4, assign68740_e105335_d_n5, assign68740_e105335_d_n6, assign68740_e105335_d_n7, assign68740_e105335_d_n8, assign68740_e105335_d_n9, assign68740_e105335_d_n10, assign68740_e105335_d_n11, assign68740_e105335_d_n14,) = {
    if ((locals.var_guard1605 != 0.0) && (locals.var_guard1613 == 0.0)) {
        let assign68740_e105330: f64 = (locals.var_etun * locals.var_etun);
        let assign68740_e105331: f64 = (locals.var_uc_glkb2 / assign68740_e105330);
        let assign68740_e105333: f64 = (assign68740_e105331 * locals.var_t2);
        (assign68740_e105333, (((-((locals.var_uc_glkb2 * ((locals.var_etun_dn0 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn0))) / (assign68740_e105330 * assign68740_e105330))) * locals.var_t2) + (assign68740_e105331 * locals.var_t2_dn0)), (((-((locals.var_uc_glkb2 * ((locals.var_etun_dn2 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn2))) / (assign68740_e105330 * assign68740_e105330))) * locals.var_t2) + (assign68740_e105331 * locals.var_t2_dn2)), (((-((locals.var_uc_glkb2 * ((locals.var_etun_dn4 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn4))) / (assign68740_e105330 * assign68740_e105330))) * locals.var_t2) + (assign68740_e105331 * locals.var_t2_dn4)), (((-((locals.var_uc_glkb2 * ((locals.var_etun_dn5 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn5))) / (assign68740_e105330 * assign68740_e105330))) * locals.var_t2) + (assign68740_e105331 * locals.var_t2_dn5)), (((-((locals.var_uc_glkb2 * ((locals.var_etun_dn6 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn6))) / (assign68740_e105330 * assign68740_e105330))) * locals.var_t2) + (assign68740_e105331 * locals.var_t2_dn6)), (((-((locals.var_uc_glkb2 * ((locals.var_etun_dn7 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn7))) / (assign68740_e105330 * assign68740_e105330))) * locals.var_t2) + (assign68740_e105331 * locals.var_t2_dn7)), (((-((locals.var_uc_glkb2 * ((locals.var_etun_dn8 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn8))) / (assign68740_e105330 * assign68740_e105330))) * locals.var_t2) + (assign68740_e105331 * locals.var_t2_dn8)), (((-((locals.var_uc_glkb2 * ((locals.var_etun_dn9 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn9))) / (assign68740_e105330 * assign68740_e105330))) * locals.var_t2) + (assign68740_e105331 * locals.var_t2_dn9)), (((-((locals.var_uc_glkb2 * ((locals.var_etun_dn10 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn10))) / (assign68740_e105330 * assign68740_e105330))) * locals.var_t2) + (assign68740_e105331 * locals.var_t2_dn10)), (((-((locals.var_uc_glkb2 * ((locals.var_etun_dn11 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn11))) / (assign68740_e105330 * assign68740_e105330))) * locals.var_t2) + (assign68740_e105331 * locals.var_t2_dn11)), (((-((locals.var_uc_glkb2 * ((locals.var_etun_dn14 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn14))) / (assign68740_e105330 * assign68740_e105330))) * locals.var_t2) + (assign68740_e105331 * locals.var_t2_dn14)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign68740_e105335;
        locals.var_t3_dn0 = assign68740_e105335_d_n0;
        locals.var_t3_dn2 = assign68740_e105335_d_n2;
        locals.var_t3_dn4 = assign68740_e105335_d_n4;
        locals.var_t3_dn5 = assign68740_e105335_d_n5;
        locals.var_t3_dn6 = assign68740_e105335_d_n6;
        locals.var_t3_dn7 = assign68740_e105335_d_n7;
        locals.var_t3_dn8 = assign68740_e105335_d_n8;
        locals.var_t3_dn9 = assign68740_e105335_d_n9;
        locals.var_t3_dn10 = assign68740_e105335_d_n10;
        locals.var_t3_dn11 = assign68740_e105335_d_n11;
        locals.var_t3_dn14 = assign68740_e105335_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign68750_e105346, assign68750_e105346_d_n0, assign68750_e105346_d_n2, assign68750_e105346_d_n4, assign68750_e105346_d_n5, assign68750_e105346_d_n6, assign68750_e105346_d_n7, assign68750_e105346_d_n8, assign68750_e105346_d_n9, assign68750_e105346_d_n10, assign68750_e105346_d_n11, assign68750_e105346_d_n14,) = {
    if ((locals.var_guard1605 != 0.0) && (locals.var_guard1613 == 0.0)) {
        let assign68750_e105342: f64 = (locals.var_uc_glkb1 * locals.var_weff_nf);
        let assign68750_e105344: f64 = (assign68750_e105342 * locals.var_leff);
        (assign68750_e105344, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign68750_e105346;
        locals.var_t3_dn0 = assign68750_e105346_d_n0;
        locals.var_t3_dn2 = assign68750_e105346_d_n2;
        locals.var_t3_dn4 = assign68750_e105346_d_n4;
        locals.var_t3_dn5 = assign68750_e105346_d_n5;
        locals.var_t3_dn6 = assign68750_e105346_d_n6;
        locals.var_t3_dn7 = assign68750_e105346_d_n7;
        locals.var_t3_dn8 = assign68750_e105346_d_n8;
        locals.var_t3_dn9 = assign68750_e105346_d_n9;
        locals.var_t3_dn10 = assign68750_e105346_d_n10;
        locals.var_t3_dn11 = assign68750_e105346_d_n11;
        locals.var_t3_dn14 = assign68750_e105346_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign68770_e105363, assign68770_e105363_d_n0, assign68770_e105363_d_n2, assign68770_e105363_d_n4, assign68770_e105363_d_n5, assign68770_e105363_d_n6, assign68770_e105363_d_n7, assign68770_e105363_d_n8, assign68770_e105363_d_n9, assign68770_e105363_d_n10, assign68770_e105363_d_n11, assign68770_e105363_d_n14,) = {
    if (locals.var_guard1605 != 0.0) {
        (locals.var_sqrt_eg, locals.var_sqrt_eg_dn0, locals.var_sqrt_eg_dn2, locals.var_sqrt_eg_dn4, locals.var_sqrt_eg_dn5, locals.var_sqrt_eg_dn6, locals.var_sqrt_eg_dn7, locals.var_sqrt_eg_dn8, locals.var_sqrt_eg_dn9, locals.var_sqrt_eg_dn10, locals.var_sqrt_eg_dn11, locals.var_sqrt_eg_dn14,)
    } else {
        (locals.var_eg12, locals.var_eg12_dn0, locals.var_eg12_dn2, locals.var_eg12_dn4, locals.var_eg12_dn5, locals.var_eg12_dn6, locals.var_eg12_dn7, locals.var_eg12_dn8, locals.var_eg12_dn9, locals.var_eg12_dn10, locals.var_eg12_dn11, locals.var_eg12_dn14,)
    }
};
        locals.var_eg12 = assign68770_e105363;
        locals.var_eg12_dn0 = assign68770_e105363_d_n0;
        locals.var_eg12_dn2 = assign68770_e105363_d_n2;
        locals.var_eg12_dn4 = assign68770_e105363_d_n4;
        locals.var_eg12_dn5 = assign68770_e105363_d_n5;
        locals.var_eg12_dn6 = assign68770_e105363_d_n6;
        locals.var_eg12_dn7 = assign68770_e105363_d_n7;
        locals.var_eg12_dn8 = assign68770_e105363_d_n8;
        locals.var_eg12_dn9 = assign68770_e105363_d_n9;
        locals.var_eg12_dn10 = assign68770_e105363_d_n10;
        locals.var_eg12_dn11 = assign68770_e105363_d_n11;
        locals.var_eg12_dn14 = assign68770_e105363_d_n14;
        locals.var_eg12_rv = 0.0;

        let (assign68780_e105369, assign68780_e105369_d_n0, assign68780_e105369_d_n2, assign68780_e105369_d_n4, assign68780_e105369_d_n5, assign68780_e105369_d_n6, assign68780_e105369_d_n7, assign68780_e105369_d_n8, assign68780_e105369_d_n9, assign68780_e105369_d_n10, assign68780_e105369_d_n11, assign68780_e105369_d_n14,) = {
    if (locals.var_guard1605 != 0.0) {
        let assign68780_e105367: f64 = (locals.var_eg * locals.var_eg12);
        (assign68780_e105367, ((locals.var_eg_dn0 * locals.var_eg12) + (locals.var_eg * locals.var_eg12_dn0)), ((locals.var_eg_dn2 * locals.var_eg12) + (locals.var_eg * locals.var_eg12_dn2)), ((locals.var_eg_dn4 * locals.var_eg12) + (locals.var_eg * locals.var_eg12_dn4)), ((locals.var_eg_dn5 * locals.var_eg12) + (locals.var_eg * locals.var_eg12_dn5)), ((locals.var_eg_dn6 * locals.var_eg12) + (locals.var_eg * locals.var_eg12_dn6)), ((locals.var_eg_dn7 * locals.var_eg12) + (locals.var_eg * locals.var_eg12_dn7)), ((locals.var_eg_dn8 * locals.var_eg12) + (locals.var_eg * locals.var_eg12_dn8)), ((locals.var_eg_dn9 * locals.var_eg12) + (locals.var_eg * locals.var_eg12_dn9)), ((locals.var_eg_dn10 * locals.var_eg12) + (locals.var_eg * locals.var_eg12_dn10)), ((locals.var_eg_dn11 * locals.var_eg12) + (locals.var_eg * locals.var_eg12_dn11)), ((locals.var_eg_dn14 * locals.var_eg12) + (locals.var_eg * locals.var_eg12_dn14)),)
    } else {
        (locals.var_eg32, locals.var_eg32_dn0, locals.var_eg32_dn2, locals.var_eg32_dn4, locals.var_eg32_dn5, locals.var_eg32_dn6, locals.var_eg32_dn7, locals.var_eg32_dn8, locals.var_eg32_dn9, locals.var_eg32_dn10, locals.var_eg32_dn11, locals.var_eg32_dn14,)
    }
};
        locals.var_eg32 = assign68780_e105369;
        locals.var_eg32_dn0 = assign68780_e105369_d_n0;
        locals.var_eg32_dn2 = assign68780_e105369_d_n2;
        locals.var_eg32_dn4 = assign68780_e105369_d_n4;
        locals.var_eg32_dn5 = assign68780_e105369_d_n5;
        locals.var_eg32_dn6 = assign68780_e105369_d_n6;
        locals.var_eg32_dn7 = assign68780_e105369_d_n7;
        locals.var_eg32_dn8 = assign68780_e105369_d_n8;
        locals.var_eg32_dn9 = assign68780_e105369_d_n9;
        locals.var_eg32_dn10 = assign68780_e105369_d_n10;
        locals.var_eg32_dn11 = assign68780_e105369_d_n11;
        locals.var_eg32_dn14 = assign68780_e105369_d_n14;
        locals.var_eg32_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_259(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign68790_e105386, assign68790_e105386_d_n0, assign68790_e105386_d_n2, assign68790_e105386_d_n4, assign68790_e105386_d_n5, assign68790_e105386_d_n6, assign68790_e105386_d_n7, assign68790_e105386_d_n8, assign68790_e105386_d_n9, assign68790_e105386_d_n10, assign68790_e105386_d_n11, assign68790_e105386_d_n14,) = {
    if (locals.var_guard1605 != 0.0) {
        let assign68790_e105373: f64 = (locals.var_uc_fvbs * locals.var_vbsz__blk440);
        let assign68790_e105375: f64 = (assign68790_e105373 - locals.var_vgsz__blk442);
        let assign68790_e105377: f64 = (assign68790_e105375 + locals.var_dvthsc);
        let assign68790_e105379: f64 = (assign68790_e105377 + locals.var_dvthlp);
        let assign68790_e105381: f64 = (assign68790_e105379 - locals.var_uc_fn3);
        let assign68790_e105382: f64 = (-assign68790_e105381);
        let assign68790_e105384: f64 = (assign68790_e105382 / locals.var_tox0);
        (assign68790_e105384, ((-((((locals.var_uc_fvbs * locals.var_vbsz__blk440_dn0) - locals.var_vgsz__blk442_dn0) + locals.var_dvthsc_dn0) + locals.var_dvthlp_dn0)) / locals.var_tox0), ((-((((locals.var_uc_fvbs * locals.var_vbsz__blk440_dn2) - locals.var_vgsz__blk442_dn2) + locals.var_dvthsc_dn2) + locals.var_dvthlp_dn2)) / locals.var_tox0), ((-((((locals.var_uc_fvbs * locals.var_vbsz__blk440_dn4) - locals.var_vgsz__blk442_dn4) + locals.var_dvthsc_dn4) + locals.var_dvthlp_dn4)) / locals.var_tox0), ((-((((locals.var_uc_fvbs * locals.var_vbsz__blk440_dn5) - locals.var_vgsz__blk442_dn5) + locals.var_dvthsc_dn5) + locals.var_dvthlp_dn5)) / locals.var_tox0), ((-((((locals.var_uc_fvbs * locals.var_vbsz__blk440_dn6) - locals.var_vgsz__blk442_dn6) + locals.var_dvthsc_dn6) + locals.var_dvthlp_dn6)) / locals.var_tox0), ((-((((locals.var_uc_fvbs * locals.var_vbsz__blk440_dn7) - locals.var_vgsz__blk442_dn7) + locals.var_dvthsc_dn7) + locals.var_dvthlp_dn7)) / locals.var_tox0), ((-((((locals.var_uc_fvbs * locals.var_vbsz__blk440_dn8) - locals.var_vgsz__blk442_dn8) + locals.var_dvthsc_dn8) + locals.var_dvthlp_dn8)) / locals.var_tox0), ((-((((locals.var_uc_fvbs * locals.var_vbsz__blk440_dn9) - locals.var_vgsz__blk442_dn9) + locals.var_dvthsc_dn9) + locals.var_dvthlp_dn9)) / locals.var_tox0), ((-((((locals.var_uc_fvbs * locals.var_vbsz__blk440_dn10) - locals.var_vgsz__blk442_dn10) + locals.var_dvthsc_dn10) + locals.var_dvthlp_dn10)) / locals.var_tox0), ((-((((locals.var_uc_fvbs * locals.var_vbsz__blk440_dn11) - locals.var_vgsz__blk442_dn11) + locals.var_dvthsc_dn11) + locals.var_dvthlp_dn11)) / locals.var_tox0), ((-((((locals.var_uc_fvbs * locals.var_vbsz__blk440_dn14) - locals.var_vgsz__blk442_dn14) + locals.var_dvthsc_dn14) + locals.var_dvthlp_dn14)) / locals.var_tox0),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign68790_e105386;
        locals.var_t2_dn0 = assign68790_e105386_d_n0;
        locals.var_t2_dn2 = assign68790_e105386_d_n2;
        locals.var_t2_dn4 = assign68790_e105386_d_n4;
        locals.var_t2_dn5 = assign68790_e105386_d_n5;
        locals.var_t2_dn6 = assign68790_e105386_d_n6;
        locals.var_t2_dn7 = assign68790_e105386_d_n7;
        locals.var_t2_dn8 = assign68790_e105386_d_n8;
        locals.var_t2_dn9 = assign68790_e105386_d_n9;
        locals.var_t2_dn10 = assign68790_e105386_d_n10;
        locals.var_t2_dn11 = assign68790_e105386_d_n11;
        locals.var_t2_dn14 = assign68790_e105386_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign68800_e105392, assign68800_e105392_d_n0, assign68800_e105392_d_n2, assign68800_e105392_d_n4, assign68800_e105392_d_n5, assign68800_e105392_d_n6, assign68800_e105392_d_n7, assign68800_e105392_d_n8, assign68800_e105392_d_n9, assign68800_e105392_d_n10, assign68800_e105392_d_n11, assign68800_e105392_d_n14,) = {
    if (locals.var_guard1605 != 0.0) {
        let assign68800_e105390: f64 = (locals.var_t2 * locals.var_t2);
        (assign68800_e105390, ((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)), ((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)), ((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)), ((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)), ((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)), ((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)), ((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)), ((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)), ((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)), ((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)), ((locals.var_t2_dn14 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign68800_e105392;
        locals.var_t0_dn0 = assign68800_e105392_d_n0;
        locals.var_t0_dn2 = assign68800_e105392_d_n2;
        locals.var_t0_dn4 = assign68800_e105392_d_n4;
        locals.var_t0_dn5 = assign68800_e105392_d_n5;
        locals.var_t0_dn6 = assign68800_e105392_d_n6;
        locals.var_t0_dn7 = assign68800_e105392_d_n7;
        locals.var_t0_dn8 = assign68800_e105392_d_n8;
        locals.var_t0_dn9 = assign68800_e105392_d_n9;
        locals.var_t0_dn10 = assign68800_e105392_d_n10;
        locals.var_t0_dn11 = assign68800_e105392_d_n11;
        locals.var_t0_dn14 = assign68800_e105392_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign68810_e105398, assign68810_e105398_d_n0, assign68810_e105398_d_n2, assign68810_e105398_d_n4, assign68810_e105398_d_n5, assign68810_e105398_d_n6, assign68810_e105398_d_n7, assign68810_e105398_d_n8, assign68810_e105398_d_n9, assign68810_e105398_d_n10, assign68810_e105398_d_n11, assign68810_e105398_d_n14,) = {
    if (locals.var_guard1605 != 0.0) {
        let assign68810_e105396: f64 = (locals.var_uc_fn2 * locals.var_eg32);
        (assign68810_e105396, (locals.var_uc_fn2 * locals.var_eg32_dn0), (locals.var_uc_fn2 * locals.var_eg32_dn2), (locals.var_uc_fn2 * locals.var_eg32_dn4), (locals.var_uc_fn2 * locals.var_eg32_dn5), (locals.var_uc_fn2 * locals.var_eg32_dn6), (locals.var_uc_fn2 * locals.var_eg32_dn7), (locals.var_uc_fn2 * locals.var_eg32_dn8), (locals.var_uc_fn2 * locals.var_eg32_dn9), (locals.var_uc_fn2 * locals.var_eg32_dn10), (locals.var_uc_fn2 * locals.var_eg32_dn11), (locals.var_uc_fn2 * locals.var_eg32_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign68810_e105398;
        locals.var_t1_dn0 = assign68810_e105398_d_n0;
        locals.var_t1_dn2 = assign68810_e105398_d_n2;
        locals.var_t1_dn4 = assign68810_e105398_d_n4;
        locals.var_t1_dn5 = assign68810_e105398_d_n5;
        locals.var_t1_dn6 = assign68810_e105398_d_n6;
        locals.var_t1_dn7 = assign68810_e105398_d_n7;
        locals.var_t1_dn8 = assign68810_e105398_d_n8;
        locals.var_t1_dn9 = assign68810_e105398_d_n9;
        locals.var_t1_dn10 = assign68810_e105398_d_n10;
        locals.var_t1_dn11 = assign68810_e105398_d_n11;
        locals.var_t1_dn14 = assign68810_e105398_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign68820_e105405, assign68820_e105405_d_n0, assign68820_e105405_d_n2, assign68820_e105405_d_n4, assign68820_e105405_d_n5, assign68820_e105405_d_n6, assign68820_e105405_d_n7, assign68820_e105405_d_n8, assign68820_e105405_d_n9, assign68820_e105405_d_n10, assign68820_e105405_d_n11, assign68820_e105405_d_n14,) = {
    if (locals.var_guard1605 != 0.0) {
        let assign68820_e105401: f64 = (-locals.var_t1);
        let assign68820_e105403: f64 = (assign68820_e105401 / locals.var_t2);
        (assign68820_e105403, ((((-locals.var_t1_dn0) * locals.var_t2) - (assign68820_e105401 * locals.var_t2_dn0)) / (locals.var_t2 * locals.var_t2)), ((((-locals.var_t1_dn2) * locals.var_t2) - (assign68820_e105401 * locals.var_t2_dn2)) / (locals.var_t2 * locals.var_t2)), ((((-locals.var_t1_dn4) * locals.var_t2) - (assign68820_e105401 * locals.var_t2_dn4)) / (locals.var_t2 * locals.var_t2)), ((((-locals.var_t1_dn5) * locals.var_t2) - (assign68820_e105401 * locals.var_t2_dn5)) / (locals.var_t2 * locals.var_t2)), ((((-locals.var_t1_dn6) * locals.var_t2) - (assign68820_e105401 * locals.var_t2_dn6)) / (locals.var_t2 * locals.var_t2)), ((((-locals.var_t1_dn7) * locals.var_t2) - (assign68820_e105401 * locals.var_t2_dn7)) / (locals.var_t2 * locals.var_t2)), ((((-locals.var_t1_dn8) * locals.var_t2) - (assign68820_e105401 * locals.var_t2_dn8)) / (locals.var_t2 * locals.var_t2)), ((((-locals.var_t1_dn9) * locals.var_t2) - (assign68820_e105401 * locals.var_t2_dn9)) / (locals.var_t2 * locals.var_t2)), ((((-locals.var_t1_dn10) * locals.var_t2) - (assign68820_e105401 * locals.var_t2_dn10)) / (locals.var_t2 * locals.var_t2)), ((((-locals.var_t1_dn11) * locals.var_t2) - (assign68820_e105401 * locals.var_t2_dn11)) / (locals.var_t2 * locals.var_t2)), ((((-locals.var_t1_dn14) * locals.var_t2) - (assign68820_e105401 * locals.var_t2_dn14)) / (locals.var_t2 * locals.var_t2)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign68820_e105405;
        locals.var_t3_dn0 = assign68820_e105405_d_n0;
        locals.var_t3_dn2 = assign68820_e105405_d_n2;
        locals.var_t3_dn4 = assign68820_e105405_d_n4;
        locals.var_t3_dn5 = assign68820_e105405_d_n5;
        locals.var_t3_dn6 = assign68820_e105405_d_n6;
        locals.var_t3_dn7 = assign68820_e105405_d_n7;
        locals.var_t3_dn8 = assign68820_e105405_d_n8;
        locals.var_t3_dn9 = assign68820_e105405_d_n9;
        locals.var_t3_dn10 = assign68820_e105405_d_n10;
        locals.var_t3_dn11 = assign68820_e105405_d_n11;
        locals.var_t3_dn14 = assign68820_e105405_d_n14;
        locals.var_t3_rv = 0.0;

        let assign68830_e105408: f64 = (-34.0);
        let assign68830_e105409: f64 = if locals.var_t3 < assign68830_e105408 { 1.0 } else { 0.0 };
        locals.var_guard1614 = assign68830_e105409;
        locals.var_guard1614_rv = 0.0;

        let (assign68840_e105415, assign68840_e105415_d_n0, assign68840_e105415_d_n2, assign68840_e105415_d_n4, assign68840_e105415_d_n5, assign68840_e105415_d_n6, assign68840_e105415_d_n7, assign68840_e105415_d_n8, assign68840_e105415_d_n9, assign68840_e105415_d_n10, assign68840_e105415_d_n11, assign68840_e105415_d_n14,) = {
    if ((locals.var_guard1605 != 0.0) && (locals.var_guard1614 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign68840_e105415;
        locals.var_t5_dn0 = assign68840_e105415_d_n0;
        locals.var_t5_dn2 = assign68840_e105415_d_n2;
        locals.var_t5_dn4 = assign68840_e105415_d_n4;
        locals.var_t5_dn5 = assign68840_e105415_d_n5;
        locals.var_t5_dn6 = assign68840_e105415_d_n6;
        locals.var_t5_dn7 = assign68840_e105415_d_n7;
        locals.var_t5_dn8 = assign68840_e105415_d_n8;
        locals.var_t5_dn9 = assign68840_e105415_d_n9;
        locals.var_t5_dn10 = assign68840_e105415_d_n10;
        locals.var_t5_dn11 = assign68840_e105415_d_n11;
        locals.var_t5_dn14 = assign68840_e105415_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign68850_e105423, assign68850_e105423_d_n0, assign68850_e105423_d_n2, assign68850_e105423_d_n4, assign68850_e105423_d_n5, assign68850_e105423_d_n6, assign68850_e105423_d_n7, assign68850_e105423_d_n8, assign68850_e105423_d_n9, assign68850_e105423_d_n10, assign68850_e105423_d_n11, assign68850_e105423_d_n14,) = {
    if ((locals.var_guard1605 != 0.0) && (locals.var_guard1614 == 0.0)) {
        let assign68850_e105421: f64 = (locals.var_t3).exp();
        (assign68850_e105421, (assign68850_e105421 * locals.var_t3_dn0), (assign68850_e105421 * locals.var_t3_dn2), (assign68850_e105421 * locals.var_t3_dn4), (assign68850_e105421 * locals.var_t3_dn5), (assign68850_e105421 * locals.var_t3_dn6), (assign68850_e105421 * locals.var_t3_dn7), (assign68850_e105421 * locals.var_t3_dn8), (assign68850_e105421 * locals.var_t3_dn9), (assign68850_e105421 * locals.var_t3_dn10), (assign68850_e105421 * locals.var_t3_dn11), (assign68850_e105421 * locals.var_t3_dn14),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign68850_e105423;
        locals.var_t5_dn0 = assign68850_e105423_d_n0;
        locals.var_t5_dn2 = assign68850_e105423_d_n2;
        locals.var_t5_dn4 = assign68850_e105423_d_n4;
        locals.var_t5_dn5 = assign68850_e105423_d_n5;
        locals.var_t5_dn6 = assign68850_e105423_d_n6;
        locals.var_t5_dn7 = assign68850_e105423_d_n7;
        locals.var_t5_dn8 = assign68850_e105423_d_n8;
        locals.var_t5_dn9 = assign68850_e105423_d_n9;
        locals.var_t5_dn10 = assign68850_e105423_d_n10;
        locals.var_t5_dn11 = assign68850_e105423_d_n11;
        locals.var_t5_dn14 = assign68850_e105423_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign68860_e105435, assign68860_e105435_d_n0, assign68860_e105435_d_n2, assign68860_e105435_d_n4, assign68860_e105435_d_n5, assign68860_e105435_d_n6, assign68860_e105435_d_n7, assign68860_e105435_d_n8, assign68860_e105435_d_n9, assign68860_e105435_d_n10, assign68860_e105435_d_n11, assign68860_e105435_d_n14,) = {
    if (locals.var_guard1605 != 0.0) {
        let assign68860_e105427: f64 = (1.6021918e-19 * locals.var_uc_fn1);
        let assign68860_e105429: f64 = (assign68860_e105427 * locals.var_weff_nf);
        let assign68860_e105431: f64 = (assign68860_e105429 * locals.var_lgate);
        let assign68860_e105433: f64 = (assign68860_e105431 / locals.var_eg12);
        (assign68860_e105433, (-((assign68860_e105431 * locals.var_eg12_dn0) / (locals.var_eg12 * locals.var_eg12))), (-((assign68860_e105431 * locals.var_eg12_dn2) / (locals.var_eg12 * locals.var_eg12))), (-((assign68860_e105431 * locals.var_eg12_dn4) / (locals.var_eg12 * locals.var_eg12))), (-((assign68860_e105431 * locals.var_eg12_dn5) / (locals.var_eg12 * locals.var_eg12))), (-((assign68860_e105431 * locals.var_eg12_dn6) / (locals.var_eg12 * locals.var_eg12))), (-((assign68860_e105431 * locals.var_eg12_dn7) / (locals.var_eg12 * locals.var_eg12))), (-((assign68860_e105431 * locals.var_eg12_dn8) / (locals.var_eg12 * locals.var_eg12))), (-((assign68860_e105431 * locals.var_eg12_dn9) / (locals.var_eg12 * locals.var_eg12))), (-((assign68860_e105431 * locals.var_eg12_dn10) / (locals.var_eg12 * locals.var_eg12))), (-((assign68860_e105431 * locals.var_eg12_dn11) / (locals.var_eg12 * locals.var_eg12))), (-((assign68860_e105431 * locals.var_eg12_dn14) / (locals.var_eg12 * locals.var_eg12))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign68860_e105435;
        locals.var_t4_dn0 = assign68860_e105435_d_n0;
        locals.var_t4_dn2 = assign68860_e105435_d_n2;
        locals.var_t4_dn4 = assign68860_e105435_d_n4;
        locals.var_t4_dn5 = assign68860_e105435_d_n5;
        locals.var_t4_dn6 = assign68860_e105435_d_n6;
        locals.var_t4_dn7 = assign68860_e105435_d_n7;
        locals.var_t4_dn8 = assign68860_e105435_d_n8;
        locals.var_t4_dn9 = assign68860_e105435_d_n9;
        locals.var_t4_dn10 = assign68860_e105435_d_n10;
        locals.var_t4_dn11 = assign68860_e105435_d_n11;
        locals.var_t4_dn14 = assign68860_e105435_d_n14;
        locals.var_t4_rv = 0.0;

        let assign68910_e105476: f64 = if p.p25 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1616 = assign68910_e105476;
        locals.var_guard1616_rv = 0.0;

        let (assign68920_e105488, assign68920_e105488_d_n0, assign68920_e105488_d_n2, assign68920_e105488_d_n4, assign68920_e105488_d_n5, assign68920_e105488_d_n6, assign68920_e105488_d_n7, assign68920_e105488_d_n8, assign68920_e105488_d_n9, assign68920_e105488_d_n10, assign68920_e105488_d_n11, assign68920_e105488_d_n14,) = {
    if (locals.var_guard1616 != 0.0) {
        let assign68920_e105482: f64 = (100.0 * locals.var_vds);
        let assign68920_e105483: f64 = (1.0 - assign68920_e105482);
        let assign68920_e105484: f64 = (locals.var_vds * assign68920_e105483);
        let assign68920_e105486: f64 = (assign68920_e105484 - 1e-5);
        (assign68920_e105486, ((locals.var_vds_dn0 * assign68920_e105483) + (locals.var_vds * (-(100.0 * locals.var_vds_dn0)))), ((locals.var_vds_dn2 * assign68920_e105483) + (locals.var_vds * (-(100.0 * locals.var_vds_dn2)))), ((locals.var_vds_dn4 * assign68920_e105483) + (locals.var_vds * (-(100.0 * locals.var_vds_dn4)))), ((locals.var_vds_dn5 * assign68920_e105483) + (locals.var_vds * (-(100.0 * locals.var_vds_dn5)))), ((locals.var_vds_dn6 * assign68920_e105483) + (locals.var_vds * (-(100.0 * locals.var_vds_dn6)))), ((locals.var_vds_dn7 * assign68920_e105483) + (locals.var_vds * (-(100.0 * locals.var_vds_dn7)))), ((locals.var_vds_dn8 * assign68920_e105483) + (locals.var_vds * (-(100.0 * locals.var_vds_dn8)))), ((locals.var_vds_dn9 * assign68920_e105483) + (locals.var_vds * (-(100.0 * locals.var_vds_dn9)))), ((locals.var_vds_dn10 * assign68920_e105483) + (locals.var_vds * (-(100.0 * locals.var_vds_dn10)))), ((locals.var_vds_dn11 * assign68920_e105483) + (locals.var_vds * (-(100.0 * locals.var_vds_dn11)))), ((locals.var_vds_dn14 * assign68920_e105483) + (locals.var_vds * (-(100.0 * locals.var_vds_dn14)))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign68920_e105488;
        locals.var_t1_dn0 = assign68920_e105488_d_n0;
        locals.var_t1_dn2 = assign68920_e105488_d_n2;
        locals.var_t1_dn4 = assign68920_e105488_d_n4;
        locals.var_t1_dn5 = assign68920_e105488_d_n5;
        locals.var_t1_dn6 = assign68920_e105488_d_n6;
        locals.var_t1_dn7 = assign68920_e105488_d_n7;
        locals.var_t1_dn8 = assign68920_e105488_d_n8;
        locals.var_t1_dn9 = assign68920_e105488_d_n9;
        locals.var_t1_dn10 = assign68920_e105488_d_n10;
        locals.var_t1_dn11 = assign68920_e105488_d_n11;
        locals.var_t1_dn14 = assign68920_e105488_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign68930_e105501, assign68930_e105501_d_n0, assign68930_e105501_d_n2, assign68930_e105501_d_n4, assign68930_e105501_d_n5, assign68930_e105501_d_n6, assign68930_e105501_d_n7, assign68930_e105501_d_n8, assign68930_e105501_d_n9, assign68930_e105501_d_n10, assign68930_e105501_d_n11, assign68930_e105501_d_n14,) = {
    if (locals.var_guard1616 != 0.0) {
        let assign68930_e105492: f64 = (locals.var_t1 * locals.var_t1);
        let assign68930_e105495: f64 = (4.0 * 1e-5);
        let assign68930_e105497: f64 = (assign68930_e105495 * locals.var_vds);
        let assign68930_e105498: f64 = (assign68930_e105492 + assign68930_e105497);
        let assign68930_e105499: f64 = (assign68930_e105498).sqrt();
        (assign68930_e105499, ((((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) + (assign68930_e105495 * locals.var_vds_dn0)) / (2.0 * assign68930_e105499)), ((((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) + (assign68930_e105495 * locals.var_vds_dn2)) / (2.0 * assign68930_e105499)), ((((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) + (assign68930_e105495 * locals.var_vds_dn4)) / (2.0 * assign68930_e105499)), ((((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) + (assign68930_e105495 * locals.var_vds_dn5)) / (2.0 * assign68930_e105499)), ((((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) + (assign68930_e105495 * locals.var_vds_dn6)) / (2.0 * assign68930_e105499)), ((((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) + (assign68930_e105495 * locals.var_vds_dn7)) / (2.0 * assign68930_e105499)), ((((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) + (assign68930_e105495 * locals.var_vds_dn8)) / (2.0 * assign68930_e105499)), ((((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) + (assign68930_e105495 * locals.var_vds_dn9)) / (2.0 * assign68930_e105499)), ((((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) + (assign68930_e105495 * locals.var_vds_dn10)) / (2.0 * assign68930_e105499)), ((((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) + (assign68930_e105495 * locals.var_vds_dn11)) / (2.0 * assign68930_e105499)), ((((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14)) + (assign68930_e105495 * locals.var_vds_dn14)) / (2.0 * assign68930_e105499)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign68930_e105501;
        locals.var_t2_dn0 = assign68930_e105501_d_n0;
        locals.var_t2_dn2 = assign68930_e105501_d_n2;
        locals.var_t2_dn4 = assign68930_e105501_d_n4;
        locals.var_t2_dn5 = assign68930_e105501_d_n5;
        locals.var_t2_dn6 = assign68930_e105501_d_n6;
        locals.var_t2_dn7 = assign68930_e105501_d_n7;
        locals.var_t2_dn8 = assign68930_e105501_d_n8;
        locals.var_t2_dn9 = assign68930_e105501_d_n9;
        locals.var_t2_dn10 = assign68930_e105501_d_n10;
        locals.var_t2_dn11 = assign68930_e105501_d_n11;
        locals.var_t2_dn14 = assign68930_e105501_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign68940_e105511, assign68940_e105511_d_n0, assign68940_e105511_d_n2, assign68940_e105511_d_n4, assign68940_e105511_d_n5, assign68940_e105511_d_n6, assign68940_e105511_d_n7, assign68940_e105511_d_n8, assign68940_e105511_d_n9, assign68940_e105511_d_n10, assign68940_e105511_d_n11, assign68940_e105511_d_n14,) = {
    if (locals.var_guard1616 != 0.0) {
        let assign68940_e105507: f64 = (locals.var_t1 + locals.var_t2);
        let assign68940_e105508: f64 = (0.5 * assign68940_e105507);
        let assign68940_e105509: f64 = (locals.var_vds - assign68940_e105508);
        (assign68940_e105509, (locals.var_vds_dn0 - (0.5 * (locals.var_t1_dn0 + locals.var_t2_dn0))), (locals.var_vds_dn2 - (0.5 * (locals.var_t1_dn2 + locals.var_t2_dn2))), (locals.var_vds_dn4 - (0.5 * (locals.var_t1_dn4 + locals.var_t2_dn4))), (locals.var_vds_dn5 - (0.5 * (locals.var_t1_dn5 + locals.var_t2_dn5))), (locals.var_vds_dn6 - (0.5 * (locals.var_t1_dn6 + locals.var_t2_dn6))), (locals.var_vds_dn7 - (0.5 * (locals.var_t1_dn7 + locals.var_t2_dn7))), (locals.var_vds_dn8 - (0.5 * (locals.var_t1_dn8 + locals.var_t2_dn8))), (locals.var_vds_dn9 - (0.5 * (locals.var_t1_dn9 + locals.var_t2_dn9))), (locals.var_vds_dn10 - (0.5 * (locals.var_t1_dn10 + locals.var_t2_dn10))), (locals.var_vds_dn11 - (0.5 * (locals.var_t1_dn11 + locals.var_t2_dn11))), (locals.var_vds_dn14 - (0.5 * (locals.var_t1_dn14 + locals.var_t2_dn14))),)
    } else {
        (locals.var_vdsp, locals.var_vdsp_dn0, locals.var_vdsp_dn2, locals.var_vdsp_dn4, locals.var_vdsp_dn5, locals.var_vdsp_dn6, locals.var_vdsp_dn7, locals.var_vdsp_dn8, locals.var_vdsp_dn9, locals.var_vdsp_dn10, locals.var_vdsp_dn11, locals.var_vdsp_dn14,)
    }
};
        locals.var_vdsp = assign68940_e105511;
        locals.var_vdsp_dn0 = assign68940_e105511_d_n0;
        locals.var_vdsp_dn2 = assign68940_e105511_d_n2;
        locals.var_vdsp_dn4 = assign68940_e105511_d_n4;
        locals.var_vdsp_dn5 = assign68940_e105511_d_n5;
        locals.var_vdsp_dn6 = assign68940_e105511_d_n6;
        locals.var_vdsp_dn7 = assign68940_e105511_d_n7;
        locals.var_vdsp_dn8 = assign68940_e105511_d_n8;
        locals.var_vdsp_dn9 = assign68940_e105511_d_n9;
        locals.var_vdsp_dn10 = assign68940_e105511_d_n10;
        locals.var_vdsp_dn11 = assign68940_e105511_d_n11;
        locals.var_vdsp_dn14 = assign68940_e105511_d_n14;
        locals.var_vdsp_rv = 0.0;

        let assign68950_e105514: f64 = if p.p25 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1617 = assign68950_e105514;
        locals.var_guard1617_rv = 0.0;

        let (assign68970_e105535, assign68970_e105535_d_n0, assign68970_e105535_d_n2, assign68970_e105535_d_n4, assign68970_e105535_d_n5, assign68970_e105535_d_n6, assign68970_e105535_d_n7, assign68970_e105535_d_n8, assign68970_e105535_d_n9, assign68970_e105535_d_n10, assign68970_e105535_d_n11, assign68970_e105535_d_n14,) = {
    if (locals.var_guard1617 == 0.0) {
        let assign68970_e105524: f64 = (locals.var_vdsp + p.p243);
        let assign68970_e105525: f64 = (p.p242 * assign68970_e105524);
        let assign68970_e105527: f64 = (assign68970_e105525 - locals.var_vgs);
        let assign68970_e105530: f64 = (locals.var_dvthsc + locals.var_dvthlp);
        let assign68970_e105532: f64 = (assign68970_e105530 * p.p244);
        let assign68970_e105533: f64 = (assign68970_e105527 + assign68970_e105532);
        (assign68970_e105533, ((p.p242 * locals.var_vdsp_dn0) + ((locals.var_dvthsc_dn0 + locals.var_dvthlp_dn0) * p.p244)), ((p.p242 * locals.var_vdsp_dn2) + ((locals.var_dvthsc_dn2 + locals.var_dvthlp_dn2) * p.p244)), ((p.p242 * locals.var_vdsp_dn4) + ((locals.var_dvthsc_dn4 + locals.var_dvthlp_dn4) * p.p244)), ((p.p242 * locals.var_vdsp_dn5) + ((locals.var_dvthsc_dn5 + locals.var_dvthlp_dn5) * p.p244)), (((p.p242 * locals.var_vdsp_dn6) - locals.var_vgs_dn6) + ((locals.var_dvthsc_dn6 + locals.var_dvthlp_dn6) * p.p244)), (((p.p242 * locals.var_vdsp_dn7) - locals.var_vgs_dn7) + ((locals.var_dvthsc_dn7 + locals.var_dvthlp_dn7) * p.p244)), (((p.p242 * locals.var_vdsp_dn8) - locals.var_vgs_dn8) + ((locals.var_dvthsc_dn8 + locals.var_dvthlp_dn8) * p.p244)), ((p.p242 * locals.var_vdsp_dn9) + ((locals.var_dvthsc_dn9 + locals.var_dvthlp_dn9) * p.p244)), ((p.p242 * locals.var_vdsp_dn10) + ((locals.var_dvthsc_dn10 + locals.var_dvthlp_dn10) * p.p244)), ((p.p242 * locals.var_vdsp_dn11) + ((locals.var_dvthsc_dn11 + locals.var_dvthlp_dn11) * p.p244)), ((p.p242 * locals.var_vdsp_dn14) + ((locals.var_dvthsc_dn14 + locals.var_dvthlp_dn14) * p.p244)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign68970_e105535;
        locals.var_t1_dn0 = assign68970_e105535_d_n0;
        locals.var_t1_dn2 = assign68970_e105535_d_n2;
        locals.var_t1_dn4 = assign68970_e105535_d_n4;
        locals.var_t1_dn5 = assign68970_e105535_d_n5;
        locals.var_t1_dn6 = assign68970_e105535_d_n6;
        locals.var_t1_dn7 = assign68970_e105535_d_n7;
        locals.var_t1_dn8 = assign68970_e105535_d_n8;
        locals.var_t1_dn9 = assign68970_e105535_d_n9;
        locals.var_t1_dn10 = assign68970_e105535_d_n10;
        locals.var_t1_dn11 = assign68970_e105535_d_n11;
        locals.var_t1_dn14 = assign68970_e105535_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign68980_e105542, assign68980_e105542_d_n0, assign68980_e105542_d_n2, assign68980_e105542_d_n4, assign68980_e105542_d_n5, assign68980_e105542_d_n6, assign68980_e105542_d_n7, assign68980_e105542_d_n8, assign68980_e105542_d_n9, assign68980_e105542_d_n10, assign68980_e105542_d_n11, assign68980_e105542_d_n14,) = {
    if (locals.var_guard1617 == 0.0) {
        let assign68980_e105540: f64 = (1.0 / locals.var_tox0);
        (assign68980_e105540, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign68980_e105542;
        locals.var_t2_dn0 = assign68980_e105542_d_n0;
        locals.var_t2_dn2 = assign68980_e105542_d_n2;
        locals.var_t2_dn4 = assign68980_e105542_d_n4;
        locals.var_t2_dn5 = assign68980_e105542_d_n5;
        locals.var_t2_dn6 = assign68980_e105542_d_n6;
        locals.var_t2_dn7 = assign68980_e105542_d_n7;
        locals.var_t2_dn8 = assign68980_e105542_d_n8;
        locals.var_t2_dn9 = assign68980_e105542_d_n9;
        locals.var_t2_dn10 = assign68980_e105542_d_n10;
        locals.var_t2_dn11 = assign68980_e105542_d_n11;
        locals.var_t2_dn14 = assign68980_e105542_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign68990_e105549, assign68990_e105549_d_n0, assign68990_e105549_d_n2, assign68990_e105549_d_n4, assign68990_e105549_d_n5, assign68990_e105549_d_n6, assign68990_e105549_d_n7, assign68990_e105549_d_n8, assign68990_e105549_d_n9, assign68990_e105549_d_n10, assign68990_e105549_d_n11, assign68990_e105549_d_n14,) = {
    if (locals.var_guard1617 == 0.0) {
        let assign68990_e105547: f64 = (locals.var_t1 * locals.var_t2);
        (assign68990_e105547, ((locals.var_t1_dn0 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn0)), ((locals.var_t1_dn2 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn2)), ((locals.var_t1_dn4 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn4)), ((locals.var_t1_dn5 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn5)), ((locals.var_t1_dn6 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn6)), ((locals.var_t1_dn7 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn7)), ((locals.var_t1_dn8 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn8)), ((locals.var_t1_dn9 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn9)), ((locals.var_t1_dn10 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn10)), ((locals.var_t1_dn11 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn11)), ((locals.var_t1_dn14 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn14)),)
    } else {
        (locals.var_e1, locals.var_e1_dn0, locals.var_e1_dn2, locals.var_e1_dn4, locals.var_e1_dn5, locals.var_e1_dn6, locals.var_e1_dn7, locals.var_e1_dn8, locals.var_e1_dn9, locals.var_e1_dn10, locals.var_e1_dn11, locals.var_e1_dn14,)
    }
};
        locals.var_e1 = assign68990_e105549;
        locals.var_e1_dn0 = assign68990_e105549_d_n0;
        locals.var_e1_dn2 = assign68990_e105549_d_n2;
        locals.var_e1_dn4 = assign68990_e105549_d_n4;
        locals.var_e1_dn5 = assign68990_e105549_d_n5;
        locals.var_e1_dn6 = assign68990_e105549_d_n6;
        locals.var_e1_dn7 = assign68990_e105549_d_n7;
        locals.var_e1_dn8 = assign68990_e105549_d_n8;
        locals.var_e1_dn9 = assign68990_e105549_d_n9;
        locals.var_e1_dn10 = assign68990_e105549_d_n10;
        locals.var_e1_dn11 = assign68990_e105549_d_n11;
        locals.var_e1_dn14 = assign68990_e105549_d_n14;
        locals.var_e1_rv = 0.0;

        let (assign69000_e105567, assign69000_e105567_d_n0, assign69000_e105567_d_n2, assign69000_e105567_d_n4, assign69000_e105567_d_n5, assign69000_e105567_d_n6, assign69000_e105567_d_n7, assign69000_e105567_d_n8, assign69000_e105567_d_n9, assign69000_e105567_d_n10, assign69000_e105567_d_n11, assign69000_e105567_d_n14,) = {
    if (locals.var_guard1617 == 0.0) {
        let assign69000_e105554: f64 = (locals.var_e1 * locals.var_e1);
        let assign69000_e105558: f64 = (0.01 / 0.01);
        let assign69000_e105559: f64 = (4.0 * assign69000_e105558);
        let assign69000_e105562: f64 = (0.01 / 0.01);
        let assign69000_e105563: f64 = (assign69000_e105559 * assign69000_e105562);
        let assign69000_e105564: f64 = (assign69000_e105554 + assign69000_e105563);
        let assign69000_e105565: f64 = (assign69000_e105564).sqrt();
        (assign69000_e105565, (((locals.var_e1_dn0 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn0)) / (2.0 * assign69000_e105565)), (((locals.var_e1_dn2 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn2)) / (2.0 * assign69000_e105565)), (((locals.var_e1_dn4 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn4)) / (2.0 * assign69000_e105565)), (((locals.var_e1_dn5 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn5)) / (2.0 * assign69000_e105565)), (((locals.var_e1_dn6 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn6)) / (2.0 * assign69000_e105565)), (((locals.var_e1_dn7 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn7)) / (2.0 * assign69000_e105565)), (((locals.var_e1_dn8 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn8)) / (2.0 * assign69000_e105565)), (((locals.var_e1_dn9 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn9)) / (2.0 * assign69000_e105565)), (((locals.var_e1_dn10 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn10)) / (2.0 * assign69000_e105565)), (((locals.var_e1_dn11 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn11)) / (2.0 * assign69000_e105565)), (((locals.var_e1_dn14 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn14)) / (2.0 * assign69000_e105565)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign69000_e105567;
        locals.var_tmf2_dn0 = assign69000_e105567_d_n0;
        locals.var_tmf2_dn2 = assign69000_e105567_d_n2;
        locals.var_tmf2_dn4 = assign69000_e105567_d_n4;
        locals.var_tmf2_dn5 = assign69000_e105567_d_n5;
        locals.var_tmf2_dn6 = assign69000_e105567_d_n6;
        locals.var_tmf2_dn7 = assign69000_e105567_d_n7;
        locals.var_tmf2_dn8 = assign69000_e105567_d_n8;
        locals.var_tmf2_dn9 = assign69000_e105567_d_n9;
        locals.var_tmf2_dn10 = assign69000_e105567_d_n10;
        locals.var_tmf2_dn11 = assign69000_e105567_d_n11;
        locals.var_tmf2_dn14 = assign69000_e105567_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign69010_e105578, assign69010_e105578_d_n0, assign69010_e105578_d_n2, assign69010_e105578_d_n4, assign69010_e105578_d_n5, assign69010_e105578_d_n6, assign69010_e105578_d_n7, assign69010_e105578_d_n8, assign69010_e105578_d_n9, assign69010_e105578_d_n10, assign69010_e105578_d_n11, assign69010_e105578_d_n14,) = {
    if (locals.var_guard1617 == 0.0) {
        let assign69010_e105574: f64 = (locals.var_e1 / locals.var_tmf2);
        let assign69010_e105575: f64 = (1.0 + assign69010_e105574);
        let assign69010_e105576: f64 = (0.5 * assign69010_e105575);
        (assign69010_e105576, (0.5 * (((locals.var_e1_dn0 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_e1_dn2 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_e1_dn4 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_e1_dn5 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_e1_dn6 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_e1_dn7 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_e1_dn8 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_e1_dn9 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_e1_dn10 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_e1_dn11 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_e1_dn14 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign69010_e105578;
        locals.var_t5_dn0 = assign69010_e105578_d_n0;
        locals.var_t5_dn2 = assign69010_e105578_d_n2;
        locals.var_t5_dn4 = assign69010_e105578_d_n4;
        locals.var_t5_dn5 = assign69010_e105578_d_n5;
        locals.var_t5_dn6 = assign69010_e105578_d_n6;
        locals.var_t5_dn7 = assign69010_e105578_d_n7;
        locals.var_t5_dn8 = assign69010_e105578_d_n8;
        locals.var_t5_dn9 = assign69010_e105578_d_n9;
        locals.var_t5_dn10 = assign69010_e105578_d_n10;
        locals.var_t5_dn11 = assign69010_e105578_d_n11;
        locals.var_t5_dn14 = assign69010_e105578_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign69020_e105587, assign69020_e105587_d_n0, assign69020_e105587_d_n2, assign69020_e105587_d_n4, assign69020_e105587_d_n5, assign69020_e105587_d_n6, assign69020_e105587_d_n7, assign69020_e105587_d_n8, assign69020_e105587_d_n9, assign69020_e105587_d_n10, assign69020_e105587_d_n11, assign69020_e105587_d_n14,) = {
    if (locals.var_guard1617 == 0.0) {
        let assign69020_e105584: f64 = (locals.var_e1 + locals.var_tmf2);
        let assign69020_e105585: f64 = (0.5 * assign69020_e105584);
        (assign69020_e105585, (0.5 * (locals.var_e1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_e1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_e1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_e1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_e1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_e1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_e1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_e1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_e1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_e1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_e1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_egidl, locals.var_egidl_dn0, locals.var_egidl_dn2, locals.var_egidl_dn4, locals.var_egidl_dn5, locals.var_egidl_dn6, locals.var_egidl_dn7, locals.var_egidl_dn8, locals.var_egidl_dn9, locals.var_egidl_dn10, locals.var_egidl_dn11, locals.var_egidl_dn14,)
    }
};
        locals.var_egidl = assign69020_e105587;
        locals.var_egidl_dn0 = assign69020_e105587_d_n0;
        locals.var_egidl_dn2 = assign69020_e105587_d_n2;
        locals.var_egidl_dn4 = assign69020_e105587_d_n4;
        locals.var_egidl_dn5 = assign69020_e105587_d_n5;
        locals.var_egidl_dn6 = assign69020_e105587_d_n6;
        locals.var_egidl_dn7 = assign69020_e105587_d_n7;
        locals.var_egidl_dn8 = assign69020_e105587_d_n8;
        locals.var_egidl_dn9 = assign69020_e105587_d_n9;
        locals.var_egidl_dn10 = assign69020_e105587_d_n10;
        locals.var_egidl_dn11 = assign69020_e105587_d_n11;
        locals.var_egidl_dn14 = assign69020_e105587_d_n14;
        locals.var_egidl_rv = 0.0;

        let assign69030_e105590: f64 = if locals.var_egidl < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1618 = assign69030_e105590;
        locals.var_guard1618_rv = 0.0;

        let (assign69040_e105597, assign69040_e105597_d_n0, assign69040_e105597_d_n2, assign69040_e105597_d_n4, assign69040_e105597_d_n5, assign69040_e105597_d_n6, assign69040_e105597_d_n7, assign69040_e105597_d_n8, assign69040_e105597_d_n9, assign69040_e105597_d_n10, assign69040_e105597_d_n11, assign69040_e105597_d_n14,) = {
    if ((locals.var_guard1617 == 0.0) && (locals.var_guard1618 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_egidl, locals.var_egidl_dn0, locals.var_egidl_dn2, locals.var_egidl_dn4, locals.var_egidl_dn5, locals.var_egidl_dn6, locals.var_egidl_dn7, locals.var_egidl_dn8, locals.var_egidl_dn9, locals.var_egidl_dn10, locals.var_egidl_dn11, locals.var_egidl_dn14,)
    }
};
        locals.var_egidl = assign69040_e105597;
        locals.var_egidl_dn0 = assign69040_e105597_d_n0;
        locals.var_egidl_dn2 = assign69040_e105597_d_n2;
        locals.var_egidl_dn4 = assign69040_e105597_d_n4;
        locals.var_egidl_dn5 = assign69040_e105597_d_n5;
        locals.var_egidl_dn6 = assign69040_e105597_d_n6;
        locals.var_egidl_dn7 = assign69040_e105597_d_n7;
        locals.var_egidl_dn8 = assign69040_e105597_d_n8;
        locals.var_egidl_dn9 = assign69040_e105597_d_n9;
        locals.var_egidl_dn10 = assign69040_e105597_d_n10;
        locals.var_egidl_dn11 = assign69040_e105597_d_n11;
        locals.var_egidl_dn14 = assign69040_e105597_d_n14;
        locals.var_egidl_rv = 0.0;

        let (assign69050_e105604, assign69050_e105604_d_n0, assign69050_e105604_d_n2, assign69050_e105604_d_n4, assign69050_e105604_d_n5, assign69050_e105604_d_n6, assign69050_e105604_d_n7, assign69050_e105604_d_n8, assign69050_e105604_d_n9, assign69050_e105604_d_n10, assign69050_e105604_d_n11, assign69050_e105604_d_n14,) = {
    if ((locals.var_guard1617 == 0.0) && (locals.var_guard1618 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign69050_e105604;
        locals.var_t5_dn0 = assign69050_e105604_d_n0;
        locals.var_t5_dn2 = assign69050_e105604_d_n2;
        locals.var_t5_dn4 = assign69050_e105604_d_n4;
        locals.var_t5_dn5 = assign69050_e105604_d_n5;
        locals.var_t5_dn6 = assign69050_e105604_d_n6;
        locals.var_t5_dn7 = assign69050_e105604_d_n7;
        locals.var_t5_dn8 = assign69050_e105604_d_n8;
        locals.var_t5_dn9 = assign69050_e105604_d_n9;
        locals.var_t5_dn10 = assign69050_e105604_d_n10;
        locals.var_t5_dn11 = assign69050_e105604_d_n11;
        locals.var_t5_dn14 = assign69050_e105604_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign69060_e105613, assign69060_e105613_d_n0, assign69060_e105613_d_n2, assign69060_e105613_d_n4, assign69060_e105613_d_n5, assign69060_e105613_d_n6, assign69060_e105613_d_n7, assign69060_e105613_d_n8, assign69060_e105613_d_n9, assign69060_e105613_d_n10, assign69060_e105613_d_n11, assign69060_e105613_d_n14,) = {
    if (locals.var_guard1617 == 0.0) {
        let assign69060_e105610: f64 = (locals.var_egidl + 1e-25);
        let assign69060_e105611: f64 = (1.0 / assign69060_e105610);
        (assign69060_e105611, (-(locals.var_egidl_dn0 / (assign69060_e105610 * assign69060_e105610))), (-(locals.var_egidl_dn2 / (assign69060_e105610 * assign69060_e105610))), (-(locals.var_egidl_dn4 / (assign69060_e105610 * assign69060_e105610))), (-(locals.var_egidl_dn5 / (assign69060_e105610 * assign69060_e105610))), (-(locals.var_egidl_dn6 / (assign69060_e105610 * assign69060_e105610))), (-(locals.var_egidl_dn7 / (assign69060_e105610 * assign69060_e105610))), (-(locals.var_egidl_dn8 / (assign69060_e105610 * assign69060_e105610))), (-(locals.var_egidl_dn9 / (assign69060_e105610 * assign69060_e105610))), (-(locals.var_egidl_dn10 / (assign69060_e105610 * assign69060_e105610))), (-(locals.var_egidl_dn11 / (assign69060_e105610 * assign69060_e105610))), (-(locals.var_egidl_dn14 / (assign69060_e105610 * assign69060_e105610))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign69060_e105613;
        locals.var_t3_dn0 = assign69060_e105613_d_n0;
        locals.var_t3_dn2 = assign69060_e105613_d_n2;
        locals.var_t3_dn4 = assign69060_e105613_d_n4;
        locals.var_t3_dn5 = assign69060_e105613_d_n5;
        locals.var_t3_dn6 = assign69060_e105613_d_n6;
        locals.var_t3_dn7 = assign69060_e105613_d_n7;
        locals.var_t3_dn8 = assign69060_e105613_d_n8;
        locals.var_t3_dn9 = assign69060_e105613_d_n9;
        locals.var_t3_dn10 = assign69060_e105613_d_n10;
        locals.var_t3_dn11 = assign69060_e105613_d_n11;
        locals.var_t3_dn14 = assign69060_e105613_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign69070_e105623, assign69070_e105623_d_n0, assign69070_e105623_d_n2, assign69070_e105623_d_n4, assign69070_e105623_d_n5, assign69070_e105623_d_n6, assign69070_e105623_d_n7, assign69070_e105623_d_n8, assign69070_e105623_d_n9, assign69070_e105623_d_n10, assign69070_e105623_d_n11, assign69070_e105623_d_n14,) = {
    if (locals.var_guard1617 == 0.0) {
        let assign69070_e105617: f64 = (-locals.var_uc_gidl2);
        let assign69070_e105619: f64 = (assign69070_e105617 * locals.var_egp32);
        let assign69070_e105621: f64 = (assign69070_e105619 * locals.var_t3);
        (assign69070_e105621, (((assign69070_e105617 * locals.var_egp32_dn0) * locals.var_t3) + (assign69070_e105619 * locals.var_t3_dn0)), (((assign69070_e105617 * locals.var_egp32_dn2) * locals.var_t3) + (assign69070_e105619 * locals.var_t3_dn2)), (((assign69070_e105617 * locals.var_egp32_dn4) * locals.var_t3) + (assign69070_e105619 * locals.var_t3_dn4)), (((assign69070_e105617 * locals.var_egp32_dn5) * locals.var_t3) + (assign69070_e105619 * locals.var_t3_dn5)), (((assign69070_e105617 * locals.var_egp32_dn6) * locals.var_t3) + (assign69070_e105619 * locals.var_t3_dn6)), (((assign69070_e105617 * locals.var_egp32_dn7) * locals.var_t3) + (assign69070_e105619 * locals.var_t3_dn7)), (((assign69070_e105617 * locals.var_egp32_dn8) * locals.var_t3) + (assign69070_e105619 * locals.var_t3_dn8)), (((assign69070_e105617 * locals.var_egp32_dn9) * locals.var_t3) + (assign69070_e105619 * locals.var_t3_dn9)), (((assign69070_e105617 * locals.var_egp32_dn10) * locals.var_t3) + (assign69070_e105619 * locals.var_t3_dn10)), (((assign69070_e105617 * locals.var_egp32_dn11) * locals.var_t3) + (assign69070_e105619 * locals.var_t3_dn11)), (((assign69070_e105617 * locals.var_egp32_dn14) * locals.var_t3) + (assign69070_e105619 * locals.var_t3_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign69070_e105623;
        locals.var_t0_dn0 = assign69070_e105623_d_n0;
        locals.var_t0_dn2 = assign69070_e105623_d_n2;
        locals.var_t0_dn4 = assign69070_e105623_d_n4;
        locals.var_t0_dn5 = assign69070_e105623_d_n5;
        locals.var_t0_dn6 = assign69070_e105623_d_n6;
        locals.var_t0_dn7 = assign69070_e105623_d_n7;
        locals.var_t0_dn8 = assign69070_e105623_d_n8;
        locals.var_t0_dn9 = assign69070_e105623_d_n9;
        locals.var_t0_dn10 = assign69070_e105623_d_n10;
        locals.var_t0_dn11 = assign69070_e105623_d_n11;
        locals.var_t0_dn14 = assign69070_e105623_d_n14;
        locals.var_t0_rv = 0.0;

        let assign69080_e105626: f64 = (-34.0);
        let assign69080_e105627: f64 = if locals.var_t0 < assign69080_e105626 { 1.0 } else { 0.0 };
        locals.var_guard1619 = assign69080_e105627;
        locals.var_guard1619_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_260(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign69100_e105643, assign69100_e105643_d_n0, assign69100_e105643_d_n2, assign69100_e105643_d_n4, assign69100_e105643_d_n5, assign69100_e105643_d_n6, assign69100_e105643_d_n7, assign69100_e105643_d_n8, assign69100_e105643_d_n9, assign69100_e105643_d_n10, assign69100_e105643_d_n11, assign69100_e105643_d_n14,) = {
    if ((locals.var_guard1617 == 0.0) && (locals.var_guard1619 == 0.0)) {
        let assign69100_e105641: f64 = (locals.var_t0).exp();
        (assign69100_e105641, (assign69100_e105641 * locals.var_t0_dn0), (assign69100_e105641 * locals.var_t0_dn2), (assign69100_e105641 * locals.var_t0_dn4), (assign69100_e105641 * locals.var_t0_dn5), (assign69100_e105641 * locals.var_t0_dn6), (assign69100_e105641 * locals.var_t0_dn7), (assign69100_e105641 * locals.var_t0_dn8), (assign69100_e105641 * locals.var_t0_dn9), (assign69100_e105641 * locals.var_t0_dn10), (assign69100_e105641 * locals.var_t0_dn11), (assign69100_e105641 * locals.var_t0_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign69100_e105643;
        locals.var_t1_dn0 = assign69100_e105643_d_n0;
        locals.var_t1_dn2 = assign69100_e105643_d_n2;
        locals.var_t1_dn4 = assign69100_e105643_d_n4;
        locals.var_t1_dn5 = assign69100_e105643_d_n5;
        locals.var_t1_dn6 = assign69100_e105643_d_n6;
        locals.var_t1_dn7 = assign69100_e105643_d_n7;
        locals.var_t1_dn8 = assign69100_e105643_d_n8;
        locals.var_t1_dn9 = assign69100_e105643_d_n9;
        locals.var_t1_dn10 = assign69100_e105643_d_n10;
        locals.var_t1_dn11 = assign69100_e105643_d_n11;
        locals.var_t1_dn14 = assign69100_e105643_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign69110_e105657, assign69110_e105657_d_n0, assign69110_e105657_d_n2, assign69110_e105657_d_n4, assign69110_e105657_d_n5, assign69110_e105657_d_n6, assign69110_e105657_d_n7, assign69110_e105657_d_n8, assign69110_e105657_d_n9, assign69110_e105657_d_n10, assign69110_e105657_d_n11, assign69110_e105657_d_n14,) = {
    if ((locals.var_guard1617 == 0.0) && (locals.var_guard1619 == 0.0)) {
        let assign69110_e105651: f64 = (locals.var_uc_gidl1 / locals.var_egp12);
        let assign69110_e105653: f64 = (assign69110_e105651 * 1.6021918e-19);
        let assign69110_e105655: f64 = (assign69110_e105653 * locals.var_weff_nf);
        (assign69110_e105655, (((-((locals.var_uc_gidl1 * locals.var_egp12_dn0) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_weff_nf), (((-((locals.var_uc_gidl1 * locals.var_egp12_dn2) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_weff_nf), (((-((locals.var_uc_gidl1 * locals.var_egp12_dn4) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_weff_nf), (((-((locals.var_uc_gidl1 * locals.var_egp12_dn5) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_weff_nf), (((-((locals.var_uc_gidl1 * locals.var_egp12_dn6) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_weff_nf), (((-((locals.var_uc_gidl1 * locals.var_egp12_dn7) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_weff_nf), (((-((locals.var_uc_gidl1 * locals.var_egp12_dn8) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_weff_nf), (((-((locals.var_uc_gidl1 * locals.var_egp12_dn9) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_weff_nf), (((-((locals.var_uc_gidl1 * locals.var_egp12_dn10) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_weff_nf), (((-((locals.var_uc_gidl1 * locals.var_egp12_dn11) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_weff_nf), (((-((locals.var_uc_gidl1 * locals.var_egp12_dn14) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_weff_nf),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign69110_e105657;
        locals.var_t2_dn0 = assign69110_e105657_d_n0;
        locals.var_t2_dn2 = assign69110_e105657_d_n2;
        locals.var_t2_dn4 = assign69110_e105657_d_n4;
        locals.var_t2_dn5 = assign69110_e105657_d_n5;
        locals.var_t2_dn6 = assign69110_e105657_d_n6;
        locals.var_t2_dn7 = assign69110_e105657_d_n7;
        locals.var_t2_dn8 = assign69110_e105657_d_n8;
        locals.var_t2_dn9 = assign69110_e105657_d_n9;
        locals.var_t2_dn10 = assign69110_e105657_d_n10;
        locals.var_t2_dn11 = assign69110_e105657_d_n11;
        locals.var_t2_dn14 = assign69110_e105657_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign69130_e105678, assign69130_e105678_d_n0, assign69130_e105678_d_n2, assign69130_e105678_d_n4, assign69130_e105678_d_n5, assign69130_e105678_d_n6, assign69130_e105678_d_n7, assign69130_e105678_d_n8, assign69130_e105678_d_n9, assign69130_e105678_d_n10, assign69130_e105678_d_n11, assign69130_e105678_d_n14,) = {
    if (locals.var_guard1617 == 0.0) {
        let assign69130_e105676: f64 = (locals.var_vds - locals.var_vbs);
        (assign69130_e105676, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn4, locals.var_vds_dn5, (locals.var_vds_dn6 - locals.var_vbs_dn6), locals.var_vds_dn7, (locals.var_vds_dn8 - locals.var_vbs_dn8), (locals.var_vds_dn9 - locals.var_vbs_dn9), locals.var_vds_dn10, locals.var_vds_dn11, locals.var_vds_dn14,)
    } else {
        (locals.var_vdb, locals.var_vdb_dn0, locals.var_vdb_dn2, locals.var_vdb_dn4, locals.var_vdb_dn5, locals.var_vdb_dn6, locals.var_vdb_dn7, locals.var_vdb_dn8, locals.var_vdb_dn9, locals.var_vdb_dn10, locals.var_vdb_dn11, locals.var_vdb_dn14,)
    }
};
        locals.var_vdb = assign69130_e105678;
        locals.var_vdb_dn0 = assign69130_e105678_d_n0;
        locals.var_vdb_dn2 = assign69130_e105678_d_n2;
        locals.var_vdb_dn4 = assign69130_e105678_d_n4;
        locals.var_vdb_dn5 = assign69130_e105678_d_n5;
        locals.var_vdb_dn6 = assign69130_e105678_d_n6;
        locals.var_vdb_dn7 = assign69130_e105678_d_n7;
        locals.var_vdb_dn8 = assign69130_e105678_d_n8;
        locals.var_vdb_dn9 = assign69130_e105678_d_n9;
        locals.var_vdb_dn10 = assign69130_e105678_d_n10;
        locals.var_vdb_dn11 = assign69130_e105678_d_n11;
        locals.var_vdb_dn14 = assign69130_e105678_d_n14;
        locals.var_vdb_rv = 0.0;

        let assign69140_e105681: f64 = if locals.var_vdb > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1620 = assign69140_e105681;
        locals.var_guard1620_rv = 0.0;

        let (assign69150_e105690, assign69150_e105690_d_n0, assign69150_e105690_d_n2, assign69150_e105690_d_n4, assign69150_e105690_d_n5, assign69150_e105690_d_n6, assign69150_e105690_d_n7, assign69150_e105690_d_n8, assign69150_e105690_d_n9, assign69150_e105690_d_n10, assign69150_e105690_d_n11, assign69150_e105690_d_n14,) = {
    if ((locals.var_guard1617 == 0.0) && (locals.var_guard1620 != 0.0)) {
        let assign69150_e105688: f64 = (locals.var_vdb * locals.var_vdb);
        (assign69150_e105688, ((locals.var_vdb_dn0 * locals.var_vdb) + (locals.var_vdb * locals.var_vdb_dn0)), ((locals.var_vdb_dn2 * locals.var_vdb) + (locals.var_vdb * locals.var_vdb_dn2)), ((locals.var_vdb_dn4 * locals.var_vdb) + (locals.var_vdb * locals.var_vdb_dn4)), ((locals.var_vdb_dn5 * locals.var_vdb) + (locals.var_vdb * locals.var_vdb_dn5)), ((locals.var_vdb_dn6 * locals.var_vdb) + (locals.var_vdb * locals.var_vdb_dn6)), ((locals.var_vdb_dn7 * locals.var_vdb) + (locals.var_vdb * locals.var_vdb_dn7)), ((locals.var_vdb_dn8 * locals.var_vdb) + (locals.var_vdb * locals.var_vdb_dn8)), ((locals.var_vdb_dn9 * locals.var_vdb) + (locals.var_vdb * locals.var_vdb_dn9)), ((locals.var_vdb_dn10 * locals.var_vdb) + (locals.var_vdb * locals.var_vdb_dn10)), ((locals.var_vdb_dn11 * locals.var_vdb) + (locals.var_vdb * locals.var_vdb_dn11)), ((locals.var_vdb_dn14 * locals.var_vdb) + (locals.var_vdb * locals.var_vdb_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign69150_e105690;
        locals.var_t2_dn0 = assign69150_e105690_d_n0;
        locals.var_t2_dn2 = assign69150_e105690_d_n2;
        locals.var_t2_dn4 = assign69150_e105690_d_n4;
        locals.var_t2_dn5 = assign69150_e105690_d_n5;
        locals.var_t2_dn6 = assign69150_e105690_d_n6;
        locals.var_t2_dn7 = assign69150_e105690_d_n7;
        locals.var_t2_dn8 = assign69150_e105690_d_n8;
        locals.var_t2_dn9 = assign69150_e105690_d_n9;
        locals.var_t2_dn10 = assign69150_e105690_d_n10;
        locals.var_t2_dn11 = assign69150_e105690_d_n11;
        locals.var_t2_dn14 = assign69150_e105690_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign69160_e105699, assign69160_e105699_d_n0, assign69160_e105699_d_n2, assign69160_e105699_d_n4, assign69160_e105699_d_n5, assign69160_e105699_d_n6, assign69160_e105699_d_n7, assign69160_e105699_d_n8, assign69160_e105699_d_n9, assign69160_e105699_d_n10, assign69160_e105699_d_n11, assign69160_e105699_d_n14,) = {
    if ((locals.var_guard1617 == 0.0) && (locals.var_guard1620 != 0.0)) {
        let assign69160_e105697: f64 = (locals.var_t2 * locals.var_vdb);
        (assign69160_e105697, ((locals.var_t2_dn0 * locals.var_vdb) + (locals.var_t2 * locals.var_vdb_dn0)), ((locals.var_t2_dn2 * locals.var_vdb) + (locals.var_t2 * locals.var_vdb_dn2)), ((locals.var_t2_dn4 * locals.var_vdb) + (locals.var_t2 * locals.var_vdb_dn4)), ((locals.var_t2_dn5 * locals.var_vdb) + (locals.var_t2 * locals.var_vdb_dn5)), ((locals.var_t2_dn6 * locals.var_vdb) + (locals.var_t2 * locals.var_vdb_dn6)), ((locals.var_t2_dn7 * locals.var_vdb) + (locals.var_t2 * locals.var_vdb_dn7)), ((locals.var_t2_dn8 * locals.var_vdb) + (locals.var_t2 * locals.var_vdb_dn8)), ((locals.var_t2_dn9 * locals.var_vdb) + (locals.var_t2 * locals.var_vdb_dn9)), ((locals.var_t2_dn10 * locals.var_vdb) + (locals.var_t2 * locals.var_vdb_dn10)), ((locals.var_t2_dn11 * locals.var_vdb) + (locals.var_t2 * locals.var_vdb_dn11)), ((locals.var_t2_dn14 * locals.var_vdb) + (locals.var_t2 * locals.var_vdb_dn14)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign69160_e105699;
        locals.var_t4_dn0 = assign69160_e105699_d_n0;
        locals.var_t4_dn2 = assign69160_e105699_d_n2;
        locals.var_t4_dn4 = assign69160_e105699_d_n4;
        locals.var_t4_dn5 = assign69160_e105699_d_n5;
        locals.var_t4_dn6 = assign69160_e105699_d_n6;
        locals.var_t4_dn7 = assign69160_e105699_d_n7;
        locals.var_t4_dn8 = assign69160_e105699_d_n8;
        locals.var_t4_dn9 = assign69160_e105699_d_n9;
        locals.var_t4_dn10 = assign69160_e105699_d_n10;
        locals.var_t4_dn11 = assign69160_e105699_d_n11;
        locals.var_t4_dn14 = assign69160_e105699_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign69170_e105708, assign69170_e105708_d_n0, assign69170_e105708_d_n2, assign69170_e105708_d_n4, assign69170_e105708_d_n5, assign69170_e105708_d_n6, assign69170_e105708_d_n7, assign69170_e105708_d_n8, assign69170_e105708_d_n9, assign69170_e105708_d_n10, assign69170_e105708_d_n11, assign69170_e105708_d_n14,) = {
    if ((locals.var_guard1617 == 0.0) && (locals.var_guard1620 != 0.0)) {
        let assign69170_e105706: f64 = (locals.var_t4 + 0.5);
        (assign69170_e105706, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign69170_e105708;
        locals.var_t0_dn0 = assign69170_e105708_d_n0;
        locals.var_t0_dn2 = assign69170_e105708_d_n2;
        locals.var_t0_dn4 = assign69170_e105708_d_n4;
        locals.var_t0_dn5 = assign69170_e105708_d_n5;
        locals.var_t0_dn6 = assign69170_e105708_d_n6;
        locals.var_t0_dn7 = assign69170_e105708_d_n7;
        locals.var_t0_dn8 = assign69170_e105708_d_n8;
        locals.var_t0_dn9 = assign69170_e105708_d_n9;
        locals.var_t0_dn10 = assign69170_e105708_d_n10;
        locals.var_t0_dn11 = assign69170_e105708_d_n11;
        locals.var_t0_dn14 = assign69170_e105708_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign69180_e105717, assign69180_e105717_d_n0, assign69180_e105717_d_n2, assign69180_e105717_d_n4, assign69180_e105717_d_n5, assign69180_e105717_d_n6, assign69180_e105717_d_n7, assign69180_e105717_d_n8, assign69180_e105717_d_n9, assign69180_e105717_d_n10, assign69180_e105717_d_n11, assign69180_e105717_d_n14,) = {
    if ((locals.var_guard1617 == 0.0) && (locals.var_guard1620 != 0.0)) {
        let assign69180_e105715: f64 = (locals.var_t4 / locals.var_t0);
        (assign69180_e105715, (((locals.var_t4_dn0 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn0)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn2 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn2)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn4 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn5 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn6 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn7 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn8 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn9 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn9)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn10 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn11 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn11)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn14 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn14)) / (locals.var_t0 * locals.var_t0)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign69180_e105717;
        locals.var_t5_dn0 = assign69180_e105717_d_n0;
        locals.var_t5_dn2 = assign69180_e105717_d_n2;
        locals.var_t5_dn4 = assign69180_e105717_d_n4;
        locals.var_t5_dn5 = assign69180_e105717_d_n5;
        locals.var_t5_dn6 = assign69180_e105717_d_n6;
        locals.var_t5_dn7 = assign69180_e105717_d_n7;
        locals.var_t5_dn8 = assign69180_e105717_d_n8;
        locals.var_t5_dn9 = assign69180_e105717_d_n9;
        locals.var_t5_dn10 = assign69180_e105717_d_n10;
        locals.var_t5_dn11 = assign69180_e105717_d_n11;
        locals.var_t5_dn14 = assign69180_e105717_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign69190_e105738, assign69190_e105738_d_n0, assign69190_e105738_d_n2, assign69190_e105738_d_n4, assign69190_e105738_d_n5, assign69190_e105738_d_n6, assign69190_e105738_d_n7, assign69190_e105738_d_n8, assign69190_e105738_d_n9, assign69190_e105738_d_n10, assign69190_e105738_d_n11, assign69190_e105738_d_n14,) = {
    if ((locals.var_guard1617 == 0.0) && (locals.var_guard1620 != 0.0)) {
        let assign69190_e105724: f64 = (3.0 * locals.var_t2);
        let assign69190_e105726: f64 = (assign69190_e105724 * locals.var_t0);
        let assign69190_e105729: f64 = (locals.var_t4 * 3.0);
        let assign69190_e105731: f64 = (assign69190_e105729 * locals.var_t2);
        let assign69190_e105732: f64 = (assign69190_e105726 - assign69190_e105731);
        let assign69190_e105735: f64 = (locals.var_t0 * locals.var_t0);
        let assign69190_e105736: f64 = (assign69190_e105732 / assign69190_e105735);
        (assign69190_e105736, (((((((3.0 * locals.var_t2_dn0) * locals.var_t0) + (assign69190_e105724 * locals.var_t0_dn0)) - (((locals.var_t4_dn0 * 3.0) * locals.var_t2) + (assign69190_e105729 * locals.var_t2_dn0))) * assign69190_e105735) - (assign69190_e105732 * ((locals.var_t0_dn0 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn0)))) / (assign69190_e105735 * assign69190_e105735)), (((((((3.0 * locals.var_t2_dn2) * locals.var_t0) + (assign69190_e105724 * locals.var_t0_dn2)) - (((locals.var_t4_dn2 * 3.0) * locals.var_t2) + (assign69190_e105729 * locals.var_t2_dn2))) * assign69190_e105735) - (assign69190_e105732 * ((locals.var_t0_dn2 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn2)))) / (assign69190_e105735 * assign69190_e105735)), (((((((3.0 * locals.var_t2_dn4) * locals.var_t0) + (assign69190_e105724 * locals.var_t0_dn4)) - (((locals.var_t4_dn4 * 3.0) * locals.var_t2) + (assign69190_e105729 * locals.var_t2_dn4))) * assign69190_e105735) - (assign69190_e105732 * ((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)))) / (assign69190_e105735 * assign69190_e105735)), (((((((3.0 * locals.var_t2_dn5) * locals.var_t0) + (assign69190_e105724 * locals.var_t0_dn5)) - (((locals.var_t4_dn5 * 3.0) * locals.var_t2) + (assign69190_e105729 * locals.var_t2_dn5))) * assign69190_e105735) - (assign69190_e105732 * ((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)))) / (assign69190_e105735 * assign69190_e105735)), (((((((3.0 * locals.var_t2_dn6) * locals.var_t0) + (assign69190_e105724 * locals.var_t0_dn6)) - (((locals.var_t4_dn6 * 3.0) * locals.var_t2) + (assign69190_e105729 * locals.var_t2_dn6))) * assign69190_e105735) - (assign69190_e105732 * ((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)))) / (assign69190_e105735 * assign69190_e105735)), (((((((3.0 * locals.var_t2_dn7) * locals.var_t0) + (assign69190_e105724 * locals.var_t0_dn7)) - (((locals.var_t4_dn7 * 3.0) * locals.var_t2) + (assign69190_e105729 * locals.var_t2_dn7))) * assign69190_e105735) - (assign69190_e105732 * ((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)))) / (assign69190_e105735 * assign69190_e105735)), (((((((3.0 * locals.var_t2_dn8) * locals.var_t0) + (assign69190_e105724 * locals.var_t0_dn8)) - (((locals.var_t4_dn8 * 3.0) * locals.var_t2) + (assign69190_e105729 * locals.var_t2_dn8))) * assign69190_e105735) - (assign69190_e105732 * ((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)))) / (assign69190_e105735 * assign69190_e105735)), (((((((3.0 * locals.var_t2_dn9) * locals.var_t0) + (assign69190_e105724 * locals.var_t0_dn9)) - (((locals.var_t4_dn9 * 3.0) * locals.var_t2) + (assign69190_e105729 * locals.var_t2_dn9))) * assign69190_e105735) - (assign69190_e105732 * ((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)))) / (assign69190_e105735 * assign69190_e105735)), (((((((3.0 * locals.var_t2_dn10) * locals.var_t0) + (assign69190_e105724 * locals.var_t0_dn10)) - (((locals.var_t4_dn10 * 3.0) * locals.var_t2) + (assign69190_e105729 * locals.var_t2_dn10))) * assign69190_e105735) - (assign69190_e105732 * ((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)))) / (assign69190_e105735 * assign69190_e105735)), (((((((3.0 * locals.var_t2_dn11) * locals.var_t0) + (assign69190_e105724 * locals.var_t0_dn11)) - (((locals.var_t4_dn11 * 3.0) * locals.var_t2) + (assign69190_e105729 * locals.var_t2_dn11))) * assign69190_e105735) - (assign69190_e105732 * ((locals.var_t0_dn11 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn11)))) / (assign69190_e105735 * assign69190_e105735)), (((((((3.0 * locals.var_t2_dn14) * locals.var_t0) + (assign69190_e105724 * locals.var_t0_dn14)) - (((locals.var_t4_dn14 * 3.0) * locals.var_t2) + (assign69190_e105729 * locals.var_t2_dn14))) * assign69190_e105735) - (assign69190_e105732 * ((locals.var_t0_dn14 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn14)))) / (assign69190_e105735 * assign69190_e105735)),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign69190_e105738;
        locals.var_t7_dn0 = assign69190_e105738_d_n0;
        locals.var_t7_dn2 = assign69190_e105738_d_n2;
        locals.var_t7_dn4 = assign69190_e105738_d_n4;
        locals.var_t7_dn5 = assign69190_e105738_d_n5;
        locals.var_t7_dn6 = assign69190_e105738_d_n6;
        locals.var_t7_dn7 = assign69190_e105738_d_n7;
        locals.var_t7_dn8 = assign69190_e105738_d_n8;
        locals.var_t7_dn9 = assign69190_e105738_d_n9;
        locals.var_t7_dn10 = assign69190_e105738_d_n10;
        locals.var_t7_dn11 = assign69190_e105738_d_n11;
        locals.var_t7_dn14 = assign69190_e105738_d_n14;
        locals.var_t7_rv = 0.0;

        let assign69220_e105758: f64 = if p.p25 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1621 = assign69220_e105758;
        locals.var_guard1621_rv = 0.0;

        let (assign69240_e105782, assign69240_e105782_d_n0, assign69240_e105782_d_n2, assign69240_e105782_d_n4, assign69240_e105782_d_n5, assign69240_e105782_d_n6, assign69240_e105782_d_n7, assign69240_e105782_d_n8, assign69240_e105782_d_n9, assign69240_e105782_d_n10, assign69240_e105782_d_n11, assign69240_e105782_d_n14,) = {
    if (locals.var_guard1621 == 0.0) {
        let assign69240_e105767: f64 = (-locals.var_vdsp);
        let assign69240_e105769: f64 = (assign69240_e105767 + p.p243);
        let assign69240_e105770: f64 = (p.p242 * assign69240_e105769);
        let assign69240_e105773: f64 = (locals.var_vgs - locals.var_vdsp);
        let assign69240_e105774: f64 = (assign69240_e105770 - assign69240_e105773);
        let assign69240_e105777: f64 = (locals.var_dvthsc + locals.var_dvthlp);
        let assign69240_e105779: f64 = (assign69240_e105777 * p.p244);
        let assign69240_e105780: f64 = (assign69240_e105774 + assign69240_e105779);
        (assign69240_e105780, (((p.p242 * (-locals.var_vdsp_dn0)) - (-locals.var_vdsp_dn0)) + ((locals.var_dvthsc_dn0 + locals.var_dvthlp_dn0) * p.p244)), (((p.p242 * (-locals.var_vdsp_dn2)) - (-locals.var_vdsp_dn2)) + ((locals.var_dvthsc_dn2 + locals.var_dvthlp_dn2) * p.p244)), (((p.p242 * (-locals.var_vdsp_dn4)) - (-locals.var_vdsp_dn4)) + ((locals.var_dvthsc_dn4 + locals.var_dvthlp_dn4) * p.p244)), (((p.p242 * (-locals.var_vdsp_dn5)) - (-locals.var_vdsp_dn5)) + ((locals.var_dvthsc_dn5 + locals.var_dvthlp_dn5) * p.p244)), (((p.p242 * (-locals.var_vdsp_dn6)) - (locals.var_vgs_dn6 - locals.var_vdsp_dn6)) + ((locals.var_dvthsc_dn6 + locals.var_dvthlp_dn6) * p.p244)), (((p.p242 * (-locals.var_vdsp_dn7)) - (locals.var_vgs_dn7 - locals.var_vdsp_dn7)) + ((locals.var_dvthsc_dn7 + locals.var_dvthlp_dn7) * p.p244)), (((p.p242 * (-locals.var_vdsp_dn8)) - (locals.var_vgs_dn8 - locals.var_vdsp_dn8)) + ((locals.var_dvthsc_dn8 + locals.var_dvthlp_dn8) * p.p244)), (((p.p242 * (-locals.var_vdsp_dn9)) - (-locals.var_vdsp_dn9)) + ((locals.var_dvthsc_dn9 + locals.var_dvthlp_dn9) * p.p244)), (((p.p242 * (-locals.var_vdsp_dn10)) - (-locals.var_vdsp_dn10)) + ((locals.var_dvthsc_dn10 + locals.var_dvthlp_dn10) * p.p244)), (((p.p242 * (-locals.var_vdsp_dn11)) - (-locals.var_vdsp_dn11)) + ((locals.var_dvthsc_dn11 + locals.var_dvthlp_dn11) * p.p244)), (((p.p242 * (-locals.var_vdsp_dn14)) - (-locals.var_vdsp_dn14)) + ((locals.var_dvthsc_dn14 + locals.var_dvthlp_dn14) * p.p244)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign69240_e105782;
        locals.var_t1_dn0 = assign69240_e105782_d_n0;
        locals.var_t1_dn2 = assign69240_e105782_d_n2;
        locals.var_t1_dn4 = assign69240_e105782_d_n4;
        locals.var_t1_dn5 = assign69240_e105782_d_n5;
        locals.var_t1_dn6 = assign69240_e105782_d_n6;
        locals.var_t1_dn7 = assign69240_e105782_d_n7;
        locals.var_t1_dn8 = assign69240_e105782_d_n8;
        locals.var_t1_dn9 = assign69240_e105782_d_n9;
        locals.var_t1_dn10 = assign69240_e105782_d_n10;
        locals.var_t1_dn11 = assign69240_e105782_d_n11;
        locals.var_t1_dn14 = assign69240_e105782_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign69250_e105789, assign69250_e105789_d_n0, assign69250_e105789_d_n2, assign69250_e105789_d_n4, assign69250_e105789_d_n5, assign69250_e105789_d_n6, assign69250_e105789_d_n7, assign69250_e105789_d_n8, assign69250_e105789_d_n9, assign69250_e105789_d_n10, assign69250_e105789_d_n11, assign69250_e105789_d_n14,) = {
    if (locals.var_guard1621 == 0.0) {
        let assign69250_e105787: f64 = (1.0 / locals.var_tox0);
        (assign69250_e105787, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign69250_e105789;
        locals.var_t2_dn0 = assign69250_e105789_d_n0;
        locals.var_t2_dn2 = assign69250_e105789_d_n2;
        locals.var_t2_dn4 = assign69250_e105789_d_n4;
        locals.var_t2_dn5 = assign69250_e105789_d_n5;
        locals.var_t2_dn6 = assign69250_e105789_d_n6;
        locals.var_t2_dn7 = assign69250_e105789_d_n7;
        locals.var_t2_dn8 = assign69250_e105789_d_n8;
        locals.var_t2_dn9 = assign69250_e105789_d_n9;
        locals.var_t2_dn10 = assign69250_e105789_d_n10;
        locals.var_t2_dn11 = assign69250_e105789_d_n11;
        locals.var_t2_dn14 = assign69250_e105789_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign69260_e105796, assign69260_e105796_d_n0, assign69260_e105796_d_n2, assign69260_e105796_d_n4, assign69260_e105796_d_n5, assign69260_e105796_d_n6, assign69260_e105796_d_n7, assign69260_e105796_d_n8, assign69260_e105796_d_n9, assign69260_e105796_d_n10, assign69260_e105796_d_n11, assign69260_e105796_d_n14,) = {
    if (locals.var_guard1621 == 0.0) {
        let assign69260_e105794: f64 = (locals.var_t1 * locals.var_t2);
        (assign69260_e105794, ((locals.var_t1_dn0 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn0)), ((locals.var_t1_dn2 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn2)), ((locals.var_t1_dn4 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn4)), ((locals.var_t1_dn5 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn5)), ((locals.var_t1_dn6 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn6)), ((locals.var_t1_dn7 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn7)), ((locals.var_t1_dn8 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn8)), ((locals.var_t1_dn9 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn9)), ((locals.var_t1_dn10 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn10)), ((locals.var_t1_dn11 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn11)), ((locals.var_t1_dn14 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn14)),)
    } else {
        (locals.var_e1, locals.var_e1_dn0, locals.var_e1_dn2, locals.var_e1_dn4, locals.var_e1_dn5, locals.var_e1_dn6, locals.var_e1_dn7, locals.var_e1_dn8, locals.var_e1_dn9, locals.var_e1_dn10, locals.var_e1_dn11, locals.var_e1_dn14,)
    }
};
        locals.var_e1 = assign69260_e105796;
        locals.var_e1_dn0 = assign69260_e105796_d_n0;
        locals.var_e1_dn2 = assign69260_e105796_d_n2;
        locals.var_e1_dn4 = assign69260_e105796_d_n4;
        locals.var_e1_dn5 = assign69260_e105796_d_n5;
        locals.var_e1_dn6 = assign69260_e105796_d_n6;
        locals.var_e1_dn7 = assign69260_e105796_d_n7;
        locals.var_e1_dn8 = assign69260_e105796_d_n8;
        locals.var_e1_dn9 = assign69260_e105796_d_n9;
        locals.var_e1_dn10 = assign69260_e105796_d_n10;
        locals.var_e1_dn11 = assign69260_e105796_d_n11;
        locals.var_e1_dn14 = assign69260_e105796_d_n14;
        locals.var_e1_rv = 0.0;

        let (assign69270_e105814, assign69270_e105814_d_n0, assign69270_e105814_d_n2, assign69270_e105814_d_n4, assign69270_e105814_d_n5, assign69270_e105814_d_n6, assign69270_e105814_d_n7, assign69270_e105814_d_n8, assign69270_e105814_d_n9, assign69270_e105814_d_n10, assign69270_e105814_d_n11, assign69270_e105814_d_n14,) = {
    if (locals.var_guard1621 == 0.0) {
        let assign69270_e105801: f64 = (locals.var_e1 * locals.var_e1);
        let assign69270_e105805: f64 = (0.01 / 0.01);
        let assign69270_e105806: f64 = (4.0 * assign69270_e105805);
        let assign69270_e105809: f64 = (0.01 / 0.01);
        let assign69270_e105810: f64 = (assign69270_e105806 * assign69270_e105809);
        let assign69270_e105811: f64 = (assign69270_e105801 + assign69270_e105810);
        let assign69270_e105812: f64 = (assign69270_e105811).sqrt();
        (assign69270_e105812, (((locals.var_e1_dn0 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn0)) / (2.0 * assign69270_e105812)), (((locals.var_e1_dn2 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn2)) / (2.0 * assign69270_e105812)), (((locals.var_e1_dn4 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn4)) / (2.0 * assign69270_e105812)), (((locals.var_e1_dn5 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn5)) / (2.0 * assign69270_e105812)), (((locals.var_e1_dn6 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn6)) / (2.0 * assign69270_e105812)), (((locals.var_e1_dn7 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn7)) / (2.0 * assign69270_e105812)), (((locals.var_e1_dn8 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn8)) / (2.0 * assign69270_e105812)), (((locals.var_e1_dn9 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn9)) / (2.0 * assign69270_e105812)), (((locals.var_e1_dn10 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn10)) / (2.0 * assign69270_e105812)), (((locals.var_e1_dn11 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn11)) / (2.0 * assign69270_e105812)), (((locals.var_e1_dn14 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn14)) / (2.0 * assign69270_e105812)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign69270_e105814;
        locals.var_tmf2_dn0 = assign69270_e105814_d_n0;
        locals.var_tmf2_dn2 = assign69270_e105814_d_n2;
        locals.var_tmf2_dn4 = assign69270_e105814_d_n4;
        locals.var_tmf2_dn5 = assign69270_e105814_d_n5;
        locals.var_tmf2_dn6 = assign69270_e105814_d_n6;
        locals.var_tmf2_dn7 = assign69270_e105814_d_n7;
        locals.var_tmf2_dn8 = assign69270_e105814_d_n8;
        locals.var_tmf2_dn9 = assign69270_e105814_d_n9;
        locals.var_tmf2_dn10 = assign69270_e105814_d_n10;
        locals.var_tmf2_dn11 = assign69270_e105814_d_n11;
        locals.var_tmf2_dn14 = assign69270_e105814_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign69280_e105825, assign69280_e105825_d_n0, assign69280_e105825_d_n2, assign69280_e105825_d_n4, assign69280_e105825_d_n5, assign69280_e105825_d_n6, assign69280_e105825_d_n7, assign69280_e105825_d_n8, assign69280_e105825_d_n9, assign69280_e105825_d_n10, assign69280_e105825_d_n11, assign69280_e105825_d_n14,) = {
    if (locals.var_guard1621 == 0.0) {
        let assign69280_e105821: f64 = (locals.var_e1 / locals.var_tmf2);
        let assign69280_e105822: f64 = (1.0 + assign69280_e105821);
        let assign69280_e105823: f64 = (0.5 * assign69280_e105822);
        (assign69280_e105823, (0.5 * (((locals.var_e1_dn0 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_e1_dn2 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_e1_dn4 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_e1_dn5 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_e1_dn6 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_e1_dn7 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_e1_dn8 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_e1_dn9 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_e1_dn10 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_e1_dn11 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_e1_dn14 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign69280_e105825;
        locals.var_t5_dn0 = assign69280_e105825_d_n0;
        locals.var_t5_dn2 = assign69280_e105825_d_n2;
        locals.var_t5_dn4 = assign69280_e105825_d_n4;
        locals.var_t5_dn5 = assign69280_e105825_d_n5;
        locals.var_t5_dn6 = assign69280_e105825_d_n6;
        locals.var_t5_dn7 = assign69280_e105825_d_n7;
        locals.var_t5_dn8 = assign69280_e105825_d_n8;
        locals.var_t5_dn9 = assign69280_e105825_d_n9;
        locals.var_t5_dn10 = assign69280_e105825_d_n10;
        locals.var_t5_dn11 = assign69280_e105825_d_n11;
        locals.var_t5_dn14 = assign69280_e105825_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign69290_e105834, assign69290_e105834_d_n0, assign69290_e105834_d_n2, assign69290_e105834_d_n4, assign69290_e105834_d_n5, assign69290_e105834_d_n6, assign69290_e105834_d_n7, assign69290_e105834_d_n8, assign69290_e105834_d_n9, assign69290_e105834_d_n10, assign69290_e105834_d_n11, assign69290_e105834_d_n14,) = {
    if (locals.var_guard1621 == 0.0) {
        let assign69290_e105831: f64 = (locals.var_e1 + locals.var_tmf2);
        let assign69290_e105832: f64 = (0.5 * assign69290_e105831);
        (assign69290_e105832, (0.5 * (locals.var_e1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_e1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_e1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_e1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_e1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_e1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_e1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_e1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_e1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_e1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_e1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_egisl, locals.var_egisl_dn0, locals.var_egisl_dn2, locals.var_egisl_dn4, locals.var_egisl_dn5, locals.var_egisl_dn6, locals.var_egisl_dn7, locals.var_egisl_dn8, locals.var_egisl_dn9, locals.var_egisl_dn10, locals.var_egisl_dn11, locals.var_egisl_dn14,)
    }
};
        locals.var_egisl = assign69290_e105834;
        locals.var_egisl_dn0 = assign69290_e105834_d_n0;
        locals.var_egisl_dn2 = assign69290_e105834_d_n2;
        locals.var_egisl_dn4 = assign69290_e105834_d_n4;
        locals.var_egisl_dn5 = assign69290_e105834_d_n5;
        locals.var_egisl_dn6 = assign69290_e105834_d_n6;
        locals.var_egisl_dn7 = assign69290_e105834_d_n7;
        locals.var_egisl_dn8 = assign69290_e105834_d_n8;
        locals.var_egisl_dn9 = assign69290_e105834_d_n9;
        locals.var_egisl_dn10 = assign69290_e105834_d_n10;
        locals.var_egisl_dn11 = assign69290_e105834_d_n11;
        locals.var_egisl_dn14 = assign69290_e105834_d_n14;
        locals.var_egisl_rv = 0.0;

        let assign69300_e105837: f64 = if locals.var_egisl < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1622 = assign69300_e105837;
        locals.var_guard1622_rv = 0.0;

        let (assign69310_e105844, assign69310_e105844_d_n0, assign69310_e105844_d_n2, assign69310_e105844_d_n4, assign69310_e105844_d_n5, assign69310_e105844_d_n6, assign69310_e105844_d_n7, assign69310_e105844_d_n8, assign69310_e105844_d_n9, assign69310_e105844_d_n10, assign69310_e105844_d_n11, assign69310_e105844_d_n14,) = {
    if ((locals.var_guard1621 == 0.0) && (locals.var_guard1622 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_egisl, locals.var_egisl_dn0, locals.var_egisl_dn2, locals.var_egisl_dn4, locals.var_egisl_dn5, locals.var_egisl_dn6, locals.var_egisl_dn7, locals.var_egisl_dn8, locals.var_egisl_dn9, locals.var_egisl_dn10, locals.var_egisl_dn11, locals.var_egisl_dn14,)
    }
};
        locals.var_egisl = assign69310_e105844;
        locals.var_egisl_dn0 = assign69310_e105844_d_n0;
        locals.var_egisl_dn2 = assign69310_e105844_d_n2;
        locals.var_egisl_dn4 = assign69310_e105844_d_n4;
        locals.var_egisl_dn5 = assign69310_e105844_d_n5;
        locals.var_egisl_dn6 = assign69310_e105844_d_n6;
        locals.var_egisl_dn7 = assign69310_e105844_d_n7;
        locals.var_egisl_dn8 = assign69310_e105844_d_n8;
        locals.var_egisl_dn9 = assign69310_e105844_d_n9;
        locals.var_egisl_dn10 = assign69310_e105844_d_n10;
        locals.var_egisl_dn11 = assign69310_e105844_d_n11;
        locals.var_egisl_dn14 = assign69310_e105844_d_n14;
        locals.var_egisl_rv = 0.0;

        let (assign69320_e105851, assign69320_e105851_d_n0, assign69320_e105851_d_n2, assign69320_e105851_d_n4, assign69320_e105851_d_n5, assign69320_e105851_d_n6, assign69320_e105851_d_n7, assign69320_e105851_d_n8, assign69320_e105851_d_n9, assign69320_e105851_d_n10, assign69320_e105851_d_n11, assign69320_e105851_d_n14,) = {
    if ((locals.var_guard1621 == 0.0) && (locals.var_guard1622 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign69320_e105851;
        locals.var_t5_dn0 = assign69320_e105851_d_n0;
        locals.var_t5_dn2 = assign69320_e105851_d_n2;
        locals.var_t5_dn4 = assign69320_e105851_d_n4;
        locals.var_t5_dn5 = assign69320_e105851_d_n5;
        locals.var_t5_dn6 = assign69320_e105851_d_n6;
        locals.var_t5_dn7 = assign69320_e105851_d_n7;
        locals.var_t5_dn8 = assign69320_e105851_d_n8;
        locals.var_t5_dn9 = assign69320_e105851_d_n9;
        locals.var_t5_dn10 = assign69320_e105851_d_n10;
        locals.var_t5_dn11 = assign69320_e105851_d_n11;
        locals.var_t5_dn14 = assign69320_e105851_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign69330_e105860, assign69330_e105860_d_n0, assign69330_e105860_d_n2, assign69330_e105860_d_n4, assign69330_e105860_d_n5, assign69330_e105860_d_n6, assign69330_e105860_d_n7, assign69330_e105860_d_n8, assign69330_e105860_d_n9, assign69330_e105860_d_n10, assign69330_e105860_d_n11, assign69330_e105860_d_n14,) = {
    if (locals.var_guard1621 == 0.0) {
        let assign69330_e105857: f64 = (locals.var_egisl + 1e-25);
        let assign69330_e105858: f64 = (1.0 / assign69330_e105857);
        (assign69330_e105858, (-(locals.var_egisl_dn0 / (assign69330_e105857 * assign69330_e105857))), (-(locals.var_egisl_dn2 / (assign69330_e105857 * assign69330_e105857))), (-(locals.var_egisl_dn4 / (assign69330_e105857 * assign69330_e105857))), (-(locals.var_egisl_dn5 / (assign69330_e105857 * assign69330_e105857))), (-(locals.var_egisl_dn6 / (assign69330_e105857 * assign69330_e105857))), (-(locals.var_egisl_dn7 / (assign69330_e105857 * assign69330_e105857))), (-(locals.var_egisl_dn8 / (assign69330_e105857 * assign69330_e105857))), (-(locals.var_egisl_dn9 / (assign69330_e105857 * assign69330_e105857))), (-(locals.var_egisl_dn10 / (assign69330_e105857 * assign69330_e105857))), (-(locals.var_egisl_dn11 / (assign69330_e105857 * assign69330_e105857))), (-(locals.var_egisl_dn14 / (assign69330_e105857 * assign69330_e105857))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign69330_e105860;
        locals.var_t3_dn0 = assign69330_e105860_d_n0;
        locals.var_t3_dn2 = assign69330_e105860_d_n2;
        locals.var_t3_dn4 = assign69330_e105860_d_n4;
        locals.var_t3_dn5 = assign69330_e105860_d_n5;
        locals.var_t3_dn6 = assign69330_e105860_d_n6;
        locals.var_t3_dn7 = assign69330_e105860_d_n7;
        locals.var_t3_dn8 = assign69330_e105860_d_n8;
        locals.var_t3_dn9 = assign69330_e105860_d_n9;
        locals.var_t3_dn10 = assign69330_e105860_d_n10;
        locals.var_t3_dn11 = assign69330_e105860_d_n11;
        locals.var_t3_dn14 = assign69330_e105860_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign69340_e105870, assign69340_e105870_d_n0, assign69340_e105870_d_n2, assign69340_e105870_d_n4, assign69340_e105870_d_n5, assign69340_e105870_d_n6, assign69340_e105870_d_n7, assign69340_e105870_d_n8, assign69340_e105870_d_n9, assign69340_e105870_d_n10, assign69340_e105870_d_n11, assign69340_e105870_d_n14,) = {
    if (locals.var_guard1621 == 0.0) {
        let assign69340_e105864: f64 = (-locals.var_uc_gidl2);
        let assign69340_e105866: f64 = (assign69340_e105864 * locals.var_egp32);
        let assign69340_e105868: f64 = (assign69340_e105866 * locals.var_t3);
        (assign69340_e105868, (((assign69340_e105864 * locals.var_egp32_dn0) * locals.var_t3) + (assign69340_e105866 * locals.var_t3_dn0)), (((assign69340_e105864 * locals.var_egp32_dn2) * locals.var_t3) + (assign69340_e105866 * locals.var_t3_dn2)), (((assign69340_e105864 * locals.var_egp32_dn4) * locals.var_t3) + (assign69340_e105866 * locals.var_t3_dn4)), (((assign69340_e105864 * locals.var_egp32_dn5) * locals.var_t3) + (assign69340_e105866 * locals.var_t3_dn5)), (((assign69340_e105864 * locals.var_egp32_dn6) * locals.var_t3) + (assign69340_e105866 * locals.var_t3_dn6)), (((assign69340_e105864 * locals.var_egp32_dn7) * locals.var_t3) + (assign69340_e105866 * locals.var_t3_dn7)), (((assign69340_e105864 * locals.var_egp32_dn8) * locals.var_t3) + (assign69340_e105866 * locals.var_t3_dn8)), (((assign69340_e105864 * locals.var_egp32_dn9) * locals.var_t3) + (assign69340_e105866 * locals.var_t3_dn9)), (((assign69340_e105864 * locals.var_egp32_dn10) * locals.var_t3) + (assign69340_e105866 * locals.var_t3_dn10)), (((assign69340_e105864 * locals.var_egp32_dn11) * locals.var_t3) + (assign69340_e105866 * locals.var_t3_dn11)), (((assign69340_e105864 * locals.var_egp32_dn14) * locals.var_t3) + (assign69340_e105866 * locals.var_t3_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign69340_e105870;
        locals.var_t0_dn0 = assign69340_e105870_d_n0;
        locals.var_t0_dn2 = assign69340_e105870_d_n2;
        locals.var_t0_dn4 = assign69340_e105870_d_n4;
        locals.var_t0_dn5 = assign69340_e105870_d_n5;
        locals.var_t0_dn6 = assign69340_e105870_d_n6;
        locals.var_t0_dn7 = assign69340_e105870_d_n7;
        locals.var_t0_dn8 = assign69340_e105870_d_n8;
        locals.var_t0_dn9 = assign69340_e105870_d_n9;
        locals.var_t0_dn10 = assign69340_e105870_d_n10;
        locals.var_t0_dn11 = assign69340_e105870_d_n11;
        locals.var_t0_dn14 = assign69340_e105870_d_n14;
        locals.var_t0_rv = 0.0;

        let assign69350_e105873: f64 = (-34.0);
        let assign69350_e105874: f64 = if locals.var_t0 < assign69350_e105873 { 1.0 } else { 0.0 };
        locals.var_guard1623 = assign69350_e105874;
        locals.var_guard1623_rv = 0.0;

        let (assign69370_e105890, assign69370_e105890_d_n0, assign69370_e105890_d_n2, assign69370_e105890_d_n4, assign69370_e105890_d_n5, assign69370_e105890_d_n6, assign69370_e105890_d_n7, assign69370_e105890_d_n8, assign69370_e105890_d_n9, assign69370_e105890_d_n10, assign69370_e105890_d_n11, assign69370_e105890_d_n14,) = {
    if ((locals.var_guard1621 == 0.0) && (locals.var_guard1623 == 0.0)) {
        let assign69370_e105888: f64 = (locals.var_t0).exp();
        (assign69370_e105888, (assign69370_e105888 * locals.var_t0_dn0), (assign69370_e105888 * locals.var_t0_dn2), (assign69370_e105888 * locals.var_t0_dn4), (assign69370_e105888 * locals.var_t0_dn5), (assign69370_e105888 * locals.var_t0_dn6), (assign69370_e105888 * locals.var_t0_dn7), (assign69370_e105888 * locals.var_t0_dn8), (assign69370_e105888 * locals.var_t0_dn9), (assign69370_e105888 * locals.var_t0_dn10), (assign69370_e105888 * locals.var_t0_dn11), (assign69370_e105888 * locals.var_t0_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign69370_e105890;
        locals.var_t1_dn0 = assign69370_e105890_d_n0;
        locals.var_t1_dn2 = assign69370_e105890_d_n2;
        locals.var_t1_dn4 = assign69370_e105890_d_n4;
        locals.var_t1_dn5 = assign69370_e105890_d_n5;
        locals.var_t1_dn6 = assign69370_e105890_d_n6;
        locals.var_t1_dn7 = assign69370_e105890_d_n7;
        locals.var_t1_dn8 = assign69370_e105890_d_n8;
        locals.var_t1_dn9 = assign69370_e105890_d_n9;
        locals.var_t1_dn10 = assign69370_e105890_d_n10;
        locals.var_t1_dn11 = assign69370_e105890_d_n11;
        locals.var_t1_dn14 = assign69370_e105890_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign69380_e105900, assign69380_e105900_d_n0, assign69380_e105900_d_n2, assign69380_e105900_d_n4, assign69380_e105900_d_n5, assign69380_e105900_d_n6, assign69380_e105900_d_n7, assign69380_e105900_d_n8, assign69380_e105900_d_n9, assign69380_e105900_d_n10, assign69380_e105900_d_n11, assign69380_e105900_d_n14,) = {
    if ((locals.var_guard1621 == 0.0) && (locals.var_guard1623 == 0.0)) {
        let assign69380_e105898: f64 = (1.0 / locals.var_egp12);
        (assign69380_e105898, (-(locals.var_egp12_dn0 / (locals.var_egp12 * locals.var_egp12))), (-(locals.var_egp12_dn2 / (locals.var_egp12 * locals.var_egp12))), (-(locals.var_egp12_dn4 / (locals.var_egp12 * locals.var_egp12))), (-(locals.var_egp12_dn5 / (locals.var_egp12 * locals.var_egp12))), (-(locals.var_egp12_dn6 / (locals.var_egp12 * locals.var_egp12))), (-(locals.var_egp12_dn7 / (locals.var_egp12 * locals.var_egp12))), (-(locals.var_egp12_dn8 / (locals.var_egp12 * locals.var_egp12))), (-(locals.var_egp12_dn9 / (locals.var_egp12 * locals.var_egp12))), (-(locals.var_egp12_dn10 / (locals.var_egp12 * locals.var_egp12))), (-(locals.var_egp12_dn11 / (locals.var_egp12 * locals.var_egp12))), (-(locals.var_egp12_dn14 / (locals.var_egp12 * locals.var_egp12))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign69380_e105900;
        locals.var_t3_dn0 = assign69380_e105900_d_n0;
        locals.var_t3_dn2 = assign69380_e105900_d_n2;
        locals.var_t3_dn4 = assign69380_e105900_d_n4;
        locals.var_t3_dn5 = assign69380_e105900_d_n5;
        locals.var_t3_dn6 = assign69380_e105900_d_n6;
        locals.var_t3_dn7 = assign69380_e105900_d_n7;
        locals.var_t3_dn8 = assign69380_e105900_d_n8;
        locals.var_t3_dn9 = assign69380_e105900_d_n9;
        locals.var_t3_dn10 = assign69380_e105900_d_n10;
        locals.var_t3_dn11 = assign69380_e105900_d_n11;
        locals.var_t3_dn14 = assign69380_e105900_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign69390_e105914, assign69390_e105914_d_n0, assign69390_e105914_d_n2, assign69390_e105914_d_n4, assign69390_e105914_d_n5, assign69390_e105914_d_n6, assign69390_e105914_d_n7, assign69390_e105914_d_n8, assign69390_e105914_d_n9, assign69390_e105914_d_n10, assign69390_e105914_d_n11, assign69390_e105914_d_n14,) = {
    if ((locals.var_guard1621 == 0.0) && (locals.var_guard1623 == 0.0)) {
        let assign69390_e105908: f64 = (locals.var_uc_gidl1 * locals.var_t3);
        let assign69390_e105910: f64 = (assign69390_e105908 * 1.6021918e-19);
        let assign69390_e105912: f64 = (assign69390_e105910 * locals.var_weff_nf);
        (assign69390_e105912, (((locals.var_uc_gidl1 * locals.var_t3_dn0) * 1.6021918e-19) * locals.var_weff_nf), (((locals.var_uc_gidl1 * locals.var_t3_dn2) * 1.6021918e-19) * locals.var_weff_nf), (((locals.var_uc_gidl1 * locals.var_t3_dn4) * 1.6021918e-19) * locals.var_weff_nf), (((locals.var_uc_gidl1 * locals.var_t3_dn5) * 1.6021918e-19) * locals.var_weff_nf), (((locals.var_uc_gidl1 * locals.var_t3_dn6) * 1.6021918e-19) * locals.var_weff_nf), (((locals.var_uc_gidl1 * locals.var_t3_dn7) * 1.6021918e-19) * locals.var_weff_nf), (((locals.var_uc_gidl1 * locals.var_t3_dn8) * 1.6021918e-19) * locals.var_weff_nf), (((locals.var_uc_gidl1 * locals.var_t3_dn9) * 1.6021918e-19) * locals.var_weff_nf), (((locals.var_uc_gidl1 * locals.var_t3_dn10) * 1.6021918e-19) * locals.var_weff_nf), (((locals.var_uc_gidl1 * locals.var_t3_dn11) * 1.6021918e-19) * locals.var_weff_nf), (((locals.var_uc_gidl1 * locals.var_t3_dn14) * 1.6021918e-19) * locals.var_weff_nf),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign69390_e105914;
        locals.var_t2_dn0 = assign69390_e105914_d_n0;
        locals.var_t2_dn2 = assign69390_e105914_d_n2;
        locals.var_t2_dn4 = assign69390_e105914_d_n4;
        locals.var_t2_dn5 = assign69390_e105914_d_n5;
        locals.var_t2_dn6 = assign69390_e105914_d_n6;
        locals.var_t2_dn7 = assign69390_e105914_d_n7;
        locals.var_t2_dn8 = assign69390_e105914_d_n8;
        locals.var_t2_dn9 = assign69390_e105914_d_n9;
        locals.var_t2_dn10 = assign69390_e105914_d_n10;
        locals.var_t2_dn11 = assign69390_e105914_d_n11;
        locals.var_t2_dn14 = assign69390_e105914_d_n14;
        locals.var_t2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_261(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign69410_e105934, assign69410_e105934_d_n6, assign69410_e105934_d_n8, assign69410_e105934_d_n9,) = {
    if (locals.var_guard1621 == 0.0) {
        let assign69410_e105932: f64 = (-locals.var_vbs);
        (assign69410_e105932, (-locals.var_vbs_dn6), (-locals.var_vbs_dn8), (-locals.var_vbs_dn9),)
    } else {
        (locals.var_vsb, locals.var_vsb_dn6, locals.var_vsb_dn8, locals.var_vsb_dn9,)
    }
};
        locals.var_vsb = assign69410_e105934;
        locals.var_vsb_dn6 = assign69410_e105934_d_n6;
        locals.var_vsb_dn8 = assign69410_e105934_d_n8;
        locals.var_vsb_dn9 = assign69410_e105934_d_n9;
        locals.var_vsb_rv = 0.0;

        let assign69420_e105937: f64 = if locals.var_vsb > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1624 = assign69420_e105937;
        locals.var_guard1624_rv = 0.0;

        let (assign69430_e105946, assign69430_e105946_d_n0, assign69430_e105946_d_n2, assign69430_e105946_d_n4, assign69430_e105946_d_n5, assign69430_e105946_d_n6, assign69430_e105946_d_n7, assign69430_e105946_d_n8, assign69430_e105946_d_n9, assign69430_e105946_d_n10, assign69430_e105946_d_n11, assign69430_e105946_d_n14,) = {
    if ((locals.var_guard1621 == 0.0) && (locals.var_guard1624 != 0.0)) {
        let assign69430_e105944: f64 = (locals.var_vsb * locals.var_vsb);
        (assign69430_e105944, 0.0, 0.0, 0.0, 0.0, ((locals.var_vsb_dn6 * locals.var_vsb) + (locals.var_vsb * locals.var_vsb_dn6)), 0.0, ((locals.var_vsb_dn8 * locals.var_vsb) + (locals.var_vsb * locals.var_vsb_dn8)), ((locals.var_vsb_dn9 * locals.var_vsb) + (locals.var_vsb * locals.var_vsb_dn9)), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign69430_e105946;
        locals.var_t2_dn0 = assign69430_e105946_d_n0;
        locals.var_t2_dn2 = assign69430_e105946_d_n2;
        locals.var_t2_dn4 = assign69430_e105946_d_n4;
        locals.var_t2_dn5 = assign69430_e105946_d_n5;
        locals.var_t2_dn6 = assign69430_e105946_d_n6;
        locals.var_t2_dn7 = assign69430_e105946_d_n7;
        locals.var_t2_dn8 = assign69430_e105946_d_n8;
        locals.var_t2_dn9 = assign69430_e105946_d_n9;
        locals.var_t2_dn10 = assign69430_e105946_d_n10;
        locals.var_t2_dn11 = assign69430_e105946_d_n11;
        locals.var_t2_dn14 = assign69430_e105946_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign69440_e105955, assign69440_e105955_d_n0, assign69440_e105955_d_n2, assign69440_e105955_d_n4, assign69440_e105955_d_n5, assign69440_e105955_d_n6, assign69440_e105955_d_n7, assign69440_e105955_d_n8, assign69440_e105955_d_n9, assign69440_e105955_d_n10, assign69440_e105955_d_n11, assign69440_e105955_d_n14,) = {
    if ((locals.var_guard1621 == 0.0) && (locals.var_guard1624 != 0.0)) {
        let assign69440_e105953: f64 = (locals.var_t2 * locals.var_vsb);
        (assign69440_e105953, (locals.var_t2_dn0 * locals.var_vsb), (locals.var_t2_dn2 * locals.var_vsb), (locals.var_t2_dn4 * locals.var_vsb), (locals.var_t2_dn5 * locals.var_vsb), ((locals.var_t2_dn6 * locals.var_vsb) + (locals.var_t2 * locals.var_vsb_dn6)), (locals.var_t2_dn7 * locals.var_vsb), ((locals.var_t2_dn8 * locals.var_vsb) + (locals.var_t2 * locals.var_vsb_dn8)), ((locals.var_t2_dn9 * locals.var_vsb) + (locals.var_t2 * locals.var_vsb_dn9)), (locals.var_t2_dn10 * locals.var_vsb), (locals.var_t2_dn11 * locals.var_vsb), (locals.var_t2_dn14 * locals.var_vsb),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign69440_e105955;
        locals.var_t4_dn0 = assign69440_e105955_d_n0;
        locals.var_t4_dn2 = assign69440_e105955_d_n2;
        locals.var_t4_dn4 = assign69440_e105955_d_n4;
        locals.var_t4_dn5 = assign69440_e105955_d_n5;
        locals.var_t4_dn6 = assign69440_e105955_d_n6;
        locals.var_t4_dn7 = assign69440_e105955_d_n7;
        locals.var_t4_dn8 = assign69440_e105955_d_n8;
        locals.var_t4_dn9 = assign69440_e105955_d_n9;
        locals.var_t4_dn10 = assign69440_e105955_d_n10;
        locals.var_t4_dn11 = assign69440_e105955_d_n11;
        locals.var_t4_dn14 = assign69440_e105955_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign69450_e105964, assign69450_e105964_d_n0, assign69450_e105964_d_n2, assign69450_e105964_d_n4, assign69450_e105964_d_n5, assign69450_e105964_d_n6, assign69450_e105964_d_n7, assign69450_e105964_d_n8, assign69450_e105964_d_n9, assign69450_e105964_d_n10, assign69450_e105964_d_n11, assign69450_e105964_d_n14,) = {
    if ((locals.var_guard1621 == 0.0) && (locals.var_guard1624 != 0.0)) {
        let assign69450_e105962: f64 = (locals.var_t4 + 0.5);
        (assign69450_e105962, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign69450_e105964;
        locals.var_t0_dn0 = assign69450_e105964_d_n0;
        locals.var_t0_dn2 = assign69450_e105964_d_n2;
        locals.var_t0_dn4 = assign69450_e105964_d_n4;
        locals.var_t0_dn5 = assign69450_e105964_d_n5;
        locals.var_t0_dn6 = assign69450_e105964_d_n6;
        locals.var_t0_dn7 = assign69450_e105964_d_n7;
        locals.var_t0_dn8 = assign69450_e105964_d_n8;
        locals.var_t0_dn9 = assign69450_e105964_d_n9;
        locals.var_t0_dn10 = assign69450_e105964_d_n10;
        locals.var_t0_dn11 = assign69450_e105964_d_n11;
        locals.var_t0_dn14 = assign69450_e105964_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign69460_e105973, assign69460_e105973_d_n0, assign69460_e105973_d_n2, assign69460_e105973_d_n4, assign69460_e105973_d_n5, assign69460_e105973_d_n6, assign69460_e105973_d_n7, assign69460_e105973_d_n8, assign69460_e105973_d_n9, assign69460_e105973_d_n10, assign69460_e105973_d_n11, assign69460_e105973_d_n14,) = {
    if ((locals.var_guard1621 == 0.0) && (locals.var_guard1624 != 0.0)) {
        let assign69460_e105971: f64 = (locals.var_t4 / locals.var_t0);
        (assign69460_e105971, (((locals.var_t4_dn0 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn0)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn2 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn2)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn4 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn5 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn6 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn7 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn8 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn9 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn9)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn10 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn11 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn11)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn14 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn14)) / (locals.var_t0 * locals.var_t0)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign69460_e105973;
        locals.var_t5_dn0 = assign69460_e105973_d_n0;
        locals.var_t5_dn2 = assign69460_e105973_d_n2;
        locals.var_t5_dn4 = assign69460_e105973_d_n4;
        locals.var_t5_dn5 = assign69460_e105973_d_n5;
        locals.var_t5_dn6 = assign69460_e105973_d_n6;
        locals.var_t5_dn7 = assign69460_e105973_d_n7;
        locals.var_t5_dn8 = assign69460_e105973_d_n8;
        locals.var_t5_dn9 = assign69460_e105973_d_n9;
        locals.var_t5_dn10 = assign69460_e105973_d_n10;
        locals.var_t5_dn11 = assign69460_e105973_d_n11;
        locals.var_t5_dn14 = assign69460_e105973_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign69470_e105994, assign69470_e105994_d_n0, assign69470_e105994_d_n2, assign69470_e105994_d_n4, assign69470_e105994_d_n5, assign69470_e105994_d_n6, assign69470_e105994_d_n7, assign69470_e105994_d_n8, assign69470_e105994_d_n9, assign69470_e105994_d_n10, assign69470_e105994_d_n11, assign69470_e105994_d_n14,) = {
    if ((locals.var_guard1621 == 0.0) && (locals.var_guard1624 != 0.0)) {
        let assign69470_e105980: f64 = (3.0 * locals.var_t2);
        let assign69470_e105982: f64 = (assign69470_e105980 * locals.var_t0);
        let assign69470_e105985: f64 = (locals.var_t4 * 3.0);
        let assign69470_e105987: f64 = (assign69470_e105985 * locals.var_t2);
        let assign69470_e105988: f64 = (assign69470_e105982 - assign69470_e105987);
        let assign69470_e105991: f64 = (locals.var_t0 * locals.var_t0);
        let assign69470_e105992: f64 = (assign69470_e105988 / assign69470_e105991);
        (assign69470_e105992, (((((((3.0 * locals.var_t2_dn0) * locals.var_t0) + (assign69470_e105980 * locals.var_t0_dn0)) - (((locals.var_t4_dn0 * 3.0) * locals.var_t2) + (assign69470_e105985 * locals.var_t2_dn0))) * assign69470_e105991) - (assign69470_e105988 * ((locals.var_t0_dn0 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn0)))) / (assign69470_e105991 * assign69470_e105991)), (((((((3.0 * locals.var_t2_dn2) * locals.var_t0) + (assign69470_e105980 * locals.var_t0_dn2)) - (((locals.var_t4_dn2 * 3.0) * locals.var_t2) + (assign69470_e105985 * locals.var_t2_dn2))) * assign69470_e105991) - (assign69470_e105988 * ((locals.var_t0_dn2 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn2)))) / (assign69470_e105991 * assign69470_e105991)), (((((((3.0 * locals.var_t2_dn4) * locals.var_t0) + (assign69470_e105980 * locals.var_t0_dn4)) - (((locals.var_t4_dn4 * 3.0) * locals.var_t2) + (assign69470_e105985 * locals.var_t2_dn4))) * assign69470_e105991) - (assign69470_e105988 * ((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)))) / (assign69470_e105991 * assign69470_e105991)), (((((((3.0 * locals.var_t2_dn5) * locals.var_t0) + (assign69470_e105980 * locals.var_t0_dn5)) - (((locals.var_t4_dn5 * 3.0) * locals.var_t2) + (assign69470_e105985 * locals.var_t2_dn5))) * assign69470_e105991) - (assign69470_e105988 * ((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)))) / (assign69470_e105991 * assign69470_e105991)), (((((((3.0 * locals.var_t2_dn6) * locals.var_t0) + (assign69470_e105980 * locals.var_t0_dn6)) - (((locals.var_t4_dn6 * 3.0) * locals.var_t2) + (assign69470_e105985 * locals.var_t2_dn6))) * assign69470_e105991) - (assign69470_e105988 * ((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)))) / (assign69470_e105991 * assign69470_e105991)), (((((((3.0 * locals.var_t2_dn7) * locals.var_t0) + (assign69470_e105980 * locals.var_t0_dn7)) - (((locals.var_t4_dn7 * 3.0) * locals.var_t2) + (assign69470_e105985 * locals.var_t2_dn7))) * assign69470_e105991) - (assign69470_e105988 * ((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)))) / (assign69470_e105991 * assign69470_e105991)), (((((((3.0 * locals.var_t2_dn8) * locals.var_t0) + (assign69470_e105980 * locals.var_t0_dn8)) - (((locals.var_t4_dn8 * 3.0) * locals.var_t2) + (assign69470_e105985 * locals.var_t2_dn8))) * assign69470_e105991) - (assign69470_e105988 * ((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)))) / (assign69470_e105991 * assign69470_e105991)), (((((((3.0 * locals.var_t2_dn9) * locals.var_t0) + (assign69470_e105980 * locals.var_t0_dn9)) - (((locals.var_t4_dn9 * 3.0) * locals.var_t2) + (assign69470_e105985 * locals.var_t2_dn9))) * assign69470_e105991) - (assign69470_e105988 * ((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)))) / (assign69470_e105991 * assign69470_e105991)), (((((((3.0 * locals.var_t2_dn10) * locals.var_t0) + (assign69470_e105980 * locals.var_t0_dn10)) - (((locals.var_t4_dn10 * 3.0) * locals.var_t2) + (assign69470_e105985 * locals.var_t2_dn10))) * assign69470_e105991) - (assign69470_e105988 * ((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)))) / (assign69470_e105991 * assign69470_e105991)), (((((((3.0 * locals.var_t2_dn11) * locals.var_t0) + (assign69470_e105980 * locals.var_t0_dn11)) - (((locals.var_t4_dn11 * 3.0) * locals.var_t2) + (assign69470_e105985 * locals.var_t2_dn11))) * assign69470_e105991) - (assign69470_e105988 * ((locals.var_t0_dn11 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn11)))) / (assign69470_e105991 * assign69470_e105991)), (((((((3.0 * locals.var_t2_dn14) * locals.var_t0) + (assign69470_e105980 * locals.var_t0_dn14)) - (((locals.var_t4_dn14 * 3.0) * locals.var_t2) + (assign69470_e105985 * locals.var_t2_dn14))) * assign69470_e105991) - (assign69470_e105988 * ((locals.var_t0_dn14 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn14)))) / (assign69470_e105991 * assign69470_e105991)),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign69470_e105994;
        locals.var_t7_dn0 = assign69470_e105994_d_n0;
        locals.var_t7_dn2 = assign69470_e105994_d_n2;
        locals.var_t7_dn4 = assign69470_e105994_d_n4;
        locals.var_t7_dn5 = assign69470_e105994_d_n5;
        locals.var_t7_dn6 = assign69470_e105994_d_n6;
        locals.var_t7_dn7 = assign69470_e105994_d_n7;
        locals.var_t7_dn8 = assign69470_e105994_d_n8;
        locals.var_t7_dn9 = assign69470_e105994_d_n9;
        locals.var_t7_dn10 = assign69470_e105994_d_n10;
        locals.var_t7_dn11 = assign69470_e105994_d_n11;
        locals.var_t7_dn14 = assign69470_e105994_d_n14;
        locals.var_t7_rv = 0.0;

        locals.var_flg_coovlps = 0.0;
        locals.var_flg_coovlps_rv = 0.0;

        locals.var_flg_coovlp = 0.0;
        locals.var_flg_coovlp_rv = 0.0;

        locals.var_flg_calcqover = 0.0;
        locals.var_flg_calcqover_rv = 0.0;

        locals.var_flg_never_reach_vfbover = 0.0;
        locals.var_flg_never_reach_vfbover_rv = 0.0;

        locals.var_flg_calcqover = 0.0;
        locals.var_flg_calcqover_rv = 0.0;

        let assign69560_e106020: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1629 = assign69560_e106020;
        locals.var_guard1629_rv = 0.0;

        let assign69570_e106023: f64 = if 1.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1630 = assign69570_e106023;
        locals.var_guard1630_rv = 0.0;

        let assign69580_e106026: f64 = if 1.0 == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard1631 = assign69580_e106026;
        locals.var_guard1631_rv = 0.0;

        let assign69590_e106029: f64 = if 1.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1632 = assign69590_e106029;
        locals.var_guard1632_rv = 0.0;

        let assign69600_e106040: f64 = if (((p.p36 == 1.0) && (p.p66 > 0.0)) && (locals.var_uc_novers > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1633 = assign69600_e106040;
        locals.var_guard1633_rv = 0.0;

        let (assign69610_e106046,) = {
    if ((locals.var_guard1629 != 0.0) && (locals.var_guard1633 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_calcqover,)
    }
};
        locals.var_flg_calcqover = assign69610_e106046;
        locals.var_flg_calcqover_rv = 0.0;

        let (assign69620_e106052,) = {
    if ((locals.var_guard1629 != 0.0) && (locals.var_guard1633 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_coovlps,)
    }
};
        locals.var_flg_coovlps = assign69620_e106052;
        locals.var_flg_coovlps_rv = 0.0;

        let (assign69630_e106060, assign69630_e106060_d_n2, assign69630_e106060_d_n7, assign69630_e106060_d_n8, assign69630_e106060_d_n9,) = {
    if ((locals.var_guard1629 != 0.0) && (locals.var_guard1633 != 0.0)) {
        let assign69630_e106058: f64 = (locals.var_vgsi - locals.var_vbsi);
        (assign69630_e106058, 0.0, locals.var_vgsi_dn7, (locals.var_vgsi_dn8 - locals.var_vbsi_dn8), (-locals.var_vbsi_dn9),)
    } else {
        (locals.var_vgbgmt, locals.var_vgbgmt_dn2, locals.var_vgbgmt_dn7, locals.var_vgbgmt_dn8, locals.var_vgbgmt_dn9,)
    }
};
        locals.var_vgbgmt = assign69630_e106060;
        locals.var_vgbgmt_dn2 = assign69630_e106060_d_n2;
        locals.var_vgbgmt_dn7 = assign69630_e106060_d_n7;
        locals.var_vgbgmt_dn8 = assign69630_e106060_d_n8;
        locals.var_vgbgmt_dn9 = assign69630_e106060_d_n9;
        locals.var_vgbgmt_rv = 0.0;

        let (assign69640_e106067, assign69640_e106067_d_n0, assign69640_e106067_d_n2, assign69640_e106067_d_n4, assign69640_e106067_d_n5, assign69640_e106067_d_n6, assign69640_e106067_d_n7, assign69640_e106067_d_n8, assign69640_e106067_d_n9, assign69640_e106067_d_n10, assign69640_e106067_d_n11, assign69640_e106067_d_n14,) = {
    if ((locals.var_guard1629 != 0.0) && (locals.var_guard1633 != 0.0)) {
        let assign69640_e106065: f64 = (-locals.var_vbsi);
        (assign69640_e106065, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, (-locals.var_vbsi_dn8), (-locals.var_vbsi_dn9), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn11, locals.var_vxbgmt_dn14,)
    }
};
        locals.var_vxbgmt = assign69640_e106067;
        locals.var_vxbgmt_dn0 = assign69640_e106067_d_n0;
        locals.var_vxbgmt_dn2 = assign69640_e106067_d_n2;
        locals.var_vxbgmt_dn4 = assign69640_e106067_d_n4;
        locals.var_vxbgmt_dn5 = assign69640_e106067_d_n5;
        locals.var_vxbgmt_dn6 = assign69640_e106067_d_n6;
        locals.var_vxbgmt_dn7 = assign69640_e106067_d_n7;
        locals.var_vxbgmt_dn8 = assign69640_e106067_d_n8;
        locals.var_vxbgmt_dn9 = assign69640_e106067_d_n9;
        locals.var_vxbgmt_dn10 = assign69640_e106067_d_n10;
        locals.var_vxbgmt_dn11 = assign69640_e106067_d_n11;
        locals.var_vxbgmt_dn14 = assign69640_e106067_d_n14;
        locals.var_vxbgmt_rv = 0.0;

        let (assign69650_e106073,) = {
    if ((locals.var_guard1629 != 0.0) && (locals.var_guard1633 != 0.0)) {
        (locals.var_uc_novers,)
    } else {
        (locals.var_nover_func,)
    }
};
        locals.var_nover_func = assign69650_e106073;
        locals.var_nover_func_rv = 0.0;

        let (assign69660_e106079, assign69660_e106079_d_n0, assign69660_e106079_d_n2, assign69660_e106079_d_n4, assign69660_e106079_d_n5, assign69660_e106079_d_n6, assign69660_e106079_d_n7, assign69660_e106079_d_n8, assign69660_e106079_d_n9, assign69660_e106079_d_n10, assign69660_e106079_d_n11, assign69660_e106079_d_n14,) = {
    if ((locals.var_guard1629 != 0.0) && (locals.var_guard1633 != 0.0)) {
        (p.p66, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_lover_func, locals.var_lover_func_dn0, locals.var_lover_func_dn2, locals.var_lover_func_dn4, locals.var_lover_func_dn5, locals.var_lover_func_dn6, locals.var_lover_func_dn7, locals.var_lover_func_dn8, locals.var_lover_func_dn9, locals.var_lover_func_dn10, locals.var_lover_func_dn11, locals.var_lover_func_dn14,)
    }
};
        locals.var_lover_func = assign69660_e106079;
        locals.var_lover_func_dn0 = assign69660_e106079_d_n0;
        locals.var_lover_func_dn2 = assign69660_e106079_d_n2;
        locals.var_lover_func_dn4 = assign69660_e106079_d_n4;
        locals.var_lover_func_dn5 = assign69660_e106079_d_n5;
        locals.var_lover_func_dn6 = assign69660_e106079_d_n6;
        locals.var_lover_func_dn7 = assign69660_e106079_d_n7;
        locals.var_lover_func_dn8 = assign69660_e106079_d_n8;
        locals.var_lover_func_dn9 = assign69660_e106079_d_n9;
        locals.var_lover_func_dn10 = assign69660_e106079_d_n10;
        locals.var_lover_func_dn11 = assign69660_e106079_d_n11;
        locals.var_lover_func_dn14 = assign69660_e106079_d_n14;
        locals.var_lover_func_rv = 0.0;

        let (assign69670_e106085, assign69670_e106085_d_n0, assign69670_e106085_d_n2, assign69670_e106085_d_n4, assign69670_e106085_d_n5, assign69670_e106085_d_n6, assign69670_e106085_d_n7, assign69670_e106085_d_n8, assign69670_e106085_d_n9, assign69670_e106085_d_n10, assign69670_e106085_d_n11, assign69670_e106085_d_n14,) = {
    if ((locals.var_guard1629 != 0.0) && (locals.var_guard1633 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_wdep_func, locals.var_wdep_func_dn0, locals.var_wdep_func_dn2, locals.var_wdep_func_dn4, locals.var_wdep_func_dn5, locals.var_wdep_func_dn6, locals.var_wdep_func_dn7, locals.var_wdep_func_dn8, locals.var_wdep_func_dn9, locals.var_wdep_func_dn10, locals.var_wdep_func_dn11, locals.var_wdep_func_dn14,)
    }
};
        locals.var_wdep_func = assign69670_e106085;
        locals.var_wdep_func_dn0 = assign69670_e106085_d_n0;
        locals.var_wdep_func_dn2 = assign69670_e106085_d_n2;
        locals.var_wdep_func_dn4 = assign69670_e106085_d_n4;
        locals.var_wdep_func_dn5 = assign69670_e106085_d_n5;
        locals.var_wdep_func_dn6 = assign69670_e106085_d_n6;
        locals.var_wdep_func_dn7 = assign69670_e106085_d_n7;
        locals.var_wdep_func_dn8 = assign69670_e106085_d_n8;
        locals.var_wdep_func_dn9 = assign69670_e106085_d_n9;
        locals.var_wdep_func_dn10 = assign69670_e106085_d_n10;
        locals.var_wdep_func_dn11 = assign69670_e106085_d_n11;
        locals.var_wdep_func_dn14 = assign69670_e106085_d_n14;
        locals.var_wdep_func_rv = 0.0;

        let (assign69680_e106091, assign69680_e106091_d_n0, assign69680_e106091_d_n2, assign69680_e106091_d_n4, assign69680_e106091_d_n5, assign69680_e106091_d_n6, assign69680_e106091_d_n7, assign69680_e106091_d_n8, assign69680_e106091_d_n9, assign69680_e106091_d_n10, assign69680_e106091_d_n11, assign69680_e106091_d_n14,) = {
    if ((locals.var_guard1629 != 0.0) && (locals.var_guard1633 != 0.0)) {
        (locals.var_cnst0overs, locals.var_cnst0overs_dn0, locals.var_cnst0overs_dn2, locals.var_cnst0overs_dn4, locals.var_cnst0overs_dn5, locals.var_cnst0overs_dn6, locals.var_cnst0overs_dn7, locals.var_cnst0overs_dn8, locals.var_cnst0overs_dn9, locals.var_cnst0overs_dn10, locals.var_cnst0overs_dn11, locals.var_cnst0overs_dn14,)
    } else {
        (locals.var_cnst0over_func, locals.var_cnst0over_func_dn0, locals.var_cnst0over_func_dn2, locals.var_cnst0over_func_dn4, locals.var_cnst0over_func_dn5, locals.var_cnst0over_func_dn6, locals.var_cnst0over_func_dn7, locals.var_cnst0over_func_dn8, locals.var_cnst0over_func_dn9, locals.var_cnst0over_func_dn10, locals.var_cnst0over_func_dn11, locals.var_cnst0over_func_dn14,)
    }
};
        locals.var_cnst0over_func = assign69680_e106091;
        locals.var_cnst0over_func_dn0 = assign69680_e106091_d_n0;
        locals.var_cnst0over_func_dn2 = assign69680_e106091_d_n2;
        locals.var_cnst0over_func_dn4 = assign69680_e106091_d_n4;
        locals.var_cnst0over_func_dn5 = assign69680_e106091_d_n5;
        locals.var_cnst0over_func_dn6 = assign69680_e106091_d_n6;
        locals.var_cnst0over_func_dn7 = assign69680_e106091_d_n7;
        locals.var_cnst0over_func_dn8 = assign69680_e106091_d_n8;
        locals.var_cnst0over_func_dn9 = assign69680_e106091_d_n9;
        locals.var_cnst0over_func_dn10 = assign69680_e106091_d_n10;
        locals.var_cnst0over_func_dn11 = assign69680_e106091_d_n11;
        locals.var_cnst0over_func_dn14 = assign69680_e106091_d_n14;
        locals.var_cnst0over_func_rv = 0.0;

        let (assign69690_e106097,) = {
    if ((locals.var_guard1629 != 0.0) && (locals.var_guard1633 != 0.0)) {
        (locals.var_cox0,)
    } else {
        (locals.var_cox0_func,)
    }
};
        locals.var_cox0_func = assign69690_e106097;
        locals.var_cox0_func_rv = 0.0;

        let assign69700_e106116: f64 = if (((((p.p36 == 1.0) && (p.p66 > 0.0)) && (locals.var_uc_novers > 0.0)) && (locals.var_uc_cvdsover != 0.0)) && (p.p55 != 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard1634 = assign69700_e106116;
        locals.var_guard1634_rv = 0.0;

        let (assign69710_e106125,) = {
    if (((locals.var_guard1630 != 0.0) && (locals.var_guard1629 == 0.0)) && (locals.var_guard1634 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_calcqover,)
    }
};
        locals.var_flg_calcqover = assign69710_e106125;
        locals.var_flg_calcqover_rv = 0.0;

        let (assign69720_e106136, assign69720_e106136_d_n2, assign69720_e106136_d_n7, assign69720_e106136_d_n8, assign69720_e106136_d_n9,) = {
    if (((locals.var_guard1630 != 0.0) && (locals.var_guard1629 == 0.0)) && (locals.var_guard1634 != 0.0)) {
        let assign69720_e106134: f64 = (locals.var_vgsei - locals.var_vbsei);
        (assign69720_e106134, (locals.var_vgsei_dn2 - locals.var_vbsei_dn2), locals.var_vgsei_dn7, 0.0, (-locals.var_vbsei_dn9),)
    } else {
        (locals.var_vgbgmt, locals.var_vgbgmt_dn2, locals.var_vgbgmt_dn7, locals.var_vgbgmt_dn8, locals.var_vgbgmt_dn9,)
    }
};
        locals.var_vgbgmt = assign69720_e106136;
        locals.var_vgbgmt_dn2 = assign69720_e106136_d_n2;
        locals.var_vgbgmt_dn7 = assign69720_e106136_d_n7;
        locals.var_vgbgmt_dn8 = assign69720_e106136_d_n8;
        locals.var_vgbgmt_dn9 = assign69720_e106136_d_n9;
        locals.var_vgbgmt_rv = 0.0;

        let (assign69730_e106146, assign69730_e106146_d_n0, assign69730_e106146_d_n2, assign69730_e106146_d_n4, assign69730_e106146_d_n5, assign69730_e106146_d_n6, assign69730_e106146_d_n7, assign69730_e106146_d_n8, assign69730_e106146_d_n9, assign69730_e106146_d_n10, assign69730_e106146_d_n11, assign69730_e106146_d_n14,) = {
    if (((locals.var_guard1630 != 0.0) && (locals.var_guard1629 == 0.0)) && (locals.var_guard1634 != 0.0)) {
        let assign69730_e106144: f64 = (-locals.var_vbsei);
        (assign69730_e106144, 0.0, (-locals.var_vbsei_dn2), 0.0, 0.0, 0.0, 0.0, 0.0, (-locals.var_vbsei_dn9), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn11, locals.var_vxbgmt_dn14,)
    }
};
        locals.var_vxbgmt = assign69730_e106146;
        locals.var_vxbgmt_dn0 = assign69730_e106146_d_n0;
        locals.var_vxbgmt_dn2 = assign69730_e106146_d_n2;
        locals.var_vxbgmt_dn4 = assign69730_e106146_d_n4;
        locals.var_vxbgmt_dn5 = assign69730_e106146_d_n5;
        locals.var_vxbgmt_dn6 = assign69730_e106146_d_n6;
        locals.var_vxbgmt_dn7 = assign69730_e106146_d_n7;
        locals.var_vxbgmt_dn8 = assign69730_e106146_d_n8;
        locals.var_vxbgmt_dn9 = assign69730_e106146_d_n9;
        locals.var_vxbgmt_dn10 = assign69730_e106146_d_n10;
        locals.var_vxbgmt_dn11 = assign69730_e106146_d_n11;
        locals.var_vxbgmt_dn14 = assign69730_e106146_d_n14;
        locals.var_vxbgmt_rv = 0.0;

        let assign69740_e106157: f64 = if (((p.p35 == 1.0) && (p.p63 > 0.0)) && (locals.var_uc_nover > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1635 = assign69740_e106157;
        locals.var_guard1635_rv = 0.0;

        let (assign69750_e106168,) = {
    if (((locals.var_guard1631 != 0.0) && (!((locals.var_guard1629 != 0.0) || (locals.var_guard1630 != 0.0)))) && (locals.var_guard1635 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_calcqover,)
    }
};
        locals.var_flg_calcqover = assign69750_e106168;
        locals.var_flg_calcqover_rv = 0.0;

        let (assign69760_e106179,) = {
    if (((locals.var_guard1631 != 0.0) && (!((locals.var_guard1629 != 0.0) || (locals.var_guard1630 != 0.0)))) && (locals.var_guard1635 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_coovlp,)
    }
};
        locals.var_flg_coovlp = assign69760_e106179;
        locals.var_flg_coovlp_rv = 0.0;

        let (assign69770_e106192, assign69770_e106192_d_n2, assign69770_e106192_d_n7, assign69770_e106192_d_n8, assign69770_e106192_d_n9,) = {
    if (((locals.var_guard1631 != 0.0) && (!((locals.var_guard1629 != 0.0) || (locals.var_guard1630 != 0.0)))) && (locals.var_guard1635 != 0.0)) {
        let assign69770_e106190: f64 = (locals.var_vgsi - locals.var_vbsi);
        (assign69770_e106190, 0.0, locals.var_vgsi_dn7, (locals.var_vgsi_dn8 - locals.var_vbsi_dn8), (-locals.var_vbsi_dn9),)
    } else {
        (locals.var_vgbgmt, locals.var_vgbgmt_dn2, locals.var_vgbgmt_dn7, locals.var_vgbgmt_dn8, locals.var_vgbgmt_dn9,)
    }
};
        locals.var_vgbgmt = assign69770_e106192;
        locals.var_vgbgmt_dn2 = assign69770_e106192_d_n2;
        locals.var_vgbgmt_dn7 = assign69770_e106192_d_n7;
        locals.var_vgbgmt_dn8 = assign69770_e106192_d_n8;
        locals.var_vgbgmt_dn9 = assign69770_e106192_d_n9;
        locals.var_vgbgmt_rv = 0.0;

        let (assign69780_e106205, assign69780_e106205_d_n0, assign69780_e106205_d_n2, assign69780_e106205_d_n4, assign69780_e106205_d_n5, assign69780_e106205_d_n6, assign69780_e106205_d_n7, assign69780_e106205_d_n8, assign69780_e106205_d_n9, assign69780_e106205_d_n10, assign69780_e106205_d_n11, assign69780_e106205_d_n14,) = {
    if (((locals.var_guard1631 != 0.0) && (!((locals.var_guard1629 != 0.0) || (locals.var_guard1630 != 0.0)))) && (locals.var_guard1635 != 0.0)) {
        let assign69780_e106203: f64 = (locals.var_vdsi - locals.var_vbsi);
        (assign69780_e106203, 0.0, 0.0, 0.0, 0.0, locals.var_vdsi_dn6, 0.0, (locals.var_vdsi_dn8 - locals.var_vbsi_dn8), (-locals.var_vbsi_dn9), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn11, locals.var_vxbgmt_dn14,)
    }
};
        locals.var_vxbgmt = assign69780_e106205;
        locals.var_vxbgmt_dn0 = assign69780_e106205_d_n0;
        locals.var_vxbgmt_dn2 = assign69780_e106205_d_n2;
        locals.var_vxbgmt_dn4 = assign69780_e106205_d_n4;
        locals.var_vxbgmt_dn5 = assign69780_e106205_d_n5;
        locals.var_vxbgmt_dn6 = assign69780_e106205_d_n6;
        locals.var_vxbgmt_dn7 = assign69780_e106205_d_n7;
        locals.var_vxbgmt_dn8 = assign69780_e106205_d_n8;
        locals.var_vxbgmt_dn9 = assign69780_e106205_d_n9;
        locals.var_vxbgmt_dn10 = assign69780_e106205_d_n10;
        locals.var_vxbgmt_dn11 = assign69780_e106205_d_n11;
        locals.var_vxbgmt_dn14 = assign69780_e106205_d_n14;
        locals.var_vxbgmt_rv = 0.0;

        let (assign69790_e106216,) = {
    if (((locals.var_guard1631 != 0.0) && (!((locals.var_guard1629 != 0.0) || (locals.var_guard1630 != 0.0)))) && (locals.var_guard1635 != 0.0)) {
        (locals.var_uc_nover,)
    } else {
        (locals.var_nover_func,)
    }
};
        locals.var_nover_func = assign69790_e106216;
        locals.var_nover_func_rv = 0.0;

        let (assign69800_e106231, assign69800_e106231_d_n0, assign69800_e106231_d_n2, assign69800_e106231_d_n4, assign69800_e106231_d_n5, assign69800_e106231_d_n6, assign69800_e106231_d_n7, assign69800_e106231_d_n8, assign69800_e106231_d_n9, assign69800_e106231_d_n10, assign69800_e106231_d_n11, assign69800_e106231_d_n14,) = {
    if (((locals.var_guard1631 != 0.0) && (!((locals.var_guard1629 != 0.0) || (locals.var_guard1630 != 0.0)))) && (locals.var_guard1635 != 0.0)) {
        let assign69800_e106228: f64 = (p.p64 * p.p55);
        let assign69800_e106229: f64 = (p.p63 + assign69800_e106228);
        (assign69800_e106229, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_lover_func, locals.var_lover_func_dn0, locals.var_lover_func_dn2, locals.var_lover_func_dn4, locals.var_lover_func_dn5, locals.var_lover_func_dn6, locals.var_lover_func_dn7, locals.var_lover_func_dn8, locals.var_lover_func_dn9, locals.var_lover_func_dn10, locals.var_lover_func_dn11, locals.var_lover_func_dn14,)
    }
};
        locals.var_lover_func = assign69800_e106231;
        locals.var_lover_func_dn0 = assign69800_e106231_d_n0;
        locals.var_lover_func_dn2 = assign69800_e106231_d_n2;
        locals.var_lover_func_dn4 = assign69800_e106231_d_n4;
        locals.var_lover_func_dn5 = assign69800_e106231_d_n5;
        locals.var_lover_func_dn6 = assign69800_e106231_d_n6;
        locals.var_lover_func_dn7 = assign69800_e106231_d_n7;
        locals.var_lover_func_dn8 = assign69800_e106231_d_n8;
        locals.var_lover_func_dn9 = assign69800_e106231_d_n9;
        locals.var_lover_func_dn10 = assign69800_e106231_d_n10;
        locals.var_lover_func_dn11 = assign69800_e106231_d_n11;
        locals.var_lover_func_dn14 = assign69800_e106231_d_n14;
        locals.var_lover_func_rv = 0.0;

        let (assign69810_e106242, assign69810_e106242_d_n0, assign69810_e106242_d_n2, assign69810_e106242_d_n4, assign69810_e106242_d_n5, assign69810_e106242_d_n6, assign69810_e106242_d_n7, assign69810_e106242_d_n8, assign69810_e106242_d_n9, assign69810_e106242_d_n10, assign69810_e106242_d_n11, assign69810_e106242_d_n14,) = {
    if (((locals.var_guard1631 != 0.0) && (!((locals.var_guard1629 != 0.0) || (locals.var_guard1630 != 0.0)))) && (locals.var_guard1635 != 0.0)) {
        (locals.var_wdep, locals.var_wdep_dn0, locals.var_wdep_dn2, locals.var_wdep_dn4, locals.var_wdep_dn5, locals.var_wdep_dn6, locals.var_wdep_dn7, locals.var_wdep_dn8, locals.var_wdep_dn9, locals.var_wdep_dn10, locals.var_wdep_dn11, locals.var_wdep_dn14,)
    } else {
        (locals.var_wdep_func, locals.var_wdep_func_dn0, locals.var_wdep_func_dn2, locals.var_wdep_func_dn4, locals.var_wdep_func_dn5, locals.var_wdep_func_dn6, locals.var_wdep_func_dn7, locals.var_wdep_func_dn8, locals.var_wdep_func_dn9, locals.var_wdep_func_dn10, locals.var_wdep_func_dn11, locals.var_wdep_func_dn14,)
    }
};
        locals.var_wdep_func = assign69810_e106242;
        locals.var_wdep_func_dn0 = assign69810_e106242_d_n0;
        locals.var_wdep_func_dn2 = assign69810_e106242_d_n2;
        locals.var_wdep_func_dn4 = assign69810_e106242_d_n4;
        locals.var_wdep_func_dn5 = assign69810_e106242_d_n5;
        locals.var_wdep_func_dn6 = assign69810_e106242_d_n6;
        locals.var_wdep_func_dn7 = assign69810_e106242_d_n7;
        locals.var_wdep_func_dn8 = assign69810_e106242_d_n8;
        locals.var_wdep_func_dn9 = assign69810_e106242_d_n9;
        locals.var_wdep_func_dn10 = assign69810_e106242_d_n10;
        locals.var_wdep_func_dn11 = assign69810_e106242_d_n11;
        locals.var_wdep_func_dn14 = assign69810_e106242_d_n14;
        locals.var_wdep_func_rv = 0.0;

        let (assign69820_e106253, assign69820_e106253_d_n0, assign69820_e106253_d_n2, assign69820_e106253_d_n4, assign69820_e106253_d_n5, assign69820_e106253_d_n6, assign69820_e106253_d_n7, assign69820_e106253_d_n8, assign69820_e106253_d_n9, assign69820_e106253_d_n10, assign69820_e106253_d_n11, assign69820_e106253_d_n14,) = {
    if (((locals.var_guard1631 != 0.0) && (!((locals.var_guard1629 != 0.0) || (locals.var_guard1630 != 0.0)))) && (locals.var_guard1635 != 0.0)) {
        (locals.var_cnst0over, locals.var_cnst0over_dn0, locals.var_cnst0over_dn2, locals.var_cnst0over_dn4, locals.var_cnst0over_dn5, locals.var_cnst0over_dn6, locals.var_cnst0over_dn7, locals.var_cnst0over_dn8, locals.var_cnst0over_dn9, locals.var_cnst0over_dn10, locals.var_cnst0over_dn11, locals.var_cnst0over_dn14,)
    } else {
        (locals.var_cnst0over_func, locals.var_cnst0over_func_dn0, locals.var_cnst0over_func_dn2, locals.var_cnst0over_func_dn4, locals.var_cnst0over_func_dn5, locals.var_cnst0over_func_dn6, locals.var_cnst0over_func_dn7, locals.var_cnst0over_func_dn8, locals.var_cnst0over_func_dn9, locals.var_cnst0over_func_dn10, locals.var_cnst0over_func_dn11, locals.var_cnst0over_func_dn14,)
    }
};
        locals.var_cnst0over_func = assign69820_e106253;
        locals.var_cnst0over_func_dn0 = assign69820_e106253_d_n0;
        locals.var_cnst0over_func_dn2 = assign69820_e106253_d_n2;
        locals.var_cnst0over_func_dn4 = assign69820_e106253_d_n4;
        locals.var_cnst0over_func_dn5 = assign69820_e106253_d_n5;
        locals.var_cnst0over_func_dn6 = assign69820_e106253_d_n6;
        locals.var_cnst0over_func_dn7 = assign69820_e106253_d_n7;
        locals.var_cnst0over_func_dn8 = assign69820_e106253_d_n8;
        locals.var_cnst0over_func_dn9 = assign69820_e106253_d_n9;
        locals.var_cnst0over_func_dn10 = assign69820_e106253_d_n10;
        locals.var_cnst0over_func_dn11 = assign69820_e106253_d_n11;
        locals.var_cnst0over_func_dn14 = assign69820_e106253_d_n14;
        locals.var_cnst0over_func_rv = 0.0;

        let (assign69830_e106264,) = {
    if (((locals.var_guard1631 != 0.0) && (!((locals.var_guard1629 != 0.0) || (locals.var_guard1630 != 0.0)))) && (locals.var_guard1635 != 0.0)) {
        (locals.var_coxb0,)
    } else {
        (locals.var_cox0_func,)
    }
};
        locals.var_cox0_func = assign69830_e106264;
        locals.var_cox0_func_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_262(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign69840_e106276, assign69840_e106276_d_n0, assign69840_e106276_d_n2, assign69840_e106276_d_n4, assign69840_e106276_d_n5, assign69840_e106276_d_n6, assign69840_e106276_d_n7, assign69840_e106276_d_n8, assign69840_e106276_d_n9, assign69840_e106276_d_n10, assign69840_e106276_d_n11, assign69840_e106276_d_n14,) = {
    if (((locals.var_guard1631 != 0.0) && (!((locals.var_guard1629 != 0.0) || (locals.var_guard1630 != 0.0)))) && (locals.var_guard1635 != 0.0)) {
        let assign69840_e106274: f64 = (-locals.var_lover_func);
        (assign69840_e106274, (-locals.var_lover_func_dn0), (-locals.var_lover_func_dn2), (-locals.var_lover_func_dn4), (-locals.var_lover_func_dn5), (-locals.var_lover_func_dn6), (-locals.var_lover_func_dn7), (-locals.var_lover_func_dn8), (-locals.var_lover_func_dn9), (-locals.var_lover_func_dn10), (-locals.var_lover_func_dn11), (-locals.var_lover_func_dn14),)
    } else {
        (locals.var_lover_func, locals.var_lover_func_dn0, locals.var_lover_func_dn2, locals.var_lover_func_dn4, locals.var_lover_func_dn5, locals.var_lover_func_dn6, locals.var_lover_func_dn7, locals.var_lover_func_dn8, locals.var_lover_func_dn9, locals.var_lover_func_dn10, locals.var_lover_func_dn11, locals.var_lover_func_dn14,)
    }
};
        locals.var_lover_func = assign69840_e106276;
        locals.var_lover_func_dn0 = assign69840_e106276_d_n0;
        locals.var_lover_func_dn2 = assign69840_e106276_d_n2;
        locals.var_lover_func_dn4 = assign69840_e106276_d_n4;
        locals.var_lover_func_dn5 = assign69840_e106276_d_n5;
        locals.var_lover_func_dn6 = assign69840_e106276_d_n6;
        locals.var_lover_func_dn7 = assign69840_e106276_d_n7;
        locals.var_lover_func_dn8 = assign69840_e106276_d_n8;
        locals.var_lover_func_dn9 = assign69840_e106276_d_n9;
        locals.var_lover_func_dn10 = assign69840_e106276_d_n10;
        locals.var_lover_func_dn11 = assign69840_e106276_d_n11;
        locals.var_lover_func_dn14 = assign69840_e106276_d_n14;
        locals.var_lover_func_rv = 0.0;

        let assign69850_e106287: f64 = if (((locals.var_lover_func < 0.0) && (p.p432 > 0.0)) && (p.p55 == 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard1636 = assign69850_e106287;
        locals.var_guard1636_rv = 0.0;

        let (assign69860_e106301, assign69860_e106301_d_n0, assign69860_e106301_d_n2, assign69860_e106301_d_n4, assign69860_e106301_d_n5, assign69860_e106301_d_n6, assign69860_e106301_d_n7, assign69860_e106301_d_n8, assign69860_e106301_d_n9, assign69860_e106301_d_n10, assign69860_e106301_d_n11, assign69860_e106301_d_n14,) = {
    if ((((locals.var_guard1631 != 0.0) && (!((locals.var_guard1629 != 0.0) || (locals.var_guard1630 != 0.0)))) && (locals.var_guard1635 != 0.0)) && (locals.var_guard1636 != 0.0)) {
        let assign69860_e106299: f64 = (-locals.var_lover_func);
        (assign69860_e106299, (-locals.var_lover_func_dn0), (-locals.var_lover_func_dn2), (-locals.var_lover_func_dn4), (-locals.var_lover_func_dn5), (-locals.var_lover_func_dn6), (-locals.var_lover_func_dn7), (-locals.var_lover_func_dn8), (-locals.var_lover_func_dn9), (-locals.var_lover_func_dn10), (-locals.var_lover_func_dn11), (-locals.var_lover_func_dn14),)
    } else {
        (locals.var_lover_func, locals.var_lover_func_dn0, locals.var_lover_func_dn2, locals.var_lover_func_dn4, locals.var_lover_func_dn5, locals.var_lover_func_dn6, locals.var_lover_func_dn7, locals.var_lover_func_dn8, locals.var_lover_func_dn9, locals.var_lover_func_dn10, locals.var_lover_func_dn11, locals.var_lover_func_dn14,)
    }
};
        locals.var_lover_func = assign69860_e106301;
        locals.var_lover_func_dn0 = assign69860_e106301_d_n0;
        locals.var_lover_func_dn2 = assign69860_e106301_d_n2;
        locals.var_lover_func_dn4 = assign69860_e106301_d_n4;
        locals.var_lover_func_dn5 = assign69860_e106301_d_n5;
        locals.var_lover_func_dn6 = assign69860_e106301_d_n6;
        locals.var_lover_func_dn7 = assign69860_e106301_d_n7;
        locals.var_lover_func_dn8 = assign69860_e106301_d_n8;
        locals.var_lover_func_dn9 = assign69860_e106301_d_n9;
        locals.var_lover_func_dn10 = assign69860_e106301_d_n10;
        locals.var_lover_func_dn11 = assign69860_e106301_d_n11;
        locals.var_lover_func_dn14 = assign69860_e106301_d_n14;
        locals.var_lover_func_rv = 0.0;

        let (assign69870_e106314, assign69870_e106314_d_n0, assign69870_e106314_d_n2, assign69870_e106314_d_n4, assign69870_e106314_d_n5, assign69870_e106314_d_n6, assign69870_e106314_d_n7, assign69870_e106314_d_n8, assign69870_e106314_d_n9, assign69870_e106314_d_n10, assign69870_e106314_d_n11, assign69870_e106314_d_n14,) = {
    if ((((locals.var_guard1631 != 0.0) && (!((locals.var_guard1629 != 0.0) || (locals.var_guard1630 != 0.0)))) && (locals.var_guard1635 != 0.0)) && (locals.var_guard1636 != 0.0)) {
        (p.p63, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign69870_e106314;
        locals.var_t1_dn0 = assign69870_e106314_d_n0;
        locals.var_t1_dn2 = assign69870_e106314_d_n2;
        locals.var_t1_dn4 = assign69870_e106314_d_n4;
        locals.var_t1_dn5 = assign69870_e106314_d_n5;
        locals.var_t1_dn6 = assign69870_e106314_d_n6;
        locals.var_t1_dn7 = assign69870_e106314_d_n7;
        locals.var_t1_dn8 = assign69870_e106314_d_n8;
        locals.var_t1_dn9 = assign69870_e106314_d_n9;
        locals.var_t1_dn10 = assign69870_e106314_d_n10;
        locals.var_t1_dn11 = assign69870_e106314_d_n11;
        locals.var_t1_dn14 = assign69870_e106314_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign69880_e106333, assign69880_e106333_d_n0, assign69880_e106333_d_n2, assign69880_e106333_d_n4, assign69880_e106333_d_n5, assign69880_e106333_d_n6, assign69880_e106333_d_n7, assign69880_e106333_d_n8, assign69880_e106333_d_n9, assign69880_e106333_d_n10, assign69880_e106333_d_n11, assign69880_e106333_d_n14,) = {
    if ((((locals.var_guard1631 != 0.0) && (!((locals.var_guard1629 != 0.0) || (locals.var_guard1630 != 0.0)))) && (locals.var_guard1635 != 0.0)) && (locals.var_guard1636 != 0.0)) {
        let assign69880_e106327: f64 = (locals.var_t1 * locals.var_t1);
        let assign69880_e106329: f64 = (assign69880_e106327 / locals.var_kjunc);
        let assign69880_e106331: f64 = (assign69880_e106329 - p.p137);
        (assign69880_e106331, (((((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) * locals.var_kjunc) - (assign69880_e106327 * locals.var_kjunc_dn0)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) * locals.var_kjunc) - (assign69880_e106327 * locals.var_kjunc_dn2)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) * locals.var_kjunc) - (assign69880_e106327 * locals.var_kjunc_dn4)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) * locals.var_kjunc) - (assign69880_e106327 * locals.var_kjunc_dn5)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) * locals.var_kjunc) - (assign69880_e106327 * locals.var_kjunc_dn6)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) * locals.var_kjunc) - (assign69880_e106327 * locals.var_kjunc_dn7)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) * locals.var_kjunc) - (assign69880_e106327 * locals.var_kjunc_dn8)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) * locals.var_kjunc) - (assign69880_e106327 * locals.var_kjunc_dn9)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) * locals.var_kjunc) - (assign69880_e106327 * locals.var_kjunc_dn10)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) * locals.var_kjunc) - (assign69880_e106327 * locals.var_kjunc_dn11)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14)) * locals.var_kjunc) - (assign69880_e106327 * locals.var_kjunc_dn14)) / (locals.var_kjunc * locals.var_kjunc)),)
    } else {
        (locals.var_vxb_lim, locals.var_vxb_lim_dn0, locals.var_vxb_lim_dn2, locals.var_vxb_lim_dn4, locals.var_vxb_lim_dn5, locals.var_vxb_lim_dn6, locals.var_vxb_lim_dn7, locals.var_vxb_lim_dn8, locals.var_vxb_lim_dn9, locals.var_vxb_lim_dn10, locals.var_vxb_lim_dn11, locals.var_vxb_lim_dn14,)
    }
};
        locals.var_vxb_lim = assign69880_e106333;
        locals.var_vxb_lim_dn0 = assign69880_e106333_d_n0;
        locals.var_vxb_lim_dn2 = assign69880_e106333_d_n2;
        locals.var_vxb_lim_dn4 = assign69880_e106333_d_n4;
        locals.var_vxb_lim_dn5 = assign69880_e106333_d_n5;
        locals.var_vxb_lim_dn6 = assign69880_e106333_d_n6;
        locals.var_vxb_lim_dn7 = assign69880_e106333_d_n7;
        locals.var_vxb_lim_dn8 = assign69880_e106333_d_n8;
        locals.var_vxb_lim_dn9 = assign69880_e106333_d_n9;
        locals.var_vxb_lim_dn10 = assign69880_e106333_d_n10;
        locals.var_vxb_lim_dn11 = assign69880_e106333_d_n11;
        locals.var_vxb_lim_dn14 = assign69880_e106333_d_n14;
        locals.var_vxb_lim_rv = 0.0;

        let assign69890_e106336: f64 = if p.p113 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1637 = assign69890_e106336;
        locals.var_guard1637_rv = 0.0;

        let assign69900_e106343: f64 = if ((locals.var_vxbgmt == 0.0) || (p.p113 <= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1638 = assign69900_e106343;
        locals.var_guard1638_rv = 0.0;

        let (assign69910_e106360, assign69910_e106360_d_n0, assign69910_e106360_d_n2, assign69910_e106360_d_n4, assign69910_e106360_d_n5, assign69910_e106360_d_n6, assign69910_e106360_d_n7, assign69910_e106360_d_n8, assign69910_e106360_d_n9, assign69910_e106360_d_n10, assign69910_e106360_d_n11, assign69910_e106360_d_n14,) = {
    if ((((((locals.var_guard1631 != 0.0) && (!((locals.var_guard1629 != 0.0) || (locals.var_guard1630 != 0.0)))) && (locals.var_guard1635 != 0.0)) && (locals.var_guard1636 != 0.0)) && (locals.var_guard1637 != 0.0)) && (locals.var_guard1638 != 0.0)) {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn11, locals.var_vxbgmt_dn14,)
    } else {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn11, locals.var_vxbgmt_dn14,)
    }
};
        locals.var_vxbgmt = assign69910_e106360;
        locals.var_vxbgmt_dn0 = assign69910_e106360_d_n0;
        locals.var_vxbgmt_dn2 = assign69910_e106360_d_n2;
        locals.var_vxbgmt_dn4 = assign69910_e106360_d_n4;
        locals.var_vxbgmt_dn5 = assign69910_e106360_d_n5;
        locals.var_vxbgmt_dn6 = assign69910_e106360_d_n6;
        locals.var_vxbgmt_dn7 = assign69910_e106360_d_n7;
        locals.var_vxbgmt_dn8 = assign69910_e106360_d_n8;
        locals.var_vxbgmt_dn9 = assign69910_e106360_d_n9;
        locals.var_vxbgmt_dn10 = assign69910_e106360_d_n10;
        locals.var_vxbgmt_dn11 = assign69910_e106360_d_n11;
        locals.var_vxbgmt_dn14 = assign69910_e106360_d_n14;
        locals.var_vxbgmt_rv = 0.0;

        let (assign69920_e106384, assign69920_e106384_d_n0, assign69920_e106384_d_n2, assign69920_e106384_d_n4, assign69920_e106384_d_n5, assign69920_e106384_d_n6, assign69920_e106384_d_n7, assign69920_e106384_d_n8, assign69920_e106384_d_n9, assign69920_e106384_d_n10, assign69920_e106384_d_n11, assign69920_e106384_d_n14,) = {
    if ((((((locals.var_guard1631 != 0.0) && (!((locals.var_guard1629 != 0.0) || (locals.var_guard1630 != 0.0)))) && (locals.var_guard1635 != 0.0)) && (locals.var_guard1636 != 0.0)) && (locals.var_guard1637 != 0.0)) && (locals.var_guard1638 == 0.0)) {
        let (assign69920_e106382,) = {
            if (locals.var_vxbgmt < 0.0) {
                let assign69920_e106380: f64 = (-1.0);
                (assign69920_e106380,)
            } else {
                (1.0,)
            }
        };
        (assign69920_e106382, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf3, locals.var_tmf3_dn0, locals.var_tmf3_dn2, locals.var_tmf3_dn4, locals.var_tmf3_dn5, locals.var_tmf3_dn6, locals.var_tmf3_dn7, locals.var_tmf3_dn8, locals.var_tmf3_dn9, locals.var_tmf3_dn10, locals.var_tmf3_dn11, locals.var_tmf3_dn14,)
    }
};
        locals.var_tmf3 = assign69920_e106384;
        locals.var_tmf3_dn0 = assign69920_e106384_d_n0;
        locals.var_tmf3_dn2 = assign69920_e106384_d_n2;
        locals.var_tmf3_dn4 = assign69920_e106384_d_n4;
        locals.var_tmf3_dn5 = assign69920_e106384_d_n5;
        locals.var_tmf3_dn6 = assign69920_e106384_d_n6;
        locals.var_tmf3_dn7 = assign69920_e106384_d_n7;
        locals.var_tmf3_dn8 = assign69920_e106384_d_n8;
        locals.var_tmf3_dn9 = assign69920_e106384_d_n9;
        locals.var_tmf3_dn10 = assign69920_e106384_d_n10;
        locals.var_tmf3_dn11 = assign69920_e106384_d_n11;
        locals.var_tmf3_dn14 = assign69920_e106384_d_n14;
        locals.var_tmf3_rv = 0.0;

        let (assign69930_e106404, assign69930_e106404_d_n0, assign69930_e106404_d_n2, assign69930_e106404_d_n4, assign69930_e106404_d_n5, assign69930_e106404_d_n6, assign69930_e106404_d_n7, assign69930_e106404_d_n8, assign69930_e106404_d_n9, assign69930_e106404_d_n10, assign69930_e106404_d_n11, assign69930_e106404_d_n14,) = {
    if ((((((locals.var_guard1631 != 0.0) && (!((locals.var_guard1629 != 0.0) || (locals.var_guard1630 != 0.0)))) && (locals.var_guard1635 != 0.0)) && (locals.var_guard1636 != 0.0)) && (locals.var_guard1637 != 0.0)) && (locals.var_guard1638 == 0.0)) {
        let assign69930_e106402: f64 = (locals.var_tmf3 * locals.var_vxbgmt);
        (assign69930_e106402, ((locals.var_tmf3_dn0 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn0)), ((locals.var_tmf3_dn2 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn2)), ((locals.var_tmf3_dn4 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn4)), ((locals.var_tmf3_dn5 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn5)), ((locals.var_tmf3_dn6 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn6)), ((locals.var_tmf3_dn7 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn7)), ((locals.var_tmf3_dn8 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn8)), ((locals.var_tmf3_dn9 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn9)), ((locals.var_tmf3_dn10 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn10)), ((locals.var_tmf3_dn11 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn11)), ((locals.var_tmf3_dn14 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn14)),)
    } else {
        (locals.var_tmf4, locals.var_tmf4_dn0, locals.var_tmf4_dn2, locals.var_tmf4_dn4, locals.var_tmf4_dn5, locals.var_tmf4_dn6, locals.var_tmf4_dn7, locals.var_tmf4_dn8, locals.var_tmf4_dn9, locals.var_tmf4_dn10, locals.var_tmf4_dn11, locals.var_tmf4_dn14,)
    }
};
        locals.var_tmf4 = assign69930_e106404;
        locals.var_tmf4_dn0 = assign69930_e106404_d_n0;
        locals.var_tmf4_dn2 = assign69930_e106404_d_n2;
        locals.var_tmf4_dn4 = assign69930_e106404_d_n4;
        locals.var_tmf4_dn5 = assign69930_e106404_d_n5;
        locals.var_tmf4_dn6 = assign69930_e106404_d_n6;
        locals.var_tmf4_dn7 = assign69930_e106404_d_n7;
        locals.var_tmf4_dn8 = assign69930_e106404_d_n8;
        locals.var_tmf4_dn9 = assign69930_e106404_d_n9;
        locals.var_tmf4_dn10 = assign69930_e106404_d_n10;
        locals.var_tmf4_dn11 = assign69930_e106404_d_n11;
        locals.var_tmf4_dn14 = assign69930_e106404_d_n14;
        locals.var_tmf4_rv = 0.0;

        let (assign69940_e106428, assign69940_e106428_d_n0, assign69940_e106428_d_n2, assign69940_e106428_d_n4, assign69940_e106428_d_n5, assign69940_e106428_d_n6, assign69940_e106428_d_n7, assign69940_e106428_d_n8, assign69940_e106428_d_n9, assign69940_e106428_d_n10, assign69940_e106428_d_n11, assign69940_e106428_d_n14,) = {
    if ((((((locals.var_guard1631 != 0.0) && (!((locals.var_guard1629 != 0.0) || (locals.var_guard1630 != 0.0)))) && (locals.var_guard1635 != 0.0)) && (locals.var_guard1636 != 0.0)) && (locals.var_guard1637 != 0.0)) && (locals.var_guard1638 == 0.0)) {
        let assign69940_e106423: f64 = (locals.var_tmf4 / locals.var_vxb_lim);
        let assign69940_e106425: f64 = (assign69940_e106423).powf(p.p113);
        let assign69940_e106426: f64 = (1.0 + assign69940_e106425);
        (assign69940_e106426, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign69940_e106423).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn0 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn0)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign69940_e106425 * (p.p113 * ((((locals.var_tmf4_dn0 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn0)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign69940_e106423))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign69940_e106423).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn2 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn2)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign69940_e106425 * (p.p113 * ((((locals.var_tmf4_dn2 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn2)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign69940_e106423))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign69940_e106423).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn4 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn4)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign69940_e106425 * (p.p113 * ((((locals.var_tmf4_dn4 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn4)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign69940_e106423))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign69940_e106423).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn5 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn5)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign69940_e106425 * (p.p113 * ((((locals.var_tmf4_dn5 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn5)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign69940_e106423))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign69940_e106423).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn6 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn6)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign69940_e106425 * (p.p113 * ((((locals.var_tmf4_dn6 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn6)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign69940_e106423))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign69940_e106423).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn7 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn7)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign69940_e106425 * (p.p113 * ((((locals.var_tmf4_dn7 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn7)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign69940_e106423))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign69940_e106423).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn8 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn8)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign69940_e106425 * (p.p113 * ((((locals.var_tmf4_dn8 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn8)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign69940_e106423))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign69940_e106423).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn9 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn9)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign69940_e106425 * (p.p113 * ((((locals.var_tmf4_dn9 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn9)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign69940_e106423))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign69940_e106423).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn10 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn10)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign69940_e106425 * (p.p113 * ((((locals.var_tmf4_dn10 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn10)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign69940_e106423))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign69940_e106423).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn11 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn11)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign69940_e106425 * (p.p113 * ((((locals.var_tmf4_dn11 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn11)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign69940_e106423))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign69940_e106423).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn14 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn14)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign69940_e106425 * (p.p113 * ((((locals.var_tmf4_dn14 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn14)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign69940_e106423))) },)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign69940_e106428;
        locals.var_tmf1_dn0 = assign69940_e106428_d_n0;
        locals.var_tmf1_dn2 = assign69940_e106428_d_n2;
        locals.var_tmf1_dn4 = assign69940_e106428_d_n4;
        locals.var_tmf1_dn5 = assign69940_e106428_d_n5;
        locals.var_tmf1_dn6 = assign69940_e106428_d_n6;
        locals.var_tmf1_dn7 = assign69940_e106428_d_n7;
        locals.var_tmf1_dn8 = assign69940_e106428_d_n8;
        locals.var_tmf1_dn9 = assign69940_e106428_d_n9;
        locals.var_tmf1_dn10 = assign69940_e106428_d_n10;
        locals.var_tmf1_dn11 = assign69940_e106428_d_n11;
        locals.var_tmf1_dn14 = assign69940_e106428_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign69950_e106450, assign69950_e106450_d_n0, assign69950_e106450_d_n2, assign69950_e106450_d_n4, assign69950_e106450_d_n5, assign69950_e106450_d_n6, assign69950_e106450_d_n7, assign69950_e106450_d_n8, assign69950_e106450_d_n9, assign69950_e106450_d_n10, assign69950_e106450_d_n11, assign69950_e106450_d_n14,) = {
    if ((((((locals.var_guard1631 != 0.0) && (!((locals.var_guard1629 != 0.0) || (locals.var_guard1630 != 0.0)))) && (locals.var_guard1635 != 0.0)) && (locals.var_guard1636 != 0.0)) && (locals.var_guard1637 != 0.0)) && (locals.var_guard1638 == 0.0)) {
        let assign69950_e106447: f64 = (1.0 / p.p113);
        let assign69950_e106448: f64 = (locals.var_tmf1).powf(assign69950_e106447);
        (assign69950_e106448, if 0.0 == 0.0 && ((assign69950_e106447) as f64).is_finite() && ((assign69950_e106447) as f64).fract() == 0.0 { if assign69950_e106447 == 0.0 { 0.0 } else { (assign69950_e106447 * ((locals.var_tmf1).powf(assign69950_e106447 - 1.0) * locals.var_tmf1_dn0)) } } else { (assign69950_e106448 * (assign69950_e106447 * (locals.var_tmf1_dn0 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign69950_e106447) as f64).is_finite() && ((assign69950_e106447) as f64).fract() == 0.0 { if assign69950_e106447 == 0.0 { 0.0 } else { (assign69950_e106447 * ((locals.var_tmf1).powf(assign69950_e106447 - 1.0) * locals.var_tmf1_dn2)) } } else { (assign69950_e106448 * (assign69950_e106447 * (locals.var_tmf1_dn2 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign69950_e106447) as f64).is_finite() && ((assign69950_e106447) as f64).fract() == 0.0 { if assign69950_e106447 == 0.0 { 0.0 } else { (assign69950_e106447 * ((locals.var_tmf1).powf(assign69950_e106447 - 1.0) * locals.var_tmf1_dn4)) } } else { (assign69950_e106448 * (assign69950_e106447 * (locals.var_tmf1_dn4 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign69950_e106447) as f64).is_finite() && ((assign69950_e106447) as f64).fract() == 0.0 { if assign69950_e106447 == 0.0 { 0.0 } else { (assign69950_e106447 * ((locals.var_tmf1).powf(assign69950_e106447 - 1.0) * locals.var_tmf1_dn5)) } } else { (assign69950_e106448 * (assign69950_e106447 * (locals.var_tmf1_dn5 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign69950_e106447) as f64).is_finite() && ((assign69950_e106447) as f64).fract() == 0.0 { if assign69950_e106447 == 0.0 { 0.0 } else { (assign69950_e106447 * ((locals.var_tmf1).powf(assign69950_e106447 - 1.0) * locals.var_tmf1_dn6)) } } else { (assign69950_e106448 * (assign69950_e106447 * (locals.var_tmf1_dn6 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign69950_e106447) as f64).is_finite() && ((assign69950_e106447) as f64).fract() == 0.0 { if assign69950_e106447 == 0.0 { 0.0 } else { (assign69950_e106447 * ((locals.var_tmf1).powf(assign69950_e106447 - 1.0) * locals.var_tmf1_dn7)) } } else { (assign69950_e106448 * (assign69950_e106447 * (locals.var_tmf1_dn7 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign69950_e106447) as f64).is_finite() && ((assign69950_e106447) as f64).fract() == 0.0 { if assign69950_e106447 == 0.0 { 0.0 } else { (assign69950_e106447 * ((locals.var_tmf1).powf(assign69950_e106447 - 1.0) * locals.var_tmf1_dn8)) } } else { (assign69950_e106448 * (assign69950_e106447 * (locals.var_tmf1_dn8 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign69950_e106447) as f64).is_finite() && ((assign69950_e106447) as f64).fract() == 0.0 { if assign69950_e106447 == 0.0 { 0.0 } else { (assign69950_e106447 * ((locals.var_tmf1).powf(assign69950_e106447 - 1.0) * locals.var_tmf1_dn9)) } } else { (assign69950_e106448 * (assign69950_e106447 * (locals.var_tmf1_dn9 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign69950_e106447) as f64).is_finite() && ((assign69950_e106447) as f64).fract() == 0.0 { if assign69950_e106447 == 0.0 { 0.0 } else { (assign69950_e106447 * ((locals.var_tmf1).powf(assign69950_e106447 - 1.0) * locals.var_tmf1_dn10)) } } else { (assign69950_e106448 * (assign69950_e106447 * (locals.var_tmf1_dn10 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign69950_e106447) as f64).is_finite() && ((assign69950_e106447) as f64).fract() == 0.0 { if assign69950_e106447 == 0.0 { 0.0 } else { (assign69950_e106447 * ((locals.var_tmf1).powf(assign69950_e106447 - 1.0) * locals.var_tmf1_dn11)) } } else { (assign69950_e106448 * (assign69950_e106447 * (locals.var_tmf1_dn11 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign69950_e106447) as f64).is_finite() && ((assign69950_e106447) as f64).fract() == 0.0 { if assign69950_e106447 == 0.0 { 0.0 } else { (assign69950_e106447 * ((locals.var_tmf1).powf(assign69950_e106447 - 1.0) * locals.var_tmf1_dn14)) } } else { (assign69950_e106448 * (assign69950_e106447 * (locals.var_tmf1_dn14 / locals.var_tmf1))) },)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign69950_e106450;
        locals.var_tmf2_dn0 = assign69950_e106450_d_n0;
        locals.var_tmf2_dn2 = assign69950_e106450_d_n2;
        locals.var_tmf2_dn4 = assign69950_e106450_d_n4;
        locals.var_tmf2_dn5 = assign69950_e106450_d_n5;
        locals.var_tmf2_dn6 = assign69950_e106450_d_n6;
        locals.var_tmf2_dn7 = assign69950_e106450_d_n7;
        locals.var_tmf2_dn8 = assign69950_e106450_d_n8;
        locals.var_tmf2_dn9 = assign69950_e106450_d_n9;
        locals.var_tmf2_dn10 = assign69950_e106450_d_n10;
        locals.var_tmf2_dn11 = assign69950_e106450_d_n11;
        locals.var_tmf2_dn14 = assign69950_e106450_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign69960_e106472, assign69960_e106472_d_n0, assign69960_e106472_d_n2, assign69960_e106472_d_n4, assign69960_e106472_d_n5, assign69960_e106472_d_n6, assign69960_e106472_d_n7, assign69960_e106472_d_n8, assign69960_e106472_d_n9, assign69960_e106472_d_n10, assign69960_e106472_d_n11, assign69960_e106472_d_n14,) = {
    if ((((((locals.var_guard1631 != 0.0) && (!((locals.var_guard1629 != 0.0) || (locals.var_guard1630 != 0.0)))) && (locals.var_guard1635 != 0.0)) && (locals.var_guard1636 != 0.0)) && (locals.var_guard1637 != 0.0)) && (locals.var_guard1638 == 0.0)) {
        let assign69960_e106468: f64 = (locals.var_tmf3 * locals.var_tmf4);
        let assign69960_e106470: f64 = (assign69960_e106468 / locals.var_tmf2);
        (assign69960_e106470, (((((locals.var_tmf3_dn0 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn0)) * locals.var_tmf2) - (assign69960_e106468 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn2 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn2)) * locals.var_tmf2) - (assign69960_e106468 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn4 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn4)) * locals.var_tmf2) - (assign69960_e106468 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn5 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn5)) * locals.var_tmf2) - (assign69960_e106468 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn6 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn6)) * locals.var_tmf2) - (assign69960_e106468 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn7 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn7)) * locals.var_tmf2) - (assign69960_e106468 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn8 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn8)) * locals.var_tmf2) - (assign69960_e106468 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn9 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn9)) * locals.var_tmf2) - (assign69960_e106468 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn10 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn10)) * locals.var_tmf2) - (assign69960_e106468 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn11 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn11)) * locals.var_tmf2) - (assign69960_e106468 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn14 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn14)) * locals.var_tmf2) - (assign69960_e106468 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2)),)
    } else {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn11, locals.var_vxbgmt_dn14,)
    }
};
        locals.var_vxbgmt = assign69960_e106472;
        locals.var_vxbgmt_dn0 = assign69960_e106472_d_n0;
        locals.var_vxbgmt_dn2 = assign69960_e106472_d_n2;
        locals.var_vxbgmt_dn4 = assign69960_e106472_d_n4;
        locals.var_vxbgmt_dn5 = assign69960_e106472_d_n5;
        locals.var_vxbgmt_dn6 = assign69960_e106472_d_n6;
        locals.var_vxbgmt_dn7 = assign69960_e106472_d_n7;
        locals.var_vxbgmt_dn8 = assign69960_e106472_d_n8;
        locals.var_vxbgmt_dn9 = assign69960_e106472_d_n9;
        locals.var_vxbgmt_dn10 = assign69960_e106472_d_n10;
        locals.var_vxbgmt_dn11 = assign69960_e106472_d_n11;
        locals.var_vxbgmt_dn14 = assign69960_e106472_d_n14;
        locals.var_vxbgmt_rv = 0.0;

        let (assign69970_e106500, assign69970_e106500_d_n0, assign69970_e106500_d_n2, assign69970_e106500_d_n4, assign69970_e106500_d_n5, assign69970_e106500_d_n6, assign69970_e106500_d_n7, assign69970_e106500_d_n8, assign69970_e106500_d_n9, assign69970_e106500_d_n10, assign69970_e106500_d_n11, assign69970_e106500_d_n14,) = {
    if (((((locals.var_guard1631 != 0.0) && (!((locals.var_guard1629 != 0.0) || (locals.var_guard1630 != 0.0)))) && (locals.var_guard1635 != 0.0)) && (locals.var_guard1636 != 0.0)) && (locals.var_guard1637 != 0.0)) {
        let assign69970_e106487: f64 = (locals.var_vxbgmt + p.p137);
        let assign69970_e106490: f64 = (locals.var_vxbgmt + p.p137);
        let assign69970_e106491: f64 = (assign69970_e106487 * assign69970_e106490);
        let assign69970_e106494: f64 = (4.0 * 0.1);
        let assign69970_e106496: f64 = (assign69970_e106494 * 0.1);
        let assign69970_e106497: f64 = (assign69970_e106491 + assign69970_e106496);
        let assign69970_e106498: f64 = (assign69970_e106497).sqrt();
        (assign69970_e106498, (((locals.var_vxbgmt_dn0 * assign69970_e106490) + (assign69970_e106487 * locals.var_vxbgmt_dn0)) / (2.0 * assign69970_e106498)), (((locals.var_vxbgmt_dn2 * assign69970_e106490) + (assign69970_e106487 * locals.var_vxbgmt_dn2)) / (2.0 * assign69970_e106498)), (((locals.var_vxbgmt_dn4 * assign69970_e106490) + (assign69970_e106487 * locals.var_vxbgmt_dn4)) / (2.0 * assign69970_e106498)), (((locals.var_vxbgmt_dn5 * assign69970_e106490) + (assign69970_e106487 * locals.var_vxbgmt_dn5)) / (2.0 * assign69970_e106498)), (((locals.var_vxbgmt_dn6 * assign69970_e106490) + (assign69970_e106487 * locals.var_vxbgmt_dn6)) / (2.0 * assign69970_e106498)), (((locals.var_vxbgmt_dn7 * assign69970_e106490) + (assign69970_e106487 * locals.var_vxbgmt_dn7)) / (2.0 * assign69970_e106498)), (((locals.var_vxbgmt_dn8 * assign69970_e106490) + (assign69970_e106487 * locals.var_vxbgmt_dn8)) / (2.0 * assign69970_e106498)), (((locals.var_vxbgmt_dn9 * assign69970_e106490) + (assign69970_e106487 * locals.var_vxbgmt_dn9)) / (2.0 * assign69970_e106498)), (((locals.var_vxbgmt_dn10 * assign69970_e106490) + (assign69970_e106487 * locals.var_vxbgmt_dn10)) / (2.0 * assign69970_e106498)), (((locals.var_vxbgmt_dn11 * assign69970_e106490) + (assign69970_e106487 * locals.var_vxbgmt_dn11)) / (2.0 * assign69970_e106498)), (((locals.var_vxbgmt_dn14 * assign69970_e106490) + (assign69970_e106487 * locals.var_vxbgmt_dn14)) / (2.0 * assign69970_e106498)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign69970_e106500;
        locals.var_tmf2_dn0 = assign69970_e106500_d_n0;
        locals.var_tmf2_dn2 = assign69970_e106500_d_n2;
        locals.var_tmf2_dn4 = assign69970_e106500_d_n4;
        locals.var_tmf2_dn5 = assign69970_e106500_d_n5;
        locals.var_tmf2_dn6 = assign69970_e106500_d_n6;
        locals.var_tmf2_dn7 = assign69970_e106500_d_n7;
        locals.var_tmf2_dn8 = assign69970_e106500_d_n8;
        locals.var_tmf2_dn9 = assign69970_e106500_d_n9;
        locals.var_tmf2_dn10 = assign69970_e106500_d_n10;
        locals.var_tmf2_dn11 = assign69970_e106500_d_n11;
        locals.var_tmf2_dn14 = assign69970_e106500_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign69980_e106523, assign69980_e106523_d_n0, assign69980_e106523_d_n2, assign69980_e106523_d_n4, assign69980_e106523_d_n5, assign69980_e106523_d_n6, assign69980_e106523_d_n7, assign69980_e106523_d_n8, assign69980_e106523_d_n9, assign69980_e106523_d_n10, assign69980_e106523_d_n11, assign69980_e106523_d_n14,) = {
    if (((((locals.var_guard1631 != 0.0) && (!((locals.var_guard1629 != 0.0) || (locals.var_guard1630 != 0.0)))) && (locals.var_guard1635 != 0.0)) && (locals.var_guard1636 != 0.0)) && (locals.var_guard1637 != 0.0)) {
        let assign69980_e106517: f64 = (locals.var_vxbgmt + p.p137);
        let assign69980_e106519: f64 = (assign69980_e106517 / locals.var_tmf2);
        let assign69980_e106520: f64 = (1.0 + assign69980_e106519);
        let assign69980_e106521: f64 = (0.5 * assign69980_e106520);
        (assign69980_e106521, (0.5 * (((locals.var_vxbgmt_dn0 * locals.var_tmf2) - (assign69980_e106517 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn2 * locals.var_tmf2) - (assign69980_e106517 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn4 * locals.var_tmf2) - (assign69980_e106517 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn5 * locals.var_tmf2) - (assign69980_e106517 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn6 * locals.var_tmf2) - (assign69980_e106517 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn7 * locals.var_tmf2) - (assign69980_e106517 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn8 * locals.var_tmf2) - (assign69980_e106517 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn9 * locals.var_tmf2) - (assign69980_e106517 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn10 * locals.var_tmf2) - (assign69980_e106517 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn11 * locals.var_tmf2) - (assign69980_e106517 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn14 * locals.var_tmf2) - (assign69980_e106517 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign69980_e106523;
        locals.var_t9_dn0 = assign69980_e106523_d_n0;
        locals.var_t9_dn2 = assign69980_e106523_d_n2;
        locals.var_t9_dn4 = assign69980_e106523_d_n4;
        locals.var_t9_dn5 = assign69980_e106523_d_n5;
        locals.var_t9_dn6 = assign69980_e106523_d_n6;
        locals.var_t9_dn7 = assign69980_e106523_d_n7;
        locals.var_t9_dn8 = assign69980_e106523_d_n8;
        locals.var_t9_dn9 = assign69980_e106523_d_n9;
        locals.var_t9_dn10 = assign69980_e106523_d_n10;
        locals.var_t9_dn11 = assign69980_e106523_d_n11;
        locals.var_t9_dn14 = assign69980_e106523_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign69990_e106544, assign69990_e106544_d_n0, assign69990_e106544_d_n2, assign69990_e106544_d_n4, assign69990_e106544_d_n5, assign69990_e106544_d_n6, assign69990_e106544_d_n7, assign69990_e106544_d_n8, assign69990_e106544_d_n9, assign69990_e106544_d_n10, assign69990_e106544_d_n11, assign69990_e106544_d_n14,) = {
    if (((((locals.var_guard1631 != 0.0) && (!((locals.var_guard1629 != 0.0) || (locals.var_guard1630 != 0.0)))) && (locals.var_guard1635 != 0.0)) && (locals.var_guard1636 != 0.0)) && (locals.var_guard1637 != 0.0)) {
        let assign69990_e106539: f64 = (locals.var_vxbgmt + p.p137);
        let assign69990_e106541: f64 = (assign69990_e106539 + locals.var_tmf2);
        let assign69990_e106542: f64 = (0.5 * assign69990_e106541);
        (assign69990_e106542, (0.5 * (locals.var_vxbgmt_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_vxbgmt_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_vxbgmt_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_vxbgmt_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_vxbgmt_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_vxbgmt_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_vxbgmt_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_vxbgmt_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_vxbgmt_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_vxbgmt_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_vxbgmt_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign69990_e106544;
        locals.var_t2_dn0 = assign69990_e106544_d_n0;
        locals.var_t2_dn2 = assign69990_e106544_d_n2;
        locals.var_t2_dn4 = assign69990_e106544_d_n4;
        locals.var_t2_dn5 = assign69990_e106544_d_n5;
        locals.var_t2_dn6 = assign69990_e106544_d_n6;
        locals.var_t2_dn7 = assign69990_e106544_d_n7;
        locals.var_t2_dn8 = assign69990_e106544_d_n8;
        locals.var_t2_dn9 = assign69990_e106544_d_n9;
        locals.var_t2_dn10 = assign69990_e106544_d_n10;
        locals.var_t2_dn11 = assign69990_e106544_d_n11;
        locals.var_t2_dn14 = assign69990_e106544_d_n14;
        locals.var_t2_rv = 0.0;

        let assign70000_e106547: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1639 = assign70000_e106547;
        locals.var_guard1639_rv = 0.0;

        let (assign70010_e106564, assign70010_e106564_d_n0, assign70010_e106564_d_n2, assign70010_e106564_d_n4, assign70010_e106564_d_n5, assign70010_e106564_d_n6, assign70010_e106564_d_n7, assign70010_e106564_d_n8, assign70010_e106564_d_n9, assign70010_e106564_d_n10, assign70010_e106564_d_n11, assign70010_e106564_d_n14,) = {
    if ((((((locals.var_guard1631 != 0.0) && (!((locals.var_guard1629 != 0.0) || (locals.var_guard1630 != 0.0)))) && (locals.var_guard1635 != 0.0)) && (locals.var_guard1636 != 0.0)) && (locals.var_guard1637 != 0.0)) && (locals.var_guard1639 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign70010_e106564;
        locals.var_t2_dn0 = assign70010_e106564_d_n0;
        locals.var_t2_dn2 = assign70010_e106564_d_n2;
        locals.var_t2_dn4 = assign70010_e106564_d_n4;
        locals.var_t2_dn5 = assign70010_e106564_d_n5;
        locals.var_t2_dn6 = assign70010_e106564_d_n6;
        locals.var_t2_dn7 = assign70010_e106564_d_n7;
        locals.var_t2_dn8 = assign70010_e106564_d_n8;
        locals.var_t2_dn9 = assign70010_e106564_d_n9;
        locals.var_t2_dn10 = assign70010_e106564_d_n10;
        locals.var_t2_dn11 = assign70010_e106564_d_n11;
        locals.var_t2_dn14 = assign70010_e106564_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign70020_e106581, assign70020_e106581_d_n0, assign70020_e106581_d_n2, assign70020_e106581_d_n4, assign70020_e106581_d_n5, assign70020_e106581_d_n6, assign70020_e106581_d_n7, assign70020_e106581_d_n8, assign70020_e106581_d_n9, assign70020_e106581_d_n10, assign70020_e106581_d_n11, assign70020_e106581_d_n14,) = {
    if ((((((locals.var_guard1631 != 0.0) && (!((locals.var_guard1629 != 0.0) || (locals.var_guard1630 != 0.0)))) && (locals.var_guard1635 != 0.0)) && (locals.var_guard1636 != 0.0)) && (locals.var_guard1637 != 0.0)) && (locals.var_guard1639 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign70020_e106581;
        locals.var_t9_dn0 = assign70020_e106581_d_n0;
        locals.var_t9_dn2 = assign70020_e106581_d_n2;
        locals.var_t9_dn4 = assign70020_e106581_d_n4;
        locals.var_t9_dn5 = assign70020_e106581_d_n5;
        locals.var_t9_dn6 = assign70020_e106581_d_n6;
        locals.var_t9_dn7 = assign70020_e106581_d_n7;
        locals.var_t9_dn8 = assign70020_e106581_d_n8;
        locals.var_t9_dn9 = assign70020_e106581_d_n9;
        locals.var_t9_dn10 = assign70020_e106581_d_n10;
        locals.var_t9_dn11 = assign70020_e106581_d_n11;
        locals.var_t9_dn14 = assign70020_e106581_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign70030_e106601, assign70030_e106601_d_n0, assign70030_e106601_d_n2, assign70030_e106601_d_n4, assign70030_e106601_d_n5, assign70030_e106601_d_n6, assign70030_e106601_d_n7, assign70030_e106601_d_n8, assign70030_e106601_d_n9, assign70030_e106601_d_n10, assign70030_e106601_d_n11, assign70030_e106601_d_n14,) = {
    if (((((locals.var_guard1631 != 0.0) && (!((locals.var_guard1629 != 0.0) || (locals.var_guard1630 != 0.0)))) && (locals.var_guard1635 != 0.0)) && (locals.var_guard1636 != 0.0)) && (locals.var_guard1637 != 0.0)) {
        let assign70030_e106596: f64 = (locals.var_kjunc * locals.var_t2);
        let assign70030_e106597: f64 = (assign70030_e106596).sqrt();
        let assign70030_e106599: f64 = (assign70030_e106597 * p.p432);
        (assign70030_e106599, ((((locals.var_kjunc_dn0 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn0)) / (2.0 * assign70030_e106597)) * p.p432), ((((locals.var_kjunc_dn2 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn2)) / (2.0 * assign70030_e106597)) * p.p432), ((((locals.var_kjunc_dn4 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn4)) / (2.0 * assign70030_e106597)) * p.p432), ((((locals.var_kjunc_dn5 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn5)) / (2.0 * assign70030_e106597)) * p.p432), ((((locals.var_kjunc_dn6 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn6)) / (2.0 * assign70030_e106597)) * p.p432), ((((locals.var_kjunc_dn7 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn7)) / (2.0 * assign70030_e106597)) * p.p432), ((((locals.var_kjunc_dn8 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn8)) / (2.0 * assign70030_e106597)) * p.p432), ((((locals.var_kjunc_dn9 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn9)) / (2.0 * assign70030_e106597)) * p.p432), ((((locals.var_kjunc_dn10 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn10)) / (2.0 * assign70030_e106597)) * p.p432), ((((locals.var_kjunc_dn11 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn11)) / (2.0 * assign70030_e106597)) * p.p432), ((((locals.var_kjunc_dn14 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn14)) / (2.0 * assign70030_e106597)) * p.p432),)
    } else {
        (locals.var_wjunc0, locals.var_wjunc0_dn0, locals.var_wjunc0_dn2, locals.var_wjunc0_dn4, locals.var_wjunc0_dn5, locals.var_wjunc0_dn6, locals.var_wjunc0_dn7, locals.var_wjunc0_dn8, locals.var_wjunc0_dn9, locals.var_wjunc0_dn10, locals.var_wjunc0_dn11, locals.var_wjunc0_dn14,)
    }
};
        locals.var_wjunc0 = assign70030_e106601;
        locals.var_wjunc0_dn0 = assign70030_e106601_d_n0;
        locals.var_wjunc0_dn2 = assign70030_e106601_d_n2;
        locals.var_wjunc0_dn4 = assign70030_e106601_d_n4;
        locals.var_wjunc0_dn5 = assign70030_e106601_d_n5;
        locals.var_wjunc0_dn6 = assign70030_e106601_d_n6;
        locals.var_wjunc0_dn7 = assign70030_e106601_d_n7;
        locals.var_wjunc0_dn8 = assign70030_e106601_d_n8;
        locals.var_wjunc0_dn9 = assign70030_e106601_d_n9;
        locals.var_wjunc0_dn10 = assign70030_e106601_d_n10;
        locals.var_wjunc0_dn11 = assign70030_e106601_d_n11;
        locals.var_wjunc0_dn14 = assign70030_e106601_d_n14;
        locals.var_wjunc0_rv = 0.0;

        let (assign70040_e106618, assign70040_e106618_d_n0, assign70040_e106618_d_n2, assign70040_e106618_d_n4, assign70040_e106618_d_n5, assign70040_e106618_d_n6, assign70040_e106618_d_n7, assign70040_e106618_d_n8, assign70040_e106618_d_n9, assign70040_e106618_d_n10, assign70040_e106618_d_n11, assign70040_e106618_d_n14,) = {
    if (((((locals.var_guard1631 != 0.0) && (!((locals.var_guard1629 != 0.0) || (locals.var_guard1630 != 0.0)))) && (locals.var_guard1635 != 0.0)) && (locals.var_guard1636 != 0.0)) && (locals.var_guard1637 != 0.0)) {
        let assign70040_e106616: f64 = (locals.var_lover_func - locals.var_wjunc0);
        (assign70040_e106616, (locals.var_lover_func_dn0 - locals.var_wjunc0_dn0), (locals.var_lover_func_dn2 - locals.var_wjunc0_dn2), (locals.var_lover_func_dn4 - locals.var_wjunc0_dn4), (locals.var_lover_func_dn5 - locals.var_wjunc0_dn5), (locals.var_lover_func_dn6 - locals.var_wjunc0_dn6), (locals.var_lover_func_dn7 - locals.var_wjunc0_dn7), (locals.var_lover_func_dn8 - locals.var_wjunc0_dn8), (locals.var_lover_func_dn9 - locals.var_wjunc0_dn9), (locals.var_lover_func_dn10 - locals.var_wjunc0_dn10), (locals.var_lover_func_dn11 - locals.var_wjunc0_dn11), (locals.var_lover_func_dn14 - locals.var_wjunc0_dn14),)
    } else {
        (locals.var_lover_func, locals.var_lover_func_dn0, locals.var_lover_func_dn2, locals.var_lover_func_dn4, locals.var_lover_func_dn5, locals.var_lover_func_dn6, locals.var_lover_func_dn7, locals.var_lover_func_dn8, locals.var_lover_func_dn9, locals.var_lover_func_dn10, locals.var_lover_func_dn11, locals.var_lover_func_dn14,)
    }
};
        locals.var_lover_func = assign70040_e106618;
        locals.var_lover_func_dn0 = assign70040_e106618_d_n0;
        locals.var_lover_func_dn2 = assign70040_e106618_d_n2;
        locals.var_lover_func_dn4 = assign70040_e106618_d_n4;
        locals.var_lover_func_dn5 = assign70040_e106618_d_n5;
        locals.var_lover_func_dn6 = assign70040_e106618_d_n6;
        locals.var_lover_func_dn7 = assign70040_e106618_d_n7;
        locals.var_lover_func_dn8 = assign70040_e106618_d_n8;
        locals.var_lover_func_dn9 = assign70040_e106618_d_n9;
        locals.var_lover_func_dn10 = assign70040_e106618_d_n10;
        locals.var_lover_func_dn11 = assign70040_e106618_d_n11;
        locals.var_lover_func_dn14 = assign70040_e106618_d_n14;
        locals.var_lover_func_rv = 0.0;

        let assign70050_e106637: f64 = if (((((p.p35 == 1.0) && (p.p63 > 0.0)) && (locals.var_uc_nover > 0.0)) && (locals.var_uc_cvdsover != 0.0)) && (p.p55 != 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard1640 = assign70050_e106637;
        locals.var_guard1640_rv = 0.0;

        let (assign70060_e106650,) = {
    if (((locals.var_guard1632 != 0.0) && (!(((locals.var_guard1629 != 0.0) || (locals.var_guard1630 != 0.0)) || (locals.var_guard1631 != 0.0)))) && (locals.var_guard1640 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_calcqover,)
    }
};
        locals.var_flg_calcqover = assign70060_e106650;
        locals.var_flg_calcqover_rv = 0.0;

        let (assign70070_e106665, assign70070_e106665_d_n2, assign70070_e106665_d_n7, assign70070_e106665_d_n8, assign70070_e106665_d_n9,) = {
    if (((locals.var_guard1632 != 0.0) && (!(((locals.var_guard1629 != 0.0) || (locals.var_guard1630 != 0.0)) || (locals.var_guard1631 != 0.0)))) && (locals.var_guard1640 != 0.0)) {
        let assign70070_e106663: f64 = (locals.var_vgsei - locals.var_vbsei);
        (assign70070_e106663, (locals.var_vgsei_dn2 - locals.var_vbsei_dn2), locals.var_vgsei_dn7, 0.0, (-locals.var_vbsei_dn9),)
    } else {
        (locals.var_vgbgmt, locals.var_vgbgmt_dn2, locals.var_vgbgmt_dn7, locals.var_vgbgmt_dn8, locals.var_vgbgmt_dn9,)
    }
};
        locals.var_vgbgmt = assign70070_e106665;
        locals.var_vgbgmt_dn2 = assign70070_e106665_d_n2;
        locals.var_vgbgmt_dn7 = assign70070_e106665_d_n7;
        locals.var_vgbgmt_dn8 = assign70070_e106665_d_n8;
        locals.var_vgbgmt_dn9 = assign70070_e106665_d_n9;
        locals.var_vgbgmt_rv = 0.0;

        let (assign70080_e106680, assign70080_e106680_d_n0, assign70080_e106680_d_n2, assign70080_e106680_d_n4, assign70080_e106680_d_n5, assign70080_e106680_d_n6, assign70080_e106680_d_n7, assign70080_e106680_d_n8, assign70080_e106680_d_n9, assign70080_e106680_d_n10, assign70080_e106680_d_n11, assign70080_e106680_d_n14,) = {
    if (((locals.var_guard1632 != 0.0) && (!(((locals.var_guard1629 != 0.0) || (locals.var_guard1630 != 0.0)) || (locals.var_guard1631 != 0.0)))) && (locals.var_guard1640 != 0.0)) {
        let assign70080_e106678: f64 = (locals.var_vdsei - locals.var_vbsei);
        (assign70080_e106678, locals.var_vdsei_dn0, (locals.var_vdsei_dn2 - locals.var_vbsei_dn2), 0.0, 0.0, 0.0, 0.0, 0.0, (-locals.var_vbsei_dn9), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn11, locals.var_vxbgmt_dn14,)
    }
};
        locals.var_vxbgmt = assign70080_e106680;
        locals.var_vxbgmt_dn0 = assign70080_e106680_d_n0;
        locals.var_vxbgmt_dn2 = assign70080_e106680_d_n2;
        locals.var_vxbgmt_dn4 = assign70080_e106680_d_n4;
        locals.var_vxbgmt_dn5 = assign70080_e106680_d_n5;
        locals.var_vxbgmt_dn6 = assign70080_e106680_d_n6;
        locals.var_vxbgmt_dn7 = assign70080_e106680_d_n7;
        locals.var_vxbgmt_dn8 = assign70080_e106680_d_n8;
        locals.var_vxbgmt_dn9 = assign70080_e106680_d_n9;
        locals.var_vxbgmt_dn10 = assign70080_e106680_d_n10;
        locals.var_vxbgmt_dn11 = assign70080_e106680_d_n11;
        locals.var_vxbgmt_dn14 = assign70080_e106680_d_n14;
        locals.var_vxbgmt_rv = 0.0;

        let (assign70090_e106684, assign70090_e106684_d_n0, assign70090_e106684_d_n2, assign70090_e106684_d_n4, assign70090_e106684_d_n5, assign70090_e106684_d_n6, assign70090_e106684_d_n7, assign70090_e106684_d_n8, assign70090_e106684_d_n9, assign70090_e106684_d_n10, assign70090_e106684_d_n11, assign70090_e106684_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.4, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbs_bnd_over, locals.var_vbs_bnd_over_dn0, locals.var_vbs_bnd_over_dn2, locals.var_vbs_bnd_over_dn4, locals.var_vbs_bnd_over_dn5, locals.var_vbs_bnd_over_dn6, locals.var_vbs_bnd_over_dn7, locals.var_vbs_bnd_over_dn8, locals.var_vbs_bnd_over_dn9, locals.var_vbs_bnd_over_dn10, locals.var_vbs_bnd_over_dn11, locals.var_vbs_bnd_over_dn14,)
    }
};
        locals.var_vbs_bnd_over = assign70090_e106684;
        locals.var_vbs_bnd_over_dn0 = assign70090_e106684_d_n0;
        locals.var_vbs_bnd_over_dn2 = assign70090_e106684_d_n2;
        locals.var_vbs_bnd_over_dn4 = assign70090_e106684_d_n4;
        locals.var_vbs_bnd_over_dn5 = assign70090_e106684_d_n5;
        locals.var_vbs_bnd_over_dn6 = assign70090_e106684_d_n6;
        locals.var_vbs_bnd_over_dn7 = assign70090_e106684_d_n7;
        locals.var_vbs_bnd_over_dn8 = assign70090_e106684_d_n8;
        locals.var_vbs_bnd_over_dn9 = assign70090_e106684_d_n9;
        locals.var_vbs_bnd_over_dn10 = assign70090_e106684_d_n10;
        locals.var_vbs_bnd_over_dn11 = assign70090_e106684_d_n11;
        locals.var_vbs_bnd_over_dn14 = assign70090_e106684_d_n14;
        locals.var_vbs_bnd_over_rv = 0.0;

        let (assign70110_e106692,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0,)
    } else {
        (locals.var_flg_fd_mode,)
    }
};
        locals.var_flg_fd_mode = assign70110_e106692;
        locals.var_flg_fd_mode_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_263(
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign70120_e106696, assign70120_e106696_d_n0, assign70120_e106696_d_n2, assign70120_e106696_d_n4, assign70120_e106696_d_n5, assign70120_e106696_d_n6, assign70120_e106696_d_n7, assign70120_e106696_d_n8, assign70120_e106696_d_n9, assign70120_e106696_d_n10, assign70120_e106696_d_n11, assign70120_e106696_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn14,)
    }
};
        locals.var_fb = assign70120_e106696;
        locals.var_fb_dn0 = assign70120_e106696_d_n0;
        locals.var_fb_dn2 = assign70120_e106696_d_n2;
        locals.var_fb_dn4 = assign70120_e106696_d_n4;
        locals.var_fb_dn5 = assign70120_e106696_d_n5;
        locals.var_fb_dn6 = assign70120_e106696_d_n6;
        locals.var_fb_dn7 = assign70120_e106696_d_n7;
        locals.var_fb_dn8 = assign70120_e106696_d_n8;
        locals.var_fb_dn9 = assign70120_e106696_d_n9;
        locals.var_fb_dn10 = assign70120_e106696_d_n10;
        locals.var_fb_dn11 = assign70120_e106696_d_n11;
        locals.var_fb_dn14 = assign70120_e106696_d_n14;
        locals.var_fb_rv = 0.0;

        let (assign70130_e106700, assign70130_e106700_d_n0, assign70130_e106700_d_n2, assign70130_e106700_d_n4, assign70130_e106700_d_n5, assign70130_e106700_d_n6, assign70130_e106700_d_n7, assign70130_e106700_d_n8, assign70130_e106700_d_n9, assign70130_e106700_d_n10, assign70130_e106700_d_n11, assign70130_e106700_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn11, locals.var_fs01_dn14,)
    }
};
        locals.var_fs01 = assign70130_e106700;
        locals.var_fs01_dn0 = assign70130_e106700_d_n0;
        locals.var_fs01_dn2 = assign70130_e106700_d_n2;
        locals.var_fs01_dn4 = assign70130_e106700_d_n4;
        locals.var_fs01_dn5 = assign70130_e106700_d_n5;
        locals.var_fs01_dn6 = assign70130_e106700_d_n6;
        locals.var_fs01_dn7 = assign70130_e106700_d_n7;
        locals.var_fs01_dn8 = assign70130_e106700_d_n8;
        locals.var_fs01_dn9 = assign70130_e106700_d_n9;
        locals.var_fs01_dn10 = assign70130_e106700_d_n10;
        locals.var_fs01_dn11 = assign70130_e106700_d_n11;
        locals.var_fs01_dn14 = assign70130_e106700_d_n14;
        locals.var_fs01_rv = 0.0;

        let (assign70140_e106704, assign70140_e106704_d_n0, assign70140_e106704_d_n2, assign70140_e106704_d_n4, assign70140_e106704_d_n5, assign70140_e106704_d_n6, assign70140_e106704_d_n7, assign70140_e106704_d_n8, assign70140_e106704_d_n9, assign70140_e106704_d_n10, assign70140_e106704_d_n11, assign70140_e106704_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn11, locals.var_fs02_dn14,)
    }
};
        locals.var_fs02 = assign70140_e106704;
        locals.var_fs02_dn0 = assign70140_e106704_d_n0;
        locals.var_fs02_dn2 = assign70140_e106704_d_n2;
        locals.var_fs02_dn4 = assign70140_e106704_d_n4;
        locals.var_fs02_dn5 = assign70140_e106704_d_n5;
        locals.var_fs02_dn6 = assign70140_e106704_d_n6;
        locals.var_fs02_dn7 = assign70140_e106704_d_n7;
        locals.var_fs02_dn8 = assign70140_e106704_d_n8;
        locals.var_fs02_dn9 = assign70140_e106704_d_n9;
        locals.var_fs02_dn10 = assign70140_e106704_d_n10;
        locals.var_fs02_dn11 = assign70140_e106704_d_n11;
        locals.var_fs02_dn14 = assign70140_e106704_d_n14;
        locals.var_fs02_rv = 0.0;

        let (assign70150_e106708, assign70150_e106708_d_n0, assign70150_e106708_d_n2, assign70150_e106708_d_n4, assign70150_e106708_d_n5, assign70150_e106708_d_n6, assign70150_e106708_d_n7, assign70150_e106708_d_n8, assign70150_e106708_d_n9, assign70150_e106708_d_n10, assign70150_e106708_d_n11, assign70150_e106708_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs0, locals.var_fs0_dn0, locals.var_fs0_dn2, locals.var_fs0_dn4, locals.var_fs0_dn5, locals.var_fs0_dn6, locals.var_fs0_dn7, locals.var_fs0_dn8, locals.var_fs0_dn9, locals.var_fs0_dn10, locals.var_fs0_dn11, locals.var_fs0_dn14,)
    }
};
        locals.var_fs0 = assign70150_e106708;
        locals.var_fs0_dn0 = assign70150_e106708_d_n0;
        locals.var_fs0_dn2 = assign70150_e106708_d_n2;
        locals.var_fs0_dn4 = assign70150_e106708_d_n4;
        locals.var_fs0_dn5 = assign70150_e106708_d_n5;
        locals.var_fs0_dn6 = assign70150_e106708_d_n6;
        locals.var_fs0_dn7 = assign70150_e106708_d_n7;
        locals.var_fs0_dn8 = assign70150_e106708_d_n8;
        locals.var_fs0_dn9 = assign70150_e106708_d_n9;
        locals.var_fs0_dn10 = assign70150_e106708_d_n10;
        locals.var_fs0_dn11 = assign70150_e106708_d_n11;
        locals.var_fs0_dn14 = assign70150_e106708_d_n14;
        locals.var_fs0_rv = 0.0;

        let (assign70160_e106712, assign70160_e106712_d_n0, assign70160_e106712_d_n2, assign70160_e106712_d_n4, assign70160_e106712_d_n5, assign70160_e106712_d_n6, assign70160_e106712_d_n7, assign70160_e106712_d_n8, assign70160_e106712_d_n9, assign70160_e106712_d_n10, assign70160_e106712_d_n11, assign70160_e106712_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn4, locals.var_dps0_dn5, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn8, locals.var_dps0_dn9, locals.var_dps0_dn10, locals.var_dps0_dn11, locals.var_dps0_dn14,)
    }
};
        locals.var_dps0 = assign70160_e106712;
        locals.var_dps0_dn0 = assign70160_e106712_d_n0;
        locals.var_dps0_dn2 = assign70160_e106712_d_n2;
        locals.var_dps0_dn4 = assign70160_e106712_d_n4;
        locals.var_dps0_dn5 = assign70160_e106712_d_n5;
        locals.var_dps0_dn6 = assign70160_e106712_d_n6;
        locals.var_dps0_dn7 = assign70160_e106712_d_n7;
        locals.var_dps0_dn8 = assign70160_e106712_d_n8;
        locals.var_dps0_dn9 = assign70160_e106712_d_n9;
        locals.var_dps0_dn10 = assign70160_e106712_d_n10;
        locals.var_dps0_dn11 = assign70160_e106712_d_n11;
        locals.var_dps0_dn14 = assign70160_e106712_d_n14;
        locals.var_dps0_rv = 0.0;

        let (assign70170_e106716, assign70170_e106716_d_n0, assign70170_e106716_d_n2, assign70170_e106716_d_n4, assign70170_e106716_d_n5, assign70170_e106716_d_n6, assign70170_e106716_d_n7, assign70170_e106716_d_n8, assign70170_e106716_d_n9, assign70170_e106716_d_n10, assign70170_e106716_d_n11, assign70170_e106716_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs0_dps0, locals.var_fs0_dps0_dn0, locals.var_fs0_dps0_dn2, locals.var_fs0_dps0_dn4, locals.var_fs0_dps0_dn5, locals.var_fs0_dps0_dn6, locals.var_fs0_dps0_dn7, locals.var_fs0_dps0_dn8, locals.var_fs0_dps0_dn9, locals.var_fs0_dps0_dn10, locals.var_fs0_dps0_dn11, locals.var_fs0_dps0_dn14,)
    }
};
        locals.var_fs0_dps0 = assign70170_e106716;
        locals.var_fs0_dps0_dn0 = assign70170_e106716_d_n0;
        locals.var_fs0_dps0_dn2 = assign70170_e106716_d_n2;
        locals.var_fs0_dps0_dn4 = assign70170_e106716_d_n4;
        locals.var_fs0_dps0_dn5 = assign70170_e106716_d_n5;
        locals.var_fs0_dps0_dn6 = assign70170_e106716_d_n6;
        locals.var_fs0_dps0_dn7 = assign70170_e106716_d_n7;
        locals.var_fs0_dps0_dn8 = assign70170_e106716_d_n8;
        locals.var_fs0_dps0_dn9 = assign70170_e106716_d_n9;
        locals.var_fs0_dps0_dn10 = assign70170_e106716_d_n10;
        locals.var_fs0_dps0_dn11 = assign70170_e106716_d_n11;
        locals.var_fs0_dps0_dn14 = assign70170_e106716_d_n14;
        locals.var_fs0_dps0_rv = 0.0;

        let (assign70180_e106720, assign70180_e106720_d_n0, assign70180_e106720_d_n2, assign70180_e106720_d_n4, assign70180_e106720_d_n5, assign70180_e106720_d_n6, assign70180_e106720_d_n7, assign70180_e106720_d_n8, assign70180_e106720_d_n9, assign70180_e106720_d_n10, assign70180_e106720_d_n11, assign70180_e106720_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn11, locals.var_fs02_dps0_dn14,)
    }
};
        locals.var_fs02_dps0 = assign70180_e106720;
        locals.var_fs02_dps0_dn0 = assign70180_e106720_d_n0;
        locals.var_fs02_dps0_dn2 = assign70180_e106720_d_n2;
        locals.var_fs02_dps0_dn4 = assign70180_e106720_d_n4;
        locals.var_fs02_dps0_dn5 = assign70180_e106720_d_n5;
        locals.var_fs02_dps0_dn6 = assign70180_e106720_d_n6;
        locals.var_fs02_dps0_dn7 = assign70180_e106720_d_n7;
        locals.var_fs02_dps0_dn8 = assign70180_e106720_d_n8;
        locals.var_fs02_dps0_dn9 = assign70180_e106720_d_n9;
        locals.var_fs02_dps0_dn10 = assign70180_e106720_d_n10;
        locals.var_fs02_dps0_dn11 = assign70180_e106720_d_n11;
        locals.var_fs02_dps0_dn14 = assign70180_e106720_d_n14;
        locals.var_fs02_dps0_rv = 0.0;

        let (assign70190_e106724, assign70190_e106724_d_n0, assign70190_e106724_d_n2, assign70190_e106724_d_n4, assign70190_e106724_d_n5, assign70190_e106724_d_n6, assign70190_e106724_d_n7, assign70190_e106724_d_n8, assign70190_e106724_d_n9, assign70190_e106724_d_n10, assign70190_e106724_d_n11, assign70190_e106724_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn14,)
    }
};
        locals.var_fb_dpss = assign70190_e106724;
        locals.var_fb_dpss_dn0 = assign70190_e106724_d_n0;
        locals.var_fb_dpss_dn2 = assign70190_e106724_d_n2;
        locals.var_fb_dpss_dn4 = assign70190_e106724_d_n4;
        locals.var_fb_dpss_dn5 = assign70190_e106724_d_n5;
        locals.var_fb_dpss_dn6 = assign70190_e106724_d_n6;
        locals.var_fb_dpss_dn7 = assign70190_e106724_d_n7;
        locals.var_fb_dpss_dn8 = assign70190_e106724_d_n8;
        locals.var_fb_dpss_dn9 = assign70190_e106724_d_n9;
        locals.var_fb_dpss_dn10 = assign70190_e106724_d_n10;
        locals.var_fb_dpss_dn11 = assign70190_e106724_d_n11;
        locals.var_fb_dpss_dn14 = assign70190_e106724_d_n14;
        locals.var_fb_dpss_rv = 0.0;

        let (assign70200_e106728, assign70200_e106728_d_n0, assign70200_e106728_d_n2, assign70200_e106728_d_n4, assign70200_e106728_d_n5, assign70200_e106728_d_n6, assign70200_e106728_d_n7, assign70200_e106728_d_n8, assign70200_e106728_d_n9, assign70200_e106728_d_n10, assign70200_e106728_d_n11, assign70200_e106728_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn11, locals.var_fs01_dps0_dn14,)
    }
};
        locals.var_fs01_dps0 = assign70200_e106728;
        locals.var_fs01_dps0_dn0 = assign70200_e106728_d_n0;
        locals.var_fs01_dps0_dn2 = assign70200_e106728_d_n2;
        locals.var_fs01_dps0_dn4 = assign70200_e106728_d_n4;
        locals.var_fs01_dps0_dn5 = assign70200_e106728_d_n5;
        locals.var_fs01_dps0_dn6 = assign70200_e106728_d_n6;
        locals.var_fs01_dps0_dn7 = assign70200_e106728_d_n7;
        locals.var_fs01_dps0_dn8 = assign70200_e106728_d_n8;
        locals.var_fs01_dps0_dn9 = assign70200_e106728_d_n9;
        locals.var_fs01_dps0_dn10 = assign70200_e106728_d_n10;
        locals.var_fs01_dps0_dn11 = assign70200_e106728_d_n11;
        locals.var_fs01_dps0_dn14 = assign70200_e106728_d_n14;
        locals.var_fs01_dps0_rv = 0.0;

        let (assign70210_e106732, assign70210_e106732_d_n0, assign70210_e106732_d_n2, assign70210_e106732_d_n4, assign70210_e106732_d_n5, assign70210_e106732_d_n6, assign70210_e106732_d_n7, assign70210_e106732_d_n8, assign70210_e106732_d_n9, assign70210_e106732_d_n10, assign70210_e106732_d_n11, assign70210_e106732_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn11, locals.var_chi_1_dn14,)
    }
};
        locals.var_chi_1 = assign70210_e106732;
        locals.var_chi_1_dn0 = assign70210_e106732_d_n0;
        locals.var_chi_1_dn2 = assign70210_e106732_d_n2;
        locals.var_chi_1_dn4 = assign70210_e106732_d_n4;
        locals.var_chi_1_dn5 = assign70210_e106732_d_n5;
        locals.var_chi_1_dn6 = assign70210_e106732_d_n6;
        locals.var_chi_1_dn7 = assign70210_e106732_d_n7;
        locals.var_chi_1_dn8 = assign70210_e106732_d_n8;
        locals.var_chi_1_dn9 = assign70210_e106732_d_n9;
        locals.var_chi_1_dn10 = assign70210_e106732_d_n10;
        locals.var_chi_1_dn11 = assign70210_e106732_d_n11;
        locals.var_chi_1_dn14 = assign70210_e106732_d_n14;
        locals.var_chi_1_rv = 0.0;

        let (assign70220_e106736, assign70220_e106736_d_n0, assign70220_e106736_d_n2, assign70220_e106736_d_n4, assign70220_e106736_d_n5, assign70220_e106736_d_n6, assign70220_e106736_d_n7, assign70220_e106736_d_n8, assign70220_e106736_d_n9, assign70220_e106736_d_n10, assign70220_e106736_d_n11, assign70220_e106736_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_chi_a, locals.var_chi_a_dn0, locals.var_chi_a_dn2, locals.var_chi_a_dn4, locals.var_chi_a_dn5, locals.var_chi_a_dn6, locals.var_chi_a_dn7, locals.var_chi_a_dn8, locals.var_chi_a_dn9, locals.var_chi_a_dn10, locals.var_chi_a_dn11, locals.var_chi_a_dn14,)
    }
};
        locals.var_chi_a = assign70220_e106736;
        locals.var_chi_a_dn0 = assign70220_e106736_d_n0;
        locals.var_chi_a_dn2 = assign70220_e106736_d_n2;
        locals.var_chi_a_dn4 = assign70220_e106736_d_n4;
        locals.var_chi_a_dn5 = assign70220_e106736_d_n5;
        locals.var_chi_a_dn6 = assign70220_e106736_d_n6;
        locals.var_chi_a_dn7 = assign70220_e106736_d_n7;
        locals.var_chi_a_dn8 = assign70220_e106736_d_n8;
        locals.var_chi_a_dn9 = assign70220_e106736_d_n9;
        locals.var_chi_a_dn10 = assign70220_e106736_d_n10;
        locals.var_chi_a_dn11 = assign70220_e106736_d_n11;
        locals.var_chi_a_dn14 = assign70220_e106736_d_n14;
        locals.var_chi_a_rv = 0.0;

        let (assign70230_e106740, assign70230_e106740_d_n0, assign70230_e106740_d_n2, assign70230_e106740_d_n4, assign70230_e106740_d_n5, assign70230_e106740_d_n6, assign70230_e106740_d_n7, assign70230_e106740_d_n8, assign70230_e106740_d_n9, assign70230_e106740_d_n10, assign70230_e106740_d_n11, assign70230_e106740_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_chi_b, locals.var_chi_b_dn0, locals.var_chi_b_dn2, locals.var_chi_b_dn4, locals.var_chi_b_dn5, locals.var_chi_b_dn6, locals.var_chi_b_dn7, locals.var_chi_b_dn8, locals.var_chi_b_dn9, locals.var_chi_b_dn10, locals.var_chi_b_dn11, locals.var_chi_b_dn14,)
    }
};
        locals.var_chi_b = assign70230_e106740;
        locals.var_chi_b_dn0 = assign70230_e106740_d_n0;
        locals.var_chi_b_dn2 = assign70230_e106740_d_n2;
        locals.var_chi_b_dn4 = assign70230_e106740_d_n4;
        locals.var_chi_b_dn5 = assign70230_e106740_d_n5;
        locals.var_chi_b_dn6 = assign70230_e106740_d_n6;
        locals.var_chi_b_dn7 = assign70230_e106740_d_n7;
        locals.var_chi_b_dn8 = assign70230_e106740_d_n8;
        locals.var_chi_b_dn9 = assign70230_e106740_d_n9;
        locals.var_chi_b_dn10 = assign70230_e106740_d_n10;
        locals.var_chi_b_dn11 = assign70230_e106740_d_n11;
        locals.var_chi_b_dn14 = assign70230_e106740_d_n14;
        locals.var_chi_b_rv = 0.0;

        let (assign70240_e106745,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign70240_e106743: f64 = (-1.0);
        (assign70240_e106743,)
    } else {
        (locals.var_flg_conv,)
    }
};
        locals.var_flg_conv = assign70240_e106745;
        locals.var_flg_conv_rv = 0.0;

        let (assign70250_e106749, assign70250_e106749_d_n0, assign70250_e106749_d_n2, assign70250_e106749_d_n4, assign70250_e106749_d_n5, assign70250_e106749_d_n6, assign70250_e106749_d_n7, assign70250_e106749_d_n8, assign70250_e106749_d_n9, assign70250_e106749_d_n10, assign70250_e106749_d_n11, assign70250_e106749_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ps0ld_ini, locals.var_ps0ld_ini_dn0, locals.var_ps0ld_ini_dn2, locals.var_ps0ld_ini_dn4, locals.var_ps0ld_ini_dn5, locals.var_ps0ld_ini_dn6, locals.var_ps0ld_ini_dn7, locals.var_ps0ld_ini_dn8, locals.var_ps0ld_ini_dn9, locals.var_ps0ld_ini_dn10, locals.var_ps0ld_ini_dn11, locals.var_ps0ld_ini_dn14,)
    }
};
        locals.var_ps0ld_ini = assign70250_e106749;
        locals.var_ps0ld_ini_dn0 = assign70250_e106749_d_n0;
        locals.var_ps0ld_ini_dn2 = assign70250_e106749_d_n2;
        locals.var_ps0ld_ini_dn4 = assign70250_e106749_d_n4;
        locals.var_ps0ld_ini_dn5 = assign70250_e106749_d_n5;
        locals.var_ps0ld_ini_dn6 = assign70250_e106749_d_n6;
        locals.var_ps0ld_ini_dn7 = assign70250_e106749_d_n7;
        locals.var_ps0ld_ini_dn8 = assign70250_e106749_d_n8;
        locals.var_ps0ld_ini_dn9 = assign70250_e106749_d_n9;
        locals.var_ps0ld_ini_dn10 = assign70250_e106749_d_n10;
        locals.var_ps0ld_ini_dn11 = assign70250_e106749_d_n11;
        locals.var_ps0ld_ini_dn14 = assign70250_e106749_d_n14;
        locals.var_ps0ld_ini_rv = 0.0;

        let (assign70260_e106753, assign70260_e106753_d_n0, assign70260_e106753_d_n2, assign70260_e106753_d_n4, assign70260_e106753_d_n5, assign70260_e106753_d_n6, assign70260_e106753_d_n7, assign70260_e106753_d_n8, assign70260_e106753_d_n9, assign70260_e106753_d_n10, assign70260_e106753_d_n11, assign70260_e106753_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fbsq, locals.var_fbsq_dn0, locals.var_fbsq_dn2, locals.var_fbsq_dn4, locals.var_fbsq_dn5, locals.var_fbsq_dn6, locals.var_fbsq_dn7, locals.var_fbsq_dn8, locals.var_fbsq_dn9, locals.var_fbsq_dn10, locals.var_fbsq_dn11, locals.var_fbsq_dn14,)
    }
};
        locals.var_fbsq = assign70260_e106753;
        locals.var_fbsq_dn0 = assign70260_e106753_d_n0;
        locals.var_fbsq_dn2 = assign70260_e106753_d_n2;
        locals.var_fbsq_dn4 = assign70260_e106753_d_n4;
        locals.var_fbsq_dn5 = assign70260_e106753_d_n5;
        locals.var_fbsq_dn6 = assign70260_e106753_d_n6;
        locals.var_fbsq_dn7 = assign70260_e106753_d_n7;
        locals.var_fbsq_dn8 = assign70260_e106753_d_n8;
        locals.var_fbsq_dn9 = assign70260_e106753_d_n9;
        locals.var_fbsq_dn10 = assign70260_e106753_d_n10;
        locals.var_fbsq_dn11 = assign70260_e106753_d_n11;
        locals.var_fbsq_dn14 = assign70260_e106753_d_n14;
        locals.var_fbsq_rv = 0.0;

        let (assign70270_e106764, assign70270_e106764_d_n0, assign70270_e106764_d_n2, assign70270_e106764_d_n4, assign70270_e106764_d_n5, assign70270_e106764_d_n6, assign70270_e106764_d_n7, assign70270_e106764_d_n8, assign70270_e106764_d_n9, assign70270_e106764_d_n10, assign70270_e106764_d_n11, assign70270_e106764_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign70270_e106757: f64 = (2.0 * locals.var_beta_inv);
        let assign70270_e106760: f64 = (locals.var_nover_func / locals.var_nin);
        let assign70270_e106761: f64 = (assign70270_e106760).ln();
        let assign70270_e106762: f64 = (assign70270_e106757 * assign70270_e106761);
        (assign70270_e106762, (((2.0 * locals.var_beta_inv_dn0) * assign70270_e106761) + (assign70270_e106757 * ((-((locals.var_nover_func * locals.var_nin_dn0) / (locals.var_nin * locals.var_nin))) / assign70270_e106760))), (((2.0 * locals.var_beta_inv_dn2) * assign70270_e106761) + (assign70270_e106757 * ((-((locals.var_nover_func * locals.var_nin_dn2) / (locals.var_nin * locals.var_nin))) / assign70270_e106760))), (((2.0 * locals.var_beta_inv_dn4) * assign70270_e106761) + (assign70270_e106757 * ((-((locals.var_nover_func * locals.var_nin_dn4) / (locals.var_nin * locals.var_nin))) / assign70270_e106760))), (((2.0 * locals.var_beta_inv_dn5) * assign70270_e106761) + (assign70270_e106757 * ((-((locals.var_nover_func * locals.var_nin_dn5) / (locals.var_nin * locals.var_nin))) / assign70270_e106760))), (((2.0 * locals.var_beta_inv_dn6) * assign70270_e106761) + (assign70270_e106757 * ((-((locals.var_nover_func * locals.var_nin_dn6) / (locals.var_nin * locals.var_nin))) / assign70270_e106760))), (((2.0 * locals.var_beta_inv_dn7) * assign70270_e106761) + (assign70270_e106757 * ((-((locals.var_nover_func * locals.var_nin_dn7) / (locals.var_nin * locals.var_nin))) / assign70270_e106760))), (((2.0 * locals.var_beta_inv_dn8) * assign70270_e106761) + (assign70270_e106757 * ((-((locals.var_nover_func * locals.var_nin_dn8) / (locals.var_nin * locals.var_nin))) / assign70270_e106760))), (((2.0 * locals.var_beta_inv_dn9) * assign70270_e106761) + (assign70270_e106757 * ((-((locals.var_nover_func * locals.var_nin_dn9) / (locals.var_nin * locals.var_nin))) / assign70270_e106760))), (((2.0 * locals.var_beta_inv_dn10) * assign70270_e106761) + (assign70270_e106757 * ((-((locals.var_nover_func * locals.var_nin_dn10) / (locals.var_nin * locals.var_nin))) / assign70270_e106760))), (((2.0 * locals.var_beta_inv_dn11) * assign70270_e106761) + (assign70270_e106757 * ((-((locals.var_nover_func * locals.var_nin_dn11) / (locals.var_nin * locals.var_nin))) / assign70270_e106760))), (((2.0 * locals.var_beta_inv_dn14) * assign70270_e106761) + (assign70270_e106757 * ((-((locals.var_nover_func * locals.var_nin_dn14) / (locals.var_nin * locals.var_nin))) / assign70270_e106760))),)
    } else {
        (locals.var_pb2over, locals.var_pb2over_dn0, locals.var_pb2over_dn2, locals.var_pb2over_dn4, locals.var_pb2over_dn5, locals.var_pb2over_dn6, locals.var_pb2over_dn7, locals.var_pb2over_dn8, locals.var_pb2over_dn9, locals.var_pb2over_dn10, locals.var_pb2over_dn11, locals.var_pb2over_dn14,)
    }
};
        locals.var_pb2over = assign70270_e106764;
        locals.var_pb2over_dn0 = assign70270_e106764_d_n0;
        locals.var_pb2over_dn2 = assign70270_e106764_d_n2;
        locals.var_pb2over_dn4 = assign70270_e106764_d_n4;
        locals.var_pb2over_dn5 = assign70270_e106764_d_n5;
        locals.var_pb2over_dn6 = assign70270_e106764_d_n6;
        locals.var_pb2over_dn7 = assign70270_e106764_d_n7;
        locals.var_pb2over_dn8 = assign70270_e106764_d_n8;
        locals.var_pb2over_dn9 = assign70270_e106764_d_n9;
        locals.var_pb2over_dn10 = assign70270_e106764_d_n10;
        locals.var_pb2over_dn11 = assign70270_e106764_d_n11;
        locals.var_pb2over_dn14 = assign70270_e106764_d_n14;
        locals.var_pb2over_rv = 0.0;

        let (assign70280_e106772, assign70280_e106772_d_n0, assign70280_e106772_d_n2, assign70280_e106772_d_n4, assign70280_e106772_d_n5, assign70280_e106772_d_n6, assign70280_e106772_d_n7, assign70280_e106772_d_n8, assign70280_e106772_d_n9, assign70280_e106772_d_n10, assign70280_e106772_d_n11, assign70280_e106772_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign70280_e106768: f64 = (0.8 - locals.var_pb2over);
        let assign70280_e106770: f64 = (assign70280_e106768 - 0.1);
        (assign70280_e106770, (-locals.var_pb2over_dn0), (-locals.var_pb2over_dn2), (-locals.var_pb2over_dn4), (-locals.var_pb2over_dn5), (-locals.var_pb2over_dn6), (-locals.var_pb2over_dn7), (-locals.var_pb2over_dn8), (-locals.var_pb2over_dn9), (-locals.var_pb2over_dn10), (-locals.var_pb2over_dn11), (-locals.var_pb2over_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign70280_e106772;
        locals.var_tmf1_dn0 = assign70280_e106772_d_n0;
        locals.var_tmf1_dn2 = assign70280_e106772_d_n2;
        locals.var_tmf1_dn4 = assign70280_e106772_d_n4;
        locals.var_tmf1_dn5 = assign70280_e106772_d_n5;
        locals.var_tmf1_dn6 = assign70280_e106772_d_n6;
        locals.var_tmf1_dn7 = assign70280_e106772_d_n7;
        locals.var_tmf1_dn8 = assign70280_e106772_d_n8;
        locals.var_tmf1_dn9 = assign70280_e106772_d_n9;
        locals.var_tmf1_dn10 = assign70280_e106772_d_n10;
        locals.var_tmf1_dn11 = assign70280_e106772_d_n11;
        locals.var_tmf1_dn14 = assign70280_e106772_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign70290_e106780, assign70290_e106780_d_n0, assign70290_e106780_d_n2, assign70290_e106780_d_n4, assign70290_e106780_d_n5, assign70290_e106780_d_n6, assign70290_e106780_d_n7, assign70290_e106780_d_n8, assign70290_e106780_d_n9, assign70290_e106780_d_n10, assign70290_e106780_d_n11, assign70290_e106780_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign70290_e106776: f64 = (4.0 * 0.8);
        let assign70290_e106778: f64 = (assign70290_e106776 * 0.1);
        (assign70290_e106778, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign70290_e106780;
        locals.var_tmf2_dn0 = assign70290_e106780_d_n0;
        locals.var_tmf2_dn2 = assign70290_e106780_d_n2;
        locals.var_tmf2_dn4 = assign70290_e106780_d_n4;
        locals.var_tmf2_dn5 = assign70290_e106780_d_n5;
        locals.var_tmf2_dn6 = assign70290_e106780_d_n6;
        locals.var_tmf2_dn7 = assign70290_e106780_d_n7;
        locals.var_tmf2_dn8 = assign70290_e106780_d_n8;
        locals.var_tmf2_dn9 = assign70290_e106780_d_n9;
        locals.var_tmf2_dn10 = assign70290_e106780_d_n10;
        locals.var_tmf2_dn11 = assign70290_e106780_d_n11;
        locals.var_tmf2_dn14 = assign70290_e106780_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign70300_e106790, assign70300_e106790_d_n0, assign70300_e106790_d_n2, assign70300_e106790_d_n4, assign70300_e106790_d_n5, assign70300_e106790_d_n6, assign70300_e106790_d_n7, assign70300_e106790_d_n8, assign70300_e106790_d_n9, assign70300_e106790_d_n10, assign70300_e106790_d_n11, assign70300_e106790_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let (assign70300_e106788, assign70300_e106788_d_n0, assign70300_e106788_d_n2, assign70300_e106788_d_n4, assign70300_e106788_d_n5, assign70300_e106788_d_n6, assign70300_e106788_d_n7, assign70300_e106788_d_n8, assign70300_e106788_d_n9, assign70300_e106788_d_n10, assign70300_e106788_d_n11, assign70300_e106788_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign70300_e106787: f64 = (-locals.var_tmf2);
                (assign70300_e106787, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign70300_e106788, assign70300_e106788_d_n0, assign70300_e106788_d_n2, assign70300_e106788_d_n4, assign70300_e106788_d_n5, assign70300_e106788_d_n6, assign70300_e106788_d_n7, assign70300_e106788_d_n8, assign70300_e106788_d_n9, assign70300_e106788_d_n10, assign70300_e106788_d_n11, assign70300_e106788_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign70300_e106790;
        locals.var_tmf2_dn0 = assign70300_e106790_d_n0;
        locals.var_tmf2_dn2 = assign70300_e106790_d_n2;
        locals.var_tmf2_dn4 = assign70300_e106790_d_n4;
        locals.var_tmf2_dn5 = assign70300_e106790_d_n5;
        locals.var_tmf2_dn6 = assign70300_e106790_d_n6;
        locals.var_tmf2_dn7 = assign70300_e106790_d_n7;
        locals.var_tmf2_dn8 = assign70300_e106790_d_n8;
        locals.var_tmf2_dn9 = assign70300_e106790_d_n9;
        locals.var_tmf2_dn10 = assign70300_e106790_d_n10;
        locals.var_tmf2_dn11 = assign70300_e106790_d_n11;
        locals.var_tmf2_dn14 = assign70300_e106790_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign70310_e106799, assign70310_e106799_d_n0, assign70310_e106799_d_n2, assign70310_e106799_d_n4, assign70310_e106799_d_n5, assign70310_e106799_d_n6, assign70310_e106799_d_n7, assign70310_e106799_d_n8, assign70310_e106799_d_n9, assign70310_e106799_d_n10, assign70310_e106799_d_n11, assign70310_e106799_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign70310_e106794: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign70310_e106796: f64 = (assign70310_e106794 + locals.var_tmf2);
        let assign70310_e106797: f64 = (assign70310_e106796).sqrt();
        (assign70310_e106797, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign70310_e106797)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign70310_e106797)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign70310_e106797)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign70310_e106797)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign70310_e106797)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign70310_e106797)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign70310_e106797)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign70310_e106797)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign70310_e106797)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign70310_e106797)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign70310_e106797)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign70310_e106799;
        locals.var_tmf2_dn0 = assign70310_e106799_d_n0;
        locals.var_tmf2_dn2 = assign70310_e106799_d_n2;
        locals.var_tmf2_dn4 = assign70310_e106799_d_n4;
        locals.var_tmf2_dn5 = assign70310_e106799_d_n5;
        locals.var_tmf2_dn6 = assign70310_e106799_d_n6;
        locals.var_tmf2_dn7 = assign70310_e106799_d_n7;
        locals.var_tmf2_dn8 = assign70310_e106799_d_n8;
        locals.var_tmf2_dn9 = assign70310_e106799_d_n9;
        locals.var_tmf2_dn10 = assign70310_e106799_d_n10;
        locals.var_tmf2_dn11 = assign70310_e106799_d_n11;
        locals.var_tmf2_dn14 = assign70310_e106799_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign70320_e106809, assign70320_e106809_d_n0, assign70320_e106809_d_n2, assign70320_e106809_d_n4, assign70320_e106809_d_n5, assign70320_e106809_d_n6, assign70320_e106809_d_n7, assign70320_e106809_d_n8, assign70320_e106809_d_n9, assign70320_e106809_d_n10, assign70320_e106809_d_n11, assign70320_e106809_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign70320_e106805: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign70320_e106806: f64 = (1.0 + assign70320_e106805);
        let assign70320_e106807: f64 = (0.5 * assign70320_e106806);
        (assign70320_e106807, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign70320_e106809;
        locals.var_t0_dn0 = assign70320_e106809_d_n0;
        locals.var_t0_dn2 = assign70320_e106809_d_n2;
        locals.var_t0_dn4 = assign70320_e106809_d_n4;
        locals.var_t0_dn5 = assign70320_e106809_d_n5;
        locals.var_t0_dn6 = assign70320_e106809_d_n6;
        locals.var_t0_dn7 = assign70320_e106809_d_n7;
        locals.var_t0_dn8 = assign70320_e106809_d_n8;
        locals.var_t0_dn9 = assign70320_e106809_d_n9;
        locals.var_t0_dn10 = assign70320_e106809_d_n10;
        locals.var_t0_dn11 = assign70320_e106809_d_n11;
        locals.var_t0_dn14 = assign70320_e106809_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign70330_e106819, assign70330_e106819_d_n0, assign70330_e106819_d_n2, assign70330_e106819_d_n4, assign70330_e106819_d_n5, assign70330_e106819_d_n6, assign70330_e106819_d_n7, assign70330_e106819_d_n8, assign70330_e106819_d_n9, assign70330_e106819_d_n10, assign70330_e106819_d_n11, assign70330_e106819_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign70330_e106815: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign70330_e106816: f64 = (0.5 * assign70330_e106815);
        let assign70330_e106817: f64 = (0.8 - assign70330_e106816);
        (assign70330_e106817, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (-(0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (-(0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (-(0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (-(0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (-(0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (-(0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (-(0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (-(0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (-(0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_vbs_max_over, locals.var_vbs_max_over_dn0, locals.var_vbs_max_over_dn2, locals.var_vbs_max_over_dn4, locals.var_vbs_max_over_dn5, locals.var_vbs_max_over_dn6, locals.var_vbs_max_over_dn7, locals.var_vbs_max_over_dn8, locals.var_vbs_max_over_dn9, locals.var_vbs_max_over_dn10, locals.var_vbs_max_over_dn11, locals.var_vbs_max_over_dn14,)
    }
};
        locals.var_vbs_max_over = assign70330_e106819;
        locals.var_vbs_max_over_dn0 = assign70330_e106819_d_n0;
        locals.var_vbs_max_over_dn2 = assign70330_e106819_d_n2;
        locals.var_vbs_max_over_dn4 = assign70330_e106819_d_n4;
        locals.var_vbs_max_over_dn5 = assign70330_e106819_d_n5;
        locals.var_vbs_max_over_dn6 = assign70330_e106819_d_n6;
        locals.var_vbs_max_over_dn7 = assign70330_e106819_d_n7;
        locals.var_vbs_max_over_dn8 = assign70330_e106819_d_n8;
        locals.var_vbs_max_over_dn9 = assign70330_e106819_d_n9;
        locals.var_vbs_max_over_dn10 = assign70330_e106819_d_n10;
        locals.var_vbs_max_over_dn11 = assign70330_e106819_d_n11;
        locals.var_vbs_max_over_dn14 = assign70330_e106819_d_n14;
        locals.var_vbs_max_over_rv = 0.0;

        let assign70340_e106823: f64 = (locals.var_vbs_max_over * 0.5);
        let assign70340_e106824: f64 = if locals.var_vbs_bnd_over > assign70340_e106823 { 1.0 } else { 0.0 };
        locals.var_guard1653 = assign70340_e106824;
        locals.var_guard1653_rv = 0.0;

        let (assign70350_e106832, assign70350_e106832_d_n0, assign70350_e106832_d_n2, assign70350_e106832_d_n4, assign70350_e106832_d_n5, assign70350_e106832_d_n6, assign70350_e106832_d_n7, assign70350_e106832_d_n8, assign70350_e106832_d_n9, assign70350_e106832_d_n10, assign70350_e106832_d_n11, assign70350_e106832_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1653 != 0.0)) {
        let assign70350_e106830: f64 = (0.5 * locals.var_vbs_max_over);
        (assign70350_e106830, (0.5 * locals.var_vbs_max_over_dn0), (0.5 * locals.var_vbs_max_over_dn2), (0.5 * locals.var_vbs_max_over_dn4), (0.5 * locals.var_vbs_max_over_dn5), (0.5 * locals.var_vbs_max_over_dn6), (0.5 * locals.var_vbs_max_over_dn7), (0.5 * locals.var_vbs_max_over_dn8), (0.5 * locals.var_vbs_max_over_dn9), (0.5 * locals.var_vbs_max_over_dn10), (0.5 * locals.var_vbs_max_over_dn11), (0.5 * locals.var_vbs_max_over_dn14),)
    } else {
        (locals.var_vbs_bnd_over, locals.var_vbs_bnd_over_dn0, locals.var_vbs_bnd_over_dn2, locals.var_vbs_bnd_over_dn4, locals.var_vbs_bnd_over_dn5, locals.var_vbs_bnd_over_dn6, locals.var_vbs_bnd_over_dn7, locals.var_vbs_bnd_over_dn8, locals.var_vbs_bnd_over_dn9, locals.var_vbs_bnd_over_dn10, locals.var_vbs_bnd_over_dn11, locals.var_vbs_bnd_over_dn14,)
    }
};
        locals.var_vbs_bnd_over = assign70350_e106832;
        locals.var_vbs_bnd_over_dn0 = assign70350_e106832_d_n0;
        locals.var_vbs_bnd_over_dn2 = assign70350_e106832_d_n2;
        locals.var_vbs_bnd_over_dn4 = assign70350_e106832_d_n4;
        locals.var_vbs_bnd_over_dn5 = assign70350_e106832_d_n5;
        locals.var_vbs_bnd_over_dn6 = assign70350_e106832_d_n6;
        locals.var_vbs_bnd_over_dn7 = assign70350_e106832_d_n7;
        locals.var_vbs_bnd_over_dn8 = assign70350_e106832_d_n8;
        locals.var_vbs_bnd_over_dn9 = assign70350_e106832_d_n9;
        locals.var_vbs_bnd_over_dn10 = assign70350_e106832_d_n10;
        locals.var_vbs_bnd_over_dn11 = assign70350_e106832_d_n11;
        locals.var_vbs_bnd_over_dn14 = assign70350_e106832_d_n14;
        locals.var_vbs_bnd_over_rv = 0.0;

        let assign70360_e106834: f64 = if param_given[338] { 1.0 } else { 0.0 };
        locals.var_guard1654 = assign70360_e106834;
        locals.var_guard1654_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_264(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign70370_e106840, assign70370_e106840_d_n0, assign70370_e106840_d_n2, assign70370_e106840_d_n4, assign70370_e106840_d_n5, assign70370_e106840_d_n6, assign70370_e106840_d_n7, assign70370_e106840_d_n8, assign70370_e106840_d_n9, assign70370_e106840_d_n10, assign70370_e106840_d_n11, assign70370_e106840_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1654 != 0.0)) {
        (p.p338, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbs_max_over, locals.var_vbs_max_over_dn0, locals.var_vbs_max_over_dn2, locals.var_vbs_max_over_dn4, locals.var_vbs_max_over_dn5, locals.var_vbs_max_over_dn6, locals.var_vbs_max_over_dn7, locals.var_vbs_max_over_dn8, locals.var_vbs_max_over_dn9, locals.var_vbs_max_over_dn10, locals.var_vbs_max_over_dn11, locals.var_vbs_max_over_dn14,)
    }
};
        locals.var_vbs_max_over = assign70370_e106840;
        locals.var_vbs_max_over_dn0 = assign70370_e106840_d_n0;
        locals.var_vbs_max_over_dn2 = assign70370_e106840_d_n2;
        locals.var_vbs_max_over_dn4 = assign70370_e106840_d_n4;
        locals.var_vbs_max_over_dn5 = assign70370_e106840_d_n5;
        locals.var_vbs_max_over_dn6 = assign70370_e106840_d_n6;
        locals.var_vbs_max_over_dn7 = assign70370_e106840_d_n7;
        locals.var_vbs_max_over_dn8 = assign70370_e106840_d_n8;
        locals.var_vbs_max_over_dn9 = assign70370_e106840_d_n9;
        locals.var_vbs_max_over_dn10 = assign70370_e106840_d_n10;
        locals.var_vbs_max_over_dn11 = assign70370_e106840_d_n11;
        locals.var_vbs_max_over_dn14 = assign70370_e106840_d_n14;
        locals.var_vbs_max_over_rv = 0.0;

        let assign70380_e106842: f64 = if param_given[339] { 1.0 } else { 0.0 };
        locals.var_guard1655 = assign70380_e106842;
        locals.var_guard1655_rv = 0.0;

        let (assign70390_e106848, assign70390_e106848_d_n0, assign70390_e106848_d_n2, assign70390_e106848_d_n4, assign70390_e106848_d_n5, assign70390_e106848_d_n6, assign70390_e106848_d_n7, assign70390_e106848_d_n8, assign70390_e106848_d_n9, assign70390_e106848_d_n10, assign70390_e106848_d_n11, assign70390_e106848_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1655 != 0.0)) {
        (p.p339, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbs_bnd_over, locals.var_vbs_bnd_over_dn0, locals.var_vbs_bnd_over_dn2, locals.var_vbs_bnd_over_dn4, locals.var_vbs_bnd_over_dn5, locals.var_vbs_bnd_over_dn6, locals.var_vbs_bnd_over_dn7, locals.var_vbs_bnd_over_dn8, locals.var_vbs_bnd_over_dn9, locals.var_vbs_bnd_over_dn10, locals.var_vbs_bnd_over_dn11, locals.var_vbs_bnd_over_dn14,)
    }
};
        locals.var_vbs_bnd_over = assign70390_e106848;
        locals.var_vbs_bnd_over_dn0 = assign70390_e106848_d_n0;
        locals.var_vbs_bnd_over_dn2 = assign70390_e106848_d_n2;
        locals.var_vbs_bnd_over_dn4 = assign70390_e106848_d_n4;
        locals.var_vbs_bnd_over_dn5 = assign70390_e106848_d_n5;
        locals.var_vbs_bnd_over_dn6 = assign70390_e106848_d_n6;
        locals.var_vbs_bnd_over_dn7 = assign70390_e106848_d_n7;
        locals.var_vbs_bnd_over_dn8 = assign70390_e106848_d_n8;
        locals.var_vbs_bnd_over_dn9 = assign70390_e106848_d_n9;
        locals.var_vbs_bnd_over_dn10 = assign70390_e106848_d_n10;
        locals.var_vbs_bnd_over_dn11 = assign70390_e106848_d_n11;
        locals.var_vbs_bnd_over_dn14 = assign70390_e106848_d_n14;
        locals.var_vbs_bnd_over_rv = 0.0;

        let assign70400_e106850: f64 = if param_given[338] { 1.0 } else { 0.0 };
        locals.var_guard1656 = assign70400_e106850;
        locals.var_guard1656_rv = 0.0;

        let (assign70410_e106861, assign70410_e106861_d_n0, assign70410_e106861_d_n2, assign70410_e106861_d_n4, assign70410_e106861_d_n5, assign70410_e106861_d_n6, assign70410_e106861_d_n7, assign70410_e106861_d_n8, assign70410_e106861_d_n9, assign70410_e106861_d_n10, assign70410_e106861_d_n11, assign70410_e106861_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1655 == 0.0)) && (locals.var_guard1656 != 0.0)) {
        let assign70410_e106859: f64 = (0.5 * locals.var_vbs_max_over);
        (assign70410_e106859, (0.5 * locals.var_vbs_max_over_dn0), (0.5 * locals.var_vbs_max_over_dn2), (0.5 * locals.var_vbs_max_over_dn4), (0.5 * locals.var_vbs_max_over_dn5), (0.5 * locals.var_vbs_max_over_dn6), (0.5 * locals.var_vbs_max_over_dn7), (0.5 * locals.var_vbs_max_over_dn8), (0.5 * locals.var_vbs_max_over_dn9), (0.5 * locals.var_vbs_max_over_dn10), (0.5 * locals.var_vbs_max_over_dn11), (0.5 * locals.var_vbs_max_over_dn14),)
    } else {
        (locals.var_vbs_bnd_over, locals.var_vbs_bnd_over_dn0, locals.var_vbs_bnd_over_dn2, locals.var_vbs_bnd_over_dn4, locals.var_vbs_bnd_over_dn5, locals.var_vbs_bnd_over_dn6, locals.var_vbs_bnd_over_dn7, locals.var_vbs_bnd_over_dn8, locals.var_vbs_bnd_over_dn9, locals.var_vbs_bnd_over_dn10, locals.var_vbs_bnd_over_dn11, locals.var_vbs_bnd_over_dn14,)
    }
};
        locals.var_vbs_bnd_over = assign70410_e106861;
        locals.var_vbs_bnd_over_dn0 = assign70410_e106861_d_n0;
        locals.var_vbs_bnd_over_dn2 = assign70410_e106861_d_n2;
        locals.var_vbs_bnd_over_dn4 = assign70410_e106861_d_n4;
        locals.var_vbs_bnd_over_dn5 = assign70410_e106861_d_n5;
        locals.var_vbs_bnd_over_dn6 = assign70410_e106861_d_n6;
        locals.var_vbs_bnd_over_dn7 = assign70410_e106861_d_n7;
        locals.var_vbs_bnd_over_dn8 = assign70410_e106861_d_n8;
        locals.var_vbs_bnd_over_dn9 = assign70410_e106861_d_n9;
        locals.var_vbs_bnd_over_dn10 = assign70410_e106861_d_n10;
        locals.var_vbs_bnd_over_dn11 = assign70410_e106861_d_n11;
        locals.var_vbs_bnd_over_dn14 = assign70410_e106861_d_n14;
        locals.var_vbs_bnd_over_rv = 0.0;

        let assign70420_e106865: f64 = (locals.var_vbs_max_over * 0.5);
        let assign70420_e106866: f64 = if locals.var_vbs_bnd_over > assign70420_e106865 { 1.0 } else { 0.0 };
        locals.var_guard1657 = assign70420_e106866;
        locals.var_guard1657_rv = 0.0;

        let (assign70430_e106874, assign70430_e106874_d_n0, assign70430_e106874_d_n2, assign70430_e106874_d_n4, assign70430_e106874_d_n5, assign70430_e106874_d_n6, assign70430_e106874_d_n7, assign70430_e106874_d_n8, assign70430_e106874_d_n9, assign70430_e106874_d_n10, assign70430_e106874_d_n11, assign70430_e106874_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1657 != 0.0)) {
        let assign70430_e106872: f64 = (0.5 * locals.var_vbs_max_over);
        (assign70430_e106872, (0.5 * locals.var_vbs_max_over_dn0), (0.5 * locals.var_vbs_max_over_dn2), (0.5 * locals.var_vbs_max_over_dn4), (0.5 * locals.var_vbs_max_over_dn5), (0.5 * locals.var_vbs_max_over_dn6), (0.5 * locals.var_vbs_max_over_dn7), (0.5 * locals.var_vbs_max_over_dn8), (0.5 * locals.var_vbs_max_over_dn9), (0.5 * locals.var_vbs_max_over_dn10), (0.5 * locals.var_vbs_max_over_dn11), (0.5 * locals.var_vbs_max_over_dn14),)
    } else {
        (locals.var_vbs_bnd_over, locals.var_vbs_bnd_over_dn0, locals.var_vbs_bnd_over_dn2, locals.var_vbs_bnd_over_dn4, locals.var_vbs_bnd_over_dn5, locals.var_vbs_bnd_over_dn6, locals.var_vbs_bnd_over_dn7, locals.var_vbs_bnd_over_dn8, locals.var_vbs_bnd_over_dn9, locals.var_vbs_bnd_over_dn10, locals.var_vbs_bnd_over_dn11, locals.var_vbs_bnd_over_dn14,)
    }
};
        locals.var_vbs_bnd_over = assign70430_e106874;
        locals.var_vbs_bnd_over_dn0 = assign70430_e106874_d_n0;
        locals.var_vbs_bnd_over_dn2 = assign70430_e106874_d_n2;
        locals.var_vbs_bnd_over_dn4 = assign70430_e106874_d_n4;
        locals.var_vbs_bnd_over_dn5 = assign70430_e106874_d_n5;
        locals.var_vbs_bnd_over_dn6 = assign70430_e106874_d_n6;
        locals.var_vbs_bnd_over_dn7 = assign70430_e106874_d_n7;
        locals.var_vbs_bnd_over_dn8 = assign70430_e106874_d_n8;
        locals.var_vbs_bnd_over_dn9 = assign70430_e106874_d_n9;
        locals.var_vbs_bnd_over_dn10 = assign70430_e106874_d_n10;
        locals.var_vbs_bnd_over_dn11 = assign70430_e106874_d_n11;
        locals.var_vbs_bnd_over_dn14 = assign70430_e106874_d_n14;
        locals.var_vbs_bnd_over_rv = 0.0;

        let assign70440_e106877: f64 = if p.p38 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1658 = assign70440_e106877;
        locals.var_guard1658_rv = 0.0;

        let (assign70450_e106884, assign70450_e106884_d_n0, assign70450_e106884_d_n2, assign70450_e106884_d_n4, assign70450_e106884_d_n5, assign70450_e106884_d_n6, assign70450_e106884_d_n7, assign70450_e106884_d_n8, assign70450_e106884_d_n9, assign70450_e106884_d_n10, assign70450_e106884_d_n11, assign70450_e106884_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1658 != 0.0)) {
        let assign70450_e106882: f64 = (-locals.var_vxbgmt);
        (assign70450_e106882, (-locals.var_vxbgmt_dn0), (-locals.var_vxbgmt_dn2), (-locals.var_vxbgmt_dn4), (-locals.var_vxbgmt_dn5), (-locals.var_vxbgmt_dn6), (-locals.var_vxbgmt_dn7), (-locals.var_vxbgmt_dn8), (-locals.var_vxbgmt_dn9), (-locals.var_vxbgmt_dn10), (-locals.var_vxbgmt_dn11), (-locals.var_vxbgmt_dn14),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign70450_e106884;
        locals.var_t0_dn0 = assign70450_e106884_d_n0;
        locals.var_t0_dn2 = assign70450_e106884_d_n2;
        locals.var_t0_dn4 = assign70450_e106884_d_n4;
        locals.var_t0_dn5 = assign70450_e106884_d_n5;
        locals.var_t0_dn6 = assign70450_e106884_d_n6;
        locals.var_t0_dn7 = assign70450_e106884_d_n7;
        locals.var_t0_dn8 = assign70450_e106884_d_n8;
        locals.var_t0_dn9 = assign70450_e106884_d_n9;
        locals.var_t0_dn10 = assign70450_e106884_d_n10;
        locals.var_t0_dn11 = assign70450_e106884_d_n11;
        locals.var_t0_dn14 = assign70450_e106884_d_n14;
        locals.var_t0_rv = 0.0;

        let assign70460_e106887: f64 = if locals.var_t0 > locals.var_vbs_bnd_over { 1.0 } else { 0.0 };
        locals.var_guard1659 = assign70460_e106887;
        locals.var_guard1659_rv = 0.0;

        let (assign70470_e106897, assign70470_e106897_d_n0, assign70470_e106897_d_n2, assign70470_e106897_d_n4, assign70470_e106897_d_n5, assign70470_e106897_d_n6, assign70470_e106897_d_n7, assign70470_e106897_d_n8, assign70470_e106897_d_n9, assign70470_e106897_d_n10, assign70470_e106897_d_n11, assign70470_e106897_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1658 != 0.0)) && (locals.var_guard1659 != 0.0)) {
        let assign70470_e106895: f64 = (locals.var_t0 - locals.var_vbs_bnd_over);
        (assign70470_e106895, (locals.var_t0_dn0 - locals.var_vbs_bnd_over_dn0), (locals.var_t0_dn2 - locals.var_vbs_bnd_over_dn2), (locals.var_t0_dn4 - locals.var_vbs_bnd_over_dn4), (locals.var_t0_dn5 - locals.var_vbs_bnd_over_dn5), (locals.var_t0_dn6 - locals.var_vbs_bnd_over_dn6), (locals.var_t0_dn7 - locals.var_vbs_bnd_over_dn7), (locals.var_t0_dn8 - locals.var_vbs_bnd_over_dn8), (locals.var_t0_dn9 - locals.var_vbs_bnd_over_dn9), (locals.var_t0_dn10 - locals.var_vbs_bnd_over_dn10), (locals.var_t0_dn11 - locals.var_vbs_bnd_over_dn11), (locals.var_t0_dn14 - locals.var_vbs_bnd_over_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign70470_e106897;
        locals.var_t1_dn0 = assign70470_e106897_d_n0;
        locals.var_t1_dn2 = assign70470_e106897_d_n2;
        locals.var_t1_dn4 = assign70470_e106897_d_n4;
        locals.var_t1_dn5 = assign70470_e106897_d_n5;
        locals.var_t1_dn6 = assign70470_e106897_d_n6;
        locals.var_t1_dn7 = assign70470_e106897_d_n7;
        locals.var_t1_dn8 = assign70470_e106897_d_n8;
        locals.var_t1_dn9 = assign70470_e106897_d_n9;
        locals.var_t1_dn10 = assign70470_e106897_d_n10;
        locals.var_t1_dn11 = assign70470_e106897_d_n11;
        locals.var_t1_dn14 = assign70470_e106897_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign70480_e106907, assign70480_e106907_d_n0, assign70480_e106907_d_n2, assign70480_e106907_d_n4, assign70480_e106907_d_n5, assign70480_e106907_d_n6, assign70480_e106907_d_n7, assign70480_e106907_d_n8, assign70480_e106907_d_n9, assign70480_e106907_d_n10, assign70480_e106907_d_n11, assign70480_e106907_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1658 != 0.0)) && (locals.var_guard1659 != 0.0)) {
        let assign70480_e106905: f64 = (locals.var_vbs_max_over - locals.var_vbs_bnd_over);
        (assign70480_e106905, (locals.var_vbs_max_over_dn0 - locals.var_vbs_bnd_over_dn0), (locals.var_vbs_max_over_dn2 - locals.var_vbs_bnd_over_dn2), (locals.var_vbs_max_over_dn4 - locals.var_vbs_bnd_over_dn4), (locals.var_vbs_max_over_dn5 - locals.var_vbs_bnd_over_dn5), (locals.var_vbs_max_over_dn6 - locals.var_vbs_bnd_over_dn6), (locals.var_vbs_max_over_dn7 - locals.var_vbs_bnd_over_dn7), (locals.var_vbs_max_over_dn8 - locals.var_vbs_bnd_over_dn8), (locals.var_vbs_max_over_dn9 - locals.var_vbs_bnd_over_dn9), (locals.var_vbs_max_over_dn10 - locals.var_vbs_bnd_over_dn10), (locals.var_vbs_max_over_dn11 - locals.var_vbs_bnd_over_dn11), (locals.var_vbs_max_over_dn14 - locals.var_vbs_bnd_over_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign70480_e106907;
        locals.var_t2_dn0 = assign70480_e106907_d_n0;
        locals.var_t2_dn2 = assign70480_e106907_d_n2;
        locals.var_t2_dn4 = assign70480_e106907_d_n4;
        locals.var_t2_dn5 = assign70480_e106907_d_n5;
        locals.var_t2_dn6 = assign70480_e106907_d_n6;
        locals.var_t2_dn7 = assign70480_e106907_d_n7;
        locals.var_t2_dn8 = assign70480_e106907_d_n8;
        locals.var_t2_dn9 = assign70480_e106907_d_n9;
        locals.var_t2_dn10 = assign70480_e106907_d_n10;
        locals.var_t2_dn11 = assign70480_e106907_d_n11;
        locals.var_t2_dn14 = assign70480_e106907_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign70490_e106917, assign70490_e106917_d_n0, assign70490_e106917_d_n2, assign70490_e106917_d_n4, assign70490_e106917_d_n5, assign70490_e106917_d_n6, assign70490_e106917_d_n7, assign70490_e106917_d_n8, assign70490_e106917_d_n9, assign70490_e106917_d_n10, assign70490_e106917_d_n11, assign70490_e106917_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1658 != 0.0)) && (locals.var_guard1659 != 0.0)) {
        let assign70490_e106915: f64 = (locals.var_t1 / locals.var_t2);
        (assign70490_e106915, (((locals.var_t1_dn0 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn0)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn2 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn2)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn4 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn4)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn5 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn5)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn6 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn6)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn7 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn7)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn8 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn8)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn9 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn9)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn10 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn10)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn11 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn11)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn14 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn14)) / (locals.var_t2 * locals.var_t2)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign70490_e106917;
        locals.var_tmf1_dn0 = assign70490_e106917_d_n0;
        locals.var_tmf1_dn2 = assign70490_e106917_d_n2;
        locals.var_tmf1_dn4 = assign70490_e106917_d_n4;
        locals.var_tmf1_dn5 = assign70490_e106917_d_n5;
        locals.var_tmf1_dn6 = assign70490_e106917_d_n6;
        locals.var_tmf1_dn7 = assign70490_e106917_d_n7;
        locals.var_tmf1_dn8 = assign70490_e106917_d_n8;
        locals.var_tmf1_dn9 = assign70490_e106917_d_n9;
        locals.var_tmf1_dn10 = assign70490_e106917_d_n10;
        locals.var_tmf1_dn11 = assign70490_e106917_d_n11;
        locals.var_tmf1_dn14 = assign70490_e106917_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign70500_e106927, assign70500_e106927_d_n0, assign70500_e106927_d_n2, assign70500_e106927_d_n4, assign70500_e106927_d_n5, assign70500_e106927_d_n6, assign70500_e106927_d_n7, assign70500_e106927_d_n8, assign70500_e106927_d_n9, assign70500_e106927_d_n10, assign70500_e106927_d_n11, assign70500_e106927_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1658 != 0.0)) && (locals.var_guard1659 != 0.0)) {
        let assign70500_e106925: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign70500_e106925, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign70500_e106927;
        locals.var_tmf2_dn0 = assign70500_e106927_d_n0;
        locals.var_tmf2_dn2 = assign70500_e106927_d_n2;
        locals.var_tmf2_dn4 = assign70500_e106927_d_n4;
        locals.var_tmf2_dn5 = assign70500_e106927_d_n5;
        locals.var_tmf2_dn6 = assign70500_e106927_d_n6;
        locals.var_tmf2_dn7 = assign70500_e106927_d_n7;
        locals.var_tmf2_dn8 = assign70500_e106927_d_n8;
        locals.var_tmf2_dn9 = assign70500_e106927_d_n9;
        locals.var_tmf2_dn10 = assign70500_e106927_d_n10;
        locals.var_tmf2_dn11 = assign70500_e106927_d_n11;
        locals.var_tmf2_dn14 = assign70500_e106927_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign70510_e106937, assign70510_e106937_d_n0, assign70510_e106937_d_n2, assign70510_e106937_d_n4, assign70510_e106937_d_n5, assign70510_e106937_d_n6, assign70510_e106937_d_n7, assign70510_e106937_d_n8, assign70510_e106937_d_n9, assign70510_e106937_d_n10, assign70510_e106937_d_n11, assign70510_e106937_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1658 != 0.0)) && (locals.var_guard1659 != 0.0)) {
        let assign70510_e106935: f64 = (locals.var_tmf2 * locals.var_tmf1);
        (assign70510_e106935, ((locals.var_tmf2_dn0 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn0)), ((locals.var_tmf2_dn2 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn2)), ((locals.var_tmf2_dn4 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn4)), ((locals.var_tmf2_dn5 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn5)), ((locals.var_tmf2_dn6 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn6)), ((locals.var_tmf2_dn7 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn7)), ((locals.var_tmf2_dn8 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn8)), ((locals.var_tmf2_dn9 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn9)), ((locals.var_tmf2_dn10 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn10)), ((locals.var_tmf2_dn11 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn11)), ((locals.var_tmf2_dn14 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_tmf3, locals.var_tmf3_dn0, locals.var_tmf3_dn2, locals.var_tmf3_dn4, locals.var_tmf3_dn5, locals.var_tmf3_dn6, locals.var_tmf3_dn7, locals.var_tmf3_dn8, locals.var_tmf3_dn9, locals.var_tmf3_dn10, locals.var_tmf3_dn11, locals.var_tmf3_dn14,)
    }
};
        locals.var_tmf3 = assign70510_e106937;
        locals.var_tmf3_dn0 = assign70510_e106937_d_n0;
        locals.var_tmf3_dn2 = assign70510_e106937_d_n2;
        locals.var_tmf3_dn4 = assign70510_e106937_d_n4;
        locals.var_tmf3_dn5 = assign70510_e106937_d_n5;
        locals.var_tmf3_dn6 = assign70510_e106937_d_n6;
        locals.var_tmf3_dn7 = assign70510_e106937_d_n7;
        locals.var_tmf3_dn8 = assign70510_e106937_d_n8;
        locals.var_tmf3_dn9 = assign70510_e106937_d_n9;
        locals.var_tmf3_dn10 = assign70510_e106937_d_n10;
        locals.var_tmf3_dn11 = assign70510_e106937_d_n11;
        locals.var_tmf3_dn14 = assign70510_e106937_d_n14;
        locals.var_tmf3_rv = 0.0;

        let (assign70520_e106947, assign70520_e106947_d_n0, assign70520_e106947_d_n2, assign70520_e106947_d_n4, assign70520_e106947_d_n5, assign70520_e106947_d_n6, assign70520_e106947_d_n7, assign70520_e106947_d_n8, assign70520_e106947_d_n9, assign70520_e106947_d_n10, assign70520_e106947_d_n11, assign70520_e106947_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1658 != 0.0)) && (locals.var_guard1659 != 0.0)) {
        let assign70520_e106945: f64 = (locals.var_tmf2 * locals.var_tmf2);
        (assign70520_e106945, ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0)), ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2)), ((locals.var_tmf2_dn4 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn4)), ((locals.var_tmf2_dn5 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn5)), ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6)), ((locals.var_tmf2_dn7 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn7)), ((locals.var_tmf2_dn8 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn8)), ((locals.var_tmf2_dn9 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn9)), ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10)), ((locals.var_tmf2_dn11 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn11)), ((locals.var_tmf2_dn14 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn14)),)
    } else {
        (locals.var_tmf4, locals.var_tmf4_dn0, locals.var_tmf4_dn2, locals.var_tmf4_dn4, locals.var_tmf4_dn5, locals.var_tmf4_dn6, locals.var_tmf4_dn7, locals.var_tmf4_dn8, locals.var_tmf4_dn9, locals.var_tmf4_dn10, locals.var_tmf4_dn11, locals.var_tmf4_dn14,)
    }
};
        locals.var_tmf4 = assign70520_e106947;
        locals.var_tmf4_dn0 = assign70520_e106947_d_n0;
        locals.var_tmf4_dn2 = assign70520_e106947_d_n2;
        locals.var_tmf4_dn4 = assign70520_e106947_d_n4;
        locals.var_tmf4_dn5 = assign70520_e106947_d_n5;
        locals.var_tmf4_dn6 = assign70520_e106947_d_n6;
        locals.var_tmf4_dn7 = assign70520_e106947_d_n7;
        locals.var_tmf4_dn8 = assign70520_e106947_d_n8;
        locals.var_tmf4_dn9 = assign70520_e106947_d_n9;
        locals.var_tmf4_dn10 = assign70520_e106947_d_n10;
        locals.var_tmf4_dn11 = assign70520_e106947_d_n11;
        locals.var_tmf4_dn14 = assign70520_e106947_d_n14;
        locals.var_tmf4_rv = 0.0;

        let (assign70530_e106965, assign70530_e106965_d_n0, assign70530_e106965_d_n2, assign70530_e106965_d_n4, assign70530_e106965_d_n5, assign70530_e106965_d_n6, assign70530_e106965_d_n7, assign70530_e106965_d_n8, assign70530_e106965_d_n9, assign70530_e106965_d_n10, assign70530_e106965_d_n11, assign70530_e106965_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1658 != 0.0)) && (locals.var_guard1659 != 0.0)) {
        let assign70530_e106956: f64 = (1.0 + locals.var_tmf1);
        let assign70530_e106958: f64 = (assign70530_e106956 + locals.var_tmf2);
        let assign70530_e106960: f64 = (assign70530_e106958 + locals.var_tmf3);
        let assign70530_e106962: f64 = (assign70530_e106960 + locals.var_tmf4);
        let assign70530_e106963: f64 = (1.0 / assign70530_e106962);
        (assign70530_e106963, (-((((locals.var_tmf1_dn0 + locals.var_tmf2_dn0) + locals.var_tmf3_dn0) + locals.var_tmf4_dn0) / (assign70530_e106962 * assign70530_e106962))), (-((((locals.var_tmf1_dn2 + locals.var_tmf2_dn2) + locals.var_tmf3_dn2) + locals.var_tmf4_dn2) / (assign70530_e106962 * assign70530_e106962))), (-((((locals.var_tmf1_dn4 + locals.var_tmf2_dn4) + locals.var_tmf3_dn4) + locals.var_tmf4_dn4) / (assign70530_e106962 * assign70530_e106962))), (-((((locals.var_tmf1_dn5 + locals.var_tmf2_dn5) + locals.var_tmf3_dn5) + locals.var_tmf4_dn5) / (assign70530_e106962 * assign70530_e106962))), (-((((locals.var_tmf1_dn6 + locals.var_tmf2_dn6) + locals.var_tmf3_dn6) + locals.var_tmf4_dn6) / (assign70530_e106962 * assign70530_e106962))), (-((((locals.var_tmf1_dn7 + locals.var_tmf2_dn7) + locals.var_tmf3_dn7) + locals.var_tmf4_dn7) / (assign70530_e106962 * assign70530_e106962))), (-((((locals.var_tmf1_dn8 + locals.var_tmf2_dn8) + locals.var_tmf3_dn8) + locals.var_tmf4_dn8) / (assign70530_e106962 * assign70530_e106962))), (-((((locals.var_tmf1_dn9 + locals.var_tmf2_dn9) + locals.var_tmf3_dn9) + locals.var_tmf4_dn9) / (assign70530_e106962 * assign70530_e106962))), (-((((locals.var_tmf1_dn10 + locals.var_tmf2_dn10) + locals.var_tmf3_dn10) + locals.var_tmf4_dn10) / (assign70530_e106962 * assign70530_e106962))), (-((((locals.var_tmf1_dn11 + locals.var_tmf2_dn11) + locals.var_tmf3_dn11) + locals.var_tmf4_dn11) / (assign70530_e106962 * assign70530_e106962))), (-((((locals.var_tmf1_dn14 + locals.var_tmf2_dn14) + locals.var_tmf3_dn14) + locals.var_tmf4_dn14) / (assign70530_e106962 * assign70530_e106962))),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign70530_e106965;
        locals.var_tmf0_dn0 = assign70530_e106965_d_n0;
        locals.var_tmf0_dn2 = assign70530_e106965_d_n2;
        locals.var_tmf0_dn4 = assign70530_e106965_d_n4;
        locals.var_tmf0_dn5 = assign70530_e106965_d_n5;
        locals.var_tmf0_dn6 = assign70530_e106965_d_n6;
        locals.var_tmf0_dn7 = assign70530_e106965_d_n7;
        locals.var_tmf0_dn8 = assign70530_e106965_d_n8;
        locals.var_tmf0_dn9 = assign70530_e106965_d_n9;
        locals.var_tmf0_dn10 = assign70530_e106965_d_n10;
        locals.var_tmf0_dn11 = assign70530_e106965_d_n11;
        locals.var_tmf0_dn14 = assign70530_e106965_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign70540_e106990, assign70540_e106990_d_n0, assign70540_e106990_d_n2, assign70540_e106990_d_n4, assign70540_e106990_d_n5, assign70540_e106990_d_n6, assign70540_e106990_d_n7, assign70540_e106990_d_n8, assign70540_e106990_d_n9, assign70540_e106990_d_n10, assign70540_e106990_d_n11, assign70540_e106990_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1658 != 0.0)) && (locals.var_guard1659 != 0.0)) {
        let assign70540_e106974: f64 = (2.0 * locals.var_tmf1);
        let assign70540_e106975: f64 = (1.0 + assign70540_e106974);
        let assign70540_e106978: f64 = (3.0 * locals.var_tmf2);
        let assign70540_e106979: f64 = (assign70540_e106975 + assign70540_e106978);
        let assign70540_e106982: f64 = (4.0 * locals.var_tmf3);
        let assign70540_e106983: f64 = (assign70540_e106979 + assign70540_e106982);
        let assign70540_e106984: f64 = (-assign70540_e106983);
        let assign70540_e106986: f64 = (assign70540_e106984 * locals.var_tmf0);
        let assign70540_e106988: f64 = (assign70540_e106986 * locals.var_tmf0);
        (assign70540_e106988, (((((-(((2.0 * locals.var_tmf1_dn0) + (3.0 * locals.var_tmf2_dn0)) + (4.0 * locals.var_tmf3_dn0))) * locals.var_tmf0) + (assign70540_e106984 * locals.var_tmf0_dn0)) * locals.var_tmf0) + (assign70540_e106986 * locals.var_tmf0_dn0)), (((((-(((2.0 * locals.var_tmf1_dn2) + (3.0 * locals.var_tmf2_dn2)) + (4.0 * locals.var_tmf3_dn2))) * locals.var_tmf0) + (assign70540_e106984 * locals.var_tmf0_dn2)) * locals.var_tmf0) + (assign70540_e106986 * locals.var_tmf0_dn2)), (((((-(((2.0 * locals.var_tmf1_dn4) + (3.0 * locals.var_tmf2_dn4)) + (4.0 * locals.var_tmf3_dn4))) * locals.var_tmf0) + (assign70540_e106984 * locals.var_tmf0_dn4)) * locals.var_tmf0) + (assign70540_e106986 * locals.var_tmf0_dn4)), (((((-(((2.0 * locals.var_tmf1_dn5) + (3.0 * locals.var_tmf2_dn5)) + (4.0 * locals.var_tmf3_dn5))) * locals.var_tmf0) + (assign70540_e106984 * locals.var_tmf0_dn5)) * locals.var_tmf0) + (assign70540_e106986 * locals.var_tmf0_dn5)), (((((-(((2.0 * locals.var_tmf1_dn6) + (3.0 * locals.var_tmf2_dn6)) + (4.0 * locals.var_tmf3_dn6))) * locals.var_tmf0) + (assign70540_e106984 * locals.var_tmf0_dn6)) * locals.var_tmf0) + (assign70540_e106986 * locals.var_tmf0_dn6)), (((((-(((2.0 * locals.var_tmf1_dn7) + (3.0 * locals.var_tmf2_dn7)) + (4.0 * locals.var_tmf3_dn7))) * locals.var_tmf0) + (assign70540_e106984 * locals.var_tmf0_dn7)) * locals.var_tmf0) + (assign70540_e106986 * locals.var_tmf0_dn7)), (((((-(((2.0 * locals.var_tmf1_dn8) + (3.0 * locals.var_tmf2_dn8)) + (4.0 * locals.var_tmf3_dn8))) * locals.var_tmf0) + (assign70540_e106984 * locals.var_tmf0_dn8)) * locals.var_tmf0) + (assign70540_e106986 * locals.var_tmf0_dn8)), (((((-(((2.0 * locals.var_tmf1_dn9) + (3.0 * locals.var_tmf2_dn9)) + (4.0 * locals.var_tmf3_dn9))) * locals.var_tmf0) + (assign70540_e106984 * locals.var_tmf0_dn9)) * locals.var_tmf0) + (assign70540_e106986 * locals.var_tmf0_dn9)), (((((-(((2.0 * locals.var_tmf1_dn10) + (3.0 * locals.var_tmf2_dn10)) + (4.0 * locals.var_tmf3_dn10))) * locals.var_tmf0) + (assign70540_e106984 * locals.var_tmf0_dn10)) * locals.var_tmf0) + (assign70540_e106986 * locals.var_tmf0_dn10)), (((((-(((2.0 * locals.var_tmf1_dn11) + (3.0 * locals.var_tmf2_dn11)) + (4.0 * locals.var_tmf3_dn11))) * locals.var_tmf0) + (assign70540_e106984 * locals.var_tmf0_dn11)) * locals.var_tmf0) + (assign70540_e106986 * locals.var_tmf0_dn11)), (((((-(((2.0 * locals.var_tmf1_dn14) + (3.0 * locals.var_tmf2_dn14)) + (4.0 * locals.var_tmf3_dn14))) * locals.var_tmf0) + (assign70540_e106984 * locals.var_tmf0_dn14)) * locals.var_tmf0) + (assign70540_e106986 * locals.var_tmf0_dn14)),)
    } else {
        (locals.var_t11, locals.var_t11_dn0, locals.var_t11_dn2, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11, locals.var_t11_dn14,)
    }
};
        locals.var_t11 = assign70540_e106990;
        locals.var_t11_dn0 = assign70540_e106990_d_n0;
        locals.var_t11_dn2 = assign70540_e106990_d_n2;
        locals.var_t11_dn4 = assign70540_e106990_d_n4;
        locals.var_t11_dn5 = assign70540_e106990_d_n5;
        locals.var_t11_dn6 = assign70540_e106990_d_n6;
        locals.var_t11_dn7 = assign70540_e106990_d_n7;
        locals.var_t11_dn8 = assign70540_e106990_d_n8;
        locals.var_t11_dn9 = assign70540_e106990_d_n9;
        locals.var_t11_dn10 = assign70540_e106990_d_n10;
        locals.var_t11_dn11 = assign70540_e106990_d_n11;
        locals.var_t11_dn14 = assign70540_e106990_d_n14;
        locals.var_t11_rv = 0.0;

        let (assign70550_e107002, assign70550_e107002_d_n0, assign70550_e107002_d_n2, assign70550_e107002_d_n4, assign70550_e107002_d_n5, assign70550_e107002_d_n6, assign70550_e107002_d_n7, assign70550_e107002_d_n8, assign70550_e107002_d_n9, assign70550_e107002_d_n10, assign70550_e107002_d_n11, assign70550_e107002_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1658 != 0.0)) && (locals.var_guard1659 != 0.0)) {
        let assign70550_e106999: f64 = (1.0 - locals.var_tmf0);
        let assign70550_e107000: f64 = (locals.var_t2 * assign70550_e106999);
        (assign70550_e107000, ((locals.var_t2_dn0 * assign70550_e106999) + (locals.var_t2 * (-locals.var_tmf0_dn0))), ((locals.var_t2_dn2 * assign70550_e106999) + (locals.var_t2 * (-locals.var_tmf0_dn2))), ((locals.var_t2_dn4 * assign70550_e106999) + (locals.var_t2 * (-locals.var_tmf0_dn4))), ((locals.var_t2_dn5 * assign70550_e106999) + (locals.var_t2 * (-locals.var_tmf0_dn5))), ((locals.var_t2_dn6 * assign70550_e106999) + (locals.var_t2 * (-locals.var_tmf0_dn6))), ((locals.var_t2_dn7 * assign70550_e106999) + (locals.var_t2 * (-locals.var_tmf0_dn7))), ((locals.var_t2_dn8 * assign70550_e106999) + (locals.var_t2 * (-locals.var_tmf0_dn8))), ((locals.var_t2_dn9 * assign70550_e106999) + (locals.var_t2 * (-locals.var_tmf0_dn9))), ((locals.var_t2_dn10 * assign70550_e106999) + (locals.var_t2 * (-locals.var_tmf0_dn10))), ((locals.var_t2_dn11 * assign70550_e106999) + (locals.var_t2 * (-locals.var_tmf0_dn11))), ((locals.var_t2_dn14 * assign70550_e106999) + (locals.var_t2 * (-locals.var_tmf0_dn14))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn14,)
    }
};
        locals.var_ty = assign70550_e107002;
        locals.var_ty_dn0 = assign70550_e107002_d_n0;
        locals.var_ty_dn2 = assign70550_e107002_d_n2;
        locals.var_ty_dn4 = assign70550_e107002_d_n4;
        locals.var_ty_dn5 = assign70550_e107002_d_n5;
        locals.var_ty_dn6 = assign70550_e107002_d_n6;
        locals.var_ty_dn7 = assign70550_e107002_d_n7;
        locals.var_ty_dn8 = assign70550_e107002_d_n8;
        locals.var_ty_dn9 = assign70550_e107002_d_n9;
        locals.var_ty_dn10 = assign70550_e107002_d_n10;
        locals.var_ty_dn11 = assign70550_e107002_d_n11;
        locals.var_ty_dn14 = assign70550_e107002_d_n14;
        locals.var_ty_rv = 0.0;

        let (assign70560_e107016, assign70560_e107016_d_n0, assign70560_e107016_d_n2, assign70560_e107016_d_n4, assign70560_e107016_d_n5, assign70560_e107016_d_n6, assign70560_e107016_d_n7, assign70560_e107016_d_n8, assign70560_e107016_d_n9, assign70560_e107016_d_n10, assign70560_e107016_d_n11, assign70560_e107016_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1658 != 0.0)) && (locals.var_guard1659 != 0.0)) {
        let assign70560_e107010: f64 = (1.0 - locals.var_tmf0);
        let assign70560_e107013: f64 = (locals.var_tmf1 * locals.var_t11);
        let assign70560_e107014: f64 = (assign70560_e107010 + assign70560_e107013);
        (assign70560_e107014, ((-locals.var_tmf0_dn0) + ((locals.var_tmf1_dn0 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn0))), ((-locals.var_tmf0_dn2) + ((locals.var_tmf1_dn2 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn2))), ((-locals.var_tmf0_dn4) + ((locals.var_tmf1_dn4 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn4))), ((-locals.var_tmf0_dn5) + ((locals.var_tmf1_dn5 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn5))), ((-locals.var_tmf0_dn6) + ((locals.var_tmf1_dn6 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn6))), ((-locals.var_tmf0_dn7) + ((locals.var_tmf1_dn7 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn7))), ((-locals.var_tmf0_dn8) + ((locals.var_tmf1_dn8 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn8))), ((-locals.var_tmf0_dn9) + ((locals.var_tmf1_dn9 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn9))), ((-locals.var_tmf0_dn10) + ((locals.var_tmf1_dn10 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn10))), ((-locals.var_tmf0_dn11) + ((locals.var_tmf1_dn11 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn11))), ((-locals.var_tmf0_dn14) + ((locals.var_tmf1_dn14 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn14))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign70560_e107016;
        locals.var_t0_dn0 = assign70560_e107016_d_n0;
        locals.var_t0_dn2 = assign70560_e107016_d_n2;
        locals.var_t0_dn4 = assign70560_e107016_d_n4;
        locals.var_t0_dn5 = assign70560_e107016_d_n5;
        locals.var_t0_dn6 = assign70560_e107016_d_n6;
        locals.var_t0_dn7 = assign70560_e107016_d_n7;
        locals.var_t0_dn8 = assign70560_e107016_d_n8;
        locals.var_t0_dn9 = assign70560_e107016_d_n9;
        locals.var_t0_dn10 = assign70560_e107016_d_n10;
        locals.var_t0_dn11 = assign70560_e107016_d_n11;
        locals.var_t0_dn14 = assign70560_e107016_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign70570_e107025, assign70570_e107025_d_n0, assign70570_e107025_d_n2, assign70570_e107025_d_n4, assign70570_e107025_d_n5, assign70570_e107025_d_n6, assign70570_e107025_d_n7, assign70570_e107025_d_n8, assign70570_e107025_d_n9, assign70570_e107025_d_n10, assign70570_e107025_d_n11, assign70570_e107025_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1658 != 0.0)) && (locals.var_guard1659 != 0.0)) {
        let assign70570_e107023: f64 = (-locals.var_t11);
        (assign70570_e107023, (-locals.var_t11_dn0), (-locals.var_t11_dn2), (-locals.var_t11_dn4), (-locals.var_t11_dn5), (-locals.var_t11_dn6), (-locals.var_t11_dn7), (-locals.var_t11_dn8), (-locals.var_t11_dn9), (-locals.var_t11_dn10), (-locals.var_t11_dn11), (-locals.var_t11_dn14),)
    } else {
        (locals.var_t11, locals.var_t11_dn0, locals.var_t11_dn2, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11, locals.var_t11_dn14,)
    }
};
        locals.var_t11 = assign70570_e107025;
        locals.var_t11_dn0 = assign70570_e107025_d_n0;
        locals.var_t11_dn2 = assign70570_e107025_d_n2;
        locals.var_t11_dn4 = assign70570_e107025_d_n4;
        locals.var_t11_dn5 = assign70570_e107025_d_n5;
        locals.var_t11_dn6 = assign70570_e107025_d_n6;
        locals.var_t11_dn7 = assign70570_e107025_d_n7;
        locals.var_t11_dn8 = assign70570_e107025_d_n8;
        locals.var_t11_dn9 = assign70570_e107025_d_n9;
        locals.var_t11_dn10 = assign70570_e107025_d_n10;
        locals.var_t11_dn11 = assign70570_e107025_d_n11;
        locals.var_t11_dn14 = assign70570_e107025_d_n14;
        locals.var_t11_rv = 0.0;

        let (assign70580_e107035, assign70580_e107035_d_n0, assign70580_e107035_d_n2, assign70580_e107035_d_n4, assign70580_e107035_d_n5, assign70580_e107035_d_n6, assign70580_e107035_d_n7, assign70580_e107035_d_n8, assign70580_e107035_d_n9, assign70580_e107035_d_n10, assign70580_e107035_d_n11, assign70580_e107035_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1658 != 0.0)) && (locals.var_guard1659 != 0.0)) {
        let assign70580_e107033: f64 = (locals.var_vbs_bnd_over + locals.var_ty);
        (assign70580_e107033, (locals.var_vbs_bnd_over_dn0 + locals.var_ty_dn0), (locals.var_vbs_bnd_over_dn2 + locals.var_ty_dn2), (locals.var_vbs_bnd_over_dn4 + locals.var_ty_dn4), (locals.var_vbs_bnd_over_dn5 + locals.var_ty_dn5), (locals.var_vbs_bnd_over_dn6 + locals.var_ty_dn6), (locals.var_vbs_bnd_over_dn7 + locals.var_ty_dn7), (locals.var_vbs_bnd_over_dn8 + locals.var_ty_dn8), (locals.var_vbs_bnd_over_dn9 + locals.var_ty_dn9), (locals.var_vbs_bnd_over_dn10 + locals.var_ty_dn10), (locals.var_vbs_bnd_over_dn11 + locals.var_ty_dn11), (locals.var_vbs_bnd_over_dn14 + locals.var_ty_dn14),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign70580_e107035;
        locals.var_t10_dn0 = assign70580_e107035_d_n0;
        locals.var_t10_dn2 = assign70580_e107035_d_n2;
        locals.var_t10_dn4 = assign70580_e107035_d_n4;
        locals.var_t10_dn5 = assign70580_e107035_d_n5;
        locals.var_t10_dn6 = assign70580_e107035_d_n6;
        locals.var_t10_dn7 = assign70580_e107035_d_n7;
        locals.var_t10_dn8 = assign70580_e107035_d_n8;
        locals.var_t10_dn9 = assign70580_e107035_d_n9;
        locals.var_t10_dn10 = assign70580_e107035_d_n10;
        locals.var_t10_dn11 = assign70580_e107035_d_n11;
        locals.var_t10_dn14 = assign70580_e107035_d_n14;
        locals.var_t10_rv = 0.0;

        let (assign70590_e107044, assign70590_e107044_d_n0, assign70590_e107044_d_n2, assign70590_e107044_d_n4, assign70590_e107044_d_n5, assign70590_e107044_d_n6, assign70590_e107044_d_n7, assign70590_e107044_d_n8, assign70590_e107044_d_n9, assign70590_e107044_d_n10, assign70590_e107044_d_n11, assign70590_e107044_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1658 != 0.0)) && (locals.var_guard1659 == 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign70590_e107044;
        locals.var_t10_dn0 = assign70590_e107044_d_n0;
        locals.var_t10_dn2 = assign70590_e107044_d_n2;
        locals.var_t10_dn4 = assign70590_e107044_d_n4;
        locals.var_t10_dn5 = assign70590_e107044_d_n5;
        locals.var_t10_dn6 = assign70590_e107044_d_n6;
        locals.var_t10_dn7 = assign70590_e107044_d_n7;
        locals.var_t10_dn8 = assign70590_e107044_d_n8;
        locals.var_t10_dn9 = assign70590_e107044_d_n9;
        locals.var_t10_dn10 = assign70590_e107044_d_n10;
        locals.var_t10_dn11 = assign70590_e107044_d_n11;
        locals.var_t10_dn14 = assign70590_e107044_d_n14;
        locals.var_t10_rv = 0.0;

        let (assign70600_e107051, assign70600_e107051_d_n0, assign70600_e107051_d_n2, assign70600_e107051_d_n4, assign70600_e107051_d_n5, assign70600_e107051_d_n6, assign70600_e107051_d_n7, assign70600_e107051_d_n8, assign70600_e107051_d_n9, assign70600_e107051_d_n10, assign70600_e107051_d_n11, assign70600_e107051_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1658 != 0.0)) {
        let assign70600_e107049: f64 = (-locals.var_t10);
        (assign70600_e107049, (-locals.var_t10_dn0), (-locals.var_t10_dn2), (-locals.var_t10_dn4), (-locals.var_t10_dn5), (-locals.var_t10_dn6), (-locals.var_t10_dn7), (-locals.var_t10_dn8), (-locals.var_t10_dn9), (-locals.var_t10_dn10), (-locals.var_t10_dn11), (-locals.var_t10_dn14),)
    } else {
        (locals.var_vxbgmtcl, locals.var_vxbgmtcl_dn0, locals.var_vxbgmtcl_dn2, locals.var_vxbgmtcl_dn4, locals.var_vxbgmtcl_dn5, locals.var_vxbgmtcl_dn6, locals.var_vxbgmtcl_dn7, locals.var_vxbgmtcl_dn8, locals.var_vxbgmtcl_dn9, locals.var_vxbgmtcl_dn10, locals.var_vxbgmtcl_dn11, locals.var_vxbgmtcl_dn14,)
    }
};
        locals.var_vxbgmtcl = assign70600_e107051;
        locals.var_vxbgmtcl_dn0 = assign70600_e107051_d_n0;
        locals.var_vxbgmtcl_dn2 = assign70600_e107051_d_n2;
        locals.var_vxbgmtcl_dn4 = assign70600_e107051_d_n4;
        locals.var_vxbgmtcl_dn5 = assign70600_e107051_d_n5;
        locals.var_vxbgmtcl_dn6 = assign70600_e107051_d_n6;
        locals.var_vxbgmtcl_dn7 = assign70600_e107051_d_n7;
        locals.var_vxbgmtcl_dn8 = assign70600_e107051_d_n8;
        locals.var_vxbgmtcl_dn9 = assign70600_e107051_d_n9;
        locals.var_vxbgmtcl_dn10 = assign70600_e107051_d_n10;
        locals.var_vxbgmtcl_dn11 = assign70600_e107051_d_n11;
        locals.var_vxbgmtcl_dn14 = assign70600_e107051_d_n14;
        locals.var_vxbgmtcl_rv = 0.0;

        let (assign70610_e107058, assign70610_e107058_d_n0, assign70610_e107058_d_n2, assign70610_e107058_d_n4, assign70610_e107058_d_n5, assign70610_e107058_d_n6, assign70610_e107058_d_n7, assign70610_e107058_d_n8, assign70610_e107058_d_n9, assign70610_e107058_d_n10, assign70610_e107058_d_n11, assign70610_e107058_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1658 == 0.0)) {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn11, locals.var_vxbgmt_dn14,)
    } else {
        (locals.var_vxbgmtcl, locals.var_vxbgmtcl_dn0, locals.var_vxbgmtcl_dn2, locals.var_vxbgmtcl_dn4, locals.var_vxbgmtcl_dn5, locals.var_vxbgmtcl_dn6, locals.var_vxbgmtcl_dn7, locals.var_vxbgmtcl_dn8, locals.var_vxbgmtcl_dn9, locals.var_vxbgmtcl_dn10, locals.var_vxbgmtcl_dn11, locals.var_vxbgmtcl_dn14,)
    }
};
        locals.var_vxbgmtcl = assign70610_e107058;
        locals.var_vxbgmtcl_dn0 = assign70610_e107058_d_n0;
        locals.var_vxbgmtcl_dn2 = assign70610_e107058_d_n2;
        locals.var_vxbgmtcl_dn4 = assign70610_e107058_d_n4;
        locals.var_vxbgmtcl_dn5 = assign70610_e107058_d_n5;
        locals.var_vxbgmtcl_dn6 = assign70610_e107058_d_n6;
        locals.var_vxbgmtcl_dn7 = assign70610_e107058_d_n7;
        locals.var_vxbgmtcl_dn8 = assign70610_e107058_d_n8;
        locals.var_vxbgmtcl_dn9 = assign70610_e107058_d_n9;
        locals.var_vxbgmtcl_dn10 = assign70610_e107058_d_n10;
        locals.var_vxbgmtcl_dn11 = assign70610_e107058_d_n11;
        locals.var_vxbgmtcl_dn14 = assign70610_e107058_d_n14;
        locals.var_vxbgmtcl_rv = 0.0;

        let (assign70620_e107064, assign70620_e107064_d_n0, assign70620_e107064_d_n2, assign70620_e107064_d_n4, assign70620_e107064_d_n5, assign70620_e107064_d_n6, assign70620_e107064_d_n7, assign70620_e107064_d_n8, assign70620_e107064_d_n9, assign70620_e107064_d_n10, assign70620_e107064_d_n11, assign70620_e107064_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign70620_e107062: f64 = (locals.var_cnst0over_func / locals.var_cox0_func);
        (assign70620_e107062, (locals.var_cnst0over_func_dn0 / locals.var_cox0_func), (locals.var_cnst0over_func_dn2 / locals.var_cox0_func), (locals.var_cnst0over_func_dn4 / locals.var_cox0_func), (locals.var_cnst0over_func_dn5 / locals.var_cox0_func), (locals.var_cnst0over_func_dn6 / locals.var_cox0_func), (locals.var_cnst0over_func_dn7 / locals.var_cox0_func), (locals.var_cnst0over_func_dn8 / locals.var_cox0_func), (locals.var_cnst0over_func_dn9 / locals.var_cox0_func), (locals.var_cnst0over_func_dn10 / locals.var_cox0_func), (locals.var_cnst0over_func_dn11 / locals.var_cox0_func), (locals.var_cnst0over_func_dn14 / locals.var_cox0_func),)
    } else {
        (locals.var_fac1, locals.var_fac1_dn0, locals.var_fac1_dn2, locals.var_fac1_dn4, locals.var_fac1_dn5, locals.var_fac1_dn6, locals.var_fac1_dn7, locals.var_fac1_dn8, locals.var_fac1_dn9, locals.var_fac1_dn10, locals.var_fac1_dn11, locals.var_fac1_dn14,)
    }
};
        locals.var_fac1 = assign70620_e107064;
        locals.var_fac1_dn0 = assign70620_e107064_d_n0;
        locals.var_fac1_dn2 = assign70620_e107064_d_n2;
        locals.var_fac1_dn4 = assign70620_e107064_d_n4;
        locals.var_fac1_dn5 = assign70620_e107064_d_n5;
        locals.var_fac1_dn6 = assign70620_e107064_d_n6;
        locals.var_fac1_dn7 = assign70620_e107064_d_n7;
        locals.var_fac1_dn8 = assign70620_e107064_d_n8;
        locals.var_fac1_dn9 = assign70620_e107064_d_n9;
        locals.var_fac1_dn10 = assign70620_e107064_d_n10;
        locals.var_fac1_dn11 = assign70620_e107064_d_n11;
        locals.var_fac1_dn14 = assign70620_e107064_d_n14;
        locals.var_fac1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_265(
        locals: &mut StampLocals,
    ) {
        let (assign70630_e107070, assign70630_e107070_d_n0, assign70630_e107070_d_n2, assign70630_e107070_d_n4, assign70630_e107070_d_n5, assign70630_e107070_d_n6, assign70630_e107070_d_n7, assign70630_e107070_d_n8, assign70630_e107070_d_n9, assign70630_e107070_d_n10, assign70630_e107070_d_n11, assign70630_e107070_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign70630_e107068: f64 = (locals.var_fac1 * locals.var_fac1);
        (assign70630_e107068, ((locals.var_fac1_dn0 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn0)), ((locals.var_fac1_dn2 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn2)), ((locals.var_fac1_dn4 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn4)), ((locals.var_fac1_dn5 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn5)), ((locals.var_fac1_dn6 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn6)), ((locals.var_fac1_dn7 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn7)), ((locals.var_fac1_dn8 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn8)), ((locals.var_fac1_dn9 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn9)), ((locals.var_fac1_dn10 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn10)), ((locals.var_fac1_dn11 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn11)), ((locals.var_fac1_dn14 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn14)),)
    } else {
        (locals.var_fac1p2, locals.var_fac1p2_dn0, locals.var_fac1p2_dn2, locals.var_fac1p2_dn4, locals.var_fac1p2_dn5, locals.var_fac1p2_dn6, locals.var_fac1p2_dn7, locals.var_fac1p2_dn8, locals.var_fac1p2_dn9, locals.var_fac1p2_dn10, locals.var_fac1p2_dn11, locals.var_fac1p2_dn14,)
    }
};
        locals.var_fac1p2 = assign70630_e107070;
        locals.var_fac1p2_dn0 = assign70630_e107070_d_n0;
        locals.var_fac1p2_dn2 = assign70630_e107070_d_n2;
        locals.var_fac1p2_dn4 = assign70630_e107070_d_n4;
        locals.var_fac1p2_dn5 = assign70630_e107070_d_n5;
        locals.var_fac1p2_dn6 = assign70630_e107070_d_n6;
        locals.var_fac1p2_dn7 = assign70630_e107070_d_n7;
        locals.var_fac1p2_dn8 = assign70630_e107070_d_n8;
        locals.var_fac1p2_dn9 = assign70630_e107070_d_n9;
        locals.var_fac1p2_dn10 = assign70630_e107070_d_n10;
        locals.var_fac1p2_dn11 = assign70630_e107070_d_n11;
        locals.var_fac1p2_dn14 = assign70630_e107070_d_n14;
        locals.var_fac1p2_rv = 0.0;

        let (assign70640_e107077, assign70640_e107077_d_n2, assign70640_e107077_d_n7, assign70640_e107077_d_n8, assign70640_e107077_d_n9,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign70640_e107073: f64 = (-locals.var_vgbgmt);
        let assign70640_e107075: f64 = (assign70640_e107073 + locals.var_uc_vfbover);
        (assign70640_e107075, (-locals.var_vgbgmt_dn2), (-locals.var_vgbgmt_dn7), (-locals.var_vgbgmt_dn8), (-locals.var_vgbgmt_dn9),)
    } else {
        (locals.var_vgpld, locals.var_vgpld_dn2, locals.var_vgpld_dn7, locals.var_vgpld_dn8, locals.var_vgpld_dn9,)
    }
};
        locals.var_vgpld = assign70640_e107077;
        locals.var_vgpld_dn2 = assign70640_e107077_d_n2;
        locals.var_vgpld_dn7 = assign70640_e107077_d_n7;
        locals.var_vgpld_dn8 = assign70640_e107077_d_n8;
        locals.var_vgpld_dn9 = assign70640_e107077_d_n9;
        locals.var_vgpld_rv = 0.0;

        let (assign70650_e107086, assign70650_e107086_d_n0, assign70650_e107086_d_n2, assign70650_e107086_d_n4, assign70650_e107086_d_n5, assign70650_e107086_d_n6, assign70650_e107086_d_n7, assign70650_e107086_d_n8, assign70650_e107086_d_n9, assign70650_e107086_d_n10, assign70650_e107086_d_n11, assign70650_e107086_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign70650_e107080: f64 = (-locals.var_vxbgmtcl);
        let assign70650_e107083: f64 = (10.0 * 2.220446049250313e-16);
        let assign70650_e107084: f64 = (assign70650_e107080 + assign70650_e107083);
        (assign70650_e107084, (-locals.var_vxbgmtcl_dn0), (-locals.var_vxbgmtcl_dn2), (-locals.var_vxbgmtcl_dn4), (-locals.var_vxbgmtcl_dn5), (-locals.var_vxbgmtcl_dn6), (-locals.var_vxbgmtcl_dn7), (-locals.var_vxbgmtcl_dn8), (-locals.var_vxbgmtcl_dn9), (-locals.var_vxbgmtcl_dn10), (-locals.var_vxbgmtcl_dn11), (-locals.var_vxbgmtcl_dn14),)
    } else {
        (locals.var_vgb_fb_ld, locals.var_vgb_fb_ld_dn0, locals.var_vgb_fb_ld_dn2, locals.var_vgb_fb_ld_dn4, locals.var_vgb_fb_ld_dn5, locals.var_vgb_fb_ld_dn6, locals.var_vgb_fb_ld_dn7, locals.var_vgb_fb_ld_dn8, locals.var_vgb_fb_ld_dn9, locals.var_vgb_fb_ld_dn10, locals.var_vgb_fb_ld_dn11, locals.var_vgb_fb_ld_dn14,)
    }
};
        locals.var_vgb_fb_ld = assign70650_e107086;
        locals.var_vgb_fb_ld_dn0 = assign70650_e107086_d_n0;
        locals.var_vgb_fb_ld_dn2 = assign70650_e107086_d_n2;
        locals.var_vgb_fb_ld_dn4 = assign70650_e107086_d_n4;
        locals.var_vgb_fb_ld_dn5 = assign70650_e107086_d_n5;
        locals.var_vgb_fb_ld_dn6 = assign70650_e107086_d_n6;
        locals.var_vgb_fb_ld_dn7 = assign70650_e107086_d_n7;
        locals.var_vgb_fb_ld_dn8 = assign70650_e107086_d_n8;
        locals.var_vgb_fb_ld_dn9 = assign70650_e107086_d_n9;
        locals.var_vgb_fb_ld_dn10 = assign70650_e107086_d_n10;
        locals.var_vgb_fb_ld_dn11 = assign70650_e107086_d_n11;
        locals.var_vgb_fb_ld_dn14 = assign70650_e107086_d_n14;
        locals.var_vgb_fb_ld_rv = 0.0;

        let (assign70660_e107090, assign70660_e107090_d_n0, assign70660_e107090_d_n2, assign70660_e107090_d_n4, assign70660_e107090_d_n5, assign70660_e107090_d_n6, assign70660_e107090_d_n7, assign70660_e107090_d_n8, assign70660_e107090_d_n9, assign70660_e107090_d_n10, assign70660_e107090_d_n11, assign70660_e107090_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_dep_ld, locals.var_q_dep_ld_dn0, locals.var_q_dep_ld_dn2, locals.var_q_dep_ld_dn4, locals.var_q_dep_ld_dn5, locals.var_q_dep_ld_dn6, locals.var_q_dep_ld_dn7, locals.var_q_dep_ld_dn8, locals.var_q_dep_ld_dn9, locals.var_q_dep_ld_dn10, locals.var_q_dep_ld_dn11, locals.var_q_dep_ld_dn14,)
    }
};
        locals.var_q_dep_ld = assign70660_e107090;
        locals.var_q_dep_ld_dn0 = assign70660_e107090_d_n0;
        locals.var_q_dep_ld_dn2 = assign70660_e107090_d_n2;
        locals.var_q_dep_ld_dn4 = assign70660_e107090_d_n4;
        locals.var_q_dep_ld_dn5 = assign70660_e107090_d_n5;
        locals.var_q_dep_ld_dn6 = assign70660_e107090_d_n6;
        locals.var_q_dep_ld_dn7 = assign70660_e107090_d_n7;
        locals.var_q_dep_ld_dn8 = assign70660_e107090_d_n8;
        locals.var_q_dep_ld_dn9 = assign70660_e107090_d_n9;
        locals.var_q_dep_ld_dn10 = assign70660_e107090_d_n10;
        locals.var_q_dep_ld_dn11 = assign70660_e107090_d_n11;
        locals.var_q_dep_ld_dn14 = assign70660_e107090_d_n14;
        locals.var_q_dep_ld_rv = 0.0;

        let (assign70670_e107096,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign70670_e107094: f64 = (1.6021918e-19 * locals.var_nover_func);
        (assign70670_e107094,)
    } else {
        (locals.var_q_nsubld,)
    }
};
        locals.var_q_nsubld = assign70670_e107096;
        locals.var_q_nsubld_rv = 0.0;

        let (assign70680_e107102, assign70680_e107102_d_n0, assign70680_e107102_d_n2, assign70680_e107102_d_n4, assign70680_e107102_d_n5, assign70680_e107102_d_n6, assign70680_e107102_d_n7, assign70680_e107102_d_n8, assign70680_e107102_d_n9, assign70680_e107102_d_n10, assign70680_e107102_d_n11, assign70680_e107102_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign70680_e107100: f64 = (locals.var_nin / locals.var_nover_func);
        (assign70680_e107100, (locals.var_nin_dn0 / locals.var_nover_func), (locals.var_nin_dn2 / locals.var_nover_func), (locals.var_nin_dn4 / locals.var_nover_func), (locals.var_nin_dn5 / locals.var_nover_func), (locals.var_nin_dn6 / locals.var_nover_func), (locals.var_nin_dn7 / locals.var_nover_func), (locals.var_nin_dn8 / locals.var_nover_func), (locals.var_nin_dn9 / locals.var_nover_func), (locals.var_nin_dn10 / locals.var_nover_func), (locals.var_nin_dn11 / locals.var_nover_func), (locals.var_nin_dn14 / locals.var_nover_func),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign70680_e107102;
        locals.var_t0_dn0 = assign70680_e107102_d_n0;
        locals.var_t0_dn2 = assign70680_e107102_d_n2;
        locals.var_t0_dn4 = assign70680_e107102_d_n4;
        locals.var_t0_dn5 = assign70680_e107102_d_n5;
        locals.var_t0_dn6 = assign70680_e107102_d_n6;
        locals.var_t0_dn7 = assign70680_e107102_d_n7;
        locals.var_t0_dn8 = assign70680_e107102_d_n8;
        locals.var_t0_dn9 = assign70680_e107102_d_n9;
        locals.var_t0_dn10 = assign70680_e107102_d_n10;
        locals.var_t0_dn11 = assign70680_e107102_d_n11;
        locals.var_t0_dn14 = assign70680_e107102_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign70690_e107108, assign70690_e107108_d_n0, assign70690_e107108_d_n2, assign70690_e107108_d_n4, assign70690_e107108_d_n5, assign70690_e107108_d_n6, assign70690_e107108_d_n7, assign70690_e107108_d_n8, assign70690_e107108_d_n9, assign70690_e107108_d_n10, assign70690_e107108_d_n11, assign70690_e107108_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign70690_e107106: f64 = (locals.var_t0 * locals.var_t0);
        (assign70690_e107106, ((locals.var_t0_dn0 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn0)), ((locals.var_t0_dn2 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn2)), ((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)), ((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)), ((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)), ((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)), ((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)), ((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)), ((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)), ((locals.var_t0_dn11 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn11)), ((locals.var_t0_dn14 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn14)),)
    } else {
        (locals.var_cnst1over, locals.var_cnst1over_dn0, locals.var_cnst1over_dn2, locals.var_cnst1over_dn4, locals.var_cnst1over_dn5, locals.var_cnst1over_dn6, locals.var_cnst1over_dn7, locals.var_cnst1over_dn8, locals.var_cnst1over_dn9, locals.var_cnst1over_dn10, locals.var_cnst1over_dn11, locals.var_cnst1over_dn14,)
    }
};
        locals.var_cnst1over = assign70690_e107108;
        locals.var_cnst1over_dn0 = assign70690_e107108_d_n0;
        locals.var_cnst1over_dn2 = assign70690_e107108_d_n2;
        locals.var_cnst1over_dn4 = assign70690_e107108_d_n4;
        locals.var_cnst1over_dn5 = assign70690_e107108_d_n5;
        locals.var_cnst1over_dn6 = assign70690_e107108_d_n6;
        locals.var_cnst1over_dn7 = assign70690_e107108_d_n7;
        locals.var_cnst1over_dn8 = assign70690_e107108_d_n8;
        locals.var_cnst1over_dn9 = assign70690_e107108_d_n9;
        locals.var_cnst1over_dn10 = assign70690_e107108_d_n10;
        locals.var_cnst1over_dn11 = assign70690_e107108_d_n11;
        locals.var_cnst1over_dn14 = assign70690_e107108_d_n14;
        locals.var_cnst1over_rv = 0.0;

        let assign70700_e107111: f64 = (-locals.var_vxbgmtcl);
        let assign70700_e107112: f64 = (locals.var_beta * assign70700_e107111);
        let assign70700_e107114: f64 = if assign70700_e107112 >= 500.0 { 1.0 } else { 0.0 };
        locals.var_guard1660 = assign70700_e107114;
        locals.var_guard1660_rv = 0.0;

        let (assign70710_e107129, assign70710_e107129_d_n0, assign70710_e107129_d_n2, assign70710_e107129_d_n4, assign70710_e107129_d_n5, assign70710_e107129_d_n6, assign70710_e107129_d_n7, assign70710_e107129_d_n8, assign70710_e107129_d_n9, assign70710_e107129_d_n10, assign70710_e107129_d_n11, assign70710_e107129_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1660 != 0.0)) {
        let assign70710_e107122: f64 = (-locals.var_vxbgmtcl);
        let assign70710_e107123: f64 = (locals.var_beta * assign70710_e107122);
        let assign70710_e107124: f64 = (1.0 + assign70710_e107123);
        let assign70710_e107126: f64 = (assign70710_e107124 - 500.0);
        let assign70710_e107127: f64 = (1.403592217853e217 * assign70710_e107126);
        (assign70710_e107127, (1.403592217853e217 * ((locals.var_beta_dn0 * assign70710_e107122) + (locals.var_beta * (-locals.var_vxbgmtcl_dn0)))), (1.403592217853e217 * ((locals.var_beta_dn2 * assign70710_e107122) + (locals.var_beta * (-locals.var_vxbgmtcl_dn2)))), (1.403592217853e217 * ((locals.var_beta_dn4 * assign70710_e107122) + (locals.var_beta * (-locals.var_vxbgmtcl_dn4)))), (1.403592217853e217 * ((locals.var_beta_dn5 * assign70710_e107122) + (locals.var_beta * (-locals.var_vxbgmtcl_dn5)))), (1.403592217853e217 * ((locals.var_beta_dn6 * assign70710_e107122) + (locals.var_beta * (-locals.var_vxbgmtcl_dn6)))), (1.403592217853e217 * ((locals.var_beta_dn7 * assign70710_e107122) + (locals.var_beta * (-locals.var_vxbgmtcl_dn7)))), (1.403592217853e217 * ((locals.var_beta_dn8 * assign70710_e107122) + (locals.var_beta * (-locals.var_vxbgmtcl_dn8)))), (1.403592217853e217 * ((locals.var_beta_dn9 * assign70710_e107122) + (locals.var_beta * (-locals.var_vxbgmtcl_dn9)))), (1.403592217853e217 * ((locals.var_beta_dn10 * assign70710_e107122) + (locals.var_beta * (-locals.var_vxbgmtcl_dn10)))), (1.403592217853e217 * ((locals.var_beta_dn11 * assign70710_e107122) + (locals.var_beta * (-locals.var_vxbgmtcl_dn11)))), (1.403592217853e217 * ((locals.var_beta_dn14 * assign70710_e107122) + (locals.var_beta * (-locals.var_vxbgmtcl_dn14)))),)
    } else {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn11, locals.var_exp_bvbs_dn14,)
    }
};
        locals.var_exp_bvbs = assign70710_e107129;
        locals.var_exp_bvbs_dn0 = assign70710_e107129_d_n0;
        locals.var_exp_bvbs_dn2 = assign70710_e107129_d_n2;
        locals.var_exp_bvbs_dn4 = assign70710_e107129_d_n4;
        locals.var_exp_bvbs_dn5 = assign70710_e107129_d_n5;
        locals.var_exp_bvbs_dn6 = assign70710_e107129_d_n6;
        locals.var_exp_bvbs_dn7 = assign70710_e107129_d_n7;
        locals.var_exp_bvbs_dn8 = assign70710_e107129_d_n8;
        locals.var_exp_bvbs_dn9 = assign70710_e107129_d_n9;
        locals.var_exp_bvbs_dn10 = assign70710_e107129_d_n10;
        locals.var_exp_bvbs_dn11 = assign70710_e107129_d_n11;
        locals.var_exp_bvbs_dn14 = assign70710_e107129_d_n14;
        locals.var_exp_bvbs_rv = 0.0;

        let (assign70720_e107135, assign70720_e107135_d_n0, assign70720_e107135_d_n2, assign70720_e107135_d_n4, assign70720_e107135_d_n5, assign70720_e107135_d_n6, assign70720_e107135_d_n7, assign70720_e107135_d_n8, assign70720_e107135_d_n9, assign70720_e107135_d_n10, assign70720_e107135_d_n11, assign70720_e107135_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1660 != 0.0)) {
        (1.403592217853e217, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign70720_e107135;
        locals.var_t0_dn0 = assign70720_e107135_d_n0;
        locals.var_t0_dn2 = assign70720_e107135_d_n2;
        locals.var_t0_dn4 = assign70720_e107135_d_n4;
        locals.var_t0_dn5 = assign70720_e107135_d_n5;
        locals.var_t0_dn6 = assign70720_e107135_d_n6;
        locals.var_t0_dn7 = assign70720_e107135_d_n7;
        locals.var_t0_dn8 = assign70720_e107135_d_n8;
        locals.var_t0_dn9 = assign70720_e107135_d_n9;
        locals.var_t0_dn10 = assign70720_e107135_d_n10;
        locals.var_t0_dn11 = assign70720_e107135_d_n11;
        locals.var_t0_dn14 = assign70720_e107135_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign70730_e107145, assign70730_e107145_d_n0, assign70730_e107145_d_n2, assign70730_e107145_d_n4, assign70730_e107145_d_n5, assign70730_e107145_d_n6, assign70730_e107145_d_n7, assign70730_e107145_d_n8, assign70730_e107145_d_n9, assign70730_e107145_d_n10, assign70730_e107145_d_n11, assign70730_e107145_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1660 == 0.0)) {
        let assign70730_e107142: f64 = (-locals.var_vxbgmtcl);
        let assign70730_e107143: f64 = (locals.var_beta * assign70730_e107142);
        (assign70730_e107143, ((locals.var_beta_dn0 * assign70730_e107142) + (locals.var_beta * (-locals.var_vxbgmtcl_dn0))), ((locals.var_beta_dn2 * assign70730_e107142) + (locals.var_beta * (-locals.var_vxbgmtcl_dn2))), ((locals.var_beta_dn4 * assign70730_e107142) + (locals.var_beta * (-locals.var_vxbgmtcl_dn4))), ((locals.var_beta_dn5 * assign70730_e107142) + (locals.var_beta * (-locals.var_vxbgmtcl_dn5))), ((locals.var_beta_dn6 * assign70730_e107142) + (locals.var_beta * (-locals.var_vxbgmtcl_dn6))), ((locals.var_beta_dn7 * assign70730_e107142) + (locals.var_beta * (-locals.var_vxbgmtcl_dn7))), ((locals.var_beta_dn8 * assign70730_e107142) + (locals.var_beta * (-locals.var_vxbgmtcl_dn8))), ((locals.var_beta_dn9 * assign70730_e107142) + (locals.var_beta * (-locals.var_vxbgmtcl_dn9))), ((locals.var_beta_dn10 * assign70730_e107142) + (locals.var_beta * (-locals.var_vxbgmtcl_dn10))), ((locals.var_beta_dn11 * assign70730_e107142) + (locals.var_beta * (-locals.var_vxbgmtcl_dn11))), ((locals.var_beta_dn14 * assign70730_e107142) + (locals.var_beta * (-locals.var_vxbgmtcl_dn14))),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign70730_e107145;
        locals.var_tmf1_dn0 = assign70730_e107145_d_n0;
        locals.var_tmf1_dn2 = assign70730_e107145_d_n2;
        locals.var_tmf1_dn4 = assign70730_e107145_d_n4;
        locals.var_tmf1_dn5 = assign70730_e107145_d_n5;
        locals.var_tmf1_dn6 = assign70730_e107145_d_n6;
        locals.var_tmf1_dn7 = assign70730_e107145_d_n7;
        locals.var_tmf1_dn8 = assign70730_e107145_d_n8;
        locals.var_tmf1_dn9 = assign70730_e107145_d_n9;
        locals.var_tmf1_dn10 = assign70730_e107145_d_n10;
        locals.var_tmf1_dn11 = assign70730_e107145_d_n11;
        locals.var_tmf1_dn14 = assign70730_e107145_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign70740_e107152, assign70740_e107152_d_n0, assign70740_e107152_d_n2, assign70740_e107152_d_n4, assign70740_e107152_d_n5, assign70740_e107152_d_n6, assign70740_e107152_d_n7, assign70740_e107152_d_n8, assign70740_e107152_d_n9, assign70740_e107152_d_n10, assign70740_e107152_d_n11, assign70740_e107152_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1660 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn11, locals.var_exp_bvbs_dn14,)
    }
};
        locals.var_exp_bvbs = assign70740_e107152;
        locals.var_exp_bvbs_dn0 = assign70740_e107152_d_n0;
        locals.var_exp_bvbs_dn2 = assign70740_e107152_d_n2;
        locals.var_exp_bvbs_dn4 = assign70740_e107152_d_n4;
        locals.var_exp_bvbs_dn5 = assign70740_e107152_d_n5;
        locals.var_exp_bvbs_dn6 = assign70740_e107152_d_n6;
        locals.var_exp_bvbs_dn7 = assign70740_e107152_d_n7;
        locals.var_exp_bvbs_dn8 = assign70740_e107152_d_n8;
        locals.var_exp_bvbs_dn9 = assign70740_e107152_d_n9;
        locals.var_exp_bvbs_dn10 = assign70740_e107152_d_n10;
        locals.var_exp_bvbs_dn11 = assign70740_e107152_d_n11;
        locals.var_exp_bvbs_dn14 = assign70740_e107152_d_n14;
        locals.var_exp_bvbs_rv = 0.0;

        let mut assign70750_loop_guard: usize = 0;
        while {
            let assign70750_cond_e107160: f64 = if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1660 == 0.0)) && (locals.var_tmf1 >= 60.0)) { 1.0 } else { 0.0 };
            assign70750_cond_e107160 != 0.0
        } {
            assign70750_loop_guard += 1;
            assert!(assign70750_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign70750_body0_e107169, assign70750_body0_e107169_d_n0, assign70750_body0_e107169_d_n2, assign70750_body0_e107169_d_n4, assign70750_body0_e107169_d_n5, assign70750_body0_e107169_d_n6, assign70750_body0_e107169_d_n7, assign70750_body0_e107169_d_n8, assign70750_body0_e107169_d_n9, assign70750_body0_e107169_d_n10, assign70750_body0_e107169_d_n11, assign70750_body0_e107169_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1660 == 0.0)) {
        let assign70750_body0_e107167: f64 = (locals.var_exp_bvbs * 1.14200738981568e26);
        (assign70750_body0_e107167, (locals.var_exp_bvbs_dn0 * 1.14200738981568e26), (locals.var_exp_bvbs_dn2 * 1.14200738981568e26), (locals.var_exp_bvbs_dn4 * 1.14200738981568e26), (locals.var_exp_bvbs_dn5 * 1.14200738981568e26), (locals.var_exp_bvbs_dn6 * 1.14200738981568e26), (locals.var_exp_bvbs_dn7 * 1.14200738981568e26), (locals.var_exp_bvbs_dn8 * 1.14200738981568e26), (locals.var_exp_bvbs_dn9 * 1.14200738981568e26), (locals.var_exp_bvbs_dn10 * 1.14200738981568e26), (locals.var_exp_bvbs_dn11 * 1.14200738981568e26), (locals.var_exp_bvbs_dn14 * 1.14200738981568e26),)
    } else {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn11, locals.var_exp_bvbs_dn14,)
    }
};
            locals.var_exp_bvbs = assign70750_body0_e107169;
            locals.var_exp_bvbs_dn0 = assign70750_body0_e107169_d_n0;
            locals.var_exp_bvbs_dn2 = assign70750_body0_e107169_d_n2;
            locals.var_exp_bvbs_dn4 = assign70750_body0_e107169_d_n4;
            locals.var_exp_bvbs_dn5 = assign70750_body0_e107169_d_n5;
            locals.var_exp_bvbs_dn6 = assign70750_body0_e107169_d_n6;
            locals.var_exp_bvbs_dn7 = assign70750_body0_e107169_d_n7;
            locals.var_exp_bvbs_dn8 = assign70750_body0_e107169_d_n8;
            locals.var_exp_bvbs_dn9 = assign70750_body0_e107169_d_n9;
            locals.var_exp_bvbs_dn10 = assign70750_body0_e107169_d_n10;
            locals.var_exp_bvbs_dn11 = assign70750_body0_e107169_d_n11;
            locals.var_exp_bvbs_dn14 = assign70750_body0_e107169_d_n14;
            locals.var_exp_bvbs_rv = 0.0;
            let (assign70750_body1_e107178, assign70750_body1_e107178_d_n0, assign70750_body1_e107178_d_n2, assign70750_body1_e107178_d_n4, assign70750_body1_e107178_d_n5, assign70750_body1_e107178_d_n6, assign70750_body1_e107178_d_n7, assign70750_body1_e107178_d_n8, assign70750_body1_e107178_d_n9, assign70750_body1_e107178_d_n10, assign70750_body1_e107178_d_n11, assign70750_body1_e107178_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1660 == 0.0)) {
        let assign70750_body1_e107176: f64 = (locals.var_tmf1 - 60.0);
        (assign70750_body1_e107176, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
            locals.var_tmf1 = assign70750_body1_e107178;
            locals.var_tmf1_dn0 = assign70750_body1_e107178_d_n0;
            locals.var_tmf1_dn2 = assign70750_body1_e107178_d_n2;
            locals.var_tmf1_dn4 = assign70750_body1_e107178_d_n4;
            locals.var_tmf1_dn5 = assign70750_body1_e107178_d_n5;
            locals.var_tmf1_dn6 = assign70750_body1_e107178_d_n6;
            locals.var_tmf1_dn7 = assign70750_body1_e107178_d_n7;
            locals.var_tmf1_dn8 = assign70750_body1_e107178_d_n8;
            locals.var_tmf1_dn9 = assign70750_body1_e107178_d_n9;
            locals.var_tmf1_dn10 = assign70750_body1_e107178_d_n10;
            locals.var_tmf1_dn11 = assign70750_body1_e107178_d_n11;
            locals.var_tmf1_dn14 = assign70750_body1_e107178_d_n14;
            locals.var_tmf1_rv = 0.0;
        }

        let (assign70760_e107188, assign70760_e107188_d_n0, assign70760_e107188_d_n2, assign70760_e107188_d_n4, assign70760_e107188_d_n5, assign70760_e107188_d_n6, assign70760_e107188_d_n7, assign70760_e107188_d_n8, assign70760_e107188_d_n9, assign70760_e107188_d_n10, assign70760_e107188_d_n11, assign70760_e107188_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1660 == 0.0)) {
        let assign70760_e107185: f64 = (locals.var_tmf1).exp();
        let assign70760_e107186: f64 = (locals.var_exp_bvbs * assign70760_e107185);
        (assign70760_e107186, ((locals.var_exp_bvbs_dn0 * assign70760_e107185) + (locals.var_exp_bvbs * (assign70760_e107185 * locals.var_tmf1_dn0))), ((locals.var_exp_bvbs_dn2 * assign70760_e107185) + (locals.var_exp_bvbs * (assign70760_e107185 * locals.var_tmf1_dn2))), ((locals.var_exp_bvbs_dn4 * assign70760_e107185) + (locals.var_exp_bvbs * (assign70760_e107185 * locals.var_tmf1_dn4))), ((locals.var_exp_bvbs_dn5 * assign70760_e107185) + (locals.var_exp_bvbs * (assign70760_e107185 * locals.var_tmf1_dn5))), ((locals.var_exp_bvbs_dn6 * assign70760_e107185) + (locals.var_exp_bvbs * (assign70760_e107185 * locals.var_tmf1_dn6))), ((locals.var_exp_bvbs_dn7 * assign70760_e107185) + (locals.var_exp_bvbs * (assign70760_e107185 * locals.var_tmf1_dn7))), ((locals.var_exp_bvbs_dn8 * assign70760_e107185) + (locals.var_exp_bvbs * (assign70760_e107185 * locals.var_tmf1_dn8))), ((locals.var_exp_bvbs_dn9 * assign70760_e107185) + (locals.var_exp_bvbs * (assign70760_e107185 * locals.var_tmf1_dn9))), ((locals.var_exp_bvbs_dn10 * assign70760_e107185) + (locals.var_exp_bvbs * (assign70760_e107185 * locals.var_tmf1_dn10))), ((locals.var_exp_bvbs_dn11 * assign70760_e107185) + (locals.var_exp_bvbs * (assign70760_e107185 * locals.var_tmf1_dn11))), ((locals.var_exp_bvbs_dn14 * assign70760_e107185) + (locals.var_exp_bvbs * (assign70760_e107185 * locals.var_tmf1_dn14))),)
    } else {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn11, locals.var_exp_bvbs_dn14,)
    }
};
        locals.var_exp_bvbs = assign70760_e107188;
        locals.var_exp_bvbs_dn0 = assign70760_e107188_d_n0;
        locals.var_exp_bvbs_dn2 = assign70760_e107188_d_n2;
        locals.var_exp_bvbs_dn4 = assign70760_e107188_d_n4;
        locals.var_exp_bvbs_dn5 = assign70760_e107188_d_n5;
        locals.var_exp_bvbs_dn6 = assign70760_e107188_d_n6;
        locals.var_exp_bvbs_dn7 = assign70760_e107188_d_n7;
        locals.var_exp_bvbs_dn8 = assign70760_e107188_d_n8;
        locals.var_exp_bvbs_dn9 = assign70760_e107188_d_n9;
        locals.var_exp_bvbs_dn10 = assign70760_e107188_d_n10;
        locals.var_exp_bvbs_dn11 = assign70760_e107188_d_n11;
        locals.var_exp_bvbs_dn14 = assign70760_e107188_d_n14;
        locals.var_exp_bvbs_rv = 0.0;

        let (assign70770_e107195, assign70770_e107195_d_n0, assign70770_e107195_d_n2, assign70770_e107195_d_n4, assign70770_e107195_d_n5, assign70770_e107195_d_n6, assign70770_e107195_d_n7, assign70770_e107195_d_n8, assign70770_e107195_d_n9, assign70770_e107195_d_n10, assign70770_e107195_d_n11, assign70770_e107195_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1660 == 0.0)) {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn11, locals.var_exp_bvbs_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign70770_e107195;
        locals.var_t0_dn0 = assign70770_e107195_d_n0;
        locals.var_t0_dn2 = assign70770_e107195_d_n2;
        locals.var_t0_dn4 = assign70770_e107195_d_n4;
        locals.var_t0_dn5 = assign70770_e107195_d_n5;
        locals.var_t0_dn6 = assign70770_e107195_d_n6;
        locals.var_t0_dn7 = assign70770_e107195_d_n7;
        locals.var_t0_dn8 = assign70770_e107195_d_n8;
        locals.var_t0_dn9 = assign70770_e107195_d_n9;
        locals.var_t0_dn10 = assign70770_e107195_d_n10;
        locals.var_t0_dn11 = assign70770_e107195_d_n11;
        locals.var_t0_dn14 = assign70770_e107195_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign70780_e107208, assign70780_e107208_d_n0, assign70780_e107208_d_n2, assign70780_e107208_d_n4, assign70780_e107208_d_n5, assign70780_e107208_d_n6, assign70780_e107208_d_n7, assign70780_e107208_d_n8, assign70780_e107208_d_n9, assign70780_e107208_d_n10, assign70780_e107208_d_n11, assign70780_e107208_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign70780_e107200: f64 = (-locals.var_vgpld);
        let assign70780_e107202: f64 = (assign70780_e107200 * 0.5);
        let assign70780_e107204: f64 = (assign70780_e107202 - 0.5);
        let assign70780_e107206: f64 = (assign70780_e107204 - 1.0);
        (assign70780_e107206, 0.0, ((-locals.var_vgpld_dn2) * 0.5), 0.0, 0.0, 0.0, ((-locals.var_vgpld_dn7) * 0.5), ((-locals.var_vgpld_dn8) * 0.5), ((-locals.var_vgpld_dn9) * 0.5), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign70780_e107208;
        locals.var_tmf1_dn0 = assign70780_e107208_d_n0;
        locals.var_tmf1_dn2 = assign70780_e107208_d_n2;
        locals.var_tmf1_dn4 = assign70780_e107208_d_n4;
        locals.var_tmf1_dn5 = assign70780_e107208_d_n5;
        locals.var_tmf1_dn6 = assign70780_e107208_d_n6;
        locals.var_tmf1_dn7 = assign70780_e107208_d_n7;
        locals.var_tmf1_dn8 = assign70780_e107208_d_n8;
        locals.var_tmf1_dn9 = assign70780_e107208_d_n9;
        locals.var_tmf1_dn10 = assign70780_e107208_d_n10;
        locals.var_tmf1_dn11 = assign70780_e107208_d_n11;
        locals.var_tmf1_dn14 = assign70780_e107208_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign70790_e107218, assign70790_e107218_d_n0, assign70790_e107218_d_n2, assign70790_e107218_d_n4, assign70790_e107218_d_n5, assign70790_e107218_d_n6, assign70790_e107218_d_n7, assign70790_e107218_d_n8, assign70790_e107218_d_n9, assign70790_e107218_d_n10, assign70790_e107218_d_n11, assign70790_e107218_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign70790_e107214: f64 = (4.0 * 0.5);
        let assign70790_e107216: f64 = assign70790_e107214;
        (assign70790_e107216, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign70790_e107218;
        locals.var_tmf2_dn0 = assign70790_e107218_d_n0;
        locals.var_tmf2_dn2 = assign70790_e107218_d_n2;
        locals.var_tmf2_dn4 = assign70790_e107218_d_n4;
        locals.var_tmf2_dn5 = assign70790_e107218_d_n5;
        locals.var_tmf2_dn6 = assign70790_e107218_d_n6;
        locals.var_tmf2_dn7 = assign70790_e107218_d_n7;
        locals.var_tmf2_dn8 = assign70790_e107218_d_n8;
        locals.var_tmf2_dn9 = assign70790_e107218_d_n9;
        locals.var_tmf2_dn10 = assign70790_e107218_d_n10;
        locals.var_tmf2_dn11 = assign70790_e107218_d_n11;
        locals.var_tmf2_dn14 = assign70790_e107218_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign70800_e107230, assign70800_e107230_d_n0, assign70800_e107230_d_n2, assign70800_e107230_d_n4, assign70800_e107230_d_n5, assign70800_e107230_d_n6, assign70800_e107230_d_n7, assign70800_e107230_d_n8, assign70800_e107230_d_n9, assign70800_e107230_d_n10, assign70800_e107230_d_n11, assign70800_e107230_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let (assign70800_e107228, assign70800_e107228_d_n0, assign70800_e107228_d_n2, assign70800_e107228_d_n4, assign70800_e107228_d_n5, assign70800_e107228_d_n6, assign70800_e107228_d_n7, assign70800_e107228_d_n8, assign70800_e107228_d_n9, assign70800_e107228_d_n10, assign70800_e107228_d_n11, assign70800_e107228_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign70800_e107227: f64 = (-locals.var_tmf2);
                (assign70800_e107227, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign70800_e107228, assign70800_e107228_d_n0, assign70800_e107228_d_n2, assign70800_e107228_d_n4, assign70800_e107228_d_n5, assign70800_e107228_d_n6, assign70800_e107228_d_n7, assign70800_e107228_d_n8, assign70800_e107228_d_n9, assign70800_e107228_d_n10, assign70800_e107228_d_n11, assign70800_e107228_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign70800_e107230;
        locals.var_tmf2_dn0 = assign70800_e107230_d_n0;
        locals.var_tmf2_dn2 = assign70800_e107230_d_n2;
        locals.var_tmf2_dn4 = assign70800_e107230_d_n4;
        locals.var_tmf2_dn5 = assign70800_e107230_d_n5;
        locals.var_tmf2_dn6 = assign70800_e107230_d_n6;
        locals.var_tmf2_dn7 = assign70800_e107230_d_n7;
        locals.var_tmf2_dn8 = assign70800_e107230_d_n8;
        locals.var_tmf2_dn9 = assign70800_e107230_d_n9;
        locals.var_tmf2_dn10 = assign70800_e107230_d_n10;
        locals.var_tmf2_dn11 = assign70800_e107230_d_n11;
        locals.var_tmf2_dn14 = assign70800_e107230_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign70810_e107241, assign70810_e107241_d_n0, assign70810_e107241_d_n2, assign70810_e107241_d_n4, assign70810_e107241_d_n5, assign70810_e107241_d_n6, assign70810_e107241_d_n7, assign70810_e107241_d_n8, assign70810_e107241_d_n9, assign70810_e107241_d_n10, assign70810_e107241_d_n11, assign70810_e107241_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign70810_e107236: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign70810_e107238: f64 = (assign70810_e107236 + locals.var_tmf2);
        let assign70810_e107239: f64 = (assign70810_e107238).sqrt();
        (assign70810_e107239, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign70810_e107239)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign70810_e107239)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign70810_e107239)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign70810_e107239)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign70810_e107239)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign70810_e107239)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign70810_e107239)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign70810_e107239)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign70810_e107239)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign70810_e107239)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign70810_e107239)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign70810_e107241;
        locals.var_tmf2_dn0 = assign70810_e107241_d_n0;
        locals.var_tmf2_dn2 = assign70810_e107241_d_n2;
        locals.var_tmf2_dn4 = assign70810_e107241_d_n4;
        locals.var_tmf2_dn5 = assign70810_e107241_d_n5;
        locals.var_tmf2_dn6 = assign70810_e107241_d_n6;
        locals.var_tmf2_dn7 = assign70810_e107241_d_n7;
        locals.var_tmf2_dn8 = assign70810_e107241_d_n8;
        locals.var_tmf2_dn9 = assign70810_e107241_d_n9;
        locals.var_tmf2_dn10 = assign70810_e107241_d_n10;
        locals.var_tmf2_dn11 = assign70810_e107241_d_n11;
        locals.var_tmf2_dn14 = assign70810_e107241_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign70820_e107253, assign70820_e107253_d_n0, assign70820_e107253_d_n2, assign70820_e107253_d_n4, assign70820_e107253_d_n5, assign70820_e107253_d_n6, assign70820_e107253_d_n7, assign70820_e107253_d_n8, assign70820_e107253_d_n9, assign70820_e107253_d_n10, assign70820_e107253_d_n11, assign70820_e107253_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign70820_e107249: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign70820_e107250: f64 = (1.0 + assign70820_e107249);
        let assign70820_e107251: f64 = (0.5 * assign70820_e107250);
        (assign70820_e107251, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign70820_e107253;
        locals.var_t0_dn0 = assign70820_e107253_d_n0;
        locals.var_t0_dn2 = assign70820_e107253_d_n2;
        locals.var_t0_dn4 = assign70820_e107253_d_n4;
        locals.var_t0_dn5 = assign70820_e107253_d_n5;
        locals.var_t0_dn6 = assign70820_e107253_d_n6;
        locals.var_t0_dn7 = assign70820_e107253_d_n7;
        locals.var_t0_dn8 = assign70820_e107253_d_n8;
        locals.var_t0_dn9 = assign70820_e107253_d_n9;
        locals.var_t0_dn10 = assign70820_e107253_d_n10;
        locals.var_t0_dn11 = assign70820_e107253_d_n11;
        locals.var_t0_dn14 = assign70820_e107253_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign70830_e107265, assign70830_e107265_d_n0, assign70830_e107265_d_n2, assign70830_e107265_d_n4, assign70830_e107265_d_n5, assign70830_e107265_d_n6, assign70830_e107265_d_n7, assign70830_e107265_d_n8, assign70830_e107265_d_n9, assign70830_e107265_d_n10, assign70830_e107265_d_n11, assign70830_e107265_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign70830_e107261: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign70830_e107262: f64 = (0.5 * assign70830_e107261);
        let assign70830_e107263: f64 = (0.5 + assign70830_e107262);
        (assign70830_e107263, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign70830_e107265;
        locals.var_t1_dn0 = assign70830_e107265_d_n0;
        locals.var_t1_dn2 = assign70830_e107265_d_n2;
        locals.var_t1_dn4 = assign70830_e107265_d_n4;
        locals.var_t1_dn5 = assign70830_e107265_d_n5;
        locals.var_t1_dn6 = assign70830_e107265_d_n6;
        locals.var_t1_dn7 = assign70830_e107265_d_n7;
        locals.var_t1_dn8 = assign70830_e107265_d_n8;
        locals.var_t1_dn9 = assign70830_e107265_d_n9;
        locals.var_t1_dn10 = assign70830_e107265_d_n10;
        locals.var_t1_dn11 = assign70830_e107265_d_n11;
        locals.var_t1_dn14 = assign70830_e107265_d_n14;
        locals.var_t1_rv = 0.0;

        let assign70840_e107268: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign70840_e107271: f64 = (-locals.var_t1);
        let assign70840_e107276: f64 = if ((assign70840_e107268 > assign70840_e107271) && (locals.var_t1 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1661 = assign70840_e107276;
        locals.var_guard1661_rv = 0.0;

        let (assign70850_e107290, assign70850_e107290_d_n0, assign70850_e107290_d_n2, assign70850_e107290_d_n4, assign70850_e107290_d_n5, assign70850_e107290_d_n6, assign70850_e107290_d_n7, assign70850_e107290_d_n8, assign70850_e107290_d_n9, assign70850_e107290_d_n10, assign70850_e107290_d_n11, assign70850_e107290_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1661 != 0.0)) {
        let assign70850_e107284: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign70850_e107286: f64 = assign70850_e107284;
        let assign70850_e107288: f64 = (assign70850_e107286 + locals.var_t1);
        (assign70850_e107288, (locals.var_vxbgmtcl_dn0 + locals.var_t1_dn0), ((locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2) + locals.var_t1_dn2), (locals.var_vxbgmtcl_dn4 + locals.var_t1_dn4), (locals.var_vxbgmtcl_dn5 + locals.var_t1_dn5), (locals.var_vxbgmtcl_dn6 + locals.var_t1_dn6), ((locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7) + locals.var_t1_dn7), ((locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8) + locals.var_t1_dn8), ((locals.var_vgpld_dn9 + locals.var_vxbgmtcl_dn9) + locals.var_t1_dn9), (locals.var_vxbgmtcl_dn10 + locals.var_t1_dn10), (locals.var_vxbgmtcl_dn11 + locals.var_t1_dn11), (locals.var_vxbgmtcl_dn14 + locals.var_t1_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign70850_e107290;
        locals.var_tmf1_dn0 = assign70850_e107290_d_n0;
        locals.var_tmf1_dn2 = assign70850_e107290_d_n2;
        locals.var_tmf1_dn4 = assign70850_e107290_d_n4;
        locals.var_tmf1_dn5 = assign70850_e107290_d_n5;
        locals.var_tmf1_dn6 = assign70850_e107290_d_n6;
        locals.var_tmf1_dn7 = assign70850_e107290_d_n7;
        locals.var_tmf1_dn8 = assign70850_e107290_d_n8;
        locals.var_tmf1_dn9 = assign70850_e107290_d_n9;
        locals.var_tmf1_dn10 = assign70850_e107290_d_n10;
        locals.var_tmf1_dn11 = assign70850_e107290_d_n11;
        locals.var_tmf1_dn14 = assign70850_e107290_d_n14;
        locals.var_tmf1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_266(
        locals: &mut StampLocals,
    ) {
        let (assign70860_e107300, assign70860_e107300_d_n0, assign70860_e107300_d_n2, assign70860_e107300_d_n4, assign70860_e107300_d_n5, assign70860_e107300_d_n6, assign70860_e107300_d_n7, assign70860_e107300_d_n8, assign70860_e107300_d_n9, assign70860_e107300_d_n10, assign70860_e107300_d_n11, assign70860_e107300_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1661 != 0.0)) {
        let assign70860_e107298: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign70860_e107298, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign70860_e107300;
        locals.var_x2_dn0 = assign70860_e107300_d_n0;
        locals.var_x2_dn2 = assign70860_e107300_d_n2;
        locals.var_x2_dn4 = assign70860_e107300_d_n4;
        locals.var_x2_dn5 = assign70860_e107300_d_n5;
        locals.var_x2_dn6 = assign70860_e107300_d_n6;
        locals.var_x2_dn7 = assign70860_e107300_d_n7;
        locals.var_x2_dn8 = assign70860_e107300_d_n8;
        locals.var_x2_dn9 = assign70860_e107300_d_n9;
        locals.var_x2_dn10 = assign70860_e107300_d_n10;
        locals.var_x2_dn11 = assign70860_e107300_d_n11;
        locals.var_x2_dn14 = assign70860_e107300_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign70870_e107310, assign70870_e107310_d_n0, assign70870_e107310_d_n2, assign70870_e107310_d_n4, assign70870_e107310_d_n5, assign70870_e107310_d_n6, assign70870_e107310_d_n7, assign70870_e107310_d_n8, assign70870_e107310_d_n9, assign70870_e107310_d_n10, assign70870_e107310_d_n11, assign70870_e107310_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1661 != 0.0)) {
        let assign70870_e107308: f64 = (locals.var_t1 * locals.var_t1);
        (assign70870_e107308, ((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)), ((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)), ((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)), ((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)), ((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)), ((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)), ((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)), ((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)), ((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)), ((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)), ((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14)),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign70870_e107310;
        locals.var_xmax2_dn0 = assign70870_e107310_d_n0;
        locals.var_xmax2_dn2 = assign70870_e107310_d_n2;
        locals.var_xmax2_dn4 = assign70870_e107310_d_n4;
        locals.var_xmax2_dn5 = assign70870_e107310_d_n5;
        locals.var_xmax2_dn6 = assign70870_e107310_d_n6;
        locals.var_xmax2_dn7 = assign70870_e107310_d_n7;
        locals.var_xmax2_dn8 = assign70870_e107310_d_n8;
        locals.var_xmax2_dn9 = assign70870_e107310_d_n9;
        locals.var_xmax2_dn10 = assign70870_e107310_d_n10;
        locals.var_xmax2_dn11 = assign70870_e107310_d_n11;
        locals.var_xmax2_dn14 = assign70870_e107310_d_n14;
        locals.var_xmax2_rv = 0.0;

        let (assign70880_e107318, assign70880_e107318_d_n0, assign70880_e107318_d_n2, assign70880_e107318_d_n4, assign70880_e107318_d_n5, assign70880_e107318_d_n6, assign70880_e107318_d_n7, assign70880_e107318_d_n8, assign70880_e107318_d_n9, assign70880_e107318_d_n10, assign70880_e107318_d_n11, assign70880_e107318_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1661 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign70880_e107318;
        locals.var_xp_dn0 = assign70880_e107318_d_n0;
        locals.var_xp_dn2 = assign70880_e107318_d_n2;
        locals.var_xp_dn4 = assign70880_e107318_d_n4;
        locals.var_xp_dn5 = assign70880_e107318_d_n5;
        locals.var_xp_dn6 = assign70880_e107318_d_n6;
        locals.var_xp_dn7 = assign70880_e107318_d_n7;
        locals.var_xp_dn8 = assign70880_e107318_d_n8;
        locals.var_xp_dn9 = assign70880_e107318_d_n9;
        locals.var_xp_dn10 = assign70880_e107318_d_n10;
        locals.var_xp_dn11 = assign70880_e107318_d_n11;
        locals.var_xp_dn14 = assign70880_e107318_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign70890_e107326, assign70890_e107326_d_n0, assign70890_e107326_d_n2, assign70890_e107326_d_n4, assign70890_e107326_d_n5, assign70890_e107326_d_n6, assign70890_e107326_d_n7, assign70890_e107326_d_n8, assign70890_e107326_d_n9, assign70890_e107326_d_n10, assign70890_e107326_d_n11, assign70890_e107326_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1661 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign70890_e107326;
        locals.var_xmp_dn0 = assign70890_e107326_d_n0;
        locals.var_xmp_dn2 = assign70890_e107326_d_n2;
        locals.var_xmp_dn4 = assign70890_e107326_d_n4;
        locals.var_xmp_dn5 = assign70890_e107326_d_n5;
        locals.var_xmp_dn6 = assign70890_e107326_d_n6;
        locals.var_xmp_dn7 = assign70890_e107326_d_n7;
        locals.var_xmp_dn8 = assign70890_e107326_d_n8;
        locals.var_xmp_dn9 = assign70890_e107326_d_n9;
        locals.var_xmp_dn10 = assign70890_e107326_d_n10;
        locals.var_xmp_dn11 = assign70890_e107326_d_n11;
        locals.var_xmp_dn14 = assign70890_e107326_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign70900_e107334,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1661 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign70900_e107334;
        locals.var_m0_rv = 0.0;

        let (assign70910_e107342,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1661 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign70910_e107342;
        locals.var_mm_rv = 0.0;

        let (assign70920_e107350, assign70920_e107350_d_n0, assign70920_e107350_d_n2, assign70920_e107350_d_n4, assign70920_e107350_d_n5, assign70920_e107350_d_n6, assign70920_e107350_d_n7, assign70920_e107350_d_n8, assign70920_e107350_d_n9, assign70920_e107350_d_n10, assign70920_e107350_d_n11, assign70920_e107350_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1661 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign70920_e107350;
        locals.var_arg_dn0 = assign70920_e107350_d_n0;
        locals.var_arg_dn2 = assign70920_e107350_d_n2;
        locals.var_arg_dn4 = assign70920_e107350_d_n4;
        locals.var_arg_dn5 = assign70920_e107350_d_n5;
        locals.var_arg_dn6 = assign70920_e107350_d_n6;
        locals.var_arg_dn7 = assign70920_e107350_d_n7;
        locals.var_arg_dn8 = assign70920_e107350_d_n8;
        locals.var_arg_dn9 = assign70920_e107350_d_n9;
        locals.var_arg_dn10 = assign70920_e107350_d_n10;
        locals.var_arg_dn11 = assign70920_e107350_d_n11;
        locals.var_arg_dn14 = assign70920_e107350_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign70930_e107358, assign70930_e107358_d_n0, assign70930_e107358_d_n2, assign70930_e107358_d_n4, assign70930_e107358_d_n5, assign70930_e107358_d_n6, assign70930_e107358_d_n7, assign70930_e107358_d_n8, assign70930_e107358_d_n9, assign70930_e107358_d_n10, assign70930_e107358_d_n11, assign70930_e107358_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1661 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign70930_e107358;
        locals.var_dnm_dn0 = assign70930_e107358_d_n0;
        locals.var_dnm_dn2 = assign70930_e107358_d_n2;
        locals.var_dnm_dn4 = assign70930_e107358_d_n4;
        locals.var_dnm_dn5 = assign70930_e107358_d_n5;
        locals.var_dnm_dn6 = assign70930_e107358_d_n6;
        locals.var_dnm_dn7 = assign70930_e107358_d_n7;
        locals.var_dnm_dn8 = assign70930_e107358_d_n8;
        locals.var_dnm_dn9 = assign70930_e107358_d_n9;
        locals.var_dnm_dn10 = assign70930_e107358_d_n10;
        locals.var_dnm_dn11 = assign70930_e107358_d_n11;
        locals.var_dnm_dn14 = assign70930_e107358_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign70940_e107368, assign70940_e107368_d_n0, assign70940_e107368_d_n2, assign70940_e107368_d_n4, assign70940_e107368_d_n5, assign70940_e107368_d_n6, assign70940_e107368_d_n7, assign70940_e107368_d_n8, assign70940_e107368_d_n9, assign70940_e107368_d_n10, assign70940_e107368_d_n11, assign70940_e107368_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1661 != 0.0)) {
        let assign70940_e107366: f64 = (locals.var_xp * locals.var_x2);
        (assign70940_e107366, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign70940_e107368;
        locals.var_xp_dn0 = assign70940_e107368_d_n0;
        locals.var_xp_dn2 = assign70940_e107368_d_n2;
        locals.var_xp_dn4 = assign70940_e107368_d_n4;
        locals.var_xp_dn5 = assign70940_e107368_d_n5;
        locals.var_xp_dn6 = assign70940_e107368_d_n6;
        locals.var_xp_dn7 = assign70940_e107368_d_n7;
        locals.var_xp_dn8 = assign70940_e107368_d_n8;
        locals.var_xp_dn9 = assign70940_e107368_d_n9;
        locals.var_xp_dn10 = assign70940_e107368_d_n10;
        locals.var_xp_dn11 = assign70940_e107368_d_n11;
        locals.var_xp_dn14 = assign70940_e107368_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign70950_e107378, assign70950_e107378_d_n0, assign70950_e107378_d_n2, assign70950_e107378_d_n4, assign70950_e107378_d_n5, assign70950_e107378_d_n6, assign70950_e107378_d_n7, assign70950_e107378_d_n8, assign70950_e107378_d_n9, assign70950_e107378_d_n10, assign70950_e107378_d_n11, assign70950_e107378_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1661 != 0.0)) {
        let assign70950_e107376: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign70950_e107376, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign70950_e107378;
        locals.var_xmp_dn0 = assign70950_e107378_d_n0;
        locals.var_xmp_dn2 = assign70950_e107378_d_n2;
        locals.var_xmp_dn4 = assign70950_e107378_d_n4;
        locals.var_xmp_dn5 = assign70950_e107378_d_n5;
        locals.var_xmp_dn6 = assign70950_e107378_d_n6;
        locals.var_xmp_dn7 = assign70950_e107378_d_n7;
        locals.var_xmp_dn8 = assign70950_e107378_d_n8;
        locals.var_xmp_dn9 = assign70950_e107378_d_n9;
        locals.var_xmp_dn10 = assign70950_e107378_d_n10;
        locals.var_xmp_dn11 = assign70950_e107378_d_n11;
        locals.var_xmp_dn14 = assign70950_e107378_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign70960_e107388, assign70960_e107388_d_n0, assign70960_e107388_d_n2, assign70960_e107388_d_n4, assign70960_e107388_d_n5, assign70960_e107388_d_n6, assign70960_e107388_d_n7, assign70960_e107388_d_n8, assign70960_e107388_d_n9, assign70960_e107388_d_n10, assign70960_e107388_d_n11, assign70960_e107388_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1661 != 0.0)) {
        let assign70960_e107386: f64 = (locals.var_xp + locals.var_xmp);
        (assign70960_e107386, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign70960_e107388;
        locals.var_arg_dn0 = assign70960_e107388_d_n0;
        locals.var_arg_dn2 = assign70960_e107388_d_n2;
        locals.var_arg_dn4 = assign70960_e107388_d_n4;
        locals.var_arg_dn5 = assign70960_e107388_d_n5;
        locals.var_arg_dn6 = assign70960_e107388_d_n6;
        locals.var_arg_dn7 = assign70960_e107388_d_n7;
        locals.var_arg_dn8 = assign70960_e107388_d_n8;
        locals.var_arg_dn9 = assign70960_e107388_d_n9;
        locals.var_arg_dn10 = assign70960_e107388_d_n10;
        locals.var_arg_dn11 = assign70960_e107388_d_n11;
        locals.var_arg_dn14 = assign70960_e107388_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign70970_e107396, assign70970_e107396_d_n0, assign70970_e107396_d_n2, assign70970_e107396_d_n4, assign70970_e107396_d_n5, assign70970_e107396_d_n6, assign70970_e107396_d_n7, assign70970_e107396_d_n8, assign70970_e107396_d_n9, assign70970_e107396_d_n10, assign70970_e107396_d_n11, assign70970_e107396_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1661 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign70970_e107396;
        locals.var_dnm_dn0 = assign70970_e107396_d_n0;
        locals.var_dnm_dn2 = assign70970_e107396_d_n2;
        locals.var_dnm_dn4 = assign70970_e107396_d_n4;
        locals.var_dnm_dn5 = assign70970_e107396_d_n5;
        locals.var_dnm_dn6 = assign70970_e107396_d_n6;
        locals.var_dnm_dn7 = assign70970_e107396_d_n7;
        locals.var_dnm_dn8 = assign70970_e107396_d_n8;
        locals.var_dnm_dn9 = assign70970_e107396_d_n9;
        locals.var_dnm_dn10 = assign70970_e107396_d_n10;
        locals.var_dnm_dn11 = assign70970_e107396_d_n11;
        locals.var_dnm_dn14 = assign70970_e107396_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign70980_e107411: f64 = if ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1662 = assign70980_e107411;
        locals.var_guard1662_rv = 0.0;

        let assign70990_e107414: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1663 = assign70990_e107414;
        locals.var_guard1663_rv = 0.0;

        let (assign71000_e107426,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1661 != 0.0)) && (locals.var_guard1662 != 0.0)) && (locals.var_guard1663 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign71000_e107426;
        locals.var_mm_rv = 0.0;

        let assign71010_e107429: f64 = if 1.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1664 = assign71010_e107429;
        locals.var_guard1664_rv = 0.0;

        let (assign71020_e107444,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1661 != 0.0)) && (locals.var_guard1662 != 0.0)) && (locals.var_guard1663 == 0.0)) && (locals.var_guard1664 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign71020_e107444;
        locals.var_mm_rv = 0.0;

        let assign71030_e107447: f64 = if 1.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1665 = assign71030_e107447;
        locals.var_guard1665_rv = 0.0;

        let (assign71040_e107465,) = {
    if (((((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1661 != 0.0)) && (locals.var_guard1662 != 0.0)) && (locals.var_guard1663 == 0.0)) && (locals.var_guard1664 == 0.0)) && (locals.var_guard1665 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign71040_e107465;
        locals.var_mm_rv = 0.0;

        let assign71050_e107468: f64 = if 1.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1666 = assign71050_e107468;
        locals.var_guard1666_rv = 0.0;

        let (assign71060_e107489,) = {
    if ((((((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1661 != 0.0)) && (locals.var_guard1662 != 0.0)) && (locals.var_guard1663 == 0.0)) && (locals.var_guard1664 == 0.0)) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1666 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign71060_e107489;
        locals.var_mm_rv = 0.0;

        let (assign71070_e107499,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1661 != 0.0)) && (locals.var_guard1662 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign71070_e107499;
        locals.var_m0_rv = 0.0;

        let mut assign71080_loop_guard: usize = 0;
        while {
            let assign71080_cond_e107510: f64 = if (((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1661 != 0.0)) && (locals.var_guard1662 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign71080_cond_e107510 != 0.0
        } {
            assign71080_loop_guard += 1;
            assert!(assign71080_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign71080_body0_e107521, assign71080_body0_e107521_d_n0, assign71080_body0_e107521_d_n2, assign71080_body0_e107521_d_n4, assign71080_body0_e107521_d_n5, assign71080_body0_e107521_d_n6, assign71080_body0_e107521_d_n7, assign71080_body0_e107521_d_n8, assign71080_body0_e107521_d_n9, assign71080_body0_e107521_d_n10, assign71080_body0_e107521_d_n11, assign71080_body0_e107521_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1661 != 0.0)) && (locals.var_guard1662 != 0.0)) {
        let assign71080_body0_e107519: f64 = (locals.var_dnm).sqrt();
        (assign71080_body0_e107519, (locals.var_dnm_dn0 / (2.0 * assign71080_body0_e107519)), (locals.var_dnm_dn2 / (2.0 * assign71080_body0_e107519)), (locals.var_dnm_dn4 / (2.0 * assign71080_body0_e107519)), (locals.var_dnm_dn5 / (2.0 * assign71080_body0_e107519)), (locals.var_dnm_dn6 / (2.0 * assign71080_body0_e107519)), (locals.var_dnm_dn7 / (2.0 * assign71080_body0_e107519)), (locals.var_dnm_dn8 / (2.0 * assign71080_body0_e107519)), (locals.var_dnm_dn9 / (2.0 * assign71080_body0_e107519)), (locals.var_dnm_dn10 / (2.0 * assign71080_body0_e107519)), (locals.var_dnm_dn11 / (2.0 * assign71080_body0_e107519)), (locals.var_dnm_dn14 / (2.0 * assign71080_body0_e107519)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign71080_body0_e107521;
            locals.var_dnm_dn0 = assign71080_body0_e107521_d_n0;
            locals.var_dnm_dn2 = assign71080_body0_e107521_d_n2;
            locals.var_dnm_dn4 = assign71080_body0_e107521_d_n4;
            locals.var_dnm_dn5 = assign71080_body0_e107521_d_n5;
            locals.var_dnm_dn6 = assign71080_body0_e107521_d_n6;
            locals.var_dnm_dn7 = assign71080_body0_e107521_d_n7;
            locals.var_dnm_dn8 = assign71080_body0_e107521_d_n8;
            locals.var_dnm_dn9 = assign71080_body0_e107521_d_n9;
            locals.var_dnm_dn10 = assign71080_body0_e107521_d_n10;
            locals.var_dnm_dn11 = assign71080_body0_e107521_d_n11;
            locals.var_dnm_dn14 = assign71080_body0_e107521_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign71080_body1_e107533,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1661 != 0.0)) && (locals.var_guard1662 != 0.0)) {
        let assign71080_body1_e107531: f64 = (locals.var_m0 + 1.0);
        (assign71080_body1_e107531,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign71080_body1_e107533;
            locals.var_m0_rv = 0.0;
        }

        let (assign71090_e107555, assign71090_e107555_d_n0, assign71090_e107555_d_n2, assign71090_e107555_d_n4, assign71090_e107555_d_n5, assign71090_e107555_d_n6, assign71090_e107555_d_n7, assign71090_e107555_d_n8, assign71090_e107555_d_n9, assign71090_e107555_d_n10, assign71090_e107555_d_n11, assign71090_e107555_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1661 != 0.0)) && (locals.var_guard1662 == 0.0)) {
        let (assign71090_e107553, assign71090_e107553_d_n0, assign71090_e107553_d_n2, assign71090_e107553_d_n4, assign71090_e107553_d_n5, assign71090_e107553_d_n6, assign71090_e107553_d_n7, assign71090_e107553_d_n8, assign71090_e107553_d_n9, assign71090_e107553_d_n10, assign71090_e107553_d_n11, assign71090_e107553_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign71090_e107550: f64 = 2.0;
                let assign71090_e107551: f64 = (1.0 / assign71090_e107550);
                let assign71090_e107552: f64 = (locals.var_dnm).powf(assign71090_e107551);
                (assign71090_e107552, if 0.0 == 0.0 && ((assign71090_e107551) as f64).is_finite() && ((assign71090_e107551) as f64).fract() == 0.0 { if assign71090_e107551 == 0.0 { 0.0 } else { (assign71090_e107551 * ((locals.var_dnm).powf(assign71090_e107551 - 1.0) * locals.var_dnm_dn0)) } } else { (assign71090_e107552 * (assign71090_e107551 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign71090_e107551) as f64).is_finite() && ((assign71090_e107551) as f64).fract() == 0.0 { if assign71090_e107551 == 0.0 { 0.0 } else { (assign71090_e107551 * ((locals.var_dnm).powf(assign71090_e107551 - 1.0) * locals.var_dnm_dn2)) } } else { (assign71090_e107552 * (assign71090_e107551 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign71090_e107551) as f64).is_finite() && ((assign71090_e107551) as f64).fract() == 0.0 { if assign71090_e107551 == 0.0 { 0.0 } else { (assign71090_e107551 * ((locals.var_dnm).powf(assign71090_e107551 - 1.0) * locals.var_dnm_dn4)) } } else { (assign71090_e107552 * (assign71090_e107551 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign71090_e107551) as f64).is_finite() && ((assign71090_e107551) as f64).fract() == 0.0 { if assign71090_e107551 == 0.0 { 0.0 } else { (assign71090_e107551 * ((locals.var_dnm).powf(assign71090_e107551 - 1.0) * locals.var_dnm_dn5)) } } else { (assign71090_e107552 * (assign71090_e107551 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign71090_e107551) as f64).is_finite() && ((assign71090_e107551) as f64).fract() == 0.0 { if assign71090_e107551 == 0.0 { 0.0 } else { (assign71090_e107551 * ((locals.var_dnm).powf(assign71090_e107551 - 1.0) * locals.var_dnm_dn6)) } } else { (assign71090_e107552 * (assign71090_e107551 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign71090_e107551) as f64).is_finite() && ((assign71090_e107551) as f64).fract() == 0.0 { if assign71090_e107551 == 0.0 { 0.0 } else { (assign71090_e107551 * ((locals.var_dnm).powf(assign71090_e107551 - 1.0) * locals.var_dnm_dn7)) } } else { (assign71090_e107552 * (assign71090_e107551 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign71090_e107551) as f64).is_finite() && ((assign71090_e107551) as f64).fract() == 0.0 { if assign71090_e107551 == 0.0 { 0.0 } else { (assign71090_e107551 * ((locals.var_dnm).powf(assign71090_e107551 - 1.0) * locals.var_dnm_dn8)) } } else { (assign71090_e107552 * (assign71090_e107551 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign71090_e107551) as f64).is_finite() && ((assign71090_e107551) as f64).fract() == 0.0 { if assign71090_e107551 == 0.0 { 0.0 } else { (assign71090_e107551 * ((locals.var_dnm).powf(assign71090_e107551 - 1.0) * locals.var_dnm_dn9)) } } else { (assign71090_e107552 * (assign71090_e107551 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign71090_e107551) as f64).is_finite() && ((assign71090_e107551) as f64).fract() == 0.0 { if assign71090_e107551 == 0.0 { 0.0 } else { (assign71090_e107551 * ((locals.var_dnm).powf(assign71090_e107551 - 1.0) * locals.var_dnm_dn10)) } } else { (assign71090_e107552 * (assign71090_e107551 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign71090_e107551) as f64).is_finite() && ((assign71090_e107551) as f64).fract() == 0.0 { if assign71090_e107551 == 0.0 { 0.0 } else { (assign71090_e107551 * ((locals.var_dnm).powf(assign71090_e107551 - 1.0) * locals.var_dnm_dn11)) } } else { (assign71090_e107552 * (assign71090_e107551 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign71090_e107551) as f64).is_finite() && ((assign71090_e107551) as f64).fract() == 0.0 { if assign71090_e107551 == 0.0 { 0.0 } else { (assign71090_e107551 * ((locals.var_dnm).powf(assign71090_e107551 - 1.0) * locals.var_dnm_dn14)) } } else { (assign71090_e107552 * (assign71090_e107551 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign71090_e107553, assign71090_e107553_d_n0, assign71090_e107553_d_n2, assign71090_e107553_d_n4, assign71090_e107553_d_n5, assign71090_e107553_d_n6, assign71090_e107553_d_n7, assign71090_e107553_d_n8, assign71090_e107553_d_n9, assign71090_e107553_d_n10, assign71090_e107553_d_n11, assign71090_e107553_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign71090_e107555;
        locals.var_dnm_dn0 = assign71090_e107555_d_n0;
        locals.var_dnm_dn2 = assign71090_e107555_d_n2;
        locals.var_dnm_dn4 = assign71090_e107555_d_n4;
        locals.var_dnm_dn5 = assign71090_e107555_d_n5;
        locals.var_dnm_dn6 = assign71090_e107555_d_n6;
        locals.var_dnm_dn7 = assign71090_e107555_d_n7;
        locals.var_dnm_dn8 = assign71090_e107555_d_n8;
        locals.var_dnm_dn9 = assign71090_e107555_d_n9;
        locals.var_dnm_dn10 = assign71090_e107555_d_n10;
        locals.var_dnm_dn11 = assign71090_e107555_d_n11;
        locals.var_dnm_dn14 = assign71090_e107555_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign71100_e107565, assign71100_e107565_d_n0, assign71100_e107565_d_n2, assign71100_e107565_d_n4, assign71100_e107565_d_n5, assign71100_e107565_d_n6, assign71100_e107565_d_n7, assign71100_e107565_d_n8, assign71100_e107565_d_n9, assign71100_e107565_d_n10, assign71100_e107565_d_n11, assign71100_e107565_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1661 != 0.0)) {
        let assign71100_e107563: f64 = (1.0 / locals.var_dnm);
        (assign71100_e107563, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign71100_e107565;
        locals.var_dnm_dn0 = assign71100_e107565_d_n0;
        locals.var_dnm_dn2 = assign71100_e107565_d_n2;
        locals.var_dnm_dn4 = assign71100_e107565_d_n4;
        locals.var_dnm_dn5 = assign71100_e107565_d_n5;
        locals.var_dnm_dn6 = assign71100_e107565_d_n6;
        locals.var_dnm_dn7 = assign71100_e107565_d_n7;
        locals.var_dnm_dn8 = assign71100_e107565_d_n8;
        locals.var_dnm_dn9 = assign71100_e107565_d_n9;
        locals.var_dnm_dn10 = assign71100_e107565_d_n10;
        locals.var_dnm_dn11 = assign71100_e107565_d_n11;
        locals.var_dnm_dn14 = assign71100_e107565_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign71110_e107577, assign71110_e107577_d_n0, assign71110_e107577_d_n2, assign71110_e107577_d_n4, assign71110_e107577_d_n5, assign71110_e107577_d_n6, assign71110_e107577_d_n7, assign71110_e107577_d_n8, assign71110_e107577_d_n9, assign71110_e107577_d_n10, assign71110_e107577_d_n11, assign71110_e107577_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1661 != 0.0)) {
        let assign71110_e107573: f64 = (locals.var_tmf1 * locals.var_t1);
        let assign71110_e107575: f64 = (assign71110_e107573 * locals.var_dnm);
        (assign71110_e107575, ((((locals.var_tmf1_dn0 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn0)) * locals.var_dnm) + (assign71110_e107573 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn2)) * locals.var_dnm) + (assign71110_e107573 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn4 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn4)) * locals.var_dnm) + (assign71110_e107573 * locals.var_dnm_dn4)), ((((locals.var_tmf1_dn5 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn5)) * locals.var_dnm) + (assign71110_e107573 * locals.var_dnm_dn5)), ((((locals.var_tmf1_dn6 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn6)) * locals.var_dnm) + (assign71110_e107573 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn7)) * locals.var_dnm) + (assign71110_e107573 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn8 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn8)) * locals.var_dnm) + (assign71110_e107573 * locals.var_dnm_dn8)), ((((locals.var_tmf1_dn9 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn9)) * locals.var_dnm) + (assign71110_e107573 * locals.var_dnm_dn9)), ((((locals.var_tmf1_dn10 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn10)) * locals.var_dnm) + (assign71110_e107573 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn11 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn11)) * locals.var_dnm) + (assign71110_e107573 * locals.var_dnm_dn11)), ((((locals.var_tmf1_dn14 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn14)) * locals.var_dnm) + (assign71110_e107573 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign71110_e107577;
        locals.var_tmf0_dn0 = assign71110_e107577_d_n0;
        locals.var_tmf0_dn2 = assign71110_e107577_d_n2;
        locals.var_tmf0_dn4 = assign71110_e107577_d_n4;
        locals.var_tmf0_dn5 = assign71110_e107577_d_n5;
        locals.var_tmf0_dn6 = assign71110_e107577_d_n6;
        locals.var_tmf0_dn7 = assign71110_e107577_d_n7;
        locals.var_tmf0_dn8 = assign71110_e107577_d_n8;
        locals.var_tmf0_dn9 = assign71110_e107577_d_n9;
        locals.var_tmf0_dn10 = assign71110_e107577_d_n10;
        locals.var_tmf0_dn11 = assign71110_e107577_d_n11;
        locals.var_tmf0_dn14 = assign71110_e107577_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign71120_e107591, assign71120_e107591_d_n0, assign71120_e107591_d_n2, assign71120_e107591_d_n4, assign71120_e107591_d_n5, assign71120_e107591_d_n6, assign71120_e107591_d_n7, assign71120_e107591_d_n8, assign71120_e107591_d_n9, assign71120_e107591_d_n10, assign71120_e107591_d_n11, assign71120_e107591_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1661 != 0.0)) {
        let assign71120_e107585: f64 = (locals.var_t1 * locals.var_xmp);
        let assign71120_e107587: f64 = (assign71120_e107585 * locals.var_dnm);
        let assign71120_e107589: f64 = (assign71120_e107587 / locals.var_arg);
        (assign71120_e107589, (((((((locals.var_t1_dn0 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn0)) * locals.var_dnm) + (assign71120_e107585 * locals.var_dnm_dn0)) * locals.var_arg) - (assign71120_e107587 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn2 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn2)) * locals.var_dnm) + (assign71120_e107585 * locals.var_dnm_dn2)) * locals.var_arg) - (assign71120_e107587 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn4 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn4)) * locals.var_dnm) + (assign71120_e107585 * locals.var_dnm_dn4)) * locals.var_arg) - (assign71120_e107587 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn5 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn5)) * locals.var_dnm) + (assign71120_e107585 * locals.var_dnm_dn5)) * locals.var_arg) - (assign71120_e107587 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn6 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn6)) * locals.var_dnm) + (assign71120_e107585 * locals.var_dnm_dn6)) * locals.var_arg) - (assign71120_e107587 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn7 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn7)) * locals.var_dnm) + (assign71120_e107585 * locals.var_dnm_dn7)) * locals.var_arg) - (assign71120_e107587 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn8 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn8)) * locals.var_dnm) + (assign71120_e107585 * locals.var_dnm_dn8)) * locals.var_arg) - (assign71120_e107587 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn9 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn9)) * locals.var_dnm) + (assign71120_e107585 * locals.var_dnm_dn9)) * locals.var_arg) - (assign71120_e107587 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn10 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn10)) * locals.var_dnm) + (assign71120_e107585 * locals.var_dnm_dn10)) * locals.var_arg) - (assign71120_e107587 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn11 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn11)) * locals.var_dnm) + (assign71120_e107585 * locals.var_dnm_dn11)) * locals.var_arg) - (assign71120_e107587 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn14 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn14)) * locals.var_dnm) + (assign71120_e107585 * locals.var_dnm_dn14)) * locals.var_arg) - (assign71120_e107587 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign71120_e107591;
        locals.var_t0_dn0 = assign71120_e107591_d_n0;
        locals.var_t0_dn2 = assign71120_e107591_d_n2;
        locals.var_t0_dn4 = assign71120_e107591_d_n4;
        locals.var_t0_dn5 = assign71120_e107591_d_n5;
        locals.var_t0_dn6 = assign71120_e107591_d_n6;
        locals.var_t0_dn7 = assign71120_e107591_d_n7;
        locals.var_t0_dn8 = assign71120_e107591_d_n8;
        locals.var_t0_dn9 = assign71120_e107591_d_n9;
        locals.var_t0_dn10 = assign71120_e107591_d_n10;
        locals.var_t0_dn11 = assign71120_e107591_d_n11;
        locals.var_t0_dn14 = assign71120_e107591_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign71130_e107603, assign71130_e107603_d_n0, assign71130_e107603_d_n2, assign71130_e107603_d_n4, assign71130_e107603_d_n5, assign71130_e107603_d_n6, assign71130_e107603_d_n7, assign71130_e107603_d_n8, assign71130_e107603_d_n9, assign71130_e107603_d_n10, assign71130_e107603_d_n11, assign71130_e107603_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1661 != 0.0)) {
        let assign71130_e107599: f64 = (-locals.var_t1);
        let assign71130_e107601: f64 = (assign71130_e107599 + locals.var_tmf0);
        (assign71130_e107601, ((-locals.var_t1_dn0) + locals.var_tmf0_dn0), ((-locals.var_t1_dn2) + locals.var_tmf0_dn2), ((-locals.var_t1_dn4) + locals.var_tmf0_dn4), ((-locals.var_t1_dn5) + locals.var_tmf0_dn5), ((-locals.var_t1_dn6) + locals.var_tmf0_dn6), ((-locals.var_t1_dn7) + locals.var_tmf0_dn7), ((-locals.var_t1_dn8) + locals.var_tmf0_dn8), ((-locals.var_t1_dn9) + locals.var_tmf0_dn9), ((-locals.var_t1_dn10) + locals.var_tmf0_dn10), ((-locals.var_t1_dn11) + locals.var_tmf0_dn11), ((-locals.var_t1_dn14) + locals.var_tmf0_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign71130_e107603;
        locals.var_t1_dn0 = assign71130_e107603_d_n0;
        locals.var_t1_dn2 = assign71130_e107603_d_n2;
        locals.var_t1_dn4 = assign71130_e107603_d_n4;
        locals.var_t1_dn5 = assign71130_e107603_d_n5;
        locals.var_t1_dn6 = assign71130_e107603_d_n6;
        locals.var_t1_dn7 = assign71130_e107603_d_n7;
        locals.var_t1_dn8 = assign71130_e107603_d_n8;
        locals.var_t1_dn9 = assign71130_e107603_d_n9;
        locals.var_t1_dn10 = assign71130_e107603_d_n10;
        locals.var_t1_dn11 = assign71130_e107603_d_n11;
        locals.var_t1_dn14 = assign71130_e107603_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign71140_e107611, assign71140_e107611_d_n0, assign71140_e107611_d_n2, assign71140_e107611_d_n4, assign71140_e107611_d_n5, assign71140_e107611_d_n6, assign71140_e107611_d_n7, assign71140_e107611_d_n8, assign71140_e107611_d_n9, assign71140_e107611_d_n10, assign71140_e107611_d_n11, assign71140_e107611_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1661 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign71140_e107611;
        locals.var_t0_dn0 = assign71140_e107611_d_n0;
        locals.var_t0_dn2 = assign71140_e107611_d_n2;
        locals.var_t0_dn4 = assign71140_e107611_d_n4;
        locals.var_t0_dn5 = assign71140_e107611_d_n5;
        locals.var_t0_dn6 = assign71140_e107611_d_n6;
        locals.var_t0_dn7 = assign71140_e107611_d_n7;
        locals.var_t0_dn8 = assign71140_e107611_d_n8;
        locals.var_t0_dn9 = assign71140_e107611_d_n9;
        locals.var_t0_dn10 = assign71140_e107611_d_n10;
        locals.var_t0_dn11 = assign71140_e107611_d_n11;
        locals.var_t0_dn14 = assign71140_e107611_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign71150_e107622, assign71150_e107622_d_n0, assign71150_e107622_d_n2, assign71150_e107622_d_n4, assign71150_e107622_d_n5, assign71150_e107622_d_n6, assign71150_e107622_d_n7, assign71150_e107622_d_n8, assign71150_e107622_d_n9, assign71150_e107622_d_n10, assign71150_e107622_d_n11, assign71150_e107622_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1661 == 0.0)) {
        let assign71150_e107620: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        (assign71150_e107620, locals.var_vxbgmtcl_dn0, (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2), locals.var_vxbgmtcl_dn4, locals.var_vxbgmtcl_dn5, locals.var_vxbgmtcl_dn6, (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7), (locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8), (locals.var_vgpld_dn9 + locals.var_vxbgmtcl_dn9), locals.var_vxbgmtcl_dn10, locals.var_vxbgmtcl_dn11, locals.var_vxbgmtcl_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign71150_e107622;
        locals.var_t1_dn0 = assign71150_e107622_d_n0;
        locals.var_t1_dn2 = assign71150_e107622_d_n2;
        locals.var_t1_dn4 = assign71150_e107622_d_n4;
        locals.var_t1_dn5 = assign71150_e107622_d_n5;
        locals.var_t1_dn6 = assign71150_e107622_d_n6;
        locals.var_t1_dn7 = assign71150_e107622_d_n7;
        locals.var_t1_dn8 = assign71150_e107622_d_n8;
        locals.var_t1_dn9 = assign71150_e107622_d_n9;
        locals.var_t1_dn10 = assign71150_e107622_d_n10;
        locals.var_t1_dn11 = assign71150_e107622_d_n11;
        locals.var_t1_dn14 = assign71150_e107622_d_n14;
        locals.var_t1_rv = 0.0;

    }
}
