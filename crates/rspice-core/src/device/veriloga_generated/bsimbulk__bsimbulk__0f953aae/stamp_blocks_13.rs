#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_105(
        locals: &mut StampLocals,
    ) {
        let (assign32450_e42928, assign32450_e42928_d_n0, assign32450_e42928_d_n2, assign32450_e42928_d_n3, assign32450_e42928_d_n4, assign32450_e42928_d_n5, assign32450_e42928_d_n6, assign32450_e42928_d_n7, assign32450_e42928_d_n8, assign32450_e42928_d_n9, assign32450_e42928_d_n10, assign32450_e42928_d_n11, assign32450_e42928_d_n12, assign32450_e42928_d_n13, assign32450_e42928_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        (locals.var_sqrtpsip, locals.var_sqrtpsip_dn0, locals.var_sqrtpsip_dn2, locals.var_sqrtpsip_dn3, locals.var_sqrtpsip_dn4, locals.var_sqrtpsip_dn5, locals.var_sqrtpsip_dn6, locals.var_sqrtpsip_dn7, locals.var_sqrtpsip_dn8, locals.var_sqrtpsip_dn9, locals.var_sqrtpsip_dn10, locals.var_sqrtpsip_dn11, locals.var_sqrtpsip_dn12, locals.var_sqrtpsip_dn13, locals.var_sqrtpsip_dn14,)
    } else {
        (locals.var_sqrtpsisa, locals.var_sqrtpsisa_dn0, locals.var_sqrtpsisa_dn2, locals.var_sqrtpsisa_dn3, locals.var_sqrtpsisa_dn4, locals.var_sqrtpsisa_dn5, locals.var_sqrtpsisa_dn6, locals.var_sqrtpsisa_dn7, locals.var_sqrtpsisa_dn8, locals.var_sqrtpsisa_dn9, locals.var_sqrtpsisa_dn10, locals.var_sqrtpsisa_dn11, locals.var_sqrtpsisa_dn12, locals.var_sqrtpsisa_dn13, locals.var_sqrtpsisa_dn14,)
    }
};
        locals.var_sqrtpsisa = assign32450_e42928;
        locals.var_sqrtpsisa_dn0 = assign32450_e42928_d_n0;
        locals.var_sqrtpsisa_dn2 = assign32450_e42928_d_n2;
        locals.var_sqrtpsisa_dn3 = assign32450_e42928_d_n3;
        locals.var_sqrtpsisa_dn4 = assign32450_e42928_d_n4;
        locals.var_sqrtpsisa_dn5 = assign32450_e42928_d_n5;
        locals.var_sqrtpsisa_dn6 = assign32450_e42928_d_n6;
        locals.var_sqrtpsisa_dn7 = assign32450_e42928_d_n7;
        locals.var_sqrtpsisa_dn8 = assign32450_e42928_d_n8;
        locals.var_sqrtpsisa_dn9 = assign32450_e42928_d_n9;
        locals.var_sqrtpsisa_dn10 = assign32450_e42928_d_n10;
        locals.var_sqrtpsisa_dn11 = assign32450_e42928_d_n11;
        locals.var_sqrtpsisa_dn12 = assign32450_e42928_d_n12;
        locals.var_sqrtpsisa_dn13 = assign32450_e42928_d_n13;
        locals.var_sqrtpsisa_dn14 = assign32450_e42928_d_n14;
        locals.var_sqrtpsisa_rv = 0.0;

        let assign32460_e42931: f64 = (-68.0);
        let assign32460_e42932: f64 = if locals.var_t8 <= assign32460_e42931 { 1.0 } else { 0.0 };
        locals.var_guard740 = assign32460_e42932;
        locals.var_guard740_rv = 0.0;

        let (assign32470_e42939, assign32470_e42939_d_n0, assign32470_e42939_d_n2, assign32470_e42939_d_n3, assign32470_e42939_d_n4, assign32470_e42939_d_n5, assign32470_e42939_d_n6, assign32470_e42939_d_n7, assign32470_e42939_d_n8, assign32470_e42939_d_n9, assign32470_e42939_d_n10, assign32470_e42939_d_n11, assign32470_e42939_d_n12, assign32470_e42939_d_n13, assign32470_e42939_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard740 != 0.0)) {
        let assign32470_e42937: f64 = (-100.0);
        (assign32470_e42937, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign32470_e42939;
        locals.var_t4_dn0 = assign32470_e42939_d_n0;
        locals.var_t4_dn2 = assign32470_e42939_d_n2;
        locals.var_t4_dn3 = assign32470_e42939_d_n3;
        locals.var_t4_dn4 = assign32470_e42939_d_n4;
        locals.var_t4_dn5 = assign32470_e42939_d_n5;
        locals.var_t4_dn6 = assign32470_e42939_d_n6;
        locals.var_t4_dn7 = assign32470_e42939_d_n7;
        locals.var_t4_dn8 = assign32470_e42939_d_n8;
        locals.var_t4_dn9 = assign32470_e42939_d_n9;
        locals.var_t4_dn10 = assign32470_e42939_d_n10;
        locals.var_t4_dn11 = assign32470_e42939_d_n11;
        locals.var_t4_dn12 = assign32470_e42939_d_n12;
        locals.var_t4_dn13 = assign32470_e42939_d_n13;
        locals.var_t4_dn14 = assign32470_e42939_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign32480_e42945, assign32480_e42945_d_n0, assign32480_e42945_d_n2, assign32480_e42945_d_n3, assign32480_e42945_d_n4, assign32480_e42945_d_n5, assign32480_e42945_d_n6, assign32480_e42945_d_n7, assign32480_e42945_d_n8, assign32480_e42945_d_n9, assign32480_e42945_d_n10, assign32480_e42945_d_n11, assign32480_e42945_d_n12, assign32480_e42945_d_n13, assign32480_e42945_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard740 != 0.0)) {
        (20.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn13, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign32480_e42945;
        locals.var_t5_dn0 = assign32480_e42945_d_n0;
        locals.var_t5_dn2 = assign32480_e42945_d_n2;
        locals.var_t5_dn3 = assign32480_e42945_d_n3;
        locals.var_t5_dn4 = assign32480_e42945_d_n4;
        locals.var_t5_dn5 = assign32480_e42945_d_n5;
        locals.var_t5_dn6 = assign32480_e42945_d_n6;
        locals.var_t5_dn7 = assign32480_e42945_d_n7;
        locals.var_t5_dn8 = assign32480_e42945_d_n8;
        locals.var_t5_dn9 = assign32480_e42945_d_n9;
        locals.var_t5_dn10 = assign32480_e42945_d_n10;
        locals.var_t5_dn11 = assign32480_e42945_d_n11;
        locals.var_t5_dn12 = assign32480_e42945_d_n12;
        locals.var_t5_dn13 = assign32480_e42945_d_n13;
        locals.var_t5_dn14 = assign32480_e42945_d_n14;
        locals.var_t5_rv = 0.0;

        let assign32490_e42950: f64 = (0.5 * locals.var_t5);
        let assign32490_e42951: f64 = (locals.var_t4 - assign32490_e42950);
        let assign32490_e42952: f64 = if locals.var_t8 < assign32490_e42951 { 1.0 } else { 0.0 };
        locals.var_guard741 = assign32490_e42952;
        locals.var_guard741_rv = 0.0;

        let (assign32500_e42961, assign32500_e42961_d_n0, assign32500_e42961_d_n2, assign32500_e42961_d_n3, assign32500_e42961_d_n4, assign32500_e42961_d_n5, assign32500_e42961_d_n6, assign32500_e42961_d_n7, assign32500_e42961_d_n8, assign32500_e42961_d_n9, assign32500_e42961_d_n10, assign32500_e42961_d_n11, assign32500_e42961_d_n12, assign32500_e42961_d_n13, assign32500_e42961_d_n14,) = {
    if (((locals.var_guard730 != 0.0) && (locals.var_guard740 != 0.0)) && (locals.var_guard741 != 0.0)) {
        let assign32500_e42959: f64 = { let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign32500_e42959, ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn0), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn2), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn3), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn4), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn5), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn6), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn7), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn8), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn9), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn10), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn11), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn12), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn13), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn14),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign32500_e42961;
        locals.var_t3_dn0 = assign32500_e42961_d_n0;
        locals.var_t3_dn2 = assign32500_e42961_d_n2;
        locals.var_t3_dn3 = assign32500_e42961_d_n3;
        locals.var_t3_dn4 = assign32500_e42961_d_n4;
        locals.var_t3_dn5 = assign32500_e42961_d_n5;
        locals.var_t3_dn6 = assign32500_e42961_d_n6;
        locals.var_t3_dn7 = assign32500_e42961_d_n7;
        locals.var_t3_dn8 = assign32500_e42961_d_n8;
        locals.var_t3_dn9 = assign32500_e42961_d_n9;
        locals.var_t3_dn10 = assign32500_e42961_d_n10;
        locals.var_t3_dn11 = assign32500_e42961_d_n11;
        locals.var_t3_dn12 = assign32500_e42961_d_n12;
        locals.var_t3_dn13 = assign32500_e42961_d_n13;
        locals.var_t3_dn14 = assign32500_e42961_d_n14;
        locals.var_t3_rv = 0.0;

        let assign32510_e42966: f64 = (0.5 * locals.var_t5);
        let assign32510_e42967: f64 = (locals.var_t4 + assign32510_e42966);
        let assign32510_e42968: f64 = if locals.var_t8 > assign32510_e42967 { 1.0 } else { 0.0 };
        locals.var_guard742 = assign32510_e42968;
        locals.var_guard742_rv = 0.0;

        let (assign32520_e42980, assign32520_e42980_d_n0, assign32520_e42980_d_n2, assign32520_e42980_d_n3, assign32520_e42980_d_n4, assign32520_e42980_d_n5, assign32520_e42980_d_n6, assign32520_e42980_d_n7, assign32520_e42980_d_n8, assign32520_e42980_d_n9, assign32520_e42980_d_n10, assign32520_e42980_d_n11, assign32520_e42980_d_n12, assign32520_e42980_d_n13, assign32520_e42980_d_n14,) = {
    if ((((locals.var_guard730 != 0.0) && (locals.var_guard740 != 0.0)) && (locals.var_guard741 == 0.0)) && (locals.var_guard742 != 0.0)) {
        let assign32520_e42978: f64 = { let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign32520_e42978, ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn0), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn2), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn3), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn4), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn5), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn6), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn7), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn8), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn9), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn10), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn11), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn12), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn13), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn14),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign32520_e42980;
        locals.var_t3_dn0 = assign32520_e42980_d_n0;
        locals.var_t3_dn2 = assign32520_e42980_d_n2;
        locals.var_t3_dn3 = assign32520_e42980_d_n3;
        locals.var_t3_dn4 = assign32520_e42980_d_n4;
        locals.var_t3_dn5 = assign32520_e42980_d_n5;
        locals.var_t3_dn6 = assign32520_e42980_d_n6;
        locals.var_t3_dn7 = assign32520_e42980_d_n7;
        locals.var_t3_dn8 = assign32520_e42980_d_n8;
        locals.var_t3_dn9 = assign32520_e42980_d_n9;
        locals.var_t3_dn10 = assign32520_e42980_d_n10;
        locals.var_t3_dn11 = assign32520_e42980_d_n11;
        locals.var_t3_dn12 = assign32520_e42980_d_n12;
        locals.var_t3_dn13 = assign32520_e42980_d_n13;
        locals.var_t3_dn14 = assign32520_e42980_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign32530_e42996, assign32530_e42996_d_n0, assign32530_e42996_d_n2, assign32530_e42996_d_n3, assign32530_e42996_d_n4, assign32530_e42996_d_n5, assign32530_e42996_d_n6, assign32530_e42996_d_n7, assign32530_e42996_d_n8, assign32530_e42996_d_n9, assign32530_e42996_d_n10, assign32530_e42996_d_n11, assign32530_e42996_d_n12, assign32530_e42996_d_n13, assign32530_e42996_d_n14,) = {
    if ((((locals.var_guard730 != 0.0) && (locals.var_guard740 != 0.0)) && (locals.var_guard741 == 0.0)) && (locals.var_guard742 == 0.0)) {
        let assign32530_e42992: f64 = (locals.var_t8 - locals.var_t4);
        let assign32530_e42994: f64 = (assign32530_e42992 / locals.var_t5);
        (assign32530_e42994, ((((locals.var_t8_dn0 - locals.var_t4_dn0) * locals.var_t5) - (assign32530_e42992 * locals.var_t5_dn0)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn2 - locals.var_t4_dn2) * locals.var_t5) - (assign32530_e42992 * locals.var_t5_dn2)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn3 - locals.var_t4_dn3) * locals.var_t5) - (assign32530_e42992 * locals.var_t5_dn3)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn4 - locals.var_t4_dn4) * locals.var_t5) - (assign32530_e42992 * locals.var_t5_dn4)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn5 - locals.var_t4_dn5) * locals.var_t5) - (assign32530_e42992 * locals.var_t5_dn5)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn6 - locals.var_t4_dn6) * locals.var_t5) - (assign32530_e42992 * locals.var_t5_dn6)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn7 - locals.var_t4_dn7) * locals.var_t5) - (assign32530_e42992 * locals.var_t5_dn7)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn8 - locals.var_t4_dn8) * locals.var_t5) - (assign32530_e42992 * locals.var_t5_dn8)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn9 - locals.var_t4_dn9) * locals.var_t5) - (assign32530_e42992 * locals.var_t5_dn9)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn10 - locals.var_t4_dn10) * locals.var_t5) - (assign32530_e42992 * locals.var_t5_dn10)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn11 - locals.var_t4_dn11) * locals.var_t5) - (assign32530_e42992 * locals.var_t5_dn11)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn12 - locals.var_t4_dn12) * locals.var_t5) - (assign32530_e42992 * locals.var_t5_dn12)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn13 - locals.var_t4_dn13) * locals.var_t5) - (assign32530_e42992 * locals.var_t5_dn13)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn14 - locals.var_t4_dn14) * locals.var_t5) - (assign32530_e42992 * locals.var_t5_dn14)) / (locals.var_t5 * locals.var_t5)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign32530_e42996;
        locals.var_t2_dn0 = assign32530_e42996_d_n0;
        locals.var_t2_dn2 = assign32530_e42996_d_n2;
        locals.var_t2_dn3 = assign32530_e42996_d_n3;
        locals.var_t2_dn4 = assign32530_e42996_d_n4;
        locals.var_t2_dn5 = assign32530_e42996_d_n5;
        locals.var_t2_dn6 = assign32530_e42996_d_n6;
        locals.var_t2_dn7 = assign32530_e42996_d_n7;
        locals.var_t2_dn8 = assign32530_e42996_d_n8;
        locals.var_t2_dn9 = assign32530_e42996_d_n9;
        locals.var_t2_dn10 = assign32530_e42996_d_n10;
        locals.var_t2_dn11 = assign32530_e42996_d_n11;
        locals.var_t2_dn12 = assign32530_e42996_d_n12;
        locals.var_t2_dn13 = assign32530_e42996_d_n13;
        locals.var_t2_dn14 = assign32530_e42996_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign32540_e43010, assign32540_e43010_d_n0, assign32540_e43010_d_n2, assign32540_e43010_d_n3, assign32540_e43010_d_n4, assign32540_e43010_d_n5, assign32540_e43010_d_n6, assign32540_e43010_d_n7, assign32540_e43010_d_n8, assign32540_e43010_d_n9, assign32540_e43010_d_n10, assign32540_e43010_d_n11, assign32540_e43010_d_n12, assign32540_e43010_d_n13, assign32540_e43010_d_n14,) = {
    if ((((locals.var_guard730 != 0.0) && (locals.var_guard740 != 0.0)) && (locals.var_guard741 == 0.0)) && (locals.var_guard742 == 0.0)) {
        let assign32540_e43008: f64 = (locals.var_t2 * locals.var_t2);
        (assign32540_e43008, ((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)), ((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)), ((locals.var_t2_dn3 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn3)), ((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)), ((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)), ((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)), ((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)), ((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)), ((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)), ((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)), ((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)), ((locals.var_t2_dn12 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn12)), ((locals.var_t2_dn13 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn13)), ((locals.var_t2_dn14 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn14)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12, locals.var_t6_dn13, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign32540_e43010;
        locals.var_t6_dn0 = assign32540_e43010_d_n0;
        locals.var_t6_dn2 = assign32540_e43010_d_n2;
        locals.var_t6_dn3 = assign32540_e43010_d_n3;
        locals.var_t6_dn4 = assign32540_e43010_d_n4;
        locals.var_t6_dn5 = assign32540_e43010_d_n5;
        locals.var_t6_dn6 = assign32540_e43010_d_n6;
        locals.var_t6_dn7 = assign32540_e43010_d_n7;
        locals.var_t6_dn8 = assign32540_e43010_d_n8;
        locals.var_t6_dn9 = assign32540_e43010_d_n9;
        locals.var_t6_dn10 = assign32540_e43010_d_n10;
        locals.var_t6_dn11 = assign32540_e43010_d_n11;
        locals.var_t6_dn12 = assign32540_e43010_d_n12;
        locals.var_t6_dn13 = assign32540_e43010_d_n13;
        locals.var_t6_dn14 = assign32540_e43010_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign32550_e43045, assign32550_e43045_d_n0, assign32550_e43045_d_n2, assign32550_e43045_d_n3, assign32550_e43045_d_n4, assign32550_e43045_d_n5, assign32550_e43045_d_n6, assign32550_e43045_d_n7, assign32550_e43045_d_n8, assign32550_e43045_d_n9, assign32550_e43045_d_n10, assign32550_e43045_d_n11, assign32550_e43045_d_n12, assign32550_e43045_d_n13, assign32550_e43045_d_n14,) = {
    if ((((locals.var_guard730 != 0.0) && (locals.var_guard740 != 0.0)) && (locals.var_guard741 == 0.0)) && (locals.var_guard742 == 0.0)) {
        let assign32550_e43024: f64 = (5.0 / 64.0);
        let assign32550_e43027: f64 = (0.5 * locals.var_t2);
        let assign32550_e43028: f64 = (assign32550_e43024 + assign32550_e43027);
        let assign32550_e43032: f64 = (15.0 / 16.0);
        let assign32550_e43036: f64 = (1.25 - locals.var_t6);
        let assign32550_e43037: f64 = (locals.var_t6 * assign32550_e43036);
        let assign32550_e43038: f64 = (assign32550_e43032 - assign32550_e43037);
        let assign32550_e43039: f64 = (locals.var_t6 * assign32550_e43038);
        let assign32550_e43040: f64 = (assign32550_e43028 + assign32550_e43039);
        let assign32550_e43041: f64 = (locals.var_t5 * assign32550_e43040);
        let assign32550_e43042: f64 = (locals.var_t4 + assign32550_e43041);
        let assign32550_e43043: f64 = { let limited_exp_arg = assign32550_e43042; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign32550_e43043, ({ let limited_exp_arg = assign32550_e43042; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn0 + ((locals.var_t5_dn0 * assign32550_e43040) + (locals.var_t5 * ((0.5 * locals.var_t2_dn0) + ((locals.var_t6_dn0 * assign32550_e43038) + (locals.var_t6 * (-((locals.var_t6_dn0 * assign32550_e43036) + (locals.var_t6 * (-locals.var_t6_dn0))))))))))), ({ let limited_exp_arg = assign32550_e43042; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn2 + ((locals.var_t5_dn2 * assign32550_e43040) + (locals.var_t5 * ((0.5 * locals.var_t2_dn2) + ((locals.var_t6_dn2 * assign32550_e43038) + (locals.var_t6 * (-((locals.var_t6_dn2 * assign32550_e43036) + (locals.var_t6 * (-locals.var_t6_dn2))))))))))), ({ let limited_exp_arg = assign32550_e43042; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn3 + ((locals.var_t5_dn3 * assign32550_e43040) + (locals.var_t5 * ((0.5 * locals.var_t2_dn3) + ((locals.var_t6_dn3 * assign32550_e43038) + (locals.var_t6 * (-((locals.var_t6_dn3 * assign32550_e43036) + (locals.var_t6 * (-locals.var_t6_dn3))))))))))), ({ let limited_exp_arg = assign32550_e43042; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn4 + ((locals.var_t5_dn4 * assign32550_e43040) + (locals.var_t5 * ((0.5 * locals.var_t2_dn4) + ((locals.var_t6_dn4 * assign32550_e43038) + (locals.var_t6 * (-((locals.var_t6_dn4 * assign32550_e43036) + (locals.var_t6 * (-locals.var_t6_dn4))))))))))), ({ let limited_exp_arg = assign32550_e43042; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn5 + ((locals.var_t5_dn5 * assign32550_e43040) + (locals.var_t5 * ((0.5 * locals.var_t2_dn5) + ((locals.var_t6_dn5 * assign32550_e43038) + (locals.var_t6 * (-((locals.var_t6_dn5 * assign32550_e43036) + (locals.var_t6 * (-locals.var_t6_dn5))))))))))), ({ let limited_exp_arg = assign32550_e43042; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn6 + ((locals.var_t5_dn6 * assign32550_e43040) + (locals.var_t5 * ((0.5 * locals.var_t2_dn6) + ((locals.var_t6_dn6 * assign32550_e43038) + (locals.var_t6 * (-((locals.var_t6_dn6 * assign32550_e43036) + (locals.var_t6 * (-locals.var_t6_dn6))))))))))), ({ let limited_exp_arg = assign32550_e43042; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn7 + ((locals.var_t5_dn7 * assign32550_e43040) + (locals.var_t5 * ((0.5 * locals.var_t2_dn7) + ((locals.var_t6_dn7 * assign32550_e43038) + (locals.var_t6 * (-((locals.var_t6_dn7 * assign32550_e43036) + (locals.var_t6 * (-locals.var_t6_dn7))))))))))), ({ let limited_exp_arg = assign32550_e43042; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn8 + ((locals.var_t5_dn8 * assign32550_e43040) + (locals.var_t5 * ((0.5 * locals.var_t2_dn8) + ((locals.var_t6_dn8 * assign32550_e43038) + (locals.var_t6 * (-((locals.var_t6_dn8 * assign32550_e43036) + (locals.var_t6 * (-locals.var_t6_dn8))))))))))), ({ let limited_exp_arg = assign32550_e43042; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn9 + ((locals.var_t5_dn9 * assign32550_e43040) + (locals.var_t5 * ((0.5 * locals.var_t2_dn9) + ((locals.var_t6_dn9 * assign32550_e43038) + (locals.var_t6 * (-((locals.var_t6_dn9 * assign32550_e43036) + (locals.var_t6 * (-locals.var_t6_dn9))))))))))), ({ let limited_exp_arg = assign32550_e43042; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn10 + ((locals.var_t5_dn10 * assign32550_e43040) + (locals.var_t5 * ((0.5 * locals.var_t2_dn10) + ((locals.var_t6_dn10 * assign32550_e43038) + (locals.var_t6 * (-((locals.var_t6_dn10 * assign32550_e43036) + (locals.var_t6 * (-locals.var_t6_dn10))))))))))), ({ let limited_exp_arg = assign32550_e43042; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn11 + ((locals.var_t5_dn11 * assign32550_e43040) + (locals.var_t5 * ((0.5 * locals.var_t2_dn11) + ((locals.var_t6_dn11 * assign32550_e43038) + (locals.var_t6 * (-((locals.var_t6_dn11 * assign32550_e43036) + (locals.var_t6 * (-locals.var_t6_dn11))))))))))), ({ let limited_exp_arg = assign32550_e43042; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn12 + ((locals.var_t5_dn12 * assign32550_e43040) + (locals.var_t5 * ((0.5 * locals.var_t2_dn12) + ((locals.var_t6_dn12 * assign32550_e43038) + (locals.var_t6 * (-((locals.var_t6_dn12 * assign32550_e43036) + (locals.var_t6 * (-locals.var_t6_dn12))))))))))), ({ let limited_exp_arg = assign32550_e43042; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn13 + ((locals.var_t5_dn13 * assign32550_e43040) + (locals.var_t5 * ((0.5 * locals.var_t2_dn13) + ((locals.var_t6_dn13 * assign32550_e43038) + (locals.var_t6 * (-((locals.var_t6_dn13 * assign32550_e43036) + (locals.var_t6 * (-locals.var_t6_dn13))))))))))), ({ let limited_exp_arg = assign32550_e43042; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn14 + ((locals.var_t5_dn14 * assign32550_e43040) + (locals.var_t5 * ((0.5 * locals.var_t2_dn14) + ((locals.var_t6_dn14 * assign32550_e43038) + (locals.var_t6 * (-((locals.var_t6_dn14 * assign32550_e43036) + (locals.var_t6 * (-locals.var_t6_dn14))))))))))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign32550_e43045;
        locals.var_t3_dn0 = assign32550_e43045_d_n0;
        locals.var_t3_dn2 = assign32550_e43045_d_n2;
        locals.var_t3_dn3 = assign32550_e43045_d_n3;
        locals.var_t3_dn4 = assign32550_e43045_d_n4;
        locals.var_t3_dn5 = assign32550_e43045_d_n5;
        locals.var_t3_dn6 = assign32550_e43045_d_n6;
        locals.var_t3_dn7 = assign32550_e43045_d_n7;
        locals.var_t3_dn8 = assign32550_e43045_d_n8;
        locals.var_t3_dn9 = assign32550_e43045_d_n9;
        locals.var_t3_dn10 = assign32550_e43045_d_n10;
        locals.var_t3_dn11 = assign32550_e43045_d_n11;
        locals.var_t3_dn12 = assign32550_e43045_d_n12;
        locals.var_t3_dn13 = assign32550_e43045_d_n13;
        locals.var_t3_dn14 = assign32550_e43045_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign32560_e43078, assign32560_e43078_d_n0, assign32560_e43078_d_n2, assign32560_e43078_d_n3, assign32560_e43078_d_n4, assign32560_e43078_d_n5, assign32560_e43078_d_n6, assign32560_e43078_d_n7, assign32560_e43078_d_n8, assign32560_e43078_d_n9, assign32560_e43078_d_n10, assign32560_e43078_d_n11, assign32560_e43078_d_n12, assign32560_e43078_d_n13, assign32560_e43078_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard740 != 0.0)) {
        let assign32560_e43052: f64 = (1.0 + locals.var_t1);
        let assign32560_e43055: f64 = locals.var_t8;
        let assign32560_e43056: f64 = (assign32560_e43052 - assign32560_e43055);
        let assign32560_e43060: f64 = (2.0 * locals.var_t0);
        let assign32560_e43063: f64 = (locals.var_t3 * 2.0);
        let assign32560_e43065: f64 = (assign32560_e43063 * locals.var_t0);
        let assign32560_e43068: f64 = (2.0 * locals.var_sqrtpsisa);
        let assign32560_e43069: f64 = (assign32560_e43065 + assign32560_e43068);
        let assign32560_e43070: f64 = (assign32560_e43060 * assign32560_e43069);
        let assign32560_e43072: f64 = (assign32560_e43070).max(1e-38);
        let assign32560_e43073: f64 = (assign32560_e43072).ln();
        let assign32560_e43074: f64 = assign32560_e43073;
        let assign32560_e43075: f64 = (assign32560_e43056 - assign32560_e43074);
        let assign32560_e43076: f64 = (locals.var_t3 * assign32560_e43075);
        (assign32560_e43076, ((locals.var_t3_dn0 * assign32560_e43075) + (locals.var_t3 * ((locals.var_t1_dn0 - locals.var_t8_dn0) - (if assign32560_e43070 >= 1e-38 { (((2.0 * locals.var_t0_dn0) * assign32560_e43069) + (assign32560_e43060 * ((((locals.var_t3_dn0 * 2.0) * locals.var_t0) + (assign32560_e43063 * locals.var_t0_dn0)) + (2.0 * locals.var_sqrtpsisa_dn0)))) } else { 0.0 } / assign32560_e43072)))), ((locals.var_t3_dn2 * assign32560_e43075) + (locals.var_t3 * ((locals.var_t1_dn2 - locals.var_t8_dn2) - (if assign32560_e43070 >= 1e-38 { (((2.0 * locals.var_t0_dn2) * assign32560_e43069) + (assign32560_e43060 * ((((locals.var_t3_dn2 * 2.0) * locals.var_t0) + (assign32560_e43063 * locals.var_t0_dn2)) + (2.0 * locals.var_sqrtpsisa_dn2)))) } else { 0.0 } / assign32560_e43072)))), ((locals.var_t3_dn3 * assign32560_e43075) + (locals.var_t3 * ((locals.var_t1_dn3 - locals.var_t8_dn3) - (if assign32560_e43070 >= 1e-38 { (((2.0 * locals.var_t0_dn3) * assign32560_e43069) + (assign32560_e43060 * ((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign32560_e43063 * locals.var_t0_dn3)) + (2.0 * locals.var_sqrtpsisa_dn3)))) } else { 0.0 } / assign32560_e43072)))), ((locals.var_t3_dn4 * assign32560_e43075) + (locals.var_t3 * ((locals.var_t1_dn4 - locals.var_t8_dn4) - (if assign32560_e43070 >= 1e-38 { (((2.0 * locals.var_t0_dn4) * assign32560_e43069) + (assign32560_e43060 * ((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign32560_e43063 * locals.var_t0_dn4)) + (2.0 * locals.var_sqrtpsisa_dn4)))) } else { 0.0 } / assign32560_e43072)))), ((locals.var_t3_dn5 * assign32560_e43075) + (locals.var_t3 * ((locals.var_t1_dn5 - locals.var_t8_dn5) - (if assign32560_e43070 >= 1e-38 { (((2.0 * locals.var_t0_dn5) * assign32560_e43069) + (assign32560_e43060 * ((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign32560_e43063 * locals.var_t0_dn5)) + (2.0 * locals.var_sqrtpsisa_dn5)))) } else { 0.0 } / assign32560_e43072)))), ((locals.var_t3_dn6 * assign32560_e43075) + (locals.var_t3 * ((locals.var_t1_dn6 - locals.var_t8_dn6) - (if assign32560_e43070 >= 1e-38 { (((2.0 * locals.var_t0_dn6) * assign32560_e43069) + (assign32560_e43060 * ((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign32560_e43063 * locals.var_t0_dn6)) + (2.0 * locals.var_sqrtpsisa_dn6)))) } else { 0.0 } / assign32560_e43072)))), ((locals.var_t3_dn7 * assign32560_e43075) + (locals.var_t3 * ((locals.var_t1_dn7 - locals.var_t8_dn7) - (if assign32560_e43070 >= 1e-38 { (((2.0 * locals.var_t0_dn7) * assign32560_e43069) + (assign32560_e43060 * ((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign32560_e43063 * locals.var_t0_dn7)) + (2.0 * locals.var_sqrtpsisa_dn7)))) } else { 0.0 } / assign32560_e43072)))), ((locals.var_t3_dn8 * assign32560_e43075) + (locals.var_t3 * ((locals.var_t1_dn8 - locals.var_t8_dn8) - (if assign32560_e43070 >= 1e-38 { (((2.0 * locals.var_t0_dn8) * assign32560_e43069) + (assign32560_e43060 * ((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign32560_e43063 * locals.var_t0_dn8)) + (2.0 * locals.var_sqrtpsisa_dn8)))) } else { 0.0 } / assign32560_e43072)))), ((locals.var_t3_dn9 * assign32560_e43075) + (locals.var_t3 * ((locals.var_t1_dn9 - locals.var_t8_dn9) - (if assign32560_e43070 >= 1e-38 { (((2.0 * locals.var_t0_dn9) * assign32560_e43069) + (assign32560_e43060 * ((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign32560_e43063 * locals.var_t0_dn9)) + (2.0 * locals.var_sqrtpsisa_dn9)))) } else { 0.0 } / assign32560_e43072)))), ((locals.var_t3_dn10 * assign32560_e43075) + (locals.var_t3 * ((locals.var_t1_dn10 - locals.var_t8_dn10) - (if assign32560_e43070 >= 1e-38 { (((2.0 * locals.var_t0_dn10) * assign32560_e43069) + (assign32560_e43060 * ((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign32560_e43063 * locals.var_t0_dn10)) + (2.0 * locals.var_sqrtpsisa_dn10)))) } else { 0.0 } / assign32560_e43072)))), ((locals.var_t3_dn11 * assign32560_e43075) + (locals.var_t3 * ((locals.var_t1_dn11 - locals.var_t8_dn11) - (if assign32560_e43070 >= 1e-38 { (((2.0 * locals.var_t0_dn11) * assign32560_e43069) + (assign32560_e43060 * ((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign32560_e43063 * locals.var_t0_dn11)) + (2.0 * locals.var_sqrtpsisa_dn11)))) } else { 0.0 } / assign32560_e43072)))), ((locals.var_t3_dn12 * assign32560_e43075) + (locals.var_t3 * ((locals.var_t1_dn12 - locals.var_t8_dn12) - (if assign32560_e43070 >= 1e-38 { (((2.0 * locals.var_t0_dn12) * assign32560_e43069) + (assign32560_e43060 * ((((locals.var_t3_dn12 * 2.0) * locals.var_t0) + (assign32560_e43063 * locals.var_t0_dn12)) + (2.0 * locals.var_sqrtpsisa_dn12)))) } else { 0.0 } / assign32560_e43072)))), ((locals.var_t3_dn13 * assign32560_e43075) + (locals.var_t3 * ((locals.var_t1_dn13 - locals.var_t8_dn13) - (if assign32560_e43070 >= 1e-38 { (((2.0 * locals.var_t0_dn13) * assign32560_e43069) + (assign32560_e43060 * ((((locals.var_t3_dn13 * 2.0) * locals.var_t0) + (assign32560_e43063 * locals.var_t0_dn13)) + (2.0 * locals.var_sqrtpsisa_dn13)))) } else { 0.0 } / assign32560_e43072)))), ((locals.var_t3_dn14 * assign32560_e43075) + (locals.var_t3 * ((locals.var_t1_dn14 - locals.var_t8_dn14) - (if assign32560_e43070 >= 1e-38 { (((2.0 * locals.var_t0_dn14) * assign32560_e43069) + (assign32560_e43060 * ((((locals.var_t3_dn14 * 2.0) * locals.var_t0) + (assign32560_e43063 * locals.var_t0_dn14)) + (2.0 * locals.var_sqrtpsisa_dn14)))) } else { 0.0 } / assign32560_e43072)))),)
    } else {
        (locals.var_qdeff_edge, locals.var_qdeff_edge_dn0, locals.var_qdeff_edge_dn2, locals.var_qdeff_edge_dn3, locals.var_qdeff_edge_dn4, locals.var_qdeff_edge_dn5, locals.var_qdeff_edge_dn6, locals.var_qdeff_edge_dn7, locals.var_qdeff_edge_dn8, locals.var_qdeff_edge_dn9, locals.var_qdeff_edge_dn10, locals.var_qdeff_edge_dn11, locals.var_qdeff_edge_dn12, locals.var_qdeff_edge_dn13, locals.var_qdeff_edge_dn14,)
    }
};
        locals.var_qdeff_edge = assign32560_e43078;
        locals.var_qdeff_edge_dn0 = assign32560_e43078_d_n0;
        locals.var_qdeff_edge_dn2 = assign32560_e43078_d_n2;
        locals.var_qdeff_edge_dn3 = assign32560_e43078_d_n3;
        locals.var_qdeff_edge_dn4 = assign32560_e43078_d_n4;
        locals.var_qdeff_edge_dn5 = assign32560_e43078_d_n5;
        locals.var_qdeff_edge_dn6 = assign32560_e43078_d_n6;
        locals.var_qdeff_edge_dn7 = assign32560_e43078_d_n7;
        locals.var_qdeff_edge_dn8 = assign32560_e43078_d_n8;
        locals.var_qdeff_edge_dn9 = assign32560_e43078_d_n9;
        locals.var_qdeff_edge_dn10 = assign32560_e43078_d_n10;
        locals.var_qdeff_edge_dn11 = assign32560_e43078_d_n11;
        locals.var_qdeff_edge_dn12 = assign32560_e43078_d_n12;
        locals.var_qdeff_edge_dn13 = assign32560_e43078_d_n13;
        locals.var_qdeff_edge_dn14 = assign32560_e43078_d_n14;
        locals.var_qdeff_edge_rv = 0.0;

        let (assign32570_e43086, assign32570_e43086_d_n0, assign32570_e43086_d_n2, assign32570_e43086_d_n3, assign32570_e43086_d_n4, assign32570_e43086_d_n5, assign32570_e43086_d_n6, assign32570_e43086_d_n7, assign32570_e43086_d_n8, assign32570_e43086_d_n9, assign32570_e43086_d_n10, assign32570_e43086_d_n11, assign32570_e43086_d_n12, assign32570_e43086_d_n13, assign32570_e43086_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard740 == 0.0)) {
        let assign32570_e43084: f64 = { let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign32570_e43084, ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn0), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn2), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn3), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn4), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn5), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn6), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn7), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn8), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn9), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn10), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn11), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn12), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn13), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn14),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign32570_e43086;
        locals.var_t3_dn0 = assign32570_e43086_d_n0;
        locals.var_t3_dn2 = assign32570_e43086_d_n2;
        locals.var_t3_dn3 = assign32570_e43086_d_n3;
        locals.var_t3_dn4 = assign32570_e43086_d_n4;
        locals.var_t3_dn5 = assign32570_e43086_d_n5;
        locals.var_t3_dn6 = assign32570_e43086_d_n6;
        locals.var_t3_dn7 = assign32570_e43086_d_n7;
        locals.var_t3_dn8 = assign32570_e43086_d_n8;
        locals.var_t3_dn9 = assign32570_e43086_d_n9;
        locals.var_t3_dn10 = assign32570_e43086_d_n10;
        locals.var_t3_dn11 = assign32570_e43086_d_n11;
        locals.var_t3_dn12 = assign32570_e43086_d_n12;
        locals.var_t3_dn13 = assign32570_e43086_d_n13;
        locals.var_t3_dn14 = assign32570_e43086_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign32580_e43095, assign32580_e43095_d_n0, assign32580_e43095_d_n2, assign32580_e43095_d_n3, assign32580_e43095_d_n4, assign32580_e43095_d_n5, assign32580_e43095_d_n6, assign32580_e43095_d_n7, assign32580_e43095_d_n8, assign32580_e43095_d_n9, assign32580_e43095_d_n10, assign32580_e43095_d_n11, assign32580_e43095_d_n12, assign32580_e43095_d_n13, assign32580_e43095_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard740 == 0.0)) {
        let assign32580_e43093: f64 = (1.0 / locals.var_sqrtpsisa);
        (assign32580_e43093, (-(locals.var_sqrtpsisa_dn0 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn2 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn3 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn4 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn5 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn6 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn7 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn8 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn9 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn10 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn11 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn12 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn13 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn14 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))),)
    } else {
        (locals.var_sqrtpsisainv, locals.var_sqrtpsisainv_dn0, locals.var_sqrtpsisainv_dn2, locals.var_sqrtpsisainv_dn3, locals.var_sqrtpsisainv_dn4, locals.var_sqrtpsisainv_dn5, locals.var_sqrtpsisainv_dn6, locals.var_sqrtpsisainv_dn7, locals.var_sqrtpsisainv_dn8, locals.var_sqrtpsisainv_dn9, locals.var_sqrtpsisainv_dn10, locals.var_sqrtpsisainv_dn11, locals.var_sqrtpsisainv_dn12, locals.var_sqrtpsisainv_dn13, locals.var_sqrtpsisainv_dn14,)
    }
};
        locals.var_sqrtpsisainv = assign32580_e43095;
        locals.var_sqrtpsisainv_dn0 = assign32580_e43095_d_n0;
        locals.var_sqrtpsisainv_dn2 = assign32580_e43095_d_n2;
        locals.var_sqrtpsisainv_dn3 = assign32580_e43095_d_n3;
        locals.var_sqrtpsisainv_dn4 = assign32580_e43095_d_n4;
        locals.var_sqrtpsisainv_dn5 = assign32580_e43095_d_n5;
        locals.var_sqrtpsisainv_dn6 = assign32580_e43095_d_n6;
        locals.var_sqrtpsisainv_dn7 = assign32580_e43095_d_n7;
        locals.var_sqrtpsisainv_dn8 = assign32580_e43095_d_n8;
        locals.var_sqrtpsisainv_dn9 = assign32580_e43095_d_n9;
        locals.var_sqrtpsisainv_dn10 = assign32580_e43095_d_n10;
        locals.var_sqrtpsisainv_dn11 = assign32580_e43095_d_n11;
        locals.var_sqrtpsisainv_dn12 = assign32580_e43095_d_n12;
        locals.var_sqrtpsisainv_dn13 = assign32580_e43095_d_n13;
        locals.var_sqrtpsisainv_dn14 = assign32580_e43095_d_n14;
        locals.var_sqrtpsisainv_rv = 0.0;

        let (assign32590_e43127, assign32590_e43127_d_n0, assign32590_e43127_d_n2, assign32590_e43127_d_n3, assign32590_e43127_d_n4, assign32590_e43127_d_n5, assign32590_e43127_d_n6, assign32590_e43127_d_n7, assign32590_e43127_d_n8, assign32590_e43127_d_n9, assign32590_e43127_d_n10, assign32590_e43127_d_n11, assign32590_e43127_d_n12, assign32590_e43127_d_n13, assign32590_e43127_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard740 == 0.0)) {
        let assign32590_e43102: f64 = (2.0 * locals.var_t3);
        let assign32590_e43106: f64 = (locals.var_t3 * 2.0);
        let assign32590_e43108: f64 = (assign32590_e43106 * locals.var_t0);
        let assign32590_e43111: f64 = (locals.var_t3 * 2.0);
        let assign32590_e43113: f64 = (assign32590_e43111 * locals.var_t0);
        let assign32590_e43116: f64 = (2.0 * locals.var_sqrtpsisa);
        let assign32590_e43117: f64 = (assign32590_e43113 + assign32590_e43116);
        let assign32590_e43118: f64 = (assign32590_e43108 * assign32590_e43117);
        let assign32590_e43120: f64 = (assign32590_e43118).max(1e-38);
        let assign32590_e43121: f64 = (assign32590_e43120).ln();
        let assign32590_e43122: f64 = assign32590_e43121;
        let assign32590_e43123: f64 = (assign32590_e43102 + assign32590_e43122);
        let assign32590_e43125: f64 = (assign32590_e43123 - locals.var_t1);
        (assign32590_e43125, (((2.0 * locals.var_t3_dn0) + (if assign32590_e43118 >= 1e-38 { (((((locals.var_t3_dn0 * 2.0) * locals.var_t0) + (assign32590_e43106 * locals.var_t0_dn0)) * assign32590_e43117) + (assign32590_e43108 * ((((locals.var_t3_dn0 * 2.0) * locals.var_t0) + (assign32590_e43111 * locals.var_t0_dn0)) + (2.0 * locals.var_sqrtpsisa_dn0)))) } else { 0.0 } / assign32590_e43120)) - locals.var_t1_dn0), (((2.0 * locals.var_t3_dn2) + (if assign32590_e43118 >= 1e-38 { (((((locals.var_t3_dn2 * 2.0) * locals.var_t0) + (assign32590_e43106 * locals.var_t0_dn2)) * assign32590_e43117) + (assign32590_e43108 * ((((locals.var_t3_dn2 * 2.0) * locals.var_t0) + (assign32590_e43111 * locals.var_t0_dn2)) + (2.0 * locals.var_sqrtpsisa_dn2)))) } else { 0.0 } / assign32590_e43120)) - locals.var_t1_dn2), (((2.0 * locals.var_t3_dn3) + (if assign32590_e43118 >= 1e-38 { (((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign32590_e43106 * locals.var_t0_dn3)) * assign32590_e43117) + (assign32590_e43108 * ((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign32590_e43111 * locals.var_t0_dn3)) + (2.0 * locals.var_sqrtpsisa_dn3)))) } else { 0.0 } / assign32590_e43120)) - locals.var_t1_dn3), (((2.0 * locals.var_t3_dn4) + (if assign32590_e43118 >= 1e-38 { (((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign32590_e43106 * locals.var_t0_dn4)) * assign32590_e43117) + (assign32590_e43108 * ((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign32590_e43111 * locals.var_t0_dn4)) + (2.0 * locals.var_sqrtpsisa_dn4)))) } else { 0.0 } / assign32590_e43120)) - locals.var_t1_dn4), (((2.0 * locals.var_t3_dn5) + (if assign32590_e43118 >= 1e-38 { (((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign32590_e43106 * locals.var_t0_dn5)) * assign32590_e43117) + (assign32590_e43108 * ((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign32590_e43111 * locals.var_t0_dn5)) + (2.0 * locals.var_sqrtpsisa_dn5)))) } else { 0.0 } / assign32590_e43120)) - locals.var_t1_dn5), (((2.0 * locals.var_t3_dn6) + (if assign32590_e43118 >= 1e-38 { (((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign32590_e43106 * locals.var_t0_dn6)) * assign32590_e43117) + (assign32590_e43108 * ((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign32590_e43111 * locals.var_t0_dn6)) + (2.0 * locals.var_sqrtpsisa_dn6)))) } else { 0.0 } / assign32590_e43120)) - locals.var_t1_dn6), (((2.0 * locals.var_t3_dn7) + (if assign32590_e43118 >= 1e-38 { (((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign32590_e43106 * locals.var_t0_dn7)) * assign32590_e43117) + (assign32590_e43108 * ((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign32590_e43111 * locals.var_t0_dn7)) + (2.0 * locals.var_sqrtpsisa_dn7)))) } else { 0.0 } / assign32590_e43120)) - locals.var_t1_dn7), (((2.0 * locals.var_t3_dn8) + (if assign32590_e43118 >= 1e-38 { (((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign32590_e43106 * locals.var_t0_dn8)) * assign32590_e43117) + (assign32590_e43108 * ((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign32590_e43111 * locals.var_t0_dn8)) + (2.0 * locals.var_sqrtpsisa_dn8)))) } else { 0.0 } / assign32590_e43120)) - locals.var_t1_dn8), (((2.0 * locals.var_t3_dn9) + (if assign32590_e43118 >= 1e-38 { (((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign32590_e43106 * locals.var_t0_dn9)) * assign32590_e43117) + (assign32590_e43108 * ((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign32590_e43111 * locals.var_t0_dn9)) + (2.0 * locals.var_sqrtpsisa_dn9)))) } else { 0.0 } / assign32590_e43120)) - locals.var_t1_dn9), (((2.0 * locals.var_t3_dn10) + (if assign32590_e43118 >= 1e-38 { (((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign32590_e43106 * locals.var_t0_dn10)) * assign32590_e43117) + (assign32590_e43108 * ((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign32590_e43111 * locals.var_t0_dn10)) + (2.0 * locals.var_sqrtpsisa_dn10)))) } else { 0.0 } / assign32590_e43120)) - locals.var_t1_dn10), (((2.0 * locals.var_t3_dn11) + (if assign32590_e43118 >= 1e-38 { (((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign32590_e43106 * locals.var_t0_dn11)) * assign32590_e43117) + (assign32590_e43108 * ((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign32590_e43111 * locals.var_t0_dn11)) + (2.0 * locals.var_sqrtpsisa_dn11)))) } else { 0.0 } / assign32590_e43120)) - locals.var_t1_dn11), (((2.0 * locals.var_t3_dn12) + (if assign32590_e43118 >= 1e-38 { (((((locals.var_t3_dn12 * 2.0) * locals.var_t0) + (assign32590_e43106 * locals.var_t0_dn12)) * assign32590_e43117) + (assign32590_e43108 * ((((locals.var_t3_dn12 * 2.0) * locals.var_t0) + (assign32590_e43111 * locals.var_t0_dn12)) + (2.0 * locals.var_sqrtpsisa_dn12)))) } else { 0.0 } / assign32590_e43120)) - locals.var_t1_dn12), (((2.0 * locals.var_t3_dn13) + (if assign32590_e43118 >= 1e-38 { (((((locals.var_t3_dn13 * 2.0) * locals.var_t0) + (assign32590_e43106 * locals.var_t0_dn13)) * assign32590_e43117) + (assign32590_e43108 * ((((locals.var_t3_dn13 * 2.0) * locals.var_t0) + (assign32590_e43111 * locals.var_t0_dn13)) + (2.0 * locals.var_sqrtpsisa_dn13)))) } else { 0.0 } / assign32590_e43120)) - locals.var_t1_dn13), (((2.0 * locals.var_t3_dn14) + (if assign32590_e43118 >= 1e-38 { (((((locals.var_t3_dn14 * 2.0) * locals.var_t0) + (assign32590_e43106 * locals.var_t0_dn14)) * assign32590_e43117) + (assign32590_e43108 * ((((locals.var_t3_dn14 * 2.0) * locals.var_t0) + (assign32590_e43111 * locals.var_t0_dn14)) + (2.0 * locals.var_sqrtpsisa_dn14)))) } else { 0.0 } / assign32590_e43120)) - locals.var_t1_dn14),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign32590_e43127;
        locals.var_t4_dn0 = assign32590_e43127_d_n0;
        locals.var_t4_dn2 = assign32590_e43127_d_n2;
        locals.var_t4_dn3 = assign32590_e43127_d_n3;
        locals.var_t4_dn4 = assign32590_e43127_d_n4;
        locals.var_t4_dn5 = assign32590_e43127_d_n5;
        locals.var_t4_dn6 = assign32590_e43127_d_n6;
        locals.var_t4_dn7 = assign32590_e43127_d_n7;
        locals.var_t4_dn8 = assign32590_e43127_d_n8;
        locals.var_t4_dn9 = assign32590_e43127_d_n9;
        locals.var_t4_dn10 = assign32590_e43127_d_n10;
        locals.var_t4_dn11 = assign32590_e43127_d_n11;
        locals.var_t4_dn12 = assign32590_e43127_d_n12;
        locals.var_t4_dn13 = assign32590_e43127_d_n13;
        locals.var_t4_dn14 = assign32590_e43127_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign32600_e43152, assign32600_e43152_d_n0, assign32600_e43152_d_n2, assign32600_e43152_d_n3, assign32600_e43152_d_n4, assign32600_e43152_d_n5, assign32600_e43152_d_n6, assign32600_e43152_d_n7, assign32600_e43152_d_n8, assign32600_e43152_d_n9, assign32600_e43152_d_n10, assign32600_e43152_d_n11, assign32600_e43152_d_n12, assign32600_e43152_d_n13, assign32600_e43152_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard740 == 0.0)) {
        let assign32600_e43135: f64 = 1.0;
        let assign32600_e43137: f64 = (assign32600_e43135 / locals.var_t3);
        let assign32600_e43138: f64 = (2.0 + assign32600_e43137);
        let assign32600_e43142: f64 = (locals.var_t0 + locals.var_sqrtpsisainv);
        let assign32600_e43143: f64 = assign32600_e43142;
        let assign32600_e43146: f64 = (locals.var_t0 * locals.var_t3);
        let assign32600_e43148: f64 = (assign32600_e43146 + locals.var_sqrtpsisa);
        let assign32600_e43149: f64 = (assign32600_e43143 / assign32600_e43148);
        let assign32600_e43150: f64 = (assign32600_e43138 + assign32600_e43149);
        (assign32600_e43150, ((-((assign32600_e43135 * locals.var_t3_dn0) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn0 + locals.var_sqrtpsisainv_dn0) * assign32600_e43148) - (assign32600_e43143 * (((locals.var_t0_dn0 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn0)) + locals.var_sqrtpsisa_dn0))) / (assign32600_e43148 * assign32600_e43148))), ((-((assign32600_e43135 * locals.var_t3_dn2) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn2 + locals.var_sqrtpsisainv_dn2) * assign32600_e43148) - (assign32600_e43143 * (((locals.var_t0_dn2 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn2)) + locals.var_sqrtpsisa_dn2))) / (assign32600_e43148 * assign32600_e43148))), ((-((assign32600_e43135 * locals.var_t3_dn3) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn3 + locals.var_sqrtpsisainv_dn3) * assign32600_e43148) - (assign32600_e43143 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3))) / (assign32600_e43148 * assign32600_e43148))), ((-((assign32600_e43135 * locals.var_t3_dn4) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn4 + locals.var_sqrtpsisainv_dn4) * assign32600_e43148) - (assign32600_e43143 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4))) / (assign32600_e43148 * assign32600_e43148))), ((-((assign32600_e43135 * locals.var_t3_dn5) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn5 + locals.var_sqrtpsisainv_dn5) * assign32600_e43148) - (assign32600_e43143 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5))) / (assign32600_e43148 * assign32600_e43148))), ((-((assign32600_e43135 * locals.var_t3_dn6) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn6 + locals.var_sqrtpsisainv_dn6) * assign32600_e43148) - (assign32600_e43143 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6))) / (assign32600_e43148 * assign32600_e43148))), ((-((assign32600_e43135 * locals.var_t3_dn7) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn7 + locals.var_sqrtpsisainv_dn7) * assign32600_e43148) - (assign32600_e43143 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7))) / (assign32600_e43148 * assign32600_e43148))), ((-((assign32600_e43135 * locals.var_t3_dn8) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn8 + locals.var_sqrtpsisainv_dn8) * assign32600_e43148) - (assign32600_e43143 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8))) / (assign32600_e43148 * assign32600_e43148))), ((-((assign32600_e43135 * locals.var_t3_dn9) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn9 + locals.var_sqrtpsisainv_dn9) * assign32600_e43148) - (assign32600_e43143 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9))) / (assign32600_e43148 * assign32600_e43148))), ((-((assign32600_e43135 * locals.var_t3_dn10) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn10 + locals.var_sqrtpsisainv_dn10) * assign32600_e43148) - (assign32600_e43143 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10))) / (assign32600_e43148 * assign32600_e43148))), ((-((assign32600_e43135 * locals.var_t3_dn11) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn11 + locals.var_sqrtpsisainv_dn11) * assign32600_e43148) - (assign32600_e43143 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11))) / (assign32600_e43148 * assign32600_e43148))), ((-((assign32600_e43135 * locals.var_t3_dn12) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn12 + locals.var_sqrtpsisainv_dn12) * assign32600_e43148) - (assign32600_e43143 * (((locals.var_t0_dn12 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn12)) + locals.var_sqrtpsisa_dn12))) / (assign32600_e43148 * assign32600_e43148))), ((-((assign32600_e43135 * locals.var_t3_dn13) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn13 + locals.var_sqrtpsisainv_dn13) * assign32600_e43148) - (assign32600_e43143 * (((locals.var_t0_dn13 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn13)) + locals.var_sqrtpsisa_dn13))) / (assign32600_e43148 * assign32600_e43148))), ((-((assign32600_e43135 * locals.var_t3_dn14) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn14 + locals.var_sqrtpsisainv_dn14) * assign32600_e43148) - (assign32600_e43143 * (((locals.var_t0_dn14 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn14)) + locals.var_sqrtpsisa_dn14))) / (assign32600_e43148 * assign32600_e43148))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn13, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign32600_e43152;
        locals.var_t5_dn0 = assign32600_e43152_d_n0;
        locals.var_t5_dn2 = assign32600_e43152_d_n2;
        locals.var_t5_dn3 = assign32600_e43152_d_n3;
        locals.var_t5_dn4 = assign32600_e43152_d_n4;
        locals.var_t5_dn5 = assign32600_e43152_d_n5;
        locals.var_t5_dn6 = assign32600_e43152_d_n6;
        locals.var_t5_dn7 = assign32600_e43152_d_n7;
        locals.var_t5_dn8 = assign32600_e43152_d_n8;
        locals.var_t5_dn9 = assign32600_e43152_d_n9;
        locals.var_t5_dn10 = assign32600_e43152_d_n10;
        locals.var_t5_dn11 = assign32600_e43152_d_n11;
        locals.var_t5_dn12 = assign32600_e43152_d_n12;
        locals.var_t5_dn13 = assign32600_e43152_d_n13;
        locals.var_t5_dn14 = assign32600_e43152_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign32610_e43163, assign32610_e43163_d_n0, assign32610_e43163_d_n2, assign32610_e43163_d_n3, assign32610_e43163_d_n4, assign32610_e43163_d_n5, assign32610_e43163_d_n6, assign32610_e43163_d_n7, assign32610_e43163_d_n8, assign32610_e43163_d_n9, assign32610_e43163_d_n10, assign32610_e43163_d_n11, assign32610_e43163_d_n12, assign32610_e43163_d_n13, assign32610_e43163_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard740 == 0.0)) {
        let assign32610_e43160: f64 = (locals.var_t4 / locals.var_t5);
        let assign32610_e43161: f64 = (locals.var_t3 - assign32610_e43160);
        (assign32610_e43161, (locals.var_t3_dn0 - (((locals.var_t4_dn0 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn0)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn2 - (((locals.var_t4_dn2 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn2)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn3 - (((locals.var_t4_dn3 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn3)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn4 - (((locals.var_t4_dn4 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn4)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn5 - (((locals.var_t4_dn5 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn5)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn6 - (((locals.var_t4_dn6 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn6)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn7 - (((locals.var_t4_dn7 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn7)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn8 - (((locals.var_t4_dn8 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn8)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn9 - (((locals.var_t4_dn9 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn9)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn10 - (((locals.var_t4_dn10 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn10)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn11 - (((locals.var_t4_dn11 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn11)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn12 - (((locals.var_t4_dn12 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn12)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn13 - (((locals.var_t4_dn13 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn13)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn14 - (((locals.var_t4_dn14 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn14)) / (locals.var_t5 * locals.var_t5))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign32610_e43163;
        locals.var_t3_dn0 = assign32610_e43163_d_n0;
        locals.var_t3_dn2 = assign32610_e43163_d_n2;
        locals.var_t3_dn3 = assign32610_e43163_d_n3;
        locals.var_t3_dn4 = assign32610_e43163_d_n4;
        locals.var_t3_dn5 = assign32610_e43163_d_n5;
        locals.var_t3_dn6 = assign32610_e43163_d_n6;
        locals.var_t3_dn7 = assign32610_e43163_d_n7;
        locals.var_t3_dn8 = assign32610_e43163_d_n8;
        locals.var_t3_dn9 = assign32610_e43163_d_n9;
        locals.var_t3_dn10 = assign32610_e43163_d_n10;
        locals.var_t3_dn11 = assign32610_e43163_d_n11;
        locals.var_t3_dn12 = assign32610_e43163_d_n12;
        locals.var_t3_dn13 = assign32610_e43163_d_n13;
        locals.var_t3_dn14 = assign32610_e43163_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign32620_e43195, assign32620_e43195_d_n0, assign32620_e43195_d_n2, assign32620_e43195_d_n3, assign32620_e43195_d_n4, assign32620_e43195_d_n5, assign32620_e43195_d_n6, assign32620_e43195_d_n7, assign32620_e43195_d_n8, assign32620_e43195_d_n9, assign32620_e43195_d_n10, assign32620_e43195_d_n11, assign32620_e43195_d_n12, assign32620_e43195_d_n13, assign32620_e43195_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard740 == 0.0)) {
        let assign32620_e43170: f64 = (2.0 * locals.var_t3);
        let assign32620_e43174: f64 = (locals.var_t3 * 2.0);
        let assign32620_e43176: f64 = (assign32620_e43174 * locals.var_t0);
        let assign32620_e43179: f64 = (locals.var_t3 * 2.0);
        let assign32620_e43181: f64 = (assign32620_e43179 * locals.var_t0);
        let assign32620_e43184: f64 = (2.0 * locals.var_sqrtpsisa);
        let assign32620_e43185: f64 = (assign32620_e43181 + assign32620_e43184);
        let assign32620_e43186: f64 = (assign32620_e43176 * assign32620_e43185);
        let assign32620_e43188: f64 = (assign32620_e43186).max(1e-38);
        let assign32620_e43189: f64 = (assign32620_e43188).ln();
        let assign32620_e43190: f64 = assign32620_e43189;
        let assign32620_e43191: f64 = (assign32620_e43170 + assign32620_e43190);
        let assign32620_e43193: f64 = (assign32620_e43191 - locals.var_t1);
        (assign32620_e43193, (((2.0 * locals.var_t3_dn0) + (if assign32620_e43186 >= 1e-38 { (((((locals.var_t3_dn0 * 2.0) * locals.var_t0) + (assign32620_e43174 * locals.var_t0_dn0)) * assign32620_e43185) + (assign32620_e43176 * ((((locals.var_t3_dn0 * 2.0) * locals.var_t0) + (assign32620_e43179 * locals.var_t0_dn0)) + (2.0 * locals.var_sqrtpsisa_dn0)))) } else { 0.0 } / assign32620_e43188)) - locals.var_t1_dn0), (((2.0 * locals.var_t3_dn2) + (if assign32620_e43186 >= 1e-38 { (((((locals.var_t3_dn2 * 2.0) * locals.var_t0) + (assign32620_e43174 * locals.var_t0_dn2)) * assign32620_e43185) + (assign32620_e43176 * ((((locals.var_t3_dn2 * 2.0) * locals.var_t0) + (assign32620_e43179 * locals.var_t0_dn2)) + (2.0 * locals.var_sqrtpsisa_dn2)))) } else { 0.0 } / assign32620_e43188)) - locals.var_t1_dn2), (((2.0 * locals.var_t3_dn3) + (if assign32620_e43186 >= 1e-38 { (((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign32620_e43174 * locals.var_t0_dn3)) * assign32620_e43185) + (assign32620_e43176 * ((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign32620_e43179 * locals.var_t0_dn3)) + (2.0 * locals.var_sqrtpsisa_dn3)))) } else { 0.0 } / assign32620_e43188)) - locals.var_t1_dn3), (((2.0 * locals.var_t3_dn4) + (if assign32620_e43186 >= 1e-38 { (((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign32620_e43174 * locals.var_t0_dn4)) * assign32620_e43185) + (assign32620_e43176 * ((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign32620_e43179 * locals.var_t0_dn4)) + (2.0 * locals.var_sqrtpsisa_dn4)))) } else { 0.0 } / assign32620_e43188)) - locals.var_t1_dn4), (((2.0 * locals.var_t3_dn5) + (if assign32620_e43186 >= 1e-38 { (((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign32620_e43174 * locals.var_t0_dn5)) * assign32620_e43185) + (assign32620_e43176 * ((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign32620_e43179 * locals.var_t0_dn5)) + (2.0 * locals.var_sqrtpsisa_dn5)))) } else { 0.0 } / assign32620_e43188)) - locals.var_t1_dn5), (((2.0 * locals.var_t3_dn6) + (if assign32620_e43186 >= 1e-38 { (((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign32620_e43174 * locals.var_t0_dn6)) * assign32620_e43185) + (assign32620_e43176 * ((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign32620_e43179 * locals.var_t0_dn6)) + (2.0 * locals.var_sqrtpsisa_dn6)))) } else { 0.0 } / assign32620_e43188)) - locals.var_t1_dn6), (((2.0 * locals.var_t3_dn7) + (if assign32620_e43186 >= 1e-38 { (((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign32620_e43174 * locals.var_t0_dn7)) * assign32620_e43185) + (assign32620_e43176 * ((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign32620_e43179 * locals.var_t0_dn7)) + (2.0 * locals.var_sqrtpsisa_dn7)))) } else { 0.0 } / assign32620_e43188)) - locals.var_t1_dn7), (((2.0 * locals.var_t3_dn8) + (if assign32620_e43186 >= 1e-38 { (((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign32620_e43174 * locals.var_t0_dn8)) * assign32620_e43185) + (assign32620_e43176 * ((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign32620_e43179 * locals.var_t0_dn8)) + (2.0 * locals.var_sqrtpsisa_dn8)))) } else { 0.0 } / assign32620_e43188)) - locals.var_t1_dn8), (((2.0 * locals.var_t3_dn9) + (if assign32620_e43186 >= 1e-38 { (((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign32620_e43174 * locals.var_t0_dn9)) * assign32620_e43185) + (assign32620_e43176 * ((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign32620_e43179 * locals.var_t0_dn9)) + (2.0 * locals.var_sqrtpsisa_dn9)))) } else { 0.0 } / assign32620_e43188)) - locals.var_t1_dn9), (((2.0 * locals.var_t3_dn10) + (if assign32620_e43186 >= 1e-38 { (((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign32620_e43174 * locals.var_t0_dn10)) * assign32620_e43185) + (assign32620_e43176 * ((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign32620_e43179 * locals.var_t0_dn10)) + (2.0 * locals.var_sqrtpsisa_dn10)))) } else { 0.0 } / assign32620_e43188)) - locals.var_t1_dn10), (((2.0 * locals.var_t3_dn11) + (if assign32620_e43186 >= 1e-38 { (((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign32620_e43174 * locals.var_t0_dn11)) * assign32620_e43185) + (assign32620_e43176 * ((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign32620_e43179 * locals.var_t0_dn11)) + (2.0 * locals.var_sqrtpsisa_dn11)))) } else { 0.0 } / assign32620_e43188)) - locals.var_t1_dn11), (((2.0 * locals.var_t3_dn12) + (if assign32620_e43186 >= 1e-38 { (((((locals.var_t3_dn12 * 2.0) * locals.var_t0) + (assign32620_e43174 * locals.var_t0_dn12)) * assign32620_e43185) + (assign32620_e43176 * ((((locals.var_t3_dn12 * 2.0) * locals.var_t0) + (assign32620_e43179 * locals.var_t0_dn12)) + (2.0 * locals.var_sqrtpsisa_dn12)))) } else { 0.0 } / assign32620_e43188)) - locals.var_t1_dn12), (((2.0 * locals.var_t3_dn13) + (if assign32620_e43186 >= 1e-38 { (((((locals.var_t3_dn13 * 2.0) * locals.var_t0) + (assign32620_e43174 * locals.var_t0_dn13)) * assign32620_e43185) + (assign32620_e43176 * ((((locals.var_t3_dn13 * 2.0) * locals.var_t0) + (assign32620_e43179 * locals.var_t0_dn13)) + (2.0 * locals.var_sqrtpsisa_dn13)))) } else { 0.0 } / assign32620_e43188)) - locals.var_t1_dn13), (((2.0 * locals.var_t3_dn14) + (if assign32620_e43186 >= 1e-38 { (((((locals.var_t3_dn14 * 2.0) * locals.var_t0) + (assign32620_e43174 * locals.var_t0_dn14)) * assign32620_e43185) + (assign32620_e43176 * ((((locals.var_t3_dn14 * 2.0) * locals.var_t0) + (assign32620_e43179 * locals.var_t0_dn14)) + (2.0 * locals.var_sqrtpsisa_dn14)))) } else { 0.0 } / assign32620_e43188)) - locals.var_t1_dn14),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign32620_e43195;
        locals.var_t4_dn0 = assign32620_e43195_d_n0;
        locals.var_t4_dn2 = assign32620_e43195_d_n2;
        locals.var_t4_dn3 = assign32620_e43195_d_n3;
        locals.var_t4_dn4 = assign32620_e43195_d_n4;
        locals.var_t4_dn5 = assign32620_e43195_d_n5;
        locals.var_t4_dn6 = assign32620_e43195_d_n6;
        locals.var_t4_dn7 = assign32620_e43195_d_n7;
        locals.var_t4_dn8 = assign32620_e43195_d_n8;
        locals.var_t4_dn9 = assign32620_e43195_d_n9;
        locals.var_t4_dn10 = assign32620_e43195_d_n10;
        locals.var_t4_dn11 = assign32620_e43195_d_n11;
        locals.var_t4_dn12 = assign32620_e43195_d_n12;
        locals.var_t4_dn13 = assign32620_e43195_d_n13;
        locals.var_t4_dn14 = assign32620_e43195_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign32630_e43220, assign32630_e43220_d_n0, assign32630_e43220_d_n2, assign32630_e43220_d_n3, assign32630_e43220_d_n4, assign32630_e43220_d_n5, assign32630_e43220_d_n6, assign32630_e43220_d_n7, assign32630_e43220_d_n8, assign32630_e43220_d_n9, assign32630_e43220_d_n10, assign32630_e43220_d_n11, assign32630_e43220_d_n12, assign32630_e43220_d_n13, assign32630_e43220_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard740 == 0.0)) {
        let assign32630_e43203: f64 = 1.0;
        let assign32630_e43205: f64 = (assign32630_e43203 / locals.var_t3);
        let assign32630_e43206: f64 = (2.0 + assign32630_e43205);
        let assign32630_e43210: f64 = (locals.var_t0 + locals.var_sqrtpsisainv);
        let assign32630_e43211: f64 = assign32630_e43210;
        let assign32630_e43214: f64 = (locals.var_t0 * locals.var_t3);
        let assign32630_e43216: f64 = (assign32630_e43214 + locals.var_sqrtpsisa);
        let assign32630_e43217: f64 = (assign32630_e43211 / assign32630_e43216);
        let assign32630_e43218: f64 = (assign32630_e43206 + assign32630_e43217);
        (assign32630_e43218, ((-((assign32630_e43203 * locals.var_t3_dn0) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn0 + locals.var_sqrtpsisainv_dn0) * assign32630_e43216) - (assign32630_e43211 * (((locals.var_t0_dn0 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn0)) + locals.var_sqrtpsisa_dn0))) / (assign32630_e43216 * assign32630_e43216))), ((-((assign32630_e43203 * locals.var_t3_dn2) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn2 + locals.var_sqrtpsisainv_dn2) * assign32630_e43216) - (assign32630_e43211 * (((locals.var_t0_dn2 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn2)) + locals.var_sqrtpsisa_dn2))) / (assign32630_e43216 * assign32630_e43216))), ((-((assign32630_e43203 * locals.var_t3_dn3) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn3 + locals.var_sqrtpsisainv_dn3) * assign32630_e43216) - (assign32630_e43211 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3))) / (assign32630_e43216 * assign32630_e43216))), ((-((assign32630_e43203 * locals.var_t3_dn4) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn4 + locals.var_sqrtpsisainv_dn4) * assign32630_e43216) - (assign32630_e43211 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4))) / (assign32630_e43216 * assign32630_e43216))), ((-((assign32630_e43203 * locals.var_t3_dn5) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn5 + locals.var_sqrtpsisainv_dn5) * assign32630_e43216) - (assign32630_e43211 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5))) / (assign32630_e43216 * assign32630_e43216))), ((-((assign32630_e43203 * locals.var_t3_dn6) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn6 + locals.var_sqrtpsisainv_dn6) * assign32630_e43216) - (assign32630_e43211 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6))) / (assign32630_e43216 * assign32630_e43216))), ((-((assign32630_e43203 * locals.var_t3_dn7) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn7 + locals.var_sqrtpsisainv_dn7) * assign32630_e43216) - (assign32630_e43211 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7))) / (assign32630_e43216 * assign32630_e43216))), ((-((assign32630_e43203 * locals.var_t3_dn8) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn8 + locals.var_sqrtpsisainv_dn8) * assign32630_e43216) - (assign32630_e43211 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8))) / (assign32630_e43216 * assign32630_e43216))), ((-((assign32630_e43203 * locals.var_t3_dn9) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn9 + locals.var_sqrtpsisainv_dn9) * assign32630_e43216) - (assign32630_e43211 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9))) / (assign32630_e43216 * assign32630_e43216))), ((-((assign32630_e43203 * locals.var_t3_dn10) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn10 + locals.var_sqrtpsisainv_dn10) * assign32630_e43216) - (assign32630_e43211 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10))) / (assign32630_e43216 * assign32630_e43216))), ((-((assign32630_e43203 * locals.var_t3_dn11) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn11 + locals.var_sqrtpsisainv_dn11) * assign32630_e43216) - (assign32630_e43211 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11))) / (assign32630_e43216 * assign32630_e43216))), ((-((assign32630_e43203 * locals.var_t3_dn12) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn12 + locals.var_sqrtpsisainv_dn12) * assign32630_e43216) - (assign32630_e43211 * (((locals.var_t0_dn12 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn12)) + locals.var_sqrtpsisa_dn12))) / (assign32630_e43216 * assign32630_e43216))), ((-((assign32630_e43203 * locals.var_t3_dn13) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn13 + locals.var_sqrtpsisainv_dn13) * assign32630_e43216) - (assign32630_e43211 * (((locals.var_t0_dn13 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn13)) + locals.var_sqrtpsisa_dn13))) / (assign32630_e43216 * assign32630_e43216))), ((-((assign32630_e43203 * locals.var_t3_dn14) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn14 + locals.var_sqrtpsisainv_dn14) * assign32630_e43216) - (assign32630_e43211 * (((locals.var_t0_dn14 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn14)) + locals.var_sqrtpsisa_dn14))) / (assign32630_e43216 * assign32630_e43216))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn13, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign32630_e43220;
        locals.var_t5_dn0 = assign32630_e43220_d_n0;
        locals.var_t5_dn2 = assign32630_e43220_d_n2;
        locals.var_t5_dn3 = assign32630_e43220_d_n3;
        locals.var_t5_dn4 = assign32630_e43220_d_n4;
        locals.var_t5_dn5 = assign32630_e43220_d_n5;
        locals.var_t5_dn6 = assign32630_e43220_d_n6;
        locals.var_t5_dn7 = assign32630_e43220_d_n7;
        locals.var_t5_dn8 = assign32630_e43220_d_n8;
        locals.var_t5_dn9 = assign32630_e43220_d_n9;
        locals.var_t5_dn10 = assign32630_e43220_d_n10;
        locals.var_t5_dn11 = assign32630_e43220_d_n11;
        locals.var_t5_dn12 = assign32630_e43220_d_n12;
        locals.var_t5_dn13 = assign32630_e43220_d_n13;
        locals.var_t5_dn14 = assign32630_e43220_d_n14;
        locals.var_t5_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_106(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign32640_e43247, assign32640_e43247_d_n0, assign32640_e43247_d_n2, assign32640_e43247_d_n3, assign32640_e43247_d_n4, assign32640_e43247_d_n5, assign32640_e43247_d_n6, assign32640_e43247_d_n7, assign32640_e43247_d_n8, assign32640_e43247_d_n9, assign32640_e43247_d_n10, assign32640_e43247_d_n11, assign32640_e43247_d_n12, assign32640_e43247_d_n13, assign32640_e43247_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard740 == 0.0)) {
        let assign32640_e43228: f64 = (locals.var_t0 + locals.var_sqrtpsisainv);
        let assign32640_e43231: f64 = (locals.var_t0 * locals.var_t3);
        let assign32640_e43233: f64 = (assign32640_e43231 + locals.var_sqrtpsisa);
        let assign32640_e43234: f64 = (assign32640_e43228 / assign32640_e43233);
        let assign32640_e43235: f64 = assign32640_e43234;
        let assign32640_e43238: f64 = (locals.var_t0 + locals.var_sqrtpsisainv);
        let assign32640_e43241: f64 = (locals.var_t0 * locals.var_t3);
        let assign32640_e43243: f64 = (assign32640_e43241 + locals.var_sqrtpsisa);
        let assign32640_e43244: f64 = (assign32640_e43238 / assign32640_e43243);
        let assign32640_e43245: f64 = (assign32640_e43235 * assign32640_e43244);
        (assign32640_e43245, ((((((locals.var_t0_dn0 + locals.var_sqrtpsisainv_dn0) * assign32640_e43233) - (assign32640_e43228 * (((locals.var_t0_dn0 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn0)) + locals.var_sqrtpsisa_dn0))) / (assign32640_e43233 * assign32640_e43233)) * assign32640_e43244) + (assign32640_e43235 * ((((locals.var_t0_dn0 + locals.var_sqrtpsisainv_dn0) * assign32640_e43243) - (assign32640_e43238 * (((locals.var_t0_dn0 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn0)) + locals.var_sqrtpsisa_dn0))) / (assign32640_e43243 * assign32640_e43243)))), ((((((locals.var_t0_dn2 + locals.var_sqrtpsisainv_dn2) * assign32640_e43233) - (assign32640_e43228 * (((locals.var_t0_dn2 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn2)) + locals.var_sqrtpsisa_dn2))) / (assign32640_e43233 * assign32640_e43233)) * assign32640_e43244) + (assign32640_e43235 * ((((locals.var_t0_dn2 + locals.var_sqrtpsisainv_dn2) * assign32640_e43243) - (assign32640_e43238 * (((locals.var_t0_dn2 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn2)) + locals.var_sqrtpsisa_dn2))) / (assign32640_e43243 * assign32640_e43243)))), ((((((locals.var_t0_dn3 + locals.var_sqrtpsisainv_dn3) * assign32640_e43233) - (assign32640_e43228 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3))) / (assign32640_e43233 * assign32640_e43233)) * assign32640_e43244) + (assign32640_e43235 * ((((locals.var_t0_dn3 + locals.var_sqrtpsisainv_dn3) * assign32640_e43243) - (assign32640_e43238 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3))) / (assign32640_e43243 * assign32640_e43243)))), ((((((locals.var_t0_dn4 + locals.var_sqrtpsisainv_dn4) * assign32640_e43233) - (assign32640_e43228 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4))) / (assign32640_e43233 * assign32640_e43233)) * assign32640_e43244) + (assign32640_e43235 * ((((locals.var_t0_dn4 + locals.var_sqrtpsisainv_dn4) * assign32640_e43243) - (assign32640_e43238 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4))) / (assign32640_e43243 * assign32640_e43243)))), ((((((locals.var_t0_dn5 + locals.var_sqrtpsisainv_dn5) * assign32640_e43233) - (assign32640_e43228 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5))) / (assign32640_e43233 * assign32640_e43233)) * assign32640_e43244) + (assign32640_e43235 * ((((locals.var_t0_dn5 + locals.var_sqrtpsisainv_dn5) * assign32640_e43243) - (assign32640_e43238 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5))) / (assign32640_e43243 * assign32640_e43243)))), ((((((locals.var_t0_dn6 + locals.var_sqrtpsisainv_dn6) * assign32640_e43233) - (assign32640_e43228 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6))) / (assign32640_e43233 * assign32640_e43233)) * assign32640_e43244) + (assign32640_e43235 * ((((locals.var_t0_dn6 + locals.var_sqrtpsisainv_dn6) * assign32640_e43243) - (assign32640_e43238 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6))) / (assign32640_e43243 * assign32640_e43243)))), ((((((locals.var_t0_dn7 + locals.var_sqrtpsisainv_dn7) * assign32640_e43233) - (assign32640_e43228 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7))) / (assign32640_e43233 * assign32640_e43233)) * assign32640_e43244) + (assign32640_e43235 * ((((locals.var_t0_dn7 + locals.var_sqrtpsisainv_dn7) * assign32640_e43243) - (assign32640_e43238 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7))) / (assign32640_e43243 * assign32640_e43243)))), ((((((locals.var_t0_dn8 + locals.var_sqrtpsisainv_dn8) * assign32640_e43233) - (assign32640_e43228 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8))) / (assign32640_e43233 * assign32640_e43233)) * assign32640_e43244) + (assign32640_e43235 * ((((locals.var_t0_dn8 + locals.var_sqrtpsisainv_dn8) * assign32640_e43243) - (assign32640_e43238 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8))) / (assign32640_e43243 * assign32640_e43243)))), ((((((locals.var_t0_dn9 + locals.var_sqrtpsisainv_dn9) * assign32640_e43233) - (assign32640_e43228 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9))) / (assign32640_e43233 * assign32640_e43233)) * assign32640_e43244) + (assign32640_e43235 * ((((locals.var_t0_dn9 + locals.var_sqrtpsisainv_dn9) * assign32640_e43243) - (assign32640_e43238 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9))) / (assign32640_e43243 * assign32640_e43243)))), ((((((locals.var_t0_dn10 + locals.var_sqrtpsisainv_dn10) * assign32640_e43233) - (assign32640_e43228 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10))) / (assign32640_e43233 * assign32640_e43233)) * assign32640_e43244) + (assign32640_e43235 * ((((locals.var_t0_dn10 + locals.var_sqrtpsisainv_dn10) * assign32640_e43243) - (assign32640_e43238 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10))) / (assign32640_e43243 * assign32640_e43243)))), ((((((locals.var_t0_dn11 + locals.var_sqrtpsisainv_dn11) * assign32640_e43233) - (assign32640_e43228 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11))) / (assign32640_e43233 * assign32640_e43233)) * assign32640_e43244) + (assign32640_e43235 * ((((locals.var_t0_dn11 + locals.var_sqrtpsisainv_dn11) * assign32640_e43243) - (assign32640_e43238 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11))) / (assign32640_e43243 * assign32640_e43243)))), ((((((locals.var_t0_dn12 + locals.var_sqrtpsisainv_dn12) * assign32640_e43233) - (assign32640_e43228 * (((locals.var_t0_dn12 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn12)) + locals.var_sqrtpsisa_dn12))) / (assign32640_e43233 * assign32640_e43233)) * assign32640_e43244) + (assign32640_e43235 * ((((locals.var_t0_dn12 + locals.var_sqrtpsisainv_dn12) * assign32640_e43243) - (assign32640_e43238 * (((locals.var_t0_dn12 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn12)) + locals.var_sqrtpsisa_dn12))) / (assign32640_e43243 * assign32640_e43243)))), ((((((locals.var_t0_dn13 + locals.var_sqrtpsisainv_dn13) * assign32640_e43233) - (assign32640_e43228 * (((locals.var_t0_dn13 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn13)) + locals.var_sqrtpsisa_dn13))) / (assign32640_e43233 * assign32640_e43233)) * assign32640_e43244) + (assign32640_e43235 * ((((locals.var_t0_dn13 + locals.var_sqrtpsisainv_dn13) * assign32640_e43243) - (assign32640_e43238 * (((locals.var_t0_dn13 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn13)) + locals.var_sqrtpsisa_dn13))) / (assign32640_e43243 * assign32640_e43243)))), ((((((locals.var_t0_dn14 + locals.var_sqrtpsisainv_dn14) * assign32640_e43233) - (assign32640_e43228 * (((locals.var_t0_dn14 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn14)) + locals.var_sqrtpsisa_dn14))) / (assign32640_e43233 * assign32640_e43233)) * assign32640_e43244) + (assign32640_e43235 * ((((locals.var_t0_dn14 + locals.var_sqrtpsisainv_dn14) * assign32640_e43243) - (assign32640_e43238 * (((locals.var_t0_dn14 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn14)) + locals.var_sqrtpsisa_dn14))) / (assign32640_e43243 * assign32640_e43243)))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12, locals.var_t6_dn13, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign32640_e43247;
        locals.var_t6_dn0 = assign32640_e43247_d_n0;
        locals.var_t6_dn2 = assign32640_e43247_d_n2;
        locals.var_t6_dn3 = assign32640_e43247_d_n3;
        locals.var_t6_dn4 = assign32640_e43247_d_n4;
        locals.var_t6_dn5 = assign32640_e43247_d_n5;
        locals.var_t6_dn6 = assign32640_e43247_d_n6;
        locals.var_t6_dn7 = assign32640_e43247_d_n7;
        locals.var_t6_dn8 = assign32640_e43247_d_n8;
        locals.var_t6_dn9 = assign32640_e43247_d_n9;
        locals.var_t6_dn10 = assign32640_e43247_d_n10;
        locals.var_t6_dn11 = assign32640_e43247_d_n11;
        locals.var_t6_dn12 = assign32640_e43247_d_n12;
        locals.var_t6_dn13 = assign32640_e43247_d_n13;
        locals.var_t6_dn14 = assign32640_e43247_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign32650_e43281, assign32650_e43281_d_n0, assign32650_e43281_d_n2, assign32650_e43281_d_n3, assign32650_e43281_d_n4, assign32650_e43281_d_n5, assign32650_e43281_d_n6, assign32650_e43281_d_n7, assign32650_e43281_d_n8, assign32650_e43281_d_n9, assign32650_e43281_d_n10, assign32650_e43281_d_n11, assign32650_e43281_d_n12, assign32650_e43281_d_n13, assign32650_e43281_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard740 == 0.0)) {
        let assign32650_e43253: f64 = (-1.0);
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_t3;
        let assign32650_e43256: f64 = (1.0 * __rspice_inv_cse_0);
        let assign32650_e43259: f64 = (1.0 * __rspice_inv_cse_0);
        let assign32650_e43260: f64 = (assign32650_e43256 * assign32650_e43259);
        let assign32650_e43261: f64 = (assign32650_e43253 * assign32650_e43260);
        let assign32650_e43264: f64 = 1.0;
        let assign32650_e43267: f64 = (locals.var_sqrtpsisa * locals.var_sqrtpsisa);
        let assign32650_e43269: f64 = (assign32650_e43267 * locals.var_sqrtpsisa);
        let assign32650_e43272: f64 = (locals.var_t0 * locals.var_t3);
        let assign32650_e43274: f64 = (assign32650_e43272 + locals.var_sqrtpsisa);
        let assign32650_e43275: f64 = (assign32650_e43269 * assign32650_e43274);
        let assign32650_e43276: f64 = (assign32650_e43264 / assign32650_e43275);
        let assign32650_e43277: f64 = (assign32650_e43261 - assign32650_e43276);
        let assign32650_e43279: f64 = (assign32650_e43277 - locals.var_t6);
        (assign32650_e43279, (((assign32650_e43253 * (((-(locals.var_t3_dn0 / (locals.var_t3 * locals.var_t3))) * assign32650_e43259) + (assign32650_e43256 * (-(locals.var_t3_dn0 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32650_e43264 * ((((((locals.var_sqrtpsisa_dn0 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn0)) * locals.var_sqrtpsisa) + (assign32650_e43267 * locals.var_sqrtpsisa_dn0)) * assign32650_e43274) + (assign32650_e43269 * (((locals.var_t0_dn0 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn0)) + locals.var_sqrtpsisa_dn0)))) / (assign32650_e43275 * assign32650_e43275)))) - locals.var_t6_dn0), (((assign32650_e43253 * (((-(locals.var_t3_dn2 / (locals.var_t3 * locals.var_t3))) * assign32650_e43259) + (assign32650_e43256 * (-(locals.var_t3_dn2 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32650_e43264 * ((((((locals.var_sqrtpsisa_dn2 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn2)) * locals.var_sqrtpsisa) + (assign32650_e43267 * locals.var_sqrtpsisa_dn2)) * assign32650_e43274) + (assign32650_e43269 * (((locals.var_t0_dn2 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn2)) + locals.var_sqrtpsisa_dn2)))) / (assign32650_e43275 * assign32650_e43275)))) - locals.var_t6_dn2), (((assign32650_e43253 * (((-(locals.var_t3_dn3 / (locals.var_t3 * locals.var_t3))) * assign32650_e43259) + (assign32650_e43256 * (-(locals.var_t3_dn3 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32650_e43264 * ((((((locals.var_sqrtpsisa_dn3 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn3)) * locals.var_sqrtpsisa) + (assign32650_e43267 * locals.var_sqrtpsisa_dn3)) * assign32650_e43274) + (assign32650_e43269 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3)))) / (assign32650_e43275 * assign32650_e43275)))) - locals.var_t6_dn3), (((assign32650_e43253 * (((-(locals.var_t3_dn4 / (locals.var_t3 * locals.var_t3))) * assign32650_e43259) + (assign32650_e43256 * (-(locals.var_t3_dn4 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32650_e43264 * ((((((locals.var_sqrtpsisa_dn4 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn4)) * locals.var_sqrtpsisa) + (assign32650_e43267 * locals.var_sqrtpsisa_dn4)) * assign32650_e43274) + (assign32650_e43269 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4)))) / (assign32650_e43275 * assign32650_e43275)))) - locals.var_t6_dn4), (((assign32650_e43253 * (((-(locals.var_t3_dn5 / (locals.var_t3 * locals.var_t3))) * assign32650_e43259) + (assign32650_e43256 * (-(locals.var_t3_dn5 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32650_e43264 * ((((((locals.var_sqrtpsisa_dn5 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn5)) * locals.var_sqrtpsisa) + (assign32650_e43267 * locals.var_sqrtpsisa_dn5)) * assign32650_e43274) + (assign32650_e43269 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5)))) / (assign32650_e43275 * assign32650_e43275)))) - locals.var_t6_dn5), (((assign32650_e43253 * (((-(locals.var_t3_dn6 / (locals.var_t3 * locals.var_t3))) * assign32650_e43259) + (assign32650_e43256 * (-(locals.var_t3_dn6 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32650_e43264 * ((((((locals.var_sqrtpsisa_dn6 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn6)) * locals.var_sqrtpsisa) + (assign32650_e43267 * locals.var_sqrtpsisa_dn6)) * assign32650_e43274) + (assign32650_e43269 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6)))) / (assign32650_e43275 * assign32650_e43275)))) - locals.var_t6_dn6), (((assign32650_e43253 * (((-(locals.var_t3_dn7 / (locals.var_t3 * locals.var_t3))) * assign32650_e43259) + (assign32650_e43256 * (-(locals.var_t3_dn7 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32650_e43264 * ((((((locals.var_sqrtpsisa_dn7 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn7)) * locals.var_sqrtpsisa) + (assign32650_e43267 * locals.var_sqrtpsisa_dn7)) * assign32650_e43274) + (assign32650_e43269 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7)))) / (assign32650_e43275 * assign32650_e43275)))) - locals.var_t6_dn7), (((assign32650_e43253 * (((-(locals.var_t3_dn8 / (locals.var_t3 * locals.var_t3))) * assign32650_e43259) + (assign32650_e43256 * (-(locals.var_t3_dn8 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32650_e43264 * ((((((locals.var_sqrtpsisa_dn8 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn8)) * locals.var_sqrtpsisa) + (assign32650_e43267 * locals.var_sqrtpsisa_dn8)) * assign32650_e43274) + (assign32650_e43269 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8)))) / (assign32650_e43275 * assign32650_e43275)))) - locals.var_t6_dn8), (((assign32650_e43253 * (((-(locals.var_t3_dn9 / (locals.var_t3 * locals.var_t3))) * assign32650_e43259) + (assign32650_e43256 * (-(locals.var_t3_dn9 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32650_e43264 * ((((((locals.var_sqrtpsisa_dn9 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn9)) * locals.var_sqrtpsisa) + (assign32650_e43267 * locals.var_sqrtpsisa_dn9)) * assign32650_e43274) + (assign32650_e43269 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9)))) / (assign32650_e43275 * assign32650_e43275)))) - locals.var_t6_dn9), (((assign32650_e43253 * (((-(locals.var_t3_dn10 / (locals.var_t3 * locals.var_t3))) * assign32650_e43259) + (assign32650_e43256 * (-(locals.var_t3_dn10 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32650_e43264 * ((((((locals.var_sqrtpsisa_dn10 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn10)) * locals.var_sqrtpsisa) + (assign32650_e43267 * locals.var_sqrtpsisa_dn10)) * assign32650_e43274) + (assign32650_e43269 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10)))) / (assign32650_e43275 * assign32650_e43275)))) - locals.var_t6_dn10), (((assign32650_e43253 * (((-(locals.var_t3_dn11 / (locals.var_t3 * locals.var_t3))) * assign32650_e43259) + (assign32650_e43256 * (-(locals.var_t3_dn11 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32650_e43264 * ((((((locals.var_sqrtpsisa_dn11 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn11)) * locals.var_sqrtpsisa) + (assign32650_e43267 * locals.var_sqrtpsisa_dn11)) * assign32650_e43274) + (assign32650_e43269 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11)))) / (assign32650_e43275 * assign32650_e43275)))) - locals.var_t6_dn11), (((assign32650_e43253 * (((-(locals.var_t3_dn12 / (locals.var_t3 * locals.var_t3))) * assign32650_e43259) + (assign32650_e43256 * (-(locals.var_t3_dn12 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32650_e43264 * ((((((locals.var_sqrtpsisa_dn12 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn12)) * locals.var_sqrtpsisa) + (assign32650_e43267 * locals.var_sqrtpsisa_dn12)) * assign32650_e43274) + (assign32650_e43269 * (((locals.var_t0_dn12 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn12)) + locals.var_sqrtpsisa_dn12)))) / (assign32650_e43275 * assign32650_e43275)))) - locals.var_t6_dn12), (((assign32650_e43253 * (((-(locals.var_t3_dn13 / (locals.var_t3 * locals.var_t3))) * assign32650_e43259) + (assign32650_e43256 * (-(locals.var_t3_dn13 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32650_e43264 * ((((((locals.var_sqrtpsisa_dn13 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn13)) * locals.var_sqrtpsisa) + (assign32650_e43267 * locals.var_sqrtpsisa_dn13)) * assign32650_e43274) + (assign32650_e43269 * (((locals.var_t0_dn13 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn13)) + locals.var_sqrtpsisa_dn13)))) / (assign32650_e43275 * assign32650_e43275)))) - locals.var_t6_dn13), (((assign32650_e43253 * (((-(locals.var_t3_dn14 / (locals.var_t3 * locals.var_t3))) * assign32650_e43259) + (assign32650_e43256 * (-(locals.var_t3_dn14 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32650_e43264 * ((((((locals.var_sqrtpsisa_dn14 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn14)) * locals.var_sqrtpsisa) + (assign32650_e43267 * locals.var_sqrtpsisa_dn14)) * assign32650_e43274) + (assign32650_e43269 * (((locals.var_t0_dn14 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn14)) + locals.var_sqrtpsisa_dn14)))) / (assign32650_e43275 * assign32650_e43275)))) - locals.var_t6_dn14),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn12, locals.var_t7_dn13, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign32650_e43281;
        locals.var_t7_dn0 = assign32650_e43281_d_n0;
        locals.var_t7_dn2 = assign32650_e43281_d_n2;
        locals.var_t7_dn3 = assign32650_e43281_d_n3;
        locals.var_t7_dn4 = assign32650_e43281_d_n4;
        locals.var_t7_dn5 = assign32650_e43281_d_n5;
        locals.var_t7_dn6 = assign32650_e43281_d_n6;
        locals.var_t7_dn7 = assign32650_e43281_d_n7;
        locals.var_t7_dn8 = assign32650_e43281_d_n8;
        locals.var_t7_dn9 = assign32650_e43281_d_n9;
        locals.var_t7_dn10 = assign32650_e43281_d_n10;
        locals.var_t7_dn11 = assign32650_e43281_d_n11;
        locals.var_t7_dn12 = assign32650_e43281_d_n12;
        locals.var_t7_dn13 = assign32650_e43281_d_n13;
        locals.var_t7_dn14 = assign32650_e43281_d_n14;
        locals.var_t7_rv = 0.0;

        let (assign32660_e43304, assign32660_e43304_d_n0, assign32660_e43304_d_n2, assign32660_e43304_d_n3, assign32660_e43304_d_n4, assign32660_e43304_d_n5, assign32660_e43304_d_n6, assign32660_e43304_d_n7, assign32660_e43304_d_n8, assign32660_e43304_d_n9, assign32660_e43304_d_n10, assign32660_e43304_d_n11, assign32660_e43304_d_n12, assign32660_e43304_d_n13, assign32660_e43304_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard740 == 0.0)) {
        let assign32660_e43289: f64 = (locals.var_t4 / locals.var_t5);
        let assign32660_e43293: f64 = (locals.var_t4 * locals.var_t7);
        let assign32660_e43296: f64 = (2.0 * locals.var_t5);
        let assign32660_e43298: f64 = (assign32660_e43296 * locals.var_t5);
        let assign32660_e43299: f64 = (assign32660_e43293 / assign32660_e43298);
        let assign32660_e43300: f64 = (1.0 + assign32660_e43299);
        let assign32660_e43301: f64 = (assign32660_e43289 * assign32660_e43300);
        let assign32660_e43302: f64 = (locals.var_t3 - assign32660_e43301);
        (assign32660_e43302, (locals.var_t3_dn0 - (((((locals.var_t4_dn0 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn0)) / (locals.var_t5 * locals.var_t5)) * assign32660_e43300) + (assign32660_e43289 * (((((locals.var_t4_dn0 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn0)) * assign32660_e43298) - (assign32660_e43293 * (((2.0 * locals.var_t5_dn0) * locals.var_t5) + (assign32660_e43296 * locals.var_t5_dn0)))) / (assign32660_e43298 * assign32660_e43298))))), (locals.var_t3_dn2 - (((((locals.var_t4_dn2 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn2)) / (locals.var_t5 * locals.var_t5)) * assign32660_e43300) + (assign32660_e43289 * (((((locals.var_t4_dn2 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn2)) * assign32660_e43298) - (assign32660_e43293 * (((2.0 * locals.var_t5_dn2) * locals.var_t5) + (assign32660_e43296 * locals.var_t5_dn2)))) / (assign32660_e43298 * assign32660_e43298))))), (locals.var_t3_dn3 - (((((locals.var_t4_dn3 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn3)) / (locals.var_t5 * locals.var_t5)) * assign32660_e43300) + (assign32660_e43289 * (((((locals.var_t4_dn3 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn3)) * assign32660_e43298) - (assign32660_e43293 * (((2.0 * locals.var_t5_dn3) * locals.var_t5) + (assign32660_e43296 * locals.var_t5_dn3)))) / (assign32660_e43298 * assign32660_e43298))))), (locals.var_t3_dn4 - (((((locals.var_t4_dn4 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn4)) / (locals.var_t5 * locals.var_t5)) * assign32660_e43300) + (assign32660_e43289 * (((((locals.var_t4_dn4 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn4)) * assign32660_e43298) - (assign32660_e43293 * (((2.0 * locals.var_t5_dn4) * locals.var_t5) + (assign32660_e43296 * locals.var_t5_dn4)))) / (assign32660_e43298 * assign32660_e43298))))), (locals.var_t3_dn5 - (((((locals.var_t4_dn5 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn5)) / (locals.var_t5 * locals.var_t5)) * assign32660_e43300) + (assign32660_e43289 * (((((locals.var_t4_dn5 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn5)) * assign32660_e43298) - (assign32660_e43293 * (((2.0 * locals.var_t5_dn5) * locals.var_t5) + (assign32660_e43296 * locals.var_t5_dn5)))) / (assign32660_e43298 * assign32660_e43298))))), (locals.var_t3_dn6 - (((((locals.var_t4_dn6 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn6)) / (locals.var_t5 * locals.var_t5)) * assign32660_e43300) + (assign32660_e43289 * (((((locals.var_t4_dn6 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn6)) * assign32660_e43298) - (assign32660_e43293 * (((2.0 * locals.var_t5_dn6) * locals.var_t5) + (assign32660_e43296 * locals.var_t5_dn6)))) / (assign32660_e43298 * assign32660_e43298))))), (locals.var_t3_dn7 - (((((locals.var_t4_dn7 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn7)) / (locals.var_t5 * locals.var_t5)) * assign32660_e43300) + (assign32660_e43289 * (((((locals.var_t4_dn7 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn7)) * assign32660_e43298) - (assign32660_e43293 * (((2.0 * locals.var_t5_dn7) * locals.var_t5) + (assign32660_e43296 * locals.var_t5_dn7)))) / (assign32660_e43298 * assign32660_e43298))))), (locals.var_t3_dn8 - (((((locals.var_t4_dn8 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn8)) / (locals.var_t5 * locals.var_t5)) * assign32660_e43300) + (assign32660_e43289 * (((((locals.var_t4_dn8 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn8)) * assign32660_e43298) - (assign32660_e43293 * (((2.0 * locals.var_t5_dn8) * locals.var_t5) + (assign32660_e43296 * locals.var_t5_dn8)))) / (assign32660_e43298 * assign32660_e43298))))), (locals.var_t3_dn9 - (((((locals.var_t4_dn9 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn9)) / (locals.var_t5 * locals.var_t5)) * assign32660_e43300) + (assign32660_e43289 * (((((locals.var_t4_dn9 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn9)) * assign32660_e43298) - (assign32660_e43293 * (((2.0 * locals.var_t5_dn9) * locals.var_t5) + (assign32660_e43296 * locals.var_t5_dn9)))) / (assign32660_e43298 * assign32660_e43298))))), (locals.var_t3_dn10 - (((((locals.var_t4_dn10 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn10)) / (locals.var_t5 * locals.var_t5)) * assign32660_e43300) + (assign32660_e43289 * (((((locals.var_t4_dn10 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn10)) * assign32660_e43298) - (assign32660_e43293 * (((2.0 * locals.var_t5_dn10) * locals.var_t5) + (assign32660_e43296 * locals.var_t5_dn10)))) / (assign32660_e43298 * assign32660_e43298))))), (locals.var_t3_dn11 - (((((locals.var_t4_dn11 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn11)) / (locals.var_t5 * locals.var_t5)) * assign32660_e43300) + (assign32660_e43289 * (((((locals.var_t4_dn11 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn11)) * assign32660_e43298) - (assign32660_e43293 * (((2.0 * locals.var_t5_dn11) * locals.var_t5) + (assign32660_e43296 * locals.var_t5_dn11)))) / (assign32660_e43298 * assign32660_e43298))))), (locals.var_t3_dn12 - (((((locals.var_t4_dn12 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn12)) / (locals.var_t5 * locals.var_t5)) * assign32660_e43300) + (assign32660_e43289 * (((((locals.var_t4_dn12 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn12)) * assign32660_e43298) - (assign32660_e43293 * (((2.0 * locals.var_t5_dn12) * locals.var_t5) + (assign32660_e43296 * locals.var_t5_dn12)))) / (assign32660_e43298 * assign32660_e43298))))), (locals.var_t3_dn13 - (((((locals.var_t4_dn13 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn13)) / (locals.var_t5 * locals.var_t5)) * assign32660_e43300) + (assign32660_e43289 * (((((locals.var_t4_dn13 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn13)) * assign32660_e43298) - (assign32660_e43293 * (((2.0 * locals.var_t5_dn13) * locals.var_t5) + (assign32660_e43296 * locals.var_t5_dn13)))) / (assign32660_e43298 * assign32660_e43298))))), (locals.var_t3_dn14 - (((((locals.var_t4_dn14 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn14)) / (locals.var_t5 * locals.var_t5)) * assign32660_e43300) + (assign32660_e43289 * (((((locals.var_t4_dn14 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn14)) * assign32660_e43298) - (assign32660_e43293 * (((2.0 * locals.var_t5_dn14) * locals.var_t5) + (assign32660_e43296 * locals.var_t5_dn14)))) / (assign32660_e43298 * assign32660_e43298))))),)
    } else {
        (locals.var_qdeff_edge, locals.var_qdeff_edge_dn0, locals.var_qdeff_edge_dn2, locals.var_qdeff_edge_dn3, locals.var_qdeff_edge_dn4, locals.var_qdeff_edge_dn5, locals.var_qdeff_edge_dn6, locals.var_qdeff_edge_dn7, locals.var_qdeff_edge_dn8, locals.var_qdeff_edge_dn9, locals.var_qdeff_edge_dn10, locals.var_qdeff_edge_dn11, locals.var_qdeff_edge_dn12, locals.var_qdeff_edge_dn13, locals.var_qdeff_edge_dn14,)
    }
};
        locals.var_qdeff_edge = assign32660_e43304;
        locals.var_qdeff_edge_dn0 = assign32660_e43304_d_n0;
        locals.var_qdeff_edge_dn2 = assign32660_e43304_d_n2;
        locals.var_qdeff_edge_dn3 = assign32660_e43304_d_n3;
        locals.var_qdeff_edge_dn4 = assign32660_e43304_d_n4;
        locals.var_qdeff_edge_dn5 = assign32660_e43304_d_n5;
        locals.var_qdeff_edge_dn6 = assign32660_e43304_d_n6;
        locals.var_qdeff_edge_dn7 = assign32660_e43304_d_n7;
        locals.var_qdeff_edge_dn8 = assign32660_e43304_d_n8;
        locals.var_qdeff_edge_dn9 = assign32660_e43304_d_n9;
        locals.var_qdeff_edge_dn10 = assign32660_e43304_d_n10;
        locals.var_qdeff_edge_dn11 = assign32660_e43304_d_n11;
        locals.var_qdeff_edge_dn12 = assign32660_e43304_d_n12;
        locals.var_qdeff_edge_dn13 = assign32660_e43304_d_n13;
        locals.var_qdeff_edge_dn14 = assign32660_e43304_d_n14;
        locals.var_qdeff_edge_rv = 0.0;

        let assign32670_e43310: f64 = (-2500.0);
        let assign32670_e43312: f64 = (assign32670_e43310 * 2.0);
        let assign32670_e43314: f64 = if ((1.0 == 0.0) && (locals.var_psip < assign32670_e43312)) { 1.0 } else { 0.0 };
        locals.var_guard743 = assign32670_e43314;
        locals.var_guard743_rv = 0.0;

        let (assign32680_e43327, assign32680_e43327_d_n0, assign32680_e43327_d_n2, assign32680_e43327_d_n3, assign32680_e43327_d_n4, assign32680_e43327_d_n5, assign32680_e43327_d_n6, assign32680_e43327_d_n7, assign32680_e43327_d_n8, assign32680_e43327_d_n9, assign32680_e43327_d_n10, assign32680_e43327_d_n11, assign32680_e43327_d_n12, assign32680_e43327_d_n13, assign32680_e43327_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard743 != 0.0)) {
        let assign32680_e43319: f64 = (-2.0);
        let assign32680_e43321: f64 = (assign32680_e43319 * 2.0);
        let assign32680_e43324: f64 = (16.0 * locals.var_psip);
        let assign32680_e43325: f64 = (assign32680_e43321 / assign32680_e43324);
        (assign32680_e43325, (-((assign32680_e43321 * (16.0 * locals.var_psip_dn0)) / (assign32680_e43324 * assign32680_e43324))), (-((assign32680_e43321 * (16.0 * locals.var_psip_dn2)) / (assign32680_e43324 * assign32680_e43324))), (-((assign32680_e43321 * (16.0 * locals.var_psip_dn3)) / (assign32680_e43324 * assign32680_e43324))), (-((assign32680_e43321 * (16.0 * locals.var_psip_dn4)) / (assign32680_e43324 * assign32680_e43324))), (-((assign32680_e43321 * (16.0 * locals.var_psip_dn5)) / (assign32680_e43324 * assign32680_e43324))), (-((assign32680_e43321 * (16.0 * locals.var_psip_dn6)) / (assign32680_e43324 * assign32680_e43324))), (-((assign32680_e43321 * (16.0 * locals.var_psip_dn7)) / (assign32680_e43324 * assign32680_e43324))), (-((assign32680_e43321 * (16.0 * locals.var_psip_dn8)) / (assign32680_e43324 * assign32680_e43324))), (-((assign32680_e43321 * (16.0 * locals.var_psip_dn9)) / (assign32680_e43324 * assign32680_e43324))), (-((assign32680_e43321 * (16.0 * locals.var_psip_dn10)) / (assign32680_e43324 * assign32680_e43324))), (-((assign32680_e43321 * (16.0 * locals.var_psip_dn11)) / (assign32680_e43324 * assign32680_e43324))), (-((assign32680_e43321 * (16.0 * locals.var_psip_dn12)) / (assign32680_e43324 * assign32680_e43324))), (-((assign32680_e43321 * (16.0 * locals.var_psip_dn13)) / (assign32680_e43324 * assign32680_e43324))), (-((assign32680_e43321 * (16.0 * locals.var_psip_dn14)) / (assign32680_e43324 * assign32680_e43324))),)
    } else {
        (locals.var_psipclamp, locals.var_psipclamp_dn0, locals.var_psipclamp_dn2, locals.var_psipclamp_dn3, locals.var_psipclamp_dn4, locals.var_psipclamp_dn5, locals.var_psipclamp_dn6, locals.var_psipclamp_dn7, locals.var_psipclamp_dn8, locals.var_psipclamp_dn9, locals.var_psipclamp_dn10, locals.var_psipclamp_dn11, locals.var_psipclamp_dn12, locals.var_psipclamp_dn13, locals.var_psipclamp_dn14,)
    }
};
        locals.var_psipclamp = assign32680_e43327;
        locals.var_psipclamp_dn0 = assign32680_e43327_d_n0;
        locals.var_psipclamp_dn2 = assign32680_e43327_d_n2;
        locals.var_psipclamp_dn3 = assign32680_e43327_d_n3;
        locals.var_psipclamp_dn4 = assign32680_e43327_d_n4;
        locals.var_psipclamp_dn5 = assign32680_e43327_d_n5;
        locals.var_psipclamp_dn6 = assign32680_e43327_d_n6;
        locals.var_psipclamp_dn7 = assign32680_e43327_d_n7;
        locals.var_psipclamp_dn8 = assign32680_e43327_d_n8;
        locals.var_psipclamp_dn9 = assign32680_e43327_d_n9;
        locals.var_psipclamp_dn10 = assign32680_e43327_d_n10;
        locals.var_psipclamp_dn11 = assign32680_e43327_d_n11;
        locals.var_psipclamp_dn12 = assign32680_e43327_d_n12;
        locals.var_psipclamp_dn13 = assign32680_e43327_d_n13;
        locals.var_psipclamp_dn14 = assign32680_e43327_d_n14;
        locals.var_psipclamp_rv = 0.0;

        let (assign32690_e43353, assign32690_e43353_d_n0, assign32690_e43353_d_n2, assign32690_e43353_d_n3, assign32690_e43353_d_n4, assign32690_e43353_d_n5, assign32690_e43353_d_n6, assign32690_e43353_d_n7, assign32690_e43353_d_n8, assign32690_e43353_d_n9, assign32690_e43353_d_n10, assign32690_e43353_d_n11, assign32690_e43353_d_n12, assign32690_e43353_d_n13, assign32690_e43353_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard743 == 0.0)) {
        let assign32690_e43335: f64 = (locals.var_psip + 1.0);
        let assign32690_e43338: f64 = (locals.var_psip - 1.0);
        let assign32690_e43341: f64 = (locals.var_psip - 1.0);
        let assign32690_e43342: f64 = (assign32690_e43338 * assign32690_e43341);
        let assign32690_e43345: f64 = (0.25 * 2.0);
        let assign32690_e43347: f64 = (assign32690_e43345 * 2.0);
        let assign32690_e43348: f64 = (assign32690_e43342 + assign32690_e43347);
        let assign32690_e43349: f64 = (assign32690_e43348).sqrt();
        let assign32690_e43350: f64 = (assign32690_e43335 + assign32690_e43349);
        let assign32690_e43351: f64 = (0.5 * assign32690_e43350);
        (assign32690_e43351, (0.5 * (locals.var_psip_dn0 + (((locals.var_psip_dn0 * assign32690_e43341) + (assign32690_e43338 * locals.var_psip_dn0)) / (2.0 * assign32690_e43349)))), (0.5 * (locals.var_psip_dn2 + (((locals.var_psip_dn2 * assign32690_e43341) + (assign32690_e43338 * locals.var_psip_dn2)) / (2.0 * assign32690_e43349)))), (0.5 * (locals.var_psip_dn3 + (((locals.var_psip_dn3 * assign32690_e43341) + (assign32690_e43338 * locals.var_psip_dn3)) / (2.0 * assign32690_e43349)))), (0.5 * (locals.var_psip_dn4 + (((locals.var_psip_dn4 * assign32690_e43341) + (assign32690_e43338 * locals.var_psip_dn4)) / (2.0 * assign32690_e43349)))), (0.5 * (locals.var_psip_dn5 + (((locals.var_psip_dn5 * assign32690_e43341) + (assign32690_e43338 * locals.var_psip_dn5)) / (2.0 * assign32690_e43349)))), (0.5 * (locals.var_psip_dn6 + (((locals.var_psip_dn6 * assign32690_e43341) + (assign32690_e43338 * locals.var_psip_dn6)) / (2.0 * assign32690_e43349)))), (0.5 * (locals.var_psip_dn7 + (((locals.var_psip_dn7 * assign32690_e43341) + (assign32690_e43338 * locals.var_psip_dn7)) / (2.0 * assign32690_e43349)))), (0.5 * (locals.var_psip_dn8 + (((locals.var_psip_dn8 * assign32690_e43341) + (assign32690_e43338 * locals.var_psip_dn8)) / (2.0 * assign32690_e43349)))), (0.5 * (locals.var_psip_dn9 + (((locals.var_psip_dn9 * assign32690_e43341) + (assign32690_e43338 * locals.var_psip_dn9)) / (2.0 * assign32690_e43349)))), (0.5 * (locals.var_psip_dn10 + (((locals.var_psip_dn10 * assign32690_e43341) + (assign32690_e43338 * locals.var_psip_dn10)) / (2.0 * assign32690_e43349)))), (0.5 * (locals.var_psip_dn11 + (((locals.var_psip_dn11 * assign32690_e43341) + (assign32690_e43338 * locals.var_psip_dn11)) / (2.0 * assign32690_e43349)))), (0.5 * (locals.var_psip_dn12 + (((locals.var_psip_dn12 * assign32690_e43341) + (assign32690_e43338 * locals.var_psip_dn12)) / (2.0 * assign32690_e43349)))), (0.5 * (locals.var_psip_dn13 + (((locals.var_psip_dn13 * assign32690_e43341) + (assign32690_e43338 * locals.var_psip_dn13)) / (2.0 * assign32690_e43349)))), (0.5 * (locals.var_psip_dn14 + (((locals.var_psip_dn14 * assign32690_e43341) + (assign32690_e43338 * locals.var_psip_dn14)) / (2.0 * assign32690_e43349)))),)
    } else {
        (locals.var_psipclamp, locals.var_psipclamp_dn0, locals.var_psipclamp_dn2, locals.var_psipclamp_dn3, locals.var_psipclamp_dn4, locals.var_psipclamp_dn5, locals.var_psipclamp_dn6, locals.var_psipclamp_dn7, locals.var_psipclamp_dn8, locals.var_psipclamp_dn9, locals.var_psipclamp_dn10, locals.var_psipclamp_dn11, locals.var_psipclamp_dn12, locals.var_psipclamp_dn13, locals.var_psipclamp_dn14,)
    }
};
        locals.var_psipclamp = assign32690_e43353;
        locals.var_psipclamp_dn0 = assign32690_e43353_d_n0;
        locals.var_psipclamp_dn2 = assign32690_e43353_d_n2;
        locals.var_psipclamp_dn3 = assign32690_e43353_d_n3;
        locals.var_psipclamp_dn4 = assign32690_e43353_d_n4;
        locals.var_psipclamp_dn5 = assign32690_e43353_d_n5;
        locals.var_psipclamp_dn6 = assign32690_e43353_d_n6;
        locals.var_psipclamp_dn7 = assign32690_e43353_d_n7;
        locals.var_psipclamp_dn8 = assign32690_e43353_d_n8;
        locals.var_psipclamp_dn9 = assign32690_e43353_d_n9;
        locals.var_psipclamp_dn10 = assign32690_e43353_d_n10;
        locals.var_psipclamp_dn11 = assign32690_e43353_d_n11;
        locals.var_psipclamp_dn12 = assign32690_e43353_d_n12;
        locals.var_psipclamp_dn13 = assign32690_e43353_d_n13;
        locals.var_psipclamp_dn14 = assign32690_e43353_d_n14;
        locals.var_psipclamp_rv = 0.0;

        let (assign32700_e43358, assign32700_e43358_d_n0, assign32700_e43358_d_n2, assign32700_e43358_d_n3, assign32700_e43358_d_n4, assign32700_e43358_d_n5, assign32700_e43358_d_n6, assign32700_e43358_d_n7, assign32700_e43358_d_n8, assign32700_e43358_d_n9, assign32700_e43358_d_n10, assign32700_e43358_d_n11, assign32700_e43358_d_n12, assign32700_e43358_d_n13, assign32700_e43358_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32700_e43356: f64 = (locals.var_psipclamp).sqrt();
        (assign32700_e43356, (locals.var_psipclamp_dn0 / (2.0 * assign32700_e43356)), (locals.var_psipclamp_dn2 / (2.0 * assign32700_e43356)), (locals.var_psipclamp_dn3 / (2.0 * assign32700_e43356)), (locals.var_psipclamp_dn4 / (2.0 * assign32700_e43356)), (locals.var_psipclamp_dn5 / (2.0 * assign32700_e43356)), (locals.var_psipclamp_dn6 / (2.0 * assign32700_e43356)), (locals.var_psipclamp_dn7 / (2.0 * assign32700_e43356)), (locals.var_psipclamp_dn8 / (2.0 * assign32700_e43356)), (locals.var_psipclamp_dn9 / (2.0 * assign32700_e43356)), (locals.var_psipclamp_dn10 / (2.0 * assign32700_e43356)), (locals.var_psipclamp_dn11 / (2.0 * assign32700_e43356)), (locals.var_psipclamp_dn12 / (2.0 * assign32700_e43356)), (locals.var_psipclamp_dn13 / (2.0 * assign32700_e43356)), (locals.var_psipclamp_dn14 / (2.0 * assign32700_e43356)),)
    } else {
        (locals.var_sqrtpsip, locals.var_sqrtpsip_dn0, locals.var_sqrtpsip_dn2, locals.var_sqrtpsip_dn3, locals.var_sqrtpsip_dn4, locals.var_sqrtpsip_dn5, locals.var_sqrtpsip_dn6, locals.var_sqrtpsip_dn7, locals.var_sqrtpsip_dn8, locals.var_sqrtpsip_dn9, locals.var_sqrtpsip_dn10, locals.var_sqrtpsip_dn11, locals.var_sqrtpsip_dn12, locals.var_sqrtpsip_dn13, locals.var_sqrtpsip_dn14,)
    }
};
        locals.var_sqrtpsip = assign32700_e43358;
        locals.var_sqrtpsip_dn0 = assign32700_e43358_d_n0;
        locals.var_sqrtpsip_dn2 = assign32700_e43358_d_n2;
        locals.var_sqrtpsip_dn3 = assign32700_e43358_d_n3;
        locals.var_sqrtpsip_dn4 = assign32700_e43358_d_n4;
        locals.var_sqrtpsip_dn5 = assign32700_e43358_d_n5;
        locals.var_sqrtpsip_dn6 = assign32700_e43358_d_n6;
        locals.var_sqrtpsip_dn7 = assign32700_e43358_d_n7;
        locals.var_sqrtpsip_dn8 = assign32700_e43358_d_n8;
        locals.var_sqrtpsip_dn9 = assign32700_e43358_d_n9;
        locals.var_sqrtpsip_dn10 = assign32700_e43358_d_n10;
        locals.var_sqrtpsip_dn11 = assign32700_e43358_d_n11;
        locals.var_sqrtpsip_dn12 = assign32700_e43358_d_n12;
        locals.var_sqrtpsip_dn13 = assign32700_e43358_d_n13;
        locals.var_sqrtpsip_dn14 = assign32700_e43358_d_n14;
        locals.var_sqrtpsip_rv = 0.0;

        let (assign32710_e43368, assign32710_e43368_d_n0, assign32710_e43368_d_n2, assign32710_e43368_d_n3, assign32710_e43368_d_n4, assign32710_e43368_d_n5, assign32710_e43368_d_n6, assign32710_e43368_d_n7, assign32710_e43368_d_n8, assign32710_e43368_d_n9, assign32710_e43368_d_n10, assign32710_e43368_d_n11, assign32710_e43368_d_n12, assign32710_e43368_d_n13, assign32710_e43368_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32710_e43362: f64 = (locals.var_psip - locals.var_qs_edge);
        let assign32710_e43364: f64 = (assign32710_e43362 - locals.var_qdeff_edge);
        let assign32710_e43366: f64 = (assign32710_e43364 - 1.0);
        (assign32710_e43366, ((locals.var_psip_dn0 - locals.var_qs_edge_dn0) - locals.var_qdeff_edge_dn0), ((locals.var_psip_dn2 - locals.var_qs_edge_dn2) - locals.var_qdeff_edge_dn2), ((locals.var_psip_dn3 - locals.var_qs_edge_dn3) - locals.var_qdeff_edge_dn3), ((locals.var_psip_dn4 - locals.var_qs_edge_dn4) - locals.var_qdeff_edge_dn4), ((locals.var_psip_dn5 - locals.var_qs_edge_dn5) - locals.var_qdeff_edge_dn5), ((locals.var_psip_dn6 - locals.var_qs_edge_dn6) - locals.var_qdeff_edge_dn6), ((locals.var_psip_dn7 - locals.var_qs_edge_dn7) - locals.var_qdeff_edge_dn7), ((locals.var_psip_dn8 - locals.var_qs_edge_dn8) - locals.var_qdeff_edge_dn8), ((locals.var_psip_dn9 - locals.var_qs_edge_dn9) - locals.var_qdeff_edge_dn9), ((locals.var_psip_dn10 - locals.var_qs_edge_dn10) - locals.var_qdeff_edge_dn10), ((locals.var_psip_dn11 - locals.var_qs_edge_dn11) - locals.var_qdeff_edge_dn11), ((locals.var_psip_dn12 - locals.var_qs_edge_dn12) - locals.var_qdeff_edge_dn12), ((locals.var_psip_dn13 - locals.var_qs_edge_dn13) - locals.var_qdeff_edge_dn13), ((locals.var_psip_dn14 - locals.var_qs_edge_dn14) - locals.var_qdeff_edge_dn14),)
    } else {
        (locals.var_psiavg, locals.var_psiavg_dn0, locals.var_psiavg_dn2, locals.var_psiavg_dn3, locals.var_psiavg_dn4, locals.var_psiavg_dn5, locals.var_psiavg_dn6, locals.var_psiavg_dn7, locals.var_psiavg_dn8, locals.var_psiavg_dn9, locals.var_psiavg_dn10, locals.var_psiavg_dn11, locals.var_psiavg_dn12, locals.var_psiavg_dn13, locals.var_psiavg_dn14,)
    }
};
        locals.var_psiavg = assign32710_e43368;
        locals.var_psiavg_dn0 = assign32710_e43368_d_n0;
        locals.var_psiavg_dn2 = assign32710_e43368_d_n2;
        locals.var_psiavg_dn3 = assign32710_e43368_d_n3;
        locals.var_psiavg_dn4 = assign32710_e43368_d_n4;
        locals.var_psiavg_dn5 = assign32710_e43368_d_n5;
        locals.var_psiavg_dn6 = assign32710_e43368_d_n6;
        locals.var_psiavg_dn7 = assign32710_e43368_d_n7;
        locals.var_psiavg_dn8 = assign32710_e43368_d_n8;
        locals.var_psiavg_dn9 = assign32710_e43368_d_n9;
        locals.var_psiavg_dn10 = assign32710_e43368_d_n10;
        locals.var_psiavg_dn11 = assign32710_e43368_d_n11;
        locals.var_psiavg_dn12 = assign32710_e43368_d_n12;
        locals.var_psiavg_dn13 = assign32710_e43368_d_n13;
        locals.var_psiavg_dn14 = assign32710_e43368_d_n14;
        locals.var_psiavg_rv = 0.0;

        let assign32720_e43374: f64 = (-2500.0);
        let assign32720_e43376: f64 = (assign32720_e43374 * 2.0);
        let assign32720_e43378: f64 = if ((1.0 == 0.0) && (locals.var_psiavg < assign32720_e43376)) { 1.0 } else { 0.0 };
        locals.var_guard744 = assign32720_e43378;
        locals.var_guard744_rv = 0.0;

        let (assign32730_e43391, assign32730_e43391_d_n0, assign32730_e43391_d_n2, assign32730_e43391_d_n3, assign32730_e43391_d_n4, assign32730_e43391_d_n5, assign32730_e43391_d_n6, assign32730_e43391_d_n7, assign32730_e43391_d_n8, assign32730_e43391_d_n9, assign32730_e43391_d_n10, assign32730_e43391_d_n11, assign32730_e43391_d_n12, assign32730_e43391_d_n13, assign32730_e43391_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard744 != 0.0)) {
        let assign32730_e43383: f64 = (-2.0);
        let assign32730_e43385: f64 = (assign32730_e43383 * 2.0);
        let assign32730_e43388: f64 = (16.0 * locals.var_psiavg);
        let assign32730_e43389: f64 = (assign32730_e43385 / assign32730_e43388);
        (assign32730_e43389, (-((assign32730_e43385 * (16.0 * locals.var_psiavg_dn0)) / (assign32730_e43388 * assign32730_e43388))), (-((assign32730_e43385 * (16.0 * locals.var_psiavg_dn2)) / (assign32730_e43388 * assign32730_e43388))), (-((assign32730_e43385 * (16.0 * locals.var_psiavg_dn3)) / (assign32730_e43388 * assign32730_e43388))), (-((assign32730_e43385 * (16.0 * locals.var_psiavg_dn4)) / (assign32730_e43388 * assign32730_e43388))), (-((assign32730_e43385 * (16.0 * locals.var_psiavg_dn5)) / (assign32730_e43388 * assign32730_e43388))), (-((assign32730_e43385 * (16.0 * locals.var_psiavg_dn6)) / (assign32730_e43388 * assign32730_e43388))), (-((assign32730_e43385 * (16.0 * locals.var_psiavg_dn7)) / (assign32730_e43388 * assign32730_e43388))), (-((assign32730_e43385 * (16.0 * locals.var_psiavg_dn8)) / (assign32730_e43388 * assign32730_e43388))), (-((assign32730_e43385 * (16.0 * locals.var_psiavg_dn9)) / (assign32730_e43388 * assign32730_e43388))), (-((assign32730_e43385 * (16.0 * locals.var_psiavg_dn10)) / (assign32730_e43388 * assign32730_e43388))), (-((assign32730_e43385 * (16.0 * locals.var_psiavg_dn11)) / (assign32730_e43388 * assign32730_e43388))), (-((assign32730_e43385 * (16.0 * locals.var_psiavg_dn12)) / (assign32730_e43388 * assign32730_e43388))), (-((assign32730_e43385 * (16.0 * locals.var_psiavg_dn13)) / (assign32730_e43388 * assign32730_e43388))), (-((assign32730_e43385 * (16.0 * locals.var_psiavg_dn14)) / (assign32730_e43388 * assign32730_e43388))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign32730_e43391;
        locals.var_t0_dn0 = assign32730_e43391_d_n0;
        locals.var_t0_dn2 = assign32730_e43391_d_n2;
        locals.var_t0_dn3 = assign32730_e43391_d_n3;
        locals.var_t0_dn4 = assign32730_e43391_d_n4;
        locals.var_t0_dn5 = assign32730_e43391_d_n5;
        locals.var_t0_dn6 = assign32730_e43391_d_n6;
        locals.var_t0_dn7 = assign32730_e43391_d_n7;
        locals.var_t0_dn8 = assign32730_e43391_d_n8;
        locals.var_t0_dn9 = assign32730_e43391_d_n9;
        locals.var_t0_dn10 = assign32730_e43391_d_n10;
        locals.var_t0_dn11 = assign32730_e43391_d_n11;
        locals.var_t0_dn12 = assign32730_e43391_d_n12;
        locals.var_t0_dn13 = assign32730_e43391_d_n13;
        locals.var_t0_dn14 = assign32730_e43391_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign32740_e43417, assign32740_e43417_d_n0, assign32740_e43417_d_n2, assign32740_e43417_d_n3, assign32740_e43417_d_n4, assign32740_e43417_d_n5, assign32740_e43417_d_n6, assign32740_e43417_d_n7, assign32740_e43417_d_n8, assign32740_e43417_d_n9, assign32740_e43417_d_n10, assign32740_e43417_d_n11, assign32740_e43417_d_n12, assign32740_e43417_d_n13, assign32740_e43417_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard744 == 0.0)) {
        let assign32740_e43399: f64 = (locals.var_psiavg + 1.0);
        let assign32740_e43402: f64 = (locals.var_psiavg - 1.0);
        let assign32740_e43405: f64 = (locals.var_psiavg - 1.0);
        let assign32740_e43406: f64 = (assign32740_e43402 * assign32740_e43405);
        let assign32740_e43409: f64 = (0.25 * 2.0);
        let assign32740_e43411: f64 = (assign32740_e43409 * 2.0);
        let assign32740_e43412: f64 = (assign32740_e43406 + assign32740_e43411);
        let assign32740_e43413: f64 = (assign32740_e43412).sqrt();
        let assign32740_e43414: f64 = (assign32740_e43399 + assign32740_e43413);
        let assign32740_e43415: f64 = (0.5 * assign32740_e43414);
        (assign32740_e43415, (0.5 * (locals.var_psiavg_dn0 + (((locals.var_psiavg_dn0 * assign32740_e43405) + (assign32740_e43402 * locals.var_psiavg_dn0)) / (2.0 * assign32740_e43413)))), (0.5 * (locals.var_psiavg_dn2 + (((locals.var_psiavg_dn2 * assign32740_e43405) + (assign32740_e43402 * locals.var_psiavg_dn2)) / (2.0 * assign32740_e43413)))), (0.5 * (locals.var_psiavg_dn3 + (((locals.var_psiavg_dn3 * assign32740_e43405) + (assign32740_e43402 * locals.var_psiavg_dn3)) / (2.0 * assign32740_e43413)))), (0.5 * (locals.var_psiavg_dn4 + (((locals.var_psiavg_dn4 * assign32740_e43405) + (assign32740_e43402 * locals.var_psiavg_dn4)) / (2.0 * assign32740_e43413)))), (0.5 * (locals.var_psiavg_dn5 + (((locals.var_psiavg_dn5 * assign32740_e43405) + (assign32740_e43402 * locals.var_psiavg_dn5)) / (2.0 * assign32740_e43413)))), (0.5 * (locals.var_psiavg_dn6 + (((locals.var_psiavg_dn6 * assign32740_e43405) + (assign32740_e43402 * locals.var_psiavg_dn6)) / (2.0 * assign32740_e43413)))), (0.5 * (locals.var_psiavg_dn7 + (((locals.var_psiavg_dn7 * assign32740_e43405) + (assign32740_e43402 * locals.var_psiavg_dn7)) / (2.0 * assign32740_e43413)))), (0.5 * (locals.var_psiavg_dn8 + (((locals.var_psiavg_dn8 * assign32740_e43405) + (assign32740_e43402 * locals.var_psiavg_dn8)) / (2.0 * assign32740_e43413)))), (0.5 * (locals.var_psiavg_dn9 + (((locals.var_psiavg_dn9 * assign32740_e43405) + (assign32740_e43402 * locals.var_psiavg_dn9)) / (2.0 * assign32740_e43413)))), (0.5 * (locals.var_psiavg_dn10 + (((locals.var_psiavg_dn10 * assign32740_e43405) + (assign32740_e43402 * locals.var_psiavg_dn10)) / (2.0 * assign32740_e43413)))), (0.5 * (locals.var_psiavg_dn11 + (((locals.var_psiavg_dn11 * assign32740_e43405) + (assign32740_e43402 * locals.var_psiavg_dn11)) / (2.0 * assign32740_e43413)))), (0.5 * (locals.var_psiavg_dn12 + (((locals.var_psiavg_dn12 * assign32740_e43405) + (assign32740_e43402 * locals.var_psiavg_dn12)) / (2.0 * assign32740_e43413)))), (0.5 * (locals.var_psiavg_dn13 + (((locals.var_psiavg_dn13 * assign32740_e43405) + (assign32740_e43402 * locals.var_psiavg_dn13)) / (2.0 * assign32740_e43413)))), (0.5 * (locals.var_psiavg_dn14 + (((locals.var_psiavg_dn14 * assign32740_e43405) + (assign32740_e43402 * locals.var_psiavg_dn14)) / (2.0 * assign32740_e43413)))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign32740_e43417;
        locals.var_t0_dn0 = assign32740_e43417_d_n0;
        locals.var_t0_dn2 = assign32740_e43417_d_n2;
        locals.var_t0_dn3 = assign32740_e43417_d_n3;
        locals.var_t0_dn4 = assign32740_e43417_d_n4;
        locals.var_t0_dn5 = assign32740_e43417_d_n5;
        locals.var_t0_dn6 = assign32740_e43417_d_n6;
        locals.var_t0_dn7 = assign32740_e43417_d_n7;
        locals.var_t0_dn8 = assign32740_e43417_d_n8;
        locals.var_t0_dn9 = assign32740_e43417_d_n9;
        locals.var_t0_dn10 = assign32740_e43417_d_n10;
        locals.var_t0_dn11 = assign32740_e43417_d_n11;
        locals.var_t0_dn12 = assign32740_e43417_d_n12;
        locals.var_t0_dn13 = assign32740_e43417_d_n13;
        locals.var_t0_dn14 = assign32740_e43417_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign32750_e43422, assign32750_e43422_d_n0, assign32750_e43422_d_n2, assign32750_e43422_d_n3, assign32750_e43422_d_n4, assign32750_e43422_d_n5, assign32750_e43422_d_n6, assign32750_e43422_d_n7, assign32750_e43422_d_n8, assign32750_e43422_d_n9, assign32750_e43422_d_n10, assign32750_e43422_d_n11, assign32750_e43422_d_n12, assign32750_e43422_d_n13, assign32750_e43422_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32750_e43420: f64 = (locals.var_t0).sqrt();
        (assign32750_e43420, (locals.var_t0_dn0 / (2.0 * assign32750_e43420)), (locals.var_t0_dn2 / (2.0 * assign32750_e43420)), (locals.var_t0_dn3 / (2.0 * assign32750_e43420)), (locals.var_t0_dn4 / (2.0 * assign32750_e43420)), (locals.var_t0_dn5 / (2.0 * assign32750_e43420)), (locals.var_t0_dn6 / (2.0 * assign32750_e43420)), (locals.var_t0_dn7 / (2.0 * assign32750_e43420)), (locals.var_t0_dn8 / (2.0 * assign32750_e43420)), (locals.var_t0_dn9 / (2.0 * assign32750_e43420)), (locals.var_t0_dn10 / (2.0 * assign32750_e43420)), (locals.var_t0_dn11 / (2.0 * assign32750_e43420)), (locals.var_t0_dn12 / (2.0 * assign32750_e43420)), (locals.var_t0_dn13 / (2.0 * assign32750_e43420)), (locals.var_t0_dn14 / (2.0 * assign32750_e43420)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign32750_e43422;
        locals.var_t2_dn0 = assign32750_e43422_d_n0;
        locals.var_t2_dn2 = assign32750_e43422_d_n2;
        locals.var_t2_dn3 = assign32750_e43422_d_n3;
        locals.var_t2_dn4 = assign32750_e43422_d_n4;
        locals.var_t2_dn5 = assign32750_e43422_d_n5;
        locals.var_t2_dn6 = assign32750_e43422_d_n6;
        locals.var_t2_dn7 = assign32750_e43422_d_n7;
        locals.var_t2_dn8 = assign32750_e43422_d_n8;
        locals.var_t2_dn9 = assign32750_e43422_d_n9;
        locals.var_t2_dn10 = assign32750_e43422_d_n10;
        locals.var_t2_dn11 = assign32750_e43422_d_n11;
        locals.var_t2_dn12 = assign32750_e43422_d_n12;
        locals.var_t2_dn13 = assign32750_e43422_d_n13;
        locals.var_t2_dn14 = assign32750_e43422_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign32760_e43432, assign32760_e43432_d_n0, assign32760_e43432_d_n2, assign32760_e43432_d_n3, assign32760_e43432_d_n4, assign32760_e43432_d_n5, assign32760_e43432_d_n6, assign32760_e43432_d_n7, assign32760_e43432_d_n8, assign32760_e43432_d_n9, assign32760_e43432_d_n10, assign32760_e43432_d_n11, assign32760_e43432_d_n12, assign32760_e43432_d_n13, assign32760_e43432_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32760_e43428: f64 = (locals.var_sqrtpsip + locals.var_t2);
        let assign32760_e43429: f64 = (locals.var_gam_edge / assign32760_e43428);
        let assign32760_e43430: f64 = (1.0 + assign32760_e43429);
        (assign32760_e43430, (((locals.var_gam_edge_dn0 * assign32760_e43428) - (locals.var_gam_edge * (locals.var_sqrtpsip_dn0 + locals.var_t2_dn0))) / (assign32760_e43428 * assign32760_e43428)), (((locals.var_gam_edge_dn2 * assign32760_e43428) - (locals.var_gam_edge * (locals.var_sqrtpsip_dn2 + locals.var_t2_dn2))) / (assign32760_e43428 * assign32760_e43428)), (((locals.var_gam_edge_dn3 * assign32760_e43428) - (locals.var_gam_edge * (locals.var_sqrtpsip_dn3 + locals.var_t2_dn3))) / (assign32760_e43428 * assign32760_e43428)), (((locals.var_gam_edge_dn4 * assign32760_e43428) - (locals.var_gam_edge * (locals.var_sqrtpsip_dn4 + locals.var_t2_dn4))) / (assign32760_e43428 * assign32760_e43428)), (((locals.var_gam_edge_dn5 * assign32760_e43428) - (locals.var_gam_edge * (locals.var_sqrtpsip_dn5 + locals.var_t2_dn5))) / (assign32760_e43428 * assign32760_e43428)), (((locals.var_gam_edge_dn6 * assign32760_e43428) - (locals.var_gam_edge * (locals.var_sqrtpsip_dn6 + locals.var_t2_dn6))) / (assign32760_e43428 * assign32760_e43428)), (((locals.var_gam_edge_dn7 * assign32760_e43428) - (locals.var_gam_edge * (locals.var_sqrtpsip_dn7 + locals.var_t2_dn7))) / (assign32760_e43428 * assign32760_e43428)), (((locals.var_gam_edge_dn8 * assign32760_e43428) - (locals.var_gam_edge * (locals.var_sqrtpsip_dn8 + locals.var_t2_dn8))) / (assign32760_e43428 * assign32760_e43428)), (((locals.var_gam_edge_dn9 * assign32760_e43428) - (locals.var_gam_edge * (locals.var_sqrtpsip_dn9 + locals.var_t2_dn9))) / (assign32760_e43428 * assign32760_e43428)), (((locals.var_gam_edge_dn10 * assign32760_e43428) - (locals.var_gam_edge * (locals.var_sqrtpsip_dn10 + locals.var_t2_dn10))) / (assign32760_e43428 * assign32760_e43428)), (((locals.var_gam_edge_dn11 * assign32760_e43428) - (locals.var_gam_edge * (locals.var_sqrtpsip_dn11 + locals.var_t2_dn11))) / (assign32760_e43428 * assign32760_e43428)), (((locals.var_gam_edge_dn12 * assign32760_e43428) - (locals.var_gam_edge * (locals.var_sqrtpsip_dn12 + locals.var_t2_dn12))) / (assign32760_e43428 * assign32760_e43428)), (((locals.var_gam_edge_dn13 * assign32760_e43428) - (locals.var_gam_edge * (locals.var_sqrtpsip_dn13 + locals.var_t2_dn13))) / (assign32760_e43428 * assign32760_e43428)), (((locals.var_gam_edge_dn14 * assign32760_e43428) - (locals.var_gam_edge * (locals.var_sqrtpsip_dn14 + locals.var_t2_dn14))) / (assign32760_e43428 * assign32760_e43428)),)
    } else {
        (locals.var_nq_edge, locals.var_nq_edge_dn0, locals.var_nq_edge_dn2, locals.var_nq_edge_dn3, locals.var_nq_edge_dn4, locals.var_nq_edge_dn5, locals.var_nq_edge_dn6, locals.var_nq_edge_dn7, locals.var_nq_edge_dn8, locals.var_nq_edge_dn9, locals.var_nq_edge_dn10, locals.var_nq_edge_dn11, locals.var_nq_edge_dn12, locals.var_nq_edge_dn13, locals.var_nq_edge_dn14,)
    }
};
        locals.var_nq_edge = assign32760_e43432;
        locals.var_nq_edge_dn0 = assign32760_e43432_d_n0;
        locals.var_nq_edge_dn2 = assign32760_e43432_d_n2;
        locals.var_nq_edge_dn3 = assign32760_e43432_d_n3;
        locals.var_nq_edge_dn4 = assign32760_e43432_d_n4;
        locals.var_nq_edge_dn5 = assign32760_e43432_d_n5;
        locals.var_nq_edge_dn6 = assign32760_e43432_d_n6;
        locals.var_nq_edge_dn7 = assign32760_e43432_d_n7;
        locals.var_nq_edge_dn8 = assign32760_e43432_d_n8;
        locals.var_nq_edge_dn9 = assign32760_e43432_d_n9;
        locals.var_nq_edge_dn10 = assign32760_e43432_d_n10;
        locals.var_nq_edge_dn11 = assign32760_e43432_d_n11;
        locals.var_nq_edge_dn12 = assign32760_e43432_d_n12;
        locals.var_nq_edge_dn13 = assign32760_e43432_d_n13;
        locals.var_nq_edge_dn14 = assign32760_e43432_d_n14;
        locals.var_nq_edge_rv = 0.0;

        let (assign32770_e43464, assign32770_e43464_d_n0, assign32770_e43464_d_n2, assign32770_e43464_d_n3, assign32770_e43464_d_n4, assign32770_e43464_d_n5, assign32770_e43464_d_n6, assign32770_e43464_d_n7, assign32770_e43464_d_n8, assign32770_e43464_d_n9, assign32770_e43464_d_n10, assign32770_e43464_d_n11, assign32770_e43464_d_n12, assign32770_e43464_d_n13, assign32770_e43464_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32770_e43436: f64 = (2.0 * p.p2);
        let assign32770_e43438: f64 = (assign32770_e43436 * locals.var_nq_edge);
        let assign32770_e43440: f64 = (assign32770_e43438 * locals.var_ueff);
        let assign32770_e43442: f64 = (assign32770_e43440 * p.p957);
        let assign32770_e43444: f64 = (assign32770_e43442 / locals.var_leff);
        let assign32770_e43446: f64 = (assign32770_e43444 * locals.var_cox);
        let assign32770_e43448: f64 = (assign32770_e43446 * locals.var_nvt);
        let assign32770_e43450: f64 = (assign32770_e43448 * locals.var_nvt);
        let assign32770_e43453: f64 = (locals.var_qs_edge - locals.var_qdeff_edge);
        let assign32770_e43456: f64 = (1.0 + locals.var_qs_edge);
        let assign32770_e43458: f64 = (assign32770_e43456 + locals.var_qdeff_edge);
        let assign32770_e43459: f64 = (assign32770_e43453 * assign32770_e43458);
        let assign32770_e43460: f64 = (assign32770_e43450 * assign32770_e43459);
        let assign32770_e43462: f64 = (assign32770_e43460 * locals.var_moc);
        (assign32770_e43462, ((((((((((((((assign32770_e43436 * locals.var_nq_edge_dn0) * locals.var_ueff) + (assign32770_e43438 * locals.var_ueff_dn0)) * p.p957) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign32770_e43446 * locals.var_nvt_dn0)) * locals.var_nvt) + (assign32770_e43448 * locals.var_nvt_dn0)) * assign32770_e43459) + (assign32770_e43450 * (((locals.var_qs_edge_dn0 - locals.var_qdeff_edge_dn0) * assign32770_e43458) + (assign32770_e43453 * (locals.var_qs_edge_dn0 + locals.var_qdeff_edge_dn0))))) * locals.var_moc) + (assign32770_e43460 * locals.var_moc_dn0)), ((((((((((((((assign32770_e43436 * locals.var_nq_edge_dn2) * locals.var_ueff) + (assign32770_e43438 * locals.var_ueff_dn2)) * p.p957) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign32770_e43446 * locals.var_nvt_dn2)) * locals.var_nvt) + (assign32770_e43448 * locals.var_nvt_dn2)) * assign32770_e43459) + (assign32770_e43450 * (((locals.var_qs_edge_dn2 - locals.var_qdeff_edge_dn2) * assign32770_e43458) + (assign32770_e43453 * (locals.var_qs_edge_dn2 + locals.var_qdeff_edge_dn2))))) * locals.var_moc) + (assign32770_e43460 * locals.var_moc_dn2)), ((((((((((((((assign32770_e43436 * locals.var_nq_edge_dn3) * locals.var_ueff) + (assign32770_e43438 * locals.var_ueff_dn3)) * p.p957) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign32770_e43446 * locals.var_nvt_dn3)) * locals.var_nvt) + (assign32770_e43448 * locals.var_nvt_dn3)) * assign32770_e43459) + (assign32770_e43450 * (((locals.var_qs_edge_dn3 - locals.var_qdeff_edge_dn3) * assign32770_e43458) + (assign32770_e43453 * (locals.var_qs_edge_dn3 + locals.var_qdeff_edge_dn3))))) * locals.var_moc) + (assign32770_e43460 * locals.var_moc_dn3)), ((((((((((((((assign32770_e43436 * locals.var_nq_edge_dn4) * locals.var_ueff) + (assign32770_e43438 * locals.var_ueff_dn4)) * p.p957) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign32770_e43446 * locals.var_nvt_dn4)) * locals.var_nvt) + (assign32770_e43448 * locals.var_nvt_dn4)) * assign32770_e43459) + (assign32770_e43450 * (((locals.var_qs_edge_dn4 - locals.var_qdeff_edge_dn4) * assign32770_e43458) + (assign32770_e43453 * (locals.var_qs_edge_dn4 + locals.var_qdeff_edge_dn4))))) * locals.var_moc) + (assign32770_e43460 * locals.var_moc_dn4)), ((((((((((((((assign32770_e43436 * locals.var_nq_edge_dn5) * locals.var_ueff) + (assign32770_e43438 * locals.var_ueff_dn5)) * p.p957) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign32770_e43446 * locals.var_nvt_dn5)) * locals.var_nvt) + (assign32770_e43448 * locals.var_nvt_dn5)) * assign32770_e43459) + (assign32770_e43450 * (((locals.var_qs_edge_dn5 - locals.var_qdeff_edge_dn5) * assign32770_e43458) + (assign32770_e43453 * (locals.var_qs_edge_dn5 + locals.var_qdeff_edge_dn5))))) * locals.var_moc) + (assign32770_e43460 * locals.var_moc_dn5)), ((((((((((((((assign32770_e43436 * locals.var_nq_edge_dn6) * locals.var_ueff) + (assign32770_e43438 * locals.var_ueff_dn6)) * p.p957) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign32770_e43446 * locals.var_nvt_dn6)) * locals.var_nvt) + (assign32770_e43448 * locals.var_nvt_dn6)) * assign32770_e43459) + (assign32770_e43450 * (((locals.var_qs_edge_dn6 - locals.var_qdeff_edge_dn6) * assign32770_e43458) + (assign32770_e43453 * (locals.var_qs_edge_dn6 + locals.var_qdeff_edge_dn6))))) * locals.var_moc) + (assign32770_e43460 * locals.var_moc_dn6)), ((((((((((((((assign32770_e43436 * locals.var_nq_edge_dn7) * locals.var_ueff) + (assign32770_e43438 * locals.var_ueff_dn7)) * p.p957) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign32770_e43446 * locals.var_nvt_dn7)) * locals.var_nvt) + (assign32770_e43448 * locals.var_nvt_dn7)) * assign32770_e43459) + (assign32770_e43450 * (((locals.var_qs_edge_dn7 - locals.var_qdeff_edge_dn7) * assign32770_e43458) + (assign32770_e43453 * (locals.var_qs_edge_dn7 + locals.var_qdeff_edge_dn7))))) * locals.var_moc) + (assign32770_e43460 * locals.var_moc_dn7)), ((((((((((((((assign32770_e43436 * locals.var_nq_edge_dn8) * locals.var_ueff) + (assign32770_e43438 * locals.var_ueff_dn8)) * p.p957) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign32770_e43446 * locals.var_nvt_dn8)) * locals.var_nvt) + (assign32770_e43448 * locals.var_nvt_dn8)) * assign32770_e43459) + (assign32770_e43450 * (((locals.var_qs_edge_dn8 - locals.var_qdeff_edge_dn8) * assign32770_e43458) + (assign32770_e43453 * (locals.var_qs_edge_dn8 + locals.var_qdeff_edge_dn8))))) * locals.var_moc) + (assign32770_e43460 * locals.var_moc_dn8)), ((((((((((((((assign32770_e43436 * locals.var_nq_edge_dn9) * locals.var_ueff) + (assign32770_e43438 * locals.var_ueff_dn9)) * p.p957) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign32770_e43446 * locals.var_nvt_dn9)) * locals.var_nvt) + (assign32770_e43448 * locals.var_nvt_dn9)) * assign32770_e43459) + (assign32770_e43450 * (((locals.var_qs_edge_dn9 - locals.var_qdeff_edge_dn9) * assign32770_e43458) + (assign32770_e43453 * (locals.var_qs_edge_dn9 + locals.var_qdeff_edge_dn9))))) * locals.var_moc) + (assign32770_e43460 * locals.var_moc_dn9)), ((((((((((((((assign32770_e43436 * locals.var_nq_edge_dn10) * locals.var_ueff) + (assign32770_e43438 * locals.var_ueff_dn10)) * p.p957) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign32770_e43446 * locals.var_nvt_dn10)) * locals.var_nvt) + (assign32770_e43448 * locals.var_nvt_dn10)) * assign32770_e43459) + (assign32770_e43450 * (((locals.var_qs_edge_dn10 - locals.var_qdeff_edge_dn10) * assign32770_e43458) + (assign32770_e43453 * (locals.var_qs_edge_dn10 + locals.var_qdeff_edge_dn10))))) * locals.var_moc) + (assign32770_e43460 * locals.var_moc_dn10)), ((((((((((((((assign32770_e43436 * locals.var_nq_edge_dn11) * locals.var_ueff) + (assign32770_e43438 * locals.var_ueff_dn11)) * p.p957) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign32770_e43446 * locals.var_nvt_dn11)) * locals.var_nvt) + (assign32770_e43448 * locals.var_nvt_dn11)) * assign32770_e43459) + (assign32770_e43450 * (((locals.var_qs_edge_dn11 - locals.var_qdeff_edge_dn11) * assign32770_e43458) + (assign32770_e43453 * (locals.var_qs_edge_dn11 + locals.var_qdeff_edge_dn11))))) * locals.var_moc) + (assign32770_e43460 * locals.var_moc_dn11)), ((((((((((((((assign32770_e43436 * locals.var_nq_edge_dn12) * locals.var_ueff) + (assign32770_e43438 * locals.var_ueff_dn12)) * p.p957) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign32770_e43446 * locals.var_nvt_dn12)) * locals.var_nvt) + (assign32770_e43448 * locals.var_nvt_dn12)) * assign32770_e43459) + (assign32770_e43450 * (((locals.var_qs_edge_dn12 - locals.var_qdeff_edge_dn12) * assign32770_e43458) + (assign32770_e43453 * (locals.var_qs_edge_dn12 + locals.var_qdeff_edge_dn12))))) * locals.var_moc) + (assign32770_e43460 * locals.var_moc_dn12)), ((((((((((((((assign32770_e43436 * locals.var_nq_edge_dn13) * locals.var_ueff) + (assign32770_e43438 * locals.var_ueff_dn13)) * p.p957) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign32770_e43446 * locals.var_nvt_dn13)) * locals.var_nvt) + (assign32770_e43448 * locals.var_nvt_dn13)) * assign32770_e43459) + (assign32770_e43450 * (((locals.var_qs_edge_dn13 - locals.var_qdeff_edge_dn13) * assign32770_e43458) + (assign32770_e43453 * (locals.var_qs_edge_dn13 + locals.var_qdeff_edge_dn13))))) * locals.var_moc) + (assign32770_e43460 * locals.var_moc_dn13)), ((((((((((((((assign32770_e43436 * locals.var_nq_edge_dn14) * locals.var_ueff) + (assign32770_e43438 * locals.var_ueff_dn14)) * p.p957) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign32770_e43446 * locals.var_nvt_dn14)) * locals.var_nvt) + (assign32770_e43448 * locals.var_nvt_dn14)) * assign32770_e43459) + (assign32770_e43450 * (((locals.var_qs_edge_dn14 - locals.var_qdeff_edge_dn14) * assign32770_e43458) + (assign32770_e43453 * (locals.var_qs_edge_dn14 + locals.var_qdeff_edge_dn14))))) * locals.var_moc) + (assign32770_e43460 * locals.var_moc_dn14)),)
    } else {
        (locals.var_ids_edge, locals.var_ids_edge_dn0, locals.var_ids_edge_dn2, locals.var_ids_edge_dn3, locals.var_ids_edge_dn4, locals.var_ids_edge_dn5, locals.var_ids_edge_dn6, locals.var_ids_edge_dn7, locals.var_ids_edge_dn8, locals.var_ids_edge_dn9, locals.var_ids_edge_dn10, locals.var_ids_edge_dn11, locals.var_ids_edge_dn12, locals.var_ids_edge_dn13, locals.var_ids_edge_dn14,)
    }
};
        locals.var_ids_edge = assign32770_e43464;
        locals.var_ids_edge_dn0 = assign32770_e43464_d_n0;
        locals.var_ids_edge_dn2 = assign32770_e43464_d_n2;
        locals.var_ids_edge_dn3 = assign32770_e43464_d_n3;
        locals.var_ids_edge_dn4 = assign32770_e43464_d_n4;
        locals.var_ids_edge_dn5 = assign32770_e43464_d_n5;
        locals.var_ids_edge_dn6 = assign32770_e43464_d_n6;
        locals.var_ids_edge_dn7 = assign32770_e43464_d_n7;
        locals.var_ids_edge_dn8 = assign32770_e43464_d_n8;
        locals.var_ids_edge_dn9 = assign32770_e43464_d_n9;
        locals.var_ids_edge_dn10 = assign32770_e43464_d_n10;
        locals.var_ids_edge_dn11 = assign32770_e43464_d_n11;
        locals.var_ids_edge_dn12 = assign32770_e43464_d_n12;
        locals.var_ids_edge_dn13 = assign32770_e43464_d_n13;
        locals.var_ids_edge_dn14 = assign32770_e43464_d_n14;
        locals.var_ids_edge_rv = 0.0;

        let (assign32780_e43470, assign32780_e43470_d_n0, assign32780_e43470_d_n2, assign32780_e43470_d_n3, assign32780_e43470_d_n4, assign32780_e43470_d_n5, assign32780_e43470_d_n6, assign32780_e43470_d_n7, assign32780_e43470_d_n8, assign32780_e43470_d_n9, assign32780_e43470_d_n10, assign32780_e43470_d_n11, assign32780_e43470_d_n12, assign32780_e43470_d_n13, assign32780_e43470_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32780_e43468: f64 = (locals.var_ids_edge + locals.var_ids);
        (assign32780_e43468, (locals.var_ids_edge_dn0 + locals.var_ids_dn0), (locals.var_ids_edge_dn2 + locals.var_ids_dn2), (locals.var_ids_edge_dn3 + locals.var_ids_dn3), (locals.var_ids_edge_dn4 + locals.var_ids_dn4), (locals.var_ids_edge_dn5 + locals.var_ids_dn5), (locals.var_ids_edge_dn6 + locals.var_ids_dn6), (locals.var_ids_edge_dn7 + locals.var_ids_dn7), (locals.var_ids_edge_dn8 + locals.var_ids_dn8), (locals.var_ids_edge_dn9 + locals.var_ids_dn9), (locals.var_ids_edge_dn10 + locals.var_ids_dn10), (locals.var_ids_edge_dn11 + locals.var_ids_dn11), (locals.var_ids_edge_dn12 + locals.var_ids_dn12), (locals.var_ids_edge_dn13 + locals.var_ids_dn13), (locals.var_ids_edge_dn14 + locals.var_ids_dn14),)
    } else {
        (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn3, locals.var_ids_dn4, locals.var_ids_dn5, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn8, locals.var_ids_dn9, locals.var_ids_dn10, locals.var_ids_dn11, locals.var_ids_dn12, locals.var_ids_dn13, locals.var_ids_dn14,)
    }
};
        locals.var_ids = assign32780_e43470;
        locals.var_ids_dn0 = assign32780_e43470_d_n0;
        locals.var_ids_dn2 = assign32780_e43470_d_n2;
        locals.var_ids_dn3 = assign32780_e43470_d_n3;
        locals.var_ids_dn4 = assign32780_e43470_d_n4;
        locals.var_ids_dn5 = assign32780_e43470_d_n5;
        locals.var_ids_dn6 = assign32780_e43470_d_n6;
        locals.var_ids_dn7 = assign32780_e43470_d_n7;
        locals.var_ids_dn8 = assign32780_e43470_d_n8;
        locals.var_ids_dn9 = assign32780_e43470_d_n9;
        locals.var_ids_dn10 = assign32780_e43470_d_n10;
        locals.var_ids_dn11 = assign32780_e43470_d_n11;
        locals.var_ids_dn12 = assign32780_e43470_d_n12;
        locals.var_ids_dn13 = assign32780_e43470_d_n13;
        locals.var_ids_dn14 = assign32780_e43470_d_n14;
        locals.var_ids_rv = 0.0;

        let (assign32790_e43476,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32790_e43474: f64 = (p.p785 * p.p1062);
        (assign32790_e43474,)
    } else {
        (locals.var_noia_edge,)
    }
};
        locals.var_noia_edge = assign32790_e43476;
        locals.var_noia_edge_rv = 0.0;

        let (assign32800_e43482,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32800_e43480: f64 = (p.p799 * p.p1062);
        (assign32800_e43480,)
    } else {
        (locals.var_noib_edge,)
    }
};
        locals.var_noib_edge = assign32800_e43482;
        locals.var_noib_edge_rv = 0.0;

        let (assign32810_e43488,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32810_e43486: f64 = (p.p800 * p.p1062);
        (assign32810_e43486,)
    } else {
        (locals.var_noic_edge,)
    }
};
        locals.var_noic_edge = assign32810_e43488;
        locals.var_noic_edge_rv = 0.0;

        let (assign32820_e43496,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32820_e43493: f64 = (2.0 * locals.var_lintnoi_i);
        let assign32820_e43494: f64 = (locals.var_leff - assign32820_e43493);
        (assign32820_e43494,)
    } else {
        (locals.var_leffnoi_edge,)
    }
};
        locals.var_leffnoi_edge = assign32820_e43496;
        locals.var_leffnoi_edge_rv = 0.0;

        let (assign32830_e43502,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32830_e43500: f64 = (locals.var_leffnoi_edge * locals.var_leffnoi_edge);
        (assign32830_e43500,)
    } else {
        (locals.var_leffnoisq_edge,)
    }
};
        locals.var_leffnoisq_edge = assign32830_e43502;
        locals.var_leffnoisq_edge_rv = 0.0;

        let (assign32840_e43514, assign32840_e43514_d_n0, assign32840_e43514_d_n2, assign32840_e43514_d_n3, assign32840_e43514_d_n4, assign32840_e43514_d_n5, assign32840_e43514_d_n6, assign32840_e43514_d_n7, assign32840_e43514_d_n8, assign32840_e43514_d_n9, assign32840_e43514_d_n10, assign32840_e43514_d_n11, assign32840_e43514_d_n12, assign32840_e43514_d_n13, assign32840_e43514_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32840_e43506: f64 = (locals.var_vt / 1.60219e-19);
        let assign32840_e43509: f64 = (locals.var_cox + locals.var_cdep);
        let assign32840_e43511: f64 = (assign32840_e43509 + locals.var_citedge_i);
        let assign32840_e43512: f64 = (assign32840_e43506 * assign32840_e43511);
        (assign32840_e43512, (assign32840_e43506 * locals.var_cdep_dn0), (assign32840_e43506 * locals.var_cdep_dn2), (assign32840_e43506 * locals.var_cdep_dn3), (((locals.var_vt_dn4 / 1.60219e-19) * assign32840_e43511) + (assign32840_e43506 * locals.var_cdep_dn4)), (assign32840_e43506 * locals.var_cdep_dn5), (assign32840_e43506 * locals.var_cdep_dn6), (assign32840_e43506 * locals.var_cdep_dn7), (assign32840_e43506 * locals.var_cdep_dn8), (assign32840_e43506 * locals.var_cdep_dn9), (assign32840_e43506 * locals.var_cdep_dn10), (assign32840_e43506 * locals.var_cdep_dn11), (assign32840_e43506 * locals.var_cdep_dn12), (assign32840_e43506 * locals.var_cdep_dn13), (assign32840_e43506 * locals.var_cdep_dn14),)
    } else {
        (locals.var_nstar, locals.var_nstar_dn0, locals.var_nstar_dn2, locals.var_nstar_dn3, locals.var_nstar_dn4, locals.var_nstar_dn5, locals.var_nstar_dn6, locals.var_nstar_dn7, locals.var_nstar_dn8, locals.var_nstar_dn9, locals.var_nstar_dn10, locals.var_nstar_dn11, locals.var_nstar_dn12, locals.var_nstar_dn13, locals.var_nstar_dn14,)
    }
};
        locals.var_nstar = assign32840_e43514;
        locals.var_nstar_dn0 = assign32840_e43514_d_n0;
        locals.var_nstar_dn2 = assign32840_e43514_d_n2;
        locals.var_nstar_dn3 = assign32840_e43514_d_n3;
        locals.var_nstar_dn4 = assign32840_e43514_d_n4;
        locals.var_nstar_dn5 = assign32840_e43514_d_n5;
        locals.var_nstar_dn6 = assign32840_e43514_d_n6;
        locals.var_nstar_dn7 = assign32840_e43514_d_n7;
        locals.var_nstar_dn8 = assign32840_e43514_d_n8;
        locals.var_nstar_dn9 = assign32840_e43514_d_n9;
        locals.var_nstar_dn10 = assign32840_e43514_d_n10;
        locals.var_nstar_dn11 = assign32840_e43514_d_n11;
        locals.var_nstar_dn12 = assign32840_e43514_d_n12;
        locals.var_nstar_dn13 = assign32840_e43514_d_n13;
        locals.var_nstar_dn14 = assign32840_e43514_d_n14;
        locals.var_nstar_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_107(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign32850_e43528, assign32850_e43528_d_n0, assign32850_e43528_d_n2, assign32850_e43528_d_n3, assign32850_e43528_d_n4, assign32850_e43528_d_n5, assign32850_e43528_d_n6, assign32850_e43528_d_n7, assign32850_e43528_d_n8, assign32850_e43528_d_n9, assign32850_e43528_d_n10, assign32850_e43528_d_n11, assign32850_e43528_d_n12, assign32850_e43528_d_n13, assign32850_e43528_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32850_e43518: f64 = (2.0 * locals.var_nq_edge);
        let assign32850_e43520: f64 = (assign32850_e43518 * locals.var_cox);
        let assign32850_e43522: f64 = (assign32850_e43520 * locals.var_vt);
        let assign32850_e43524: f64 = (assign32850_e43522 * locals.var_qdeff_edge);
        let assign32850_e43526: f64 = (assign32850_e43524 / 1.60219e-19);
        (assign32850_e43526, ((((((2.0 * locals.var_nq_edge_dn0) * locals.var_cox) * locals.var_vt) * locals.var_qdeff_edge) + (assign32850_e43522 * locals.var_qdeff_edge_dn0)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn2) * locals.var_cox) * locals.var_vt) * locals.var_qdeff_edge) + (assign32850_e43522 * locals.var_qdeff_edge_dn2)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn3) * locals.var_cox) * locals.var_vt) * locals.var_qdeff_edge) + (assign32850_e43522 * locals.var_qdeff_edge_dn3)) / 1.60219e-19), (((((((2.0 * locals.var_nq_edge_dn4) * locals.var_cox) * locals.var_vt) + (assign32850_e43520 * locals.var_vt_dn4)) * locals.var_qdeff_edge) + (assign32850_e43522 * locals.var_qdeff_edge_dn4)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn5) * locals.var_cox) * locals.var_vt) * locals.var_qdeff_edge) + (assign32850_e43522 * locals.var_qdeff_edge_dn5)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn6) * locals.var_cox) * locals.var_vt) * locals.var_qdeff_edge) + (assign32850_e43522 * locals.var_qdeff_edge_dn6)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn7) * locals.var_cox) * locals.var_vt) * locals.var_qdeff_edge) + (assign32850_e43522 * locals.var_qdeff_edge_dn7)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn8) * locals.var_cox) * locals.var_vt) * locals.var_qdeff_edge) + (assign32850_e43522 * locals.var_qdeff_edge_dn8)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn9) * locals.var_cox) * locals.var_vt) * locals.var_qdeff_edge) + (assign32850_e43522 * locals.var_qdeff_edge_dn9)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn10) * locals.var_cox) * locals.var_vt) * locals.var_qdeff_edge) + (assign32850_e43522 * locals.var_qdeff_edge_dn10)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn11) * locals.var_cox) * locals.var_vt) * locals.var_qdeff_edge) + (assign32850_e43522 * locals.var_qdeff_edge_dn11)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn12) * locals.var_cox) * locals.var_vt) * locals.var_qdeff_edge) + (assign32850_e43522 * locals.var_qdeff_edge_dn12)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn13) * locals.var_cox) * locals.var_vt) * locals.var_qdeff_edge) + (assign32850_e43522 * locals.var_qdeff_edge_dn13)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn14) * locals.var_cox) * locals.var_vt) * locals.var_qdeff_edge) + (assign32850_e43522 * locals.var_qdeff_edge_dn14)) / 1.60219e-19),)
    } else {
        (locals.var_nl, locals.var_nl_dn0, locals.var_nl_dn2, locals.var_nl_dn3, locals.var_nl_dn4, locals.var_nl_dn5, locals.var_nl_dn6, locals.var_nl_dn7, locals.var_nl_dn8, locals.var_nl_dn9, locals.var_nl_dn10, locals.var_nl_dn11, locals.var_nl_dn12, locals.var_nl_dn13, locals.var_nl_dn14,)
    }
};
        locals.var_nl = assign32850_e43528;
        locals.var_nl_dn0 = assign32850_e43528_d_n0;
        locals.var_nl_dn2 = assign32850_e43528_d_n2;
        locals.var_nl_dn3 = assign32850_e43528_d_n3;
        locals.var_nl_dn4 = assign32850_e43528_d_n4;
        locals.var_nl_dn5 = assign32850_e43528_d_n5;
        locals.var_nl_dn6 = assign32850_e43528_d_n6;
        locals.var_nl_dn7 = assign32850_e43528_d_n7;
        locals.var_nl_dn8 = assign32850_e43528_d_n8;
        locals.var_nl_dn9 = assign32850_e43528_d_n9;
        locals.var_nl_dn10 = assign32850_e43528_d_n10;
        locals.var_nl_dn11 = assign32850_e43528_d_n11;
        locals.var_nl_dn12 = assign32850_e43528_d_n12;
        locals.var_nl_dn13 = assign32850_e43528_d_n13;
        locals.var_nl_dn14 = assign32850_e43528_d_n14;
        locals.var_nl_rv = 0.0;

        let (assign32860_e43543, assign32860_e43543_d_n0, assign32860_e43543_d_n2, assign32860_e43543_d_n3, assign32860_e43543_d_n4, assign32860_e43543_d_n5, assign32860_e43543_d_n6, assign32860_e43543_d_n7, assign32860_e43543_d_n8, assign32860_e43543_d_n9, assign32860_e43543_d_n10, assign32860_e43543_d_n11, assign32860_e43543_d_n12, assign32860_e43543_d_n13, assign32860_e43543_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32860_e43532: f64 = (1.60219e-19 * 1.60219e-19);
        let assign32860_e43534: f64 = (assign32860_e43532 * 1.60219e-19);
        let assign32860_e43536: f64 = (assign32860_e43534 * locals.var_vt);
        let assign32860_e43538: f64 = (locals.var_ids_edge).abs();
        let assign32860_e43539: f64 = (assign32860_e43536 * assign32860_e43538);
        let assign32860_e43541: f64 = (assign32860_e43539 * locals.var_ueff);
        (assign32860_e43541, (((assign32860_e43536 * if locals.var_ids_edge >= 0.0 { locals.var_ids_edge_dn0 } else { (-locals.var_ids_edge_dn0) }) * locals.var_ueff) + (assign32860_e43539 * locals.var_ueff_dn0)), (((assign32860_e43536 * if locals.var_ids_edge >= 0.0 { locals.var_ids_edge_dn2 } else { (-locals.var_ids_edge_dn2) }) * locals.var_ueff) + (assign32860_e43539 * locals.var_ueff_dn2)), (((assign32860_e43536 * if locals.var_ids_edge >= 0.0 { locals.var_ids_edge_dn3 } else { (-locals.var_ids_edge_dn3) }) * locals.var_ueff) + (assign32860_e43539 * locals.var_ueff_dn3)), (((((assign32860_e43534 * locals.var_vt_dn4) * assign32860_e43538) + (assign32860_e43536 * if locals.var_ids_edge >= 0.0 { locals.var_ids_edge_dn4 } else { (-locals.var_ids_edge_dn4) })) * locals.var_ueff) + (assign32860_e43539 * locals.var_ueff_dn4)), (((assign32860_e43536 * if locals.var_ids_edge >= 0.0 { locals.var_ids_edge_dn5 } else { (-locals.var_ids_edge_dn5) }) * locals.var_ueff) + (assign32860_e43539 * locals.var_ueff_dn5)), (((assign32860_e43536 * if locals.var_ids_edge >= 0.0 { locals.var_ids_edge_dn6 } else { (-locals.var_ids_edge_dn6) }) * locals.var_ueff) + (assign32860_e43539 * locals.var_ueff_dn6)), (((assign32860_e43536 * if locals.var_ids_edge >= 0.0 { locals.var_ids_edge_dn7 } else { (-locals.var_ids_edge_dn7) }) * locals.var_ueff) + (assign32860_e43539 * locals.var_ueff_dn7)), (((assign32860_e43536 * if locals.var_ids_edge >= 0.0 { locals.var_ids_edge_dn8 } else { (-locals.var_ids_edge_dn8) }) * locals.var_ueff) + (assign32860_e43539 * locals.var_ueff_dn8)), (((assign32860_e43536 * if locals.var_ids_edge >= 0.0 { locals.var_ids_edge_dn9 } else { (-locals.var_ids_edge_dn9) }) * locals.var_ueff) + (assign32860_e43539 * locals.var_ueff_dn9)), (((assign32860_e43536 * if locals.var_ids_edge >= 0.0 { locals.var_ids_edge_dn10 } else { (-locals.var_ids_edge_dn10) }) * locals.var_ueff) + (assign32860_e43539 * locals.var_ueff_dn10)), (((assign32860_e43536 * if locals.var_ids_edge >= 0.0 { locals.var_ids_edge_dn11 } else { (-locals.var_ids_edge_dn11) }) * locals.var_ueff) + (assign32860_e43539 * locals.var_ueff_dn11)), (((assign32860_e43536 * if locals.var_ids_edge >= 0.0 { locals.var_ids_edge_dn12 } else { (-locals.var_ids_edge_dn12) }) * locals.var_ueff) + (assign32860_e43539 * locals.var_ueff_dn12)), (((assign32860_e43536 * if locals.var_ids_edge >= 0.0 { locals.var_ids_edge_dn13 } else { (-locals.var_ids_edge_dn13) }) * locals.var_ueff) + (assign32860_e43539 * locals.var_ueff_dn13)), (((assign32860_e43536 * if locals.var_ids_edge >= 0.0 { locals.var_ids_edge_dn14 } else { (-locals.var_ids_edge_dn14) }) * locals.var_ueff) + (assign32860_e43539 * locals.var_ueff_dn14)),)
    } else {
        (locals.var_t0a, locals.var_t0a_dn0, locals.var_t0a_dn2, locals.var_t0a_dn3, locals.var_t0a_dn4, locals.var_t0a_dn5, locals.var_t0a_dn6, locals.var_t0a_dn7, locals.var_t0a_dn8, locals.var_t0a_dn9, locals.var_t0a_dn10, locals.var_t0a_dn11, locals.var_t0a_dn12, locals.var_t0a_dn13, locals.var_t0a_dn14,)
    }
};
        locals.var_t0a = assign32860_e43543;
        locals.var_t0a_dn0 = assign32860_e43543_d_n0;
        locals.var_t0a_dn2 = assign32860_e43543_d_n2;
        locals.var_t0a_dn3 = assign32860_e43543_d_n3;
        locals.var_t0a_dn4 = assign32860_e43543_d_n4;
        locals.var_t0a_dn5 = assign32860_e43543_d_n5;
        locals.var_t0a_dn6 = assign32860_e43543_d_n6;
        locals.var_t0a_dn7 = assign32860_e43543_d_n7;
        locals.var_t0a_dn8 = assign32860_e43543_d_n8;
        locals.var_t0a_dn9 = assign32860_e43543_d_n9;
        locals.var_t0a_dn10 = assign32860_e43543_d_n10;
        locals.var_t0a_dn11 = assign32860_e43543_d_n11;
        locals.var_t0a_dn12 = assign32860_e43543_d_n12;
        locals.var_t0a_dn13 = assign32860_e43543_d_n13;
        locals.var_t0a_dn14 = assign32860_e43543_d_n14;
        locals.var_t0a_rv = 0.0;

        let (assign32870_e43553, assign32870_e43553_d_n0, assign32870_e43553_d_n2, assign32870_e43553_d_n3, assign32870_e43553_d_n4, assign32870_e43553_d_n5, assign32870_e43553_d_n6, assign32870_e43553_d_n7, assign32870_e43553_d_n8, assign32870_e43553_d_n9, assign32870_e43553_d_n10, assign32870_e43553_d_n11, assign32870_e43553_d_n12, assign32870_e43553_d_n13, assign32870_e43553_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32870_e43547: f64 = (1.60219e-19 * locals.var_vt);
        let assign32870_e43549: f64 = (assign32870_e43547 * locals.var_ids_edge);
        let assign32870_e43551: f64 = (assign32870_e43549 * locals.var_ids_edge);
        (assign32870_e43551, (((assign32870_e43547 * locals.var_ids_edge_dn0) * locals.var_ids_edge) + (assign32870_e43549 * locals.var_ids_edge_dn0)), (((assign32870_e43547 * locals.var_ids_edge_dn2) * locals.var_ids_edge) + (assign32870_e43549 * locals.var_ids_edge_dn2)), (((assign32870_e43547 * locals.var_ids_edge_dn3) * locals.var_ids_edge) + (assign32870_e43549 * locals.var_ids_edge_dn3)), (((((1.60219e-19 * locals.var_vt_dn4) * locals.var_ids_edge) + (assign32870_e43547 * locals.var_ids_edge_dn4)) * locals.var_ids_edge) + (assign32870_e43549 * locals.var_ids_edge_dn4)), (((assign32870_e43547 * locals.var_ids_edge_dn5) * locals.var_ids_edge) + (assign32870_e43549 * locals.var_ids_edge_dn5)), (((assign32870_e43547 * locals.var_ids_edge_dn6) * locals.var_ids_edge) + (assign32870_e43549 * locals.var_ids_edge_dn6)), (((assign32870_e43547 * locals.var_ids_edge_dn7) * locals.var_ids_edge) + (assign32870_e43549 * locals.var_ids_edge_dn7)), (((assign32870_e43547 * locals.var_ids_edge_dn8) * locals.var_ids_edge) + (assign32870_e43549 * locals.var_ids_edge_dn8)), (((assign32870_e43547 * locals.var_ids_edge_dn9) * locals.var_ids_edge) + (assign32870_e43549 * locals.var_ids_edge_dn9)), (((assign32870_e43547 * locals.var_ids_edge_dn10) * locals.var_ids_edge) + (assign32870_e43549 * locals.var_ids_edge_dn10)), (((assign32870_e43547 * locals.var_ids_edge_dn11) * locals.var_ids_edge) + (assign32870_e43549 * locals.var_ids_edge_dn11)), (((assign32870_e43547 * locals.var_ids_edge_dn12) * locals.var_ids_edge) + (assign32870_e43549 * locals.var_ids_edge_dn12)), (((assign32870_e43547 * locals.var_ids_edge_dn13) * locals.var_ids_edge) + (assign32870_e43549 * locals.var_ids_edge_dn13)), (((assign32870_e43547 * locals.var_ids_edge_dn14) * locals.var_ids_edge) + (assign32870_e43549 * locals.var_ids_edge_dn14)),)
    } else {
        (locals.var_t0b, locals.var_t0b_dn0, locals.var_t0b_dn2, locals.var_t0b_dn3, locals.var_t0b_dn4, locals.var_t0b_dn5, locals.var_t0b_dn6, locals.var_t0b_dn7, locals.var_t0b_dn8, locals.var_t0b_dn9, locals.var_t0b_dn10, locals.var_t0b_dn11, locals.var_t0b_dn12, locals.var_t0b_dn13, locals.var_t0b_dn14,)
    }
};
        locals.var_t0b = assign32870_e43553;
        locals.var_t0b_dn0 = assign32870_e43553_d_n0;
        locals.var_t0b_dn2 = assign32870_e43553_d_n2;
        locals.var_t0b_dn3 = assign32870_e43553_d_n3;
        locals.var_t0b_dn4 = assign32870_e43553_d_n4;
        locals.var_t0b_dn5 = assign32870_e43553_d_n5;
        locals.var_t0b_dn6 = assign32870_e43553_d_n6;
        locals.var_t0b_dn7 = assign32870_e43553_d_n7;
        locals.var_t0b_dn8 = assign32870_e43553_d_n8;
        locals.var_t0b_dn9 = assign32870_e43553_d_n9;
        locals.var_t0b_dn10 = assign32870_e43553_d_n10;
        locals.var_t0b_dn11 = assign32870_e43553_d_n11;
        locals.var_t0b_dn12 = assign32870_e43553_d_n12;
        locals.var_t0b_dn13 = assign32870_e43553_d_n13;
        locals.var_t0b_dn14 = assign32870_e43553_d_n14;
        locals.var_t0b_rv = 0.0;

        let (assign32880_e43567, assign32880_e43567_d_n0, assign32880_e43567_d_n2, assign32880_e43567_d_n3, assign32880_e43567_d_n4, assign32880_e43567_d_n5, assign32880_e43567_d_n6, assign32880_e43567_d_n7, assign32880_e43567_d_n8, assign32880_e43567_d_n9, assign32880_e43567_d_n10, assign32880_e43567_d_n11, assign32880_e43567_d_n12, assign32880_e43567_d_n13, assign32880_e43567_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32880_e43558: f64 = (locals.var_noib_edge * locals.var_nl);
        let assign32880_e43559: f64 = (locals.var_noia_edge + assign32880_e43558);
        let assign32880_e43562: f64 = (locals.var_noic_edge * locals.var_nl);
        let assign32880_e43564: f64 = (assign32880_e43562 * locals.var_nl);
        let assign32880_e43565: f64 = (assign32880_e43559 + assign32880_e43564);
        (assign32880_e43565, ((locals.var_noib_edge * locals.var_nl_dn0) + (((locals.var_noic_edge * locals.var_nl_dn0) * locals.var_nl) + (assign32880_e43562 * locals.var_nl_dn0))), ((locals.var_noib_edge * locals.var_nl_dn2) + (((locals.var_noic_edge * locals.var_nl_dn2) * locals.var_nl) + (assign32880_e43562 * locals.var_nl_dn2))), ((locals.var_noib_edge * locals.var_nl_dn3) + (((locals.var_noic_edge * locals.var_nl_dn3) * locals.var_nl) + (assign32880_e43562 * locals.var_nl_dn3))), ((locals.var_noib_edge * locals.var_nl_dn4) + (((locals.var_noic_edge * locals.var_nl_dn4) * locals.var_nl) + (assign32880_e43562 * locals.var_nl_dn4))), ((locals.var_noib_edge * locals.var_nl_dn5) + (((locals.var_noic_edge * locals.var_nl_dn5) * locals.var_nl) + (assign32880_e43562 * locals.var_nl_dn5))), ((locals.var_noib_edge * locals.var_nl_dn6) + (((locals.var_noic_edge * locals.var_nl_dn6) * locals.var_nl) + (assign32880_e43562 * locals.var_nl_dn6))), ((locals.var_noib_edge * locals.var_nl_dn7) + (((locals.var_noic_edge * locals.var_nl_dn7) * locals.var_nl) + (assign32880_e43562 * locals.var_nl_dn7))), ((locals.var_noib_edge * locals.var_nl_dn8) + (((locals.var_noic_edge * locals.var_nl_dn8) * locals.var_nl) + (assign32880_e43562 * locals.var_nl_dn8))), ((locals.var_noib_edge * locals.var_nl_dn9) + (((locals.var_noic_edge * locals.var_nl_dn9) * locals.var_nl) + (assign32880_e43562 * locals.var_nl_dn9))), ((locals.var_noib_edge * locals.var_nl_dn10) + (((locals.var_noic_edge * locals.var_nl_dn10) * locals.var_nl) + (assign32880_e43562 * locals.var_nl_dn10))), ((locals.var_noib_edge * locals.var_nl_dn11) + (((locals.var_noic_edge * locals.var_nl_dn11) * locals.var_nl) + (assign32880_e43562 * locals.var_nl_dn11))), ((locals.var_noib_edge * locals.var_nl_dn12) + (((locals.var_noic_edge * locals.var_nl_dn12) * locals.var_nl) + (assign32880_e43562 * locals.var_nl_dn12))), ((locals.var_noib_edge * locals.var_nl_dn13) + (((locals.var_noic_edge * locals.var_nl_dn13) * locals.var_nl) + (assign32880_e43562 * locals.var_nl_dn13))), ((locals.var_noib_edge * locals.var_nl_dn14) + (((locals.var_noic_edge * locals.var_nl_dn14) * locals.var_nl) + (assign32880_e43562 * locals.var_nl_dn14))),)
    } else {
        (locals.var_t0c, locals.var_t0c_dn0, locals.var_t0c_dn2, locals.var_t0c_dn3, locals.var_t0c_dn4, locals.var_t0c_dn5, locals.var_t0c_dn6, locals.var_t0c_dn7, locals.var_t0c_dn8, locals.var_t0c_dn9, locals.var_t0c_dn10, locals.var_t0c_dn11, locals.var_t0c_dn12, locals.var_t0c_dn13, locals.var_t0c_dn14,)
    }
};
        locals.var_t0c = assign32880_e43567;
        locals.var_t0c_dn0 = assign32880_e43567_d_n0;
        locals.var_t0c_dn2 = assign32880_e43567_d_n2;
        locals.var_t0c_dn3 = assign32880_e43567_d_n3;
        locals.var_t0c_dn4 = assign32880_e43567_d_n4;
        locals.var_t0c_dn5 = assign32880_e43567_d_n5;
        locals.var_t0c_dn6 = assign32880_e43567_d_n6;
        locals.var_t0c_dn7 = assign32880_e43567_d_n7;
        locals.var_t0c_dn8 = assign32880_e43567_d_n8;
        locals.var_t0c_dn9 = assign32880_e43567_d_n9;
        locals.var_t0c_dn10 = assign32880_e43567_d_n10;
        locals.var_t0c_dn11 = assign32880_e43567_d_n11;
        locals.var_t0c_dn12 = assign32880_e43567_d_n12;
        locals.var_t0c_dn13 = assign32880_e43567_d_n13;
        locals.var_t0c_dn14 = assign32880_e43567_d_n14;
        locals.var_t0c_rv = 0.0;

        let (assign32890_e43577, assign32890_e43577_d_n0, assign32890_e43577_d_n2, assign32890_e43577_d_n3, assign32890_e43577_d_n4, assign32890_e43577_d_n5, assign32890_e43577_d_n6, assign32890_e43577_d_n7, assign32890_e43577_d_n8, assign32890_e43577_d_n9, assign32890_e43577_d_n10, assign32890_e43577_d_n11, assign32890_e43577_d_n12, assign32890_e43577_d_n13, assign32890_e43577_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32890_e43571: f64 = (locals.var_nl + locals.var_nstar);
        let assign32890_e43574: f64 = (locals.var_nl + locals.var_nstar);
        let assign32890_e43575: f64 = (assign32890_e43571 * assign32890_e43574);
        (assign32890_e43575, (((locals.var_nl_dn0 + locals.var_nstar_dn0) * assign32890_e43574) + (assign32890_e43571 * (locals.var_nl_dn0 + locals.var_nstar_dn0))), (((locals.var_nl_dn2 + locals.var_nstar_dn2) * assign32890_e43574) + (assign32890_e43571 * (locals.var_nl_dn2 + locals.var_nstar_dn2))), (((locals.var_nl_dn3 + locals.var_nstar_dn3) * assign32890_e43574) + (assign32890_e43571 * (locals.var_nl_dn3 + locals.var_nstar_dn3))), (((locals.var_nl_dn4 + locals.var_nstar_dn4) * assign32890_e43574) + (assign32890_e43571 * (locals.var_nl_dn4 + locals.var_nstar_dn4))), (((locals.var_nl_dn5 + locals.var_nstar_dn5) * assign32890_e43574) + (assign32890_e43571 * (locals.var_nl_dn5 + locals.var_nstar_dn5))), (((locals.var_nl_dn6 + locals.var_nstar_dn6) * assign32890_e43574) + (assign32890_e43571 * (locals.var_nl_dn6 + locals.var_nstar_dn6))), (((locals.var_nl_dn7 + locals.var_nstar_dn7) * assign32890_e43574) + (assign32890_e43571 * (locals.var_nl_dn7 + locals.var_nstar_dn7))), (((locals.var_nl_dn8 + locals.var_nstar_dn8) * assign32890_e43574) + (assign32890_e43571 * (locals.var_nl_dn8 + locals.var_nstar_dn8))), (((locals.var_nl_dn9 + locals.var_nstar_dn9) * assign32890_e43574) + (assign32890_e43571 * (locals.var_nl_dn9 + locals.var_nstar_dn9))), (((locals.var_nl_dn10 + locals.var_nstar_dn10) * assign32890_e43574) + (assign32890_e43571 * (locals.var_nl_dn10 + locals.var_nstar_dn10))), (((locals.var_nl_dn11 + locals.var_nstar_dn11) * assign32890_e43574) + (assign32890_e43571 * (locals.var_nl_dn11 + locals.var_nstar_dn11))), (((locals.var_nl_dn12 + locals.var_nstar_dn12) * assign32890_e43574) + (assign32890_e43571 * (locals.var_nl_dn12 + locals.var_nstar_dn12))), (((locals.var_nl_dn13 + locals.var_nstar_dn13) * assign32890_e43574) + (assign32890_e43571 * (locals.var_nl_dn13 + locals.var_nstar_dn13))), (((locals.var_nl_dn14 + locals.var_nstar_dn14) * assign32890_e43574) + (assign32890_e43571 * (locals.var_nl_dn14 + locals.var_nstar_dn14))),)
    } else {
        (locals.var_t0d, locals.var_t0d_dn0, locals.var_t0d_dn2, locals.var_t0d_dn3, locals.var_t0d_dn4, locals.var_t0d_dn5, locals.var_t0d_dn6, locals.var_t0d_dn7, locals.var_t0d_dn8, locals.var_t0d_dn9, locals.var_t0d_dn10, locals.var_t0d_dn11, locals.var_t0d_dn12, locals.var_t0d_dn13, locals.var_t0d_dn14,)
    }
};
        locals.var_t0d = assign32890_e43577;
        locals.var_t0d_dn0 = assign32890_e43577_d_n0;
        locals.var_t0d_dn2 = assign32890_e43577_d_n2;
        locals.var_t0d_dn3 = assign32890_e43577_d_n3;
        locals.var_t0d_dn4 = assign32890_e43577_d_n4;
        locals.var_t0d_dn5 = assign32890_e43577_d_n5;
        locals.var_t0d_dn6 = assign32890_e43577_d_n6;
        locals.var_t0d_dn7 = assign32890_e43577_d_n7;
        locals.var_t0d_dn8 = assign32890_e43577_d_n8;
        locals.var_t0d_dn9 = assign32890_e43577_d_n9;
        locals.var_t0d_dn10 = assign32890_e43577_d_n10;
        locals.var_t0d_dn11 = assign32890_e43577_d_n11;
        locals.var_t0d_dn12 = assign32890_e43577_d_n12;
        locals.var_t0d_dn13 = assign32890_e43577_d_n13;
        locals.var_t0d_dn14 = assign32890_e43577_d_n14;
        locals.var_t0d_rv = 0.0;

        let (assign32900_e43585, assign32900_e43585_d_n0, assign32900_e43585_d_n2, assign32900_e43585_d_n3, assign32900_e43585_d_n4, assign32900_e43585_d_n5, assign32900_e43585_d_n6, assign32900_e43585_d_n7, assign32900_e43585_d_n8, assign32900_e43585_d_n9, assign32900_e43585_d_n10, assign32900_e43585_d_n11, assign32900_e43585_d_n12, assign32900_e43585_d_n13, assign32900_e43585_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32900_e43581: f64 = (locals.var_noia_edge * 1.60219e-19);
        let assign32900_e43583: f64 = (assign32900_e43581 * locals.var_vt);
        (assign32900_e43583, 0.0, 0.0, 0.0, (assign32900_e43581 * locals.var_vt_dn4), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0e, locals.var_t0e_dn0, locals.var_t0e_dn2, locals.var_t0e_dn3, locals.var_t0e_dn4, locals.var_t0e_dn5, locals.var_t0e_dn6, locals.var_t0e_dn7, locals.var_t0e_dn8, locals.var_t0e_dn9, locals.var_t0e_dn10, locals.var_t0e_dn11, locals.var_t0e_dn12, locals.var_t0e_dn13, locals.var_t0e_dn14,)
    }
};
        locals.var_t0e = assign32900_e43585;
        locals.var_t0e_dn0 = assign32900_e43585_d_n0;
        locals.var_t0e_dn2 = assign32900_e43585_d_n2;
        locals.var_t0e_dn3 = assign32900_e43585_d_n3;
        locals.var_t0e_dn4 = assign32900_e43585_d_n4;
        locals.var_t0e_dn5 = assign32900_e43585_d_n5;
        locals.var_t0e_dn6 = assign32900_e43585_d_n6;
        locals.var_t0e_dn7 = assign32900_e43585_d_n7;
        locals.var_t0e_dn8 = assign32900_e43585_d_n8;
        locals.var_t0e_dn9 = assign32900_e43585_d_n9;
        locals.var_t0e_dn10 = assign32900_e43585_d_n10;
        locals.var_t0e_dn11 = assign32900_e43585_d_n11;
        locals.var_t0e_dn12 = assign32900_e43585_d_n12;
        locals.var_t0e_dn13 = assign32900_e43585_d_n13;
        locals.var_t0e_dn14 = assign32900_e43585_d_n14;
        locals.var_t0e_rv = 0.0;

        let (assign32910_e43599, assign32910_e43599_d_n0, assign32910_e43599_d_n2, assign32910_e43599_d_n3, assign32910_e43599_d_n4, assign32910_e43599_d_n5, assign32910_e43599_d_n6, assign32910_e43599_d_n7, assign32910_e43599_d_n8, assign32910_e43599_d_n9, assign32910_e43599_d_n10, assign32910_e43599_d_n11, assign32910_e43599_d_n12, assign32910_e43599_d_n13, assign32910_e43599_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32910_e43589: f64 = (2.0 * locals.var_nq_edge);
        let assign32910_e43591: f64 = (assign32910_e43589 * locals.var_cox);
        let assign32910_e43593: f64 = (assign32910_e43591 * locals.var_vt);
        let assign32910_e43595: f64 = (assign32910_e43593 * locals.var_qs_edge);
        let assign32910_e43597: f64 = (assign32910_e43595 / 1.60219e-19);
        (assign32910_e43597, ((((((2.0 * locals.var_nq_edge_dn0) * locals.var_cox) * locals.var_vt) * locals.var_qs_edge) + (assign32910_e43593 * locals.var_qs_edge_dn0)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn2) * locals.var_cox) * locals.var_vt) * locals.var_qs_edge) + (assign32910_e43593 * locals.var_qs_edge_dn2)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn3) * locals.var_cox) * locals.var_vt) * locals.var_qs_edge) + (assign32910_e43593 * locals.var_qs_edge_dn3)) / 1.60219e-19), (((((((2.0 * locals.var_nq_edge_dn4) * locals.var_cox) * locals.var_vt) + (assign32910_e43591 * locals.var_vt_dn4)) * locals.var_qs_edge) + (assign32910_e43593 * locals.var_qs_edge_dn4)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn5) * locals.var_cox) * locals.var_vt) * locals.var_qs_edge) + (assign32910_e43593 * locals.var_qs_edge_dn5)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn6) * locals.var_cox) * locals.var_vt) * locals.var_qs_edge) + (assign32910_e43593 * locals.var_qs_edge_dn6)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn7) * locals.var_cox) * locals.var_vt) * locals.var_qs_edge) + (assign32910_e43593 * locals.var_qs_edge_dn7)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn8) * locals.var_cox) * locals.var_vt) * locals.var_qs_edge) + (assign32910_e43593 * locals.var_qs_edge_dn8)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn9) * locals.var_cox) * locals.var_vt) * locals.var_qs_edge) + (assign32910_e43593 * locals.var_qs_edge_dn9)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn10) * locals.var_cox) * locals.var_vt) * locals.var_qs_edge) + (assign32910_e43593 * locals.var_qs_edge_dn10)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn11) * locals.var_cox) * locals.var_vt) * locals.var_qs_edge) + (assign32910_e43593 * locals.var_qs_edge_dn11)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn12) * locals.var_cox) * locals.var_vt) * locals.var_qs_edge) + (assign32910_e43593 * locals.var_qs_edge_dn12)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn13) * locals.var_cox) * locals.var_vt) * locals.var_qs_edge) + (assign32910_e43593 * locals.var_qs_edge_dn13)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn14) * locals.var_cox) * locals.var_vt) * locals.var_qs_edge) + (assign32910_e43593 * locals.var_qs_edge_dn14)) / 1.60219e-19),)
    } else {
        (locals.var_n0, locals.var_n0_dn0, locals.var_n0_dn2, locals.var_n0_dn3, locals.var_n0_dn4, locals.var_n0_dn5, locals.var_n0_dn6, locals.var_n0_dn7, locals.var_n0_dn8, locals.var_n0_dn9, locals.var_n0_dn10, locals.var_n0_dn11, locals.var_n0_dn12, locals.var_n0_dn13, locals.var_n0_dn14,)
    }
};
        locals.var_n0 = assign32910_e43599;
        locals.var_n0_dn0 = assign32910_e43599_d_n0;
        locals.var_n0_dn2 = assign32910_e43599_d_n2;
        locals.var_n0_dn3 = assign32910_e43599_d_n3;
        locals.var_n0_dn4 = assign32910_e43599_d_n4;
        locals.var_n0_dn5 = assign32910_e43599_d_n5;
        locals.var_n0_dn6 = assign32910_e43599_d_n6;
        locals.var_n0_dn7 = assign32910_e43599_d_n7;
        locals.var_n0_dn8 = assign32910_e43599_d_n8;
        locals.var_n0_dn9 = assign32910_e43599_d_n9;
        locals.var_n0_dn10 = assign32910_e43599_d_n10;
        locals.var_n0_dn11 = assign32910_e43599_d_n11;
        locals.var_n0_dn12 = assign32910_e43599_d_n12;
        locals.var_n0_dn13 = assign32910_e43599_d_n13;
        locals.var_n0_dn14 = assign32910_e43599_d_n14;
        locals.var_n0_rv = 0.0;

        let (assign32920_e43614, assign32920_e43614_d_n0, assign32920_e43614_d_n2, assign32920_e43614_d_n3, assign32920_e43614_d_n4, assign32920_e43614_d_n5, assign32920_e43614_d_n6, assign32920_e43614_d_n7, assign32920_e43614_d_n8, assign32920_e43614_d_n9, assign32920_e43614_d_n10, assign32920_e43614_d_n11, assign32920_e43614_d_n12, assign32920_e43614_d_n13, assign32920_e43614_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32920_e43604: f64 = (locals.var_n0 + locals.var_nstar);
        let assign32920_e43607: f64 = (locals.var_nl + locals.var_nstar);
        let assign32920_e43608: f64 = (assign32920_e43604 / assign32920_e43607);
        let assign32920_e43610: f64 = (assign32920_e43608).max(1e-38);
        let assign32920_e43611: f64 = (assign32920_e43610).ln();
        let assign32920_e43612: f64 = (locals.var_noia_edge * assign32920_e43611);
        (assign32920_e43612, (locals.var_noia_edge * (if assign32920_e43608 >= 1e-38 { ((((locals.var_n0_dn0 + locals.var_nstar_dn0) * assign32920_e43607) - (assign32920_e43604 * (locals.var_nl_dn0 + locals.var_nstar_dn0))) / (assign32920_e43607 * assign32920_e43607)) } else { 0.0 } / assign32920_e43610)), (locals.var_noia_edge * (if assign32920_e43608 >= 1e-38 { ((((locals.var_n0_dn2 + locals.var_nstar_dn2) * assign32920_e43607) - (assign32920_e43604 * (locals.var_nl_dn2 + locals.var_nstar_dn2))) / (assign32920_e43607 * assign32920_e43607)) } else { 0.0 } / assign32920_e43610)), (locals.var_noia_edge * (if assign32920_e43608 >= 1e-38 { ((((locals.var_n0_dn3 + locals.var_nstar_dn3) * assign32920_e43607) - (assign32920_e43604 * (locals.var_nl_dn3 + locals.var_nstar_dn3))) / (assign32920_e43607 * assign32920_e43607)) } else { 0.0 } / assign32920_e43610)), (locals.var_noia_edge * (if assign32920_e43608 >= 1e-38 { ((((locals.var_n0_dn4 + locals.var_nstar_dn4) * assign32920_e43607) - (assign32920_e43604 * (locals.var_nl_dn4 + locals.var_nstar_dn4))) / (assign32920_e43607 * assign32920_e43607)) } else { 0.0 } / assign32920_e43610)), (locals.var_noia_edge * (if assign32920_e43608 >= 1e-38 { ((((locals.var_n0_dn5 + locals.var_nstar_dn5) * assign32920_e43607) - (assign32920_e43604 * (locals.var_nl_dn5 + locals.var_nstar_dn5))) / (assign32920_e43607 * assign32920_e43607)) } else { 0.0 } / assign32920_e43610)), (locals.var_noia_edge * (if assign32920_e43608 >= 1e-38 { ((((locals.var_n0_dn6 + locals.var_nstar_dn6) * assign32920_e43607) - (assign32920_e43604 * (locals.var_nl_dn6 + locals.var_nstar_dn6))) / (assign32920_e43607 * assign32920_e43607)) } else { 0.0 } / assign32920_e43610)), (locals.var_noia_edge * (if assign32920_e43608 >= 1e-38 { ((((locals.var_n0_dn7 + locals.var_nstar_dn7) * assign32920_e43607) - (assign32920_e43604 * (locals.var_nl_dn7 + locals.var_nstar_dn7))) / (assign32920_e43607 * assign32920_e43607)) } else { 0.0 } / assign32920_e43610)), (locals.var_noia_edge * (if assign32920_e43608 >= 1e-38 { ((((locals.var_n0_dn8 + locals.var_nstar_dn8) * assign32920_e43607) - (assign32920_e43604 * (locals.var_nl_dn8 + locals.var_nstar_dn8))) / (assign32920_e43607 * assign32920_e43607)) } else { 0.0 } / assign32920_e43610)), (locals.var_noia_edge * (if assign32920_e43608 >= 1e-38 { ((((locals.var_n0_dn9 + locals.var_nstar_dn9) * assign32920_e43607) - (assign32920_e43604 * (locals.var_nl_dn9 + locals.var_nstar_dn9))) / (assign32920_e43607 * assign32920_e43607)) } else { 0.0 } / assign32920_e43610)), (locals.var_noia_edge * (if assign32920_e43608 >= 1e-38 { ((((locals.var_n0_dn10 + locals.var_nstar_dn10) * assign32920_e43607) - (assign32920_e43604 * (locals.var_nl_dn10 + locals.var_nstar_dn10))) / (assign32920_e43607 * assign32920_e43607)) } else { 0.0 } / assign32920_e43610)), (locals.var_noia_edge * (if assign32920_e43608 >= 1e-38 { ((((locals.var_n0_dn11 + locals.var_nstar_dn11) * assign32920_e43607) - (assign32920_e43604 * (locals.var_nl_dn11 + locals.var_nstar_dn11))) / (assign32920_e43607 * assign32920_e43607)) } else { 0.0 } / assign32920_e43610)), (locals.var_noia_edge * (if assign32920_e43608 >= 1e-38 { ((((locals.var_n0_dn12 + locals.var_nstar_dn12) * assign32920_e43607) - (assign32920_e43604 * (locals.var_nl_dn12 + locals.var_nstar_dn12))) / (assign32920_e43607 * assign32920_e43607)) } else { 0.0 } / assign32920_e43610)), (locals.var_noia_edge * (if assign32920_e43608 >= 1e-38 { ((((locals.var_n0_dn13 + locals.var_nstar_dn13) * assign32920_e43607) - (assign32920_e43604 * (locals.var_nl_dn13 + locals.var_nstar_dn13))) / (assign32920_e43607 * assign32920_e43607)) } else { 0.0 } / assign32920_e43610)), (locals.var_noia_edge * (if assign32920_e43608 >= 1e-38 { ((((locals.var_n0_dn14 + locals.var_nstar_dn14) * assign32920_e43607) - (assign32920_e43604 * (locals.var_nl_dn14 + locals.var_nstar_dn14))) / (assign32920_e43607 * assign32920_e43607)) } else { 0.0 } / assign32920_e43610)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign32920_e43614;
        locals.var_t1_dn0 = assign32920_e43614_d_n0;
        locals.var_t1_dn2 = assign32920_e43614_d_n2;
        locals.var_t1_dn3 = assign32920_e43614_d_n3;
        locals.var_t1_dn4 = assign32920_e43614_d_n4;
        locals.var_t1_dn5 = assign32920_e43614_d_n5;
        locals.var_t1_dn6 = assign32920_e43614_d_n6;
        locals.var_t1_dn7 = assign32920_e43614_d_n7;
        locals.var_t1_dn8 = assign32920_e43614_d_n8;
        locals.var_t1_dn9 = assign32920_e43614_d_n9;
        locals.var_t1_dn10 = assign32920_e43614_d_n10;
        locals.var_t1_dn11 = assign32920_e43614_d_n11;
        locals.var_t1_dn12 = assign32920_e43614_d_n12;
        locals.var_t1_dn13 = assign32920_e43614_d_n13;
        locals.var_t1_dn14 = assign32920_e43614_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign32930_e43622, assign32930_e43622_d_n0, assign32930_e43622_d_n2, assign32930_e43622_d_n3, assign32930_e43622_d_n4, assign32930_e43622_d_n5, assign32930_e43622_d_n6, assign32930_e43622_d_n7, assign32930_e43622_d_n8, assign32930_e43622_d_n9, assign32930_e43622_d_n10, assign32930_e43622_d_n11, assign32930_e43622_d_n12, assign32930_e43622_d_n13, assign32930_e43622_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32930_e43619: f64 = (locals.var_n0 - locals.var_nl);
        let assign32930_e43620: f64 = (locals.var_noib_edge * assign32930_e43619);
        (assign32930_e43620, (locals.var_noib_edge * (locals.var_n0_dn0 - locals.var_nl_dn0)), (locals.var_noib_edge * (locals.var_n0_dn2 - locals.var_nl_dn2)), (locals.var_noib_edge * (locals.var_n0_dn3 - locals.var_nl_dn3)), (locals.var_noib_edge * (locals.var_n0_dn4 - locals.var_nl_dn4)), (locals.var_noib_edge * (locals.var_n0_dn5 - locals.var_nl_dn5)), (locals.var_noib_edge * (locals.var_n0_dn6 - locals.var_nl_dn6)), (locals.var_noib_edge * (locals.var_n0_dn7 - locals.var_nl_dn7)), (locals.var_noib_edge * (locals.var_n0_dn8 - locals.var_nl_dn8)), (locals.var_noib_edge * (locals.var_n0_dn9 - locals.var_nl_dn9)), (locals.var_noib_edge * (locals.var_n0_dn10 - locals.var_nl_dn10)), (locals.var_noib_edge * (locals.var_n0_dn11 - locals.var_nl_dn11)), (locals.var_noib_edge * (locals.var_n0_dn12 - locals.var_nl_dn12)), (locals.var_noib_edge * (locals.var_n0_dn13 - locals.var_nl_dn13)), (locals.var_noib_edge * (locals.var_n0_dn14 - locals.var_nl_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign32930_e43622;
        locals.var_t2_dn0 = assign32930_e43622_d_n0;
        locals.var_t2_dn2 = assign32930_e43622_d_n2;
        locals.var_t2_dn3 = assign32930_e43622_d_n3;
        locals.var_t2_dn4 = assign32930_e43622_d_n4;
        locals.var_t2_dn5 = assign32930_e43622_d_n5;
        locals.var_t2_dn6 = assign32930_e43622_d_n6;
        locals.var_t2_dn7 = assign32930_e43622_d_n7;
        locals.var_t2_dn8 = assign32930_e43622_d_n8;
        locals.var_t2_dn9 = assign32930_e43622_d_n9;
        locals.var_t2_dn10 = assign32930_e43622_d_n10;
        locals.var_t2_dn11 = assign32930_e43622_d_n11;
        locals.var_t2_dn12 = assign32930_e43622_d_n12;
        locals.var_t2_dn13 = assign32930_e43622_d_n13;
        locals.var_t2_dn14 = assign32930_e43622_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign32940_e43636, assign32940_e43636_d_n0, assign32940_e43636_d_n2, assign32940_e43636_d_n3, assign32940_e43636_d_n4, assign32940_e43636_d_n5, assign32940_e43636_d_n6, assign32940_e43636_d_n7, assign32940_e43636_d_n8, assign32940_e43636_d_n9, assign32940_e43636_d_n10, assign32940_e43636_d_n11, assign32940_e43636_d_n12, assign32940_e43636_d_n13, assign32940_e43636_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32940_e43626: f64 = (0.5 * locals.var_noic_edge);
        let assign32940_e43629: f64 = (locals.var_n0 * locals.var_n0);
        let assign32940_e43632: f64 = (locals.var_nl * locals.var_nl);
        let assign32940_e43633: f64 = (assign32940_e43629 - assign32940_e43632);
        let assign32940_e43634: f64 = (assign32940_e43626 * assign32940_e43633);
        (assign32940_e43634, (assign32940_e43626 * (((locals.var_n0_dn0 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn0)) - ((locals.var_nl_dn0 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn0)))), (assign32940_e43626 * (((locals.var_n0_dn2 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn2)) - ((locals.var_nl_dn2 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn2)))), (assign32940_e43626 * (((locals.var_n0_dn3 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn3)) - ((locals.var_nl_dn3 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn3)))), (assign32940_e43626 * (((locals.var_n0_dn4 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn4)) - ((locals.var_nl_dn4 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn4)))), (assign32940_e43626 * (((locals.var_n0_dn5 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn5)) - ((locals.var_nl_dn5 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn5)))), (assign32940_e43626 * (((locals.var_n0_dn6 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn6)) - ((locals.var_nl_dn6 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn6)))), (assign32940_e43626 * (((locals.var_n0_dn7 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn7)) - ((locals.var_nl_dn7 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn7)))), (assign32940_e43626 * (((locals.var_n0_dn8 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn8)) - ((locals.var_nl_dn8 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn8)))), (assign32940_e43626 * (((locals.var_n0_dn9 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn9)) - ((locals.var_nl_dn9 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn9)))), (assign32940_e43626 * (((locals.var_n0_dn10 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn10)) - ((locals.var_nl_dn10 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn10)))), (assign32940_e43626 * (((locals.var_n0_dn11 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn11)) - ((locals.var_nl_dn11 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn11)))), (assign32940_e43626 * (((locals.var_n0_dn12 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn12)) - ((locals.var_nl_dn12 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn12)))), (assign32940_e43626 * (((locals.var_n0_dn13 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn13)) - ((locals.var_nl_dn13 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn13)))), (assign32940_e43626 * (((locals.var_n0_dn14 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn14)) - ((locals.var_nl_dn14 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn14)))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign32940_e43636;
        locals.var_t3_dn0 = assign32940_e43636_d_n0;
        locals.var_t3_dn2 = assign32940_e43636_d_n2;
        locals.var_t3_dn3 = assign32940_e43636_d_n3;
        locals.var_t3_dn4 = assign32940_e43636_d_n4;
        locals.var_t3_dn5 = assign32940_e43636_d_n5;
        locals.var_t3_dn6 = assign32940_e43636_d_n6;
        locals.var_t3_dn7 = assign32940_e43636_d_n7;
        locals.var_t3_dn8 = assign32940_e43636_d_n8;
        locals.var_t3_dn9 = assign32940_e43636_d_n9;
        locals.var_t3_dn10 = assign32940_e43636_d_n10;
        locals.var_t3_dn11 = assign32940_e43636_d_n11;
        locals.var_t3_dn12 = assign32940_e43636_d_n12;
        locals.var_t3_dn13 = assign32940_e43636_d_n13;
        locals.var_t3_dn14 = assign32940_e43636_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign32950_e43646, assign32950_e43646_d_n0, assign32950_e43646_d_n2, assign32950_e43646_d_n3, assign32950_e43646_d_n4, assign32950_e43646_d_n5, assign32950_e43646_d_n6, assign32950_e43646_d_n7, assign32950_e43646_d_n8, assign32950_e43646_d_n9, assign32950_e43646_d_n10, assign32950_e43646_d_n11, assign32950_e43646_d_n12, assign32950_e43646_d_n13, assign32950_e43646_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32950_e43640: f64 = (10000000000.0 * locals.var_leffnoisq_edge);
        let assign32950_e43642: f64 = (assign32950_e43640 * p.p957);
        let assign32950_e43644: f64 = (assign32950_e43642 * p.p2);
        (assign32950_e43644, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign32950_e43646;
        locals.var_t4_dn0 = assign32950_e43646_d_n0;
        locals.var_t4_dn2 = assign32950_e43646_d_n2;
        locals.var_t4_dn3 = assign32950_e43646_d_n3;
        locals.var_t4_dn4 = assign32950_e43646_d_n4;
        locals.var_t4_dn5 = assign32950_e43646_d_n5;
        locals.var_t4_dn6 = assign32950_e43646_d_n6;
        locals.var_t4_dn7 = assign32950_e43646_d_n7;
        locals.var_t4_dn8 = assign32950_e43646_d_n8;
        locals.var_t4_dn9 = assign32950_e43646_d_n9;
        locals.var_t4_dn10 = assign32950_e43646_d_n10;
        locals.var_t4_dn11 = assign32950_e43646_d_n11;
        locals.var_t4_dn12 = assign32950_e43646_d_n12;
        locals.var_t4_dn13 = assign32950_e43646_d_n13;
        locals.var_t4_dn14 = assign32950_e43646_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign32960_e43668, assign32960_e43668_d_n0, assign32960_e43668_d_n2, assign32960_e43668_d_n3, assign32960_e43668_d_n4, assign32960_e43668_d_n5, assign32960_e43668_d_n6, assign32960_e43668_d_n7, assign32960_e43668_d_n8, assign32960_e43668_d_n9, assign32960_e43668_d_n10, assign32960_e43668_d_n11, assign32960_e43668_d_n12, assign32960_e43668_d_n13, assign32960_e43668_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32960_e43650: f64 = (locals.var_t0a / locals.var_t0);
        let assign32960_e43653: f64 = (locals.var_t1 + locals.var_t2);
        let assign32960_e43655: f64 = (assign32960_e43653 + locals.var_t3);
        let assign32960_e43656: f64 = (assign32960_e43650 * assign32960_e43655);
        let assign32960_e43659: f64 = (locals.var_t0b / locals.var_t4);
        let assign32960_e43661: f64 = (assign32960_e43659 * locals.var_delclm);
        let assign32960_e43663: f64 = (assign32960_e43661 * locals.var_t0c);
        let assign32960_e43665: f64 = (assign32960_e43663 / locals.var_t0d);
        let assign32960_e43666: f64 = (assign32960_e43656 + assign32960_e43665);
        (assign32960_e43666, ((((((locals.var_t0a_dn0 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn0)) / (locals.var_t0 * locals.var_t0)) * assign32960_e43655) + (assign32960_e43650 * ((locals.var_t1_dn0 + locals.var_t2_dn0) + locals.var_t3_dn0))) + ((((((((((locals.var_t0b_dn0 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn0)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign32960_e43659 * locals.var_delclm_dn0)) * locals.var_t0c) + (assign32960_e43661 * locals.var_t0c_dn0)) * locals.var_t0d) - (assign32960_e43663 * locals.var_t0d_dn0)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn2 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn2)) / (locals.var_t0 * locals.var_t0)) * assign32960_e43655) + (assign32960_e43650 * ((locals.var_t1_dn2 + locals.var_t2_dn2) + locals.var_t3_dn2))) + ((((((((((locals.var_t0b_dn2 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn2)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign32960_e43659 * locals.var_delclm_dn2)) * locals.var_t0c) + (assign32960_e43661 * locals.var_t0c_dn2)) * locals.var_t0d) - (assign32960_e43663 * locals.var_t0d_dn2)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn3 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn3)) / (locals.var_t0 * locals.var_t0)) * assign32960_e43655) + (assign32960_e43650 * ((locals.var_t1_dn3 + locals.var_t2_dn3) + locals.var_t3_dn3))) + ((((((((((locals.var_t0b_dn3 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn3)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign32960_e43659 * locals.var_delclm_dn3)) * locals.var_t0c) + (assign32960_e43661 * locals.var_t0c_dn3)) * locals.var_t0d) - (assign32960_e43663 * locals.var_t0d_dn3)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn4 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)) * assign32960_e43655) + (assign32960_e43650 * ((locals.var_t1_dn4 + locals.var_t2_dn4) + locals.var_t3_dn4))) + ((((((((((locals.var_t0b_dn4 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign32960_e43659 * locals.var_delclm_dn4)) * locals.var_t0c) + (assign32960_e43661 * locals.var_t0c_dn4)) * locals.var_t0d) - (assign32960_e43663 * locals.var_t0d_dn4)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn5 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)) * assign32960_e43655) + (assign32960_e43650 * ((locals.var_t1_dn5 + locals.var_t2_dn5) + locals.var_t3_dn5))) + ((((((((((locals.var_t0b_dn5 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign32960_e43659 * locals.var_delclm_dn5)) * locals.var_t0c) + (assign32960_e43661 * locals.var_t0c_dn5)) * locals.var_t0d) - (assign32960_e43663 * locals.var_t0d_dn5)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn6 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)) * assign32960_e43655) + (assign32960_e43650 * ((locals.var_t1_dn6 + locals.var_t2_dn6) + locals.var_t3_dn6))) + ((((((((((locals.var_t0b_dn6 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign32960_e43659 * locals.var_delclm_dn6)) * locals.var_t0c) + (assign32960_e43661 * locals.var_t0c_dn6)) * locals.var_t0d) - (assign32960_e43663 * locals.var_t0d_dn6)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn7 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)) * assign32960_e43655) + (assign32960_e43650 * ((locals.var_t1_dn7 + locals.var_t2_dn7) + locals.var_t3_dn7))) + ((((((((((locals.var_t0b_dn7 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn7)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign32960_e43659 * locals.var_delclm_dn7)) * locals.var_t0c) + (assign32960_e43661 * locals.var_t0c_dn7)) * locals.var_t0d) - (assign32960_e43663 * locals.var_t0d_dn7)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn8 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)) * assign32960_e43655) + (assign32960_e43650 * ((locals.var_t1_dn8 + locals.var_t2_dn8) + locals.var_t3_dn8))) + ((((((((((locals.var_t0b_dn8 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign32960_e43659 * locals.var_delclm_dn8)) * locals.var_t0c) + (assign32960_e43661 * locals.var_t0c_dn8)) * locals.var_t0d) - (assign32960_e43663 * locals.var_t0d_dn8)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn9 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn9)) / (locals.var_t0 * locals.var_t0)) * assign32960_e43655) + (assign32960_e43650 * ((locals.var_t1_dn9 + locals.var_t2_dn9) + locals.var_t3_dn9))) + ((((((((((locals.var_t0b_dn9 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn9)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign32960_e43659 * locals.var_delclm_dn9)) * locals.var_t0c) + (assign32960_e43661 * locals.var_t0c_dn9)) * locals.var_t0d) - (assign32960_e43663 * locals.var_t0d_dn9)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn10 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)) * assign32960_e43655) + (assign32960_e43650 * ((locals.var_t1_dn10 + locals.var_t2_dn10) + locals.var_t3_dn10))) + ((((((((((locals.var_t0b_dn10 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign32960_e43659 * locals.var_delclm_dn10)) * locals.var_t0c) + (assign32960_e43661 * locals.var_t0c_dn10)) * locals.var_t0d) - (assign32960_e43663 * locals.var_t0d_dn10)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn11 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn11)) / (locals.var_t0 * locals.var_t0)) * assign32960_e43655) + (assign32960_e43650 * ((locals.var_t1_dn11 + locals.var_t2_dn11) + locals.var_t3_dn11))) + ((((((((((locals.var_t0b_dn11 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn11)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign32960_e43659 * locals.var_delclm_dn11)) * locals.var_t0c) + (assign32960_e43661 * locals.var_t0c_dn11)) * locals.var_t0d) - (assign32960_e43663 * locals.var_t0d_dn11)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn12 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn12)) / (locals.var_t0 * locals.var_t0)) * assign32960_e43655) + (assign32960_e43650 * ((locals.var_t1_dn12 + locals.var_t2_dn12) + locals.var_t3_dn12))) + ((((((((((locals.var_t0b_dn12 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn12)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign32960_e43659 * locals.var_delclm_dn12)) * locals.var_t0c) + (assign32960_e43661 * locals.var_t0c_dn12)) * locals.var_t0d) - (assign32960_e43663 * locals.var_t0d_dn12)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn13 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn13)) / (locals.var_t0 * locals.var_t0)) * assign32960_e43655) + (assign32960_e43650 * ((locals.var_t1_dn13 + locals.var_t2_dn13) + locals.var_t3_dn13))) + ((((((((((locals.var_t0b_dn13 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn13)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign32960_e43659 * locals.var_delclm_dn13)) * locals.var_t0c) + (assign32960_e43661 * locals.var_t0c_dn13)) * locals.var_t0d) - (assign32960_e43663 * locals.var_t0d_dn13)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn14 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn14)) / (locals.var_t0 * locals.var_t0)) * assign32960_e43655) + (assign32960_e43650 * ((locals.var_t1_dn14 + locals.var_t2_dn14) + locals.var_t3_dn14))) + ((((((((((locals.var_t0b_dn14 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn14)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign32960_e43659 * locals.var_delclm_dn14)) * locals.var_t0c) + (assign32960_e43661 * locals.var_t0c_dn14)) * locals.var_t0d) - (assign32960_e43663 * locals.var_t0d_dn14)) / (locals.var_t0d * locals.var_t0d))),)
    } else {
        (locals.var_ssi, locals.var_ssi_dn0, locals.var_ssi_dn2, locals.var_ssi_dn3, locals.var_ssi_dn4, locals.var_ssi_dn5, locals.var_ssi_dn6, locals.var_ssi_dn7, locals.var_ssi_dn8, locals.var_ssi_dn9, locals.var_ssi_dn10, locals.var_ssi_dn11, locals.var_ssi_dn12, locals.var_ssi_dn13, locals.var_ssi_dn14,)
    }
};
        locals.var_ssi = assign32960_e43668;
        locals.var_ssi_dn0 = assign32960_e43668_d_n0;
        locals.var_ssi_dn2 = assign32960_e43668_d_n2;
        locals.var_ssi_dn3 = assign32960_e43668_d_n3;
        locals.var_ssi_dn4 = assign32960_e43668_d_n4;
        locals.var_ssi_dn5 = assign32960_e43668_d_n5;
        locals.var_ssi_dn6 = assign32960_e43668_d_n6;
        locals.var_ssi_dn7 = assign32960_e43668_d_n7;
        locals.var_ssi_dn8 = assign32960_e43668_d_n8;
        locals.var_ssi_dn9 = assign32960_e43668_d_n9;
        locals.var_ssi_dn10 = assign32960_e43668_d_n10;
        locals.var_ssi_dn11 = assign32960_e43668_d_n11;
        locals.var_ssi_dn12 = assign32960_e43668_d_n12;
        locals.var_ssi_dn13 = assign32960_e43668_d_n13;
        locals.var_ssi_dn14 = assign32960_e43668_d_n14;
        locals.var_ssi_rv = 0.0;

        let (assign32970_e43682, assign32970_e43682_d_n0, assign32970_e43682_d_n2, assign32970_e43682_d_n3, assign32970_e43682_d_n4, assign32970_e43682_d_n5, assign32970_e43682_d_n6, assign32970_e43682_d_n7, assign32970_e43682_d_n8, assign32970_e43682_d_n9, assign32970_e43682_d_n10, assign32970_e43682_d_n11, assign32970_e43682_d_n12, assign32970_e43682_d_n13, assign32970_e43682_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32970_e43672: f64 = (p.p957 * p.p2);
        let assign32970_e43674: f64 = (assign32970_e43672 * locals.var_leffnoi_edge);
        let assign32970_e43676: f64 = (assign32970_e43674 * 10000000000.0);
        let assign32970_e43678: f64 = (assign32970_e43676 * locals.var_nstar);
        let assign32970_e43680: f64 = (assign32970_e43678 * locals.var_nstar);
        (assign32970_e43680, (((assign32970_e43676 * locals.var_nstar_dn0) * locals.var_nstar) + (assign32970_e43678 * locals.var_nstar_dn0)), (((assign32970_e43676 * locals.var_nstar_dn2) * locals.var_nstar) + (assign32970_e43678 * locals.var_nstar_dn2)), (((assign32970_e43676 * locals.var_nstar_dn3) * locals.var_nstar) + (assign32970_e43678 * locals.var_nstar_dn3)), (((assign32970_e43676 * locals.var_nstar_dn4) * locals.var_nstar) + (assign32970_e43678 * locals.var_nstar_dn4)), (((assign32970_e43676 * locals.var_nstar_dn5) * locals.var_nstar) + (assign32970_e43678 * locals.var_nstar_dn5)), (((assign32970_e43676 * locals.var_nstar_dn6) * locals.var_nstar) + (assign32970_e43678 * locals.var_nstar_dn6)), (((assign32970_e43676 * locals.var_nstar_dn7) * locals.var_nstar) + (assign32970_e43678 * locals.var_nstar_dn7)), (((assign32970_e43676 * locals.var_nstar_dn8) * locals.var_nstar) + (assign32970_e43678 * locals.var_nstar_dn8)), (((assign32970_e43676 * locals.var_nstar_dn9) * locals.var_nstar) + (assign32970_e43678 * locals.var_nstar_dn9)), (((assign32970_e43676 * locals.var_nstar_dn10) * locals.var_nstar) + (assign32970_e43678 * locals.var_nstar_dn10)), (((assign32970_e43676 * locals.var_nstar_dn11) * locals.var_nstar) + (assign32970_e43678 * locals.var_nstar_dn11)), (((assign32970_e43676 * locals.var_nstar_dn12) * locals.var_nstar) + (assign32970_e43678 * locals.var_nstar_dn12)), (((assign32970_e43676 * locals.var_nstar_dn13) * locals.var_nstar) + (assign32970_e43678 * locals.var_nstar_dn13)), (((assign32970_e43676 * locals.var_nstar_dn14) * locals.var_nstar) + (assign32970_e43678 * locals.var_nstar_dn14)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn13, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign32970_e43682;
        locals.var_t5_dn0 = assign32970_e43682_d_n0;
        locals.var_t5_dn2 = assign32970_e43682_d_n2;
        locals.var_t5_dn3 = assign32970_e43682_d_n3;
        locals.var_t5_dn4 = assign32970_e43682_d_n4;
        locals.var_t5_dn5 = assign32970_e43682_d_n5;
        locals.var_t5_dn6 = assign32970_e43682_d_n6;
        locals.var_t5_dn7 = assign32970_e43682_d_n7;
        locals.var_t5_dn8 = assign32970_e43682_d_n8;
        locals.var_t5_dn9 = assign32970_e43682_d_n9;
        locals.var_t5_dn10 = assign32970_e43682_d_n10;
        locals.var_t5_dn11 = assign32970_e43682_d_n11;
        locals.var_t5_dn12 = assign32970_e43682_d_n12;
        locals.var_t5_dn13 = assign32970_e43682_d_n13;
        locals.var_t5_dn14 = assign32970_e43682_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign32980_e43692, assign32980_e43692_d_n0, assign32980_e43692_d_n2, assign32980_e43692_d_n3, assign32980_e43692_d_n4, assign32980_e43692_d_n5, assign32980_e43692_d_n6, assign32980_e43692_d_n7, assign32980_e43692_d_n8, assign32980_e43692_d_n9, assign32980_e43692_d_n10, assign32980_e43692_d_n11, assign32980_e43692_d_n12, assign32980_e43692_d_n13, assign32980_e43692_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32980_e43686: f64 = (locals.var_t0e / locals.var_t5);
        let assign32980_e43688: f64 = (assign32980_e43686 * locals.var_ids_edge);
        let assign32980_e43690: f64 = (assign32980_e43688 * locals.var_ids_edge);
        (assign32980_e43690, (((((((locals.var_t0e_dn0 * locals.var_t5) - (locals.var_t0e * locals.var_t5_dn0)) / (locals.var_t5 * locals.var_t5)) * locals.var_ids_edge) + (assign32980_e43686 * locals.var_ids_edge_dn0)) * locals.var_ids_edge) + (assign32980_e43688 * locals.var_ids_edge_dn0)), (((((((locals.var_t0e_dn2 * locals.var_t5) - (locals.var_t0e * locals.var_t5_dn2)) / (locals.var_t5 * locals.var_t5)) * locals.var_ids_edge) + (assign32980_e43686 * locals.var_ids_edge_dn2)) * locals.var_ids_edge) + (assign32980_e43688 * locals.var_ids_edge_dn2)), (((((((locals.var_t0e_dn3 * locals.var_t5) - (locals.var_t0e * locals.var_t5_dn3)) / (locals.var_t5 * locals.var_t5)) * locals.var_ids_edge) + (assign32980_e43686 * locals.var_ids_edge_dn3)) * locals.var_ids_edge) + (assign32980_e43688 * locals.var_ids_edge_dn3)), (((((((locals.var_t0e_dn4 * locals.var_t5) - (locals.var_t0e * locals.var_t5_dn4)) / (locals.var_t5 * locals.var_t5)) * locals.var_ids_edge) + (assign32980_e43686 * locals.var_ids_edge_dn4)) * locals.var_ids_edge) + (assign32980_e43688 * locals.var_ids_edge_dn4)), (((((((locals.var_t0e_dn5 * locals.var_t5) - (locals.var_t0e * locals.var_t5_dn5)) / (locals.var_t5 * locals.var_t5)) * locals.var_ids_edge) + (assign32980_e43686 * locals.var_ids_edge_dn5)) * locals.var_ids_edge) + (assign32980_e43688 * locals.var_ids_edge_dn5)), (((((((locals.var_t0e_dn6 * locals.var_t5) - (locals.var_t0e * locals.var_t5_dn6)) / (locals.var_t5 * locals.var_t5)) * locals.var_ids_edge) + (assign32980_e43686 * locals.var_ids_edge_dn6)) * locals.var_ids_edge) + (assign32980_e43688 * locals.var_ids_edge_dn6)), (((((((locals.var_t0e_dn7 * locals.var_t5) - (locals.var_t0e * locals.var_t5_dn7)) / (locals.var_t5 * locals.var_t5)) * locals.var_ids_edge) + (assign32980_e43686 * locals.var_ids_edge_dn7)) * locals.var_ids_edge) + (assign32980_e43688 * locals.var_ids_edge_dn7)), (((((((locals.var_t0e_dn8 * locals.var_t5) - (locals.var_t0e * locals.var_t5_dn8)) / (locals.var_t5 * locals.var_t5)) * locals.var_ids_edge) + (assign32980_e43686 * locals.var_ids_edge_dn8)) * locals.var_ids_edge) + (assign32980_e43688 * locals.var_ids_edge_dn8)), (((((((locals.var_t0e_dn9 * locals.var_t5) - (locals.var_t0e * locals.var_t5_dn9)) / (locals.var_t5 * locals.var_t5)) * locals.var_ids_edge) + (assign32980_e43686 * locals.var_ids_edge_dn9)) * locals.var_ids_edge) + (assign32980_e43688 * locals.var_ids_edge_dn9)), (((((((locals.var_t0e_dn10 * locals.var_t5) - (locals.var_t0e * locals.var_t5_dn10)) / (locals.var_t5 * locals.var_t5)) * locals.var_ids_edge) + (assign32980_e43686 * locals.var_ids_edge_dn10)) * locals.var_ids_edge) + (assign32980_e43688 * locals.var_ids_edge_dn10)), (((((((locals.var_t0e_dn11 * locals.var_t5) - (locals.var_t0e * locals.var_t5_dn11)) / (locals.var_t5 * locals.var_t5)) * locals.var_ids_edge) + (assign32980_e43686 * locals.var_ids_edge_dn11)) * locals.var_ids_edge) + (assign32980_e43688 * locals.var_ids_edge_dn11)), (((((((locals.var_t0e_dn12 * locals.var_t5) - (locals.var_t0e * locals.var_t5_dn12)) / (locals.var_t5 * locals.var_t5)) * locals.var_ids_edge) + (assign32980_e43686 * locals.var_ids_edge_dn12)) * locals.var_ids_edge) + (assign32980_e43688 * locals.var_ids_edge_dn12)), (((((((locals.var_t0e_dn13 * locals.var_t5) - (locals.var_t0e * locals.var_t5_dn13)) / (locals.var_t5 * locals.var_t5)) * locals.var_ids_edge) + (assign32980_e43686 * locals.var_ids_edge_dn13)) * locals.var_ids_edge) + (assign32980_e43688 * locals.var_ids_edge_dn13)), (((((((locals.var_t0e_dn14 * locals.var_t5) - (locals.var_t0e * locals.var_t5_dn14)) / (locals.var_t5 * locals.var_t5)) * locals.var_ids_edge) + (assign32980_e43686 * locals.var_ids_edge_dn14)) * locals.var_ids_edge) + (assign32980_e43688 * locals.var_ids_edge_dn14)),)
    } else {
        (locals.var_swi, locals.var_swi_dn0, locals.var_swi_dn2, locals.var_swi_dn3, locals.var_swi_dn4, locals.var_swi_dn5, locals.var_swi_dn6, locals.var_swi_dn7, locals.var_swi_dn8, locals.var_swi_dn9, locals.var_swi_dn10, locals.var_swi_dn11, locals.var_swi_dn12, locals.var_swi_dn13, locals.var_swi_dn14,)
    }
};
        locals.var_swi = assign32980_e43692;
        locals.var_swi_dn0 = assign32980_e43692_d_n0;
        locals.var_swi_dn2 = assign32980_e43692_d_n2;
        locals.var_swi_dn3 = assign32980_e43692_d_n3;
        locals.var_swi_dn4 = assign32980_e43692_d_n4;
        locals.var_swi_dn5 = assign32980_e43692_d_n5;
        locals.var_swi_dn6 = assign32980_e43692_d_n6;
        locals.var_swi_dn7 = assign32980_e43692_d_n7;
        locals.var_swi_dn8 = assign32980_e43692_d_n8;
        locals.var_swi_dn9 = assign32980_e43692_d_n9;
        locals.var_swi_dn10 = assign32980_e43692_d_n10;
        locals.var_swi_dn11 = assign32980_e43692_d_n11;
        locals.var_swi_dn12 = assign32980_e43692_d_n12;
        locals.var_swi_dn13 = assign32980_e43692_d_n13;
        locals.var_swi_dn14 = assign32980_e43692_d_n14;
        locals.var_swi_rv = 0.0;

        let (assign32990_e43698, assign32990_e43698_d_n0, assign32990_e43698_d_n2, assign32990_e43698_d_n3, assign32990_e43698_d_n4, assign32990_e43698_d_n5, assign32990_e43698_d_n6, assign32990_e43698_d_n7, assign32990_e43698_d_n8, assign32990_e43698_d_n9, assign32990_e43698_d_n10, assign32990_e43698_d_n11, assign32990_e43698_d_n12, assign32990_e43698_d_n13, assign32990_e43698_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32990_e43696: f64 = (locals.var_swi + locals.var_ssi);
        (assign32990_e43696, (locals.var_swi_dn0 + locals.var_ssi_dn0), (locals.var_swi_dn2 + locals.var_ssi_dn2), (locals.var_swi_dn3 + locals.var_ssi_dn3), (locals.var_swi_dn4 + locals.var_ssi_dn4), (locals.var_swi_dn5 + locals.var_ssi_dn5), (locals.var_swi_dn6 + locals.var_ssi_dn6), (locals.var_swi_dn7 + locals.var_ssi_dn7), (locals.var_swi_dn8 + locals.var_ssi_dn8), (locals.var_swi_dn9 + locals.var_ssi_dn9), (locals.var_swi_dn10 + locals.var_ssi_dn10), (locals.var_swi_dn11 + locals.var_ssi_dn11), (locals.var_swi_dn12 + locals.var_ssi_dn12), (locals.var_swi_dn13 + locals.var_ssi_dn13), (locals.var_swi_dn14 + locals.var_ssi_dn14),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12, locals.var_t6_dn13, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign32990_e43698;
        locals.var_t6_dn0 = assign32990_e43698_d_n0;
        locals.var_t6_dn2 = assign32990_e43698_d_n2;
        locals.var_t6_dn3 = assign32990_e43698_d_n3;
        locals.var_t6_dn4 = assign32990_e43698_d_n4;
        locals.var_t6_dn5 = assign32990_e43698_d_n5;
        locals.var_t6_dn6 = assign32990_e43698_d_n6;
        locals.var_t6_dn7 = assign32990_e43698_d_n7;
        locals.var_t6_dn8 = assign32990_e43698_d_n8;
        locals.var_t6_dn9 = assign32990_e43698_d_n9;
        locals.var_t6_dn10 = assign32990_e43698_d_n10;
        locals.var_t6_dn11 = assign32990_e43698_d_n11;
        locals.var_t6_dn12 = assign32990_e43698_d_n12;
        locals.var_t6_dn13 = assign32990_e43698_d_n13;
        locals.var_t6_dn14 = assign32990_e43698_d_n14;
        locals.var_t6_rv = 0.0;

        let assign33000_e43701: f64 = if locals.var_t6 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard745 = assign33000_e43701;
        locals.var_guard745_rv = 0.0;

        let (assign33010_e43711, assign33010_e43711_d_n0, assign33010_e43711_d_n2, assign33010_e43711_d_n3, assign33010_e43711_d_n4, assign33010_e43711_d_n5, assign33010_e43711_d_n6, assign33010_e43711_d_n7, assign33010_e43711_d_n8, assign33010_e43711_d_n9, assign33010_e43711_d_n10, assign33010_e43711_d_n11, assign33010_e43711_d_n12, assign33010_e43711_d_n13, assign33010_e43711_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard745 != 0.0)) {
        let assign33010_e43707: f64 = (locals.var_ssi * locals.var_swi);
        let assign33010_e43709: f64 = (assign33010_e43707 / locals.var_t6);
        (assign33010_e43709, (((((locals.var_ssi_dn0 * locals.var_swi) + (locals.var_ssi * locals.var_swi_dn0)) * locals.var_t6) - (assign33010_e43707 * locals.var_t6_dn0)) / (locals.var_t6 * locals.var_t6)), (((((locals.var_ssi_dn2 * locals.var_swi) + (locals.var_ssi * locals.var_swi_dn2)) * locals.var_t6) - (assign33010_e43707 * locals.var_t6_dn2)) / (locals.var_t6 * locals.var_t6)), (((((locals.var_ssi_dn3 * locals.var_swi) + (locals.var_ssi * locals.var_swi_dn3)) * locals.var_t6) - (assign33010_e43707 * locals.var_t6_dn3)) / (locals.var_t6 * locals.var_t6)), (((((locals.var_ssi_dn4 * locals.var_swi) + (locals.var_ssi * locals.var_swi_dn4)) * locals.var_t6) - (assign33010_e43707 * locals.var_t6_dn4)) / (locals.var_t6 * locals.var_t6)), (((((locals.var_ssi_dn5 * locals.var_swi) + (locals.var_ssi * locals.var_swi_dn5)) * locals.var_t6) - (assign33010_e43707 * locals.var_t6_dn5)) / (locals.var_t6 * locals.var_t6)), (((((locals.var_ssi_dn6 * locals.var_swi) + (locals.var_ssi * locals.var_swi_dn6)) * locals.var_t6) - (assign33010_e43707 * locals.var_t6_dn6)) / (locals.var_t6 * locals.var_t6)), (((((locals.var_ssi_dn7 * locals.var_swi) + (locals.var_ssi * locals.var_swi_dn7)) * locals.var_t6) - (assign33010_e43707 * locals.var_t6_dn7)) / (locals.var_t6 * locals.var_t6)), (((((locals.var_ssi_dn8 * locals.var_swi) + (locals.var_ssi * locals.var_swi_dn8)) * locals.var_t6) - (assign33010_e43707 * locals.var_t6_dn8)) / (locals.var_t6 * locals.var_t6)), (((((locals.var_ssi_dn9 * locals.var_swi) + (locals.var_ssi * locals.var_swi_dn9)) * locals.var_t6) - (assign33010_e43707 * locals.var_t6_dn9)) / (locals.var_t6 * locals.var_t6)), (((((locals.var_ssi_dn10 * locals.var_swi) + (locals.var_ssi * locals.var_swi_dn10)) * locals.var_t6) - (assign33010_e43707 * locals.var_t6_dn10)) / (locals.var_t6 * locals.var_t6)), (((((locals.var_ssi_dn11 * locals.var_swi) + (locals.var_ssi * locals.var_swi_dn11)) * locals.var_t6) - (assign33010_e43707 * locals.var_t6_dn11)) / (locals.var_t6 * locals.var_t6)), (((((locals.var_ssi_dn12 * locals.var_swi) + (locals.var_ssi * locals.var_swi_dn12)) * locals.var_t6) - (assign33010_e43707 * locals.var_t6_dn12)) / (locals.var_t6 * locals.var_t6)), (((((locals.var_ssi_dn13 * locals.var_swi) + (locals.var_ssi * locals.var_swi_dn13)) * locals.var_t6) - (assign33010_e43707 * locals.var_t6_dn13)) / (locals.var_t6 * locals.var_t6)), (((((locals.var_ssi_dn14 * locals.var_swi) + (locals.var_ssi * locals.var_swi_dn14)) * locals.var_t6) - (assign33010_e43707 * locals.var_t6_dn14)) / (locals.var_t6 * locals.var_t6)),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn12, locals.var_t7_dn13, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign33010_e43711;
        locals.var_t7_dn0 = assign33010_e43711_d_n0;
        locals.var_t7_dn2 = assign33010_e43711_d_n2;
        locals.var_t7_dn3 = assign33010_e43711_d_n3;
        locals.var_t7_dn4 = assign33010_e43711_d_n4;
        locals.var_t7_dn5 = assign33010_e43711_d_n5;
        locals.var_t7_dn6 = assign33010_e43711_d_n6;
        locals.var_t7_dn7 = assign33010_e43711_d_n7;
        locals.var_t7_dn8 = assign33010_e43711_d_n8;
        locals.var_t7_dn9 = assign33010_e43711_d_n9;
        locals.var_t7_dn10 = assign33010_e43711_d_n10;
        locals.var_t7_dn11 = assign33010_e43711_d_n11;
        locals.var_t7_dn12 = assign33010_e43711_d_n12;
        locals.var_t7_dn13 = assign33010_e43711_d_n13;
        locals.var_t7_dn14 = assign33010_e43711_d_n14;
        locals.var_t7_rv = 0.0;

        let (assign33020_e43725, assign33020_e43725_d_n0, assign33020_e43725_d_n2, assign33020_e43725_d_n3, assign33020_e43725_d_n4, assign33020_e43725_d_n5, assign33020_e43725_d_n6, assign33020_e43725_d_n7, assign33020_e43725_d_n8, assign33020_e43725_d_n9, assign33020_e43725_d_n10, assign33020_e43725_d_n11, assign33020_e43725_d_n12, assign33020_e43725_d_n13, assign33020_e43725_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard745 != 0.0)) {
        let assign33020_e43719: f64 = (locals.var_qs_edge - locals.var_qdeff_edge);
        let assign33020_e43721: f64 = (assign33020_e43719).powf(p.p1064);
        let assign33020_e43722: f64 = (p.p1063 * assign33020_e43721);
        let assign33020_e43723: f64 = (1.0 + assign33020_e43722);
        (assign33020_e43723, (p.p1063 * if 0.0 == 0.0 && ((p.p1064) as f64).is_finite() && ((p.p1064) as f64).fract() == 0.0 { if p.p1064 == 0.0 { 0.0 } else { (p.p1064 * ((assign33020_e43719).powf(p.p1064 - 1.0) * (locals.var_qs_edge_dn0 - locals.var_qdeff_edge_dn0))) } } else { (assign33020_e43721 * (p.p1064 * ((locals.var_qs_edge_dn0 - locals.var_qdeff_edge_dn0) / assign33020_e43719))) }), (p.p1063 * if 0.0 == 0.0 && ((p.p1064) as f64).is_finite() && ((p.p1064) as f64).fract() == 0.0 { if p.p1064 == 0.0 { 0.0 } else { (p.p1064 * ((assign33020_e43719).powf(p.p1064 - 1.0) * (locals.var_qs_edge_dn2 - locals.var_qdeff_edge_dn2))) } } else { (assign33020_e43721 * (p.p1064 * ((locals.var_qs_edge_dn2 - locals.var_qdeff_edge_dn2) / assign33020_e43719))) }), (p.p1063 * if 0.0 == 0.0 && ((p.p1064) as f64).is_finite() && ((p.p1064) as f64).fract() == 0.0 { if p.p1064 == 0.0 { 0.0 } else { (p.p1064 * ((assign33020_e43719).powf(p.p1064 - 1.0) * (locals.var_qs_edge_dn3 - locals.var_qdeff_edge_dn3))) } } else { (assign33020_e43721 * (p.p1064 * ((locals.var_qs_edge_dn3 - locals.var_qdeff_edge_dn3) / assign33020_e43719))) }), (p.p1063 * if 0.0 == 0.0 && ((p.p1064) as f64).is_finite() && ((p.p1064) as f64).fract() == 0.0 { if p.p1064 == 0.0 { 0.0 } else { (p.p1064 * ((assign33020_e43719).powf(p.p1064 - 1.0) * (locals.var_qs_edge_dn4 - locals.var_qdeff_edge_dn4))) } } else { (assign33020_e43721 * (p.p1064 * ((locals.var_qs_edge_dn4 - locals.var_qdeff_edge_dn4) / assign33020_e43719))) }), (p.p1063 * if 0.0 == 0.0 && ((p.p1064) as f64).is_finite() && ((p.p1064) as f64).fract() == 0.0 { if p.p1064 == 0.0 { 0.0 } else { (p.p1064 * ((assign33020_e43719).powf(p.p1064 - 1.0) * (locals.var_qs_edge_dn5 - locals.var_qdeff_edge_dn5))) } } else { (assign33020_e43721 * (p.p1064 * ((locals.var_qs_edge_dn5 - locals.var_qdeff_edge_dn5) / assign33020_e43719))) }), (p.p1063 * if 0.0 == 0.0 && ((p.p1064) as f64).is_finite() && ((p.p1064) as f64).fract() == 0.0 { if p.p1064 == 0.0 { 0.0 } else { (p.p1064 * ((assign33020_e43719).powf(p.p1064 - 1.0) * (locals.var_qs_edge_dn6 - locals.var_qdeff_edge_dn6))) } } else { (assign33020_e43721 * (p.p1064 * ((locals.var_qs_edge_dn6 - locals.var_qdeff_edge_dn6) / assign33020_e43719))) }), (p.p1063 * if 0.0 == 0.0 && ((p.p1064) as f64).is_finite() && ((p.p1064) as f64).fract() == 0.0 { if p.p1064 == 0.0 { 0.0 } else { (p.p1064 * ((assign33020_e43719).powf(p.p1064 - 1.0) * (locals.var_qs_edge_dn7 - locals.var_qdeff_edge_dn7))) } } else { (assign33020_e43721 * (p.p1064 * ((locals.var_qs_edge_dn7 - locals.var_qdeff_edge_dn7) / assign33020_e43719))) }), (p.p1063 * if 0.0 == 0.0 && ((p.p1064) as f64).is_finite() && ((p.p1064) as f64).fract() == 0.0 { if p.p1064 == 0.0 { 0.0 } else { (p.p1064 * ((assign33020_e43719).powf(p.p1064 - 1.0) * (locals.var_qs_edge_dn8 - locals.var_qdeff_edge_dn8))) } } else { (assign33020_e43721 * (p.p1064 * ((locals.var_qs_edge_dn8 - locals.var_qdeff_edge_dn8) / assign33020_e43719))) }), (p.p1063 * if 0.0 == 0.0 && ((p.p1064) as f64).is_finite() && ((p.p1064) as f64).fract() == 0.0 { if p.p1064 == 0.0 { 0.0 } else { (p.p1064 * ((assign33020_e43719).powf(p.p1064 - 1.0) * (locals.var_qs_edge_dn9 - locals.var_qdeff_edge_dn9))) } } else { (assign33020_e43721 * (p.p1064 * ((locals.var_qs_edge_dn9 - locals.var_qdeff_edge_dn9) / assign33020_e43719))) }), (p.p1063 * if 0.0 == 0.0 && ((p.p1064) as f64).is_finite() && ((p.p1064) as f64).fract() == 0.0 { if p.p1064 == 0.0 { 0.0 } else { (p.p1064 * ((assign33020_e43719).powf(p.p1064 - 1.0) * (locals.var_qs_edge_dn10 - locals.var_qdeff_edge_dn10))) } } else { (assign33020_e43721 * (p.p1064 * ((locals.var_qs_edge_dn10 - locals.var_qdeff_edge_dn10) / assign33020_e43719))) }), (p.p1063 * if 0.0 == 0.0 && ((p.p1064) as f64).is_finite() && ((p.p1064) as f64).fract() == 0.0 { if p.p1064 == 0.0 { 0.0 } else { (p.p1064 * ((assign33020_e43719).powf(p.p1064 - 1.0) * (locals.var_qs_edge_dn11 - locals.var_qdeff_edge_dn11))) } } else { (assign33020_e43721 * (p.p1064 * ((locals.var_qs_edge_dn11 - locals.var_qdeff_edge_dn11) / assign33020_e43719))) }), (p.p1063 * if 0.0 == 0.0 && ((p.p1064) as f64).is_finite() && ((p.p1064) as f64).fract() == 0.0 { if p.p1064 == 0.0 { 0.0 } else { (p.p1064 * ((assign33020_e43719).powf(p.p1064 - 1.0) * (locals.var_qs_edge_dn12 - locals.var_qdeff_edge_dn12))) } } else { (assign33020_e43721 * (p.p1064 * ((locals.var_qs_edge_dn12 - locals.var_qdeff_edge_dn12) / assign33020_e43719))) }), (p.p1063 * if 0.0 == 0.0 && ((p.p1064) as f64).is_finite() && ((p.p1064) as f64).fract() == 0.0 { if p.p1064 == 0.0 { 0.0 } else { (p.p1064 * ((assign33020_e43719).powf(p.p1064 - 1.0) * (locals.var_qs_edge_dn13 - locals.var_qdeff_edge_dn13))) } } else { (assign33020_e43721 * (p.p1064 * ((locals.var_qs_edge_dn13 - locals.var_qdeff_edge_dn13) / assign33020_e43719))) }), (p.p1063 * if 0.0 == 0.0 && ((p.p1064) as f64).is_finite() && ((p.p1064) as f64).fract() == 0.0 { if p.p1064 == 0.0 { 0.0 } else { (p.p1064 * ((assign33020_e43719).powf(p.p1064 - 1.0) * (locals.var_qs_edge_dn14 - locals.var_qdeff_edge_dn14))) } } else { (assign33020_e43721 * (p.p1064 * ((locals.var_qs_edge_dn14 - locals.var_qdeff_edge_dn14) / assign33020_e43719))) }),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn3, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn12, locals.var_t8_dn13, locals.var_t8_dn14,)
    }
};
        locals.var_t8 = assign33020_e43725;
        locals.var_t8_dn0 = assign33020_e43725_d_n0;
        locals.var_t8_dn2 = assign33020_e43725_d_n2;
        locals.var_t8_dn3 = assign33020_e43725_d_n3;
        locals.var_t8_dn4 = assign33020_e43725_d_n4;
        locals.var_t8_dn5 = assign33020_e43725_d_n5;
        locals.var_t8_dn6 = assign33020_e43725_d_n6;
        locals.var_t8_dn7 = assign33020_e43725_d_n7;
        locals.var_t8_dn8 = assign33020_e43725_d_n8;
        locals.var_t8_dn9 = assign33020_e43725_d_n9;
        locals.var_t8_dn10 = assign33020_e43725_d_n10;
        locals.var_t8_dn11 = assign33020_e43725_d_n11;
        locals.var_t8_dn12 = assign33020_e43725_d_n12;
        locals.var_t8_dn13 = assign33020_e43725_d_n13;
        locals.var_t8_dn14 = assign33020_e43725_d_n14;
        locals.var_t8_rv = 0.0;

        let assign33060_e43756: f64 = if locals.var_sigvds > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard746 = assign33060_e43756;
        locals.var_guard746_rv = 0.0;

        let (assign33070_e43764, assign33070_e43764_d_n0, assign33070_e43764_d_n2, assign33070_e43764_d_n3, assign33070_e43764_d_n4, assign33070_e43764_d_n5, assign33070_e43764_d_n6, assign33070_e43764_d_n7, assign33070_e43764_d_n8, assign33070_e43764_d_n9, assign33070_e43764_d_n10, assign33070_e43764_d_n11, assign33070_e43764_d_n12, assign33070_e43764_d_n13, assign33070_e43764_d_n14,) = {
    if (locals.var_guard746 != 0.0) {
        let assign33070_e43760: f64 = (locals.var_devsign * p.p29);
        let assign33070_e43762: f64 = (assign33070_e43760 * locals.var_qsi);
        (assign33070_e43762, (assign33070_e43760 * locals.var_qsi_dn0), (assign33070_e43760 * locals.var_qsi_dn2), (assign33070_e43760 * locals.var_qsi_dn3), (assign33070_e43760 * locals.var_qsi_dn4), (assign33070_e43760 * locals.var_qsi_dn5), (assign33070_e43760 * locals.var_qsi_dn6), (assign33070_e43760 * locals.var_qsi_dn7), (assign33070_e43760 * locals.var_qsi_dn8), (assign33070_e43760 * locals.var_qsi_dn9), (assign33070_e43760 * locals.var_qsi_dn10), (assign33070_e43760 * locals.var_qsi_dn11), (assign33070_e43760 * locals.var_qsi_dn12), (assign33070_e43760 * locals.var_qsi_dn13), (assign33070_e43760 * locals.var_qsi_dn14),)
    } else {
        (locals.var_qsi_1, locals.var_qsi_1_dn0, locals.var_qsi_1_dn2, locals.var_qsi_1_dn3, locals.var_qsi_1_dn4, locals.var_qsi_1_dn5, locals.var_qsi_1_dn6, locals.var_qsi_1_dn7, locals.var_qsi_1_dn8, locals.var_qsi_1_dn9, locals.var_qsi_1_dn10, locals.var_qsi_1_dn11, locals.var_qsi_1_dn12, locals.var_qsi_1_dn13, locals.var_qsi_1_dn14,)
    }
};
        locals.var_qsi_1 = assign33070_e43764;
        locals.var_qsi_1_dn0 = assign33070_e43764_d_n0;
        locals.var_qsi_1_dn2 = assign33070_e43764_d_n2;
        locals.var_qsi_1_dn3 = assign33070_e43764_d_n3;
        locals.var_qsi_1_dn4 = assign33070_e43764_d_n4;
        locals.var_qsi_1_dn5 = assign33070_e43764_d_n5;
        locals.var_qsi_1_dn6 = assign33070_e43764_d_n6;
        locals.var_qsi_1_dn7 = assign33070_e43764_d_n7;
        locals.var_qsi_1_dn8 = assign33070_e43764_d_n8;
        locals.var_qsi_1_dn9 = assign33070_e43764_d_n9;
        locals.var_qsi_1_dn10 = assign33070_e43764_d_n10;
        locals.var_qsi_1_dn11 = assign33070_e43764_d_n11;
        locals.var_qsi_1_dn12 = assign33070_e43764_d_n12;
        locals.var_qsi_1_dn13 = assign33070_e43764_d_n13;
        locals.var_qsi_1_dn14 = assign33070_e43764_d_n14;
        locals.var_qsi_1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_108(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let (assign33080_e43772, assign33080_e43772_d_n0, assign33080_e43772_d_n2, assign33080_e43772_d_n3, assign33080_e43772_d_n4, assign33080_e43772_d_n5, assign33080_e43772_d_n6, assign33080_e43772_d_n7, assign33080_e43772_d_n8, assign33080_e43772_d_n9, assign33080_e43772_d_n10, assign33080_e43772_d_n11, assign33080_e43772_d_n12, assign33080_e43772_d_n13, assign33080_e43772_d_n14,) = {
    if (locals.var_guard746 != 0.0) {
        let assign33080_e43768: f64 = (locals.var_devsign * p.p29);
        let assign33080_e43770: f64 = (assign33080_e43768 * locals.var_qdi);
        (assign33080_e43770, (assign33080_e43768 * locals.var_qdi_dn0), (assign33080_e43768 * locals.var_qdi_dn2), (assign33080_e43768 * locals.var_qdi_dn3), (assign33080_e43768 * locals.var_qdi_dn4), (assign33080_e43768 * locals.var_qdi_dn5), (assign33080_e43768 * locals.var_qdi_dn6), (assign33080_e43768 * locals.var_qdi_dn7), (assign33080_e43768 * locals.var_qdi_dn8), (assign33080_e43768 * locals.var_qdi_dn9), (assign33080_e43768 * locals.var_qdi_dn10), (assign33080_e43768 * locals.var_qdi_dn11), (assign33080_e43768 * locals.var_qdi_dn12), (assign33080_e43768 * locals.var_qdi_dn13), (assign33080_e43768 * locals.var_qdi_dn14),)
    } else {
        (locals.var_qdi_1, locals.var_qdi_1_dn0, locals.var_qdi_1_dn2, locals.var_qdi_1_dn3, locals.var_qdi_1_dn4, locals.var_qdi_1_dn5, locals.var_qdi_1_dn6, locals.var_qdi_1_dn7, locals.var_qdi_1_dn8, locals.var_qdi_1_dn9, locals.var_qdi_1_dn10, locals.var_qdi_1_dn11, locals.var_qdi_1_dn12, locals.var_qdi_1_dn13, locals.var_qdi_1_dn14,)
    }
};
        locals.var_qdi_1 = assign33080_e43772;
        locals.var_qdi_1_dn0 = assign33080_e43772_d_n0;
        locals.var_qdi_1_dn2 = assign33080_e43772_d_n2;
        locals.var_qdi_1_dn3 = assign33080_e43772_d_n3;
        locals.var_qdi_1_dn4 = assign33080_e43772_d_n4;
        locals.var_qdi_1_dn5 = assign33080_e43772_d_n5;
        locals.var_qdi_1_dn6 = assign33080_e43772_d_n6;
        locals.var_qdi_1_dn7 = assign33080_e43772_d_n7;
        locals.var_qdi_1_dn8 = assign33080_e43772_d_n8;
        locals.var_qdi_1_dn9 = assign33080_e43772_d_n9;
        locals.var_qdi_1_dn10 = assign33080_e43772_d_n10;
        locals.var_qdi_1_dn11 = assign33080_e43772_d_n11;
        locals.var_qdi_1_dn12 = assign33080_e43772_d_n12;
        locals.var_qdi_1_dn13 = assign33080_e43772_d_n13;
        locals.var_qdi_1_dn14 = assign33080_e43772_d_n14;
        locals.var_qdi_1_rv = 0.0;

        let (assign33110_e43807, assign33110_e43807_d_n0, assign33110_e43807_d_n2, assign33110_e43807_d_n3, assign33110_e43807_d_n4, assign33110_e43807_d_n5, assign33110_e43807_d_n6, assign33110_e43807_d_n7, assign33110_e43807_d_n8, assign33110_e43807_d_n9, assign33110_e43807_d_n10, assign33110_e43807_d_n11, assign33110_e43807_d_n12, assign33110_e43807_d_n13, assign33110_e43807_d_n14,) = {
    if (locals.var_guard746 == 0.0) {
        let assign33110_e43803: f64 = (locals.var_devsign * p.p29);
        let assign33110_e43805: f64 = (assign33110_e43803 * locals.var_qdi);
        (assign33110_e43805, (assign33110_e43803 * locals.var_qdi_dn0), (assign33110_e43803 * locals.var_qdi_dn2), (assign33110_e43803 * locals.var_qdi_dn3), (assign33110_e43803 * locals.var_qdi_dn4), (assign33110_e43803 * locals.var_qdi_dn5), (assign33110_e43803 * locals.var_qdi_dn6), (assign33110_e43803 * locals.var_qdi_dn7), (assign33110_e43803 * locals.var_qdi_dn8), (assign33110_e43803 * locals.var_qdi_dn9), (assign33110_e43803 * locals.var_qdi_dn10), (assign33110_e43803 * locals.var_qdi_dn11), (assign33110_e43803 * locals.var_qdi_dn12), (assign33110_e43803 * locals.var_qdi_dn13), (assign33110_e43803 * locals.var_qdi_dn14),)
    } else {
        (locals.var_qsi_1, locals.var_qsi_1_dn0, locals.var_qsi_1_dn2, locals.var_qsi_1_dn3, locals.var_qsi_1_dn4, locals.var_qsi_1_dn5, locals.var_qsi_1_dn6, locals.var_qsi_1_dn7, locals.var_qsi_1_dn8, locals.var_qsi_1_dn9, locals.var_qsi_1_dn10, locals.var_qsi_1_dn11, locals.var_qsi_1_dn12, locals.var_qsi_1_dn13, locals.var_qsi_1_dn14,)
    }
};
        locals.var_qsi_1 = assign33110_e43807;
        locals.var_qsi_1_dn0 = assign33110_e43807_d_n0;
        locals.var_qsi_1_dn2 = assign33110_e43807_d_n2;
        locals.var_qsi_1_dn3 = assign33110_e43807_d_n3;
        locals.var_qsi_1_dn4 = assign33110_e43807_d_n4;
        locals.var_qsi_1_dn5 = assign33110_e43807_d_n5;
        locals.var_qsi_1_dn6 = assign33110_e43807_d_n6;
        locals.var_qsi_1_dn7 = assign33110_e43807_d_n7;
        locals.var_qsi_1_dn8 = assign33110_e43807_d_n8;
        locals.var_qsi_1_dn9 = assign33110_e43807_d_n9;
        locals.var_qsi_1_dn10 = assign33110_e43807_d_n10;
        locals.var_qsi_1_dn11 = assign33110_e43807_d_n11;
        locals.var_qsi_1_dn12 = assign33110_e43807_d_n12;
        locals.var_qsi_1_dn13 = assign33110_e43807_d_n13;
        locals.var_qsi_1_dn14 = assign33110_e43807_d_n14;
        locals.var_qsi_1_rv = 0.0;

        let (assign33120_e43816, assign33120_e43816_d_n0, assign33120_e43816_d_n2, assign33120_e43816_d_n3, assign33120_e43816_d_n4, assign33120_e43816_d_n5, assign33120_e43816_d_n6, assign33120_e43816_d_n7, assign33120_e43816_d_n8, assign33120_e43816_d_n9, assign33120_e43816_d_n10, assign33120_e43816_d_n11, assign33120_e43816_d_n12, assign33120_e43816_d_n13, assign33120_e43816_d_n14,) = {
    if (locals.var_guard746 == 0.0) {
        let assign33120_e43812: f64 = (locals.var_devsign * p.p29);
        let assign33120_e43814: f64 = (assign33120_e43812 * locals.var_qsi);
        (assign33120_e43814, (assign33120_e43812 * locals.var_qsi_dn0), (assign33120_e43812 * locals.var_qsi_dn2), (assign33120_e43812 * locals.var_qsi_dn3), (assign33120_e43812 * locals.var_qsi_dn4), (assign33120_e43812 * locals.var_qsi_dn5), (assign33120_e43812 * locals.var_qsi_dn6), (assign33120_e43812 * locals.var_qsi_dn7), (assign33120_e43812 * locals.var_qsi_dn8), (assign33120_e43812 * locals.var_qsi_dn9), (assign33120_e43812 * locals.var_qsi_dn10), (assign33120_e43812 * locals.var_qsi_dn11), (assign33120_e43812 * locals.var_qsi_dn12), (assign33120_e43812 * locals.var_qsi_dn13), (assign33120_e43812 * locals.var_qsi_dn14),)
    } else {
        (locals.var_qdi_1, locals.var_qdi_1_dn0, locals.var_qdi_1_dn2, locals.var_qdi_1_dn3, locals.var_qdi_1_dn4, locals.var_qdi_1_dn5, locals.var_qdi_1_dn6, locals.var_qdi_1_dn7, locals.var_qdi_1_dn8, locals.var_qdi_1_dn9, locals.var_qdi_1_dn10, locals.var_qdi_1_dn11, locals.var_qdi_1_dn12, locals.var_qdi_1_dn13, locals.var_qdi_1_dn14,)
    }
};
        locals.var_qdi_1 = assign33120_e43816;
        locals.var_qdi_1_dn0 = assign33120_e43816_d_n0;
        locals.var_qdi_1_dn2 = assign33120_e43816_d_n2;
        locals.var_qdi_1_dn3 = assign33120_e43816_d_n3;
        locals.var_qdi_1_dn4 = assign33120_e43816_d_n4;
        locals.var_qdi_1_dn5 = assign33120_e43816_d_n5;
        locals.var_qdi_1_dn6 = assign33120_e43816_d_n6;
        locals.var_qdi_1_dn7 = assign33120_e43816_d_n7;
        locals.var_qdi_1_dn8 = assign33120_e43816_d_n8;
        locals.var_qdi_1_dn9 = assign33120_e43816_d_n9;
        locals.var_qdi_1_dn10 = assign33120_e43816_d_n10;
        locals.var_qdi_1_dn11 = assign33120_e43816_d_n11;
        locals.var_qdi_1_dn12 = assign33120_e43816_d_n12;
        locals.var_qdi_1_dn13 = assign33120_e43816_d_n13;
        locals.var_qdi_1_dn14 = assign33120_e43816_d_n14;
        locals.var_qdi_1_rv = 0.0;

        let assign33160_e43858: f64 = if ((p.p1094 == 1.0) && (p.p1095 == 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard747 = assign33160_e43858;
        locals.var_guard747_rv = 0.0;

        let (assign33170_e43864, assign33170_e43864_d_n0, assign33170_e43864_d_n2, assign33170_e43864_d_n3, assign33170_e43864_d_n4, assign33170_e43864_d_n5, assign33170_e43864_d_n6, assign33170_e43864_d_n7, assign33170_e43864_d_n8, assign33170_e43864_d_n9, assign33170_e43864_d_n10, assign33170_e43864_d_n11, assign33170_e43864_d_n12, assign33170_e43864_d_n13, assign33170_e43864_d_n14,) = {
    if (locals.var_guard747 != 0.0) {
        let assign33170_e43862: f64 = (locals.var_qovb + locals.var_qiov);
        (assign33170_e43862, (locals.var_qovb_dn0 + locals.var_qiov_dn0), (locals.var_qovb_dn2 + locals.var_qiov_dn2), (locals.var_qovb_dn3 + locals.var_qiov_dn3), (locals.var_qovb_dn4 + locals.var_qiov_dn4), (locals.var_qovb_dn5 + locals.var_qiov_dn5), (locals.var_qovb_dn6 + locals.var_qiov_dn6), (locals.var_qovb_dn7 + locals.var_qiov_dn7), (locals.var_qovb_dn8 + locals.var_qiov_dn8), (locals.var_qovb_dn9 + locals.var_qiov_dn9), (locals.var_qovb_dn10 + locals.var_qiov_dn10), (locals.var_qovb_dn11 + locals.var_qiov_dn11), (locals.var_qovb_dn12 + locals.var_qiov_dn12), (locals.var_qovb_dn13 + locals.var_qiov_dn13), (locals.var_qovb_dn14 + locals.var_qiov_dn14),)
    } else {
        (locals.var_qovb, locals.var_qovb_dn0, locals.var_qovb_dn2, locals.var_qovb_dn3, locals.var_qovb_dn4, locals.var_qovb_dn5, locals.var_qovb_dn6, locals.var_qovb_dn7, locals.var_qovb_dn8, locals.var_qovb_dn9, locals.var_qovb_dn10, locals.var_qovb_dn11, locals.var_qovb_dn12, locals.var_qovb_dn13, locals.var_qovb_dn14,)
    }
};
        locals.var_qovb = assign33170_e43864;
        locals.var_qovb_dn0 = assign33170_e43864_d_n0;
        locals.var_qovb_dn2 = assign33170_e43864_d_n2;
        locals.var_qovb_dn3 = assign33170_e43864_d_n3;
        locals.var_qovb_dn4 = assign33170_e43864_d_n4;
        locals.var_qovb_dn5 = assign33170_e43864_d_n5;
        locals.var_qovb_dn6 = assign33170_e43864_d_n6;
        locals.var_qovb_dn7 = assign33170_e43864_d_n7;
        locals.var_qovb_dn8 = assign33170_e43864_d_n8;
        locals.var_qovb_dn9 = assign33170_e43864_d_n9;
        locals.var_qovb_dn10 = assign33170_e43864_d_n10;
        locals.var_qovb_dn11 = assign33170_e43864_d_n11;
        locals.var_qovb_dn12 = assign33170_e43864_d_n12;
        locals.var_qovb_dn13 = assign33170_e43864_d_n13;
        locals.var_qovb_dn14 = assign33170_e43864_d_n14;
        locals.var_qovb_rv = 0.0;

        let (assign33180_e43870, assign33180_e43870_d_n0, assign33180_e43870_d_n2, assign33180_e43870_d_n3, assign33180_e43870_d_n4, assign33180_e43870_d_n5, assign33180_e43870_d_n6, assign33180_e43870_d_n7, assign33180_e43870_d_n8, assign33180_e43870_d_n9, assign33180_e43870_d_n10, assign33180_e43870_d_n11, assign33180_e43870_d_n12, assign33180_e43870_d_n13, assign33180_e43870_d_n14,) = {
    if (locals.var_guard747 != 0.0) {
        let assign33180_e43868: f64 = (locals.var_qovd + locals.var_qbov);
        (assign33180_e43868, (locals.var_qovd_dn0 + locals.var_qbov_dn0), (locals.var_qovd_dn2 + locals.var_qbov_dn2), (locals.var_qovd_dn3 + locals.var_qbov_dn3), (locals.var_qovd_dn4 + locals.var_qbov_dn4), (locals.var_qovd_dn5 + locals.var_qbov_dn5), (locals.var_qovd_dn6 + locals.var_qbov_dn6), (locals.var_qovd_dn7 + locals.var_qbov_dn7), (locals.var_qovd_dn8 + locals.var_qbov_dn8), (locals.var_qovd_dn9 + locals.var_qbov_dn9), (locals.var_qovd_dn10 + locals.var_qbov_dn10), (locals.var_qovd_dn11 + locals.var_qbov_dn11), (locals.var_qovd_dn12 + locals.var_qbov_dn12), (locals.var_qovd_dn13 + locals.var_qbov_dn13), (locals.var_qovd_dn14 + locals.var_qbov_dn14),)
    } else {
        (locals.var_qovd, locals.var_qovd_dn0, locals.var_qovd_dn2, locals.var_qovd_dn3, locals.var_qovd_dn4, locals.var_qovd_dn5, locals.var_qovd_dn6, locals.var_qovd_dn7, locals.var_qovd_dn8, locals.var_qovd_dn9, locals.var_qovd_dn10, locals.var_qovd_dn11, locals.var_qovd_dn12, locals.var_qovd_dn13, locals.var_qovd_dn14,)
    }
};
        locals.var_qovd = assign33180_e43870;
        locals.var_qovd_dn0 = assign33180_e43870_d_n0;
        locals.var_qovd_dn2 = assign33180_e43870_d_n2;
        locals.var_qovd_dn3 = assign33180_e43870_d_n3;
        locals.var_qovd_dn4 = assign33180_e43870_d_n4;
        locals.var_qovd_dn5 = assign33180_e43870_d_n5;
        locals.var_qovd_dn6 = assign33180_e43870_d_n6;
        locals.var_qovd_dn7 = assign33180_e43870_d_n7;
        locals.var_qovd_dn8 = assign33180_e43870_d_n8;
        locals.var_qovd_dn9 = assign33180_e43870_d_n9;
        locals.var_qovd_dn10 = assign33180_e43870_d_n10;
        locals.var_qovd_dn11 = assign33180_e43870_d_n11;
        locals.var_qovd_dn12 = assign33180_e43870_d_n12;
        locals.var_qovd_dn13 = assign33180_e43870_d_n13;
        locals.var_qovd_dn14 = assign33180_e43870_d_n14;
        locals.var_qovd_rv = 0.0;

        let assign33190_e43873: f64 = if p.p1096 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard748 = assign33190_e43873;
        locals.var_guard748_rv = 0.0;

        let (assign33200_e43881, assign33200_e43881_d_n0, assign33200_e43881_d_n2, assign33200_e43881_d_n3, assign33200_e43881_d_n4, assign33200_e43881_d_n5, assign33200_e43881_d_n6, assign33200_e43881_d_n7, assign33200_e43881_d_n8, assign33200_e43881_d_n9, assign33200_e43881_d_n10, assign33200_e43881_d_n11, assign33200_e43881_d_n12, assign33200_e43881_d_n13, assign33200_e43881_d_n14,) = {
    if ((locals.var_guard747 != 0.0) && (locals.var_guard748 != 0.0)) {
        let assign33200_e43879: f64 = (locals.var_qovb + locals.var_qiovs);
        (assign33200_e43879, (locals.var_qovb_dn0 + locals.var_qiovs_dn0), (locals.var_qovb_dn2 + locals.var_qiovs_dn2), (locals.var_qovb_dn3 + locals.var_qiovs_dn3), (locals.var_qovb_dn4 + locals.var_qiovs_dn4), (locals.var_qovb_dn5 + locals.var_qiovs_dn5), (locals.var_qovb_dn6 + locals.var_qiovs_dn6), (locals.var_qovb_dn7 + locals.var_qiovs_dn7), (locals.var_qovb_dn8 + locals.var_qiovs_dn8), (locals.var_qovb_dn9 + locals.var_qiovs_dn9), (locals.var_qovb_dn10 + locals.var_qiovs_dn10), (locals.var_qovb_dn11 + locals.var_qiovs_dn11), (locals.var_qovb_dn12 + locals.var_qiovs_dn12), (locals.var_qovb_dn13 + locals.var_qiovs_dn13), (locals.var_qovb_dn14 + locals.var_qiovs_dn14),)
    } else {
        (locals.var_qovb, locals.var_qovb_dn0, locals.var_qovb_dn2, locals.var_qovb_dn3, locals.var_qovb_dn4, locals.var_qovb_dn5, locals.var_qovb_dn6, locals.var_qovb_dn7, locals.var_qovb_dn8, locals.var_qovb_dn9, locals.var_qovb_dn10, locals.var_qovb_dn11, locals.var_qovb_dn12, locals.var_qovb_dn13, locals.var_qovb_dn14,)
    }
};
        locals.var_qovb = assign33200_e43881;
        locals.var_qovb_dn0 = assign33200_e43881_d_n0;
        locals.var_qovb_dn2 = assign33200_e43881_d_n2;
        locals.var_qovb_dn3 = assign33200_e43881_d_n3;
        locals.var_qovb_dn4 = assign33200_e43881_d_n4;
        locals.var_qovb_dn5 = assign33200_e43881_d_n5;
        locals.var_qovb_dn6 = assign33200_e43881_d_n6;
        locals.var_qovb_dn7 = assign33200_e43881_d_n7;
        locals.var_qovb_dn8 = assign33200_e43881_d_n8;
        locals.var_qovb_dn9 = assign33200_e43881_d_n9;
        locals.var_qovb_dn10 = assign33200_e43881_d_n10;
        locals.var_qovb_dn11 = assign33200_e43881_d_n11;
        locals.var_qovb_dn12 = assign33200_e43881_d_n12;
        locals.var_qovb_dn13 = assign33200_e43881_d_n13;
        locals.var_qovb_dn14 = assign33200_e43881_d_n14;
        locals.var_qovb_rv = 0.0;

        let (assign33210_e43889, assign33210_e43889_d_n0, assign33210_e43889_d_n2, assign33210_e43889_d_n3, assign33210_e43889_d_n4, assign33210_e43889_d_n5, assign33210_e43889_d_n6, assign33210_e43889_d_n7, assign33210_e43889_d_n8, assign33210_e43889_d_n9, assign33210_e43889_d_n10, assign33210_e43889_d_n11, assign33210_e43889_d_n12, assign33210_e43889_d_n13, assign33210_e43889_d_n14,) = {
    if ((locals.var_guard747 != 0.0) && (locals.var_guard748 != 0.0)) {
        let assign33210_e43887: f64 = (locals.var_qovs + locals.var_qbovs);
        (assign33210_e43887, (locals.var_qovs_dn0 + locals.var_qbovs_dn0), (locals.var_qovs_dn2 + locals.var_qbovs_dn2), (locals.var_qovs_dn3 + locals.var_qbovs_dn3), (locals.var_qovs_dn4 + locals.var_qbovs_dn4), (locals.var_qovs_dn5 + locals.var_qbovs_dn5), (locals.var_qovs_dn6 + locals.var_qbovs_dn6), (locals.var_qovs_dn7 + locals.var_qbovs_dn7), (locals.var_qovs_dn8 + locals.var_qbovs_dn8), (locals.var_qovs_dn9 + locals.var_qbovs_dn9), (locals.var_qovs_dn10 + locals.var_qbovs_dn10), (locals.var_qovs_dn11 + locals.var_qbovs_dn11), (locals.var_qovs_dn12 + locals.var_qbovs_dn12), (locals.var_qovs_dn13 + locals.var_qbovs_dn13), (locals.var_qovs_dn14 + locals.var_qbovs_dn14),)
    } else {
        (locals.var_qovs, locals.var_qovs_dn0, locals.var_qovs_dn2, locals.var_qovs_dn3, locals.var_qovs_dn4, locals.var_qovs_dn5, locals.var_qovs_dn6, locals.var_qovs_dn7, locals.var_qovs_dn8, locals.var_qovs_dn9, locals.var_qovs_dn10, locals.var_qovs_dn11, locals.var_qovs_dn12, locals.var_qovs_dn13, locals.var_qovs_dn14,)
    }
};
        locals.var_qovs = assign33210_e43889;
        locals.var_qovs_dn0 = assign33210_e43889_d_n0;
        locals.var_qovs_dn2 = assign33210_e43889_d_n2;
        locals.var_qovs_dn3 = assign33210_e43889_d_n3;
        locals.var_qovs_dn4 = assign33210_e43889_d_n4;
        locals.var_qovs_dn5 = assign33210_e43889_d_n5;
        locals.var_qovs_dn6 = assign33210_e43889_d_n6;
        locals.var_qovs_dn7 = assign33210_e43889_d_n7;
        locals.var_qovs_dn8 = assign33210_e43889_d_n8;
        locals.var_qovs_dn9 = assign33210_e43889_d_n9;
        locals.var_qovs_dn10 = assign33210_e43889_d_n10;
        locals.var_qovs_dn11 = assign33210_e43889_d_n11;
        locals.var_qovs_dn12 = assign33210_e43889_d_n12;
        locals.var_qovs_dn13 = assign33210_e43889_d_n13;
        locals.var_qovs_dn14 = assign33210_e43889_d_n14;
        locals.var_qovs_rv = 0.0;

        let assign33230_e43897: f64 = (locals.var_devsign * p.p29);
        let assign33230_e43899: f64 = (assign33230_e43897 * locals.var_qgi);
        locals.var_qgi_1 = assign33230_e43899;
        locals.var_qgi_1_dn0 = (assign33230_e43897 * locals.var_qgi_dn0);
        locals.var_qgi_1_dn2 = (assign33230_e43897 * locals.var_qgi_dn2);
        locals.var_qgi_1_dn3 = (assign33230_e43897 * locals.var_qgi_dn3);
        locals.var_qgi_1_dn4 = (assign33230_e43897 * locals.var_qgi_dn4);
        locals.var_qgi_1_dn5 = (assign33230_e43897 * locals.var_qgi_dn5);
        locals.var_qgi_1_dn6 = (assign33230_e43897 * locals.var_qgi_dn6);
        locals.var_qgi_1_dn7 = (assign33230_e43897 * locals.var_qgi_dn7);
        locals.var_qgi_1_dn8 = (assign33230_e43897 * locals.var_qgi_dn8);
        locals.var_qgi_1_dn9 = (assign33230_e43897 * locals.var_qgi_dn9);
        locals.var_qgi_1_dn10 = (assign33230_e43897 * locals.var_qgi_dn10);
        locals.var_qgi_1_dn11 = (assign33230_e43897 * locals.var_qgi_dn11);
        locals.var_qgi_1_dn12 = (assign33230_e43897 * locals.var_qgi_dn12);
        locals.var_qgi_1_dn13 = (assign33230_e43897 * locals.var_qgi_dn13);
        locals.var_qgi_1_dn14 = (assign33230_e43897 * locals.var_qgi_dn14);
        locals.var_qgi_1_rv = 0.0;

        let assign33870_e44253: f64 = if ((p.p42 != 2.0) && (locals.var_rdraingeo > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard754 = assign33870_e44253;
        locals.var_guard754_rv = 0.0;

        let (assign33880_e44259, assign33880_e44259_d_n0, assign33880_e44259_d_n2, assign33880_e44259_d_n3, assign33880_e44259_d_n4, assign33880_e44259_d_n5, assign33880_e44259_d_n6, assign33880_e44259_d_n7, assign33880_e44259_d_n8, assign33880_e44259_d_n9, assign33880_e44259_d_n10, assign33880_e44259_d_n11, assign33880_e44259_d_n12, assign33880_e44259_d_n13, assign33880_e44259_d_n14,) = {
    if (locals.var_guard754 != 0.0) {
        let assign33880_e44257: f64 = (1.0 / locals.var_rdrain);
        (assign33880_e44257, (-(locals.var_rdrain_dn0 / (locals.var_rdrain * locals.var_rdrain))), (-(locals.var_rdrain_dn2 / (locals.var_rdrain * locals.var_rdrain))), (-(locals.var_rdrain_dn3 / (locals.var_rdrain * locals.var_rdrain))), (-(locals.var_rdrain_dn4 / (locals.var_rdrain * locals.var_rdrain))), (-(locals.var_rdrain_dn5 / (locals.var_rdrain * locals.var_rdrain))), (-(locals.var_rdrain_dn6 / (locals.var_rdrain * locals.var_rdrain))), (-(locals.var_rdrain_dn7 / (locals.var_rdrain * locals.var_rdrain))), (-(locals.var_rdrain_dn8 / (locals.var_rdrain * locals.var_rdrain))), (-(locals.var_rdrain_dn9 / (locals.var_rdrain * locals.var_rdrain))), (-(locals.var_rdrain_dn10 / (locals.var_rdrain * locals.var_rdrain))), (-(locals.var_rdrain_dn11 / (locals.var_rdrain * locals.var_rdrain))), (-(locals.var_rdrain_dn12 / (locals.var_rdrain * locals.var_rdrain))), (-(locals.var_rdrain_dn13 / (locals.var_rdrain * locals.var_rdrain))), (-(locals.var_rdrain_dn14 / (locals.var_rdrain * locals.var_rdrain))),)
    } else {
        (locals.var_gdpr, locals.var_gdpr_dn0, locals.var_gdpr_dn2, locals.var_gdpr_dn3, locals.var_gdpr_dn4, locals.var_gdpr_dn5, locals.var_gdpr_dn6, locals.var_gdpr_dn7, locals.var_gdpr_dn8, locals.var_gdpr_dn9, locals.var_gdpr_dn10, locals.var_gdpr_dn11, locals.var_gdpr_dn12, locals.var_gdpr_dn13, locals.var_gdpr_dn14,)
    }
};
        locals.var_gdpr = assign33880_e44259;
        locals.var_gdpr_dn0 = assign33880_e44259_d_n0;
        locals.var_gdpr_dn2 = assign33880_e44259_d_n2;
        locals.var_gdpr_dn3 = assign33880_e44259_d_n3;
        locals.var_gdpr_dn4 = assign33880_e44259_d_n4;
        locals.var_gdpr_dn5 = assign33880_e44259_d_n5;
        locals.var_gdpr_dn6 = assign33880_e44259_d_n6;
        locals.var_gdpr_dn7 = assign33880_e44259_d_n7;
        locals.var_gdpr_dn8 = assign33880_e44259_d_n8;
        locals.var_gdpr_dn9 = assign33880_e44259_d_n9;
        locals.var_gdpr_dn10 = assign33880_e44259_d_n10;
        locals.var_gdpr_dn11 = assign33880_e44259_d_n11;
        locals.var_gdpr_dn12 = assign33880_e44259_d_n12;
        locals.var_gdpr_dn13 = assign33880_e44259_d_n13;
        locals.var_gdpr_dn14 = assign33880_e44259_d_n14;
        locals.var_gdpr_rv = 0.0;

        let assign33890_e44270: f64 = if (((p.p42 == 1.0) && (p.p1094 == 1.0)) && (p.p1110 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard755 = assign33890_e44270;
        locals.var_guard755_rv = 0.0;

        let (assign33900_e44278, assign33900_e44278_d_n0, assign33900_e44278_d_n2, assign33900_e44278_d_n3, assign33900_e44278_d_n4, assign33900_e44278_d_n5, assign33900_e44278_d_n6, assign33900_e44278_d_n7, assign33900_e44278_d_n8, assign33900_e44278_d_n9, assign33900_e44278_d_n10, assign33900_e44278_d_n11, assign33900_e44278_d_n12, assign33900_e44278_d_n13, assign33900_e44278_d_n14,) = {
    if ((locals.var_guard754 != 0.0) && (locals.var_guard755 != 0.0)) {
        let assign33900_e44276: f64 = (1.0 / locals.var_rdrift_d);
        (assign33900_e44276, (-(locals.var_rdrift_d_dn0 / (locals.var_rdrift_d * locals.var_rdrift_d))), (-(locals.var_rdrift_d_dn2 / (locals.var_rdrift_d * locals.var_rdrift_d))), (-(locals.var_rdrift_d_dn3 / (locals.var_rdrift_d * locals.var_rdrift_d))), (-(locals.var_rdrift_d_dn4 / (locals.var_rdrift_d * locals.var_rdrift_d))), (-(locals.var_rdrift_d_dn5 / (locals.var_rdrift_d * locals.var_rdrift_d))), (-(locals.var_rdrift_d_dn6 / (locals.var_rdrift_d * locals.var_rdrift_d))), (-(locals.var_rdrift_d_dn7 / (locals.var_rdrift_d * locals.var_rdrift_d))), (-(locals.var_rdrift_d_dn8 / (locals.var_rdrift_d * locals.var_rdrift_d))), (-(locals.var_rdrift_d_dn9 / (locals.var_rdrift_d * locals.var_rdrift_d))), (-(locals.var_rdrift_d_dn10 / (locals.var_rdrift_d * locals.var_rdrift_d))), (-(locals.var_rdrift_d_dn11 / (locals.var_rdrift_d * locals.var_rdrift_d))), (-(locals.var_rdrift_d_dn12 / (locals.var_rdrift_d * locals.var_rdrift_d))), (-(locals.var_rdrift_d_dn13 / (locals.var_rdrift_d * locals.var_rdrift_d))), (-(locals.var_rdrift_d_dn14 / (locals.var_rdrift_d * locals.var_rdrift_d))),)
    } else {
        (locals.var_gdrift_d, locals.var_gdrift_d_dn0, locals.var_gdrift_d_dn2, locals.var_gdrift_d_dn3, locals.var_gdrift_d_dn4, locals.var_gdrift_d_dn5, locals.var_gdrift_d_dn6, locals.var_gdrift_d_dn7, locals.var_gdrift_d_dn8, locals.var_gdrift_d_dn9, locals.var_gdrift_d_dn10, locals.var_gdrift_d_dn11, locals.var_gdrift_d_dn12, locals.var_gdrift_d_dn13, locals.var_gdrift_d_dn14,)
    }
};
        locals.var_gdrift_d = assign33900_e44278;
        locals.var_gdrift_d_dn0 = assign33900_e44278_d_n0;
        locals.var_gdrift_d_dn2 = assign33900_e44278_d_n2;
        locals.var_gdrift_d_dn3 = assign33900_e44278_d_n3;
        locals.var_gdrift_d_dn4 = assign33900_e44278_d_n4;
        locals.var_gdrift_d_dn5 = assign33900_e44278_d_n5;
        locals.var_gdrift_d_dn6 = assign33900_e44278_d_n6;
        locals.var_gdrift_d_dn7 = assign33900_e44278_d_n7;
        locals.var_gdrift_d_dn8 = assign33900_e44278_d_n8;
        locals.var_gdrift_d_dn9 = assign33900_e44278_d_n9;
        locals.var_gdrift_d_dn10 = assign33900_e44278_d_n10;
        locals.var_gdrift_d_dn11 = assign33900_e44278_d_n11;
        locals.var_gdrift_d_dn12 = assign33900_e44278_d_n12;
        locals.var_gdrift_d_dn13 = assign33900_e44278_d_n13;
        locals.var_gdrift_d_dn14 = assign33900_e44278_d_n14;
        locals.var_gdrift_d_rv = 0.0;

        let assign33910_e44285: f64 = if ((p.p42 != 2.0) && (locals.var_rsourcegeo > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard756 = assign33910_e44285;
        locals.var_guard756_rv = 0.0;

        let (assign33920_e44291, assign33920_e44291_d_n0, assign33920_e44291_d_n2, assign33920_e44291_d_n3, assign33920_e44291_d_n4, assign33920_e44291_d_n5, assign33920_e44291_d_n6, assign33920_e44291_d_n7, assign33920_e44291_d_n8, assign33920_e44291_d_n9, assign33920_e44291_d_n10, assign33920_e44291_d_n11, assign33920_e44291_d_n12, assign33920_e44291_d_n13, assign33920_e44291_d_n14,) = {
    if (locals.var_guard756 != 0.0) {
        let assign33920_e44289: f64 = (1.0 / locals.var_rsource);
        (assign33920_e44289, (-(locals.var_rsource_dn0 / (locals.var_rsource * locals.var_rsource))), (-(locals.var_rsource_dn2 / (locals.var_rsource * locals.var_rsource))), (-(locals.var_rsource_dn3 / (locals.var_rsource * locals.var_rsource))), (-(locals.var_rsource_dn4 / (locals.var_rsource * locals.var_rsource))), (-(locals.var_rsource_dn5 / (locals.var_rsource * locals.var_rsource))), (-(locals.var_rsource_dn6 / (locals.var_rsource * locals.var_rsource))), (-(locals.var_rsource_dn7 / (locals.var_rsource * locals.var_rsource))), (-(locals.var_rsource_dn8 / (locals.var_rsource * locals.var_rsource))), (-(locals.var_rsource_dn9 / (locals.var_rsource * locals.var_rsource))), (-(locals.var_rsource_dn10 / (locals.var_rsource * locals.var_rsource))), (-(locals.var_rsource_dn11 / (locals.var_rsource * locals.var_rsource))), (-(locals.var_rsource_dn12 / (locals.var_rsource * locals.var_rsource))), (-(locals.var_rsource_dn13 / (locals.var_rsource * locals.var_rsource))), (-(locals.var_rsource_dn14 / (locals.var_rsource * locals.var_rsource))),)
    } else {
        (locals.var_gspr, locals.var_gspr_dn0, locals.var_gspr_dn2, locals.var_gspr_dn3, locals.var_gspr_dn4, locals.var_gspr_dn5, locals.var_gspr_dn6, locals.var_gspr_dn7, locals.var_gspr_dn8, locals.var_gspr_dn9, locals.var_gspr_dn10, locals.var_gspr_dn11, locals.var_gspr_dn12, locals.var_gspr_dn13, locals.var_gspr_dn14,)
    }
};
        locals.var_gspr = assign33920_e44291;
        locals.var_gspr_dn0 = assign33920_e44291_d_n0;
        locals.var_gspr_dn2 = assign33920_e44291_d_n2;
        locals.var_gspr_dn3 = assign33920_e44291_d_n3;
        locals.var_gspr_dn4 = assign33920_e44291_d_n4;
        locals.var_gspr_dn5 = assign33920_e44291_d_n5;
        locals.var_gspr_dn6 = assign33920_e44291_d_n6;
        locals.var_gspr_dn7 = assign33920_e44291_d_n7;
        locals.var_gspr_dn8 = assign33920_e44291_d_n8;
        locals.var_gspr_dn9 = assign33920_e44291_d_n9;
        locals.var_gspr_dn10 = assign33920_e44291_d_n10;
        locals.var_gspr_dn11 = assign33920_e44291_d_n11;
        locals.var_gspr_dn12 = assign33920_e44291_d_n12;
        locals.var_gspr_dn13 = assign33920_e44291_d_n13;
        locals.var_gspr_dn14 = assign33920_e44291_d_n14;
        locals.var_gspr_rv = 0.0;

        let assign33930_e44302: f64 = if (((p.p42 == 1.0) && (p.p1094 == 1.0)) && (p.p1112 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard757 = assign33930_e44302;
        locals.var_guard757_rv = 0.0;

        let (assign33940_e44310, assign33940_e44310_d_n0, assign33940_e44310_d_n2, assign33940_e44310_d_n3, assign33940_e44310_d_n4, assign33940_e44310_d_n5, assign33940_e44310_d_n6, assign33940_e44310_d_n7, assign33940_e44310_d_n8, assign33940_e44310_d_n9, assign33940_e44310_d_n10, assign33940_e44310_d_n11, assign33940_e44310_d_n12, assign33940_e44310_d_n13, assign33940_e44310_d_n14,) = {
    if ((locals.var_guard756 != 0.0) && (locals.var_guard757 != 0.0)) {
        let assign33940_e44308: f64 = (1.0 / locals.var_rdrift_s);
        (assign33940_e44308, (-(locals.var_rdrift_s_dn0 / (locals.var_rdrift_s * locals.var_rdrift_s))), (-(locals.var_rdrift_s_dn2 / (locals.var_rdrift_s * locals.var_rdrift_s))), (-(locals.var_rdrift_s_dn3 / (locals.var_rdrift_s * locals.var_rdrift_s))), (-(locals.var_rdrift_s_dn4 / (locals.var_rdrift_s * locals.var_rdrift_s))), (-(locals.var_rdrift_s_dn5 / (locals.var_rdrift_s * locals.var_rdrift_s))), (-(locals.var_rdrift_s_dn6 / (locals.var_rdrift_s * locals.var_rdrift_s))), (-(locals.var_rdrift_s_dn7 / (locals.var_rdrift_s * locals.var_rdrift_s))), (-(locals.var_rdrift_s_dn8 / (locals.var_rdrift_s * locals.var_rdrift_s))), (-(locals.var_rdrift_s_dn9 / (locals.var_rdrift_s * locals.var_rdrift_s))), (-(locals.var_rdrift_s_dn10 / (locals.var_rdrift_s * locals.var_rdrift_s))), (-(locals.var_rdrift_s_dn11 / (locals.var_rdrift_s * locals.var_rdrift_s))), (-(locals.var_rdrift_s_dn12 / (locals.var_rdrift_s * locals.var_rdrift_s))), (-(locals.var_rdrift_s_dn13 / (locals.var_rdrift_s * locals.var_rdrift_s))), (-(locals.var_rdrift_s_dn14 / (locals.var_rdrift_s * locals.var_rdrift_s))),)
    } else {
        (locals.var_gdrift_s, locals.var_gdrift_s_dn0, locals.var_gdrift_s_dn2, locals.var_gdrift_s_dn3, locals.var_gdrift_s_dn4, locals.var_gdrift_s_dn5, locals.var_gdrift_s_dn6, locals.var_gdrift_s_dn7, locals.var_gdrift_s_dn8, locals.var_gdrift_s_dn9, locals.var_gdrift_s_dn10, locals.var_gdrift_s_dn11, locals.var_gdrift_s_dn12, locals.var_gdrift_s_dn13, locals.var_gdrift_s_dn14,)
    }
};
        locals.var_gdrift_s = assign33940_e44310;
        locals.var_gdrift_s_dn0 = assign33940_e44310_d_n0;
        locals.var_gdrift_s_dn2 = assign33940_e44310_d_n2;
        locals.var_gdrift_s_dn3 = assign33940_e44310_d_n3;
        locals.var_gdrift_s_dn4 = assign33940_e44310_d_n4;
        locals.var_gdrift_s_dn5 = assign33940_e44310_d_n5;
        locals.var_gdrift_s_dn6 = assign33940_e44310_d_n6;
        locals.var_gdrift_s_dn7 = assign33940_e44310_d_n7;
        locals.var_gdrift_s_dn8 = assign33940_e44310_d_n8;
        locals.var_gdrift_s_dn9 = assign33940_e44310_d_n9;
        locals.var_gdrift_s_dn10 = assign33940_e44310_d_n10;
        locals.var_gdrift_s_dn11 = assign33940_e44310_d_n11;
        locals.var_gdrift_s_dn12 = assign33940_e44310_d_n12;
        locals.var_gdrift_s_dn13 = assign33940_e44310_d_n13;
        locals.var_gdrift_s_dn14 = assign33940_e44310_d_n14;
        locals.var_gdrift_s_rv = 0.0;

        let assign34020_e44360: f64 = if ((p.p49 != 0.0) && (p.p909 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard763 = assign34020_e44360;
        locals.var_guard763_rv = 0.0;

        let (assign34030_e44370, assign34030_e44370_d_n0, assign34030_e44370_d_n2, assign34030_e44370_d_n3, assign34030_e44370_d_n4, assign34030_e44370_d_n5, assign34030_e44370_d_n6, assign34030_e44370_d_n7, assign34030_e44370_d_n8, assign34030_e44370_d_n9, assign34030_e44370_d_n10, assign34030_e44370_d_n11, assign34030_e44370_d_n12, assign34030_e44370_d_n13, assign34030_e44370_d_n14,) = {
    if (locals.var_guard763 != 0.0) {
        let assign34030_e44364: f64 = (locals.var_devsign * locals.var_sigvds);
        let assign34030_e44366: f64 = (assign34030_e44364 * locals.var_ids);
        let assign34030_e44368: f64 = (assign34030_e44366 * (nv5 - nv7));
        (assign34030_e44368, ((assign34030_e44364 * locals.var_ids_dn0) * (nv5 - nv7)), ((assign34030_e44364 * locals.var_ids_dn2) * (nv5 - nv7)), ((assign34030_e44364 * locals.var_ids_dn3) * (nv5 - nv7)), ((assign34030_e44364 * locals.var_ids_dn4) * (nv5 - nv7)), (((assign34030_e44364 * locals.var_ids_dn5) * (nv5 - nv7)) + assign34030_e44366), ((assign34030_e44364 * locals.var_ids_dn6) * (nv5 - nv7)), (((assign34030_e44364 * locals.var_ids_dn7) * (nv5 - nv7)) + (-assign34030_e44366)), ((assign34030_e44364 * locals.var_ids_dn8) * (nv5 - nv7)), ((assign34030_e44364 * locals.var_ids_dn9) * (nv5 - nv7)), ((assign34030_e44364 * locals.var_ids_dn10) * (nv5 - nv7)), ((assign34030_e44364 * locals.var_ids_dn11) * (nv5 - nv7)), ((assign34030_e44364 * locals.var_ids_dn12) * (nv5 - nv7)), ((assign34030_e44364 * locals.var_ids_dn13) * (nv5 - nv7)), ((assign34030_e44364 * locals.var_ids_dn14) * (nv5 - nv7)),)
    } else {
        (locals.var_pdiss, locals.var_pdiss_dn0, locals.var_pdiss_dn2, locals.var_pdiss_dn3, locals.var_pdiss_dn4, locals.var_pdiss_dn5, locals.var_pdiss_dn6, locals.var_pdiss_dn7, locals.var_pdiss_dn8, locals.var_pdiss_dn9, locals.var_pdiss_dn10, locals.var_pdiss_dn11, locals.var_pdiss_dn12, locals.var_pdiss_dn13, locals.var_pdiss_dn14,)
    }
};
        locals.var_pdiss = assign34030_e44370;
        locals.var_pdiss_dn0 = assign34030_e44370_d_n0;
        locals.var_pdiss_dn2 = assign34030_e44370_d_n2;
        locals.var_pdiss_dn3 = assign34030_e44370_d_n3;
        locals.var_pdiss_dn4 = assign34030_e44370_d_n4;
        locals.var_pdiss_dn5 = assign34030_e44370_d_n5;
        locals.var_pdiss_dn6 = assign34030_e44370_d_n6;
        locals.var_pdiss_dn7 = assign34030_e44370_d_n7;
        locals.var_pdiss_dn8 = assign34030_e44370_d_n8;
        locals.var_pdiss_dn9 = assign34030_e44370_d_n9;
        locals.var_pdiss_dn10 = assign34030_e44370_d_n10;
        locals.var_pdiss_dn11 = assign34030_e44370_d_n11;
        locals.var_pdiss_dn12 = assign34030_e44370_d_n12;
        locals.var_pdiss_dn13 = assign34030_e44370_d_n13;
        locals.var_pdiss_dn14 = assign34030_e44370_d_n14;
        locals.var_pdiss_rv = 0.0;

        let assign34040_e44377: f64 = if ((p.p42 != 2.0) && (locals.var_rdraingeo > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard764 = assign34040_e44377;
        locals.var_guard764_rv = 0.0;

        let assign34050_e44388: f64 = if (((p.p42 == 1.0) && (p.p1094 == 1.0)) && (p.p1110 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard765 = assign34050_e44388;
        locals.var_guard765_rv = 0.0;

        let (assign34060_e44408, assign34060_e44408_d_n0, assign34060_e44408_d_n2, assign34060_e44408_d_n3, assign34060_e44408_d_n4, assign34060_e44408_d_n5, assign34060_e44408_d_n6, assign34060_e44408_d_n7, assign34060_e44408_d_n8, assign34060_e44408_d_n9, assign34060_e44408_d_n10, assign34060_e44408_d_n11, assign34060_e44408_d_n12, assign34060_e44408_d_n13, assign34060_e44408_d_n14,) = {
    if (((locals.var_guard763 != 0.0) && (locals.var_guard764 != 0.0)) && (locals.var_guard765 != 0.0)) {
        let assign34060_e44397: f64 = ((nv0 - nv6) * (nv0 - nv6));
        let assign34060_e44399: f64 = (assign34060_e44397 * locals.var_gdpr);
        let assign34060_e44400: f64 = (locals.var_pdiss + assign34060_e44399);
        let assign34060_e44403: f64 = ((nv6 - nv5) * (nv6 - nv5));
        let assign34060_e44405: f64 = (assign34060_e44403 * locals.var_gdrift_d);
        let assign34060_e44406: f64 = (assign34060_e44400 + assign34060_e44405);
        (assign34060_e44406, ((locals.var_pdiss_dn0 + ((((nv0 - nv6) + (nv0 - nv6)) * locals.var_gdpr) + (assign34060_e44397 * locals.var_gdpr_dn0))) + (assign34060_e44403 * locals.var_gdrift_d_dn0)), ((locals.var_pdiss_dn2 + (assign34060_e44397 * locals.var_gdpr_dn2)) + (assign34060_e44403 * locals.var_gdrift_d_dn2)), ((locals.var_pdiss_dn3 + (assign34060_e44397 * locals.var_gdpr_dn3)) + (assign34060_e44403 * locals.var_gdrift_d_dn3)), ((locals.var_pdiss_dn4 + (assign34060_e44397 * locals.var_gdpr_dn4)) + (assign34060_e44403 * locals.var_gdrift_d_dn4)), ((locals.var_pdiss_dn5 + (assign34060_e44397 * locals.var_gdpr_dn5)) + ((((-(nv6 - nv5)) + (-(nv6 - nv5))) * locals.var_gdrift_d) + (assign34060_e44403 * locals.var_gdrift_d_dn5))), ((locals.var_pdiss_dn6 + ((((-(nv0 - nv6)) + (-(nv0 - nv6))) * locals.var_gdpr) + (assign34060_e44397 * locals.var_gdpr_dn6))) + ((((nv6 - nv5) + (nv6 - nv5)) * locals.var_gdrift_d) + (assign34060_e44403 * locals.var_gdrift_d_dn6))), ((locals.var_pdiss_dn7 + (assign34060_e44397 * locals.var_gdpr_dn7)) + (assign34060_e44403 * locals.var_gdrift_d_dn7)), ((locals.var_pdiss_dn8 + (assign34060_e44397 * locals.var_gdpr_dn8)) + (assign34060_e44403 * locals.var_gdrift_d_dn8)), ((locals.var_pdiss_dn9 + (assign34060_e44397 * locals.var_gdpr_dn9)) + (assign34060_e44403 * locals.var_gdrift_d_dn9)), ((locals.var_pdiss_dn10 + (assign34060_e44397 * locals.var_gdpr_dn10)) + (assign34060_e44403 * locals.var_gdrift_d_dn10)), ((locals.var_pdiss_dn11 + (assign34060_e44397 * locals.var_gdpr_dn11)) + (assign34060_e44403 * locals.var_gdrift_d_dn11)), ((locals.var_pdiss_dn12 + (assign34060_e44397 * locals.var_gdpr_dn12)) + (assign34060_e44403 * locals.var_gdrift_d_dn12)), ((locals.var_pdiss_dn13 + (assign34060_e44397 * locals.var_gdpr_dn13)) + (assign34060_e44403 * locals.var_gdrift_d_dn13)), ((locals.var_pdiss_dn14 + (assign34060_e44397 * locals.var_gdpr_dn14)) + (assign34060_e44403 * locals.var_gdrift_d_dn14)),)
    } else {
        (locals.var_pdiss, locals.var_pdiss_dn0, locals.var_pdiss_dn2, locals.var_pdiss_dn3, locals.var_pdiss_dn4, locals.var_pdiss_dn5, locals.var_pdiss_dn6, locals.var_pdiss_dn7, locals.var_pdiss_dn8, locals.var_pdiss_dn9, locals.var_pdiss_dn10, locals.var_pdiss_dn11, locals.var_pdiss_dn12, locals.var_pdiss_dn13, locals.var_pdiss_dn14,)
    }
};
        locals.var_pdiss = assign34060_e44408;
        locals.var_pdiss_dn0 = assign34060_e44408_d_n0;
        locals.var_pdiss_dn2 = assign34060_e44408_d_n2;
        locals.var_pdiss_dn3 = assign34060_e44408_d_n3;
        locals.var_pdiss_dn4 = assign34060_e44408_d_n4;
        locals.var_pdiss_dn5 = assign34060_e44408_d_n5;
        locals.var_pdiss_dn6 = assign34060_e44408_d_n6;
        locals.var_pdiss_dn7 = assign34060_e44408_d_n7;
        locals.var_pdiss_dn8 = assign34060_e44408_d_n8;
        locals.var_pdiss_dn9 = assign34060_e44408_d_n9;
        locals.var_pdiss_dn10 = assign34060_e44408_d_n10;
        locals.var_pdiss_dn11 = assign34060_e44408_d_n11;
        locals.var_pdiss_dn12 = assign34060_e44408_d_n12;
        locals.var_pdiss_dn13 = assign34060_e44408_d_n13;
        locals.var_pdiss_dn14 = assign34060_e44408_d_n14;
        locals.var_pdiss_rv = 0.0;

        let (assign34070_e44423, assign34070_e44423_d_n0, assign34070_e44423_d_n2, assign34070_e44423_d_n3, assign34070_e44423_d_n4, assign34070_e44423_d_n5, assign34070_e44423_d_n6, assign34070_e44423_d_n7, assign34070_e44423_d_n8, assign34070_e44423_d_n9, assign34070_e44423_d_n10, assign34070_e44423_d_n11, assign34070_e44423_d_n12, assign34070_e44423_d_n13, assign34070_e44423_d_n14,) = {
    if (((locals.var_guard763 != 0.0) && (locals.var_guard764 != 0.0)) && (locals.var_guard765 == 0.0)) {
        let assign34070_e44418: f64 = ((nv0 - nv6) * (nv0 - nv6));
        let assign34070_e44420: f64 = (assign34070_e44418 * locals.var_gdpr);
        let assign34070_e44421: f64 = (locals.var_pdiss + assign34070_e44420);
        (assign34070_e44421, (locals.var_pdiss_dn0 + ((((nv0 - nv6) + (nv0 - nv6)) * locals.var_gdpr) + (assign34070_e44418 * locals.var_gdpr_dn0))), (locals.var_pdiss_dn2 + (assign34070_e44418 * locals.var_gdpr_dn2)), (locals.var_pdiss_dn3 + (assign34070_e44418 * locals.var_gdpr_dn3)), (locals.var_pdiss_dn4 + (assign34070_e44418 * locals.var_gdpr_dn4)), (locals.var_pdiss_dn5 + (assign34070_e44418 * locals.var_gdpr_dn5)), (locals.var_pdiss_dn6 + ((((-(nv0 - nv6)) + (-(nv0 - nv6))) * locals.var_gdpr) + (assign34070_e44418 * locals.var_gdpr_dn6))), (locals.var_pdiss_dn7 + (assign34070_e44418 * locals.var_gdpr_dn7)), (locals.var_pdiss_dn8 + (assign34070_e44418 * locals.var_gdpr_dn8)), (locals.var_pdiss_dn9 + (assign34070_e44418 * locals.var_gdpr_dn9)), (locals.var_pdiss_dn10 + (assign34070_e44418 * locals.var_gdpr_dn10)), (locals.var_pdiss_dn11 + (assign34070_e44418 * locals.var_gdpr_dn11)), (locals.var_pdiss_dn12 + (assign34070_e44418 * locals.var_gdpr_dn12)), (locals.var_pdiss_dn13 + (assign34070_e44418 * locals.var_gdpr_dn13)), (locals.var_pdiss_dn14 + (assign34070_e44418 * locals.var_gdpr_dn14)),)
    } else {
        (locals.var_pdiss, locals.var_pdiss_dn0, locals.var_pdiss_dn2, locals.var_pdiss_dn3, locals.var_pdiss_dn4, locals.var_pdiss_dn5, locals.var_pdiss_dn6, locals.var_pdiss_dn7, locals.var_pdiss_dn8, locals.var_pdiss_dn9, locals.var_pdiss_dn10, locals.var_pdiss_dn11, locals.var_pdiss_dn12, locals.var_pdiss_dn13, locals.var_pdiss_dn14,)
    }
};
        locals.var_pdiss = assign34070_e44423;
        locals.var_pdiss_dn0 = assign34070_e44423_d_n0;
        locals.var_pdiss_dn2 = assign34070_e44423_d_n2;
        locals.var_pdiss_dn3 = assign34070_e44423_d_n3;
        locals.var_pdiss_dn4 = assign34070_e44423_d_n4;
        locals.var_pdiss_dn5 = assign34070_e44423_d_n5;
        locals.var_pdiss_dn6 = assign34070_e44423_d_n6;
        locals.var_pdiss_dn7 = assign34070_e44423_d_n7;
        locals.var_pdiss_dn8 = assign34070_e44423_d_n8;
        locals.var_pdiss_dn9 = assign34070_e44423_d_n9;
        locals.var_pdiss_dn10 = assign34070_e44423_d_n10;
        locals.var_pdiss_dn11 = assign34070_e44423_d_n11;
        locals.var_pdiss_dn12 = assign34070_e44423_d_n12;
        locals.var_pdiss_dn13 = assign34070_e44423_d_n13;
        locals.var_pdiss_dn14 = assign34070_e44423_d_n14;
        locals.var_pdiss_rv = 0.0;

        let assign34080_e44430: f64 = if ((p.p42 != 2.0) && (locals.var_rsourcegeo > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard766 = assign34080_e44430;
        locals.var_guard766_rv = 0.0;

        let assign34090_e44441: f64 = if (((p.p42 == 1.0) && (p.p1094 == 1.0)) && (p.p1112 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard767 = assign34090_e44441;
        locals.var_guard767_rv = 0.0;

        let (assign34100_e44461, assign34100_e44461_d_n0, assign34100_e44461_d_n2, assign34100_e44461_d_n3, assign34100_e44461_d_n4, assign34100_e44461_d_n5, assign34100_e44461_d_n6, assign34100_e44461_d_n7, assign34100_e44461_d_n8, assign34100_e44461_d_n9, assign34100_e44461_d_n10, assign34100_e44461_d_n11, assign34100_e44461_d_n12, assign34100_e44461_d_n13, assign34100_e44461_d_n14,) = {
    if (((locals.var_guard763 != 0.0) && (locals.var_guard766 != 0.0)) && (locals.var_guard767 != 0.0)) {
        let assign34100_e44450: f64 = ((nv2 - nv8) * (nv2 - nv8));
        let assign34100_e44452: f64 = (assign34100_e44450 * locals.var_gspr);
        let assign34100_e44453: f64 = (locals.var_pdiss + assign34100_e44452);
        let assign34100_e44456: f64 = ((nv8 - nv7) * (nv8 - nv7));
        let assign34100_e44458: f64 = (assign34100_e44456 * locals.var_gdrift_s);
        let assign34100_e44459: f64 = (assign34100_e44453 + assign34100_e44458);
        (assign34100_e44459, ((locals.var_pdiss_dn0 + (assign34100_e44450 * locals.var_gspr_dn0)) + (assign34100_e44456 * locals.var_gdrift_s_dn0)), ((locals.var_pdiss_dn2 + ((((nv2 - nv8) + (nv2 - nv8)) * locals.var_gspr) + (assign34100_e44450 * locals.var_gspr_dn2))) + (assign34100_e44456 * locals.var_gdrift_s_dn2)), ((locals.var_pdiss_dn3 + (assign34100_e44450 * locals.var_gspr_dn3)) + (assign34100_e44456 * locals.var_gdrift_s_dn3)), ((locals.var_pdiss_dn4 + (assign34100_e44450 * locals.var_gspr_dn4)) + (assign34100_e44456 * locals.var_gdrift_s_dn4)), ((locals.var_pdiss_dn5 + (assign34100_e44450 * locals.var_gspr_dn5)) + (assign34100_e44456 * locals.var_gdrift_s_dn5)), ((locals.var_pdiss_dn6 + (assign34100_e44450 * locals.var_gspr_dn6)) + (assign34100_e44456 * locals.var_gdrift_s_dn6)), ((locals.var_pdiss_dn7 + (assign34100_e44450 * locals.var_gspr_dn7)) + ((((-(nv8 - nv7)) + (-(nv8 - nv7))) * locals.var_gdrift_s) + (assign34100_e44456 * locals.var_gdrift_s_dn7))), ((locals.var_pdiss_dn8 + ((((-(nv2 - nv8)) + (-(nv2 - nv8))) * locals.var_gspr) + (assign34100_e44450 * locals.var_gspr_dn8))) + ((((nv8 - nv7) + (nv8 - nv7)) * locals.var_gdrift_s) + (assign34100_e44456 * locals.var_gdrift_s_dn8))), ((locals.var_pdiss_dn9 + (assign34100_e44450 * locals.var_gspr_dn9)) + (assign34100_e44456 * locals.var_gdrift_s_dn9)), ((locals.var_pdiss_dn10 + (assign34100_e44450 * locals.var_gspr_dn10)) + (assign34100_e44456 * locals.var_gdrift_s_dn10)), ((locals.var_pdiss_dn11 + (assign34100_e44450 * locals.var_gspr_dn11)) + (assign34100_e44456 * locals.var_gdrift_s_dn11)), ((locals.var_pdiss_dn12 + (assign34100_e44450 * locals.var_gspr_dn12)) + (assign34100_e44456 * locals.var_gdrift_s_dn12)), ((locals.var_pdiss_dn13 + (assign34100_e44450 * locals.var_gspr_dn13)) + (assign34100_e44456 * locals.var_gdrift_s_dn13)), ((locals.var_pdiss_dn14 + (assign34100_e44450 * locals.var_gspr_dn14)) + (assign34100_e44456 * locals.var_gdrift_s_dn14)),)
    } else {
        (locals.var_pdiss, locals.var_pdiss_dn0, locals.var_pdiss_dn2, locals.var_pdiss_dn3, locals.var_pdiss_dn4, locals.var_pdiss_dn5, locals.var_pdiss_dn6, locals.var_pdiss_dn7, locals.var_pdiss_dn8, locals.var_pdiss_dn9, locals.var_pdiss_dn10, locals.var_pdiss_dn11, locals.var_pdiss_dn12, locals.var_pdiss_dn13, locals.var_pdiss_dn14,)
    }
};
        locals.var_pdiss = assign34100_e44461;
        locals.var_pdiss_dn0 = assign34100_e44461_d_n0;
        locals.var_pdiss_dn2 = assign34100_e44461_d_n2;
        locals.var_pdiss_dn3 = assign34100_e44461_d_n3;
        locals.var_pdiss_dn4 = assign34100_e44461_d_n4;
        locals.var_pdiss_dn5 = assign34100_e44461_d_n5;
        locals.var_pdiss_dn6 = assign34100_e44461_d_n6;
        locals.var_pdiss_dn7 = assign34100_e44461_d_n7;
        locals.var_pdiss_dn8 = assign34100_e44461_d_n8;
        locals.var_pdiss_dn9 = assign34100_e44461_d_n9;
        locals.var_pdiss_dn10 = assign34100_e44461_d_n10;
        locals.var_pdiss_dn11 = assign34100_e44461_d_n11;
        locals.var_pdiss_dn12 = assign34100_e44461_d_n12;
        locals.var_pdiss_dn13 = assign34100_e44461_d_n13;
        locals.var_pdiss_dn14 = assign34100_e44461_d_n14;
        locals.var_pdiss_rv = 0.0;

        let (assign34110_e44476, assign34110_e44476_d_n0, assign34110_e44476_d_n2, assign34110_e44476_d_n3, assign34110_e44476_d_n4, assign34110_e44476_d_n5, assign34110_e44476_d_n6, assign34110_e44476_d_n7, assign34110_e44476_d_n8, assign34110_e44476_d_n9, assign34110_e44476_d_n10, assign34110_e44476_d_n11, assign34110_e44476_d_n12, assign34110_e44476_d_n13, assign34110_e44476_d_n14,) = {
    if (((locals.var_guard763 != 0.0) && (locals.var_guard766 != 0.0)) && (locals.var_guard767 == 0.0)) {
        let assign34110_e44471: f64 = ((nv2 - nv8) * (nv2 - nv8));
        let assign34110_e44473: f64 = (assign34110_e44471 * locals.var_gspr);
        let assign34110_e44474: f64 = (locals.var_pdiss + assign34110_e44473);
        (assign34110_e44474, (locals.var_pdiss_dn0 + (assign34110_e44471 * locals.var_gspr_dn0)), (locals.var_pdiss_dn2 + ((((nv2 - nv8) + (nv2 - nv8)) * locals.var_gspr) + (assign34110_e44471 * locals.var_gspr_dn2))), (locals.var_pdiss_dn3 + (assign34110_e44471 * locals.var_gspr_dn3)), (locals.var_pdiss_dn4 + (assign34110_e44471 * locals.var_gspr_dn4)), (locals.var_pdiss_dn5 + (assign34110_e44471 * locals.var_gspr_dn5)), (locals.var_pdiss_dn6 + (assign34110_e44471 * locals.var_gspr_dn6)), (locals.var_pdiss_dn7 + (assign34110_e44471 * locals.var_gspr_dn7)), (locals.var_pdiss_dn8 + ((((-(nv2 - nv8)) + (-(nv2 - nv8))) * locals.var_gspr) + (assign34110_e44471 * locals.var_gspr_dn8))), (locals.var_pdiss_dn9 + (assign34110_e44471 * locals.var_gspr_dn9)), (locals.var_pdiss_dn10 + (assign34110_e44471 * locals.var_gspr_dn10)), (locals.var_pdiss_dn11 + (assign34110_e44471 * locals.var_gspr_dn11)), (locals.var_pdiss_dn12 + (assign34110_e44471 * locals.var_gspr_dn12)), (locals.var_pdiss_dn13 + (assign34110_e44471 * locals.var_gspr_dn13)), (locals.var_pdiss_dn14 + (assign34110_e44471 * locals.var_gspr_dn14)),)
    } else {
        (locals.var_pdiss, locals.var_pdiss_dn0, locals.var_pdiss_dn2, locals.var_pdiss_dn3, locals.var_pdiss_dn4, locals.var_pdiss_dn5, locals.var_pdiss_dn6, locals.var_pdiss_dn7, locals.var_pdiss_dn8, locals.var_pdiss_dn9, locals.var_pdiss_dn10, locals.var_pdiss_dn11, locals.var_pdiss_dn12, locals.var_pdiss_dn13, locals.var_pdiss_dn14,)
    }
};
        locals.var_pdiss = assign34110_e44476;
        locals.var_pdiss_dn0 = assign34110_e44476_d_n0;
        locals.var_pdiss_dn2 = assign34110_e44476_d_n2;
        locals.var_pdiss_dn3 = assign34110_e44476_d_n3;
        locals.var_pdiss_dn4 = assign34110_e44476_d_n4;
        locals.var_pdiss_dn5 = assign34110_e44476_d_n5;
        locals.var_pdiss_dn6 = assign34110_e44476_d_n6;
        locals.var_pdiss_dn7 = assign34110_e44476_d_n7;
        locals.var_pdiss_dn8 = assign34110_e44476_d_n8;
        locals.var_pdiss_dn9 = assign34110_e44476_d_n9;
        locals.var_pdiss_dn10 = assign34110_e44476_d_n10;
        locals.var_pdiss_dn11 = assign34110_e44476_d_n11;
        locals.var_pdiss_dn12 = assign34110_e44476_d_n12;
        locals.var_pdiss_dn13 = assign34110_e44476_d_n13;
        locals.var_pdiss_dn14 = assign34110_e44476_d_n14;
        locals.var_pdiss_rv = 0.0;

        let assign34130_e44482: f64 = if p.p8 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard769 = assign34130_e44482;
        locals.var_guard769_rv = 0.0;

        let assign34140_e44485: f64 = if p.p1097 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard770 = assign34140_e44485;
        locals.var_guard770_rv = 0.0;

        let assign34160_e44499: f64 = if ((p.p8 != 0.0) && (p.p1097 == 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard772 = assign34160_e44499;
        locals.var_guard772_rv = 0.0;

    }

    pub(super) fn stamp_transient_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv15 = ctx.node_voltage(nodes[15]);
        let nv16 = ctx.node_voltage(nodes[16]);
        let (eq7_e1272, eq7_e1272_d_n0, eq7_e1272_d_n2, eq7_e1272_d_n3, eq7_e1272_d_n4, eq7_e1272_d_n5, eq7_e1272_d_n6, eq7_e1272_d_n7, eq7_e1272_d_n8, eq7_e1272_d_n9, eq7_e1272_d_n10, eq7_e1272_d_n11, eq7_e1272_d_n12, eq7_e1272_d_n13, eq7_e1272_d_n14, eq7_e1272_d_n16,) = {
    if ((locals.var_guard698 != 0.0) && (locals.var_guard697 == 0.0)) {
        let eq7_e1268: f64 = (-locals.var_sqig);
        let eq7_e1270: f64 = (eq7_e1268 * (nv16 - 0.0));
        let eq7_e1270_d_n0: f64 = ((-locals.var_sqig_dn0) * (nv16 - 0.0));
        let eq7_e1270_d_n2: f64 = ((-locals.var_sqig_dn2) * (nv16 - 0.0));
        let eq7_e1270_d_n3: f64 = ((-locals.var_sqig_dn3) * (nv16 - 0.0));
        let eq7_e1270_d_n4: f64 = ((-locals.var_sqig_dn4) * (nv16 - 0.0));
        let eq7_e1270_d_n5: f64 = ((-locals.var_sqig_dn5) * (nv16 - 0.0));
        let eq7_e1270_d_n6: f64 = ((-locals.var_sqig_dn6) * (nv16 - 0.0));
        let eq7_e1270_d_n7: f64 = ((-locals.var_sqig_dn7) * (nv16 - 0.0));
        let eq7_e1270_d_n8: f64 = ((-locals.var_sqig_dn8) * (nv16 - 0.0));
        let eq7_e1270_d_n9: f64 = ((-locals.var_sqig_dn9) * (nv16 - 0.0));
        let eq7_e1270_d_n10: f64 = ((-locals.var_sqig_dn10) * (nv16 - 0.0));
        let eq7_e1270_d_n11: f64 = ((-locals.var_sqig_dn11) * (nv16 - 0.0));
        let eq7_e1270_d_n12: f64 = ((-locals.var_sqig_dn12) * (nv16 - 0.0));
        let eq7_e1270_d_n13: f64 = ((-locals.var_sqig_dn13) * (nv16 - 0.0));
        let eq7_e1270_d_n14: f64 = ((-locals.var_sqig_dn14) * (nv16 - 0.0));
        (eq7_e1270, eq7_e1270_d_n0, eq7_e1270_d_n2, eq7_e1270_d_n3, eq7_e1270_d_n4, eq7_e1270_d_n5, eq7_e1270_d_n6, eq7_e1270_d_n7, eq7_e1270_d_n8, eq7_e1270_d_n9, eq7_e1270_d_n10, eq7_e1270_d_n11, eq7_e1270_d_n12, eq7_e1270_d_n13, eq7_e1270_d_n14, eq7_e1268,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_value: f64 = eq7_e1272;
        let eq7_node_derivative_indices: [usize; 15] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 16];
        let eq7_node_derivatives: [f64; 15] = [eq7_e1272_d_n0, eq7_e1272_d_n2, eq7_e1272_d_n3, eq7_e1272_d_n4, eq7_e1272_d_n5, eq7_e1272_d_n6, eq7_e1272_d_n7, eq7_e1272_d_n8, eq7_e1272_d_n9, eq7_e1272_d_n10, eq7_e1272_d_n11, eq7_e1272_d_n12, eq7_e1272_d_n13, eq7_e1272_d_n14, eq7_e1272_d_n16];
        let eq7_branch_derivative_indices: [usize; 0] = [];
        let eq7_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(15),
            None,
            multiplicity * (eq7_value),
            &eq7_node_derivative_indices,
            &eq7_node_derivatives,
            &eq7_branch_derivative_indices,
            &eq7_branch_derivatives,
            multiplicity,
        );
        let (eq8_e1290, eq8_e1290_d_n0, eq8_e1290_d_n2, eq8_e1290_d_n3, eq8_e1290_d_n4, eq8_e1290_d_n5, eq8_e1290_d_n6, eq8_e1290_d_n7, eq8_e1290_d_n8, eq8_e1290_d_n9, eq8_e1290_d_n10, eq8_e1290_d_n11, eq8_e1290_d_n12, eq8_e1290_d_n13, eq8_e1290_d_n14, eq8_e1290_d_n15,) = {
    if ((locals.var_guard698 != 0.0) && (locals.var_guard697 == 0.0)) {
        let eq8_e1279: f64 = (locals.var_mig * locals.var_cox);
        let eq8_e1279_d_n0: f64 = (locals.var_mig_dn0 * locals.var_cox);
        let eq8_e1279_d_n2: f64 = (locals.var_mig_dn2 * locals.var_cox);
        let eq8_e1279_d_n3: f64 = (locals.var_mig_dn3 * locals.var_cox);
        let eq8_e1279_d_n4: f64 = (locals.var_mig_dn4 * locals.var_cox);
        let eq8_e1279_d_n5: f64 = (locals.var_mig_dn5 * locals.var_cox);
        let eq8_e1279_d_n6: f64 = (locals.var_mig_dn6 * locals.var_cox);
        let eq8_e1279_d_n7: f64 = (locals.var_mig_dn7 * locals.var_cox);
        let eq8_e1279_d_n8: f64 = (locals.var_mig_dn8 * locals.var_cox);
        let eq8_e1279_d_n9: f64 = (locals.var_mig_dn9 * locals.var_cox);
        let eq8_e1279_d_n10: f64 = (locals.var_mig_dn10 * locals.var_cox);
        let eq8_e1279_d_n11: f64 = (locals.var_mig_dn11 * locals.var_cox);
        let eq8_e1279_d_n12: f64 = (locals.var_mig_dn12 * locals.var_cox);
        let eq8_e1279_d_n13: f64 = (locals.var_mig_dn13 * locals.var_cox);
        let eq8_e1279_d_n14: f64 = (locals.var_mig_dn14 * locals.var_cox);
        let eq8_e1281: f64 = (eq8_e1279 * locals.var_weff);
        let eq8_e1281_d_n0: f64 = (eq8_e1279_d_n0 * locals.var_weff);
        let eq8_e1281_d_n2: f64 = (eq8_e1279_d_n2 * locals.var_weff);
        let eq8_e1281_d_n3: f64 = (eq8_e1279_d_n3 * locals.var_weff);
        let eq8_e1281_d_n4: f64 = (eq8_e1279_d_n4 * locals.var_weff);
        let eq8_e1281_d_n5: f64 = (eq8_e1279_d_n5 * locals.var_weff);
        let eq8_e1281_d_n6: f64 = (eq8_e1279_d_n6 * locals.var_weff);
        let eq8_e1281_d_n7: f64 = (eq8_e1279_d_n7 * locals.var_weff);
        let eq8_e1281_d_n8: f64 = (eq8_e1279_d_n8 * locals.var_weff);
        let eq8_e1281_d_n9: f64 = (eq8_e1279_d_n9 * locals.var_weff);
        let eq8_e1281_d_n10: f64 = (eq8_e1279_d_n10 * locals.var_weff);
        let eq8_e1281_d_n11: f64 = (eq8_e1279_d_n11 * locals.var_weff);
        let eq8_e1281_d_n12: f64 = (eq8_e1279_d_n12 * locals.var_weff);
        let eq8_e1281_d_n13: f64 = (eq8_e1279_d_n13 * locals.var_weff);
        let eq8_e1281_d_n14: f64 = (eq8_e1279_d_n14 * locals.var_weff);
        let eq8_e1283: f64 = (eq8_e1281 * p.p2);
        let eq8_e1283_d_n0: f64 = (eq8_e1281_d_n0 * p.p2);
        let eq8_e1283_d_n2: f64 = (eq8_e1281_d_n2 * p.p2);
        let eq8_e1283_d_n3: f64 = (eq8_e1281_d_n3 * p.p2);
        let eq8_e1283_d_n4: f64 = (eq8_e1281_d_n4 * p.p2);
        let eq8_e1283_d_n5: f64 = (eq8_e1281_d_n5 * p.p2);
        let eq8_e1283_d_n6: f64 = (eq8_e1281_d_n6 * p.p2);
        let eq8_e1283_d_n7: f64 = (eq8_e1281_d_n7 * p.p2);
        let eq8_e1283_d_n8: f64 = (eq8_e1281_d_n8 * p.p2);
        let eq8_e1283_d_n9: f64 = (eq8_e1281_d_n9 * p.p2);
        let eq8_e1283_d_n10: f64 = (eq8_e1281_d_n10 * p.p2);
        let eq8_e1283_d_n11: f64 = (eq8_e1281_d_n11 * p.p2);
        let eq8_e1283_d_n12: f64 = (eq8_e1281_d_n12 * p.p2);
        let eq8_e1283_d_n13: f64 = (eq8_e1281_d_n13 * p.p2);
        let eq8_e1283_d_n14: f64 = (eq8_e1281_d_n14 * p.p2);
        let eq8_e1285: f64 = (eq8_e1283 * locals.var_leff);
        let eq8_e1285_d_n0: f64 = (eq8_e1283_d_n0 * locals.var_leff);
        let eq8_e1285_d_n2: f64 = (eq8_e1283_d_n2 * locals.var_leff);
        let eq8_e1285_d_n3: f64 = (eq8_e1283_d_n3 * locals.var_leff);
        let eq8_e1285_d_n4: f64 = (eq8_e1283_d_n4 * locals.var_leff);
        let eq8_e1285_d_n5: f64 = (eq8_e1283_d_n5 * locals.var_leff);
        let eq8_e1285_d_n6: f64 = (eq8_e1283_d_n6 * locals.var_leff);
        let eq8_e1285_d_n7: f64 = (eq8_e1283_d_n7 * locals.var_leff);
        let eq8_e1285_d_n8: f64 = (eq8_e1283_d_n8 * locals.var_leff);
        let eq8_e1285_d_n9: f64 = (eq8_e1283_d_n9 * locals.var_leff);
        let eq8_e1285_d_n10: f64 = (eq8_e1283_d_n10 * locals.var_leff);
        let eq8_e1285_d_n11: f64 = (eq8_e1283_d_n11 * locals.var_leff);
        let eq8_e1285_d_n12: f64 = (eq8_e1283_d_n12 * locals.var_leff);
        let eq8_e1285_d_n13: f64 = (eq8_e1283_d_n13 * locals.var_leff);
        let eq8_e1285_d_n14: f64 = (eq8_e1283_d_n14 * locals.var_leff);
        let eq8_e1287: f64 = (eq8_e1285 * (nv15 - 0.0));
        let eq8_e1287_d_n0: f64 = (eq8_e1285_d_n0 * (nv15 - 0.0));
        let eq8_e1287_d_n2: f64 = (eq8_e1285_d_n2 * (nv15 - 0.0));
        let eq8_e1287_d_n3: f64 = (eq8_e1285_d_n3 * (nv15 - 0.0));
        let eq8_e1287_d_n4: f64 = (eq8_e1285_d_n4 * (nv15 - 0.0));
        let eq8_e1287_d_n5: f64 = (eq8_e1285_d_n5 * (nv15 - 0.0));
        let eq8_e1287_d_n6: f64 = (eq8_e1285_d_n6 * (nv15 - 0.0));
        let eq8_e1287_d_n7: f64 = (eq8_e1285_d_n7 * (nv15 - 0.0));
        let eq8_e1287_d_n8: f64 = (eq8_e1285_d_n8 * (nv15 - 0.0));
        let eq8_e1287_d_n9: f64 = (eq8_e1285_d_n9 * (nv15 - 0.0));
        let eq8_e1287_d_n10: f64 = (eq8_e1285_d_n10 * (nv15 - 0.0));
        let eq8_e1287_d_n11: f64 = (eq8_e1285_d_n11 * (nv15 - 0.0));
        let eq8_e1287_d_n12: f64 = (eq8_e1285_d_n12 * (nv15 - 0.0));
        let eq8_e1287_d_n13: f64 = (eq8_e1285_d_n13 * (nv15 - 0.0));
        let eq8_e1287_d_n14: f64 = (eq8_e1285_d_n14 * (nv15 - 0.0));
        let eq8_e1288: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, eq8_e1287);
        (eq8_e1288, (eq8_e1287_d_n0 * ddt_scale), (eq8_e1287_d_n2 * ddt_scale), (eq8_e1287_d_n3 * ddt_scale), (eq8_e1287_d_n4 * ddt_scale), (eq8_e1287_d_n5 * ddt_scale), (eq8_e1287_d_n6 * ddt_scale), (eq8_e1287_d_n7 * ddt_scale), (eq8_e1287_d_n8 * ddt_scale), (eq8_e1287_d_n9 * ddt_scale), (eq8_e1287_d_n10 * ddt_scale), (eq8_e1287_d_n11 * ddt_scale), (eq8_e1287_d_n12 * ddt_scale), (eq8_e1287_d_n13 * ddt_scale), (eq8_e1287_d_n14 * ddt_scale), (eq8_e1285 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq8_value: f64 = eq8_e1290;
        let eq8_node_derivative_indices: [usize; 15] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        let eq8_node_derivatives: [f64; 15] = [eq8_e1290_d_n0, eq8_e1290_d_n2, eq8_e1290_d_n3, eq8_e1290_d_n4, eq8_e1290_d_n5, eq8_e1290_d_n6, eq8_e1290_d_n7, eq8_e1290_d_n8, eq8_e1290_d_n9, eq8_e1290_d_n10, eq8_e1290_d_n11, eq8_e1290_d_n12, eq8_e1290_d_n13, eq8_e1290_d_n14, eq8_e1290_d_n15];
        let eq8_branch_derivative_indices: [usize; 0] = [];
        let eq8_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(15),
            None,
            multiplicity * (eq8_value),
            &eq8_node_derivative_indices,
            &eq8_node_derivatives,
            &eq8_branch_derivative_indices,
            &eq8_branch_derivatives,
            multiplicity,
        );
        let (eq10_e1318, eq10_e1318_d_n0, eq10_e1318_d_n2, eq10_e1318_d_n3, eq10_e1318_d_n4, eq10_e1318_d_n5, eq10_e1318_d_n6, eq10_e1318_d_n7, eq10_e1318_d_n8, eq10_e1318_d_n9, eq10_e1318_d_n10, eq10_e1318_d_n11, eq10_e1318_d_n12, eq10_e1318_d_n13, eq10_e1318_d_n14, eq10_e1318_d_n16,) = {
    if ((locals.var_guard698 != 0.0) && (locals.var_guard697 == 0.0)) {
        let eq10_e1314: f64 = (locals.var_sqid * p.p28);
        let eq10_e1314_d_n0: f64 = (locals.var_sqid_dn0 * p.p28);
        let eq10_e1314_d_n2: f64 = (locals.var_sqid_dn2 * p.p28);
        let eq10_e1314_d_n3: f64 = (locals.var_sqid_dn3 * p.p28);
        let eq10_e1314_d_n4: f64 = (locals.var_sqid_dn4 * p.p28);
        let eq10_e1314_d_n5: f64 = (locals.var_sqid_dn5 * p.p28);
        let eq10_e1314_d_n6: f64 = (locals.var_sqid_dn6 * p.p28);
        let eq10_e1314_d_n7: f64 = (locals.var_sqid_dn7 * p.p28);
        let eq10_e1314_d_n8: f64 = (locals.var_sqid_dn8 * p.p28);
        let eq10_e1314_d_n9: f64 = (locals.var_sqid_dn9 * p.p28);
        let eq10_e1314_d_n10: f64 = (locals.var_sqid_dn10 * p.p28);
        let eq10_e1314_d_n11: f64 = (locals.var_sqid_dn11 * p.p28);
        let eq10_e1314_d_n12: f64 = (locals.var_sqid_dn12 * p.p28);
        let eq10_e1314_d_n13: f64 = (locals.var_sqid_dn13 * p.p28);
        let eq10_e1314_d_n14: f64 = (locals.var_sqid_dn14 * p.p28);
        let eq10_e1316: f64 = (eq10_e1314 * (nv16 - 0.0));
        let eq10_e1316_d_n0: f64 = (eq10_e1314_d_n0 * (nv16 - 0.0));
        let eq10_e1316_d_n2: f64 = (eq10_e1314_d_n2 * (nv16 - 0.0));
        let eq10_e1316_d_n3: f64 = (eq10_e1314_d_n3 * (nv16 - 0.0));
        let eq10_e1316_d_n4: f64 = (eq10_e1314_d_n4 * (nv16 - 0.0));
        let eq10_e1316_d_n5: f64 = (eq10_e1314_d_n5 * (nv16 - 0.0));
        let eq10_e1316_d_n6: f64 = (eq10_e1314_d_n6 * (nv16 - 0.0));
        let eq10_e1316_d_n7: f64 = (eq10_e1314_d_n7 * (nv16 - 0.0));
        let eq10_e1316_d_n8: f64 = (eq10_e1314_d_n8 * (nv16 - 0.0));
        let eq10_e1316_d_n9: f64 = (eq10_e1314_d_n9 * (nv16 - 0.0));
        let eq10_e1316_d_n10: f64 = (eq10_e1314_d_n10 * (nv16 - 0.0));
        let eq10_e1316_d_n11: f64 = (eq10_e1314_d_n11 * (nv16 - 0.0));
        let eq10_e1316_d_n12: f64 = (eq10_e1314_d_n12 * (nv16 - 0.0));
        let eq10_e1316_d_n13: f64 = (eq10_e1314_d_n13 * (nv16 - 0.0));
        let eq10_e1316_d_n14: f64 = (eq10_e1314_d_n14 * (nv16 - 0.0));
        (eq10_e1316, eq10_e1316_d_n0, eq10_e1316_d_n2, eq10_e1316_d_n3, eq10_e1316_d_n4, eq10_e1316_d_n5, eq10_e1316_d_n6, eq10_e1316_d_n7, eq10_e1316_d_n8, eq10_e1316_d_n9, eq10_e1316_d_n10, eq10_e1316_d_n11, eq10_e1316_d_n12, eq10_e1316_d_n13, eq10_e1316_d_n14, eq10_e1314,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq10_value: f64 = eq10_e1318;
        let eq10_node_derivative_indices: [usize; 15] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 16];
        let eq10_node_derivatives: [f64; 15] = [eq10_e1318_d_n0, eq10_e1318_d_n2, eq10_e1318_d_n3, eq10_e1318_d_n4, eq10_e1318_d_n5, eq10_e1318_d_n6, eq10_e1318_d_n7, eq10_e1318_d_n8, eq10_e1318_d_n9, eq10_e1318_d_n10, eq10_e1318_d_n11, eq10_e1318_d_n12, eq10_e1318_d_n13, eq10_e1318_d_n14, eq10_e1318_d_n16];
        let eq10_branch_derivative_indices: [usize; 0] = [];
        let eq10_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(5),
            Some(7),
            multiplicity * (eq10_value),
            &eq10_node_derivative_indices,
            &eq10_node_derivatives,
            &eq10_branch_derivative_indices,
            &eq10_branch_derivatives,
            multiplicity,
        );
        let (eq11_e1344, eq11_e1344_d_n0, eq11_e1344_d_n2, eq11_e1344_d_n3, eq11_e1344_d_n4, eq11_e1344_d_n5, eq11_e1344_d_n6, eq11_e1344_d_n7, eq11_e1344_d_n8, eq11_e1344_d_n9, eq11_e1344_d_n10, eq11_e1344_d_n11, eq11_e1344_d_n12, eq11_e1344_d_n13, eq11_e1344_d_n14, eq11_e1344_d_n15,) = {
    if ((locals.var_guard698 != 0.0) && (locals.var_guard697 == 0.0)) {
        let eq11_e1327: f64 = (1.0 + locals.var_sigvds);
        let eq11_e1329: f64 = (eq11_e1327 * locals.var_mig);
        let eq11_e1329_d_n0: f64 = (eq11_e1327 * locals.var_mig_dn0);
        let eq11_e1329_d_n2: f64 = (eq11_e1327 * locals.var_mig_dn2);
        let eq11_e1329_d_n3: f64 = (eq11_e1327 * locals.var_mig_dn3);
        let eq11_e1329_d_n4: f64 = (eq11_e1327 * locals.var_mig_dn4);
        let eq11_e1329_d_n5: f64 = (eq11_e1327 * locals.var_mig_dn5);
        let eq11_e1329_d_n6: f64 = (eq11_e1327 * locals.var_mig_dn6);
        let eq11_e1329_d_n7: f64 = (eq11_e1327 * locals.var_mig_dn7);
        let eq11_e1329_d_n8: f64 = (eq11_e1327 * locals.var_mig_dn8);
        let eq11_e1329_d_n9: f64 = (eq11_e1327 * locals.var_mig_dn9);
        let eq11_e1329_d_n10: f64 = (eq11_e1327 * locals.var_mig_dn10);
        let eq11_e1329_d_n11: f64 = (eq11_e1327 * locals.var_mig_dn11);
        let eq11_e1329_d_n12: f64 = (eq11_e1327 * locals.var_mig_dn12);
        let eq11_e1329_d_n13: f64 = (eq11_e1327 * locals.var_mig_dn13);
        let eq11_e1329_d_n14: f64 = (eq11_e1327 * locals.var_mig_dn14);
        let eq11_e1331: f64 = (eq11_e1329 * locals.var_cox);
        let eq11_e1331_d_n0: f64 = (eq11_e1329_d_n0 * locals.var_cox);
        let eq11_e1331_d_n2: f64 = (eq11_e1329_d_n2 * locals.var_cox);
        let eq11_e1331_d_n3: f64 = (eq11_e1329_d_n3 * locals.var_cox);
        let eq11_e1331_d_n4: f64 = (eq11_e1329_d_n4 * locals.var_cox);
        let eq11_e1331_d_n5: f64 = (eq11_e1329_d_n5 * locals.var_cox);
        let eq11_e1331_d_n6: f64 = (eq11_e1329_d_n6 * locals.var_cox);
        let eq11_e1331_d_n7: f64 = (eq11_e1329_d_n7 * locals.var_cox);
        let eq11_e1331_d_n8: f64 = (eq11_e1329_d_n8 * locals.var_cox);
        let eq11_e1331_d_n9: f64 = (eq11_e1329_d_n9 * locals.var_cox);
        let eq11_e1331_d_n10: f64 = (eq11_e1329_d_n10 * locals.var_cox);
        let eq11_e1331_d_n11: f64 = (eq11_e1329_d_n11 * locals.var_cox);
        let eq11_e1331_d_n12: f64 = (eq11_e1329_d_n12 * locals.var_cox);
        let eq11_e1331_d_n13: f64 = (eq11_e1329_d_n13 * locals.var_cox);
        let eq11_e1331_d_n14: f64 = (eq11_e1329_d_n14 * locals.var_cox);
        let eq11_e1333: f64 = (eq11_e1331 * locals.var_weff);
        let eq11_e1333_d_n0: f64 = (eq11_e1331_d_n0 * locals.var_weff);
        let eq11_e1333_d_n2: f64 = (eq11_e1331_d_n2 * locals.var_weff);
        let eq11_e1333_d_n3: f64 = (eq11_e1331_d_n3 * locals.var_weff);
        let eq11_e1333_d_n4: f64 = (eq11_e1331_d_n4 * locals.var_weff);
        let eq11_e1333_d_n5: f64 = (eq11_e1331_d_n5 * locals.var_weff);
        let eq11_e1333_d_n6: f64 = (eq11_e1331_d_n6 * locals.var_weff);
        let eq11_e1333_d_n7: f64 = (eq11_e1331_d_n7 * locals.var_weff);
        let eq11_e1333_d_n8: f64 = (eq11_e1331_d_n8 * locals.var_weff);
        let eq11_e1333_d_n9: f64 = (eq11_e1331_d_n9 * locals.var_weff);
        let eq11_e1333_d_n10: f64 = (eq11_e1331_d_n10 * locals.var_weff);
        let eq11_e1333_d_n11: f64 = (eq11_e1331_d_n11 * locals.var_weff);
        let eq11_e1333_d_n12: f64 = (eq11_e1331_d_n12 * locals.var_weff);
        let eq11_e1333_d_n13: f64 = (eq11_e1331_d_n13 * locals.var_weff);
        let eq11_e1333_d_n14: f64 = (eq11_e1331_d_n14 * locals.var_weff);
        let eq11_e1335: f64 = (eq11_e1333 * p.p2);
        let eq11_e1335_d_n0: f64 = (eq11_e1333_d_n0 * p.p2);
        let eq11_e1335_d_n2: f64 = (eq11_e1333_d_n2 * p.p2);
        let eq11_e1335_d_n3: f64 = (eq11_e1333_d_n3 * p.p2);
        let eq11_e1335_d_n4: f64 = (eq11_e1333_d_n4 * p.p2);
        let eq11_e1335_d_n5: f64 = (eq11_e1333_d_n5 * p.p2);
        let eq11_e1335_d_n6: f64 = (eq11_e1333_d_n6 * p.p2);
        let eq11_e1335_d_n7: f64 = (eq11_e1333_d_n7 * p.p2);
        let eq11_e1335_d_n8: f64 = (eq11_e1333_d_n8 * p.p2);
        let eq11_e1335_d_n9: f64 = (eq11_e1333_d_n9 * p.p2);
        let eq11_e1335_d_n10: f64 = (eq11_e1333_d_n10 * p.p2);
        let eq11_e1335_d_n11: f64 = (eq11_e1333_d_n11 * p.p2);
        let eq11_e1335_d_n12: f64 = (eq11_e1333_d_n12 * p.p2);
        let eq11_e1335_d_n13: f64 = (eq11_e1333_d_n13 * p.p2);
        let eq11_e1335_d_n14: f64 = (eq11_e1333_d_n14 * p.p2);
        let eq11_e1337: f64 = (eq11_e1335 * locals.var_leff);
        let eq11_e1337_d_n0: f64 = (eq11_e1335_d_n0 * locals.var_leff);
        let eq11_e1337_d_n2: f64 = (eq11_e1335_d_n2 * locals.var_leff);
        let eq11_e1337_d_n3: f64 = (eq11_e1335_d_n3 * locals.var_leff);
        let eq11_e1337_d_n4: f64 = (eq11_e1335_d_n4 * locals.var_leff);
        let eq11_e1337_d_n5: f64 = (eq11_e1335_d_n5 * locals.var_leff);
        let eq11_e1337_d_n6: f64 = (eq11_e1335_d_n6 * locals.var_leff);
        let eq11_e1337_d_n7: f64 = (eq11_e1335_d_n7 * locals.var_leff);
        let eq11_e1337_d_n8: f64 = (eq11_e1335_d_n8 * locals.var_leff);
        let eq11_e1337_d_n9: f64 = (eq11_e1335_d_n9 * locals.var_leff);
        let eq11_e1337_d_n10: f64 = (eq11_e1335_d_n10 * locals.var_leff);
        let eq11_e1337_d_n11: f64 = (eq11_e1335_d_n11 * locals.var_leff);
        let eq11_e1337_d_n12: f64 = (eq11_e1335_d_n12 * locals.var_leff);
        let eq11_e1337_d_n13: f64 = (eq11_e1335_d_n13 * locals.var_leff);
        let eq11_e1337_d_n14: f64 = (eq11_e1335_d_n14 * locals.var_leff);
        let eq11_e1339: f64 = (eq11_e1337 * (nv15 - 0.0));
        let eq11_e1339_d_n0: f64 = (eq11_e1337_d_n0 * (nv15 - 0.0));
        let eq11_e1339_d_n2: f64 = (eq11_e1337_d_n2 * (nv15 - 0.0));
        let eq11_e1339_d_n3: f64 = (eq11_e1337_d_n3 * (nv15 - 0.0));
        let eq11_e1339_d_n4: f64 = (eq11_e1337_d_n4 * (nv15 - 0.0));
        let eq11_e1339_d_n5: f64 = (eq11_e1337_d_n5 * (nv15 - 0.0));
        let eq11_e1339_d_n6: f64 = (eq11_e1337_d_n6 * (nv15 - 0.0));
        let eq11_e1339_d_n7: f64 = (eq11_e1337_d_n7 * (nv15 - 0.0));
        let eq11_e1339_d_n8: f64 = (eq11_e1337_d_n8 * (nv15 - 0.0));
        let eq11_e1339_d_n9: f64 = (eq11_e1337_d_n9 * (nv15 - 0.0));
        let eq11_e1339_d_n10: f64 = (eq11_e1337_d_n10 * (nv15 - 0.0));
        let eq11_e1339_d_n11: f64 = (eq11_e1337_d_n11 * (nv15 - 0.0));
        let eq11_e1339_d_n12: f64 = (eq11_e1337_d_n12 * (nv15 - 0.0));
        let eq11_e1339_d_n13: f64 = (eq11_e1337_d_n13 * (nv15 - 0.0));
        let eq11_e1339_d_n14: f64 = (eq11_e1337_d_n14 * (nv15 - 0.0));
        let eq11_e1340: f64 = (0.5 * eq11_e1339);
        let eq11_e1340_d_n0: f64 = (0.5 * eq11_e1339_d_n0);
        let eq11_e1340_d_n2: f64 = (0.5 * eq11_e1339_d_n2);
        let eq11_e1340_d_n3: f64 = (0.5 * eq11_e1339_d_n3);
        let eq11_e1340_d_n4: f64 = (0.5 * eq11_e1339_d_n4);
        let eq11_e1340_d_n5: f64 = (0.5 * eq11_e1339_d_n5);
        let eq11_e1340_d_n6: f64 = (0.5 * eq11_e1339_d_n6);
        let eq11_e1340_d_n7: f64 = (0.5 * eq11_e1339_d_n7);
        let eq11_e1340_d_n8: f64 = (0.5 * eq11_e1339_d_n8);
        let eq11_e1340_d_n9: f64 = (0.5 * eq11_e1339_d_n9);
        let eq11_e1340_d_n10: f64 = (0.5 * eq11_e1339_d_n10);
        let eq11_e1340_d_n11: f64 = (0.5 * eq11_e1339_d_n11);
        let eq11_e1340_d_n12: f64 = (0.5 * eq11_e1339_d_n12);
        let eq11_e1340_d_n13: f64 = (0.5 * eq11_e1339_d_n13);
        let eq11_e1340_d_n14: f64 = (0.5 * eq11_e1339_d_n14);
        let eq11_e1340_d_n15: f64 = (0.5 * eq11_e1337);
        let eq11_e1341: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, eq11_e1340);
        let eq11_e1342: f64 = (p.p29 * eq11_e1341);
        let eq11_e1342_d_n0: f64 = (p.p29 * (eq11_e1340_d_n0 * ddt_scale));
        let eq11_e1342_d_n2: f64 = (p.p29 * (eq11_e1340_d_n2 * ddt_scale));
        let eq11_e1342_d_n3: f64 = (p.p29 * (eq11_e1340_d_n3 * ddt_scale));
        let eq11_e1342_d_n4: f64 = (p.p29 * (eq11_e1340_d_n4 * ddt_scale));
        let eq11_e1342_d_n5: f64 = (p.p29 * (eq11_e1340_d_n5 * ddt_scale));
        let eq11_e1342_d_n6: f64 = (p.p29 * (eq11_e1340_d_n6 * ddt_scale));
        let eq11_e1342_d_n7: f64 = (p.p29 * (eq11_e1340_d_n7 * ddt_scale));
        let eq11_e1342_d_n8: f64 = (p.p29 * (eq11_e1340_d_n8 * ddt_scale));
        let eq11_e1342_d_n9: f64 = (p.p29 * (eq11_e1340_d_n9 * ddt_scale));
        let eq11_e1342_d_n10: f64 = (p.p29 * (eq11_e1340_d_n10 * ddt_scale));
        let eq11_e1342_d_n11: f64 = (p.p29 * (eq11_e1340_d_n11 * ddt_scale));
        let eq11_e1342_d_n12: f64 = (p.p29 * (eq11_e1340_d_n12 * ddt_scale));
        let eq11_e1342_d_n13: f64 = (p.p29 * (eq11_e1340_d_n13 * ddt_scale));
        let eq11_e1342_d_n14: f64 = (p.p29 * (eq11_e1340_d_n14 * ddt_scale));
        let eq11_e1342_d_n15: f64 = (p.p29 * (eq11_e1340_d_n15 * ddt_scale));
        (eq11_e1342, eq11_e1342_d_n0, eq11_e1342_d_n2, eq11_e1342_d_n3, eq11_e1342_d_n4, eq11_e1342_d_n5, eq11_e1342_d_n6, eq11_e1342_d_n7, eq11_e1342_d_n8, eq11_e1342_d_n9, eq11_e1342_d_n10, eq11_e1342_d_n11, eq11_e1342_d_n12, eq11_e1342_d_n13, eq11_e1342_d_n14, eq11_e1342_d_n15,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq11_value: f64 = eq11_e1344;
        let eq11_node_derivative_indices: [usize; 15] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        let eq11_node_derivatives: [f64; 15] = [eq11_e1344_d_n0, eq11_e1344_d_n2, eq11_e1344_d_n3, eq11_e1344_d_n4, eq11_e1344_d_n5, eq11_e1344_d_n6, eq11_e1344_d_n7, eq11_e1344_d_n8, eq11_e1344_d_n9, eq11_e1344_d_n10, eq11_e1344_d_n11, eq11_e1344_d_n12, eq11_e1344_d_n13, eq11_e1344_d_n14, eq11_e1344_d_n15];
        let eq11_branch_derivative_indices: [usize; 0] = [];
        let eq11_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(9),
            Some(7),
            multiplicity * (eq11_value),
            &eq11_node_derivative_indices,
            &eq11_node_derivatives,
            &eq11_branch_derivative_indices,
            &eq11_branch_derivatives,
            multiplicity,
        );
        let (eq12_e1370, eq12_e1370_d_n0, eq12_e1370_d_n2, eq12_e1370_d_n3, eq12_e1370_d_n4, eq12_e1370_d_n5, eq12_e1370_d_n6, eq12_e1370_d_n7, eq12_e1370_d_n8, eq12_e1370_d_n9, eq12_e1370_d_n10, eq12_e1370_d_n11, eq12_e1370_d_n12, eq12_e1370_d_n13, eq12_e1370_d_n14, eq12_e1370_d_n15,) = {
    if ((locals.var_guard698 != 0.0) && (locals.var_guard697 == 0.0)) {
        let eq12_e1353: f64 = (1.0 - locals.var_sigvds);
        let eq12_e1355: f64 = (eq12_e1353 * locals.var_mig);
        let eq12_e1355_d_n0: f64 = (eq12_e1353 * locals.var_mig_dn0);
        let eq12_e1355_d_n2: f64 = (eq12_e1353 * locals.var_mig_dn2);
        let eq12_e1355_d_n3: f64 = (eq12_e1353 * locals.var_mig_dn3);
        let eq12_e1355_d_n4: f64 = (eq12_e1353 * locals.var_mig_dn4);
        let eq12_e1355_d_n5: f64 = (eq12_e1353 * locals.var_mig_dn5);
        let eq12_e1355_d_n6: f64 = (eq12_e1353 * locals.var_mig_dn6);
        let eq12_e1355_d_n7: f64 = (eq12_e1353 * locals.var_mig_dn7);
        let eq12_e1355_d_n8: f64 = (eq12_e1353 * locals.var_mig_dn8);
        let eq12_e1355_d_n9: f64 = (eq12_e1353 * locals.var_mig_dn9);
        let eq12_e1355_d_n10: f64 = (eq12_e1353 * locals.var_mig_dn10);
        let eq12_e1355_d_n11: f64 = (eq12_e1353 * locals.var_mig_dn11);
        let eq12_e1355_d_n12: f64 = (eq12_e1353 * locals.var_mig_dn12);
        let eq12_e1355_d_n13: f64 = (eq12_e1353 * locals.var_mig_dn13);
        let eq12_e1355_d_n14: f64 = (eq12_e1353 * locals.var_mig_dn14);
        let eq12_e1357: f64 = (eq12_e1355 * locals.var_cox);
        let eq12_e1357_d_n0: f64 = (eq12_e1355_d_n0 * locals.var_cox);
        let eq12_e1357_d_n2: f64 = (eq12_e1355_d_n2 * locals.var_cox);
        let eq12_e1357_d_n3: f64 = (eq12_e1355_d_n3 * locals.var_cox);
        let eq12_e1357_d_n4: f64 = (eq12_e1355_d_n4 * locals.var_cox);
        let eq12_e1357_d_n5: f64 = (eq12_e1355_d_n5 * locals.var_cox);
        let eq12_e1357_d_n6: f64 = (eq12_e1355_d_n6 * locals.var_cox);
        let eq12_e1357_d_n7: f64 = (eq12_e1355_d_n7 * locals.var_cox);
        let eq12_e1357_d_n8: f64 = (eq12_e1355_d_n8 * locals.var_cox);
        let eq12_e1357_d_n9: f64 = (eq12_e1355_d_n9 * locals.var_cox);
        let eq12_e1357_d_n10: f64 = (eq12_e1355_d_n10 * locals.var_cox);
        let eq12_e1357_d_n11: f64 = (eq12_e1355_d_n11 * locals.var_cox);
        let eq12_e1357_d_n12: f64 = (eq12_e1355_d_n12 * locals.var_cox);
        let eq12_e1357_d_n13: f64 = (eq12_e1355_d_n13 * locals.var_cox);
        let eq12_e1357_d_n14: f64 = (eq12_e1355_d_n14 * locals.var_cox);
        let eq12_e1359: f64 = (eq12_e1357 * locals.var_weff);
        let eq12_e1359_d_n0: f64 = (eq12_e1357_d_n0 * locals.var_weff);
        let eq12_e1359_d_n2: f64 = (eq12_e1357_d_n2 * locals.var_weff);
        let eq12_e1359_d_n3: f64 = (eq12_e1357_d_n3 * locals.var_weff);
        let eq12_e1359_d_n4: f64 = (eq12_e1357_d_n4 * locals.var_weff);
        let eq12_e1359_d_n5: f64 = (eq12_e1357_d_n5 * locals.var_weff);
        let eq12_e1359_d_n6: f64 = (eq12_e1357_d_n6 * locals.var_weff);
        let eq12_e1359_d_n7: f64 = (eq12_e1357_d_n7 * locals.var_weff);
        let eq12_e1359_d_n8: f64 = (eq12_e1357_d_n8 * locals.var_weff);
        let eq12_e1359_d_n9: f64 = (eq12_e1357_d_n9 * locals.var_weff);
        let eq12_e1359_d_n10: f64 = (eq12_e1357_d_n10 * locals.var_weff);
        let eq12_e1359_d_n11: f64 = (eq12_e1357_d_n11 * locals.var_weff);
        let eq12_e1359_d_n12: f64 = (eq12_e1357_d_n12 * locals.var_weff);
        let eq12_e1359_d_n13: f64 = (eq12_e1357_d_n13 * locals.var_weff);
        let eq12_e1359_d_n14: f64 = (eq12_e1357_d_n14 * locals.var_weff);
        let eq12_e1361: f64 = (eq12_e1359 * p.p2);
        let eq12_e1361_d_n0: f64 = (eq12_e1359_d_n0 * p.p2);
        let eq12_e1361_d_n2: f64 = (eq12_e1359_d_n2 * p.p2);
        let eq12_e1361_d_n3: f64 = (eq12_e1359_d_n3 * p.p2);
        let eq12_e1361_d_n4: f64 = (eq12_e1359_d_n4 * p.p2);
        let eq12_e1361_d_n5: f64 = (eq12_e1359_d_n5 * p.p2);
        let eq12_e1361_d_n6: f64 = (eq12_e1359_d_n6 * p.p2);
        let eq12_e1361_d_n7: f64 = (eq12_e1359_d_n7 * p.p2);
        let eq12_e1361_d_n8: f64 = (eq12_e1359_d_n8 * p.p2);
        let eq12_e1361_d_n9: f64 = (eq12_e1359_d_n9 * p.p2);
        let eq12_e1361_d_n10: f64 = (eq12_e1359_d_n10 * p.p2);
        let eq12_e1361_d_n11: f64 = (eq12_e1359_d_n11 * p.p2);
        let eq12_e1361_d_n12: f64 = (eq12_e1359_d_n12 * p.p2);
        let eq12_e1361_d_n13: f64 = (eq12_e1359_d_n13 * p.p2);
        let eq12_e1361_d_n14: f64 = (eq12_e1359_d_n14 * p.p2);
        let eq12_e1363: f64 = (eq12_e1361 * locals.var_leff);
        let eq12_e1363_d_n0: f64 = (eq12_e1361_d_n0 * locals.var_leff);
        let eq12_e1363_d_n2: f64 = (eq12_e1361_d_n2 * locals.var_leff);
        let eq12_e1363_d_n3: f64 = (eq12_e1361_d_n3 * locals.var_leff);
        let eq12_e1363_d_n4: f64 = (eq12_e1361_d_n4 * locals.var_leff);
        let eq12_e1363_d_n5: f64 = (eq12_e1361_d_n5 * locals.var_leff);
        let eq12_e1363_d_n6: f64 = (eq12_e1361_d_n6 * locals.var_leff);
        let eq12_e1363_d_n7: f64 = (eq12_e1361_d_n7 * locals.var_leff);
        let eq12_e1363_d_n8: f64 = (eq12_e1361_d_n8 * locals.var_leff);
        let eq12_e1363_d_n9: f64 = (eq12_e1361_d_n9 * locals.var_leff);
        let eq12_e1363_d_n10: f64 = (eq12_e1361_d_n10 * locals.var_leff);
        let eq12_e1363_d_n11: f64 = (eq12_e1361_d_n11 * locals.var_leff);
        let eq12_e1363_d_n12: f64 = (eq12_e1361_d_n12 * locals.var_leff);
        let eq12_e1363_d_n13: f64 = (eq12_e1361_d_n13 * locals.var_leff);
        let eq12_e1363_d_n14: f64 = (eq12_e1361_d_n14 * locals.var_leff);
        let eq12_e1365: f64 = (eq12_e1363 * (nv15 - 0.0));
        let eq12_e1365_d_n0: f64 = (eq12_e1363_d_n0 * (nv15 - 0.0));
        let eq12_e1365_d_n2: f64 = (eq12_e1363_d_n2 * (nv15 - 0.0));
        let eq12_e1365_d_n3: f64 = (eq12_e1363_d_n3 * (nv15 - 0.0));
        let eq12_e1365_d_n4: f64 = (eq12_e1363_d_n4 * (nv15 - 0.0));
        let eq12_e1365_d_n5: f64 = (eq12_e1363_d_n5 * (nv15 - 0.0));
        let eq12_e1365_d_n6: f64 = (eq12_e1363_d_n6 * (nv15 - 0.0));
        let eq12_e1365_d_n7: f64 = (eq12_e1363_d_n7 * (nv15 - 0.0));
        let eq12_e1365_d_n8: f64 = (eq12_e1363_d_n8 * (nv15 - 0.0));
        let eq12_e1365_d_n9: f64 = (eq12_e1363_d_n9 * (nv15 - 0.0));
        let eq12_e1365_d_n10: f64 = (eq12_e1363_d_n10 * (nv15 - 0.0));
        let eq12_e1365_d_n11: f64 = (eq12_e1363_d_n11 * (nv15 - 0.0));
        let eq12_e1365_d_n12: f64 = (eq12_e1363_d_n12 * (nv15 - 0.0));
        let eq12_e1365_d_n13: f64 = (eq12_e1363_d_n13 * (nv15 - 0.0));
        let eq12_e1365_d_n14: f64 = (eq12_e1363_d_n14 * (nv15 - 0.0));
        let eq12_e1366: f64 = (0.5 * eq12_e1365);
        let eq12_e1366_d_n0: f64 = (0.5 * eq12_e1365_d_n0);
        let eq12_e1366_d_n2: f64 = (0.5 * eq12_e1365_d_n2);
        let eq12_e1366_d_n3: f64 = (0.5 * eq12_e1365_d_n3);
        let eq12_e1366_d_n4: f64 = (0.5 * eq12_e1365_d_n4);
        let eq12_e1366_d_n5: f64 = (0.5 * eq12_e1365_d_n5);
        let eq12_e1366_d_n6: f64 = (0.5 * eq12_e1365_d_n6);
        let eq12_e1366_d_n7: f64 = (0.5 * eq12_e1365_d_n7);
        let eq12_e1366_d_n8: f64 = (0.5 * eq12_e1365_d_n8);
        let eq12_e1366_d_n9: f64 = (0.5 * eq12_e1365_d_n9);
        let eq12_e1366_d_n10: f64 = (0.5 * eq12_e1365_d_n10);
        let eq12_e1366_d_n11: f64 = (0.5 * eq12_e1365_d_n11);
        let eq12_e1366_d_n12: f64 = (0.5 * eq12_e1365_d_n12);
        let eq12_e1366_d_n13: f64 = (0.5 * eq12_e1365_d_n13);
        let eq12_e1366_d_n14: f64 = (0.5 * eq12_e1365_d_n14);
        let eq12_e1366_d_n15: f64 = (0.5 * eq12_e1363);
        let eq12_e1367: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, eq12_e1366);
        let eq12_e1368: f64 = (p.p29 * eq12_e1367);
        let eq12_e1368_d_n0: f64 = (p.p29 * (eq12_e1366_d_n0 * ddt_scale));
        let eq12_e1368_d_n2: f64 = (p.p29 * (eq12_e1366_d_n2 * ddt_scale));
        let eq12_e1368_d_n3: f64 = (p.p29 * (eq12_e1366_d_n3 * ddt_scale));
        let eq12_e1368_d_n4: f64 = (p.p29 * (eq12_e1366_d_n4 * ddt_scale));
        let eq12_e1368_d_n5: f64 = (p.p29 * (eq12_e1366_d_n5 * ddt_scale));
        let eq12_e1368_d_n6: f64 = (p.p29 * (eq12_e1366_d_n6 * ddt_scale));
        let eq12_e1368_d_n7: f64 = (p.p29 * (eq12_e1366_d_n7 * ddt_scale));
        let eq12_e1368_d_n8: f64 = (p.p29 * (eq12_e1366_d_n8 * ddt_scale));
        let eq12_e1368_d_n9: f64 = (p.p29 * (eq12_e1366_d_n9 * ddt_scale));
        let eq12_e1368_d_n10: f64 = (p.p29 * (eq12_e1366_d_n10 * ddt_scale));
        let eq12_e1368_d_n11: f64 = (p.p29 * (eq12_e1366_d_n11 * ddt_scale));
        let eq12_e1368_d_n12: f64 = (p.p29 * (eq12_e1366_d_n12 * ddt_scale));
        let eq12_e1368_d_n13: f64 = (p.p29 * (eq12_e1366_d_n13 * ddt_scale));
        let eq12_e1368_d_n14: f64 = (p.p29 * (eq12_e1366_d_n14 * ddt_scale));
        let eq12_e1368_d_n15: f64 = (p.p29 * (eq12_e1366_d_n15 * ddt_scale));
        (eq12_e1368, eq12_e1368_d_n0, eq12_e1368_d_n2, eq12_e1368_d_n3, eq12_e1368_d_n4, eq12_e1368_d_n5, eq12_e1368_d_n6, eq12_e1368_d_n7, eq12_e1368_d_n8, eq12_e1368_d_n9, eq12_e1368_d_n10, eq12_e1368_d_n11, eq12_e1368_d_n12, eq12_e1368_d_n13, eq12_e1368_d_n14, eq12_e1368_d_n15,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq12_value: f64 = eq12_e1370;
        let eq12_node_derivative_indices: [usize; 15] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        let eq12_node_derivatives: [f64; 15] = [eq12_e1370_d_n0, eq12_e1370_d_n2, eq12_e1370_d_n3, eq12_e1370_d_n4, eq12_e1370_d_n5, eq12_e1370_d_n6, eq12_e1370_d_n7, eq12_e1370_d_n8, eq12_e1370_d_n9, eq12_e1370_d_n10, eq12_e1370_d_n11, eq12_e1370_d_n12, eq12_e1370_d_n13, eq12_e1370_d_n14, eq12_e1370_d_n15];
        let eq12_branch_derivative_indices: [usize; 0] = [];
        let eq12_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(9),
            Some(5),
            multiplicity * (eq12_value),
            &eq12_node_derivative_indices,
            &eq12_node_derivatives,
            &eq12_branch_derivative_indices,
            &eq12_branch_derivatives,
            multiplicity,
        );
        let eq19_e1428: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, locals.var_qgi_1);
        let eq19_value: f64 = eq19_e1428;
        let eq19_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq19_node_derivatives: [f64; 14] = [(locals.var_qgi_1_dn0 * ddt_scale), (locals.var_qgi_1_dn2 * ddt_scale), (locals.var_qgi_1_dn3 * ddt_scale), (locals.var_qgi_1_dn4 * ddt_scale), (locals.var_qgi_1_dn5 * ddt_scale), (locals.var_qgi_1_dn6 * ddt_scale), (locals.var_qgi_1_dn7 * ddt_scale), (locals.var_qgi_1_dn8 * ddt_scale), (locals.var_qgi_1_dn9 * ddt_scale), (locals.var_qgi_1_dn10 * ddt_scale), (locals.var_qgi_1_dn11 * ddt_scale), (locals.var_qgi_1_dn12 * ddt_scale), (locals.var_qgi_1_dn13 * ddt_scale), (locals.var_qgi_1_dn14 * ddt_scale)];
        let eq19_branch_derivative_indices: [usize; 0] = [];
        let eq19_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(9),
            Some(11),
            multiplicity * (eq19_value),
            &eq19_node_derivative_indices,
            &eq19_node_derivatives,
            &eq19_branch_derivative_indices,
            &eq19_branch_derivatives,
            multiplicity,
        );
        let eq20_e1430: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, locals.var_qsi_1);
        let eq20_value: f64 = eq20_e1430;
        let eq20_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq20_node_derivatives: [f64; 14] = [(locals.var_qsi_1_dn0 * ddt_scale), (locals.var_qsi_1_dn2 * ddt_scale), (locals.var_qsi_1_dn3 * ddt_scale), (locals.var_qsi_1_dn4 * ddt_scale), (locals.var_qsi_1_dn5 * ddt_scale), (locals.var_qsi_1_dn6 * ddt_scale), (locals.var_qsi_1_dn7 * ddt_scale), (locals.var_qsi_1_dn8 * ddt_scale), (locals.var_qsi_1_dn9 * ddt_scale), (locals.var_qsi_1_dn10 * ddt_scale), (locals.var_qsi_1_dn11 * ddt_scale), (locals.var_qsi_1_dn12 * ddt_scale), (locals.var_qsi_1_dn13 * ddt_scale), (locals.var_qsi_1_dn14 * ddt_scale)];
        let eq20_branch_derivative_indices: [usize; 0] = [];
        let eq20_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(7),
            Some(11),
            multiplicity * (eq20_value),
            &eq20_node_derivative_indices,
            &eq20_node_derivatives,
            &eq20_branch_derivative_indices,
            &eq20_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_1(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let eq21_e1432: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, locals.var_qdi_1);
        let eq21_value: f64 = eq21_e1432;
        let eq21_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq21_node_derivatives: [f64; 14] = [(locals.var_qdi_1_dn0 * ddt_scale), (locals.var_qdi_1_dn2 * ddt_scale), (locals.var_qdi_1_dn3 * ddt_scale), (locals.var_qdi_1_dn4 * ddt_scale), (locals.var_qdi_1_dn5 * ddt_scale), (locals.var_qdi_1_dn6 * ddt_scale), (locals.var_qdi_1_dn7 * ddt_scale), (locals.var_qdi_1_dn8 * ddt_scale), (locals.var_qdi_1_dn9 * ddt_scale), (locals.var_qdi_1_dn10 * ddt_scale), (locals.var_qdi_1_dn11 * ddt_scale), (locals.var_qdi_1_dn12 * ddt_scale), (locals.var_qdi_1_dn13 * ddt_scale), (locals.var_qdi_1_dn14 * ddt_scale)];
        let eq21_branch_derivative_indices: [usize; 0] = [];
        let eq21_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(5),
            Some(11),
            multiplicity * (eq21_value),
            &eq21_node_derivative_indices,
            &eq21_node_derivatives,
            &eq21_branch_derivative_indices,
            &eq21_branch_derivatives,
            multiplicity,
        );
        let eq22_e1435: f64 = (-locals.var_devsign);
        let eq22_e1437: f64 = (eq22_e1435 * locals.var_qovs);
        let eq22_e1437_d_n0: f64 = (eq22_e1435 * locals.var_qovs_dn0);
        let eq22_e1437_d_n2: f64 = (eq22_e1435 * locals.var_qovs_dn2);
        let eq22_e1437_d_n3: f64 = (eq22_e1435 * locals.var_qovs_dn3);
        let eq22_e1437_d_n4: f64 = (eq22_e1435 * locals.var_qovs_dn4);
        let eq22_e1437_d_n5: f64 = (eq22_e1435 * locals.var_qovs_dn5);
        let eq22_e1437_d_n6: f64 = (eq22_e1435 * locals.var_qovs_dn6);
        let eq22_e1437_d_n7: f64 = (eq22_e1435 * locals.var_qovs_dn7);
        let eq22_e1437_d_n8: f64 = (eq22_e1435 * locals.var_qovs_dn8);
        let eq22_e1437_d_n9: f64 = (eq22_e1435 * locals.var_qovs_dn9);
        let eq22_e1437_d_n10: f64 = (eq22_e1435 * locals.var_qovs_dn10);
        let eq22_e1437_d_n11: f64 = (eq22_e1435 * locals.var_qovs_dn11);
        let eq22_e1437_d_n12: f64 = (eq22_e1435 * locals.var_qovs_dn12);
        let eq22_e1437_d_n13: f64 = (eq22_e1435 * locals.var_qovs_dn13);
        let eq22_e1437_d_n14: f64 = (eq22_e1435 * locals.var_qovs_dn14);
        let eq22_e1438: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, eq22_e1437);
        let eq22_e1439: f64 = (p.p29 * eq22_e1438);
        let eq22_e1439_d_n0: f64 = (p.p29 * (eq22_e1437_d_n0 * ddt_scale));
        let eq22_e1439_d_n2: f64 = (p.p29 * (eq22_e1437_d_n2 * ddt_scale));
        let eq22_e1439_d_n3: f64 = (p.p29 * (eq22_e1437_d_n3 * ddt_scale));
        let eq22_e1439_d_n4: f64 = (p.p29 * (eq22_e1437_d_n4 * ddt_scale));
        let eq22_e1439_d_n5: f64 = (p.p29 * (eq22_e1437_d_n5 * ddt_scale));
        let eq22_e1439_d_n6: f64 = (p.p29 * (eq22_e1437_d_n6 * ddt_scale));
        let eq22_e1439_d_n7: f64 = (p.p29 * (eq22_e1437_d_n7 * ddt_scale));
        let eq22_e1439_d_n8: f64 = (p.p29 * (eq22_e1437_d_n8 * ddt_scale));
        let eq22_e1439_d_n9: f64 = (p.p29 * (eq22_e1437_d_n9 * ddt_scale));
        let eq22_e1439_d_n10: f64 = (p.p29 * (eq22_e1437_d_n10 * ddt_scale));
        let eq22_e1439_d_n11: f64 = (p.p29 * (eq22_e1437_d_n11 * ddt_scale));
        let eq22_e1439_d_n12: f64 = (p.p29 * (eq22_e1437_d_n12 * ddt_scale));
        let eq22_e1439_d_n13: f64 = (p.p29 * (eq22_e1437_d_n13 * ddt_scale));
        let eq22_e1439_d_n14: f64 = (p.p29 * (eq22_e1437_d_n14 * ddt_scale));
        let eq22_value: f64 = eq22_e1439;
        let eq22_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq22_node_derivatives: [f64; 14] = [eq22_e1439_d_n0, eq22_e1439_d_n2, eq22_e1439_d_n3, eq22_e1439_d_n4, eq22_e1439_d_n5, eq22_e1439_d_n6, eq22_e1439_d_n7, eq22_e1439_d_n8, eq22_e1439_d_n9, eq22_e1439_d_n10, eq22_e1439_d_n11, eq22_e1439_d_n12, eq22_e1439_d_n13, eq22_e1439_d_n14];
        let eq22_branch_derivative_indices: [usize; 0] = [];
        let eq22_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(10),
            Some(7),
            multiplicity * (eq22_value),
            &eq22_node_derivative_indices,
            &eq22_node_derivatives,
            &eq22_branch_derivative_indices,
            &eq22_branch_derivatives,
            multiplicity,
        );
        let eq23_e1442: f64 = (-locals.var_devsign);
        let eq23_e1444: f64 = (eq23_e1442 * locals.var_qovd);
        let eq23_e1444_d_n0: f64 = (eq23_e1442 * locals.var_qovd_dn0);
        let eq23_e1444_d_n2: f64 = (eq23_e1442 * locals.var_qovd_dn2);
        let eq23_e1444_d_n3: f64 = (eq23_e1442 * locals.var_qovd_dn3);
        let eq23_e1444_d_n4: f64 = (eq23_e1442 * locals.var_qovd_dn4);
        let eq23_e1444_d_n5: f64 = (eq23_e1442 * locals.var_qovd_dn5);
        let eq23_e1444_d_n6: f64 = (eq23_e1442 * locals.var_qovd_dn6);
        let eq23_e1444_d_n7: f64 = (eq23_e1442 * locals.var_qovd_dn7);
        let eq23_e1444_d_n8: f64 = (eq23_e1442 * locals.var_qovd_dn8);
        let eq23_e1444_d_n9: f64 = (eq23_e1442 * locals.var_qovd_dn9);
        let eq23_e1444_d_n10: f64 = (eq23_e1442 * locals.var_qovd_dn10);
        let eq23_e1444_d_n11: f64 = (eq23_e1442 * locals.var_qovd_dn11);
        let eq23_e1444_d_n12: f64 = (eq23_e1442 * locals.var_qovd_dn12);
        let eq23_e1444_d_n13: f64 = (eq23_e1442 * locals.var_qovd_dn13);
        let eq23_e1444_d_n14: f64 = (eq23_e1442 * locals.var_qovd_dn14);
        let eq23_e1445: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, eq23_e1444);
        let eq23_e1446: f64 = (p.p29 * eq23_e1445);
        let eq23_e1446_d_n0: f64 = (p.p29 * (eq23_e1444_d_n0 * ddt_scale));
        let eq23_e1446_d_n2: f64 = (p.p29 * (eq23_e1444_d_n2 * ddt_scale));
        let eq23_e1446_d_n3: f64 = (p.p29 * (eq23_e1444_d_n3 * ddt_scale));
        let eq23_e1446_d_n4: f64 = (p.p29 * (eq23_e1444_d_n4 * ddt_scale));
        let eq23_e1446_d_n5: f64 = (p.p29 * (eq23_e1444_d_n5 * ddt_scale));
        let eq23_e1446_d_n6: f64 = (p.p29 * (eq23_e1444_d_n6 * ddt_scale));
        let eq23_e1446_d_n7: f64 = (p.p29 * (eq23_e1444_d_n7 * ddt_scale));
        let eq23_e1446_d_n8: f64 = (p.p29 * (eq23_e1444_d_n8 * ddt_scale));
        let eq23_e1446_d_n9: f64 = (p.p29 * (eq23_e1444_d_n9 * ddt_scale));
        let eq23_e1446_d_n10: f64 = (p.p29 * (eq23_e1444_d_n10 * ddt_scale));
        let eq23_e1446_d_n11: f64 = (p.p29 * (eq23_e1444_d_n11 * ddt_scale));
        let eq23_e1446_d_n12: f64 = (p.p29 * (eq23_e1444_d_n12 * ddt_scale));
        let eq23_e1446_d_n13: f64 = (p.p29 * (eq23_e1444_d_n13 * ddt_scale));
        let eq23_e1446_d_n14: f64 = (p.p29 * (eq23_e1444_d_n14 * ddt_scale));
        let eq23_value: f64 = eq23_e1446;
        let eq23_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq23_node_derivatives: [f64; 14] = [eq23_e1446_d_n0, eq23_e1446_d_n2, eq23_e1446_d_n3, eq23_e1446_d_n4, eq23_e1446_d_n5, eq23_e1446_d_n6, eq23_e1446_d_n7, eq23_e1446_d_n8, eq23_e1446_d_n9, eq23_e1446_d_n10, eq23_e1446_d_n11, eq23_e1446_d_n12, eq23_e1446_d_n13, eq23_e1446_d_n14];
        let eq23_branch_derivative_indices: [usize; 0] = [];
        let eq23_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(10),
            Some(5),
            multiplicity * (eq23_value),
            &eq23_node_derivative_indices,
            &eq23_node_derivatives,
            &eq23_branch_derivative_indices,
            &eq23_branch_derivatives,
            multiplicity,
        );
        let eq24_e1449: f64 = (-locals.var_devsign);
        let eq24_e1451: f64 = (eq24_e1449 * locals.var_qovb);
        let eq24_e1451_d_n0: f64 = (eq24_e1449 * locals.var_qovb_dn0);
        let eq24_e1451_d_n2: f64 = (eq24_e1449 * locals.var_qovb_dn2);
        let eq24_e1451_d_n3: f64 = (eq24_e1449 * locals.var_qovb_dn3);
        let eq24_e1451_d_n4: f64 = (eq24_e1449 * locals.var_qovb_dn4);
        let eq24_e1451_d_n5: f64 = (eq24_e1449 * locals.var_qovb_dn5);
        let eq24_e1451_d_n6: f64 = (eq24_e1449 * locals.var_qovb_dn6);
        let eq24_e1451_d_n7: f64 = (eq24_e1449 * locals.var_qovb_dn7);
        let eq24_e1451_d_n8: f64 = (eq24_e1449 * locals.var_qovb_dn8);
        let eq24_e1451_d_n9: f64 = (eq24_e1449 * locals.var_qovb_dn9);
        let eq24_e1451_d_n10: f64 = (eq24_e1449 * locals.var_qovb_dn10);
        let eq24_e1451_d_n11: f64 = (eq24_e1449 * locals.var_qovb_dn11);
        let eq24_e1451_d_n12: f64 = (eq24_e1449 * locals.var_qovb_dn12);
        let eq24_e1451_d_n13: f64 = (eq24_e1449 * locals.var_qovb_dn13);
        let eq24_e1451_d_n14: f64 = (eq24_e1449 * locals.var_qovb_dn14);
        let eq24_e1452: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, eq24_e1451);
        let eq24_e1453: f64 = (p.p29 * eq24_e1452);
        let eq24_e1453_d_n0: f64 = (p.p29 * (eq24_e1451_d_n0 * ddt_scale));
        let eq24_e1453_d_n2: f64 = (p.p29 * (eq24_e1451_d_n2 * ddt_scale));
        let eq24_e1453_d_n3: f64 = (p.p29 * (eq24_e1451_d_n3 * ddt_scale));
        let eq24_e1453_d_n4: f64 = (p.p29 * (eq24_e1451_d_n4 * ddt_scale));
        let eq24_e1453_d_n5: f64 = (p.p29 * (eq24_e1451_d_n5 * ddt_scale));
        let eq24_e1453_d_n6: f64 = (p.p29 * (eq24_e1451_d_n6 * ddt_scale));
        let eq24_e1453_d_n7: f64 = (p.p29 * (eq24_e1451_d_n7 * ddt_scale));
        let eq24_e1453_d_n8: f64 = (p.p29 * (eq24_e1451_d_n8 * ddt_scale));
        let eq24_e1453_d_n9: f64 = (p.p29 * (eq24_e1451_d_n9 * ddt_scale));
        let eq24_e1453_d_n10: f64 = (p.p29 * (eq24_e1451_d_n10 * ddt_scale));
        let eq24_e1453_d_n11: f64 = (p.p29 * (eq24_e1451_d_n11 * ddt_scale));
        let eq24_e1453_d_n12: f64 = (p.p29 * (eq24_e1451_d_n12 * ddt_scale));
        let eq24_e1453_d_n13: f64 = (p.p29 * (eq24_e1451_d_n13 * ddt_scale));
        let eq24_e1453_d_n14: f64 = (p.p29 * (eq24_e1451_d_n14 * ddt_scale));
        let eq24_value: f64 = eq24_e1453;
        let eq24_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq24_node_derivatives: [f64; 14] = [eq24_e1453_d_n0, eq24_e1453_d_n2, eq24_e1453_d_n3, eq24_e1453_d_n4, eq24_e1453_d_n5, eq24_e1453_d_n6, eq24_e1453_d_n7, eq24_e1453_d_n8, eq24_e1453_d_n9, eq24_e1453_d_n10, eq24_e1453_d_n11, eq24_e1453_d_n12, eq24_e1453_d_n13, eq24_e1453_d_n14];
        let eq24_branch_derivative_indices: [usize; 0] = [];
        let eq24_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(10),
            Some(11),
            multiplicity * (eq24_value),
            &eq24_node_derivative_indices,
            &eq24_node_derivatives,
            &eq24_branch_derivative_indices,
            &eq24_branch_derivatives,
            multiplicity,
        );
        let eq25_e1456: f64 = (locals.var_devsign * p.p28);
        let eq25_e1458: f64 = (eq25_e1456 * locals.var_sigvds);
        let eq25_e1460: f64 = (eq25_e1458 * locals.var_ids);
        let eq25_e1460_d_n0: f64 = (eq25_e1458 * locals.var_ids_dn0);
        let eq25_e1460_d_n2: f64 = (eq25_e1458 * locals.var_ids_dn2);
        let eq25_e1460_d_n3: f64 = (eq25_e1458 * locals.var_ids_dn3);
        let eq25_e1460_d_n4: f64 = (eq25_e1458 * locals.var_ids_dn4);
        let eq25_e1460_d_n5: f64 = (eq25_e1458 * locals.var_ids_dn5);
        let eq25_e1460_d_n6: f64 = (eq25_e1458 * locals.var_ids_dn6);
        let eq25_e1460_d_n7: f64 = (eq25_e1458 * locals.var_ids_dn7);
        let eq25_e1460_d_n8: f64 = (eq25_e1458 * locals.var_ids_dn8);
        let eq25_e1460_d_n9: f64 = (eq25_e1458 * locals.var_ids_dn9);
        let eq25_e1460_d_n10: f64 = (eq25_e1458 * locals.var_ids_dn10);
        let eq25_e1460_d_n11: f64 = (eq25_e1458 * locals.var_ids_dn11);
        let eq25_e1460_d_n12: f64 = (eq25_e1458 * locals.var_ids_dn12);
        let eq25_e1460_d_n13: f64 = (eq25_e1458 * locals.var_ids_dn13);
        let eq25_e1460_d_n14: f64 = (eq25_e1458 * locals.var_ids_dn14);
        let eq25_value: f64 = eq25_e1460;
        let eq25_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq25_node_derivatives: [f64; 14] = [eq25_e1460_d_n0, eq25_e1460_d_n2, eq25_e1460_d_n3, eq25_e1460_d_n4, eq25_e1460_d_n5, eq25_e1460_d_n6, eq25_e1460_d_n7, eq25_e1460_d_n8, eq25_e1460_d_n9, eq25_e1460_d_n10, eq25_e1460_d_n11, eq25_e1460_d_n12, eq25_e1460_d_n13, eq25_e1460_d_n14];
        let eq25_branch_derivative_indices: [usize; 0] = [];
        let eq25_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(5),
            Some(7),
            multiplicity * (eq25_value),
            &eq25_node_derivative_indices,
            &eq25_node_derivatives,
            &eq25_branch_derivative_indices,
            &eq25_branch_derivatives,
            multiplicity,
        );
        let (eq34_e1514, eq34_e1514_d_n0, eq34_e1514_d_n2, eq34_e1514_d_n3, eq34_e1514_d_n4, eq34_e1514_d_n5, eq34_e1514_d_n6, eq34_e1514_d_n7, eq34_e1514_d_n8, eq34_e1514_d_n9, eq34_e1514_d_n10, eq34_e1514_d_n11, eq34_e1514_d_n12, eq34_e1514_d_n13, eq34_e1514_d_n14,) = {
    if (locals.var_guard754 != 0.0) {
        let eq34_e1510: f64 = (p.p28 * (nv0 - nv6));
        let eq34_e1512: f64 = (eq34_e1510 * locals.var_gdpr);
        let eq34_e1512_d_n0: f64 = ((p.p28 * locals.var_gdpr) + (eq34_e1510 * locals.var_gdpr_dn0));
        let eq34_e1512_d_n2: f64 = (eq34_e1510 * locals.var_gdpr_dn2);
        let eq34_e1512_d_n3: f64 = (eq34_e1510 * locals.var_gdpr_dn3);
        let eq34_e1512_d_n4: f64 = (eq34_e1510 * locals.var_gdpr_dn4);
        let eq34_e1512_d_n5: f64 = (eq34_e1510 * locals.var_gdpr_dn5);
        let eq34_e1512_d_n6: f64 = (((-p.p28) * locals.var_gdpr) + (eq34_e1510 * locals.var_gdpr_dn6));
        let eq34_e1512_d_n7: f64 = (eq34_e1510 * locals.var_gdpr_dn7);
        let eq34_e1512_d_n8: f64 = (eq34_e1510 * locals.var_gdpr_dn8);
        let eq34_e1512_d_n9: f64 = (eq34_e1510 * locals.var_gdpr_dn9);
        let eq34_e1512_d_n10: f64 = (eq34_e1510 * locals.var_gdpr_dn10);
        let eq34_e1512_d_n11: f64 = (eq34_e1510 * locals.var_gdpr_dn11);
        let eq34_e1512_d_n12: f64 = (eq34_e1510 * locals.var_gdpr_dn12);
        let eq34_e1512_d_n13: f64 = (eq34_e1510 * locals.var_gdpr_dn13);
        let eq34_e1512_d_n14: f64 = (eq34_e1510 * locals.var_gdpr_dn14);
        (eq34_e1512, eq34_e1512_d_n0, eq34_e1512_d_n2, eq34_e1512_d_n3, eq34_e1512_d_n4, eq34_e1512_d_n5, eq34_e1512_d_n6, eq34_e1512_d_n7, eq34_e1512_d_n8, eq34_e1512_d_n9, eq34_e1512_d_n10, eq34_e1512_d_n11, eq34_e1512_d_n12, eq34_e1512_d_n13, eq34_e1512_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq34_value: f64 = eq34_e1514;
        let eq34_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq34_node_derivatives: [f64; 14] = [eq34_e1514_d_n0, eq34_e1514_d_n2, eq34_e1514_d_n3, eq34_e1514_d_n4, eq34_e1514_d_n5, eq34_e1514_d_n6, eq34_e1514_d_n7, eq34_e1514_d_n8, eq34_e1514_d_n9, eq34_e1514_d_n10, eq34_e1514_d_n11, eq34_e1514_d_n12, eq34_e1514_d_n13, eq34_e1514_d_n14];
        let eq34_branch_derivative_indices: [usize; 0] = [];
        let eq34_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(0),
            Some(6),
            multiplicity * (eq34_value),
            &eq34_node_derivative_indices,
            &eq34_node_derivatives,
            &eq34_branch_derivative_indices,
            &eq34_branch_derivatives,
            multiplicity,
        );
        let (eq36_e1534, eq36_e1534_d_n0, eq36_e1534_d_n2, eq36_e1534_d_n3, eq36_e1534_d_n4, eq36_e1534_d_n5, eq36_e1534_d_n6, eq36_e1534_d_n7, eq36_e1534_d_n8, eq36_e1534_d_n9, eq36_e1534_d_n10, eq36_e1534_d_n11, eq36_e1534_d_n12, eq36_e1534_d_n13, eq36_e1534_d_n14,) = {
    if ((locals.var_guard754 != 0.0) && (locals.var_guard755 != 0.0)) {
        let eq36_e1530: f64 = (p.p28 * (nv6 - nv5));
        let eq36_e1532: f64 = (eq36_e1530 * locals.var_gdrift_d);
        let eq36_e1532_d_n0: f64 = (eq36_e1530 * locals.var_gdrift_d_dn0);
        let eq36_e1532_d_n2: f64 = (eq36_e1530 * locals.var_gdrift_d_dn2);
        let eq36_e1532_d_n3: f64 = (eq36_e1530 * locals.var_gdrift_d_dn3);
        let eq36_e1532_d_n4: f64 = (eq36_e1530 * locals.var_gdrift_d_dn4);
        let eq36_e1532_d_n5: f64 = (((-p.p28) * locals.var_gdrift_d) + (eq36_e1530 * locals.var_gdrift_d_dn5));
        let eq36_e1532_d_n6: f64 = ((p.p28 * locals.var_gdrift_d) + (eq36_e1530 * locals.var_gdrift_d_dn6));
        let eq36_e1532_d_n7: f64 = (eq36_e1530 * locals.var_gdrift_d_dn7);
        let eq36_e1532_d_n8: f64 = (eq36_e1530 * locals.var_gdrift_d_dn8);
        let eq36_e1532_d_n9: f64 = (eq36_e1530 * locals.var_gdrift_d_dn9);
        let eq36_e1532_d_n10: f64 = (eq36_e1530 * locals.var_gdrift_d_dn10);
        let eq36_e1532_d_n11: f64 = (eq36_e1530 * locals.var_gdrift_d_dn11);
        let eq36_e1532_d_n12: f64 = (eq36_e1530 * locals.var_gdrift_d_dn12);
        let eq36_e1532_d_n13: f64 = (eq36_e1530 * locals.var_gdrift_d_dn13);
        let eq36_e1532_d_n14: f64 = (eq36_e1530 * locals.var_gdrift_d_dn14);
        (eq36_e1532, eq36_e1532_d_n0, eq36_e1532_d_n2, eq36_e1532_d_n3, eq36_e1532_d_n4, eq36_e1532_d_n5, eq36_e1532_d_n6, eq36_e1532_d_n7, eq36_e1532_d_n8, eq36_e1532_d_n9, eq36_e1532_d_n10, eq36_e1532_d_n11, eq36_e1532_d_n12, eq36_e1532_d_n13, eq36_e1532_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq36_value: f64 = eq36_e1534;
        let eq36_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq36_node_derivatives: [f64; 14] = [eq36_e1534_d_n0, eq36_e1534_d_n2, eq36_e1534_d_n3, eq36_e1534_d_n4, eq36_e1534_d_n5, eq36_e1534_d_n6, eq36_e1534_d_n7, eq36_e1534_d_n8, eq36_e1534_d_n9, eq36_e1534_d_n10, eq36_e1534_d_n11, eq36_e1534_d_n12, eq36_e1534_d_n13, eq36_e1534_d_n14];
        let eq36_branch_derivative_indices: [usize; 0] = [];
        let eq36_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(6),
            Some(5),
            multiplicity * (eq36_value),
            &eq36_node_derivative_indices,
            &eq36_node_derivatives,
            &eq36_branch_derivative_indices,
            &eq36_branch_derivatives,
            multiplicity,
        );
        let (eq39_e1572,) = {
    if ((locals.var_guard754 != 0.0) && (locals.var_guard755 == 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq39_value: f64 = eq39_e1572;
        stamper.stamp_potential_const_local(
            1,
            eq39_value,
        );
        let (eq40_e1577,) = {
    if (locals.var_guard754 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq40_value: f64 = eq40_e1577;
        stamper.stamp_potential_const_local(
            2,
            eq40_value,
        );
        let (eq41_e1582,) = {
    if (locals.var_guard754 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq41_value: f64 = eq41_e1582;
        stamper.stamp_potential_const_local(
            3,
            eq41_value,
        );
        let (eq42_e1590, eq42_e1590_d_n0, eq42_e1590_d_n2, eq42_e1590_d_n3, eq42_e1590_d_n4, eq42_e1590_d_n5, eq42_e1590_d_n6, eq42_e1590_d_n7, eq42_e1590_d_n8, eq42_e1590_d_n9, eq42_e1590_d_n10, eq42_e1590_d_n11, eq42_e1590_d_n12, eq42_e1590_d_n13, eq42_e1590_d_n14,) = {
    if (locals.var_guard756 != 0.0) {
        let eq42_e1586: f64 = (p.p28 * (nv2 - nv8));
        let eq42_e1588: f64 = (eq42_e1586 * locals.var_gspr);
        let eq42_e1588_d_n0: f64 = (eq42_e1586 * locals.var_gspr_dn0);
        let eq42_e1588_d_n2: f64 = ((p.p28 * locals.var_gspr) + (eq42_e1586 * locals.var_gspr_dn2));
        let eq42_e1588_d_n3: f64 = (eq42_e1586 * locals.var_gspr_dn3);
        let eq42_e1588_d_n4: f64 = (eq42_e1586 * locals.var_gspr_dn4);
        let eq42_e1588_d_n5: f64 = (eq42_e1586 * locals.var_gspr_dn5);
        let eq42_e1588_d_n6: f64 = (eq42_e1586 * locals.var_gspr_dn6);
        let eq42_e1588_d_n7: f64 = (eq42_e1586 * locals.var_gspr_dn7);
        let eq42_e1588_d_n8: f64 = (((-p.p28) * locals.var_gspr) + (eq42_e1586 * locals.var_gspr_dn8));
        let eq42_e1588_d_n9: f64 = (eq42_e1586 * locals.var_gspr_dn9);
        let eq42_e1588_d_n10: f64 = (eq42_e1586 * locals.var_gspr_dn10);
        let eq42_e1588_d_n11: f64 = (eq42_e1586 * locals.var_gspr_dn11);
        let eq42_e1588_d_n12: f64 = (eq42_e1586 * locals.var_gspr_dn12);
        let eq42_e1588_d_n13: f64 = (eq42_e1586 * locals.var_gspr_dn13);
        let eq42_e1588_d_n14: f64 = (eq42_e1586 * locals.var_gspr_dn14);
        (eq42_e1588, eq42_e1588_d_n0, eq42_e1588_d_n2, eq42_e1588_d_n3, eq42_e1588_d_n4, eq42_e1588_d_n5, eq42_e1588_d_n6, eq42_e1588_d_n7, eq42_e1588_d_n8, eq42_e1588_d_n9, eq42_e1588_d_n10, eq42_e1588_d_n11, eq42_e1588_d_n12, eq42_e1588_d_n13, eq42_e1588_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq42_value: f64 = eq42_e1590;
        let eq42_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq42_node_derivatives: [f64; 14] = [eq42_e1590_d_n0, eq42_e1590_d_n2, eq42_e1590_d_n3, eq42_e1590_d_n4, eq42_e1590_d_n5, eq42_e1590_d_n6, eq42_e1590_d_n7, eq42_e1590_d_n8, eq42_e1590_d_n9, eq42_e1590_d_n10, eq42_e1590_d_n11, eq42_e1590_d_n12, eq42_e1590_d_n13, eq42_e1590_d_n14];
        let eq42_branch_derivative_indices: [usize; 0] = [];
        let eq42_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(2),
            Some(8),
            multiplicity * (eq42_value),
            &eq42_node_derivative_indices,
            &eq42_node_derivatives,
            &eq42_branch_derivative_indices,
            &eq42_branch_derivatives,
            multiplicity,
        );
        let (eq44_e1610, eq44_e1610_d_n0, eq44_e1610_d_n2, eq44_e1610_d_n3, eq44_e1610_d_n4, eq44_e1610_d_n5, eq44_e1610_d_n6, eq44_e1610_d_n7, eq44_e1610_d_n8, eq44_e1610_d_n9, eq44_e1610_d_n10, eq44_e1610_d_n11, eq44_e1610_d_n12, eq44_e1610_d_n13, eq44_e1610_d_n14,) = {
    if ((locals.var_guard756 != 0.0) && (locals.var_guard757 != 0.0)) {
        let eq44_e1606: f64 = (p.p28 * (nv8 - nv7));
        let eq44_e1608: f64 = (eq44_e1606 * locals.var_gdrift_s);
        let eq44_e1608_d_n0: f64 = (eq44_e1606 * locals.var_gdrift_s_dn0);
        let eq44_e1608_d_n2: f64 = (eq44_e1606 * locals.var_gdrift_s_dn2);
        let eq44_e1608_d_n3: f64 = (eq44_e1606 * locals.var_gdrift_s_dn3);
        let eq44_e1608_d_n4: f64 = (eq44_e1606 * locals.var_gdrift_s_dn4);
        let eq44_e1608_d_n5: f64 = (eq44_e1606 * locals.var_gdrift_s_dn5);
        let eq44_e1608_d_n6: f64 = (eq44_e1606 * locals.var_gdrift_s_dn6);
        let eq44_e1608_d_n7: f64 = (((-p.p28) * locals.var_gdrift_s) + (eq44_e1606 * locals.var_gdrift_s_dn7));
        let eq44_e1608_d_n8: f64 = ((p.p28 * locals.var_gdrift_s) + (eq44_e1606 * locals.var_gdrift_s_dn8));
        let eq44_e1608_d_n9: f64 = (eq44_e1606 * locals.var_gdrift_s_dn9);
        let eq44_e1608_d_n10: f64 = (eq44_e1606 * locals.var_gdrift_s_dn10);
        let eq44_e1608_d_n11: f64 = (eq44_e1606 * locals.var_gdrift_s_dn11);
        let eq44_e1608_d_n12: f64 = (eq44_e1606 * locals.var_gdrift_s_dn12);
        let eq44_e1608_d_n13: f64 = (eq44_e1606 * locals.var_gdrift_s_dn13);
        let eq44_e1608_d_n14: f64 = (eq44_e1606 * locals.var_gdrift_s_dn14);
        (eq44_e1608, eq44_e1608_d_n0, eq44_e1608_d_n2, eq44_e1608_d_n3, eq44_e1608_d_n4, eq44_e1608_d_n5, eq44_e1608_d_n6, eq44_e1608_d_n7, eq44_e1608_d_n8, eq44_e1608_d_n9, eq44_e1608_d_n10, eq44_e1608_d_n11, eq44_e1608_d_n12, eq44_e1608_d_n13, eq44_e1608_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq44_value: f64 = eq44_e1610;
        let eq44_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq44_node_derivatives: [f64; 14] = [eq44_e1610_d_n0, eq44_e1610_d_n2, eq44_e1610_d_n3, eq44_e1610_d_n4, eq44_e1610_d_n5, eq44_e1610_d_n6, eq44_e1610_d_n7, eq44_e1610_d_n8, eq44_e1610_d_n9, eq44_e1610_d_n10, eq44_e1610_d_n11, eq44_e1610_d_n12, eq44_e1610_d_n13, eq44_e1610_d_n14];
        let eq44_branch_derivative_indices: [usize; 0] = [];
        let eq44_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(8),
            Some(7),
            multiplicity * (eq44_value),
            &eq44_node_derivative_indices,
            &eq44_node_derivatives,
            &eq44_branch_derivative_indices,
            &eq44_branch_derivatives,
            multiplicity,
        );
        let (eq47_e1648,) = {
    if ((locals.var_guard756 != 0.0) && (locals.var_guard757 == 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq47_value: f64 = eq47_e1648;
        stamper.stamp_potential_const_local(
            4,
            eq47_value,
        );
        let (eq48_e1653,) = {
    if (locals.var_guard756 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq48_value: f64 = eq48_e1653;
        stamper.stamp_potential_const_local(
            5,
            eq48_value,
        );
        let (eq49_e1658,) = {
    if (locals.var_guard756 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq49_value: f64 = eq49_e1658;
        stamper.stamp_potential_const_local(
            6,
            eq49_value,
        );
        let (eq55_e1708, eq55_e1708_d_n0, eq55_e1708_d_n2, eq55_e1708_d_n3, eq55_e1708_d_n4, eq55_e1708_d_n5, eq55_e1708_d_n6, eq55_e1708_d_n7, eq55_e1708_d_n8, eq55_e1708_d_n9, eq55_e1708_d_n10, eq55_e1708_d_n11, eq55_e1708_d_n12, eq55_e1708_d_n13, eq55_e1708_d_n14,) = {
    if (locals.var_guard763 != 0.0) {
        let eq55_e1699: f64 = (locals.var_deltemp1 * locals.var_gth);
        let eq55_e1699_d_n4: f64 = (locals.var_deltemp1_dn4 * locals.var_gth);
        let eq55_e1702: f64 = (locals.var_deltemp1 * locals.var_cth);
        let eq55_e1702_d_n4: f64 = (locals.var_deltemp1_dn4 * locals.var_cth);
        let eq55_e1703: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, eq55_e1702);
        let eq55_e1704: f64 = (eq55_e1699 + eq55_e1703);
        let eq55_e1704_d_n4: f64 = (eq55_e1699_d_n4 + (eq55_e1702_d_n4 * ddt_scale));
        let eq55_e1706: f64 = (eq55_e1704 - locals.var_pdiss);
        let eq55_e1706_d_n4: f64 = (eq55_e1704_d_n4 - locals.var_pdiss_dn4);
        (eq55_e1706, (-locals.var_pdiss_dn0), (-locals.var_pdiss_dn2), (-locals.var_pdiss_dn3), eq55_e1706_d_n4, (-locals.var_pdiss_dn5), (-locals.var_pdiss_dn6), (-locals.var_pdiss_dn7), (-locals.var_pdiss_dn8), (-locals.var_pdiss_dn9), (-locals.var_pdiss_dn10), (-locals.var_pdiss_dn11), (-locals.var_pdiss_dn12), (-locals.var_pdiss_dn13), (-locals.var_pdiss_dn14),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq55_value: f64 = eq55_e1708;
        let eq55_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq55_node_derivatives: [f64; 14] = [eq55_e1708_d_n0, eq55_e1708_d_n2, eq55_e1708_d_n3, eq55_e1708_d_n4, eq55_e1708_d_n5, eq55_e1708_d_n6, eq55_e1708_d_n7, eq55_e1708_d_n8, eq55_e1708_d_n9, eq55_e1708_d_n10, eq55_e1708_d_n11, eq55_e1708_d_n12, eq55_e1708_d_n13, eq55_e1708_d_n14];
        let eq55_branch_derivative_indices: [usize; 0] = [];
        let eq55_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(4),
            None,
            multiplicity * (eq55_value),
            &eq55_node_derivative_indices,
            &eq55_node_derivatives,
            &eq55_branch_derivative_indices,
            &eq55_branch_derivatives,
            multiplicity,
        );
        let (eq71_e1841, eq71_e1841_d_n0, eq71_e1841_d_n2, eq71_e1841_d_n3, eq71_e1841_d_n4, eq71_e1841_d_n5, eq71_e1841_d_n6, eq71_e1841_d_n7, eq71_e1841_d_n8, eq71_e1841_d_n9, eq71_e1841_d_n10, eq71_e1841_d_n11, eq71_e1841_d_n12, eq71_e1841_d_n13, eq71_e1841_d_n14,) = {
    if (locals.var_guard769 != 0.0) {
        let eq71_e1837: f64 = (p.p29 * locals.var_qbsj);
        let eq71_e1837_d_n0: f64 = (p.p29 * locals.var_qbsj_dn0);
        let eq71_e1837_d_n2: f64 = (p.p29 * locals.var_qbsj_dn2);
        let eq71_e1837_d_n3: f64 = (p.p29 * locals.var_qbsj_dn3);
        let eq71_e1837_d_n4: f64 = (p.p29 * locals.var_qbsj_dn4);
        let eq71_e1837_d_n5: f64 = (p.p29 * locals.var_qbsj_dn5);
        let eq71_e1837_d_n6: f64 = (p.p29 * locals.var_qbsj_dn6);
        let eq71_e1837_d_n7: f64 = (p.p29 * locals.var_qbsj_dn7);
        let eq71_e1837_d_n8: f64 = (p.p29 * locals.var_qbsj_dn8);
        let eq71_e1837_d_n9: f64 = (p.p29 * locals.var_qbsj_dn9);
        let eq71_e1837_d_n10: f64 = (p.p29 * locals.var_qbsj_dn10);
        let eq71_e1837_d_n11: f64 = (p.p29 * locals.var_qbsj_dn11);
        let eq71_e1837_d_n12: f64 = (p.p29 * locals.var_qbsj_dn12);
        let eq71_e1837_d_n13: f64 = (p.p29 * locals.var_qbsj_dn13);
        let eq71_e1837_d_n14: f64 = (p.p29 * locals.var_qbsj_dn14);
        let eq71_e1838: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, eq71_e1837);
        let eq71_e1839: f64 = (locals.var_devsign * eq71_e1838);
        let eq71_e1839_d_n0: f64 = (locals.var_devsign * (eq71_e1837_d_n0 * ddt_scale));
        let eq71_e1839_d_n2: f64 = (locals.var_devsign * (eq71_e1837_d_n2 * ddt_scale));
        let eq71_e1839_d_n3: f64 = (locals.var_devsign * (eq71_e1837_d_n3 * ddt_scale));
        let eq71_e1839_d_n4: f64 = (locals.var_devsign * (eq71_e1837_d_n4 * ddt_scale));
        let eq71_e1839_d_n5: f64 = (locals.var_devsign * (eq71_e1837_d_n5 * ddt_scale));
        let eq71_e1839_d_n6: f64 = (locals.var_devsign * (eq71_e1837_d_n6 * ddt_scale));
        let eq71_e1839_d_n7: f64 = (locals.var_devsign * (eq71_e1837_d_n7 * ddt_scale));
        let eq71_e1839_d_n8: f64 = (locals.var_devsign * (eq71_e1837_d_n8 * ddt_scale));
        let eq71_e1839_d_n9: f64 = (locals.var_devsign * (eq71_e1837_d_n9 * ddt_scale));
        let eq71_e1839_d_n10: f64 = (locals.var_devsign * (eq71_e1837_d_n10 * ddt_scale));
        let eq71_e1839_d_n11: f64 = (locals.var_devsign * (eq71_e1837_d_n11 * ddt_scale));
        let eq71_e1839_d_n12: f64 = (locals.var_devsign * (eq71_e1837_d_n12 * ddt_scale));
        let eq71_e1839_d_n13: f64 = (locals.var_devsign * (eq71_e1837_d_n13 * ddt_scale));
        let eq71_e1839_d_n14: f64 = (locals.var_devsign * (eq71_e1837_d_n14 * ddt_scale));
        (eq71_e1839, eq71_e1839_d_n0, eq71_e1839_d_n2, eq71_e1839_d_n3, eq71_e1839_d_n4, eq71_e1839_d_n5, eq71_e1839_d_n6, eq71_e1839_d_n7, eq71_e1839_d_n8, eq71_e1839_d_n9, eq71_e1839_d_n10, eq71_e1839_d_n11, eq71_e1839_d_n12, eq71_e1839_d_n13, eq71_e1839_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq71_value: f64 = eq71_e1841;
        let eq71_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq71_node_derivatives: [f64; 14] = [eq71_e1841_d_n0, eq71_e1841_d_n2, eq71_e1841_d_n3, eq71_e1841_d_n4, eq71_e1841_d_n5, eq71_e1841_d_n6, eq71_e1841_d_n7, eq71_e1841_d_n8, eq71_e1841_d_n9, eq71_e1841_d_n10, eq71_e1841_d_n11, eq71_e1841_d_n12, eq71_e1841_d_n13, eq71_e1841_d_n14];
        let eq71_branch_derivative_indices: [usize; 0] = [];
        let eq71_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(12),
            Some(7),
            multiplicity * (eq71_value),
            &eq71_node_derivative_indices,
            &eq71_node_derivatives,
            &eq71_branch_derivative_indices,
            &eq71_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_2(
        stamper: &mut GeneratedStamper<'_>,
        p: &Parameters,
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        locals: &mut StampLocals,
    ) {
        let (eq73_e1868, eq73_e1868_d_n0, eq73_e1868_d_n2, eq73_e1868_d_n3, eq73_e1868_d_n4, eq73_e1868_d_n5, eq73_e1868_d_n6, eq73_e1868_d_n7, eq73_e1868_d_n8, eq73_e1868_d_n9, eq73_e1868_d_n10, eq73_e1868_d_n11, eq73_e1868_d_n12, eq73_e1868_d_n13, eq73_e1868_d_n14,) = {
    if ((locals.var_guard769 != 0.0) && (locals.var_guard770 != 0.0)) {
        let eq73_e1864: f64 = (p.p29 * locals.var_qbdj);
        let eq73_e1864_d_n0: f64 = (p.p29 * locals.var_qbdj_dn0);
        let eq73_e1864_d_n2: f64 = (p.p29 * locals.var_qbdj_dn2);
        let eq73_e1864_d_n3: f64 = (p.p29 * locals.var_qbdj_dn3);
        let eq73_e1864_d_n4: f64 = (p.p29 * locals.var_qbdj_dn4);
        let eq73_e1864_d_n5: f64 = (p.p29 * locals.var_qbdj_dn5);
        let eq73_e1864_d_n6: f64 = (p.p29 * locals.var_qbdj_dn6);
        let eq73_e1864_d_n7: f64 = (p.p29 * locals.var_qbdj_dn7);
        let eq73_e1864_d_n8: f64 = (p.p29 * locals.var_qbdj_dn8);
        let eq73_e1864_d_n9: f64 = (p.p29 * locals.var_qbdj_dn9);
        let eq73_e1864_d_n10: f64 = (p.p29 * locals.var_qbdj_dn10);
        let eq73_e1864_d_n11: f64 = (p.p29 * locals.var_qbdj_dn11);
        let eq73_e1864_d_n12: f64 = (p.p29 * locals.var_qbdj_dn12);
        let eq73_e1864_d_n13: f64 = (p.p29 * locals.var_qbdj_dn13);
        let eq73_e1864_d_n14: f64 = (p.p29 * locals.var_qbdj_dn14);
        let eq73_e1865: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 11, eq73_e1864);
        let eq73_e1866: f64 = (locals.var_devsign * eq73_e1865);
        let eq73_e1866_d_n0: f64 = (locals.var_devsign * (eq73_e1864_d_n0 * ddt_scale));
        let eq73_e1866_d_n2: f64 = (locals.var_devsign * (eq73_e1864_d_n2 * ddt_scale));
        let eq73_e1866_d_n3: f64 = (locals.var_devsign * (eq73_e1864_d_n3 * ddt_scale));
        let eq73_e1866_d_n4: f64 = (locals.var_devsign * (eq73_e1864_d_n4 * ddt_scale));
        let eq73_e1866_d_n5: f64 = (locals.var_devsign * (eq73_e1864_d_n5 * ddt_scale));
        let eq73_e1866_d_n6: f64 = (locals.var_devsign * (eq73_e1864_d_n6 * ddt_scale));
        let eq73_e1866_d_n7: f64 = (locals.var_devsign * (eq73_e1864_d_n7 * ddt_scale));
        let eq73_e1866_d_n8: f64 = (locals.var_devsign * (eq73_e1864_d_n8 * ddt_scale));
        let eq73_e1866_d_n9: f64 = (locals.var_devsign * (eq73_e1864_d_n9 * ddt_scale));
        let eq73_e1866_d_n10: f64 = (locals.var_devsign * (eq73_e1864_d_n10 * ddt_scale));
        let eq73_e1866_d_n11: f64 = (locals.var_devsign * (eq73_e1864_d_n11 * ddt_scale));
        let eq73_e1866_d_n12: f64 = (locals.var_devsign * (eq73_e1864_d_n12 * ddt_scale));
        let eq73_e1866_d_n13: f64 = (locals.var_devsign * (eq73_e1864_d_n13 * ddt_scale));
        let eq73_e1866_d_n14: f64 = (locals.var_devsign * (eq73_e1864_d_n14 * ddt_scale));
        (eq73_e1866, eq73_e1866_d_n0, eq73_e1866_d_n2, eq73_e1866_d_n3, eq73_e1866_d_n4, eq73_e1866_d_n5, eq73_e1866_d_n6, eq73_e1866_d_n7, eq73_e1866_d_n8, eq73_e1866_d_n9, eq73_e1866_d_n10, eq73_e1866_d_n11, eq73_e1866_d_n12, eq73_e1866_d_n13, eq73_e1866_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq73_value: f64 = eq73_e1868;
        let eq73_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq73_node_derivatives: [f64; 14] = [eq73_e1868_d_n0, eq73_e1868_d_n2, eq73_e1868_d_n3, eq73_e1868_d_n4, eq73_e1868_d_n5, eq73_e1868_d_n6, eq73_e1868_d_n7, eq73_e1868_d_n8, eq73_e1868_d_n9, eq73_e1868_d_n10, eq73_e1868_d_n11, eq73_e1868_d_n12, eq73_e1868_d_n13, eq73_e1868_d_n14];
        let eq73_branch_derivative_indices: [usize; 0] = [];
        let eq73_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(13),
            Some(5),
            multiplicity * (eq73_value),
            &eq73_node_derivative_indices,
            &eq73_node_derivatives,
            &eq73_branch_derivative_indices,
            &eq73_branch_derivatives,
            multiplicity,
        );
        let (eq76_e1908, eq76_e1908_d_n0, eq76_e1908_d_n2, eq76_e1908_d_n3, eq76_e1908_d_n4, eq76_e1908_d_n5, eq76_e1908_d_n6, eq76_e1908_d_n7, eq76_e1908_d_n8, eq76_e1908_d_n9, eq76_e1908_d_n10, eq76_e1908_d_n11, eq76_e1908_d_n12, eq76_e1908_d_n13, eq76_e1908_d_n14,) = {
    if (locals.var_guard769 == 0.0) {
        let eq76_e1904: f64 = (p.p29 * locals.var_qbsj);
        let eq76_e1904_d_n0: f64 = (p.p29 * locals.var_qbsj_dn0);
        let eq76_e1904_d_n2: f64 = (p.p29 * locals.var_qbsj_dn2);
        let eq76_e1904_d_n3: f64 = (p.p29 * locals.var_qbsj_dn3);
        let eq76_e1904_d_n4: f64 = (p.p29 * locals.var_qbsj_dn4);
        let eq76_e1904_d_n5: f64 = (p.p29 * locals.var_qbsj_dn5);
        let eq76_e1904_d_n6: f64 = (p.p29 * locals.var_qbsj_dn6);
        let eq76_e1904_d_n7: f64 = (p.p29 * locals.var_qbsj_dn7);
        let eq76_e1904_d_n8: f64 = (p.p29 * locals.var_qbsj_dn8);
        let eq76_e1904_d_n9: f64 = (p.p29 * locals.var_qbsj_dn9);
        let eq76_e1904_d_n10: f64 = (p.p29 * locals.var_qbsj_dn10);
        let eq76_e1904_d_n11: f64 = (p.p29 * locals.var_qbsj_dn11);
        let eq76_e1904_d_n12: f64 = (p.p29 * locals.var_qbsj_dn12);
        let eq76_e1904_d_n13: f64 = (p.p29 * locals.var_qbsj_dn13);
        let eq76_e1904_d_n14: f64 = (p.p29 * locals.var_qbsj_dn14);
        let eq76_e1905: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 12, eq76_e1904);
        let eq76_e1906: f64 = (locals.var_devsign * eq76_e1905);
        let eq76_e1906_d_n0: f64 = (locals.var_devsign * (eq76_e1904_d_n0 * ddt_scale));
        let eq76_e1906_d_n2: f64 = (locals.var_devsign * (eq76_e1904_d_n2 * ddt_scale));
        let eq76_e1906_d_n3: f64 = (locals.var_devsign * (eq76_e1904_d_n3 * ddt_scale));
        let eq76_e1906_d_n4: f64 = (locals.var_devsign * (eq76_e1904_d_n4 * ddt_scale));
        let eq76_e1906_d_n5: f64 = (locals.var_devsign * (eq76_e1904_d_n5 * ddt_scale));
        let eq76_e1906_d_n6: f64 = (locals.var_devsign * (eq76_e1904_d_n6 * ddt_scale));
        let eq76_e1906_d_n7: f64 = (locals.var_devsign * (eq76_e1904_d_n7 * ddt_scale));
        let eq76_e1906_d_n8: f64 = (locals.var_devsign * (eq76_e1904_d_n8 * ddt_scale));
        let eq76_e1906_d_n9: f64 = (locals.var_devsign * (eq76_e1904_d_n9 * ddt_scale));
        let eq76_e1906_d_n10: f64 = (locals.var_devsign * (eq76_e1904_d_n10 * ddt_scale));
        let eq76_e1906_d_n11: f64 = (locals.var_devsign * (eq76_e1904_d_n11 * ddt_scale));
        let eq76_e1906_d_n12: f64 = (locals.var_devsign * (eq76_e1904_d_n12 * ddt_scale));
        let eq76_e1906_d_n13: f64 = (locals.var_devsign * (eq76_e1904_d_n13 * ddt_scale));
        let eq76_e1906_d_n14: f64 = (locals.var_devsign * (eq76_e1904_d_n14 * ddt_scale));
        (eq76_e1906, eq76_e1906_d_n0, eq76_e1906_d_n2, eq76_e1906_d_n3, eq76_e1906_d_n4, eq76_e1906_d_n5, eq76_e1906_d_n6, eq76_e1906_d_n7, eq76_e1906_d_n8, eq76_e1906_d_n9, eq76_e1906_d_n10, eq76_e1906_d_n11, eq76_e1906_d_n12, eq76_e1906_d_n13, eq76_e1906_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq76_value: f64 = eq76_e1908;
        let eq76_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq76_node_derivatives: [f64; 14] = [eq76_e1908_d_n0, eq76_e1908_d_n2, eq76_e1908_d_n3, eq76_e1908_d_n4, eq76_e1908_d_n5, eq76_e1908_d_n6, eq76_e1908_d_n7, eq76_e1908_d_n8, eq76_e1908_d_n9, eq76_e1908_d_n10, eq76_e1908_d_n11, eq76_e1908_d_n12, eq76_e1908_d_n13, eq76_e1908_d_n14];
        let eq76_branch_derivative_indices: [usize; 0] = [];
        let eq76_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(11),
            Some(7),
            multiplicity * (eq76_value),
            &eq76_node_derivative_indices,
            &eq76_node_derivatives,
            &eq76_branch_derivative_indices,
            &eq76_branch_derivatives,
            multiplicity,
        );
        let (eq77_e1918, eq77_e1918_d_n0, eq77_e1918_d_n2, eq77_e1918_d_n3, eq77_e1918_d_n4, eq77_e1918_d_n5, eq77_e1918_d_n6, eq77_e1918_d_n7, eq77_e1918_d_n8, eq77_e1918_d_n9, eq77_e1918_d_n10, eq77_e1918_d_n11, eq77_e1918_d_n12, eq77_e1918_d_n13, eq77_e1918_d_n14,) = {
    if (locals.var_guard769 == 0.0) {
        let eq77_e1914: f64 = (p.p29 * locals.var_qbdj);
        let eq77_e1914_d_n0: f64 = (p.p29 * locals.var_qbdj_dn0);
        let eq77_e1914_d_n2: f64 = (p.p29 * locals.var_qbdj_dn2);
        let eq77_e1914_d_n3: f64 = (p.p29 * locals.var_qbdj_dn3);
        let eq77_e1914_d_n4: f64 = (p.p29 * locals.var_qbdj_dn4);
        let eq77_e1914_d_n5: f64 = (p.p29 * locals.var_qbdj_dn5);
        let eq77_e1914_d_n6: f64 = (p.p29 * locals.var_qbdj_dn6);
        let eq77_e1914_d_n7: f64 = (p.p29 * locals.var_qbdj_dn7);
        let eq77_e1914_d_n8: f64 = (p.p29 * locals.var_qbdj_dn8);
        let eq77_e1914_d_n9: f64 = (p.p29 * locals.var_qbdj_dn9);
        let eq77_e1914_d_n10: f64 = (p.p29 * locals.var_qbdj_dn10);
        let eq77_e1914_d_n11: f64 = (p.p29 * locals.var_qbdj_dn11);
        let eq77_e1914_d_n12: f64 = (p.p29 * locals.var_qbdj_dn12);
        let eq77_e1914_d_n13: f64 = (p.p29 * locals.var_qbdj_dn13);
        let eq77_e1914_d_n14: f64 = (p.p29 * locals.var_qbdj_dn14);
        let eq77_e1915: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 13, eq77_e1914);
        let eq77_e1916: f64 = (locals.var_devsign * eq77_e1915);
        let eq77_e1916_d_n0: f64 = (locals.var_devsign * (eq77_e1914_d_n0 * ddt_scale));
        let eq77_e1916_d_n2: f64 = (locals.var_devsign * (eq77_e1914_d_n2 * ddt_scale));
        let eq77_e1916_d_n3: f64 = (locals.var_devsign * (eq77_e1914_d_n3 * ddt_scale));
        let eq77_e1916_d_n4: f64 = (locals.var_devsign * (eq77_e1914_d_n4 * ddt_scale));
        let eq77_e1916_d_n5: f64 = (locals.var_devsign * (eq77_e1914_d_n5 * ddt_scale));
        let eq77_e1916_d_n6: f64 = (locals.var_devsign * (eq77_e1914_d_n6 * ddt_scale));
        let eq77_e1916_d_n7: f64 = (locals.var_devsign * (eq77_e1914_d_n7 * ddt_scale));
        let eq77_e1916_d_n8: f64 = (locals.var_devsign * (eq77_e1914_d_n8 * ddt_scale));
        let eq77_e1916_d_n9: f64 = (locals.var_devsign * (eq77_e1914_d_n9 * ddt_scale));
        let eq77_e1916_d_n10: f64 = (locals.var_devsign * (eq77_e1914_d_n10 * ddt_scale));
        let eq77_e1916_d_n11: f64 = (locals.var_devsign * (eq77_e1914_d_n11 * ddt_scale));
        let eq77_e1916_d_n12: f64 = (locals.var_devsign * (eq77_e1914_d_n12 * ddt_scale));
        let eq77_e1916_d_n13: f64 = (locals.var_devsign * (eq77_e1914_d_n13 * ddt_scale));
        let eq77_e1916_d_n14: f64 = (locals.var_devsign * (eq77_e1914_d_n14 * ddt_scale));
        (eq77_e1916, eq77_e1916_d_n0, eq77_e1916_d_n2, eq77_e1916_d_n3, eq77_e1916_d_n4, eq77_e1916_d_n5, eq77_e1916_d_n6, eq77_e1916_d_n7, eq77_e1916_d_n8, eq77_e1916_d_n9, eq77_e1916_d_n10, eq77_e1916_d_n11, eq77_e1916_d_n12, eq77_e1916_d_n13, eq77_e1916_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq77_value: f64 = eq77_e1918;
        let eq77_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq77_node_derivatives: [f64; 14] = [eq77_e1918_d_n0, eq77_e1918_d_n2, eq77_e1918_d_n3, eq77_e1918_d_n4, eq77_e1918_d_n5, eq77_e1918_d_n6, eq77_e1918_d_n7, eq77_e1918_d_n8, eq77_e1918_d_n9, eq77_e1918_d_n10, eq77_e1918_d_n11, eq77_e1918_d_n12, eq77_e1918_d_n13, eq77_e1918_d_n14];
        let eq77_branch_derivative_indices: [usize; 0] = [];
        let eq77_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(11),
            Some(5),
            multiplicity * (eq77_value),
            &eq77_node_derivative_indices,
            &eq77_node_derivatives,
            &eq77_branch_derivative_indices,
            &eq77_branch_derivatives,
            multiplicity,
        );
        let (eq83_e1984, eq83_e1984_d_n0, eq83_e1984_d_n2, eq83_e1984_d_n3, eq83_e1984_d_n4, eq83_e1984_d_n5, eq83_e1984_d_n6, eq83_e1984_d_n7, eq83_e1984_d_n8, eq83_e1984_d_n9, eq83_e1984_d_n10, eq83_e1984_d_n11, eq83_e1984_d_n12, eq83_e1984_d_n13, eq83_e1984_d_n14,) = {
    if (locals.var_guard772 != 0.0) {
        let eq83_e1980: f64 = (p.p29 * locals.var_qbdj);
        let eq83_e1980_d_n0: f64 = (p.p29 * locals.var_qbdj_dn0);
        let eq83_e1980_d_n2: f64 = (p.p29 * locals.var_qbdj_dn2);
        let eq83_e1980_d_n3: f64 = (p.p29 * locals.var_qbdj_dn3);
        let eq83_e1980_d_n4: f64 = (p.p29 * locals.var_qbdj_dn4);
        let eq83_e1980_d_n5: f64 = (p.p29 * locals.var_qbdj_dn5);
        let eq83_e1980_d_n6: f64 = (p.p29 * locals.var_qbdj_dn6);
        let eq83_e1980_d_n7: f64 = (p.p29 * locals.var_qbdj_dn7);
        let eq83_e1980_d_n8: f64 = (p.p29 * locals.var_qbdj_dn8);
        let eq83_e1980_d_n9: f64 = (p.p29 * locals.var_qbdj_dn9);
        let eq83_e1980_d_n10: f64 = (p.p29 * locals.var_qbdj_dn10);
        let eq83_e1980_d_n11: f64 = (p.p29 * locals.var_qbdj_dn11);
        let eq83_e1980_d_n12: f64 = (p.p29 * locals.var_qbdj_dn12);
        let eq83_e1980_d_n13: f64 = (p.p29 * locals.var_qbdj_dn13);
        let eq83_e1980_d_n14: f64 = (p.p29 * locals.var_qbdj_dn14);
        let eq83_e1981: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 14, eq83_e1980);
        let eq83_e1982: f64 = (locals.var_devsign * eq83_e1981);
        let eq83_e1982_d_n0: f64 = (locals.var_devsign * (eq83_e1980_d_n0 * ddt_scale));
        let eq83_e1982_d_n2: f64 = (locals.var_devsign * (eq83_e1980_d_n2 * ddt_scale));
        let eq83_e1982_d_n3: f64 = (locals.var_devsign * (eq83_e1980_d_n3 * ddt_scale));
        let eq83_e1982_d_n4: f64 = (locals.var_devsign * (eq83_e1980_d_n4 * ddt_scale));
        let eq83_e1982_d_n5: f64 = (locals.var_devsign * (eq83_e1980_d_n5 * ddt_scale));
        let eq83_e1982_d_n6: f64 = (locals.var_devsign * (eq83_e1980_d_n6 * ddt_scale));
        let eq83_e1982_d_n7: f64 = (locals.var_devsign * (eq83_e1980_d_n7 * ddt_scale));
        let eq83_e1982_d_n8: f64 = (locals.var_devsign * (eq83_e1980_d_n8 * ddt_scale));
        let eq83_e1982_d_n9: f64 = (locals.var_devsign * (eq83_e1980_d_n9 * ddt_scale));
        let eq83_e1982_d_n10: f64 = (locals.var_devsign * (eq83_e1980_d_n10 * ddt_scale));
        let eq83_e1982_d_n11: f64 = (locals.var_devsign * (eq83_e1980_d_n11 * ddt_scale));
        let eq83_e1982_d_n12: f64 = (locals.var_devsign * (eq83_e1980_d_n12 * ddt_scale));
        let eq83_e1982_d_n13: f64 = (locals.var_devsign * (eq83_e1980_d_n13 * ddt_scale));
        let eq83_e1982_d_n14: f64 = (locals.var_devsign * (eq83_e1980_d_n14 * ddt_scale));
        (eq83_e1982, eq83_e1982_d_n0, eq83_e1982_d_n2, eq83_e1982_d_n3, eq83_e1982_d_n4, eq83_e1982_d_n5, eq83_e1982_d_n6, eq83_e1982_d_n7, eq83_e1982_d_n8, eq83_e1982_d_n9, eq83_e1982_d_n10, eq83_e1982_d_n11, eq83_e1982_d_n12, eq83_e1982_d_n13, eq83_e1982_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq83_value: f64 = eq83_e1984;
        let eq83_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq83_node_derivatives: [f64; 14] = [eq83_e1984_d_n0, eq83_e1984_d_n2, eq83_e1984_d_n3, eq83_e1984_d_n4, eq83_e1984_d_n5, eq83_e1984_d_n6, eq83_e1984_d_n7, eq83_e1984_d_n8, eq83_e1984_d_n9, eq83_e1984_d_n10, eq83_e1984_d_n11, eq83_e1984_d_n12, eq83_e1984_d_n13, eq83_e1984_d_n14];
        let eq83_branch_derivative_indices: [usize; 0] = [];
        let eq83_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(13),
            Some(5),
            multiplicity * (eq83_value),
            &eq83_node_derivative_indices,
            &eq83_node_derivatives,
            &eq83_branch_derivative_indices,
            &eq83_branch_derivatives,
            multiplicity,
        );
        let (eq84_e1993, eq84_e1993_d_n0, eq84_e1993_d_n2, eq84_e1993_d_n3, eq84_e1993_d_n4, eq84_e1993_d_n5, eq84_e1993_d_n6, eq84_e1993_d_n7, eq84_e1993_d_n8, eq84_e1993_d_n9, eq84_e1993_d_n10, eq84_e1993_d_n11, eq84_e1993_d_n12, eq84_e1993_d_n13, eq84_e1993_d_n14,) = {
    if (locals.var_guard772 != 0.0) {
        let eq84_e1989: f64 = (p.p29 * locals.var_qbdj_ext);
        let eq84_e1989_d_n0: f64 = (p.p29 * locals.var_qbdj_ext_dn0);
        let eq84_e1989_d_n2: f64 = (p.p29 * locals.var_qbdj_ext_dn2);
        let eq84_e1989_d_n3: f64 = (p.p29 * locals.var_qbdj_ext_dn3);
        let eq84_e1989_d_n4: f64 = (p.p29 * locals.var_qbdj_ext_dn4);
        let eq84_e1989_d_n5: f64 = (p.p29 * locals.var_qbdj_ext_dn5);
        let eq84_e1989_d_n6: f64 = (p.p29 * locals.var_qbdj_ext_dn6);
        let eq84_e1989_d_n7: f64 = (p.p29 * locals.var_qbdj_ext_dn7);
        let eq84_e1989_d_n8: f64 = (p.p29 * locals.var_qbdj_ext_dn8);
        let eq84_e1989_d_n9: f64 = (p.p29 * locals.var_qbdj_ext_dn9);
        let eq84_e1989_d_n10: f64 = (p.p29 * locals.var_qbdj_ext_dn10);
        let eq84_e1989_d_n11: f64 = (p.p29 * locals.var_qbdj_ext_dn11);
        let eq84_e1989_d_n12: f64 = (p.p29 * locals.var_qbdj_ext_dn12);
        let eq84_e1989_d_n13: f64 = (p.p29 * locals.var_qbdj_ext_dn13);
        let eq84_e1989_d_n14: f64 = (p.p29 * locals.var_qbdj_ext_dn14);
        let eq84_e1990: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 15, eq84_e1989);
        let eq84_e1991: f64 = (locals.var_devsign * eq84_e1990);
        let eq84_e1991_d_n0: f64 = (locals.var_devsign * (eq84_e1989_d_n0 * ddt_scale));
        let eq84_e1991_d_n2: f64 = (locals.var_devsign * (eq84_e1989_d_n2 * ddt_scale));
        let eq84_e1991_d_n3: f64 = (locals.var_devsign * (eq84_e1989_d_n3 * ddt_scale));
        let eq84_e1991_d_n4: f64 = (locals.var_devsign * (eq84_e1989_d_n4 * ddt_scale));
        let eq84_e1991_d_n5: f64 = (locals.var_devsign * (eq84_e1989_d_n5 * ddt_scale));
        let eq84_e1991_d_n6: f64 = (locals.var_devsign * (eq84_e1989_d_n6 * ddt_scale));
        let eq84_e1991_d_n7: f64 = (locals.var_devsign * (eq84_e1989_d_n7 * ddt_scale));
        let eq84_e1991_d_n8: f64 = (locals.var_devsign * (eq84_e1989_d_n8 * ddt_scale));
        let eq84_e1991_d_n9: f64 = (locals.var_devsign * (eq84_e1989_d_n9 * ddt_scale));
        let eq84_e1991_d_n10: f64 = (locals.var_devsign * (eq84_e1989_d_n10 * ddt_scale));
        let eq84_e1991_d_n11: f64 = (locals.var_devsign * (eq84_e1989_d_n11 * ddt_scale));
        let eq84_e1991_d_n12: f64 = (locals.var_devsign * (eq84_e1989_d_n12 * ddt_scale));
        let eq84_e1991_d_n13: f64 = (locals.var_devsign * (eq84_e1989_d_n13 * ddt_scale));
        let eq84_e1991_d_n14: f64 = (locals.var_devsign * (eq84_e1989_d_n14 * ddt_scale));
        (eq84_e1991, eq84_e1991_d_n0, eq84_e1991_d_n2, eq84_e1991_d_n3, eq84_e1991_d_n4, eq84_e1991_d_n5, eq84_e1991_d_n6, eq84_e1991_d_n7, eq84_e1991_d_n8, eq84_e1991_d_n9, eq84_e1991_d_n10, eq84_e1991_d_n11, eq84_e1991_d_n12, eq84_e1991_d_n13, eq84_e1991_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq84_value: f64 = eq84_e1993;
        let eq84_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq84_node_derivatives: [f64; 14] = [eq84_e1993_d_n0, eq84_e1993_d_n2, eq84_e1993_d_n3, eq84_e1993_d_n4, eq84_e1993_d_n5, eq84_e1993_d_n6, eq84_e1993_d_n7, eq84_e1993_d_n8, eq84_e1993_d_n9, eq84_e1993_d_n10, eq84_e1993_d_n11, eq84_e1993_d_n12, eq84_e1993_d_n13, eq84_e1993_d_n14];
        let eq84_branch_derivative_indices: [usize; 0] = [];
        let eq84_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(13),
            Some(14),
            multiplicity * (eq84_value),
            &eq84_node_derivative_indices,
            &eq84_node_derivatives,
            &eq84_branch_derivative_indices,
            &eq84_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        locals: &mut StampLocals,
    ) {
        let nv15 = ctx.node_voltage(nodes[15]);
        let (eq8_e1290, eq8_e1290_d_n0, eq8_e1290_d_n2, eq8_e1290_d_n3, eq8_e1290_d_n4, eq8_e1290_d_n5, eq8_e1290_d_n6, eq8_e1290_d_n7, eq8_e1290_d_n8, eq8_e1290_d_n9, eq8_e1290_d_n10, eq8_e1290_d_n11, eq8_e1290_d_n12, eq8_e1290_d_n13, eq8_e1290_d_n14, eq8_e1290_d_n15, eq8_e1290_q,) = {
    if ((locals.var_guard698 != 0.0) && (locals.var_guard697 == 0.0)) {
        let eq8_e1279: f64 = (locals.var_mig * locals.var_cox);
        let eq8_e1279_d_n0: f64 = (locals.var_mig_dn0 * locals.var_cox);
        let eq8_e1279_d_n2: f64 = (locals.var_mig_dn2 * locals.var_cox);
        let eq8_e1279_d_n3: f64 = (locals.var_mig_dn3 * locals.var_cox);
        let eq8_e1279_d_n4: f64 = (locals.var_mig_dn4 * locals.var_cox);
        let eq8_e1279_d_n5: f64 = (locals.var_mig_dn5 * locals.var_cox);
        let eq8_e1279_d_n6: f64 = (locals.var_mig_dn6 * locals.var_cox);
        let eq8_e1279_d_n7: f64 = (locals.var_mig_dn7 * locals.var_cox);
        let eq8_e1279_d_n8: f64 = (locals.var_mig_dn8 * locals.var_cox);
        let eq8_e1279_d_n9: f64 = (locals.var_mig_dn9 * locals.var_cox);
        let eq8_e1279_d_n10: f64 = (locals.var_mig_dn10 * locals.var_cox);
        let eq8_e1279_d_n11: f64 = (locals.var_mig_dn11 * locals.var_cox);
        let eq8_e1279_d_n12: f64 = (locals.var_mig_dn12 * locals.var_cox);
        let eq8_e1279_d_n13: f64 = (locals.var_mig_dn13 * locals.var_cox);
        let eq8_e1279_d_n14: f64 = (locals.var_mig_dn14 * locals.var_cox);
        let eq8_e1281: f64 = (eq8_e1279 * locals.var_weff);
        let eq8_e1281_d_n0: f64 = (eq8_e1279_d_n0 * locals.var_weff);
        let eq8_e1281_d_n2: f64 = (eq8_e1279_d_n2 * locals.var_weff);
        let eq8_e1281_d_n3: f64 = (eq8_e1279_d_n3 * locals.var_weff);
        let eq8_e1281_d_n4: f64 = (eq8_e1279_d_n4 * locals.var_weff);
        let eq8_e1281_d_n5: f64 = (eq8_e1279_d_n5 * locals.var_weff);
        let eq8_e1281_d_n6: f64 = (eq8_e1279_d_n6 * locals.var_weff);
        let eq8_e1281_d_n7: f64 = (eq8_e1279_d_n7 * locals.var_weff);
        let eq8_e1281_d_n8: f64 = (eq8_e1279_d_n8 * locals.var_weff);
        let eq8_e1281_d_n9: f64 = (eq8_e1279_d_n9 * locals.var_weff);
        let eq8_e1281_d_n10: f64 = (eq8_e1279_d_n10 * locals.var_weff);
        let eq8_e1281_d_n11: f64 = (eq8_e1279_d_n11 * locals.var_weff);
        let eq8_e1281_d_n12: f64 = (eq8_e1279_d_n12 * locals.var_weff);
        let eq8_e1281_d_n13: f64 = (eq8_e1279_d_n13 * locals.var_weff);
        let eq8_e1281_d_n14: f64 = (eq8_e1279_d_n14 * locals.var_weff);
        let eq8_e1283: f64 = (eq8_e1281 * p.p2);
        let eq8_e1283_d_n0: f64 = (eq8_e1281_d_n0 * p.p2);
        let eq8_e1283_d_n2: f64 = (eq8_e1281_d_n2 * p.p2);
        let eq8_e1283_d_n3: f64 = (eq8_e1281_d_n3 * p.p2);
        let eq8_e1283_d_n4: f64 = (eq8_e1281_d_n4 * p.p2);
        let eq8_e1283_d_n5: f64 = (eq8_e1281_d_n5 * p.p2);
        let eq8_e1283_d_n6: f64 = (eq8_e1281_d_n6 * p.p2);
        let eq8_e1283_d_n7: f64 = (eq8_e1281_d_n7 * p.p2);
        let eq8_e1283_d_n8: f64 = (eq8_e1281_d_n8 * p.p2);
        let eq8_e1283_d_n9: f64 = (eq8_e1281_d_n9 * p.p2);
        let eq8_e1283_d_n10: f64 = (eq8_e1281_d_n10 * p.p2);
        let eq8_e1283_d_n11: f64 = (eq8_e1281_d_n11 * p.p2);
        let eq8_e1283_d_n12: f64 = (eq8_e1281_d_n12 * p.p2);
        let eq8_e1283_d_n13: f64 = (eq8_e1281_d_n13 * p.p2);
        let eq8_e1283_d_n14: f64 = (eq8_e1281_d_n14 * p.p2);
        let eq8_e1285: f64 = (eq8_e1283 * locals.var_leff);
        let eq8_e1285_d_n0: f64 = (eq8_e1283_d_n0 * locals.var_leff);
        let eq8_e1285_d_n2: f64 = (eq8_e1283_d_n2 * locals.var_leff);
        let eq8_e1285_d_n3: f64 = (eq8_e1283_d_n3 * locals.var_leff);
        let eq8_e1285_d_n4: f64 = (eq8_e1283_d_n4 * locals.var_leff);
        let eq8_e1285_d_n5: f64 = (eq8_e1283_d_n5 * locals.var_leff);
        let eq8_e1285_d_n6: f64 = (eq8_e1283_d_n6 * locals.var_leff);
        let eq8_e1285_d_n7: f64 = (eq8_e1283_d_n7 * locals.var_leff);
        let eq8_e1285_d_n8: f64 = (eq8_e1283_d_n8 * locals.var_leff);
        let eq8_e1285_d_n9: f64 = (eq8_e1283_d_n9 * locals.var_leff);
        let eq8_e1285_d_n10: f64 = (eq8_e1283_d_n10 * locals.var_leff);
        let eq8_e1285_d_n11: f64 = (eq8_e1283_d_n11 * locals.var_leff);
        let eq8_e1285_d_n12: f64 = (eq8_e1283_d_n12 * locals.var_leff);
        let eq8_e1285_d_n13: f64 = (eq8_e1283_d_n13 * locals.var_leff);
        let eq8_e1285_d_n14: f64 = (eq8_e1283_d_n14 * locals.var_leff);
        let eq8_e1287: f64 = (eq8_e1285 * (nv15 - 0.0));
        let eq8_e1287_d_n0: f64 = (eq8_e1285_d_n0 * (nv15 - 0.0));
        let eq8_e1287_d_n2: f64 = (eq8_e1285_d_n2 * (nv15 - 0.0));
        let eq8_e1287_d_n3: f64 = (eq8_e1285_d_n3 * (nv15 - 0.0));
        let eq8_e1287_d_n4: f64 = (eq8_e1285_d_n4 * (nv15 - 0.0));
        let eq8_e1287_d_n5: f64 = (eq8_e1285_d_n5 * (nv15 - 0.0));
        let eq8_e1287_d_n6: f64 = (eq8_e1285_d_n6 * (nv15 - 0.0));
        let eq8_e1287_d_n7: f64 = (eq8_e1285_d_n7 * (nv15 - 0.0));
        let eq8_e1287_d_n8: f64 = (eq8_e1285_d_n8 * (nv15 - 0.0));
        let eq8_e1287_d_n9: f64 = (eq8_e1285_d_n9 * (nv15 - 0.0));
        let eq8_e1287_d_n10: f64 = (eq8_e1285_d_n10 * (nv15 - 0.0));
        let eq8_e1287_d_n11: f64 = (eq8_e1285_d_n11 * (nv15 - 0.0));
        let eq8_e1287_d_n12: f64 = (eq8_e1285_d_n12 * (nv15 - 0.0));
        let eq8_e1287_d_n13: f64 = (eq8_e1285_d_n13 * (nv15 - 0.0));
        let eq8_e1287_d_n14: f64 = (eq8_e1285_d_n14 * (nv15 - 0.0));
        let eq8_e1288_q: f64 = eq8_e1287;
        (eq8_e1287, eq8_e1287_d_n0, eq8_e1287_d_n2, eq8_e1287_d_n3, eq8_e1287_d_n4, eq8_e1287_d_n5, eq8_e1287_d_n6, eq8_e1287_d_n7, eq8_e1287_d_n8, eq8_e1287_d_n9, eq8_e1287_d_n10, eq8_e1287_d_n11, eq8_e1287_d_n12, eq8_e1287_d_n13, eq8_e1287_d_n14, eq8_e1285, eq8_e1288_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq8_reactive_node_derivatives: [f64; 17] = [eq8_e1290_d_n0, 0.0, eq8_e1290_d_n2, eq8_e1290_d_n3, eq8_e1290_d_n4, eq8_e1290_d_n5, eq8_e1290_d_n6, eq8_e1290_d_n7, eq8_e1290_d_n8, eq8_e1290_d_n9, eq8_e1290_d_n10, eq8_e1290_d_n11, eq8_e1290_d_n12, eq8_e1290_d_n13, eq8_e1290_d_n14, eq8_e1290_d_n15, 0.0];
        let eq8_reactive_branch_derivatives: [f64; 14] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[15]),
            None,
            nodes,
            &eq8_reactive_node_derivatives,
            branches,
            &eq8_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq11_e1344, eq11_e1344_d_n0, eq11_e1344_d_n2, eq11_e1344_d_n3, eq11_e1344_d_n4, eq11_e1344_d_n5, eq11_e1344_d_n6, eq11_e1344_d_n7, eq11_e1344_d_n8, eq11_e1344_d_n9, eq11_e1344_d_n10, eq11_e1344_d_n11, eq11_e1344_d_n12, eq11_e1344_d_n13, eq11_e1344_d_n14, eq11_e1344_d_n15, eq11_e1344_q,) = {
    if ((locals.var_guard698 != 0.0) && (locals.var_guard697 == 0.0)) {
        let eq11_e1327: f64 = (1.0 + locals.var_sigvds);
        let eq11_e1329: f64 = (eq11_e1327 * locals.var_mig);
        let eq11_e1329_d_n0: f64 = (eq11_e1327 * locals.var_mig_dn0);
        let eq11_e1329_d_n2: f64 = (eq11_e1327 * locals.var_mig_dn2);
        let eq11_e1329_d_n3: f64 = (eq11_e1327 * locals.var_mig_dn3);
        let eq11_e1329_d_n4: f64 = (eq11_e1327 * locals.var_mig_dn4);
        let eq11_e1329_d_n5: f64 = (eq11_e1327 * locals.var_mig_dn5);
        let eq11_e1329_d_n6: f64 = (eq11_e1327 * locals.var_mig_dn6);
        let eq11_e1329_d_n7: f64 = (eq11_e1327 * locals.var_mig_dn7);
        let eq11_e1329_d_n8: f64 = (eq11_e1327 * locals.var_mig_dn8);
        let eq11_e1329_d_n9: f64 = (eq11_e1327 * locals.var_mig_dn9);
        let eq11_e1329_d_n10: f64 = (eq11_e1327 * locals.var_mig_dn10);
        let eq11_e1329_d_n11: f64 = (eq11_e1327 * locals.var_mig_dn11);
        let eq11_e1329_d_n12: f64 = (eq11_e1327 * locals.var_mig_dn12);
        let eq11_e1329_d_n13: f64 = (eq11_e1327 * locals.var_mig_dn13);
        let eq11_e1329_d_n14: f64 = (eq11_e1327 * locals.var_mig_dn14);
        let eq11_e1331: f64 = (eq11_e1329 * locals.var_cox);
        let eq11_e1331_d_n0: f64 = (eq11_e1329_d_n0 * locals.var_cox);
        let eq11_e1331_d_n2: f64 = (eq11_e1329_d_n2 * locals.var_cox);
        let eq11_e1331_d_n3: f64 = (eq11_e1329_d_n3 * locals.var_cox);
        let eq11_e1331_d_n4: f64 = (eq11_e1329_d_n4 * locals.var_cox);
        let eq11_e1331_d_n5: f64 = (eq11_e1329_d_n5 * locals.var_cox);
        let eq11_e1331_d_n6: f64 = (eq11_e1329_d_n6 * locals.var_cox);
        let eq11_e1331_d_n7: f64 = (eq11_e1329_d_n7 * locals.var_cox);
        let eq11_e1331_d_n8: f64 = (eq11_e1329_d_n8 * locals.var_cox);
        let eq11_e1331_d_n9: f64 = (eq11_e1329_d_n9 * locals.var_cox);
        let eq11_e1331_d_n10: f64 = (eq11_e1329_d_n10 * locals.var_cox);
        let eq11_e1331_d_n11: f64 = (eq11_e1329_d_n11 * locals.var_cox);
        let eq11_e1331_d_n12: f64 = (eq11_e1329_d_n12 * locals.var_cox);
        let eq11_e1331_d_n13: f64 = (eq11_e1329_d_n13 * locals.var_cox);
        let eq11_e1331_d_n14: f64 = (eq11_e1329_d_n14 * locals.var_cox);
        let eq11_e1333: f64 = (eq11_e1331 * locals.var_weff);
        let eq11_e1333_d_n0: f64 = (eq11_e1331_d_n0 * locals.var_weff);
        let eq11_e1333_d_n2: f64 = (eq11_e1331_d_n2 * locals.var_weff);
        let eq11_e1333_d_n3: f64 = (eq11_e1331_d_n3 * locals.var_weff);
        let eq11_e1333_d_n4: f64 = (eq11_e1331_d_n4 * locals.var_weff);
        let eq11_e1333_d_n5: f64 = (eq11_e1331_d_n5 * locals.var_weff);
        let eq11_e1333_d_n6: f64 = (eq11_e1331_d_n6 * locals.var_weff);
        let eq11_e1333_d_n7: f64 = (eq11_e1331_d_n7 * locals.var_weff);
        let eq11_e1333_d_n8: f64 = (eq11_e1331_d_n8 * locals.var_weff);
        let eq11_e1333_d_n9: f64 = (eq11_e1331_d_n9 * locals.var_weff);
        let eq11_e1333_d_n10: f64 = (eq11_e1331_d_n10 * locals.var_weff);
        let eq11_e1333_d_n11: f64 = (eq11_e1331_d_n11 * locals.var_weff);
        let eq11_e1333_d_n12: f64 = (eq11_e1331_d_n12 * locals.var_weff);
        let eq11_e1333_d_n13: f64 = (eq11_e1331_d_n13 * locals.var_weff);
        let eq11_e1333_d_n14: f64 = (eq11_e1331_d_n14 * locals.var_weff);
        let eq11_e1335: f64 = (eq11_e1333 * p.p2);
        let eq11_e1335_d_n0: f64 = (eq11_e1333_d_n0 * p.p2);
        let eq11_e1335_d_n2: f64 = (eq11_e1333_d_n2 * p.p2);
        let eq11_e1335_d_n3: f64 = (eq11_e1333_d_n3 * p.p2);
        let eq11_e1335_d_n4: f64 = (eq11_e1333_d_n4 * p.p2);
        let eq11_e1335_d_n5: f64 = (eq11_e1333_d_n5 * p.p2);
        let eq11_e1335_d_n6: f64 = (eq11_e1333_d_n6 * p.p2);
        let eq11_e1335_d_n7: f64 = (eq11_e1333_d_n7 * p.p2);
        let eq11_e1335_d_n8: f64 = (eq11_e1333_d_n8 * p.p2);
        let eq11_e1335_d_n9: f64 = (eq11_e1333_d_n9 * p.p2);
        let eq11_e1335_d_n10: f64 = (eq11_e1333_d_n10 * p.p2);
        let eq11_e1335_d_n11: f64 = (eq11_e1333_d_n11 * p.p2);
        let eq11_e1335_d_n12: f64 = (eq11_e1333_d_n12 * p.p2);
        let eq11_e1335_d_n13: f64 = (eq11_e1333_d_n13 * p.p2);
        let eq11_e1335_d_n14: f64 = (eq11_e1333_d_n14 * p.p2);
        let eq11_e1337: f64 = (eq11_e1335 * locals.var_leff);
        let eq11_e1337_d_n0: f64 = (eq11_e1335_d_n0 * locals.var_leff);
        let eq11_e1337_d_n2: f64 = (eq11_e1335_d_n2 * locals.var_leff);
        let eq11_e1337_d_n3: f64 = (eq11_e1335_d_n3 * locals.var_leff);
        let eq11_e1337_d_n4: f64 = (eq11_e1335_d_n4 * locals.var_leff);
        let eq11_e1337_d_n5: f64 = (eq11_e1335_d_n5 * locals.var_leff);
        let eq11_e1337_d_n6: f64 = (eq11_e1335_d_n6 * locals.var_leff);
        let eq11_e1337_d_n7: f64 = (eq11_e1335_d_n7 * locals.var_leff);
        let eq11_e1337_d_n8: f64 = (eq11_e1335_d_n8 * locals.var_leff);
        let eq11_e1337_d_n9: f64 = (eq11_e1335_d_n9 * locals.var_leff);
        let eq11_e1337_d_n10: f64 = (eq11_e1335_d_n10 * locals.var_leff);
        let eq11_e1337_d_n11: f64 = (eq11_e1335_d_n11 * locals.var_leff);
        let eq11_e1337_d_n12: f64 = (eq11_e1335_d_n12 * locals.var_leff);
        let eq11_e1337_d_n13: f64 = (eq11_e1335_d_n13 * locals.var_leff);
        let eq11_e1337_d_n14: f64 = (eq11_e1335_d_n14 * locals.var_leff);
        let eq11_e1339: f64 = (eq11_e1337 * (nv15 - 0.0));
        let eq11_e1339_d_n0: f64 = (eq11_e1337_d_n0 * (nv15 - 0.0));
        let eq11_e1339_d_n2: f64 = (eq11_e1337_d_n2 * (nv15 - 0.0));
        let eq11_e1339_d_n3: f64 = (eq11_e1337_d_n3 * (nv15 - 0.0));
        let eq11_e1339_d_n4: f64 = (eq11_e1337_d_n4 * (nv15 - 0.0));
        let eq11_e1339_d_n5: f64 = (eq11_e1337_d_n5 * (nv15 - 0.0));
        let eq11_e1339_d_n6: f64 = (eq11_e1337_d_n6 * (nv15 - 0.0));
        let eq11_e1339_d_n7: f64 = (eq11_e1337_d_n7 * (nv15 - 0.0));
        let eq11_e1339_d_n8: f64 = (eq11_e1337_d_n8 * (nv15 - 0.0));
        let eq11_e1339_d_n9: f64 = (eq11_e1337_d_n9 * (nv15 - 0.0));
        let eq11_e1339_d_n10: f64 = (eq11_e1337_d_n10 * (nv15 - 0.0));
        let eq11_e1339_d_n11: f64 = (eq11_e1337_d_n11 * (nv15 - 0.0));
        let eq11_e1339_d_n12: f64 = (eq11_e1337_d_n12 * (nv15 - 0.0));
        let eq11_e1339_d_n13: f64 = (eq11_e1337_d_n13 * (nv15 - 0.0));
        let eq11_e1339_d_n14: f64 = (eq11_e1337_d_n14 * (nv15 - 0.0));
        let eq11_e1340: f64 = (0.5 * eq11_e1339);
        let eq11_e1340_d_n0: f64 = (0.5 * eq11_e1339_d_n0);
        let eq11_e1340_d_n2: f64 = (0.5 * eq11_e1339_d_n2);
        let eq11_e1340_d_n3: f64 = (0.5 * eq11_e1339_d_n3);
        let eq11_e1340_d_n4: f64 = (0.5 * eq11_e1339_d_n4);
        let eq11_e1340_d_n5: f64 = (0.5 * eq11_e1339_d_n5);
        let eq11_e1340_d_n6: f64 = (0.5 * eq11_e1339_d_n6);
        let eq11_e1340_d_n7: f64 = (0.5 * eq11_e1339_d_n7);
        let eq11_e1340_d_n8: f64 = (0.5 * eq11_e1339_d_n8);
        let eq11_e1340_d_n9: f64 = (0.5 * eq11_e1339_d_n9);
        let eq11_e1340_d_n10: f64 = (0.5 * eq11_e1339_d_n10);
        let eq11_e1340_d_n11: f64 = (0.5 * eq11_e1339_d_n11);
        let eq11_e1340_d_n12: f64 = (0.5 * eq11_e1339_d_n12);
        let eq11_e1340_d_n13: f64 = (0.5 * eq11_e1339_d_n13);
        let eq11_e1340_d_n14: f64 = (0.5 * eq11_e1339_d_n14);
        let eq11_e1340_d_n15: f64 = (0.5 * eq11_e1337);
        let eq11_e1341_q: f64 = eq11_e1340;
        let eq11_e1342: f64 = (p.p29 * eq11_e1340);
        let eq11_e1342_d_n0: f64 = (p.p29 * eq11_e1340_d_n0);
        let eq11_e1342_d_n2: f64 = (p.p29 * eq11_e1340_d_n2);
        let eq11_e1342_d_n3: f64 = (p.p29 * eq11_e1340_d_n3);
        let eq11_e1342_d_n4: f64 = (p.p29 * eq11_e1340_d_n4);
        let eq11_e1342_d_n5: f64 = (p.p29 * eq11_e1340_d_n5);
        let eq11_e1342_d_n6: f64 = (p.p29 * eq11_e1340_d_n6);
        let eq11_e1342_d_n7: f64 = (p.p29 * eq11_e1340_d_n7);
        let eq11_e1342_d_n8: f64 = (p.p29 * eq11_e1340_d_n8);
        let eq11_e1342_d_n9: f64 = (p.p29 * eq11_e1340_d_n9);
        let eq11_e1342_d_n10: f64 = (p.p29 * eq11_e1340_d_n10);
        let eq11_e1342_d_n11: f64 = (p.p29 * eq11_e1340_d_n11);
        let eq11_e1342_d_n12: f64 = (p.p29 * eq11_e1340_d_n12);
        let eq11_e1342_d_n13: f64 = (p.p29 * eq11_e1340_d_n13);
        let eq11_e1342_d_n14: f64 = (p.p29 * eq11_e1340_d_n14);
        let eq11_e1342_d_n15: f64 = (p.p29 * eq11_e1340_d_n15);
        let eq11_e1342_q: f64 = (p.p29 * eq11_e1341_q);
        (eq11_e1342, eq11_e1342_d_n0, eq11_e1342_d_n2, eq11_e1342_d_n3, eq11_e1342_d_n4, eq11_e1342_d_n5, eq11_e1342_d_n6, eq11_e1342_d_n7, eq11_e1342_d_n8, eq11_e1342_d_n9, eq11_e1342_d_n10, eq11_e1342_d_n11, eq11_e1342_d_n12, eq11_e1342_d_n13, eq11_e1342_d_n14, eq11_e1342_d_n15, eq11_e1342_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq11_reactive_node_derivatives: [f64; 17] = [eq11_e1344_d_n0, 0.0, eq11_e1344_d_n2, eq11_e1344_d_n3, eq11_e1344_d_n4, eq11_e1344_d_n5, eq11_e1344_d_n6, eq11_e1344_d_n7, eq11_e1344_d_n8, eq11_e1344_d_n9, eq11_e1344_d_n10, eq11_e1344_d_n11, eq11_e1344_d_n12, eq11_e1344_d_n13, eq11_e1344_d_n14, eq11_e1344_d_n15, 0.0];
        let eq11_reactive_branch_derivatives: [f64; 14] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            nodes,
            &eq11_reactive_node_derivatives,
            branches,
            &eq11_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq12_e1370, eq12_e1370_d_n0, eq12_e1370_d_n2, eq12_e1370_d_n3, eq12_e1370_d_n4, eq12_e1370_d_n5, eq12_e1370_d_n6, eq12_e1370_d_n7, eq12_e1370_d_n8, eq12_e1370_d_n9, eq12_e1370_d_n10, eq12_e1370_d_n11, eq12_e1370_d_n12, eq12_e1370_d_n13, eq12_e1370_d_n14, eq12_e1370_d_n15, eq12_e1370_q,) = {
    if ((locals.var_guard698 != 0.0) && (locals.var_guard697 == 0.0)) {
        let eq12_e1353: f64 = (1.0 - locals.var_sigvds);
        let eq12_e1355: f64 = (eq12_e1353 * locals.var_mig);
        let eq12_e1355_d_n0: f64 = (eq12_e1353 * locals.var_mig_dn0);
        let eq12_e1355_d_n2: f64 = (eq12_e1353 * locals.var_mig_dn2);
        let eq12_e1355_d_n3: f64 = (eq12_e1353 * locals.var_mig_dn3);
        let eq12_e1355_d_n4: f64 = (eq12_e1353 * locals.var_mig_dn4);
        let eq12_e1355_d_n5: f64 = (eq12_e1353 * locals.var_mig_dn5);
        let eq12_e1355_d_n6: f64 = (eq12_e1353 * locals.var_mig_dn6);
        let eq12_e1355_d_n7: f64 = (eq12_e1353 * locals.var_mig_dn7);
        let eq12_e1355_d_n8: f64 = (eq12_e1353 * locals.var_mig_dn8);
        let eq12_e1355_d_n9: f64 = (eq12_e1353 * locals.var_mig_dn9);
        let eq12_e1355_d_n10: f64 = (eq12_e1353 * locals.var_mig_dn10);
        let eq12_e1355_d_n11: f64 = (eq12_e1353 * locals.var_mig_dn11);
        let eq12_e1355_d_n12: f64 = (eq12_e1353 * locals.var_mig_dn12);
        let eq12_e1355_d_n13: f64 = (eq12_e1353 * locals.var_mig_dn13);
        let eq12_e1355_d_n14: f64 = (eq12_e1353 * locals.var_mig_dn14);
        let eq12_e1357: f64 = (eq12_e1355 * locals.var_cox);
        let eq12_e1357_d_n0: f64 = (eq12_e1355_d_n0 * locals.var_cox);
        let eq12_e1357_d_n2: f64 = (eq12_e1355_d_n2 * locals.var_cox);
        let eq12_e1357_d_n3: f64 = (eq12_e1355_d_n3 * locals.var_cox);
        let eq12_e1357_d_n4: f64 = (eq12_e1355_d_n4 * locals.var_cox);
        let eq12_e1357_d_n5: f64 = (eq12_e1355_d_n5 * locals.var_cox);
        let eq12_e1357_d_n6: f64 = (eq12_e1355_d_n6 * locals.var_cox);
        let eq12_e1357_d_n7: f64 = (eq12_e1355_d_n7 * locals.var_cox);
        let eq12_e1357_d_n8: f64 = (eq12_e1355_d_n8 * locals.var_cox);
        let eq12_e1357_d_n9: f64 = (eq12_e1355_d_n9 * locals.var_cox);
        let eq12_e1357_d_n10: f64 = (eq12_e1355_d_n10 * locals.var_cox);
        let eq12_e1357_d_n11: f64 = (eq12_e1355_d_n11 * locals.var_cox);
        let eq12_e1357_d_n12: f64 = (eq12_e1355_d_n12 * locals.var_cox);
        let eq12_e1357_d_n13: f64 = (eq12_e1355_d_n13 * locals.var_cox);
        let eq12_e1357_d_n14: f64 = (eq12_e1355_d_n14 * locals.var_cox);
        let eq12_e1359: f64 = (eq12_e1357 * locals.var_weff);
        let eq12_e1359_d_n0: f64 = (eq12_e1357_d_n0 * locals.var_weff);
        let eq12_e1359_d_n2: f64 = (eq12_e1357_d_n2 * locals.var_weff);
        let eq12_e1359_d_n3: f64 = (eq12_e1357_d_n3 * locals.var_weff);
        let eq12_e1359_d_n4: f64 = (eq12_e1357_d_n4 * locals.var_weff);
        let eq12_e1359_d_n5: f64 = (eq12_e1357_d_n5 * locals.var_weff);
        let eq12_e1359_d_n6: f64 = (eq12_e1357_d_n6 * locals.var_weff);
        let eq12_e1359_d_n7: f64 = (eq12_e1357_d_n7 * locals.var_weff);
        let eq12_e1359_d_n8: f64 = (eq12_e1357_d_n8 * locals.var_weff);
        let eq12_e1359_d_n9: f64 = (eq12_e1357_d_n9 * locals.var_weff);
        let eq12_e1359_d_n10: f64 = (eq12_e1357_d_n10 * locals.var_weff);
        let eq12_e1359_d_n11: f64 = (eq12_e1357_d_n11 * locals.var_weff);
        let eq12_e1359_d_n12: f64 = (eq12_e1357_d_n12 * locals.var_weff);
        let eq12_e1359_d_n13: f64 = (eq12_e1357_d_n13 * locals.var_weff);
        let eq12_e1359_d_n14: f64 = (eq12_e1357_d_n14 * locals.var_weff);
        let eq12_e1361: f64 = (eq12_e1359 * p.p2);
        let eq12_e1361_d_n0: f64 = (eq12_e1359_d_n0 * p.p2);
        let eq12_e1361_d_n2: f64 = (eq12_e1359_d_n2 * p.p2);
        let eq12_e1361_d_n3: f64 = (eq12_e1359_d_n3 * p.p2);
        let eq12_e1361_d_n4: f64 = (eq12_e1359_d_n4 * p.p2);
        let eq12_e1361_d_n5: f64 = (eq12_e1359_d_n5 * p.p2);
        let eq12_e1361_d_n6: f64 = (eq12_e1359_d_n6 * p.p2);
        let eq12_e1361_d_n7: f64 = (eq12_e1359_d_n7 * p.p2);
        let eq12_e1361_d_n8: f64 = (eq12_e1359_d_n8 * p.p2);
        let eq12_e1361_d_n9: f64 = (eq12_e1359_d_n9 * p.p2);
        let eq12_e1361_d_n10: f64 = (eq12_e1359_d_n10 * p.p2);
        let eq12_e1361_d_n11: f64 = (eq12_e1359_d_n11 * p.p2);
        let eq12_e1361_d_n12: f64 = (eq12_e1359_d_n12 * p.p2);
        let eq12_e1361_d_n13: f64 = (eq12_e1359_d_n13 * p.p2);
        let eq12_e1361_d_n14: f64 = (eq12_e1359_d_n14 * p.p2);
        let eq12_e1363: f64 = (eq12_e1361 * locals.var_leff);
        let eq12_e1363_d_n0: f64 = (eq12_e1361_d_n0 * locals.var_leff);
        let eq12_e1363_d_n2: f64 = (eq12_e1361_d_n2 * locals.var_leff);
        let eq12_e1363_d_n3: f64 = (eq12_e1361_d_n3 * locals.var_leff);
        let eq12_e1363_d_n4: f64 = (eq12_e1361_d_n4 * locals.var_leff);
        let eq12_e1363_d_n5: f64 = (eq12_e1361_d_n5 * locals.var_leff);
        let eq12_e1363_d_n6: f64 = (eq12_e1361_d_n6 * locals.var_leff);
        let eq12_e1363_d_n7: f64 = (eq12_e1361_d_n7 * locals.var_leff);
        let eq12_e1363_d_n8: f64 = (eq12_e1361_d_n8 * locals.var_leff);
        let eq12_e1363_d_n9: f64 = (eq12_e1361_d_n9 * locals.var_leff);
        let eq12_e1363_d_n10: f64 = (eq12_e1361_d_n10 * locals.var_leff);
        let eq12_e1363_d_n11: f64 = (eq12_e1361_d_n11 * locals.var_leff);
        let eq12_e1363_d_n12: f64 = (eq12_e1361_d_n12 * locals.var_leff);
        let eq12_e1363_d_n13: f64 = (eq12_e1361_d_n13 * locals.var_leff);
        let eq12_e1363_d_n14: f64 = (eq12_e1361_d_n14 * locals.var_leff);
        let eq12_e1365: f64 = (eq12_e1363 * (nv15 - 0.0));
        let eq12_e1365_d_n0: f64 = (eq12_e1363_d_n0 * (nv15 - 0.0));
        let eq12_e1365_d_n2: f64 = (eq12_e1363_d_n2 * (nv15 - 0.0));
        let eq12_e1365_d_n3: f64 = (eq12_e1363_d_n3 * (nv15 - 0.0));
        let eq12_e1365_d_n4: f64 = (eq12_e1363_d_n4 * (nv15 - 0.0));
        let eq12_e1365_d_n5: f64 = (eq12_e1363_d_n5 * (nv15 - 0.0));
        let eq12_e1365_d_n6: f64 = (eq12_e1363_d_n6 * (nv15 - 0.0));
        let eq12_e1365_d_n7: f64 = (eq12_e1363_d_n7 * (nv15 - 0.0));
        let eq12_e1365_d_n8: f64 = (eq12_e1363_d_n8 * (nv15 - 0.0));
        let eq12_e1365_d_n9: f64 = (eq12_e1363_d_n9 * (nv15 - 0.0));
        let eq12_e1365_d_n10: f64 = (eq12_e1363_d_n10 * (nv15 - 0.0));
        let eq12_e1365_d_n11: f64 = (eq12_e1363_d_n11 * (nv15 - 0.0));
        let eq12_e1365_d_n12: f64 = (eq12_e1363_d_n12 * (nv15 - 0.0));
        let eq12_e1365_d_n13: f64 = (eq12_e1363_d_n13 * (nv15 - 0.0));
        let eq12_e1365_d_n14: f64 = (eq12_e1363_d_n14 * (nv15 - 0.0));
        let eq12_e1366: f64 = (0.5 * eq12_e1365);
        let eq12_e1366_d_n0: f64 = (0.5 * eq12_e1365_d_n0);
        let eq12_e1366_d_n2: f64 = (0.5 * eq12_e1365_d_n2);
        let eq12_e1366_d_n3: f64 = (0.5 * eq12_e1365_d_n3);
        let eq12_e1366_d_n4: f64 = (0.5 * eq12_e1365_d_n4);
        let eq12_e1366_d_n5: f64 = (0.5 * eq12_e1365_d_n5);
        let eq12_e1366_d_n6: f64 = (0.5 * eq12_e1365_d_n6);
        let eq12_e1366_d_n7: f64 = (0.5 * eq12_e1365_d_n7);
        let eq12_e1366_d_n8: f64 = (0.5 * eq12_e1365_d_n8);
        let eq12_e1366_d_n9: f64 = (0.5 * eq12_e1365_d_n9);
        let eq12_e1366_d_n10: f64 = (0.5 * eq12_e1365_d_n10);
        let eq12_e1366_d_n11: f64 = (0.5 * eq12_e1365_d_n11);
        let eq12_e1366_d_n12: f64 = (0.5 * eq12_e1365_d_n12);
        let eq12_e1366_d_n13: f64 = (0.5 * eq12_e1365_d_n13);
        let eq12_e1366_d_n14: f64 = (0.5 * eq12_e1365_d_n14);
        let eq12_e1366_d_n15: f64 = (0.5 * eq12_e1363);
        let eq12_e1367_q: f64 = eq12_e1366;
        let eq12_e1368: f64 = (p.p29 * eq12_e1366);
        let eq12_e1368_d_n0: f64 = (p.p29 * eq12_e1366_d_n0);
        let eq12_e1368_d_n2: f64 = (p.p29 * eq12_e1366_d_n2);
        let eq12_e1368_d_n3: f64 = (p.p29 * eq12_e1366_d_n3);
        let eq12_e1368_d_n4: f64 = (p.p29 * eq12_e1366_d_n4);
        let eq12_e1368_d_n5: f64 = (p.p29 * eq12_e1366_d_n5);
        let eq12_e1368_d_n6: f64 = (p.p29 * eq12_e1366_d_n6);
        let eq12_e1368_d_n7: f64 = (p.p29 * eq12_e1366_d_n7);
        let eq12_e1368_d_n8: f64 = (p.p29 * eq12_e1366_d_n8);
        let eq12_e1368_d_n9: f64 = (p.p29 * eq12_e1366_d_n9);
        let eq12_e1368_d_n10: f64 = (p.p29 * eq12_e1366_d_n10);
        let eq12_e1368_d_n11: f64 = (p.p29 * eq12_e1366_d_n11);
        let eq12_e1368_d_n12: f64 = (p.p29 * eq12_e1366_d_n12);
        let eq12_e1368_d_n13: f64 = (p.p29 * eq12_e1366_d_n13);
        let eq12_e1368_d_n14: f64 = (p.p29 * eq12_e1366_d_n14);
        let eq12_e1368_d_n15: f64 = (p.p29 * eq12_e1366_d_n15);
        let eq12_e1368_q: f64 = (p.p29 * eq12_e1367_q);
        (eq12_e1368, eq12_e1368_d_n0, eq12_e1368_d_n2, eq12_e1368_d_n3, eq12_e1368_d_n4, eq12_e1368_d_n5, eq12_e1368_d_n6, eq12_e1368_d_n7, eq12_e1368_d_n8, eq12_e1368_d_n9, eq12_e1368_d_n10, eq12_e1368_d_n11, eq12_e1368_d_n12, eq12_e1368_d_n13, eq12_e1368_d_n14, eq12_e1368_d_n15, eq12_e1368_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq12_reactive_node_derivatives: [f64; 17] = [eq12_e1370_d_n0, 0.0, eq12_e1370_d_n2, eq12_e1370_d_n3, eq12_e1370_d_n4, eq12_e1370_d_n5, eq12_e1370_d_n6, eq12_e1370_d_n7, eq12_e1370_d_n8, eq12_e1370_d_n9, eq12_e1370_d_n10, eq12_e1370_d_n11, eq12_e1370_d_n12, eq12_e1370_d_n13, eq12_e1370_d_n14, eq12_e1370_d_n15, 0.0];
        let eq12_reactive_branch_derivatives: [f64; 14] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[5]),
            nodes,
            &eq12_reactive_node_derivatives,
            branches,
            &eq12_reactive_branch_derivatives,
            multiplicity,
        );
        let eq19_e1428_q: f64 = locals.var_qgi_1;
        let eq19_reactive_node_derivatives: [f64; 17] = [locals.var_qgi_1_dn0, 0.0, locals.var_qgi_1_dn2, locals.var_qgi_1_dn3, locals.var_qgi_1_dn4, locals.var_qgi_1_dn5, locals.var_qgi_1_dn6, locals.var_qgi_1_dn7, locals.var_qgi_1_dn8, locals.var_qgi_1_dn9, locals.var_qgi_1_dn10, locals.var_qgi_1_dn11, locals.var_qgi_1_dn12, locals.var_qgi_1_dn13, locals.var_qgi_1_dn14, 0.0, 0.0];
        let eq19_reactive_branch_derivatives: [f64; 14] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[11]),
            nodes,
            &eq19_reactive_node_derivatives,
            branches,
            &eq19_reactive_branch_derivatives,
            multiplicity,
        );
        let eq20_e1430_q: f64 = locals.var_qsi_1;
        let eq20_reactive_node_derivatives: [f64; 17] = [locals.var_qsi_1_dn0, 0.0, locals.var_qsi_1_dn2, locals.var_qsi_1_dn3, locals.var_qsi_1_dn4, locals.var_qsi_1_dn5, locals.var_qsi_1_dn6, locals.var_qsi_1_dn7, locals.var_qsi_1_dn8, locals.var_qsi_1_dn9, locals.var_qsi_1_dn10, locals.var_qsi_1_dn11, locals.var_qsi_1_dn12, locals.var_qsi_1_dn13, locals.var_qsi_1_dn14, 0.0, 0.0];
        let eq20_reactive_branch_derivatives: [f64; 14] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[11]),
            nodes,
            &eq20_reactive_node_derivatives,
            branches,
            &eq20_reactive_branch_derivatives,
            multiplicity,
        );
        let eq21_e1432_q: f64 = locals.var_qdi_1;
        let eq21_reactive_node_derivatives: [f64; 17] = [locals.var_qdi_1_dn0, 0.0, locals.var_qdi_1_dn2, locals.var_qdi_1_dn3, locals.var_qdi_1_dn4, locals.var_qdi_1_dn5, locals.var_qdi_1_dn6, locals.var_qdi_1_dn7, locals.var_qdi_1_dn8, locals.var_qdi_1_dn9, locals.var_qdi_1_dn10, locals.var_qdi_1_dn11, locals.var_qdi_1_dn12, locals.var_qdi_1_dn13, locals.var_qdi_1_dn14, 0.0, 0.0];
        let eq21_reactive_branch_derivatives: [f64; 14] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[11]),
            nodes,
            &eq21_reactive_node_derivatives,
            branches,
            &eq21_reactive_branch_derivatives,
            multiplicity,
        );
        let eq22_e1435: f64 = (-locals.var_devsign);
        let eq22_e1437: f64 = (eq22_e1435 * locals.var_qovs);
        let eq22_e1437_d_n0: f64 = (eq22_e1435 * locals.var_qovs_dn0);
        let eq22_e1437_d_n2: f64 = (eq22_e1435 * locals.var_qovs_dn2);
        let eq22_e1437_d_n3: f64 = (eq22_e1435 * locals.var_qovs_dn3);
        let eq22_e1437_d_n4: f64 = (eq22_e1435 * locals.var_qovs_dn4);
        let eq22_e1437_d_n5: f64 = (eq22_e1435 * locals.var_qovs_dn5);
        let eq22_e1437_d_n6: f64 = (eq22_e1435 * locals.var_qovs_dn6);
        let eq22_e1437_d_n7: f64 = (eq22_e1435 * locals.var_qovs_dn7);
        let eq22_e1437_d_n8: f64 = (eq22_e1435 * locals.var_qovs_dn8);
        let eq22_e1437_d_n9: f64 = (eq22_e1435 * locals.var_qovs_dn9);
        let eq22_e1437_d_n10: f64 = (eq22_e1435 * locals.var_qovs_dn10);
        let eq22_e1437_d_n11: f64 = (eq22_e1435 * locals.var_qovs_dn11);
        let eq22_e1437_d_n12: f64 = (eq22_e1435 * locals.var_qovs_dn12);
        let eq22_e1437_d_n13: f64 = (eq22_e1435 * locals.var_qovs_dn13);
        let eq22_e1437_d_n14: f64 = (eq22_e1435 * locals.var_qovs_dn14);
        let eq22_e1438_q: f64 = eq22_e1437;
        let eq22_e1439: f64 = (p.p29 * eq22_e1437);
        let eq22_e1439_d_n0: f64 = (p.p29 * eq22_e1437_d_n0);
        let eq22_e1439_d_n2: f64 = (p.p29 * eq22_e1437_d_n2);
        let eq22_e1439_d_n3: f64 = (p.p29 * eq22_e1437_d_n3);
        let eq22_e1439_d_n4: f64 = (p.p29 * eq22_e1437_d_n4);
        let eq22_e1439_d_n5: f64 = (p.p29 * eq22_e1437_d_n5);
        let eq22_e1439_d_n6: f64 = (p.p29 * eq22_e1437_d_n6);
        let eq22_e1439_d_n7: f64 = (p.p29 * eq22_e1437_d_n7);
        let eq22_e1439_d_n8: f64 = (p.p29 * eq22_e1437_d_n8);
        let eq22_e1439_d_n9: f64 = (p.p29 * eq22_e1437_d_n9);
        let eq22_e1439_d_n10: f64 = (p.p29 * eq22_e1437_d_n10);
        let eq22_e1439_d_n11: f64 = (p.p29 * eq22_e1437_d_n11);
        let eq22_e1439_d_n12: f64 = (p.p29 * eq22_e1437_d_n12);
        let eq22_e1439_d_n13: f64 = (p.p29 * eq22_e1437_d_n13);
        let eq22_e1439_d_n14: f64 = (p.p29 * eq22_e1437_d_n14);
        let eq22_e1439_q: f64 = (p.p29 * eq22_e1438_q);
        let eq22_reactive_node_derivatives: [f64; 17] = [eq22_e1439_d_n0, 0.0, eq22_e1439_d_n2, eq22_e1439_d_n3, eq22_e1439_d_n4, eq22_e1439_d_n5, eq22_e1439_d_n6, eq22_e1439_d_n7, eq22_e1439_d_n8, eq22_e1439_d_n9, eq22_e1439_d_n10, eq22_e1439_d_n11, eq22_e1439_d_n12, eq22_e1439_d_n13, eq22_e1439_d_n14, 0.0, 0.0];
        let eq22_reactive_branch_derivatives: [f64; 14] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[7]),
            nodes,
            &eq22_reactive_node_derivatives,
            branches,
            &eq22_reactive_branch_derivatives,
            multiplicity,
        );
        let eq23_e1442: f64 = (-locals.var_devsign);
        let eq23_e1444: f64 = (eq23_e1442 * locals.var_qovd);
        let eq23_e1444_d_n0: f64 = (eq23_e1442 * locals.var_qovd_dn0);
        let eq23_e1444_d_n2: f64 = (eq23_e1442 * locals.var_qovd_dn2);
        let eq23_e1444_d_n3: f64 = (eq23_e1442 * locals.var_qovd_dn3);
        let eq23_e1444_d_n4: f64 = (eq23_e1442 * locals.var_qovd_dn4);
        let eq23_e1444_d_n5: f64 = (eq23_e1442 * locals.var_qovd_dn5);
        let eq23_e1444_d_n6: f64 = (eq23_e1442 * locals.var_qovd_dn6);
        let eq23_e1444_d_n7: f64 = (eq23_e1442 * locals.var_qovd_dn7);
        let eq23_e1444_d_n8: f64 = (eq23_e1442 * locals.var_qovd_dn8);
        let eq23_e1444_d_n9: f64 = (eq23_e1442 * locals.var_qovd_dn9);
        let eq23_e1444_d_n10: f64 = (eq23_e1442 * locals.var_qovd_dn10);
        let eq23_e1444_d_n11: f64 = (eq23_e1442 * locals.var_qovd_dn11);
        let eq23_e1444_d_n12: f64 = (eq23_e1442 * locals.var_qovd_dn12);
        let eq23_e1444_d_n13: f64 = (eq23_e1442 * locals.var_qovd_dn13);
        let eq23_e1444_d_n14: f64 = (eq23_e1442 * locals.var_qovd_dn14);
        let eq23_e1445_q: f64 = eq23_e1444;
        let eq23_e1446: f64 = (p.p29 * eq23_e1444);
        let eq23_e1446_d_n0: f64 = (p.p29 * eq23_e1444_d_n0);
        let eq23_e1446_d_n2: f64 = (p.p29 * eq23_e1444_d_n2);
        let eq23_e1446_d_n3: f64 = (p.p29 * eq23_e1444_d_n3);
        let eq23_e1446_d_n4: f64 = (p.p29 * eq23_e1444_d_n4);
        let eq23_e1446_d_n5: f64 = (p.p29 * eq23_e1444_d_n5);
        let eq23_e1446_d_n6: f64 = (p.p29 * eq23_e1444_d_n6);
        let eq23_e1446_d_n7: f64 = (p.p29 * eq23_e1444_d_n7);
        let eq23_e1446_d_n8: f64 = (p.p29 * eq23_e1444_d_n8);
        let eq23_e1446_d_n9: f64 = (p.p29 * eq23_e1444_d_n9);
        let eq23_e1446_d_n10: f64 = (p.p29 * eq23_e1444_d_n10);
        let eq23_e1446_d_n11: f64 = (p.p29 * eq23_e1444_d_n11);
        let eq23_e1446_d_n12: f64 = (p.p29 * eq23_e1444_d_n12);
        let eq23_e1446_d_n13: f64 = (p.p29 * eq23_e1444_d_n13);
        let eq23_e1446_d_n14: f64 = (p.p29 * eq23_e1444_d_n14);
        let eq23_e1446_q: f64 = (p.p29 * eq23_e1445_q);
        let eq23_reactive_node_derivatives: [f64; 17] = [eq23_e1446_d_n0, 0.0, eq23_e1446_d_n2, eq23_e1446_d_n3, eq23_e1446_d_n4, eq23_e1446_d_n5, eq23_e1446_d_n6, eq23_e1446_d_n7, eq23_e1446_d_n8, eq23_e1446_d_n9, eq23_e1446_d_n10, eq23_e1446_d_n11, eq23_e1446_d_n12, eq23_e1446_d_n13, eq23_e1446_d_n14, 0.0, 0.0];
        let eq23_reactive_branch_derivatives: [f64; 14] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[5]),
            nodes,
            &eq23_reactive_node_derivatives,
            branches,
            &eq23_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_1(
        stamper: &mut GeneratedReactiveStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        locals: &mut StampLocals,
    ) {
        let eq24_e1449: f64 = (-locals.var_devsign);
        let eq24_e1451: f64 = (eq24_e1449 * locals.var_qovb);
        let eq24_e1451_d_n0: f64 = (eq24_e1449 * locals.var_qovb_dn0);
        let eq24_e1451_d_n2: f64 = (eq24_e1449 * locals.var_qovb_dn2);
        let eq24_e1451_d_n3: f64 = (eq24_e1449 * locals.var_qovb_dn3);
        let eq24_e1451_d_n4: f64 = (eq24_e1449 * locals.var_qovb_dn4);
        let eq24_e1451_d_n5: f64 = (eq24_e1449 * locals.var_qovb_dn5);
        let eq24_e1451_d_n6: f64 = (eq24_e1449 * locals.var_qovb_dn6);
        let eq24_e1451_d_n7: f64 = (eq24_e1449 * locals.var_qovb_dn7);
        let eq24_e1451_d_n8: f64 = (eq24_e1449 * locals.var_qovb_dn8);
        let eq24_e1451_d_n9: f64 = (eq24_e1449 * locals.var_qovb_dn9);
        let eq24_e1451_d_n10: f64 = (eq24_e1449 * locals.var_qovb_dn10);
        let eq24_e1451_d_n11: f64 = (eq24_e1449 * locals.var_qovb_dn11);
        let eq24_e1451_d_n12: f64 = (eq24_e1449 * locals.var_qovb_dn12);
        let eq24_e1451_d_n13: f64 = (eq24_e1449 * locals.var_qovb_dn13);
        let eq24_e1451_d_n14: f64 = (eq24_e1449 * locals.var_qovb_dn14);
        let eq24_e1452_q: f64 = eq24_e1451;
        let eq24_e1453: f64 = (p.p29 * eq24_e1451);
        let eq24_e1453_d_n0: f64 = (p.p29 * eq24_e1451_d_n0);
        let eq24_e1453_d_n2: f64 = (p.p29 * eq24_e1451_d_n2);
        let eq24_e1453_d_n3: f64 = (p.p29 * eq24_e1451_d_n3);
        let eq24_e1453_d_n4: f64 = (p.p29 * eq24_e1451_d_n4);
        let eq24_e1453_d_n5: f64 = (p.p29 * eq24_e1451_d_n5);
        let eq24_e1453_d_n6: f64 = (p.p29 * eq24_e1451_d_n6);
        let eq24_e1453_d_n7: f64 = (p.p29 * eq24_e1451_d_n7);
        let eq24_e1453_d_n8: f64 = (p.p29 * eq24_e1451_d_n8);
        let eq24_e1453_d_n9: f64 = (p.p29 * eq24_e1451_d_n9);
        let eq24_e1453_d_n10: f64 = (p.p29 * eq24_e1451_d_n10);
        let eq24_e1453_d_n11: f64 = (p.p29 * eq24_e1451_d_n11);
        let eq24_e1453_d_n12: f64 = (p.p29 * eq24_e1451_d_n12);
        let eq24_e1453_d_n13: f64 = (p.p29 * eq24_e1451_d_n13);
        let eq24_e1453_d_n14: f64 = (p.p29 * eq24_e1451_d_n14);
        let eq24_e1453_q: f64 = (p.p29 * eq24_e1452_q);
        let eq24_reactive_node_derivatives: [f64; 17] = [eq24_e1453_d_n0, 0.0, eq24_e1453_d_n2, eq24_e1453_d_n3, eq24_e1453_d_n4, eq24_e1453_d_n5, eq24_e1453_d_n6, eq24_e1453_d_n7, eq24_e1453_d_n8, eq24_e1453_d_n9, eq24_e1453_d_n10, eq24_e1453_d_n11, eq24_e1453_d_n12, eq24_e1453_d_n13, eq24_e1453_d_n14, 0.0, 0.0];
        let eq24_reactive_branch_derivatives: [f64; 14] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[11]),
            nodes,
            &eq24_reactive_node_derivatives,
            branches,
            &eq24_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq55_e1708, eq55_e1708_d_n0, eq55_e1708_d_n2, eq55_e1708_d_n3, eq55_e1708_d_n4, eq55_e1708_d_n5, eq55_e1708_d_n6, eq55_e1708_d_n7, eq55_e1708_d_n8, eq55_e1708_d_n9, eq55_e1708_d_n10, eq55_e1708_d_n11, eq55_e1708_d_n12, eq55_e1708_d_n13, eq55_e1708_d_n14, eq55_e1708_q, eq55_e1708_q_d_n4,) = {
    if (locals.var_guard763 != 0.0) {
        let eq55_e1699: f64 = (locals.var_deltemp1 * locals.var_gth);
        let eq55_e1699_d_n4: f64 = (locals.var_deltemp1_dn4 * locals.var_gth);
        let eq55_e1702: f64 = (locals.var_deltemp1 * locals.var_cth);
        let eq55_e1702_d_n4: f64 = (locals.var_deltemp1_dn4 * locals.var_cth);
        let eq55_e1703_q: f64 = eq55_e1702;
        let eq55_e1704: f64 = (eq55_e1699 + eq55_e1702);
        let eq55_e1704_d_n4: f64 = (eq55_e1699_d_n4 + eq55_e1702_d_n4);
        let eq55_e1704_q: f64 = eq55_e1703_q;
        let eq55_e1706: f64 = (eq55_e1704 - locals.var_pdiss);
        let eq55_e1706_d_n4: f64 = (eq55_e1704_d_n4 - locals.var_pdiss_dn4);
        let eq55_e1706_q: f64 = eq55_e1704_q;
        (eq55_e1706, (-locals.var_pdiss_dn0), (-locals.var_pdiss_dn2), (-locals.var_pdiss_dn3), eq55_e1706_d_n4, (-locals.var_pdiss_dn5), (-locals.var_pdiss_dn6), (-locals.var_pdiss_dn7), (-locals.var_pdiss_dn8), (-locals.var_pdiss_dn9), (-locals.var_pdiss_dn10), (-locals.var_pdiss_dn11), (-locals.var_pdiss_dn12), (-locals.var_pdiss_dn13), (-locals.var_pdiss_dn14), eq55_e1706_q, eq55_e1702_d_n4,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[4]),
            None,
            nodes[4],
            multiplicity * (eq55_e1708_q_d_n4),
        );
        let (eq71_e1841, eq71_e1841_d_n0, eq71_e1841_d_n2, eq71_e1841_d_n3, eq71_e1841_d_n4, eq71_e1841_d_n5, eq71_e1841_d_n6, eq71_e1841_d_n7, eq71_e1841_d_n8, eq71_e1841_d_n9, eq71_e1841_d_n10, eq71_e1841_d_n11, eq71_e1841_d_n12, eq71_e1841_d_n13, eq71_e1841_d_n14, eq71_e1841_q,) = {
    if (locals.var_guard769 != 0.0) {
        let eq71_e1837: f64 = (p.p29 * locals.var_qbsj);
        let eq71_e1837_d_n0: f64 = (p.p29 * locals.var_qbsj_dn0);
        let eq71_e1837_d_n2: f64 = (p.p29 * locals.var_qbsj_dn2);
        let eq71_e1837_d_n3: f64 = (p.p29 * locals.var_qbsj_dn3);
        let eq71_e1837_d_n4: f64 = (p.p29 * locals.var_qbsj_dn4);
        let eq71_e1837_d_n5: f64 = (p.p29 * locals.var_qbsj_dn5);
        let eq71_e1837_d_n6: f64 = (p.p29 * locals.var_qbsj_dn6);
        let eq71_e1837_d_n7: f64 = (p.p29 * locals.var_qbsj_dn7);
        let eq71_e1837_d_n8: f64 = (p.p29 * locals.var_qbsj_dn8);
        let eq71_e1837_d_n9: f64 = (p.p29 * locals.var_qbsj_dn9);
        let eq71_e1837_d_n10: f64 = (p.p29 * locals.var_qbsj_dn10);
        let eq71_e1837_d_n11: f64 = (p.p29 * locals.var_qbsj_dn11);
        let eq71_e1837_d_n12: f64 = (p.p29 * locals.var_qbsj_dn12);
        let eq71_e1837_d_n13: f64 = (p.p29 * locals.var_qbsj_dn13);
        let eq71_e1837_d_n14: f64 = (p.p29 * locals.var_qbsj_dn14);
        let eq71_e1838_q: f64 = eq71_e1837;
        let eq71_e1839: f64 = (locals.var_devsign * eq71_e1837);
        let eq71_e1839_d_n0: f64 = (locals.var_devsign * eq71_e1837_d_n0);
        let eq71_e1839_d_n2: f64 = (locals.var_devsign * eq71_e1837_d_n2);
        let eq71_e1839_d_n3: f64 = (locals.var_devsign * eq71_e1837_d_n3);
        let eq71_e1839_d_n4: f64 = (locals.var_devsign * eq71_e1837_d_n4);
        let eq71_e1839_d_n5: f64 = (locals.var_devsign * eq71_e1837_d_n5);
        let eq71_e1839_d_n6: f64 = (locals.var_devsign * eq71_e1837_d_n6);
        let eq71_e1839_d_n7: f64 = (locals.var_devsign * eq71_e1837_d_n7);
        let eq71_e1839_d_n8: f64 = (locals.var_devsign * eq71_e1837_d_n8);
        let eq71_e1839_d_n9: f64 = (locals.var_devsign * eq71_e1837_d_n9);
        let eq71_e1839_d_n10: f64 = (locals.var_devsign * eq71_e1837_d_n10);
        let eq71_e1839_d_n11: f64 = (locals.var_devsign * eq71_e1837_d_n11);
        let eq71_e1839_d_n12: f64 = (locals.var_devsign * eq71_e1837_d_n12);
        let eq71_e1839_d_n13: f64 = (locals.var_devsign * eq71_e1837_d_n13);
        let eq71_e1839_d_n14: f64 = (locals.var_devsign * eq71_e1837_d_n14);
        let eq71_e1839_q: f64 = (locals.var_devsign * eq71_e1838_q);
        (eq71_e1839, eq71_e1839_d_n0, eq71_e1839_d_n2, eq71_e1839_d_n3, eq71_e1839_d_n4, eq71_e1839_d_n5, eq71_e1839_d_n6, eq71_e1839_d_n7, eq71_e1839_d_n8, eq71_e1839_d_n9, eq71_e1839_d_n10, eq71_e1839_d_n11, eq71_e1839_d_n12, eq71_e1839_d_n13, eq71_e1839_d_n14, eq71_e1839_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq71_reactive_node_derivatives: [f64; 17] = [eq71_e1841_d_n0, 0.0, eq71_e1841_d_n2, eq71_e1841_d_n3, eq71_e1841_d_n4, eq71_e1841_d_n5, eq71_e1841_d_n6, eq71_e1841_d_n7, eq71_e1841_d_n8, eq71_e1841_d_n9, eq71_e1841_d_n10, eq71_e1841_d_n11, eq71_e1841_d_n12, eq71_e1841_d_n13, eq71_e1841_d_n14, 0.0, 0.0];
        let eq71_reactive_branch_derivatives: [f64; 14] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[12]),
            Some(nodes[7]),
            nodes,
            &eq71_reactive_node_derivatives,
            branches,
            &eq71_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq73_e1868, eq73_e1868_d_n0, eq73_e1868_d_n2, eq73_e1868_d_n3, eq73_e1868_d_n4, eq73_e1868_d_n5, eq73_e1868_d_n6, eq73_e1868_d_n7, eq73_e1868_d_n8, eq73_e1868_d_n9, eq73_e1868_d_n10, eq73_e1868_d_n11, eq73_e1868_d_n12, eq73_e1868_d_n13, eq73_e1868_d_n14, eq73_e1868_q,) = {
    if ((locals.var_guard769 != 0.0) && (locals.var_guard770 != 0.0)) {
        let eq73_e1864: f64 = (p.p29 * locals.var_qbdj);
        let eq73_e1864_d_n0: f64 = (p.p29 * locals.var_qbdj_dn0);
        let eq73_e1864_d_n2: f64 = (p.p29 * locals.var_qbdj_dn2);
        let eq73_e1864_d_n3: f64 = (p.p29 * locals.var_qbdj_dn3);
        let eq73_e1864_d_n4: f64 = (p.p29 * locals.var_qbdj_dn4);
        let eq73_e1864_d_n5: f64 = (p.p29 * locals.var_qbdj_dn5);
        let eq73_e1864_d_n6: f64 = (p.p29 * locals.var_qbdj_dn6);
        let eq73_e1864_d_n7: f64 = (p.p29 * locals.var_qbdj_dn7);
        let eq73_e1864_d_n8: f64 = (p.p29 * locals.var_qbdj_dn8);
        let eq73_e1864_d_n9: f64 = (p.p29 * locals.var_qbdj_dn9);
        let eq73_e1864_d_n10: f64 = (p.p29 * locals.var_qbdj_dn10);
        let eq73_e1864_d_n11: f64 = (p.p29 * locals.var_qbdj_dn11);
        let eq73_e1864_d_n12: f64 = (p.p29 * locals.var_qbdj_dn12);
        let eq73_e1864_d_n13: f64 = (p.p29 * locals.var_qbdj_dn13);
        let eq73_e1864_d_n14: f64 = (p.p29 * locals.var_qbdj_dn14);
        let eq73_e1865_q: f64 = eq73_e1864;
        let eq73_e1866: f64 = (locals.var_devsign * eq73_e1864);
        let eq73_e1866_d_n0: f64 = (locals.var_devsign * eq73_e1864_d_n0);
        let eq73_e1866_d_n2: f64 = (locals.var_devsign * eq73_e1864_d_n2);
        let eq73_e1866_d_n3: f64 = (locals.var_devsign * eq73_e1864_d_n3);
        let eq73_e1866_d_n4: f64 = (locals.var_devsign * eq73_e1864_d_n4);
        let eq73_e1866_d_n5: f64 = (locals.var_devsign * eq73_e1864_d_n5);
        let eq73_e1866_d_n6: f64 = (locals.var_devsign * eq73_e1864_d_n6);
        let eq73_e1866_d_n7: f64 = (locals.var_devsign * eq73_e1864_d_n7);
        let eq73_e1866_d_n8: f64 = (locals.var_devsign * eq73_e1864_d_n8);
        let eq73_e1866_d_n9: f64 = (locals.var_devsign * eq73_e1864_d_n9);
        let eq73_e1866_d_n10: f64 = (locals.var_devsign * eq73_e1864_d_n10);
        let eq73_e1866_d_n11: f64 = (locals.var_devsign * eq73_e1864_d_n11);
        let eq73_e1866_d_n12: f64 = (locals.var_devsign * eq73_e1864_d_n12);
        let eq73_e1866_d_n13: f64 = (locals.var_devsign * eq73_e1864_d_n13);
        let eq73_e1866_d_n14: f64 = (locals.var_devsign * eq73_e1864_d_n14);
        let eq73_e1866_q: f64 = (locals.var_devsign * eq73_e1865_q);
        (eq73_e1866, eq73_e1866_d_n0, eq73_e1866_d_n2, eq73_e1866_d_n3, eq73_e1866_d_n4, eq73_e1866_d_n5, eq73_e1866_d_n6, eq73_e1866_d_n7, eq73_e1866_d_n8, eq73_e1866_d_n9, eq73_e1866_d_n10, eq73_e1866_d_n11, eq73_e1866_d_n12, eq73_e1866_d_n13, eq73_e1866_d_n14, eq73_e1866_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq73_reactive_node_derivatives: [f64; 17] = [eq73_e1868_d_n0, 0.0, eq73_e1868_d_n2, eq73_e1868_d_n3, eq73_e1868_d_n4, eq73_e1868_d_n5, eq73_e1868_d_n6, eq73_e1868_d_n7, eq73_e1868_d_n8, eq73_e1868_d_n9, eq73_e1868_d_n10, eq73_e1868_d_n11, eq73_e1868_d_n12, eq73_e1868_d_n13, eq73_e1868_d_n14, 0.0, 0.0];
        let eq73_reactive_branch_derivatives: [f64; 14] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[13]),
            Some(nodes[5]),
            nodes,
            &eq73_reactive_node_derivatives,
            branches,
            &eq73_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq76_e1908, eq76_e1908_d_n0, eq76_e1908_d_n2, eq76_e1908_d_n3, eq76_e1908_d_n4, eq76_e1908_d_n5, eq76_e1908_d_n6, eq76_e1908_d_n7, eq76_e1908_d_n8, eq76_e1908_d_n9, eq76_e1908_d_n10, eq76_e1908_d_n11, eq76_e1908_d_n12, eq76_e1908_d_n13, eq76_e1908_d_n14, eq76_e1908_q,) = {
    if (locals.var_guard769 == 0.0) {
        let eq76_e1904: f64 = (p.p29 * locals.var_qbsj);
        let eq76_e1904_d_n0: f64 = (p.p29 * locals.var_qbsj_dn0);
        let eq76_e1904_d_n2: f64 = (p.p29 * locals.var_qbsj_dn2);
        let eq76_e1904_d_n3: f64 = (p.p29 * locals.var_qbsj_dn3);
        let eq76_e1904_d_n4: f64 = (p.p29 * locals.var_qbsj_dn4);
        let eq76_e1904_d_n5: f64 = (p.p29 * locals.var_qbsj_dn5);
        let eq76_e1904_d_n6: f64 = (p.p29 * locals.var_qbsj_dn6);
        let eq76_e1904_d_n7: f64 = (p.p29 * locals.var_qbsj_dn7);
        let eq76_e1904_d_n8: f64 = (p.p29 * locals.var_qbsj_dn8);
        let eq76_e1904_d_n9: f64 = (p.p29 * locals.var_qbsj_dn9);
        let eq76_e1904_d_n10: f64 = (p.p29 * locals.var_qbsj_dn10);
        let eq76_e1904_d_n11: f64 = (p.p29 * locals.var_qbsj_dn11);
        let eq76_e1904_d_n12: f64 = (p.p29 * locals.var_qbsj_dn12);
        let eq76_e1904_d_n13: f64 = (p.p29 * locals.var_qbsj_dn13);
        let eq76_e1904_d_n14: f64 = (p.p29 * locals.var_qbsj_dn14);
        let eq76_e1905_q: f64 = eq76_e1904;
        let eq76_e1906: f64 = (locals.var_devsign * eq76_e1904);
        let eq76_e1906_d_n0: f64 = (locals.var_devsign * eq76_e1904_d_n0);
        let eq76_e1906_d_n2: f64 = (locals.var_devsign * eq76_e1904_d_n2);
        let eq76_e1906_d_n3: f64 = (locals.var_devsign * eq76_e1904_d_n3);
        let eq76_e1906_d_n4: f64 = (locals.var_devsign * eq76_e1904_d_n4);
        let eq76_e1906_d_n5: f64 = (locals.var_devsign * eq76_e1904_d_n5);
        let eq76_e1906_d_n6: f64 = (locals.var_devsign * eq76_e1904_d_n6);
        let eq76_e1906_d_n7: f64 = (locals.var_devsign * eq76_e1904_d_n7);
        let eq76_e1906_d_n8: f64 = (locals.var_devsign * eq76_e1904_d_n8);
        let eq76_e1906_d_n9: f64 = (locals.var_devsign * eq76_e1904_d_n9);
        let eq76_e1906_d_n10: f64 = (locals.var_devsign * eq76_e1904_d_n10);
        let eq76_e1906_d_n11: f64 = (locals.var_devsign * eq76_e1904_d_n11);
        let eq76_e1906_d_n12: f64 = (locals.var_devsign * eq76_e1904_d_n12);
        let eq76_e1906_d_n13: f64 = (locals.var_devsign * eq76_e1904_d_n13);
        let eq76_e1906_d_n14: f64 = (locals.var_devsign * eq76_e1904_d_n14);
        let eq76_e1906_q: f64 = (locals.var_devsign * eq76_e1905_q);
        (eq76_e1906, eq76_e1906_d_n0, eq76_e1906_d_n2, eq76_e1906_d_n3, eq76_e1906_d_n4, eq76_e1906_d_n5, eq76_e1906_d_n6, eq76_e1906_d_n7, eq76_e1906_d_n8, eq76_e1906_d_n9, eq76_e1906_d_n10, eq76_e1906_d_n11, eq76_e1906_d_n12, eq76_e1906_d_n13, eq76_e1906_d_n14, eq76_e1906_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq76_reactive_node_derivatives: [f64; 17] = [eq76_e1908_d_n0, 0.0, eq76_e1908_d_n2, eq76_e1908_d_n3, eq76_e1908_d_n4, eq76_e1908_d_n5, eq76_e1908_d_n6, eq76_e1908_d_n7, eq76_e1908_d_n8, eq76_e1908_d_n9, eq76_e1908_d_n10, eq76_e1908_d_n11, eq76_e1908_d_n12, eq76_e1908_d_n13, eq76_e1908_d_n14, 0.0, 0.0];
        let eq76_reactive_branch_derivatives: [f64; 14] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[7]),
            nodes,
            &eq76_reactive_node_derivatives,
            branches,
            &eq76_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq77_e1918, eq77_e1918_d_n0, eq77_e1918_d_n2, eq77_e1918_d_n3, eq77_e1918_d_n4, eq77_e1918_d_n5, eq77_e1918_d_n6, eq77_e1918_d_n7, eq77_e1918_d_n8, eq77_e1918_d_n9, eq77_e1918_d_n10, eq77_e1918_d_n11, eq77_e1918_d_n12, eq77_e1918_d_n13, eq77_e1918_d_n14, eq77_e1918_q,) = {
    if (locals.var_guard769 == 0.0) {
        let eq77_e1914: f64 = (p.p29 * locals.var_qbdj);
        let eq77_e1914_d_n0: f64 = (p.p29 * locals.var_qbdj_dn0);
        let eq77_e1914_d_n2: f64 = (p.p29 * locals.var_qbdj_dn2);
        let eq77_e1914_d_n3: f64 = (p.p29 * locals.var_qbdj_dn3);
        let eq77_e1914_d_n4: f64 = (p.p29 * locals.var_qbdj_dn4);
        let eq77_e1914_d_n5: f64 = (p.p29 * locals.var_qbdj_dn5);
        let eq77_e1914_d_n6: f64 = (p.p29 * locals.var_qbdj_dn6);
        let eq77_e1914_d_n7: f64 = (p.p29 * locals.var_qbdj_dn7);
        let eq77_e1914_d_n8: f64 = (p.p29 * locals.var_qbdj_dn8);
        let eq77_e1914_d_n9: f64 = (p.p29 * locals.var_qbdj_dn9);
        let eq77_e1914_d_n10: f64 = (p.p29 * locals.var_qbdj_dn10);
        let eq77_e1914_d_n11: f64 = (p.p29 * locals.var_qbdj_dn11);
        let eq77_e1914_d_n12: f64 = (p.p29 * locals.var_qbdj_dn12);
        let eq77_e1914_d_n13: f64 = (p.p29 * locals.var_qbdj_dn13);
        let eq77_e1914_d_n14: f64 = (p.p29 * locals.var_qbdj_dn14);
        let eq77_e1915_q: f64 = eq77_e1914;
        let eq77_e1916: f64 = (locals.var_devsign * eq77_e1914);
        let eq77_e1916_d_n0: f64 = (locals.var_devsign * eq77_e1914_d_n0);
        let eq77_e1916_d_n2: f64 = (locals.var_devsign * eq77_e1914_d_n2);
        let eq77_e1916_d_n3: f64 = (locals.var_devsign * eq77_e1914_d_n3);
        let eq77_e1916_d_n4: f64 = (locals.var_devsign * eq77_e1914_d_n4);
        let eq77_e1916_d_n5: f64 = (locals.var_devsign * eq77_e1914_d_n5);
        let eq77_e1916_d_n6: f64 = (locals.var_devsign * eq77_e1914_d_n6);
        let eq77_e1916_d_n7: f64 = (locals.var_devsign * eq77_e1914_d_n7);
        let eq77_e1916_d_n8: f64 = (locals.var_devsign * eq77_e1914_d_n8);
        let eq77_e1916_d_n9: f64 = (locals.var_devsign * eq77_e1914_d_n9);
        let eq77_e1916_d_n10: f64 = (locals.var_devsign * eq77_e1914_d_n10);
        let eq77_e1916_d_n11: f64 = (locals.var_devsign * eq77_e1914_d_n11);
        let eq77_e1916_d_n12: f64 = (locals.var_devsign * eq77_e1914_d_n12);
        let eq77_e1916_d_n13: f64 = (locals.var_devsign * eq77_e1914_d_n13);
        let eq77_e1916_d_n14: f64 = (locals.var_devsign * eq77_e1914_d_n14);
        let eq77_e1916_q: f64 = (locals.var_devsign * eq77_e1915_q);
        (eq77_e1916, eq77_e1916_d_n0, eq77_e1916_d_n2, eq77_e1916_d_n3, eq77_e1916_d_n4, eq77_e1916_d_n5, eq77_e1916_d_n6, eq77_e1916_d_n7, eq77_e1916_d_n8, eq77_e1916_d_n9, eq77_e1916_d_n10, eq77_e1916_d_n11, eq77_e1916_d_n12, eq77_e1916_d_n13, eq77_e1916_d_n14, eq77_e1916_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq77_reactive_node_derivatives: [f64; 17] = [eq77_e1918_d_n0, 0.0, eq77_e1918_d_n2, eq77_e1918_d_n3, eq77_e1918_d_n4, eq77_e1918_d_n5, eq77_e1918_d_n6, eq77_e1918_d_n7, eq77_e1918_d_n8, eq77_e1918_d_n9, eq77_e1918_d_n10, eq77_e1918_d_n11, eq77_e1918_d_n12, eq77_e1918_d_n13, eq77_e1918_d_n14, 0.0, 0.0];
        let eq77_reactive_branch_derivatives: [f64; 14] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[5]),
            nodes,
            &eq77_reactive_node_derivatives,
            branches,
            &eq77_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq83_e1984, eq83_e1984_d_n0, eq83_e1984_d_n2, eq83_e1984_d_n3, eq83_e1984_d_n4, eq83_e1984_d_n5, eq83_e1984_d_n6, eq83_e1984_d_n7, eq83_e1984_d_n8, eq83_e1984_d_n9, eq83_e1984_d_n10, eq83_e1984_d_n11, eq83_e1984_d_n12, eq83_e1984_d_n13, eq83_e1984_d_n14, eq83_e1984_q,) = {
    if (locals.var_guard772 != 0.0) {
        let eq83_e1980: f64 = (p.p29 * locals.var_qbdj);
        let eq83_e1980_d_n0: f64 = (p.p29 * locals.var_qbdj_dn0);
        let eq83_e1980_d_n2: f64 = (p.p29 * locals.var_qbdj_dn2);
        let eq83_e1980_d_n3: f64 = (p.p29 * locals.var_qbdj_dn3);
        let eq83_e1980_d_n4: f64 = (p.p29 * locals.var_qbdj_dn4);
        let eq83_e1980_d_n5: f64 = (p.p29 * locals.var_qbdj_dn5);
        let eq83_e1980_d_n6: f64 = (p.p29 * locals.var_qbdj_dn6);
        let eq83_e1980_d_n7: f64 = (p.p29 * locals.var_qbdj_dn7);
        let eq83_e1980_d_n8: f64 = (p.p29 * locals.var_qbdj_dn8);
        let eq83_e1980_d_n9: f64 = (p.p29 * locals.var_qbdj_dn9);
        let eq83_e1980_d_n10: f64 = (p.p29 * locals.var_qbdj_dn10);
        let eq83_e1980_d_n11: f64 = (p.p29 * locals.var_qbdj_dn11);
        let eq83_e1980_d_n12: f64 = (p.p29 * locals.var_qbdj_dn12);
        let eq83_e1980_d_n13: f64 = (p.p29 * locals.var_qbdj_dn13);
        let eq83_e1980_d_n14: f64 = (p.p29 * locals.var_qbdj_dn14);
        let eq83_e1981_q: f64 = eq83_e1980;
        let eq83_e1982: f64 = (locals.var_devsign * eq83_e1980);
        let eq83_e1982_d_n0: f64 = (locals.var_devsign * eq83_e1980_d_n0);
        let eq83_e1982_d_n2: f64 = (locals.var_devsign * eq83_e1980_d_n2);
        let eq83_e1982_d_n3: f64 = (locals.var_devsign * eq83_e1980_d_n3);
        let eq83_e1982_d_n4: f64 = (locals.var_devsign * eq83_e1980_d_n4);
        let eq83_e1982_d_n5: f64 = (locals.var_devsign * eq83_e1980_d_n5);
        let eq83_e1982_d_n6: f64 = (locals.var_devsign * eq83_e1980_d_n6);
        let eq83_e1982_d_n7: f64 = (locals.var_devsign * eq83_e1980_d_n7);
        let eq83_e1982_d_n8: f64 = (locals.var_devsign * eq83_e1980_d_n8);
        let eq83_e1982_d_n9: f64 = (locals.var_devsign * eq83_e1980_d_n9);
        let eq83_e1982_d_n10: f64 = (locals.var_devsign * eq83_e1980_d_n10);
        let eq83_e1982_d_n11: f64 = (locals.var_devsign * eq83_e1980_d_n11);
        let eq83_e1982_d_n12: f64 = (locals.var_devsign * eq83_e1980_d_n12);
        let eq83_e1982_d_n13: f64 = (locals.var_devsign * eq83_e1980_d_n13);
        let eq83_e1982_d_n14: f64 = (locals.var_devsign * eq83_e1980_d_n14);
        let eq83_e1982_q: f64 = (locals.var_devsign * eq83_e1981_q);
        (eq83_e1982, eq83_e1982_d_n0, eq83_e1982_d_n2, eq83_e1982_d_n3, eq83_e1982_d_n4, eq83_e1982_d_n5, eq83_e1982_d_n6, eq83_e1982_d_n7, eq83_e1982_d_n8, eq83_e1982_d_n9, eq83_e1982_d_n10, eq83_e1982_d_n11, eq83_e1982_d_n12, eq83_e1982_d_n13, eq83_e1982_d_n14, eq83_e1982_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq83_reactive_node_derivatives: [f64; 17] = [eq83_e1984_d_n0, 0.0, eq83_e1984_d_n2, eq83_e1984_d_n3, eq83_e1984_d_n4, eq83_e1984_d_n5, eq83_e1984_d_n6, eq83_e1984_d_n7, eq83_e1984_d_n8, eq83_e1984_d_n9, eq83_e1984_d_n10, eq83_e1984_d_n11, eq83_e1984_d_n12, eq83_e1984_d_n13, eq83_e1984_d_n14, 0.0, 0.0];
        let eq83_reactive_branch_derivatives: [f64; 14] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[13]),
            Some(nodes[5]),
            nodes,
            &eq83_reactive_node_derivatives,
            branches,
            &eq83_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq84_e1993, eq84_e1993_d_n0, eq84_e1993_d_n2, eq84_e1993_d_n3, eq84_e1993_d_n4, eq84_e1993_d_n5, eq84_e1993_d_n6, eq84_e1993_d_n7, eq84_e1993_d_n8, eq84_e1993_d_n9, eq84_e1993_d_n10, eq84_e1993_d_n11, eq84_e1993_d_n12, eq84_e1993_d_n13, eq84_e1993_d_n14, eq84_e1993_q,) = {
    if (locals.var_guard772 != 0.0) {
        let eq84_e1989: f64 = (p.p29 * locals.var_qbdj_ext);
        let eq84_e1989_d_n0: f64 = (p.p29 * locals.var_qbdj_ext_dn0);
        let eq84_e1989_d_n2: f64 = (p.p29 * locals.var_qbdj_ext_dn2);
        let eq84_e1989_d_n3: f64 = (p.p29 * locals.var_qbdj_ext_dn3);
        let eq84_e1989_d_n4: f64 = (p.p29 * locals.var_qbdj_ext_dn4);
        let eq84_e1989_d_n5: f64 = (p.p29 * locals.var_qbdj_ext_dn5);
        let eq84_e1989_d_n6: f64 = (p.p29 * locals.var_qbdj_ext_dn6);
        let eq84_e1989_d_n7: f64 = (p.p29 * locals.var_qbdj_ext_dn7);
        let eq84_e1989_d_n8: f64 = (p.p29 * locals.var_qbdj_ext_dn8);
        let eq84_e1989_d_n9: f64 = (p.p29 * locals.var_qbdj_ext_dn9);
        let eq84_e1989_d_n10: f64 = (p.p29 * locals.var_qbdj_ext_dn10);
        let eq84_e1989_d_n11: f64 = (p.p29 * locals.var_qbdj_ext_dn11);
        let eq84_e1989_d_n12: f64 = (p.p29 * locals.var_qbdj_ext_dn12);
        let eq84_e1989_d_n13: f64 = (p.p29 * locals.var_qbdj_ext_dn13);
        let eq84_e1989_d_n14: f64 = (p.p29 * locals.var_qbdj_ext_dn14);
        let eq84_e1990_q: f64 = eq84_e1989;
        let eq84_e1991: f64 = (locals.var_devsign * eq84_e1989);
        let eq84_e1991_d_n0: f64 = (locals.var_devsign * eq84_e1989_d_n0);
        let eq84_e1991_d_n2: f64 = (locals.var_devsign * eq84_e1989_d_n2);
        let eq84_e1991_d_n3: f64 = (locals.var_devsign * eq84_e1989_d_n3);
        let eq84_e1991_d_n4: f64 = (locals.var_devsign * eq84_e1989_d_n4);
        let eq84_e1991_d_n5: f64 = (locals.var_devsign * eq84_e1989_d_n5);
        let eq84_e1991_d_n6: f64 = (locals.var_devsign * eq84_e1989_d_n6);
        let eq84_e1991_d_n7: f64 = (locals.var_devsign * eq84_e1989_d_n7);
        let eq84_e1991_d_n8: f64 = (locals.var_devsign * eq84_e1989_d_n8);
        let eq84_e1991_d_n9: f64 = (locals.var_devsign * eq84_e1989_d_n9);
        let eq84_e1991_d_n10: f64 = (locals.var_devsign * eq84_e1989_d_n10);
        let eq84_e1991_d_n11: f64 = (locals.var_devsign * eq84_e1989_d_n11);
        let eq84_e1991_d_n12: f64 = (locals.var_devsign * eq84_e1989_d_n12);
        let eq84_e1991_d_n13: f64 = (locals.var_devsign * eq84_e1989_d_n13);
        let eq84_e1991_d_n14: f64 = (locals.var_devsign * eq84_e1989_d_n14);
        let eq84_e1991_q: f64 = (locals.var_devsign * eq84_e1990_q);
        (eq84_e1991, eq84_e1991_d_n0, eq84_e1991_d_n2, eq84_e1991_d_n3, eq84_e1991_d_n4, eq84_e1991_d_n5, eq84_e1991_d_n6, eq84_e1991_d_n7, eq84_e1991_d_n8, eq84_e1991_d_n9, eq84_e1991_d_n10, eq84_e1991_d_n11, eq84_e1991_d_n12, eq84_e1991_d_n13, eq84_e1991_d_n14, eq84_e1991_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq84_reactive_node_derivatives: [f64; 17] = [eq84_e1993_d_n0, 0.0, eq84_e1993_d_n2, eq84_e1993_d_n3, eq84_e1993_d_n4, eq84_e1993_d_n5, eq84_e1993_d_n6, eq84_e1993_d_n7, eq84_e1993_d_n8, eq84_e1993_d_n9, eq84_e1993_d_n10, eq84_e1993_d_n11, eq84_e1993_d_n12, eq84_e1993_d_n13, eq84_e1993_d_n14, 0.0, 0.0];
        let eq84_reactive_branch_derivatives: [f64; 14] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[13]),
            Some(nodes[14]),
            nodes,
            &eq84_reactive_node_derivatives,
            branches,
            &eq84_reactive_branch_derivatives,
            multiplicity,
        );
    }
}
