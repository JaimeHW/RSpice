#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_176(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign30010_e47356, assign30010_e47356_d_n0, assign30010_e47356_d_n1, assign30010_e47356_d_n2, assign30010_e47356_d_n3, assign30010_e47356_d_n4, assign30010_e47356_d_n5, assign30010_e47356_d_n6, assign30010_e47356_d_n7, assign30010_e47356_d_n8, assign30010_e47356_d_n9, assign30010_e47356_d_n12, assign30010_e47356_d_n14, assign30010_e47356_d_n15, assign30010_e47356_d_n16, assign30010_e47356_d_n17, assign30010_e47356_d_n18, assign30010_e47356_d_n19, assign30010_e47356_d_n20, assign30010_e47356_d_n21, assign30010_e47356_d_n22,) = {
    if (((locals.var_guard504 == 0.0) && (locals.var_guard513 != 0.0)) && (locals.var_guard516 != 0.0)) {
        let assign30010_e47353: f64 = { let limited_exp_arg = locals.var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign30010_e47354: f64 = (1.0 + assign30010_e47353);
        (assign30010_e47354, ({ let limited_exp_arg = locals.var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg12_dn0), ({ let limited_exp_arg = locals.var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg12_dn1), ({ let limited_exp_arg = locals.var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg12_dn2), ({ let limited_exp_arg = locals.var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg12_dn3), ({ let limited_exp_arg = locals.var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg12_dn4), ({ let limited_exp_arg = locals.var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg12_dn5), ({ let limited_exp_arg = locals.var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg12_dn6), ({ let limited_exp_arg = locals.var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg12_dn7), ({ let limited_exp_arg = locals.var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg12_dn8), ({ let limited_exp_arg = locals.var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg12_dn9), ({ let limited_exp_arg = locals.var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg12_dn12), ({ let limited_exp_arg = locals.var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg12_dn14), ({ let limited_exp_arg = locals.var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg12_dn15), ({ let limited_exp_arg = locals.var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg12_dn16), ({ let limited_exp_arg = locals.var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg12_dn17), ({ let limited_exp_arg = locals.var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg12_dn18), ({ let limited_exp_arg = locals.var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg12_dn19), ({ let limited_exp_arg = locals.var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg12_dn20), ({ let limited_exp_arg = locals.var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg12_dn21), ({ let limited_exp_arg = locals.var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg12_dn22),)
    } else {
        (locals.var_t5dg12, locals.var_t5dg12_dn0, locals.var_t5dg12_dn1, locals.var_t5dg12_dn2, locals.var_t5dg12_dn3, locals.var_t5dg12_dn4, locals.var_t5dg12_dn5, locals.var_t5dg12_dn6, locals.var_t5dg12_dn7, locals.var_t5dg12_dn8, locals.var_t5dg12_dn9, locals.var_t5dg12_dn12, locals.var_t5dg12_dn14, locals.var_t5dg12_dn15, locals.var_t5dg12_dn16, locals.var_t5dg12_dn17, locals.var_t5dg12_dn18, locals.var_t5dg12_dn19, locals.var_t5dg12_dn20, locals.var_t5dg12_dn21, locals.var_t5dg12_dn22,)
    }
};
        locals.var_t5dg12 = assign30010_e47356;
        locals.var_t5dg12_dn0 = assign30010_e47356_d_n0;
        locals.var_t5dg12_dn1 = assign30010_e47356_d_n1;
        locals.var_t5dg12_dn2 = assign30010_e47356_d_n2;
        locals.var_t5dg12_dn3 = assign30010_e47356_d_n3;
        locals.var_t5dg12_dn4 = assign30010_e47356_d_n4;
        locals.var_t5dg12_dn5 = assign30010_e47356_d_n5;
        locals.var_t5dg12_dn6 = assign30010_e47356_d_n6;
        locals.var_t5dg12_dn7 = assign30010_e47356_d_n7;
        locals.var_t5dg12_dn8 = assign30010_e47356_d_n8;
        locals.var_t5dg12_dn9 = assign30010_e47356_d_n9;
        locals.var_t5dg12_dn12 = assign30010_e47356_d_n12;
        locals.var_t5dg12_dn14 = assign30010_e47356_d_n14;
        locals.var_t5dg12_dn15 = assign30010_e47356_d_n15;
        locals.var_t5dg12_dn16 = assign30010_e47356_d_n16;
        locals.var_t5dg12_dn17 = assign30010_e47356_d_n17;
        locals.var_t5dg12_dn18 = assign30010_e47356_d_n18;
        locals.var_t5dg12_dn19 = assign30010_e47356_d_n19;
        locals.var_t5dg12_dn20 = assign30010_e47356_d_n20;
        locals.var_t5dg12_dn21 = assign30010_e47356_d_n21;
        locals.var_t5dg12_dn22 = assign30010_e47356_d_n22;

        let (assign30020_e47376, assign30020_e47376_d_n0, assign30020_e47376_d_n1, assign30020_e47376_d_n2, assign30020_e47376_d_n3, assign30020_e47376_d_n4, assign30020_e47376_d_n5, assign30020_e47376_d_n6, assign30020_e47376_d_n7, assign30020_e47376_d_n8, assign30020_e47376_d_n9, assign30020_e47376_d_n12, assign30020_e47376_d_n14, assign30020_e47376_d_n15, assign30020_e47376_d_n16, assign30020_e47376_d_n17, assign30020_e47376_d_n18, assign30020_e47376_d_n19, assign30020_e47376_d_n20, assign30020_e47376_d_n21, assign30020_e47376_d_n22,) = {
    if (((locals.var_guard504 == 0.0) && (locals.var_guard513 != 0.0)) && (locals.var_guard516 != 0.0)) {
        let assign30020_e47364: f64 = (-1.0);
        let assign30020_e47366: f64 = (assign30020_e47364 * locals.var_cch);
        let assign30020_e47369: f64 = (locals.var_t5ng02 / locals.var_t5dg02);
        let assign30020_e47370: f64 = (assign30020_e47366 - assign30020_e47369);
        let assign30020_e47373: f64 = (locals.var_t5ng12 / locals.var_t5dg12);
        let assign30020_e47374: f64 = (assign30020_e47370 - assign30020_e47373);
        (assign30020_e47374, ((-(((locals.var_t5ng02_dn0 * locals.var_t5dg02) - (locals.var_t5ng02 * locals.var_t5dg02_dn0)) / (locals.var_t5dg02 * locals.var_t5dg02))) - (((locals.var_t5ng12_dn0 * locals.var_t5dg12) - (locals.var_t5ng12 * locals.var_t5dg12_dn0)) / (locals.var_t5dg12 * locals.var_t5dg12))), ((-(((locals.var_t5ng02_dn1 * locals.var_t5dg02) - (locals.var_t5ng02 * locals.var_t5dg02_dn1)) / (locals.var_t5dg02 * locals.var_t5dg02))) - (((locals.var_t5ng12_dn1 * locals.var_t5dg12) - (locals.var_t5ng12 * locals.var_t5dg12_dn1)) / (locals.var_t5dg12 * locals.var_t5dg12))), ((-(((locals.var_t5ng02_dn2 * locals.var_t5dg02) - (locals.var_t5ng02 * locals.var_t5dg02_dn2)) / (locals.var_t5dg02 * locals.var_t5dg02))) - (((locals.var_t5ng12_dn2 * locals.var_t5dg12) - (locals.var_t5ng12 * locals.var_t5dg12_dn2)) / (locals.var_t5dg12 * locals.var_t5dg12))), ((-(((locals.var_t5ng02_dn3 * locals.var_t5dg02) - (locals.var_t5ng02 * locals.var_t5dg02_dn3)) / (locals.var_t5dg02 * locals.var_t5dg02))) - (((locals.var_t5ng12_dn3 * locals.var_t5dg12) - (locals.var_t5ng12 * locals.var_t5dg12_dn3)) / (locals.var_t5dg12 * locals.var_t5dg12))), ((-(((locals.var_t5ng02_dn4 * locals.var_t5dg02) - (locals.var_t5ng02 * locals.var_t5dg02_dn4)) / (locals.var_t5dg02 * locals.var_t5dg02))) - (((locals.var_t5ng12_dn4 * locals.var_t5dg12) - (locals.var_t5ng12 * locals.var_t5dg12_dn4)) / (locals.var_t5dg12 * locals.var_t5dg12))), ((-(((locals.var_t5ng02_dn5 * locals.var_t5dg02) - (locals.var_t5ng02 * locals.var_t5dg02_dn5)) / (locals.var_t5dg02 * locals.var_t5dg02))) - (((locals.var_t5ng12_dn5 * locals.var_t5dg12) - (locals.var_t5ng12 * locals.var_t5dg12_dn5)) / (locals.var_t5dg12 * locals.var_t5dg12))), ((-(((locals.var_t5ng02_dn6 * locals.var_t5dg02) - (locals.var_t5ng02 * locals.var_t5dg02_dn6)) / (locals.var_t5dg02 * locals.var_t5dg02))) - (((locals.var_t5ng12_dn6 * locals.var_t5dg12) - (locals.var_t5ng12 * locals.var_t5dg12_dn6)) / (locals.var_t5dg12 * locals.var_t5dg12))), ((-(((locals.var_t5ng02_dn7 * locals.var_t5dg02) - (locals.var_t5ng02 * locals.var_t5dg02_dn7)) / (locals.var_t5dg02 * locals.var_t5dg02))) - (((locals.var_t5ng12_dn7 * locals.var_t5dg12) - (locals.var_t5ng12 * locals.var_t5dg12_dn7)) / (locals.var_t5dg12 * locals.var_t5dg12))), ((-(((locals.var_t5ng02_dn8 * locals.var_t5dg02) - (locals.var_t5ng02 * locals.var_t5dg02_dn8)) / (locals.var_t5dg02 * locals.var_t5dg02))) - (((locals.var_t5ng12_dn8 * locals.var_t5dg12) - (locals.var_t5ng12 * locals.var_t5dg12_dn8)) / (locals.var_t5dg12 * locals.var_t5dg12))), ((-(((locals.var_t5ng02_dn9 * locals.var_t5dg02) - (locals.var_t5ng02 * locals.var_t5dg02_dn9)) / (locals.var_t5dg02 * locals.var_t5dg02))) - (((locals.var_t5ng12_dn9 * locals.var_t5dg12) - (locals.var_t5ng12 * locals.var_t5dg12_dn9)) / (locals.var_t5dg12 * locals.var_t5dg12))), ((-(((locals.var_t5ng02_dn12 * locals.var_t5dg02) - (locals.var_t5ng02 * locals.var_t5dg02_dn12)) / (locals.var_t5dg02 * locals.var_t5dg02))) - (((locals.var_t5ng12_dn12 * locals.var_t5dg12) - (locals.var_t5ng12 * locals.var_t5dg12_dn12)) / (locals.var_t5dg12 * locals.var_t5dg12))), ((-(((locals.var_t5ng02_dn14 * locals.var_t5dg02) - (locals.var_t5ng02 * locals.var_t5dg02_dn14)) / (locals.var_t5dg02 * locals.var_t5dg02))) - (((locals.var_t5ng12_dn14 * locals.var_t5dg12) - (locals.var_t5ng12 * locals.var_t5dg12_dn14)) / (locals.var_t5dg12 * locals.var_t5dg12))), ((-(((locals.var_t5ng02_dn15 * locals.var_t5dg02) - (locals.var_t5ng02 * locals.var_t5dg02_dn15)) / (locals.var_t5dg02 * locals.var_t5dg02))) - (((locals.var_t5ng12_dn15 * locals.var_t5dg12) - (locals.var_t5ng12 * locals.var_t5dg12_dn15)) / (locals.var_t5dg12 * locals.var_t5dg12))), ((-(((locals.var_t5ng02_dn16 * locals.var_t5dg02) - (locals.var_t5ng02 * locals.var_t5dg02_dn16)) / (locals.var_t5dg02 * locals.var_t5dg02))) - (((locals.var_t5ng12_dn16 * locals.var_t5dg12) - (locals.var_t5ng12 * locals.var_t5dg12_dn16)) / (locals.var_t5dg12 * locals.var_t5dg12))), ((-(((locals.var_t5ng02_dn17 * locals.var_t5dg02) - (locals.var_t5ng02 * locals.var_t5dg02_dn17)) / (locals.var_t5dg02 * locals.var_t5dg02))) - (((locals.var_t5ng12_dn17 * locals.var_t5dg12) - (locals.var_t5ng12 * locals.var_t5dg12_dn17)) / (locals.var_t5dg12 * locals.var_t5dg12))), ((-(((locals.var_t5ng02_dn18 * locals.var_t5dg02) - (locals.var_t5ng02 * locals.var_t5dg02_dn18)) / (locals.var_t5dg02 * locals.var_t5dg02))) - (((locals.var_t5ng12_dn18 * locals.var_t5dg12) - (locals.var_t5ng12 * locals.var_t5dg12_dn18)) / (locals.var_t5dg12 * locals.var_t5dg12))), ((-(((locals.var_t5ng02_dn19 * locals.var_t5dg02) - (locals.var_t5ng02 * locals.var_t5dg02_dn19)) / (locals.var_t5dg02 * locals.var_t5dg02))) - (((locals.var_t5ng12_dn19 * locals.var_t5dg12) - (locals.var_t5ng12 * locals.var_t5dg12_dn19)) / (locals.var_t5dg12 * locals.var_t5dg12))), ((-(((locals.var_t5ng02_dn20 * locals.var_t5dg02) - (locals.var_t5ng02 * locals.var_t5dg02_dn20)) / (locals.var_t5dg02 * locals.var_t5dg02))) - (((locals.var_t5ng12_dn20 * locals.var_t5dg12) - (locals.var_t5ng12 * locals.var_t5dg12_dn20)) / (locals.var_t5dg12 * locals.var_t5dg12))), ((-(((locals.var_t5ng02_dn21 * locals.var_t5dg02) - (locals.var_t5ng02 * locals.var_t5dg02_dn21)) / (locals.var_t5dg02 * locals.var_t5dg02))) - (((locals.var_t5ng12_dn21 * locals.var_t5dg12) - (locals.var_t5ng12 * locals.var_t5dg12_dn21)) / (locals.var_t5dg12 * locals.var_t5dg12))), ((-(((locals.var_t5ng02_dn22 * locals.var_t5dg02) - (locals.var_t5ng02 * locals.var_t5dg02_dn22)) / (locals.var_t5dg02 * locals.var_t5dg02))) - (((locals.var_t5ng12_dn22 * locals.var_t5dg12) - (locals.var_t5ng12 * locals.var_t5dg12_dn22)) / (locals.var_t5dg12 * locals.var_t5dg12))),)
    } else {
        (locals.var_t52, locals.var_t52_dn0, locals.var_t52_dn1, locals.var_t52_dn2, locals.var_t52_dn3, locals.var_t52_dn4, locals.var_t52_dn5, locals.var_t52_dn6, locals.var_t52_dn7, locals.var_t52_dn8, locals.var_t52_dn9, locals.var_t52_dn12, locals.var_t52_dn14, locals.var_t52_dn15, locals.var_t52_dn16, locals.var_t52_dn17, locals.var_t52_dn18, locals.var_t52_dn19, locals.var_t52_dn20, locals.var_t52_dn21, locals.var_t52_dn22,)
    }
};
        locals.var_t52 = assign30020_e47376;
        locals.var_t52_dn0 = assign30020_e47376_d_n0;
        locals.var_t52_dn1 = assign30020_e47376_d_n1;
        locals.var_t52_dn2 = assign30020_e47376_d_n2;
        locals.var_t52_dn3 = assign30020_e47376_d_n3;
        locals.var_t52_dn4 = assign30020_e47376_d_n4;
        locals.var_t52_dn5 = assign30020_e47376_d_n5;
        locals.var_t52_dn6 = assign30020_e47376_d_n6;
        locals.var_t52_dn7 = assign30020_e47376_d_n7;
        locals.var_t52_dn8 = assign30020_e47376_d_n8;
        locals.var_t52_dn9 = assign30020_e47376_d_n9;
        locals.var_t52_dn12 = assign30020_e47376_d_n12;
        locals.var_t52_dn14 = assign30020_e47376_d_n14;
        locals.var_t52_dn15 = assign30020_e47376_d_n15;
        locals.var_t52_dn16 = assign30020_e47376_d_n16;
        locals.var_t52_dn17 = assign30020_e47376_d_n17;
        locals.var_t52_dn18 = assign30020_e47376_d_n18;
        locals.var_t52_dn19 = assign30020_e47376_d_n19;
        locals.var_t52_dn20 = assign30020_e47376_d_n20;
        locals.var_t52_dn21 = assign30020_e47376_d_n21;
        locals.var_t52_dn22 = assign30020_e47376_d_n22;

        let (assign30030_e47389, assign30030_e47389_d_n0, assign30030_e47389_d_n1, assign30030_e47389_d_n2, assign30030_e47389_d_n3, assign30030_e47389_d_n4, assign30030_e47389_d_n5, assign30030_e47389_d_n6, assign30030_e47389_d_n7, assign30030_e47389_d_n8, assign30030_e47389_d_n9, assign30030_e47389_d_n12, assign30030_e47389_d_n14, assign30030_e47389_d_n15, assign30030_e47389_d_n16, assign30030_e47389_d_n17, assign30030_e47389_d_n18, assign30030_e47389_d_n19, assign30030_e47389_d_n20, assign30030_e47389_d_n21, assign30030_e47389_d_n22,) = {
    if (((locals.var_guard504 == 0.0) && (locals.var_guard513 != 0.0)) && (locals.var_guard516 != 0.0)) {
        let assign30030_e47386: f64 = (locals.var_t42 / locals.var_t52);
        let assign30030_e47387: f64 = (locals.var_ef2 - assign30030_e47386);
        (assign30030_e47387, (locals.var_ef2_dn0 - (((locals.var_t42_dn0 * locals.var_t52) - (locals.var_t42 * locals.var_t52_dn0)) / (locals.var_t52 * locals.var_t52))), (locals.var_ef2_dn1 - (((locals.var_t42_dn1 * locals.var_t52) - (locals.var_t42 * locals.var_t52_dn1)) / (locals.var_t52 * locals.var_t52))), (locals.var_ef2_dn2 - (((locals.var_t42_dn2 * locals.var_t52) - (locals.var_t42 * locals.var_t52_dn2)) / (locals.var_t52 * locals.var_t52))), (locals.var_ef2_dn3 - (((locals.var_t42_dn3 * locals.var_t52) - (locals.var_t42 * locals.var_t52_dn3)) / (locals.var_t52 * locals.var_t52))), (locals.var_ef2_dn4 - (((locals.var_t42_dn4 * locals.var_t52) - (locals.var_t42 * locals.var_t52_dn4)) / (locals.var_t52 * locals.var_t52))), (locals.var_ef2_dn5 - (((locals.var_t42_dn5 * locals.var_t52) - (locals.var_t42 * locals.var_t52_dn5)) / (locals.var_t52 * locals.var_t52))), (locals.var_ef2_dn6 - (((locals.var_t42_dn6 * locals.var_t52) - (locals.var_t42 * locals.var_t52_dn6)) / (locals.var_t52 * locals.var_t52))), (locals.var_ef2_dn7 - (((locals.var_t42_dn7 * locals.var_t52) - (locals.var_t42 * locals.var_t52_dn7)) / (locals.var_t52 * locals.var_t52))), (locals.var_ef2_dn8 - (((locals.var_t42_dn8 * locals.var_t52) - (locals.var_t42 * locals.var_t52_dn8)) / (locals.var_t52 * locals.var_t52))), (locals.var_ef2_dn9 - (((locals.var_t42_dn9 * locals.var_t52) - (locals.var_t42 * locals.var_t52_dn9)) / (locals.var_t52 * locals.var_t52))), (locals.var_ef2_dn12 - (((locals.var_t42_dn12 * locals.var_t52) - (locals.var_t42 * locals.var_t52_dn12)) / (locals.var_t52 * locals.var_t52))), (locals.var_ef2_dn14 - (((locals.var_t42_dn14 * locals.var_t52) - (locals.var_t42 * locals.var_t52_dn14)) / (locals.var_t52 * locals.var_t52))), (locals.var_ef2_dn15 - (((locals.var_t42_dn15 * locals.var_t52) - (locals.var_t42 * locals.var_t52_dn15)) / (locals.var_t52 * locals.var_t52))), (locals.var_ef2_dn16 - (((locals.var_t42_dn16 * locals.var_t52) - (locals.var_t42 * locals.var_t52_dn16)) / (locals.var_t52 * locals.var_t52))), (locals.var_ef2_dn17 - (((locals.var_t42_dn17 * locals.var_t52) - (locals.var_t42 * locals.var_t52_dn17)) / (locals.var_t52 * locals.var_t52))), (locals.var_ef2_dn18 - (((locals.var_t42_dn18 * locals.var_t52) - (locals.var_t42 * locals.var_t52_dn18)) / (locals.var_t52 * locals.var_t52))), (locals.var_ef2_dn19 - (((locals.var_t42_dn19 * locals.var_t52) - (locals.var_t42 * locals.var_t52_dn19)) / (locals.var_t52 * locals.var_t52))), (locals.var_ef2_dn20 - (((locals.var_t42_dn20 * locals.var_t52) - (locals.var_t42 * locals.var_t52_dn20)) / (locals.var_t52 * locals.var_t52))), (locals.var_ef2_dn21 - (((locals.var_t42_dn21 * locals.var_t52) - (locals.var_t42 * locals.var_t52_dn21)) / (locals.var_t52 * locals.var_t52))), (locals.var_ef2_dn22 - (((locals.var_t42_dn22 * locals.var_t52) - (locals.var_t42 * locals.var_t52_dn22)) / (locals.var_t52 * locals.var_t52))),)
    } else {
        (locals.var_ef3, locals.var_ef3_dn0, locals.var_ef3_dn1, locals.var_ef3_dn2, locals.var_ef3_dn3, locals.var_ef3_dn4, locals.var_ef3_dn5, locals.var_ef3_dn6, locals.var_ef3_dn7, locals.var_ef3_dn8, locals.var_ef3_dn9, locals.var_ef3_dn12, locals.var_ef3_dn14, locals.var_ef3_dn15, locals.var_ef3_dn16, locals.var_ef3_dn17, locals.var_ef3_dn18, locals.var_ef3_dn19, locals.var_ef3_dn20, locals.var_ef3_dn21, locals.var_ef3_dn22,)
    }
};
        locals.var_ef3 = assign30030_e47389;
        locals.var_ef3_dn0 = assign30030_e47389_d_n0;
        locals.var_ef3_dn1 = assign30030_e47389_d_n1;
        locals.var_ef3_dn2 = assign30030_e47389_d_n2;
        locals.var_ef3_dn3 = assign30030_e47389_d_n3;
        locals.var_ef3_dn4 = assign30030_e47389_d_n4;
        locals.var_ef3_dn5 = assign30030_e47389_d_n5;
        locals.var_ef3_dn6 = assign30030_e47389_d_n6;
        locals.var_ef3_dn7 = assign30030_e47389_d_n7;
        locals.var_ef3_dn8 = assign30030_e47389_d_n8;
        locals.var_ef3_dn9 = assign30030_e47389_d_n9;
        locals.var_ef3_dn12 = assign30030_e47389_d_n12;
        locals.var_ef3_dn14 = assign30030_e47389_d_n14;
        locals.var_ef3_dn15 = assign30030_e47389_d_n15;
        locals.var_ef3_dn16 = assign30030_e47389_d_n16;
        locals.var_ef3_dn17 = assign30030_e47389_d_n17;
        locals.var_ef3_dn18 = assign30030_e47389_d_n18;
        locals.var_ef3_dn19 = assign30030_e47389_d_n19;
        locals.var_ef3_dn20 = assign30030_e47389_d_n20;
        locals.var_ef3_dn21 = assign30030_e47389_d_n21;
        locals.var_ef3_dn22 = assign30030_e47389_d_n22;

        let (assign30040_e47398, assign30040_e47398_d_n0, assign30040_e47398_d_n1, assign30040_e47398_d_n2, assign30040_e47398_d_n3, assign30040_e47398_d_n4, assign30040_e47398_d_n5, assign30040_e47398_d_n6, assign30040_e47398_d_n7, assign30040_e47398_d_n8, assign30040_e47398_d_n9, assign30040_e47398_d_n12, assign30040_e47398_d_n14, assign30040_e47398_d_n15, assign30040_e47398_d_n16, assign30040_e47398_d_n17, assign30040_e47398_d_n18, assign30040_e47398_d_n19, assign30040_e47398_d_n20, assign30040_e47398_d_n21, assign30040_e47398_d_n22,) = {
    if (((locals.var_guard504 == 0.0) && (locals.var_guard513 != 0.0)) && (locals.var_guard516 != 0.0)) {
        (locals.var_ef3, locals.var_ef3_dn0, locals.var_ef3_dn1, locals.var_ef3_dn2, locals.var_ef3_dn3, locals.var_ef3_dn4, locals.var_ef3_dn5, locals.var_ef3_dn6, locals.var_ef3_dn7, locals.var_ef3_dn8, locals.var_ef3_dn9, locals.var_ef3_dn12, locals.var_ef3_dn14, locals.var_ef3_dn15, locals.var_ef3_dn16, locals.var_ef3_dn17, locals.var_ef3_dn18, locals.var_ef3_dn19, locals.var_ef3_dn20, locals.var_ef3_dn21, locals.var_ef3_dn22,)
    } else {
        (locals.var_psis_fp4s, locals.var_psis_fp4s_dn0, locals.var_psis_fp4s_dn1, locals.var_psis_fp4s_dn2, locals.var_psis_fp4s_dn3, locals.var_psis_fp4s_dn4, locals.var_psis_fp4s_dn5, locals.var_psis_fp4s_dn6, locals.var_psis_fp4s_dn7, locals.var_psis_fp4s_dn8, locals.var_psis_fp4s_dn9, locals.var_psis_fp4s_dn12, locals.var_psis_fp4s_dn14, locals.var_psis_fp4s_dn15, locals.var_psis_fp4s_dn16, locals.var_psis_fp4s_dn17, locals.var_psis_fp4s_dn18, locals.var_psis_fp4s_dn19, locals.var_psis_fp4s_dn20, locals.var_psis_fp4s_dn21, locals.var_psis_fp4s_dn22,)
    }
};
        locals.var_psis_fp4s = assign30040_e47398;
        locals.var_psis_fp4s_dn0 = assign30040_e47398_d_n0;
        locals.var_psis_fp4s_dn1 = assign30040_e47398_d_n1;
        locals.var_psis_fp4s_dn2 = assign30040_e47398_d_n2;
        locals.var_psis_fp4s_dn3 = assign30040_e47398_d_n3;
        locals.var_psis_fp4s_dn4 = assign30040_e47398_d_n4;
        locals.var_psis_fp4s_dn5 = assign30040_e47398_d_n5;
        locals.var_psis_fp4s_dn6 = assign30040_e47398_d_n6;
        locals.var_psis_fp4s_dn7 = assign30040_e47398_d_n7;
        locals.var_psis_fp4s_dn8 = assign30040_e47398_d_n8;
        locals.var_psis_fp4s_dn9 = assign30040_e47398_d_n9;
        locals.var_psis_fp4s_dn12 = assign30040_e47398_d_n12;
        locals.var_psis_fp4s_dn14 = assign30040_e47398_d_n14;
        locals.var_psis_fp4s_dn15 = assign30040_e47398_d_n15;
        locals.var_psis_fp4s_dn16 = assign30040_e47398_d_n16;
        locals.var_psis_fp4s_dn17 = assign30040_e47398_d_n17;
        locals.var_psis_fp4s_dn18 = assign30040_e47398_d_n18;
        locals.var_psis_fp4s_dn19 = assign30040_e47398_d_n19;
        locals.var_psis_fp4s_dn20 = assign30040_e47398_d_n20;
        locals.var_psis_fp4s_dn21 = assign30040_e47398_d_n21;
        locals.var_psis_fp4s_dn22 = assign30040_e47398_d_n22;

        let (assign30050_e47408, assign30050_e47408_d_n0, assign30050_e47408_d_n1, assign30050_e47408_d_n2, assign30050_e47408_d_n3, assign30050_e47408_d_n4, assign30050_e47408_d_n5, assign30050_e47408_d_n6, assign30050_e47408_d_n7, assign30050_e47408_d_n8, assign30050_e47408_d_n9, assign30050_e47408_d_n12, assign30050_e47408_d_n14, assign30050_e47408_d_n15, assign30050_e47408_d_n16, assign30050_e47408_d_n17, assign30050_e47408_d_n18, assign30050_e47408_d_n19, assign30050_e47408_d_n20, assign30050_e47408_d_n21, assign30050_e47408_d_n22,) = {
    if (((locals.var_guard504 == 0.0) && (locals.var_guard513 != 0.0)) && (locals.var_guard516 == 0.0)) {
        (locals.var_ef1, locals.var_ef1_dn0, locals.var_ef1_dn1, locals.var_ef1_dn2, locals.var_ef1_dn3, locals.var_ef1_dn4, locals.var_ef1_dn5, locals.var_ef1_dn6, locals.var_ef1_dn7, locals.var_ef1_dn8, locals.var_ef1_dn9, locals.var_ef1_dn12, locals.var_ef1_dn14, locals.var_ef1_dn15, locals.var_ef1_dn16, locals.var_ef1_dn17, locals.var_ef1_dn18, locals.var_ef1_dn19, locals.var_ef1_dn20, locals.var_ef1_dn21, locals.var_ef1_dn22,)
    } else {
        (locals.var_psis_fp4s, locals.var_psis_fp4s_dn0, locals.var_psis_fp4s_dn1, locals.var_psis_fp4s_dn2, locals.var_psis_fp4s_dn3, locals.var_psis_fp4s_dn4, locals.var_psis_fp4s_dn5, locals.var_psis_fp4s_dn6, locals.var_psis_fp4s_dn7, locals.var_psis_fp4s_dn8, locals.var_psis_fp4s_dn9, locals.var_psis_fp4s_dn12, locals.var_psis_fp4s_dn14, locals.var_psis_fp4s_dn15, locals.var_psis_fp4s_dn16, locals.var_psis_fp4s_dn17, locals.var_psis_fp4s_dn18, locals.var_psis_fp4s_dn19, locals.var_psis_fp4s_dn20, locals.var_psis_fp4s_dn21, locals.var_psis_fp4s_dn22,)
    }
};
        locals.var_psis_fp4s = assign30050_e47408;
        locals.var_psis_fp4s_dn0 = assign30050_e47408_d_n0;
        locals.var_psis_fp4s_dn1 = assign30050_e47408_d_n1;
        locals.var_psis_fp4s_dn2 = assign30050_e47408_d_n2;
        locals.var_psis_fp4s_dn3 = assign30050_e47408_d_n3;
        locals.var_psis_fp4s_dn4 = assign30050_e47408_d_n4;
        locals.var_psis_fp4s_dn5 = assign30050_e47408_d_n5;
        locals.var_psis_fp4s_dn6 = assign30050_e47408_d_n6;
        locals.var_psis_fp4s_dn7 = assign30050_e47408_d_n7;
        locals.var_psis_fp4s_dn8 = assign30050_e47408_d_n8;
        locals.var_psis_fp4s_dn9 = assign30050_e47408_d_n9;
        locals.var_psis_fp4s_dn12 = assign30050_e47408_d_n12;
        locals.var_psis_fp4s_dn14 = assign30050_e47408_d_n14;
        locals.var_psis_fp4s_dn15 = assign30050_e47408_d_n15;
        locals.var_psis_fp4s_dn16 = assign30050_e47408_d_n16;
        locals.var_psis_fp4s_dn17 = assign30050_e47408_d_n17;
        locals.var_psis_fp4s_dn18 = assign30050_e47408_d_n18;
        locals.var_psis_fp4s_dn19 = assign30050_e47408_d_n19;
        locals.var_psis_fp4s_dn20 = assign30050_e47408_d_n20;
        locals.var_psis_fp4s_dn21 = assign30050_e47408_d_n21;
        locals.var_psis_fp4s_dn22 = assign30050_e47408_d_n22;

        let (assign30060_e47415, assign30060_e47415_d_n21, assign30060_e47415_d_n22,) = {
    if ((locals.var_guard504 == 0.0) && (locals.var_guard513 != 0.0)) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_vds_fp4s, locals.var_vds_fp4s_dn21, locals.var_vds_fp4s_dn22,)
    }
};
        locals.var_vds_fp4s = assign30060_e47415;
        locals.var_vds_fp4s_dn21 = assign30060_e47415_d_n21;
        locals.var_vds_fp4s_dn22 = assign30060_e47415_d_n22;

        let (assign30070_e47428, assign30070_e47428_d_n0, assign30070_e47428_d_n1, assign30070_e47428_d_n2, assign30070_e47428_d_n3, assign30070_e47428_d_n4, assign30070_e47428_d_n5, assign30070_e47428_d_n6, assign30070_e47428_d_n7, assign30070_e47428_d_n8, assign30070_e47428_d_n9, assign30070_e47428_d_n12, assign30070_e47428_d_n14, assign30070_e47428_d_n15, assign30070_e47428_d_n16, assign30070_e47428_d_n17, assign30070_e47428_d_n18, assign30070_e47428_d_n19, assign30070_e47428_d_n20, assign30070_e47428_d_n21, assign30070_e47428_d_n22,) = {
    if ((locals.var_guard504 == 0.0) && (locals.var_guard513 != 0.0)) {
        let assign30070_e47423: f64 = (locals.var_tdev / locals.var_tnom);
        let assign30070_e47425: f64 = (assign30070_e47423).powf(p.p20);
        let assign30070_e47426: f64 = (p.p202 * assign30070_e47425);
        (assign30070_e47426, 0.0, 0.0, 0.0, 0.0, (p.p202 * if 0.0 == 0.0 && ((p.p20) as f64).is_finite() && ((p.p20) as f64).fract() == 0.0 { if p.p20 == 0.0 { 0.0 } else { (p.p20 * ((assign30070_e47423).powf(p.p20 - 1.0) * (locals.var_tdev_dn4 / locals.var_tnom))) } } else { (assign30070_e47425 * (p.p20 * ((locals.var_tdev_dn4 / locals.var_tnom) / assign30070_e47423))) }), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_mulf_tdev, locals.var_mulf_tdev_dn0, locals.var_mulf_tdev_dn1, locals.var_mulf_tdev_dn2, locals.var_mulf_tdev_dn3, locals.var_mulf_tdev_dn4, locals.var_mulf_tdev_dn5, locals.var_mulf_tdev_dn6, locals.var_mulf_tdev_dn7, locals.var_mulf_tdev_dn8, locals.var_mulf_tdev_dn9, locals.var_mulf_tdev_dn12, locals.var_mulf_tdev_dn14, locals.var_mulf_tdev_dn15, locals.var_mulf_tdev_dn16, locals.var_mulf_tdev_dn17, locals.var_mulf_tdev_dn18, locals.var_mulf_tdev_dn19, locals.var_mulf_tdev_dn20, locals.var_mulf_tdev_dn21, locals.var_mulf_tdev_dn22,)
    }
};
        locals.var_mulf_tdev = assign30070_e47428;
        locals.var_mulf_tdev_dn0 = assign30070_e47428_d_n0;
        locals.var_mulf_tdev_dn1 = assign30070_e47428_d_n1;
        locals.var_mulf_tdev_dn2 = assign30070_e47428_d_n2;
        locals.var_mulf_tdev_dn3 = assign30070_e47428_d_n3;
        locals.var_mulf_tdev_dn4 = assign30070_e47428_d_n4;
        locals.var_mulf_tdev_dn5 = assign30070_e47428_d_n5;
        locals.var_mulf_tdev_dn6 = assign30070_e47428_d_n6;
        locals.var_mulf_tdev_dn7 = assign30070_e47428_d_n7;
        locals.var_mulf_tdev_dn8 = assign30070_e47428_d_n8;
        locals.var_mulf_tdev_dn9 = assign30070_e47428_d_n9;
        locals.var_mulf_tdev_dn12 = assign30070_e47428_d_n12;
        locals.var_mulf_tdev_dn14 = assign30070_e47428_d_n14;
        locals.var_mulf_tdev_dn15 = assign30070_e47428_d_n15;
        locals.var_mulf_tdev_dn16 = assign30070_e47428_d_n16;
        locals.var_mulf_tdev_dn17 = assign30070_e47428_d_n17;
        locals.var_mulf_tdev_dn18 = assign30070_e47428_d_n18;
        locals.var_mulf_tdev_dn19 = assign30070_e47428_d_n19;
        locals.var_mulf_tdev_dn20 = assign30070_e47428_d_n20;
        locals.var_mulf_tdev_dn21 = assign30070_e47428_d_n21;
        locals.var_mulf_tdev_dn22 = assign30070_e47428_d_n22;

        let (assign30080_e47441, assign30080_e47441_d_n0, assign30080_e47441_d_n1, assign30080_e47441_d_n2, assign30080_e47441_d_n3, assign30080_e47441_d_n4, assign30080_e47441_d_n5, assign30080_e47441_d_n6, assign30080_e47441_d_n7, assign30080_e47441_d_n8, assign30080_e47441_d_n9, assign30080_e47441_d_n12, assign30080_e47441_d_n14, assign30080_e47441_d_n15, assign30080_e47441_d_n16, assign30080_e47441_d_n17, assign30080_e47441_d_n18, assign30080_e47441_d_n19, assign30080_e47441_d_n20, assign30080_e47441_d_n21, assign30080_e47441_d_n22,) = {
    if ((locals.var_guard504 == 0.0) && (locals.var_guard513 != 0.0)) {
        let assign30080_e47436: f64 = (locals.var_tdev / locals.var_tnom);
        let assign30080_e47438: f64 = (assign30080_e47436).powf(p.p19);
        let assign30080_e47439: f64 = (p.p203 * assign30080_e47438);
        (assign30080_e47439, 0.0, 0.0, 0.0, 0.0, (p.p203 * if 0.0 == 0.0 && ((p.p19) as f64).is_finite() && ((p.p19) as f64).fract() == 0.0 { if p.p19 == 0.0 { 0.0 } else { (p.p19 * ((assign30080_e47436).powf(p.p19 - 1.0) * (locals.var_tdev_dn4 / locals.var_tnom))) } } else { (assign30080_e47438 * (p.p19 * ((locals.var_tdev_dn4 / locals.var_tnom) / assign30080_e47436))) }), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vsat_tdev, locals.var_vsat_tdev_dn0, locals.var_vsat_tdev_dn1, locals.var_vsat_tdev_dn2, locals.var_vsat_tdev_dn3, locals.var_vsat_tdev_dn4, locals.var_vsat_tdev_dn5, locals.var_vsat_tdev_dn6, locals.var_vsat_tdev_dn7, locals.var_vsat_tdev_dn8, locals.var_vsat_tdev_dn9, locals.var_vsat_tdev_dn12, locals.var_vsat_tdev_dn14, locals.var_vsat_tdev_dn15, locals.var_vsat_tdev_dn16, locals.var_vsat_tdev_dn17, locals.var_vsat_tdev_dn18, locals.var_vsat_tdev_dn19, locals.var_vsat_tdev_dn20, locals.var_vsat_tdev_dn21, locals.var_vsat_tdev_dn22,)
    }
};
        locals.var_vsat_tdev = assign30080_e47441;
        locals.var_vsat_tdev_dn0 = assign30080_e47441_d_n0;
        locals.var_vsat_tdev_dn1 = assign30080_e47441_d_n1;
        locals.var_vsat_tdev_dn2 = assign30080_e47441_d_n2;
        locals.var_vsat_tdev_dn3 = assign30080_e47441_d_n3;
        locals.var_vsat_tdev_dn4 = assign30080_e47441_d_n4;
        locals.var_vsat_tdev_dn5 = assign30080_e47441_d_n5;
        locals.var_vsat_tdev_dn6 = assign30080_e47441_d_n6;
        locals.var_vsat_tdev_dn7 = assign30080_e47441_d_n7;
        locals.var_vsat_tdev_dn8 = assign30080_e47441_d_n8;
        locals.var_vsat_tdev_dn9 = assign30080_e47441_d_n9;
        locals.var_vsat_tdev_dn12 = assign30080_e47441_d_n12;
        locals.var_vsat_tdev_dn14 = assign30080_e47441_d_n14;
        locals.var_vsat_tdev_dn15 = assign30080_e47441_d_n15;
        locals.var_vsat_tdev_dn16 = assign30080_e47441_d_n16;
        locals.var_vsat_tdev_dn17 = assign30080_e47441_d_n17;
        locals.var_vsat_tdev_dn18 = assign30080_e47441_d_n18;
        locals.var_vsat_tdev_dn19 = assign30080_e47441_d_n19;
        locals.var_vsat_tdev_dn20 = assign30080_e47441_d_n20;
        locals.var_vsat_tdev_dn21 = assign30080_e47441_d_n21;
        locals.var_vsat_tdev_dn22 = assign30080_e47441_d_n22;

        let (assign30090_e47455, assign30090_e47455_d_n0, assign30090_e47455_d_n1, assign30090_e47455_d_n2, assign30090_e47455_d_n3, assign30090_e47455_d_n4, assign30090_e47455_d_n5, assign30090_e47455_d_n6, assign30090_e47455_d_n7, assign30090_e47455_d_n8, assign30090_e47455_d_n9, assign30090_e47455_d_n12, assign30090_e47455_d_n14, assign30090_e47455_d_n15, assign30090_e47455_d_n16, assign30090_e47455_d_n17, assign30090_e47455_d_n18, assign30090_e47455_d_n19, assign30090_e47455_d_n20, assign30090_e47455_d_n21, assign30090_e47455_d_n22,) = {
    if ((locals.var_guard504 == 0.0) && (locals.var_guard513 != 0.0)) {
        let assign30090_e47448: f64 = (locals.var_cg_fp4s / p.p9);
        let assign30090_e47451: f64 = (locals.var_vg0_fp4s - locals.var_psis_fp4s);
        let assign30090_e47452: f64 = (assign30090_e47451).abs();
        let assign30090_e47453: f64 = (assign30090_e47448 * assign30090_e47452);
        (assign30090_e47453, (assign30090_e47448 * if assign30090_e47451 >= 0.0 { (locals.var_vg0_fp4s_dn0 - locals.var_psis_fp4s_dn0) } else { (-(locals.var_vg0_fp4s_dn0 - locals.var_psis_fp4s_dn0)) }), (assign30090_e47448 * if assign30090_e47451 >= 0.0 { (locals.var_vg0_fp4s_dn1 - locals.var_psis_fp4s_dn1) } else { (-(locals.var_vg0_fp4s_dn1 - locals.var_psis_fp4s_dn1)) }), (assign30090_e47448 * if assign30090_e47451 >= 0.0 { (locals.var_vg0_fp4s_dn2 - locals.var_psis_fp4s_dn2) } else { (-(locals.var_vg0_fp4s_dn2 - locals.var_psis_fp4s_dn2)) }), (assign30090_e47448 * if assign30090_e47451 >= 0.0 { (locals.var_vg0_fp4s_dn3 - locals.var_psis_fp4s_dn3) } else { (-(locals.var_vg0_fp4s_dn3 - locals.var_psis_fp4s_dn3)) }), (assign30090_e47448 * if assign30090_e47451 >= 0.0 { (locals.var_vg0_fp4s_dn4 - locals.var_psis_fp4s_dn4) } else { (-(locals.var_vg0_fp4s_dn4 - locals.var_psis_fp4s_dn4)) }), (assign30090_e47448 * if assign30090_e47451 >= 0.0 { (locals.var_vg0_fp4s_dn5 - locals.var_psis_fp4s_dn5) } else { (-(locals.var_vg0_fp4s_dn5 - locals.var_psis_fp4s_dn5)) }), (assign30090_e47448 * if assign30090_e47451 >= 0.0 { (locals.var_vg0_fp4s_dn6 - locals.var_psis_fp4s_dn6) } else { (-(locals.var_vg0_fp4s_dn6 - locals.var_psis_fp4s_dn6)) }), (assign30090_e47448 * if assign30090_e47451 >= 0.0 { (locals.var_vg0_fp4s_dn7 - locals.var_psis_fp4s_dn7) } else { (-(locals.var_vg0_fp4s_dn7 - locals.var_psis_fp4s_dn7)) }), (assign30090_e47448 * if assign30090_e47451 >= 0.0 { (locals.var_vg0_fp4s_dn8 - locals.var_psis_fp4s_dn8) } else { (-(locals.var_vg0_fp4s_dn8 - locals.var_psis_fp4s_dn8)) }), (assign30090_e47448 * if assign30090_e47451 >= 0.0 { (locals.var_vg0_fp4s_dn9 - locals.var_psis_fp4s_dn9) } else { (-(locals.var_vg0_fp4s_dn9 - locals.var_psis_fp4s_dn9)) }), (assign30090_e47448 * if assign30090_e47451 >= 0.0 { (locals.var_vg0_fp4s_dn12 - locals.var_psis_fp4s_dn12) } else { (-(locals.var_vg0_fp4s_dn12 - locals.var_psis_fp4s_dn12)) }), (assign30090_e47448 * if assign30090_e47451 >= 0.0 { (locals.var_vg0_fp4s_dn14 - locals.var_psis_fp4s_dn14) } else { (-(locals.var_vg0_fp4s_dn14 - locals.var_psis_fp4s_dn14)) }), (assign30090_e47448 * if assign30090_e47451 >= 0.0 { (locals.var_vg0_fp4s_dn15 - locals.var_psis_fp4s_dn15) } else { (-(locals.var_vg0_fp4s_dn15 - locals.var_psis_fp4s_dn15)) }), (assign30090_e47448 * if assign30090_e47451 >= 0.0 { (locals.var_vg0_fp4s_dn16 - locals.var_psis_fp4s_dn16) } else { (-(locals.var_vg0_fp4s_dn16 - locals.var_psis_fp4s_dn16)) }), (assign30090_e47448 * if assign30090_e47451 >= 0.0 { (locals.var_vg0_fp4s_dn17 - locals.var_psis_fp4s_dn17) } else { (-(locals.var_vg0_fp4s_dn17 - locals.var_psis_fp4s_dn17)) }), (assign30090_e47448 * if assign30090_e47451 >= 0.0 { (locals.var_vg0_fp4s_dn18 - locals.var_psis_fp4s_dn18) } else { (-(locals.var_vg0_fp4s_dn18 - locals.var_psis_fp4s_dn18)) }), (assign30090_e47448 * if assign30090_e47451 >= 0.0 { (locals.var_vg0_fp4s_dn19 - locals.var_psis_fp4s_dn19) } else { (-(locals.var_vg0_fp4s_dn19 - locals.var_psis_fp4s_dn19)) }), (assign30090_e47448 * if assign30090_e47451 >= 0.0 { (locals.var_vg0_fp4s_dn20 - locals.var_psis_fp4s_dn20) } else { (-(locals.var_vg0_fp4s_dn20 - locals.var_psis_fp4s_dn20)) }), (assign30090_e47448 * if assign30090_e47451 >= 0.0 { (locals.var_vg0_fp4s_dn21 - locals.var_psis_fp4s_dn21) } else { (-(locals.var_vg0_fp4s_dn21 - locals.var_psis_fp4s_dn21)) }), (assign30090_e47448 * if assign30090_e47451 >= 0.0 { (locals.var_vg0_fp4s_dn22 - locals.var_psis_fp4s_dn22) } else { (-(locals.var_vg0_fp4s_dn22 - locals.var_psis_fp4s_dn22)) }),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn1, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn12, locals.var_t0_dn14, locals.var_t0_dn15, locals.var_t0_dn16, locals.var_t0_dn17, locals.var_t0_dn18, locals.var_t0_dn19, locals.var_t0_dn20, locals.var_t0_dn21, locals.var_t0_dn22,)
    }
};
        locals.var_t0 = assign30090_e47455;
        locals.var_t0_dn0 = assign30090_e47455_d_n0;
        locals.var_t0_dn1 = assign30090_e47455_d_n1;
        locals.var_t0_dn2 = assign30090_e47455_d_n2;
        locals.var_t0_dn3 = assign30090_e47455_d_n3;
        locals.var_t0_dn4 = assign30090_e47455_d_n4;
        locals.var_t0_dn5 = assign30090_e47455_d_n5;
        locals.var_t0_dn6 = assign30090_e47455_d_n6;
        locals.var_t0_dn7 = assign30090_e47455_d_n7;
        locals.var_t0_dn8 = assign30090_e47455_d_n8;
        locals.var_t0_dn9 = assign30090_e47455_d_n9;
        locals.var_t0_dn12 = assign30090_e47455_d_n12;
        locals.var_t0_dn14 = assign30090_e47455_d_n14;
        locals.var_t0_dn15 = assign30090_e47455_d_n15;
        locals.var_t0_dn16 = assign30090_e47455_d_n16;
        locals.var_t0_dn17 = assign30090_e47455_d_n17;
        locals.var_t0_dn18 = assign30090_e47455_d_n18;
        locals.var_t0_dn19 = assign30090_e47455_d_n19;
        locals.var_t0_dn20 = assign30090_e47455_d_n20;
        locals.var_t0_dn21 = assign30090_e47455_d_n21;
        locals.var_t0_dn22 = assign30090_e47455_d_n22;

        let (assign30100_e47469, assign30100_e47469_d_n0, assign30100_e47469_d_n1, assign30100_e47469_d_n2, assign30100_e47469_d_n3, assign30100_e47469_d_n4, assign30100_e47469_d_n5, assign30100_e47469_d_n6, assign30100_e47469_d_n7, assign30100_e47469_d_n8, assign30100_e47469_d_n9, assign30100_e47469_d_n12, assign30100_e47469_d_n14, assign30100_e47469_d_n15, assign30100_e47469_d_n16, assign30100_e47469_d_n17, assign30100_e47469_d_n18, assign30100_e47469_d_n19, assign30100_e47469_d_n20, assign30100_e47469_d_n21, assign30100_e47469_d_n22,) = {
    if ((locals.var_guard504 == 0.0) && (locals.var_guard513 != 0.0)) {
        let assign30100_e47462: f64 = (locals.var_cepi / p.p9);
        let assign30100_e47465: f64 = (locals.var_vbs - locals.var_psis_fp4s);
        let assign30100_e47466: f64 = (assign30100_e47465).abs();
        let assign30100_e47467: f64 = (assign30100_e47462 * assign30100_e47466);
        (assign30100_e47467, (assign30100_e47462 * if assign30100_e47465 >= 0.0 { (-locals.var_psis_fp4s_dn0) } else { (-(-locals.var_psis_fp4s_dn0)) }), (assign30100_e47462 * if assign30100_e47465 >= 0.0 { (-locals.var_psis_fp4s_dn1) } else { (-(-locals.var_psis_fp4s_dn1)) }), (assign30100_e47462 * if assign30100_e47465 >= 0.0 { (-locals.var_psis_fp4s_dn2) } else { (-(-locals.var_psis_fp4s_dn2)) }), (assign30100_e47462 * if assign30100_e47465 >= 0.0 { (locals.var_vbs_dn3 - locals.var_psis_fp4s_dn3) } else { (-(locals.var_vbs_dn3 - locals.var_psis_fp4s_dn3)) }), (assign30100_e47462 * if assign30100_e47465 >= 0.0 { (-locals.var_psis_fp4s_dn4) } else { (-(-locals.var_psis_fp4s_dn4)) }), (assign30100_e47462 * if assign30100_e47465 >= 0.0 { (-locals.var_psis_fp4s_dn5) } else { (-(-locals.var_psis_fp4s_dn5)) }), (assign30100_e47462 * if assign30100_e47465 >= 0.0 { (-locals.var_psis_fp4s_dn6) } else { (-(-locals.var_psis_fp4s_dn6)) }), (assign30100_e47462 * if assign30100_e47465 >= 0.0 { (locals.var_vbs_dn7 - locals.var_psis_fp4s_dn7) } else { (-(locals.var_vbs_dn7 - locals.var_psis_fp4s_dn7)) }), (assign30100_e47462 * if assign30100_e47465 >= 0.0 { (locals.var_vbs_dn8 - locals.var_psis_fp4s_dn8) } else { (-(locals.var_vbs_dn8 - locals.var_psis_fp4s_dn8)) }), (assign30100_e47462 * if assign30100_e47465 >= 0.0 { (-locals.var_psis_fp4s_dn9) } else { (-(-locals.var_psis_fp4s_dn9)) }), (assign30100_e47462 * if assign30100_e47465 >= 0.0 { (-locals.var_psis_fp4s_dn12) } else { (-(-locals.var_psis_fp4s_dn12)) }), (assign30100_e47462 * if assign30100_e47465 >= 0.0 { (-locals.var_psis_fp4s_dn14) } else { (-(-locals.var_psis_fp4s_dn14)) }), (assign30100_e47462 * if assign30100_e47465 >= 0.0 { (-locals.var_psis_fp4s_dn15) } else { (-(-locals.var_psis_fp4s_dn15)) }), (assign30100_e47462 * if assign30100_e47465 >= 0.0 { (-locals.var_psis_fp4s_dn16) } else { (-(-locals.var_psis_fp4s_dn16)) }), (assign30100_e47462 * if assign30100_e47465 >= 0.0 { (-locals.var_psis_fp4s_dn17) } else { (-(-locals.var_psis_fp4s_dn17)) }), (assign30100_e47462 * if assign30100_e47465 >= 0.0 { (-locals.var_psis_fp4s_dn18) } else { (-(-locals.var_psis_fp4s_dn18)) }), (assign30100_e47462 * if assign30100_e47465 >= 0.0 { (-locals.var_psis_fp4s_dn19) } else { (-(-locals.var_psis_fp4s_dn19)) }), (assign30100_e47462 * if assign30100_e47465 >= 0.0 { (-locals.var_psis_fp4s_dn20) } else { (-(-locals.var_psis_fp4s_dn20)) }), (assign30100_e47462 * if assign30100_e47465 >= 0.0 { (-locals.var_psis_fp4s_dn21) } else { (-(-locals.var_psis_fp4s_dn21)) }), (assign30100_e47462 * if assign30100_e47465 >= 0.0 { (-locals.var_psis_fp4s_dn22) } else { (-(-locals.var_psis_fp4s_dn22)) }),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn1, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn12, locals.var_t1_dn14, locals.var_t1_dn15, locals.var_t1_dn16, locals.var_t1_dn17, locals.var_t1_dn18, locals.var_t1_dn19, locals.var_t1_dn20, locals.var_t1_dn21, locals.var_t1_dn22,)
    }
};
        locals.var_t1 = assign30100_e47469;
        locals.var_t1_dn0 = assign30100_e47469_d_n0;
        locals.var_t1_dn1 = assign30100_e47469_d_n1;
        locals.var_t1_dn2 = assign30100_e47469_d_n2;
        locals.var_t1_dn3 = assign30100_e47469_d_n3;
        locals.var_t1_dn4 = assign30100_e47469_d_n4;
        locals.var_t1_dn5 = assign30100_e47469_d_n5;
        locals.var_t1_dn6 = assign30100_e47469_d_n6;
        locals.var_t1_dn7 = assign30100_e47469_d_n7;
        locals.var_t1_dn8 = assign30100_e47469_d_n8;
        locals.var_t1_dn9 = assign30100_e47469_d_n9;
        locals.var_t1_dn12 = assign30100_e47469_d_n12;
        locals.var_t1_dn14 = assign30100_e47469_d_n14;
        locals.var_t1_dn15 = assign30100_e47469_d_n15;
        locals.var_t1_dn16 = assign30100_e47469_d_n16;
        locals.var_t1_dn17 = assign30100_e47469_d_n17;
        locals.var_t1_dn18 = assign30100_e47469_d_n18;
        locals.var_t1_dn19 = assign30100_e47469_d_n19;
        locals.var_t1_dn20 = assign30100_e47469_d_n20;
        locals.var_t1_dn21 = assign30100_e47469_d_n21;
        locals.var_t1_dn22 = assign30100_e47469_d_n22;

        let (assign30110_e47492, assign30110_e47492_d_n0, assign30110_e47492_d_n1, assign30110_e47492_d_n2, assign30110_e47492_d_n3, assign30110_e47492_d_n4, assign30110_e47492_d_n5, assign30110_e47492_d_n6, assign30110_e47492_d_n7, assign30110_e47492_d_n8, assign30110_e47492_d_n9, assign30110_e47492_d_n12, assign30110_e47492_d_n14, assign30110_e47492_d_n15, assign30110_e47492_d_n16, assign30110_e47492_d_n17, assign30110_e47492_d_n18, assign30110_e47492_d_n19, assign30110_e47492_d_n20, assign30110_e47492_d_n21, assign30110_e47492_d_n22,) = {
    if ((locals.var_guard504 == 0.0) && (locals.var_guard513 != 0.0)) {
        let assign30110_e47478: f64 = (p.p14 * locals.var_t0);
        let assign30110_e47479: f64 = (1.0 + assign30110_e47478);
        let assign30110_e47483: f64 = (locals.var_t0 * locals.var_t0);
        let assign30110_e47484: f64 = (p.p15 * assign30110_e47483);
        let assign30110_e47485: f64 = (assign30110_e47479 + assign30110_e47484);
        let assign30110_e47488: f64 = (p.p16 * locals.var_t1);
        let assign30110_e47489: f64 = (assign30110_e47485 + assign30110_e47488);
        let assign30110_e47490: f64 = (locals.var_mulf_tdev / assign30110_e47489);
        (assign30110_e47490, (((locals.var_mulf_tdev_dn0 * assign30110_e47489) - (locals.var_mulf_tdev * (((p.p14 * locals.var_t0_dn0) + (p.p15 * ((locals.var_t0_dn0 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn0)))) + (p.p16 * locals.var_t1_dn0)))) / (assign30110_e47489 * assign30110_e47489)), (((locals.var_mulf_tdev_dn1 * assign30110_e47489) - (locals.var_mulf_tdev * (((p.p14 * locals.var_t0_dn1) + (p.p15 * ((locals.var_t0_dn1 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn1)))) + (p.p16 * locals.var_t1_dn1)))) / (assign30110_e47489 * assign30110_e47489)), (((locals.var_mulf_tdev_dn2 * assign30110_e47489) - (locals.var_mulf_tdev * (((p.p14 * locals.var_t0_dn2) + (p.p15 * ((locals.var_t0_dn2 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn2)))) + (p.p16 * locals.var_t1_dn2)))) / (assign30110_e47489 * assign30110_e47489)), (((locals.var_mulf_tdev_dn3 * assign30110_e47489) - (locals.var_mulf_tdev * (((p.p14 * locals.var_t0_dn3) + (p.p15 * ((locals.var_t0_dn3 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn3)))) + (p.p16 * locals.var_t1_dn3)))) / (assign30110_e47489 * assign30110_e47489)), (((locals.var_mulf_tdev_dn4 * assign30110_e47489) - (locals.var_mulf_tdev * (((p.p14 * locals.var_t0_dn4) + (p.p15 * ((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)))) + (p.p16 * locals.var_t1_dn4)))) / (assign30110_e47489 * assign30110_e47489)), (((locals.var_mulf_tdev_dn5 * assign30110_e47489) - (locals.var_mulf_tdev * (((p.p14 * locals.var_t0_dn5) + (p.p15 * ((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)))) + (p.p16 * locals.var_t1_dn5)))) / (assign30110_e47489 * assign30110_e47489)), (((locals.var_mulf_tdev_dn6 * assign30110_e47489) - (locals.var_mulf_tdev * (((p.p14 * locals.var_t0_dn6) + (p.p15 * ((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)))) + (p.p16 * locals.var_t1_dn6)))) / (assign30110_e47489 * assign30110_e47489)), (((locals.var_mulf_tdev_dn7 * assign30110_e47489) - (locals.var_mulf_tdev * (((p.p14 * locals.var_t0_dn7) + (p.p15 * ((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)))) + (p.p16 * locals.var_t1_dn7)))) / (assign30110_e47489 * assign30110_e47489)), (((locals.var_mulf_tdev_dn8 * assign30110_e47489) - (locals.var_mulf_tdev * (((p.p14 * locals.var_t0_dn8) + (p.p15 * ((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)))) + (p.p16 * locals.var_t1_dn8)))) / (assign30110_e47489 * assign30110_e47489)), (((locals.var_mulf_tdev_dn9 * assign30110_e47489) - (locals.var_mulf_tdev * (((p.p14 * locals.var_t0_dn9) + (p.p15 * ((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)))) + (p.p16 * locals.var_t1_dn9)))) / (assign30110_e47489 * assign30110_e47489)), (((locals.var_mulf_tdev_dn12 * assign30110_e47489) - (locals.var_mulf_tdev * (((p.p14 * locals.var_t0_dn12) + (p.p15 * ((locals.var_t0_dn12 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn12)))) + (p.p16 * locals.var_t1_dn12)))) / (assign30110_e47489 * assign30110_e47489)), (((locals.var_mulf_tdev_dn14 * assign30110_e47489) - (locals.var_mulf_tdev * (((p.p14 * locals.var_t0_dn14) + (p.p15 * ((locals.var_t0_dn14 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn14)))) + (p.p16 * locals.var_t1_dn14)))) / (assign30110_e47489 * assign30110_e47489)), (((locals.var_mulf_tdev_dn15 * assign30110_e47489) - (locals.var_mulf_tdev * (((p.p14 * locals.var_t0_dn15) + (p.p15 * ((locals.var_t0_dn15 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn15)))) + (p.p16 * locals.var_t1_dn15)))) / (assign30110_e47489 * assign30110_e47489)), (((locals.var_mulf_tdev_dn16 * assign30110_e47489) - (locals.var_mulf_tdev * (((p.p14 * locals.var_t0_dn16) + (p.p15 * ((locals.var_t0_dn16 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn16)))) + (p.p16 * locals.var_t1_dn16)))) / (assign30110_e47489 * assign30110_e47489)), (((locals.var_mulf_tdev_dn17 * assign30110_e47489) - (locals.var_mulf_tdev * (((p.p14 * locals.var_t0_dn17) + (p.p15 * ((locals.var_t0_dn17 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn17)))) + (p.p16 * locals.var_t1_dn17)))) / (assign30110_e47489 * assign30110_e47489)), (((locals.var_mulf_tdev_dn18 * assign30110_e47489) - (locals.var_mulf_tdev * (((p.p14 * locals.var_t0_dn18) + (p.p15 * ((locals.var_t0_dn18 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn18)))) + (p.p16 * locals.var_t1_dn18)))) / (assign30110_e47489 * assign30110_e47489)), (((locals.var_mulf_tdev_dn19 * assign30110_e47489) - (locals.var_mulf_tdev * (((p.p14 * locals.var_t0_dn19) + (p.p15 * ((locals.var_t0_dn19 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn19)))) + (p.p16 * locals.var_t1_dn19)))) / (assign30110_e47489 * assign30110_e47489)), (((locals.var_mulf_tdev_dn20 * assign30110_e47489) - (locals.var_mulf_tdev * (((p.p14 * locals.var_t0_dn20) + (p.p15 * ((locals.var_t0_dn20 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn20)))) + (p.p16 * locals.var_t1_dn20)))) / (assign30110_e47489 * assign30110_e47489)), (((locals.var_mulf_tdev_dn21 * assign30110_e47489) - (locals.var_mulf_tdev * (((p.p14 * locals.var_t0_dn21) + (p.p15 * ((locals.var_t0_dn21 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn21)))) + (p.p16 * locals.var_t1_dn21)))) / (assign30110_e47489 * assign30110_e47489)), (((locals.var_mulf_tdev_dn22 * assign30110_e47489) - (locals.var_mulf_tdev * (((p.p14 * locals.var_t0_dn22) + (p.p15 * ((locals.var_t0_dn22 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn22)))) + (p.p16 * locals.var_t1_dn22)))) / (assign30110_e47489 * assign30110_e47489)),)
    } else {
        (locals.var_mu_eff, locals.var_mu_eff_dn0, locals.var_mu_eff_dn1, locals.var_mu_eff_dn2, locals.var_mu_eff_dn3, locals.var_mu_eff_dn4, locals.var_mu_eff_dn5, locals.var_mu_eff_dn6, locals.var_mu_eff_dn7, locals.var_mu_eff_dn8, locals.var_mu_eff_dn9, locals.var_mu_eff_dn12, locals.var_mu_eff_dn14, locals.var_mu_eff_dn15, locals.var_mu_eff_dn16, locals.var_mu_eff_dn17, locals.var_mu_eff_dn18, locals.var_mu_eff_dn19, locals.var_mu_eff_dn20, locals.var_mu_eff_dn21, locals.var_mu_eff_dn22,)
    }
};
        locals.var_mu_eff = assign30110_e47492;
        locals.var_mu_eff_dn0 = assign30110_e47492_d_n0;
        locals.var_mu_eff_dn1 = assign30110_e47492_d_n1;
        locals.var_mu_eff_dn2 = assign30110_e47492_d_n2;
        locals.var_mu_eff_dn3 = assign30110_e47492_d_n3;
        locals.var_mu_eff_dn4 = assign30110_e47492_d_n4;
        locals.var_mu_eff_dn5 = assign30110_e47492_d_n5;
        locals.var_mu_eff_dn6 = assign30110_e47492_d_n6;
        locals.var_mu_eff_dn7 = assign30110_e47492_d_n7;
        locals.var_mu_eff_dn8 = assign30110_e47492_d_n8;
        locals.var_mu_eff_dn9 = assign30110_e47492_d_n9;
        locals.var_mu_eff_dn12 = assign30110_e47492_d_n12;
        locals.var_mu_eff_dn14 = assign30110_e47492_d_n14;
        locals.var_mu_eff_dn15 = assign30110_e47492_d_n15;
        locals.var_mu_eff_dn16 = assign30110_e47492_d_n16;
        locals.var_mu_eff_dn17 = assign30110_e47492_d_n17;
        locals.var_mu_eff_dn18 = assign30110_e47492_d_n18;
        locals.var_mu_eff_dn19 = assign30110_e47492_d_n19;
        locals.var_mu_eff_dn20 = assign30110_e47492_d_n20;
        locals.var_mu_eff_dn21 = assign30110_e47492_d_n21;
        locals.var_mu_eff_dn22 = assign30110_e47492_d_n22;

        let (assign30120_e47503, assign30120_e47503_d_n0, assign30120_e47503_d_n1, assign30120_e47503_d_n2, assign30120_e47503_d_n3, assign30120_e47503_d_n4, assign30120_e47503_d_n5, assign30120_e47503_d_n6, assign30120_e47503_d_n7, assign30120_e47503_d_n8, assign30120_e47503_d_n9, assign30120_e47503_d_n12, assign30120_e47503_d_n14, assign30120_e47503_d_n15, assign30120_e47503_d_n16, assign30120_e47503_d_n17, assign30120_e47503_d_n18, assign30120_e47503_d_n19, assign30120_e47503_d_n20, assign30120_e47503_d_n21, assign30120_e47503_d_n22,) = {
    if ((locals.var_guard504 == 0.0) && (locals.var_guard513 != 0.0)) {
        let assign30120_e47499: f64 = (2.0 * locals.var_vsat_tdev);
        let assign30120_e47501: f64 = (assign30120_e47499 / locals.var_mu_eff);
        (assign30120_e47501, ((((2.0 * locals.var_vsat_tdev_dn0) * locals.var_mu_eff) - (assign30120_e47499 * locals.var_mu_eff_dn0)) / (locals.var_mu_eff * locals.var_mu_eff)), ((((2.0 * locals.var_vsat_tdev_dn1) * locals.var_mu_eff) - (assign30120_e47499 * locals.var_mu_eff_dn1)) / (locals.var_mu_eff * locals.var_mu_eff)), ((((2.0 * locals.var_vsat_tdev_dn2) * locals.var_mu_eff) - (assign30120_e47499 * locals.var_mu_eff_dn2)) / (locals.var_mu_eff * locals.var_mu_eff)), ((((2.0 * locals.var_vsat_tdev_dn3) * locals.var_mu_eff) - (assign30120_e47499 * locals.var_mu_eff_dn3)) / (locals.var_mu_eff * locals.var_mu_eff)), ((((2.0 * locals.var_vsat_tdev_dn4) * locals.var_mu_eff) - (assign30120_e47499 * locals.var_mu_eff_dn4)) / (locals.var_mu_eff * locals.var_mu_eff)), ((((2.0 * locals.var_vsat_tdev_dn5) * locals.var_mu_eff) - (assign30120_e47499 * locals.var_mu_eff_dn5)) / (locals.var_mu_eff * locals.var_mu_eff)), ((((2.0 * locals.var_vsat_tdev_dn6) * locals.var_mu_eff) - (assign30120_e47499 * locals.var_mu_eff_dn6)) / (locals.var_mu_eff * locals.var_mu_eff)), ((((2.0 * locals.var_vsat_tdev_dn7) * locals.var_mu_eff) - (assign30120_e47499 * locals.var_mu_eff_dn7)) / (locals.var_mu_eff * locals.var_mu_eff)), ((((2.0 * locals.var_vsat_tdev_dn8) * locals.var_mu_eff) - (assign30120_e47499 * locals.var_mu_eff_dn8)) / (locals.var_mu_eff * locals.var_mu_eff)), ((((2.0 * locals.var_vsat_tdev_dn9) * locals.var_mu_eff) - (assign30120_e47499 * locals.var_mu_eff_dn9)) / (locals.var_mu_eff * locals.var_mu_eff)), ((((2.0 * locals.var_vsat_tdev_dn12) * locals.var_mu_eff) - (assign30120_e47499 * locals.var_mu_eff_dn12)) / (locals.var_mu_eff * locals.var_mu_eff)), ((((2.0 * locals.var_vsat_tdev_dn14) * locals.var_mu_eff) - (assign30120_e47499 * locals.var_mu_eff_dn14)) / (locals.var_mu_eff * locals.var_mu_eff)), ((((2.0 * locals.var_vsat_tdev_dn15) * locals.var_mu_eff) - (assign30120_e47499 * locals.var_mu_eff_dn15)) / (locals.var_mu_eff * locals.var_mu_eff)), ((((2.0 * locals.var_vsat_tdev_dn16) * locals.var_mu_eff) - (assign30120_e47499 * locals.var_mu_eff_dn16)) / (locals.var_mu_eff * locals.var_mu_eff)), ((((2.0 * locals.var_vsat_tdev_dn17) * locals.var_mu_eff) - (assign30120_e47499 * locals.var_mu_eff_dn17)) / (locals.var_mu_eff * locals.var_mu_eff)), ((((2.0 * locals.var_vsat_tdev_dn18) * locals.var_mu_eff) - (assign30120_e47499 * locals.var_mu_eff_dn18)) / (locals.var_mu_eff * locals.var_mu_eff)), ((((2.0 * locals.var_vsat_tdev_dn19) * locals.var_mu_eff) - (assign30120_e47499 * locals.var_mu_eff_dn19)) / (locals.var_mu_eff * locals.var_mu_eff)), ((((2.0 * locals.var_vsat_tdev_dn20) * locals.var_mu_eff) - (assign30120_e47499 * locals.var_mu_eff_dn20)) / (locals.var_mu_eff * locals.var_mu_eff)), ((((2.0 * locals.var_vsat_tdev_dn21) * locals.var_mu_eff) - (assign30120_e47499 * locals.var_mu_eff_dn21)) / (locals.var_mu_eff * locals.var_mu_eff)), ((((2.0 * locals.var_vsat_tdev_dn22) * locals.var_mu_eff) - (assign30120_e47499 * locals.var_mu_eff_dn22)) / (locals.var_mu_eff * locals.var_mu_eff)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn1, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn12, locals.var_t0_dn14, locals.var_t0_dn15, locals.var_t0_dn16, locals.var_t0_dn17, locals.var_t0_dn18, locals.var_t0_dn19, locals.var_t0_dn20, locals.var_t0_dn21, locals.var_t0_dn22,)
    }
};
        locals.var_t0 = assign30120_e47503;
        locals.var_t0_dn0 = assign30120_e47503_d_n0;
        locals.var_t0_dn1 = assign30120_e47503_d_n1;
        locals.var_t0_dn2 = assign30120_e47503_d_n2;
        locals.var_t0_dn3 = assign30120_e47503_d_n3;
        locals.var_t0_dn4 = assign30120_e47503_d_n4;
        locals.var_t0_dn5 = assign30120_e47503_d_n5;
        locals.var_t0_dn6 = assign30120_e47503_d_n6;
        locals.var_t0_dn7 = assign30120_e47503_d_n7;
        locals.var_t0_dn8 = assign30120_e47503_d_n8;
        locals.var_t0_dn9 = assign30120_e47503_d_n9;
        locals.var_t0_dn12 = assign30120_e47503_d_n12;
        locals.var_t0_dn14 = assign30120_e47503_d_n14;
        locals.var_t0_dn15 = assign30120_e47503_d_n15;
        locals.var_t0_dn16 = assign30120_e47503_d_n16;
        locals.var_t0_dn17 = assign30120_e47503_d_n17;
        locals.var_t0_dn18 = assign30120_e47503_d_n18;
        locals.var_t0_dn19 = assign30120_e47503_d_n19;
        locals.var_t0_dn20 = assign30120_e47503_d_n20;
        locals.var_t0_dn21 = assign30120_e47503_d_n21;
        locals.var_t0_dn22 = assign30120_e47503_d_n22;

        let (assign30130_e47525, assign30130_e47525_d_n0, assign30130_e47525_d_n1, assign30130_e47525_d_n2, assign30130_e47525_d_n3, assign30130_e47525_d_n4, assign30130_e47525_d_n5, assign30130_e47525_d_n6, assign30130_e47525_d_n7, assign30130_e47525_d_n8, assign30130_e47525_d_n9, assign30130_e47525_d_n12, assign30130_e47525_d_n14, assign30130_e47525_d_n15, assign30130_e47525_d_n16, assign30130_e47525_d_n17, assign30130_e47525_d_n18, assign30130_e47525_d_n19, assign30130_e47525_d_n20, assign30130_e47525_d_n21, assign30130_e47525_d_n22,) = {
    if ((locals.var_guard504 == 0.0) && (locals.var_guard513 != 0.0)) {
        let assign30130_e47510: f64 = (0.5 * locals.var_vg0_fp4s);
        let assign30130_e47514: f64 = (locals.var_vg0_fp4s * locals.var_vg0_fp4s);
        let assign30130_e47517: f64 = (4.0 * 0.3);
        let assign30130_e47519: f64 = (assign30130_e47517 * 0.3);
        let assign30130_e47520: f64 = (assign30130_e47514 + assign30130_e47519);
        let assign30130_e47521: f64 = (assign30130_e47520).sqrt();
        let assign30130_e47522: f64 = (0.5 * assign30130_e47521);
        let assign30130_e47523: f64 = (assign30130_e47510 + assign30130_e47522);
        (assign30130_e47523, ((0.5 * locals.var_vg0_fp4s_dn0) + (0.5 * (((locals.var_vg0_fp4s_dn0 * locals.var_vg0_fp4s) + (locals.var_vg0_fp4s * locals.var_vg0_fp4s_dn0)) / (2.0 * assign30130_e47521)))), ((0.5 * locals.var_vg0_fp4s_dn1) + (0.5 * (((locals.var_vg0_fp4s_dn1 * locals.var_vg0_fp4s) + (locals.var_vg0_fp4s * locals.var_vg0_fp4s_dn1)) / (2.0 * assign30130_e47521)))), ((0.5 * locals.var_vg0_fp4s_dn2) + (0.5 * (((locals.var_vg0_fp4s_dn2 * locals.var_vg0_fp4s) + (locals.var_vg0_fp4s * locals.var_vg0_fp4s_dn2)) / (2.0 * assign30130_e47521)))), ((0.5 * locals.var_vg0_fp4s_dn3) + (0.5 * (((locals.var_vg0_fp4s_dn3 * locals.var_vg0_fp4s) + (locals.var_vg0_fp4s * locals.var_vg0_fp4s_dn3)) / (2.0 * assign30130_e47521)))), ((0.5 * locals.var_vg0_fp4s_dn4) + (0.5 * (((locals.var_vg0_fp4s_dn4 * locals.var_vg0_fp4s) + (locals.var_vg0_fp4s * locals.var_vg0_fp4s_dn4)) / (2.0 * assign30130_e47521)))), ((0.5 * locals.var_vg0_fp4s_dn5) + (0.5 * (((locals.var_vg0_fp4s_dn5 * locals.var_vg0_fp4s) + (locals.var_vg0_fp4s * locals.var_vg0_fp4s_dn5)) / (2.0 * assign30130_e47521)))), ((0.5 * locals.var_vg0_fp4s_dn6) + (0.5 * (((locals.var_vg0_fp4s_dn6 * locals.var_vg0_fp4s) + (locals.var_vg0_fp4s * locals.var_vg0_fp4s_dn6)) / (2.0 * assign30130_e47521)))), ((0.5 * locals.var_vg0_fp4s_dn7) + (0.5 * (((locals.var_vg0_fp4s_dn7 * locals.var_vg0_fp4s) + (locals.var_vg0_fp4s * locals.var_vg0_fp4s_dn7)) / (2.0 * assign30130_e47521)))), ((0.5 * locals.var_vg0_fp4s_dn8) + (0.5 * (((locals.var_vg0_fp4s_dn8 * locals.var_vg0_fp4s) + (locals.var_vg0_fp4s * locals.var_vg0_fp4s_dn8)) / (2.0 * assign30130_e47521)))), ((0.5 * locals.var_vg0_fp4s_dn9) + (0.5 * (((locals.var_vg0_fp4s_dn9 * locals.var_vg0_fp4s) + (locals.var_vg0_fp4s * locals.var_vg0_fp4s_dn9)) / (2.0 * assign30130_e47521)))), ((0.5 * locals.var_vg0_fp4s_dn12) + (0.5 * (((locals.var_vg0_fp4s_dn12 * locals.var_vg0_fp4s) + (locals.var_vg0_fp4s * locals.var_vg0_fp4s_dn12)) / (2.0 * assign30130_e47521)))), ((0.5 * locals.var_vg0_fp4s_dn14) + (0.5 * (((locals.var_vg0_fp4s_dn14 * locals.var_vg0_fp4s) + (locals.var_vg0_fp4s * locals.var_vg0_fp4s_dn14)) / (2.0 * assign30130_e47521)))), ((0.5 * locals.var_vg0_fp4s_dn15) + (0.5 * (((locals.var_vg0_fp4s_dn15 * locals.var_vg0_fp4s) + (locals.var_vg0_fp4s * locals.var_vg0_fp4s_dn15)) / (2.0 * assign30130_e47521)))), ((0.5 * locals.var_vg0_fp4s_dn16) + (0.5 * (((locals.var_vg0_fp4s_dn16 * locals.var_vg0_fp4s) + (locals.var_vg0_fp4s * locals.var_vg0_fp4s_dn16)) / (2.0 * assign30130_e47521)))), ((0.5 * locals.var_vg0_fp4s_dn17) + (0.5 * (((locals.var_vg0_fp4s_dn17 * locals.var_vg0_fp4s) + (locals.var_vg0_fp4s * locals.var_vg0_fp4s_dn17)) / (2.0 * assign30130_e47521)))), ((0.5 * locals.var_vg0_fp4s_dn18) + (0.5 * (((locals.var_vg0_fp4s_dn18 * locals.var_vg0_fp4s) + (locals.var_vg0_fp4s * locals.var_vg0_fp4s_dn18)) / (2.0 * assign30130_e47521)))), ((0.5 * locals.var_vg0_fp4s_dn19) + (0.5 * (((locals.var_vg0_fp4s_dn19 * locals.var_vg0_fp4s) + (locals.var_vg0_fp4s * locals.var_vg0_fp4s_dn19)) / (2.0 * assign30130_e47521)))), ((0.5 * locals.var_vg0_fp4s_dn20) + (0.5 * (((locals.var_vg0_fp4s_dn20 * locals.var_vg0_fp4s) + (locals.var_vg0_fp4s * locals.var_vg0_fp4s_dn20)) / (2.0 * assign30130_e47521)))), ((0.5 * locals.var_vg0_fp4s_dn21) + (0.5 * (((locals.var_vg0_fp4s_dn21 * locals.var_vg0_fp4s) + (locals.var_vg0_fp4s * locals.var_vg0_fp4s_dn21)) / (2.0 * assign30130_e47521)))), ((0.5 * locals.var_vg0_fp4s_dn22) + (0.5 * (((locals.var_vg0_fp4s_dn22 * locals.var_vg0_fp4s) + (locals.var_vg0_fp4s * locals.var_vg0_fp4s_dn22)) / (2.0 * assign30130_e47521)))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn1, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn12, locals.var_t1_dn14, locals.var_t1_dn15, locals.var_t1_dn16, locals.var_t1_dn17, locals.var_t1_dn18, locals.var_t1_dn19, locals.var_t1_dn20, locals.var_t1_dn21, locals.var_t1_dn22,)
    }
};
        locals.var_t1 = assign30130_e47525;
        locals.var_t1_dn0 = assign30130_e47525_d_n0;
        locals.var_t1_dn1 = assign30130_e47525_d_n1;
        locals.var_t1_dn2 = assign30130_e47525_d_n2;
        locals.var_t1_dn3 = assign30130_e47525_d_n3;
        locals.var_t1_dn4 = assign30130_e47525_d_n4;
        locals.var_t1_dn5 = assign30130_e47525_d_n5;
        locals.var_t1_dn6 = assign30130_e47525_d_n6;
        locals.var_t1_dn7 = assign30130_e47525_d_n7;
        locals.var_t1_dn8 = assign30130_e47525_d_n8;
        locals.var_t1_dn9 = assign30130_e47525_d_n9;
        locals.var_t1_dn12 = assign30130_e47525_d_n12;
        locals.var_t1_dn14 = assign30130_e47525_d_n14;
        locals.var_t1_dn15 = assign30130_e47525_d_n15;
        locals.var_t1_dn16 = assign30130_e47525_d_n16;
        locals.var_t1_dn17 = assign30130_e47525_d_n17;
        locals.var_t1_dn18 = assign30130_e47525_d_n18;
        locals.var_t1_dn19 = assign30130_e47525_d_n19;
        locals.var_t1_dn20 = assign30130_e47525_d_n20;
        locals.var_t1_dn21 = assign30130_e47525_d_n21;
        locals.var_t1_dn22 = assign30130_e47525_d_n22;

        let (assign30140_e47542, assign30140_e47542_d_n0, assign30140_e47542_d_n1, assign30140_e47542_d_n2, assign30140_e47542_d_n3, assign30140_e47542_d_n4, assign30140_e47542_d_n5, assign30140_e47542_d_n6, assign30140_e47542_d_n7, assign30140_e47542_d_n8, assign30140_e47542_d_n9, assign30140_e47542_d_n12, assign30140_e47542_d_n14, assign30140_e47542_d_n15, assign30140_e47542_d_n16, assign30140_e47542_d_n17, assign30140_e47542_d_n18, assign30140_e47542_d_n19, assign30140_e47542_d_n20, assign30140_e47542_d_n21, assign30140_e47542_d_n22,) = {
    if ((locals.var_guard504 == 0.0) && (locals.var_guard513 != 0.0)) {
        let assign30140_e47532: f64 = (locals.var_t0 * p.p200);
        let assign30140_e47534: f64 = (assign30140_e47532 * locals.var_t1);
        let assign30140_e47537: f64 = (locals.var_t0 * p.p200);
        let assign30140_e47539: f64 = (assign30140_e47537 + locals.var_t1);
        let assign30140_e47540: f64 = (assign30140_e47534 / assign30140_e47539);
        (assign30140_e47540, ((((((locals.var_t0_dn0 * p.p200) * locals.var_t1) + (assign30140_e47532 * locals.var_t1_dn0)) * assign30140_e47539) - (assign30140_e47534 * ((locals.var_t0_dn0 * p.p200) + locals.var_t1_dn0))) / (assign30140_e47539 * assign30140_e47539)), ((((((locals.var_t0_dn1 * p.p200) * locals.var_t1) + (assign30140_e47532 * locals.var_t1_dn1)) * assign30140_e47539) - (assign30140_e47534 * ((locals.var_t0_dn1 * p.p200) + locals.var_t1_dn1))) / (assign30140_e47539 * assign30140_e47539)), ((((((locals.var_t0_dn2 * p.p200) * locals.var_t1) + (assign30140_e47532 * locals.var_t1_dn2)) * assign30140_e47539) - (assign30140_e47534 * ((locals.var_t0_dn2 * p.p200) + locals.var_t1_dn2))) / (assign30140_e47539 * assign30140_e47539)), ((((((locals.var_t0_dn3 * p.p200) * locals.var_t1) + (assign30140_e47532 * locals.var_t1_dn3)) * assign30140_e47539) - (assign30140_e47534 * ((locals.var_t0_dn3 * p.p200) + locals.var_t1_dn3))) / (assign30140_e47539 * assign30140_e47539)), ((((((locals.var_t0_dn4 * p.p200) * locals.var_t1) + (assign30140_e47532 * locals.var_t1_dn4)) * assign30140_e47539) - (assign30140_e47534 * ((locals.var_t0_dn4 * p.p200) + locals.var_t1_dn4))) / (assign30140_e47539 * assign30140_e47539)), ((((((locals.var_t0_dn5 * p.p200) * locals.var_t1) + (assign30140_e47532 * locals.var_t1_dn5)) * assign30140_e47539) - (assign30140_e47534 * ((locals.var_t0_dn5 * p.p200) + locals.var_t1_dn5))) / (assign30140_e47539 * assign30140_e47539)), ((((((locals.var_t0_dn6 * p.p200) * locals.var_t1) + (assign30140_e47532 * locals.var_t1_dn6)) * assign30140_e47539) - (assign30140_e47534 * ((locals.var_t0_dn6 * p.p200) + locals.var_t1_dn6))) / (assign30140_e47539 * assign30140_e47539)), ((((((locals.var_t0_dn7 * p.p200) * locals.var_t1) + (assign30140_e47532 * locals.var_t1_dn7)) * assign30140_e47539) - (assign30140_e47534 * ((locals.var_t0_dn7 * p.p200) + locals.var_t1_dn7))) / (assign30140_e47539 * assign30140_e47539)), ((((((locals.var_t0_dn8 * p.p200) * locals.var_t1) + (assign30140_e47532 * locals.var_t1_dn8)) * assign30140_e47539) - (assign30140_e47534 * ((locals.var_t0_dn8 * p.p200) + locals.var_t1_dn8))) / (assign30140_e47539 * assign30140_e47539)), ((((((locals.var_t0_dn9 * p.p200) * locals.var_t1) + (assign30140_e47532 * locals.var_t1_dn9)) * assign30140_e47539) - (assign30140_e47534 * ((locals.var_t0_dn9 * p.p200) + locals.var_t1_dn9))) / (assign30140_e47539 * assign30140_e47539)), ((((((locals.var_t0_dn12 * p.p200) * locals.var_t1) + (assign30140_e47532 * locals.var_t1_dn12)) * assign30140_e47539) - (assign30140_e47534 * ((locals.var_t0_dn12 * p.p200) + locals.var_t1_dn12))) / (assign30140_e47539 * assign30140_e47539)), ((((((locals.var_t0_dn14 * p.p200) * locals.var_t1) + (assign30140_e47532 * locals.var_t1_dn14)) * assign30140_e47539) - (assign30140_e47534 * ((locals.var_t0_dn14 * p.p200) + locals.var_t1_dn14))) / (assign30140_e47539 * assign30140_e47539)), ((((((locals.var_t0_dn15 * p.p200) * locals.var_t1) + (assign30140_e47532 * locals.var_t1_dn15)) * assign30140_e47539) - (assign30140_e47534 * ((locals.var_t0_dn15 * p.p200) + locals.var_t1_dn15))) / (assign30140_e47539 * assign30140_e47539)), ((((((locals.var_t0_dn16 * p.p200) * locals.var_t1) + (assign30140_e47532 * locals.var_t1_dn16)) * assign30140_e47539) - (assign30140_e47534 * ((locals.var_t0_dn16 * p.p200) + locals.var_t1_dn16))) / (assign30140_e47539 * assign30140_e47539)), ((((((locals.var_t0_dn17 * p.p200) * locals.var_t1) + (assign30140_e47532 * locals.var_t1_dn17)) * assign30140_e47539) - (assign30140_e47534 * ((locals.var_t0_dn17 * p.p200) + locals.var_t1_dn17))) / (assign30140_e47539 * assign30140_e47539)), ((((((locals.var_t0_dn18 * p.p200) * locals.var_t1) + (assign30140_e47532 * locals.var_t1_dn18)) * assign30140_e47539) - (assign30140_e47534 * ((locals.var_t0_dn18 * p.p200) + locals.var_t1_dn18))) / (assign30140_e47539 * assign30140_e47539)), ((((((locals.var_t0_dn19 * p.p200) * locals.var_t1) + (assign30140_e47532 * locals.var_t1_dn19)) * assign30140_e47539) - (assign30140_e47534 * ((locals.var_t0_dn19 * p.p200) + locals.var_t1_dn19))) / (assign30140_e47539 * assign30140_e47539)), ((((((locals.var_t0_dn20 * p.p200) * locals.var_t1) + (assign30140_e47532 * locals.var_t1_dn20)) * assign30140_e47539) - (assign30140_e47534 * ((locals.var_t0_dn20 * p.p200) + locals.var_t1_dn20))) / (assign30140_e47539 * assign30140_e47539)), ((((((locals.var_t0_dn21 * p.p200) * locals.var_t1) + (assign30140_e47532 * locals.var_t1_dn21)) * assign30140_e47539) - (assign30140_e47534 * ((locals.var_t0_dn21 * p.p200) + locals.var_t1_dn21))) / (assign30140_e47539 * assign30140_e47539)), ((((((locals.var_t0_dn22 * p.p200) * locals.var_t1) + (assign30140_e47532 * locals.var_t1_dn22)) * assign30140_e47539) - (assign30140_e47534 * ((locals.var_t0_dn22 * p.p200) + locals.var_t1_dn22))) / (assign30140_e47539 * assign30140_e47539)),)
    } else {
        (locals.var_vdsat, locals.var_vdsat_dn0, locals.var_vdsat_dn1, locals.var_vdsat_dn2, locals.var_vdsat_dn3, locals.var_vdsat_dn4, locals.var_vdsat_dn5, locals.var_vdsat_dn6, locals.var_vdsat_dn7, locals.var_vdsat_dn8, locals.var_vdsat_dn9, locals.var_vdsat_dn12, locals.var_vdsat_dn14, locals.var_vdsat_dn15, locals.var_vdsat_dn16, locals.var_vdsat_dn17, locals.var_vdsat_dn18, locals.var_vdsat_dn19, locals.var_vdsat_dn20, locals.var_vdsat_dn21, locals.var_vdsat_dn22,)
    }
};
        locals.var_vdsat = assign30140_e47542;
        locals.var_vdsat_dn0 = assign30140_e47542_d_n0;
        locals.var_vdsat_dn1 = assign30140_e47542_d_n1;
        locals.var_vdsat_dn2 = assign30140_e47542_d_n2;
        locals.var_vdsat_dn3 = assign30140_e47542_d_n3;
        locals.var_vdsat_dn4 = assign30140_e47542_d_n4;
        locals.var_vdsat_dn5 = assign30140_e47542_d_n5;
        locals.var_vdsat_dn6 = assign30140_e47542_d_n6;
        locals.var_vdsat_dn7 = assign30140_e47542_d_n7;
        locals.var_vdsat_dn8 = assign30140_e47542_d_n8;
        locals.var_vdsat_dn9 = assign30140_e47542_d_n9;
        locals.var_vdsat_dn12 = assign30140_e47542_d_n12;
        locals.var_vdsat_dn14 = assign30140_e47542_d_n14;
        locals.var_vdsat_dn15 = assign30140_e47542_d_n15;
        locals.var_vdsat_dn16 = assign30140_e47542_d_n16;
        locals.var_vdsat_dn17 = assign30140_e47542_d_n17;
        locals.var_vdsat_dn18 = assign30140_e47542_d_n18;
        locals.var_vdsat_dn19 = assign30140_e47542_d_n19;
        locals.var_vdsat_dn20 = assign30140_e47542_d_n20;
        locals.var_vdsat_dn21 = assign30140_e47542_d_n21;
        locals.var_vdsat_dn22 = assign30140_e47542_d_n22;

        let (assign30150_e47553, assign30150_e47553_d_n0, assign30150_e47553_d_n1, assign30150_e47553_d_n2, assign30150_e47553_d_n3, assign30150_e47553_d_n4, assign30150_e47553_d_n5, assign30150_e47553_d_n6, assign30150_e47553_d_n7, assign30150_e47553_d_n8, assign30150_e47553_d_n9, assign30150_e47553_d_n12, assign30150_e47553_d_n14, assign30150_e47553_d_n15, assign30150_e47553_d_n16, assign30150_e47553_d_n17, assign30150_e47553_d_n18, assign30150_e47553_d_n19, assign30150_e47553_d_n20, assign30150_e47553_d_n21, assign30150_e47553_d_n22,) = {
    if ((locals.var_guard504 == 0.0) && (locals.var_guard513 != 0.0)) {
        let assign30150_e47549: f64 = (locals.var_vds_fp4s / locals.var_vdsat);
        let assign30150_e47551: f64 = (assign30150_e47549).powf(p.p18);
        (assign30150_e47551, if 0.0 == 0.0 && ((p.p18) as f64).is_finite() && ((p.p18) as f64).fract() == 0.0 { if p.p18 == 0.0 { 0.0 } else { (p.p18 * ((assign30150_e47549).powf(p.p18 - 1.0) * (-((locals.var_vds_fp4s * locals.var_vdsat_dn0) / (locals.var_vdsat * locals.var_vdsat))))) } } else { (assign30150_e47551 * (p.p18 * ((-((locals.var_vds_fp4s * locals.var_vdsat_dn0) / (locals.var_vdsat * locals.var_vdsat))) / assign30150_e47549))) }, if 0.0 == 0.0 && ((p.p18) as f64).is_finite() && ((p.p18) as f64).fract() == 0.0 { if p.p18 == 0.0 { 0.0 } else { (p.p18 * ((assign30150_e47549).powf(p.p18 - 1.0) * (-((locals.var_vds_fp4s * locals.var_vdsat_dn1) / (locals.var_vdsat * locals.var_vdsat))))) } } else { (assign30150_e47551 * (p.p18 * ((-((locals.var_vds_fp4s * locals.var_vdsat_dn1) / (locals.var_vdsat * locals.var_vdsat))) / assign30150_e47549))) }, if 0.0 == 0.0 && ((p.p18) as f64).is_finite() && ((p.p18) as f64).fract() == 0.0 { if p.p18 == 0.0 { 0.0 } else { (p.p18 * ((assign30150_e47549).powf(p.p18 - 1.0) * (-((locals.var_vds_fp4s * locals.var_vdsat_dn2) / (locals.var_vdsat * locals.var_vdsat))))) } } else { (assign30150_e47551 * (p.p18 * ((-((locals.var_vds_fp4s * locals.var_vdsat_dn2) / (locals.var_vdsat * locals.var_vdsat))) / assign30150_e47549))) }, if 0.0 == 0.0 && ((p.p18) as f64).is_finite() && ((p.p18) as f64).fract() == 0.0 { if p.p18 == 0.0 { 0.0 } else { (p.p18 * ((assign30150_e47549).powf(p.p18 - 1.0) * (-((locals.var_vds_fp4s * locals.var_vdsat_dn3) / (locals.var_vdsat * locals.var_vdsat))))) } } else { (assign30150_e47551 * (p.p18 * ((-((locals.var_vds_fp4s * locals.var_vdsat_dn3) / (locals.var_vdsat * locals.var_vdsat))) / assign30150_e47549))) }, if 0.0 == 0.0 && ((p.p18) as f64).is_finite() && ((p.p18) as f64).fract() == 0.0 { if p.p18 == 0.0 { 0.0 } else { (p.p18 * ((assign30150_e47549).powf(p.p18 - 1.0) * (-((locals.var_vds_fp4s * locals.var_vdsat_dn4) / (locals.var_vdsat * locals.var_vdsat))))) } } else { (assign30150_e47551 * (p.p18 * ((-((locals.var_vds_fp4s * locals.var_vdsat_dn4) / (locals.var_vdsat * locals.var_vdsat))) / assign30150_e47549))) }, if 0.0 == 0.0 && ((p.p18) as f64).is_finite() && ((p.p18) as f64).fract() == 0.0 { if p.p18 == 0.0 { 0.0 } else { (p.p18 * ((assign30150_e47549).powf(p.p18 - 1.0) * (-((locals.var_vds_fp4s * locals.var_vdsat_dn5) / (locals.var_vdsat * locals.var_vdsat))))) } } else { (assign30150_e47551 * (p.p18 * ((-((locals.var_vds_fp4s * locals.var_vdsat_dn5) / (locals.var_vdsat * locals.var_vdsat))) / assign30150_e47549))) }, if 0.0 == 0.0 && ((p.p18) as f64).is_finite() && ((p.p18) as f64).fract() == 0.0 { if p.p18 == 0.0 { 0.0 } else { (p.p18 * ((assign30150_e47549).powf(p.p18 - 1.0) * (-((locals.var_vds_fp4s * locals.var_vdsat_dn6) / (locals.var_vdsat * locals.var_vdsat))))) } } else { (assign30150_e47551 * (p.p18 * ((-((locals.var_vds_fp4s * locals.var_vdsat_dn6) / (locals.var_vdsat * locals.var_vdsat))) / assign30150_e47549))) }, if 0.0 == 0.0 && ((p.p18) as f64).is_finite() && ((p.p18) as f64).fract() == 0.0 { if p.p18 == 0.0 { 0.0 } else { (p.p18 * ((assign30150_e47549).powf(p.p18 - 1.0) * (-((locals.var_vds_fp4s * locals.var_vdsat_dn7) / (locals.var_vdsat * locals.var_vdsat))))) } } else { (assign30150_e47551 * (p.p18 * ((-((locals.var_vds_fp4s * locals.var_vdsat_dn7) / (locals.var_vdsat * locals.var_vdsat))) / assign30150_e47549))) }, if 0.0 == 0.0 && ((p.p18) as f64).is_finite() && ((p.p18) as f64).fract() == 0.0 { if p.p18 == 0.0 { 0.0 } else { (p.p18 * ((assign30150_e47549).powf(p.p18 - 1.0) * (-((locals.var_vds_fp4s * locals.var_vdsat_dn8) / (locals.var_vdsat * locals.var_vdsat))))) } } else { (assign30150_e47551 * (p.p18 * ((-((locals.var_vds_fp4s * locals.var_vdsat_dn8) / (locals.var_vdsat * locals.var_vdsat))) / assign30150_e47549))) }, if 0.0 == 0.0 && ((p.p18) as f64).is_finite() && ((p.p18) as f64).fract() == 0.0 { if p.p18 == 0.0 { 0.0 } else { (p.p18 * ((assign30150_e47549).powf(p.p18 - 1.0) * (-((locals.var_vds_fp4s * locals.var_vdsat_dn9) / (locals.var_vdsat * locals.var_vdsat))))) } } else { (assign30150_e47551 * (p.p18 * ((-((locals.var_vds_fp4s * locals.var_vdsat_dn9) / (locals.var_vdsat * locals.var_vdsat))) / assign30150_e47549))) }, if 0.0 == 0.0 && ((p.p18) as f64).is_finite() && ((p.p18) as f64).fract() == 0.0 { if p.p18 == 0.0 { 0.0 } else { (p.p18 * ((assign30150_e47549).powf(p.p18 - 1.0) * (-((locals.var_vds_fp4s * locals.var_vdsat_dn12) / (locals.var_vdsat * locals.var_vdsat))))) } } else { (assign30150_e47551 * (p.p18 * ((-((locals.var_vds_fp4s * locals.var_vdsat_dn12) / (locals.var_vdsat * locals.var_vdsat))) / assign30150_e47549))) }, if 0.0 == 0.0 && ((p.p18) as f64).is_finite() && ((p.p18) as f64).fract() == 0.0 { if p.p18 == 0.0 { 0.0 } else { (p.p18 * ((assign30150_e47549).powf(p.p18 - 1.0) * (-((locals.var_vds_fp4s * locals.var_vdsat_dn14) / (locals.var_vdsat * locals.var_vdsat))))) } } else { (assign30150_e47551 * (p.p18 * ((-((locals.var_vds_fp4s * locals.var_vdsat_dn14) / (locals.var_vdsat * locals.var_vdsat))) / assign30150_e47549))) }, if 0.0 == 0.0 && ((p.p18) as f64).is_finite() && ((p.p18) as f64).fract() == 0.0 { if p.p18 == 0.0 { 0.0 } else { (p.p18 * ((assign30150_e47549).powf(p.p18 - 1.0) * (-((locals.var_vds_fp4s * locals.var_vdsat_dn15) / (locals.var_vdsat * locals.var_vdsat))))) } } else { (assign30150_e47551 * (p.p18 * ((-((locals.var_vds_fp4s * locals.var_vdsat_dn15) / (locals.var_vdsat * locals.var_vdsat))) / assign30150_e47549))) }, if 0.0 == 0.0 && ((p.p18) as f64).is_finite() && ((p.p18) as f64).fract() == 0.0 { if p.p18 == 0.0 { 0.0 } else { (p.p18 * ((assign30150_e47549).powf(p.p18 - 1.0) * (-((locals.var_vds_fp4s * locals.var_vdsat_dn16) / (locals.var_vdsat * locals.var_vdsat))))) } } else { (assign30150_e47551 * (p.p18 * ((-((locals.var_vds_fp4s * locals.var_vdsat_dn16) / (locals.var_vdsat * locals.var_vdsat))) / assign30150_e47549))) }, if 0.0 == 0.0 && ((p.p18) as f64).is_finite() && ((p.p18) as f64).fract() == 0.0 { if p.p18 == 0.0 { 0.0 } else { (p.p18 * ((assign30150_e47549).powf(p.p18 - 1.0) * (-((locals.var_vds_fp4s * locals.var_vdsat_dn17) / (locals.var_vdsat * locals.var_vdsat))))) } } else { (assign30150_e47551 * (p.p18 * ((-((locals.var_vds_fp4s * locals.var_vdsat_dn17) / (locals.var_vdsat * locals.var_vdsat))) / assign30150_e47549))) }, if 0.0 == 0.0 && ((p.p18) as f64).is_finite() && ((p.p18) as f64).fract() == 0.0 { if p.p18 == 0.0 { 0.0 } else { (p.p18 * ((assign30150_e47549).powf(p.p18 - 1.0) * (-((locals.var_vds_fp4s * locals.var_vdsat_dn18) / (locals.var_vdsat * locals.var_vdsat))))) } } else { (assign30150_e47551 * (p.p18 * ((-((locals.var_vds_fp4s * locals.var_vdsat_dn18) / (locals.var_vdsat * locals.var_vdsat))) / assign30150_e47549))) }, if 0.0 == 0.0 && ((p.p18) as f64).is_finite() && ((p.p18) as f64).fract() == 0.0 { if p.p18 == 0.0 { 0.0 } else { (p.p18 * ((assign30150_e47549).powf(p.p18 - 1.0) * (-((locals.var_vds_fp4s * locals.var_vdsat_dn19) / (locals.var_vdsat * locals.var_vdsat))))) } } else { (assign30150_e47551 * (p.p18 * ((-((locals.var_vds_fp4s * locals.var_vdsat_dn19) / (locals.var_vdsat * locals.var_vdsat))) / assign30150_e47549))) }, if 0.0 == 0.0 && ((p.p18) as f64).is_finite() && ((p.p18) as f64).fract() == 0.0 { if p.p18 == 0.0 { 0.0 } else { (p.p18 * ((assign30150_e47549).powf(p.p18 - 1.0) * (-((locals.var_vds_fp4s * locals.var_vdsat_dn20) / (locals.var_vdsat * locals.var_vdsat))))) } } else { (assign30150_e47551 * (p.p18 * ((-((locals.var_vds_fp4s * locals.var_vdsat_dn20) / (locals.var_vdsat * locals.var_vdsat))) / assign30150_e47549))) }, if 0.0 == 0.0 && ((p.p18) as f64).is_finite() && ((p.p18) as f64).fract() == 0.0 { if p.p18 == 0.0 { 0.0 } else { (p.p18 * ((assign30150_e47549).powf(p.p18 - 1.0) * (((locals.var_vds_fp4s_dn21 * locals.var_vdsat) - (locals.var_vds_fp4s * locals.var_vdsat_dn21)) / (locals.var_vdsat * locals.var_vdsat)))) } } else { (assign30150_e47551 * (p.p18 * ((((locals.var_vds_fp4s_dn21 * locals.var_vdsat) - (locals.var_vds_fp4s * locals.var_vdsat_dn21)) / (locals.var_vdsat * locals.var_vdsat)) / assign30150_e47549))) }, if 0.0 == 0.0 && ((p.p18) as f64).is_finite() && ((p.p18) as f64).fract() == 0.0 { if p.p18 == 0.0 { 0.0 } else { (p.p18 * ((assign30150_e47549).powf(p.p18 - 1.0) * (((locals.var_vds_fp4s_dn22 * locals.var_vdsat) - (locals.var_vds_fp4s * locals.var_vdsat_dn22)) / (locals.var_vdsat * locals.var_vdsat)))) } } else { (assign30150_e47551 * (p.p18 * ((((locals.var_vds_fp4s_dn22 * locals.var_vdsat) - (locals.var_vds_fp4s * locals.var_vdsat_dn22)) / (locals.var_vdsat * locals.var_vdsat)) / assign30150_e47549))) },)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn1, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn12, locals.var_t0_dn14, locals.var_t0_dn15, locals.var_t0_dn16, locals.var_t0_dn17, locals.var_t0_dn18, locals.var_t0_dn19, locals.var_t0_dn20, locals.var_t0_dn21, locals.var_t0_dn22,)
    }
};
        locals.var_t0 = assign30150_e47553;
        locals.var_t0_dn0 = assign30150_e47553_d_n0;
        locals.var_t0_dn1 = assign30150_e47553_d_n1;
        locals.var_t0_dn2 = assign30150_e47553_d_n2;
        locals.var_t0_dn3 = assign30150_e47553_d_n3;
        locals.var_t0_dn4 = assign30150_e47553_d_n4;
        locals.var_t0_dn5 = assign30150_e47553_d_n5;
        locals.var_t0_dn6 = assign30150_e47553_d_n6;
        locals.var_t0_dn7 = assign30150_e47553_d_n7;
        locals.var_t0_dn8 = assign30150_e47553_d_n8;
        locals.var_t0_dn9 = assign30150_e47553_d_n9;
        locals.var_t0_dn12 = assign30150_e47553_d_n12;
        locals.var_t0_dn14 = assign30150_e47553_d_n14;
        locals.var_t0_dn15 = assign30150_e47553_d_n15;
        locals.var_t0_dn16 = assign30150_e47553_d_n16;
        locals.var_t0_dn17 = assign30150_e47553_d_n17;
        locals.var_t0_dn18 = assign30150_e47553_d_n18;
        locals.var_t0_dn19 = assign30150_e47553_d_n19;
        locals.var_t0_dn20 = assign30150_e47553_d_n20;
        locals.var_t0_dn21 = assign30150_e47553_d_n21;
        locals.var_t0_dn22 = assign30150_e47553_d_n22;

        let (assign30160_e47567, assign30160_e47567_d_n0, assign30160_e47567_d_n1, assign30160_e47567_d_n2, assign30160_e47567_d_n3, assign30160_e47567_d_n4, assign30160_e47567_d_n5, assign30160_e47567_d_n6, assign30160_e47567_d_n7, assign30160_e47567_d_n8, assign30160_e47567_d_n9, assign30160_e47567_d_n12, assign30160_e47567_d_n14, assign30160_e47567_d_n15, assign30160_e47567_d_n16, assign30160_e47567_d_n17, assign30160_e47567_d_n18, assign30160_e47567_d_n19, assign30160_e47567_d_n20, assign30160_e47567_d_n21, assign30160_e47567_d_n22,) = {
    if ((locals.var_guard504 == 0.0) && (locals.var_guard513 != 0.0)) {
        let assign30160_e47560: f64 = (1.0 + locals.var_t0);
        let assign30160_e47562: f64 = (-1.0);
        let assign30160_e47564: f64 = (assign30160_e47562 / p.p18);
        let assign30160_e47565: f64 = (assign30160_e47560).powf(assign30160_e47564);
        (assign30160_e47565, if 0.0 == 0.0 && ((assign30160_e47564) as f64).is_finite() && ((assign30160_e47564) as f64).fract() == 0.0 { if assign30160_e47564 == 0.0 { 0.0 } else { (assign30160_e47564 * ((assign30160_e47560).powf(assign30160_e47564 - 1.0) * locals.var_t0_dn0)) } } else { (assign30160_e47565 * (assign30160_e47564 * (locals.var_t0_dn0 / assign30160_e47560))) }, if 0.0 == 0.0 && ((assign30160_e47564) as f64).is_finite() && ((assign30160_e47564) as f64).fract() == 0.0 { if assign30160_e47564 == 0.0 { 0.0 } else { (assign30160_e47564 * ((assign30160_e47560).powf(assign30160_e47564 - 1.0) * locals.var_t0_dn1)) } } else { (assign30160_e47565 * (assign30160_e47564 * (locals.var_t0_dn1 / assign30160_e47560))) }, if 0.0 == 0.0 && ((assign30160_e47564) as f64).is_finite() && ((assign30160_e47564) as f64).fract() == 0.0 { if assign30160_e47564 == 0.0 { 0.0 } else { (assign30160_e47564 * ((assign30160_e47560).powf(assign30160_e47564 - 1.0) * locals.var_t0_dn2)) } } else { (assign30160_e47565 * (assign30160_e47564 * (locals.var_t0_dn2 / assign30160_e47560))) }, if 0.0 == 0.0 && ((assign30160_e47564) as f64).is_finite() && ((assign30160_e47564) as f64).fract() == 0.0 { if assign30160_e47564 == 0.0 { 0.0 } else { (assign30160_e47564 * ((assign30160_e47560).powf(assign30160_e47564 - 1.0) * locals.var_t0_dn3)) } } else { (assign30160_e47565 * (assign30160_e47564 * (locals.var_t0_dn3 / assign30160_e47560))) }, if 0.0 == 0.0 && ((assign30160_e47564) as f64).is_finite() && ((assign30160_e47564) as f64).fract() == 0.0 { if assign30160_e47564 == 0.0 { 0.0 } else { (assign30160_e47564 * ((assign30160_e47560).powf(assign30160_e47564 - 1.0) * locals.var_t0_dn4)) } } else { (assign30160_e47565 * (assign30160_e47564 * (locals.var_t0_dn4 / assign30160_e47560))) }, if 0.0 == 0.0 && ((assign30160_e47564) as f64).is_finite() && ((assign30160_e47564) as f64).fract() == 0.0 { if assign30160_e47564 == 0.0 { 0.0 } else { (assign30160_e47564 * ((assign30160_e47560).powf(assign30160_e47564 - 1.0) * locals.var_t0_dn5)) } } else { (assign30160_e47565 * (assign30160_e47564 * (locals.var_t0_dn5 / assign30160_e47560))) }, if 0.0 == 0.0 && ((assign30160_e47564) as f64).is_finite() && ((assign30160_e47564) as f64).fract() == 0.0 { if assign30160_e47564 == 0.0 { 0.0 } else { (assign30160_e47564 * ((assign30160_e47560).powf(assign30160_e47564 - 1.0) * locals.var_t0_dn6)) } } else { (assign30160_e47565 * (assign30160_e47564 * (locals.var_t0_dn6 / assign30160_e47560))) }, if 0.0 == 0.0 && ((assign30160_e47564) as f64).is_finite() && ((assign30160_e47564) as f64).fract() == 0.0 { if assign30160_e47564 == 0.0 { 0.0 } else { (assign30160_e47564 * ((assign30160_e47560).powf(assign30160_e47564 - 1.0) * locals.var_t0_dn7)) } } else { (assign30160_e47565 * (assign30160_e47564 * (locals.var_t0_dn7 / assign30160_e47560))) }, if 0.0 == 0.0 && ((assign30160_e47564) as f64).is_finite() && ((assign30160_e47564) as f64).fract() == 0.0 { if assign30160_e47564 == 0.0 { 0.0 } else { (assign30160_e47564 * ((assign30160_e47560).powf(assign30160_e47564 - 1.0) * locals.var_t0_dn8)) } } else { (assign30160_e47565 * (assign30160_e47564 * (locals.var_t0_dn8 / assign30160_e47560))) }, if 0.0 == 0.0 && ((assign30160_e47564) as f64).is_finite() && ((assign30160_e47564) as f64).fract() == 0.0 { if assign30160_e47564 == 0.0 { 0.0 } else { (assign30160_e47564 * ((assign30160_e47560).powf(assign30160_e47564 - 1.0) * locals.var_t0_dn9)) } } else { (assign30160_e47565 * (assign30160_e47564 * (locals.var_t0_dn9 / assign30160_e47560))) }, if 0.0 == 0.0 && ((assign30160_e47564) as f64).is_finite() && ((assign30160_e47564) as f64).fract() == 0.0 { if assign30160_e47564 == 0.0 { 0.0 } else { (assign30160_e47564 * ((assign30160_e47560).powf(assign30160_e47564 - 1.0) * locals.var_t0_dn12)) } } else { (assign30160_e47565 * (assign30160_e47564 * (locals.var_t0_dn12 / assign30160_e47560))) }, if 0.0 == 0.0 && ((assign30160_e47564) as f64).is_finite() && ((assign30160_e47564) as f64).fract() == 0.0 { if assign30160_e47564 == 0.0 { 0.0 } else { (assign30160_e47564 * ((assign30160_e47560).powf(assign30160_e47564 - 1.0) * locals.var_t0_dn14)) } } else { (assign30160_e47565 * (assign30160_e47564 * (locals.var_t0_dn14 / assign30160_e47560))) }, if 0.0 == 0.0 && ((assign30160_e47564) as f64).is_finite() && ((assign30160_e47564) as f64).fract() == 0.0 { if assign30160_e47564 == 0.0 { 0.0 } else { (assign30160_e47564 * ((assign30160_e47560).powf(assign30160_e47564 - 1.0) * locals.var_t0_dn15)) } } else { (assign30160_e47565 * (assign30160_e47564 * (locals.var_t0_dn15 / assign30160_e47560))) }, if 0.0 == 0.0 && ((assign30160_e47564) as f64).is_finite() && ((assign30160_e47564) as f64).fract() == 0.0 { if assign30160_e47564 == 0.0 { 0.0 } else { (assign30160_e47564 * ((assign30160_e47560).powf(assign30160_e47564 - 1.0) * locals.var_t0_dn16)) } } else { (assign30160_e47565 * (assign30160_e47564 * (locals.var_t0_dn16 / assign30160_e47560))) }, if 0.0 == 0.0 && ((assign30160_e47564) as f64).is_finite() && ((assign30160_e47564) as f64).fract() == 0.0 { if assign30160_e47564 == 0.0 { 0.0 } else { (assign30160_e47564 * ((assign30160_e47560).powf(assign30160_e47564 - 1.0) * locals.var_t0_dn17)) } } else { (assign30160_e47565 * (assign30160_e47564 * (locals.var_t0_dn17 / assign30160_e47560))) }, if 0.0 == 0.0 && ((assign30160_e47564) as f64).is_finite() && ((assign30160_e47564) as f64).fract() == 0.0 { if assign30160_e47564 == 0.0 { 0.0 } else { (assign30160_e47564 * ((assign30160_e47560).powf(assign30160_e47564 - 1.0) * locals.var_t0_dn18)) } } else { (assign30160_e47565 * (assign30160_e47564 * (locals.var_t0_dn18 / assign30160_e47560))) }, if 0.0 == 0.0 && ((assign30160_e47564) as f64).is_finite() && ((assign30160_e47564) as f64).fract() == 0.0 { if assign30160_e47564 == 0.0 { 0.0 } else { (assign30160_e47564 * ((assign30160_e47560).powf(assign30160_e47564 - 1.0) * locals.var_t0_dn19)) } } else { (assign30160_e47565 * (assign30160_e47564 * (locals.var_t0_dn19 / assign30160_e47560))) }, if 0.0 == 0.0 && ((assign30160_e47564) as f64).is_finite() && ((assign30160_e47564) as f64).fract() == 0.0 { if assign30160_e47564 == 0.0 { 0.0 } else { (assign30160_e47564 * ((assign30160_e47560).powf(assign30160_e47564 - 1.0) * locals.var_t0_dn20)) } } else { (assign30160_e47565 * (assign30160_e47564 * (locals.var_t0_dn20 / assign30160_e47560))) }, if 0.0 == 0.0 && ((assign30160_e47564) as f64).is_finite() && ((assign30160_e47564) as f64).fract() == 0.0 { if assign30160_e47564 == 0.0 { 0.0 } else { (assign30160_e47564 * ((assign30160_e47560).powf(assign30160_e47564 - 1.0) * locals.var_t0_dn21)) } } else { (assign30160_e47565 * (assign30160_e47564 * (locals.var_t0_dn21 / assign30160_e47560))) }, if 0.0 == 0.0 && ((assign30160_e47564) as f64).is_finite() && ((assign30160_e47564) as f64).fract() == 0.0 { if assign30160_e47564 == 0.0 { 0.0 } else { (assign30160_e47564 * ((assign30160_e47560).powf(assign30160_e47564 - 1.0) * locals.var_t0_dn22)) } } else { (assign30160_e47565 * (assign30160_e47564 * (locals.var_t0_dn22 / assign30160_e47560))) },)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn1, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn12, locals.var_t1_dn14, locals.var_t1_dn15, locals.var_t1_dn16, locals.var_t1_dn17, locals.var_t1_dn18, locals.var_t1_dn19, locals.var_t1_dn20, locals.var_t1_dn21, locals.var_t1_dn22,)
    }
};
        locals.var_t1 = assign30160_e47567;
        locals.var_t1_dn0 = assign30160_e47567_d_n0;
        locals.var_t1_dn1 = assign30160_e47567_d_n1;
        locals.var_t1_dn2 = assign30160_e47567_d_n2;
        locals.var_t1_dn3 = assign30160_e47567_d_n3;
        locals.var_t1_dn4 = assign30160_e47567_d_n4;
        locals.var_t1_dn5 = assign30160_e47567_d_n5;
        locals.var_t1_dn6 = assign30160_e47567_d_n6;
        locals.var_t1_dn7 = assign30160_e47567_d_n7;
        locals.var_t1_dn8 = assign30160_e47567_d_n8;
        locals.var_t1_dn9 = assign30160_e47567_d_n9;
        locals.var_t1_dn12 = assign30160_e47567_d_n12;
        locals.var_t1_dn14 = assign30160_e47567_d_n14;
        locals.var_t1_dn15 = assign30160_e47567_d_n15;
        locals.var_t1_dn16 = assign30160_e47567_d_n16;
        locals.var_t1_dn17 = assign30160_e47567_d_n17;
        locals.var_t1_dn18 = assign30160_e47567_d_n18;
        locals.var_t1_dn19 = assign30160_e47567_d_n19;
        locals.var_t1_dn20 = assign30160_e47567_d_n20;
        locals.var_t1_dn21 = assign30160_e47567_d_n21;
        locals.var_t1_dn22 = assign30160_e47567_d_n22;

    }

    pub(super) fn stamp_transient_block_177(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign30170_e47576, assign30170_e47576_d_n0, assign30170_e47576_d_n1, assign30170_e47576_d_n2, assign30170_e47576_d_n3, assign30170_e47576_d_n4, assign30170_e47576_d_n5, assign30170_e47576_d_n6, assign30170_e47576_d_n7, assign30170_e47576_d_n8, assign30170_e47576_d_n9, assign30170_e47576_d_n12, assign30170_e47576_d_n14, assign30170_e47576_d_n15, assign30170_e47576_d_n16, assign30170_e47576_d_n17, assign30170_e47576_d_n18, assign30170_e47576_d_n19, assign30170_e47576_d_n20, assign30170_e47576_d_n21, assign30170_e47576_d_n22,) = {
    if ((locals.var_guard504 == 0.0) && (locals.var_guard513 != 0.0)) {
        let assign30170_e47574: f64 = (locals.var_vds_fp4s * locals.var_t1);
        (assign30170_e47574, (locals.var_vds_fp4s * locals.var_t1_dn0), (locals.var_vds_fp4s * locals.var_t1_dn1), (locals.var_vds_fp4s * locals.var_t1_dn2), (locals.var_vds_fp4s * locals.var_t1_dn3), (locals.var_vds_fp4s * locals.var_t1_dn4), (locals.var_vds_fp4s * locals.var_t1_dn5), (locals.var_vds_fp4s * locals.var_t1_dn6), (locals.var_vds_fp4s * locals.var_t1_dn7), (locals.var_vds_fp4s * locals.var_t1_dn8), (locals.var_vds_fp4s * locals.var_t1_dn9), (locals.var_vds_fp4s * locals.var_t1_dn12), (locals.var_vds_fp4s * locals.var_t1_dn14), (locals.var_vds_fp4s * locals.var_t1_dn15), (locals.var_vds_fp4s * locals.var_t1_dn16), (locals.var_vds_fp4s * locals.var_t1_dn17), (locals.var_vds_fp4s * locals.var_t1_dn18), (locals.var_vds_fp4s * locals.var_t1_dn19), (locals.var_vds_fp4s * locals.var_t1_dn20), ((locals.var_vds_fp4s_dn21 * locals.var_t1) + (locals.var_vds_fp4s * locals.var_t1_dn21)), ((locals.var_vds_fp4s_dn22 * locals.var_t1) + (locals.var_vds_fp4s * locals.var_t1_dn22)),)
    } else {
        (locals.var_vdeff, locals.var_vdeff_dn0, locals.var_vdeff_dn1, locals.var_vdeff_dn2, locals.var_vdeff_dn3, locals.var_vdeff_dn4, locals.var_vdeff_dn5, locals.var_vdeff_dn6, locals.var_vdeff_dn7, locals.var_vdeff_dn8, locals.var_vdeff_dn9, locals.var_vdeff_dn12, locals.var_vdeff_dn14, locals.var_vdeff_dn15, locals.var_vdeff_dn16, locals.var_vdeff_dn17, locals.var_vdeff_dn18, locals.var_vdeff_dn19, locals.var_vdeff_dn20, locals.var_vdeff_dn21, locals.var_vdeff_dn22,)
    }
};
        locals.var_vdeff = assign30170_e47576;
        locals.var_vdeff_dn0 = assign30170_e47576_d_n0;
        locals.var_vdeff_dn1 = assign30170_e47576_d_n1;
        locals.var_vdeff_dn2 = assign30170_e47576_d_n2;
        locals.var_vdeff_dn3 = assign30170_e47576_d_n3;
        locals.var_vdeff_dn4 = assign30170_e47576_d_n4;
        locals.var_vdeff_dn5 = assign30170_e47576_d_n5;
        locals.var_vdeff_dn6 = assign30170_e47576_d_n6;
        locals.var_vdeff_dn7 = assign30170_e47576_d_n7;
        locals.var_vdeff_dn8 = assign30170_e47576_d_n8;
        locals.var_vdeff_dn9 = assign30170_e47576_d_n9;
        locals.var_vdeff_dn12 = assign30170_e47576_d_n12;
        locals.var_vdeff_dn14 = assign30170_e47576_d_n14;
        locals.var_vdeff_dn15 = assign30170_e47576_d_n15;
        locals.var_vdeff_dn16 = assign30170_e47576_d_n16;
        locals.var_vdeff_dn17 = assign30170_e47576_d_n17;
        locals.var_vdeff_dn18 = assign30170_e47576_d_n18;
        locals.var_vdeff_dn19 = assign30170_e47576_d_n19;
        locals.var_vdeff_dn20 = assign30170_e47576_d_n20;
        locals.var_vdeff_dn21 = assign30170_e47576_d_n21;
        locals.var_vdeff_dn22 = assign30170_e47576_d_n22;

        let (assign30180_e47585, assign30180_e47585_d_n0, assign30180_e47585_d_n1, assign30180_e47585_d_n2, assign30180_e47585_d_n3, assign30180_e47585_d_n4, assign30180_e47585_d_n5, assign30180_e47585_d_n6, assign30180_e47585_d_n7, assign30180_e47585_d_n8, assign30180_e47585_d_n9, assign30180_e47585_d_n12, assign30180_e47585_d_n14, assign30180_e47585_d_n15, assign30180_e47585_d_n16, assign30180_e47585_d_n17, assign30180_e47585_d_n18, assign30180_e47585_d_n19, assign30180_e47585_d_n20, assign30180_e47585_d_n21, assign30180_e47585_d_n22,) = {
    if ((locals.var_guard504 == 0.0) && (locals.var_guard513 != 0.0)) {
        let assign30180_e47583: f64 = (locals.var_vg0_fp4s - locals.var_vdeff);
        (assign30180_e47583, (locals.var_vg0_fp4s_dn0 - locals.var_vdeff_dn0), (locals.var_vg0_fp4s_dn1 - locals.var_vdeff_dn1), (locals.var_vg0_fp4s_dn2 - locals.var_vdeff_dn2), (locals.var_vg0_fp4s_dn3 - locals.var_vdeff_dn3), (locals.var_vg0_fp4s_dn4 - locals.var_vdeff_dn4), (locals.var_vg0_fp4s_dn5 - locals.var_vdeff_dn5), (locals.var_vg0_fp4s_dn6 - locals.var_vdeff_dn6), (locals.var_vg0_fp4s_dn7 - locals.var_vdeff_dn7), (locals.var_vg0_fp4s_dn8 - locals.var_vdeff_dn8), (locals.var_vg0_fp4s_dn9 - locals.var_vdeff_dn9), (locals.var_vg0_fp4s_dn12 - locals.var_vdeff_dn12), (locals.var_vg0_fp4s_dn14 - locals.var_vdeff_dn14), (locals.var_vg0_fp4s_dn15 - locals.var_vdeff_dn15), (locals.var_vg0_fp4s_dn16 - locals.var_vdeff_dn16), (locals.var_vg0_fp4s_dn17 - locals.var_vdeff_dn17), (locals.var_vg0_fp4s_dn18 - locals.var_vdeff_dn18), (locals.var_vg0_fp4s_dn19 - locals.var_vdeff_dn19), (locals.var_vg0_fp4s_dn20 - locals.var_vdeff_dn20), (locals.var_vg0_fp4s_dn21 - locals.var_vdeff_dn21), (locals.var_vg0_fp4s_dn22 - locals.var_vdeff_dn22),)
    } else {
        (locals.var_vgdeff, locals.var_vgdeff_dn0, locals.var_vgdeff_dn1, locals.var_vgdeff_dn2, locals.var_vgdeff_dn3, locals.var_vgdeff_dn4, locals.var_vgdeff_dn5, locals.var_vgdeff_dn6, locals.var_vgdeff_dn7, locals.var_vgdeff_dn8, locals.var_vgdeff_dn9, locals.var_vgdeff_dn12, locals.var_vgdeff_dn14, locals.var_vgdeff_dn15, locals.var_vgdeff_dn16, locals.var_vgdeff_dn17, locals.var_vgdeff_dn18, locals.var_vgdeff_dn19, locals.var_vgdeff_dn20, locals.var_vgdeff_dn21, locals.var_vgdeff_dn22,)
    }
};
        locals.var_vgdeff = assign30180_e47585;
        locals.var_vgdeff_dn0 = assign30180_e47585_d_n0;
        locals.var_vgdeff_dn1 = assign30180_e47585_d_n1;
        locals.var_vgdeff_dn2 = assign30180_e47585_d_n2;
        locals.var_vgdeff_dn3 = assign30180_e47585_d_n3;
        locals.var_vgdeff_dn4 = assign30180_e47585_d_n4;
        locals.var_vgdeff_dn5 = assign30180_e47585_d_n5;
        locals.var_vgdeff_dn6 = assign30180_e47585_d_n6;
        locals.var_vgdeff_dn7 = assign30180_e47585_d_n7;
        locals.var_vgdeff_dn8 = assign30180_e47585_d_n8;
        locals.var_vgdeff_dn9 = assign30180_e47585_d_n9;
        locals.var_vgdeff_dn12 = assign30180_e47585_d_n12;
        locals.var_vgdeff_dn14 = assign30180_e47585_d_n14;
        locals.var_vgdeff_dn15 = assign30180_e47585_d_n15;
        locals.var_vgdeff_dn16 = assign30180_e47585_d_n16;
        locals.var_vgdeff_dn17 = assign30180_e47585_d_n17;
        locals.var_vgdeff_dn18 = assign30180_e47585_d_n18;
        locals.var_vgdeff_dn19 = assign30180_e47585_d_n19;
        locals.var_vgdeff_dn20 = assign30180_e47585_d_n20;
        locals.var_vgdeff_dn21 = assign30180_e47585_d_n21;
        locals.var_vgdeff_dn22 = assign30180_e47585_d_n22;

        let (assign30190_e47592, assign30190_e47592_d_n0, assign30190_e47592_d_n1, assign30190_e47592_d_n2, assign30190_e47592_d_n3, assign30190_e47592_d_n4, assign30190_e47592_d_n5, assign30190_e47592_d_n6, assign30190_e47592_d_n7, assign30190_e47592_d_n8, assign30190_e47592_d_n9, assign30190_e47592_d_n12, assign30190_e47592_d_n14, assign30190_e47592_d_n15, assign30190_e47592_d_n16, assign30190_e47592_d_n17, assign30190_e47592_d_n18, assign30190_e47592_d_n19, assign30190_e47592_d_n20, assign30190_e47592_d_n21, assign30190_e47592_d_n22,) = {
    if ((locals.var_guard504 == 0.0) && (locals.var_guard513 != 0.0)) {
        (locals.var_vgdeff, locals.var_vgdeff_dn0, locals.var_vgdeff_dn1, locals.var_vgdeff_dn2, locals.var_vgdeff_dn3, locals.var_vgdeff_dn4, locals.var_vgdeff_dn5, locals.var_vgdeff_dn6, locals.var_vgdeff_dn7, locals.var_vgdeff_dn8, locals.var_vgdeff_dn9, locals.var_vgdeff_dn12, locals.var_vgdeff_dn14, locals.var_vgdeff_dn15, locals.var_vgdeff_dn16, locals.var_vgdeff_dn17, locals.var_vgdeff_dn18, locals.var_vgdeff_dn19, locals.var_vgdeff_dn20, locals.var_vgdeff_dn21, locals.var_vgdeff_dn22,)
    } else {
        (locals.var_vgod, locals.var_vgod_dn0, locals.var_vgod_dn1, locals.var_vgod_dn2, locals.var_vgod_dn3, locals.var_vgod_dn4, locals.var_vgod_dn5, locals.var_vgod_dn6, locals.var_vgod_dn7, locals.var_vgod_dn8, locals.var_vgod_dn9, locals.var_vgod_dn12, locals.var_vgod_dn14, locals.var_vgod_dn15, locals.var_vgod_dn16, locals.var_vgod_dn17, locals.var_vgod_dn18, locals.var_vgod_dn19, locals.var_vgod_dn20, locals.var_vgod_dn21, locals.var_vgod_dn22,)
    }
};
        locals.var_vgod = assign30190_e47592;
        locals.var_vgod_dn0 = assign30190_e47592_d_n0;
        locals.var_vgod_dn1 = assign30190_e47592_d_n1;
        locals.var_vgod_dn2 = assign30190_e47592_d_n2;
        locals.var_vgod_dn3 = assign30190_e47592_d_n3;
        locals.var_vgod_dn4 = assign30190_e47592_d_n4;
        locals.var_vgod_dn5 = assign30190_e47592_d_n5;
        locals.var_vgod_dn6 = assign30190_e47592_d_n6;
        locals.var_vgod_dn7 = assign30190_e47592_d_n7;
        locals.var_vgod_dn8 = assign30190_e47592_d_n8;
        locals.var_vgod_dn9 = assign30190_e47592_d_n9;
        locals.var_vgod_dn12 = assign30190_e47592_d_n12;
        locals.var_vgod_dn14 = assign30190_e47592_d_n14;
        locals.var_vgod_dn15 = assign30190_e47592_d_n15;
        locals.var_vgod_dn16 = assign30190_e47592_d_n16;
        locals.var_vgod_dn17 = assign30190_e47592_d_n17;
        locals.var_vgod_dn18 = assign30190_e47592_d_n18;
        locals.var_vgod_dn19 = assign30190_e47592_d_n19;
        locals.var_vgod_dn20 = assign30190_e47592_d_n20;
        locals.var_vgod_dn21 = assign30190_e47592_d_n21;
        locals.var_vgod_dn22 = assign30190_e47592_d_n22;

        let (assign30200_e47614, assign30200_e47614_d_n0, assign30200_e47614_d_n1, assign30200_e47614_d_n2, assign30200_e47614_d_n3, assign30200_e47614_d_n4, assign30200_e47614_d_n5, assign30200_e47614_d_n6, assign30200_e47614_d_n7, assign30200_e47614_d_n8, assign30200_e47614_d_n9, assign30200_e47614_d_n12, assign30200_e47614_d_n14, assign30200_e47614_d_n15, assign30200_e47614_d_n16, assign30200_e47614_d_n17, assign30200_e47614_d_n18, assign30200_e47614_d_n19, assign30200_e47614_d_n20, assign30200_e47614_d_n21, assign30200_e47614_d_n22,) = {
    if ((locals.var_guard504 == 0.0) && (locals.var_guard513 != 0.0)) {
        let assign30200_e47599: f64 = (0.5 * locals.var_vgod);
        let assign30200_e47603: f64 = (locals.var_vgod * locals.var_vgod);
        let assign30200_e47606: f64 = (4.0 * 0.3);
        let assign30200_e47608: f64 = (assign30200_e47606 * 0.3);
        let assign30200_e47609: f64 = (assign30200_e47603 + assign30200_e47608);
        let assign30200_e47610: f64 = (assign30200_e47609).sqrt();
        let assign30200_e47611: f64 = (0.5 * assign30200_e47610);
        let assign30200_e47612: f64 = (assign30200_e47599 + assign30200_e47611);
        (assign30200_e47612, ((0.5 * locals.var_vgod_dn0) + (0.5 * (((locals.var_vgod_dn0 * locals.var_vgod) + (locals.var_vgod * locals.var_vgod_dn0)) / (2.0 * assign30200_e47610)))), ((0.5 * locals.var_vgod_dn1) + (0.5 * (((locals.var_vgod_dn1 * locals.var_vgod) + (locals.var_vgod * locals.var_vgod_dn1)) / (2.0 * assign30200_e47610)))), ((0.5 * locals.var_vgod_dn2) + (0.5 * (((locals.var_vgod_dn2 * locals.var_vgod) + (locals.var_vgod * locals.var_vgod_dn2)) / (2.0 * assign30200_e47610)))), ((0.5 * locals.var_vgod_dn3) + (0.5 * (((locals.var_vgod_dn3 * locals.var_vgod) + (locals.var_vgod * locals.var_vgod_dn3)) / (2.0 * assign30200_e47610)))), ((0.5 * locals.var_vgod_dn4) + (0.5 * (((locals.var_vgod_dn4 * locals.var_vgod) + (locals.var_vgod * locals.var_vgod_dn4)) / (2.0 * assign30200_e47610)))), ((0.5 * locals.var_vgod_dn5) + (0.5 * (((locals.var_vgod_dn5 * locals.var_vgod) + (locals.var_vgod * locals.var_vgod_dn5)) / (2.0 * assign30200_e47610)))), ((0.5 * locals.var_vgod_dn6) + (0.5 * (((locals.var_vgod_dn6 * locals.var_vgod) + (locals.var_vgod * locals.var_vgod_dn6)) / (2.0 * assign30200_e47610)))), ((0.5 * locals.var_vgod_dn7) + (0.5 * (((locals.var_vgod_dn7 * locals.var_vgod) + (locals.var_vgod * locals.var_vgod_dn7)) / (2.0 * assign30200_e47610)))), ((0.5 * locals.var_vgod_dn8) + (0.5 * (((locals.var_vgod_dn8 * locals.var_vgod) + (locals.var_vgod * locals.var_vgod_dn8)) / (2.0 * assign30200_e47610)))), ((0.5 * locals.var_vgod_dn9) + (0.5 * (((locals.var_vgod_dn9 * locals.var_vgod) + (locals.var_vgod * locals.var_vgod_dn9)) / (2.0 * assign30200_e47610)))), ((0.5 * locals.var_vgod_dn12) + (0.5 * (((locals.var_vgod_dn12 * locals.var_vgod) + (locals.var_vgod * locals.var_vgod_dn12)) / (2.0 * assign30200_e47610)))), ((0.5 * locals.var_vgod_dn14) + (0.5 * (((locals.var_vgod_dn14 * locals.var_vgod) + (locals.var_vgod * locals.var_vgod_dn14)) / (2.0 * assign30200_e47610)))), ((0.5 * locals.var_vgod_dn15) + (0.5 * (((locals.var_vgod_dn15 * locals.var_vgod) + (locals.var_vgod * locals.var_vgod_dn15)) / (2.0 * assign30200_e47610)))), ((0.5 * locals.var_vgod_dn16) + (0.5 * (((locals.var_vgod_dn16 * locals.var_vgod) + (locals.var_vgod * locals.var_vgod_dn16)) / (2.0 * assign30200_e47610)))), ((0.5 * locals.var_vgod_dn17) + (0.5 * (((locals.var_vgod_dn17 * locals.var_vgod) + (locals.var_vgod * locals.var_vgod_dn17)) / (2.0 * assign30200_e47610)))), ((0.5 * locals.var_vgod_dn18) + (0.5 * (((locals.var_vgod_dn18 * locals.var_vgod) + (locals.var_vgod * locals.var_vgod_dn18)) / (2.0 * assign30200_e47610)))), ((0.5 * locals.var_vgod_dn19) + (0.5 * (((locals.var_vgod_dn19 * locals.var_vgod) + (locals.var_vgod * locals.var_vgod_dn19)) / (2.0 * assign30200_e47610)))), ((0.5 * locals.var_vgod_dn20) + (0.5 * (((locals.var_vgod_dn20 * locals.var_vgod) + (locals.var_vgod * locals.var_vgod_dn20)) / (2.0 * assign30200_e47610)))), ((0.5 * locals.var_vgod_dn21) + (0.5 * (((locals.var_vgod_dn21 * locals.var_vgod) + (locals.var_vgod * locals.var_vgod_dn21)) / (2.0 * assign30200_e47610)))), ((0.5 * locals.var_vgod_dn22) + (0.5 * (((locals.var_vgod_dn22 * locals.var_vgod) + (locals.var_vgod * locals.var_vgod_dn22)) / (2.0 * assign30200_e47610)))),)
    } else {
        (locals.var_vgodp, locals.var_vgodp_dn0, locals.var_vgodp_dn1, locals.var_vgodp_dn2, locals.var_vgodp_dn3, locals.var_vgodp_dn4, locals.var_vgodp_dn5, locals.var_vgodp_dn6, locals.var_vgodp_dn7, locals.var_vgodp_dn8, locals.var_vgodp_dn9, locals.var_vgodp_dn12, locals.var_vgodp_dn14, locals.var_vgodp_dn15, locals.var_vgodp_dn16, locals.var_vgodp_dn17, locals.var_vgodp_dn18, locals.var_vgodp_dn19, locals.var_vgodp_dn20, locals.var_vgodp_dn21, locals.var_vgodp_dn22,)
    }
};
        locals.var_vgodp = assign30200_e47614;
        locals.var_vgodp_dn0 = assign30200_e47614_d_n0;
        locals.var_vgodp_dn1 = assign30200_e47614_d_n1;
        locals.var_vgodp_dn2 = assign30200_e47614_d_n2;
        locals.var_vgodp_dn3 = assign30200_e47614_d_n3;
        locals.var_vgodp_dn4 = assign30200_e47614_d_n4;
        locals.var_vgodp_dn5 = assign30200_e47614_d_n5;
        locals.var_vgodp_dn6 = assign30200_e47614_d_n6;
        locals.var_vgodp_dn7 = assign30200_e47614_d_n7;
        locals.var_vgodp_dn8 = assign30200_e47614_d_n8;
        locals.var_vgodp_dn9 = assign30200_e47614_d_n9;
        locals.var_vgodp_dn12 = assign30200_e47614_d_n12;
        locals.var_vgodp_dn14 = assign30200_e47614_d_n14;
        locals.var_vgodp_dn15 = assign30200_e47614_d_n15;
        locals.var_vgodp_dn16 = assign30200_e47614_d_n16;
        locals.var_vgodp_dn17 = assign30200_e47614_d_n17;
        locals.var_vgodp_dn18 = assign30200_e47614_d_n18;
        locals.var_vgodp_dn19 = assign30200_e47614_d_n19;
        locals.var_vgodp_dn20 = assign30200_e47614_d_n20;
        locals.var_vgodp_dn21 = assign30200_e47614_d_n21;
        locals.var_vgodp_dn22 = assign30200_e47614_d_n22;

        let (assign30210_e47621, assign30210_e47621_d_n0, assign30210_e47621_d_n1, assign30210_e47621_d_n2, assign30210_e47621_d_n3, assign30210_e47621_d_n4, assign30210_e47621_d_n5, assign30210_e47621_d_n6, assign30210_e47621_d_n7, assign30210_e47621_d_n8, assign30210_e47621_d_n9, assign30210_e47621_d_n12, assign30210_e47621_d_n14, assign30210_e47621_d_n15, assign30210_e47621_d_n16, assign30210_e47621_d_n17, assign30210_e47621_d_n18, assign30210_e47621_d_n19, assign30210_e47621_d_n20, assign30210_e47621_d_n21, assign30210_e47621_d_n22,) = {
    if ((locals.var_guard504 == 0.0) && (locals.var_guard513 != 0.0)) {
        (locals.var_vgodp, locals.var_vgodp_dn0, locals.var_vgodp_dn1, locals.var_vgodp_dn2, locals.var_vgodp_dn3, locals.var_vgodp_dn4, locals.var_vgodp_dn5, locals.var_vgodp_dn6, locals.var_vgodp_dn7, locals.var_vgodp_dn8, locals.var_vgodp_dn9, locals.var_vgodp_dn12, locals.var_vgodp_dn14, locals.var_vgodp_dn15, locals.var_vgodp_dn16, locals.var_vgodp_dn17, locals.var_vgodp_dn18, locals.var_vgodp_dn19, locals.var_vgodp_dn20, locals.var_vgodp_dn21, locals.var_vgodp_dn22,)
    } else {
        (locals.var_vgop, locals.var_vgop_dn0, locals.var_vgop_dn1, locals.var_vgop_dn2, locals.var_vgop_dn3, locals.var_vgop_dn4, locals.var_vgop_dn5, locals.var_vgop_dn6, locals.var_vgop_dn7, locals.var_vgop_dn8, locals.var_vgop_dn9, locals.var_vgop_dn12, locals.var_vgop_dn14, locals.var_vgop_dn15, locals.var_vgop_dn16, locals.var_vgop_dn17, locals.var_vgop_dn18, locals.var_vgop_dn19, locals.var_vgop_dn20, locals.var_vgop_dn21, locals.var_vgop_dn22,)
    }
};
        locals.var_vgop = assign30210_e47621;
        locals.var_vgop_dn0 = assign30210_e47621_d_n0;
        locals.var_vgop_dn1 = assign30210_e47621_d_n1;
        locals.var_vgop_dn2 = assign30210_e47621_d_n2;
        locals.var_vgop_dn3 = assign30210_e47621_d_n3;
        locals.var_vgop_dn4 = assign30210_e47621_d_n4;
        locals.var_vgop_dn5 = assign30210_e47621_d_n5;
        locals.var_vgop_dn6 = assign30210_e47621_d_n6;
        locals.var_vgop_dn7 = assign30210_e47621_d_n7;
        locals.var_vgop_dn8 = assign30210_e47621_d_n8;
        locals.var_vgop_dn9 = assign30210_e47621_d_n9;
        locals.var_vgop_dn12 = assign30210_e47621_d_n12;
        locals.var_vgop_dn14 = assign30210_e47621_d_n14;
        locals.var_vgop_dn15 = assign30210_e47621_d_n15;
        locals.var_vgop_dn16 = assign30210_e47621_d_n16;
        locals.var_vgop_dn17 = assign30210_e47621_d_n17;
        locals.var_vgop_dn18 = assign30210_e47621_d_n18;
        locals.var_vgop_dn19 = assign30210_e47621_d_n19;
        locals.var_vgop_dn20 = assign30210_e47621_d_n20;
        locals.var_vgop_dn21 = assign30210_e47621_d_n21;
        locals.var_vgop_dn22 = assign30210_e47621_d_n22;

        let (assign30220_e47639, assign30220_e47639_d_n0, assign30220_e47639_d_n1, assign30220_e47639_d_n2, assign30220_e47639_d_n3, assign30220_e47639_d_n4, assign30220_e47639_d_n5, assign30220_e47639_d_n6, assign30220_e47639_d_n7, assign30220_e47639_d_n8, assign30220_e47639_d_n9, assign30220_e47639_d_n12, assign30220_e47639_d_n14, assign30220_e47639_d_n15, assign30220_e47639_d_n16, assign30220_e47639_d_n17, assign30220_e47639_d_n18, assign30220_e47639_d_n19, assign30220_e47639_d_n20, assign30220_e47639_d_n21, assign30220_e47639_d_n22,) = {
    if ((locals.var_guard504 == 0.0) && (locals.var_guard513 != 0.0)) {
        let assign30220_e47628: f64 = (locals.var_vgop * locals.var_alphan);
        let assign30220_e47631: f64 = (locals.var_vgop * locals.var_vgop);
        let assign30220_e47634: f64 = (locals.var_alphan * locals.var_alphan);
        let assign30220_e47635: f64 = (assign30220_e47631 + assign30220_e47634);
        let assign30220_e47636: f64 = (assign30220_e47635).sqrt();
        let assign30220_e47637: f64 = (assign30220_e47628 / assign30220_e47636);
        (assign30220_e47637, ((((locals.var_vgop_dn0 * locals.var_alphan) * assign30220_e47636) - (assign30220_e47628 * (((locals.var_vgop_dn0 * locals.var_vgop) + (locals.var_vgop * locals.var_vgop_dn0)) / (2.0 * assign30220_e47636)))) / (assign30220_e47636 * assign30220_e47636)), ((((locals.var_vgop_dn1 * locals.var_alphan) * assign30220_e47636) - (assign30220_e47628 * (((locals.var_vgop_dn1 * locals.var_vgop) + (locals.var_vgop * locals.var_vgop_dn1)) / (2.0 * assign30220_e47636)))) / (assign30220_e47636 * assign30220_e47636)), ((((locals.var_vgop_dn2 * locals.var_alphan) * assign30220_e47636) - (assign30220_e47628 * (((locals.var_vgop_dn2 * locals.var_vgop) + (locals.var_vgop * locals.var_vgop_dn2)) / (2.0 * assign30220_e47636)))) / (assign30220_e47636 * assign30220_e47636)), ((((locals.var_vgop_dn3 * locals.var_alphan) * assign30220_e47636) - (assign30220_e47628 * (((locals.var_vgop_dn3 * locals.var_vgop) + (locals.var_vgop * locals.var_vgop_dn3)) / (2.0 * assign30220_e47636)))) / (assign30220_e47636 * assign30220_e47636)), (((((locals.var_vgop_dn4 * locals.var_alphan) + (locals.var_vgop * locals.var_alphan_dn4)) * assign30220_e47636) - (assign30220_e47628 * ((((locals.var_vgop_dn4 * locals.var_vgop) + (locals.var_vgop * locals.var_vgop_dn4)) + ((locals.var_alphan_dn4 * locals.var_alphan) + (locals.var_alphan * locals.var_alphan_dn4))) / (2.0 * assign30220_e47636)))) / (assign30220_e47636 * assign30220_e47636)), ((((locals.var_vgop_dn5 * locals.var_alphan) * assign30220_e47636) - (assign30220_e47628 * (((locals.var_vgop_dn5 * locals.var_vgop) + (locals.var_vgop * locals.var_vgop_dn5)) / (2.0 * assign30220_e47636)))) / (assign30220_e47636 * assign30220_e47636)), (((((locals.var_vgop_dn6 * locals.var_alphan) + (locals.var_vgop * locals.var_alphan_dn6)) * assign30220_e47636) - (assign30220_e47628 * ((((locals.var_vgop_dn6 * locals.var_vgop) + (locals.var_vgop * locals.var_vgop_dn6)) + ((locals.var_alphan_dn6 * locals.var_alphan) + (locals.var_alphan * locals.var_alphan_dn6))) / (2.0 * assign30220_e47636)))) / (assign30220_e47636 * assign30220_e47636)), (((((locals.var_vgop_dn7 * locals.var_alphan) + (locals.var_vgop * locals.var_alphan_dn7)) * assign30220_e47636) - (assign30220_e47628 * ((((locals.var_vgop_dn7 * locals.var_vgop) + (locals.var_vgop * locals.var_vgop_dn7)) + ((locals.var_alphan_dn7 * locals.var_alphan) + (locals.var_alphan * locals.var_alphan_dn7))) / (2.0 * assign30220_e47636)))) / (assign30220_e47636 * assign30220_e47636)), (((((locals.var_vgop_dn8 * locals.var_alphan) + (locals.var_vgop * locals.var_alphan_dn8)) * assign30220_e47636) - (assign30220_e47628 * ((((locals.var_vgop_dn8 * locals.var_vgop) + (locals.var_vgop * locals.var_vgop_dn8)) + ((locals.var_alphan_dn8 * locals.var_alphan) + (locals.var_alphan * locals.var_alphan_dn8))) / (2.0 * assign30220_e47636)))) / (assign30220_e47636 * assign30220_e47636)), ((((locals.var_vgop_dn9 * locals.var_alphan) * assign30220_e47636) - (assign30220_e47628 * (((locals.var_vgop_dn9 * locals.var_vgop) + (locals.var_vgop * locals.var_vgop_dn9)) / (2.0 * assign30220_e47636)))) / (assign30220_e47636 * assign30220_e47636)), ((((locals.var_vgop_dn12 * locals.var_alphan) * assign30220_e47636) - (assign30220_e47628 * (((locals.var_vgop_dn12 * locals.var_vgop) + (locals.var_vgop * locals.var_vgop_dn12)) / (2.0 * assign30220_e47636)))) / (assign30220_e47636 * assign30220_e47636)), ((((locals.var_vgop_dn14 * locals.var_alphan) * assign30220_e47636) - (assign30220_e47628 * (((locals.var_vgop_dn14 * locals.var_vgop) + (locals.var_vgop * locals.var_vgop_dn14)) / (2.0 * assign30220_e47636)))) / (assign30220_e47636 * assign30220_e47636)), (((((locals.var_vgop_dn15 * locals.var_alphan) + (locals.var_vgop * locals.var_alphan_dn15)) * assign30220_e47636) - (assign30220_e47628 * ((((locals.var_vgop_dn15 * locals.var_vgop) + (locals.var_vgop * locals.var_vgop_dn15)) + ((locals.var_alphan_dn15 * locals.var_alphan) + (locals.var_alphan * locals.var_alphan_dn15))) / (2.0 * assign30220_e47636)))) / (assign30220_e47636 * assign30220_e47636)), (((((locals.var_vgop_dn16 * locals.var_alphan) + (locals.var_vgop * locals.var_alphan_dn16)) * assign30220_e47636) - (assign30220_e47628 * ((((locals.var_vgop_dn16 * locals.var_vgop) + (locals.var_vgop * locals.var_vgop_dn16)) + ((locals.var_alphan_dn16 * locals.var_alphan) + (locals.var_alphan * locals.var_alphan_dn16))) / (2.0 * assign30220_e47636)))) / (assign30220_e47636 * assign30220_e47636)), (((((locals.var_vgop_dn17 * locals.var_alphan) + (locals.var_vgop * locals.var_alphan_dn17)) * assign30220_e47636) - (assign30220_e47628 * ((((locals.var_vgop_dn17 * locals.var_vgop) + (locals.var_vgop * locals.var_vgop_dn17)) + ((locals.var_alphan_dn17 * locals.var_alphan) + (locals.var_alphan * locals.var_alphan_dn17))) / (2.0 * assign30220_e47636)))) / (assign30220_e47636 * assign30220_e47636)), (((((locals.var_vgop_dn18 * locals.var_alphan) + (locals.var_vgop * locals.var_alphan_dn18)) * assign30220_e47636) - (assign30220_e47628 * ((((locals.var_vgop_dn18 * locals.var_vgop) + (locals.var_vgop * locals.var_vgop_dn18)) + ((locals.var_alphan_dn18 * locals.var_alphan) + (locals.var_alphan * locals.var_alphan_dn18))) / (2.0 * assign30220_e47636)))) / (assign30220_e47636 * assign30220_e47636)), (((((locals.var_vgop_dn19 * locals.var_alphan) + (locals.var_vgop * locals.var_alphan_dn19)) * assign30220_e47636) - (assign30220_e47628 * ((((locals.var_vgop_dn19 * locals.var_vgop) + (locals.var_vgop * locals.var_vgop_dn19)) + ((locals.var_alphan_dn19 * locals.var_alphan) + (locals.var_alphan * locals.var_alphan_dn19))) / (2.0 * assign30220_e47636)))) / (assign30220_e47636 * assign30220_e47636)), (((((locals.var_vgop_dn20 * locals.var_alphan) + (locals.var_vgop * locals.var_alphan_dn20)) * assign30220_e47636) - (assign30220_e47628 * ((((locals.var_vgop_dn20 * locals.var_vgop) + (locals.var_vgop * locals.var_vgop_dn20)) + ((locals.var_alphan_dn20 * locals.var_alphan) + (locals.var_alphan * locals.var_alphan_dn20))) / (2.0 * assign30220_e47636)))) / (assign30220_e47636 * assign30220_e47636)), (((((locals.var_vgop_dn21 * locals.var_alphan) + (locals.var_vgop * locals.var_alphan_dn21)) * assign30220_e47636) - (assign30220_e47628 * ((((locals.var_vgop_dn21 * locals.var_vgop) + (locals.var_vgop * locals.var_vgop_dn21)) + ((locals.var_alphan_dn21 * locals.var_alphan) + (locals.var_alphan * locals.var_alphan_dn21))) / (2.0 * assign30220_e47636)))) / (assign30220_e47636 * assign30220_e47636)), (((((locals.var_vgop_dn22 * locals.var_alphan) + (locals.var_vgop * locals.var_alphan_dn22)) * assign30220_e47636) - (assign30220_e47628 * ((((locals.var_vgop_dn22 * locals.var_vgop) + (locals.var_vgop * locals.var_vgop_dn22)) + ((locals.var_alphan_dn22 * locals.var_alphan) + (locals.var_alphan * locals.var_alphan_dn22))) / (2.0 * assign30220_e47636)))) / (assign30220_e47636 * assign30220_e47636)),)
    } else {
        (locals.var_dvgon, locals.var_dvgon_dn0, locals.var_dvgon_dn1, locals.var_dvgon_dn2, locals.var_dvgon_dn3, locals.var_dvgon_dn4, locals.var_dvgon_dn5, locals.var_dvgon_dn6, locals.var_dvgon_dn7, locals.var_dvgon_dn8, locals.var_dvgon_dn9, locals.var_dvgon_dn12, locals.var_dvgon_dn14, locals.var_dvgon_dn15, locals.var_dvgon_dn16, locals.var_dvgon_dn17, locals.var_dvgon_dn18, locals.var_dvgon_dn19, locals.var_dvgon_dn20, locals.var_dvgon_dn21, locals.var_dvgon_dn22,)
    }
};
        locals.var_dvgon = assign30220_e47639;
        locals.var_dvgon_dn0 = assign30220_e47639_d_n0;
        locals.var_dvgon_dn1 = assign30220_e47639_d_n1;
        locals.var_dvgon_dn2 = assign30220_e47639_d_n2;
        locals.var_dvgon_dn3 = assign30220_e47639_d_n3;
        locals.var_dvgon_dn4 = assign30220_e47639_d_n4;
        locals.var_dvgon_dn5 = assign30220_e47639_d_n5;
        locals.var_dvgon_dn6 = assign30220_e47639_d_n6;
        locals.var_dvgon_dn7 = assign30220_e47639_d_n7;
        locals.var_dvgon_dn8 = assign30220_e47639_d_n8;
        locals.var_dvgon_dn9 = assign30220_e47639_d_n9;
        locals.var_dvgon_dn12 = assign30220_e47639_d_n12;
        locals.var_dvgon_dn14 = assign30220_e47639_d_n14;
        locals.var_dvgon_dn15 = assign30220_e47639_d_n15;
        locals.var_dvgon_dn16 = assign30220_e47639_d_n16;
        locals.var_dvgon_dn17 = assign30220_e47639_d_n17;
        locals.var_dvgon_dn18 = assign30220_e47639_d_n18;
        locals.var_dvgon_dn19 = assign30220_e47639_d_n19;
        locals.var_dvgon_dn20 = assign30220_e47639_d_n20;
        locals.var_dvgon_dn21 = assign30220_e47639_d_n21;
        locals.var_dvgon_dn22 = assign30220_e47639_d_n22;

        let (assign30230_e47657, assign30230_e47657_d_n0, assign30230_e47657_d_n1, assign30230_e47657_d_n2, assign30230_e47657_d_n3, assign30230_e47657_d_n4, assign30230_e47657_d_n5, assign30230_e47657_d_n6, assign30230_e47657_d_n7, assign30230_e47657_d_n8, assign30230_e47657_d_n9, assign30230_e47657_d_n12, assign30230_e47657_d_n14, assign30230_e47657_d_n15, assign30230_e47657_d_n16, assign30230_e47657_d_n17, assign30230_e47657_d_n18, assign30230_e47657_d_n19, assign30230_e47657_d_n20, assign30230_e47657_d_n21, assign30230_e47657_d_n22,) = {
    if ((locals.var_guard504 == 0.0) && (locals.var_guard513 != 0.0)) {
        let assign30230_e47646: f64 = (locals.var_vgop * locals.var_alphad);
        let assign30230_e47649: f64 = (locals.var_vgop * locals.var_vgop);
        let assign30230_e47652: f64 = (locals.var_alphad * locals.var_alphad);
        let assign30230_e47653: f64 = (assign30230_e47649 + assign30230_e47652);
        let assign30230_e47654: f64 = (assign30230_e47653).sqrt();
        let assign30230_e47655: f64 = (assign30230_e47646 / assign30230_e47654);
        (assign30230_e47655, ((((locals.var_vgop_dn0 * locals.var_alphad) * assign30230_e47654) - (assign30230_e47646 * (((locals.var_vgop_dn0 * locals.var_vgop) + (locals.var_vgop * locals.var_vgop_dn0)) / (2.0 * assign30230_e47654)))) / (assign30230_e47654 * assign30230_e47654)), ((((locals.var_vgop_dn1 * locals.var_alphad) * assign30230_e47654) - (assign30230_e47646 * (((locals.var_vgop_dn1 * locals.var_vgop) + (locals.var_vgop * locals.var_vgop_dn1)) / (2.0 * assign30230_e47654)))) / (assign30230_e47654 * assign30230_e47654)), ((((locals.var_vgop_dn2 * locals.var_alphad) * assign30230_e47654) - (assign30230_e47646 * (((locals.var_vgop_dn2 * locals.var_vgop) + (locals.var_vgop * locals.var_vgop_dn2)) / (2.0 * assign30230_e47654)))) / (assign30230_e47654 * assign30230_e47654)), ((((locals.var_vgop_dn3 * locals.var_alphad) * assign30230_e47654) - (assign30230_e47646 * (((locals.var_vgop_dn3 * locals.var_vgop) + (locals.var_vgop * locals.var_vgop_dn3)) / (2.0 * assign30230_e47654)))) / (assign30230_e47654 * assign30230_e47654)), (((((locals.var_vgop_dn4 * locals.var_alphad) + (locals.var_vgop * locals.var_alphad_dn4)) * assign30230_e47654) - (assign30230_e47646 * ((((locals.var_vgop_dn4 * locals.var_vgop) + (locals.var_vgop * locals.var_vgop_dn4)) + ((locals.var_alphad_dn4 * locals.var_alphad) + (locals.var_alphad * locals.var_alphad_dn4))) / (2.0 * assign30230_e47654)))) / (assign30230_e47654 * assign30230_e47654)), ((((locals.var_vgop_dn5 * locals.var_alphad) * assign30230_e47654) - (assign30230_e47646 * (((locals.var_vgop_dn5 * locals.var_vgop) + (locals.var_vgop * locals.var_vgop_dn5)) / (2.0 * assign30230_e47654)))) / (assign30230_e47654 * assign30230_e47654)), (((((locals.var_vgop_dn6 * locals.var_alphad) + (locals.var_vgop * locals.var_alphad_dn6)) * assign30230_e47654) - (assign30230_e47646 * ((((locals.var_vgop_dn6 * locals.var_vgop) + (locals.var_vgop * locals.var_vgop_dn6)) + ((locals.var_alphad_dn6 * locals.var_alphad) + (locals.var_alphad * locals.var_alphad_dn6))) / (2.0 * assign30230_e47654)))) / (assign30230_e47654 * assign30230_e47654)), (((((locals.var_vgop_dn7 * locals.var_alphad) + (locals.var_vgop * locals.var_alphad_dn7)) * assign30230_e47654) - (assign30230_e47646 * ((((locals.var_vgop_dn7 * locals.var_vgop) + (locals.var_vgop * locals.var_vgop_dn7)) + ((locals.var_alphad_dn7 * locals.var_alphad) + (locals.var_alphad * locals.var_alphad_dn7))) / (2.0 * assign30230_e47654)))) / (assign30230_e47654 * assign30230_e47654)), (((((locals.var_vgop_dn8 * locals.var_alphad) + (locals.var_vgop * locals.var_alphad_dn8)) * assign30230_e47654) - (assign30230_e47646 * ((((locals.var_vgop_dn8 * locals.var_vgop) + (locals.var_vgop * locals.var_vgop_dn8)) + ((locals.var_alphad_dn8 * locals.var_alphad) + (locals.var_alphad * locals.var_alphad_dn8))) / (2.0 * assign30230_e47654)))) / (assign30230_e47654 * assign30230_e47654)), ((((locals.var_vgop_dn9 * locals.var_alphad) * assign30230_e47654) - (assign30230_e47646 * (((locals.var_vgop_dn9 * locals.var_vgop) + (locals.var_vgop * locals.var_vgop_dn9)) / (2.0 * assign30230_e47654)))) / (assign30230_e47654 * assign30230_e47654)), ((((locals.var_vgop_dn12 * locals.var_alphad) * assign30230_e47654) - (assign30230_e47646 * (((locals.var_vgop_dn12 * locals.var_vgop) + (locals.var_vgop * locals.var_vgop_dn12)) / (2.0 * assign30230_e47654)))) / (assign30230_e47654 * assign30230_e47654)), ((((locals.var_vgop_dn14 * locals.var_alphad) * assign30230_e47654) - (assign30230_e47646 * (((locals.var_vgop_dn14 * locals.var_vgop) + (locals.var_vgop * locals.var_vgop_dn14)) / (2.0 * assign30230_e47654)))) / (assign30230_e47654 * assign30230_e47654)), (((((locals.var_vgop_dn15 * locals.var_alphad) + (locals.var_vgop * locals.var_alphad_dn15)) * assign30230_e47654) - (assign30230_e47646 * ((((locals.var_vgop_dn15 * locals.var_vgop) + (locals.var_vgop * locals.var_vgop_dn15)) + ((locals.var_alphad_dn15 * locals.var_alphad) + (locals.var_alphad * locals.var_alphad_dn15))) / (2.0 * assign30230_e47654)))) / (assign30230_e47654 * assign30230_e47654)), (((((locals.var_vgop_dn16 * locals.var_alphad) + (locals.var_vgop * locals.var_alphad_dn16)) * assign30230_e47654) - (assign30230_e47646 * ((((locals.var_vgop_dn16 * locals.var_vgop) + (locals.var_vgop * locals.var_vgop_dn16)) + ((locals.var_alphad_dn16 * locals.var_alphad) + (locals.var_alphad * locals.var_alphad_dn16))) / (2.0 * assign30230_e47654)))) / (assign30230_e47654 * assign30230_e47654)), (((((locals.var_vgop_dn17 * locals.var_alphad) + (locals.var_vgop * locals.var_alphad_dn17)) * assign30230_e47654) - (assign30230_e47646 * ((((locals.var_vgop_dn17 * locals.var_vgop) + (locals.var_vgop * locals.var_vgop_dn17)) + ((locals.var_alphad_dn17 * locals.var_alphad) + (locals.var_alphad * locals.var_alphad_dn17))) / (2.0 * assign30230_e47654)))) / (assign30230_e47654 * assign30230_e47654)), (((((locals.var_vgop_dn18 * locals.var_alphad) + (locals.var_vgop * locals.var_alphad_dn18)) * assign30230_e47654) - (assign30230_e47646 * ((((locals.var_vgop_dn18 * locals.var_vgop) + (locals.var_vgop * locals.var_vgop_dn18)) + ((locals.var_alphad_dn18 * locals.var_alphad) + (locals.var_alphad * locals.var_alphad_dn18))) / (2.0 * assign30230_e47654)))) / (assign30230_e47654 * assign30230_e47654)), (((((locals.var_vgop_dn19 * locals.var_alphad) + (locals.var_vgop * locals.var_alphad_dn19)) * assign30230_e47654) - (assign30230_e47646 * ((((locals.var_vgop_dn19 * locals.var_vgop) + (locals.var_vgop * locals.var_vgop_dn19)) + ((locals.var_alphad_dn19 * locals.var_alphad) + (locals.var_alphad * locals.var_alphad_dn19))) / (2.0 * assign30230_e47654)))) / (assign30230_e47654 * assign30230_e47654)), (((((locals.var_vgop_dn20 * locals.var_alphad) + (locals.var_vgop * locals.var_alphad_dn20)) * assign30230_e47654) - (assign30230_e47646 * ((((locals.var_vgop_dn20 * locals.var_vgop) + (locals.var_vgop * locals.var_vgop_dn20)) + ((locals.var_alphad_dn20 * locals.var_alphad) + (locals.var_alphad * locals.var_alphad_dn20))) / (2.0 * assign30230_e47654)))) / (assign30230_e47654 * assign30230_e47654)), (((((locals.var_vgop_dn21 * locals.var_alphad) + (locals.var_vgop * locals.var_alphad_dn21)) * assign30230_e47654) - (assign30230_e47646 * ((((locals.var_vgop_dn21 * locals.var_vgop) + (locals.var_vgop * locals.var_vgop_dn21)) + ((locals.var_alphad_dn21 * locals.var_alphad) + (locals.var_alphad * locals.var_alphad_dn21))) / (2.0 * assign30230_e47654)))) / (assign30230_e47654 * assign30230_e47654)), (((((locals.var_vgop_dn22 * locals.var_alphad) + (locals.var_vgop * locals.var_alphad_dn22)) * assign30230_e47654) - (assign30230_e47646 * ((((locals.var_vgop_dn22 * locals.var_vgop) + (locals.var_vgop * locals.var_vgop_dn22)) + ((locals.var_alphad_dn22 * locals.var_alphad) + (locals.var_alphad * locals.var_alphad_dn22))) / (2.0 * assign30230_e47654)))) / (assign30230_e47654 * assign30230_e47654)),)
    } else {
        (locals.var_dvgod, locals.var_dvgod_dn0, locals.var_dvgod_dn1, locals.var_dvgod_dn2, locals.var_dvgod_dn3, locals.var_dvgod_dn4, locals.var_dvgod_dn5, locals.var_dvgod_dn6, locals.var_dvgod_dn7, locals.var_dvgod_dn8, locals.var_dvgod_dn9, locals.var_dvgod_dn12, locals.var_dvgod_dn14, locals.var_dvgod_dn15, locals.var_dvgod_dn16, locals.var_dvgod_dn17, locals.var_dvgod_dn18, locals.var_dvgod_dn19, locals.var_dvgod_dn20, locals.var_dvgod_dn21, locals.var_dvgod_dn22,)
    }
};
        locals.var_dvgod = assign30230_e47657;
        locals.var_dvgod_dn0 = assign30230_e47657_d_n0;
        locals.var_dvgod_dn1 = assign30230_e47657_d_n1;
        locals.var_dvgod_dn2 = assign30230_e47657_d_n2;
        locals.var_dvgod_dn3 = assign30230_e47657_d_n3;
        locals.var_dvgod_dn4 = assign30230_e47657_d_n4;
        locals.var_dvgod_dn5 = assign30230_e47657_d_n5;
        locals.var_dvgod_dn6 = assign30230_e47657_d_n6;
        locals.var_dvgod_dn7 = assign30230_e47657_d_n7;
        locals.var_dvgod_dn8 = assign30230_e47657_d_n8;
        locals.var_dvgod_dn9 = assign30230_e47657_d_n9;
        locals.var_dvgod_dn12 = assign30230_e47657_d_n12;
        locals.var_dvgod_dn14 = assign30230_e47657_d_n14;
        locals.var_dvgod_dn15 = assign30230_e47657_d_n15;
        locals.var_dvgod_dn16 = assign30230_e47657_d_n16;
        locals.var_dvgod_dn17 = assign30230_e47657_d_n17;
        locals.var_dvgod_dn18 = assign30230_e47657_d_n18;
        locals.var_dvgod_dn19 = assign30230_e47657_d_n19;
        locals.var_dvgod_dn20 = assign30230_e47657_d_n20;
        locals.var_dvgod_dn21 = assign30230_e47657_d_n21;
        locals.var_dvgod_dn22 = assign30230_e47657_d_n22;

        let (assign30240_e47703, assign30240_e47703_d_n0, assign30240_e47703_d_n1, assign30240_e47703_d_n2, assign30240_e47703_d_n3, assign30240_e47703_d_n4, assign30240_e47703_d_n5, assign30240_e47703_d_n6, assign30240_e47703_d_n7, assign30240_e47703_d_n8, assign30240_e47703_d_n9, assign30240_e47703_d_n12, assign30240_e47703_d_n14, assign30240_e47703_d_n15, assign30240_e47703_d_n16, assign30240_e47703_d_n17, assign30240_e47703_d_n18, assign30240_e47703_d_n19, assign30240_e47703_d_n20, assign30240_e47703_d_n21, assign30240_e47703_d_n22,) = {
    if ((locals.var_guard504 == 0.0) && (locals.var_guard513 != 0.0)) {
        let assign30240_e47667: f64 = (locals.var_beta * locals.var_dvgon);
        let assign30240_e47668: f64 = (assign30240_e47667).ln();
        let assign30240_e47669: f64 = (1.0 - assign30240_e47668);
        let assign30240_e47670: f64 = (locals.var_vtv * assign30240_e47669);
        let assign30240_e47671: f64 = (locals.var_vgop + assign30240_e47670);
        let assign30240_e47674: f64 = (p.p208 / 3.0);
        let assign30240_e47677: f64 = (locals.var_cch * locals.var_vgop);
        let assign30240_e47679: f64 = (assign30240_e47677).powf(0.6666666666666666);
        let assign30240_e47680: f64 = (assign30240_e47674 * assign30240_e47679);
        let assign30240_e47681: f64 = (assign30240_e47671 - assign30240_e47680);
        let assign30240_e47686: f64 = (locals.var_vtv / locals.var_dvgod);
        let assign30240_e47687: f64 = (1.0 + assign30240_e47686);
        let assign30240_e47688: f64 = (locals.var_vgop * assign30240_e47687);
        let assign30240_e47691: f64 = (2.0 * p.p208);
        let assign30240_e47693: f64 = (assign30240_e47691 / 3.0);
        let assign30240_e47696: f64 = (locals.var_cch * locals.var_vgop);
        let assign30240_e47698: f64 = (assign30240_e47696).powf(0.6666666666666666);
        let assign30240_e47699: f64 = (assign30240_e47693 * assign30240_e47698);
        let assign30240_e47700: f64 = (assign30240_e47688 + assign30240_e47699);
        let assign30240_e47701: f64 = (assign30240_e47681 / assign30240_e47700);
        (assign30240_e47701, (((((locals.var_vgop_dn0 + (locals.var_vtv * (-((locals.var_beta * locals.var_dvgon_dn0) / assign30240_e47667)))) - (assign30240_e47674 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((assign30240_e47677).powf(0.6666666666666666 - 1.0) * (locals.var_cch * locals.var_vgop_dn0))) } } else { (assign30240_e47679 * (0.6666666666666666 * ((locals.var_cch * locals.var_vgop_dn0) / assign30240_e47677))) })) * assign30240_e47700) - (assign30240_e47681 * (((locals.var_vgop_dn0 * assign30240_e47687) + (locals.var_vgop * (-((locals.var_vtv * locals.var_dvgod_dn0) / (locals.var_dvgod * locals.var_dvgod))))) + (assign30240_e47693 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((assign30240_e47696).powf(0.6666666666666666 - 1.0) * (locals.var_cch * locals.var_vgop_dn0))) } } else { (assign30240_e47698 * (0.6666666666666666 * ((locals.var_cch * locals.var_vgop_dn0) / assign30240_e47696))) })))) / (assign30240_e47700 * assign30240_e47700)), (((((locals.var_vgop_dn1 + (locals.var_vtv * (-((locals.var_beta * locals.var_dvgon_dn1) / assign30240_e47667)))) - (assign30240_e47674 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((assign30240_e47677).powf(0.6666666666666666 - 1.0) * (locals.var_cch * locals.var_vgop_dn1))) } } else { (assign30240_e47679 * (0.6666666666666666 * ((locals.var_cch * locals.var_vgop_dn1) / assign30240_e47677))) })) * assign30240_e47700) - (assign30240_e47681 * (((locals.var_vgop_dn1 * assign30240_e47687) + (locals.var_vgop * (-((locals.var_vtv * locals.var_dvgod_dn1) / (locals.var_dvgod * locals.var_dvgod))))) + (assign30240_e47693 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((assign30240_e47696).powf(0.6666666666666666 - 1.0) * (locals.var_cch * locals.var_vgop_dn1))) } } else { (assign30240_e47698 * (0.6666666666666666 * ((locals.var_cch * locals.var_vgop_dn1) / assign30240_e47696))) })))) / (assign30240_e47700 * assign30240_e47700)), (((((locals.var_vgop_dn2 + (locals.var_vtv * (-((locals.var_beta * locals.var_dvgon_dn2) / assign30240_e47667)))) - (assign30240_e47674 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((assign30240_e47677).powf(0.6666666666666666 - 1.0) * (locals.var_cch * locals.var_vgop_dn2))) } } else { (assign30240_e47679 * (0.6666666666666666 * ((locals.var_cch * locals.var_vgop_dn2) / assign30240_e47677))) })) * assign30240_e47700) - (assign30240_e47681 * (((locals.var_vgop_dn2 * assign30240_e47687) + (locals.var_vgop * (-((locals.var_vtv * locals.var_dvgod_dn2) / (locals.var_dvgod * locals.var_dvgod))))) + (assign30240_e47693 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((assign30240_e47696).powf(0.6666666666666666 - 1.0) * (locals.var_cch * locals.var_vgop_dn2))) } } else { (assign30240_e47698 * (0.6666666666666666 * ((locals.var_cch * locals.var_vgop_dn2) / assign30240_e47696))) })))) / (assign30240_e47700 * assign30240_e47700)), (((((locals.var_vgop_dn3 + (locals.var_vtv * (-((locals.var_beta * locals.var_dvgon_dn3) / assign30240_e47667)))) - (assign30240_e47674 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((assign30240_e47677).powf(0.6666666666666666 - 1.0) * (locals.var_cch * locals.var_vgop_dn3))) } } else { (assign30240_e47679 * (0.6666666666666666 * ((locals.var_cch * locals.var_vgop_dn3) / assign30240_e47677))) })) * assign30240_e47700) - (assign30240_e47681 * (((locals.var_vgop_dn3 * assign30240_e47687) + (locals.var_vgop * (-((locals.var_vtv * locals.var_dvgod_dn3) / (locals.var_dvgod * locals.var_dvgod))))) + (assign30240_e47693 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((assign30240_e47696).powf(0.6666666666666666 - 1.0) * (locals.var_cch * locals.var_vgop_dn3))) } } else { (assign30240_e47698 * (0.6666666666666666 * ((locals.var_cch * locals.var_vgop_dn3) / assign30240_e47696))) })))) / (assign30240_e47700 * assign30240_e47700)), (((((locals.var_vgop_dn4 + ((locals.var_vtv_dn4 * assign30240_e47669) + (locals.var_vtv * (-(((locals.var_beta_dn4 * locals.var_dvgon) + (locals.var_beta * locals.var_dvgon_dn4)) / assign30240_e47667))))) - (assign30240_e47674 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((assign30240_e47677).powf(0.6666666666666666 - 1.0) * (locals.var_cch * locals.var_vgop_dn4))) } } else { (assign30240_e47679 * (0.6666666666666666 * ((locals.var_cch * locals.var_vgop_dn4) / assign30240_e47677))) })) * assign30240_e47700) - (assign30240_e47681 * (((locals.var_vgop_dn4 * assign30240_e47687) + (locals.var_vgop * (((locals.var_vtv_dn4 * locals.var_dvgod) - (locals.var_vtv * locals.var_dvgod_dn4)) / (locals.var_dvgod * locals.var_dvgod)))) + (assign30240_e47693 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((assign30240_e47696).powf(0.6666666666666666 - 1.0) * (locals.var_cch * locals.var_vgop_dn4))) } } else { (assign30240_e47698 * (0.6666666666666666 * ((locals.var_cch * locals.var_vgop_dn4) / assign30240_e47696))) })))) / (assign30240_e47700 * assign30240_e47700)), (((((locals.var_vgop_dn5 + (locals.var_vtv * (-((locals.var_beta * locals.var_dvgon_dn5) / assign30240_e47667)))) - (assign30240_e47674 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((assign30240_e47677).powf(0.6666666666666666 - 1.0) * (locals.var_cch * locals.var_vgop_dn5))) } } else { (assign30240_e47679 * (0.6666666666666666 * ((locals.var_cch * locals.var_vgop_dn5) / assign30240_e47677))) })) * assign30240_e47700) - (assign30240_e47681 * (((locals.var_vgop_dn5 * assign30240_e47687) + (locals.var_vgop * (-((locals.var_vtv * locals.var_dvgod_dn5) / (locals.var_dvgod * locals.var_dvgod))))) + (assign30240_e47693 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((assign30240_e47696).powf(0.6666666666666666 - 1.0) * (locals.var_cch * locals.var_vgop_dn5))) } } else { (assign30240_e47698 * (0.6666666666666666 * ((locals.var_cch * locals.var_vgop_dn5) / assign30240_e47696))) })))) / (assign30240_e47700 * assign30240_e47700)), (((((locals.var_vgop_dn6 + ((locals.var_vtv_dn6 * assign30240_e47669) + (locals.var_vtv * (-(((locals.var_beta_dn6 * locals.var_dvgon) + (locals.var_beta * locals.var_dvgon_dn6)) / assign30240_e47667))))) - (assign30240_e47674 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((assign30240_e47677).powf(0.6666666666666666 - 1.0) * (locals.var_cch * locals.var_vgop_dn6))) } } else { (assign30240_e47679 * (0.6666666666666666 * ((locals.var_cch * locals.var_vgop_dn6) / assign30240_e47677))) })) * assign30240_e47700) - (assign30240_e47681 * (((locals.var_vgop_dn6 * assign30240_e47687) + (locals.var_vgop * (((locals.var_vtv_dn6 * locals.var_dvgod) - (locals.var_vtv * locals.var_dvgod_dn6)) / (locals.var_dvgod * locals.var_dvgod)))) + (assign30240_e47693 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((assign30240_e47696).powf(0.6666666666666666 - 1.0) * (locals.var_cch * locals.var_vgop_dn6))) } } else { (assign30240_e47698 * (0.6666666666666666 * ((locals.var_cch * locals.var_vgop_dn6) / assign30240_e47696))) })))) / (assign30240_e47700 * assign30240_e47700)), (((((locals.var_vgop_dn7 + ((locals.var_vtv_dn7 * assign30240_e47669) + (locals.var_vtv * (-(((locals.var_beta_dn7 * locals.var_dvgon) + (locals.var_beta * locals.var_dvgon_dn7)) / assign30240_e47667))))) - (assign30240_e47674 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((assign30240_e47677).powf(0.6666666666666666 - 1.0) * (locals.var_cch * locals.var_vgop_dn7))) } } else { (assign30240_e47679 * (0.6666666666666666 * ((locals.var_cch * locals.var_vgop_dn7) / assign30240_e47677))) })) * assign30240_e47700) - (assign30240_e47681 * (((locals.var_vgop_dn7 * assign30240_e47687) + (locals.var_vgop * (((locals.var_vtv_dn7 * locals.var_dvgod) - (locals.var_vtv * locals.var_dvgod_dn7)) / (locals.var_dvgod * locals.var_dvgod)))) + (assign30240_e47693 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((assign30240_e47696).powf(0.6666666666666666 - 1.0) * (locals.var_cch * locals.var_vgop_dn7))) } } else { (assign30240_e47698 * (0.6666666666666666 * ((locals.var_cch * locals.var_vgop_dn7) / assign30240_e47696))) })))) / (assign30240_e47700 * assign30240_e47700)), (((((locals.var_vgop_dn8 + ((locals.var_vtv_dn8 * assign30240_e47669) + (locals.var_vtv * (-(((locals.var_beta_dn8 * locals.var_dvgon) + (locals.var_beta * locals.var_dvgon_dn8)) / assign30240_e47667))))) - (assign30240_e47674 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((assign30240_e47677).powf(0.6666666666666666 - 1.0) * (locals.var_cch * locals.var_vgop_dn8))) } } else { (assign30240_e47679 * (0.6666666666666666 * ((locals.var_cch * locals.var_vgop_dn8) / assign30240_e47677))) })) * assign30240_e47700) - (assign30240_e47681 * (((locals.var_vgop_dn8 * assign30240_e47687) + (locals.var_vgop * (((locals.var_vtv_dn8 * locals.var_dvgod) - (locals.var_vtv * locals.var_dvgod_dn8)) / (locals.var_dvgod * locals.var_dvgod)))) + (assign30240_e47693 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((assign30240_e47696).powf(0.6666666666666666 - 1.0) * (locals.var_cch * locals.var_vgop_dn8))) } } else { (assign30240_e47698 * (0.6666666666666666 * ((locals.var_cch * locals.var_vgop_dn8) / assign30240_e47696))) })))) / (assign30240_e47700 * assign30240_e47700)), (((((locals.var_vgop_dn9 + (locals.var_vtv * (-((locals.var_beta * locals.var_dvgon_dn9) / assign30240_e47667)))) - (assign30240_e47674 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((assign30240_e47677).powf(0.6666666666666666 - 1.0) * (locals.var_cch * locals.var_vgop_dn9))) } } else { (assign30240_e47679 * (0.6666666666666666 * ((locals.var_cch * locals.var_vgop_dn9) / assign30240_e47677))) })) * assign30240_e47700) - (assign30240_e47681 * (((locals.var_vgop_dn9 * assign30240_e47687) + (locals.var_vgop * (-((locals.var_vtv * locals.var_dvgod_dn9) / (locals.var_dvgod * locals.var_dvgod))))) + (assign30240_e47693 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((assign30240_e47696).powf(0.6666666666666666 - 1.0) * (locals.var_cch * locals.var_vgop_dn9))) } } else { (assign30240_e47698 * (0.6666666666666666 * ((locals.var_cch * locals.var_vgop_dn9) / assign30240_e47696))) })))) / (assign30240_e47700 * assign30240_e47700)), (((((locals.var_vgop_dn12 + (locals.var_vtv * (-((locals.var_beta * locals.var_dvgon_dn12) / assign30240_e47667)))) - (assign30240_e47674 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((assign30240_e47677).powf(0.6666666666666666 - 1.0) * (locals.var_cch * locals.var_vgop_dn12))) } } else { (assign30240_e47679 * (0.6666666666666666 * ((locals.var_cch * locals.var_vgop_dn12) / assign30240_e47677))) })) * assign30240_e47700) - (assign30240_e47681 * (((locals.var_vgop_dn12 * assign30240_e47687) + (locals.var_vgop * (-((locals.var_vtv * locals.var_dvgod_dn12) / (locals.var_dvgod * locals.var_dvgod))))) + (assign30240_e47693 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((assign30240_e47696).powf(0.6666666666666666 - 1.0) * (locals.var_cch * locals.var_vgop_dn12))) } } else { (assign30240_e47698 * (0.6666666666666666 * ((locals.var_cch * locals.var_vgop_dn12) / assign30240_e47696))) })))) / (assign30240_e47700 * assign30240_e47700)), (((((locals.var_vgop_dn14 + (locals.var_vtv * (-((locals.var_beta * locals.var_dvgon_dn14) / assign30240_e47667)))) - (assign30240_e47674 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((assign30240_e47677).powf(0.6666666666666666 - 1.0) * (locals.var_cch * locals.var_vgop_dn14))) } } else { (assign30240_e47679 * (0.6666666666666666 * ((locals.var_cch * locals.var_vgop_dn14) / assign30240_e47677))) })) * assign30240_e47700) - (assign30240_e47681 * (((locals.var_vgop_dn14 * assign30240_e47687) + (locals.var_vgop * (-((locals.var_vtv * locals.var_dvgod_dn14) / (locals.var_dvgod * locals.var_dvgod))))) + (assign30240_e47693 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((assign30240_e47696).powf(0.6666666666666666 - 1.0) * (locals.var_cch * locals.var_vgop_dn14))) } } else { (assign30240_e47698 * (0.6666666666666666 * ((locals.var_cch * locals.var_vgop_dn14) / assign30240_e47696))) })))) / (assign30240_e47700 * assign30240_e47700)), (((((locals.var_vgop_dn15 + ((locals.var_vtv_dn15 * assign30240_e47669) + (locals.var_vtv * (-(((locals.var_beta_dn15 * locals.var_dvgon) + (locals.var_beta * locals.var_dvgon_dn15)) / assign30240_e47667))))) - (assign30240_e47674 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((assign30240_e47677).powf(0.6666666666666666 - 1.0) * (locals.var_cch * locals.var_vgop_dn15))) } } else { (assign30240_e47679 * (0.6666666666666666 * ((locals.var_cch * locals.var_vgop_dn15) / assign30240_e47677))) })) * assign30240_e47700) - (assign30240_e47681 * (((locals.var_vgop_dn15 * assign30240_e47687) + (locals.var_vgop * (((locals.var_vtv_dn15 * locals.var_dvgod) - (locals.var_vtv * locals.var_dvgod_dn15)) / (locals.var_dvgod * locals.var_dvgod)))) + (assign30240_e47693 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((assign30240_e47696).powf(0.6666666666666666 - 1.0) * (locals.var_cch * locals.var_vgop_dn15))) } } else { (assign30240_e47698 * (0.6666666666666666 * ((locals.var_cch * locals.var_vgop_dn15) / assign30240_e47696))) })))) / (assign30240_e47700 * assign30240_e47700)), (((((locals.var_vgop_dn16 + ((locals.var_vtv_dn16 * assign30240_e47669) + (locals.var_vtv * (-(((locals.var_beta_dn16 * locals.var_dvgon) + (locals.var_beta * locals.var_dvgon_dn16)) / assign30240_e47667))))) - (assign30240_e47674 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((assign30240_e47677).powf(0.6666666666666666 - 1.0) * (locals.var_cch * locals.var_vgop_dn16))) } } else { (assign30240_e47679 * (0.6666666666666666 * ((locals.var_cch * locals.var_vgop_dn16) / assign30240_e47677))) })) * assign30240_e47700) - (assign30240_e47681 * (((locals.var_vgop_dn16 * assign30240_e47687) + (locals.var_vgop * (((locals.var_vtv_dn16 * locals.var_dvgod) - (locals.var_vtv * locals.var_dvgod_dn16)) / (locals.var_dvgod * locals.var_dvgod)))) + (assign30240_e47693 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((assign30240_e47696).powf(0.6666666666666666 - 1.0) * (locals.var_cch * locals.var_vgop_dn16))) } } else { (assign30240_e47698 * (0.6666666666666666 * ((locals.var_cch * locals.var_vgop_dn16) / assign30240_e47696))) })))) / (assign30240_e47700 * assign30240_e47700)), (((((locals.var_vgop_dn17 + ((locals.var_vtv_dn17 * assign30240_e47669) + (locals.var_vtv * (-(((locals.var_beta_dn17 * locals.var_dvgon) + (locals.var_beta * locals.var_dvgon_dn17)) / assign30240_e47667))))) - (assign30240_e47674 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((assign30240_e47677).powf(0.6666666666666666 - 1.0) * (locals.var_cch * locals.var_vgop_dn17))) } } else { (assign30240_e47679 * (0.6666666666666666 * ((locals.var_cch * locals.var_vgop_dn17) / assign30240_e47677))) })) * assign30240_e47700) - (assign30240_e47681 * (((locals.var_vgop_dn17 * assign30240_e47687) + (locals.var_vgop * (((locals.var_vtv_dn17 * locals.var_dvgod) - (locals.var_vtv * locals.var_dvgod_dn17)) / (locals.var_dvgod * locals.var_dvgod)))) + (assign30240_e47693 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((assign30240_e47696).powf(0.6666666666666666 - 1.0) * (locals.var_cch * locals.var_vgop_dn17))) } } else { (assign30240_e47698 * (0.6666666666666666 * ((locals.var_cch * locals.var_vgop_dn17) / assign30240_e47696))) })))) / (assign30240_e47700 * assign30240_e47700)), (((((locals.var_vgop_dn18 + ((locals.var_vtv_dn18 * assign30240_e47669) + (locals.var_vtv * (-(((locals.var_beta_dn18 * locals.var_dvgon) + (locals.var_beta * locals.var_dvgon_dn18)) / assign30240_e47667))))) - (assign30240_e47674 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((assign30240_e47677).powf(0.6666666666666666 - 1.0) * (locals.var_cch * locals.var_vgop_dn18))) } } else { (assign30240_e47679 * (0.6666666666666666 * ((locals.var_cch * locals.var_vgop_dn18) / assign30240_e47677))) })) * assign30240_e47700) - (assign30240_e47681 * (((locals.var_vgop_dn18 * assign30240_e47687) + (locals.var_vgop * (((locals.var_vtv_dn18 * locals.var_dvgod) - (locals.var_vtv * locals.var_dvgod_dn18)) / (locals.var_dvgod * locals.var_dvgod)))) + (assign30240_e47693 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((assign30240_e47696).powf(0.6666666666666666 - 1.0) * (locals.var_cch * locals.var_vgop_dn18))) } } else { (assign30240_e47698 * (0.6666666666666666 * ((locals.var_cch * locals.var_vgop_dn18) / assign30240_e47696))) })))) / (assign30240_e47700 * assign30240_e47700)), (((((locals.var_vgop_dn19 + ((locals.var_vtv_dn19 * assign30240_e47669) + (locals.var_vtv * (-(((locals.var_beta_dn19 * locals.var_dvgon) + (locals.var_beta * locals.var_dvgon_dn19)) / assign30240_e47667))))) - (assign30240_e47674 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((assign30240_e47677).powf(0.6666666666666666 - 1.0) * (locals.var_cch * locals.var_vgop_dn19))) } } else { (assign30240_e47679 * (0.6666666666666666 * ((locals.var_cch * locals.var_vgop_dn19) / assign30240_e47677))) })) * assign30240_e47700) - (assign30240_e47681 * (((locals.var_vgop_dn19 * assign30240_e47687) + (locals.var_vgop * (((locals.var_vtv_dn19 * locals.var_dvgod) - (locals.var_vtv * locals.var_dvgod_dn19)) / (locals.var_dvgod * locals.var_dvgod)))) + (assign30240_e47693 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((assign30240_e47696).powf(0.6666666666666666 - 1.0) * (locals.var_cch * locals.var_vgop_dn19))) } } else { (assign30240_e47698 * (0.6666666666666666 * ((locals.var_cch * locals.var_vgop_dn19) / assign30240_e47696))) })))) / (assign30240_e47700 * assign30240_e47700)), (((((locals.var_vgop_dn20 + ((locals.var_vtv_dn20 * assign30240_e47669) + (locals.var_vtv * (-(((locals.var_beta_dn20 * locals.var_dvgon) + (locals.var_beta * locals.var_dvgon_dn20)) / assign30240_e47667))))) - (assign30240_e47674 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((assign30240_e47677).powf(0.6666666666666666 - 1.0) * (locals.var_cch * locals.var_vgop_dn20))) } } else { (assign30240_e47679 * (0.6666666666666666 * ((locals.var_cch * locals.var_vgop_dn20) / assign30240_e47677))) })) * assign30240_e47700) - (assign30240_e47681 * (((locals.var_vgop_dn20 * assign30240_e47687) + (locals.var_vgop * (((locals.var_vtv_dn20 * locals.var_dvgod) - (locals.var_vtv * locals.var_dvgod_dn20)) / (locals.var_dvgod * locals.var_dvgod)))) + (assign30240_e47693 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((assign30240_e47696).powf(0.6666666666666666 - 1.0) * (locals.var_cch * locals.var_vgop_dn20))) } } else { (assign30240_e47698 * (0.6666666666666666 * ((locals.var_cch * locals.var_vgop_dn20) / assign30240_e47696))) })))) / (assign30240_e47700 * assign30240_e47700)), (((((locals.var_vgop_dn21 + ((locals.var_vtv_dn21 * assign30240_e47669) + (locals.var_vtv * (-(((locals.var_beta_dn21 * locals.var_dvgon) + (locals.var_beta * locals.var_dvgon_dn21)) / assign30240_e47667))))) - (assign30240_e47674 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((assign30240_e47677).powf(0.6666666666666666 - 1.0) * (locals.var_cch * locals.var_vgop_dn21))) } } else { (assign30240_e47679 * (0.6666666666666666 * ((locals.var_cch * locals.var_vgop_dn21) / assign30240_e47677))) })) * assign30240_e47700) - (assign30240_e47681 * (((locals.var_vgop_dn21 * assign30240_e47687) + (locals.var_vgop * (((locals.var_vtv_dn21 * locals.var_dvgod) - (locals.var_vtv * locals.var_dvgod_dn21)) / (locals.var_dvgod * locals.var_dvgod)))) + (assign30240_e47693 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((assign30240_e47696).powf(0.6666666666666666 - 1.0) * (locals.var_cch * locals.var_vgop_dn21))) } } else { (assign30240_e47698 * (0.6666666666666666 * ((locals.var_cch * locals.var_vgop_dn21) / assign30240_e47696))) })))) / (assign30240_e47700 * assign30240_e47700)), (((((locals.var_vgop_dn22 + ((locals.var_vtv_dn22 * assign30240_e47669) + (locals.var_vtv * (-(((locals.var_beta_dn22 * locals.var_dvgon) + (locals.var_beta * locals.var_dvgon_dn22)) / assign30240_e47667))))) - (assign30240_e47674 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((assign30240_e47677).powf(0.6666666666666666 - 1.0) * (locals.var_cch * locals.var_vgop_dn22))) } } else { (assign30240_e47679 * (0.6666666666666666 * ((locals.var_cch * locals.var_vgop_dn22) / assign30240_e47677))) })) * assign30240_e47700) - (assign30240_e47681 * (((locals.var_vgop_dn22 * assign30240_e47687) + (locals.var_vgop * (((locals.var_vtv_dn22 * locals.var_dvgod) - (locals.var_vtv * locals.var_dvgod_dn22)) / (locals.var_dvgod * locals.var_dvgod)))) + (assign30240_e47693 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((assign30240_e47696).powf(0.6666666666666666 - 1.0) * (locals.var_cch * locals.var_vgop_dn22))) } } else { (assign30240_e47698 * (0.6666666666666666 * ((locals.var_cch * locals.var_vgop_dn22) / assign30240_e47696))) })))) / (assign30240_e47700 * assign30240_e47700)),)
    } else {
        (locals.var_hx, locals.var_hx_dn0, locals.var_hx_dn1, locals.var_hx_dn2, locals.var_hx_dn3, locals.var_hx_dn4, locals.var_hx_dn5, locals.var_hx_dn6, locals.var_hx_dn7, locals.var_hx_dn8, locals.var_hx_dn9, locals.var_hx_dn12, locals.var_hx_dn14, locals.var_hx_dn15, locals.var_hx_dn16, locals.var_hx_dn17, locals.var_hx_dn18, locals.var_hx_dn19, locals.var_hx_dn20, locals.var_hx_dn21, locals.var_hx_dn22,)
    }
};
        locals.var_hx = assign30240_e47703;
        locals.var_hx_dn0 = assign30240_e47703_d_n0;
        locals.var_hx_dn1 = assign30240_e47703_d_n1;
        locals.var_hx_dn2 = assign30240_e47703_d_n2;
        locals.var_hx_dn3 = assign30240_e47703_d_n3;
        locals.var_hx_dn4 = assign30240_e47703_d_n4;
        locals.var_hx_dn5 = assign30240_e47703_d_n5;
        locals.var_hx_dn6 = assign30240_e47703_d_n6;
        locals.var_hx_dn7 = assign30240_e47703_d_n7;
        locals.var_hx_dn8 = assign30240_e47703_d_n8;
        locals.var_hx_dn9 = assign30240_e47703_d_n9;
        locals.var_hx_dn12 = assign30240_e47703_d_n12;
        locals.var_hx_dn14 = assign30240_e47703_d_n14;
        locals.var_hx_dn15 = assign30240_e47703_d_n15;
        locals.var_hx_dn16 = assign30240_e47703_d_n16;
        locals.var_hx_dn17 = assign30240_e47703_d_n17;
        locals.var_hx_dn18 = assign30240_e47703_d_n18;
        locals.var_hx_dn19 = assign30240_e47703_d_n19;
        locals.var_hx_dn20 = assign30240_e47703_d_n20;
        locals.var_hx_dn21 = assign30240_e47703_d_n21;
        locals.var_hx_dn22 = assign30240_e47703_d_n22;

        let (assign30250_e47714, assign30250_e47714_d_n0, assign30250_e47714_d_n1, assign30250_e47714_d_n2, assign30250_e47714_d_n3, assign30250_e47714_d_n4, assign30250_e47714_d_n5, assign30250_e47714_d_n6, assign30250_e47714_d_n7, assign30250_e47714_d_n8, assign30250_e47714_d_n9, assign30250_e47714_d_n12, assign30250_e47714_d_n14, assign30250_e47714_d_n15, assign30250_e47714_d_n16, assign30250_e47714_d_n17, assign30250_e47714_d_n18, assign30250_e47714_d_n19, assign30250_e47714_d_n20, assign30250_e47714_d_n21, assign30250_e47714_d_n22,) = {
    if ((locals.var_guard504 == 0.0) && (locals.var_guard513 != 0.0)) {
        let assign30250_e47711: f64 = (2.0 * locals.var_vtv);
        let assign30250_e47712: f64 = (locals.var_vgod / assign30250_e47711);
        (assign30250_e47712, (locals.var_vgod_dn0 / assign30250_e47711), (locals.var_vgod_dn1 / assign30250_e47711), (locals.var_vgod_dn2 / assign30250_e47711), (locals.var_vgod_dn3 / assign30250_e47711), (((locals.var_vgod_dn4 * assign30250_e47711) - (locals.var_vgod * (2.0 * locals.var_vtv_dn4))) / (assign30250_e47711 * assign30250_e47711)), (locals.var_vgod_dn5 / assign30250_e47711), (((locals.var_vgod_dn6 * assign30250_e47711) - (locals.var_vgod * (2.0 * locals.var_vtv_dn6))) / (assign30250_e47711 * assign30250_e47711)), (((locals.var_vgod_dn7 * assign30250_e47711) - (locals.var_vgod * (2.0 * locals.var_vtv_dn7))) / (assign30250_e47711 * assign30250_e47711)), (((locals.var_vgod_dn8 * assign30250_e47711) - (locals.var_vgod * (2.0 * locals.var_vtv_dn8))) / (assign30250_e47711 * assign30250_e47711)), (locals.var_vgod_dn9 / assign30250_e47711), (locals.var_vgod_dn12 / assign30250_e47711), (locals.var_vgod_dn14 / assign30250_e47711), (((locals.var_vgod_dn15 * assign30250_e47711) - (locals.var_vgod * (2.0 * locals.var_vtv_dn15))) / (assign30250_e47711 * assign30250_e47711)), (((locals.var_vgod_dn16 * assign30250_e47711) - (locals.var_vgod * (2.0 * locals.var_vtv_dn16))) / (assign30250_e47711 * assign30250_e47711)), (((locals.var_vgod_dn17 * assign30250_e47711) - (locals.var_vgod * (2.0 * locals.var_vtv_dn17))) / (assign30250_e47711 * assign30250_e47711)), (((locals.var_vgod_dn18 * assign30250_e47711) - (locals.var_vgod * (2.0 * locals.var_vtv_dn18))) / (assign30250_e47711 * assign30250_e47711)), (((locals.var_vgod_dn19 * assign30250_e47711) - (locals.var_vgod * (2.0 * locals.var_vtv_dn19))) / (assign30250_e47711 * assign30250_e47711)), (((locals.var_vgod_dn20 * assign30250_e47711) - (locals.var_vgod * (2.0 * locals.var_vtv_dn20))) / (assign30250_e47711 * assign30250_e47711)), (((locals.var_vgod_dn21 * assign30250_e47711) - (locals.var_vgod * (2.0 * locals.var_vtv_dn21))) / (assign30250_e47711 * assign30250_e47711)), (((locals.var_vgod_dn22 * assign30250_e47711) - (locals.var_vgod * (2.0 * locals.var_vtv_dn22))) / (assign30250_e47711 * assign30250_e47711)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn1, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn12, locals.var_t0_dn14, locals.var_t0_dn15, locals.var_t0_dn16, locals.var_t0_dn17, locals.var_t0_dn18, locals.var_t0_dn19, locals.var_t0_dn20, locals.var_t0_dn21, locals.var_t0_dn22,)
    }
};
        locals.var_t0 = assign30250_e47714;
        locals.var_t0_dn0 = assign30250_e47714_d_n0;
        locals.var_t0_dn1 = assign30250_e47714_d_n1;
        locals.var_t0_dn2 = assign30250_e47714_d_n2;
        locals.var_t0_dn3 = assign30250_e47714_d_n3;
        locals.var_t0_dn4 = assign30250_e47714_d_n4;
        locals.var_t0_dn5 = assign30250_e47714_d_n5;
        locals.var_t0_dn6 = assign30250_e47714_d_n6;
        locals.var_t0_dn7 = assign30250_e47714_d_n7;
        locals.var_t0_dn8 = assign30250_e47714_d_n8;
        locals.var_t0_dn9 = assign30250_e47714_d_n9;
        locals.var_t0_dn12 = assign30250_e47714_d_n12;
        locals.var_t0_dn14 = assign30250_e47714_d_n14;
        locals.var_t0_dn15 = assign30250_e47714_d_n15;
        locals.var_t0_dn16 = assign30250_e47714_d_n16;
        locals.var_t0_dn17 = assign30250_e47714_d_n17;
        locals.var_t0_dn18 = assign30250_e47714_d_n18;
        locals.var_t0_dn19 = assign30250_e47714_d_n19;
        locals.var_t0_dn20 = assign30250_e47714_d_n20;
        locals.var_t0_dn21 = assign30250_e47714_d_n21;
        locals.var_t0_dn22 = assign30250_e47714_d_n22;

        let assign30260_e47717: f64 = if locals.var_t0 < 200.0 { 1.0 } else { 0.0 };
        locals.var_guard517 = assign30260_e47717;

        let (assign30270_e47729, assign30270_e47729_d_n0, assign30270_e47729_d_n1, assign30270_e47729_d_n2, assign30270_e47729_d_n3, assign30270_e47729_d_n4, assign30270_e47729_d_n5, assign30270_e47729_d_n6, assign30270_e47729_d_n7, assign30270_e47729_d_n8, assign30270_e47729_d_n9, assign30270_e47729_d_n12, assign30270_e47729_d_n14, assign30270_e47729_d_n15, assign30270_e47729_d_n16, assign30270_e47729_d_n17, assign30270_e47729_d_n18, assign30270_e47729_d_n19, assign30270_e47729_d_n20, assign30270_e47729_d_n21, assign30270_e47729_d_n22,) = {
    if (((locals.var_guard504 == 0.0) && (locals.var_guard513 != 0.0)) && (locals.var_guard517 != 0.0)) {
        let assign30270_e47726: f64 = (locals.var_t0 / 4.0);
        let assign30270_e47727: f64 = { let limited_exp_arg = assign30270_e47726; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign30270_e47727, ({ let limited_exp_arg = assign30270_e47726; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn0 / 4.0)), ({ let limited_exp_arg = assign30270_e47726; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn1 / 4.0)), ({ let limited_exp_arg = assign30270_e47726; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn2 / 4.0)), ({ let limited_exp_arg = assign30270_e47726; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn3 / 4.0)), ({ let limited_exp_arg = assign30270_e47726; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn4 / 4.0)), ({ let limited_exp_arg = assign30270_e47726; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn5 / 4.0)), ({ let limited_exp_arg = assign30270_e47726; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn6 / 4.0)), ({ let limited_exp_arg = assign30270_e47726; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn7 / 4.0)), ({ let limited_exp_arg = assign30270_e47726; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn8 / 4.0)), ({ let limited_exp_arg = assign30270_e47726; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn9 / 4.0)), ({ let limited_exp_arg = assign30270_e47726; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn12 / 4.0)), ({ let limited_exp_arg = assign30270_e47726; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn14 / 4.0)), ({ let limited_exp_arg = assign30270_e47726; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn15 / 4.0)), ({ let limited_exp_arg = assign30270_e47726; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn16 / 4.0)), ({ let limited_exp_arg = assign30270_e47726; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn17 / 4.0)), ({ let limited_exp_arg = assign30270_e47726; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn18 / 4.0)), ({ let limited_exp_arg = assign30270_e47726; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn19 / 4.0)), ({ let limited_exp_arg = assign30270_e47726; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn20 / 4.0)), ({ let limited_exp_arg = assign30270_e47726; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn21 / 4.0)), ({ let limited_exp_arg = assign30270_e47726; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn22 / 4.0)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn1, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn12, locals.var_t1_dn14, locals.var_t1_dn15, locals.var_t1_dn16, locals.var_t1_dn17, locals.var_t1_dn18, locals.var_t1_dn19, locals.var_t1_dn20, locals.var_t1_dn21, locals.var_t1_dn22,)
    }
};
        locals.var_t1 = assign30270_e47729;
        locals.var_t1_dn0 = assign30270_e47729_d_n0;
        locals.var_t1_dn1 = assign30270_e47729_d_n1;
        locals.var_t1_dn2 = assign30270_e47729_d_n2;
        locals.var_t1_dn3 = assign30270_e47729_d_n3;
        locals.var_t1_dn4 = assign30270_e47729_d_n4;
        locals.var_t1_dn5 = assign30270_e47729_d_n5;
        locals.var_t1_dn6 = assign30270_e47729_d_n6;
        locals.var_t1_dn7 = assign30270_e47729_d_n7;
        locals.var_t1_dn8 = assign30270_e47729_d_n8;
        locals.var_t1_dn9 = assign30270_e47729_d_n9;
        locals.var_t1_dn12 = assign30270_e47729_d_n12;
        locals.var_t1_dn14 = assign30270_e47729_d_n14;
        locals.var_t1_dn15 = assign30270_e47729_d_n15;
        locals.var_t1_dn16 = assign30270_e47729_d_n16;
        locals.var_t1_dn17 = assign30270_e47729_d_n17;
        locals.var_t1_dn18 = assign30270_e47729_d_n18;
        locals.var_t1_dn19 = assign30270_e47729_d_n19;
        locals.var_t1_dn20 = assign30270_e47729_d_n20;
        locals.var_t1_dn21 = assign30270_e47729_d_n21;
        locals.var_t1_dn22 = assign30270_e47729_d_n22;

        let (assign30280_e47744, assign30280_e47744_d_n0, assign30280_e47744_d_n1, assign30280_e47744_d_n2, assign30280_e47744_d_n3, assign30280_e47744_d_n4, assign30280_e47744_d_n5, assign30280_e47744_d_n6, assign30280_e47744_d_n7, assign30280_e47744_d_n8, assign30280_e47744_d_n9, assign30280_e47744_d_n12, assign30280_e47744_d_n14, assign30280_e47744_d_n15, assign30280_e47744_d_n16, assign30280_e47744_d_n17, assign30280_e47744_d_n18, assign30280_e47744_d_n19, assign30280_e47744_d_n20, assign30280_e47744_d_n21, assign30280_e47744_d_n22,) = {
    if (((locals.var_guard504 == 0.0) && (locals.var_guard513 != 0.0)) && (locals.var_guard517 != 0.0)) {
        let assign30280_e47737: f64 = (-3.0);
        let assign30280_e47739: f64 = (assign30280_e47737 * locals.var_t0);
        let assign30280_e47741: f64 = (assign30280_e47739 / 4.0);
        let assign30280_e47742: f64 = { let limited_exp_arg = assign30280_e47741; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign30280_e47742, ({ let limited_exp_arg = assign30280_e47741; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((assign30280_e47737 * locals.var_t0_dn0) / 4.0)), ({ let limited_exp_arg = assign30280_e47741; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((assign30280_e47737 * locals.var_t0_dn1) / 4.0)), ({ let limited_exp_arg = assign30280_e47741; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((assign30280_e47737 * locals.var_t0_dn2) / 4.0)), ({ let limited_exp_arg = assign30280_e47741; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((assign30280_e47737 * locals.var_t0_dn3) / 4.0)), ({ let limited_exp_arg = assign30280_e47741; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((assign30280_e47737 * locals.var_t0_dn4) / 4.0)), ({ let limited_exp_arg = assign30280_e47741; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((assign30280_e47737 * locals.var_t0_dn5) / 4.0)), ({ let limited_exp_arg = assign30280_e47741; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((assign30280_e47737 * locals.var_t0_dn6) / 4.0)), ({ let limited_exp_arg = assign30280_e47741; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((assign30280_e47737 * locals.var_t0_dn7) / 4.0)), ({ let limited_exp_arg = assign30280_e47741; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((assign30280_e47737 * locals.var_t0_dn8) / 4.0)), ({ let limited_exp_arg = assign30280_e47741; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((assign30280_e47737 * locals.var_t0_dn9) / 4.0)), ({ let limited_exp_arg = assign30280_e47741; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((assign30280_e47737 * locals.var_t0_dn12) / 4.0)), ({ let limited_exp_arg = assign30280_e47741; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((assign30280_e47737 * locals.var_t0_dn14) / 4.0)), ({ let limited_exp_arg = assign30280_e47741; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((assign30280_e47737 * locals.var_t0_dn15) / 4.0)), ({ let limited_exp_arg = assign30280_e47741; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((assign30280_e47737 * locals.var_t0_dn16) / 4.0)), ({ let limited_exp_arg = assign30280_e47741; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((assign30280_e47737 * locals.var_t0_dn17) / 4.0)), ({ let limited_exp_arg = assign30280_e47741; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((assign30280_e47737 * locals.var_t0_dn18) / 4.0)), ({ let limited_exp_arg = assign30280_e47741; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((assign30280_e47737 * locals.var_t0_dn19) / 4.0)), ({ let limited_exp_arg = assign30280_e47741; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((assign30280_e47737 * locals.var_t0_dn20) / 4.0)), ({ let limited_exp_arg = assign30280_e47741; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((assign30280_e47737 * locals.var_t0_dn21) / 4.0)), ({ let limited_exp_arg = assign30280_e47741; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((assign30280_e47737 * locals.var_t0_dn22) / 4.0)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn1, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn12, locals.var_t2_dn14, locals.var_t2_dn15, locals.var_t2_dn16, locals.var_t2_dn17, locals.var_t2_dn18, locals.var_t2_dn19, locals.var_t2_dn20, locals.var_t2_dn21, locals.var_t2_dn22,)
    }
};
        locals.var_t2 = assign30280_e47744;
        locals.var_t2_dn0 = assign30280_e47744_d_n0;
        locals.var_t2_dn1 = assign30280_e47744_d_n1;
        locals.var_t2_dn2 = assign30280_e47744_d_n2;
        locals.var_t2_dn3 = assign30280_e47744_d_n3;
        locals.var_t2_dn4 = assign30280_e47744_d_n4;
        locals.var_t2_dn5 = assign30280_e47744_d_n5;
        locals.var_t2_dn6 = assign30280_e47744_d_n6;
        locals.var_t2_dn7 = assign30280_e47744_d_n7;
        locals.var_t2_dn8 = assign30280_e47744_d_n8;
        locals.var_t2_dn9 = assign30280_e47744_d_n9;
        locals.var_t2_dn12 = assign30280_e47744_d_n12;
        locals.var_t2_dn14 = assign30280_e47744_d_n14;
        locals.var_t2_dn15 = assign30280_e47744_d_n15;
        locals.var_t2_dn16 = assign30280_e47744_d_n16;
        locals.var_t2_dn17 = assign30280_e47744_d_n17;
        locals.var_t2_dn18 = assign30280_e47744_d_n18;
        locals.var_t2_dn19 = assign30280_e47744_d_n19;
        locals.var_t2_dn20 = assign30280_e47744_d_n20;
        locals.var_t2_dn21 = assign30280_e47744_d_n21;
        locals.var_t2_dn22 = assign30280_e47744_d_n22;

        let (assign30290_e47786, assign30290_e47786_d_n0, assign30290_e47786_d_n1, assign30290_e47786_d_n2, assign30290_e47786_d_n3, assign30290_e47786_d_n4, assign30290_e47786_d_n5, assign30290_e47786_d_n6, assign30290_e47786_d_n7, assign30290_e47786_d_n8, assign30290_e47786_d_n9, assign30290_e47786_d_n12, assign30290_e47786_d_n14, assign30290_e47786_d_n15, assign30290_e47786_d_n16, assign30290_e47786_d_n17, assign30290_e47786_d_n18, assign30290_e47786_d_n19, assign30290_e47786_d_n20, assign30290_e47786_d_n21, assign30290_e47786_d_n22,) = {
    if (((locals.var_guard504 == 0.0) && (locals.var_guard513 != 0.0)) && (locals.var_guard517 != 0.0)) {
        let assign30290_e47753: f64 = (2.0 * locals.var_vtv);
        let assign30290_e47755: f64 = (assign30290_e47753 * locals.var_cch);
        let assign30290_e47758: f64 = (3.0 * locals.var_t0);
        let assign30290_e47760: f64 = (assign30290_e47758 / 4.0);
        let assign30290_e47763: f64 = (locals.var_t1 + locals.var_t2);
        let assign30290_e47764: f64 = (assign30290_e47763).ln();
        let assign30290_e47765: f64 = (assign30290_e47760 + assign30290_e47764);
        let assign30290_e47766: f64 = (assign30290_e47755 * assign30290_e47765);
        let assign30290_e47769: f64 = (1.0 / locals.var_hx);
        let assign30290_e47772: f64 = (locals.var_cch / 3.24e17);
        let assign30290_e47774: f64 = (-1.0);
        let assign30290_e47776: f64 = (assign30290_e47774 * locals.var_vgod);
        let assign30290_e47779: f64 = (2.0 * locals.var_vtv);
        let assign30290_e47780: f64 = (assign30290_e47776 / assign30290_e47779);
        let assign30290_e47781: f64 = { let limited_exp_arg = assign30290_e47780; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign30290_e47782: f64 = (assign30290_e47772 * assign30290_e47781);
        let assign30290_e47783: f64 = (assign30290_e47769 + assign30290_e47782);
        let assign30290_e47784: f64 = (assign30290_e47766 / assign30290_e47783);
        (assign30290_e47784, ((((assign30290_e47755 * (((3.0 * locals.var_t0_dn0) / 4.0) + ((locals.var_t1_dn0 + locals.var_t2_dn0) / assign30290_e47763))) * assign30290_e47783) - (assign30290_e47766 * ((-(locals.var_hx_dn0 / (locals.var_hx * locals.var_hx))) + (assign30290_e47772 * ({ let limited_exp_arg = assign30290_e47780; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((assign30290_e47774 * locals.var_vgod_dn0) / assign30290_e47779)))))) / (assign30290_e47783 * assign30290_e47783)), ((((assign30290_e47755 * (((3.0 * locals.var_t0_dn1) / 4.0) + ((locals.var_t1_dn1 + locals.var_t2_dn1) / assign30290_e47763))) * assign30290_e47783) - (assign30290_e47766 * ((-(locals.var_hx_dn1 / (locals.var_hx * locals.var_hx))) + (assign30290_e47772 * ({ let limited_exp_arg = assign30290_e47780; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((assign30290_e47774 * locals.var_vgod_dn1) / assign30290_e47779)))))) / (assign30290_e47783 * assign30290_e47783)), ((((assign30290_e47755 * (((3.0 * locals.var_t0_dn2) / 4.0) + ((locals.var_t1_dn2 + locals.var_t2_dn2) / assign30290_e47763))) * assign30290_e47783) - (assign30290_e47766 * ((-(locals.var_hx_dn2 / (locals.var_hx * locals.var_hx))) + (assign30290_e47772 * ({ let limited_exp_arg = assign30290_e47780; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((assign30290_e47774 * locals.var_vgod_dn2) / assign30290_e47779)))))) / (assign30290_e47783 * assign30290_e47783)), ((((assign30290_e47755 * (((3.0 * locals.var_t0_dn3) / 4.0) + ((locals.var_t1_dn3 + locals.var_t2_dn3) / assign30290_e47763))) * assign30290_e47783) - (assign30290_e47766 * ((-(locals.var_hx_dn3 / (locals.var_hx * locals.var_hx))) + (assign30290_e47772 * ({ let limited_exp_arg = assign30290_e47780; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((assign30290_e47774 * locals.var_vgod_dn3) / assign30290_e47779)))))) / (assign30290_e47783 * assign30290_e47783)), (((((((2.0 * locals.var_vtv_dn4) * locals.var_cch) * assign30290_e47765) + (assign30290_e47755 * (((3.0 * locals.var_t0_dn4) / 4.0) + ((locals.var_t1_dn4 + locals.var_t2_dn4) / assign30290_e47763)))) * assign30290_e47783) - (assign30290_e47766 * ((-(locals.var_hx_dn4 / (locals.var_hx * locals.var_hx))) + (assign30290_e47772 * ({ let limited_exp_arg = assign30290_e47780; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((assign30290_e47774 * locals.var_vgod_dn4) * assign30290_e47779) - (assign30290_e47776 * (2.0 * locals.var_vtv_dn4))) / (assign30290_e47779 * assign30290_e47779))))))) / (assign30290_e47783 * assign30290_e47783)), ((((assign30290_e47755 * (((3.0 * locals.var_t0_dn5) / 4.0) + ((locals.var_t1_dn5 + locals.var_t2_dn5) / assign30290_e47763))) * assign30290_e47783) - (assign30290_e47766 * ((-(locals.var_hx_dn5 / (locals.var_hx * locals.var_hx))) + (assign30290_e47772 * ({ let limited_exp_arg = assign30290_e47780; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((assign30290_e47774 * locals.var_vgod_dn5) / assign30290_e47779)))))) / (assign30290_e47783 * assign30290_e47783)), (((((((2.0 * locals.var_vtv_dn6) * locals.var_cch) * assign30290_e47765) + (assign30290_e47755 * (((3.0 * locals.var_t0_dn6) / 4.0) + ((locals.var_t1_dn6 + locals.var_t2_dn6) / assign30290_e47763)))) * assign30290_e47783) - (assign30290_e47766 * ((-(locals.var_hx_dn6 / (locals.var_hx * locals.var_hx))) + (assign30290_e47772 * ({ let limited_exp_arg = assign30290_e47780; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((assign30290_e47774 * locals.var_vgod_dn6) * assign30290_e47779) - (assign30290_e47776 * (2.0 * locals.var_vtv_dn6))) / (assign30290_e47779 * assign30290_e47779))))))) / (assign30290_e47783 * assign30290_e47783)), (((((((2.0 * locals.var_vtv_dn7) * locals.var_cch) * assign30290_e47765) + (assign30290_e47755 * (((3.0 * locals.var_t0_dn7) / 4.0) + ((locals.var_t1_dn7 + locals.var_t2_dn7) / assign30290_e47763)))) * assign30290_e47783) - (assign30290_e47766 * ((-(locals.var_hx_dn7 / (locals.var_hx * locals.var_hx))) + (assign30290_e47772 * ({ let limited_exp_arg = assign30290_e47780; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((assign30290_e47774 * locals.var_vgod_dn7) * assign30290_e47779) - (assign30290_e47776 * (2.0 * locals.var_vtv_dn7))) / (assign30290_e47779 * assign30290_e47779))))))) / (assign30290_e47783 * assign30290_e47783)), (((((((2.0 * locals.var_vtv_dn8) * locals.var_cch) * assign30290_e47765) + (assign30290_e47755 * (((3.0 * locals.var_t0_dn8) / 4.0) + ((locals.var_t1_dn8 + locals.var_t2_dn8) / assign30290_e47763)))) * assign30290_e47783) - (assign30290_e47766 * ((-(locals.var_hx_dn8 / (locals.var_hx * locals.var_hx))) + (assign30290_e47772 * ({ let limited_exp_arg = assign30290_e47780; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((assign30290_e47774 * locals.var_vgod_dn8) * assign30290_e47779) - (assign30290_e47776 * (2.0 * locals.var_vtv_dn8))) / (assign30290_e47779 * assign30290_e47779))))))) / (assign30290_e47783 * assign30290_e47783)), ((((assign30290_e47755 * (((3.0 * locals.var_t0_dn9) / 4.0) + ((locals.var_t1_dn9 + locals.var_t2_dn9) / assign30290_e47763))) * assign30290_e47783) - (assign30290_e47766 * ((-(locals.var_hx_dn9 / (locals.var_hx * locals.var_hx))) + (assign30290_e47772 * ({ let limited_exp_arg = assign30290_e47780; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((assign30290_e47774 * locals.var_vgod_dn9) / assign30290_e47779)))))) / (assign30290_e47783 * assign30290_e47783)), ((((assign30290_e47755 * (((3.0 * locals.var_t0_dn12) / 4.0) + ((locals.var_t1_dn12 + locals.var_t2_dn12) / assign30290_e47763))) * assign30290_e47783) - (assign30290_e47766 * ((-(locals.var_hx_dn12 / (locals.var_hx * locals.var_hx))) + (assign30290_e47772 * ({ let limited_exp_arg = assign30290_e47780; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((assign30290_e47774 * locals.var_vgod_dn12) / assign30290_e47779)))))) / (assign30290_e47783 * assign30290_e47783)), ((((assign30290_e47755 * (((3.0 * locals.var_t0_dn14) / 4.0) + ((locals.var_t1_dn14 + locals.var_t2_dn14) / assign30290_e47763))) * assign30290_e47783) - (assign30290_e47766 * ((-(locals.var_hx_dn14 / (locals.var_hx * locals.var_hx))) + (assign30290_e47772 * ({ let limited_exp_arg = assign30290_e47780; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((assign30290_e47774 * locals.var_vgod_dn14) / assign30290_e47779)))))) / (assign30290_e47783 * assign30290_e47783)), (((((((2.0 * locals.var_vtv_dn15) * locals.var_cch) * assign30290_e47765) + (assign30290_e47755 * (((3.0 * locals.var_t0_dn15) / 4.0) + ((locals.var_t1_dn15 + locals.var_t2_dn15) / assign30290_e47763)))) * assign30290_e47783) - (assign30290_e47766 * ((-(locals.var_hx_dn15 / (locals.var_hx * locals.var_hx))) + (assign30290_e47772 * ({ let limited_exp_arg = assign30290_e47780; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((assign30290_e47774 * locals.var_vgod_dn15) * assign30290_e47779) - (assign30290_e47776 * (2.0 * locals.var_vtv_dn15))) / (assign30290_e47779 * assign30290_e47779))))))) / (assign30290_e47783 * assign30290_e47783)), (((((((2.0 * locals.var_vtv_dn16) * locals.var_cch) * assign30290_e47765) + (assign30290_e47755 * (((3.0 * locals.var_t0_dn16) / 4.0) + ((locals.var_t1_dn16 + locals.var_t2_dn16) / assign30290_e47763)))) * assign30290_e47783) - (assign30290_e47766 * ((-(locals.var_hx_dn16 / (locals.var_hx * locals.var_hx))) + (assign30290_e47772 * ({ let limited_exp_arg = assign30290_e47780; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((assign30290_e47774 * locals.var_vgod_dn16) * assign30290_e47779) - (assign30290_e47776 * (2.0 * locals.var_vtv_dn16))) / (assign30290_e47779 * assign30290_e47779))))))) / (assign30290_e47783 * assign30290_e47783)), (((((((2.0 * locals.var_vtv_dn17) * locals.var_cch) * assign30290_e47765) + (assign30290_e47755 * (((3.0 * locals.var_t0_dn17) / 4.0) + ((locals.var_t1_dn17 + locals.var_t2_dn17) / assign30290_e47763)))) * assign30290_e47783) - (assign30290_e47766 * ((-(locals.var_hx_dn17 / (locals.var_hx * locals.var_hx))) + (assign30290_e47772 * ({ let limited_exp_arg = assign30290_e47780; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((assign30290_e47774 * locals.var_vgod_dn17) * assign30290_e47779) - (assign30290_e47776 * (2.0 * locals.var_vtv_dn17))) / (assign30290_e47779 * assign30290_e47779))))))) / (assign30290_e47783 * assign30290_e47783)), (((((((2.0 * locals.var_vtv_dn18) * locals.var_cch) * assign30290_e47765) + (assign30290_e47755 * (((3.0 * locals.var_t0_dn18) / 4.0) + ((locals.var_t1_dn18 + locals.var_t2_dn18) / assign30290_e47763)))) * assign30290_e47783) - (assign30290_e47766 * ((-(locals.var_hx_dn18 / (locals.var_hx * locals.var_hx))) + (assign30290_e47772 * ({ let limited_exp_arg = assign30290_e47780; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((assign30290_e47774 * locals.var_vgod_dn18) * assign30290_e47779) - (assign30290_e47776 * (2.0 * locals.var_vtv_dn18))) / (assign30290_e47779 * assign30290_e47779))))))) / (assign30290_e47783 * assign30290_e47783)), (((((((2.0 * locals.var_vtv_dn19) * locals.var_cch) * assign30290_e47765) + (assign30290_e47755 * (((3.0 * locals.var_t0_dn19) / 4.0) + ((locals.var_t1_dn19 + locals.var_t2_dn19) / assign30290_e47763)))) * assign30290_e47783) - (assign30290_e47766 * ((-(locals.var_hx_dn19 / (locals.var_hx * locals.var_hx))) + (assign30290_e47772 * ({ let limited_exp_arg = assign30290_e47780; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((assign30290_e47774 * locals.var_vgod_dn19) * assign30290_e47779) - (assign30290_e47776 * (2.0 * locals.var_vtv_dn19))) / (assign30290_e47779 * assign30290_e47779))))))) / (assign30290_e47783 * assign30290_e47783)), (((((((2.0 * locals.var_vtv_dn20) * locals.var_cch) * assign30290_e47765) + (assign30290_e47755 * (((3.0 * locals.var_t0_dn20) / 4.0) + ((locals.var_t1_dn20 + locals.var_t2_dn20) / assign30290_e47763)))) * assign30290_e47783) - (assign30290_e47766 * ((-(locals.var_hx_dn20 / (locals.var_hx * locals.var_hx))) + (assign30290_e47772 * ({ let limited_exp_arg = assign30290_e47780; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((assign30290_e47774 * locals.var_vgod_dn20) * assign30290_e47779) - (assign30290_e47776 * (2.0 * locals.var_vtv_dn20))) / (assign30290_e47779 * assign30290_e47779))))))) / (assign30290_e47783 * assign30290_e47783)), (((((((2.0 * locals.var_vtv_dn21) * locals.var_cch) * assign30290_e47765) + (assign30290_e47755 * (((3.0 * locals.var_t0_dn21) / 4.0) + ((locals.var_t1_dn21 + locals.var_t2_dn21) / assign30290_e47763)))) * assign30290_e47783) - (assign30290_e47766 * ((-(locals.var_hx_dn21 / (locals.var_hx * locals.var_hx))) + (assign30290_e47772 * ({ let limited_exp_arg = assign30290_e47780; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((assign30290_e47774 * locals.var_vgod_dn21) * assign30290_e47779) - (assign30290_e47776 * (2.0 * locals.var_vtv_dn21))) / (assign30290_e47779 * assign30290_e47779))))))) / (assign30290_e47783 * assign30290_e47783)), (((((((2.0 * locals.var_vtv_dn22) * locals.var_cch) * assign30290_e47765) + (assign30290_e47755 * (((3.0 * locals.var_t0_dn22) / 4.0) + ((locals.var_t1_dn22 + locals.var_t2_dn22) / assign30290_e47763)))) * assign30290_e47783) - (assign30290_e47766 * ((-(locals.var_hx_dn22 / (locals.var_hx * locals.var_hx))) + (assign30290_e47772 * ({ let limited_exp_arg = assign30290_e47780; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((assign30290_e47774 * locals.var_vgod_dn22) * assign30290_e47779) - (assign30290_e47776 * (2.0 * locals.var_vtv_dn22))) / (assign30290_e47779 * assign30290_e47779))))))) / (assign30290_e47783 * assign30290_e47783)),)
    } else {
        (locals.var_ndx, locals.var_ndx_dn0, locals.var_ndx_dn1, locals.var_ndx_dn2, locals.var_ndx_dn3, locals.var_ndx_dn4, locals.var_ndx_dn5, locals.var_ndx_dn6, locals.var_ndx_dn7, locals.var_ndx_dn8, locals.var_ndx_dn9, locals.var_ndx_dn12, locals.var_ndx_dn14, locals.var_ndx_dn15, locals.var_ndx_dn16, locals.var_ndx_dn17, locals.var_ndx_dn18, locals.var_ndx_dn19, locals.var_ndx_dn20, locals.var_ndx_dn21, locals.var_ndx_dn22,)
    }
};
        locals.var_ndx = assign30290_e47786;
        locals.var_ndx_dn0 = assign30290_e47786_d_n0;
        locals.var_ndx_dn1 = assign30290_e47786_d_n1;
        locals.var_ndx_dn2 = assign30290_e47786_d_n2;
        locals.var_ndx_dn3 = assign30290_e47786_d_n3;
        locals.var_ndx_dn4 = assign30290_e47786_d_n4;
        locals.var_ndx_dn5 = assign30290_e47786_d_n5;
        locals.var_ndx_dn6 = assign30290_e47786_d_n6;
        locals.var_ndx_dn7 = assign30290_e47786_d_n7;
        locals.var_ndx_dn8 = assign30290_e47786_d_n8;
        locals.var_ndx_dn9 = assign30290_e47786_d_n9;
        locals.var_ndx_dn12 = assign30290_e47786_d_n12;
        locals.var_ndx_dn14 = assign30290_e47786_d_n14;
        locals.var_ndx_dn15 = assign30290_e47786_d_n15;
        locals.var_ndx_dn16 = assign30290_e47786_d_n16;
        locals.var_ndx_dn17 = assign30290_e47786_d_n17;
        locals.var_ndx_dn18 = assign30290_e47786_d_n18;
        locals.var_ndx_dn19 = assign30290_e47786_d_n19;
        locals.var_ndx_dn20 = assign30290_e47786_d_n20;
        locals.var_ndx_dn21 = assign30290_e47786_d_n21;
        locals.var_ndx_dn22 = assign30290_e47786_d_n22;

        let (assign30300_e47824, assign30300_e47824_d_n0, assign30300_e47824_d_n1, assign30300_e47824_d_n2, assign30300_e47824_d_n3, assign30300_e47824_d_n4, assign30300_e47824_d_n5, assign30300_e47824_d_n6, assign30300_e47824_d_n7, assign30300_e47824_d_n8, assign30300_e47824_d_n9, assign30300_e47824_d_n12, assign30300_e47824_d_n14, assign30300_e47824_d_n15, assign30300_e47824_d_n16, assign30300_e47824_d_n17, assign30300_e47824_d_n18, assign30300_e47824_d_n19, assign30300_e47824_d_n20, assign30300_e47824_d_n21, assign30300_e47824_d_n22,) = {
    if (((locals.var_guard504 == 0.0) && (locals.var_guard513 != 0.0)) && (locals.var_guard517 == 0.0)) {
        let assign30300_e47796: f64 = (2.0 * locals.var_vtv);
        let assign30300_e47798: f64 = (assign30300_e47796 * locals.var_cch);
        let assign30300_e47801: f64 = locals.var_t0;
        let assign30300_e47803: f64 = assign30300_e47801;
        let assign30300_e47804: f64 = (assign30300_e47798 * assign30300_e47803);
        let assign30300_e47807: f64 = (1.0 / locals.var_hx);
        let assign30300_e47810: f64 = (locals.var_cch / 3.24e17);
        let assign30300_e47812: f64 = (-1.0);
        let assign30300_e47814: f64 = (assign30300_e47812 * locals.var_vgod);
        let assign30300_e47817: f64 = (2.0 * locals.var_vtv);
        let assign30300_e47818: f64 = (assign30300_e47814 / assign30300_e47817);
        let assign30300_e47819: f64 = { let limited_exp_arg = assign30300_e47818; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign30300_e47820: f64 = (assign30300_e47810 * assign30300_e47819);
        let assign30300_e47821: f64 = (assign30300_e47807 + assign30300_e47820);
        let assign30300_e47822: f64 = (assign30300_e47804 / assign30300_e47821);
        (assign30300_e47822, ((((assign30300_e47798 * locals.var_t0_dn0) * assign30300_e47821) - (assign30300_e47804 * ((-(locals.var_hx_dn0 / (locals.var_hx * locals.var_hx))) + (assign30300_e47810 * ({ let limited_exp_arg = assign30300_e47818; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((assign30300_e47812 * locals.var_vgod_dn0) / assign30300_e47817)))))) / (assign30300_e47821 * assign30300_e47821)), ((((assign30300_e47798 * locals.var_t0_dn1) * assign30300_e47821) - (assign30300_e47804 * ((-(locals.var_hx_dn1 / (locals.var_hx * locals.var_hx))) + (assign30300_e47810 * ({ let limited_exp_arg = assign30300_e47818; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((assign30300_e47812 * locals.var_vgod_dn1) / assign30300_e47817)))))) / (assign30300_e47821 * assign30300_e47821)), ((((assign30300_e47798 * locals.var_t0_dn2) * assign30300_e47821) - (assign30300_e47804 * ((-(locals.var_hx_dn2 / (locals.var_hx * locals.var_hx))) + (assign30300_e47810 * ({ let limited_exp_arg = assign30300_e47818; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((assign30300_e47812 * locals.var_vgod_dn2) / assign30300_e47817)))))) / (assign30300_e47821 * assign30300_e47821)), ((((assign30300_e47798 * locals.var_t0_dn3) * assign30300_e47821) - (assign30300_e47804 * ((-(locals.var_hx_dn3 / (locals.var_hx * locals.var_hx))) + (assign30300_e47810 * ({ let limited_exp_arg = assign30300_e47818; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((assign30300_e47812 * locals.var_vgod_dn3) / assign30300_e47817)))))) / (assign30300_e47821 * assign30300_e47821)), (((((((2.0 * locals.var_vtv_dn4) * locals.var_cch) * assign30300_e47803) + (assign30300_e47798 * locals.var_t0_dn4)) * assign30300_e47821) - (assign30300_e47804 * ((-(locals.var_hx_dn4 / (locals.var_hx * locals.var_hx))) + (assign30300_e47810 * ({ let limited_exp_arg = assign30300_e47818; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((assign30300_e47812 * locals.var_vgod_dn4) * assign30300_e47817) - (assign30300_e47814 * (2.0 * locals.var_vtv_dn4))) / (assign30300_e47817 * assign30300_e47817))))))) / (assign30300_e47821 * assign30300_e47821)), ((((assign30300_e47798 * locals.var_t0_dn5) * assign30300_e47821) - (assign30300_e47804 * ((-(locals.var_hx_dn5 / (locals.var_hx * locals.var_hx))) + (assign30300_e47810 * ({ let limited_exp_arg = assign30300_e47818; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((assign30300_e47812 * locals.var_vgod_dn5) / assign30300_e47817)))))) / (assign30300_e47821 * assign30300_e47821)), (((((((2.0 * locals.var_vtv_dn6) * locals.var_cch) * assign30300_e47803) + (assign30300_e47798 * locals.var_t0_dn6)) * assign30300_e47821) - (assign30300_e47804 * ((-(locals.var_hx_dn6 / (locals.var_hx * locals.var_hx))) + (assign30300_e47810 * ({ let limited_exp_arg = assign30300_e47818; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((assign30300_e47812 * locals.var_vgod_dn6) * assign30300_e47817) - (assign30300_e47814 * (2.0 * locals.var_vtv_dn6))) / (assign30300_e47817 * assign30300_e47817))))))) / (assign30300_e47821 * assign30300_e47821)), (((((((2.0 * locals.var_vtv_dn7) * locals.var_cch) * assign30300_e47803) + (assign30300_e47798 * locals.var_t0_dn7)) * assign30300_e47821) - (assign30300_e47804 * ((-(locals.var_hx_dn7 / (locals.var_hx * locals.var_hx))) + (assign30300_e47810 * ({ let limited_exp_arg = assign30300_e47818; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((assign30300_e47812 * locals.var_vgod_dn7) * assign30300_e47817) - (assign30300_e47814 * (2.0 * locals.var_vtv_dn7))) / (assign30300_e47817 * assign30300_e47817))))))) / (assign30300_e47821 * assign30300_e47821)), (((((((2.0 * locals.var_vtv_dn8) * locals.var_cch) * assign30300_e47803) + (assign30300_e47798 * locals.var_t0_dn8)) * assign30300_e47821) - (assign30300_e47804 * ((-(locals.var_hx_dn8 / (locals.var_hx * locals.var_hx))) + (assign30300_e47810 * ({ let limited_exp_arg = assign30300_e47818; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((assign30300_e47812 * locals.var_vgod_dn8) * assign30300_e47817) - (assign30300_e47814 * (2.0 * locals.var_vtv_dn8))) / (assign30300_e47817 * assign30300_e47817))))))) / (assign30300_e47821 * assign30300_e47821)), ((((assign30300_e47798 * locals.var_t0_dn9) * assign30300_e47821) - (assign30300_e47804 * ((-(locals.var_hx_dn9 / (locals.var_hx * locals.var_hx))) + (assign30300_e47810 * ({ let limited_exp_arg = assign30300_e47818; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((assign30300_e47812 * locals.var_vgod_dn9) / assign30300_e47817)))))) / (assign30300_e47821 * assign30300_e47821)), ((((assign30300_e47798 * locals.var_t0_dn12) * assign30300_e47821) - (assign30300_e47804 * ((-(locals.var_hx_dn12 / (locals.var_hx * locals.var_hx))) + (assign30300_e47810 * ({ let limited_exp_arg = assign30300_e47818; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((assign30300_e47812 * locals.var_vgod_dn12) / assign30300_e47817)))))) / (assign30300_e47821 * assign30300_e47821)), ((((assign30300_e47798 * locals.var_t0_dn14) * assign30300_e47821) - (assign30300_e47804 * ((-(locals.var_hx_dn14 / (locals.var_hx * locals.var_hx))) + (assign30300_e47810 * ({ let limited_exp_arg = assign30300_e47818; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((assign30300_e47812 * locals.var_vgod_dn14) / assign30300_e47817)))))) / (assign30300_e47821 * assign30300_e47821)), (((((((2.0 * locals.var_vtv_dn15) * locals.var_cch) * assign30300_e47803) + (assign30300_e47798 * locals.var_t0_dn15)) * assign30300_e47821) - (assign30300_e47804 * ((-(locals.var_hx_dn15 / (locals.var_hx * locals.var_hx))) + (assign30300_e47810 * ({ let limited_exp_arg = assign30300_e47818; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((assign30300_e47812 * locals.var_vgod_dn15) * assign30300_e47817) - (assign30300_e47814 * (2.0 * locals.var_vtv_dn15))) / (assign30300_e47817 * assign30300_e47817))))))) / (assign30300_e47821 * assign30300_e47821)), (((((((2.0 * locals.var_vtv_dn16) * locals.var_cch) * assign30300_e47803) + (assign30300_e47798 * locals.var_t0_dn16)) * assign30300_e47821) - (assign30300_e47804 * ((-(locals.var_hx_dn16 / (locals.var_hx * locals.var_hx))) + (assign30300_e47810 * ({ let limited_exp_arg = assign30300_e47818; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((assign30300_e47812 * locals.var_vgod_dn16) * assign30300_e47817) - (assign30300_e47814 * (2.0 * locals.var_vtv_dn16))) / (assign30300_e47817 * assign30300_e47817))))))) / (assign30300_e47821 * assign30300_e47821)), (((((((2.0 * locals.var_vtv_dn17) * locals.var_cch) * assign30300_e47803) + (assign30300_e47798 * locals.var_t0_dn17)) * assign30300_e47821) - (assign30300_e47804 * ((-(locals.var_hx_dn17 / (locals.var_hx * locals.var_hx))) + (assign30300_e47810 * ({ let limited_exp_arg = assign30300_e47818; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((assign30300_e47812 * locals.var_vgod_dn17) * assign30300_e47817) - (assign30300_e47814 * (2.0 * locals.var_vtv_dn17))) / (assign30300_e47817 * assign30300_e47817))))))) / (assign30300_e47821 * assign30300_e47821)), (((((((2.0 * locals.var_vtv_dn18) * locals.var_cch) * assign30300_e47803) + (assign30300_e47798 * locals.var_t0_dn18)) * assign30300_e47821) - (assign30300_e47804 * ((-(locals.var_hx_dn18 / (locals.var_hx * locals.var_hx))) + (assign30300_e47810 * ({ let limited_exp_arg = assign30300_e47818; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((assign30300_e47812 * locals.var_vgod_dn18) * assign30300_e47817) - (assign30300_e47814 * (2.0 * locals.var_vtv_dn18))) / (assign30300_e47817 * assign30300_e47817))))))) / (assign30300_e47821 * assign30300_e47821)), (((((((2.0 * locals.var_vtv_dn19) * locals.var_cch) * assign30300_e47803) + (assign30300_e47798 * locals.var_t0_dn19)) * assign30300_e47821) - (assign30300_e47804 * ((-(locals.var_hx_dn19 / (locals.var_hx * locals.var_hx))) + (assign30300_e47810 * ({ let limited_exp_arg = assign30300_e47818; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((assign30300_e47812 * locals.var_vgod_dn19) * assign30300_e47817) - (assign30300_e47814 * (2.0 * locals.var_vtv_dn19))) / (assign30300_e47817 * assign30300_e47817))))))) / (assign30300_e47821 * assign30300_e47821)), (((((((2.0 * locals.var_vtv_dn20) * locals.var_cch) * assign30300_e47803) + (assign30300_e47798 * locals.var_t0_dn20)) * assign30300_e47821) - (assign30300_e47804 * ((-(locals.var_hx_dn20 / (locals.var_hx * locals.var_hx))) + (assign30300_e47810 * ({ let limited_exp_arg = assign30300_e47818; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((assign30300_e47812 * locals.var_vgod_dn20) * assign30300_e47817) - (assign30300_e47814 * (2.0 * locals.var_vtv_dn20))) / (assign30300_e47817 * assign30300_e47817))))))) / (assign30300_e47821 * assign30300_e47821)), (((((((2.0 * locals.var_vtv_dn21) * locals.var_cch) * assign30300_e47803) + (assign30300_e47798 * locals.var_t0_dn21)) * assign30300_e47821) - (assign30300_e47804 * ((-(locals.var_hx_dn21 / (locals.var_hx * locals.var_hx))) + (assign30300_e47810 * ({ let limited_exp_arg = assign30300_e47818; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((assign30300_e47812 * locals.var_vgod_dn21) * assign30300_e47817) - (assign30300_e47814 * (2.0 * locals.var_vtv_dn21))) / (assign30300_e47817 * assign30300_e47817))))))) / (assign30300_e47821 * assign30300_e47821)), (((((((2.0 * locals.var_vtv_dn22) * locals.var_cch) * assign30300_e47803) + (assign30300_e47798 * locals.var_t0_dn22)) * assign30300_e47821) - (assign30300_e47804 * ((-(locals.var_hx_dn22 / (locals.var_hx * locals.var_hx))) + (assign30300_e47810 * ({ let limited_exp_arg = assign30300_e47818; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((assign30300_e47812 * locals.var_vgod_dn22) * assign30300_e47817) - (assign30300_e47814 * (2.0 * locals.var_vtv_dn22))) / (assign30300_e47817 * assign30300_e47817))))))) / (assign30300_e47821 * assign30300_e47821)),)
    } else {
        (locals.var_ndx, locals.var_ndx_dn0, locals.var_ndx_dn1, locals.var_ndx_dn2, locals.var_ndx_dn3, locals.var_ndx_dn4, locals.var_ndx_dn5, locals.var_ndx_dn6, locals.var_ndx_dn7, locals.var_ndx_dn8, locals.var_ndx_dn9, locals.var_ndx_dn12, locals.var_ndx_dn14, locals.var_ndx_dn15, locals.var_ndx_dn16, locals.var_ndx_dn17, locals.var_ndx_dn18, locals.var_ndx_dn19, locals.var_ndx_dn20, locals.var_ndx_dn21, locals.var_ndx_dn22,)
    }
};
        locals.var_ndx = assign30300_e47824;
        locals.var_ndx_dn0 = assign30300_e47824_d_n0;
        locals.var_ndx_dn1 = assign30300_e47824_d_n1;
        locals.var_ndx_dn2 = assign30300_e47824_d_n2;
        locals.var_ndx_dn3 = assign30300_e47824_d_n3;
        locals.var_ndx_dn4 = assign30300_e47824_d_n4;
        locals.var_ndx_dn5 = assign30300_e47824_d_n5;
        locals.var_ndx_dn6 = assign30300_e47824_d_n6;
        locals.var_ndx_dn7 = assign30300_e47824_d_n7;
        locals.var_ndx_dn8 = assign30300_e47824_d_n8;
        locals.var_ndx_dn9 = assign30300_e47824_d_n9;
        locals.var_ndx_dn12 = assign30300_e47824_d_n12;
        locals.var_ndx_dn14 = assign30300_e47824_d_n14;
        locals.var_ndx_dn15 = assign30300_e47824_d_n15;
        locals.var_ndx_dn16 = assign30300_e47824_d_n16;
        locals.var_ndx_dn17 = assign30300_e47824_d_n17;
        locals.var_ndx_dn18 = assign30300_e47824_d_n18;
        locals.var_ndx_dn19 = assign30300_e47824_d_n19;
        locals.var_ndx_dn20 = assign30300_e47824_d_n20;
        locals.var_ndx_dn21 = assign30300_e47824_d_n21;
        locals.var_ndx_dn22 = assign30300_e47824_d_n22;

        let (assign30310_e47835, assign30310_e47835_d_n0, assign30310_e47835_d_n1, assign30310_e47835_d_n2, assign30310_e47835_d_n3, assign30310_e47835_d_n4, assign30310_e47835_d_n5, assign30310_e47835_d_n6, assign30310_e47835_d_n7, assign30310_e47835_d_n8, assign30310_e47835_d_n9, assign30310_e47835_d_n12, assign30310_e47835_d_n14, assign30310_e47835_d_n15, assign30310_e47835_d_n16, assign30310_e47835_d_n17, assign30310_e47835_d_n18, assign30310_e47835_d_n19, assign30310_e47835_d_n20, assign30310_e47835_d_n21, assign30310_e47835_d_n22,) = {
    if ((locals.var_guard504 == 0.0) && (locals.var_guard513 != 0.0)) {
        let assign30310_e47832: f64 = (locals.var_ndx / locals.var_cch);
        let assign30310_e47833: f64 = (locals.var_vgod - assign30310_e47832);
        (assign30310_e47833, (locals.var_vgod_dn0 - (locals.var_ndx_dn0 / locals.var_cch)), (locals.var_vgod_dn1 - (locals.var_ndx_dn1 / locals.var_cch)), (locals.var_vgod_dn2 - (locals.var_ndx_dn2 / locals.var_cch)), (locals.var_vgod_dn3 - (locals.var_ndx_dn3 / locals.var_cch)), (locals.var_vgod_dn4 - (locals.var_ndx_dn4 / locals.var_cch)), (locals.var_vgod_dn5 - (locals.var_ndx_dn5 / locals.var_cch)), (locals.var_vgod_dn6 - (locals.var_ndx_dn6 / locals.var_cch)), (locals.var_vgod_dn7 - (locals.var_ndx_dn7 / locals.var_cch)), (locals.var_vgod_dn8 - (locals.var_ndx_dn8 / locals.var_cch)), (locals.var_vgod_dn9 - (locals.var_ndx_dn9 / locals.var_cch)), (locals.var_vgod_dn12 - (locals.var_ndx_dn12 / locals.var_cch)), (locals.var_vgod_dn14 - (locals.var_ndx_dn14 / locals.var_cch)), (locals.var_vgod_dn15 - (locals.var_ndx_dn15 / locals.var_cch)), (locals.var_vgod_dn16 - (locals.var_ndx_dn16 / locals.var_cch)), (locals.var_vgod_dn17 - (locals.var_ndx_dn17 / locals.var_cch)), (locals.var_vgod_dn18 - (locals.var_ndx_dn18 / locals.var_cch)), (locals.var_vgod_dn19 - (locals.var_ndx_dn19 / locals.var_cch)), (locals.var_vgod_dn20 - (locals.var_ndx_dn20 / locals.var_cch)), (locals.var_vgod_dn21 - (locals.var_ndx_dn21 / locals.var_cch)), (locals.var_vgod_dn22 - (locals.var_ndx_dn22 / locals.var_cch)),)
    } else {
        (locals.var_ef1, locals.var_ef1_dn0, locals.var_ef1_dn1, locals.var_ef1_dn2, locals.var_ef1_dn3, locals.var_ef1_dn4, locals.var_ef1_dn5, locals.var_ef1_dn6, locals.var_ef1_dn7, locals.var_ef1_dn8, locals.var_ef1_dn9, locals.var_ef1_dn12, locals.var_ef1_dn14, locals.var_ef1_dn15, locals.var_ef1_dn16, locals.var_ef1_dn17, locals.var_ef1_dn18, locals.var_ef1_dn19, locals.var_ef1_dn20, locals.var_ef1_dn21, locals.var_ef1_dn22,)
    }
};
        locals.var_ef1 = assign30310_e47835;
        locals.var_ef1_dn0 = assign30310_e47835_d_n0;
        locals.var_ef1_dn1 = assign30310_e47835_d_n1;
        locals.var_ef1_dn2 = assign30310_e47835_d_n2;
        locals.var_ef1_dn3 = assign30310_e47835_d_n3;
        locals.var_ef1_dn4 = assign30310_e47835_d_n4;
        locals.var_ef1_dn5 = assign30310_e47835_d_n5;
        locals.var_ef1_dn6 = assign30310_e47835_d_n6;
        locals.var_ef1_dn7 = assign30310_e47835_d_n7;
        locals.var_ef1_dn8 = assign30310_e47835_d_n8;
        locals.var_ef1_dn9 = assign30310_e47835_d_n9;
        locals.var_ef1_dn12 = assign30310_e47835_d_n12;
        locals.var_ef1_dn14 = assign30310_e47835_d_n14;
        locals.var_ef1_dn15 = assign30310_e47835_d_n15;
        locals.var_ef1_dn16 = assign30310_e47835_d_n16;
        locals.var_ef1_dn17 = assign30310_e47835_d_n17;
        locals.var_ef1_dn18 = assign30310_e47835_d_n18;
        locals.var_ef1_dn19 = assign30310_e47835_d_n19;
        locals.var_ef1_dn20 = assign30310_e47835_d_n20;
        locals.var_ef1_dn21 = assign30310_e47835_d_n21;
        locals.var_ef1_dn22 = assign30310_e47835_d_n22;

        let assign30320_e47838: f64 = (locals.var_ef1 - locals.var_vgod);
        let assign30320_e47839: f64 = (assign30320_e47838).abs();
        let assign30320_e47841: f64 = if assign30320_e47839 > 1e-19 { 1.0 } else { 0.0 };
        locals.var_guard518 = assign30320_e47841;

    }

    pub(super) fn stamp_transient_block_178(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign30330_e47852, assign30330_e47852_d_n0, assign30330_e47852_d_n1, assign30330_e47852_d_n2, assign30330_e47852_d_n3, assign30330_e47852_d_n4, assign30330_e47852_d_n5, assign30330_e47852_d_n6, assign30330_e47852_d_n7, assign30330_e47852_d_n8, assign30330_e47852_d_n9, assign30330_e47852_d_n12, assign30330_e47852_d_n14, assign30330_e47852_d_n15, assign30330_e47852_d_n16, assign30330_e47852_d_n17, assign30330_e47852_d_n18, assign30330_e47852_d_n19, assign30330_e47852_d_n20, assign30330_e47852_d_n21, assign30330_e47852_d_n22,) = {
    if (((locals.var_guard504 == 0.0) && (locals.var_guard513 != 0.0)) && (locals.var_guard518 != 0.0)) {
        let assign30330_e47850: f64 = (locals.var_vgod - locals.var_ef1);
        (assign30330_e47850, (locals.var_vgod_dn0 - locals.var_ef1_dn0), (locals.var_vgod_dn1 - locals.var_ef1_dn1), (locals.var_vgod_dn2 - locals.var_ef1_dn2), (locals.var_vgod_dn3 - locals.var_ef1_dn3), (locals.var_vgod_dn4 - locals.var_ef1_dn4), (locals.var_vgod_dn5 - locals.var_ef1_dn5), (locals.var_vgod_dn6 - locals.var_ef1_dn6), (locals.var_vgod_dn7 - locals.var_ef1_dn7), (locals.var_vgod_dn8 - locals.var_ef1_dn8), (locals.var_vgod_dn9 - locals.var_ef1_dn9), (locals.var_vgod_dn12 - locals.var_ef1_dn12), (locals.var_vgod_dn14 - locals.var_ef1_dn14), (locals.var_vgod_dn15 - locals.var_ef1_dn15), (locals.var_vgod_dn16 - locals.var_ef1_dn16), (locals.var_vgod_dn17 - locals.var_ef1_dn17), (locals.var_vgod_dn18 - locals.var_ef1_dn18), (locals.var_vgod_dn19 - locals.var_ef1_dn19), (locals.var_vgod_dn20 - locals.var_ef1_dn20), (locals.var_vgod_dn21 - locals.var_ef1_dn21), (locals.var_vgod_dn22 - locals.var_ef1_dn22),)
    } else {
        (locals.var_vgef1, locals.var_vgef1_dn0, locals.var_vgef1_dn1, locals.var_vgef1_dn2, locals.var_vgef1_dn3, locals.var_vgef1_dn4, locals.var_vgef1_dn5, locals.var_vgef1_dn6, locals.var_vgef1_dn7, locals.var_vgef1_dn8, locals.var_vgef1_dn9, locals.var_vgef1_dn12, locals.var_vgef1_dn14, locals.var_vgef1_dn15, locals.var_vgef1_dn16, locals.var_vgef1_dn17, locals.var_vgef1_dn18, locals.var_vgef1_dn19, locals.var_vgef1_dn20, locals.var_vgef1_dn21, locals.var_vgef1_dn22,)
    }
};
        locals.var_vgef1 = assign30330_e47852;
        locals.var_vgef1_dn0 = assign30330_e47852_d_n0;
        locals.var_vgef1_dn1 = assign30330_e47852_d_n1;
        locals.var_vgef1_dn2 = assign30330_e47852_d_n2;
        locals.var_vgef1_dn3 = assign30330_e47852_d_n3;
        locals.var_vgef1_dn4 = assign30330_e47852_d_n4;
        locals.var_vgef1_dn5 = assign30330_e47852_d_n5;
        locals.var_vgef1_dn6 = assign30330_e47852_d_n6;
        locals.var_vgef1_dn7 = assign30330_e47852_d_n7;
        locals.var_vgef1_dn8 = assign30330_e47852_d_n8;
        locals.var_vgef1_dn9 = assign30330_e47852_d_n9;
        locals.var_vgef1_dn12 = assign30330_e47852_d_n12;
        locals.var_vgef1_dn14 = assign30330_e47852_d_n14;
        locals.var_vgef1_dn15 = assign30330_e47852_d_n15;
        locals.var_vgef1_dn16 = assign30330_e47852_d_n16;
        locals.var_vgef1_dn17 = assign30330_e47852_d_n17;
        locals.var_vgef1_dn18 = assign30330_e47852_d_n18;
        locals.var_vgef1_dn19 = assign30330_e47852_d_n19;
        locals.var_vgef1_dn20 = assign30330_e47852_d_n20;
        locals.var_vgef1_dn21 = assign30330_e47852_d_n21;
        locals.var_vgef1_dn22 = assign30330_e47852_d_n22;

        let (assign30340_e47876, assign30340_e47876_d_n0, assign30340_e47876_d_n1, assign30340_e47876_d_n2, assign30340_e47876_d_n3, assign30340_e47876_d_n4, assign30340_e47876_d_n5, assign30340_e47876_d_n6, assign30340_e47876_d_n7, assign30340_e47876_d_n8, assign30340_e47876_d_n9, assign30340_e47876_d_n12, assign30340_e47876_d_n14, assign30340_e47876_d_n15, assign30340_e47876_d_n16, assign30340_e47876_d_n17, assign30340_e47876_d_n18, assign30340_e47876_d_n19, assign30340_e47876_d_n20, assign30340_e47876_d_n21, assign30340_e47876_d_n22,) = {
    if (((locals.var_guard504 == 0.0) && (locals.var_guard513 != 0.0)) && (locals.var_guard518 != 0.0)) {
        let assign30340_e47861: f64 = (0.5 * locals.var_vgef1);
        let assign30340_e47865: f64 = (locals.var_vgef1 * locals.var_vgef1);
        let assign30340_e47868: f64 = (4.0 * 1e-9);
        let assign30340_e47870: f64 = (assign30340_e47868 * 1e-9);
        let assign30340_e47871: f64 = (assign30340_e47865 + assign30340_e47870);
        let assign30340_e47872: f64 = (assign30340_e47871).sqrt();
        let assign30340_e47873: f64 = (0.5 * assign30340_e47872);
        let assign30340_e47874: f64 = (assign30340_e47861 + assign30340_e47873);
        (assign30340_e47874, ((0.5 * locals.var_vgef1_dn0) + (0.5 * (((locals.var_vgef1_dn0 * locals.var_vgef1) + (locals.var_vgef1 * locals.var_vgef1_dn0)) / (2.0 * assign30340_e47872)))), ((0.5 * locals.var_vgef1_dn1) + (0.5 * (((locals.var_vgef1_dn1 * locals.var_vgef1) + (locals.var_vgef1 * locals.var_vgef1_dn1)) / (2.0 * assign30340_e47872)))), ((0.5 * locals.var_vgef1_dn2) + (0.5 * (((locals.var_vgef1_dn2 * locals.var_vgef1) + (locals.var_vgef1 * locals.var_vgef1_dn2)) / (2.0 * assign30340_e47872)))), ((0.5 * locals.var_vgef1_dn3) + (0.5 * (((locals.var_vgef1_dn3 * locals.var_vgef1) + (locals.var_vgef1 * locals.var_vgef1_dn3)) / (2.0 * assign30340_e47872)))), ((0.5 * locals.var_vgef1_dn4) + (0.5 * (((locals.var_vgef1_dn4 * locals.var_vgef1) + (locals.var_vgef1 * locals.var_vgef1_dn4)) / (2.0 * assign30340_e47872)))), ((0.5 * locals.var_vgef1_dn5) + (0.5 * (((locals.var_vgef1_dn5 * locals.var_vgef1) + (locals.var_vgef1 * locals.var_vgef1_dn5)) / (2.0 * assign30340_e47872)))), ((0.5 * locals.var_vgef1_dn6) + (0.5 * (((locals.var_vgef1_dn6 * locals.var_vgef1) + (locals.var_vgef1 * locals.var_vgef1_dn6)) / (2.0 * assign30340_e47872)))), ((0.5 * locals.var_vgef1_dn7) + (0.5 * (((locals.var_vgef1_dn7 * locals.var_vgef1) + (locals.var_vgef1 * locals.var_vgef1_dn7)) / (2.0 * assign30340_e47872)))), ((0.5 * locals.var_vgef1_dn8) + (0.5 * (((locals.var_vgef1_dn8 * locals.var_vgef1) + (locals.var_vgef1 * locals.var_vgef1_dn8)) / (2.0 * assign30340_e47872)))), ((0.5 * locals.var_vgef1_dn9) + (0.5 * (((locals.var_vgef1_dn9 * locals.var_vgef1) + (locals.var_vgef1 * locals.var_vgef1_dn9)) / (2.0 * assign30340_e47872)))), ((0.5 * locals.var_vgef1_dn12) + (0.5 * (((locals.var_vgef1_dn12 * locals.var_vgef1) + (locals.var_vgef1 * locals.var_vgef1_dn12)) / (2.0 * assign30340_e47872)))), ((0.5 * locals.var_vgef1_dn14) + (0.5 * (((locals.var_vgef1_dn14 * locals.var_vgef1) + (locals.var_vgef1 * locals.var_vgef1_dn14)) / (2.0 * assign30340_e47872)))), ((0.5 * locals.var_vgef1_dn15) + (0.5 * (((locals.var_vgef1_dn15 * locals.var_vgef1) + (locals.var_vgef1 * locals.var_vgef1_dn15)) / (2.0 * assign30340_e47872)))), ((0.5 * locals.var_vgef1_dn16) + (0.5 * (((locals.var_vgef1_dn16 * locals.var_vgef1) + (locals.var_vgef1 * locals.var_vgef1_dn16)) / (2.0 * assign30340_e47872)))), ((0.5 * locals.var_vgef1_dn17) + (0.5 * (((locals.var_vgef1_dn17 * locals.var_vgef1) + (locals.var_vgef1 * locals.var_vgef1_dn17)) / (2.0 * assign30340_e47872)))), ((0.5 * locals.var_vgef1_dn18) + (0.5 * (((locals.var_vgef1_dn18 * locals.var_vgef1) + (locals.var_vgef1 * locals.var_vgef1_dn18)) / (2.0 * assign30340_e47872)))), ((0.5 * locals.var_vgef1_dn19) + (0.5 * (((locals.var_vgef1_dn19 * locals.var_vgef1) + (locals.var_vgef1 * locals.var_vgef1_dn19)) / (2.0 * assign30340_e47872)))), ((0.5 * locals.var_vgef1_dn20) + (0.5 * (((locals.var_vgef1_dn20 * locals.var_vgef1) + (locals.var_vgef1 * locals.var_vgef1_dn20)) / (2.0 * assign30340_e47872)))), ((0.5 * locals.var_vgef1_dn21) + (0.5 * (((locals.var_vgef1_dn21 * locals.var_vgef1) + (locals.var_vgef1 * locals.var_vgef1_dn21)) / (2.0 * assign30340_e47872)))), ((0.5 * locals.var_vgef1_dn22) + (0.5 * (((locals.var_vgef1_dn22 * locals.var_vgef1) + (locals.var_vgef1 * locals.var_vgef1_dn22)) / (2.0 * assign30340_e47872)))),)
    } else {
        (locals.var_vgef1, locals.var_vgef1_dn0, locals.var_vgef1_dn1, locals.var_vgef1_dn2, locals.var_vgef1_dn3, locals.var_vgef1_dn4, locals.var_vgef1_dn5, locals.var_vgef1_dn6, locals.var_vgef1_dn7, locals.var_vgef1_dn8, locals.var_vgef1_dn9, locals.var_vgef1_dn12, locals.var_vgef1_dn14, locals.var_vgef1_dn15, locals.var_vgef1_dn16, locals.var_vgef1_dn17, locals.var_vgef1_dn18, locals.var_vgef1_dn19, locals.var_vgef1_dn20, locals.var_vgef1_dn21, locals.var_vgef1_dn22,)
    }
};
        locals.var_vgef1 = assign30340_e47876;
        locals.var_vgef1_dn0 = assign30340_e47876_d_n0;
        locals.var_vgef1_dn1 = assign30340_e47876_d_n1;
        locals.var_vgef1_dn2 = assign30340_e47876_d_n2;
        locals.var_vgef1_dn3 = assign30340_e47876_d_n3;
        locals.var_vgef1_dn4 = assign30340_e47876_d_n4;
        locals.var_vgef1_dn5 = assign30340_e47876_d_n5;
        locals.var_vgef1_dn6 = assign30340_e47876_d_n6;
        locals.var_vgef1_dn7 = assign30340_e47876_d_n7;
        locals.var_vgef1_dn8 = assign30340_e47876_d_n8;
        locals.var_vgef1_dn9 = assign30340_e47876_d_n9;
        locals.var_vgef1_dn12 = assign30340_e47876_d_n12;
        locals.var_vgef1_dn14 = assign30340_e47876_d_n14;
        locals.var_vgef1_dn15 = assign30340_e47876_d_n15;
        locals.var_vgef1_dn16 = assign30340_e47876_d_n16;
        locals.var_vgef1_dn17 = assign30340_e47876_d_n17;
        locals.var_vgef1_dn18 = assign30340_e47876_d_n18;
        locals.var_vgef1_dn19 = assign30340_e47876_d_n19;
        locals.var_vgef1_dn20 = assign30340_e47876_d_n20;
        locals.var_vgef1_dn21 = assign30340_e47876_d_n21;
        locals.var_vgef1_dn22 = assign30340_e47876_d_n22;

        let (assign30350_e47887, assign30350_e47887_d_n0, assign30350_e47887_d_n1, assign30350_e47887_d_n2, assign30350_e47887_d_n3, assign30350_e47887_d_n4, assign30350_e47887_d_n5, assign30350_e47887_d_n6, assign30350_e47887_d_n7, assign30350_e47887_d_n8, assign30350_e47887_d_n9, assign30350_e47887_d_n12, assign30350_e47887_d_n14, assign30350_e47887_d_n15, assign30350_e47887_d_n16, assign30350_e47887_d_n17, assign30350_e47887_d_n18, assign30350_e47887_d_n19, assign30350_e47887_d_n20, assign30350_e47887_d_n21, assign30350_e47887_d_n22,) = {
    if (((locals.var_guard504 == 0.0) && (locals.var_guard513 != 0.0)) && (locals.var_guard518 != 0.0)) {
        let assign30350_e47885: f64 = (locals.var_cch).powf(0.6666666666666666);
        (assign30350_e47885, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn1, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn12, locals.var_t0_dn14, locals.var_t0_dn15, locals.var_t0_dn16, locals.var_t0_dn17, locals.var_t0_dn18, locals.var_t0_dn19, locals.var_t0_dn20, locals.var_t0_dn21, locals.var_t0_dn22,)
    }
};
        locals.var_t0 = assign30350_e47887;
        locals.var_t0_dn0 = assign30350_e47887_d_n0;
        locals.var_t0_dn1 = assign30350_e47887_d_n1;
        locals.var_t0_dn2 = assign30350_e47887_d_n2;
        locals.var_t0_dn3 = assign30350_e47887_d_n3;
        locals.var_t0_dn4 = assign30350_e47887_d_n4;
        locals.var_t0_dn5 = assign30350_e47887_d_n5;
        locals.var_t0_dn6 = assign30350_e47887_d_n6;
        locals.var_t0_dn7 = assign30350_e47887_d_n7;
        locals.var_t0_dn8 = assign30350_e47887_d_n8;
        locals.var_t0_dn9 = assign30350_e47887_d_n9;
        locals.var_t0_dn12 = assign30350_e47887_d_n12;
        locals.var_t0_dn14 = assign30350_e47887_d_n14;
        locals.var_t0_dn15 = assign30350_e47887_d_n15;
        locals.var_t0_dn16 = assign30350_e47887_d_n16;
        locals.var_t0_dn17 = assign30350_e47887_d_n17;
        locals.var_t0_dn18 = assign30350_e47887_d_n18;
        locals.var_t0_dn19 = assign30350_e47887_d_n19;
        locals.var_t0_dn20 = assign30350_e47887_d_n20;
        locals.var_t0_dn21 = assign30350_e47887_d_n21;
        locals.var_t0_dn22 = assign30350_e47887_d_n22;

        let (assign30360_e47898, assign30360_e47898_d_n0, assign30360_e47898_d_n1, assign30360_e47898_d_n2, assign30360_e47898_d_n3, assign30360_e47898_d_n4, assign30360_e47898_d_n5, assign30360_e47898_d_n6, assign30360_e47898_d_n7, assign30360_e47898_d_n8, assign30360_e47898_d_n9, assign30360_e47898_d_n12, assign30360_e47898_d_n14, assign30360_e47898_d_n15, assign30360_e47898_d_n16, assign30360_e47898_d_n17, assign30360_e47898_d_n18, assign30360_e47898_d_n19, assign30360_e47898_d_n20, assign30360_e47898_d_n21, assign30360_e47898_d_n22,) = {
    if (((locals.var_guard504 == 0.0) && (locals.var_guard513 != 0.0)) && (locals.var_guard518 != 0.0)) {
        let assign30360_e47896: f64 = (locals.var_vgef1).powf(0.6666666666666666);
        (assign30360_e47896, if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((locals.var_vgef1).powf(0.6666666666666666 - 1.0) * locals.var_vgef1_dn0)) } } else { (assign30360_e47896 * (0.6666666666666666 * (locals.var_vgef1_dn0 / locals.var_vgef1))) }, if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((locals.var_vgef1).powf(0.6666666666666666 - 1.0) * locals.var_vgef1_dn1)) } } else { (assign30360_e47896 * (0.6666666666666666 * (locals.var_vgef1_dn1 / locals.var_vgef1))) }, if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((locals.var_vgef1).powf(0.6666666666666666 - 1.0) * locals.var_vgef1_dn2)) } } else { (assign30360_e47896 * (0.6666666666666666 * (locals.var_vgef1_dn2 / locals.var_vgef1))) }, if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((locals.var_vgef1).powf(0.6666666666666666 - 1.0) * locals.var_vgef1_dn3)) } } else { (assign30360_e47896 * (0.6666666666666666 * (locals.var_vgef1_dn3 / locals.var_vgef1))) }, if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((locals.var_vgef1).powf(0.6666666666666666 - 1.0) * locals.var_vgef1_dn4)) } } else { (assign30360_e47896 * (0.6666666666666666 * (locals.var_vgef1_dn4 / locals.var_vgef1))) }, if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((locals.var_vgef1).powf(0.6666666666666666 - 1.0) * locals.var_vgef1_dn5)) } } else { (assign30360_e47896 * (0.6666666666666666 * (locals.var_vgef1_dn5 / locals.var_vgef1))) }, if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((locals.var_vgef1).powf(0.6666666666666666 - 1.0) * locals.var_vgef1_dn6)) } } else { (assign30360_e47896 * (0.6666666666666666 * (locals.var_vgef1_dn6 / locals.var_vgef1))) }, if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((locals.var_vgef1).powf(0.6666666666666666 - 1.0) * locals.var_vgef1_dn7)) } } else { (assign30360_e47896 * (0.6666666666666666 * (locals.var_vgef1_dn7 / locals.var_vgef1))) }, if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((locals.var_vgef1).powf(0.6666666666666666 - 1.0) * locals.var_vgef1_dn8)) } } else { (assign30360_e47896 * (0.6666666666666666 * (locals.var_vgef1_dn8 / locals.var_vgef1))) }, if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((locals.var_vgef1).powf(0.6666666666666666 - 1.0) * locals.var_vgef1_dn9)) } } else { (assign30360_e47896 * (0.6666666666666666 * (locals.var_vgef1_dn9 / locals.var_vgef1))) }, if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((locals.var_vgef1).powf(0.6666666666666666 - 1.0) * locals.var_vgef1_dn12)) } } else { (assign30360_e47896 * (0.6666666666666666 * (locals.var_vgef1_dn12 / locals.var_vgef1))) }, if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((locals.var_vgef1).powf(0.6666666666666666 - 1.0) * locals.var_vgef1_dn14)) } } else { (assign30360_e47896 * (0.6666666666666666 * (locals.var_vgef1_dn14 / locals.var_vgef1))) }, if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((locals.var_vgef1).powf(0.6666666666666666 - 1.0) * locals.var_vgef1_dn15)) } } else { (assign30360_e47896 * (0.6666666666666666 * (locals.var_vgef1_dn15 / locals.var_vgef1))) }, if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((locals.var_vgef1).powf(0.6666666666666666 - 1.0) * locals.var_vgef1_dn16)) } } else { (assign30360_e47896 * (0.6666666666666666 * (locals.var_vgef1_dn16 / locals.var_vgef1))) }, if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((locals.var_vgef1).powf(0.6666666666666666 - 1.0) * locals.var_vgef1_dn17)) } } else { (assign30360_e47896 * (0.6666666666666666 * (locals.var_vgef1_dn17 / locals.var_vgef1))) }, if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((locals.var_vgef1).powf(0.6666666666666666 - 1.0) * locals.var_vgef1_dn18)) } } else { (assign30360_e47896 * (0.6666666666666666 * (locals.var_vgef1_dn18 / locals.var_vgef1))) }, if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((locals.var_vgef1).powf(0.6666666666666666 - 1.0) * locals.var_vgef1_dn19)) } } else { (assign30360_e47896 * (0.6666666666666666 * (locals.var_vgef1_dn19 / locals.var_vgef1))) }, if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((locals.var_vgef1).powf(0.6666666666666666 - 1.0) * locals.var_vgef1_dn20)) } } else { (assign30360_e47896 * (0.6666666666666666 * (locals.var_vgef1_dn20 / locals.var_vgef1))) }, if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((locals.var_vgef1).powf(0.6666666666666666 - 1.0) * locals.var_vgef1_dn21)) } } else { (assign30360_e47896 * (0.6666666666666666 * (locals.var_vgef1_dn21 / locals.var_vgef1))) }, if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((locals.var_vgef1).powf(0.6666666666666666 - 1.0) * locals.var_vgef1_dn22)) } } else { (assign30360_e47896 * (0.6666666666666666 * (locals.var_vgef1_dn22 / locals.var_vgef1))) },)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn1, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn12, locals.var_t1_dn14, locals.var_t1_dn15, locals.var_t1_dn16, locals.var_t1_dn17, locals.var_t1_dn18, locals.var_t1_dn19, locals.var_t1_dn20, locals.var_t1_dn21, locals.var_t1_dn22,)
    }
};
        locals.var_t1 = assign30360_e47898;
        locals.var_t1_dn0 = assign30360_e47898_d_n0;
        locals.var_t1_dn1 = assign30360_e47898_d_n1;
        locals.var_t1_dn2 = assign30360_e47898_d_n2;
        locals.var_t1_dn3 = assign30360_e47898_d_n3;
        locals.var_t1_dn4 = assign30360_e47898_d_n4;
        locals.var_t1_dn5 = assign30360_e47898_d_n5;
        locals.var_t1_dn6 = assign30360_e47898_d_n6;
        locals.var_t1_dn7 = assign30360_e47898_d_n7;
        locals.var_t1_dn8 = assign30360_e47898_d_n8;
        locals.var_t1_dn9 = assign30360_e47898_d_n9;
        locals.var_t1_dn12 = assign30360_e47898_d_n12;
        locals.var_t1_dn14 = assign30360_e47898_d_n14;
        locals.var_t1_dn15 = assign30360_e47898_d_n15;
        locals.var_t1_dn16 = assign30360_e47898_d_n16;
        locals.var_t1_dn17 = assign30360_e47898_d_n17;
        locals.var_t1_dn18 = assign30360_e47898_d_n18;
        locals.var_t1_dn19 = assign30360_e47898_d_n19;
        locals.var_t1_dn20 = assign30360_e47898_d_n20;
        locals.var_t1_dn21 = assign30360_e47898_d_n21;
        locals.var_t1_dn22 = assign30360_e47898_d_n22;

        let (assign30370_e47910, assign30370_e47910_d_n0, assign30370_e47910_d_n1, assign30370_e47910_d_n2, assign30370_e47910_d_n3, assign30370_e47910_d_n4, assign30370_e47910_d_n5, assign30370_e47910_d_n6, assign30370_e47910_d_n7, assign30370_e47910_d_n8, assign30370_e47910_d_n9, assign30370_e47910_d_n12, assign30370_e47910_d_n14, assign30370_e47910_d_n15, assign30370_e47910_d_n16, assign30370_e47910_d_n17, assign30370_e47910_d_n18, assign30370_e47910_d_n19, assign30370_e47910_d_n20, assign30370_e47910_d_n21, assign30370_e47910_d_n22,) = {
    if (((locals.var_guard504 == 0.0) && (locals.var_guard513 != 0.0)) && (locals.var_guard518 != 0.0)) {
        let assign30370_e47907: f64 = (-0.3333333333333333);
        let assign30370_e47908: f64 = (locals.var_vgef1).powf(assign30370_e47907);
        (assign30370_e47908, if 0.0 == 0.0 && ((assign30370_e47907) as f64).is_finite() && ((assign30370_e47907) as f64).fract() == 0.0 { if assign30370_e47907 == 0.0 { 0.0 } else { (assign30370_e47907 * ((locals.var_vgef1).powf(assign30370_e47907 - 1.0) * locals.var_vgef1_dn0)) } } else { (assign30370_e47908 * (assign30370_e47907 * (locals.var_vgef1_dn0 / locals.var_vgef1))) }, if 0.0 == 0.0 && ((assign30370_e47907) as f64).is_finite() && ((assign30370_e47907) as f64).fract() == 0.0 { if assign30370_e47907 == 0.0 { 0.0 } else { (assign30370_e47907 * ((locals.var_vgef1).powf(assign30370_e47907 - 1.0) * locals.var_vgef1_dn1)) } } else { (assign30370_e47908 * (assign30370_e47907 * (locals.var_vgef1_dn1 / locals.var_vgef1))) }, if 0.0 == 0.0 && ((assign30370_e47907) as f64).is_finite() && ((assign30370_e47907) as f64).fract() == 0.0 { if assign30370_e47907 == 0.0 { 0.0 } else { (assign30370_e47907 * ((locals.var_vgef1).powf(assign30370_e47907 - 1.0) * locals.var_vgef1_dn2)) } } else { (assign30370_e47908 * (assign30370_e47907 * (locals.var_vgef1_dn2 / locals.var_vgef1))) }, if 0.0 == 0.0 && ((assign30370_e47907) as f64).is_finite() && ((assign30370_e47907) as f64).fract() == 0.0 { if assign30370_e47907 == 0.0 { 0.0 } else { (assign30370_e47907 * ((locals.var_vgef1).powf(assign30370_e47907 - 1.0) * locals.var_vgef1_dn3)) } } else { (assign30370_e47908 * (assign30370_e47907 * (locals.var_vgef1_dn3 / locals.var_vgef1))) }, if 0.0 == 0.0 && ((assign30370_e47907) as f64).is_finite() && ((assign30370_e47907) as f64).fract() == 0.0 { if assign30370_e47907 == 0.0 { 0.0 } else { (assign30370_e47907 * ((locals.var_vgef1).powf(assign30370_e47907 - 1.0) * locals.var_vgef1_dn4)) } } else { (assign30370_e47908 * (assign30370_e47907 * (locals.var_vgef1_dn4 / locals.var_vgef1))) }, if 0.0 == 0.0 && ((assign30370_e47907) as f64).is_finite() && ((assign30370_e47907) as f64).fract() == 0.0 { if assign30370_e47907 == 0.0 { 0.0 } else { (assign30370_e47907 * ((locals.var_vgef1).powf(assign30370_e47907 - 1.0) * locals.var_vgef1_dn5)) } } else { (assign30370_e47908 * (assign30370_e47907 * (locals.var_vgef1_dn5 / locals.var_vgef1))) }, if 0.0 == 0.0 && ((assign30370_e47907) as f64).is_finite() && ((assign30370_e47907) as f64).fract() == 0.0 { if assign30370_e47907 == 0.0 { 0.0 } else { (assign30370_e47907 * ((locals.var_vgef1).powf(assign30370_e47907 - 1.0) * locals.var_vgef1_dn6)) } } else { (assign30370_e47908 * (assign30370_e47907 * (locals.var_vgef1_dn6 / locals.var_vgef1))) }, if 0.0 == 0.0 && ((assign30370_e47907) as f64).is_finite() && ((assign30370_e47907) as f64).fract() == 0.0 { if assign30370_e47907 == 0.0 { 0.0 } else { (assign30370_e47907 * ((locals.var_vgef1).powf(assign30370_e47907 - 1.0) * locals.var_vgef1_dn7)) } } else { (assign30370_e47908 * (assign30370_e47907 * (locals.var_vgef1_dn7 / locals.var_vgef1))) }, if 0.0 == 0.0 && ((assign30370_e47907) as f64).is_finite() && ((assign30370_e47907) as f64).fract() == 0.0 { if assign30370_e47907 == 0.0 { 0.0 } else { (assign30370_e47907 * ((locals.var_vgef1).powf(assign30370_e47907 - 1.0) * locals.var_vgef1_dn8)) } } else { (assign30370_e47908 * (assign30370_e47907 * (locals.var_vgef1_dn8 / locals.var_vgef1))) }, if 0.0 == 0.0 && ((assign30370_e47907) as f64).is_finite() && ((assign30370_e47907) as f64).fract() == 0.0 { if assign30370_e47907 == 0.0 { 0.0 } else { (assign30370_e47907 * ((locals.var_vgef1).powf(assign30370_e47907 - 1.0) * locals.var_vgef1_dn9)) } } else { (assign30370_e47908 * (assign30370_e47907 * (locals.var_vgef1_dn9 / locals.var_vgef1))) }, if 0.0 == 0.0 && ((assign30370_e47907) as f64).is_finite() && ((assign30370_e47907) as f64).fract() == 0.0 { if assign30370_e47907 == 0.0 { 0.0 } else { (assign30370_e47907 * ((locals.var_vgef1).powf(assign30370_e47907 - 1.0) * locals.var_vgef1_dn12)) } } else { (assign30370_e47908 * (assign30370_e47907 * (locals.var_vgef1_dn12 / locals.var_vgef1))) }, if 0.0 == 0.0 && ((assign30370_e47907) as f64).is_finite() && ((assign30370_e47907) as f64).fract() == 0.0 { if assign30370_e47907 == 0.0 { 0.0 } else { (assign30370_e47907 * ((locals.var_vgef1).powf(assign30370_e47907 - 1.0) * locals.var_vgef1_dn14)) } } else { (assign30370_e47908 * (assign30370_e47907 * (locals.var_vgef1_dn14 / locals.var_vgef1))) }, if 0.0 == 0.0 && ((assign30370_e47907) as f64).is_finite() && ((assign30370_e47907) as f64).fract() == 0.0 { if assign30370_e47907 == 0.0 { 0.0 } else { (assign30370_e47907 * ((locals.var_vgef1).powf(assign30370_e47907 - 1.0) * locals.var_vgef1_dn15)) } } else { (assign30370_e47908 * (assign30370_e47907 * (locals.var_vgef1_dn15 / locals.var_vgef1))) }, if 0.0 == 0.0 && ((assign30370_e47907) as f64).is_finite() && ((assign30370_e47907) as f64).fract() == 0.0 { if assign30370_e47907 == 0.0 { 0.0 } else { (assign30370_e47907 * ((locals.var_vgef1).powf(assign30370_e47907 - 1.0) * locals.var_vgef1_dn16)) } } else { (assign30370_e47908 * (assign30370_e47907 * (locals.var_vgef1_dn16 / locals.var_vgef1))) }, if 0.0 == 0.0 && ((assign30370_e47907) as f64).is_finite() && ((assign30370_e47907) as f64).fract() == 0.0 { if assign30370_e47907 == 0.0 { 0.0 } else { (assign30370_e47907 * ((locals.var_vgef1).powf(assign30370_e47907 - 1.0) * locals.var_vgef1_dn17)) } } else { (assign30370_e47908 * (assign30370_e47907 * (locals.var_vgef1_dn17 / locals.var_vgef1))) }, if 0.0 == 0.0 && ((assign30370_e47907) as f64).is_finite() && ((assign30370_e47907) as f64).fract() == 0.0 { if assign30370_e47907 == 0.0 { 0.0 } else { (assign30370_e47907 * ((locals.var_vgef1).powf(assign30370_e47907 - 1.0) * locals.var_vgef1_dn18)) } } else { (assign30370_e47908 * (assign30370_e47907 * (locals.var_vgef1_dn18 / locals.var_vgef1))) }, if 0.0 == 0.0 && ((assign30370_e47907) as f64).is_finite() && ((assign30370_e47907) as f64).fract() == 0.0 { if assign30370_e47907 == 0.0 { 0.0 } else { (assign30370_e47907 * ((locals.var_vgef1).powf(assign30370_e47907 - 1.0) * locals.var_vgef1_dn19)) } } else { (assign30370_e47908 * (assign30370_e47907 * (locals.var_vgef1_dn19 / locals.var_vgef1))) }, if 0.0 == 0.0 && ((assign30370_e47907) as f64).is_finite() && ((assign30370_e47907) as f64).fract() == 0.0 { if assign30370_e47907 == 0.0 { 0.0 } else { (assign30370_e47907 * ((locals.var_vgef1).powf(assign30370_e47907 - 1.0) * locals.var_vgef1_dn20)) } } else { (assign30370_e47908 * (assign30370_e47907 * (locals.var_vgef1_dn20 / locals.var_vgef1))) }, if 0.0 == 0.0 && ((assign30370_e47907) as f64).is_finite() && ((assign30370_e47907) as f64).fract() == 0.0 { if assign30370_e47907 == 0.0 { 0.0 } else { (assign30370_e47907 * ((locals.var_vgef1).powf(assign30370_e47907 - 1.0) * locals.var_vgef1_dn21)) } } else { (assign30370_e47908 * (assign30370_e47907 * (locals.var_vgef1_dn21 / locals.var_vgef1))) }, if 0.0 == 0.0 && ((assign30370_e47907) as f64).is_finite() && ((assign30370_e47907) as f64).fract() == 0.0 { if assign30370_e47907 == 0.0 { 0.0 } else { (assign30370_e47907 * ((locals.var_vgef1).powf(assign30370_e47907 - 1.0) * locals.var_vgef1_dn22)) } } else { (assign30370_e47908 * (assign30370_e47907 * (locals.var_vgef1_dn22 / locals.var_vgef1))) },)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn1, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn12, locals.var_t2_dn14, locals.var_t2_dn15, locals.var_t2_dn16, locals.var_t2_dn17, locals.var_t2_dn18, locals.var_t2_dn19, locals.var_t2_dn20, locals.var_t2_dn21, locals.var_t2_dn22,)
    }
};
        locals.var_t2 = assign30370_e47910;
        locals.var_t2_dn0 = assign30370_e47910_d_n0;
        locals.var_t2_dn1 = assign30370_e47910_d_n1;
        locals.var_t2_dn2 = assign30370_e47910_d_n2;
        locals.var_t2_dn3 = assign30370_e47910_d_n3;
        locals.var_t2_dn4 = assign30370_e47910_d_n4;
        locals.var_t2_dn5 = assign30370_e47910_d_n5;
        locals.var_t2_dn6 = assign30370_e47910_d_n6;
        locals.var_t2_dn7 = assign30370_e47910_d_n7;
        locals.var_t2_dn8 = assign30370_e47910_d_n8;
        locals.var_t2_dn9 = assign30370_e47910_d_n9;
        locals.var_t2_dn12 = assign30370_e47910_d_n12;
        locals.var_t2_dn14 = assign30370_e47910_d_n14;
        locals.var_t2_dn15 = assign30370_e47910_d_n15;
        locals.var_t2_dn16 = assign30370_e47910_d_n16;
        locals.var_t2_dn17 = assign30370_e47910_d_n17;
        locals.var_t2_dn18 = assign30370_e47910_d_n18;
        locals.var_t2_dn19 = assign30370_e47910_d_n19;
        locals.var_t2_dn20 = assign30370_e47910_d_n20;
        locals.var_t2_dn21 = assign30370_e47910_d_n21;
        locals.var_t2_dn22 = assign30370_e47910_d_n22;

        let (assign30380_e47923, assign30380_e47923_d_n0, assign30380_e47923_d_n1, assign30380_e47923_d_n2, assign30380_e47923_d_n3, assign30380_e47923_d_n4, assign30380_e47923_d_n5, assign30380_e47923_d_n6, assign30380_e47923_d_n7, assign30380_e47923_d_n8, assign30380_e47923_d_n9, assign30380_e47923_d_n12, assign30380_e47923_d_n14, assign30380_e47923_d_n15, assign30380_e47923_d_n16, assign30380_e47923_d_n17, assign30380_e47923_d_n18, assign30380_e47923_d_n19, assign30380_e47923_d_n20, assign30380_e47923_d_n21, assign30380_e47923_d_n22,) = {
    if (((locals.var_guard504 == 0.0) && (locals.var_guard513 != 0.0)) && (locals.var_guard518 != 0.0)) {
        let assign30380_e47919: f64 = (p.p208 * locals.var_t0);
        let assign30380_e47921: f64 = (assign30380_e47919 * locals.var_t1);
        (assign30380_e47921, (((p.p208 * locals.var_t0_dn0) * locals.var_t1) + (assign30380_e47919 * locals.var_t1_dn0)), (((p.p208 * locals.var_t0_dn1) * locals.var_t1) + (assign30380_e47919 * locals.var_t1_dn1)), (((p.p208 * locals.var_t0_dn2) * locals.var_t1) + (assign30380_e47919 * locals.var_t1_dn2)), (((p.p208 * locals.var_t0_dn3) * locals.var_t1) + (assign30380_e47919 * locals.var_t1_dn3)), (((p.p208 * locals.var_t0_dn4) * locals.var_t1) + (assign30380_e47919 * locals.var_t1_dn4)), (((p.p208 * locals.var_t0_dn5) * locals.var_t1) + (assign30380_e47919 * locals.var_t1_dn5)), (((p.p208 * locals.var_t0_dn6) * locals.var_t1) + (assign30380_e47919 * locals.var_t1_dn6)), (((p.p208 * locals.var_t0_dn7) * locals.var_t1) + (assign30380_e47919 * locals.var_t1_dn7)), (((p.p208 * locals.var_t0_dn8) * locals.var_t1) + (assign30380_e47919 * locals.var_t1_dn8)), (((p.p208 * locals.var_t0_dn9) * locals.var_t1) + (assign30380_e47919 * locals.var_t1_dn9)), (((p.p208 * locals.var_t0_dn12) * locals.var_t1) + (assign30380_e47919 * locals.var_t1_dn12)), (((p.p208 * locals.var_t0_dn14) * locals.var_t1) + (assign30380_e47919 * locals.var_t1_dn14)), (((p.p208 * locals.var_t0_dn15) * locals.var_t1) + (assign30380_e47919 * locals.var_t1_dn15)), (((p.p208 * locals.var_t0_dn16) * locals.var_t1) + (assign30380_e47919 * locals.var_t1_dn16)), (((p.p208 * locals.var_t0_dn17) * locals.var_t1) + (assign30380_e47919 * locals.var_t1_dn17)), (((p.p208 * locals.var_t0_dn18) * locals.var_t1) + (assign30380_e47919 * locals.var_t1_dn18)), (((p.p208 * locals.var_t0_dn19) * locals.var_t1) + (assign30380_e47919 * locals.var_t1_dn19)), (((p.p208 * locals.var_t0_dn20) * locals.var_t1) + (assign30380_e47919 * locals.var_t1_dn20)), (((p.p208 * locals.var_t0_dn21) * locals.var_t1) + (assign30380_e47919 * locals.var_t1_dn21)), (((p.p208 * locals.var_t0_dn22) * locals.var_t1) + (assign30380_e47919 * locals.var_t1_dn22)),)
    } else {
        (locals.var_vgef23g0, locals.var_vgef23g0_dn0, locals.var_vgef23g0_dn1, locals.var_vgef23g0_dn2, locals.var_vgef23g0_dn3, locals.var_vgef23g0_dn4, locals.var_vgef23g0_dn5, locals.var_vgef23g0_dn6, locals.var_vgef23g0_dn7, locals.var_vgef23g0_dn8, locals.var_vgef23g0_dn9, locals.var_vgef23g0_dn12, locals.var_vgef23g0_dn14, locals.var_vgef23g0_dn15, locals.var_vgef23g0_dn16, locals.var_vgef23g0_dn17, locals.var_vgef23g0_dn18, locals.var_vgef23g0_dn19, locals.var_vgef23g0_dn20, locals.var_vgef23g0_dn21, locals.var_vgef23g0_dn22,)
    }
};
        locals.var_vgef23g0 = assign30380_e47923;
        locals.var_vgef23g0_dn0 = assign30380_e47923_d_n0;
        locals.var_vgef23g0_dn1 = assign30380_e47923_d_n1;
        locals.var_vgef23g0_dn2 = assign30380_e47923_d_n2;
        locals.var_vgef23g0_dn3 = assign30380_e47923_d_n3;
        locals.var_vgef23g0_dn4 = assign30380_e47923_d_n4;
        locals.var_vgef23g0_dn5 = assign30380_e47923_d_n5;
        locals.var_vgef23g0_dn6 = assign30380_e47923_d_n6;
        locals.var_vgef23g0_dn7 = assign30380_e47923_d_n7;
        locals.var_vgef23g0_dn8 = assign30380_e47923_d_n8;
        locals.var_vgef23g0_dn9 = assign30380_e47923_d_n9;
        locals.var_vgef23g0_dn12 = assign30380_e47923_d_n12;
        locals.var_vgef23g0_dn14 = assign30380_e47923_d_n14;
        locals.var_vgef23g0_dn15 = assign30380_e47923_d_n15;
        locals.var_vgef23g0_dn16 = assign30380_e47923_d_n16;
        locals.var_vgef23g0_dn17 = assign30380_e47923_d_n17;
        locals.var_vgef23g0_dn18 = assign30380_e47923_d_n18;
        locals.var_vgef23g0_dn19 = assign30380_e47923_d_n19;
        locals.var_vgef23g0_dn20 = assign30380_e47923_d_n20;
        locals.var_vgef23g0_dn21 = assign30380_e47923_d_n21;
        locals.var_vgef23g0_dn22 = assign30380_e47923_d_n22;

        let (assign30390_e47936, assign30390_e47936_d_n0, assign30390_e47936_d_n1, assign30390_e47936_d_n2, assign30390_e47936_d_n3, assign30390_e47936_d_n4, assign30390_e47936_d_n5, assign30390_e47936_d_n6, assign30390_e47936_d_n7, assign30390_e47936_d_n8, assign30390_e47936_d_n9, assign30390_e47936_d_n12, assign30390_e47936_d_n14, assign30390_e47936_d_n15, assign30390_e47936_d_n16, assign30390_e47936_d_n17, assign30390_e47936_d_n18, assign30390_e47936_d_n19, assign30390_e47936_d_n20, assign30390_e47936_d_n21, assign30390_e47936_d_n22,) = {
    if (((locals.var_guard504 == 0.0) && (locals.var_guard513 != 0.0)) && (locals.var_guard518 != 0.0)) {
        let assign30390_e47932: f64 = (p.p209 * locals.var_t0);
        let assign30390_e47934: f64 = (assign30390_e47932 * locals.var_t1);
        (assign30390_e47934, (((p.p209 * locals.var_t0_dn0) * locals.var_t1) + (assign30390_e47932 * locals.var_t1_dn0)), (((p.p209 * locals.var_t0_dn1) * locals.var_t1) + (assign30390_e47932 * locals.var_t1_dn1)), (((p.p209 * locals.var_t0_dn2) * locals.var_t1) + (assign30390_e47932 * locals.var_t1_dn2)), (((p.p209 * locals.var_t0_dn3) * locals.var_t1) + (assign30390_e47932 * locals.var_t1_dn3)), (((p.p209 * locals.var_t0_dn4) * locals.var_t1) + (assign30390_e47932 * locals.var_t1_dn4)), (((p.p209 * locals.var_t0_dn5) * locals.var_t1) + (assign30390_e47932 * locals.var_t1_dn5)), (((p.p209 * locals.var_t0_dn6) * locals.var_t1) + (assign30390_e47932 * locals.var_t1_dn6)), (((p.p209 * locals.var_t0_dn7) * locals.var_t1) + (assign30390_e47932 * locals.var_t1_dn7)), (((p.p209 * locals.var_t0_dn8) * locals.var_t1) + (assign30390_e47932 * locals.var_t1_dn8)), (((p.p209 * locals.var_t0_dn9) * locals.var_t1) + (assign30390_e47932 * locals.var_t1_dn9)), (((p.p209 * locals.var_t0_dn12) * locals.var_t1) + (assign30390_e47932 * locals.var_t1_dn12)), (((p.p209 * locals.var_t0_dn14) * locals.var_t1) + (assign30390_e47932 * locals.var_t1_dn14)), (((p.p209 * locals.var_t0_dn15) * locals.var_t1) + (assign30390_e47932 * locals.var_t1_dn15)), (((p.p209 * locals.var_t0_dn16) * locals.var_t1) + (assign30390_e47932 * locals.var_t1_dn16)), (((p.p209 * locals.var_t0_dn17) * locals.var_t1) + (assign30390_e47932 * locals.var_t1_dn17)), (((p.p209 * locals.var_t0_dn18) * locals.var_t1) + (assign30390_e47932 * locals.var_t1_dn18)), (((p.p209 * locals.var_t0_dn19) * locals.var_t1) + (assign30390_e47932 * locals.var_t1_dn19)), (((p.p209 * locals.var_t0_dn20) * locals.var_t1) + (assign30390_e47932 * locals.var_t1_dn20)), (((p.p209 * locals.var_t0_dn21) * locals.var_t1) + (assign30390_e47932 * locals.var_t1_dn21)), (((p.p209 * locals.var_t0_dn22) * locals.var_t1) + (assign30390_e47932 * locals.var_t1_dn22)),)
    } else {
        (locals.var_vgef23g1, locals.var_vgef23g1_dn0, locals.var_vgef23g1_dn1, locals.var_vgef23g1_dn2, locals.var_vgef23g1_dn3, locals.var_vgef23g1_dn4, locals.var_vgef23g1_dn5, locals.var_vgef23g1_dn6, locals.var_vgef23g1_dn7, locals.var_vgef23g1_dn8, locals.var_vgef23g1_dn9, locals.var_vgef23g1_dn12, locals.var_vgef23g1_dn14, locals.var_vgef23g1_dn15, locals.var_vgef23g1_dn16, locals.var_vgef23g1_dn17, locals.var_vgef23g1_dn18, locals.var_vgef23g1_dn19, locals.var_vgef23g1_dn20, locals.var_vgef23g1_dn21, locals.var_vgef23g1_dn22,)
    }
};
        locals.var_vgef23g1 = assign30390_e47936;
        locals.var_vgef23g1_dn0 = assign30390_e47936_d_n0;
        locals.var_vgef23g1_dn1 = assign30390_e47936_d_n1;
        locals.var_vgef23g1_dn2 = assign30390_e47936_d_n2;
        locals.var_vgef23g1_dn3 = assign30390_e47936_d_n3;
        locals.var_vgef23g1_dn4 = assign30390_e47936_d_n4;
        locals.var_vgef23g1_dn5 = assign30390_e47936_d_n5;
        locals.var_vgef23g1_dn6 = assign30390_e47936_d_n6;
        locals.var_vgef23g1_dn7 = assign30390_e47936_d_n7;
        locals.var_vgef23g1_dn8 = assign30390_e47936_d_n8;
        locals.var_vgef23g1_dn9 = assign30390_e47936_d_n9;
        locals.var_vgef23g1_dn12 = assign30390_e47936_d_n12;
        locals.var_vgef23g1_dn14 = assign30390_e47936_d_n14;
        locals.var_vgef23g1_dn15 = assign30390_e47936_d_n15;
        locals.var_vgef23g1_dn16 = assign30390_e47936_d_n16;
        locals.var_vgef23g1_dn17 = assign30390_e47936_d_n17;
        locals.var_vgef23g1_dn18 = assign30390_e47936_d_n18;
        locals.var_vgef23g1_dn19 = assign30390_e47936_d_n19;
        locals.var_vgef23g1_dn20 = assign30390_e47936_d_n20;
        locals.var_vgef23g1_dn21 = assign30390_e47936_d_n21;
        locals.var_vgef23g1_dn22 = assign30390_e47936_d_n22;

        let (assign30400_e47951, assign30400_e47951_d_n0, assign30400_e47951_d_n1, assign30400_e47951_d_n2, assign30400_e47951_d_n3, assign30400_e47951_d_n4, assign30400_e47951_d_n5, assign30400_e47951_d_n6, assign30400_e47951_d_n7, assign30400_e47951_d_n8, assign30400_e47951_d_n9, assign30400_e47951_d_n12, assign30400_e47951_d_n14, assign30400_e47951_d_n15, assign30400_e47951_d_n16, assign30400_e47951_d_n17, assign30400_e47951_d_n18, assign30400_e47951_d_n19, assign30400_e47951_d_n20, assign30400_e47951_d_n21, assign30400_e47951_d_n22,) = {
    if (((locals.var_guard504 == 0.0) && (locals.var_guard513 != 0.0)) && (locals.var_guard518 != 0.0)) {
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_vtv;
        let assign30400_e47945: f64 = (locals.var_ef1 * __rspice_inv_cse_0);
        let assign30400_e47948: f64 = (locals.var_vgef23g0 * __rspice_inv_cse_0);
        let assign30400_e47949: f64 = (assign30400_e47945 - assign30400_e47948);
        (assign30400_e47949, ((locals.var_ef1_dn0 / locals.var_vtv) - (locals.var_vgef23g0_dn0 / locals.var_vtv)), ((locals.var_ef1_dn1 / locals.var_vtv) - (locals.var_vgef23g0_dn1 / locals.var_vtv)), ((locals.var_ef1_dn2 / locals.var_vtv) - (locals.var_vgef23g0_dn2 / locals.var_vtv)), ((locals.var_ef1_dn3 / locals.var_vtv) - (locals.var_vgef23g0_dn3 / locals.var_vtv)), ((((locals.var_ef1_dn4 * locals.var_vtv) - (locals.var_ef1 * locals.var_vtv_dn4)) / (locals.var_vtv * locals.var_vtv)) - (((locals.var_vgef23g0_dn4 * locals.var_vtv) - (locals.var_vgef23g0 * locals.var_vtv_dn4)) / (locals.var_vtv * locals.var_vtv))), ((locals.var_ef1_dn5 / locals.var_vtv) - (locals.var_vgef23g0_dn5 / locals.var_vtv)), ((((locals.var_ef1_dn6 * locals.var_vtv) - (locals.var_ef1 * locals.var_vtv_dn6)) / (locals.var_vtv * locals.var_vtv)) - (((locals.var_vgef23g0_dn6 * locals.var_vtv) - (locals.var_vgef23g0 * locals.var_vtv_dn6)) / (locals.var_vtv * locals.var_vtv))), ((((locals.var_ef1_dn7 * locals.var_vtv) - (locals.var_ef1 * locals.var_vtv_dn7)) / (locals.var_vtv * locals.var_vtv)) - (((locals.var_vgef23g0_dn7 * locals.var_vtv) - (locals.var_vgef23g0 * locals.var_vtv_dn7)) / (locals.var_vtv * locals.var_vtv))), ((((locals.var_ef1_dn8 * locals.var_vtv) - (locals.var_ef1 * locals.var_vtv_dn8)) / (locals.var_vtv * locals.var_vtv)) - (((locals.var_vgef23g0_dn8 * locals.var_vtv) - (locals.var_vgef23g0 * locals.var_vtv_dn8)) / (locals.var_vtv * locals.var_vtv))), ((locals.var_ef1_dn9 / locals.var_vtv) - (locals.var_vgef23g0_dn9 / locals.var_vtv)), ((locals.var_ef1_dn12 / locals.var_vtv) - (locals.var_vgef23g0_dn12 / locals.var_vtv)), ((locals.var_ef1_dn14 / locals.var_vtv) - (locals.var_vgef23g0_dn14 / locals.var_vtv)), ((((locals.var_ef1_dn15 * locals.var_vtv) - (locals.var_ef1 * locals.var_vtv_dn15)) / (locals.var_vtv * locals.var_vtv)) - (((locals.var_vgef23g0_dn15 * locals.var_vtv) - (locals.var_vgef23g0 * locals.var_vtv_dn15)) / (locals.var_vtv * locals.var_vtv))), ((((locals.var_ef1_dn16 * locals.var_vtv) - (locals.var_ef1 * locals.var_vtv_dn16)) / (locals.var_vtv * locals.var_vtv)) - (((locals.var_vgef23g0_dn16 * locals.var_vtv) - (locals.var_vgef23g0 * locals.var_vtv_dn16)) / (locals.var_vtv * locals.var_vtv))), ((((locals.var_ef1_dn17 * locals.var_vtv) - (locals.var_ef1 * locals.var_vtv_dn17)) / (locals.var_vtv * locals.var_vtv)) - (((locals.var_vgef23g0_dn17 * locals.var_vtv) - (locals.var_vgef23g0 * locals.var_vtv_dn17)) / (locals.var_vtv * locals.var_vtv))), ((((locals.var_ef1_dn18 * locals.var_vtv) - (locals.var_ef1 * locals.var_vtv_dn18)) / (locals.var_vtv * locals.var_vtv)) - (((locals.var_vgef23g0_dn18 * locals.var_vtv) - (locals.var_vgef23g0 * locals.var_vtv_dn18)) / (locals.var_vtv * locals.var_vtv))), ((((locals.var_ef1_dn19 * locals.var_vtv) - (locals.var_ef1 * locals.var_vtv_dn19)) / (locals.var_vtv * locals.var_vtv)) - (((locals.var_vgef23g0_dn19 * locals.var_vtv) - (locals.var_vgef23g0 * locals.var_vtv_dn19)) / (locals.var_vtv * locals.var_vtv))), ((((locals.var_ef1_dn20 * locals.var_vtv) - (locals.var_ef1 * locals.var_vtv_dn20)) / (locals.var_vtv * locals.var_vtv)) - (((locals.var_vgef23g0_dn20 * locals.var_vtv) - (locals.var_vgef23g0 * locals.var_vtv_dn20)) / (locals.var_vtv * locals.var_vtv))), ((((locals.var_ef1_dn21 * locals.var_vtv) - (locals.var_ef1 * locals.var_vtv_dn21)) / (locals.var_vtv * locals.var_vtv)) - (((locals.var_vgef23g0_dn21 * locals.var_vtv) - (locals.var_vgef23g0 * locals.var_vtv_dn21)) / (locals.var_vtv * locals.var_vtv))), ((((locals.var_ef1_dn22 * locals.var_vtv) - (locals.var_ef1 * locals.var_vtv_dn22)) / (locals.var_vtv * locals.var_vtv)) - (((locals.var_vgef23g0_dn22 * locals.var_vtv) - (locals.var_vgef23g0 * locals.var_vtv_dn22)) / (locals.var_vtv * locals.var_vtv))),)
    } else {
        (locals.var_tg0, locals.var_tg0_dn0, locals.var_tg0_dn1, locals.var_tg0_dn2, locals.var_tg0_dn3, locals.var_tg0_dn4, locals.var_tg0_dn5, locals.var_tg0_dn6, locals.var_tg0_dn7, locals.var_tg0_dn8, locals.var_tg0_dn9, locals.var_tg0_dn12, locals.var_tg0_dn14, locals.var_tg0_dn15, locals.var_tg0_dn16, locals.var_tg0_dn17, locals.var_tg0_dn18, locals.var_tg0_dn19, locals.var_tg0_dn20, locals.var_tg0_dn21, locals.var_tg0_dn22,)
    }
};
        locals.var_tg0 = assign30400_e47951;
        locals.var_tg0_dn0 = assign30400_e47951_d_n0;
        locals.var_tg0_dn1 = assign30400_e47951_d_n1;
        locals.var_tg0_dn2 = assign30400_e47951_d_n2;
        locals.var_tg0_dn3 = assign30400_e47951_d_n3;
        locals.var_tg0_dn4 = assign30400_e47951_d_n4;
        locals.var_tg0_dn5 = assign30400_e47951_d_n5;
        locals.var_tg0_dn6 = assign30400_e47951_d_n6;
        locals.var_tg0_dn7 = assign30400_e47951_d_n7;
        locals.var_tg0_dn8 = assign30400_e47951_d_n8;
        locals.var_tg0_dn9 = assign30400_e47951_d_n9;
        locals.var_tg0_dn12 = assign30400_e47951_d_n12;
        locals.var_tg0_dn14 = assign30400_e47951_d_n14;
        locals.var_tg0_dn15 = assign30400_e47951_d_n15;
        locals.var_tg0_dn16 = assign30400_e47951_d_n16;
        locals.var_tg0_dn17 = assign30400_e47951_d_n17;
        locals.var_tg0_dn18 = assign30400_e47951_d_n18;
        locals.var_tg0_dn19 = assign30400_e47951_d_n19;
        locals.var_tg0_dn20 = assign30400_e47951_d_n20;
        locals.var_tg0_dn21 = assign30400_e47951_d_n21;
        locals.var_tg0_dn22 = assign30400_e47951_d_n22;

        let (assign30410_e47966, assign30410_e47966_d_n0, assign30410_e47966_d_n1, assign30410_e47966_d_n2, assign30410_e47966_d_n3, assign30410_e47966_d_n4, assign30410_e47966_d_n5, assign30410_e47966_d_n6, assign30410_e47966_d_n7, assign30410_e47966_d_n8, assign30410_e47966_d_n9, assign30410_e47966_d_n12, assign30410_e47966_d_n14, assign30410_e47966_d_n15, assign30410_e47966_d_n16, assign30410_e47966_d_n17, assign30410_e47966_d_n18, assign30410_e47966_d_n19, assign30410_e47966_d_n20, assign30410_e47966_d_n21, assign30410_e47966_d_n22,) = {
    if (((locals.var_guard504 == 0.0) && (locals.var_guard513 != 0.0)) && (locals.var_guard518 != 0.0)) {
        let __rspice_inv_cse_1: f64 = 1.0 / locals.var_vtv;
        let assign30410_e47960: f64 = (locals.var_ef1 * __rspice_inv_cse_1);
        let assign30410_e47963: f64 = (locals.var_vgef23g1 * __rspice_inv_cse_1);
        let assign30410_e47964: f64 = (assign30410_e47960 - assign30410_e47963);
        (assign30410_e47964, ((locals.var_ef1_dn0 / locals.var_vtv) - (locals.var_vgef23g1_dn0 / locals.var_vtv)), ((locals.var_ef1_dn1 / locals.var_vtv) - (locals.var_vgef23g1_dn1 / locals.var_vtv)), ((locals.var_ef1_dn2 / locals.var_vtv) - (locals.var_vgef23g1_dn2 / locals.var_vtv)), ((locals.var_ef1_dn3 / locals.var_vtv) - (locals.var_vgef23g1_dn3 / locals.var_vtv)), ((((locals.var_ef1_dn4 * locals.var_vtv) - (locals.var_ef1 * locals.var_vtv_dn4)) / (locals.var_vtv * locals.var_vtv)) - (((locals.var_vgef23g1_dn4 * locals.var_vtv) - (locals.var_vgef23g1 * locals.var_vtv_dn4)) / (locals.var_vtv * locals.var_vtv))), ((locals.var_ef1_dn5 / locals.var_vtv) - (locals.var_vgef23g1_dn5 / locals.var_vtv)), ((((locals.var_ef1_dn6 * locals.var_vtv) - (locals.var_ef1 * locals.var_vtv_dn6)) / (locals.var_vtv * locals.var_vtv)) - (((locals.var_vgef23g1_dn6 * locals.var_vtv) - (locals.var_vgef23g1 * locals.var_vtv_dn6)) / (locals.var_vtv * locals.var_vtv))), ((((locals.var_ef1_dn7 * locals.var_vtv) - (locals.var_ef1 * locals.var_vtv_dn7)) / (locals.var_vtv * locals.var_vtv)) - (((locals.var_vgef23g1_dn7 * locals.var_vtv) - (locals.var_vgef23g1 * locals.var_vtv_dn7)) / (locals.var_vtv * locals.var_vtv))), ((((locals.var_ef1_dn8 * locals.var_vtv) - (locals.var_ef1 * locals.var_vtv_dn8)) / (locals.var_vtv * locals.var_vtv)) - (((locals.var_vgef23g1_dn8 * locals.var_vtv) - (locals.var_vgef23g1 * locals.var_vtv_dn8)) / (locals.var_vtv * locals.var_vtv))), ((locals.var_ef1_dn9 / locals.var_vtv) - (locals.var_vgef23g1_dn9 / locals.var_vtv)), ((locals.var_ef1_dn12 / locals.var_vtv) - (locals.var_vgef23g1_dn12 / locals.var_vtv)), ((locals.var_ef1_dn14 / locals.var_vtv) - (locals.var_vgef23g1_dn14 / locals.var_vtv)), ((((locals.var_ef1_dn15 * locals.var_vtv) - (locals.var_ef1 * locals.var_vtv_dn15)) / (locals.var_vtv * locals.var_vtv)) - (((locals.var_vgef23g1_dn15 * locals.var_vtv) - (locals.var_vgef23g1 * locals.var_vtv_dn15)) / (locals.var_vtv * locals.var_vtv))), ((((locals.var_ef1_dn16 * locals.var_vtv) - (locals.var_ef1 * locals.var_vtv_dn16)) / (locals.var_vtv * locals.var_vtv)) - (((locals.var_vgef23g1_dn16 * locals.var_vtv) - (locals.var_vgef23g1 * locals.var_vtv_dn16)) / (locals.var_vtv * locals.var_vtv))), ((((locals.var_ef1_dn17 * locals.var_vtv) - (locals.var_ef1 * locals.var_vtv_dn17)) / (locals.var_vtv * locals.var_vtv)) - (((locals.var_vgef23g1_dn17 * locals.var_vtv) - (locals.var_vgef23g1 * locals.var_vtv_dn17)) / (locals.var_vtv * locals.var_vtv))), ((((locals.var_ef1_dn18 * locals.var_vtv) - (locals.var_ef1 * locals.var_vtv_dn18)) / (locals.var_vtv * locals.var_vtv)) - (((locals.var_vgef23g1_dn18 * locals.var_vtv) - (locals.var_vgef23g1 * locals.var_vtv_dn18)) / (locals.var_vtv * locals.var_vtv))), ((((locals.var_ef1_dn19 * locals.var_vtv) - (locals.var_ef1 * locals.var_vtv_dn19)) / (locals.var_vtv * locals.var_vtv)) - (((locals.var_vgef23g1_dn19 * locals.var_vtv) - (locals.var_vgef23g1 * locals.var_vtv_dn19)) / (locals.var_vtv * locals.var_vtv))), ((((locals.var_ef1_dn20 * locals.var_vtv) - (locals.var_ef1 * locals.var_vtv_dn20)) / (locals.var_vtv * locals.var_vtv)) - (((locals.var_vgef23g1_dn20 * locals.var_vtv) - (locals.var_vgef23g1 * locals.var_vtv_dn20)) / (locals.var_vtv * locals.var_vtv))), ((((locals.var_ef1_dn21 * locals.var_vtv) - (locals.var_ef1 * locals.var_vtv_dn21)) / (locals.var_vtv * locals.var_vtv)) - (((locals.var_vgef23g1_dn21 * locals.var_vtv) - (locals.var_vgef23g1 * locals.var_vtv_dn21)) / (locals.var_vtv * locals.var_vtv))), ((((locals.var_ef1_dn22 * locals.var_vtv) - (locals.var_ef1 * locals.var_vtv_dn22)) / (locals.var_vtv * locals.var_vtv)) - (((locals.var_vgef23g1_dn22 * locals.var_vtv) - (locals.var_vgef23g1 * locals.var_vtv_dn22)) / (locals.var_vtv * locals.var_vtv))),)
    } else {
        (locals.var_tg1, locals.var_tg1_dn0, locals.var_tg1_dn1, locals.var_tg1_dn2, locals.var_tg1_dn3, locals.var_tg1_dn4, locals.var_tg1_dn5, locals.var_tg1_dn6, locals.var_tg1_dn7, locals.var_tg1_dn8, locals.var_tg1_dn9, locals.var_tg1_dn12, locals.var_tg1_dn14, locals.var_tg1_dn15, locals.var_tg1_dn16, locals.var_tg1_dn17, locals.var_tg1_dn18, locals.var_tg1_dn19, locals.var_tg1_dn20, locals.var_tg1_dn21, locals.var_tg1_dn22,)
    }
};
        locals.var_tg1 = assign30410_e47966;
        locals.var_tg1_dn0 = assign30410_e47966_d_n0;
        locals.var_tg1_dn1 = assign30410_e47966_d_n1;
        locals.var_tg1_dn2 = assign30410_e47966_d_n2;
        locals.var_tg1_dn3 = assign30410_e47966_d_n3;
        locals.var_tg1_dn4 = assign30410_e47966_d_n4;
        locals.var_tg1_dn5 = assign30410_e47966_d_n5;
        locals.var_tg1_dn6 = assign30410_e47966_d_n6;
        locals.var_tg1_dn7 = assign30410_e47966_d_n7;
        locals.var_tg1_dn8 = assign30410_e47966_d_n8;
        locals.var_tg1_dn9 = assign30410_e47966_d_n9;
        locals.var_tg1_dn12 = assign30410_e47966_d_n12;
        locals.var_tg1_dn14 = assign30410_e47966_d_n14;
        locals.var_tg1_dn15 = assign30410_e47966_d_n15;
        locals.var_tg1_dn16 = assign30410_e47966_d_n16;
        locals.var_tg1_dn17 = assign30410_e47966_d_n17;
        locals.var_tg1_dn18 = assign30410_e47966_d_n18;
        locals.var_tg1_dn19 = assign30410_e47966_d_n19;
        locals.var_tg1_dn20 = assign30410_e47966_d_n20;
        locals.var_tg1_dn21 = assign30410_e47966_d_n21;
        locals.var_tg1_dn22 = assign30410_e47966_d_n22;

        let (assign30420_e48053, assign30420_e48053_d_n0, assign30420_e48053_d_n1, assign30420_e48053_d_n2, assign30420_e48053_d_n3, assign30420_e48053_d_n4, assign30420_e48053_d_n5, assign30420_e48053_d_n6, assign30420_e48053_d_n7, assign30420_e48053_d_n8, assign30420_e48053_d_n9, assign30420_e48053_d_n12, assign30420_e48053_d_n14, assign30420_e48053_d_n15, assign30420_e48053_d_n16, assign30420_e48053_d_n17, assign30420_e48053_d_n18, assign30420_e48053_d_n19, assign30420_e48053_d_n20, assign30420_e48053_d_n21, assign30420_e48053_d_n22,) = {
    if (((locals.var_guard504 == 0.0) && (locals.var_guard513 != 0.0)) && (locals.var_guard518 != 0.0)) {
        let assign30420_e47975: f64 = (locals.var_cch * locals.var_vgef1);
        let assign30420_e47978: f64 = (3.24e17 * locals.var_vtv);
        let assign30420_e47985: f64 = (-37.0);
        let (assign30420_e48011, assign30420_e48011_d_n0, assign30420_e48011_d_n1, assign30420_e48011_d_n2, assign30420_e48011_d_n3, assign30420_e48011_d_n4, assign30420_e48011_d_n5, assign30420_e48011_d_n6, assign30420_e48011_d_n7, assign30420_e48011_d_n8, assign30420_e48011_d_n9, assign30420_e48011_d_n12, assign30420_e48011_d_n14, assign30420_e48011_d_n15, assign30420_e48011_d_n16, assign30420_e48011_d_n17, assign30420_e48011_d_n18, assign30420_e48011_d_n19, assign30420_e48011_d_n20, assign30420_e48011_d_n21, assign30420_e48011_d_n22,) = {
            if ((!(locals.var_tg0 >= 37.0)) && (!(locals.var_tg0 <= assign30420_e47985))) {
                let assign30420_e47990: f64 = (locals.var_tg0).exp();
                let assign30420_e47992: f64 = (assign30420_e47990 + 1.0);
                let assign30420_e47993: f64 = (assign30420_e47992).ln();
                (assign30420_e47993, ((assign30420_e47990 * locals.var_tg0_dn0) / assign30420_e47992), ((assign30420_e47990 * locals.var_tg0_dn1) / assign30420_e47992), ((assign30420_e47990 * locals.var_tg0_dn2) / assign30420_e47992), ((assign30420_e47990 * locals.var_tg0_dn3) / assign30420_e47992), ((assign30420_e47990 * locals.var_tg0_dn4) / assign30420_e47992), ((assign30420_e47990 * locals.var_tg0_dn5) / assign30420_e47992), ((assign30420_e47990 * locals.var_tg0_dn6) / assign30420_e47992), ((assign30420_e47990 * locals.var_tg0_dn7) / assign30420_e47992), ((assign30420_e47990 * locals.var_tg0_dn8) / assign30420_e47992), ((assign30420_e47990 * locals.var_tg0_dn9) / assign30420_e47992), ((assign30420_e47990 * locals.var_tg0_dn12) / assign30420_e47992), ((assign30420_e47990 * locals.var_tg0_dn14) / assign30420_e47992), ((assign30420_e47990 * locals.var_tg0_dn15) / assign30420_e47992), ((assign30420_e47990 * locals.var_tg0_dn16) / assign30420_e47992), ((assign30420_e47990 * locals.var_tg0_dn17) / assign30420_e47992), ((assign30420_e47990 * locals.var_tg0_dn18) / assign30420_e47992), ((assign30420_e47990 * locals.var_tg0_dn19) / assign30420_e47992), ((assign30420_e47990 * locals.var_tg0_dn20) / assign30420_e47992), ((assign30420_e47990 * locals.var_tg0_dn21) / assign30420_e47992), ((assign30420_e47990 * locals.var_tg0_dn22) / assign30420_e47992),)
            } else {
                let assign30420_e48000: f64 = (-37.0);
                let (assign30420_e48010, assign30420_e48010_d_n0, assign30420_e48010_d_n1, assign30420_e48010_d_n2, assign30420_e48010_d_n3, assign30420_e48010_d_n4, assign30420_e48010_d_n5, assign30420_e48010_d_n6, assign30420_e48010_d_n7, assign30420_e48010_d_n8, assign30420_e48010_d_n9, assign30420_e48010_d_n12, assign30420_e48010_d_n14, assign30420_e48010_d_n15, assign30420_e48010_d_n16, assign30420_e48010_d_n17, assign30420_e48010_d_n18, assign30420_e48010_d_n19, assign30420_e48010_d_n20, assign30420_e48010_d_n21, assign30420_e48010_d_n22,) = {
                    if ((!(locals.var_tg0 >= 37.0)) && (locals.var_tg0 <= assign30420_e48000)) {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    } else {
                        let (assign30420_e48009, assign30420_e48009_d_n0, assign30420_e48009_d_n1, assign30420_e48009_d_n2, assign30420_e48009_d_n3, assign30420_e48009_d_n4, assign30420_e48009_d_n5, assign30420_e48009_d_n6, assign30420_e48009_d_n7, assign30420_e48009_d_n8, assign30420_e48009_d_n9, assign30420_e48009_d_n12, assign30420_e48009_d_n14, assign30420_e48009_d_n15, assign30420_e48009_d_n16, assign30420_e48009_d_n17, assign30420_e48009_d_n18, assign30420_e48009_d_n19, assign30420_e48009_d_n20, assign30420_e48009_d_n21, assign30420_e48009_d_n22,) = {
                            if (locals.var_tg0 >= 37.0) {
                                (locals.var_tg0, locals.var_tg0_dn0, locals.var_tg0_dn1, locals.var_tg0_dn2, locals.var_tg0_dn3, locals.var_tg0_dn4, locals.var_tg0_dn5, locals.var_tg0_dn6, locals.var_tg0_dn7, locals.var_tg0_dn8, locals.var_tg0_dn9, locals.var_tg0_dn12, locals.var_tg0_dn14, locals.var_tg0_dn15, locals.var_tg0_dn16, locals.var_tg0_dn17, locals.var_tg0_dn18, locals.var_tg0_dn19, locals.var_tg0_dn20, locals.var_tg0_dn21, locals.var_tg0_dn22,)
                            } else {
                                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign30420_e48009, assign30420_e48009_d_n0, assign30420_e48009_d_n1, assign30420_e48009_d_n2, assign30420_e48009_d_n3, assign30420_e48009_d_n4, assign30420_e48009_d_n5, assign30420_e48009_d_n6, assign30420_e48009_d_n7, assign30420_e48009_d_n8, assign30420_e48009_d_n9, assign30420_e48009_d_n12, assign30420_e48009_d_n14, assign30420_e48009_d_n15, assign30420_e48009_d_n16, assign30420_e48009_d_n17, assign30420_e48009_d_n18, assign30420_e48009_d_n19, assign30420_e48009_d_n20, assign30420_e48009_d_n21, assign30420_e48009_d_n22,)
                    }
                };
                (assign30420_e48010, assign30420_e48010_d_n0, assign30420_e48010_d_n1, assign30420_e48010_d_n2, assign30420_e48010_d_n3, assign30420_e48010_d_n4, assign30420_e48010_d_n5, assign30420_e48010_d_n6, assign30420_e48010_d_n7, assign30420_e48010_d_n8, assign30420_e48010_d_n9, assign30420_e48010_d_n12, assign30420_e48010_d_n14, assign30420_e48010_d_n15, assign30420_e48010_d_n16, assign30420_e48010_d_n17, assign30420_e48010_d_n18, assign30420_e48010_d_n19, assign30420_e48010_d_n20, assign30420_e48010_d_n21, assign30420_e48010_d_n22,)
            }
        };
        let assign30420_e48012: f64 = (assign30420_e47978 * assign30420_e48011);
        let assign30420_e48013: f64 = (assign30420_e47975 - assign30420_e48012);
        let assign30420_e48016: f64 = (3.24e17 * locals.var_vtv);
        let assign30420_e48023: f64 = (-37.0);
        let (assign30420_e48049, assign30420_e48049_d_n0, assign30420_e48049_d_n1, assign30420_e48049_d_n2, assign30420_e48049_d_n3, assign30420_e48049_d_n4, assign30420_e48049_d_n5, assign30420_e48049_d_n6, assign30420_e48049_d_n7, assign30420_e48049_d_n8, assign30420_e48049_d_n9, assign30420_e48049_d_n12, assign30420_e48049_d_n14, assign30420_e48049_d_n15, assign30420_e48049_d_n16, assign30420_e48049_d_n17, assign30420_e48049_d_n18, assign30420_e48049_d_n19, assign30420_e48049_d_n20, assign30420_e48049_d_n21, assign30420_e48049_d_n22,) = {
            if ((!(locals.var_tg1 >= 37.0)) && (!(locals.var_tg1 <= assign30420_e48023))) {
                let assign30420_e48028: f64 = (locals.var_tg1).exp();
                let assign30420_e48030: f64 = (assign30420_e48028 + 1.0);
                let assign30420_e48031: f64 = (assign30420_e48030).ln();
                (assign30420_e48031, ((assign30420_e48028 * locals.var_tg1_dn0) / assign30420_e48030), ((assign30420_e48028 * locals.var_tg1_dn1) / assign30420_e48030), ((assign30420_e48028 * locals.var_tg1_dn2) / assign30420_e48030), ((assign30420_e48028 * locals.var_tg1_dn3) / assign30420_e48030), ((assign30420_e48028 * locals.var_tg1_dn4) / assign30420_e48030), ((assign30420_e48028 * locals.var_tg1_dn5) / assign30420_e48030), ((assign30420_e48028 * locals.var_tg1_dn6) / assign30420_e48030), ((assign30420_e48028 * locals.var_tg1_dn7) / assign30420_e48030), ((assign30420_e48028 * locals.var_tg1_dn8) / assign30420_e48030), ((assign30420_e48028 * locals.var_tg1_dn9) / assign30420_e48030), ((assign30420_e48028 * locals.var_tg1_dn12) / assign30420_e48030), ((assign30420_e48028 * locals.var_tg1_dn14) / assign30420_e48030), ((assign30420_e48028 * locals.var_tg1_dn15) / assign30420_e48030), ((assign30420_e48028 * locals.var_tg1_dn16) / assign30420_e48030), ((assign30420_e48028 * locals.var_tg1_dn17) / assign30420_e48030), ((assign30420_e48028 * locals.var_tg1_dn18) / assign30420_e48030), ((assign30420_e48028 * locals.var_tg1_dn19) / assign30420_e48030), ((assign30420_e48028 * locals.var_tg1_dn20) / assign30420_e48030), ((assign30420_e48028 * locals.var_tg1_dn21) / assign30420_e48030), ((assign30420_e48028 * locals.var_tg1_dn22) / assign30420_e48030),)
            } else {
                let assign30420_e48038: f64 = (-37.0);
                let (assign30420_e48048, assign30420_e48048_d_n0, assign30420_e48048_d_n1, assign30420_e48048_d_n2, assign30420_e48048_d_n3, assign30420_e48048_d_n4, assign30420_e48048_d_n5, assign30420_e48048_d_n6, assign30420_e48048_d_n7, assign30420_e48048_d_n8, assign30420_e48048_d_n9, assign30420_e48048_d_n12, assign30420_e48048_d_n14, assign30420_e48048_d_n15, assign30420_e48048_d_n16, assign30420_e48048_d_n17, assign30420_e48048_d_n18, assign30420_e48048_d_n19, assign30420_e48048_d_n20, assign30420_e48048_d_n21, assign30420_e48048_d_n22,) = {
                    if ((!(locals.var_tg1 >= 37.0)) && (locals.var_tg1 <= assign30420_e48038)) {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    } else {
                        let (assign30420_e48047, assign30420_e48047_d_n0, assign30420_e48047_d_n1, assign30420_e48047_d_n2, assign30420_e48047_d_n3, assign30420_e48047_d_n4, assign30420_e48047_d_n5, assign30420_e48047_d_n6, assign30420_e48047_d_n7, assign30420_e48047_d_n8, assign30420_e48047_d_n9, assign30420_e48047_d_n12, assign30420_e48047_d_n14, assign30420_e48047_d_n15, assign30420_e48047_d_n16, assign30420_e48047_d_n17, assign30420_e48047_d_n18, assign30420_e48047_d_n19, assign30420_e48047_d_n20, assign30420_e48047_d_n21, assign30420_e48047_d_n22,) = {
                            if (locals.var_tg1 >= 37.0) {
                                (locals.var_tg1, locals.var_tg1_dn0, locals.var_tg1_dn1, locals.var_tg1_dn2, locals.var_tg1_dn3, locals.var_tg1_dn4, locals.var_tg1_dn5, locals.var_tg1_dn6, locals.var_tg1_dn7, locals.var_tg1_dn8, locals.var_tg1_dn9, locals.var_tg1_dn12, locals.var_tg1_dn14, locals.var_tg1_dn15, locals.var_tg1_dn16, locals.var_tg1_dn17, locals.var_tg1_dn18, locals.var_tg1_dn19, locals.var_tg1_dn20, locals.var_tg1_dn21, locals.var_tg1_dn22,)
                            } else {
                                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign30420_e48047, assign30420_e48047_d_n0, assign30420_e48047_d_n1, assign30420_e48047_d_n2, assign30420_e48047_d_n3, assign30420_e48047_d_n4, assign30420_e48047_d_n5, assign30420_e48047_d_n6, assign30420_e48047_d_n7, assign30420_e48047_d_n8, assign30420_e48047_d_n9, assign30420_e48047_d_n12, assign30420_e48047_d_n14, assign30420_e48047_d_n15, assign30420_e48047_d_n16, assign30420_e48047_d_n17, assign30420_e48047_d_n18, assign30420_e48047_d_n19, assign30420_e48047_d_n20, assign30420_e48047_d_n21, assign30420_e48047_d_n22,)
                    }
                };
                (assign30420_e48048, assign30420_e48048_d_n0, assign30420_e48048_d_n1, assign30420_e48048_d_n2, assign30420_e48048_d_n3, assign30420_e48048_d_n4, assign30420_e48048_d_n5, assign30420_e48048_d_n6, assign30420_e48048_d_n7, assign30420_e48048_d_n8, assign30420_e48048_d_n9, assign30420_e48048_d_n12, assign30420_e48048_d_n14, assign30420_e48048_d_n15, assign30420_e48048_d_n16, assign30420_e48048_d_n17, assign30420_e48048_d_n18, assign30420_e48048_d_n19, assign30420_e48048_d_n20, assign30420_e48048_d_n21, assign30420_e48048_d_n22,)
            }
        };
        let assign30420_e48050: f64 = (assign30420_e48016 * assign30420_e48049);
        let assign30420_e48051: f64 = (assign30420_e48013 - assign30420_e48050);
        (assign30420_e48051, (((locals.var_cch * locals.var_vgef1_dn0) - (assign30420_e47978 * assign30420_e48011_d_n0)) - (assign30420_e48016 * assign30420_e48049_d_n0)), (((locals.var_cch * locals.var_vgef1_dn1) - (assign30420_e47978 * assign30420_e48011_d_n1)) - (assign30420_e48016 * assign30420_e48049_d_n1)), (((locals.var_cch * locals.var_vgef1_dn2) - (assign30420_e47978 * assign30420_e48011_d_n2)) - (assign30420_e48016 * assign30420_e48049_d_n2)), (((locals.var_cch * locals.var_vgef1_dn3) - (assign30420_e47978 * assign30420_e48011_d_n3)) - (assign30420_e48016 * assign30420_e48049_d_n3)), (((locals.var_cch * locals.var_vgef1_dn4) - (((3.24e17 * locals.var_vtv_dn4) * assign30420_e48011) + (assign30420_e47978 * assign30420_e48011_d_n4))) - (((3.24e17 * locals.var_vtv_dn4) * assign30420_e48049) + (assign30420_e48016 * assign30420_e48049_d_n4))), (((locals.var_cch * locals.var_vgef1_dn5) - (assign30420_e47978 * assign30420_e48011_d_n5)) - (assign30420_e48016 * assign30420_e48049_d_n5)), (((locals.var_cch * locals.var_vgef1_dn6) - (((3.24e17 * locals.var_vtv_dn6) * assign30420_e48011) + (assign30420_e47978 * assign30420_e48011_d_n6))) - (((3.24e17 * locals.var_vtv_dn6) * assign30420_e48049) + (assign30420_e48016 * assign30420_e48049_d_n6))), (((locals.var_cch * locals.var_vgef1_dn7) - (((3.24e17 * locals.var_vtv_dn7) * assign30420_e48011) + (assign30420_e47978 * assign30420_e48011_d_n7))) - (((3.24e17 * locals.var_vtv_dn7) * assign30420_e48049) + (assign30420_e48016 * assign30420_e48049_d_n7))), (((locals.var_cch * locals.var_vgef1_dn8) - (((3.24e17 * locals.var_vtv_dn8) * assign30420_e48011) + (assign30420_e47978 * assign30420_e48011_d_n8))) - (((3.24e17 * locals.var_vtv_dn8) * assign30420_e48049) + (assign30420_e48016 * assign30420_e48049_d_n8))), (((locals.var_cch * locals.var_vgef1_dn9) - (assign30420_e47978 * assign30420_e48011_d_n9)) - (assign30420_e48016 * assign30420_e48049_d_n9)), (((locals.var_cch * locals.var_vgef1_dn12) - (assign30420_e47978 * assign30420_e48011_d_n12)) - (assign30420_e48016 * assign30420_e48049_d_n12)), (((locals.var_cch * locals.var_vgef1_dn14) - (assign30420_e47978 * assign30420_e48011_d_n14)) - (assign30420_e48016 * assign30420_e48049_d_n14)), (((locals.var_cch * locals.var_vgef1_dn15) - (((3.24e17 * locals.var_vtv_dn15) * assign30420_e48011) + (assign30420_e47978 * assign30420_e48011_d_n15))) - (((3.24e17 * locals.var_vtv_dn15) * assign30420_e48049) + (assign30420_e48016 * assign30420_e48049_d_n15))), (((locals.var_cch * locals.var_vgef1_dn16) - (((3.24e17 * locals.var_vtv_dn16) * assign30420_e48011) + (assign30420_e47978 * assign30420_e48011_d_n16))) - (((3.24e17 * locals.var_vtv_dn16) * assign30420_e48049) + (assign30420_e48016 * assign30420_e48049_d_n16))), (((locals.var_cch * locals.var_vgef1_dn17) - (((3.24e17 * locals.var_vtv_dn17) * assign30420_e48011) + (assign30420_e47978 * assign30420_e48011_d_n17))) - (((3.24e17 * locals.var_vtv_dn17) * assign30420_e48049) + (assign30420_e48016 * assign30420_e48049_d_n17))), (((locals.var_cch * locals.var_vgef1_dn18) - (((3.24e17 * locals.var_vtv_dn18) * assign30420_e48011) + (assign30420_e47978 * assign30420_e48011_d_n18))) - (((3.24e17 * locals.var_vtv_dn18) * assign30420_e48049) + (assign30420_e48016 * assign30420_e48049_d_n18))), (((locals.var_cch * locals.var_vgef1_dn19) - (((3.24e17 * locals.var_vtv_dn19) * assign30420_e48011) + (assign30420_e47978 * assign30420_e48011_d_n19))) - (((3.24e17 * locals.var_vtv_dn19) * assign30420_e48049) + (assign30420_e48016 * assign30420_e48049_d_n19))), (((locals.var_cch * locals.var_vgef1_dn20) - (((3.24e17 * locals.var_vtv_dn20) * assign30420_e48011) + (assign30420_e47978 * assign30420_e48011_d_n20))) - (((3.24e17 * locals.var_vtv_dn20) * assign30420_e48049) + (assign30420_e48016 * assign30420_e48049_d_n20))), (((locals.var_cch * locals.var_vgef1_dn21) - (((3.24e17 * locals.var_vtv_dn21) * assign30420_e48011) + (assign30420_e47978 * assign30420_e48011_d_n21))) - (((3.24e17 * locals.var_vtv_dn21) * assign30420_e48049) + (assign30420_e48016 * assign30420_e48049_d_n21))), (((locals.var_cch * locals.var_vgef1_dn22) - (((3.24e17 * locals.var_vtv_dn22) * assign30420_e48011) + (assign30420_e47978 * assign30420_e48011_d_n22))) - (((3.24e17 * locals.var_vtv_dn22) * assign30420_e48049) + (assign30420_e48016 * assign30420_e48049_d_n22))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn1, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn12, locals.var_t4_dn14, locals.var_t4_dn15, locals.var_t4_dn16, locals.var_t4_dn17, locals.var_t4_dn18, locals.var_t4_dn19, locals.var_t4_dn20, locals.var_t4_dn21, locals.var_t4_dn22,)
    }
};
        locals.var_t4 = assign30420_e48053;
        locals.var_t4_dn0 = assign30420_e48053_d_n0;
        locals.var_t4_dn1 = assign30420_e48053_d_n1;
        locals.var_t4_dn2 = assign30420_e48053_d_n2;
        locals.var_t4_dn3 = assign30420_e48053_d_n3;
        locals.var_t4_dn4 = assign30420_e48053_d_n4;
        locals.var_t4_dn5 = assign30420_e48053_d_n5;
        locals.var_t4_dn6 = assign30420_e48053_d_n6;
        locals.var_t4_dn7 = assign30420_e48053_d_n7;
        locals.var_t4_dn8 = assign30420_e48053_d_n8;
        locals.var_t4_dn9 = assign30420_e48053_d_n9;
        locals.var_t4_dn12 = assign30420_e48053_d_n12;
        locals.var_t4_dn14 = assign30420_e48053_d_n14;
        locals.var_t4_dn15 = assign30420_e48053_d_n15;
        locals.var_t4_dn16 = assign30420_e48053_d_n16;
        locals.var_t4_dn17 = assign30420_e48053_d_n17;
        locals.var_t4_dn18 = assign30420_e48053_d_n18;
        locals.var_t4_dn19 = assign30420_e48053_d_n19;
        locals.var_t4_dn20 = assign30420_e48053_d_n20;
        locals.var_t4_dn21 = assign30420_e48053_d_n21;
        locals.var_t4_dn22 = assign30420_e48053_d_n22;

        let (assign30430_e48066, assign30430_e48066_d_n0, assign30430_e48066_d_n1, assign30430_e48066_d_n2, assign30430_e48066_d_n3, assign30430_e48066_d_n4, assign30430_e48066_d_n5, assign30430_e48066_d_n6, assign30430_e48066_d_n7, assign30430_e48066_d_n8, assign30430_e48066_d_n9, assign30430_e48066_d_n12, assign30430_e48066_d_n14, assign30430_e48066_d_n15, assign30430_e48066_d_n16, assign30430_e48066_d_n17, assign30430_e48066_d_n18, assign30430_e48066_d_n19, assign30430_e48066_d_n20, assign30430_e48066_d_n21, assign30430_e48066_d_n22,) = {
    if (((locals.var_guard504 == 0.0) && (locals.var_guard513 != 0.0)) && (locals.var_guard518 != 0.0)) {
        let assign30430_e48062: f64 = (p.p208 * locals.var_t0);
        let assign30430_e48064: f64 = (assign30430_e48062 * locals.var_t2);
        (assign30430_e48064, (((p.p208 * locals.var_t0_dn0) * locals.var_t2) + (assign30430_e48062 * locals.var_t2_dn0)), (((p.p208 * locals.var_t0_dn1) * locals.var_t2) + (assign30430_e48062 * locals.var_t2_dn1)), (((p.p208 * locals.var_t0_dn2) * locals.var_t2) + (assign30430_e48062 * locals.var_t2_dn2)), (((p.p208 * locals.var_t0_dn3) * locals.var_t2) + (assign30430_e48062 * locals.var_t2_dn3)), (((p.p208 * locals.var_t0_dn4) * locals.var_t2) + (assign30430_e48062 * locals.var_t2_dn4)), (((p.p208 * locals.var_t0_dn5) * locals.var_t2) + (assign30430_e48062 * locals.var_t2_dn5)), (((p.p208 * locals.var_t0_dn6) * locals.var_t2) + (assign30430_e48062 * locals.var_t2_dn6)), (((p.p208 * locals.var_t0_dn7) * locals.var_t2) + (assign30430_e48062 * locals.var_t2_dn7)), (((p.p208 * locals.var_t0_dn8) * locals.var_t2) + (assign30430_e48062 * locals.var_t2_dn8)), (((p.p208 * locals.var_t0_dn9) * locals.var_t2) + (assign30430_e48062 * locals.var_t2_dn9)), (((p.p208 * locals.var_t0_dn12) * locals.var_t2) + (assign30430_e48062 * locals.var_t2_dn12)), (((p.p208 * locals.var_t0_dn14) * locals.var_t2) + (assign30430_e48062 * locals.var_t2_dn14)), (((p.p208 * locals.var_t0_dn15) * locals.var_t2) + (assign30430_e48062 * locals.var_t2_dn15)), (((p.p208 * locals.var_t0_dn16) * locals.var_t2) + (assign30430_e48062 * locals.var_t2_dn16)), (((p.p208 * locals.var_t0_dn17) * locals.var_t2) + (assign30430_e48062 * locals.var_t2_dn17)), (((p.p208 * locals.var_t0_dn18) * locals.var_t2) + (assign30430_e48062 * locals.var_t2_dn18)), (((p.p208 * locals.var_t0_dn19) * locals.var_t2) + (assign30430_e48062 * locals.var_t2_dn19)), (((p.p208 * locals.var_t0_dn20) * locals.var_t2) + (assign30430_e48062 * locals.var_t2_dn20)), (((p.p208 * locals.var_t0_dn21) * locals.var_t2) + (assign30430_e48062 * locals.var_t2_dn21)), (((p.p208 * locals.var_t0_dn22) * locals.var_t2) + (assign30430_e48062 * locals.var_t2_dn22)),)
    } else {
        (locals.var_vgefm13g0, locals.var_vgefm13g0_dn0, locals.var_vgefm13g0_dn1, locals.var_vgefm13g0_dn2, locals.var_vgefm13g0_dn3, locals.var_vgefm13g0_dn4, locals.var_vgefm13g0_dn5, locals.var_vgefm13g0_dn6, locals.var_vgefm13g0_dn7, locals.var_vgefm13g0_dn8, locals.var_vgefm13g0_dn9, locals.var_vgefm13g0_dn12, locals.var_vgefm13g0_dn14, locals.var_vgefm13g0_dn15, locals.var_vgefm13g0_dn16, locals.var_vgefm13g0_dn17, locals.var_vgefm13g0_dn18, locals.var_vgefm13g0_dn19, locals.var_vgefm13g0_dn20, locals.var_vgefm13g0_dn21, locals.var_vgefm13g0_dn22,)
    }
};
        locals.var_vgefm13g0 = assign30430_e48066;
        locals.var_vgefm13g0_dn0 = assign30430_e48066_d_n0;
        locals.var_vgefm13g0_dn1 = assign30430_e48066_d_n1;
        locals.var_vgefm13g0_dn2 = assign30430_e48066_d_n2;
        locals.var_vgefm13g0_dn3 = assign30430_e48066_d_n3;
        locals.var_vgefm13g0_dn4 = assign30430_e48066_d_n4;
        locals.var_vgefm13g0_dn5 = assign30430_e48066_d_n5;
        locals.var_vgefm13g0_dn6 = assign30430_e48066_d_n6;
        locals.var_vgefm13g0_dn7 = assign30430_e48066_d_n7;
        locals.var_vgefm13g0_dn8 = assign30430_e48066_d_n8;
        locals.var_vgefm13g0_dn9 = assign30430_e48066_d_n9;
        locals.var_vgefm13g0_dn12 = assign30430_e48066_d_n12;
        locals.var_vgefm13g0_dn14 = assign30430_e48066_d_n14;
        locals.var_vgefm13g0_dn15 = assign30430_e48066_d_n15;
        locals.var_vgefm13g0_dn16 = assign30430_e48066_d_n16;
        locals.var_vgefm13g0_dn17 = assign30430_e48066_d_n17;
        locals.var_vgefm13g0_dn18 = assign30430_e48066_d_n18;
        locals.var_vgefm13g0_dn19 = assign30430_e48066_d_n19;
        locals.var_vgefm13g0_dn20 = assign30430_e48066_d_n20;
        locals.var_vgefm13g0_dn21 = assign30430_e48066_d_n21;
        locals.var_vgefm13g0_dn22 = assign30430_e48066_d_n22;

        let (assign30440_e48079, assign30440_e48079_d_n0, assign30440_e48079_d_n1, assign30440_e48079_d_n2, assign30440_e48079_d_n3, assign30440_e48079_d_n4, assign30440_e48079_d_n5, assign30440_e48079_d_n6, assign30440_e48079_d_n7, assign30440_e48079_d_n8, assign30440_e48079_d_n9, assign30440_e48079_d_n12, assign30440_e48079_d_n14, assign30440_e48079_d_n15, assign30440_e48079_d_n16, assign30440_e48079_d_n17, assign30440_e48079_d_n18, assign30440_e48079_d_n19, assign30440_e48079_d_n20, assign30440_e48079_d_n21, assign30440_e48079_d_n22,) = {
    if (((locals.var_guard504 == 0.0) && (locals.var_guard513 != 0.0)) && (locals.var_guard518 != 0.0)) {
        let assign30440_e48075: f64 = (p.p209 * locals.var_t0);
        let assign30440_e48077: f64 = (assign30440_e48075 * locals.var_t2);
        (assign30440_e48077, (((p.p209 * locals.var_t0_dn0) * locals.var_t2) + (assign30440_e48075 * locals.var_t2_dn0)), (((p.p209 * locals.var_t0_dn1) * locals.var_t2) + (assign30440_e48075 * locals.var_t2_dn1)), (((p.p209 * locals.var_t0_dn2) * locals.var_t2) + (assign30440_e48075 * locals.var_t2_dn2)), (((p.p209 * locals.var_t0_dn3) * locals.var_t2) + (assign30440_e48075 * locals.var_t2_dn3)), (((p.p209 * locals.var_t0_dn4) * locals.var_t2) + (assign30440_e48075 * locals.var_t2_dn4)), (((p.p209 * locals.var_t0_dn5) * locals.var_t2) + (assign30440_e48075 * locals.var_t2_dn5)), (((p.p209 * locals.var_t0_dn6) * locals.var_t2) + (assign30440_e48075 * locals.var_t2_dn6)), (((p.p209 * locals.var_t0_dn7) * locals.var_t2) + (assign30440_e48075 * locals.var_t2_dn7)), (((p.p209 * locals.var_t0_dn8) * locals.var_t2) + (assign30440_e48075 * locals.var_t2_dn8)), (((p.p209 * locals.var_t0_dn9) * locals.var_t2) + (assign30440_e48075 * locals.var_t2_dn9)), (((p.p209 * locals.var_t0_dn12) * locals.var_t2) + (assign30440_e48075 * locals.var_t2_dn12)), (((p.p209 * locals.var_t0_dn14) * locals.var_t2) + (assign30440_e48075 * locals.var_t2_dn14)), (((p.p209 * locals.var_t0_dn15) * locals.var_t2) + (assign30440_e48075 * locals.var_t2_dn15)), (((p.p209 * locals.var_t0_dn16) * locals.var_t2) + (assign30440_e48075 * locals.var_t2_dn16)), (((p.p209 * locals.var_t0_dn17) * locals.var_t2) + (assign30440_e48075 * locals.var_t2_dn17)), (((p.p209 * locals.var_t0_dn18) * locals.var_t2) + (assign30440_e48075 * locals.var_t2_dn18)), (((p.p209 * locals.var_t0_dn19) * locals.var_t2) + (assign30440_e48075 * locals.var_t2_dn19)), (((p.p209 * locals.var_t0_dn20) * locals.var_t2) + (assign30440_e48075 * locals.var_t2_dn20)), (((p.p209 * locals.var_t0_dn21) * locals.var_t2) + (assign30440_e48075 * locals.var_t2_dn21)), (((p.p209 * locals.var_t0_dn22) * locals.var_t2) + (assign30440_e48075 * locals.var_t2_dn22)),)
    } else {
        (locals.var_vgefm13g1, locals.var_vgefm13g1_dn0, locals.var_vgefm13g1_dn1, locals.var_vgefm13g1_dn2, locals.var_vgefm13g1_dn3, locals.var_vgefm13g1_dn4, locals.var_vgefm13g1_dn5, locals.var_vgefm13g1_dn6, locals.var_vgefm13g1_dn7, locals.var_vgefm13g1_dn8, locals.var_vgefm13g1_dn9, locals.var_vgefm13g1_dn12, locals.var_vgefm13g1_dn14, locals.var_vgefm13g1_dn15, locals.var_vgefm13g1_dn16, locals.var_vgefm13g1_dn17, locals.var_vgefm13g1_dn18, locals.var_vgefm13g1_dn19, locals.var_vgefm13g1_dn20, locals.var_vgefm13g1_dn21, locals.var_vgefm13g1_dn22,)
    }
};
        locals.var_vgefm13g1 = assign30440_e48079;
        locals.var_vgefm13g1_dn0 = assign30440_e48079_d_n0;
        locals.var_vgefm13g1_dn1 = assign30440_e48079_d_n1;
        locals.var_vgefm13g1_dn2 = assign30440_e48079_d_n2;
        locals.var_vgefm13g1_dn3 = assign30440_e48079_d_n3;
        locals.var_vgefm13g1_dn4 = assign30440_e48079_d_n4;
        locals.var_vgefm13g1_dn5 = assign30440_e48079_d_n5;
        locals.var_vgefm13g1_dn6 = assign30440_e48079_d_n6;
        locals.var_vgefm13g1_dn7 = assign30440_e48079_d_n7;
        locals.var_vgefm13g1_dn8 = assign30440_e48079_d_n8;
        locals.var_vgefm13g1_dn9 = assign30440_e48079_d_n9;
        locals.var_vgefm13g1_dn12 = assign30440_e48079_d_n12;
        locals.var_vgefm13g1_dn14 = assign30440_e48079_d_n14;
        locals.var_vgefm13g1_dn15 = assign30440_e48079_d_n15;
        locals.var_vgefm13g1_dn16 = assign30440_e48079_d_n16;
        locals.var_vgefm13g1_dn17 = assign30440_e48079_d_n17;
        locals.var_vgefm13g1_dn18 = assign30440_e48079_d_n18;
        locals.var_vgefm13g1_dn19 = assign30440_e48079_d_n19;
        locals.var_vgefm13g1_dn20 = assign30440_e48079_d_n20;
        locals.var_vgefm13g1_dn21 = assign30440_e48079_d_n21;
        locals.var_vgefm13g1_dn22 = assign30440_e48079_d_n22;

        let (assign30450_e48097, assign30450_e48097_d_n0, assign30450_e48097_d_n1, assign30450_e48097_d_n2, assign30450_e48097_d_n3, assign30450_e48097_d_n4, assign30450_e48097_d_n5, assign30450_e48097_d_n6, assign30450_e48097_d_n7, assign30450_e48097_d_n8, assign30450_e48097_d_n9, assign30450_e48097_d_n12, assign30450_e48097_d_n14, assign30450_e48097_d_n15, assign30450_e48097_d_n16, assign30450_e48097_d_n17, assign30450_e48097_d_n18, assign30450_e48097_d_n19, assign30450_e48097_d_n20, assign30450_e48097_d_n21, assign30450_e48097_d_n22,) = {
    if (((locals.var_guard504 == 0.0) && (locals.var_guard513 != 0.0)) && (locals.var_guard518 != 0.0)) {
        let assign30450_e48087: f64 = { let limited_exp_arg = locals.var_tg0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign30450_e48089: f64 = (assign30450_e48087 * 3.24e17);
        let assign30450_e48093: f64 = (0.6666666666666666 * locals.var_vgefm13g0);
        let assign30450_e48094: f64 = (1.0 + assign30450_e48093);
        let assign30450_e48095: f64 = (assign30450_e48089 * assign30450_e48094);
        (assign30450_e48095, (((({ let limited_exp_arg = locals.var_tg0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg0_dn0) * 3.24e17) * assign30450_e48094) + (assign30450_e48089 * (0.6666666666666666 * locals.var_vgefm13g0_dn0))), (((({ let limited_exp_arg = locals.var_tg0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg0_dn1) * 3.24e17) * assign30450_e48094) + (assign30450_e48089 * (0.6666666666666666 * locals.var_vgefm13g0_dn1))), (((({ let limited_exp_arg = locals.var_tg0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg0_dn2) * 3.24e17) * assign30450_e48094) + (assign30450_e48089 * (0.6666666666666666 * locals.var_vgefm13g0_dn2))), (((({ let limited_exp_arg = locals.var_tg0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg0_dn3) * 3.24e17) * assign30450_e48094) + (assign30450_e48089 * (0.6666666666666666 * locals.var_vgefm13g0_dn3))), (((({ let limited_exp_arg = locals.var_tg0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg0_dn4) * 3.24e17) * assign30450_e48094) + (assign30450_e48089 * (0.6666666666666666 * locals.var_vgefm13g0_dn4))), (((({ let limited_exp_arg = locals.var_tg0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg0_dn5) * 3.24e17) * assign30450_e48094) + (assign30450_e48089 * (0.6666666666666666 * locals.var_vgefm13g0_dn5))), (((({ let limited_exp_arg = locals.var_tg0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg0_dn6) * 3.24e17) * assign30450_e48094) + (assign30450_e48089 * (0.6666666666666666 * locals.var_vgefm13g0_dn6))), (((({ let limited_exp_arg = locals.var_tg0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg0_dn7) * 3.24e17) * assign30450_e48094) + (assign30450_e48089 * (0.6666666666666666 * locals.var_vgefm13g0_dn7))), (((({ let limited_exp_arg = locals.var_tg0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg0_dn8) * 3.24e17) * assign30450_e48094) + (assign30450_e48089 * (0.6666666666666666 * locals.var_vgefm13g0_dn8))), (((({ let limited_exp_arg = locals.var_tg0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg0_dn9) * 3.24e17) * assign30450_e48094) + (assign30450_e48089 * (0.6666666666666666 * locals.var_vgefm13g0_dn9))), (((({ let limited_exp_arg = locals.var_tg0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg0_dn12) * 3.24e17) * assign30450_e48094) + (assign30450_e48089 * (0.6666666666666666 * locals.var_vgefm13g0_dn12))), (((({ let limited_exp_arg = locals.var_tg0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg0_dn14) * 3.24e17) * assign30450_e48094) + (assign30450_e48089 * (0.6666666666666666 * locals.var_vgefm13g0_dn14))), (((({ let limited_exp_arg = locals.var_tg0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg0_dn15) * 3.24e17) * assign30450_e48094) + (assign30450_e48089 * (0.6666666666666666 * locals.var_vgefm13g0_dn15))), (((({ let limited_exp_arg = locals.var_tg0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg0_dn16) * 3.24e17) * assign30450_e48094) + (assign30450_e48089 * (0.6666666666666666 * locals.var_vgefm13g0_dn16))), (((({ let limited_exp_arg = locals.var_tg0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg0_dn17) * 3.24e17) * assign30450_e48094) + (assign30450_e48089 * (0.6666666666666666 * locals.var_vgefm13g0_dn17))), (((({ let limited_exp_arg = locals.var_tg0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg0_dn18) * 3.24e17) * assign30450_e48094) + (assign30450_e48089 * (0.6666666666666666 * locals.var_vgefm13g0_dn18))), (((({ let limited_exp_arg = locals.var_tg0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg0_dn19) * 3.24e17) * assign30450_e48094) + (assign30450_e48089 * (0.6666666666666666 * locals.var_vgefm13g0_dn19))), (((({ let limited_exp_arg = locals.var_tg0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg0_dn20) * 3.24e17) * assign30450_e48094) + (assign30450_e48089 * (0.6666666666666666 * locals.var_vgefm13g0_dn20))), (((({ let limited_exp_arg = locals.var_tg0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg0_dn21) * 3.24e17) * assign30450_e48094) + (assign30450_e48089 * (0.6666666666666666 * locals.var_vgefm13g0_dn21))), (((({ let limited_exp_arg = locals.var_tg0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg0_dn22) * 3.24e17) * assign30450_e48094) + (assign30450_e48089 * (0.6666666666666666 * locals.var_vgefm13g0_dn22))),)
    } else {
        (locals.var_t5ng0, locals.var_t5ng0_dn0, locals.var_t5ng0_dn1, locals.var_t5ng0_dn2, locals.var_t5ng0_dn3, locals.var_t5ng0_dn4, locals.var_t5ng0_dn5, locals.var_t5ng0_dn6, locals.var_t5ng0_dn7, locals.var_t5ng0_dn8, locals.var_t5ng0_dn9, locals.var_t5ng0_dn12, locals.var_t5ng0_dn14, locals.var_t5ng0_dn15, locals.var_t5ng0_dn16, locals.var_t5ng0_dn17, locals.var_t5ng0_dn18, locals.var_t5ng0_dn19, locals.var_t5ng0_dn20, locals.var_t5ng0_dn21, locals.var_t5ng0_dn22,)
    }
};
        locals.var_t5ng0 = assign30450_e48097;
        locals.var_t5ng0_dn0 = assign30450_e48097_d_n0;
        locals.var_t5ng0_dn1 = assign30450_e48097_d_n1;
        locals.var_t5ng0_dn2 = assign30450_e48097_d_n2;
        locals.var_t5ng0_dn3 = assign30450_e48097_d_n3;
        locals.var_t5ng0_dn4 = assign30450_e48097_d_n4;
        locals.var_t5ng0_dn5 = assign30450_e48097_d_n5;
        locals.var_t5ng0_dn6 = assign30450_e48097_d_n6;
        locals.var_t5ng0_dn7 = assign30450_e48097_d_n7;
        locals.var_t5ng0_dn8 = assign30450_e48097_d_n8;
        locals.var_t5ng0_dn9 = assign30450_e48097_d_n9;
        locals.var_t5ng0_dn12 = assign30450_e48097_d_n12;
        locals.var_t5ng0_dn14 = assign30450_e48097_d_n14;
        locals.var_t5ng0_dn15 = assign30450_e48097_d_n15;
        locals.var_t5ng0_dn16 = assign30450_e48097_d_n16;
        locals.var_t5ng0_dn17 = assign30450_e48097_d_n17;
        locals.var_t5ng0_dn18 = assign30450_e48097_d_n18;
        locals.var_t5ng0_dn19 = assign30450_e48097_d_n19;
        locals.var_t5ng0_dn20 = assign30450_e48097_d_n20;
        locals.var_t5ng0_dn21 = assign30450_e48097_d_n21;
        locals.var_t5ng0_dn22 = assign30450_e48097_d_n22;

        let (assign30460_e48109, assign30460_e48109_d_n0, assign30460_e48109_d_n1, assign30460_e48109_d_n2, assign30460_e48109_d_n3, assign30460_e48109_d_n4, assign30460_e48109_d_n5, assign30460_e48109_d_n6, assign30460_e48109_d_n7, assign30460_e48109_d_n8, assign30460_e48109_d_n9, assign30460_e48109_d_n12, assign30460_e48109_d_n14, assign30460_e48109_d_n15, assign30460_e48109_d_n16, assign30460_e48109_d_n17, assign30460_e48109_d_n18, assign30460_e48109_d_n19, assign30460_e48109_d_n20, assign30460_e48109_d_n21, assign30460_e48109_d_n22,) = {
    if (((locals.var_guard504 == 0.0) && (locals.var_guard513 != 0.0)) && (locals.var_guard518 != 0.0)) {
        let assign30460_e48106: f64 = { let limited_exp_arg = locals.var_tg0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign30460_e48107: f64 = (1.0 + assign30460_e48106);
        (assign30460_e48107, ({ let limited_exp_arg = locals.var_tg0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg0_dn0), ({ let limited_exp_arg = locals.var_tg0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg0_dn1), ({ let limited_exp_arg = locals.var_tg0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg0_dn2), ({ let limited_exp_arg = locals.var_tg0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg0_dn3), ({ let limited_exp_arg = locals.var_tg0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg0_dn4), ({ let limited_exp_arg = locals.var_tg0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg0_dn5), ({ let limited_exp_arg = locals.var_tg0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg0_dn6), ({ let limited_exp_arg = locals.var_tg0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg0_dn7), ({ let limited_exp_arg = locals.var_tg0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg0_dn8), ({ let limited_exp_arg = locals.var_tg0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg0_dn9), ({ let limited_exp_arg = locals.var_tg0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg0_dn12), ({ let limited_exp_arg = locals.var_tg0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg0_dn14), ({ let limited_exp_arg = locals.var_tg0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg0_dn15), ({ let limited_exp_arg = locals.var_tg0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg0_dn16), ({ let limited_exp_arg = locals.var_tg0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg0_dn17), ({ let limited_exp_arg = locals.var_tg0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg0_dn18), ({ let limited_exp_arg = locals.var_tg0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg0_dn19), ({ let limited_exp_arg = locals.var_tg0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg0_dn20), ({ let limited_exp_arg = locals.var_tg0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg0_dn21), ({ let limited_exp_arg = locals.var_tg0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg0_dn22),)
    } else {
        (locals.var_t5dg0, locals.var_t5dg0_dn0, locals.var_t5dg0_dn1, locals.var_t5dg0_dn2, locals.var_t5dg0_dn3, locals.var_t5dg0_dn4, locals.var_t5dg0_dn5, locals.var_t5dg0_dn6, locals.var_t5dg0_dn7, locals.var_t5dg0_dn8, locals.var_t5dg0_dn9, locals.var_t5dg0_dn12, locals.var_t5dg0_dn14, locals.var_t5dg0_dn15, locals.var_t5dg0_dn16, locals.var_t5dg0_dn17, locals.var_t5dg0_dn18, locals.var_t5dg0_dn19, locals.var_t5dg0_dn20, locals.var_t5dg0_dn21, locals.var_t5dg0_dn22,)
    }
};
        locals.var_t5dg0 = assign30460_e48109;
        locals.var_t5dg0_dn0 = assign30460_e48109_d_n0;
        locals.var_t5dg0_dn1 = assign30460_e48109_d_n1;
        locals.var_t5dg0_dn2 = assign30460_e48109_d_n2;
        locals.var_t5dg0_dn3 = assign30460_e48109_d_n3;
        locals.var_t5dg0_dn4 = assign30460_e48109_d_n4;
        locals.var_t5dg0_dn5 = assign30460_e48109_d_n5;
        locals.var_t5dg0_dn6 = assign30460_e48109_d_n6;
        locals.var_t5dg0_dn7 = assign30460_e48109_d_n7;
        locals.var_t5dg0_dn8 = assign30460_e48109_d_n8;
        locals.var_t5dg0_dn9 = assign30460_e48109_d_n9;
        locals.var_t5dg0_dn12 = assign30460_e48109_d_n12;
        locals.var_t5dg0_dn14 = assign30460_e48109_d_n14;
        locals.var_t5dg0_dn15 = assign30460_e48109_d_n15;
        locals.var_t5dg0_dn16 = assign30460_e48109_d_n16;
        locals.var_t5dg0_dn17 = assign30460_e48109_d_n17;
        locals.var_t5dg0_dn18 = assign30460_e48109_d_n18;
        locals.var_t5dg0_dn19 = assign30460_e48109_d_n19;
        locals.var_t5dg0_dn20 = assign30460_e48109_d_n20;
        locals.var_t5dg0_dn21 = assign30460_e48109_d_n21;
        locals.var_t5dg0_dn22 = assign30460_e48109_d_n22;

    }

    pub(super) fn stamp_transient_block_179(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign30470_e48127, assign30470_e48127_d_n0, assign30470_e48127_d_n1, assign30470_e48127_d_n2, assign30470_e48127_d_n3, assign30470_e48127_d_n4, assign30470_e48127_d_n5, assign30470_e48127_d_n6, assign30470_e48127_d_n7, assign30470_e48127_d_n8, assign30470_e48127_d_n9, assign30470_e48127_d_n12, assign30470_e48127_d_n14, assign30470_e48127_d_n15, assign30470_e48127_d_n16, assign30470_e48127_d_n17, assign30470_e48127_d_n18, assign30470_e48127_d_n19, assign30470_e48127_d_n20, assign30470_e48127_d_n21, assign30470_e48127_d_n22,) = {
    if (((locals.var_guard504 == 0.0) && (locals.var_guard513 != 0.0)) && (locals.var_guard518 != 0.0)) {
        let assign30470_e48117: f64 = { let limited_exp_arg = locals.var_tg1; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign30470_e48119: f64 = (assign30470_e48117 * 3.24e17);
        let assign30470_e48123: f64 = (0.6666666666666666 * locals.var_vgefm13g1);
        let assign30470_e48124: f64 = (1.0 + assign30470_e48123);
        let assign30470_e48125: f64 = (assign30470_e48119 * assign30470_e48124);
        (assign30470_e48125, (((({ let limited_exp_arg = locals.var_tg1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg1_dn0) * 3.24e17) * assign30470_e48124) + (assign30470_e48119 * (0.6666666666666666 * locals.var_vgefm13g1_dn0))), (((({ let limited_exp_arg = locals.var_tg1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg1_dn1) * 3.24e17) * assign30470_e48124) + (assign30470_e48119 * (0.6666666666666666 * locals.var_vgefm13g1_dn1))), (((({ let limited_exp_arg = locals.var_tg1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg1_dn2) * 3.24e17) * assign30470_e48124) + (assign30470_e48119 * (0.6666666666666666 * locals.var_vgefm13g1_dn2))), (((({ let limited_exp_arg = locals.var_tg1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg1_dn3) * 3.24e17) * assign30470_e48124) + (assign30470_e48119 * (0.6666666666666666 * locals.var_vgefm13g1_dn3))), (((({ let limited_exp_arg = locals.var_tg1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg1_dn4) * 3.24e17) * assign30470_e48124) + (assign30470_e48119 * (0.6666666666666666 * locals.var_vgefm13g1_dn4))), (((({ let limited_exp_arg = locals.var_tg1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg1_dn5) * 3.24e17) * assign30470_e48124) + (assign30470_e48119 * (0.6666666666666666 * locals.var_vgefm13g1_dn5))), (((({ let limited_exp_arg = locals.var_tg1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg1_dn6) * 3.24e17) * assign30470_e48124) + (assign30470_e48119 * (0.6666666666666666 * locals.var_vgefm13g1_dn6))), (((({ let limited_exp_arg = locals.var_tg1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg1_dn7) * 3.24e17) * assign30470_e48124) + (assign30470_e48119 * (0.6666666666666666 * locals.var_vgefm13g1_dn7))), (((({ let limited_exp_arg = locals.var_tg1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg1_dn8) * 3.24e17) * assign30470_e48124) + (assign30470_e48119 * (0.6666666666666666 * locals.var_vgefm13g1_dn8))), (((({ let limited_exp_arg = locals.var_tg1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg1_dn9) * 3.24e17) * assign30470_e48124) + (assign30470_e48119 * (0.6666666666666666 * locals.var_vgefm13g1_dn9))), (((({ let limited_exp_arg = locals.var_tg1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg1_dn12) * 3.24e17) * assign30470_e48124) + (assign30470_e48119 * (0.6666666666666666 * locals.var_vgefm13g1_dn12))), (((({ let limited_exp_arg = locals.var_tg1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg1_dn14) * 3.24e17) * assign30470_e48124) + (assign30470_e48119 * (0.6666666666666666 * locals.var_vgefm13g1_dn14))), (((({ let limited_exp_arg = locals.var_tg1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg1_dn15) * 3.24e17) * assign30470_e48124) + (assign30470_e48119 * (0.6666666666666666 * locals.var_vgefm13g1_dn15))), (((({ let limited_exp_arg = locals.var_tg1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg1_dn16) * 3.24e17) * assign30470_e48124) + (assign30470_e48119 * (0.6666666666666666 * locals.var_vgefm13g1_dn16))), (((({ let limited_exp_arg = locals.var_tg1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg1_dn17) * 3.24e17) * assign30470_e48124) + (assign30470_e48119 * (0.6666666666666666 * locals.var_vgefm13g1_dn17))), (((({ let limited_exp_arg = locals.var_tg1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg1_dn18) * 3.24e17) * assign30470_e48124) + (assign30470_e48119 * (0.6666666666666666 * locals.var_vgefm13g1_dn18))), (((({ let limited_exp_arg = locals.var_tg1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg1_dn19) * 3.24e17) * assign30470_e48124) + (assign30470_e48119 * (0.6666666666666666 * locals.var_vgefm13g1_dn19))), (((({ let limited_exp_arg = locals.var_tg1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg1_dn20) * 3.24e17) * assign30470_e48124) + (assign30470_e48119 * (0.6666666666666666 * locals.var_vgefm13g1_dn20))), (((({ let limited_exp_arg = locals.var_tg1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg1_dn21) * 3.24e17) * assign30470_e48124) + (assign30470_e48119 * (0.6666666666666666 * locals.var_vgefm13g1_dn21))), (((({ let limited_exp_arg = locals.var_tg1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg1_dn22) * 3.24e17) * assign30470_e48124) + (assign30470_e48119 * (0.6666666666666666 * locals.var_vgefm13g1_dn22))),)
    } else {
        (locals.var_t5ng1, locals.var_t5ng1_dn0, locals.var_t5ng1_dn1, locals.var_t5ng1_dn2, locals.var_t5ng1_dn3, locals.var_t5ng1_dn4, locals.var_t5ng1_dn5, locals.var_t5ng1_dn6, locals.var_t5ng1_dn7, locals.var_t5ng1_dn8, locals.var_t5ng1_dn9, locals.var_t5ng1_dn12, locals.var_t5ng1_dn14, locals.var_t5ng1_dn15, locals.var_t5ng1_dn16, locals.var_t5ng1_dn17, locals.var_t5ng1_dn18, locals.var_t5ng1_dn19, locals.var_t5ng1_dn20, locals.var_t5ng1_dn21, locals.var_t5ng1_dn22,)
    }
};
        locals.var_t5ng1 = assign30470_e48127;
        locals.var_t5ng1_dn0 = assign30470_e48127_d_n0;
        locals.var_t5ng1_dn1 = assign30470_e48127_d_n1;
        locals.var_t5ng1_dn2 = assign30470_e48127_d_n2;
        locals.var_t5ng1_dn3 = assign30470_e48127_d_n3;
        locals.var_t5ng1_dn4 = assign30470_e48127_d_n4;
        locals.var_t5ng1_dn5 = assign30470_e48127_d_n5;
        locals.var_t5ng1_dn6 = assign30470_e48127_d_n6;
        locals.var_t5ng1_dn7 = assign30470_e48127_d_n7;
        locals.var_t5ng1_dn8 = assign30470_e48127_d_n8;
        locals.var_t5ng1_dn9 = assign30470_e48127_d_n9;
        locals.var_t5ng1_dn12 = assign30470_e48127_d_n12;
        locals.var_t5ng1_dn14 = assign30470_e48127_d_n14;
        locals.var_t5ng1_dn15 = assign30470_e48127_d_n15;
        locals.var_t5ng1_dn16 = assign30470_e48127_d_n16;
        locals.var_t5ng1_dn17 = assign30470_e48127_d_n17;
        locals.var_t5ng1_dn18 = assign30470_e48127_d_n18;
        locals.var_t5ng1_dn19 = assign30470_e48127_d_n19;
        locals.var_t5ng1_dn20 = assign30470_e48127_d_n20;
        locals.var_t5ng1_dn21 = assign30470_e48127_d_n21;
        locals.var_t5ng1_dn22 = assign30470_e48127_d_n22;

        let (assign30480_e48139, assign30480_e48139_d_n0, assign30480_e48139_d_n1, assign30480_e48139_d_n2, assign30480_e48139_d_n3, assign30480_e48139_d_n4, assign30480_e48139_d_n5, assign30480_e48139_d_n6, assign30480_e48139_d_n7, assign30480_e48139_d_n8, assign30480_e48139_d_n9, assign30480_e48139_d_n12, assign30480_e48139_d_n14, assign30480_e48139_d_n15, assign30480_e48139_d_n16, assign30480_e48139_d_n17, assign30480_e48139_d_n18, assign30480_e48139_d_n19, assign30480_e48139_d_n20, assign30480_e48139_d_n21, assign30480_e48139_d_n22,) = {
    if (((locals.var_guard504 == 0.0) && (locals.var_guard513 != 0.0)) && (locals.var_guard518 != 0.0)) {
        let assign30480_e48136: f64 = { let limited_exp_arg = locals.var_tg1; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign30480_e48137: f64 = (1.0 + assign30480_e48136);
        (assign30480_e48137, ({ let limited_exp_arg = locals.var_tg1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg1_dn0), ({ let limited_exp_arg = locals.var_tg1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg1_dn1), ({ let limited_exp_arg = locals.var_tg1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg1_dn2), ({ let limited_exp_arg = locals.var_tg1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg1_dn3), ({ let limited_exp_arg = locals.var_tg1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg1_dn4), ({ let limited_exp_arg = locals.var_tg1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg1_dn5), ({ let limited_exp_arg = locals.var_tg1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg1_dn6), ({ let limited_exp_arg = locals.var_tg1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg1_dn7), ({ let limited_exp_arg = locals.var_tg1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg1_dn8), ({ let limited_exp_arg = locals.var_tg1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg1_dn9), ({ let limited_exp_arg = locals.var_tg1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg1_dn12), ({ let limited_exp_arg = locals.var_tg1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg1_dn14), ({ let limited_exp_arg = locals.var_tg1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg1_dn15), ({ let limited_exp_arg = locals.var_tg1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg1_dn16), ({ let limited_exp_arg = locals.var_tg1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg1_dn17), ({ let limited_exp_arg = locals.var_tg1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg1_dn18), ({ let limited_exp_arg = locals.var_tg1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg1_dn19), ({ let limited_exp_arg = locals.var_tg1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg1_dn20), ({ let limited_exp_arg = locals.var_tg1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg1_dn21), ({ let limited_exp_arg = locals.var_tg1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg1_dn22),)
    } else {
        (locals.var_t5dg1, locals.var_t5dg1_dn0, locals.var_t5dg1_dn1, locals.var_t5dg1_dn2, locals.var_t5dg1_dn3, locals.var_t5dg1_dn4, locals.var_t5dg1_dn5, locals.var_t5dg1_dn6, locals.var_t5dg1_dn7, locals.var_t5dg1_dn8, locals.var_t5dg1_dn9, locals.var_t5dg1_dn12, locals.var_t5dg1_dn14, locals.var_t5dg1_dn15, locals.var_t5dg1_dn16, locals.var_t5dg1_dn17, locals.var_t5dg1_dn18, locals.var_t5dg1_dn19, locals.var_t5dg1_dn20, locals.var_t5dg1_dn21, locals.var_t5dg1_dn22,)
    }
};
        locals.var_t5dg1 = assign30480_e48139;
        locals.var_t5dg1_dn0 = assign30480_e48139_d_n0;
        locals.var_t5dg1_dn1 = assign30480_e48139_d_n1;
        locals.var_t5dg1_dn2 = assign30480_e48139_d_n2;
        locals.var_t5dg1_dn3 = assign30480_e48139_d_n3;
        locals.var_t5dg1_dn4 = assign30480_e48139_d_n4;
        locals.var_t5dg1_dn5 = assign30480_e48139_d_n5;
        locals.var_t5dg1_dn6 = assign30480_e48139_d_n6;
        locals.var_t5dg1_dn7 = assign30480_e48139_d_n7;
        locals.var_t5dg1_dn8 = assign30480_e48139_d_n8;
        locals.var_t5dg1_dn9 = assign30480_e48139_d_n9;
        locals.var_t5dg1_dn12 = assign30480_e48139_d_n12;
        locals.var_t5dg1_dn14 = assign30480_e48139_d_n14;
        locals.var_t5dg1_dn15 = assign30480_e48139_d_n15;
        locals.var_t5dg1_dn16 = assign30480_e48139_d_n16;
        locals.var_t5dg1_dn17 = assign30480_e48139_d_n17;
        locals.var_t5dg1_dn18 = assign30480_e48139_d_n18;
        locals.var_t5dg1_dn19 = assign30480_e48139_d_n19;
        locals.var_t5dg1_dn20 = assign30480_e48139_d_n20;
        locals.var_t5dg1_dn21 = assign30480_e48139_d_n21;
        locals.var_t5dg1_dn22 = assign30480_e48139_d_n22;

        let (assign30490_e48159, assign30490_e48159_d_n0, assign30490_e48159_d_n1, assign30490_e48159_d_n2, assign30490_e48159_d_n3, assign30490_e48159_d_n4, assign30490_e48159_d_n5, assign30490_e48159_d_n6, assign30490_e48159_d_n7, assign30490_e48159_d_n8, assign30490_e48159_d_n9, assign30490_e48159_d_n12, assign30490_e48159_d_n14, assign30490_e48159_d_n15, assign30490_e48159_d_n16, assign30490_e48159_d_n17, assign30490_e48159_d_n18, assign30490_e48159_d_n19, assign30490_e48159_d_n20, assign30490_e48159_d_n21, assign30490_e48159_d_n22,) = {
    if (((locals.var_guard504 == 0.0) && (locals.var_guard513 != 0.0)) && (locals.var_guard518 != 0.0)) {
        let assign30490_e48147: f64 = (-1.0);
        let assign30490_e48149: f64 = (assign30490_e48147 * locals.var_cch);
        let assign30490_e48152: f64 = (locals.var_t5ng0 / locals.var_t5dg0);
        let assign30490_e48153: f64 = (assign30490_e48149 - assign30490_e48152);
        let assign30490_e48156: f64 = (locals.var_t5ng1 / locals.var_t5dg1);
        let assign30490_e48157: f64 = (assign30490_e48153 - assign30490_e48156);
        (assign30490_e48157, ((-(((locals.var_t5ng0_dn0 * locals.var_t5dg0) - (locals.var_t5ng0 * locals.var_t5dg0_dn0)) / (locals.var_t5dg0 * locals.var_t5dg0))) - (((locals.var_t5ng1_dn0 * locals.var_t5dg1) - (locals.var_t5ng1 * locals.var_t5dg1_dn0)) / (locals.var_t5dg1 * locals.var_t5dg1))), ((-(((locals.var_t5ng0_dn1 * locals.var_t5dg0) - (locals.var_t5ng0 * locals.var_t5dg0_dn1)) / (locals.var_t5dg0 * locals.var_t5dg0))) - (((locals.var_t5ng1_dn1 * locals.var_t5dg1) - (locals.var_t5ng1 * locals.var_t5dg1_dn1)) / (locals.var_t5dg1 * locals.var_t5dg1))), ((-(((locals.var_t5ng0_dn2 * locals.var_t5dg0) - (locals.var_t5ng0 * locals.var_t5dg0_dn2)) / (locals.var_t5dg0 * locals.var_t5dg0))) - (((locals.var_t5ng1_dn2 * locals.var_t5dg1) - (locals.var_t5ng1 * locals.var_t5dg1_dn2)) / (locals.var_t5dg1 * locals.var_t5dg1))), ((-(((locals.var_t5ng0_dn3 * locals.var_t5dg0) - (locals.var_t5ng0 * locals.var_t5dg0_dn3)) / (locals.var_t5dg0 * locals.var_t5dg0))) - (((locals.var_t5ng1_dn3 * locals.var_t5dg1) - (locals.var_t5ng1 * locals.var_t5dg1_dn3)) / (locals.var_t5dg1 * locals.var_t5dg1))), ((-(((locals.var_t5ng0_dn4 * locals.var_t5dg0) - (locals.var_t5ng0 * locals.var_t5dg0_dn4)) / (locals.var_t5dg0 * locals.var_t5dg0))) - (((locals.var_t5ng1_dn4 * locals.var_t5dg1) - (locals.var_t5ng1 * locals.var_t5dg1_dn4)) / (locals.var_t5dg1 * locals.var_t5dg1))), ((-(((locals.var_t5ng0_dn5 * locals.var_t5dg0) - (locals.var_t5ng0 * locals.var_t5dg0_dn5)) / (locals.var_t5dg0 * locals.var_t5dg0))) - (((locals.var_t5ng1_dn5 * locals.var_t5dg1) - (locals.var_t5ng1 * locals.var_t5dg1_dn5)) / (locals.var_t5dg1 * locals.var_t5dg1))), ((-(((locals.var_t5ng0_dn6 * locals.var_t5dg0) - (locals.var_t5ng0 * locals.var_t5dg0_dn6)) / (locals.var_t5dg0 * locals.var_t5dg0))) - (((locals.var_t5ng1_dn6 * locals.var_t5dg1) - (locals.var_t5ng1 * locals.var_t5dg1_dn6)) / (locals.var_t5dg1 * locals.var_t5dg1))), ((-(((locals.var_t5ng0_dn7 * locals.var_t5dg0) - (locals.var_t5ng0 * locals.var_t5dg0_dn7)) / (locals.var_t5dg0 * locals.var_t5dg0))) - (((locals.var_t5ng1_dn7 * locals.var_t5dg1) - (locals.var_t5ng1 * locals.var_t5dg1_dn7)) / (locals.var_t5dg1 * locals.var_t5dg1))), ((-(((locals.var_t5ng0_dn8 * locals.var_t5dg0) - (locals.var_t5ng0 * locals.var_t5dg0_dn8)) / (locals.var_t5dg0 * locals.var_t5dg0))) - (((locals.var_t5ng1_dn8 * locals.var_t5dg1) - (locals.var_t5ng1 * locals.var_t5dg1_dn8)) / (locals.var_t5dg1 * locals.var_t5dg1))), ((-(((locals.var_t5ng0_dn9 * locals.var_t5dg0) - (locals.var_t5ng0 * locals.var_t5dg0_dn9)) / (locals.var_t5dg0 * locals.var_t5dg0))) - (((locals.var_t5ng1_dn9 * locals.var_t5dg1) - (locals.var_t5ng1 * locals.var_t5dg1_dn9)) / (locals.var_t5dg1 * locals.var_t5dg1))), ((-(((locals.var_t5ng0_dn12 * locals.var_t5dg0) - (locals.var_t5ng0 * locals.var_t5dg0_dn12)) / (locals.var_t5dg0 * locals.var_t5dg0))) - (((locals.var_t5ng1_dn12 * locals.var_t5dg1) - (locals.var_t5ng1 * locals.var_t5dg1_dn12)) / (locals.var_t5dg1 * locals.var_t5dg1))), ((-(((locals.var_t5ng0_dn14 * locals.var_t5dg0) - (locals.var_t5ng0 * locals.var_t5dg0_dn14)) / (locals.var_t5dg0 * locals.var_t5dg0))) - (((locals.var_t5ng1_dn14 * locals.var_t5dg1) - (locals.var_t5ng1 * locals.var_t5dg1_dn14)) / (locals.var_t5dg1 * locals.var_t5dg1))), ((-(((locals.var_t5ng0_dn15 * locals.var_t5dg0) - (locals.var_t5ng0 * locals.var_t5dg0_dn15)) / (locals.var_t5dg0 * locals.var_t5dg0))) - (((locals.var_t5ng1_dn15 * locals.var_t5dg1) - (locals.var_t5ng1 * locals.var_t5dg1_dn15)) / (locals.var_t5dg1 * locals.var_t5dg1))), ((-(((locals.var_t5ng0_dn16 * locals.var_t5dg0) - (locals.var_t5ng0 * locals.var_t5dg0_dn16)) / (locals.var_t5dg0 * locals.var_t5dg0))) - (((locals.var_t5ng1_dn16 * locals.var_t5dg1) - (locals.var_t5ng1 * locals.var_t5dg1_dn16)) / (locals.var_t5dg1 * locals.var_t5dg1))), ((-(((locals.var_t5ng0_dn17 * locals.var_t5dg0) - (locals.var_t5ng0 * locals.var_t5dg0_dn17)) / (locals.var_t5dg0 * locals.var_t5dg0))) - (((locals.var_t5ng1_dn17 * locals.var_t5dg1) - (locals.var_t5ng1 * locals.var_t5dg1_dn17)) / (locals.var_t5dg1 * locals.var_t5dg1))), ((-(((locals.var_t5ng0_dn18 * locals.var_t5dg0) - (locals.var_t5ng0 * locals.var_t5dg0_dn18)) / (locals.var_t5dg0 * locals.var_t5dg0))) - (((locals.var_t5ng1_dn18 * locals.var_t5dg1) - (locals.var_t5ng1 * locals.var_t5dg1_dn18)) / (locals.var_t5dg1 * locals.var_t5dg1))), ((-(((locals.var_t5ng0_dn19 * locals.var_t5dg0) - (locals.var_t5ng0 * locals.var_t5dg0_dn19)) / (locals.var_t5dg0 * locals.var_t5dg0))) - (((locals.var_t5ng1_dn19 * locals.var_t5dg1) - (locals.var_t5ng1 * locals.var_t5dg1_dn19)) / (locals.var_t5dg1 * locals.var_t5dg1))), ((-(((locals.var_t5ng0_dn20 * locals.var_t5dg0) - (locals.var_t5ng0 * locals.var_t5dg0_dn20)) / (locals.var_t5dg0 * locals.var_t5dg0))) - (((locals.var_t5ng1_dn20 * locals.var_t5dg1) - (locals.var_t5ng1 * locals.var_t5dg1_dn20)) / (locals.var_t5dg1 * locals.var_t5dg1))), ((-(((locals.var_t5ng0_dn21 * locals.var_t5dg0) - (locals.var_t5ng0 * locals.var_t5dg0_dn21)) / (locals.var_t5dg0 * locals.var_t5dg0))) - (((locals.var_t5ng1_dn21 * locals.var_t5dg1) - (locals.var_t5ng1 * locals.var_t5dg1_dn21)) / (locals.var_t5dg1 * locals.var_t5dg1))), ((-(((locals.var_t5ng0_dn22 * locals.var_t5dg0) - (locals.var_t5ng0 * locals.var_t5dg0_dn22)) / (locals.var_t5dg0 * locals.var_t5dg0))) - (((locals.var_t5ng1_dn22 * locals.var_t5dg1) - (locals.var_t5ng1 * locals.var_t5dg1_dn22)) / (locals.var_t5dg1 * locals.var_t5dg1))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn1, locals.var_t5_dn2, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn12, locals.var_t5_dn14, locals.var_t5_dn15, locals.var_t5_dn16, locals.var_t5_dn17, locals.var_t5_dn18, locals.var_t5_dn19, locals.var_t5_dn20, locals.var_t5_dn21, locals.var_t5_dn22,)
    }
};
        locals.var_t5 = assign30490_e48159;
        locals.var_t5_dn0 = assign30490_e48159_d_n0;
        locals.var_t5_dn1 = assign30490_e48159_d_n1;
        locals.var_t5_dn2 = assign30490_e48159_d_n2;
        locals.var_t5_dn3 = assign30490_e48159_d_n3;
        locals.var_t5_dn4 = assign30490_e48159_d_n4;
        locals.var_t5_dn5 = assign30490_e48159_d_n5;
        locals.var_t5_dn6 = assign30490_e48159_d_n6;
        locals.var_t5_dn7 = assign30490_e48159_d_n7;
        locals.var_t5_dn8 = assign30490_e48159_d_n8;
        locals.var_t5_dn9 = assign30490_e48159_d_n9;
        locals.var_t5_dn12 = assign30490_e48159_d_n12;
        locals.var_t5_dn14 = assign30490_e48159_d_n14;
        locals.var_t5_dn15 = assign30490_e48159_d_n15;
        locals.var_t5_dn16 = assign30490_e48159_d_n16;
        locals.var_t5_dn17 = assign30490_e48159_d_n17;
        locals.var_t5_dn18 = assign30490_e48159_d_n18;
        locals.var_t5_dn19 = assign30490_e48159_d_n19;
        locals.var_t5_dn20 = assign30490_e48159_d_n20;
        locals.var_t5_dn21 = assign30490_e48159_d_n21;
        locals.var_t5_dn22 = assign30490_e48159_d_n22;

        let (assign30500_e48172, assign30500_e48172_d_n0, assign30500_e48172_d_n1, assign30500_e48172_d_n2, assign30500_e48172_d_n3, assign30500_e48172_d_n4, assign30500_e48172_d_n5, assign30500_e48172_d_n6, assign30500_e48172_d_n7, assign30500_e48172_d_n8, assign30500_e48172_d_n9, assign30500_e48172_d_n12, assign30500_e48172_d_n14, assign30500_e48172_d_n15, assign30500_e48172_d_n16, assign30500_e48172_d_n17, assign30500_e48172_d_n18, assign30500_e48172_d_n19, assign30500_e48172_d_n20, assign30500_e48172_d_n21, assign30500_e48172_d_n22,) = {
    if (((locals.var_guard504 == 0.0) && (locals.var_guard513 != 0.0)) && (locals.var_guard518 != 0.0)) {
        let assign30500_e48169: f64 = (locals.var_t4 / locals.var_t5);
        let assign30500_e48170: f64 = (locals.var_ef1 - assign30500_e48169);
        (assign30500_e48170, (locals.var_ef1_dn0 - (((locals.var_t4_dn0 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn0)) / (locals.var_t5 * locals.var_t5))), (locals.var_ef1_dn1 - (((locals.var_t4_dn1 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn1)) / (locals.var_t5 * locals.var_t5))), (locals.var_ef1_dn2 - (((locals.var_t4_dn2 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn2)) / (locals.var_t5 * locals.var_t5))), (locals.var_ef1_dn3 - (((locals.var_t4_dn3 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn3)) / (locals.var_t5 * locals.var_t5))), (locals.var_ef1_dn4 - (((locals.var_t4_dn4 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn4)) / (locals.var_t5 * locals.var_t5))), (locals.var_ef1_dn5 - (((locals.var_t4_dn5 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn5)) / (locals.var_t5 * locals.var_t5))), (locals.var_ef1_dn6 - (((locals.var_t4_dn6 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn6)) / (locals.var_t5 * locals.var_t5))), (locals.var_ef1_dn7 - (((locals.var_t4_dn7 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn7)) / (locals.var_t5 * locals.var_t5))), (locals.var_ef1_dn8 - (((locals.var_t4_dn8 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn8)) / (locals.var_t5 * locals.var_t5))), (locals.var_ef1_dn9 - (((locals.var_t4_dn9 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn9)) / (locals.var_t5 * locals.var_t5))), (locals.var_ef1_dn12 - (((locals.var_t4_dn12 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn12)) / (locals.var_t5 * locals.var_t5))), (locals.var_ef1_dn14 - (((locals.var_t4_dn14 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn14)) / (locals.var_t5 * locals.var_t5))), (locals.var_ef1_dn15 - (((locals.var_t4_dn15 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn15)) / (locals.var_t5 * locals.var_t5))), (locals.var_ef1_dn16 - (((locals.var_t4_dn16 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn16)) / (locals.var_t5 * locals.var_t5))), (locals.var_ef1_dn17 - (((locals.var_t4_dn17 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn17)) / (locals.var_t5 * locals.var_t5))), (locals.var_ef1_dn18 - (((locals.var_t4_dn18 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn18)) / (locals.var_t5 * locals.var_t5))), (locals.var_ef1_dn19 - (((locals.var_t4_dn19 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn19)) / (locals.var_t5 * locals.var_t5))), (locals.var_ef1_dn20 - (((locals.var_t4_dn20 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn20)) / (locals.var_t5 * locals.var_t5))), (locals.var_ef1_dn21 - (((locals.var_t4_dn21 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn21)) / (locals.var_t5 * locals.var_t5))), (locals.var_ef1_dn22 - (((locals.var_t4_dn22 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn22)) / (locals.var_t5 * locals.var_t5))),)
    } else {
        (locals.var_ef2, locals.var_ef2_dn0, locals.var_ef2_dn1, locals.var_ef2_dn2, locals.var_ef2_dn3, locals.var_ef2_dn4, locals.var_ef2_dn5, locals.var_ef2_dn6, locals.var_ef2_dn7, locals.var_ef2_dn8, locals.var_ef2_dn9, locals.var_ef2_dn12, locals.var_ef2_dn14, locals.var_ef2_dn15, locals.var_ef2_dn16, locals.var_ef2_dn17, locals.var_ef2_dn18, locals.var_ef2_dn19, locals.var_ef2_dn20, locals.var_ef2_dn21, locals.var_ef2_dn22,)
    }
};
        locals.var_ef2 = assign30500_e48172;
        locals.var_ef2_dn0 = assign30500_e48172_d_n0;
        locals.var_ef2_dn1 = assign30500_e48172_d_n1;
        locals.var_ef2_dn2 = assign30500_e48172_d_n2;
        locals.var_ef2_dn3 = assign30500_e48172_d_n3;
        locals.var_ef2_dn4 = assign30500_e48172_d_n4;
        locals.var_ef2_dn5 = assign30500_e48172_d_n5;
        locals.var_ef2_dn6 = assign30500_e48172_d_n6;
        locals.var_ef2_dn7 = assign30500_e48172_d_n7;
        locals.var_ef2_dn8 = assign30500_e48172_d_n8;
        locals.var_ef2_dn9 = assign30500_e48172_d_n9;
        locals.var_ef2_dn12 = assign30500_e48172_d_n12;
        locals.var_ef2_dn14 = assign30500_e48172_d_n14;
        locals.var_ef2_dn15 = assign30500_e48172_d_n15;
        locals.var_ef2_dn16 = assign30500_e48172_d_n16;
        locals.var_ef2_dn17 = assign30500_e48172_d_n17;
        locals.var_ef2_dn18 = assign30500_e48172_d_n18;
        locals.var_ef2_dn19 = assign30500_e48172_d_n19;
        locals.var_ef2_dn20 = assign30500_e48172_d_n20;
        locals.var_ef2_dn21 = assign30500_e48172_d_n21;
        locals.var_ef2_dn22 = assign30500_e48172_d_n22;

        let (assign30510_e48183, assign30510_e48183_d_n0, assign30510_e48183_d_n1, assign30510_e48183_d_n2, assign30510_e48183_d_n3, assign30510_e48183_d_n4, assign30510_e48183_d_n5, assign30510_e48183_d_n6, assign30510_e48183_d_n7, assign30510_e48183_d_n8, assign30510_e48183_d_n9, assign30510_e48183_d_n12, assign30510_e48183_d_n14, assign30510_e48183_d_n15, assign30510_e48183_d_n16, assign30510_e48183_d_n17, assign30510_e48183_d_n18, assign30510_e48183_d_n19, assign30510_e48183_d_n20, assign30510_e48183_d_n21, assign30510_e48183_d_n22,) = {
    if (((locals.var_guard504 == 0.0) && (locals.var_guard513 != 0.0)) && (locals.var_guard518 != 0.0)) {
        let assign30510_e48181: f64 = (locals.var_vgod - locals.var_ef2);
        (assign30510_e48181, (locals.var_vgod_dn0 - locals.var_ef2_dn0), (locals.var_vgod_dn1 - locals.var_ef2_dn1), (locals.var_vgod_dn2 - locals.var_ef2_dn2), (locals.var_vgod_dn3 - locals.var_ef2_dn3), (locals.var_vgod_dn4 - locals.var_ef2_dn4), (locals.var_vgod_dn5 - locals.var_ef2_dn5), (locals.var_vgod_dn6 - locals.var_ef2_dn6), (locals.var_vgod_dn7 - locals.var_ef2_dn7), (locals.var_vgod_dn8 - locals.var_ef2_dn8), (locals.var_vgod_dn9 - locals.var_ef2_dn9), (locals.var_vgod_dn12 - locals.var_ef2_dn12), (locals.var_vgod_dn14 - locals.var_ef2_dn14), (locals.var_vgod_dn15 - locals.var_ef2_dn15), (locals.var_vgod_dn16 - locals.var_ef2_dn16), (locals.var_vgod_dn17 - locals.var_ef2_dn17), (locals.var_vgod_dn18 - locals.var_ef2_dn18), (locals.var_vgod_dn19 - locals.var_ef2_dn19), (locals.var_vgod_dn20 - locals.var_ef2_dn20), (locals.var_vgod_dn21 - locals.var_ef2_dn21), (locals.var_vgod_dn22 - locals.var_ef2_dn22),)
    } else {
        (locals.var_vgef2, locals.var_vgef2_dn0, locals.var_vgef2_dn1, locals.var_vgef2_dn2, locals.var_vgef2_dn3, locals.var_vgef2_dn4, locals.var_vgef2_dn5, locals.var_vgef2_dn6, locals.var_vgef2_dn7, locals.var_vgef2_dn8, locals.var_vgef2_dn9, locals.var_vgef2_dn12, locals.var_vgef2_dn14, locals.var_vgef2_dn15, locals.var_vgef2_dn16, locals.var_vgef2_dn17, locals.var_vgef2_dn18, locals.var_vgef2_dn19, locals.var_vgef2_dn20, locals.var_vgef2_dn21, locals.var_vgef2_dn22,)
    }
};
        locals.var_vgef2 = assign30510_e48183;
        locals.var_vgef2_dn0 = assign30510_e48183_d_n0;
        locals.var_vgef2_dn1 = assign30510_e48183_d_n1;
        locals.var_vgef2_dn2 = assign30510_e48183_d_n2;
        locals.var_vgef2_dn3 = assign30510_e48183_d_n3;
        locals.var_vgef2_dn4 = assign30510_e48183_d_n4;
        locals.var_vgef2_dn5 = assign30510_e48183_d_n5;
        locals.var_vgef2_dn6 = assign30510_e48183_d_n6;
        locals.var_vgef2_dn7 = assign30510_e48183_d_n7;
        locals.var_vgef2_dn8 = assign30510_e48183_d_n8;
        locals.var_vgef2_dn9 = assign30510_e48183_d_n9;
        locals.var_vgef2_dn12 = assign30510_e48183_d_n12;
        locals.var_vgef2_dn14 = assign30510_e48183_d_n14;
        locals.var_vgef2_dn15 = assign30510_e48183_d_n15;
        locals.var_vgef2_dn16 = assign30510_e48183_d_n16;
        locals.var_vgef2_dn17 = assign30510_e48183_d_n17;
        locals.var_vgef2_dn18 = assign30510_e48183_d_n18;
        locals.var_vgef2_dn19 = assign30510_e48183_d_n19;
        locals.var_vgef2_dn20 = assign30510_e48183_d_n20;
        locals.var_vgef2_dn21 = assign30510_e48183_d_n21;
        locals.var_vgef2_dn22 = assign30510_e48183_d_n22;

        let (assign30520_e48207, assign30520_e48207_d_n0, assign30520_e48207_d_n1, assign30520_e48207_d_n2, assign30520_e48207_d_n3, assign30520_e48207_d_n4, assign30520_e48207_d_n5, assign30520_e48207_d_n6, assign30520_e48207_d_n7, assign30520_e48207_d_n8, assign30520_e48207_d_n9, assign30520_e48207_d_n12, assign30520_e48207_d_n14, assign30520_e48207_d_n15, assign30520_e48207_d_n16, assign30520_e48207_d_n17, assign30520_e48207_d_n18, assign30520_e48207_d_n19, assign30520_e48207_d_n20, assign30520_e48207_d_n21, assign30520_e48207_d_n22,) = {
    if (((locals.var_guard504 == 0.0) && (locals.var_guard513 != 0.0)) && (locals.var_guard518 != 0.0)) {
        let assign30520_e48192: f64 = (0.5 * locals.var_vgef2);
        let assign30520_e48196: f64 = (locals.var_vgef2 * locals.var_vgef2);
        let assign30520_e48199: f64 = (4.0 * 1e-9);
        let assign30520_e48201: f64 = (assign30520_e48199 * 1e-9);
        let assign30520_e48202: f64 = (assign30520_e48196 + assign30520_e48201);
        let assign30520_e48203: f64 = (assign30520_e48202).sqrt();
        let assign30520_e48204: f64 = (0.5 * assign30520_e48203);
        let assign30520_e48205: f64 = (assign30520_e48192 + assign30520_e48204);
        (assign30520_e48205, ((0.5 * locals.var_vgef2_dn0) + (0.5 * (((locals.var_vgef2_dn0 * locals.var_vgef2) + (locals.var_vgef2 * locals.var_vgef2_dn0)) / (2.0 * assign30520_e48203)))), ((0.5 * locals.var_vgef2_dn1) + (0.5 * (((locals.var_vgef2_dn1 * locals.var_vgef2) + (locals.var_vgef2 * locals.var_vgef2_dn1)) / (2.0 * assign30520_e48203)))), ((0.5 * locals.var_vgef2_dn2) + (0.5 * (((locals.var_vgef2_dn2 * locals.var_vgef2) + (locals.var_vgef2 * locals.var_vgef2_dn2)) / (2.0 * assign30520_e48203)))), ((0.5 * locals.var_vgef2_dn3) + (0.5 * (((locals.var_vgef2_dn3 * locals.var_vgef2) + (locals.var_vgef2 * locals.var_vgef2_dn3)) / (2.0 * assign30520_e48203)))), ((0.5 * locals.var_vgef2_dn4) + (0.5 * (((locals.var_vgef2_dn4 * locals.var_vgef2) + (locals.var_vgef2 * locals.var_vgef2_dn4)) / (2.0 * assign30520_e48203)))), ((0.5 * locals.var_vgef2_dn5) + (0.5 * (((locals.var_vgef2_dn5 * locals.var_vgef2) + (locals.var_vgef2 * locals.var_vgef2_dn5)) / (2.0 * assign30520_e48203)))), ((0.5 * locals.var_vgef2_dn6) + (0.5 * (((locals.var_vgef2_dn6 * locals.var_vgef2) + (locals.var_vgef2 * locals.var_vgef2_dn6)) / (2.0 * assign30520_e48203)))), ((0.5 * locals.var_vgef2_dn7) + (0.5 * (((locals.var_vgef2_dn7 * locals.var_vgef2) + (locals.var_vgef2 * locals.var_vgef2_dn7)) / (2.0 * assign30520_e48203)))), ((0.5 * locals.var_vgef2_dn8) + (0.5 * (((locals.var_vgef2_dn8 * locals.var_vgef2) + (locals.var_vgef2 * locals.var_vgef2_dn8)) / (2.0 * assign30520_e48203)))), ((0.5 * locals.var_vgef2_dn9) + (0.5 * (((locals.var_vgef2_dn9 * locals.var_vgef2) + (locals.var_vgef2 * locals.var_vgef2_dn9)) / (2.0 * assign30520_e48203)))), ((0.5 * locals.var_vgef2_dn12) + (0.5 * (((locals.var_vgef2_dn12 * locals.var_vgef2) + (locals.var_vgef2 * locals.var_vgef2_dn12)) / (2.0 * assign30520_e48203)))), ((0.5 * locals.var_vgef2_dn14) + (0.5 * (((locals.var_vgef2_dn14 * locals.var_vgef2) + (locals.var_vgef2 * locals.var_vgef2_dn14)) / (2.0 * assign30520_e48203)))), ((0.5 * locals.var_vgef2_dn15) + (0.5 * (((locals.var_vgef2_dn15 * locals.var_vgef2) + (locals.var_vgef2 * locals.var_vgef2_dn15)) / (2.0 * assign30520_e48203)))), ((0.5 * locals.var_vgef2_dn16) + (0.5 * (((locals.var_vgef2_dn16 * locals.var_vgef2) + (locals.var_vgef2 * locals.var_vgef2_dn16)) / (2.0 * assign30520_e48203)))), ((0.5 * locals.var_vgef2_dn17) + (0.5 * (((locals.var_vgef2_dn17 * locals.var_vgef2) + (locals.var_vgef2 * locals.var_vgef2_dn17)) / (2.0 * assign30520_e48203)))), ((0.5 * locals.var_vgef2_dn18) + (0.5 * (((locals.var_vgef2_dn18 * locals.var_vgef2) + (locals.var_vgef2 * locals.var_vgef2_dn18)) / (2.0 * assign30520_e48203)))), ((0.5 * locals.var_vgef2_dn19) + (0.5 * (((locals.var_vgef2_dn19 * locals.var_vgef2) + (locals.var_vgef2 * locals.var_vgef2_dn19)) / (2.0 * assign30520_e48203)))), ((0.5 * locals.var_vgef2_dn20) + (0.5 * (((locals.var_vgef2_dn20 * locals.var_vgef2) + (locals.var_vgef2 * locals.var_vgef2_dn20)) / (2.0 * assign30520_e48203)))), ((0.5 * locals.var_vgef2_dn21) + (0.5 * (((locals.var_vgef2_dn21 * locals.var_vgef2) + (locals.var_vgef2 * locals.var_vgef2_dn21)) / (2.0 * assign30520_e48203)))), ((0.5 * locals.var_vgef2_dn22) + (0.5 * (((locals.var_vgef2_dn22 * locals.var_vgef2) + (locals.var_vgef2 * locals.var_vgef2_dn22)) / (2.0 * assign30520_e48203)))),)
    } else {
        (locals.var_vgef2, locals.var_vgef2_dn0, locals.var_vgef2_dn1, locals.var_vgef2_dn2, locals.var_vgef2_dn3, locals.var_vgef2_dn4, locals.var_vgef2_dn5, locals.var_vgef2_dn6, locals.var_vgef2_dn7, locals.var_vgef2_dn8, locals.var_vgef2_dn9, locals.var_vgef2_dn12, locals.var_vgef2_dn14, locals.var_vgef2_dn15, locals.var_vgef2_dn16, locals.var_vgef2_dn17, locals.var_vgef2_dn18, locals.var_vgef2_dn19, locals.var_vgef2_dn20, locals.var_vgef2_dn21, locals.var_vgef2_dn22,)
    }
};
        locals.var_vgef2 = assign30520_e48207;
        locals.var_vgef2_dn0 = assign30520_e48207_d_n0;
        locals.var_vgef2_dn1 = assign30520_e48207_d_n1;
        locals.var_vgef2_dn2 = assign30520_e48207_d_n2;
        locals.var_vgef2_dn3 = assign30520_e48207_d_n3;
        locals.var_vgef2_dn4 = assign30520_e48207_d_n4;
        locals.var_vgef2_dn5 = assign30520_e48207_d_n5;
        locals.var_vgef2_dn6 = assign30520_e48207_d_n6;
        locals.var_vgef2_dn7 = assign30520_e48207_d_n7;
        locals.var_vgef2_dn8 = assign30520_e48207_d_n8;
        locals.var_vgef2_dn9 = assign30520_e48207_d_n9;
        locals.var_vgef2_dn12 = assign30520_e48207_d_n12;
        locals.var_vgef2_dn14 = assign30520_e48207_d_n14;
        locals.var_vgef2_dn15 = assign30520_e48207_d_n15;
        locals.var_vgef2_dn16 = assign30520_e48207_d_n16;
        locals.var_vgef2_dn17 = assign30520_e48207_d_n17;
        locals.var_vgef2_dn18 = assign30520_e48207_d_n18;
        locals.var_vgef2_dn19 = assign30520_e48207_d_n19;
        locals.var_vgef2_dn20 = assign30520_e48207_d_n20;
        locals.var_vgef2_dn21 = assign30520_e48207_d_n21;
        locals.var_vgef2_dn22 = assign30520_e48207_d_n22;

        let (assign30530_e48222, assign30530_e48222_d_n0, assign30530_e48222_d_n1, assign30530_e48222_d_n2, assign30530_e48222_d_n3, assign30530_e48222_d_n4, assign30530_e48222_d_n5, assign30530_e48222_d_n6, assign30530_e48222_d_n7, assign30530_e48222_d_n8, assign30530_e48222_d_n9, assign30530_e48222_d_n12, assign30530_e48222_d_n14, assign30530_e48222_d_n15, assign30530_e48222_d_n16, assign30530_e48222_d_n17, assign30530_e48222_d_n18, assign30530_e48222_d_n19, assign30530_e48222_d_n20, assign30530_e48222_d_n21, assign30530_e48222_d_n22,) = {
    if (((locals.var_guard504 == 0.0) && (locals.var_guard513 != 0.0)) && (locals.var_guard518 != 0.0)) {
        let assign30530_e48216: f64 = (p.p208 * locals.var_t0);
        let assign30530_e48219: f64 = (locals.var_vgef2).powf(0.6666666666666666);
        let assign30530_e48220: f64 = (assign30530_e48216 * assign30530_e48219);
        (assign30530_e48220, (((p.p208 * locals.var_t0_dn0) * assign30530_e48219) + (assign30530_e48216 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((locals.var_vgef2).powf(0.6666666666666666 - 1.0) * locals.var_vgef2_dn0)) } } else { (assign30530_e48219 * (0.6666666666666666 * (locals.var_vgef2_dn0 / locals.var_vgef2))) })), (((p.p208 * locals.var_t0_dn1) * assign30530_e48219) + (assign30530_e48216 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((locals.var_vgef2).powf(0.6666666666666666 - 1.0) * locals.var_vgef2_dn1)) } } else { (assign30530_e48219 * (0.6666666666666666 * (locals.var_vgef2_dn1 / locals.var_vgef2))) })), (((p.p208 * locals.var_t0_dn2) * assign30530_e48219) + (assign30530_e48216 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((locals.var_vgef2).powf(0.6666666666666666 - 1.0) * locals.var_vgef2_dn2)) } } else { (assign30530_e48219 * (0.6666666666666666 * (locals.var_vgef2_dn2 / locals.var_vgef2))) })), (((p.p208 * locals.var_t0_dn3) * assign30530_e48219) + (assign30530_e48216 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((locals.var_vgef2).powf(0.6666666666666666 - 1.0) * locals.var_vgef2_dn3)) } } else { (assign30530_e48219 * (0.6666666666666666 * (locals.var_vgef2_dn3 / locals.var_vgef2))) })), (((p.p208 * locals.var_t0_dn4) * assign30530_e48219) + (assign30530_e48216 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((locals.var_vgef2).powf(0.6666666666666666 - 1.0) * locals.var_vgef2_dn4)) } } else { (assign30530_e48219 * (0.6666666666666666 * (locals.var_vgef2_dn4 / locals.var_vgef2))) })), (((p.p208 * locals.var_t0_dn5) * assign30530_e48219) + (assign30530_e48216 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((locals.var_vgef2).powf(0.6666666666666666 - 1.0) * locals.var_vgef2_dn5)) } } else { (assign30530_e48219 * (0.6666666666666666 * (locals.var_vgef2_dn5 / locals.var_vgef2))) })), (((p.p208 * locals.var_t0_dn6) * assign30530_e48219) + (assign30530_e48216 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((locals.var_vgef2).powf(0.6666666666666666 - 1.0) * locals.var_vgef2_dn6)) } } else { (assign30530_e48219 * (0.6666666666666666 * (locals.var_vgef2_dn6 / locals.var_vgef2))) })), (((p.p208 * locals.var_t0_dn7) * assign30530_e48219) + (assign30530_e48216 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((locals.var_vgef2).powf(0.6666666666666666 - 1.0) * locals.var_vgef2_dn7)) } } else { (assign30530_e48219 * (0.6666666666666666 * (locals.var_vgef2_dn7 / locals.var_vgef2))) })), (((p.p208 * locals.var_t0_dn8) * assign30530_e48219) + (assign30530_e48216 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((locals.var_vgef2).powf(0.6666666666666666 - 1.0) * locals.var_vgef2_dn8)) } } else { (assign30530_e48219 * (0.6666666666666666 * (locals.var_vgef2_dn8 / locals.var_vgef2))) })), (((p.p208 * locals.var_t0_dn9) * assign30530_e48219) + (assign30530_e48216 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((locals.var_vgef2).powf(0.6666666666666666 - 1.0) * locals.var_vgef2_dn9)) } } else { (assign30530_e48219 * (0.6666666666666666 * (locals.var_vgef2_dn9 / locals.var_vgef2))) })), (((p.p208 * locals.var_t0_dn12) * assign30530_e48219) + (assign30530_e48216 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((locals.var_vgef2).powf(0.6666666666666666 - 1.0) * locals.var_vgef2_dn12)) } } else { (assign30530_e48219 * (0.6666666666666666 * (locals.var_vgef2_dn12 / locals.var_vgef2))) })), (((p.p208 * locals.var_t0_dn14) * assign30530_e48219) + (assign30530_e48216 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((locals.var_vgef2).powf(0.6666666666666666 - 1.0) * locals.var_vgef2_dn14)) } } else { (assign30530_e48219 * (0.6666666666666666 * (locals.var_vgef2_dn14 / locals.var_vgef2))) })), (((p.p208 * locals.var_t0_dn15) * assign30530_e48219) + (assign30530_e48216 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((locals.var_vgef2).powf(0.6666666666666666 - 1.0) * locals.var_vgef2_dn15)) } } else { (assign30530_e48219 * (0.6666666666666666 * (locals.var_vgef2_dn15 / locals.var_vgef2))) })), (((p.p208 * locals.var_t0_dn16) * assign30530_e48219) + (assign30530_e48216 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((locals.var_vgef2).powf(0.6666666666666666 - 1.0) * locals.var_vgef2_dn16)) } } else { (assign30530_e48219 * (0.6666666666666666 * (locals.var_vgef2_dn16 / locals.var_vgef2))) })), (((p.p208 * locals.var_t0_dn17) * assign30530_e48219) + (assign30530_e48216 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((locals.var_vgef2).powf(0.6666666666666666 - 1.0) * locals.var_vgef2_dn17)) } } else { (assign30530_e48219 * (0.6666666666666666 * (locals.var_vgef2_dn17 / locals.var_vgef2))) })), (((p.p208 * locals.var_t0_dn18) * assign30530_e48219) + (assign30530_e48216 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((locals.var_vgef2).powf(0.6666666666666666 - 1.0) * locals.var_vgef2_dn18)) } } else { (assign30530_e48219 * (0.6666666666666666 * (locals.var_vgef2_dn18 / locals.var_vgef2))) })), (((p.p208 * locals.var_t0_dn19) * assign30530_e48219) + (assign30530_e48216 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((locals.var_vgef2).powf(0.6666666666666666 - 1.0) * locals.var_vgef2_dn19)) } } else { (assign30530_e48219 * (0.6666666666666666 * (locals.var_vgef2_dn19 / locals.var_vgef2))) })), (((p.p208 * locals.var_t0_dn20) * assign30530_e48219) + (assign30530_e48216 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((locals.var_vgef2).powf(0.6666666666666666 - 1.0) * locals.var_vgef2_dn20)) } } else { (assign30530_e48219 * (0.6666666666666666 * (locals.var_vgef2_dn20 / locals.var_vgef2))) })), (((p.p208 * locals.var_t0_dn21) * assign30530_e48219) + (assign30530_e48216 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((locals.var_vgef2).powf(0.6666666666666666 - 1.0) * locals.var_vgef2_dn21)) } } else { (assign30530_e48219 * (0.6666666666666666 * (locals.var_vgef2_dn21 / locals.var_vgef2))) })), (((p.p208 * locals.var_t0_dn22) * assign30530_e48219) + (assign30530_e48216 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((locals.var_vgef2).powf(0.6666666666666666 - 1.0) * locals.var_vgef2_dn22)) } } else { (assign30530_e48219 * (0.6666666666666666 * (locals.var_vgef2_dn22 / locals.var_vgef2))) })),)
    } else {
        (locals.var_vgef223g0, locals.var_vgef223g0_dn0, locals.var_vgef223g0_dn1, locals.var_vgef223g0_dn2, locals.var_vgef223g0_dn3, locals.var_vgef223g0_dn4, locals.var_vgef223g0_dn5, locals.var_vgef223g0_dn6, locals.var_vgef223g0_dn7, locals.var_vgef223g0_dn8, locals.var_vgef223g0_dn9, locals.var_vgef223g0_dn12, locals.var_vgef223g0_dn14, locals.var_vgef223g0_dn15, locals.var_vgef223g0_dn16, locals.var_vgef223g0_dn17, locals.var_vgef223g0_dn18, locals.var_vgef223g0_dn19, locals.var_vgef223g0_dn20, locals.var_vgef223g0_dn21, locals.var_vgef223g0_dn22,)
    }
};
        locals.var_vgef223g0 = assign30530_e48222;
        locals.var_vgef223g0_dn0 = assign30530_e48222_d_n0;
        locals.var_vgef223g0_dn1 = assign30530_e48222_d_n1;
        locals.var_vgef223g0_dn2 = assign30530_e48222_d_n2;
        locals.var_vgef223g0_dn3 = assign30530_e48222_d_n3;
        locals.var_vgef223g0_dn4 = assign30530_e48222_d_n4;
        locals.var_vgef223g0_dn5 = assign30530_e48222_d_n5;
        locals.var_vgef223g0_dn6 = assign30530_e48222_d_n6;
        locals.var_vgef223g0_dn7 = assign30530_e48222_d_n7;
        locals.var_vgef223g0_dn8 = assign30530_e48222_d_n8;
        locals.var_vgef223g0_dn9 = assign30530_e48222_d_n9;
        locals.var_vgef223g0_dn12 = assign30530_e48222_d_n12;
        locals.var_vgef223g0_dn14 = assign30530_e48222_d_n14;
        locals.var_vgef223g0_dn15 = assign30530_e48222_d_n15;
        locals.var_vgef223g0_dn16 = assign30530_e48222_d_n16;
        locals.var_vgef223g0_dn17 = assign30530_e48222_d_n17;
        locals.var_vgef223g0_dn18 = assign30530_e48222_d_n18;
        locals.var_vgef223g0_dn19 = assign30530_e48222_d_n19;
        locals.var_vgef223g0_dn20 = assign30530_e48222_d_n20;
        locals.var_vgef223g0_dn21 = assign30530_e48222_d_n21;
        locals.var_vgef223g0_dn22 = assign30530_e48222_d_n22;

        let (assign30540_e48237, assign30540_e48237_d_n0, assign30540_e48237_d_n1, assign30540_e48237_d_n2, assign30540_e48237_d_n3, assign30540_e48237_d_n4, assign30540_e48237_d_n5, assign30540_e48237_d_n6, assign30540_e48237_d_n7, assign30540_e48237_d_n8, assign30540_e48237_d_n9, assign30540_e48237_d_n12, assign30540_e48237_d_n14, assign30540_e48237_d_n15, assign30540_e48237_d_n16, assign30540_e48237_d_n17, assign30540_e48237_d_n18, assign30540_e48237_d_n19, assign30540_e48237_d_n20, assign30540_e48237_d_n21, assign30540_e48237_d_n22,) = {
    if (((locals.var_guard504 == 0.0) && (locals.var_guard513 != 0.0)) && (locals.var_guard518 != 0.0)) {
        let assign30540_e48231: f64 = (p.p209 * locals.var_t0);
        let assign30540_e48234: f64 = (locals.var_vgef2).powf(0.6666666666666666);
        let assign30540_e48235: f64 = (assign30540_e48231 * assign30540_e48234);
        (assign30540_e48235, (((p.p209 * locals.var_t0_dn0) * assign30540_e48234) + (assign30540_e48231 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((locals.var_vgef2).powf(0.6666666666666666 - 1.0) * locals.var_vgef2_dn0)) } } else { (assign30540_e48234 * (0.6666666666666666 * (locals.var_vgef2_dn0 / locals.var_vgef2))) })), (((p.p209 * locals.var_t0_dn1) * assign30540_e48234) + (assign30540_e48231 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((locals.var_vgef2).powf(0.6666666666666666 - 1.0) * locals.var_vgef2_dn1)) } } else { (assign30540_e48234 * (0.6666666666666666 * (locals.var_vgef2_dn1 / locals.var_vgef2))) })), (((p.p209 * locals.var_t0_dn2) * assign30540_e48234) + (assign30540_e48231 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((locals.var_vgef2).powf(0.6666666666666666 - 1.0) * locals.var_vgef2_dn2)) } } else { (assign30540_e48234 * (0.6666666666666666 * (locals.var_vgef2_dn2 / locals.var_vgef2))) })), (((p.p209 * locals.var_t0_dn3) * assign30540_e48234) + (assign30540_e48231 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((locals.var_vgef2).powf(0.6666666666666666 - 1.0) * locals.var_vgef2_dn3)) } } else { (assign30540_e48234 * (0.6666666666666666 * (locals.var_vgef2_dn3 / locals.var_vgef2))) })), (((p.p209 * locals.var_t0_dn4) * assign30540_e48234) + (assign30540_e48231 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((locals.var_vgef2).powf(0.6666666666666666 - 1.0) * locals.var_vgef2_dn4)) } } else { (assign30540_e48234 * (0.6666666666666666 * (locals.var_vgef2_dn4 / locals.var_vgef2))) })), (((p.p209 * locals.var_t0_dn5) * assign30540_e48234) + (assign30540_e48231 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((locals.var_vgef2).powf(0.6666666666666666 - 1.0) * locals.var_vgef2_dn5)) } } else { (assign30540_e48234 * (0.6666666666666666 * (locals.var_vgef2_dn5 / locals.var_vgef2))) })), (((p.p209 * locals.var_t0_dn6) * assign30540_e48234) + (assign30540_e48231 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((locals.var_vgef2).powf(0.6666666666666666 - 1.0) * locals.var_vgef2_dn6)) } } else { (assign30540_e48234 * (0.6666666666666666 * (locals.var_vgef2_dn6 / locals.var_vgef2))) })), (((p.p209 * locals.var_t0_dn7) * assign30540_e48234) + (assign30540_e48231 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((locals.var_vgef2).powf(0.6666666666666666 - 1.0) * locals.var_vgef2_dn7)) } } else { (assign30540_e48234 * (0.6666666666666666 * (locals.var_vgef2_dn7 / locals.var_vgef2))) })), (((p.p209 * locals.var_t0_dn8) * assign30540_e48234) + (assign30540_e48231 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((locals.var_vgef2).powf(0.6666666666666666 - 1.0) * locals.var_vgef2_dn8)) } } else { (assign30540_e48234 * (0.6666666666666666 * (locals.var_vgef2_dn8 / locals.var_vgef2))) })), (((p.p209 * locals.var_t0_dn9) * assign30540_e48234) + (assign30540_e48231 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((locals.var_vgef2).powf(0.6666666666666666 - 1.0) * locals.var_vgef2_dn9)) } } else { (assign30540_e48234 * (0.6666666666666666 * (locals.var_vgef2_dn9 / locals.var_vgef2))) })), (((p.p209 * locals.var_t0_dn12) * assign30540_e48234) + (assign30540_e48231 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((locals.var_vgef2).powf(0.6666666666666666 - 1.0) * locals.var_vgef2_dn12)) } } else { (assign30540_e48234 * (0.6666666666666666 * (locals.var_vgef2_dn12 / locals.var_vgef2))) })), (((p.p209 * locals.var_t0_dn14) * assign30540_e48234) + (assign30540_e48231 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((locals.var_vgef2).powf(0.6666666666666666 - 1.0) * locals.var_vgef2_dn14)) } } else { (assign30540_e48234 * (0.6666666666666666 * (locals.var_vgef2_dn14 / locals.var_vgef2))) })), (((p.p209 * locals.var_t0_dn15) * assign30540_e48234) + (assign30540_e48231 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((locals.var_vgef2).powf(0.6666666666666666 - 1.0) * locals.var_vgef2_dn15)) } } else { (assign30540_e48234 * (0.6666666666666666 * (locals.var_vgef2_dn15 / locals.var_vgef2))) })), (((p.p209 * locals.var_t0_dn16) * assign30540_e48234) + (assign30540_e48231 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((locals.var_vgef2).powf(0.6666666666666666 - 1.0) * locals.var_vgef2_dn16)) } } else { (assign30540_e48234 * (0.6666666666666666 * (locals.var_vgef2_dn16 / locals.var_vgef2))) })), (((p.p209 * locals.var_t0_dn17) * assign30540_e48234) + (assign30540_e48231 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((locals.var_vgef2).powf(0.6666666666666666 - 1.0) * locals.var_vgef2_dn17)) } } else { (assign30540_e48234 * (0.6666666666666666 * (locals.var_vgef2_dn17 / locals.var_vgef2))) })), (((p.p209 * locals.var_t0_dn18) * assign30540_e48234) + (assign30540_e48231 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((locals.var_vgef2).powf(0.6666666666666666 - 1.0) * locals.var_vgef2_dn18)) } } else { (assign30540_e48234 * (0.6666666666666666 * (locals.var_vgef2_dn18 / locals.var_vgef2))) })), (((p.p209 * locals.var_t0_dn19) * assign30540_e48234) + (assign30540_e48231 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((locals.var_vgef2).powf(0.6666666666666666 - 1.0) * locals.var_vgef2_dn19)) } } else { (assign30540_e48234 * (0.6666666666666666 * (locals.var_vgef2_dn19 / locals.var_vgef2))) })), (((p.p209 * locals.var_t0_dn20) * assign30540_e48234) + (assign30540_e48231 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((locals.var_vgef2).powf(0.6666666666666666 - 1.0) * locals.var_vgef2_dn20)) } } else { (assign30540_e48234 * (0.6666666666666666 * (locals.var_vgef2_dn20 / locals.var_vgef2))) })), (((p.p209 * locals.var_t0_dn21) * assign30540_e48234) + (assign30540_e48231 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((locals.var_vgef2).powf(0.6666666666666666 - 1.0) * locals.var_vgef2_dn21)) } } else { (assign30540_e48234 * (0.6666666666666666 * (locals.var_vgef2_dn21 / locals.var_vgef2))) })), (((p.p209 * locals.var_t0_dn22) * assign30540_e48234) + (assign30540_e48231 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((locals.var_vgef2).powf(0.6666666666666666 - 1.0) * locals.var_vgef2_dn22)) } } else { (assign30540_e48234 * (0.6666666666666666 * (locals.var_vgef2_dn22 / locals.var_vgef2))) })),)
    } else {
        (locals.var_vgef223g1, locals.var_vgef223g1_dn0, locals.var_vgef223g1_dn1, locals.var_vgef223g1_dn2, locals.var_vgef223g1_dn3, locals.var_vgef223g1_dn4, locals.var_vgef223g1_dn5, locals.var_vgef223g1_dn6, locals.var_vgef223g1_dn7, locals.var_vgef223g1_dn8, locals.var_vgef223g1_dn9, locals.var_vgef223g1_dn12, locals.var_vgef223g1_dn14, locals.var_vgef223g1_dn15, locals.var_vgef223g1_dn16, locals.var_vgef223g1_dn17, locals.var_vgef223g1_dn18, locals.var_vgef223g1_dn19, locals.var_vgef223g1_dn20, locals.var_vgef223g1_dn21, locals.var_vgef223g1_dn22,)
    }
};
        locals.var_vgef223g1 = assign30540_e48237;
        locals.var_vgef223g1_dn0 = assign30540_e48237_d_n0;
        locals.var_vgef223g1_dn1 = assign30540_e48237_d_n1;
        locals.var_vgef223g1_dn2 = assign30540_e48237_d_n2;
        locals.var_vgef223g1_dn3 = assign30540_e48237_d_n3;
        locals.var_vgef223g1_dn4 = assign30540_e48237_d_n4;
        locals.var_vgef223g1_dn5 = assign30540_e48237_d_n5;
        locals.var_vgef223g1_dn6 = assign30540_e48237_d_n6;
        locals.var_vgef223g1_dn7 = assign30540_e48237_d_n7;
        locals.var_vgef223g1_dn8 = assign30540_e48237_d_n8;
        locals.var_vgef223g1_dn9 = assign30540_e48237_d_n9;
        locals.var_vgef223g1_dn12 = assign30540_e48237_d_n12;
        locals.var_vgef223g1_dn14 = assign30540_e48237_d_n14;
        locals.var_vgef223g1_dn15 = assign30540_e48237_d_n15;
        locals.var_vgef223g1_dn16 = assign30540_e48237_d_n16;
        locals.var_vgef223g1_dn17 = assign30540_e48237_d_n17;
        locals.var_vgef223g1_dn18 = assign30540_e48237_d_n18;
        locals.var_vgef223g1_dn19 = assign30540_e48237_d_n19;
        locals.var_vgef223g1_dn20 = assign30540_e48237_d_n20;
        locals.var_vgef223g1_dn21 = assign30540_e48237_d_n21;
        locals.var_vgef223g1_dn22 = assign30540_e48237_d_n22;

        let (assign30550_e48252, assign30550_e48252_d_n0, assign30550_e48252_d_n1, assign30550_e48252_d_n2, assign30550_e48252_d_n3, assign30550_e48252_d_n4, assign30550_e48252_d_n5, assign30550_e48252_d_n6, assign30550_e48252_d_n7, assign30550_e48252_d_n8, assign30550_e48252_d_n9, assign30550_e48252_d_n12, assign30550_e48252_d_n14, assign30550_e48252_d_n15, assign30550_e48252_d_n16, assign30550_e48252_d_n17, assign30550_e48252_d_n18, assign30550_e48252_d_n19, assign30550_e48252_d_n20, assign30550_e48252_d_n21, assign30550_e48252_d_n22,) = {
    if (((locals.var_guard504 == 0.0) && (locals.var_guard513 != 0.0)) && (locals.var_guard518 != 0.0)) {
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_vtv;
        let assign30550_e48246: f64 = (locals.var_ef2 * __rspice_inv_cse_0);
        let assign30550_e48249: f64 = (locals.var_vgef223g0 * __rspice_inv_cse_0);
        let assign30550_e48250: f64 = (assign30550_e48246 - assign30550_e48249);
        (assign30550_e48250, ((locals.var_ef2_dn0 / locals.var_vtv) - (locals.var_vgef223g0_dn0 / locals.var_vtv)), ((locals.var_ef2_dn1 / locals.var_vtv) - (locals.var_vgef223g0_dn1 / locals.var_vtv)), ((locals.var_ef2_dn2 / locals.var_vtv) - (locals.var_vgef223g0_dn2 / locals.var_vtv)), ((locals.var_ef2_dn3 / locals.var_vtv) - (locals.var_vgef223g0_dn3 / locals.var_vtv)), ((((locals.var_ef2_dn4 * locals.var_vtv) - (locals.var_ef2 * locals.var_vtv_dn4)) / (locals.var_vtv * locals.var_vtv)) - (((locals.var_vgef223g0_dn4 * locals.var_vtv) - (locals.var_vgef223g0 * locals.var_vtv_dn4)) / (locals.var_vtv * locals.var_vtv))), ((locals.var_ef2_dn5 / locals.var_vtv) - (locals.var_vgef223g0_dn5 / locals.var_vtv)), ((((locals.var_ef2_dn6 * locals.var_vtv) - (locals.var_ef2 * locals.var_vtv_dn6)) / (locals.var_vtv * locals.var_vtv)) - (((locals.var_vgef223g0_dn6 * locals.var_vtv) - (locals.var_vgef223g0 * locals.var_vtv_dn6)) / (locals.var_vtv * locals.var_vtv))), ((((locals.var_ef2_dn7 * locals.var_vtv) - (locals.var_ef2 * locals.var_vtv_dn7)) / (locals.var_vtv * locals.var_vtv)) - (((locals.var_vgef223g0_dn7 * locals.var_vtv) - (locals.var_vgef223g0 * locals.var_vtv_dn7)) / (locals.var_vtv * locals.var_vtv))), ((((locals.var_ef2_dn8 * locals.var_vtv) - (locals.var_ef2 * locals.var_vtv_dn8)) / (locals.var_vtv * locals.var_vtv)) - (((locals.var_vgef223g0_dn8 * locals.var_vtv) - (locals.var_vgef223g0 * locals.var_vtv_dn8)) / (locals.var_vtv * locals.var_vtv))), ((locals.var_ef2_dn9 / locals.var_vtv) - (locals.var_vgef223g0_dn9 / locals.var_vtv)), ((locals.var_ef2_dn12 / locals.var_vtv) - (locals.var_vgef223g0_dn12 / locals.var_vtv)), ((locals.var_ef2_dn14 / locals.var_vtv) - (locals.var_vgef223g0_dn14 / locals.var_vtv)), ((((locals.var_ef2_dn15 * locals.var_vtv) - (locals.var_ef2 * locals.var_vtv_dn15)) / (locals.var_vtv * locals.var_vtv)) - (((locals.var_vgef223g0_dn15 * locals.var_vtv) - (locals.var_vgef223g0 * locals.var_vtv_dn15)) / (locals.var_vtv * locals.var_vtv))), ((((locals.var_ef2_dn16 * locals.var_vtv) - (locals.var_ef2 * locals.var_vtv_dn16)) / (locals.var_vtv * locals.var_vtv)) - (((locals.var_vgef223g0_dn16 * locals.var_vtv) - (locals.var_vgef223g0 * locals.var_vtv_dn16)) / (locals.var_vtv * locals.var_vtv))), ((((locals.var_ef2_dn17 * locals.var_vtv) - (locals.var_ef2 * locals.var_vtv_dn17)) / (locals.var_vtv * locals.var_vtv)) - (((locals.var_vgef223g0_dn17 * locals.var_vtv) - (locals.var_vgef223g0 * locals.var_vtv_dn17)) / (locals.var_vtv * locals.var_vtv))), ((((locals.var_ef2_dn18 * locals.var_vtv) - (locals.var_ef2 * locals.var_vtv_dn18)) / (locals.var_vtv * locals.var_vtv)) - (((locals.var_vgef223g0_dn18 * locals.var_vtv) - (locals.var_vgef223g0 * locals.var_vtv_dn18)) / (locals.var_vtv * locals.var_vtv))), ((((locals.var_ef2_dn19 * locals.var_vtv) - (locals.var_ef2 * locals.var_vtv_dn19)) / (locals.var_vtv * locals.var_vtv)) - (((locals.var_vgef223g0_dn19 * locals.var_vtv) - (locals.var_vgef223g0 * locals.var_vtv_dn19)) / (locals.var_vtv * locals.var_vtv))), ((((locals.var_ef2_dn20 * locals.var_vtv) - (locals.var_ef2 * locals.var_vtv_dn20)) / (locals.var_vtv * locals.var_vtv)) - (((locals.var_vgef223g0_dn20 * locals.var_vtv) - (locals.var_vgef223g0 * locals.var_vtv_dn20)) / (locals.var_vtv * locals.var_vtv))), ((((locals.var_ef2_dn21 * locals.var_vtv) - (locals.var_ef2 * locals.var_vtv_dn21)) / (locals.var_vtv * locals.var_vtv)) - (((locals.var_vgef223g0_dn21 * locals.var_vtv) - (locals.var_vgef223g0 * locals.var_vtv_dn21)) / (locals.var_vtv * locals.var_vtv))), ((((locals.var_ef2_dn22 * locals.var_vtv) - (locals.var_ef2 * locals.var_vtv_dn22)) / (locals.var_vtv * locals.var_vtv)) - (((locals.var_vgef223g0_dn22 * locals.var_vtv) - (locals.var_vgef223g0 * locals.var_vtv_dn22)) / (locals.var_vtv * locals.var_vtv))),)
    } else {
        (locals.var_tg02, locals.var_tg02_dn0, locals.var_tg02_dn1, locals.var_tg02_dn2, locals.var_tg02_dn3, locals.var_tg02_dn4, locals.var_tg02_dn5, locals.var_tg02_dn6, locals.var_tg02_dn7, locals.var_tg02_dn8, locals.var_tg02_dn9, locals.var_tg02_dn12, locals.var_tg02_dn14, locals.var_tg02_dn15, locals.var_tg02_dn16, locals.var_tg02_dn17, locals.var_tg02_dn18, locals.var_tg02_dn19, locals.var_tg02_dn20, locals.var_tg02_dn21, locals.var_tg02_dn22,)
    }
};
        locals.var_tg02 = assign30550_e48252;
        locals.var_tg02_dn0 = assign30550_e48252_d_n0;
        locals.var_tg02_dn1 = assign30550_e48252_d_n1;
        locals.var_tg02_dn2 = assign30550_e48252_d_n2;
        locals.var_tg02_dn3 = assign30550_e48252_d_n3;
        locals.var_tg02_dn4 = assign30550_e48252_d_n4;
        locals.var_tg02_dn5 = assign30550_e48252_d_n5;
        locals.var_tg02_dn6 = assign30550_e48252_d_n6;
        locals.var_tg02_dn7 = assign30550_e48252_d_n7;
        locals.var_tg02_dn8 = assign30550_e48252_d_n8;
        locals.var_tg02_dn9 = assign30550_e48252_d_n9;
        locals.var_tg02_dn12 = assign30550_e48252_d_n12;
        locals.var_tg02_dn14 = assign30550_e48252_d_n14;
        locals.var_tg02_dn15 = assign30550_e48252_d_n15;
        locals.var_tg02_dn16 = assign30550_e48252_d_n16;
        locals.var_tg02_dn17 = assign30550_e48252_d_n17;
        locals.var_tg02_dn18 = assign30550_e48252_d_n18;
        locals.var_tg02_dn19 = assign30550_e48252_d_n19;
        locals.var_tg02_dn20 = assign30550_e48252_d_n20;
        locals.var_tg02_dn21 = assign30550_e48252_d_n21;
        locals.var_tg02_dn22 = assign30550_e48252_d_n22;

        let (assign30560_e48267, assign30560_e48267_d_n0, assign30560_e48267_d_n1, assign30560_e48267_d_n2, assign30560_e48267_d_n3, assign30560_e48267_d_n4, assign30560_e48267_d_n5, assign30560_e48267_d_n6, assign30560_e48267_d_n7, assign30560_e48267_d_n8, assign30560_e48267_d_n9, assign30560_e48267_d_n12, assign30560_e48267_d_n14, assign30560_e48267_d_n15, assign30560_e48267_d_n16, assign30560_e48267_d_n17, assign30560_e48267_d_n18, assign30560_e48267_d_n19, assign30560_e48267_d_n20, assign30560_e48267_d_n21, assign30560_e48267_d_n22,) = {
    if (((locals.var_guard504 == 0.0) && (locals.var_guard513 != 0.0)) && (locals.var_guard518 != 0.0)) {
        let __rspice_inv_cse_1: f64 = 1.0 / locals.var_vtv;
        let assign30560_e48261: f64 = (locals.var_ef2 * __rspice_inv_cse_1);
        let assign30560_e48264: f64 = (locals.var_vgef223g1 * __rspice_inv_cse_1);
        let assign30560_e48265: f64 = (assign30560_e48261 - assign30560_e48264);
        (assign30560_e48265, ((locals.var_ef2_dn0 / locals.var_vtv) - (locals.var_vgef223g1_dn0 / locals.var_vtv)), ((locals.var_ef2_dn1 / locals.var_vtv) - (locals.var_vgef223g1_dn1 / locals.var_vtv)), ((locals.var_ef2_dn2 / locals.var_vtv) - (locals.var_vgef223g1_dn2 / locals.var_vtv)), ((locals.var_ef2_dn3 / locals.var_vtv) - (locals.var_vgef223g1_dn3 / locals.var_vtv)), ((((locals.var_ef2_dn4 * locals.var_vtv) - (locals.var_ef2 * locals.var_vtv_dn4)) / (locals.var_vtv * locals.var_vtv)) - (((locals.var_vgef223g1_dn4 * locals.var_vtv) - (locals.var_vgef223g1 * locals.var_vtv_dn4)) / (locals.var_vtv * locals.var_vtv))), ((locals.var_ef2_dn5 / locals.var_vtv) - (locals.var_vgef223g1_dn5 / locals.var_vtv)), ((((locals.var_ef2_dn6 * locals.var_vtv) - (locals.var_ef2 * locals.var_vtv_dn6)) / (locals.var_vtv * locals.var_vtv)) - (((locals.var_vgef223g1_dn6 * locals.var_vtv) - (locals.var_vgef223g1 * locals.var_vtv_dn6)) / (locals.var_vtv * locals.var_vtv))), ((((locals.var_ef2_dn7 * locals.var_vtv) - (locals.var_ef2 * locals.var_vtv_dn7)) / (locals.var_vtv * locals.var_vtv)) - (((locals.var_vgef223g1_dn7 * locals.var_vtv) - (locals.var_vgef223g1 * locals.var_vtv_dn7)) / (locals.var_vtv * locals.var_vtv))), ((((locals.var_ef2_dn8 * locals.var_vtv) - (locals.var_ef2 * locals.var_vtv_dn8)) / (locals.var_vtv * locals.var_vtv)) - (((locals.var_vgef223g1_dn8 * locals.var_vtv) - (locals.var_vgef223g1 * locals.var_vtv_dn8)) / (locals.var_vtv * locals.var_vtv))), ((locals.var_ef2_dn9 / locals.var_vtv) - (locals.var_vgef223g1_dn9 / locals.var_vtv)), ((locals.var_ef2_dn12 / locals.var_vtv) - (locals.var_vgef223g1_dn12 / locals.var_vtv)), ((locals.var_ef2_dn14 / locals.var_vtv) - (locals.var_vgef223g1_dn14 / locals.var_vtv)), ((((locals.var_ef2_dn15 * locals.var_vtv) - (locals.var_ef2 * locals.var_vtv_dn15)) / (locals.var_vtv * locals.var_vtv)) - (((locals.var_vgef223g1_dn15 * locals.var_vtv) - (locals.var_vgef223g1 * locals.var_vtv_dn15)) / (locals.var_vtv * locals.var_vtv))), ((((locals.var_ef2_dn16 * locals.var_vtv) - (locals.var_ef2 * locals.var_vtv_dn16)) / (locals.var_vtv * locals.var_vtv)) - (((locals.var_vgef223g1_dn16 * locals.var_vtv) - (locals.var_vgef223g1 * locals.var_vtv_dn16)) / (locals.var_vtv * locals.var_vtv))), ((((locals.var_ef2_dn17 * locals.var_vtv) - (locals.var_ef2 * locals.var_vtv_dn17)) / (locals.var_vtv * locals.var_vtv)) - (((locals.var_vgef223g1_dn17 * locals.var_vtv) - (locals.var_vgef223g1 * locals.var_vtv_dn17)) / (locals.var_vtv * locals.var_vtv))), ((((locals.var_ef2_dn18 * locals.var_vtv) - (locals.var_ef2 * locals.var_vtv_dn18)) / (locals.var_vtv * locals.var_vtv)) - (((locals.var_vgef223g1_dn18 * locals.var_vtv) - (locals.var_vgef223g1 * locals.var_vtv_dn18)) / (locals.var_vtv * locals.var_vtv))), ((((locals.var_ef2_dn19 * locals.var_vtv) - (locals.var_ef2 * locals.var_vtv_dn19)) / (locals.var_vtv * locals.var_vtv)) - (((locals.var_vgef223g1_dn19 * locals.var_vtv) - (locals.var_vgef223g1 * locals.var_vtv_dn19)) / (locals.var_vtv * locals.var_vtv))), ((((locals.var_ef2_dn20 * locals.var_vtv) - (locals.var_ef2 * locals.var_vtv_dn20)) / (locals.var_vtv * locals.var_vtv)) - (((locals.var_vgef223g1_dn20 * locals.var_vtv) - (locals.var_vgef223g1 * locals.var_vtv_dn20)) / (locals.var_vtv * locals.var_vtv))), ((((locals.var_ef2_dn21 * locals.var_vtv) - (locals.var_ef2 * locals.var_vtv_dn21)) / (locals.var_vtv * locals.var_vtv)) - (((locals.var_vgef223g1_dn21 * locals.var_vtv) - (locals.var_vgef223g1 * locals.var_vtv_dn21)) / (locals.var_vtv * locals.var_vtv))), ((((locals.var_ef2_dn22 * locals.var_vtv) - (locals.var_ef2 * locals.var_vtv_dn22)) / (locals.var_vtv * locals.var_vtv)) - (((locals.var_vgef223g1_dn22 * locals.var_vtv) - (locals.var_vgef223g1 * locals.var_vtv_dn22)) / (locals.var_vtv * locals.var_vtv))),)
    } else {
        (locals.var_tg12, locals.var_tg12_dn0, locals.var_tg12_dn1, locals.var_tg12_dn2, locals.var_tg12_dn3, locals.var_tg12_dn4, locals.var_tg12_dn5, locals.var_tg12_dn6, locals.var_tg12_dn7, locals.var_tg12_dn8, locals.var_tg12_dn9, locals.var_tg12_dn12, locals.var_tg12_dn14, locals.var_tg12_dn15, locals.var_tg12_dn16, locals.var_tg12_dn17, locals.var_tg12_dn18, locals.var_tg12_dn19, locals.var_tg12_dn20, locals.var_tg12_dn21, locals.var_tg12_dn22,)
    }
};
        locals.var_tg12 = assign30560_e48267;
        locals.var_tg12_dn0 = assign30560_e48267_d_n0;
        locals.var_tg12_dn1 = assign30560_e48267_d_n1;
        locals.var_tg12_dn2 = assign30560_e48267_d_n2;
        locals.var_tg12_dn3 = assign30560_e48267_d_n3;
        locals.var_tg12_dn4 = assign30560_e48267_d_n4;
        locals.var_tg12_dn5 = assign30560_e48267_d_n5;
        locals.var_tg12_dn6 = assign30560_e48267_d_n6;
        locals.var_tg12_dn7 = assign30560_e48267_d_n7;
        locals.var_tg12_dn8 = assign30560_e48267_d_n8;
        locals.var_tg12_dn9 = assign30560_e48267_d_n9;
        locals.var_tg12_dn12 = assign30560_e48267_d_n12;
        locals.var_tg12_dn14 = assign30560_e48267_d_n14;
        locals.var_tg12_dn15 = assign30560_e48267_d_n15;
        locals.var_tg12_dn16 = assign30560_e48267_d_n16;
        locals.var_tg12_dn17 = assign30560_e48267_d_n17;
        locals.var_tg12_dn18 = assign30560_e48267_d_n18;
        locals.var_tg12_dn19 = assign30560_e48267_d_n19;
        locals.var_tg12_dn20 = assign30560_e48267_d_n20;
        locals.var_tg12_dn21 = assign30560_e48267_d_n21;
        locals.var_tg12_dn22 = assign30560_e48267_d_n22;

        let (assign30570_e48354, assign30570_e48354_d_n0, assign30570_e48354_d_n1, assign30570_e48354_d_n2, assign30570_e48354_d_n3, assign30570_e48354_d_n4, assign30570_e48354_d_n5, assign30570_e48354_d_n6, assign30570_e48354_d_n7, assign30570_e48354_d_n8, assign30570_e48354_d_n9, assign30570_e48354_d_n12, assign30570_e48354_d_n14, assign30570_e48354_d_n15, assign30570_e48354_d_n16, assign30570_e48354_d_n17, assign30570_e48354_d_n18, assign30570_e48354_d_n19, assign30570_e48354_d_n20, assign30570_e48354_d_n21, assign30570_e48354_d_n22,) = {
    if (((locals.var_guard504 == 0.0) && (locals.var_guard513 != 0.0)) && (locals.var_guard518 != 0.0)) {
        let assign30570_e48276: f64 = (locals.var_cch * locals.var_vgef2);
        let assign30570_e48279: f64 = (3.24e17 * locals.var_vtv);
        let assign30570_e48286: f64 = (-37.0);
        let (assign30570_e48312, assign30570_e48312_d_n0, assign30570_e48312_d_n1, assign30570_e48312_d_n2, assign30570_e48312_d_n3, assign30570_e48312_d_n4, assign30570_e48312_d_n5, assign30570_e48312_d_n6, assign30570_e48312_d_n7, assign30570_e48312_d_n8, assign30570_e48312_d_n9, assign30570_e48312_d_n12, assign30570_e48312_d_n14, assign30570_e48312_d_n15, assign30570_e48312_d_n16, assign30570_e48312_d_n17, assign30570_e48312_d_n18, assign30570_e48312_d_n19, assign30570_e48312_d_n20, assign30570_e48312_d_n21, assign30570_e48312_d_n22,) = {
            if ((!(locals.var_tg02 >= 37.0)) && (!(locals.var_tg02 <= assign30570_e48286))) {
                let assign30570_e48291: f64 = (locals.var_tg02).exp();
                let assign30570_e48293: f64 = (assign30570_e48291 + 1.0);
                let assign30570_e48294: f64 = (assign30570_e48293).ln();
                (assign30570_e48294, ((assign30570_e48291 * locals.var_tg02_dn0) / assign30570_e48293), ((assign30570_e48291 * locals.var_tg02_dn1) / assign30570_e48293), ((assign30570_e48291 * locals.var_tg02_dn2) / assign30570_e48293), ((assign30570_e48291 * locals.var_tg02_dn3) / assign30570_e48293), ((assign30570_e48291 * locals.var_tg02_dn4) / assign30570_e48293), ((assign30570_e48291 * locals.var_tg02_dn5) / assign30570_e48293), ((assign30570_e48291 * locals.var_tg02_dn6) / assign30570_e48293), ((assign30570_e48291 * locals.var_tg02_dn7) / assign30570_e48293), ((assign30570_e48291 * locals.var_tg02_dn8) / assign30570_e48293), ((assign30570_e48291 * locals.var_tg02_dn9) / assign30570_e48293), ((assign30570_e48291 * locals.var_tg02_dn12) / assign30570_e48293), ((assign30570_e48291 * locals.var_tg02_dn14) / assign30570_e48293), ((assign30570_e48291 * locals.var_tg02_dn15) / assign30570_e48293), ((assign30570_e48291 * locals.var_tg02_dn16) / assign30570_e48293), ((assign30570_e48291 * locals.var_tg02_dn17) / assign30570_e48293), ((assign30570_e48291 * locals.var_tg02_dn18) / assign30570_e48293), ((assign30570_e48291 * locals.var_tg02_dn19) / assign30570_e48293), ((assign30570_e48291 * locals.var_tg02_dn20) / assign30570_e48293), ((assign30570_e48291 * locals.var_tg02_dn21) / assign30570_e48293), ((assign30570_e48291 * locals.var_tg02_dn22) / assign30570_e48293),)
            } else {
                let assign30570_e48301: f64 = (-37.0);
                let (assign30570_e48311, assign30570_e48311_d_n0, assign30570_e48311_d_n1, assign30570_e48311_d_n2, assign30570_e48311_d_n3, assign30570_e48311_d_n4, assign30570_e48311_d_n5, assign30570_e48311_d_n6, assign30570_e48311_d_n7, assign30570_e48311_d_n8, assign30570_e48311_d_n9, assign30570_e48311_d_n12, assign30570_e48311_d_n14, assign30570_e48311_d_n15, assign30570_e48311_d_n16, assign30570_e48311_d_n17, assign30570_e48311_d_n18, assign30570_e48311_d_n19, assign30570_e48311_d_n20, assign30570_e48311_d_n21, assign30570_e48311_d_n22,) = {
                    if ((!(locals.var_tg02 >= 37.0)) && (locals.var_tg02 <= assign30570_e48301)) {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    } else {
                        let (assign30570_e48310, assign30570_e48310_d_n0, assign30570_e48310_d_n1, assign30570_e48310_d_n2, assign30570_e48310_d_n3, assign30570_e48310_d_n4, assign30570_e48310_d_n5, assign30570_e48310_d_n6, assign30570_e48310_d_n7, assign30570_e48310_d_n8, assign30570_e48310_d_n9, assign30570_e48310_d_n12, assign30570_e48310_d_n14, assign30570_e48310_d_n15, assign30570_e48310_d_n16, assign30570_e48310_d_n17, assign30570_e48310_d_n18, assign30570_e48310_d_n19, assign30570_e48310_d_n20, assign30570_e48310_d_n21, assign30570_e48310_d_n22,) = {
                            if (locals.var_tg02 >= 37.0) {
                                (locals.var_tg02, locals.var_tg02_dn0, locals.var_tg02_dn1, locals.var_tg02_dn2, locals.var_tg02_dn3, locals.var_tg02_dn4, locals.var_tg02_dn5, locals.var_tg02_dn6, locals.var_tg02_dn7, locals.var_tg02_dn8, locals.var_tg02_dn9, locals.var_tg02_dn12, locals.var_tg02_dn14, locals.var_tg02_dn15, locals.var_tg02_dn16, locals.var_tg02_dn17, locals.var_tg02_dn18, locals.var_tg02_dn19, locals.var_tg02_dn20, locals.var_tg02_dn21, locals.var_tg02_dn22,)
                            } else {
                                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign30570_e48310, assign30570_e48310_d_n0, assign30570_e48310_d_n1, assign30570_e48310_d_n2, assign30570_e48310_d_n3, assign30570_e48310_d_n4, assign30570_e48310_d_n5, assign30570_e48310_d_n6, assign30570_e48310_d_n7, assign30570_e48310_d_n8, assign30570_e48310_d_n9, assign30570_e48310_d_n12, assign30570_e48310_d_n14, assign30570_e48310_d_n15, assign30570_e48310_d_n16, assign30570_e48310_d_n17, assign30570_e48310_d_n18, assign30570_e48310_d_n19, assign30570_e48310_d_n20, assign30570_e48310_d_n21, assign30570_e48310_d_n22,)
                    }
                };
                (assign30570_e48311, assign30570_e48311_d_n0, assign30570_e48311_d_n1, assign30570_e48311_d_n2, assign30570_e48311_d_n3, assign30570_e48311_d_n4, assign30570_e48311_d_n5, assign30570_e48311_d_n6, assign30570_e48311_d_n7, assign30570_e48311_d_n8, assign30570_e48311_d_n9, assign30570_e48311_d_n12, assign30570_e48311_d_n14, assign30570_e48311_d_n15, assign30570_e48311_d_n16, assign30570_e48311_d_n17, assign30570_e48311_d_n18, assign30570_e48311_d_n19, assign30570_e48311_d_n20, assign30570_e48311_d_n21, assign30570_e48311_d_n22,)
            }
        };
        let assign30570_e48313: f64 = (assign30570_e48279 * assign30570_e48312);
        let assign30570_e48314: f64 = (assign30570_e48276 - assign30570_e48313);
        let assign30570_e48317: f64 = (3.24e17 * locals.var_vtv);
        let assign30570_e48324: f64 = (-37.0);
        let (assign30570_e48350, assign30570_e48350_d_n0, assign30570_e48350_d_n1, assign30570_e48350_d_n2, assign30570_e48350_d_n3, assign30570_e48350_d_n4, assign30570_e48350_d_n5, assign30570_e48350_d_n6, assign30570_e48350_d_n7, assign30570_e48350_d_n8, assign30570_e48350_d_n9, assign30570_e48350_d_n12, assign30570_e48350_d_n14, assign30570_e48350_d_n15, assign30570_e48350_d_n16, assign30570_e48350_d_n17, assign30570_e48350_d_n18, assign30570_e48350_d_n19, assign30570_e48350_d_n20, assign30570_e48350_d_n21, assign30570_e48350_d_n22,) = {
            if ((!(locals.var_tg12 >= 37.0)) && (!(locals.var_tg12 <= assign30570_e48324))) {
                let assign30570_e48329: f64 = (locals.var_tg12).exp();
                let assign30570_e48331: f64 = (assign30570_e48329 + 1.0);
                let assign30570_e48332: f64 = (assign30570_e48331).ln();
                (assign30570_e48332, ((assign30570_e48329 * locals.var_tg12_dn0) / assign30570_e48331), ((assign30570_e48329 * locals.var_tg12_dn1) / assign30570_e48331), ((assign30570_e48329 * locals.var_tg12_dn2) / assign30570_e48331), ((assign30570_e48329 * locals.var_tg12_dn3) / assign30570_e48331), ((assign30570_e48329 * locals.var_tg12_dn4) / assign30570_e48331), ((assign30570_e48329 * locals.var_tg12_dn5) / assign30570_e48331), ((assign30570_e48329 * locals.var_tg12_dn6) / assign30570_e48331), ((assign30570_e48329 * locals.var_tg12_dn7) / assign30570_e48331), ((assign30570_e48329 * locals.var_tg12_dn8) / assign30570_e48331), ((assign30570_e48329 * locals.var_tg12_dn9) / assign30570_e48331), ((assign30570_e48329 * locals.var_tg12_dn12) / assign30570_e48331), ((assign30570_e48329 * locals.var_tg12_dn14) / assign30570_e48331), ((assign30570_e48329 * locals.var_tg12_dn15) / assign30570_e48331), ((assign30570_e48329 * locals.var_tg12_dn16) / assign30570_e48331), ((assign30570_e48329 * locals.var_tg12_dn17) / assign30570_e48331), ((assign30570_e48329 * locals.var_tg12_dn18) / assign30570_e48331), ((assign30570_e48329 * locals.var_tg12_dn19) / assign30570_e48331), ((assign30570_e48329 * locals.var_tg12_dn20) / assign30570_e48331), ((assign30570_e48329 * locals.var_tg12_dn21) / assign30570_e48331), ((assign30570_e48329 * locals.var_tg12_dn22) / assign30570_e48331),)
            } else {
                let assign30570_e48339: f64 = (-37.0);
                let (assign30570_e48349, assign30570_e48349_d_n0, assign30570_e48349_d_n1, assign30570_e48349_d_n2, assign30570_e48349_d_n3, assign30570_e48349_d_n4, assign30570_e48349_d_n5, assign30570_e48349_d_n6, assign30570_e48349_d_n7, assign30570_e48349_d_n8, assign30570_e48349_d_n9, assign30570_e48349_d_n12, assign30570_e48349_d_n14, assign30570_e48349_d_n15, assign30570_e48349_d_n16, assign30570_e48349_d_n17, assign30570_e48349_d_n18, assign30570_e48349_d_n19, assign30570_e48349_d_n20, assign30570_e48349_d_n21, assign30570_e48349_d_n22,) = {
                    if ((!(locals.var_tg12 >= 37.0)) && (locals.var_tg12 <= assign30570_e48339)) {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    } else {
                        let (assign30570_e48348, assign30570_e48348_d_n0, assign30570_e48348_d_n1, assign30570_e48348_d_n2, assign30570_e48348_d_n3, assign30570_e48348_d_n4, assign30570_e48348_d_n5, assign30570_e48348_d_n6, assign30570_e48348_d_n7, assign30570_e48348_d_n8, assign30570_e48348_d_n9, assign30570_e48348_d_n12, assign30570_e48348_d_n14, assign30570_e48348_d_n15, assign30570_e48348_d_n16, assign30570_e48348_d_n17, assign30570_e48348_d_n18, assign30570_e48348_d_n19, assign30570_e48348_d_n20, assign30570_e48348_d_n21, assign30570_e48348_d_n22,) = {
                            if (locals.var_tg12 >= 37.0) {
                                (locals.var_tg12, locals.var_tg12_dn0, locals.var_tg12_dn1, locals.var_tg12_dn2, locals.var_tg12_dn3, locals.var_tg12_dn4, locals.var_tg12_dn5, locals.var_tg12_dn6, locals.var_tg12_dn7, locals.var_tg12_dn8, locals.var_tg12_dn9, locals.var_tg12_dn12, locals.var_tg12_dn14, locals.var_tg12_dn15, locals.var_tg12_dn16, locals.var_tg12_dn17, locals.var_tg12_dn18, locals.var_tg12_dn19, locals.var_tg12_dn20, locals.var_tg12_dn21, locals.var_tg12_dn22,)
                            } else {
                                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign30570_e48348, assign30570_e48348_d_n0, assign30570_e48348_d_n1, assign30570_e48348_d_n2, assign30570_e48348_d_n3, assign30570_e48348_d_n4, assign30570_e48348_d_n5, assign30570_e48348_d_n6, assign30570_e48348_d_n7, assign30570_e48348_d_n8, assign30570_e48348_d_n9, assign30570_e48348_d_n12, assign30570_e48348_d_n14, assign30570_e48348_d_n15, assign30570_e48348_d_n16, assign30570_e48348_d_n17, assign30570_e48348_d_n18, assign30570_e48348_d_n19, assign30570_e48348_d_n20, assign30570_e48348_d_n21, assign30570_e48348_d_n22,)
                    }
                };
                (assign30570_e48349, assign30570_e48349_d_n0, assign30570_e48349_d_n1, assign30570_e48349_d_n2, assign30570_e48349_d_n3, assign30570_e48349_d_n4, assign30570_e48349_d_n5, assign30570_e48349_d_n6, assign30570_e48349_d_n7, assign30570_e48349_d_n8, assign30570_e48349_d_n9, assign30570_e48349_d_n12, assign30570_e48349_d_n14, assign30570_e48349_d_n15, assign30570_e48349_d_n16, assign30570_e48349_d_n17, assign30570_e48349_d_n18, assign30570_e48349_d_n19, assign30570_e48349_d_n20, assign30570_e48349_d_n21, assign30570_e48349_d_n22,)
            }
        };
        let assign30570_e48351: f64 = (assign30570_e48317 * assign30570_e48350);
        let assign30570_e48352: f64 = (assign30570_e48314 - assign30570_e48351);
        (assign30570_e48352, (((locals.var_cch * locals.var_vgef2_dn0) - (assign30570_e48279 * assign30570_e48312_d_n0)) - (assign30570_e48317 * assign30570_e48350_d_n0)), (((locals.var_cch * locals.var_vgef2_dn1) - (assign30570_e48279 * assign30570_e48312_d_n1)) - (assign30570_e48317 * assign30570_e48350_d_n1)), (((locals.var_cch * locals.var_vgef2_dn2) - (assign30570_e48279 * assign30570_e48312_d_n2)) - (assign30570_e48317 * assign30570_e48350_d_n2)), (((locals.var_cch * locals.var_vgef2_dn3) - (assign30570_e48279 * assign30570_e48312_d_n3)) - (assign30570_e48317 * assign30570_e48350_d_n3)), (((locals.var_cch * locals.var_vgef2_dn4) - (((3.24e17 * locals.var_vtv_dn4) * assign30570_e48312) + (assign30570_e48279 * assign30570_e48312_d_n4))) - (((3.24e17 * locals.var_vtv_dn4) * assign30570_e48350) + (assign30570_e48317 * assign30570_e48350_d_n4))), (((locals.var_cch * locals.var_vgef2_dn5) - (assign30570_e48279 * assign30570_e48312_d_n5)) - (assign30570_e48317 * assign30570_e48350_d_n5)), (((locals.var_cch * locals.var_vgef2_dn6) - (((3.24e17 * locals.var_vtv_dn6) * assign30570_e48312) + (assign30570_e48279 * assign30570_e48312_d_n6))) - (((3.24e17 * locals.var_vtv_dn6) * assign30570_e48350) + (assign30570_e48317 * assign30570_e48350_d_n6))), (((locals.var_cch * locals.var_vgef2_dn7) - (((3.24e17 * locals.var_vtv_dn7) * assign30570_e48312) + (assign30570_e48279 * assign30570_e48312_d_n7))) - (((3.24e17 * locals.var_vtv_dn7) * assign30570_e48350) + (assign30570_e48317 * assign30570_e48350_d_n7))), (((locals.var_cch * locals.var_vgef2_dn8) - (((3.24e17 * locals.var_vtv_dn8) * assign30570_e48312) + (assign30570_e48279 * assign30570_e48312_d_n8))) - (((3.24e17 * locals.var_vtv_dn8) * assign30570_e48350) + (assign30570_e48317 * assign30570_e48350_d_n8))), (((locals.var_cch * locals.var_vgef2_dn9) - (assign30570_e48279 * assign30570_e48312_d_n9)) - (assign30570_e48317 * assign30570_e48350_d_n9)), (((locals.var_cch * locals.var_vgef2_dn12) - (assign30570_e48279 * assign30570_e48312_d_n12)) - (assign30570_e48317 * assign30570_e48350_d_n12)), (((locals.var_cch * locals.var_vgef2_dn14) - (assign30570_e48279 * assign30570_e48312_d_n14)) - (assign30570_e48317 * assign30570_e48350_d_n14)), (((locals.var_cch * locals.var_vgef2_dn15) - (((3.24e17 * locals.var_vtv_dn15) * assign30570_e48312) + (assign30570_e48279 * assign30570_e48312_d_n15))) - (((3.24e17 * locals.var_vtv_dn15) * assign30570_e48350) + (assign30570_e48317 * assign30570_e48350_d_n15))), (((locals.var_cch * locals.var_vgef2_dn16) - (((3.24e17 * locals.var_vtv_dn16) * assign30570_e48312) + (assign30570_e48279 * assign30570_e48312_d_n16))) - (((3.24e17 * locals.var_vtv_dn16) * assign30570_e48350) + (assign30570_e48317 * assign30570_e48350_d_n16))), (((locals.var_cch * locals.var_vgef2_dn17) - (((3.24e17 * locals.var_vtv_dn17) * assign30570_e48312) + (assign30570_e48279 * assign30570_e48312_d_n17))) - (((3.24e17 * locals.var_vtv_dn17) * assign30570_e48350) + (assign30570_e48317 * assign30570_e48350_d_n17))), (((locals.var_cch * locals.var_vgef2_dn18) - (((3.24e17 * locals.var_vtv_dn18) * assign30570_e48312) + (assign30570_e48279 * assign30570_e48312_d_n18))) - (((3.24e17 * locals.var_vtv_dn18) * assign30570_e48350) + (assign30570_e48317 * assign30570_e48350_d_n18))), (((locals.var_cch * locals.var_vgef2_dn19) - (((3.24e17 * locals.var_vtv_dn19) * assign30570_e48312) + (assign30570_e48279 * assign30570_e48312_d_n19))) - (((3.24e17 * locals.var_vtv_dn19) * assign30570_e48350) + (assign30570_e48317 * assign30570_e48350_d_n19))), (((locals.var_cch * locals.var_vgef2_dn20) - (((3.24e17 * locals.var_vtv_dn20) * assign30570_e48312) + (assign30570_e48279 * assign30570_e48312_d_n20))) - (((3.24e17 * locals.var_vtv_dn20) * assign30570_e48350) + (assign30570_e48317 * assign30570_e48350_d_n20))), (((locals.var_cch * locals.var_vgef2_dn21) - (((3.24e17 * locals.var_vtv_dn21) * assign30570_e48312) + (assign30570_e48279 * assign30570_e48312_d_n21))) - (((3.24e17 * locals.var_vtv_dn21) * assign30570_e48350) + (assign30570_e48317 * assign30570_e48350_d_n21))), (((locals.var_cch * locals.var_vgef2_dn22) - (((3.24e17 * locals.var_vtv_dn22) * assign30570_e48312) + (assign30570_e48279 * assign30570_e48312_d_n22))) - (((3.24e17 * locals.var_vtv_dn22) * assign30570_e48350) + (assign30570_e48317 * assign30570_e48350_d_n22))),)
    } else {
        (locals.var_t42, locals.var_t42_dn0, locals.var_t42_dn1, locals.var_t42_dn2, locals.var_t42_dn3, locals.var_t42_dn4, locals.var_t42_dn5, locals.var_t42_dn6, locals.var_t42_dn7, locals.var_t42_dn8, locals.var_t42_dn9, locals.var_t42_dn12, locals.var_t42_dn14, locals.var_t42_dn15, locals.var_t42_dn16, locals.var_t42_dn17, locals.var_t42_dn18, locals.var_t42_dn19, locals.var_t42_dn20, locals.var_t42_dn21, locals.var_t42_dn22,)
    }
};
        locals.var_t42 = assign30570_e48354;
        locals.var_t42_dn0 = assign30570_e48354_d_n0;
        locals.var_t42_dn1 = assign30570_e48354_d_n1;
        locals.var_t42_dn2 = assign30570_e48354_d_n2;
        locals.var_t42_dn3 = assign30570_e48354_d_n3;
        locals.var_t42_dn4 = assign30570_e48354_d_n4;
        locals.var_t42_dn5 = assign30570_e48354_d_n5;
        locals.var_t42_dn6 = assign30570_e48354_d_n6;
        locals.var_t42_dn7 = assign30570_e48354_d_n7;
        locals.var_t42_dn8 = assign30570_e48354_d_n8;
        locals.var_t42_dn9 = assign30570_e48354_d_n9;
        locals.var_t42_dn12 = assign30570_e48354_d_n12;
        locals.var_t42_dn14 = assign30570_e48354_d_n14;
        locals.var_t42_dn15 = assign30570_e48354_d_n15;
        locals.var_t42_dn16 = assign30570_e48354_d_n16;
        locals.var_t42_dn17 = assign30570_e48354_d_n17;
        locals.var_t42_dn18 = assign30570_e48354_d_n18;
        locals.var_t42_dn19 = assign30570_e48354_d_n19;
        locals.var_t42_dn20 = assign30570_e48354_d_n20;
        locals.var_t42_dn21 = assign30570_e48354_d_n21;
        locals.var_t42_dn22 = assign30570_e48354_d_n22;

        let (assign30580_e48370, assign30580_e48370_d_n0, assign30580_e48370_d_n1, assign30580_e48370_d_n2, assign30580_e48370_d_n3, assign30580_e48370_d_n4, assign30580_e48370_d_n5, assign30580_e48370_d_n6, assign30580_e48370_d_n7, assign30580_e48370_d_n8, assign30580_e48370_d_n9, assign30580_e48370_d_n12, assign30580_e48370_d_n14, assign30580_e48370_d_n15, assign30580_e48370_d_n16, assign30580_e48370_d_n17, assign30580_e48370_d_n18, assign30580_e48370_d_n19, assign30580_e48370_d_n20, assign30580_e48370_d_n21, assign30580_e48370_d_n22,) = {
    if (((locals.var_guard504 == 0.0) && (locals.var_guard513 != 0.0)) && (locals.var_guard518 != 0.0)) {
        let assign30580_e48363: f64 = (p.p208 * locals.var_t0);
        let assign30580_e48366: f64 = (-0.3333333333333333);
        let assign30580_e48367: f64 = (locals.var_vgef2).powf(assign30580_e48366);
        let assign30580_e48368: f64 = (assign30580_e48363 * assign30580_e48367);
        (assign30580_e48368, (((p.p208 * locals.var_t0_dn0) * assign30580_e48367) + (assign30580_e48363 * if 0.0 == 0.0 && ((assign30580_e48366) as f64).is_finite() && ((assign30580_e48366) as f64).fract() == 0.0 { if assign30580_e48366 == 0.0 { 0.0 } else { (assign30580_e48366 * ((locals.var_vgef2).powf(assign30580_e48366 - 1.0) * locals.var_vgef2_dn0)) } } else { (assign30580_e48367 * (assign30580_e48366 * (locals.var_vgef2_dn0 / locals.var_vgef2))) })), (((p.p208 * locals.var_t0_dn1) * assign30580_e48367) + (assign30580_e48363 * if 0.0 == 0.0 && ((assign30580_e48366) as f64).is_finite() && ((assign30580_e48366) as f64).fract() == 0.0 { if assign30580_e48366 == 0.0 { 0.0 } else { (assign30580_e48366 * ((locals.var_vgef2).powf(assign30580_e48366 - 1.0) * locals.var_vgef2_dn1)) } } else { (assign30580_e48367 * (assign30580_e48366 * (locals.var_vgef2_dn1 / locals.var_vgef2))) })), (((p.p208 * locals.var_t0_dn2) * assign30580_e48367) + (assign30580_e48363 * if 0.0 == 0.0 && ((assign30580_e48366) as f64).is_finite() && ((assign30580_e48366) as f64).fract() == 0.0 { if assign30580_e48366 == 0.0 { 0.0 } else { (assign30580_e48366 * ((locals.var_vgef2).powf(assign30580_e48366 - 1.0) * locals.var_vgef2_dn2)) } } else { (assign30580_e48367 * (assign30580_e48366 * (locals.var_vgef2_dn2 / locals.var_vgef2))) })), (((p.p208 * locals.var_t0_dn3) * assign30580_e48367) + (assign30580_e48363 * if 0.0 == 0.0 && ((assign30580_e48366) as f64).is_finite() && ((assign30580_e48366) as f64).fract() == 0.0 { if assign30580_e48366 == 0.0 { 0.0 } else { (assign30580_e48366 * ((locals.var_vgef2).powf(assign30580_e48366 - 1.0) * locals.var_vgef2_dn3)) } } else { (assign30580_e48367 * (assign30580_e48366 * (locals.var_vgef2_dn3 / locals.var_vgef2))) })), (((p.p208 * locals.var_t0_dn4) * assign30580_e48367) + (assign30580_e48363 * if 0.0 == 0.0 && ((assign30580_e48366) as f64).is_finite() && ((assign30580_e48366) as f64).fract() == 0.0 { if assign30580_e48366 == 0.0 { 0.0 } else { (assign30580_e48366 * ((locals.var_vgef2).powf(assign30580_e48366 - 1.0) * locals.var_vgef2_dn4)) } } else { (assign30580_e48367 * (assign30580_e48366 * (locals.var_vgef2_dn4 / locals.var_vgef2))) })), (((p.p208 * locals.var_t0_dn5) * assign30580_e48367) + (assign30580_e48363 * if 0.0 == 0.0 && ((assign30580_e48366) as f64).is_finite() && ((assign30580_e48366) as f64).fract() == 0.0 { if assign30580_e48366 == 0.0 { 0.0 } else { (assign30580_e48366 * ((locals.var_vgef2).powf(assign30580_e48366 - 1.0) * locals.var_vgef2_dn5)) } } else { (assign30580_e48367 * (assign30580_e48366 * (locals.var_vgef2_dn5 / locals.var_vgef2))) })), (((p.p208 * locals.var_t0_dn6) * assign30580_e48367) + (assign30580_e48363 * if 0.0 == 0.0 && ((assign30580_e48366) as f64).is_finite() && ((assign30580_e48366) as f64).fract() == 0.0 { if assign30580_e48366 == 0.0 { 0.0 } else { (assign30580_e48366 * ((locals.var_vgef2).powf(assign30580_e48366 - 1.0) * locals.var_vgef2_dn6)) } } else { (assign30580_e48367 * (assign30580_e48366 * (locals.var_vgef2_dn6 / locals.var_vgef2))) })), (((p.p208 * locals.var_t0_dn7) * assign30580_e48367) + (assign30580_e48363 * if 0.0 == 0.0 && ((assign30580_e48366) as f64).is_finite() && ((assign30580_e48366) as f64).fract() == 0.0 { if assign30580_e48366 == 0.0 { 0.0 } else { (assign30580_e48366 * ((locals.var_vgef2).powf(assign30580_e48366 - 1.0) * locals.var_vgef2_dn7)) } } else { (assign30580_e48367 * (assign30580_e48366 * (locals.var_vgef2_dn7 / locals.var_vgef2))) })), (((p.p208 * locals.var_t0_dn8) * assign30580_e48367) + (assign30580_e48363 * if 0.0 == 0.0 && ((assign30580_e48366) as f64).is_finite() && ((assign30580_e48366) as f64).fract() == 0.0 { if assign30580_e48366 == 0.0 { 0.0 } else { (assign30580_e48366 * ((locals.var_vgef2).powf(assign30580_e48366 - 1.0) * locals.var_vgef2_dn8)) } } else { (assign30580_e48367 * (assign30580_e48366 * (locals.var_vgef2_dn8 / locals.var_vgef2))) })), (((p.p208 * locals.var_t0_dn9) * assign30580_e48367) + (assign30580_e48363 * if 0.0 == 0.0 && ((assign30580_e48366) as f64).is_finite() && ((assign30580_e48366) as f64).fract() == 0.0 { if assign30580_e48366 == 0.0 { 0.0 } else { (assign30580_e48366 * ((locals.var_vgef2).powf(assign30580_e48366 - 1.0) * locals.var_vgef2_dn9)) } } else { (assign30580_e48367 * (assign30580_e48366 * (locals.var_vgef2_dn9 / locals.var_vgef2))) })), (((p.p208 * locals.var_t0_dn12) * assign30580_e48367) + (assign30580_e48363 * if 0.0 == 0.0 && ((assign30580_e48366) as f64).is_finite() && ((assign30580_e48366) as f64).fract() == 0.0 { if assign30580_e48366 == 0.0 { 0.0 } else { (assign30580_e48366 * ((locals.var_vgef2).powf(assign30580_e48366 - 1.0) * locals.var_vgef2_dn12)) } } else { (assign30580_e48367 * (assign30580_e48366 * (locals.var_vgef2_dn12 / locals.var_vgef2))) })), (((p.p208 * locals.var_t0_dn14) * assign30580_e48367) + (assign30580_e48363 * if 0.0 == 0.0 && ((assign30580_e48366) as f64).is_finite() && ((assign30580_e48366) as f64).fract() == 0.0 { if assign30580_e48366 == 0.0 { 0.0 } else { (assign30580_e48366 * ((locals.var_vgef2).powf(assign30580_e48366 - 1.0) * locals.var_vgef2_dn14)) } } else { (assign30580_e48367 * (assign30580_e48366 * (locals.var_vgef2_dn14 / locals.var_vgef2))) })), (((p.p208 * locals.var_t0_dn15) * assign30580_e48367) + (assign30580_e48363 * if 0.0 == 0.0 && ((assign30580_e48366) as f64).is_finite() && ((assign30580_e48366) as f64).fract() == 0.0 { if assign30580_e48366 == 0.0 { 0.0 } else { (assign30580_e48366 * ((locals.var_vgef2).powf(assign30580_e48366 - 1.0) * locals.var_vgef2_dn15)) } } else { (assign30580_e48367 * (assign30580_e48366 * (locals.var_vgef2_dn15 / locals.var_vgef2))) })), (((p.p208 * locals.var_t0_dn16) * assign30580_e48367) + (assign30580_e48363 * if 0.0 == 0.0 && ((assign30580_e48366) as f64).is_finite() && ((assign30580_e48366) as f64).fract() == 0.0 { if assign30580_e48366 == 0.0 { 0.0 } else { (assign30580_e48366 * ((locals.var_vgef2).powf(assign30580_e48366 - 1.0) * locals.var_vgef2_dn16)) } } else { (assign30580_e48367 * (assign30580_e48366 * (locals.var_vgef2_dn16 / locals.var_vgef2))) })), (((p.p208 * locals.var_t0_dn17) * assign30580_e48367) + (assign30580_e48363 * if 0.0 == 0.0 && ((assign30580_e48366) as f64).is_finite() && ((assign30580_e48366) as f64).fract() == 0.0 { if assign30580_e48366 == 0.0 { 0.0 } else { (assign30580_e48366 * ((locals.var_vgef2).powf(assign30580_e48366 - 1.0) * locals.var_vgef2_dn17)) } } else { (assign30580_e48367 * (assign30580_e48366 * (locals.var_vgef2_dn17 / locals.var_vgef2))) })), (((p.p208 * locals.var_t0_dn18) * assign30580_e48367) + (assign30580_e48363 * if 0.0 == 0.0 && ((assign30580_e48366) as f64).is_finite() && ((assign30580_e48366) as f64).fract() == 0.0 { if assign30580_e48366 == 0.0 { 0.0 } else { (assign30580_e48366 * ((locals.var_vgef2).powf(assign30580_e48366 - 1.0) * locals.var_vgef2_dn18)) } } else { (assign30580_e48367 * (assign30580_e48366 * (locals.var_vgef2_dn18 / locals.var_vgef2))) })), (((p.p208 * locals.var_t0_dn19) * assign30580_e48367) + (assign30580_e48363 * if 0.0 == 0.0 && ((assign30580_e48366) as f64).is_finite() && ((assign30580_e48366) as f64).fract() == 0.0 { if assign30580_e48366 == 0.0 { 0.0 } else { (assign30580_e48366 * ((locals.var_vgef2).powf(assign30580_e48366 - 1.0) * locals.var_vgef2_dn19)) } } else { (assign30580_e48367 * (assign30580_e48366 * (locals.var_vgef2_dn19 / locals.var_vgef2))) })), (((p.p208 * locals.var_t0_dn20) * assign30580_e48367) + (assign30580_e48363 * if 0.0 == 0.0 && ((assign30580_e48366) as f64).is_finite() && ((assign30580_e48366) as f64).fract() == 0.0 { if assign30580_e48366 == 0.0 { 0.0 } else { (assign30580_e48366 * ((locals.var_vgef2).powf(assign30580_e48366 - 1.0) * locals.var_vgef2_dn20)) } } else { (assign30580_e48367 * (assign30580_e48366 * (locals.var_vgef2_dn20 / locals.var_vgef2))) })), (((p.p208 * locals.var_t0_dn21) * assign30580_e48367) + (assign30580_e48363 * if 0.0 == 0.0 && ((assign30580_e48366) as f64).is_finite() && ((assign30580_e48366) as f64).fract() == 0.0 { if assign30580_e48366 == 0.0 { 0.0 } else { (assign30580_e48366 * ((locals.var_vgef2).powf(assign30580_e48366 - 1.0) * locals.var_vgef2_dn21)) } } else { (assign30580_e48367 * (assign30580_e48366 * (locals.var_vgef2_dn21 / locals.var_vgef2))) })), (((p.p208 * locals.var_t0_dn22) * assign30580_e48367) + (assign30580_e48363 * if 0.0 == 0.0 && ((assign30580_e48366) as f64).is_finite() && ((assign30580_e48366) as f64).fract() == 0.0 { if assign30580_e48366 == 0.0 { 0.0 } else { (assign30580_e48366 * ((locals.var_vgef2).powf(assign30580_e48366 - 1.0) * locals.var_vgef2_dn22)) } } else { (assign30580_e48367 * (assign30580_e48366 * (locals.var_vgef2_dn22 / locals.var_vgef2))) })),)
    } else {
        (locals.var_vgefm213g0, locals.var_vgefm213g0_dn0, locals.var_vgefm213g0_dn1, locals.var_vgefm213g0_dn2, locals.var_vgefm213g0_dn3, locals.var_vgefm213g0_dn4, locals.var_vgefm213g0_dn5, locals.var_vgefm213g0_dn6, locals.var_vgefm213g0_dn7, locals.var_vgefm213g0_dn8, locals.var_vgefm213g0_dn9, locals.var_vgefm213g0_dn12, locals.var_vgefm213g0_dn14, locals.var_vgefm213g0_dn15, locals.var_vgefm213g0_dn16, locals.var_vgefm213g0_dn17, locals.var_vgefm213g0_dn18, locals.var_vgefm213g0_dn19, locals.var_vgefm213g0_dn20, locals.var_vgefm213g0_dn21, locals.var_vgefm213g0_dn22,)
    }
};
        locals.var_vgefm213g0 = assign30580_e48370;
        locals.var_vgefm213g0_dn0 = assign30580_e48370_d_n0;
        locals.var_vgefm213g0_dn1 = assign30580_e48370_d_n1;
        locals.var_vgefm213g0_dn2 = assign30580_e48370_d_n2;
        locals.var_vgefm213g0_dn3 = assign30580_e48370_d_n3;
        locals.var_vgefm213g0_dn4 = assign30580_e48370_d_n4;
        locals.var_vgefm213g0_dn5 = assign30580_e48370_d_n5;
        locals.var_vgefm213g0_dn6 = assign30580_e48370_d_n6;
        locals.var_vgefm213g0_dn7 = assign30580_e48370_d_n7;
        locals.var_vgefm213g0_dn8 = assign30580_e48370_d_n8;
        locals.var_vgefm213g0_dn9 = assign30580_e48370_d_n9;
        locals.var_vgefm213g0_dn12 = assign30580_e48370_d_n12;
        locals.var_vgefm213g0_dn14 = assign30580_e48370_d_n14;
        locals.var_vgefm213g0_dn15 = assign30580_e48370_d_n15;
        locals.var_vgefm213g0_dn16 = assign30580_e48370_d_n16;
        locals.var_vgefm213g0_dn17 = assign30580_e48370_d_n17;
        locals.var_vgefm213g0_dn18 = assign30580_e48370_d_n18;
        locals.var_vgefm213g0_dn19 = assign30580_e48370_d_n19;
        locals.var_vgefm213g0_dn20 = assign30580_e48370_d_n20;
        locals.var_vgefm213g0_dn21 = assign30580_e48370_d_n21;
        locals.var_vgefm213g0_dn22 = assign30580_e48370_d_n22;

        let (assign30590_e48386, assign30590_e48386_d_n0, assign30590_e48386_d_n1, assign30590_e48386_d_n2, assign30590_e48386_d_n3, assign30590_e48386_d_n4, assign30590_e48386_d_n5, assign30590_e48386_d_n6, assign30590_e48386_d_n7, assign30590_e48386_d_n8, assign30590_e48386_d_n9, assign30590_e48386_d_n12, assign30590_e48386_d_n14, assign30590_e48386_d_n15, assign30590_e48386_d_n16, assign30590_e48386_d_n17, assign30590_e48386_d_n18, assign30590_e48386_d_n19, assign30590_e48386_d_n20, assign30590_e48386_d_n21, assign30590_e48386_d_n22,) = {
    if (((locals.var_guard504 == 0.0) && (locals.var_guard513 != 0.0)) && (locals.var_guard518 != 0.0)) {
        let assign30590_e48379: f64 = (p.p209 * locals.var_t0);
        let assign30590_e48382: f64 = (-0.3333333333333333);
        let assign30590_e48383: f64 = (locals.var_vgef2).powf(assign30590_e48382);
        let assign30590_e48384: f64 = (assign30590_e48379 * assign30590_e48383);
        (assign30590_e48384, (((p.p209 * locals.var_t0_dn0) * assign30590_e48383) + (assign30590_e48379 * if 0.0 == 0.0 && ((assign30590_e48382) as f64).is_finite() && ((assign30590_e48382) as f64).fract() == 0.0 { if assign30590_e48382 == 0.0 { 0.0 } else { (assign30590_e48382 * ((locals.var_vgef2).powf(assign30590_e48382 - 1.0) * locals.var_vgef2_dn0)) } } else { (assign30590_e48383 * (assign30590_e48382 * (locals.var_vgef2_dn0 / locals.var_vgef2))) })), (((p.p209 * locals.var_t0_dn1) * assign30590_e48383) + (assign30590_e48379 * if 0.0 == 0.0 && ((assign30590_e48382) as f64).is_finite() && ((assign30590_e48382) as f64).fract() == 0.0 { if assign30590_e48382 == 0.0 { 0.0 } else { (assign30590_e48382 * ((locals.var_vgef2).powf(assign30590_e48382 - 1.0) * locals.var_vgef2_dn1)) } } else { (assign30590_e48383 * (assign30590_e48382 * (locals.var_vgef2_dn1 / locals.var_vgef2))) })), (((p.p209 * locals.var_t0_dn2) * assign30590_e48383) + (assign30590_e48379 * if 0.0 == 0.0 && ((assign30590_e48382) as f64).is_finite() && ((assign30590_e48382) as f64).fract() == 0.0 { if assign30590_e48382 == 0.0 { 0.0 } else { (assign30590_e48382 * ((locals.var_vgef2).powf(assign30590_e48382 - 1.0) * locals.var_vgef2_dn2)) } } else { (assign30590_e48383 * (assign30590_e48382 * (locals.var_vgef2_dn2 / locals.var_vgef2))) })), (((p.p209 * locals.var_t0_dn3) * assign30590_e48383) + (assign30590_e48379 * if 0.0 == 0.0 && ((assign30590_e48382) as f64).is_finite() && ((assign30590_e48382) as f64).fract() == 0.0 { if assign30590_e48382 == 0.0 { 0.0 } else { (assign30590_e48382 * ((locals.var_vgef2).powf(assign30590_e48382 - 1.0) * locals.var_vgef2_dn3)) } } else { (assign30590_e48383 * (assign30590_e48382 * (locals.var_vgef2_dn3 / locals.var_vgef2))) })), (((p.p209 * locals.var_t0_dn4) * assign30590_e48383) + (assign30590_e48379 * if 0.0 == 0.0 && ((assign30590_e48382) as f64).is_finite() && ((assign30590_e48382) as f64).fract() == 0.0 { if assign30590_e48382 == 0.0 { 0.0 } else { (assign30590_e48382 * ((locals.var_vgef2).powf(assign30590_e48382 - 1.0) * locals.var_vgef2_dn4)) } } else { (assign30590_e48383 * (assign30590_e48382 * (locals.var_vgef2_dn4 / locals.var_vgef2))) })), (((p.p209 * locals.var_t0_dn5) * assign30590_e48383) + (assign30590_e48379 * if 0.0 == 0.0 && ((assign30590_e48382) as f64).is_finite() && ((assign30590_e48382) as f64).fract() == 0.0 { if assign30590_e48382 == 0.0 { 0.0 } else { (assign30590_e48382 * ((locals.var_vgef2).powf(assign30590_e48382 - 1.0) * locals.var_vgef2_dn5)) } } else { (assign30590_e48383 * (assign30590_e48382 * (locals.var_vgef2_dn5 / locals.var_vgef2))) })), (((p.p209 * locals.var_t0_dn6) * assign30590_e48383) + (assign30590_e48379 * if 0.0 == 0.0 && ((assign30590_e48382) as f64).is_finite() && ((assign30590_e48382) as f64).fract() == 0.0 { if assign30590_e48382 == 0.0 { 0.0 } else { (assign30590_e48382 * ((locals.var_vgef2).powf(assign30590_e48382 - 1.0) * locals.var_vgef2_dn6)) } } else { (assign30590_e48383 * (assign30590_e48382 * (locals.var_vgef2_dn6 / locals.var_vgef2))) })), (((p.p209 * locals.var_t0_dn7) * assign30590_e48383) + (assign30590_e48379 * if 0.0 == 0.0 && ((assign30590_e48382) as f64).is_finite() && ((assign30590_e48382) as f64).fract() == 0.0 { if assign30590_e48382 == 0.0 { 0.0 } else { (assign30590_e48382 * ((locals.var_vgef2).powf(assign30590_e48382 - 1.0) * locals.var_vgef2_dn7)) } } else { (assign30590_e48383 * (assign30590_e48382 * (locals.var_vgef2_dn7 / locals.var_vgef2))) })), (((p.p209 * locals.var_t0_dn8) * assign30590_e48383) + (assign30590_e48379 * if 0.0 == 0.0 && ((assign30590_e48382) as f64).is_finite() && ((assign30590_e48382) as f64).fract() == 0.0 { if assign30590_e48382 == 0.0 { 0.0 } else { (assign30590_e48382 * ((locals.var_vgef2).powf(assign30590_e48382 - 1.0) * locals.var_vgef2_dn8)) } } else { (assign30590_e48383 * (assign30590_e48382 * (locals.var_vgef2_dn8 / locals.var_vgef2))) })), (((p.p209 * locals.var_t0_dn9) * assign30590_e48383) + (assign30590_e48379 * if 0.0 == 0.0 && ((assign30590_e48382) as f64).is_finite() && ((assign30590_e48382) as f64).fract() == 0.0 { if assign30590_e48382 == 0.0 { 0.0 } else { (assign30590_e48382 * ((locals.var_vgef2).powf(assign30590_e48382 - 1.0) * locals.var_vgef2_dn9)) } } else { (assign30590_e48383 * (assign30590_e48382 * (locals.var_vgef2_dn9 / locals.var_vgef2))) })), (((p.p209 * locals.var_t0_dn12) * assign30590_e48383) + (assign30590_e48379 * if 0.0 == 0.0 && ((assign30590_e48382) as f64).is_finite() && ((assign30590_e48382) as f64).fract() == 0.0 { if assign30590_e48382 == 0.0 { 0.0 } else { (assign30590_e48382 * ((locals.var_vgef2).powf(assign30590_e48382 - 1.0) * locals.var_vgef2_dn12)) } } else { (assign30590_e48383 * (assign30590_e48382 * (locals.var_vgef2_dn12 / locals.var_vgef2))) })), (((p.p209 * locals.var_t0_dn14) * assign30590_e48383) + (assign30590_e48379 * if 0.0 == 0.0 && ((assign30590_e48382) as f64).is_finite() && ((assign30590_e48382) as f64).fract() == 0.0 { if assign30590_e48382 == 0.0 { 0.0 } else { (assign30590_e48382 * ((locals.var_vgef2).powf(assign30590_e48382 - 1.0) * locals.var_vgef2_dn14)) } } else { (assign30590_e48383 * (assign30590_e48382 * (locals.var_vgef2_dn14 / locals.var_vgef2))) })), (((p.p209 * locals.var_t0_dn15) * assign30590_e48383) + (assign30590_e48379 * if 0.0 == 0.0 && ((assign30590_e48382) as f64).is_finite() && ((assign30590_e48382) as f64).fract() == 0.0 { if assign30590_e48382 == 0.0 { 0.0 } else { (assign30590_e48382 * ((locals.var_vgef2).powf(assign30590_e48382 - 1.0) * locals.var_vgef2_dn15)) } } else { (assign30590_e48383 * (assign30590_e48382 * (locals.var_vgef2_dn15 / locals.var_vgef2))) })), (((p.p209 * locals.var_t0_dn16) * assign30590_e48383) + (assign30590_e48379 * if 0.0 == 0.0 && ((assign30590_e48382) as f64).is_finite() && ((assign30590_e48382) as f64).fract() == 0.0 { if assign30590_e48382 == 0.0 { 0.0 } else { (assign30590_e48382 * ((locals.var_vgef2).powf(assign30590_e48382 - 1.0) * locals.var_vgef2_dn16)) } } else { (assign30590_e48383 * (assign30590_e48382 * (locals.var_vgef2_dn16 / locals.var_vgef2))) })), (((p.p209 * locals.var_t0_dn17) * assign30590_e48383) + (assign30590_e48379 * if 0.0 == 0.0 && ((assign30590_e48382) as f64).is_finite() && ((assign30590_e48382) as f64).fract() == 0.0 { if assign30590_e48382 == 0.0 { 0.0 } else { (assign30590_e48382 * ((locals.var_vgef2).powf(assign30590_e48382 - 1.0) * locals.var_vgef2_dn17)) } } else { (assign30590_e48383 * (assign30590_e48382 * (locals.var_vgef2_dn17 / locals.var_vgef2))) })), (((p.p209 * locals.var_t0_dn18) * assign30590_e48383) + (assign30590_e48379 * if 0.0 == 0.0 && ((assign30590_e48382) as f64).is_finite() && ((assign30590_e48382) as f64).fract() == 0.0 { if assign30590_e48382 == 0.0 { 0.0 } else { (assign30590_e48382 * ((locals.var_vgef2).powf(assign30590_e48382 - 1.0) * locals.var_vgef2_dn18)) } } else { (assign30590_e48383 * (assign30590_e48382 * (locals.var_vgef2_dn18 / locals.var_vgef2))) })), (((p.p209 * locals.var_t0_dn19) * assign30590_e48383) + (assign30590_e48379 * if 0.0 == 0.0 && ((assign30590_e48382) as f64).is_finite() && ((assign30590_e48382) as f64).fract() == 0.0 { if assign30590_e48382 == 0.0 { 0.0 } else { (assign30590_e48382 * ((locals.var_vgef2).powf(assign30590_e48382 - 1.0) * locals.var_vgef2_dn19)) } } else { (assign30590_e48383 * (assign30590_e48382 * (locals.var_vgef2_dn19 / locals.var_vgef2))) })), (((p.p209 * locals.var_t0_dn20) * assign30590_e48383) + (assign30590_e48379 * if 0.0 == 0.0 && ((assign30590_e48382) as f64).is_finite() && ((assign30590_e48382) as f64).fract() == 0.0 { if assign30590_e48382 == 0.0 { 0.0 } else { (assign30590_e48382 * ((locals.var_vgef2).powf(assign30590_e48382 - 1.0) * locals.var_vgef2_dn20)) } } else { (assign30590_e48383 * (assign30590_e48382 * (locals.var_vgef2_dn20 / locals.var_vgef2))) })), (((p.p209 * locals.var_t0_dn21) * assign30590_e48383) + (assign30590_e48379 * if 0.0 == 0.0 && ((assign30590_e48382) as f64).is_finite() && ((assign30590_e48382) as f64).fract() == 0.0 { if assign30590_e48382 == 0.0 { 0.0 } else { (assign30590_e48382 * ((locals.var_vgef2).powf(assign30590_e48382 - 1.0) * locals.var_vgef2_dn21)) } } else { (assign30590_e48383 * (assign30590_e48382 * (locals.var_vgef2_dn21 / locals.var_vgef2))) })), (((p.p209 * locals.var_t0_dn22) * assign30590_e48383) + (assign30590_e48379 * if 0.0 == 0.0 && ((assign30590_e48382) as f64).is_finite() && ((assign30590_e48382) as f64).fract() == 0.0 { if assign30590_e48382 == 0.0 { 0.0 } else { (assign30590_e48382 * ((locals.var_vgef2).powf(assign30590_e48382 - 1.0) * locals.var_vgef2_dn22)) } } else { (assign30590_e48383 * (assign30590_e48382 * (locals.var_vgef2_dn22 / locals.var_vgef2))) })),)
    } else {
        (locals.var_vgefm213g1, locals.var_vgefm213g1_dn0, locals.var_vgefm213g1_dn1, locals.var_vgefm213g1_dn2, locals.var_vgefm213g1_dn3, locals.var_vgefm213g1_dn4, locals.var_vgefm213g1_dn5, locals.var_vgefm213g1_dn6, locals.var_vgefm213g1_dn7, locals.var_vgefm213g1_dn8, locals.var_vgefm213g1_dn9, locals.var_vgefm213g1_dn12, locals.var_vgefm213g1_dn14, locals.var_vgefm213g1_dn15, locals.var_vgefm213g1_dn16, locals.var_vgefm213g1_dn17, locals.var_vgefm213g1_dn18, locals.var_vgefm213g1_dn19, locals.var_vgefm213g1_dn20, locals.var_vgefm213g1_dn21, locals.var_vgefm213g1_dn22,)
    }
};
        locals.var_vgefm213g1 = assign30590_e48386;
        locals.var_vgefm213g1_dn0 = assign30590_e48386_d_n0;
        locals.var_vgefm213g1_dn1 = assign30590_e48386_d_n1;
        locals.var_vgefm213g1_dn2 = assign30590_e48386_d_n2;
        locals.var_vgefm213g1_dn3 = assign30590_e48386_d_n3;
        locals.var_vgefm213g1_dn4 = assign30590_e48386_d_n4;
        locals.var_vgefm213g1_dn5 = assign30590_e48386_d_n5;
        locals.var_vgefm213g1_dn6 = assign30590_e48386_d_n6;
        locals.var_vgefm213g1_dn7 = assign30590_e48386_d_n7;
        locals.var_vgefm213g1_dn8 = assign30590_e48386_d_n8;
        locals.var_vgefm213g1_dn9 = assign30590_e48386_d_n9;
        locals.var_vgefm213g1_dn12 = assign30590_e48386_d_n12;
        locals.var_vgefm213g1_dn14 = assign30590_e48386_d_n14;
        locals.var_vgefm213g1_dn15 = assign30590_e48386_d_n15;
        locals.var_vgefm213g1_dn16 = assign30590_e48386_d_n16;
        locals.var_vgefm213g1_dn17 = assign30590_e48386_d_n17;
        locals.var_vgefm213g1_dn18 = assign30590_e48386_d_n18;
        locals.var_vgefm213g1_dn19 = assign30590_e48386_d_n19;
        locals.var_vgefm213g1_dn20 = assign30590_e48386_d_n20;
        locals.var_vgefm213g1_dn21 = assign30590_e48386_d_n21;
        locals.var_vgefm213g1_dn22 = assign30590_e48386_d_n22;

    }

    pub(super) fn stamp_transient_block_180(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign30600_e48404, assign30600_e48404_d_n0, assign30600_e48404_d_n1, assign30600_e48404_d_n2, assign30600_e48404_d_n3, assign30600_e48404_d_n4, assign30600_e48404_d_n5, assign30600_e48404_d_n6, assign30600_e48404_d_n7, assign30600_e48404_d_n8, assign30600_e48404_d_n9, assign30600_e48404_d_n12, assign30600_e48404_d_n14, assign30600_e48404_d_n15, assign30600_e48404_d_n16, assign30600_e48404_d_n17, assign30600_e48404_d_n18, assign30600_e48404_d_n19, assign30600_e48404_d_n20, assign30600_e48404_d_n21, assign30600_e48404_d_n22,) = {
    if (((locals.var_guard504 == 0.0) && (locals.var_guard513 != 0.0)) && (locals.var_guard518 != 0.0)) {
        let assign30600_e48394: f64 = { let limited_exp_arg = locals.var_tg02; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign30600_e48396: f64 = (assign30600_e48394 * 3.24e17);
        let assign30600_e48400: f64 = (0.6666666666666666 * locals.var_vgefm213g0);
        let assign30600_e48401: f64 = (1.0 + assign30600_e48400);
        let assign30600_e48402: f64 = (assign30600_e48396 * assign30600_e48401);
        (assign30600_e48402, (((({ let limited_exp_arg = locals.var_tg02; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg02_dn0) * 3.24e17) * assign30600_e48401) + (assign30600_e48396 * (0.6666666666666666 * locals.var_vgefm213g0_dn0))), (((({ let limited_exp_arg = locals.var_tg02; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg02_dn1) * 3.24e17) * assign30600_e48401) + (assign30600_e48396 * (0.6666666666666666 * locals.var_vgefm213g0_dn1))), (((({ let limited_exp_arg = locals.var_tg02; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg02_dn2) * 3.24e17) * assign30600_e48401) + (assign30600_e48396 * (0.6666666666666666 * locals.var_vgefm213g0_dn2))), (((({ let limited_exp_arg = locals.var_tg02; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg02_dn3) * 3.24e17) * assign30600_e48401) + (assign30600_e48396 * (0.6666666666666666 * locals.var_vgefm213g0_dn3))), (((({ let limited_exp_arg = locals.var_tg02; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg02_dn4) * 3.24e17) * assign30600_e48401) + (assign30600_e48396 * (0.6666666666666666 * locals.var_vgefm213g0_dn4))), (((({ let limited_exp_arg = locals.var_tg02; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg02_dn5) * 3.24e17) * assign30600_e48401) + (assign30600_e48396 * (0.6666666666666666 * locals.var_vgefm213g0_dn5))), (((({ let limited_exp_arg = locals.var_tg02; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg02_dn6) * 3.24e17) * assign30600_e48401) + (assign30600_e48396 * (0.6666666666666666 * locals.var_vgefm213g0_dn6))), (((({ let limited_exp_arg = locals.var_tg02; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg02_dn7) * 3.24e17) * assign30600_e48401) + (assign30600_e48396 * (0.6666666666666666 * locals.var_vgefm213g0_dn7))), (((({ let limited_exp_arg = locals.var_tg02; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg02_dn8) * 3.24e17) * assign30600_e48401) + (assign30600_e48396 * (0.6666666666666666 * locals.var_vgefm213g0_dn8))), (((({ let limited_exp_arg = locals.var_tg02; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg02_dn9) * 3.24e17) * assign30600_e48401) + (assign30600_e48396 * (0.6666666666666666 * locals.var_vgefm213g0_dn9))), (((({ let limited_exp_arg = locals.var_tg02; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg02_dn12) * 3.24e17) * assign30600_e48401) + (assign30600_e48396 * (0.6666666666666666 * locals.var_vgefm213g0_dn12))), (((({ let limited_exp_arg = locals.var_tg02; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg02_dn14) * 3.24e17) * assign30600_e48401) + (assign30600_e48396 * (0.6666666666666666 * locals.var_vgefm213g0_dn14))), (((({ let limited_exp_arg = locals.var_tg02; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg02_dn15) * 3.24e17) * assign30600_e48401) + (assign30600_e48396 * (0.6666666666666666 * locals.var_vgefm213g0_dn15))), (((({ let limited_exp_arg = locals.var_tg02; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg02_dn16) * 3.24e17) * assign30600_e48401) + (assign30600_e48396 * (0.6666666666666666 * locals.var_vgefm213g0_dn16))), (((({ let limited_exp_arg = locals.var_tg02; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg02_dn17) * 3.24e17) * assign30600_e48401) + (assign30600_e48396 * (0.6666666666666666 * locals.var_vgefm213g0_dn17))), (((({ let limited_exp_arg = locals.var_tg02; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg02_dn18) * 3.24e17) * assign30600_e48401) + (assign30600_e48396 * (0.6666666666666666 * locals.var_vgefm213g0_dn18))), (((({ let limited_exp_arg = locals.var_tg02; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg02_dn19) * 3.24e17) * assign30600_e48401) + (assign30600_e48396 * (0.6666666666666666 * locals.var_vgefm213g0_dn19))), (((({ let limited_exp_arg = locals.var_tg02; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg02_dn20) * 3.24e17) * assign30600_e48401) + (assign30600_e48396 * (0.6666666666666666 * locals.var_vgefm213g0_dn20))), (((({ let limited_exp_arg = locals.var_tg02; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg02_dn21) * 3.24e17) * assign30600_e48401) + (assign30600_e48396 * (0.6666666666666666 * locals.var_vgefm213g0_dn21))), (((({ let limited_exp_arg = locals.var_tg02; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg02_dn22) * 3.24e17) * assign30600_e48401) + (assign30600_e48396 * (0.6666666666666666 * locals.var_vgefm213g0_dn22))),)
    } else {
        (locals.var_t5ng02, locals.var_t5ng02_dn0, locals.var_t5ng02_dn1, locals.var_t5ng02_dn2, locals.var_t5ng02_dn3, locals.var_t5ng02_dn4, locals.var_t5ng02_dn5, locals.var_t5ng02_dn6, locals.var_t5ng02_dn7, locals.var_t5ng02_dn8, locals.var_t5ng02_dn9, locals.var_t5ng02_dn12, locals.var_t5ng02_dn14, locals.var_t5ng02_dn15, locals.var_t5ng02_dn16, locals.var_t5ng02_dn17, locals.var_t5ng02_dn18, locals.var_t5ng02_dn19, locals.var_t5ng02_dn20, locals.var_t5ng02_dn21, locals.var_t5ng02_dn22,)
    }
};
        locals.var_t5ng02 = assign30600_e48404;
        locals.var_t5ng02_dn0 = assign30600_e48404_d_n0;
        locals.var_t5ng02_dn1 = assign30600_e48404_d_n1;
        locals.var_t5ng02_dn2 = assign30600_e48404_d_n2;
        locals.var_t5ng02_dn3 = assign30600_e48404_d_n3;
        locals.var_t5ng02_dn4 = assign30600_e48404_d_n4;
        locals.var_t5ng02_dn5 = assign30600_e48404_d_n5;
        locals.var_t5ng02_dn6 = assign30600_e48404_d_n6;
        locals.var_t5ng02_dn7 = assign30600_e48404_d_n7;
        locals.var_t5ng02_dn8 = assign30600_e48404_d_n8;
        locals.var_t5ng02_dn9 = assign30600_e48404_d_n9;
        locals.var_t5ng02_dn12 = assign30600_e48404_d_n12;
        locals.var_t5ng02_dn14 = assign30600_e48404_d_n14;
        locals.var_t5ng02_dn15 = assign30600_e48404_d_n15;
        locals.var_t5ng02_dn16 = assign30600_e48404_d_n16;
        locals.var_t5ng02_dn17 = assign30600_e48404_d_n17;
        locals.var_t5ng02_dn18 = assign30600_e48404_d_n18;
        locals.var_t5ng02_dn19 = assign30600_e48404_d_n19;
        locals.var_t5ng02_dn20 = assign30600_e48404_d_n20;
        locals.var_t5ng02_dn21 = assign30600_e48404_d_n21;
        locals.var_t5ng02_dn22 = assign30600_e48404_d_n22;

        let (assign30610_e48416, assign30610_e48416_d_n0, assign30610_e48416_d_n1, assign30610_e48416_d_n2, assign30610_e48416_d_n3, assign30610_e48416_d_n4, assign30610_e48416_d_n5, assign30610_e48416_d_n6, assign30610_e48416_d_n7, assign30610_e48416_d_n8, assign30610_e48416_d_n9, assign30610_e48416_d_n12, assign30610_e48416_d_n14, assign30610_e48416_d_n15, assign30610_e48416_d_n16, assign30610_e48416_d_n17, assign30610_e48416_d_n18, assign30610_e48416_d_n19, assign30610_e48416_d_n20, assign30610_e48416_d_n21, assign30610_e48416_d_n22,) = {
    if (((locals.var_guard504 == 0.0) && (locals.var_guard513 != 0.0)) && (locals.var_guard518 != 0.0)) {
        let assign30610_e48413: f64 = { let limited_exp_arg = locals.var_tg02; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign30610_e48414: f64 = (1.0 + assign30610_e48413);
        (assign30610_e48414, ({ let limited_exp_arg = locals.var_tg02; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg02_dn0), ({ let limited_exp_arg = locals.var_tg02; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg02_dn1), ({ let limited_exp_arg = locals.var_tg02; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg02_dn2), ({ let limited_exp_arg = locals.var_tg02; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg02_dn3), ({ let limited_exp_arg = locals.var_tg02; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg02_dn4), ({ let limited_exp_arg = locals.var_tg02; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg02_dn5), ({ let limited_exp_arg = locals.var_tg02; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg02_dn6), ({ let limited_exp_arg = locals.var_tg02; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg02_dn7), ({ let limited_exp_arg = locals.var_tg02; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg02_dn8), ({ let limited_exp_arg = locals.var_tg02; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg02_dn9), ({ let limited_exp_arg = locals.var_tg02; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg02_dn12), ({ let limited_exp_arg = locals.var_tg02; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg02_dn14), ({ let limited_exp_arg = locals.var_tg02; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg02_dn15), ({ let limited_exp_arg = locals.var_tg02; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg02_dn16), ({ let limited_exp_arg = locals.var_tg02; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg02_dn17), ({ let limited_exp_arg = locals.var_tg02; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg02_dn18), ({ let limited_exp_arg = locals.var_tg02; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg02_dn19), ({ let limited_exp_arg = locals.var_tg02; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg02_dn20), ({ let limited_exp_arg = locals.var_tg02; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg02_dn21), ({ let limited_exp_arg = locals.var_tg02; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg02_dn22),)
    } else {
        (locals.var_t5dg02, locals.var_t5dg02_dn0, locals.var_t5dg02_dn1, locals.var_t5dg02_dn2, locals.var_t5dg02_dn3, locals.var_t5dg02_dn4, locals.var_t5dg02_dn5, locals.var_t5dg02_dn6, locals.var_t5dg02_dn7, locals.var_t5dg02_dn8, locals.var_t5dg02_dn9, locals.var_t5dg02_dn12, locals.var_t5dg02_dn14, locals.var_t5dg02_dn15, locals.var_t5dg02_dn16, locals.var_t5dg02_dn17, locals.var_t5dg02_dn18, locals.var_t5dg02_dn19, locals.var_t5dg02_dn20, locals.var_t5dg02_dn21, locals.var_t5dg02_dn22,)
    }
};
        locals.var_t5dg02 = assign30610_e48416;
        locals.var_t5dg02_dn0 = assign30610_e48416_d_n0;
        locals.var_t5dg02_dn1 = assign30610_e48416_d_n1;
        locals.var_t5dg02_dn2 = assign30610_e48416_d_n2;
        locals.var_t5dg02_dn3 = assign30610_e48416_d_n3;
        locals.var_t5dg02_dn4 = assign30610_e48416_d_n4;
        locals.var_t5dg02_dn5 = assign30610_e48416_d_n5;
        locals.var_t5dg02_dn6 = assign30610_e48416_d_n6;
        locals.var_t5dg02_dn7 = assign30610_e48416_d_n7;
        locals.var_t5dg02_dn8 = assign30610_e48416_d_n8;
        locals.var_t5dg02_dn9 = assign30610_e48416_d_n9;
        locals.var_t5dg02_dn12 = assign30610_e48416_d_n12;
        locals.var_t5dg02_dn14 = assign30610_e48416_d_n14;
        locals.var_t5dg02_dn15 = assign30610_e48416_d_n15;
        locals.var_t5dg02_dn16 = assign30610_e48416_d_n16;
        locals.var_t5dg02_dn17 = assign30610_e48416_d_n17;
        locals.var_t5dg02_dn18 = assign30610_e48416_d_n18;
        locals.var_t5dg02_dn19 = assign30610_e48416_d_n19;
        locals.var_t5dg02_dn20 = assign30610_e48416_d_n20;
        locals.var_t5dg02_dn21 = assign30610_e48416_d_n21;
        locals.var_t5dg02_dn22 = assign30610_e48416_d_n22;

        let (assign30620_e48434, assign30620_e48434_d_n0, assign30620_e48434_d_n1, assign30620_e48434_d_n2, assign30620_e48434_d_n3, assign30620_e48434_d_n4, assign30620_e48434_d_n5, assign30620_e48434_d_n6, assign30620_e48434_d_n7, assign30620_e48434_d_n8, assign30620_e48434_d_n9, assign30620_e48434_d_n12, assign30620_e48434_d_n14, assign30620_e48434_d_n15, assign30620_e48434_d_n16, assign30620_e48434_d_n17, assign30620_e48434_d_n18, assign30620_e48434_d_n19, assign30620_e48434_d_n20, assign30620_e48434_d_n21, assign30620_e48434_d_n22,) = {
    if (((locals.var_guard504 == 0.0) && (locals.var_guard513 != 0.0)) && (locals.var_guard518 != 0.0)) {
        let assign30620_e48424: f64 = { let limited_exp_arg = locals.var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign30620_e48426: f64 = (assign30620_e48424 * 3.24e17);
        let assign30620_e48430: f64 = (0.6666666666666666 * locals.var_vgefm213g1);
        let assign30620_e48431: f64 = (1.0 + assign30620_e48430);
        let assign30620_e48432: f64 = (assign30620_e48426 * assign30620_e48431);
        (assign30620_e48432, (((({ let limited_exp_arg = locals.var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg12_dn0) * 3.24e17) * assign30620_e48431) + (assign30620_e48426 * (0.6666666666666666 * locals.var_vgefm213g1_dn0))), (((({ let limited_exp_arg = locals.var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg12_dn1) * 3.24e17) * assign30620_e48431) + (assign30620_e48426 * (0.6666666666666666 * locals.var_vgefm213g1_dn1))), (((({ let limited_exp_arg = locals.var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg12_dn2) * 3.24e17) * assign30620_e48431) + (assign30620_e48426 * (0.6666666666666666 * locals.var_vgefm213g1_dn2))), (((({ let limited_exp_arg = locals.var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg12_dn3) * 3.24e17) * assign30620_e48431) + (assign30620_e48426 * (0.6666666666666666 * locals.var_vgefm213g1_dn3))), (((({ let limited_exp_arg = locals.var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg12_dn4) * 3.24e17) * assign30620_e48431) + (assign30620_e48426 * (0.6666666666666666 * locals.var_vgefm213g1_dn4))), (((({ let limited_exp_arg = locals.var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg12_dn5) * 3.24e17) * assign30620_e48431) + (assign30620_e48426 * (0.6666666666666666 * locals.var_vgefm213g1_dn5))), (((({ let limited_exp_arg = locals.var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg12_dn6) * 3.24e17) * assign30620_e48431) + (assign30620_e48426 * (0.6666666666666666 * locals.var_vgefm213g1_dn6))), (((({ let limited_exp_arg = locals.var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg12_dn7) * 3.24e17) * assign30620_e48431) + (assign30620_e48426 * (0.6666666666666666 * locals.var_vgefm213g1_dn7))), (((({ let limited_exp_arg = locals.var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg12_dn8) * 3.24e17) * assign30620_e48431) + (assign30620_e48426 * (0.6666666666666666 * locals.var_vgefm213g1_dn8))), (((({ let limited_exp_arg = locals.var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg12_dn9) * 3.24e17) * assign30620_e48431) + (assign30620_e48426 * (0.6666666666666666 * locals.var_vgefm213g1_dn9))), (((({ let limited_exp_arg = locals.var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg12_dn12) * 3.24e17) * assign30620_e48431) + (assign30620_e48426 * (0.6666666666666666 * locals.var_vgefm213g1_dn12))), (((({ let limited_exp_arg = locals.var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg12_dn14) * 3.24e17) * assign30620_e48431) + (assign30620_e48426 * (0.6666666666666666 * locals.var_vgefm213g1_dn14))), (((({ let limited_exp_arg = locals.var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg12_dn15) * 3.24e17) * assign30620_e48431) + (assign30620_e48426 * (0.6666666666666666 * locals.var_vgefm213g1_dn15))), (((({ let limited_exp_arg = locals.var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg12_dn16) * 3.24e17) * assign30620_e48431) + (assign30620_e48426 * (0.6666666666666666 * locals.var_vgefm213g1_dn16))), (((({ let limited_exp_arg = locals.var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg12_dn17) * 3.24e17) * assign30620_e48431) + (assign30620_e48426 * (0.6666666666666666 * locals.var_vgefm213g1_dn17))), (((({ let limited_exp_arg = locals.var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg12_dn18) * 3.24e17) * assign30620_e48431) + (assign30620_e48426 * (0.6666666666666666 * locals.var_vgefm213g1_dn18))), (((({ let limited_exp_arg = locals.var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg12_dn19) * 3.24e17) * assign30620_e48431) + (assign30620_e48426 * (0.6666666666666666 * locals.var_vgefm213g1_dn19))), (((({ let limited_exp_arg = locals.var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg12_dn20) * 3.24e17) * assign30620_e48431) + (assign30620_e48426 * (0.6666666666666666 * locals.var_vgefm213g1_dn20))), (((({ let limited_exp_arg = locals.var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg12_dn21) * 3.24e17) * assign30620_e48431) + (assign30620_e48426 * (0.6666666666666666 * locals.var_vgefm213g1_dn21))), (((({ let limited_exp_arg = locals.var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg12_dn22) * 3.24e17) * assign30620_e48431) + (assign30620_e48426 * (0.6666666666666666 * locals.var_vgefm213g1_dn22))),)
    } else {
        (locals.var_t5ng12, locals.var_t5ng12_dn0, locals.var_t5ng12_dn1, locals.var_t5ng12_dn2, locals.var_t5ng12_dn3, locals.var_t5ng12_dn4, locals.var_t5ng12_dn5, locals.var_t5ng12_dn6, locals.var_t5ng12_dn7, locals.var_t5ng12_dn8, locals.var_t5ng12_dn9, locals.var_t5ng12_dn12, locals.var_t5ng12_dn14, locals.var_t5ng12_dn15, locals.var_t5ng12_dn16, locals.var_t5ng12_dn17, locals.var_t5ng12_dn18, locals.var_t5ng12_dn19, locals.var_t5ng12_dn20, locals.var_t5ng12_dn21, locals.var_t5ng12_dn22,)
    }
};
        locals.var_t5ng12 = assign30620_e48434;
        locals.var_t5ng12_dn0 = assign30620_e48434_d_n0;
        locals.var_t5ng12_dn1 = assign30620_e48434_d_n1;
        locals.var_t5ng12_dn2 = assign30620_e48434_d_n2;
        locals.var_t5ng12_dn3 = assign30620_e48434_d_n3;
        locals.var_t5ng12_dn4 = assign30620_e48434_d_n4;
        locals.var_t5ng12_dn5 = assign30620_e48434_d_n5;
        locals.var_t5ng12_dn6 = assign30620_e48434_d_n6;
        locals.var_t5ng12_dn7 = assign30620_e48434_d_n7;
        locals.var_t5ng12_dn8 = assign30620_e48434_d_n8;
        locals.var_t5ng12_dn9 = assign30620_e48434_d_n9;
        locals.var_t5ng12_dn12 = assign30620_e48434_d_n12;
        locals.var_t5ng12_dn14 = assign30620_e48434_d_n14;
        locals.var_t5ng12_dn15 = assign30620_e48434_d_n15;
        locals.var_t5ng12_dn16 = assign30620_e48434_d_n16;
        locals.var_t5ng12_dn17 = assign30620_e48434_d_n17;
        locals.var_t5ng12_dn18 = assign30620_e48434_d_n18;
        locals.var_t5ng12_dn19 = assign30620_e48434_d_n19;
        locals.var_t5ng12_dn20 = assign30620_e48434_d_n20;
        locals.var_t5ng12_dn21 = assign30620_e48434_d_n21;
        locals.var_t5ng12_dn22 = assign30620_e48434_d_n22;

        let (assign30630_e48446, assign30630_e48446_d_n0, assign30630_e48446_d_n1, assign30630_e48446_d_n2, assign30630_e48446_d_n3, assign30630_e48446_d_n4, assign30630_e48446_d_n5, assign30630_e48446_d_n6, assign30630_e48446_d_n7, assign30630_e48446_d_n8, assign30630_e48446_d_n9, assign30630_e48446_d_n12, assign30630_e48446_d_n14, assign30630_e48446_d_n15, assign30630_e48446_d_n16, assign30630_e48446_d_n17, assign30630_e48446_d_n18, assign30630_e48446_d_n19, assign30630_e48446_d_n20, assign30630_e48446_d_n21, assign30630_e48446_d_n22,) = {
    if (((locals.var_guard504 == 0.0) && (locals.var_guard513 != 0.0)) && (locals.var_guard518 != 0.0)) {
        let assign30630_e48443: f64 = { let limited_exp_arg = locals.var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign30630_e48444: f64 = (1.0 + assign30630_e48443);
        (assign30630_e48444, ({ let limited_exp_arg = locals.var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg12_dn0), ({ let limited_exp_arg = locals.var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg12_dn1), ({ let limited_exp_arg = locals.var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg12_dn2), ({ let limited_exp_arg = locals.var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg12_dn3), ({ let limited_exp_arg = locals.var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg12_dn4), ({ let limited_exp_arg = locals.var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg12_dn5), ({ let limited_exp_arg = locals.var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg12_dn6), ({ let limited_exp_arg = locals.var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg12_dn7), ({ let limited_exp_arg = locals.var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg12_dn8), ({ let limited_exp_arg = locals.var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg12_dn9), ({ let limited_exp_arg = locals.var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg12_dn12), ({ let limited_exp_arg = locals.var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg12_dn14), ({ let limited_exp_arg = locals.var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg12_dn15), ({ let limited_exp_arg = locals.var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg12_dn16), ({ let limited_exp_arg = locals.var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg12_dn17), ({ let limited_exp_arg = locals.var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg12_dn18), ({ let limited_exp_arg = locals.var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg12_dn19), ({ let limited_exp_arg = locals.var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg12_dn20), ({ let limited_exp_arg = locals.var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg12_dn21), ({ let limited_exp_arg = locals.var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_tg12_dn22),)
    } else {
        (locals.var_t5dg12, locals.var_t5dg12_dn0, locals.var_t5dg12_dn1, locals.var_t5dg12_dn2, locals.var_t5dg12_dn3, locals.var_t5dg12_dn4, locals.var_t5dg12_dn5, locals.var_t5dg12_dn6, locals.var_t5dg12_dn7, locals.var_t5dg12_dn8, locals.var_t5dg12_dn9, locals.var_t5dg12_dn12, locals.var_t5dg12_dn14, locals.var_t5dg12_dn15, locals.var_t5dg12_dn16, locals.var_t5dg12_dn17, locals.var_t5dg12_dn18, locals.var_t5dg12_dn19, locals.var_t5dg12_dn20, locals.var_t5dg12_dn21, locals.var_t5dg12_dn22,)
    }
};
        locals.var_t5dg12 = assign30630_e48446;
        locals.var_t5dg12_dn0 = assign30630_e48446_d_n0;
        locals.var_t5dg12_dn1 = assign30630_e48446_d_n1;
        locals.var_t5dg12_dn2 = assign30630_e48446_d_n2;
        locals.var_t5dg12_dn3 = assign30630_e48446_d_n3;
        locals.var_t5dg12_dn4 = assign30630_e48446_d_n4;
        locals.var_t5dg12_dn5 = assign30630_e48446_d_n5;
        locals.var_t5dg12_dn6 = assign30630_e48446_d_n6;
        locals.var_t5dg12_dn7 = assign30630_e48446_d_n7;
        locals.var_t5dg12_dn8 = assign30630_e48446_d_n8;
        locals.var_t5dg12_dn9 = assign30630_e48446_d_n9;
        locals.var_t5dg12_dn12 = assign30630_e48446_d_n12;
        locals.var_t5dg12_dn14 = assign30630_e48446_d_n14;
        locals.var_t5dg12_dn15 = assign30630_e48446_d_n15;
        locals.var_t5dg12_dn16 = assign30630_e48446_d_n16;
        locals.var_t5dg12_dn17 = assign30630_e48446_d_n17;
        locals.var_t5dg12_dn18 = assign30630_e48446_d_n18;
        locals.var_t5dg12_dn19 = assign30630_e48446_d_n19;
        locals.var_t5dg12_dn20 = assign30630_e48446_d_n20;
        locals.var_t5dg12_dn21 = assign30630_e48446_d_n21;
        locals.var_t5dg12_dn22 = assign30630_e48446_d_n22;

        let (assign30640_e48466, assign30640_e48466_d_n0, assign30640_e48466_d_n1, assign30640_e48466_d_n2, assign30640_e48466_d_n3, assign30640_e48466_d_n4, assign30640_e48466_d_n5, assign30640_e48466_d_n6, assign30640_e48466_d_n7, assign30640_e48466_d_n8, assign30640_e48466_d_n9, assign30640_e48466_d_n12, assign30640_e48466_d_n14, assign30640_e48466_d_n15, assign30640_e48466_d_n16, assign30640_e48466_d_n17, assign30640_e48466_d_n18, assign30640_e48466_d_n19, assign30640_e48466_d_n20, assign30640_e48466_d_n21, assign30640_e48466_d_n22,) = {
    if (((locals.var_guard504 == 0.0) && (locals.var_guard513 != 0.0)) && (locals.var_guard518 != 0.0)) {
        let assign30640_e48454: f64 = (-1.0);
        let assign30640_e48456: f64 = (assign30640_e48454 * locals.var_cch);
        let assign30640_e48459: f64 = (locals.var_t5ng02 / locals.var_t5dg02);
        let assign30640_e48460: f64 = (assign30640_e48456 - assign30640_e48459);
        let assign30640_e48463: f64 = (locals.var_t5ng12 / locals.var_t5dg12);
        let assign30640_e48464: f64 = (assign30640_e48460 - assign30640_e48463);
        (assign30640_e48464, ((-(((locals.var_t5ng02_dn0 * locals.var_t5dg02) - (locals.var_t5ng02 * locals.var_t5dg02_dn0)) / (locals.var_t5dg02 * locals.var_t5dg02))) - (((locals.var_t5ng12_dn0 * locals.var_t5dg12) - (locals.var_t5ng12 * locals.var_t5dg12_dn0)) / (locals.var_t5dg12 * locals.var_t5dg12))), ((-(((locals.var_t5ng02_dn1 * locals.var_t5dg02) - (locals.var_t5ng02 * locals.var_t5dg02_dn1)) / (locals.var_t5dg02 * locals.var_t5dg02))) - (((locals.var_t5ng12_dn1 * locals.var_t5dg12) - (locals.var_t5ng12 * locals.var_t5dg12_dn1)) / (locals.var_t5dg12 * locals.var_t5dg12))), ((-(((locals.var_t5ng02_dn2 * locals.var_t5dg02) - (locals.var_t5ng02 * locals.var_t5dg02_dn2)) / (locals.var_t5dg02 * locals.var_t5dg02))) - (((locals.var_t5ng12_dn2 * locals.var_t5dg12) - (locals.var_t5ng12 * locals.var_t5dg12_dn2)) / (locals.var_t5dg12 * locals.var_t5dg12))), ((-(((locals.var_t5ng02_dn3 * locals.var_t5dg02) - (locals.var_t5ng02 * locals.var_t5dg02_dn3)) / (locals.var_t5dg02 * locals.var_t5dg02))) - (((locals.var_t5ng12_dn3 * locals.var_t5dg12) - (locals.var_t5ng12 * locals.var_t5dg12_dn3)) / (locals.var_t5dg12 * locals.var_t5dg12))), ((-(((locals.var_t5ng02_dn4 * locals.var_t5dg02) - (locals.var_t5ng02 * locals.var_t5dg02_dn4)) / (locals.var_t5dg02 * locals.var_t5dg02))) - (((locals.var_t5ng12_dn4 * locals.var_t5dg12) - (locals.var_t5ng12 * locals.var_t5dg12_dn4)) / (locals.var_t5dg12 * locals.var_t5dg12))), ((-(((locals.var_t5ng02_dn5 * locals.var_t5dg02) - (locals.var_t5ng02 * locals.var_t5dg02_dn5)) / (locals.var_t5dg02 * locals.var_t5dg02))) - (((locals.var_t5ng12_dn5 * locals.var_t5dg12) - (locals.var_t5ng12 * locals.var_t5dg12_dn5)) / (locals.var_t5dg12 * locals.var_t5dg12))), ((-(((locals.var_t5ng02_dn6 * locals.var_t5dg02) - (locals.var_t5ng02 * locals.var_t5dg02_dn6)) / (locals.var_t5dg02 * locals.var_t5dg02))) - (((locals.var_t5ng12_dn6 * locals.var_t5dg12) - (locals.var_t5ng12 * locals.var_t5dg12_dn6)) / (locals.var_t5dg12 * locals.var_t5dg12))), ((-(((locals.var_t5ng02_dn7 * locals.var_t5dg02) - (locals.var_t5ng02 * locals.var_t5dg02_dn7)) / (locals.var_t5dg02 * locals.var_t5dg02))) - (((locals.var_t5ng12_dn7 * locals.var_t5dg12) - (locals.var_t5ng12 * locals.var_t5dg12_dn7)) / (locals.var_t5dg12 * locals.var_t5dg12))), ((-(((locals.var_t5ng02_dn8 * locals.var_t5dg02) - (locals.var_t5ng02 * locals.var_t5dg02_dn8)) / (locals.var_t5dg02 * locals.var_t5dg02))) - (((locals.var_t5ng12_dn8 * locals.var_t5dg12) - (locals.var_t5ng12 * locals.var_t5dg12_dn8)) / (locals.var_t5dg12 * locals.var_t5dg12))), ((-(((locals.var_t5ng02_dn9 * locals.var_t5dg02) - (locals.var_t5ng02 * locals.var_t5dg02_dn9)) / (locals.var_t5dg02 * locals.var_t5dg02))) - (((locals.var_t5ng12_dn9 * locals.var_t5dg12) - (locals.var_t5ng12 * locals.var_t5dg12_dn9)) / (locals.var_t5dg12 * locals.var_t5dg12))), ((-(((locals.var_t5ng02_dn12 * locals.var_t5dg02) - (locals.var_t5ng02 * locals.var_t5dg02_dn12)) / (locals.var_t5dg02 * locals.var_t5dg02))) - (((locals.var_t5ng12_dn12 * locals.var_t5dg12) - (locals.var_t5ng12 * locals.var_t5dg12_dn12)) / (locals.var_t5dg12 * locals.var_t5dg12))), ((-(((locals.var_t5ng02_dn14 * locals.var_t5dg02) - (locals.var_t5ng02 * locals.var_t5dg02_dn14)) / (locals.var_t5dg02 * locals.var_t5dg02))) - (((locals.var_t5ng12_dn14 * locals.var_t5dg12) - (locals.var_t5ng12 * locals.var_t5dg12_dn14)) / (locals.var_t5dg12 * locals.var_t5dg12))), ((-(((locals.var_t5ng02_dn15 * locals.var_t5dg02) - (locals.var_t5ng02 * locals.var_t5dg02_dn15)) / (locals.var_t5dg02 * locals.var_t5dg02))) - (((locals.var_t5ng12_dn15 * locals.var_t5dg12) - (locals.var_t5ng12 * locals.var_t5dg12_dn15)) / (locals.var_t5dg12 * locals.var_t5dg12))), ((-(((locals.var_t5ng02_dn16 * locals.var_t5dg02) - (locals.var_t5ng02 * locals.var_t5dg02_dn16)) / (locals.var_t5dg02 * locals.var_t5dg02))) - (((locals.var_t5ng12_dn16 * locals.var_t5dg12) - (locals.var_t5ng12 * locals.var_t5dg12_dn16)) / (locals.var_t5dg12 * locals.var_t5dg12))), ((-(((locals.var_t5ng02_dn17 * locals.var_t5dg02) - (locals.var_t5ng02 * locals.var_t5dg02_dn17)) / (locals.var_t5dg02 * locals.var_t5dg02))) - (((locals.var_t5ng12_dn17 * locals.var_t5dg12) - (locals.var_t5ng12 * locals.var_t5dg12_dn17)) / (locals.var_t5dg12 * locals.var_t5dg12))), ((-(((locals.var_t5ng02_dn18 * locals.var_t5dg02) - (locals.var_t5ng02 * locals.var_t5dg02_dn18)) / (locals.var_t5dg02 * locals.var_t5dg02))) - (((locals.var_t5ng12_dn18 * locals.var_t5dg12) - (locals.var_t5ng12 * locals.var_t5dg12_dn18)) / (locals.var_t5dg12 * locals.var_t5dg12))), ((-(((locals.var_t5ng02_dn19 * locals.var_t5dg02) - (locals.var_t5ng02 * locals.var_t5dg02_dn19)) / (locals.var_t5dg02 * locals.var_t5dg02))) - (((locals.var_t5ng12_dn19 * locals.var_t5dg12) - (locals.var_t5ng12 * locals.var_t5dg12_dn19)) / (locals.var_t5dg12 * locals.var_t5dg12))), ((-(((locals.var_t5ng02_dn20 * locals.var_t5dg02) - (locals.var_t5ng02 * locals.var_t5dg02_dn20)) / (locals.var_t5dg02 * locals.var_t5dg02))) - (((locals.var_t5ng12_dn20 * locals.var_t5dg12) - (locals.var_t5ng12 * locals.var_t5dg12_dn20)) / (locals.var_t5dg12 * locals.var_t5dg12))), ((-(((locals.var_t5ng02_dn21 * locals.var_t5dg02) - (locals.var_t5ng02 * locals.var_t5dg02_dn21)) / (locals.var_t5dg02 * locals.var_t5dg02))) - (((locals.var_t5ng12_dn21 * locals.var_t5dg12) - (locals.var_t5ng12 * locals.var_t5dg12_dn21)) / (locals.var_t5dg12 * locals.var_t5dg12))), ((-(((locals.var_t5ng02_dn22 * locals.var_t5dg02) - (locals.var_t5ng02 * locals.var_t5dg02_dn22)) / (locals.var_t5dg02 * locals.var_t5dg02))) - (((locals.var_t5ng12_dn22 * locals.var_t5dg12) - (locals.var_t5ng12 * locals.var_t5dg12_dn22)) / (locals.var_t5dg12 * locals.var_t5dg12))),)
    } else {
        (locals.var_t52, locals.var_t52_dn0, locals.var_t52_dn1, locals.var_t52_dn2, locals.var_t52_dn3, locals.var_t52_dn4, locals.var_t52_dn5, locals.var_t52_dn6, locals.var_t52_dn7, locals.var_t52_dn8, locals.var_t52_dn9, locals.var_t52_dn12, locals.var_t52_dn14, locals.var_t52_dn15, locals.var_t52_dn16, locals.var_t52_dn17, locals.var_t52_dn18, locals.var_t52_dn19, locals.var_t52_dn20, locals.var_t52_dn21, locals.var_t52_dn22,)
    }
};
        locals.var_t52 = assign30640_e48466;
        locals.var_t52_dn0 = assign30640_e48466_d_n0;
        locals.var_t52_dn1 = assign30640_e48466_d_n1;
        locals.var_t52_dn2 = assign30640_e48466_d_n2;
        locals.var_t52_dn3 = assign30640_e48466_d_n3;
        locals.var_t52_dn4 = assign30640_e48466_d_n4;
        locals.var_t52_dn5 = assign30640_e48466_d_n5;
        locals.var_t52_dn6 = assign30640_e48466_d_n6;
        locals.var_t52_dn7 = assign30640_e48466_d_n7;
        locals.var_t52_dn8 = assign30640_e48466_d_n8;
        locals.var_t52_dn9 = assign30640_e48466_d_n9;
        locals.var_t52_dn12 = assign30640_e48466_d_n12;
        locals.var_t52_dn14 = assign30640_e48466_d_n14;
        locals.var_t52_dn15 = assign30640_e48466_d_n15;
        locals.var_t52_dn16 = assign30640_e48466_d_n16;
        locals.var_t52_dn17 = assign30640_e48466_d_n17;
        locals.var_t52_dn18 = assign30640_e48466_d_n18;
        locals.var_t52_dn19 = assign30640_e48466_d_n19;
        locals.var_t52_dn20 = assign30640_e48466_d_n20;
        locals.var_t52_dn21 = assign30640_e48466_d_n21;
        locals.var_t52_dn22 = assign30640_e48466_d_n22;

        let (assign30650_e48479, assign30650_e48479_d_n0, assign30650_e48479_d_n1, assign30650_e48479_d_n2, assign30650_e48479_d_n3, assign30650_e48479_d_n4, assign30650_e48479_d_n5, assign30650_e48479_d_n6, assign30650_e48479_d_n7, assign30650_e48479_d_n8, assign30650_e48479_d_n9, assign30650_e48479_d_n12, assign30650_e48479_d_n14, assign30650_e48479_d_n15, assign30650_e48479_d_n16, assign30650_e48479_d_n17, assign30650_e48479_d_n18, assign30650_e48479_d_n19, assign30650_e48479_d_n20, assign30650_e48479_d_n21, assign30650_e48479_d_n22,) = {
    if (((locals.var_guard504 == 0.0) && (locals.var_guard513 != 0.0)) && (locals.var_guard518 != 0.0)) {
        let assign30650_e48476: f64 = (locals.var_t42 / locals.var_t52);
        let assign30650_e48477: f64 = (locals.var_ef2 - assign30650_e48476);
        (assign30650_e48477, (locals.var_ef2_dn0 - (((locals.var_t42_dn0 * locals.var_t52) - (locals.var_t42 * locals.var_t52_dn0)) / (locals.var_t52 * locals.var_t52))), (locals.var_ef2_dn1 - (((locals.var_t42_dn1 * locals.var_t52) - (locals.var_t42 * locals.var_t52_dn1)) / (locals.var_t52 * locals.var_t52))), (locals.var_ef2_dn2 - (((locals.var_t42_dn2 * locals.var_t52) - (locals.var_t42 * locals.var_t52_dn2)) / (locals.var_t52 * locals.var_t52))), (locals.var_ef2_dn3 - (((locals.var_t42_dn3 * locals.var_t52) - (locals.var_t42 * locals.var_t52_dn3)) / (locals.var_t52 * locals.var_t52))), (locals.var_ef2_dn4 - (((locals.var_t42_dn4 * locals.var_t52) - (locals.var_t42 * locals.var_t52_dn4)) / (locals.var_t52 * locals.var_t52))), (locals.var_ef2_dn5 - (((locals.var_t42_dn5 * locals.var_t52) - (locals.var_t42 * locals.var_t52_dn5)) / (locals.var_t52 * locals.var_t52))), (locals.var_ef2_dn6 - (((locals.var_t42_dn6 * locals.var_t52) - (locals.var_t42 * locals.var_t52_dn6)) / (locals.var_t52 * locals.var_t52))), (locals.var_ef2_dn7 - (((locals.var_t42_dn7 * locals.var_t52) - (locals.var_t42 * locals.var_t52_dn7)) / (locals.var_t52 * locals.var_t52))), (locals.var_ef2_dn8 - (((locals.var_t42_dn8 * locals.var_t52) - (locals.var_t42 * locals.var_t52_dn8)) / (locals.var_t52 * locals.var_t52))), (locals.var_ef2_dn9 - (((locals.var_t42_dn9 * locals.var_t52) - (locals.var_t42 * locals.var_t52_dn9)) / (locals.var_t52 * locals.var_t52))), (locals.var_ef2_dn12 - (((locals.var_t42_dn12 * locals.var_t52) - (locals.var_t42 * locals.var_t52_dn12)) / (locals.var_t52 * locals.var_t52))), (locals.var_ef2_dn14 - (((locals.var_t42_dn14 * locals.var_t52) - (locals.var_t42 * locals.var_t52_dn14)) / (locals.var_t52 * locals.var_t52))), (locals.var_ef2_dn15 - (((locals.var_t42_dn15 * locals.var_t52) - (locals.var_t42 * locals.var_t52_dn15)) / (locals.var_t52 * locals.var_t52))), (locals.var_ef2_dn16 - (((locals.var_t42_dn16 * locals.var_t52) - (locals.var_t42 * locals.var_t52_dn16)) / (locals.var_t52 * locals.var_t52))), (locals.var_ef2_dn17 - (((locals.var_t42_dn17 * locals.var_t52) - (locals.var_t42 * locals.var_t52_dn17)) / (locals.var_t52 * locals.var_t52))), (locals.var_ef2_dn18 - (((locals.var_t42_dn18 * locals.var_t52) - (locals.var_t42 * locals.var_t52_dn18)) / (locals.var_t52 * locals.var_t52))), (locals.var_ef2_dn19 - (((locals.var_t42_dn19 * locals.var_t52) - (locals.var_t42 * locals.var_t52_dn19)) / (locals.var_t52 * locals.var_t52))), (locals.var_ef2_dn20 - (((locals.var_t42_dn20 * locals.var_t52) - (locals.var_t42 * locals.var_t52_dn20)) / (locals.var_t52 * locals.var_t52))), (locals.var_ef2_dn21 - (((locals.var_t42_dn21 * locals.var_t52) - (locals.var_t42 * locals.var_t52_dn21)) / (locals.var_t52 * locals.var_t52))), (locals.var_ef2_dn22 - (((locals.var_t42_dn22 * locals.var_t52) - (locals.var_t42 * locals.var_t52_dn22)) / (locals.var_t52 * locals.var_t52))),)
    } else {
        (locals.var_ef3, locals.var_ef3_dn0, locals.var_ef3_dn1, locals.var_ef3_dn2, locals.var_ef3_dn3, locals.var_ef3_dn4, locals.var_ef3_dn5, locals.var_ef3_dn6, locals.var_ef3_dn7, locals.var_ef3_dn8, locals.var_ef3_dn9, locals.var_ef3_dn12, locals.var_ef3_dn14, locals.var_ef3_dn15, locals.var_ef3_dn16, locals.var_ef3_dn17, locals.var_ef3_dn18, locals.var_ef3_dn19, locals.var_ef3_dn20, locals.var_ef3_dn21, locals.var_ef3_dn22,)
    }
};
        locals.var_ef3 = assign30650_e48479;
        locals.var_ef3_dn0 = assign30650_e48479_d_n0;
        locals.var_ef3_dn1 = assign30650_e48479_d_n1;
        locals.var_ef3_dn2 = assign30650_e48479_d_n2;
        locals.var_ef3_dn3 = assign30650_e48479_d_n3;
        locals.var_ef3_dn4 = assign30650_e48479_d_n4;
        locals.var_ef3_dn5 = assign30650_e48479_d_n5;
        locals.var_ef3_dn6 = assign30650_e48479_d_n6;
        locals.var_ef3_dn7 = assign30650_e48479_d_n7;
        locals.var_ef3_dn8 = assign30650_e48479_d_n8;
        locals.var_ef3_dn9 = assign30650_e48479_d_n9;
        locals.var_ef3_dn12 = assign30650_e48479_d_n12;
        locals.var_ef3_dn14 = assign30650_e48479_d_n14;
        locals.var_ef3_dn15 = assign30650_e48479_d_n15;
        locals.var_ef3_dn16 = assign30650_e48479_d_n16;
        locals.var_ef3_dn17 = assign30650_e48479_d_n17;
        locals.var_ef3_dn18 = assign30650_e48479_d_n18;
        locals.var_ef3_dn19 = assign30650_e48479_d_n19;
        locals.var_ef3_dn20 = assign30650_e48479_d_n20;
        locals.var_ef3_dn21 = assign30650_e48479_d_n21;
        locals.var_ef3_dn22 = assign30650_e48479_d_n22;

        let (assign30660_e48490, assign30660_e48490_d_n0, assign30660_e48490_d_n1, assign30660_e48490_d_n2, assign30660_e48490_d_n3, assign30660_e48490_d_n4, assign30660_e48490_d_n5, assign30660_e48490_d_n6, assign30660_e48490_d_n7, assign30660_e48490_d_n8, assign30660_e48490_d_n9, assign30660_e48490_d_n12, assign30660_e48490_d_n14, assign30660_e48490_d_n15, assign30660_e48490_d_n16, assign30660_e48490_d_n17, assign30660_e48490_d_n18, assign30660_e48490_d_n19, assign30660_e48490_d_n20, assign30660_e48490_d_n21, assign30660_e48490_d_n22,) = {
    if (((locals.var_guard504 == 0.0) && (locals.var_guard513 != 0.0)) && (locals.var_guard518 != 0.0)) {
        let assign30660_e48488: f64 = (locals.var_ef3 + locals.var_vdeff);
        (assign30660_e48488, (locals.var_ef3_dn0 + locals.var_vdeff_dn0), (locals.var_ef3_dn1 + locals.var_vdeff_dn1), (locals.var_ef3_dn2 + locals.var_vdeff_dn2), (locals.var_ef3_dn3 + locals.var_vdeff_dn3), (locals.var_ef3_dn4 + locals.var_vdeff_dn4), (locals.var_ef3_dn5 + locals.var_vdeff_dn5), (locals.var_ef3_dn6 + locals.var_vdeff_dn6), (locals.var_ef3_dn7 + locals.var_vdeff_dn7), (locals.var_ef3_dn8 + locals.var_vdeff_dn8), (locals.var_ef3_dn9 + locals.var_vdeff_dn9), (locals.var_ef3_dn12 + locals.var_vdeff_dn12), (locals.var_ef3_dn14 + locals.var_vdeff_dn14), (locals.var_ef3_dn15 + locals.var_vdeff_dn15), (locals.var_ef3_dn16 + locals.var_vdeff_dn16), (locals.var_ef3_dn17 + locals.var_vdeff_dn17), (locals.var_ef3_dn18 + locals.var_vdeff_dn18), (locals.var_ef3_dn19 + locals.var_vdeff_dn19), (locals.var_ef3_dn20 + locals.var_vdeff_dn20), (locals.var_ef3_dn21 + locals.var_vdeff_dn21), (locals.var_ef3_dn22 + locals.var_vdeff_dn22),)
    } else {
        (locals.var_psid_fp4s, locals.var_psid_fp4s_dn0, locals.var_psid_fp4s_dn1, locals.var_psid_fp4s_dn2, locals.var_psid_fp4s_dn3, locals.var_psid_fp4s_dn4, locals.var_psid_fp4s_dn5, locals.var_psid_fp4s_dn6, locals.var_psid_fp4s_dn7, locals.var_psid_fp4s_dn8, locals.var_psid_fp4s_dn9, locals.var_psid_fp4s_dn12, locals.var_psid_fp4s_dn14, locals.var_psid_fp4s_dn15, locals.var_psid_fp4s_dn16, locals.var_psid_fp4s_dn17, locals.var_psid_fp4s_dn18, locals.var_psid_fp4s_dn19, locals.var_psid_fp4s_dn20, locals.var_psid_fp4s_dn21, locals.var_psid_fp4s_dn22,)
    }
};
        locals.var_psid_fp4s = assign30660_e48490;
        locals.var_psid_fp4s_dn0 = assign30660_e48490_d_n0;
        locals.var_psid_fp4s_dn1 = assign30660_e48490_d_n1;
        locals.var_psid_fp4s_dn2 = assign30660_e48490_d_n2;
        locals.var_psid_fp4s_dn3 = assign30660_e48490_d_n3;
        locals.var_psid_fp4s_dn4 = assign30660_e48490_d_n4;
        locals.var_psid_fp4s_dn5 = assign30660_e48490_d_n5;
        locals.var_psid_fp4s_dn6 = assign30660_e48490_d_n6;
        locals.var_psid_fp4s_dn7 = assign30660_e48490_d_n7;
        locals.var_psid_fp4s_dn8 = assign30660_e48490_d_n8;
        locals.var_psid_fp4s_dn9 = assign30660_e48490_d_n9;
        locals.var_psid_fp4s_dn12 = assign30660_e48490_d_n12;
        locals.var_psid_fp4s_dn14 = assign30660_e48490_d_n14;
        locals.var_psid_fp4s_dn15 = assign30660_e48490_d_n15;
        locals.var_psid_fp4s_dn16 = assign30660_e48490_d_n16;
        locals.var_psid_fp4s_dn17 = assign30660_e48490_d_n17;
        locals.var_psid_fp4s_dn18 = assign30660_e48490_d_n18;
        locals.var_psid_fp4s_dn19 = assign30660_e48490_d_n19;
        locals.var_psid_fp4s_dn20 = assign30660_e48490_d_n20;
        locals.var_psid_fp4s_dn21 = assign30660_e48490_d_n21;
        locals.var_psid_fp4s_dn22 = assign30660_e48490_d_n22;

        let (assign30670_e48502, assign30670_e48502_d_n0, assign30670_e48502_d_n1, assign30670_e48502_d_n2, assign30670_e48502_d_n3, assign30670_e48502_d_n4, assign30670_e48502_d_n5, assign30670_e48502_d_n6, assign30670_e48502_d_n7, assign30670_e48502_d_n8, assign30670_e48502_d_n9, assign30670_e48502_d_n12, assign30670_e48502_d_n14, assign30670_e48502_d_n15, assign30670_e48502_d_n16, assign30670_e48502_d_n17, assign30670_e48502_d_n18, assign30670_e48502_d_n19, assign30670_e48502_d_n20, assign30670_e48502_d_n21, assign30670_e48502_d_n22,) = {
    if (((locals.var_guard504 == 0.0) && (locals.var_guard513 != 0.0)) && (locals.var_guard518 == 0.0)) {
        let assign30670_e48500: f64 = (locals.var_ef1 + locals.var_vdeff);
        (assign30670_e48500, (locals.var_ef1_dn0 + locals.var_vdeff_dn0), (locals.var_ef1_dn1 + locals.var_vdeff_dn1), (locals.var_ef1_dn2 + locals.var_vdeff_dn2), (locals.var_ef1_dn3 + locals.var_vdeff_dn3), (locals.var_ef1_dn4 + locals.var_vdeff_dn4), (locals.var_ef1_dn5 + locals.var_vdeff_dn5), (locals.var_ef1_dn6 + locals.var_vdeff_dn6), (locals.var_ef1_dn7 + locals.var_vdeff_dn7), (locals.var_ef1_dn8 + locals.var_vdeff_dn8), (locals.var_ef1_dn9 + locals.var_vdeff_dn9), (locals.var_ef1_dn12 + locals.var_vdeff_dn12), (locals.var_ef1_dn14 + locals.var_vdeff_dn14), (locals.var_ef1_dn15 + locals.var_vdeff_dn15), (locals.var_ef1_dn16 + locals.var_vdeff_dn16), (locals.var_ef1_dn17 + locals.var_vdeff_dn17), (locals.var_ef1_dn18 + locals.var_vdeff_dn18), (locals.var_ef1_dn19 + locals.var_vdeff_dn19), (locals.var_ef1_dn20 + locals.var_vdeff_dn20), (locals.var_ef1_dn21 + locals.var_vdeff_dn21), (locals.var_ef1_dn22 + locals.var_vdeff_dn22),)
    } else {
        (locals.var_psid_fp4s, locals.var_psid_fp4s_dn0, locals.var_psid_fp4s_dn1, locals.var_psid_fp4s_dn2, locals.var_psid_fp4s_dn3, locals.var_psid_fp4s_dn4, locals.var_psid_fp4s_dn5, locals.var_psid_fp4s_dn6, locals.var_psid_fp4s_dn7, locals.var_psid_fp4s_dn8, locals.var_psid_fp4s_dn9, locals.var_psid_fp4s_dn12, locals.var_psid_fp4s_dn14, locals.var_psid_fp4s_dn15, locals.var_psid_fp4s_dn16, locals.var_psid_fp4s_dn17, locals.var_psid_fp4s_dn18, locals.var_psid_fp4s_dn19, locals.var_psid_fp4s_dn20, locals.var_psid_fp4s_dn21, locals.var_psid_fp4s_dn22,)
    }
};
        locals.var_psid_fp4s = assign30670_e48502;
        locals.var_psid_fp4s_dn0 = assign30670_e48502_d_n0;
        locals.var_psid_fp4s_dn1 = assign30670_e48502_d_n1;
        locals.var_psid_fp4s_dn2 = assign30670_e48502_d_n2;
        locals.var_psid_fp4s_dn3 = assign30670_e48502_d_n3;
        locals.var_psid_fp4s_dn4 = assign30670_e48502_d_n4;
        locals.var_psid_fp4s_dn5 = assign30670_e48502_d_n5;
        locals.var_psid_fp4s_dn6 = assign30670_e48502_d_n6;
        locals.var_psid_fp4s_dn7 = assign30670_e48502_d_n7;
        locals.var_psid_fp4s_dn8 = assign30670_e48502_d_n8;
        locals.var_psid_fp4s_dn9 = assign30670_e48502_d_n9;
        locals.var_psid_fp4s_dn12 = assign30670_e48502_d_n12;
        locals.var_psid_fp4s_dn14 = assign30670_e48502_d_n14;
        locals.var_psid_fp4s_dn15 = assign30670_e48502_d_n15;
        locals.var_psid_fp4s_dn16 = assign30670_e48502_d_n16;
        locals.var_psid_fp4s_dn17 = assign30670_e48502_d_n17;
        locals.var_psid_fp4s_dn18 = assign30670_e48502_d_n18;
        locals.var_psid_fp4s_dn19 = assign30670_e48502_d_n19;
        locals.var_psid_fp4s_dn20 = assign30670_e48502_d_n20;
        locals.var_psid_fp4s_dn21 = assign30670_e48502_d_n21;
        locals.var_psid_fp4s_dn22 = assign30670_e48502_d_n22;

        let (assign30680_e48513, assign30680_e48513_d_n0, assign30680_e48513_d_n1, assign30680_e48513_d_n2, assign30680_e48513_d_n3, assign30680_e48513_d_n4, assign30680_e48513_d_n5, assign30680_e48513_d_n6, assign30680_e48513_d_n7, assign30680_e48513_d_n8, assign30680_e48513_d_n9, assign30680_e48513_d_n12, assign30680_e48513_d_n14, assign30680_e48513_d_n15, assign30680_e48513_d_n16, assign30680_e48513_d_n17, assign30680_e48513_d_n18, assign30680_e48513_d_n19, assign30680_e48513_d_n20, assign30680_e48513_d_n21, assign30680_e48513_d_n22,) = {
    if ((locals.var_guard504 == 0.0) && (locals.var_guard513 != 0.0)) {
        let assign30680_e48510: f64 = (locals.var_psis_fp4s + locals.var_psid_fp4s);
        let assign30680_e48511: f64 = (0.5 * assign30680_e48510);
        (assign30680_e48511, (0.5 * (locals.var_psis_fp4s_dn0 + locals.var_psid_fp4s_dn0)), (0.5 * (locals.var_psis_fp4s_dn1 + locals.var_psid_fp4s_dn1)), (0.5 * (locals.var_psis_fp4s_dn2 + locals.var_psid_fp4s_dn2)), (0.5 * (locals.var_psis_fp4s_dn3 + locals.var_psid_fp4s_dn3)), (0.5 * (locals.var_psis_fp4s_dn4 + locals.var_psid_fp4s_dn4)), (0.5 * (locals.var_psis_fp4s_dn5 + locals.var_psid_fp4s_dn5)), (0.5 * (locals.var_psis_fp4s_dn6 + locals.var_psid_fp4s_dn6)), (0.5 * (locals.var_psis_fp4s_dn7 + locals.var_psid_fp4s_dn7)), (0.5 * (locals.var_psis_fp4s_dn8 + locals.var_psid_fp4s_dn8)), (0.5 * (locals.var_psis_fp4s_dn9 + locals.var_psid_fp4s_dn9)), (0.5 * (locals.var_psis_fp4s_dn12 + locals.var_psid_fp4s_dn12)), (0.5 * (locals.var_psis_fp4s_dn14 + locals.var_psid_fp4s_dn14)), (0.5 * (locals.var_psis_fp4s_dn15 + locals.var_psid_fp4s_dn15)), (0.5 * (locals.var_psis_fp4s_dn16 + locals.var_psid_fp4s_dn16)), (0.5 * (locals.var_psis_fp4s_dn17 + locals.var_psid_fp4s_dn17)), (0.5 * (locals.var_psis_fp4s_dn18 + locals.var_psid_fp4s_dn18)), (0.5 * (locals.var_psis_fp4s_dn19 + locals.var_psid_fp4s_dn19)), (0.5 * (locals.var_psis_fp4s_dn20 + locals.var_psid_fp4s_dn20)), (0.5 * (locals.var_psis_fp4s_dn21 + locals.var_psid_fp4s_dn21)), (0.5 * (locals.var_psis_fp4s_dn22 + locals.var_psid_fp4s_dn22)),)
    } else {
        (locals.var_psim_fp4s, locals.var_psim_fp4s_dn0, locals.var_psim_fp4s_dn1, locals.var_psim_fp4s_dn2, locals.var_psim_fp4s_dn3, locals.var_psim_fp4s_dn4, locals.var_psim_fp4s_dn5, locals.var_psim_fp4s_dn6, locals.var_psim_fp4s_dn7, locals.var_psim_fp4s_dn8, locals.var_psim_fp4s_dn9, locals.var_psim_fp4s_dn12, locals.var_psim_fp4s_dn14, locals.var_psim_fp4s_dn15, locals.var_psim_fp4s_dn16, locals.var_psim_fp4s_dn17, locals.var_psim_fp4s_dn18, locals.var_psim_fp4s_dn19, locals.var_psim_fp4s_dn20, locals.var_psim_fp4s_dn21, locals.var_psim_fp4s_dn22,)
    }
};
        locals.var_psim_fp4s = assign30680_e48513;
        locals.var_psim_fp4s_dn0 = assign30680_e48513_d_n0;
        locals.var_psim_fp4s_dn1 = assign30680_e48513_d_n1;
        locals.var_psim_fp4s_dn2 = assign30680_e48513_d_n2;
        locals.var_psim_fp4s_dn3 = assign30680_e48513_d_n3;
        locals.var_psim_fp4s_dn4 = assign30680_e48513_d_n4;
        locals.var_psim_fp4s_dn5 = assign30680_e48513_d_n5;
        locals.var_psim_fp4s_dn6 = assign30680_e48513_d_n6;
        locals.var_psim_fp4s_dn7 = assign30680_e48513_d_n7;
        locals.var_psim_fp4s_dn8 = assign30680_e48513_d_n8;
        locals.var_psim_fp4s_dn9 = assign30680_e48513_d_n9;
        locals.var_psim_fp4s_dn12 = assign30680_e48513_d_n12;
        locals.var_psim_fp4s_dn14 = assign30680_e48513_d_n14;
        locals.var_psim_fp4s_dn15 = assign30680_e48513_d_n15;
        locals.var_psim_fp4s_dn16 = assign30680_e48513_d_n16;
        locals.var_psim_fp4s_dn17 = assign30680_e48513_d_n17;
        locals.var_psim_fp4s_dn18 = assign30680_e48513_d_n18;
        locals.var_psim_fp4s_dn19 = assign30680_e48513_d_n19;
        locals.var_psim_fp4s_dn20 = assign30680_e48513_d_n20;
        locals.var_psim_fp4s_dn21 = assign30680_e48513_d_n21;
        locals.var_psim_fp4s_dn22 = assign30680_e48513_d_n22;

        let (assign30690_e48522, assign30690_e48522_d_n0, assign30690_e48522_d_n1, assign30690_e48522_d_n2, assign30690_e48522_d_n3, assign30690_e48522_d_n4, assign30690_e48522_d_n5, assign30690_e48522_d_n6, assign30690_e48522_d_n7, assign30690_e48522_d_n8, assign30690_e48522_d_n9, assign30690_e48522_d_n12, assign30690_e48522_d_n14, assign30690_e48522_d_n15, assign30690_e48522_d_n16, assign30690_e48522_d_n17, assign30690_e48522_d_n18, assign30690_e48522_d_n19, assign30690_e48522_d_n20, assign30690_e48522_d_n21, assign30690_e48522_d_n22,) = {
    if ((locals.var_guard504 == 0.0) && (locals.var_guard513 != 0.0)) {
        let assign30690_e48520: f64 = (locals.var_psid_fp4s - locals.var_psis_fp4s);
        (assign30690_e48520, (locals.var_psid_fp4s_dn0 - locals.var_psis_fp4s_dn0), (locals.var_psid_fp4s_dn1 - locals.var_psis_fp4s_dn1), (locals.var_psid_fp4s_dn2 - locals.var_psis_fp4s_dn2), (locals.var_psid_fp4s_dn3 - locals.var_psis_fp4s_dn3), (locals.var_psid_fp4s_dn4 - locals.var_psis_fp4s_dn4), (locals.var_psid_fp4s_dn5 - locals.var_psis_fp4s_dn5), (locals.var_psid_fp4s_dn6 - locals.var_psis_fp4s_dn6), (locals.var_psid_fp4s_dn7 - locals.var_psis_fp4s_dn7), (locals.var_psid_fp4s_dn8 - locals.var_psis_fp4s_dn8), (locals.var_psid_fp4s_dn9 - locals.var_psis_fp4s_dn9), (locals.var_psid_fp4s_dn12 - locals.var_psis_fp4s_dn12), (locals.var_psid_fp4s_dn14 - locals.var_psis_fp4s_dn14), (locals.var_psid_fp4s_dn15 - locals.var_psis_fp4s_dn15), (locals.var_psid_fp4s_dn16 - locals.var_psis_fp4s_dn16), (locals.var_psid_fp4s_dn17 - locals.var_psis_fp4s_dn17), (locals.var_psid_fp4s_dn18 - locals.var_psis_fp4s_dn18), (locals.var_psid_fp4s_dn19 - locals.var_psis_fp4s_dn19), (locals.var_psid_fp4s_dn20 - locals.var_psis_fp4s_dn20), (locals.var_psid_fp4s_dn21 - locals.var_psis_fp4s_dn21), (locals.var_psid_fp4s_dn22 - locals.var_psis_fp4s_dn22),)
    } else {
        (locals.var_psisd_fp4s, locals.var_psisd_fp4s_dn0, locals.var_psisd_fp4s_dn1, locals.var_psisd_fp4s_dn2, locals.var_psisd_fp4s_dn3, locals.var_psisd_fp4s_dn4, locals.var_psisd_fp4s_dn5, locals.var_psisd_fp4s_dn6, locals.var_psisd_fp4s_dn7, locals.var_psisd_fp4s_dn8, locals.var_psisd_fp4s_dn9, locals.var_psisd_fp4s_dn12, locals.var_psisd_fp4s_dn14, locals.var_psisd_fp4s_dn15, locals.var_psisd_fp4s_dn16, locals.var_psisd_fp4s_dn17, locals.var_psisd_fp4s_dn18, locals.var_psisd_fp4s_dn19, locals.var_psisd_fp4s_dn20, locals.var_psisd_fp4s_dn21, locals.var_psisd_fp4s_dn22,)
    }
};
        locals.var_psisd_fp4s = assign30690_e48522;
        locals.var_psisd_fp4s_dn0 = assign30690_e48522_d_n0;
        locals.var_psisd_fp4s_dn1 = assign30690_e48522_d_n1;
        locals.var_psisd_fp4s_dn2 = assign30690_e48522_d_n2;
        locals.var_psisd_fp4s_dn3 = assign30690_e48522_d_n3;
        locals.var_psisd_fp4s_dn4 = assign30690_e48522_d_n4;
        locals.var_psisd_fp4s_dn5 = assign30690_e48522_d_n5;
        locals.var_psisd_fp4s_dn6 = assign30690_e48522_d_n6;
        locals.var_psisd_fp4s_dn7 = assign30690_e48522_d_n7;
        locals.var_psisd_fp4s_dn8 = assign30690_e48522_d_n8;
        locals.var_psisd_fp4s_dn9 = assign30690_e48522_d_n9;
        locals.var_psisd_fp4s_dn12 = assign30690_e48522_d_n12;
        locals.var_psisd_fp4s_dn14 = assign30690_e48522_d_n14;
        locals.var_psisd_fp4s_dn15 = assign30690_e48522_d_n15;
        locals.var_psisd_fp4s_dn16 = assign30690_e48522_d_n16;
        locals.var_psisd_fp4s_dn17 = assign30690_e48522_d_n17;
        locals.var_psisd_fp4s_dn18 = assign30690_e48522_d_n18;
        locals.var_psisd_fp4s_dn19 = assign30690_e48522_d_n19;
        locals.var_psisd_fp4s_dn20 = assign30690_e48522_d_n20;
        locals.var_psisd_fp4s_dn21 = assign30690_e48522_d_n21;
        locals.var_psisd_fp4s_dn22 = assign30690_e48522_d_n22;

        let (assign30700_e48531, assign30700_e48531_d_n0, assign30700_e48531_d_n1, assign30700_e48531_d_n2, assign30700_e48531_d_n3, assign30700_e48531_d_n4, assign30700_e48531_d_n5, assign30700_e48531_d_n6, assign30700_e48531_d_n7, assign30700_e48531_d_n8, assign30700_e48531_d_n9, assign30700_e48531_d_n12, assign30700_e48531_d_n14, assign30700_e48531_d_n15, assign30700_e48531_d_n16, assign30700_e48531_d_n17, assign30700_e48531_d_n18, assign30700_e48531_d_n19, assign30700_e48531_d_n20, assign30700_e48531_d_n21, assign30700_e48531_d_n22,) = {
    if ((locals.var_guard504 == 0.0) && (locals.var_guard513 != 0.0)) {
        let assign30700_e48529: f64 = (locals.var_psid_fp4s - locals.var_psis_fp4s);
        (assign30700_e48529, (locals.var_psid_fp4s_dn0 - locals.var_psis_fp4s_dn0), (locals.var_psid_fp4s_dn1 - locals.var_psis_fp4s_dn1), (locals.var_psid_fp4s_dn2 - locals.var_psis_fp4s_dn2), (locals.var_psid_fp4s_dn3 - locals.var_psis_fp4s_dn3), (locals.var_psid_fp4s_dn4 - locals.var_psis_fp4s_dn4), (locals.var_psid_fp4s_dn5 - locals.var_psis_fp4s_dn5), (locals.var_psid_fp4s_dn6 - locals.var_psis_fp4s_dn6), (locals.var_psid_fp4s_dn7 - locals.var_psis_fp4s_dn7), (locals.var_psid_fp4s_dn8 - locals.var_psis_fp4s_dn8), (locals.var_psid_fp4s_dn9 - locals.var_psis_fp4s_dn9), (locals.var_psid_fp4s_dn12 - locals.var_psis_fp4s_dn12), (locals.var_psid_fp4s_dn14 - locals.var_psis_fp4s_dn14), (locals.var_psid_fp4s_dn15 - locals.var_psis_fp4s_dn15), (locals.var_psid_fp4s_dn16 - locals.var_psis_fp4s_dn16), (locals.var_psid_fp4s_dn17 - locals.var_psis_fp4s_dn17), (locals.var_psid_fp4s_dn18 - locals.var_psis_fp4s_dn18), (locals.var_psid_fp4s_dn19 - locals.var_psis_fp4s_dn19), (locals.var_psid_fp4s_dn20 - locals.var_psis_fp4s_dn20), (locals.var_psid_fp4s_dn21 - locals.var_psis_fp4s_dn21), (locals.var_psid_fp4s_dn22 - locals.var_psis_fp4s_dn22),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn1, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn12, locals.var_t1_dn14, locals.var_t1_dn15, locals.var_t1_dn16, locals.var_t1_dn17, locals.var_t1_dn18, locals.var_t1_dn19, locals.var_t1_dn20, locals.var_t1_dn21, locals.var_t1_dn22,)
    }
};
        locals.var_t1 = assign30700_e48531;
        locals.var_t1_dn0 = assign30700_e48531_d_n0;
        locals.var_t1_dn1 = assign30700_e48531_d_n1;
        locals.var_t1_dn2 = assign30700_e48531_d_n2;
        locals.var_t1_dn3 = assign30700_e48531_d_n3;
        locals.var_t1_dn4 = assign30700_e48531_d_n4;
        locals.var_t1_dn5 = assign30700_e48531_d_n5;
        locals.var_t1_dn6 = assign30700_e48531_d_n6;
        locals.var_t1_dn7 = assign30700_e48531_d_n7;
        locals.var_t1_dn8 = assign30700_e48531_d_n8;
        locals.var_t1_dn9 = assign30700_e48531_d_n9;
        locals.var_t1_dn12 = assign30700_e48531_d_n12;
        locals.var_t1_dn14 = assign30700_e48531_d_n14;
        locals.var_t1_dn15 = assign30700_e48531_d_n15;
        locals.var_t1_dn16 = assign30700_e48531_d_n16;
        locals.var_t1_dn17 = assign30700_e48531_d_n17;
        locals.var_t1_dn18 = assign30700_e48531_d_n18;
        locals.var_t1_dn19 = assign30700_e48531_d_n19;
        locals.var_t1_dn20 = assign30700_e48531_d_n20;
        locals.var_t1_dn21 = assign30700_e48531_d_n21;
        locals.var_t1_dn22 = assign30700_e48531_d_n22;

        let (assign30710_e48542, assign30710_e48542_d_n0, assign30710_e48542_d_n1, assign30710_e48542_d_n2, assign30710_e48542_d_n3, assign30710_e48542_d_n4, assign30710_e48542_d_n5, assign30710_e48542_d_n6, assign30710_e48542_d_n7, assign30710_e48542_d_n8, assign30710_e48542_d_n9, assign30710_e48542_d_n12, assign30710_e48542_d_n14, assign30710_e48542_d_n15, assign30710_e48542_d_n16, assign30710_e48542_d_n17, assign30710_e48542_d_n18, assign30710_e48542_d_n19, assign30710_e48542_d_n20, assign30710_e48542_d_n21, assign30710_e48542_d_n22,) = {
    if ((locals.var_guard504 == 0.0) && (locals.var_guard513 != 0.0)) {
        let assign30710_e48538: f64 = (locals.var_vg0_fp4s + locals.var_vtv);
        let assign30710_e48540: f64 = (assign30710_e48538 - locals.var_psim_fp4s);
        (assign30710_e48540, (locals.var_vg0_fp4s_dn0 - locals.var_psim_fp4s_dn0), (locals.var_vg0_fp4s_dn1 - locals.var_psim_fp4s_dn1), (locals.var_vg0_fp4s_dn2 - locals.var_psim_fp4s_dn2), (locals.var_vg0_fp4s_dn3 - locals.var_psim_fp4s_dn3), ((locals.var_vg0_fp4s_dn4 + locals.var_vtv_dn4) - locals.var_psim_fp4s_dn4), (locals.var_vg0_fp4s_dn5 - locals.var_psim_fp4s_dn5), ((locals.var_vg0_fp4s_dn6 + locals.var_vtv_dn6) - locals.var_psim_fp4s_dn6), ((locals.var_vg0_fp4s_dn7 + locals.var_vtv_dn7) - locals.var_psim_fp4s_dn7), ((locals.var_vg0_fp4s_dn8 + locals.var_vtv_dn8) - locals.var_psim_fp4s_dn8), (locals.var_vg0_fp4s_dn9 - locals.var_psim_fp4s_dn9), (locals.var_vg0_fp4s_dn12 - locals.var_psim_fp4s_dn12), (locals.var_vg0_fp4s_dn14 - locals.var_psim_fp4s_dn14), ((locals.var_vg0_fp4s_dn15 + locals.var_vtv_dn15) - locals.var_psim_fp4s_dn15), ((locals.var_vg0_fp4s_dn16 + locals.var_vtv_dn16) - locals.var_psim_fp4s_dn16), ((locals.var_vg0_fp4s_dn17 + locals.var_vtv_dn17) - locals.var_psim_fp4s_dn17), ((locals.var_vg0_fp4s_dn18 + locals.var_vtv_dn18) - locals.var_psim_fp4s_dn18), ((locals.var_vg0_fp4s_dn19 + locals.var_vtv_dn19) - locals.var_psim_fp4s_dn19), ((locals.var_vg0_fp4s_dn20 + locals.var_vtv_dn20) - locals.var_psim_fp4s_dn20), ((locals.var_vg0_fp4s_dn21 + locals.var_vtv_dn21) - locals.var_psim_fp4s_dn21), ((locals.var_vg0_fp4s_dn22 + locals.var_vtv_dn22) - locals.var_psim_fp4s_dn22),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn1, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn12, locals.var_t2_dn14, locals.var_t2_dn15, locals.var_t2_dn16, locals.var_t2_dn17, locals.var_t2_dn18, locals.var_t2_dn19, locals.var_t2_dn20, locals.var_t2_dn21, locals.var_t2_dn22,)
    }
};
        locals.var_t2 = assign30710_e48542;
        locals.var_t2_dn0 = assign30710_e48542_d_n0;
        locals.var_t2_dn1 = assign30710_e48542_d_n1;
        locals.var_t2_dn2 = assign30710_e48542_d_n2;
        locals.var_t2_dn3 = assign30710_e48542_d_n3;
        locals.var_t2_dn4 = assign30710_e48542_d_n4;
        locals.var_t2_dn5 = assign30710_e48542_d_n5;
        locals.var_t2_dn6 = assign30710_e48542_d_n6;
        locals.var_t2_dn7 = assign30710_e48542_d_n7;
        locals.var_t2_dn8 = assign30710_e48542_d_n8;
        locals.var_t2_dn9 = assign30710_e48542_d_n9;
        locals.var_t2_dn12 = assign30710_e48542_d_n12;
        locals.var_t2_dn14 = assign30710_e48542_d_n14;
        locals.var_t2_dn15 = assign30710_e48542_d_n15;
        locals.var_t2_dn16 = assign30710_e48542_d_n16;
        locals.var_t2_dn17 = assign30710_e48542_d_n17;
        locals.var_t2_dn18 = assign30710_e48542_d_n18;
        locals.var_t2_dn19 = assign30710_e48542_d_n19;
        locals.var_t2_dn20 = assign30710_e48542_d_n20;
        locals.var_t2_dn21 = assign30710_e48542_d_n21;
        locals.var_t2_dn22 = assign30710_e48542_d_n22;

        let (assign30720_e48569, assign30720_e48569_d_n0, assign30720_e48569_d_n1, assign30720_e48569_d_n2, assign30720_e48569_d_n3, assign30720_e48569_d_n4, assign30720_e48569_d_n5, assign30720_e48569_d_n6, assign30720_e48569_d_n7, assign30720_e48569_d_n8, assign30720_e48569_d_n9, assign30720_e48569_d_n12, assign30720_e48569_d_n14, assign30720_e48569_d_n15, assign30720_e48569_d_n16, assign30720_e48569_d_n17, assign30720_e48569_d_n18, assign30720_e48569_d_n19, assign30720_e48569_d_n20, assign30720_e48569_d_n21, assign30720_e48569_d_n22,) = {
    if ((locals.var_guard504 == 0.0) && (locals.var_guard513 != 0.0)) {
        let assign30720_e48549: f64 = (locals.var_cg_fp4s * p.p4);
        let assign30720_e48551: f64 = (assign30720_e48549 * p.p5);
        let assign30720_e48553: f64 = (assign30720_e48551 * p.p200);
        let assign30720_e48556: f64 = (locals.var_vg0_fp4s - locals.var_psim_fp4s);
        let assign30720_e48559: f64 = (0.5 * locals.var_t1);
        let assign30720_e48561: f64 = (assign30720_e48559 * locals.var_t1);
        let assign30720_e48564: f64 = (6.0 * locals.var_t2);
        let assign30720_e48565: f64 = (assign30720_e48561 / assign30720_e48564);
        let assign30720_e48566: f64 = (assign30720_e48556 + assign30720_e48565);
        let assign30720_e48567: f64 = (assign30720_e48553 * assign30720_e48566);
        (assign30720_e48567, (assign30720_e48553 * ((locals.var_vg0_fp4s_dn0 - locals.var_psim_fp4s_dn0) + ((((((0.5 * locals.var_t1_dn0) * locals.var_t1) + (assign30720_e48559 * locals.var_t1_dn0)) * assign30720_e48564) - (assign30720_e48561 * (6.0 * locals.var_t2_dn0))) / (assign30720_e48564 * assign30720_e48564)))), (assign30720_e48553 * ((locals.var_vg0_fp4s_dn1 - locals.var_psim_fp4s_dn1) + ((((((0.5 * locals.var_t1_dn1) * locals.var_t1) + (assign30720_e48559 * locals.var_t1_dn1)) * assign30720_e48564) - (assign30720_e48561 * (6.0 * locals.var_t2_dn1))) / (assign30720_e48564 * assign30720_e48564)))), (assign30720_e48553 * ((locals.var_vg0_fp4s_dn2 - locals.var_psim_fp4s_dn2) + ((((((0.5 * locals.var_t1_dn2) * locals.var_t1) + (assign30720_e48559 * locals.var_t1_dn2)) * assign30720_e48564) - (assign30720_e48561 * (6.0 * locals.var_t2_dn2))) / (assign30720_e48564 * assign30720_e48564)))), (assign30720_e48553 * ((locals.var_vg0_fp4s_dn3 - locals.var_psim_fp4s_dn3) + ((((((0.5 * locals.var_t1_dn3) * locals.var_t1) + (assign30720_e48559 * locals.var_t1_dn3)) * assign30720_e48564) - (assign30720_e48561 * (6.0 * locals.var_t2_dn3))) / (assign30720_e48564 * assign30720_e48564)))), (assign30720_e48553 * ((locals.var_vg0_fp4s_dn4 - locals.var_psim_fp4s_dn4) + ((((((0.5 * locals.var_t1_dn4) * locals.var_t1) + (assign30720_e48559 * locals.var_t1_dn4)) * assign30720_e48564) - (assign30720_e48561 * (6.0 * locals.var_t2_dn4))) / (assign30720_e48564 * assign30720_e48564)))), (assign30720_e48553 * ((locals.var_vg0_fp4s_dn5 - locals.var_psim_fp4s_dn5) + ((((((0.5 * locals.var_t1_dn5) * locals.var_t1) + (assign30720_e48559 * locals.var_t1_dn5)) * assign30720_e48564) - (assign30720_e48561 * (6.0 * locals.var_t2_dn5))) / (assign30720_e48564 * assign30720_e48564)))), (assign30720_e48553 * ((locals.var_vg0_fp4s_dn6 - locals.var_psim_fp4s_dn6) + ((((((0.5 * locals.var_t1_dn6) * locals.var_t1) + (assign30720_e48559 * locals.var_t1_dn6)) * assign30720_e48564) - (assign30720_e48561 * (6.0 * locals.var_t2_dn6))) / (assign30720_e48564 * assign30720_e48564)))), (assign30720_e48553 * ((locals.var_vg0_fp4s_dn7 - locals.var_psim_fp4s_dn7) + ((((((0.5 * locals.var_t1_dn7) * locals.var_t1) + (assign30720_e48559 * locals.var_t1_dn7)) * assign30720_e48564) - (assign30720_e48561 * (6.0 * locals.var_t2_dn7))) / (assign30720_e48564 * assign30720_e48564)))), (assign30720_e48553 * ((locals.var_vg0_fp4s_dn8 - locals.var_psim_fp4s_dn8) + ((((((0.5 * locals.var_t1_dn8) * locals.var_t1) + (assign30720_e48559 * locals.var_t1_dn8)) * assign30720_e48564) - (assign30720_e48561 * (6.0 * locals.var_t2_dn8))) / (assign30720_e48564 * assign30720_e48564)))), (assign30720_e48553 * ((locals.var_vg0_fp4s_dn9 - locals.var_psim_fp4s_dn9) + ((((((0.5 * locals.var_t1_dn9) * locals.var_t1) + (assign30720_e48559 * locals.var_t1_dn9)) * assign30720_e48564) - (assign30720_e48561 * (6.0 * locals.var_t2_dn9))) / (assign30720_e48564 * assign30720_e48564)))), (assign30720_e48553 * ((locals.var_vg0_fp4s_dn12 - locals.var_psim_fp4s_dn12) + ((((((0.5 * locals.var_t1_dn12) * locals.var_t1) + (assign30720_e48559 * locals.var_t1_dn12)) * assign30720_e48564) - (assign30720_e48561 * (6.0 * locals.var_t2_dn12))) / (assign30720_e48564 * assign30720_e48564)))), (assign30720_e48553 * ((locals.var_vg0_fp4s_dn14 - locals.var_psim_fp4s_dn14) + ((((((0.5 * locals.var_t1_dn14) * locals.var_t1) + (assign30720_e48559 * locals.var_t1_dn14)) * assign30720_e48564) - (assign30720_e48561 * (6.0 * locals.var_t2_dn14))) / (assign30720_e48564 * assign30720_e48564)))), (assign30720_e48553 * ((locals.var_vg0_fp4s_dn15 - locals.var_psim_fp4s_dn15) + ((((((0.5 * locals.var_t1_dn15) * locals.var_t1) + (assign30720_e48559 * locals.var_t1_dn15)) * assign30720_e48564) - (assign30720_e48561 * (6.0 * locals.var_t2_dn15))) / (assign30720_e48564 * assign30720_e48564)))), (assign30720_e48553 * ((locals.var_vg0_fp4s_dn16 - locals.var_psim_fp4s_dn16) + ((((((0.5 * locals.var_t1_dn16) * locals.var_t1) + (assign30720_e48559 * locals.var_t1_dn16)) * assign30720_e48564) - (assign30720_e48561 * (6.0 * locals.var_t2_dn16))) / (assign30720_e48564 * assign30720_e48564)))), (assign30720_e48553 * ((locals.var_vg0_fp4s_dn17 - locals.var_psim_fp4s_dn17) + ((((((0.5 * locals.var_t1_dn17) * locals.var_t1) + (assign30720_e48559 * locals.var_t1_dn17)) * assign30720_e48564) - (assign30720_e48561 * (6.0 * locals.var_t2_dn17))) / (assign30720_e48564 * assign30720_e48564)))), (assign30720_e48553 * ((locals.var_vg0_fp4s_dn18 - locals.var_psim_fp4s_dn18) + ((((((0.5 * locals.var_t1_dn18) * locals.var_t1) + (assign30720_e48559 * locals.var_t1_dn18)) * assign30720_e48564) - (assign30720_e48561 * (6.0 * locals.var_t2_dn18))) / (assign30720_e48564 * assign30720_e48564)))), (assign30720_e48553 * ((locals.var_vg0_fp4s_dn19 - locals.var_psim_fp4s_dn19) + ((((((0.5 * locals.var_t1_dn19) * locals.var_t1) + (assign30720_e48559 * locals.var_t1_dn19)) * assign30720_e48564) - (assign30720_e48561 * (6.0 * locals.var_t2_dn19))) / (assign30720_e48564 * assign30720_e48564)))), (assign30720_e48553 * ((locals.var_vg0_fp4s_dn20 - locals.var_psim_fp4s_dn20) + ((((((0.5 * locals.var_t1_dn20) * locals.var_t1) + (assign30720_e48559 * locals.var_t1_dn20)) * assign30720_e48564) - (assign30720_e48561 * (6.0 * locals.var_t2_dn20))) / (assign30720_e48564 * assign30720_e48564)))), (assign30720_e48553 * ((locals.var_vg0_fp4s_dn21 - locals.var_psim_fp4s_dn21) + ((((((0.5 * locals.var_t1_dn21) * locals.var_t1) + (assign30720_e48559 * locals.var_t1_dn21)) * assign30720_e48564) - (assign30720_e48561 * (6.0 * locals.var_t2_dn21))) / (assign30720_e48564 * assign30720_e48564)))), (assign30720_e48553 * ((locals.var_vg0_fp4s_dn22 - locals.var_psim_fp4s_dn22) + ((((((0.5 * locals.var_t1_dn22) * locals.var_t1) + (assign30720_e48559 * locals.var_t1_dn22)) * assign30720_e48564) - (assign30720_e48561 * (6.0 * locals.var_t2_dn22))) / (assign30720_e48564 * assign30720_e48564)))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn1, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn12, locals.var_t3_dn14, locals.var_t3_dn15, locals.var_t3_dn16, locals.var_t3_dn17, locals.var_t3_dn18, locals.var_t3_dn19, locals.var_t3_dn20, locals.var_t3_dn21, locals.var_t3_dn22,)
    }
};
        locals.var_t3 = assign30720_e48569;
        locals.var_t3_dn0 = assign30720_e48569_d_n0;
        locals.var_t3_dn1 = assign30720_e48569_d_n1;
        locals.var_t3_dn2 = assign30720_e48569_d_n2;
        locals.var_t3_dn3 = assign30720_e48569_d_n3;
        locals.var_t3_dn4 = assign30720_e48569_d_n4;
        locals.var_t3_dn5 = assign30720_e48569_d_n5;
        locals.var_t3_dn6 = assign30720_e48569_d_n6;
        locals.var_t3_dn7 = assign30720_e48569_d_n7;
        locals.var_t3_dn8 = assign30720_e48569_d_n8;
        locals.var_t3_dn9 = assign30720_e48569_d_n9;
        locals.var_t3_dn12 = assign30720_e48569_d_n12;
        locals.var_t3_dn14 = assign30720_e48569_d_n14;
        locals.var_t3_dn15 = assign30720_e48569_d_n15;
        locals.var_t3_dn16 = assign30720_e48569_d_n16;
        locals.var_t3_dn17 = assign30720_e48569_d_n17;
        locals.var_t3_dn18 = assign30720_e48569_d_n18;
        locals.var_t3_dn19 = assign30720_e48569_d_n19;
        locals.var_t3_dn20 = assign30720_e48569_d_n20;
        locals.var_t3_dn21 = assign30720_e48569_d_n21;
        locals.var_t3_dn22 = assign30720_e48569_d_n22;

        let (assign30730_e48580, assign30730_e48580_d_n0, assign30730_e48580_d_n1, assign30730_e48580_d_n2, assign30730_e48580_d_n3, assign30730_e48580_d_n4, assign30730_e48580_d_n5, assign30730_e48580_d_n6, assign30730_e48580_d_n7, assign30730_e48580_d_n8, assign30730_e48580_d_n9, assign30730_e48580_d_n12, assign30730_e48580_d_n14, assign30730_e48580_d_n15, assign30730_e48580_d_n16, assign30730_e48580_d_n17, assign30730_e48580_d_n18, assign30730_e48580_d_n19, assign30730_e48580_d_n20, assign30730_e48580_d_n21, assign30730_e48580_d_n22,) = {
    if ((locals.var_guard504 == 0.0) && (locals.var_guard513 != 0.0)) {
        let assign30730_e48577: f64 = (locals.var_t3 / p.p245);
        let assign30730_e48578: f64 = (1e26 * assign30730_e48577);
        (assign30730_e48578, (1e26 * (locals.var_t3_dn0 / p.p245)), (1e26 * (locals.var_t3_dn1 / p.p245)), (1e26 * (locals.var_t3_dn2 / p.p245)), (1e26 * (locals.var_t3_dn3 / p.p245)), (1e26 * (locals.var_t3_dn4 / p.p245)), (1e26 * (locals.var_t3_dn5 / p.p245)), (1e26 * (locals.var_t3_dn6 / p.p245)), (1e26 * (locals.var_t3_dn7 / p.p245)), (1e26 * (locals.var_t3_dn8 / p.p245)), (1e26 * (locals.var_t3_dn9 / p.p245)), (1e26 * (locals.var_t3_dn12 / p.p245)), (1e26 * (locals.var_t3_dn14 / p.p245)), (1e26 * (locals.var_t3_dn15 / p.p245)), (1e26 * (locals.var_t3_dn16 / p.p245)), (1e26 * (locals.var_t3_dn17 / p.p245)), (1e26 * (locals.var_t3_dn18 / p.p245)), (1e26 * (locals.var_t3_dn19 / p.p245)), (1e26 * (locals.var_t3_dn20 / p.p245)), (1e26 * (locals.var_t3_dn21 / p.p245)), (1e26 * (locals.var_t3_dn22 / p.p245)),)
    } else {
        (locals.var_t0_1, locals.var_t0_1_dn0, locals.var_t0_1_dn1, locals.var_t0_1_dn2, locals.var_t0_1_dn3, locals.var_t0_1_dn4, locals.var_t0_1_dn5, locals.var_t0_1_dn6, locals.var_t0_1_dn7, locals.var_t0_1_dn8, locals.var_t0_1_dn9, locals.var_t0_1_dn12, locals.var_t0_1_dn14, locals.var_t0_1_dn15, locals.var_t0_1_dn16, locals.var_t0_1_dn17, locals.var_t0_1_dn18, locals.var_t0_1_dn19, locals.var_t0_1_dn20, locals.var_t0_1_dn21, locals.var_t0_1_dn22,)
    }
};
        locals.var_t0_1 = assign30730_e48580;
        locals.var_t0_1_dn0 = assign30730_e48580_d_n0;
        locals.var_t0_1_dn1 = assign30730_e48580_d_n1;
        locals.var_t0_1_dn2 = assign30730_e48580_d_n2;
        locals.var_t0_1_dn3 = assign30730_e48580_d_n3;
        locals.var_t0_1_dn4 = assign30730_e48580_d_n4;
        locals.var_t0_1_dn5 = assign30730_e48580_d_n5;
        locals.var_t0_1_dn6 = assign30730_e48580_d_n6;
        locals.var_t0_1_dn7 = assign30730_e48580_d_n7;
        locals.var_t0_1_dn8 = assign30730_e48580_d_n8;
        locals.var_t0_1_dn9 = assign30730_e48580_d_n9;
        locals.var_t0_1_dn12 = assign30730_e48580_d_n12;
        locals.var_t0_1_dn14 = assign30730_e48580_d_n14;
        locals.var_t0_1_dn15 = assign30730_e48580_d_n15;
        locals.var_t0_1_dn16 = assign30730_e48580_d_n16;
        locals.var_t0_1_dn17 = assign30730_e48580_d_n17;
        locals.var_t0_1_dn18 = assign30730_e48580_d_n18;
        locals.var_t0_1_dn19 = assign30730_e48580_d_n19;
        locals.var_t0_1_dn20 = assign30730_e48580_d_n20;
        locals.var_t0_1_dn21 = assign30730_e48580_d_n21;
        locals.var_t0_1_dn22 = assign30730_e48580_d_n22;

        let (assign30740_e48591, assign30740_e48591_d_n0, assign30740_e48591_d_n1, assign30740_e48591_d_n2, assign30740_e48591_d_n3, assign30740_e48591_d_n4, assign30740_e48591_d_n5, assign30740_e48591_d_n6, assign30740_e48591_d_n7, assign30740_e48591_d_n8, assign30740_e48591_d_n9, assign30740_e48591_d_n12, assign30740_e48591_d_n14, assign30740_e48591_d_n15, assign30740_e48591_d_n16, assign30740_e48591_d_n17, assign30740_e48591_d_n18, assign30740_e48591_d_n19, assign30740_e48591_d_n20, assign30740_e48591_d_n21, assign30740_e48591_d_n22,) = {
    if ((locals.var_guard504 == 0.0) && (locals.var_guard513 != 0.0)) {
        let assign30740_e48588: f64 = (locals.var_t0_1).powf(p.p244);
        let assign30740_e48589: f64 = (1.0 + assign30740_e48588);
        (assign30740_e48589, if 0.0 == 0.0 && ((p.p244) as f64).is_finite() && ((p.p244) as f64).fract() == 0.0 { if p.p244 == 0.0 { 0.0 } else { (p.p244 * ((locals.var_t0_1).powf(p.p244 - 1.0) * locals.var_t0_1_dn0)) } } else { (assign30740_e48588 * (p.p244 * (locals.var_t0_1_dn0 / locals.var_t0_1))) }, if 0.0 == 0.0 && ((p.p244) as f64).is_finite() && ((p.p244) as f64).fract() == 0.0 { if p.p244 == 0.0 { 0.0 } else { (p.p244 * ((locals.var_t0_1).powf(p.p244 - 1.0) * locals.var_t0_1_dn1)) } } else { (assign30740_e48588 * (p.p244 * (locals.var_t0_1_dn1 / locals.var_t0_1))) }, if 0.0 == 0.0 && ((p.p244) as f64).is_finite() && ((p.p244) as f64).fract() == 0.0 { if p.p244 == 0.0 { 0.0 } else { (p.p244 * ((locals.var_t0_1).powf(p.p244 - 1.0) * locals.var_t0_1_dn2)) } } else { (assign30740_e48588 * (p.p244 * (locals.var_t0_1_dn2 / locals.var_t0_1))) }, if 0.0 == 0.0 && ((p.p244) as f64).is_finite() && ((p.p244) as f64).fract() == 0.0 { if p.p244 == 0.0 { 0.0 } else { (p.p244 * ((locals.var_t0_1).powf(p.p244 - 1.0) * locals.var_t0_1_dn3)) } } else { (assign30740_e48588 * (p.p244 * (locals.var_t0_1_dn3 / locals.var_t0_1))) }, if 0.0 == 0.0 && ((p.p244) as f64).is_finite() && ((p.p244) as f64).fract() == 0.0 { if p.p244 == 0.0 { 0.0 } else { (p.p244 * ((locals.var_t0_1).powf(p.p244 - 1.0) * locals.var_t0_1_dn4)) } } else { (assign30740_e48588 * (p.p244 * (locals.var_t0_1_dn4 / locals.var_t0_1))) }, if 0.0 == 0.0 && ((p.p244) as f64).is_finite() && ((p.p244) as f64).fract() == 0.0 { if p.p244 == 0.0 { 0.0 } else { (p.p244 * ((locals.var_t0_1).powf(p.p244 - 1.0) * locals.var_t0_1_dn5)) } } else { (assign30740_e48588 * (p.p244 * (locals.var_t0_1_dn5 / locals.var_t0_1))) }, if 0.0 == 0.0 && ((p.p244) as f64).is_finite() && ((p.p244) as f64).fract() == 0.0 { if p.p244 == 0.0 { 0.0 } else { (p.p244 * ((locals.var_t0_1).powf(p.p244 - 1.0) * locals.var_t0_1_dn6)) } } else { (assign30740_e48588 * (p.p244 * (locals.var_t0_1_dn6 / locals.var_t0_1))) }, if 0.0 == 0.0 && ((p.p244) as f64).is_finite() && ((p.p244) as f64).fract() == 0.0 { if p.p244 == 0.0 { 0.0 } else { (p.p244 * ((locals.var_t0_1).powf(p.p244 - 1.0) * locals.var_t0_1_dn7)) } } else { (assign30740_e48588 * (p.p244 * (locals.var_t0_1_dn7 / locals.var_t0_1))) }, if 0.0 == 0.0 && ((p.p244) as f64).is_finite() && ((p.p244) as f64).fract() == 0.0 { if p.p244 == 0.0 { 0.0 } else { (p.p244 * ((locals.var_t0_1).powf(p.p244 - 1.0) * locals.var_t0_1_dn8)) } } else { (assign30740_e48588 * (p.p244 * (locals.var_t0_1_dn8 / locals.var_t0_1))) }, if 0.0 == 0.0 && ((p.p244) as f64).is_finite() && ((p.p244) as f64).fract() == 0.0 { if p.p244 == 0.0 { 0.0 } else { (p.p244 * ((locals.var_t0_1).powf(p.p244 - 1.0) * locals.var_t0_1_dn9)) } } else { (assign30740_e48588 * (p.p244 * (locals.var_t0_1_dn9 / locals.var_t0_1))) }, if 0.0 == 0.0 && ((p.p244) as f64).is_finite() && ((p.p244) as f64).fract() == 0.0 { if p.p244 == 0.0 { 0.0 } else { (p.p244 * ((locals.var_t0_1).powf(p.p244 - 1.0) * locals.var_t0_1_dn12)) } } else { (assign30740_e48588 * (p.p244 * (locals.var_t0_1_dn12 / locals.var_t0_1))) }, if 0.0 == 0.0 && ((p.p244) as f64).is_finite() && ((p.p244) as f64).fract() == 0.0 { if p.p244 == 0.0 { 0.0 } else { (p.p244 * ((locals.var_t0_1).powf(p.p244 - 1.0) * locals.var_t0_1_dn14)) } } else { (assign30740_e48588 * (p.p244 * (locals.var_t0_1_dn14 / locals.var_t0_1))) }, if 0.0 == 0.0 && ((p.p244) as f64).is_finite() && ((p.p244) as f64).fract() == 0.0 { if p.p244 == 0.0 { 0.0 } else { (p.p244 * ((locals.var_t0_1).powf(p.p244 - 1.0) * locals.var_t0_1_dn15)) } } else { (assign30740_e48588 * (p.p244 * (locals.var_t0_1_dn15 / locals.var_t0_1))) }, if 0.0 == 0.0 && ((p.p244) as f64).is_finite() && ((p.p244) as f64).fract() == 0.0 { if p.p244 == 0.0 { 0.0 } else { (p.p244 * ((locals.var_t0_1).powf(p.p244 - 1.0) * locals.var_t0_1_dn16)) } } else { (assign30740_e48588 * (p.p244 * (locals.var_t0_1_dn16 / locals.var_t0_1))) }, if 0.0 == 0.0 && ((p.p244) as f64).is_finite() && ((p.p244) as f64).fract() == 0.0 { if p.p244 == 0.0 { 0.0 } else { (p.p244 * ((locals.var_t0_1).powf(p.p244 - 1.0) * locals.var_t0_1_dn17)) } } else { (assign30740_e48588 * (p.p244 * (locals.var_t0_1_dn17 / locals.var_t0_1))) }, if 0.0 == 0.0 && ((p.p244) as f64).is_finite() && ((p.p244) as f64).fract() == 0.0 { if p.p244 == 0.0 { 0.0 } else { (p.p244 * ((locals.var_t0_1).powf(p.p244 - 1.0) * locals.var_t0_1_dn18)) } } else { (assign30740_e48588 * (p.p244 * (locals.var_t0_1_dn18 / locals.var_t0_1))) }, if 0.0 == 0.0 && ((p.p244) as f64).is_finite() && ((p.p244) as f64).fract() == 0.0 { if p.p244 == 0.0 { 0.0 } else { (p.p244 * ((locals.var_t0_1).powf(p.p244 - 1.0) * locals.var_t0_1_dn19)) } } else { (assign30740_e48588 * (p.p244 * (locals.var_t0_1_dn19 / locals.var_t0_1))) }, if 0.0 == 0.0 && ((p.p244) as f64).is_finite() && ((p.p244) as f64).fract() == 0.0 { if p.p244 == 0.0 { 0.0 } else { (p.p244 * ((locals.var_t0_1).powf(p.p244 - 1.0) * locals.var_t0_1_dn20)) } } else { (assign30740_e48588 * (p.p244 * (locals.var_t0_1_dn20 / locals.var_t0_1))) }, if 0.0 == 0.0 && ((p.p244) as f64).is_finite() && ((p.p244) as f64).fract() == 0.0 { if p.p244 == 0.0 { 0.0 } else { (p.p244 * ((locals.var_t0_1).powf(p.p244 - 1.0) * locals.var_t0_1_dn21)) } } else { (assign30740_e48588 * (p.p244 * (locals.var_t0_1_dn21 / locals.var_t0_1))) }, if 0.0 == 0.0 && ((p.p244) as f64).is_finite() && ((p.p244) as f64).fract() == 0.0 { if p.p244 == 0.0 { 0.0 } else { (p.p244 * ((locals.var_t0_1).powf(p.p244 - 1.0) * locals.var_t0_1_dn22)) } } else { (assign30740_e48588 * (p.p244 * (locals.var_t0_1_dn22 / locals.var_t0_1))) },)
    } else {
        (locals.var_t1_1, locals.var_t1_1_dn0, locals.var_t1_1_dn1, locals.var_t1_1_dn2, locals.var_t1_1_dn3, locals.var_t1_1_dn4, locals.var_t1_1_dn5, locals.var_t1_1_dn6, locals.var_t1_1_dn7, locals.var_t1_1_dn8, locals.var_t1_1_dn9, locals.var_t1_1_dn12, locals.var_t1_1_dn14, locals.var_t1_1_dn15, locals.var_t1_1_dn16, locals.var_t1_1_dn17, locals.var_t1_1_dn18, locals.var_t1_1_dn19, locals.var_t1_1_dn20, locals.var_t1_1_dn21, locals.var_t1_1_dn22,)
    }
};
        locals.var_t1_1 = assign30740_e48591;
        locals.var_t1_1_dn0 = assign30740_e48591_d_n0;
        locals.var_t1_1_dn1 = assign30740_e48591_d_n1;
        locals.var_t1_1_dn2 = assign30740_e48591_d_n2;
        locals.var_t1_1_dn3 = assign30740_e48591_d_n3;
        locals.var_t1_1_dn4 = assign30740_e48591_d_n4;
        locals.var_t1_1_dn5 = assign30740_e48591_d_n5;
        locals.var_t1_1_dn6 = assign30740_e48591_d_n6;
        locals.var_t1_1_dn7 = assign30740_e48591_d_n7;
        locals.var_t1_1_dn8 = assign30740_e48591_d_n8;
        locals.var_t1_1_dn9 = assign30740_e48591_d_n9;
        locals.var_t1_1_dn12 = assign30740_e48591_d_n12;
        locals.var_t1_1_dn14 = assign30740_e48591_d_n14;
        locals.var_t1_1_dn15 = assign30740_e48591_d_n15;
        locals.var_t1_1_dn16 = assign30740_e48591_d_n16;
        locals.var_t1_1_dn17 = assign30740_e48591_d_n17;
        locals.var_t1_1_dn18 = assign30740_e48591_d_n18;
        locals.var_t1_1_dn19 = assign30740_e48591_d_n19;
        locals.var_t1_1_dn20 = assign30740_e48591_d_n20;
        locals.var_t1_1_dn21 = assign30740_e48591_d_n21;
        locals.var_t1_1_dn22 = assign30740_e48591_d_n22;

        let (assign30750_e48600, assign30750_e48600_d_n0, assign30750_e48600_d_n1, assign30750_e48600_d_n2, assign30750_e48600_d_n3, assign30750_e48600_d_n4, assign30750_e48600_d_n5, assign30750_e48600_d_n6, assign30750_e48600_d_n7, assign30750_e48600_d_n8, assign30750_e48600_d_n9, assign30750_e48600_d_n12, assign30750_e48600_d_n14, assign30750_e48600_d_n15, assign30750_e48600_d_n16, assign30750_e48600_d_n17, assign30750_e48600_d_n18, assign30750_e48600_d_n19, assign30750_e48600_d_n20, assign30750_e48600_d_n21, assign30750_e48600_d_n22,) = {
    if ((locals.var_guard504 == 0.0) && (locals.var_guard513 != 0.0)) {
        let assign30750_e48598: f64 = (p.p243 / locals.var_t1_1);
        (assign30750_e48598, (-((p.p243 * locals.var_t1_1_dn0) / (locals.var_t1_1 * locals.var_t1_1))), (-((p.p243 * locals.var_t1_1_dn1) / (locals.var_t1_1 * locals.var_t1_1))), (-((p.p243 * locals.var_t1_1_dn2) / (locals.var_t1_1 * locals.var_t1_1))), (-((p.p243 * locals.var_t1_1_dn3) / (locals.var_t1_1 * locals.var_t1_1))), (-((p.p243 * locals.var_t1_1_dn4) / (locals.var_t1_1 * locals.var_t1_1))), (-((p.p243 * locals.var_t1_1_dn5) / (locals.var_t1_1 * locals.var_t1_1))), (-((p.p243 * locals.var_t1_1_dn6) / (locals.var_t1_1 * locals.var_t1_1))), (-((p.p243 * locals.var_t1_1_dn7) / (locals.var_t1_1 * locals.var_t1_1))), (-((p.p243 * locals.var_t1_1_dn8) / (locals.var_t1_1 * locals.var_t1_1))), (-((p.p243 * locals.var_t1_1_dn9) / (locals.var_t1_1 * locals.var_t1_1))), (-((p.p243 * locals.var_t1_1_dn12) / (locals.var_t1_1 * locals.var_t1_1))), (-((p.p243 * locals.var_t1_1_dn14) / (locals.var_t1_1 * locals.var_t1_1))), (-((p.p243 * locals.var_t1_1_dn15) / (locals.var_t1_1 * locals.var_t1_1))), (-((p.p243 * locals.var_t1_1_dn16) / (locals.var_t1_1 * locals.var_t1_1))), (-((p.p243 * locals.var_t1_1_dn17) / (locals.var_t1_1 * locals.var_t1_1))), (-((p.p243 * locals.var_t1_1_dn18) / (locals.var_t1_1 * locals.var_t1_1))), (-((p.p243 * locals.var_t1_1_dn19) / (locals.var_t1_1 * locals.var_t1_1))), (-((p.p243 * locals.var_t1_1_dn20) / (locals.var_t1_1 * locals.var_t1_1))), (-((p.p243 * locals.var_t1_1_dn21) / (locals.var_t1_1 * locals.var_t1_1))), (-((p.p243 * locals.var_t1_1_dn22) / (locals.var_t1_1 * locals.var_t1_1))),)
    } else {
        (locals.var_xdcinv, locals.var_xdcinv_dn0, locals.var_xdcinv_dn1, locals.var_xdcinv_dn2, locals.var_xdcinv_dn3, locals.var_xdcinv_dn4, locals.var_xdcinv_dn5, locals.var_xdcinv_dn6, locals.var_xdcinv_dn7, locals.var_xdcinv_dn8, locals.var_xdcinv_dn9, locals.var_xdcinv_dn12, locals.var_xdcinv_dn14, locals.var_xdcinv_dn15, locals.var_xdcinv_dn16, locals.var_xdcinv_dn17, locals.var_xdcinv_dn18, locals.var_xdcinv_dn19, locals.var_xdcinv_dn20, locals.var_xdcinv_dn21, locals.var_xdcinv_dn22,)
    }
};
        locals.var_xdcinv = assign30750_e48600;
        locals.var_xdcinv_dn0 = assign30750_e48600_d_n0;
        locals.var_xdcinv_dn1 = assign30750_e48600_d_n1;
        locals.var_xdcinv_dn2 = assign30750_e48600_d_n2;
        locals.var_xdcinv_dn3 = assign30750_e48600_d_n3;
        locals.var_xdcinv_dn4 = assign30750_e48600_d_n4;
        locals.var_xdcinv_dn5 = assign30750_e48600_d_n5;
        locals.var_xdcinv_dn6 = assign30750_e48600_d_n6;
        locals.var_xdcinv_dn7 = assign30750_e48600_d_n7;
        locals.var_xdcinv_dn8 = assign30750_e48600_d_n8;
        locals.var_xdcinv_dn9 = assign30750_e48600_d_n9;
        locals.var_xdcinv_dn12 = assign30750_e48600_d_n12;
        locals.var_xdcinv_dn14 = assign30750_e48600_d_n14;
        locals.var_xdcinv_dn15 = assign30750_e48600_d_n15;
        locals.var_xdcinv_dn16 = assign30750_e48600_d_n16;
        locals.var_xdcinv_dn17 = assign30750_e48600_d_n17;
        locals.var_xdcinv_dn18 = assign30750_e48600_d_n18;
        locals.var_xdcinv_dn19 = assign30750_e48600_d_n19;
        locals.var_xdcinv_dn20 = assign30750_e48600_d_n20;
        locals.var_xdcinv_dn21 = assign30750_e48600_d_n21;
        locals.var_xdcinv_dn22 = assign30750_e48600_d_n22;

    }

    pub(super) fn stamp_transient_block_181(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let (assign30760_e48611, assign30760_e48611_d_n0, assign30760_e48611_d_n1, assign30760_e48611_d_n2, assign30760_e48611_d_n3, assign30760_e48611_d_n4, assign30760_e48611_d_n5, assign30760_e48611_d_n6, assign30760_e48611_d_n7, assign30760_e48611_d_n8, assign30760_e48611_d_n9, assign30760_e48611_d_n12, assign30760_e48611_d_n14, assign30760_e48611_d_n15, assign30760_e48611_d_n16, assign30760_e48611_d_n17, assign30760_e48611_d_n18, assign30760_e48611_d_n19, assign30760_e48611_d_n20, assign30760_e48611_d_n21, assign30760_e48611_d_n22,) = {
    if ((locals.var_guard504 == 0.0) && (locals.var_guard513 != 0.0)) {
        let assign30760_e48608: f64 = (p.p199 + locals.var_xdcinv);
        let assign30760_e48609: f64 = (p.p9 / assign30760_e48608);
        (assign30760_e48609, (-((p.p9 * locals.var_xdcinv_dn0) / (assign30760_e48608 * assign30760_e48608))), (-((p.p9 * locals.var_xdcinv_dn1) / (assign30760_e48608 * assign30760_e48608))), (-((p.p9 * locals.var_xdcinv_dn2) / (assign30760_e48608 * assign30760_e48608))), (-((p.p9 * locals.var_xdcinv_dn3) / (assign30760_e48608 * assign30760_e48608))), (-((p.p9 * locals.var_xdcinv_dn4) / (assign30760_e48608 * assign30760_e48608))), (-((p.p9 * locals.var_xdcinv_dn5) / (assign30760_e48608 * assign30760_e48608))), (-((p.p9 * locals.var_xdcinv_dn6) / (assign30760_e48608 * assign30760_e48608))), (-((p.p9 * locals.var_xdcinv_dn7) / (assign30760_e48608 * assign30760_e48608))), (-((p.p9 * locals.var_xdcinv_dn8) / (assign30760_e48608 * assign30760_e48608))), (-((p.p9 * locals.var_xdcinv_dn9) / (assign30760_e48608 * assign30760_e48608))), (-((p.p9 * locals.var_xdcinv_dn12) / (assign30760_e48608 * assign30760_e48608))), (-((p.p9 * locals.var_xdcinv_dn14) / (assign30760_e48608 * assign30760_e48608))), (-((p.p9 * locals.var_xdcinv_dn15) / (assign30760_e48608 * assign30760_e48608))), (-((p.p9 * locals.var_xdcinv_dn16) / (assign30760_e48608 * assign30760_e48608))), (-((p.p9 * locals.var_xdcinv_dn17) / (assign30760_e48608 * assign30760_e48608))), (-((p.p9 * locals.var_xdcinv_dn18) / (assign30760_e48608 * assign30760_e48608))), (-((p.p9 * locals.var_xdcinv_dn19) / (assign30760_e48608 * assign30760_e48608))), (-((p.p9 * locals.var_xdcinv_dn20) / (assign30760_e48608 * assign30760_e48608))), (-((p.p9 * locals.var_xdcinv_dn21) / (assign30760_e48608 * assign30760_e48608))), (-((p.p9 * locals.var_xdcinv_dn22) / (assign30760_e48608 * assign30760_e48608))),)
    } else {
        (locals.var_cg_qme, locals.var_cg_qme_dn0, locals.var_cg_qme_dn1, locals.var_cg_qme_dn2, locals.var_cg_qme_dn3, locals.var_cg_qme_dn4, locals.var_cg_qme_dn5, locals.var_cg_qme_dn6, locals.var_cg_qme_dn7, locals.var_cg_qme_dn8, locals.var_cg_qme_dn9, locals.var_cg_qme_dn12, locals.var_cg_qme_dn14, locals.var_cg_qme_dn15, locals.var_cg_qme_dn16, locals.var_cg_qme_dn17, locals.var_cg_qme_dn18, locals.var_cg_qme_dn19, locals.var_cg_qme_dn20, locals.var_cg_qme_dn21, locals.var_cg_qme_dn22,)
    }
};
        locals.var_cg_qme = assign30760_e48611;
        locals.var_cg_qme_dn0 = assign30760_e48611_d_n0;
        locals.var_cg_qme_dn1 = assign30760_e48611_d_n1;
        locals.var_cg_qme_dn2 = assign30760_e48611_d_n2;
        locals.var_cg_qme_dn3 = assign30760_e48611_d_n3;
        locals.var_cg_qme_dn4 = assign30760_e48611_d_n4;
        locals.var_cg_qme_dn5 = assign30760_e48611_d_n5;
        locals.var_cg_qme_dn6 = assign30760_e48611_d_n6;
        locals.var_cg_qme_dn7 = assign30760_e48611_d_n7;
        locals.var_cg_qme_dn8 = assign30760_e48611_d_n8;
        locals.var_cg_qme_dn9 = assign30760_e48611_d_n9;
        locals.var_cg_qme_dn12 = assign30760_e48611_d_n12;
        locals.var_cg_qme_dn14 = assign30760_e48611_d_n14;
        locals.var_cg_qme_dn15 = assign30760_e48611_d_n15;
        locals.var_cg_qme_dn16 = assign30760_e48611_d_n16;
        locals.var_cg_qme_dn17 = assign30760_e48611_d_n17;
        locals.var_cg_qme_dn18 = assign30760_e48611_d_n18;
        locals.var_cg_qme_dn19 = assign30760_e48611_d_n19;
        locals.var_cg_qme_dn20 = assign30760_e48611_d_n20;
        locals.var_cg_qme_dn21 = assign30760_e48611_d_n21;
        locals.var_cg_qme_dn22 = assign30760_e48611_d_n22;

        let (assign30770_e48638, assign30770_e48638_d_n0, assign30770_e48638_d_n1, assign30770_e48638_d_n2, assign30770_e48638_d_n3, assign30770_e48638_d_n4, assign30770_e48638_d_n5, assign30770_e48638_d_n6, assign30770_e48638_d_n7, assign30770_e48638_d_n8, assign30770_e48638_d_n9, assign30770_e48638_d_n12, assign30770_e48638_d_n14, assign30770_e48638_d_n15, assign30770_e48638_d_n16, assign30770_e48638_d_n17, assign30770_e48638_d_n18, assign30770_e48638_d_n19, assign30770_e48638_d_n20, assign30770_e48638_d_n21, assign30770_e48638_d_n22,) = {
    if ((locals.var_guard504 == 0.0) && (locals.var_guard513 != 0.0)) {
        let assign30770_e48618: f64 = (locals.var_cg_qme * p.p4);
        let assign30770_e48620: f64 = (assign30770_e48618 * p.p5);
        let assign30770_e48622: f64 = (assign30770_e48620 * p.p200);
        let assign30770_e48625: f64 = (locals.var_vg0_fp4s - locals.var_psim_fp4s);
        let assign30770_e48628: f64 = (0.5 * locals.var_t1);
        let assign30770_e48630: f64 = (assign30770_e48628 * locals.var_t1);
        let assign30770_e48633: f64 = (6.0 * locals.var_t2);
        let assign30770_e48634: f64 = (assign30770_e48630 / assign30770_e48633);
        let assign30770_e48635: f64 = (assign30770_e48625 + assign30770_e48634);
        let assign30770_e48636: f64 = (assign30770_e48622 * assign30770_e48635);
        (assign30770_e48636, (((((locals.var_cg_qme_dn0 * p.p4) * p.p5) * p.p200) * assign30770_e48635) + (assign30770_e48622 * ((locals.var_vg0_fp4s_dn0 - locals.var_psim_fp4s_dn0) + ((((((0.5 * locals.var_t1_dn0) * locals.var_t1) + (assign30770_e48628 * locals.var_t1_dn0)) * assign30770_e48633) - (assign30770_e48630 * (6.0 * locals.var_t2_dn0))) / (assign30770_e48633 * assign30770_e48633))))), (((((locals.var_cg_qme_dn1 * p.p4) * p.p5) * p.p200) * assign30770_e48635) + (assign30770_e48622 * ((locals.var_vg0_fp4s_dn1 - locals.var_psim_fp4s_dn1) + ((((((0.5 * locals.var_t1_dn1) * locals.var_t1) + (assign30770_e48628 * locals.var_t1_dn1)) * assign30770_e48633) - (assign30770_e48630 * (6.0 * locals.var_t2_dn1))) / (assign30770_e48633 * assign30770_e48633))))), (((((locals.var_cg_qme_dn2 * p.p4) * p.p5) * p.p200) * assign30770_e48635) + (assign30770_e48622 * ((locals.var_vg0_fp4s_dn2 - locals.var_psim_fp4s_dn2) + ((((((0.5 * locals.var_t1_dn2) * locals.var_t1) + (assign30770_e48628 * locals.var_t1_dn2)) * assign30770_e48633) - (assign30770_e48630 * (6.0 * locals.var_t2_dn2))) / (assign30770_e48633 * assign30770_e48633))))), (((((locals.var_cg_qme_dn3 * p.p4) * p.p5) * p.p200) * assign30770_e48635) + (assign30770_e48622 * ((locals.var_vg0_fp4s_dn3 - locals.var_psim_fp4s_dn3) + ((((((0.5 * locals.var_t1_dn3) * locals.var_t1) + (assign30770_e48628 * locals.var_t1_dn3)) * assign30770_e48633) - (assign30770_e48630 * (6.0 * locals.var_t2_dn3))) / (assign30770_e48633 * assign30770_e48633))))), (((((locals.var_cg_qme_dn4 * p.p4) * p.p5) * p.p200) * assign30770_e48635) + (assign30770_e48622 * ((locals.var_vg0_fp4s_dn4 - locals.var_psim_fp4s_dn4) + ((((((0.5 * locals.var_t1_dn4) * locals.var_t1) + (assign30770_e48628 * locals.var_t1_dn4)) * assign30770_e48633) - (assign30770_e48630 * (6.0 * locals.var_t2_dn4))) / (assign30770_e48633 * assign30770_e48633))))), (((((locals.var_cg_qme_dn5 * p.p4) * p.p5) * p.p200) * assign30770_e48635) + (assign30770_e48622 * ((locals.var_vg0_fp4s_dn5 - locals.var_psim_fp4s_dn5) + ((((((0.5 * locals.var_t1_dn5) * locals.var_t1) + (assign30770_e48628 * locals.var_t1_dn5)) * assign30770_e48633) - (assign30770_e48630 * (6.0 * locals.var_t2_dn5))) / (assign30770_e48633 * assign30770_e48633))))), (((((locals.var_cg_qme_dn6 * p.p4) * p.p5) * p.p200) * assign30770_e48635) + (assign30770_e48622 * ((locals.var_vg0_fp4s_dn6 - locals.var_psim_fp4s_dn6) + ((((((0.5 * locals.var_t1_dn6) * locals.var_t1) + (assign30770_e48628 * locals.var_t1_dn6)) * assign30770_e48633) - (assign30770_e48630 * (6.0 * locals.var_t2_dn6))) / (assign30770_e48633 * assign30770_e48633))))), (((((locals.var_cg_qme_dn7 * p.p4) * p.p5) * p.p200) * assign30770_e48635) + (assign30770_e48622 * ((locals.var_vg0_fp4s_dn7 - locals.var_psim_fp4s_dn7) + ((((((0.5 * locals.var_t1_dn7) * locals.var_t1) + (assign30770_e48628 * locals.var_t1_dn7)) * assign30770_e48633) - (assign30770_e48630 * (6.0 * locals.var_t2_dn7))) / (assign30770_e48633 * assign30770_e48633))))), (((((locals.var_cg_qme_dn8 * p.p4) * p.p5) * p.p200) * assign30770_e48635) + (assign30770_e48622 * ((locals.var_vg0_fp4s_dn8 - locals.var_psim_fp4s_dn8) + ((((((0.5 * locals.var_t1_dn8) * locals.var_t1) + (assign30770_e48628 * locals.var_t1_dn8)) * assign30770_e48633) - (assign30770_e48630 * (6.0 * locals.var_t2_dn8))) / (assign30770_e48633 * assign30770_e48633))))), (((((locals.var_cg_qme_dn9 * p.p4) * p.p5) * p.p200) * assign30770_e48635) + (assign30770_e48622 * ((locals.var_vg0_fp4s_dn9 - locals.var_psim_fp4s_dn9) + ((((((0.5 * locals.var_t1_dn9) * locals.var_t1) + (assign30770_e48628 * locals.var_t1_dn9)) * assign30770_e48633) - (assign30770_e48630 * (6.0 * locals.var_t2_dn9))) / (assign30770_e48633 * assign30770_e48633))))), (((((locals.var_cg_qme_dn12 * p.p4) * p.p5) * p.p200) * assign30770_e48635) + (assign30770_e48622 * ((locals.var_vg0_fp4s_dn12 - locals.var_psim_fp4s_dn12) + ((((((0.5 * locals.var_t1_dn12) * locals.var_t1) + (assign30770_e48628 * locals.var_t1_dn12)) * assign30770_e48633) - (assign30770_e48630 * (6.0 * locals.var_t2_dn12))) / (assign30770_e48633 * assign30770_e48633))))), (((((locals.var_cg_qme_dn14 * p.p4) * p.p5) * p.p200) * assign30770_e48635) + (assign30770_e48622 * ((locals.var_vg0_fp4s_dn14 - locals.var_psim_fp4s_dn14) + ((((((0.5 * locals.var_t1_dn14) * locals.var_t1) + (assign30770_e48628 * locals.var_t1_dn14)) * assign30770_e48633) - (assign30770_e48630 * (6.0 * locals.var_t2_dn14))) / (assign30770_e48633 * assign30770_e48633))))), (((((locals.var_cg_qme_dn15 * p.p4) * p.p5) * p.p200) * assign30770_e48635) + (assign30770_e48622 * ((locals.var_vg0_fp4s_dn15 - locals.var_psim_fp4s_dn15) + ((((((0.5 * locals.var_t1_dn15) * locals.var_t1) + (assign30770_e48628 * locals.var_t1_dn15)) * assign30770_e48633) - (assign30770_e48630 * (6.0 * locals.var_t2_dn15))) / (assign30770_e48633 * assign30770_e48633))))), (((((locals.var_cg_qme_dn16 * p.p4) * p.p5) * p.p200) * assign30770_e48635) + (assign30770_e48622 * ((locals.var_vg0_fp4s_dn16 - locals.var_psim_fp4s_dn16) + ((((((0.5 * locals.var_t1_dn16) * locals.var_t1) + (assign30770_e48628 * locals.var_t1_dn16)) * assign30770_e48633) - (assign30770_e48630 * (6.0 * locals.var_t2_dn16))) / (assign30770_e48633 * assign30770_e48633))))), (((((locals.var_cg_qme_dn17 * p.p4) * p.p5) * p.p200) * assign30770_e48635) + (assign30770_e48622 * ((locals.var_vg0_fp4s_dn17 - locals.var_psim_fp4s_dn17) + ((((((0.5 * locals.var_t1_dn17) * locals.var_t1) + (assign30770_e48628 * locals.var_t1_dn17)) * assign30770_e48633) - (assign30770_e48630 * (6.0 * locals.var_t2_dn17))) / (assign30770_e48633 * assign30770_e48633))))), (((((locals.var_cg_qme_dn18 * p.p4) * p.p5) * p.p200) * assign30770_e48635) + (assign30770_e48622 * ((locals.var_vg0_fp4s_dn18 - locals.var_psim_fp4s_dn18) + ((((((0.5 * locals.var_t1_dn18) * locals.var_t1) + (assign30770_e48628 * locals.var_t1_dn18)) * assign30770_e48633) - (assign30770_e48630 * (6.0 * locals.var_t2_dn18))) / (assign30770_e48633 * assign30770_e48633))))), (((((locals.var_cg_qme_dn19 * p.p4) * p.p5) * p.p200) * assign30770_e48635) + (assign30770_e48622 * ((locals.var_vg0_fp4s_dn19 - locals.var_psim_fp4s_dn19) + ((((((0.5 * locals.var_t1_dn19) * locals.var_t1) + (assign30770_e48628 * locals.var_t1_dn19)) * assign30770_e48633) - (assign30770_e48630 * (6.0 * locals.var_t2_dn19))) / (assign30770_e48633 * assign30770_e48633))))), (((((locals.var_cg_qme_dn20 * p.p4) * p.p5) * p.p200) * assign30770_e48635) + (assign30770_e48622 * ((locals.var_vg0_fp4s_dn20 - locals.var_psim_fp4s_dn20) + ((((((0.5 * locals.var_t1_dn20) * locals.var_t1) + (assign30770_e48628 * locals.var_t1_dn20)) * assign30770_e48633) - (assign30770_e48630 * (6.0 * locals.var_t2_dn20))) / (assign30770_e48633 * assign30770_e48633))))), (((((locals.var_cg_qme_dn21 * p.p4) * p.p5) * p.p200) * assign30770_e48635) + (assign30770_e48622 * ((locals.var_vg0_fp4s_dn21 - locals.var_psim_fp4s_dn21) + ((((((0.5 * locals.var_t1_dn21) * locals.var_t1) + (assign30770_e48628 * locals.var_t1_dn21)) * assign30770_e48633) - (assign30770_e48630 * (6.0 * locals.var_t2_dn21))) / (assign30770_e48633 * assign30770_e48633))))), (((((locals.var_cg_qme_dn22 * p.p4) * p.p5) * p.p200) * assign30770_e48635) + (assign30770_e48622 * ((locals.var_vg0_fp4s_dn22 - locals.var_psim_fp4s_dn22) + ((((((0.5 * locals.var_t1_dn22) * locals.var_t1) + (assign30770_e48628 * locals.var_t1_dn22)) * assign30770_e48633) - (assign30770_e48630 * (6.0 * locals.var_t2_dn22))) / (assign30770_e48633 * assign30770_e48633))))),)
    } else {
        (locals.var_qg_fp4s, locals.var_qg_fp4s_dn0, locals.var_qg_fp4s_dn1, locals.var_qg_fp4s_dn2, locals.var_qg_fp4s_dn3, locals.var_qg_fp4s_dn4, locals.var_qg_fp4s_dn5, locals.var_qg_fp4s_dn6, locals.var_qg_fp4s_dn7, locals.var_qg_fp4s_dn8, locals.var_qg_fp4s_dn9, locals.var_qg_fp4s_dn12, locals.var_qg_fp4s_dn14, locals.var_qg_fp4s_dn15, locals.var_qg_fp4s_dn16, locals.var_qg_fp4s_dn17, locals.var_qg_fp4s_dn18, locals.var_qg_fp4s_dn19, locals.var_qg_fp4s_dn20, locals.var_qg_fp4s_dn21, locals.var_qg_fp4s_dn22,)
    }
};
        locals.var_qg_fp4s = assign30770_e48638;
        locals.var_qg_fp4s_dn0 = assign30770_e48638_d_n0;
        locals.var_qg_fp4s_dn1 = assign30770_e48638_d_n1;
        locals.var_qg_fp4s_dn2 = assign30770_e48638_d_n2;
        locals.var_qg_fp4s_dn3 = assign30770_e48638_d_n3;
        locals.var_qg_fp4s_dn4 = assign30770_e48638_d_n4;
        locals.var_qg_fp4s_dn5 = assign30770_e48638_d_n5;
        locals.var_qg_fp4s_dn6 = assign30770_e48638_d_n6;
        locals.var_qg_fp4s_dn7 = assign30770_e48638_d_n7;
        locals.var_qg_fp4s_dn8 = assign30770_e48638_d_n8;
        locals.var_qg_fp4s_dn9 = assign30770_e48638_d_n9;
        locals.var_qg_fp4s_dn12 = assign30770_e48638_d_n12;
        locals.var_qg_fp4s_dn14 = assign30770_e48638_d_n14;
        locals.var_qg_fp4s_dn15 = assign30770_e48638_d_n15;
        locals.var_qg_fp4s_dn16 = assign30770_e48638_d_n16;
        locals.var_qg_fp4s_dn17 = assign30770_e48638_d_n17;
        locals.var_qg_fp4s_dn18 = assign30770_e48638_d_n18;
        locals.var_qg_fp4s_dn19 = assign30770_e48638_d_n19;
        locals.var_qg_fp4s_dn20 = assign30770_e48638_d_n20;
        locals.var_qg_fp4s_dn21 = assign30770_e48638_d_n21;
        locals.var_qg_fp4s_dn22 = assign30770_e48638_d_n22;

        let (assign30780_e48649, assign30780_e48649_d_n0, assign30780_e48649_d_n1, assign30780_e48649_d_n2, assign30780_e48649_d_n3, assign30780_e48649_d_n4, assign30780_e48649_d_n5, assign30780_e48649_d_n6, assign30780_e48649_d_n7, assign30780_e48649_d_n8, assign30780_e48649_d_n9, assign30780_e48649_d_n12, assign30780_e48649_d_n14, assign30780_e48649_d_n15, assign30780_e48649_d_n16, assign30780_e48649_d_n17, assign30780_e48649_d_n18, assign30780_e48649_d_n19, assign30780_e48649_d_n20, assign30780_e48649_d_n21, assign30780_e48649_d_n22,) = {
    if ((locals.var_guard504 == 0.0) && (locals.var_guard513 != 0.0)) {
        let assign30780_e48645: f64 = (locals.var_vg0_fp4s + locals.var_vtv);
        let assign30780_e48647: f64 = (assign30780_e48645 - locals.var_psim_fp4s);
        (assign30780_e48647, (locals.var_vg0_fp4s_dn0 - locals.var_psim_fp4s_dn0), (locals.var_vg0_fp4s_dn1 - locals.var_psim_fp4s_dn1), (locals.var_vg0_fp4s_dn2 - locals.var_psim_fp4s_dn2), (locals.var_vg0_fp4s_dn3 - locals.var_psim_fp4s_dn3), ((locals.var_vg0_fp4s_dn4 + locals.var_vtv_dn4) - locals.var_psim_fp4s_dn4), (locals.var_vg0_fp4s_dn5 - locals.var_psim_fp4s_dn5), ((locals.var_vg0_fp4s_dn6 + locals.var_vtv_dn6) - locals.var_psim_fp4s_dn6), ((locals.var_vg0_fp4s_dn7 + locals.var_vtv_dn7) - locals.var_psim_fp4s_dn7), ((locals.var_vg0_fp4s_dn8 + locals.var_vtv_dn8) - locals.var_psim_fp4s_dn8), (locals.var_vg0_fp4s_dn9 - locals.var_psim_fp4s_dn9), (locals.var_vg0_fp4s_dn12 - locals.var_psim_fp4s_dn12), (locals.var_vg0_fp4s_dn14 - locals.var_psim_fp4s_dn14), ((locals.var_vg0_fp4s_dn15 + locals.var_vtv_dn15) - locals.var_psim_fp4s_dn15), ((locals.var_vg0_fp4s_dn16 + locals.var_vtv_dn16) - locals.var_psim_fp4s_dn16), ((locals.var_vg0_fp4s_dn17 + locals.var_vtv_dn17) - locals.var_psim_fp4s_dn17), ((locals.var_vg0_fp4s_dn18 + locals.var_vtv_dn18) - locals.var_psim_fp4s_dn18), ((locals.var_vg0_fp4s_dn19 + locals.var_vtv_dn19) - locals.var_psim_fp4s_dn19), ((locals.var_vg0_fp4s_dn20 + locals.var_vtv_dn20) - locals.var_psim_fp4s_dn20), ((locals.var_vg0_fp4s_dn21 + locals.var_vtv_dn21) - locals.var_psim_fp4s_dn21), ((locals.var_vg0_fp4s_dn22 + locals.var_vtv_dn22) - locals.var_psim_fp4s_dn22),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn1, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn12, locals.var_t0_dn14, locals.var_t0_dn15, locals.var_t0_dn16, locals.var_t0_dn17, locals.var_t0_dn18, locals.var_t0_dn19, locals.var_t0_dn20, locals.var_t0_dn21, locals.var_t0_dn22,)
    }
};
        locals.var_t0 = assign30780_e48649;
        locals.var_t0_dn0 = assign30780_e48649_d_n0;
        locals.var_t0_dn1 = assign30780_e48649_d_n1;
        locals.var_t0_dn2 = assign30780_e48649_d_n2;
        locals.var_t0_dn3 = assign30780_e48649_d_n3;
        locals.var_t0_dn4 = assign30780_e48649_d_n4;
        locals.var_t0_dn5 = assign30780_e48649_d_n5;
        locals.var_t0_dn6 = assign30780_e48649_d_n6;
        locals.var_t0_dn7 = assign30780_e48649_d_n7;
        locals.var_t0_dn8 = assign30780_e48649_d_n8;
        locals.var_t0_dn9 = assign30780_e48649_d_n9;
        locals.var_t0_dn12 = assign30780_e48649_d_n12;
        locals.var_t0_dn14 = assign30780_e48649_d_n14;
        locals.var_t0_dn15 = assign30780_e48649_d_n15;
        locals.var_t0_dn16 = assign30780_e48649_d_n16;
        locals.var_t0_dn17 = assign30780_e48649_d_n17;
        locals.var_t0_dn18 = assign30780_e48649_d_n18;
        locals.var_t0_dn19 = assign30780_e48649_d_n19;
        locals.var_t0_dn20 = assign30780_e48649_d_n20;
        locals.var_t0_dn21 = assign30780_e48649_d_n21;
        locals.var_t0_dn22 = assign30780_e48649_d_n22;

        let (assign30790_e48662, assign30790_e48662_d_n0, assign30790_e48662_d_n1, assign30790_e48662_d_n2, assign30790_e48662_d_n3, assign30790_e48662_d_n4, assign30790_e48662_d_n5, assign30790_e48662_d_n6, assign30790_e48662_d_n7, assign30790_e48662_d_n8, assign30790_e48662_d_n9, assign30790_e48662_d_n12, assign30790_e48662_d_n14, assign30790_e48662_d_n15, assign30790_e48662_d_n16, assign30790_e48662_d_n17, assign30790_e48662_d_n18, assign30790_e48662_d_n19, assign30790_e48662_d_n20, assign30790_e48662_d_n21, assign30790_e48662_d_n22,) = {
    if ((locals.var_guard504 == 0.0) && (locals.var_guard513 != 0.0)) {
        let assign30790_e48657: f64 = (2.0 * locals.var_psid_fp4s);
        let assign30790_e48658: f64 = (locals.var_psis_fp4s + assign30790_e48657);
        let assign30790_e48660: f64 = (assign30790_e48658 / 3.0);
        (assign30790_e48660, ((locals.var_psis_fp4s_dn0 + (2.0 * locals.var_psid_fp4s_dn0)) / 3.0), ((locals.var_psis_fp4s_dn1 + (2.0 * locals.var_psid_fp4s_dn1)) / 3.0), ((locals.var_psis_fp4s_dn2 + (2.0 * locals.var_psid_fp4s_dn2)) / 3.0), ((locals.var_psis_fp4s_dn3 + (2.0 * locals.var_psid_fp4s_dn3)) / 3.0), ((locals.var_psis_fp4s_dn4 + (2.0 * locals.var_psid_fp4s_dn4)) / 3.0), ((locals.var_psis_fp4s_dn5 + (2.0 * locals.var_psid_fp4s_dn5)) / 3.0), ((locals.var_psis_fp4s_dn6 + (2.0 * locals.var_psid_fp4s_dn6)) / 3.0), ((locals.var_psis_fp4s_dn7 + (2.0 * locals.var_psid_fp4s_dn7)) / 3.0), ((locals.var_psis_fp4s_dn8 + (2.0 * locals.var_psid_fp4s_dn8)) / 3.0), ((locals.var_psis_fp4s_dn9 + (2.0 * locals.var_psid_fp4s_dn9)) / 3.0), ((locals.var_psis_fp4s_dn12 + (2.0 * locals.var_psid_fp4s_dn12)) / 3.0), ((locals.var_psis_fp4s_dn14 + (2.0 * locals.var_psid_fp4s_dn14)) / 3.0), ((locals.var_psis_fp4s_dn15 + (2.0 * locals.var_psid_fp4s_dn15)) / 3.0), ((locals.var_psis_fp4s_dn16 + (2.0 * locals.var_psid_fp4s_dn16)) / 3.0), ((locals.var_psis_fp4s_dn17 + (2.0 * locals.var_psid_fp4s_dn17)) / 3.0), ((locals.var_psis_fp4s_dn18 + (2.0 * locals.var_psid_fp4s_dn18)) / 3.0), ((locals.var_psis_fp4s_dn19 + (2.0 * locals.var_psid_fp4s_dn19)) / 3.0), ((locals.var_psis_fp4s_dn20 + (2.0 * locals.var_psid_fp4s_dn20)) / 3.0), ((locals.var_psis_fp4s_dn21 + (2.0 * locals.var_psid_fp4s_dn21)) / 3.0), ((locals.var_psis_fp4s_dn22 + (2.0 * locals.var_psid_fp4s_dn22)) / 3.0),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn1, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn12, locals.var_t1_dn14, locals.var_t1_dn15, locals.var_t1_dn16, locals.var_t1_dn17, locals.var_t1_dn18, locals.var_t1_dn19, locals.var_t1_dn20, locals.var_t1_dn21, locals.var_t1_dn22,)
    }
};
        locals.var_t1 = assign30790_e48662;
        locals.var_t1_dn0 = assign30790_e48662_d_n0;
        locals.var_t1_dn1 = assign30790_e48662_d_n1;
        locals.var_t1_dn2 = assign30790_e48662_d_n2;
        locals.var_t1_dn3 = assign30790_e48662_d_n3;
        locals.var_t1_dn4 = assign30790_e48662_d_n4;
        locals.var_t1_dn5 = assign30790_e48662_d_n5;
        locals.var_t1_dn6 = assign30790_e48662_d_n6;
        locals.var_t1_dn7 = assign30790_e48662_d_n7;
        locals.var_t1_dn8 = assign30790_e48662_d_n8;
        locals.var_t1_dn9 = assign30790_e48662_d_n9;
        locals.var_t1_dn12 = assign30790_e48662_d_n12;
        locals.var_t1_dn14 = assign30790_e48662_d_n14;
        locals.var_t1_dn15 = assign30790_e48662_d_n15;
        locals.var_t1_dn16 = assign30790_e48662_d_n16;
        locals.var_t1_dn17 = assign30790_e48662_d_n17;
        locals.var_t1_dn18 = assign30790_e48662_d_n18;
        locals.var_t1_dn19 = assign30790_e48662_d_n19;
        locals.var_t1_dn20 = assign30790_e48662_d_n20;
        locals.var_t1_dn21 = assign30790_e48662_d_n21;
        locals.var_t1_dn22 = assign30790_e48662_d_n22;

        let (assign30800_e48677, assign30800_e48677_d_n0, assign30800_e48677_d_n1, assign30800_e48677_d_n2, assign30800_e48677_d_n3, assign30800_e48677_d_n4, assign30800_e48677_d_n5, assign30800_e48677_d_n6, assign30800_e48677_d_n7, assign30800_e48677_d_n8, assign30800_e48677_d_n9, assign30800_e48677_d_n12, assign30800_e48677_d_n14, assign30800_e48677_d_n15, assign30800_e48677_d_n16, assign30800_e48677_d_n17, assign30800_e48677_d_n18, assign30800_e48677_d_n19, assign30800_e48677_d_n20, assign30800_e48677_d_n21, assign30800_e48677_d_n22,) = {
    if ((locals.var_guard504 == 0.0) && (locals.var_guard513 != 0.0)) {
        let assign30800_e48669: f64 = (1.0 / 12.0);
        let assign30800_e48672: f64 = (locals.var_psisd_fp4s * locals.var_psisd_fp4s);
        let assign30800_e48673: f64 = (assign30800_e48669 * assign30800_e48672);
        let assign30800_e48675: f64 = (assign30800_e48673 / locals.var_t0);
        (assign30800_e48675, ((((assign30800_e48669 * ((locals.var_psisd_fp4s_dn0 * locals.var_psisd_fp4s) + (locals.var_psisd_fp4s * locals.var_psisd_fp4s_dn0))) * locals.var_t0) - (assign30800_e48673 * locals.var_t0_dn0)) / (locals.var_t0 * locals.var_t0)), ((((assign30800_e48669 * ((locals.var_psisd_fp4s_dn1 * locals.var_psisd_fp4s) + (locals.var_psisd_fp4s * locals.var_psisd_fp4s_dn1))) * locals.var_t0) - (assign30800_e48673 * locals.var_t0_dn1)) / (locals.var_t0 * locals.var_t0)), ((((assign30800_e48669 * ((locals.var_psisd_fp4s_dn2 * locals.var_psisd_fp4s) + (locals.var_psisd_fp4s * locals.var_psisd_fp4s_dn2))) * locals.var_t0) - (assign30800_e48673 * locals.var_t0_dn2)) / (locals.var_t0 * locals.var_t0)), ((((assign30800_e48669 * ((locals.var_psisd_fp4s_dn3 * locals.var_psisd_fp4s) + (locals.var_psisd_fp4s * locals.var_psisd_fp4s_dn3))) * locals.var_t0) - (assign30800_e48673 * locals.var_t0_dn3)) / (locals.var_t0 * locals.var_t0)), ((((assign30800_e48669 * ((locals.var_psisd_fp4s_dn4 * locals.var_psisd_fp4s) + (locals.var_psisd_fp4s * locals.var_psisd_fp4s_dn4))) * locals.var_t0) - (assign30800_e48673 * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)), ((((assign30800_e48669 * ((locals.var_psisd_fp4s_dn5 * locals.var_psisd_fp4s) + (locals.var_psisd_fp4s * locals.var_psisd_fp4s_dn5))) * locals.var_t0) - (assign30800_e48673 * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)), ((((assign30800_e48669 * ((locals.var_psisd_fp4s_dn6 * locals.var_psisd_fp4s) + (locals.var_psisd_fp4s * locals.var_psisd_fp4s_dn6))) * locals.var_t0) - (assign30800_e48673 * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)), ((((assign30800_e48669 * ((locals.var_psisd_fp4s_dn7 * locals.var_psisd_fp4s) + (locals.var_psisd_fp4s * locals.var_psisd_fp4s_dn7))) * locals.var_t0) - (assign30800_e48673 * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)), ((((assign30800_e48669 * ((locals.var_psisd_fp4s_dn8 * locals.var_psisd_fp4s) + (locals.var_psisd_fp4s * locals.var_psisd_fp4s_dn8))) * locals.var_t0) - (assign30800_e48673 * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)), ((((assign30800_e48669 * ((locals.var_psisd_fp4s_dn9 * locals.var_psisd_fp4s) + (locals.var_psisd_fp4s * locals.var_psisd_fp4s_dn9))) * locals.var_t0) - (assign30800_e48673 * locals.var_t0_dn9)) / (locals.var_t0 * locals.var_t0)), ((((assign30800_e48669 * ((locals.var_psisd_fp4s_dn12 * locals.var_psisd_fp4s) + (locals.var_psisd_fp4s * locals.var_psisd_fp4s_dn12))) * locals.var_t0) - (assign30800_e48673 * locals.var_t0_dn12)) / (locals.var_t0 * locals.var_t0)), ((((assign30800_e48669 * ((locals.var_psisd_fp4s_dn14 * locals.var_psisd_fp4s) + (locals.var_psisd_fp4s * locals.var_psisd_fp4s_dn14))) * locals.var_t0) - (assign30800_e48673 * locals.var_t0_dn14)) / (locals.var_t0 * locals.var_t0)), ((((assign30800_e48669 * ((locals.var_psisd_fp4s_dn15 * locals.var_psisd_fp4s) + (locals.var_psisd_fp4s * locals.var_psisd_fp4s_dn15))) * locals.var_t0) - (assign30800_e48673 * locals.var_t0_dn15)) / (locals.var_t0 * locals.var_t0)), ((((assign30800_e48669 * ((locals.var_psisd_fp4s_dn16 * locals.var_psisd_fp4s) + (locals.var_psisd_fp4s * locals.var_psisd_fp4s_dn16))) * locals.var_t0) - (assign30800_e48673 * locals.var_t0_dn16)) / (locals.var_t0 * locals.var_t0)), ((((assign30800_e48669 * ((locals.var_psisd_fp4s_dn17 * locals.var_psisd_fp4s) + (locals.var_psisd_fp4s * locals.var_psisd_fp4s_dn17))) * locals.var_t0) - (assign30800_e48673 * locals.var_t0_dn17)) / (locals.var_t0 * locals.var_t0)), ((((assign30800_e48669 * ((locals.var_psisd_fp4s_dn18 * locals.var_psisd_fp4s) + (locals.var_psisd_fp4s * locals.var_psisd_fp4s_dn18))) * locals.var_t0) - (assign30800_e48673 * locals.var_t0_dn18)) / (locals.var_t0 * locals.var_t0)), ((((assign30800_e48669 * ((locals.var_psisd_fp4s_dn19 * locals.var_psisd_fp4s) + (locals.var_psisd_fp4s * locals.var_psisd_fp4s_dn19))) * locals.var_t0) - (assign30800_e48673 * locals.var_t0_dn19)) / (locals.var_t0 * locals.var_t0)), ((((assign30800_e48669 * ((locals.var_psisd_fp4s_dn20 * locals.var_psisd_fp4s) + (locals.var_psisd_fp4s * locals.var_psisd_fp4s_dn20))) * locals.var_t0) - (assign30800_e48673 * locals.var_t0_dn20)) / (locals.var_t0 * locals.var_t0)), ((((assign30800_e48669 * ((locals.var_psisd_fp4s_dn21 * locals.var_psisd_fp4s) + (locals.var_psisd_fp4s * locals.var_psisd_fp4s_dn21))) * locals.var_t0) - (assign30800_e48673 * locals.var_t0_dn21)) / (locals.var_t0 * locals.var_t0)), ((((assign30800_e48669 * ((locals.var_psisd_fp4s_dn22 * locals.var_psisd_fp4s) + (locals.var_psisd_fp4s * locals.var_psisd_fp4s_dn22))) * locals.var_t0) - (assign30800_e48673 * locals.var_t0_dn22)) / (locals.var_t0 * locals.var_t0)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn1, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn12, locals.var_t2_dn14, locals.var_t2_dn15, locals.var_t2_dn16, locals.var_t2_dn17, locals.var_t2_dn18, locals.var_t2_dn19, locals.var_t2_dn20, locals.var_t2_dn21, locals.var_t2_dn22,)
    }
};
        locals.var_t2 = assign30800_e48677;
        locals.var_t2_dn0 = assign30800_e48677_d_n0;
        locals.var_t2_dn1 = assign30800_e48677_d_n1;
        locals.var_t2_dn2 = assign30800_e48677_d_n2;
        locals.var_t2_dn3 = assign30800_e48677_d_n3;
        locals.var_t2_dn4 = assign30800_e48677_d_n4;
        locals.var_t2_dn5 = assign30800_e48677_d_n5;
        locals.var_t2_dn6 = assign30800_e48677_d_n6;
        locals.var_t2_dn7 = assign30800_e48677_d_n7;
        locals.var_t2_dn8 = assign30800_e48677_d_n8;
        locals.var_t2_dn9 = assign30800_e48677_d_n9;
        locals.var_t2_dn12 = assign30800_e48677_d_n12;
        locals.var_t2_dn14 = assign30800_e48677_d_n14;
        locals.var_t2_dn15 = assign30800_e48677_d_n15;
        locals.var_t2_dn16 = assign30800_e48677_d_n16;
        locals.var_t2_dn17 = assign30800_e48677_d_n17;
        locals.var_t2_dn18 = assign30800_e48677_d_n18;
        locals.var_t2_dn19 = assign30800_e48677_d_n19;
        locals.var_t2_dn20 = assign30800_e48677_d_n20;
        locals.var_t2_dn21 = assign30800_e48677_d_n21;
        locals.var_t2_dn22 = assign30800_e48677_d_n22;

        let (assign30810_e48696, assign30810_e48696_d_n0, assign30810_e48696_d_n1, assign30810_e48696_d_n2, assign30810_e48696_d_n3, assign30810_e48696_d_n4, assign30810_e48696_d_n5, assign30810_e48696_d_n6, assign30810_e48696_d_n7, assign30810_e48696_d_n8, assign30810_e48696_d_n9, assign30810_e48696_d_n12, assign30810_e48696_d_n14, assign30810_e48696_d_n15, assign30810_e48696_d_n16, assign30810_e48696_d_n17, assign30810_e48696_d_n18, assign30810_e48696_d_n19, assign30810_e48696_d_n20, assign30810_e48696_d_n21, assign30810_e48696_d_n22,) = {
    if ((locals.var_guard504 == 0.0) && (locals.var_guard513 != 0.0)) {
        let assign30810_e48684: f64 = (1.0 / 120.0);
        let assign30810_e48687: f64 = (locals.var_psisd_fp4s * locals.var_psisd_fp4s);
        let assign30810_e48689: f64 = (assign30810_e48687 * locals.var_psisd_fp4s);
        let assign30810_e48690: f64 = (assign30810_e48684 * assign30810_e48689);
        let assign30810_e48693: f64 = (locals.var_t0 * locals.var_t0);
        let assign30810_e48694: f64 = (assign30810_e48690 / assign30810_e48693);
        (assign30810_e48694, ((((assign30810_e48684 * ((((locals.var_psisd_fp4s_dn0 * locals.var_psisd_fp4s) + (locals.var_psisd_fp4s * locals.var_psisd_fp4s_dn0)) * locals.var_psisd_fp4s) + (assign30810_e48687 * locals.var_psisd_fp4s_dn0))) * assign30810_e48693) - (assign30810_e48690 * ((locals.var_t0_dn0 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn0)))) / (assign30810_e48693 * assign30810_e48693)), ((((assign30810_e48684 * ((((locals.var_psisd_fp4s_dn1 * locals.var_psisd_fp4s) + (locals.var_psisd_fp4s * locals.var_psisd_fp4s_dn1)) * locals.var_psisd_fp4s) + (assign30810_e48687 * locals.var_psisd_fp4s_dn1))) * assign30810_e48693) - (assign30810_e48690 * ((locals.var_t0_dn1 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn1)))) / (assign30810_e48693 * assign30810_e48693)), ((((assign30810_e48684 * ((((locals.var_psisd_fp4s_dn2 * locals.var_psisd_fp4s) + (locals.var_psisd_fp4s * locals.var_psisd_fp4s_dn2)) * locals.var_psisd_fp4s) + (assign30810_e48687 * locals.var_psisd_fp4s_dn2))) * assign30810_e48693) - (assign30810_e48690 * ((locals.var_t0_dn2 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn2)))) / (assign30810_e48693 * assign30810_e48693)), ((((assign30810_e48684 * ((((locals.var_psisd_fp4s_dn3 * locals.var_psisd_fp4s) + (locals.var_psisd_fp4s * locals.var_psisd_fp4s_dn3)) * locals.var_psisd_fp4s) + (assign30810_e48687 * locals.var_psisd_fp4s_dn3))) * assign30810_e48693) - (assign30810_e48690 * ((locals.var_t0_dn3 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn3)))) / (assign30810_e48693 * assign30810_e48693)), ((((assign30810_e48684 * ((((locals.var_psisd_fp4s_dn4 * locals.var_psisd_fp4s) + (locals.var_psisd_fp4s * locals.var_psisd_fp4s_dn4)) * locals.var_psisd_fp4s) + (assign30810_e48687 * locals.var_psisd_fp4s_dn4))) * assign30810_e48693) - (assign30810_e48690 * ((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)))) / (assign30810_e48693 * assign30810_e48693)), ((((assign30810_e48684 * ((((locals.var_psisd_fp4s_dn5 * locals.var_psisd_fp4s) + (locals.var_psisd_fp4s * locals.var_psisd_fp4s_dn5)) * locals.var_psisd_fp4s) + (assign30810_e48687 * locals.var_psisd_fp4s_dn5))) * assign30810_e48693) - (assign30810_e48690 * ((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)))) / (assign30810_e48693 * assign30810_e48693)), ((((assign30810_e48684 * ((((locals.var_psisd_fp4s_dn6 * locals.var_psisd_fp4s) + (locals.var_psisd_fp4s * locals.var_psisd_fp4s_dn6)) * locals.var_psisd_fp4s) + (assign30810_e48687 * locals.var_psisd_fp4s_dn6))) * assign30810_e48693) - (assign30810_e48690 * ((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)))) / (assign30810_e48693 * assign30810_e48693)), ((((assign30810_e48684 * ((((locals.var_psisd_fp4s_dn7 * locals.var_psisd_fp4s) + (locals.var_psisd_fp4s * locals.var_psisd_fp4s_dn7)) * locals.var_psisd_fp4s) + (assign30810_e48687 * locals.var_psisd_fp4s_dn7))) * assign30810_e48693) - (assign30810_e48690 * ((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)))) / (assign30810_e48693 * assign30810_e48693)), ((((assign30810_e48684 * ((((locals.var_psisd_fp4s_dn8 * locals.var_psisd_fp4s) + (locals.var_psisd_fp4s * locals.var_psisd_fp4s_dn8)) * locals.var_psisd_fp4s) + (assign30810_e48687 * locals.var_psisd_fp4s_dn8))) * assign30810_e48693) - (assign30810_e48690 * ((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)))) / (assign30810_e48693 * assign30810_e48693)), ((((assign30810_e48684 * ((((locals.var_psisd_fp4s_dn9 * locals.var_psisd_fp4s) + (locals.var_psisd_fp4s * locals.var_psisd_fp4s_dn9)) * locals.var_psisd_fp4s) + (assign30810_e48687 * locals.var_psisd_fp4s_dn9))) * assign30810_e48693) - (assign30810_e48690 * ((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)))) / (assign30810_e48693 * assign30810_e48693)), ((((assign30810_e48684 * ((((locals.var_psisd_fp4s_dn12 * locals.var_psisd_fp4s) + (locals.var_psisd_fp4s * locals.var_psisd_fp4s_dn12)) * locals.var_psisd_fp4s) + (assign30810_e48687 * locals.var_psisd_fp4s_dn12))) * assign30810_e48693) - (assign30810_e48690 * ((locals.var_t0_dn12 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn12)))) / (assign30810_e48693 * assign30810_e48693)), ((((assign30810_e48684 * ((((locals.var_psisd_fp4s_dn14 * locals.var_psisd_fp4s) + (locals.var_psisd_fp4s * locals.var_psisd_fp4s_dn14)) * locals.var_psisd_fp4s) + (assign30810_e48687 * locals.var_psisd_fp4s_dn14))) * assign30810_e48693) - (assign30810_e48690 * ((locals.var_t0_dn14 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn14)))) / (assign30810_e48693 * assign30810_e48693)), ((((assign30810_e48684 * ((((locals.var_psisd_fp4s_dn15 * locals.var_psisd_fp4s) + (locals.var_psisd_fp4s * locals.var_psisd_fp4s_dn15)) * locals.var_psisd_fp4s) + (assign30810_e48687 * locals.var_psisd_fp4s_dn15))) * assign30810_e48693) - (assign30810_e48690 * ((locals.var_t0_dn15 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn15)))) / (assign30810_e48693 * assign30810_e48693)), ((((assign30810_e48684 * ((((locals.var_psisd_fp4s_dn16 * locals.var_psisd_fp4s) + (locals.var_psisd_fp4s * locals.var_psisd_fp4s_dn16)) * locals.var_psisd_fp4s) + (assign30810_e48687 * locals.var_psisd_fp4s_dn16))) * assign30810_e48693) - (assign30810_e48690 * ((locals.var_t0_dn16 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn16)))) / (assign30810_e48693 * assign30810_e48693)), ((((assign30810_e48684 * ((((locals.var_psisd_fp4s_dn17 * locals.var_psisd_fp4s) + (locals.var_psisd_fp4s * locals.var_psisd_fp4s_dn17)) * locals.var_psisd_fp4s) + (assign30810_e48687 * locals.var_psisd_fp4s_dn17))) * assign30810_e48693) - (assign30810_e48690 * ((locals.var_t0_dn17 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn17)))) / (assign30810_e48693 * assign30810_e48693)), ((((assign30810_e48684 * ((((locals.var_psisd_fp4s_dn18 * locals.var_psisd_fp4s) + (locals.var_psisd_fp4s * locals.var_psisd_fp4s_dn18)) * locals.var_psisd_fp4s) + (assign30810_e48687 * locals.var_psisd_fp4s_dn18))) * assign30810_e48693) - (assign30810_e48690 * ((locals.var_t0_dn18 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn18)))) / (assign30810_e48693 * assign30810_e48693)), ((((assign30810_e48684 * ((((locals.var_psisd_fp4s_dn19 * locals.var_psisd_fp4s) + (locals.var_psisd_fp4s * locals.var_psisd_fp4s_dn19)) * locals.var_psisd_fp4s) + (assign30810_e48687 * locals.var_psisd_fp4s_dn19))) * assign30810_e48693) - (assign30810_e48690 * ((locals.var_t0_dn19 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn19)))) / (assign30810_e48693 * assign30810_e48693)), ((((assign30810_e48684 * ((((locals.var_psisd_fp4s_dn20 * locals.var_psisd_fp4s) + (locals.var_psisd_fp4s * locals.var_psisd_fp4s_dn20)) * locals.var_psisd_fp4s) + (assign30810_e48687 * locals.var_psisd_fp4s_dn20))) * assign30810_e48693) - (assign30810_e48690 * ((locals.var_t0_dn20 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn20)))) / (assign30810_e48693 * assign30810_e48693)), ((((assign30810_e48684 * ((((locals.var_psisd_fp4s_dn21 * locals.var_psisd_fp4s) + (locals.var_psisd_fp4s * locals.var_psisd_fp4s_dn21)) * locals.var_psisd_fp4s) + (assign30810_e48687 * locals.var_psisd_fp4s_dn21))) * assign30810_e48693) - (assign30810_e48690 * ((locals.var_t0_dn21 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn21)))) / (assign30810_e48693 * assign30810_e48693)), ((((assign30810_e48684 * ((((locals.var_psisd_fp4s_dn22 * locals.var_psisd_fp4s) + (locals.var_psisd_fp4s * locals.var_psisd_fp4s_dn22)) * locals.var_psisd_fp4s) + (assign30810_e48687 * locals.var_psisd_fp4s_dn22))) * assign30810_e48693) - (assign30810_e48690 * ((locals.var_t0_dn22 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn22)))) / (assign30810_e48693 * assign30810_e48693)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn1, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn12, locals.var_t3_dn14, locals.var_t3_dn15, locals.var_t3_dn16, locals.var_t3_dn17, locals.var_t3_dn18, locals.var_t3_dn19, locals.var_t3_dn20, locals.var_t3_dn21, locals.var_t3_dn22,)
    }
};
        locals.var_t3 = assign30810_e48696;
        locals.var_t3_dn0 = assign30810_e48696_d_n0;
        locals.var_t3_dn1 = assign30810_e48696_d_n1;
        locals.var_t3_dn2 = assign30810_e48696_d_n2;
        locals.var_t3_dn3 = assign30810_e48696_d_n3;
        locals.var_t3_dn4 = assign30810_e48696_d_n4;
        locals.var_t3_dn5 = assign30810_e48696_d_n5;
        locals.var_t3_dn6 = assign30810_e48696_d_n6;
        locals.var_t3_dn7 = assign30810_e48696_d_n7;
        locals.var_t3_dn8 = assign30810_e48696_d_n8;
        locals.var_t3_dn9 = assign30810_e48696_d_n9;
        locals.var_t3_dn12 = assign30810_e48696_d_n12;
        locals.var_t3_dn14 = assign30810_e48696_d_n14;
        locals.var_t3_dn15 = assign30810_e48696_d_n15;
        locals.var_t3_dn16 = assign30810_e48696_d_n16;
        locals.var_t3_dn17 = assign30810_e48696_d_n17;
        locals.var_t3_dn18 = assign30810_e48696_d_n18;
        locals.var_t3_dn19 = assign30810_e48696_d_n19;
        locals.var_t3_dn20 = assign30810_e48696_d_n20;
        locals.var_t3_dn21 = assign30810_e48696_d_n21;
        locals.var_t3_dn22 = assign30810_e48696_d_n22;

        let (assign30820_e48720, assign30820_e48720_d_n0, assign30820_e48720_d_n1, assign30820_e48720_d_n2, assign30820_e48720_d_n3, assign30820_e48720_d_n4, assign30820_e48720_d_n5, assign30820_e48720_d_n6, assign30820_e48720_d_n7, assign30820_e48720_d_n8, assign30820_e48720_d_n9, assign30820_e48720_d_n12, assign30820_e48720_d_n14, assign30820_e48720_d_n15, assign30820_e48720_d_n16, assign30820_e48720_d_n17, assign30820_e48720_d_n18, assign30820_e48720_d_n19, assign30820_e48720_d_n20, assign30820_e48720_d_n21, assign30820_e48720_d_n22,) = {
    if ((locals.var_guard504 == 0.0) && (locals.var_guard513 != 0.0)) {
        let assign30820_e48703: f64 = (locals.var_cg_qme * p.p4);
        let assign30820_e48705: f64 = (assign30820_e48703 * p.p200);
        let assign30820_e48707: f64 = (assign30820_e48705 * p.p5);
        let assign30820_e48709: f64 = (assign30820_e48707 * 0.5);
        let assign30820_e48710: f64 = (-assign30820_e48709);
        let assign30820_e48713: f64 = (locals.var_vg0_fp4s - locals.var_t1);
        let assign30820_e48715: f64 = (assign30820_e48713 + locals.var_t2);
        let assign30820_e48717: f64 = (assign30820_e48715 + locals.var_t3);
        let assign30820_e48718: f64 = (assign30820_e48710 * assign30820_e48717);
        (assign30820_e48718, (((-((((locals.var_cg_qme_dn0 * p.p4) * p.p200) * p.p5) * 0.5)) * assign30820_e48717) + (assign30820_e48710 * (((locals.var_vg0_fp4s_dn0 - locals.var_t1_dn0) + locals.var_t2_dn0) + locals.var_t3_dn0))), (((-((((locals.var_cg_qme_dn1 * p.p4) * p.p200) * p.p5) * 0.5)) * assign30820_e48717) + (assign30820_e48710 * (((locals.var_vg0_fp4s_dn1 - locals.var_t1_dn1) + locals.var_t2_dn1) + locals.var_t3_dn1))), (((-((((locals.var_cg_qme_dn2 * p.p4) * p.p200) * p.p5) * 0.5)) * assign30820_e48717) + (assign30820_e48710 * (((locals.var_vg0_fp4s_dn2 - locals.var_t1_dn2) + locals.var_t2_dn2) + locals.var_t3_dn2))), (((-((((locals.var_cg_qme_dn3 * p.p4) * p.p200) * p.p5) * 0.5)) * assign30820_e48717) + (assign30820_e48710 * (((locals.var_vg0_fp4s_dn3 - locals.var_t1_dn3) + locals.var_t2_dn3) + locals.var_t3_dn3))), (((-((((locals.var_cg_qme_dn4 * p.p4) * p.p200) * p.p5) * 0.5)) * assign30820_e48717) + (assign30820_e48710 * (((locals.var_vg0_fp4s_dn4 - locals.var_t1_dn4) + locals.var_t2_dn4) + locals.var_t3_dn4))), (((-((((locals.var_cg_qme_dn5 * p.p4) * p.p200) * p.p5) * 0.5)) * assign30820_e48717) + (assign30820_e48710 * (((locals.var_vg0_fp4s_dn5 - locals.var_t1_dn5) + locals.var_t2_dn5) + locals.var_t3_dn5))), (((-((((locals.var_cg_qme_dn6 * p.p4) * p.p200) * p.p5) * 0.5)) * assign30820_e48717) + (assign30820_e48710 * (((locals.var_vg0_fp4s_dn6 - locals.var_t1_dn6) + locals.var_t2_dn6) + locals.var_t3_dn6))), (((-((((locals.var_cg_qme_dn7 * p.p4) * p.p200) * p.p5) * 0.5)) * assign30820_e48717) + (assign30820_e48710 * (((locals.var_vg0_fp4s_dn7 - locals.var_t1_dn7) + locals.var_t2_dn7) + locals.var_t3_dn7))), (((-((((locals.var_cg_qme_dn8 * p.p4) * p.p200) * p.p5) * 0.5)) * assign30820_e48717) + (assign30820_e48710 * (((locals.var_vg0_fp4s_dn8 - locals.var_t1_dn8) + locals.var_t2_dn8) + locals.var_t3_dn8))), (((-((((locals.var_cg_qme_dn9 * p.p4) * p.p200) * p.p5) * 0.5)) * assign30820_e48717) + (assign30820_e48710 * (((locals.var_vg0_fp4s_dn9 - locals.var_t1_dn9) + locals.var_t2_dn9) + locals.var_t3_dn9))), (((-((((locals.var_cg_qme_dn12 * p.p4) * p.p200) * p.p5) * 0.5)) * assign30820_e48717) + (assign30820_e48710 * (((locals.var_vg0_fp4s_dn12 - locals.var_t1_dn12) + locals.var_t2_dn12) + locals.var_t3_dn12))), (((-((((locals.var_cg_qme_dn14 * p.p4) * p.p200) * p.p5) * 0.5)) * assign30820_e48717) + (assign30820_e48710 * (((locals.var_vg0_fp4s_dn14 - locals.var_t1_dn14) + locals.var_t2_dn14) + locals.var_t3_dn14))), (((-((((locals.var_cg_qme_dn15 * p.p4) * p.p200) * p.p5) * 0.5)) * assign30820_e48717) + (assign30820_e48710 * (((locals.var_vg0_fp4s_dn15 - locals.var_t1_dn15) + locals.var_t2_dn15) + locals.var_t3_dn15))), (((-((((locals.var_cg_qme_dn16 * p.p4) * p.p200) * p.p5) * 0.5)) * assign30820_e48717) + (assign30820_e48710 * (((locals.var_vg0_fp4s_dn16 - locals.var_t1_dn16) + locals.var_t2_dn16) + locals.var_t3_dn16))), (((-((((locals.var_cg_qme_dn17 * p.p4) * p.p200) * p.p5) * 0.5)) * assign30820_e48717) + (assign30820_e48710 * (((locals.var_vg0_fp4s_dn17 - locals.var_t1_dn17) + locals.var_t2_dn17) + locals.var_t3_dn17))), (((-((((locals.var_cg_qme_dn18 * p.p4) * p.p200) * p.p5) * 0.5)) * assign30820_e48717) + (assign30820_e48710 * (((locals.var_vg0_fp4s_dn18 - locals.var_t1_dn18) + locals.var_t2_dn18) + locals.var_t3_dn18))), (((-((((locals.var_cg_qme_dn19 * p.p4) * p.p200) * p.p5) * 0.5)) * assign30820_e48717) + (assign30820_e48710 * (((locals.var_vg0_fp4s_dn19 - locals.var_t1_dn19) + locals.var_t2_dn19) + locals.var_t3_dn19))), (((-((((locals.var_cg_qme_dn20 * p.p4) * p.p200) * p.p5) * 0.5)) * assign30820_e48717) + (assign30820_e48710 * (((locals.var_vg0_fp4s_dn20 - locals.var_t1_dn20) + locals.var_t2_dn20) + locals.var_t3_dn20))), (((-((((locals.var_cg_qme_dn21 * p.p4) * p.p200) * p.p5) * 0.5)) * assign30820_e48717) + (assign30820_e48710 * (((locals.var_vg0_fp4s_dn21 - locals.var_t1_dn21) + locals.var_t2_dn21) + locals.var_t3_dn21))), (((-((((locals.var_cg_qme_dn22 * p.p4) * p.p200) * p.p5) * 0.5)) * assign30820_e48717) + (assign30820_e48710 * (((locals.var_vg0_fp4s_dn22 - locals.var_t1_dn22) + locals.var_t2_dn22) + locals.var_t3_dn22))),)
    } else {
        (locals.var_qd_fp4s, locals.var_qd_fp4s_dn0, locals.var_qd_fp4s_dn1, locals.var_qd_fp4s_dn2, locals.var_qd_fp4s_dn3, locals.var_qd_fp4s_dn4, locals.var_qd_fp4s_dn5, locals.var_qd_fp4s_dn6, locals.var_qd_fp4s_dn7, locals.var_qd_fp4s_dn8, locals.var_qd_fp4s_dn9, locals.var_qd_fp4s_dn12, locals.var_qd_fp4s_dn14, locals.var_qd_fp4s_dn15, locals.var_qd_fp4s_dn16, locals.var_qd_fp4s_dn17, locals.var_qd_fp4s_dn18, locals.var_qd_fp4s_dn19, locals.var_qd_fp4s_dn20, locals.var_qd_fp4s_dn21, locals.var_qd_fp4s_dn22,)
    }
};
        locals.var_qd_fp4s = assign30820_e48720;
        locals.var_qd_fp4s_dn0 = assign30820_e48720_d_n0;
        locals.var_qd_fp4s_dn1 = assign30820_e48720_d_n1;
        locals.var_qd_fp4s_dn2 = assign30820_e48720_d_n2;
        locals.var_qd_fp4s_dn3 = assign30820_e48720_d_n3;
        locals.var_qd_fp4s_dn4 = assign30820_e48720_d_n4;
        locals.var_qd_fp4s_dn5 = assign30820_e48720_d_n5;
        locals.var_qd_fp4s_dn6 = assign30820_e48720_d_n6;
        locals.var_qd_fp4s_dn7 = assign30820_e48720_d_n7;
        locals.var_qd_fp4s_dn8 = assign30820_e48720_d_n8;
        locals.var_qd_fp4s_dn9 = assign30820_e48720_d_n9;
        locals.var_qd_fp4s_dn12 = assign30820_e48720_d_n12;
        locals.var_qd_fp4s_dn14 = assign30820_e48720_d_n14;
        locals.var_qd_fp4s_dn15 = assign30820_e48720_d_n15;
        locals.var_qd_fp4s_dn16 = assign30820_e48720_d_n16;
        locals.var_qd_fp4s_dn17 = assign30820_e48720_d_n17;
        locals.var_qd_fp4s_dn18 = assign30820_e48720_d_n18;
        locals.var_qd_fp4s_dn19 = assign30820_e48720_d_n19;
        locals.var_qd_fp4s_dn20 = assign30820_e48720_d_n20;
        locals.var_qd_fp4s_dn21 = assign30820_e48720_d_n21;
        locals.var_qd_fp4s_dn22 = assign30820_e48720_d_n22;

        let (assign30830_e48728, assign30830_e48728_d_n0, assign30830_e48728_d_n1, assign30830_e48728_d_n2, assign30830_e48728_d_n3, assign30830_e48728_d_n4, assign30830_e48728_d_n5, assign30830_e48728_d_n6, assign30830_e48728_d_n7, assign30830_e48728_d_n8, assign30830_e48728_d_n9, assign30830_e48728_d_n12, assign30830_e48728_d_n14, assign30830_e48728_d_n15, assign30830_e48728_d_n16, assign30830_e48728_d_n17, assign30830_e48728_d_n18, assign30830_e48728_d_n19, assign30830_e48728_d_n20, assign30830_e48728_d_n21, assign30830_e48728_d_n22,) = {
    if ((locals.var_guard504 == 0.0) && (locals.var_guard513 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qg_fp4s, locals.var_qg_fp4s_dn0, locals.var_qg_fp4s_dn1, locals.var_qg_fp4s_dn2, locals.var_qg_fp4s_dn3, locals.var_qg_fp4s_dn4, locals.var_qg_fp4s_dn5, locals.var_qg_fp4s_dn6, locals.var_qg_fp4s_dn7, locals.var_qg_fp4s_dn8, locals.var_qg_fp4s_dn9, locals.var_qg_fp4s_dn12, locals.var_qg_fp4s_dn14, locals.var_qg_fp4s_dn15, locals.var_qg_fp4s_dn16, locals.var_qg_fp4s_dn17, locals.var_qg_fp4s_dn18, locals.var_qg_fp4s_dn19, locals.var_qg_fp4s_dn20, locals.var_qg_fp4s_dn21, locals.var_qg_fp4s_dn22,)
    }
};
        locals.var_qg_fp4s = assign30830_e48728;
        locals.var_qg_fp4s_dn0 = assign30830_e48728_d_n0;
        locals.var_qg_fp4s_dn1 = assign30830_e48728_d_n1;
        locals.var_qg_fp4s_dn2 = assign30830_e48728_d_n2;
        locals.var_qg_fp4s_dn3 = assign30830_e48728_d_n3;
        locals.var_qg_fp4s_dn4 = assign30830_e48728_d_n4;
        locals.var_qg_fp4s_dn5 = assign30830_e48728_d_n5;
        locals.var_qg_fp4s_dn6 = assign30830_e48728_d_n6;
        locals.var_qg_fp4s_dn7 = assign30830_e48728_d_n7;
        locals.var_qg_fp4s_dn8 = assign30830_e48728_d_n8;
        locals.var_qg_fp4s_dn9 = assign30830_e48728_d_n9;
        locals.var_qg_fp4s_dn12 = assign30830_e48728_d_n12;
        locals.var_qg_fp4s_dn14 = assign30830_e48728_d_n14;
        locals.var_qg_fp4s_dn15 = assign30830_e48728_d_n15;
        locals.var_qg_fp4s_dn16 = assign30830_e48728_d_n16;
        locals.var_qg_fp4s_dn17 = assign30830_e48728_d_n17;
        locals.var_qg_fp4s_dn18 = assign30830_e48728_d_n18;
        locals.var_qg_fp4s_dn19 = assign30830_e48728_d_n19;
        locals.var_qg_fp4s_dn20 = assign30830_e48728_d_n20;
        locals.var_qg_fp4s_dn21 = assign30830_e48728_d_n21;
        locals.var_qg_fp4s_dn22 = assign30830_e48728_d_n22;

        let (assign30840_e48736, assign30840_e48736_d_n0, assign30840_e48736_d_n1, assign30840_e48736_d_n2, assign30840_e48736_d_n3, assign30840_e48736_d_n4, assign30840_e48736_d_n5, assign30840_e48736_d_n6, assign30840_e48736_d_n7, assign30840_e48736_d_n8, assign30840_e48736_d_n9, assign30840_e48736_d_n12, assign30840_e48736_d_n14, assign30840_e48736_d_n15, assign30840_e48736_d_n16, assign30840_e48736_d_n17, assign30840_e48736_d_n18, assign30840_e48736_d_n19, assign30840_e48736_d_n20, assign30840_e48736_d_n21, assign30840_e48736_d_n22,) = {
    if ((locals.var_guard504 == 0.0) && (locals.var_guard513 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qd_fp4s, locals.var_qd_fp4s_dn0, locals.var_qd_fp4s_dn1, locals.var_qd_fp4s_dn2, locals.var_qd_fp4s_dn3, locals.var_qd_fp4s_dn4, locals.var_qd_fp4s_dn5, locals.var_qd_fp4s_dn6, locals.var_qd_fp4s_dn7, locals.var_qd_fp4s_dn8, locals.var_qd_fp4s_dn9, locals.var_qd_fp4s_dn12, locals.var_qd_fp4s_dn14, locals.var_qd_fp4s_dn15, locals.var_qd_fp4s_dn16, locals.var_qd_fp4s_dn17, locals.var_qd_fp4s_dn18, locals.var_qd_fp4s_dn19, locals.var_qd_fp4s_dn20, locals.var_qd_fp4s_dn21, locals.var_qd_fp4s_dn22,)
    }
};
        locals.var_qd_fp4s = assign30840_e48736;
        locals.var_qd_fp4s_dn0 = assign30840_e48736_d_n0;
        locals.var_qd_fp4s_dn1 = assign30840_e48736_d_n1;
        locals.var_qd_fp4s_dn2 = assign30840_e48736_d_n2;
        locals.var_qd_fp4s_dn3 = assign30840_e48736_d_n3;
        locals.var_qd_fp4s_dn4 = assign30840_e48736_d_n4;
        locals.var_qd_fp4s_dn5 = assign30840_e48736_d_n5;
        locals.var_qd_fp4s_dn6 = assign30840_e48736_d_n6;
        locals.var_qd_fp4s_dn7 = assign30840_e48736_d_n7;
        locals.var_qd_fp4s_dn8 = assign30840_e48736_d_n8;
        locals.var_qd_fp4s_dn9 = assign30840_e48736_d_n9;
        locals.var_qd_fp4s_dn12 = assign30840_e48736_d_n12;
        locals.var_qd_fp4s_dn14 = assign30840_e48736_d_n14;
        locals.var_qd_fp4s_dn15 = assign30840_e48736_d_n15;
        locals.var_qd_fp4s_dn16 = assign30840_e48736_d_n16;
        locals.var_qd_fp4s_dn17 = assign30840_e48736_d_n17;
        locals.var_qd_fp4s_dn18 = assign30840_e48736_d_n18;
        locals.var_qd_fp4s_dn19 = assign30840_e48736_d_n19;
        locals.var_qd_fp4s_dn20 = assign30840_e48736_d_n20;
        locals.var_qd_fp4s_dn21 = assign30840_e48736_d_n21;
        locals.var_qd_fp4s_dn22 = assign30840_e48736_d_n22;

        let assign30990_e48883: f64 = if p.p255 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard524 = assign30990_e48883;

        let (assign31000_e48893, assign31000_e48893_d_n1, assign31000_e48893_d_n2, assign31000_e48893_d_n10,) = {
    if (locals.var_guard524 != 0.0) {
        let assign31000_e48887: f64 = (p.p4 * p.p5);
        let assign31000_e48889: f64 = (assign31000_e48887 * p.p210);
        let assign31000_e48891: f64 = (assign31000_e48889 * (nv10 - nv2));
        (assign31000_e48891, 0.0, (-assign31000_e48889), assign31000_e48889,)
    } else {
        (locals.var_qsov, locals.var_qsov_dn1, locals.var_qsov_dn2, locals.var_qsov_dn10,)
    }
};
        locals.var_qsov = assign31000_e48893;
        locals.var_qsov_dn1 = assign31000_e48893_d_n1;
        locals.var_qsov_dn2 = assign31000_e48893_d_n2;
        locals.var_qsov_dn10 = assign31000_e48893_d_n10;

        let (assign31010_e48908, assign31010_e48908_d_n0, assign31010_e48908_d_n2,) = {
    if (locals.var_guard524 != 0.0) {
        let assign31010_e48897: f64 = ((nv0 - nv2) * p.p214);
        let assign31010_e48900: f64 = ((nv0 - nv2) * (nv0 - nv2));
        let assign31010_e48903: f64 = (p.p214 * p.p214);
        let assign31010_e48904: f64 = (assign31010_e48900 + assign31010_e48903);
        let assign31010_e48905: f64 = (assign31010_e48904).sqrt();
        let assign31010_e48906: f64 = (assign31010_e48897 / assign31010_e48905);
        (assign31010_e48906, (((p.p214 * assign31010_e48905) - (assign31010_e48897 * (((nv0 - nv2) + (nv0 - nv2)) / (2.0 * assign31010_e48905)))) / (assign31010_e48905 * assign31010_e48905)), ((((-p.p214) * assign31010_e48905) - (assign31010_e48897 * (((-(nv0 - nv2)) + (-(nv0 - nv2))) / (2.0 * assign31010_e48905)))) / (assign31010_e48905 * assign31010_e48905)),)
    } else {
        (locals.var_vdseffcv, locals.var_vdseffcv_dn0, locals.var_vdseffcv_dn2,)
    }
};
        locals.var_vdseffcv = assign31010_e48908;
        locals.var_vdseffcv_dn0 = assign31010_e48908_d_n0;
        locals.var_vdseffcv_dn2 = assign31010_e48908_d_n2;

        let (assign31020_e48918,) = {
    if (locals.var_guard524 != 0.0) {
        let assign31020_e48914: f64 = (2.0 * p.p214);
        let assign31020_e48915: f64 = (p.p211 / assign31020_e48914);
        let assign31020_e48916: f64 = (p.p213).min(assign31020_e48915);
        (assign31020_e48916,)
    } else {
        (locals.var_cgdl_l,)
    }
};
        locals.var_cgdl_l = assign31020_e48918;

        let (assign31030_e48934, assign31030_e48934_d_n0, assign31030_e48934_d_n2,) = {
    if (locals.var_guard524 != 0.0) {
        let assign31030_e48922: f64 = (p.p4 * p.p5);
        let assign31030_e48924: f64 = (assign31030_e48922 * p.p211);
        let assign31030_e48927: f64 = (p.p4 * p.p5);
        let assign31030_e48929: f64 = (assign31030_e48927 * locals.var_cgdl_l);
        let assign31030_e48931: f64 = (assign31030_e48929 * locals.var_vdseffcv);
        let assign31030_e48932: f64 = (assign31030_e48924 - assign31030_e48931);
        (assign31030_e48932, (-(assign31030_e48929 * locals.var_vdseffcv_dn0)), (-(assign31030_e48929 * locals.var_vdseffcv_dn2)),)
    } else {
        (locals.var_cgdvar, locals.var_cgdvar_dn0, locals.var_cgdvar_dn2,)
    }
};
        locals.var_cgdvar = assign31030_e48934;
        locals.var_cgdvar_dn0 = assign31030_e48934_d_n0;
        locals.var_cgdvar_dn2 = assign31030_e48934_d_n2;

        let (assign31040_e48942, assign31040_e48942_d_n0, assign31040_e48942_d_n1, assign31040_e48942_d_n2, assign31040_e48942_d_n10,) = {
    if (locals.var_guard524 != 0.0) {
        let assign31040_e48938: f64 = (locals.var_cgdvar).max(0.0);
        let assign31040_e48940: f64 = (assign31040_e48938 * (nv10 - nv0));
        (assign31040_e48940, ((if locals.var_cgdvar >= 0.0 { locals.var_cgdvar_dn0 } else { 0.0 } * (nv10 - nv0)) + (-assign31040_e48938)), 0.0, (if locals.var_cgdvar >= 0.0 { locals.var_cgdvar_dn2 } else { 0.0 } * (nv10 - nv0)), assign31040_e48938,)
    } else {
        (locals.var_qdov, locals.var_qdov_dn0, locals.var_qdov_dn1, locals.var_qdov_dn2, locals.var_qdov_dn10,)
    }
};
        locals.var_qdov = assign31040_e48942;
        locals.var_qdov_dn0 = assign31040_e48942_d_n0;
        locals.var_qdov_dn1 = assign31040_e48942_d_n1;
        locals.var_qdov_dn2 = assign31040_e48942_d_n2;
        locals.var_qdov_dn10 = assign31040_e48942_d_n10;

        let (assign31050_e48953, assign31050_e48953_d_n1, assign31050_e48953_d_n2, assign31050_e48953_d_n10,) = {
    if (locals.var_guard524 == 0.0) {
        let assign31050_e48947: f64 = (p.p4 * p.p5);
        let assign31050_e48949: f64 = (assign31050_e48947 * p.p210);
        let assign31050_e48951: f64 = (assign31050_e48949 * (nv1 - nv2));
        (assign31050_e48951, assign31050_e48949, (-assign31050_e48949), 0.0,)
    } else {
        (locals.var_qsov, locals.var_qsov_dn1, locals.var_qsov_dn2, locals.var_qsov_dn10,)
    }
};
        locals.var_qsov = assign31050_e48953;
        locals.var_qsov_dn1 = assign31050_e48953_d_n1;
        locals.var_qsov_dn2 = assign31050_e48953_d_n2;
        locals.var_qsov_dn10 = assign31050_e48953_d_n10;

        let (assign31060_e48969, assign31060_e48969_d_n0, assign31060_e48969_d_n2,) = {
    if (locals.var_guard524 == 0.0) {
        let assign31060_e48958: f64 = ((nv0 - nv2) * p.p214);
        let assign31060_e48961: f64 = ((nv0 - nv2) * (nv0 - nv2));
        let assign31060_e48964: f64 = (p.p214 * p.p214);
        let assign31060_e48965: f64 = (assign31060_e48961 + assign31060_e48964);
        let assign31060_e48966: f64 = (assign31060_e48965).sqrt();
        let assign31060_e48967: f64 = (assign31060_e48958 / assign31060_e48966);
        (assign31060_e48967, (((p.p214 * assign31060_e48966) - (assign31060_e48958 * (((nv0 - nv2) + (nv0 - nv2)) / (2.0 * assign31060_e48966)))) / (assign31060_e48966 * assign31060_e48966)), ((((-p.p214) * assign31060_e48966) - (assign31060_e48958 * (((-(nv0 - nv2)) + (-(nv0 - nv2))) / (2.0 * assign31060_e48966)))) / (assign31060_e48966 * assign31060_e48966)),)
    } else {
        (locals.var_vdseffcv, locals.var_vdseffcv_dn0, locals.var_vdseffcv_dn2,)
    }
};
        locals.var_vdseffcv = assign31060_e48969;
        locals.var_vdseffcv_dn0 = assign31060_e48969_d_n0;
        locals.var_vdseffcv_dn2 = assign31060_e48969_d_n2;

        let (assign31070_e48980,) = {
    if (locals.var_guard524 == 0.0) {
        let assign31070_e48976: f64 = (2.0 * p.p214);
        let assign31070_e48977: f64 = (p.p211 / assign31070_e48976);
        let assign31070_e48978: f64 = (p.p213).min(assign31070_e48977);
        (assign31070_e48978,)
    } else {
        (locals.var_cgdl_l,)
    }
};
        locals.var_cgdl_l = assign31070_e48980;

        let (assign31080_e48997, assign31080_e48997_d_n0, assign31080_e48997_d_n2,) = {
    if (locals.var_guard524 == 0.0) {
        let assign31080_e48985: f64 = (p.p4 * p.p5);
        let assign31080_e48987: f64 = (assign31080_e48985 * p.p211);
        let assign31080_e48990: f64 = (p.p4 * p.p5);
        let assign31080_e48992: f64 = (assign31080_e48990 * locals.var_cgdl_l);
        let assign31080_e48994: f64 = (assign31080_e48992 * locals.var_vdseffcv);
        let assign31080_e48995: f64 = (assign31080_e48987 - assign31080_e48994);
        (assign31080_e48995, (-(assign31080_e48992 * locals.var_vdseffcv_dn0)), (-(assign31080_e48992 * locals.var_vdseffcv_dn2)),)
    } else {
        (locals.var_cgdvar, locals.var_cgdvar_dn0, locals.var_cgdvar_dn2,)
    }
};
        locals.var_cgdvar = assign31080_e48997;
        locals.var_cgdvar_dn0 = assign31080_e48997_d_n0;
        locals.var_cgdvar_dn2 = assign31080_e48997_d_n2;

        let (assign31090_e49006, assign31090_e49006_d_n0, assign31090_e49006_d_n1, assign31090_e49006_d_n2, assign31090_e49006_d_n10,) = {
    if (locals.var_guard524 == 0.0) {
        let assign31090_e49002: f64 = (locals.var_cgdvar).max(0.0);
        let assign31090_e49004: f64 = (assign31090_e49002 * (nv1 - nv0));
        (assign31090_e49004, ((if locals.var_cgdvar >= 0.0 { locals.var_cgdvar_dn0 } else { 0.0 } * (nv1 - nv0)) + (-assign31090_e49002)), assign31090_e49002, (if locals.var_cgdvar >= 0.0 { locals.var_cgdvar_dn2 } else { 0.0 } * (nv1 - nv0)), 0.0,)
    } else {
        (locals.var_qdov, locals.var_qdov_dn0, locals.var_qdov_dn1, locals.var_qdov_dn2, locals.var_qdov_dn10,)
    }
};
        locals.var_qdov = assign31090_e49006;
        locals.var_qdov_dn0 = assign31090_e49006_d_n0;
        locals.var_qdov_dn1 = assign31090_e49006_d_n1;
        locals.var_qdov_dn2 = assign31090_e49006_d_n2;
        locals.var_qdov_dn10 = assign31090_e49006_d_n10;

        let assign31100_e49009: f64 = (p.p4 * p.p5);
        let assign31100_e49011: f64 = (assign31100_e49009 * p.p212);
        let assign31100_e49013: f64 = (assign31100_e49011 * (nv0 - nv2));
        locals.var_qdsov = assign31100_e49013;
        locals.var_qdsov_dn0 = assign31100_e49011;
        locals.var_qdsov_dn2 = (-assign31100_e49011);

        let assign31150_e49030: f64 = (p.p4 * p.p5);
        let assign31150_e49032: f64 = (assign31150_e49030 * p.p215);
        let assign31150_e49034: f64 = (assign31150_e49032 * (nv3 - nv0));
        locals.var_qbdov = assign31150_e49034;
        locals.var_qbdov_dn0 = (-assign31150_e49032);
        locals.var_qbdov_dn3 = assign31150_e49032;

        let assign31160_e49037: f64 = (p.p4 * p.p5);
        let assign31160_e49039: f64 = (assign31160_e49037 * p.p216);
        let assign31160_e49041: f64 = (assign31160_e49039 * (nv3 - nv2));
        locals.var_qbsov = assign31160_e49041;
        locals.var_qbsov_dn2 = (-assign31160_e49039);
        locals.var_qbsov_dn3 = assign31160_e49039;

        let assign31170_e49044: f64 = (p.p4 * p.p5);
        let assign31170_e49046: f64 = (assign31170_e49044 * p.p217);
        let assign31170_e49048: f64 = (assign31170_e49046 * (nv3 - nv1));
        locals.var_qbgov = assign31170_e49048;
        locals.var_qbgov_dn1 = (-assign31170_e49046);
        locals.var_qbgov_dn3 = assign31170_e49046;

        let assign31180_e49052: f64 = (locals.var_tdev / locals.var_tnom);
        let assign31180_e49054: f64 = (assign31180_e49052 - 1.0);
        let assign31180_e49056: f64 = (assign31180_e49054 * p.p285);
        let assign31180_e49057: f64 = (p.p279 + assign31180_e49056);
        locals.var_vbisb_t = assign31180_e49057;
        locals.var_vbisb_t_dn4 = ((locals.var_tdev_dn4 / locals.var_tnom) * p.p285);

        let assign31190_e49061: f64 = (locals.var_tdev / locals.var_tnom);
        let assign31190_e49063: f64 = (assign31190_e49061 - 1.0);
        let assign31190_e49065: f64 = (assign31190_e49063 * p.p283);
        let assign31190_e49066: f64 = (p.p275 + assign31190_e49065);
        locals.var_nsb_t = assign31190_e49066;
        locals.var_nsb_t_dn4 = ((locals.var_tdev_dn4 / locals.var_tnom) * p.p283);

        let assign31200_e49071: f64 = (locals.var_tdev / locals.var_tnom);
        let assign31200_e49073: f64 = (assign31200_e49071 - 1.0);
        let assign31200_e49074: f64 = (p.p281 * assign31200_e49073);
        let assign31200_e49075: f64 = (assign31200_e49074).exp();
        let assign31200_e49076: f64 = (p.p277 * assign31200_e49075);
        locals.var_isb_t = assign31200_e49076;
        locals.var_isb_t_dn4 = (p.p277 * (assign31200_e49075 * (p.p281 * (locals.var_tdev_dn4 / locals.var_tnom))));

        let assign31210_e49080: f64 = (locals.var_tdev / locals.var_tnom);
        let assign31210_e49082: f64 = (assign31210_e49080 - 1.0);
        let assign31210_e49084: f64 = (assign31210_e49082 * p.p286);
        let assign31210_e49085: f64 = (p.p280 + assign31210_e49084);
        locals.var_vbidb_t = assign31210_e49085;
        locals.var_vbidb_t_dn4 = ((locals.var_tdev_dn4 / locals.var_tnom) * p.p286);

    }

    pub(super) fn stamp_transient_block_182(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let assign31220_e49089: f64 = (locals.var_tdev / locals.var_tnom);
        let assign31220_e49091: f64 = (assign31220_e49089 - 1.0);
        let assign31220_e49093: f64 = (assign31220_e49091 * p.p284);
        let assign31220_e49094: f64 = (p.p276 + assign31220_e49093);
        locals.var_ndb_t = assign31220_e49094;
        locals.var_ndb_t_dn4 = ((locals.var_tdev_dn4 / locals.var_tnom) * p.p284);

        let assign31230_e49099: f64 = (locals.var_tdev / locals.var_tnom);
        let assign31230_e49101: f64 = (assign31230_e49099 - 1.0);
        let assign31230_e49102: f64 = (p.p282 * assign31230_e49101);
        let assign31230_e49103: f64 = (assign31230_e49102).exp();
        let assign31230_e49104: f64 = (p.p278 * assign31230_e49103);
        locals.var_idb_t = assign31230_e49104;
        locals.var_idb_t_dn4 = (p.p278 * (assign31230_e49103 * (p.p282 * (locals.var_tdev_dn4 / locals.var_tnom))));

        let assign31240_e49107: f64 = (p.p4 * p.p5);
        let assign31240_e49109: f64 = (assign31240_e49107 * locals.var_idb_t);
        locals.var_t3 = assign31240_e49109;
        locals.var_t3_dn0 = 0.0;
        locals.var_t3_dn1 = 0.0;
        locals.var_t3_dn2 = 0.0;
        locals.var_t3_dn3 = 0.0;
        locals.var_t3_dn4 = (assign31240_e49107 * locals.var_idb_t_dn4);
        locals.var_t3_dn5 = 0.0;
        locals.var_t3_dn6 = 0.0;
        locals.var_t3_dn7 = 0.0;
        locals.var_t3_dn8 = 0.0;
        locals.var_t3_dn9 = 0.0;
        locals.var_t3_dn12 = 0.0;
        locals.var_t3_dn14 = 0.0;
        locals.var_t3_dn15 = 0.0;
        locals.var_t3_dn16 = 0.0;
        locals.var_t3_dn17 = 0.0;
        locals.var_t3_dn18 = 0.0;
        locals.var_t3_dn19 = 0.0;
        locals.var_t3_dn20 = 0.0;
        locals.var_t3_dn21 = 0.0;
        locals.var_t3_dn22 = 0.0;

        let assign31250_e49112: f64 = ((nv0 - nv3) - locals.var_vbidb_t);
        let assign31250_e49114: f64 = (assign31250_e49112).max(0.0);
        locals.var_vbdl = assign31250_e49114;
        locals.var_vbdl_dn0 = if assign31250_e49112 >= 0.0 { 1.0 } else { 0.0 };
        locals.var_vbdl_dn3 = if assign31250_e49112 >= 0.0 { -1.0 } else { 0.0 };
        locals.var_vbdl_dn4 = if assign31250_e49112 >= 0.0 { (-locals.var_vbidb_t_dn4) } else { 0.0 };

        let assign31260_e49117: f64 = if locals.var_t3 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard525 = assign31260_e49117;

        let assign31270_e49120: f64 = if locals.var_vbdl > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard526 = assign31270_e49120;

        let (assign31280_e49132, assign31280_e49132_d_n0, assign31280_e49132_d_n2, assign31280_e49132_d_n3, assign31280_e49132_d_n4, assign31280_e49132_d_n7, assign31280_e49132_d_n8, assign31280_e49132_d_n9,) = {
    if ((locals.var_guard525 != 0.0) && (locals.var_guard526 != 0.0)) {
        let assign31280_e49126: f64 = (locals.var_vbdl).powf(1.0);
        let assign31280_e49129: f64 = (locals.var_ndb_t * locals.var_vth);
        let assign31280_e49130: f64 = (assign31280_e49126 / assign31280_e49129);
        (assign31280_e49130, (if 0.0 == 0.0 && ((1.0) as f64).is_finite() && ((1.0) as f64).fract() == 0.0 { if 1.0 == 0.0 { 0.0 } else { ((locals.var_vbdl).powf(1.0 - 1.0) * locals.var_vbdl_dn0) } } else { (assign31280_e49126 * (locals.var_vbdl_dn0 / locals.var_vbdl)) } / assign31280_e49129), 0.0, (if 0.0 == 0.0 && ((1.0) as f64).is_finite() && ((1.0) as f64).fract() == 0.0 { if 1.0 == 0.0 { 0.0 } else { ((locals.var_vbdl).powf(1.0 - 1.0) * locals.var_vbdl_dn3) } } else { (assign31280_e49126 * (locals.var_vbdl_dn3 / locals.var_vbdl)) } / assign31280_e49129), (((if 0.0 == 0.0 && ((1.0) as f64).is_finite() && ((1.0) as f64).fract() == 0.0 { if 1.0 == 0.0 { 0.0 } else { ((locals.var_vbdl).powf(1.0 - 1.0) * locals.var_vbdl_dn4) } } else { (assign31280_e49126 * (locals.var_vbdl_dn4 / locals.var_vbdl)) } * assign31280_e49129) - (assign31280_e49126 * ((locals.var_ndb_t_dn4 * locals.var_vth) + (locals.var_ndb_t * locals.var_vth_dn4)))) / (assign31280_e49129 * assign31280_e49129)), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn3, locals.var_arg_dn4, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9,)
    }
};
        locals.var_arg = assign31280_e49132;
        locals.var_arg_dn0 = assign31280_e49132_d_n0;
        locals.var_arg_dn2 = assign31280_e49132_d_n2;
        locals.var_arg_dn3 = assign31280_e49132_d_n3;
        locals.var_arg_dn4 = assign31280_e49132_d_n4;
        locals.var_arg_dn7 = assign31280_e49132_d_n7;
        locals.var_arg_dn8 = assign31280_e49132_d_n8;
        locals.var_arg_dn9 = assign31280_e49132_d_n9;

        let assign31290_e49135: f64 = if locals.var_arg > 80.0 { 1.0 } else { 0.0 };
        locals.var_guard527 = assign31290_e49135;

        let (assign31300_e49147, assign31300_e49147_d_n0, assign31300_e49147_d_n2, assign31300_e49147_d_n3, assign31300_e49147_d_n4, assign31300_e49147_d_n7, assign31300_e49147_d_n8, assign31300_e49147_d_n9,) = {
    if (((locals.var_guard525 != 0.0) && (locals.var_guard526 != 0.0)) && (locals.var_guard527 != 0.0)) {
        let assign31300_e49144: f64 = (locals.var_arg - 80.0);
        let assign31300_e49145: f64 = (1.0 + assign31300_e49144);
        (assign31300_e49145, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn3, locals.var_arg_dn4, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9,)
    } else {
        (locals.var_le, locals.var_le_dn0, locals.var_le_dn2, locals.var_le_dn3, locals.var_le_dn4, locals.var_le_dn7, locals.var_le_dn8, locals.var_le_dn9,)
    }
};
        locals.var_le = assign31300_e49147;
        locals.var_le_dn0 = assign31300_e49147_d_n0;
        locals.var_le_dn2 = assign31300_e49147_d_n2;
        locals.var_le_dn3 = assign31300_e49147_d_n3;
        locals.var_le_dn4 = assign31300_e49147_d_n4;
        locals.var_le_dn7 = assign31300_e49147_d_n7;
        locals.var_le_dn8 = assign31300_e49147_d_n8;
        locals.var_le_dn9 = assign31300_e49147_d_n9;

        let (assign31310_e49155, assign31310_e49155_d_n0, assign31310_e49155_d_n2, assign31310_e49155_d_n3, assign31310_e49155_d_n4, assign31310_e49155_d_n7, assign31310_e49155_d_n8, assign31310_e49155_d_n9,) = {
    if (((locals.var_guard525 != 0.0) && (locals.var_guard526 != 0.0)) && (locals.var_guard527 != 0.0)) {
        (80.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn3, locals.var_arg_dn4, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9,)
    }
};
        locals.var_arg = assign31310_e49155;
        locals.var_arg_dn0 = assign31310_e49155_d_n0;
        locals.var_arg_dn2 = assign31310_e49155_d_n2;
        locals.var_arg_dn3 = assign31310_e49155_d_n3;
        locals.var_arg_dn4 = assign31310_e49155_d_n4;
        locals.var_arg_dn7 = assign31310_e49155_d_n7;
        locals.var_arg_dn8 = assign31310_e49155_d_n8;
        locals.var_arg_dn9 = assign31310_e49155_d_n9;

        let (assign31320_e49164, assign31320_e49164_d_n0, assign31320_e49164_d_n2, assign31320_e49164_d_n3, assign31320_e49164_d_n4, assign31320_e49164_d_n7, assign31320_e49164_d_n8, assign31320_e49164_d_n9,) = {
    if (((locals.var_guard525 != 0.0) && (locals.var_guard526 != 0.0)) && (locals.var_guard527 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_le, locals.var_le_dn0, locals.var_le_dn2, locals.var_le_dn3, locals.var_le_dn4, locals.var_le_dn7, locals.var_le_dn8, locals.var_le_dn9,)
    }
};
        locals.var_le = assign31320_e49164;
        locals.var_le_dn0 = assign31320_e49164_d_n0;
        locals.var_le_dn2 = assign31320_e49164_d_n2;
        locals.var_le_dn3 = assign31320_e49164_d_n3;
        locals.var_le_dn4 = assign31320_e49164_d_n4;
        locals.var_le_dn7 = assign31320_e49164_d_n7;
        locals.var_le_dn8 = assign31320_e49164_d_n8;
        locals.var_le_dn9 = assign31320_e49164_d_n9;

        let (assign31330_e49173, assign31330_e49173_d_n0, assign31330_e49173_d_n2, assign31330_e49173_d_n3, assign31330_e49173_d_n4, assign31330_e49173_d_n7, assign31330_e49173_d_n8, assign31330_e49173_d_n9,) = {
    if ((locals.var_guard525 != 0.0) && (locals.var_guard526 != 0.0)) {
        let assign31330_e49170: f64 = (locals.var_arg).exp();
        let assign31330_e49171: f64 = (locals.var_le * assign31330_e49170);
        (assign31330_e49171, ((locals.var_le_dn0 * assign31330_e49170) + (locals.var_le * (assign31330_e49170 * locals.var_arg_dn0))), ((locals.var_le_dn2 * assign31330_e49170) + (locals.var_le * (assign31330_e49170 * locals.var_arg_dn2))), ((locals.var_le_dn3 * assign31330_e49170) + (locals.var_le * (assign31330_e49170 * locals.var_arg_dn3))), ((locals.var_le_dn4 * assign31330_e49170) + (locals.var_le * (assign31330_e49170 * locals.var_arg_dn4))), ((locals.var_le_dn7 * assign31330_e49170) + (locals.var_le * (assign31330_e49170 * locals.var_arg_dn7))), ((locals.var_le_dn8 * assign31330_e49170) + (locals.var_le * (assign31330_e49170 * locals.var_arg_dn8))), ((locals.var_le_dn9 * assign31330_e49170) + (locals.var_le * (assign31330_e49170 * locals.var_arg_dn9))),)
    } else {
        (locals.var_le, locals.var_le_dn0, locals.var_le_dn2, locals.var_le_dn3, locals.var_le_dn4, locals.var_le_dn7, locals.var_le_dn8, locals.var_le_dn9,)
    }
};
        locals.var_le = assign31330_e49173;
        locals.var_le_dn0 = assign31330_e49173_d_n0;
        locals.var_le_dn2 = assign31330_e49173_d_n2;
        locals.var_le_dn3 = assign31330_e49173_d_n3;
        locals.var_le_dn4 = assign31330_e49173_d_n4;
        locals.var_le_dn7 = assign31330_e49173_d_n7;
        locals.var_le_dn8 = assign31330_e49173_d_n8;
        locals.var_le_dn9 = assign31330_e49173_d_n9;

        let (assign31340_e49183, assign31340_e49183_d_n0, assign31340_e49183_d_n1, assign31340_e49183_d_n2, assign31340_e49183_d_n3, assign31340_e49183_d_n4, assign31340_e49183_d_n5, assign31340_e49183_d_n6, assign31340_e49183_d_n7, assign31340_e49183_d_n8, assign31340_e49183_d_n9, assign31340_e49183_d_n12, assign31340_e49183_d_n14, assign31340_e49183_d_n15, assign31340_e49183_d_n16, assign31340_e49183_d_n17, assign31340_e49183_d_n18, assign31340_e49183_d_n19, assign31340_e49183_d_n20, assign31340_e49183_d_n21, assign31340_e49183_d_n22,) = {
    if ((locals.var_guard525 != 0.0) && (locals.var_guard526 != 0.0)) {
        let assign31340_e49180: f64 = (locals.var_le - 1.0);
        let assign31340_e49181: f64 = (locals.var_t3 * assign31340_e49180);
        (assign31340_e49181, ((locals.var_t3_dn0 * assign31340_e49180) + (locals.var_t3 * locals.var_le_dn0)), (locals.var_t3_dn1 * assign31340_e49180), ((locals.var_t3_dn2 * assign31340_e49180) + (locals.var_t3 * locals.var_le_dn2)), ((locals.var_t3_dn3 * assign31340_e49180) + (locals.var_t3 * locals.var_le_dn3)), ((locals.var_t3_dn4 * assign31340_e49180) + (locals.var_t3 * locals.var_le_dn4)), (locals.var_t3_dn5 * assign31340_e49180), (locals.var_t3_dn6 * assign31340_e49180), ((locals.var_t3_dn7 * assign31340_e49180) + (locals.var_t3 * locals.var_le_dn7)), ((locals.var_t3_dn8 * assign31340_e49180) + (locals.var_t3 * locals.var_le_dn8)), ((locals.var_t3_dn9 * assign31340_e49180) + (locals.var_t3 * locals.var_le_dn9)), (locals.var_t3_dn12 * assign31340_e49180), (locals.var_t3_dn14 * assign31340_e49180), (locals.var_t3_dn15 * assign31340_e49180), (locals.var_t3_dn16 * assign31340_e49180), (locals.var_t3_dn17 * assign31340_e49180), (locals.var_t3_dn18 * assign31340_e49180), (locals.var_t3_dn19 * assign31340_e49180), (locals.var_t3_dn20 * assign31340_e49180), (locals.var_t3_dn21 * assign31340_e49180), (locals.var_t3_dn22 * assign31340_e49180),)
    } else {
        (locals.var_idb, locals.var_idb_dn0, locals.var_idb_dn1, locals.var_idb_dn2, locals.var_idb_dn3, locals.var_idb_dn4, locals.var_idb_dn5, locals.var_idb_dn6, locals.var_idb_dn7, locals.var_idb_dn8, locals.var_idb_dn9, locals.var_idb_dn12, locals.var_idb_dn14, locals.var_idb_dn15, locals.var_idb_dn16, locals.var_idb_dn17, locals.var_idb_dn18, locals.var_idb_dn19, locals.var_idb_dn20, locals.var_idb_dn21, locals.var_idb_dn22,)
    }
};
        locals.var_idb = assign31340_e49183;
        locals.var_idb_dn0 = assign31340_e49183_d_n0;
        locals.var_idb_dn1 = assign31340_e49183_d_n1;
        locals.var_idb_dn2 = assign31340_e49183_d_n2;
        locals.var_idb_dn3 = assign31340_e49183_d_n3;
        locals.var_idb_dn4 = assign31340_e49183_d_n4;
        locals.var_idb_dn5 = assign31340_e49183_d_n5;
        locals.var_idb_dn6 = assign31340_e49183_d_n6;
        locals.var_idb_dn7 = assign31340_e49183_d_n7;
        locals.var_idb_dn8 = assign31340_e49183_d_n8;
        locals.var_idb_dn9 = assign31340_e49183_d_n9;
        locals.var_idb_dn12 = assign31340_e49183_d_n12;
        locals.var_idb_dn14 = assign31340_e49183_d_n14;
        locals.var_idb_dn15 = assign31340_e49183_d_n15;
        locals.var_idb_dn16 = assign31340_e49183_d_n16;
        locals.var_idb_dn17 = assign31340_e49183_d_n17;
        locals.var_idb_dn18 = assign31340_e49183_d_n18;
        locals.var_idb_dn19 = assign31340_e49183_d_n19;
        locals.var_idb_dn20 = assign31340_e49183_d_n20;
        locals.var_idb_dn21 = assign31340_e49183_d_n21;
        locals.var_idb_dn22 = assign31340_e49183_d_n22;

        let (assign31350_e49194, assign31350_e49194_d_n0, assign31350_e49194_d_n2, assign31350_e49194_d_n3, assign31350_e49194_d_n4, assign31350_e49194_d_n7, assign31350_e49194_d_n8, assign31350_e49194_d_n9,) = {
    if ((locals.var_guard525 != 0.0) && (locals.var_guard526 == 0.0)) {
        let assign31350_e49191: f64 = (locals.var_ndb_t * locals.var_vth);
        let assign31350_e49192: f64 = (locals.var_vbdl / assign31350_e49191);
        (assign31350_e49192, (locals.var_vbdl_dn0 / assign31350_e49191), 0.0, (locals.var_vbdl_dn3 / assign31350_e49191), (((locals.var_vbdl_dn4 * assign31350_e49191) - (locals.var_vbdl * ((locals.var_ndb_t_dn4 * locals.var_vth) + (locals.var_ndb_t * locals.var_vth_dn4)))) / (assign31350_e49191 * assign31350_e49191)), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn3, locals.var_arg_dn4, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9,)
    }
};
        locals.var_arg = assign31350_e49194;
        locals.var_arg_dn0 = assign31350_e49194_d_n0;
        locals.var_arg_dn2 = assign31350_e49194_d_n2;
        locals.var_arg_dn3 = assign31350_e49194_d_n3;
        locals.var_arg_dn4 = assign31350_e49194_d_n4;
        locals.var_arg_dn7 = assign31350_e49194_d_n7;
        locals.var_arg_dn8 = assign31350_e49194_d_n8;
        locals.var_arg_dn9 = assign31350_e49194_d_n9;

        let assign31360_e49197: f64 = if locals.var_arg > 80.0 { 1.0 } else { 0.0 };
        locals.var_guard528 = assign31360_e49197;

        let (assign31370_e49210, assign31370_e49210_d_n0, assign31370_e49210_d_n2, assign31370_e49210_d_n3, assign31370_e49210_d_n4, assign31370_e49210_d_n7, assign31370_e49210_d_n8, assign31370_e49210_d_n9,) = {
    if (((locals.var_guard525 != 0.0) && (locals.var_guard526 == 0.0)) && (locals.var_guard528 != 0.0)) {
        let assign31370_e49207: f64 = (locals.var_arg - 80.0);
        let assign31370_e49208: f64 = (1.0 + assign31370_e49207);
        (assign31370_e49208, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn3, locals.var_arg_dn4, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9,)
    } else {
        (locals.var_le, locals.var_le_dn0, locals.var_le_dn2, locals.var_le_dn3, locals.var_le_dn4, locals.var_le_dn7, locals.var_le_dn8, locals.var_le_dn9,)
    }
};
        locals.var_le = assign31370_e49210;
        locals.var_le_dn0 = assign31370_e49210_d_n0;
        locals.var_le_dn2 = assign31370_e49210_d_n2;
        locals.var_le_dn3 = assign31370_e49210_d_n3;
        locals.var_le_dn4 = assign31370_e49210_d_n4;
        locals.var_le_dn7 = assign31370_e49210_d_n7;
        locals.var_le_dn8 = assign31370_e49210_d_n8;
        locals.var_le_dn9 = assign31370_e49210_d_n9;

        let (assign31380_e49219, assign31380_e49219_d_n0, assign31380_e49219_d_n2, assign31380_e49219_d_n3, assign31380_e49219_d_n4, assign31380_e49219_d_n7, assign31380_e49219_d_n8, assign31380_e49219_d_n9,) = {
    if (((locals.var_guard525 != 0.0) && (locals.var_guard526 == 0.0)) && (locals.var_guard528 != 0.0)) {
        (80.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn3, locals.var_arg_dn4, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9,)
    }
};
        locals.var_arg = assign31380_e49219;
        locals.var_arg_dn0 = assign31380_e49219_d_n0;
        locals.var_arg_dn2 = assign31380_e49219_d_n2;
        locals.var_arg_dn3 = assign31380_e49219_d_n3;
        locals.var_arg_dn4 = assign31380_e49219_d_n4;
        locals.var_arg_dn7 = assign31380_e49219_d_n7;
        locals.var_arg_dn8 = assign31380_e49219_d_n8;
        locals.var_arg_dn9 = assign31380_e49219_d_n9;

        let (assign31390_e49229, assign31390_e49229_d_n0, assign31390_e49229_d_n2, assign31390_e49229_d_n3, assign31390_e49229_d_n4, assign31390_e49229_d_n7, assign31390_e49229_d_n8, assign31390_e49229_d_n9,) = {
    if (((locals.var_guard525 != 0.0) && (locals.var_guard526 == 0.0)) && (locals.var_guard528 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_le, locals.var_le_dn0, locals.var_le_dn2, locals.var_le_dn3, locals.var_le_dn4, locals.var_le_dn7, locals.var_le_dn8, locals.var_le_dn9,)
    }
};
        locals.var_le = assign31390_e49229;
        locals.var_le_dn0 = assign31390_e49229_d_n0;
        locals.var_le_dn2 = assign31390_e49229_d_n2;
        locals.var_le_dn3 = assign31390_e49229_d_n3;
        locals.var_le_dn4 = assign31390_e49229_d_n4;
        locals.var_le_dn7 = assign31390_e49229_d_n7;
        locals.var_le_dn8 = assign31390_e49229_d_n8;
        locals.var_le_dn9 = assign31390_e49229_d_n9;

        let (assign31400_e49239, assign31400_e49239_d_n0, assign31400_e49239_d_n2, assign31400_e49239_d_n3, assign31400_e49239_d_n4, assign31400_e49239_d_n7, assign31400_e49239_d_n8, assign31400_e49239_d_n9,) = {
    if ((locals.var_guard525 != 0.0) && (locals.var_guard526 == 0.0)) {
        let assign31400_e49236: f64 = (locals.var_arg).exp();
        let assign31400_e49237: f64 = (locals.var_le * assign31400_e49236);
        (assign31400_e49237, ((locals.var_le_dn0 * assign31400_e49236) + (locals.var_le * (assign31400_e49236 * locals.var_arg_dn0))), ((locals.var_le_dn2 * assign31400_e49236) + (locals.var_le * (assign31400_e49236 * locals.var_arg_dn2))), ((locals.var_le_dn3 * assign31400_e49236) + (locals.var_le * (assign31400_e49236 * locals.var_arg_dn3))), ((locals.var_le_dn4 * assign31400_e49236) + (locals.var_le * (assign31400_e49236 * locals.var_arg_dn4))), ((locals.var_le_dn7 * assign31400_e49236) + (locals.var_le * (assign31400_e49236 * locals.var_arg_dn7))), ((locals.var_le_dn8 * assign31400_e49236) + (locals.var_le * (assign31400_e49236 * locals.var_arg_dn8))), ((locals.var_le_dn9 * assign31400_e49236) + (locals.var_le * (assign31400_e49236 * locals.var_arg_dn9))),)
    } else {
        (locals.var_le, locals.var_le_dn0, locals.var_le_dn2, locals.var_le_dn3, locals.var_le_dn4, locals.var_le_dn7, locals.var_le_dn8, locals.var_le_dn9,)
    }
};
        locals.var_le = assign31400_e49239;
        locals.var_le_dn0 = assign31400_e49239_d_n0;
        locals.var_le_dn2 = assign31400_e49239_d_n2;
        locals.var_le_dn3 = assign31400_e49239_d_n3;
        locals.var_le_dn4 = assign31400_e49239_d_n4;
        locals.var_le_dn7 = assign31400_e49239_d_n7;
        locals.var_le_dn8 = assign31400_e49239_d_n8;
        locals.var_le_dn9 = assign31400_e49239_d_n9;

        let (assign31410_e49250, assign31410_e49250_d_n0, assign31410_e49250_d_n1, assign31410_e49250_d_n2, assign31410_e49250_d_n3, assign31410_e49250_d_n4, assign31410_e49250_d_n5, assign31410_e49250_d_n6, assign31410_e49250_d_n7, assign31410_e49250_d_n8, assign31410_e49250_d_n9, assign31410_e49250_d_n12, assign31410_e49250_d_n14, assign31410_e49250_d_n15, assign31410_e49250_d_n16, assign31410_e49250_d_n17, assign31410_e49250_d_n18, assign31410_e49250_d_n19, assign31410_e49250_d_n20, assign31410_e49250_d_n21, assign31410_e49250_d_n22,) = {
    if ((locals.var_guard525 != 0.0) && (locals.var_guard526 == 0.0)) {
        let assign31410_e49247: f64 = (locals.var_le - 1.0);
        let assign31410_e49248: f64 = (locals.var_t3 * assign31410_e49247);
        (assign31410_e49248, ((locals.var_t3_dn0 * assign31410_e49247) + (locals.var_t3 * locals.var_le_dn0)), (locals.var_t3_dn1 * assign31410_e49247), ((locals.var_t3_dn2 * assign31410_e49247) + (locals.var_t3 * locals.var_le_dn2)), ((locals.var_t3_dn3 * assign31410_e49247) + (locals.var_t3 * locals.var_le_dn3)), ((locals.var_t3_dn4 * assign31410_e49247) + (locals.var_t3 * locals.var_le_dn4)), (locals.var_t3_dn5 * assign31410_e49247), (locals.var_t3_dn6 * assign31410_e49247), ((locals.var_t3_dn7 * assign31410_e49247) + (locals.var_t3 * locals.var_le_dn7)), ((locals.var_t3_dn8 * assign31410_e49247) + (locals.var_t3 * locals.var_le_dn8)), ((locals.var_t3_dn9 * assign31410_e49247) + (locals.var_t3 * locals.var_le_dn9)), (locals.var_t3_dn12 * assign31410_e49247), (locals.var_t3_dn14 * assign31410_e49247), (locals.var_t3_dn15 * assign31410_e49247), (locals.var_t3_dn16 * assign31410_e49247), (locals.var_t3_dn17 * assign31410_e49247), (locals.var_t3_dn18 * assign31410_e49247), (locals.var_t3_dn19 * assign31410_e49247), (locals.var_t3_dn20 * assign31410_e49247), (locals.var_t3_dn21 * assign31410_e49247), (locals.var_t3_dn22 * assign31410_e49247),)
    } else {
        (locals.var_idb, locals.var_idb_dn0, locals.var_idb_dn1, locals.var_idb_dn2, locals.var_idb_dn3, locals.var_idb_dn4, locals.var_idb_dn5, locals.var_idb_dn6, locals.var_idb_dn7, locals.var_idb_dn8, locals.var_idb_dn9, locals.var_idb_dn12, locals.var_idb_dn14, locals.var_idb_dn15, locals.var_idb_dn16, locals.var_idb_dn17, locals.var_idb_dn18, locals.var_idb_dn19, locals.var_idb_dn20, locals.var_idb_dn21, locals.var_idb_dn22,)
    }
};
        locals.var_idb = assign31410_e49250;
        locals.var_idb_dn0 = assign31410_e49250_d_n0;
        locals.var_idb_dn1 = assign31410_e49250_d_n1;
        locals.var_idb_dn2 = assign31410_e49250_d_n2;
        locals.var_idb_dn3 = assign31410_e49250_d_n3;
        locals.var_idb_dn4 = assign31410_e49250_d_n4;
        locals.var_idb_dn5 = assign31410_e49250_d_n5;
        locals.var_idb_dn6 = assign31410_e49250_d_n6;
        locals.var_idb_dn7 = assign31410_e49250_d_n7;
        locals.var_idb_dn8 = assign31410_e49250_d_n8;
        locals.var_idb_dn9 = assign31410_e49250_d_n9;
        locals.var_idb_dn12 = assign31410_e49250_d_n12;
        locals.var_idb_dn14 = assign31410_e49250_d_n14;
        locals.var_idb_dn15 = assign31410_e49250_d_n15;
        locals.var_idb_dn16 = assign31410_e49250_d_n16;
        locals.var_idb_dn17 = assign31410_e49250_d_n17;
        locals.var_idb_dn18 = assign31410_e49250_d_n18;
        locals.var_idb_dn19 = assign31410_e49250_d_n19;
        locals.var_idb_dn20 = assign31410_e49250_d_n20;
        locals.var_idb_dn21 = assign31410_e49250_d_n21;
        locals.var_idb_dn22 = assign31410_e49250_d_n22;

        let (assign31420_e49255, assign31420_e49255_d_n0, assign31420_e49255_d_n1, assign31420_e49255_d_n2, assign31420_e49255_d_n3, assign31420_e49255_d_n4, assign31420_e49255_d_n5, assign31420_e49255_d_n6, assign31420_e49255_d_n7, assign31420_e49255_d_n8, assign31420_e49255_d_n9, assign31420_e49255_d_n12, assign31420_e49255_d_n14, assign31420_e49255_d_n15, assign31420_e49255_d_n16, assign31420_e49255_d_n17, assign31420_e49255_d_n18, assign31420_e49255_d_n19, assign31420_e49255_d_n20, assign31420_e49255_d_n21, assign31420_e49255_d_n22,) = {
    if (locals.var_guard525 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_idb, locals.var_idb_dn0, locals.var_idb_dn1, locals.var_idb_dn2, locals.var_idb_dn3, locals.var_idb_dn4, locals.var_idb_dn5, locals.var_idb_dn6, locals.var_idb_dn7, locals.var_idb_dn8, locals.var_idb_dn9, locals.var_idb_dn12, locals.var_idb_dn14, locals.var_idb_dn15, locals.var_idb_dn16, locals.var_idb_dn17, locals.var_idb_dn18, locals.var_idb_dn19, locals.var_idb_dn20, locals.var_idb_dn21, locals.var_idb_dn22,)
    }
};
        locals.var_idb = assign31420_e49255;
        locals.var_idb_dn0 = assign31420_e49255_d_n0;
        locals.var_idb_dn1 = assign31420_e49255_d_n1;
        locals.var_idb_dn2 = assign31420_e49255_d_n2;
        locals.var_idb_dn3 = assign31420_e49255_d_n3;
        locals.var_idb_dn4 = assign31420_e49255_d_n4;
        locals.var_idb_dn5 = assign31420_e49255_d_n5;
        locals.var_idb_dn6 = assign31420_e49255_d_n6;
        locals.var_idb_dn7 = assign31420_e49255_d_n7;
        locals.var_idb_dn8 = assign31420_e49255_d_n8;
        locals.var_idb_dn9 = assign31420_e49255_d_n9;
        locals.var_idb_dn12 = assign31420_e49255_d_n12;
        locals.var_idb_dn14 = assign31420_e49255_d_n14;
        locals.var_idb_dn15 = assign31420_e49255_d_n15;
        locals.var_idb_dn16 = assign31420_e49255_d_n16;
        locals.var_idb_dn17 = assign31420_e49255_d_n17;
        locals.var_idb_dn18 = assign31420_e49255_d_n18;
        locals.var_idb_dn19 = assign31420_e49255_d_n19;
        locals.var_idb_dn20 = assign31420_e49255_d_n20;
        locals.var_idb_dn21 = assign31420_e49255_d_n21;
        locals.var_idb_dn22 = assign31420_e49255_d_n22;

        let assign31430_e49258: f64 = ((nv2 - nv3) - locals.var_vbisb_t);
        let assign31430_e49260: f64 = (assign31430_e49258).max(0.0);
        locals.var_vbsl = assign31430_e49260;
        locals.var_vbsl_dn2 = if assign31430_e49258 >= 0.0 { 1.0 } else { 0.0 };
        locals.var_vbsl_dn3 = if assign31430_e49258 >= 0.0 { -1.0 } else { 0.0 };
        locals.var_vbsl_dn4 = if assign31430_e49258 >= 0.0 { (-locals.var_vbisb_t_dn4) } else { 0.0 };

        let assign31440_e49263: f64 = (p.p4 * p.p5);
        let assign31440_e49265: f64 = (assign31440_e49263 * locals.var_isb_t);
        locals.var_t3 = assign31440_e49265;
        locals.var_t3_dn0 = 0.0;
        locals.var_t3_dn1 = 0.0;
        locals.var_t3_dn2 = 0.0;
        locals.var_t3_dn3 = 0.0;
        locals.var_t3_dn4 = (assign31440_e49263 * locals.var_isb_t_dn4);
        locals.var_t3_dn5 = 0.0;
        locals.var_t3_dn6 = 0.0;
        locals.var_t3_dn7 = 0.0;
        locals.var_t3_dn8 = 0.0;
        locals.var_t3_dn9 = 0.0;
        locals.var_t3_dn12 = 0.0;
        locals.var_t3_dn14 = 0.0;
        locals.var_t3_dn15 = 0.0;
        locals.var_t3_dn16 = 0.0;
        locals.var_t3_dn17 = 0.0;
        locals.var_t3_dn18 = 0.0;
        locals.var_t3_dn19 = 0.0;
        locals.var_t3_dn20 = 0.0;
        locals.var_t3_dn21 = 0.0;
        locals.var_t3_dn22 = 0.0;

        let assign31450_e49268: f64 = if locals.var_t3 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard529 = assign31450_e49268;

        let assign31460_e49271: f64 = if locals.var_vbsl > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard530 = assign31460_e49271;

        let (assign31470_e49283, assign31470_e49283_d_n0, assign31470_e49283_d_n2, assign31470_e49283_d_n3, assign31470_e49283_d_n4, assign31470_e49283_d_n7, assign31470_e49283_d_n8, assign31470_e49283_d_n9,) = {
    if ((locals.var_guard529 != 0.0) && (locals.var_guard530 != 0.0)) {
        let assign31470_e49277: f64 = (locals.var_vbsl).powf(1.0);
        let assign31470_e49280: f64 = (locals.var_nsb_t * locals.var_vth);
        let assign31470_e49281: f64 = (assign31470_e49277 / assign31470_e49280);
        (assign31470_e49281, 0.0, (if 0.0 == 0.0 && ((1.0) as f64).is_finite() && ((1.0) as f64).fract() == 0.0 { if 1.0 == 0.0 { 0.0 } else { ((locals.var_vbsl).powf(1.0 - 1.0) * locals.var_vbsl_dn2) } } else { (assign31470_e49277 * (locals.var_vbsl_dn2 / locals.var_vbsl)) } / assign31470_e49280), (if 0.0 == 0.0 && ((1.0) as f64).is_finite() && ((1.0) as f64).fract() == 0.0 { if 1.0 == 0.0 { 0.0 } else { ((locals.var_vbsl).powf(1.0 - 1.0) * locals.var_vbsl_dn3) } } else { (assign31470_e49277 * (locals.var_vbsl_dn3 / locals.var_vbsl)) } / assign31470_e49280), (((if 0.0 == 0.0 && ((1.0) as f64).is_finite() && ((1.0) as f64).fract() == 0.0 { if 1.0 == 0.0 { 0.0 } else { ((locals.var_vbsl).powf(1.0 - 1.0) * locals.var_vbsl_dn4) } } else { (assign31470_e49277 * (locals.var_vbsl_dn4 / locals.var_vbsl)) } * assign31470_e49280) - (assign31470_e49277 * ((locals.var_nsb_t_dn4 * locals.var_vth) + (locals.var_nsb_t * locals.var_vth_dn4)))) / (assign31470_e49280 * assign31470_e49280)), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn3, locals.var_arg_dn4, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9,)
    }
};
        locals.var_arg = assign31470_e49283;
        locals.var_arg_dn0 = assign31470_e49283_d_n0;
        locals.var_arg_dn2 = assign31470_e49283_d_n2;
        locals.var_arg_dn3 = assign31470_e49283_d_n3;
        locals.var_arg_dn4 = assign31470_e49283_d_n4;
        locals.var_arg_dn7 = assign31470_e49283_d_n7;
        locals.var_arg_dn8 = assign31470_e49283_d_n8;
        locals.var_arg_dn9 = assign31470_e49283_d_n9;

        let assign31480_e49286: f64 = if locals.var_arg > 80.0 { 1.0 } else { 0.0 };
        locals.var_guard531 = assign31480_e49286;

        let (assign31490_e49298, assign31490_e49298_d_n0, assign31490_e49298_d_n2, assign31490_e49298_d_n3, assign31490_e49298_d_n4, assign31490_e49298_d_n7, assign31490_e49298_d_n8, assign31490_e49298_d_n9,) = {
    if (((locals.var_guard529 != 0.0) && (locals.var_guard530 != 0.0)) && (locals.var_guard531 != 0.0)) {
        let assign31490_e49295: f64 = (locals.var_arg - 80.0);
        let assign31490_e49296: f64 = (1.0 + assign31490_e49295);
        (assign31490_e49296, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn3, locals.var_arg_dn4, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9,)
    } else {
        (locals.var_le, locals.var_le_dn0, locals.var_le_dn2, locals.var_le_dn3, locals.var_le_dn4, locals.var_le_dn7, locals.var_le_dn8, locals.var_le_dn9,)
    }
};
        locals.var_le = assign31490_e49298;
        locals.var_le_dn0 = assign31490_e49298_d_n0;
        locals.var_le_dn2 = assign31490_e49298_d_n2;
        locals.var_le_dn3 = assign31490_e49298_d_n3;
        locals.var_le_dn4 = assign31490_e49298_d_n4;
        locals.var_le_dn7 = assign31490_e49298_d_n7;
        locals.var_le_dn8 = assign31490_e49298_d_n8;
        locals.var_le_dn9 = assign31490_e49298_d_n9;

        let (assign31500_e49306, assign31500_e49306_d_n0, assign31500_e49306_d_n2, assign31500_e49306_d_n3, assign31500_e49306_d_n4, assign31500_e49306_d_n7, assign31500_e49306_d_n8, assign31500_e49306_d_n9,) = {
    if (((locals.var_guard529 != 0.0) && (locals.var_guard530 != 0.0)) && (locals.var_guard531 != 0.0)) {
        (80.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn3, locals.var_arg_dn4, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9,)
    }
};
        locals.var_arg = assign31500_e49306;
        locals.var_arg_dn0 = assign31500_e49306_d_n0;
        locals.var_arg_dn2 = assign31500_e49306_d_n2;
        locals.var_arg_dn3 = assign31500_e49306_d_n3;
        locals.var_arg_dn4 = assign31500_e49306_d_n4;
        locals.var_arg_dn7 = assign31500_e49306_d_n7;
        locals.var_arg_dn8 = assign31500_e49306_d_n8;
        locals.var_arg_dn9 = assign31500_e49306_d_n9;

        let (assign31510_e49315, assign31510_e49315_d_n0, assign31510_e49315_d_n2, assign31510_e49315_d_n3, assign31510_e49315_d_n4, assign31510_e49315_d_n7, assign31510_e49315_d_n8, assign31510_e49315_d_n9,) = {
    if (((locals.var_guard529 != 0.0) && (locals.var_guard530 != 0.0)) && (locals.var_guard531 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_le, locals.var_le_dn0, locals.var_le_dn2, locals.var_le_dn3, locals.var_le_dn4, locals.var_le_dn7, locals.var_le_dn8, locals.var_le_dn9,)
    }
};
        locals.var_le = assign31510_e49315;
        locals.var_le_dn0 = assign31510_e49315_d_n0;
        locals.var_le_dn2 = assign31510_e49315_d_n2;
        locals.var_le_dn3 = assign31510_e49315_d_n3;
        locals.var_le_dn4 = assign31510_e49315_d_n4;
        locals.var_le_dn7 = assign31510_e49315_d_n7;
        locals.var_le_dn8 = assign31510_e49315_d_n8;
        locals.var_le_dn9 = assign31510_e49315_d_n9;

        let (assign31520_e49324, assign31520_e49324_d_n0, assign31520_e49324_d_n2, assign31520_e49324_d_n3, assign31520_e49324_d_n4, assign31520_e49324_d_n7, assign31520_e49324_d_n8, assign31520_e49324_d_n9,) = {
    if ((locals.var_guard529 != 0.0) && (locals.var_guard530 != 0.0)) {
        let assign31520_e49321: f64 = (locals.var_arg).exp();
        let assign31520_e49322: f64 = (locals.var_le * assign31520_e49321);
        (assign31520_e49322, ((locals.var_le_dn0 * assign31520_e49321) + (locals.var_le * (assign31520_e49321 * locals.var_arg_dn0))), ((locals.var_le_dn2 * assign31520_e49321) + (locals.var_le * (assign31520_e49321 * locals.var_arg_dn2))), ((locals.var_le_dn3 * assign31520_e49321) + (locals.var_le * (assign31520_e49321 * locals.var_arg_dn3))), ((locals.var_le_dn4 * assign31520_e49321) + (locals.var_le * (assign31520_e49321 * locals.var_arg_dn4))), ((locals.var_le_dn7 * assign31520_e49321) + (locals.var_le * (assign31520_e49321 * locals.var_arg_dn7))), ((locals.var_le_dn8 * assign31520_e49321) + (locals.var_le * (assign31520_e49321 * locals.var_arg_dn8))), ((locals.var_le_dn9 * assign31520_e49321) + (locals.var_le * (assign31520_e49321 * locals.var_arg_dn9))),)
    } else {
        (locals.var_le, locals.var_le_dn0, locals.var_le_dn2, locals.var_le_dn3, locals.var_le_dn4, locals.var_le_dn7, locals.var_le_dn8, locals.var_le_dn9,)
    }
};
        locals.var_le = assign31520_e49324;
        locals.var_le_dn0 = assign31520_e49324_d_n0;
        locals.var_le_dn2 = assign31520_e49324_d_n2;
        locals.var_le_dn3 = assign31520_e49324_d_n3;
        locals.var_le_dn4 = assign31520_e49324_d_n4;
        locals.var_le_dn7 = assign31520_e49324_d_n7;
        locals.var_le_dn8 = assign31520_e49324_d_n8;
        locals.var_le_dn9 = assign31520_e49324_d_n9;

        let (assign31530_e49334, assign31530_e49334_d_n0, assign31530_e49334_d_n1, assign31530_e49334_d_n2, assign31530_e49334_d_n3, assign31530_e49334_d_n4, assign31530_e49334_d_n5, assign31530_e49334_d_n6, assign31530_e49334_d_n7, assign31530_e49334_d_n8, assign31530_e49334_d_n9, assign31530_e49334_d_n12, assign31530_e49334_d_n14, assign31530_e49334_d_n15, assign31530_e49334_d_n16, assign31530_e49334_d_n17, assign31530_e49334_d_n18, assign31530_e49334_d_n19, assign31530_e49334_d_n20, assign31530_e49334_d_n21, assign31530_e49334_d_n22,) = {
    if ((locals.var_guard529 != 0.0) && (locals.var_guard530 != 0.0)) {
        let assign31530_e49331: f64 = (locals.var_le - 1.0);
        let assign31530_e49332: f64 = (locals.var_t3 * assign31530_e49331);
        (assign31530_e49332, ((locals.var_t3_dn0 * assign31530_e49331) + (locals.var_t3 * locals.var_le_dn0)), (locals.var_t3_dn1 * assign31530_e49331), ((locals.var_t3_dn2 * assign31530_e49331) + (locals.var_t3 * locals.var_le_dn2)), ((locals.var_t3_dn3 * assign31530_e49331) + (locals.var_t3 * locals.var_le_dn3)), ((locals.var_t3_dn4 * assign31530_e49331) + (locals.var_t3 * locals.var_le_dn4)), (locals.var_t3_dn5 * assign31530_e49331), (locals.var_t3_dn6 * assign31530_e49331), ((locals.var_t3_dn7 * assign31530_e49331) + (locals.var_t3 * locals.var_le_dn7)), ((locals.var_t3_dn8 * assign31530_e49331) + (locals.var_t3 * locals.var_le_dn8)), ((locals.var_t3_dn9 * assign31530_e49331) + (locals.var_t3 * locals.var_le_dn9)), (locals.var_t3_dn12 * assign31530_e49331), (locals.var_t3_dn14 * assign31530_e49331), (locals.var_t3_dn15 * assign31530_e49331), (locals.var_t3_dn16 * assign31530_e49331), (locals.var_t3_dn17 * assign31530_e49331), (locals.var_t3_dn18 * assign31530_e49331), (locals.var_t3_dn19 * assign31530_e49331), (locals.var_t3_dn20 * assign31530_e49331), (locals.var_t3_dn21 * assign31530_e49331), (locals.var_t3_dn22 * assign31530_e49331),)
    } else {
        (locals.var_isb, locals.var_isb_dn0, locals.var_isb_dn1, locals.var_isb_dn2, locals.var_isb_dn3, locals.var_isb_dn4, locals.var_isb_dn5, locals.var_isb_dn6, locals.var_isb_dn7, locals.var_isb_dn8, locals.var_isb_dn9, locals.var_isb_dn12, locals.var_isb_dn14, locals.var_isb_dn15, locals.var_isb_dn16, locals.var_isb_dn17, locals.var_isb_dn18, locals.var_isb_dn19, locals.var_isb_dn20, locals.var_isb_dn21, locals.var_isb_dn22,)
    }
};
        locals.var_isb = assign31530_e49334;
        locals.var_isb_dn0 = assign31530_e49334_d_n0;
        locals.var_isb_dn1 = assign31530_e49334_d_n1;
        locals.var_isb_dn2 = assign31530_e49334_d_n2;
        locals.var_isb_dn3 = assign31530_e49334_d_n3;
        locals.var_isb_dn4 = assign31530_e49334_d_n4;
        locals.var_isb_dn5 = assign31530_e49334_d_n5;
        locals.var_isb_dn6 = assign31530_e49334_d_n6;
        locals.var_isb_dn7 = assign31530_e49334_d_n7;
        locals.var_isb_dn8 = assign31530_e49334_d_n8;
        locals.var_isb_dn9 = assign31530_e49334_d_n9;
        locals.var_isb_dn12 = assign31530_e49334_d_n12;
        locals.var_isb_dn14 = assign31530_e49334_d_n14;
        locals.var_isb_dn15 = assign31530_e49334_d_n15;
        locals.var_isb_dn16 = assign31530_e49334_d_n16;
        locals.var_isb_dn17 = assign31530_e49334_d_n17;
        locals.var_isb_dn18 = assign31530_e49334_d_n18;
        locals.var_isb_dn19 = assign31530_e49334_d_n19;
        locals.var_isb_dn20 = assign31530_e49334_d_n20;
        locals.var_isb_dn21 = assign31530_e49334_d_n21;
        locals.var_isb_dn22 = assign31530_e49334_d_n22;

        let (assign31540_e49345, assign31540_e49345_d_n0, assign31540_e49345_d_n2, assign31540_e49345_d_n3, assign31540_e49345_d_n4, assign31540_e49345_d_n7, assign31540_e49345_d_n8, assign31540_e49345_d_n9,) = {
    if ((locals.var_guard529 != 0.0) && (locals.var_guard530 == 0.0)) {
        let assign31540_e49342: f64 = (locals.var_nsb_t * locals.var_vth);
        let assign31540_e49343: f64 = (locals.var_vbsl / assign31540_e49342);
        (assign31540_e49343, 0.0, (locals.var_vbsl_dn2 / assign31540_e49342), (locals.var_vbsl_dn3 / assign31540_e49342), (((locals.var_vbsl_dn4 * assign31540_e49342) - (locals.var_vbsl * ((locals.var_nsb_t_dn4 * locals.var_vth) + (locals.var_nsb_t * locals.var_vth_dn4)))) / (assign31540_e49342 * assign31540_e49342)), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn3, locals.var_arg_dn4, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9,)
    }
};
        locals.var_arg = assign31540_e49345;
        locals.var_arg_dn0 = assign31540_e49345_d_n0;
        locals.var_arg_dn2 = assign31540_e49345_d_n2;
        locals.var_arg_dn3 = assign31540_e49345_d_n3;
        locals.var_arg_dn4 = assign31540_e49345_d_n4;
        locals.var_arg_dn7 = assign31540_e49345_d_n7;
        locals.var_arg_dn8 = assign31540_e49345_d_n8;
        locals.var_arg_dn9 = assign31540_e49345_d_n9;

        let assign31550_e49348: f64 = if locals.var_arg > 80.0 { 1.0 } else { 0.0 };
        locals.var_guard532 = assign31550_e49348;

    }

    pub(super) fn stamp_transient_block_183(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let (assign31560_e49361, assign31560_e49361_d_n0, assign31560_e49361_d_n2, assign31560_e49361_d_n3, assign31560_e49361_d_n4, assign31560_e49361_d_n7, assign31560_e49361_d_n8, assign31560_e49361_d_n9,) = {
    if (((locals.var_guard529 != 0.0) && (locals.var_guard530 == 0.0)) && (locals.var_guard532 != 0.0)) {
        let assign31560_e49358: f64 = (locals.var_arg - 80.0);
        let assign31560_e49359: f64 = (1.0 + assign31560_e49358);
        (assign31560_e49359, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn3, locals.var_arg_dn4, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9,)
    } else {
        (locals.var_le, locals.var_le_dn0, locals.var_le_dn2, locals.var_le_dn3, locals.var_le_dn4, locals.var_le_dn7, locals.var_le_dn8, locals.var_le_dn9,)
    }
};
        locals.var_le = assign31560_e49361;
        locals.var_le_dn0 = assign31560_e49361_d_n0;
        locals.var_le_dn2 = assign31560_e49361_d_n2;
        locals.var_le_dn3 = assign31560_e49361_d_n3;
        locals.var_le_dn4 = assign31560_e49361_d_n4;
        locals.var_le_dn7 = assign31560_e49361_d_n7;
        locals.var_le_dn8 = assign31560_e49361_d_n8;
        locals.var_le_dn9 = assign31560_e49361_d_n9;

        let (assign31570_e49370, assign31570_e49370_d_n0, assign31570_e49370_d_n2, assign31570_e49370_d_n3, assign31570_e49370_d_n4, assign31570_e49370_d_n7, assign31570_e49370_d_n8, assign31570_e49370_d_n9,) = {
    if (((locals.var_guard529 != 0.0) && (locals.var_guard530 == 0.0)) && (locals.var_guard532 != 0.0)) {
        (80.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn3, locals.var_arg_dn4, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9,)
    }
};
        locals.var_arg = assign31570_e49370;
        locals.var_arg_dn0 = assign31570_e49370_d_n0;
        locals.var_arg_dn2 = assign31570_e49370_d_n2;
        locals.var_arg_dn3 = assign31570_e49370_d_n3;
        locals.var_arg_dn4 = assign31570_e49370_d_n4;
        locals.var_arg_dn7 = assign31570_e49370_d_n7;
        locals.var_arg_dn8 = assign31570_e49370_d_n8;
        locals.var_arg_dn9 = assign31570_e49370_d_n9;

        let (assign31580_e49380, assign31580_e49380_d_n0, assign31580_e49380_d_n2, assign31580_e49380_d_n3, assign31580_e49380_d_n4, assign31580_e49380_d_n7, assign31580_e49380_d_n8, assign31580_e49380_d_n9,) = {
    if (((locals.var_guard529 != 0.0) && (locals.var_guard530 == 0.0)) && (locals.var_guard532 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_le, locals.var_le_dn0, locals.var_le_dn2, locals.var_le_dn3, locals.var_le_dn4, locals.var_le_dn7, locals.var_le_dn8, locals.var_le_dn9,)
    }
};
        locals.var_le = assign31580_e49380;
        locals.var_le_dn0 = assign31580_e49380_d_n0;
        locals.var_le_dn2 = assign31580_e49380_d_n2;
        locals.var_le_dn3 = assign31580_e49380_d_n3;
        locals.var_le_dn4 = assign31580_e49380_d_n4;
        locals.var_le_dn7 = assign31580_e49380_d_n7;
        locals.var_le_dn8 = assign31580_e49380_d_n8;
        locals.var_le_dn9 = assign31580_e49380_d_n9;

        let (assign31590_e49390, assign31590_e49390_d_n0, assign31590_e49390_d_n2, assign31590_e49390_d_n3, assign31590_e49390_d_n4, assign31590_e49390_d_n7, assign31590_e49390_d_n8, assign31590_e49390_d_n9,) = {
    if ((locals.var_guard529 != 0.0) && (locals.var_guard530 == 0.0)) {
        let assign31590_e49387: f64 = (locals.var_arg).exp();
        let assign31590_e49388: f64 = (locals.var_le * assign31590_e49387);
        (assign31590_e49388, ((locals.var_le_dn0 * assign31590_e49387) + (locals.var_le * (assign31590_e49387 * locals.var_arg_dn0))), ((locals.var_le_dn2 * assign31590_e49387) + (locals.var_le * (assign31590_e49387 * locals.var_arg_dn2))), ((locals.var_le_dn3 * assign31590_e49387) + (locals.var_le * (assign31590_e49387 * locals.var_arg_dn3))), ((locals.var_le_dn4 * assign31590_e49387) + (locals.var_le * (assign31590_e49387 * locals.var_arg_dn4))), ((locals.var_le_dn7 * assign31590_e49387) + (locals.var_le * (assign31590_e49387 * locals.var_arg_dn7))), ((locals.var_le_dn8 * assign31590_e49387) + (locals.var_le * (assign31590_e49387 * locals.var_arg_dn8))), ((locals.var_le_dn9 * assign31590_e49387) + (locals.var_le * (assign31590_e49387 * locals.var_arg_dn9))),)
    } else {
        (locals.var_le, locals.var_le_dn0, locals.var_le_dn2, locals.var_le_dn3, locals.var_le_dn4, locals.var_le_dn7, locals.var_le_dn8, locals.var_le_dn9,)
    }
};
        locals.var_le = assign31590_e49390;
        locals.var_le_dn0 = assign31590_e49390_d_n0;
        locals.var_le_dn2 = assign31590_e49390_d_n2;
        locals.var_le_dn3 = assign31590_e49390_d_n3;
        locals.var_le_dn4 = assign31590_e49390_d_n4;
        locals.var_le_dn7 = assign31590_e49390_d_n7;
        locals.var_le_dn8 = assign31590_e49390_d_n8;
        locals.var_le_dn9 = assign31590_e49390_d_n9;

        let (assign31600_e49401, assign31600_e49401_d_n0, assign31600_e49401_d_n1, assign31600_e49401_d_n2, assign31600_e49401_d_n3, assign31600_e49401_d_n4, assign31600_e49401_d_n5, assign31600_e49401_d_n6, assign31600_e49401_d_n7, assign31600_e49401_d_n8, assign31600_e49401_d_n9, assign31600_e49401_d_n12, assign31600_e49401_d_n14, assign31600_e49401_d_n15, assign31600_e49401_d_n16, assign31600_e49401_d_n17, assign31600_e49401_d_n18, assign31600_e49401_d_n19, assign31600_e49401_d_n20, assign31600_e49401_d_n21, assign31600_e49401_d_n22,) = {
    if ((locals.var_guard529 != 0.0) && (locals.var_guard530 == 0.0)) {
        let assign31600_e49398: f64 = (locals.var_le - 1.0);
        let assign31600_e49399: f64 = (locals.var_t3 * assign31600_e49398);
        (assign31600_e49399, ((locals.var_t3_dn0 * assign31600_e49398) + (locals.var_t3 * locals.var_le_dn0)), (locals.var_t3_dn1 * assign31600_e49398), ((locals.var_t3_dn2 * assign31600_e49398) + (locals.var_t3 * locals.var_le_dn2)), ((locals.var_t3_dn3 * assign31600_e49398) + (locals.var_t3 * locals.var_le_dn3)), ((locals.var_t3_dn4 * assign31600_e49398) + (locals.var_t3 * locals.var_le_dn4)), (locals.var_t3_dn5 * assign31600_e49398), (locals.var_t3_dn6 * assign31600_e49398), ((locals.var_t3_dn7 * assign31600_e49398) + (locals.var_t3 * locals.var_le_dn7)), ((locals.var_t3_dn8 * assign31600_e49398) + (locals.var_t3 * locals.var_le_dn8)), ((locals.var_t3_dn9 * assign31600_e49398) + (locals.var_t3 * locals.var_le_dn9)), (locals.var_t3_dn12 * assign31600_e49398), (locals.var_t3_dn14 * assign31600_e49398), (locals.var_t3_dn15 * assign31600_e49398), (locals.var_t3_dn16 * assign31600_e49398), (locals.var_t3_dn17 * assign31600_e49398), (locals.var_t3_dn18 * assign31600_e49398), (locals.var_t3_dn19 * assign31600_e49398), (locals.var_t3_dn20 * assign31600_e49398), (locals.var_t3_dn21 * assign31600_e49398), (locals.var_t3_dn22 * assign31600_e49398),)
    } else {
        (locals.var_isb, locals.var_isb_dn0, locals.var_isb_dn1, locals.var_isb_dn2, locals.var_isb_dn3, locals.var_isb_dn4, locals.var_isb_dn5, locals.var_isb_dn6, locals.var_isb_dn7, locals.var_isb_dn8, locals.var_isb_dn9, locals.var_isb_dn12, locals.var_isb_dn14, locals.var_isb_dn15, locals.var_isb_dn16, locals.var_isb_dn17, locals.var_isb_dn18, locals.var_isb_dn19, locals.var_isb_dn20, locals.var_isb_dn21, locals.var_isb_dn22,)
    }
};
        locals.var_isb = assign31600_e49401;
        locals.var_isb_dn0 = assign31600_e49401_d_n0;
        locals.var_isb_dn1 = assign31600_e49401_d_n1;
        locals.var_isb_dn2 = assign31600_e49401_d_n2;
        locals.var_isb_dn3 = assign31600_e49401_d_n3;
        locals.var_isb_dn4 = assign31600_e49401_d_n4;
        locals.var_isb_dn5 = assign31600_e49401_d_n5;
        locals.var_isb_dn6 = assign31600_e49401_d_n6;
        locals.var_isb_dn7 = assign31600_e49401_d_n7;
        locals.var_isb_dn8 = assign31600_e49401_d_n8;
        locals.var_isb_dn9 = assign31600_e49401_d_n9;
        locals.var_isb_dn12 = assign31600_e49401_d_n12;
        locals.var_isb_dn14 = assign31600_e49401_d_n14;
        locals.var_isb_dn15 = assign31600_e49401_d_n15;
        locals.var_isb_dn16 = assign31600_e49401_d_n16;
        locals.var_isb_dn17 = assign31600_e49401_d_n17;
        locals.var_isb_dn18 = assign31600_e49401_d_n18;
        locals.var_isb_dn19 = assign31600_e49401_d_n19;
        locals.var_isb_dn20 = assign31600_e49401_d_n20;
        locals.var_isb_dn21 = assign31600_e49401_d_n21;
        locals.var_isb_dn22 = assign31600_e49401_d_n22;

        let (assign31610_e49406, assign31610_e49406_d_n0, assign31610_e49406_d_n1, assign31610_e49406_d_n2, assign31610_e49406_d_n3, assign31610_e49406_d_n4, assign31610_e49406_d_n5, assign31610_e49406_d_n6, assign31610_e49406_d_n7, assign31610_e49406_d_n8, assign31610_e49406_d_n9, assign31610_e49406_d_n12, assign31610_e49406_d_n14, assign31610_e49406_d_n15, assign31610_e49406_d_n16, assign31610_e49406_d_n17, assign31610_e49406_d_n18, assign31610_e49406_d_n19, assign31610_e49406_d_n20, assign31610_e49406_d_n21, assign31610_e49406_d_n22,) = {
    if (locals.var_guard529 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isb, locals.var_isb_dn0, locals.var_isb_dn1, locals.var_isb_dn2, locals.var_isb_dn3, locals.var_isb_dn4, locals.var_isb_dn5, locals.var_isb_dn6, locals.var_isb_dn7, locals.var_isb_dn8, locals.var_isb_dn9, locals.var_isb_dn12, locals.var_isb_dn14, locals.var_isb_dn15, locals.var_isb_dn16, locals.var_isb_dn17, locals.var_isb_dn18, locals.var_isb_dn19, locals.var_isb_dn20, locals.var_isb_dn21, locals.var_isb_dn22,)
    }
};
        locals.var_isb = assign31610_e49406;
        locals.var_isb_dn0 = assign31610_e49406_d_n0;
        locals.var_isb_dn1 = assign31610_e49406_d_n1;
        locals.var_isb_dn2 = assign31610_e49406_d_n2;
        locals.var_isb_dn3 = assign31610_e49406_d_n3;
        locals.var_isb_dn4 = assign31610_e49406_d_n4;
        locals.var_isb_dn5 = assign31610_e49406_d_n5;
        locals.var_isb_dn6 = assign31610_e49406_d_n6;
        locals.var_isb_dn7 = assign31610_e49406_d_n7;
        locals.var_isb_dn8 = assign31610_e49406_d_n8;
        locals.var_isb_dn9 = assign31610_e49406_d_n9;
        locals.var_isb_dn12 = assign31610_e49406_d_n12;
        locals.var_isb_dn14 = assign31610_e49406_d_n14;
        locals.var_isb_dn15 = assign31610_e49406_d_n15;
        locals.var_isb_dn16 = assign31610_e49406_d_n16;
        locals.var_isb_dn17 = assign31610_e49406_d_n17;
        locals.var_isb_dn18 = assign31610_e49406_d_n18;
        locals.var_isb_dn19 = assign31610_e49406_d_n19;
        locals.var_isb_dn20 = assign31610_e49406_d_n20;
        locals.var_isb_dn21 = assign31610_e49406_d_n21;
        locals.var_isb_dn22 = assign31610_e49406_d_n22;

        let assign31720_e49545: f64 = if p.p255 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard535 = assign31720_e49545;

        let assign31730_e49548: f64 = if p.p149 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard536 = assign31730_e49548;

        let assign31740_e49551: f64 = if p.p150 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard537 = assign31740_e49551;

        let assign31750_e49554: f64 = if p.p150 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard538 = assign31750_e49554;

        let assign31760_e49557: f64 = if p.p150 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard539 = assign31760_e49557;

        let assign31770_e49560: f64 = if p.p150 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard540 = assign31770_e49560;

        let assign31780_e49563: f64 = if p.p149 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard541 = assign31780_e49563;

        let assign31790_e49566: f64 = if p.p151 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard542 = assign31790_e49566;

        let assign31800_e49569: f64 = if p.p151 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard543 = assign31800_e49569;

        let assign31810_e49572: f64 = if p.p151 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard544 = assign31810_e49572;

        let assign31820_e49575: f64 = if p.p151 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard545 = assign31820_e49575;

        let assign31830_e49578: f64 = if p.p149 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard546 = assign31830_e49578;

        let assign31840_e49581: f64 = if p.p152 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard547 = assign31840_e49581;

        let assign31850_e49584: f64 = if p.p152 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard548 = assign31850_e49584;

        let assign31860_e49587: f64 = if p.p152 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard549 = assign31860_e49587;

        let assign31870_e49590: f64 = if p.p152 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard550 = assign31870_e49590;

        let assign31880_e49593: f64 = if p.p149 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard551 = assign31880_e49593;

        let assign31890_e49596: f64 = if p.p153 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard552 = assign31890_e49596;

        let assign31900_e49599: f64 = if p.p153 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard553 = assign31900_e49599;

        let assign31910_e49602: f64 = if p.p153 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard554 = assign31910_e49602;

        let assign31920_e49605: f64 = if p.p153 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard555 = assign31920_e49605;

        let assign31930_e49608: f64 = if p.p149 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard556 = assign31930_e49608;

        let assign31940_e49611: f64 = if p.p154 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard557 = assign31940_e49611;

        let assign31950_e49614: f64 = if p.p154 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard558 = assign31950_e49614;

        let assign31960_e49617: f64 = if p.p154 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard559 = assign31960_e49617;

        let assign31970_e49620: f64 = if p.p154 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard560 = assign31970_e49620;

        let assign31980_e49623: f64 = if p.p149 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard561 = assign31980_e49623;

        let assign31990_e49626: f64 = if p.p155 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard562 = assign31990_e49626;

        let assign32000_e49629: f64 = if p.p155 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard563 = assign32000_e49629;

        let assign32010_e49632: f64 = if p.p155 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard564 = assign32010_e49632;

        let assign32020_e49635: f64 = if p.p155 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard565 = assign32020_e49635;

        let assign32030_e49638: f64 = if p.p149 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard566 = assign32030_e49638;

        let assign32040_e49641: f64 = if p.p156 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard567 = assign32040_e49641;

        let assign32050_e49644: f64 = if p.p156 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard568 = assign32050_e49644;

        let assign32060_e49647: f64 = if p.p156 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard569 = assign32060_e49647;

        let assign32070_e49650: f64 = if p.p156 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard570 = assign32070_e49650;

        let assign32080_e49653: f64 = if p.p149 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard571 = assign32080_e49653;

        let assign32090_e49656: f64 = if p.p157 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard572 = assign32090_e49656;

        let assign32100_e49659: f64 = if p.p157 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard573 = assign32100_e49659;

        let assign32110_e49662: f64 = if p.p157 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard574 = assign32110_e49662;

        let assign32120_e49665: f64 = if p.p157 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard575 = assign32120_e49665;

        let assign32130_e49670: f64 = (locals.var_tdev / locals.var_tnom);
        let assign32130_e49672: f64 = (assign32130_e49670 - 1.0);
        let assign32130_e49674: f64 = (assign32130_e49672 * p.p227);
        let assign32130_e49675: f64 = (p.p220 + assign32130_e49674);
        let assign32130_e49677: f64 = (assign32130_e49675 * (nv0 - nv2));
        let assign32130_e49678: f64 = (p.p222 - assign32130_e49677);
        locals.var_qfr = assign32130_e49678;
        locals.var_qfr_dn0 = (-assign32130_e49675);
        locals.var_qfr_dn2 = (-(-assign32130_e49675));
        locals.var_qfr_dn4 = (-(((locals.var_tdev_dn4 / locals.var_tnom) * p.p227) * (nv0 - nv2)));

        let assign32140_e49681: f64 = (p.p4 * p.p5);
        let assign32140_e49684: f64 = (1e-25 + locals.var_qfr);
        let assign32140_e49688: f64 = (1e-25 + locals.var_qfr);
        let assign32140_e49691: f64 = (locals.var_qfr - 1e-25);
        let assign32140_e49694: f64 = (locals.var_qfr - 1e-25);
        let assign32140_e49695: f64 = (assign32140_e49691 * assign32140_e49694);
        let assign32140_e49697: f64 = (assign32140_e49695 + p.p221);
        let assign32140_e49698: f64 = (assign32140_e49697).sqrt();
        let assign32140_e49699: f64 = (assign32140_e49688 - assign32140_e49698);
        let assign32140_e49700: f64 = (0.5 * assign32140_e49699);
        let assign32140_e49701: f64 = (assign32140_e49684 - assign32140_e49700);
        let assign32140_e49702: f64 = (assign32140_e49681 * assign32140_e49701);
        locals.var_qfr = assign32140_e49702;
        locals.var_qfr_dn0 = (assign32140_e49681 * (locals.var_qfr_dn0 - (0.5 * (locals.var_qfr_dn0 - (((locals.var_qfr_dn0 * assign32140_e49694) + (assign32140_e49691 * locals.var_qfr_dn0)) / (2.0 * assign32140_e49698))))));
        locals.var_qfr_dn2 = (assign32140_e49681 * (locals.var_qfr_dn2 - (0.5 * (locals.var_qfr_dn2 - (((locals.var_qfr_dn2 * assign32140_e49694) + (assign32140_e49691 * locals.var_qfr_dn2)) / (2.0 * assign32140_e49698))))));
        locals.var_qfr_dn4 = (assign32140_e49681 * (locals.var_qfr_dn4 - (0.5 * (locals.var_qfr_dn4 - (((locals.var_qfr_dn4 * assign32140_e49694) + (assign32140_e49691 * locals.var_qfr_dn4)) / (2.0 * assign32140_e49698))))));

        let assign32150_e49707: f64 = (locals.var_tdev / locals.var_tnom);
        let assign32150_e49709: f64 = (assign32150_e49707 - 1.0);
        let assign32150_e49711: f64 = (assign32150_e49709 * p.p226);
        let assign32150_e49712: f64 = (p.p218 - assign32150_e49711);
        let assign32150_e49714: f64 = (assign32150_e49712 + 1e-18);
        let assign32150_e49718: f64 = (locals.var_tdev / locals.var_tnom);
        let assign32150_e49720: f64 = (assign32150_e49718 - 1.0);
        let assign32150_e49722: f64 = (assign32150_e49720 * p.p226);
        let assign32150_e49723: f64 = (p.p218 - assign32150_e49722);
        let assign32150_e49725: f64 = (assign32150_e49723 - 1e-18);
        let assign32150_e49729: f64 = (locals.var_tdev / locals.var_tnom);
        let assign32150_e49731: f64 = (assign32150_e49729 - 1.0);
        let assign32150_e49733: f64 = (assign32150_e49731 * p.p226);
        let assign32150_e49734: f64 = (p.p218 - assign32150_e49733);
        let assign32150_e49736: f64 = (assign32150_e49734 - 1e-18);
        let assign32150_e49737: f64 = (assign32150_e49725 * assign32150_e49736);
        let assign32150_e49740: f64 = (0.25 * 1e-19);
        let assign32150_e49742: f64 = (assign32150_e49740 * 1e-19);
        let assign32150_e49743: f64 = (assign32150_e49737 + assign32150_e49742);
        let assign32150_e49744: f64 = (assign32150_e49743).sqrt();
        let assign32150_e49745: f64 = (assign32150_e49714 + assign32150_e49744);
        let assign32150_e49746: f64 = (0.5 * assign32150_e49745);
        locals.var_t0 = assign32150_e49746;
        locals.var_t0_dn0 = 0.0;
        locals.var_t0_dn1 = 0.0;
        locals.var_t0_dn2 = 0.0;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = (0.5 * ((-((locals.var_tdev_dn4 / locals.var_tnom) * p.p226)) + ((((-((locals.var_tdev_dn4 / locals.var_tnom) * p.p226)) * assign32150_e49736) + (assign32150_e49725 * (-((locals.var_tdev_dn4 / locals.var_tnom) * p.p226)))) / (2.0 * assign32150_e49744))));
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn12 = 0.0;
        locals.var_t0_dn14 = 0.0;
        locals.var_t0_dn15 = 0.0;
        locals.var_t0_dn16 = 0.0;
        locals.var_t0_dn17 = 0.0;
        locals.var_t0_dn18 = 0.0;
        locals.var_t0_dn19 = 0.0;
        locals.var_t0_dn20 = 0.0;
        locals.var_t0_dn21 = 0.0;
        locals.var_t0_dn22 = 0.0;

        let assign32160_e49749: f64 = (p.p4 * p.p5);
        let assign32160_e49751: f64 = (assign32160_e49749 * locals.var_t0);
        let assign32160_e49753: f64 = (assign32160_e49751 * (nv9 - nv2));
        locals.var_qfr2 = assign32160_e49753;
        locals.var_qfr2_dn0 = ((assign32160_e49749 * locals.var_t0_dn0) * (nv9 - nv2));
        locals.var_qfr2_dn1 = ((assign32160_e49749 * locals.var_t0_dn1) * (nv9 - nv2));
        locals.var_qfr2_dn2 = (((assign32160_e49749 * locals.var_t0_dn2) * (nv9 - nv2)) + (-assign32160_e49751));
        locals.var_qfr2_dn3 = ((assign32160_e49749 * locals.var_t0_dn3) * (nv9 - nv2));
        locals.var_qfr2_dn4 = ((assign32160_e49749 * locals.var_t0_dn4) * (nv9 - nv2));
        locals.var_qfr2_dn5 = ((assign32160_e49749 * locals.var_t0_dn5) * (nv9 - nv2));
        locals.var_qfr2_dn6 = ((assign32160_e49749 * locals.var_t0_dn6) * (nv9 - nv2));
        locals.var_qfr2_dn7 = ((assign32160_e49749 * locals.var_t0_dn7) * (nv9 - nv2));
        locals.var_qfr2_dn8 = ((assign32160_e49749 * locals.var_t0_dn8) * (nv9 - nv2));
        locals.var_qfr2_dn9 = (((assign32160_e49749 * locals.var_t0_dn9) * (nv9 - nv2)) + assign32160_e49751);
        locals.var_qfr2_dn12 = ((assign32160_e49749 * locals.var_t0_dn12) * (nv9 - nv2));
        locals.var_qfr2_dn14 = ((assign32160_e49749 * locals.var_t0_dn14) * (nv9 - nv2));
        locals.var_qfr2_dn15 = ((assign32160_e49749 * locals.var_t0_dn15) * (nv9 - nv2));
        locals.var_qfr2_dn16 = ((assign32160_e49749 * locals.var_t0_dn16) * (nv9 - nv2));
        locals.var_qfr2_dn17 = ((assign32160_e49749 * locals.var_t0_dn17) * (nv9 - nv2));
        locals.var_qfr2_dn18 = ((assign32160_e49749 * locals.var_t0_dn18) * (nv9 - nv2));
        locals.var_qfr2_dn19 = ((assign32160_e49749 * locals.var_t0_dn19) * (nv9 - nv2));
        locals.var_qfr2_dn20 = ((assign32160_e49749 * locals.var_t0_dn20) * (nv9 - nv2));
        locals.var_qfr2_dn21 = ((assign32160_e49749 * locals.var_t0_dn21) * (nv9 - nv2));
        locals.var_qfr2_dn22 = ((assign32160_e49749 * locals.var_t0_dn22) * (nv9 - nv2));

        let assign32170_e49756: f64 = (p.p4 * p.p5);
        let assign32170_e49758: f64 = (assign32170_e49756 * p.p219);
        let assign32170_e49760: f64 = (assign32170_e49758 * (nv2 - nv0));
        locals.var_qfr3 = assign32170_e49760;
        locals.var_qfr3_dn0 = (-assign32170_e49758);
        locals.var_qfr3_dn2 = assign32170_e49758;

        let assign32180_e49764: f64 = (locals.var_tdev / locals.var_tnom);
        let assign32180_e49766: f64 = (assign32180_e49764 - 1.0);
        let assign32180_e49768: f64 = (assign32180_e49766 * p.p225);
        let assign32180_e49769: f64 = (p.p224 - assign32180_e49768);
        let assign32180_e49772: f64 = (p.p229).ln();
        let assign32180_e49773: f64 = (-assign32180_e49772);
        let assign32180_e49775: f64 = (assign32180_e49773 / p.p228);
        let assign32180_e49776: f64 = { let limited_exp_arg = assign32180_e49775; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign32180_e49777: f64 = (1.0 - assign32180_e49776);
        let assign32180_e49778: f64 = (assign32180_e49769 * assign32180_e49777);
        locals.var_t0 = assign32180_e49778;
        locals.var_t0_dn0 = 0.0;
        locals.var_t0_dn1 = 0.0;
        locals.var_t0_dn2 = 0.0;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = ((-((locals.var_tdev_dn4 / locals.var_tnom) * p.p225)) * assign32180_e49777);
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn12 = 0.0;
        locals.var_t0_dn14 = 0.0;
        locals.var_t0_dn15 = 0.0;
        locals.var_t0_dn16 = 0.0;
        locals.var_t0_dn17 = 0.0;
        locals.var_t0_dn18 = 0.0;
        locals.var_t0_dn19 = 0.0;
        locals.var_t0_dn20 = 0.0;
        locals.var_t0_dn21 = 0.0;
        locals.var_t0_dn22 = 0.0;

        let assign32190_e49781: f64 = (locals.var_t0 - (nv2 - nv0));
        let assign32190_e49783: f64 = (assign32190_e49781 / locals.var_vth);
        locals.var_t1 = assign32190_e49783;
        locals.var_t1_dn0 = ((locals.var_t0_dn0 - -1.0) / locals.var_vth);
        locals.var_t1_dn1 = (locals.var_t0_dn1 / locals.var_vth);
        locals.var_t1_dn2 = ((locals.var_t0_dn2 - 1.0) / locals.var_vth);
        locals.var_t1_dn3 = (locals.var_t0_dn3 / locals.var_vth);
        locals.var_t1_dn4 = (((locals.var_t0_dn4 * locals.var_vth) - (assign32190_e49781 * locals.var_vth_dn4)) / (locals.var_vth * locals.var_vth));
        locals.var_t1_dn5 = (locals.var_t0_dn5 / locals.var_vth);
        locals.var_t1_dn6 = (locals.var_t0_dn6 / locals.var_vth);
        locals.var_t1_dn7 = (locals.var_t0_dn7 / locals.var_vth);
        locals.var_t1_dn8 = (locals.var_t0_dn8 / locals.var_vth);
        locals.var_t1_dn9 = (locals.var_t0_dn9 / locals.var_vth);
        locals.var_t1_dn12 = (locals.var_t0_dn12 / locals.var_vth);
        locals.var_t1_dn14 = (locals.var_t0_dn14 / locals.var_vth);
        locals.var_t1_dn15 = (locals.var_t0_dn15 / locals.var_vth);
        locals.var_t1_dn16 = (locals.var_t0_dn16 / locals.var_vth);
        locals.var_t1_dn17 = (locals.var_t0_dn17 / locals.var_vth);
        locals.var_t1_dn18 = (locals.var_t0_dn18 / locals.var_vth);
        locals.var_t1_dn19 = (locals.var_t0_dn19 / locals.var_vth);
        locals.var_t1_dn20 = (locals.var_t0_dn20 / locals.var_vth);
        locals.var_t1_dn21 = (locals.var_t0_dn21 / locals.var_vth);
        locals.var_t1_dn22 = (locals.var_t0_dn22 / locals.var_vth);

        let assign32200_e49786: f64 = (p.p230 * locals.var_t1);
        let assign32200_e49788: f64 = (assign32200_e49786 * locals.var_t1);
        let assign32200_e49790: f64 = (assign32200_e49788 + 1.92);
        let assign32200_e49791: f64 = (assign32200_e49790).sqrt();
        locals.var_t2 = assign32200_e49791;
        locals.var_t2_dn0 = ((((p.p230 * locals.var_t1_dn0) * locals.var_t1) + (assign32200_e49786 * locals.var_t1_dn0)) / (2.0 * assign32200_e49791));
        locals.var_t2_dn1 = ((((p.p230 * locals.var_t1_dn1) * locals.var_t1) + (assign32200_e49786 * locals.var_t1_dn1)) / (2.0 * assign32200_e49791));
        locals.var_t2_dn2 = ((((p.p230 * locals.var_t1_dn2) * locals.var_t1) + (assign32200_e49786 * locals.var_t1_dn2)) / (2.0 * assign32200_e49791));
        locals.var_t2_dn3 = ((((p.p230 * locals.var_t1_dn3) * locals.var_t1) + (assign32200_e49786 * locals.var_t1_dn3)) / (2.0 * assign32200_e49791));
        locals.var_t2_dn4 = ((((p.p230 * locals.var_t1_dn4) * locals.var_t1) + (assign32200_e49786 * locals.var_t1_dn4)) / (2.0 * assign32200_e49791));
        locals.var_t2_dn5 = ((((p.p230 * locals.var_t1_dn5) * locals.var_t1) + (assign32200_e49786 * locals.var_t1_dn5)) / (2.0 * assign32200_e49791));
        locals.var_t2_dn6 = ((((p.p230 * locals.var_t1_dn6) * locals.var_t1) + (assign32200_e49786 * locals.var_t1_dn6)) / (2.0 * assign32200_e49791));
        locals.var_t2_dn7 = ((((p.p230 * locals.var_t1_dn7) * locals.var_t1) + (assign32200_e49786 * locals.var_t1_dn7)) / (2.0 * assign32200_e49791));
        locals.var_t2_dn8 = ((((p.p230 * locals.var_t1_dn8) * locals.var_t1) + (assign32200_e49786 * locals.var_t1_dn8)) / (2.0 * assign32200_e49791));
        locals.var_t2_dn9 = ((((p.p230 * locals.var_t1_dn9) * locals.var_t1) + (assign32200_e49786 * locals.var_t1_dn9)) / (2.0 * assign32200_e49791));
        locals.var_t2_dn12 = ((((p.p230 * locals.var_t1_dn12) * locals.var_t1) + (assign32200_e49786 * locals.var_t1_dn12)) / (2.0 * assign32200_e49791));
        locals.var_t2_dn14 = ((((p.p230 * locals.var_t1_dn14) * locals.var_t1) + (assign32200_e49786 * locals.var_t1_dn14)) / (2.0 * assign32200_e49791));
        locals.var_t2_dn15 = ((((p.p230 * locals.var_t1_dn15) * locals.var_t1) + (assign32200_e49786 * locals.var_t1_dn15)) / (2.0 * assign32200_e49791));
        locals.var_t2_dn16 = ((((p.p230 * locals.var_t1_dn16) * locals.var_t1) + (assign32200_e49786 * locals.var_t1_dn16)) / (2.0 * assign32200_e49791));
        locals.var_t2_dn17 = ((((p.p230 * locals.var_t1_dn17) * locals.var_t1) + (assign32200_e49786 * locals.var_t1_dn17)) / (2.0 * assign32200_e49791));
        locals.var_t2_dn18 = ((((p.p230 * locals.var_t1_dn18) * locals.var_t1) + (assign32200_e49786 * locals.var_t1_dn18)) / (2.0 * assign32200_e49791));
        locals.var_t2_dn19 = ((((p.p230 * locals.var_t1_dn19) * locals.var_t1) + (assign32200_e49786 * locals.var_t1_dn19)) / (2.0 * assign32200_e49791));
        locals.var_t2_dn20 = ((((p.p230 * locals.var_t1_dn20) * locals.var_t1) + (assign32200_e49786 * locals.var_t1_dn20)) / (2.0 * assign32200_e49791));
        locals.var_t2_dn21 = ((((p.p230 * locals.var_t1_dn21) * locals.var_t1) + (assign32200_e49786 * locals.var_t1_dn21)) / (2.0 * assign32200_e49791));
        locals.var_t2_dn22 = ((((p.p230 * locals.var_t1_dn22) * locals.var_t1) + (assign32200_e49786 * locals.var_t1_dn22)) / (2.0 * assign32200_e49791));

        let assign32210_e49794: f64 = (locals.var_t1 + locals.var_t2);
        let assign32210_e49796: f64 = (assign32210_e49794 * 0.5);
        locals.var_t3 = assign32210_e49796;
        locals.var_t3_dn0 = ((locals.var_t1_dn0 + locals.var_t2_dn0) * 0.5);
        locals.var_t3_dn1 = ((locals.var_t1_dn1 + locals.var_t2_dn1) * 0.5);
        locals.var_t3_dn2 = ((locals.var_t1_dn2 + locals.var_t2_dn2) * 0.5);
        locals.var_t3_dn3 = ((locals.var_t1_dn3 + locals.var_t2_dn3) * 0.5);
        locals.var_t3_dn4 = ((locals.var_t1_dn4 + locals.var_t2_dn4) * 0.5);
        locals.var_t3_dn5 = ((locals.var_t1_dn5 + locals.var_t2_dn5) * 0.5);
        locals.var_t3_dn6 = ((locals.var_t1_dn6 + locals.var_t2_dn6) * 0.5);
        locals.var_t3_dn7 = ((locals.var_t1_dn7 + locals.var_t2_dn7) * 0.5);
        locals.var_t3_dn8 = ((locals.var_t1_dn8 + locals.var_t2_dn8) * 0.5);
        locals.var_t3_dn9 = ((locals.var_t1_dn9 + locals.var_t2_dn9) * 0.5);
        locals.var_t3_dn12 = ((locals.var_t1_dn12 + locals.var_t2_dn12) * 0.5);
        locals.var_t3_dn14 = ((locals.var_t1_dn14 + locals.var_t2_dn14) * 0.5);
        locals.var_t3_dn15 = ((locals.var_t1_dn15 + locals.var_t2_dn15) * 0.5);
        locals.var_t3_dn16 = ((locals.var_t1_dn16 + locals.var_t2_dn16) * 0.5);
        locals.var_t3_dn17 = ((locals.var_t1_dn17 + locals.var_t2_dn17) * 0.5);
        locals.var_t3_dn18 = ((locals.var_t1_dn18 + locals.var_t2_dn18) * 0.5);
        locals.var_t3_dn19 = ((locals.var_t1_dn19 + locals.var_t2_dn19) * 0.5);
        locals.var_t3_dn20 = ((locals.var_t1_dn20 + locals.var_t2_dn20) * 0.5);
        locals.var_t3_dn21 = ((locals.var_t1_dn21 + locals.var_t2_dn21) * 0.5);
        locals.var_t3_dn22 = ((locals.var_t1_dn22 + locals.var_t2_dn22) * 0.5);

        let assign32220_e49800: f64 = (locals.var_vth * locals.var_t3);
        let assign32220_e49801: f64 = (locals.var_t0 - assign32220_e49800);
        locals.var_t4 = assign32220_e49801;
        locals.var_t4_dn0 = (locals.var_t0_dn0 - (locals.var_vth * locals.var_t3_dn0));
        locals.var_t4_dn1 = (locals.var_t0_dn1 - (locals.var_vth * locals.var_t3_dn1));
        locals.var_t4_dn2 = (locals.var_t0_dn2 - (locals.var_vth * locals.var_t3_dn2));
        locals.var_t4_dn3 = (locals.var_t0_dn3 - (locals.var_vth * locals.var_t3_dn3));
        locals.var_t4_dn4 = (locals.var_t0_dn4 - ((locals.var_vth_dn4 * locals.var_t3) + (locals.var_vth * locals.var_t3_dn4)));
        locals.var_t4_dn5 = (locals.var_t0_dn5 - (locals.var_vth * locals.var_t3_dn5));
        locals.var_t4_dn6 = (locals.var_t0_dn6 - (locals.var_vth * locals.var_t3_dn6));
        locals.var_t4_dn7 = (locals.var_t0_dn7 - (locals.var_vth * locals.var_t3_dn7));
        locals.var_t4_dn8 = (locals.var_t0_dn8 - (locals.var_vth * locals.var_t3_dn8));
        locals.var_t4_dn9 = (locals.var_t0_dn9 - (locals.var_vth * locals.var_t3_dn9));
        locals.var_t4_dn12 = (locals.var_t0_dn12 - (locals.var_vth * locals.var_t3_dn12));
        locals.var_t4_dn14 = (locals.var_t0_dn14 - (locals.var_vth * locals.var_t3_dn14));
        locals.var_t4_dn15 = (locals.var_t0_dn15 - (locals.var_vth * locals.var_t3_dn15));
        locals.var_t4_dn16 = (locals.var_t0_dn16 - (locals.var_vth * locals.var_t3_dn16));
        locals.var_t4_dn17 = (locals.var_t0_dn17 - (locals.var_vth * locals.var_t3_dn17));
        locals.var_t4_dn18 = (locals.var_t0_dn18 - (locals.var_vth * locals.var_t3_dn18));
        locals.var_t4_dn19 = (locals.var_t0_dn19 - (locals.var_vth * locals.var_t3_dn19));
        locals.var_t4_dn20 = (locals.var_t0_dn20 - (locals.var_vth * locals.var_t3_dn20));
        locals.var_t4_dn21 = (locals.var_t0_dn21 - (locals.var_vth * locals.var_t3_dn21));
        locals.var_t4_dn22 = (locals.var_t0_dn22 - (locals.var_vth * locals.var_t3_dn22));

        let assign32230_e49805: f64 = (locals.var_t4 / p.p224);
        let assign32230_e49806: f64 = (1.0 - assign32230_e49805);
        let assign32230_e49807: f64 = (assign32230_e49806).ln();
        locals.var_t6 = assign32230_e49807;
        locals.var_t6_dn0 = ((-(locals.var_t4_dn0 / p.p224)) / assign32230_e49806);
        locals.var_t6_dn1 = ((-(locals.var_t4_dn1 / p.p224)) / assign32230_e49806);
        locals.var_t6_dn2 = ((-(locals.var_t4_dn2 / p.p224)) / assign32230_e49806);
        locals.var_t6_dn3 = ((-(locals.var_t4_dn3 / p.p224)) / assign32230_e49806);
        locals.var_t6_dn4 = ((-(locals.var_t4_dn4 / p.p224)) / assign32230_e49806);
        locals.var_t6_dn5 = ((-(locals.var_t4_dn5 / p.p224)) / assign32230_e49806);
        locals.var_t6_dn6 = ((-(locals.var_t4_dn6 / p.p224)) / assign32230_e49806);
        locals.var_t6_dn7 = ((-(locals.var_t4_dn7 / p.p224)) / assign32230_e49806);
        locals.var_t6_dn8 = ((-(locals.var_t4_dn8 / p.p224)) / assign32230_e49806);
        locals.var_t6_dn9 = ((-(locals.var_t4_dn9 / p.p224)) / assign32230_e49806);
        locals.var_t6_dn12 = ((-(locals.var_t4_dn12 / p.p224)) / assign32230_e49806);
        locals.var_t6_dn14 = ((-(locals.var_t4_dn14 / p.p224)) / assign32230_e49806);
        locals.var_t6_dn15 = ((-(locals.var_t4_dn15 / p.p224)) / assign32230_e49806);
        locals.var_t6_dn16 = ((-(locals.var_t4_dn16 / p.p224)) / assign32230_e49806);
        locals.var_t6_dn17 = ((-(locals.var_t4_dn17 / p.p224)) / assign32230_e49806);
        locals.var_t6_dn18 = ((-(locals.var_t4_dn18 / p.p224)) / assign32230_e49806);
        locals.var_t6_dn19 = ((-(locals.var_t4_dn19 / p.p224)) / assign32230_e49806);
        locals.var_t6_dn20 = ((-(locals.var_t4_dn20 / p.p224)) / assign32230_e49806);
        locals.var_t6_dn21 = ((-(locals.var_t4_dn21 / p.p224)) / assign32230_e49806);
        locals.var_t6_dn22 = ((-(locals.var_t4_dn22 / p.p224)) / assign32230_e49806);

    }

    pub(super) fn stamp_transient_block_184(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let assign32240_e49812: f64 = (locals.var_tdev / locals.var_tnom);
        let assign32240_e49814: f64 = (assign32240_e49812 - 1.0);
        let assign32240_e49816: f64 = (assign32240_e49814 * p.p225);
        let assign32240_e49817: f64 = (p.p224 - assign32240_e49816);
        let assign32240_e49818: f64 = (p.p223 * assign32240_e49817);
        let assign32240_e49823: f64 = (1.0 - p.p228);
        let assign32240_e49824: f64 = (locals.var_t6 * assign32240_e49823);
        let assign32240_e49825: f64 = { let limited_exp_arg = assign32240_e49824; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign32240_e49826: f64 = (1.0 - assign32240_e49825);
        let assign32240_e49827: f64 = (assign32240_e49818 * assign32240_e49826);
        let assign32240_e49830: f64 = (1.0 - p.p228);
        let assign32240_e49831: f64 = (assign32240_e49827 / assign32240_e49830);
        locals.var_t8 = assign32240_e49831;
        locals.var_t8_dn0 = ((assign32240_e49818 * (-({ let limited_exp_arg = assign32240_e49824; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t6_dn0 * assign32240_e49823)))) / assign32240_e49830);
        locals.var_t8_dn1 = ((assign32240_e49818 * (-({ let limited_exp_arg = assign32240_e49824; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t6_dn1 * assign32240_e49823)))) / assign32240_e49830);
        locals.var_t8_dn2 = ((assign32240_e49818 * (-({ let limited_exp_arg = assign32240_e49824; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t6_dn2 * assign32240_e49823)))) / assign32240_e49830);
        locals.var_t8_dn3 = ((assign32240_e49818 * (-({ let limited_exp_arg = assign32240_e49824; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t6_dn3 * assign32240_e49823)))) / assign32240_e49830);
        locals.var_t8_dn4 = ((((p.p223 * (-((locals.var_tdev_dn4 / locals.var_tnom) * p.p225))) * assign32240_e49826) + (assign32240_e49818 * (-({ let limited_exp_arg = assign32240_e49824; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t6_dn4 * assign32240_e49823))))) / assign32240_e49830);
        locals.var_t8_dn5 = ((assign32240_e49818 * (-({ let limited_exp_arg = assign32240_e49824; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t6_dn5 * assign32240_e49823)))) / assign32240_e49830);
        locals.var_t8_dn6 = ((assign32240_e49818 * (-({ let limited_exp_arg = assign32240_e49824; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t6_dn6 * assign32240_e49823)))) / assign32240_e49830);
        locals.var_t8_dn7 = ((assign32240_e49818 * (-({ let limited_exp_arg = assign32240_e49824; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t6_dn7 * assign32240_e49823)))) / assign32240_e49830);
        locals.var_t8_dn8 = ((assign32240_e49818 * (-({ let limited_exp_arg = assign32240_e49824; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t6_dn8 * assign32240_e49823)))) / assign32240_e49830);
        locals.var_t8_dn9 = ((assign32240_e49818 * (-({ let limited_exp_arg = assign32240_e49824; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t6_dn9 * assign32240_e49823)))) / assign32240_e49830);
        locals.var_t8_dn12 = ((assign32240_e49818 * (-({ let limited_exp_arg = assign32240_e49824; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t6_dn12 * assign32240_e49823)))) / assign32240_e49830);
        locals.var_t8_dn14 = ((assign32240_e49818 * (-({ let limited_exp_arg = assign32240_e49824; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t6_dn14 * assign32240_e49823)))) / assign32240_e49830);
        locals.var_t8_dn15 = ((assign32240_e49818 * (-({ let limited_exp_arg = assign32240_e49824; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t6_dn15 * assign32240_e49823)))) / assign32240_e49830);
        locals.var_t8_dn16 = ((assign32240_e49818 * (-({ let limited_exp_arg = assign32240_e49824; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t6_dn16 * assign32240_e49823)))) / assign32240_e49830);
        locals.var_t8_dn17 = ((assign32240_e49818 * (-({ let limited_exp_arg = assign32240_e49824; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t6_dn17 * assign32240_e49823)))) / assign32240_e49830);
        locals.var_t8_dn18 = ((assign32240_e49818 * (-({ let limited_exp_arg = assign32240_e49824; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t6_dn18 * assign32240_e49823)))) / assign32240_e49830);
        locals.var_t8_dn19 = ((assign32240_e49818 * (-({ let limited_exp_arg = assign32240_e49824; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t6_dn19 * assign32240_e49823)))) / assign32240_e49830);
        locals.var_t8_dn20 = ((assign32240_e49818 * (-({ let limited_exp_arg = assign32240_e49824; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t6_dn20 * assign32240_e49823)))) / assign32240_e49830);
        locals.var_t8_dn21 = ((assign32240_e49818 * (-({ let limited_exp_arg = assign32240_e49824; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t6_dn21 * assign32240_e49823)))) / assign32240_e49830);
        locals.var_t8_dn22 = ((assign32240_e49818 * (-({ let limited_exp_arg = assign32240_e49824; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t6_dn22 * assign32240_e49823)))) / assign32240_e49830);

        let assign32250_e49834: f64 = (p.p4 * p.p5);
        let assign32250_e49838: f64 = (p.p229 * p.p223);
        let assign32250_e49841: f64 = ((nv2 - nv0) - locals.var_t4);
        let assign32250_e49842: f64 = (assign32250_e49838 * assign32250_e49841);
        let assign32250_e49843: f64 = (locals.var_t8 + assign32250_e49842);
        let assign32250_e49844: f64 = (assign32250_e49834 * assign32250_e49843);
        locals.var_qdep = assign32250_e49844;
        locals.var_qdep_dn0 = (assign32250_e49834 * (locals.var_t8_dn0 + (assign32250_e49838 * (-1.0 - locals.var_t4_dn0))));
        locals.var_qdep_dn1 = (assign32250_e49834 * (locals.var_t8_dn1 + (assign32250_e49838 * (-locals.var_t4_dn1))));
        locals.var_qdep_dn2 = (assign32250_e49834 * (locals.var_t8_dn2 + (assign32250_e49838 * (1.0 - locals.var_t4_dn2))));
        locals.var_qdep_dn3 = (assign32250_e49834 * (locals.var_t8_dn3 + (assign32250_e49838 * (-locals.var_t4_dn3))));
        locals.var_qdep_dn4 = (assign32250_e49834 * (locals.var_t8_dn4 + (assign32250_e49838 * (-locals.var_t4_dn4))));
        locals.var_qdep_dn5 = (assign32250_e49834 * (locals.var_t8_dn5 + (assign32250_e49838 * (-locals.var_t4_dn5))));
        locals.var_qdep_dn6 = (assign32250_e49834 * (locals.var_t8_dn6 + (assign32250_e49838 * (-locals.var_t4_dn6))));
        locals.var_qdep_dn7 = (assign32250_e49834 * (locals.var_t8_dn7 + (assign32250_e49838 * (-locals.var_t4_dn7))));
        locals.var_qdep_dn8 = (assign32250_e49834 * (locals.var_t8_dn8 + (assign32250_e49838 * (-locals.var_t4_dn8))));
        locals.var_qdep_dn9 = (assign32250_e49834 * (locals.var_t8_dn9 + (assign32250_e49838 * (-locals.var_t4_dn9))));
        locals.var_qdep_dn12 = (assign32250_e49834 * (locals.var_t8_dn12 + (assign32250_e49838 * (-locals.var_t4_dn12))));
        locals.var_qdep_dn14 = (assign32250_e49834 * (locals.var_t8_dn14 + (assign32250_e49838 * (-locals.var_t4_dn14))));
        locals.var_qdep_dn15 = (assign32250_e49834 * (locals.var_t8_dn15 + (assign32250_e49838 * (-locals.var_t4_dn15))));
        locals.var_qdep_dn16 = (assign32250_e49834 * (locals.var_t8_dn16 + (assign32250_e49838 * (-locals.var_t4_dn16))));
        locals.var_qdep_dn17 = (assign32250_e49834 * (locals.var_t8_dn17 + (assign32250_e49838 * (-locals.var_t4_dn17))));
        locals.var_qdep_dn18 = (assign32250_e49834 * (locals.var_t8_dn18 + (assign32250_e49838 * (-locals.var_t4_dn18))));
        locals.var_qdep_dn19 = (assign32250_e49834 * (locals.var_t8_dn19 + (assign32250_e49838 * (-locals.var_t4_dn19))));
        locals.var_qdep_dn20 = (assign32250_e49834 * (locals.var_t8_dn20 + (assign32250_e49838 * (-locals.var_t4_dn20))));
        locals.var_qdep_dn21 = (assign32250_e49834 * (locals.var_t8_dn21 + (assign32250_e49838 * (-locals.var_t4_dn21))));
        locals.var_qdep_dn22 = (assign32250_e49834 * (locals.var_t8_dn22 + (assign32250_e49838 * (-locals.var_t4_dn22))));

        let assign32260_e49851: f64 = if ((p.p31 == 1.0) && (p.p32 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard576 = assign32260_e49851;

    }

    pub(super) fn stamp_reactive_block_0(
        locals: &mut StampLocals,
    ) {
        locals.var_t6 = 0.0;
        locals.var_t6_dn0 = 0.0;
        locals.var_t6_dn1 = 0.0;
        locals.var_t6_dn2 = 0.0;
        locals.var_t6_dn3 = 0.0;
        locals.var_t6_dn4 = 0.0;
        locals.var_t6_dn5 = 0.0;
        locals.var_t6_dn6 = 0.0;
        locals.var_t6_dn7 = 0.0;
        locals.var_t6_dn8 = 0.0;
        locals.var_t6_dn9 = 0.0;
        locals.var_t6_dn12 = 0.0;
        locals.var_t6_dn14 = 0.0;
        locals.var_t6_dn15 = 0.0;
        locals.var_t6_dn16 = 0.0;
        locals.var_t6_dn17 = 0.0;
        locals.var_t6_dn18 = 0.0;
        locals.var_t6_dn19 = 0.0;
        locals.var_t6_dn20 = 0.0;
        locals.var_t6_dn21 = 0.0;
        locals.var_t6_dn22 = 0.0;
        locals.var_t6_rv = 0.0;

        locals.var_t8 = 0.0;
        locals.var_t8_dn0 = 0.0;
        locals.var_t8_dn1 = 0.0;
        locals.var_t8_dn2 = 0.0;
        locals.var_t8_dn3 = 0.0;
        locals.var_t8_dn4 = 0.0;
        locals.var_t8_dn5 = 0.0;
        locals.var_t8_dn6 = 0.0;
        locals.var_t8_dn7 = 0.0;
        locals.var_t8_dn8 = 0.0;
        locals.var_t8_dn9 = 0.0;
        locals.var_t8_dn12 = 0.0;
        locals.var_t8_dn14 = 0.0;
        locals.var_t8_dn15 = 0.0;
        locals.var_t8_dn16 = 0.0;
        locals.var_t8_dn17 = 0.0;
        locals.var_t8_dn18 = 0.0;
        locals.var_t8_dn19 = 0.0;
        locals.var_t8_dn20 = 0.0;
        locals.var_t8_dn21 = 0.0;
        locals.var_t8_dn22 = 0.0;
        locals.var_t8_rv = 0.0;

        locals.var_qdep = 0.0;
        locals.var_qdep_dn0 = 0.0;
        locals.var_qdep_dn1 = 0.0;
        locals.var_qdep_dn2 = 0.0;
        locals.var_qdep_dn3 = 0.0;
        locals.var_qdep_dn4 = 0.0;
        locals.var_qdep_dn5 = 0.0;
        locals.var_qdep_dn6 = 0.0;
        locals.var_qdep_dn7 = 0.0;
        locals.var_qdep_dn8 = 0.0;
        locals.var_qdep_dn9 = 0.0;
        locals.var_qdep_dn12 = 0.0;
        locals.var_qdep_dn14 = 0.0;
        locals.var_qdep_dn15 = 0.0;
        locals.var_qdep_dn16 = 0.0;
        locals.var_qdep_dn17 = 0.0;
        locals.var_qdep_dn18 = 0.0;
        locals.var_qdep_dn19 = 0.0;
        locals.var_qdep_dn20 = 0.0;
        locals.var_qdep_dn21 = 0.0;
        locals.var_qdep_dn22 = 0.0;
        locals.var_qdep_rv = 0.0;

        locals.var_qfr = 0.0;
        locals.var_qfr_dn0 = 0.0;
        locals.var_qfr_dn2 = 0.0;
        locals.var_qfr_dn4 = 0.0;
        locals.var_qfr_rv = 0.0;

        locals.var_qfr2 = 0.0;
        locals.var_qfr2_dn0 = 0.0;
        locals.var_qfr2_dn1 = 0.0;
        locals.var_qfr2_dn2 = 0.0;
        locals.var_qfr2_dn3 = 0.0;
        locals.var_qfr2_dn4 = 0.0;
        locals.var_qfr2_dn5 = 0.0;
        locals.var_qfr2_dn6 = 0.0;
        locals.var_qfr2_dn7 = 0.0;
        locals.var_qfr2_dn8 = 0.0;
        locals.var_qfr2_dn9 = 0.0;
        locals.var_qfr2_dn12 = 0.0;
        locals.var_qfr2_dn14 = 0.0;
        locals.var_qfr2_dn15 = 0.0;
        locals.var_qfr2_dn16 = 0.0;
        locals.var_qfr2_dn17 = 0.0;
        locals.var_qfr2_dn18 = 0.0;
        locals.var_qfr2_dn19 = 0.0;
        locals.var_qfr2_dn20 = 0.0;
        locals.var_qfr2_dn21 = 0.0;
        locals.var_qfr2_dn22 = 0.0;
        locals.var_qfr2_rv = 0.0;

        locals.var_qfr3 = 0.0;
        locals.var_qfr3_dn0 = 0.0;
        locals.var_qfr3_dn2 = 0.0;
        locals.var_qfr3_rv = 0.0;

        locals.var_vcap = 1.0;
        locals.var_vcap_dn4 = 0.0;
        locals.var_vcap_dn5 = 0.0;
        locals.var_vcap_rv = 0.0;

        locals.var_voff_cap = 0.0;
        locals.var_voff_cap_dn4 = 0.0;
        locals.var_voff_cap_dn5 = 0.0;
        locals.var_voff_cap_rv = 0.0;

        locals.var_eta0_cap = 0.0;
        locals.var_eta0_cap_dn4 = 0.0;
        locals.var_eta0_cap_dn5 = 0.0;
        locals.var_eta0_cap_rv = 0.0;

        locals.var_ids = 0.0;
        locals.var_ids_dn0 = 0.0;
        locals.var_ids_dn1 = 0.0;
        locals.var_ids_dn2 = 0.0;
        locals.var_ids_dn3 = 0.0;
        locals.var_ids_dn4 = 0.0;
        locals.var_ids_dn5 = 0.0;
        locals.var_ids_dn6 = 0.0;
        locals.var_ids_dn7 = 0.0;
        locals.var_ids_dn8 = 0.0;
        locals.var_ids_dn9 = 0.0;
        locals.var_ids_dn12 = 0.0;
        locals.var_ids_dn14 = 0.0;
        locals.var_ids_dn15 = 0.0;
        locals.var_ids_dn16 = 0.0;
        locals.var_ids_dn17 = 0.0;
        locals.var_ids_dn18 = 0.0;
        locals.var_ids_dn19 = 0.0;
        locals.var_ids_dn20 = 0.0;
        locals.var_ids_dn21 = 0.0;
        locals.var_ids_dn22 = 0.0;
        locals.var_ids_rv = 0.0;

        locals.var_voff_trap = 0.0;
        locals.var_voff_trap_dn6 = 0.0;
        locals.var_voff_trap_rv = 0.0;

        locals.var_cdscd_trap = 0.0;
        locals.var_cdscd_trap_dn6 = 0.0;
        locals.var_cdscd_trap_rv = 0.0;

        locals.var_eta0_trap = 0.0;
        locals.var_eta0_trap_dn6 = 0.0;
        locals.var_eta0_trap_rv = 0.0;

        locals.var_vg0_fp1 = 0.0;
        locals.var_vg0_fp1_dn0 = 0.0;
        locals.var_vg0_fp1_dn1 = 0.0;
        locals.var_vg0_fp1_dn2 = 0.0;
        locals.var_vg0_fp1_dn3 = 0.0;
        locals.var_vg0_fp1_dn4 = 0.0;
        locals.var_vg0_fp1_dn5 = 0.0;
        locals.var_vg0_fp1_dn6 = 0.0;
        locals.var_vg0_fp1_dn7 = 0.0;
        locals.var_vg0_fp1_dn8 = 0.0;
        locals.var_vg0_fp1_dn9 = 0.0;
        locals.var_vg0_fp1_dn12 = 0.0;
        locals.var_vg0_fp1_dn14 = 0.0;
        locals.var_vg0_fp1_dn15 = 0.0;
        locals.var_vg0_fp1_dn16 = 0.0;
        locals.var_vg0_fp1_dn17 = 0.0;
        locals.var_vg0_fp1_dn18 = 0.0;
        locals.var_vg0_fp1_dn19 = 0.0;
        locals.var_vg0_fp1_dn20 = 0.0;
        locals.var_vg0_fp1_dn21 = 0.0;
        locals.var_vg0_fp1_dn22 = 0.0;
        locals.var_vg0_fp1_rv = 0.0;

        locals.var_cg_fp1 = 0.0;
        locals.var_cg_fp1_rv = 0.0;

        locals.var_psis_fp1 = 0.0;
        locals.var_psis_fp1_dn0 = 0.0;
        locals.var_psis_fp1_dn1 = 0.0;
        locals.var_psis_fp1_dn2 = 0.0;
        locals.var_psis_fp1_dn3 = 0.0;
        locals.var_psis_fp1_dn4 = 0.0;
        locals.var_psis_fp1_dn5 = 0.0;
        locals.var_psis_fp1_dn6 = 0.0;
        locals.var_psis_fp1_dn7 = 0.0;
        locals.var_psis_fp1_dn8 = 0.0;
        locals.var_psis_fp1_dn9 = 0.0;
        locals.var_psis_fp1_dn12 = 0.0;
        locals.var_psis_fp1_dn14 = 0.0;
        locals.var_psis_fp1_dn15 = 0.0;
        locals.var_psis_fp1_dn16 = 0.0;
        locals.var_psis_fp1_dn17 = 0.0;
        locals.var_psis_fp1_dn18 = 0.0;
        locals.var_psis_fp1_dn19 = 0.0;
        locals.var_psis_fp1_dn20 = 0.0;
        locals.var_psis_fp1_dn21 = 0.0;
        locals.var_psis_fp1_dn22 = 0.0;
        locals.var_psis_fp1_rv = 0.0;

        locals.var_psid_fp1 = 0.0;
        locals.var_psid_fp1_dn0 = 0.0;
        locals.var_psid_fp1_dn1 = 0.0;
        locals.var_psid_fp1_dn2 = 0.0;
        locals.var_psid_fp1_dn3 = 0.0;
        locals.var_psid_fp1_dn4 = 0.0;
        locals.var_psid_fp1_dn5 = 0.0;
        locals.var_psid_fp1_dn6 = 0.0;
        locals.var_psid_fp1_dn7 = 0.0;
        locals.var_psid_fp1_dn8 = 0.0;
        locals.var_psid_fp1_dn9 = 0.0;
        locals.var_psid_fp1_dn12 = 0.0;
        locals.var_psid_fp1_dn14 = 0.0;
        locals.var_psid_fp1_dn15 = 0.0;
        locals.var_psid_fp1_dn16 = 0.0;
        locals.var_psid_fp1_dn17 = 0.0;
        locals.var_psid_fp1_dn18 = 0.0;
        locals.var_psid_fp1_dn19 = 0.0;
        locals.var_psid_fp1_dn20 = 0.0;
        locals.var_psid_fp1_dn21 = 0.0;
        locals.var_psid_fp1_dn22 = 0.0;
        locals.var_psid_fp1_rv = 0.0;

        locals.var_psim_fp1 = 0.0;
        locals.var_psim_fp1_dn0 = 0.0;
        locals.var_psim_fp1_dn1 = 0.0;
        locals.var_psim_fp1_dn2 = 0.0;
        locals.var_psim_fp1_dn3 = 0.0;
        locals.var_psim_fp1_dn4 = 0.0;
        locals.var_psim_fp1_dn5 = 0.0;
        locals.var_psim_fp1_dn6 = 0.0;
        locals.var_psim_fp1_dn7 = 0.0;
        locals.var_psim_fp1_dn8 = 0.0;
        locals.var_psim_fp1_dn9 = 0.0;
        locals.var_psim_fp1_dn12 = 0.0;
        locals.var_psim_fp1_dn14 = 0.0;
        locals.var_psim_fp1_dn15 = 0.0;
        locals.var_psim_fp1_dn16 = 0.0;
        locals.var_psim_fp1_dn17 = 0.0;
        locals.var_psim_fp1_dn18 = 0.0;
        locals.var_psim_fp1_dn19 = 0.0;
        locals.var_psim_fp1_dn20 = 0.0;
        locals.var_psim_fp1_dn21 = 0.0;
        locals.var_psim_fp1_dn22 = 0.0;
        locals.var_psim_fp1_rv = 0.0;

        locals.var_psisd_fp1 = 0.0;
        locals.var_psisd_fp1_dn0 = 0.0;
        locals.var_psisd_fp1_dn1 = 0.0;
        locals.var_psisd_fp1_dn2 = 0.0;
        locals.var_psisd_fp1_dn3 = 0.0;
        locals.var_psisd_fp1_dn4 = 0.0;
        locals.var_psisd_fp1_dn5 = 0.0;
        locals.var_psisd_fp1_dn6 = 0.0;
        locals.var_psisd_fp1_dn7 = 0.0;
        locals.var_psisd_fp1_dn8 = 0.0;
        locals.var_psisd_fp1_dn9 = 0.0;
        locals.var_psisd_fp1_dn12 = 0.0;
        locals.var_psisd_fp1_dn14 = 0.0;
        locals.var_psisd_fp1_dn15 = 0.0;
        locals.var_psisd_fp1_dn16 = 0.0;
        locals.var_psisd_fp1_dn17 = 0.0;
        locals.var_psisd_fp1_dn18 = 0.0;
        locals.var_psisd_fp1_dn19 = 0.0;
        locals.var_psisd_fp1_dn20 = 0.0;
        locals.var_psisd_fp1_dn21 = 0.0;
        locals.var_psisd_fp1_dn22 = 0.0;
        locals.var_psisd_fp1_rv = 0.0;

        locals.var_qg_fp1 = 0.0;
        locals.var_qg_fp1_dn0 = 0.0;
        locals.var_qg_fp1_dn1 = 0.0;
        locals.var_qg_fp1_dn2 = 0.0;
        locals.var_qg_fp1_dn3 = 0.0;
        locals.var_qg_fp1_dn4 = 0.0;
        locals.var_qg_fp1_dn5 = 0.0;
        locals.var_qg_fp1_dn6 = 0.0;
        locals.var_qg_fp1_dn7 = 0.0;
        locals.var_qg_fp1_dn8 = 0.0;
        locals.var_qg_fp1_dn9 = 0.0;
        locals.var_qg_fp1_dn12 = 0.0;
        locals.var_qg_fp1_dn14 = 0.0;
        locals.var_qg_fp1_dn15 = 0.0;
        locals.var_qg_fp1_dn16 = 0.0;
        locals.var_qg_fp1_dn17 = 0.0;
        locals.var_qg_fp1_dn18 = 0.0;
        locals.var_qg_fp1_dn19 = 0.0;
        locals.var_qg_fp1_dn20 = 0.0;
        locals.var_qg_fp1_dn21 = 0.0;
        locals.var_qg_fp1_dn22 = 0.0;
        locals.var_qg_fp1_rv = 0.0;

        locals.var_qd_fp1 = 0.0;
        locals.var_qd_fp1_dn0 = 0.0;
        locals.var_qd_fp1_dn1 = 0.0;
        locals.var_qd_fp1_dn2 = 0.0;
        locals.var_qd_fp1_dn3 = 0.0;
        locals.var_qd_fp1_dn4 = 0.0;
        locals.var_qd_fp1_dn5 = 0.0;
        locals.var_qd_fp1_dn6 = 0.0;
        locals.var_qd_fp1_dn7 = 0.0;
        locals.var_qd_fp1_dn8 = 0.0;
        locals.var_qd_fp1_dn9 = 0.0;
        locals.var_qd_fp1_dn12 = 0.0;
        locals.var_qd_fp1_dn14 = 0.0;
        locals.var_qd_fp1_dn15 = 0.0;
        locals.var_qd_fp1_dn16 = 0.0;
        locals.var_qd_fp1_dn17 = 0.0;
        locals.var_qd_fp1_dn18 = 0.0;
        locals.var_qd_fp1_dn19 = 0.0;
        locals.var_qd_fp1_dn20 = 0.0;
        locals.var_qd_fp1_dn21 = 0.0;
        locals.var_qd_fp1_dn22 = 0.0;
        locals.var_qd_fp1_rv = 0.0;

        locals.var_vgs_fp1 = 0.0;
        locals.var_vgs_fp1_dn2 = 0.0;
        locals.var_vgs_fp1_dn7 = 0.0;
        locals.var_vgs_fp1_dn9 = 0.0;
        locals.var_vgs_fp1_dn15 = 0.0;
        locals.var_vgs_fp1_rv = 0.0;

        locals.var_vds_fp1 = 0.0;
        locals.var_vds_fp1_dn7 = 0.0;
        locals.var_vds_fp1_dn15 = 0.0;
        locals.var_vds_fp1_rv = 0.0;

        locals.var_vg0_fp1s = 0.0;
        locals.var_vg0_fp1s_dn0 = 0.0;
        locals.var_vg0_fp1s_dn1 = 0.0;
        locals.var_vg0_fp1s_dn2 = 0.0;
        locals.var_vg0_fp1s_dn3 = 0.0;
        locals.var_vg0_fp1s_dn4 = 0.0;
        locals.var_vg0_fp1s_dn5 = 0.0;
        locals.var_vg0_fp1s_dn6 = 0.0;
        locals.var_vg0_fp1s_dn7 = 0.0;
        locals.var_vg0_fp1s_dn8 = 0.0;
        locals.var_vg0_fp1s_dn9 = 0.0;
        locals.var_vg0_fp1s_dn12 = 0.0;
        locals.var_vg0_fp1s_dn14 = 0.0;
        locals.var_vg0_fp1s_dn15 = 0.0;
        locals.var_vg0_fp1s_dn16 = 0.0;
        locals.var_vg0_fp1s_dn17 = 0.0;
        locals.var_vg0_fp1s_dn18 = 0.0;
        locals.var_vg0_fp1s_dn19 = 0.0;
        locals.var_vg0_fp1s_dn20 = 0.0;
        locals.var_vg0_fp1s_dn21 = 0.0;
        locals.var_vg0_fp1s_dn22 = 0.0;
        locals.var_vg0_fp1s_rv = 0.0;

        locals.var_cg_fp1s = 0.0;
        locals.var_cg_fp1s_rv = 0.0;

        locals.var_psis_fp1s = 0.0;
        locals.var_psis_fp1s_dn0 = 0.0;
        locals.var_psis_fp1s_dn1 = 0.0;
        locals.var_psis_fp1s_dn2 = 0.0;
        locals.var_psis_fp1s_dn3 = 0.0;
        locals.var_psis_fp1s_dn4 = 0.0;
        locals.var_psis_fp1s_dn5 = 0.0;
        locals.var_psis_fp1s_dn6 = 0.0;
        locals.var_psis_fp1s_dn7 = 0.0;
        locals.var_psis_fp1s_dn8 = 0.0;
        locals.var_psis_fp1s_dn9 = 0.0;
        locals.var_psis_fp1s_dn12 = 0.0;
        locals.var_psis_fp1s_dn14 = 0.0;
        locals.var_psis_fp1s_dn15 = 0.0;
        locals.var_psis_fp1s_dn16 = 0.0;
        locals.var_psis_fp1s_dn17 = 0.0;
        locals.var_psis_fp1s_dn18 = 0.0;
        locals.var_psis_fp1s_dn19 = 0.0;
        locals.var_psis_fp1s_dn20 = 0.0;
        locals.var_psis_fp1s_dn21 = 0.0;
        locals.var_psis_fp1s_dn22 = 0.0;
        locals.var_psis_fp1s_rv = 0.0;

        locals.var_psid_fp1s = 0.0;
        locals.var_psid_fp1s_dn0 = 0.0;
        locals.var_psid_fp1s_dn1 = 0.0;
        locals.var_psid_fp1s_dn2 = 0.0;
        locals.var_psid_fp1s_dn3 = 0.0;
        locals.var_psid_fp1s_dn4 = 0.0;
        locals.var_psid_fp1s_dn5 = 0.0;
        locals.var_psid_fp1s_dn6 = 0.0;
        locals.var_psid_fp1s_dn7 = 0.0;
        locals.var_psid_fp1s_dn8 = 0.0;
        locals.var_psid_fp1s_dn9 = 0.0;
        locals.var_psid_fp1s_dn12 = 0.0;
        locals.var_psid_fp1s_dn14 = 0.0;
        locals.var_psid_fp1s_dn15 = 0.0;
        locals.var_psid_fp1s_dn16 = 0.0;
        locals.var_psid_fp1s_dn17 = 0.0;
        locals.var_psid_fp1s_dn18 = 0.0;
        locals.var_psid_fp1s_dn19 = 0.0;
        locals.var_psid_fp1s_dn20 = 0.0;
        locals.var_psid_fp1s_dn21 = 0.0;
        locals.var_psid_fp1s_dn22 = 0.0;
        locals.var_psid_fp1s_rv = 0.0;

        locals.var_psim_fp1s = 0.0;
        locals.var_psim_fp1s_dn0 = 0.0;
        locals.var_psim_fp1s_dn1 = 0.0;
        locals.var_psim_fp1s_dn2 = 0.0;
        locals.var_psim_fp1s_dn3 = 0.0;
        locals.var_psim_fp1s_dn4 = 0.0;
        locals.var_psim_fp1s_dn5 = 0.0;
        locals.var_psim_fp1s_dn6 = 0.0;
        locals.var_psim_fp1s_dn7 = 0.0;
        locals.var_psim_fp1s_dn8 = 0.0;
        locals.var_psim_fp1s_dn9 = 0.0;
        locals.var_psim_fp1s_dn12 = 0.0;
        locals.var_psim_fp1s_dn14 = 0.0;
        locals.var_psim_fp1s_dn15 = 0.0;
        locals.var_psim_fp1s_dn16 = 0.0;
        locals.var_psim_fp1s_dn17 = 0.0;
        locals.var_psim_fp1s_dn18 = 0.0;
        locals.var_psim_fp1s_dn19 = 0.0;
        locals.var_psim_fp1s_dn20 = 0.0;
        locals.var_psim_fp1s_dn21 = 0.0;
        locals.var_psim_fp1s_dn22 = 0.0;
        locals.var_psim_fp1s_rv = 0.0;

        locals.var_psisd_fp1s = 0.0;
        locals.var_psisd_fp1s_dn0 = 0.0;
        locals.var_psisd_fp1s_dn1 = 0.0;
        locals.var_psisd_fp1s_dn2 = 0.0;
        locals.var_psisd_fp1s_dn3 = 0.0;
        locals.var_psisd_fp1s_dn4 = 0.0;
        locals.var_psisd_fp1s_dn5 = 0.0;
        locals.var_psisd_fp1s_dn6 = 0.0;
        locals.var_psisd_fp1s_dn7 = 0.0;
        locals.var_psisd_fp1s_dn8 = 0.0;
        locals.var_psisd_fp1s_dn9 = 0.0;
        locals.var_psisd_fp1s_dn12 = 0.0;
        locals.var_psisd_fp1s_dn14 = 0.0;
        locals.var_psisd_fp1s_dn15 = 0.0;
        locals.var_psisd_fp1s_dn16 = 0.0;
        locals.var_psisd_fp1s_dn17 = 0.0;
        locals.var_psisd_fp1s_dn18 = 0.0;
        locals.var_psisd_fp1s_dn19 = 0.0;
        locals.var_psisd_fp1s_dn20 = 0.0;
        locals.var_psisd_fp1s_dn21 = 0.0;
        locals.var_psisd_fp1s_dn22 = 0.0;
        locals.var_psisd_fp1s_rv = 0.0;

        locals.var_qg_fp1s = 0.0;
        locals.var_qg_fp1s_dn0 = 0.0;
        locals.var_qg_fp1s_dn1 = 0.0;
        locals.var_qg_fp1s_dn2 = 0.0;
        locals.var_qg_fp1s_dn3 = 0.0;
        locals.var_qg_fp1s_dn4 = 0.0;
        locals.var_qg_fp1s_dn5 = 0.0;
        locals.var_qg_fp1s_dn6 = 0.0;
        locals.var_qg_fp1s_dn7 = 0.0;
        locals.var_qg_fp1s_dn8 = 0.0;
        locals.var_qg_fp1s_dn9 = 0.0;
        locals.var_qg_fp1s_dn12 = 0.0;
        locals.var_qg_fp1s_dn14 = 0.0;
        locals.var_qg_fp1s_dn15 = 0.0;
        locals.var_qg_fp1s_dn16 = 0.0;
        locals.var_qg_fp1s_dn17 = 0.0;
        locals.var_qg_fp1s_dn18 = 0.0;
        locals.var_qg_fp1s_dn19 = 0.0;
        locals.var_qg_fp1s_dn20 = 0.0;
        locals.var_qg_fp1s_dn21 = 0.0;
        locals.var_qg_fp1s_dn22 = 0.0;
        locals.var_qg_fp1s_rv = 0.0;

        locals.var_qd_fp1s = 0.0;
        locals.var_qd_fp1s_dn0 = 0.0;
        locals.var_qd_fp1s_dn1 = 0.0;
        locals.var_qd_fp1s_dn2 = 0.0;
        locals.var_qd_fp1s_dn3 = 0.0;
        locals.var_qd_fp1s_dn4 = 0.0;
        locals.var_qd_fp1s_dn5 = 0.0;
        locals.var_qd_fp1s_dn6 = 0.0;
        locals.var_qd_fp1s_dn7 = 0.0;
        locals.var_qd_fp1s_dn8 = 0.0;
        locals.var_qd_fp1s_dn9 = 0.0;
        locals.var_qd_fp1s_dn12 = 0.0;
        locals.var_qd_fp1s_dn14 = 0.0;
        locals.var_qd_fp1s_dn15 = 0.0;
        locals.var_qd_fp1s_dn16 = 0.0;
        locals.var_qd_fp1s_dn17 = 0.0;
        locals.var_qd_fp1s_dn18 = 0.0;
        locals.var_qd_fp1s_dn19 = 0.0;
        locals.var_qd_fp1s_dn20 = 0.0;
        locals.var_qd_fp1s_dn21 = 0.0;
        locals.var_qd_fp1s_dn22 = 0.0;
        locals.var_qd_fp1s_rv = 0.0;

        locals.var_vgs_fp1s = 0.0;
        locals.var_vgs_fp1s_dn2 = 0.0;
        locals.var_vgs_fp1s_dn8 = 0.0;
        locals.var_vgs_fp1s_dn9 = 0.0;
        locals.var_vgs_fp1s_dn19 = 0.0;
        locals.var_vgs_fp1s_rv = 0.0;

        locals.var_vds_fp1s = 0.0;
        locals.var_vds_fp1s_dn8 = 0.0;
        locals.var_vds_fp1s_dn19 = 0.0;
        locals.var_vds_fp1s_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_1(
        locals: &mut StampLocals,
    ) {
        locals.var_vg0_fp2 = 0.0;
        locals.var_vg0_fp2_dn0 = 0.0;
        locals.var_vg0_fp2_dn1 = 0.0;
        locals.var_vg0_fp2_dn2 = 0.0;
        locals.var_vg0_fp2_dn3 = 0.0;
        locals.var_vg0_fp2_dn4 = 0.0;
        locals.var_vg0_fp2_dn5 = 0.0;
        locals.var_vg0_fp2_dn6 = 0.0;
        locals.var_vg0_fp2_dn7 = 0.0;
        locals.var_vg0_fp2_dn8 = 0.0;
        locals.var_vg0_fp2_dn9 = 0.0;
        locals.var_vg0_fp2_dn12 = 0.0;
        locals.var_vg0_fp2_dn14 = 0.0;
        locals.var_vg0_fp2_dn15 = 0.0;
        locals.var_vg0_fp2_dn16 = 0.0;
        locals.var_vg0_fp2_dn17 = 0.0;
        locals.var_vg0_fp2_dn18 = 0.0;
        locals.var_vg0_fp2_dn19 = 0.0;
        locals.var_vg0_fp2_dn20 = 0.0;
        locals.var_vg0_fp2_dn21 = 0.0;
        locals.var_vg0_fp2_dn22 = 0.0;
        locals.var_vg0_fp2_rv = 0.0;

        locals.var_cg_fp2 = 0.0;
        locals.var_cg_fp2_rv = 0.0;

        locals.var_psis_fp2 = 0.0;
        locals.var_psis_fp2_dn0 = 0.0;
        locals.var_psis_fp2_dn1 = 0.0;
        locals.var_psis_fp2_dn2 = 0.0;
        locals.var_psis_fp2_dn3 = 0.0;
        locals.var_psis_fp2_dn4 = 0.0;
        locals.var_psis_fp2_dn5 = 0.0;
        locals.var_psis_fp2_dn6 = 0.0;
        locals.var_psis_fp2_dn7 = 0.0;
        locals.var_psis_fp2_dn8 = 0.0;
        locals.var_psis_fp2_dn9 = 0.0;
        locals.var_psis_fp2_dn12 = 0.0;
        locals.var_psis_fp2_dn14 = 0.0;
        locals.var_psis_fp2_dn15 = 0.0;
        locals.var_psis_fp2_dn16 = 0.0;
        locals.var_psis_fp2_dn17 = 0.0;
        locals.var_psis_fp2_dn18 = 0.0;
        locals.var_psis_fp2_dn19 = 0.0;
        locals.var_psis_fp2_dn20 = 0.0;
        locals.var_psis_fp2_dn21 = 0.0;
        locals.var_psis_fp2_dn22 = 0.0;
        locals.var_psis_fp2_rv = 0.0;

        locals.var_psid_fp2 = 0.0;
        locals.var_psid_fp2_dn0 = 0.0;
        locals.var_psid_fp2_dn1 = 0.0;
        locals.var_psid_fp2_dn2 = 0.0;
        locals.var_psid_fp2_dn3 = 0.0;
        locals.var_psid_fp2_dn4 = 0.0;
        locals.var_psid_fp2_dn5 = 0.0;
        locals.var_psid_fp2_dn6 = 0.0;
        locals.var_psid_fp2_dn7 = 0.0;
        locals.var_psid_fp2_dn8 = 0.0;
        locals.var_psid_fp2_dn9 = 0.0;
        locals.var_psid_fp2_dn12 = 0.0;
        locals.var_psid_fp2_dn14 = 0.0;
        locals.var_psid_fp2_dn15 = 0.0;
        locals.var_psid_fp2_dn16 = 0.0;
        locals.var_psid_fp2_dn17 = 0.0;
        locals.var_psid_fp2_dn18 = 0.0;
        locals.var_psid_fp2_dn19 = 0.0;
        locals.var_psid_fp2_dn20 = 0.0;
        locals.var_psid_fp2_dn21 = 0.0;
        locals.var_psid_fp2_dn22 = 0.0;
        locals.var_psid_fp2_rv = 0.0;

        locals.var_psim_fp2 = 0.0;
        locals.var_psim_fp2_dn0 = 0.0;
        locals.var_psim_fp2_dn1 = 0.0;
        locals.var_psim_fp2_dn2 = 0.0;
        locals.var_psim_fp2_dn3 = 0.0;
        locals.var_psim_fp2_dn4 = 0.0;
        locals.var_psim_fp2_dn5 = 0.0;
        locals.var_psim_fp2_dn6 = 0.0;
        locals.var_psim_fp2_dn7 = 0.0;
        locals.var_psim_fp2_dn8 = 0.0;
        locals.var_psim_fp2_dn9 = 0.0;
        locals.var_psim_fp2_dn12 = 0.0;
        locals.var_psim_fp2_dn14 = 0.0;
        locals.var_psim_fp2_dn15 = 0.0;
        locals.var_psim_fp2_dn16 = 0.0;
        locals.var_psim_fp2_dn17 = 0.0;
        locals.var_psim_fp2_dn18 = 0.0;
        locals.var_psim_fp2_dn19 = 0.0;
        locals.var_psim_fp2_dn20 = 0.0;
        locals.var_psim_fp2_dn21 = 0.0;
        locals.var_psim_fp2_dn22 = 0.0;
        locals.var_psim_fp2_rv = 0.0;

        locals.var_psisd_fp2 = 0.0;
        locals.var_psisd_fp2_dn0 = 0.0;
        locals.var_psisd_fp2_dn1 = 0.0;
        locals.var_psisd_fp2_dn2 = 0.0;
        locals.var_psisd_fp2_dn3 = 0.0;
        locals.var_psisd_fp2_dn4 = 0.0;
        locals.var_psisd_fp2_dn5 = 0.0;
        locals.var_psisd_fp2_dn6 = 0.0;
        locals.var_psisd_fp2_dn7 = 0.0;
        locals.var_psisd_fp2_dn8 = 0.0;
        locals.var_psisd_fp2_dn9 = 0.0;
        locals.var_psisd_fp2_dn12 = 0.0;
        locals.var_psisd_fp2_dn14 = 0.0;
        locals.var_psisd_fp2_dn15 = 0.0;
        locals.var_psisd_fp2_dn16 = 0.0;
        locals.var_psisd_fp2_dn17 = 0.0;
        locals.var_psisd_fp2_dn18 = 0.0;
        locals.var_psisd_fp2_dn19 = 0.0;
        locals.var_psisd_fp2_dn20 = 0.0;
        locals.var_psisd_fp2_dn21 = 0.0;
        locals.var_psisd_fp2_dn22 = 0.0;
        locals.var_psisd_fp2_rv = 0.0;

        locals.var_qg_fp2 = 0.0;
        locals.var_qg_fp2_dn0 = 0.0;
        locals.var_qg_fp2_dn1 = 0.0;
        locals.var_qg_fp2_dn2 = 0.0;
        locals.var_qg_fp2_dn3 = 0.0;
        locals.var_qg_fp2_dn4 = 0.0;
        locals.var_qg_fp2_dn5 = 0.0;
        locals.var_qg_fp2_dn6 = 0.0;
        locals.var_qg_fp2_dn7 = 0.0;
        locals.var_qg_fp2_dn8 = 0.0;
        locals.var_qg_fp2_dn9 = 0.0;
        locals.var_qg_fp2_dn12 = 0.0;
        locals.var_qg_fp2_dn14 = 0.0;
        locals.var_qg_fp2_dn15 = 0.0;
        locals.var_qg_fp2_dn16 = 0.0;
        locals.var_qg_fp2_dn17 = 0.0;
        locals.var_qg_fp2_dn18 = 0.0;
        locals.var_qg_fp2_dn19 = 0.0;
        locals.var_qg_fp2_dn20 = 0.0;
        locals.var_qg_fp2_dn21 = 0.0;
        locals.var_qg_fp2_dn22 = 0.0;
        locals.var_qg_fp2_rv = 0.0;

        locals.var_qd_fp2 = 0.0;
        locals.var_qd_fp2_dn0 = 0.0;
        locals.var_qd_fp2_dn1 = 0.0;
        locals.var_qd_fp2_dn2 = 0.0;
        locals.var_qd_fp2_dn3 = 0.0;
        locals.var_qd_fp2_dn4 = 0.0;
        locals.var_qd_fp2_dn5 = 0.0;
        locals.var_qd_fp2_dn6 = 0.0;
        locals.var_qd_fp2_dn7 = 0.0;
        locals.var_qd_fp2_dn8 = 0.0;
        locals.var_qd_fp2_dn9 = 0.0;
        locals.var_qd_fp2_dn12 = 0.0;
        locals.var_qd_fp2_dn14 = 0.0;
        locals.var_qd_fp2_dn15 = 0.0;
        locals.var_qd_fp2_dn16 = 0.0;
        locals.var_qd_fp2_dn17 = 0.0;
        locals.var_qd_fp2_dn18 = 0.0;
        locals.var_qd_fp2_dn19 = 0.0;
        locals.var_qd_fp2_dn20 = 0.0;
        locals.var_qd_fp2_dn21 = 0.0;
        locals.var_qd_fp2_dn22 = 0.0;
        locals.var_qd_fp2_rv = 0.0;

        locals.var_vgs_fp2 = 0.0;
        locals.var_vgs_fp2_dn2 = 0.0;
        locals.var_vgs_fp2_dn7 = 0.0;
        locals.var_vgs_fp2_dn9 = 0.0;
        locals.var_vgs_fp2_dn15 = 0.0;
        locals.var_vgs_fp2_dn16 = 0.0;
        locals.var_vgs_fp2_rv = 0.0;

        locals.var_vds_fp2 = 0.0;
        locals.var_vds_fp2_dn15 = 0.0;
        locals.var_vds_fp2_dn16 = 0.0;
        locals.var_vds_fp2_rv = 0.0;

        locals.var_vg0_fp2s = 0.0;
        locals.var_vg0_fp2s_dn0 = 0.0;
        locals.var_vg0_fp2s_dn1 = 0.0;
        locals.var_vg0_fp2s_dn2 = 0.0;
        locals.var_vg0_fp2s_dn3 = 0.0;
        locals.var_vg0_fp2s_dn4 = 0.0;
        locals.var_vg0_fp2s_dn5 = 0.0;
        locals.var_vg0_fp2s_dn6 = 0.0;
        locals.var_vg0_fp2s_dn7 = 0.0;
        locals.var_vg0_fp2s_dn8 = 0.0;
        locals.var_vg0_fp2s_dn9 = 0.0;
        locals.var_vg0_fp2s_dn12 = 0.0;
        locals.var_vg0_fp2s_dn14 = 0.0;
        locals.var_vg0_fp2s_dn15 = 0.0;
        locals.var_vg0_fp2s_dn16 = 0.0;
        locals.var_vg0_fp2s_dn17 = 0.0;
        locals.var_vg0_fp2s_dn18 = 0.0;
        locals.var_vg0_fp2s_dn19 = 0.0;
        locals.var_vg0_fp2s_dn20 = 0.0;
        locals.var_vg0_fp2s_dn21 = 0.0;
        locals.var_vg0_fp2s_dn22 = 0.0;
        locals.var_vg0_fp2s_rv = 0.0;

        locals.var_cg_fp2s = 0.0;
        locals.var_cg_fp2s_rv = 0.0;

        locals.var_psis_fp2s = 0.0;
        locals.var_psis_fp2s_dn0 = 0.0;
        locals.var_psis_fp2s_dn1 = 0.0;
        locals.var_psis_fp2s_dn2 = 0.0;
        locals.var_psis_fp2s_dn3 = 0.0;
        locals.var_psis_fp2s_dn4 = 0.0;
        locals.var_psis_fp2s_dn5 = 0.0;
        locals.var_psis_fp2s_dn6 = 0.0;
        locals.var_psis_fp2s_dn7 = 0.0;
        locals.var_psis_fp2s_dn8 = 0.0;
        locals.var_psis_fp2s_dn9 = 0.0;
        locals.var_psis_fp2s_dn12 = 0.0;
        locals.var_psis_fp2s_dn14 = 0.0;
        locals.var_psis_fp2s_dn15 = 0.0;
        locals.var_psis_fp2s_dn16 = 0.0;
        locals.var_psis_fp2s_dn17 = 0.0;
        locals.var_psis_fp2s_dn18 = 0.0;
        locals.var_psis_fp2s_dn19 = 0.0;
        locals.var_psis_fp2s_dn20 = 0.0;
        locals.var_psis_fp2s_dn21 = 0.0;
        locals.var_psis_fp2s_dn22 = 0.0;
        locals.var_psis_fp2s_rv = 0.0;

        locals.var_psid_fp2s = 0.0;
        locals.var_psid_fp2s_dn0 = 0.0;
        locals.var_psid_fp2s_dn1 = 0.0;
        locals.var_psid_fp2s_dn2 = 0.0;
        locals.var_psid_fp2s_dn3 = 0.0;
        locals.var_psid_fp2s_dn4 = 0.0;
        locals.var_psid_fp2s_dn5 = 0.0;
        locals.var_psid_fp2s_dn6 = 0.0;
        locals.var_psid_fp2s_dn7 = 0.0;
        locals.var_psid_fp2s_dn8 = 0.0;
        locals.var_psid_fp2s_dn9 = 0.0;
        locals.var_psid_fp2s_dn12 = 0.0;
        locals.var_psid_fp2s_dn14 = 0.0;
        locals.var_psid_fp2s_dn15 = 0.0;
        locals.var_psid_fp2s_dn16 = 0.0;
        locals.var_psid_fp2s_dn17 = 0.0;
        locals.var_psid_fp2s_dn18 = 0.0;
        locals.var_psid_fp2s_dn19 = 0.0;
        locals.var_psid_fp2s_dn20 = 0.0;
        locals.var_psid_fp2s_dn21 = 0.0;
        locals.var_psid_fp2s_dn22 = 0.0;
        locals.var_psid_fp2s_rv = 0.0;

        locals.var_psim_fp2s = 0.0;
        locals.var_psim_fp2s_dn0 = 0.0;
        locals.var_psim_fp2s_dn1 = 0.0;
        locals.var_psim_fp2s_dn2 = 0.0;
        locals.var_psim_fp2s_dn3 = 0.0;
        locals.var_psim_fp2s_dn4 = 0.0;
        locals.var_psim_fp2s_dn5 = 0.0;
        locals.var_psim_fp2s_dn6 = 0.0;
        locals.var_psim_fp2s_dn7 = 0.0;
        locals.var_psim_fp2s_dn8 = 0.0;
        locals.var_psim_fp2s_dn9 = 0.0;
        locals.var_psim_fp2s_dn12 = 0.0;
        locals.var_psim_fp2s_dn14 = 0.0;
        locals.var_psim_fp2s_dn15 = 0.0;
        locals.var_psim_fp2s_dn16 = 0.0;
        locals.var_psim_fp2s_dn17 = 0.0;
        locals.var_psim_fp2s_dn18 = 0.0;
        locals.var_psim_fp2s_dn19 = 0.0;
        locals.var_psim_fp2s_dn20 = 0.0;
        locals.var_psim_fp2s_dn21 = 0.0;
        locals.var_psim_fp2s_dn22 = 0.0;
        locals.var_psim_fp2s_rv = 0.0;

        locals.var_psisd_fp2s = 0.0;
        locals.var_psisd_fp2s_dn0 = 0.0;
        locals.var_psisd_fp2s_dn1 = 0.0;
        locals.var_psisd_fp2s_dn2 = 0.0;
        locals.var_psisd_fp2s_dn3 = 0.0;
        locals.var_psisd_fp2s_dn4 = 0.0;
        locals.var_psisd_fp2s_dn5 = 0.0;
        locals.var_psisd_fp2s_dn6 = 0.0;
        locals.var_psisd_fp2s_dn7 = 0.0;
        locals.var_psisd_fp2s_dn8 = 0.0;
        locals.var_psisd_fp2s_dn9 = 0.0;
        locals.var_psisd_fp2s_dn12 = 0.0;
        locals.var_psisd_fp2s_dn14 = 0.0;
        locals.var_psisd_fp2s_dn15 = 0.0;
        locals.var_psisd_fp2s_dn16 = 0.0;
        locals.var_psisd_fp2s_dn17 = 0.0;
        locals.var_psisd_fp2s_dn18 = 0.0;
        locals.var_psisd_fp2s_dn19 = 0.0;
        locals.var_psisd_fp2s_dn20 = 0.0;
        locals.var_psisd_fp2s_dn21 = 0.0;
        locals.var_psisd_fp2s_dn22 = 0.0;
        locals.var_psisd_fp2s_rv = 0.0;

        locals.var_qg_fp2s = 0.0;
        locals.var_qg_fp2s_dn0 = 0.0;
        locals.var_qg_fp2s_dn1 = 0.0;
        locals.var_qg_fp2s_dn2 = 0.0;
        locals.var_qg_fp2s_dn3 = 0.0;
        locals.var_qg_fp2s_dn4 = 0.0;
        locals.var_qg_fp2s_dn5 = 0.0;
        locals.var_qg_fp2s_dn6 = 0.0;
        locals.var_qg_fp2s_dn7 = 0.0;
        locals.var_qg_fp2s_dn8 = 0.0;
        locals.var_qg_fp2s_dn9 = 0.0;
        locals.var_qg_fp2s_dn12 = 0.0;
        locals.var_qg_fp2s_dn14 = 0.0;
        locals.var_qg_fp2s_dn15 = 0.0;
        locals.var_qg_fp2s_dn16 = 0.0;
        locals.var_qg_fp2s_dn17 = 0.0;
        locals.var_qg_fp2s_dn18 = 0.0;
        locals.var_qg_fp2s_dn19 = 0.0;
        locals.var_qg_fp2s_dn20 = 0.0;
        locals.var_qg_fp2s_dn21 = 0.0;
        locals.var_qg_fp2s_dn22 = 0.0;
        locals.var_qg_fp2s_rv = 0.0;

        locals.var_qd_fp2s = 0.0;
        locals.var_qd_fp2s_dn0 = 0.0;
        locals.var_qd_fp2s_dn1 = 0.0;
        locals.var_qd_fp2s_dn2 = 0.0;
        locals.var_qd_fp2s_dn3 = 0.0;
        locals.var_qd_fp2s_dn4 = 0.0;
        locals.var_qd_fp2s_dn5 = 0.0;
        locals.var_qd_fp2s_dn6 = 0.0;
        locals.var_qd_fp2s_dn7 = 0.0;
        locals.var_qd_fp2s_dn8 = 0.0;
        locals.var_qd_fp2s_dn9 = 0.0;
        locals.var_qd_fp2s_dn12 = 0.0;
        locals.var_qd_fp2s_dn14 = 0.0;
        locals.var_qd_fp2s_dn15 = 0.0;
        locals.var_qd_fp2s_dn16 = 0.0;
        locals.var_qd_fp2s_dn17 = 0.0;
        locals.var_qd_fp2s_dn18 = 0.0;
        locals.var_qd_fp2s_dn19 = 0.0;
        locals.var_qd_fp2s_dn20 = 0.0;
        locals.var_qd_fp2s_dn21 = 0.0;
        locals.var_qd_fp2s_dn22 = 0.0;
        locals.var_qd_fp2s_rv = 0.0;

        locals.var_vgs_fp2s = 0.0;
        locals.var_vgs_fp2s_dn2 = 0.0;
        locals.var_vgs_fp2s_dn8 = 0.0;
        locals.var_vgs_fp2s_dn9 = 0.0;
        locals.var_vgs_fp2s_dn19 = 0.0;
        locals.var_vgs_fp2s_dn20 = 0.0;
        locals.var_vgs_fp2s_rv = 0.0;

        locals.var_vds_fp2s = 0.0;
        locals.var_vds_fp2s_dn19 = 0.0;
        locals.var_vds_fp2s_dn20 = 0.0;
        locals.var_vds_fp2s_rv = 0.0;

        locals.var_vg0_fp3 = 0.0;
        locals.var_vg0_fp3_dn0 = 0.0;
        locals.var_vg0_fp3_dn1 = 0.0;
        locals.var_vg0_fp3_dn2 = 0.0;
        locals.var_vg0_fp3_dn3 = 0.0;
        locals.var_vg0_fp3_dn4 = 0.0;
        locals.var_vg0_fp3_dn5 = 0.0;
        locals.var_vg0_fp3_dn6 = 0.0;
        locals.var_vg0_fp3_dn7 = 0.0;
        locals.var_vg0_fp3_dn8 = 0.0;
        locals.var_vg0_fp3_dn9 = 0.0;
        locals.var_vg0_fp3_dn12 = 0.0;
        locals.var_vg0_fp3_dn14 = 0.0;
        locals.var_vg0_fp3_dn15 = 0.0;
        locals.var_vg0_fp3_dn16 = 0.0;
        locals.var_vg0_fp3_dn17 = 0.0;
        locals.var_vg0_fp3_dn18 = 0.0;
        locals.var_vg0_fp3_dn19 = 0.0;
        locals.var_vg0_fp3_dn20 = 0.0;
        locals.var_vg0_fp3_dn21 = 0.0;
        locals.var_vg0_fp3_dn22 = 0.0;
        locals.var_vg0_fp3_rv = 0.0;

        locals.var_cg_fp3 = 0.0;
        locals.var_cg_fp3_rv = 0.0;

        locals.var_psis_fp3 = 0.0;
        locals.var_psis_fp3_dn0 = 0.0;
        locals.var_psis_fp3_dn1 = 0.0;
        locals.var_psis_fp3_dn2 = 0.0;
        locals.var_psis_fp3_dn3 = 0.0;
        locals.var_psis_fp3_dn4 = 0.0;
        locals.var_psis_fp3_dn5 = 0.0;
        locals.var_psis_fp3_dn6 = 0.0;
        locals.var_psis_fp3_dn7 = 0.0;
        locals.var_psis_fp3_dn8 = 0.0;
        locals.var_psis_fp3_dn9 = 0.0;
        locals.var_psis_fp3_dn12 = 0.0;
        locals.var_psis_fp3_dn14 = 0.0;
        locals.var_psis_fp3_dn15 = 0.0;
        locals.var_psis_fp3_dn16 = 0.0;
        locals.var_psis_fp3_dn17 = 0.0;
        locals.var_psis_fp3_dn18 = 0.0;
        locals.var_psis_fp3_dn19 = 0.0;
        locals.var_psis_fp3_dn20 = 0.0;
        locals.var_psis_fp3_dn21 = 0.0;
        locals.var_psis_fp3_dn22 = 0.0;
        locals.var_psis_fp3_rv = 0.0;

        locals.var_psid_fp3 = 0.0;
        locals.var_psid_fp3_dn0 = 0.0;
        locals.var_psid_fp3_dn1 = 0.0;
        locals.var_psid_fp3_dn2 = 0.0;
        locals.var_psid_fp3_dn3 = 0.0;
        locals.var_psid_fp3_dn4 = 0.0;
        locals.var_psid_fp3_dn5 = 0.0;
        locals.var_psid_fp3_dn6 = 0.0;
        locals.var_psid_fp3_dn7 = 0.0;
        locals.var_psid_fp3_dn8 = 0.0;
        locals.var_psid_fp3_dn9 = 0.0;
        locals.var_psid_fp3_dn12 = 0.0;
        locals.var_psid_fp3_dn14 = 0.0;
        locals.var_psid_fp3_dn15 = 0.0;
        locals.var_psid_fp3_dn16 = 0.0;
        locals.var_psid_fp3_dn17 = 0.0;
        locals.var_psid_fp3_dn18 = 0.0;
        locals.var_psid_fp3_dn19 = 0.0;
        locals.var_psid_fp3_dn20 = 0.0;
        locals.var_psid_fp3_dn21 = 0.0;
        locals.var_psid_fp3_dn22 = 0.0;
        locals.var_psid_fp3_rv = 0.0;

        locals.var_psim_fp3 = 0.0;
        locals.var_psim_fp3_dn0 = 0.0;
        locals.var_psim_fp3_dn1 = 0.0;
        locals.var_psim_fp3_dn2 = 0.0;
        locals.var_psim_fp3_dn3 = 0.0;
        locals.var_psim_fp3_dn4 = 0.0;
        locals.var_psim_fp3_dn5 = 0.0;
        locals.var_psim_fp3_dn6 = 0.0;
        locals.var_psim_fp3_dn7 = 0.0;
        locals.var_psim_fp3_dn8 = 0.0;
        locals.var_psim_fp3_dn9 = 0.0;
        locals.var_psim_fp3_dn12 = 0.0;
        locals.var_psim_fp3_dn14 = 0.0;
        locals.var_psim_fp3_dn15 = 0.0;
        locals.var_psim_fp3_dn16 = 0.0;
        locals.var_psim_fp3_dn17 = 0.0;
        locals.var_psim_fp3_dn18 = 0.0;
        locals.var_psim_fp3_dn19 = 0.0;
        locals.var_psim_fp3_dn20 = 0.0;
        locals.var_psim_fp3_dn21 = 0.0;
        locals.var_psim_fp3_dn22 = 0.0;
        locals.var_psim_fp3_rv = 0.0;

        locals.var_psisd_fp3 = 0.0;
        locals.var_psisd_fp3_dn0 = 0.0;
        locals.var_psisd_fp3_dn1 = 0.0;
        locals.var_psisd_fp3_dn2 = 0.0;
        locals.var_psisd_fp3_dn3 = 0.0;
        locals.var_psisd_fp3_dn4 = 0.0;
        locals.var_psisd_fp3_dn5 = 0.0;
        locals.var_psisd_fp3_dn6 = 0.0;
        locals.var_psisd_fp3_dn7 = 0.0;
        locals.var_psisd_fp3_dn8 = 0.0;
        locals.var_psisd_fp3_dn9 = 0.0;
        locals.var_psisd_fp3_dn12 = 0.0;
        locals.var_psisd_fp3_dn14 = 0.0;
        locals.var_psisd_fp3_dn15 = 0.0;
        locals.var_psisd_fp3_dn16 = 0.0;
        locals.var_psisd_fp3_dn17 = 0.0;
        locals.var_psisd_fp3_dn18 = 0.0;
        locals.var_psisd_fp3_dn19 = 0.0;
        locals.var_psisd_fp3_dn20 = 0.0;
        locals.var_psisd_fp3_dn21 = 0.0;
        locals.var_psisd_fp3_dn22 = 0.0;
        locals.var_psisd_fp3_rv = 0.0;

        locals.var_qg_fp3 = 0.0;
        locals.var_qg_fp3_dn0 = 0.0;
        locals.var_qg_fp3_dn1 = 0.0;
        locals.var_qg_fp3_dn2 = 0.0;
        locals.var_qg_fp3_dn3 = 0.0;
        locals.var_qg_fp3_dn4 = 0.0;
        locals.var_qg_fp3_dn5 = 0.0;
        locals.var_qg_fp3_dn6 = 0.0;
        locals.var_qg_fp3_dn7 = 0.0;
        locals.var_qg_fp3_dn8 = 0.0;
        locals.var_qg_fp3_dn9 = 0.0;
        locals.var_qg_fp3_dn12 = 0.0;
        locals.var_qg_fp3_dn14 = 0.0;
        locals.var_qg_fp3_dn15 = 0.0;
        locals.var_qg_fp3_dn16 = 0.0;
        locals.var_qg_fp3_dn17 = 0.0;
        locals.var_qg_fp3_dn18 = 0.0;
        locals.var_qg_fp3_dn19 = 0.0;
        locals.var_qg_fp3_dn20 = 0.0;
        locals.var_qg_fp3_dn21 = 0.0;
        locals.var_qg_fp3_dn22 = 0.0;
        locals.var_qg_fp3_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_2(
        locals: &mut StampLocals,
    ) {
        locals.var_qd_fp3 = 0.0;
        locals.var_qd_fp3_dn0 = 0.0;
        locals.var_qd_fp3_dn1 = 0.0;
        locals.var_qd_fp3_dn2 = 0.0;
        locals.var_qd_fp3_dn3 = 0.0;
        locals.var_qd_fp3_dn4 = 0.0;
        locals.var_qd_fp3_dn5 = 0.0;
        locals.var_qd_fp3_dn6 = 0.0;
        locals.var_qd_fp3_dn7 = 0.0;
        locals.var_qd_fp3_dn8 = 0.0;
        locals.var_qd_fp3_dn9 = 0.0;
        locals.var_qd_fp3_dn12 = 0.0;
        locals.var_qd_fp3_dn14 = 0.0;
        locals.var_qd_fp3_dn15 = 0.0;
        locals.var_qd_fp3_dn16 = 0.0;
        locals.var_qd_fp3_dn17 = 0.0;
        locals.var_qd_fp3_dn18 = 0.0;
        locals.var_qd_fp3_dn19 = 0.0;
        locals.var_qd_fp3_dn20 = 0.0;
        locals.var_qd_fp3_dn21 = 0.0;
        locals.var_qd_fp3_dn22 = 0.0;
        locals.var_qd_fp3_rv = 0.0;

        locals.var_vgs_fp3 = 0.0;
        locals.var_vgs_fp3_dn2 = 0.0;
        locals.var_vgs_fp3_dn7 = 0.0;
        locals.var_vgs_fp3_dn9 = 0.0;
        locals.var_vgs_fp3_dn16 = 0.0;
        locals.var_vgs_fp3_dn17 = 0.0;
        locals.var_vgs_fp3_rv = 0.0;

        locals.var_vds_fp3 = 0.0;
        locals.var_vds_fp3_dn16 = 0.0;
        locals.var_vds_fp3_dn17 = 0.0;
        locals.var_vds_fp3_rv = 0.0;

        locals.var_vg0_fp3s = 0.0;
        locals.var_vg0_fp3s_dn0 = 0.0;
        locals.var_vg0_fp3s_dn1 = 0.0;
        locals.var_vg0_fp3s_dn2 = 0.0;
        locals.var_vg0_fp3s_dn3 = 0.0;
        locals.var_vg0_fp3s_dn4 = 0.0;
        locals.var_vg0_fp3s_dn5 = 0.0;
        locals.var_vg0_fp3s_dn6 = 0.0;
        locals.var_vg0_fp3s_dn7 = 0.0;
        locals.var_vg0_fp3s_dn8 = 0.0;
        locals.var_vg0_fp3s_dn9 = 0.0;
        locals.var_vg0_fp3s_dn12 = 0.0;
        locals.var_vg0_fp3s_dn14 = 0.0;
        locals.var_vg0_fp3s_dn15 = 0.0;
        locals.var_vg0_fp3s_dn16 = 0.0;
        locals.var_vg0_fp3s_dn17 = 0.0;
        locals.var_vg0_fp3s_dn18 = 0.0;
        locals.var_vg0_fp3s_dn19 = 0.0;
        locals.var_vg0_fp3s_dn20 = 0.0;
        locals.var_vg0_fp3s_dn21 = 0.0;
        locals.var_vg0_fp3s_dn22 = 0.0;
        locals.var_vg0_fp3s_rv = 0.0;

        locals.var_cg_fp3s = 0.0;
        locals.var_cg_fp3s_rv = 0.0;

        locals.var_psis_fp3s = 0.0;
        locals.var_psis_fp3s_dn0 = 0.0;
        locals.var_psis_fp3s_dn1 = 0.0;
        locals.var_psis_fp3s_dn2 = 0.0;
        locals.var_psis_fp3s_dn3 = 0.0;
        locals.var_psis_fp3s_dn4 = 0.0;
        locals.var_psis_fp3s_dn5 = 0.0;
        locals.var_psis_fp3s_dn6 = 0.0;
        locals.var_psis_fp3s_dn7 = 0.0;
        locals.var_psis_fp3s_dn8 = 0.0;
        locals.var_psis_fp3s_dn9 = 0.0;
        locals.var_psis_fp3s_dn12 = 0.0;
        locals.var_psis_fp3s_dn14 = 0.0;
        locals.var_psis_fp3s_dn15 = 0.0;
        locals.var_psis_fp3s_dn16 = 0.0;
        locals.var_psis_fp3s_dn17 = 0.0;
        locals.var_psis_fp3s_dn18 = 0.0;
        locals.var_psis_fp3s_dn19 = 0.0;
        locals.var_psis_fp3s_dn20 = 0.0;
        locals.var_psis_fp3s_dn21 = 0.0;
        locals.var_psis_fp3s_dn22 = 0.0;
        locals.var_psis_fp3s_rv = 0.0;

        locals.var_psid_fp3s = 0.0;
        locals.var_psid_fp3s_dn0 = 0.0;
        locals.var_psid_fp3s_dn1 = 0.0;
        locals.var_psid_fp3s_dn2 = 0.0;
        locals.var_psid_fp3s_dn3 = 0.0;
        locals.var_psid_fp3s_dn4 = 0.0;
        locals.var_psid_fp3s_dn5 = 0.0;
        locals.var_psid_fp3s_dn6 = 0.0;
        locals.var_psid_fp3s_dn7 = 0.0;
        locals.var_psid_fp3s_dn8 = 0.0;
        locals.var_psid_fp3s_dn9 = 0.0;
        locals.var_psid_fp3s_dn12 = 0.0;
        locals.var_psid_fp3s_dn14 = 0.0;
        locals.var_psid_fp3s_dn15 = 0.0;
        locals.var_psid_fp3s_dn16 = 0.0;
        locals.var_psid_fp3s_dn17 = 0.0;
        locals.var_psid_fp3s_dn18 = 0.0;
        locals.var_psid_fp3s_dn19 = 0.0;
        locals.var_psid_fp3s_dn20 = 0.0;
        locals.var_psid_fp3s_dn21 = 0.0;
        locals.var_psid_fp3s_dn22 = 0.0;
        locals.var_psid_fp3s_rv = 0.0;

        locals.var_psim_fp3s = 0.0;
        locals.var_psim_fp3s_dn0 = 0.0;
        locals.var_psim_fp3s_dn1 = 0.0;
        locals.var_psim_fp3s_dn2 = 0.0;
        locals.var_psim_fp3s_dn3 = 0.0;
        locals.var_psim_fp3s_dn4 = 0.0;
        locals.var_psim_fp3s_dn5 = 0.0;
        locals.var_psim_fp3s_dn6 = 0.0;
        locals.var_psim_fp3s_dn7 = 0.0;
        locals.var_psim_fp3s_dn8 = 0.0;
        locals.var_psim_fp3s_dn9 = 0.0;
        locals.var_psim_fp3s_dn12 = 0.0;
        locals.var_psim_fp3s_dn14 = 0.0;
        locals.var_psim_fp3s_dn15 = 0.0;
        locals.var_psim_fp3s_dn16 = 0.0;
        locals.var_psim_fp3s_dn17 = 0.0;
        locals.var_psim_fp3s_dn18 = 0.0;
        locals.var_psim_fp3s_dn19 = 0.0;
        locals.var_psim_fp3s_dn20 = 0.0;
        locals.var_psim_fp3s_dn21 = 0.0;
        locals.var_psim_fp3s_dn22 = 0.0;
        locals.var_psim_fp3s_rv = 0.0;

        locals.var_psisd_fp3s = 0.0;
        locals.var_psisd_fp3s_dn0 = 0.0;
        locals.var_psisd_fp3s_dn1 = 0.0;
        locals.var_psisd_fp3s_dn2 = 0.0;
        locals.var_psisd_fp3s_dn3 = 0.0;
        locals.var_psisd_fp3s_dn4 = 0.0;
        locals.var_psisd_fp3s_dn5 = 0.0;
        locals.var_psisd_fp3s_dn6 = 0.0;
        locals.var_psisd_fp3s_dn7 = 0.0;
        locals.var_psisd_fp3s_dn8 = 0.0;
        locals.var_psisd_fp3s_dn9 = 0.0;
        locals.var_psisd_fp3s_dn12 = 0.0;
        locals.var_psisd_fp3s_dn14 = 0.0;
        locals.var_psisd_fp3s_dn15 = 0.0;
        locals.var_psisd_fp3s_dn16 = 0.0;
        locals.var_psisd_fp3s_dn17 = 0.0;
        locals.var_psisd_fp3s_dn18 = 0.0;
        locals.var_psisd_fp3s_dn19 = 0.0;
        locals.var_psisd_fp3s_dn20 = 0.0;
        locals.var_psisd_fp3s_dn21 = 0.0;
        locals.var_psisd_fp3s_dn22 = 0.0;
        locals.var_psisd_fp3s_rv = 0.0;

        locals.var_qg_fp3s = 0.0;
        locals.var_qg_fp3s_dn0 = 0.0;
        locals.var_qg_fp3s_dn1 = 0.0;
        locals.var_qg_fp3s_dn2 = 0.0;
        locals.var_qg_fp3s_dn3 = 0.0;
        locals.var_qg_fp3s_dn4 = 0.0;
        locals.var_qg_fp3s_dn5 = 0.0;
        locals.var_qg_fp3s_dn6 = 0.0;
        locals.var_qg_fp3s_dn7 = 0.0;
        locals.var_qg_fp3s_dn8 = 0.0;
        locals.var_qg_fp3s_dn9 = 0.0;
        locals.var_qg_fp3s_dn12 = 0.0;
        locals.var_qg_fp3s_dn14 = 0.0;
        locals.var_qg_fp3s_dn15 = 0.0;
        locals.var_qg_fp3s_dn16 = 0.0;
        locals.var_qg_fp3s_dn17 = 0.0;
        locals.var_qg_fp3s_dn18 = 0.0;
        locals.var_qg_fp3s_dn19 = 0.0;
        locals.var_qg_fp3s_dn20 = 0.0;
        locals.var_qg_fp3s_dn21 = 0.0;
        locals.var_qg_fp3s_dn22 = 0.0;
        locals.var_qg_fp3s_rv = 0.0;

        locals.var_qd_fp3s = 0.0;
        locals.var_qd_fp3s_dn0 = 0.0;
        locals.var_qd_fp3s_dn1 = 0.0;
        locals.var_qd_fp3s_dn2 = 0.0;
        locals.var_qd_fp3s_dn3 = 0.0;
        locals.var_qd_fp3s_dn4 = 0.0;
        locals.var_qd_fp3s_dn5 = 0.0;
        locals.var_qd_fp3s_dn6 = 0.0;
        locals.var_qd_fp3s_dn7 = 0.0;
        locals.var_qd_fp3s_dn8 = 0.0;
        locals.var_qd_fp3s_dn9 = 0.0;
        locals.var_qd_fp3s_dn12 = 0.0;
        locals.var_qd_fp3s_dn14 = 0.0;
        locals.var_qd_fp3s_dn15 = 0.0;
        locals.var_qd_fp3s_dn16 = 0.0;
        locals.var_qd_fp3s_dn17 = 0.0;
        locals.var_qd_fp3s_dn18 = 0.0;
        locals.var_qd_fp3s_dn19 = 0.0;
        locals.var_qd_fp3s_dn20 = 0.0;
        locals.var_qd_fp3s_dn21 = 0.0;
        locals.var_qd_fp3s_dn22 = 0.0;
        locals.var_qd_fp3s_rv = 0.0;

        locals.var_vgs_fp3s = 0.0;
        locals.var_vgs_fp3s_dn2 = 0.0;
        locals.var_vgs_fp3s_dn8 = 0.0;
        locals.var_vgs_fp3s_dn9 = 0.0;
        locals.var_vgs_fp3s_dn20 = 0.0;
        locals.var_vgs_fp3s_dn21 = 0.0;
        locals.var_vgs_fp3s_rv = 0.0;

        locals.var_vds_fp3s = 0.0;
        locals.var_vds_fp3s_dn20 = 0.0;
        locals.var_vds_fp3s_dn21 = 0.0;
        locals.var_vds_fp3s_rv = 0.0;

        locals.var_vg0_fp4 = 0.0;
        locals.var_vg0_fp4_dn0 = 0.0;
        locals.var_vg0_fp4_dn1 = 0.0;
        locals.var_vg0_fp4_dn2 = 0.0;
        locals.var_vg0_fp4_dn3 = 0.0;
        locals.var_vg0_fp4_dn4 = 0.0;
        locals.var_vg0_fp4_dn5 = 0.0;
        locals.var_vg0_fp4_dn6 = 0.0;
        locals.var_vg0_fp4_dn7 = 0.0;
        locals.var_vg0_fp4_dn8 = 0.0;
        locals.var_vg0_fp4_dn9 = 0.0;
        locals.var_vg0_fp4_dn12 = 0.0;
        locals.var_vg0_fp4_dn14 = 0.0;
        locals.var_vg0_fp4_dn15 = 0.0;
        locals.var_vg0_fp4_dn16 = 0.0;
        locals.var_vg0_fp4_dn17 = 0.0;
        locals.var_vg0_fp4_dn18 = 0.0;
        locals.var_vg0_fp4_dn19 = 0.0;
        locals.var_vg0_fp4_dn20 = 0.0;
        locals.var_vg0_fp4_dn21 = 0.0;
        locals.var_vg0_fp4_dn22 = 0.0;
        locals.var_vg0_fp4_rv = 0.0;

        locals.var_cg_fp4 = 0.0;
        locals.var_cg_fp4_rv = 0.0;

        locals.var_psis_fp4 = 0.0;
        locals.var_psis_fp4_dn0 = 0.0;
        locals.var_psis_fp4_dn1 = 0.0;
        locals.var_psis_fp4_dn2 = 0.0;
        locals.var_psis_fp4_dn3 = 0.0;
        locals.var_psis_fp4_dn4 = 0.0;
        locals.var_psis_fp4_dn5 = 0.0;
        locals.var_psis_fp4_dn6 = 0.0;
        locals.var_psis_fp4_dn7 = 0.0;
        locals.var_psis_fp4_dn8 = 0.0;
        locals.var_psis_fp4_dn9 = 0.0;
        locals.var_psis_fp4_dn12 = 0.0;
        locals.var_psis_fp4_dn14 = 0.0;
        locals.var_psis_fp4_dn15 = 0.0;
        locals.var_psis_fp4_dn16 = 0.0;
        locals.var_psis_fp4_dn17 = 0.0;
        locals.var_psis_fp4_dn18 = 0.0;
        locals.var_psis_fp4_dn19 = 0.0;
        locals.var_psis_fp4_dn20 = 0.0;
        locals.var_psis_fp4_dn21 = 0.0;
        locals.var_psis_fp4_dn22 = 0.0;
        locals.var_psis_fp4_rv = 0.0;

        locals.var_psid_fp4 = 0.0;
        locals.var_psid_fp4_dn0 = 0.0;
        locals.var_psid_fp4_dn1 = 0.0;
        locals.var_psid_fp4_dn2 = 0.0;
        locals.var_psid_fp4_dn3 = 0.0;
        locals.var_psid_fp4_dn4 = 0.0;
        locals.var_psid_fp4_dn5 = 0.0;
        locals.var_psid_fp4_dn6 = 0.0;
        locals.var_psid_fp4_dn7 = 0.0;
        locals.var_psid_fp4_dn8 = 0.0;
        locals.var_psid_fp4_dn9 = 0.0;
        locals.var_psid_fp4_dn12 = 0.0;
        locals.var_psid_fp4_dn14 = 0.0;
        locals.var_psid_fp4_dn15 = 0.0;
        locals.var_psid_fp4_dn16 = 0.0;
        locals.var_psid_fp4_dn17 = 0.0;
        locals.var_psid_fp4_dn18 = 0.0;
        locals.var_psid_fp4_dn19 = 0.0;
        locals.var_psid_fp4_dn20 = 0.0;
        locals.var_psid_fp4_dn21 = 0.0;
        locals.var_psid_fp4_dn22 = 0.0;
        locals.var_psid_fp4_rv = 0.0;

        locals.var_psim_fp4 = 0.0;
        locals.var_psim_fp4_dn0 = 0.0;
        locals.var_psim_fp4_dn1 = 0.0;
        locals.var_psim_fp4_dn2 = 0.0;
        locals.var_psim_fp4_dn3 = 0.0;
        locals.var_psim_fp4_dn4 = 0.0;
        locals.var_psim_fp4_dn5 = 0.0;
        locals.var_psim_fp4_dn6 = 0.0;
        locals.var_psim_fp4_dn7 = 0.0;
        locals.var_psim_fp4_dn8 = 0.0;
        locals.var_psim_fp4_dn9 = 0.0;
        locals.var_psim_fp4_dn12 = 0.0;
        locals.var_psim_fp4_dn14 = 0.0;
        locals.var_psim_fp4_dn15 = 0.0;
        locals.var_psim_fp4_dn16 = 0.0;
        locals.var_psim_fp4_dn17 = 0.0;
        locals.var_psim_fp4_dn18 = 0.0;
        locals.var_psim_fp4_dn19 = 0.0;
        locals.var_psim_fp4_dn20 = 0.0;
        locals.var_psim_fp4_dn21 = 0.0;
        locals.var_psim_fp4_dn22 = 0.0;
        locals.var_psim_fp4_rv = 0.0;

        locals.var_psisd_fp4 = 0.0;
        locals.var_psisd_fp4_dn0 = 0.0;
        locals.var_psisd_fp4_dn1 = 0.0;
        locals.var_psisd_fp4_dn2 = 0.0;
        locals.var_psisd_fp4_dn3 = 0.0;
        locals.var_psisd_fp4_dn4 = 0.0;
        locals.var_psisd_fp4_dn5 = 0.0;
        locals.var_psisd_fp4_dn6 = 0.0;
        locals.var_psisd_fp4_dn7 = 0.0;
        locals.var_psisd_fp4_dn8 = 0.0;
        locals.var_psisd_fp4_dn9 = 0.0;
        locals.var_psisd_fp4_dn12 = 0.0;
        locals.var_psisd_fp4_dn14 = 0.0;
        locals.var_psisd_fp4_dn15 = 0.0;
        locals.var_psisd_fp4_dn16 = 0.0;
        locals.var_psisd_fp4_dn17 = 0.0;
        locals.var_psisd_fp4_dn18 = 0.0;
        locals.var_psisd_fp4_dn19 = 0.0;
        locals.var_psisd_fp4_dn20 = 0.0;
        locals.var_psisd_fp4_dn21 = 0.0;
        locals.var_psisd_fp4_dn22 = 0.0;
        locals.var_psisd_fp4_rv = 0.0;

        locals.var_qg_fp4 = 0.0;
        locals.var_qg_fp4_dn0 = 0.0;
        locals.var_qg_fp4_dn1 = 0.0;
        locals.var_qg_fp4_dn2 = 0.0;
        locals.var_qg_fp4_dn3 = 0.0;
        locals.var_qg_fp4_dn4 = 0.0;
        locals.var_qg_fp4_dn5 = 0.0;
        locals.var_qg_fp4_dn6 = 0.0;
        locals.var_qg_fp4_dn7 = 0.0;
        locals.var_qg_fp4_dn8 = 0.0;
        locals.var_qg_fp4_dn9 = 0.0;
        locals.var_qg_fp4_dn12 = 0.0;
        locals.var_qg_fp4_dn14 = 0.0;
        locals.var_qg_fp4_dn15 = 0.0;
        locals.var_qg_fp4_dn16 = 0.0;
        locals.var_qg_fp4_dn17 = 0.0;
        locals.var_qg_fp4_dn18 = 0.0;
        locals.var_qg_fp4_dn19 = 0.0;
        locals.var_qg_fp4_dn20 = 0.0;
        locals.var_qg_fp4_dn21 = 0.0;
        locals.var_qg_fp4_dn22 = 0.0;
        locals.var_qg_fp4_rv = 0.0;

        locals.var_qd_fp4 = 0.0;
        locals.var_qd_fp4_dn0 = 0.0;
        locals.var_qd_fp4_dn1 = 0.0;
        locals.var_qd_fp4_dn2 = 0.0;
        locals.var_qd_fp4_dn3 = 0.0;
        locals.var_qd_fp4_dn4 = 0.0;
        locals.var_qd_fp4_dn5 = 0.0;
        locals.var_qd_fp4_dn6 = 0.0;
        locals.var_qd_fp4_dn7 = 0.0;
        locals.var_qd_fp4_dn8 = 0.0;
        locals.var_qd_fp4_dn9 = 0.0;
        locals.var_qd_fp4_dn12 = 0.0;
        locals.var_qd_fp4_dn14 = 0.0;
        locals.var_qd_fp4_dn15 = 0.0;
        locals.var_qd_fp4_dn16 = 0.0;
        locals.var_qd_fp4_dn17 = 0.0;
        locals.var_qd_fp4_dn18 = 0.0;
        locals.var_qd_fp4_dn19 = 0.0;
        locals.var_qd_fp4_dn20 = 0.0;
        locals.var_qd_fp4_dn21 = 0.0;
        locals.var_qd_fp4_dn22 = 0.0;
        locals.var_qd_fp4_rv = 0.0;

        locals.var_vgs_fp4 = 0.0;
        locals.var_vgs_fp4_dn2 = 0.0;
        locals.var_vgs_fp4_dn7 = 0.0;
        locals.var_vgs_fp4_dn9 = 0.0;
        locals.var_vgs_fp4_dn17 = 0.0;
        locals.var_vgs_fp4_dn18 = 0.0;
        locals.var_vgs_fp4_rv = 0.0;

        locals.var_vds_fp4 = 0.0;
        locals.var_vds_fp4_dn17 = 0.0;
        locals.var_vds_fp4_dn18 = 0.0;
        locals.var_vds_fp4_rv = 0.0;

        locals.var_vg0_fp4s = 0.0;
        locals.var_vg0_fp4s_dn0 = 0.0;
        locals.var_vg0_fp4s_dn1 = 0.0;
        locals.var_vg0_fp4s_dn2 = 0.0;
        locals.var_vg0_fp4s_dn3 = 0.0;
        locals.var_vg0_fp4s_dn4 = 0.0;
        locals.var_vg0_fp4s_dn5 = 0.0;
        locals.var_vg0_fp4s_dn6 = 0.0;
        locals.var_vg0_fp4s_dn7 = 0.0;
        locals.var_vg0_fp4s_dn8 = 0.0;
        locals.var_vg0_fp4s_dn9 = 0.0;
        locals.var_vg0_fp4s_dn12 = 0.0;
        locals.var_vg0_fp4s_dn14 = 0.0;
        locals.var_vg0_fp4s_dn15 = 0.0;
        locals.var_vg0_fp4s_dn16 = 0.0;
        locals.var_vg0_fp4s_dn17 = 0.0;
        locals.var_vg0_fp4s_dn18 = 0.0;
        locals.var_vg0_fp4s_dn19 = 0.0;
        locals.var_vg0_fp4s_dn20 = 0.0;
        locals.var_vg0_fp4s_dn21 = 0.0;
        locals.var_vg0_fp4s_dn22 = 0.0;
        locals.var_vg0_fp4s_rv = 0.0;

        locals.var_cg_fp4s = 0.0;
        locals.var_cg_fp4s_rv = 0.0;

        locals.var_psis_fp4s = 0.0;
        locals.var_psis_fp4s_dn0 = 0.0;
        locals.var_psis_fp4s_dn1 = 0.0;
        locals.var_psis_fp4s_dn2 = 0.0;
        locals.var_psis_fp4s_dn3 = 0.0;
        locals.var_psis_fp4s_dn4 = 0.0;
        locals.var_psis_fp4s_dn5 = 0.0;
        locals.var_psis_fp4s_dn6 = 0.0;
        locals.var_psis_fp4s_dn7 = 0.0;
        locals.var_psis_fp4s_dn8 = 0.0;
        locals.var_psis_fp4s_dn9 = 0.0;
        locals.var_psis_fp4s_dn12 = 0.0;
        locals.var_psis_fp4s_dn14 = 0.0;
        locals.var_psis_fp4s_dn15 = 0.0;
        locals.var_psis_fp4s_dn16 = 0.0;
        locals.var_psis_fp4s_dn17 = 0.0;
        locals.var_psis_fp4s_dn18 = 0.0;
        locals.var_psis_fp4s_dn19 = 0.0;
        locals.var_psis_fp4s_dn20 = 0.0;
        locals.var_psis_fp4s_dn21 = 0.0;
        locals.var_psis_fp4s_dn22 = 0.0;
        locals.var_psis_fp4s_rv = 0.0;

        locals.var_psid_fp4s = 0.0;
        locals.var_psid_fp4s_dn0 = 0.0;
        locals.var_psid_fp4s_dn1 = 0.0;
        locals.var_psid_fp4s_dn2 = 0.0;
        locals.var_psid_fp4s_dn3 = 0.0;
        locals.var_psid_fp4s_dn4 = 0.0;
        locals.var_psid_fp4s_dn5 = 0.0;
        locals.var_psid_fp4s_dn6 = 0.0;
        locals.var_psid_fp4s_dn7 = 0.0;
        locals.var_psid_fp4s_dn8 = 0.0;
        locals.var_psid_fp4s_dn9 = 0.0;
        locals.var_psid_fp4s_dn12 = 0.0;
        locals.var_psid_fp4s_dn14 = 0.0;
        locals.var_psid_fp4s_dn15 = 0.0;
        locals.var_psid_fp4s_dn16 = 0.0;
        locals.var_psid_fp4s_dn17 = 0.0;
        locals.var_psid_fp4s_dn18 = 0.0;
        locals.var_psid_fp4s_dn19 = 0.0;
        locals.var_psid_fp4s_dn20 = 0.0;
        locals.var_psid_fp4s_dn21 = 0.0;
        locals.var_psid_fp4s_dn22 = 0.0;
        locals.var_psid_fp4s_rv = 0.0;

        locals.var_psim_fp4s = 0.0;
        locals.var_psim_fp4s_dn0 = 0.0;
        locals.var_psim_fp4s_dn1 = 0.0;
        locals.var_psim_fp4s_dn2 = 0.0;
        locals.var_psim_fp4s_dn3 = 0.0;
        locals.var_psim_fp4s_dn4 = 0.0;
        locals.var_psim_fp4s_dn5 = 0.0;
        locals.var_psim_fp4s_dn6 = 0.0;
        locals.var_psim_fp4s_dn7 = 0.0;
        locals.var_psim_fp4s_dn8 = 0.0;
        locals.var_psim_fp4s_dn9 = 0.0;
        locals.var_psim_fp4s_dn12 = 0.0;
        locals.var_psim_fp4s_dn14 = 0.0;
        locals.var_psim_fp4s_dn15 = 0.0;
        locals.var_psim_fp4s_dn16 = 0.0;
        locals.var_psim_fp4s_dn17 = 0.0;
        locals.var_psim_fp4s_dn18 = 0.0;
        locals.var_psim_fp4s_dn19 = 0.0;
        locals.var_psim_fp4s_dn20 = 0.0;
        locals.var_psim_fp4s_dn21 = 0.0;
        locals.var_psim_fp4s_dn22 = 0.0;
        locals.var_psim_fp4s_rv = 0.0;

        locals.var_psisd_fp4s = 0.0;
        locals.var_psisd_fp4s_dn0 = 0.0;
        locals.var_psisd_fp4s_dn1 = 0.0;
        locals.var_psisd_fp4s_dn2 = 0.0;
        locals.var_psisd_fp4s_dn3 = 0.0;
        locals.var_psisd_fp4s_dn4 = 0.0;
        locals.var_psisd_fp4s_dn5 = 0.0;
        locals.var_psisd_fp4s_dn6 = 0.0;
        locals.var_psisd_fp4s_dn7 = 0.0;
        locals.var_psisd_fp4s_dn8 = 0.0;
        locals.var_psisd_fp4s_dn9 = 0.0;
        locals.var_psisd_fp4s_dn12 = 0.0;
        locals.var_psisd_fp4s_dn14 = 0.0;
        locals.var_psisd_fp4s_dn15 = 0.0;
        locals.var_psisd_fp4s_dn16 = 0.0;
        locals.var_psisd_fp4s_dn17 = 0.0;
        locals.var_psisd_fp4s_dn18 = 0.0;
        locals.var_psisd_fp4s_dn19 = 0.0;
        locals.var_psisd_fp4s_dn20 = 0.0;
        locals.var_psisd_fp4s_dn21 = 0.0;
        locals.var_psisd_fp4s_dn22 = 0.0;
        locals.var_psisd_fp4s_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_3(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let ctx_temp = ctx.temperature();
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        locals.var_qg_fp4s = 0.0;
        locals.var_qg_fp4s_dn0 = 0.0;
        locals.var_qg_fp4s_dn1 = 0.0;
        locals.var_qg_fp4s_dn2 = 0.0;
        locals.var_qg_fp4s_dn3 = 0.0;
        locals.var_qg_fp4s_dn4 = 0.0;
        locals.var_qg_fp4s_dn5 = 0.0;
        locals.var_qg_fp4s_dn6 = 0.0;
        locals.var_qg_fp4s_dn7 = 0.0;
        locals.var_qg_fp4s_dn8 = 0.0;
        locals.var_qg_fp4s_dn9 = 0.0;
        locals.var_qg_fp4s_dn12 = 0.0;
        locals.var_qg_fp4s_dn14 = 0.0;
        locals.var_qg_fp4s_dn15 = 0.0;
        locals.var_qg_fp4s_dn16 = 0.0;
        locals.var_qg_fp4s_dn17 = 0.0;
        locals.var_qg_fp4s_dn18 = 0.0;
        locals.var_qg_fp4s_dn19 = 0.0;
        locals.var_qg_fp4s_dn20 = 0.0;
        locals.var_qg_fp4s_dn21 = 0.0;
        locals.var_qg_fp4s_dn22 = 0.0;
        locals.var_qg_fp4s_rv = 0.0;

        locals.var_qd_fp4s = 0.0;
        locals.var_qd_fp4s_dn0 = 0.0;
        locals.var_qd_fp4s_dn1 = 0.0;
        locals.var_qd_fp4s_dn2 = 0.0;
        locals.var_qd_fp4s_dn3 = 0.0;
        locals.var_qd_fp4s_dn4 = 0.0;
        locals.var_qd_fp4s_dn5 = 0.0;
        locals.var_qd_fp4s_dn6 = 0.0;
        locals.var_qd_fp4s_dn7 = 0.0;
        locals.var_qd_fp4s_dn8 = 0.0;
        locals.var_qd_fp4s_dn9 = 0.0;
        locals.var_qd_fp4s_dn12 = 0.0;
        locals.var_qd_fp4s_dn14 = 0.0;
        locals.var_qd_fp4s_dn15 = 0.0;
        locals.var_qd_fp4s_dn16 = 0.0;
        locals.var_qd_fp4s_dn17 = 0.0;
        locals.var_qd_fp4s_dn18 = 0.0;
        locals.var_qd_fp4s_dn19 = 0.0;
        locals.var_qd_fp4s_dn20 = 0.0;
        locals.var_qd_fp4s_dn21 = 0.0;
        locals.var_qd_fp4s_dn22 = 0.0;
        locals.var_qd_fp4s_rv = 0.0;

        locals.var_vgs_fp4s = 0.0;
        locals.var_vgs_fp4s_dn2 = 0.0;
        locals.var_vgs_fp4s_dn8 = 0.0;
        locals.var_vgs_fp4s_dn9 = 0.0;
        locals.var_vgs_fp4s_dn21 = 0.0;
        locals.var_vgs_fp4s_dn22 = 0.0;
        locals.var_vgs_fp4s_rv = 0.0;

        locals.var_vds_fp4s = 0.0;
        locals.var_vds_fp4s_dn21 = 0.0;
        locals.var_vds_fp4s_dn22 = 0.0;
        locals.var_vds_fp4s_rv = 0.0;

        locals.var_cr = 0.01;
        locals.var_cr_dn0 = 0.0;
        locals.var_cr_dn1 = 0.0;
        locals.var_cr_dn2 = 0.0;
        locals.var_cr_dn3 = 0.0;
        locals.var_cr_dn4 = 0.0;
        locals.var_cr_dn5 = 0.0;
        locals.var_cr_dn6 = 0.0;
        locals.var_cr_dn7 = 0.0;
        locals.var_cr_dn8 = 0.0;
        locals.var_cr_dn9 = 0.0;
        locals.var_cr_dn12 = 0.0;
        locals.var_cr_dn14 = 0.0;
        locals.var_cr_dn15 = 0.0;
        locals.var_cr_dn16 = 0.0;
        locals.var_cr_dn17 = 0.0;
        locals.var_cr_dn18 = 0.0;
        locals.var_cr_dn19 = 0.0;
        locals.var_cr_dn20 = 0.0;
        locals.var_cr_dn21 = 0.0;
        locals.var_cr_dn22 = 0.0;
        locals.var_cr_rv = 0.0;

        locals.var_crm = 0.01;
        locals.var_crm_dn0 = 0.0;
        locals.var_crm_dn1 = 0.0;
        locals.var_crm_dn2 = 0.0;
        locals.var_crm_dn3 = 0.0;
        locals.var_crm_dn4 = 0.0;
        locals.var_crm_dn5 = 0.0;
        locals.var_crm_dn6 = 0.0;
        locals.var_crm_dn7 = 0.0;
        locals.var_crm_dn8 = 0.0;
        locals.var_crm_dn9 = 0.0;
        locals.var_crm_dn12 = 0.0;
        locals.var_crm_dn14 = 0.0;
        locals.var_crm_dn15 = 0.0;
        locals.var_crm_dn16 = 0.0;
        locals.var_crm_dn17 = 0.0;
        locals.var_crm_dn18 = 0.0;
        locals.var_crm_dn19 = 0.0;
        locals.var_crm_dn20 = 0.0;
        locals.var_crm_dn21 = 0.0;
        locals.var_crm_dn22 = 0.0;
        locals.var_crm_rv = 0.0;

        locals.var_sigvdsfp1 = 1.0;
        locals.var_sigvdsfp1_rv = 0.0;

        locals.var_sigvdsfp2 = 1.0;
        locals.var_sigvdsfp2_rv = 0.0;

        locals.var_sigvdsfp3 = 1.0;
        locals.var_sigvdsfp3_rv = 0.0;

        locals.var_sigvdsfp4 = 1.0;
        locals.var_sigvdsfp4_rv = 0.0;

        locals.var_sigvdsfp1s = 1.0;
        locals.var_sigvdsfp1s_rv = 0.0;

        locals.var_sigvdsfp2s = 1.0;
        locals.var_sigvdsfp2s_rv = 0.0;

        locals.var_sigvdsfp3s = 1.0;
        locals.var_sigvdsfp3s_rv = 0.0;

        locals.var_sigvdsfp4s = 1.0;
        locals.var_sigvdsfp4s_rv = 0.0;

        locals.var_mvgs = 0.0;
        locals.var_mvgs_dn8 = 0.0;
        locals.var_mvgs_dn9 = 0.0;
        locals.var_mvgs_rv = 0.0;

        locals.var_mvgd = 0.0;
        locals.var_mvgd_dn7 = 0.0;
        locals.var_mvgd_dn9 = 0.0;
        locals.var_mvgd_rv = 0.0;

        locals.var_vbis_t = 0.0;
        locals.var_vbis_t_dn4 = 0.0;
        locals.var_vbis_t_rv = 0.0;

        locals.var_vbid_t = 0.0;
        locals.var_vbid_t_dn4 = 0.0;
        locals.var_vbid_t_rv = 0.0;

        locals.var_njgs_t = 1.0;
        locals.var_njgs_t_dn4 = 0.0;
        locals.var_njgs_t_rv = 0.0;

        locals.var_njgd_t = 1.0;
        locals.var_njgd_t_dn4 = 0.0;
        locals.var_njgd_t_rv = 0.0;

        locals.var_voffdlag = 0.0;
        locals.var_voffdlag_dn0 = 0.0;
        locals.var_voffdlag_dn1 = 0.0;
        locals.var_voffdlag_dn2 = 0.0;
        locals.var_voffdlag_dn3 = 0.0;
        locals.var_voffdlag_dn4 = 0.0;
        locals.var_voffdlag_dn5 = 0.0;
        locals.var_voffdlag_dn6 = 0.0;
        locals.var_voffdlag_dn7 = 0.0;
        locals.var_voffdlag_dn8 = 0.0;
        locals.var_voffdlag_dn9 = 0.0;
        locals.var_voffdlag_dn12 = 0.0;
        locals.var_voffdlag_dn14 = 0.0;
        locals.var_voffdlag_dn15 = 0.0;
        locals.var_voffdlag_dn16 = 0.0;
        locals.var_voffdlag_dn17 = 0.0;
        locals.var_voffdlag_dn18 = 0.0;
        locals.var_voffdlag_dn19 = 0.0;
        locals.var_voffdlag_dn20 = 0.0;
        locals.var_voffdlag_dn21 = 0.0;
        locals.var_voffdlag_dn22 = 0.0;
        locals.var_voffdlag_rv = 0.0;

        locals.var_voffglag = 0.0;
        locals.var_voffglag_dn0 = 0.0;
        locals.var_voffglag_dn1 = 0.0;
        locals.var_voffglag_dn2 = 0.0;
        locals.var_voffglag_dn3 = 0.0;
        locals.var_voffglag_dn4 = 0.0;
        locals.var_voffglag_dn5 = 0.0;
        locals.var_voffglag_dn6 = 0.0;
        locals.var_voffglag_dn7 = 0.0;
        locals.var_voffglag_dn8 = 0.0;
        locals.var_voffglag_dn9 = 0.0;
        locals.var_voffglag_dn12 = 0.0;
        locals.var_voffglag_dn14 = 0.0;
        locals.var_voffglag_dn15 = 0.0;
        locals.var_voffglag_dn16 = 0.0;
        locals.var_voffglag_dn17 = 0.0;
        locals.var_voffglag_dn18 = 0.0;
        locals.var_voffglag_dn19 = 0.0;
        locals.var_voffglag_dn20 = 0.0;
        locals.var_voffglag_dn21 = 0.0;
        locals.var_voffglag_dn22 = 0.0;
        locals.var_voffglag_rv = 0.0;

        locals.var_u0glag = 0.0;
        locals.var_u0glag_dn0 = 0.0;
        locals.var_u0glag_dn1 = 0.0;
        locals.var_u0glag_dn2 = 0.0;
        locals.var_u0glag_dn3 = 0.0;
        locals.var_u0glag_dn4 = 0.0;
        locals.var_u0glag_dn5 = 0.0;
        locals.var_u0glag_dn6 = 0.0;
        locals.var_u0glag_dn7 = 0.0;
        locals.var_u0glag_dn8 = 0.0;
        locals.var_u0glag_dn9 = 0.0;
        locals.var_u0glag_dn12 = 0.0;
        locals.var_u0glag_dn14 = 0.0;
        locals.var_u0glag_dn15 = 0.0;
        locals.var_u0glag_dn16 = 0.0;
        locals.var_u0glag_dn17 = 0.0;
        locals.var_u0glag_dn18 = 0.0;
        locals.var_u0glag_dn19 = 0.0;
        locals.var_u0glag_dn20 = 0.0;
        locals.var_u0glag_dn21 = 0.0;
        locals.var_u0glag_dn22 = 0.0;
        locals.var_u0glag_rv = 0.0;

        locals.var_ns0ddlag = 0.0;
        locals.var_ns0ddlag_dn0 = 0.0;
        locals.var_ns0ddlag_dn1 = 0.0;
        locals.var_ns0ddlag_dn2 = 0.0;
        locals.var_ns0ddlag_dn3 = 0.0;
        locals.var_ns0ddlag_dn4 = 0.0;
        locals.var_ns0ddlag_dn5 = 0.0;
        locals.var_ns0ddlag_dn6 = 0.0;
        locals.var_ns0ddlag_dn7 = 0.0;
        locals.var_ns0ddlag_dn8 = 0.0;
        locals.var_ns0ddlag_dn9 = 0.0;
        locals.var_ns0ddlag_dn12 = 0.0;
        locals.var_ns0ddlag_dn14 = 0.0;
        locals.var_ns0ddlag_dn15 = 0.0;
        locals.var_ns0ddlag_dn16 = 0.0;
        locals.var_ns0ddlag_dn17 = 0.0;
        locals.var_ns0ddlag_dn18 = 0.0;
        locals.var_ns0ddlag_dn19 = 0.0;
        locals.var_ns0ddlag_dn20 = 0.0;
        locals.var_ns0ddlag_dn21 = 0.0;
        locals.var_ns0ddlag_dn22 = 0.0;
        locals.var_ns0ddlag_rv = 0.0;

        locals.var_ns0sdlag = 0.0;
        locals.var_ns0sdlag_dn0 = 0.0;
        locals.var_ns0sdlag_dn1 = 0.0;
        locals.var_ns0sdlag_dn2 = 0.0;
        locals.var_ns0sdlag_dn3 = 0.0;
        locals.var_ns0sdlag_dn4 = 0.0;
        locals.var_ns0sdlag_dn5 = 0.0;
        locals.var_ns0sdlag_dn6 = 0.0;
        locals.var_ns0sdlag_dn7 = 0.0;
        locals.var_ns0sdlag_dn8 = 0.0;
        locals.var_ns0sdlag_dn9 = 0.0;
        locals.var_ns0sdlag_dn12 = 0.0;
        locals.var_ns0sdlag_dn14 = 0.0;
        locals.var_ns0sdlag_dn15 = 0.0;
        locals.var_ns0sdlag_dn16 = 0.0;
        locals.var_ns0sdlag_dn17 = 0.0;
        locals.var_ns0sdlag_dn18 = 0.0;
        locals.var_ns0sdlag_dn19 = 0.0;
        locals.var_ns0sdlag_dn20 = 0.0;
        locals.var_ns0sdlag_dn21 = 0.0;
        locals.var_ns0sdlag_dn22 = 0.0;
        locals.var_ns0sdlag_rv = 0.0;

        locals.var_vsatglag = 0.0;
        locals.var_vsatglag_dn0 = 0.0;
        locals.var_vsatglag_dn1 = 0.0;
        locals.var_vsatglag_dn2 = 0.0;
        locals.var_vsatglag_dn3 = 0.0;
        locals.var_vsatglag_dn4 = 0.0;
        locals.var_vsatglag_dn5 = 0.0;
        locals.var_vsatglag_dn6 = 0.0;
        locals.var_vsatglag_dn7 = 0.0;
        locals.var_vsatglag_dn8 = 0.0;
        locals.var_vsatglag_dn9 = 0.0;
        locals.var_vsatglag_dn12 = 0.0;
        locals.var_vsatglag_dn14 = 0.0;
        locals.var_vsatglag_dn15 = 0.0;
        locals.var_vsatglag_dn16 = 0.0;
        locals.var_vsatglag_dn17 = 0.0;
        locals.var_vsatglag_dn18 = 0.0;
        locals.var_vsatglag_dn19 = 0.0;
        locals.var_vsatglag_dn20 = 0.0;
        locals.var_vsatglag_dn21 = 0.0;
        locals.var_vsatglag_dn22 = 0.0;
        locals.var_vsatglag_rv = 0.0;

        locals.var_ns0dglag = 0.0;
        locals.var_ns0dglag_dn0 = 0.0;
        locals.var_ns0dglag_dn1 = 0.0;
        locals.var_ns0dglag_dn2 = 0.0;
        locals.var_ns0dglag_dn3 = 0.0;
        locals.var_ns0dglag_dn4 = 0.0;
        locals.var_ns0dglag_dn5 = 0.0;
        locals.var_ns0dglag_dn6 = 0.0;
        locals.var_ns0dglag_dn7 = 0.0;
        locals.var_ns0dglag_dn8 = 0.0;
        locals.var_ns0dglag_dn9 = 0.0;
        locals.var_ns0dglag_dn12 = 0.0;
        locals.var_ns0dglag_dn14 = 0.0;
        locals.var_ns0dglag_dn15 = 0.0;
        locals.var_ns0dglag_dn16 = 0.0;
        locals.var_ns0dglag_dn17 = 0.0;
        locals.var_ns0dglag_dn18 = 0.0;
        locals.var_ns0dglag_dn19 = 0.0;
        locals.var_ns0dglag_dn20 = 0.0;
        locals.var_ns0dglag_dn21 = 0.0;
        locals.var_ns0dglag_dn22 = 0.0;
        locals.var_ns0dglag_rv = 0.0;

        locals.var_ns0sglag = 0.0;
        locals.var_ns0sglag_dn0 = 0.0;
        locals.var_ns0sglag_dn1 = 0.0;
        locals.var_ns0sglag_dn2 = 0.0;
        locals.var_ns0sglag_dn3 = 0.0;
        locals.var_ns0sglag_dn4 = 0.0;
        locals.var_ns0sglag_dn5 = 0.0;
        locals.var_ns0sglag_dn6 = 0.0;
        locals.var_ns0sglag_dn7 = 0.0;
        locals.var_ns0sglag_dn8 = 0.0;
        locals.var_ns0sglag_dn9 = 0.0;
        locals.var_ns0sglag_dn12 = 0.0;
        locals.var_ns0sglag_dn14 = 0.0;
        locals.var_ns0sglag_dn15 = 0.0;
        locals.var_ns0sglag_dn16 = 0.0;
        locals.var_ns0sglag_dn17 = 0.0;
        locals.var_ns0sglag_dn18 = 0.0;
        locals.var_ns0sglag_dn19 = 0.0;
        locals.var_ns0sglag_dn20 = 0.0;
        locals.var_ns0sglag_dn21 = 0.0;
        locals.var_ns0sglag_dn22 = 0.0;
        locals.var_ns0sglag_rv = 0.0;

        locals.var_rdsmod_i = p.p34;
        locals.var_rdsmod_i_rv = 0.0;

        let assign1440_e2932: f64 = if p.p149 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard350 = assign1440_e2932;
        locals.var_guard350_rv = 0.0;

        let assign1450_e2935: f64 = if locals.var_rdsmod_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard351 = assign1450_e2935;
        locals.var_guard351_rv = 0.0;

        let (assign1460_e2941,) = {
    if ((locals.var_guard350 != 0.0) && (locals.var_guard351 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_rdsmod_i,)
    }
};
        locals.var_rdsmod_i = assign1460_e2941;
        locals.var_rdsmod_i_rv = 0.0;

        let assign1470_e2944: f64 = (p.p0 + 273.15);
        locals.var_tnom = assign1470_e2944;
        locals.var_tnom_rv = 0.0;

        locals.var_vds_noswap = (nv7 - nv8);
        locals.var_vds_noswap_dn7 = 1.0;
        locals.var_vds_noswap_dn8 = -1.0;
        locals.var_vds_noswap_rv = 0.0;

        locals.var_vgs_noswap = (nv9 - nv8);
        locals.var_vgs_noswap_dn8 = -1.0;
        locals.var_vgs_noswap_dn9 = 1.0;
        locals.var_vgs_noswap_rv = 0.0;

        locals.var_vgd_noswap = (nv9 - nv7);
        locals.var_vgd_noswap_dn7 = -1.0;
        locals.var_vgd_noswap_dn9 = 1.0;
        locals.var_vgd_noswap_rv = 0.0;

        locals.var_vbs_noswap = (nv3 - nv8);
        locals.var_vbs_noswap_dn3 = 1.0;
        locals.var_vbs_noswap_dn8 = -1.0;
        locals.var_vbs_noswap_rv = 0.0;

        locals.var_vbd_noswap = (nv3 - nv7);
        locals.var_vbd_noswap_dn3 = 1.0;
        locals.var_vbd_noswap_dn7 = -1.0;
        locals.var_vbd_noswap_rv = 0.0;

        locals.var_sigvds = 1.0;
        locals.var_sigvds_rv = 0.0;

        let assign1540_e2953: f64 = if locals.var_vds_noswap < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard352 = assign1540_e2953;
        locals.var_guard352_rv = 0.0;

        let (assign1550_e2958,) = {
    if (locals.var_guard352 != 0.0) {
        let assign1550_e2956: f64 = (-1.0);
        (assign1550_e2956,)
    } else {
        (locals.var_sigvds,)
    }
};
        locals.var_sigvds = assign1550_e2958;
        locals.var_sigvds_rv = 0.0;

        let (assign1560_e2964, assign1560_e2964_d_n7, assign1560_e2964_d_n8,) = {
    if (locals.var_guard352 != 0.0) {
        let assign1560_e2962: f64 = (locals.var_sigvds * locals.var_vds_noswap);
        (assign1560_e2962, (locals.var_sigvds * locals.var_vds_noswap_dn7), (locals.var_sigvds * locals.var_vds_noswap_dn8),)
    } else {
        (locals.var_vds, locals.var_vds_dn7, locals.var_vds_dn8,)
    }
};
        locals.var_vds = assign1560_e2964;
        locals.var_vds_dn7 = assign1560_e2964_d_n7;
        locals.var_vds_dn8 = assign1560_e2964_d_n8;
        locals.var_vds_rv = 0.0;

        let (assign1570_e2968, assign1570_e2968_d_n7, assign1570_e2968_d_n8, assign1570_e2968_d_n9,) = {
    if (locals.var_guard352 != 0.0) {
        (locals.var_vgd_noswap, locals.var_vgd_noswap_dn7, 0.0, locals.var_vgd_noswap_dn9,)
    } else {
        (locals.var_vgs, locals.var_vgs_dn7, locals.var_vgs_dn8, locals.var_vgs_dn9,)
    }
};
        locals.var_vgs = assign1570_e2968;
        locals.var_vgs_dn7 = assign1570_e2968_d_n7;
        locals.var_vgs_dn8 = assign1570_e2968_d_n8;
        locals.var_vgs_dn9 = assign1570_e2968_d_n9;
        locals.var_vgs_rv = 0.0;

        let (assign1580_e2972, assign1580_e2972_d_n3, assign1580_e2972_d_n7, assign1580_e2972_d_n8,) = {
    if (locals.var_guard352 != 0.0) {
        (locals.var_vbd_noswap, locals.var_vbd_noswap_dn3, locals.var_vbd_noswap_dn7, 0.0,)
    } else {
        (locals.var_vbs, locals.var_vbs_dn3, locals.var_vbs_dn7, locals.var_vbs_dn8,)
    }
};
        locals.var_vbs = assign1580_e2972;
        locals.var_vbs_dn3 = assign1580_e2972_d_n3;
        locals.var_vbs_dn7 = assign1580_e2972_d_n7;
        locals.var_vbs_dn8 = assign1580_e2972_d_n8;
        locals.var_vbs_rv = 0.0;

        let (assign1590_e2977, assign1590_e2977_d_n7, assign1590_e2977_d_n8,) = {
    if (locals.var_guard352 == 0.0) {
        (locals.var_vds_noswap, locals.var_vds_noswap_dn7, locals.var_vds_noswap_dn8,)
    } else {
        (locals.var_vds, locals.var_vds_dn7, locals.var_vds_dn8,)
    }
};
        locals.var_vds = assign1590_e2977;
        locals.var_vds_dn7 = assign1590_e2977_d_n7;
        locals.var_vds_dn8 = assign1590_e2977_d_n8;
        locals.var_vds_rv = 0.0;

        let (assign1600_e2982, assign1600_e2982_d_n7, assign1600_e2982_d_n8, assign1600_e2982_d_n9,) = {
    if (locals.var_guard352 == 0.0) {
        (locals.var_vgs_noswap, 0.0, locals.var_vgs_noswap_dn8, locals.var_vgs_noswap_dn9,)
    } else {
        (locals.var_vgs, locals.var_vgs_dn7, locals.var_vgs_dn8, locals.var_vgs_dn9,)
    }
};
        locals.var_vgs = assign1600_e2982;
        locals.var_vgs_dn7 = assign1600_e2982_d_n7;
        locals.var_vgs_dn8 = assign1600_e2982_d_n8;
        locals.var_vgs_dn9 = assign1600_e2982_d_n9;
        locals.var_vgs_rv = 0.0;

        let (assign1610_e2987, assign1610_e2987_d_n3, assign1610_e2987_d_n7, assign1610_e2987_d_n8,) = {
    if (locals.var_guard352 == 0.0) {
        (locals.var_vbs_noswap, locals.var_vbs_noswap_dn3, 0.0, locals.var_vbs_noswap_dn8,)
    } else {
        (locals.var_vbs, locals.var_vbs_dn3, locals.var_vbs_dn7, locals.var_vbs_dn8,)
    }
};
        locals.var_vbs = assign1610_e2987;
        locals.var_vbs_dn3 = assign1610_e2987_d_n3;
        locals.var_vbs_dn7 = assign1610_e2987_d_n7;
        locals.var_vbs_dn8 = assign1610_e2987_d_n8;
        locals.var_vbs_rv = 0.0;

        let assign1620_e2990: f64 = (locals.var_vds * locals.var_vds);
        let assign1620_e2992: f64 = (assign1620_e2990 + 0.01);
        let assign1620_e2993: f64 = (assign1620_e2992).sqrt();
        let assign1620_e2995: f64 = (assign1620_e2993 - 0.1);
        locals.var_vdsx = assign1620_e2995;
        locals.var_vdsx_dn7 = (((locals.var_vds_dn7 * locals.var_vds) + (locals.var_vds * locals.var_vds_dn7)) / (2.0 * assign1620_e2993));
        locals.var_vdsx_dn8 = (((locals.var_vds_dn8 * locals.var_vds) + (locals.var_vds * locals.var_vds_dn8)) / (2.0 * assign1620_e2993));
        locals.var_vdsx_rv = 0.0;

        let assign1640_e3004: f64 = ctx_temp;
        let assign1640_e3006: f64 = (assign1640_e3004 + (nv4 - 0.0));
        let assign1640_e3008: f64 = (assign1640_e3006 + p.p274);
        locals.var_tdev = assign1640_e3008;
        locals.var_tdev_dn4 = 1.0;
        locals.var_tdev_rv = 0.0;

        let assign1650_e3011: f64 = (8.617087e-5 * locals.var_tdev);
        locals.var_vth = assign1650_e3011;
        locals.var_vth_dn4 = (8.617087e-5 * locals.var_tdev_dn4);
        locals.var_vth_rv = 0.0;

        let assign1660_e3014: f64 = if p.p81 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard353 = assign1660_e3014;
        locals.var_guard353_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_4(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let assign1670_e3017: f64 = if p.p81 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard354 = assign1670_e3017;
        locals.var_guard354_rv = 0.0;

        let assign1680_e3020: f64 = if p.p81 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard355 = assign1680_e3020;
        locals.var_guard355_rv = 0.0;

        let assign1690_e3023: f64 = if p.p81 == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard356 = assign1690_e3023;
        locals.var_guard356_rv = 0.0;

        let assign1700_e3026: f64 = if p.p81 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard357 = assign1700_e3026;
        locals.var_guard357_rv = 0.0;

        let assign1710_e3029: f64 = if p.p81 == 5.0 { 1.0 } else { 0.0 };
        locals.var_guard358 = assign1710_e3029;
        locals.var_guard358_rv = 0.0;

        let (assign1720_e3036, assign1720_e3036_d_n4, assign1720_e3036_d_n5,) = {
    if ((locals.var_guard354 != 0.0) && (locals.var_guard353 == 0.0)) {
        ((nv5 - 0.0), 0.0, 1.0,)
    } else {
        (locals.var_vcap, locals.var_vcap_dn4, locals.var_vcap_dn5,)
    }
};
        locals.var_vcap = assign1720_e3036;
        locals.var_vcap_dn4 = assign1720_e3036_d_n4;
        locals.var_vcap_dn5 = assign1720_e3036_d_n5;
        locals.var_vcap_rv = 0.0;

        let (assign1730_e3062, assign1730_e3062_d_n4, assign1730_e3062_d_n5,) = {
    if ((locals.var_guard354 != 0.0) && (locals.var_guard353 == 0.0)) {
        let assign1730_e3044: f64 = (locals.var_vcap + locals.var_vth);
        let assign1730_e3047: f64 = (locals.var_vcap - locals.var_vth);
        let assign1730_e3050: f64 = (locals.var_vcap - locals.var_vth);
        let assign1730_e3051: f64 = (assign1730_e3047 * assign1730_e3050);
        let assign1730_e3054: f64 = (0.25 * p.p128);
        let assign1730_e3056: f64 = (assign1730_e3054 * p.p128);
        let assign1730_e3057: f64 = (assign1730_e3051 + assign1730_e3056);
        let assign1730_e3058: f64 = (assign1730_e3057).sqrt();
        let assign1730_e3059: f64 = (assign1730_e3044 + assign1730_e3058);
        let assign1730_e3060: f64 = (0.5 * assign1730_e3059);
        (assign1730_e3060, (0.5 * ((locals.var_vcap_dn4 + locals.var_vth_dn4) + ((((locals.var_vcap_dn4 - locals.var_vth_dn4) * assign1730_e3050) + (assign1730_e3047 * (locals.var_vcap_dn4 - locals.var_vth_dn4))) / (2.0 * assign1730_e3058)))), (0.5 * (locals.var_vcap_dn5 + (((locals.var_vcap_dn5 * assign1730_e3050) + (assign1730_e3047 * locals.var_vcap_dn5)) / (2.0 * assign1730_e3058)))),)
    } else {
        (locals.var_vcap, locals.var_vcap_dn4, locals.var_vcap_dn5,)
    }
};
        locals.var_vcap = assign1730_e3062;
        locals.var_vcap_dn4 = assign1730_e3062_d_n4;
        locals.var_vcap_dn5 = assign1730_e3062_d_n5;
        locals.var_vcap_rv = 0.0;

        let (assign1740_e3077, assign1740_e3077_d_n4, assign1740_e3077_d_n5,) = {
    if ((locals.var_guard354 != 0.0) && (locals.var_guard353 == 0.0)) {
        let assign1740_e3070: f64 = (-1.0);
        let assign1740_e3072: f64 = (assign1740_e3070 / locals.var_vcap);
        let assign1740_e3073: f64 = { let limited_exp_arg = assign1740_e3072; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign1740_e3074: f64 = (p.p101 * assign1740_e3073);
        let assign1740_e3075: f64 = (p.p100 + assign1740_e3074);
        (assign1740_e3075, (p.p101 * ({ let limited_exp_arg = assign1740_e3072; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-((assign1740_e3070 * locals.var_vcap_dn4) / (locals.var_vcap * locals.var_vcap))))), (p.p101 * ({ let limited_exp_arg = assign1740_e3072; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-((assign1740_e3070 * locals.var_vcap_dn5) / (locals.var_vcap * locals.var_vcap))))),)
    } else {
        (locals.var_voff_cap, locals.var_voff_cap_dn4, locals.var_voff_cap_dn5,)
    }
};
        locals.var_voff_cap = assign1740_e3077;
        locals.var_voff_cap_dn4 = assign1740_e3077_d_n4;
        locals.var_voff_cap_dn5 = assign1740_e3077_d_n5;
        locals.var_voff_cap_rv = 0.0;

        let (assign1770_e3122, assign1770_e3122_d_n4, assign1770_e3122_d_n5,) = {
    if ((locals.var_guard354 != 0.0) && (locals.var_guard353 == 0.0)) {
        let assign1770_e3115: f64 = (-1.0);
        let assign1770_e3117: f64 = (assign1770_e3115 / locals.var_vcap);
        let assign1770_e3118: f64 = { let limited_exp_arg = assign1770_e3117; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign1770_e3119: f64 = (p.p103 * assign1770_e3118);
        let assign1770_e3120: f64 = (p.p102 + assign1770_e3119);
        (assign1770_e3120, (p.p103 * ({ let limited_exp_arg = assign1770_e3117; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-((assign1770_e3115 * locals.var_vcap_dn4) / (locals.var_vcap * locals.var_vcap))))), (p.p103 * ({ let limited_exp_arg = assign1770_e3117; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-((assign1770_e3115 * locals.var_vcap_dn5) / (locals.var_vcap * locals.var_vcap))))),)
    } else {
        (locals.var_eta0_cap, locals.var_eta0_cap_dn4, locals.var_eta0_cap_dn5,)
    }
};
        locals.var_eta0_cap = assign1770_e3122;
        locals.var_eta0_cap_dn4 = assign1770_e3122_d_n4;
        locals.var_eta0_cap_dn5 = assign1770_e3122_d_n5;
        locals.var_eta0_cap_rv = 0.0;

        let (assign1790_e3146, assign1790_e3146_d_n6,) = {
    if ((locals.var_guard355 != 0.0) && (!((locals.var_guard353 != 0.0) || (locals.var_guard354 != 0.0)))) {
        let assign1790_e3144: f64 = (p.p113 * (nv6 - 0.0));
        (assign1790_e3144, p.p113,)
    } else {
        (locals.var_voff_trap, locals.var_voff_trap_dn6,)
    }
};
        locals.var_voff_trap = assign1790_e3146;
        locals.var_voff_trap_dn6 = assign1790_e3146_d_n6;
        locals.var_voff_trap_rv = 0.0;

        let (assign1810_e3175, assign1810_e3175_d_n6,) = {
    if ((locals.var_guard355 != 0.0) && (!((locals.var_guard353 != 0.0) || (locals.var_guard354 != 0.0)))) {
        let assign1810_e3173: f64 = (p.p114 * (nv6 - 0.0));
        (assign1810_e3173, p.p114,)
    } else {
        (locals.var_cdscd_trap, locals.var_cdscd_trap_dn6,)
    }
};
        locals.var_cdscd_trap = assign1810_e3175;
        locals.var_cdscd_trap_dn6 = assign1810_e3175_d_n6;
        locals.var_cdscd_trap_rv = 0.0;

        let (assign1820_e3186, assign1820_e3186_d_n6,) = {
    if ((locals.var_guard355 != 0.0) && (!((locals.var_guard353 != 0.0) || (locals.var_guard354 != 0.0)))) {
        let assign1820_e3184: f64 = (p.p115 * (nv6 - 0.0));
        (assign1820_e3184, p.p115,)
    } else {
        (locals.var_eta0_trap, locals.var_eta0_trap_dn6,)
    }
};
        locals.var_eta0_trap = assign1820_e3186;
        locals.var_eta0_trap_dn6 = assign1820_e3186_d_n6;
        locals.var_eta0_trap_rv = 0.0;

        let (assign1830_e3197, assign1830_e3197_d_n0, assign1830_e3197_d_n1,) = {
    if ((locals.var_guard356 != 0.0) && (!(((locals.var_guard353 != 0.0) || (locals.var_guard354 != 0.0)) || (locals.var_guard355 != 0.0)))) {
        ((nv0 - nv1), 1.0, -1.0,)
    } else {
        (locals.var_vdg, locals.var_vdg_dn0, locals.var_vdg_dn1,)
    }
};
        locals.var_vdg = assign1830_e3197;
        locals.var_vdg_dn0 = assign1830_e3197_d_n0;
        locals.var_vdg_dn1 = assign1830_e3197_d_n1;
        locals.var_vdg_rv = 0.0;

        let (assign1840_e3216, assign1840_e3216_d_n0, assign1840_e3216_d_n1, assign1840_e3216_d_n2, assign1840_e3216_d_n3, assign1840_e3216_d_n4, assign1840_e3216_d_n5, assign1840_e3216_d_n6, assign1840_e3216_d_n7, assign1840_e3216_d_n8, assign1840_e3216_d_n9, assign1840_e3216_d_n12, assign1840_e3216_d_n14, assign1840_e3216_d_n15, assign1840_e3216_d_n16, assign1840_e3216_d_n17, assign1840_e3216_d_n18, assign1840_e3216_d_n19, assign1840_e3216_d_n20, assign1840_e3216_d_n21, assign1840_e3216_d_n22,) = {
    if ((locals.var_guard356 != 0.0) && (!(((locals.var_guard353 != 0.0) || (locals.var_guard354 != 0.0)) || (locals.var_guard355 != 0.0)))) {
        let assign1840_e3210: f64 = (locals.var_vdg * p.p123);
        let assign1840_e3211: f64 = (1.0 + assign1840_e3210);
        let assign1840_e3212: f64 = (p.p124 / assign1840_e3211);
        let assign1840_e3214: f64 = (assign1840_e3212 * locals.var_vdg);
        (assign1840_e3214, (((-((p.p124 * (locals.var_vdg_dn0 * p.p123)) / (assign1840_e3211 * assign1840_e3211))) * locals.var_vdg) + (assign1840_e3212 * locals.var_vdg_dn0)), (((-((p.p124 * (locals.var_vdg_dn1 * p.p123)) / (assign1840_e3211 * assign1840_e3211))) * locals.var_vdg) + (assign1840_e3212 * locals.var_vdg_dn1)), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn1, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn12, locals.var_t1_dn14, locals.var_t1_dn15, locals.var_t1_dn16, locals.var_t1_dn17, locals.var_t1_dn18, locals.var_t1_dn19, locals.var_t1_dn20, locals.var_t1_dn21, locals.var_t1_dn22,)
    }
};
        locals.var_t1 = assign1840_e3216;
        locals.var_t1_dn0 = assign1840_e3216_d_n0;
        locals.var_t1_dn1 = assign1840_e3216_d_n1;
        locals.var_t1_dn2 = assign1840_e3216_d_n2;
        locals.var_t1_dn3 = assign1840_e3216_d_n3;
        locals.var_t1_dn4 = assign1840_e3216_d_n4;
        locals.var_t1_dn5 = assign1840_e3216_d_n5;
        locals.var_t1_dn6 = assign1840_e3216_d_n6;
        locals.var_t1_dn7 = assign1840_e3216_d_n7;
        locals.var_t1_dn8 = assign1840_e3216_d_n8;
        locals.var_t1_dn9 = assign1840_e3216_d_n9;
        locals.var_t1_dn12 = assign1840_e3216_d_n12;
        locals.var_t1_dn14 = assign1840_e3216_d_n14;
        locals.var_t1_dn15 = assign1840_e3216_d_n15;
        locals.var_t1_dn16 = assign1840_e3216_d_n16;
        locals.var_t1_dn17 = assign1840_e3216_d_n17;
        locals.var_t1_dn18 = assign1840_e3216_d_n18;
        locals.var_t1_dn19 = assign1840_e3216_d_n19;
        locals.var_t1_dn20 = assign1840_e3216_d_n20;
        locals.var_t1_dn21 = assign1840_e3216_d_n21;
        locals.var_t1_dn22 = assign1840_e3216_d_n22;
        locals.var_t1_rv = 0.0;

        let (assign1850_e3231, assign1850_e3231_d_n0, assign1850_e3231_d_n1, assign1850_e3231_d_n2, assign1850_e3231_d_n3, assign1850_e3231_d_n4, assign1850_e3231_d_n5, assign1850_e3231_d_n6, assign1850_e3231_d_n7, assign1850_e3231_d_n8, assign1850_e3231_d_n9, assign1850_e3231_d_n12, assign1850_e3231_d_n14, assign1850_e3231_d_n15, assign1850_e3231_d_n16, assign1850_e3231_d_n17, assign1850_e3231_d_n18, assign1850_e3231_d_n19, assign1850_e3231_d_n20, assign1850_e3231_d_n21, assign1850_e3231_d_n22,) = {
    if ((locals.var_guard356 != 0.0) && (!(((locals.var_guard353 != 0.0) || (locals.var_guard354 != 0.0)) || (locals.var_guard355 != 0.0)))) {
        let assign1850_e3228: f64 = (locals.var_vdg - p.p127);
        let assign1850_e3229: f64 = (p.p125 * assign1850_e3228);
        (assign1850_e3229, (p.p125 * locals.var_vdg_dn0), (p.p125 * locals.var_vdg_dn1), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn1, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn12, locals.var_t2_dn14, locals.var_t2_dn15, locals.var_t2_dn16, locals.var_t2_dn17, locals.var_t2_dn18, locals.var_t2_dn19, locals.var_t2_dn20, locals.var_t2_dn21, locals.var_t2_dn22,)
    }
};
        locals.var_t2 = assign1850_e3231;
        locals.var_t2_dn0 = assign1850_e3231_d_n0;
        locals.var_t2_dn1 = assign1850_e3231_d_n1;
        locals.var_t2_dn2 = assign1850_e3231_d_n2;
        locals.var_t2_dn3 = assign1850_e3231_d_n3;
        locals.var_t2_dn4 = assign1850_e3231_d_n4;
        locals.var_t2_dn5 = assign1850_e3231_d_n5;
        locals.var_t2_dn6 = assign1850_e3231_d_n6;
        locals.var_t2_dn7 = assign1850_e3231_d_n7;
        locals.var_t2_dn8 = assign1850_e3231_d_n8;
        locals.var_t2_dn9 = assign1850_e3231_d_n9;
        locals.var_t2_dn12 = assign1850_e3231_d_n12;
        locals.var_t2_dn14 = assign1850_e3231_d_n14;
        locals.var_t2_dn15 = assign1850_e3231_d_n15;
        locals.var_t2_dn16 = assign1850_e3231_d_n16;
        locals.var_t2_dn17 = assign1850_e3231_d_n17;
        locals.var_t2_dn18 = assign1850_e3231_d_n18;
        locals.var_t2_dn19 = assign1850_e3231_d_n19;
        locals.var_t2_dn20 = assign1850_e3231_d_n20;
        locals.var_t2_dn21 = assign1850_e3231_d_n21;
        locals.var_t2_dn22 = assign1850_e3231_d_n22;
        locals.var_t2_rv = 0.0;

        let (assign1870_e3280, assign1870_e3280_d_n0, assign1870_e3280_d_n1, assign1870_e3280_d_n2, assign1870_e3280_d_n3, assign1870_e3280_d_n4, assign1870_e3280_d_n5, assign1870_e3280_d_n6, assign1870_e3280_d_n7, assign1870_e3280_d_n8, assign1870_e3280_d_n9, assign1870_e3280_d_n12, assign1870_e3280_d_n14, assign1870_e3280_d_n15, assign1870_e3280_d_n16, assign1870_e3280_d_n17, assign1870_e3280_d_n18, assign1870_e3280_d_n19, assign1870_e3280_d_n20, assign1870_e3280_d_n21, assign1870_e3280_d_n22,) = {
    if ((locals.var_guard356 != 0.0) && (!(((locals.var_guard353 != 0.0) || (locals.var_guard354 != 0.0)) || (locals.var_guard355 != 0.0)))) {
        let assign1870_e3271: f64 = (-2.0);
        let assign1870_e3274: f64 = ((nv1 - nv2) - p.p10);
        let assign1870_e3275: f64 = (assign1870_e3271 * assign1870_e3274);
        let assign1870_e3277: f64 = (assign1870_e3275 / p.p122);
        let assign1870_e3278: f64 = (assign1870_e3277).exp();
        (assign1870_e3278, 0.0, (assign1870_e3278 * (assign1870_e3271 / p.p122)), (assign1870_e3278 * ((-assign1870_e3271) / p.p122)), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn1, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn12, locals.var_t0_dn14, locals.var_t0_dn15, locals.var_t0_dn16, locals.var_t0_dn17, locals.var_t0_dn18, locals.var_t0_dn19, locals.var_t0_dn20, locals.var_t0_dn21, locals.var_t0_dn22,)
    }
};
        locals.var_t0 = assign1870_e3280;
        locals.var_t0_dn0 = assign1870_e3280_d_n0;
        locals.var_t0_dn1 = assign1870_e3280_d_n1;
        locals.var_t0_dn2 = assign1870_e3280_d_n2;
        locals.var_t0_dn3 = assign1870_e3280_d_n3;
        locals.var_t0_dn4 = assign1870_e3280_d_n4;
        locals.var_t0_dn5 = assign1870_e3280_d_n5;
        locals.var_t0_dn6 = assign1870_e3280_d_n6;
        locals.var_t0_dn7 = assign1870_e3280_d_n7;
        locals.var_t0_dn8 = assign1870_e3280_d_n8;
        locals.var_t0_dn9 = assign1870_e3280_d_n9;
        locals.var_t0_dn12 = assign1870_e3280_d_n12;
        locals.var_t0_dn14 = assign1870_e3280_d_n14;
        locals.var_t0_dn15 = assign1870_e3280_d_n15;
        locals.var_t0_dn16 = assign1870_e3280_d_n16;
        locals.var_t0_dn17 = assign1870_e3280_d_n17;
        locals.var_t0_dn18 = assign1870_e3280_d_n18;
        locals.var_t0_dn19 = assign1870_e3280_d_n19;
        locals.var_t0_dn20 = assign1870_e3280_d_n20;
        locals.var_t0_dn21 = assign1870_e3280_d_n21;
        locals.var_t0_dn22 = assign1870_e3280_d_n22;
        locals.var_t0_rv = 0.0;

        let (assign1880_e3307, assign1880_e3307_d_n0, assign1880_e3307_d_n1, assign1880_e3307_d_n2, assign1880_e3307_d_n3, assign1880_e3307_d_n4, assign1880_e3307_d_n5, assign1880_e3307_d_n6, assign1880_e3307_d_n7, assign1880_e3307_d_n8, assign1880_e3307_d_n9, assign1880_e3307_d_n12, assign1880_e3307_d_n14, assign1880_e3307_d_n15, assign1880_e3307_d_n16, assign1880_e3307_d_n17, assign1880_e3307_d_n18, assign1880_e3307_d_n19, assign1880_e3307_d_n20, assign1880_e3307_d_n21, assign1880_e3307_d_n22,) = {
    if ((locals.var_guard356 != 0.0) && (!(((locals.var_guard353 != 0.0) || (locals.var_guard354 != 0.0)) || (locals.var_guard355 != 0.0)))) {
        let assign1880_e3292: f64 = (p.p120 - 1e-9);
        let assign1880_e3294: f64 = (assign1880_e3292 * 0.5);
        let assign1880_e3298: f64 = (1.0 - locals.var_t0);
        let assign1880_e3301: f64 = (1.0 + locals.var_t0);
        let assign1880_e3302: f64 = (assign1880_e3298 / assign1880_e3301);
        let assign1880_e3303: f64 = (1.0 + assign1880_e3302);
        let assign1880_e3304: f64 = (assign1880_e3294 * assign1880_e3303);
        let assign1880_e3305: f64 = (1e-9 + assign1880_e3304);
        (assign1880_e3305, (assign1880_e3294 * ((((-locals.var_t0_dn0) * assign1880_e3301) - (assign1880_e3298 * locals.var_t0_dn0)) / (assign1880_e3301 * assign1880_e3301))), (assign1880_e3294 * ((((-locals.var_t0_dn1) * assign1880_e3301) - (assign1880_e3298 * locals.var_t0_dn1)) / (assign1880_e3301 * assign1880_e3301))), (assign1880_e3294 * ((((-locals.var_t0_dn2) * assign1880_e3301) - (assign1880_e3298 * locals.var_t0_dn2)) / (assign1880_e3301 * assign1880_e3301))), (assign1880_e3294 * ((((-locals.var_t0_dn3) * assign1880_e3301) - (assign1880_e3298 * locals.var_t0_dn3)) / (assign1880_e3301 * assign1880_e3301))), (assign1880_e3294 * ((((-locals.var_t0_dn4) * assign1880_e3301) - (assign1880_e3298 * locals.var_t0_dn4)) / (assign1880_e3301 * assign1880_e3301))), (assign1880_e3294 * ((((-locals.var_t0_dn5) * assign1880_e3301) - (assign1880_e3298 * locals.var_t0_dn5)) / (assign1880_e3301 * assign1880_e3301))), (assign1880_e3294 * ((((-locals.var_t0_dn6) * assign1880_e3301) - (assign1880_e3298 * locals.var_t0_dn6)) / (assign1880_e3301 * assign1880_e3301))), (assign1880_e3294 * ((((-locals.var_t0_dn7) * assign1880_e3301) - (assign1880_e3298 * locals.var_t0_dn7)) / (assign1880_e3301 * assign1880_e3301))), (assign1880_e3294 * ((((-locals.var_t0_dn8) * assign1880_e3301) - (assign1880_e3298 * locals.var_t0_dn8)) / (assign1880_e3301 * assign1880_e3301))), (assign1880_e3294 * ((((-locals.var_t0_dn9) * assign1880_e3301) - (assign1880_e3298 * locals.var_t0_dn9)) / (assign1880_e3301 * assign1880_e3301))), (assign1880_e3294 * ((((-locals.var_t0_dn12) * assign1880_e3301) - (assign1880_e3298 * locals.var_t0_dn12)) / (assign1880_e3301 * assign1880_e3301))), (assign1880_e3294 * ((((-locals.var_t0_dn14) * assign1880_e3301) - (assign1880_e3298 * locals.var_t0_dn14)) / (assign1880_e3301 * assign1880_e3301))), (assign1880_e3294 * ((((-locals.var_t0_dn15) * assign1880_e3301) - (assign1880_e3298 * locals.var_t0_dn15)) / (assign1880_e3301 * assign1880_e3301))), (assign1880_e3294 * ((((-locals.var_t0_dn16) * assign1880_e3301) - (assign1880_e3298 * locals.var_t0_dn16)) / (assign1880_e3301 * assign1880_e3301))), (assign1880_e3294 * ((((-locals.var_t0_dn17) * assign1880_e3301) - (assign1880_e3298 * locals.var_t0_dn17)) / (assign1880_e3301 * assign1880_e3301))), (assign1880_e3294 * ((((-locals.var_t0_dn18) * assign1880_e3301) - (assign1880_e3298 * locals.var_t0_dn18)) / (assign1880_e3301 * assign1880_e3301))), (assign1880_e3294 * ((((-locals.var_t0_dn19) * assign1880_e3301) - (assign1880_e3298 * locals.var_t0_dn19)) / (assign1880_e3301 * assign1880_e3301))), (assign1880_e3294 * ((((-locals.var_t0_dn20) * assign1880_e3301) - (assign1880_e3298 * locals.var_t0_dn20)) / (assign1880_e3301 * assign1880_e3301))), (assign1880_e3294 * ((((-locals.var_t0_dn21) * assign1880_e3301) - (assign1880_e3298 * locals.var_t0_dn21)) / (assign1880_e3301 * assign1880_e3301))), (assign1880_e3294 * ((((-locals.var_t0_dn22) * assign1880_e3301) - (assign1880_e3298 * locals.var_t0_dn22)) / (assign1880_e3301 * assign1880_e3301))),)
    } else {
        (locals.var_ct, locals.var_ct_dn0, locals.var_ct_dn1, locals.var_ct_dn2, locals.var_ct_dn3, locals.var_ct_dn4, locals.var_ct_dn5, locals.var_ct_dn6, locals.var_ct_dn7, locals.var_ct_dn8, locals.var_ct_dn9, locals.var_ct_dn12, locals.var_ct_dn14, locals.var_ct_dn15, locals.var_ct_dn16, locals.var_ct_dn17, locals.var_ct_dn18, locals.var_ct_dn19, locals.var_ct_dn20, locals.var_ct_dn21, locals.var_ct_dn22,)
    }
};
        locals.var_ct = assign1880_e3307;
        locals.var_ct_dn0 = assign1880_e3307_d_n0;
        locals.var_ct_dn1 = assign1880_e3307_d_n1;
        locals.var_ct_dn2 = assign1880_e3307_d_n2;
        locals.var_ct_dn3 = assign1880_e3307_d_n3;
        locals.var_ct_dn4 = assign1880_e3307_d_n4;
        locals.var_ct_dn5 = assign1880_e3307_d_n5;
        locals.var_ct_dn6 = assign1880_e3307_d_n6;
        locals.var_ct_dn7 = assign1880_e3307_d_n7;
        locals.var_ct_dn8 = assign1880_e3307_d_n8;
        locals.var_ct_dn9 = assign1880_e3307_d_n9;
        locals.var_ct_dn12 = assign1880_e3307_d_n12;
        locals.var_ct_dn14 = assign1880_e3307_d_n14;
        locals.var_ct_dn15 = assign1880_e3307_d_n15;
        locals.var_ct_dn16 = assign1880_e3307_d_n16;
        locals.var_ct_dn17 = assign1880_e3307_d_n17;
        locals.var_ct_dn18 = assign1880_e3307_d_n18;
        locals.var_ct_dn19 = assign1880_e3307_d_n19;
        locals.var_ct_dn20 = assign1880_e3307_d_n20;
        locals.var_ct_dn21 = assign1880_e3307_d_n21;
        locals.var_ct_dn22 = assign1880_e3307_d_n22;
        locals.var_ct_rv = 0.0;

        let (assign1910_e3351, assign1910_e3351_d_n0, assign1910_e3351_d_n1, assign1910_e3351_d_n2, assign1910_e3351_d_n3, assign1910_e3351_d_n4, assign1910_e3351_d_n5, assign1910_e3351_d_n6, assign1910_e3351_d_n7, assign1910_e3351_d_n8, assign1910_e3351_d_n9, assign1910_e3351_d_n12, assign1910_e3351_d_n14, assign1910_e3351_d_n15, assign1910_e3351_d_n16, assign1910_e3351_d_n17, assign1910_e3351_d_n18, assign1910_e3351_d_n19, assign1910_e3351_d_n20, assign1910_e3351_d_n21, assign1910_e3351_d_n22,) = {
    if ((locals.var_guard357 != 0.0) && (!((((locals.var_guard353 != 0.0) || (locals.var_guard354 != 0.0)) || (locals.var_guard355 != 0.0)) || (locals.var_guard356 != 0.0)))) {
        let assign1910_e3349: f64 = ((nv0 - nv2)).abs();
        (assign1910_e3349, if (nv0 - nv2) >= 0.0 { 1.0 } else { (-1.0) }, 0.0, if (nv0 - nv2) >= 0.0 { -1.0 } else { 1.0 }, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn1, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn12, locals.var_t0_dn14, locals.var_t0_dn15, locals.var_t0_dn16, locals.var_t0_dn17, locals.var_t0_dn18, locals.var_t0_dn19, locals.var_t0_dn20, locals.var_t0_dn21, locals.var_t0_dn22,)
    }
};
        locals.var_t0 = assign1910_e3351;
        locals.var_t0_dn0 = assign1910_e3351_d_n0;
        locals.var_t0_dn1 = assign1910_e3351_d_n1;
        locals.var_t0_dn2 = assign1910_e3351_d_n2;
        locals.var_t0_dn3 = assign1910_e3351_d_n3;
        locals.var_t0_dn4 = assign1910_e3351_d_n4;
        locals.var_t0_dn5 = assign1910_e3351_d_n5;
        locals.var_t0_dn6 = assign1910_e3351_d_n6;
        locals.var_t0_dn7 = assign1910_e3351_d_n7;
        locals.var_t0_dn8 = assign1910_e3351_d_n8;
        locals.var_t0_dn9 = assign1910_e3351_d_n9;
        locals.var_t0_dn12 = assign1910_e3351_d_n12;
        locals.var_t0_dn14 = assign1910_e3351_d_n14;
        locals.var_t0_dn15 = assign1910_e3351_d_n15;
        locals.var_t0_dn16 = assign1910_e3351_d_n16;
        locals.var_t0_dn17 = assign1910_e3351_d_n17;
        locals.var_t0_dn18 = assign1910_e3351_d_n18;
        locals.var_t0_dn19 = assign1910_e3351_d_n19;
        locals.var_t0_dn20 = assign1910_e3351_d_n20;
        locals.var_t0_dn21 = assign1910_e3351_d_n21;
        locals.var_t0_dn22 = assign1910_e3351_d_n22;
        locals.var_t0_rv = 0.0;

        let (assign1930_e3387, assign1930_e3387_d_n0, assign1930_e3387_d_n1, assign1930_e3387_d_n2, assign1930_e3387_d_n3, assign1930_e3387_d_n4, assign1930_e3387_d_n5, assign1930_e3387_d_n6, assign1930_e3387_d_n7, assign1930_e3387_d_n8, assign1930_e3387_d_n9, assign1930_e3387_d_n12, assign1930_e3387_d_n14, assign1930_e3387_d_n15, assign1930_e3387_d_n16, assign1930_e3387_d_n17, assign1930_e3387_d_n18, assign1930_e3387_d_n19, assign1930_e3387_d_n20, assign1930_e3387_d_n21, assign1930_e3387_d_n22,) = {
    if ((locals.var_guard357 != 0.0) && (!((((locals.var_guard353 != 0.0) || (locals.var_guard354 != 0.0)) || (locals.var_guard355 != 0.0)) || (locals.var_guard356 != 0.0)))) {
        let assign1930_e3385: f64 = ((nv1 - nv2)).abs();
        (assign1930_e3385, 0.0, if (nv1 - nv2) >= 0.0 { 1.0 } else { (-1.0) }, if (nv1 - nv2) >= 0.0 { -1.0 } else { 1.0 }, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn1, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn12, locals.var_t1_dn14, locals.var_t1_dn15, locals.var_t1_dn16, locals.var_t1_dn17, locals.var_t1_dn18, locals.var_t1_dn19, locals.var_t1_dn20, locals.var_t1_dn21, locals.var_t1_dn22,)
    }
};
        locals.var_t1 = assign1930_e3387;
        locals.var_t1_dn0 = assign1930_e3387_d_n0;
        locals.var_t1_dn1 = assign1930_e3387_d_n1;
        locals.var_t1_dn2 = assign1930_e3387_d_n2;
        locals.var_t1_dn3 = assign1930_e3387_d_n3;
        locals.var_t1_dn4 = assign1930_e3387_d_n4;
        locals.var_t1_dn5 = assign1930_e3387_d_n5;
        locals.var_t1_dn6 = assign1930_e3387_d_n6;
        locals.var_t1_dn7 = assign1930_e3387_d_n7;
        locals.var_t1_dn8 = assign1930_e3387_d_n8;
        locals.var_t1_dn9 = assign1930_e3387_d_n9;
        locals.var_t1_dn12 = assign1930_e3387_d_n12;
        locals.var_t1_dn14 = assign1930_e3387_d_n14;
        locals.var_t1_dn15 = assign1930_e3387_d_n15;
        locals.var_t1_dn16 = assign1930_e3387_d_n16;
        locals.var_t1_dn17 = assign1930_e3387_d_n17;
        locals.var_t1_dn18 = assign1930_e3387_d_n18;
        locals.var_t1_dn19 = assign1930_e3387_d_n19;
        locals.var_t1_dn20 = assign1930_e3387_d_n20;
        locals.var_t1_dn21 = assign1930_e3387_d_n21;
        locals.var_t1_dn22 = assign1930_e3387_d_n22;
        locals.var_t1_rv = 0.0;

        let (assign1950_e3425, assign1950_e3425_d_n0, assign1950_e3425_d_n2, assign1950_e3425_d_n5, assign1950_e3425_d_n12,) = {
    if ((locals.var_guard357 != 0.0) && (!((((locals.var_guard353 != 0.0) || (locals.var_guard354 != 0.0)) || (locals.var_guard355 != 0.0)) || (locals.var_guard356 != 0.0)))) {
        let assign1950_e3422: f64 = ((nv0 - nv2)).abs();
        let assign1950_e3423: f64 = ((nv12 - 0.0) - assign1950_e3422);
        (assign1950_e3423, (-if (nv0 - nv2) >= 0.0 { 1.0 } else { (-1.0) }), (-if (nv0 - nv2) >= 0.0 { -1.0 } else { 1.0 }), 0.0, 1.0,)
    } else {
        (locals.var_vaux, locals.var_vaux_dn0, locals.var_vaux_dn2, locals.var_vaux_dn5, locals.var_vaux_dn12,)
    }
};
        locals.var_vaux = assign1950_e3425;
        locals.var_vaux_dn0 = assign1950_e3425_d_n0;
        locals.var_vaux_dn2 = assign1950_e3425_d_n2;
        locals.var_vaux_dn5 = assign1950_e3425_d_n5;
        locals.var_vaux_dn12 = assign1950_e3425_d_n12;
        locals.var_vaux_rv = 0.0;

        let (assign1960_e3457, assign1960_e3457_d_n0, assign1960_e3457_d_n2, assign1960_e3457_d_n5, assign1960_e3457_d_n12,) = {
    if ((locals.var_guard357 != 0.0) && (!((((locals.var_guard353 != 0.0) || (locals.var_guard354 != 0.0)) || (locals.var_guard355 != 0.0)) || (locals.var_guard356 != 0.0)))) {
        let assign1960_e3439: f64 = locals.var_vaux;
        let assign1960_e3442: f64 = locals.var_vaux;
        let assign1960_e3445: f64 = locals.var_vaux;
        let assign1960_e3446: f64 = (assign1960_e3442 * assign1960_e3445);
        let assign1960_e3449: f64 = (0.25 * 1e-30);
        let assign1960_e3451: f64 = (assign1960_e3449 * 1e-30);
        let assign1960_e3452: f64 = (assign1960_e3446 + assign1960_e3451);
        let assign1960_e3453: f64 = (assign1960_e3452).sqrt();
        let assign1960_e3454: f64 = (assign1960_e3439 + assign1960_e3453);
        let assign1960_e3455: f64 = (0.5 * assign1960_e3454);
        (assign1960_e3455, (0.5 * (locals.var_vaux_dn0 + (((locals.var_vaux_dn0 * assign1960_e3445) + (assign1960_e3442 * locals.var_vaux_dn0)) / (2.0 * assign1960_e3453)))), (0.5 * (locals.var_vaux_dn2 + (((locals.var_vaux_dn2 * assign1960_e3445) + (assign1960_e3442 * locals.var_vaux_dn2)) / (2.0 * assign1960_e3453)))), (0.5 * (locals.var_vaux_dn5 + (((locals.var_vaux_dn5 * assign1960_e3445) + (assign1960_e3442 * locals.var_vaux_dn5)) / (2.0 * assign1960_e3453)))), (0.5 * (locals.var_vaux_dn12 + (((locals.var_vaux_dn12 * assign1960_e3445) + (assign1960_e3442 * locals.var_vaux_dn12)) / (2.0 * assign1960_e3453)))),)
    } else {
        (locals.var_vaux, locals.var_vaux_dn0, locals.var_vaux_dn2, locals.var_vaux_dn5, locals.var_vaux_dn12,)
    }
};
        locals.var_vaux = assign1960_e3457;
        locals.var_vaux_dn0 = assign1960_e3457_d_n0;
        locals.var_vaux_dn2 = assign1960_e3457_d_n2;
        locals.var_vaux_dn5 = assign1960_e3457_d_n5;
        locals.var_vaux_dn12 = assign1960_e3457_d_n12;
        locals.var_vaux_rv = 0.0;

        let (assign1970_e3473, assign1970_e3473_d_n1, assign1970_e3473_d_n2, assign1970_e3473_d_n14,) = {
    if ((locals.var_guard357 != 0.0) && (!((((locals.var_guard353 != 0.0) || (locals.var_guard354 != 0.0)) || (locals.var_guard355 != 0.0)) || (locals.var_guard356 != 0.0)))) {
        let assign1970_e3470: f64 = ((nv1 - nv2)).abs();
        let assign1970_e3471: f64 = ((nv14 - 0.0) - assign1970_e3470);
        (assign1970_e3471, (-if (nv1 - nv2) >= 0.0 { 1.0 } else { (-1.0) }), (-if (nv1 - nv2) >= 0.0 { -1.0 } else { 1.0 }), 1.0,)
    } else {
        (locals.var_vauxg, locals.var_vauxg_dn1, locals.var_vauxg_dn2, locals.var_vauxg_dn14,)
    }
};
        locals.var_vauxg = assign1970_e3473;
        locals.var_vauxg_dn1 = assign1970_e3473_d_n1;
        locals.var_vauxg_dn2 = assign1970_e3473_d_n2;
        locals.var_vauxg_dn14 = assign1970_e3473_d_n14;
        locals.var_vauxg_rv = 0.0;

        let (assign1980_e3505, assign1980_e3505_d_n1, assign1980_e3505_d_n2, assign1980_e3505_d_n14,) = {
    if ((locals.var_guard357 != 0.0) && (!((((locals.var_guard353 != 0.0) || (locals.var_guard354 != 0.0)) || (locals.var_guard355 != 0.0)) || (locals.var_guard356 != 0.0)))) {
        let assign1980_e3487: f64 = locals.var_vauxg;
        let assign1980_e3490: f64 = locals.var_vauxg;
        let assign1980_e3493: f64 = locals.var_vauxg;
        let assign1980_e3494: f64 = (assign1980_e3490 * assign1980_e3493);
        let assign1980_e3497: f64 = (0.25 * 1e-30);
        let assign1980_e3499: f64 = (assign1980_e3497 * 1e-30);
        let assign1980_e3500: f64 = (assign1980_e3494 + assign1980_e3499);
        let assign1980_e3501: f64 = (assign1980_e3500).sqrt();
        let assign1980_e3502: f64 = (assign1980_e3487 + assign1980_e3501);
        let assign1980_e3503: f64 = (0.5 * assign1980_e3502);
        (assign1980_e3503, (0.5 * (locals.var_vauxg_dn1 + (((locals.var_vauxg_dn1 * assign1980_e3493) + (assign1980_e3490 * locals.var_vauxg_dn1)) / (2.0 * assign1980_e3501)))), (0.5 * (locals.var_vauxg_dn2 + (((locals.var_vauxg_dn2 * assign1980_e3493) + (assign1980_e3490 * locals.var_vauxg_dn2)) / (2.0 * assign1980_e3501)))), (0.5 * (locals.var_vauxg_dn14 + (((locals.var_vauxg_dn14 * assign1980_e3493) + (assign1980_e3490 * locals.var_vauxg_dn14)) / (2.0 * assign1980_e3501)))),)
    } else {
        (locals.var_vauxg, locals.var_vauxg_dn1, locals.var_vauxg_dn2, locals.var_vauxg_dn14,)
    }
};
        locals.var_vauxg = assign1980_e3505;
        locals.var_vauxg_dn1 = assign1980_e3505_d_n1;
        locals.var_vauxg_dn2 = assign1980_e3505_d_n2;
        locals.var_vauxg_dn14 = assign1980_e3505_d_n14;
        locals.var_vauxg_rv = 0.0;

        let (assign1990_e3520, assign1990_e3520_d_n0, assign1990_e3520_d_n1, assign1990_e3520_d_n2, assign1990_e3520_d_n3, assign1990_e3520_d_n4, assign1990_e3520_d_n5, assign1990_e3520_d_n6, assign1990_e3520_d_n7, assign1990_e3520_d_n8, assign1990_e3520_d_n9, assign1990_e3520_d_n12, assign1990_e3520_d_n14, assign1990_e3520_d_n15, assign1990_e3520_d_n16, assign1990_e3520_d_n17, assign1990_e3520_d_n18, assign1990_e3520_d_n19, assign1990_e3520_d_n20, assign1990_e3520_d_n21, assign1990_e3520_d_n22,) = {
    if ((locals.var_guard357 != 0.0) && (!((((locals.var_guard353 != 0.0) || (locals.var_guard354 != 0.0)) || (locals.var_guard355 != 0.0)) || (locals.var_guard356 != 0.0)))) {
        let assign1990_e3518: f64 = (locals.var_vaux * p.p89);
        (assign1990_e3518, (locals.var_vaux_dn0 * p.p89), 0.0, (locals.var_vaux_dn2 * p.p89), 0.0, 0.0, (locals.var_vaux_dn5 * p.p89), 0.0, 0.0, 0.0, 0.0, (locals.var_vaux_dn12 * p.p89), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn1, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn12, locals.var_t0_dn14, locals.var_t0_dn15, locals.var_t0_dn16, locals.var_t0_dn17, locals.var_t0_dn18, locals.var_t0_dn19, locals.var_t0_dn20, locals.var_t0_dn21, locals.var_t0_dn22,)
    }
};
        locals.var_t0 = assign1990_e3520;
        locals.var_t0_dn0 = assign1990_e3520_d_n0;
        locals.var_t0_dn1 = assign1990_e3520_d_n1;
        locals.var_t0_dn2 = assign1990_e3520_d_n2;
        locals.var_t0_dn3 = assign1990_e3520_d_n3;
        locals.var_t0_dn4 = assign1990_e3520_d_n4;
        locals.var_t0_dn5 = assign1990_e3520_d_n5;
        locals.var_t0_dn6 = assign1990_e3520_d_n6;
        locals.var_t0_dn7 = assign1990_e3520_d_n7;
        locals.var_t0_dn8 = assign1990_e3520_d_n8;
        locals.var_t0_dn9 = assign1990_e3520_d_n9;
        locals.var_t0_dn12 = assign1990_e3520_d_n12;
        locals.var_t0_dn14 = assign1990_e3520_d_n14;
        locals.var_t0_dn15 = assign1990_e3520_d_n15;
        locals.var_t0_dn16 = assign1990_e3520_d_n16;
        locals.var_t0_dn17 = assign1990_e3520_d_n17;
        locals.var_t0_dn18 = assign1990_e3520_d_n18;
        locals.var_t0_dn19 = assign1990_e3520_d_n19;
        locals.var_t0_dn20 = assign1990_e3520_d_n20;
        locals.var_t0_dn21 = assign1990_e3520_d_n21;
        locals.var_t0_dn22 = assign1990_e3520_d_n22;
        locals.var_t0_rv = 0.0;

        let (assign2000_e3540, assign2000_e3540_d_n0, assign2000_e3540_d_n1, assign2000_e3540_d_n2, assign2000_e3540_d_n3, assign2000_e3540_d_n4, assign2000_e3540_d_n5, assign2000_e3540_d_n6, assign2000_e3540_d_n7, assign2000_e3540_d_n8, assign2000_e3540_d_n9, assign2000_e3540_d_n12, assign2000_e3540_d_n14, assign2000_e3540_d_n15, assign2000_e3540_d_n16, assign2000_e3540_d_n17, assign2000_e3540_d_n18, assign2000_e3540_d_n19, assign2000_e3540_d_n20, assign2000_e3540_d_n21, assign2000_e3540_d_n22,) = {
    if ((locals.var_guard357 != 0.0) && (!((((locals.var_guard353 != 0.0) || (locals.var_guard354 != 0.0)) || (locals.var_guard355 != 0.0)) || (locals.var_guard356 != 0.0)))) {
        let assign2000_e3533: f64 = (locals.var_vaux * locals.var_vaux);
        let assign2000_e3536: f64 = (p.p89 * p.p89);
        let assign2000_e3537: f64 = (assign2000_e3533 + assign2000_e3536);
        let assign2000_e3538: f64 = (assign2000_e3537).sqrt();
        (assign2000_e3538, (((locals.var_vaux_dn0 * locals.var_vaux) + (locals.var_vaux * locals.var_vaux_dn0)) / (2.0 * assign2000_e3538)), 0.0, (((locals.var_vaux_dn2 * locals.var_vaux) + (locals.var_vaux * locals.var_vaux_dn2)) / (2.0 * assign2000_e3538)), 0.0, 0.0, (((locals.var_vaux_dn5 * locals.var_vaux) + (locals.var_vaux * locals.var_vaux_dn5)) / (2.0 * assign2000_e3538)), 0.0, 0.0, 0.0, 0.0, (((locals.var_vaux_dn12 * locals.var_vaux) + (locals.var_vaux * locals.var_vaux_dn12)) / (2.0 * assign2000_e3538)), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn1, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn12, locals.var_t1_dn14, locals.var_t1_dn15, locals.var_t1_dn16, locals.var_t1_dn17, locals.var_t1_dn18, locals.var_t1_dn19, locals.var_t1_dn20, locals.var_t1_dn21, locals.var_t1_dn22,)
    }
};
        locals.var_t1 = assign2000_e3540;
        locals.var_t1_dn0 = assign2000_e3540_d_n0;
        locals.var_t1_dn1 = assign2000_e3540_d_n1;
        locals.var_t1_dn2 = assign2000_e3540_d_n2;
        locals.var_t1_dn3 = assign2000_e3540_d_n3;
        locals.var_t1_dn4 = assign2000_e3540_d_n4;
        locals.var_t1_dn5 = assign2000_e3540_d_n5;
        locals.var_t1_dn6 = assign2000_e3540_d_n6;
        locals.var_t1_dn7 = assign2000_e3540_d_n7;
        locals.var_t1_dn8 = assign2000_e3540_d_n8;
        locals.var_t1_dn9 = assign2000_e3540_d_n9;
        locals.var_t1_dn12 = assign2000_e3540_d_n12;
        locals.var_t1_dn14 = assign2000_e3540_d_n14;
        locals.var_t1_dn15 = assign2000_e3540_d_n15;
        locals.var_t1_dn16 = assign2000_e3540_d_n16;
        locals.var_t1_dn17 = assign2000_e3540_d_n17;
        locals.var_t1_dn18 = assign2000_e3540_d_n18;
        locals.var_t1_dn19 = assign2000_e3540_d_n19;
        locals.var_t1_dn20 = assign2000_e3540_d_n20;
        locals.var_t1_dn21 = assign2000_e3540_d_n21;
        locals.var_t1_dn22 = assign2000_e3540_d_n22;
        locals.var_t1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_5(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign2010_e3560, assign2010_e3560_d_n0, assign2010_e3560_d_n1, assign2010_e3560_d_n2, assign2010_e3560_d_n3, assign2010_e3560_d_n4, assign2010_e3560_d_n5, assign2010_e3560_d_n6, assign2010_e3560_d_n7, assign2010_e3560_d_n8, assign2010_e3560_d_n9, assign2010_e3560_d_n12, assign2010_e3560_d_n14, assign2010_e3560_d_n15, assign2010_e3560_d_n16, assign2010_e3560_d_n17, assign2010_e3560_d_n18, assign2010_e3560_d_n19, assign2010_e3560_d_n20, assign2010_e3560_d_n21, assign2010_e3560_d_n22,) = {
    if ((locals.var_guard357 != 0.0) && (!((((locals.var_guard353 != 0.0) || (locals.var_guard354 != 0.0)) || (locals.var_guard355 != 0.0)) || (locals.var_guard356 != 0.0)))) {
        let assign2010_e3553: f64 = (p.p91 * p.p10);
        let assign2010_e3554: f64 = (assign2010_e3553).abs();
        let assign2010_e3557: f64 = (locals.var_t0 / locals.var_t1);
        let assign2010_e3558: f64 = (assign2010_e3554 * assign2010_e3557);
        (assign2010_e3558, (assign2010_e3554 * (((locals.var_t0_dn0 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn0)) / (locals.var_t1 * locals.var_t1))), (assign2010_e3554 * (((locals.var_t0_dn1 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn1)) / (locals.var_t1 * locals.var_t1))), (assign2010_e3554 * (((locals.var_t0_dn2 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn2)) / (locals.var_t1 * locals.var_t1))), (assign2010_e3554 * (((locals.var_t0_dn3 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn3)) / (locals.var_t1 * locals.var_t1))), (assign2010_e3554 * (((locals.var_t0_dn4 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn4)) / (locals.var_t1 * locals.var_t1))), (assign2010_e3554 * (((locals.var_t0_dn5 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn5)) / (locals.var_t1 * locals.var_t1))), (assign2010_e3554 * (((locals.var_t0_dn6 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn6)) / (locals.var_t1 * locals.var_t1))), (assign2010_e3554 * (((locals.var_t0_dn7 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn7)) / (locals.var_t1 * locals.var_t1))), (assign2010_e3554 * (((locals.var_t0_dn8 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn8)) / (locals.var_t1 * locals.var_t1))), (assign2010_e3554 * (((locals.var_t0_dn9 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn9)) / (locals.var_t1 * locals.var_t1))), (assign2010_e3554 * (((locals.var_t0_dn12 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn12)) / (locals.var_t1 * locals.var_t1))), (assign2010_e3554 * (((locals.var_t0_dn14 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn14)) / (locals.var_t1 * locals.var_t1))), (assign2010_e3554 * (((locals.var_t0_dn15 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn15)) / (locals.var_t1 * locals.var_t1))), (assign2010_e3554 * (((locals.var_t0_dn16 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn16)) / (locals.var_t1 * locals.var_t1))), (assign2010_e3554 * (((locals.var_t0_dn17 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn17)) / (locals.var_t1 * locals.var_t1))), (assign2010_e3554 * (((locals.var_t0_dn18 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn18)) / (locals.var_t1 * locals.var_t1))), (assign2010_e3554 * (((locals.var_t0_dn19 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn19)) / (locals.var_t1 * locals.var_t1))), (assign2010_e3554 * (((locals.var_t0_dn20 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn20)) / (locals.var_t1 * locals.var_t1))), (assign2010_e3554 * (((locals.var_t0_dn21 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn21)) / (locals.var_t1 * locals.var_t1))), (assign2010_e3554 * (((locals.var_t0_dn22 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn22)) / (locals.var_t1 * locals.var_t1))),)
    } else {
        (locals.var_voffdlag, locals.var_voffdlag_dn0, locals.var_voffdlag_dn1, locals.var_voffdlag_dn2, locals.var_voffdlag_dn3, locals.var_voffdlag_dn4, locals.var_voffdlag_dn5, locals.var_voffdlag_dn6, locals.var_voffdlag_dn7, locals.var_voffdlag_dn8, locals.var_voffdlag_dn9, locals.var_voffdlag_dn12, locals.var_voffdlag_dn14, locals.var_voffdlag_dn15, locals.var_voffdlag_dn16, locals.var_voffdlag_dn17, locals.var_voffdlag_dn18, locals.var_voffdlag_dn19, locals.var_voffdlag_dn20, locals.var_voffdlag_dn21, locals.var_voffdlag_dn22,)
    }
};
        locals.var_voffdlag = assign2010_e3560;
        locals.var_voffdlag_dn0 = assign2010_e3560_d_n0;
        locals.var_voffdlag_dn1 = assign2010_e3560_d_n1;
        locals.var_voffdlag_dn2 = assign2010_e3560_d_n2;
        locals.var_voffdlag_dn3 = assign2010_e3560_d_n3;
        locals.var_voffdlag_dn4 = assign2010_e3560_d_n4;
        locals.var_voffdlag_dn5 = assign2010_e3560_d_n5;
        locals.var_voffdlag_dn6 = assign2010_e3560_d_n6;
        locals.var_voffdlag_dn7 = assign2010_e3560_d_n7;
        locals.var_voffdlag_dn8 = assign2010_e3560_d_n8;
        locals.var_voffdlag_dn9 = assign2010_e3560_d_n9;
        locals.var_voffdlag_dn12 = assign2010_e3560_d_n12;
        locals.var_voffdlag_dn14 = assign2010_e3560_d_n14;
        locals.var_voffdlag_dn15 = assign2010_e3560_d_n15;
        locals.var_voffdlag_dn16 = assign2010_e3560_d_n16;
        locals.var_voffdlag_dn17 = assign2010_e3560_d_n17;
        locals.var_voffdlag_dn18 = assign2010_e3560_d_n18;
        locals.var_voffdlag_dn19 = assign2010_e3560_d_n19;
        locals.var_voffdlag_dn20 = assign2010_e3560_d_n20;
        locals.var_voffdlag_dn21 = assign2010_e3560_d_n21;
        locals.var_voffdlag_dn22 = assign2010_e3560_d_n22;
        locals.var_voffdlag_rv = 0.0;

        let (assign2020_e3575, assign2020_e3575_d_n0, assign2020_e3575_d_n1, assign2020_e3575_d_n2, assign2020_e3575_d_n3, assign2020_e3575_d_n4, assign2020_e3575_d_n5, assign2020_e3575_d_n6, assign2020_e3575_d_n7, assign2020_e3575_d_n8, assign2020_e3575_d_n9, assign2020_e3575_d_n12, assign2020_e3575_d_n14, assign2020_e3575_d_n15, assign2020_e3575_d_n16, assign2020_e3575_d_n17, assign2020_e3575_d_n18, assign2020_e3575_d_n19, assign2020_e3575_d_n20, assign2020_e3575_d_n21, assign2020_e3575_d_n22,) = {
    if ((locals.var_guard357 != 0.0) && (!((((locals.var_guard353 != 0.0) || (locals.var_guard354 != 0.0)) || (locals.var_guard355 != 0.0)) || (locals.var_guard356 != 0.0)))) {
        let assign2020_e3573: f64 = (locals.var_vauxg * p.p90);
        (assign2020_e3573, 0.0, (locals.var_vauxg_dn1 * p.p90), (locals.var_vauxg_dn2 * p.p90), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, (locals.var_vauxg_dn14 * p.p90), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn1, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn12, locals.var_t0_dn14, locals.var_t0_dn15, locals.var_t0_dn16, locals.var_t0_dn17, locals.var_t0_dn18, locals.var_t0_dn19, locals.var_t0_dn20, locals.var_t0_dn21, locals.var_t0_dn22,)
    }
};
        locals.var_t0 = assign2020_e3575;
        locals.var_t0_dn0 = assign2020_e3575_d_n0;
        locals.var_t0_dn1 = assign2020_e3575_d_n1;
        locals.var_t0_dn2 = assign2020_e3575_d_n2;
        locals.var_t0_dn3 = assign2020_e3575_d_n3;
        locals.var_t0_dn4 = assign2020_e3575_d_n4;
        locals.var_t0_dn5 = assign2020_e3575_d_n5;
        locals.var_t0_dn6 = assign2020_e3575_d_n6;
        locals.var_t0_dn7 = assign2020_e3575_d_n7;
        locals.var_t0_dn8 = assign2020_e3575_d_n8;
        locals.var_t0_dn9 = assign2020_e3575_d_n9;
        locals.var_t0_dn12 = assign2020_e3575_d_n12;
        locals.var_t0_dn14 = assign2020_e3575_d_n14;
        locals.var_t0_dn15 = assign2020_e3575_d_n15;
        locals.var_t0_dn16 = assign2020_e3575_d_n16;
        locals.var_t0_dn17 = assign2020_e3575_d_n17;
        locals.var_t0_dn18 = assign2020_e3575_d_n18;
        locals.var_t0_dn19 = assign2020_e3575_d_n19;
        locals.var_t0_dn20 = assign2020_e3575_d_n20;
        locals.var_t0_dn21 = assign2020_e3575_d_n21;
        locals.var_t0_dn22 = assign2020_e3575_d_n22;
        locals.var_t0_rv = 0.0;

        let (assign2030_e3595, assign2030_e3595_d_n0, assign2030_e3595_d_n1, assign2030_e3595_d_n2, assign2030_e3595_d_n3, assign2030_e3595_d_n4, assign2030_e3595_d_n5, assign2030_e3595_d_n6, assign2030_e3595_d_n7, assign2030_e3595_d_n8, assign2030_e3595_d_n9, assign2030_e3595_d_n12, assign2030_e3595_d_n14, assign2030_e3595_d_n15, assign2030_e3595_d_n16, assign2030_e3595_d_n17, assign2030_e3595_d_n18, assign2030_e3595_d_n19, assign2030_e3595_d_n20, assign2030_e3595_d_n21, assign2030_e3595_d_n22,) = {
    if ((locals.var_guard357 != 0.0) && (!((((locals.var_guard353 != 0.0) || (locals.var_guard354 != 0.0)) || (locals.var_guard355 != 0.0)) || (locals.var_guard356 != 0.0)))) {
        let assign2030_e3588: f64 = (locals.var_vauxg * locals.var_vauxg);
        let assign2030_e3591: f64 = (p.p90 * p.p90);
        let assign2030_e3592: f64 = (assign2030_e3588 + assign2030_e3591);
        let assign2030_e3593: f64 = (assign2030_e3592).sqrt();
        (assign2030_e3593, 0.0, (((locals.var_vauxg_dn1 * locals.var_vauxg) + (locals.var_vauxg * locals.var_vauxg_dn1)) / (2.0 * assign2030_e3593)), (((locals.var_vauxg_dn2 * locals.var_vauxg) + (locals.var_vauxg * locals.var_vauxg_dn2)) / (2.0 * assign2030_e3593)), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, (((locals.var_vauxg_dn14 * locals.var_vauxg) + (locals.var_vauxg * locals.var_vauxg_dn14)) / (2.0 * assign2030_e3593)), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn1, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn12, locals.var_t1_dn14, locals.var_t1_dn15, locals.var_t1_dn16, locals.var_t1_dn17, locals.var_t1_dn18, locals.var_t1_dn19, locals.var_t1_dn20, locals.var_t1_dn21, locals.var_t1_dn22,)
    }
};
        locals.var_t1 = assign2030_e3595;
        locals.var_t1_dn0 = assign2030_e3595_d_n0;
        locals.var_t1_dn1 = assign2030_e3595_d_n1;
        locals.var_t1_dn2 = assign2030_e3595_d_n2;
        locals.var_t1_dn3 = assign2030_e3595_d_n3;
        locals.var_t1_dn4 = assign2030_e3595_d_n4;
        locals.var_t1_dn5 = assign2030_e3595_d_n5;
        locals.var_t1_dn6 = assign2030_e3595_d_n6;
        locals.var_t1_dn7 = assign2030_e3595_d_n7;
        locals.var_t1_dn8 = assign2030_e3595_d_n8;
        locals.var_t1_dn9 = assign2030_e3595_d_n9;
        locals.var_t1_dn12 = assign2030_e3595_d_n12;
        locals.var_t1_dn14 = assign2030_e3595_d_n14;
        locals.var_t1_dn15 = assign2030_e3595_d_n15;
        locals.var_t1_dn16 = assign2030_e3595_d_n16;
        locals.var_t1_dn17 = assign2030_e3595_d_n17;
        locals.var_t1_dn18 = assign2030_e3595_d_n18;
        locals.var_t1_dn19 = assign2030_e3595_d_n19;
        locals.var_t1_dn20 = assign2030_e3595_d_n20;
        locals.var_t1_dn21 = assign2030_e3595_d_n21;
        locals.var_t1_dn22 = assign2030_e3595_d_n22;
        locals.var_t1_rv = 0.0;

        let (assign2040_e3615, assign2040_e3615_d_n0, assign2040_e3615_d_n1, assign2040_e3615_d_n2, assign2040_e3615_d_n3, assign2040_e3615_d_n4, assign2040_e3615_d_n5, assign2040_e3615_d_n6, assign2040_e3615_d_n7, assign2040_e3615_d_n8, assign2040_e3615_d_n9, assign2040_e3615_d_n12, assign2040_e3615_d_n14, assign2040_e3615_d_n15, assign2040_e3615_d_n16, assign2040_e3615_d_n17, assign2040_e3615_d_n18, assign2040_e3615_d_n19, assign2040_e3615_d_n20, assign2040_e3615_d_n21, assign2040_e3615_d_n22,) = {
    if ((locals.var_guard357 != 0.0) && (!((((locals.var_guard353 != 0.0) || (locals.var_guard354 != 0.0)) || (locals.var_guard355 != 0.0)) || (locals.var_guard356 != 0.0)))) {
        let assign2040_e3608: f64 = (p.p92 * p.p10);
        let assign2040_e3609: f64 = (assign2040_e3608).abs();
        let assign2040_e3612: f64 = (locals.var_t0 / locals.var_t1);
        let assign2040_e3613: f64 = (assign2040_e3609 * assign2040_e3612);
        (assign2040_e3613, (assign2040_e3609 * (((locals.var_t0_dn0 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn0)) / (locals.var_t1 * locals.var_t1))), (assign2040_e3609 * (((locals.var_t0_dn1 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn1)) / (locals.var_t1 * locals.var_t1))), (assign2040_e3609 * (((locals.var_t0_dn2 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn2)) / (locals.var_t1 * locals.var_t1))), (assign2040_e3609 * (((locals.var_t0_dn3 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn3)) / (locals.var_t1 * locals.var_t1))), (assign2040_e3609 * (((locals.var_t0_dn4 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn4)) / (locals.var_t1 * locals.var_t1))), (assign2040_e3609 * (((locals.var_t0_dn5 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn5)) / (locals.var_t1 * locals.var_t1))), (assign2040_e3609 * (((locals.var_t0_dn6 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn6)) / (locals.var_t1 * locals.var_t1))), (assign2040_e3609 * (((locals.var_t0_dn7 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn7)) / (locals.var_t1 * locals.var_t1))), (assign2040_e3609 * (((locals.var_t0_dn8 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn8)) / (locals.var_t1 * locals.var_t1))), (assign2040_e3609 * (((locals.var_t0_dn9 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn9)) / (locals.var_t1 * locals.var_t1))), (assign2040_e3609 * (((locals.var_t0_dn12 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn12)) / (locals.var_t1 * locals.var_t1))), (assign2040_e3609 * (((locals.var_t0_dn14 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn14)) / (locals.var_t1 * locals.var_t1))), (assign2040_e3609 * (((locals.var_t0_dn15 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn15)) / (locals.var_t1 * locals.var_t1))), (assign2040_e3609 * (((locals.var_t0_dn16 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn16)) / (locals.var_t1 * locals.var_t1))), (assign2040_e3609 * (((locals.var_t0_dn17 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn17)) / (locals.var_t1 * locals.var_t1))), (assign2040_e3609 * (((locals.var_t0_dn18 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn18)) / (locals.var_t1 * locals.var_t1))), (assign2040_e3609 * (((locals.var_t0_dn19 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn19)) / (locals.var_t1 * locals.var_t1))), (assign2040_e3609 * (((locals.var_t0_dn20 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn20)) / (locals.var_t1 * locals.var_t1))), (assign2040_e3609 * (((locals.var_t0_dn21 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn21)) / (locals.var_t1 * locals.var_t1))), (assign2040_e3609 * (((locals.var_t0_dn22 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn22)) / (locals.var_t1 * locals.var_t1))),)
    } else {
        (locals.var_voffglag, locals.var_voffglag_dn0, locals.var_voffglag_dn1, locals.var_voffglag_dn2, locals.var_voffglag_dn3, locals.var_voffglag_dn4, locals.var_voffglag_dn5, locals.var_voffglag_dn6, locals.var_voffglag_dn7, locals.var_voffglag_dn8, locals.var_voffglag_dn9, locals.var_voffglag_dn12, locals.var_voffglag_dn14, locals.var_voffglag_dn15, locals.var_voffglag_dn16, locals.var_voffglag_dn17, locals.var_voffglag_dn18, locals.var_voffglag_dn19, locals.var_voffglag_dn20, locals.var_voffglag_dn21, locals.var_voffglag_dn22,)
    }
};
        locals.var_voffglag = assign2040_e3615;
        locals.var_voffglag_dn0 = assign2040_e3615_d_n0;
        locals.var_voffglag_dn1 = assign2040_e3615_d_n1;
        locals.var_voffglag_dn2 = assign2040_e3615_d_n2;
        locals.var_voffglag_dn3 = assign2040_e3615_d_n3;
        locals.var_voffglag_dn4 = assign2040_e3615_d_n4;
        locals.var_voffglag_dn5 = assign2040_e3615_d_n5;
        locals.var_voffglag_dn6 = assign2040_e3615_d_n6;
        locals.var_voffglag_dn7 = assign2040_e3615_d_n7;
        locals.var_voffglag_dn8 = assign2040_e3615_d_n8;
        locals.var_voffglag_dn9 = assign2040_e3615_d_n9;
        locals.var_voffglag_dn12 = assign2040_e3615_d_n12;
        locals.var_voffglag_dn14 = assign2040_e3615_d_n14;
        locals.var_voffglag_dn15 = assign2040_e3615_d_n15;
        locals.var_voffglag_dn16 = assign2040_e3615_d_n16;
        locals.var_voffglag_dn17 = assign2040_e3615_d_n17;
        locals.var_voffglag_dn18 = assign2040_e3615_d_n18;
        locals.var_voffglag_dn19 = assign2040_e3615_d_n19;
        locals.var_voffglag_dn20 = assign2040_e3615_d_n20;
        locals.var_voffglag_dn21 = assign2040_e3615_d_n21;
        locals.var_voffglag_dn22 = assign2040_e3615_d_n22;
        locals.var_voffglag_rv = 0.0;

        let (assign2050_e3630, assign2050_e3630_d_n0, assign2050_e3630_d_n1, assign2050_e3630_d_n2, assign2050_e3630_d_n3, assign2050_e3630_d_n4, assign2050_e3630_d_n5, assign2050_e3630_d_n6, assign2050_e3630_d_n7, assign2050_e3630_d_n8, assign2050_e3630_d_n9, assign2050_e3630_d_n12, assign2050_e3630_d_n14, assign2050_e3630_d_n15, assign2050_e3630_d_n16, assign2050_e3630_d_n17, assign2050_e3630_d_n18, assign2050_e3630_d_n19, assign2050_e3630_d_n20, assign2050_e3630_d_n21, assign2050_e3630_d_n22,) = {
    if ((locals.var_guard357 != 0.0) && (!((((locals.var_guard353 != 0.0) || (locals.var_guard354 != 0.0)) || (locals.var_guard355 != 0.0)) || (locals.var_guard356 != 0.0)))) {
        let assign2050_e3628: f64 = (locals.var_vauxg * p.p90);
        (assign2050_e3628, 0.0, (locals.var_vauxg_dn1 * p.p90), (locals.var_vauxg_dn2 * p.p90), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, (locals.var_vauxg_dn14 * p.p90), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn1, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn12, locals.var_t0_dn14, locals.var_t0_dn15, locals.var_t0_dn16, locals.var_t0_dn17, locals.var_t0_dn18, locals.var_t0_dn19, locals.var_t0_dn20, locals.var_t0_dn21, locals.var_t0_dn22,)
    }
};
        locals.var_t0 = assign2050_e3630;
        locals.var_t0_dn0 = assign2050_e3630_d_n0;
        locals.var_t0_dn1 = assign2050_e3630_d_n1;
        locals.var_t0_dn2 = assign2050_e3630_d_n2;
        locals.var_t0_dn3 = assign2050_e3630_d_n3;
        locals.var_t0_dn4 = assign2050_e3630_d_n4;
        locals.var_t0_dn5 = assign2050_e3630_d_n5;
        locals.var_t0_dn6 = assign2050_e3630_d_n6;
        locals.var_t0_dn7 = assign2050_e3630_d_n7;
        locals.var_t0_dn8 = assign2050_e3630_d_n8;
        locals.var_t0_dn9 = assign2050_e3630_d_n9;
        locals.var_t0_dn12 = assign2050_e3630_d_n12;
        locals.var_t0_dn14 = assign2050_e3630_d_n14;
        locals.var_t0_dn15 = assign2050_e3630_d_n15;
        locals.var_t0_dn16 = assign2050_e3630_d_n16;
        locals.var_t0_dn17 = assign2050_e3630_d_n17;
        locals.var_t0_dn18 = assign2050_e3630_d_n18;
        locals.var_t0_dn19 = assign2050_e3630_d_n19;
        locals.var_t0_dn20 = assign2050_e3630_d_n20;
        locals.var_t0_dn21 = assign2050_e3630_d_n21;
        locals.var_t0_dn22 = assign2050_e3630_d_n22;
        locals.var_t0_rv = 0.0;

        let (assign2060_e3650, assign2060_e3650_d_n0, assign2060_e3650_d_n1, assign2060_e3650_d_n2, assign2060_e3650_d_n3, assign2060_e3650_d_n4, assign2060_e3650_d_n5, assign2060_e3650_d_n6, assign2060_e3650_d_n7, assign2060_e3650_d_n8, assign2060_e3650_d_n9, assign2060_e3650_d_n12, assign2060_e3650_d_n14, assign2060_e3650_d_n15, assign2060_e3650_d_n16, assign2060_e3650_d_n17, assign2060_e3650_d_n18, assign2060_e3650_d_n19, assign2060_e3650_d_n20, assign2060_e3650_d_n21, assign2060_e3650_d_n22,) = {
    if ((locals.var_guard357 != 0.0) && (!((((locals.var_guard353 != 0.0) || (locals.var_guard354 != 0.0)) || (locals.var_guard355 != 0.0)) || (locals.var_guard356 != 0.0)))) {
        let assign2060_e3643: f64 = (locals.var_vauxg * locals.var_vauxg);
        let assign2060_e3646: f64 = (p.p90 * p.p90);
        let assign2060_e3647: f64 = (assign2060_e3643 + assign2060_e3646);
        let assign2060_e3648: f64 = (assign2060_e3647).sqrt();
        (assign2060_e3648, 0.0, (((locals.var_vauxg_dn1 * locals.var_vauxg) + (locals.var_vauxg * locals.var_vauxg_dn1)) / (2.0 * assign2060_e3648)), (((locals.var_vauxg_dn2 * locals.var_vauxg) + (locals.var_vauxg * locals.var_vauxg_dn2)) / (2.0 * assign2060_e3648)), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, (((locals.var_vauxg_dn14 * locals.var_vauxg) + (locals.var_vauxg * locals.var_vauxg_dn14)) / (2.0 * assign2060_e3648)), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn1, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn12, locals.var_t1_dn14, locals.var_t1_dn15, locals.var_t1_dn16, locals.var_t1_dn17, locals.var_t1_dn18, locals.var_t1_dn19, locals.var_t1_dn20, locals.var_t1_dn21, locals.var_t1_dn22,)
    }
};
        locals.var_t1 = assign2060_e3650;
        locals.var_t1_dn0 = assign2060_e3650_d_n0;
        locals.var_t1_dn1 = assign2060_e3650_d_n1;
        locals.var_t1_dn2 = assign2060_e3650_d_n2;
        locals.var_t1_dn3 = assign2060_e3650_d_n3;
        locals.var_t1_dn4 = assign2060_e3650_d_n4;
        locals.var_t1_dn5 = assign2060_e3650_d_n5;
        locals.var_t1_dn6 = assign2060_e3650_d_n6;
        locals.var_t1_dn7 = assign2060_e3650_d_n7;
        locals.var_t1_dn8 = assign2060_e3650_d_n8;
        locals.var_t1_dn9 = assign2060_e3650_d_n9;
        locals.var_t1_dn12 = assign2060_e3650_d_n12;
        locals.var_t1_dn14 = assign2060_e3650_d_n14;
        locals.var_t1_dn15 = assign2060_e3650_d_n15;
        locals.var_t1_dn16 = assign2060_e3650_d_n16;
        locals.var_t1_dn17 = assign2060_e3650_d_n17;
        locals.var_t1_dn18 = assign2060_e3650_d_n18;
        locals.var_t1_dn19 = assign2060_e3650_d_n19;
        locals.var_t1_dn20 = assign2060_e3650_d_n20;
        locals.var_t1_dn21 = assign2060_e3650_d_n21;
        locals.var_t1_dn22 = assign2060_e3650_d_n22;
        locals.var_t1_rv = 0.0;

        let (assign2070_e3670, assign2070_e3670_d_n0, assign2070_e3670_d_n1, assign2070_e3670_d_n2, assign2070_e3670_d_n3, assign2070_e3670_d_n4, assign2070_e3670_d_n5, assign2070_e3670_d_n6, assign2070_e3670_d_n7, assign2070_e3670_d_n8, assign2070_e3670_d_n9, assign2070_e3670_d_n12, assign2070_e3670_d_n14, assign2070_e3670_d_n15, assign2070_e3670_d_n16, assign2070_e3670_d_n17, assign2070_e3670_d_n18, assign2070_e3670_d_n19, assign2070_e3670_d_n20, assign2070_e3670_d_n21, assign2070_e3670_d_n22,) = {
    if ((locals.var_guard357 != 0.0) && (!((((locals.var_guard353 != 0.0) || (locals.var_guard354 != 0.0)) || (locals.var_guard355 != 0.0)) || (locals.var_guard356 != 0.0)))) {
        let assign2070_e3663: f64 = (p.p93 * p.p13);
        let assign2070_e3664: f64 = (assign2070_e3663).abs();
        let assign2070_e3667: f64 = (locals.var_t0 / locals.var_t1);
        let assign2070_e3668: f64 = (assign2070_e3664 * assign2070_e3667);
        (assign2070_e3668, (assign2070_e3664 * (((locals.var_t0_dn0 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn0)) / (locals.var_t1 * locals.var_t1))), (assign2070_e3664 * (((locals.var_t0_dn1 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn1)) / (locals.var_t1 * locals.var_t1))), (assign2070_e3664 * (((locals.var_t0_dn2 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn2)) / (locals.var_t1 * locals.var_t1))), (assign2070_e3664 * (((locals.var_t0_dn3 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn3)) / (locals.var_t1 * locals.var_t1))), (assign2070_e3664 * (((locals.var_t0_dn4 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn4)) / (locals.var_t1 * locals.var_t1))), (assign2070_e3664 * (((locals.var_t0_dn5 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn5)) / (locals.var_t1 * locals.var_t1))), (assign2070_e3664 * (((locals.var_t0_dn6 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn6)) / (locals.var_t1 * locals.var_t1))), (assign2070_e3664 * (((locals.var_t0_dn7 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn7)) / (locals.var_t1 * locals.var_t1))), (assign2070_e3664 * (((locals.var_t0_dn8 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn8)) / (locals.var_t1 * locals.var_t1))), (assign2070_e3664 * (((locals.var_t0_dn9 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn9)) / (locals.var_t1 * locals.var_t1))), (assign2070_e3664 * (((locals.var_t0_dn12 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn12)) / (locals.var_t1 * locals.var_t1))), (assign2070_e3664 * (((locals.var_t0_dn14 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn14)) / (locals.var_t1 * locals.var_t1))), (assign2070_e3664 * (((locals.var_t0_dn15 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn15)) / (locals.var_t1 * locals.var_t1))), (assign2070_e3664 * (((locals.var_t0_dn16 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn16)) / (locals.var_t1 * locals.var_t1))), (assign2070_e3664 * (((locals.var_t0_dn17 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn17)) / (locals.var_t1 * locals.var_t1))), (assign2070_e3664 * (((locals.var_t0_dn18 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn18)) / (locals.var_t1 * locals.var_t1))), (assign2070_e3664 * (((locals.var_t0_dn19 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn19)) / (locals.var_t1 * locals.var_t1))), (assign2070_e3664 * (((locals.var_t0_dn20 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn20)) / (locals.var_t1 * locals.var_t1))), (assign2070_e3664 * (((locals.var_t0_dn21 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn21)) / (locals.var_t1 * locals.var_t1))), (assign2070_e3664 * (((locals.var_t0_dn22 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn22)) / (locals.var_t1 * locals.var_t1))),)
    } else {
        (locals.var_u0glag, locals.var_u0glag_dn0, locals.var_u0glag_dn1, locals.var_u0glag_dn2, locals.var_u0glag_dn3, locals.var_u0glag_dn4, locals.var_u0glag_dn5, locals.var_u0glag_dn6, locals.var_u0glag_dn7, locals.var_u0glag_dn8, locals.var_u0glag_dn9, locals.var_u0glag_dn12, locals.var_u0glag_dn14, locals.var_u0glag_dn15, locals.var_u0glag_dn16, locals.var_u0glag_dn17, locals.var_u0glag_dn18, locals.var_u0glag_dn19, locals.var_u0glag_dn20, locals.var_u0glag_dn21, locals.var_u0glag_dn22,)
    }
};
        locals.var_u0glag = assign2070_e3670;
        locals.var_u0glag_dn0 = assign2070_e3670_d_n0;
        locals.var_u0glag_dn1 = assign2070_e3670_d_n1;
        locals.var_u0glag_dn2 = assign2070_e3670_d_n2;
        locals.var_u0glag_dn3 = assign2070_e3670_d_n3;
        locals.var_u0glag_dn4 = assign2070_e3670_d_n4;
        locals.var_u0glag_dn5 = assign2070_e3670_d_n5;
        locals.var_u0glag_dn6 = assign2070_e3670_d_n6;
        locals.var_u0glag_dn7 = assign2070_e3670_d_n7;
        locals.var_u0glag_dn8 = assign2070_e3670_d_n8;
        locals.var_u0glag_dn9 = assign2070_e3670_d_n9;
        locals.var_u0glag_dn12 = assign2070_e3670_d_n12;
        locals.var_u0glag_dn14 = assign2070_e3670_d_n14;
        locals.var_u0glag_dn15 = assign2070_e3670_d_n15;
        locals.var_u0glag_dn16 = assign2070_e3670_d_n16;
        locals.var_u0glag_dn17 = assign2070_e3670_d_n17;
        locals.var_u0glag_dn18 = assign2070_e3670_d_n18;
        locals.var_u0glag_dn19 = assign2070_e3670_d_n19;
        locals.var_u0glag_dn20 = assign2070_e3670_d_n20;
        locals.var_u0glag_dn21 = assign2070_e3670_d_n21;
        locals.var_u0glag_dn22 = assign2070_e3670_d_n22;
        locals.var_u0glag_rv = 0.0;

        let (assign2080_e3685, assign2080_e3685_d_n0, assign2080_e3685_d_n1, assign2080_e3685_d_n2, assign2080_e3685_d_n3, assign2080_e3685_d_n4, assign2080_e3685_d_n5, assign2080_e3685_d_n6, assign2080_e3685_d_n7, assign2080_e3685_d_n8, assign2080_e3685_d_n9, assign2080_e3685_d_n12, assign2080_e3685_d_n14, assign2080_e3685_d_n15, assign2080_e3685_d_n16, assign2080_e3685_d_n17, assign2080_e3685_d_n18, assign2080_e3685_d_n19, assign2080_e3685_d_n20, assign2080_e3685_d_n21, assign2080_e3685_d_n22,) = {
    if ((locals.var_guard357 != 0.0) && (!((((locals.var_guard353 != 0.0) || (locals.var_guard354 != 0.0)) || (locals.var_guard355 != 0.0)) || (locals.var_guard356 != 0.0)))) {
        let assign2080_e3683: f64 = (locals.var_vauxg * p.p90);
        (assign2080_e3683, 0.0, (locals.var_vauxg_dn1 * p.p90), (locals.var_vauxg_dn2 * p.p90), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, (locals.var_vauxg_dn14 * p.p90), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn1, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn12, locals.var_t0_dn14, locals.var_t0_dn15, locals.var_t0_dn16, locals.var_t0_dn17, locals.var_t0_dn18, locals.var_t0_dn19, locals.var_t0_dn20, locals.var_t0_dn21, locals.var_t0_dn22,)
    }
};
        locals.var_t0 = assign2080_e3685;
        locals.var_t0_dn0 = assign2080_e3685_d_n0;
        locals.var_t0_dn1 = assign2080_e3685_d_n1;
        locals.var_t0_dn2 = assign2080_e3685_d_n2;
        locals.var_t0_dn3 = assign2080_e3685_d_n3;
        locals.var_t0_dn4 = assign2080_e3685_d_n4;
        locals.var_t0_dn5 = assign2080_e3685_d_n5;
        locals.var_t0_dn6 = assign2080_e3685_d_n6;
        locals.var_t0_dn7 = assign2080_e3685_d_n7;
        locals.var_t0_dn8 = assign2080_e3685_d_n8;
        locals.var_t0_dn9 = assign2080_e3685_d_n9;
        locals.var_t0_dn12 = assign2080_e3685_d_n12;
        locals.var_t0_dn14 = assign2080_e3685_d_n14;
        locals.var_t0_dn15 = assign2080_e3685_d_n15;
        locals.var_t0_dn16 = assign2080_e3685_d_n16;
        locals.var_t0_dn17 = assign2080_e3685_d_n17;
        locals.var_t0_dn18 = assign2080_e3685_d_n18;
        locals.var_t0_dn19 = assign2080_e3685_d_n19;
        locals.var_t0_dn20 = assign2080_e3685_d_n20;
        locals.var_t0_dn21 = assign2080_e3685_d_n21;
        locals.var_t0_dn22 = assign2080_e3685_d_n22;
        locals.var_t0_rv = 0.0;

        let (assign2090_e3705, assign2090_e3705_d_n0, assign2090_e3705_d_n1, assign2090_e3705_d_n2, assign2090_e3705_d_n3, assign2090_e3705_d_n4, assign2090_e3705_d_n5, assign2090_e3705_d_n6, assign2090_e3705_d_n7, assign2090_e3705_d_n8, assign2090_e3705_d_n9, assign2090_e3705_d_n12, assign2090_e3705_d_n14, assign2090_e3705_d_n15, assign2090_e3705_d_n16, assign2090_e3705_d_n17, assign2090_e3705_d_n18, assign2090_e3705_d_n19, assign2090_e3705_d_n20, assign2090_e3705_d_n21, assign2090_e3705_d_n22,) = {
    if ((locals.var_guard357 != 0.0) && (!((((locals.var_guard353 != 0.0) || (locals.var_guard354 != 0.0)) || (locals.var_guard355 != 0.0)) || (locals.var_guard356 != 0.0)))) {
        let assign2090_e3698: f64 = (locals.var_vauxg * locals.var_vauxg);
        let assign2090_e3701: f64 = (p.p90 * p.p90);
        let assign2090_e3702: f64 = (assign2090_e3698 + assign2090_e3701);
        let assign2090_e3703: f64 = (assign2090_e3702).sqrt();
        (assign2090_e3703, 0.0, (((locals.var_vauxg_dn1 * locals.var_vauxg) + (locals.var_vauxg * locals.var_vauxg_dn1)) / (2.0 * assign2090_e3703)), (((locals.var_vauxg_dn2 * locals.var_vauxg) + (locals.var_vauxg * locals.var_vauxg_dn2)) / (2.0 * assign2090_e3703)), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, (((locals.var_vauxg_dn14 * locals.var_vauxg) + (locals.var_vauxg * locals.var_vauxg_dn14)) / (2.0 * assign2090_e3703)), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn1, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn12, locals.var_t1_dn14, locals.var_t1_dn15, locals.var_t1_dn16, locals.var_t1_dn17, locals.var_t1_dn18, locals.var_t1_dn19, locals.var_t1_dn20, locals.var_t1_dn21, locals.var_t1_dn22,)
    }
};
        locals.var_t1 = assign2090_e3705;
        locals.var_t1_dn0 = assign2090_e3705_d_n0;
        locals.var_t1_dn1 = assign2090_e3705_d_n1;
        locals.var_t1_dn2 = assign2090_e3705_d_n2;
        locals.var_t1_dn3 = assign2090_e3705_d_n3;
        locals.var_t1_dn4 = assign2090_e3705_d_n4;
        locals.var_t1_dn5 = assign2090_e3705_d_n5;
        locals.var_t1_dn6 = assign2090_e3705_d_n6;
        locals.var_t1_dn7 = assign2090_e3705_d_n7;
        locals.var_t1_dn8 = assign2090_e3705_d_n8;
        locals.var_t1_dn9 = assign2090_e3705_d_n9;
        locals.var_t1_dn12 = assign2090_e3705_d_n12;
        locals.var_t1_dn14 = assign2090_e3705_d_n14;
        locals.var_t1_dn15 = assign2090_e3705_d_n15;
        locals.var_t1_dn16 = assign2090_e3705_d_n16;
        locals.var_t1_dn17 = assign2090_e3705_d_n17;
        locals.var_t1_dn18 = assign2090_e3705_d_n18;
        locals.var_t1_dn19 = assign2090_e3705_d_n19;
        locals.var_t1_dn20 = assign2090_e3705_d_n20;
        locals.var_t1_dn21 = assign2090_e3705_d_n21;
        locals.var_t1_dn22 = assign2090_e3705_d_n22;
        locals.var_t1_rv = 0.0;

        let (assign2100_e3725, assign2100_e3725_d_n0, assign2100_e3725_d_n1, assign2100_e3725_d_n2, assign2100_e3725_d_n3, assign2100_e3725_d_n4, assign2100_e3725_d_n5, assign2100_e3725_d_n6, assign2100_e3725_d_n7, assign2100_e3725_d_n8, assign2100_e3725_d_n9, assign2100_e3725_d_n12, assign2100_e3725_d_n14, assign2100_e3725_d_n15, assign2100_e3725_d_n16, assign2100_e3725_d_n17, assign2100_e3725_d_n18, assign2100_e3725_d_n19, assign2100_e3725_d_n20, assign2100_e3725_d_n21, assign2100_e3725_d_n22,) = {
    if ((locals.var_guard357 != 0.0) && (!((((locals.var_guard353 != 0.0) || (locals.var_guard354 != 0.0)) || (locals.var_guard355 != 0.0)) || (locals.var_guard356 != 0.0)))) {
        let assign2100_e3718: f64 = (p.p94 * p.p17);
        let assign2100_e3719: f64 = (assign2100_e3718).abs();
        let assign2100_e3722: f64 = (locals.var_t0 / locals.var_t1);
        let assign2100_e3723: f64 = (assign2100_e3719 * assign2100_e3722);
        (assign2100_e3723, (assign2100_e3719 * (((locals.var_t0_dn0 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn0)) / (locals.var_t1 * locals.var_t1))), (assign2100_e3719 * (((locals.var_t0_dn1 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn1)) / (locals.var_t1 * locals.var_t1))), (assign2100_e3719 * (((locals.var_t0_dn2 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn2)) / (locals.var_t1 * locals.var_t1))), (assign2100_e3719 * (((locals.var_t0_dn3 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn3)) / (locals.var_t1 * locals.var_t1))), (assign2100_e3719 * (((locals.var_t0_dn4 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn4)) / (locals.var_t1 * locals.var_t1))), (assign2100_e3719 * (((locals.var_t0_dn5 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn5)) / (locals.var_t1 * locals.var_t1))), (assign2100_e3719 * (((locals.var_t0_dn6 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn6)) / (locals.var_t1 * locals.var_t1))), (assign2100_e3719 * (((locals.var_t0_dn7 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn7)) / (locals.var_t1 * locals.var_t1))), (assign2100_e3719 * (((locals.var_t0_dn8 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn8)) / (locals.var_t1 * locals.var_t1))), (assign2100_e3719 * (((locals.var_t0_dn9 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn9)) / (locals.var_t1 * locals.var_t1))), (assign2100_e3719 * (((locals.var_t0_dn12 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn12)) / (locals.var_t1 * locals.var_t1))), (assign2100_e3719 * (((locals.var_t0_dn14 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn14)) / (locals.var_t1 * locals.var_t1))), (assign2100_e3719 * (((locals.var_t0_dn15 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn15)) / (locals.var_t1 * locals.var_t1))), (assign2100_e3719 * (((locals.var_t0_dn16 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn16)) / (locals.var_t1 * locals.var_t1))), (assign2100_e3719 * (((locals.var_t0_dn17 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn17)) / (locals.var_t1 * locals.var_t1))), (assign2100_e3719 * (((locals.var_t0_dn18 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn18)) / (locals.var_t1 * locals.var_t1))), (assign2100_e3719 * (((locals.var_t0_dn19 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn19)) / (locals.var_t1 * locals.var_t1))), (assign2100_e3719 * (((locals.var_t0_dn20 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn20)) / (locals.var_t1 * locals.var_t1))), (assign2100_e3719 * (((locals.var_t0_dn21 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn21)) / (locals.var_t1 * locals.var_t1))), (assign2100_e3719 * (((locals.var_t0_dn22 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn22)) / (locals.var_t1 * locals.var_t1))),)
    } else {
        (locals.var_vsatglag, locals.var_vsatglag_dn0, locals.var_vsatglag_dn1, locals.var_vsatglag_dn2, locals.var_vsatglag_dn3, locals.var_vsatglag_dn4, locals.var_vsatglag_dn5, locals.var_vsatglag_dn6, locals.var_vsatglag_dn7, locals.var_vsatglag_dn8, locals.var_vsatglag_dn9, locals.var_vsatglag_dn12, locals.var_vsatglag_dn14, locals.var_vsatglag_dn15, locals.var_vsatglag_dn16, locals.var_vsatglag_dn17, locals.var_vsatglag_dn18, locals.var_vsatglag_dn19, locals.var_vsatglag_dn20, locals.var_vsatglag_dn21, locals.var_vsatglag_dn22,)
    }
};
        locals.var_vsatglag = assign2100_e3725;
        locals.var_vsatglag_dn0 = assign2100_e3725_d_n0;
        locals.var_vsatglag_dn1 = assign2100_e3725_d_n1;
        locals.var_vsatglag_dn2 = assign2100_e3725_d_n2;
        locals.var_vsatglag_dn3 = assign2100_e3725_d_n3;
        locals.var_vsatglag_dn4 = assign2100_e3725_d_n4;
        locals.var_vsatglag_dn5 = assign2100_e3725_d_n5;
        locals.var_vsatglag_dn6 = assign2100_e3725_d_n6;
        locals.var_vsatglag_dn7 = assign2100_e3725_d_n7;
        locals.var_vsatglag_dn8 = assign2100_e3725_d_n8;
        locals.var_vsatglag_dn9 = assign2100_e3725_d_n9;
        locals.var_vsatglag_dn12 = assign2100_e3725_d_n12;
        locals.var_vsatglag_dn14 = assign2100_e3725_d_n14;
        locals.var_vsatglag_dn15 = assign2100_e3725_d_n15;
        locals.var_vsatglag_dn16 = assign2100_e3725_d_n16;
        locals.var_vsatglag_dn17 = assign2100_e3725_d_n17;
        locals.var_vsatglag_dn18 = assign2100_e3725_d_n18;
        locals.var_vsatglag_dn19 = assign2100_e3725_d_n19;
        locals.var_vsatglag_dn20 = assign2100_e3725_d_n20;
        locals.var_vsatglag_dn21 = assign2100_e3725_d_n21;
        locals.var_vsatglag_dn22 = assign2100_e3725_d_n22;
        locals.var_vsatglag_rv = 0.0;

        let (assign2110_e3740, assign2110_e3740_d_n0, assign2110_e3740_d_n1, assign2110_e3740_d_n2, assign2110_e3740_d_n3, assign2110_e3740_d_n4, assign2110_e3740_d_n5, assign2110_e3740_d_n6, assign2110_e3740_d_n7, assign2110_e3740_d_n8, assign2110_e3740_d_n9, assign2110_e3740_d_n12, assign2110_e3740_d_n14, assign2110_e3740_d_n15, assign2110_e3740_d_n16, assign2110_e3740_d_n17, assign2110_e3740_d_n18, assign2110_e3740_d_n19, assign2110_e3740_d_n20, assign2110_e3740_d_n21, assign2110_e3740_d_n22,) = {
    if ((locals.var_guard357 != 0.0) && (!((((locals.var_guard353 != 0.0) || (locals.var_guard354 != 0.0)) || (locals.var_guard355 != 0.0)) || (locals.var_guard356 != 0.0)))) {
        let assign2110_e3738: f64 = (locals.var_vaux * p.p89);
        (assign2110_e3738, (locals.var_vaux_dn0 * p.p89), 0.0, (locals.var_vaux_dn2 * p.p89), 0.0, 0.0, (locals.var_vaux_dn5 * p.p89), 0.0, 0.0, 0.0, 0.0, (locals.var_vaux_dn12 * p.p89), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn1, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn12, locals.var_t0_dn14, locals.var_t0_dn15, locals.var_t0_dn16, locals.var_t0_dn17, locals.var_t0_dn18, locals.var_t0_dn19, locals.var_t0_dn20, locals.var_t0_dn21, locals.var_t0_dn22,)
    }
};
        locals.var_t0 = assign2110_e3740;
        locals.var_t0_dn0 = assign2110_e3740_d_n0;
        locals.var_t0_dn1 = assign2110_e3740_d_n1;
        locals.var_t0_dn2 = assign2110_e3740_d_n2;
        locals.var_t0_dn3 = assign2110_e3740_d_n3;
        locals.var_t0_dn4 = assign2110_e3740_d_n4;
        locals.var_t0_dn5 = assign2110_e3740_d_n5;
        locals.var_t0_dn6 = assign2110_e3740_d_n6;
        locals.var_t0_dn7 = assign2110_e3740_d_n7;
        locals.var_t0_dn8 = assign2110_e3740_d_n8;
        locals.var_t0_dn9 = assign2110_e3740_d_n9;
        locals.var_t0_dn12 = assign2110_e3740_d_n12;
        locals.var_t0_dn14 = assign2110_e3740_d_n14;
        locals.var_t0_dn15 = assign2110_e3740_d_n15;
        locals.var_t0_dn16 = assign2110_e3740_d_n16;
        locals.var_t0_dn17 = assign2110_e3740_d_n17;
        locals.var_t0_dn18 = assign2110_e3740_d_n18;
        locals.var_t0_dn19 = assign2110_e3740_d_n19;
        locals.var_t0_dn20 = assign2110_e3740_d_n20;
        locals.var_t0_dn21 = assign2110_e3740_d_n21;
        locals.var_t0_dn22 = assign2110_e3740_d_n22;
        locals.var_t0_rv = 0.0;

        let (assign2120_e3760, assign2120_e3760_d_n0, assign2120_e3760_d_n1, assign2120_e3760_d_n2, assign2120_e3760_d_n3, assign2120_e3760_d_n4, assign2120_e3760_d_n5, assign2120_e3760_d_n6, assign2120_e3760_d_n7, assign2120_e3760_d_n8, assign2120_e3760_d_n9, assign2120_e3760_d_n12, assign2120_e3760_d_n14, assign2120_e3760_d_n15, assign2120_e3760_d_n16, assign2120_e3760_d_n17, assign2120_e3760_d_n18, assign2120_e3760_d_n19, assign2120_e3760_d_n20, assign2120_e3760_d_n21, assign2120_e3760_d_n22,) = {
    if ((locals.var_guard357 != 0.0) && (!((((locals.var_guard353 != 0.0) || (locals.var_guard354 != 0.0)) || (locals.var_guard355 != 0.0)) || (locals.var_guard356 != 0.0)))) {
        let assign2120_e3753: f64 = (locals.var_vaux * locals.var_vaux);
        let assign2120_e3756: f64 = (p.p89 * p.p89);
        let assign2120_e3757: f64 = (assign2120_e3753 + assign2120_e3756);
        let assign2120_e3758: f64 = (assign2120_e3757).sqrt();
        (assign2120_e3758, (((locals.var_vaux_dn0 * locals.var_vaux) + (locals.var_vaux * locals.var_vaux_dn0)) / (2.0 * assign2120_e3758)), 0.0, (((locals.var_vaux_dn2 * locals.var_vaux) + (locals.var_vaux * locals.var_vaux_dn2)) / (2.0 * assign2120_e3758)), 0.0, 0.0, (((locals.var_vaux_dn5 * locals.var_vaux) + (locals.var_vaux * locals.var_vaux_dn5)) / (2.0 * assign2120_e3758)), 0.0, 0.0, 0.0, 0.0, (((locals.var_vaux_dn12 * locals.var_vaux) + (locals.var_vaux * locals.var_vaux_dn12)) / (2.0 * assign2120_e3758)), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn1, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn12, locals.var_t1_dn14, locals.var_t1_dn15, locals.var_t1_dn16, locals.var_t1_dn17, locals.var_t1_dn18, locals.var_t1_dn19, locals.var_t1_dn20, locals.var_t1_dn21, locals.var_t1_dn22,)
    }
};
        locals.var_t1 = assign2120_e3760;
        locals.var_t1_dn0 = assign2120_e3760_d_n0;
        locals.var_t1_dn1 = assign2120_e3760_d_n1;
        locals.var_t1_dn2 = assign2120_e3760_d_n2;
        locals.var_t1_dn3 = assign2120_e3760_d_n3;
        locals.var_t1_dn4 = assign2120_e3760_d_n4;
        locals.var_t1_dn5 = assign2120_e3760_d_n5;
        locals.var_t1_dn6 = assign2120_e3760_d_n6;
        locals.var_t1_dn7 = assign2120_e3760_d_n7;
        locals.var_t1_dn8 = assign2120_e3760_d_n8;
        locals.var_t1_dn9 = assign2120_e3760_d_n9;
        locals.var_t1_dn12 = assign2120_e3760_d_n12;
        locals.var_t1_dn14 = assign2120_e3760_d_n14;
        locals.var_t1_dn15 = assign2120_e3760_d_n15;
        locals.var_t1_dn16 = assign2120_e3760_d_n16;
        locals.var_t1_dn17 = assign2120_e3760_d_n17;
        locals.var_t1_dn18 = assign2120_e3760_d_n18;
        locals.var_t1_dn19 = assign2120_e3760_d_n19;
        locals.var_t1_dn20 = assign2120_e3760_d_n20;
        locals.var_t1_dn21 = assign2120_e3760_d_n21;
        locals.var_t1_dn22 = assign2120_e3760_d_n22;
        locals.var_t1_rv = 0.0;

        let (assign2130_e3780, assign2130_e3780_d_n0, assign2130_e3780_d_n1, assign2130_e3780_d_n2, assign2130_e3780_d_n3, assign2130_e3780_d_n4, assign2130_e3780_d_n5, assign2130_e3780_d_n6, assign2130_e3780_d_n7, assign2130_e3780_d_n8, assign2130_e3780_d_n9, assign2130_e3780_d_n12, assign2130_e3780_d_n14, assign2130_e3780_d_n15, assign2130_e3780_d_n16, assign2130_e3780_d_n17, assign2130_e3780_d_n18, assign2130_e3780_d_n19, assign2130_e3780_d_n20, assign2130_e3780_d_n21, assign2130_e3780_d_n22,) = {
    if ((locals.var_guard357 != 0.0) && (!((((locals.var_guard353 != 0.0) || (locals.var_guard354 != 0.0)) || (locals.var_guard355 != 0.0)) || (locals.var_guard356 != 0.0)))) {
        let assign2130_e3773: f64 = (p.p95 * p.p36);
        let assign2130_e3774: f64 = (assign2130_e3773).abs();
        let assign2130_e3777: f64 = (locals.var_t0 / locals.var_t1);
        let assign2130_e3778: f64 = (assign2130_e3774 * assign2130_e3777);
        (assign2130_e3778, (assign2130_e3774 * (((locals.var_t0_dn0 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn0)) / (locals.var_t1 * locals.var_t1))), (assign2130_e3774 * (((locals.var_t0_dn1 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn1)) / (locals.var_t1 * locals.var_t1))), (assign2130_e3774 * (((locals.var_t0_dn2 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn2)) / (locals.var_t1 * locals.var_t1))), (assign2130_e3774 * (((locals.var_t0_dn3 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn3)) / (locals.var_t1 * locals.var_t1))), (assign2130_e3774 * (((locals.var_t0_dn4 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn4)) / (locals.var_t1 * locals.var_t1))), (assign2130_e3774 * (((locals.var_t0_dn5 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn5)) / (locals.var_t1 * locals.var_t1))), (assign2130_e3774 * (((locals.var_t0_dn6 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn6)) / (locals.var_t1 * locals.var_t1))), (assign2130_e3774 * (((locals.var_t0_dn7 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn7)) / (locals.var_t1 * locals.var_t1))), (assign2130_e3774 * (((locals.var_t0_dn8 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn8)) / (locals.var_t1 * locals.var_t1))), (assign2130_e3774 * (((locals.var_t0_dn9 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn9)) / (locals.var_t1 * locals.var_t1))), (assign2130_e3774 * (((locals.var_t0_dn12 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn12)) / (locals.var_t1 * locals.var_t1))), (assign2130_e3774 * (((locals.var_t0_dn14 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn14)) / (locals.var_t1 * locals.var_t1))), (assign2130_e3774 * (((locals.var_t0_dn15 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn15)) / (locals.var_t1 * locals.var_t1))), (assign2130_e3774 * (((locals.var_t0_dn16 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn16)) / (locals.var_t1 * locals.var_t1))), (assign2130_e3774 * (((locals.var_t0_dn17 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn17)) / (locals.var_t1 * locals.var_t1))), (assign2130_e3774 * (((locals.var_t0_dn18 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn18)) / (locals.var_t1 * locals.var_t1))), (assign2130_e3774 * (((locals.var_t0_dn19 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn19)) / (locals.var_t1 * locals.var_t1))), (assign2130_e3774 * (((locals.var_t0_dn20 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn20)) / (locals.var_t1 * locals.var_t1))), (assign2130_e3774 * (((locals.var_t0_dn21 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn21)) / (locals.var_t1 * locals.var_t1))), (assign2130_e3774 * (((locals.var_t0_dn22 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn22)) / (locals.var_t1 * locals.var_t1))),)
    } else {
        (locals.var_ns0sdlag, locals.var_ns0sdlag_dn0, locals.var_ns0sdlag_dn1, locals.var_ns0sdlag_dn2, locals.var_ns0sdlag_dn3, locals.var_ns0sdlag_dn4, locals.var_ns0sdlag_dn5, locals.var_ns0sdlag_dn6, locals.var_ns0sdlag_dn7, locals.var_ns0sdlag_dn8, locals.var_ns0sdlag_dn9, locals.var_ns0sdlag_dn12, locals.var_ns0sdlag_dn14, locals.var_ns0sdlag_dn15, locals.var_ns0sdlag_dn16, locals.var_ns0sdlag_dn17, locals.var_ns0sdlag_dn18, locals.var_ns0sdlag_dn19, locals.var_ns0sdlag_dn20, locals.var_ns0sdlag_dn21, locals.var_ns0sdlag_dn22,)
    }
};
        locals.var_ns0sdlag = assign2130_e3780;
        locals.var_ns0sdlag_dn0 = assign2130_e3780_d_n0;
        locals.var_ns0sdlag_dn1 = assign2130_e3780_d_n1;
        locals.var_ns0sdlag_dn2 = assign2130_e3780_d_n2;
        locals.var_ns0sdlag_dn3 = assign2130_e3780_d_n3;
        locals.var_ns0sdlag_dn4 = assign2130_e3780_d_n4;
        locals.var_ns0sdlag_dn5 = assign2130_e3780_d_n5;
        locals.var_ns0sdlag_dn6 = assign2130_e3780_d_n6;
        locals.var_ns0sdlag_dn7 = assign2130_e3780_d_n7;
        locals.var_ns0sdlag_dn8 = assign2130_e3780_d_n8;
        locals.var_ns0sdlag_dn9 = assign2130_e3780_d_n9;
        locals.var_ns0sdlag_dn12 = assign2130_e3780_d_n12;
        locals.var_ns0sdlag_dn14 = assign2130_e3780_d_n14;
        locals.var_ns0sdlag_dn15 = assign2130_e3780_d_n15;
        locals.var_ns0sdlag_dn16 = assign2130_e3780_d_n16;
        locals.var_ns0sdlag_dn17 = assign2130_e3780_d_n17;
        locals.var_ns0sdlag_dn18 = assign2130_e3780_d_n18;
        locals.var_ns0sdlag_dn19 = assign2130_e3780_d_n19;
        locals.var_ns0sdlag_dn20 = assign2130_e3780_d_n20;
        locals.var_ns0sdlag_dn21 = assign2130_e3780_d_n21;
        locals.var_ns0sdlag_dn22 = assign2130_e3780_d_n22;
        locals.var_ns0sdlag_rv = 0.0;

        let (assign2140_e3795, assign2140_e3795_d_n0, assign2140_e3795_d_n1, assign2140_e3795_d_n2, assign2140_e3795_d_n3, assign2140_e3795_d_n4, assign2140_e3795_d_n5, assign2140_e3795_d_n6, assign2140_e3795_d_n7, assign2140_e3795_d_n8, assign2140_e3795_d_n9, assign2140_e3795_d_n12, assign2140_e3795_d_n14, assign2140_e3795_d_n15, assign2140_e3795_d_n16, assign2140_e3795_d_n17, assign2140_e3795_d_n18, assign2140_e3795_d_n19, assign2140_e3795_d_n20, assign2140_e3795_d_n21, assign2140_e3795_d_n22,) = {
    if ((locals.var_guard357 != 0.0) && (!((((locals.var_guard353 != 0.0) || (locals.var_guard354 != 0.0)) || (locals.var_guard355 != 0.0)) || (locals.var_guard356 != 0.0)))) {
        let assign2140_e3793: f64 = (locals.var_vaux * p.p89);
        (assign2140_e3793, (locals.var_vaux_dn0 * p.p89), 0.0, (locals.var_vaux_dn2 * p.p89), 0.0, 0.0, (locals.var_vaux_dn5 * p.p89), 0.0, 0.0, 0.0, 0.0, (locals.var_vaux_dn12 * p.p89), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn1, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn12, locals.var_t0_dn14, locals.var_t0_dn15, locals.var_t0_dn16, locals.var_t0_dn17, locals.var_t0_dn18, locals.var_t0_dn19, locals.var_t0_dn20, locals.var_t0_dn21, locals.var_t0_dn22,)
    }
};
        locals.var_t0 = assign2140_e3795;
        locals.var_t0_dn0 = assign2140_e3795_d_n0;
        locals.var_t0_dn1 = assign2140_e3795_d_n1;
        locals.var_t0_dn2 = assign2140_e3795_d_n2;
        locals.var_t0_dn3 = assign2140_e3795_d_n3;
        locals.var_t0_dn4 = assign2140_e3795_d_n4;
        locals.var_t0_dn5 = assign2140_e3795_d_n5;
        locals.var_t0_dn6 = assign2140_e3795_d_n6;
        locals.var_t0_dn7 = assign2140_e3795_d_n7;
        locals.var_t0_dn8 = assign2140_e3795_d_n8;
        locals.var_t0_dn9 = assign2140_e3795_d_n9;
        locals.var_t0_dn12 = assign2140_e3795_d_n12;
        locals.var_t0_dn14 = assign2140_e3795_d_n14;
        locals.var_t0_dn15 = assign2140_e3795_d_n15;
        locals.var_t0_dn16 = assign2140_e3795_d_n16;
        locals.var_t0_dn17 = assign2140_e3795_d_n17;
        locals.var_t0_dn18 = assign2140_e3795_d_n18;
        locals.var_t0_dn19 = assign2140_e3795_d_n19;
        locals.var_t0_dn20 = assign2140_e3795_d_n20;
        locals.var_t0_dn21 = assign2140_e3795_d_n21;
        locals.var_t0_dn22 = assign2140_e3795_d_n22;
        locals.var_t0_rv = 0.0;

        let (assign2150_e3815, assign2150_e3815_d_n0, assign2150_e3815_d_n1, assign2150_e3815_d_n2, assign2150_e3815_d_n3, assign2150_e3815_d_n4, assign2150_e3815_d_n5, assign2150_e3815_d_n6, assign2150_e3815_d_n7, assign2150_e3815_d_n8, assign2150_e3815_d_n9, assign2150_e3815_d_n12, assign2150_e3815_d_n14, assign2150_e3815_d_n15, assign2150_e3815_d_n16, assign2150_e3815_d_n17, assign2150_e3815_d_n18, assign2150_e3815_d_n19, assign2150_e3815_d_n20, assign2150_e3815_d_n21, assign2150_e3815_d_n22,) = {
    if ((locals.var_guard357 != 0.0) && (!((((locals.var_guard353 != 0.0) || (locals.var_guard354 != 0.0)) || (locals.var_guard355 != 0.0)) || (locals.var_guard356 != 0.0)))) {
        let assign2150_e3808: f64 = (locals.var_vaux * locals.var_vaux);
        let assign2150_e3811: f64 = (p.p89 * p.p89);
        let assign2150_e3812: f64 = (assign2150_e3808 + assign2150_e3811);
        let assign2150_e3813: f64 = (assign2150_e3812).sqrt();
        (assign2150_e3813, (((locals.var_vaux_dn0 * locals.var_vaux) + (locals.var_vaux * locals.var_vaux_dn0)) / (2.0 * assign2150_e3813)), 0.0, (((locals.var_vaux_dn2 * locals.var_vaux) + (locals.var_vaux * locals.var_vaux_dn2)) / (2.0 * assign2150_e3813)), 0.0, 0.0, (((locals.var_vaux_dn5 * locals.var_vaux) + (locals.var_vaux * locals.var_vaux_dn5)) / (2.0 * assign2150_e3813)), 0.0, 0.0, 0.0, 0.0, (((locals.var_vaux_dn12 * locals.var_vaux) + (locals.var_vaux * locals.var_vaux_dn12)) / (2.0 * assign2150_e3813)), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn1, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn12, locals.var_t1_dn14, locals.var_t1_dn15, locals.var_t1_dn16, locals.var_t1_dn17, locals.var_t1_dn18, locals.var_t1_dn19, locals.var_t1_dn20, locals.var_t1_dn21, locals.var_t1_dn22,)
    }
};
        locals.var_t1 = assign2150_e3815;
        locals.var_t1_dn0 = assign2150_e3815_d_n0;
        locals.var_t1_dn1 = assign2150_e3815_d_n1;
        locals.var_t1_dn2 = assign2150_e3815_d_n2;
        locals.var_t1_dn3 = assign2150_e3815_d_n3;
        locals.var_t1_dn4 = assign2150_e3815_d_n4;
        locals.var_t1_dn5 = assign2150_e3815_d_n5;
        locals.var_t1_dn6 = assign2150_e3815_d_n6;
        locals.var_t1_dn7 = assign2150_e3815_d_n7;
        locals.var_t1_dn8 = assign2150_e3815_d_n8;
        locals.var_t1_dn9 = assign2150_e3815_d_n9;
        locals.var_t1_dn12 = assign2150_e3815_d_n12;
        locals.var_t1_dn14 = assign2150_e3815_d_n14;
        locals.var_t1_dn15 = assign2150_e3815_d_n15;
        locals.var_t1_dn16 = assign2150_e3815_d_n16;
        locals.var_t1_dn17 = assign2150_e3815_d_n17;
        locals.var_t1_dn18 = assign2150_e3815_d_n18;
        locals.var_t1_dn19 = assign2150_e3815_d_n19;
        locals.var_t1_dn20 = assign2150_e3815_d_n20;
        locals.var_t1_dn21 = assign2150_e3815_d_n21;
        locals.var_t1_dn22 = assign2150_e3815_d_n22;
        locals.var_t1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_6(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let (assign2160_e3835, assign2160_e3835_d_n0, assign2160_e3835_d_n1, assign2160_e3835_d_n2, assign2160_e3835_d_n3, assign2160_e3835_d_n4, assign2160_e3835_d_n5, assign2160_e3835_d_n6, assign2160_e3835_d_n7, assign2160_e3835_d_n8, assign2160_e3835_d_n9, assign2160_e3835_d_n12, assign2160_e3835_d_n14, assign2160_e3835_d_n15, assign2160_e3835_d_n16, assign2160_e3835_d_n17, assign2160_e3835_d_n18, assign2160_e3835_d_n19, assign2160_e3835_d_n20, assign2160_e3835_d_n21, assign2160_e3835_d_n22,) = {
    if ((locals.var_guard357 != 0.0) && (!((((locals.var_guard353 != 0.0) || (locals.var_guard354 != 0.0)) || (locals.var_guard355 != 0.0)) || (locals.var_guard356 != 0.0)))) {
        let assign2160_e3828: f64 = (p.p96 * p.p37);
        let assign2160_e3829: f64 = (assign2160_e3828).abs();
        let assign2160_e3832: f64 = (locals.var_t0 / locals.var_t1);
        let assign2160_e3833: f64 = (assign2160_e3829 * assign2160_e3832);
        (assign2160_e3833, (assign2160_e3829 * (((locals.var_t0_dn0 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn0)) / (locals.var_t1 * locals.var_t1))), (assign2160_e3829 * (((locals.var_t0_dn1 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn1)) / (locals.var_t1 * locals.var_t1))), (assign2160_e3829 * (((locals.var_t0_dn2 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn2)) / (locals.var_t1 * locals.var_t1))), (assign2160_e3829 * (((locals.var_t0_dn3 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn3)) / (locals.var_t1 * locals.var_t1))), (assign2160_e3829 * (((locals.var_t0_dn4 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn4)) / (locals.var_t1 * locals.var_t1))), (assign2160_e3829 * (((locals.var_t0_dn5 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn5)) / (locals.var_t1 * locals.var_t1))), (assign2160_e3829 * (((locals.var_t0_dn6 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn6)) / (locals.var_t1 * locals.var_t1))), (assign2160_e3829 * (((locals.var_t0_dn7 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn7)) / (locals.var_t1 * locals.var_t1))), (assign2160_e3829 * (((locals.var_t0_dn8 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn8)) / (locals.var_t1 * locals.var_t1))), (assign2160_e3829 * (((locals.var_t0_dn9 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn9)) / (locals.var_t1 * locals.var_t1))), (assign2160_e3829 * (((locals.var_t0_dn12 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn12)) / (locals.var_t1 * locals.var_t1))), (assign2160_e3829 * (((locals.var_t0_dn14 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn14)) / (locals.var_t1 * locals.var_t1))), (assign2160_e3829 * (((locals.var_t0_dn15 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn15)) / (locals.var_t1 * locals.var_t1))), (assign2160_e3829 * (((locals.var_t0_dn16 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn16)) / (locals.var_t1 * locals.var_t1))), (assign2160_e3829 * (((locals.var_t0_dn17 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn17)) / (locals.var_t1 * locals.var_t1))), (assign2160_e3829 * (((locals.var_t0_dn18 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn18)) / (locals.var_t1 * locals.var_t1))), (assign2160_e3829 * (((locals.var_t0_dn19 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn19)) / (locals.var_t1 * locals.var_t1))), (assign2160_e3829 * (((locals.var_t0_dn20 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn20)) / (locals.var_t1 * locals.var_t1))), (assign2160_e3829 * (((locals.var_t0_dn21 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn21)) / (locals.var_t1 * locals.var_t1))), (assign2160_e3829 * (((locals.var_t0_dn22 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn22)) / (locals.var_t1 * locals.var_t1))),)
    } else {
        (locals.var_ns0ddlag, locals.var_ns0ddlag_dn0, locals.var_ns0ddlag_dn1, locals.var_ns0ddlag_dn2, locals.var_ns0ddlag_dn3, locals.var_ns0ddlag_dn4, locals.var_ns0ddlag_dn5, locals.var_ns0ddlag_dn6, locals.var_ns0ddlag_dn7, locals.var_ns0ddlag_dn8, locals.var_ns0ddlag_dn9, locals.var_ns0ddlag_dn12, locals.var_ns0ddlag_dn14, locals.var_ns0ddlag_dn15, locals.var_ns0ddlag_dn16, locals.var_ns0ddlag_dn17, locals.var_ns0ddlag_dn18, locals.var_ns0ddlag_dn19, locals.var_ns0ddlag_dn20, locals.var_ns0ddlag_dn21, locals.var_ns0ddlag_dn22,)
    }
};
        locals.var_ns0ddlag = assign2160_e3835;
        locals.var_ns0ddlag_dn0 = assign2160_e3835_d_n0;
        locals.var_ns0ddlag_dn1 = assign2160_e3835_d_n1;
        locals.var_ns0ddlag_dn2 = assign2160_e3835_d_n2;
        locals.var_ns0ddlag_dn3 = assign2160_e3835_d_n3;
        locals.var_ns0ddlag_dn4 = assign2160_e3835_d_n4;
        locals.var_ns0ddlag_dn5 = assign2160_e3835_d_n5;
        locals.var_ns0ddlag_dn6 = assign2160_e3835_d_n6;
        locals.var_ns0ddlag_dn7 = assign2160_e3835_d_n7;
        locals.var_ns0ddlag_dn8 = assign2160_e3835_d_n8;
        locals.var_ns0ddlag_dn9 = assign2160_e3835_d_n9;
        locals.var_ns0ddlag_dn12 = assign2160_e3835_d_n12;
        locals.var_ns0ddlag_dn14 = assign2160_e3835_d_n14;
        locals.var_ns0ddlag_dn15 = assign2160_e3835_d_n15;
        locals.var_ns0ddlag_dn16 = assign2160_e3835_d_n16;
        locals.var_ns0ddlag_dn17 = assign2160_e3835_d_n17;
        locals.var_ns0ddlag_dn18 = assign2160_e3835_d_n18;
        locals.var_ns0ddlag_dn19 = assign2160_e3835_d_n19;
        locals.var_ns0ddlag_dn20 = assign2160_e3835_d_n20;
        locals.var_ns0ddlag_dn21 = assign2160_e3835_d_n21;
        locals.var_ns0ddlag_dn22 = assign2160_e3835_d_n22;
        locals.var_ns0ddlag_rv = 0.0;

        let (assign2210_e3970, assign2210_e3970_d_n0, assign2210_e3970_d_n2, assign2210_e3970_d_n5, assign2210_e3970_d_n12,) = {
    if ((locals.var_guard358 != 0.0) && (!(((((locals.var_guard353 != 0.0) || (locals.var_guard354 != 0.0)) || (locals.var_guard355 != 0.0)) || (locals.var_guard356 != 0.0)) || (locals.var_guard357 != 0.0)))) {
        ((nv5 - 0.0), 0.0, 0.0, 1.0, 0.0,)
    } else {
        (locals.var_vaux, locals.var_vaux_dn0, locals.var_vaux_dn2, locals.var_vaux_dn5, locals.var_vaux_dn12,)
    }
};
        locals.var_vaux = assign2210_e3970;
        locals.var_vaux_dn0 = assign2210_e3970_d_n0;
        locals.var_vaux_dn2 = assign2210_e3970_d_n2;
        locals.var_vaux_dn5 = assign2210_e3970_d_n5;
        locals.var_vaux_dn12 = assign2210_e3970_d_n12;
        locals.var_vaux_rv = 0.0;

        let (assign2220_e3985, assign2220_e3985_d_n6,) = {
    if ((locals.var_guard358 != 0.0) && (!(((((locals.var_guard353 != 0.0) || (locals.var_guard354 != 0.0)) || (locals.var_guard355 != 0.0)) || (locals.var_guard356 != 0.0)) || (locals.var_guard357 != 0.0)))) {
        ((nv6 - 0.0), 1.0,)
    } else {
        (locals.var_vauy, locals.var_vauy_dn6,)
    }
};
        locals.var_vauy = assign2220_e3985;
        locals.var_vauy_dn6 = assign2220_e3985_d_n6;
        locals.var_vauy_rv = 0.0;

        let (assign2230_e4002, assign2230_e4002_d_n0, assign2230_e4002_d_n1, assign2230_e4002_d_n2, assign2230_e4002_d_n3, assign2230_e4002_d_n4, assign2230_e4002_d_n5, assign2230_e4002_d_n6, assign2230_e4002_d_n7, assign2230_e4002_d_n8, assign2230_e4002_d_n9, assign2230_e4002_d_n12, assign2230_e4002_d_n14, assign2230_e4002_d_n15, assign2230_e4002_d_n16, assign2230_e4002_d_n17, assign2230_e4002_d_n18, assign2230_e4002_d_n19, assign2230_e4002_d_n20, assign2230_e4002_d_n21, assign2230_e4002_d_n22,) = {
    if ((locals.var_guard358 != 0.0) && (!(((((locals.var_guard353 != 0.0) || (locals.var_guard354 != 0.0)) || (locals.var_guard355 != 0.0)) || (locals.var_guard356 != 0.0)) || (locals.var_guard357 != 0.0)))) {
        let assign2230_e4000: f64 = (locals.var_vaux * p.p89);
        (assign2230_e4000, (locals.var_vaux_dn0 * p.p89), 0.0, (locals.var_vaux_dn2 * p.p89), 0.0, 0.0, (locals.var_vaux_dn5 * p.p89), 0.0, 0.0, 0.0, 0.0, (locals.var_vaux_dn12 * p.p89), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn1, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn12, locals.var_t0_dn14, locals.var_t0_dn15, locals.var_t0_dn16, locals.var_t0_dn17, locals.var_t0_dn18, locals.var_t0_dn19, locals.var_t0_dn20, locals.var_t0_dn21, locals.var_t0_dn22,)
    }
};
        locals.var_t0 = assign2230_e4002;
        locals.var_t0_dn0 = assign2230_e4002_d_n0;
        locals.var_t0_dn1 = assign2230_e4002_d_n1;
        locals.var_t0_dn2 = assign2230_e4002_d_n2;
        locals.var_t0_dn3 = assign2230_e4002_d_n3;
        locals.var_t0_dn4 = assign2230_e4002_d_n4;
        locals.var_t0_dn5 = assign2230_e4002_d_n5;
        locals.var_t0_dn6 = assign2230_e4002_d_n6;
        locals.var_t0_dn7 = assign2230_e4002_d_n7;
        locals.var_t0_dn8 = assign2230_e4002_d_n8;
        locals.var_t0_dn9 = assign2230_e4002_d_n9;
        locals.var_t0_dn12 = assign2230_e4002_d_n12;
        locals.var_t0_dn14 = assign2230_e4002_d_n14;
        locals.var_t0_dn15 = assign2230_e4002_d_n15;
        locals.var_t0_dn16 = assign2230_e4002_d_n16;
        locals.var_t0_dn17 = assign2230_e4002_d_n17;
        locals.var_t0_dn18 = assign2230_e4002_d_n18;
        locals.var_t0_dn19 = assign2230_e4002_d_n19;
        locals.var_t0_dn20 = assign2230_e4002_d_n20;
        locals.var_t0_dn21 = assign2230_e4002_d_n21;
        locals.var_t0_dn22 = assign2230_e4002_d_n22;
        locals.var_t0_rv = 0.0;

        let (assign2240_e4024, assign2240_e4024_d_n0, assign2240_e4024_d_n1, assign2240_e4024_d_n2, assign2240_e4024_d_n3, assign2240_e4024_d_n4, assign2240_e4024_d_n5, assign2240_e4024_d_n6, assign2240_e4024_d_n7, assign2240_e4024_d_n8, assign2240_e4024_d_n9, assign2240_e4024_d_n12, assign2240_e4024_d_n14, assign2240_e4024_d_n15, assign2240_e4024_d_n16, assign2240_e4024_d_n17, assign2240_e4024_d_n18, assign2240_e4024_d_n19, assign2240_e4024_d_n20, assign2240_e4024_d_n21, assign2240_e4024_d_n22,) = {
    if ((locals.var_guard358 != 0.0) && (!(((((locals.var_guard353 != 0.0) || (locals.var_guard354 != 0.0)) || (locals.var_guard355 != 0.0)) || (locals.var_guard356 != 0.0)) || (locals.var_guard357 != 0.0)))) {
        let assign2240_e4017: f64 = (locals.var_vaux * locals.var_vaux);
        let assign2240_e4020: f64 = (p.p89 * p.p89);
        let assign2240_e4021: f64 = (assign2240_e4017 + assign2240_e4020);
        let assign2240_e4022: f64 = (assign2240_e4021).sqrt();
        (assign2240_e4022, (((locals.var_vaux_dn0 * locals.var_vaux) + (locals.var_vaux * locals.var_vaux_dn0)) / (2.0 * assign2240_e4022)), 0.0, (((locals.var_vaux_dn2 * locals.var_vaux) + (locals.var_vaux * locals.var_vaux_dn2)) / (2.0 * assign2240_e4022)), 0.0, 0.0, (((locals.var_vaux_dn5 * locals.var_vaux) + (locals.var_vaux * locals.var_vaux_dn5)) / (2.0 * assign2240_e4022)), 0.0, 0.0, 0.0, 0.0, (((locals.var_vaux_dn12 * locals.var_vaux) + (locals.var_vaux * locals.var_vaux_dn12)) / (2.0 * assign2240_e4022)), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn1, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn12, locals.var_t1_dn14, locals.var_t1_dn15, locals.var_t1_dn16, locals.var_t1_dn17, locals.var_t1_dn18, locals.var_t1_dn19, locals.var_t1_dn20, locals.var_t1_dn21, locals.var_t1_dn22,)
    }
};
        locals.var_t1 = assign2240_e4024;
        locals.var_t1_dn0 = assign2240_e4024_d_n0;
        locals.var_t1_dn1 = assign2240_e4024_d_n1;
        locals.var_t1_dn2 = assign2240_e4024_d_n2;
        locals.var_t1_dn3 = assign2240_e4024_d_n3;
        locals.var_t1_dn4 = assign2240_e4024_d_n4;
        locals.var_t1_dn5 = assign2240_e4024_d_n5;
        locals.var_t1_dn6 = assign2240_e4024_d_n6;
        locals.var_t1_dn7 = assign2240_e4024_d_n7;
        locals.var_t1_dn8 = assign2240_e4024_d_n8;
        locals.var_t1_dn9 = assign2240_e4024_d_n9;
        locals.var_t1_dn12 = assign2240_e4024_d_n12;
        locals.var_t1_dn14 = assign2240_e4024_d_n14;
        locals.var_t1_dn15 = assign2240_e4024_d_n15;
        locals.var_t1_dn16 = assign2240_e4024_d_n16;
        locals.var_t1_dn17 = assign2240_e4024_d_n17;
        locals.var_t1_dn18 = assign2240_e4024_d_n18;
        locals.var_t1_dn19 = assign2240_e4024_d_n19;
        locals.var_t1_dn20 = assign2240_e4024_d_n20;
        locals.var_t1_dn21 = assign2240_e4024_d_n21;
        locals.var_t1_dn22 = assign2240_e4024_d_n22;
        locals.var_t1_rv = 0.0;

        let (assign2250_e4046, assign2250_e4046_d_n0, assign2250_e4046_d_n1, assign2250_e4046_d_n2, assign2250_e4046_d_n3, assign2250_e4046_d_n4, assign2250_e4046_d_n5, assign2250_e4046_d_n6, assign2250_e4046_d_n7, assign2250_e4046_d_n8, assign2250_e4046_d_n9, assign2250_e4046_d_n12, assign2250_e4046_d_n14, assign2250_e4046_d_n15, assign2250_e4046_d_n16, assign2250_e4046_d_n17, assign2250_e4046_d_n18, assign2250_e4046_d_n19, assign2250_e4046_d_n20, assign2250_e4046_d_n21, assign2250_e4046_d_n22,) = {
    if ((locals.var_guard358 != 0.0) && (!(((((locals.var_guard353 != 0.0) || (locals.var_guard354 != 0.0)) || (locals.var_guard355 != 0.0)) || (locals.var_guard356 != 0.0)) || (locals.var_guard357 != 0.0)))) {
        let assign2250_e4039: f64 = (p.p91 * p.p10);
        let assign2250_e4040: f64 = (assign2250_e4039).abs();
        let assign2250_e4043: f64 = (locals.var_t0 / locals.var_t1);
        let assign2250_e4044: f64 = (assign2250_e4040 * assign2250_e4043);
        (assign2250_e4044, (assign2250_e4040 * (((locals.var_t0_dn0 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn0)) / (locals.var_t1 * locals.var_t1))), (assign2250_e4040 * (((locals.var_t0_dn1 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn1)) / (locals.var_t1 * locals.var_t1))), (assign2250_e4040 * (((locals.var_t0_dn2 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn2)) / (locals.var_t1 * locals.var_t1))), (assign2250_e4040 * (((locals.var_t0_dn3 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn3)) / (locals.var_t1 * locals.var_t1))), (assign2250_e4040 * (((locals.var_t0_dn4 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn4)) / (locals.var_t1 * locals.var_t1))), (assign2250_e4040 * (((locals.var_t0_dn5 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn5)) / (locals.var_t1 * locals.var_t1))), (assign2250_e4040 * (((locals.var_t0_dn6 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn6)) / (locals.var_t1 * locals.var_t1))), (assign2250_e4040 * (((locals.var_t0_dn7 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn7)) / (locals.var_t1 * locals.var_t1))), (assign2250_e4040 * (((locals.var_t0_dn8 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn8)) / (locals.var_t1 * locals.var_t1))), (assign2250_e4040 * (((locals.var_t0_dn9 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn9)) / (locals.var_t1 * locals.var_t1))), (assign2250_e4040 * (((locals.var_t0_dn12 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn12)) / (locals.var_t1 * locals.var_t1))), (assign2250_e4040 * (((locals.var_t0_dn14 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn14)) / (locals.var_t1 * locals.var_t1))), (assign2250_e4040 * (((locals.var_t0_dn15 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn15)) / (locals.var_t1 * locals.var_t1))), (assign2250_e4040 * (((locals.var_t0_dn16 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn16)) / (locals.var_t1 * locals.var_t1))), (assign2250_e4040 * (((locals.var_t0_dn17 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn17)) / (locals.var_t1 * locals.var_t1))), (assign2250_e4040 * (((locals.var_t0_dn18 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn18)) / (locals.var_t1 * locals.var_t1))), (assign2250_e4040 * (((locals.var_t0_dn19 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn19)) / (locals.var_t1 * locals.var_t1))), (assign2250_e4040 * (((locals.var_t0_dn20 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn20)) / (locals.var_t1 * locals.var_t1))), (assign2250_e4040 * (((locals.var_t0_dn21 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn21)) / (locals.var_t1 * locals.var_t1))), (assign2250_e4040 * (((locals.var_t0_dn22 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn22)) / (locals.var_t1 * locals.var_t1))),)
    } else {
        (locals.var_voffdlag, locals.var_voffdlag_dn0, locals.var_voffdlag_dn1, locals.var_voffdlag_dn2, locals.var_voffdlag_dn3, locals.var_voffdlag_dn4, locals.var_voffdlag_dn5, locals.var_voffdlag_dn6, locals.var_voffdlag_dn7, locals.var_voffdlag_dn8, locals.var_voffdlag_dn9, locals.var_voffdlag_dn12, locals.var_voffdlag_dn14, locals.var_voffdlag_dn15, locals.var_voffdlag_dn16, locals.var_voffdlag_dn17, locals.var_voffdlag_dn18, locals.var_voffdlag_dn19, locals.var_voffdlag_dn20, locals.var_voffdlag_dn21, locals.var_voffdlag_dn22,)
    }
};
        locals.var_voffdlag = assign2250_e4046;
        locals.var_voffdlag_dn0 = assign2250_e4046_d_n0;
        locals.var_voffdlag_dn1 = assign2250_e4046_d_n1;
        locals.var_voffdlag_dn2 = assign2250_e4046_d_n2;
        locals.var_voffdlag_dn3 = assign2250_e4046_d_n3;
        locals.var_voffdlag_dn4 = assign2250_e4046_d_n4;
        locals.var_voffdlag_dn5 = assign2250_e4046_d_n5;
        locals.var_voffdlag_dn6 = assign2250_e4046_d_n6;
        locals.var_voffdlag_dn7 = assign2250_e4046_d_n7;
        locals.var_voffdlag_dn8 = assign2250_e4046_d_n8;
        locals.var_voffdlag_dn9 = assign2250_e4046_d_n9;
        locals.var_voffdlag_dn12 = assign2250_e4046_d_n12;
        locals.var_voffdlag_dn14 = assign2250_e4046_d_n14;
        locals.var_voffdlag_dn15 = assign2250_e4046_d_n15;
        locals.var_voffdlag_dn16 = assign2250_e4046_d_n16;
        locals.var_voffdlag_dn17 = assign2250_e4046_d_n17;
        locals.var_voffdlag_dn18 = assign2250_e4046_d_n18;
        locals.var_voffdlag_dn19 = assign2250_e4046_d_n19;
        locals.var_voffdlag_dn20 = assign2250_e4046_d_n20;
        locals.var_voffdlag_dn21 = assign2250_e4046_d_n21;
        locals.var_voffdlag_dn22 = assign2250_e4046_d_n22;
        locals.var_voffdlag_rv = 0.0;

        let (assign2260_e4063, assign2260_e4063_d_n0, assign2260_e4063_d_n1, assign2260_e4063_d_n2, assign2260_e4063_d_n3, assign2260_e4063_d_n4, assign2260_e4063_d_n5, assign2260_e4063_d_n6, assign2260_e4063_d_n7, assign2260_e4063_d_n8, assign2260_e4063_d_n9, assign2260_e4063_d_n12, assign2260_e4063_d_n14, assign2260_e4063_d_n15, assign2260_e4063_d_n16, assign2260_e4063_d_n17, assign2260_e4063_d_n18, assign2260_e4063_d_n19, assign2260_e4063_d_n20, assign2260_e4063_d_n21, assign2260_e4063_d_n22,) = {
    if ((locals.var_guard358 != 0.0) && (!(((((locals.var_guard353 != 0.0) || (locals.var_guard354 != 0.0)) || (locals.var_guard355 != 0.0)) || (locals.var_guard356 != 0.0)) || (locals.var_guard357 != 0.0)))) {
        let assign2260_e4061: f64 = (locals.var_vaux * p.p89);
        (assign2260_e4061, (locals.var_vaux_dn0 * p.p89), 0.0, (locals.var_vaux_dn2 * p.p89), 0.0, 0.0, (locals.var_vaux_dn5 * p.p89), 0.0, 0.0, 0.0, 0.0, (locals.var_vaux_dn12 * p.p89), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn1, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn12, locals.var_t0_dn14, locals.var_t0_dn15, locals.var_t0_dn16, locals.var_t0_dn17, locals.var_t0_dn18, locals.var_t0_dn19, locals.var_t0_dn20, locals.var_t0_dn21, locals.var_t0_dn22,)
    }
};
        locals.var_t0 = assign2260_e4063;
        locals.var_t0_dn0 = assign2260_e4063_d_n0;
        locals.var_t0_dn1 = assign2260_e4063_d_n1;
        locals.var_t0_dn2 = assign2260_e4063_d_n2;
        locals.var_t0_dn3 = assign2260_e4063_d_n3;
        locals.var_t0_dn4 = assign2260_e4063_d_n4;
        locals.var_t0_dn5 = assign2260_e4063_d_n5;
        locals.var_t0_dn6 = assign2260_e4063_d_n6;
        locals.var_t0_dn7 = assign2260_e4063_d_n7;
        locals.var_t0_dn8 = assign2260_e4063_d_n8;
        locals.var_t0_dn9 = assign2260_e4063_d_n9;
        locals.var_t0_dn12 = assign2260_e4063_d_n12;
        locals.var_t0_dn14 = assign2260_e4063_d_n14;
        locals.var_t0_dn15 = assign2260_e4063_d_n15;
        locals.var_t0_dn16 = assign2260_e4063_d_n16;
        locals.var_t0_dn17 = assign2260_e4063_d_n17;
        locals.var_t0_dn18 = assign2260_e4063_d_n18;
        locals.var_t0_dn19 = assign2260_e4063_d_n19;
        locals.var_t0_dn20 = assign2260_e4063_d_n20;
        locals.var_t0_dn21 = assign2260_e4063_d_n21;
        locals.var_t0_dn22 = assign2260_e4063_d_n22;
        locals.var_t0_rv = 0.0;

        let (assign2270_e4085, assign2270_e4085_d_n0, assign2270_e4085_d_n1, assign2270_e4085_d_n2, assign2270_e4085_d_n3, assign2270_e4085_d_n4, assign2270_e4085_d_n5, assign2270_e4085_d_n6, assign2270_e4085_d_n7, assign2270_e4085_d_n8, assign2270_e4085_d_n9, assign2270_e4085_d_n12, assign2270_e4085_d_n14, assign2270_e4085_d_n15, assign2270_e4085_d_n16, assign2270_e4085_d_n17, assign2270_e4085_d_n18, assign2270_e4085_d_n19, assign2270_e4085_d_n20, assign2270_e4085_d_n21, assign2270_e4085_d_n22,) = {
    if ((locals.var_guard358 != 0.0) && (!(((((locals.var_guard353 != 0.0) || (locals.var_guard354 != 0.0)) || (locals.var_guard355 != 0.0)) || (locals.var_guard356 != 0.0)) || (locals.var_guard357 != 0.0)))) {
        let assign2270_e4078: f64 = (locals.var_vaux * locals.var_vaux);
        let assign2270_e4081: f64 = (p.p89 * p.p89);
        let assign2270_e4082: f64 = (assign2270_e4078 + assign2270_e4081);
        let assign2270_e4083: f64 = (assign2270_e4082).sqrt();
        (assign2270_e4083, (((locals.var_vaux_dn0 * locals.var_vaux) + (locals.var_vaux * locals.var_vaux_dn0)) / (2.0 * assign2270_e4083)), 0.0, (((locals.var_vaux_dn2 * locals.var_vaux) + (locals.var_vaux * locals.var_vaux_dn2)) / (2.0 * assign2270_e4083)), 0.0, 0.0, (((locals.var_vaux_dn5 * locals.var_vaux) + (locals.var_vaux * locals.var_vaux_dn5)) / (2.0 * assign2270_e4083)), 0.0, 0.0, 0.0, 0.0, (((locals.var_vaux_dn12 * locals.var_vaux) + (locals.var_vaux * locals.var_vaux_dn12)) / (2.0 * assign2270_e4083)), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn1, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn12, locals.var_t1_dn14, locals.var_t1_dn15, locals.var_t1_dn16, locals.var_t1_dn17, locals.var_t1_dn18, locals.var_t1_dn19, locals.var_t1_dn20, locals.var_t1_dn21, locals.var_t1_dn22,)
    }
};
        locals.var_t1 = assign2270_e4085;
        locals.var_t1_dn0 = assign2270_e4085_d_n0;
        locals.var_t1_dn1 = assign2270_e4085_d_n1;
        locals.var_t1_dn2 = assign2270_e4085_d_n2;
        locals.var_t1_dn3 = assign2270_e4085_d_n3;
        locals.var_t1_dn4 = assign2270_e4085_d_n4;
        locals.var_t1_dn5 = assign2270_e4085_d_n5;
        locals.var_t1_dn6 = assign2270_e4085_d_n6;
        locals.var_t1_dn7 = assign2270_e4085_d_n7;
        locals.var_t1_dn8 = assign2270_e4085_d_n8;
        locals.var_t1_dn9 = assign2270_e4085_d_n9;
        locals.var_t1_dn12 = assign2270_e4085_d_n12;
        locals.var_t1_dn14 = assign2270_e4085_d_n14;
        locals.var_t1_dn15 = assign2270_e4085_d_n15;
        locals.var_t1_dn16 = assign2270_e4085_d_n16;
        locals.var_t1_dn17 = assign2270_e4085_d_n17;
        locals.var_t1_dn18 = assign2270_e4085_d_n18;
        locals.var_t1_dn19 = assign2270_e4085_d_n19;
        locals.var_t1_dn20 = assign2270_e4085_d_n20;
        locals.var_t1_dn21 = assign2270_e4085_d_n21;
        locals.var_t1_dn22 = assign2270_e4085_d_n22;
        locals.var_t1_rv = 0.0;

        let (assign2280_e4107, assign2280_e4107_d_n0, assign2280_e4107_d_n1, assign2280_e4107_d_n2, assign2280_e4107_d_n3, assign2280_e4107_d_n4, assign2280_e4107_d_n5, assign2280_e4107_d_n6, assign2280_e4107_d_n7, assign2280_e4107_d_n8, assign2280_e4107_d_n9, assign2280_e4107_d_n12, assign2280_e4107_d_n14, assign2280_e4107_d_n15, assign2280_e4107_d_n16, assign2280_e4107_d_n17, assign2280_e4107_d_n18, assign2280_e4107_d_n19, assign2280_e4107_d_n20, assign2280_e4107_d_n21, assign2280_e4107_d_n22,) = {
    if ((locals.var_guard358 != 0.0) && (!(((((locals.var_guard353 != 0.0) || (locals.var_guard354 != 0.0)) || (locals.var_guard355 != 0.0)) || (locals.var_guard356 != 0.0)) || (locals.var_guard357 != 0.0)))) {
        let assign2280_e4100: f64 = (p.p95 * p.p36);
        let assign2280_e4101: f64 = (assign2280_e4100).abs();
        let assign2280_e4104: f64 = (locals.var_t0 / locals.var_t1);
        let assign2280_e4105: f64 = (assign2280_e4101 * assign2280_e4104);
        (assign2280_e4105, (assign2280_e4101 * (((locals.var_t0_dn0 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn0)) / (locals.var_t1 * locals.var_t1))), (assign2280_e4101 * (((locals.var_t0_dn1 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn1)) / (locals.var_t1 * locals.var_t1))), (assign2280_e4101 * (((locals.var_t0_dn2 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn2)) / (locals.var_t1 * locals.var_t1))), (assign2280_e4101 * (((locals.var_t0_dn3 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn3)) / (locals.var_t1 * locals.var_t1))), (assign2280_e4101 * (((locals.var_t0_dn4 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn4)) / (locals.var_t1 * locals.var_t1))), (assign2280_e4101 * (((locals.var_t0_dn5 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn5)) / (locals.var_t1 * locals.var_t1))), (assign2280_e4101 * (((locals.var_t0_dn6 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn6)) / (locals.var_t1 * locals.var_t1))), (assign2280_e4101 * (((locals.var_t0_dn7 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn7)) / (locals.var_t1 * locals.var_t1))), (assign2280_e4101 * (((locals.var_t0_dn8 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn8)) / (locals.var_t1 * locals.var_t1))), (assign2280_e4101 * (((locals.var_t0_dn9 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn9)) / (locals.var_t1 * locals.var_t1))), (assign2280_e4101 * (((locals.var_t0_dn12 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn12)) / (locals.var_t1 * locals.var_t1))), (assign2280_e4101 * (((locals.var_t0_dn14 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn14)) / (locals.var_t1 * locals.var_t1))), (assign2280_e4101 * (((locals.var_t0_dn15 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn15)) / (locals.var_t1 * locals.var_t1))), (assign2280_e4101 * (((locals.var_t0_dn16 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn16)) / (locals.var_t1 * locals.var_t1))), (assign2280_e4101 * (((locals.var_t0_dn17 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn17)) / (locals.var_t1 * locals.var_t1))), (assign2280_e4101 * (((locals.var_t0_dn18 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn18)) / (locals.var_t1 * locals.var_t1))), (assign2280_e4101 * (((locals.var_t0_dn19 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn19)) / (locals.var_t1 * locals.var_t1))), (assign2280_e4101 * (((locals.var_t0_dn20 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn20)) / (locals.var_t1 * locals.var_t1))), (assign2280_e4101 * (((locals.var_t0_dn21 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn21)) / (locals.var_t1 * locals.var_t1))), (assign2280_e4101 * (((locals.var_t0_dn22 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn22)) / (locals.var_t1 * locals.var_t1))),)
    } else {
        (locals.var_ns0sdlag, locals.var_ns0sdlag_dn0, locals.var_ns0sdlag_dn1, locals.var_ns0sdlag_dn2, locals.var_ns0sdlag_dn3, locals.var_ns0sdlag_dn4, locals.var_ns0sdlag_dn5, locals.var_ns0sdlag_dn6, locals.var_ns0sdlag_dn7, locals.var_ns0sdlag_dn8, locals.var_ns0sdlag_dn9, locals.var_ns0sdlag_dn12, locals.var_ns0sdlag_dn14, locals.var_ns0sdlag_dn15, locals.var_ns0sdlag_dn16, locals.var_ns0sdlag_dn17, locals.var_ns0sdlag_dn18, locals.var_ns0sdlag_dn19, locals.var_ns0sdlag_dn20, locals.var_ns0sdlag_dn21, locals.var_ns0sdlag_dn22,)
    }
};
        locals.var_ns0sdlag = assign2280_e4107;
        locals.var_ns0sdlag_dn0 = assign2280_e4107_d_n0;
        locals.var_ns0sdlag_dn1 = assign2280_e4107_d_n1;
        locals.var_ns0sdlag_dn2 = assign2280_e4107_d_n2;
        locals.var_ns0sdlag_dn3 = assign2280_e4107_d_n3;
        locals.var_ns0sdlag_dn4 = assign2280_e4107_d_n4;
        locals.var_ns0sdlag_dn5 = assign2280_e4107_d_n5;
        locals.var_ns0sdlag_dn6 = assign2280_e4107_d_n6;
        locals.var_ns0sdlag_dn7 = assign2280_e4107_d_n7;
        locals.var_ns0sdlag_dn8 = assign2280_e4107_d_n8;
        locals.var_ns0sdlag_dn9 = assign2280_e4107_d_n9;
        locals.var_ns0sdlag_dn12 = assign2280_e4107_d_n12;
        locals.var_ns0sdlag_dn14 = assign2280_e4107_d_n14;
        locals.var_ns0sdlag_dn15 = assign2280_e4107_d_n15;
        locals.var_ns0sdlag_dn16 = assign2280_e4107_d_n16;
        locals.var_ns0sdlag_dn17 = assign2280_e4107_d_n17;
        locals.var_ns0sdlag_dn18 = assign2280_e4107_d_n18;
        locals.var_ns0sdlag_dn19 = assign2280_e4107_d_n19;
        locals.var_ns0sdlag_dn20 = assign2280_e4107_d_n20;
        locals.var_ns0sdlag_dn21 = assign2280_e4107_d_n21;
        locals.var_ns0sdlag_dn22 = assign2280_e4107_d_n22;
        locals.var_ns0sdlag_rv = 0.0;

        let (assign2290_e4124, assign2290_e4124_d_n0, assign2290_e4124_d_n1, assign2290_e4124_d_n2, assign2290_e4124_d_n3, assign2290_e4124_d_n4, assign2290_e4124_d_n5, assign2290_e4124_d_n6, assign2290_e4124_d_n7, assign2290_e4124_d_n8, assign2290_e4124_d_n9, assign2290_e4124_d_n12, assign2290_e4124_d_n14, assign2290_e4124_d_n15, assign2290_e4124_d_n16, assign2290_e4124_d_n17, assign2290_e4124_d_n18, assign2290_e4124_d_n19, assign2290_e4124_d_n20, assign2290_e4124_d_n21, assign2290_e4124_d_n22,) = {
    if ((locals.var_guard358 != 0.0) && (!(((((locals.var_guard353 != 0.0) || (locals.var_guard354 != 0.0)) || (locals.var_guard355 != 0.0)) || (locals.var_guard356 != 0.0)) || (locals.var_guard357 != 0.0)))) {
        let assign2290_e4122: f64 = (locals.var_vaux * p.p89);
        (assign2290_e4122, (locals.var_vaux_dn0 * p.p89), 0.0, (locals.var_vaux_dn2 * p.p89), 0.0, 0.0, (locals.var_vaux_dn5 * p.p89), 0.0, 0.0, 0.0, 0.0, (locals.var_vaux_dn12 * p.p89), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn1, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn12, locals.var_t0_dn14, locals.var_t0_dn15, locals.var_t0_dn16, locals.var_t0_dn17, locals.var_t0_dn18, locals.var_t0_dn19, locals.var_t0_dn20, locals.var_t0_dn21, locals.var_t0_dn22,)
    }
};
        locals.var_t0 = assign2290_e4124;
        locals.var_t0_dn0 = assign2290_e4124_d_n0;
        locals.var_t0_dn1 = assign2290_e4124_d_n1;
        locals.var_t0_dn2 = assign2290_e4124_d_n2;
        locals.var_t0_dn3 = assign2290_e4124_d_n3;
        locals.var_t0_dn4 = assign2290_e4124_d_n4;
        locals.var_t0_dn5 = assign2290_e4124_d_n5;
        locals.var_t0_dn6 = assign2290_e4124_d_n6;
        locals.var_t0_dn7 = assign2290_e4124_d_n7;
        locals.var_t0_dn8 = assign2290_e4124_d_n8;
        locals.var_t0_dn9 = assign2290_e4124_d_n9;
        locals.var_t0_dn12 = assign2290_e4124_d_n12;
        locals.var_t0_dn14 = assign2290_e4124_d_n14;
        locals.var_t0_dn15 = assign2290_e4124_d_n15;
        locals.var_t0_dn16 = assign2290_e4124_d_n16;
        locals.var_t0_dn17 = assign2290_e4124_d_n17;
        locals.var_t0_dn18 = assign2290_e4124_d_n18;
        locals.var_t0_dn19 = assign2290_e4124_d_n19;
        locals.var_t0_dn20 = assign2290_e4124_d_n20;
        locals.var_t0_dn21 = assign2290_e4124_d_n21;
        locals.var_t0_dn22 = assign2290_e4124_d_n22;
        locals.var_t0_rv = 0.0;

        let (assign2300_e4146, assign2300_e4146_d_n0, assign2300_e4146_d_n1, assign2300_e4146_d_n2, assign2300_e4146_d_n3, assign2300_e4146_d_n4, assign2300_e4146_d_n5, assign2300_e4146_d_n6, assign2300_e4146_d_n7, assign2300_e4146_d_n8, assign2300_e4146_d_n9, assign2300_e4146_d_n12, assign2300_e4146_d_n14, assign2300_e4146_d_n15, assign2300_e4146_d_n16, assign2300_e4146_d_n17, assign2300_e4146_d_n18, assign2300_e4146_d_n19, assign2300_e4146_d_n20, assign2300_e4146_d_n21, assign2300_e4146_d_n22,) = {
    if ((locals.var_guard358 != 0.0) && (!(((((locals.var_guard353 != 0.0) || (locals.var_guard354 != 0.0)) || (locals.var_guard355 != 0.0)) || (locals.var_guard356 != 0.0)) || (locals.var_guard357 != 0.0)))) {
        let assign2300_e4139: f64 = (locals.var_vaux * locals.var_vaux);
        let assign2300_e4142: f64 = (p.p89 * p.p89);
        let assign2300_e4143: f64 = (assign2300_e4139 + assign2300_e4142);
        let assign2300_e4144: f64 = (assign2300_e4143).sqrt();
        (assign2300_e4144, (((locals.var_vaux_dn0 * locals.var_vaux) + (locals.var_vaux * locals.var_vaux_dn0)) / (2.0 * assign2300_e4144)), 0.0, (((locals.var_vaux_dn2 * locals.var_vaux) + (locals.var_vaux * locals.var_vaux_dn2)) / (2.0 * assign2300_e4144)), 0.0, 0.0, (((locals.var_vaux_dn5 * locals.var_vaux) + (locals.var_vaux * locals.var_vaux_dn5)) / (2.0 * assign2300_e4144)), 0.0, 0.0, 0.0, 0.0, (((locals.var_vaux_dn12 * locals.var_vaux) + (locals.var_vaux * locals.var_vaux_dn12)) / (2.0 * assign2300_e4144)), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn1, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn12, locals.var_t1_dn14, locals.var_t1_dn15, locals.var_t1_dn16, locals.var_t1_dn17, locals.var_t1_dn18, locals.var_t1_dn19, locals.var_t1_dn20, locals.var_t1_dn21, locals.var_t1_dn22,)
    }
};
        locals.var_t1 = assign2300_e4146;
        locals.var_t1_dn0 = assign2300_e4146_d_n0;
        locals.var_t1_dn1 = assign2300_e4146_d_n1;
        locals.var_t1_dn2 = assign2300_e4146_d_n2;
        locals.var_t1_dn3 = assign2300_e4146_d_n3;
        locals.var_t1_dn4 = assign2300_e4146_d_n4;
        locals.var_t1_dn5 = assign2300_e4146_d_n5;
        locals.var_t1_dn6 = assign2300_e4146_d_n6;
        locals.var_t1_dn7 = assign2300_e4146_d_n7;
        locals.var_t1_dn8 = assign2300_e4146_d_n8;
        locals.var_t1_dn9 = assign2300_e4146_d_n9;
        locals.var_t1_dn12 = assign2300_e4146_d_n12;
        locals.var_t1_dn14 = assign2300_e4146_d_n14;
        locals.var_t1_dn15 = assign2300_e4146_d_n15;
        locals.var_t1_dn16 = assign2300_e4146_d_n16;
        locals.var_t1_dn17 = assign2300_e4146_d_n17;
        locals.var_t1_dn18 = assign2300_e4146_d_n18;
        locals.var_t1_dn19 = assign2300_e4146_d_n19;
        locals.var_t1_dn20 = assign2300_e4146_d_n20;
        locals.var_t1_dn21 = assign2300_e4146_d_n21;
        locals.var_t1_dn22 = assign2300_e4146_d_n22;
        locals.var_t1_rv = 0.0;

        let (assign2310_e4168, assign2310_e4168_d_n0, assign2310_e4168_d_n1, assign2310_e4168_d_n2, assign2310_e4168_d_n3, assign2310_e4168_d_n4, assign2310_e4168_d_n5, assign2310_e4168_d_n6, assign2310_e4168_d_n7, assign2310_e4168_d_n8, assign2310_e4168_d_n9, assign2310_e4168_d_n12, assign2310_e4168_d_n14, assign2310_e4168_d_n15, assign2310_e4168_d_n16, assign2310_e4168_d_n17, assign2310_e4168_d_n18, assign2310_e4168_d_n19, assign2310_e4168_d_n20, assign2310_e4168_d_n21, assign2310_e4168_d_n22,) = {
    if ((locals.var_guard358 != 0.0) && (!(((((locals.var_guard353 != 0.0) || (locals.var_guard354 != 0.0)) || (locals.var_guard355 != 0.0)) || (locals.var_guard356 != 0.0)) || (locals.var_guard357 != 0.0)))) {
        let assign2310_e4161: f64 = (p.p96 * p.p37);
        let assign2310_e4162: f64 = (assign2310_e4161).abs();
        let assign2310_e4165: f64 = (locals.var_t0 / locals.var_t1);
        let assign2310_e4166: f64 = (assign2310_e4162 * assign2310_e4165);
        (assign2310_e4166, (assign2310_e4162 * (((locals.var_t0_dn0 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn0)) / (locals.var_t1 * locals.var_t1))), (assign2310_e4162 * (((locals.var_t0_dn1 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn1)) / (locals.var_t1 * locals.var_t1))), (assign2310_e4162 * (((locals.var_t0_dn2 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn2)) / (locals.var_t1 * locals.var_t1))), (assign2310_e4162 * (((locals.var_t0_dn3 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn3)) / (locals.var_t1 * locals.var_t1))), (assign2310_e4162 * (((locals.var_t0_dn4 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn4)) / (locals.var_t1 * locals.var_t1))), (assign2310_e4162 * (((locals.var_t0_dn5 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn5)) / (locals.var_t1 * locals.var_t1))), (assign2310_e4162 * (((locals.var_t0_dn6 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn6)) / (locals.var_t1 * locals.var_t1))), (assign2310_e4162 * (((locals.var_t0_dn7 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn7)) / (locals.var_t1 * locals.var_t1))), (assign2310_e4162 * (((locals.var_t0_dn8 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn8)) / (locals.var_t1 * locals.var_t1))), (assign2310_e4162 * (((locals.var_t0_dn9 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn9)) / (locals.var_t1 * locals.var_t1))), (assign2310_e4162 * (((locals.var_t0_dn12 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn12)) / (locals.var_t1 * locals.var_t1))), (assign2310_e4162 * (((locals.var_t0_dn14 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn14)) / (locals.var_t1 * locals.var_t1))), (assign2310_e4162 * (((locals.var_t0_dn15 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn15)) / (locals.var_t1 * locals.var_t1))), (assign2310_e4162 * (((locals.var_t0_dn16 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn16)) / (locals.var_t1 * locals.var_t1))), (assign2310_e4162 * (((locals.var_t0_dn17 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn17)) / (locals.var_t1 * locals.var_t1))), (assign2310_e4162 * (((locals.var_t0_dn18 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn18)) / (locals.var_t1 * locals.var_t1))), (assign2310_e4162 * (((locals.var_t0_dn19 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn19)) / (locals.var_t1 * locals.var_t1))), (assign2310_e4162 * (((locals.var_t0_dn20 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn20)) / (locals.var_t1 * locals.var_t1))), (assign2310_e4162 * (((locals.var_t0_dn21 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn21)) / (locals.var_t1 * locals.var_t1))), (assign2310_e4162 * (((locals.var_t0_dn22 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn22)) / (locals.var_t1 * locals.var_t1))),)
    } else {
        (locals.var_ns0ddlag, locals.var_ns0ddlag_dn0, locals.var_ns0ddlag_dn1, locals.var_ns0ddlag_dn2, locals.var_ns0ddlag_dn3, locals.var_ns0ddlag_dn4, locals.var_ns0ddlag_dn5, locals.var_ns0ddlag_dn6, locals.var_ns0ddlag_dn7, locals.var_ns0ddlag_dn8, locals.var_ns0ddlag_dn9, locals.var_ns0ddlag_dn12, locals.var_ns0ddlag_dn14, locals.var_ns0ddlag_dn15, locals.var_ns0ddlag_dn16, locals.var_ns0ddlag_dn17, locals.var_ns0ddlag_dn18, locals.var_ns0ddlag_dn19, locals.var_ns0ddlag_dn20, locals.var_ns0ddlag_dn21, locals.var_ns0ddlag_dn22,)
    }
};
        locals.var_ns0ddlag = assign2310_e4168;
        locals.var_ns0ddlag_dn0 = assign2310_e4168_d_n0;
        locals.var_ns0ddlag_dn1 = assign2310_e4168_d_n1;
        locals.var_ns0ddlag_dn2 = assign2310_e4168_d_n2;
        locals.var_ns0ddlag_dn3 = assign2310_e4168_d_n3;
        locals.var_ns0ddlag_dn4 = assign2310_e4168_d_n4;
        locals.var_ns0ddlag_dn5 = assign2310_e4168_d_n5;
        locals.var_ns0ddlag_dn6 = assign2310_e4168_d_n6;
        locals.var_ns0ddlag_dn7 = assign2310_e4168_d_n7;
        locals.var_ns0ddlag_dn8 = assign2310_e4168_d_n8;
        locals.var_ns0ddlag_dn9 = assign2310_e4168_d_n9;
        locals.var_ns0ddlag_dn12 = assign2310_e4168_d_n12;
        locals.var_ns0ddlag_dn14 = assign2310_e4168_d_n14;
        locals.var_ns0ddlag_dn15 = assign2310_e4168_d_n15;
        locals.var_ns0ddlag_dn16 = assign2310_e4168_d_n16;
        locals.var_ns0ddlag_dn17 = assign2310_e4168_d_n17;
        locals.var_ns0ddlag_dn18 = assign2310_e4168_d_n18;
        locals.var_ns0ddlag_dn19 = assign2310_e4168_d_n19;
        locals.var_ns0ddlag_dn20 = assign2310_e4168_d_n20;
        locals.var_ns0ddlag_dn21 = assign2310_e4168_d_n21;
        locals.var_ns0ddlag_dn22 = assign2310_e4168_d_n22;
        locals.var_ns0ddlag_rv = 0.0;

        let (assign2320_e4185, assign2320_e4185_d_n0, assign2320_e4185_d_n1, assign2320_e4185_d_n2, assign2320_e4185_d_n3, assign2320_e4185_d_n4, assign2320_e4185_d_n5, assign2320_e4185_d_n6, assign2320_e4185_d_n7, assign2320_e4185_d_n8, assign2320_e4185_d_n9, assign2320_e4185_d_n12, assign2320_e4185_d_n14, assign2320_e4185_d_n15, assign2320_e4185_d_n16, assign2320_e4185_d_n17, assign2320_e4185_d_n18, assign2320_e4185_d_n19, assign2320_e4185_d_n20, assign2320_e4185_d_n21, assign2320_e4185_d_n22,) = {
    if ((locals.var_guard358 != 0.0) && (!(((((locals.var_guard353 != 0.0) || (locals.var_guard354 != 0.0)) || (locals.var_guard355 != 0.0)) || (locals.var_guard356 != 0.0)) || (locals.var_guard357 != 0.0)))) {
        let assign2320_e4183: f64 = (locals.var_vauy * p.p90);
        (assign2320_e4183, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, (locals.var_vauy_dn6 * p.p90), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn1, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn12, locals.var_t0_dn14, locals.var_t0_dn15, locals.var_t0_dn16, locals.var_t0_dn17, locals.var_t0_dn18, locals.var_t0_dn19, locals.var_t0_dn20, locals.var_t0_dn21, locals.var_t0_dn22,)
    }
};
        locals.var_t0 = assign2320_e4185;
        locals.var_t0_dn0 = assign2320_e4185_d_n0;
        locals.var_t0_dn1 = assign2320_e4185_d_n1;
        locals.var_t0_dn2 = assign2320_e4185_d_n2;
        locals.var_t0_dn3 = assign2320_e4185_d_n3;
        locals.var_t0_dn4 = assign2320_e4185_d_n4;
        locals.var_t0_dn5 = assign2320_e4185_d_n5;
        locals.var_t0_dn6 = assign2320_e4185_d_n6;
        locals.var_t0_dn7 = assign2320_e4185_d_n7;
        locals.var_t0_dn8 = assign2320_e4185_d_n8;
        locals.var_t0_dn9 = assign2320_e4185_d_n9;
        locals.var_t0_dn12 = assign2320_e4185_d_n12;
        locals.var_t0_dn14 = assign2320_e4185_d_n14;
        locals.var_t0_dn15 = assign2320_e4185_d_n15;
        locals.var_t0_dn16 = assign2320_e4185_d_n16;
        locals.var_t0_dn17 = assign2320_e4185_d_n17;
        locals.var_t0_dn18 = assign2320_e4185_d_n18;
        locals.var_t0_dn19 = assign2320_e4185_d_n19;
        locals.var_t0_dn20 = assign2320_e4185_d_n20;
        locals.var_t0_dn21 = assign2320_e4185_d_n21;
        locals.var_t0_dn22 = assign2320_e4185_d_n22;
        locals.var_t0_rv = 0.0;

        let (assign2330_e4207, assign2330_e4207_d_n0, assign2330_e4207_d_n1, assign2330_e4207_d_n2, assign2330_e4207_d_n3, assign2330_e4207_d_n4, assign2330_e4207_d_n5, assign2330_e4207_d_n6, assign2330_e4207_d_n7, assign2330_e4207_d_n8, assign2330_e4207_d_n9, assign2330_e4207_d_n12, assign2330_e4207_d_n14, assign2330_e4207_d_n15, assign2330_e4207_d_n16, assign2330_e4207_d_n17, assign2330_e4207_d_n18, assign2330_e4207_d_n19, assign2330_e4207_d_n20, assign2330_e4207_d_n21, assign2330_e4207_d_n22,) = {
    if ((locals.var_guard358 != 0.0) && (!(((((locals.var_guard353 != 0.0) || (locals.var_guard354 != 0.0)) || (locals.var_guard355 != 0.0)) || (locals.var_guard356 != 0.0)) || (locals.var_guard357 != 0.0)))) {
        let assign2330_e4200: f64 = (locals.var_vauy * locals.var_vauy);
        let assign2330_e4203: f64 = (p.p90 * p.p90);
        let assign2330_e4204: f64 = (assign2330_e4200 + assign2330_e4203);
        let assign2330_e4205: f64 = (assign2330_e4204).sqrt();
        (assign2330_e4205, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, (((locals.var_vauy_dn6 * locals.var_vauy) + (locals.var_vauy * locals.var_vauy_dn6)) / (2.0 * assign2330_e4205)), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn1, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn12, locals.var_t1_dn14, locals.var_t1_dn15, locals.var_t1_dn16, locals.var_t1_dn17, locals.var_t1_dn18, locals.var_t1_dn19, locals.var_t1_dn20, locals.var_t1_dn21, locals.var_t1_dn22,)
    }
};
        locals.var_t1 = assign2330_e4207;
        locals.var_t1_dn0 = assign2330_e4207_d_n0;
        locals.var_t1_dn1 = assign2330_e4207_d_n1;
        locals.var_t1_dn2 = assign2330_e4207_d_n2;
        locals.var_t1_dn3 = assign2330_e4207_d_n3;
        locals.var_t1_dn4 = assign2330_e4207_d_n4;
        locals.var_t1_dn5 = assign2330_e4207_d_n5;
        locals.var_t1_dn6 = assign2330_e4207_d_n6;
        locals.var_t1_dn7 = assign2330_e4207_d_n7;
        locals.var_t1_dn8 = assign2330_e4207_d_n8;
        locals.var_t1_dn9 = assign2330_e4207_d_n9;
        locals.var_t1_dn12 = assign2330_e4207_d_n12;
        locals.var_t1_dn14 = assign2330_e4207_d_n14;
        locals.var_t1_dn15 = assign2330_e4207_d_n15;
        locals.var_t1_dn16 = assign2330_e4207_d_n16;
        locals.var_t1_dn17 = assign2330_e4207_d_n17;
        locals.var_t1_dn18 = assign2330_e4207_d_n18;
        locals.var_t1_dn19 = assign2330_e4207_d_n19;
        locals.var_t1_dn20 = assign2330_e4207_d_n20;
        locals.var_t1_dn21 = assign2330_e4207_d_n21;
        locals.var_t1_dn22 = assign2330_e4207_d_n22;
        locals.var_t1_rv = 0.0;

        let (assign2340_e4229, assign2340_e4229_d_n0, assign2340_e4229_d_n1, assign2340_e4229_d_n2, assign2340_e4229_d_n3, assign2340_e4229_d_n4, assign2340_e4229_d_n5, assign2340_e4229_d_n6, assign2340_e4229_d_n7, assign2340_e4229_d_n8, assign2340_e4229_d_n9, assign2340_e4229_d_n12, assign2340_e4229_d_n14, assign2340_e4229_d_n15, assign2340_e4229_d_n16, assign2340_e4229_d_n17, assign2340_e4229_d_n18, assign2340_e4229_d_n19, assign2340_e4229_d_n20, assign2340_e4229_d_n21, assign2340_e4229_d_n22,) = {
    if ((locals.var_guard358 != 0.0) && (!(((((locals.var_guard353 != 0.0) || (locals.var_guard354 != 0.0)) || (locals.var_guard355 != 0.0)) || (locals.var_guard356 != 0.0)) || (locals.var_guard357 != 0.0)))) {
        let assign2340_e4222: f64 = (p.p92 * p.p10);
        let assign2340_e4223: f64 = (assign2340_e4222).abs();
        let assign2340_e4226: f64 = (locals.var_t0 / locals.var_t1);
        let assign2340_e4227: f64 = (assign2340_e4223 * assign2340_e4226);
        (assign2340_e4227, (assign2340_e4223 * (((locals.var_t0_dn0 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn0)) / (locals.var_t1 * locals.var_t1))), (assign2340_e4223 * (((locals.var_t0_dn1 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn1)) / (locals.var_t1 * locals.var_t1))), (assign2340_e4223 * (((locals.var_t0_dn2 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn2)) / (locals.var_t1 * locals.var_t1))), (assign2340_e4223 * (((locals.var_t0_dn3 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn3)) / (locals.var_t1 * locals.var_t1))), (assign2340_e4223 * (((locals.var_t0_dn4 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn4)) / (locals.var_t1 * locals.var_t1))), (assign2340_e4223 * (((locals.var_t0_dn5 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn5)) / (locals.var_t1 * locals.var_t1))), (assign2340_e4223 * (((locals.var_t0_dn6 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn6)) / (locals.var_t1 * locals.var_t1))), (assign2340_e4223 * (((locals.var_t0_dn7 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn7)) / (locals.var_t1 * locals.var_t1))), (assign2340_e4223 * (((locals.var_t0_dn8 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn8)) / (locals.var_t1 * locals.var_t1))), (assign2340_e4223 * (((locals.var_t0_dn9 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn9)) / (locals.var_t1 * locals.var_t1))), (assign2340_e4223 * (((locals.var_t0_dn12 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn12)) / (locals.var_t1 * locals.var_t1))), (assign2340_e4223 * (((locals.var_t0_dn14 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn14)) / (locals.var_t1 * locals.var_t1))), (assign2340_e4223 * (((locals.var_t0_dn15 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn15)) / (locals.var_t1 * locals.var_t1))), (assign2340_e4223 * (((locals.var_t0_dn16 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn16)) / (locals.var_t1 * locals.var_t1))), (assign2340_e4223 * (((locals.var_t0_dn17 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn17)) / (locals.var_t1 * locals.var_t1))), (assign2340_e4223 * (((locals.var_t0_dn18 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn18)) / (locals.var_t1 * locals.var_t1))), (assign2340_e4223 * (((locals.var_t0_dn19 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn19)) / (locals.var_t1 * locals.var_t1))), (assign2340_e4223 * (((locals.var_t0_dn20 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn20)) / (locals.var_t1 * locals.var_t1))), (assign2340_e4223 * (((locals.var_t0_dn21 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn21)) / (locals.var_t1 * locals.var_t1))), (assign2340_e4223 * (((locals.var_t0_dn22 * locals.var_t1) - (locals.var_t0 * locals.var_t1_dn22)) / (locals.var_t1 * locals.var_t1))),)
    } else {
        (locals.var_voffglag, locals.var_voffglag_dn0, locals.var_voffglag_dn1, locals.var_voffglag_dn2, locals.var_voffglag_dn3, locals.var_voffglag_dn4, locals.var_voffglag_dn5, locals.var_voffglag_dn6, locals.var_voffglag_dn7, locals.var_voffglag_dn8, locals.var_voffglag_dn9, locals.var_voffglag_dn12, locals.var_voffglag_dn14, locals.var_voffglag_dn15, locals.var_voffglag_dn16, locals.var_voffglag_dn17, locals.var_voffglag_dn18, locals.var_voffglag_dn19, locals.var_voffglag_dn20, locals.var_voffglag_dn21, locals.var_voffglag_dn22,)
    }
};
        locals.var_voffglag = assign2340_e4229;
        locals.var_voffglag_dn0 = assign2340_e4229_d_n0;
        locals.var_voffglag_dn1 = assign2340_e4229_d_n1;
        locals.var_voffglag_dn2 = assign2340_e4229_d_n2;
        locals.var_voffglag_dn3 = assign2340_e4229_d_n3;
        locals.var_voffglag_dn4 = assign2340_e4229_d_n4;
        locals.var_voffglag_dn5 = assign2340_e4229_d_n5;
        locals.var_voffglag_dn6 = assign2340_e4229_d_n6;
        locals.var_voffglag_dn7 = assign2340_e4229_d_n7;
        locals.var_voffglag_dn8 = assign2340_e4229_d_n8;
        locals.var_voffglag_dn9 = assign2340_e4229_d_n9;
        locals.var_voffglag_dn12 = assign2340_e4229_d_n12;
        locals.var_voffglag_dn14 = assign2340_e4229_d_n14;
        locals.var_voffglag_dn15 = assign2340_e4229_d_n15;
        locals.var_voffglag_dn16 = assign2340_e4229_d_n16;
        locals.var_voffglag_dn17 = assign2340_e4229_d_n17;
        locals.var_voffglag_dn18 = assign2340_e4229_d_n18;
        locals.var_voffglag_dn19 = assign2340_e4229_d_n19;
        locals.var_voffglag_dn20 = assign2340_e4229_d_n20;
        locals.var_voffglag_dn21 = assign2340_e4229_d_n21;
        locals.var_voffglag_dn22 = assign2340_e4229_d_n22;
        locals.var_voffglag_rv = 0.0;

        let (assign2350_e4246, assign2350_e4246_d_n0, assign2350_e4246_d_n1, assign2350_e4246_d_n2, assign2350_e4246_d_n3, assign2350_e4246_d_n4, assign2350_e4246_d_n5, assign2350_e4246_d_n6, assign2350_e4246_d_n7, assign2350_e4246_d_n8, assign2350_e4246_d_n9, assign2350_e4246_d_n12, assign2350_e4246_d_n14, assign2350_e4246_d_n15, assign2350_e4246_d_n16, assign2350_e4246_d_n17, assign2350_e4246_d_n18, assign2350_e4246_d_n19, assign2350_e4246_d_n20, assign2350_e4246_d_n21, assign2350_e4246_d_n22,) = {
    if ((locals.var_guard358 != 0.0) && (!(((((locals.var_guard353 != 0.0) || (locals.var_guard354 != 0.0)) || (locals.var_guard355 != 0.0)) || (locals.var_guard356 != 0.0)) || (locals.var_guard357 != 0.0)))) {
        let assign2350_e4244: f64 = (locals.var_vauy * p.p90);
        (assign2350_e4244, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, (locals.var_vauy_dn6 * p.p90), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn1, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn12, locals.var_t0_dn14, locals.var_t0_dn15, locals.var_t0_dn16, locals.var_t0_dn17, locals.var_t0_dn18, locals.var_t0_dn19, locals.var_t0_dn20, locals.var_t0_dn21, locals.var_t0_dn22,)
    }
};
        locals.var_t0 = assign2350_e4246;
        locals.var_t0_dn0 = assign2350_e4246_d_n0;
        locals.var_t0_dn1 = assign2350_e4246_d_n1;
        locals.var_t0_dn2 = assign2350_e4246_d_n2;
        locals.var_t0_dn3 = assign2350_e4246_d_n3;
        locals.var_t0_dn4 = assign2350_e4246_d_n4;
        locals.var_t0_dn5 = assign2350_e4246_d_n5;
        locals.var_t0_dn6 = assign2350_e4246_d_n6;
        locals.var_t0_dn7 = assign2350_e4246_d_n7;
        locals.var_t0_dn8 = assign2350_e4246_d_n8;
        locals.var_t0_dn9 = assign2350_e4246_d_n9;
        locals.var_t0_dn12 = assign2350_e4246_d_n12;
        locals.var_t0_dn14 = assign2350_e4246_d_n14;
        locals.var_t0_dn15 = assign2350_e4246_d_n15;
        locals.var_t0_dn16 = assign2350_e4246_d_n16;
        locals.var_t0_dn17 = assign2350_e4246_d_n17;
        locals.var_t0_dn18 = assign2350_e4246_d_n18;
        locals.var_t0_dn19 = assign2350_e4246_d_n19;
        locals.var_t0_dn20 = assign2350_e4246_d_n20;
        locals.var_t0_dn21 = assign2350_e4246_d_n21;
        locals.var_t0_dn22 = assign2350_e4246_d_n22;
        locals.var_t0_rv = 0.0;

    }
}
