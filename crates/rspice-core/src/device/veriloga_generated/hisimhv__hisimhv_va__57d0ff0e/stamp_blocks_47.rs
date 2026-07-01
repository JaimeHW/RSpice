#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_363(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign94230_e144378, assign94230_e144378_d_n0, assign94230_e144378_d_n2, assign94230_e144378_d_n4, assign94230_e144378_d_n5, assign94230_e144378_d_n6, assign94230_e144378_d_n7, assign94230_e144378_d_n8, assign94230_e144378_d_n9, assign94230_e144378_d_n10, assign94230_e144378_d_n11, assign94230_e144378_d_n14,) = {
    if (((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2182 != 0.0)) && (locals.var_guard2183 == 0.0)) {
        let assign94230_e144373: f64 = (locals.var_kjunc * locals.var_t2);
        let assign94230_e144374: f64 = (assign94230_e144373).sqrt();
        let assign94230_e144376: f64 = (assign94230_e144374 * p.p432);
        (assign94230_e144376, ((((locals.var_kjunc_dn0 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn0)) / (2.0 * assign94230_e144374)) * p.p432), ((((locals.var_kjunc_dn2 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn2)) / (2.0 * assign94230_e144374)) * p.p432), ((((locals.var_kjunc_dn4 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn4)) / (2.0 * assign94230_e144374)) * p.p432), ((((locals.var_kjunc_dn5 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn5)) / (2.0 * assign94230_e144374)) * p.p432), ((((locals.var_kjunc_dn6 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn6)) / (2.0 * assign94230_e144374)) * p.p432), ((((locals.var_kjunc_dn7 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn7)) / (2.0 * assign94230_e144374)) * p.p432), ((((locals.var_kjunc_dn8 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn8)) / (2.0 * assign94230_e144374)) * p.p432), ((((locals.var_kjunc_dn9 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn9)) / (2.0 * assign94230_e144374)) * p.p432), ((((locals.var_kjunc_dn10 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn10)) / (2.0 * assign94230_e144374)) * p.p432), ((((locals.var_kjunc_dn11 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn11)) / (2.0 * assign94230_e144374)) * p.p432), ((((locals.var_kjunc_dn14 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn14)) / (2.0 * assign94230_e144374)) * p.p432),)
    } else {
        (locals.var_wjunc0, locals.var_wjunc0_dn0, locals.var_wjunc0_dn2, locals.var_wjunc0_dn4, locals.var_wjunc0_dn5, locals.var_wjunc0_dn6, locals.var_wjunc0_dn7, locals.var_wjunc0_dn8, locals.var_wjunc0_dn9, locals.var_wjunc0_dn10, locals.var_wjunc0_dn11, locals.var_wjunc0_dn14,)
    }
};
        locals.var_wjunc0 = assign94230_e144378;
        locals.var_wjunc0_dn0 = assign94230_e144378_d_n0;
        locals.var_wjunc0_dn2 = assign94230_e144378_d_n2;
        locals.var_wjunc0_dn4 = assign94230_e144378_d_n4;
        locals.var_wjunc0_dn5 = assign94230_e144378_d_n5;
        locals.var_wjunc0_dn6 = assign94230_e144378_d_n6;
        locals.var_wjunc0_dn7 = assign94230_e144378_d_n7;
        locals.var_wjunc0_dn8 = assign94230_e144378_d_n8;
        locals.var_wjunc0_dn9 = assign94230_e144378_d_n9;
        locals.var_wjunc0_dn10 = assign94230_e144378_d_n10;
        locals.var_wjunc0_dn11 = assign94230_e144378_d_n11;
        locals.var_wjunc0_dn14 = assign94230_e144378_d_n14;
        locals.var_wjunc0_rv = 0.0;

        let (assign94240_e144394, assign94240_e144394_d_n0, assign94240_e144394_d_n2, assign94240_e144394_d_n4, assign94240_e144394_d_n5, assign94240_e144394_d_n6, assign94240_e144394_d_n7, assign94240_e144394_d_n8, assign94240_e144394_d_n9, assign94240_e144394_d_n10, assign94240_e144394_d_n11, assign94240_e144394_d_n14,) = {
    if (((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2182 != 0.0)) && (locals.var_guard2183 == 0.0)) {
        let assign94240_e144392: f64 = (p.p334 - locals.var_wjunc0);
        (assign94240_e144392, (-locals.var_wjunc0_dn0), (-locals.var_wjunc0_dn2), (-locals.var_wjunc0_dn4), (-locals.var_wjunc0_dn5), (-locals.var_wjunc0_dn6), (-locals.var_wjunc0_dn7), (-locals.var_wjunc0_dn8), (-locals.var_wjunc0_dn9), (-locals.var_wjunc0_dn10), (-locals.var_wjunc0_dn11), (-locals.var_wjunc0_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign94240_e144394;
        locals.var_t2_dn0 = assign94240_e144394_d_n0;
        locals.var_t2_dn2 = assign94240_e144394_d_n2;
        locals.var_t2_dn4 = assign94240_e144394_d_n4;
        locals.var_t2_dn5 = assign94240_e144394_d_n5;
        locals.var_t2_dn6 = assign94240_e144394_d_n6;
        locals.var_t2_dn7 = assign94240_e144394_d_n7;
        locals.var_t2_dn8 = assign94240_e144394_d_n8;
        locals.var_t2_dn9 = assign94240_e144394_d_n9;
        locals.var_t2_dn10 = assign94240_e144394_d_n10;
        locals.var_t2_dn11 = assign94240_e144394_d_n11;
        locals.var_t2_dn14 = assign94240_e144394_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign94250_e144418, assign94250_e144418_d_n0, assign94250_e144418_d_n2, assign94250_e144418_d_n4, assign94250_e144418_d_n5, assign94250_e144418_d_n6, assign94250_e144418_d_n7, assign94250_e144418_d_n8, assign94250_e144418_d_n9, assign94250_e144418_d_n10, assign94250_e144418_d_n11, assign94250_e144418_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2182 != 0.0)) {
        let assign94250_e144405: f64 = (locals.var_t2 * locals.var_t2);
        let assign94250_e144409: f64 = (p.p334 * 0.01);
        let assign94250_e144410: f64 = (4.0 * assign94250_e144409);
        let assign94250_e144413: f64 = (p.p334 * 0.01);
        let assign94250_e144414: f64 = (assign94250_e144410 * assign94250_e144413);
        let assign94250_e144415: f64 = (assign94250_e144405 + assign94250_e144414);
        let assign94250_e144416: f64 = (assign94250_e144415).sqrt();
        (assign94250_e144416, (((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)) / (2.0 * assign94250_e144416)), (((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)) / (2.0 * assign94250_e144416)), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign94250_e144416)), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign94250_e144416)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign94250_e144416)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign94250_e144416)), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign94250_e144416)), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign94250_e144416)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign94250_e144416)), (((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign94250_e144416)), (((locals.var_t2_dn14 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn14)) / (2.0 * assign94250_e144416)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign94250_e144418;
        locals.var_tmf2_dn0 = assign94250_e144418_d_n0;
        locals.var_tmf2_dn2 = assign94250_e144418_d_n2;
        locals.var_tmf2_dn4 = assign94250_e144418_d_n4;
        locals.var_tmf2_dn5 = assign94250_e144418_d_n5;
        locals.var_tmf2_dn6 = assign94250_e144418_d_n6;
        locals.var_tmf2_dn7 = assign94250_e144418_d_n7;
        locals.var_tmf2_dn8 = assign94250_e144418_d_n8;
        locals.var_tmf2_dn9 = assign94250_e144418_d_n9;
        locals.var_tmf2_dn10 = assign94250_e144418_d_n10;
        locals.var_tmf2_dn11 = assign94250_e144418_d_n11;
        locals.var_tmf2_dn14 = assign94250_e144418_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign94260_e144435, assign94260_e144435_d_n0, assign94260_e144435_d_n2, assign94260_e144435_d_n4, assign94260_e144435_d_n5, assign94260_e144435_d_n6, assign94260_e144435_d_n7, assign94260_e144435_d_n8, assign94260_e144435_d_n9, assign94260_e144435_d_n10, assign94260_e144435_d_n11, assign94260_e144435_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2182 != 0.0)) {
        let assign94260_e144431: f64 = (locals.var_t2 / locals.var_tmf2);
        let assign94260_e144432: f64 = (1.0 + assign94260_e144431);
        let assign94260_e144433: f64 = (0.5 * assign94260_e144432);
        (assign94260_e144433, (0.5 * (((locals.var_t2_dn0 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn2 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn4 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn5 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn6 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn7 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn8 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn9 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn10 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn11 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn14 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign94260_e144435;
        locals.var_t9_dn0 = assign94260_e144435_d_n0;
        locals.var_t9_dn2 = assign94260_e144435_d_n2;
        locals.var_t9_dn4 = assign94260_e144435_d_n4;
        locals.var_t9_dn5 = assign94260_e144435_d_n5;
        locals.var_t9_dn6 = assign94260_e144435_d_n6;
        locals.var_t9_dn7 = assign94260_e144435_d_n7;
        locals.var_t9_dn8 = assign94260_e144435_d_n8;
        locals.var_t9_dn9 = assign94260_e144435_d_n9;
        locals.var_t9_dn10 = assign94260_e144435_d_n10;
        locals.var_t9_dn11 = assign94260_e144435_d_n11;
        locals.var_t9_dn14 = assign94260_e144435_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign94270_e144450, assign94270_e144450_d_n0, assign94270_e144450_d_n2, assign94270_e144450_d_n4, assign94270_e144450_d_n5, assign94270_e144450_d_n6, assign94270_e144450_d_n7, assign94270_e144450_d_n8, assign94270_e144450_d_n9, assign94270_e144450_d_n10, assign94270_e144450_d_n11, assign94270_e144450_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2182 != 0.0)) {
        let assign94270_e144447: f64 = (locals.var_t2 + locals.var_tmf2);
        let assign94270_e144448: f64 = (0.5 * assign94270_e144447);
        (assign94270_e144448, (0.5 * (locals.var_t2_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t2_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t2_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t2_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t2_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t2_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t2_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t2_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t2_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t2_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_t2_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign94270_e144450;
        locals.var_t2_dn0 = assign94270_e144450_d_n0;
        locals.var_t2_dn2 = assign94270_e144450_d_n2;
        locals.var_t2_dn4 = assign94270_e144450_d_n4;
        locals.var_t2_dn5 = assign94270_e144450_d_n5;
        locals.var_t2_dn6 = assign94270_e144450_d_n6;
        locals.var_t2_dn7 = assign94270_e144450_d_n7;
        locals.var_t2_dn8 = assign94270_e144450_d_n8;
        locals.var_t2_dn9 = assign94270_e144450_d_n9;
        locals.var_t2_dn10 = assign94270_e144450_d_n10;
        locals.var_t2_dn11 = assign94270_e144450_d_n11;
        locals.var_t2_dn14 = assign94270_e144450_d_n14;
        locals.var_t2_rv = 0.0;

        let assign94280_e144453: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2185 = assign94280_e144453;
        locals.var_guard2185_rv = 0.0;

        let (assign94290_e144466, assign94290_e144466_d_n0, assign94290_e144466_d_n2, assign94290_e144466_d_n4, assign94290_e144466_d_n5, assign94290_e144466_d_n6, assign94290_e144466_d_n7, assign94290_e144466_d_n8, assign94290_e144466_d_n9, assign94290_e144466_d_n10, assign94290_e144466_d_n11, assign94290_e144466_d_n14,) = {
    if (((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2182 != 0.0)) && (locals.var_guard2185 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign94290_e144466;
        locals.var_t2_dn0 = assign94290_e144466_d_n0;
        locals.var_t2_dn2 = assign94290_e144466_d_n2;
        locals.var_t2_dn4 = assign94290_e144466_d_n4;
        locals.var_t2_dn5 = assign94290_e144466_d_n5;
        locals.var_t2_dn6 = assign94290_e144466_d_n6;
        locals.var_t2_dn7 = assign94290_e144466_d_n7;
        locals.var_t2_dn8 = assign94290_e144466_d_n8;
        locals.var_t2_dn9 = assign94290_e144466_d_n9;
        locals.var_t2_dn10 = assign94290_e144466_d_n10;
        locals.var_t2_dn11 = assign94290_e144466_d_n11;
        locals.var_t2_dn14 = assign94290_e144466_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign94300_e144479, assign94300_e144479_d_n0, assign94300_e144479_d_n2, assign94300_e144479_d_n4, assign94300_e144479_d_n5, assign94300_e144479_d_n6, assign94300_e144479_d_n7, assign94300_e144479_d_n8, assign94300_e144479_d_n9, assign94300_e144479_d_n10, assign94300_e144479_d_n11, assign94300_e144479_d_n14,) = {
    if (((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2182 != 0.0)) && (locals.var_guard2185 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign94300_e144479;
        locals.var_t9_dn0 = assign94300_e144479_d_n0;
        locals.var_t9_dn2 = assign94300_e144479_d_n2;
        locals.var_t9_dn4 = assign94300_e144479_d_n4;
        locals.var_t9_dn5 = assign94300_e144479_d_n5;
        locals.var_t9_dn6 = assign94300_e144479_d_n6;
        locals.var_t9_dn7 = assign94300_e144479_d_n7;
        locals.var_t9_dn8 = assign94300_e144479_d_n8;
        locals.var_t9_dn9 = assign94300_e144479_d_n9;
        locals.var_t9_dn10 = assign94300_e144479_d_n10;
        locals.var_t9_dn11 = assign94300_e144479_d_n11;
        locals.var_t9_dn14 = assign94300_e144479_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign94310_e144490, assign94310_e144490_d_n0, assign94310_e144490_d_n2, assign94310_e144490_d_n4, assign94310_e144490_d_n5, assign94310_e144490_d_n6, assign94310_e144490_d_n7, assign94310_e144490_d_n8, assign94310_e144490_d_n9, assign94310_e144490_d_n10, assign94310_e144490_d_n11, assign94310_e144490_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2182 != 0.0)) {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    } else {
        (locals.var_ddriftldc, locals.var_ddriftldc_dn0, locals.var_ddriftldc_dn2, locals.var_ddriftldc_dn4, locals.var_ddriftldc_dn5, locals.var_ddriftldc_dn6, locals.var_ddriftldc_dn7, locals.var_ddriftldc_dn8, locals.var_ddriftldc_dn9, locals.var_ddriftldc_dn10, locals.var_ddriftldc_dn11, locals.var_ddriftldc_dn14,)
    }
};
        locals.var_ddriftldc = assign94310_e144490;
        locals.var_ddriftldc_dn0 = assign94310_e144490_d_n0;
        locals.var_ddriftldc_dn2 = assign94310_e144490_d_n2;
        locals.var_ddriftldc_dn4 = assign94310_e144490_d_n4;
        locals.var_ddriftldc_dn5 = assign94310_e144490_d_n5;
        locals.var_ddriftldc_dn6 = assign94310_e144490_d_n6;
        locals.var_ddriftldc_dn7 = assign94310_e144490_d_n7;
        locals.var_ddriftldc_dn8 = assign94310_e144490_d_n8;
        locals.var_ddriftldc_dn9 = assign94310_e144490_d_n9;
        locals.var_ddriftldc_dn10 = assign94310_e144490_d_n10;
        locals.var_ddriftldc_dn11 = assign94310_e144490_d_n11;
        locals.var_ddriftldc_dn14 = assign94310_e144490_d_n14;
        locals.var_ddriftldc_rv = 0.0;

        let (assign94320_e144509, assign94320_e144509_d_n0, assign94320_e144509_d_n2, assign94320_e144509_d_n4, assign94320_e144509_d_n5, assign94320_e144509_d_n6, assign94320_e144509_d_n7, assign94320_e144509_d_n8, assign94320_e144509_d_n9, assign94320_e144509_d_n10, assign94320_e144509_d_n11, assign94320_e144509_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2182 != 0.0)) {
        let assign94320_e144501: f64 = (locals.var_q_nsubld__blk2119 * locals.var_ddriftldc);
        let assign94320_e144503: f64 = (assign94320_e144501 * locals.var_ddriftldc);
        let assign94320_e144505: f64 = (assign94320_e144503 / 2.0);
        let assign94320_e144507: f64 = (assign94320_e144505 / 1.034943e-10);
        (assign94320_e144507, (((((locals.var_q_nsubld__blk2119 * locals.var_ddriftldc_dn0) * locals.var_ddriftldc) + (assign94320_e144501 * locals.var_ddriftldc_dn0)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2119 * locals.var_ddriftldc_dn2) * locals.var_ddriftldc) + (assign94320_e144501 * locals.var_ddriftldc_dn2)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2119 * locals.var_ddriftldc_dn4) * locals.var_ddriftldc) + (assign94320_e144501 * locals.var_ddriftldc_dn4)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2119 * locals.var_ddriftldc_dn5) * locals.var_ddriftldc) + (assign94320_e144501 * locals.var_ddriftldc_dn5)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2119 * locals.var_ddriftldc_dn6) * locals.var_ddriftldc) + (assign94320_e144501 * locals.var_ddriftldc_dn6)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2119 * locals.var_ddriftldc_dn7) * locals.var_ddriftldc) + (assign94320_e144501 * locals.var_ddriftldc_dn7)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2119 * locals.var_ddriftldc_dn8) * locals.var_ddriftldc) + (assign94320_e144501 * locals.var_ddriftldc_dn8)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2119 * locals.var_ddriftldc_dn9) * locals.var_ddriftldc) + (assign94320_e144501 * locals.var_ddriftldc_dn9)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2119 * locals.var_ddriftldc_dn10) * locals.var_ddriftldc) + (assign94320_e144501 * locals.var_ddriftldc_dn10)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2119 * locals.var_ddriftldc_dn11) * locals.var_ddriftldc) + (assign94320_e144501 * locals.var_ddriftldc_dn11)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2119 * locals.var_ddriftldc_dn14) * locals.var_ddriftldc) + (assign94320_e144501 * locals.var_ddriftldc_dn14)) / 2.0) / 1.034943e-10),)
    } else {
        (locals.var_dphi_sb, locals.var_dphi_sb_dn0, locals.var_dphi_sb_dn2, locals.var_dphi_sb_dn4, locals.var_dphi_sb_dn5, locals.var_dphi_sb_dn6, locals.var_dphi_sb_dn7, locals.var_dphi_sb_dn8, locals.var_dphi_sb_dn9, locals.var_dphi_sb_dn10, locals.var_dphi_sb_dn11, locals.var_dphi_sb_dn14,)
    }
};
        locals.var_dphi_sb = assign94320_e144509;
        locals.var_dphi_sb_dn0 = assign94320_e144509_d_n0;
        locals.var_dphi_sb_dn2 = assign94320_e144509_d_n2;
        locals.var_dphi_sb_dn4 = assign94320_e144509_d_n4;
        locals.var_dphi_sb_dn5 = assign94320_e144509_d_n5;
        locals.var_dphi_sb_dn6 = assign94320_e144509_d_n6;
        locals.var_dphi_sb_dn7 = assign94320_e144509_d_n7;
        locals.var_dphi_sb_dn8 = assign94320_e144509_d_n8;
        locals.var_dphi_sb_dn9 = assign94320_e144509_d_n9;
        locals.var_dphi_sb_dn10 = assign94320_e144509_d_n10;
        locals.var_dphi_sb_dn11 = assign94320_e144509_d_n11;
        locals.var_dphi_sb_dn14 = assign94320_e144509_d_n14;
        locals.var_dphi_sb_rv = 0.0;

        let (assign94330_e144525, assign94330_e144525_d_n0, assign94330_e144525_d_n2, assign94330_e144525_d_n4, assign94330_e144525_d_n5, assign94330_e144525_d_n6, assign94330_e144525_d_n7, assign94330_e144525_d_n8, assign94330_e144525_d_n9, assign94330_e144525_d_n10, assign94330_e144525_d_n11, assign94330_e144525_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2182 != 0.0)) {
        let assign94330_e144520: f64 = (2.0 * locals.var_beta);
        let assign94330_e144522: f64 = (assign94330_e144520 * locals.var_dphi_sb);
        let assign94330_e144523: f64 = (assign94330_e144522).sqrt();
        (assign94330_e144523, ((((2.0 * locals.var_beta_dn0) * locals.var_dphi_sb) + (assign94330_e144520 * locals.var_dphi_sb_dn0)) / (2.0 * assign94330_e144523)), ((((2.0 * locals.var_beta_dn2) * locals.var_dphi_sb) + (assign94330_e144520 * locals.var_dphi_sb_dn2)) / (2.0 * assign94330_e144523)), ((((2.0 * locals.var_beta_dn4) * locals.var_dphi_sb) + (assign94330_e144520 * locals.var_dphi_sb_dn4)) / (2.0 * assign94330_e144523)), ((((2.0 * locals.var_beta_dn5) * locals.var_dphi_sb) + (assign94330_e144520 * locals.var_dphi_sb_dn5)) / (2.0 * assign94330_e144523)), ((((2.0 * locals.var_beta_dn6) * locals.var_dphi_sb) + (assign94330_e144520 * locals.var_dphi_sb_dn6)) / (2.0 * assign94330_e144523)), ((((2.0 * locals.var_beta_dn7) * locals.var_dphi_sb) + (assign94330_e144520 * locals.var_dphi_sb_dn7)) / (2.0 * assign94330_e144523)), ((((2.0 * locals.var_beta_dn8) * locals.var_dphi_sb) + (assign94330_e144520 * locals.var_dphi_sb_dn8)) / (2.0 * assign94330_e144523)), ((((2.0 * locals.var_beta_dn9) * locals.var_dphi_sb) + (assign94330_e144520 * locals.var_dphi_sb_dn9)) / (2.0 * assign94330_e144523)), ((((2.0 * locals.var_beta_dn10) * locals.var_dphi_sb) + (assign94330_e144520 * locals.var_dphi_sb_dn10)) / (2.0 * assign94330_e144523)), ((((2.0 * locals.var_beta_dn11) * locals.var_dphi_sb) + (assign94330_e144520 * locals.var_dphi_sb_dn11)) / (2.0 * assign94330_e144523)), ((((2.0 * locals.var_beta_dn14) * locals.var_dphi_sb) + (assign94330_e144520 * locals.var_dphi_sb_dn14)) / (2.0 * assign94330_e144523)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign94330_e144525;
        locals.var_t0_dn0 = assign94330_e144525_d_n0;
        locals.var_t0_dn2 = assign94330_e144525_d_n2;
        locals.var_t0_dn4 = assign94330_e144525_d_n4;
        locals.var_t0_dn5 = assign94330_e144525_d_n5;
        locals.var_t0_dn6 = assign94330_e144525_d_n6;
        locals.var_t0_dn7 = assign94330_e144525_d_n7;
        locals.var_t0_dn8 = assign94330_e144525_d_n8;
        locals.var_t0_dn9 = assign94330_e144525_d_n9;
        locals.var_t0_dn10 = assign94330_e144525_d_n10;
        locals.var_t0_dn11 = assign94330_e144525_d_n11;
        locals.var_t0_dn14 = assign94330_e144525_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign94340_e144543, assign94340_e144543_d_n0, assign94340_e144543_d_n2, assign94340_e144543_d_n4, assign94340_e144543_d_n5, assign94340_e144543_d_n6, assign94340_e144543_d_n7, assign94340_e144543_d_n8, assign94340_e144543_d_n9, assign94340_e144543_d_n10, assign94340_e144543_d_n11, assign94340_e144543_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2182 != 0.0)) {
        let assign94340_e144535: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign94340_e144537: f64 = (-locals.var_t0);
        let assign94340_e144538: f64 = { let limited_exp_arg = assign94340_e144537; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign94340_e144539: f64 = (assign94340_e144535 + assign94340_e144538);
        let assign94340_e144541: f64 = (assign94340_e144539 / 2.0);
        (assign94340_e144541, ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn0) + ({ let limited_exp_arg = assign94340_e144537; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn0))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn2) + ({ let limited_exp_arg = assign94340_e144537; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn2))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4) + ({ let limited_exp_arg = assign94340_e144537; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn4))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5) + ({ let limited_exp_arg = assign94340_e144537; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn5))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6) + ({ let limited_exp_arg = assign94340_e144537; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn6))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7) + ({ let limited_exp_arg = assign94340_e144537; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn7))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8) + ({ let limited_exp_arg = assign94340_e144537; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn8))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9) + ({ let limited_exp_arg = assign94340_e144537; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn9))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10) + ({ let limited_exp_arg = assign94340_e144537; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn10))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11) + ({ let limited_exp_arg = assign94340_e144537; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn11))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn14) + ({ let limited_exp_arg = assign94340_e144537; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn14))) / 2.0),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign94340_e144543;
        locals.var_t1_dn0 = assign94340_e144543_d_n0;
        locals.var_t1_dn2 = assign94340_e144543_d_n2;
        locals.var_t1_dn4 = assign94340_e144543_d_n4;
        locals.var_t1_dn5 = assign94340_e144543_d_n5;
        locals.var_t1_dn6 = assign94340_e144543_d_n6;
        locals.var_t1_dn7 = assign94340_e144543_d_n7;
        locals.var_t1_dn8 = assign94340_e144543_d_n8;
        locals.var_t1_dn9 = assign94340_e144543_d_n9;
        locals.var_t1_dn10 = assign94340_e144543_d_n10;
        locals.var_t1_dn11 = assign94340_e144543_d_n11;
        locals.var_t1_dn14 = assign94340_e144543_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign94350_e144557, assign94350_e144557_d_n0, assign94350_e144557_d_n2, assign94350_e144557_d_n4, assign94350_e144557_d_n5, assign94350_e144557_d_n6, assign94350_e144557_d_n7, assign94350_e144557_d_n8, assign94350_e144557_d_n9, assign94350_e144557_d_n10, assign94350_e144557_d_n11, assign94350_e144557_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2182 != 0.0)) {
        let assign94350_e144553: f64 = (locals.var_t1).ln();
        let assign94350_e144555: f64 = (assign94350_e144553 / locals.var_dphi_sb);
        (assign94350_e144555, ((((locals.var_t1_dn0 / locals.var_t1) * locals.var_dphi_sb) - (assign94350_e144553 * locals.var_dphi_sb_dn0)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn2 / locals.var_t1) * locals.var_dphi_sb) - (assign94350_e144553 * locals.var_dphi_sb_dn2)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn4 / locals.var_t1) * locals.var_dphi_sb) - (assign94350_e144553 * locals.var_dphi_sb_dn4)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn5 / locals.var_t1) * locals.var_dphi_sb) - (assign94350_e144553 * locals.var_dphi_sb_dn5)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn6 / locals.var_t1) * locals.var_dphi_sb) - (assign94350_e144553 * locals.var_dphi_sb_dn6)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn7 / locals.var_t1) * locals.var_dphi_sb) - (assign94350_e144553 * locals.var_dphi_sb_dn7)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn8 / locals.var_t1) * locals.var_dphi_sb) - (assign94350_e144553 * locals.var_dphi_sb_dn8)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn9 / locals.var_t1) * locals.var_dphi_sb) - (assign94350_e144553 * locals.var_dphi_sb_dn9)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn10 / locals.var_t1) * locals.var_dphi_sb) - (assign94350_e144553 * locals.var_dphi_sb_dn10)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn11 / locals.var_t1) * locals.var_dphi_sb) - (assign94350_e144553 * locals.var_dphi_sb_dn11)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn14 / locals.var_t1) * locals.var_dphi_sb) - (assign94350_e144553 * locals.var_dphi_sb_dn14)) / (locals.var_dphi_sb * locals.var_dphi_sb)),)
    } else {
        (locals.var_c_sb, locals.var_c_sb_dn0, locals.var_c_sb_dn2, locals.var_c_sb_dn4, locals.var_c_sb_dn5, locals.var_c_sb_dn6, locals.var_c_sb_dn7, locals.var_c_sb_dn8, locals.var_c_sb_dn9, locals.var_c_sb_dn10, locals.var_c_sb_dn11, locals.var_c_sb_dn14,)
    }
};
        locals.var_c_sb = assign94350_e144557;
        locals.var_c_sb_dn0 = assign94350_e144557_d_n0;
        locals.var_c_sb_dn2 = assign94350_e144557_d_n2;
        locals.var_c_sb_dn4 = assign94350_e144557_d_n4;
        locals.var_c_sb_dn5 = assign94350_e144557_d_n5;
        locals.var_c_sb_dn6 = assign94350_e144557_d_n6;
        locals.var_c_sb_dn7 = assign94350_e144557_d_n7;
        locals.var_c_sb_dn8 = assign94350_e144557_d_n8;
        locals.var_c_sb_dn9 = assign94350_e144557_d_n9;
        locals.var_c_sb_dn10 = assign94350_e144557_d_n10;
        locals.var_c_sb_dn11 = assign94350_e144557_d_n11;
        locals.var_c_sb_dn14 = assign94350_e144557_d_n14;
        locals.var_c_sb_rv = 0.0;

        let (assign94360_e144568,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2182 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign94360_e144568;
        locals.var_lp_s0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_364(
        locals: &mut StampLocals,
    ) {
        let mut assign94370_loop_guard: usize = 0;
        while {
            let assign94370_cond_e144580: f64 = (locals.var_lp_s0_max + 1.0);
            let assign94370_cond_e144582: f64 = if (((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2182 != 0.0)) && (locals.var_lp_s0 <= assign94370_cond_e144580)) { 1.0 } else { 0.0 };
            assign94370_cond_e144582 != 0.0
        } {
            assign94370_loop_guard += 1;
            assert!(assign94370_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign94370_body3_e144624, assign94370_body3_e144624_d_n0, assign94370_body3_e144624_d_n2, assign94370_body3_e144624_d_n4, assign94370_body3_e144624_d_n5, assign94370_body3_e144624_d_n6, assign94370_body3_e144624_d_n7, assign94370_body3_e144624_d_n8, assign94370_body3_e144624_d_n9, assign94370_body3_e144624_d_n10, assign94370_body3_e144624_d_n11, assign94370_body3_e144624_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2182 != 0.0)) {
        let assign94370_body3_e144622: f64 = (locals.var_ps0ld + locals.var_vxbgmtcl);
        (assign94370_body3_e144622, (locals.var_ps0ld_dn0 + locals.var_vxbgmtcl_dn0), (locals.var_ps0ld_dn2 + locals.var_vxbgmtcl_dn2), (locals.var_ps0ld_dn4 + locals.var_vxbgmtcl_dn4), (locals.var_ps0ld_dn5 + locals.var_vxbgmtcl_dn5), (locals.var_ps0ld_dn6 + locals.var_vxbgmtcl_dn6), (locals.var_ps0ld_dn7 + locals.var_vxbgmtcl_dn7), (locals.var_ps0ld_dn8 + locals.var_vxbgmtcl_dn8), (locals.var_ps0ld_dn9 + locals.var_vxbgmtcl_dn9), (locals.var_ps0ld_dn10 + locals.var_vxbgmtcl_dn10), (locals.var_ps0ld_dn11 + locals.var_vxbgmtcl_dn11), (locals.var_ps0ld_dn14 + locals.var_vxbgmtcl_dn14),)
    } else {
        (locals.var_ps0ld_vxb, locals.var_ps0ld_vxb_dn0, locals.var_ps0ld_vxb_dn2, locals.var_ps0ld_vxb_dn4, locals.var_ps0ld_vxb_dn5, locals.var_ps0ld_vxb_dn6, locals.var_ps0ld_vxb_dn7, locals.var_ps0ld_vxb_dn8, locals.var_ps0ld_vxb_dn9, locals.var_ps0ld_vxb_dn10, locals.var_ps0ld_vxb_dn11, locals.var_ps0ld_vxb_dn14,)
    }
};
            locals.var_ps0ld_vxb = assign94370_body3_e144624;
            locals.var_ps0ld_vxb_dn0 = assign94370_body3_e144624_d_n0;
            locals.var_ps0ld_vxb_dn2 = assign94370_body3_e144624_d_n2;
            locals.var_ps0ld_vxb_dn4 = assign94370_body3_e144624_d_n4;
            locals.var_ps0ld_vxb_dn5 = assign94370_body3_e144624_d_n5;
            locals.var_ps0ld_vxb_dn6 = assign94370_body3_e144624_d_n6;
            locals.var_ps0ld_vxb_dn7 = assign94370_body3_e144624_d_n7;
            locals.var_ps0ld_vxb_dn8 = assign94370_body3_e144624_d_n8;
            locals.var_ps0ld_vxb_dn9 = assign94370_body3_e144624_d_n9;
            locals.var_ps0ld_vxb_dn10 = assign94370_body3_e144624_d_n10;
            locals.var_ps0ld_vxb_dn11 = assign94370_body3_e144624_d_n11;
            locals.var_ps0ld_vxb_dn14 = assign94370_body3_e144624_d_n14;
            locals.var_ps0ld_vxb_rv = 0.0;
            let (assign94370_body4_e144637, assign94370_body4_e144637_d_n0, assign94370_body4_e144637_d_n2, assign94370_body4_e144637_d_n4, assign94370_body4_e144637_d_n5, assign94370_body4_e144637_d_n6, assign94370_body4_e144637_d_n7, assign94370_body4_e144637_d_n8, assign94370_body4_e144637_d_n9, assign94370_body4_e144637_d_n10, assign94370_body4_e144637_d_n11, assign94370_body4_e144637_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2182 != 0.0)) {
        let assign94370_body4_e144635: f64 = (locals.var_beta * locals.var_ps0ld_vxb);
        (assign94370_body4_e144635, ((locals.var_beta_dn0 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn0)), ((locals.var_beta_dn2 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn2)), ((locals.var_beta_dn4 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn4)), ((locals.var_beta_dn5 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn5)), ((locals.var_beta_dn6 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn6)), ((locals.var_beta_dn7 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn7)), ((locals.var_beta_dn8 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn8)), ((locals.var_beta_dn9 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn9)), ((locals.var_beta_dn10 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn10)), ((locals.var_beta_dn11 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn11)), ((locals.var_beta_dn14 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn14)),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
            locals.var_chi = assign94370_body4_e144637;
            locals.var_chi_dn0 = assign94370_body4_e144637_d_n0;
            locals.var_chi_dn2 = assign94370_body4_e144637_d_n2;
            locals.var_chi_dn4 = assign94370_body4_e144637_d_n4;
            locals.var_chi_dn5 = assign94370_body4_e144637_d_n5;
            locals.var_chi_dn6 = assign94370_body4_e144637_d_n6;
            locals.var_chi_dn7 = assign94370_body4_e144637_d_n7;
            locals.var_chi_dn8 = assign94370_body4_e144637_d_n8;
            locals.var_chi_dn9 = assign94370_body4_e144637_d_n9;
            locals.var_chi_dn10 = assign94370_body4_e144637_d_n10;
            locals.var_chi_dn11 = assign94370_body4_e144637_d_n11;
            locals.var_chi_dn14 = assign94370_body4_e144637_d_n14;
            locals.var_chi_rv = 0.0;
            let (assign94370_body5_e144652, assign94370_body5_e144652_d_n0, assign94370_body5_e144652_d_n2, assign94370_body5_e144652_d_n4, assign94370_body5_e144652_d_n5, assign94370_body5_e144652_d_n6, assign94370_body5_e144652_d_n7, assign94370_body5_e144652_d_n8, assign94370_body5_e144652_d_n9, assign94370_body5_e144652_d_n10, assign94370_body5_e144652_d_n11, assign94370_body5_e144652_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2182 != 0.0)) {
        let assign94370_body5_e144649: f64 = (locals.var_ps0ld_vxb - locals.var_dphi_sb);
        let assign94370_body5_e144650: f64 = (locals.var_c_sb * assign94370_body5_e144649);
        (assign94370_body5_e144650, ((locals.var_c_sb_dn0 * assign94370_body5_e144649) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn0 - locals.var_dphi_sb_dn0))), ((locals.var_c_sb_dn2 * assign94370_body5_e144649) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn2 - locals.var_dphi_sb_dn2))), ((locals.var_c_sb_dn4 * assign94370_body5_e144649) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn4 - locals.var_dphi_sb_dn4))), ((locals.var_c_sb_dn5 * assign94370_body5_e144649) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn5 - locals.var_dphi_sb_dn5))), ((locals.var_c_sb_dn6 * assign94370_body5_e144649) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn6 - locals.var_dphi_sb_dn6))), ((locals.var_c_sb_dn7 * assign94370_body5_e144649) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn7 - locals.var_dphi_sb_dn7))), ((locals.var_c_sb_dn8 * assign94370_body5_e144649) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn8 - locals.var_dphi_sb_dn8))), ((locals.var_c_sb_dn9 * assign94370_body5_e144649) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn9 - locals.var_dphi_sb_dn9))), ((locals.var_c_sb_dn10 * assign94370_body5_e144649) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn10 - locals.var_dphi_sb_dn10))), ((locals.var_c_sb_dn11 * assign94370_body5_e144649) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn11 - locals.var_dphi_sb_dn11))), ((locals.var_c_sb_dn14 * assign94370_body5_e144649) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn14 - locals.var_dphi_sb_dn14))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn14,)
    }
};
            locals.var_ty = assign94370_body5_e144652;
            locals.var_ty_dn0 = assign94370_body5_e144652_d_n0;
            locals.var_ty_dn2 = assign94370_body5_e144652_d_n2;
            locals.var_ty_dn4 = assign94370_body5_e144652_d_n4;
            locals.var_ty_dn5 = assign94370_body5_e144652_d_n5;
            locals.var_ty_dn6 = assign94370_body5_e144652_d_n6;
            locals.var_ty_dn7 = assign94370_body5_e144652_d_n7;
            locals.var_ty_dn8 = assign94370_body5_e144652_d_n8;
            locals.var_ty_dn9 = assign94370_body5_e144652_d_n9;
            locals.var_ty_dn10 = assign94370_body5_e144652_d_n10;
            locals.var_ty_dn11 = assign94370_body5_e144652_d_n11;
            locals.var_ty_dn14 = assign94370_body5_e144652_d_n14;
            locals.var_ty_rv = 0.0;
            let assign94370_body6_e144655: f64 = if locals.var_ty < 60.0 { 1.0 } else { 0.0 };
            locals.var_guard2187 = assign94370_body6_e144655;
            locals.var_guard2187_rv = 0.0;
            let (assign94370_body7_e144669, assign94370_body7_e144669_d_n0, assign94370_body7_e144669_d_n2, assign94370_body7_e144669_d_n4, assign94370_body7_e144669_d_n5, assign94370_body7_e144669_d_n6, assign94370_body7_e144669_d_n7, assign94370_body7_e144669_d_n8, assign94370_body7_e144669_d_n9, assign94370_body7_e144669_d_n10, assign94370_body7_e144669_d_n11, assign94370_body7_e144669_d_n14,) = {
    if (((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2182 != 0.0)) && (locals.var_guard2187 != 0.0)) {
        let assign94370_body7_e144667: f64 = (locals.var_ty).exp();
        (assign94370_body7_e144667, (assign94370_body7_e144667 * locals.var_ty_dn0), (assign94370_body7_e144667 * locals.var_ty_dn2), (assign94370_body7_e144667 * locals.var_ty_dn4), (assign94370_body7_e144667 * locals.var_ty_dn5), (assign94370_body7_e144667 * locals.var_ty_dn6), (assign94370_body7_e144667 * locals.var_ty_dn7), (assign94370_body7_e144667 * locals.var_ty_dn8), (assign94370_body7_e144667 * locals.var_ty_dn9), (assign94370_body7_e144667 * locals.var_ty_dn10), (assign94370_body7_e144667 * locals.var_ty_dn11), (assign94370_body7_e144667 * locals.var_ty_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
            locals.var_t1 = assign94370_body7_e144669;
            locals.var_t1_dn0 = assign94370_body7_e144669_d_n0;
            locals.var_t1_dn2 = assign94370_body7_e144669_d_n2;
            locals.var_t1_dn4 = assign94370_body7_e144669_d_n4;
            locals.var_t1_dn5 = assign94370_body7_e144669_d_n5;
            locals.var_t1_dn6 = assign94370_body7_e144669_d_n6;
            locals.var_t1_dn7 = assign94370_body7_e144669_d_n7;
            locals.var_t1_dn8 = assign94370_body7_e144669_d_n8;
            locals.var_t1_dn9 = assign94370_body7_e144669_d_n9;
            locals.var_t1_dn10 = assign94370_body7_e144669_d_n10;
            locals.var_t1_dn11 = assign94370_body7_e144669_d_n11;
            locals.var_t1_dn14 = assign94370_body7_e144669_d_n14;
            locals.var_t1_rv = 0.0;
            let (assign94370_body8_e144686, assign94370_body8_e144686_d_n0, assign94370_body8_e144686_d_n2, assign94370_body8_e144686_d_n4, assign94370_body8_e144686_d_n5, assign94370_body8_e144686_d_n6, assign94370_body8_e144686_d_n7, assign94370_body8_e144686_d_n8, assign94370_body8_e144686_d_n9, assign94370_body8_e144686_d_n10, assign94370_body8_e144686_d_n11, assign94370_body8_e144686_d_n14,) = {
    if (((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2182 != 0.0)) && (locals.var_guard2187 != 0.0)) {
        let assign94370_body8_e144681: f64 = (-locals.var_c_sb);
        let assign94370_body8_e144683: f64 = (assign94370_body8_e144681 * locals.var_dphi_sb);
        let assign94370_body8_e144684: f64 = (assign94370_body8_e144683).exp();
        (assign94370_body8_e144684, (assign94370_body8_e144684 * (((-locals.var_c_sb_dn0) * locals.var_dphi_sb) + (assign94370_body8_e144681 * locals.var_dphi_sb_dn0))), (assign94370_body8_e144684 * (((-locals.var_c_sb_dn2) * locals.var_dphi_sb) + (assign94370_body8_e144681 * locals.var_dphi_sb_dn2))), (assign94370_body8_e144684 * (((-locals.var_c_sb_dn4) * locals.var_dphi_sb) + (assign94370_body8_e144681 * locals.var_dphi_sb_dn4))), (assign94370_body8_e144684 * (((-locals.var_c_sb_dn5) * locals.var_dphi_sb) + (assign94370_body8_e144681 * locals.var_dphi_sb_dn5))), (assign94370_body8_e144684 * (((-locals.var_c_sb_dn6) * locals.var_dphi_sb) + (assign94370_body8_e144681 * locals.var_dphi_sb_dn6))), (assign94370_body8_e144684 * (((-locals.var_c_sb_dn7) * locals.var_dphi_sb) + (assign94370_body8_e144681 * locals.var_dphi_sb_dn7))), (assign94370_body8_e144684 * (((-locals.var_c_sb_dn8) * locals.var_dphi_sb) + (assign94370_body8_e144681 * locals.var_dphi_sb_dn8))), (assign94370_body8_e144684 * (((-locals.var_c_sb_dn9) * locals.var_dphi_sb) + (assign94370_body8_e144681 * locals.var_dphi_sb_dn9))), (assign94370_body8_e144684 * (((-locals.var_c_sb_dn10) * locals.var_dphi_sb) + (assign94370_body8_e144681 * locals.var_dphi_sb_dn10))), (assign94370_body8_e144684 * (((-locals.var_c_sb_dn11) * locals.var_dphi_sb) + (assign94370_body8_e144681 * locals.var_dphi_sb_dn11))), (assign94370_body8_e144684 * (((-locals.var_c_sb_dn14) * locals.var_dphi_sb) + (assign94370_body8_e144681 * locals.var_dphi_sb_dn14))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
            locals.var_t0 = assign94370_body8_e144686;
            locals.var_t0_dn0 = assign94370_body8_e144686_d_n0;
            locals.var_t0_dn2 = assign94370_body8_e144686_d_n2;
            locals.var_t0_dn4 = assign94370_body8_e144686_d_n4;
            locals.var_t0_dn5 = assign94370_body8_e144686_d_n5;
            locals.var_t0_dn6 = assign94370_body8_e144686_d_n6;
            locals.var_t0_dn7 = assign94370_body8_e144686_d_n7;
            locals.var_t0_dn8 = assign94370_body8_e144686_d_n8;
            locals.var_t0_dn9 = assign94370_body8_e144686_d_n9;
            locals.var_t0_dn10 = assign94370_body8_e144686_d_n10;
            locals.var_t0_dn11 = assign94370_body8_e144686_d_n11;
            locals.var_t0_dn14 = assign94370_body8_e144686_d_n14;
            locals.var_t0_rv = 0.0;
            let (assign94370_body9_e144701, assign94370_body9_e144701_d_n0, assign94370_body9_e144701_d_n2, assign94370_body9_e144701_d_n4, assign94370_body9_e144701_d_n5, assign94370_body9_e144701_d_n6, assign94370_body9_e144701_d_n7, assign94370_body9_e144701_d_n8, assign94370_body9_e144701_d_n9, assign94370_body9_e144701_d_n10, assign94370_body9_e144701_d_n11, assign94370_body9_e144701_d_n14,) = {
    if (((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2182 != 0.0)) && (locals.var_guard2187 != 0.0)) {
        let assign94370_body9_e144699: f64 = (locals.var_t1 - locals.var_t0);
        (assign94370_body9_e144699, (locals.var_t1_dn0 - locals.var_t0_dn0), (locals.var_t1_dn2 - locals.var_t0_dn2), (locals.var_t1_dn4 - locals.var_t0_dn4), (locals.var_t1_dn5 - locals.var_t0_dn5), (locals.var_t1_dn6 - locals.var_t0_dn6), (locals.var_t1_dn7 - locals.var_t0_dn7), (locals.var_t1_dn8 - locals.var_t0_dn8), (locals.var_t1_dn9 - locals.var_t0_dn9), (locals.var_t1_dn10 - locals.var_t0_dn10), (locals.var_t1_dn11 - locals.var_t0_dn11), (locals.var_t1_dn14 - locals.var_t0_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
            locals.var_t2 = assign94370_body9_e144701;
            locals.var_t2_dn0 = assign94370_body9_e144701_d_n0;
            locals.var_t2_dn2 = assign94370_body9_e144701_d_n2;
            locals.var_t2_dn4 = assign94370_body9_e144701_d_n4;
            locals.var_t2_dn5 = assign94370_body9_e144701_d_n5;
            locals.var_t2_dn6 = assign94370_body9_e144701_d_n6;
            locals.var_t2_dn7 = assign94370_body9_e144701_d_n7;
            locals.var_t2_dn8 = assign94370_body9_e144701_d_n8;
            locals.var_t2_dn9 = assign94370_body9_e144701_d_n9;
            locals.var_t2_dn10 = assign94370_body9_e144701_d_n10;
            locals.var_t2_dn11 = assign94370_body9_e144701_d_n11;
            locals.var_t2_dn14 = assign94370_body9_e144701_d_n14;
            locals.var_t2_rv = 0.0;
            let (assign94370_body10_e144719, assign94370_body10_e144719_d_n0, assign94370_body10_e144719_d_n2, assign94370_body10_e144719_d_n4, assign94370_body10_e144719_d_n5, assign94370_body10_e144719_d_n6, assign94370_body10_e144719_d_n7, assign94370_body10_e144719_d_n8, assign94370_body10_e144719_d_n9, assign94370_body10_e144719_d_n10, assign94370_body10_e144719_d_n11, assign94370_body10_e144719_d_n14,) = {
    if (((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2182 != 0.0)) && (locals.var_guard2187 != 0.0)) {
        let assign94370_body10_e144714: f64 = (1.0 + locals.var_t2);
        let assign94370_body10_e144715: f64 = (assign94370_body10_e144714).ln();
        let assign94370_body10_e144717: f64 = (assign94370_body10_e144715 / locals.var_c_sb);
        (assign94370_body10_e144717, ((((locals.var_t2_dn0 / assign94370_body10_e144714) * locals.var_c_sb) - (assign94370_body10_e144715 * locals.var_c_sb_dn0)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn2 / assign94370_body10_e144714) * locals.var_c_sb) - (assign94370_body10_e144715 * locals.var_c_sb_dn2)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn4 / assign94370_body10_e144714) * locals.var_c_sb) - (assign94370_body10_e144715 * locals.var_c_sb_dn4)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn5 / assign94370_body10_e144714) * locals.var_c_sb) - (assign94370_body10_e144715 * locals.var_c_sb_dn5)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn6 / assign94370_body10_e144714) * locals.var_c_sb) - (assign94370_body10_e144715 * locals.var_c_sb_dn6)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn7 / assign94370_body10_e144714) * locals.var_c_sb) - (assign94370_body10_e144715 * locals.var_c_sb_dn7)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn8 / assign94370_body10_e144714) * locals.var_c_sb) - (assign94370_body10_e144715 * locals.var_c_sb_dn8)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn9 / assign94370_body10_e144714) * locals.var_c_sb) - (assign94370_body10_e144715 * locals.var_c_sb_dn9)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn10 / assign94370_body10_e144714) * locals.var_c_sb) - (assign94370_body10_e144715 * locals.var_c_sb_dn10)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn11 / assign94370_body10_e144714) * locals.var_c_sb) - (assign94370_body10_e144715 * locals.var_c_sb_dn11)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn14 / assign94370_body10_e144714) * locals.var_c_sb) - (assign94370_body10_e144715 * locals.var_c_sb_dn14)) / (locals.var_c_sb * locals.var_c_sb)),)
    } else {
        (locals.var_phi_b, locals.var_phi_b_dn0, locals.var_phi_b_dn2, locals.var_phi_b_dn4, locals.var_phi_b_dn5, locals.var_phi_b_dn6, locals.var_phi_b_dn7, locals.var_phi_b_dn8, locals.var_phi_b_dn9, locals.var_phi_b_dn10, locals.var_phi_b_dn11, locals.var_phi_b_dn14,)
    }
};
            locals.var_phi_b = assign94370_body10_e144719;
            locals.var_phi_b_dn0 = assign94370_body10_e144719_d_n0;
            locals.var_phi_b_dn2 = assign94370_body10_e144719_d_n2;
            locals.var_phi_b_dn4 = assign94370_body10_e144719_d_n4;
            locals.var_phi_b_dn5 = assign94370_body10_e144719_d_n5;
            locals.var_phi_b_dn6 = assign94370_body10_e144719_d_n6;
            locals.var_phi_b_dn7 = assign94370_body10_e144719_d_n7;
            locals.var_phi_b_dn8 = assign94370_body10_e144719_d_n8;
            locals.var_phi_b_dn9 = assign94370_body10_e144719_d_n9;
            locals.var_phi_b_dn10 = assign94370_body10_e144719_d_n10;
            locals.var_phi_b_dn11 = assign94370_body10_e144719_d_n11;
            locals.var_phi_b_dn14 = assign94370_body10_e144719_d_n14;
            locals.var_phi_b_rv = 0.0;
            let (assign94370_body11_e144736, assign94370_body11_e144736_d_n0, assign94370_body11_e144736_d_n2, assign94370_body11_e144736_d_n4, assign94370_body11_e144736_d_n5, assign94370_body11_e144736_d_n6, assign94370_body11_e144736_d_n7, assign94370_body11_e144736_d_n8, assign94370_body11_e144736_d_n9, assign94370_body11_e144736_d_n10, assign94370_body11_e144736_d_n11, assign94370_body11_e144736_d_n14,) = {
    if (((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2182 != 0.0)) && (locals.var_guard2187 != 0.0)) {
        let assign94370_body11_e144733: f64 = (1.0 + locals.var_t2);
        let assign94370_body11_e144734: f64 = (locals.var_t1 / assign94370_body11_e144733);
        (assign94370_body11_e144734, (((locals.var_t1_dn0 * assign94370_body11_e144733) - (locals.var_t1 * locals.var_t2_dn0)) / (assign94370_body11_e144733 * assign94370_body11_e144733)), (((locals.var_t1_dn2 * assign94370_body11_e144733) - (locals.var_t1 * locals.var_t2_dn2)) / (assign94370_body11_e144733 * assign94370_body11_e144733)), (((locals.var_t1_dn4 * assign94370_body11_e144733) - (locals.var_t1 * locals.var_t2_dn4)) / (assign94370_body11_e144733 * assign94370_body11_e144733)), (((locals.var_t1_dn5 * assign94370_body11_e144733) - (locals.var_t1 * locals.var_t2_dn5)) / (assign94370_body11_e144733 * assign94370_body11_e144733)), (((locals.var_t1_dn6 * assign94370_body11_e144733) - (locals.var_t1 * locals.var_t2_dn6)) / (assign94370_body11_e144733 * assign94370_body11_e144733)), (((locals.var_t1_dn7 * assign94370_body11_e144733) - (locals.var_t1 * locals.var_t2_dn7)) / (assign94370_body11_e144733 * assign94370_body11_e144733)), (((locals.var_t1_dn8 * assign94370_body11_e144733) - (locals.var_t1 * locals.var_t2_dn8)) / (assign94370_body11_e144733 * assign94370_body11_e144733)), (((locals.var_t1_dn9 * assign94370_body11_e144733) - (locals.var_t1 * locals.var_t2_dn9)) / (assign94370_body11_e144733 * assign94370_body11_e144733)), (((locals.var_t1_dn10 * assign94370_body11_e144733) - (locals.var_t1 * locals.var_t2_dn10)) / (assign94370_body11_e144733 * assign94370_body11_e144733)), (((locals.var_t1_dn11 * assign94370_body11_e144733) - (locals.var_t1 * locals.var_t2_dn11)) / (assign94370_body11_e144733 * assign94370_body11_e144733)), (((locals.var_t1_dn14 * assign94370_body11_e144733) - (locals.var_t1 * locals.var_t2_dn14)) / (assign94370_body11_e144733 * assign94370_body11_e144733)),)
    } else {
        (locals.var_phi_b_dpss, locals.var_phi_b_dpss_dn0, locals.var_phi_b_dpss_dn2, locals.var_phi_b_dpss_dn4, locals.var_phi_b_dpss_dn5, locals.var_phi_b_dpss_dn6, locals.var_phi_b_dpss_dn7, locals.var_phi_b_dpss_dn8, locals.var_phi_b_dpss_dn9, locals.var_phi_b_dpss_dn10, locals.var_phi_b_dpss_dn11, locals.var_phi_b_dpss_dn14,)
    }
};
            locals.var_phi_b_dpss = assign94370_body11_e144736;
            locals.var_phi_b_dpss_dn0 = assign94370_body11_e144736_d_n0;
            locals.var_phi_b_dpss_dn2 = assign94370_body11_e144736_d_n2;
            locals.var_phi_b_dpss_dn4 = assign94370_body11_e144736_d_n4;
            locals.var_phi_b_dpss_dn5 = assign94370_body11_e144736_d_n5;
            locals.var_phi_b_dpss_dn6 = assign94370_body11_e144736_d_n6;
            locals.var_phi_b_dpss_dn7 = assign94370_body11_e144736_d_n7;
            locals.var_phi_b_dpss_dn8 = assign94370_body11_e144736_d_n8;
            locals.var_phi_b_dpss_dn9 = assign94370_body11_e144736_d_n9;
            locals.var_phi_b_dpss_dn10 = assign94370_body11_e144736_d_n10;
            locals.var_phi_b_dpss_dn11 = assign94370_body11_e144736_d_n11;
            locals.var_phi_b_dpss_dn14 = assign94370_body11_e144736_d_n14;
            locals.var_phi_b_dpss_rv = 0.0;
            let (assign94370_body13_e144768, assign94370_body13_e144768_d_n0, assign94370_body13_e144768_d_n2, assign94370_body13_e144768_d_n4, assign94370_body13_e144768_d_n5, assign94370_body13_e144768_d_n6, assign94370_body13_e144768_d_n7, assign94370_body13_e144768_d_n8, assign94370_body13_e144768_d_n9, assign94370_body13_e144768_d_n10, assign94370_body13_e144768_d_n11, assign94370_body13_e144768_d_n14,) = {
    if (((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2182 != 0.0)) && (locals.var_guard2187 == 0.0)) {
        let assign94370_body13_e144766: f64 = (locals.var_ps0ld_vxb - locals.var_dphi_sb);
        (assign94370_body13_e144766, (locals.var_ps0ld_vxb_dn0 - locals.var_dphi_sb_dn0), (locals.var_ps0ld_vxb_dn2 - locals.var_dphi_sb_dn2), (locals.var_ps0ld_vxb_dn4 - locals.var_dphi_sb_dn4), (locals.var_ps0ld_vxb_dn5 - locals.var_dphi_sb_dn5), (locals.var_ps0ld_vxb_dn6 - locals.var_dphi_sb_dn6), (locals.var_ps0ld_vxb_dn7 - locals.var_dphi_sb_dn7), (locals.var_ps0ld_vxb_dn8 - locals.var_dphi_sb_dn8), (locals.var_ps0ld_vxb_dn9 - locals.var_dphi_sb_dn9), (locals.var_ps0ld_vxb_dn10 - locals.var_dphi_sb_dn10), (locals.var_ps0ld_vxb_dn11 - locals.var_dphi_sb_dn11), (locals.var_ps0ld_vxb_dn14 - locals.var_dphi_sb_dn14),)
    } else {
        (locals.var_phi_b, locals.var_phi_b_dn0, locals.var_phi_b_dn2, locals.var_phi_b_dn4, locals.var_phi_b_dn5, locals.var_phi_b_dn6, locals.var_phi_b_dn7, locals.var_phi_b_dn8, locals.var_phi_b_dn9, locals.var_phi_b_dn10, locals.var_phi_b_dn11, locals.var_phi_b_dn14,)
    }
};
            locals.var_phi_b = assign94370_body13_e144768;
            locals.var_phi_b_dn0 = assign94370_body13_e144768_d_n0;
            locals.var_phi_b_dn2 = assign94370_body13_e144768_d_n2;
            locals.var_phi_b_dn4 = assign94370_body13_e144768_d_n4;
            locals.var_phi_b_dn5 = assign94370_body13_e144768_d_n5;
            locals.var_phi_b_dn6 = assign94370_body13_e144768_d_n6;
            locals.var_phi_b_dn7 = assign94370_body13_e144768_d_n7;
            locals.var_phi_b_dn8 = assign94370_body13_e144768_d_n8;
            locals.var_phi_b_dn9 = assign94370_body13_e144768_d_n9;
            locals.var_phi_b_dn10 = assign94370_body13_e144768_d_n10;
            locals.var_phi_b_dn11 = assign94370_body13_e144768_d_n11;
            locals.var_phi_b_dn14 = assign94370_body13_e144768_d_n14;
            locals.var_phi_b_rv = 0.0;
            let (assign94370_body14_e144782, assign94370_body14_e144782_d_n0, assign94370_body14_e144782_d_n2, assign94370_body14_e144782_d_n4, assign94370_body14_e144782_d_n5, assign94370_body14_e144782_d_n6, assign94370_body14_e144782_d_n7, assign94370_body14_e144782_d_n8, assign94370_body14_e144782_d_n9, assign94370_body14_e144782_d_n10, assign94370_body14_e144782_d_n11, assign94370_body14_e144782_d_n14,) = {
    if (((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2182 != 0.0)) && (locals.var_guard2187 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phi_b_dpss, locals.var_phi_b_dpss_dn0, locals.var_phi_b_dpss_dn2, locals.var_phi_b_dpss_dn4, locals.var_phi_b_dpss_dn5, locals.var_phi_b_dpss_dn6, locals.var_phi_b_dpss_dn7, locals.var_phi_b_dpss_dn8, locals.var_phi_b_dpss_dn9, locals.var_phi_b_dpss_dn10, locals.var_phi_b_dpss_dn11, locals.var_phi_b_dpss_dn14,)
    }
};
            locals.var_phi_b_dpss = assign94370_body14_e144782;
            locals.var_phi_b_dpss_dn0 = assign94370_body14_e144782_d_n0;
            locals.var_phi_b_dpss_dn2 = assign94370_body14_e144782_d_n2;
            locals.var_phi_b_dpss_dn4 = assign94370_body14_e144782_d_n4;
            locals.var_phi_b_dpss_dn5 = assign94370_body14_e144782_d_n5;
            locals.var_phi_b_dpss_dn6 = assign94370_body14_e144782_d_n6;
            locals.var_phi_b_dpss_dn7 = assign94370_body14_e144782_d_n7;
            locals.var_phi_b_dpss_dn8 = assign94370_body14_e144782_d_n8;
            locals.var_phi_b_dpss_dn9 = assign94370_body14_e144782_d_n9;
            locals.var_phi_b_dpss_dn10 = assign94370_body14_e144782_d_n10;
            locals.var_phi_b_dpss_dn11 = assign94370_body14_e144782_d_n11;
            locals.var_phi_b_dpss_dn14 = assign94370_body14_e144782_d_n14;
            locals.var_phi_b_dpss_rv = 0.0;
            let (assign94370_body15_e144795, assign94370_body15_e144795_d_n0, assign94370_body15_e144795_d_n2, assign94370_body15_e144795_d_n4, assign94370_body15_e144795_d_n5, assign94370_body15_e144795_d_n6, assign94370_body15_e144795_d_n7, assign94370_body15_e144795_d_n8, assign94370_body15_e144795_d_n9, assign94370_body15_e144795_d_n10, assign94370_body15_e144795_d_n11, assign94370_body15_e144795_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2182 != 0.0)) {
        let assign94370_body15_e144793: f64 = (locals.var_beta * locals.var_phi_b);
        (assign94370_body15_e144793, ((locals.var_beta_dn0 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn0)), ((locals.var_beta_dn2 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn2)), ((locals.var_beta_dn4 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn4)), ((locals.var_beta_dn5 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn5)), ((locals.var_beta_dn6 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn6)), ((locals.var_beta_dn7 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn7)), ((locals.var_beta_dn8 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn8)), ((locals.var_beta_dn9 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn9)), ((locals.var_beta_dn10 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn10)), ((locals.var_beta_dn11 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn11)), ((locals.var_beta_dn14 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn14)),)
    } else {
        (locals.var_chib, locals.var_chib_dn0, locals.var_chib_dn2, locals.var_chib_dn4, locals.var_chib_dn5, locals.var_chib_dn6, locals.var_chib_dn7, locals.var_chib_dn8, locals.var_chib_dn9, locals.var_chib_dn10, locals.var_chib_dn11, locals.var_chib_dn14,)
    }
};
            locals.var_chib = assign94370_body15_e144795;
            locals.var_chib_dn0 = assign94370_body15_e144795_d_n0;
            locals.var_chib_dn2 = assign94370_body15_e144795_d_n2;
            locals.var_chib_dn4 = assign94370_body15_e144795_d_n4;
            locals.var_chib_dn5 = assign94370_body15_e144795_d_n5;
            locals.var_chib_dn6 = assign94370_body15_e144795_d_n6;
            locals.var_chib_dn7 = assign94370_body15_e144795_d_n7;
            locals.var_chib_dn8 = assign94370_body15_e144795_d_n8;
            locals.var_chib_dn9 = assign94370_body15_e144795_d_n9;
            locals.var_chib_dn10 = assign94370_body15_e144795_d_n10;
            locals.var_chib_dn11 = assign94370_body15_e144795_d_n11;
            locals.var_chib_dn14 = assign94370_body15_e144795_d_n14;
            locals.var_chib_rv = 0.0;
            let assign94370_body16_e144798: f64 = if locals.var_chi < 0.0 { 1.0 } else { 0.0 };
            locals.var_guard2188 = assign94370_body16_e144798;
            locals.var_guard2188_rv = 0.0;
            let (assign94370_body18_e144827, assign94370_body18_e144827_d_n0, assign94370_body18_e144827_d_n2, assign94370_body18_e144827_d_n4, assign94370_body18_e144827_d_n5, assign94370_body18_e144827_d_n6, assign94370_body18_e144827_d_n7, assign94370_body18_e144827_d_n8, assign94370_body18_e144827_d_n9, assign94370_body18_e144827_d_n10, assign94370_body18_e144827_d_n11, assign94370_body18_e144827_d_n14,) = {
    if (((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2182 != 0.0)) && (locals.var_guard2188 != 0.0)) {
        let assign94370_body18_e144825: f64 = (-0.7071067811865475);
        (assign94370_body18_e144825, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
            locals.var_t0 = assign94370_body18_e144827;
            locals.var_t0_dn0 = assign94370_body18_e144827_d_n0;
            locals.var_t0_dn2 = assign94370_body18_e144827_d_n2;
            locals.var_t0_dn4 = assign94370_body18_e144827_d_n4;
            locals.var_t0_dn5 = assign94370_body18_e144827_d_n5;
            locals.var_t0_dn6 = assign94370_body18_e144827_d_n6;
            locals.var_t0_dn7 = assign94370_body18_e144827_d_n7;
            locals.var_t0_dn8 = assign94370_body18_e144827_d_n8;
            locals.var_t0_dn9 = assign94370_body18_e144827_d_n9;
            locals.var_t0_dn10 = assign94370_body18_e144827_d_n10;
            locals.var_t0_dn11 = assign94370_body18_e144827_d_n11;
            locals.var_t0_dn14 = assign94370_body18_e144827_d_n14;
            locals.var_t0_rv = 0.0;
            let (assign94370_body19_e144842, assign94370_body19_e144842_d_n0, assign94370_body19_e144842_d_n2, assign94370_body19_e144842_d_n4, assign94370_body19_e144842_d_n5, assign94370_body19_e144842_d_n6, assign94370_body19_e144842_d_n7, assign94370_body19_e144842_d_n8, assign94370_body19_e144842_d_n9, assign94370_body19_e144842_d_n10, assign94370_body19_e144842_d_n11, assign94370_body19_e144842_d_n14,) = {
    if (((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2182 != 0.0)) && (locals.var_guard2188 != 0.0)) {
        let assign94370_body19_e144840: f64 = (locals.var_chi * locals.var_t0);
        (assign94370_body19_e144840, ((locals.var_chi_dn0 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn0)), ((locals.var_chi_dn2 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn2)), ((locals.var_chi_dn4 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn4)), ((locals.var_chi_dn5 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn5)), ((locals.var_chi_dn6 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn6)), ((locals.var_chi_dn7 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn7)), ((locals.var_chi_dn8 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn8)), ((locals.var_chi_dn9 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn9)), ((locals.var_chi_dn10 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn10)), ((locals.var_chi_dn11 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn11)), ((locals.var_chi_dn14 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn14)),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn14,)
    }
};
            locals.var_fb = assign94370_body19_e144842;
            locals.var_fb_dn0 = assign94370_body19_e144842_d_n0;
            locals.var_fb_dn2 = assign94370_body19_e144842_d_n2;
            locals.var_fb_dn4 = assign94370_body19_e144842_d_n4;
            locals.var_fb_dn5 = assign94370_body19_e144842_d_n5;
            locals.var_fb_dn6 = assign94370_body19_e144842_d_n6;
            locals.var_fb_dn7 = assign94370_body19_e144842_d_n7;
            locals.var_fb_dn8 = assign94370_body19_e144842_d_n8;
            locals.var_fb_dn9 = assign94370_body19_e144842_d_n9;
            locals.var_fb_dn10 = assign94370_body19_e144842_d_n10;
            locals.var_fb_dn11 = assign94370_body19_e144842_d_n11;
            locals.var_fb_dn14 = assign94370_body19_e144842_d_n14;
            locals.var_fb_rv = 0.0;
            let (assign94370_body20_e144857, assign94370_body20_e144857_d_n0, assign94370_body20_e144857_d_n2, assign94370_body20_e144857_d_n4, assign94370_body20_e144857_d_n5, assign94370_body20_e144857_d_n6, assign94370_body20_e144857_d_n7, assign94370_body20_e144857_d_n8, assign94370_body20_e144857_d_n9, assign94370_body20_e144857_d_n10, assign94370_body20_e144857_d_n11, assign94370_body20_e144857_d_n14,) = {
    if (((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2182 != 0.0)) && (locals.var_guard2188 != 0.0)) {
        let assign94370_body20_e144855: f64 = (locals.var_beta * locals.var_t0);
        (assign94370_body20_e144855, ((locals.var_beta_dn0 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn0)), ((locals.var_beta_dn2 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn2)), ((locals.var_beta_dn4 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn4)), ((locals.var_beta_dn5 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn5)), ((locals.var_beta_dn6 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn6)), ((locals.var_beta_dn7 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn7)), ((locals.var_beta_dn8 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn8)), ((locals.var_beta_dn9 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn9)), ((locals.var_beta_dn10 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn10)), ((locals.var_beta_dn11 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn11)), ((locals.var_beta_dn14 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn14)),)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn14,)
    }
};
            locals.var_fb_dpss = assign94370_body20_e144857;
            locals.var_fb_dpss_dn0 = assign94370_body20_e144857_d_n0;
            locals.var_fb_dpss_dn2 = assign94370_body20_e144857_d_n2;
            locals.var_fb_dpss_dn4 = assign94370_body20_e144857_d_n4;
            locals.var_fb_dpss_dn5 = assign94370_body20_e144857_d_n5;
            locals.var_fb_dpss_dn6 = assign94370_body20_e144857_d_n6;
            locals.var_fb_dpss_dn7 = assign94370_body20_e144857_d_n7;
            locals.var_fb_dpss_dn8 = assign94370_body20_e144857_d_n8;
            locals.var_fb_dpss_dn9 = assign94370_body20_e144857_d_n9;
            locals.var_fb_dpss_dn10 = assign94370_body20_e144857_d_n10;
            locals.var_fb_dpss_dn11 = assign94370_body20_e144857_d_n11;
            locals.var_fb_dpss_dn14 = assign94370_body20_e144857_d_n14;
            locals.var_fb_dpss_rv = 0.0;
            let assign94370_body21_e144860: f64 = if locals.var_chi < 1e-6 { 1.0 } else { 0.0 };
            locals.var_guard2189 = assign94370_body21_e144860;
            locals.var_guard2189_rv = 0.0;
            let (assign94370_body23_e144916, assign94370_body23_e144916_d_n0, assign94370_body23_e144916_d_n2, assign94370_body23_e144916_d_n4, assign94370_body23_e144916_d_n5, assign94370_body23_e144916_d_n6, assign94370_body23_e144916_d_n7, assign94370_body23_e144916_d_n8, assign94370_body23_e144916_d_n9, assign94370_body23_e144916_d_n10, assign94370_body23_e144916_d_n11, assign94370_body23_e144916_d_n14,) = {
    if ((((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2182 != 0.0)) && (locals.var_guard2188 == 0.0)) && (locals.var_guard2189 != 0.0)) {
        let assign94370_body23_e144894: f64 = (locals.var_chi * locals.var_chi);
        let assign94370_body23_e144896: f64 = (assign94370_body23_e144894 / 2.0);
        let assign94370_body23_e144900: f64 = (locals.var_chi / 3.0);
        let assign94370_body23_e144904: f64 = (locals.var_chi / 4.0);
        let assign94370_body23_e144908: f64 = (locals.var_chi / 5.0);
        let assign94370_body23_e144909: f64 = (1.0 - assign94370_body23_e144908);
        let assign94370_body23_e144910: f64 = (assign94370_body23_e144904 * assign94370_body23_e144909);
        let assign94370_body23_e144911: f64 = (1.0 - assign94370_body23_e144910);
        let assign94370_body23_e144912: f64 = (assign94370_body23_e144900 * assign94370_body23_e144911);
        let assign94370_body23_e144913: f64 = (1.0 - assign94370_body23_e144912);
        let assign94370_body23_e144914: f64 = (assign94370_body23_e144896 * assign94370_body23_e144913);
        (assign94370_body23_e144914, (((((locals.var_chi_dn0 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn0)) / 2.0) * assign94370_body23_e144913) + (assign94370_body23_e144896 * (-(((locals.var_chi_dn0 / 3.0) * assign94370_body23_e144911) + (assign94370_body23_e144900 * (-(((locals.var_chi_dn0 / 4.0) * assign94370_body23_e144909) + (assign94370_body23_e144904 * (-(locals.var_chi_dn0 / 5.0)))))))))), (((((locals.var_chi_dn2 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn2)) / 2.0) * assign94370_body23_e144913) + (assign94370_body23_e144896 * (-(((locals.var_chi_dn2 / 3.0) * assign94370_body23_e144911) + (assign94370_body23_e144900 * (-(((locals.var_chi_dn2 / 4.0) * assign94370_body23_e144909) + (assign94370_body23_e144904 * (-(locals.var_chi_dn2 / 5.0)))))))))), (((((locals.var_chi_dn4 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn4)) / 2.0) * assign94370_body23_e144913) + (assign94370_body23_e144896 * (-(((locals.var_chi_dn4 / 3.0) * assign94370_body23_e144911) + (assign94370_body23_e144900 * (-(((locals.var_chi_dn4 / 4.0) * assign94370_body23_e144909) + (assign94370_body23_e144904 * (-(locals.var_chi_dn4 / 5.0)))))))))), (((((locals.var_chi_dn5 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn5)) / 2.0) * assign94370_body23_e144913) + (assign94370_body23_e144896 * (-(((locals.var_chi_dn5 / 3.0) * assign94370_body23_e144911) + (assign94370_body23_e144900 * (-(((locals.var_chi_dn5 / 4.0) * assign94370_body23_e144909) + (assign94370_body23_e144904 * (-(locals.var_chi_dn5 / 5.0)))))))))), (((((locals.var_chi_dn6 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn6)) / 2.0) * assign94370_body23_e144913) + (assign94370_body23_e144896 * (-(((locals.var_chi_dn6 / 3.0) * assign94370_body23_e144911) + (assign94370_body23_e144900 * (-(((locals.var_chi_dn6 / 4.0) * assign94370_body23_e144909) + (assign94370_body23_e144904 * (-(locals.var_chi_dn6 / 5.0)))))))))), (((((locals.var_chi_dn7 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn7)) / 2.0) * assign94370_body23_e144913) + (assign94370_body23_e144896 * (-(((locals.var_chi_dn7 / 3.0) * assign94370_body23_e144911) + (assign94370_body23_e144900 * (-(((locals.var_chi_dn7 / 4.0) * assign94370_body23_e144909) + (assign94370_body23_e144904 * (-(locals.var_chi_dn7 / 5.0)))))))))), (((((locals.var_chi_dn8 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn8)) / 2.0) * assign94370_body23_e144913) + (assign94370_body23_e144896 * (-(((locals.var_chi_dn8 / 3.0) * assign94370_body23_e144911) + (assign94370_body23_e144900 * (-(((locals.var_chi_dn8 / 4.0) * assign94370_body23_e144909) + (assign94370_body23_e144904 * (-(locals.var_chi_dn8 / 5.0)))))))))), (((((locals.var_chi_dn9 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn9)) / 2.0) * assign94370_body23_e144913) + (assign94370_body23_e144896 * (-(((locals.var_chi_dn9 / 3.0) * assign94370_body23_e144911) + (assign94370_body23_e144900 * (-(((locals.var_chi_dn9 / 4.0) * assign94370_body23_e144909) + (assign94370_body23_e144904 * (-(locals.var_chi_dn9 / 5.0)))))))))), (((((locals.var_chi_dn10 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn10)) / 2.0) * assign94370_body23_e144913) + (assign94370_body23_e144896 * (-(((locals.var_chi_dn10 / 3.0) * assign94370_body23_e144911) + (assign94370_body23_e144900 * (-(((locals.var_chi_dn10 / 4.0) * assign94370_body23_e144909) + (assign94370_body23_e144904 * (-(locals.var_chi_dn10 / 5.0)))))))))), (((((locals.var_chi_dn11 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn11)) / 2.0) * assign94370_body23_e144913) + (assign94370_body23_e144896 * (-(((locals.var_chi_dn11 / 3.0) * assign94370_body23_e144911) + (assign94370_body23_e144900 * (-(((locals.var_chi_dn11 / 4.0) * assign94370_body23_e144909) + (assign94370_body23_e144904 * (-(locals.var_chi_dn11 / 5.0)))))))))), (((((locals.var_chi_dn14 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn14)) / 2.0) * assign94370_body23_e144913) + (assign94370_body23_e144896 * (-(((locals.var_chi_dn14 / 3.0) * assign94370_body23_e144911) + (assign94370_body23_e144900 * (-(((locals.var_chi_dn14 / 4.0) * assign94370_body23_e144909) + (assign94370_body23_e144904 * (-(locals.var_chi_dn14 / 5.0)))))))))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
            locals.var_t0 = assign94370_body23_e144916;
            locals.var_t0_dn0 = assign94370_body23_e144916_d_n0;
            locals.var_t0_dn2 = assign94370_body23_e144916_d_n2;
            locals.var_t0_dn4 = assign94370_body23_e144916_d_n4;
            locals.var_t0_dn5 = assign94370_body23_e144916_d_n5;
            locals.var_t0_dn6 = assign94370_body23_e144916_d_n6;
            locals.var_t0_dn7 = assign94370_body23_e144916_d_n7;
            locals.var_t0_dn8 = assign94370_body23_e144916_d_n8;
            locals.var_t0_dn9 = assign94370_body23_e144916_d_n9;
            locals.var_t0_dn10 = assign94370_body23_e144916_d_n10;
            locals.var_t0_dn11 = assign94370_body23_e144916_d_n11;
            locals.var_t0_dn14 = assign94370_body23_e144916_d_n14;
            locals.var_t0_rv = 0.0;
            let (assign94370_body24_e144950, assign94370_body24_e144950_d_n0, assign94370_body24_e144950_d_n2, assign94370_body24_e144950_d_n4, assign94370_body24_e144950_d_n5, assign94370_body24_e144950_d_n6, assign94370_body24_e144950_d_n7, assign94370_body24_e144950_d_n8, assign94370_body24_e144950_d_n9, assign94370_body24_e144950_d_n10, assign94370_body24_e144950_d_n11, assign94370_body24_e144950_d_n14,) = {
    if ((((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2182 != 0.0)) && (locals.var_guard2188 == 0.0)) && (locals.var_guard2189 != 0.0)) {
        let assign94370_body24_e144934: f64 = (locals.var_chi / 2.0);
        let assign94370_body24_e144938: f64 = (locals.var_chi / 3.0);
        let assign94370_body24_e144942: f64 = (locals.var_chi / 4.0);
        let assign94370_body24_e144943: f64 = (1.0 - assign94370_body24_e144942);
        let assign94370_body24_e144944: f64 = (assign94370_body24_e144938 * assign94370_body24_e144943);
        let assign94370_body24_e144945: f64 = (1.0 - assign94370_body24_e144944);
        let assign94370_body24_e144946: f64 = (assign94370_body24_e144934 * assign94370_body24_e144945);
        let assign94370_body24_e144947: f64 = (1.0 - assign94370_body24_e144946);
        let assign94370_body24_e144948: f64 = (locals.var_chi * assign94370_body24_e144947);
        (assign94370_body24_e144948, ((locals.var_chi_dn0 * assign94370_body24_e144947) + (locals.var_chi * (-(((locals.var_chi_dn0 / 2.0) * assign94370_body24_e144945) + (assign94370_body24_e144934 * (-(((locals.var_chi_dn0 / 3.0) * assign94370_body24_e144943) + (assign94370_body24_e144938 * (-(locals.var_chi_dn0 / 4.0)))))))))), ((locals.var_chi_dn2 * assign94370_body24_e144947) + (locals.var_chi * (-(((locals.var_chi_dn2 / 2.0) * assign94370_body24_e144945) + (assign94370_body24_e144934 * (-(((locals.var_chi_dn2 / 3.0) * assign94370_body24_e144943) + (assign94370_body24_e144938 * (-(locals.var_chi_dn2 / 4.0)))))))))), ((locals.var_chi_dn4 * assign94370_body24_e144947) + (locals.var_chi * (-(((locals.var_chi_dn4 / 2.0) * assign94370_body24_e144945) + (assign94370_body24_e144934 * (-(((locals.var_chi_dn4 / 3.0) * assign94370_body24_e144943) + (assign94370_body24_e144938 * (-(locals.var_chi_dn4 / 4.0)))))))))), ((locals.var_chi_dn5 * assign94370_body24_e144947) + (locals.var_chi * (-(((locals.var_chi_dn5 / 2.0) * assign94370_body24_e144945) + (assign94370_body24_e144934 * (-(((locals.var_chi_dn5 / 3.0) * assign94370_body24_e144943) + (assign94370_body24_e144938 * (-(locals.var_chi_dn5 / 4.0)))))))))), ((locals.var_chi_dn6 * assign94370_body24_e144947) + (locals.var_chi * (-(((locals.var_chi_dn6 / 2.0) * assign94370_body24_e144945) + (assign94370_body24_e144934 * (-(((locals.var_chi_dn6 / 3.0) * assign94370_body24_e144943) + (assign94370_body24_e144938 * (-(locals.var_chi_dn6 / 4.0)))))))))), ((locals.var_chi_dn7 * assign94370_body24_e144947) + (locals.var_chi * (-(((locals.var_chi_dn7 / 2.0) * assign94370_body24_e144945) + (assign94370_body24_e144934 * (-(((locals.var_chi_dn7 / 3.0) * assign94370_body24_e144943) + (assign94370_body24_e144938 * (-(locals.var_chi_dn7 / 4.0)))))))))), ((locals.var_chi_dn8 * assign94370_body24_e144947) + (locals.var_chi * (-(((locals.var_chi_dn8 / 2.0) * assign94370_body24_e144945) + (assign94370_body24_e144934 * (-(((locals.var_chi_dn8 / 3.0) * assign94370_body24_e144943) + (assign94370_body24_e144938 * (-(locals.var_chi_dn8 / 4.0)))))))))), ((locals.var_chi_dn9 * assign94370_body24_e144947) + (locals.var_chi * (-(((locals.var_chi_dn9 / 2.0) * assign94370_body24_e144945) + (assign94370_body24_e144934 * (-(((locals.var_chi_dn9 / 3.0) * assign94370_body24_e144943) + (assign94370_body24_e144938 * (-(locals.var_chi_dn9 / 4.0)))))))))), ((locals.var_chi_dn10 * assign94370_body24_e144947) + (locals.var_chi * (-(((locals.var_chi_dn10 / 2.0) * assign94370_body24_e144945) + (assign94370_body24_e144934 * (-(((locals.var_chi_dn10 / 3.0) * assign94370_body24_e144943) + (assign94370_body24_e144938 * (-(locals.var_chi_dn10 / 4.0)))))))))), ((locals.var_chi_dn11 * assign94370_body24_e144947) + (locals.var_chi * (-(((locals.var_chi_dn11 / 2.0) * assign94370_body24_e144945) + (assign94370_body24_e144934 * (-(((locals.var_chi_dn11 / 3.0) * assign94370_body24_e144943) + (assign94370_body24_e144938 * (-(locals.var_chi_dn11 / 4.0)))))))))), ((locals.var_chi_dn14 * assign94370_body24_e144947) + (locals.var_chi * (-(((locals.var_chi_dn14 / 2.0) * assign94370_body24_e144945) + (assign94370_body24_e144934 * (-(((locals.var_chi_dn14 / 3.0) * assign94370_body24_e144943) + (assign94370_body24_e144938 * (-(locals.var_chi_dn14 / 4.0)))))))))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
            locals.var_t1 = assign94370_body24_e144950;
            locals.var_t1_dn0 = assign94370_body24_e144950_d_n0;
            locals.var_t1_dn2 = assign94370_body24_e144950_d_n2;
            locals.var_t1_dn4 = assign94370_body24_e144950_d_n4;
            locals.var_t1_dn5 = assign94370_body24_e144950_d_n5;
            locals.var_t1_dn6 = assign94370_body24_e144950_d_n6;
            locals.var_t1_dn7 = assign94370_body24_e144950_d_n7;
            locals.var_t1_dn8 = assign94370_body24_e144950_d_n8;
            locals.var_t1_dn9 = assign94370_body24_e144950_d_n9;
            locals.var_t1_dn10 = assign94370_body24_e144950_d_n10;
            locals.var_t1_dn11 = assign94370_body24_e144950_d_n11;
            locals.var_t1_dn14 = assign94370_body24_e144950_d_n14;
            locals.var_t1_rv = 0.0;
            let (assign94370_body25_e144988, assign94370_body25_e144988_d_n0, assign94370_body25_e144988_d_n2, assign94370_body25_e144988_d_n4, assign94370_body25_e144988_d_n5, assign94370_body25_e144988_d_n6, assign94370_body25_e144988_d_n7, assign94370_body25_e144988_d_n8, assign94370_body25_e144988_d_n9, assign94370_body25_e144988_d_n10, assign94370_body25_e144988_d_n11, assign94370_body25_e144988_d_n14,) = {
    if ((((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2182 != 0.0)) && (locals.var_guard2188 == 0.0)) && (locals.var_guard2189 != 0.0)) {
        let assign94370_body25_e144966: f64 = (locals.var_chib * locals.var_chib);
        let assign94370_body25_e144968: f64 = (assign94370_body25_e144966 / 2.0);
        let assign94370_body25_e144972: f64 = (locals.var_chib / 3.0);
        let assign94370_body25_e144976: f64 = (locals.var_chib / 4.0);
        let assign94370_body25_e144980: f64 = (locals.var_chib / 5.0);
        let assign94370_body25_e144981: f64 = (1.0 - assign94370_body25_e144980);
        let assign94370_body25_e144982: f64 = (assign94370_body25_e144976 * assign94370_body25_e144981);
        let assign94370_body25_e144983: f64 = (1.0 - assign94370_body25_e144982);
        let assign94370_body25_e144984: f64 = (assign94370_body25_e144972 * assign94370_body25_e144983);
        let assign94370_body25_e144985: f64 = (1.0 - assign94370_body25_e144984);
        let assign94370_body25_e144986: f64 = (assign94370_body25_e144968 * assign94370_body25_e144985);
        (assign94370_body25_e144986, (((((locals.var_chib_dn0 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn0)) / 2.0) * assign94370_body25_e144985) + (assign94370_body25_e144968 * (-(((locals.var_chib_dn0 / 3.0) * assign94370_body25_e144983) + (assign94370_body25_e144972 * (-(((locals.var_chib_dn0 / 4.0) * assign94370_body25_e144981) + (assign94370_body25_e144976 * (-(locals.var_chib_dn0 / 5.0)))))))))), (((((locals.var_chib_dn2 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn2)) / 2.0) * assign94370_body25_e144985) + (assign94370_body25_e144968 * (-(((locals.var_chib_dn2 / 3.0) * assign94370_body25_e144983) + (assign94370_body25_e144972 * (-(((locals.var_chib_dn2 / 4.0) * assign94370_body25_e144981) + (assign94370_body25_e144976 * (-(locals.var_chib_dn2 / 5.0)))))))))), (((((locals.var_chib_dn4 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn4)) / 2.0) * assign94370_body25_e144985) + (assign94370_body25_e144968 * (-(((locals.var_chib_dn4 / 3.0) * assign94370_body25_e144983) + (assign94370_body25_e144972 * (-(((locals.var_chib_dn4 / 4.0) * assign94370_body25_e144981) + (assign94370_body25_e144976 * (-(locals.var_chib_dn4 / 5.0)))))))))), (((((locals.var_chib_dn5 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn5)) / 2.0) * assign94370_body25_e144985) + (assign94370_body25_e144968 * (-(((locals.var_chib_dn5 / 3.0) * assign94370_body25_e144983) + (assign94370_body25_e144972 * (-(((locals.var_chib_dn5 / 4.0) * assign94370_body25_e144981) + (assign94370_body25_e144976 * (-(locals.var_chib_dn5 / 5.0)))))))))), (((((locals.var_chib_dn6 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn6)) / 2.0) * assign94370_body25_e144985) + (assign94370_body25_e144968 * (-(((locals.var_chib_dn6 / 3.0) * assign94370_body25_e144983) + (assign94370_body25_e144972 * (-(((locals.var_chib_dn6 / 4.0) * assign94370_body25_e144981) + (assign94370_body25_e144976 * (-(locals.var_chib_dn6 / 5.0)))))))))), (((((locals.var_chib_dn7 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn7)) / 2.0) * assign94370_body25_e144985) + (assign94370_body25_e144968 * (-(((locals.var_chib_dn7 / 3.0) * assign94370_body25_e144983) + (assign94370_body25_e144972 * (-(((locals.var_chib_dn7 / 4.0) * assign94370_body25_e144981) + (assign94370_body25_e144976 * (-(locals.var_chib_dn7 / 5.0)))))))))), (((((locals.var_chib_dn8 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn8)) / 2.0) * assign94370_body25_e144985) + (assign94370_body25_e144968 * (-(((locals.var_chib_dn8 / 3.0) * assign94370_body25_e144983) + (assign94370_body25_e144972 * (-(((locals.var_chib_dn8 / 4.0) * assign94370_body25_e144981) + (assign94370_body25_e144976 * (-(locals.var_chib_dn8 / 5.0)))))))))), (((((locals.var_chib_dn9 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn9)) / 2.0) * assign94370_body25_e144985) + (assign94370_body25_e144968 * (-(((locals.var_chib_dn9 / 3.0) * assign94370_body25_e144983) + (assign94370_body25_e144972 * (-(((locals.var_chib_dn9 / 4.0) * assign94370_body25_e144981) + (assign94370_body25_e144976 * (-(locals.var_chib_dn9 / 5.0)))))))))), (((((locals.var_chib_dn10 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn10)) / 2.0) * assign94370_body25_e144985) + (assign94370_body25_e144968 * (-(((locals.var_chib_dn10 / 3.0) * assign94370_body25_e144983) + (assign94370_body25_e144972 * (-(((locals.var_chib_dn10 / 4.0) * assign94370_body25_e144981) + (assign94370_body25_e144976 * (-(locals.var_chib_dn10 / 5.0)))))))))), (((((locals.var_chib_dn11 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn11)) / 2.0) * assign94370_body25_e144985) + (assign94370_body25_e144968 * (-(((locals.var_chib_dn11 / 3.0) * assign94370_body25_e144983) + (assign94370_body25_e144972 * (-(((locals.var_chib_dn11 / 4.0) * assign94370_body25_e144981) + (assign94370_body25_e144976 * (-(locals.var_chib_dn11 / 5.0)))))))))), (((((locals.var_chib_dn14 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn14)) / 2.0) * assign94370_body25_e144985) + (assign94370_body25_e144968 * (-(((locals.var_chib_dn14 / 3.0) * assign94370_body25_e144983) + (assign94370_body25_e144972 * (-(((locals.var_chib_dn14 / 4.0) * assign94370_body25_e144981) + (assign94370_body25_e144976 * (-(locals.var_chib_dn14 / 5.0)))))))))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
            locals.var_t2 = assign94370_body25_e144988;
            locals.var_t2_dn0 = assign94370_body25_e144988_d_n0;
            locals.var_t2_dn2 = assign94370_body25_e144988_d_n2;
            locals.var_t2_dn4 = assign94370_body25_e144988_d_n4;
            locals.var_t2_dn5 = assign94370_body25_e144988_d_n5;
            locals.var_t2_dn6 = assign94370_body25_e144988_d_n6;
            locals.var_t2_dn7 = assign94370_body25_e144988_d_n7;
            locals.var_t2_dn8 = assign94370_body25_e144988_d_n8;
            locals.var_t2_dn9 = assign94370_body25_e144988_d_n9;
            locals.var_t2_dn10 = assign94370_body25_e144988_d_n10;
            locals.var_t2_dn11 = assign94370_body25_e144988_d_n11;
            locals.var_t2_dn14 = assign94370_body25_e144988_d_n14;
            locals.var_t2_rv = 0.0;
            let (assign94370_body26_e145022, assign94370_body26_e145022_d_n0, assign94370_body26_e145022_d_n2, assign94370_body26_e145022_d_n4, assign94370_body26_e145022_d_n5, assign94370_body26_e145022_d_n6, assign94370_body26_e145022_d_n7, assign94370_body26_e145022_d_n8, assign94370_body26_e145022_d_n9, assign94370_body26_e145022_d_n10, assign94370_body26_e145022_d_n11, assign94370_body26_e145022_d_n14,) = {
    if ((((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2182 != 0.0)) && (locals.var_guard2188 == 0.0)) && (locals.var_guard2189 != 0.0)) {
        let assign94370_body26_e145006: f64 = (locals.var_chib / 2.0);
        let assign94370_body26_e145010: f64 = (locals.var_chib / 3.0);
        let assign94370_body26_e145014: f64 = (locals.var_chib / 4.0);
        let assign94370_body26_e145015: f64 = (1.0 - assign94370_body26_e145014);
        let assign94370_body26_e145016: f64 = (assign94370_body26_e145010 * assign94370_body26_e145015);
        let assign94370_body26_e145017: f64 = (1.0 - assign94370_body26_e145016);
        let assign94370_body26_e145018: f64 = (assign94370_body26_e145006 * assign94370_body26_e145017);
        let assign94370_body26_e145019: f64 = (1.0 - assign94370_body26_e145018);
        let assign94370_body26_e145020: f64 = (locals.var_chib * assign94370_body26_e145019);
        (assign94370_body26_e145020, ((locals.var_chib_dn0 * assign94370_body26_e145019) + (locals.var_chib * (-(((locals.var_chib_dn0 / 2.0) * assign94370_body26_e145017) + (assign94370_body26_e145006 * (-(((locals.var_chib_dn0 / 3.0) * assign94370_body26_e145015) + (assign94370_body26_e145010 * (-(locals.var_chib_dn0 / 4.0)))))))))), ((locals.var_chib_dn2 * assign94370_body26_e145019) + (locals.var_chib * (-(((locals.var_chib_dn2 / 2.0) * assign94370_body26_e145017) + (assign94370_body26_e145006 * (-(((locals.var_chib_dn2 / 3.0) * assign94370_body26_e145015) + (assign94370_body26_e145010 * (-(locals.var_chib_dn2 / 4.0)))))))))), ((locals.var_chib_dn4 * assign94370_body26_e145019) + (locals.var_chib * (-(((locals.var_chib_dn4 / 2.0) * assign94370_body26_e145017) + (assign94370_body26_e145006 * (-(((locals.var_chib_dn4 / 3.0) * assign94370_body26_e145015) + (assign94370_body26_e145010 * (-(locals.var_chib_dn4 / 4.0)))))))))), ((locals.var_chib_dn5 * assign94370_body26_e145019) + (locals.var_chib * (-(((locals.var_chib_dn5 / 2.0) * assign94370_body26_e145017) + (assign94370_body26_e145006 * (-(((locals.var_chib_dn5 / 3.0) * assign94370_body26_e145015) + (assign94370_body26_e145010 * (-(locals.var_chib_dn5 / 4.0)))))))))), ((locals.var_chib_dn6 * assign94370_body26_e145019) + (locals.var_chib * (-(((locals.var_chib_dn6 / 2.0) * assign94370_body26_e145017) + (assign94370_body26_e145006 * (-(((locals.var_chib_dn6 / 3.0) * assign94370_body26_e145015) + (assign94370_body26_e145010 * (-(locals.var_chib_dn6 / 4.0)))))))))), ((locals.var_chib_dn7 * assign94370_body26_e145019) + (locals.var_chib * (-(((locals.var_chib_dn7 / 2.0) * assign94370_body26_e145017) + (assign94370_body26_e145006 * (-(((locals.var_chib_dn7 / 3.0) * assign94370_body26_e145015) + (assign94370_body26_e145010 * (-(locals.var_chib_dn7 / 4.0)))))))))), ((locals.var_chib_dn8 * assign94370_body26_e145019) + (locals.var_chib * (-(((locals.var_chib_dn8 / 2.0) * assign94370_body26_e145017) + (assign94370_body26_e145006 * (-(((locals.var_chib_dn8 / 3.0) * assign94370_body26_e145015) + (assign94370_body26_e145010 * (-(locals.var_chib_dn8 / 4.0)))))))))), ((locals.var_chib_dn9 * assign94370_body26_e145019) + (locals.var_chib * (-(((locals.var_chib_dn9 / 2.0) * assign94370_body26_e145017) + (assign94370_body26_e145006 * (-(((locals.var_chib_dn9 / 3.0) * assign94370_body26_e145015) + (assign94370_body26_e145010 * (-(locals.var_chib_dn9 / 4.0)))))))))), ((locals.var_chib_dn10 * assign94370_body26_e145019) + (locals.var_chib * (-(((locals.var_chib_dn10 / 2.0) * assign94370_body26_e145017) + (assign94370_body26_e145006 * (-(((locals.var_chib_dn10 / 3.0) * assign94370_body26_e145015) + (assign94370_body26_e145010 * (-(locals.var_chib_dn10 / 4.0)))))))))), ((locals.var_chib_dn11 * assign94370_body26_e145019) + (locals.var_chib * (-(((locals.var_chib_dn11 / 2.0) * assign94370_body26_e145017) + (assign94370_body26_e145006 * (-(((locals.var_chib_dn11 / 3.0) * assign94370_body26_e145015) + (assign94370_body26_e145010 * (-(locals.var_chib_dn11 / 4.0)))))))))), ((locals.var_chib_dn14 * assign94370_body26_e145019) + (locals.var_chib * (-(((locals.var_chib_dn14 / 2.0) * assign94370_body26_e145017) + (assign94370_body26_e145006 * (-(((locals.var_chib_dn14 / 3.0) * assign94370_body26_e145015) + (assign94370_body26_e145010 * (-(locals.var_chib_dn14 / 4.0)))))))))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
            locals.var_t3 = assign94370_body26_e145022;
            locals.var_t3_dn0 = assign94370_body26_e145022_d_n0;
            locals.var_t3_dn2 = assign94370_body26_e145022_d_n2;
            locals.var_t3_dn4 = assign94370_body26_e145022_d_n4;
            locals.var_t3_dn5 = assign94370_body26_e145022_d_n5;
            locals.var_t3_dn6 = assign94370_body26_e145022_d_n6;
            locals.var_t3_dn7 = assign94370_body26_e145022_d_n7;
            locals.var_t3_dn8 = assign94370_body26_e145022_d_n8;
            locals.var_t3_dn9 = assign94370_body26_e145022_d_n9;
            locals.var_t3_dn10 = assign94370_body26_e145022_d_n10;
            locals.var_t3_dn11 = assign94370_body26_e145022_d_n11;
            locals.var_t3_dn14 = assign94370_body26_e145022_d_n14;
            locals.var_t3_rv = 0.0;
            let (assign94370_body27_e145040, assign94370_body27_e145040_d_n0, assign94370_body27_e145040_d_n2, assign94370_body27_e145040_d_n4, assign94370_body27_e145040_d_n5, assign94370_body27_e145040_d_n6, assign94370_body27_e145040_d_n7, assign94370_body27_e145040_d_n8, assign94370_body27_e145040_d_n9, assign94370_body27_e145040_d_n10, assign94370_body27_e145040_d_n11, assign94370_body27_e145040_d_n14,) = {
    if ((((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2182 != 0.0)) && (locals.var_guard2188 == 0.0)) && (locals.var_guard2189 != 0.0)) {
        let assign94370_body27_e145038: f64 = (locals.var_t0 - locals.var_t2);
        (assign94370_body27_e145038, (locals.var_t0_dn0 - locals.var_t2_dn0), (locals.var_t0_dn2 - locals.var_t2_dn2), (locals.var_t0_dn4 - locals.var_t2_dn4), (locals.var_t0_dn5 - locals.var_t2_dn5), (locals.var_t0_dn6 - locals.var_t2_dn6), (locals.var_t0_dn7 - locals.var_t2_dn7), (locals.var_t0_dn8 - locals.var_t2_dn8), (locals.var_t0_dn9 - locals.var_t2_dn9), (locals.var_t0_dn10 - locals.var_t2_dn10), (locals.var_t0_dn11 - locals.var_t2_dn11), (locals.var_t0_dn14 - locals.var_t2_dn14),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
            locals.var_t4 = assign94370_body27_e145040;
            locals.var_t4_dn0 = assign94370_body27_e145040_d_n0;
            locals.var_t4_dn2 = assign94370_body27_e145040_d_n2;
            locals.var_t4_dn4 = assign94370_body27_e145040_d_n4;
            locals.var_t4_dn5 = assign94370_body27_e145040_d_n5;
            locals.var_t4_dn6 = assign94370_body27_e145040_d_n6;
            locals.var_t4_dn7 = assign94370_body27_e145040_d_n7;
            locals.var_t4_dn8 = assign94370_body27_e145040_d_n8;
            locals.var_t4_dn9 = assign94370_body27_e145040_d_n9;
            locals.var_t4_dn10 = assign94370_body27_e145040_d_n10;
            locals.var_t4_dn11 = assign94370_body27_e145040_d_n11;
            locals.var_t4_dn14 = assign94370_body27_e145040_d_n14;
            locals.var_t4_rv = 0.0;
            let assign94370_body28_e145043: f64 = if locals.var_t4 > 0.0 { 1.0 } else { 0.0 };
            locals.var_guard2190 = assign94370_body28_e145043;
            locals.var_guard2190_rv = 0.0;
            let (assign94370_body29_e145062, assign94370_body29_e145062_d_n0, assign94370_body29_e145062_d_n2, assign94370_body29_e145062_d_n4, assign94370_body29_e145062_d_n5, assign94370_body29_e145062_d_n6, assign94370_body29_e145062_d_n7, assign94370_body29_e145062_d_n8, assign94370_body29_e145062_d_n9, assign94370_body29_e145062_d_n10, assign94370_body29_e145062_d_n11, assign94370_body29_e145062_d_n14,) = {
    if (((((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2182 != 0.0)) && (locals.var_guard2188 == 0.0)) && (locals.var_guard2189 != 0.0)) && (locals.var_guard2190 != 0.0)) {
        let assign94370_body29_e145060: f64 = (locals.var_t4).sqrt();
        (assign94370_body29_e145060, (locals.var_t4_dn0 / (2.0 * assign94370_body29_e145060)), (locals.var_t4_dn2 / (2.0 * assign94370_body29_e145060)), (locals.var_t4_dn4 / (2.0 * assign94370_body29_e145060)), (locals.var_t4_dn5 / (2.0 * assign94370_body29_e145060)), (locals.var_t4_dn6 / (2.0 * assign94370_body29_e145060)), (locals.var_t4_dn7 / (2.0 * assign94370_body29_e145060)), (locals.var_t4_dn8 / (2.0 * assign94370_body29_e145060)), (locals.var_t4_dn9 / (2.0 * assign94370_body29_e145060)), (locals.var_t4_dn10 / (2.0 * assign94370_body29_e145060)), (locals.var_t4_dn11 / (2.0 * assign94370_body29_e145060)), (locals.var_t4_dn14 / (2.0 * assign94370_body29_e145060)),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn14,)
    }
};
            locals.var_fb = assign94370_body29_e145062;
            locals.var_fb_dn0 = assign94370_body29_e145062_d_n0;
            locals.var_fb_dn2 = assign94370_body29_e145062_d_n2;
            locals.var_fb_dn4 = assign94370_body29_e145062_d_n4;
            locals.var_fb_dn5 = assign94370_body29_e145062_d_n5;
            locals.var_fb_dn6 = assign94370_body29_e145062_d_n6;
            locals.var_fb_dn7 = assign94370_body29_e145062_d_n7;
            locals.var_fb_dn8 = assign94370_body29_e145062_d_n8;
            locals.var_fb_dn9 = assign94370_body29_e145062_d_n9;
            locals.var_fb_dn10 = assign94370_body29_e145062_d_n10;
            locals.var_fb_dn11 = assign94370_body29_e145062_d_n11;
            locals.var_fb_dn14 = assign94370_body29_e145062_d_n14;
            locals.var_fb_rv = 0.0;
            let (assign94370_body30_e145090, assign94370_body30_e145090_d_n0, assign94370_body30_e145090_d_n2, assign94370_body30_e145090_d_n4, assign94370_body30_e145090_d_n5, assign94370_body30_e145090_d_n6, assign94370_body30_e145090_d_n7, assign94370_body30_e145090_d_n8, assign94370_body30_e145090_d_n9, assign94370_body30_e145090_d_n10, assign94370_body30_e145090_d_n11, assign94370_body30_e145090_d_n14,) = {
    if (((((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2182 != 0.0)) && (locals.var_guard2188 == 0.0)) && (locals.var_guard2189 != 0.0)) && (locals.var_guard2190 != 0.0)) {
        let assign94370_body30_e145080: f64 = (locals.var_beta * 0.5);
        let assign94370_body30_e145084: f64 = (locals.var_phi_b_dpss * locals.var_t3);
        let assign94370_body30_e145085: f64 = (locals.var_t1 - assign94370_body30_e145084);
        let assign94370_body30_e145086: f64 = (assign94370_body30_e145080 * assign94370_body30_e145085);
        let assign94370_body30_e145088: f64 = (assign94370_body30_e145086 / locals.var_fb);
        (assign94370_body30_e145088, ((((((locals.var_beta_dn0 * 0.5) * assign94370_body30_e145085) + (assign94370_body30_e145080 * (locals.var_t1_dn0 - ((locals.var_phi_b_dpss_dn0 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn0))))) * locals.var_fb) - (assign94370_body30_e145086 * locals.var_fb_dn0)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn2 * 0.5) * assign94370_body30_e145085) + (assign94370_body30_e145080 * (locals.var_t1_dn2 - ((locals.var_phi_b_dpss_dn2 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn2))))) * locals.var_fb) - (assign94370_body30_e145086 * locals.var_fb_dn2)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn4 * 0.5) * assign94370_body30_e145085) + (assign94370_body30_e145080 * (locals.var_t1_dn4 - ((locals.var_phi_b_dpss_dn4 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn4))))) * locals.var_fb) - (assign94370_body30_e145086 * locals.var_fb_dn4)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn5 * 0.5) * assign94370_body30_e145085) + (assign94370_body30_e145080 * (locals.var_t1_dn5 - ((locals.var_phi_b_dpss_dn5 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn5))))) * locals.var_fb) - (assign94370_body30_e145086 * locals.var_fb_dn5)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn6 * 0.5) * assign94370_body30_e145085) + (assign94370_body30_e145080 * (locals.var_t1_dn6 - ((locals.var_phi_b_dpss_dn6 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn6))))) * locals.var_fb) - (assign94370_body30_e145086 * locals.var_fb_dn6)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn7 * 0.5) * assign94370_body30_e145085) + (assign94370_body30_e145080 * (locals.var_t1_dn7 - ((locals.var_phi_b_dpss_dn7 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn7))))) * locals.var_fb) - (assign94370_body30_e145086 * locals.var_fb_dn7)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn8 * 0.5) * assign94370_body30_e145085) + (assign94370_body30_e145080 * (locals.var_t1_dn8 - ((locals.var_phi_b_dpss_dn8 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn8))))) * locals.var_fb) - (assign94370_body30_e145086 * locals.var_fb_dn8)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn9 * 0.5) * assign94370_body30_e145085) + (assign94370_body30_e145080 * (locals.var_t1_dn9 - ((locals.var_phi_b_dpss_dn9 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn9))))) * locals.var_fb) - (assign94370_body30_e145086 * locals.var_fb_dn9)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn10 * 0.5) * assign94370_body30_e145085) + (assign94370_body30_e145080 * (locals.var_t1_dn10 - ((locals.var_phi_b_dpss_dn10 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn10))))) * locals.var_fb) - (assign94370_body30_e145086 * locals.var_fb_dn10)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn11 * 0.5) * assign94370_body30_e145085) + (assign94370_body30_e145080 * (locals.var_t1_dn11 - ((locals.var_phi_b_dpss_dn11 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn11))))) * locals.var_fb) - (assign94370_body30_e145086 * locals.var_fb_dn11)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn14 * 0.5) * assign94370_body30_e145085) + (assign94370_body30_e145080 * (locals.var_t1_dn14 - ((locals.var_phi_b_dpss_dn14 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn14))))) * locals.var_fb) - (assign94370_body30_e145086 * locals.var_fb_dn14)) / (locals.var_fb * locals.var_fb)),)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn14,)
    }
};
            locals.var_fb_dpss = assign94370_body30_e145090;
            locals.var_fb_dpss_dn0 = assign94370_body30_e145090_d_n0;
            locals.var_fb_dpss_dn2 = assign94370_body30_e145090_d_n2;
            locals.var_fb_dpss_dn4 = assign94370_body30_e145090_d_n4;
            locals.var_fb_dpss_dn5 = assign94370_body30_e145090_d_n5;
            locals.var_fb_dpss_dn6 = assign94370_body30_e145090_d_n6;
            locals.var_fb_dpss_dn7 = assign94370_body30_e145090_d_n7;
            locals.var_fb_dpss_dn8 = assign94370_body30_e145090_d_n8;
            locals.var_fb_dpss_dn9 = assign94370_body30_e145090_d_n9;
            locals.var_fb_dpss_dn10 = assign94370_body30_e145090_d_n10;
            locals.var_fb_dpss_dn11 = assign94370_body30_e145090_d_n11;
            locals.var_fb_dpss_dn14 = assign94370_body30_e145090_d_n14;
            locals.var_fb_dpss_rv = 0.0;
            let (assign94370_body32_e145130, assign94370_body32_e145130_d_n0, assign94370_body32_e145130_d_n2, assign94370_body32_e145130_d_n4, assign94370_body32_e145130_d_n5, assign94370_body32_e145130_d_n6, assign94370_body32_e145130_d_n7, assign94370_body32_e145130_d_n8, assign94370_body32_e145130_d_n9, assign94370_body32_e145130_d_n10, assign94370_body32_e145130_d_n11, assign94370_body32_e145130_d_n14,) = {
    if (((((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2182 != 0.0)) && (locals.var_guard2188 == 0.0)) && (locals.var_guard2189 != 0.0)) && (locals.var_guard2190 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn14,)
    }
};
            locals.var_fb = assign94370_body32_e145130;
            locals.var_fb_dn0 = assign94370_body32_e145130_d_n0;
            locals.var_fb_dn2 = assign94370_body32_e145130_d_n2;
            locals.var_fb_dn4 = assign94370_body32_e145130_d_n4;
            locals.var_fb_dn5 = assign94370_body32_e145130_d_n5;
            locals.var_fb_dn6 = assign94370_body32_e145130_d_n6;
            locals.var_fb_dn7 = assign94370_body32_e145130_d_n7;
            locals.var_fb_dn8 = assign94370_body32_e145130_d_n8;
            locals.var_fb_dn9 = assign94370_body32_e145130_d_n9;
            locals.var_fb_dn10 = assign94370_body32_e145130_d_n10;
            locals.var_fb_dn11 = assign94370_body32_e145130_d_n11;
            locals.var_fb_dn14 = assign94370_body32_e145130_d_n14;
            locals.var_fb_rv = 0.0;
            let (assign94370_body33_e145149, assign94370_body33_e145149_d_n0, assign94370_body33_e145149_d_n2, assign94370_body33_e145149_d_n4, assign94370_body33_e145149_d_n5, assign94370_body33_e145149_d_n6, assign94370_body33_e145149_d_n7, assign94370_body33_e145149_d_n8, assign94370_body33_e145149_d_n9, assign94370_body33_e145149_d_n10, assign94370_body33_e145149_d_n11, assign94370_body33_e145149_d_n14,) = {
    if (((((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2182 != 0.0)) && (locals.var_guard2188 == 0.0)) && (locals.var_guard2189 != 0.0)) && (locals.var_guard2190 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn14,)
    }
};
            locals.var_fb_dpss = assign94370_body33_e145149;
            locals.var_fb_dpss_dn0 = assign94370_body33_e145149_d_n0;
            locals.var_fb_dpss_dn2 = assign94370_body33_e145149_d_n2;
            locals.var_fb_dpss_dn4 = assign94370_body33_e145149_d_n4;
            locals.var_fb_dpss_dn5 = assign94370_body33_e145149_d_n5;
            locals.var_fb_dpss_dn6 = assign94370_body33_e145149_d_n6;
            locals.var_fb_dpss_dn7 = assign94370_body33_e145149_d_n7;
            locals.var_fb_dpss_dn8 = assign94370_body33_e145149_d_n8;
            locals.var_fb_dpss_dn9 = assign94370_body33_e145149_d_n9;
            locals.var_fb_dpss_dn10 = assign94370_body33_e145149_d_n10;
            locals.var_fb_dpss_dn11 = assign94370_body33_e145149_d_n11;
            locals.var_fb_dpss_dn14 = assign94370_body33_e145149_d_n14;
            locals.var_fb_dpss_rv = 0.0;
            let (assign94370_body34_e145168, assign94370_body34_e145168_d_n0, assign94370_body34_e145168_d_n2, assign94370_body34_e145168_d_n4, assign94370_body34_e145168_d_n5, assign94370_body34_e145168_d_n6, assign94370_body34_e145168_d_n7, assign94370_body34_e145168_d_n8, assign94370_body34_e145168_d_n9, assign94370_body34_e145168_d_n10, assign94370_body34_e145168_d_n11, assign94370_body34_e145168_d_n14,) = {
    if ((((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2182 != 0.0)) && (locals.var_guard2188 == 0.0)) && (locals.var_guard2189 == 0.0)) {
        let assign94370_body34_e145165: f64 = (-locals.var_chi);
        let assign94370_body34_e145166: f64 = (assign94370_body34_e145165).exp();
        (assign94370_body34_e145166, (assign94370_body34_e145166 * (-locals.var_chi_dn0)), (assign94370_body34_e145166 * (-locals.var_chi_dn2)), (assign94370_body34_e145166 * (-locals.var_chi_dn4)), (assign94370_body34_e145166 * (-locals.var_chi_dn5)), (assign94370_body34_e145166 * (-locals.var_chi_dn6)), (assign94370_body34_e145166 * (-locals.var_chi_dn7)), (assign94370_body34_e145166 * (-locals.var_chi_dn8)), (assign94370_body34_e145166 * (-locals.var_chi_dn9)), (assign94370_body34_e145166 * (-locals.var_chi_dn10)), (assign94370_body34_e145166 * (-locals.var_chi_dn11)), (assign94370_body34_e145166 * (-locals.var_chi_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
            locals.var_t0 = assign94370_body34_e145168;
            locals.var_t0_dn0 = assign94370_body34_e145168_d_n0;
            locals.var_t0_dn2 = assign94370_body34_e145168_d_n2;
            locals.var_t0_dn4 = assign94370_body34_e145168_d_n4;
            locals.var_t0_dn5 = assign94370_body34_e145168_d_n5;
            locals.var_t0_dn6 = assign94370_body34_e145168_d_n6;
            locals.var_t0_dn7 = assign94370_body34_e145168_d_n7;
            locals.var_t0_dn8 = assign94370_body34_e145168_d_n8;
            locals.var_t0_dn9 = assign94370_body34_e145168_d_n9;
            locals.var_t0_dn10 = assign94370_body34_e145168_d_n10;
            locals.var_t0_dn11 = assign94370_body34_e145168_d_n11;
            locals.var_t0_dn14 = assign94370_body34_e145168_d_n14;
            locals.var_t0_rv = 0.0;
            let (assign94370_body35_e145187, assign94370_body35_e145187_d_n0, assign94370_body35_e145187_d_n2, assign94370_body35_e145187_d_n4, assign94370_body35_e145187_d_n5, assign94370_body35_e145187_d_n6, assign94370_body35_e145187_d_n7, assign94370_body35_e145187_d_n8, assign94370_body35_e145187_d_n9, assign94370_body35_e145187_d_n10, assign94370_body35_e145187_d_n11, assign94370_body35_e145187_d_n14,) = {
    if ((((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2182 != 0.0)) && (locals.var_guard2188 == 0.0)) && (locals.var_guard2189 == 0.0)) {
        let assign94370_body35_e145184: f64 = (-locals.var_chib);
        let assign94370_body35_e145185: f64 = (assign94370_body35_e145184).exp();
        (assign94370_body35_e145185, (assign94370_body35_e145185 * (-locals.var_chib_dn0)), (assign94370_body35_e145185 * (-locals.var_chib_dn2)), (assign94370_body35_e145185 * (-locals.var_chib_dn4)), (assign94370_body35_e145185 * (-locals.var_chib_dn5)), (assign94370_body35_e145185 * (-locals.var_chib_dn6)), (assign94370_body35_e145185 * (-locals.var_chib_dn7)), (assign94370_body35_e145185 * (-locals.var_chib_dn8)), (assign94370_body35_e145185 * (-locals.var_chib_dn9)), (assign94370_body35_e145185 * (-locals.var_chib_dn10)), (assign94370_body35_e145185 * (-locals.var_chib_dn11)), (assign94370_body35_e145185 * (-locals.var_chib_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
            locals.var_t1 = assign94370_body35_e145187;
            locals.var_t1_dn0 = assign94370_body35_e145187_d_n0;
            locals.var_t1_dn2 = assign94370_body35_e145187_d_n2;
            locals.var_t1_dn4 = assign94370_body35_e145187_d_n4;
            locals.var_t1_dn5 = assign94370_body35_e145187_d_n5;
            locals.var_t1_dn6 = assign94370_body35_e145187_d_n6;
            locals.var_t1_dn7 = assign94370_body35_e145187_d_n7;
            locals.var_t1_dn8 = assign94370_body35_e145187_d_n8;
            locals.var_t1_dn9 = assign94370_body35_e145187_d_n9;
            locals.var_t1_dn10 = assign94370_body35_e145187_d_n10;
            locals.var_t1_dn11 = assign94370_body35_e145187_d_n11;
            locals.var_t1_dn14 = assign94370_body35_e145187_d_n14;
            locals.var_t1_rv = 0.0;
            let (assign94370_body36_e145210, assign94370_body36_e145210_d_n0, assign94370_body36_e145210_d_n2, assign94370_body36_e145210_d_n4, assign94370_body36_e145210_d_n5, assign94370_body36_e145210_d_n6, assign94370_body36_e145210_d_n7, assign94370_body36_e145210_d_n8, assign94370_body36_e145210_d_n9, assign94370_body36_e145210_d_n10, assign94370_body36_e145210_d_n11, assign94370_body36_e145210_d_n14,) = {
    if ((((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2182 != 0.0)) && (locals.var_guard2188 == 0.0)) && (locals.var_guard2189 == 0.0)) {
        let assign94370_body36_e145204: f64 = (locals.var_chi - locals.var_chib);
        let assign94370_body36_e145207: f64 = (locals.var_t0 - locals.var_t1);
        let assign94370_body36_e145208: f64 = (assign94370_body36_e145204 + assign94370_body36_e145207);
        (assign94370_body36_e145208, ((locals.var_chi_dn0 - locals.var_chib_dn0) + (locals.var_t0_dn0 - locals.var_t1_dn0)), ((locals.var_chi_dn2 - locals.var_chib_dn2) + (locals.var_t0_dn2 - locals.var_t1_dn2)), ((locals.var_chi_dn4 - locals.var_chib_dn4) + (locals.var_t0_dn4 - locals.var_t1_dn4)), ((locals.var_chi_dn5 - locals.var_chib_dn5) + (locals.var_t0_dn5 - locals.var_t1_dn5)), ((locals.var_chi_dn6 - locals.var_chib_dn6) + (locals.var_t0_dn6 - locals.var_t1_dn6)), ((locals.var_chi_dn7 - locals.var_chib_dn7) + (locals.var_t0_dn7 - locals.var_t1_dn7)), ((locals.var_chi_dn8 - locals.var_chib_dn8) + (locals.var_t0_dn8 - locals.var_t1_dn8)), ((locals.var_chi_dn9 - locals.var_chib_dn9) + (locals.var_t0_dn9 - locals.var_t1_dn9)), ((locals.var_chi_dn10 - locals.var_chib_dn10) + (locals.var_t0_dn10 - locals.var_t1_dn10)), ((locals.var_chi_dn11 - locals.var_chib_dn11) + (locals.var_t0_dn11 - locals.var_t1_dn11)), ((locals.var_chi_dn14 - locals.var_chib_dn14) + (locals.var_t0_dn14 - locals.var_t1_dn14)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
            locals.var_t4 = assign94370_body36_e145210;
            locals.var_t4_dn0 = assign94370_body36_e145210_d_n0;
            locals.var_t4_dn2 = assign94370_body36_e145210_d_n2;
            locals.var_t4_dn4 = assign94370_body36_e145210_d_n4;
            locals.var_t4_dn5 = assign94370_body36_e145210_d_n5;
            locals.var_t4_dn6 = assign94370_body36_e145210_d_n6;
            locals.var_t4_dn7 = assign94370_body36_e145210_d_n7;
            locals.var_t4_dn8 = assign94370_body36_e145210_d_n8;
            locals.var_t4_dn9 = assign94370_body36_e145210_d_n9;
            locals.var_t4_dn10 = assign94370_body36_e145210_d_n10;
            locals.var_t4_dn11 = assign94370_body36_e145210_d_n11;
            locals.var_t4_dn14 = assign94370_body36_e145210_d_n14;
            locals.var_t4_rv = 0.0;
            let assign94370_body37_e145213: f64 = if locals.var_t4 > 0.0 { 1.0 } else { 0.0 };
            locals.var_guard2191 = assign94370_body37_e145213;
            locals.var_guard2191_rv = 0.0;
            let (assign94370_body38_e145233, assign94370_body38_e145233_d_n0, assign94370_body38_e145233_d_n2, assign94370_body38_e145233_d_n4, assign94370_body38_e145233_d_n5, assign94370_body38_e145233_d_n6, assign94370_body38_e145233_d_n7, assign94370_body38_e145233_d_n8, assign94370_body38_e145233_d_n9, assign94370_body38_e145233_d_n10, assign94370_body38_e145233_d_n11, assign94370_body38_e145233_d_n14,) = {
    if (((((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2182 != 0.0)) && (locals.var_guard2188 == 0.0)) && (locals.var_guard2189 == 0.0)) && (locals.var_guard2191 != 0.0)) {
        let assign94370_body38_e145231: f64 = (locals.var_t4).sqrt();
        (assign94370_body38_e145231, (locals.var_t4_dn0 / (2.0 * assign94370_body38_e145231)), (locals.var_t4_dn2 / (2.0 * assign94370_body38_e145231)), (locals.var_t4_dn4 / (2.0 * assign94370_body38_e145231)), (locals.var_t4_dn5 / (2.0 * assign94370_body38_e145231)), (locals.var_t4_dn6 / (2.0 * assign94370_body38_e145231)), (locals.var_t4_dn7 / (2.0 * assign94370_body38_e145231)), (locals.var_t4_dn8 / (2.0 * assign94370_body38_e145231)), (locals.var_t4_dn9 / (2.0 * assign94370_body38_e145231)), (locals.var_t4_dn10 / (2.0 * assign94370_body38_e145231)), (locals.var_t4_dn11 / (2.0 * assign94370_body38_e145231)), (locals.var_t4_dn14 / (2.0 * assign94370_body38_e145231)),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn14,)
    }
};
            locals.var_fb = assign94370_body38_e145233;
            locals.var_fb_dn0 = assign94370_body38_e145233_d_n0;
            locals.var_fb_dn2 = assign94370_body38_e145233_d_n2;
            locals.var_fb_dn4 = assign94370_body38_e145233_d_n4;
            locals.var_fb_dn5 = assign94370_body38_e145233_d_n5;
            locals.var_fb_dn6 = assign94370_body38_e145233_d_n6;
            locals.var_fb_dn7 = assign94370_body38_e145233_d_n7;
            locals.var_fb_dn8 = assign94370_body38_e145233_d_n8;
            locals.var_fb_dn9 = assign94370_body38_e145233_d_n9;
            locals.var_fb_dn10 = assign94370_body38_e145233_d_n10;
            locals.var_fb_dn11 = assign94370_body38_e145233_d_n11;
            locals.var_fb_dn14 = assign94370_body38_e145233_d_n14;
            locals.var_fb_rv = 0.0;
            let (assign94370_body39_e145266, assign94370_body39_e145266_d_n0, assign94370_body39_e145266_d_n2, assign94370_body39_e145266_d_n4, assign94370_body39_e145266_d_n5, assign94370_body39_e145266_d_n6, assign94370_body39_e145266_d_n7, assign94370_body39_e145266_d_n8, assign94370_body39_e145266_d_n9, assign94370_body39_e145266_d_n10, assign94370_body39_e145266_d_n11, assign94370_body39_e145266_d_n14,) = {
    if (((((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2182 != 0.0)) && (locals.var_guard2188 == 0.0)) && (locals.var_guard2189 == 0.0)) && (locals.var_guard2191 != 0.0)) {
        let assign94370_body39_e145252: f64 = (locals.var_beta * 0.5);
        let assign94370_body39_e145255: f64 = (1.0 - locals.var_t0);
        let assign94370_body39_e145259: f64 = (1.0 - locals.var_t1);
        let assign94370_body39_e145260: f64 = (locals.var_phi_b_dpss * assign94370_body39_e145259);
        let assign94370_body39_e145261: f64 = (assign94370_body39_e145255 - assign94370_body39_e145260);
        let assign94370_body39_e145262: f64 = (assign94370_body39_e145252 * assign94370_body39_e145261);
        let assign94370_body39_e145264: f64 = (assign94370_body39_e145262 / locals.var_fb);
        (assign94370_body39_e145264, ((((((locals.var_beta_dn0 * 0.5) * assign94370_body39_e145261) + (assign94370_body39_e145252 * ((-locals.var_t0_dn0) - ((locals.var_phi_b_dpss_dn0 * assign94370_body39_e145259) + (locals.var_phi_b_dpss * (-locals.var_t1_dn0)))))) * locals.var_fb) - (assign94370_body39_e145262 * locals.var_fb_dn0)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn2 * 0.5) * assign94370_body39_e145261) + (assign94370_body39_e145252 * ((-locals.var_t0_dn2) - ((locals.var_phi_b_dpss_dn2 * assign94370_body39_e145259) + (locals.var_phi_b_dpss * (-locals.var_t1_dn2)))))) * locals.var_fb) - (assign94370_body39_e145262 * locals.var_fb_dn2)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn4 * 0.5) * assign94370_body39_e145261) + (assign94370_body39_e145252 * ((-locals.var_t0_dn4) - ((locals.var_phi_b_dpss_dn4 * assign94370_body39_e145259) + (locals.var_phi_b_dpss * (-locals.var_t1_dn4)))))) * locals.var_fb) - (assign94370_body39_e145262 * locals.var_fb_dn4)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn5 * 0.5) * assign94370_body39_e145261) + (assign94370_body39_e145252 * ((-locals.var_t0_dn5) - ((locals.var_phi_b_dpss_dn5 * assign94370_body39_e145259) + (locals.var_phi_b_dpss * (-locals.var_t1_dn5)))))) * locals.var_fb) - (assign94370_body39_e145262 * locals.var_fb_dn5)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn6 * 0.5) * assign94370_body39_e145261) + (assign94370_body39_e145252 * ((-locals.var_t0_dn6) - ((locals.var_phi_b_dpss_dn6 * assign94370_body39_e145259) + (locals.var_phi_b_dpss * (-locals.var_t1_dn6)))))) * locals.var_fb) - (assign94370_body39_e145262 * locals.var_fb_dn6)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn7 * 0.5) * assign94370_body39_e145261) + (assign94370_body39_e145252 * ((-locals.var_t0_dn7) - ((locals.var_phi_b_dpss_dn7 * assign94370_body39_e145259) + (locals.var_phi_b_dpss * (-locals.var_t1_dn7)))))) * locals.var_fb) - (assign94370_body39_e145262 * locals.var_fb_dn7)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn8 * 0.5) * assign94370_body39_e145261) + (assign94370_body39_e145252 * ((-locals.var_t0_dn8) - ((locals.var_phi_b_dpss_dn8 * assign94370_body39_e145259) + (locals.var_phi_b_dpss * (-locals.var_t1_dn8)))))) * locals.var_fb) - (assign94370_body39_e145262 * locals.var_fb_dn8)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn9 * 0.5) * assign94370_body39_e145261) + (assign94370_body39_e145252 * ((-locals.var_t0_dn9) - ((locals.var_phi_b_dpss_dn9 * assign94370_body39_e145259) + (locals.var_phi_b_dpss * (-locals.var_t1_dn9)))))) * locals.var_fb) - (assign94370_body39_e145262 * locals.var_fb_dn9)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn10 * 0.5) * assign94370_body39_e145261) + (assign94370_body39_e145252 * ((-locals.var_t0_dn10) - ((locals.var_phi_b_dpss_dn10 * assign94370_body39_e145259) + (locals.var_phi_b_dpss * (-locals.var_t1_dn10)))))) * locals.var_fb) - (assign94370_body39_e145262 * locals.var_fb_dn10)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn11 * 0.5) * assign94370_body39_e145261) + (assign94370_body39_e145252 * ((-locals.var_t0_dn11) - ((locals.var_phi_b_dpss_dn11 * assign94370_body39_e145259) + (locals.var_phi_b_dpss * (-locals.var_t1_dn11)))))) * locals.var_fb) - (assign94370_body39_e145262 * locals.var_fb_dn11)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn14 * 0.5) * assign94370_body39_e145261) + (assign94370_body39_e145252 * ((-locals.var_t0_dn14) - ((locals.var_phi_b_dpss_dn14 * assign94370_body39_e145259) + (locals.var_phi_b_dpss * (-locals.var_t1_dn14)))))) * locals.var_fb) - (assign94370_body39_e145262 * locals.var_fb_dn14)) / (locals.var_fb * locals.var_fb)),)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn14,)
    }
};
            locals.var_fb_dpss = assign94370_body39_e145266;
            locals.var_fb_dpss_dn0 = assign94370_body39_e145266_d_n0;
            locals.var_fb_dpss_dn2 = assign94370_body39_e145266_d_n2;
            locals.var_fb_dpss_dn4 = assign94370_body39_e145266_d_n4;
            locals.var_fb_dpss_dn5 = assign94370_body39_e145266_d_n5;
            locals.var_fb_dpss_dn6 = assign94370_body39_e145266_d_n6;
            locals.var_fb_dpss_dn7 = assign94370_body39_e145266_d_n7;
            locals.var_fb_dpss_dn8 = assign94370_body39_e145266_d_n8;
            locals.var_fb_dpss_dn9 = assign94370_body39_e145266_d_n9;
            locals.var_fb_dpss_dn10 = assign94370_body39_e145266_d_n10;
            locals.var_fb_dpss_dn11 = assign94370_body39_e145266_d_n11;
            locals.var_fb_dpss_dn14 = assign94370_body39_e145266_d_n14;
            locals.var_fb_dpss_rv = 0.0;
            let (assign94370_body41_e145308, assign94370_body41_e145308_d_n0, assign94370_body41_e145308_d_n2, assign94370_body41_e145308_d_n4, assign94370_body41_e145308_d_n5, assign94370_body41_e145308_d_n6, assign94370_body41_e145308_d_n7, assign94370_body41_e145308_d_n8, assign94370_body41_e145308_d_n9, assign94370_body41_e145308_d_n10, assign94370_body41_e145308_d_n11, assign94370_body41_e145308_d_n14,) = {
    if (((((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2182 != 0.0)) && (locals.var_guard2188 == 0.0)) && (locals.var_guard2189 == 0.0)) && (locals.var_guard2191 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn14,)
    }
};
            locals.var_fb = assign94370_body41_e145308;
            locals.var_fb_dn0 = assign94370_body41_e145308_d_n0;
            locals.var_fb_dn2 = assign94370_body41_e145308_d_n2;
            locals.var_fb_dn4 = assign94370_body41_e145308_d_n4;
            locals.var_fb_dn5 = assign94370_body41_e145308_d_n5;
            locals.var_fb_dn6 = assign94370_body41_e145308_d_n6;
            locals.var_fb_dn7 = assign94370_body41_e145308_d_n7;
            locals.var_fb_dn8 = assign94370_body41_e145308_d_n8;
            locals.var_fb_dn9 = assign94370_body41_e145308_d_n9;
            locals.var_fb_dn10 = assign94370_body41_e145308_d_n10;
            locals.var_fb_dn11 = assign94370_body41_e145308_d_n11;
            locals.var_fb_dn14 = assign94370_body41_e145308_d_n14;
            locals.var_fb_rv = 0.0;
            let (assign94370_body42_e145328, assign94370_body42_e145328_d_n0, assign94370_body42_e145328_d_n2, assign94370_body42_e145328_d_n4, assign94370_body42_e145328_d_n5, assign94370_body42_e145328_d_n6, assign94370_body42_e145328_d_n7, assign94370_body42_e145328_d_n8, assign94370_body42_e145328_d_n9, assign94370_body42_e145328_d_n10, assign94370_body42_e145328_d_n11, assign94370_body42_e145328_d_n14,) = {
    if (((((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2182 != 0.0)) && (locals.var_guard2188 == 0.0)) && (locals.var_guard2189 == 0.0)) && (locals.var_guard2191 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn14,)
    }
};
            locals.var_fb_dpss = assign94370_body42_e145328;
            locals.var_fb_dpss_dn0 = assign94370_body42_e145328_d_n0;
            locals.var_fb_dpss_dn2 = assign94370_body42_e145328_d_n2;
            locals.var_fb_dpss_dn4 = assign94370_body42_e145328_d_n4;
            locals.var_fb_dpss_dn5 = assign94370_body42_e145328_d_n5;
            locals.var_fb_dpss_dn6 = assign94370_body42_e145328_d_n6;
            locals.var_fb_dpss_dn7 = assign94370_body42_e145328_d_n7;
            locals.var_fb_dpss_dn8 = assign94370_body42_e145328_d_n8;
            locals.var_fb_dpss_dn9 = assign94370_body42_e145328_d_n9;
            locals.var_fb_dpss_dn10 = assign94370_body42_e145328_d_n10;
            locals.var_fb_dpss_dn11 = assign94370_body42_e145328_d_n11;
            locals.var_fb_dpss_dn14 = assign94370_body42_e145328_d_n14;
            locals.var_fb_dpss_rv = 0.0;
            let assign94370_body43_e145331: f64 = if locals.var_chi < 0.0 { 1.0 } else { 0.0 };
            locals.var_guard2192 = assign94370_body43_e145331;
            locals.var_guard2192_rv = 0.0;
            let (assign94370_body45_e145359, assign94370_body45_e145359_d_n0, assign94370_body45_e145359_d_n2, assign94370_body45_e145359_d_n4, assign94370_body45_e145359_d_n5, assign94370_body45_e145359_d_n6, assign94370_body45_e145359_d_n7, assign94370_body45_e145359_d_n8, assign94370_body45_e145359_d_n9, assign94370_body45_e145359_d_n10, assign94370_body45_e145359_d_n11, assign94370_body45_e145359_d_n14,) = {
    if (((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2182 != 0.0)) && (locals.var_guard2192 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn11, locals.var_fs01_dn14,)
    }
};
            locals.var_fs01 = assign94370_body45_e145359;
            locals.var_fs01_dn0 = assign94370_body45_e145359_d_n0;
            locals.var_fs01_dn2 = assign94370_body45_e145359_d_n2;
            locals.var_fs01_dn4 = assign94370_body45_e145359_d_n4;
            locals.var_fs01_dn5 = assign94370_body45_e145359_d_n5;
            locals.var_fs01_dn6 = assign94370_body45_e145359_d_n6;
            locals.var_fs01_dn7 = assign94370_body45_e145359_d_n7;
            locals.var_fs01_dn8 = assign94370_body45_e145359_d_n8;
            locals.var_fs01_dn9 = assign94370_body45_e145359_d_n9;
            locals.var_fs01_dn10 = assign94370_body45_e145359_d_n10;
            locals.var_fs01_dn11 = assign94370_body45_e145359_d_n11;
            locals.var_fs01_dn14 = assign94370_body45_e145359_d_n14;
            locals.var_fs01_rv = 0.0;
            let (assign94370_body46_e145372, assign94370_body46_e145372_d_n0, assign94370_body46_e145372_d_n2, assign94370_body46_e145372_d_n4, assign94370_body46_e145372_d_n5, assign94370_body46_e145372_d_n6, assign94370_body46_e145372_d_n7, assign94370_body46_e145372_d_n8, assign94370_body46_e145372_d_n9, assign94370_body46_e145372_d_n10, assign94370_body46_e145372_d_n11, assign94370_body46_e145372_d_n14,) = {
    if (((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2182 != 0.0)) && (locals.var_guard2192 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn11, locals.var_fs01_dps0_dn14,)
    }
};
            locals.var_fs01_dps0 = assign94370_body46_e145372;
            locals.var_fs01_dps0_dn0 = assign94370_body46_e145372_d_n0;
            locals.var_fs01_dps0_dn2 = assign94370_body46_e145372_d_n2;
            locals.var_fs01_dps0_dn4 = assign94370_body46_e145372_d_n4;
            locals.var_fs01_dps0_dn5 = assign94370_body46_e145372_d_n5;
            locals.var_fs01_dps0_dn6 = assign94370_body46_e145372_d_n6;
            locals.var_fs01_dps0_dn7 = assign94370_body46_e145372_d_n7;
            locals.var_fs01_dps0_dn8 = assign94370_body46_e145372_d_n8;
            locals.var_fs01_dps0_dn9 = assign94370_body46_e145372_d_n9;
            locals.var_fs01_dps0_dn10 = assign94370_body46_e145372_d_n10;
            locals.var_fs01_dps0_dn11 = assign94370_body46_e145372_d_n11;
            locals.var_fs01_dps0_dn14 = assign94370_body46_e145372_d_n14;
            locals.var_fs01_dps0_rv = 0.0;
            let (assign94370_body47_e145386, assign94370_body47_e145386_d_n0, assign94370_body47_e145386_d_n2, assign94370_body47_e145386_d_n4, assign94370_body47_e145386_d_n5, assign94370_body47_e145386_d_n6, assign94370_body47_e145386_d_n7, assign94370_body47_e145386_d_n8, assign94370_body47_e145386_d_n9, assign94370_body47_e145386_d_n10, assign94370_body47_e145386_d_n11, assign94370_body47_e145386_d_n14,) = {
    if (((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2182 != 0.0)) && (locals.var_guard2192 != 0.0)) {
        let assign94370_body47_e145384: f64 = (-locals.var_fb);
        (assign94370_body47_e145384, (-locals.var_fb_dn0), (-locals.var_fb_dn2), (-locals.var_fb_dn4), (-locals.var_fb_dn5), (-locals.var_fb_dn6), (-locals.var_fb_dn7), (-locals.var_fb_dn8), (-locals.var_fb_dn9), (-locals.var_fb_dn10), (-locals.var_fb_dn11), (-locals.var_fb_dn14),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn11, locals.var_fs02_dn14,)
    }
};
            locals.var_fs02 = assign94370_body47_e145386;
            locals.var_fs02_dn0 = assign94370_body47_e145386_d_n0;
            locals.var_fs02_dn2 = assign94370_body47_e145386_d_n2;
            locals.var_fs02_dn4 = assign94370_body47_e145386_d_n4;
            locals.var_fs02_dn5 = assign94370_body47_e145386_d_n5;
            locals.var_fs02_dn6 = assign94370_body47_e145386_d_n6;
            locals.var_fs02_dn7 = assign94370_body47_e145386_d_n7;
            locals.var_fs02_dn8 = assign94370_body47_e145386_d_n8;
            locals.var_fs02_dn9 = assign94370_body47_e145386_d_n9;
            locals.var_fs02_dn10 = assign94370_body47_e145386_d_n10;
            locals.var_fs02_dn11 = assign94370_body47_e145386_d_n11;
            locals.var_fs02_dn14 = assign94370_body47_e145386_d_n14;
            locals.var_fs02_rv = 0.0;
            let (assign94370_body48_e145400, assign94370_body48_e145400_d_n0, assign94370_body48_e145400_d_n2, assign94370_body48_e145400_d_n4, assign94370_body48_e145400_d_n5, assign94370_body48_e145400_d_n6, assign94370_body48_e145400_d_n7, assign94370_body48_e145400_d_n8, assign94370_body48_e145400_d_n9, assign94370_body48_e145400_d_n10, assign94370_body48_e145400_d_n11, assign94370_body48_e145400_d_n14,) = {
    if (((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2182 != 0.0)) && (locals.var_guard2192 != 0.0)) {
        let assign94370_body48_e145398: f64 = (-locals.var_fb_dpss);
        (assign94370_body48_e145398, (-locals.var_fb_dpss_dn0), (-locals.var_fb_dpss_dn2), (-locals.var_fb_dpss_dn4), (-locals.var_fb_dpss_dn5), (-locals.var_fb_dpss_dn6), (-locals.var_fb_dpss_dn7), (-locals.var_fb_dpss_dn8), (-locals.var_fb_dpss_dn9), (-locals.var_fb_dpss_dn10), (-locals.var_fb_dpss_dn11), (-locals.var_fb_dpss_dn14),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn11, locals.var_fs02_dps0_dn14,)
    }
};
            locals.var_fs02_dps0 = assign94370_body48_e145400;
            locals.var_fs02_dps0_dn0 = assign94370_body48_e145400_d_n0;
            locals.var_fs02_dps0_dn2 = assign94370_body48_e145400_d_n2;
            locals.var_fs02_dps0_dn4 = assign94370_body48_e145400_d_n4;
            locals.var_fs02_dps0_dn5 = assign94370_body48_e145400_d_n5;
            locals.var_fs02_dps0_dn6 = assign94370_body48_e145400_d_n6;
            locals.var_fs02_dps0_dn7 = assign94370_body48_e145400_d_n7;
            locals.var_fs02_dps0_dn8 = assign94370_body48_e145400_d_n8;
            locals.var_fs02_dps0_dn9 = assign94370_body48_e145400_d_n9;
            locals.var_fs02_dps0_dn10 = assign94370_body48_e145400_d_n10;
            locals.var_fs02_dps0_dn11 = assign94370_body48_e145400_d_n11;
            locals.var_fs02_dps0_dn14 = assign94370_body48_e145400_d_n14;
            locals.var_fs02_dps0_rv = 0.0;
            let assign94370_body49_e145403: f64 = if locals.var_chi < 60.0 { 1.0 } else { 0.0 };
            locals.var_guard2193 = assign94370_body49_e145403;
            locals.var_guard2193_rv = 0.0;
            let assign94370_body50_e145406: f64 = if locals.var_chi < 5e-5 { 1.0 } else { 0.0 };
            locals.var_guard2194 = assign94370_body50_e145406;
            locals.var_guard2194_rv = 0.0;
            let (assign94370_body51_e145446, assign94370_body51_e145446_d_n0, assign94370_body51_e145446_d_n2, assign94370_body51_e145446_d_n4, assign94370_body51_e145446_d_n5, assign94370_body51_e145446_d_n6, assign94370_body51_e145446_d_n7, assign94370_body51_e145446_d_n8, assign94370_body51_e145446_d_n9, assign94370_body51_e145446_d_n10, assign94370_body51_e145446_d_n11, assign94370_body51_e145446_d_n14,) = {
    if (((((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2182 != 0.0)) && (locals.var_guard2192 == 0.0)) && (locals.var_guard2193 != 0.0)) && (locals.var_guard2194 != 0.0)) {
        let assign94370_body51_e145424: f64 = (locals.var_chi * locals.var_chi);
        let assign94370_body51_e145426: f64 = (assign94370_body51_e145424 / 2.0);
        let assign94370_body51_e145430: f64 = (locals.var_chi / 3.0);
        let assign94370_body51_e145434: f64 = (locals.var_chi / 4.0);
        let assign94370_body51_e145438: f64 = (locals.var_chi / 5.0);
        let assign94370_body51_e145439: f64 = (1.0 + assign94370_body51_e145438);
        let assign94370_body51_e145440: f64 = (assign94370_body51_e145434 * assign94370_body51_e145439);
        let assign94370_body51_e145441: f64 = (1.0 + assign94370_body51_e145440);
        let assign94370_body51_e145442: f64 = (assign94370_body51_e145430 * assign94370_body51_e145441);
        let assign94370_body51_e145443: f64 = (1.0 + assign94370_body51_e145442);
        let assign94370_body51_e145444: f64 = (assign94370_body51_e145426 * assign94370_body51_e145443);
        (assign94370_body51_e145444, (((((locals.var_chi_dn0 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn0)) / 2.0) * assign94370_body51_e145443) + (assign94370_body51_e145426 * (((locals.var_chi_dn0 / 3.0) * assign94370_body51_e145441) + (assign94370_body51_e145430 * (((locals.var_chi_dn0 / 4.0) * assign94370_body51_e145439) + (assign94370_body51_e145434 * (locals.var_chi_dn0 / 5.0))))))), (((((locals.var_chi_dn2 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn2)) / 2.0) * assign94370_body51_e145443) + (assign94370_body51_e145426 * (((locals.var_chi_dn2 / 3.0) * assign94370_body51_e145441) + (assign94370_body51_e145430 * (((locals.var_chi_dn2 / 4.0) * assign94370_body51_e145439) + (assign94370_body51_e145434 * (locals.var_chi_dn2 / 5.0))))))), (((((locals.var_chi_dn4 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn4)) / 2.0) * assign94370_body51_e145443) + (assign94370_body51_e145426 * (((locals.var_chi_dn4 / 3.0) * assign94370_body51_e145441) + (assign94370_body51_e145430 * (((locals.var_chi_dn4 / 4.0) * assign94370_body51_e145439) + (assign94370_body51_e145434 * (locals.var_chi_dn4 / 5.0))))))), (((((locals.var_chi_dn5 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn5)) / 2.0) * assign94370_body51_e145443) + (assign94370_body51_e145426 * (((locals.var_chi_dn5 / 3.0) * assign94370_body51_e145441) + (assign94370_body51_e145430 * (((locals.var_chi_dn5 / 4.0) * assign94370_body51_e145439) + (assign94370_body51_e145434 * (locals.var_chi_dn5 / 5.0))))))), (((((locals.var_chi_dn6 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn6)) / 2.0) * assign94370_body51_e145443) + (assign94370_body51_e145426 * (((locals.var_chi_dn6 / 3.0) * assign94370_body51_e145441) + (assign94370_body51_e145430 * (((locals.var_chi_dn6 / 4.0) * assign94370_body51_e145439) + (assign94370_body51_e145434 * (locals.var_chi_dn6 / 5.0))))))), (((((locals.var_chi_dn7 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn7)) / 2.0) * assign94370_body51_e145443) + (assign94370_body51_e145426 * (((locals.var_chi_dn7 / 3.0) * assign94370_body51_e145441) + (assign94370_body51_e145430 * (((locals.var_chi_dn7 / 4.0) * assign94370_body51_e145439) + (assign94370_body51_e145434 * (locals.var_chi_dn7 / 5.0))))))), (((((locals.var_chi_dn8 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn8)) / 2.0) * assign94370_body51_e145443) + (assign94370_body51_e145426 * (((locals.var_chi_dn8 / 3.0) * assign94370_body51_e145441) + (assign94370_body51_e145430 * (((locals.var_chi_dn8 / 4.0) * assign94370_body51_e145439) + (assign94370_body51_e145434 * (locals.var_chi_dn8 / 5.0))))))), (((((locals.var_chi_dn9 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn9)) / 2.0) * assign94370_body51_e145443) + (assign94370_body51_e145426 * (((locals.var_chi_dn9 / 3.0) * assign94370_body51_e145441) + (assign94370_body51_e145430 * (((locals.var_chi_dn9 / 4.0) * assign94370_body51_e145439) + (assign94370_body51_e145434 * (locals.var_chi_dn9 / 5.0))))))), (((((locals.var_chi_dn10 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn10)) / 2.0) * assign94370_body51_e145443) + (assign94370_body51_e145426 * (((locals.var_chi_dn10 / 3.0) * assign94370_body51_e145441) + (assign94370_body51_e145430 * (((locals.var_chi_dn10 / 4.0) * assign94370_body51_e145439) + (assign94370_body51_e145434 * (locals.var_chi_dn10 / 5.0))))))), (((((locals.var_chi_dn11 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn11)) / 2.0) * assign94370_body51_e145443) + (assign94370_body51_e145426 * (((locals.var_chi_dn11 / 3.0) * assign94370_body51_e145441) + (assign94370_body51_e145430 * (((locals.var_chi_dn11 / 4.0) * assign94370_body51_e145439) + (assign94370_body51_e145434 * (locals.var_chi_dn11 / 5.0))))))), (((((locals.var_chi_dn14 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn14)) / 2.0) * assign94370_body51_e145443) + (assign94370_body51_e145426 * (((locals.var_chi_dn14 / 3.0) * assign94370_body51_e145441) + (assign94370_body51_e145430 * (((locals.var_chi_dn14 / 4.0) * assign94370_body51_e145439) + (assign94370_body51_e145434 * (locals.var_chi_dn14 / 5.0))))))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
            locals.var_t0 = assign94370_body51_e145446;
            locals.var_t0_dn0 = assign94370_body51_e145446_d_n0;
            locals.var_t0_dn2 = assign94370_body51_e145446_d_n2;
            locals.var_t0_dn4 = assign94370_body51_e145446_d_n4;
            locals.var_t0_dn5 = assign94370_body51_e145446_d_n5;
            locals.var_t0_dn6 = assign94370_body51_e145446_d_n6;
            locals.var_t0_dn7 = assign94370_body51_e145446_d_n7;
            locals.var_t0_dn8 = assign94370_body51_e145446_d_n8;
            locals.var_t0_dn9 = assign94370_body51_e145446_d_n9;
            locals.var_t0_dn10 = assign94370_body51_e145446_d_n10;
            locals.var_t0_dn11 = assign94370_body51_e145446_d_n11;
            locals.var_t0_dn14 = assign94370_body51_e145446_d_n14;
            locals.var_t0_rv = 0.0;
            let (assign94370_body52_e145482, assign94370_body52_e145482_d_n0, assign94370_body52_e145482_d_n2, assign94370_body52_e145482_d_n4, assign94370_body52_e145482_d_n5, assign94370_body52_e145482_d_n6, assign94370_body52_e145482_d_n7, assign94370_body52_e145482_d_n8, assign94370_body52_e145482_d_n9, assign94370_body52_e145482_d_n10, assign94370_body52_e145482_d_n11, assign94370_body52_e145482_d_n14,) = {
    if (((((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2182 != 0.0)) && (locals.var_guard2192 == 0.0)) && (locals.var_guard2193 != 0.0)) && (locals.var_guard2194 != 0.0)) {
        let assign94370_body52_e145466: f64 = (locals.var_chi / 2.0);
        let assign94370_body52_e145470: f64 = (locals.var_chi / 3.0);
        let assign94370_body52_e145474: f64 = (locals.var_chi / 4.0);
        let assign94370_body52_e145475: f64 = (1.0 + assign94370_body52_e145474);
        let assign94370_body52_e145476: f64 = (assign94370_body52_e145470 * assign94370_body52_e145475);
        let assign94370_body52_e145477: f64 = (1.0 + assign94370_body52_e145476);
        let assign94370_body52_e145478: f64 = (assign94370_body52_e145466 * assign94370_body52_e145477);
        let assign94370_body52_e145479: f64 = (1.0 + assign94370_body52_e145478);
        let assign94370_body52_e145480: f64 = (locals.var_chi * assign94370_body52_e145479);
        (assign94370_body52_e145480, ((locals.var_chi_dn0 * assign94370_body52_e145479) + (locals.var_chi * (((locals.var_chi_dn0 / 2.0) * assign94370_body52_e145477) + (assign94370_body52_e145466 * (((locals.var_chi_dn0 / 3.0) * assign94370_body52_e145475) + (assign94370_body52_e145470 * (locals.var_chi_dn0 / 4.0))))))), ((locals.var_chi_dn2 * assign94370_body52_e145479) + (locals.var_chi * (((locals.var_chi_dn2 / 2.0) * assign94370_body52_e145477) + (assign94370_body52_e145466 * (((locals.var_chi_dn2 / 3.0) * assign94370_body52_e145475) + (assign94370_body52_e145470 * (locals.var_chi_dn2 / 4.0))))))), ((locals.var_chi_dn4 * assign94370_body52_e145479) + (locals.var_chi * (((locals.var_chi_dn4 / 2.0) * assign94370_body52_e145477) + (assign94370_body52_e145466 * (((locals.var_chi_dn4 / 3.0) * assign94370_body52_e145475) + (assign94370_body52_e145470 * (locals.var_chi_dn4 / 4.0))))))), ((locals.var_chi_dn5 * assign94370_body52_e145479) + (locals.var_chi * (((locals.var_chi_dn5 / 2.0) * assign94370_body52_e145477) + (assign94370_body52_e145466 * (((locals.var_chi_dn5 / 3.0) * assign94370_body52_e145475) + (assign94370_body52_e145470 * (locals.var_chi_dn5 / 4.0))))))), ((locals.var_chi_dn6 * assign94370_body52_e145479) + (locals.var_chi * (((locals.var_chi_dn6 / 2.0) * assign94370_body52_e145477) + (assign94370_body52_e145466 * (((locals.var_chi_dn6 / 3.0) * assign94370_body52_e145475) + (assign94370_body52_e145470 * (locals.var_chi_dn6 / 4.0))))))), ((locals.var_chi_dn7 * assign94370_body52_e145479) + (locals.var_chi * (((locals.var_chi_dn7 / 2.0) * assign94370_body52_e145477) + (assign94370_body52_e145466 * (((locals.var_chi_dn7 / 3.0) * assign94370_body52_e145475) + (assign94370_body52_e145470 * (locals.var_chi_dn7 / 4.0))))))), ((locals.var_chi_dn8 * assign94370_body52_e145479) + (locals.var_chi * (((locals.var_chi_dn8 / 2.0) * assign94370_body52_e145477) + (assign94370_body52_e145466 * (((locals.var_chi_dn8 / 3.0) * assign94370_body52_e145475) + (assign94370_body52_e145470 * (locals.var_chi_dn8 / 4.0))))))), ((locals.var_chi_dn9 * assign94370_body52_e145479) + (locals.var_chi * (((locals.var_chi_dn9 / 2.0) * assign94370_body52_e145477) + (assign94370_body52_e145466 * (((locals.var_chi_dn9 / 3.0) * assign94370_body52_e145475) + (assign94370_body52_e145470 * (locals.var_chi_dn9 / 4.0))))))), ((locals.var_chi_dn10 * assign94370_body52_e145479) + (locals.var_chi * (((locals.var_chi_dn10 / 2.0) * assign94370_body52_e145477) + (assign94370_body52_e145466 * (((locals.var_chi_dn10 / 3.0) * assign94370_body52_e145475) + (assign94370_body52_e145470 * (locals.var_chi_dn10 / 4.0))))))), ((locals.var_chi_dn11 * assign94370_body52_e145479) + (locals.var_chi * (((locals.var_chi_dn11 / 2.0) * assign94370_body52_e145477) + (assign94370_body52_e145466 * (((locals.var_chi_dn11 / 3.0) * assign94370_body52_e145475) + (assign94370_body52_e145470 * (locals.var_chi_dn11 / 4.0))))))), ((locals.var_chi_dn14 * assign94370_body52_e145479) + (locals.var_chi * (((locals.var_chi_dn14 / 2.0) * assign94370_body52_e145477) + (assign94370_body52_e145466 * (((locals.var_chi_dn14 / 3.0) * assign94370_body52_e145475) + (assign94370_body52_e145470 * (locals.var_chi_dn14 / 4.0))))))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
            locals.var_t1 = assign94370_body52_e145482;
            locals.var_t1_dn0 = assign94370_body52_e145482_d_n0;
            locals.var_t1_dn2 = assign94370_body52_e145482_d_n2;
            locals.var_t1_dn4 = assign94370_body52_e145482_d_n4;
            locals.var_t1_dn5 = assign94370_body52_e145482_d_n5;
            locals.var_t1_dn6 = assign94370_body52_e145482_d_n6;
            locals.var_t1_dn7 = assign94370_body52_e145482_d_n7;
            locals.var_t1_dn8 = assign94370_body52_e145482_d_n8;
            locals.var_t1_dn9 = assign94370_body52_e145482_d_n9;
            locals.var_t1_dn10 = assign94370_body52_e145482_d_n10;
            locals.var_t1_dn11 = assign94370_body52_e145482_d_n11;
            locals.var_t1_dn14 = assign94370_body52_e145482_d_n14;
            locals.var_t1_rv = 0.0;
            let (assign94370_body53_e145502, assign94370_body53_e145502_d_n0, assign94370_body53_e145502_d_n2, assign94370_body53_e145502_d_n4, assign94370_body53_e145502_d_n5, assign94370_body53_e145502_d_n6, assign94370_body53_e145502_d_n7, assign94370_body53_e145502_d_n8, assign94370_body53_e145502_d_n9, assign94370_body53_e145502_d_n10, assign94370_body53_e145502_d_n11, assign94370_body53_e145502_d_n14,) = {
    if (((((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2182 != 0.0)) && (locals.var_guard2192 == 0.0)) && (locals.var_guard2193 != 0.0)) && (locals.var_guard2194 != 0.0)) {
        let assign94370_body53_e145500: f64 = (locals.var_cfs1 * locals.var_t0);
        (assign94370_body53_e145500, ((locals.var_cfs1_dn0 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn0)), ((locals.var_cfs1_dn2 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn2)), ((locals.var_cfs1_dn4 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn4)), ((locals.var_cfs1_dn5 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn5)), ((locals.var_cfs1_dn6 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn6)), ((locals.var_cfs1_dn7 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn7)), ((locals.var_cfs1_dn8 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn8)), ((locals.var_cfs1_dn9 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn9)), ((locals.var_cfs1_dn10 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn10)), ((locals.var_cfs1_dn11 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn11)), ((locals.var_cfs1_dn14 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn14)),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn11, locals.var_fs01_dn14,)
    }
};
            locals.var_fs01 = assign94370_body53_e145502;
            locals.var_fs01_dn0 = assign94370_body53_e145502_d_n0;
            locals.var_fs01_dn2 = assign94370_body53_e145502_d_n2;
            locals.var_fs01_dn4 = assign94370_body53_e145502_d_n4;
            locals.var_fs01_dn5 = assign94370_body53_e145502_d_n5;
            locals.var_fs01_dn6 = assign94370_body53_e145502_d_n6;
            locals.var_fs01_dn7 = assign94370_body53_e145502_d_n7;
            locals.var_fs01_dn8 = assign94370_body53_e145502_d_n8;
            locals.var_fs01_dn9 = assign94370_body53_e145502_d_n9;
            locals.var_fs01_dn10 = assign94370_body53_e145502_d_n10;
            locals.var_fs01_dn11 = assign94370_body53_e145502_d_n11;
            locals.var_fs01_dn14 = assign94370_body53_e145502_d_n14;
            locals.var_fs01_rv = 0.0;
            let (assign94370_body54_e145524, assign94370_body54_e145524_d_n0, assign94370_body54_e145524_d_n2, assign94370_body54_e145524_d_n4, assign94370_body54_e145524_d_n5, assign94370_body54_e145524_d_n6, assign94370_body54_e145524_d_n7, assign94370_body54_e145524_d_n8, assign94370_body54_e145524_d_n9, assign94370_body54_e145524_d_n10, assign94370_body54_e145524_d_n11, assign94370_body54_e145524_d_n14,) = {
    if (((((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2182 != 0.0)) && (locals.var_guard2192 == 0.0)) && (locals.var_guard2193 != 0.0)) && (locals.var_guard2194 != 0.0)) {
        let assign94370_body54_e145520: f64 = (locals.var_cfs1 * locals.var_t1);
        let assign94370_body54_e145522: f64 = (assign94370_body54_e145520 * locals.var_beta);
        (assign94370_body54_e145522, ((((locals.var_cfs1_dn0 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn0)) * locals.var_beta) + (assign94370_body54_e145520 * locals.var_beta_dn0)), ((((locals.var_cfs1_dn2 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn2)) * locals.var_beta) + (assign94370_body54_e145520 * locals.var_beta_dn2)), ((((locals.var_cfs1_dn4 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn4)) * locals.var_beta) + (assign94370_body54_e145520 * locals.var_beta_dn4)), ((((locals.var_cfs1_dn5 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn5)) * locals.var_beta) + (assign94370_body54_e145520 * locals.var_beta_dn5)), ((((locals.var_cfs1_dn6 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn6)) * locals.var_beta) + (assign94370_body54_e145520 * locals.var_beta_dn6)), ((((locals.var_cfs1_dn7 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn7)) * locals.var_beta) + (assign94370_body54_e145520 * locals.var_beta_dn7)), ((((locals.var_cfs1_dn8 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn8)) * locals.var_beta) + (assign94370_body54_e145520 * locals.var_beta_dn8)), ((((locals.var_cfs1_dn9 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn9)) * locals.var_beta) + (assign94370_body54_e145520 * locals.var_beta_dn9)), ((((locals.var_cfs1_dn10 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn10)) * locals.var_beta) + (assign94370_body54_e145520 * locals.var_beta_dn10)), ((((locals.var_cfs1_dn11 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn11)) * locals.var_beta) + (assign94370_body54_e145520 * locals.var_beta_dn11)), ((((locals.var_cfs1_dn14 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn14)) * locals.var_beta) + (assign94370_body54_e145520 * locals.var_beta_dn14)),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn11, locals.var_fs01_dps0_dn14,)
    }
};
            locals.var_fs01_dps0 = assign94370_body54_e145524;
            locals.var_fs01_dps0_dn0 = assign94370_body54_e145524_d_n0;
            locals.var_fs01_dps0_dn2 = assign94370_body54_e145524_d_n2;
            locals.var_fs01_dps0_dn4 = assign94370_body54_e145524_d_n4;
            locals.var_fs01_dps0_dn5 = assign94370_body54_e145524_d_n5;
            locals.var_fs01_dps0_dn6 = assign94370_body54_e145524_d_n6;
            locals.var_fs01_dps0_dn7 = assign94370_body54_e145524_d_n7;
            locals.var_fs01_dps0_dn8 = assign94370_body54_e145524_d_n8;
            locals.var_fs01_dps0_dn9 = assign94370_body54_e145524_d_n9;
            locals.var_fs01_dps0_dn10 = assign94370_body54_e145524_d_n10;
            locals.var_fs01_dps0_dn11 = assign94370_body54_e145524_d_n11;
            locals.var_fs01_dps0_dn14 = assign94370_body54_e145524_d_n14;
            locals.var_fs01_dps0_rv = 0.0;
            let (assign94370_body55_e145544, assign94370_body55_e145544_d_n0, assign94370_body55_e145544_d_n2, assign94370_body55_e145544_d_n4, assign94370_body55_e145544_d_n5, assign94370_body55_e145544_d_n6, assign94370_body55_e145544_d_n7, assign94370_body55_e145544_d_n8, assign94370_body55_e145544_d_n9, assign94370_body55_e145544_d_n10, assign94370_body55_e145544_d_n11, assign94370_body55_e145544_d_n14,) = {
    if (((((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2182 != 0.0)) && (locals.var_guard2192 == 0.0)) && (locals.var_guard2193 != 0.0)) && (locals.var_guard2194 == 0.0)) {
        let assign94370_body55_e145542: f64 = (locals.var_chi).exp();
        (assign94370_body55_e145542, (assign94370_body55_e145542 * locals.var_chi_dn0), (assign94370_body55_e145542 * locals.var_chi_dn2), (assign94370_body55_e145542 * locals.var_chi_dn4), (assign94370_body55_e145542 * locals.var_chi_dn5), (assign94370_body55_e145542 * locals.var_chi_dn6), (assign94370_body55_e145542 * locals.var_chi_dn7), (assign94370_body55_e145542 * locals.var_chi_dn8), (assign94370_body55_e145542 * locals.var_chi_dn9), (assign94370_body55_e145542 * locals.var_chi_dn10), (assign94370_body55_e145542 * locals.var_chi_dn11), (assign94370_body55_e145542 * locals.var_chi_dn14),)
    } else {
        (locals.var_exp_chi, locals.var_exp_chi_dn0, locals.var_exp_chi_dn2, locals.var_exp_chi_dn4, locals.var_exp_chi_dn5, locals.var_exp_chi_dn6, locals.var_exp_chi_dn7, locals.var_exp_chi_dn8, locals.var_exp_chi_dn9, locals.var_exp_chi_dn10, locals.var_exp_chi_dn11, locals.var_exp_chi_dn14,)
    }
};
            locals.var_exp_chi = assign94370_body55_e145544;
            locals.var_exp_chi_dn0 = assign94370_body55_e145544_d_n0;
            locals.var_exp_chi_dn2 = assign94370_body55_e145544_d_n2;
            locals.var_exp_chi_dn4 = assign94370_body55_e145544_d_n4;
            locals.var_exp_chi_dn5 = assign94370_body55_e145544_d_n5;
            locals.var_exp_chi_dn6 = assign94370_body55_e145544_d_n6;
            locals.var_exp_chi_dn7 = assign94370_body55_e145544_d_n7;
            locals.var_exp_chi_dn8 = assign94370_body55_e145544_d_n8;
            locals.var_exp_chi_dn9 = assign94370_body55_e145544_d_n9;
            locals.var_exp_chi_dn10 = assign94370_body55_e145544_d_n10;
            locals.var_exp_chi_dn11 = assign94370_body55_e145544_d_n11;
            locals.var_exp_chi_dn14 = assign94370_body55_e145544_d_n14;
            locals.var_exp_chi_rv = 0.0;
            let (assign94370_body56_e145565, assign94370_body56_e145565_d_n0, assign94370_body56_e145565_d_n2, assign94370_body56_e145565_d_n4, assign94370_body56_e145565_d_n5, assign94370_body56_e145565_d_n6, assign94370_body56_e145565_d_n7, assign94370_body56_e145565_d_n8, assign94370_body56_e145565_d_n9, assign94370_body56_e145565_d_n10, assign94370_body56_e145565_d_n11, assign94370_body56_e145565_d_n14,) = {
    if (((((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2182 != 0.0)) && (locals.var_guard2192 == 0.0)) && (locals.var_guard2193 != 0.0)) && (locals.var_guard2194 == 0.0)) {
        let assign94370_body56_e145563: f64 = (locals.var_exp_chi - 1.0);
        (assign94370_body56_e145563, locals.var_exp_chi_dn0, locals.var_exp_chi_dn2, locals.var_exp_chi_dn4, locals.var_exp_chi_dn5, locals.var_exp_chi_dn6, locals.var_exp_chi_dn7, locals.var_exp_chi_dn8, locals.var_exp_chi_dn9, locals.var_exp_chi_dn10, locals.var_exp_chi_dn11, locals.var_exp_chi_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
            locals.var_t1 = assign94370_body56_e145565;
            locals.var_t1_dn0 = assign94370_body56_e145565_d_n0;
            locals.var_t1_dn2 = assign94370_body56_e145565_d_n2;
            locals.var_t1_dn4 = assign94370_body56_e145565_d_n4;
            locals.var_t1_dn5 = assign94370_body56_e145565_d_n5;
            locals.var_t1_dn6 = assign94370_body56_e145565_d_n6;
            locals.var_t1_dn7 = assign94370_body56_e145565_d_n7;
            locals.var_t1_dn8 = assign94370_body56_e145565_d_n8;
            locals.var_t1_dn9 = assign94370_body56_e145565_d_n9;
            locals.var_t1_dn10 = assign94370_body56_e145565_d_n10;
            locals.var_t1_dn11 = assign94370_body56_e145565_d_n11;
            locals.var_t1_dn14 = assign94370_body56_e145565_d_n14;
            locals.var_t1_rv = 0.0;
            let (assign94370_body57_e145588, assign94370_body57_e145588_d_n0, assign94370_body57_e145588_d_n2, assign94370_body57_e145588_d_n4, assign94370_body57_e145588_d_n5, assign94370_body57_e145588_d_n6, assign94370_body57_e145588_d_n7, assign94370_body57_e145588_d_n8, assign94370_body57_e145588_d_n9, assign94370_body57_e145588_d_n10, assign94370_body57_e145588_d_n11, assign94370_body57_e145588_d_n14,) = {
    if (((((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2182 != 0.0)) && (locals.var_guard2192 == 0.0)) && (locals.var_guard2193 != 0.0)) && (locals.var_guard2194 == 0.0)) {
        let assign94370_body57_e145585: f64 = (locals.var_t1 - locals.var_chi);
        let assign94370_body57_e145586: f64 = (locals.var_cfs1 * assign94370_body57_e145585);
        (assign94370_body57_e145586, ((locals.var_cfs1_dn0 * assign94370_body57_e145585) + (locals.var_cfs1 * (locals.var_t1_dn0 - locals.var_chi_dn0))), ((locals.var_cfs1_dn2 * assign94370_body57_e145585) + (locals.var_cfs1 * (locals.var_t1_dn2 - locals.var_chi_dn2))), ((locals.var_cfs1_dn4 * assign94370_body57_e145585) + (locals.var_cfs1 * (locals.var_t1_dn4 - locals.var_chi_dn4))), ((locals.var_cfs1_dn5 * assign94370_body57_e145585) + (locals.var_cfs1 * (locals.var_t1_dn5 - locals.var_chi_dn5))), ((locals.var_cfs1_dn6 * assign94370_body57_e145585) + (locals.var_cfs1 * (locals.var_t1_dn6 - locals.var_chi_dn6))), ((locals.var_cfs1_dn7 * assign94370_body57_e145585) + (locals.var_cfs1 * (locals.var_t1_dn7 - locals.var_chi_dn7))), ((locals.var_cfs1_dn8 * assign94370_body57_e145585) + (locals.var_cfs1 * (locals.var_t1_dn8 - locals.var_chi_dn8))), ((locals.var_cfs1_dn9 * assign94370_body57_e145585) + (locals.var_cfs1 * (locals.var_t1_dn9 - locals.var_chi_dn9))), ((locals.var_cfs1_dn10 * assign94370_body57_e145585) + (locals.var_cfs1 * (locals.var_t1_dn10 - locals.var_chi_dn10))), ((locals.var_cfs1_dn11 * assign94370_body57_e145585) + (locals.var_cfs1 * (locals.var_t1_dn11 - locals.var_chi_dn11))), ((locals.var_cfs1_dn14 * assign94370_body57_e145585) + (locals.var_cfs1 * (locals.var_t1_dn14 - locals.var_chi_dn14))),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn11, locals.var_fs01_dn14,)
    }
};
            locals.var_fs01 = assign94370_body57_e145588;
            locals.var_fs01_dn0 = assign94370_body57_e145588_d_n0;
            locals.var_fs01_dn2 = assign94370_body57_e145588_d_n2;
            locals.var_fs01_dn4 = assign94370_body57_e145588_d_n4;
            locals.var_fs01_dn5 = assign94370_body57_e145588_d_n5;
            locals.var_fs01_dn6 = assign94370_body57_e145588_d_n6;
            locals.var_fs01_dn7 = assign94370_body57_e145588_d_n7;
            locals.var_fs01_dn8 = assign94370_body57_e145588_d_n8;
            locals.var_fs01_dn9 = assign94370_body57_e145588_d_n9;
            locals.var_fs01_dn10 = assign94370_body57_e145588_d_n10;
            locals.var_fs01_dn11 = assign94370_body57_e145588_d_n11;
            locals.var_fs01_dn14 = assign94370_body57_e145588_d_n14;
            locals.var_fs01_rv = 0.0;
            let (assign94370_body58_e145611, assign94370_body58_e145611_d_n0, assign94370_body58_e145611_d_n2, assign94370_body58_e145611_d_n4, assign94370_body58_e145611_d_n5, assign94370_body58_e145611_d_n6, assign94370_body58_e145611_d_n7, assign94370_body58_e145611_d_n8, assign94370_body58_e145611_d_n9, assign94370_body58_e145611_d_n10, assign94370_body58_e145611_d_n11, assign94370_body58_e145611_d_n14,) = {
    if (((((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2182 != 0.0)) && (locals.var_guard2192 == 0.0)) && (locals.var_guard2193 != 0.0)) && (locals.var_guard2194 == 0.0)) {
        let assign94370_body58_e145607: f64 = (locals.var_cfs1 * locals.var_beta);
        let assign94370_body58_e145609: f64 = (assign94370_body58_e145607 * locals.var_t1);
        (assign94370_body58_e145609, ((((locals.var_cfs1_dn0 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn0)) * locals.var_t1) + (assign94370_body58_e145607 * locals.var_t1_dn0)), ((((locals.var_cfs1_dn2 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn2)) * locals.var_t1) + (assign94370_body58_e145607 * locals.var_t1_dn2)), ((((locals.var_cfs1_dn4 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn4)) * locals.var_t1) + (assign94370_body58_e145607 * locals.var_t1_dn4)), ((((locals.var_cfs1_dn5 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn5)) * locals.var_t1) + (assign94370_body58_e145607 * locals.var_t1_dn5)), ((((locals.var_cfs1_dn6 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn6)) * locals.var_t1) + (assign94370_body58_e145607 * locals.var_t1_dn6)), ((((locals.var_cfs1_dn7 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn7)) * locals.var_t1) + (assign94370_body58_e145607 * locals.var_t1_dn7)), ((((locals.var_cfs1_dn8 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn8)) * locals.var_t1) + (assign94370_body58_e145607 * locals.var_t1_dn8)), ((((locals.var_cfs1_dn9 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn9)) * locals.var_t1) + (assign94370_body58_e145607 * locals.var_t1_dn9)), ((((locals.var_cfs1_dn10 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn10)) * locals.var_t1) + (assign94370_body58_e145607 * locals.var_t1_dn10)), ((((locals.var_cfs1_dn11 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn11)) * locals.var_t1) + (assign94370_body58_e145607 * locals.var_t1_dn11)), ((((locals.var_cfs1_dn14 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn14)) * locals.var_t1) + (assign94370_body58_e145607 * locals.var_t1_dn14)),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn11, locals.var_fs01_dps0_dn14,)
    }
};
            locals.var_fs01_dps0 = assign94370_body58_e145611;
            locals.var_fs01_dps0_dn0 = assign94370_body58_e145611_d_n0;
            locals.var_fs01_dps0_dn2 = assign94370_body58_e145611_d_n2;
            locals.var_fs01_dps0_dn4 = assign94370_body58_e145611_d_n4;
            locals.var_fs01_dps0_dn5 = assign94370_body58_e145611_d_n5;
            locals.var_fs01_dps0_dn6 = assign94370_body58_e145611_d_n6;
            locals.var_fs01_dps0_dn7 = assign94370_body58_e145611_d_n7;
            locals.var_fs01_dps0_dn8 = assign94370_body58_e145611_d_n8;
            locals.var_fs01_dps0_dn9 = assign94370_body58_e145611_d_n9;
            locals.var_fs01_dps0_dn10 = assign94370_body58_e145611_d_n10;
            locals.var_fs01_dps0_dn11 = assign94370_body58_e145611_d_n11;
            locals.var_fs01_dps0_dn14 = assign94370_body58_e145611_d_n14;
            locals.var_fs01_dps0_rv = 0.0;
            let (assign94370_body60_e145650, assign94370_body60_e145650_d_n0, assign94370_body60_e145650_d_n2, assign94370_body60_e145650_d_n4, assign94370_body60_e145650_d_n5, assign94370_body60_e145650_d_n6, assign94370_body60_e145650_d_n7, assign94370_body60_e145650_d_n8, assign94370_body60_e145650_d_n9, assign94370_body60_e145650_d_n10, assign94370_body60_e145650_d_n11, assign94370_body60_e145650_d_n14,) = {
    if ((((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2182 != 0.0)) && (locals.var_guard2192 == 0.0)) && (locals.var_guard2193 == 0.0)) {
        let assign94370_body60_e145647: f64 = (locals.var_beta * locals.var_ps0ld);
        let assign94370_body60_e145648: f64 = (assign94370_body60_e145647).exp();
        (assign94370_body60_e145648, (assign94370_body60_e145648 * ((locals.var_beta_dn0 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn0))), (assign94370_body60_e145648 * ((locals.var_beta_dn2 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn2))), (assign94370_body60_e145648 * ((locals.var_beta_dn4 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn4))), (assign94370_body60_e145648 * ((locals.var_beta_dn5 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn5))), (assign94370_body60_e145648 * ((locals.var_beta_dn6 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn6))), (assign94370_body60_e145648 * ((locals.var_beta_dn7 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn7))), (assign94370_body60_e145648 * ((locals.var_beta_dn8 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn8))), (assign94370_body60_e145648 * ((locals.var_beta_dn9 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn9))), (assign94370_body60_e145648 * ((locals.var_beta_dn10 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn10))), (assign94370_body60_e145648 * ((locals.var_beta_dn11 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn11))), (assign94370_body60_e145648 * ((locals.var_beta_dn14 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn14))),)
    } else {
        (locals.var_exp_bps0, locals.var_exp_bps0_dn0, locals.var_exp_bps0_dn2, locals.var_exp_bps0_dn4, locals.var_exp_bps0_dn5, locals.var_exp_bps0_dn6, locals.var_exp_bps0_dn7, locals.var_exp_bps0_dn8, locals.var_exp_bps0_dn9, locals.var_exp_bps0_dn10, locals.var_exp_bps0_dn11, locals.var_exp_bps0_dn14,)
    }
};
            locals.var_exp_bps0 = assign94370_body60_e145650;
            locals.var_exp_bps0_dn0 = assign94370_body60_e145650_d_n0;
            locals.var_exp_bps0_dn2 = assign94370_body60_e145650_d_n2;
            locals.var_exp_bps0_dn4 = assign94370_body60_e145650_d_n4;
            locals.var_exp_bps0_dn5 = assign94370_body60_e145650_d_n5;
            locals.var_exp_bps0_dn6 = assign94370_body60_e145650_d_n6;
            locals.var_exp_bps0_dn7 = assign94370_body60_e145650_d_n7;
            locals.var_exp_bps0_dn8 = assign94370_body60_e145650_d_n8;
            locals.var_exp_bps0_dn9 = assign94370_body60_e145650_d_n9;
            locals.var_exp_bps0_dn10 = assign94370_body60_e145650_d_n10;
            locals.var_exp_bps0_dn11 = assign94370_body60_e145650_d_n11;
            locals.var_exp_bps0_dn14 = assign94370_body60_e145650_d_n14;
            locals.var_exp_bps0_rv = 0.0;
            let (assign94370_body61_e145675, assign94370_body61_e145675_d_n0, assign94370_body61_e145675_d_n2, assign94370_body61_e145675_d_n4, assign94370_body61_e145675_d_n5, assign94370_body61_e145675_d_n6, assign94370_body61_e145675_d_n7, assign94370_body61_e145675_d_n8, assign94370_body61_e145675_d_n9, assign94370_body61_e145675_d_n10, assign94370_body61_e145675_d_n11, assign94370_body61_e145675_d_n14,) = {
    if ((((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2182 != 0.0)) && (locals.var_guard2192 == 0.0)) && (locals.var_guard2193 == 0.0)) {
        let assign94370_body61_e145670: f64 = (locals.var_chi + 1.0);
        let assign94370_body61_e145671: f64 = (locals.var_exp_bvbs * assign94370_body61_e145670);
        let assign94370_body61_e145672: f64 = (locals.var_exp_bps0 - assign94370_body61_e145671);
        let assign94370_body61_e145673: f64 = (locals.var_cnst1over * assign94370_body61_e145672);
        (assign94370_body61_e145673, ((locals.var_cnst1over_dn0 * assign94370_body61_e145672) + (locals.var_cnst1over * (locals.var_exp_bps0_dn0 - ((locals.var_exp_bvbs_dn0 * assign94370_body61_e145670) + (locals.var_exp_bvbs * locals.var_chi_dn0))))), ((locals.var_cnst1over_dn2 * assign94370_body61_e145672) + (locals.var_cnst1over * (locals.var_exp_bps0_dn2 - ((locals.var_exp_bvbs_dn2 * assign94370_body61_e145670) + (locals.var_exp_bvbs * locals.var_chi_dn2))))), ((locals.var_cnst1over_dn4 * assign94370_body61_e145672) + (locals.var_cnst1over * (locals.var_exp_bps0_dn4 - ((locals.var_exp_bvbs_dn4 * assign94370_body61_e145670) + (locals.var_exp_bvbs * locals.var_chi_dn4))))), ((locals.var_cnst1over_dn5 * assign94370_body61_e145672) + (locals.var_cnst1over * (locals.var_exp_bps0_dn5 - ((locals.var_exp_bvbs_dn5 * assign94370_body61_e145670) + (locals.var_exp_bvbs * locals.var_chi_dn5))))), ((locals.var_cnst1over_dn6 * assign94370_body61_e145672) + (locals.var_cnst1over * (locals.var_exp_bps0_dn6 - ((locals.var_exp_bvbs_dn6 * assign94370_body61_e145670) + (locals.var_exp_bvbs * locals.var_chi_dn6))))), ((locals.var_cnst1over_dn7 * assign94370_body61_e145672) + (locals.var_cnst1over * (locals.var_exp_bps0_dn7 - ((locals.var_exp_bvbs_dn7 * assign94370_body61_e145670) + (locals.var_exp_bvbs * locals.var_chi_dn7))))), ((locals.var_cnst1over_dn8 * assign94370_body61_e145672) + (locals.var_cnst1over * (locals.var_exp_bps0_dn8 - ((locals.var_exp_bvbs_dn8 * assign94370_body61_e145670) + (locals.var_exp_bvbs * locals.var_chi_dn8))))), ((locals.var_cnst1over_dn9 * assign94370_body61_e145672) + (locals.var_cnst1over * (locals.var_exp_bps0_dn9 - ((locals.var_exp_bvbs_dn9 * assign94370_body61_e145670) + (locals.var_exp_bvbs * locals.var_chi_dn9))))), ((locals.var_cnst1over_dn10 * assign94370_body61_e145672) + (locals.var_cnst1over * (locals.var_exp_bps0_dn10 - ((locals.var_exp_bvbs_dn10 * assign94370_body61_e145670) + (locals.var_exp_bvbs * locals.var_chi_dn10))))), ((locals.var_cnst1over_dn11 * assign94370_body61_e145672) + (locals.var_cnst1over * (locals.var_exp_bps0_dn11 - ((locals.var_exp_bvbs_dn11 * assign94370_body61_e145670) + (locals.var_exp_bvbs * locals.var_chi_dn11))))), ((locals.var_cnst1over_dn14 * assign94370_body61_e145672) + (locals.var_cnst1over * (locals.var_exp_bps0_dn14 - ((locals.var_exp_bvbs_dn14 * assign94370_body61_e145670) + (locals.var_exp_bvbs * locals.var_chi_dn14))))),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn11, locals.var_fs01_dn14,)
    }
};
            locals.var_fs01 = assign94370_body61_e145675;
            locals.var_fs01_dn0 = assign94370_body61_e145675_d_n0;
            locals.var_fs01_dn2 = assign94370_body61_e145675_d_n2;
            locals.var_fs01_dn4 = assign94370_body61_e145675_d_n4;
            locals.var_fs01_dn5 = assign94370_body61_e145675_d_n5;
            locals.var_fs01_dn6 = assign94370_body61_e145675_d_n6;
            locals.var_fs01_dn7 = assign94370_body61_e145675_d_n7;
            locals.var_fs01_dn8 = assign94370_body61_e145675_d_n8;
            locals.var_fs01_dn9 = assign94370_body61_e145675_d_n9;
            locals.var_fs01_dn10 = assign94370_body61_e145675_d_n10;
            locals.var_fs01_dn11 = assign94370_body61_e145675_d_n11;
            locals.var_fs01_dn14 = assign94370_body61_e145675_d_n14;
            locals.var_fs01_rv = 0.0;
            let (assign94370_body62_e145698, assign94370_body62_e145698_d_n0, assign94370_body62_e145698_d_n2, assign94370_body62_e145698_d_n4, assign94370_body62_e145698_d_n5, assign94370_body62_e145698_d_n6, assign94370_body62_e145698_d_n7, assign94370_body62_e145698_d_n8, assign94370_body62_e145698_d_n9, assign94370_body62_e145698_d_n10, assign94370_body62_e145698_d_n11, assign94370_body62_e145698_d_n14,) = {
    if ((((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2182 != 0.0)) && (locals.var_guard2192 == 0.0)) && (locals.var_guard2193 == 0.0)) {
        let assign94370_body62_e145692: f64 = (locals.var_cnst1over * locals.var_beta);
        let assign94370_body62_e145695: f64 = (locals.var_exp_bps0 - locals.var_exp_bvbs);
        let assign94370_body62_e145696: f64 = (assign94370_body62_e145692 * assign94370_body62_e145695);
        (assign94370_body62_e145696, ((((locals.var_cnst1over_dn0 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn0)) * assign94370_body62_e145695) + (assign94370_body62_e145692 * (locals.var_exp_bps0_dn0 - locals.var_exp_bvbs_dn0))), ((((locals.var_cnst1over_dn2 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn2)) * assign94370_body62_e145695) + (assign94370_body62_e145692 * (locals.var_exp_bps0_dn2 - locals.var_exp_bvbs_dn2))), ((((locals.var_cnst1over_dn4 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn4)) * assign94370_body62_e145695) + (assign94370_body62_e145692 * (locals.var_exp_bps0_dn4 - locals.var_exp_bvbs_dn4))), ((((locals.var_cnst1over_dn5 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn5)) * assign94370_body62_e145695) + (assign94370_body62_e145692 * (locals.var_exp_bps0_dn5 - locals.var_exp_bvbs_dn5))), ((((locals.var_cnst1over_dn6 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn6)) * assign94370_body62_e145695) + (assign94370_body62_e145692 * (locals.var_exp_bps0_dn6 - locals.var_exp_bvbs_dn6))), ((((locals.var_cnst1over_dn7 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn7)) * assign94370_body62_e145695) + (assign94370_body62_e145692 * (locals.var_exp_bps0_dn7 - locals.var_exp_bvbs_dn7))), ((((locals.var_cnst1over_dn8 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn8)) * assign94370_body62_e145695) + (assign94370_body62_e145692 * (locals.var_exp_bps0_dn8 - locals.var_exp_bvbs_dn8))), ((((locals.var_cnst1over_dn9 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn9)) * assign94370_body62_e145695) + (assign94370_body62_e145692 * (locals.var_exp_bps0_dn9 - locals.var_exp_bvbs_dn9))), ((((locals.var_cnst1over_dn10 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn10)) * assign94370_body62_e145695) + (assign94370_body62_e145692 * (locals.var_exp_bps0_dn10 - locals.var_exp_bvbs_dn10))), ((((locals.var_cnst1over_dn11 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn11)) * assign94370_body62_e145695) + (assign94370_body62_e145692 * (locals.var_exp_bps0_dn11 - locals.var_exp_bvbs_dn11))), ((((locals.var_cnst1over_dn14 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn14)) * assign94370_body62_e145695) + (assign94370_body62_e145692 * (locals.var_exp_bps0_dn14 - locals.var_exp_bvbs_dn14))),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn11, locals.var_fs01_dps0_dn14,)
    }
};
            locals.var_fs01_dps0 = assign94370_body62_e145698;
            locals.var_fs01_dps0_dn0 = assign94370_body62_e145698_d_n0;
            locals.var_fs01_dps0_dn2 = assign94370_body62_e145698_d_n2;
            locals.var_fs01_dps0_dn4 = assign94370_body62_e145698_d_n4;
            locals.var_fs01_dps0_dn5 = assign94370_body62_e145698_d_n5;
            locals.var_fs01_dps0_dn6 = assign94370_body62_e145698_d_n6;
            locals.var_fs01_dps0_dn7 = assign94370_body62_e145698_d_n7;
            locals.var_fs01_dps0_dn8 = assign94370_body62_e145698_d_n8;
            locals.var_fs01_dps0_dn9 = assign94370_body62_e145698_d_n9;
            locals.var_fs01_dps0_dn10 = assign94370_body62_e145698_d_n10;
            locals.var_fs01_dps0_dn11 = assign94370_body62_e145698_d_n11;
            locals.var_fs01_dps0_dn14 = assign94370_body62_e145698_d_n14;
            locals.var_fs01_dps0_rv = 0.0;
            let assign94370_body63_e145701: f64 = if locals.var_fs01 > 0.0 { 1.0 } else { 0.0 };
            locals.var_guard2195 = assign94370_body63_e145701;
            locals.var_guard2195_rv = 0.0;
            let (assign94370_body64_e145722, assign94370_body64_e145722_d_n0, assign94370_body64_e145722_d_n2, assign94370_body64_e145722_d_n4, assign94370_body64_e145722_d_n5, assign94370_body64_e145722_d_n6, assign94370_body64_e145722_d_n7, assign94370_body64_e145722_d_n8, assign94370_body64_e145722_d_n9, assign94370_body64_e145722_d_n10, assign94370_body64_e145722_d_n11, assign94370_body64_e145722_d_n14,) = {
    if ((((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2182 != 0.0)) && (locals.var_guard2192 == 0.0)) && (locals.var_guard2195 != 0.0)) {
        let assign94370_body64_e145717: f64 = (locals.var_fb * locals.var_fb);
        let assign94370_body64_e145719: f64 = (assign94370_body64_e145717 + locals.var_fs01);
        let assign94370_body64_e145720: f64 = (assign94370_body64_e145719).sqrt();
        (assign94370_body64_e145720, ((((locals.var_fb_dn0 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn0)) + locals.var_fs01_dn0) / (2.0 * assign94370_body64_e145720)), ((((locals.var_fb_dn2 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn2)) + locals.var_fs01_dn2) / (2.0 * assign94370_body64_e145720)), ((((locals.var_fb_dn4 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn4)) + locals.var_fs01_dn4) / (2.0 * assign94370_body64_e145720)), ((((locals.var_fb_dn5 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn5)) + locals.var_fs01_dn5) / (2.0 * assign94370_body64_e145720)), ((((locals.var_fb_dn6 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn6)) + locals.var_fs01_dn6) / (2.0 * assign94370_body64_e145720)), ((((locals.var_fb_dn7 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn7)) + locals.var_fs01_dn7) / (2.0 * assign94370_body64_e145720)), ((((locals.var_fb_dn8 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn8)) + locals.var_fs01_dn8) / (2.0 * assign94370_body64_e145720)), ((((locals.var_fb_dn9 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn9)) + locals.var_fs01_dn9) / (2.0 * assign94370_body64_e145720)), ((((locals.var_fb_dn10 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn10)) + locals.var_fs01_dn10) / (2.0 * assign94370_body64_e145720)), ((((locals.var_fb_dn11 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn11)) + locals.var_fs01_dn11) / (2.0 * assign94370_body64_e145720)), ((((locals.var_fb_dn14 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn14)) + locals.var_fs01_dn14) / (2.0 * assign94370_body64_e145720)),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn11, locals.var_fs02_dn14,)
    }
};
            locals.var_fs02 = assign94370_body64_e145722;
            locals.var_fs02_dn0 = assign94370_body64_e145722_d_n0;
            locals.var_fs02_dn2 = assign94370_body64_e145722_d_n2;
            locals.var_fs02_dn4 = assign94370_body64_e145722_d_n4;
            locals.var_fs02_dn5 = assign94370_body64_e145722_d_n5;
            locals.var_fs02_dn6 = assign94370_body64_e145722_d_n6;
            locals.var_fs02_dn7 = assign94370_body64_e145722_d_n7;
            locals.var_fs02_dn8 = assign94370_body64_e145722_d_n8;
            locals.var_fs02_dn9 = assign94370_body64_e145722_d_n9;
            locals.var_fs02_dn10 = assign94370_body64_e145722_d_n10;
            locals.var_fs02_dn11 = assign94370_body64_e145722_d_n11;
            locals.var_fs02_dn14 = assign94370_body64_e145722_d_n14;
            locals.var_fs02_rv = 0.0;
            let (assign94370_body65_e145748, assign94370_body65_e145748_d_n0, assign94370_body65_e145748_d_n2, assign94370_body65_e145748_d_n4, assign94370_body65_e145748_d_n5, assign94370_body65_e145748_d_n6, assign94370_body65_e145748_d_n7, assign94370_body65_e145748_d_n8, assign94370_body65_e145748_d_n9, assign94370_body65_e145748_d_n10, assign94370_body65_e145748_d_n11, assign94370_body65_e145748_d_n14,) = {
    if ((((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2182 != 0.0)) && (locals.var_guard2192 == 0.0)) && (locals.var_guard2195 != 0.0)) {
        let assign94370_body65_e145739: f64 = (2.0 * locals.var_fb_dpss);
        let assign94370_body65_e145741: f64 = (assign94370_body65_e145739 * locals.var_fb);
        let assign94370_body65_e145743: f64 = (assign94370_body65_e145741 + locals.var_fs01_dps0);
        let assign94370_body65_e145744: f64 = (0.5 * assign94370_body65_e145743);
        let assign94370_body65_e145746: f64 = (assign94370_body65_e145744 / locals.var_fs02);
        (assign94370_body65_e145746, ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn0) * locals.var_fb) + (assign94370_body65_e145739 * locals.var_fb_dn0)) + locals.var_fs01_dps0_dn0)) * locals.var_fs02) - (assign94370_body65_e145744 * locals.var_fs02_dn0)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn2) * locals.var_fb) + (assign94370_body65_e145739 * locals.var_fb_dn2)) + locals.var_fs01_dps0_dn2)) * locals.var_fs02) - (assign94370_body65_e145744 * locals.var_fs02_dn2)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn4) * locals.var_fb) + (assign94370_body65_e145739 * locals.var_fb_dn4)) + locals.var_fs01_dps0_dn4)) * locals.var_fs02) - (assign94370_body65_e145744 * locals.var_fs02_dn4)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn5) * locals.var_fb) + (assign94370_body65_e145739 * locals.var_fb_dn5)) + locals.var_fs01_dps0_dn5)) * locals.var_fs02) - (assign94370_body65_e145744 * locals.var_fs02_dn5)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn6) * locals.var_fb) + (assign94370_body65_e145739 * locals.var_fb_dn6)) + locals.var_fs01_dps0_dn6)) * locals.var_fs02) - (assign94370_body65_e145744 * locals.var_fs02_dn6)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn7) * locals.var_fb) + (assign94370_body65_e145739 * locals.var_fb_dn7)) + locals.var_fs01_dps0_dn7)) * locals.var_fs02) - (assign94370_body65_e145744 * locals.var_fs02_dn7)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn8) * locals.var_fb) + (assign94370_body65_e145739 * locals.var_fb_dn8)) + locals.var_fs01_dps0_dn8)) * locals.var_fs02) - (assign94370_body65_e145744 * locals.var_fs02_dn8)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn9) * locals.var_fb) + (assign94370_body65_e145739 * locals.var_fb_dn9)) + locals.var_fs01_dps0_dn9)) * locals.var_fs02) - (assign94370_body65_e145744 * locals.var_fs02_dn9)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn10) * locals.var_fb) + (assign94370_body65_e145739 * locals.var_fb_dn10)) + locals.var_fs01_dps0_dn10)) * locals.var_fs02) - (assign94370_body65_e145744 * locals.var_fs02_dn10)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn11) * locals.var_fb) + (assign94370_body65_e145739 * locals.var_fb_dn11)) + locals.var_fs01_dps0_dn11)) * locals.var_fs02) - (assign94370_body65_e145744 * locals.var_fs02_dn11)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn14) * locals.var_fb) + (assign94370_body65_e145739 * locals.var_fb_dn14)) + locals.var_fs01_dps0_dn14)) * locals.var_fs02) - (assign94370_body65_e145744 * locals.var_fs02_dn14)) / (locals.var_fs02 * locals.var_fs02)),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn11, locals.var_fs02_dps0_dn14,)
    }
};
            locals.var_fs02_dps0 = assign94370_body65_e145748;
            locals.var_fs02_dps0_dn0 = assign94370_body65_e145748_d_n0;
            locals.var_fs02_dps0_dn2 = assign94370_body65_e145748_d_n2;
            locals.var_fs02_dps0_dn4 = assign94370_body65_e145748_d_n4;
            locals.var_fs02_dps0_dn5 = assign94370_body65_e145748_d_n5;
            locals.var_fs02_dps0_dn6 = assign94370_body65_e145748_d_n6;
            locals.var_fs02_dps0_dn7 = assign94370_body65_e145748_d_n7;
            locals.var_fs02_dps0_dn8 = assign94370_body65_e145748_d_n8;
            locals.var_fs02_dps0_dn9 = assign94370_body65_e145748_d_n9;
            locals.var_fs02_dps0_dn10 = assign94370_body65_e145748_d_n10;
            locals.var_fs02_dps0_dn11 = assign94370_body65_e145748_d_n11;
            locals.var_fs02_dps0_dn14 = assign94370_body65_e145748_d_n14;
            locals.var_fs02_dps0_rv = 0.0;
            let (assign94370_body67_e145784, assign94370_body67_e145784_d_n0, assign94370_body67_e145784_d_n2, assign94370_body67_e145784_d_n4, assign94370_body67_e145784_d_n5, assign94370_body67_e145784_d_n6, assign94370_body67_e145784_d_n7, assign94370_body67_e145784_d_n8, assign94370_body67_e145784_d_n9, assign94370_body67_e145784_d_n10, assign94370_body67_e145784_d_n11, assign94370_body67_e145784_d_n14,) = {
    if ((((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2182 != 0.0)) && (locals.var_guard2192 == 0.0)) && (locals.var_guard2195 == 0.0)) {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn14,)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn11, locals.var_fs02_dn14,)
    }
};
            locals.var_fs02 = assign94370_body67_e145784;
            locals.var_fs02_dn0 = assign94370_body67_e145784_d_n0;
            locals.var_fs02_dn2 = assign94370_body67_e145784_d_n2;
            locals.var_fs02_dn4 = assign94370_body67_e145784_d_n4;
            locals.var_fs02_dn5 = assign94370_body67_e145784_d_n5;
            locals.var_fs02_dn6 = assign94370_body67_e145784_d_n6;
            locals.var_fs02_dn7 = assign94370_body67_e145784_d_n7;
            locals.var_fs02_dn8 = assign94370_body67_e145784_d_n8;
            locals.var_fs02_dn9 = assign94370_body67_e145784_d_n9;
            locals.var_fs02_dn10 = assign94370_body67_e145784_d_n10;
            locals.var_fs02_dn11 = assign94370_body67_e145784_d_n11;
            locals.var_fs02_dn14 = assign94370_body67_e145784_d_n14;
            locals.var_fs02_rv = 0.0;
            let (assign94370_body68_e145801, assign94370_body68_e145801_d_n0, assign94370_body68_e145801_d_n2, assign94370_body68_e145801_d_n4, assign94370_body68_e145801_d_n5, assign94370_body68_e145801_d_n6, assign94370_body68_e145801_d_n7, assign94370_body68_e145801_d_n8, assign94370_body68_e145801_d_n9, assign94370_body68_e145801_d_n10, assign94370_body68_e145801_d_n11, assign94370_body68_e145801_d_n14,) = {
    if ((((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2182 != 0.0)) && (locals.var_guard2192 == 0.0)) && (locals.var_guard2195 == 0.0)) {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn14,)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn11, locals.var_fs02_dps0_dn14,)
    }
};
            locals.var_fs02_dps0 = assign94370_body68_e145801;
            locals.var_fs02_dps0_dn0 = assign94370_body68_e145801_d_n0;
            locals.var_fs02_dps0_dn2 = assign94370_body68_e145801_d_n2;
            locals.var_fs02_dps0_dn4 = assign94370_body68_e145801_d_n4;
            locals.var_fs02_dps0_dn5 = assign94370_body68_e145801_d_n5;
            locals.var_fs02_dps0_dn6 = assign94370_body68_e145801_d_n6;
            locals.var_fs02_dps0_dn7 = assign94370_body68_e145801_d_n7;
            locals.var_fs02_dps0_dn8 = assign94370_body68_e145801_d_n8;
            locals.var_fs02_dps0_dn9 = assign94370_body68_e145801_d_n9;
            locals.var_fs02_dps0_dn10 = assign94370_body68_e145801_d_n10;
            locals.var_fs02_dps0_dn11 = assign94370_body68_e145801_d_n11;
            locals.var_fs02_dps0_dn14 = assign94370_body68_e145801_d_n14;
            locals.var_fs02_dps0_rv = 0.0;
            let (assign94370_body69_e145819, assign94370_body69_e145819_d_n0, assign94370_body69_e145819_d_n2, assign94370_body69_e145819_d_n4, assign94370_body69_e145819_d_n5, assign94370_body69_e145819_d_n6, assign94370_body69_e145819_d_n7, assign94370_body69_e145819_d_n8, assign94370_body69_e145819_d_n9, assign94370_body69_e145819_d_n10, assign94370_body69_e145819_d_n11, assign94370_body69_e145819_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2182 != 0.0)) {
        let assign94370_body69_e145811: f64 = (-locals.var_vgpld);
        let assign94370_body69_e145813: f64 = (assign94370_body69_e145811 + locals.var_ps0ld);
        let assign94370_body69_e145816: f64 = (locals.var_fac1 * locals.var_fs02);
        let assign94370_body69_e145817: f64 = (assign94370_body69_e145813 + assign94370_body69_e145816);
        (assign94370_body69_e145817, (locals.var_ps0ld_dn0 + ((locals.var_fac1_dn0 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn0))), (((-locals.var_vgpld_dn2) + locals.var_ps0ld_dn2) + ((locals.var_fac1_dn2 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn2))), (locals.var_ps0ld_dn4 + ((locals.var_fac1_dn4 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn4))), (locals.var_ps0ld_dn5 + ((locals.var_fac1_dn5 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn5))), (locals.var_ps0ld_dn6 + ((locals.var_fac1_dn6 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn6))), (((-locals.var_vgpld_dn7) + locals.var_ps0ld_dn7) + ((locals.var_fac1_dn7 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn7))), (((-locals.var_vgpld_dn8) + locals.var_ps0ld_dn8) + ((locals.var_fac1_dn8 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn8))), (((-locals.var_vgpld_dn9) + locals.var_ps0ld_dn9) + ((locals.var_fac1_dn9 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn9))), (locals.var_ps0ld_dn10 + ((locals.var_fac1_dn10 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn10))), (locals.var_ps0ld_dn11 + ((locals.var_fac1_dn11 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn11))), (locals.var_ps0ld_dn14 + ((locals.var_fac1_dn14 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn14))),)
    } else {
        (locals.var_fs0, locals.var_fs0_dn0, locals.var_fs0_dn2, locals.var_fs0_dn4, locals.var_fs0_dn5, locals.var_fs0_dn6, locals.var_fs0_dn7, locals.var_fs0_dn8, locals.var_fs0_dn9, locals.var_fs0_dn10, locals.var_fs0_dn11, locals.var_fs0_dn14,)
    }
};
            locals.var_fs0 = assign94370_body69_e145819;
            locals.var_fs0_dn0 = assign94370_body69_e145819_d_n0;
            locals.var_fs0_dn2 = assign94370_body69_e145819_d_n2;
            locals.var_fs0_dn4 = assign94370_body69_e145819_d_n4;
            locals.var_fs0_dn5 = assign94370_body69_e145819_d_n5;
            locals.var_fs0_dn6 = assign94370_body69_e145819_d_n6;
            locals.var_fs0_dn7 = assign94370_body69_e145819_d_n7;
            locals.var_fs0_dn8 = assign94370_body69_e145819_d_n8;
            locals.var_fs0_dn9 = assign94370_body69_e145819_d_n9;
            locals.var_fs0_dn10 = assign94370_body69_e145819_d_n10;
            locals.var_fs0_dn11 = assign94370_body69_e145819_d_n11;
            locals.var_fs0_dn14 = assign94370_body69_e145819_d_n14;
            locals.var_fs0_rv = 0.0;
            let (assign94370_body70_e145834, assign94370_body70_e145834_d_n0, assign94370_body70_e145834_d_n2, assign94370_body70_e145834_d_n4, assign94370_body70_e145834_d_n5, assign94370_body70_e145834_d_n6, assign94370_body70_e145834_d_n7, assign94370_body70_e145834_d_n8, assign94370_body70_e145834_d_n9, assign94370_body70_e145834_d_n10, assign94370_body70_e145834_d_n11, assign94370_body70_e145834_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2182 != 0.0)) {
        let assign94370_body70_e145831: f64 = (locals.var_fac1 * locals.var_fs02_dps0);
        let assign94370_body70_e145832: f64 = (1.0 + assign94370_body70_e145831);
        (assign94370_body70_e145832, ((locals.var_fac1_dn0 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn0)), ((locals.var_fac1_dn2 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn2)), ((locals.var_fac1_dn4 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn4)), ((locals.var_fac1_dn5 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn5)), ((locals.var_fac1_dn6 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn6)), ((locals.var_fac1_dn7 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn7)), ((locals.var_fac1_dn8 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn8)), ((locals.var_fac1_dn9 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn9)), ((locals.var_fac1_dn10 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn10)), ((locals.var_fac1_dn11 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn11)), ((locals.var_fac1_dn14 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn14)),)
    } else {
        (locals.var_fs0_dps0, locals.var_fs0_dps0_dn0, locals.var_fs0_dps0_dn2, locals.var_fs0_dps0_dn4, locals.var_fs0_dps0_dn5, locals.var_fs0_dps0_dn6, locals.var_fs0_dps0_dn7, locals.var_fs0_dps0_dn8, locals.var_fs0_dps0_dn9, locals.var_fs0_dps0_dn10, locals.var_fs0_dps0_dn11, locals.var_fs0_dps0_dn14,)
    }
};
            locals.var_fs0_dps0 = assign94370_body70_e145834;
            locals.var_fs0_dps0_dn0 = assign94370_body70_e145834_d_n0;
            locals.var_fs0_dps0_dn2 = assign94370_body70_e145834_d_n2;
            locals.var_fs0_dps0_dn4 = assign94370_body70_e145834_d_n4;
            locals.var_fs0_dps0_dn5 = assign94370_body70_e145834_d_n5;
            locals.var_fs0_dps0_dn6 = assign94370_body70_e145834_d_n6;
            locals.var_fs0_dps0_dn7 = assign94370_body70_e145834_d_n7;
            locals.var_fs0_dps0_dn8 = assign94370_body70_e145834_d_n8;
            locals.var_fs0_dps0_dn9 = assign94370_body70_e145834_d_n9;
            locals.var_fs0_dps0_dn10 = assign94370_body70_e145834_d_n10;
            locals.var_fs0_dps0_dn11 = assign94370_body70_e145834_d_n11;
            locals.var_fs0_dps0_dn14 = assign94370_body70_e145834_d_n14;
            locals.var_fs0_dps0_rv = 0.0;
            let assign94370_body71_e145837: f64 = if locals.var_flg_conv == 1.0 { 1.0 } else { 0.0 };
            locals.var_guard2196 = assign94370_body71_e145837;
            locals.var_guard2196_rv = 0.0;
            let (assign94370_body72_e145852,) = {
    if (((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2182 != 0.0)) && (locals.var_guard2196 != 0.0)) {
        let assign94370_body72_e145850: f64 = (locals.var_lp_s0_max + 1.0);
        (assign94370_body72_e145850,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign94370_body72_e145852;
            locals.var_lp_s0_rv = 0.0;
            let (assign94370_body73_e145869, assign94370_body73_e145869_d_n0, assign94370_body73_e145869_d_n2, assign94370_body73_e145869_d_n4, assign94370_body73_e145869_d_n5, assign94370_body73_e145869_d_n6, assign94370_body73_e145869_d_n7, assign94370_body73_e145869_d_n8, assign94370_body73_e145869_d_n9, assign94370_body73_e145869_d_n10, assign94370_body73_e145869_d_n11, assign94370_body73_e145869_d_n14,) = {
    if (((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2182 != 0.0)) && (locals.var_guard2196 == 0.0)) {
        let assign94370_body73_e145865: f64 = (-locals.var_fs0);
        let assign94370_body73_e145867: f64 = (assign94370_body73_e145865 / locals.var_fs0_dps0);
        (assign94370_body73_e145867, ((((-locals.var_fs0_dn0) * locals.var_fs0_dps0) - (assign94370_body73_e145865 * locals.var_fs0_dps0_dn0)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn2) * locals.var_fs0_dps0) - (assign94370_body73_e145865 * locals.var_fs0_dps0_dn2)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn4) * locals.var_fs0_dps0) - (assign94370_body73_e145865 * locals.var_fs0_dps0_dn4)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn5) * locals.var_fs0_dps0) - (assign94370_body73_e145865 * locals.var_fs0_dps0_dn5)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn6) * locals.var_fs0_dps0) - (assign94370_body73_e145865 * locals.var_fs0_dps0_dn6)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn7) * locals.var_fs0_dps0) - (assign94370_body73_e145865 * locals.var_fs0_dps0_dn7)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn8) * locals.var_fs0_dps0) - (assign94370_body73_e145865 * locals.var_fs0_dps0_dn8)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn9) * locals.var_fs0_dps0) - (assign94370_body73_e145865 * locals.var_fs0_dps0_dn9)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn10) * locals.var_fs0_dps0) - (assign94370_body73_e145865 * locals.var_fs0_dps0_dn10)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn11) * locals.var_fs0_dps0) - (assign94370_body73_e145865 * locals.var_fs0_dps0_dn11)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn14) * locals.var_fs0_dps0) - (assign94370_body73_e145865 * locals.var_fs0_dps0_dn14)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn4, locals.var_dps0_dn5, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn8, locals.var_dps0_dn9, locals.var_dps0_dn10, locals.var_dps0_dn11, locals.var_dps0_dn14,)
    }
};
            locals.var_dps0 = assign94370_body73_e145869;
            locals.var_dps0_dn0 = assign94370_body73_e145869_d_n0;
            locals.var_dps0_dn2 = assign94370_body73_e145869_d_n2;
            locals.var_dps0_dn4 = assign94370_body73_e145869_d_n4;
            locals.var_dps0_dn5 = assign94370_body73_e145869_d_n5;
            locals.var_dps0_dn6 = assign94370_body73_e145869_d_n6;
            locals.var_dps0_dn7 = assign94370_body73_e145869_d_n7;
            locals.var_dps0_dn8 = assign94370_body73_e145869_d_n8;
            locals.var_dps0_dn9 = assign94370_body73_e145869_d_n9;
            locals.var_dps0_dn10 = assign94370_body73_e145869_d_n10;
            locals.var_dps0_dn11 = assign94370_body73_e145869_d_n11;
            locals.var_dps0_dn14 = assign94370_body73_e145869_d_n14;
            locals.var_dps0_rv = 0.0;
            let (assign94370_body74_e145896, assign94370_body74_e145896_d_n0, assign94370_body74_e145896_d_n2, assign94370_body74_e145896_d_n4, assign94370_body74_e145896_d_n5, assign94370_body74_e145896_d_n6, assign94370_body74_e145896_d_n7, assign94370_body74_e145896_d_n8, assign94370_body74_e145896_d_n9, assign94370_body74_e145896_d_n10, assign94370_body74_e145896_d_n11, assign94370_body74_e145896_d_n14,) = {
    if (((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2182 != 0.0)) && (locals.var_guard2196 == 0.0)) {
        let assign94370_body74_e145883: f64 = (0.5 * 0.1);
        let assign94370_body74_e145887: f64 = (locals.var_ps0ld).abs();
        let (assign94370_body74_e145892, assign94370_body74_e145892_d_n0, assign94370_body74_e145892_d_n2, assign94370_body74_e145892_d_n4, assign94370_body74_e145892_d_n5, assign94370_body74_e145892_d_n6, assign94370_body74_e145892_d_n7, assign94370_body74_e145892_d_n8, assign94370_body74_e145892_d_n9, assign94370_body74_e145892_d_n10, assign94370_body74_e145892_d_n11, assign94370_body74_e145892_d_n14,) = {
            if (1.0 >= assign94370_body74_e145887) {
                (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign94370_body74_e145891: f64 = (locals.var_ps0ld).abs();
                (assign94370_body74_e145891, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn0 } else { (-locals.var_ps0ld_dn0) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn2 } else { (-locals.var_ps0ld_dn2) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn4 } else { (-locals.var_ps0ld_dn4) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn5 } else { (-locals.var_ps0ld_dn5) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn6 } else { (-locals.var_ps0ld_dn6) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn7 } else { (-locals.var_ps0ld_dn7) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn8 } else { (-locals.var_ps0ld_dn8) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn9 } else { (-locals.var_ps0ld_dn9) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn10 } else { (-locals.var_ps0ld_dn10) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn11 } else { (-locals.var_ps0ld_dn11) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn14 } else { (-locals.var_ps0ld_dn14) },)
            }
        };
        let assign94370_body74_e145893: f64 = (1.0 + assign94370_body74_e145892);
        let assign94370_body74_e145894: f64 = (assign94370_body74_e145883 * assign94370_body74_e145893);
        (assign94370_body74_e145894, (assign94370_body74_e145883 * assign94370_body74_e145892_d_n0), (assign94370_body74_e145883 * assign94370_body74_e145892_d_n2), (assign94370_body74_e145883 * assign94370_body74_e145892_d_n4), (assign94370_body74_e145883 * assign94370_body74_e145892_d_n5), (assign94370_body74_e145883 * assign94370_body74_e145892_d_n6), (assign94370_body74_e145883 * assign94370_body74_e145892_d_n7), (assign94370_body74_e145883 * assign94370_body74_e145892_d_n8), (assign94370_body74_e145883 * assign94370_body74_e145892_d_n9), (assign94370_body74_e145883 * assign94370_body74_e145892_d_n10), (assign94370_body74_e145883 * assign94370_body74_e145892_d_n11), (assign94370_body74_e145883 * assign94370_body74_e145892_d_n14),)
    } else {
        (locals.var_dplim, locals.var_dplim_dn0, locals.var_dplim_dn2, locals.var_dplim_dn4, locals.var_dplim_dn5, locals.var_dplim_dn6, locals.var_dplim_dn7, locals.var_dplim_dn8, locals.var_dplim_dn9, locals.var_dplim_dn10, locals.var_dplim_dn11, locals.var_dplim_dn14,)
    }
};
            locals.var_dplim = assign94370_body74_e145896;
            locals.var_dplim_dn0 = assign94370_body74_e145896_d_n0;
            locals.var_dplim_dn2 = assign94370_body74_e145896_d_n2;
            locals.var_dplim_dn4 = assign94370_body74_e145896_d_n4;
            locals.var_dplim_dn5 = assign94370_body74_e145896_d_n5;
            locals.var_dplim_dn6 = assign94370_body74_e145896_d_n6;
            locals.var_dplim_dn7 = assign94370_body74_e145896_d_n7;
            locals.var_dplim_dn8 = assign94370_body74_e145896_d_n8;
            locals.var_dplim_dn9 = assign94370_body74_e145896_d_n9;
            locals.var_dplim_dn10 = assign94370_body74_e145896_d_n10;
            locals.var_dplim_dn11 = assign94370_body74_e145896_d_n11;
            locals.var_dplim_dn14 = assign94370_body74_e145896_d_n14;
            locals.var_dplim_rv = 0.0;
            let assign94370_body75_e145898: f64 = (locals.var_dps0).abs();
            let assign94370_body75_e145900: f64 = if assign94370_body75_e145898 > locals.var_dplim { 1.0 } else { 0.0 };
            locals.var_guard2197 = assign94370_body75_e145900;
            locals.var_guard2197_rv = 0.0;
            let (assign94370_body76_e145924, assign94370_body76_e145924_d_n0, assign94370_body76_e145924_d_n2, assign94370_body76_e145924_d_n4, assign94370_body76_e145924_d_n5, assign94370_body76_e145924_d_n6, assign94370_body76_e145924_d_n7, assign94370_body76_e145924_d_n8, assign94370_body76_e145924_d_n9, assign94370_body76_e145924_d_n10, assign94370_body76_e145924_d_n11, assign94370_body76_e145924_d_n14,) = {
    if ((((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2182 != 0.0)) && (locals.var_guard2196 == 0.0)) && (locals.var_guard2197 != 0.0)) {
        let (assign94370_body76_e145921,) = {
            if (locals.var_dps0 >= 0.0) {
                (1.0,)
            } else {
                let assign94370_body76_e145920: f64 = (-1.0);
                (assign94370_body76_e145920,)
            }
        };
        let assign94370_body76_e145922: f64 = (locals.var_dplim * assign94370_body76_e145921);
        (assign94370_body76_e145922, (locals.var_dplim_dn0 * assign94370_body76_e145921), (locals.var_dplim_dn2 * assign94370_body76_e145921), (locals.var_dplim_dn4 * assign94370_body76_e145921), (locals.var_dplim_dn5 * assign94370_body76_e145921), (locals.var_dplim_dn6 * assign94370_body76_e145921), (locals.var_dplim_dn7 * assign94370_body76_e145921), (locals.var_dplim_dn8 * assign94370_body76_e145921), (locals.var_dplim_dn9 * assign94370_body76_e145921), (locals.var_dplim_dn10 * assign94370_body76_e145921), (locals.var_dplim_dn11 * assign94370_body76_e145921), (locals.var_dplim_dn14 * assign94370_body76_e145921),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn4, locals.var_dps0_dn5, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn8, locals.var_dps0_dn9, locals.var_dps0_dn10, locals.var_dps0_dn11, locals.var_dps0_dn14,)
    }
};
            locals.var_dps0 = assign94370_body76_e145924;
            locals.var_dps0_dn0 = assign94370_body76_e145924_d_n0;
            locals.var_dps0_dn2 = assign94370_body76_e145924_d_n2;
            locals.var_dps0_dn4 = assign94370_body76_e145924_d_n4;
            locals.var_dps0_dn5 = assign94370_body76_e145924_d_n5;
            locals.var_dps0_dn6 = assign94370_body76_e145924_d_n6;
            locals.var_dps0_dn7 = assign94370_body76_e145924_d_n7;
            locals.var_dps0_dn8 = assign94370_body76_e145924_d_n8;
            locals.var_dps0_dn9 = assign94370_body76_e145924_d_n9;
            locals.var_dps0_dn10 = assign94370_body76_e145924_d_n10;
            locals.var_dps0_dn11 = assign94370_body76_e145924_d_n11;
            locals.var_dps0_dn14 = assign94370_body76_e145924_d_n14;
            locals.var_dps0_rv = 0.0;
            let (assign94370_body77_e145940, assign94370_body77_e145940_d_n0, assign94370_body77_e145940_d_n2, assign94370_body77_e145940_d_n4, assign94370_body77_e145940_d_n5, assign94370_body77_e145940_d_n6, assign94370_body77_e145940_d_n7, assign94370_body77_e145940_d_n8, assign94370_body77_e145940_d_n9, assign94370_body77_e145940_d_n10, assign94370_body77_e145940_d_n11, assign94370_body77_e145940_d_n14,) = {
    if (((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2182 != 0.0)) && (locals.var_guard2196 == 0.0)) {
        let assign94370_body77_e145938: f64 = (locals.var_ps0ld + locals.var_dps0);
        (assign94370_body77_e145938, (locals.var_ps0ld_dn0 + locals.var_dps0_dn0), (locals.var_ps0ld_dn2 + locals.var_dps0_dn2), (locals.var_ps0ld_dn4 + locals.var_dps0_dn4), (locals.var_ps0ld_dn5 + locals.var_dps0_dn5), (locals.var_ps0ld_dn6 + locals.var_dps0_dn6), (locals.var_ps0ld_dn7 + locals.var_dps0_dn7), (locals.var_ps0ld_dn8 + locals.var_dps0_dn8), (locals.var_ps0ld_dn9 + locals.var_dps0_dn9), (locals.var_ps0ld_dn10 + locals.var_dps0_dn10), (locals.var_ps0ld_dn11 + locals.var_dps0_dn11), (locals.var_ps0ld_dn14 + locals.var_dps0_dn14),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn14,)
    }
};
            locals.var_ps0ld = assign94370_body77_e145940;
            locals.var_ps0ld_dn0 = assign94370_body77_e145940_d_n0;
            locals.var_ps0ld_dn2 = assign94370_body77_e145940_d_n2;
            locals.var_ps0ld_dn4 = assign94370_body77_e145940_d_n4;
            locals.var_ps0ld_dn5 = assign94370_body77_e145940_d_n5;
            locals.var_ps0ld_dn6 = assign94370_body77_e145940_d_n6;
            locals.var_ps0ld_dn7 = assign94370_body77_e145940_d_n7;
            locals.var_ps0ld_dn8 = assign94370_body77_e145940_d_n8;
            locals.var_ps0ld_dn9 = assign94370_body77_e145940_d_n9;
            locals.var_ps0ld_dn10 = assign94370_body77_e145940_d_n10;
            locals.var_ps0ld_dn11 = assign94370_body77_e145940_d_n11;
            locals.var_ps0ld_dn14 = assign94370_body77_e145940_d_n14;
            locals.var_ps0ld_rv = 0.0;
            let assign94370_body78_e145942: f64 = (locals.var_dps0).abs();
            let assign94370_body78_e145946: f64 = (locals.var_fs0).abs();
            let assign94370_body78_e145949: f64 = if ((assign94370_body78_e145942 <= 1e-12) && (assign94370_body78_e145946 <= 1e-8)) { 1.0 } else { 0.0 };
            locals.var_guard2198 = assign94370_body78_e145949;
            locals.var_guard2198_rv = 0.0;
            let (assign94370_body79_e145965,) = {
    if ((((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2182 != 0.0)) && (locals.var_guard2196 == 0.0)) && (locals.var_guard2198 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
            locals.var_flg_conv = assign94370_body79_e145965;
            locals.var_flg_conv_rv = 0.0;
            let (assign94370_body80_e145978,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2182 != 0.0)) {
        let assign94370_body80_e145976: f64 = (locals.var_lp_s0 + 1.0);
        (assign94370_body80_e145976,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign94370_body80_e145978;
            locals.var_lp_s0_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_365(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign94390_e145994, assign94390_e145994_d_n0, assign94390_e145994_d_n2, assign94390_e145994_d_n4, assign94390_e145994_d_n5, assign94390_e145994_d_n6, assign94390_e145994_d_n7, assign94390_e145994_d_n8, assign94390_e145994_d_n9, assign94390_e145994_d_n10, assign94390_e145994_d_n11, assign94390_e145994_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2182 != 0.0)) {
        let assign94390_e145992: f64 = (locals.var_c_w_ld * locals.var_fb);
        (assign94390_e145992, ((locals.var_c_w_ld_dn0 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn0)), ((locals.var_c_w_ld_dn2 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn2)), ((locals.var_c_w_ld_dn4 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn4)), ((locals.var_c_w_ld_dn5 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn5)), ((locals.var_c_w_ld_dn6 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn6)), ((locals.var_c_w_ld_dn7 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn7)), ((locals.var_c_w_ld_dn8 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn8)), ((locals.var_c_w_ld_dn9 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn9)), ((locals.var_c_w_ld_dn10 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn10)), ((locals.var_c_w_ld_dn11 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn11)), ((locals.var_c_w_ld_dn14 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn14)),)
    } else {
        (locals.var_wdld__blk2117, locals.var_wdld__blk2117_dn0, locals.var_wdld__blk2117_dn2, locals.var_wdld__blk2117_dn4, locals.var_wdld__blk2117_dn5, locals.var_wdld__blk2117_dn6, locals.var_wdld__blk2117_dn7, locals.var_wdld__blk2117_dn8, locals.var_wdld__blk2117_dn9, locals.var_wdld__blk2117_dn10, locals.var_wdld__blk2117_dn11, locals.var_wdld__blk2117_dn14,)
    }
};
        locals.var_wdld__blk2117 = assign94390_e145994;
        locals.var_wdld__blk2117_dn0 = assign94390_e145994_d_n0;
        locals.var_wdld__blk2117_dn2 = assign94390_e145994_d_n2;
        locals.var_wdld__blk2117_dn4 = assign94390_e145994_d_n4;
        locals.var_wdld__blk2117_dn5 = assign94390_e145994_d_n5;
        locals.var_wdld__blk2117_dn6 = assign94390_e145994_d_n6;
        locals.var_wdld__blk2117_dn7 = assign94390_e145994_d_n7;
        locals.var_wdld__blk2117_dn8 = assign94390_e145994_d_n8;
        locals.var_wdld__blk2117_dn9 = assign94390_e145994_d_n9;
        locals.var_wdld__blk2117_dn10 = assign94390_e145994_d_n10;
        locals.var_wdld__blk2117_dn11 = assign94390_e145994_d_n11;
        locals.var_wdld__blk2117_dn14 = assign94390_e145994_d_n14;
        locals.var_wdld__blk2117_rv = 0.0;

        let (assign94400_e146007, assign94400_e146007_d_n0, assign94400_e146007_d_n2, assign94400_e146007_d_n4, assign94400_e146007_d_n5, assign94400_e146007_d_n6, assign94400_e146007_d_n7, assign94400_e146007_d_n8, assign94400_e146007_d_n9, assign94400_e146007_d_n10, assign94400_e146007_d_n11, assign94400_e146007_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2182 != 0.0)) {
        let assign94400_e146005: f64 = (locals.var_q_nsubld__blk2119 * locals.var_wdld__blk2117);
        (assign94400_e146005, (locals.var_q_nsubld__blk2119 * locals.var_wdld__blk2117_dn0), (locals.var_q_nsubld__blk2119 * locals.var_wdld__blk2117_dn2), (locals.var_q_nsubld__blk2119 * locals.var_wdld__blk2117_dn4), (locals.var_q_nsubld__blk2119 * locals.var_wdld__blk2117_dn5), (locals.var_q_nsubld__blk2119 * locals.var_wdld__blk2117_dn6), (locals.var_q_nsubld__blk2119 * locals.var_wdld__blk2117_dn7), (locals.var_q_nsubld__blk2119 * locals.var_wdld__blk2117_dn8), (locals.var_q_nsubld__blk2119 * locals.var_wdld__blk2117_dn9), (locals.var_q_nsubld__blk2119 * locals.var_wdld__blk2117_dn10), (locals.var_q_nsubld__blk2119 * locals.var_wdld__blk2117_dn11), (locals.var_q_nsubld__blk2119 * locals.var_wdld__blk2117_dn14),)
    } else {
        (locals.var_q_dep_ld__blk2118, locals.var_q_dep_ld__blk2118_dn0, locals.var_q_dep_ld__blk2118_dn2, locals.var_q_dep_ld__blk2118_dn4, locals.var_q_dep_ld__blk2118_dn5, locals.var_q_dep_ld__blk2118_dn6, locals.var_q_dep_ld__blk2118_dn7, locals.var_q_dep_ld__blk2118_dn8, locals.var_q_dep_ld__blk2118_dn9, locals.var_q_dep_ld__blk2118_dn10, locals.var_q_dep_ld__blk2118_dn11, locals.var_q_dep_ld__blk2118_dn14,)
    }
};
        locals.var_q_dep_ld__blk2118 = assign94400_e146007;
        locals.var_q_dep_ld__blk2118_dn0 = assign94400_e146007_d_n0;
        locals.var_q_dep_ld__blk2118_dn2 = assign94400_e146007_d_n2;
        locals.var_q_dep_ld__blk2118_dn4 = assign94400_e146007_d_n4;
        locals.var_q_dep_ld__blk2118_dn5 = assign94400_e146007_d_n5;
        locals.var_q_dep_ld__blk2118_dn6 = assign94400_e146007_d_n6;
        locals.var_q_dep_ld__blk2118_dn7 = assign94400_e146007_d_n7;
        locals.var_q_dep_ld__blk2118_dn8 = assign94400_e146007_d_n8;
        locals.var_q_dep_ld__blk2118_dn9 = assign94400_e146007_d_n9;
        locals.var_q_dep_ld__blk2118_dn10 = assign94400_e146007_d_n10;
        locals.var_q_dep_ld__blk2118_dn11 = assign94400_e146007_d_n11;
        locals.var_q_dep_ld__blk2118_dn14 = assign94400_e146007_d_n14;
        locals.var_q_dep_ld__blk2118_rv = 0.0;

        let (assign94410_e146024, assign94410_e146024_d_n0, assign94410_e146024_d_n2, assign94410_e146024_d_n4, assign94410_e146024_d_n5, assign94410_e146024_d_n6, assign94410_e146024_d_n7, assign94410_e146024_d_n8, assign94410_e146024_d_n9, assign94410_e146024_d_n10, assign94410_e146024_d_n11, assign94410_e146024_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2182 != 0.0)) {
        let assign94410_e146018: f64 = (locals.var_q_dep_ld__blk2118 / locals.var_cnst0over_func);
        let assign94410_e146021: f64 = (10.0 * 2.220446049250313e-16);
        let assign94410_e146022: f64 = (assign94410_e146018 + assign94410_e146021);
        (assign94410_e146022, (((locals.var_q_dep_ld__blk2118_dn0 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2118 * locals.var_cnst0over_func_dn0)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2118_dn2 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2118 * locals.var_cnst0over_func_dn2)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2118_dn4 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2118 * locals.var_cnst0over_func_dn4)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2118_dn5 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2118 * locals.var_cnst0over_func_dn5)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2118_dn6 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2118 * locals.var_cnst0over_func_dn6)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2118_dn7 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2118 * locals.var_cnst0over_func_dn7)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2118_dn8 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2118 * locals.var_cnst0over_func_dn8)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2118_dn9 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2118 * locals.var_cnst0over_func_dn9)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2118_dn10 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2118 * locals.var_cnst0over_func_dn10)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2118_dn11 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2118 * locals.var_cnst0over_func_dn11)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2118_dn14 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2118 * locals.var_cnst0over_func_dn14)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)),)
    } else {
        (locals.var_xi0p12, locals.var_xi0p12_dn0, locals.var_xi0p12_dn2, locals.var_xi0p12_dn4, locals.var_xi0p12_dn5, locals.var_xi0p12_dn6, locals.var_xi0p12_dn7, locals.var_xi0p12_dn8, locals.var_xi0p12_dn9, locals.var_xi0p12_dn10, locals.var_xi0p12_dn11, locals.var_xi0p12_dn14,)
    }
};
        locals.var_xi0p12 = assign94410_e146024;
        locals.var_xi0p12_dn0 = assign94410_e146024_d_n0;
        locals.var_xi0p12_dn2 = assign94410_e146024_d_n2;
        locals.var_xi0p12_dn4 = assign94410_e146024_d_n4;
        locals.var_xi0p12_dn5 = assign94410_e146024_d_n5;
        locals.var_xi0p12_dn6 = assign94410_e146024_d_n6;
        locals.var_xi0p12_dn7 = assign94410_e146024_d_n7;
        locals.var_xi0p12_dn8 = assign94410_e146024_d_n8;
        locals.var_xi0p12_dn9 = assign94410_e146024_d_n9;
        locals.var_xi0p12_dn10 = assign94410_e146024_d_n10;
        locals.var_xi0p12_dn11 = assign94410_e146024_d_n11;
        locals.var_xi0p12_dn14 = assign94410_e146024_d_n14;
        locals.var_xi0p12_rv = 0.0;

        let (assign94420_e146037, assign94420_e146037_d_n0, assign94420_e146037_d_n2, assign94420_e146037_d_n4, assign94420_e146037_d_n5, assign94420_e146037_d_n6, assign94420_e146037_d_n7, assign94420_e146037_d_n8, assign94420_e146037_d_n9, assign94420_e146037_d_n10, assign94420_e146037_d_n11, assign94420_e146037_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2182 != 0.0)) {
        let assign94420_e146035: f64 = (locals.var_cnst0over_func * locals.var_xi0p12);
        (assign94420_e146035, ((locals.var_cnst0over_func_dn0 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn0)), ((locals.var_cnst0over_func_dn2 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn2)), ((locals.var_cnst0over_func_dn4 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn4)), ((locals.var_cnst0over_func_dn5 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn5)), ((locals.var_cnst0over_func_dn6 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn6)), ((locals.var_cnst0over_func_dn7 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn7)), ((locals.var_cnst0over_func_dn8 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn8)), ((locals.var_cnst0over_func_dn9 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn9)), ((locals.var_cnst0over_func_dn10 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn10)), ((locals.var_cnst0over_func_dn11 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn11)), ((locals.var_cnst0over_func_dn14 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn14)),)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn8, locals.var_qbuld_dn9, locals.var_qbuld_dn10, locals.var_qbuld_dn11, locals.var_qbuld_dn14,)
    }
};
        locals.var_qbuld = assign94420_e146037;
        locals.var_qbuld_dn0 = assign94420_e146037_d_n0;
        locals.var_qbuld_dn2 = assign94420_e146037_d_n2;
        locals.var_qbuld_dn4 = assign94420_e146037_d_n4;
        locals.var_qbuld_dn5 = assign94420_e146037_d_n5;
        locals.var_qbuld_dn6 = assign94420_e146037_d_n6;
        locals.var_qbuld_dn7 = assign94420_e146037_d_n7;
        locals.var_qbuld_dn8 = assign94420_e146037_d_n8;
        locals.var_qbuld_dn9 = assign94420_e146037_d_n9;
        locals.var_qbuld_dn10 = assign94420_e146037_d_n10;
        locals.var_qbuld_dn11 = assign94420_e146037_d_n11;
        locals.var_qbuld_dn14 = assign94420_e146037_d_n14;
        locals.var_qbuld_rv = 0.0;

        let (assign94430_e146052, assign94430_e146052_d_n0, assign94430_e146052_d_n2, assign94430_e146052_d_n4, assign94430_e146052_d_n5, assign94430_e146052_d_n6, assign94430_e146052_d_n7, assign94430_e146052_d_n8, assign94430_e146052_d_n9, assign94430_e146052_d_n10, assign94430_e146052_d_n11, assign94430_e146052_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2182 != 0.0)) {
        let assign94430_e146049: f64 = (locals.var_fs02 + locals.var_xi0p12);
        let assign94430_e146050: f64 = (1.0 / assign94430_e146049);
        (assign94430_e146050, (-((locals.var_fs02_dn0 + locals.var_xi0p12_dn0) / (assign94430_e146049 * assign94430_e146049))), (-((locals.var_fs02_dn2 + locals.var_xi0p12_dn2) / (assign94430_e146049 * assign94430_e146049))), (-((locals.var_fs02_dn4 + locals.var_xi0p12_dn4) / (assign94430_e146049 * assign94430_e146049))), (-((locals.var_fs02_dn5 + locals.var_xi0p12_dn5) / (assign94430_e146049 * assign94430_e146049))), (-((locals.var_fs02_dn6 + locals.var_xi0p12_dn6) / (assign94430_e146049 * assign94430_e146049))), (-((locals.var_fs02_dn7 + locals.var_xi0p12_dn7) / (assign94430_e146049 * assign94430_e146049))), (-((locals.var_fs02_dn8 + locals.var_xi0p12_dn8) / (assign94430_e146049 * assign94430_e146049))), (-((locals.var_fs02_dn9 + locals.var_xi0p12_dn9) / (assign94430_e146049 * assign94430_e146049))), (-((locals.var_fs02_dn10 + locals.var_xi0p12_dn10) / (assign94430_e146049 * assign94430_e146049))), (-((locals.var_fs02_dn11 + locals.var_xi0p12_dn11) / (assign94430_e146049 * assign94430_e146049))), (-((locals.var_fs02_dn14 + locals.var_xi0p12_dn14) / (assign94430_e146049 * assign94430_e146049))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign94430_e146052;
        locals.var_t1_dn0 = assign94430_e146052_d_n0;
        locals.var_t1_dn2 = assign94430_e146052_d_n2;
        locals.var_t1_dn4 = assign94430_e146052_d_n4;
        locals.var_t1_dn5 = assign94430_e146052_d_n5;
        locals.var_t1_dn6 = assign94430_e146052_d_n6;
        locals.var_t1_dn7 = assign94430_e146052_d_n7;
        locals.var_t1_dn8 = assign94430_e146052_d_n8;
        locals.var_t1_dn9 = assign94430_e146052_d_n9;
        locals.var_t1_dn10 = assign94430_e146052_d_n10;
        locals.var_t1_dn11 = assign94430_e146052_d_n11;
        locals.var_t1_dn14 = assign94430_e146052_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign94440_e146067, assign94440_e146067_d_n0, assign94440_e146067_d_n2, assign94440_e146067_d_n4, assign94440_e146067_d_n5, assign94440_e146067_d_n6, assign94440_e146067_d_n7, assign94440_e146067_d_n8, assign94440_e146067_d_n9, assign94440_e146067_d_n10, assign94440_e146067_d_n11, assign94440_e146067_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2182 != 0.0)) {
        let assign94440_e146063: f64 = (locals.var_cnst0over_func * locals.var_fs01);
        let assign94440_e146065: f64 = (assign94440_e146063 * locals.var_t1);
        (assign94440_e146065, ((((locals.var_cnst0over_func_dn0 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn0)) * locals.var_t1) + (assign94440_e146063 * locals.var_t1_dn0)), ((((locals.var_cnst0over_func_dn2 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn2)) * locals.var_t1) + (assign94440_e146063 * locals.var_t1_dn2)), ((((locals.var_cnst0over_func_dn4 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn4)) * locals.var_t1) + (assign94440_e146063 * locals.var_t1_dn4)), ((((locals.var_cnst0over_func_dn5 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn5)) * locals.var_t1) + (assign94440_e146063 * locals.var_t1_dn5)), ((((locals.var_cnst0over_func_dn6 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn6)) * locals.var_t1) + (assign94440_e146063 * locals.var_t1_dn6)), ((((locals.var_cnst0over_func_dn7 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn7)) * locals.var_t1) + (assign94440_e146063 * locals.var_t1_dn7)), ((((locals.var_cnst0over_func_dn8 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn8)) * locals.var_t1) + (assign94440_e146063 * locals.var_t1_dn8)), ((((locals.var_cnst0over_func_dn9 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn9)) * locals.var_t1) + (assign94440_e146063 * locals.var_t1_dn9)), ((((locals.var_cnst0over_func_dn10 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn10)) * locals.var_t1) + (assign94440_e146063 * locals.var_t1_dn10)), ((((locals.var_cnst0over_func_dn11 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn11)) * locals.var_t1) + (assign94440_e146063 * locals.var_t1_dn11)), ((((locals.var_cnst0over_func_dn14 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn14)) * locals.var_t1) + (assign94440_e146063 * locals.var_t1_dn14)),)
    } else {
        (locals.var_qiuld, locals.var_qiuld_dn0, locals.var_qiuld_dn2, locals.var_qiuld_dn4, locals.var_qiuld_dn5, locals.var_qiuld_dn6, locals.var_qiuld_dn7, locals.var_qiuld_dn8, locals.var_qiuld_dn9, locals.var_qiuld_dn10, locals.var_qiuld_dn11, locals.var_qiuld_dn14,)
    }
};
        locals.var_qiuld = assign94440_e146067;
        locals.var_qiuld_dn0 = assign94440_e146067_d_n0;
        locals.var_qiuld_dn2 = assign94440_e146067_d_n2;
        locals.var_qiuld_dn4 = assign94440_e146067_d_n4;
        locals.var_qiuld_dn5 = assign94440_e146067_d_n5;
        locals.var_qiuld_dn6 = assign94440_e146067_d_n6;
        locals.var_qiuld_dn7 = assign94440_e146067_d_n7;
        locals.var_qiuld_dn8 = assign94440_e146067_d_n8;
        locals.var_qiuld_dn9 = assign94440_e146067_d_n9;
        locals.var_qiuld_dn10 = assign94440_e146067_d_n10;
        locals.var_qiuld_dn11 = assign94440_e146067_d_n11;
        locals.var_qiuld_dn14 = assign94440_e146067_d_n14;
        locals.var_qiuld_rv = 0.0;

        let (assign94450_e146080, assign94450_e146080_d_n0, assign94450_e146080_d_n2, assign94450_e146080_d_n4, assign94450_e146080_d_n5, assign94450_e146080_d_n6, assign94450_e146080_d_n7, assign94450_e146080_d_n8, assign94450_e146080_d_n9, assign94450_e146080_d_n10, assign94450_e146080_d_n11, assign94450_e146080_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2143 == 0.0)) && (locals.var_guard2182 != 0.0)) {
        let assign94450_e146078: f64 = (locals.var_qbuld + locals.var_qiuld);
        (assign94450_e146078, (locals.var_qbuld_dn0 + locals.var_qiuld_dn0), (locals.var_qbuld_dn2 + locals.var_qiuld_dn2), (locals.var_qbuld_dn4 + locals.var_qiuld_dn4), (locals.var_qbuld_dn5 + locals.var_qiuld_dn5), (locals.var_qbuld_dn6 + locals.var_qiuld_dn6), (locals.var_qbuld_dn7 + locals.var_qiuld_dn7), (locals.var_qbuld_dn8 + locals.var_qiuld_dn8), (locals.var_qbuld_dn9 + locals.var_qiuld_dn9), (locals.var_qbuld_dn10 + locals.var_qiuld_dn10), (locals.var_qbuld_dn11 + locals.var_qiuld_dn11), (locals.var_qbuld_dn14 + locals.var_qiuld_dn14),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn4, locals.var_qsuld_dn5, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn8, locals.var_qsuld_dn9, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn14,)
    }
};
        locals.var_qsuld = assign94450_e146080;
        locals.var_qsuld_dn0 = assign94450_e146080_d_n0;
        locals.var_qsuld_dn2 = assign94450_e146080_d_n2;
        locals.var_qsuld_dn4 = assign94450_e146080_d_n4;
        locals.var_qsuld_dn5 = assign94450_e146080_d_n5;
        locals.var_qsuld_dn6 = assign94450_e146080_d_n6;
        locals.var_qsuld_dn7 = assign94450_e146080_d_n7;
        locals.var_qsuld_dn8 = assign94450_e146080_d_n8;
        locals.var_qsuld_dn9 = assign94450_e146080_d_n9;
        locals.var_qsuld_dn10 = assign94450_e146080_d_n10;
        locals.var_qsuld_dn11 = assign94450_e146080_d_n11;
        locals.var_qsuld_dn14 = assign94450_e146080_d_n14;
        locals.var_qsuld_rv = 0.0;

        let assign94460_e146083: f64 = if p.p33 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard2200 = assign94460_e146083;
        locals.var_guard2200_rv = 0.0;

        let (assign94470_e146095, assign94470_e146095_d_n0, assign94470_e146095_d_n2, assign94470_e146095_d_n4, assign94470_e146095_d_n5, assign94470_e146095_d_n6, assign94470_e146095_d_n7, assign94470_e146095_d_n8, assign94470_e146095_d_n9, assign94470_e146095_d_n10, assign94470_e146095_d_n11, assign94470_e146095_d_n14,) = {
    if (((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2200 != 0.0)) {
        let assign94470_e146091: f64 = (-locals.var_vxbgmtcl);
        let assign94470_e146092: f64 = (locals.var_beta * assign94470_e146091);
        let assign94470_e146093: f64 = (assign94470_e146092).exp();
        (assign94470_e146093, (assign94470_e146093 * ((locals.var_beta_dn0 * assign94470_e146091) + (locals.var_beta * (-locals.var_vxbgmtcl_dn0)))), (assign94470_e146093 * ((locals.var_beta_dn2 * assign94470_e146091) + (locals.var_beta * (-locals.var_vxbgmtcl_dn2)))), (assign94470_e146093 * ((locals.var_beta_dn4 * assign94470_e146091) + (locals.var_beta * (-locals.var_vxbgmtcl_dn4)))), (assign94470_e146093 * ((locals.var_beta_dn5 * assign94470_e146091) + (locals.var_beta * (-locals.var_vxbgmtcl_dn5)))), (assign94470_e146093 * ((locals.var_beta_dn6 * assign94470_e146091) + (locals.var_beta * (-locals.var_vxbgmtcl_dn6)))), (assign94470_e146093 * ((locals.var_beta_dn7 * assign94470_e146091) + (locals.var_beta * (-locals.var_vxbgmtcl_dn7)))), (assign94470_e146093 * ((locals.var_beta_dn8 * assign94470_e146091) + (locals.var_beta * (-locals.var_vxbgmtcl_dn8)))), (assign94470_e146093 * ((locals.var_beta_dn9 * assign94470_e146091) + (locals.var_beta * (-locals.var_vxbgmtcl_dn9)))), (assign94470_e146093 * ((locals.var_beta_dn10 * assign94470_e146091) + (locals.var_beta * (-locals.var_vxbgmtcl_dn10)))), (assign94470_e146093 * ((locals.var_beta_dn11 * assign94470_e146091) + (locals.var_beta * (-locals.var_vxbgmtcl_dn11)))), (assign94470_e146093 * ((locals.var_beta_dn14 * assign94470_e146091) + (locals.var_beta * (-locals.var_vxbgmtcl_dn14)))),)
    } else {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn11, locals.var_exp_bvbs_dn14,)
    }
};
        locals.var_exp_bvbs = assign94470_e146095;
        locals.var_exp_bvbs_dn0 = assign94470_e146095_d_n0;
        locals.var_exp_bvbs_dn2 = assign94470_e146095_d_n2;
        locals.var_exp_bvbs_dn4 = assign94470_e146095_d_n4;
        locals.var_exp_bvbs_dn5 = assign94470_e146095_d_n5;
        locals.var_exp_bvbs_dn6 = assign94470_e146095_d_n6;
        locals.var_exp_bvbs_dn7 = assign94470_e146095_d_n7;
        locals.var_exp_bvbs_dn8 = assign94470_e146095_d_n8;
        locals.var_exp_bvbs_dn9 = assign94470_e146095_d_n9;
        locals.var_exp_bvbs_dn10 = assign94470_e146095_d_n10;
        locals.var_exp_bvbs_dn11 = assign94470_e146095_d_n11;
        locals.var_exp_bvbs_dn14 = assign94470_e146095_d_n14;
        locals.var_exp_bvbs_rv = 0.0;

        let (assign94480_e146105, assign94480_e146105_d_n0, assign94480_e146105_d_n2, assign94480_e146105_d_n4, assign94480_e146105_d_n5, assign94480_e146105_d_n6, assign94480_e146105_d_n7, assign94480_e146105_d_n8, assign94480_e146105_d_n9, assign94480_e146105_d_n10, assign94480_e146105_d_n11, assign94480_e146105_d_n14,) = {
    if (((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2200 != 0.0)) {
        let assign94480_e146103: f64 = (locals.var_nin / locals.var_nover_func);
        (assign94480_e146103, (locals.var_nin_dn0 / locals.var_nover_func), (locals.var_nin_dn2 / locals.var_nover_func), (locals.var_nin_dn4 / locals.var_nover_func), (locals.var_nin_dn5 / locals.var_nover_func), (locals.var_nin_dn6 / locals.var_nover_func), (locals.var_nin_dn7 / locals.var_nover_func), (locals.var_nin_dn8 / locals.var_nover_func), (locals.var_nin_dn9 / locals.var_nover_func), (locals.var_nin_dn10 / locals.var_nover_func), (locals.var_nin_dn11 / locals.var_nover_func), (locals.var_nin_dn14 / locals.var_nover_func),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign94480_e146105;
        locals.var_t0_dn0 = assign94480_e146105_d_n0;
        locals.var_t0_dn2 = assign94480_e146105_d_n2;
        locals.var_t0_dn4 = assign94480_e146105_d_n4;
        locals.var_t0_dn5 = assign94480_e146105_d_n5;
        locals.var_t0_dn6 = assign94480_e146105_d_n6;
        locals.var_t0_dn7 = assign94480_e146105_d_n7;
        locals.var_t0_dn8 = assign94480_e146105_d_n8;
        locals.var_t0_dn9 = assign94480_e146105_d_n9;
        locals.var_t0_dn10 = assign94480_e146105_d_n10;
        locals.var_t0_dn11 = assign94480_e146105_d_n11;
        locals.var_t0_dn14 = assign94480_e146105_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign94490_e146115, assign94490_e146115_d_n0, assign94490_e146115_d_n2, assign94490_e146115_d_n4, assign94490_e146115_d_n5, assign94490_e146115_d_n6, assign94490_e146115_d_n7, assign94490_e146115_d_n8, assign94490_e146115_d_n9, assign94490_e146115_d_n10, assign94490_e146115_d_n11, assign94490_e146115_d_n14,) = {
    if (((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2200 != 0.0)) {
        let assign94490_e146113: f64 = (locals.var_t0 * locals.var_t0);
        (assign94490_e146113, ((locals.var_t0_dn0 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn0)), ((locals.var_t0_dn2 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn2)), ((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)), ((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)), ((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)), ((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)), ((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)), ((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)), ((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)), ((locals.var_t0_dn11 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn11)), ((locals.var_t0_dn14 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn14)),)
    } else {
        (locals.var_cnst1over, locals.var_cnst1over_dn0, locals.var_cnst1over_dn2, locals.var_cnst1over_dn4, locals.var_cnst1over_dn5, locals.var_cnst1over_dn6, locals.var_cnst1over_dn7, locals.var_cnst1over_dn8, locals.var_cnst1over_dn9, locals.var_cnst1over_dn10, locals.var_cnst1over_dn11, locals.var_cnst1over_dn14,)
    }
};
        locals.var_cnst1over = assign94490_e146115;
        locals.var_cnst1over_dn0 = assign94490_e146115_d_n0;
        locals.var_cnst1over_dn2 = assign94490_e146115_d_n2;
        locals.var_cnst1over_dn4 = assign94490_e146115_d_n4;
        locals.var_cnst1over_dn5 = assign94490_e146115_d_n5;
        locals.var_cnst1over_dn6 = assign94490_e146115_d_n6;
        locals.var_cnst1over_dn7 = assign94490_e146115_d_n7;
        locals.var_cnst1over_dn8 = assign94490_e146115_d_n8;
        locals.var_cnst1over_dn9 = assign94490_e146115_d_n9;
        locals.var_cnst1over_dn10 = assign94490_e146115_d_n10;
        locals.var_cnst1over_dn11 = assign94490_e146115_d_n11;
        locals.var_cnst1over_dn14 = assign94490_e146115_d_n14;
        locals.var_cnst1over_rv = 0.0;

        let (assign94500_e146125, assign94500_e146125_d_n0, assign94500_e146125_d_n2, assign94500_e146125_d_n4, assign94500_e146125_d_n5, assign94500_e146125_d_n6, assign94500_e146125_d_n7, assign94500_e146125_d_n8, assign94500_e146125_d_n9, assign94500_e146125_d_n10, assign94500_e146125_d_n11, assign94500_e146125_d_n14,) = {
    if (((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2200 != 0.0)) {
        let assign94500_e146123: f64 = (locals.var_cnst1over * locals.var_exp_bvbs);
        (assign94500_e146123, ((locals.var_cnst1over_dn0 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn0)), ((locals.var_cnst1over_dn2 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn2)), ((locals.var_cnst1over_dn4 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn4)), ((locals.var_cnst1over_dn5 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn5)), ((locals.var_cnst1over_dn6 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn6)), ((locals.var_cnst1over_dn7 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn7)), ((locals.var_cnst1over_dn8 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn8)), ((locals.var_cnst1over_dn9 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn9)), ((locals.var_cnst1over_dn10 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn10)), ((locals.var_cnst1over_dn11 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn11)), ((locals.var_cnst1over_dn14 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn14)),)
    } else {
        (locals.var_cfs1, locals.var_cfs1_dn0, locals.var_cfs1_dn2, locals.var_cfs1_dn4, locals.var_cfs1_dn5, locals.var_cfs1_dn6, locals.var_cfs1_dn7, locals.var_cfs1_dn8, locals.var_cfs1_dn9, locals.var_cfs1_dn10, locals.var_cfs1_dn11, locals.var_cfs1_dn14,)
    }
};
        locals.var_cfs1 = assign94500_e146125;
        locals.var_cfs1_dn0 = assign94500_e146125_d_n0;
        locals.var_cfs1_dn2 = assign94500_e146125_d_n2;
        locals.var_cfs1_dn4 = assign94500_e146125_d_n4;
        locals.var_cfs1_dn5 = assign94500_e146125_d_n5;
        locals.var_cfs1_dn6 = assign94500_e146125_d_n6;
        locals.var_cfs1_dn7 = assign94500_e146125_d_n7;
        locals.var_cfs1_dn8 = assign94500_e146125_d_n8;
        locals.var_cfs1_dn9 = assign94500_e146125_d_n9;
        locals.var_cfs1_dn10 = assign94500_e146125_d_n10;
        locals.var_cfs1_dn11 = assign94500_e146125_d_n11;
        locals.var_cfs1_dn14 = assign94500_e146125_d_n14;
        locals.var_cfs1_rv = 0.0;

        let (assign94510_e146133, assign94510_e146133_d_n0, assign94510_e146133_d_n2, assign94510_e146133_d_n4, assign94510_e146133_d_n5, assign94510_e146133_d_n6, assign94510_e146133_d_n7, assign94510_e146133_d_n8, assign94510_e146133_d_n9, assign94510_e146133_d_n10, assign94510_e146133_d_n11, assign94510_e146133_d_n14,) = {
    if (((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2200 != 0.0)) {
        (locals.var_ps0ld_ini__blk2126, locals.var_ps0ld_ini__blk2126_dn0, locals.var_ps0ld_ini__blk2126_dn2, locals.var_ps0ld_ini__blk2126_dn4, locals.var_ps0ld_ini__blk2126_dn5, locals.var_ps0ld_ini__blk2126_dn6, locals.var_ps0ld_ini__blk2126_dn7, locals.var_ps0ld_ini__blk2126_dn8, locals.var_ps0ld_ini__blk2126_dn9, locals.var_ps0ld_ini__blk2126_dn10, locals.var_ps0ld_ini__blk2126_dn11, locals.var_ps0ld_ini__blk2126_dn14,)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn14,)
    }
};
        locals.var_ps0ld = assign94510_e146133;
        locals.var_ps0ld_dn0 = assign94510_e146133_d_n0;
        locals.var_ps0ld_dn2 = assign94510_e146133_d_n2;
        locals.var_ps0ld_dn4 = assign94510_e146133_d_n4;
        locals.var_ps0ld_dn5 = assign94510_e146133_d_n5;
        locals.var_ps0ld_dn6 = assign94510_e146133_d_n6;
        locals.var_ps0ld_dn7 = assign94510_e146133_d_n7;
        locals.var_ps0ld_dn8 = assign94510_e146133_d_n8;
        locals.var_ps0ld_dn9 = assign94510_e146133_d_n9;
        locals.var_ps0ld_dn10 = assign94510_e146133_d_n10;
        locals.var_ps0ld_dn11 = assign94510_e146133_d_n11;
        locals.var_ps0ld_dn14 = assign94510_e146133_d_n14;
        locals.var_ps0ld_rv = 0.0;

        let (assign94520_e146141,) = {
    if (((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2200 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
        locals.var_flg_conv = assign94520_e146141;
        locals.var_flg_conv_rv = 0.0;

        let (assign94530_e146156, assign94530_e146156_d_n0, assign94530_e146156_d_n2, assign94530_e146156_d_n4, assign94530_e146156_d_n5, assign94530_e146156_d_n6, assign94530_e146156_d_n7, assign94530_e146156_d_n8, assign94530_e146156_d_n9, assign94530_e146156_d_n10, assign94530_e146156_d_n11, assign94530_e146156_d_n14,) = {
    if (((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2200 != 0.0)) {
        let assign94530_e146150: f64 = (1.034943e-10 / locals.var_q_nsubld__blk2119);
        let assign94530_e146152: f64 = (assign94530_e146150 * locals.var_beta_inv);
        let assign94530_e146153: f64 = (2.0 * assign94530_e146152);
        let assign94530_e146154: f64 = (assign94530_e146153).sqrt();
        (assign94530_e146154, ((2.0 * (assign94530_e146150 * locals.var_beta_inv_dn0)) / (2.0 * assign94530_e146154)), ((2.0 * (assign94530_e146150 * locals.var_beta_inv_dn2)) / (2.0 * assign94530_e146154)), ((2.0 * (assign94530_e146150 * locals.var_beta_inv_dn4)) / (2.0 * assign94530_e146154)), ((2.0 * (assign94530_e146150 * locals.var_beta_inv_dn5)) / (2.0 * assign94530_e146154)), ((2.0 * (assign94530_e146150 * locals.var_beta_inv_dn6)) / (2.0 * assign94530_e146154)), ((2.0 * (assign94530_e146150 * locals.var_beta_inv_dn7)) / (2.0 * assign94530_e146154)), ((2.0 * (assign94530_e146150 * locals.var_beta_inv_dn8)) / (2.0 * assign94530_e146154)), ((2.0 * (assign94530_e146150 * locals.var_beta_inv_dn9)) / (2.0 * assign94530_e146154)), ((2.0 * (assign94530_e146150 * locals.var_beta_inv_dn10)) / (2.0 * assign94530_e146154)), ((2.0 * (assign94530_e146150 * locals.var_beta_inv_dn11)) / (2.0 * assign94530_e146154)), ((2.0 * (assign94530_e146150 * locals.var_beta_inv_dn14)) / (2.0 * assign94530_e146154)),)
    } else {
        (locals.var_c_w_ld, locals.var_c_w_ld_dn0, locals.var_c_w_ld_dn2, locals.var_c_w_ld_dn4, locals.var_c_w_ld_dn5, locals.var_c_w_ld_dn6, locals.var_c_w_ld_dn7, locals.var_c_w_ld_dn8, locals.var_c_w_ld_dn9, locals.var_c_w_ld_dn10, locals.var_c_w_ld_dn11, locals.var_c_w_ld_dn14,)
    }
};
        locals.var_c_w_ld = assign94530_e146156;
        locals.var_c_w_ld_dn0 = assign94530_e146156_d_n0;
        locals.var_c_w_ld_dn2 = assign94530_e146156_d_n2;
        locals.var_c_w_ld_dn4 = assign94530_e146156_d_n4;
        locals.var_c_w_ld_dn5 = assign94530_e146156_d_n5;
        locals.var_c_w_ld_dn6 = assign94530_e146156_d_n6;
        locals.var_c_w_ld_dn7 = assign94530_e146156_d_n7;
        locals.var_c_w_ld_dn8 = assign94530_e146156_d_n8;
        locals.var_c_w_ld_dn9 = assign94530_e146156_d_n9;
        locals.var_c_w_ld_dn10 = assign94530_e146156_d_n10;
        locals.var_c_w_ld_dn11 = assign94530_e146156_d_n11;
        locals.var_c_w_ld_dn14 = assign94530_e146156_d_n14;
        locals.var_c_w_ld_rv = 0.0;

        let assign94540_e146159: f64 = if locals.var_wdep_func > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2201 = assign94540_e146159;
        locals.var_guard2201_rv = 0.0;

        let (assign94550_e146171, assign94550_e146171_d_n0, assign94550_e146171_d_n2, assign94550_e146171_d_n4, assign94550_e146171_d_n5, assign94550_e146171_d_n6, assign94550_e146171_d_n7, assign94550_e146171_d_n8, assign94550_e146171_d_n9, assign94550_e146171_d_n10, assign94550_e146171_d_n11, assign94550_e146171_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2200 != 0.0)) && (locals.var_guard2201 != 0.0)) {
        let assign94550_e146169: f64 = (p.p334 - locals.var_wdep_func);
        (assign94550_e146169, (-locals.var_wdep_func_dn0), (-locals.var_wdep_func_dn2), (-locals.var_wdep_func_dn4), (-locals.var_wdep_func_dn5), (-locals.var_wdep_func_dn6), (-locals.var_wdep_func_dn7), (-locals.var_wdep_func_dn8), (-locals.var_wdep_func_dn9), (-locals.var_wdep_func_dn10), (-locals.var_wdep_func_dn11), (-locals.var_wdep_func_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign94550_e146171;
        locals.var_t2_dn0 = assign94550_e146171_d_n0;
        locals.var_t2_dn2 = assign94550_e146171_d_n2;
        locals.var_t2_dn4 = assign94550_e146171_d_n4;
        locals.var_t2_dn5 = assign94550_e146171_d_n5;
        locals.var_t2_dn6 = assign94550_e146171_d_n6;
        locals.var_t2_dn7 = assign94550_e146171_d_n7;
        locals.var_t2_dn8 = assign94550_e146171_d_n8;
        locals.var_t2_dn9 = assign94550_e146171_d_n9;
        locals.var_t2_dn10 = assign94550_e146171_d_n10;
        locals.var_t2_dn11 = assign94550_e146171_d_n11;
        locals.var_t2_dn14 = assign94550_e146171_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign94560_e146195, assign94560_e146195_d_n0, assign94560_e146195_d_n2, assign94560_e146195_d_n4, assign94560_e146195_d_n5, assign94560_e146195_d_n6, assign94560_e146195_d_n7, assign94560_e146195_d_n8, assign94560_e146195_d_n9, assign94560_e146195_d_n10, assign94560_e146195_d_n11, assign94560_e146195_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2200 != 0.0)) && (locals.var_guard2201 == 0.0)) {
        let assign94560_e146182: f64 = (locals.var_vdsi + p.p137);
        let assign94560_e146185: f64 = (locals.var_vdsi + p.p137);
        let assign94560_e146186: f64 = (assign94560_e146182 * assign94560_e146185);
        let assign94560_e146189: f64 = (4.0 * 0.1);
        let assign94560_e146191: f64 = (assign94560_e146189 * 0.1);
        let assign94560_e146192: f64 = (assign94560_e146186 + assign94560_e146191);
        let assign94560_e146193: f64 = (assign94560_e146192).sqrt();
        (assign94560_e146193, 0.0, 0.0, 0.0, 0.0, (((locals.var_vdsi_dn6 * assign94560_e146185) + (assign94560_e146182 * locals.var_vdsi_dn6)) / (2.0 * assign94560_e146193)), 0.0, (((locals.var_vdsi_dn8 * assign94560_e146185) + (assign94560_e146182 * locals.var_vdsi_dn8)) / (2.0 * assign94560_e146193)), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign94560_e146195;
        locals.var_tmf2_dn0 = assign94560_e146195_d_n0;
        locals.var_tmf2_dn2 = assign94560_e146195_d_n2;
        locals.var_tmf2_dn4 = assign94560_e146195_d_n4;
        locals.var_tmf2_dn5 = assign94560_e146195_d_n5;
        locals.var_tmf2_dn6 = assign94560_e146195_d_n6;
        locals.var_tmf2_dn7 = assign94560_e146195_d_n7;
        locals.var_tmf2_dn8 = assign94560_e146195_d_n8;
        locals.var_tmf2_dn9 = assign94560_e146195_d_n9;
        locals.var_tmf2_dn10 = assign94560_e146195_d_n10;
        locals.var_tmf2_dn11 = assign94560_e146195_d_n11;
        locals.var_tmf2_dn14 = assign94560_e146195_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign94570_e146214, assign94570_e146214_d_n0, assign94570_e146214_d_n2, assign94570_e146214_d_n4, assign94570_e146214_d_n5, assign94570_e146214_d_n6, assign94570_e146214_d_n7, assign94570_e146214_d_n8, assign94570_e146214_d_n9, assign94570_e146214_d_n10, assign94570_e146214_d_n11, assign94570_e146214_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2200 != 0.0)) && (locals.var_guard2201 == 0.0)) {
        let assign94570_e146208: f64 = (locals.var_vdsi + p.p137);
        let assign94570_e146210: f64 = (assign94570_e146208 / locals.var_tmf2);
        let assign94570_e146211: f64 = (1.0 + assign94570_e146210);
        let assign94570_e146212: f64 = (0.5 * assign94570_e146211);
        (assign94570_e146212, (0.5 * (-((assign94570_e146208 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign94570_e146208 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign94570_e146208 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign94570_e146208 * locals.var_tmf2_dn5) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (((locals.var_vdsi_dn6 * locals.var_tmf2) - (assign94570_e146208 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (-((assign94570_e146208 * locals.var_tmf2_dn7) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (((locals.var_vdsi_dn8 * locals.var_tmf2) - (assign94570_e146208 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (-((assign94570_e146208 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign94570_e146208 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign94570_e146208 * locals.var_tmf2_dn11) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign94570_e146208 * locals.var_tmf2_dn14) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign94570_e146214;
        locals.var_t9_dn0 = assign94570_e146214_d_n0;
        locals.var_t9_dn2 = assign94570_e146214_d_n2;
        locals.var_t9_dn4 = assign94570_e146214_d_n4;
        locals.var_t9_dn5 = assign94570_e146214_d_n5;
        locals.var_t9_dn6 = assign94570_e146214_d_n6;
        locals.var_t9_dn7 = assign94570_e146214_d_n7;
        locals.var_t9_dn8 = assign94570_e146214_d_n8;
        locals.var_t9_dn9 = assign94570_e146214_d_n9;
        locals.var_t9_dn10 = assign94570_e146214_d_n10;
        locals.var_t9_dn11 = assign94570_e146214_d_n11;
        locals.var_t9_dn14 = assign94570_e146214_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign94580_e146231, assign94580_e146231_d_n0, assign94580_e146231_d_n2, assign94580_e146231_d_n4, assign94580_e146231_d_n5, assign94580_e146231_d_n6, assign94580_e146231_d_n7, assign94580_e146231_d_n8, assign94580_e146231_d_n9, assign94580_e146231_d_n10, assign94580_e146231_d_n11, assign94580_e146231_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2200 != 0.0)) && (locals.var_guard2201 == 0.0)) {
        let assign94580_e146226: f64 = (locals.var_vdsi + p.p137);
        let assign94580_e146228: f64 = (assign94580_e146226 + locals.var_tmf2);
        let assign94580_e146229: f64 = (0.5 * assign94580_e146228);
        (assign94580_e146229, (0.5 * locals.var_tmf2_dn0), (0.5 * locals.var_tmf2_dn2), (0.5 * locals.var_tmf2_dn4), (0.5 * locals.var_tmf2_dn5), (0.5 * (locals.var_vdsi_dn6 + locals.var_tmf2_dn6)), (0.5 * locals.var_tmf2_dn7), (0.5 * (locals.var_vdsi_dn8 + locals.var_tmf2_dn8)), (0.5 * locals.var_tmf2_dn9), (0.5 * locals.var_tmf2_dn10), (0.5 * locals.var_tmf2_dn11), (0.5 * locals.var_tmf2_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign94580_e146231;
        locals.var_t2_dn0 = assign94580_e146231_d_n0;
        locals.var_t2_dn2 = assign94580_e146231_d_n2;
        locals.var_t2_dn4 = assign94580_e146231_d_n4;
        locals.var_t2_dn5 = assign94580_e146231_d_n5;
        locals.var_t2_dn6 = assign94580_e146231_d_n6;
        locals.var_t2_dn7 = assign94580_e146231_d_n7;
        locals.var_t2_dn8 = assign94580_e146231_d_n8;
        locals.var_t2_dn9 = assign94580_e146231_d_n9;
        locals.var_t2_dn10 = assign94580_e146231_d_n10;
        locals.var_t2_dn11 = assign94580_e146231_d_n11;
        locals.var_t2_dn14 = assign94580_e146231_d_n14;
        locals.var_t2_rv = 0.0;

        let assign94590_e146234: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2202 = assign94590_e146234;
        locals.var_guard2202_rv = 0.0;

        let (assign94600_e146247, assign94600_e146247_d_n0, assign94600_e146247_d_n2, assign94600_e146247_d_n4, assign94600_e146247_d_n5, assign94600_e146247_d_n6, assign94600_e146247_d_n7, assign94600_e146247_d_n8, assign94600_e146247_d_n9, assign94600_e146247_d_n10, assign94600_e146247_d_n11, assign94600_e146247_d_n14,) = {
    if (((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2200 != 0.0)) && (locals.var_guard2201 == 0.0)) && (locals.var_guard2202 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign94600_e146247;
        locals.var_t2_dn0 = assign94600_e146247_d_n0;
        locals.var_t2_dn2 = assign94600_e146247_d_n2;
        locals.var_t2_dn4 = assign94600_e146247_d_n4;
        locals.var_t2_dn5 = assign94600_e146247_d_n5;
        locals.var_t2_dn6 = assign94600_e146247_d_n6;
        locals.var_t2_dn7 = assign94600_e146247_d_n7;
        locals.var_t2_dn8 = assign94600_e146247_d_n8;
        locals.var_t2_dn9 = assign94600_e146247_d_n9;
        locals.var_t2_dn10 = assign94600_e146247_d_n10;
        locals.var_t2_dn11 = assign94600_e146247_d_n11;
        locals.var_t2_dn14 = assign94600_e146247_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign94610_e146260, assign94610_e146260_d_n0, assign94610_e146260_d_n2, assign94610_e146260_d_n4, assign94610_e146260_d_n5, assign94610_e146260_d_n6, assign94610_e146260_d_n7, assign94610_e146260_d_n8, assign94610_e146260_d_n9, assign94610_e146260_d_n10, assign94610_e146260_d_n11, assign94610_e146260_d_n14,) = {
    if (((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2200 != 0.0)) && (locals.var_guard2201 == 0.0)) && (locals.var_guard2202 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign94610_e146260;
        locals.var_t9_dn0 = assign94610_e146260_d_n0;
        locals.var_t9_dn2 = assign94610_e146260_d_n2;
        locals.var_t9_dn4 = assign94610_e146260_d_n4;
        locals.var_t9_dn5 = assign94610_e146260_d_n5;
        locals.var_t9_dn6 = assign94610_e146260_d_n6;
        locals.var_t9_dn7 = assign94610_e146260_d_n7;
        locals.var_t9_dn8 = assign94610_e146260_d_n8;
        locals.var_t9_dn9 = assign94610_e146260_d_n9;
        locals.var_t9_dn10 = assign94610_e146260_d_n10;
        locals.var_t9_dn11 = assign94610_e146260_d_n11;
        locals.var_t9_dn14 = assign94610_e146260_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign94620_e146276, assign94620_e146276_d_n0, assign94620_e146276_d_n2, assign94620_e146276_d_n4, assign94620_e146276_d_n5, assign94620_e146276_d_n6, assign94620_e146276_d_n7, assign94620_e146276_d_n8, assign94620_e146276_d_n9, assign94620_e146276_d_n10, assign94620_e146276_d_n11, assign94620_e146276_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2200 != 0.0)) && (locals.var_guard2201 == 0.0)) {
        let assign94620_e146271: f64 = (locals.var_kjunc * locals.var_t2);
        let assign94620_e146272: f64 = (assign94620_e146271).sqrt();
        let assign94620_e146274: f64 = (assign94620_e146272 * p.p432);
        (assign94620_e146274, ((((locals.var_kjunc_dn0 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn0)) / (2.0 * assign94620_e146272)) * p.p432), ((((locals.var_kjunc_dn2 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn2)) / (2.0 * assign94620_e146272)) * p.p432), ((((locals.var_kjunc_dn4 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn4)) / (2.0 * assign94620_e146272)) * p.p432), ((((locals.var_kjunc_dn5 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn5)) / (2.0 * assign94620_e146272)) * p.p432), ((((locals.var_kjunc_dn6 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn6)) / (2.0 * assign94620_e146272)) * p.p432), ((((locals.var_kjunc_dn7 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn7)) / (2.0 * assign94620_e146272)) * p.p432), ((((locals.var_kjunc_dn8 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn8)) / (2.0 * assign94620_e146272)) * p.p432), ((((locals.var_kjunc_dn9 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn9)) / (2.0 * assign94620_e146272)) * p.p432), ((((locals.var_kjunc_dn10 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn10)) / (2.0 * assign94620_e146272)) * p.p432), ((((locals.var_kjunc_dn11 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn11)) / (2.0 * assign94620_e146272)) * p.p432), ((((locals.var_kjunc_dn14 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn14)) / (2.0 * assign94620_e146272)) * p.p432),)
    } else {
        (locals.var_wjunc0, locals.var_wjunc0_dn0, locals.var_wjunc0_dn2, locals.var_wjunc0_dn4, locals.var_wjunc0_dn5, locals.var_wjunc0_dn6, locals.var_wjunc0_dn7, locals.var_wjunc0_dn8, locals.var_wjunc0_dn9, locals.var_wjunc0_dn10, locals.var_wjunc0_dn11, locals.var_wjunc0_dn14,)
    }
};
        locals.var_wjunc0 = assign94620_e146276;
        locals.var_wjunc0_dn0 = assign94620_e146276_d_n0;
        locals.var_wjunc0_dn2 = assign94620_e146276_d_n2;
        locals.var_wjunc0_dn4 = assign94620_e146276_d_n4;
        locals.var_wjunc0_dn5 = assign94620_e146276_d_n5;
        locals.var_wjunc0_dn6 = assign94620_e146276_d_n6;
        locals.var_wjunc0_dn7 = assign94620_e146276_d_n7;
        locals.var_wjunc0_dn8 = assign94620_e146276_d_n8;
        locals.var_wjunc0_dn9 = assign94620_e146276_d_n9;
        locals.var_wjunc0_dn10 = assign94620_e146276_d_n10;
        locals.var_wjunc0_dn11 = assign94620_e146276_d_n11;
        locals.var_wjunc0_dn14 = assign94620_e146276_d_n14;
        locals.var_wjunc0_rv = 0.0;

        let (assign94630_e146289, assign94630_e146289_d_n0, assign94630_e146289_d_n2, assign94630_e146289_d_n4, assign94630_e146289_d_n5, assign94630_e146289_d_n6, assign94630_e146289_d_n7, assign94630_e146289_d_n8, assign94630_e146289_d_n9, assign94630_e146289_d_n10, assign94630_e146289_d_n11, assign94630_e146289_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2200 != 0.0)) && (locals.var_guard2201 == 0.0)) {
        let assign94630_e146287: f64 = (p.p334 - locals.var_wjunc0);
        (assign94630_e146287, (-locals.var_wjunc0_dn0), (-locals.var_wjunc0_dn2), (-locals.var_wjunc0_dn4), (-locals.var_wjunc0_dn5), (-locals.var_wjunc0_dn6), (-locals.var_wjunc0_dn7), (-locals.var_wjunc0_dn8), (-locals.var_wjunc0_dn9), (-locals.var_wjunc0_dn10), (-locals.var_wjunc0_dn11), (-locals.var_wjunc0_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign94630_e146289;
        locals.var_t2_dn0 = assign94630_e146289_d_n0;
        locals.var_t2_dn2 = assign94630_e146289_d_n2;
        locals.var_t2_dn4 = assign94630_e146289_d_n4;
        locals.var_t2_dn5 = assign94630_e146289_d_n5;
        locals.var_t2_dn6 = assign94630_e146289_d_n6;
        locals.var_t2_dn7 = assign94630_e146289_d_n7;
        locals.var_t2_dn8 = assign94630_e146289_d_n8;
        locals.var_t2_dn9 = assign94630_e146289_d_n9;
        locals.var_t2_dn10 = assign94630_e146289_d_n10;
        locals.var_t2_dn11 = assign94630_e146289_d_n11;
        locals.var_t2_dn14 = assign94630_e146289_d_n14;
        locals.var_t2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_366(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign94640_e146310, assign94640_e146310_d_n0, assign94640_e146310_d_n2, assign94640_e146310_d_n4, assign94640_e146310_d_n5, assign94640_e146310_d_n6, assign94640_e146310_d_n7, assign94640_e146310_d_n8, assign94640_e146310_d_n9, assign94640_e146310_d_n10, assign94640_e146310_d_n11, assign94640_e146310_d_n14,) = {
    if (((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2200 != 0.0)) {
        let assign94640_e146297: f64 = (locals.var_t2 * locals.var_t2);
        let assign94640_e146301: f64 = (p.p334 * 0.01);
        let assign94640_e146302: f64 = (4.0 * assign94640_e146301);
        let assign94640_e146305: f64 = (p.p334 * 0.01);
        let assign94640_e146306: f64 = (assign94640_e146302 * assign94640_e146305);
        let assign94640_e146307: f64 = (assign94640_e146297 + assign94640_e146306);
        let assign94640_e146308: f64 = (assign94640_e146307).sqrt();
        (assign94640_e146308, (((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)) / (2.0 * assign94640_e146308)), (((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)) / (2.0 * assign94640_e146308)), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign94640_e146308)), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign94640_e146308)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign94640_e146308)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign94640_e146308)), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign94640_e146308)), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign94640_e146308)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign94640_e146308)), (((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign94640_e146308)), (((locals.var_t2_dn14 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn14)) / (2.0 * assign94640_e146308)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign94640_e146310;
        locals.var_tmf2_dn0 = assign94640_e146310_d_n0;
        locals.var_tmf2_dn2 = assign94640_e146310_d_n2;
        locals.var_tmf2_dn4 = assign94640_e146310_d_n4;
        locals.var_tmf2_dn5 = assign94640_e146310_d_n5;
        locals.var_tmf2_dn6 = assign94640_e146310_d_n6;
        locals.var_tmf2_dn7 = assign94640_e146310_d_n7;
        locals.var_tmf2_dn8 = assign94640_e146310_d_n8;
        locals.var_tmf2_dn9 = assign94640_e146310_d_n9;
        locals.var_tmf2_dn10 = assign94640_e146310_d_n10;
        locals.var_tmf2_dn11 = assign94640_e146310_d_n11;
        locals.var_tmf2_dn14 = assign94640_e146310_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign94650_e146324, assign94650_e146324_d_n0, assign94650_e146324_d_n2, assign94650_e146324_d_n4, assign94650_e146324_d_n5, assign94650_e146324_d_n6, assign94650_e146324_d_n7, assign94650_e146324_d_n8, assign94650_e146324_d_n9, assign94650_e146324_d_n10, assign94650_e146324_d_n11, assign94650_e146324_d_n14,) = {
    if (((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2200 != 0.0)) {
        let assign94650_e146320: f64 = (locals.var_t2 / locals.var_tmf2);
        let assign94650_e146321: f64 = (1.0 + assign94650_e146320);
        let assign94650_e146322: f64 = (0.5 * assign94650_e146321);
        (assign94650_e146322, (0.5 * (((locals.var_t2_dn0 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn2 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn4 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn5 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn6 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn7 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn8 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn9 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn10 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn11 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn14 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign94650_e146324;
        locals.var_t9_dn0 = assign94650_e146324_d_n0;
        locals.var_t9_dn2 = assign94650_e146324_d_n2;
        locals.var_t9_dn4 = assign94650_e146324_d_n4;
        locals.var_t9_dn5 = assign94650_e146324_d_n5;
        locals.var_t9_dn6 = assign94650_e146324_d_n6;
        locals.var_t9_dn7 = assign94650_e146324_d_n7;
        locals.var_t9_dn8 = assign94650_e146324_d_n8;
        locals.var_t9_dn9 = assign94650_e146324_d_n9;
        locals.var_t9_dn10 = assign94650_e146324_d_n10;
        locals.var_t9_dn11 = assign94650_e146324_d_n11;
        locals.var_t9_dn14 = assign94650_e146324_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign94660_e146336, assign94660_e146336_d_n0, assign94660_e146336_d_n2, assign94660_e146336_d_n4, assign94660_e146336_d_n5, assign94660_e146336_d_n6, assign94660_e146336_d_n7, assign94660_e146336_d_n8, assign94660_e146336_d_n9, assign94660_e146336_d_n10, assign94660_e146336_d_n11, assign94660_e146336_d_n14,) = {
    if (((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2200 != 0.0)) {
        let assign94660_e146333: f64 = (locals.var_t2 + locals.var_tmf2);
        let assign94660_e146334: f64 = (0.5 * assign94660_e146333);
        (assign94660_e146334, (0.5 * (locals.var_t2_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t2_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t2_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t2_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t2_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t2_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t2_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t2_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t2_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t2_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_t2_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign94660_e146336;
        locals.var_t2_dn0 = assign94660_e146336_d_n0;
        locals.var_t2_dn2 = assign94660_e146336_d_n2;
        locals.var_t2_dn4 = assign94660_e146336_d_n4;
        locals.var_t2_dn5 = assign94660_e146336_d_n5;
        locals.var_t2_dn6 = assign94660_e146336_d_n6;
        locals.var_t2_dn7 = assign94660_e146336_d_n7;
        locals.var_t2_dn8 = assign94660_e146336_d_n8;
        locals.var_t2_dn9 = assign94660_e146336_d_n9;
        locals.var_t2_dn10 = assign94660_e146336_d_n10;
        locals.var_t2_dn11 = assign94660_e146336_d_n11;
        locals.var_t2_dn14 = assign94660_e146336_d_n14;
        locals.var_t2_rv = 0.0;

        let assign94670_e146339: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2203 = assign94670_e146339;
        locals.var_guard2203_rv = 0.0;

        let (assign94680_e146349, assign94680_e146349_d_n0, assign94680_e146349_d_n2, assign94680_e146349_d_n4, assign94680_e146349_d_n5, assign94680_e146349_d_n6, assign94680_e146349_d_n7, assign94680_e146349_d_n8, assign94680_e146349_d_n9, assign94680_e146349_d_n10, assign94680_e146349_d_n11, assign94680_e146349_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2200 != 0.0)) && (locals.var_guard2203 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign94680_e146349;
        locals.var_t2_dn0 = assign94680_e146349_d_n0;
        locals.var_t2_dn2 = assign94680_e146349_d_n2;
        locals.var_t2_dn4 = assign94680_e146349_d_n4;
        locals.var_t2_dn5 = assign94680_e146349_d_n5;
        locals.var_t2_dn6 = assign94680_e146349_d_n6;
        locals.var_t2_dn7 = assign94680_e146349_d_n7;
        locals.var_t2_dn8 = assign94680_e146349_d_n8;
        locals.var_t2_dn9 = assign94680_e146349_d_n9;
        locals.var_t2_dn10 = assign94680_e146349_d_n10;
        locals.var_t2_dn11 = assign94680_e146349_d_n11;
        locals.var_t2_dn14 = assign94680_e146349_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign94690_e146359, assign94690_e146359_d_n0, assign94690_e146359_d_n2, assign94690_e146359_d_n4, assign94690_e146359_d_n5, assign94690_e146359_d_n6, assign94690_e146359_d_n7, assign94690_e146359_d_n8, assign94690_e146359_d_n9, assign94690_e146359_d_n10, assign94690_e146359_d_n11, assign94690_e146359_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2200 != 0.0)) && (locals.var_guard2203 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign94690_e146359;
        locals.var_t9_dn0 = assign94690_e146359_d_n0;
        locals.var_t9_dn2 = assign94690_e146359_d_n2;
        locals.var_t9_dn4 = assign94690_e146359_d_n4;
        locals.var_t9_dn5 = assign94690_e146359_d_n5;
        locals.var_t9_dn6 = assign94690_e146359_d_n6;
        locals.var_t9_dn7 = assign94690_e146359_d_n7;
        locals.var_t9_dn8 = assign94690_e146359_d_n8;
        locals.var_t9_dn9 = assign94690_e146359_d_n9;
        locals.var_t9_dn10 = assign94690_e146359_d_n10;
        locals.var_t9_dn11 = assign94690_e146359_d_n11;
        locals.var_t9_dn14 = assign94690_e146359_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign94700_e146367, assign94700_e146367_d_n0, assign94700_e146367_d_n2, assign94700_e146367_d_n4, assign94700_e146367_d_n5, assign94700_e146367_d_n6, assign94700_e146367_d_n7, assign94700_e146367_d_n8, assign94700_e146367_d_n9, assign94700_e146367_d_n10, assign94700_e146367_d_n11, assign94700_e146367_d_n14,) = {
    if (((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2200 != 0.0)) {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    } else {
        (locals.var_ddriftldc, locals.var_ddriftldc_dn0, locals.var_ddriftldc_dn2, locals.var_ddriftldc_dn4, locals.var_ddriftldc_dn5, locals.var_ddriftldc_dn6, locals.var_ddriftldc_dn7, locals.var_ddriftldc_dn8, locals.var_ddriftldc_dn9, locals.var_ddriftldc_dn10, locals.var_ddriftldc_dn11, locals.var_ddriftldc_dn14,)
    }
};
        locals.var_ddriftldc = assign94700_e146367;
        locals.var_ddriftldc_dn0 = assign94700_e146367_d_n0;
        locals.var_ddriftldc_dn2 = assign94700_e146367_d_n2;
        locals.var_ddriftldc_dn4 = assign94700_e146367_d_n4;
        locals.var_ddriftldc_dn5 = assign94700_e146367_d_n5;
        locals.var_ddriftldc_dn6 = assign94700_e146367_d_n6;
        locals.var_ddriftldc_dn7 = assign94700_e146367_d_n7;
        locals.var_ddriftldc_dn8 = assign94700_e146367_d_n8;
        locals.var_ddriftldc_dn9 = assign94700_e146367_d_n9;
        locals.var_ddriftldc_dn10 = assign94700_e146367_d_n10;
        locals.var_ddriftldc_dn11 = assign94700_e146367_d_n11;
        locals.var_ddriftldc_dn14 = assign94700_e146367_d_n14;
        locals.var_ddriftldc_rv = 0.0;

        let (assign94710_e146383, assign94710_e146383_d_n0, assign94710_e146383_d_n2, assign94710_e146383_d_n4, assign94710_e146383_d_n5, assign94710_e146383_d_n6, assign94710_e146383_d_n7, assign94710_e146383_d_n8, assign94710_e146383_d_n9, assign94710_e146383_d_n10, assign94710_e146383_d_n11, assign94710_e146383_d_n14,) = {
    if (((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2200 != 0.0)) {
        let assign94710_e146375: f64 = (locals.var_q_nsubld__blk2119 * locals.var_ddriftldc);
        let assign94710_e146377: f64 = (assign94710_e146375 * locals.var_ddriftldc);
        let assign94710_e146379: f64 = (assign94710_e146377 / 2.0);
        let assign94710_e146381: f64 = (assign94710_e146379 / 1.034943e-10);
        (assign94710_e146381, (((((locals.var_q_nsubld__blk2119 * locals.var_ddriftldc_dn0) * locals.var_ddriftldc) + (assign94710_e146375 * locals.var_ddriftldc_dn0)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2119 * locals.var_ddriftldc_dn2) * locals.var_ddriftldc) + (assign94710_e146375 * locals.var_ddriftldc_dn2)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2119 * locals.var_ddriftldc_dn4) * locals.var_ddriftldc) + (assign94710_e146375 * locals.var_ddriftldc_dn4)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2119 * locals.var_ddriftldc_dn5) * locals.var_ddriftldc) + (assign94710_e146375 * locals.var_ddriftldc_dn5)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2119 * locals.var_ddriftldc_dn6) * locals.var_ddriftldc) + (assign94710_e146375 * locals.var_ddriftldc_dn6)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2119 * locals.var_ddriftldc_dn7) * locals.var_ddriftldc) + (assign94710_e146375 * locals.var_ddriftldc_dn7)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2119 * locals.var_ddriftldc_dn8) * locals.var_ddriftldc) + (assign94710_e146375 * locals.var_ddriftldc_dn8)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2119 * locals.var_ddriftldc_dn9) * locals.var_ddriftldc) + (assign94710_e146375 * locals.var_ddriftldc_dn9)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2119 * locals.var_ddriftldc_dn10) * locals.var_ddriftldc) + (assign94710_e146375 * locals.var_ddriftldc_dn10)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2119 * locals.var_ddriftldc_dn11) * locals.var_ddriftldc) + (assign94710_e146375 * locals.var_ddriftldc_dn11)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2119 * locals.var_ddriftldc_dn14) * locals.var_ddriftldc) + (assign94710_e146375 * locals.var_ddriftldc_dn14)) / 2.0) / 1.034943e-10),)
    } else {
        (locals.var_dphi_sb, locals.var_dphi_sb_dn0, locals.var_dphi_sb_dn2, locals.var_dphi_sb_dn4, locals.var_dphi_sb_dn5, locals.var_dphi_sb_dn6, locals.var_dphi_sb_dn7, locals.var_dphi_sb_dn8, locals.var_dphi_sb_dn9, locals.var_dphi_sb_dn10, locals.var_dphi_sb_dn11, locals.var_dphi_sb_dn14,)
    }
};
        locals.var_dphi_sb = assign94710_e146383;
        locals.var_dphi_sb_dn0 = assign94710_e146383_d_n0;
        locals.var_dphi_sb_dn2 = assign94710_e146383_d_n2;
        locals.var_dphi_sb_dn4 = assign94710_e146383_d_n4;
        locals.var_dphi_sb_dn5 = assign94710_e146383_d_n5;
        locals.var_dphi_sb_dn6 = assign94710_e146383_d_n6;
        locals.var_dphi_sb_dn7 = assign94710_e146383_d_n7;
        locals.var_dphi_sb_dn8 = assign94710_e146383_d_n8;
        locals.var_dphi_sb_dn9 = assign94710_e146383_d_n9;
        locals.var_dphi_sb_dn10 = assign94710_e146383_d_n10;
        locals.var_dphi_sb_dn11 = assign94710_e146383_d_n11;
        locals.var_dphi_sb_dn14 = assign94710_e146383_d_n14;
        locals.var_dphi_sb_rv = 0.0;

        let (assign94720_e146396, assign94720_e146396_d_n0, assign94720_e146396_d_n2, assign94720_e146396_d_n4, assign94720_e146396_d_n5, assign94720_e146396_d_n6, assign94720_e146396_d_n7, assign94720_e146396_d_n8, assign94720_e146396_d_n9, assign94720_e146396_d_n10, assign94720_e146396_d_n11, assign94720_e146396_d_n14,) = {
    if (((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2200 != 0.0)) {
        let assign94720_e146391: f64 = (2.0 * locals.var_beta);
        let assign94720_e146393: f64 = (assign94720_e146391 * locals.var_dphi_sb);
        let assign94720_e146394: f64 = (assign94720_e146393).sqrt();
        (assign94720_e146394, ((((2.0 * locals.var_beta_dn0) * locals.var_dphi_sb) + (assign94720_e146391 * locals.var_dphi_sb_dn0)) / (2.0 * assign94720_e146394)), ((((2.0 * locals.var_beta_dn2) * locals.var_dphi_sb) + (assign94720_e146391 * locals.var_dphi_sb_dn2)) / (2.0 * assign94720_e146394)), ((((2.0 * locals.var_beta_dn4) * locals.var_dphi_sb) + (assign94720_e146391 * locals.var_dphi_sb_dn4)) / (2.0 * assign94720_e146394)), ((((2.0 * locals.var_beta_dn5) * locals.var_dphi_sb) + (assign94720_e146391 * locals.var_dphi_sb_dn5)) / (2.0 * assign94720_e146394)), ((((2.0 * locals.var_beta_dn6) * locals.var_dphi_sb) + (assign94720_e146391 * locals.var_dphi_sb_dn6)) / (2.0 * assign94720_e146394)), ((((2.0 * locals.var_beta_dn7) * locals.var_dphi_sb) + (assign94720_e146391 * locals.var_dphi_sb_dn7)) / (2.0 * assign94720_e146394)), ((((2.0 * locals.var_beta_dn8) * locals.var_dphi_sb) + (assign94720_e146391 * locals.var_dphi_sb_dn8)) / (2.0 * assign94720_e146394)), ((((2.0 * locals.var_beta_dn9) * locals.var_dphi_sb) + (assign94720_e146391 * locals.var_dphi_sb_dn9)) / (2.0 * assign94720_e146394)), ((((2.0 * locals.var_beta_dn10) * locals.var_dphi_sb) + (assign94720_e146391 * locals.var_dphi_sb_dn10)) / (2.0 * assign94720_e146394)), ((((2.0 * locals.var_beta_dn11) * locals.var_dphi_sb) + (assign94720_e146391 * locals.var_dphi_sb_dn11)) / (2.0 * assign94720_e146394)), ((((2.0 * locals.var_beta_dn14) * locals.var_dphi_sb) + (assign94720_e146391 * locals.var_dphi_sb_dn14)) / (2.0 * assign94720_e146394)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign94720_e146396;
        locals.var_t0_dn0 = assign94720_e146396_d_n0;
        locals.var_t0_dn2 = assign94720_e146396_d_n2;
        locals.var_t0_dn4 = assign94720_e146396_d_n4;
        locals.var_t0_dn5 = assign94720_e146396_d_n5;
        locals.var_t0_dn6 = assign94720_e146396_d_n6;
        locals.var_t0_dn7 = assign94720_e146396_d_n7;
        locals.var_t0_dn8 = assign94720_e146396_d_n8;
        locals.var_t0_dn9 = assign94720_e146396_d_n9;
        locals.var_t0_dn10 = assign94720_e146396_d_n10;
        locals.var_t0_dn11 = assign94720_e146396_d_n11;
        locals.var_t0_dn14 = assign94720_e146396_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign94730_e146411, assign94730_e146411_d_n0, assign94730_e146411_d_n2, assign94730_e146411_d_n4, assign94730_e146411_d_n5, assign94730_e146411_d_n6, assign94730_e146411_d_n7, assign94730_e146411_d_n8, assign94730_e146411_d_n9, assign94730_e146411_d_n10, assign94730_e146411_d_n11, assign94730_e146411_d_n14,) = {
    if (((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2200 != 0.0)) {
        let assign94730_e146403: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign94730_e146405: f64 = (-locals.var_t0);
        let assign94730_e146406: f64 = { let limited_exp_arg = assign94730_e146405; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign94730_e146407: f64 = (assign94730_e146403 + assign94730_e146406);
        let assign94730_e146409: f64 = (assign94730_e146407 / 2.0);
        (assign94730_e146409, ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn0) + ({ let limited_exp_arg = assign94730_e146405; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn0))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn2) + ({ let limited_exp_arg = assign94730_e146405; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn2))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4) + ({ let limited_exp_arg = assign94730_e146405; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn4))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5) + ({ let limited_exp_arg = assign94730_e146405; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn5))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6) + ({ let limited_exp_arg = assign94730_e146405; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn6))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7) + ({ let limited_exp_arg = assign94730_e146405; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn7))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8) + ({ let limited_exp_arg = assign94730_e146405; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn8))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9) + ({ let limited_exp_arg = assign94730_e146405; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn9))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10) + ({ let limited_exp_arg = assign94730_e146405; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn10))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11) + ({ let limited_exp_arg = assign94730_e146405; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn11))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn14) + ({ let limited_exp_arg = assign94730_e146405; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn14))) / 2.0),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign94730_e146411;
        locals.var_t1_dn0 = assign94730_e146411_d_n0;
        locals.var_t1_dn2 = assign94730_e146411_d_n2;
        locals.var_t1_dn4 = assign94730_e146411_d_n4;
        locals.var_t1_dn5 = assign94730_e146411_d_n5;
        locals.var_t1_dn6 = assign94730_e146411_d_n6;
        locals.var_t1_dn7 = assign94730_e146411_d_n7;
        locals.var_t1_dn8 = assign94730_e146411_d_n8;
        locals.var_t1_dn9 = assign94730_e146411_d_n9;
        locals.var_t1_dn10 = assign94730_e146411_d_n10;
        locals.var_t1_dn11 = assign94730_e146411_d_n11;
        locals.var_t1_dn14 = assign94730_e146411_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign94740_e146422, assign94740_e146422_d_n0, assign94740_e146422_d_n2, assign94740_e146422_d_n4, assign94740_e146422_d_n5, assign94740_e146422_d_n6, assign94740_e146422_d_n7, assign94740_e146422_d_n8, assign94740_e146422_d_n9, assign94740_e146422_d_n10, assign94740_e146422_d_n11, assign94740_e146422_d_n14,) = {
    if (((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2200 != 0.0)) {
        let assign94740_e146418: f64 = (locals.var_t1).ln();
        let assign94740_e146420: f64 = (assign94740_e146418 / locals.var_dphi_sb);
        (assign94740_e146420, ((((locals.var_t1_dn0 / locals.var_t1) * locals.var_dphi_sb) - (assign94740_e146418 * locals.var_dphi_sb_dn0)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn2 / locals.var_t1) * locals.var_dphi_sb) - (assign94740_e146418 * locals.var_dphi_sb_dn2)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn4 / locals.var_t1) * locals.var_dphi_sb) - (assign94740_e146418 * locals.var_dphi_sb_dn4)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn5 / locals.var_t1) * locals.var_dphi_sb) - (assign94740_e146418 * locals.var_dphi_sb_dn5)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn6 / locals.var_t1) * locals.var_dphi_sb) - (assign94740_e146418 * locals.var_dphi_sb_dn6)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn7 / locals.var_t1) * locals.var_dphi_sb) - (assign94740_e146418 * locals.var_dphi_sb_dn7)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn8 / locals.var_t1) * locals.var_dphi_sb) - (assign94740_e146418 * locals.var_dphi_sb_dn8)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn9 / locals.var_t1) * locals.var_dphi_sb) - (assign94740_e146418 * locals.var_dphi_sb_dn9)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn10 / locals.var_t1) * locals.var_dphi_sb) - (assign94740_e146418 * locals.var_dphi_sb_dn10)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn11 / locals.var_t1) * locals.var_dphi_sb) - (assign94740_e146418 * locals.var_dphi_sb_dn11)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn14 / locals.var_t1) * locals.var_dphi_sb) - (assign94740_e146418 * locals.var_dphi_sb_dn14)) / (locals.var_dphi_sb * locals.var_dphi_sb)),)
    } else {
        (locals.var_c_sb, locals.var_c_sb_dn0, locals.var_c_sb_dn2, locals.var_c_sb_dn4, locals.var_c_sb_dn5, locals.var_c_sb_dn6, locals.var_c_sb_dn7, locals.var_c_sb_dn8, locals.var_c_sb_dn9, locals.var_c_sb_dn10, locals.var_c_sb_dn11, locals.var_c_sb_dn14,)
    }
};
        locals.var_c_sb = assign94740_e146422;
        locals.var_c_sb_dn0 = assign94740_e146422_d_n0;
        locals.var_c_sb_dn2 = assign94740_e146422_d_n2;
        locals.var_c_sb_dn4 = assign94740_e146422_d_n4;
        locals.var_c_sb_dn5 = assign94740_e146422_d_n5;
        locals.var_c_sb_dn6 = assign94740_e146422_d_n6;
        locals.var_c_sb_dn7 = assign94740_e146422_d_n7;
        locals.var_c_sb_dn8 = assign94740_e146422_d_n8;
        locals.var_c_sb_dn9 = assign94740_e146422_d_n9;
        locals.var_c_sb_dn10 = assign94740_e146422_d_n10;
        locals.var_c_sb_dn11 = assign94740_e146422_d_n11;
        locals.var_c_sb_dn14 = assign94740_e146422_d_n14;
        locals.var_c_sb_rv = 0.0;

        let (assign94750_e146430,) = {
    if (((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2200 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign94750_e146430;
        locals.var_lp_s0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_367(
        locals: &mut StampLocals,
    ) {
        let mut assign94760_loop_guard: usize = 0;
        while {
            let assign94760_cond_e146439: f64 = (locals.var_lp_s0_max + 1.0);
            let assign94760_cond_e146441: f64 = if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2200 != 0.0)) && (locals.var_lp_s0 <= assign94760_cond_e146439)) { 1.0 } else { 0.0 };
            assign94760_cond_e146441 != 0.0
        } {
            assign94760_loop_guard += 1;
            assert!(assign94760_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign94760_body3_e146474, assign94760_body3_e146474_d_n0, assign94760_body3_e146474_d_n2, assign94760_body3_e146474_d_n4, assign94760_body3_e146474_d_n5, assign94760_body3_e146474_d_n6, assign94760_body3_e146474_d_n7, assign94760_body3_e146474_d_n8, assign94760_body3_e146474_d_n9, assign94760_body3_e146474_d_n10, assign94760_body3_e146474_d_n11, assign94760_body3_e146474_d_n14,) = {
    if (((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2200 != 0.0)) {
        let assign94760_body3_e146472: f64 = (locals.var_ps0ld + locals.var_vxbgmtcl);
        (assign94760_body3_e146472, (locals.var_ps0ld_dn0 + locals.var_vxbgmtcl_dn0), (locals.var_ps0ld_dn2 + locals.var_vxbgmtcl_dn2), (locals.var_ps0ld_dn4 + locals.var_vxbgmtcl_dn4), (locals.var_ps0ld_dn5 + locals.var_vxbgmtcl_dn5), (locals.var_ps0ld_dn6 + locals.var_vxbgmtcl_dn6), (locals.var_ps0ld_dn7 + locals.var_vxbgmtcl_dn7), (locals.var_ps0ld_dn8 + locals.var_vxbgmtcl_dn8), (locals.var_ps0ld_dn9 + locals.var_vxbgmtcl_dn9), (locals.var_ps0ld_dn10 + locals.var_vxbgmtcl_dn10), (locals.var_ps0ld_dn11 + locals.var_vxbgmtcl_dn11), (locals.var_ps0ld_dn14 + locals.var_vxbgmtcl_dn14),)
    } else {
        (locals.var_ps0ld_vxb, locals.var_ps0ld_vxb_dn0, locals.var_ps0ld_vxb_dn2, locals.var_ps0ld_vxb_dn4, locals.var_ps0ld_vxb_dn5, locals.var_ps0ld_vxb_dn6, locals.var_ps0ld_vxb_dn7, locals.var_ps0ld_vxb_dn8, locals.var_ps0ld_vxb_dn9, locals.var_ps0ld_vxb_dn10, locals.var_ps0ld_vxb_dn11, locals.var_ps0ld_vxb_dn14,)
    }
};
            locals.var_ps0ld_vxb = assign94760_body3_e146474;
            locals.var_ps0ld_vxb_dn0 = assign94760_body3_e146474_d_n0;
            locals.var_ps0ld_vxb_dn2 = assign94760_body3_e146474_d_n2;
            locals.var_ps0ld_vxb_dn4 = assign94760_body3_e146474_d_n4;
            locals.var_ps0ld_vxb_dn5 = assign94760_body3_e146474_d_n5;
            locals.var_ps0ld_vxb_dn6 = assign94760_body3_e146474_d_n6;
            locals.var_ps0ld_vxb_dn7 = assign94760_body3_e146474_d_n7;
            locals.var_ps0ld_vxb_dn8 = assign94760_body3_e146474_d_n8;
            locals.var_ps0ld_vxb_dn9 = assign94760_body3_e146474_d_n9;
            locals.var_ps0ld_vxb_dn10 = assign94760_body3_e146474_d_n10;
            locals.var_ps0ld_vxb_dn11 = assign94760_body3_e146474_d_n11;
            locals.var_ps0ld_vxb_dn14 = assign94760_body3_e146474_d_n14;
            locals.var_ps0ld_vxb_rv = 0.0;
            let (assign94760_body4_e146484, assign94760_body4_e146484_d_n0, assign94760_body4_e146484_d_n2, assign94760_body4_e146484_d_n4, assign94760_body4_e146484_d_n5, assign94760_body4_e146484_d_n6, assign94760_body4_e146484_d_n7, assign94760_body4_e146484_d_n8, assign94760_body4_e146484_d_n9, assign94760_body4_e146484_d_n10, assign94760_body4_e146484_d_n11, assign94760_body4_e146484_d_n14,) = {
    if (((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2200 != 0.0)) {
        let assign94760_body4_e146482: f64 = (locals.var_beta * locals.var_ps0ld_vxb);
        (assign94760_body4_e146482, ((locals.var_beta_dn0 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn0)), ((locals.var_beta_dn2 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn2)), ((locals.var_beta_dn4 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn4)), ((locals.var_beta_dn5 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn5)), ((locals.var_beta_dn6 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn6)), ((locals.var_beta_dn7 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn7)), ((locals.var_beta_dn8 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn8)), ((locals.var_beta_dn9 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn9)), ((locals.var_beta_dn10 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn10)), ((locals.var_beta_dn11 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn11)), ((locals.var_beta_dn14 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn14)),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
            locals.var_chi = assign94760_body4_e146484;
            locals.var_chi_dn0 = assign94760_body4_e146484_d_n0;
            locals.var_chi_dn2 = assign94760_body4_e146484_d_n2;
            locals.var_chi_dn4 = assign94760_body4_e146484_d_n4;
            locals.var_chi_dn5 = assign94760_body4_e146484_d_n5;
            locals.var_chi_dn6 = assign94760_body4_e146484_d_n6;
            locals.var_chi_dn7 = assign94760_body4_e146484_d_n7;
            locals.var_chi_dn8 = assign94760_body4_e146484_d_n8;
            locals.var_chi_dn9 = assign94760_body4_e146484_d_n9;
            locals.var_chi_dn10 = assign94760_body4_e146484_d_n10;
            locals.var_chi_dn11 = assign94760_body4_e146484_d_n11;
            locals.var_chi_dn14 = assign94760_body4_e146484_d_n14;
            locals.var_chi_rv = 0.0;
            let (assign94760_body5_e146496, assign94760_body5_e146496_d_n0, assign94760_body5_e146496_d_n2, assign94760_body5_e146496_d_n4, assign94760_body5_e146496_d_n5, assign94760_body5_e146496_d_n6, assign94760_body5_e146496_d_n7, assign94760_body5_e146496_d_n8, assign94760_body5_e146496_d_n9, assign94760_body5_e146496_d_n10, assign94760_body5_e146496_d_n11, assign94760_body5_e146496_d_n14,) = {
    if (((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2200 != 0.0)) {
        let assign94760_body5_e146493: f64 = (locals.var_ps0ld_vxb - locals.var_dphi_sb);
        let assign94760_body5_e146494: f64 = (locals.var_c_sb * assign94760_body5_e146493);
        (assign94760_body5_e146494, ((locals.var_c_sb_dn0 * assign94760_body5_e146493) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn0 - locals.var_dphi_sb_dn0))), ((locals.var_c_sb_dn2 * assign94760_body5_e146493) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn2 - locals.var_dphi_sb_dn2))), ((locals.var_c_sb_dn4 * assign94760_body5_e146493) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn4 - locals.var_dphi_sb_dn4))), ((locals.var_c_sb_dn5 * assign94760_body5_e146493) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn5 - locals.var_dphi_sb_dn5))), ((locals.var_c_sb_dn6 * assign94760_body5_e146493) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn6 - locals.var_dphi_sb_dn6))), ((locals.var_c_sb_dn7 * assign94760_body5_e146493) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn7 - locals.var_dphi_sb_dn7))), ((locals.var_c_sb_dn8 * assign94760_body5_e146493) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn8 - locals.var_dphi_sb_dn8))), ((locals.var_c_sb_dn9 * assign94760_body5_e146493) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn9 - locals.var_dphi_sb_dn9))), ((locals.var_c_sb_dn10 * assign94760_body5_e146493) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn10 - locals.var_dphi_sb_dn10))), ((locals.var_c_sb_dn11 * assign94760_body5_e146493) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn11 - locals.var_dphi_sb_dn11))), ((locals.var_c_sb_dn14 * assign94760_body5_e146493) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn14 - locals.var_dphi_sb_dn14))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn14,)
    }
};
            locals.var_ty = assign94760_body5_e146496;
            locals.var_ty_dn0 = assign94760_body5_e146496_d_n0;
            locals.var_ty_dn2 = assign94760_body5_e146496_d_n2;
            locals.var_ty_dn4 = assign94760_body5_e146496_d_n4;
            locals.var_ty_dn5 = assign94760_body5_e146496_d_n5;
            locals.var_ty_dn6 = assign94760_body5_e146496_d_n6;
            locals.var_ty_dn7 = assign94760_body5_e146496_d_n7;
            locals.var_ty_dn8 = assign94760_body5_e146496_d_n8;
            locals.var_ty_dn9 = assign94760_body5_e146496_d_n9;
            locals.var_ty_dn10 = assign94760_body5_e146496_d_n10;
            locals.var_ty_dn11 = assign94760_body5_e146496_d_n11;
            locals.var_ty_dn14 = assign94760_body5_e146496_d_n14;
            locals.var_ty_rv = 0.0;
            let assign94760_body6_e146499: f64 = if locals.var_ty < 60.0 { 1.0 } else { 0.0 };
            locals.var_guard2205 = assign94760_body6_e146499;
            locals.var_guard2205_rv = 0.0;
            let (assign94760_body7_e146510, assign94760_body7_e146510_d_n0, assign94760_body7_e146510_d_n2, assign94760_body7_e146510_d_n4, assign94760_body7_e146510_d_n5, assign94760_body7_e146510_d_n6, assign94760_body7_e146510_d_n7, assign94760_body7_e146510_d_n8, assign94760_body7_e146510_d_n9, assign94760_body7_e146510_d_n10, assign94760_body7_e146510_d_n11, assign94760_body7_e146510_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2200 != 0.0)) && (locals.var_guard2205 != 0.0)) {
        let assign94760_body7_e146508: f64 = (locals.var_ty).exp();
        (assign94760_body7_e146508, (assign94760_body7_e146508 * locals.var_ty_dn0), (assign94760_body7_e146508 * locals.var_ty_dn2), (assign94760_body7_e146508 * locals.var_ty_dn4), (assign94760_body7_e146508 * locals.var_ty_dn5), (assign94760_body7_e146508 * locals.var_ty_dn6), (assign94760_body7_e146508 * locals.var_ty_dn7), (assign94760_body7_e146508 * locals.var_ty_dn8), (assign94760_body7_e146508 * locals.var_ty_dn9), (assign94760_body7_e146508 * locals.var_ty_dn10), (assign94760_body7_e146508 * locals.var_ty_dn11), (assign94760_body7_e146508 * locals.var_ty_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
            locals.var_t1 = assign94760_body7_e146510;
            locals.var_t1_dn0 = assign94760_body7_e146510_d_n0;
            locals.var_t1_dn2 = assign94760_body7_e146510_d_n2;
            locals.var_t1_dn4 = assign94760_body7_e146510_d_n4;
            locals.var_t1_dn5 = assign94760_body7_e146510_d_n5;
            locals.var_t1_dn6 = assign94760_body7_e146510_d_n6;
            locals.var_t1_dn7 = assign94760_body7_e146510_d_n7;
            locals.var_t1_dn8 = assign94760_body7_e146510_d_n8;
            locals.var_t1_dn9 = assign94760_body7_e146510_d_n9;
            locals.var_t1_dn10 = assign94760_body7_e146510_d_n10;
            locals.var_t1_dn11 = assign94760_body7_e146510_d_n11;
            locals.var_t1_dn14 = assign94760_body7_e146510_d_n14;
            locals.var_t1_rv = 0.0;
            let (assign94760_body8_e146524, assign94760_body8_e146524_d_n0, assign94760_body8_e146524_d_n2, assign94760_body8_e146524_d_n4, assign94760_body8_e146524_d_n5, assign94760_body8_e146524_d_n6, assign94760_body8_e146524_d_n7, assign94760_body8_e146524_d_n8, assign94760_body8_e146524_d_n9, assign94760_body8_e146524_d_n10, assign94760_body8_e146524_d_n11, assign94760_body8_e146524_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2200 != 0.0)) && (locals.var_guard2205 != 0.0)) {
        let assign94760_body8_e146519: f64 = (-locals.var_c_sb);
        let assign94760_body8_e146521: f64 = (assign94760_body8_e146519 * locals.var_dphi_sb);
        let assign94760_body8_e146522: f64 = (assign94760_body8_e146521).exp();
        (assign94760_body8_e146522, (assign94760_body8_e146522 * (((-locals.var_c_sb_dn0) * locals.var_dphi_sb) + (assign94760_body8_e146519 * locals.var_dphi_sb_dn0))), (assign94760_body8_e146522 * (((-locals.var_c_sb_dn2) * locals.var_dphi_sb) + (assign94760_body8_e146519 * locals.var_dphi_sb_dn2))), (assign94760_body8_e146522 * (((-locals.var_c_sb_dn4) * locals.var_dphi_sb) + (assign94760_body8_e146519 * locals.var_dphi_sb_dn4))), (assign94760_body8_e146522 * (((-locals.var_c_sb_dn5) * locals.var_dphi_sb) + (assign94760_body8_e146519 * locals.var_dphi_sb_dn5))), (assign94760_body8_e146522 * (((-locals.var_c_sb_dn6) * locals.var_dphi_sb) + (assign94760_body8_e146519 * locals.var_dphi_sb_dn6))), (assign94760_body8_e146522 * (((-locals.var_c_sb_dn7) * locals.var_dphi_sb) + (assign94760_body8_e146519 * locals.var_dphi_sb_dn7))), (assign94760_body8_e146522 * (((-locals.var_c_sb_dn8) * locals.var_dphi_sb) + (assign94760_body8_e146519 * locals.var_dphi_sb_dn8))), (assign94760_body8_e146522 * (((-locals.var_c_sb_dn9) * locals.var_dphi_sb) + (assign94760_body8_e146519 * locals.var_dphi_sb_dn9))), (assign94760_body8_e146522 * (((-locals.var_c_sb_dn10) * locals.var_dphi_sb) + (assign94760_body8_e146519 * locals.var_dphi_sb_dn10))), (assign94760_body8_e146522 * (((-locals.var_c_sb_dn11) * locals.var_dphi_sb) + (assign94760_body8_e146519 * locals.var_dphi_sb_dn11))), (assign94760_body8_e146522 * (((-locals.var_c_sb_dn14) * locals.var_dphi_sb) + (assign94760_body8_e146519 * locals.var_dphi_sb_dn14))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
            locals.var_t0 = assign94760_body8_e146524;
            locals.var_t0_dn0 = assign94760_body8_e146524_d_n0;
            locals.var_t0_dn2 = assign94760_body8_e146524_d_n2;
            locals.var_t0_dn4 = assign94760_body8_e146524_d_n4;
            locals.var_t0_dn5 = assign94760_body8_e146524_d_n5;
            locals.var_t0_dn6 = assign94760_body8_e146524_d_n6;
            locals.var_t0_dn7 = assign94760_body8_e146524_d_n7;
            locals.var_t0_dn8 = assign94760_body8_e146524_d_n8;
            locals.var_t0_dn9 = assign94760_body8_e146524_d_n9;
            locals.var_t0_dn10 = assign94760_body8_e146524_d_n10;
            locals.var_t0_dn11 = assign94760_body8_e146524_d_n11;
            locals.var_t0_dn14 = assign94760_body8_e146524_d_n14;
            locals.var_t0_rv = 0.0;
            let (assign94760_body9_e146536, assign94760_body9_e146536_d_n0, assign94760_body9_e146536_d_n2, assign94760_body9_e146536_d_n4, assign94760_body9_e146536_d_n5, assign94760_body9_e146536_d_n6, assign94760_body9_e146536_d_n7, assign94760_body9_e146536_d_n8, assign94760_body9_e146536_d_n9, assign94760_body9_e146536_d_n10, assign94760_body9_e146536_d_n11, assign94760_body9_e146536_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2200 != 0.0)) && (locals.var_guard2205 != 0.0)) {
        let assign94760_body9_e146534: f64 = (locals.var_t1 - locals.var_t0);
        (assign94760_body9_e146534, (locals.var_t1_dn0 - locals.var_t0_dn0), (locals.var_t1_dn2 - locals.var_t0_dn2), (locals.var_t1_dn4 - locals.var_t0_dn4), (locals.var_t1_dn5 - locals.var_t0_dn5), (locals.var_t1_dn6 - locals.var_t0_dn6), (locals.var_t1_dn7 - locals.var_t0_dn7), (locals.var_t1_dn8 - locals.var_t0_dn8), (locals.var_t1_dn9 - locals.var_t0_dn9), (locals.var_t1_dn10 - locals.var_t0_dn10), (locals.var_t1_dn11 - locals.var_t0_dn11), (locals.var_t1_dn14 - locals.var_t0_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
            locals.var_t2 = assign94760_body9_e146536;
            locals.var_t2_dn0 = assign94760_body9_e146536_d_n0;
            locals.var_t2_dn2 = assign94760_body9_e146536_d_n2;
            locals.var_t2_dn4 = assign94760_body9_e146536_d_n4;
            locals.var_t2_dn5 = assign94760_body9_e146536_d_n5;
            locals.var_t2_dn6 = assign94760_body9_e146536_d_n6;
            locals.var_t2_dn7 = assign94760_body9_e146536_d_n7;
            locals.var_t2_dn8 = assign94760_body9_e146536_d_n8;
            locals.var_t2_dn9 = assign94760_body9_e146536_d_n9;
            locals.var_t2_dn10 = assign94760_body9_e146536_d_n10;
            locals.var_t2_dn11 = assign94760_body9_e146536_d_n11;
            locals.var_t2_dn14 = assign94760_body9_e146536_d_n14;
            locals.var_t2_rv = 0.0;
            let (assign94760_body10_e146551, assign94760_body10_e146551_d_n0, assign94760_body10_e146551_d_n2, assign94760_body10_e146551_d_n4, assign94760_body10_e146551_d_n5, assign94760_body10_e146551_d_n6, assign94760_body10_e146551_d_n7, assign94760_body10_e146551_d_n8, assign94760_body10_e146551_d_n9, assign94760_body10_e146551_d_n10, assign94760_body10_e146551_d_n11, assign94760_body10_e146551_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2200 != 0.0)) && (locals.var_guard2205 != 0.0)) {
        let assign94760_body10_e146546: f64 = (1.0 + locals.var_t2);
        let assign94760_body10_e146547: f64 = (assign94760_body10_e146546).ln();
        let assign94760_body10_e146549: f64 = (assign94760_body10_e146547 / locals.var_c_sb);
        (assign94760_body10_e146549, ((((locals.var_t2_dn0 / assign94760_body10_e146546) * locals.var_c_sb) - (assign94760_body10_e146547 * locals.var_c_sb_dn0)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn2 / assign94760_body10_e146546) * locals.var_c_sb) - (assign94760_body10_e146547 * locals.var_c_sb_dn2)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn4 / assign94760_body10_e146546) * locals.var_c_sb) - (assign94760_body10_e146547 * locals.var_c_sb_dn4)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn5 / assign94760_body10_e146546) * locals.var_c_sb) - (assign94760_body10_e146547 * locals.var_c_sb_dn5)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn6 / assign94760_body10_e146546) * locals.var_c_sb) - (assign94760_body10_e146547 * locals.var_c_sb_dn6)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn7 / assign94760_body10_e146546) * locals.var_c_sb) - (assign94760_body10_e146547 * locals.var_c_sb_dn7)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn8 / assign94760_body10_e146546) * locals.var_c_sb) - (assign94760_body10_e146547 * locals.var_c_sb_dn8)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn9 / assign94760_body10_e146546) * locals.var_c_sb) - (assign94760_body10_e146547 * locals.var_c_sb_dn9)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn10 / assign94760_body10_e146546) * locals.var_c_sb) - (assign94760_body10_e146547 * locals.var_c_sb_dn10)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn11 / assign94760_body10_e146546) * locals.var_c_sb) - (assign94760_body10_e146547 * locals.var_c_sb_dn11)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn14 / assign94760_body10_e146546) * locals.var_c_sb) - (assign94760_body10_e146547 * locals.var_c_sb_dn14)) / (locals.var_c_sb * locals.var_c_sb)),)
    } else {
        (locals.var_phi_b, locals.var_phi_b_dn0, locals.var_phi_b_dn2, locals.var_phi_b_dn4, locals.var_phi_b_dn5, locals.var_phi_b_dn6, locals.var_phi_b_dn7, locals.var_phi_b_dn8, locals.var_phi_b_dn9, locals.var_phi_b_dn10, locals.var_phi_b_dn11, locals.var_phi_b_dn14,)
    }
};
            locals.var_phi_b = assign94760_body10_e146551;
            locals.var_phi_b_dn0 = assign94760_body10_e146551_d_n0;
            locals.var_phi_b_dn2 = assign94760_body10_e146551_d_n2;
            locals.var_phi_b_dn4 = assign94760_body10_e146551_d_n4;
            locals.var_phi_b_dn5 = assign94760_body10_e146551_d_n5;
            locals.var_phi_b_dn6 = assign94760_body10_e146551_d_n6;
            locals.var_phi_b_dn7 = assign94760_body10_e146551_d_n7;
            locals.var_phi_b_dn8 = assign94760_body10_e146551_d_n8;
            locals.var_phi_b_dn9 = assign94760_body10_e146551_d_n9;
            locals.var_phi_b_dn10 = assign94760_body10_e146551_d_n10;
            locals.var_phi_b_dn11 = assign94760_body10_e146551_d_n11;
            locals.var_phi_b_dn14 = assign94760_body10_e146551_d_n14;
            locals.var_phi_b_rv = 0.0;
            let (assign94760_body11_e146565, assign94760_body11_e146565_d_n0, assign94760_body11_e146565_d_n2, assign94760_body11_e146565_d_n4, assign94760_body11_e146565_d_n5, assign94760_body11_e146565_d_n6, assign94760_body11_e146565_d_n7, assign94760_body11_e146565_d_n8, assign94760_body11_e146565_d_n9, assign94760_body11_e146565_d_n10, assign94760_body11_e146565_d_n11, assign94760_body11_e146565_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2200 != 0.0)) && (locals.var_guard2205 != 0.0)) {
        let assign94760_body11_e146562: f64 = (1.0 + locals.var_t2);
        let assign94760_body11_e146563: f64 = (locals.var_t1 / assign94760_body11_e146562);
        (assign94760_body11_e146563, (((locals.var_t1_dn0 * assign94760_body11_e146562) - (locals.var_t1 * locals.var_t2_dn0)) / (assign94760_body11_e146562 * assign94760_body11_e146562)), (((locals.var_t1_dn2 * assign94760_body11_e146562) - (locals.var_t1 * locals.var_t2_dn2)) / (assign94760_body11_e146562 * assign94760_body11_e146562)), (((locals.var_t1_dn4 * assign94760_body11_e146562) - (locals.var_t1 * locals.var_t2_dn4)) / (assign94760_body11_e146562 * assign94760_body11_e146562)), (((locals.var_t1_dn5 * assign94760_body11_e146562) - (locals.var_t1 * locals.var_t2_dn5)) / (assign94760_body11_e146562 * assign94760_body11_e146562)), (((locals.var_t1_dn6 * assign94760_body11_e146562) - (locals.var_t1 * locals.var_t2_dn6)) / (assign94760_body11_e146562 * assign94760_body11_e146562)), (((locals.var_t1_dn7 * assign94760_body11_e146562) - (locals.var_t1 * locals.var_t2_dn7)) / (assign94760_body11_e146562 * assign94760_body11_e146562)), (((locals.var_t1_dn8 * assign94760_body11_e146562) - (locals.var_t1 * locals.var_t2_dn8)) / (assign94760_body11_e146562 * assign94760_body11_e146562)), (((locals.var_t1_dn9 * assign94760_body11_e146562) - (locals.var_t1 * locals.var_t2_dn9)) / (assign94760_body11_e146562 * assign94760_body11_e146562)), (((locals.var_t1_dn10 * assign94760_body11_e146562) - (locals.var_t1 * locals.var_t2_dn10)) / (assign94760_body11_e146562 * assign94760_body11_e146562)), (((locals.var_t1_dn11 * assign94760_body11_e146562) - (locals.var_t1 * locals.var_t2_dn11)) / (assign94760_body11_e146562 * assign94760_body11_e146562)), (((locals.var_t1_dn14 * assign94760_body11_e146562) - (locals.var_t1 * locals.var_t2_dn14)) / (assign94760_body11_e146562 * assign94760_body11_e146562)),)
    } else {
        (locals.var_phi_b_dpss, locals.var_phi_b_dpss_dn0, locals.var_phi_b_dpss_dn2, locals.var_phi_b_dpss_dn4, locals.var_phi_b_dpss_dn5, locals.var_phi_b_dpss_dn6, locals.var_phi_b_dpss_dn7, locals.var_phi_b_dpss_dn8, locals.var_phi_b_dpss_dn9, locals.var_phi_b_dpss_dn10, locals.var_phi_b_dpss_dn11, locals.var_phi_b_dpss_dn14,)
    }
};
            locals.var_phi_b_dpss = assign94760_body11_e146565;
            locals.var_phi_b_dpss_dn0 = assign94760_body11_e146565_d_n0;
            locals.var_phi_b_dpss_dn2 = assign94760_body11_e146565_d_n2;
            locals.var_phi_b_dpss_dn4 = assign94760_body11_e146565_d_n4;
            locals.var_phi_b_dpss_dn5 = assign94760_body11_e146565_d_n5;
            locals.var_phi_b_dpss_dn6 = assign94760_body11_e146565_d_n6;
            locals.var_phi_b_dpss_dn7 = assign94760_body11_e146565_d_n7;
            locals.var_phi_b_dpss_dn8 = assign94760_body11_e146565_d_n8;
            locals.var_phi_b_dpss_dn9 = assign94760_body11_e146565_d_n9;
            locals.var_phi_b_dpss_dn10 = assign94760_body11_e146565_d_n10;
            locals.var_phi_b_dpss_dn11 = assign94760_body11_e146565_d_n11;
            locals.var_phi_b_dpss_dn14 = assign94760_body11_e146565_d_n14;
            locals.var_phi_b_dpss_rv = 0.0;
            let (assign94760_body12_e146578, assign94760_body12_e146578_d_n0, assign94760_body12_e146578_d_n2, assign94760_body12_e146578_d_n4, assign94760_body12_e146578_d_n5, assign94760_body12_e146578_d_n6, assign94760_body12_e146578_d_n7, assign94760_body12_e146578_d_n8, assign94760_body12_e146578_d_n9, assign94760_body12_e146578_d_n10, assign94760_body12_e146578_d_n11, assign94760_body12_e146578_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2200 != 0.0)) && (locals.var_guard2205 == 0.0)) {
        let assign94760_body12_e146576: f64 = (locals.var_ps0ld_vxb - locals.var_dphi_sb);
        (assign94760_body12_e146576, (locals.var_ps0ld_vxb_dn0 - locals.var_dphi_sb_dn0), (locals.var_ps0ld_vxb_dn2 - locals.var_dphi_sb_dn2), (locals.var_ps0ld_vxb_dn4 - locals.var_dphi_sb_dn4), (locals.var_ps0ld_vxb_dn5 - locals.var_dphi_sb_dn5), (locals.var_ps0ld_vxb_dn6 - locals.var_dphi_sb_dn6), (locals.var_ps0ld_vxb_dn7 - locals.var_dphi_sb_dn7), (locals.var_ps0ld_vxb_dn8 - locals.var_dphi_sb_dn8), (locals.var_ps0ld_vxb_dn9 - locals.var_dphi_sb_dn9), (locals.var_ps0ld_vxb_dn10 - locals.var_dphi_sb_dn10), (locals.var_ps0ld_vxb_dn11 - locals.var_dphi_sb_dn11), (locals.var_ps0ld_vxb_dn14 - locals.var_dphi_sb_dn14),)
    } else {
        (locals.var_phi_b, locals.var_phi_b_dn0, locals.var_phi_b_dn2, locals.var_phi_b_dn4, locals.var_phi_b_dn5, locals.var_phi_b_dn6, locals.var_phi_b_dn7, locals.var_phi_b_dn8, locals.var_phi_b_dn9, locals.var_phi_b_dn10, locals.var_phi_b_dn11, locals.var_phi_b_dn14,)
    }
};
            locals.var_phi_b = assign94760_body12_e146578;
            locals.var_phi_b_dn0 = assign94760_body12_e146578_d_n0;
            locals.var_phi_b_dn2 = assign94760_body12_e146578_d_n2;
            locals.var_phi_b_dn4 = assign94760_body12_e146578_d_n4;
            locals.var_phi_b_dn5 = assign94760_body12_e146578_d_n5;
            locals.var_phi_b_dn6 = assign94760_body12_e146578_d_n6;
            locals.var_phi_b_dn7 = assign94760_body12_e146578_d_n7;
            locals.var_phi_b_dn8 = assign94760_body12_e146578_d_n8;
            locals.var_phi_b_dn9 = assign94760_body12_e146578_d_n9;
            locals.var_phi_b_dn10 = assign94760_body12_e146578_d_n10;
            locals.var_phi_b_dn11 = assign94760_body12_e146578_d_n11;
            locals.var_phi_b_dn14 = assign94760_body12_e146578_d_n14;
            locals.var_phi_b_rv = 0.0;
            let (assign94760_body13_e146589, assign94760_body13_e146589_d_n0, assign94760_body13_e146589_d_n2, assign94760_body13_e146589_d_n4, assign94760_body13_e146589_d_n5, assign94760_body13_e146589_d_n6, assign94760_body13_e146589_d_n7, assign94760_body13_e146589_d_n8, assign94760_body13_e146589_d_n9, assign94760_body13_e146589_d_n10, assign94760_body13_e146589_d_n11, assign94760_body13_e146589_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2200 != 0.0)) && (locals.var_guard2205 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phi_b_dpss, locals.var_phi_b_dpss_dn0, locals.var_phi_b_dpss_dn2, locals.var_phi_b_dpss_dn4, locals.var_phi_b_dpss_dn5, locals.var_phi_b_dpss_dn6, locals.var_phi_b_dpss_dn7, locals.var_phi_b_dpss_dn8, locals.var_phi_b_dpss_dn9, locals.var_phi_b_dpss_dn10, locals.var_phi_b_dpss_dn11, locals.var_phi_b_dpss_dn14,)
    }
};
            locals.var_phi_b_dpss = assign94760_body13_e146589;
            locals.var_phi_b_dpss_dn0 = assign94760_body13_e146589_d_n0;
            locals.var_phi_b_dpss_dn2 = assign94760_body13_e146589_d_n2;
            locals.var_phi_b_dpss_dn4 = assign94760_body13_e146589_d_n4;
            locals.var_phi_b_dpss_dn5 = assign94760_body13_e146589_d_n5;
            locals.var_phi_b_dpss_dn6 = assign94760_body13_e146589_d_n6;
            locals.var_phi_b_dpss_dn7 = assign94760_body13_e146589_d_n7;
            locals.var_phi_b_dpss_dn8 = assign94760_body13_e146589_d_n8;
            locals.var_phi_b_dpss_dn9 = assign94760_body13_e146589_d_n9;
            locals.var_phi_b_dpss_dn10 = assign94760_body13_e146589_d_n10;
            locals.var_phi_b_dpss_dn11 = assign94760_body13_e146589_d_n11;
            locals.var_phi_b_dpss_dn14 = assign94760_body13_e146589_d_n14;
            locals.var_phi_b_dpss_rv = 0.0;
            let (assign94760_body14_e146599, assign94760_body14_e146599_d_n0, assign94760_body14_e146599_d_n2, assign94760_body14_e146599_d_n4, assign94760_body14_e146599_d_n5, assign94760_body14_e146599_d_n6, assign94760_body14_e146599_d_n7, assign94760_body14_e146599_d_n8, assign94760_body14_e146599_d_n9, assign94760_body14_e146599_d_n10, assign94760_body14_e146599_d_n11, assign94760_body14_e146599_d_n14,) = {
    if (((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2200 != 0.0)) {
        let assign94760_body14_e146597: f64 = (locals.var_beta * locals.var_phi_b);
        (assign94760_body14_e146597, ((locals.var_beta_dn0 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn0)), ((locals.var_beta_dn2 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn2)), ((locals.var_beta_dn4 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn4)), ((locals.var_beta_dn5 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn5)), ((locals.var_beta_dn6 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn6)), ((locals.var_beta_dn7 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn7)), ((locals.var_beta_dn8 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn8)), ((locals.var_beta_dn9 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn9)), ((locals.var_beta_dn10 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn10)), ((locals.var_beta_dn11 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn11)), ((locals.var_beta_dn14 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn14)),)
    } else {
        (locals.var_chib, locals.var_chib_dn0, locals.var_chib_dn2, locals.var_chib_dn4, locals.var_chib_dn5, locals.var_chib_dn6, locals.var_chib_dn7, locals.var_chib_dn8, locals.var_chib_dn9, locals.var_chib_dn10, locals.var_chib_dn11, locals.var_chib_dn14,)
    }
};
            locals.var_chib = assign94760_body14_e146599;
            locals.var_chib_dn0 = assign94760_body14_e146599_d_n0;
            locals.var_chib_dn2 = assign94760_body14_e146599_d_n2;
            locals.var_chib_dn4 = assign94760_body14_e146599_d_n4;
            locals.var_chib_dn5 = assign94760_body14_e146599_d_n5;
            locals.var_chib_dn6 = assign94760_body14_e146599_d_n6;
            locals.var_chib_dn7 = assign94760_body14_e146599_d_n7;
            locals.var_chib_dn8 = assign94760_body14_e146599_d_n8;
            locals.var_chib_dn9 = assign94760_body14_e146599_d_n9;
            locals.var_chib_dn10 = assign94760_body14_e146599_d_n10;
            locals.var_chib_dn11 = assign94760_body14_e146599_d_n11;
            locals.var_chib_dn14 = assign94760_body14_e146599_d_n14;
            locals.var_chib_rv = 0.0;
            let assign94760_body15_e146601: f64 = (locals.var_chi).abs();
            let assign94760_body15_e146603: f64 = if assign94760_body15_e146601 < 1e-6 { 1.0 } else { 0.0 };
            locals.var_guard2206 = assign94760_body15_e146603;
            locals.var_guard2206_rv = 0.0;
            let (assign94760_body17_e146653, assign94760_body17_e146653_d_n0, assign94760_body17_e146653_d_n2, assign94760_body17_e146653_d_n4, assign94760_body17_e146653_d_n5, assign94760_body17_e146653_d_n6, assign94760_body17_e146653_d_n7, assign94760_body17_e146653_d_n8, assign94760_body17_e146653_d_n9, assign94760_body17_e146653_d_n10, assign94760_body17_e146653_d_n11, assign94760_body17_e146653_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2200 != 0.0)) && (locals.var_guard2206 != 0.0)) {
        let assign94760_body17_e146631: f64 = (locals.var_chi * locals.var_chi);
        let assign94760_body17_e146633: f64 = (assign94760_body17_e146631 / 2.0);
        let assign94760_body17_e146637: f64 = (locals.var_chi / 3.0);
        let assign94760_body17_e146641: f64 = (locals.var_chi / 4.0);
        let assign94760_body17_e146645: f64 = (locals.var_chi / 5.0);
        let assign94760_body17_e146646: f64 = (1.0 - assign94760_body17_e146645);
        let assign94760_body17_e146647: f64 = (assign94760_body17_e146641 * assign94760_body17_e146646);
        let assign94760_body17_e146648: f64 = (1.0 - assign94760_body17_e146647);
        let assign94760_body17_e146649: f64 = (assign94760_body17_e146637 * assign94760_body17_e146648);
        let assign94760_body17_e146650: f64 = (1.0 - assign94760_body17_e146649);
        let assign94760_body17_e146651: f64 = (assign94760_body17_e146633 * assign94760_body17_e146650);
        (assign94760_body17_e146651, (((((locals.var_chi_dn0 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn0)) / 2.0) * assign94760_body17_e146650) + (assign94760_body17_e146633 * (-(((locals.var_chi_dn0 / 3.0) * assign94760_body17_e146648) + (assign94760_body17_e146637 * (-(((locals.var_chi_dn0 / 4.0) * assign94760_body17_e146646) + (assign94760_body17_e146641 * (-(locals.var_chi_dn0 / 5.0)))))))))), (((((locals.var_chi_dn2 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn2)) / 2.0) * assign94760_body17_e146650) + (assign94760_body17_e146633 * (-(((locals.var_chi_dn2 / 3.0) * assign94760_body17_e146648) + (assign94760_body17_e146637 * (-(((locals.var_chi_dn2 / 4.0) * assign94760_body17_e146646) + (assign94760_body17_e146641 * (-(locals.var_chi_dn2 / 5.0)))))))))), (((((locals.var_chi_dn4 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn4)) / 2.0) * assign94760_body17_e146650) + (assign94760_body17_e146633 * (-(((locals.var_chi_dn4 / 3.0) * assign94760_body17_e146648) + (assign94760_body17_e146637 * (-(((locals.var_chi_dn4 / 4.0) * assign94760_body17_e146646) + (assign94760_body17_e146641 * (-(locals.var_chi_dn4 / 5.0)))))))))), (((((locals.var_chi_dn5 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn5)) / 2.0) * assign94760_body17_e146650) + (assign94760_body17_e146633 * (-(((locals.var_chi_dn5 / 3.0) * assign94760_body17_e146648) + (assign94760_body17_e146637 * (-(((locals.var_chi_dn5 / 4.0) * assign94760_body17_e146646) + (assign94760_body17_e146641 * (-(locals.var_chi_dn5 / 5.0)))))))))), (((((locals.var_chi_dn6 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn6)) / 2.0) * assign94760_body17_e146650) + (assign94760_body17_e146633 * (-(((locals.var_chi_dn6 / 3.0) * assign94760_body17_e146648) + (assign94760_body17_e146637 * (-(((locals.var_chi_dn6 / 4.0) * assign94760_body17_e146646) + (assign94760_body17_e146641 * (-(locals.var_chi_dn6 / 5.0)))))))))), (((((locals.var_chi_dn7 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn7)) / 2.0) * assign94760_body17_e146650) + (assign94760_body17_e146633 * (-(((locals.var_chi_dn7 / 3.0) * assign94760_body17_e146648) + (assign94760_body17_e146637 * (-(((locals.var_chi_dn7 / 4.0) * assign94760_body17_e146646) + (assign94760_body17_e146641 * (-(locals.var_chi_dn7 / 5.0)))))))))), (((((locals.var_chi_dn8 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn8)) / 2.0) * assign94760_body17_e146650) + (assign94760_body17_e146633 * (-(((locals.var_chi_dn8 / 3.0) * assign94760_body17_e146648) + (assign94760_body17_e146637 * (-(((locals.var_chi_dn8 / 4.0) * assign94760_body17_e146646) + (assign94760_body17_e146641 * (-(locals.var_chi_dn8 / 5.0)))))))))), (((((locals.var_chi_dn9 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn9)) / 2.0) * assign94760_body17_e146650) + (assign94760_body17_e146633 * (-(((locals.var_chi_dn9 / 3.0) * assign94760_body17_e146648) + (assign94760_body17_e146637 * (-(((locals.var_chi_dn9 / 4.0) * assign94760_body17_e146646) + (assign94760_body17_e146641 * (-(locals.var_chi_dn9 / 5.0)))))))))), (((((locals.var_chi_dn10 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn10)) / 2.0) * assign94760_body17_e146650) + (assign94760_body17_e146633 * (-(((locals.var_chi_dn10 / 3.0) * assign94760_body17_e146648) + (assign94760_body17_e146637 * (-(((locals.var_chi_dn10 / 4.0) * assign94760_body17_e146646) + (assign94760_body17_e146641 * (-(locals.var_chi_dn10 / 5.0)))))))))), (((((locals.var_chi_dn11 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn11)) / 2.0) * assign94760_body17_e146650) + (assign94760_body17_e146633 * (-(((locals.var_chi_dn11 / 3.0) * assign94760_body17_e146648) + (assign94760_body17_e146637 * (-(((locals.var_chi_dn11 / 4.0) * assign94760_body17_e146646) + (assign94760_body17_e146641 * (-(locals.var_chi_dn11 / 5.0)))))))))), (((((locals.var_chi_dn14 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn14)) / 2.0) * assign94760_body17_e146650) + (assign94760_body17_e146633 * (-(((locals.var_chi_dn14 / 3.0) * assign94760_body17_e146648) + (assign94760_body17_e146637 * (-(((locals.var_chi_dn14 / 4.0) * assign94760_body17_e146646) + (assign94760_body17_e146641 * (-(locals.var_chi_dn14 / 5.0)))))))))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
            locals.var_t0 = assign94760_body17_e146653;
            locals.var_t0_dn0 = assign94760_body17_e146653_d_n0;
            locals.var_t0_dn2 = assign94760_body17_e146653_d_n2;
            locals.var_t0_dn4 = assign94760_body17_e146653_d_n4;
            locals.var_t0_dn5 = assign94760_body17_e146653_d_n5;
            locals.var_t0_dn6 = assign94760_body17_e146653_d_n6;
            locals.var_t0_dn7 = assign94760_body17_e146653_d_n7;
            locals.var_t0_dn8 = assign94760_body17_e146653_d_n8;
            locals.var_t0_dn9 = assign94760_body17_e146653_d_n9;
            locals.var_t0_dn10 = assign94760_body17_e146653_d_n10;
            locals.var_t0_dn11 = assign94760_body17_e146653_d_n11;
            locals.var_t0_dn14 = assign94760_body17_e146653_d_n14;
            locals.var_t0_rv = 0.0;
            let (assign94760_body18_e146681, assign94760_body18_e146681_d_n0, assign94760_body18_e146681_d_n2, assign94760_body18_e146681_d_n4, assign94760_body18_e146681_d_n5, assign94760_body18_e146681_d_n6, assign94760_body18_e146681_d_n7, assign94760_body18_e146681_d_n8, assign94760_body18_e146681_d_n9, assign94760_body18_e146681_d_n10, assign94760_body18_e146681_d_n11, assign94760_body18_e146681_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2200 != 0.0)) && (locals.var_guard2206 != 0.0)) {
        let assign94760_body18_e146665: f64 = (locals.var_chi / 2.0);
        let assign94760_body18_e146669: f64 = (locals.var_chi / 3.0);
        let assign94760_body18_e146673: f64 = (locals.var_chi / 4.0);
        let assign94760_body18_e146674: f64 = (1.0 - assign94760_body18_e146673);
        let assign94760_body18_e146675: f64 = (assign94760_body18_e146669 * assign94760_body18_e146674);
        let assign94760_body18_e146676: f64 = (1.0 - assign94760_body18_e146675);
        let assign94760_body18_e146677: f64 = (assign94760_body18_e146665 * assign94760_body18_e146676);
        let assign94760_body18_e146678: f64 = (1.0 - assign94760_body18_e146677);
        let assign94760_body18_e146679: f64 = (locals.var_chi * assign94760_body18_e146678);
        (assign94760_body18_e146679, ((locals.var_chi_dn0 * assign94760_body18_e146678) + (locals.var_chi * (-(((locals.var_chi_dn0 / 2.0) * assign94760_body18_e146676) + (assign94760_body18_e146665 * (-(((locals.var_chi_dn0 / 3.0) * assign94760_body18_e146674) + (assign94760_body18_e146669 * (-(locals.var_chi_dn0 / 4.0)))))))))), ((locals.var_chi_dn2 * assign94760_body18_e146678) + (locals.var_chi * (-(((locals.var_chi_dn2 / 2.0) * assign94760_body18_e146676) + (assign94760_body18_e146665 * (-(((locals.var_chi_dn2 / 3.0) * assign94760_body18_e146674) + (assign94760_body18_e146669 * (-(locals.var_chi_dn2 / 4.0)))))))))), ((locals.var_chi_dn4 * assign94760_body18_e146678) + (locals.var_chi * (-(((locals.var_chi_dn4 / 2.0) * assign94760_body18_e146676) + (assign94760_body18_e146665 * (-(((locals.var_chi_dn4 / 3.0) * assign94760_body18_e146674) + (assign94760_body18_e146669 * (-(locals.var_chi_dn4 / 4.0)))))))))), ((locals.var_chi_dn5 * assign94760_body18_e146678) + (locals.var_chi * (-(((locals.var_chi_dn5 / 2.0) * assign94760_body18_e146676) + (assign94760_body18_e146665 * (-(((locals.var_chi_dn5 / 3.0) * assign94760_body18_e146674) + (assign94760_body18_e146669 * (-(locals.var_chi_dn5 / 4.0)))))))))), ((locals.var_chi_dn6 * assign94760_body18_e146678) + (locals.var_chi * (-(((locals.var_chi_dn6 / 2.0) * assign94760_body18_e146676) + (assign94760_body18_e146665 * (-(((locals.var_chi_dn6 / 3.0) * assign94760_body18_e146674) + (assign94760_body18_e146669 * (-(locals.var_chi_dn6 / 4.0)))))))))), ((locals.var_chi_dn7 * assign94760_body18_e146678) + (locals.var_chi * (-(((locals.var_chi_dn7 / 2.0) * assign94760_body18_e146676) + (assign94760_body18_e146665 * (-(((locals.var_chi_dn7 / 3.0) * assign94760_body18_e146674) + (assign94760_body18_e146669 * (-(locals.var_chi_dn7 / 4.0)))))))))), ((locals.var_chi_dn8 * assign94760_body18_e146678) + (locals.var_chi * (-(((locals.var_chi_dn8 / 2.0) * assign94760_body18_e146676) + (assign94760_body18_e146665 * (-(((locals.var_chi_dn8 / 3.0) * assign94760_body18_e146674) + (assign94760_body18_e146669 * (-(locals.var_chi_dn8 / 4.0)))))))))), ((locals.var_chi_dn9 * assign94760_body18_e146678) + (locals.var_chi * (-(((locals.var_chi_dn9 / 2.0) * assign94760_body18_e146676) + (assign94760_body18_e146665 * (-(((locals.var_chi_dn9 / 3.0) * assign94760_body18_e146674) + (assign94760_body18_e146669 * (-(locals.var_chi_dn9 / 4.0)))))))))), ((locals.var_chi_dn10 * assign94760_body18_e146678) + (locals.var_chi * (-(((locals.var_chi_dn10 / 2.0) * assign94760_body18_e146676) + (assign94760_body18_e146665 * (-(((locals.var_chi_dn10 / 3.0) * assign94760_body18_e146674) + (assign94760_body18_e146669 * (-(locals.var_chi_dn10 / 4.0)))))))))), ((locals.var_chi_dn11 * assign94760_body18_e146678) + (locals.var_chi * (-(((locals.var_chi_dn11 / 2.0) * assign94760_body18_e146676) + (assign94760_body18_e146665 * (-(((locals.var_chi_dn11 / 3.0) * assign94760_body18_e146674) + (assign94760_body18_e146669 * (-(locals.var_chi_dn11 / 4.0)))))))))), ((locals.var_chi_dn14 * assign94760_body18_e146678) + (locals.var_chi * (-(((locals.var_chi_dn14 / 2.0) * assign94760_body18_e146676) + (assign94760_body18_e146665 * (-(((locals.var_chi_dn14 / 3.0) * assign94760_body18_e146674) + (assign94760_body18_e146669 * (-(locals.var_chi_dn14 / 4.0)))))))))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
            locals.var_t1 = assign94760_body18_e146681;
            locals.var_t1_dn0 = assign94760_body18_e146681_d_n0;
            locals.var_t1_dn2 = assign94760_body18_e146681_d_n2;
            locals.var_t1_dn4 = assign94760_body18_e146681_d_n4;
            locals.var_t1_dn5 = assign94760_body18_e146681_d_n5;
            locals.var_t1_dn6 = assign94760_body18_e146681_d_n6;
            locals.var_t1_dn7 = assign94760_body18_e146681_d_n7;
            locals.var_t1_dn8 = assign94760_body18_e146681_d_n8;
            locals.var_t1_dn9 = assign94760_body18_e146681_d_n9;
            locals.var_t1_dn10 = assign94760_body18_e146681_d_n10;
            locals.var_t1_dn11 = assign94760_body18_e146681_d_n11;
            locals.var_t1_dn14 = assign94760_body18_e146681_d_n14;
            locals.var_t1_rv = 0.0;
            let (assign94760_body19_e146713, assign94760_body19_e146713_d_n0, assign94760_body19_e146713_d_n2, assign94760_body19_e146713_d_n4, assign94760_body19_e146713_d_n5, assign94760_body19_e146713_d_n6, assign94760_body19_e146713_d_n7, assign94760_body19_e146713_d_n8, assign94760_body19_e146713_d_n9, assign94760_body19_e146713_d_n10, assign94760_body19_e146713_d_n11, assign94760_body19_e146713_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2200 != 0.0)) && (locals.var_guard2206 != 0.0)) {
        let assign94760_body19_e146691: f64 = (locals.var_chib * locals.var_chib);
        let assign94760_body19_e146693: f64 = (assign94760_body19_e146691 / 2.0);
        let assign94760_body19_e146697: f64 = (locals.var_chib / 3.0);
        let assign94760_body19_e146701: f64 = (locals.var_chib / 4.0);
        let assign94760_body19_e146705: f64 = (locals.var_chib / 5.0);
        let assign94760_body19_e146706: f64 = (1.0 - assign94760_body19_e146705);
        let assign94760_body19_e146707: f64 = (assign94760_body19_e146701 * assign94760_body19_e146706);
        let assign94760_body19_e146708: f64 = (1.0 - assign94760_body19_e146707);
        let assign94760_body19_e146709: f64 = (assign94760_body19_e146697 * assign94760_body19_e146708);
        let assign94760_body19_e146710: f64 = (1.0 - assign94760_body19_e146709);
        let assign94760_body19_e146711: f64 = (assign94760_body19_e146693 * assign94760_body19_e146710);
        (assign94760_body19_e146711, (((((locals.var_chib_dn0 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn0)) / 2.0) * assign94760_body19_e146710) + (assign94760_body19_e146693 * (-(((locals.var_chib_dn0 / 3.0) * assign94760_body19_e146708) + (assign94760_body19_e146697 * (-(((locals.var_chib_dn0 / 4.0) * assign94760_body19_e146706) + (assign94760_body19_e146701 * (-(locals.var_chib_dn0 / 5.0)))))))))), (((((locals.var_chib_dn2 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn2)) / 2.0) * assign94760_body19_e146710) + (assign94760_body19_e146693 * (-(((locals.var_chib_dn2 / 3.0) * assign94760_body19_e146708) + (assign94760_body19_e146697 * (-(((locals.var_chib_dn2 / 4.0) * assign94760_body19_e146706) + (assign94760_body19_e146701 * (-(locals.var_chib_dn2 / 5.0)))))))))), (((((locals.var_chib_dn4 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn4)) / 2.0) * assign94760_body19_e146710) + (assign94760_body19_e146693 * (-(((locals.var_chib_dn4 / 3.0) * assign94760_body19_e146708) + (assign94760_body19_e146697 * (-(((locals.var_chib_dn4 / 4.0) * assign94760_body19_e146706) + (assign94760_body19_e146701 * (-(locals.var_chib_dn4 / 5.0)))))))))), (((((locals.var_chib_dn5 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn5)) / 2.0) * assign94760_body19_e146710) + (assign94760_body19_e146693 * (-(((locals.var_chib_dn5 / 3.0) * assign94760_body19_e146708) + (assign94760_body19_e146697 * (-(((locals.var_chib_dn5 / 4.0) * assign94760_body19_e146706) + (assign94760_body19_e146701 * (-(locals.var_chib_dn5 / 5.0)))))))))), (((((locals.var_chib_dn6 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn6)) / 2.0) * assign94760_body19_e146710) + (assign94760_body19_e146693 * (-(((locals.var_chib_dn6 / 3.0) * assign94760_body19_e146708) + (assign94760_body19_e146697 * (-(((locals.var_chib_dn6 / 4.0) * assign94760_body19_e146706) + (assign94760_body19_e146701 * (-(locals.var_chib_dn6 / 5.0)))))))))), (((((locals.var_chib_dn7 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn7)) / 2.0) * assign94760_body19_e146710) + (assign94760_body19_e146693 * (-(((locals.var_chib_dn7 / 3.0) * assign94760_body19_e146708) + (assign94760_body19_e146697 * (-(((locals.var_chib_dn7 / 4.0) * assign94760_body19_e146706) + (assign94760_body19_e146701 * (-(locals.var_chib_dn7 / 5.0)))))))))), (((((locals.var_chib_dn8 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn8)) / 2.0) * assign94760_body19_e146710) + (assign94760_body19_e146693 * (-(((locals.var_chib_dn8 / 3.0) * assign94760_body19_e146708) + (assign94760_body19_e146697 * (-(((locals.var_chib_dn8 / 4.0) * assign94760_body19_e146706) + (assign94760_body19_e146701 * (-(locals.var_chib_dn8 / 5.0)))))))))), (((((locals.var_chib_dn9 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn9)) / 2.0) * assign94760_body19_e146710) + (assign94760_body19_e146693 * (-(((locals.var_chib_dn9 / 3.0) * assign94760_body19_e146708) + (assign94760_body19_e146697 * (-(((locals.var_chib_dn9 / 4.0) * assign94760_body19_e146706) + (assign94760_body19_e146701 * (-(locals.var_chib_dn9 / 5.0)))))))))), (((((locals.var_chib_dn10 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn10)) / 2.0) * assign94760_body19_e146710) + (assign94760_body19_e146693 * (-(((locals.var_chib_dn10 / 3.0) * assign94760_body19_e146708) + (assign94760_body19_e146697 * (-(((locals.var_chib_dn10 / 4.0) * assign94760_body19_e146706) + (assign94760_body19_e146701 * (-(locals.var_chib_dn10 / 5.0)))))))))), (((((locals.var_chib_dn11 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn11)) / 2.0) * assign94760_body19_e146710) + (assign94760_body19_e146693 * (-(((locals.var_chib_dn11 / 3.0) * assign94760_body19_e146708) + (assign94760_body19_e146697 * (-(((locals.var_chib_dn11 / 4.0) * assign94760_body19_e146706) + (assign94760_body19_e146701 * (-(locals.var_chib_dn11 / 5.0)))))))))), (((((locals.var_chib_dn14 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn14)) / 2.0) * assign94760_body19_e146710) + (assign94760_body19_e146693 * (-(((locals.var_chib_dn14 / 3.0) * assign94760_body19_e146708) + (assign94760_body19_e146697 * (-(((locals.var_chib_dn14 / 4.0) * assign94760_body19_e146706) + (assign94760_body19_e146701 * (-(locals.var_chib_dn14 / 5.0)))))))))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
            locals.var_t2 = assign94760_body19_e146713;
            locals.var_t2_dn0 = assign94760_body19_e146713_d_n0;
            locals.var_t2_dn2 = assign94760_body19_e146713_d_n2;
            locals.var_t2_dn4 = assign94760_body19_e146713_d_n4;
            locals.var_t2_dn5 = assign94760_body19_e146713_d_n5;
            locals.var_t2_dn6 = assign94760_body19_e146713_d_n6;
            locals.var_t2_dn7 = assign94760_body19_e146713_d_n7;
            locals.var_t2_dn8 = assign94760_body19_e146713_d_n8;
            locals.var_t2_dn9 = assign94760_body19_e146713_d_n9;
            locals.var_t2_dn10 = assign94760_body19_e146713_d_n10;
            locals.var_t2_dn11 = assign94760_body19_e146713_d_n11;
            locals.var_t2_dn14 = assign94760_body19_e146713_d_n14;
            locals.var_t2_rv = 0.0;
            let (assign94760_body20_e146741, assign94760_body20_e146741_d_n0, assign94760_body20_e146741_d_n2, assign94760_body20_e146741_d_n4, assign94760_body20_e146741_d_n5, assign94760_body20_e146741_d_n6, assign94760_body20_e146741_d_n7, assign94760_body20_e146741_d_n8, assign94760_body20_e146741_d_n9, assign94760_body20_e146741_d_n10, assign94760_body20_e146741_d_n11, assign94760_body20_e146741_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2200 != 0.0)) && (locals.var_guard2206 != 0.0)) {
        let assign94760_body20_e146725: f64 = (locals.var_chib / 2.0);
        let assign94760_body20_e146729: f64 = (locals.var_chib / 3.0);
        let assign94760_body20_e146733: f64 = (locals.var_chib / 4.0);
        let assign94760_body20_e146734: f64 = (1.0 - assign94760_body20_e146733);
        let assign94760_body20_e146735: f64 = (assign94760_body20_e146729 * assign94760_body20_e146734);
        let assign94760_body20_e146736: f64 = (1.0 - assign94760_body20_e146735);
        let assign94760_body20_e146737: f64 = (assign94760_body20_e146725 * assign94760_body20_e146736);
        let assign94760_body20_e146738: f64 = (1.0 - assign94760_body20_e146737);
        let assign94760_body20_e146739: f64 = (locals.var_chib * assign94760_body20_e146738);
        (assign94760_body20_e146739, ((locals.var_chib_dn0 * assign94760_body20_e146738) + (locals.var_chib * (-(((locals.var_chib_dn0 / 2.0) * assign94760_body20_e146736) + (assign94760_body20_e146725 * (-(((locals.var_chib_dn0 / 3.0) * assign94760_body20_e146734) + (assign94760_body20_e146729 * (-(locals.var_chib_dn0 / 4.0)))))))))), ((locals.var_chib_dn2 * assign94760_body20_e146738) + (locals.var_chib * (-(((locals.var_chib_dn2 / 2.0) * assign94760_body20_e146736) + (assign94760_body20_e146725 * (-(((locals.var_chib_dn2 / 3.0) * assign94760_body20_e146734) + (assign94760_body20_e146729 * (-(locals.var_chib_dn2 / 4.0)))))))))), ((locals.var_chib_dn4 * assign94760_body20_e146738) + (locals.var_chib * (-(((locals.var_chib_dn4 / 2.0) * assign94760_body20_e146736) + (assign94760_body20_e146725 * (-(((locals.var_chib_dn4 / 3.0) * assign94760_body20_e146734) + (assign94760_body20_e146729 * (-(locals.var_chib_dn4 / 4.0)))))))))), ((locals.var_chib_dn5 * assign94760_body20_e146738) + (locals.var_chib * (-(((locals.var_chib_dn5 / 2.0) * assign94760_body20_e146736) + (assign94760_body20_e146725 * (-(((locals.var_chib_dn5 / 3.0) * assign94760_body20_e146734) + (assign94760_body20_e146729 * (-(locals.var_chib_dn5 / 4.0)))))))))), ((locals.var_chib_dn6 * assign94760_body20_e146738) + (locals.var_chib * (-(((locals.var_chib_dn6 / 2.0) * assign94760_body20_e146736) + (assign94760_body20_e146725 * (-(((locals.var_chib_dn6 / 3.0) * assign94760_body20_e146734) + (assign94760_body20_e146729 * (-(locals.var_chib_dn6 / 4.0)))))))))), ((locals.var_chib_dn7 * assign94760_body20_e146738) + (locals.var_chib * (-(((locals.var_chib_dn7 / 2.0) * assign94760_body20_e146736) + (assign94760_body20_e146725 * (-(((locals.var_chib_dn7 / 3.0) * assign94760_body20_e146734) + (assign94760_body20_e146729 * (-(locals.var_chib_dn7 / 4.0)))))))))), ((locals.var_chib_dn8 * assign94760_body20_e146738) + (locals.var_chib * (-(((locals.var_chib_dn8 / 2.0) * assign94760_body20_e146736) + (assign94760_body20_e146725 * (-(((locals.var_chib_dn8 / 3.0) * assign94760_body20_e146734) + (assign94760_body20_e146729 * (-(locals.var_chib_dn8 / 4.0)))))))))), ((locals.var_chib_dn9 * assign94760_body20_e146738) + (locals.var_chib * (-(((locals.var_chib_dn9 / 2.0) * assign94760_body20_e146736) + (assign94760_body20_e146725 * (-(((locals.var_chib_dn9 / 3.0) * assign94760_body20_e146734) + (assign94760_body20_e146729 * (-(locals.var_chib_dn9 / 4.0)))))))))), ((locals.var_chib_dn10 * assign94760_body20_e146738) + (locals.var_chib * (-(((locals.var_chib_dn10 / 2.0) * assign94760_body20_e146736) + (assign94760_body20_e146725 * (-(((locals.var_chib_dn10 / 3.0) * assign94760_body20_e146734) + (assign94760_body20_e146729 * (-(locals.var_chib_dn10 / 4.0)))))))))), ((locals.var_chib_dn11 * assign94760_body20_e146738) + (locals.var_chib * (-(((locals.var_chib_dn11 / 2.0) * assign94760_body20_e146736) + (assign94760_body20_e146725 * (-(((locals.var_chib_dn11 / 3.0) * assign94760_body20_e146734) + (assign94760_body20_e146729 * (-(locals.var_chib_dn11 / 4.0)))))))))), ((locals.var_chib_dn14 * assign94760_body20_e146738) + (locals.var_chib * (-(((locals.var_chib_dn14 / 2.0) * assign94760_body20_e146736) + (assign94760_body20_e146725 * (-(((locals.var_chib_dn14 / 3.0) * assign94760_body20_e146734) + (assign94760_body20_e146729 * (-(locals.var_chib_dn14 / 4.0)))))))))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
            locals.var_t3 = assign94760_body20_e146741;
            locals.var_t3_dn0 = assign94760_body20_e146741_d_n0;
            locals.var_t3_dn2 = assign94760_body20_e146741_d_n2;
            locals.var_t3_dn4 = assign94760_body20_e146741_d_n4;
            locals.var_t3_dn5 = assign94760_body20_e146741_d_n5;
            locals.var_t3_dn6 = assign94760_body20_e146741_d_n6;
            locals.var_t3_dn7 = assign94760_body20_e146741_d_n7;
            locals.var_t3_dn8 = assign94760_body20_e146741_d_n8;
            locals.var_t3_dn9 = assign94760_body20_e146741_d_n9;
            locals.var_t3_dn10 = assign94760_body20_e146741_d_n10;
            locals.var_t3_dn11 = assign94760_body20_e146741_d_n11;
            locals.var_t3_dn14 = assign94760_body20_e146741_d_n14;
            locals.var_t3_rv = 0.0;
            let (assign94760_body21_e146753, assign94760_body21_e146753_d_n0, assign94760_body21_e146753_d_n2, assign94760_body21_e146753_d_n4, assign94760_body21_e146753_d_n5, assign94760_body21_e146753_d_n6, assign94760_body21_e146753_d_n7, assign94760_body21_e146753_d_n8, assign94760_body21_e146753_d_n9, assign94760_body21_e146753_d_n10, assign94760_body21_e146753_d_n11, assign94760_body21_e146753_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2200 != 0.0)) && (locals.var_guard2206 != 0.0)) {
        let assign94760_body21_e146751: f64 = (locals.var_t0 - locals.var_t2);
        (assign94760_body21_e146751, (locals.var_t0_dn0 - locals.var_t2_dn0), (locals.var_t0_dn2 - locals.var_t2_dn2), (locals.var_t0_dn4 - locals.var_t2_dn4), (locals.var_t0_dn5 - locals.var_t2_dn5), (locals.var_t0_dn6 - locals.var_t2_dn6), (locals.var_t0_dn7 - locals.var_t2_dn7), (locals.var_t0_dn8 - locals.var_t2_dn8), (locals.var_t0_dn9 - locals.var_t2_dn9), (locals.var_t0_dn10 - locals.var_t2_dn10), (locals.var_t0_dn11 - locals.var_t2_dn11), (locals.var_t0_dn14 - locals.var_t2_dn14),)
    } else {
        (locals.var_fbsq__blk2127, locals.var_fbsq__blk2127_dn0, locals.var_fbsq__blk2127_dn2, locals.var_fbsq__blk2127_dn4, locals.var_fbsq__blk2127_dn5, locals.var_fbsq__blk2127_dn6, locals.var_fbsq__blk2127_dn7, locals.var_fbsq__blk2127_dn8, locals.var_fbsq__blk2127_dn9, locals.var_fbsq__blk2127_dn10, locals.var_fbsq__blk2127_dn11, locals.var_fbsq__blk2127_dn14,)
    }
};
            locals.var_fbsq__blk2127 = assign94760_body21_e146753;
            locals.var_fbsq__blk2127_dn0 = assign94760_body21_e146753_d_n0;
            locals.var_fbsq__blk2127_dn2 = assign94760_body21_e146753_d_n2;
            locals.var_fbsq__blk2127_dn4 = assign94760_body21_e146753_d_n4;
            locals.var_fbsq__blk2127_dn5 = assign94760_body21_e146753_d_n5;
            locals.var_fbsq__blk2127_dn6 = assign94760_body21_e146753_d_n6;
            locals.var_fbsq__blk2127_dn7 = assign94760_body21_e146753_d_n7;
            locals.var_fbsq__blk2127_dn8 = assign94760_body21_e146753_d_n8;
            locals.var_fbsq__blk2127_dn9 = assign94760_body21_e146753_d_n9;
            locals.var_fbsq__blk2127_dn10 = assign94760_body21_e146753_d_n10;
            locals.var_fbsq__blk2127_dn11 = assign94760_body21_e146753_d_n11;
            locals.var_fbsq__blk2127_dn14 = assign94760_body21_e146753_d_n14;
            locals.var_fbsq__blk2127_rv = 0.0;
            let (assign94760_body22_e146769, assign94760_body22_e146769_d_n0, assign94760_body22_e146769_d_n2, assign94760_body22_e146769_d_n4, assign94760_body22_e146769_d_n5, assign94760_body22_e146769_d_n6, assign94760_body22_e146769_d_n7, assign94760_body22_e146769_d_n8, assign94760_body22_e146769_d_n9, assign94760_body22_e146769_d_n10, assign94760_body22_e146769_d_n11, assign94760_body22_e146769_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2200 != 0.0)) && (locals.var_guard2206 != 0.0)) {
        let assign94760_body22_e146765: f64 = (locals.var_phi_b_dpss * locals.var_t3);
        let assign94760_body22_e146766: f64 = (locals.var_t1 - assign94760_body22_e146765);
        let assign94760_body22_e146767: f64 = (locals.var_beta * assign94760_body22_e146766);
        (assign94760_body22_e146767, ((locals.var_beta_dn0 * assign94760_body22_e146766) + (locals.var_beta * (locals.var_t1_dn0 - ((locals.var_phi_b_dpss_dn0 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn0))))), ((locals.var_beta_dn2 * assign94760_body22_e146766) + (locals.var_beta * (locals.var_t1_dn2 - ((locals.var_phi_b_dpss_dn2 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn2))))), ((locals.var_beta_dn4 * assign94760_body22_e146766) + (locals.var_beta * (locals.var_t1_dn4 - ((locals.var_phi_b_dpss_dn4 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn4))))), ((locals.var_beta_dn5 * assign94760_body22_e146766) + (locals.var_beta * (locals.var_t1_dn5 - ((locals.var_phi_b_dpss_dn5 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn5))))), ((locals.var_beta_dn6 * assign94760_body22_e146766) + (locals.var_beta * (locals.var_t1_dn6 - ((locals.var_phi_b_dpss_dn6 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn6))))), ((locals.var_beta_dn7 * assign94760_body22_e146766) + (locals.var_beta * (locals.var_t1_dn7 - ((locals.var_phi_b_dpss_dn7 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn7))))), ((locals.var_beta_dn8 * assign94760_body22_e146766) + (locals.var_beta * (locals.var_t1_dn8 - ((locals.var_phi_b_dpss_dn8 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn8))))), ((locals.var_beta_dn9 * assign94760_body22_e146766) + (locals.var_beta * (locals.var_t1_dn9 - ((locals.var_phi_b_dpss_dn9 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn9))))), ((locals.var_beta_dn10 * assign94760_body22_e146766) + (locals.var_beta * (locals.var_t1_dn10 - ((locals.var_phi_b_dpss_dn10 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn10))))), ((locals.var_beta_dn11 * assign94760_body22_e146766) + (locals.var_beta * (locals.var_t1_dn11 - ((locals.var_phi_b_dpss_dn11 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn11))))), ((locals.var_beta_dn14 * assign94760_body22_e146766) + (locals.var_beta * (locals.var_t1_dn14 - ((locals.var_phi_b_dpss_dn14 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn14))))),)
    } else {
        (locals.var_fbsq_dpss__blk2128, locals.var_fbsq_dpss__blk2128_dn0, locals.var_fbsq_dpss__blk2128_dn2, locals.var_fbsq_dpss__blk2128_dn4, locals.var_fbsq_dpss__blk2128_dn5, locals.var_fbsq_dpss__blk2128_dn6, locals.var_fbsq_dpss__blk2128_dn7, locals.var_fbsq_dpss__blk2128_dn8, locals.var_fbsq_dpss__blk2128_dn9, locals.var_fbsq_dpss__blk2128_dn10, locals.var_fbsq_dpss__blk2128_dn11, locals.var_fbsq_dpss__blk2128_dn14,)
    }
};
            locals.var_fbsq_dpss__blk2128 = assign94760_body22_e146769;
            locals.var_fbsq_dpss__blk2128_dn0 = assign94760_body22_e146769_d_n0;
            locals.var_fbsq_dpss__blk2128_dn2 = assign94760_body22_e146769_d_n2;
            locals.var_fbsq_dpss__blk2128_dn4 = assign94760_body22_e146769_d_n4;
            locals.var_fbsq_dpss__blk2128_dn5 = assign94760_body22_e146769_d_n5;
            locals.var_fbsq_dpss__blk2128_dn6 = assign94760_body22_e146769_d_n6;
            locals.var_fbsq_dpss__blk2128_dn7 = assign94760_body22_e146769_d_n7;
            locals.var_fbsq_dpss__blk2128_dn8 = assign94760_body22_e146769_d_n8;
            locals.var_fbsq_dpss__blk2128_dn9 = assign94760_body22_e146769_d_n9;
            locals.var_fbsq_dpss__blk2128_dn10 = assign94760_body22_e146769_d_n10;
            locals.var_fbsq_dpss__blk2128_dn11 = assign94760_body22_e146769_d_n11;
            locals.var_fbsq_dpss__blk2128_dn14 = assign94760_body22_e146769_d_n14;
            locals.var_fbsq_dpss__blk2128_rv = 0.0;
            let (assign94760_body24_e146801, assign94760_body24_e146801_d_n0, assign94760_body24_e146801_d_n2, assign94760_body24_e146801_d_n4, assign94760_body24_e146801_d_n5, assign94760_body24_e146801_d_n6, assign94760_body24_e146801_d_n7, assign94760_body24_e146801_d_n8, assign94760_body24_e146801_d_n9, assign94760_body24_e146801_d_n10, assign94760_body24_e146801_d_n11, assign94760_body24_e146801_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2200 != 0.0)) && (locals.var_guard2206 == 0.0)) {
        let assign94760_body24_e146798: f64 = (-locals.var_chi);
        let assign94760_body24_e146799: f64 = (assign94760_body24_e146798).exp();
        (assign94760_body24_e146799, (assign94760_body24_e146799 * (-locals.var_chi_dn0)), (assign94760_body24_e146799 * (-locals.var_chi_dn2)), (assign94760_body24_e146799 * (-locals.var_chi_dn4)), (assign94760_body24_e146799 * (-locals.var_chi_dn5)), (assign94760_body24_e146799 * (-locals.var_chi_dn6)), (assign94760_body24_e146799 * (-locals.var_chi_dn7)), (assign94760_body24_e146799 * (-locals.var_chi_dn8)), (assign94760_body24_e146799 * (-locals.var_chi_dn9)), (assign94760_body24_e146799 * (-locals.var_chi_dn10)), (assign94760_body24_e146799 * (-locals.var_chi_dn11)), (assign94760_body24_e146799 * (-locals.var_chi_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
            locals.var_t0 = assign94760_body24_e146801;
            locals.var_t0_dn0 = assign94760_body24_e146801_d_n0;
            locals.var_t0_dn2 = assign94760_body24_e146801_d_n2;
            locals.var_t0_dn4 = assign94760_body24_e146801_d_n4;
            locals.var_t0_dn5 = assign94760_body24_e146801_d_n5;
            locals.var_t0_dn6 = assign94760_body24_e146801_d_n6;
            locals.var_t0_dn7 = assign94760_body24_e146801_d_n7;
            locals.var_t0_dn8 = assign94760_body24_e146801_d_n8;
            locals.var_t0_dn9 = assign94760_body24_e146801_d_n9;
            locals.var_t0_dn10 = assign94760_body24_e146801_d_n10;
            locals.var_t0_dn11 = assign94760_body24_e146801_d_n11;
            locals.var_t0_dn14 = assign94760_body24_e146801_d_n14;
            locals.var_t0_rv = 0.0;
            let (assign94760_body25_e146814, assign94760_body25_e146814_d_n0, assign94760_body25_e146814_d_n2, assign94760_body25_e146814_d_n4, assign94760_body25_e146814_d_n5, assign94760_body25_e146814_d_n6, assign94760_body25_e146814_d_n7, assign94760_body25_e146814_d_n8, assign94760_body25_e146814_d_n9, assign94760_body25_e146814_d_n10, assign94760_body25_e146814_d_n11, assign94760_body25_e146814_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2200 != 0.0)) && (locals.var_guard2206 == 0.0)) {
        let assign94760_body25_e146811: f64 = (-locals.var_chib);
        let assign94760_body25_e146812: f64 = (assign94760_body25_e146811).exp();
        (assign94760_body25_e146812, (assign94760_body25_e146812 * (-locals.var_chib_dn0)), (assign94760_body25_e146812 * (-locals.var_chib_dn2)), (assign94760_body25_e146812 * (-locals.var_chib_dn4)), (assign94760_body25_e146812 * (-locals.var_chib_dn5)), (assign94760_body25_e146812 * (-locals.var_chib_dn6)), (assign94760_body25_e146812 * (-locals.var_chib_dn7)), (assign94760_body25_e146812 * (-locals.var_chib_dn8)), (assign94760_body25_e146812 * (-locals.var_chib_dn9)), (assign94760_body25_e146812 * (-locals.var_chib_dn10)), (assign94760_body25_e146812 * (-locals.var_chib_dn11)), (assign94760_body25_e146812 * (-locals.var_chib_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
            locals.var_t1 = assign94760_body25_e146814;
            locals.var_t1_dn0 = assign94760_body25_e146814_d_n0;
            locals.var_t1_dn2 = assign94760_body25_e146814_d_n2;
            locals.var_t1_dn4 = assign94760_body25_e146814_d_n4;
            locals.var_t1_dn5 = assign94760_body25_e146814_d_n5;
            locals.var_t1_dn6 = assign94760_body25_e146814_d_n6;
            locals.var_t1_dn7 = assign94760_body25_e146814_d_n7;
            locals.var_t1_dn8 = assign94760_body25_e146814_d_n8;
            locals.var_t1_dn9 = assign94760_body25_e146814_d_n9;
            locals.var_t1_dn10 = assign94760_body25_e146814_d_n10;
            locals.var_t1_dn11 = assign94760_body25_e146814_d_n11;
            locals.var_t1_dn14 = assign94760_body25_e146814_d_n14;
            locals.var_t1_rv = 0.0;
            let (assign94760_body26_e146831, assign94760_body26_e146831_d_n0, assign94760_body26_e146831_d_n2, assign94760_body26_e146831_d_n4, assign94760_body26_e146831_d_n5, assign94760_body26_e146831_d_n6, assign94760_body26_e146831_d_n7, assign94760_body26_e146831_d_n8, assign94760_body26_e146831_d_n9, assign94760_body26_e146831_d_n10, assign94760_body26_e146831_d_n11, assign94760_body26_e146831_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2200 != 0.0)) && (locals.var_guard2206 == 0.0)) {
        let assign94760_body26_e146825: f64 = (locals.var_chi - locals.var_chib);
        let assign94760_body26_e146828: f64 = (locals.var_t0 - locals.var_t1);
        let assign94760_body26_e146829: f64 = (assign94760_body26_e146825 + assign94760_body26_e146828);
        (assign94760_body26_e146829, ((locals.var_chi_dn0 - locals.var_chib_dn0) + (locals.var_t0_dn0 - locals.var_t1_dn0)), ((locals.var_chi_dn2 - locals.var_chib_dn2) + (locals.var_t0_dn2 - locals.var_t1_dn2)), ((locals.var_chi_dn4 - locals.var_chib_dn4) + (locals.var_t0_dn4 - locals.var_t1_dn4)), ((locals.var_chi_dn5 - locals.var_chib_dn5) + (locals.var_t0_dn5 - locals.var_t1_dn5)), ((locals.var_chi_dn6 - locals.var_chib_dn6) + (locals.var_t0_dn6 - locals.var_t1_dn6)), ((locals.var_chi_dn7 - locals.var_chib_dn7) + (locals.var_t0_dn7 - locals.var_t1_dn7)), ((locals.var_chi_dn8 - locals.var_chib_dn8) + (locals.var_t0_dn8 - locals.var_t1_dn8)), ((locals.var_chi_dn9 - locals.var_chib_dn9) + (locals.var_t0_dn9 - locals.var_t1_dn9)), ((locals.var_chi_dn10 - locals.var_chib_dn10) + (locals.var_t0_dn10 - locals.var_t1_dn10)), ((locals.var_chi_dn11 - locals.var_chib_dn11) + (locals.var_t0_dn11 - locals.var_t1_dn11)), ((locals.var_chi_dn14 - locals.var_chib_dn14) + (locals.var_t0_dn14 - locals.var_t1_dn14)),)
    } else {
        (locals.var_fbsq__blk2127, locals.var_fbsq__blk2127_dn0, locals.var_fbsq__blk2127_dn2, locals.var_fbsq__blk2127_dn4, locals.var_fbsq__blk2127_dn5, locals.var_fbsq__blk2127_dn6, locals.var_fbsq__blk2127_dn7, locals.var_fbsq__blk2127_dn8, locals.var_fbsq__blk2127_dn9, locals.var_fbsq__blk2127_dn10, locals.var_fbsq__blk2127_dn11, locals.var_fbsq__blk2127_dn14,)
    }
};
            locals.var_fbsq__blk2127 = assign94760_body26_e146831;
            locals.var_fbsq__blk2127_dn0 = assign94760_body26_e146831_d_n0;
            locals.var_fbsq__blk2127_dn2 = assign94760_body26_e146831_d_n2;
            locals.var_fbsq__blk2127_dn4 = assign94760_body26_e146831_d_n4;
            locals.var_fbsq__blk2127_dn5 = assign94760_body26_e146831_d_n5;
            locals.var_fbsq__blk2127_dn6 = assign94760_body26_e146831_d_n6;
            locals.var_fbsq__blk2127_dn7 = assign94760_body26_e146831_d_n7;
            locals.var_fbsq__blk2127_dn8 = assign94760_body26_e146831_d_n8;
            locals.var_fbsq__blk2127_dn9 = assign94760_body26_e146831_d_n9;
            locals.var_fbsq__blk2127_dn10 = assign94760_body26_e146831_d_n10;
            locals.var_fbsq__blk2127_dn11 = assign94760_body26_e146831_d_n11;
            locals.var_fbsq__blk2127_dn14 = assign94760_body26_e146831_d_n14;
            locals.var_fbsq__blk2127_rv = 0.0;
            let (assign94760_body27_e146852, assign94760_body27_e146852_d_n0, assign94760_body27_e146852_d_n2, assign94760_body27_e146852_d_n4, assign94760_body27_e146852_d_n5, assign94760_body27_e146852_d_n6, assign94760_body27_e146852_d_n7, assign94760_body27_e146852_d_n8, assign94760_body27_e146852_d_n9, assign94760_body27_e146852_d_n10, assign94760_body27_e146852_d_n11, assign94760_body27_e146852_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2200 != 0.0)) && (locals.var_guard2206 == 0.0)) {
        let assign94760_body27_e146843: f64 = (1.0 - locals.var_t0);
        let assign94760_body27_e146847: f64 = (1.0 - locals.var_t1);
        let assign94760_body27_e146848: f64 = (locals.var_phi_b_dpss * assign94760_body27_e146847);
        let assign94760_body27_e146849: f64 = (assign94760_body27_e146843 - assign94760_body27_e146848);
        let assign94760_body27_e146850: f64 = (locals.var_beta * assign94760_body27_e146849);
        (assign94760_body27_e146850, ((locals.var_beta_dn0 * assign94760_body27_e146849) + (locals.var_beta * ((-locals.var_t0_dn0) - ((locals.var_phi_b_dpss_dn0 * assign94760_body27_e146847) + (locals.var_phi_b_dpss * (-locals.var_t1_dn0)))))), ((locals.var_beta_dn2 * assign94760_body27_e146849) + (locals.var_beta * ((-locals.var_t0_dn2) - ((locals.var_phi_b_dpss_dn2 * assign94760_body27_e146847) + (locals.var_phi_b_dpss * (-locals.var_t1_dn2)))))), ((locals.var_beta_dn4 * assign94760_body27_e146849) + (locals.var_beta * ((-locals.var_t0_dn4) - ((locals.var_phi_b_dpss_dn4 * assign94760_body27_e146847) + (locals.var_phi_b_dpss * (-locals.var_t1_dn4)))))), ((locals.var_beta_dn5 * assign94760_body27_e146849) + (locals.var_beta * ((-locals.var_t0_dn5) - ((locals.var_phi_b_dpss_dn5 * assign94760_body27_e146847) + (locals.var_phi_b_dpss * (-locals.var_t1_dn5)))))), ((locals.var_beta_dn6 * assign94760_body27_e146849) + (locals.var_beta * ((-locals.var_t0_dn6) - ((locals.var_phi_b_dpss_dn6 * assign94760_body27_e146847) + (locals.var_phi_b_dpss * (-locals.var_t1_dn6)))))), ((locals.var_beta_dn7 * assign94760_body27_e146849) + (locals.var_beta * ((-locals.var_t0_dn7) - ((locals.var_phi_b_dpss_dn7 * assign94760_body27_e146847) + (locals.var_phi_b_dpss * (-locals.var_t1_dn7)))))), ((locals.var_beta_dn8 * assign94760_body27_e146849) + (locals.var_beta * ((-locals.var_t0_dn8) - ((locals.var_phi_b_dpss_dn8 * assign94760_body27_e146847) + (locals.var_phi_b_dpss * (-locals.var_t1_dn8)))))), ((locals.var_beta_dn9 * assign94760_body27_e146849) + (locals.var_beta * ((-locals.var_t0_dn9) - ((locals.var_phi_b_dpss_dn9 * assign94760_body27_e146847) + (locals.var_phi_b_dpss * (-locals.var_t1_dn9)))))), ((locals.var_beta_dn10 * assign94760_body27_e146849) + (locals.var_beta * ((-locals.var_t0_dn10) - ((locals.var_phi_b_dpss_dn10 * assign94760_body27_e146847) + (locals.var_phi_b_dpss * (-locals.var_t1_dn10)))))), ((locals.var_beta_dn11 * assign94760_body27_e146849) + (locals.var_beta * ((-locals.var_t0_dn11) - ((locals.var_phi_b_dpss_dn11 * assign94760_body27_e146847) + (locals.var_phi_b_dpss * (-locals.var_t1_dn11)))))), ((locals.var_beta_dn14 * assign94760_body27_e146849) + (locals.var_beta * ((-locals.var_t0_dn14) - ((locals.var_phi_b_dpss_dn14 * assign94760_body27_e146847) + (locals.var_phi_b_dpss * (-locals.var_t1_dn14)))))),)
    } else {
        (locals.var_fbsq_dpss__blk2128, locals.var_fbsq_dpss__blk2128_dn0, locals.var_fbsq_dpss__blk2128_dn2, locals.var_fbsq_dpss__blk2128_dn4, locals.var_fbsq_dpss__blk2128_dn5, locals.var_fbsq_dpss__blk2128_dn6, locals.var_fbsq_dpss__blk2128_dn7, locals.var_fbsq_dpss__blk2128_dn8, locals.var_fbsq_dpss__blk2128_dn9, locals.var_fbsq_dpss__blk2128_dn10, locals.var_fbsq_dpss__blk2128_dn11, locals.var_fbsq_dpss__blk2128_dn14,)
    }
};
            locals.var_fbsq_dpss__blk2128 = assign94760_body27_e146852;
            locals.var_fbsq_dpss__blk2128_dn0 = assign94760_body27_e146852_d_n0;
            locals.var_fbsq_dpss__blk2128_dn2 = assign94760_body27_e146852_d_n2;
            locals.var_fbsq_dpss__blk2128_dn4 = assign94760_body27_e146852_d_n4;
            locals.var_fbsq_dpss__blk2128_dn5 = assign94760_body27_e146852_d_n5;
            locals.var_fbsq_dpss__blk2128_dn6 = assign94760_body27_e146852_d_n6;
            locals.var_fbsq_dpss__blk2128_dn7 = assign94760_body27_e146852_d_n7;
            locals.var_fbsq_dpss__blk2128_dn8 = assign94760_body27_e146852_d_n8;
            locals.var_fbsq_dpss__blk2128_dn9 = assign94760_body27_e146852_d_n9;
            locals.var_fbsq_dpss__blk2128_dn10 = assign94760_body27_e146852_d_n10;
            locals.var_fbsq_dpss__blk2128_dn11 = assign94760_body27_e146852_d_n11;
            locals.var_fbsq_dpss__blk2128_dn14 = assign94760_body27_e146852_d_n14;
            locals.var_fbsq_dpss__blk2128_rv = 0.0;
            let assign94760_body28_e146854: f64 = (locals.var_chi).abs();
            let assign94760_body28_e146856: f64 = if assign94760_body28_e146854 < 5e-5 { 1.0 } else { 0.0 };
            locals.var_guard2207 = assign94760_body28_e146856;
            locals.var_guard2207_rv = 0.0;
            let (assign94760_body29_e146888, assign94760_body29_e146888_d_n0, assign94760_body29_e146888_d_n2, assign94760_body29_e146888_d_n4, assign94760_body29_e146888_d_n5, assign94760_body29_e146888_d_n6, assign94760_body29_e146888_d_n7, assign94760_body29_e146888_d_n8, assign94760_body29_e146888_d_n9, assign94760_body29_e146888_d_n10, assign94760_body29_e146888_d_n11, assign94760_body29_e146888_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2200 != 0.0)) && (locals.var_guard2207 != 0.0)) {
        let assign94760_body29_e146866: f64 = (locals.var_chi * locals.var_chi);
        let assign94760_body29_e146868: f64 = (assign94760_body29_e146866 / 2.0);
        let assign94760_body29_e146872: f64 = (locals.var_chi / 3.0);
        let assign94760_body29_e146876: f64 = (locals.var_chi / 4.0);
        let assign94760_body29_e146880: f64 = (locals.var_chi / 5.0);
        let assign94760_body29_e146881: f64 = (1.0 + assign94760_body29_e146880);
        let assign94760_body29_e146882: f64 = (assign94760_body29_e146876 * assign94760_body29_e146881);
        let assign94760_body29_e146883: f64 = (1.0 + assign94760_body29_e146882);
        let assign94760_body29_e146884: f64 = (assign94760_body29_e146872 * assign94760_body29_e146883);
        let assign94760_body29_e146885: f64 = (1.0 + assign94760_body29_e146884);
        let assign94760_body29_e146886: f64 = (assign94760_body29_e146868 * assign94760_body29_e146885);
        (assign94760_body29_e146886, (((((locals.var_chi_dn0 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn0)) / 2.0) * assign94760_body29_e146885) + (assign94760_body29_e146868 * (((locals.var_chi_dn0 / 3.0) * assign94760_body29_e146883) + (assign94760_body29_e146872 * (((locals.var_chi_dn0 / 4.0) * assign94760_body29_e146881) + (assign94760_body29_e146876 * (locals.var_chi_dn0 / 5.0))))))), (((((locals.var_chi_dn2 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn2)) / 2.0) * assign94760_body29_e146885) + (assign94760_body29_e146868 * (((locals.var_chi_dn2 / 3.0) * assign94760_body29_e146883) + (assign94760_body29_e146872 * (((locals.var_chi_dn2 / 4.0) * assign94760_body29_e146881) + (assign94760_body29_e146876 * (locals.var_chi_dn2 / 5.0))))))), (((((locals.var_chi_dn4 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn4)) / 2.0) * assign94760_body29_e146885) + (assign94760_body29_e146868 * (((locals.var_chi_dn4 / 3.0) * assign94760_body29_e146883) + (assign94760_body29_e146872 * (((locals.var_chi_dn4 / 4.0) * assign94760_body29_e146881) + (assign94760_body29_e146876 * (locals.var_chi_dn4 / 5.0))))))), (((((locals.var_chi_dn5 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn5)) / 2.0) * assign94760_body29_e146885) + (assign94760_body29_e146868 * (((locals.var_chi_dn5 / 3.0) * assign94760_body29_e146883) + (assign94760_body29_e146872 * (((locals.var_chi_dn5 / 4.0) * assign94760_body29_e146881) + (assign94760_body29_e146876 * (locals.var_chi_dn5 / 5.0))))))), (((((locals.var_chi_dn6 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn6)) / 2.0) * assign94760_body29_e146885) + (assign94760_body29_e146868 * (((locals.var_chi_dn6 / 3.0) * assign94760_body29_e146883) + (assign94760_body29_e146872 * (((locals.var_chi_dn6 / 4.0) * assign94760_body29_e146881) + (assign94760_body29_e146876 * (locals.var_chi_dn6 / 5.0))))))), (((((locals.var_chi_dn7 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn7)) / 2.0) * assign94760_body29_e146885) + (assign94760_body29_e146868 * (((locals.var_chi_dn7 / 3.0) * assign94760_body29_e146883) + (assign94760_body29_e146872 * (((locals.var_chi_dn7 / 4.0) * assign94760_body29_e146881) + (assign94760_body29_e146876 * (locals.var_chi_dn7 / 5.0))))))), (((((locals.var_chi_dn8 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn8)) / 2.0) * assign94760_body29_e146885) + (assign94760_body29_e146868 * (((locals.var_chi_dn8 / 3.0) * assign94760_body29_e146883) + (assign94760_body29_e146872 * (((locals.var_chi_dn8 / 4.0) * assign94760_body29_e146881) + (assign94760_body29_e146876 * (locals.var_chi_dn8 / 5.0))))))), (((((locals.var_chi_dn9 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn9)) / 2.0) * assign94760_body29_e146885) + (assign94760_body29_e146868 * (((locals.var_chi_dn9 / 3.0) * assign94760_body29_e146883) + (assign94760_body29_e146872 * (((locals.var_chi_dn9 / 4.0) * assign94760_body29_e146881) + (assign94760_body29_e146876 * (locals.var_chi_dn9 / 5.0))))))), (((((locals.var_chi_dn10 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn10)) / 2.0) * assign94760_body29_e146885) + (assign94760_body29_e146868 * (((locals.var_chi_dn10 / 3.0) * assign94760_body29_e146883) + (assign94760_body29_e146872 * (((locals.var_chi_dn10 / 4.0) * assign94760_body29_e146881) + (assign94760_body29_e146876 * (locals.var_chi_dn10 / 5.0))))))), (((((locals.var_chi_dn11 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn11)) / 2.0) * assign94760_body29_e146885) + (assign94760_body29_e146868 * (((locals.var_chi_dn11 / 3.0) * assign94760_body29_e146883) + (assign94760_body29_e146872 * (((locals.var_chi_dn11 / 4.0) * assign94760_body29_e146881) + (assign94760_body29_e146876 * (locals.var_chi_dn11 / 5.0))))))), (((((locals.var_chi_dn14 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn14)) / 2.0) * assign94760_body29_e146885) + (assign94760_body29_e146868 * (((locals.var_chi_dn14 / 3.0) * assign94760_body29_e146883) + (assign94760_body29_e146872 * (((locals.var_chi_dn14 / 4.0) * assign94760_body29_e146881) + (assign94760_body29_e146876 * (locals.var_chi_dn14 / 5.0))))))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
            locals.var_t0 = assign94760_body29_e146888;
            locals.var_t0_dn0 = assign94760_body29_e146888_d_n0;
            locals.var_t0_dn2 = assign94760_body29_e146888_d_n2;
            locals.var_t0_dn4 = assign94760_body29_e146888_d_n4;
            locals.var_t0_dn5 = assign94760_body29_e146888_d_n5;
            locals.var_t0_dn6 = assign94760_body29_e146888_d_n6;
            locals.var_t0_dn7 = assign94760_body29_e146888_d_n7;
            locals.var_t0_dn8 = assign94760_body29_e146888_d_n8;
            locals.var_t0_dn9 = assign94760_body29_e146888_d_n9;
            locals.var_t0_dn10 = assign94760_body29_e146888_d_n10;
            locals.var_t0_dn11 = assign94760_body29_e146888_d_n11;
            locals.var_t0_dn14 = assign94760_body29_e146888_d_n14;
            locals.var_t0_rv = 0.0;
            let (assign94760_body30_e146916, assign94760_body30_e146916_d_n0, assign94760_body30_e146916_d_n2, assign94760_body30_e146916_d_n4, assign94760_body30_e146916_d_n5, assign94760_body30_e146916_d_n6, assign94760_body30_e146916_d_n7, assign94760_body30_e146916_d_n8, assign94760_body30_e146916_d_n9, assign94760_body30_e146916_d_n10, assign94760_body30_e146916_d_n11, assign94760_body30_e146916_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2200 != 0.0)) && (locals.var_guard2207 != 0.0)) {
        let assign94760_body30_e146900: f64 = (locals.var_chi / 2.0);
        let assign94760_body30_e146904: f64 = (locals.var_chi / 3.0);
        let assign94760_body30_e146908: f64 = (locals.var_chi / 4.0);
        let assign94760_body30_e146909: f64 = (1.0 + assign94760_body30_e146908);
        let assign94760_body30_e146910: f64 = (assign94760_body30_e146904 * assign94760_body30_e146909);
        let assign94760_body30_e146911: f64 = (1.0 + assign94760_body30_e146910);
        let assign94760_body30_e146912: f64 = (assign94760_body30_e146900 * assign94760_body30_e146911);
        let assign94760_body30_e146913: f64 = (1.0 + assign94760_body30_e146912);
        let assign94760_body30_e146914: f64 = (locals.var_chi * assign94760_body30_e146913);
        (assign94760_body30_e146914, ((locals.var_chi_dn0 * assign94760_body30_e146913) + (locals.var_chi * (((locals.var_chi_dn0 / 2.0) * assign94760_body30_e146911) + (assign94760_body30_e146900 * (((locals.var_chi_dn0 / 3.0) * assign94760_body30_e146909) + (assign94760_body30_e146904 * (locals.var_chi_dn0 / 4.0))))))), ((locals.var_chi_dn2 * assign94760_body30_e146913) + (locals.var_chi * (((locals.var_chi_dn2 / 2.0) * assign94760_body30_e146911) + (assign94760_body30_e146900 * (((locals.var_chi_dn2 / 3.0) * assign94760_body30_e146909) + (assign94760_body30_e146904 * (locals.var_chi_dn2 / 4.0))))))), ((locals.var_chi_dn4 * assign94760_body30_e146913) + (locals.var_chi * (((locals.var_chi_dn4 / 2.0) * assign94760_body30_e146911) + (assign94760_body30_e146900 * (((locals.var_chi_dn4 / 3.0) * assign94760_body30_e146909) + (assign94760_body30_e146904 * (locals.var_chi_dn4 / 4.0))))))), ((locals.var_chi_dn5 * assign94760_body30_e146913) + (locals.var_chi * (((locals.var_chi_dn5 / 2.0) * assign94760_body30_e146911) + (assign94760_body30_e146900 * (((locals.var_chi_dn5 / 3.0) * assign94760_body30_e146909) + (assign94760_body30_e146904 * (locals.var_chi_dn5 / 4.0))))))), ((locals.var_chi_dn6 * assign94760_body30_e146913) + (locals.var_chi * (((locals.var_chi_dn6 / 2.0) * assign94760_body30_e146911) + (assign94760_body30_e146900 * (((locals.var_chi_dn6 / 3.0) * assign94760_body30_e146909) + (assign94760_body30_e146904 * (locals.var_chi_dn6 / 4.0))))))), ((locals.var_chi_dn7 * assign94760_body30_e146913) + (locals.var_chi * (((locals.var_chi_dn7 / 2.0) * assign94760_body30_e146911) + (assign94760_body30_e146900 * (((locals.var_chi_dn7 / 3.0) * assign94760_body30_e146909) + (assign94760_body30_e146904 * (locals.var_chi_dn7 / 4.0))))))), ((locals.var_chi_dn8 * assign94760_body30_e146913) + (locals.var_chi * (((locals.var_chi_dn8 / 2.0) * assign94760_body30_e146911) + (assign94760_body30_e146900 * (((locals.var_chi_dn8 / 3.0) * assign94760_body30_e146909) + (assign94760_body30_e146904 * (locals.var_chi_dn8 / 4.0))))))), ((locals.var_chi_dn9 * assign94760_body30_e146913) + (locals.var_chi * (((locals.var_chi_dn9 / 2.0) * assign94760_body30_e146911) + (assign94760_body30_e146900 * (((locals.var_chi_dn9 / 3.0) * assign94760_body30_e146909) + (assign94760_body30_e146904 * (locals.var_chi_dn9 / 4.0))))))), ((locals.var_chi_dn10 * assign94760_body30_e146913) + (locals.var_chi * (((locals.var_chi_dn10 / 2.0) * assign94760_body30_e146911) + (assign94760_body30_e146900 * (((locals.var_chi_dn10 / 3.0) * assign94760_body30_e146909) + (assign94760_body30_e146904 * (locals.var_chi_dn10 / 4.0))))))), ((locals.var_chi_dn11 * assign94760_body30_e146913) + (locals.var_chi * (((locals.var_chi_dn11 / 2.0) * assign94760_body30_e146911) + (assign94760_body30_e146900 * (((locals.var_chi_dn11 / 3.0) * assign94760_body30_e146909) + (assign94760_body30_e146904 * (locals.var_chi_dn11 / 4.0))))))), ((locals.var_chi_dn14 * assign94760_body30_e146913) + (locals.var_chi * (((locals.var_chi_dn14 / 2.0) * assign94760_body30_e146911) + (assign94760_body30_e146900 * (((locals.var_chi_dn14 / 3.0) * assign94760_body30_e146909) + (assign94760_body30_e146904 * (locals.var_chi_dn14 / 4.0))))))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
            locals.var_t1 = assign94760_body30_e146916;
            locals.var_t1_dn0 = assign94760_body30_e146916_d_n0;
            locals.var_t1_dn2 = assign94760_body30_e146916_d_n2;
            locals.var_t1_dn4 = assign94760_body30_e146916_d_n4;
            locals.var_t1_dn5 = assign94760_body30_e146916_d_n5;
            locals.var_t1_dn6 = assign94760_body30_e146916_d_n6;
            locals.var_t1_dn7 = assign94760_body30_e146916_d_n7;
            locals.var_t1_dn8 = assign94760_body30_e146916_d_n8;
            locals.var_t1_dn9 = assign94760_body30_e146916_d_n9;
            locals.var_t1_dn10 = assign94760_body30_e146916_d_n10;
            locals.var_t1_dn11 = assign94760_body30_e146916_d_n11;
            locals.var_t1_dn14 = assign94760_body30_e146916_d_n14;
            locals.var_t1_rv = 0.0;
            let (assign94760_body31_e146928, assign94760_body31_e146928_d_n0, assign94760_body31_e146928_d_n2, assign94760_body31_e146928_d_n4, assign94760_body31_e146928_d_n5, assign94760_body31_e146928_d_n6, assign94760_body31_e146928_d_n7, assign94760_body31_e146928_d_n8, assign94760_body31_e146928_d_n9, assign94760_body31_e146928_d_n10, assign94760_body31_e146928_d_n11, assign94760_body31_e146928_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2200 != 0.0)) && (locals.var_guard2207 != 0.0)) {
        let assign94760_body31_e146926: f64 = (locals.var_cfs1 * locals.var_t0);
        (assign94760_body31_e146926, ((locals.var_cfs1_dn0 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn0)), ((locals.var_cfs1_dn2 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn2)), ((locals.var_cfs1_dn4 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn4)), ((locals.var_cfs1_dn5 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn5)), ((locals.var_cfs1_dn6 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn6)), ((locals.var_cfs1_dn7 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn7)), ((locals.var_cfs1_dn8 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn8)), ((locals.var_cfs1_dn9 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn9)), ((locals.var_cfs1_dn10 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn10)), ((locals.var_cfs1_dn11 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn11)), ((locals.var_cfs1_dn14 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn14)),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn11, locals.var_fs01_dn14,)
    }
};
            locals.var_fs01 = assign94760_body31_e146928;
            locals.var_fs01_dn0 = assign94760_body31_e146928_d_n0;
            locals.var_fs01_dn2 = assign94760_body31_e146928_d_n2;
            locals.var_fs01_dn4 = assign94760_body31_e146928_d_n4;
            locals.var_fs01_dn5 = assign94760_body31_e146928_d_n5;
            locals.var_fs01_dn6 = assign94760_body31_e146928_d_n6;
            locals.var_fs01_dn7 = assign94760_body31_e146928_d_n7;
            locals.var_fs01_dn8 = assign94760_body31_e146928_d_n8;
            locals.var_fs01_dn9 = assign94760_body31_e146928_d_n9;
            locals.var_fs01_dn10 = assign94760_body31_e146928_d_n10;
            locals.var_fs01_dn11 = assign94760_body31_e146928_d_n11;
            locals.var_fs01_dn14 = assign94760_body31_e146928_d_n14;
            locals.var_fs01_rv = 0.0;
            let (assign94760_body32_e146942, assign94760_body32_e146942_d_n0, assign94760_body32_e146942_d_n2, assign94760_body32_e146942_d_n4, assign94760_body32_e146942_d_n5, assign94760_body32_e146942_d_n6, assign94760_body32_e146942_d_n7, assign94760_body32_e146942_d_n8, assign94760_body32_e146942_d_n9, assign94760_body32_e146942_d_n10, assign94760_body32_e146942_d_n11, assign94760_body32_e146942_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2200 != 0.0)) && (locals.var_guard2207 != 0.0)) {
        let assign94760_body32_e146938: f64 = (locals.var_cfs1 * locals.var_t1);
        let assign94760_body32_e146940: f64 = (assign94760_body32_e146938 * locals.var_beta);
        (assign94760_body32_e146940, ((((locals.var_cfs1_dn0 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn0)) * locals.var_beta) + (assign94760_body32_e146938 * locals.var_beta_dn0)), ((((locals.var_cfs1_dn2 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn2)) * locals.var_beta) + (assign94760_body32_e146938 * locals.var_beta_dn2)), ((((locals.var_cfs1_dn4 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn4)) * locals.var_beta) + (assign94760_body32_e146938 * locals.var_beta_dn4)), ((((locals.var_cfs1_dn5 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn5)) * locals.var_beta) + (assign94760_body32_e146938 * locals.var_beta_dn5)), ((((locals.var_cfs1_dn6 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn6)) * locals.var_beta) + (assign94760_body32_e146938 * locals.var_beta_dn6)), ((((locals.var_cfs1_dn7 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn7)) * locals.var_beta) + (assign94760_body32_e146938 * locals.var_beta_dn7)), ((((locals.var_cfs1_dn8 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn8)) * locals.var_beta) + (assign94760_body32_e146938 * locals.var_beta_dn8)), ((((locals.var_cfs1_dn9 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn9)) * locals.var_beta) + (assign94760_body32_e146938 * locals.var_beta_dn9)), ((((locals.var_cfs1_dn10 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn10)) * locals.var_beta) + (assign94760_body32_e146938 * locals.var_beta_dn10)), ((((locals.var_cfs1_dn11 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn11)) * locals.var_beta) + (assign94760_body32_e146938 * locals.var_beta_dn11)), ((((locals.var_cfs1_dn14 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn14)) * locals.var_beta) + (assign94760_body32_e146938 * locals.var_beta_dn14)),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn11, locals.var_fs01_dps0_dn14,)
    }
};
            locals.var_fs01_dps0 = assign94760_body32_e146942;
            locals.var_fs01_dps0_dn0 = assign94760_body32_e146942_d_n0;
            locals.var_fs01_dps0_dn2 = assign94760_body32_e146942_d_n2;
            locals.var_fs01_dps0_dn4 = assign94760_body32_e146942_d_n4;
            locals.var_fs01_dps0_dn5 = assign94760_body32_e146942_d_n5;
            locals.var_fs01_dps0_dn6 = assign94760_body32_e146942_d_n6;
            locals.var_fs01_dps0_dn7 = assign94760_body32_e146942_d_n7;
            locals.var_fs01_dps0_dn8 = assign94760_body32_e146942_d_n8;
            locals.var_fs01_dps0_dn9 = assign94760_body32_e146942_d_n9;
            locals.var_fs01_dps0_dn10 = assign94760_body32_e146942_d_n10;
            locals.var_fs01_dps0_dn11 = assign94760_body32_e146942_d_n11;
            locals.var_fs01_dps0_dn14 = assign94760_body32_e146942_d_n14;
            locals.var_fs01_dps0_rv = 0.0;
            let assign94760_body33_e146944: f64 = (locals.var_chi).abs();
            let assign94760_body33_e146946: f64 = if assign94760_body33_e146944 < 60.0 { 1.0 } else { 0.0 };
            locals.var_guard2208 = assign94760_body33_e146946;
            locals.var_guard2208_rv = 0.0;
            let (assign94760_body35_e146981, assign94760_body35_e146981_d_n0, assign94760_body35_e146981_d_n2, assign94760_body35_e146981_d_n4, assign94760_body35_e146981_d_n5, assign94760_body35_e146981_d_n6, assign94760_body35_e146981_d_n7, assign94760_body35_e146981_d_n8, assign94760_body35_e146981_d_n9, assign94760_body35_e146981_d_n10, assign94760_body35_e146981_d_n11, assign94760_body35_e146981_d_n14,) = {
    if (((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2200 != 0.0)) && (locals.var_guard2207 == 0.0)) && (locals.var_guard2208 != 0.0)) {
        let assign94760_body35_e146979: f64 = (locals.var_chi).exp();
        (assign94760_body35_e146979, (assign94760_body35_e146979 * locals.var_chi_dn0), (assign94760_body35_e146979 * locals.var_chi_dn2), (assign94760_body35_e146979 * locals.var_chi_dn4), (assign94760_body35_e146979 * locals.var_chi_dn5), (assign94760_body35_e146979 * locals.var_chi_dn6), (assign94760_body35_e146979 * locals.var_chi_dn7), (assign94760_body35_e146979 * locals.var_chi_dn8), (assign94760_body35_e146979 * locals.var_chi_dn9), (assign94760_body35_e146979 * locals.var_chi_dn10), (assign94760_body35_e146979 * locals.var_chi_dn11), (assign94760_body35_e146979 * locals.var_chi_dn14),)
    } else {
        (locals.var_exp_chi, locals.var_exp_chi_dn0, locals.var_exp_chi_dn2, locals.var_exp_chi_dn4, locals.var_exp_chi_dn5, locals.var_exp_chi_dn6, locals.var_exp_chi_dn7, locals.var_exp_chi_dn8, locals.var_exp_chi_dn9, locals.var_exp_chi_dn10, locals.var_exp_chi_dn11, locals.var_exp_chi_dn14,)
    }
};
            locals.var_exp_chi = assign94760_body35_e146981;
            locals.var_exp_chi_dn0 = assign94760_body35_e146981_d_n0;
            locals.var_exp_chi_dn2 = assign94760_body35_e146981_d_n2;
            locals.var_exp_chi_dn4 = assign94760_body35_e146981_d_n4;
            locals.var_exp_chi_dn5 = assign94760_body35_e146981_d_n5;
            locals.var_exp_chi_dn6 = assign94760_body35_e146981_d_n6;
            locals.var_exp_chi_dn7 = assign94760_body35_e146981_d_n7;
            locals.var_exp_chi_dn8 = assign94760_body35_e146981_d_n8;
            locals.var_exp_chi_dn9 = assign94760_body35_e146981_d_n9;
            locals.var_exp_chi_dn10 = assign94760_body35_e146981_d_n10;
            locals.var_exp_chi_dn11 = assign94760_body35_e146981_d_n11;
            locals.var_exp_chi_dn14 = assign94760_body35_e146981_d_n14;
            locals.var_exp_chi_rv = 0.0;
            let (assign94760_body36_e146996, assign94760_body36_e146996_d_n0, assign94760_body36_e146996_d_n2, assign94760_body36_e146996_d_n4, assign94760_body36_e146996_d_n5, assign94760_body36_e146996_d_n6, assign94760_body36_e146996_d_n7, assign94760_body36_e146996_d_n8, assign94760_body36_e146996_d_n9, assign94760_body36_e146996_d_n10, assign94760_body36_e146996_d_n11, assign94760_body36_e146996_d_n14,) = {
    if (((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2200 != 0.0)) && (locals.var_guard2207 == 0.0)) && (locals.var_guard2208 != 0.0)) {
        let assign94760_body36_e146994: f64 = (locals.var_exp_chi - 1.0);
        (assign94760_body36_e146994, locals.var_exp_chi_dn0, locals.var_exp_chi_dn2, locals.var_exp_chi_dn4, locals.var_exp_chi_dn5, locals.var_exp_chi_dn6, locals.var_exp_chi_dn7, locals.var_exp_chi_dn8, locals.var_exp_chi_dn9, locals.var_exp_chi_dn10, locals.var_exp_chi_dn11, locals.var_exp_chi_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
            locals.var_t1 = assign94760_body36_e146996;
            locals.var_t1_dn0 = assign94760_body36_e146996_d_n0;
            locals.var_t1_dn2 = assign94760_body36_e146996_d_n2;
            locals.var_t1_dn4 = assign94760_body36_e146996_d_n4;
            locals.var_t1_dn5 = assign94760_body36_e146996_d_n5;
            locals.var_t1_dn6 = assign94760_body36_e146996_d_n6;
            locals.var_t1_dn7 = assign94760_body36_e146996_d_n7;
            locals.var_t1_dn8 = assign94760_body36_e146996_d_n8;
            locals.var_t1_dn9 = assign94760_body36_e146996_d_n9;
            locals.var_t1_dn10 = assign94760_body36_e146996_d_n10;
            locals.var_t1_dn11 = assign94760_body36_e146996_d_n11;
            locals.var_t1_dn14 = assign94760_body36_e146996_d_n14;
            locals.var_t1_rv = 0.0;
            let (assign94760_body37_e147013, assign94760_body37_e147013_d_n0, assign94760_body37_e147013_d_n2, assign94760_body37_e147013_d_n4, assign94760_body37_e147013_d_n5, assign94760_body37_e147013_d_n6, assign94760_body37_e147013_d_n7, assign94760_body37_e147013_d_n8, assign94760_body37_e147013_d_n9, assign94760_body37_e147013_d_n10, assign94760_body37_e147013_d_n11, assign94760_body37_e147013_d_n14,) = {
    if (((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2200 != 0.0)) && (locals.var_guard2207 == 0.0)) && (locals.var_guard2208 != 0.0)) {
        let assign94760_body37_e147010: f64 = (locals.var_t1 - locals.var_chi);
        let assign94760_body37_e147011: f64 = (locals.var_cfs1 * assign94760_body37_e147010);
        (assign94760_body37_e147011, ((locals.var_cfs1_dn0 * assign94760_body37_e147010) + (locals.var_cfs1 * (locals.var_t1_dn0 - locals.var_chi_dn0))), ((locals.var_cfs1_dn2 * assign94760_body37_e147010) + (locals.var_cfs1 * (locals.var_t1_dn2 - locals.var_chi_dn2))), ((locals.var_cfs1_dn4 * assign94760_body37_e147010) + (locals.var_cfs1 * (locals.var_t1_dn4 - locals.var_chi_dn4))), ((locals.var_cfs1_dn5 * assign94760_body37_e147010) + (locals.var_cfs1 * (locals.var_t1_dn5 - locals.var_chi_dn5))), ((locals.var_cfs1_dn6 * assign94760_body37_e147010) + (locals.var_cfs1 * (locals.var_t1_dn6 - locals.var_chi_dn6))), ((locals.var_cfs1_dn7 * assign94760_body37_e147010) + (locals.var_cfs1 * (locals.var_t1_dn7 - locals.var_chi_dn7))), ((locals.var_cfs1_dn8 * assign94760_body37_e147010) + (locals.var_cfs1 * (locals.var_t1_dn8 - locals.var_chi_dn8))), ((locals.var_cfs1_dn9 * assign94760_body37_e147010) + (locals.var_cfs1 * (locals.var_t1_dn9 - locals.var_chi_dn9))), ((locals.var_cfs1_dn10 * assign94760_body37_e147010) + (locals.var_cfs1 * (locals.var_t1_dn10 - locals.var_chi_dn10))), ((locals.var_cfs1_dn11 * assign94760_body37_e147010) + (locals.var_cfs1 * (locals.var_t1_dn11 - locals.var_chi_dn11))), ((locals.var_cfs1_dn14 * assign94760_body37_e147010) + (locals.var_cfs1 * (locals.var_t1_dn14 - locals.var_chi_dn14))),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn11, locals.var_fs01_dn14,)
    }
};
            locals.var_fs01 = assign94760_body37_e147013;
            locals.var_fs01_dn0 = assign94760_body37_e147013_d_n0;
            locals.var_fs01_dn2 = assign94760_body37_e147013_d_n2;
            locals.var_fs01_dn4 = assign94760_body37_e147013_d_n4;
            locals.var_fs01_dn5 = assign94760_body37_e147013_d_n5;
            locals.var_fs01_dn6 = assign94760_body37_e147013_d_n6;
            locals.var_fs01_dn7 = assign94760_body37_e147013_d_n7;
            locals.var_fs01_dn8 = assign94760_body37_e147013_d_n8;
            locals.var_fs01_dn9 = assign94760_body37_e147013_d_n9;
            locals.var_fs01_dn10 = assign94760_body37_e147013_d_n10;
            locals.var_fs01_dn11 = assign94760_body37_e147013_d_n11;
            locals.var_fs01_dn14 = assign94760_body37_e147013_d_n14;
            locals.var_fs01_rv = 0.0;
            let (assign94760_body38_e147030, assign94760_body38_e147030_d_n0, assign94760_body38_e147030_d_n2, assign94760_body38_e147030_d_n4, assign94760_body38_e147030_d_n5, assign94760_body38_e147030_d_n6, assign94760_body38_e147030_d_n7, assign94760_body38_e147030_d_n8, assign94760_body38_e147030_d_n9, assign94760_body38_e147030_d_n10, assign94760_body38_e147030_d_n11, assign94760_body38_e147030_d_n14,) = {
    if (((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2200 != 0.0)) && (locals.var_guard2207 == 0.0)) && (locals.var_guard2208 != 0.0)) {
        let assign94760_body38_e147026: f64 = (locals.var_cfs1 * locals.var_beta);
        let assign94760_body38_e147028: f64 = (assign94760_body38_e147026 * locals.var_t1);
        (assign94760_body38_e147028, ((((locals.var_cfs1_dn0 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn0)) * locals.var_t1) + (assign94760_body38_e147026 * locals.var_t1_dn0)), ((((locals.var_cfs1_dn2 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn2)) * locals.var_t1) + (assign94760_body38_e147026 * locals.var_t1_dn2)), ((((locals.var_cfs1_dn4 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn4)) * locals.var_t1) + (assign94760_body38_e147026 * locals.var_t1_dn4)), ((((locals.var_cfs1_dn5 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn5)) * locals.var_t1) + (assign94760_body38_e147026 * locals.var_t1_dn5)), ((((locals.var_cfs1_dn6 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn6)) * locals.var_t1) + (assign94760_body38_e147026 * locals.var_t1_dn6)), ((((locals.var_cfs1_dn7 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn7)) * locals.var_t1) + (assign94760_body38_e147026 * locals.var_t1_dn7)), ((((locals.var_cfs1_dn8 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn8)) * locals.var_t1) + (assign94760_body38_e147026 * locals.var_t1_dn8)), ((((locals.var_cfs1_dn9 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn9)) * locals.var_t1) + (assign94760_body38_e147026 * locals.var_t1_dn9)), ((((locals.var_cfs1_dn10 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn10)) * locals.var_t1) + (assign94760_body38_e147026 * locals.var_t1_dn10)), ((((locals.var_cfs1_dn11 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn11)) * locals.var_t1) + (assign94760_body38_e147026 * locals.var_t1_dn11)), ((((locals.var_cfs1_dn14 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn14)) * locals.var_t1) + (assign94760_body38_e147026 * locals.var_t1_dn14)),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn11, locals.var_fs01_dps0_dn14,)
    }
};
            locals.var_fs01_dps0 = assign94760_body38_e147030;
            locals.var_fs01_dps0_dn0 = assign94760_body38_e147030_d_n0;
            locals.var_fs01_dps0_dn2 = assign94760_body38_e147030_d_n2;
            locals.var_fs01_dps0_dn4 = assign94760_body38_e147030_d_n4;
            locals.var_fs01_dps0_dn5 = assign94760_body38_e147030_d_n5;
            locals.var_fs01_dps0_dn6 = assign94760_body38_e147030_d_n6;
            locals.var_fs01_dps0_dn7 = assign94760_body38_e147030_d_n7;
            locals.var_fs01_dps0_dn8 = assign94760_body38_e147030_d_n8;
            locals.var_fs01_dps0_dn9 = assign94760_body38_e147030_d_n9;
            locals.var_fs01_dps0_dn10 = assign94760_body38_e147030_d_n10;
            locals.var_fs01_dps0_dn11 = assign94760_body38_e147030_d_n11;
            locals.var_fs01_dps0_dn14 = assign94760_body38_e147030_d_n14;
            locals.var_fs01_dps0_rv = 0.0;
            let (assign94760_body40_e147069, assign94760_body40_e147069_d_n0, assign94760_body40_e147069_d_n2, assign94760_body40_e147069_d_n4, assign94760_body40_e147069_d_n5, assign94760_body40_e147069_d_n6, assign94760_body40_e147069_d_n7, assign94760_body40_e147069_d_n8, assign94760_body40_e147069_d_n9, assign94760_body40_e147069_d_n10, assign94760_body40_e147069_d_n11, assign94760_body40_e147069_d_n14,) = {
    if (((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2200 != 0.0)) && (locals.var_guard2207 == 0.0)) && (locals.var_guard2208 == 0.0)) {
        let assign94760_body40_e147066: f64 = (locals.var_beta * locals.var_ps0ld);
        let assign94760_body40_e147067: f64 = (assign94760_body40_e147066).exp();
        (assign94760_body40_e147067, (assign94760_body40_e147067 * ((locals.var_beta_dn0 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn0))), (assign94760_body40_e147067 * ((locals.var_beta_dn2 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn2))), (assign94760_body40_e147067 * ((locals.var_beta_dn4 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn4))), (assign94760_body40_e147067 * ((locals.var_beta_dn5 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn5))), (assign94760_body40_e147067 * ((locals.var_beta_dn6 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn6))), (assign94760_body40_e147067 * ((locals.var_beta_dn7 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn7))), (assign94760_body40_e147067 * ((locals.var_beta_dn8 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn8))), (assign94760_body40_e147067 * ((locals.var_beta_dn9 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn9))), (assign94760_body40_e147067 * ((locals.var_beta_dn10 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn10))), (assign94760_body40_e147067 * ((locals.var_beta_dn11 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn11))), (assign94760_body40_e147067 * ((locals.var_beta_dn14 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn14))),)
    } else {
        (locals.var_exp_bps0, locals.var_exp_bps0_dn0, locals.var_exp_bps0_dn2, locals.var_exp_bps0_dn4, locals.var_exp_bps0_dn5, locals.var_exp_bps0_dn6, locals.var_exp_bps0_dn7, locals.var_exp_bps0_dn8, locals.var_exp_bps0_dn9, locals.var_exp_bps0_dn10, locals.var_exp_bps0_dn11, locals.var_exp_bps0_dn14,)
    }
};
            locals.var_exp_bps0 = assign94760_body40_e147069;
            locals.var_exp_bps0_dn0 = assign94760_body40_e147069_d_n0;
            locals.var_exp_bps0_dn2 = assign94760_body40_e147069_d_n2;
            locals.var_exp_bps0_dn4 = assign94760_body40_e147069_d_n4;
            locals.var_exp_bps0_dn5 = assign94760_body40_e147069_d_n5;
            locals.var_exp_bps0_dn6 = assign94760_body40_e147069_d_n6;
            locals.var_exp_bps0_dn7 = assign94760_body40_e147069_d_n7;
            locals.var_exp_bps0_dn8 = assign94760_body40_e147069_d_n8;
            locals.var_exp_bps0_dn9 = assign94760_body40_e147069_d_n9;
            locals.var_exp_bps0_dn10 = assign94760_body40_e147069_d_n10;
            locals.var_exp_bps0_dn11 = assign94760_body40_e147069_d_n11;
            locals.var_exp_bps0_dn14 = assign94760_body40_e147069_d_n14;
            locals.var_exp_bps0_rv = 0.0;
            let (assign94760_body41_e147091, assign94760_body41_e147091_d_n0, assign94760_body41_e147091_d_n2, assign94760_body41_e147091_d_n4, assign94760_body41_e147091_d_n5, assign94760_body41_e147091_d_n6, assign94760_body41_e147091_d_n7, assign94760_body41_e147091_d_n8, assign94760_body41_e147091_d_n9, assign94760_body41_e147091_d_n10, assign94760_body41_e147091_d_n11, assign94760_body41_e147091_d_n14,) = {
    if (((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2200 != 0.0)) && (locals.var_guard2207 == 0.0)) && (locals.var_guard2208 == 0.0)) {
        let assign94760_body41_e147086: f64 = (locals.var_chi + 1.0);
        let assign94760_body41_e147087: f64 = (locals.var_exp_bvbs * assign94760_body41_e147086);
        let assign94760_body41_e147088: f64 = (locals.var_exp_bps0 - assign94760_body41_e147087);
        let assign94760_body41_e147089: f64 = (locals.var_cnst1over * assign94760_body41_e147088);
        (assign94760_body41_e147089, ((locals.var_cnst1over_dn0 * assign94760_body41_e147088) + (locals.var_cnst1over * (locals.var_exp_bps0_dn0 - ((locals.var_exp_bvbs_dn0 * assign94760_body41_e147086) + (locals.var_exp_bvbs * locals.var_chi_dn0))))), ((locals.var_cnst1over_dn2 * assign94760_body41_e147088) + (locals.var_cnst1over * (locals.var_exp_bps0_dn2 - ((locals.var_exp_bvbs_dn2 * assign94760_body41_e147086) + (locals.var_exp_bvbs * locals.var_chi_dn2))))), ((locals.var_cnst1over_dn4 * assign94760_body41_e147088) + (locals.var_cnst1over * (locals.var_exp_bps0_dn4 - ((locals.var_exp_bvbs_dn4 * assign94760_body41_e147086) + (locals.var_exp_bvbs * locals.var_chi_dn4))))), ((locals.var_cnst1over_dn5 * assign94760_body41_e147088) + (locals.var_cnst1over * (locals.var_exp_bps0_dn5 - ((locals.var_exp_bvbs_dn5 * assign94760_body41_e147086) + (locals.var_exp_bvbs * locals.var_chi_dn5))))), ((locals.var_cnst1over_dn6 * assign94760_body41_e147088) + (locals.var_cnst1over * (locals.var_exp_bps0_dn6 - ((locals.var_exp_bvbs_dn6 * assign94760_body41_e147086) + (locals.var_exp_bvbs * locals.var_chi_dn6))))), ((locals.var_cnst1over_dn7 * assign94760_body41_e147088) + (locals.var_cnst1over * (locals.var_exp_bps0_dn7 - ((locals.var_exp_bvbs_dn7 * assign94760_body41_e147086) + (locals.var_exp_bvbs * locals.var_chi_dn7))))), ((locals.var_cnst1over_dn8 * assign94760_body41_e147088) + (locals.var_cnst1over * (locals.var_exp_bps0_dn8 - ((locals.var_exp_bvbs_dn8 * assign94760_body41_e147086) + (locals.var_exp_bvbs * locals.var_chi_dn8))))), ((locals.var_cnst1over_dn9 * assign94760_body41_e147088) + (locals.var_cnst1over * (locals.var_exp_bps0_dn9 - ((locals.var_exp_bvbs_dn9 * assign94760_body41_e147086) + (locals.var_exp_bvbs * locals.var_chi_dn9))))), ((locals.var_cnst1over_dn10 * assign94760_body41_e147088) + (locals.var_cnst1over * (locals.var_exp_bps0_dn10 - ((locals.var_exp_bvbs_dn10 * assign94760_body41_e147086) + (locals.var_exp_bvbs * locals.var_chi_dn10))))), ((locals.var_cnst1over_dn11 * assign94760_body41_e147088) + (locals.var_cnst1over * (locals.var_exp_bps0_dn11 - ((locals.var_exp_bvbs_dn11 * assign94760_body41_e147086) + (locals.var_exp_bvbs * locals.var_chi_dn11))))), ((locals.var_cnst1over_dn14 * assign94760_body41_e147088) + (locals.var_cnst1over * (locals.var_exp_bps0_dn14 - ((locals.var_exp_bvbs_dn14 * assign94760_body41_e147086) + (locals.var_exp_bvbs * locals.var_chi_dn14))))),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn11, locals.var_fs01_dn14,)
    }
};
            locals.var_fs01 = assign94760_body41_e147091;
            locals.var_fs01_dn0 = assign94760_body41_e147091_d_n0;
            locals.var_fs01_dn2 = assign94760_body41_e147091_d_n2;
            locals.var_fs01_dn4 = assign94760_body41_e147091_d_n4;
            locals.var_fs01_dn5 = assign94760_body41_e147091_d_n5;
            locals.var_fs01_dn6 = assign94760_body41_e147091_d_n6;
            locals.var_fs01_dn7 = assign94760_body41_e147091_d_n7;
            locals.var_fs01_dn8 = assign94760_body41_e147091_d_n8;
            locals.var_fs01_dn9 = assign94760_body41_e147091_d_n9;
            locals.var_fs01_dn10 = assign94760_body41_e147091_d_n10;
            locals.var_fs01_dn11 = assign94760_body41_e147091_d_n11;
            locals.var_fs01_dn14 = assign94760_body41_e147091_d_n14;
            locals.var_fs01_rv = 0.0;
            let (assign94760_body42_e147111, assign94760_body42_e147111_d_n0, assign94760_body42_e147111_d_n2, assign94760_body42_e147111_d_n4, assign94760_body42_e147111_d_n5, assign94760_body42_e147111_d_n6, assign94760_body42_e147111_d_n7, assign94760_body42_e147111_d_n8, assign94760_body42_e147111_d_n9, assign94760_body42_e147111_d_n10, assign94760_body42_e147111_d_n11, assign94760_body42_e147111_d_n14,) = {
    if (((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2200 != 0.0)) && (locals.var_guard2207 == 0.0)) && (locals.var_guard2208 == 0.0)) {
        let assign94760_body42_e147105: f64 = (locals.var_cnst1over * locals.var_beta);
        let assign94760_body42_e147108: f64 = (locals.var_exp_bps0 - locals.var_exp_bvbs);
        let assign94760_body42_e147109: f64 = (assign94760_body42_e147105 * assign94760_body42_e147108);
        (assign94760_body42_e147109, ((((locals.var_cnst1over_dn0 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn0)) * assign94760_body42_e147108) + (assign94760_body42_e147105 * (locals.var_exp_bps0_dn0 - locals.var_exp_bvbs_dn0))), ((((locals.var_cnst1over_dn2 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn2)) * assign94760_body42_e147108) + (assign94760_body42_e147105 * (locals.var_exp_bps0_dn2 - locals.var_exp_bvbs_dn2))), ((((locals.var_cnst1over_dn4 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn4)) * assign94760_body42_e147108) + (assign94760_body42_e147105 * (locals.var_exp_bps0_dn4 - locals.var_exp_bvbs_dn4))), ((((locals.var_cnst1over_dn5 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn5)) * assign94760_body42_e147108) + (assign94760_body42_e147105 * (locals.var_exp_bps0_dn5 - locals.var_exp_bvbs_dn5))), ((((locals.var_cnst1over_dn6 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn6)) * assign94760_body42_e147108) + (assign94760_body42_e147105 * (locals.var_exp_bps0_dn6 - locals.var_exp_bvbs_dn6))), ((((locals.var_cnst1over_dn7 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn7)) * assign94760_body42_e147108) + (assign94760_body42_e147105 * (locals.var_exp_bps0_dn7 - locals.var_exp_bvbs_dn7))), ((((locals.var_cnst1over_dn8 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn8)) * assign94760_body42_e147108) + (assign94760_body42_e147105 * (locals.var_exp_bps0_dn8 - locals.var_exp_bvbs_dn8))), ((((locals.var_cnst1over_dn9 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn9)) * assign94760_body42_e147108) + (assign94760_body42_e147105 * (locals.var_exp_bps0_dn9 - locals.var_exp_bvbs_dn9))), ((((locals.var_cnst1over_dn10 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn10)) * assign94760_body42_e147108) + (assign94760_body42_e147105 * (locals.var_exp_bps0_dn10 - locals.var_exp_bvbs_dn10))), ((((locals.var_cnst1over_dn11 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn11)) * assign94760_body42_e147108) + (assign94760_body42_e147105 * (locals.var_exp_bps0_dn11 - locals.var_exp_bvbs_dn11))), ((((locals.var_cnst1over_dn14 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn14)) * assign94760_body42_e147108) + (assign94760_body42_e147105 * (locals.var_exp_bps0_dn14 - locals.var_exp_bvbs_dn14))),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn11, locals.var_fs01_dps0_dn14,)
    }
};
            locals.var_fs01_dps0 = assign94760_body42_e147111;
            locals.var_fs01_dps0_dn0 = assign94760_body42_e147111_d_n0;
            locals.var_fs01_dps0_dn2 = assign94760_body42_e147111_d_n2;
            locals.var_fs01_dps0_dn4 = assign94760_body42_e147111_d_n4;
            locals.var_fs01_dps0_dn5 = assign94760_body42_e147111_d_n5;
            locals.var_fs01_dps0_dn6 = assign94760_body42_e147111_d_n6;
            locals.var_fs01_dps0_dn7 = assign94760_body42_e147111_d_n7;
            locals.var_fs01_dps0_dn8 = assign94760_body42_e147111_d_n8;
            locals.var_fs01_dps0_dn9 = assign94760_body42_e147111_d_n9;
            locals.var_fs01_dps0_dn10 = assign94760_body42_e147111_d_n10;
            locals.var_fs01_dps0_dn11 = assign94760_body42_e147111_d_n11;
            locals.var_fs01_dps0_dn14 = assign94760_body42_e147111_d_n14;
            locals.var_fs01_dps0_rv = 0.0;
            let assign94760_body43_e147114: f64 = if locals.var_fs01 > 0.0 { 1.0 } else { 0.0 };
            locals.var_guard2209 = assign94760_body43_e147114;
            locals.var_guard2209_rv = 0.0;
            let (assign94760_body44_e147127, assign94760_body44_e147127_d_n0, assign94760_body44_e147127_d_n2, assign94760_body44_e147127_d_n4, assign94760_body44_e147127_d_n5, assign94760_body44_e147127_d_n6, assign94760_body44_e147127_d_n7, assign94760_body44_e147127_d_n8, assign94760_body44_e147127_d_n9, assign94760_body44_e147127_d_n10, assign94760_body44_e147127_d_n11, assign94760_body44_e147127_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2200 != 0.0)) && (locals.var_guard2209 != 0.0)) {
        let assign94760_body44_e147124: f64 = (locals.var_fbsq__blk2127 + locals.var_fs01);
        let assign94760_body44_e147125: f64 = (assign94760_body44_e147124).sqrt();
        (assign94760_body44_e147125, ((locals.var_fbsq__blk2127_dn0 + locals.var_fs01_dn0) / (2.0 * assign94760_body44_e147125)), ((locals.var_fbsq__blk2127_dn2 + locals.var_fs01_dn2) / (2.0 * assign94760_body44_e147125)), ((locals.var_fbsq__blk2127_dn4 + locals.var_fs01_dn4) / (2.0 * assign94760_body44_e147125)), ((locals.var_fbsq__blk2127_dn5 + locals.var_fs01_dn5) / (2.0 * assign94760_body44_e147125)), ((locals.var_fbsq__blk2127_dn6 + locals.var_fs01_dn6) / (2.0 * assign94760_body44_e147125)), ((locals.var_fbsq__blk2127_dn7 + locals.var_fs01_dn7) / (2.0 * assign94760_body44_e147125)), ((locals.var_fbsq__blk2127_dn8 + locals.var_fs01_dn8) / (2.0 * assign94760_body44_e147125)), ((locals.var_fbsq__blk2127_dn9 + locals.var_fs01_dn9) / (2.0 * assign94760_body44_e147125)), ((locals.var_fbsq__blk2127_dn10 + locals.var_fs01_dn10) / (2.0 * assign94760_body44_e147125)), ((locals.var_fbsq__blk2127_dn11 + locals.var_fs01_dn11) / (2.0 * assign94760_body44_e147125)), ((locals.var_fbsq__blk2127_dn14 + locals.var_fs01_dn14) / (2.0 * assign94760_body44_e147125)),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn11, locals.var_fs02_dn14,)
    }
};
            locals.var_fs02 = assign94760_body44_e147127;
            locals.var_fs02_dn0 = assign94760_body44_e147127_d_n0;
            locals.var_fs02_dn2 = assign94760_body44_e147127_d_n2;
            locals.var_fs02_dn4 = assign94760_body44_e147127_d_n4;
            locals.var_fs02_dn5 = assign94760_body44_e147127_d_n5;
            locals.var_fs02_dn6 = assign94760_body44_e147127_d_n6;
            locals.var_fs02_dn7 = assign94760_body44_e147127_d_n7;
            locals.var_fs02_dn8 = assign94760_body44_e147127_d_n8;
            locals.var_fs02_dn9 = assign94760_body44_e147127_d_n9;
            locals.var_fs02_dn10 = assign94760_body44_e147127_d_n10;
            locals.var_fs02_dn11 = assign94760_body44_e147127_d_n11;
            locals.var_fs02_dn14 = assign94760_body44_e147127_d_n14;
            locals.var_fs02_rv = 0.0;
            let (assign94760_body45_e147143, assign94760_body45_e147143_d_n0, assign94760_body45_e147143_d_n2, assign94760_body45_e147143_d_n4, assign94760_body45_e147143_d_n5, assign94760_body45_e147143_d_n6, assign94760_body45_e147143_d_n7, assign94760_body45_e147143_d_n8, assign94760_body45_e147143_d_n9, assign94760_body45_e147143_d_n10, assign94760_body45_e147143_d_n11, assign94760_body45_e147143_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2200 != 0.0)) && (locals.var_guard2209 != 0.0)) {
        let assign94760_body45_e147138: f64 = (locals.var_fbsq_dpss__blk2128 + locals.var_fs01_dps0);
        let assign94760_body45_e147139: f64 = (0.5 * assign94760_body45_e147138);
        let assign94760_body45_e147141: f64 = (assign94760_body45_e147139 / locals.var_fs02);
        (assign94760_body45_e147141, ((((0.5 * (locals.var_fbsq_dpss__blk2128_dn0 + locals.var_fs01_dps0_dn0)) * locals.var_fs02) - (assign94760_body45_e147139 * locals.var_fs02_dn0)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk2128_dn2 + locals.var_fs01_dps0_dn2)) * locals.var_fs02) - (assign94760_body45_e147139 * locals.var_fs02_dn2)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk2128_dn4 + locals.var_fs01_dps0_dn4)) * locals.var_fs02) - (assign94760_body45_e147139 * locals.var_fs02_dn4)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk2128_dn5 + locals.var_fs01_dps0_dn5)) * locals.var_fs02) - (assign94760_body45_e147139 * locals.var_fs02_dn5)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk2128_dn6 + locals.var_fs01_dps0_dn6)) * locals.var_fs02) - (assign94760_body45_e147139 * locals.var_fs02_dn6)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk2128_dn7 + locals.var_fs01_dps0_dn7)) * locals.var_fs02) - (assign94760_body45_e147139 * locals.var_fs02_dn7)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk2128_dn8 + locals.var_fs01_dps0_dn8)) * locals.var_fs02) - (assign94760_body45_e147139 * locals.var_fs02_dn8)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk2128_dn9 + locals.var_fs01_dps0_dn9)) * locals.var_fs02) - (assign94760_body45_e147139 * locals.var_fs02_dn9)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk2128_dn10 + locals.var_fs01_dps0_dn10)) * locals.var_fs02) - (assign94760_body45_e147139 * locals.var_fs02_dn10)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk2128_dn11 + locals.var_fs01_dps0_dn11)) * locals.var_fs02) - (assign94760_body45_e147139 * locals.var_fs02_dn11)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk2128_dn14 + locals.var_fs01_dps0_dn14)) * locals.var_fs02) - (assign94760_body45_e147139 * locals.var_fs02_dn14)) / (locals.var_fs02 * locals.var_fs02)),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn11, locals.var_fs02_dps0_dn14,)
    }
};
            locals.var_fs02_dps0 = assign94760_body45_e147143;
            locals.var_fs02_dps0_dn0 = assign94760_body45_e147143_d_n0;
            locals.var_fs02_dps0_dn2 = assign94760_body45_e147143_d_n2;
            locals.var_fs02_dps0_dn4 = assign94760_body45_e147143_d_n4;
            locals.var_fs02_dps0_dn5 = assign94760_body45_e147143_d_n5;
            locals.var_fs02_dps0_dn6 = assign94760_body45_e147143_d_n6;
            locals.var_fs02_dps0_dn7 = assign94760_body45_e147143_d_n7;
            locals.var_fs02_dps0_dn8 = assign94760_body45_e147143_d_n8;
            locals.var_fs02_dps0_dn9 = assign94760_body45_e147143_d_n9;
            locals.var_fs02_dps0_dn10 = assign94760_body45_e147143_d_n10;
            locals.var_fs02_dps0_dn11 = assign94760_body45_e147143_d_n11;
            locals.var_fs02_dps0_dn14 = assign94760_body45_e147143_d_n14;
            locals.var_fs02_dps0_rv = 0.0;
            let assign94760_body46_e147146: f64 = if locals.var_fbsq__blk2127 > 0.0 { 1.0 } else { 0.0 };
            locals.var_guard2210 = assign94760_body46_e147146;
            locals.var_guard2210_rv = 0.0;
            let (assign94760_body47_e147160, assign94760_body47_e147160_d_n0, assign94760_body47_e147160_d_n2, assign94760_body47_e147160_d_n4, assign94760_body47_e147160_d_n5, assign94760_body47_e147160_d_n6, assign94760_body47_e147160_d_n7, assign94760_body47_e147160_d_n8, assign94760_body47_e147160_d_n9, assign94760_body47_e147160_d_n10, assign94760_body47_e147160_d_n11, assign94760_body47_e147160_d_n14,) = {
    if (((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2200 != 0.0)) && (locals.var_guard2209 == 0.0)) && (locals.var_guard2210 != 0.0)) {
        let assign94760_body47_e147158: f64 = (locals.var_fbsq__blk2127).sqrt();
        (assign94760_body47_e147158, (locals.var_fbsq__blk2127_dn0 / (2.0 * assign94760_body47_e147158)), (locals.var_fbsq__blk2127_dn2 / (2.0 * assign94760_body47_e147158)), (locals.var_fbsq__blk2127_dn4 / (2.0 * assign94760_body47_e147158)), (locals.var_fbsq__blk2127_dn5 / (2.0 * assign94760_body47_e147158)), (locals.var_fbsq__blk2127_dn6 / (2.0 * assign94760_body47_e147158)), (locals.var_fbsq__blk2127_dn7 / (2.0 * assign94760_body47_e147158)), (locals.var_fbsq__blk2127_dn8 / (2.0 * assign94760_body47_e147158)), (locals.var_fbsq__blk2127_dn9 / (2.0 * assign94760_body47_e147158)), (locals.var_fbsq__blk2127_dn10 / (2.0 * assign94760_body47_e147158)), (locals.var_fbsq__blk2127_dn11 / (2.0 * assign94760_body47_e147158)), (locals.var_fbsq__blk2127_dn14 / (2.0 * assign94760_body47_e147158)),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn11, locals.var_fs02_dn14,)
    }
};
            locals.var_fs02 = assign94760_body47_e147160;
            locals.var_fs02_dn0 = assign94760_body47_e147160_d_n0;
            locals.var_fs02_dn2 = assign94760_body47_e147160_d_n2;
            locals.var_fs02_dn4 = assign94760_body47_e147160_d_n4;
            locals.var_fs02_dn5 = assign94760_body47_e147160_d_n5;
            locals.var_fs02_dn6 = assign94760_body47_e147160_d_n6;
            locals.var_fs02_dn7 = assign94760_body47_e147160_d_n7;
            locals.var_fs02_dn8 = assign94760_body47_e147160_d_n8;
            locals.var_fs02_dn9 = assign94760_body47_e147160_d_n9;
            locals.var_fs02_dn10 = assign94760_body47_e147160_d_n10;
            locals.var_fs02_dn11 = assign94760_body47_e147160_d_n11;
            locals.var_fs02_dn14 = assign94760_body47_e147160_d_n14;
            locals.var_fs02_rv = 0.0;
            let (assign94760_body48_e147177, assign94760_body48_e147177_d_n0, assign94760_body48_e147177_d_n2, assign94760_body48_e147177_d_n4, assign94760_body48_e147177_d_n5, assign94760_body48_e147177_d_n6, assign94760_body48_e147177_d_n7, assign94760_body48_e147177_d_n8, assign94760_body48_e147177_d_n9, assign94760_body48_e147177_d_n10, assign94760_body48_e147177_d_n11, assign94760_body48_e147177_d_n14,) = {
    if (((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2200 != 0.0)) && (locals.var_guard2209 == 0.0)) && (locals.var_guard2210 != 0.0)) {
        let assign94760_body48_e147173: f64 = (0.5 * locals.var_fbsq_dpss__blk2128);
        let assign94760_body48_e147175: f64 = (assign94760_body48_e147173 / locals.var_fs02);
        (assign94760_body48_e147175, ((((0.5 * locals.var_fbsq_dpss__blk2128_dn0) * locals.var_fs02) - (assign94760_body48_e147173 * locals.var_fs02_dn0)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk2128_dn2) * locals.var_fs02) - (assign94760_body48_e147173 * locals.var_fs02_dn2)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk2128_dn4) * locals.var_fs02) - (assign94760_body48_e147173 * locals.var_fs02_dn4)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk2128_dn5) * locals.var_fs02) - (assign94760_body48_e147173 * locals.var_fs02_dn5)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk2128_dn6) * locals.var_fs02) - (assign94760_body48_e147173 * locals.var_fs02_dn6)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk2128_dn7) * locals.var_fs02) - (assign94760_body48_e147173 * locals.var_fs02_dn7)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk2128_dn8) * locals.var_fs02) - (assign94760_body48_e147173 * locals.var_fs02_dn8)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk2128_dn9) * locals.var_fs02) - (assign94760_body48_e147173 * locals.var_fs02_dn9)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk2128_dn10) * locals.var_fs02) - (assign94760_body48_e147173 * locals.var_fs02_dn10)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk2128_dn11) * locals.var_fs02) - (assign94760_body48_e147173 * locals.var_fs02_dn11)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk2128_dn14) * locals.var_fs02) - (assign94760_body48_e147173 * locals.var_fs02_dn14)) / (locals.var_fs02 * locals.var_fs02)),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn11, locals.var_fs02_dps0_dn14,)
    }
};
            locals.var_fs02_dps0 = assign94760_body48_e147177;
            locals.var_fs02_dps0_dn0 = assign94760_body48_e147177_d_n0;
            locals.var_fs02_dps0_dn2 = assign94760_body48_e147177_d_n2;
            locals.var_fs02_dps0_dn4 = assign94760_body48_e147177_d_n4;
            locals.var_fs02_dps0_dn5 = assign94760_body48_e147177_d_n5;
            locals.var_fs02_dps0_dn6 = assign94760_body48_e147177_d_n6;
            locals.var_fs02_dps0_dn7 = assign94760_body48_e147177_d_n7;
            locals.var_fs02_dps0_dn8 = assign94760_body48_e147177_d_n8;
            locals.var_fs02_dps0_dn9 = assign94760_body48_e147177_d_n9;
            locals.var_fs02_dps0_dn10 = assign94760_body48_e147177_d_n10;
            locals.var_fs02_dps0_dn11 = assign94760_body48_e147177_d_n11;
            locals.var_fs02_dps0_dn14 = assign94760_body48_e147177_d_n14;
            locals.var_fs02_dps0_rv = 0.0;
            let (assign94760_body49_e147191, assign94760_body49_e147191_d_n0, assign94760_body49_e147191_d_n2, assign94760_body49_e147191_d_n4, assign94760_body49_e147191_d_n5, assign94760_body49_e147191_d_n6, assign94760_body49_e147191_d_n7, assign94760_body49_e147191_d_n8, assign94760_body49_e147191_d_n9, assign94760_body49_e147191_d_n10, assign94760_body49_e147191_d_n11, assign94760_body49_e147191_d_n14,) = {
    if (((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2200 != 0.0)) && (locals.var_guard2209 == 0.0)) && (locals.var_guard2210 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn11, locals.var_fs02_dn14,)
    }
};
            locals.var_fs02 = assign94760_body49_e147191;
            locals.var_fs02_dn0 = assign94760_body49_e147191_d_n0;
            locals.var_fs02_dn2 = assign94760_body49_e147191_d_n2;
            locals.var_fs02_dn4 = assign94760_body49_e147191_d_n4;
            locals.var_fs02_dn5 = assign94760_body49_e147191_d_n5;
            locals.var_fs02_dn6 = assign94760_body49_e147191_d_n6;
            locals.var_fs02_dn7 = assign94760_body49_e147191_d_n7;
            locals.var_fs02_dn8 = assign94760_body49_e147191_d_n8;
            locals.var_fs02_dn9 = assign94760_body49_e147191_d_n9;
            locals.var_fs02_dn10 = assign94760_body49_e147191_d_n10;
            locals.var_fs02_dn11 = assign94760_body49_e147191_d_n11;
            locals.var_fs02_dn14 = assign94760_body49_e147191_d_n14;
            locals.var_fs02_rv = 0.0;
            let (assign94760_body50_e147205, assign94760_body50_e147205_d_n0, assign94760_body50_e147205_d_n2, assign94760_body50_e147205_d_n4, assign94760_body50_e147205_d_n5, assign94760_body50_e147205_d_n6, assign94760_body50_e147205_d_n7, assign94760_body50_e147205_d_n8, assign94760_body50_e147205_d_n9, assign94760_body50_e147205_d_n10, assign94760_body50_e147205_d_n11, assign94760_body50_e147205_d_n14,) = {
    if (((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2200 != 0.0)) && (locals.var_guard2209 == 0.0)) && (locals.var_guard2210 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn11, locals.var_fs02_dps0_dn14,)
    }
};
            locals.var_fs02_dps0 = assign94760_body50_e147205;
            locals.var_fs02_dps0_dn0 = assign94760_body50_e147205_d_n0;
            locals.var_fs02_dps0_dn2 = assign94760_body50_e147205_d_n2;
            locals.var_fs02_dps0_dn4 = assign94760_body50_e147205_d_n4;
            locals.var_fs02_dps0_dn5 = assign94760_body50_e147205_d_n5;
            locals.var_fs02_dps0_dn6 = assign94760_body50_e147205_d_n6;
            locals.var_fs02_dps0_dn7 = assign94760_body50_e147205_d_n7;
            locals.var_fs02_dps0_dn8 = assign94760_body50_e147205_d_n8;
            locals.var_fs02_dps0_dn9 = assign94760_body50_e147205_d_n9;
            locals.var_fs02_dps0_dn10 = assign94760_body50_e147205_d_n10;
            locals.var_fs02_dps0_dn11 = assign94760_body50_e147205_d_n11;
            locals.var_fs02_dps0_dn14 = assign94760_body50_e147205_d_n14;
            locals.var_fs02_dps0_rv = 0.0;
            let (assign94760_body51_e147221, assign94760_body51_e147221_d_n0, assign94760_body51_e147221_d_n2, assign94760_body51_e147221_d_n4, assign94760_body51_e147221_d_n5, assign94760_body51_e147221_d_n6, assign94760_body51_e147221_d_n7, assign94760_body51_e147221_d_n8, assign94760_body51_e147221_d_n9, assign94760_body51_e147221_d_n10, assign94760_body51_e147221_d_n11, assign94760_body51_e147221_d_n14,) = {
    if (((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2200 != 0.0)) {
        let (assign94760_body51_e147217,) = {
            if (locals.var_chi >= 0.0) {
                (1.0,)
            } else {
                let assign94760_body51_e147216: f64 = (-1.0);
                (assign94760_body51_e147216,)
            }
        };
        let assign94760_body51_e147219: f64 = (assign94760_body51_e147217 * locals.var_fs02);
        (assign94760_body51_e147219, (assign94760_body51_e147217 * locals.var_fs02_dn0), (assign94760_body51_e147217 * locals.var_fs02_dn2), (assign94760_body51_e147217 * locals.var_fs02_dn4), (assign94760_body51_e147217 * locals.var_fs02_dn5), (assign94760_body51_e147217 * locals.var_fs02_dn6), (assign94760_body51_e147217 * locals.var_fs02_dn7), (assign94760_body51_e147217 * locals.var_fs02_dn8), (assign94760_body51_e147217 * locals.var_fs02_dn9), (assign94760_body51_e147217 * locals.var_fs02_dn10), (assign94760_body51_e147217 * locals.var_fs02_dn11), (assign94760_body51_e147217 * locals.var_fs02_dn14),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn11, locals.var_fs02_dn14,)
    }
};
            locals.var_fs02 = assign94760_body51_e147221;
            locals.var_fs02_dn0 = assign94760_body51_e147221_d_n0;
            locals.var_fs02_dn2 = assign94760_body51_e147221_d_n2;
            locals.var_fs02_dn4 = assign94760_body51_e147221_d_n4;
            locals.var_fs02_dn5 = assign94760_body51_e147221_d_n5;
            locals.var_fs02_dn6 = assign94760_body51_e147221_d_n6;
            locals.var_fs02_dn7 = assign94760_body51_e147221_d_n7;
            locals.var_fs02_dn8 = assign94760_body51_e147221_d_n8;
            locals.var_fs02_dn9 = assign94760_body51_e147221_d_n9;
            locals.var_fs02_dn10 = assign94760_body51_e147221_d_n10;
            locals.var_fs02_dn11 = assign94760_body51_e147221_d_n11;
            locals.var_fs02_dn14 = assign94760_body51_e147221_d_n14;
            locals.var_fs02_rv = 0.0;
            let (assign94760_body52_e147237, assign94760_body52_e147237_d_n0, assign94760_body52_e147237_d_n2, assign94760_body52_e147237_d_n4, assign94760_body52_e147237_d_n5, assign94760_body52_e147237_d_n6, assign94760_body52_e147237_d_n7, assign94760_body52_e147237_d_n8, assign94760_body52_e147237_d_n9, assign94760_body52_e147237_d_n10, assign94760_body52_e147237_d_n11, assign94760_body52_e147237_d_n14,) = {
    if (((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2200 != 0.0)) {
        let (assign94760_body52_e147233,) = {
            if (locals.var_chi >= 0.0) {
                (1.0,)
            } else {
                let assign94760_body52_e147232: f64 = (-1.0);
                (assign94760_body52_e147232,)
            }
        };
        let assign94760_body52_e147235: f64 = (assign94760_body52_e147233 * locals.var_fs02_dps0);
        (assign94760_body52_e147235, (assign94760_body52_e147233 * locals.var_fs02_dps0_dn0), (assign94760_body52_e147233 * locals.var_fs02_dps0_dn2), (assign94760_body52_e147233 * locals.var_fs02_dps0_dn4), (assign94760_body52_e147233 * locals.var_fs02_dps0_dn5), (assign94760_body52_e147233 * locals.var_fs02_dps0_dn6), (assign94760_body52_e147233 * locals.var_fs02_dps0_dn7), (assign94760_body52_e147233 * locals.var_fs02_dps0_dn8), (assign94760_body52_e147233 * locals.var_fs02_dps0_dn9), (assign94760_body52_e147233 * locals.var_fs02_dps0_dn10), (assign94760_body52_e147233 * locals.var_fs02_dps0_dn11), (assign94760_body52_e147233 * locals.var_fs02_dps0_dn14),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn11, locals.var_fs02_dps0_dn14,)
    }
};
            locals.var_fs02_dps0 = assign94760_body52_e147237;
            locals.var_fs02_dps0_dn0 = assign94760_body52_e147237_d_n0;
            locals.var_fs02_dps0_dn2 = assign94760_body52_e147237_d_n2;
            locals.var_fs02_dps0_dn4 = assign94760_body52_e147237_d_n4;
            locals.var_fs02_dps0_dn5 = assign94760_body52_e147237_d_n5;
            locals.var_fs02_dps0_dn6 = assign94760_body52_e147237_d_n6;
            locals.var_fs02_dps0_dn7 = assign94760_body52_e147237_d_n7;
            locals.var_fs02_dps0_dn8 = assign94760_body52_e147237_d_n8;
            locals.var_fs02_dps0_dn9 = assign94760_body52_e147237_d_n9;
            locals.var_fs02_dps0_dn10 = assign94760_body52_e147237_d_n10;
            locals.var_fs02_dps0_dn11 = assign94760_body52_e147237_d_n11;
            locals.var_fs02_dps0_dn14 = assign94760_body52_e147237_d_n14;
            locals.var_fs02_dps0_rv = 0.0;
            let (assign94760_body53_e147252, assign94760_body53_e147252_d_n0, assign94760_body53_e147252_d_n2, assign94760_body53_e147252_d_n4, assign94760_body53_e147252_d_n5, assign94760_body53_e147252_d_n6, assign94760_body53_e147252_d_n7, assign94760_body53_e147252_d_n8, assign94760_body53_e147252_d_n9, assign94760_body53_e147252_d_n10, assign94760_body53_e147252_d_n11, assign94760_body53_e147252_d_n14,) = {
    if (((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2200 != 0.0)) {
        let assign94760_body53_e147244: f64 = (-locals.var_vgpld);
        let assign94760_body53_e147246: f64 = (assign94760_body53_e147244 + locals.var_ps0ld);
        let assign94760_body53_e147249: f64 = (locals.var_fac1 * locals.var_fs02);
        let assign94760_body53_e147250: f64 = (assign94760_body53_e147246 + assign94760_body53_e147249);
        (assign94760_body53_e147250, (locals.var_ps0ld_dn0 + ((locals.var_fac1_dn0 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn0))), (((-locals.var_vgpld_dn2) + locals.var_ps0ld_dn2) + ((locals.var_fac1_dn2 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn2))), (locals.var_ps0ld_dn4 + ((locals.var_fac1_dn4 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn4))), (locals.var_ps0ld_dn5 + ((locals.var_fac1_dn5 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn5))), (locals.var_ps0ld_dn6 + ((locals.var_fac1_dn6 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn6))), (((-locals.var_vgpld_dn7) + locals.var_ps0ld_dn7) + ((locals.var_fac1_dn7 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn7))), (((-locals.var_vgpld_dn8) + locals.var_ps0ld_dn8) + ((locals.var_fac1_dn8 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn8))), (((-locals.var_vgpld_dn9) + locals.var_ps0ld_dn9) + ((locals.var_fac1_dn9 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn9))), (locals.var_ps0ld_dn10 + ((locals.var_fac1_dn10 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn10))), (locals.var_ps0ld_dn11 + ((locals.var_fac1_dn11 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn11))), (locals.var_ps0ld_dn14 + ((locals.var_fac1_dn14 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn14))),)
    } else {
        (locals.var_fs0, locals.var_fs0_dn0, locals.var_fs0_dn2, locals.var_fs0_dn4, locals.var_fs0_dn5, locals.var_fs0_dn6, locals.var_fs0_dn7, locals.var_fs0_dn8, locals.var_fs0_dn9, locals.var_fs0_dn10, locals.var_fs0_dn11, locals.var_fs0_dn14,)
    }
};
            locals.var_fs0 = assign94760_body53_e147252;
            locals.var_fs0_dn0 = assign94760_body53_e147252_d_n0;
            locals.var_fs0_dn2 = assign94760_body53_e147252_d_n2;
            locals.var_fs0_dn4 = assign94760_body53_e147252_d_n4;
            locals.var_fs0_dn5 = assign94760_body53_e147252_d_n5;
            locals.var_fs0_dn6 = assign94760_body53_e147252_d_n6;
            locals.var_fs0_dn7 = assign94760_body53_e147252_d_n7;
            locals.var_fs0_dn8 = assign94760_body53_e147252_d_n8;
            locals.var_fs0_dn9 = assign94760_body53_e147252_d_n9;
            locals.var_fs0_dn10 = assign94760_body53_e147252_d_n10;
            locals.var_fs0_dn11 = assign94760_body53_e147252_d_n11;
            locals.var_fs0_dn14 = assign94760_body53_e147252_d_n14;
            locals.var_fs0_rv = 0.0;
            let (assign94760_body54_e147264, assign94760_body54_e147264_d_n0, assign94760_body54_e147264_d_n2, assign94760_body54_e147264_d_n4, assign94760_body54_e147264_d_n5, assign94760_body54_e147264_d_n6, assign94760_body54_e147264_d_n7, assign94760_body54_e147264_d_n8, assign94760_body54_e147264_d_n9, assign94760_body54_e147264_d_n10, assign94760_body54_e147264_d_n11, assign94760_body54_e147264_d_n14,) = {
    if (((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2200 != 0.0)) {
        let assign94760_body54_e147261: f64 = (locals.var_fac1 * locals.var_fs02_dps0);
        let assign94760_body54_e147262: f64 = (1.0 + assign94760_body54_e147261);
        (assign94760_body54_e147262, ((locals.var_fac1_dn0 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn0)), ((locals.var_fac1_dn2 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn2)), ((locals.var_fac1_dn4 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn4)), ((locals.var_fac1_dn5 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn5)), ((locals.var_fac1_dn6 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn6)), ((locals.var_fac1_dn7 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn7)), ((locals.var_fac1_dn8 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn8)), ((locals.var_fac1_dn9 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn9)), ((locals.var_fac1_dn10 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn10)), ((locals.var_fac1_dn11 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn11)), ((locals.var_fac1_dn14 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn14)),)
    } else {
        (locals.var_fs0_dps0, locals.var_fs0_dps0_dn0, locals.var_fs0_dps0_dn2, locals.var_fs0_dps0_dn4, locals.var_fs0_dps0_dn5, locals.var_fs0_dps0_dn6, locals.var_fs0_dps0_dn7, locals.var_fs0_dps0_dn8, locals.var_fs0_dps0_dn9, locals.var_fs0_dps0_dn10, locals.var_fs0_dps0_dn11, locals.var_fs0_dps0_dn14,)
    }
};
            locals.var_fs0_dps0 = assign94760_body54_e147264;
            locals.var_fs0_dps0_dn0 = assign94760_body54_e147264_d_n0;
            locals.var_fs0_dps0_dn2 = assign94760_body54_e147264_d_n2;
            locals.var_fs0_dps0_dn4 = assign94760_body54_e147264_d_n4;
            locals.var_fs0_dps0_dn5 = assign94760_body54_e147264_d_n5;
            locals.var_fs0_dps0_dn6 = assign94760_body54_e147264_d_n6;
            locals.var_fs0_dps0_dn7 = assign94760_body54_e147264_d_n7;
            locals.var_fs0_dps0_dn8 = assign94760_body54_e147264_d_n8;
            locals.var_fs0_dps0_dn9 = assign94760_body54_e147264_d_n9;
            locals.var_fs0_dps0_dn10 = assign94760_body54_e147264_d_n10;
            locals.var_fs0_dps0_dn11 = assign94760_body54_e147264_d_n11;
            locals.var_fs0_dps0_dn14 = assign94760_body54_e147264_d_n14;
            locals.var_fs0_dps0_rv = 0.0;
            let assign94760_body55_e147267: f64 = if locals.var_flg_conv > 0.0 { 1.0 } else { 0.0 };
            locals.var_guard2211 = assign94760_body55_e147267;
            locals.var_guard2211_rv = 0.0;
            let (assign94760_body56_e147279,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2200 != 0.0)) && (locals.var_guard2211 != 0.0)) {
        let assign94760_body56_e147277: f64 = (locals.var_lp_s0_max + 1.0);
        (assign94760_body56_e147277,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign94760_body56_e147279;
            locals.var_lp_s0_rv = 0.0;
            let (assign94760_body57_e147293, assign94760_body57_e147293_d_n0, assign94760_body57_e147293_d_n2, assign94760_body57_e147293_d_n4, assign94760_body57_e147293_d_n5, assign94760_body57_e147293_d_n6, assign94760_body57_e147293_d_n7, assign94760_body57_e147293_d_n8, assign94760_body57_e147293_d_n9, assign94760_body57_e147293_d_n10, assign94760_body57_e147293_d_n11, assign94760_body57_e147293_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2200 != 0.0)) && (locals.var_guard2211 == 0.0)) {
        let assign94760_body57_e147289: f64 = (-locals.var_fs0);
        let assign94760_body57_e147291: f64 = (assign94760_body57_e147289 / locals.var_fs0_dps0);
        (assign94760_body57_e147291, ((((-locals.var_fs0_dn0) * locals.var_fs0_dps0) - (assign94760_body57_e147289 * locals.var_fs0_dps0_dn0)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn2) * locals.var_fs0_dps0) - (assign94760_body57_e147289 * locals.var_fs0_dps0_dn2)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn4) * locals.var_fs0_dps0) - (assign94760_body57_e147289 * locals.var_fs0_dps0_dn4)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn5) * locals.var_fs0_dps0) - (assign94760_body57_e147289 * locals.var_fs0_dps0_dn5)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn6) * locals.var_fs0_dps0) - (assign94760_body57_e147289 * locals.var_fs0_dps0_dn6)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn7) * locals.var_fs0_dps0) - (assign94760_body57_e147289 * locals.var_fs0_dps0_dn7)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn8) * locals.var_fs0_dps0) - (assign94760_body57_e147289 * locals.var_fs0_dps0_dn8)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn9) * locals.var_fs0_dps0) - (assign94760_body57_e147289 * locals.var_fs0_dps0_dn9)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn10) * locals.var_fs0_dps0) - (assign94760_body57_e147289 * locals.var_fs0_dps0_dn10)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn11) * locals.var_fs0_dps0) - (assign94760_body57_e147289 * locals.var_fs0_dps0_dn11)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn14) * locals.var_fs0_dps0) - (assign94760_body57_e147289 * locals.var_fs0_dps0_dn14)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn4, locals.var_dps0_dn5, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn8, locals.var_dps0_dn9, locals.var_dps0_dn10, locals.var_dps0_dn11, locals.var_dps0_dn14,)
    }
};
            locals.var_dps0 = assign94760_body57_e147293;
            locals.var_dps0_dn0 = assign94760_body57_e147293_d_n0;
            locals.var_dps0_dn2 = assign94760_body57_e147293_d_n2;
            locals.var_dps0_dn4 = assign94760_body57_e147293_d_n4;
            locals.var_dps0_dn5 = assign94760_body57_e147293_d_n5;
            locals.var_dps0_dn6 = assign94760_body57_e147293_d_n6;
            locals.var_dps0_dn7 = assign94760_body57_e147293_d_n7;
            locals.var_dps0_dn8 = assign94760_body57_e147293_d_n8;
            locals.var_dps0_dn9 = assign94760_body57_e147293_d_n9;
            locals.var_dps0_dn10 = assign94760_body57_e147293_d_n10;
            locals.var_dps0_dn11 = assign94760_body57_e147293_d_n11;
            locals.var_dps0_dn14 = assign94760_body57_e147293_d_n14;
            locals.var_dps0_rv = 0.0;
            let (assign94760_body58_e147317, assign94760_body58_e147317_d_n0, assign94760_body58_e147317_d_n2, assign94760_body58_e147317_d_n4, assign94760_body58_e147317_d_n5, assign94760_body58_e147317_d_n6, assign94760_body58_e147317_d_n7, assign94760_body58_e147317_d_n8, assign94760_body58_e147317_d_n9, assign94760_body58_e147317_d_n10, assign94760_body58_e147317_d_n11, assign94760_body58_e147317_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2200 != 0.0)) && (locals.var_guard2211 == 0.0)) {
        let assign94760_body58_e147304: f64 = (0.5 * 0.1);
        let assign94760_body58_e147308: f64 = (locals.var_ps0ld).abs();
        let (assign94760_body58_e147313, assign94760_body58_e147313_d_n0, assign94760_body58_e147313_d_n2, assign94760_body58_e147313_d_n4, assign94760_body58_e147313_d_n5, assign94760_body58_e147313_d_n6, assign94760_body58_e147313_d_n7, assign94760_body58_e147313_d_n8, assign94760_body58_e147313_d_n9, assign94760_body58_e147313_d_n10, assign94760_body58_e147313_d_n11, assign94760_body58_e147313_d_n14,) = {
            if (1.0 >= assign94760_body58_e147308) {
                (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign94760_body58_e147312: f64 = (locals.var_ps0ld).abs();
                (assign94760_body58_e147312, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn0 } else { (-locals.var_ps0ld_dn0) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn2 } else { (-locals.var_ps0ld_dn2) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn4 } else { (-locals.var_ps0ld_dn4) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn5 } else { (-locals.var_ps0ld_dn5) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn6 } else { (-locals.var_ps0ld_dn6) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn7 } else { (-locals.var_ps0ld_dn7) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn8 } else { (-locals.var_ps0ld_dn8) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn9 } else { (-locals.var_ps0ld_dn9) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn10 } else { (-locals.var_ps0ld_dn10) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn11 } else { (-locals.var_ps0ld_dn11) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn14 } else { (-locals.var_ps0ld_dn14) },)
            }
        };
        let assign94760_body58_e147314: f64 = (1.0 + assign94760_body58_e147313);
        let assign94760_body58_e147315: f64 = (assign94760_body58_e147304 * assign94760_body58_e147314);
        (assign94760_body58_e147315, (assign94760_body58_e147304 * assign94760_body58_e147313_d_n0), (assign94760_body58_e147304 * assign94760_body58_e147313_d_n2), (assign94760_body58_e147304 * assign94760_body58_e147313_d_n4), (assign94760_body58_e147304 * assign94760_body58_e147313_d_n5), (assign94760_body58_e147304 * assign94760_body58_e147313_d_n6), (assign94760_body58_e147304 * assign94760_body58_e147313_d_n7), (assign94760_body58_e147304 * assign94760_body58_e147313_d_n8), (assign94760_body58_e147304 * assign94760_body58_e147313_d_n9), (assign94760_body58_e147304 * assign94760_body58_e147313_d_n10), (assign94760_body58_e147304 * assign94760_body58_e147313_d_n11), (assign94760_body58_e147304 * assign94760_body58_e147313_d_n14),)
    } else {
        (locals.var_dplim, locals.var_dplim_dn0, locals.var_dplim_dn2, locals.var_dplim_dn4, locals.var_dplim_dn5, locals.var_dplim_dn6, locals.var_dplim_dn7, locals.var_dplim_dn8, locals.var_dplim_dn9, locals.var_dplim_dn10, locals.var_dplim_dn11, locals.var_dplim_dn14,)
    }
};
            locals.var_dplim = assign94760_body58_e147317;
            locals.var_dplim_dn0 = assign94760_body58_e147317_d_n0;
            locals.var_dplim_dn2 = assign94760_body58_e147317_d_n2;
            locals.var_dplim_dn4 = assign94760_body58_e147317_d_n4;
            locals.var_dplim_dn5 = assign94760_body58_e147317_d_n5;
            locals.var_dplim_dn6 = assign94760_body58_e147317_d_n6;
            locals.var_dplim_dn7 = assign94760_body58_e147317_d_n7;
            locals.var_dplim_dn8 = assign94760_body58_e147317_d_n8;
            locals.var_dplim_dn9 = assign94760_body58_e147317_d_n9;
            locals.var_dplim_dn10 = assign94760_body58_e147317_d_n10;
            locals.var_dplim_dn11 = assign94760_body58_e147317_d_n11;
            locals.var_dplim_dn14 = assign94760_body58_e147317_d_n14;
            locals.var_dplim_rv = 0.0;
            let assign94760_body59_e147319: f64 = (locals.var_dps0).abs();
            let assign94760_body59_e147321: f64 = if assign94760_body59_e147319 > locals.var_dplim { 1.0 } else { 0.0 };
            locals.var_guard2212 = assign94760_body59_e147321;
            locals.var_guard2212_rv = 0.0;
            let (assign94760_body60_e147342, assign94760_body60_e147342_d_n0, assign94760_body60_e147342_d_n2, assign94760_body60_e147342_d_n4, assign94760_body60_e147342_d_n5, assign94760_body60_e147342_d_n6, assign94760_body60_e147342_d_n7, assign94760_body60_e147342_d_n8, assign94760_body60_e147342_d_n9, assign94760_body60_e147342_d_n10, assign94760_body60_e147342_d_n11, assign94760_body60_e147342_d_n14,) = {
    if (((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2200 != 0.0)) && (locals.var_guard2211 == 0.0)) && (locals.var_guard2212 != 0.0)) {
        let (assign94760_body60_e147339,) = {
            if (locals.var_dps0 >= 0.0) {
                (1.0,)
            } else {
                let assign94760_body60_e147338: f64 = (-1.0);
                (assign94760_body60_e147338,)
            }
        };
        let assign94760_body60_e147340: f64 = (locals.var_dplim * assign94760_body60_e147339);
        (assign94760_body60_e147340, (locals.var_dplim_dn0 * assign94760_body60_e147339), (locals.var_dplim_dn2 * assign94760_body60_e147339), (locals.var_dplim_dn4 * assign94760_body60_e147339), (locals.var_dplim_dn5 * assign94760_body60_e147339), (locals.var_dplim_dn6 * assign94760_body60_e147339), (locals.var_dplim_dn7 * assign94760_body60_e147339), (locals.var_dplim_dn8 * assign94760_body60_e147339), (locals.var_dplim_dn9 * assign94760_body60_e147339), (locals.var_dplim_dn10 * assign94760_body60_e147339), (locals.var_dplim_dn11 * assign94760_body60_e147339), (locals.var_dplim_dn14 * assign94760_body60_e147339),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn4, locals.var_dps0_dn5, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn8, locals.var_dps0_dn9, locals.var_dps0_dn10, locals.var_dps0_dn11, locals.var_dps0_dn14,)
    }
};
            locals.var_dps0 = assign94760_body60_e147342;
            locals.var_dps0_dn0 = assign94760_body60_e147342_d_n0;
            locals.var_dps0_dn2 = assign94760_body60_e147342_d_n2;
            locals.var_dps0_dn4 = assign94760_body60_e147342_d_n4;
            locals.var_dps0_dn5 = assign94760_body60_e147342_d_n5;
            locals.var_dps0_dn6 = assign94760_body60_e147342_d_n6;
            locals.var_dps0_dn7 = assign94760_body60_e147342_d_n7;
            locals.var_dps0_dn8 = assign94760_body60_e147342_d_n8;
            locals.var_dps0_dn9 = assign94760_body60_e147342_d_n9;
            locals.var_dps0_dn10 = assign94760_body60_e147342_d_n10;
            locals.var_dps0_dn11 = assign94760_body60_e147342_d_n11;
            locals.var_dps0_dn14 = assign94760_body60_e147342_d_n14;
            locals.var_dps0_rv = 0.0;
            let (assign94760_body61_e147355, assign94760_body61_e147355_d_n0, assign94760_body61_e147355_d_n2, assign94760_body61_e147355_d_n4, assign94760_body61_e147355_d_n5, assign94760_body61_e147355_d_n6, assign94760_body61_e147355_d_n7, assign94760_body61_e147355_d_n8, assign94760_body61_e147355_d_n9, assign94760_body61_e147355_d_n10, assign94760_body61_e147355_d_n11, assign94760_body61_e147355_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2200 != 0.0)) && (locals.var_guard2211 == 0.0)) {
        let assign94760_body61_e147353: f64 = (locals.var_ps0ld + locals.var_dps0);
        (assign94760_body61_e147353, (locals.var_ps0ld_dn0 + locals.var_dps0_dn0), (locals.var_ps0ld_dn2 + locals.var_dps0_dn2), (locals.var_ps0ld_dn4 + locals.var_dps0_dn4), (locals.var_ps0ld_dn5 + locals.var_dps0_dn5), (locals.var_ps0ld_dn6 + locals.var_dps0_dn6), (locals.var_ps0ld_dn7 + locals.var_dps0_dn7), (locals.var_ps0ld_dn8 + locals.var_dps0_dn8), (locals.var_ps0ld_dn9 + locals.var_dps0_dn9), (locals.var_ps0ld_dn10 + locals.var_dps0_dn10), (locals.var_ps0ld_dn11 + locals.var_dps0_dn11), (locals.var_ps0ld_dn14 + locals.var_dps0_dn14),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn14,)
    }
};
            locals.var_ps0ld = assign94760_body61_e147355;
            locals.var_ps0ld_dn0 = assign94760_body61_e147355_d_n0;
            locals.var_ps0ld_dn2 = assign94760_body61_e147355_d_n2;
            locals.var_ps0ld_dn4 = assign94760_body61_e147355_d_n4;
            locals.var_ps0ld_dn5 = assign94760_body61_e147355_d_n5;
            locals.var_ps0ld_dn6 = assign94760_body61_e147355_d_n6;
            locals.var_ps0ld_dn7 = assign94760_body61_e147355_d_n7;
            locals.var_ps0ld_dn8 = assign94760_body61_e147355_d_n8;
            locals.var_ps0ld_dn9 = assign94760_body61_e147355_d_n9;
            locals.var_ps0ld_dn10 = assign94760_body61_e147355_d_n10;
            locals.var_ps0ld_dn11 = assign94760_body61_e147355_d_n11;
            locals.var_ps0ld_dn14 = assign94760_body61_e147355_d_n14;
            locals.var_ps0ld_rv = 0.0;
            let assign94760_body62_e147357: f64 = (locals.var_dps0).abs();
            let assign94760_body62_e147361: f64 = (locals.var_fs0).abs();
            let assign94760_body62_e147364: f64 = if ((assign94760_body62_e147357 <= 1e-12) && (assign94760_body62_e147361 <= 1e-8)) { 1.0 } else { 0.0 };
            locals.var_guard2213 = assign94760_body62_e147364;
            locals.var_guard2213_rv = 0.0;
            let (assign94760_body63_e147379,) = {
    if (((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2200 != 0.0)) && (locals.var_guard2211 == 0.0)) && (locals.var_guard2213 != 0.0)) {
        let assign94760_body63_e147377: f64 = (locals.var_flg_conv + 2.0);
        (assign94760_body63_e147377,)
    } else {
        (locals.var_flg_conv,)
    }
};
            locals.var_flg_conv = assign94760_body63_e147379;
            locals.var_flg_conv_rv = 0.0;
            let (assign94760_body64_e147389,) = {
    if (((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2200 != 0.0)) {
        let assign94760_body64_e147387: f64 = (locals.var_lp_s0 + 1.0);
        (assign94760_body64_e147387,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign94760_body64_e147389;
            locals.var_lp_s0_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_368(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign94780_e147414, assign94780_e147414_d_n0, assign94780_e147414_d_n2, assign94780_e147414_d_n4, assign94780_e147414_d_n5, assign94780_e147414_d_n6, assign94780_e147414_d_n7, assign94780_e147414_d_n8, assign94780_e147414_d_n9, assign94780_e147414_d_n10, assign94780_e147414_d_n11, assign94780_e147414_d_n14,) = {
    if (((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2200 != 0.0)) {
        let (assign94780_e147412, assign94780_e147412_d_n0, assign94780_e147412_d_n2, assign94780_e147412_d_n4, assign94780_e147412_d_n5, assign94780_e147412_d_n6, assign94780_e147412_d_n7, assign94780_e147412_d_n8, assign94780_e147412_d_n9, assign94780_e147412_d_n10, assign94780_e147412_d_n11, assign94780_e147412_d_n14,) = {
            if (locals.var_fbsq__blk2127 >= 0.0) {
                let (assign94780_e147407,) = {
                    if (locals.var_chi >= 0.0) {
                        (1.0,)
                    } else {
                        let assign94780_e147406: f64 = (-1.0);
                        (assign94780_e147406,)
                    }
                };
                let assign94780_e147409: f64 = (locals.var_fbsq__blk2127).sqrt();
                let assign94780_e147410: f64 = (assign94780_e147407 * assign94780_e147409);
                (assign94780_e147410, (assign94780_e147407 * (locals.var_fbsq__blk2127_dn0 / (2.0 * assign94780_e147409))), (assign94780_e147407 * (locals.var_fbsq__blk2127_dn2 / (2.0 * assign94780_e147409))), (assign94780_e147407 * (locals.var_fbsq__blk2127_dn4 / (2.0 * assign94780_e147409))), (assign94780_e147407 * (locals.var_fbsq__blk2127_dn5 / (2.0 * assign94780_e147409))), (assign94780_e147407 * (locals.var_fbsq__blk2127_dn6 / (2.0 * assign94780_e147409))), (assign94780_e147407 * (locals.var_fbsq__blk2127_dn7 / (2.0 * assign94780_e147409))), (assign94780_e147407 * (locals.var_fbsq__blk2127_dn8 / (2.0 * assign94780_e147409))), (assign94780_e147407 * (locals.var_fbsq__blk2127_dn9 / (2.0 * assign94780_e147409))), (assign94780_e147407 * (locals.var_fbsq__blk2127_dn10 / (2.0 * assign94780_e147409))), (assign94780_e147407 * (locals.var_fbsq__blk2127_dn11 / (2.0 * assign94780_e147409))), (assign94780_e147407 * (locals.var_fbsq__blk2127_dn14 / (2.0 * assign94780_e147409))),)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign94780_e147412, assign94780_e147412_d_n0, assign94780_e147412_d_n2, assign94780_e147412_d_n4, assign94780_e147412_d_n5, assign94780_e147412_d_n6, assign94780_e147412_d_n7, assign94780_e147412_d_n8, assign94780_e147412_d_n9, assign94780_e147412_d_n10, assign94780_e147412_d_n11, assign94780_e147412_d_n14,)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn14,)
    }
};
        locals.var_fb = assign94780_e147414;
        locals.var_fb_dn0 = assign94780_e147414_d_n0;
        locals.var_fb_dn2 = assign94780_e147414_d_n2;
        locals.var_fb_dn4 = assign94780_e147414_d_n4;
        locals.var_fb_dn5 = assign94780_e147414_d_n5;
        locals.var_fb_dn6 = assign94780_e147414_d_n6;
        locals.var_fb_dn7 = assign94780_e147414_d_n7;
        locals.var_fb_dn8 = assign94780_e147414_d_n8;
        locals.var_fb_dn9 = assign94780_e147414_d_n9;
        locals.var_fb_dn10 = assign94780_e147414_d_n10;
        locals.var_fb_dn11 = assign94780_e147414_d_n11;
        locals.var_fb_dn14 = assign94780_e147414_d_n14;
        locals.var_fb_rv = 0.0;

        let (assign94790_e147424, assign94790_e147424_d_n0, assign94790_e147424_d_n2, assign94790_e147424_d_n4, assign94790_e147424_d_n5, assign94790_e147424_d_n6, assign94790_e147424_d_n7, assign94790_e147424_d_n8, assign94790_e147424_d_n9, assign94790_e147424_d_n10, assign94790_e147424_d_n11, assign94790_e147424_d_n14,) = {
    if (((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2200 != 0.0)) {
        let assign94790_e147422: f64 = (locals.var_c_w_ld * locals.var_fb);
        (assign94790_e147422, ((locals.var_c_w_ld_dn0 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn0)), ((locals.var_c_w_ld_dn2 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn2)), ((locals.var_c_w_ld_dn4 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn4)), ((locals.var_c_w_ld_dn5 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn5)), ((locals.var_c_w_ld_dn6 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn6)), ((locals.var_c_w_ld_dn7 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn7)), ((locals.var_c_w_ld_dn8 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn8)), ((locals.var_c_w_ld_dn9 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn9)), ((locals.var_c_w_ld_dn10 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn10)), ((locals.var_c_w_ld_dn11 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn11)), ((locals.var_c_w_ld_dn14 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn14)),)
    } else {
        (locals.var_wdld__blk2117, locals.var_wdld__blk2117_dn0, locals.var_wdld__blk2117_dn2, locals.var_wdld__blk2117_dn4, locals.var_wdld__blk2117_dn5, locals.var_wdld__blk2117_dn6, locals.var_wdld__blk2117_dn7, locals.var_wdld__blk2117_dn8, locals.var_wdld__blk2117_dn9, locals.var_wdld__blk2117_dn10, locals.var_wdld__blk2117_dn11, locals.var_wdld__blk2117_dn14,)
    }
};
        locals.var_wdld__blk2117 = assign94790_e147424;
        locals.var_wdld__blk2117_dn0 = assign94790_e147424_d_n0;
        locals.var_wdld__blk2117_dn2 = assign94790_e147424_d_n2;
        locals.var_wdld__blk2117_dn4 = assign94790_e147424_d_n4;
        locals.var_wdld__blk2117_dn5 = assign94790_e147424_d_n5;
        locals.var_wdld__blk2117_dn6 = assign94790_e147424_d_n6;
        locals.var_wdld__blk2117_dn7 = assign94790_e147424_d_n7;
        locals.var_wdld__blk2117_dn8 = assign94790_e147424_d_n8;
        locals.var_wdld__blk2117_dn9 = assign94790_e147424_d_n9;
        locals.var_wdld__blk2117_dn10 = assign94790_e147424_d_n10;
        locals.var_wdld__blk2117_dn11 = assign94790_e147424_d_n11;
        locals.var_wdld__blk2117_dn14 = assign94790_e147424_d_n14;
        locals.var_wdld__blk2117_rv = 0.0;

        let (assign94800_e147434, assign94800_e147434_d_n0, assign94800_e147434_d_n2, assign94800_e147434_d_n4, assign94800_e147434_d_n5, assign94800_e147434_d_n6, assign94800_e147434_d_n7, assign94800_e147434_d_n8, assign94800_e147434_d_n9, assign94800_e147434_d_n10, assign94800_e147434_d_n11, assign94800_e147434_d_n14,) = {
    if (((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2200 != 0.0)) {
        let assign94800_e147432: f64 = (locals.var_q_nsubld__blk2119 * locals.var_wdld__blk2117);
        (assign94800_e147432, (locals.var_q_nsubld__blk2119 * locals.var_wdld__blk2117_dn0), (locals.var_q_nsubld__blk2119 * locals.var_wdld__blk2117_dn2), (locals.var_q_nsubld__blk2119 * locals.var_wdld__blk2117_dn4), (locals.var_q_nsubld__blk2119 * locals.var_wdld__blk2117_dn5), (locals.var_q_nsubld__blk2119 * locals.var_wdld__blk2117_dn6), (locals.var_q_nsubld__blk2119 * locals.var_wdld__blk2117_dn7), (locals.var_q_nsubld__blk2119 * locals.var_wdld__blk2117_dn8), (locals.var_q_nsubld__blk2119 * locals.var_wdld__blk2117_dn9), (locals.var_q_nsubld__blk2119 * locals.var_wdld__blk2117_dn10), (locals.var_q_nsubld__blk2119 * locals.var_wdld__blk2117_dn11), (locals.var_q_nsubld__blk2119 * locals.var_wdld__blk2117_dn14),)
    } else {
        (locals.var_q_dep_ld__blk2118, locals.var_q_dep_ld__blk2118_dn0, locals.var_q_dep_ld__blk2118_dn2, locals.var_q_dep_ld__blk2118_dn4, locals.var_q_dep_ld__blk2118_dn5, locals.var_q_dep_ld__blk2118_dn6, locals.var_q_dep_ld__blk2118_dn7, locals.var_q_dep_ld__blk2118_dn8, locals.var_q_dep_ld__blk2118_dn9, locals.var_q_dep_ld__blk2118_dn10, locals.var_q_dep_ld__blk2118_dn11, locals.var_q_dep_ld__blk2118_dn14,)
    }
};
        locals.var_q_dep_ld__blk2118 = assign94800_e147434;
        locals.var_q_dep_ld__blk2118_dn0 = assign94800_e147434_d_n0;
        locals.var_q_dep_ld__blk2118_dn2 = assign94800_e147434_d_n2;
        locals.var_q_dep_ld__blk2118_dn4 = assign94800_e147434_d_n4;
        locals.var_q_dep_ld__blk2118_dn5 = assign94800_e147434_d_n5;
        locals.var_q_dep_ld__blk2118_dn6 = assign94800_e147434_d_n6;
        locals.var_q_dep_ld__blk2118_dn7 = assign94800_e147434_d_n7;
        locals.var_q_dep_ld__blk2118_dn8 = assign94800_e147434_d_n8;
        locals.var_q_dep_ld__blk2118_dn9 = assign94800_e147434_d_n9;
        locals.var_q_dep_ld__blk2118_dn10 = assign94800_e147434_d_n10;
        locals.var_q_dep_ld__blk2118_dn11 = assign94800_e147434_d_n11;
        locals.var_q_dep_ld__blk2118_dn14 = assign94800_e147434_d_n14;
        locals.var_q_dep_ld__blk2118_rv = 0.0;

        let (assign94810_e147448, assign94810_e147448_d_n0, assign94810_e147448_d_n2, assign94810_e147448_d_n4, assign94810_e147448_d_n5, assign94810_e147448_d_n6, assign94810_e147448_d_n7, assign94810_e147448_d_n8, assign94810_e147448_d_n9, assign94810_e147448_d_n10, assign94810_e147448_d_n11, assign94810_e147448_d_n14,) = {
    if (((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2200 != 0.0)) {
        let assign94810_e147442: f64 = (locals.var_q_dep_ld__blk2118 / locals.var_cnst0over_func);
        let assign94810_e147445: f64 = (10.0 * 2.220446049250313e-16);
        let assign94810_e147446: f64 = (assign94810_e147442 + assign94810_e147445);
        (assign94810_e147446, (((locals.var_q_dep_ld__blk2118_dn0 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2118 * locals.var_cnst0over_func_dn0)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2118_dn2 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2118 * locals.var_cnst0over_func_dn2)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2118_dn4 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2118 * locals.var_cnst0over_func_dn4)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2118_dn5 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2118 * locals.var_cnst0over_func_dn5)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2118_dn6 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2118 * locals.var_cnst0over_func_dn6)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2118_dn7 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2118 * locals.var_cnst0over_func_dn7)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2118_dn8 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2118 * locals.var_cnst0over_func_dn8)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2118_dn9 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2118 * locals.var_cnst0over_func_dn9)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2118_dn10 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2118 * locals.var_cnst0over_func_dn10)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2118_dn11 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2118 * locals.var_cnst0over_func_dn11)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2118_dn14 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2118 * locals.var_cnst0over_func_dn14)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)),)
    } else {
        (locals.var_xi0p12, locals.var_xi0p12_dn0, locals.var_xi0p12_dn2, locals.var_xi0p12_dn4, locals.var_xi0p12_dn5, locals.var_xi0p12_dn6, locals.var_xi0p12_dn7, locals.var_xi0p12_dn8, locals.var_xi0p12_dn9, locals.var_xi0p12_dn10, locals.var_xi0p12_dn11, locals.var_xi0p12_dn14,)
    }
};
        locals.var_xi0p12 = assign94810_e147448;
        locals.var_xi0p12_dn0 = assign94810_e147448_d_n0;
        locals.var_xi0p12_dn2 = assign94810_e147448_d_n2;
        locals.var_xi0p12_dn4 = assign94810_e147448_d_n4;
        locals.var_xi0p12_dn5 = assign94810_e147448_d_n5;
        locals.var_xi0p12_dn6 = assign94810_e147448_d_n6;
        locals.var_xi0p12_dn7 = assign94810_e147448_d_n7;
        locals.var_xi0p12_dn8 = assign94810_e147448_d_n8;
        locals.var_xi0p12_dn9 = assign94810_e147448_d_n9;
        locals.var_xi0p12_dn10 = assign94810_e147448_d_n10;
        locals.var_xi0p12_dn11 = assign94810_e147448_d_n11;
        locals.var_xi0p12_dn14 = assign94810_e147448_d_n14;
        locals.var_xi0p12_rv = 0.0;

        let (assign94820_e147458, assign94820_e147458_d_n0, assign94820_e147458_d_n2, assign94820_e147458_d_n4, assign94820_e147458_d_n5, assign94820_e147458_d_n6, assign94820_e147458_d_n7, assign94820_e147458_d_n8, assign94820_e147458_d_n9, assign94820_e147458_d_n10, assign94820_e147458_d_n11, assign94820_e147458_d_n14,) = {
    if (((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2200 != 0.0)) {
        let assign94820_e147456: f64 = (locals.var_cnst0over_func * locals.var_xi0p12);
        (assign94820_e147456, ((locals.var_cnst0over_func_dn0 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn0)), ((locals.var_cnst0over_func_dn2 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn2)), ((locals.var_cnst0over_func_dn4 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn4)), ((locals.var_cnst0over_func_dn5 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn5)), ((locals.var_cnst0over_func_dn6 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn6)), ((locals.var_cnst0over_func_dn7 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn7)), ((locals.var_cnst0over_func_dn8 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn8)), ((locals.var_cnst0over_func_dn9 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn9)), ((locals.var_cnst0over_func_dn10 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn10)), ((locals.var_cnst0over_func_dn11 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn11)), ((locals.var_cnst0over_func_dn14 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn14)),)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn8, locals.var_qbuld_dn9, locals.var_qbuld_dn10, locals.var_qbuld_dn11, locals.var_qbuld_dn14,)
    }
};
        locals.var_qbuld = assign94820_e147458;
        locals.var_qbuld_dn0 = assign94820_e147458_d_n0;
        locals.var_qbuld_dn2 = assign94820_e147458_d_n2;
        locals.var_qbuld_dn4 = assign94820_e147458_d_n4;
        locals.var_qbuld_dn5 = assign94820_e147458_d_n5;
        locals.var_qbuld_dn6 = assign94820_e147458_d_n6;
        locals.var_qbuld_dn7 = assign94820_e147458_d_n7;
        locals.var_qbuld_dn8 = assign94820_e147458_d_n8;
        locals.var_qbuld_dn9 = assign94820_e147458_d_n9;
        locals.var_qbuld_dn10 = assign94820_e147458_d_n10;
        locals.var_qbuld_dn11 = assign94820_e147458_d_n11;
        locals.var_qbuld_dn14 = assign94820_e147458_d_n14;
        locals.var_qbuld_rv = 0.0;

        let (assign94830_e147470, assign94830_e147470_d_n0, assign94830_e147470_d_n2, assign94830_e147470_d_n4, assign94830_e147470_d_n5, assign94830_e147470_d_n6, assign94830_e147470_d_n7, assign94830_e147470_d_n8, assign94830_e147470_d_n9, assign94830_e147470_d_n10, assign94830_e147470_d_n11, assign94830_e147470_d_n14,) = {
    if (((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2200 != 0.0)) {
        let assign94830_e147467: f64 = (locals.var_fs02 + locals.var_xi0p12);
        let assign94830_e147468: f64 = (1.0 / assign94830_e147467);
        (assign94830_e147468, (-((locals.var_fs02_dn0 + locals.var_xi0p12_dn0) / (assign94830_e147467 * assign94830_e147467))), (-((locals.var_fs02_dn2 + locals.var_xi0p12_dn2) / (assign94830_e147467 * assign94830_e147467))), (-((locals.var_fs02_dn4 + locals.var_xi0p12_dn4) / (assign94830_e147467 * assign94830_e147467))), (-((locals.var_fs02_dn5 + locals.var_xi0p12_dn5) / (assign94830_e147467 * assign94830_e147467))), (-((locals.var_fs02_dn6 + locals.var_xi0p12_dn6) / (assign94830_e147467 * assign94830_e147467))), (-((locals.var_fs02_dn7 + locals.var_xi0p12_dn7) / (assign94830_e147467 * assign94830_e147467))), (-((locals.var_fs02_dn8 + locals.var_xi0p12_dn8) / (assign94830_e147467 * assign94830_e147467))), (-((locals.var_fs02_dn9 + locals.var_xi0p12_dn9) / (assign94830_e147467 * assign94830_e147467))), (-((locals.var_fs02_dn10 + locals.var_xi0p12_dn10) / (assign94830_e147467 * assign94830_e147467))), (-((locals.var_fs02_dn11 + locals.var_xi0p12_dn11) / (assign94830_e147467 * assign94830_e147467))), (-((locals.var_fs02_dn14 + locals.var_xi0p12_dn14) / (assign94830_e147467 * assign94830_e147467))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign94830_e147470;
        locals.var_t1_dn0 = assign94830_e147470_d_n0;
        locals.var_t1_dn2 = assign94830_e147470_d_n2;
        locals.var_t1_dn4 = assign94830_e147470_d_n4;
        locals.var_t1_dn5 = assign94830_e147470_d_n5;
        locals.var_t1_dn6 = assign94830_e147470_d_n6;
        locals.var_t1_dn7 = assign94830_e147470_d_n7;
        locals.var_t1_dn8 = assign94830_e147470_d_n8;
        locals.var_t1_dn9 = assign94830_e147470_d_n9;
        locals.var_t1_dn10 = assign94830_e147470_d_n10;
        locals.var_t1_dn11 = assign94830_e147470_d_n11;
        locals.var_t1_dn14 = assign94830_e147470_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign94840_e147482, assign94840_e147482_d_n0, assign94840_e147482_d_n2, assign94840_e147482_d_n4, assign94840_e147482_d_n5, assign94840_e147482_d_n6, assign94840_e147482_d_n7, assign94840_e147482_d_n8, assign94840_e147482_d_n9, assign94840_e147482_d_n10, assign94840_e147482_d_n11, assign94840_e147482_d_n14,) = {
    if (((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2200 != 0.0)) {
        let assign94840_e147478: f64 = (locals.var_cnst0over_func * locals.var_fs01);
        let assign94840_e147480: f64 = (assign94840_e147478 * locals.var_t1);
        (assign94840_e147480, ((((locals.var_cnst0over_func_dn0 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn0)) * locals.var_t1) + (assign94840_e147478 * locals.var_t1_dn0)), ((((locals.var_cnst0over_func_dn2 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn2)) * locals.var_t1) + (assign94840_e147478 * locals.var_t1_dn2)), ((((locals.var_cnst0over_func_dn4 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn4)) * locals.var_t1) + (assign94840_e147478 * locals.var_t1_dn4)), ((((locals.var_cnst0over_func_dn5 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn5)) * locals.var_t1) + (assign94840_e147478 * locals.var_t1_dn5)), ((((locals.var_cnst0over_func_dn6 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn6)) * locals.var_t1) + (assign94840_e147478 * locals.var_t1_dn6)), ((((locals.var_cnst0over_func_dn7 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn7)) * locals.var_t1) + (assign94840_e147478 * locals.var_t1_dn7)), ((((locals.var_cnst0over_func_dn8 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn8)) * locals.var_t1) + (assign94840_e147478 * locals.var_t1_dn8)), ((((locals.var_cnst0over_func_dn9 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn9)) * locals.var_t1) + (assign94840_e147478 * locals.var_t1_dn9)), ((((locals.var_cnst0over_func_dn10 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn10)) * locals.var_t1) + (assign94840_e147478 * locals.var_t1_dn10)), ((((locals.var_cnst0over_func_dn11 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn11)) * locals.var_t1) + (assign94840_e147478 * locals.var_t1_dn11)), ((((locals.var_cnst0over_func_dn14 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn14)) * locals.var_t1) + (assign94840_e147478 * locals.var_t1_dn14)),)
    } else {
        (locals.var_qiuld, locals.var_qiuld_dn0, locals.var_qiuld_dn2, locals.var_qiuld_dn4, locals.var_qiuld_dn5, locals.var_qiuld_dn6, locals.var_qiuld_dn7, locals.var_qiuld_dn8, locals.var_qiuld_dn9, locals.var_qiuld_dn10, locals.var_qiuld_dn11, locals.var_qiuld_dn14,)
    }
};
        locals.var_qiuld = assign94840_e147482;
        locals.var_qiuld_dn0 = assign94840_e147482_d_n0;
        locals.var_qiuld_dn2 = assign94840_e147482_d_n2;
        locals.var_qiuld_dn4 = assign94840_e147482_d_n4;
        locals.var_qiuld_dn5 = assign94840_e147482_d_n5;
        locals.var_qiuld_dn6 = assign94840_e147482_d_n6;
        locals.var_qiuld_dn7 = assign94840_e147482_d_n7;
        locals.var_qiuld_dn8 = assign94840_e147482_d_n8;
        locals.var_qiuld_dn9 = assign94840_e147482_d_n9;
        locals.var_qiuld_dn10 = assign94840_e147482_d_n10;
        locals.var_qiuld_dn11 = assign94840_e147482_d_n11;
        locals.var_qiuld_dn14 = assign94840_e147482_d_n14;
        locals.var_qiuld_rv = 0.0;

        let (assign94850_e147492, assign94850_e147492_d_n0, assign94850_e147492_d_n2, assign94850_e147492_d_n4, assign94850_e147492_d_n5, assign94850_e147492_d_n6, assign94850_e147492_d_n7, assign94850_e147492_d_n8, assign94850_e147492_d_n9, assign94850_e147492_d_n10, assign94850_e147492_d_n11, assign94850_e147492_d_n14,) = {
    if (((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2200 != 0.0)) {
        let assign94850_e147490: f64 = (locals.var_qbuld + locals.var_qiuld);
        (assign94850_e147490, (locals.var_qbuld_dn0 + locals.var_qiuld_dn0), (locals.var_qbuld_dn2 + locals.var_qiuld_dn2), (locals.var_qbuld_dn4 + locals.var_qiuld_dn4), (locals.var_qbuld_dn5 + locals.var_qiuld_dn5), (locals.var_qbuld_dn6 + locals.var_qiuld_dn6), (locals.var_qbuld_dn7 + locals.var_qiuld_dn7), (locals.var_qbuld_dn8 + locals.var_qiuld_dn8), (locals.var_qbuld_dn9 + locals.var_qiuld_dn9), (locals.var_qbuld_dn10 + locals.var_qiuld_dn10), (locals.var_qbuld_dn11 + locals.var_qiuld_dn11), (locals.var_qbuld_dn14 + locals.var_qiuld_dn14),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn4, locals.var_qsuld_dn5, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn8, locals.var_qsuld_dn9, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn14,)
    }
};
        locals.var_qsuld = assign94850_e147492;
        locals.var_qsuld_dn0 = assign94850_e147492_d_n0;
        locals.var_qsuld_dn2 = assign94850_e147492_d_n2;
        locals.var_qsuld_dn4 = assign94850_e147492_d_n4;
        locals.var_qsuld_dn5 = assign94850_e147492_d_n5;
        locals.var_qsuld_dn6 = assign94850_e147492_d_n6;
        locals.var_qsuld_dn7 = assign94850_e147492_d_n7;
        locals.var_qsuld_dn8 = assign94850_e147492_d_n8;
        locals.var_qsuld_dn9 = assign94850_e147492_d_n9;
        locals.var_qsuld_dn10 = assign94850_e147492_d_n10;
        locals.var_qsuld_dn11 = assign94850_e147492_d_n11;
        locals.var_qsuld_dn14 = assign94850_e147492_d_n14;
        locals.var_qsuld_rv = 0.0;

        let (assign94860_e147500, assign94860_e147500_d_n0, assign94860_e147500_d_n2, assign94860_e147500_d_n4, assign94860_e147500_d_n5, assign94860_e147500_d_n6, assign94860_e147500_d_n7, assign94860_e147500_d_n8, assign94860_e147500_d_n9, assign94860_e147500_d_n10, assign94860_e147500_d_n11, assign94860_e147500_d_n14,) = {
    if ((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) {
        let assign94860_e147498: f64 = (locals.var_qsuld - locals.var_qbuld);
        (assign94860_e147498, (locals.var_qsuld_dn0 - locals.var_qbuld_dn0), (locals.var_qsuld_dn2 - locals.var_qbuld_dn2), (locals.var_qsuld_dn4 - locals.var_qbuld_dn4), (locals.var_qsuld_dn5 - locals.var_qbuld_dn5), (locals.var_qsuld_dn6 - locals.var_qbuld_dn6), (locals.var_qsuld_dn7 - locals.var_qbuld_dn7), (locals.var_qsuld_dn8 - locals.var_qbuld_dn8), (locals.var_qsuld_dn9 - locals.var_qbuld_dn9), (locals.var_qsuld_dn10 - locals.var_qbuld_dn10), (locals.var_qsuld_dn11 - locals.var_qbuld_dn11), (locals.var_qsuld_dn14 - locals.var_qbuld_dn14),)
    } else {
        (locals.var_qiuld, locals.var_qiuld_dn0, locals.var_qiuld_dn2, locals.var_qiuld_dn4, locals.var_qiuld_dn5, locals.var_qiuld_dn6, locals.var_qiuld_dn7, locals.var_qiuld_dn8, locals.var_qiuld_dn9, locals.var_qiuld_dn10, locals.var_qiuld_dn11, locals.var_qiuld_dn14,)
    }
};
        locals.var_qiuld = assign94860_e147500;
        locals.var_qiuld_dn0 = assign94860_e147500_d_n0;
        locals.var_qiuld_dn2 = assign94860_e147500_d_n2;
        locals.var_qiuld_dn4 = assign94860_e147500_d_n4;
        locals.var_qiuld_dn5 = assign94860_e147500_d_n5;
        locals.var_qiuld_dn6 = assign94860_e147500_d_n6;
        locals.var_qiuld_dn7 = assign94860_e147500_d_n7;
        locals.var_qiuld_dn8 = assign94860_e147500_d_n8;
        locals.var_qiuld_dn9 = assign94860_e147500_d_n9;
        locals.var_qiuld_dn10 = assign94860_e147500_d_n10;
        locals.var_qiuld_dn11 = assign94860_e147500_d_n11;
        locals.var_qiuld_dn14 = assign94860_e147500_d_n14;
        locals.var_qiuld_rv = 0.0;

        let assign94870_e147503: f64 = if locals.var_lover_func < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2215 = assign94870_e147503;
        locals.var_guard2215_rv = 0.0;

        let (assign94880_e147512, assign94880_e147512_d_n0, assign94880_e147512_d_n2, assign94880_e147512_d_n4, assign94880_e147512_d_n5, assign94880_e147512_d_n6, assign94880_e147512_d_n7, assign94880_e147512_d_n8, assign94880_e147512_d_n9, assign94880_e147512_d_n10, assign94880_e147512_d_n11, assign94880_e147512_d_n14,) = {
    if (((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2215 != 0.0)) {
        let assign94880_e147510: f64 = (-locals.var_lover_func);
        (assign94880_e147510, (-locals.var_lover_func_dn0), (-locals.var_lover_func_dn2), (-locals.var_lover_func_dn4), (-locals.var_lover_func_dn5), (-locals.var_lover_func_dn6), (-locals.var_lover_func_dn7), (-locals.var_lover_func_dn8), (-locals.var_lover_func_dn9), (-locals.var_lover_func_dn10), (-locals.var_lover_func_dn11), (-locals.var_lover_func_dn14),)
    } else {
        (locals.var_lover_func, locals.var_lover_func_dn0, locals.var_lover_func_dn2, locals.var_lover_func_dn4, locals.var_lover_func_dn5, locals.var_lover_func_dn6, locals.var_lover_func_dn7, locals.var_lover_func_dn8, locals.var_lover_func_dn9, locals.var_lover_func_dn10, locals.var_lover_func_dn11, locals.var_lover_func_dn14,)
    }
};
        locals.var_lover_func = assign94880_e147512;
        locals.var_lover_func_dn0 = assign94880_e147512_d_n0;
        locals.var_lover_func_dn2 = assign94880_e147512_d_n2;
        locals.var_lover_func_dn4 = assign94880_e147512_d_n4;
        locals.var_lover_func_dn5 = assign94880_e147512_d_n5;
        locals.var_lover_func_dn6 = assign94880_e147512_d_n6;
        locals.var_lover_func_dn7 = assign94880_e147512_d_n7;
        locals.var_lover_func_dn8 = assign94880_e147512_d_n8;
        locals.var_lover_func_dn9 = assign94880_e147512_d_n9;
        locals.var_lover_func_dn10 = assign94880_e147512_d_n10;
        locals.var_lover_func_dn11 = assign94880_e147512_d_n11;
        locals.var_lover_func_dn14 = assign94880_e147512_d_n14;
        locals.var_lover_func_rv = 0.0;

        let assign94890_e147515: f64 = if p.p55 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2216 = assign94890_e147515;
        locals.var_guard2216_rv = 0.0;

        let assign94900_e147518: f64 = if p.p50 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2217 = assign94900_e147518;
        locals.var_guard2217_rv = 0.0;

        let (assign94910_e147531, assign94910_e147531_d_n0, assign94910_e147531_d_n2, assign94910_e147531_d_n4, assign94910_e147531_d_n5, assign94910_e147531_d_n6, assign94910_e147531_d_n7, assign94910_e147531_d_n8, assign94910_e147531_d_n9, assign94910_e147531_d_n10, assign94910_e147531_d_n11, assign94910_e147531_d_n14,) = {
    if (((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2215 != 0.0)) && (locals.var_guard2216 != 0.0)) && (locals.var_guard2217 != 0.0)) {
        let assign94910_e147529: f64 = (-locals.var_ps0ld);
        (assign94910_e147529, (-locals.var_ps0ld_dn0), (-locals.var_ps0ld_dn2), (-locals.var_ps0ld_dn4), (-locals.var_ps0ld_dn5), (-locals.var_ps0ld_dn6), (-locals.var_ps0ld_dn7), (-locals.var_ps0ld_dn8), (-locals.var_ps0ld_dn9), (-locals.var_ps0ld_dn10), (-locals.var_ps0ld_dn11), (-locals.var_ps0ld_dn14),)
    } else {
        (locals.var_vx__blk2120, locals.var_vx__blk2120_dn0, locals.var_vx__blk2120_dn2, locals.var_vx__blk2120_dn4, locals.var_vx__blk2120_dn5, locals.var_vx__blk2120_dn6, locals.var_vx__blk2120_dn7, locals.var_vx__blk2120_dn8, locals.var_vx__blk2120_dn9, locals.var_vx__blk2120_dn10, locals.var_vx__blk2120_dn11, locals.var_vx__blk2120_dn14,)
    }
};
        locals.var_vx__blk2120 = assign94910_e147531;
        locals.var_vx__blk2120_dn0 = assign94910_e147531_d_n0;
        locals.var_vx__blk2120_dn2 = assign94910_e147531_d_n2;
        locals.var_vx__blk2120_dn4 = assign94910_e147531_d_n4;
        locals.var_vx__blk2120_dn5 = assign94910_e147531_d_n5;
        locals.var_vx__blk2120_dn6 = assign94910_e147531_d_n6;
        locals.var_vx__blk2120_dn7 = assign94910_e147531_d_n7;
        locals.var_vx__blk2120_dn8 = assign94910_e147531_d_n8;
        locals.var_vx__blk2120_dn9 = assign94910_e147531_d_n9;
        locals.var_vx__blk2120_dn10 = assign94910_e147531_d_n10;
        locals.var_vx__blk2120_dn11 = assign94910_e147531_d_n11;
        locals.var_vx__blk2120_dn14 = assign94910_e147531_d_n14;
        locals.var_vx__blk2120_rv = 0.0;

        let (assign94920_e147544, assign94920_e147544_d_n0, assign94920_e147544_d_n2, assign94920_e147544_d_n4, assign94920_e147544_d_n5, assign94920_e147544_d_n6, assign94920_e147544_d_n7, assign94920_e147544_d_n8, assign94920_e147544_d_n9, assign94920_e147544_d_n10, assign94920_e147544_d_n11, assign94920_e147544_d_n14,) = {
    if (((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2215 != 0.0)) && (locals.var_guard2216 != 0.0)) && (locals.var_guard2217 == 0.0)) {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn11, locals.var_vxbgmt_dn14,)
    } else {
        (locals.var_vx__blk2120, locals.var_vx__blk2120_dn0, locals.var_vx__blk2120_dn2, locals.var_vx__blk2120_dn4, locals.var_vx__blk2120_dn5, locals.var_vx__blk2120_dn6, locals.var_vx__blk2120_dn7, locals.var_vx__blk2120_dn8, locals.var_vx__blk2120_dn9, locals.var_vx__blk2120_dn10, locals.var_vx__blk2120_dn11, locals.var_vx__blk2120_dn14,)
    }
};
        locals.var_vx__blk2120 = assign94920_e147544;
        locals.var_vx__blk2120_dn0 = assign94920_e147544_d_n0;
        locals.var_vx__blk2120_dn2 = assign94920_e147544_d_n2;
        locals.var_vx__blk2120_dn4 = assign94920_e147544_d_n4;
        locals.var_vx__blk2120_dn5 = assign94920_e147544_d_n5;
        locals.var_vx__blk2120_dn6 = assign94920_e147544_d_n6;
        locals.var_vx__blk2120_dn7 = assign94920_e147544_d_n7;
        locals.var_vx__blk2120_dn8 = assign94920_e147544_d_n8;
        locals.var_vx__blk2120_dn9 = assign94920_e147544_d_n9;
        locals.var_vx__blk2120_dn10 = assign94920_e147544_d_n10;
        locals.var_vx__blk2120_dn11 = assign94920_e147544_d_n11;
        locals.var_vx__blk2120_dn14 = assign94920_e147544_d_n14;
        locals.var_vx__blk2120_rv = 0.0;

        let (assign94930_e147567, assign94930_e147567_d_n0, assign94930_e147567_d_n2, assign94930_e147567_d_n4, assign94930_e147567_d_n5, assign94930_e147567_d_n6, assign94930_e147567_d_n7, assign94930_e147567_d_n8, assign94930_e147567_d_n9, assign94930_e147567_d_n10, assign94930_e147567_d_n11, assign94930_e147567_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2215 != 0.0)) && (locals.var_guard2216 != 0.0)) {
        let assign94930_e147554: f64 = (locals.var_vx__blk2120 + p.p137);
        let assign94930_e147557: f64 = (locals.var_vx__blk2120 + p.p137);
        let assign94930_e147558: f64 = (assign94930_e147554 * assign94930_e147557);
        let assign94930_e147561: f64 = (4.0 * 0.1);
        let assign94930_e147563: f64 = (assign94930_e147561 * 0.1);
        let assign94930_e147564: f64 = (assign94930_e147558 + assign94930_e147563);
        let assign94930_e147565: f64 = (assign94930_e147564).sqrt();
        (assign94930_e147565, (((locals.var_vx__blk2120_dn0 * assign94930_e147557) + (assign94930_e147554 * locals.var_vx__blk2120_dn0)) / (2.0 * assign94930_e147565)), (((locals.var_vx__blk2120_dn2 * assign94930_e147557) + (assign94930_e147554 * locals.var_vx__blk2120_dn2)) / (2.0 * assign94930_e147565)), (((locals.var_vx__blk2120_dn4 * assign94930_e147557) + (assign94930_e147554 * locals.var_vx__blk2120_dn4)) / (2.0 * assign94930_e147565)), (((locals.var_vx__blk2120_dn5 * assign94930_e147557) + (assign94930_e147554 * locals.var_vx__blk2120_dn5)) / (2.0 * assign94930_e147565)), (((locals.var_vx__blk2120_dn6 * assign94930_e147557) + (assign94930_e147554 * locals.var_vx__blk2120_dn6)) / (2.0 * assign94930_e147565)), (((locals.var_vx__blk2120_dn7 * assign94930_e147557) + (assign94930_e147554 * locals.var_vx__blk2120_dn7)) / (2.0 * assign94930_e147565)), (((locals.var_vx__blk2120_dn8 * assign94930_e147557) + (assign94930_e147554 * locals.var_vx__blk2120_dn8)) / (2.0 * assign94930_e147565)), (((locals.var_vx__blk2120_dn9 * assign94930_e147557) + (assign94930_e147554 * locals.var_vx__blk2120_dn9)) / (2.0 * assign94930_e147565)), (((locals.var_vx__blk2120_dn10 * assign94930_e147557) + (assign94930_e147554 * locals.var_vx__blk2120_dn10)) / (2.0 * assign94930_e147565)), (((locals.var_vx__blk2120_dn11 * assign94930_e147557) + (assign94930_e147554 * locals.var_vx__blk2120_dn11)) / (2.0 * assign94930_e147565)), (((locals.var_vx__blk2120_dn14 * assign94930_e147557) + (assign94930_e147554 * locals.var_vx__blk2120_dn14)) / (2.0 * assign94930_e147565)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign94930_e147567;
        locals.var_tmf2_dn0 = assign94930_e147567_d_n0;
        locals.var_tmf2_dn2 = assign94930_e147567_d_n2;
        locals.var_tmf2_dn4 = assign94930_e147567_d_n4;
        locals.var_tmf2_dn5 = assign94930_e147567_d_n5;
        locals.var_tmf2_dn6 = assign94930_e147567_d_n6;
        locals.var_tmf2_dn7 = assign94930_e147567_d_n7;
        locals.var_tmf2_dn8 = assign94930_e147567_d_n8;
        locals.var_tmf2_dn9 = assign94930_e147567_d_n9;
        locals.var_tmf2_dn10 = assign94930_e147567_d_n10;
        locals.var_tmf2_dn11 = assign94930_e147567_d_n11;
        locals.var_tmf2_dn14 = assign94930_e147567_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign94940_e147585, assign94940_e147585_d_n0, assign94940_e147585_d_n2, assign94940_e147585_d_n4, assign94940_e147585_d_n5, assign94940_e147585_d_n6, assign94940_e147585_d_n7, assign94940_e147585_d_n8, assign94940_e147585_d_n9, assign94940_e147585_d_n10, assign94940_e147585_d_n11, assign94940_e147585_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2215 != 0.0)) && (locals.var_guard2216 != 0.0)) {
        let assign94940_e147579: f64 = (locals.var_vx__blk2120 + p.p137);
        let assign94940_e147581: f64 = (assign94940_e147579 / locals.var_tmf2);
        let assign94940_e147582: f64 = (1.0 + assign94940_e147581);
        let assign94940_e147583: f64 = (0.5 * assign94940_e147582);
        (assign94940_e147583, (0.5 * (((locals.var_vx__blk2120_dn0 * locals.var_tmf2) - (assign94940_e147579 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk2120_dn2 * locals.var_tmf2) - (assign94940_e147579 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk2120_dn4 * locals.var_tmf2) - (assign94940_e147579 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk2120_dn5 * locals.var_tmf2) - (assign94940_e147579 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk2120_dn6 * locals.var_tmf2) - (assign94940_e147579 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk2120_dn7 * locals.var_tmf2) - (assign94940_e147579 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk2120_dn8 * locals.var_tmf2) - (assign94940_e147579 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk2120_dn9 * locals.var_tmf2) - (assign94940_e147579 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk2120_dn10 * locals.var_tmf2) - (assign94940_e147579 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk2120_dn11 * locals.var_tmf2) - (assign94940_e147579 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk2120_dn14 * locals.var_tmf2) - (assign94940_e147579 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign94940_e147585;
        locals.var_t9_dn0 = assign94940_e147585_d_n0;
        locals.var_t9_dn2 = assign94940_e147585_d_n2;
        locals.var_t9_dn4 = assign94940_e147585_d_n4;
        locals.var_t9_dn5 = assign94940_e147585_d_n5;
        locals.var_t9_dn6 = assign94940_e147585_d_n6;
        locals.var_t9_dn7 = assign94940_e147585_d_n7;
        locals.var_t9_dn8 = assign94940_e147585_d_n8;
        locals.var_t9_dn9 = assign94940_e147585_d_n9;
        locals.var_t9_dn10 = assign94940_e147585_d_n10;
        locals.var_t9_dn11 = assign94940_e147585_d_n11;
        locals.var_t9_dn14 = assign94940_e147585_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign94950_e147601, assign94950_e147601_d_n0, assign94950_e147601_d_n2, assign94950_e147601_d_n4, assign94950_e147601_d_n5, assign94950_e147601_d_n6, assign94950_e147601_d_n7, assign94950_e147601_d_n8, assign94950_e147601_d_n9, assign94950_e147601_d_n10, assign94950_e147601_d_n11, assign94950_e147601_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2215 != 0.0)) && (locals.var_guard2216 != 0.0)) {
        let assign94950_e147596: f64 = (locals.var_vx__blk2120 + p.p137);
        let assign94950_e147598: f64 = (assign94950_e147596 + locals.var_tmf2);
        let assign94950_e147599: f64 = (0.5 * assign94950_e147598);
        (assign94950_e147599, (0.5 * (locals.var_vx__blk2120_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_vx__blk2120_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_vx__blk2120_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_vx__blk2120_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_vx__blk2120_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_vx__blk2120_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_vx__blk2120_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_vx__blk2120_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_vx__blk2120_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_vx__blk2120_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_vx__blk2120_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign94950_e147601;
        locals.var_t2_dn0 = assign94950_e147601_d_n0;
        locals.var_t2_dn2 = assign94950_e147601_d_n2;
        locals.var_t2_dn4 = assign94950_e147601_d_n4;
        locals.var_t2_dn5 = assign94950_e147601_d_n5;
        locals.var_t2_dn6 = assign94950_e147601_d_n6;
        locals.var_t2_dn7 = assign94950_e147601_d_n7;
        locals.var_t2_dn8 = assign94950_e147601_d_n8;
        locals.var_t2_dn9 = assign94950_e147601_d_n9;
        locals.var_t2_dn10 = assign94950_e147601_d_n10;
        locals.var_t2_dn11 = assign94950_e147601_d_n11;
        locals.var_t2_dn14 = assign94950_e147601_d_n14;
        locals.var_t2_rv = 0.0;

        let assign94960_e147604: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2218 = assign94960_e147604;
        locals.var_guard2218_rv = 0.0;

        let (assign94970_e147616, assign94970_e147616_d_n0, assign94970_e147616_d_n2, assign94970_e147616_d_n4, assign94970_e147616_d_n5, assign94970_e147616_d_n6, assign94970_e147616_d_n7, assign94970_e147616_d_n8, assign94970_e147616_d_n9, assign94970_e147616_d_n10, assign94970_e147616_d_n11, assign94970_e147616_d_n14,) = {
    if (((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2215 != 0.0)) && (locals.var_guard2216 != 0.0)) && (locals.var_guard2218 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign94970_e147616;
        locals.var_t2_dn0 = assign94970_e147616_d_n0;
        locals.var_t2_dn2 = assign94970_e147616_d_n2;
        locals.var_t2_dn4 = assign94970_e147616_d_n4;
        locals.var_t2_dn5 = assign94970_e147616_d_n5;
        locals.var_t2_dn6 = assign94970_e147616_d_n6;
        locals.var_t2_dn7 = assign94970_e147616_d_n7;
        locals.var_t2_dn8 = assign94970_e147616_d_n8;
        locals.var_t2_dn9 = assign94970_e147616_d_n9;
        locals.var_t2_dn10 = assign94970_e147616_d_n10;
        locals.var_t2_dn11 = assign94970_e147616_d_n11;
        locals.var_t2_dn14 = assign94970_e147616_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign94980_e147628, assign94980_e147628_d_n0, assign94980_e147628_d_n2, assign94980_e147628_d_n4, assign94980_e147628_d_n5, assign94980_e147628_d_n6, assign94980_e147628_d_n7, assign94980_e147628_d_n8, assign94980_e147628_d_n9, assign94980_e147628_d_n10, assign94980_e147628_d_n11, assign94980_e147628_d_n14,) = {
    if (((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2215 != 0.0)) && (locals.var_guard2216 != 0.0)) && (locals.var_guard2218 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign94980_e147628;
        locals.var_t9_dn0 = assign94980_e147628_d_n0;
        locals.var_t9_dn2 = assign94980_e147628_d_n2;
        locals.var_t9_dn4 = assign94980_e147628_d_n4;
        locals.var_t9_dn5 = assign94980_e147628_d_n5;
        locals.var_t9_dn6 = assign94980_e147628_d_n6;
        locals.var_t9_dn7 = assign94980_e147628_d_n7;
        locals.var_t9_dn8 = assign94980_e147628_d_n8;
        locals.var_t9_dn9 = assign94980_e147628_d_n9;
        locals.var_t9_dn10 = assign94980_e147628_d_n10;
        locals.var_t9_dn11 = assign94980_e147628_d_n11;
        locals.var_t9_dn14 = assign94980_e147628_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign94990_e147643, assign94990_e147643_d_n0, assign94990_e147643_d_n2, assign94990_e147643_d_n4, assign94990_e147643_d_n5, assign94990_e147643_d_n6, assign94990_e147643_d_n7, assign94990_e147643_d_n8, assign94990_e147643_d_n9, assign94990_e147643_d_n10, assign94990_e147643_d_n11, assign94990_e147643_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2215 != 0.0)) && (locals.var_guard2216 != 0.0)) {
        let assign94990_e147638: f64 = (locals.var_kjunc * locals.var_t2);
        let assign94990_e147639: f64 = (assign94990_e147638).sqrt();
        let assign94990_e147641: f64 = (assign94990_e147639 * p.p432);
        (assign94990_e147641, ((((locals.var_kjunc_dn0 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn0)) / (2.0 * assign94990_e147639)) * p.p432), ((((locals.var_kjunc_dn2 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn2)) / (2.0 * assign94990_e147639)) * p.p432), ((((locals.var_kjunc_dn4 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn4)) / (2.0 * assign94990_e147639)) * p.p432), ((((locals.var_kjunc_dn5 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn5)) / (2.0 * assign94990_e147639)) * p.p432), ((((locals.var_kjunc_dn6 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn6)) / (2.0 * assign94990_e147639)) * p.p432), ((((locals.var_kjunc_dn7 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn7)) / (2.0 * assign94990_e147639)) * p.p432), ((((locals.var_kjunc_dn8 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn8)) / (2.0 * assign94990_e147639)) * p.p432), ((((locals.var_kjunc_dn9 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn9)) / (2.0 * assign94990_e147639)) * p.p432), ((((locals.var_kjunc_dn10 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn10)) / (2.0 * assign94990_e147639)) * p.p432), ((((locals.var_kjunc_dn11 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn11)) / (2.0 * assign94990_e147639)) * p.p432), ((((locals.var_kjunc_dn14 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn14)) / (2.0 * assign94990_e147639)) * p.p432),)
    } else {
        (locals.var_wjunc0, locals.var_wjunc0_dn0, locals.var_wjunc0_dn2, locals.var_wjunc0_dn4, locals.var_wjunc0_dn5, locals.var_wjunc0_dn6, locals.var_wjunc0_dn7, locals.var_wjunc0_dn8, locals.var_wjunc0_dn9, locals.var_wjunc0_dn10, locals.var_wjunc0_dn11, locals.var_wjunc0_dn14,)
    }
};
        locals.var_wjunc0 = assign94990_e147643;
        locals.var_wjunc0_dn0 = assign94990_e147643_d_n0;
        locals.var_wjunc0_dn2 = assign94990_e147643_d_n2;
        locals.var_wjunc0_dn4 = assign94990_e147643_d_n4;
        locals.var_wjunc0_dn5 = assign94990_e147643_d_n5;
        locals.var_wjunc0_dn6 = assign94990_e147643_d_n6;
        locals.var_wjunc0_dn7 = assign94990_e147643_d_n7;
        locals.var_wjunc0_dn8 = assign94990_e147643_d_n8;
        locals.var_wjunc0_dn9 = assign94990_e147643_d_n9;
        locals.var_wjunc0_dn10 = assign94990_e147643_d_n10;
        locals.var_wjunc0_dn11 = assign94990_e147643_d_n11;
        locals.var_wjunc0_dn14 = assign94990_e147643_d_n14;
        locals.var_wjunc0_rv = 0.0;

        let (assign95000_e147659, assign95000_e147659_d_n0, assign95000_e147659_d_n2, assign95000_e147659_d_n4, assign95000_e147659_d_n5, assign95000_e147659_d_n6, assign95000_e147659_d_n7, assign95000_e147659_d_n8, assign95000_e147659_d_n9, assign95000_e147659_d_n10, assign95000_e147659_d_n11, assign95000_e147659_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2215 != 0.0)) && (locals.var_guard2216 != 0.0)) {
        let assign95000_e147653: f64 = (locals.var_lover_func - locals.var_wjunc0);
        let assign95000_e147656: f64 = (0.1 * locals.var_lover_func);
        let assign95000_e147657: f64 = (assign95000_e147653 - assign95000_e147656);
        (assign95000_e147657, ((locals.var_lover_func_dn0 - locals.var_wjunc0_dn0) - (0.1 * locals.var_lover_func_dn0)), ((locals.var_lover_func_dn2 - locals.var_wjunc0_dn2) - (0.1 * locals.var_lover_func_dn2)), ((locals.var_lover_func_dn4 - locals.var_wjunc0_dn4) - (0.1 * locals.var_lover_func_dn4)), ((locals.var_lover_func_dn5 - locals.var_wjunc0_dn5) - (0.1 * locals.var_lover_func_dn5)), ((locals.var_lover_func_dn6 - locals.var_wjunc0_dn6) - (0.1 * locals.var_lover_func_dn6)), ((locals.var_lover_func_dn7 - locals.var_wjunc0_dn7) - (0.1 * locals.var_lover_func_dn7)), ((locals.var_lover_func_dn8 - locals.var_wjunc0_dn8) - (0.1 * locals.var_lover_func_dn8)), ((locals.var_lover_func_dn9 - locals.var_wjunc0_dn9) - (0.1 * locals.var_lover_func_dn9)), ((locals.var_lover_func_dn10 - locals.var_wjunc0_dn10) - (0.1 * locals.var_lover_func_dn10)), ((locals.var_lover_func_dn11 - locals.var_wjunc0_dn11) - (0.1 * locals.var_lover_func_dn11)), ((locals.var_lover_func_dn14 - locals.var_wjunc0_dn14) - (0.1 * locals.var_lover_func_dn14)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign95000_e147659;
        locals.var_tmf1_dn0 = assign95000_e147659_d_n0;
        locals.var_tmf1_dn2 = assign95000_e147659_d_n2;
        locals.var_tmf1_dn4 = assign95000_e147659_d_n4;
        locals.var_tmf1_dn5 = assign95000_e147659_d_n5;
        locals.var_tmf1_dn6 = assign95000_e147659_d_n6;
        locals.var_tmf1_dn7 = assign95000_e147659_d_n7;
        locals.var_tmf1_dn8 = assign95000_e147659_d_n8;
        locals.var_tmf1_dn9 = assign95000_e147659_d_n9;
        locals.var_tmf1_dn10 = assign95000_e147659_d_n10;
        locals.var_tmf1_dn11 = assign95000_e147659_d_n11;
        locals.var_tmf1_dn14 = assign95000_e147659_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign95010_e147675, assign95010_e147675_d_n0, assign95010_e147675_d_n2, assign95010_e147675_d_n4, assign95010_e147675_d_n5, assign95010_e147675_d_n6, assign95010_e147675_d_n7, assign95010_e147675_d_n8, assign95010_e147675_d_n9, assign95010_e147675_d_n10, assign95010_e147675_d_n11, assign95010_e147675_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2215 != 0.0)) && (locals.var_guard2216 != 0.0)) {
        let assign95010_e147669: f64 = (4.0 * locals.var_lover_func);
        let assign95010_e147672: f64 = (0.1 * locals.var_lover_func);
        let assign95010_e147673: f64 = (assign95010_e147669 * assign95010_e147672);
        (assign95010_e147673, (((4.0 * locals.var_lover_func_dn0) * assign95010_e147672) + (assign95010_e147669 * (0.1 * locals.var_lover_func_dn0))), (((4.0 * locals.var_lover_func_dn2) * assign95010_e147672) + (assign95010_e147669 * (0.1 * locals.var_lover_func_dn2))), (((4.0 * locals.var_lover_func_dn4) * assign95010_e147672) + (assign95010_e147669 * (0.1 * locals.var_lover_func_dn4))), (((4.0 * locals.var_lover_func_dn5) * assign95010_e147672) + (assign95010_e147669 * (0.1 * locals.var_lover_func_dn5))), (((4.0 * locals.var_lover_func_dn6) * assign95010_e147672) + (assign95010_e147669 * (0.1 * locals.var_lover_func_dn6))), (((4.0 * locals.var_lover_func_dn7) * assign95010_e147672) + (assign95010_e147669 * (0.1 * locals.var_lover_func_dn7))), (((4.0 * locals.var_lover_func_dn8) * assign95010_e147672) + (assign95010_e147669 * (0.1 * locals.var_lover_func_dn8))), (((4.0 * locals.var_lover_func_dn9) * assign95010_e147672) + (assign95010_e147669 * (0.1 * locals.var_lover_func_dn9))), (((4.0 * locals.var_lover_func_dn10) * assign95010_e147672) + (assign95010_e147669 * (0.1 * locals.var_lover_func_dn10))), (((4.0 * locals.var_lover_func_dn11) * assign95010_e147672) + (assign95010_e147669 * (0.1 * locals.var_lover_func_dn11))), (((4.0 * locals.var_lover_func_dn14) * assign95010_e147672) + (assign95010_e147669 * (0.1 * locals.var_lover_func_dn14))),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign95010_e147675;
        locals.var_tmf2_dn0 = assign95010_e147675_d_n0;
        locals.var_tmf2_dn2 = assign95010_e147675_d_n2;
        locals.var_tmf2_dn4 = assign95010_e147675_d_n4;
        locals.var_tmf2_dn5 = assign95010_e147675_d_n5;
        locals.var_tmf2_dn6 = assign95010_e147675_d_n6;
        locals.var_tmf2_dn7 = assign95010_e147675_d_n7;
        locals.var_tmf2_dn8 = assign95010_e147675_d_n8;
        locals.var_tmf2_dn9 = assign95010_e147675_d_n9;
        locals.var_tmf2_dn10 = assign95010_e147675_d_n10;
        locals.var_tmf2_dn11 = assign95010_e147675_d_n11;
        locals.var_tmf2_dn14 = assign95010_e147675_d_n14;
        locals.var_tmf2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_369(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign95020_e147691, assign95020_e147691_d_n0, assign95020_e147691_d_n2, assign95020_e147691_d_n4, assign95020_e147691_d_n5, assign95020_e147691_d_n6, assign95020_e147691_d_n7, assign95020_e147691_d_n8, assign95020_e147691_d_n9, assign95020_e147691_d_n10, assign95020_e147691_d_n11, assign95020_e147691_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2215 != 0.0)) && (locals.var_guard2216 != 0.0)) {
        let (assign95020_e147689, assign95020_e147689_d_n0, assign95020_e147689_d_n2, assign95020_e147689_d_n4, assign95020_e147689_d_n5, assign95020_e147689_d_n6, assign95020_e147689_d_n7, assign95020_e147689_d_n8, assign95020_e147689_d_n9, assign95020_e147689_d_n10, assign95020_e147689_d_n11, assign95020_e147689_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign95020_e147688: f64 = (-locals.var_tmf2);
                (assign95020_e147688, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign95020_e147689, assign95020_e147689_d_n0, assign95020_e147689_d_n2, assign95020_e147689_d_n4, assign95020_e147689_d_n5, assign95020_e147689_d_n6, assign95020_e147689_d_n7, assign95020_e147689_d_n8, assign95020_e147689_d_n9, assign95020_e147689_d_n10, assign95020_e147689_d_n11, assign95020_e147689_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign95020_e147691;
        locals.var_tmf2_dn0 = assign95020_e147691_d_n0;
        locals.var_tmf2_dn2 = assign95020_e147691_d_n2;
        locals.var_tmf2_dn4 = assign95020_e147691_d_n4;
        locals.var_tmf2_dn5 = assign95020_e147691_d_n5;
        locals.var_tmf2_dn6 = assign95020_e147691_d_n6;
        locals.var_tmf2_dn7 = assign95020_e147691_d_n7;
        locals.var_tmf2_dn8 = assign95020_e147691_d_n8;
        locals.var_tmf2_dn9 = assign95020_e147691_d_n9;
        locals.var_tmf2_dn10 = assign95020_e147691_d_n10;
        locals.var_tmf2_dn11 = assign95020_e147691_d_n11;
        locals.var_tmf2_dn14 = assign95020_e147691_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign95030_e147706, assign95030_e147706_d_n0, assign95030_e147706_d_n2, assign95030_e147706_d_n4, assign95030_e147706_d_n5, assign95030_e147706_d_n6, assign95030_e147706_d_n7, assign95030_e147706_d_n8, assign95030_e147706_d_n9, assign95030_e147706_d_n10, assign95030_e147706_d_n11, assign95030_e147706_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2215 != 0.0)) && (locals.var_guard2216 != 0.0)) {
        let assign95030_e147701: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign95030_e147703: f64 = (assign95030_e147701 + locals.var_tmf2);
        let assign95030_e147704: f64 = (assign95030_e147703).sqrt();
        (assign95030_e147704, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign95030_e147704)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign95030_e147704)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign95030_e147704)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign95030_e147704)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign95030_e147704)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign95030_e147704)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign95030_e147704)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign95030_e147704)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign95030_e147704)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign95030_e147704)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign95030_e147704)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign95030_e147706;
        locals.var_tmf2_dn0 = assign95030_e147706_d_n0;
        locals.var_tmf2_dn2 = assign95030_e147706_d_n2;
        locals.var_tmf2_dn4 = assign95030_e147706_d_n4;
        locals.var_tmf2_dn5 = assign95030_e147706_d_n5;
        locals.var_tmf2_dn6 = assign95030_e147706_d_n6;
        locals.var_tmf2_dn7 = assign95030_e147706_d_n7;
        locals.var_tmf2_dn8 = assign95030_e147706_d_n8;
        locals.var_tmf2_dn9 = assign95030_e147706_d_n9;
        locals.var_tmf2_dn10 = assign95030_e147706_d_n10;
        locals.var_tmf2_dn11 = assign95030_e147706_d_n11;
        locals.var_tmf2_dn14 = assign95030_e147706_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign95040_e147722, assign95040_e147722_d_n0, assign95040_e147722_d_n2, assign95040_e147722_d_n4, assign95040_e147722_d_n5, assign95040_e147722_d_n6, assign95040_e147722_d_n7, assign95040_e147722_d_n8, assign95040_e147722_d_n9, assign95040_e147722_d_n10, assign95040_e147722_d_n11, assign95040_e147722_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2215 != 0.0)) && (locals.var_guard2216 != 0.0)) {
        let assign95040_e147718: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign95040_e147719: f64 = (1.0 + assign95040_e147718);
        let assign95040_e147720: f64 = (0.5 * assign95040_e147719);
        (assign95040_e147720, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign95040_e147722;
        locals.var_t0_dn0 = assign95040_e147722_d_n0;
        locals.var_t0_dn2 = assign95040_e147722_d_n2;
        locals.var_t0_dn4 = assign95040_e147722_d_n4;
        locals.var_t0_dn5 = assign95040_e147722_d_n5;
        locals.var_t0_dn6 = assign95040_e147722_d_n6;
        locals.var_t0_dn7 = assign95040_e147722_d_n7;
        locals.var_t0_dn8 = assign95040_e147722_d_n8;
        locals.var_t0_dn9 = assign95040_e147722_d_n9;
        locals.var_t0_dn10 = assign95040_e147722_d_n10;
        locals.var_t0_dn11 = assign95040_e147722_d_n11;
        locals.var_t0_dn14 = assign95040_e147722_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign95050_e147738, assign95050_e147738_d_n0, assign95050_e147738_d_n2, assign95050_e147738_d_n4, assign95050_e147738_d_n5, assign95050_e147738_d_n6, assign95050_e147738_d_n7, assign95050_e147738_d_n8, assign95050_e147738_d_n9, assign95050_e147738_d_n10, assign95050_e147738_d_n11, assign95050_e147738_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2215 != 0.0)) && (locals.var_guard2216 != 0.0)) {
        let assign95050_e147734: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign95050_e147735: f64 = (0.5 * assign95050_e147734);
        let assign95050_e147736: f64 = (locals.var_lover_func - assign95050_e147735);
        (assign95050_e147736, (locals.var_lover_func_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_lover_func_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_lover_func_dn4 - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_lover_func_dn5 - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_lover_func_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_lover_func_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_lover_func_dn8 - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_lover_func_dn9 - (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (locals.var_lover_func_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_lover_func_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_lover_func_dn14 - (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_wjuncld, locals.var_wjuncld_dn0, locals.var_wjuncld_dn2, locals.var_wjuncld_dn4, locals.var_wjuncld_dn5, locals.var_wjuncld_dn6, locals.var_wjuncld_dn7, locals.var_wjuncld_dn8, locals.var_wjuncld_dn9, locals.var_wjuncld_dn10, locals.var_wjuncld_dn11, locals.var_wjuncld_dn14,)
    }
};
        locals.var_wjuncld = assign95050_e147738;
        locals.var_wjuncld_dn0 = assign95050_e147738_d_n0;
        locals.var_wjuncld_dn2 = assign95050_e147738_d_n2;
        locals.var_wjuncld_dn4 = assign95050_e147738_d_n4;
        locals.var_wjuncld_dn5 = assign95050_e147738_d_n5;
        locals.var_wjuncld_dn6 = assign95050_e147738_d_n6;
        locals.var_wjuncld_dn7 = assign95050_e147738_d_n7;
        locals.var_wjuncld_dn8 = assign95050_e147738_d_n8;
        locals.var_wjuncld_dn9 = assign95050_e147738_d_n9;
        locals.var_wjuncld_dn10 = assign95050_e147738_d_n10;
        locals.var_wjuncld_dn11 = assign95050_e147738_d_n11;
        locals.var_wjuncld_dn14 = assign95050_e147738_d_n14;
        locals.var_wjuncld_rv = 0.0;

        let (assign95060_e147750, assign95060_e147750_d_n0, assign95060_e147750_d_n2, assign95060_e147750_d_n4, assign95060_e147750_d_n5, assign95060_e147750_d_n6, assign95060_e147750_d_n7, assign95060_e147750_d_n8, assign95060_e147750_d_n9, assign95060_e147750_d_n10, assign95060_e147750_d_n11, assign95060_e147750_d_n14,) = {
    if ((((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) && (locals.var_guard2215 != 0.0)) && (locals.var_guard2216 != 0.0)) {
        let assign95060_e147748: f64 = (locals.var_lover_func - locals.var_wjuncld);
        (assign95060_e147748, (locals.var_lover_func_dn0 - locals.var_wjuncld_dn0), (locals.var_lover_func_dn2 - locals.var_wjuncld_dn2), (locals.var_lover_func_dn4 - locals.var_wjuncld_dn4), (locals.var_lover_func_dn5 - locals.var_wjuncld_dn5), (locals.var_lover_func_dn6 - locals.var_wjuncld_dn6), (locals.var_lover_func_dn7 - locals.var_wjuncld_dn7), (locals.var_lover_func_dn8 - locals.var_wjuncld_dn8), (locals.var_lover_func_dn9 - locals.var_wjuncld_dn9), (locals.var_lover_func_dn10 - locals.var_wjuncld_dn10), (locals.var_lover_func_dn11 - locals.var_wjuncld_dn11), (locals.var_lover_func_dn14 - locals.var_wjuncld_dn14),)
    } else {
        (locals.var_lover_func, locals.var_lover_func_dn0, locals.var_lover_func_dn2, locals.var_lover_func_dn4, locals.var_lover_func_dn5, locals.var_lover_func_dn6, locals.var_lover_func_dn7, locals.var_lover_func_dn8, locals.var_lover_func_dn9, locals.var_lover_func_dn10, locals.var_lover_func_dn11, locals.var_lover_func_dn14,)
    }
};
        locals.var_lover_func = assign95060_e147750;
        locals.var_lover_func_dn0 = assign95060_e147750_d_n0;
        locals.var_lover_func_dn2 = assign95060_e147750_d_n2;
        locals.var_lover_func_dn4 = assign95060_e147750_d_n4;
        locals.var_lover_func_dn5 = assign95060_e147750_d_n5;
        locals.var_lover_func_dn6 = assign95060_e147750_d_n6;
        locals.var_lover_func_dn7 = assign95060_e147750_d_n7;
        locals.var_lover_func_dn8 = assign95060_e147750_d_n8;
        locals.var_lover_func_dn9 = assign95060_e147750_d_n9;
        locals.var_lover_func_dn10 = assign95060_e147750_d_n10;
        locals.var_lover_func_dn11 = assign95060_e147750_d_n11;
        locals.var_lover_func_dn14 = assign95060_e147750_d_n14;
        locals.var_lover_func_rv = 0.0;

        let (assign95070_e147756, assign95070_e147756_d_n0, assign95070_e147756_d_n2, assign95070_e147756_d_n4, assign95070_e147756_d_n5, assign95070_e147756_d_n6, assign95070_e147756_d_n7, assign95070_e147756_d_n8, assign95070_e147756_d_n9, assign95070_e147756_d_n10, assign95070_e147756_d_n11, assign95070_e147756_d_n14,) = {
    if ((locals.var_guard2115 != 0.0) && (locals.var_guard2116 != 0.0)) {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn8, locals.var_qbuld_dn9, locals.var_qbuld_dn10, locals.var_qbuld_dn11, locals.var_qbuld_dn14,)
    } else {
        (locals.var_rd_qbuld, locals.var_rd_qbuld_dn0, locals.var_rd_qbuld_dn2, locals.var_rd_qbuld_dn4, locals.var_rd_qbuld_dn5, locals.var_rd_qbuld_dn6, locals.var_rd_qbuld_dn7, locals.var_rd_qbuld_dn8, locals.var_rd_qbuld_dn9, locals.var_rd_qbuld_dn10, locals.var_rd_qbuld_dn11, locals.var_rd_qbuld_dn14,)
    }
};
        locals.var_rd_qbuld = assign95070_e147756;
        locals.var_rd_qbuld_dn0 = assign95070_e147756_d_n0;
        locals.var_rd_qbuld_dn2 = assign95070_e147756_d_n2;
        locals.var_rd_qbuld_dn4 = assign95070_e147756_d_n4;
        locals.var_rd_qbuld_dn5 = assign95070_e147756_d_n5;
        locals.var_rd_qbuld_dn6 = assign95070_e147756_d_n6;
        locals.var_rd_qbuld_dn7 = assign95070_e147756_d_n7;
        locals.var_rd_qbuld_dn8 = assign95070_e147756_d_n8;
        locals.var_rd_qbuld_dn9 = assign95070_e147756_d_n9;
        locals.var_rd_qbuld_dn10 = assign95070_e147756_d_n10;
        locals.var_rd_qbuld_dn11 = assign95070_e147756_d_n11;
        locals.var_rd_qbuld_dn14 = assign95070_e147756_d_n14;
        locals.var_rd_qbuld_rv = 0.0;

        let assign95080_e147767: f64 = if (((p.p36 == 1.0) && (p.p66 > 0.0)) && (locals.var_uc_novers == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2219 = assign95080_e147767;
        locals.var_guard2219_rv = 0.0;

        let (assign95090_e147771,) = {
    if (locals.var_guard2219 != 0.0) {
        (1.0,)
    } else {
        (locals.var_flg_coovlps,)
    }
};
        locals.var_flg_coovlps = assign95090_e147771;
        locals.var_flg_coovlps_rv = 0.0;

        let (assign95100_e147775,) = {
    if (locals.var_guard2219 != 0.0) {
        (locals.var_mks_ovslp,)
    } else {
        (locals.var_cov_slp,)
    }
};
        locals.var_cov_slp = assign95100_e147775;
        locals.var_cov_slp_rv = 0.0;

        let (assign95110_e147779,) = {
    if (locals.var_guard2219 != 0.0) {
        (p.p276,)
    } else {
        (locals.var_cov_mag,)
    }
};
        locals.var_cov_mag = assign95110_e147779;
        locals.var_cov_mag_rv = 0.0;

        let (assign95120_e147785, assign95120_e147785_d_n0, assign95120_e147785_d_n2, assign95120_e147785_d_n4, assign95120_e147785_d_n5, assign95120_e147785_d_n6, assign95120_e147785_d_n7, assign95120_e147785_d_n8, assign95120_e147785_d_n9, assign95120_e147785_d_n10, assign95120_e147785_d_n11, assign95120_e147785_d_n14,) = {
    if (locals.var_guard2219 != 0.0) {
        let assign95120_e147783: f64 = (locals.var_cox0 * locals.var_weffcv_nf);
        (assign95120_e147783, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign95120_e147785;
        locals.var_t1_dn0 = assign95120_e147785_d_n0;
        locals.var_t1_dn2 = assign95120_e147785_d_n2;
        locals.var_t1_dn4 = assign95120_e147785_d_n4;
        locals.var_t1_dn5 = assign95120_e147785_d_n5;
        locals.var_t1_dn6 = assign95120_e147785_d_n6;
        locals.var_t1_dn7 = assign95120_e147785_d_n7;
        locals.var_t1_dn8 = assign95120_e147785_d_n8;
        locals.var_t1_dn9 = assign95120_e147785_d_n9;
        locals.var_t1_dn10 = assign95120_e147785_d_n10;
        locals.var_t1_dn11 = assign95120_e147785_d_n11;
        locals.var_t1_dn14 = assign95120_e147785_d_n14;
        locals.var_t1_rv = 0.0;

        let assign95130_e147788: f64 = if locals.var_mode == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2220 = assign95130_e147788;
        locals.var_guard2220_rv = 0.0;

        let (assign95140_e147800, assign95140_e147800_d_n0, assign95140_e147800_d_n2, assign95140_e147800_d_n4, assign95140_e147800_d_n5, assign95140_e147800_d_n6, assign95140_e147800_d_n7, assign95140_e147800_d_n8, assign95140_e147800_d_n9, assign95140_e147800_d_n10, assign95140_e147800_d_n11, assign95140_e147800_d_n14,) = {
    if ((locals.var_guard2219 != 0.0) && (locals.var_guard2220 != 0.0)) {
        let assign95140_e147794: f64 = (locals.var_cov_slp * locals.var_t1);
        let assign95140_e147797: f64 = (locals.var_cov_mag + locals.var_vgs);
        let assign95140_e147798: f64 = (assign95140_e147794 * assign95140_e147797);
        (assign95140_e147798, ((locals.var_cov_slp * locals.var_t1_dn0) * assign95140_e147797), ((locals.var_cov_slp * locals.var_t1_dn2) * assign95140_e147797), ((locals.var_cov_slp * locals.var_t1_dn4) * assign95140_e147797), ((locals.var_cov_slp * locals.var_t1_dn5) * assign95140_e147797), (((locals.var_cov_slp * locals.var_t1_dn6) * assign95140_e147797) + (assign95140_e147794 * locals.var_vgs_dn6)), (((locals.var_cov_slp * locals.var_t1_dn7) * assign95140_e147797) + (assign95140_e147794 * locals.var_vgs_dn7)), (((locals.var_cov_slp * locals.var_t1_dn8) * assign95140_e147797) + (assign95140_e147794 * locals.var_vgs_dn8)), ((locals.var_cov_slp * locals.var_t1_dn9) * assign95140_e147797), ((locals.var_cov_slp * locals.var_t1_dn10) * assign95140_e147797), ((locals.var_cov_slp * locals.var_t1_dn11) * assign95140_e147797), ((locals.var_cov_slp * locals.var_t1_dn14) * assign95140_e147797),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign95140_e147800;
        locals.var_t4_dn0 = assign95140_e147800_d_n0;
        locals.var_t4_dn2 = assign95140_e147800_d_n2;
        locals.var_t4_dn4 = assign95140_e147800_d_n4;
        locals.var_t4_dn5 = assign95140_e147800_d_n5;
        locals.var_t4_dn6 = assign95140_e147800_d_n6;
        locals.var_t4_dn7 = assign95140_e147800_d_n7;
        locals.var_t4_dn8 = assign95140_e147800_d_n8;
        locals.var_t4_dn9 = assign95140_e147800_d_n9;
        locals.var_t4_dn10 = assign95140_e147800_d_n10;
        locals.var_t4_dn11 = assign95140_e147800_d_n11;
        locals.var_t4_dn14 = assign95140_e147800_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign95150_e147808, assign95150_e147808_d_n0, assign95150_e147808_d_n2, assign95150_e147808_d_n4, assign95150_e147808_d_n5, assign95150_e147808_d_n6, assign95150_e147808_d_n7, assign95150_e147808_d_n8, assign95150_e147808_d_n9, assign95150_e147808_d_n10, assign95150_e147808_d_n11, assign95150_e147808_d_n14,) = {
    if ((locals.var_guard2219 != 0.0) && (locals.var_guard2220 != 0.0)) {
        let assign95150_e147806: f64 = (p.p66 * locals.var_t1);
        (assign95150_e147806, (p.p66 * locals.var_t1_dn0), (p.p66 * locals.var_t1_dn2), (p.p66 * locals.var_t1_dn4), (p.p66 * locals.var_t1_dn5), (p.p66 * locals.var_t1_dn6), (p.p66 * locals.var_t1_dn7), (p.p66 * locals.var_t1_dn8), (p.p66 * locals.var_t1_dn9), (p.p66 * locals.var_t1_dn10), (p.p66 * locals.var_t1_dn11), (p.p66 * locals.var_t1_dn14),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign95150_e147808;
        locals.var_t5_dn0 = assign95150_e147808_d_n0;
        locals.var_t5_dn2 = assign95150_e147808_d_n2;
        locals.var_t5_dn4 = assign95150_e147808_d_n4;
        locals.var_t5_dn5 = assign95150_e147808_d_n5;
        locals.var_t5_dn6 = assign95150_e147808_d_n6;
        locals.var_t5_dn7 = assign95150_e147808_d_n7;
        locals.var_t5_dn8 = assign95150_e147808_d_n8;
        locals.var_t5_dn9 = assign95150_e147808_d_n9;
        locals.var_t5_dn10 = assign95150_e147808_d_n10;
        locals.var_t5_dn11 = assign95150_e147808_d_n11;
        locals.var_t5_dn14 = assign95150_e147808_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign95160_e147816, assign95160_e147816_d_n0, assign95160_e147816_d_n2, assign95160_e147816_d_n4, assign95160_e147816_d_n5, assign95160_e147816_d_n6, assign95160_e147816_d_n7, assign95160_e147816_d_n8, assign95160_e147816_d_n9, assign95160_e147816_d_n10, assign95160_e147816_d_n11, assign95160_e147816_d_n14,) = {
    if ((locals.var_guard2219 != 0.0) && (locals.var_guard2220 != 0.0)) {
        let assign95160_e147814: f64 = (1.2 - locals.var_ps0);
        (assign95160_e147814, (-locals.var_ps0_dn0), (-locals.var_ps0_dn2), (-locals.var_ps0_dn4), (-locals.var_ps0_dn5), (-locals.var_ps0_dn6), (-locals.var_ps0_dn7), (-locals.var_ps0_dn8), (-locals.var_ps0_dn9), (-locals.var_ps0_dn10), (-locals.var_ps0_dn11), (-locals.var_ps0_dn14),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign95160_e147816;
        locals.var_t9_dn0 = assign95160_e147816_d_n0;
        locals.var_t9_dn2 = assign95160_e147816_d_n2;
        locals.var_t9_dn4 = assign95160_e147816_d_n4;
        locals.var_t9_dn5 = assign95160_e147816_d_n5;
        locals.var_t9_dn6 = assign95160_e147816_d_n6;
        locals.var_t9_dn7 = assign95160_e147816_d_n7;
        locals.var_t9_dn8 = assign95160_e147816_d_n8;
        locals.var_t9_dn9 = assign95160_e147816_d_n9;
        locals.var_t9_dn10 = assign95160_e147816_d_n10;
        locals.var_t9_dn11 = assign95160_e147816_d_n11;
        locals.var_t9_dn14 = assign95160_e147816_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign95170_e147828, assign95170_e147828_d_n0, assign95170_e147828_d_n2, assign95170_e147828_d_n4, assign95170_e147828_d_n5, assign95170_e147828_d_n6, assign95170_e147828_d_n7, assign95170_e147828_d_n8, assign95170_e147828_d_n9, assign95170_e147828_d_n10, assign95170_e147828_d_n11, assign95170_e147828_d_n14,) = {
    if ((locals.var_guard2219 != 0.0) && (locals.var_guard2220 != 0.0)) {
        let assign95170_e147822: f64 = (locals.var_vgs * locals.var_t5);
        let assign95170_e147825: f64 = (locals.var_t4 * locals.var_t9);
        let assign95170_e147826: f64 = (assign95170_e147822 - assign95170_e147825);
        (assign95170_e147826, ((locals.var_vgs * locals.var_t5_dn0) - ((locals.var_t4_dn0 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn0))), ((locals.var_vgs * locals.var_t5_dn2) - ((locals.var_t4_dn2 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn2))), ((locals.var_vgs * locals.var_t5_dn4) - ((locals.var_t4_dn4 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn4))), ((locals.var_vgs * locals.var_t5_dn5) - ((locals.var_t4_dn5 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn5))), (((locals.var_vgs_dn6 * locals.var_t5) + (locals.var_vgs * locals.var_t5_dn6)) - ((locals.var_t4_dn6 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn6))), (((locals.var_vgs_dn7 * locals.var_t5) + (locals.var_vgs * locals.var_t5_dn7)) - ((locals.var_t4_dn7 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn7))), (((locals.var_vgs_dn8 * locals.var_t5) + (locals.var_vgs * locals.var_t5_dn8)) - ((locals.var_t4_dn8 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn8))), ((locals.var_vgs * locals.var_t5_dn9) - ((locals.var_t4_dn9 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn9))), ((locals.var_vgs * locals.var_t5_dn10) - ((locals.var_t4_dn10 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn10))), ((locals.var_vgs * locals.var_t5_dn11) - ((locals.var_t4_dn11 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn11))), ((locals.var_vgs * locals.var_t5_dn14) - ((locals.var_t4_dn14 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn14))),)
    } else {
        (locals.var_qgos, locals.var_qgos_dn0, locals.var_qgos_dn2, locals.var_qgos_dn4, locals.var_qgos_dn5, locals.var_qgos_dn6, locals.var_qgos_dn7, locals.var_qgos_dn8, locals.var_qgos_dn9, locals.var_qgos_dn10, locals.var_qgos_dn11, locals.var_qgos_dn14,)
    }
};
        locals.var_qgos = assign95170_e147828;
        locals.var_qgos_dn0 = assign95170_e147828_d_n0;
        locals.var_qgos_dn2 = assign95170_e147828_d_n2;
        locals.var_qgos_dn4 = assign95170_e147828_d_n4;
        locals.var_qgos_dn5 = assign95170_e147828_d_n5;
        locals.var_qgos_dn6 = assign95170_e147828_d_n6;
        locals.var_qgos_dn7 = assign95170_e147828_d_n7;
        locals.var_qgos_dn8 = assign95170_e147828_d_n8;
        locals.var_qgos_dn9 = assign95170_e147828_d_n9;
        locals.var_qgos_dn10 = assign95170_e147828_d_n10;
        locals.var_qgos_dn11 = assign95170_e147828_d_n11;
        locals.var_qgos_dn14 = assign95170_e147828_d_n14;
        locals.var_qgos_rv = 0.0;

        let (assign95180_e147843, assign95180_e147843_d_n0, assign95180_e147843_d_n2, assign95180_e147843_d_n4, assign95180_e147843_d_n5, assign95180_e147843_d_n6, assign95180_e147843_d_n7, assign95180_e147843_d_n8, assign95180_e147843_d_n9, assign95180_e147843_d_n10, assign95180_e147843_d_n11, assign95180_e147843_d_n14,) = {
    if ((locals.var_guard2219 != 0.0) && (locals.var_guard2220 == 0.0)) {
        let assign95180_e147835: f64 = (locals.var_cov_slp * locals.var_t1);
        let assign95180_e147838: f64 = (locals.var_cov_mag + locals.var_vgs);
        let assign95180_e147840: f64 = (assign95180_e147838 - locals.var_vds);
        let assign95180_e147841: f64 = (assign95180_e147835 * assign95180_e147840);
        (assign95180_e147841, (((locals.var_cov_slp * locals.var_t1_dn0) * assign95180_e147840) + (assign95180_e147835 * (-locals.var_vds_dn0))), (((locals.var_cov_slp * locals.var_t1_dn2) * assign95180_e147840) + (assign95180_e147835 * (-locals.var_vds_dn2))), (((locals.var_cov_slp * locals.var_t1_dn4) * assign95180_e147840) + (assign95180_e147835 * (-locals.var_vds_dn4))), (((locals.var_cov_slp * locals.var_t1_dn5) * assign95180_e147840) + (assign95180_e147835 * (-locals.var_vds_dn5))), (((locals.var_cov_slp * locals.var_t1_dn6) * assign95180_e147840) + (assign95180_e147835 * (locals.var_vgs_dn6 - locals.var_vds_dn6))), (((locals.var_cov_slp * locals.var_t1_dn7) * assign95180_e147840) + (assign95180_e147835 * (locals.var_vgs_dn7 - locals.var_vds_dn7))), (((locals.var_cov_slp * locals.var_t1_dn8) * assign95180_e147840) + (assign95180_e147835 * (locals.var_vgs_dn8 - locals.var_vds_dn8))), (((locals.var_cov_slp * locals.var_t1_dn9) * assign95180_e147840) + (assign95180_e147835 * (-locals.var_vds_dn9))), (((locals.var_cov_slp * locals.var_t1_dn10) * assign95180_e147840) + (assign95180_e147835 * (-locals.var_vds_dn10))), (((locals.var_cov_slp * locals.var_t1_dn11) * assign95180_e147840) + (assign95180_e147835 * (-locals.var_vds_dn11))), (((locals.var_cov_slp * locals.var_t1_dn14) * assign95180_e147840) + (assign95180_e147835 * (-locals.var_vds_dn14))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign95180_e147843;
        locals.var_t4_dn0 = assign95180_e147843_d_n0;
        locals.var_t4_dn2 = assign95180_e147843_d_n2;
        locals.var_t4_dn4 = assign95180_e147843_d_n4;
        locals.var_t4_dn5 = assign95180_e147843_d_n5;
        locals.var_t4_dn6 = assign95180_e147843_d_n6;
        locals.var_t4_dn7 = assign95180_e147843_d_n7;
        locals.var_t4_dn8 = assign95180_e147843_d_n8;
        locals.var_t4_dn9 = assign95180_e147843_d_n9;
        locals.var_t4_dn10 = assign95180_e147843_d_n10;
        locals.var_t4_dn11 = assign95180_e147843_d_n11;
        locals.var_t4_dn14 = assign95180_e147843_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign95190_e147852, assign95190_e147852_d_n0, assign95190_e147852_d_n2, assign95190_e147852_d_n4, assign95190_e147852_d_n5, assign95190_e147852_d_n6, assign95190_e147852_d_n7, assign95190_e147852_d_n8, assign95190_e147852_d_n9, assign95190_e147852_d_n10, assign95190_e147852_d_n11, assign95190_e147852_d_n14,) = {
    if ((locals.var_guard2219 != 0.0) && (locals.var_guard2220 == 0.0)) {
        let assign95190_e147850: f64 = (p.p66 * locals.var_t1);
        (assign95190_e147850, (p.p66 * locals.var_t1_dn0), (p.p66 * locals.var_t1_dn2), (p.p66 * locals.var_t1_dn4), (p.p66 * locals.var_t1_dn5), (p.p66 * locals.var_t1_dn6), (p.p66 * locals.var_t1_dn7), (p.p66 * locals.var_t1_dn8), (p.p66 * locals.var_t1_dn9), (p.p66 * locals.var_t1_dn10), (p.p66 * locals.var_t1_dn11), (p.p66 * locals.var_t1_dn14),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign95190_e147852;
        locals.var_t5_dn0 = assign95190_e147852_d_n0;
        locals.var_t5_dn2 = assign95190_e147852_d_n2;
        locals.var_t5_dn4 = assign95190_e147852_d_n4;
        locals.var_t5_dn5 = assign95190_e147852_d_n5;
        locals.var_t5_dn6 = assign95190_e147852_d_n6;
        locals.var_t5_dn7 = assign95190_e147852_d_n7;
        locals.var_t5_dn8 = assign95190_e147852_d_n8;
        locals.var_t5_dn9 = assign95190_e147852_d_n9;
        locals.var_t5_dn10 = assign95190_e147852_d_n10;
        locals.var_t5_dn11 = assign95190_e147852_d_n11;
        locals.var_t5_dn14 = assign95190_e147852_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign95200_e147863, assign95200_e147863_d_n0, assign95200_e147863_d_n2, assign95200_e147863_d_n4, assign95200_e147863_d_n5, assign95200_e147863_d_n6, assign95200_e147863_d_n7, assign95200_e147863_d_n8, assign95200_e147863_d_n9, assign95200_e147863_d_n10, assign95200_e147863_d_n11, assign95200_e147863_d_n14,) = {
    if ((locals.var_guard2219 != 0.0) && (locals.var_guard2220 == 0.0)) {
        let assign95200_e147859: f64 = (1.2 + locals.var_vds);
        let assign95200_e147861: f64 = (assign95200_e147859 - locals.var_psl);
        (assign95200_e147861, (locals.var_vds_dn0 - locals.var_psl_dn0), (locals.var_vds_dn2 - locals.var_psl_dn2), (locals.var_vds_dn4 - locals.var_psl_dn4), (locals.var_vds_dn5 - locals.var_psl_dn5), (locals.var_vds_dn6 - locals.var_psl_dn6), (locals.var_vds_dn7 - locals.var_psl_dn7), (locals.var_vds_dn8 - locals.var_psl_dn8), (locals.var_vds_dn9 - locals.var_psl_dn9), (locals.var_vds_dn10 - locals.var_psl_dn10), (locals.var_vds_dn11 - locals.var_psl_dn11), (locals.var_vds_dn14 - locals.var_psl_dn14),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign95200_e147863;
        locals.var_t9_dn0 = assign95200_e147863_d_n0;
        locals.var_t9_dn2 = assign95200_e147863_d_n2;
        locals.var_t9_dn4 = assign95200_e147863_d_n4;
        locals.var_t9_dn5 = assign95200_e147863_d_n5;
        locals.var_t9_dn6 = assign95200_e147863_d_n6;
        locals.var_t9_dn7 = assign95200_e147863_d_n7;
        locals.var_t9_dn8 = assign95200_e147863_d_n8;
        locals.var_t9_dn9 = assign95200_e147863_d_n9;
        locals.var_t9_dn10 = assign95200_e147863_d_n10;
        locals.var_t9_dn11 = assign95200_e147863_d_n11;
        locals.var_t9_dn14 = assign95200_e147863_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign95210_e147878, assign95210_e147878_d_n0, assign95210_e147878_d_n2, assign95210_e147878_d_n4, assign95210_e147878_d_n5, assign95210_e147878_d_n6, assign95210_e147878_d_n7, assign95210_e147878_d_n8, assign95210_e147878_d_n9, assign95210_e147878_d_n10, assign95210_e147878_d_n11, assign95210_e147878_d_n14,) = {
    if ((locals.var_guard2219 != 0.0) && (locals.var_guard2220 == 0.0)) {
        let assign95210_e147870: f64 = (locals.var_vgs - locals.var_vds);
        let assign95210_e147872: f64 = (assign95210_e147870 * locals.var_t5);
        let assign95210_e147875: f64 = (locals.var_t4 * locals.var_t9);
        let assign95210_e147876: f64 = (assign95210_e147872 - assign95210_e147875);
        (assign95210_e147876, ((((-locals.var_vds_dn0) * locals.var_t5) + (assign95210_e147870 * locals.var_t5_dn0)) - ((locals.var_t4_dn0 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn0))), ((((-locals.var_vds_dn2) * locals.var_t5) + (assign95210_e147870 * locals.var_t5_dn2)) - ((locals.var_t4_dn2 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn2))), ((((-locals.var_vds_dn4) * locals.var_t5) + (assign95210_e147870 * locals.var_t5_dn4)) - ((locals.var_t4_dn4 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn4))), ((((-locals.var_vds_dn5) * locals.var_t5) + (assign95210_e147870 * locals.var_t5_dn5)) - ((locals.var_t4_dn5 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn5))), ((((locals.var_vgs_dn6 - locals.var_vds_dn6) * locals.var_t5) + (assign95210_e147870 * locals.var_t5_dn6)) - ((locals.var_t4_dn6 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn6))), ((((locals.var_vgs_dn7 - locals.var_vds_dn7) * locals.var_t5) + (assign95210_e147870 * locals.var_t5_dn7)) - ((locals.var_t4_dn7 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn7))), ((((locals.var_vgs_dn8 - locals.var_vds_dn8) * locals.var_t5) + (assign95210_e147870 * locals.var_t5_dn8)) - ((locals.var_t4_dn8 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn8))), ((((-locals.var_vds_dn9) * locals.var_t5) + (assign95210_e147870 * locals.var_t5_dn9)) - ((locals.var_t4_dn9 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn9))), ((((-locals.var_vds_dn10) * locals.var_t5) + (assign95210_e147870 * locals.var_t5_dn10)) - ((locals.var_t4_dn10 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn10))), ((((-locals.var_vds_dn11) * locals.var_t5) + (assign95210_e147870 * locals.var_t5_dn11)) - ((locals.var_t4_dn11 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn11))), ((((-locals.var_vds_dn14) * locals.var_t5) + (assign95210_e147870 * locals.var_t5_dn14)) - ((locals.var_t4_dn14 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn14))),)
    } else {
        (locals.var_qgos, locals.var_qgos_dn0, locals.var_qgos_dn2, locals.var_qgos_dn4, locals.var_qgos_dn5, locals.var_qgos_dn6, locals.var_qgos_dn7, locals.var_qgos_dn8, locals.var_qgos_dn9, locals.var_qgos_dn10, locals.var_qgos_dn11, locals.var_qgos_dn14,)
    }
};
        locals.var_qgos = assign95210_e147878;
        locals.var_qgos_dn0 = assign95210_e147878_d_n0;
        locals.var_qgos_dn2 = assign95210_e147878_d_n2;
        locals.var_qgos_dn4 = assign95210_e147878_d_n4;
        locals.var_qgos_dn5 = assign95210_e147878_d_n5;
        locals.var_qgos_dn6 = assign95210_e147878_d_n6;
        locals.var_qgos_dn7 = assign95210_e147878_d_n7;
        locals.var_qgos_dn8 = assign95210_e147878_d_n8;
        locals.var_qgos_dn9 = assign95210_e147878_d_n9;
        locals.var_qgos_dn10 = assign95210_e147878_d_n10;
        locals.var_qgos_dn11 = assign95210_e147878_d_n11;
        locals.var_qgos_dn14 = assign95210_e147878_d_n14;
        locals.var_qgos_rv = 0.0;

        let assign95220_e147889: f64 = if (((p.p35 == 1.0) && (p.p63 > 0.0)) && (locals.var_uc_nover == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2221 = assign95220_e147889;
        locals.var_guard2221_rv = 0.0;

        let (assign95230_e147893,) = {
    if (locals.var_guard2221 != 0.0) {
        (1.0,)
    } else {
        (locals.var_flg_coovlp,)
    }
};
        locals.var_flg_coovlp = assign95230_e147893;
        locals.var_flg_coovlp_rv = 0.0;

        let (assign95240_e147897,) = {
    if (locals.var_guard2221 != 0.0) {
        (locals.var_mks_ovslp,)
    } else {
        (locals.var_cov_slp,)
    }
};
        locals.var_cov_slp = assign95240_e147897;
        locals.var_cov_slp_rv = 0.0;

        let (assign95250_e147901,) = {
    if (locals.var_guard2221 != 0.0) {
        (p.p276,)
    } else {
        (locals.var_cov_mag,)
    }
};
        locals.var_cov_mag = assign95250_e147901;
        locals.var_cov_mag_rv = 0.0;

        let (assign95260_e147907, assign95260_e147907_d_n0, assign95260_e147907_d_n2, assign95260_e147907_d_n4, assign95260_e147907_d_n5, assign95260_e147907_d_n6, assign95260_e147907_d_n7, assign95260_e147907_d_n8, assign95260_e147907_d_n9, assign95260_e147907_d_n10, assign95260_e147907_d_n11, assign95260_e147907_d_n14,) = {
    if (locals.var_guard2221 != 0.0) {
        let assign95260_e147905: f64 = (locals.var_coxb0 * locals.var_weffcv_nf);
        (assign95260_e147905, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign95260_e147907;
        locals.var_t1_dn0 = assign95260_e147907_d_n0;
        locals.var_t1_dn2 = assign95260_e147907_d_n2;
        locals.var_t1_dn4 = assign95260_e147907_d_n4;
        locals.var_t1_dn5 = assign95260_e147907_d_n5;
        locals.var_t1_dn6 = assign95260_e147907_d_n6;
        locals.var_t1_dn7 = assign95260_e147907_d_n7;
        locals.var_t1_dn8 = assign95260_e147907_d_n8;
        locals.var_t1_dn9 = assign95260_e147907_d_n9;
        locals.var_t1_dn10 = assign95260_e147907_d_n10;
        locals.var_t1_dn11 = assign95260_e147907_d_n11;
        locals.var_t1_dn14 = assign95260_e147907_d_n14;
        locals.var_t1_rv = 0.0;

        let assign95270_e147910: f64 = if locals.var_mode == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2222 = assign95270_e147910;
        locals.var_guard2222_rv = 0.0;

        let (assign95280_e147924, assign95280_e147924_d_n0, assign95280_e147924_d_n2, assign95280_e147924_d_n4, assign95280_e147924_d_n5, assign95280_e147924_d_n6, assign95280_e147924_d_n7, assign95280_e147924_d_n8, assign95280_e147924_d_n9, assign95280_e147924_d_n10, assign95280_e147924_d_n11, assign95280_e147924_d_n14,) = {
    if ((locals.var_guard2221 != 0.0) && (locals.var_guard2222 != 0.0)) {
        let assign95280_e147916: f64 = (locals.var_cov_slp * locals.var_t1);
        let assign95280_e147919: f64 = (locals.var_cov_mag + locals.var_vgs);
        let assign95280_e147921: f64 = (assign95280_e147919 - locals.var_vds);
        let assign95280_e147922: f64 = (assign95280_e147916 * assign95280_e147921);
        (assign95280_e147922, (((locals.var_cov_slp * locals.var_t1_dn0) * assign95280_e147921) + (assign95280_e147916 * (-locals.var_vds_dn0))), (((locals.var_cov_slp * locals.var_t1_dn2) * assign95280_e147921) + (assign95280_e147916 * (-locals.var_vds_dn2))), (((locals.var_cov_slp * locals.var_t1_dn4) * assign95280_e147921) + (assign95280_e147916 * (-locals.var_vds_dn4))), (((locals.var_cov_slp * locals.var_t1_dn5) * assign95280_e147921) + (assign95280_e147916 * (-locals.var_vds_dn5))), (((locals.var_cov_slp * locals.var_t1_dn6) * assign95280_e147921) + (assign95280_e147916 * (locals.var_vgs_dn6 - locals.var_vds_dn6))), (((locals.var_cov_slp * locals.var_t1_dn7) * assign95280_e147921) + (assign95280_e147916 * (locals.var_vgs_dn7 - locals.var_vds_dn7))), (((locals.var_cov_slp * locals.var_t1_dn8) * assign95280_e147921) + (assign95280_e147916 * (locals.var_vgs_dn8 - locals.var_vds_dn8))), (((locals.var_cov_slp * locals.var_t1_dn9) * assign95280_e147921) + (assign95280_e147916 * (-locals.var_vds_dn9))), (((locals.var_cov_slp * locals.var_t1_dn10) * assign95280_e147921) + (assign95280_e147916 * (-locals.var_vds_dn10))), (((locals.var_cov_slp * locals.var_t1_dn11) * assign95280_e147921) + (assign95280_e147916 * (-locals.var_vds_dn11))), (((locals.var_cov_slp * locals.var_t1_dn14) * assign95280_e147921) + (assign95280_e147916 * (-locals.var_vds_dn14))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign95280_e147924;
        locals.var_t4_dn0 = assign95280_e147924_d_n0;
        locals.var_t4_dn2 = assign95280_e147924_d_n2;
        locals.var_t4_dn4 = assign95280_e147924_d_n4;
        locals.var_t4_dn5 = assign95280_e147924_d_n5;
        locals.var_t4_dn6 = assign95280_e147924_d_n6;
        locals.var_t4_dn7 = assign95280_e147924_d_n7;
        locals.var_t4_dn8 = assign95280_e147924_d_n8;
        locals.var_t4_dn9 = assign95280_e147924_d_n9;
        locals.var_t4_dn10 = assign95280_e147924_d_n10;
        locals.var_t4_dn11 = assign95280_e147924_d_n11;
        locals.var_t4_dn14 = assign95280_e147924_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign95290_e147932, assign95290_e147932_d_n0, assign95290_e147932_d_n2, assign95290_e147932_d_n4, assign95290_e147932_d_n5, assign95290_e147932_d_n6, assign95290_e147932_d_n7, assign95290_e147932_d_n8, assign95290_e147932_d_n9, assign95290_e147932_d_n10, assign95290_e147932_d_n11, assign95290_e147932_d_n14,) = {
    if ((locals.var_guard2221 != 0.0) && (locals.var_guard2222 != 0.0)) {
        let assign95290_e147930: f64 = (p.p63 * locals.var_t1);
        (assign95290_e147930, (p.p63 * locals.var_t1_dn0), (p.p63 * locals.var_t1_dn2), (p.p63 * locals.var_t1_dn4), (p.p63 * locals.var_t1_dn5), (p.p63 * locals.var_t1_dn6), (p.p63 * locals.var_t1_dn7), (p.p63 * locals.var_t1_dn8), (p.p63 * locals.var_t1_dn9), (p.p63 * locals.var_t1_dn10), (p.p63 * locals.var_t1_dn11), (p.p63 * locals.var_t1_dn14),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign95290_e147932;
        locals.var_t5_dn0 = assign95290_e147932_d_n0;
        locals.var_t5_dn2 = assign95290_e147932_d_n2;
        locals.var_t5_dn4 = assign95290_e147932_d_n4;
        locals.var_t5_dn5 = assign95290_e147932_d_n5;
        locals.var_t5_dn6 = assign95290_e147932_d_n6;
        locals.var_t5_dn7 = assign95290_e147932_d_n7;
        locals.var_t5_dn8 = assign95290_e147932_d_n8;
        locals.var_t5_dn9 = assign95290_e147932_d_n9;
        locals.var_t5_dn10 = assign95290_e147932_d_n10;
        locals.var_t5_dn11 = assign95290_e147932_d_n11;
        locals.var_t5_dn14 = assign95290_e147932_d_n14;
        locals.var_t5_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_370(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign95300_e147942, assign95300_e147942_d_n0, assign95300_e147942_d_n2, assign95300_e147942_d_n4, assign95300_e147942_d_n5, assign95300_e147942_d_n6, assign95300_e147942_d_n7, assign95300_e147942_d_n8, assign95300_e147942_d_n9, assign95300_e147942_d_n10, assign95300_e147942_d_n11, assign95300_e147942_d_n14,) = {
    if ((locals.var_guard2221 != 0.0) && (locals.var_guard2222 != 0.0)) {
        let assign95300_e147938: f64 = (1.2 + locals.var_vds);
        let assign95300_e147940: f64 = (assign95300_e147938 - locals.var_psl);
        (assign95300_e147940, (locals.var_vds_dn0 - locals.var_psl_dn0), (locals.var_vds_dn2 - locals.var_psl_dn2), (locals.var_vds_dn4 - locals.var_psl_dn4), (locals.var_vds_dn5 - locals.var_psl_dn5), (locals.var_vds_dn6 - locals.var_psl_dn6), (locals.var_vds_dn7 - locals.var_psl_dn7), (locals.var_vds_dn8 - locals.var_psl_dn8), (locals.var_vds_dn9 - locals.var_psl_dn9), (locals.var_vds_dn10 - locals.var_psl_dn10), (locals.var_vds_dn11 - locals.var_psl_dn11), (locals.var_vds_dn14 - locals.var_psl_dn14),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign95300_e147942;
        locals.var_t9_dn0 = assign95300_e147942_d_n0;
        locals.var_t9_dn2 = assign95300_e147942_d_n2;
        locals.var_t9_dn4 = assign95300_e147942_d_n4;
        locals.var_t9_dn5 = assign95300_e147942_d_n5;
        locals.var_t9_dn6 = assign95300_e147942_d_n6;
        locals.var_t9_dn7 = assign95300_e147942_d_n7;
        locals.var_t9_dn8 = assign95300_e147942_d_n8;
        locals.var_t9_dn9 = assign95300_e147942_d_n9;
        locals.var_t9_dn10 = assign95300_e147942_d_n10;
        locals.var_t9_dn11 = assign95300_e147942_d_n11;
        locals.var_t9_dn14 = assign95300_e147942_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign95310_e147956, assign95310_e147956_d_n0, assign95310_e147956_d_n2, assign95310_e147956_d_n4, assign95310_e147956_d_n5, assign95310_e147956_d_n6, assign95310_e147956_d_n7, assign95310_e147956_d_n8, assign95310_e147956_d_n9, assign95310_e147956_d_n10, assign95310_e147956_d_n11, assign95310_e147956_d_n14,) = {
    if ((locals.var_guard2221 != 0.0) && (locals.var_guard2222 != 0.0)) {
        let assign95310_e147948: f64 = (locals.var_vgs - locals.var_vds);
        let assign95310_e147950: f64 = (assign95310_e147948 * locals.var_t5);
        let assign95310_e147953: f64 = (locals.var_t4 * locals.var_t9);
        let assign95310_e147954: f64 = (assign95310_e147950 - assign95310_e147953);
        (assign95310_e147954, ((((-locals.var_vds_dn0) * locals.var_t5) + (assign95310_e147948 * locals.var_t5_dn0)) - ((locals.var_t4_dn0 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn0))), ((((-locals.var_vds_dn2) * locals.var_t5) + (assign95310_e147948 * locals.var_t5_dn2)) - ((locals.var_t4_dn2 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn2))), ((((-locals.var_vds_dn4) * locals.var_t5) + (assign95310_e147948 * locals.var_t5_dn4)) - ((locals.var_t4_dn4 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn4))), ((((-locals.var_vds_dn5) * locals.var_t5) + (assign95310_e147948 * locals.var_t5_dn5)) - ((locals.var_t4_dn5 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn5))), ((((locals.var_vgs_dn6 - locals.var_vds_dn6) * locals.var_t5) + (assign95310_e147948 * locals.var_t5_dn6)) - ((locals.var_t4_dn6 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn6))), ((((locals.var_vgs_dn7 - locals.var_vds_dn7) * locals.var_t5) + (assign95310_e147948 * locals.var_t5_dn7)) - ((locals.var_t4_dn7 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn7))), ((((locals.var_vgs_dn8 - locals.var_vds_dn8) * locals.var_t5) + (assign95310_e147948 * locals.var_t5_dn8)) - ((locals.var_t4_dn8 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn8))), ((((-locals.var_vds_dn9) * locals.var_t5) + (assign95310_e147948 * locals.var_t5_dn9)) - ((locals.var_t4_dn9 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn9))), ((((-locals.var_vds_dn10) * locals.var_t5) + (assign95310_e147948 * locals.var_t5_dn10)) - ((locals.var_t4_dn10 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn10))), ((((-locals.var_vds_dn11) * locals.var_t5) + (assign95310_e147948 * locals.var_t5_dn11)) - ((locals.var_t4_dn11 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn11))), ((((-locals.var_vds_dn14) * locals.var_t5) + (assign95310_e147948 * locals.var_t5_dn14)) - ((locals.var_t4_dn14 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn14))),)
    } else {
        (locals.var_qgod, locals.var_qgod_dn0, locals.var_qgod_dn2, locals.var_qgod_dn4, locals.var_qgod_dn5, locals.var_qgod_dn6, locals.var_qgod_dn7, locals.var_qgod_dn8, locals.var_qgod_dn9, locals.var_qgod_dn10, locals.var_qgod_dn11, locals.var_qgod_dn14,)
    }
};
        locals.var_qgod = assign95310_e147956;
        locals.var_qgod_dn0 = assign95310_e147956_d_n0;
        locals.var_qgod_dn2 = assign95310_e147956_d_n2;
        locals.var_qgod_dn4 = assign95310_e147956_d_n4;
        locals.var_qgod_dn5 = assign95310_e147956_d_n5;
        locals.var_qgod_dn6 = assign95310_e147956_d_n6;
        locals.var_qgod_dn7 = assign95310_e147956_d_n7;
        locals.var_qgod_dn8 = assign95310_e147956_d_n8;
        locals.var_qgod_dn9 = assign95310_e147956_d_n9;
        locals.var_qgod_dn10 = assign95310_e147956_d_n10;
        locals.var_qgod_dn11 = assign95310_e147956_d_n11;
        locals.var_qgod_dn14 = assign95310_e147956_d_n14;
        locals.var_qgod_rv = 0.0;

        let (assign95320_e147969, assign95320_e147969_d_n0, assign95320_e147969_d_n2, assign95320_e147969_d_n4, assign95320_e147969_d_n5, assign95320_e147969_d_n6, assign95320_e147969_d_n7, assign95320_e147969_d_n8, assign95320_e147969_d_n9, assign95320_e147969_d_n10, assign95320_e147969_d_n11, assign95320_e147969_d_n14,) = {
    if ((locals.var_guard2221 != 0.0) && (locals.var_guard2222 == 0.0)) {
        let assign95320_e147963: f64 = (locals.var_cov_slp * locals.var_t1);
        let assign95320_e147966: f64 = (locals.var_cov_mag + locals.var_vgs);
        let assign95320_e147967: f64 = (assign95320_e147963 * assign95320_e147966);
        (assign95320_e147967, ((locals.var_cov_slp * locals.var_t1_dn0) * assign95320_e147966), ((locals.var_cov_slp * locals.var_t1_dn2) * assign95320_e147966), ((locals.var_cov_slp * locals.var_t1_dn4) * assign95320_e147966), ((locals.var_cov_slp * locals.var_t1_dn5) * assign95320_e147966), (((locals.var_cov_slp * locals.var_t1_dn6) * assign95320_e147966) + (assign95320_e147963 * locals.var_vgs_dn6)), (((locals.var_cov_slp * locals.var_t1_dn7) * assign95320_e147966) + (assign95320_e147963 * locals.var_vgs_dn7)), (((locals.var_cov_slp * locals.var_t1_dn8) * assign95320_e147966) + (assign95320_e147963 * locals.var_vgs_dn8)), ((locals.var_cov_slp * locals.var_t1_dn9) * assign95320_e147966), ((locals.var_cov_slp * locals.var_t1_dn10) * assign95320_e147966), ((locals.var_cov_slp * locals.var_t1_dn11) * assign95320_e147966), ((locals.var_cov_slp * locals.var_t1_dn14) * assign95320_e147966),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign95320_e147969;
        locals.var_t4_dn0 = assign95320_e147969_d_n0;
        locals.var_t4_dn2 = assign95320_e147969_d_n2;
        locals.var_t4_dn4 = assign95320_e147969_d_n4;
        locals.var_t4_dn5 = assign95320_e147969_d_n5;
        locals.var_t4_dn6 = assign95320_e147969_d_n6;
        locals.var_t4_dn7 = assign95320_e147969_d_n7;
        locals.var_t4_dn8 = assign95320_e147969_d_n8;
        locals.var_t4_dn9 = assign95320_e147969_d_n9;
        locals.var_t4_dn10 = assign95320_e147969_d_n10;
        locals.var_t4_dn11 = assign95320_e147969_d_n11;
        locals.var_t4_dn14 = assign95320_e147969_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign95330_e147978, assign95330_e147978_d_n0, assign95330_e147978_d_n2, assign95330_e147978_d_n4, assign95330_e147978_d_n5, assign95330_e147978_d_n6, assign95330_e147978_d_n7, assign95330_e147978_d_n8, assign95330_e147978_d_n9, assign95330_e147978_d_n10, assign95330_e147978_d_n11, assign95330_e147978_d_n14,) = {
    if ((locals.var_guard2221 != 0.0) && (locals.var_guard2222 == 0.0)) {
        let assign95330_e147976: f64 = (p.p63 * locals.var_t1);
        (assign95330_e147976, (p.p63 * locals.var_t1_dn0), (p.p63 * locals.var_t1_dn2), (p.p63 * locals.var_t1_dn4), (p.p63 * locals.var_t1_dn5), (p.p63 * locals.var_t1_dn6), (p.p63 * locals.var_t1_dn7), (p.p63 * locals.var_t1_dn8), (p.p63 * locals.var_t1_dn9), (p.p63 * locals.var_t1_dn10), (p.p63 * locals.var_t1_dn11), (p.p63 * locals.var_t1_dn14),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign95330_e147978;
        locals.var_t5_dn0 = assign95330_e147978_d_n0;
        locals.var_t5_dn2 = assign95330_e147978_d_n2;
        locals.var_t5_dn4 = assign95330_e147978_d_n4;
        locals.var_t5_dn5 = assign95330_e147978_d_n5;
        locals.var_t5_dn6 = assign95330_e147978_d_n6;
        locals.var_t5_dn7 = assign95330_e147978_d_n7;
        locals.var_t5_dn8 = assign95330_e147978_d_n8;
        locals.var_t5_dn9 = assign95330_e147978_d_n9;
        locals.var_t5_dn10 = assign95330_e147978_d_n10;
        locals.var_t5_dn11 = assign95330_e147978_d_n11;
        locals.var_t5_dn14 = assign95330_e147978_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign95340_e147987, assign95340_e147987_d_n0, assign95340_e147987_d_n2, assign95340_e147987_d_n4, assign95340_e147987_d_n5, assign95340_e147987_d_n6, assign95340_e147987_d_n7, assign95340_e147987_d_n8, assign95340_e147987_d_n9, assign95340_e147987_d_n10, assign95340_e147987_d_n11, assign95340_e147987_d_n14,) = {
    if ((locals.var_guard2221 != 0.0) && (locals.var_guard2222 == 0.0)) {
        let assign95340_e147985: f64 = (1.2 - locals.var_ps0);
        (assign95340_e147985, (-locals.var_ps0_dn0), (-locals.var_ps0_dn2), (-locals.var_ps0_dn4), (-locals.var_ps0_dn5), (-locals.var_ps0_dn6), (-locals.var_ps0_dn7), (-locals.var_ps0_dn8), (-locals.var_ps0_dn9), (-locals.var_ps0_dn10), (-locals.var_ps0_dn11), (-locals.var_ps0_dn14),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign95340_e147987;
        locals.var_t9_dn0 = assign95340_e147987_d_n0;
        locals.var_t9_dn2 = assign95340_e147987_d_n2;
        locals.var_t9_dn4 = assign95340_e147987_d_n4;
        locals.var_t9_dn5 = assign95340_e147987_d_n5;
        locals.var_t9_dn6 = assign95340_e147987_d_n6;
        locals.var_t9_dn7 = assign95340_e147987_d_n7;
        locals.var_t9_dn8 = assign95340_e147987_d_n8;
        locals.var_t9_dn9 = assign95340_e147987_d_n9;
        locals.var_t9_dn10 = assign95340_e147987_d_n10;
        locals.var_t9_dn11 = assign95340_e147987_d_n11;
        locals.var_t9_dn14 = assign95340_e147987_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign95350_e148000, assign95350_e148000_d_n0, assign95350_e148000_d_n2, assign95350_e148000_d_n4, assign95350_e148000_d_n5, assign95350_e148000_d_n6, assign95350_e148000_d_n7, assign95350_e148000_d_n8, assign95350_e148000_d_n9, assign95350_e148000_d_n10, assign95350_e148000_d_n11, assign95350_e148000_d_n14,) = {
    if ((locals.var_guard2221 != 0.0) && (locals.var_guard2222 == 0.0)) {
        let assign95350_e147994: f64 = (locals.var_vgs * locals.var_t5);
        let assign95350_e147997: f64 = (locals.var_t4 * locals.var_t9);
        let assign95350_e147998: f64 = (assign95350_e147994 - assign95350_e147997);
        (assign95350_e147998, ((locals.var_vgs * locals.var_t5_dn0) - ((locals.var_t4_dn0 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn0))), ((locals.var_vgs * locals.var_t5_dn2) - ((locals.var_t4_dn2 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn2))), ((locals.var_vgs * locals.var_t5_dn4) - ((locals.var_t4_dn4 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn4))), ((locals.var_vgs * locals.var_t5_dn5) - ((locals.var_t4_dn5 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn5))), (((locals.var_vgs_dn6 * locals.var_t5) + (locals.var_vgs * locals.var_t5_dn6)) - ((locals.var_t4_dn6 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn6))), (((locals.var_vgs_dn7 * locals.var_t5) + (locals.var_vgs * locals.var_t5_dn7)) - ((locals.var_t4_dn7 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn7))), (((locals.var_vgs_dn8 * locals.var_t5) + (locals.var_vgs * locals.var_t5_dn8)) - ((locals.var_t4_dn8 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn8))), ((locals.var_vgs * locals.var_t5_dn9) - ((locals.var_t4_dn9 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn9))), ((locals.var_vgs * locals.var_t5_dn10) - ((locals.var_t4_dn10 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn10))), ((locals.var_vgs * locals.var_t5_dn11) - ((locals.var_t4_dn11 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn11))), ((locals.var_vgs * locals.var_t5_dn14) - ((locals.var_t4_dn14 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn14))),)
    } else {
        (locals.var_qgod, locals.var_qgod_dn0, locals.var_qgod_dn2, locals.var_qgod_dn4, locals.var_qgod_dn5, locals.var_qgod_dn6, locals.var_qgod_dn7, locals.var_qgod_dn8, locals.var_qgod_dn9, locals.var_qgod_dn10, locals.var_qgod_dn11, locals.var_qgod_dn14,)
    }
};
        locals.var_qgod = assign95350_e148000;
        locals.var_qgod_dn0 = assign95350_e148000_d_n0;
        locals.var_qgod_dn2 = assign95350_e148000_d_n2;
        locals.var_qgod_dn4 = assign95350_e148000_d_n4;
        locals.var_qgod_dn5 = assign95350_e148000_d_n5;
        locals.var_qgod_dn6 = assign95350_e148000_d_n6;
        locals.var_qgod_dn7 = assign95350_e148000_d_n7;
        locals.var_qgod_dn8 = assign95350_e148000_d_n8;
        locals.var_qgod_dn9 = assign95350_e148000_d_n9;
        locals.var_qgod_dn10 = assign95350_e148000_d_n10;
        locals.var_qgod_dn11 = assign95350_e148000_d_n11;
        locals.var_qgod_dn14 = assign95350_e148000_d_n14;
        locals.var_qgod_rv = 0.0;

        let (assign95360_e148007,) = {
    if (locals.var_cgso_given != 0.0) {
        let assign95360_e148004: f64 = (-locals.var_weffcv_nf);
        let assign95360_e148005: f64 = (locals.var_uc_cgso * assign95360_e148004);
        (assign95360_e148005,)
    } else {
        (locals.var_cgsoe,)
    }
};
        locals.var_cgsoe = assign95360_e148007;
        locals.var_cgsoe_rv = 0.0;

        let assign95370_e148010: f64 = if locals.var_flg_coovlps == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2223 = assign95370_e148010;
        locals.var_guard2223_rv = 0.0;

        let (assign95380_e148022,) = {
    if ((locals.var_cgso_given == 0.0) && (locals.var_guard2223 != 0.0)) {
        let assign95380_e148016: f64 = (-locals.var_cox0);
        let assign95380_e148018: f64 = (assign95380_e148016 * p.p66);
        let assign95380_e148020: f64 = (assign95380_e148018 * locals.var_weffcv_nf);
        (assign95380_e148020,)
    } else {
        (locals.var_cgsoe,)
    }
};
        locals.var_cgsoe = assign95380_e148022;
        locals.var_cgsoe_rv = 0.0;

        let assign95390_e148024: f64 = (-locals.var_cgsoe);
        let assign95390_e148026: f64 = (assign95390_e148024 * locals.var_vgsei);
        locals.var_qgso = assign95390_e148026;
        locals.var_qgso_dn2 = (assign95390_e148024 * locals.var_vgsei_dn2);
        locals.var_qgso_dn7 = (assign95390_e148024 * locals.var_vgsei_dn7);
        locals.var_qgso_rv = 0.0;

        let (assign95400_e148033,) = {
    if (locals.var_cgdo_given != 0.0) {
        let assign95400_e148030: f64 = (-locals.var_weffcv_nf);
        let assign95400_e148031: f64 = (locals.var_uc_cgdo * assign95400_e148030);
        (assign95400_e148031,)
    } else {
        (locals.var_cgdoe,)
    }
};
        locals.var_cgdoe = assign95400_e148033;
        locals.var_cgdoe_rv = 0.0;

        let assign95410_e148036: f64 = if locals.var_flg_coovlp == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2224 = assign95410_e148036;
        locals.var_guard2224_rv = 0.0;

        let (assign95420_e148048,) = {
    if ((locals.var_cgdo_given == 0.0) && (locals.var_guard2224 != 0.0)) {
        let assign95420_e148042: f64 = (-locals.var_coxb0);
        let assign95420_e148044: f64 = (assign95420_e148042 * p.p63);
        let assign95420_e148046: f64 = (assign95420_e148044 * locals.var_weffcv_nf);
        (assign95420_e148046,)
    } else {
        (locals.var_cgdoe,)
    }
};
        locals.var_cgdoe = assign95420_e148048;
        locals.var_cgdoe_rv = 0.0;

        let assign95430_e148050: f64 = (-locals.var_cgdoe);
        let assign95430_e148053: f64 = (locals.var_vgsei - locals.var_vdsei);
        let assign95430_e148054: f64 = (assign95430_e148050 * assign95430_e148053);
        locals.var_qgdo = assign95430_e148054;
        locals.var_qgdo_dn0 = (assign95430_e148050 * (-locals.var_vdsei_dn0));
        locals.var_qgdo_dn2 = (assign95430_e148050 * (locals.var_vgsei_dn2 - locals.var_vdsei_dn2));
        locals.var_qgdo_dn7 = (assign95430_e148050 * locals.var_vgsei_dn7);
        locals.var_qgdo_rv = 0.0;

        let assign95440_e148057: f64 = if locals.var_mode == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2225 = assign95440_e148057;
        locals.var_guard2225_rv = 0.0;

        let (assign95450_e148065, assign95450_e148065_d_n0, assign95450_e148065_d_n2, assign95450_e148065_d_n4, assign95450_e148065_d_n5, assign95450_e148065_d_n6, assign95450_e148065_d_n7, assign95450_e148065_d_n8, assign95450_e148065_d_n9, assign95450_e148065_d_n10, assign95450_e148065_d_n11, assign95450_e148065_d_n14,) = {
    if (locals.var_guard2225 != 0.0) {
        let assign95450_e148062: f64 = (locals.var_vds - locals.var_pds);
        let assign95450_e148063: f64 = (p.p431 * assign95450_e148062);
        (assign95450_e148063, (p.p431 * (locals.var_vds_dn0 - locals.var_pds_dn0)), (p.p431 * (locals.var_vds_dn2 - locals.var_pds_dn2)), (p.p431 * (locals.var_vds_dn4 - locals.var_pds_dn4)), (p.p431 * (locals.var_vds_dn5 - locals.var_pds_dn5)), (p.p431 * (locals.var_vds_dn6 - locals.var_pds_dn6)), (p.p431 * (locals.var_vds_dn7 - locals.var_pds_dn7)), (p.p431 * (locals.var_vds_dn8 - locals.var_pds_dn8)), (p.p431 * (locals.var_vds_dn9 - locals.var_pds_dn9)), (p.p431 * (locals.var_vds_dn10 - locals.var_pds_dn10)), (p.p431 * (locals.var_vds_dn11 - locals.var_pds_dn11)), (p.p431 * (locals.var_vds_dn14 - locals.var_pds_dn14)),)
    } else {
        (locals.var_qodad, locals.var_qodad_dn0, locals.var_qodad_dn2, locals.var_qodad_dn4, locals.var_qodad_dn5, locals.var_qodad_dn6, locals.var_qodad_dn7, locals.var_qodad_dn8, locals.var_qodad_dn9, locals.var_qodad_dn10, locals.var_qodad_dn11, locals.var_qodad_dn14,)
    }
};
        locals.var_qodad = assign95450_e148065;
        locals.var_qodad_dn0 = assign95450_e148065_d_n0;
        locals.var_qodad_dn2 = assign95450_e148065_d_n2;
        locals.var_qodad_dn4 = assign95450_e148065_d_n4;
        locals.var_qodad_dn5 = assign95450_e148065_d_n5;
        locals.var_qodad_dn6 = assign95450_e148065_d_n6;
        locals.var_qodad_dn7 = assign95450_e148065_d_n7;
        locals.var_qodad_dn8 = assign95450_e148065_d_n8;
        locals.var_qodad_dn9 = assign95450_e148065_d_n9;
        locals.var_qodad_dn10 = assign95450_e148065_d_n10;
        locals.var_qodad_dn11 = assign95450_e148065_d_n11;
        locals.var_qodad_dn14 = assign95450_e148065_d_n14;
        locals.var_qodad_rv = 0.0;

        let (assign95460_e148071, assign95460_e148071_d_n0, assign95460_e148071_d_n2, assign95460_e148071_d_n4, assign95460_e148071_d_n5, assign95460_e148071_d_n6, assign95460_e148071_d_n7, assign95460_e148071_d_n8, assign95460_e148071_d_n9, assign95460_e148071_d_n10, assign95460_e148071_d_n11, assign95460_e148071_d_n14,) = {
    if (locals.var_guard2225 != 0.0) {
        let assign95460_e148069: f64 = (locals.var_t4 * locals.var_qodad);
        (assign95460_e148069, ((locals.var_t4_dn0 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn0)), ((locals.var_t4_dn2 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn2)), ((locals.var_t4_dn4 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn4)), ((locals.var_t4_dn5 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn5)), ((locals.var_t4_dn6 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn6)), ((locals.var_t4_dn7 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn7)), ((locals.var_t4_dn8 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn8)), ((locals.var_t4_dn9 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn9)), ((locals.var_t4_dn10 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn10)), ((locals.var_t4_dn11 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn11)), ((locals.var_t4_dn14 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn14)),)
    } else {
        (locals.var_qovd_add, locals.var_qovd_add_dn0, locals.var_qovd_add_dn2, locals.var_qovd_add_dn4, locals.var_qovd_add_dn5, locals.var_qovd_add_dn6, locals.var_qovd_add_dn7, locals.var_qovd_add_dn8, locals.var_qovd_add_dn9, locals.var_qovd_add_dn10, locals.var_qovd_add_dn11, locals.var_qovd_add_dn14,)
    }
};
        locals.var_qovd_add = assign95460_e148071;
        locals.var_qovd_add_dn0 = assign95460_e148071_d_n0;
        locals.var_qovd_add_dn2 = assign95460_e148071_d_n2;
        locals.var_qovd_add_dn4 = assign95460_e148071_d_n4;
        locals.var_qovd_add_dn5 = assign95460_e148071_d_n5;
        locals.var_qovd_add_dn6 = assign95460_e148071_d_n6;
        locals.var_qovd_add_dn7 = assign95460_e148071_d_n7;
        locals.var_qovd_add_dn8 = assign95460_e148071_d_n8;
        locals.var_qovd_add_dn9 = assign95460_e148071_d_n9;
        locals.var_qovd_add_dn10 = assign95460_e148071_d_n10;
        locals.var_qovd_add_dn11 = assign95460_e148071_d_n11;
        locals.var_qovd_add_dn14 = assign95460_e148071_d_n14;
        locals.var_qovd_add_rv = 0.0;

        let (assign95470_e148077, assign95470_e148077_d_n0, assign95470_e148077_d_n2, assign95470_e148077_d_n4, assign95470_e148077_d_n5, assign95470_e148077_d_n6, assign95470_e148077_d_n7, assign95470_e148077_d_n8, assign95470_e148077_d_n9, assign95470_e148077_d_n10, assign95470_e148077_d_n11, assign95470_e148077_d_n14,) = {
    if (locals.var_guard2225 != 0.0) {
        let assign95470_e148075: f64 = (locals.var_t4 * locals.var_qodad);
        (assign95470_e148075, ((locals.var_t4_dn0 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn0)), ((locals.var_t4_dn2 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn2)), ((locals.var_t4_dn4 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn4)), ((locals.var_t4_dn5 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn5)), ((locals.var_t4_dn6 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn6)), ((locals.var_t4_dn7 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn7)), ((locals.var_t4_dn8 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn8)), ((locals.var_t4_dn9 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn9)), ((locals.var_t4_dn10 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn10)), ((locals.var_t4_dn11 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn11)), ((locals.var_t4_dn14 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn14)),)
    } else {
        (locals.var_qbdld_add, locals.var_qbdld_add_dn0, locals.var_qbdld_add_dn2, locals.var_qbdld_add_dn4, locals.var_qbdld_add_dn5, locals.var_qbdld_add_dn6, locals.var_qbdld_add_dn7, locals.var_qbdld_add_dn8, locals.var_qbdld_add_dn9, locals.var_qbdld_add_dn10, locals.var_qbdld_add_dn11, locals.var_qbdld_add_dn14,)
    }
};
        locals.var_qbdld_add = assign95470_e148077;
        locals.var_qbdld_add_dn0 = assign95470_e148077_d_n0;
        locals.var_qbdld_add_dn2 = assign95470_e148077_d_n2;
        locals.var_qbdld_add_dn4 = assign95470_e148077_d_n4;
        locals.var_qbdld_add_dn5 = assign95470_e148077_d_n5;
        locals.var_qbdld_add_dn6 = assign95470_e148077_d_n6;
        locals.var_qbdld_add_dn7 = assign95470_e148077_d_n7;
        locals.var_qbdld_add_dn8 = assign95470_e148077_d_n8;
        locals.var_qbdld_add_dn9 = assign95470_e148077_d_n9;
        locals.var_qbdld_add_dn10 = assign95470_e148077_d_n10;
        locals.var_qbdld_add_dn11 = assign95470_e148077_d_n11;
        locals.var_qbdld_add_dn14 = assign95470_e148077_d_n14;
        locals.var_qbdld_add_rv = 0.0;

        let (assign95480_e148087, assign95480_e148087_d_n0, assign95480_e148087_d_n2, assign95480_e148087_d_n4, assign95480_e148087_d_n5, assign95480_e148087_d_n6, assign95480_e148087_d_n7, assign95480_e148087_d_n8, assign95480_e148087_d_n9, assign95480_e148087_d_n10, assign95480_e148087_d_n11, assign95480_e148087_d_n14,) = {
    if (locals.var_guard2225 == 0.0) {
        let assign95480_e148081: f64 = (-p.p431);
        let assign95480_e148084: f64 = (locals.var_vds - locals.var_pds);
        let assign95480_e148085: f64 = (assign95480_e148081 * assign95480_e148084);
        (assign95480_e148085, (assign95480_e148081 * (locals.var_vds_dn0 - locals.var_pds_dn0)), (assign95480_e148081 * (locals.var_vds_dn2 - locals.var_pds_dn2)), (assign95480_e148081 * (locals.var_vds_dn4 - locals.var_pds_dn4)), (assign95480_e148081 * (locals.var_vds_dn5 - locals.var_pds_dn5)), (assign95480_e148081 * (locals.var_vds_dn6 - locals.var_pds_dn6)), (assign95480_e148081 * (locals.var_vds_dn7 - locals.var_pds_dn7)), (assign95480_e148081 * (locals.var_vds_dn8 - locals.var_pds_dn8)), (assign95480_e148081 * (locals.var_vds_dn9 - locals.var_pds_dn9)), (assign95480_e148081 * (locals.var_vds_dn10 - locals.var_pds_dn10)), (assign95480_e148081 * (locals.var_vds_dn11 - locals.var_pds_dn11)), (assign95480_e148081 * (locals.var_vds_dn14 - locals.var_pds_dn14)),)
    } else {
        (locals.var_qodad, locals.var_qodad_dn0, locals.var_qodad_dn2, locals.var_qodad_dn4, locals.var_qodad_dn5, locals.var_qodad_dn6, locals.var_qodad_dn7, locals.var_qodad_dn8, locals.var_qodad_dn9, locals.var_qodad_dn10, locals.var_qodad_dn11, locals.var_qodad_dn14,)
    }
};
        locals.var_qodad = assign95480_e148087;
        locals.var_qodad_dn0 = assign95480_e148087_d_n0;
        locals.var_qodad_dn2 = assign95480_e148087_d_n2;
        locals.var_qodad_dn4 = assign95480_e148087_d_n4;
        locals.var_qodad_dn5 = assign95480_e148087_d_n5;
        locals.var_qodad_dn6 = assign95480_e148087_d_n6;
        locals.var_qodad_dn7 = assign95480_e148087_d_n7;
        locals.var_qodad_dn8 = assign95480_e148087_d_n8;
        locals.var_qodad_dn9 = assign95480_e148087_d_n9;
        locals.var_qodad_dn10 = assign95480_e148087_d_n10;
        locals.var_qodad_dn11 = assign95480_e148087_d_n11;
        locals.var_qodad_dn14 = assign95480_e148087_d_n14;
        locals.var_qodad_rv = 0.0;

        let (assign95490_e148094, assign95490_e148094_d_n0, assign95490_e148094_d_n2, assign95490_e148094_d_n4, assign95490_e148094_d_n5, assign95490_e148094_d_n6, assign95490_e148094_d_n7, assign95490_e148094_d_n8, assign95490_e148094_d_n9, assign95490_e148094_d_n10, assign95490_e148094_d_n11, assign95490_e148094_d_n14,) = {
    if (locals.var_guard2225 == 0.0) {
        let assign95490_e148092: f64 = (locals.var_t4 * locals.var_qodad);
        (assign95490_e148092, ((locals.var_t4_dn0 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn0)), ((locals.var_t4_dn2 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn2)), ((locals.var_t4_dn4 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn4)), ((locals.var_t4_dn5 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn5)), ((locals.var_t4_dn6 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn6)), ((locals.var_t4_dn7 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn7)), ((locals.var_t4_dn8 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn8)), ((locals.var_t4_dn9 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn9)), ((locals.var_t4_dn10 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn10)), ((locals.var_t4_dn11 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn11)), ((locals.var_t4_dn14 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn14)),)
    } else {
        (locals.var_qovs_add, locals.var_qovs_add_dn0, locals.var_qovs_add_dn2, locals.var_qovs_add_dn4, locals.var_qovs_add_dn5, locals.var_qovs_add_dn6, locals.var_qovs_add_dn7, locals.var_qovs_add_dn8, locals.var_qovs_add_dn9, locals.var_qovs_add_dn10, locals.var_qovs_add_dn11, locals.var_qovs_add_dn14,)
    }
};
        locals.var_qovs_add = assign95490_e148094;
        locals.var_qovs_add_dn0 = assign95490_e148094_d_n0;
        locals.var_qovs_add_dn2 = assign95490_e148094_d_n2;
        locals.var_qovs_add_dn4 = assign95490_e148094_d_n4;
        locals.var_qovs_add_dn5 = assign95490_e148094_d_n5;
        locals.var_qovs_add_dn6 = assign95490_e148094_d_n6;
        locals.var_qovs_add_dn7 = assign95490_e148094_d_n7;
        locals.var_qovs_add_dn8 = assign95490_e148094_d_n8;
        locals.var_qovs_add_dn9 = assign95490_e148094_d_n9;
        locals.var_qovs_add_dn10 = assign95490_e148094_d_n10;
        locals.var_qovs_add_dn11 = assign95490_e148094_d_n11;
        locals.var_qovs_add_dn14 = assign95490_e148094_d_n14;
        locals.var_qovs_add_rv = 0.0;

        let (assign95500_e148101, assign95500_e148101_d_n0, assign95500_e148101_d_n2, assign95500_e148101_d_n4, assign95500_e148101_d_n5, assign95500_e148101_d_n6, assign95500_e148101_d_n7, assign95500_e148101_d_n8, assign95500_e148101_d_n9, assign95500_e148101_d_n10, assign95500_e148101_d_n11, assign95500_e148101_d_n14,) = {
    if (locals.var_guard2225 == 0.0) {
        let assign95500_e148099: f64 = (locals.var_t4 * locals.var_qodad);
        (assign95500_e148099, ((locals.var_t4_dn0 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn0)), ((locals.var_t4_dn2 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn2)), ((locals.var_t4_dn4 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn4)), ((locals.var_t4_dn5 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn5)), ((locals.var_t4_dn6 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn6)), ((locals.var_t4_dn7 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn7)), ((locals.var_t4_dn8 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn8)), ((locals.var_t4_dn9 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn9)), ((locals.var_t4_dn10 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn10)), ((locals.var_t4_dn11 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn11)), ((locals.var_t4_dn14 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn14)),)
    } else {
        (locals.var_qbsld_add, locals.var_qbsld_add_dn0, locals.var_qbsld_add_dn2, locals.var_qbsld_add_dn4, locals.var_qbsld_add_dn5, locals.var_qbsld_add_dn6, locals.var_qbsld_add_dn7, locals.var_qbsld_add_dn8, locals.var_qbsld_add_dn9, locals.var_qbsld_add_dn10, locals.var_qbsld_add_dn11, locals.var_qbsld_add_dn14,)
    }
};
        locals.var_qbsld_add = assign95500_e148101;
        locals.var_qbsld_add_dn0 = assign95500_e148101_d_n0;
        locals.var_qbsld_add_dn2 = assign95500_e148101_d_n2;
        locals.var_qbsld_add_dn4 = assign95500_e148101_d_n4;
        locals.var_qbsld_add_dn5 = assign95500_e148101_d_n5;
        locals.var_qbsld_add_dn6 = assign95500_e148101_d_n6;
        locals.var_qbsld_add_dn7 = assign95500_e148101_d_n7;
        locals.var_qbsld_add_dn8 = assign95500_e148101_d_n8;
        locals.var_qbsld_add_dn9 = assign95500_e148101_d_n9;
        locals.var_qbsld_add_dn10 = assign95500_e148101_d_n10;
        locals.var_qbsld_add_dn11 = assign95500_e148101_d_n11;
        locals.var_qbsld_add_dn14 = assign95500_e148101_d_n14;
        locals.var_qbsld_add_rv = 0.0;

        let assign95510_e148103: f64 = (-locals.var_uc_cgbo);
        let assign95510_e148105: f64 = (assign95510_e148103 * locals.var_lgate);
        locals.var_cgbo_loc = assign95510_e148105;
        locals.var_cgbo_loc_rv = 0.0;

        let assign95520_e148107: f64 = (-locals.var_cgbo_loc);
        let assign95520_e148110: f64 = (locals.var_vgsi - locals.var_vbsi);
        let assign95520_e148111: f64 = (assign95520_e148107 * assign95520_e148110);
        locals.var_qgbo = assign95520_e148111;
        locals.var_qgbo_dn7 = (assign95520_e148107 * locals.var_vgsi_dn7);
        locals.var_qgbo_dn8 = (assign95520_e148107 * (locals.var_vgsi_dn8 - locals.var_vbsi_dn8));
        locals.var_qgbo_dn9 = (assign95520_e148107 * (-locals.var_vbsi_dn9));
        locals.var_qgbo_rv = 0.0;

        locals.var_aclm = locals.var_uc_clm1;
        locals.var_aclm_rv = 0.0;

        let assign95540_e148115: f64 = if locals.var_flg_noqi != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2226 = assign95540_e148115;
        locals.var_guard2226_rv = 0.0;

        let (assign95550_e148129, assign95550_e148129_d_n0, assign95550_e148129_d_n2, assign95550_e148129_d_n4, assign95550_e148129_d_n5, assign95550_e148129_d_n6, assign95550_e148129_d_n7, assign95550_e148129_d_n8, assign95550_e148129_d_n9, assign95550_e148129_d_n10, assign95550_e148129_d_n11, assign95550_e148129_d_n14,) = {
    if (locals.var_guard2226 != 0.0) {
        let assign95550_e148120: f64 = (locals.var_vds + locals.var_ps0);
        let assign95550_e148121: f64 = (locals.var_aclm * assign95550_e148120);
        let assign95550_e148124: f64 = (1.0 - locals.var_aclm);
        let assign95550_e148126: f64 = (assign95550_e148124 * locals.var_psl);
        let assign95550_e148127: f64 = (assign95550_e148121 + assign95550_e148126);
        (assign95550_e148127, ((locals.var_aclm * (locals.var_vds_dn0 + locals.var_ps0_dn0)) + (assign95550_e148124 * locals.var_psl_dn0)), ((locals.var_aclm * (locals.var_vds_dn2 + locals.var_ps0_dn2)) + (assign95550_e148124 * locals.var_psl_dn2)), ((locals.var_aclm * (locals.var_vds_dn4 + locals.var_ps0_dn4)) + (assign95550_e148124 * locals.var_psl_dn4)), ((locals.var_aclm * (locals.var_vds_dn5 + locals.var_ps0_dn5)) + (assign95550_e148124 * locals.var_psl_dn5)), ((locals.var_aclm * (locals.var_vds_dn6 + locals.var_ps0_dn6)) + (assign95550_e148124 * locals.var_psl_dn6)), ((locals.var_aclm * (locals.var_vds_dn7 + locals.var_ps0_dn7)) + (assign95550_e148124 * locals.var_psl_dn7)), ((locals.var_aclm * (locals.var_vds_dn8 + locals.var_ps0_dn8)) + (assign95550_e148124 * locals.var_psl_dn8)), ((locals.var_aclm * (locals.var_vds_dn9 + locals.var_ps0_dn9)) + (assign95550_e148124 * locals.var_psl_dn9)), ((locals.var_aclm * (locals.var_vds_dn10 + locals.var_ps0_dn10)) + (assign95550_e148124 * locals.var_psl_dn10)), ((locals.var_aclm * (locals.var_vds_dn11 + locals.var_ps0_dn11)) + (assign95550_e148124 * locals.var_psl_dn11)), ((locals.var_aclm * (locals.var_vds_dn14 + locals.var_ps0_dn14)) + (assign95550_e148124 * locals.var_psl_dn14)),)
    } else {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn4, locals.var_psdl_dn5, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn8, locals.var_psdl_dn9, locals.var_psdl_dn10, locals.var_psdl_dn11, locals.var_psdl_dn14,)
    }
};
        locals.var_psdl = assign95550_e148129;
        locals.var_psdl_dn0 = assign95550_e148129_d_n0;
        locals.var_psdl_dn2 = assign95550_e148129_d_n2;
        locals.var_psdl_dn4 = assign95550_e148129_d_n4;
        locals.var_psdl_dn5 = assign95550_e148129_d_n5;
        locals.var_psdl_dn6 = assign95550_e148129_d_n6;
        locals.var_psdl_dn7 = assign95550_e148129_d_n7;
        locals.var_psdl_dn8 = assign95550_e148129_d_n8;
        locals.var_psdl_dn9 = assign95550_e148129_d_n9;
        locals.var_psdl_dn10 = assign95550_e148129_d_n10;
        locals.var_psdl_dn11 = assign95550_e148129_d_n11;
        locals.var_psdl_dn14 = assign95550_e148129_d_n14;
        locals.var_psdl_rv = 0.0;

        let assign95560_e148133: f64 = (locals.var_ps0 + locals.var_vds);
        let assign95560_e148136: f64 = (10.0 * 2.220446049250313e-16);
        let assign95560_e148137: f64 = (assign95560_e148133 - assign95560_e148136);
        let assign95560_e148140: f64 = (10.0 * 2.220446049250313e-16);
        let assign95560_e148141: f64 = (assign95560_e148137 - assign95560_e148140);
        let assign95560_e148145: f64 = (10.0 * 2.220446049250313e-16);
        let assign95560_e148148: f64 = if ((locals.var_psdl > assign95560_e148141) && (assign95560_e148145 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2227 = assign95560_e148148;
        locals.var_guard2227_rv = 0.0;

        let (assign95570_e148166, assign95570_e148166_d_n0, assign95570_e148166_d_n2, assign95570_e148166_d_n4, assign95570_e148166_d_n5, assign95570_e148166_d_n6, assign95570_e148166_d_n7, assign95570_e148166_d_n8, assign95570_e148166_d_n9, assign95570_e148166_d_n10, assign95570_e148166_d_n11, assign95570_e148166_d_n14,) = {
    if ((locals.var_guard2226 != 0.0) && (locals.var_guard2227 != 0.0)) {
        let assign95570_e148155: f64 = (locals.var_ps0 + locals.var_vds);
        let assign95570_e148158: f64 = (10.0 * 2.220446049250313e-16);
        let assign95570_e148159: f64 = (assign95570_e148155 - assign95570_e148158);
        let assign95570_e148160: f64 = (locals.var_psdl - assign95570_e148159);
        let assign95570_e148163: f64 = (10.0 * 2.220446049250313e-16);
        let assign95570_e148164: f64 = (assign95570_e148160 + assign95570_e148163);
        (assign95570_e148164, (locals.var_psdl_dn0 - (locals.var_ps0_dn0 + locals.var_vds_dn0)), (locals.var_psdl_dn2 - (locals.var_ps0_dn2 + locals.var_vds_dn2)), (locals.var_psdl_dn4 - (locals.var_ps0_dn4 + locals.var_vds_dn4)), (locals.var_psdl_dn5 - (locals.var_ps0_dn5 + locals.var_vds_dn5)), (locals.var_psdl_dn6 - (locals.var_ps0_dn6 + locals.var_vds_dn6)), (locals.var_psdl_dn7 - (locals.var_ps0_dn7 + locals.var_vds_dn7)), (locals.var_psdl_dn8 - (locals.var_ps0_dn8 + locals.var_vds_dn8)), (locals.var_psdl_dn9 - (locals.var_ps0_dn9 + locals.var_vds_dn9)), (locals.var_psdl_dn10 - (locals.var_ps0_dn10 + locals.var_vds_dn10)), (locals.var_psdl_dn11 - (locals.var_ps0_dn11 + locals.var_vds_dn11)), (locals.var_psdl_dn14 - (locals.var_ps0_dn14 + locals.var_vds_dn14)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign95570_e148166;
        locals.var_tmf1_dn0 = assign95570_e148166_d_n0;
        locals.var_tmf1_dn2 = assign95570_e148166_d_n2;
        locals.var_tmf1_dn4 = assign95570_e148166_d_n4;
        locals.var_tmf1_dn5 = assign95570_e148166_d_n5;
        locals.var_tmf1_dn6 = assign95570_e148166_d_n6;
        locals.var_tmf1_dn7 = assign95570_e148166_d_n7;
        locals.var_tmf1_dn8 = assign95570_e148166_d_n8;
        locals.var_tmf1_dn9 = assign95570_e148166_d_n9;
        locals.var_tmf1_dn10 = assign95570_e148166_d_n10;
        locals.var_tmf1_dn11 = assign95570_e148166_d_n11;
        locals.var_tmf1_dn14 = assign95570_e148166_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign95580_e148174, assign95580_e148174_d_n0, assign95580_e148174_d_n2, assign95580_e148174_d_n4, assign95580_e148174_d_n5, assign95580_e148174_d_n6, assign95580_e148174_d_n7, assign95580_e148174_d_n8, assign95580_e148174_d_n9, assign95580_e148174_d_n10, assign95580_e148174_d_n11, assign95580_e148174_d_n14,) = {
    if ((locals.var_guard2226 != 0.0) && (locals.var_guard2227 != 0.0)) {
        let assign95580_e148172: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign95580_e148172, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign95580_e148174;
        locals.var_x2_dn0 = assign95580_e148174_d_n0;
        locals.var_x2_dn2 = assign95580_e148174_d_n2;
        locals.var_x2_dn4 = assign95580_e148174_d_n4;
        locals.var_x2_dn5 = assign95580_e148174_d_n5;
        locals.var_x2_dn6 = assign95580_e148174_d_n6;
        locals.var_x2_dn7 = assign95580_e148174_d_n7;
        locals.var_x2_dn8 = assign95580_e148174_d_n8;
        locals.var_x2_dn9 = assign95580_e148174_d_n9;
        locals.var_x2_dn10 = assign95580_e148174_d_n10;
        locals.var_x2_dn11 = assign95580_e148174_d_n11;
        locals.var_x2_dn14 = assign95580_e148174_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign95590_e148186, assign95590_e148186_d_n0, assign95590_e148186_d_n2, assign95590_e148186_d_n4, assign95590_e148186_d_n5, assign95590_e148186_d_n6, assign95590_e148186_d_n7, assign95590_e148186_d_n8, assign95590_e148186_d_n9, assign95590_e148186_d_n10, assign95590_e148186_d_n11, assign95590_e148186_d_n14,) = {
    if ((locals.var_guard2226 != 0.0) && (locals.var_guard2227 != 0.0)) {
        let assign95590_e148180: f64 = (10.0 * 2.220446049250313e-16);
        let assign95590_e148183: f64 = (10.0 * 2.220446049250313e-16);
        let assign95590_e148184: f64 = (assign95590_e148180 * assign95590_e148183);
        (assign95590_e148184, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign95590_e148186;
        locals.var_xmax2_dn0 = assign95590_e148186_d_n0;
        locals.var_xmax2_dn2 = assign95590_e148186_d_n2;
        locals.var_xmax2_dn4 = assign95590_e148186_d_n4;
        locals.var_xmax2_dn5 = assign95590_e148186_d_n5;
        locals.var_xmax2_dn6 = assign95590_e148186_d_n6;
        locals.var_xmax2_dn7 = assign95590_e148186_d_n7;
        locals.var_xmax2_dn8 = assign95590_e148186_d_n8;
        locals.var_xmax2_dn9 = assign95590_e148186_d_n9;
        locals.var_xmax2_dn10 = assign95590_e148186_d_n10;
        locals.var_xmax2_dn11 = assign95590_e148186_d_n11;
        locals.var_xmax2_dn14 = assign95590_e148186_d_n14;
        locals.var_xmax2_rv = 0.0;

        let (assign95600_e148192, assign95600_e148192_d_n0, assign95600_e148192_d_n2, assign95600_e148192_d_n4, assign95600_e148192_d_n5, assign95600_e148192_d_n6, assign95600_e148192_d_n7, assign95600_e148192_d_n8, assign95600_e148192_d_n9, assign95600_e148192_d_n10, assign95600_e148192_d_n11, assign95600_e148192_d_n14,) = {
    if ((locals.var_guard2226 != 0.0) && (locals.var_guard2227 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign95600_e148192;
        locals.var_xp_dn0 = assign95600_e148192_d_n0;
        locals.var_xp_dn2 = assign95600_e148192_d_n2;
        locals.var_xp_dn4 = assign95600_e148192_d_n4;
        locals.var_xp_dn5 = assign95600_e148192_d_n5;
        locals.var_xp_dn6 = assign95600_e148192_d_n6;
        locals.var_xp_dn7 = assign95600_e148192_d_n7;
        locals.var_xp_dn8 = assign95600_e148192_d_n8;
        locals.var_xp_dn9 = assign95600_e148192_d_n9;
        locals.var_xp_dn10 = assign95600_e148192_d_n10;
        locals.var_xp_dn11 = assign95600_e148192_d_n11;
        locals.var_xp_dn14 = assign95600_e148192_d_n14;
        locals.var_xp_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_371(
        locals: &mut StampLocals,
    ) {
        let (assign95610_e148198, assign95610_e148198_d_n0, assign95610_e148198_d_n2, assign95610_e148198_d_n4, assign95610_e148198_d_n5, assign95610_e148198_d_n6, assign95610_e148198_d_n7, assign95610_e148198_d_n8, assign95610_e148198_d_n9, assign95610_e148198_d_n10, assign95610_e148198_d_n11, assign95610_e148198_d_n14,) = {
    if ((locals.var_guard2226 != 0.0) && (locals.var_guard2227 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign95610_e148198;
        locals.var_xmp_dn0 = assign95610_e148198_d_n0;
        locals.var_xmp_dn2 = assign95610_e148198_d_n2;
        locals.var_xmp_dn4 = assign95610_e148198_d_n4;
        locals.var_xmp_dn5 = assign95610_e148198_d_n5;
        locals.var_xmp_dn6 = assign95610_e148198_d_n6;
        locals.var_xmp_dn7 = assign95610_e148198_d_n7;
        locals.var_xmp_dn8 = assign95610_e148198_d_n8;
        locals.var_xmp_dn9 = assign95610_e148198_d_n9;
        locals.var_xmp_dn10 = assign95610_e148198_d_n10;
        locals.var_xmp_dn11 = assign95610_e148198_d_n11;
        locals.var_xmp_dn14 = assign95610_e148198_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign95620_e148204,) = {
    if ((locals.var_guard2226 != 0.0) && (locals.var_guard2227 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign95620_e148204;
        locals.var_m0_rv = 0.0;

        let (assign95630_e148210,) = {
    if ((locals.var_guard2226 != 0.0) && (locals.var_guard2227 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign95630_e148210;
        locals.var_mm_rv = 0.0;

        let (assign95640_e148216, assign95640_e148216_d_n0, assign95640_e148216_d_n2, assign95640_e148216_d_n4, assign95640_e148216_d_n5, assign95640_e148216_d_n6, assign95640_e148216_d_n7, assign95640_e148216_d_n8, assign95640_e148216_d_n9, assign95640_e148216_d_n10, assign95640_e148216_d_n11, assign95640_e148216_d_n14,) = {
    if ((locals.var_guard2226 != 0.0) && (locals.var_guard2227 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign95640_e148216;
        locals.var_arg_dn0 = assign95640_e148216_d_n0;
        locals.var_arg_dn2 = assign95640_e148216_d_n2;
        locals.var_arg_dn4 = assign95640_e148216_d_n4;
        locals.var_arg_dn5 = assign95640_e148216_d_n5;
        locals.var_arg_dn6 = assign95640_e148216_d_n6;
        locals.var_arg_dn7 = assign95640_e148216_d_n7;
        locals.var_arg_dn8 = assign95640_e148216_d_n8;
        locals.var_arg_dn9 = assign95640_e148216_d_n9;
        locals.var_arg_dn10 = assign95640_e148216_d_n10;
        locals.var_arg_dn11 = assign95640_e148216_d_n11;
        locals.var_arg_dn14 = assign95640_e148216_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign95650_e148222, assign95650_e148222_d_n0, assign95650_e148222_d_n2, assign95650_e148222_d_n4, assign95650_e148222_d_n5, assign95650_e148222_d_n6, assign95650_e148222_d_n7, assign95650_e148222_d_n8, assign95650_e148222_d_n9, assign95650_e148222_d_n10, assign95650_e148222_d_n11, assign95650_e148222_d_n14,) = {
    if ((locals.var_guard2226 != 0.0) && (locals.var_guard2227 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign95650_e148222;
        locals.var_dnm_dn0 = assign95650_e148222_d_n0;
        locals.var_dnm_dn2 = assign95650_e148222_d_n2;
        locals.var_dnm_dn4 = assign95650_e148222_d_n4;
        locals.var_dnm_dn5 = assign95650_e148222_d_n5;
        locals.var_dnm_dn6 = assign95650_e148222_d_n6;
        locals.var_dnm_dn7 = assign95650_e148222_d_n7;
        locals.var_dnm_dn8 = assign95650_e148222_d_n8;
        locals.var_dnm_dn9 = assign95650_e148222_d_n9;
        locals.var_dnm_dn10 = assign95650_e148222_d_n10;
        locals.var_dnm_dn11 = assign95650_e148222_d_n11;
        locals.var_dnm_dn14 = assign95650_e148222_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign95660_e148230, assign95660_e148230_d_n0, assign95660_e148230_d_n2, assign95660_e148230_d_n4, assign95660_e148230_d_n5, assign95660_e148230_d_n6, assign95660_e148230_d_n7, assign95660_e148230_d_n8, assign95660_e148230_d_n9, assign95660_e148230_d_n10, assign95660_e148230_d_n11, assign95660_e148230_d_n14,) = {
    if ((locals.var_guard2226 != 0.0) && (locals.var_guard2227 != 0.0)) {
        let assign95660_e148228: f64 = (locals.var_xp * locals.var_x2);
        (assign95660_e148228, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign95660_e148230;
        locals.var_xp_dn0 = assign95660_e148230_d_n0;
        locals.var_xp_dn2 = assign95660_e148230_d_n2;
        locals.var_xp_dn4 = assign95660_e148230_d_n4;
        locals.var_xp_dn5 = assign95660_e148230_d_n5;
        locals.var_xp_dn6 = assign95660_e148230_d_n6;
        locals.var_xp_dn7 = assign95660_e148230_d_n7;
        locals.var_xp_dn8 = assign95660_e148230_d_n8;
        locals.var_xp_dn9 = assign95660_e148230_d_n9;
        locals.var_xp_dn10 = assign95660_e148230_d_n10;
        locals.var_xp_dn11 = assign95660_e148230_d_n11;
        locals.var_xp_dn14 = assign95660_e148230_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign95670_e148238, assign95670_e148238_d_n0, assign95670_e148238_d_n2, assign95670_e148238_d_n4, assign95670_e148238_d_n5, assign95670_e148238_d_n6, assign95670_e148238_d_n7, assign95670_e148238_d_n8, assign95670_e148238_d_n9, assign95670_e148238_d_n10, assign95670_e148238_d_n11, assign95670_e148238_d_n14,) = {
    if ((locals.var_guard2226 != 0.0) && (locals.var_guard2227 != 0.0)) {
        let assign95670_e148236: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign95670_e148236, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign95670_e148238;
        locals.var_xmp_dn0 = assign95670_e148238_d_n0;
        locals.var_xmp_dn2 = assign95670_e148238_d_n2;
        locals.var_xmp_dn4 = assign95670_e148238_d_n4;
        locals.var_xmp_dn5 = assign95670_e148238_d_n5;
        locals.var_xmp_dn6 = assign95670_e148238_d_n6;
        locals.var_xmp_dn7 = assign95670_e148238_d_n7;
        locals.var_xmp_dn8 = assign95670_e148238_d_n8;
        locals.var_xmp_dn9 = assign95670_e148238_d_n9;
        locals.var_xmp_dn10 = assign95670_e148238_d_n10;
        locals.var_xmp_dn11 = assign95670_e148238_d_n11;
        locals.var_xmp_dn14 = assign95670_e148238_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign95680_e148246, assign95680_e148246_d_n0, assign95680_e148246_d_n2, assign95680_e148246_d_n4, assign95680_e148246_d_n5, assign95680_e148246_d_n6, assign95680_e148246_d_n7, assign95680_e148246_d_n8, assign95680_e148246_d_n9, assign95680_e148246_d_n10, assign95680_e148246_d_n11, assign95680_e148246_d_n14,) = {
    if ((locals.var_guard2226 != 0.0) && (locals.var_guard2227 != 0.0)) {
        let assign95680_e148244: f64 = (locals.var_xp * locals.var_x2);
        (assign95680_e148244, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign95680_e148246;
        locals.var_xp_dn0 = assign95680_e148246_d_n0;
        locals.var_xp_dn2 = assign95680_e148246_d_n2;
        locals.var_xp_dn4 = assign95680_e148246_d_n4;
        locals.var_xp_dn5 = assign95680_e148246_d_n5;
        locals.var_xp_dn6 = assign95680_e148246_d_n6;
        locals.var_xp_dn7 = assign95680_e148246_d_n7;
        locals.var_xp_dn8 = assign95680_e148246_d_n8;
        locals.var_xp_dn9 = assign95680_e148246_d_n9;
        locals.var_xp_dn10 = assign95680_e148246_d_n10;
        locals.var_xp_dn11 = assign95680_e148246_d_n11;
        locals.var_xp_dn14 = assign95680_e148246_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign95690_e148254, assign95690_e148254_d_n0, assign95690_e148254_d_n2, assign95690_e148254_d_n4, assign95690_e148254_d_n5, assign95690_e148254_d_n6, assign95690_e148254_d_n7, assign95690_e148254_d_n8, assign95690_e148254_d_n9, assign95690_e148254_d_n10, assign95690_e148254_d_n11, assign95690_e148254_d_n14,) = {
    if ((locals.var_guard2226 != 0.0) && (locals.var_guard2227 != 0.0)) {
        let assign95690_e148252: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign95690_e148252, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign95690_e148254;
        locals.var_xmp_dn0 = assign95690_e148254_d_n0;
        locals.var_xmp_dn2 = assign95690_e148254_d_n2;
        locals.var_xmp_dn4 = assign95690_e148254_d_n4;
        locals.var_xmp_dn5 = assign95690_e148254_d_n5;
        locals.var_xmp_dn6 = assign95690_e148254_d_n6;
        locals.var_xmp_dn7 = assign95690_e148254_d_n7;
        locals.var_xmp_dn8 = assign95690_e148254_d_n8;
        locals.var_xmp_dn9 = assign95690_e148254_d_n9;
        locals.var_xmp_dn10 = assign95690_e148254_d_n10;
        locals.var_xmp_dn11 = assign95690_e148254_d_n11;
        locals.var_xmp_dn14 = assign95690_e148254_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign95700_e148262, assign95700_e148262_d_n0, assign95700_e148262_d_n2, assign95700_e148262_d_n4, assign95700_e148262_d_n5, assign95700_e148262_d_n6, assign95700_e148262_d_n7, assign95700_e148262_d_n8, assign95700_e148262_d_n9, assign95700_e148262_d_n10, assign95700_e148262_d_n11, assign95700_e148262_d_n14,) = {
    if ((locals.var_guard2226 != 0.0) && (locals.var_guard2227 != 0.0)) {
        let assign95700_e148260: f64 = (locals.var_xp + locals.var_xmp);
        (assign95700_e148260, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign95700_e148262;
        locals.var_arg_dn0 = assign95700_e148262_d_n0;
        locals.var_arg_dn2 = assign95700_e148262_d_n2;
        locals.var_arg_dn4 = assign95700_e148262_d_n4;
        locals.var_arg_dn5 = assign95700_e148262_d_n5;
        locals.var_arg_dn6 = assign95700_e148262_d_n6;
        locals.var_arg_dn7 = assign95700_e148262_d_n7;
        locals.var_arg_dn8 = assign95700_e148262_d_n8;
        locals.var_arg_dn9 = assign95700_e148262_d_n9;
        locals.var_arg_dn10 = assign95700_e148262_d_n10;
        locals.var_arg_dn11 = assign95700_e148262_d_n11;
        locals.var_arg_dn14 = assign95700_e148262_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign95710_e148268, assign95710_e148268_d_n0, assign95710_e148268_d_n2, assign95710_e148268_d_n4, assign95710_e148268_d_n5, assign95710_e148268_d_n6, assign95710_e148268_d_n7, assign95710_e148268_d_n8, assign95710_e148268_d_n9, assign95710_e148268_d_n10, assign95710_e148268_d_n11, assign95710_e148268_d_n14,) = {
    if ((locals.var_guard2226 != 0.0) && (locals.var_guard2227 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign95710_e148268;
        locals.var_dnm_dn0 = assign95710_e148268_d_n0;
        locals.var_dnm_dn2 = assign95710_e148268_d_n2;
        locals.var_dnm_dn4 = assign95710_e148268_d_n4;
        locals.var_dnm_dn5 = assign95710_e148268_d_n5;
        locals.var_dnm_dn6 = assign95710_e148268_d_n6;
        locals.var_dnm_dn7 = assign95710_e148268_d_n7;
        locals.var_dnm_dn8 = assign95710_e148268_d_n8;
        locals.var_dnm_dn9 = assign95710_e148268_d_n9;
        locals.var_dnm_dn10 = assign95710_e148268_d_n10;
        locals.var_dnm_dn11 = assign95710_e148268_d_n11;
        locals.var_dnm_dn14 = assign95710_e148268_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign95720_e148283: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard2228 = assign95720_e148283;
        locals.var_guard2228_rv = 0.0;

        let assign95730_e148286: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2229 = assign95730_e148286;
        locals.var_guard2229_rv = 0.0;

        let (assign95740_e148296,) = {
    if ((((locals.var_guard2226 != 0.0) && (locals.var_guard2227 != 0.0)) && (locals.var_guard2228 != 0.0)) && (locals.var_guard2229 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign95740_e148296;
        locals.var_mm_rv = 0.0;

        let assign95750_e148299: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard2230 = assign95750_e148299;
        locals.var_guard2230_rv = 0.0;

        let (assign95760_e148312,) = {
    if (((((locals.var_guard2226 != 0.0) && (locals.var_guard2227 != 0.0)) && (locals.var_guard2228 != 0.0)) && (locals.var_guard2229 == 0.0)) && (locals.var_guard2230 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign95760_e148312;
        locals.var_mm_rv = 0.0;

        let assign95770_e148315: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard2231 = assign95770_e148315;
        locals.var_guard2231_rv = 0.0;

        let (assign95780_e148331,) = {
    if ((((((locals.var_guard2226 != 0.0) && (locals.var_guard2227 != 0.0)) && (locals.var_guard2228 != 0.0)) && (locals.var_guard2229 == 0.0)) && (locals.var_guard2230 == 0.0)) && (locals.var_guard2231 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign95780_e148331;
        locals.var_mm_rv = 0.0;

        let assign95790_e148334: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard2232 = assign95790_e148334;
        locals.var_guard2232_rv = 0.0;

        let (assign95800_e148353,) = {
    if (((((((locals.var_guard2226 != 0.0) && (locals.var_guard2227 != 0.0)) && (locals.var_guard2228 != 0.0)) && (locals.var_guard2229 == 0.0)) && (locals.var_guard2230 == 0.0)) && (locals.var_guard2231 == 0.0)) && (locals.var_guard2232 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign95800_e148353;
        locals.var_mm_rv = 0.0;

        let (assign95810_e148361,) = {
    if (((locals.var_guard2226 != 0.0) && (locals.var_guard2227 != 0.0)) && (locals.var_guard2228 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign95810_e148361;
        locals.var_m0_rv = 0.0;

        let mut assign95820_loop_guard: usize = 0;
        while {
            let assign95820_cond_e148370: f64 = if ((((locals.var_guard2226 != 0.0) && (locals.var_guard2227 != 0.0)) && (locals.var_guard2228 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign95820_cond_e148370 != 0.0
        } {
            assign95820_loop_guard += 1;
            assert!(assign95820_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign95820_body0_e148379, assign95820_body0_e148379_d_n0, assign95820_body0_e148379_d_n2, assign95820_body0_e148379_d_n4, assign95820_body0_e148379_d_n5, assign95820_body0_e148379_d_n6, assign95820_body0_e148379_d_n7, assign95820_body0_e148379_d_n8, assign95820_body0_e148379_d_n9, assign95820_body0_e148379_d_n10, assign95820_body0_e148379_d_n11, assign95820_body0_e148379_d_n14,) = {
    if (((locals.var_guard2226 != 0.0) && (locals.var_guard2227 != 0.0)) && (locals.var_guard2228 != 0.0)) {
        let assign95820_body0_e148377: f64 = (locals.var_dnm).sqrt();
        (assign95820_body0_e148377, (locals.var_dnm_dn0 / (2.0 * assign95820_body0_e148377)), (locals.var_dnm_dn2 / (2.0 * assign95820_body0_e148377)), (locals.var_dnm_dn4 / (2.0 * assign95820_body0_e148377)), (locals.var_dnm_dn5 / (2.0 * assign95820_body0_e148377)), (locals.var_dnm_dn6 / (2.0 * assign95820_body0_e148377)), (locals.var_dnm_dn7 / (2.0 * assign95820_body0_e148377)), (locals.var_dnm_dn8 / (2.0 * assign95820_body0_e148377)), (locals.var_dnm_dn9 / (2.0 * assign95820_body0_e148377)), (locals.var_dnm_dn10 / (2.0 * assign95820_body0_e148377)), (locals.var_dnm_dn11 / (2.0 * assign95820_body0_e148377)), (locals.var_dnm_dn14 / (2.0 * assign95820_body0_e148377)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign95820_body0_e148379;
            locals.var_dnm_dn0 = assign95820_body0_e148379_d_n0;
            locals.var_dnm_dn2 = assign95820_body0_e148379_d_n2;
            locals.var_dnm_dn4 = assign95820_body0_e148379_d_n4;
            locals.var_dnm_dn5 = assign95820_body0_e148379_d_n5;
            locals.var_dnm_dn6 = assign95820_body0_e148379_d_n6;
            locals.var_dnm_dn7 = assign95820_body0_e148379_d_n7;
            locals.var_dnm_dn8 = assign95820_body0_e148379_d_n8;
            locals.var_dnm_dn9 = assign95820_body0_e148379_d_n9;
            locals.var_dnm_dn10 = assign95820_body0_e148379_d_n10;
            locals.var_dnm_dn11 = assign95820_body0_e148379_d_n11;
            locals.var_dnm_dn14 = assign95820_body0_e148379_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign95820_body1_e148389,) = {
    if (((locals.var_guard2226 != 0.0) && (locals.var_guard2227 != 0.0)) && (locals.var_guard2228 != 0.0)) {
        let assign95820_body1_e148387: f64 = (locals.var_m0 + 1.0);
        (assign95820_body1_e148387,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign95820_body1_e148389;
            locals.var_m0_rv = 0.0;
        }

        let (assign95830_e148409, assign95830_e148409_d_n0, assign95830_e148409_d_n2, assign95830_e148409_d_n4, assign95830_e148409_d_n5, assign95830_e148409_d_n6, assign95830_e148409_d_n7, assign95830_e148409_d_n8, assign95830_e148409_d_n9, assign95830_e148409_d_n10, assign95830_e148409_d_n11, assign95830_e148409_d_n14,) = {
    if (((locals.var_guard2226 != 0.0) && (locals.var_guard2227 != 0.0)) && (locals.var_guard2228 == 0.0)) {
        let (assign95830_e148407, assign95830_e148407_d_n0, assign95830_e148407_d_n2, assign95830_e148407_d_n4, assign95830_e148407_d_n5, assign95830_e148407_d_n6, assign95830_e148407_d_n7, assign95830_e148407_d_n8, assign95830_e148407_d_n9, assign95830_e148407_d_n10, assign95830_e148407_d_n11, assign95830_e148407_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign95830_e148404: f64 = (2.0 * 2.0);
                let assign95830_e148405: f64 = (1.0 / assign95830_e148404);
                let assign95830_e148406: f64 = (locals.var_dnm).powf(assign95830_e148405);
                (assign95830_e148406, if 0.0 == 0.0 && ((assign95830_e148405) as f64).is_finite() && ((assign95830_e148405) as f64).fract() == 0.0 { if assign95830_e148405 == 0.0 { 0.0 } else { (assign95830_e148405 * ((locals.var_dnm).powf(assign95830_e148405 - 1.0) * locals.var_dnm_dn0)) } } else { (assign95830_e148406 * (assign95830_e148405 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign95830_e148405) as f64).is_finite() && ((assign95830_e148405) as f64).fract() == 0.0 { if assign95830_e148405 == 0.0 { 0.0 } else { (assign95830_e148405 * ((locals.var_dnm).powf(assign95830_e148405 - 1.0) * locals.var_dnm_dn2)) } } else { (assign95830_e148406 * (assign95830_e148405 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign95830_e148405) as f64).is_finite() && ((assign95830_e148405) as f64).fract() == 0.0 { if assign95830_e148405 == 0.0 { 0.0 } else { (assign95830_e148405 * ((locals.var_dnm).powf(assign95830_e148405 - 1.0) * locals.var_dnm_dn4)) } } else { (assign95830_e148406 * (assign95830_e148405 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign95830_e148405) as f64).is_finite() && ((assign95830_e148405) as f64).fract() == 0.0 { if assign95830_e148405 == 0.0 { 0.0 } else { (assign95830_e148405 * ((locals.var_dnm).powf(assign95830_e148405 - 1.0) * locals.var_dnm_dn5)) } } else { (assign95830_e148406 * (assign95830_e148405 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign95830_e148405) as f64).is_finite() && ((assign95830_e148405) as f64).fract() == 0.0 { if assign95830_e148405 == 0.0 { 0.0 } else { (assign95830_e148405 * ((locals.var_dnm).powf(assign95830_e148405 - 1.0) * locals.var_dnm_dn6)) } } else { (assign95830_e148406 * (assign95830_e148405 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign95830_e148405) as f64).is_finite() && ((assign95830_e148405) as f64).fract() == 0.0 { if assign95830_e148405 == 0.0 { 0.0 } else { (assign95830_e148405 * ((locals.var_dnm).powf(assign95830_e148405 - 1.0) * locals.var_dnm_dn7)) } } else { (assign95830_e148406 * (assign95830_e148405 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign95830_e148405) as f64).is_finite() && ((assign95830_e148405) as f64).fract() == 0.0 { if assign95830_e148405 == 0.0 { 0.0 } else { (assign95830_e148405 * ((locals.var_dnm).powf(assign95830_e148405 - 1.0) * locals.var_dnm_dn8)) } } else { (assign95830_e148406 * (assign95830_e148405 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign95830_e148405) as f64).is_finite() && ((assign95830_e148405) as f64).fract() == 0.0 { if assign95830_e148405 == 0.0 { 0.0 } else { (assign95830_e148405 * ((locals.var_dnm).powf(assign95830_e148405 - 1.0) * locals.var_dnm_dn9)) } } else { (assign95830_e148406 * (assign95830_e148405 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign95830_e148405) as f64).is_finite() && ((assign95830_e148405) as f64).fract() == 0.0 { if assign95830_e148405 == 0.0 { 0.0 } else { (assign95830_e148405 * ((locals.var_dnm).powf(assign95830_e148405 - 1.0) * locals.var_dnm_dn10)) } } else { (assign95830_e148406 * (assign95830_e148405 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign95830_e148405) as f64).is_finite() && ((assign95830_e148405) as f64).fract() == 0.0 { if assign95830_e148405 == 0.0 { 0.0 } else { (assign95830_e148405 * ((locals.var_dnm).powf(assign95830_e148405 - 1.0) * locals.var_dnm_dn11)) } } else { (assign95830_e148406 * (assign95830_e148405 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign95830_e148405) as f64).is_finite() && ((assign95830_e148405) as f64).fract() == 0.0 { if assign95830_e148405 == 0.0 { 0.0 } else { (assign95830_e148405 * ((locals.var_dnm).powf(assign95830_e148405 - 1.0) * locals.var_dnm_dn14)) } } else { (assign95830_e148406 * (assign95830_e148405 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign95830_e148407, assign95830_e148407_d_n0, assign95830_e148407_d_n2, assign95830_e148407_d_n4, assign95830_e148407_d_n5, assign95830_e148407_d_n6, assign95830_e148407_d_n7, assign95830_e148407_d_n8, assign95830_e148407_d_n9, assign95830_e148407_d_n10, assign95830_e148407_d_n11, assign95830_e148407_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign95830_e148409;
        locals.var_dnm_dn0 = assign95830_e148409_d_n0;
        locals.var_dnm_dn2 = assign95830_e148409_d_n2;
        locals.var_dnm_dn4 = assign95830_e148409_d_n4;
        locals.var_dnm_dn5 = assign95830_e148409_d_n5;
        locals.var_dnm_dn6 = assign95830_e148409_d_n6;
        locals.var_dnm_dn7 = assign95830_e148409_d_n7;
        locals.var_dnm_dn8 = assign95830_e148409_d_n8;
        locals.var_dnm_dn9 = assign95830_e148409_d_n9;
        locals.var_dnm_dn10 = assign95830_e148409_d_n10;
        locals.var_dnm_dn11 = assign95830_e148409_d_n11;
        locals.var_dnm_dn14 = assign95830_e148409_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign95840_e148417, assign95840_e148417_d_n0, assign95840_e148417_d_n2, assign95840_e148417_d_n4, assign95840_e148417_d_n5, assign95840_e148417_d_n6, assign95840_e148417_d_n7, assign95840_e148417_d_n8, assign95840_e148417_d_n9, assign95840_e148417_d_n10, assign95840_e148417_d_n11, assign95840_e148417_d_n14,) = {
    if ((locals.var_guard2226 != 0.0) && (locals.var_guard2227 != 0.0)) {
        let assign95840_e148415: f64 = (1.0 / locals.var_dnm);
        (assign95840_e148415, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign95840_e148417;
        locals.var_dnm_dn0 = assign95840_e148417_d_n0;
        locals.var_dnm_dn2 = assign95840_e148417_d_n2;
        locals.var_dnm_dn4 = assign95840_e148417_d_n4;
        locals.var_dnm_dn5 = assign95840_e148417_d_n5;
        locals.var_dnm_dn6 = assign95840_e148417_d_n6;
        locals.var_dnm_dn7 = assign95840_e148417_d_n7;
        locals.var_dnm_dn8 = assign95840_e148417_d_n8;
        locals.var_dnm_dn9 = assign95840_e148417_d_n9;
        locals.var_dnm_dn10 = assign95840_e148417_d_n10;
        locals.var_dnm_dn11 = assign95840_e148417_d_n11;
        locals.var_dnm_dn14 = assign95840_e148417_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign95850_e148429, assign95850_e148429_d_n0, assign95850_e148429_d_n2, assign95850_e148429_d_n4, assign95850_e148429_d_n5, assign95850_e148429_d_n6, assign95850_e148429_d_n7, assign95850_e148429_d_n8, assign95850_e148429_d_n9, assign95850_e148429_d_n10, assign95850_e148429_d_n11, assign95850_e148429_d_n14,) = {
    if ((locals.var_guard2226 != 0.0) && (locals.var_guard2227 != 0.0)) {
        let assign95850_e148424: f64 = (10.0 * 2.220446049250313e-16);
        let assign95850_e148425: f64 = (locals.var_tmf1 * assign95850_e148424);
        let assign95850_e148427: f64 = (assign95850_e148425 * locals.var_dnm);
        (assign95850_e148427, (((locals.var_tmf1_dn0 * assign95850_e148424) * locals.var_dnm) + (assign95850_e148425 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * assign95850_e148424) * locals.var_dnm) + (assign95850_e148425 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * assign95850_e148424) * locals.var_dnm) + (assign95850_e148425 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * assign95850_e148424) * locals.var_dnm) + (assign95850_e148425 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * assign95850_e148424) * locals.var_dnm) + (assign95850_e148425 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * assign95850_e148424) * locals.var_dnm) + (assign95850_e148425 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * assign95850_e148424) * locals.var_dnm) + (assign95850_e148425 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * assign95850_e148424) * locals.var_dnm) + (assign95850_e148425 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * assign95850_e148424) * locals.var_dnm) + (assign95850_e148425 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * assign95850_e148424) * locals.var_dnm) + (assign95850_e148425 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * assign95850_e148424) * locals.var_dnm) + (assign95850_e148425 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign95850_e148429;
        locals.var_tmf0_dn0 = assign95850_e148429_d_n0;
        locals.var_tmf0_dn2 = assign95850_e148429_d_n2;
        locals.var_tmf0_dn4 = assign95850_e148429_d_n4;
        locals.var_tmf0_dn5 = assign95850_e148429_d_n5;
        locals.var_tmf0_dn6 = assign95850_e148429_d_n6;
        locals.var_tmf0_dn7 = assign95850_e148429_d_n7;
        locals.var_tmf0_dn8 = assign95850_e148429_d_n8;
        locals.var_tmf0_dn9 = assign95850_e148429_d_n9;
        locals.var_tmf0_dn10 = assign95850_e148429_d_n10;
        locals.var_tmf0_dn11 = assign95850_e148429_d_n11;
        locals.var_tmf0_dn14 = assign95850_e148429_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign95860_e148443, assign95860_e148443_d_n0, assign95860_e148443_d_n2, assign95860_e148443_d_n4, assign95860_e148443_d_n5, assign95860_e148443_d_n6, assign95860_e148443_d_n7, assign95860_e148443_d_n8, assign95860_e148443_d_n9, assign95860_e148443_d_n10, assign95860_e148443_d_n11, assign95860_e148443_d_n14,) = {
    if ((locals.var_guard2226 != 0.0) && (locals.var_guard2227 != 0.0)) {
        let assign95860_e148435: f64 = (10.0 * 2.220446049250313e-16);
        let assign95860_e148437: f64 = (assign95860_e148435 * locals.var_xmp);
        let assign95860_e148439: f64 = (assign95860_e148437 * locals.var_dnm);
        let assign95860_e148441: f64 = (assign95860_e148439 / locals.var_arg);
        (assign95860_e148441, ((((((assign95860_e148435 * locals.var_xmp_dn0) * locals.var_dnm) + (assign95860_e148437 * locals.var_dnm_dn0)) * locals.var_arg) - (assign95860_e148439 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((assign95860_e148435 * locals.var_xmp_dn2) * locals.var_dnm) + (assign95860_e148437 * locals.var_dnm_dn2)) * locals.var_arg) - (assign95860_e148439 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((assign95860_e148435 * locals.var_xmp_dn4) * locals.var_dnm) + (assign95860_e148437 * locals.var_dnm_dn4)) * locals.var_arg) - (assign95860_e148439 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((assign95860_e148435 * locals.var_xmp_dn5) * locals.var_dnm) + (assign95860_e148437 * locals.var_dnm_dn5)) * locals.var_arg) - (assign95860_e148439 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((assign95860_e148435 * locals.var_xmp_dn6) * locals.var_dnm) + (assign95860_e148437 * locals.var_dnm_dn6)) * locals.var_arg) - (assign95860_e148439 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((assign95860_e148435 * locals.var_xmp_dn7) * locals.var_dnm) + (assign95860_e148437 * locals.var_dnm_dn7)) * locals.var_arg) - (assign95860_e148439 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((assign95860_e148435 * locals.var_xmp_dn8) * locals.var_dnm) + (assign95860_e148437 * locals.var_dnm_dn8)) * locals.var_arg) - (assign95860_e148439 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((assign95860_e148435 * locals.var_xmp_dn9) * locals.var_dnm) + (assign95860_e148437 * locals.var_dnm_dn9)) * locals.var_arg) - (assign95860_e148439 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((assign95860_e148435 * locals.var_xmp_dn10) * locals.var_dnm) + (assign95860_e148437 * locals.var_dnm_dn10)) * locals.var_arg) - (assign95860_e148439 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((assign95860_e148435 * locals.var_xmp_dn11) * locals.var_dnm) + (assign95860_e148437 * locals.var_dnm_dn11)) * locals.var_arg) - (assign95860_e148439 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((assign95860_e148435 * locals.var_xmp_dn14) * locals.var_dnm) + (assign95860_e148437 * locals.var_dnm_dn14)) * locals.var_arg) - (assign95860_e148439 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign95860_e148443;
        locals.var_t0_dn0 = assign95860_e148443_d_n0;
        locals.var_t0_dn2 = assign95860_e148443_d_n2;
        locals.var_t0_dn4 = assign95860_e148443_d_n4;
        locals.var_t0_dn5 = assign95860_e148443_d_n5;
        locals.var_t0_dn6 = assign95860_e148443_d_n6;
        locals.var_t0_dn7 = assign95860_e148443_d_n7;
        locals.var_t0_dn8 = assign95860_e148443_d_n8;
        locals.var_t0_dn9 = assign95860_e148443_d_n9;
        locals.var_t0_dn10 = assign95860_e148443_d_n10;
        locals.var_t0_dn11 = assign95860_e148443_d_n11;
        locals.var_t0_dn14 = assign95860_e148443_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign95870_e148461, assign95870_e148461_d_n0, assign95870_e148461_d_n2, assign95870_e148461_d_n4, assign95870_e148461_d_n5, assign95870_e148461_d_n6, assign95870_e148461_d_n7, assign95870_e148461_d_n8, assign95870_e148461_d_n9, assign95870_e148461_d_n10, assign95870_e148461_d_n11, assign95870_e148461_d_n14,) = {
    if ((locals.var_guard2226 != 0.0) && (locals.var_guard2227 != 0.0)) {
        let assign95870_e148449: f64 = (locals.var_ps0 + locals.var_vds);
        let assign95870_e148452: f64 = (10.0 * 2.220446049250313e-16);
        let assign95870_e148453: f64 = (assign95870_e148449 - assign95870_e148452);
        let assign95870_e148456: f64 = (10.0 * 2.220446049250313e-16);
        let assign95870_e148457: f64 = (assign95870_e148453 - assign95870_e148456);
        let assign95870_e148459: f64 = (assign95870_e148457 + locals.var_tmf0);
        (assign95870_e148459, ((locals.var_ps0_dn0 + locals.var_vds_dn0) + locals.var_tmf0_dn0), ((locals.var_ps0_dn2 + locals.var_vds_dn2) + locals.var_tmf0_dn2), ((locals.var_ps0_dn4 + locals.var_vds_dn4) + locals.var_tmf0_dn4), ((locals.var_ps0_dn5 + locals.var_vds_dn5) + locals.var_tmf0_dn5), ((locals.var_ps0_dn6 + locals.var_vds_dn6) + locals.var_tmf0_dn6), ((locals.var_ps0_dn7 + locals.var_vds_dn7) + locals.var_tmf0_dn7), ((locals.var_ps0_dn8 + locals.var_vds_dn8) + locals.var_tmf0_dn8), ((locals.var_ps0_dn9 + locals.var_vds_dn9) + locals.var_tmf0_dn9), ((locals.var_ps0_dn10 + locals.var_vds_dn10) + locals.var_tmf0_dn10), ((locals.var_ps0_dn11 + locals.var_vds_dn11) + locals.var_tmf0_dn11), ((locals.var_ps0_dn14 + locals.var_vds_dn14) + locals.var_tmf0_dn14),)
    } else {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn4, locals.var_psdl_dn5, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn8, locals.var_psdl_dn9, locals.var_psdl_dn10, locals.var_psdl_dn11, locals.var_psdl_dn14,)
    }
};
        locals.var_psdl = assign95870_e148461;
        locals.var_psdl_dn0 = assign95870_e148461_d_n0;
        locals.var_psdl_dn2 = assign95870_e148461_d_n2;
        locals.var_psdl_dn4 = assign95870_e148461_d_n4;
        locals.var_psdl_dn5 = assign95870_e148461_d_n5;
        locals.var_psdl_dn6 = assign95870_e148461_d_n6;
        locals.var_psdl_dn7 = assign95870_e148461_d_n7;
        locals.var_psdl_dn8 = assign95870_e148461_d_n8;
        locals.var_psdl_dn9 = assign95870_e148461_d_n9;
        locals.var_psdl_dn10 = assign95870_e148461_d_n10;
        locals.var_psdl_dn11 = assign95870_e148461_d_n11;
        locals.var_psdl_dn14 = assign95870_e148461_d_n14;
        locals.var_psdl_rv = 0.0;

        let (assign95880_e148467, assign95880_e148467_d_n0, assign95880_e148467_d_n2, assign95880_e148467_d_n4, assign95880_e148467_d_n5, assign95880_e148467_d_n6, assign95880_e148467_d_n7, assign95880_e148467_d_n8, assign95880_e148467_d_n9, assign95880_e148467_d_n10, assign95880_e148467_d_n11, assign95880_e148467_d_n14,) = {
    if ((locals.var_guard2226 != 0.0) && (locals.var_guard2227 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign95880_e148467;
        locals.var_t0_dn0 = assign95880_e148467_d_n0;
        locals.var_t0_dn2 = assign95880_e148467_d_n2;
        locals.var_t0_dn4 = assign95880_e148467_d_n4;
        locals.var_t0_dn5 = assign95880_e148467_d_n5;
        locals.var_t0_dn6 = assign95880_e148467_d_n6;
        locals.var_t0_dn7 = assign95880_e148467_d_n7;
        locals.var_t0_dn8 = assign95880_e148467_d_n8;
        locals.var_t0_dn9 = assign95880_e148467_d_n9;
        locals.var_t0_dn10 = assign95880_e148467_d_n10;
        locals.var_t0_dn11 = assign95880_e148467_d_n11;
        locals.var_t0_dn14 = assign95880_e148467_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign95890_e148474, assign95890_e148474_d_n0, assign95890_e148474_d_n2, assign95890_e148474_d_n4, assign95890_e148474_d_n5, assign95890_e148474_d_n6, assign95890_e148474_d_n7, assign95890_e148474_d_n8, assign95890_e148474_d_n9, assign95890_e148474_d_n10, assign95890_e148474_d_n11, assign95890_e148474_d_n14,) = {
    if ((locals.var_guard2226 != 0.0) && (locals.var_guard2227 == 0.0)) {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn4, locals.var_psdl_dn5, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn8, locals.var_psdl_dn9, locals.var_psdl_dn10, locals.var_psdl_dn11, locals.var_psdl_dn14,)
    } else {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn4, locals.var_psdl_dn5, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn8, locals.var_psdl_dn9, locals.var_psdl_dn10, locals.var_psdl_dn11, locals.var_psdl_dn14,)
    }
};
        locals.var_psdl = assign95890_e148474;
        locals.var_psdl_dn0 = assign95890_e148474_d_n0;
        locals.var_psdl_dn2 = assign95890_e148474_d_n2;
        locals.var_psdl_dn4 = assign95890_e148474_d_n4;
        locals.var_psdl_dn5 = assign95890_e148474_d_n5;
        locals.var_psdl_dn6 = assign95890_e148474_d_n6;
        locals.var_psdl_dn7 = assign95890_e148474_d_n7;
        locals.var_psdl_dn8 = assign95890_e148474_d_n8;
        locals.var_psdl_dn9 = assign95890_e148474_d_n9;
        locals.var_psdl_dn10 = assign95890_e148474_d_n10;
        locals.var_psdl_dn11 = assign95890_e148474_d_n11;
        locals.var_psdl_dn14 = assign95890_e148474_d_n14;
        locals.var_psdl_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_372(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign95900_e148481, assign95900_e148481_d_n0, assign95900_e148481_d_n2, assign95900_e148481_d_n4, assign95900_e148481_d_n5, assign95900_e148481_d_n6, assign95900_e148481_d_n7, assign95900_e148481_d_n8, assign95900_e148481_d_n9, assign95900_e148481_d_n10, assign95900_e148481_d_n11, assign95900_e148481_d_n14,) = {
    if ((locals.var_guard2226 != 0.0) && (locals.var_guard2227 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign95900_e148481;
        locals.var_t0_dn0 = assign95900_e148481_d_n0;
        locals.var_t0_dn2 = assign95900_e148481_d_n2;
        locals.var_t0_dn4 = assign95900_e148481_d_n4;
        locals.var_t0_dn5 = assign95900_e148481_d_n5;
        locals.var_t0_dn6 = assign95900_e148481_d_n6;
        locals.var_t0_dn7 = assign95900_e148481_d_n7;
        locals.var_t0_dn8 = assign95900_e148481_d_n8;
        locals.var_t0_dn9 = assign95900_e148481_d_n9;
        locals.var_t0_dn10 = assign95900_e148481_d_n10;
        locals.var_t0_dn11 = assign95900_e148481_d_n11;
        locals.var_t0_dn14 = assign95900_e148481_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign95910_e148487, assign95910_e148487_d_n0, assign95910_e148487_d_n2, assign95910_e148487_d_n4, assign95910_e148487_d_n5, assign95910_e148487_d_n6, assign95910_e148487_d_n7, assign95910_e148487_d_n8, assign95910_e148487_d_n9, assign95910_e148487_d_n10, assign95910_e148487_d_n11, assign95910_e148487_d_n14,) = {
    if ((locals.var_guard2226 != 0.0) && (locals.var_flg_qy != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ec, locals.var_ec_dn0, locals.var_ec_dn2, locals.var_ec_dn4, locals.var_ec_dn5, locals.var_ec_dn6, locals.var_ec_dn7, locals.var_ec_dn8, locals.var_ec_dn9, locals.var_ec_dn10, locals.var_ec_dn11, locals.var_ec_dn14,)
    }
};
        locals.var_ec = assign95910_e148487;
        locals.var_ec_dn0 = assign95910_e148487_d_n0;
        locals.var_ec_dn2 = assign95910_e148487_d_n2;
        locals.var_ec_dn4 = assign95910_e148487_d_n4;
        locals.var_ec_dn5 = assign95910_e148487_d_n5;
        locals.var_ec_dn6 = assign95910_e148487_d_n6;
        locals.var_ec_dn7 = assign95910_e148487_d_n7;
        locals.var_ec_dn8 = assign95910_e148487_d_n8;
        locals.var_ec_dn9 = assign95910_e148487_d_n9;
        locals.var_ec_dn10 = assign95910_e148487_d_n10;
        locals.var_ec_dn11 = assign95910_e148487_d_n11;
        locals.var_ec_dn14 = assign95910_e148487_d_n14;
        locals.var_ec_rv = 0.0;

        let assign95920_e148494: f64 = if ((locals.var_idd < 1e-15) || (locals.var_vdseff < 1e-6)) { 1.0 } else { 0.0 };
        locals.var_guard2233 = assign95920_e148494;
        locals.var_guard2233_rv = 0.0;

        let (assign95930_e148503, assign95930_e148503_d_n0, assign95930_e148503_d_n2, assign95930_e148503_d_n4, assign95930_e148503_d_n5, assign95930_e148503_d_n6, assign95930_e148503_d_n7, assign95930_e148503_d_n8, assign95930_e148503_d_n9, assign95930_e148503_d_n10, assign95930_e148503_d_n11, assign95930_e148503_d_n14,) = {
    if (((locals.var_guard2226 == 0.0) && (locals.var_flg_qy != 0.0)) && (locals.var_guard2233 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ec, locals.var_ec_dn0, locals.var_ec_dn2, locals.var_ec_dn4, locals.var_ec_dn5, locals.var_ec_dn6, locals.var_ec_dn7, locals.var_ec_dn8, locals.var_ec_dn9, locals.var_ec_dn10, locals.var_ec_dn11, locals.var_ec_dn14,)
    }
};
        locals.var_ec = assign95930_e148503;
        locals.var_ec_dn0 = assign95930_e148503_d_n0;
        locals.var_ec_dn2 = assign95930_e148503_d_n2;
        locals.var_ec_dn4 = assign95930_e148503_d_n4;
        locals.var_ec_dn5 = assign95930_e148503_d_n5;
        locals.var_ec_dn6 = assign95930_e148503_d_n6;
        locals.var_ec_dn7 = assign95930_e148503_d_n7;
        locals.var_ec_dn8 = assign95930_e148503_d_n8;
        locals.var_ec_dn9 = assign95930_e148503_d_n9;
        locals.var_ec_dn10 = assign95930_e148503_d_n10;
        locals.var_ec_dn11 = assign95930_e148503_d_n11;
        locals.var_ec_dn14 = assign95930_e148503_d_n14;
        locals.var_ec_rv = 0.0;

        let (assign95940_e148519, assign95940_e148519_d_n0, assign95940_e148519_d_n2, assign95940_e148519_d_n4, assign95940_e148519_d_n5, assign95940_e148519_d_n6, assign95940_e148519_d_n7, assign95940_e148519_d_n8, assign95940_e148519_d_n9, assign95940_e148519_d_n10, assign95940_e148519_d_n11, assign95940_e148519_d_n14,) = {
    if (((locals.var_guard2226 == 0.0) && (locals.var_flg_qy != 0.0)) && (locals.var_guard2233 == 0.0)) {
        let assign95940_e148513: f64 = (locals.var_idd / locals.var_qn0);
        let assign95940_e148515: f64 = (assign95940_e148513 * locals.var_beta_inv);
        let assign95940_e148517: f64 = (assign95940_e148515 / locals.var_leff);
        (assign95940_e148517, ((((((locals.var_idd_dn0 * locals.var_qn0) - (locals.var_idd * locals.var_qn0_dn0)) / (locals.var_qn0 * locals.var_qn0)) * locals.var_beta_inv) + (assign95940_e148513 * locals.var_beta_inv_dn0)) / locals.var_leff), ((((((locals.var_idd_dn2 * locals.var_qn0) - (locals.var_idd * locals.var_qn0_dn2)) / (locals.var_qn0 * locals.var_qn0)) * locals.var_beta_inv) + (assign95940_e148513 * locals.var_beta_inv_dn2)) / locals.var_leff), ((((((locals.var_idd_dn4 * locals.var_qn0) - (locals.var_idd * locals.var_qn0_dn4)) / (locals.var_qn0 * locals.var_qn0)) * locals.var_beta_inv) + (assign95940_e148513 * locals.var_beta_inv_dn4)) / locals.var_leff), ((((((locals.var_idd_dn5 * locals.var_qn0) - (locals.var_idd * locals.var_qn0_dn5)) / (locals.var_qn0 * locals.var_qn0)) * locals.var_beta_inv) + (assign95940_e148513 * locals.var_beta_inv_dn5)) / locals.var_leff), ((((((locals.var_idd_dn6 * locals.var_qn0) - (locals.var_idd * locals.var_qn0_dn6)) / (locals.var_qn0 * locals.var_qn0)) * locals.var_beta_inv) + (assign95940_e148513 * locals.var_beta_inv_dn6)) / locals.var_leff), ((((((locals.var_idd_dn7 * locals.var_qn0) - (locals.var_idd * locals.var_qn0_dn7)) / (locals.var_qn0 * locals.var_qn0)) * locals.var_beta_inv) + (assign95940_e148513 * locals.var_beta_inv_dn7)) / locals.var_leff), ((((((locals.var_idd_dn8 * locals.var_qn0) - (locals.var_idd * locals.var_qn0_dn8)) / (locals.var_qn0 * locals.var_qn0)) * locals.var_beta_inv) + (assign95940_e148513 * locals.var_beta_inv_dn8)) / locals.var_leff), ((((((locals.var_idd_dn9 * locals.var_qn0) - (locals.var_idd * locals.var_qn0_dn9)) / (locals.var_qn0 * locals.var_qn0)) * locals.var_beta_inv) + (assign95940_e148513 * locals.var_beta_inv_dn9)) / locals.var_leff), ((((((locals.var_idd_dn10 * locals.var_qn0) - (locals.var_idd * locals.var_qn0_dn10)) / (locals.var_qn0 * locals.var_qn0)) * locals.var_beta_inv) + (assign95940_e148513 * locals.var_beta_inv_dn10)) / locals.var_leff), ((((((locals.var_idd_dn11 * locals.var_qn0) - (locals.var_idd * locals.var_qn0_dn11)) / (locals.var_qn0 * locals.var_qn0)) * locals.var_beta_inv) + (assign95940_e148513 * locals.var_beta_inv_dn11)) / locals.var_leff), ((((((locals.var_idd_dn14 * locals.var_qn0) - (locals.var_idd * locals.var_qn0_dn14)) / (locals.var_qn0 * locals.var_qn0)) * locals.var_beta_inv) + (assign95940_e148513 * locals.var_beta_inv_dn14)) / locals.var_leff),)
    } else {
        (locals.var_ec, locals.var_ec_dn0, locals.var_ec_dn2, locals.var_ec_dn4, locals.var_ec_dn5, locals.var_ec_dn6, locals.var_ec_dn7, locals.var_ec_dn8, locals.var_ec_dn9, locals.var_ec_dn10, locals.var_ec_dn11, locals.var_ec_dn14,)
    }
};
        locals.var_ec = assign95940_e148519;
        locals.var_ec_dn0 = assign95940_e148519_d_n0;
        locals.var_ec_dn2 = assign95940_e148519_d_n2;
        locals.var_ec_dn4 = assign95940_e148519_d_n4;
        locals.var_ec_dn5 = assign95940_e148519_d_n5;
        locals.var_ec_dn6 = assign95940_e148519_d_n6;
        locals.var_ec_dn7 = assign95940_e148519_d_n7;
        locals.var_ec_dn8 = assign95940_e148519_d_n8;
        locals.var_ec_dn9 = assign95940_e148519_d_n9;
        locals.var_ec_dn10 = assign95940_e148519_d_n10;
        locals.var_ec_dn11 = assign95940_e148519_d_n11;
        locals.var_ec_dn14 = assign95940_e148519_d_n14;
        locals.var_ec_rv = 0.0;

        let assign95950_e148522: f64 = if locals.var_flg_qy == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2234 = assign95950_e148522;
        locals.var_guard2234_rv = 0.0;

        let (assign95960_e148526, assign95960_e148526_d_n0, assign95960_e148526_d_n2, assign95960_e148526_d_n4, assign95960_e148526_d_n5, assign95960_e148526_d_n6, assign95960_e148526_d_n7, assign95960_e148526_d_n8, assign95960_e148526_d_n9, assign95960_e148526_d_n10, assign95960_e148526_d_n11, assign95960_e148526_d_n14,) = {
    if (locals.var_guard2234 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qy, locals.var_qy_dn0, locals.var_qy_dn2, locals.var_qy_dn4, locals.var_qy_dn5, locals.var_qy_dn6, locals.var_qy_dn7, locals.var_qy_dn8, locals.var_qy_dn9, locals.var_qy_dn10, locals.var_qy_dn11, locals.var_qy_dn14,)
    }
};
        locals.var_qy = assign95960_e148526;
        locals.var_qy_dn0 = assign95960_e148526_d_n0;
        locals.var_qy_dn2 = assign95960_e148526_d_n2;
        locals.var_qy_dn4 = assign95960_e148526_d_n4;
        locals.var_qy_dn5 = assign95960_e148526_d_n5;
        locals.var_qy_dn6 = assign95960_e148526_d_n6;
        locals.var_qy_dn7 = assign95960_e148526_d_n7;
        locals.var_qy_dn8 = assign95960_e148526_d_n8;
        locals.var_qy_dn9 = assign95960_e148526_d_n9;
        locals.var_qy_dn10 = assign95960_e148526_d_n10;
        locals.var_qy_dn11 = assign95960_e148526_d_n11;
        locals.var_qy_dn14 = assign95960_e148526_d_n14;
        locals.var_qy_rv = 0.0;

        let (assign95970_e148537, assign95970_e148537_d_n0, assign95970_e148537_d_n2, assign95970_e148537_d_n4, assign95970_e148537_d_n5, assign95970_e148537_d_n6, assign95970_e148537_d_n7, assign95970_e148537_d_n8, assign95970_e148537_d_n9, assign95970_e148537_d_n10, assign95970_e148537_d_n11, assign95970_e148537_d_n14,) = {
    if (locals.var_guard2234 == 0.0) {
        let assign95970_e148531: f64 = (1.034943e-10 * locals.var_weffcv_nf);
        let assign95970_e148533: f64 = (assign95970_e148531 * locals.var_wdpl);
        let assign95970_e148535: f64 = (assign95970_e148533 * 1.3);
        (assign95970_e148535, ((assign95970_e148531 * locals.var_wdpl_dn0) * 1.3), ((assign95970_e148531 * locals.var_wdpl_dn2) * 1.3), ((assign95970_e148531 * locals.var_wdpl_dn4) * 1.3), ((assign95970_e148531 * locals.var_wdpl_dn5) * 1.3), ((assign95970_e148531 * locals.var_wdpl_dn6) * 1.3), ((assign95970_e148531 * locals.var_wdpl_dn7) * 1.3), ((assign95970_e148531 * locals.var_wdpl_dn8) * 1.3), ((assign95970_e148531 * locals.var_wdpl_dn9) * 1.3), ((assign95970_e148531 * locals.var_wdpl_dn10) * 1.3), ((assign95970_e148531 * locals.var_wdpl_dn11) * 1.3), ((assign95970_e148531 * locals.var_wdpl_dn14) * 1.3),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign95970_e148537;
        locals.var_t2_dn0 = assign95970_e148537_d_n0;
        locals.var_t2_dn2 = assign95970_e148537_d_n2;
        locals.var_t2_dn4 = assign95970_e148537_d_n4;
        locals.var_t2_dn5 = assign95970_e148537_d_n5;
        locals.var_t2_dn6 = assign95970_e148537_d_n6;
        locals.var_t2_dn7 = assign95970_e148537_d_n7;
        locals.var_t2_dn8 = assign95970_e148537_d_n8;
        locals.var_t2_dn9 = assign95970_e148537_d_n9;
        locals.var_t2_dn10 = assign95970_e148537_d_n10;
        locals.var_t2_dn11 = assign95970_e148537_d_n11;
        locals.var_t2_dn14 = assign95970_e148537_d_n14;
        locals.var_t2_rv = 0.0;

        let assign95980_e148540: f64 = if p.p133 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2235 = assign95980_e148540;
        locals.var_guard2235_rv = 0.0;

        let (assign95990_e148551, assign95990_e148551_d_n0, assign95990_e148551_d_n2, assign95990_e148551_d_n4, assign95990_e148551_d_n5, assign95990_e148551_d_n6, assign95990_e148551_d_n7, assign95990_e148551_d_n8, assign95990_e148551_d_n9, assign95990_e148551_d_n10, assign95990_e148551_d_n11, assign95990_e148551_d_n14,) = {
    if ((locals.var_guard2234 == 0.0) && (locals.var_guard2235 != 0.0)) {
        let assign95990_e148547: f64 = (locals.var_ec * locals.var_leff);
        let assign95990_e148549: f64 = (assign95990_e148547 + locals.var_ps0);
        (assign95990_e148549, ((locals.var_ec_dn0 * locals.var_leff) + locals.var_ps0_dn0), ((locals.var_ec_dn2 * locals.var_leff) + locals.var_ps0_dn2), ((locals.var_ec_dn4 * locals.var_leff) + locals.var_ps0_dn4), ((locals.var_ec_dn5 * locals.var_leff) + locals.var_ps0_dn5), ((locals.var_ec_dn6 * locals.var_leff) + locals.var_ps0_dn6), ((locals.var_ec_dn7 * locals.var_leff) + locals.var_ps0_dn7), ((locals.var_ec_dn8 * locals.var_leff) + locals.var_ps0_dn8), ((locals.var_ec_dn9 * locals.var_leff) + locals.var_ps0_dn9), ((locals.var_ec_dn10 * locals.var_leff) + locals.var_ps0_dn10), ((locals.var_ec_dn11 * locals.var_leff) + locals.var_ps0_dn11), ((locals.var_ec_dn14 * locals.var_leff) + locals.var_ps0_dn14),)
    } else {
        (locals.var_pslk, locals.var_pslk_dn0, locals.var_pslk_dn2, locals.var_pslk_dn4, locals.var_pslk_dn5, locals.var_pslk_dn6, locals.var_pslk_dn7, locals.var_pslk_dn8, locals.var_pslk_dn9, locals.var_pslk_dn10, locals.var_pslk_dn11, locals.var_pslk_dn14,)
    }
};
        locals.var_pslk = assign95990_e148551;
        locals.var_pslk_dn0 = assign95990_e148551_d_n0;
        locals.var_pslk_dn2 = assign95990_e148551_d_n2;
        locals.var_pslk_dn4 = assign95990_e148551_d_n4;
        locals.var_pslk_dn5 = assign95990_e148551_d_n5;
        locals.var_pslk_dn6 = assign95990_e148551_d_n6;
        locals.var_pslk_dn7 = assign95990_e148551_d_n7;
        locals.var_pslk_dn8 = assign95990_e148551_d_n8;
        locals.var_pslk_dn9 = assign95990_e148551_d_n9;
        locals.var_pslk_dn10 = assign95990_e148551_d_n10;
        locals.var_pslk_dn11 = assign95990_e148551_d_n11;
        locals.var_pslk_dn14 = assign95990_e148551_d_n14;
        locals.var_pslk_rv = 0.0;

        let (assign96000_e148568, assign96000_e148568_d_n0, assign96000_e148568_d_n2, assign96000_e148568_d_n4, assign96000_e148568_d_n5, assign96000_e148568_d_n6, assign96000_e148568_d_n7, assign96000_e148568_d_n8, assign96000_e148568_d_n9, assign96000_e148568_d_n10, assign96000_e148568_d_n11, assign96000_e148568_d_n14,) = {
    if ((locals.var_guard2234 == 0.0) && (locals.var_guard2235 != 0.0)) {
        let assign96000_e148559: f64 = (locals.var_vdsz__blk443 + locals.var_ps0);
        let assign96000_e148560: f64 = (locals.var_aclm * assign96000_e148559);
        let assign96000_e148563: f64 = (1.0 - locals.var_aclm);
        let assign96000_e148565: f64 = (assign96000_e148563 * locals.var_pslk);
        let assign96000_e148566: f64 = (assign96000_e148560 + assign96000_e148565);
        (assign96000_e148566, ((locals.var_aclm * (locals.var_vdsz__blk443_dn0 + locals.var_ps0_dn0)) + (assign96000_e148563 * locals.var_pslk_dn0)), ((locals.var_aclm * (locals.var_vdsz__blk443_dn2 + locals.var_ps0_dn2)) + (assign96000_e148563 * locals.var_pslk_dn2)), ((locals.var_aclm * (locals.var_vdsz__blk443_dn4 + locals.var_ps0_dn4)) + (assign96000_e148563 * locals.var_pslk_dn4)), ((locals.var_aclm * (locals.var_vdsz__blk443_dn5 + locals.var_ps0_dn5)) + (assign96000_e148563 * locals.var_pslk_dn5)), ((locals.var_aclm * (locals.var_vdsz__blk443_dn6 + locals.var_ps0_dn6)) + (assign96000_e148563 * locals.var_pslk_dn6)), ((locals.var_aclm * (locals.var_vdsz__blk443_dn7 + locals.var_ps0_dn7)) + (assign96000_e148563 * locals.var_pslk_dn7)), ((locals.var_aclm * (locals.var_vdsz__blk443_dn8 + locals.var_ps0_dn8)) + (assign96000_e148563 * locals.var_pslk_dn8)), ((locals.var_aclm * (locals.var_vdsz__blk443_dn9 + locals.var_ps0_dn9)) + (assign96000_e148563 * locals.var_pslk_dn9)), ((locals.var_aclm * (locals.var_vdsz__blk443_dn10 + locals.var_ps0_dn10)) + (assign96000_e148563 * locals.var_pslk_dn10)), ((locals.var_aclm * (locals.var_vdsz__blk443_dn11 + locals.var_ps0_dn11)) + (assign96000_e148563 * locals.var_pslk_dn11)), ((locals.var_aclm * (locals.var_vdsz__blk443_dn14 + locals.var_ps0_dn14)) + (assign96000_e148563 * locals.var_pslk_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign96000_e148568;
        locals.var_t1_dn0 = assign96000_e148568_d_n0;
        locals.var_t1_dn2 = assign96000_e148568_d_n2;
        locals.var_t1_dn4 = assign96000_e148568_d_n4;
        locals.var_t1_dn5 = assign96000_e148568_d_n5;
        locals.var_t1_dn6 = assign96000_e148568_d_n6;
        locals.var_t1_dn7 = assign96000_e148568_d_n7;
        locals.var_t1_dn8 = assign96000_e148568_d_n8;
        locals.var_t1_dn9 = assign96000_e148568_d_n9;
        locals.var_t1_dn10 = assign96000_e148568_d_n10;
        locals.var_t1_dn11 = assign96000_e148568_d_n11;
        locals.var_t1_dn14 = assign96000_e148568_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign96010_e148584, assign96010_e148584_d_n0, assign96010_e148584_d_n2, assign96010_e148584_d_n4, assign96010_e148584_d_n5, assign96010_e148584_d_n6, assign96010_e148584_d_n7, assign96010_e148584_d_n8, assign96010_e148584_d_n9, assign96010_e148584_d_n10, assign96010_e148584_d_n11, assign96010_e148584_d_n14,) = {
    if ((locals.var_guard2234 == 0.0) && (locals.var_guard2235 != 0.0)) {
        let assign96010_e148575: f64 = (locals.var_ps0 + locals.var_vdsz__blk443);
        let assign96010_e148577: f64 = (assign96010_e148575 - locals.var_t1);
        let assign96010_e148579: f64 = (assign96010_e148577 / p.p133);
        let assign96010_e148580: f64 = (-assign96010_e148579);
        let assign96010_e148582: f64 = (assign96010_e148580 * locals.var_t2);
        (assign96010_e148582, (((-(((locals.var_ps0_dn0 + locals.var_vdsz__blk443_dn0) - locals.var_t1_dn0) / p.p133)) * locals.var_t2) + (assign96010_e148580 * locals.var_t2_dn0)), (((-(((locals.var_ps0_dn2 + locals.var_vdsz__blk443_dn2) - locals.var_t1_dn2) / p.p133)) * locals.var_t2) + (assign96010_e148580 * locals.var_t2_dn2)), (((-(((locals.var_ps0_dn4 + locals.var_vdsz__blk443_dn4) - locals.var_t1_dn4) / p.p133)) * locals.var_t2) + (assign96010_e148580 * locals.var_t2_dn4)), (((-(((locals.var_ps0_dn5 + locals.var_vdsz__blk443_dn5) - locals.var_t1_dn5) / p.p133)) * locals.var_t2) + (assign96010_e148580 * locals.var_t2_dn5)), (((-(((locals.var_ps0_dn6 + locals.var_vdsz__blk443_dn6) - locals.var_t1_dn6) / p.p133)) * locals.var_t2) + (assign96010_e148580 * locals.var_t2_dn6)), (((-(((locals.var_ps0_dn7 + locals.var_vdsz__blk443_dn7) - locals.var_t1_dn7) / p.p133)) * locals.var_t2) + (assign96010_e148580 * locals.var_t2_dn7)), (((-(((locals.var_ps0_dn8 + locals.var_vdsz__blk443_dn8) - locals.var_t1_dn8) / p.p133)) * locals.var_t2) + (assign96010_e148580 * locals.var_t2_dn8)), (((-(((locals.var_ps0_dn9 + locals.var_vdsz__blk443_dn9) - locals.var_t1_dn9) / p.p133)) * locals.var_t2) + (assign96010_e148580 * locals.var_t2_dn9)), (((-(((locals.var_ps0_dn10 + locals.var_vdsz__blk443_dn10) - locals.var_t1_dn10) / p.p133)) * locals.var_t2) + (assign96010_e148580 * locals.var_t2_dn10)), (((-(((locals.var_ps0_dn11 + locals.var_vdsz__blk443_dn11) - locals.var_t1_dn11) / p.p133)) * locals.var_t2) + (assign96010_e148580 * locals.var_t2_dn11)), (((-(((locals.var_ps0_dn14 + locals.var_vdsz__blk443_dn14) - locals.var_t1_dn14) / p.p133)) * locals.var_t2) + (assign96010_e148580 * locals.var_t2_dn14)),)
    } else {
        (locals.var_qy, locals.var_qy_dn0, locals.var_qy_dn2, locals.var_qy_dn4, locals.var_qy_dn5, locals.var_qy_dn6, locals.var_qy_dn7, locals.var_qy_dn8, locals.var_qy_dn9, locals.var_qy_dn10, locals.var_qy_dn11, locals.var_qy_dn14,)
    }
};
        locals.var_qy = assign96010_e148584;
        locals.var_qy_dn0 = assign96010_e148584_d_n0;
        locals.var_qy_dn2 = assign96010_e148584_d_n2;
        locals.var_qy_dn4 = assign96010_e148584_d_n4;
        locals.var_qy_dn5 = assign96010_e148584_d_n5;
        locals.var_qy_dn6 = assign96010_e148584_d_n6;
        locals.var_qy_dn7 = assign96010_e148584_d_n7;
        locals.var_qy_dn8 = assign96010_e148584_d_n8;
        locals.var_qy_dn9 = assign96010_e148584_d_n9;
        locals.var_qy_dn10 = assign96010_e148584_d_n10;
        locals.var_qy_dn11 = assign96010_e148584_d_n11;
        locals.var_qy_dn14 = assign96010_e148584_d_n14;
        locals.var_qy_rv = 0.0;

        let assign96020_e148587: f64 = if p.p134 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2236 = assign96020_e148587;
        locals.var_guard2236_rv = 0.0;

        let (assign96030_e148598, assign96030_e148598_d_n0, assign96030_e148598_d_n2, assign96030_e148598_d_n4, assign96030_e148598_d_n5, assign96030_e148598_d_n6, assign96030_e148598_d_n7, assign96030_e148598_d_n8, assign96030_e148598_d_n9, assign96030_e148598_d_n10, assign96030_e148598_d_n11, assign96030_e148598_d_n14,) = {
    if ((locals.var_guard2234 == 0.0) && (locals.var_guard2236 != 0.0)) {
        let assign96030_e148595: f64 = (locals.var_cqyb0 * locals.var_vbs);
        let assign96030_e148596: f64 = (locals.var_qy + assign96030_e148595);
        (assign96030_e148596, locals.var_qy_dn0, locals.var_qy_dn2, locals.var_qy_dn4, locals.var_qy_dn5, (locals.var_qy_dn6 + (locals.var_cqyb0 * locals.var_vbs_dn6)), locals.var_qy_dn7, (locals.var_qy_dn8 + (locals.var_cqyb0 * locals.var_vbs_dn8)), (locals.var_qy_dn9 + (locals.var_cqyb0 * locals.var_vbs_dn9)), locals.var_qy_dn10, locals.var_qy_dn11, locals.var_qy_dn14,)
    } else {
        (locals.var_qy, locals.var_qy_dn0, locals.var_qy_dn2, locals.var_qy_dn4, locals.var_qy_dn5, locals.var_qy_dn6, locals.var_qy_dn7, locals.var_qy_dn8, locals.var_qy_dn9, locals.var_qy_dn10, locals.var_qy_dn11, locals.var_qy_dn14,)
    }
};
        locals.var_qy = assign96030_e148598;
        locals.var_qy_dn0 = assign96030_e148598_d_n0;
        locals.var_qy_dn2 = assign96030_e148598_d_n2;
        locals.var_qy_dn4 = assign96030_e148598_d_n4;
        locals.var_qy_dn5 = assign96030_e148598_d_n5;
        locals.var_qy_dn6 = assign96030_e148598_d_n6;
        locals.var_qy_dn7 = assign96030_e148598_d_n7;
        locals.var_qy_dn8 = assign96030_e148598_d_n8;
        locals.var_qy_dn9 = assign96030_e148598_d_n9;
        locals.var_qy_dn10 = assign96030_e148598_d_n10;
        locals.var_qy_dn11 = assign96030_e148598_d_n11;
        locals.var_qy_dn14 = assign96030_e148598_d_n14;
        locals.var_qy_rv = 0.0;

        locals.var_cfd = locals.var_cfrng;
        locals.var_cfd_rv = 0.0;

        locals.var_cfs = locals.var_cfrng;
        locals.var_cfs_rv = 0.0;

        let assign96060_e148604: f64 = (locals.var_vgsei - locals.var_vdsei);
        let assign96060_e148605: f64 = (locals.var_cfd * assign96060_e148604);
        locals.var_qfd = assign96060_e148605;
        locals.var_qfd_dn0 = (locals.var_cfd * (-locals.var_vdsei_dn0));
        locals.var_qfd_dn2 = (locals.var_cfd * (locals.var_vgsei_dn2 - locals.var_vdsei_dn2));
        locals.var_qfd_dn7 = (locals.var_cfd * locals.var_vgsei_dn7);
        locals.var_qfd_rv = 0.0;

        let assign96070_e148608: f64 = (locals.var_cfs * locals.var_vgsei);
        locals.var_qfs = assign96070_e148608;
        locals.var_qfs_dn2 = (locals.var_cfs * locals.var_vgsei_dn2);
        locals.var_qfs_dn7 = (locals.var_cfs * locals.var_vgsei_dn7);
        locals.var_qfs_rv = 0.0;

        let assign96080_e148615: f64 = if ((p.p53 > 0.0) && (locals.var_uc_rth0 != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2237 = assign96080_e148615;
        locals.var_guard2237_rv = 0.0;

        let (assign96090_e148621, assign96090_e148621_d_n0, assign96090_e148621_d_n2, assign96090_e148621_d_n4, assign96090_e148621_d_n5, assign96090_e148621_d_n6, assign96090_e148621_d_n7, assign96090_e148621_d_n8, assign96090_e148621_d_n9, assign96090_e148621_d_n10, assign96090_e148621_d_n11, assign96090_e148621_d_n14,) = {
    if (locals.var_guard2237 != 0.0) {
        let assign96090_e148619: f64 = (locals.var_tratio * locals.var_tratio);
        (assign96090_e148619, ((locals.var_tratio_dn0 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn0)), ((locals.var_tratio_dn2 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn2)), ((locals.var_tratio_dn4 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn4)), ((locals.var_tratio_dn5 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn5)), ((locals.var_tratio_dn6 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn6)), ((locals.var_tratio_dn7 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn7)), ((locals.var_tratio_dn8 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn8)), ((locals.var_tratio_dn9 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn9)), ((locals.var_tratio_dn10 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn10)), ((locals.var_tratio_dn11 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn11)), ((locals.var_tratio_dn14 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign96090_e148621;
        locals.var_t0_dn0 = assign96090_e148621_d_n0;
        locals.var_t0_dn2 = assign96090_e148621_d_n2;
        locals.var_t0_dn4 = assign96090_e148621_d_n4;
        locals.var_t0_dn5 = assign96090_e148621_d_n5;
        locals.var_t0_dn6 = assign96090_e148621_d_n6;
        locals.var_t0_dn7 = assign96090_e148621_d_n7;
        locals.var_t0_dn8 = assign96090_e148621_d_n8;
        locals.var_t0_dn9 = assign96090_e148621_d_n9;
        locals.var_t0_dn10 = assign96090_e148621_d_n10;
        locals.var_t0_dn11 = assign96090_e148621_d_n11;
        locals.var_t0_dn14 = assign96090_e148621_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign96100_e148640, assign96100_e148640_d_n0, assign96100_e148640_d_n2, assign96100_e148640_d_n4, assign96100_e148640_d_n5, assign96100_e148640_d_n6, assign96100_e148640_d_n7, assign96100_e148640_d_n8, assign96100_e148640_d_n9, assign96100_e148640_d_n10, assign96100_e148640_d_n11, assign96100_e148640_d_n14,) = {
    if (locals.var_guard2237 != 0.0) {
        let assign96100_e148626: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign96100_e148629: f64 = (locals.var_eg * locals.var_beta);
        let assign96100_e148630: f64 = (assign96100_e148626 - assign96100_e148629);
        let assign96100_e148633: f64 = (p.p499 * locals.var_log_tratio);
        let assign96100_e148634: f64 = (assign96100_e148630 + assign96100_e148633);
        let assign96100_e148636: f64 = (assign96100_e148634 / locals.var_uc_njd);
        let assign96100_e148637: f64 = (assign96100_e148636).exp();
        let assign96100_e148638: f64 = (locals.var_uc_js0d * assign96100_e148637);
        (assign96100_e148638, (locals.var_uc_js0d * (assign96100_e148637 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p499 * locals.var_log_tratio_dn0)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96100_e148637 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p499 * locals.var_log_tratio_dn2)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96100_e148637 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p499 * locals.var_log_tratio_dn4)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96100_e148637 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p499 * locals.var_log_tratio_dn5)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96100_e148637 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p499 * locals.var_log_tratio_dn6)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96100_e148637 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p499 * locals.var_log_tratio_dn7)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96100_e148637 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p499 * locals.var_log_tratio_dn8)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96100_e148637 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p499 * locals.var_log_tratio_dn9)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96100_e148637 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p499 * locals.var_log_tratio_dn10)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96100_e148637 * (((-((locals.var_eg_dn11 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn11))) + (p.p499 * locals.var_log_tratio_dn11)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96100_e148637 * (((-((locals.var_eg_dn14 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn14))) + (p.p499 * locals.var_log_tratio_dn14)) / locals.var_uc_njd))),)
    } else {
        (locals.var_js, locals.var_js_dn0, locals.var_js_dn2, locals.var_js_dn4, locals.var_js_dn5, locals.var_js_dn6, locals.var_js_dn7, locals.var_js_dn8, locals.var_js_dn9, locals.var_js_dn10, locals.var_js_dn11, locals.var_js_dn14,)
    }
};
        locals.var_js = assign96100_e148640;
        locals.var_js_dn0 = assign96100_e148640_d_n0;
        locals.var_js_dn2 = assign96100_e148640_d_n2;
        locals.var_js_dn4 = assign96100_e148640_d_n4;
        locals.var_js_dn5 = assign96100_e148640_d_n5;
        locals.var_js_dn6 = assign96100_e148640_d_n6;
        locals.var_js_dn7 = assign96100_e148640_d_n7;
        locals.var_js_dn8 = assign96100_e148640_d_n8;
        locals.var_js_dn9 = assign96100_e148640_d_n9;
        locals.var_js_dn10 = assign96100_e148640_d_n10;
        locals.var_js_dn11 = assign96100_e148640_d_n11;
        locals.var_js_dn14 = assign96100_e148640_d_n14;
        locals.var_js_rv = 0.0;

        let (assign96110_e148659, assign96110_e148659_d_n0, assign96110_e148659_d_n2, assign96110_e148659_d_n4, assign96110_e148659_d_n5, assign96110_e148659_d_n6, assign96110_e148659_d_n7, assign96110_e148659_d_n8, assign96110_e148659_d_n9, assign96110_e148659_d_n10, assign96110_e148659_d_n11, assign96110_e148659_d_n14,) = {
    if (locals.var_guard2237 != 0.0) {
        let assign96110_e148645: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign96110_e148648: f64 = (locals.var_eg * locals.var_beta);
        let assign96110_e148649: f64 = (assign96110_e148645 - assign96110_e148648);
        let assign96110_e148652: f64 = (p.p499 * locals.var_log_tratio);
        let assign96110_e148653: f64 = (assign96110_e148649 + assign96110_e148652);
        let assign96110_e148655: f64 = (assign96110_e148653 / p.p497);
        let assign96110_e148656: f64 = (assign96110_e148655).exp();
        let assign96110_e148657: f64 = (locals.var_uc_js0swd * assign96110_e148656);
        (assign96110_e148657, (locals.var_uc_js0swd * (assign96110_e148656 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p499 * locals.var_log_tratio_dn0)) / p.p497))), (locals.var_uc_js0swd * (assign96110_e148656 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p499 * locals.var_log_tratio_dn2)) / p.p497))), (locals.var_uc_js0swd * (assign96110_e148656 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p499 * locals.var_log_tratio_dn4)) / p.p497))), (locals.var_uc_js0swd * (assign96110_e148656 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p499 * locals.var_log_tratio_dn5)) / p.p497))), (locals.var_uc_js0swd * (assign96110_e148656 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p499 * locals.var_log_tratio_dn6)) / p.p497))), (locals.var_uc_js0swd * (assign96110_e148656 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p499 * locals.var_log_tratio_dn7)) / p.p497))), (locals.var_uc_js0swd * (assign96110_e148656 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p499 * locals.var_log_tratio_dn8)) / p.p497))), (locals.var_uc_js0swd * (assign96110_e148656 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p499 * locals.var_log_tratio_dn9)) / p.p497))), (locals.var_uc_js0swd * (assign96110_e148656 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p499 * locals.var_log_tratio_dn10)) / p.p497))), (locals.var_uc_js0swd * (assign96110_e148656 * (((-((locals.var_eg_dn11 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn11))) + (p.p499 * locals.var_log_tratio_dn11)) / p.p497))), (locals.var_uc_js0swd * (assign96110_e148656 * (((-((locals.var_eg_dn14 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn14))) + (p.p499 * locals.var_log_tratio_dn14)) / p.p497))),)
    } else {
        (locals.var_jssw, locals.var_jssw_dn0, locals.var_jssw_dn2, locals.var_jssw_dn4, locals.var_jssw_dn5, locals.var_jssw_dn6, locals.var_jssw_dn7, locals.var_jssw_dn8, locals.var_jssw_dn9, locals.var_jssw_dn10, locals.var_jssw_dn11, locals.var_jssw_dn14,)
    }
};
        locals.var_jssw = assign96110_e148659;
        locals.var_jssw_dn0 = assign96110_e148659_d_n0;
        locals.var_jssw_dn2 = assign96110_e148659_d_n2;
        locals.var_jssw_dn4 = assign96110_e148659_d_n4;
        locals.var_jssw_dn5 = assign96110_e148659_d_n5;
        locals.var_jssw_dn6 = assign96110_e148659_d_n6;
        locals.var_jssw_dn7 = assign96110_e148659_d_n7;
        locals.var_jssw_dn8 = assign96110_e148659_d_n8;
        locals.var_jssw_dn9 = assign96110_e148659_d_n9;
        locals.var_jssw_dn10 = assign96110_e148659_d_n10;
        locals.var_jssw_dn11 = assign96110_e148659_d_n11;
        locals.var_jssw_dn14 = assign96110_e148659_d_n14;
        locals.var_jssw_rv = 0.0;

        let (assign96120_e148678, assign96120_e148678_d_n0, assign96120_e148678_d_n2, assign96120_e148678_d_n4, assign96120_e148678_d_n5, assign96120_e148678_d_n6, assign96120_e148678_d_n7, assign96120_e148678_d_n8, assign96120_e148678_d_n9, assign96120_e148678_d_n10, assign96120_e148678_d_n11, assign96120_e148678_d_n14,) = {
    if (locals.var_guard2237 != 0.0) {
        let assign96120_e148664: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign96120_e148667: f64 = (locals.var_eg * locals.var_beta);
        let assign96120_e148668: f64 = (assign96120_e148664 - assign96120_e148667);
        let assign96120_e148671: f64 = (p.p499 * locals.var_log_tratio);
        let assign96120_e148672: f64 = (assign96120_e148668 + assign96120_e148671);
        let assign96120_e148674: f64 = (assign96120_e148672 / p.p498);
        let assign96120_e148675: f64 = (assign96120_e148674).exp();
        let assign96120_e148676: f64 = (p.p495 * assign96120_e148675);
        (assign96120_e148676, (p.p495 * (assign96120_e148675 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p499 * locals.var_log_tratio_dn0)) / p.p498))), (p.p495 * (assign96120_e148675 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p499 * locals.var_log_tratio_dn2)) / p.p498))), (p.p495 * (assign96120_e148675 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p499 * locals.var_log_tratio_dn4)) / p.p498))), (p.p495 * (assign96120_e148675 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p499 * locals.var_log_tratio_dn5)) / p.p498))), (p.p495 * (assign96120_e148675 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p499 * locals.var_log_tratio_dn6)) / p.p498))), (p.p495 * (assign96120_e148675 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p499 * locals.var_log_tratio_dn7)) / p.p498))), (p.p495 * (assign96120_e148675 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p499 * locals.var_log_tratio_dn8)) / p.p498))), (p.p495 * (assign96120_e148675 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p499 * locals.var_log_tratio_dn9)) / p.p498))), (p.p495 * (assign96120_e148675 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p499 * locals.var_log_tratio_dn10)) / p.p498))), (p.p495 * (assign96120_e148675 * (((-((locals.var_eg_dn11 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn11))) + (p.p499 * locals.var_log_tratio_dn11)) / p.p498))), (p.p495 * (assign96120_e148675 * (((-((locals.var_eg_dn14 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn14))) + (p.p499 * locals.var_log_tratio_dn14)) / p.p498))),)
    } else {
        (locals.var_jsswg, locals.var_jsswg_dn0, locals.var_jsswg_dn2, locals.var_jsswg_dn4, locals.var_jsswg_dn5, locals.var_jsswg_dn6, locals.var_jsswg_dn7, locals.var_jsswg_dn8, locals.var_jsswg_dn9, locals.var_jsswg_dn10, locals.var_jsswg_dn11, locals.var_jsswg_dn14,)
    }
};
        locals.var_jsswg = assign96120_e148678;
        locals.var_jsswg_dn0 = assign96120_e148678_d_n0;
        locals.var_jsswg_dn2 = assign96120_e148678_d_n2;
        locals.var_jsswg_dn4 = assign96120_e148678_d_n4;
        locals.var_jsswg_dn5 = assign96120_e148678_d_n5;
        locals.var_jsswg_dn6 = assign96120_e148678_d_n6;
        locals.var_jsswg_dn7 = assign96120_e148678_d_n7;
        locals.var_jsswg_dn8 = assign96120_e148678_d_n8;
        locals.var_jsswg_dn9 = assign96120_e148678_d_n9;
        locals.var_jsswg_dn10 = assign96120_e148678_d_n10;
        locals.var_jsswg_dn11 = assign96120_e148678_d_n11;
        locals.var_jsswg_dn14 = assign96120_e148678_d_n14;
        locals.var_jsswg_rv = 0.0;

        let (assign96130_e148697, assign96130_e148697_d_n0, assign96130_e148697_d_n2, assign96130_e148697_d_n4, assign96130_e148697_d_n5, assign96130_e148697_d_n6, assign96130_e148697_d_n7, assign96130_e148697_d_n8, assign96130_e148697_d_n9, assign96130_e148697_d_n10, assign96130_e148697_d_n11, assign96130_e148697_d_n14,) = {
    if (locals.var_guard2237 != 0.0) {
        let assign96130_e148683: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign96130_e148686: f64 = (locals.var_eg * locals.var_beta);
        let assign96130_e148687: f64 = (assign96130_e148683 - assign96130_e148686);
        let assign96130_e148690: f64 = (p.p509 * locals.var_log_tratio);
        let assign96130_e148691: f64 = (assign96130_e148687 + assign96130_e148690);
        let assign96130_e148693: f64 = (assign96130_e148691 / locals.var_uc_njd);
        let assign96130_e148694: f64 = (assign96130_e148693).exp();
        let assign96130_e148695: f64 = (locals.var_uc_js0d * assign96130_e148694);
        (assign96130_e148695, (locals.var_uc_js0d * (assign96130_e148694 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p509 * locals.var_log_tratio_dn0)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96130_e148694 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p509 * locals.var_log_tratio_dn2)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96130_e148694 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p509 * locals.var_log_tratio_dn4)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96130_e148694 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p509 * locals.var_log_tratio_dn5)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96130_e148694 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p509 * locals.var_log_tratio_dn6)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96130_e148694 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p509 * locals.var_log_tratio_dn7)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96130_e148694 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p509 * locals.var_log_tratio_dn8)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96130_e148694 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p509 * locals.var_log_tratio_dn9)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96130_e148694 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p509 * locals.var_log_tratio_dn10)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96130_e148694 * (((-((locals.var_eg_dn11 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn11))) + (p.p509 * locals.var_log_tratio_dn11)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96130_e148694 * (((-((locals.var_eg_dn14 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn14))) + (p.p509 * locals.var_log_tratio_dn14)) / locals.var_uc_njd))),)
    } else {
        (locals.var_js2, locals.var_js2_dn0, locals.var_js2_dn2, locals.var_js2_dn4, locals.var_js2_dn5, locals.var_js2_dn6, locals.var_js2_dn7, locals.var_js2_dn8, locals.var_js2_dn9, locals.var_js2_dn10, locals.var_js2_dn11, locals.var_js2_dn14,)
    }
};
        locals.var_js2 = assign96130_e148697;
        locals.var_js2_dn0 = assign96130_e148697_d_n0;
        locals.var_js2_dn2 = assign96130_e148697_d_n2;
        locals.var_js2_dn4 = assign96130_e148697_d_n4;
        locals.var_js2_dn5 = assign96130_e148697_d_n5;
        locals.var_js2_dn6 = assign96130_e148697_d_n6;
        locals.var_js2_dn7 = assign96130_e148697_d_n7;
        locals.var_js2_dn8 = assign96130_e148697_d_n8;
        locals.var_js2_dn9 = assign96130_e148697_d_n9;
        locals.var_js2_dn10 = assign96130_e148697_d_n10;
        locals.var_js2_dn11 = assign96130_e148697_d_n11;
        locals.var_js2_dn14 = assign96130_e148697_d_n14;
        locals.var_js2_rv = 0.0;

        let (assign96140_e148716, assign96140_e148716_d_n0, assign96140_e148716_d_n2, assign96140_e148716_d_n4, assign96140_e148716_d_n5, assign96140_e148716_d_n6, assign96140_e148716_d_n7, assign96140_e148716_d_n8, assign96140_e148716_d_n9, assign96140_e148716_d_n10, assign96140_e148716_d_n11, assign96140_e148716_d_n14,) = {
    if (locals.var_guard2237 != 0.0) {
        let assign96140_e148702: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign96140_e148705: f64 = (locals.var_eg * locals.var_beta);
        let assign96140_e148706: f64 = (assign96140_e148702 - assign96140_e148705);
        let assign96140_e148709: f64 = (p.p509 * locals.var_log_tratio);
        let assign96140_e148710: f64 = (assign96140_e148706 + assign96140_e148709);
        let assign96140_e148712: f64 = (assign96140_e148710 / p.p497);
        let assign96140_e148713: f64 = (assign96140_e148712).exp();
        let assign96140_e148714: f64 = (locals.var_uc_js0swd * assign96140_e148713);
        (assign96140_e148714, (locals.var_uc_js0swd * (assign96140_e148713 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p509 * locals.var_log_tratio_dn0)) / p.p497))), (locals.var_uc_js0swd * (assign96140_e148713 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p509 * locals.var_log_tratio_dn2)) / p.p497))), (locals.var_uc_js0swd * (assign96140_e148713 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p509 * locals.var_log_tratio_dn4)) / p.p497))), (locals.var_uc_js0swd * (assign96140_e148713 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p509 * locals.var_log_tratio_dn5)) / p.p497))), (locals.var_uc_js0swd * (assign96140_e148713 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p509 * locals.var_log_tratio_dn6)) / p.p497))), (locals.var_uc_js0swd * (assign96140_e148713 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p509 * locals.var_log_tratio_dn7)) / p.p497))), (locals.var_uc_js0swd * (assign96140_e148713 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p509 * locals.var_log_tratio_dn8)) / p.p497))), (locals.var_uc_js0swd * (assign96140_e148713 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p509 * locals.var_log_tratio_dn9)) / p.p497))), (locals.var_uc_js0swd * (assign96140_e148713 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p509 * locals.var_log_tratio_dn10)) / p.p497))), (locals.var_uc_js0swd * (assign96140_e148713 * (((-((locals.var_eg_dn11 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn11))) + (p.p509 * locals.var_log_tratio_dn11)) / p.p497))), (locals.var_uc_js0swd * (assign96140_e148713 * (((-((locals.var_eg_dn14 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn14))) + (p.p509 * locals.var_log_tratio_dn14)) / p.p497))),)
    } else {
        (locals.var_jssw2, locals.var_jssw2_dn0, locals.var_jssw2_dn2, locals.var_jssw2_dn4, locals.var_jssw2_dn5, locals.var_jssw2_dn6, locals.var_jssw2_dn7, locals.var_jssw2_dn8, locals.var_jssw2_dn9, locals.var_jssw2_dn10, locals.var_jssw2_dn11, locals.var_jssw2_dn14,)
    }
};
        locals.var_jssw2 = assign96140_e148716;
        locals.var_jssw2_dn0 = assign96140_e148716_d_n0;
        locals.var_jssw2_dn2 = assign96140_e148716_d_n2;
        locals.var_jssw2_dn4 = assign96140_e148716_d_n4;
        locals.var_jssw2_dn5 = assign96140_e148716_d_n5;
        locals.var_jssw2_dn6 = assign96140_e148716_d_n6;
        locals.var_jssw2_dn7 = assign96140_e148716_d_n7;
        locals.var_jssw2_dn8 = assign96140_e148716_d_n8;
        locals.var_jssw2_dn9 = assign96140_e148716_d_n9;
        locals.var_jssw2_dn10 = assign96140_e148716_d_n10;
        locals.var_jssw2_dn11 = assign96140_e148716_d_n11;
        locals.var_jssw2_dn14 = assign96140_e148716_d_n14;
        locals.var_jssw2_rv = 0.0;

        let (assign96150_e148735, assign96150_e148735_d_n0, assign96150_e148735_d_n2, assign96150_e148735_d_n4, assign96150_e148735_d_n5, assign96150_e148735_d_n6, assign96150_e148735_d_n7, assign96150_e148735_d_n8, assign96150_e148735_d_n9, assign96150_e148735_d_n10, assign96150_e148735_d_n11, assign96150_e148735_d_n14,) = {
    if (locals.var_guard2237 != 0.0) {
        let assign96150_e148721: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign96150_e148724: f64 = (locals.var_eg * locals.var_beta);
        let assign96150_e148725: f64 = (assign96150_e148721 - assign96150_e148724);
        let assign96150_e148728: f64 = (p.p509 * locals.var_log_tratio);
        let assign96150_e148729: f64 = (assign96150_e148725 + assign96150_e148728);
        let assign96150_e148731: f64 = (assign96150_e148729 / p.p498);
        let assign96150_e148732: f64 = (assign96150_e148731).exp();
        let assign96150_e148733: f64 = (p.p495 * assign96150_e148732);
        (assign96150_e148733, (p.p495 * (assign96150_e148732 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p509 * locals.var_log_tratio_dn0)) / p.p498))), (p.p495 * (assign96150_e148732 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p509 * locals.var_log_tratio_dn2)) / p.p498))), (p.p495 * (assign96150_e148732 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p509 * locals.var_log_tratio_dn4)) / p.p498))), (p.p495 * (assign96150_e148732 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p509 * locals.var_log_tratio_dn5)) / p.p498))), (p.p495 * (assign96150_e148732 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p509 * locals.var_log_tratio_dn6)) / p.p498))), (p.p495 * (assign96150_e148732 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p509 * locals.var_log_tratio_dn7)) / p.p498))), (p.p495 * (assign96150_e148732 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p509 * locals.var_log_tratio_dn8)) / p.p498))), (p.p495 * (assign96150_e148732 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p509 * locals.var_log_tratio_dn9)) / p.p498))), (p.p495 * (assign96150_e148732 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p509 * locals.var_log_tratio_dn10)) / p.p498))), (p.p495 * (assign96150_e148732 * (((-((locals.var_eg_dn11 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn11))) + (p.p509 * locals.var_log_tratio_dn11)) / p.p498))), (p.p495 * (assign96150_e148732 * (((-((locals.var_eg_dn14 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn14))) + (p.p509 * locals.var_log_tratio_dn14)) / p.p498))),)
    } else {
        (locals.var_jsswg2, locals.var_jsswg2_dn0, locals.var_jsswg2_dn2, locals.var_jsswg2_dn4, locals.var_jsswg2_dn5, locals.var_jsswg2_dn6, locals.var_jsswg2_dn7, locals.var_jsswg2_dn8, locals.var_jsswg2_dn9, locals.var_jsswg2_dn10, locals.var_jsswg2_dn11, locals.var_jsswg2_dn14,)
    }
};
        locals.var_jsswg2 = assign96150_e148735;
        locals.var_jsswg2_dn0 = assign96150_e148735_d_n0;
        locals.var_jsswg2_dn2 = assign96150_e148735_d_n2;
        locals.var_jsswg2_dn4 = assign96150_e148735_d_n4;
        locals.var_jsswg2_dn5 = assign96150_e148735_d_n5;
        locals.var_jsswg2_dn6 = assign96150_e148735_d_n6;
        locals.var_jsswg2_dn7 = assign96150_e148735_d_n7;
        locals.var_jsswg2_dn8 = assign96150_e148735_d_n8;
        locals.var_jsswg2_dn9 = assign96150_e148735_d_n9;
        locals.var_jsswg2_dn10 = assign96150_e148735_d_n10;
        locals.var_jsswg2_dn11 = assign96150_e148735_d_n11;
        locals.var_jsswg2_dn14 = assign96150_e148735_d_n14;
        locals.var_jsswg2_rv = 0.0;

        let assign96160_e148738: f64 = if p.p48 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2238 = assign96160_e148738;
        locals.var_guard2238_rv = 0.0;

        let assign96170_e148741: f64 = if p.p15 > locals.var_weff_nf { 1.0 } else { 0.0 };
        locals.var_guard2239 = assign96170_e148741;
        locals.var_guard2239_rv = 0.0;

        let (assign96180_e148751, assign96180_e148751_d_n0, assign96180_e148751_d_n2, assign96180_e148751_d_n4, assign96180_e148751_d_n5, assign96180_e148751_d_n6, assign96180_e148751_d_n7, assign96180_e148751_d_n8, assign96180_e148751_d_n9, assign96180_e148751_d_n10, assign96180_e148751_d_n11, assign96180_e148751_d_n14,) = {
    if (((locals.var_guard2237 != 0.0) && (locals.var_guard2238 != 0.0)) && (locals.var_guard2239 != 0.0)) {
        let assign96180_e148749: f64 = (p.p13 * locals.var_js);
        (assign96180_e148749, (p.p13 * locals.var_js_dn0), (p.p13 * locals.var_js_dn2), (p.p13 * locals.var_js_dn4), (p.p13 * locals.var_js_dn5), (p.p13 * locals.var_js_dn6), (p.p13 * locals.var_js_dn7), (p.p13 * locals.var_js_dn8), (p.p13 * locals.var_js_dn9), (p.p13 * locals.var_js_dn10), (p.p13 * locals.var_js_dn11), (p.p13 * locals.var_js_dn14),)
    } else {
        (locals.var_isbd_btm, locals.var_isbd_btm_dn0, locals.var_isbd_btm_dn2, locals.var_isbd_btm_dn4, locals.var_isbd_btm_dn5, locals.var_isbd_btm_dn6, locals.var_isbd_btm_dn7, locals.var_isbd_btm_dn8, locals.var_isbd_btm_dn9, locals.var_isbd_btm_dn10, locals.var_isbd_btm_dn11, locals.var_isbd_btm_dn14,)
    }
};
        locals.var_isbd_btm = assign96180_e148751;
        locals.var_isbd_btm_dn0 = assign96180_e148751_d_n0;
        locals.var_isbd_btm_dn2 = assign96180_e148751_d_n2;
        locals.var_isbd_btm_dn4 = assign96180_e148751_d_n4;
        locals.var_isbd_btm_dn5 = assign96180_e148751_d_n5;
        locals.var_isbd_btm_dn6 = assign96180_e148751_d_n6;
        locals.var_isbd_btm_dn7 = assign96180_e148751_d_n7;
        locals.var_isbd_btm_dn8 = assign96180_e148751_d_n8;
        locals.var_isbd_btm_dn9 = assign96180_e148751_d_n9;
        locals.var_isbd_btm_dn10 = assign96180_e148751_d_n10;
        locals.var_isbd_btm_dn11 = assign96180_e148751_d_n11;
        locals.var_isbd_btm_dn14 = assign96180_e148751_d_n14;
        locals.var_isbd_btm_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_373(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign96190_e148761, assign96190_e148761_d_n0, assign96190_e148761_d_n2, assign96190_e148761_d_n4, assign96190_e148761_d_n5, assign96190_e148761_d_n6, assign96190_e148761_d_n7, assign96190_e148761_d_n8, assign96190_e148761_d_n9, assign96190_e148761_d_n10, assign96190_e148761_d_n11, assign96190_e148761_d_n14,) = {
    if (((locals.var_guard2237 != 0.0) && (locals.var_guard2238 != 0.0)) && (locals.var_guard2239 != 0.0)) {
        let assign96190_e148759: f64 = (p.p13 * locals.var_js2);
        (assign96190_e148759, (p.p13 * locals.var_js2_dn0), (p.p13 * locals.var_js2_dn2), (p.p13 * locals.var_js2_dn4), (p.p13 * locals.var_js2_dn5), (p.p13 * locals.var_js2_dn6), (p.p13 * locals.var_js2_dn7), (p.p13 * locals.var_js2_dn8), (p.p13 * locals.var_js2_dn9), (p.p13 * locals.var_js2_dn10), (p.p13 * locals.var_js2_dn11), (p.p13 * locals.var_js2_dn14),)
    } else {
        (locals.var_isbd2_btm, locals.var_isbd2_btm_dn0, locals.var_isbd2_btm_dn2, locals.var_isbd2_btm_dn4, locals.var_isbd2_btm_dn5, locals.var_isbd2_btm_dn6, locals.var_isbd2_btm_dn7, locals.var_isbd2_btm_dn8, locals.var_isbd2_btm_dn9, locals.var_isbd2_btm_dn10, locals.var_isbd2_btm_dn11, locals.var_isbd2_btm_dn14,)
    }
};
        locals.var_isbd2_btm = assign96190_e148761;
        locals.var_isbd2_btm_dn0 = assign96190_e148761_d_n0;
        locals.var_isbd2_btm_dn2 = assign96190_e148761_d_n2;
        locals.var_isbd2_btm_dn4 = assign96190_e148761_d_n4;
        locals.var_isbd2_btm_dn5 = assign96190_e148761_d_n5;
        locals.var_isbd2_btm_dn6 = assign96190_e148761_d_n6;
        locals.var_isbd2_btm_dn7 = assign96190_e148761_d_n7;
        locals.var_isbd2_btm_dn8 = assign96190_e148761_d_n8;
        locals.var_isbd2_btm_dn9 = assign96190_e148761_d_n9;
        locals.var_isbd2_btm_dn10 = assign96190_e148761_d_n10;
        locals.var_isbd2_btm_dn11 = assign96190_e148761_d_n11;
        locals.var_isbd2_btm_dn14 = assign96190_e148761_d_n14;
        locals.var_isbd2_btm_rv = 0.0;

        let (assign96200_e148773, assign96200_e148773_d_n0, assign96200_e148773_d_n2, assign96200_e148773_d_n4, assign96200_e148773_d_n5, assign96200_e148773_d_n6, assign96200_e148773_d_n7, assign96200_e148773_d_n8, assign96200_e148773_d_n9, assign96200_e148773_d_n10, assign96200_e148773_d_n11, assign96200_e148773_d_n14,) = {
    if (((locals.var_guard2237 != 0.0) && (locals.var_guard2238 != 0.0)) && (locals.var_guard2239 != 0.0)) {
        let assign96200_e148769: f64 = (p.p15 - locals.var_weff_nf);
        let assign96200_e148771: f64 = (assign96200_e148769 * locals.var_jssw);
        (assign96200_e148771, (assign96200_e148769 * locals.var_jssw_dn0), (assign96200_e148769 * locals.var_jssw_dn2), (assign96200_e148769 * locals.var_jssw_dn4), (assign96200_e148769 * locals.var_jssw_dn5), (assign96200_e148769 * locals.var_jssw_dn6), (assign96200_e148769 * locals.var_jssw_dn7), (assign96200_e148769 * locals.var_jssw_dn8), (assign96200_e148769 * locals.var_jssw_dn9), (assign96200_e148769 * locals.var_jssw_dn10), (assign96200_e148769 * locals.var_jssw_dn11), (assign96200_e148769 * locals.var_jssw_dn14),)
    } else {
        (locals.var_isbd_sws, locals.var_isbd_sws_dn0, locals.var_isbd_sws_dn2, locals.var_isbd_sws_dn4, locals.var_isbd_sws_dn5, locals.var_isbd_sws_dn6, locals.var_isbd_sws_dn7, locals.var_isbd_sws_dn8, locals.var_isbd_sws_dn9, locals.var_isbd_sws_dn10, locals.var_isbd_sws_dn11, locals.var_isbd_sws_dn14,)
    }
};
        locals.var_isbd_sws = assign96200_e148773;
        locals.var_isbd_sws_dn0 = assign96200_e148773_d_n0;
        locals.var_isbd_sws_dn2 = assign96200_e148773_d_n2;
        locals.var_isbd_sws_dn4 = assign96200_e148773_d_n4;
        locals.var_isbd_sws_dn5 = assign96200_e148773_d_n5;
        locals.var_isbd_sws_dn6 = assign96200_e148773_d_n6;
        locals.var_isbd_sws_dn7 = assign96200_e148773_d_n7;
        locals.var_isbd_sws_dn8 = assign96200_e148773_d_n8;
        locals.var_isbd_sws_dn9 = assign96200_e148773_d_n9;
        locals.var_isbd_sws_dn10 = assign96200_e148773_d_n10;
        locals.var_isbd_sws_dn11 = assign96200_e148773_d_n11;
        locals.var_isbd_sws_dn14 = assign96200_e148773_d_n14;
        locals.var_isbd_sws_rv = 0.0;

        let (assign96210_e148785, assign96210_e148785_d_n0, assign96210_e148785_d_n2, assign96210_e148785_d_n4, assign96210_e148785_d_n5, assign96210_e148785_d_n6, assign96210_e148785_d_n7, assign96210_e148785_d_n8, assign96210_e148785_d_n9, assign96210_e148785_d_n10, assign96210_e148785_d_n11, assign96210_e148785_d_n14,) = {
    if (((locals.var_guard2237 != 0.0) && (locals.var_guard2238 != 0.0)) && (locals.var_guard2239 != 0.0)) {
        let assign96210_e148781: f64 = (p.p15 - locals.var_weff_nf);
        let assign96210_e148783: f64 = (assign96210_e148781 * locals.var_jssw2);
        (assign96210_e148783, (assign96210_e148781 * locals.var_jssw2_dn0), (assign96210_e148781 * locals.var_jssw2_dn2), (assign96210_e148781 * locals.var_jssw2_dn4), (assign96210_e148781 * locals.var_jssw2_dn5), (assign96210_e148781 * locals.var_jssw2_dn6), (assign96210_e148781 * locals.var_jssw2_dn7), (assign96210_e148781 * locals.var_jssw2_dn8), (assign96210_e148781 * locals.var_jssw2_dn9), (assign96210_e148781 * locals.var_jssw2_dn10), (assign96210_e148781 * locals.var_jssw2_dn11), (assign96210_e148781 * locals.var_jssw2_dn14),)
    } else {
        (locals.var_isbd2_sws, locals.var_isbd2_sws_dn0, locals.var_isbd2_sws_dn2, locals.var_isbd2_sws_dn4, locals.var_isbd2_sws_dn5, locals.var_isbd2_sws_dn6, locals.var_isbd2_sws_dn7, locals.var_isbd2_sws_dn8, locals.var_isbd2_sws_dn9, locals.var_isbd2_sws_dn10, locals.var_isbd2_sws_dn11, locals.var_isbd2_sws_dn14,)
    }
};
        locals.var_isbd2_sws = assign96210_e148785;
        locals.var_isbd2_sws_dn0 = assign96210_e148785_d_n0;
        locals.var_isbd2_sws_dn2 = assign96210_e148785_d_n2;
        locals.var_isbd2_sws_dn4 = assign96210_e148785_d_n4;
        locals.var_isbd2_sws_dn5 = assign96210_e148785_d_n5;
        locals.var_isbd2_sws_dn6 = assign96210_e148785_d_n6;
        locals.var_isbd2_sws_dn7 = assign96210_e148785_d_n7;
        locals.var_isbd2_sws_dn8 = assign96210_e148785_d_n8;
        locals.var_isbd2_sws_dn9 = assign96210_e148785_d_n9;
        locals.var_isbd2_sws_dn10 = assign96210_e148785_d_n10;
        locals.var_isbd2_sws_dn11 = assign96210_e148785_d_n11;
        locals.var_isbd2_sws_dn14 = assign96210_e148785_d_n14;
        locals.var_isbd2_sws_rv = 0.0;

        let (assign96220_e148795, assign96220_e148795_d_n0, assign96220_e148795_d_n2, assign96220_e148795_d_n4, assign96220_e148795_d_n5, assign96220_e148795_d_n6, assign96220_e148795_d_n7, assign96220_e148795_d_n8, assign96220_e148795_d_n9, assign96220_e148795_d_n10, assign96220_e148795_d_n11, assign96220_e148795_d_n14,) = {
    if (((locals.var_guard2237 != 0.0) && (locals.var_guard2238 != 0.0)) && (locals.var_guard2239 != 0.0)) {
        let assign96220_e148793: f64 = (locals.var_weff_nf * locals.var_jsswg);
        (assign96220_e148793, (locals.var_weff_nf * locals.var_jsswg_dn0), (locals.var_weff_nf * locals.var_jsswg_dn2), (locals.var_weff_nf * locals.var_jsswg_dn4), (locals.var_weff_nf * locals.var_jsswg_dn5), (locals.var_weff_nf * locals.var_jsswg_dn6), (locals.var_weff_nf * locals.var_jsswg_dn7), (locals.var_weff_nf * locals.var_jsswg_dn8), (locals.var_weff_nf * locals.var_jsswg_dn9), (locals.var_weff_nf * locals.var_jsswg_dn10), (locals.var_weff_nf * locals.var_jsswg_dn11), (locals.var_weff_nf * locals.var_jsswg_dn14),)
    } else {
        (locals.var_isbd_swg, locals.var_isbd_swg_dn0, locals.var_isbd_swg_dn2, locals.var_isbd_swg_dn4, locals.var_isbd_swg_dn5, locals.var_isbd_swg_dn6, locals.var_isbd_swg_dn7, locals.var_isbd_swg_dn8, locals.var_isbd_swg_dn9, locals.var_isbd_swg_dn10, locals.var_isbd_swg_dn11, locals.var_isbd_swg_dn14,)
    }
};
        locals.var_isbd_swg = assign96220_e148795;
        locals.var_isbd_swg_dn0 = assign96220_e148795_d_n0;
        locals.var_isbd_swg_dn2 = assign96220_e148795_d_n2;
        locals.var_isbd_swg_dn4 = assign96220_e148795_d_n4;
        locals.var_isbd_swg_dn5 = assign96220_e148795_d_n5;
        locals.var_isbd_swg_dn6 = assign96220_e148795_d_n6;
        locals.var_isbd_swg_dn7 = assign96220_e148795_d_n7;
        locals.var_isbd_swg_dn8 = assign96220_e148795_d_n8;
        locals.var_isbd_swg_dn9 = assign96220_e148795_d_n9;
        locals.var_isbd_swg_dn10 = assign96220_e148795_d_n10;
        locals.var_isbd_swg_dn11 = assign96220_e148795_d_n11;
        locals.var_isbd_swg_dn14 = assign96220_e148795_d_n14;
        locals.var_isbd_swg_rv = 0.0;

        let (assign96230_e148805, assign96230_e148805_d_n0, assign96230_e148805_d_n2, assign96230_e148805_d_n4, assign96230_e148805_d_n5, assign96230_e148805_d_n6, assign96230_e148805_d_n7, assign96230_e148805_d_n8, assign96230_e148805_d_n9, assign96230_e148805_d_n10, assign96230_e148805_d_n11, assign96230_e148805_d_n14,) = {
    if (((locals.var_guard2237 != 0.0) && (locals.var_guard2238 != 0.0)) && (locals.var_guard2239 != 0.0)) {
        let assign96230_e148803: f64 = (locals.var_weff_nf * locals.var_jsswg2);
        (assign96230_e148803, (locals.var_weff_nf * locals.var_jsswg2_dn0), (locals.var_weff_nf * locals.var_jsswg2_dn2), (locals.var_weff_nf * locals.var_jsswg2_dn4), (locals.var_weff_nf * locals.var_jsswg2_dn5), (locals.var_weff_nf * locals.var_jsswg2_dn6), (locals.var_weff_nf * locals.var_jsswg2_dn7), (locals.var_weff_nf * locals.var_jsswg2_dn8), (locals.var_weff_nf * locals.var_jsswg2_dn9), (locals.var_weff_nf * locals.var_jsswg2_dn10), (locals.var_weff_nf * locals.var_jsswg2_dn11), (locals.var_weff_nf * locals.var_jsswg2_dn14),)
    } else {
        (locals.var_isbd2_swg, locals.var_isbd2_swg_dn0, locals.var_isbd2_swg_dn2, locals.var_isbd2_swg_dn4, locals.var_isbd2_swg_dn5, locals.var_isbd2_swg_dn6, locals.var_isbd2_swg_dn7, locals.var_isbd2_swg_dn8, locals.var_isbd2_swg_dn9, locals.var_isbd2_swg_dn10, locals.var_isbd2_swg_dn11, locals.var_isbd2_swg_dn14,)
    }
};
        locals.var_isbd2_swg = assign96230_e148805;
        locals.var_isbd2_swg_dn0 = assign96230_e148805_d_n0;
        locals.var_isbd2_swg_dn2 = assign96230_e148805_d_n2;
        locals.var_isbd2_swg_dn4 = assign96230_e148805_d_n4;
        locals.var_isbd2_swg_dn5 = assign96230_e148805_d_n5;
        locals.var_isbd2_swg_dn6 = assign96230_e148805_d_n6;
        locals.var_isbd2_swg_dn7 = assign96230_e148805_d_n7;
        locals.var_isbd2_swg_dn8 = assign96230_e148805_d_n8;
        locals.var_isbd2_swg_dn9 = assign96230_e148805_d_n9;
        locals.var_isbd2_swg_dn10 = assign96230_e148805_d_n10;
        locals.var_isbd2_swg_dn11 = assign96230_e148805_d_n11;
        locals.var_isbd2_swg_dn14 = assign96230_e148805_d_n14;
        locals.var_isbd2_swg_rv = 0.0;

        let (assign96240_e148816, assign96240_e148816_d_n0, assign96240_e148816_d_n2, assign96240_e148816_d_n4, assign96240_e148816_d_n5, assign96240_e148816_d_n6, assign96240_e148816_d_n7, assign96240_e148816_d_n8, assign96240_e148816_d_n9, assign96240_e148816_d_n10, assign96240_e148816_d_n11, assign96240_e148816_d_n14,) = {
    if (((locals.var_guard2237 != 0.0) && (locals.var_guard2238 != 0.0)) && (locals.var_guard2239 == 0.0)) {
        let assign96240_e148814: f64 = (p.p13 * locals.var_js);
        (assign96240_e148814, (p.p13 * locals.var_js_dn0), (p.p13 * locals.var_js_dn2), (p.p13 * locals.var_js_dn4), (p.p13 * locals.var_js_dn5), (p.p13 * locals.var_js_dn6), (p.p13 * locals.var_js_dn7), (p.p13 * locals.var_js_dn8), (p.p13 * locals.var_js_dn9), (p.p13 * locals.var_js_dn10), (p.p13 * locals.var_js_dn11), (p.p13 * locals.var_js_dn14),)
    } else {
        (locals.var_isbd_btm, locals.var_isbd_btm_dn0, locals.var_isbd_btm_dn2, locals.var_isbd_btm_dn4, locals.var_isbd_btm_dn5, locals.var_isbd_btm_dn6, locals.var_isbd_btm_dn7, locals.var_isbd_btm_dn8, locals.var_isbd_btm_dn9, locals.var_isbd_btm_dn10, locals.var_isbd_btm_dn11, locals.var_isbd_btm_dn14,)
    }
};
        locals.var_isbd_btm = assign96240_e148816;
        locals.var_isbd_btm_dn0 = assign96240_e148816_d_n0;
        locals.var_isbd_btm_dn2 = assign96240_e148816_d_n2;
        locals.var_isbd_btm_dn4 = assign96240_e148816_d_n4;
        locals.var_isbd_btm_dn5 = assign96240_e148816_d_n5;
        locals.var_isbd_btm_dn6 = assign96240_e148816_d_n6;
        locals.var_isbd_btm_dn7 = assign96240_e148816_d_n7;
        locals.var_isbd_btm_dn8 = assign96240_e148816_d_n8;
        locals.var_isbd_btm_dn9 = assign96240_e148816_d_n9;
        locals.var_isbd_btm_dn10 = assign96240_e148816_d_n10;
        locals.var_isbd_btm_dn11 = assign96240_e148816_d_n11;
        locals.var_isbd_btm_dn14 = assign96240_e148816_d_n14;
        locals.var_isbd_btm_rv = 0.0;

        let (assign96250_e148827, assign96250_e148827_d_n0, assign96250_e148827_d_n2, assign96250_e148827_d_n4, assign96250_e148827_d_n5, assign96250_e148827_d_n6, assign96250_e148827_d_n7, assign96250_e148827_d_n8, assign96250_e148827_d_n9, assign96250_e148827_d_n10, assign96250_e148827_d_n11, assign96250_e148827_d_n14,) = {
    if (((locals.var_guard2237 != 0.0) && (locals.var_guard2238 != 0.0)) && (locals.var_guard2239 == 0.0)) {
        let assign96250_e148825: f64 = (p.p13 * locals.var_js2);
        (assign96250_e148825, (p.p13 * locals.var_js2_dn0), (p.p13 * locals.var_js2_dn2), (p.p13 * locals.var_js2_dn4), (p.p13 * locals.var_js2_dn5), (p.p13 * locals.var_js2_dn6), (p.p13 * locals.var_js2_dn7), (p.p13 * locals.var_js2_dn8), (p.p13 * locals.var_js2_dn9), (p.p13 * locals.var_js2_dn10), (p.p13 * locals.var_js2_dn11), (p.p13 * locals.var_js2_dn14),)
    } else {
        (locals.var_isbd2_btm, locals.var_isbd2_btm_dn0, locals.var_isbd2_btm_dn2, locals.var_isbd2_btm_dn4, locals.var_isbd2_btm_dn5, locals.var_isbd2_btm_dn6, locals.var_isbd2_btm_dn7, locals.var_isbd2_btm_dn8, locals.var_isbd2_btm_dn9, locals.var_isbd2_btm_dn10, locals.var_isbd2_btm_dn11, locals.var_isbd2_btm_dn14,)
    }
};
        locals.var_isbd2_btm = assign96250_e148827;
        locals.var_isbd2_btm_dn0 = assign96250_e148827_d_n0;
        locals.var_isbd2_btm_dn2 = assign96250_e148827_d_n2;
        locals.var_isbd2_btm_dn4 = assign96250_e148827_d_n4;
        locals.var_isbd2_btm_dn5 = assign96250_e148827_d_n5;
        locals.var_isbd2_btm_dn6 = assign96250_e148827_d_n6;
        locals.var_isbd2_btm_dn7 = assign96250_e148827_d_n7;
        locals.var_isbd2_btm_dn8 = assign96250_e148827_d_n8;
        locals.var_isbd2_btm_dn9 = assign96250_e148827_d_n9;
        locals.var_isbd2_btm_dn10 = assign96250_e148827_d_n10;
        locals.var_isbd2_btm_dn11 = assign96250_e148827_d_n11;
        locals.var_isbd2_btm_dn14 = assign96250_e148827_d_n14;
        locals.var_isbd2_btm_rv = 0.0;

        let (assign96260_e148836, assign96260_e148836_d_n0, assign96260_e148836_d_n2, assign96260_e148836_d_n4, assign96260_e148836_d_n5, assign96260_e148836_d_n6, assign96260_e148836_d_n7, assign96260_e148836_d_n8, assign96260_e148836_d_n9, assign96260_e148836_d_n10, assign96260_e148836_d_n11, assign96260_e148836_d_n14,) = {
    if (((locals.var_guard2237 != 0.0) && (locals.var_guard2238 != 0.0)) && (locals.var_guard2239 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isbd_sws, locals.var_isbd_sws_dn0, locals.var_isbd_sws_dn2, locals.var_isbd_sws_dn4, locals.var_isbd_sws_dn5, locals.var_isbd_sws_dn6, locals.var_isbd_sws_dn7, locals.var_isbd_sws_dn8, locals.var_isbd_sws_dn9, locals.var_isbd_sws_dn10, locals.var_isbd_sws_dn11, locals.var_isbd_sws_dn14,)
    }
};
        locals.var_isbd_sws = assign96260_e148836;
        locals.var_isbd_sws_dn0 = assign96260_e148836_d_n0;
        locals.var_isbd_sws_dn2 = assign96260_e148836_d_n2;
        locals.var_isbd_sws_dn4 = assign96260_e148836_d_n4;
        locals.var_isbd_sws_dn5 = assign96260_e148836_d_n5;
        locals.var_isbd_sws_dn6 = assign96260_e148836_d_n6;
        locals.var_isbd_sws_dn7 = assign96260_e148836_d_n7;
        locals.var_isbd_sws_dn8 = assign96260_e148836_d_n8;
        locals.var_isbd_sws_dn9 = assign96260_e148836_d_n9;
        locals.var_isbd_sws_dn10 = assign96260_e148836_d_n10;
        locals.var_isbd_sws_dn11 = assign96260_e148836_d_n11;
        locals.var_isbd_sws_dn14 = assign96260_e148836_d_n14;
        locals.var_isbd_sws_rv = 0.0;

        let (assign96270_e148845, assign96270_e148845_d_n0, assign96270_e148845_d_n2, assign96270_e148845_d_n4, assign96270_e148845_d_n5, assign96270_e148845_d_n6, assign96270_e148845_d_n7, assign96270_e148845_d_n8, assign96270_e148845_d_n9, assign96270_e148845_d_n10, assign96270_e148845_d_n11, assign96270_e148845_d_n14,) = {
    if (((locals.var_guard2237 != 0.0) && (locals.var_guard2238 != 0.0)) && (locals.var_guard2239 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isbd2_sws, locals.var_isbd2_sws_dn0, locals.var_isbd2_sws_dn2, locals.var_isbd2_sws_dn4, locals.var_isbd2_sws_dn5, locals.var_isbd2_sws_dn6, locals.var_isbd2_sws_dn7, locals.var_isbd2_sws_dn8, locals.var_isbd2_sws_dn9, locals.var_isbd2_sws_dn10, locals.var_isbd2_sws_dn11, locals.var_isbd2_sws_dn14,)
    }
};
        locals.var_isbd2_sws = assign96270_e148845;
        locals.var_isbd2_sws_dn0 = assign96270_e148845_d_n0;
        locals.var_isbd2_sws_dn2 = assign96270_e148845_d_n2;
        locals.var_isbd2_sws_dn4 = assign96270_e148845_d_n4;
        locals.var_isbd2_sws_dn5 = assign96270_e148845_d_n5;
        locals.var_isbd2_sws_dn6 = assign96270_e148845_d_n6;
        locals.var_isbd2_sws_dn7 = assign96270_e148845_d_n7;
        locals.var_isbd2_sws_dn8 = assign96270_e148845_d_n8;
        locals.var_isbd2_sws_dn9 = assign96270_e148845_d_n9;
        locals.var_isbd2_sws_dn10 = assign96270_e148845_d_n10;
        locals.var_isbd2_sws_dn11 = assign96270_e148845_d_n11;
        locals.var_isbd2_sws_dn14 = assign96270_e148845_d_n14;
        locals.var_isbd2_sws_rv = 0.0;

        let (assign96280_e148856, assign96280_e148856_d_n0, assign96280_e148856_d_n2, assign96280_e148856_d_n4, assign96280_e148856_d_n5, assign96280_e148856_d_n6, assign96280_e148856_d_n7, assign96280_e148856_d_n8, assign96280_e148856_d_n9, assign96280_e148856_d_n10, assign96280_e148856_d_n11, assign96280_e148856_d_n14,) = {
    if (((locals.var_guard2237 != 0.0) && (locals.var_guard2238 != 0.0)) && (locals.var_guard2239 == 0.0)) {
        let assign96280_e148854: f64 = (p.p15 * locals.var_jsswg);
        (assign96280_e148854, (p.p15 * locals.var_jsswg_dn0), (p.p15 * locals.var_jsswg_dn2), (p.p15 * locals.var_jsswg_dn4), (p.p15 * locals.var_jsswg_dn5), (p.p15 * locals.var_jsswg_dn6), (p.p15 * locals.var_jsswg_dn7), (p.p15 * locals.var_jsswg_dn8), (p.p15 * locals.var_jsswg_dn9), (p.p15 * locals.var_jsswg_dn10), (p.p15 * locals.var_jsswg_dn11), (p.p15 * locals.var_jsswg_dn14),)
    } else {
        (locals.var_isbd_swg, locals.var_isbd_swg_dn0, locals.var_isbd_swg_dn2, locals.var_isbd_swg_dn4, locals.var_isbd_swg_dn5, locals.var_isbd_swg_dn6, locals.var_isbd_swg_dn7, locals.var_isbd_swg_dn8, locals.var_isbd_swg_dn9, locals.var_isbd_swg_dn10, locals.var_isbd_swg_dn11, locals.var_isbd_swg_dn14,)
    }
};
        locals.var_isbd_swg = assign96280_e148856;
        locals.var_isbd_swg_dn0 = assign96280_e148856_d_n0;
        locals.var_isbd_swg_dn2 = assign96280_e148856_d_n2;
        locals.var_isbd_swg_dn4 = assign96280_e148856_d_n4;
        locals.var_isbd_swg_dn5 = assign96280_e148856_d_n5;
        locals.var_isbd_swg_dn6 = assign96280_e148856_d_n6;
        locals.var_isbd_swg_dn7 = assign96280_e148856_d_n7;
        locals.var_isbd_swg_dn8 = assign96280_e148856_d_n8;
        locals.var_isbd_swg_dn9 = assign96280_e148856_d_n9;
        locals.var_isbd_swg_dn10 = assign96280_e148856_d_n10;
        locals.var_isbd_swg_dn11 = assign96280_e148856_d_n11;
        locals.var_isbd_swg_dn14 = assign96280_e148856_d_n14;
        locals.var_isbd_swg_rv = 0.0;

        let (assign96290_e148867, assign96290_e148867_d_n0, assign96290_e148867_d_n2, assign96290_e148867_d_n4, assign96290_e148867_d_n5, assign96290_e148867_d_n6, assign96290_e148867_d_n7, assign96290_e148867_d_n8, assign96290_e148867_d_n9, assign96290_e148867_d_n10, assign96290_e148867_d_n11, assign96290_e148867_d_n14,) = {
    if (((locals.var_guard2237 != 0.0) && (locals.var_guard2238 != 0.0)) && (locals.var_guard2239 == 0.0)) {
        let assign96290_e148865: f64 = (p.p15 * locals.var_jsswg2);
        (assign96290_e148865, (p.p15 * locals.var_jsswg2_dn0), (p.p15 * locals.var_jsswg2_dn2), (p.p15 * locals.var_jsswg2_dn4), (p.p15 * locals.var_jsswg2_dn5), (p.p15 * locals.var_jsswg2_dn6), (p.p15 * locals.var_jsswg2_dn7), (p.p15 * locals.var_jsswg2_dn8), (p.p15 * locals.var_jsswg2_dn9), (p.p15 * locals.var_jsswg2_dn10), (p.p15 * locals.var_jsswg2_dn11), (p.p15 * locals.var_jsswg2_dn14),)
    } else {
        (locals.var_isbd2_swg, locals.var_isbd2_swg_dn0, locals.var_isbd2_swg_dn2, locals.var_isbd2_swg_dn4, locals.var_isbd2_swg_dn5, locals.var_isbd2_swg_dn6, locals.var_isbd2_swg_dn7, locals.var_isbd2_swg_dn8, locals.var_isbd2_swg_dn9, locals.var_isbd2_swg_dn10, locals.var_isbd2_swg_dn11, locals.var_isbd2_swg_dn14,)
    }
};
        locals.var_isbd2_swg = assign96290_e148867;
        locals.var_isbd2_swg_dn0 = assign96290_e148867_d_n0;
        locals.var_isbd2_swg_dn2 = assign96290_e148867_d_n2;
        locals.var_isbd2_swg_dn4 = assign96290_e148867_d_n4;
        locals.var_isbd2_swg_dn5 = assign96290_e148867_d_n5;
        locals.var_isbd2_swg_dn6 = assign96290_e148867_d_n6;
        locals.var_isbd2_swg_dn7 = assign96290_e148867_d_n7;
        locals.var_isbd2_swg_dn8 = assign96290_e148867_d_n8;
        locals.var_isbd2_swg_dn9 = assign96290_e148867_d_n9;
        locals.var_isbd2_swg_dn10 = assign96290_e148867_d_n10;
        locals.var_isbd2_swg_dn11 = assign96290_e148867_d_n11;
        locals.var_isbd2_swg_dn14 = assign96290_e148867_d_n14;
        locals.var_isbd2_swg_rv = 0.0;

        let (assign96300_e148876, assign96300_e148876_d_n0, assign96300_e148876_d_n2, assign96300_e148876_d_n4, assign96300_e148876_d_n5, assign96300_e148876_d_n6, assign96300_e148876_d_n7, assign96300_e148876_d_n8, assign96300_e148876_d_n9, assign96300_e148876_d_n10, assign96300_e148876_d_n11, assign96300_e148876_d_n14,) = {
    if ((locals.var_guard2237 != 0.0) && (locals.var_guard2238 == 0.0)) {
        let assign96300_e148874: f64 = (p.p13 * locals.var_js);
        (assign96300_e148874, (p.p13 * locals.var_js_dn0), (p.p13 * locals.var_js_dn2), (p.p13 * locals.var_js_dn4), (p.p13 * locals.var_js_dn5), (p.p13 * locals.var_js_dn6), (p.p13 * locals.var_js_dn7), (p.p13 * locals.var_js_dn8), (p.p13 * locals.var_js_dn9), (p.p13 * locals.var_js_dn10), (p.p13 * locals.var_js_dn11), (p.p13 * locals.var_js_dn14),)
    } else {
        (locals.var_isbd_btm, locals.var_isbd_btm_dn0, locals.var_isbd_btm_dn2, locals.var_isbd_btm_dn4, locals.var_isbd_btm_dn5, locals.var_isbd_btm_dn6, locals.var_isbd_btm_dn7, locals.var_isbd_btm_dn8, locals.var_isbd_btm_dn9, locals.var_isbd_btm_dn10, locals.var_isbd_btm_dn11, locals.var_isbd_btm_dn14,)
    }
};
        locals.var_isbd_btm = assign96300_e148876;
        locals.var_isbd_btm_dn0 = assign96300_e148876_d_n0;
        locals.var_isbd_btm_dn2 = assign96300_e148876_d_n2;
        locals.var_isbd_btm_dn4 = assign96300_e148876_d_n4;
        locals.var_isbd_btm_dn5 = assign96300_e148876_d_n5;
        locals.var_isbd_btm_dn6 = assign96300_e148876_d_n6;
        locals.var_isbd_btm_dn7 = assign96300_e148876_d_n7;
        locals.var_isbd_btm_dn8 = assign96300_e148876_d_n8;
        locals.var_isbd_btm_dn9 = assign96300_e148876_d_n9;
        locals.var_isbd_btm_dn10 = assign96300_e148876_d_n10;
        locals.var_isbd_btm_dn11 = assign96300_e148876_d_n11;
        locals.var_isbd_btm_dn14 = assign96300_e148876_d_n14;
        locals.var_isbd_btm_rv = 0.0;

        let (assign96310_e148885, assign96310_e148885_d_n0, assign96310_e148885_d_n2, assign96310_e148885_d_n4, assign96310_e148885_d_n5, assign96310_e148885_d_n6, assign96310_e148885_d_n7, assign96310_e148885_d_n8, assign96310_e148885_d_n9, assign96310_e148885_d_n10, assign96310_e148885_d_n11, assign96310_e148885_d_n14,) = {
    if ((locals.var_guard2237 != 0.0) && (locals.var_guard2238 == 0.0)) {
        let assign96310_e148883: f64 = (p.p13 * locals.var_js2);
        (assign96310_e148883, (p.p13 * locals.var_js2_dn0), (p.p13 * locals.var_js2_dn2), (p.p13 * locals.var_js2_dn4), (p.p13 * locals.var_js2_dn5), (p.p13 * locals.var_js2_dn6), (p.p13 * locals.var_js2_dn7), (p.p13 * locals.var_js2_dn8), (p.p13 * locals.var_js2_dn9), (p.p13 * locals.var_js2_dn10), (p.p13 * locals.var_js2_dn11), (p.p13 * locals.var_js2_dn14),)
    } else {
        (locals.var_isbd2_btm, locals.var_isbd2_btm_dn0, locals.var_isbd2_btm_dn2, locals.var_isbd2_btm_dn4, locals.var_isbd2_btm_dn5, locals.var_isbd2_btm_dn6, locals.var_isbd2_btm_dn7, locals.var_isbd2_btm_dn8, locals.var_isbd2_btm_dn9, locals.var_isbd2_btm_dn10, locals.var_isbd2_btm_dn11, locals.var_isbd2_btm_dn14,)
    }
};
        locals.var_isbd2_btm = assign96310_e148885;
        locals.var_isbd2_btm_dn0 = assign96310_e148885_d_n0;
        locals.var_isbd2_btm_dn2 = assign96310_e148885_d_n2;
        locals.var_isbd2_btm_dn4 = assign96310_e148885_d_n4;
        locals.var_isbd2_btm_dn5 = assign96310_e148885_d_n5;
        locals.var_isbd2_btm_dn6 = assign96310_e148885_d_n6;
        locals.var_isbd2_btm_dn7 = assign96310_e148885_d_n7;
        locals.var_isbd2_btm_dn8 = assign96310_e148885_d_n8;
        locals.var_isbd2_btm_dn9 = assign96310_e148885_d_n9;
        locals.var_isbd2_btm_dn10 = assign96310_e148885_d_n10;
        locals.var_isbd2_btm_dn11 = assign96310_e148885_d_n11;
        locals.var_isbd2_btm_dn14 = assign96310_e148885_d_n14;
        locals.var_isbd2_btm_rv = 0.0;

        let (assign96320_e148894, assign96320_e148894_d_n0, assign96320_e148894_d_n2, assign96320_e148894_d_n4, assign96320_e148894_d_n5, assign96320_e148894_d_n6, assign96320_e148894_d_n7, assign96320_e148894_d_n8, assign96320_e148894_d_n9, assign96320_e148894_d_n10, assign96320_e148894_d_n11, assign96320_e148894_d_n14,) = {
    if ((locals.var_guard2237 != 0.0) && (locals.var_guard2238 == 0.0)) {
        let assign96320_e148892: f64 = (p.p15 * locals.var_jssw);
        (assign96320_e148892, (p.p15 * locals.var_jssw_dn0), (p.p15 * locals.var_jssw_dn2), (p.p15 * locals.var_jssw_dn4), (p.p15 * locals.var_jssw_dn5), (p.p15 * locals.var_jssw_dn6), (p.p15 * locals.var_jssw_dn7), (p.p15 * locals.var_jssw_dn8), (p.p15 * locals.var_jssw_dn9), (p.p15 * locals.var_jssw_dn10), (p.p15 * locals.var_jssw_dn11), (p.p15 * locals.var_jssw_dn14),)
    } else {
        (locals.var_isbd_sws, locals.var_isbd_sws_dn0, locals.var_isbd_sws_dn2, locals.var_isbd_sws_dn4, locals.var_isbd_sws_dn5, locals.var_isbd_sws_dn6, locals.var_isbd_sws_dn7, locals.var_isbd_sws_dn8, locals.var_isbd_sws_dn9, locals.var_isbd_sws_dn10, locals.var_isbd_sws_dn11, locals.var_isbd_sws_dn14,)
    }
};
        locals.var_isbd_sws = assign96320_e148894;
        locals.var_isbd_sws_dn0 = assign96320_e148894_d_n0;
        locals.var_isbd_sws_dn2 = assign96320_e148894_d_n2;
        locals.var_isbd_sws_dn4 = assign96320_e148894_d_n4;
        locals.var_isbd_sws_dn5 = assign96320_e148894_d_n5;
        locals.var_isbd_sws_dn6 = assign96320_e148894_d_n6;
        locals.var_isbd_sws_dn7 = assign96320_e148894_d_n7;
        locals.var_isbd_sws_dn8 = assign96320_e148894_d_n8;
        locals.var_isbd_sws_dn9 = assign96320_e148894_d_n9;
        locals.var_isbd_sws_dn10 = assign96320_e148894_d_n10;
        locals.var_isbd_sws_dn11 = assign96320_e148894_d_n11;
        locals.var_isbd_sws_dn14 = assign96320_e148894_d_n14;
        locals.var_isbd_sws_rv = 0.0;

        let (assign96330_e148903, assign96330_e148903_d_n0, assign96330_e148903_d_n2, assign96330_e148903_d_n4, assign96330_e148903_d_n5, assign96330_e148903_d_n6, assign96330_e148903_d_n7, assign96330_e148903_d_n8, assign96330_e148903_d_n9, assign96330_e148903_d_n10, assign96330_e148903_d_n11, assign96330_e148903_d_n14,) = {
    if ((locals.var_guard2237 != 0.0) && (locals.var_guard2238 == 0.0)) {
        let assign96330_e148901: f64 = (p.p15 * locals.var_jssw2);
        (assign96330_e148901, (p.p15 * locals.var_jssw2_dn0), (p.p15 * locals.var_jssw2_dn2), (p.p15 * locals.var_jssw2_dn4), (p.p15 * locals.var_jssw2_dn5), (p.p15 * locals.var_jssw2_dn6), (p.p15 * locals.var_jssw2_dn7), (p.p15 * locals.var_jssw2_dn8), (p.p15 * locals.var_jssw2_dn9), (p.p15 * locals.var_jssw2_dn10), (p.p15 * locals.var_jssw2_dn11), (p.p15 * locals.var_jssw2_dn14),)
    } else {
        (locals.var_isbd2_sws, locals.var_isbd2_sws_dn0, locals.var_isbd2_sws_dn2, locals.var_isbd2_sws_dn4, locals.var_isbd2_sws_dn5, locals.var_isbd2_sws_dn6, locals.var_isbd2_sws_dn7, locals.var_isbd2_sws_dn8, locals.var_isbd2_sws_dn9, locals.var_isbd2_sws_dn10, locals.var_isbd2_sws_dn11, locals.var_isbd2_sws_dn14,)
    }
};
        locals.var_isbd2_sws = assign96330_e148903;
        locals.var_isbd2_sws_dn0 = assign96330_e148903_d_n0;
        locals.var_isbd2_sws_dn2 = assign96330_e148903_d_n2;
        locals.var_isbd2_sws_dn4 = assign96330_e148903_d_n4;
        locals.var_isbd2_sws_dn5 = assign96330_e148903_d_n5;
        locals.var_isbd2_sws_dn6 = assign96330_e148903_d_n6;
        locals.var_isbd2_sws_dn7 = assign96330_e148903_d_n7;
        locals.var_isbd2_sws_dn8 = assign96330_e148903_d_n8;
        locals.var_isbd2_sws_dn9 = assign96330_e148903_d_n9;
        locals.var_isbd2_sws_dn10 = assign96330_e148903_d_n10;
        locals.var_isbd2_sws_dn11 = assign96330_e148903_d_n11;
        locals.var_isbd2_sws_dn14 = assign96330_e148903_d_n14;
        locals.var_isbd2_sws_rv = 0.0;

        let (assign96340_e148910, assign96340_e148910_d_n0, assign96340_e148910_d_n2, assign96340_e148910_d_n4, assign96340_e148910_d_n5, assign96340_e148910_d_n6, assign96340_e148910_d_n7, assign96340_e148910_d_n8, assign96340_e148910_d_n9, assign96340_e148910_d_n10, assign96340_e148910_d_n11, assign96340_e148910_d_n14,) = {
    if ((locals.var_guard2237 != 0.0) && (locals.var_guard2238 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isbd_swg, locals.var_isbd_swg_dn0, locals.var_isbd_swg_dn2, locals.var_isbd_swg_dn4, locals.var_isbd_swg_dn5, locals.var_isbd_swg_dn6, locals.var_isbd_swg_dn7, locals.var_isbd_swg_dn8, locals.var_isbd_swg_dn9, locals.var_isbd_swg_dn10, locals.var_isbd_swg_dn11, locals.var_isbd_swg_dn14,)
    }
};
        locals.var_isbd_swg = assign96340_e148910;
        locals.var_isbd_swg_dn0 = assign96340_e148910_d_n0;
        locals.var_isbd_swg_dn2 = assign96340_e148910_d_n2;
        locals.var_isbd_swg_dn4 = assign96340_e148910_d_n4;
        locals.var_isbd_swg_dn5 = assign96340_e148910_d_n5;
        locals.var_isbd_swg_dn6 = assign96340_e148910_d_n6;
        locals.var_isbd_swg_dn7 = assign96340_e148910_d_n7;
        locals.var_isbd_swg_dn8 = assign96340_e148910_d_n8;
        locals.var_isbd_swg_dn9 = assign96340_e148910_d_n9;
        locals.var_isbd_swg_dn10 = assign96340_e148910_d_n10;
        locals.var_isbd_swg_dn11 = assign96340_e148910_d_n11;
        locals.var_isbd_swg_dn14 = assign96340_e148910_d_n14;
        locals.var_isbd_swg_rv = 0.0;

        let (assign96350_e148917, assign96350_e148917_d_n0, assign96350_e148917_d_n2, assign96350_e148917_d_n4, assign96350_e148917_d_n5, assign96350_e148917_d_n6, assign96350_e148917_d_n7, assign96350_e148917_d_n8, assign96350_e148917_d_n9, assign96350_e148917_d_n10, assign96350_e148917_d_n11, assign96350_e148917_d_n14,) = {
    if ((locals.var_guard2237 != 0.0) && (locals.var_guard2238 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isbd2_swg, locals.var_isbd2_swg_dn0, locals.var_isbd2_swg_dn2, locals.var_isbd2_swg_dn4, locals.var_isbd2_swg_dn5, locals.var_isbd2_swg_dn6, locals.var_isbd2_swg_dn7, locals.var_isbd2_swg_dn8, locals.var_isbd2_swg_dn9, locals.var_isbd2_swg_dn10, locals.var_isbd2_swg_dn11, locals.var_isbd2_swg_dn14,)
    }
};
        locals.var_isbd2_swg = assign96350_e148917;
        locals.var_isbd2_swg_dn0 = assign96350_e148917_d_n0;
        locals.var_isbd2_swg_dn2 = assign96350_e148917_d_n2;
        locals.var_isbd2_swg_dn4 = assign96350_e148917_d_n4;
        locals.var_isbd2_swg_dn5 = assign96350_e148917_d_n5;
        locals.var_isbd2_swg_dn6 = assign96350_e148917_d_n6;
        locals.var_isbd2_swg_dn7 = assign96350_e148917_d_n7;
        locals.var_isbd2_swg_dn8 = assign96350_e148917_d_n8;
        locals.var_isbd2_swg_dn9 = assign96350_e148917_d_n9;
        locals.var_isbd2_swg_dn10 = assign96350_e148917_d_n10;
        locals.var_isbd2_swg_dn11 = assign96350_e148917_d_n11;
        locals.var_isbd2_swg_dn14 = assign96350_e148917_d_n14;
        locals.var_isbd2_swg_rv = 0.0;

        let (assign96360_e148925, assign96360_e148925_d_n0, assign96360_e148925_d_n2, assign96360_e148925_d_n4, assign96360_e148925_d_n5, assign96360_e148925_d_n6, assign96360_e148925_d_n7, assign96360_e148925_d_n8, assign96360_e148925_d_n9, assign96360_e148925_d_n10, assign96360_e148925_d_n11, assign96360_e148925_d_n14,) = {
    if (locals.var_guard2237 != 0.0) {
        let assign96360_e148921: f64 = (locals.var_isbd_btm + locals.var_isbd_sws);
        let assign96360_e148923: f64 = (assign96360_e148921 + locals.var_isbd_swg);
        (assign96360_e148923, ((locals.var_isbd_btm_dn0 + locals.var_isbd_sws_dn0) + locals.var_isbd_swg_dn0), ((locals.var_isbd_btm_dn2 + locals.var_isbd_sws_dn2) + locals.var_isbd_swg_dn2), ((locals.var_isbd_btm_dn4 + locals.var_isbd_sws_dn4) + locals.var_isbd_swg_dn4), ((locals.var_isbd_btm_dn5 + locals.var_isbd_sws_dn5) + locals.var_isbd_swg_dn5), ((locals.var_isbd_btm_dn6 + locals.var_isbd_sws_dn6) + locals.var_isbd_swg_dn6), ((locals.var_isbd_btm_dn7 + locals.var_isbd_sws_dn7) + locals.var_isbd_swg_dn7), ((locals.var_isbd_btm_dn8 + locals.var_isbd_sws_dn8) + locals.var_isbd_swg_dn8), ((locals.var_isbd_btm_dn9 + locals.var_isbd_sws_dn9) + locals.var_isbd_swg_dn9), ((locals.var_isbd_btm_dn10 + locals.var_isbd_sws_dn10) + locals.var_isbd_swg_dn10), ((locals.var_isbd_btm_dn11 + locals.var_isbd_sws_dn11) + locals.var_isbd_swg_dn11), ((locals.var_isbd_btm_dn14 + locals.var_isbd_sws_dn14) + locals.var_isbd_swg_dn14),)
    } else {
        (locals.var_isbd, locals.var_isbd_dn0, locals.var_isbd_dn2, locals.var_isbd_dn4, locals.var_isbd_dn5, locals.var_isbd_dn6, locals.var_isbd_dn7, locals.var_isbd_dn8, locals.var_isbd_dn9, locals.var_isbd_dn10, locals.var_isbd_dn11, locals.var_isbd_dn14,)
    }
};
        locals.var_isbd = assign96360_e148925;
        locals.var_isbd_dn0 = assign96360_e148925_d_n0;
        locals.var_isbd_dn2 = assign96360_e148925_d_n2;
        locals.var_isbd_dn4 = assign96360_e148925_d_n4;
        locals.var_isbd_dn5 = assign96360_e148925_d_n5;
        locals.var_isbd_dn6 = assign96360_e148925_d_n6;
        locals.var_isbd_dn7 = assign96360_e148925_d_n7;
        locals.var_isbd_dn8 = assign96360_e148925_d_n8;
        locals.var_isbd_dn9 = assign96360_e148925_d_n9;
        locals.var_isbd_dn10 = assign96360_e148925_d_n10;
        locals.var_isbd_dn11 = assign96360_e148925_d_n11;
        locals.var_isbd_dn14 = assign96360_e148925_d_n14;
        locals.var_isbd_rv = 0.0;

        let assign96370_e148928: f64 = if locals.var_isbd > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2240 = assign96370_e148928;
        locals.var_guard2240_rv = 0.0;

        let (assign96380_e148936, assign96380_e148936_d_n0, assign96380_e148936_d_n2, assign96380_e148936_d_n4, assign96380_e148936_d_n5, assign96380_e148936_d_n6, assign96380_e148936_d_n7, assign96380_e148936_d_n8, assign96380_e148936_d_n9, assign96380_e148936_d_n10, assign96380_e148936_d_n11, assign96380_e148936_d_n14,) = {
    if ((locals.var_guard2237 != 0.0) && (locals.var_guard2240 != 0.0)) {
        let assign96380_e148934: f64 = (locals.var_isbd + 1e-25);
        (assign96380_e148934, locals.var_isbd_dn0, locals.var_isbd_dn2, locals.var_isbd_dn4, locals.var_isbd_dn5, locals.var_isbd_dn6, locals.var_isbd_dn7, locals.var_isbd_dn8, locals.var_isbd_dn9, locals.var_isbd_dn10, locals.var_isbd_dn11, locals.var_isbd_dn14,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign96380_e148936;
        locals.var_t2_dn0 = assign96380_e148936_d_n0;
        locals.var_t2_dn2 = assign96380_e148936_d_n2;
        locals.var_t2_dn4 = assign96380_e148936_d_n4;
        locals.var_t2_dn5 = assign96380_e148936_d_n5;
        locals.var_t2_dn6 = assign96380_e148936_d_n6;
        locals.var_t2_dn7 = assign96380_e148936_d_n7;
        locals.var_t2_dn8 = assign96380_e148936_d_n8;
        locals.var_t2_dn9 = assign96380_e148936_d_n9;
        locals.var_t2_dn10 = assign96380_e148936_d_n10;
        locals.var_t2_dn11 = assign96380_e148936_d_n11;
        locals.var_t2_dn14 = assign96380_e148936_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign96390_e148953, assign96390_e148953_d_n0, assign96390_e148953_d_n2, assign96390_e148953_d_n4, assign96390_e148953_d_n5, assign96390_e148953_d_n6, assign96390_e148953_d_n7, assign96390_e148953_d_n8, assign96390_e148953_d_n9, assign96390_e148953_d_n10, assign96390_e148953_d_n11, assign96390_e148953_d_n14,) = {
    if ((locals.var_guard2237 != 0.0) && (locals.var_guard2240 != 0.0)) {
        let assign96390_e148942: f64 = (locals.var_uc_njd / locals.var_beta);
        let assign96390_e148945: f64 = (locals.var_uc_vdiffjd * locals.var_t0);
        let assign96390_e148947: f64 = (assign96390_e148945 / locals.var_t2);
        let assign96390_e148949: f64 = (assign96390_e148947 + 1.0);
        let assign96390_e148950: f64 = (assign96390_e148949).ln();
        let assign96390_e148951: f64 = (assign96390_e148942 * assign96390_e148950);
        (assign96390_e148951, (((-((locals.var_uc_njd * locals.var_beta_dn0) / (locals.var_beta * locals.var_beta))) * assign96390_e148950) + (assign96390_e148942 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn0) * locals.var_t2) - (assign96390_e148945 * locals.var_t2_dn0)) / (locals.var_t2 * locals.var_t2)) / assign96390_e148949))), (((-((locals.var_uc_njd * locals.var_beta_dn2) / (locals.var_beta * locals.var_beta))) * assign96390_e148950) + (assign96390_e148942 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn2) * locals.var_t2) - (assign96390_e148945 * locals.var_t2_dn2)) / (locals.var_t2 * locals.var_t2)) / assign96390_e148949))), (((-((locals.var_uc_njd * locals.var_beta_dn4) / (locals.var_beta * locals.var_beta))) * assign96390_e148950) + (assign96390_e148942 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn4) * locals.var_t2) - (assign96390_e148945 * locals.var_t2_dn4)) / (locals.var_t2 * locals.var_t2)) / assign96390_e148949))), (((-((locals.var_uc_njd * locals.var_beta_dn5) / (locals.var_beta * locals.var_beta))) * assign96390_e148950) + (assign96390_e148942 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn5) * locals.var_t2) - (assign96390_e148945 * locals.var_t2_dn5)) / (locals.var_t2 * locals.var_t2)) / assign96390_e148949))), (((-((locals.var_uc_njd * locals.var_beta_dn6) / (locals.var_beta * locals.var_beta))) * assign96390_e148950) + (assign96390_e148942 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn6) * locals.var_t2) - (assign96390_e148945 * locals.var_t2_dn6)) / (locals.var_t2 * locals.var_t2)) / assign96390_e148949))), (((-((locals.var_uc_njd * locals.var_beta_dn7) / (locals.var_beta * locals.var_beta))) * assign96390_e148950) + (assign96390_e148942 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn7) * locals.var_t2) - (assign96390_e148945 * locals.var_t2_dn7)) / (locals.var_t2 * locals.var_t2)) / assign96390_e148949))), (((-((locals.var_uc_njd * locals.var_beta_dn8) / (locals.var_beta * locals.var_beta))) * assign96390_e148950) + (assign96390_e148942 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn8) * locals.var_t2) - (assign96390_e148945 * locals.var_t2_dn8)) / (locals.var_t2 * locals.var_t2)) / assign96390_e148949))), (((-((locals.var_uc_njd * locals.var_beta_dn9) / (locals.var_beta * locals.var_beta))) * assign96390_e148950) + (assign96390_e148942 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn9) * locals.var_t2) - (assign96390_e148945 * locals.var_t2_dn9)) / (locals.var_t2 * locals.var_t2)) / assign96390_e148949))), (((-((locals.var_uc_njd * locals.var_beta_dn10) / (locals.var_beta * locals.var_beta))) * assign96390_e148950) + (assign96390_e148942 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn10) * locals.var_t2) - (assign96390_e148945 * locals.var_t2_dn10)) / (locals.var_t2 * locals.var_t2)) / assign96390_e148949))), (((-((locals.var_uc_njd * locals.var_beta_dn11) / (locals.var_beta * locals.var_beta))) * assign96390_e148950) + (assign96390_e148942 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn11) * locals.var_t2) - (assign96390_e148945 * locals.var_t2_dn11)) / (locals.var_t2 * locals.var_t2)) / assign96390_e148949))), (((-((locals.var_uc_njd * locals.var_beta_dn14) / (locals.var_beta * locals.var_beta))) * assign96390_e148950) + (assign96390_e148942 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn14) * locals.var_t2) - (assign96390_e148945 * locals.var_t2_dn14)) / (locals.var_t2 * locals.var_t2)) / assign96390_e148949))),)
    } else {
        (locals.var_vbdt, locals.var_vbdt_dn0, locals.var_vbdt_dn2, locals.var_vbdt_dn4, locals.var_vbdt_dn5, locals.var_vbdt_dn6, locals.var_vbdt_dn7, locals.var_vbdt_dn8, locals.var_vbdt_dn9, locals.var_vbdt_dn10, locals.var_vbdt_dn11, locals.var_vbdt_dn14,)
    }
};
        locals.var_vbdt = assign96390_e148953;
        locals.var_vbdt_dn0 = assign96390_e148953_d_n0;
        locals.var_vbdt_dn2 = assign96390_e148953_d_n2;
        locals.var_vbdt_dn4 = assign96390_e148953_d_n4;
        locals.var_vbdt_dn5 = assign96390_e148953_d_n5;
        locals.var_vbdt_dn6 = assign96390_e148953_d_n6;
        locals.var_vbdt_dn7 = assign96390_e148953_d_n7;
        locals.var_vbdt_dn8 = assign96390_e148953_d_n8;
        locals.var_vbdt_dn9 = assign96390_e148953_d_n9;
        locals.var_vbdt_dn10 = assign96390_e148953_d_n10;
        locals.var_vbdt_dn11 = assign96390_e148953_d_n11;
        locals.var_vbdt_dn14 = assign96390_e148953_d_n14;
        locals.var_vbdt_rv = 0.0;

        let (assign96400_e148964, assign96400_e148964_d_n0, assign96400_e148964_d_n2, assign96400_e148964_d_n4, assign96400_e148964_d_n5, assign96400_e148964_d_n6, assign96400_e148964_d_n7, assign96400_e148964_d_n8, assign96400_e148964_d_n9, assign96400_e148964_d_n10, assign96400_e148964_d_n11, assign96400_e148964_d_n14,) = {
    if ((locals.var_guard2237 != 0.0) && (locals.var_guard2240 != 0.0)) {
        let assign96400_e148959: f64 = (locals.var_tratio - 1.0);
        let assign96400_e148961: f64 = (assign96400_e148959 * p.p512);
        let assign96400_e148962: f64 = (assign96400_e148961).exp();
        (assign96400_e148962, (assign96400_e148962 * (locals.var_tratio_dn0 * p.p512)), (assign96400_e148962 * (locals.var_tratio_dn2 * p.p512)), (assign96400_e148962 * (locals.var_tratio_dn4 * p.p512)), (assign96400_e148962 * (locals.var_tratio_dn5 * p.p512)), (assign96400_e148962 * (locals.var_tratio_dn6 * p.p512)), (assign96400_e148962 * (locals.var_tratio_dn7 * p.p512)), (assign96400_e148962 * (locals.var_tratio_dn8 * p.p512)), (assign96400_e148962 * (locals.var_tratio_dn9 * p.p512)), (assign96400_e148962 * (locals.var_tratio_dn10 * p.p512)), (assign96400_e148962 * (locals.var_tratio_dn11 * p.p512)), (assign96400_e148962 * (locals.var_tratio_dn14 * p.p512)),)
    } else {
        (locals.var_exptempd, locals.var_exptempd_dn0, locals.var_exptempd_dn2, locals.var_exptempd_dn4, locals.var_exptempd_dn5, locals.var_exptempd_dn6, locals.var_exptempd_dn7, locals.var_exptempd_dn8, locals.var_exptempd_dn9, locals.var_exptempd_dn10, locals.var_exptempd_dn11, locals.var_exptempd_dn14,)
    }
};
        locals.var_exptempd = assign96400_e148964;
        locals.var_exptempd_dn0 = assign96400_e148964_d_n0;
        locals.var_exptempd_dn2 = assign96400_e148964_d_n2;
        locals.var_exptempd_dn4 = assign96400_e148964_d_n4;
        locals.var_exptempd_dn5 = assign96400_e148964_d_n5;
        locals.var_exptempd_dn6 = assign96400_e148964_d_n6;
        locals.var_exptempd_dn7 = assign96400_e148964_d_n7;
        locals.var_exptempd_dn8 = assign96400_e148964_d_n8;
        locals.var_exptempd_dn9 = assign96400_e148964_d_n9;
        locals.var_exptempd_dn10 = assign96400_e148964_d_n10;
        locals.var_exptempd_dn11 = assign96400_e148964_d_n11;
        locals.var_exptempd_dn14 = assign96400_e148964_d_n14;
        locals.var_exptempd_rv = 0.0;

        let (assign96410_e148974, assign96410_e148974_d_n0, assign96410_e148974_d_n2, assign96410_e148974_d_n4, assign96410_e148974_d_n5, assign96410_e148974_d_n6, assign96410_e148974_d_n7, assign96410_e148974_d_n8, assign96410_e148974_d_n9, assign96410_e148974_d_n10, assign96410_e148974_d_n11, assign96410_e148974_d_n14,) = {
    if ((locals.var_guard2237 != 0.0) && (locals.var_guard2240 != 0.0)) {
        let assign96410_e148971: f64 = (locals.var_uc_njd / locals.var_beta);
        let assign96410_e148972: f64 = (1.0 / assign96410_e148971);
        (assign96410_e148972, (-((-((locals.var_uc_njd * locals.var_beta_dn0) / (locals.var_beta * locals.var_beta))) / (assign96410_e148971 * assign96410_e148971))), (-((-((locals.var_uc_njd * locals.var_beta_dn2) / (locals.var_beta * locals.var_beta))) / (assign96410_e148971 * assign96410_e148971))), (-((-((locals.var_uc_njd * locals.var_beta_dn4) / (locals.var_beta * locals.var_beta))) / (assign96410_e148971 * assign96410_e148971))), (-((-((locals.var_uc_njd * locals.var_beta_dn5) / (locals.var_beta * locals.var_beta))) / (assign96410_e148971 * assign96410_e148971))), (-((-((locals.var_uc_njd * locals.var_beta_dn6) / (locals.var_beta * locals.var_beta))) / (assign96410_e148971 * assign96410_e148971))), (-((-((locals.var_uc_njd * locals.var_beta_dn7) / (locals.var_beta * locals.var_beta))) / (assign96410_e148971 * assign96410_e148971))), (-((-((locals.var_uc_njd * locals.var_beta_dn8) / (locals.var_beta * locals.var_beta))) / (assign96410_e148971 * assign96410_e148971))), (-((-((locals.var_uc_njd * locals.var_beta_dn9) / (locals.var_beta * locals.var_beta))) / (assign96410_e148971 * assign96410_e148971))), (-((-((locals.var_uc_njd * locals.var_beta_dn10) / (locals.var_beta * locals.var_beta))) / (assign96410_e148971 * assign96410_e148971))), (-((-((locals.var_uc_njd * locals.var_beta_dn11) / (locals.var_beta * locals.var_beta))) / (assign96410_e148971 * assign96410_e148971))), (-((-((locals.var_uc_njd * locals.var_beta_dn14) / (locals.var_beta * locals.var_beta))) / (assign96410_e148971 * assign96410_e148971))),)
    } else {
        (locals.var_jd_nvtm_invd, locals.var_jd_nvtm_invd_dn0, locals.var_jd_nvtm_invd_dn2, locals.var_jd_nvtm_invd_dn4, locals.var_jd_nvtm_invd_dn5, locals.var_jd_nvtm_invd_dn6, locals.var_jd_nvtm_invd_dn7, locals.var_jd_nvtm_invd_dn8, locals.var_jd_nvtm_invd_dn9, locals.var_jd_nvtm_invd_dn10, locals.var_jd_nvtm_invd_dn11, locals.var_jd_nvtm_invd_dn14,)
    }
};
        locals.var_jd_nvtm_invd = assign96410_e148974;
        locals.var_jd_nvtm_invd_dn0 = assign96410_e148974_d_n0;
        locals.var_jd_nvtm_invd_dn2 = assign96410_e148974_d_n2;
        locals.var_jd_nvtm_invd_dn4 = assign96410_e148974_d_n4;
        locals.var_jd_nvtm_invd_dn5 = assign96410_e148974_d_n5;
        locals.var_jd_nvtm_invd_dn6 = assign96410_e148974_d_n6;
        locals.var_jd_nvtm_invd_dn7 = assign96410_e148974_d_n7;
        locals.var_jd_nvtm_invd_dn8 = assign96410_e148974_d_n8;
        locals.var_jd_nvtm_invd_dn9 = assign96410_e148974_d_n9;
        locals.var_jd_nvtm_invd_dn10 = assign96410_e148974_d_n10;
        locals.var_jd_nvtm_invd_dn11 = assign96410_e148974_d_n11;
        locals.var_jd_nvtm_invd_dn14 = assign96410_e148974_d_n14;
        locals.var_jd_nvtm_invd_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_374(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign96420_e148983, assign96420_e148983_d_n0, assign96420_e148983_d_n2, assign96420_e148983_d_n4, assign96420_e148983_d_n5, assign96420_e148983_d_n6, assign96420_e148983_d_n7, assign96420_e148983_d_n8, assign96420_e148983_d_n9, assign96420_e148983_d_n10, assign96420_e148983_d_n11, assign96420_e148983_d_n14,) = {
    if ((locals.var_guard2237 != 0.0) && (locals.var_guard2240 != 0.0)) {
        let assign96420_e148980: f64 = (locals.var_vbdt * locals.var_jd_nvtm_invd);
        let assign96420_e148981: f64 = (assign96420_e148980).exp();
        (assign96420_e148981, (assign96420_e148981 * ((locals.var_vbdt_dn0 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn0))), (assign96420_e148981 * ((locals.var_vbdt_dn2 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn2))), (assign96420_e148981 * ((locals.var_vbdt_dn4 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn4))), (assign96420_e148981 * ((locals.var_vbdt_dn5 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn5))), (assign96420_e148981 * ((locals.var_vbdt_dn6 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn6))), (assign96420_e148981 * ((locals.var_vbdt_dn7 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn7))), (assign96420_e148981 * ((locals.var_vbdt_dn8 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn8))), (assign96420_e148981 * ((locals.var_vbdt_dn9 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn9))), (assign96420_e148981 * ((locals.var_vbdt_dn10 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn10))), (assign96420_e148981 * ((locals.var_vbdt_dn11 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn11))), (assign96420_e148981 * ((locals.var_vbdt_dn14 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn14))),)
    } else {
        (locals.var_jd_expcd, locals.var_jd_expcd_dn0, locals.var_jd_expcd_dn2, locals.var_jd_expcd_dn4, locals.var_jd_expcd_dn5, locals.var_jd_expcd_dn6, locals.var_jd_expcd_dn7, locals.var_jd_expcd_dn8, locals.var_jd_expcd_dn9, locals.var_jd_expcd_dn10, locals.var_jd_expcd_dn11, locals.var_jd_expcd_dn14,)
    }
};
        locals.var_jd_expcd = assign96420_e148983;
        locals.var_jd_expcd_dn0 = assign96420_e148983_d_n0;
        locals.var_jd_expcd_dn2 = assign96420_e148983_d_n2;
        locals.var_jd_expcd_dn4 = assign96420_e148983_d_n4;
        locals.var_jd_expcd_dn5 = assign96420_e148983_d_n5;
        locals.var_jd_expcd_dn6 = assign96420_e148983_d_n6;
        locals.var_jd_expcd_dn7 = assign96420_e148983_d_n7;
        locals.var_jd_expcd_dn8 = assign96420_e148983_d_n8;
        locals.var_jd_expcd_dn9 = assign96420_e148983_d_n9;
        locals.var_jd_expcd_dn10 = assign96420_e148983_d_n10;
        locals.var_jd_expcd_dn11 = assign96420_e148983_d_n11;
        locals.var_jd_expcd_dn14 = assign96420_e148983_d_n14;
        locals.var_jd_expcd_rv = 0.0;

        let (assign96430_e149002, assign96430_e149002_d_n0, assign96430_e149002_d_n2, assign96430_e149002_d_n4, assign96430_e149002_d_n5, assign96430_e149002_d_n6, assign96430_e149002_d_n7, assign96430_e149002_d_n8, assign96430_e149002_d_n9, assign96430_e149002_d_n10, assign96430_e149002_d_n11, assign96430_e149002_d_n14,) = {
    if (locals.var_guard2237 != 0.0) {
        let assign96430_e148988: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign96430_e148991: f64 = (locals.var_eg * locals.var_beta);
        let assign96430_e148992: f64 = (assign96430_e148988 - assign96430_e148991);
        let assign96430_e148995: f64 = (p.p522 * locals.var_log_tratio);
        let assign96430_e148996: f64 = (assign96430_e148992 + assign96430_e148995);
        let assign96430_e148998: f64 = (assign96430_e148996 / locals.var_uc_njs);
        let assign96430_e148999: f64 = (assign96430_e148998).exp();
        let assign96430_e149000: f64 = (locals.var_uc_js0s * assign96430_e148999);
        (assign96430_e149000, (locals.var_uc_js0s * (assign96430_e148999 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p522 * locals.var_log_tratio_dn0)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96430_e148999 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p522 * locals.var_log_tratio_dn2)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96430_e148999 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p522 * locals.var_log_tratio_dn4)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96430_e148999 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p522 * locals.var_log_tratio_dn5)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96430_e148999 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p522 * locals.var_log_tratio_dn6)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96430_e148999 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p522 * locals.var_log_tratio_dn7)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96430_e148999 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p522 * locals.var_log_tratio_dn8)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96430_e148999 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p522 * locals.var_log_tratio_dn9)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96430_e148999 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p522 * locals.var_log_tratio_dn10)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96430_e148999 * (((-((locals.var_eg_dn11 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn11))) + (p.p522 * locals.var_log_tratio_dn11)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96430_e148999 * (((-((locals.var_eg_dn14 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn14))) + (p.p522 * locals.var_log_tratio_dn14)) / locals.var_uc_njs))),)
    } else {
        (locals.var_js, locals.var_js_dn0, locals.var_js_dn2, locals.var_js_dn4, locals.var_js_dn5, locals.var_js_dn6, locals.var_js_dn7, locals.var_js_dn8, locals.var_js_dn9, locals.var_js_dn10, locals.var_js_dn11, locals.var_js_dn14,)
    }
};
        locals.var_js = assign96430_e149002;
        locals.var_js_dn0 = assign96430_e149002_d_n0;
        locals.var_js_dn2 = assign96430_e149002_d_n2;
        locals.var_js_dn4 = assign96430_e149002_d_n4;
        locals.var_js_dn5 = assign96430_e149002_d_n5;
        locals.var_js_dn6 = assign96430_e149002_d_n6;
        locals.var_js_dn7 = assign96430_e149002_d_n7;
        locals.var_js_dn8 = assign96430_e149002_d_n8;
        locals.var_js_dn9 = assign96430_e149002_d_n9;
        locals.var_js_dn10 = assign96430_e149002_d_n10;
        locals.var_js_dn11 = assign96430_e149002_d_n11;
        locals.var_js_dn14 = assign96430_e149002_d_n14;
        locals.var_js_rv = 0.0;

        let (assign96440_e149021, assign96440_e149021_d_n0, assign96440_e149021_d_n2, assign96440_e149021_d_n4, assign96440_e149021_d_n5, assign96440_e149021_d_n6, assign96440_e149021_d_n7, assign96440_e149021_d_n8, assign96440_e149021_d_n9, assign96440_e149021_d_n10, assign96440_e149021_d_n11, assign96440_e149021_d_n14,) = {
    if (locals.var_guard2237 != 0.0) {
        let assign96440_e149007: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign96440_e149010: f64 = (locals.var_eg * locals.var_beta);
        let assign96440_e149011: f64 = (assign96440_e149007 - assign96440_e149010);
        let assign96440_e149014: f64 = (p.p522 * locals.var_log_tratio);
        let assign96440_e149015: f64 = (assign96440_e149011 + assign96440_e149014);
        let assign96440_e149017: f64 = (assign96440_e149015 / p.p520);
        let assign96440_e149018: f64 = (assign96440_e149017).exp();
        let assign96440_e149019: f64 = (locals.var_uc_js0sws * assign96440_e149018);
        (assign96440_e149019, (locals.var_uc_js0sws * (assign96440_e149018 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p522 * locals.var_log_tratio_dn0)) / p.p520))), (locals.var_uc_js0sws * (assign96440_e149018 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p522 * locals.var_log_tratio_dn2)) / p.p520))), (locals.var_uc_js0sws * (assign96440_e149018 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p522 * locals.var_log_tratio_dn4)) / p.p520))), (locals.var_uc_js0sws * (assign96440_e149018 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p522 * locals.var_log_tratio_dn5)) / p.p520))), (locals.var_uc_js0sws * (assign96440_e149018 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p522 * locals.var_log_tratio_dn6)) / p.p520))), (locals.var_uc_js0sws * (assign96440_e149018 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p522 * locals.var_log_tratio_dn7)) / p.p520))), (locals.var_uc_js0sws * (assign96440_e149018 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p522 * locals.var_log_tratio_dn8)) / p.p520))), (locals.var_uc_js0sws * (assign96440_e149018 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p522 * locals.var_log_tratio_dn9)) / p.p520))), (locals.var_uc_js0sws * (assign96440_e149018 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p522 * locals.var_log_tratio_dn10)) / p.p520))), (locals.var_uc_js0sws * (assign96440_e149018 * (((-((locals.var_eg_dn11 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn11))) + (p.p522 * locals.var_log_tratio_dn11)) / p.p520))), (locals.var_uc_js0sws * (assign96440_e149018 * (((-((locals.var_eg_dn14 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn14))) + (p.p522 * locals.var_log_tratio_dn14)) / p.p520))),)
    } else {
        (locals.var_jssw, locals.var_jssw_dn0, locals.var_jssw_dn2, locals.var_jssw_dn4, locals.var_jssw_dn5, locals.var_jssw_dn6, locals.var_jssw_dn7, locals.var_jssw_dn8, locals.var_jssw_dn9, locals.var_jssw_dn10, locals.var_jssw_dn11, locals.var_jssw_dn14,)
    }
};
        locals.var_jssw = assign96440_e149021;
        locals.var_jssw_dn0 = assign96440_e149021_d_n0;
        locals.var_jssw_dn2 = assign96440_e149021_d_n2;
        locals.var_jssw_dn4 = assign96440_e149021_d_n4;
        locals.var_jssw_dn5 = assign96440_e149021_d_n5;
        locals.var_jssw_dn6 = assign96440_e149021_d_n6;
        locals.var_jssw_dn7 = assign96440_e149021_d_n7;
        locals.var_jssw_dn8 = assign96440_e149021_d_n8;
        locals.var_jssw_dn9 = assign96440_e149021_d_n9;
        locals.var_jssw_dn10 = assign96440_e149021_d_n10;
        locals.var_jssw_dn11 = assign96440_e149021_d_n11;
        locals.var_jssw_dn14 = assign96440_e149021_d_n14;
        locals.var_jssw_rv = 0.0;

        let (assign96450_e149040, assign96450_e149040_d_n0, assign96450_e149040_d_n2, assign96450_e149040_d_n4, assign96450_e149040_d_n5, assign96450_e149040_d_n6, assign96450_e149040_d_n7, assign96450_e149040_d_n8, assign96450_e149040_d_n9, assign96450_e149040_d_n10, assign96450_e149040_d_n11, assign96450_e149040_d_n14,) = {
    if (locals.var_guard2237 != 0.0) {
        let assign96450_e149026: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign96450_e149029: f64 = (locals.var_eg * locals.var_beta);
        let assign96450_e149030: f64 = (assign96450_e149026 - assign96450_e149029);
        let assign96450_e149033: f64 = (p.p522 * locals.var_log_tratio);
        let assign96450_e149034: f64 = (assign96450_e149030 + assign96450_e149033);
        let assign96450_e149036: f64 = (assign96450_e149034 / p.p521);
        let assign96450_e149037: f64 = (assign96450_e149036).exp();
        let assign96450_e149038: f64 = (p.p518 * assign96450_e149037);
        (assign96450_e149038, (p.p518 * (assign96450_e149037 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p522 * locals.var_log_tratio_dn0)) / p.p521))), (p.p518 * (assign96450_e149037 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p522 * locals.var_log_tratio_dn2)) / p.p521))), (p.p518 * (assign96450_e149037 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p522 * locals.var_log_tratio_dn4)) / p.p521))), (p.p518 * (assign96450_e149037 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p522 * locals.var_log_tratio_dn5)) / p.p521))), (p.p518 * (assign96450_e149037 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p522 * locals.var_log_tratio_dn6)) / p.p521))), (p.p518 * (assign96450_e149037 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p522 * locals.var_log_tratio_dn7)) / p.p521))), (p.p518 * (assign96450_e149037 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p522 * locals.var_log_tratio_dn8)) / p.p521))), (p.p518 * (assign96450_e149037 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p522 * locals.var_log_tratio_dn9)) / p.p521))), (p.p518 * (assign96450_e149037 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p522 * locals.var_log_tratio_dn10)) / p.p521))), (p.p518 * (assign96450_e149037 * (((-((locals.var_eg_dn11 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn11))) + (p.p522 * locals.var_log_tratio_dn11)) / p.p521))), (p.p518 * (assign96450_e149037 * (((-((locals.var_eg_dn14 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn14))) + (p.p522 * locals.var_log_tratio_dn14)) / p.p521))),)
    } else {
        (locals.var_jsswg, locals.var_jsswg_dn0, locals.var_jsswg_dn2, locals.var_jsswg_dn4, locals.var_jsswg_dn5, locals.var_jsswg_dn6, locals.var_jsswg_dn7, locals.var_jsswg_dn8, locals.var_jsswg_dn9, locals.var_jsswg_dn10, locals.var_jsswg_dn11, locals.var_jsswg_dn14,)
    }
};
        locals.var_jsswg = assign96450_e149040;
        locals.var_jsswg_dn0 = assign96450_e149040_d_n0;
        locals.var_jsswg_dn2 = assign96450_e149040_d_n2;
        locals.var_jsswg_dn4 = assign96450_e149040_d_n4;
        locals.var_jsswg_dn5 = assign96450_e149040_d_n5;
        locals.var_jsswg_dn6 = assign96450_e149040_d_n6;
        locals.var_jsswg_dn7 = assign96450_e149040_d_n7;
        locals.var_jsswg_dn8 = assign96450_e149040_d_n8;
        locals.var_jsswg_dn9 = assign96450_e149040_d_n9;
        locals.var_jsswg_dn10 = assign96450_e149040_d_n10;
        locals.var_jsswg_dn11 = assign96450_e149040_d_n11;
        locals.var_jsswg_dn14 = assign96450_e149040_d_n14;
        locals.var_jsswg_rv = 0.0;

        let (assign96460_e149059, assign96460_e149059_d_n0, assign96460_e149059_d_n2, assign96460_e149059_d_n4, assign96460_e149059_d_n5, assign96460_e149059_d_n6, assign96460_e149059_d_n7, assign96460_e149059_d_n8, assign96460_e149059_d_n9, assign96460_e149059_d_n10, assign96460_e149059_d_n11, assign96460_e149059_d_n14,) = {
    if (locals.var_guard2237 != 0.0) {
        let assign96460_e149045: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign96460_e149048: f64 = (locals.var_eg * locals.var_beta);
        let assign96460_e149049: f64 = (assign96460_e149045 - assign96460_e149048);
        let assign96460_e149052: f64 = (p.p532 * locals.var_log_tratio);
        let assign96460_e149053: f64 = (assign96460_e149049 + assign96460_e149052);
        let assign96460_e149055: f64 = (assign96460_e149053 / locals.var_uc_njs);
        let assign96460_e149056: f64 = (assign96460_e149055).exp();
        let assign96460_e149057: f64 = (locals.var_uc_js0s * assign96460_e149056);
        (assign96460_e149057, (locals.var_uc_js0s * (assign96460_e149056 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p532 * locals.var_log_tratio_dn0)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96460_e149056 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p532 * locals.var_log_tratio_dn2)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96460_e149056 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p532 * locals.var_log_tratio_dn4)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96460_e149056 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p532 * locals.var_log_tratio_dn5)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96460_e149056 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p532 * locals.var_log_tratio_dn6)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96460_e149056 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p532 * locals.var_log_tratio_dn7)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96460_e149056 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p532 * locals.var_log_tratio_dn8)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96460_e149056 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p532 * locals.var_log_tratio_dn9)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96460_e149056 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p532 * locals.var_log_tratio_dn10)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96460_e149056 * (((-((locals.var_eg_dn11 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn11))) + (p.p532 * locals.var_log_tratio_dn11)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96460_e149056 * (((-((locals.var_eg_dn14 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn14))) + (p.p532 * locals.var_log_tratio_dn14)) / locals.var_uc_njs))),)
    } else {
        (locals.var_js2, locals.var_js2_dn0, locals.var_js2_dn2, locals.var_js2_dn4, locals.var_js2_dn5, locals.var_js2_dn6, locals.var_js2_dn7, locals.var_js2_dn8, locals.var_js2_dn9, locals.var_js2_dn10, locals.var_js2_dn11, locals.var_js2_dn14,)
    }
};
        locals.var_js2 = assign96460_e149059;
        locals.var_js2_dn0 = assign96460_e149059_d_n0;
        locals.var_js2_dn2 = assign96460_e149059_d_n2;
        locals.var_js2_dn4 = assign96460_e149059_d_n4;
        locals.var_js2_dn5 = assign96460_e149059_d_n5;
        locals.var_js2_dn6 = assign96460_e149059_d_n6;
        locals.var_js2_dn7 = assign96460_e149059_d_n7;
        locals.var_js2_dn8 = assign96460_e149059_d_n8;
        locals.var_js2_dn9 = assign96460_e149059_d_n9;
        locals.var_js2_dn10 = assign96460_e149059_d_n10;
        locals.var_js2_dn11 = assign96460_e149059_d_n11;
        locals.var_js2_dn14 = assign96460_e149059_d_n14;
        locals.var_js2_rv = 0.0;

        let (assign96470_e149078, assign96470_e149078_d_n0, assign96470_e149078_d_n2, assign96470_e149078_d_n4, assign96470_e149078_d_n5, assign96470_e149078_d_n6, assign96470_e149078_d_n7, assign96470_e149078_d_n8, assign96470_e149078_d_n9, assign96470_e149078_d_n10, assign96470_e149078_d_n11, assign96470_e149078_d_n14,) = {
    if (locals.var_guard2237 != 0.0) {
        let assign96470_e149064: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign96470_e149067: f64 = (locals.var_eg * locals.var_beta);
        let assign96470_e149068: f64 = (assign96470_e149064 - assign96470_e149067);
        let assign96470_e149071: f64 = (p.p532 * locals.var_log_tratio);
        let assign96470_e149072: f64 = (assign96470_e149068 + assign96470_e149071);
        let assign96470_e149074: f64 = (assign96470_e149072 / p.p520);
        let assign96470_e149075: f64 = (assign96470_e149074).exp();
        let assign96470_e149076: f64 = (locals.var_uc_js0sws * assign96470_e149075);
        (assign96470_e149076, (locals.var_uc_js0sws * (assign96470_e149075 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p532 * locals.var_log_tratio_dn0)) / p.p520))), (locals.var_uc_js0sws * (assign96470_e149075 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p532 * locals.var_log_tratio_dn2)) / p.p520))), (locals.var_uc_js0sws * (assign96470_e149075 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p532 * locals.var_log_tratio_dn4)) / p.p520))), (locals.var_uc_js0sws * (assign96470_e149075 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p532 * locals.var_log_tratio_dn5)) / p.p520))), (locals.var_uc_js0sws * (assign96470_e149075 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p532 * locals.var_log_tratio_dn6)) / p.p520))), (locals.var_uc_js0sws * (assign96470_e149075 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p532 * locals.var_log_tratio_dn7)) / p.p520))), (locals.var_uc_js0sws * (assign96470_e149075 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p532 * locals.var_log_tratio_dn8)) / p.p520))), (locals.var_uc_js0sws * (assign96470_e149075 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p532 * locals.var_log_tratio_dn9)) / p.p520))), (locals.var_uc_js0sws * (assign96470_e149075 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p532 * locals.var_log_tratio_dn10)) / p.p520))), (locals.var_uc_js0sws * (assign96470_e149075 * (((-((locals.var_eg_dn11 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn11))) + (p.p532 * locals.var_log_tratio_dn11)) / p.p520))), (locals.var_uc_js0sws * (assign96470_e149075 * (((-((locals.var_eg_dn14 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn14))) + (p.p532 * locals.var_log_tratio_dn14)) / p.p520))),)
    } else {
        (locals.var_jssw2, locals.var_jssw2_dn0, locals.var_jssw2_dn2, locals.var_jssw2_dn4, locals.var_jssw2_dn5, locals.var_jssw2_dn6, locals.var_jssw2_dn7, locals.var_jssw2_dn8, locals.var_jssw2_dn9, locals.var_jssw2_dn10, locals.var_jssw2_dn11, locals.var_jssw2_dn14,)
    }
};
        locals.var_jssw2 = assign96470_e149078;
        locals.var_jssw2_dn0 = assign96470_e149078_d_n0;
        locals.var_jssw2_dn2 = assign96470_e149078_d_n2;
        locals.var_jssw2_dn4 = assign96470_e149078_d_n4;
        locals.var_jssw2_dn5 = assign96470_e149078_d_n5;
        locals.var_jssw2_dn6 = assign96470_e149078_d_n6;
        locals.var_jssw2_dn7 = assign96470_e149078_d_n7;
        locals.var_jssw2_dn8 = assign96470_e149078_d_n8;
        locals.var_jssw2_dn9 = assign96470_e149078_d_n9;
        locals.var_jssw2_dn10 = assign96470_e149078_d_n10;
        locals.var_jssw2_dn11 = assign96470_e149078_d_n11;
        locals.var_jssw2_dn14 = assign96470_e149078_d_n14;
        locals.var_jssw2_rv = 0.0;

        let (assign96480_e149097, assign96480_e149097_d_n0, assign96480_e149097_d_n2, assign96480_e149097_d_n4, assign96480_e149097_d_n5, assign96480_e149097_d_n6, assign96480_e149097_d_n7, assign96480_e149097_d_n8, assign96480_e149097_d_n9, assign96480_e149097_d_n10, assign96480_e149097_d_n11, assign96480_e149097_d_n14,) = {
    if (locals.var_guard2237 != 0.0) {
        let assign96480_e149083: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign96480_e149086: f64 = (locals.var_eg * locals.var_beta);
        let assign96480_e149087: f64 = (assign96480_e149083 - assign96480_e149086);
        let assign96480_e149090: f64 = (p.p532 * locals.var_log_tratio);
        let assign96480_e149091: f64 = (assign96480_e149087 + assign96480_e149090);
        let assign96480_e149093: f64 = (assign96480_e149091 / p.p521);
        let assign96480_e149094: f64 = (assign96480_e149093).exp();
        let assign96480_e149095: f64 = (p.p518 * assign96480_e149094);
        (assign96480_e149095, (p.p518 * (assign96480_e149094 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p532 * locals.var_log_tratio_dn0)) / p.p521))), (p.p518 * (assign96480_e149094 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p532 * locals.var_log_tratio_dn2)) / p.p521))), (p.p518 * (assign96480_e149094 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p532 * locals.var_log_tratio_dn4)) / p.p521))), (p.p518 * (assign96480_e149094 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p532 * locals.var_log_tratio_dn5)) / p.p521))), (p.p518 * (assign96480_e149094 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p532 * locals.var_log_tratio_dn6)) / p.p521))), (p.p518 * (assign96480_e149094 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p532 * locals.var_log_tratio_dn7)) / p.p521))), (p.p518 * (assign96480_e149094 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p532 * locals.var_log_tratio_dn8)) / p.p521))), (p.p518 * (assign96480_e149094 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p532 * locals.var_log_tratio_dn9)) / p.p521))), (p.p518 * (assign96480_e149094 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p532 * locals.var_log_tratio_dn10)) / p.p521))), (p.p518 * (assign96480_e149094 * (((-((locals.var_eg_dn11 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn11))) + (p.p532 * locals.var_log_tratio_dn11)) / p.p521))), (p.p518 * (assign96480_e149094 * (((-((locals.var_eg_dn14 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn14))) + (p.p532 * locals.var_log_tratio_dn14)) / p.p521))),)
    } else {
        (locals.var_jsswg2, locals.var_jsswg2_dn0, locals.var_jsswg2_dn2, locals.var_jsswg2_dn4, locals.var_jsswg2_dn5, locals.var_jsswg2_dn6, locals.var_jsswg2_dn7, locals.var_jsswg2_dn8, locals.var_jsswg2_dn9, locals.var_jsswg2_dn10, locals.var_jsswg2_dn11, locals.var_jsswg2_dn14,)
    }
};
        locals.var_jsswg2 = assign96480_e149097;
        locals.var_jsswg2_dn0 = assign96480_e149097_d_n0;
        locals.var_jsswg2_dn2 = assign96480_e149097_d_n2;
        locals.var_jsswg2_dn4 = assign96480_e149097_d_n4;
        locals.var_jsswg2_dn5 = assign96480_e149097_d_n5;
        locals.var_jsswg2_dn6 = assign96480_e149097_d_n6;
        locals.var_jsswg2_dn7 = assign96480_e149097_d_n7;
        locals.var_jsswg2_dn8 = assign96480_e149097_d_n8;
        locals.var_jsswg2_dn9 = assign96480_e149097_d_n9;
        locals.var_jsswg2_dn10 = assign96480_e149097_d_n10;
        locals.var_jsswg2_dn11 = assign96480_e149097_d_n11;
        locals.var_jsswg2_dn14 = assign96480_e149097_d_n14;
        locals.var_jsswg2_rv = 0.0;

        let assign96490_e149100: f64 = if p.p48 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2241 = assign96490_e149100;
        locals.var_guard2241_rv = 0.0;

        let assign96500_e149103: f64 = if p.p16 > locals.var_weff_nf { 1.0 } else { 0.0 };
        locals.var_guard2242 = assign96500_e149103;
        locals.var_guard2242_rv = 0.0;

        let (assign96510_e149113, assign96510_e149113_d_n0, assign96510_e149113_d_n2, assign96510_e149113_d_n4, assign96510_e149113_d_n5, assign96510_e149113_d_n6, assign96510_e149113_d_n7, assign96510_e149113_d_n8, assign96510_e149113_d_n9, assign96510_e149113_d_n10, assign96510_e149113_d_n11, assign96510_e149113_d_n14,) = {
    if (((locals.var_guard2237 != 0.0) && (locals.var_guard2241 != 0.0)) && (locals.var_guard2242 != 0.0)) {
        let assign96510_e149111: f64 = (p.p14 * locals.var_js);
        (assign96510_e149111, (p.p14 * locals.var_js_dn0), (p.p14 * locals.var_js_dn2), (p.p14 * locals.var_js_dn4), (p.p14 * locals.var_js_dn5), (p.p14 * locals.var_js_dn6), (p.p14 * locals.var_js_dn7), (p.p14 * locals.var_js_dn8), (p.p14 * locals.var_js_dn9), (p.p14 * locals.var_js_dn10), (p.p14 * locals.var_js_dn11), (p.p14 * locals.var_js_dn14),)
    } else {
        (locals.var_isbs_btm, locals.var_isbs_btm_dn0, locals.var_isbs_btm_dn2, locals.var_isbs_btm_dn4, locals.var_isbs_btm_dn5, locals.var_isbs_btm_dn6, locals.var_isbs_btm_dn7, locals.var_isbs_btm_dn8, locals.var_isbs_btm_dn9, locals.var_isbs_btm_dn10, locals.var_isbs_btm_dn11, locals.var_isbs_btm_dn14,)
    }
};
        locals.var_isbs_btm = assign96510_e149113;
        locals.var_isbs_btm_dn0 = assign96510_e149113_d_n0;
        locals.var_isbs_btm_dn2 = assign96510_e149113_d_n2;
        locals.var_isbs_btm_dn4 = assign96510_e149113_d_n4;
        locals.var_isbs_btm_dn5 = assign96510_e149113_d_n5;
        locals.var_isbs_btm_dn6 = assign96510_e149113_d_n6;
        locals.var_isbs_btm_dn7 = assign96510_e149113_d_n7;
        locals.var_isbs_btm_dn8 = assign96510_e149113_d_n8;
        locals.var_isbs_btm_dn9 = assign96510_e149113_d_n9;
        locals.var_isbs_btm_dn10 = assign96510_e149113_d_n10;
        locals.var_isbs_btm_dn11 = assign96510_e149113_d_n11;
        locals.var_isbs_btm_dn14 = assign96510_e149113_d_n14;
        locals.var_isbs_btm_rv = 0.0;

        let (assign96520_e149123, assign96520_e149123_d_n0, assign96520_e149123_d_n2, assign96520_e149123_d_n4, assign96520_e149123_d_n5, assign96520_e149123_d_n6, assign96520_e149123_d_n7, assign96520_e149123_d_n8, assign96520_e149123_d_n9, assign96520_e149123_d_n10, assign96520_e149123_d_n11, assign96520_e149123_d_n14,) = {
    if (((locals.var_guard2237 != 0.0) && (locals.var_guard2241 != 0.0)) && (locals.var_guard2242 != 0.0)) {
        let assign96520_e149121: f64 = (p.p14 * locals.var_js2);
        (assign96520_e149121, (p.p14 * locals.var_js2_dn0), (p.p14 * locals.var_js2_dn2), (p.p14 * locals.var_js2_dn4), (p.p14 * locals.var_js2_dn5), (p.p14 * locals.var_js2_dn6), (p.p14 * locals.var_js2_dn7), (p.p14 * locals.var_js2_dn8), (p.p14 * locals.var_js2_dn9), (p.p14 * locals.var_js2_dn10), (p.p14 * locals.var_js2_dn11), (p.p14 * locals.var_js2_dn14),)
    } else {
        (locals.var_isbs2_btm, locals.var_isbs2_btm_dn0, locals.var_isbs2_btm_dn2, locals.var_isbs2_btm_dn4, locals.var_isbs2_btm_dn5, locals.var_isbs2_btm_dn6, locals.var_isbs2_btm_dn7, locals.var_isbs2_btm_dn8, locals.var_isbs2_btm_dn9, locals.var_isbs2_btm_dn10, locals.var_isbs2_btm_dn11, locals.var_isbs2_btm_dn14,)
    }
};
        locals.var_isbs2_btm = assign96520_e149123;
        locals.var_isbs2_btm_dn0 = assign96520_e149123_d_n0;
        locals.var_isbs2_btm_dn2 = assign96520_e149123_d_n2;
        locals.var_isbs2_btm_dn4 = assign96520_e149123_d_n4;
        locals.var_isbs2_btm_dn5 = assign96520_e149123_d_n5;
        locals.var_isbs2_btm_dn6 = assign96520_e149123_d_n6;
        locals.var_isbs2_btm_dn7 = assign96520_e149123_d_n7;
        locals.var_isbs2_btm_dn8 = assign96520_e149123_d_n8;
        locals.var_isbs2_btm_dn9 = assign96520_e149123_d_n9;
        locals.var_isbs2_btm_dn10 = assign96520_e149123_d_n10;
        locals.var_isbs2_btm_dn11 = assign96520_e149123_d_n11;
        locals.var_isbs2_btm_dn14 = assign96520_e149123_d_n14;
        locals.var_isbs2_btm_rv = 0.0;

        let (assign96530_e149135, assign96530_e149135_d_n0, assign96530_e149135_d_n2, assign96530_e149135_d_n4, assign96530_e149135_d_n5, assign96530_e149135_d_n6, assign96530_e149135_d_n7, assign96530_e149135_d_n8, assign96530_e149135_d_n9, assign96530_e149135_d_n10, assign96530_e149135_d_n11, assign96530_e149135_d_n14,) = {
    if (((locals.var_guard2237 != 0.0) && (locals.var_guard2241 != 0.0)) && (locals.var_guard2242 != 0.0)) {
        let assign96530_e149131: f64 = (p.p16 - locals.var_weff_nf);
        let assign96530_e149133: f64 = (assign96530_e149131 * locals.var_jssw);
        (assign96530_e149133, (assign96530_e149131 * locals.var_jssw_dn0), (assign96530_e149131 * locals.var_jssw_dn2), (assign96530_e149131 * locals.var_jssw_dn4), (assign96530_e149131 * locals.var_jssw_dn5), (assign96530_e149131 * locals.var_jssw_dn6), (assign96530_e149131 * locals.var_jssw_dn7), (assign96530_e149131 * locals.var_jssw_dn8), (assign96530_e149131 * locals.var_jssw_dn9), (assign96530_e149131 * locals.var_jssw_dn10), (assign96530_e149131 * locals.var_jssw_dn11), (assign96530_e149131 * locals.var_jssw_dn14),)
    } else {
        (locals.var_isbs_sws, locals.var_isbs_sws_dn0, locals.var_isbs_sws_dn2, locals.var_isbs_sws_dn4, locals.var_isbs_sws_dn5, locals.var_isbs_sws_dn6, locals.var_isbs_sws_dn7, locals.var_isbs_sws_dn8, locals.var_isbs_sws_dn9, locals.var_isbs_sws_dn10, locals.var_isbs_sws_dn11, locals.var_isbs_sws_dn14,)
    }
};
        locals.var_isbs_sws = assign96530_e149135;
        locals.var_isbs_sws_dn0 = assign96530_e149135_d_n0;
        locals.var_isbs_sws_dn2 = assign96530_e149135_d_n2;
        locals.var_isbs_sws_dn4 = assign96530_e149135_d_n4;
        locals.var_isbs_sws_dn5 = assign96530_e149135_d_n5;
        locals.var_isbs_sws_dn6 = assign96530_e149135_d_n6;
        locals.var_isbs_sws_dn7 = assign96530_e149135_d_n7;
        locals.var_isbs_sws_dn8 = assign96530_e149135_d_n8;
        locals.var_isbs_sws_dn9 = assign96530_e149135_d_n9;
        locals.var_isbs_sws_dn10 = assign96530_e149135_d_n10;
        locals.var_isbs_sws_dn11 = assign96530_e149135_d_n11;
        locals.var_isbs_sws_dn14 = assign96530_e149135_d_n14;
        locals.var_isbs_sws_rv = 0.0;

        let (assign96540_e149147, assign96540_e149147_d_n0, assign96540_e149147_d_n2, assign96540_e149147_d_n4, assign96540_e149147_d_n5, assign96540_e149147_d_n6, assign96540_e149147_d_n7, assign96540_e149147_d_n8, assign96540_e149147_d_n9, assign96540_e149147_d_n10, assign96540_e149147_d_n11, assign96540_e149147_d_n14,) = {
    if (((locals.var_guard2237 != 0.0) && (locals.var_guard2241 != 0.0)) && (locals.var_guard2242 != 0.0)) {
        let assign96540_e149143: f64 = (p.p16 - locals.var_weff_nf);
        let assign96540_e149145: f64 = (assign96540_e149143 * locals.var_jssw2);
        (assign96540_e149145, (assign96540_e149143 * locals.var_jssw2_dn0), (assign96540_e149143 * locals.var_jssw2_dn2), (assign96540_e149143 * locals.var_jssw2_dn4), (assign96540_e149143 * locals.var_jssw2_dn5), (assign96540_e149143 * locals.var_jssw2_dn6), (assign96540_e149143 * locals.var_jssw2_dn7), (assign96540_e149143 * locals.var_jssw2_dn8), (assign96540_e149143 * locals.var_jssw2_dn9), (assign96540_e149143 * locals.var_jssw2_dn10), (assign96540_e149143 * locals.var_jssw2_dn11), (assign96540_e149143 * locals.var_jssw2_dn14),)
    } else {
        (locals.var_isbs2_sws, locals.var_isbs2_sws_dn0, locals.var_isbs2_sws_dn2, locals.var_isbs2_sws_dn4, locals.var_isbs2_sws_dn5, locals.var_isbs2_sws_dn6, locals.var_isbs2_sws_dn7, locals.var_isbs2_sws_dn8, locals.var_isbs2_sws_dn9, locals.var_isbs2_sws_dn10, locals.var_isbs2_sws_dn11, locals.var_isbs2_sws_dn14,)
    }
};
        locals.var_isbs2_sws = assign96540_e149147;
        locals.var_isbs2_sws_dn0 = assign96540_e149147_d_n0;
        locals.var_isbs2_sws_dn2 = assign96540_e149147_d_n2;
        locals.var_isbs2_sws_dn4 = assign96540_e149147_d_n4;
        locals.var_isbs2_sws_dn5 = assign96540_e149147_d_n5;
        locals.var_isbs2_sws_dn6 = assign96540_e149147_d_n6;
        locals.var_isbs2_sws_dn7 = assign96540_e149147_d_n7;
        locals.var_isbs2_sws_dn8 = assign96540_e149147_d_n8;
        locals.var_isbs2_sws_dn9 = assign96540_e149147_d_n9;
        locals.var_isbs2_sws_dn10 = assign96540_e149147_d_n10;
        locals.var_isbs2_sws_dn11 = assign96540_e149147_d_n11;
        locals.var_isbs2_sws_dn14 = assign96540_e149147_d_n14;
        locals.var_isbs2_sws_rv = 0.0;

        let (assign96550_e149157, assign96550_e149157_d_n0, assign96550_e149157_d_n2, assign96550_e149157_d_n4, assign96550_e149157_d_n5, assign96550_e149157_d_n6, assign96550_e149157_d_n7, assign96550_e149157_d_n8, assign96550_e149157_d_n9, assign96550_e149157_d_n10, assign96550_e149157_d_n11, assign96550_e149157_d_n14,) = {
    if (((locals.var_guard2237 != 0.0) && (locals.var_guard2241 != 0.0)) && (locals.var_guard2242 != 0.0)) {
        let assign96550_e149155: f64 = (locals.var_weff_nf * locals.var_jsswg);
        (assign96550_e149155, (locals.var_weff_nf * locals.var_jsswg_dn0), (locals.var_weff_nf * locals.var_jsswg_dn2), (locals.var_weff_nf * locals.var_jsswg_dn4), (locals.var_weff_nf * locals.var_jsswg_dn5), (locals.var_weff_nf * locals.var_jsswg_dn6), (locals.var_weff_nf * locals.var_jsswg_dn7), (locals.var_weff_nf * locals.var_jsswg_dn8), (locals.var_weff_nf * locals.var_jsswg_dn9), (locals.var_weff_nf * locals.var_jsswg_dn10), (locals.var_weff_nf * locals.var_jsswg_dn11), (locals.var_weff_nf * locals.var_jsswg_dn14),)
    } else {
        (locals.var_isbs_swg, locals.var_isbs_swg_dn0, locals.var_isbs_swg_dn2, locals.var_isbs_swg_dn4, locals.var_isbs_swg_dn5, locals.var_isbs_swg_dn6, locals.var_isbs_swg_dn7, locals.var_isbs_swg_dn8, locals.var_isbs_swg_dn9, locals.var_isbs_swg_dn10, locals.var_isbs_swg_dn11, locals.var_isbs_swg_dn14,)
    }
};
        locals.var_isbs_swg = assign96550_e149157;
        locals.var_isbs_swg_dn0 = assign96550_e149157_d_n0;
        locals.var_isbs_swg_dn2 = assign96550_e149157_d_n2;
        locals.var_isbs_swg_dn4 = assign96550_e149157_d_n4;
        locals.var_isbs_swg_dn5 = assign96550_e149157_d_n5;
        locals.var_isbs_swg_dn6 = assign96550_e149157_d_n6;
        locals.var_isbs_swg_dn7 = assign96550_e149157_d_n7;
        locals.var_isbs_swg_dn8 = assign96550_e149157_d_n8;
        locals.var_isbs_swg_dn9 = assign96550_e149157_d_n9;
        locals.var_isbs_swg_dn10 = assign96550_e149157_d_n10;
        locals.var_isbs_swg_dn11 = assign96550_e149157_d_n11;
        locals.var_isbs_swg_dn14 = assign96550_e149157_d_n14;
        locals.var_isbs_swg_rv = 0.0;

        let (assign96560_e149167, assign96560_e149167_d_n0, assign96560_e149167_d_n2, assign96560_e149167_d_n4, assign96560_e149167_d_n5, assign96560_e149167_d_n6, assign96560_e149167_d_n7, assign96560_e149167_d_n8, assign96560_e149167_d_n9, assign96560_e149167_d_n10, assign96560_e149167_d_n11, assign96560_e149167_d_n14,) = {
    if (((locals.var_guard2237 != 0.0) && (locals.var_guard2241 != 0.0)) && (locals.var_guard2242 != 0.0)) {
        let assign96560_e149165: f64 = (locals.var_weff_nf * locals.var_jsswg2);
        (assign96560_e149165, (locals.var_weff_nf * locals.var_jsswg2_dn0), (locals.var_weff_nf * locals.var_jsswg2_dn2), (locals.var_weff_nf * locals.var_jsswg2_dn4), (locals.var_weff_nf * locals.var_jsswg2_dn5), (locals.var_weff_nf * locals.var_jsswg2_dn6), (locals.var_weff_nf * locals.var_jsswg2_dn7), (locals.var_weff_nf * locals.var_jsswg2_dn8), (locals.var_weff_nf * locals.var_jsswg2_dn9), (locals.var_weff_nf * locals.var_jsswg2_dn10), (locals.var_weff_nf * locals.var_jsswg2_dn11), (locals.var_weff_nf * locals.var_jsswg2_dn14),)
    } else {
        (locals.var_isbs2_swg, locals.var_isbs2_swg_dn0, locals.var_isbs2_swg_dn2, locals.var_isbs2_swg_dn4, locals.var_isbs2_swg_dn5, locals.var_isbs2_swg_dn6, locals.var_isbs2_swg_dn7, locals.var_isbs2_swg_dn8, locals.var_isbs2_swg_dn9, locals.var_isbs2_swg_dn10, locals.var_isbs2_swg_dn11, locals.var_isbs2_swg_dn14,)
    }
};
        locals.var_isbs2_swg = assign96560_e149167;
        locals.var_isbs2_swg_dn0 = assign96560_e149167_d_n0;
        locals.var_isbs2_swg_dn2 = assign96560_e149167_d_n2;
        locals.var_isbs2_swg_dn4 = assign96560_e149167_d_n4;
        locals.var_isbs2_swg_dn5 = assign96560_e149167_d_n5;
        locals.var_isbs2_swg_dn6 = assign96560_e149167_d_n6;
        locals.var_isbs2_swg_dn7 = assign96560_e149167_d_n7;
        locals.var_isbs2_swg_dn8 = assign96560_e149167_d_n8;
        locals.var_isbs2_swg_dn9 = assign96560_e149167_d_n9;
        locals.var_isbs2_swg_dn10 = assign96560_e149167_d_n10;
        locals.var_isbs2_swg_dn11 = assign96560_e149167_d_n11;
        locals.var_isbs2_swg_dn14 = assign96560_e149167_d_n14;
        locals.var_isbs2_swg_rv = 0.0;

        let (assign96570_e149178, assign96570_e149178_d_n0, assign96570_e149178_d_n2, assign96570_e149178_d_n4, assign96570_e149178_d_n5, assign96570_e149178_d_n6, assign96570_e149178_d_n7, assign96570_e149178_d_n8, assign96570_e149178_d_n9, assign96570_e149178_d_n10, assign96570_e149178_d_n11, assign96570_e149178_d_n14,) = {
    if (((locals.var_guard2237 != 0.0) && (locals.var_guard2241 != 0.0)) && (locals.var_guard2242 == 0.0)) {
        let assign96570_e149176: f64 = (p.p14 * locals.var_js);
        (assign96570_e149176, (p.p14 * locals.var_js_dn0), (p.p14 * locals.var_js_dn2), (p.p14 * locals.var_js_dn4), (p.p14 * locals.var_js_dn5), (p.p14 * locals.var_js_dn6), (p.p14 * locals.var_js_dn7), (p.p14 * locals.var_js_dn8), (p.p14 * locals.var_js_dn9), (p.p14 * locals.var_js_dn10), (p.p14 * locals.var_js_dn11), (p.p14 * locals.var_js_dn14),)
    } else {
        (locals.var_isbs_btm, locals.var_isbs_btm_dn0, locals.var_isbs_btm_dn2, locals.var_isbs_btm_dn4, locals.var_isbs_btm_dn5, locals.var_isbs_btm_dn6, locals.var_isbs_btm_dn7, locals.var_isbs_btm_dn8, locals.var_isbs_btm_dn9, locals.var_isbs_btm_dn10, locals.var_isbs_btm_dn11, locals.var_isbs_btm_dn14,)
    }
};
        locals.var_isbs_btm = assign96570_e149178;
        locals.var_isbs_btm_dn0 = assign96570_e149178_d_n0;
        locals.var_isbs_btm_dn2 = assign96570_e149178_d_n2;
        locals.var_isbs_btm_dn4 = assign96570_e149178_d_n4;
        locals.var_isbs_btm_dn5 = assign96570_e149178_d_n5;
        locals.var_isbs_btm_dn6 = assign96570_e149178_d_n6;
        locals.var_isbs_btm_dn7 = assign96570_e149178_d_n7;
        locals.var_isbs_btm_dn8 = assign96570_e149178_d_n8;
        locals.var_isbs_btm_dn9 = assign96570_e149178_d_n9;
        locals.var_isbs_btm_dn10 = assign96570_e149178_d_n10;
        locals.var_isbs_btm_dn11 = assign96570_e149178_d_n11;
        locals.var_isbs_btm_dn14 = assign96570_e149178_d_n14;
        locals.var_isbs_btm_rv = 0.0;

        let (assign96580_e149189, assign96580_e149189_d_n0, assign96580_e149189_d_n2, assign96580_e149189_d_n4, assign96580_e149189_d_n5, assign96580_e149189_d_n6, assign96580_e149189_d_n7, assign96580_e149189_d_n8, assign96580_e149189_d_n9, assign96580_e149189_d_n10, assign96580_e149189_d_n11, assign96580_e149189_d_n14,) = {
    if (((locals.var_guard2237 != 0.0) && (locals.var_guard2241 != 0.0)) && (locals.var_guard2242 == 0.0)) {
        let assign96580_e149187: f64 = (p.p14 * locals.var_js2);
        (assign96580_e149187, (p.p14 * locals.var_js2_dn0), (p.p14 * locals.var_js2_dn2), (p.p14 * locals.var_js2_dn4), (p.p14 * locals.var_js2_dn5), (p.p14 * locals.var_js2_dn6), (p.p14 * locals.var_js2_dn7), (p.p14 * locals.var_js2_dn8), (p.p14 * locals.var_js2_dn9), (p.p14 * locals.var_js2_dn10), (p.p14 * locals.var_js2_dn11), (p.p14 * locals.var_js2_dn14),)
    } else {
        (locals.var_isbs2_btm, locals.var_isbs2_btm_dn0, locals.var_isbs2_btm_dn2, locals.var_isbs2_btm_dn4, locals.var_isbs2_btm_dn5, locals.var_isbs2_btm_dn6, locals.var_isbs2_btm_dn7, locals.var_isbs2_btm_dn8, locals.var_isbs2_btm_dn9, locals.var_isbs2_btm_dn10, locals.var_isbs2_btm_dn11, locals.var_isbs2_btm_dn14,)
    }
};
        locals.var_isbs2_btm = assign96580_e149189;
        locals.var_isbs2_btm_dn0 = assign96580_e149189_d_n0;
        locals.var_isbs2_btm_dn2 = assign96580_e149189_d_n2;
        locals.var_isbs2_btm_dn4 = assign96580_e149189_d_n4;
        locals.var_isbs2_btm_dn5 = assign96580_e149189_d_n5;
        locals.var_isbs2_btm_dn6 = assign96580_e149189_d_n6;
        locals.var_isbs2_btm_dn7 = assign96580_e149189_d_n7;
        locals.var_isbs2_btm_dn8 = assign96580_e149189_d_n8;
        locals.var_isbs2_btm_dn9 = assign96580_e149189_d_n9;
        locals.var_isbs2_btm_dn10 = assign96580_e149189_d_n10;
        locals.var_isbs2_btm_dn11 = assign96580_e149189_d_n11;
        locals.var_isbs2_btm_dn14 = assign96580_e149189_d_n14;
        locals.var_isbs2_btm_rv = 0.0;

        let (assign96590_e149198, assign96590_e149198_d_n0, assign96590_e149198_d_n2, assign96590_e149198_d_n4, assign96590_e149198_d_n5, assign96590_e149198_d_n6, assign96590_e149198_d_n7, assign96590_e149198_d_n8, assign96590_e149198_d_n9, assign96590_e149198_d_n10, assign96590_e149198_d_n11, assign96590_e149198_d_n14,) = {
    if (((locals.var_guard2237 != 0.0) && (locals.var_guard2241 != 0.0)) && (locals.var_guard2242 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isbs_sws, locals.var_isbs_sws_dn0, locals.var_isbs_sws_dn2, locals.var_isbs_sws_dn4, locals.var_isbs_sws_dn5, locals.var_isbs_sws_dn6, locals.var_isbs_sws_dn7, locals.var_isbs_sws_dn8, locals.var_isbs_sws_dn9, locals.var_isbs_sws_dn10, locals.var_isbs_sws_dn11, locals.var_isbs_sws_dn14,)
    }
};
        locals.var_isbs_sws = assign96590_e149198;
        locals.var_isbs_sws_dn0 = assign96590_e149198_d_n0;
        locals.var_isbs_sws_dn2 = assign96590_e149198_d_n2;
        locals.var_isbs_sws_dn4 = assign96590_e149198_d_n4;
        locals.var_isbs_sws_dn5 = assign96590_e149198_d_n5;
        locals.var_isbs_sws_dn6 = assign96590_e149198_d_n6;
        locals.var_isbs_sws_dn7 = assign96590_e149198_d_n7;
        locals.var_isbs_sws_dn8 = assign96590_e149198_d_n8;
        locals.var_isbs_sws_dn9 = assign96590_e149198_d_n9;
        locals.var_isbs_sws_dn10 = assign96590_e149198_d_n10;
        locals.var_isbs_sws_dn11 = assign96590_e149198_d_n11;
        locals.var_isbs_sws_dn14 = assign96590_e149198_d_n14;
        locals.var_isbs_sws_rv = 0.0;

        let (assign96600_e149207, assign96600_e149207_d_n0, assign96600_e149207_d_n2, assign96600_e149207_d_n4, assign96600_e149207_d_n5, assign96600_e149207_d_n6, assign96600_e149207_d_n7, assign96600_e149207_d_n8, assign96600_e149207_d_n9, assign96600_e149207_d_n10, assign96600_e149207_d_n11, assign96600_e149207_d_n14,) = {
    if (((locals.var_guard2237 != 0.0) && (locals.var_guard2241 != 0.0)) && (locals.var_guard2242 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isbs2_sws, locals.var_isbs2_sws_dn0, locals.var_isbs2_sws_dn2, locals.var_isbs2_sws_dn4, locals.var_isbs2_sws_dn5, locals.var_isbs2_sws_dn6, locals.var_isbs2_sws_dn7, locals.var_isbs2_sws_dn8, locals.var_isbs2_sws_dn9, locals.var_isbs2_sws_dn10, locals.var_isbs2_sws_dn11, locals.var_isbs2_sws_dn14,)
    }
};
        locals.var_isbs2_sws = assign96600_e149207;
        locals.var_isbs2_sws_dn0 = assign96600_e149207_d_n0;
        locals.var_isbs2_sws_dn2 = assign96600_e149207_d_n2;
        locals.var_isbs2_sws_dn4 = assign96600_e149207_d_n4;
        locals.var_isbs2_sws_dn5 = assign96600_e149207_d_n5;
        locals.var_isbs2_sws_dn6 = assign96600_e149207_d_n6;
        locals.var_isbs2_sws_dn7 = assign96600_e149207_d_n7;
        locals.var_isbs2_sws_dn8 = assign96600_e149207_d_n8;
        locals.var_isbs2_sws_dn9 = assign96600_e149207_d_n9;
        locals.var_isbs2_sws_dn10 = assign96600_e149207_d_n10;
        locals.var_isbs2_sws_dn11 = assign96600_e149207_d_n11;
        locals.var_isbs2_sws_dn14 = assign96600_e149207_d_n14;
        locals.var_isbs2_sws_rv = 0.0;

        let (assign96610_e149218, assign96610_e149218_d_n0, assign96610_e149218_d_n2, assign96610_e149218_d_n4, assign96610_e149218_d_n5, assign96610_e149218_d_n6, assign96610_e149218_d_n7, assign96610_e149218_d_n8, assign96610_e149218_d_n9, assign96610_e149218_d_n10, assign96610_e149218_d_n11, assign96610_e149218_d_n14,) = {
    if (((locals.var_guard2237 != 0.0) && (locals.var_guard2241 != 0.0)) && (locals.var_guard2242 == 0.0)) {
        let assign96610_e149216: f64 = (p.p16 * locals.var_jsswg);
        (assign96610_e149216, (p.p16 * locals.var_jsswg_dn0), (p.p16 * locals.var_jsswg_dn2), (p.p16 * locals.var_jsswg_dn4), (p.p16 * locals.var_jsswg_dn5), (p.p16 * locals.var_jsswg_dn6), (p.p16 * locals.var_jsswg_dn7), (p.p16 * locals.var_jsswg_dn8), (p.p16 * locals.var_jsswg_dn9), (p.p16 * locals.var_jsswg_dn10), (p.p16 * locals.var_jsswg_dn11), (p.p16 * locals.var_jsswg_dn14),)
    } else {
        (locals.var_isbs_swg, locals.var_isbs_swg_dn0, locals.var_isbs_swg_dn2, locals.var_isbs_swg_dn4, locals.var_isbs_swg_dn5, locals.var_isbs_swg_dn6, locals.var_isbs_swg_dn7, locals.var_isbs_swg_dn8, locals.var_isbs_swg_dn9, locals.var_isbs_swg_dn10, locals.var_isbs_swg_dn11, locals.var_isbs_swg_dn14,)
    }
};
        locals.var_isbs_swg = assign96610_e149218;
        locals.var_isbs_swg_dn0 = assign96610_e149218_d_n0;
        locals.var_isbs_swg_dn2 = assign96610_e149218_d_n2;
        locals.var_isbs_swg_dn4 = assign96610_e149218_d_n4;
        locals.var_isbs_swg_dn5 = assign96610_e149218_d_n5;
        locals.var_isbs_swg_dn6 = assign96610_e149218_d_n6;
        locals.var_isbs_swg_dn7 = assign96610_e149218_d_n7;
        locals.var_isbs_swg_dn8 = assign96610_e149218_d_n8;
        locals.var_isbs_swg_dn9 = assign96610_e149218_d_n9;
        locals.var_isbs_swg_dn10 = assign96610_e149218_d_n10;
        locals.var_isbs_swg_dn11 = assign96610_e149218_d_n11;
        locals.var_isbs_swg_dn14 = assign96610_e149218_d_n14;
        locals.var_isbs_swg_rv = 0.0;

        let (assign96620_e149229, assign96620_e149229_d_n0, assign96620_e149229_d_n2, assign96620_e149229_d_n4, assign96620_e149229_d_n5, assign96620_e149229_d_n6, assign96620_e149229_d_n7, assign96620_e149229_d_n8, assign96620_e149229_d_n9, assign96620_e149229_d_n10, assign96620_e149229_d_n11, assign96620_e149229_d_n14,) = {
    if (((locals.var_guard2237 != 0.0) && (locals.var_guard2241 != 0.0)) && (locals.var_guard2242 == 0.0)) {
        let assign96620_e149227: f64 = (p.p16 * locals.var_jsswg2);
        (assign96620_e149227, (p.p16 * locals.var_jsswg2_dn0), (p.p16 * locals.var_jsswg2_dn2), (p.p16 * locals.var_jsswg2_dn4), (p.p16 * locals.var_jsswg2_dn5), (p.p16 * locals.var_jsswg2_dn6), (p.p16 * locals.var_jsswg2_dn7), (p.p16 * locals.var_jsswg2_dn8), (p.p16 * locals.var_jsswg2_dn9), (p.p16 * locals.var_jsswg2_dn10), (p.p16 * locals.var_jsswg2_dn11), (p.p16 * locals.var_jsswg2_dn14),)
    } else {
        (locals.var_isbs2_swg, locals.var_isbs2_swg_dn0, locals.var_isbs2_swg_dn2, locals.var_isbs2_swg_dn4, locals.var_isbs2_swg_dn5, locals.var_isbs2_swg_dn6, locals.var_isbs2_swg_dn7, locals.var_isbs2_swg_dn8, locals.var_isbs2_swg_dn9, locals.var_isbs2_swg_dn10, locals.var_isbs2_swg_dn11, locals.var_isbs2_swg_dn14,)
    }
};
        locals.var_isbs2_swg = assign96620_e149229;
        locals.var_isbs2_swg_dn0 = assign96620_e149229_d_n0;
        locals.var_isbs2_swg_dn2 = assign96620_e149229_d_n2;
        locals.var_isbs2_swg_dn4 = assign96620_e149229_d_n4;
        locals.var_isbs2_swg_dn5 = assign96620_e149229_d_n5;
        locals.var_isbs2_swg_dn6 = assign96620_e149229_d_n6;
        locals.var_isbs2_swg_dn7 = assign96620_e149229_d_n7;
        locals.var_isbs2_swg_dn8 = assign96620_e149229_d_n8;
        locals.var_isbs2_swg_dn9 = assign96620_e149229_d_n9;
        locals.var_isbs2_swg_dn10 = assign96620_e149229_d_n10;
        locals.var_isbs2_swg_dn11 = assign96620_e149229_d_n11;
        locals.var_isbs2_swg_dn14 = assign96620_e149229_d_n14;
        locals.var_isbs2_swg_rv = 0.0;

        let (assign96630_e149238, assign96630_e149238_d_n0, assign96630_e149238_d_n2, assign96630_e149238_d_n4, assign96630_e149238_d_n5, assign96630_e149238_d_n6, assign96630_e149238_d_n7, assign96630_e149238_d_n8, assign96630_e149238_d_n9, assign96630_e149238_d_n10, assign96630_e149238_d_n11, assign96630_e149238_d_n14,) = {
    if ((locals.var_guard2237 != 0.0) && (locals.var_guard2241 == 0.0)) {
        let assign96630_e149236: f64 = (p.p14 * locals.var_js);
        (assign96630_e149236, (p.p14 * locals.var_js_dn0), (p.p14 * locals.var_js_dn2), (p.p14 * locals.var_js_dn4), (p.p14 * locals.var_js_dn5), (p.p14 * locals.var_js_dn6), (p.p14 * locals.var_js_dn7), (p.p14 * locals.var_js_dn8), (p.p14 * locals.var_js_dn9), (p.p14 * locals.var_js_dn10), (p.p14 * locals.var_js_dn11), (p.p14 * locals.var_js_dn14),)
    } else {
        (locals.var_isbs_btm, locals.var_isbs_btm_dn0, locals.var_isbs_btm_dn2, locals.var_isbs_btm_dn4, locals.var_isbs_btm_dn5, locals.var_isbs_btm_dn6, locals.var_isbs_btm_dn7, locals.var_isbs_btm_dn8, locals.var_isbs_btm_dn9, locals.var_isbs_btm_dn10, locals.var_isbs_btm_dn11, locals.var_isbs_btm_dn14,)
    }
};
        locals.var_isbs_btm = assign96630_e149238;
        locals.var_isbs_btm_dn0 = assign96630_e149238_d_n0;
        locals.var_isbs_btm_dn2 = assign96630_e149238_d_n2;
        locals.var_isbs_btm_dn4 = assign96630_e149238_d_n4;
        locals.var_isbs_btm_dn5 = assign96630_e149238_d_n5;
        locals.var_isbs_btm_dn6 = assign96630_e149238_d_n6;
        locals.var_isbs_btm_dn7 = assign96630_e149238_d_n7;
        locals.var_isbs_btm_dn8 = assign96630_e149238_d_n8;
        locals.var_isbs_btm_dn9 = assign96630_e149238_d_n9;
        locals.var_isbs_btm_dn10 = assign96630_e149238_d_n10;
        locals.var_isbs_btm_dn11 = assign96630_e149238_d_n11;
        locals.var_isbs_btm_dn14 = assign96630_e149238_d_n14;
        locals.var_isbs_btm_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_375(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign96640_e149247, assign96640_e149247_d_n0, assign96640_e149247_d_n2, assign96640_e149247_d_n4, assign96640_e149247_d_n5, assign96640_e149247_d_n6, assign96640_e149247_d_n7, assign96640_e149247_d_n8, assign96640_e149247_d_n9, assign96640_e149247_d_n10, assign96640_e149247_d_n11, assign96640_e149247_d_n14,) = {
    if ((locals.var_guard2237 != 0.0) && (locals.var_guard2241 == 0.0)) {
        let assign96640_e149245: f64 = (p.p14 * locals.var_js2);
        (assign96640_e149245, (p.p14 * locals.var_js2_dn0), (p.p14 * locals.var_js2_dn2), (p.p14 * locals.var_js2_dn4), (p.p14 * locals.var_js2_dn5), (p.p14 * locals.var_js2_dn6), (p.p14 * locals.var_js2_dn7), (p.p14 * locals.var_js2_dn8), (p.p14 * locals.var_js2_dn9), (p.p14 * locals.var_js2_dn10), (p.p14 * locals.var_js2_dn11), (p.p14 * locals.var_js2_dn14),)
    } else {
        (locals.var_isbs2_btm, locals.var_isbs2_btm_dn0, locals.var_isbs2_btm_dn2, locals.var_isbs2_btm_dn4, locals.var_isbs2_btm_dn5, locals.var_isbs2_btm_dn6, locals.var_isbs2_btm_dn7, locals.var_isbs2_btm_dn8, locals.var_isbs2_btm_dn9, locals.var_isbs2_btm_dn10, locals.var_isbs2_btm_dn11, locals.var_isbs2_btm_dn14,)
    }
};
        locals.var_isbs2_btm = assign96640_e149247;
        locals.var_isbs2_btm_dn0 = assign96640_e149247_d_n0;
        locals.var_isbs2_btm_dn2 = assign96640_e149247_d_n2;
        locals.var_isbs2_btm_dn4 = assign96640_e149247_d_n4;
        locals.var_isbs2_btm_dn5 = assign96640_e149247_d_n5;
        locals.var_isbs2_btm_dn6 = assign96640_e149247_d_n6;
        locals.var_isbs2_btm_dn7 = assign96640_e149247_d_n7;
        locals.var_isbs2_btm_dn8 = assign96640_e149247_d_n8;
        locals.var_isbs2_btm_dn9 = assign96640_e149247_d_n9;
        locals.var_isbs2_btm_dn10 = assign96640_e149247_d_n10;
        locals.var_isbs2_btm_dn11 = assign96640_e149247_d_n11;
        locals.var_isbs2_btm_dn14 = assign96640_e149247_d_n14;
        locals.var_isbs2_btm_rv = 0.0;

        let (assign96650_e149256, assign96650_e149256_d_n0, assign96650_e149256_d_n2, assign96650_e149256_d_n4, assign96650_e149256_d_n5, assign96650_e149256_d_n6, assign96650_e149256_d_n7, assign96650_e149256_d_n8, assign96650_e149256_d_n9, assign96650_e149256_d_n10, assign96650_e149256_d_n11, assign96650_e149256_d_n14,) = {
    if ((locals.var_guard2237 != 0.0) && (locals.var_guard2241 == 0.0)) {
        let assign96650_e149254: f64 = (p.p16 * locals.var_jssw);
        (assign96650_e149254, (p.p16 * locals.var_jssw_dn0), (p.p16 * locals.var_jssw_dn2), (p.p16 * locals.var_jssw_dn4), (p.p16 * locals.var_jssw_dn5), (p.p16 * locals.var_jssw_dn6), (p.p16 * locals.var_jssw_dn7), (p.p16 * locals.var_jssw_dn8), (p.p16 * locals.var_jssw_dn9), (p.p16 * locals.var_jssw_dn10), (p.p16 * locals.var_jssw_dn11), (p.p16 * locals.var_jssw_dn14),)
    } else {
        (locals.var_isbs_sws, locals.var_isbs_sws_dn0, locals.var_isbs_sws_dn2, locals.var_isbs_sws_dn4, locals.var_isbs_sws_dn5, locals.var_isbs_sws_dn6, locals.var_isbs_sws_dn7, locals.var_isbs_sws_dn8, locals.var_isbs_sws_dn9, locals.var_isbs_sws_dn10, locals.var_isbs_sws_dn11, locals.var_isbs_sws_dn14,)
    }
};
        locals.var_isbs_sws = assign96650_e149256;
        locals.var_isbs_sws_dn0 = assign96650_e149256_d_n0;
        locals.var_isbs_sws_dn2 = assign96650_e149256_d_n2;
        locals.var_isbs_sws_dn4 = assign96650_e149256_d_n4;
        locals.var_isbs_sws_dn5 = assign96650_e149256_d_n5;
        locals.var_isbs_sws_dn6 = assign96650_e149256_d_n6;
        locals.var_isbs_sws_dn7 = assign96650_e149256_d_n7;
        locals.var_isbs_sws_dn8 = assign96650_e149256_d_n8;
        locals.var_isbs_sws_dn9 = assign96650_e149256_d_n9;
        locals.var_isbs_sws_dn10 = assign96650_e149256_d_n10;
        locals.var_isbs_sws_dn11 = assign96650_e149256_d_n11;
        locals.var_isbs_sws_dn14 = assign96650_e149256_d_n14;
        locals.var_isbs_sws_rv = 0.0;

        let (assign96660_e149265, assign96660_e149265_d_n0, assign96660_e149265_d_n2, assign96660_e149265_d_n4, assign96660_e149265_d_n5, assign96660_e149265_d_n6, assign96660_e149265_d_n7, assign96660_e149265_d_n8, assign96660_e149265_d_n9, assign96660_e149265_d_n10, assign96660_e149265_d_n11, assign96660_e149265_d_n14,) = {
    if ((locals.var_guard2237 != 0.0) && (locals.var_guard2241 == 0.0)) {
        let assign96660_e149263: f64 = (p.p16 * locals.var_jssw2);
        (assign96660_e149263, (p.p16 * locals.var_jssw2_dn0), (p.p16 * locals.var_jssw2_dn2), (p.p16 * locals.var_jssw2_dn4), (p.p16 * locals.var_jssw2_dn5), (p.p16 * locals.var_jssw2_dn6), (p.p16 * locals.var_jssw2_dn7), (p.p16 * locals.var_jssw2_dn8), (p.p16 * locals.var_jssw2_dn9), (p.p16 * locals.var_jssw2_dn10), (p.p16 * locals.var_jssw2_dn11), (p.p16 * locals.var_jssw2_dn14),)
    } else {
        (locals.var_isbs2_sws, locals.var_isbs2_sws_dn0, locals.var_isbs2_sws_dn2, locals.var_isbs2_sws_dn4, locals.var_isbs2_sws_dn5, locals.var_isbs2_sws_dn6, locals.var_isbs2_sws_dn7, locals.var_isbs2_sws_dn8, locals.var_isbs2_sws_dn9, locals.var_isbs2_sws_dn10, locals.var_isbs2_sws_dn11, locals.var_isbs2_sws_dn14,)
    }
};
        locals.var_isbs2_sws = assign96660_e149265;
        locals.var_isbs2_sws_dn0 = assign96660_e149265_d_n0;
        locals.var_isbs2_sws_dn2 = assign96660_e149265_d_n2;
        locals.var_isbs2_sws_dn4 = assign96660_e149265_d_n4;
        locals.var_isbs2_sws_dn5 = assign96660_e149265_d_n5;
        locals.var_isbs2_sws_dn6 = assign96660_e149265_d_n6;
        locals.var_isbs2_sws_dn7 = assign96660_e149265_d_n7;
        locals.var_isbs2_sws_dn8 = assign96660_e149265_d_n8;
        locals.var_isbs2_sws_dn9 = assign96660_e149265_d_n9;
        locals.var_isbs2_sws_dn10 = assign96660_e149265_d_n10;
        locals.var_isbs2_sws_dn11 = assign96660_e149265_d_n11;
        locals.var_isbs2_sws_dn14 = assign96660_e149265_d_n14;
        locals.var_isbs2_sws_rv = 0.0;

        let (assign96670_e149272, assign96670_e149272_d_n0, assign96670_e149272_d_n2, assign96670_e149272_d_n4, assign96670_e149272_d_n5, assign96670_e149272_d_n6, assign96670_e149272_d_n7, assign96670_e149272_d_n8, assign96670_e149272_d_n9, assign96670_e149272_d_n10, assign96670_e149272_d_n11, assign96670_e149272_d_n14,) = {
    if ((locals.var_guard2237 != 0.0) && (locals.var_guard2241 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isbs_swg, locals.var_isbs_swg_dn0, locals.var_isbs_swg_dn2, locals.var_isbs_swg_dn4, locals.var_isbs_swg_dn5, locals.var_isbs_swg_dn6, locals.var_isbs_swg_dn7, locals.var_isbs_swg_dn8, locals.var_isbs_swg_dn9, locals.var_isbs_swg_dn10, locals.var_isbs_swg_dn11, locals.var_isbs_swg_dn14,)
    }
};
        locals.var_isbs_swg = assign96670_e149272;
        locals.var_isbs_swg_dn0 = assign96670_e149272_d_n0;
        locals.var_isbs_swg_dn2 = assign96670_e149272_d_n2;
        locals.var_isbs_swg_dn4 = assign96670_e149272_d_n4;
        locals.var_isbs_swg_dn5 = assign96670_e149272_d_n5;
        locals.var_isbs_swg_dn6 = assign96670_e149272_d_n6;
        locals.var_isbs_swg_dn7 = assign96670_e149272_d_n7;
        locals.var_isbs_swg_dn8 = assign96670_e149272_d_n8;
        locals.var_isbs_swg_dn9 = assign96670_e149272_d_n9;
        locals.var_isbs_swg_dn10 = assign96670_e149272_d_n10;
        locals.var_isbs_swg_dn11 = assign96670_e149272_d_n11;
        locals.var_isbs_swg_dn14 = assign96670_e149272_d_n14;
        locals.var_isbs_swg_rv = 0.0;

        let (assign96680_e149279, assign96680_e149279_d_n0, assign96680_e149279_d_n2, assign96680_e149279_d_n4, assign96680_e149279_d_n5, assign96680_e149279_d_n6, assign96680_e149279_d_n7, assign96680_e149279_d_n8, assign96680_e149279_d_n9, assign96680_e149279_d_n10, assign96680_e149279_d_n11, assign96680_e149279_d_n14,) = {
    if ((locals.var_guard2237 != 0.0) && (locals.var_guard2241 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isbs2_swg, locals.var_isbs2_swg_dn0, locals.var_isbs2_swg_dn2, locals.var_isbs2_swg_dn4, locals.var_isbs2_swg_dn5, locals.var_isbs2_swg_dn6, locals.var_isbs2_swg_dn7, locals.var_isbs2_swg_dn8, locals.var_isbs2_swg_dn9, locals.var_isbs2_swg_dn10, locals.var_isbs2_swg_dn11, locals.var_isbs2_swg_dn14,)
    }
};
        locals.var_isbs2_swg = assign96680_e149279;
        locals.var_isbs2_swg_dn0 = assign96680_e149279_d_n0;
        locals.var_isbs2_swg_dn2 = assign96680_e149279_d_n2;
        locals.var_isbs2_swg_dn4 = assign96680_e149279_d_n4;
        locals.var_isbs2_swg_dn5 = assign96680_e149279_d_n5;
        locals.var_isbs2_swg_dn6 = assign96680_e149279_d_n6;
        locals.var_isbs2_swg_dn7 = assign96680_e149279_d_n7;
        locals.var_isbs2_swg_dn8 = assign96680_e149279_d_n8;
        locals.var_isbs2_swg_dn9 = assign96680_e149279_d_n9;
        locals.var_isbs2_swg_dn10 = assign96680_e149279_d_n10;
        locals.var_isbs2_swg_dn11 = assign96680_e149279_d_n11;
        locals.var_isbs2_swg_dn14 = assign96680_e149279_d_n14;
        locals.var_isbs2_swg_rv = 0.0;

        let (assign96690_e149287, assign96690_e149287_d_n0, assign96690_e149287_d_n2, assign96690_e149287_d_n4, assign96690_e149287_d_n5, assign96690_e149287_d_n6, assign96690_e149287_d_n7, assign96690_e149287_d_n8, assign96690_e149287_d_n9, assign96690_e149287_d_n10, assign96690_e149287_d_n11, assign96690_e149287_d_n14,) = {
    if (locals.var_guard2237 != 0.0) {
        let assign96690_e149283: f64 = (locals.var_isbs_btm + locals.var_isbs_sws);
        let assign96690_e149285: f64 = (assign96690_e149283 + locals.var_isbs_swg);
        (assign96690_e149285, ((locals.var_isbs_btm_dn0 + locals.var_isbs_sws_dn0) + locals.var_isbs_swg_dn0), ((locals.var_isbs_btm_dn2 + locals.var_isbs_sws_dn2) + locals.var_isbs_swg_dn2), ((locals.var_isbs_btm_dn4 + locals.var_isbs_sws_dn4) + locals.var_isbs_swg_dn4), ((locals.var_isbs_btm_dn5 + locals.var_isbs_sws_dn5) + locals.var_isbs_swg_dn5), ((locals.var_isbs_btm_dn6 + locals.var_isbs_sws_dn6) + locals.var_isbs_swg_dn6), ((locals.var_isbs_btm_dn7 + locals.var_isbs_sws_dn7) + locals.var_isbs_swg_dn7), ((locals.var_isbs_btm_dn8 + locals.var_isbs_sws_dn8) + locals.var_isbs_swg_dn8), ((locals.var_isbs_btm_dn9 + locals.var_isbs_sws_dn9) + locals.var_isbs_swg_dn9), ((locals.var_isbs_btm_dn10 + locals.var_isbs_sws_dn10) + locals.var_isbs_swg_dn10), ((locals.var_isbs_btm_dn11 + locals.var_isbs_sws_dn11) + locals.var_isbs_swg_dn11), ((locals.var_isbs_btm_dn14 + locals.var_isbs_sws_dn14) + locals.var_isbs_swg_dn14),)
    } else {
        (locals.var_isbs, locals.var_isbs_dn0, locals.var_isbs_dn2, locals.var_isbs_dn4, locals.var_isbs_dn5, locals.var_isbs_dn6, locals.var_isbs_dn7, locals.var_isbs_dn8, locals.var_isbs_dn9, locals.var_isbs_dn10, locals.var_isbs_dn11, locals.var_isbs_dn14,)
    }
};
        locals.var_isbs = assign96690_e149287;
        locals.var_isbs_dn0 = assign96690_e149287_d_n0;
        locals.var_isbs_dn2 = assign96690_e149287_d_n2;
        locals.var_isbs_dn4 = assign96690_e149287_d_n4;
        locals.var_isbs_dn5 = assign96690_e149287_d_n5;
        locals.var_isbs_dn6 = assign96690_e149287_d_n6;
        locals.var_isbs_dn7 = assign96690_e149287_d_n7;
        locals.var_isbs_dn8 = assign96690_e149287_d_n8;
        locals.var_isbs_dn9 = assign96690_e149287_d_n9;
        locals.var_isbs_dn10 = assign96690_e149287_d_n10;
        locals.var_isbs_dn11 = assign96690_e149287_d_n11;
        locals.var_isbs_dn14 = assign96690_e149287_d_n14;
        locals.var_isbs_rv = 0.0;

        let assign96700_e149290: f64 = if locals.var_isbs > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2243 = assign96700_e149290;
        locals.var_guard2243_rv = 0.0;

        let (assign96710_e149298, assign96710_e149298_d_n0, assign96710_e149298_d_n2, assign96710_e149298_d_n4, assign96710_e149298_d_n5, assign96710_e149298_d_n6, assign96710_e149298_d_n7, assign96710_e149298_d_n8, assign96710_e149298_d_n9, assign96710_e149298_d_n10, assign96710_e149298_d_n11, assign96710_e149298_d_n14,) = {
    if ((locals.var_guard2237 != 0.0) && (locals.var_guard2243 != 0.0)) {
        let assign96710_e149296: f64 = (locals.var_isbs + 1e-25);
        (assign96710_e149296, locals.var_isbs_dn0, locals.var_isbs_dn2, locals.var_isbs_dn4, locals.var_isbs_dn5, locals.var_isbs_dn6, locals.var_isbs_dn7, locals.var_isbs_dn8, locals.var_isbs_dn9, locals.var_isbs_dn10, locals.var_isbs_dn11, locals.var_isbs_dn14,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign96710_e149298;
        locals.var_t3_dn0 = assign96710_e149298_d_n0;
        locals.var_t3_dn2 = assign96710_e149298_d_n2;
        locals.var_t3_dn4 = assign96710_e149298_d_n4;
        locals.var_t3_dn5 = assign96710_e149298_d_n5;
        locals.var_t3_dn6 = assign96710_e149298_d_n6;
        locals.var_t3_dn7 = assign96710_e149298_d_n7;
        locals.var_t3_dn8 = assign96710_e149298_d_n8;
        locals.var_t3_dn9 = assign96710_e149298_d_n9;
        locals.var_t3_dn10 = assign96710_e149298_d_n10;
        locals.var_t3_dn11 = assign96710_e149298_d_n11;
        locals.var_t3_dn14 = assign96710_e149298_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign96720_e149315, assign96720_e149315_d_n0, assign96720_e149315_d_n2, assign96720_e149315_d_n4, assign96720_e149315_d_n5, assign96720_e149315_d_n6, assign96720_e149315_d_n7, assign96720_e149315_d_n8, assign96720_e149315_d_n9, assign96720_e149315_d_n10, assign96720_e149315_d_n11, assign96720_e149315_d_n14,) = {
    if ((locals.var_guard2237 != 0.0) && (locals.var_guard2243 != 0.0)) {
        let assign96720_e149304: f64 = (locals.var_uc_njs / locals.var_beta);
        let assign96720_e149307: f64 = (locals.var_uc_vdiffjs * locals.var_t0);
        let assign96720_e149309: f64 = (assign96720_e149307 / locals.var_t3);
        let assign96720_e149311: f64 = (assign96720_e149309 + 1.0);
        let assign96720_e149312: f64 = (assign96720_e149311).ln();
        let assign96720_e149313: f64 = (assign96720_e149304 * assign96720_e149312);
        (assign96720_e149313, (((-((locals.var_uc_njs * locals.var_beta_dn0) / (locals.var_beta * locals.var_beta))) * assign96720_e149312) + (assign96720_e149304 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn0) * locals.var_t3) - (assign96720_e149307 * locals.var_t3_dn0)) / (locals.var_t3 * locals.var_t3)) / assign96720_e149311))), (((-((locals.var_uc_njs * locals.var_beta_dn2) / (locals.var_beta * locals.var_beta))) * assign96720_e149312) + (assign96720_e149304 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn2) * locals.var_t3) - (assign96720_e149307 * locals.var_t3_dn2)) / (locals.var_t3 * locals.var_t3)) / assign96720_e149311))), (((-((locals.var_uc_njs * locals.var_beta_dn4) / (locals.var_beta * locals.var_beta))) * assign96720_e149312) + (assign96720_e149304 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn4) * locals.var_t3) - (assign96720_e149307 * locals.var_t3_dn4)) / (locals.var_t3 * locals.var_t3)) / assign96720_e149311))), (((-((locals.var_uc_njs * locals.var_beta_dn5) / (locals.var_beta * locals.var_beta))) * assign96720_e149312) + (assign96720_e149304 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn5) * locals.var_t3) - (assign96720_e149307 * locals.var_t3_dn5)) / (locals.var_t3 * locals.var_t3)) / assign96720_e149311))), (((-((locals.var_uc_njs * locals.var_beta_dn6) / (locals.var_beta * locals.var_beta))) * assign96720_e149312) + (assign96720_e149304 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn6) * locals.var_t3) - (assign96720_e149307 * locals.var_t3_dn6)) / (locals.var_t3 * locals.var_t3)) / assign96720_e149311))), (((-((locals.var_uc_njs * locals.var_beta_dn7) / (locals.var_beta * locals.var_beta))) * assign96720_e149312) + (assign96720_e149304 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn7) * locals.var_t3) - (assign96720_e149307 * locals.var_t3_dn7)) / (locals.var_t3 * locals.var_t3)) / assign96720_e149311))), (((-((locals.var_uc_njs * locals.var_beta_dn8) / (locals.var_beta * locals.var_beta))) * assign96720_e149312) + (assign96720_e149304 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn8) * locals.var_t3) - (assign96720_e149307 * locals.var_t3_dn8)) / (locals.var_t3 * locals.var_t3)) / assign96720_e149311))), (((-((locals.var_uc_njs * locals.var_beta_dn9) / (locals.var_beta * locals.var_beta))) * assign96720_e149312) + (assign96720_e149304 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn9) * locals.var_t3) - (assign96720_e149307 * locals.var_t3_dn9)) / (locals.var_t3 * locals.var_t3)) / assign96720_e149311))), (((-((locals.var_uc_njs * locals.var_beta_dn10) / (locals.var_beta * locals.var_beta))) * assign96720_e149312) + (assign96720_e149304 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn10) * locals.var_t3) - (assign96720_e149307 * locals.var_t3_dn10)) / (locals.var_t3 * locals.var_t3)) / assign96720_e149311))), (((-((locals.var_uc_njs * locals.var_beta_dn11) / (locals.var_beta * locals.var_beta))) * assign96720_e149312) + (assign96720_e149304 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn11) * locals.var_t3) - (assign96720_e149307 * locals.var_t3_dn11)) / (locals.var_t3 * locals.var_t3)) / assign96720_e149311))), (((-((locals.var_uc_njs * locals.var_beta_dn14) / (locals.var_beta * locals.var_beta))) * assign96720_e149312) + (assign96720_e149304 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn14) * locals.var_t3) - (assign96720_e149307 * locals.var_t3_dn14)) / (locals.var_t3 * locals.var_t3)) / assign96720_e149311))),)
    } else {
        (locals.var_vbst, locals.var_vbst_dn0, locals.var_vbst_dn2, locals.var_vbst_dn4, locals.var_vbst_dn5, locals.var_vbst_dn6, locals.var_vbst_dn7, locals.var_vbst_dn8, locals.var_vbst_dn9, locals.var_vbst_dn10, locals.var_vbst_dn11, locals.var_vbst_dn14,)
    }
};
        locals.var_vbst = assign96720_e149315;
        locals.var_vbst_dn0 = assign96720_e149315_d_n0;
        locals.var_vbst_dn2 = assign96720_e149315_d_n2;
        locals.var_vbst_dn4 = assign96720_e149315_d_n4;
        locals.var_vbst_dn5 = assign96720_e149315_d_n5;
        locals.var_vbst_dn6 = assign96720_e149315_d_n6;
        locals.var_vbst_dn7 = assign96720_e149315_d_n7;
        locals.var_vbst_dn8 = assign96720_e149315_d_n8;
        locals.var_vbst_dn9 = assign96720_e149315_d_n9;
        locals.var_vbst_dn10 = assign96720_e149315_d_n10;
        locals.var_vbst_dn11 = assign96720_e149315_d_n11;
        locals.var_vbst_dn14 = assign96720_e149315_d_n14;
        locals.var_vbst_rv = 0.0;

        let (assign96730_e149326, assign96730_e149326_d_n0, assign96730_e149326_d_n2, assign96730_e149326_d_n4, assign96730_e149326_d_n5, assign96730_e149326_d_n6, assign96730_e149326_d_n7, assign96730_e149326_d_n8, assign96730_e149326_d_n9, assign96730_e149326_d_n10, assign96730_e149326_d_n11, assign96730_e149326_d_n14,) = {
    if ((locals.var_guard2237 != 0.0) && (locals.var_guard2243 != 0.0)) {
        let assign96730_e149321: f64 = (locals.var_tratio - 1.0);
        let assign96730_e149323: f64 = (assign96730_e149321 * p.p535);
        let assign96730_e149324: f64 = (assign96730_e149323).exp();
        (assign96730_e149324, (assign96730_e149324 * (locals.var_tratio_dn0 * p.p535)), (assign96730_e149324 * (locals.var_tratio_dn2 * p.p535)), (assign96730_e149324 * (locals.var_tratio_dn4 * p.p535)), (assign96730_e149324 * (locals.var_tratio_dn5 * p.p535)), (assign96730_e149324 * (locals.var_tratio_dn6 * p.p535)), (assign96730_e149324 * (locals.var_tratio_dn7 * p.p535)), (assign96730_e149324 * (locals.var_tratio_dn8 * p.p535)), (assign96730_e149324 * (locals.var_tratio_dn9 * p.p535)), (assign96730_e149324 * (locals.var_tratio_dn10 * p.p535)), (assign96730_e149324 * (locals.var_tratio_dn11 * p.p535)), (assign96730_e149324 * (locals.var_tratio_dn14 * p.p535)),)
    } else {
        (locals.var_exptemps, locals.var_exptemps_dn0, locals.var_exptemps_dn2, locals.var_exptemps_dn4, locals.var_exptemps_dn5, locals.var_exptemps_dn6, locals.var_exptemps_dn7, locals.var_exptemps_dn8, locals.var_exptemps_dn9, locals.var_exptemps_dn10, locals.var_exptemps_dn11, locals.var_exptemps_dn14,)
    }
};
        locals.var_exptemps = assign96730_e149326;
        locals.var_exptemps_dn0 = assign96730_e149326_d_n0;
        locals.var_exptemps_dn2 = assign96730_e149326_d_n2;
        locals.var_exptemps_dn4 = assign96730_e149326_d_n4;
        locals.var_exptemps_dn5 = assign96730_e149326_d_n5;
        locals.var_exptemps_dn6 = assign96730_e149326_d_n6;
        locals.var_exptemps_dn7 = assign96730_e149326_d_n7;
        locals.var_exptemps_dn8 = assign96730_e149326_d_n8;
        locals.var_exptemps_dn9 = assign96730_e149326_d_n9;
        locals.var_exptemps_dn10 = assign96730_e149326_d_n10;
        locals.var_exptemps_dn11 = assign96730_e149326_d_n11;
        locals.var_exptemps_dn14 = assign96730_e149326_d_n14;
        locals.var_exptemps_rv = 0.0;

        let (assign96740_e149336, assign96740_e149336_d_n0, assign96740_e149336_d_n2, assign96740_e149336_d_n4, assign96740_e149336_d_n5, assign96740_e149336_d_n6, assign96740_e149336_d_n7, assign96740_e149336_d_n8, assign96740_e149336_d_n9, assign96740_e149336_d_n10, assign96740_e149336_d_n11, assign96740_e149336_d_n14,) = {
    if ((locals.var_guard2237 != 0.0) && (locals.var_guard2243 != 0.0)) {
        let assign96740_e149333: f64 = (locals.var_uc_njs / locals.var_beta);
        let assign96740_e149334: f64 = (1.0 / assign96740_e149333);
        (assign96740_e149334, (-((-((locals.var_uc_njs * locals.var_beta_dn0) / (locals.var_beta * locals.var_beta))) / (assign96740_e149333 * assign96740_e149333))), (-((-((locals.var_uc_njs * locals.var_beta_dn2) / (locals.var_beta * locals.var_beta))) / (assign96740_e149333 * assign96740_e149333))), (-((-((locals.var_uc_njs * locals.var_beta_dn4) / (locals.var_beta * locals.var_beta))) / (assign96740_e149333 * assign96740_e149333))), (-((-((locals.var_uc_njs * locals.var_beta_dn5) / (locals.var_beta * locals.var_beta))) / (assign96740_e149333 * assign96740_e149333))), (-((-((locals.var_uc_njs * locals.var_beta_dn6) / (locals.var_beta * locals.var_beta))) / (assign96740_e149333 * assign96740_e149333))), (-((-((locals.var_uc_njs * locals.var_beta_dn7) / (locals.var_beta * locals.var_beta))) / (assign96740_e149333 * assign96740_e149333))), (-((-((locals.var_uc_njs * locals.var_beta_dn8) / (locals.var_beta * locals.var_beta))) / (assign96740_e149333 * assign96740_e149333))), (-((-((locals.var_uc_njs * locals.var_beta_dn9) / (locals.var_beta * locals.var_beta))) / (assign96740_e149333 * assign96740_e149333))), (-((-((locals.var_uc_njs * locals.var_beta_dn10) / (locals.var_beta * locals.var_beta))) / (assign96740_e149333 * assign96740_e149333))), (-((-((locals.var_uc_njs * locals.var_beta_dn11) / (locals.var_beta * locals.var_beta))) / (assign96740_e149333 * assign96740_e149333))), (-((-((locals.var_uc_njs * locals.var_beta_dn14) / (locals.var_beta * locals.var_beta))) / (assign96740_e149333 * assign96740_e149333))),)
    } else {
        (locals.var_jd_nvtm_invs, locals.var_jd_nvtm_invs_dn0, locals.var_jd_nvtm_invs_dn2, locals.var_jd_nvtm_invs_dn4, locals.var_jd_nvtm_invs_dn5, locals.var_jd_nvtm_invs_dn6, locals.var_jd_nvtm_invs_dn7, locals.var_jd_nvtm_invs_dn8, locals.var_jd_nvtm_invs_dn9, locals.var_jd_nvtm_invs_dn10, locals.var_jd_nvtm_invs_dn11, locals.var_jd_nvtm_invs_dn14,)
    }
};
        locals.var_jd_nvtm_invs = assign96740_e149336;
        locals.var_jd_nvtm_invs_dn0 = assign96740_e149336_d_n0;
        locals.var_jd_nvtm_invs_dn2 = assign96740_e149336_d_n2;
        locals.var_jd_nvtm_invs_dn4 = assign96740_e149336_d_n4;
        locals.var_jd_nvtm_invs_dn5 = assign96740_e149336_d_n5;
        locals.var_jd_nvtm_invs_dn6 = assign96740_e149336_d_n6;
        locals.var_jd_nvtm_invs_dn7 = assign96740_e149336_d_n7;
        locals.var_jd_nvtm_invs_dn8 = assign96740_e149336_d_n8;
        locals.var_jd_nvtm_invs_dn9 = assign96740_e149336_d_n9;
        locals.var_jd_nvtm_invs_dn10 = assign96740_e149336_d_n10;
        locals.var_jd_nvtm_invs_dn11 = assign96740_e149336_d_n11;
        locals.var_jd_nvtm_invs_dn14 = assign96740_e149336_d_n14;
        locals.var_jd_nvtm_invs_rv = 0.0;

        let (assign96750_e149345, assign96750_e149345_d_n0, assign96750_e149345_d_n2, assign96750_e149345_d_n4, assign96750_e149345_d_n5, assign96750_e149345_d_n6, assign96750_e149345_d_n7, assign96750_e149345_d_n8, assign96750_e149345_d_n9, assign96750_e149345_d_n10, assign96750_e149345_d_n11, assign96750_e149345_d_n14,) = {
    if ((locals.var_guard2237 != 0.0) && (locals.var_guard2243 != 0.0)) {
        let assign96750_e149342: f64 = (locals.var_vbst * locals.var_jd_nvtm_invs);
        let assign96750_e149343: f64 = (assign96750_e149342).exp();
        (assign96750_e149343, (assign96750_e149343 * ((locals.var_vbst_dn0 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn0))), (assign96750_e149343 * ((locals.var_vbst_dn2 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn2))), (assign96750_e149343 * ((locals.var_vbst_dn4 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn4))), (assign96750_e149343 * ((locals.var_vbst_dn5 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn5))), (assign96750_e149343 * ((locals.var_vbst_dn6 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn6))), (assign96750_e149343 * ((locals.var_vbst_dn7 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn7))), (assign96750_e149343 * ((locals.var_vbst_dn8 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn8))), (assign96750_e149343 * ((locals.var_vbst_dn9 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn9))), (assign96750_e149343 * ((locals.var_vbst_dn10 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn10))), (assign96750_e149343 * ((locals.var_vbst_dn11 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn11))), (assign96750_e149343 * ((locals.var_vbst_dn14 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn14))),)
    } else {
        (locals.var_jd_expcs, locals.var_jd_expcs_dn0, locals.var_jd_expcs_dn2, locals.var_jd_expcs_dn4, locals.var_jd_expcs_dn5, locals.var_jd_expcs_dn6, locals.var_jd_expcs_dn7, locals.var_jd_expcs_dn8, locals.var_jd_expcs_dn9, locals.var_jd_expcs_dn10, locals.var_jd_expcs_dn11, locals.var_jd_expcs_dn14,)
    }
};
        locals.var_jd_expcs = assign96750_e149345;
        locals.var_jd_expcs_dn0 = assign96750_e149345_d_n0;
        locals.var_jd_expcs_dn2 = assign96750_e149345_d_n2;
        locals.var_jd_expcs_dn4 = assign96750_e149345_d_n4;
        locals.var_jd_expcs_dn5 = assign96750_e149345_d_n5;
        locals.var_jd_expcs_dn6 = assign96750_e149345_d_n6;
        locals.var_jd_expcs_dn7 = assign96750_e149345_d_n7;
        locals.var_jd_expcs_dn8 = assign96750_e149345_d_n8;
        locals.var_jd_expcs_dn9 = assign96750_e149345_d_n9;
        locals.var_jd_expcs_dn10 = assign96750_e149345_d_n10;
        locals.var_jd_expcs_dn11 = assign96750_e149345_d_n11;
        locals.var_jd_expcs_dn14 = assign96750_e149345_d_n14;
        locals.var_jd_expcs_rv = 0.0;

        let (assign96760_e149357, assign96760_e149357_d_n0, assign96760_e149357_d_n2, assign96760_e149357_d_n4, assign96760_e149357_d_n5, assign96760_e149357_d_n6, assign96760_e149357_d_n7, assign96760_e149357_d_n8, assign96760_e149357_d_n9, assign96760_e149357_d_n10, assign96760_e149357_d_n11, assign96760_e149357_d_n14,) = {
    if (locals.var_guard2237 != 0.0) {
        let assign96760_e149349: f64 = (p.p500 * p.p13);
        let assign96760_e149353: f64 = (p.p481 * locals.var_tdiff);
        let assign96760_e149354: f64 = (1.0 + assign96760_e149353);
        let assign96760_e149355: f64 = (assign96760_e149349 * assign96760_e149354);
        (assign96760_e149355, (assign96760_e149349 * (p.p481 * locals.var_tdiff_dn0)), (assign96760_e149349 * (p.p481 * locals.var_tdiff_dn2)), (assign96760_e149349 * (p.p481 * locals.var_tdiff_dn4)), (assign96760_e149349 * (p.p481 * locals.var_tdiff_dn5)), (assign96760_e149349 * (p.p481 * locals.var_tdiff_dn6)), (assign96760_e149349 * (p.p481 * locals.var_tdiff_dn7)), (assign96760_e149349 * (p.p481 * locals.var_tdiff_dn8)), (assign96760_e149349 * (p.p481 * locals.var_tdiff_dn9)), (assign96760_e149349 * (p.p481 * locals.var_tdiff_dn10)), (assign96760_e149349 * (p.p481 * locals.var_tdiff_dn11)), (assign96760_e149349 * (p.p481 * locals.var_tdiff_dn14)),)
    } else {
        (locals.var_czbd, locals.var_czbd_dn0, locals.var_czbd_dn2, locals.var_czbd_dn4, locals.var_czbd_dn5, locals.var_czbd_dn6, locals.var_czbd_dn7, locals.var_czbd_dn8, locals.var_czbd_dn9, locals.var_czbd_dn10, locals.var_czbd_dn11, locals.var_czbd_dn14,)
    }
};
        locals.var_czbd = assign96760_e149357;
        locals.var_czbd_dn0 = assign96760_e149357_d_n0;
        locals.var_czbd_dn2 = assign96760_e149357_d_n2;
        locals.var_czbd_dn4 = assign96760_e149357_d_n4;
        locals.var_czbd_dn5 = assign96760_e149357_d_n5;
        locals.var_czbd_dn6 = assign96760_e149357_d_n6;
        locals.var_czbd_dn7 = assign96760_e149357_d_n7;
        locals.var_czbd_dn8 = assign96760_e149357_d_n8;
        locals.var_czbd_dn9 = assign96760_e149357_d_n9;
        locals.var_czbd_dn10 = assign96760_e149357_d_n10;
        locals.var_czbd_dn11 = assign96760_e149357_d_n11;
        locals.var_czbd_dn14 = assign96760_e149357_d_n14;
        locals.var_czbd_rv = 0.0;

        let assign96770_e149360: f64 = if p.p15 > locals.var_weff_nf { 1.0 } else { 0.0 };
        locals.var_guard2244 = assign96770_e149360;
        locals.var_guard2244_rv = 0.0;

        let (assign96780_e149376, assign96780_e149376_d_n0, assign96780_e149376_d_n2, assign96780_e149376_d_n4, assign96780_e149376_d_n5, assign96780_e149376_d_n6, assign96780_e149376_d_n7, assign96780_e149376_d_n8, assign96780_e149376_d_n9, assign96780_e149376_d_n10, assign96780_e149376_d_n11, assign96780_e149376_d_n14,) = {
    if ((locals.var_guard2237 != 0.0) && (locals.var_guard2244 != 0.0)) {
        let assign96780_e149367: f64 = (p.p15 - locals.var_weff_nf);
        let assign96780_e149368: f64 = (p.p501 * assign96780_e149367);
        let assign96780_e149372: f64 = (p.p483 * locals.var_tdiff);
        let assign96780_e149373: f64 = (1.0 + assign96780_e149372);
        let assign96780_e149374: f64 = (assign96780_e149368 * assign96780_e149373);
        (assign96780_e149374, (assign96780_e149368 * (p.p483 * locals.var_tdiff_dn0)), (assign96780_e149368 * (p.p483 * locals.var_tdiff_dn2)), (assign96780_e149368 * (p.p483 * locals.var_tdiff_dn4)), (assign96780_e149368 * (p.p483 * locals.var_tdiff_dn5)), (assign96780_e149368 * (p.p483 * locals.var_tdiff_dn6)), (assign96780_e149368 * (p.p483 * locals.var_tdiff_dn7)), (assign96780_e149368 * (p.p483 * locals.var_tdiff_dn8)), (assign96780_e149368 * (p.p483 * locals.var_tdiff_dn9)), (assign96780_e149368 * (p.p483 * locals.var_tdiff_dn10)), (assign96780_e149368 * (p.p483 * locals.var_tdiff_dn11)), (assign96780_e149368 * (p.p483 * locals.var_tdiff_dn14)),)
    } else {
        (locals.var_czbdsw, locals.var_czbdsw_dn0, locals.var_czbdsw_dn2, locals.var_czbdsw_dn4, locals.var_czbdsw_dn5, locals.var_czbdsw_dn6, locals.var_czbdsw_dn7, locals.var_czbdsw_dn8, locals.var_czbdsw_dn9, locals.var_czbdsw_dn10, locals.var_czbdsw_dn11, locals.var_czbdsw_dn14,)
    }
};
        locals.var_czbdsw = assign96780_e149376;
        locals.var_czbdsw_dn0 = assign96780_e149376_d_n0;
        locals.var_czbdsw_dn2 = assign96780_e149376_d_n2;
        locals.var_czbdsw_dn4 = assign96780_e149376_d_n4;
        locals.var_czbdsw_dn5 = assign96780_e149376_d_n5;
        locals.var_czbdsw_dn6 = assign96780_e149376_d_n6;
        locals.var_czbdsw_dn7 = assign96780_e149376_d_n7;
        locals.var_czbdsw_dn8 = assign96780_e149376_d_n8;
        locals.var_czbdsw_dn9 = assign96780_e149376_d_n9;
        locals.var_czbdsw_dn10 = assign96780_e149376_d_n10;
        locals.var_czbdsw_dn11 = assign96780_e149376_d_n11;
        locals.var_czbdsw_dn14 = assign96780_e149376_d_n14;
        locals.var_czbdsw_rv = 0.0;

        let (assign96790_e149390, assign96790_e149390_d_n0, assign96790_e149390_d_n2, assign96790_e149390_d_n4, assign96790_e149390_d_n5, assign96790_e149390_d_n6, assign96790_e149390_d_n7, assign96790_e149390_d_n8, assign96790_e149390_d_n9, assign96790_e149390_d_n10, assign96790_e149390_d_n11, assign96790_e149390_d_n14,) = {
    if ((locals.var_guard2237 != 0.0) && (locals.var_guard2244 != 0.0)) {
        let assign96790_e149382: f64 = (p.p502 * locals.var_weff_nf);
        let assign96790_e149386: f64 = (p.p485 * locals.var_tdiff);
        let assign96790_e149387: f64 = (1.0 + assign96790_e149386);
        let assign96790_e149388: f64 = (assign96790_e149382 * assign96790_e149387);
        (assign96790_e149388, (assign96790_e149382 * (p.p485 * locals.var_tdiff_dn0)), (assign96790_e149382 * (p.p485 * locals.var_tdiff_dn2)), (assign96790_e149382 * (p.p485 * locals.var_tdiff_dn4)), (assign96790_e149382 * (p.p485 * locals.var_tdiff_dn5)), (assign96790_e149382 * (p.p485 * locals.var_tdiff_dn6)), (assign96790_e149382 * (p.p485 * locals.var_tdiff_dn7)), (assign96790_e149382 * (p.p485 * locals.var_tdiff_dn8)), (assign96790_e149382 * (p.p485 * locals.var_tdiff_dn9)), (assign96790_e149382 * (p.p485 * locals.var_tdiff_dn10)), (assign96790_e149382 * (p.p485 * locals.var_tdiff_dn11)), (assign96790_e149382 * (p.p485 * locals.var_tdiff_dn14)),)
    } else {
        (locals.var_czbdswg, locals.var_czbdswg_dn0, locals.var_czbdswg_dn2, locals.var_czbdswg_dn4, locals.var_czbdswg_dn5, locals.var_czbdswg_dn6, locals.var_czbdswg_dn7, locals.var_czbdswg_dn8, locals.var_czbdswg_dn9, locals.var_czbdswg_dn10, locals.var_czbdswg_dn11, locals.var_czbdswg_dn14,)
    }
};
        locals.var_czbdswg = assign96790_e149390;
        locals.var_czbdswg_dn0 = assign96790_e149390_d_n0;
        locals.var_czbdswg_dn2 = assign96790_e149390_d_n2;
        locals.var_czbdswg_dn4 = assign96790_e149390_d_n4;
        locals.var_czbdswg_dn5 = assign96790_e149390_d_n5;
        locals.var_czbdswg_dn6 = assign96790_e149390_d_n6;
        locals.var_czbdswg_dn7 = assign96790_e149390_d_n7;
        locals.var_czbdswg_dn8 = assign96790_e149390_d_n8;
        locals.var_czbdswg_dn9 = assign96790_e149390_d_n9;
        locals.var_czbdswg_dn10 = assign96790_e149390_d_n10;
        locals.var_czbdswg_dn11 = assign96790_e149390_d_n11;
        locals.var_czbdswg_dn14 = assign96790_e149390_d_n14;
        locals.var_czbdswg_rv = 0.0;

        let (assign96800_e149397, assign96800_e149397_d_n0, assign96800_e149397_d_n2, assign96800_e149397_d_n4, assign96800_e149397_d_n5, assign96800_e149397_d_n6, assign96800_e149397_d_n7, assign96800_e149397_d_n8, assign96800_e149397_d_n9, assign96800_e149397_d_n10, assign96800_e149397_d_n11, assign96800_e149397_d_n14,) = {
    if ((locals.var_guard2237 != 0.0) && (locals.var_guard2244 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_czbdsw, locals.var_czbdsw_dn0, locals.var_czbdsw_dn2, locals.var_czbdsw_dn4, locals.var_czbdsw_dn5, locals.var_czbdsw_dn6, locals.var_czbdsw_dn7, locals.var_czbdsw_dn8, locals.var_czbdsw_dn9, locals.var_czbdsw_dn10, locals.var_czbdsw_dn11, locals.var_czbdsw_dn14,)
    }
};
        locals.var_czbdsw = assign96800_e149397;
        locals.var_czbdsw_dn0 = assign96800_e149397_d_n0;
        locals.var_czbdsw_dn2 = assign96800_e149397_d_n2;
        locals.var_czbdsw_dn4 = assign96800_e149397_d_n4;
        locals.var_czbdsw_dn5 = assign96800_e149397_d_n5;
        locals.var_czbdsw_dn6 = assign96800_e149397_d_n6;
        locals.var_czbdsw_dn7 = assign96800_e149397_d_n7;
        locals.var_czbdsw_dn8 = assign96800_e149397_d_n8;
        locals.var_czbdsw_dn9 = assign96800_e149397_d_n9;
        locals.var_czbdsw_dn10 = assign96800_e149397_d_n10;
        locals.var_czbdsw_dn11 = assign96800_e149397_d_n11;
        locals.var_czbdsw_dn14 = assign96800_e149397_d_n14;
        locals.var_czbdsw_rv = 0.0;

        let (assign96810_e149412, assign96810_e149412_d_n0, assign96810_e149412_d_n2, assign96810_e149412_d_n4, assign96810_e149412_d_n5, assign96810_e149412_d_n6, assign96810_e149412_d_n7, assign96810_e149412_d_n8, assign96810_e149412_d_n9, assign96810_e149412_d_n10, assign96810_e149412_d_n11, assign96810_e149412_d_n14,) = {
    if ((locals.var_guard2237 != 0.0) && (locals.var_guard2244 == 0.0)) {
        let assign96810_e149404: f64 = (p.p502 * p.p15);
        let assign96810_e149408: f64 = (p.p485 * locals.var_tdiff);
        let assign96810_e149409: f64 = (1.0 + assign96810_e149408);
        let assign96810_e149410: f64 = (assign96810_e149404 * assign96810_e149409);
        (assign96810_e149410, (assign96810_e149404 * (p.p485 * locals.var_tdiff_dn0)), (assign96810_e149404 * (p.p485 * locals.var_tdiff_dn2)), (assign96810_e149404 * (p.p485 * locals.var_tdiff_dn4)), (assign96810_e149404 * (p.p485 * locals.var_tdiff_dn5)), (assign96810_e149404 * (p.p485 * locals.var_tdiff_dn6)), (assign96810_e149404 * (p.p485 * locals.var_tdiff_dn7)), (assign96810_e149404 * (p.p485 * locals.var_tdiff_dn8)), (assign96810_e149404 * (p.p485 * locals.var_tdiff_dn9)), (assign96810_e149404 * (p.p485 * locals.var_tdiff_dn10)), (assign96810_e149404 * (p.p485 * locals.var_tdiff_dn11)), (assign96810_e149404 * (p.p485 * locals.var_tdiff_dn14)),)
    } else {
        (locals.var_czbdswg, locals.var_czbdswg_dn0, locals.var_czbdswg_dn2, locals.var_czbdswg_dn4, locals.var_czbdswg_dn5, locals.var_czbdswg_dn6, locals.var_czbdswg_dn7, locals.var_czbdswg_dn8, locals.var_czbdswg_dn9, locals.var_czbdswg_dn10, locals.var_czbdswg_dn11, locals.var_czbdswg_dn14,)
    }
};
        locals.var_czbdswg = assign96810_e149412;
        locals.var_czbdswg_dn0 = assign96810_e149412_d_n0;
        locals.var_czbdswg_dn2 = assign96810_e149412_d_n2;
        locals.var_czbdswg_dn4 = assign96810_e149412_d_n4;
        locals.var_czbdswg_dn5 = assign96810_e149412_d_n5;
        locals.var_czbdswg_dn6 = assign96810_e149412_d_n6;
        locals.var_czbdswg_dn7 = assign96810_e149412_d_n7;
        locals.var_czbdswg_dn8 = assign96810_e149412_d_n8;
        locals.var_czbdswg_dn9 = assign96810_e149412_d_n9;
        locals.var_czbdswg_dn10 = assign96810_e149412_d_n10;
        locals.var_czbdswg_dn11 = assign96810_e149412_d_n11;
        locals.var_czbdswg_dn14 = assign96810_e149412_d_n14;
        locals.var_czbdswg_rv = 0.0;

        let assign96820_e149415: f64 = if locals.var_czbd < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2245 = assign96820_e149415;
        locals.var_guard2245_rv = 0.0;

        let (assign96830_e149421, assign96830_e149421_d_n0, assign96830_e149421_d_n2, assign96830_e149421_d_n4, assign96830_e149421_d_n5, assign96830_e149421_d_n6, assign96830_e149421_d_n7, assign96830_e149421_d_n8, assign96830_e149421_d_n9, assign96830_e149421_d_n10, assign96830_e149421_d_n11, assign96830_e149421_d_n14,) = {
    if ((locals.var_guard2237 != 0.0) && (locals.var_guard2245 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_czbd, locals.var_czbd_dn0, locals.var_czbd_dn2, locals.var_czbd_dn4, locals.var_czbd_dn5, locals.var_czbd_dn6, locals.var_czbd_dn7, locals.var_czbd_dn8, locals.var_czbd_dn9, locals.var_czbd_dn10, locals.var_czbd_dn11, locals.var_czbd_dn14,)
    }
};
        locals.var_czbd = assign96830_e149421;
        locals.var_czbd_dn0 = assign96830_e149421_d_n0;
        locals.var_czbd_dn2 = assign96830_e149421_d_n2;
        locals.var_czbd_dn4 = assign96830_e149421_d_n4;
        locals.var_czbd_dn5 = assign96830_e149421_d_n5;
        locals.var_czbd_dn6 = assign96830_e149421_d_n6;
        locals.var_czbd_dn7 = assign96830_e149421_d_n7;
        locals.var_czbd_dn8 = assign96830_e149421_d_n8;
        locals.var_czbd_dn9 = assign96830_e149421_d_n9;
        locals.var_czbd_dn10 = assign96830_e149421_d_n10;
        locals.var_czbd_dn11 = assign96830_e149421_d_n11;
        locals.var_czbd_dn14 = assign96830_e149421_d_n14;
        locals.var_czbd_rv = 0.0;

        let assign96840_e149424: f64 = if locals.var_czbdsw < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2246 = assign96840_e149424;
        locals.var_guard2246_rv = 0.0;

        let (assign96850_e149430, assign96850_e149430_d_n0, assign96850_e149430_d_n2, assign96850_e149430_d_n4, assign96850_e149430_d_n5, assign96850_e149430_d_n6, assign96850_e149430_d_n7, assign96850_e149430_d_n8, assign96850_e149430_d_n9, assign96850_e149430_d_n10, assign96850_e149430_d_n11, assign96850_e149430_d_n14,) = {
    if ((locals.var_guard2237 != 0.0) && (locals.var_guard2246 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_czbdsw, locals.var_czbdsw_dn0, locals.var_czbdsw_dn2, locals.var_czbdsw_dn4, locals.var_czbdsw_dn5, locals.var_czbdsw_dn6, locals.var_czbdsw_dn7, locals.var_czbdsw_dn8, locals.var_czbdsw_dn9, locals.var_czbdsw_dn10, locals.var_czbdsw_dn11, locals.var_czbdsw_dn14,)
    }
};
        locals.var_czbdsw = assign96850_e149430;
        locals.var_czbdsw_dn0 = assign96850_e149430_d_n0;
        locals.var_czbdsw_dn2 = assign96850_e149430_d_n2;
        locals.var_czbdsw_dn4 = assign96850_e149430_d_n4;
        locals.var_czbdsw_dn5 = assign96850_e149430_d_n5;
        locals.var_czbdsw_dn6 = assign96850_e149430_d_n6;
        locals.var_czbdsw_dn7 = assign96850_e149430_d_n7;
        locals.var_czbdsw_dn8 = assign96850_e149430_d_n8;
        locals.var_czbdsw_dn9 = assign96850_e149430_d_n9;
        locals.var_czbdsw_dn10 = assign96850_e149430_d_n10;
        locals.var_czbdsw_dn11 = assign96850_e149430_d_n11;
        locals.var_czbdsw_dn14 = assign96850_e149430_d_n14;
        locals.var_czbdsw_rv = 0.0;

        let assign96860_e149433: f64 = if locals.var_czbdswg < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2247 = assign96860_e149433;
        locals.var_guard2247_rv = 0.0;

        let (assign96870_e149439, assign96870_e149439_d_n0, assign96870_e149439_d_n2, assign96870_e149439_d_n4, assign96870_e149439_d_n5, assign96870_e149439_d_n6, assign96870_e149439_d_n7, assign96870_e149439_d_n8, assign96870_e149439_d_n9, assign96870_e149439_d_n10, assign96870_e149439_d_n11, assign96870_e149439_d_n14,) = {
    if ((locals.var_guard2237 != 0.0) && (locals.var_guard2247 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_czbdswg, locals.var_czbdswg_dn0, locals.var_czbdswg_dn2, locals.var_czbdswg_dn4, locals.var_czbdswg_dn5, locals.var_czbdswg_dn6, locals.var_czbdswg_dn7, locals.var_czbdswg_dn8, locals.var_czbdswg_dn9, locals.var_czbdswg_dn10, locals.var_czbdswg_dn11, locals.var_czbdswg_dn14,)
    }
};
        locals.var_czbdswg = assign96870_e149439;
        locals.var_czbdswg_dn0 = assign96870_e149439_d_n0;
        locals.var_czbdswg_dn2 = assign96870_e149439_d_n2;
        locals.var_czbdswg_dn4 = assign96870_e149439_d_n4;
        locals.var_czbdswg_dn5 = assign96870_e149439_d_n5;
        locals.var_czbdswg_dn6 = assign96870_e149439_d_n6;
        locals.var_czbdswg_dn7 = assign96870_e149439_d_n7;
        locals.var_czbdswg_dn8 = assign96870_e149439_d_n8;
        locals.var_czbdswg_dn9 = assign96870_e149439_d_n9;
        locals.var_czbdswg_dn10 = assign96870_e149439_d_n10;
        locals.var_czbdswg_dn11 = assign96870_e149439_d_n11;
        locals.var_czbdswg_dn14 = assign96870_e149439_d_n14;
        locals.var_czbdswg_rv = 0.0;

        let (assign96880_e149447, assign96880_e149447_d_n0, assign96880_e149447_d_n2, assign96880_e149447_d_n4, assign96880_e149447_d_n5, assign96880_e149447_d_n6, assign96880_e149447_d_n7, assign96880_e149447_d_n8, assign96880_e149447_d_n9, assign96880_e149447_d_n10, assign96880_e149447_d_n11, assign96880_e149447_d_n14,) = {
    if (locals.var_guard2237 != 0.0) {
        let assign96880_e149444: f64 = (p.p487 * locals.var_tdiff);
        let assign96880_e149445: f64 = (p.p506 - assign96880_e149444);
        (assign96880_e149445, (-(p.p487 * locals.var_tdiff_dn0)), (-(p.p487 * locals.var_tdiff_dn2)), (-(p.p487 * locals.var_tdiff_dn4)), (-(p.p487 * locals.var_tdiff_dn5)), (-(p.p487 * locals.var_tdiff_dn6)), (-(p.p487 * locals.var_tdiff_dn7)), (-(p.p487 * locals.var_tdiff_dn8)), (-(p.p487 * locals.var_tdiff_dn9)), (-(p.p487 * locals.var_tdiff_dn10)), (-(p.p487 * locals.var_tdiff_dn11)), (-(p.p487 * locals.var_tdiff_dn14)),)
    } else {
        (locals.var_pzbd, locals.var_pzbd_dn0, locals.var_pzbd_dn2, locals.var_pzbd_dn4, locals.var_pzbd_dn5, locals.var_pzbd_dn6, locals.var_pzbd_dn7, locals.var_pzbd_dn8, locals.var_pzbd_dn9, locals.var_pzbd_dn10, locals.var_pzbd_dn11, locals.var_pzbd_dn14,)
    }
};
        locals.var_pzbd = assign96880_e149447;
        locals.var_pzbd_dn0 = assign96880_e149447_d_n0;
        locals.var_pzbd_dn2 = assign96880_e149447_d_n2;
        locals.var_pzbd_dn4 = assign96880_e149447_d_n4;
        locals.var_pzbd_dn5 = assign96880_e149447_d_n5;
        locals.var_pzbd_dn6 = assign96880_e149447_d_n6;
        locals.var_pzbd_dn7 = assign96880_e149447_d_n7;
        locals.var_pzbd_dn8 = assign96880_e149447_d_n8;
        locals.var_pzbd_dn9 = assign96880_e149447_d_n9;
        locals.var_pzbd_dn10 = assign96880_e149447_d_n10;
        locals.var_pzbd_dn11 = assign96880_e149447_d_n11;
        locals.var_pzbd_dn14 = assign96880_e149447_d_n14;
        locals.var_pzbd_rv = 0.0;

        let (assign96890_e149455, assign96890_e149455_d_n0, assign96890_e149455_d_n2, assign96890_e149455_d_n4, assign96890_e149455_d_n5, assign96890_e149455_d_n6, assign96890_e149455_d_n7, assign96890_e149455_d_n8, assign96890_e149455_d_n9, assign96890_e149455_d_n10, assign96890_e149455_d_n11, assign96890_e149455_d_n14,) = {
    if (locals.var_guard2237 != 0.0) {
        let assign96890_e149452: f64 = (p.p489 * locals.var_tdiff);
        let assign96890_e149453: f64 = (p.p507 - assign96890_e149452);
        (assign96890_e149453, (-(p.p489 * locals.var_tdiff_dn0)), (-(p.p489 * locals.var_tdiff_dn2)), (-(p.p489 * locals.var_tdiff_dn4)), (-(p.p489 * locals.var_tdiff_dn5)), (-(p.p489 * locals.var_tdiff_dn6)), (-(p.p489 * locals.var_tdiff_dn7)), (-(p.p489 * locals.var_tdiff_dn8)), (-(p.p489 * locals.var_tdiff_dn9)), (-(p.p489 * locals.var_tdiff_dn10)), (-(p.p489 * locals.var_tdiff_dn11)), (-(p.p489 * locals.var_tdiff_dn14)),)
    } else {
        (locals.var_pzbdsw, locals.var_pzbdsw_dn0, locals.var_pzbdsw_dn2, locals.var_pzbdsw_dn4, locals.var_pzbdsw_dn5, locals.var_pzbdsw_dn6, locals.var_pzbdsw_dn7, locals.var_pzbdsw_dn8, locals.var_pzbdsw_dn9, locals.var_pzbdsw_dn10, locals.var_pzbdsw_dn11, locals.var_pzbdsw_dn14,)
    }
};
        locals.var_pzbdsw = assign96890_e149455;
        locals.var_pzbdsw_dn0 = assign96890_e149455_d_n0;
        locals.var_pzbdsw_dn2 = assign96890_e149455_d_n2;
        locals.var_pzbdsw_dn4 = assign96890_e149455_d_n4;
        locals.var_pzbdsw_dn5 = assign96890_e149455_d_n5;
        locals.var_pzbdsw_dn6 = assign96890_e149455_d_n6;
        locals.var_pzbdsw_dn7 = assign96890_e149455_d_n7;
        locals.var_pzbdsw_dn8 = assign96890_e149455_d_n8;
        locals.var_pzbdsw_dn9 = assign96890_e149455_d_n9;
        locals.var_pzbdsw_dn10 = assign96890_e149455_d_n10;
        locals.var_pzbdsw_dn11 = assign96890_e149455_d_n11;
        locals.var_pzbdsw_dn14 = assign96890_e149455_d_n14;
        locals.var_pzbdsw_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_376(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let ctx_temp = ctx.temperature();
        let (assign96900_e149463, assign96900_e149463_d_n0, assign96900_e149463_d_n2, assign96900_e149463_d_n4, assign96900_e149463_d_n5, assign96900_e149463_d_n6, assign96900_e149463_d_n7, assign96900_e149463_d_n8, assign96900_e149463_d_n9, assign96900_e149463_d_n10, assign96900_e149463_d_n11, assign96900_e149463_d_n14,) = {
    if (locals.var_guard2237 != 0.0) {
        let assign96900_e149460: f64 = (p.p491 * locals.var_tdiff);
        let assign96900_e149461: f64 = (p.p508 - assign96900_e149460);
        (assign96900_e149461, (-(p.p491 * locals.var_tdiff_dn0)), (-(p.p491 * locals.var_tdiff_dn2)), (-(p.p491 * locals.var_tdiff_dn4)), (-(p.p491 * locals.var_tdiff_dn5)), (-(p.p491 * locals.var_tdiff_dn6)), (-(p.p491 * locals.var_tdiff_dn7)), (-(p.p491 * locals.var_tdiff_dn8)), (-(p.p491 * locals.var_tdiff_dn9)), (-(p.p491 * locals.var_tdiff_dn10)), (-(p.p491 * locals.var_tdiff_dn11)), (-(p.p491 * locals.var_tdiff_dn14)),)
    } else {
        (locals.var_pzbdswg, locals.var_pzbdswg_dn0, locals.var_pzbdswg_dn2, locals.var_pzbdswg_dn4, locals.var_pzbdswg_dn5, locals.var_pzbdswg_dn6, locals.var_pzbdswg_dn7, locals.var_pzbdswg_dn8, locals.var_pzbdswg_dn9, locals.var_pzbdswg_dn10, locals.var_pzbdswg_dn11, locals.var_pzbdswg_dn14,)
    }
};
        locals.var_pzbdswg = assign96900_e149463;
        locals.var_pzbdswg_dn0 = assign96900_e149463_d_n0;
        locals.var_pzbdswg_dn2 = assign96900_e149463_d_n2;
        locals.var_pzbdswg_dn4 = assign96900_e149463_d_n4;
        locals.var_pzbdswg_dn5 = assign96900_e149463_d_n5;
        locals.var_pzbdswg_dn6 = assign96900_e149463_d_n6;
        locals.var_pzbdswg_dn7 = assign96900_e149463_d_n7;
        locals.var_pzbdswg_dn8 = assign96900_e149463_d_n8;
        locals.var_pzbdswg_dn9 = assign96900_e149463_d_n9;
        locals.var_pzbdswg_dn10 = assign96900_e149463_d_n10;
        locals.var_pzbdswg_dn11 = assign96900_e149463_d_n11;
        locals.var_pzbdswg_dn14 = assign96900_e149463_d_n14;
        locals.var_pzbdswg_rv = 0.0;

        let assign96910_e149470: f64 = if ((locals.var_pzbd < 0.01) && (p.p13 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2248 = assign96910_e149470;
        locals.var_guard2248_rv = 0.0;

        let (assign96920_e149476, assign96920_e149476_d_n0, assign96920_e149476_d_n2, assign96920_e149476_d_n4, assign96920_e149476_d_n5, assign96920_e149476_d_n6, assign96920_e149476_d_n7, assign96920_e149476_d_n8, assign96920_e149476_d_n9, assign96920_e149476_d_n10, assign96920_e149476_d_n11, assign96920_e149476_d_n14,) = {
    if ((locals.var_guard2237 != 0.0) && (locals.var_guard2248 != 0.0)) {
        (0.01, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pzbd, locals.var_pzbd_dn0, locals.var_pzbd_dn2, locals.var_pzbd_dn4, locals.var_pzbd_dn5, locals.var_pzbd_dn6, locals.var_pzbd_dn7, locals.var_pzbd_dn8, locals.var_pzbd_dn9, locals.var_pzbd_dn10, locals.var_pzbd_dn11, locals.var_pzbd_dn14,)
    }
};
        locals.var_pzbd = assign96920_e149476;
        locals.var_pzbd_dn0 = assign96920_e149476_d_n0;
        locals.var_pzbd_dn2 = assign96920_e149476_d_n2;
        locals.var_pzbd_dn4 = assign96920_e149476_d_n4;
        locals.var_pzbd_dn5 = assign96920_e149476_d_n5;
        locals.var_pzbd_dn6 = assign96920_e149476_d_n6;
        locals.var_pzbd_dn7 = assign96920_e149476_d_n7;
        locals.var_pzbd_dn8 = assign96920_e149476_d_n8;
        locals.var_pzbd_dn9 = assign96920_e149476_d_n9;
        locals.var_pzbd_dn10 = assign96920_e149476_d_n10;
        locals.var_pzbd_dn11 = assign96920_e149476_d_n11;
        locals.var_pzbd_dn14 = assign96920_e149476_d_n14;
        locals.var_pzbd_rv = 0.0;

        let assign96930_e149483: f64 = if ((locals.var_pzbdsw < 0.01) && (p.p15 > locals.var_weff_nf)) { 1.0 } else { 0.0 };
        locals.var_guard2249 = assign96930_e149483;
        locals.var_guard2249_rv = 0.0;

        let (assign96940_e149489, assign96940_e149489_d_n0, assign96940_e149489_d_n2, assign96940_e149489_d_n4, assign96940_e149489_d_n5, assign96940_e149489_d_n6, assign96940_e149489_d_n7, assign96940_e149489_d_n8, assign96940_e149489_d_n9, assign96940_e149489_d_n10, assign96940_e149489_d_n11, assign96940_e149489_d_n14,) = {
    if ((locals.var_guard2237 != 0.0) && (locals.var_guard2249 != 0.0)) {
        (0.01, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pzbdsw, locals.var_pzbdsw_dn0, locals.var_pzbdsw_dn2, locals.var_pzbdsw_dn4, locals.var_pzbdsw_dn5, locals.var_pzbdsw_dn6, locals.var_pzbdsw_dn7, locals.var_pzbdsw_dn8, locals.var_pzbdsw_dn9, locals.var_pzbdsw_dn10, locals.var_pzbdsw_dn11, locals.var_pzbdsw_dn14,)
    }
};
        locals.var_pzbdsw = assign96940_e149489;
        locals.var_pzbdsw_dn0 = assign96940_e149489_d_n0;
        locals.var_pzbdsw_dn2 = assign96940_e149489_d_n2;
        locals.var_pzbdsw_dn4 = assign96940_e149489_d_n4;
        locals.var_pzbdsw_dn5 = assign96940_e149489_d_n5;
        locals.var_pzbdsw_dn6 = assign96940_e149489_d_n6;
        locals.var_pzbdsw_dn7 = assign96940_e149489_d_n7;
        locals.var_pzbdsw_dn8 = assign96940_e149489_d_n8;
        locals.var_pzbdsw_dn9 = assign96940_e149489_d_n9;
        locals.var_pzbdsw_dn10 = assign96940_e149489_d_n10;
        locals.var_pzbdsw_dn11 = assign96940_e149489_d_n11;
        locals.var_pzbdsw_dn14 = assign96940_e149489_d_n14;
        locals.var_pzbdsw_rv = 0.0;

        let assign96950_e149496: f64 = if ((locals.var_pzbdswg < 0.01) && (p.p15 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2250 = assign96950_e149496;
        locals.var_guard2250_rv = 0.0;

        let (assign96960_e149502, assign96960_e149502_d_n0, assign96960_e149502_d_n2, assign96960_e149502_d_n4, assign96960_e149502_d_n5, assign96960_e149502_d_n6, assign96960_e149502_d_n7, assign96960_e149502_d_n8, assign96960_e149502_d_n9, assign96960_e149502_d_n10, assign96960_e149502_d_n11, assign96960_e149502_d_n14,) = {
    if ((locals.var_guard2237 != 0.0) && (locals.var_guard2250 != 0.0)) {
        (0.01, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pzbdswg, locals.var_pzbdswg_dn0, locals.var_pzbdswg_dn2, locals.var_pzbdswg_dn4, locals.var_pzbdswg_dn5, locals.var_pzbdswg_dn6, locals.var_pzbdswg_dn7, locals.var_pzbdswg_dn8, locals.var_pzbdswg_dn9, locals.var_pzbdswg_dn10, locals.var_pzbdswg_dn11, locals.var_pzbdswg_dn14,)
    }
};
        locals.var_pzbdswg = assign96960_e149502;
        locals.var_pzbdswg_dn0 = assign96960_e149502_d_n0;
        locals.var_pzbdswg_dn2 = assign96960_e149502_d_n2;
        locals.var_pzbdswg_dn4 = assign96960_e149502_d_n4;
        locals.var_pzbdswg_dn5 = assign96960_e149502_d_n5;
        locals.var_pzbdswg_dn6 = assign96960_e149502_d_n6;
        locals.var_pzbdswg_dn7 = assign96960_e149502_d_n7;
        locals.var_pzbdswg_dn8 = assign96960_e149502_d_n8;
        locals.var_pzbdswg_dn9 = assign96960_e149502_d_n9;
        locals.var_pzbdswg_dn10 = assign96960_e149502_d_n10;
        locals.var_pzbdswg_dn11 = assign96960_e149502_d_n11;
        locals.var_pzbdswg_dn14 = assign96960_e149502_d_n14;
        locals.var_pzbdswg_rv = 0.0;

        let (assign96970_e149514, assign96970_e149514_d_n0, assign96970_e149514_d_n2, assign96970_e149514_d_n4, assign96970_e149514_d_n5, assign96970_e149514_d_n6, assign96970_e149514_d_n7, assign96970_e149514_d_n8, assign96970_e149514_d_n9, assign96970_e149514_d_n10, assign96970_e149514_d_n11, assign96970_e149514_d_n14,) = {
    if (locals.var_guard2237 != 0.0) {
        let assign96970_e149506: f64 = (p.p523 * p.p14);
        let assign96970_e149510: f64 = (p.p482 * locals.var_tdiff);
        let assign96970_e149511: f64 = (1.0 + assign96970_e149510);
        let assign96970_e149512: f64 = (assign96970_e149506 * assign96970_e149511);
        (assign96970_e149512, (assign96970_e149506 * (p.p482 * locals.var_tdiff_dn0)), (assign96970_e149506 * (p.p482 * locals.var_tdiff_dn2)), (assign96970_e149506 * (p.p482 * locals.var_tdiff_dn4)), (assign96970_e149506 * (p.p482 * locals.var_tdiff_dn5)), (assign96970_e149506 * (p.p482 * locals.var_tdiff_dn6)), (assign96970_e149506 * (p.p482 * locals.var_tdiff_dn7)), (assign96970_e149506 * (p.p482 * locals.var_tdiff_dn8)), (assign96970_e149506 * (p.p482 * locals.var_tdiff_dn9)), (assign96970_e149506 * (p.p482 * locals.var_tdiff_dn10)), (assign96970_e149506 * (p.p482 * locals.var_tdiff_dn11)), (assign96970_e149506 * (p.p482 * locals.var_tdiff_dn14)),)
    } else {
        (locals.var_czbs, locals.var_czbs_dn0, locals.var_czbs_dn2, locals.var_czbs_dn4, locals.var_czbs_dn5, locals.var_czbs_dn6, locals.var_czbs_dn7, locals.var_czbs_dn8, locals.var_czbs_dn9, locals.var_czbs_dn10, locals.var_czbs_dn11, locals.var_czbs_dn14,)
    }
};
        locals.var_czbs = assign96970_e149514;
        locals.var_czbs_dn0 = assign96970_e149514_d_n0;
        locals.var_czbs_dn2 = assign96970_e149514_d_n2;
        locals.var_czbs_dn4 = assign96970_e149514_d_n4;
        locals.var_czbs_dn5 = assign96970_e149514_d_n5;
        locals.var_czbs_dn6 = assign96970_e149514_d_n6;
        locals.var_czbs_dn7 = assign96970_e149514_d_n7;
        locals.var_czbs_dn8 = assign96970_e149514_d_n8;
        locals.var_czbs_dn9 = assign96970_e149514_d_n9;
        locals.var_czbs_dn10 = assign96970_e149514_d_n10;
        locals.var_czbs_dn11 = assign96970_e149514_d_n11;
        locals.var_czbs_dn14 = assign96970_e149514_d_n14;
        locals.var_czbs_rv = 0.0;

        let assign96980_e149517: f64 = if p.p16 > locals.var_weff_nf { 1.0 } else { 0.0 };
        locals.var_guard2251 = assign96980_e149517;
        locals.var_guard2251_rv = 0.0;

        let (assign96990_e149533, assign96990_e149533_d_n0, assign96990_e149533_d_n2, assign96990_e149533_d_n4, assign96990_e149533_d_n5, assign96990_e149533_d_n6, assign96990_e149533_d_n7, assign96990_e149533_d_n8, assign96990_e149533_d_n9, assign96990_e149533_d_n10, assign96990_e149533_d_n11, assign96990_e149533_d_n14,) = {
    if ((locals.var_guard2237 != 0.0) && (locals.var_guard2251 != 0.0)) {
        let assign96990_e149524: f64 = (p.p16 - locals.var_weff_nf);
        let assign96990_e149525: f64 = (p.p524 * assign96990_e149524);
        let assign96990_e149529: f64 = (p.p484 * locals.var_tdiff);
        let assign96990_e149530: f64 = (1.0 + assign96990_e149529);
        let assign96990_e149531: f64 = (assign96990_e149525 * assign96990_e149530);
        (assign96990_e149531, (assign96990_e149525 * (p.p484 * locals.var_tdiff_dn0)), (assign96990_e149525 * (p.p484 * locals.var_tdiff_dn2)), (assign96990_e149525 * (p.p484 * locals.var_tdiff_dn4)), (assign96990_e149525 * (p.p484 * locals.var_tdiff_dn5)), (assign96990_e149525 * (p.p484 * locals.var_tdiff_dn6)), (assign96990_e149525 * (p.p484 * locals.var_tdiff_dn7)), (assign96990_e149525 * (p.p484 * locals.var_tdiff_dn8)), (assign96990_e149525 * (p.p484 * locals.var_tdiff_dn9)), (assign96990_e149525 * (p.p484 * locals.var_tdiff_dn10)), (assign96990_e149525 * (p.p484 * locals.var_tdiff_dn11)), (assign96990_e149525 * (p.p484 * locals.var_tdiff_dn14)),)
    } else {
        (locals.var_czbssw, locals.var_czbssw_dn0, locals.var_czbssw_dn2, locals.var_czbssw_dn4, locals.var_czbssw_dn5, locals.var_czbssw_dn6, locals.var_czbssw_dn7, locals.var_czbssw_dn8, locals.var_czbssw_dn9, locals.var_czbssw_dn10, locals.var_czbssw_dn11, locals.var_czbssw_dn14,)
    }
};
        locals.var_czbssw = assign96990_e149533;
        locals.var_czbssw_dn0 = assign96990_e149533_d_n0;
        locals.var_czbssw_dn2 = assign96990_e149533_d_n2;
        locals.var_czbssw_dn4 = assign96990_e149533_d_n4;
        locals.var_czbssw_dn5 = assign96990_e149533_d_n5;
        locals.var_czbssw_dn6 = assign96990_e149533_d_n6;
        locals.var_czbssw_dn7 = assign96990_e149533_d_n7;
        locals.var_czbssw_dn8 = assign96990_e149533_d_n8;
        locals.var_czbssw_dn9 = assign96990_e149533_d_n9;
        locals.var_czbssw_dn10 = assign96990_e149533_d_n10;
        locals.var_czbssw_dn11 = assign96990_e149533_d_n11;
        locals.var_czbssw_dn14 = assign96990_e149533_d_n14;
        locals.var_czbssw_rv = 0.0;

        let (assign97000_e149547, assign97000_e149547_d_n0, assign97000_e149547_d_n2, assign97000_e149547_d_n4, assign97000_e149547_d_n5, assign97000_e149547_d_n6, assign97000_e149547_d_n7, assign97000_e149547_d_n8, assign97000_e149547_d_n9, assign97000_e149547_d_n10, assign97000_e149547_d_n11, assign97000_e149547_d_n14,) = {
    if ((locals.var_guard2237 != 0.0) && (locals.var_guard2251 != 0.0)) {
        let assign97000_e149539: f64 = (p.p525 * locals.var_weff_nf);
        let assign97000_e149543: f64 = (p.p486 * locals.var_tdiff);
        let assign97000_e149544: f64 = (1.0 + assign97000_e149543);
        let assign97000_e149545: f64 = (assign97000_e149539 * assign97000_e149544);
        (assign97000_e149545, (assign97000_e149539 * (p.p486 * locals.var_tdiff_dn0)), (assign97000_e149539 * (p.p486 * locals.var_tdiff_dn2)), (assign97000_e149539 * (p.p486 * locals.var_tdiff_dn4)), (assign97000_e149539 * (p.p486 * locals.var_tdiff_dn5)), (assign97000_e149539 * (p.p486 * locals.var_tdiff_dn6)), (assign97000_e149539 * (p.p486 * locals.var_tdiff_dn7)), (assign97000_e149539 * (p.p486 * locals.var_tdiff_dn8)), (assign97000_e149539 * (p.p486 * locals.var_tdiff_dn9)), (assign97000_e149539 * (p.p486 * locals.var_tdiff_dn10)), (assign97000_e149539 * (p.p486 * locals.var_tdiff_dn11)), (assign97000_e149539 * (p.p486 * locals.var_tdiff_dn14)),)
    } else {
        (locals.var_czbsswg, locals.var_czbsswg_dn0, locals.var_czbsswg_dn2, locals.var_czbsswg_dn4, locals.var_czbsswg_dn5, locals.var_czbsswg_dn6, locals.var_czbsswg_dn7, locals.var_czbsswg_dn8, locals.var_czbsswg_dn9, locals.var_czbsswg_dn10, locals.var_czbsswg_dn11, locals.var_czbsswg_dn14,)
    }
};
        locals.var_czbsswg = assign97000_e149547;
        locals.var_czbsswg_dn0 = assign97000_e149547_d_n0;
        locals.var_czbsswg_dn2 = assign97000_e149547_d_n2;
        locals.var_czbsswg_dn4 = assign97000_e149547_d_n4;
        locals.var_czbsswg_dn5 = assign97000_e149547_d_n5;
        locals.var_czbsswg_dn6 = assign97000_e149547_d_n6;
        locals.var_czbsswg_dn7 = assign97000_e149547_d_n7;
        locals.var_czbsswg_dn8 = assign97000_e149547_d_n8;
        locals.var_czbsswg_dn9 = assign97000_e149547_d_n9;
        locals.var_czbsswg_dn10 = assign97000_e149547_d_n10;
        locals.var_czbsswg_dn11 = assign97000_e149547_d_n11;
        locals.var_czbsswg_dn14 = assign97000_e149547_d_n14;
        locals.var_czbsswg_rv = 0.0;

        let (assign97010_e149554, assign97010_e149554_d_n0, assign97010_e149554_d_n2, assign97010_e149554_d_n4, assign97010_e149554_d_n5, assign97010_e149554_d_n6, assign97010_e149554_d_n7, assign97010_e149554_d_n8, assign97010_e149554_d_n9, assign97010_e149554_d_n10, assign97010_e149554_d_n11, assign97010_e149554_d_n14,) = {
    if ((locals.var_guard2237 != 0.0) && (locals.var_guard2251 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_czbssw, locals.var_czbssw_dn0, locals.var_czbssw_dn2, locals.var_czbssw_dn4, locals.var_czbssw_dn5, locals.var_czbssw_dn6, locals.var_czbssw_dn7, locals.var_czbssw_dn8, locals.var_czbssw_dn9, locals.var_czbssw_dn10, locals.var_czbssw_dn11, locals.var_czbssw_dn14,)
    }
};
        locals.var_czbssw = assign97010_e149554;
        locals.var_czbssw_dn0 = assign97010_e149554_d_n0;
        locals.var_czbssw_dn2 = assign97010_e149554_d_n2;
        locals.var_czbssw_dn4 = assign97010_e149554_d_n4;
        locals.var_czbssw_dn5 = assign97010_e149554_d_n5;
        locals.var_czbssw_dn6 = assign97010_e149554_d_n6;
        locals.var_czbssw_dn7 = assign97010_e149554_d_n7;
        locals.var_czbssw_dn8 = assign97010_e149554_d_n8;
        locals.var_czbssw_dn9 = assign97010_e149554_d_n9;
        locals.var_czbssw_dn10 = assign97010_e149554_d_n10;
        locals.var_czbssw_dn11 = assign97010_e149554_d_n11;
        locals.var_czbssw_dn14 = assign97010_e149554_d_n14;
        locals.var_czbssw_rv = 0.0;

        let (assign97020_e149569, assign97020_e149569_d_n0, assign97020_e149569_d_n2, assign97020_e149569_d_n4, assign97020_e149569_d_n5, assign97020_e149569_d_n6, assign97020_e149569_d_n7, assign97020_e149569_d_n8, assign97020_e149569_d_n9, assign97020_e149569_d_n10, assign97020_e149569_d_n11, assign97020_e149569_d_n14,) = {
    if ((locals.var_guard2237 != 0.0) && (locals.var_guard2251 == 0.0)) {
        let assign97020_e149561: f64 = (p.p525 * p.p16);
        let assign97020_e149565: f64 = (p.p486 * locals.var_tdiff);
        let assign97020_e149566: f64 = (1.0 + assign97020_e149565);
        let assign97020_e149567: f64 = (assign97020_e149561 * assign97020_e149566);
        (assign97020_e149567, (assign97020_e149561 * (p.p486 * locals.var_tdiff_dn0)), (assign97020_e149561 * (p.p486 * locals.var_tdiff_dn2)), (assign97020_e149561 * (p.p486 * locals.var_tdiff_dn4)), (assign97020_e149561 * (p.p486 * locals.var_tdiff_dn5)), (assign97020_e149561 * (p.p486 * locals.var_tdiff_dn6)), (assign97020_e149561 * (p.p486 * locals.var_tdiff_dn7)), (assign97020_e149561 * (p.p486 * locals.var_tdiff_dn8)), (assign97020_e149561 * (p.p486 * locals.var_tdiff_dn9)), (assign97020_e149561 * (p.p486 * locals.var_tdiff_dn10)), (assign97020_e149561 * (p.p486 * locals.var_tdiff_dn11)), (assign97020_e149561 * (p.p486 * locals.var_tdiff_dn14)),)
    } else {
        (locals.var_czbsswg, locals.var_czbsswg_dn0, locals.var_czbsswg_dn2, locals.var_czbsswg_dn4, locals.var_czbsswg_dn5, locals.var_czbsswg_dn6, locals.var_czbsswg_dn7, locals.var_czbsswg_dn8, locals.var_czbsswg_dn9, locals.var_czbsswg_dn10, locals.var_czbsswg_dn11, locals.var_czbsswg_dn14,)
    }
};
        locals.var_czbsswg = assign97020_e149569;
        locals.var_czbsswg_dn0 = assign97020_e149569_d_n0;
        locals.var_czbsswg_dn2 = assign97020_e149569_d_n2;
        locals.var_czbsswg_dn4 = assign97020_e149569_d_n4;
        locals.var_czbsswg_dn5 = assign97020_e149569_d_n5;
        locals.var_czbsswg_dn6 = assign97020_e149569_d_n6;
        locals.var_czbsswg_dn7 = assign97020_e149569_d_n7;
        locals.var_czbsswg_dn8 = assign97020_e149569_d_n8;
        locals.var_czbsswg_dn9 = assign97020_e149569_d_n9;
        locals.var_czbsswg_dn10 = assign97020_e149569_d_n10;
        locals.var_czbsswg_dn11 = assign97020_e149569_d_n11;
        locals.var_czbsswg_dn14 = assign97020_e149569_d_n14;
        locals.var_czbsswg_rv = 0.0;

        let assign97030_e149572: f64 = if locals.var_czbs < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2252 = assign97030_e149572;
        locals.var_guard2252_rv = 0.0;

        let (assign97040_e149578, assign97040_e149578_d_n0, assign97040_e149578_d_n2, assign97040_e149578_d_n4, assign97040_e149578_d_n5, assign97040_e149578_d_n6, assign97040_e149578_d_n7, assign97040_e149578_d_n8, assign97040_e149578_d_n9, assign97040_e149578_d_n10, assign97040_e149578_d_n11, assign97040_e149578_d_n14,) = {
    if ((locals.var_guard2237 != 0.0) && (locals.var_guard2252 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_czbs, locals.var_czbs_dn0, locals.var_czbs_dn2, locals.var_czbs_dn4, locals.var_czbs_dn5, locals.var_czbs_dn6, locals.var_czbs_dn7, locals.var_czbs_dn8, locals.var_czbs_dn9, locals.var_czbs_dn10, locals.var_czbs_dn11, locals.var_czbs_dn14,)
    }
};
        locals.var_czbs = assign97040_e149578;
        locals.var_czbs_dn0 = assign97040_e149578_d_n0;
        locals.var_czbs_dn2 = assign97040_e149578_d_n2;
        locals.var_czbs_dn4 = assign97040_e149578_d_n4;
        locals.var_czbs_dn5 = assign97040_e149578_d_n5;
        locals.var_czbs_dn6 = assign97040_e149578_d_n6;
        locals.var_czbs_dn7 = assign97040_e149578_d_n7;
        locals.var_czbs_dn8 = assign97040_e149578_d_n8;
        locals.var_czbs_dn9 = assign97040_e149578_d_n9;
        locals.var_czbs_dn10 = assign97040_e149578_d_n10;
        locals.var_czbs_dn11 = assign97040_e149578_d_n11;
        locals.var_czbs_dn14 = assign97040_e149578_d_n14;
        locals.var_czbs_rv = 0.0;

        let assign97050_e149581: f64 = if locals.var_czbssw < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2253 = assign97050_e149581;
        locals.var_guard2253_rv = 0.0;

        let (assign97060_e149587, assign97060_e149587_d_n0, assign97060_e149587_d_n2, assign97060_e149587_d_n4, assign97060_e149587_d_n5, assign97060_e149587_d_n6, assign97060_e149587_d_n7, assign97060_e149587_d_n8, assign97060_e149587_d_n9, assign97060_e149587_d_n10, assign97060_e149587_d_n11, assign97060_e149587_d_n14,) = {
    if ((locals.var_guard2237 != 0.0) && (locals.var_guard2253 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_czbssw, locals.var_czbssw_dn0, locals.var_czbssw_dn2, locals.var_czbssw_dn4, locals.var_czbssw_dn5, locals.var_czbssw_dn6, locals.var_czbssw_dn7, locals.var_czbssw_dn8, locals.var_czbssw_dn9, locals.var_czbssw_dn10, locals.var_czbssw_dn11, locals.var_czbssw_dn14,)
    }
};
        locals.var_czbssw = assign97060_e149587;
        locals.var_czbssw_dn0 = assign97060_e149587_d_n0;
        locals.var_czbssw_dn2 = assign97060_e149587_d_n2;
        locals.var_czbssw_dn4 = assign97060_e149587_d_n4;
        locals.var_czbssw_dn5 = assign97060_e149587_d_n5;
        locals.var_czbssw_dn6 = assign97060_e149587_d_n6;
        locals.var_czbssw_dn7 = assign97060_e149587_d_n7;
        locals.var_czbssw_dn8 = assign97060_e149587_d_n8;
        locals.var_czbssw_dn9 = assign97060_e149587_d_n9;
        locals.var_czbssw_dn10 = assign97060_e149587_d_n10;
        locals.var_czbssw_dn11 = assign97060_e149587_d_n11;
        locals.var_czbssw_dn14 = assign97060_e149587_d_n14;
        locals.var_czbssw_rv = 0.0;

        let assign97070_e149590: f64 = if locals.var_czbsswg < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2254 = assign97070_e149590;
        locals.var_guard2254_rv = 0.0;

        let (assign97080_e149596, assign97080_e149596_d_n0, assign97080_e149596_d_n2, assign97080_e149596_d_n4, assign97080_e149596_d_n5, assign97080_e149596_d_n6, assign97080_e149596_d_n7, assign97080_e149596_d_n8, assign97080_e149596_d_n9, assign97080_e149596_d_n10, assign97080_e149596_d_n11, assign97080_e149596_d_n14,) = {
    if ((locals.var_guard2237 != 0.0) && (locals.var_guard2254 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_czbsswg, locals.var_czbsswg_dn0, locals.var_czbsswg_dn2, locals.var_czbsswg_dn4, locals.var_czbsswg_dn5, locals.var_czbsswg_dn6, locals.var_czbsswg_dn7, locals.var_czbsswg_dn8, locals.var_czbsswg_dn9, locals.var_czbsswg_dn10, locals.var_czbsswg_dn11, locals.var_czbsswg_dn14,)
    }
};
        locals.var_czbsswg = assign97080_e149596;
        locals.var_czbsswg_dn0 = assign97080_e149596_d_n0;
        locals.var_czbsswg_dn2 = assign97080_e149596_d_n2;
        locals.var_czbsswg_dn4 = assign97080_e149596_d_n4;
        locals.var_czbsswg_dn5 = assign97080_e149596_d_n5;
        locals.var_czbsswg_dn6 = assign97080_e149596_d_n6;
        locals.var_czbsswg_dn7 = assign97080_e149596_d_n7;
        locals.var_czbsswg_dn8 = assign97080_e149596_d_n8;
        locals.var_czbsswg_dn9 = assign97080_e149596_d_n9;
        locals.var_czbsswg_dn10 = assign97080_e149596_d_n10;
        locals.var_czbsswg_dn11 = assign97080_e149596_d_n11;
        locals.var_czbsswg_dn14 = assign97080_e149596_d_n14;
        locals.var_czbsswg_rv = 0.0;

        let (assign97090_e149604, assign97090_e149604_d_n0, assign97090_e149604_d_n2, assign97090_e149604_d_n4, assign97090_e149604_d_n5, assign97090_e149604_d_n6, assign97090_e149604_d_n7, assign97090_e149604_d_n8, assign97090_e149604_d_n9, assign97090_e149604_d_n10, assign97090_e149604_d_n11, assign97090_e149604_d_n14,) = {
    if (locals.var_guard2237 != 0.0) {
        let assign97090_e149601: f64 = (p.p488 * locals.var_tdiff);
        let assign97090_e149602: f64 = (p.p529 - assign97090_e149601);
        (assign97090_e149602, (-(p.p488 * locals.var_tdiff_dn0)), (-(p.p488 * locals.var_tdiff_dn2)), (-(p.p488 * locals.var_tdiff_dn4)), (-(p.p488 * locals.var_tdiff_dn5)), (-(p.p488 * locals.var_tdiff_dn6)), (-(p.p488 * locals.var_tdiff_dn7)), (-(p.p488 * locals.var_tdiff_dn8)), (-(p.p488 * locals.var_tdiff_dn9)), (-(p.p488 * locals.var_tdiff_dn10)), (-(p.p488 * locals.var_tdiff_dn11)), (-(p.p488 * locals.var_tdiff_dn14)),)
    } else {
        (locals.var_pzbs, locals.var_pzbs_dn0, locals.var_pzbs_dn2, locals.var_pzbs_dn4, locals.var_pzbs_dn5, locals.var_pzbs_dn6, locals.var_pzbs_dn7, locals.var_pzbs_dn8, locals.var_pzbs_dn9, locals.var_pzbs_dn10, locals.var_pzbs_dn11, locals.var_pzbs_dn14,)
    }
};
        locals.var_pzbs = assign97090_e149604;
        locals.var_pzbs_dn0 = assign97090_e149604_d_n0;
        locals.var_pzbs_dn2 = assign97090_e149604_d_n2;
        locals.var_pzbs_dn4 = assign97090_e149604_d_n4;
        locals.var_pzbs_dn5 = assign97090_e149604_d_n5;
        locals.var_pzbs_dn6 = assign97090_e149604_d_n6;
        locals.var_pzbs_dn7 = assign97090_e149604_d_n7;
        locals.var_pzbs_dn8 = assign97090_e149604_d_n8;
        locals.var_pzbs_dn9 = assign97090_e149604_d_n9;
        locals.var_pzbs_dn10 = assign97090_e149604_d_n10;
        locals.var_pzbs_dn11 = assign97090_e149604_d_n11;
        locals.var_pzbs_dn14 = assign97090_e149604_d_n14;
        locals.var_pzbs_rv = 0.0;

        let (assign97100_e149612, assign97100_e149612_d_n0, assign97100_e149612_d_n2, assign97100_e149612_d_n4, assign97100_e149612_d_n5, assign97100_e149612_d_n6, assign97100_e149612_d_n7, assign97100_e149612_d_n8, assign97100_e149612_d_n9, assign97100_e149612_d_n10, assign97100_e149612_d_n11, assign97100_e149612_d_n14,) = {
    if (locals.var_guard2237 != 0.0) {
        let assign97100_e149609: f64 = (p.p490 * locals.var_tdiff);
        let assign97100_e149610: f64 = (p.p530 - assign97100_e149609);
        (assign97100_e149610, (-(p.p490 * locals.var_tdiff_dn0)), (-(p.p490 * locals.var_tdiff_dn2)), (-(p.p490 * locals.var_tdiff_dn4)), (-(p.p490 * locals.var_tdiff_dn5)), (-(p.p490 * locals.var_tdiff_dn6)), (-(p.p490 * locals.var_tdiff_dn7)), (-(p.p490 * locals.var_tdiff_dn8)), (-(p.p490 * locals.var_tdiff_dn9)), (-(p.p490 * locals.var_tdiff_dn10)), (-(p.p490 * locals.var_tdiff_dn11)), (-(p.p490 * locals.var_tdiff_dn14)),)
    } else {
        (locals.var_pzbssw, locals.var_pzbssw_dn0, locals.var_pzbssw_dn2, locals.var_pzbssw_dn4, locals.var_pzbssw_dn5, locals.var_pzbssw_dn6, locals.var_pzbssw_dn7, locals.var_pzbssw_dn8, locals.var_pzbssw_dn9, locals.var_pzbssw_dn10, locals.var_pzbssw_dn11, locals.var_pzbssw_dn14,)
    }
};
        locals.var_pzbssw = assign97100_e149612;
        locals.var_pzbssw_dn0 = assign97100_e149612_d_n0;
        locals.var_pzbssw_dn2 = assign97100_e149612_d_n2;
        locals.var_pzbssw_dn4 = assign97100_e149612_d_n4;
        locals.var_pzbssw_dn5 = assign97100_e149612_d_n5;
        locals.var_pzbssw_dn6 = assign97100_e149612_d_n6;
        locals.var_pzbssw_dn7 = assign97100_e149612_d_n7;
        locals.var_pzbssw_dn8 = assign97100_e149612_d_n8;
        locals.var_pzbssw_dn9 = assign97100_e149612_d_n9;
        locals.var_pzbssw_dn10 = assign97100_e149612_d_n10;
        locals.var_pzbssw_dn11 = assign97100_e149612_d_n11;
        locals.var_pzbssw_dn14 = assign97100_e149612_d_n14;
        locals.var_pzbssw_rv = 0.0;

        let (assign97110_e149620, assign97110_e149620_d_n0, assign97110_e149620_d_n2, assign97110_e149620_d_n4, assign97110_e149620_d_n5, assign97110_e149620_d_n6, assign97110_e149620_d_n7, assign97110_e149620_d_n8, assign97110_e149620_d_n9, assign97110_e149620_d_n10, assign97110_e149620_d_n11, assign97110_e149620_d_n14,) = {
    if (locals.var_guard2237 != 0.0) {
        let assign97110_e149617: f64 = (p.p492 * locals.var_tdiff);
        let assign97110_e149618: f64 = (p.p531 - assign97110_e149617);
        (assign97110_e149618, (-(p.p492 * locals.var_tdiff_dn0)), (-(p.p492 * locals.var_tdiff_dn2)), (-(p.p492 * locals.var_tdiff_dn4)), (-(p.p492 * locals.var_tdiff_dn5)), (-(p.p492 * locals.var_tdiff_dn6)), (-(p.p492 * locals.var_tdiff_dn7)), (-(p.p492 * locals.var_tdiff_dn8)), (-(p.p492 * locals.var_tdiff_dn9)), (-(p.p492 * locals.var_tdiff_dn10)), (-(p.p492 * locals.var_tdiff_dn11)), (-(p.p492 * locals.var_tdiff_dn14)),)
    } else {
        (locals.var_pzbsswg, locals.var_pzbsswg_dn0, locals.var_pzbsswg_dn2, locals.var_pzbsswg_dn4, locals.var_pzbsswg_dn5, locals.var_pzbsswg_dn6, locals.var_pzbsswg_dn7, locals.var_pzbsswg_dn8, locals.var_pzbsswg_dn9, locals.var_pzbsswg_dn10, locals.var_pzbsswg_dn11, locals.var_pzbsswg_dn14,)
    }
};
        locals.var_pzbsswg = assign97110_e149620;
        locals.var_pzbsswg_dn0 = assign97110_e149620_d_n0;
        locals.var_pzbsswg_dn2 = assign97110_e149620_d_n2;
        locals.var_pzbsswg_dn4 = assign97110_e149620_d_n4;
        locals.var_pzbsswg_dn5 = assign97110_e149620_d_n5;
        locals.var_pzbsswg_dn6 = assign97110_e149620_d_n6;
        locals.var_pzbsswg_dn7 = assign97110_e149620_d_n7;
        locals.var_pzbsswg_dn8 = assign97110_e149620_d_n8;
        locals.var_pzbsswg_dn9 = assign97110_e149620_d_n9;
        locals.var_pzbsswg_dn10 = assign97110_e149620_d_n10;
        locals.var_pzbsswg_dn11 = assign97110_e149620_d_n11;
        locals.var_pzbsswg_dn14 = assign97110_e149620_d_n14;
        locals.var_pzbsswg_rv = 0.0;

        let assign97120_e149627: f64 = if ((locals.var_pzbs < 0.01) && (p.p14 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2255 = assign97120_e149627;
        locals.var_guard2255_rv = 0.0;

        let (assign97130_e149633, assign97130_e149633_d_n0, assign97130_e149633_d_n2, assign97130_e149633_d_n4, assign97130_e149633_d_n5, assign97130_e149633_d_n6, assign97130_e149633_d_n7, assign97130_e149633_d_n8, assign97130_e149633_d_n9, assign97130_e149633_d_n10, assign97130_e149633_d_n11, assign97130_e149633_d_n14,) = {
    if ((locals.var_guard2237 != 0.0) && (locals.var_guard2255 != 0.0)) {
        (0.01, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pzbs, locals.var_pzbs_dn0, locals.var_pzbs_dn2, locals.var_pzbs_dn4, locals.var_pzbs_dn5, locals.var_pzbs_dn6, locals.var_pzbs_dn7, locals.var_pzbs_dn8, locals.var_pzbs_dn9, locals.var_pzbs_dn10, locals.var_pzbs_dn11, locals.var_pzbs_dn14,)
    }
};
        locals.var_pzbs = assign97130_e149633;
        locals.var_pzbs_dn0 = assign97130_e149633_d_n0;
        locals.var_pzbs_dn2 = assign97130_e149633_d_n2;
        locals.var_pzbs_dn4 = assign97130_e149633_d_n4;
        locals.var_pzbs_dn5 = assign97130_e149633_d_n5;
        locals.var_pzbs_dn6 = assign97130_e149633_d_n6;
        locals.var_pzbs_dn7 = assign97130_e149633_d_n7;
        locals.var_pzbs_dn8 = assign97130_e149633_d_n8;
        locals.var_pzbs_dn9 = assign97130_e149633_d_n9;
        locals.var_pzbs_dn10 = assign97130_e149633_d_n10;
        locals.var_pzbs_dn11 = assign97130_e149633_d_n11;
        locals.var_pzbs_dn14 = assign97130_e149633_d_n14;
        locals.var_pzbs_rv = 0.0;

        let assign97140_e149640: f64 = if ((locals.var_pzbssw < 0.01) && (p.p16 > locals.var_weff_nf)) { 1.0 } else { 0.0 };
        locals.var_guard2256 = assign97140_e149640;
        locals.var_guard2256_rv = 0.0;

        let (assign97150_e149646, assign97150_e149646_d_n0, assign97150_e149646_d_n2, assign97150_e149646_d_n4, assign97150_e149646_d_n5, assign97150_e149646_d_n6, assign97150_e149646_d_n7, assign97150_e149646_d_n8, assign97150_e149646_d_n9, assign97150_e149646_d_n10, assign97150_e149646_d_n11, assign97150_e149646_d_n14,) = {
    if ((locals.var_guard2237 != 0.0) && (locals.var_guard2256 != 0.0)) {
        (0.01, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pzbssw, locals.var_pzbssw_dn0, locals.var_pzbssw_dn2, locals.var_pzbssw_dn4, locals.var_pzbssw_dn5, locals.var_pzbssw_dn6, locals.var_pzbssw_dn7, locals.var_pzbssw_dn8, locals.var_pzbssw_dn9, locals.var_pzbssw_dn10, locals.var_pzbssw_dn11, locals.var_pzbssw_dn14,)
    }
};
        locals.var_pzbssw = assign97150_e149646;
        locals.var_pzbssw_dn0 = assign97150_e149646_d_n0;
        locals.var_pzbssw_dn2 = assign97150_e149646_d_n2;
        locals.var_pzbssw_dn4 = assign97150_e149646_d_n4;
        locals.var_pzbssw_dn5 = assign97150_e149646_d_n5;
        locals.var_pzbssw_dn6 = assign97150_e149646_d_n6;
        locals.var_pzbssw_dn7 = assign97150_e149646_d_n7;
        locals.var_pzbssw_dn8 = assign97150_e149646_d_n8;
        locals.var_pzbssw_dn9 = assign97150_e149646_d_n9;
        locals.var_pzbssw_dn10 = assign97150_e149646_d_n10;
        locals.var_pzbssw_dn11 = assign97150_e149646_d_n11;
        locals.var_pzbssw_dn14 = assign97150_e149646_d_n14;
        locals.var_pzbssw_rv = 0.0;

        let assign97160_e149653: f64 = if ((locals.var_pzbsswg < 0.01) && (p.p16 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2257 = assign97160_e149653;
        locals.var_guard2257_rv = 0.0;

        let (assign97170_e149659, assign97170_e149659_d_n0, assign97170_e149659_d_n2, assign97170_e149659_d_n4, assign97170_e149659_d_n5, assign97170_e149659_d_n6, assign97170_e149659_d_n7, assign97170_e149659_d_n8, assign97170_e149659_d_n9, assign97170_e149659_d_n10, assign97170_e149659_d_n11, assign97170_e149659_d_n14,) = {
    if ((locals.var_guard2237 != 0.0) && (locals.var_guard2257 != 0.0)) {
        (0.01, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pzbsswg, locals.var_pzbsswg_dn0, locals.var_pzbsswg_dn2, locals.var_pzbsswg_dn4, locals.var_pzbsswg_dn5, locals.var_pzbsswg_dn6, locals.var_pzbsswg_dn7, locals.var_pzbsswg_dn8, locals.var_pzbsswg_dn9, locals.var_pzbsswg_dn10, locals.var_pzbsswg_dn11, locals.var_pzbsswg_dn14,)
    }
};
        locals.var_pzbsswg = assign97170_e149659;
        locals.var_pzbsswg_dn0 = assign97170_e149659_d_n0;
        locals.var_pzbsswg_dn2 = assign97170_e149659_d_n2;
        locals.var_pzbsswg_dn4 = assign97170_e149659_d_n4;
        locals.var_pzbsswg_dn5 = assign97170_e149659_d_n5;
        locals.var_pzbsswg_dn6 = assign97170_e149659_d_n6;
        locals.var_pzbsswg_dn7 = assign97170_e149659_d_n7;
        locals.var_pzbsswg_dn8 = assign97170_e149659_d_n8;
        locals.var_pzbsswg_dn9 = assign97170_e149659_d_n9;
        locals.var_pzbsswg_dn10 = assign97170_e149659_d_n10;
        locals.var_pzbsswg_dn11 = assign97170_e149659_d_n11;
        locals.var_pzbsswg_dn14 = assign97170_e149659_d_n14;
        locals.var_pzbsswg_rv = 0.0;

        let (assign97180_e149666, assign97180_e149666_d_n0, assign97180_e149666_d_n2, assign97180_e149666_d_n4, assign97180_e149666_d_n5, assign97180_e149666_d_n6, assign97180_e149666_d_n7, assign97180_e149666_d_n8, assign97180_e149666_d_n9, assign97180_e149666_d_n10, assign97180_e149666_d_n11, assign97180_e149666_d_n14,) = {
    if (locals.var_guard2237 == 0.0) {
        let assign97180_e149662: f64 = ctx_temp;
        let assign97180_e149664: f64 = (assign97180_e149662 + p.p11);
        (assign97180_e149664, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ttemp, locals.var_ttemp_dn0, locals.var_ttemp_dn2, locals.var_ttemp_dn4, locals.var_ttemp_dn5, locals.var_ttemp_dn6, locals.var_ttemp_dn7, locals.var_ttemp_dn8, locals.var_ttemp_dn9, locals.var_ttemp_dn10, locals.var_ttemp_dn11, locals.var_ttemp_dn14,)
    }
};
        locals.var_ttemp = assign97180_e149666;
        locals.var_ttemp_dn0 = assign97180_e149666_d_n0;
        locals.var_ttemp_dn2 = assign97180_e149666_d_n2;
        locals.var_ttemp_dn4 = assign97180_e149666_d_n4;
        locals.var_ttemp_dn5 = assign97180_e149666_d_n5;
        locals.var_ttemp_dn6 = assign97180_e149666_d_n6;
        locals.var_ttemp_dn7 = assign97180_e149666_d_n7;
        locals.var_ttemp_dn8 = assign97180_e149666_d_n8;
        locals.var_ttemp_dn9 = assign97180_e149666_d_n9;
        locals.var_ttemp_dn10 = assign97180_e149666_d_n10;
        locals.var_ttemp_dn11 = assign97180_e149666_d_n11;
        locals.var_ttemp_dn14 = assign97180_e149666_d_n14;
        locals.var_ttemp_rv = 0.0;

        let assign97190_e149669: f64 = (p.p511 * locals.var_jd_nvtm_invd);
        locals.var_t10 = assign97190_e149669;
        locals.var_t10_dn0 = (p.p511 * locals.var_jd_nvtm_invd_dn0);
        locals.var_t10_dn2 = (p.p511 * locals.var_jd_nvtm_invd_dn2);
        locals.var_t10_dn4 = (p.p511 * locals.var_jd_nvtm_invd_dn4);
        locals.var_t10_dn5 = (p.p511 * locals.var_jd_nvtm_invd_dn5);
        locals.var_t10_dn6 = (p.p511 * locals.var_jd_nvtm_invd_dn6);
        locals.var_t10_dn7 = (p.p511 * locals.var_jd_nvtm_invd_dn7);
        locals.var_t10_dn8 = (p.p511 * locals.var_jd_nvtm_invd_dn8);
        locals.var_t10_dn9 = (p.p511 * locals.var_jd_nvtm_invd_dn9);
        locals.var_t10_dn10 = (p.p511 * locals.var_jd_nvtm_invd_dn10);
        locals.var_t10_dn11 = (p.p511 * locals.var_jd_nvtm_invd_dn11);
        locals.var_t10_dn14 = (p.p511 * locals.var_jd_nvtm_invd_dn14);
        locals.var_t10_rv = 0.0;

        let assign97200_e149672: f64 = (p.p510 * locals.var_exptempd);
        locals.var_t9 = assign97200_e149672;
        locals.var_t9_dn0 = (p.p510 * locals.var_exptempd_dn0);
        locals.var_t9_dn2 = (p.p510 * locals.var_exptempd_dn2);
        locals.var_t9_dn4 = (p.p510 * locals.var_exptempd_dn4);
        locals.var_t9_dn5 = (p.p510 * locals.var_exptempd_dn5);
        locals.var_t9_dn6 = (p.p510 * locals.var_exptempd_dn6);
        locals.var_t9_dn7 = (p.p510 * locals.var_exptempd_dn7);
        locals.var_t9_dn8 = (p.p510 * locals.var_exptempd_dn8);
        locals.var_t9_dn9 = (p.p510 * locals.var_exptempd_dn9);
        locals.var_t9_dn10 = (p.p510 * locals.var_exptempd_dn10);
        locals.var_t9_dn11 = (p.p510 * locals.var_exptempd_dn11);
        locals.var_t9_dn14 = (p.p510 * locals.var_exptempd_dn14);
        locals.var_t9_rv = 0.0;

        let assign97210_e149675: f64 = if locals.var_isbd_btm > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2258 = assign97210_e149675;
        locals.var_guard2258_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_377(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign97220_e149681, assign97220_e149681_d_n0, assign97220_e149681_d_n2, assign97220_e149681_d_n4, assign97220_e149681_d_n5, assign97220_e149681_d_n6, assign97220_e149681_d_n7, assign97220_e149681_d_n8, assign97220_e149681_d_n9, assign97220_e149681_d_n10, assign97220_e149681_d_n11, assign97220_e149681_d_n14,) = {
    if (locals.var_guard2258 != 0.0) {
        let assign97220_e149679: f64 = (locals.var_isbd2_btm * locals.var_t9);
        (assign97220_e149679, ((locals.var_isbd2_btm_dn0 * locals.var_t9) + (locals.var_isbd2_btm * locals.var_t9_dn0)), ((locals.var_isbd2_btm_dn2 * locals.var_t9) + (locals.var_isbd2_btm * locals.var_t9_dn2)), ((locals.var_isbd2_btm_dn4 * locals.var_t9) + (locals.var_isbd2_btm * locals.var_t9_dn4)), ((locals.var_isbd2_btm_dn5 * locals.var_t9) + (locals.var_isbd2_btm * locals.var_t9_dn5)), ((locals.var_isbd2_btm_dn6 * locals.var_t9) + (locals.var_isbd2_btm * locals.var_t9_dn6)), ((locals.var_isbd2_btm_dn7 * locals.var_t9) + (locals.var_isbd2_btm * locals.var_t9_dn7)), ((locals.var_isbd2_btm_dn8 * locals.var_t9) + (locals.var_isbd2_btm * locals.var_t9_dn8)), ((locals.var_isbd2_btm_dn9 * locals.var_t9) + (locals.var_isbd2_btm * locals.var_t9_dn9)), ((locals.var_isbd2_btm_dn10 * locals.var_t9) + (locals.var_isbd2_btm * locals.var_t9_dn10)), ((locals.var_isbd2_btm_dn11 * locals.var_t9) + (locals.var_isbd2_btm * locals.var_t9_dn11)), ((locals.var_isbd2_btm_dn14 * locals.var_t9) + (locals.var_isbd2_btm * locals.var_t9_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign97220_e149681;
        locals.var_t0_dn0 = assign97220_e149681_d_n0;
        locals.var_t0_dn2 = assign97220_e149681_d_n2;
        locals.var_t0_dn4 = assign97220_e149681_d_n4;
        locals.var_t0_dn5 = assign97220_e149681_d_n5;
        locals.var_t0_dn6 = assign97220_e149681_d_n6;
        locals.var_t0_dn7 = assign97220_e149681_d_n7;
        locals.var_t0_dn8 = assign97220_e149681_d_n8;
        locals.var_t0_dn9 = assign97220_e149681_d_n9;
        locals.var_t0_dn10 = assign97220_e149681_d_n10;
        locals.var_t0_dn11 = assign97220_e149681_d_n11;
        locals.var_t0_dn14 = assign97220_e149681_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign97230_e149688, assign97230_e149688_d_n0, assign97230_e149688_d_n2, assign97230_e149688_d_n4, assign97230_e149688_d_n5, assign97230_e149688_d_n6, assign97230_e149688_d_n7, assign97230_e149688_d_n8, assign97230_e149688_d_n9, assign97230_e149688_d_n10, assign97230_e149688_d_n11, assign97230_e149688_d_n14,) = {
    if (locals.var_guard2258 != 0.0) {
        let assign97230_e149684: f64 = (-locals.var_vbd_jct);
        let assign97230_e149686: f64 = (assign97230_e149684 * locals.var_t10);
        (assign97230_e149686, (((-locals.var_vbd_jct_dn0) * locals.var_t10) + (assign97230_e149684 * locals.var_t10_dn0)), (assign97230_e149684 * locals.var_t10_dn2), (assign97230_e149684 * locals.var_t10_dn4), (assign97230_e149684 * locals.var_t10_dn5), (assign97230_e149684 * locals.var_t10_dn6), (assign97230_e149684 * locals.var_t10_dn7), (assign97230_e149684 * locals.var_t10_dn8), (assign97230_e149684 * locals.var_t10_dn9), (((-locals.var_vbd_jct_dn10) * locals.var_t10) + (assign97230_e149684 * locals.var_t10_dn10)), (assign97230_e149684 * locals.var_t10_dn11), (assign97230_e149684 * locals.var_t10_dn14),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign97230_e149688;
        locals.var_tx_dn0 = assign97230_e149688_d_n0;
        locals.var_tx_dn2 = assign97230_e149688_d_n2;
        locals.var_tx_dn4 = assign97230_e149688_d_n4;
        locals.var_tx_dn5 = assign97230_e149688_d_n5;
        locals.var_tx_dn6 = assign97230_e149688_d_n6;
        locals.var_tx_dn7 = assign97230_e149688_d_n7;
        locals.var_tx_dn8 = assign97230_e149688_d_n8;
        locals.var_tx_dn9 = assign97230_e149688_d_n9;
        locals.var_tx_dn10 = assign97230_e149688_d_n10;
        locals.var_tx_dn11 = assign97230_e149688_d_n11;
        locals.var_tx_dn14 = assign97230_e149688_d_n14;
        locals.var_tx_rv = 0.0;

        let (assign97240_e149693, assign97240_e149693_d_n0, assign97240_e149693_d_n2, assign97240_e149693_d_n4, assign97240_e149693_d_n5, assign97240_e149693_d_n6, assign97240_e149693_d_n7, assign97240_e149693_d_n8, assign97240_e149693_d_n9, assign97240_e149693_d_n10, assign97240_e149693_d_n11, assign97240_e149693_d_n14,) = {
    if (locals.var_guard2258 != 0.0) {
        let assign97240_e149691: f64 = (locals.var_tx).exp();
        (assign97240_e149691, (assign97240_e149691 * locals.var_tx_dn0), (assign97240_e149691 * locals.var_tx_dn2), (assign97240_e149691 * locals.var_tx_dn4), (assign97240_e149691 * locals.var_tx_dn5), (assign97240_e149691 * locals.var_tx_dn6), (assign97240_e149691 * locals.var_tx_dn7), (assign97240_e149691 * locals.var_tx_dn8), (assign97240_e149691 * locals.var_tx_dn9), (assign97240_e149691 * locals.var_tx_dn10), (assign97240_e149691 * locals.var_tx_dn11), (assign97240_e149691 * locals.var_tx_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign97240_e149693;
        locals.var_t2_dn0 = assign97240_e149693_d_n0;
        locals.var_t2_dn2 = assign97240_e149693_d_n2;
        locals.var_t2_dn4 = assign97240_e149693_d_n4;
        locals.var_t2_dn5 = assign97240_e149693_d_n5;
        locals.var_t2_dn6 = assign97240_e149693_d_n6;
        locals.var_t2_dn7 = assign97240_e149693_d_n7;
        locals.var_t2_dn8 = assign97240_e149693_d_n8;
        locals.var_t2_dn9 = assign97240_e149693_d_n9;
        locals.var_t2_dn10 = assign97240_e149693_d_n10;
        locals.var_t2_dn11 = assign97240_e149693_d_n11;
        locals.var_t2_dn14 = assign97240_e149693_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign97250_e149697, assign97250_e149697_d_n0, assign97250_e149697_d_n2, assign97250_e149697_d_n4, assign97250_e149697_d_n5, assign97250_e149697_d_n6, assign97250_e149697_d_n7, assign97250_e149697_d_n8, assign97250_e149697_d_n9, assign97250_e149697_d_n10, assign97250_e149697_d_n11, assign97250_e149697_d_n14,) = {
    if (locals.var_guard2258 != 0.0) {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign97250_e149697;
        locals.var_t3_dn0 = assign97250_e149697_d_n0;
        locals.var_t3_dn2 = assign97250_e149697_d_n2;
        locals.var_t3_dn4 = assign97250_e149697_d_n4;
        locals.var_t3_dn5 = assign97250_e149697_d_n5;
        locals.var_t3_dn6 = assign97250_e149697_d_n6;
        locals.var_t3_dn7 = assign97250_e149697_d_n7;
        locals.var_t3_dn8 = assign97250_e149697_d_n8;
        locals.var_t3_dn9 = assign97250_e149697_d_n9;
        locals.var_t3_dn10 = assign97250_e149697_d_n10;
        locals.var_t3_dn11 = assign97250_e149697_d_n11;
        locals.var_t3_dn14 = assign97250_e149697_d_n14;
        locals.var_t3_rv = 0.0;

        let assign97260_e149700: f64 = if locals.var_vbd_jct < locals.var_vbdt { 1.0 } else { 0.0 };
        locals.var_guard2259 = assign97260_e149700;
        locals.var_guard2259_rv = 0.0;

        let (assign97270_e149708, assign97270_e149708_d_n0, assign97270_e149708_d_n2, assign97270_e149708_d_n4, assign97270_e149708_d_n5, assign97270_e149708_d_n6, assign97270_e149708_d_n7, assign97270_e149708_d_n8, assign97270_e149708_d_n9, assign97270_e149708_d_n10, assign97270_e149708_d_n11, assign97270_e149708_d_n14,) = {
    if ((locals.var_guard2258 != 0.0) && (locals.var_guard2259 != 0.0)) {
        let assign97270_e149706: f64 = (locals.var_vbd_jct * locals.var_jd_nvtm_invd);
        (assign97270_e149706, ((locals.var_vbd_jct_dn0 * locals.var_jd_nvtm_invd) + (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn0)), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn2), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn4), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn5), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn6), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn7), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn8), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn9), ((locals.var_vbd_jct_dn10 * locals.var_jd_nvtm_invd) + (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn10)), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn11), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn14),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign97270_e149708;
        locals.var_tx_dn0 = assign97270_e149708_d_n0;
        locals.var_tx_dn2 = assign97270_e149708_d_n2;
        locals.var_tx_dn4 = assign97270_e149708_d_n4;
        locals.var_tx_dn5 = assign97270_e149708_d_n5;
        locals.var_tx_dn6 = assign97270_e149708_d_n6;
        locals.var_tx_dn7 = assign97270_e149708_d_n7;
        locals.var_tx_dn8 = assign97270_e149708_d_n8;
        locals.var_tx_dn9 = assign97270_e149708_d_n9;
        locals.var_tx_dn10 = assign97270_e149708_d_n10;
        locals.var_tx_dn11 = assign97270_e149708_d_n11;
        locals.var_tx_dn14 = assign97270_e149708_d_n14;
        locals.var_tx_rv = 0.0;

        let assign97280_e149711: f64 = (-3.0);
        let assign97280_e149713: f64 = (assign97280_e149711 * 34.0);
        let assign97280_e149714: f64 = if locals.var_tx < assign97280_e149713 { 1.0 } else { 0.0 };
        locals.var_guard2260 = assign97280_e149714;
        locals.var_guard2260_rv = 0.0;

        let (assign97290_e149722, assign97290_e149722_d_n0, assign97290_e149722_d_n2, assign97290_e149722_d_n4, assign97290_e149722_d_n5, assign97290_e149722_d_n6, assign97290_e149722_d_n7, assign97290_e149722_d_n8, assign97290_e149722_d_n9, assign97290_e149722_d_n10, assign97290_e149722_d_n11, assign97290_e149722_d_n14,) = {
    if (((locals.var_guard2258 != 0.0) && (locals.var_guard2259 != 0.0)) && (locals.var_guard2260 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign97290_e149722;
        locals.var_t1_dn0 = assign97290_e149722_d_n0;
        locals.var_t1_dn2 = assign97290_e149722_d_n2;
        locals.var_t1_dn4 = assign97290_e149722_d_n4;
        locals.var_t1_dn5 = assign97290_e149722_d_n5;
        locals.var_t1_dn6 = assign97290_e149722_d_n6;
        locals.var_t1_dn7 = assign97290_e149722_d_n7;
        locals.var_t1_dn8 = assign97290_e149722_d_n8;
        locals.var_t1_dn9 = assign97290_e149722_d_n9;
        locals.var_t1_dn10 = assign97290_e149722_d_n10;
        locals.var_t1_dn11 = assign97290_e149722_d_n11;
        locals.var_t1_dn14 = assign97290_e149722_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign97300_e149732, assign97300_e149732_d_n0, assign97300_e149732_d_n2, assign97300_e149732_d_n4, assign97300_e149732_d_n5, assign97300_e149732_d_n6, assign97300_e149732_d_n7, assign97300_e149732_d_n8, assign97300_e149732_d_n9, assign97300_e149732_d_n10, assign97300_e149732_d_n11, assign97300_e149732_d_n14,) = {
    if (((locals.var_guard2258 != 0.0) && (locals.var_guard2259 != 0.0)) && (locals.var_guard2260 == 0.0)) {
        let assign97300_e149730: f64 = (locals.var_tx).exp();
        (assign97300_e149730, (assign97300_e149730 * locals.var_tx_dn0), (assign97300_e149730 * locals.var_tx_dn2), (assign97300_e149730 * locals.var_tx_dn4), (assign97300_e149730 * locals.var_tx_dn5), (assign97300_e149730 * locals.var_tx_dn6), (assign97300_e149730 * locals.var_tx_dn7), (assign97300_e149730 * locals.var_tx_dn8), (assign97300_e149730 * locals.var_tx_dn9), (assign97300_e149730 * locals.var_tx_dn10), (assign97300_e149730 * locals.var_tx_dn11), (assign97300_e149730 * locals.var_tx_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign97300_e149732;
        locals.var_t1_dn0 = assign97300_e149732_d_n0;
        locals.var_t1_dn2 = assign97300_e149732_d_n2;
        locals.var_t1_dn4 = assign97300_e149732_d_n4;
        locals.var_t1_dn5 = assign97300_e149732_d_n5;
        locals.var_t1_dn6 = assign97300_e149732_d_n6;
        locals.var_t1_dn7 = assign97300_e149732_d_n7;
        locals.var_t1_dn8 = assign97300_e149732_d_n8;
        locals.var_t1_dn9 = assign97300_e149732_d_n9;
        locals.var_t1_dn10 = assign97300_e149732_d_n10;
        locals.var_t1_dn11 = assign97300_e149732_d_n11;
        locals.var_t1_dn14 = assign97300_e149732_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign97320_e149761, assign97320_e149761_d_n0, assign97320_e149761_d_n2, assign97320_e149761_d_n4, assign97320_e149761_d_n5, assign97320_e149761_d_n6, assign97320_e149761_d_n7, assign97320_e149761_d_n8, assign97320_e149761_d_n9, assign97320_e149761_d_n10, assign97320_e149761_d_n11, assign97320_e149761_d_n14,) = {
    if ((locals.var_guard2258 != 0.0) && (locals.var_guard2259 == 0.0)) {
        (locals.var_jd_expcd, locals.var_jd_expcd_dn0, locals.var_jd_expcd_dn2, locals.var_jd_expcd_dn4, locals.var_jd_expcd_dn5, locals.var_jd_expcd_dn6, locals.var_jd_expcd_dn7, locals.var_jd_expcd_dn8, locals.var_jd_expcd_dn9, locals.var_jd_expcd_dn10, locals.var_jd_expcd_dn11, locals.var_jd_expcd_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign97320_e149761;
        locals.var_t1_dn0 = assign97320_e149761_d_n0;
        locals.var_t1_dn2 = assign97320_e149761_d_n2;
        locals.var_t1_dn4 = assign97320_e149761_d_n4;
        locals.var_t1_dn5 = assign97320_e149761_d_n5;
        locals.var_t1_dn6 = assign97320_e149761_d_n6;
        locals.var_t1_dn7 = assign97320_e149761_d_n7;
        locals.var_t1_dn8 = assign97320_e149761_d_n8;
        locals.var_t1_dn9 = assign97320_e149761_d_n9;
        locals.var_t1_dn10 = assign97320_e149761_d_n10;
        locals.var_t1_dn11 = assign97320_e149761_d_n11;
        locals.var_t1_dn14 = assign97320_e149761_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign97330_e149772, assign97330_e149772_d_n0, assign97330_e149772_d_n2, assign97330_e149772_d_n4, assign97330_e149772_d_n5, assign97330_e149772_d_n6, assign97330_e149772_d_n7, assign97330_e149772_d_n8, assign97330_e149772_d_n9, assign97330_e149772_d_n10, assign97330_e149772_d_n11, assign97330_e149772_d_n14,) = {
    if ((locals.var_guard2258 != 0.0) && (locals.var_guard2259 == 0.0)) {
        let assign97330_e149768: f64 = (locals.var_isbd_btm * locals.var_jd_nvtm_invd);
        let assign97330_e149770: f64 = (assign97330_e149768 * locals.var_t1);
        (assign97330_e149770, ((((locals.var_isbd_btm_dn0 * locals.var_jd_nvtm_invd) + (locals.var_isbd_btm * locals.var_jd_nvtm_invd_dn0)) * locals.var_t1) + (assign97330_e149768 * locals.var_t1_dn0)), ((((locals.var_isbd_btm_dn2 * locals.var_jd_nvtm_invd) + (locals.var_isbd_btm * locals.var_jd_nvtm_invd_dn2)) * locals.var_t1) + (assign97330_e149768 * locals.var_t1_dn2)), ((((locals.var_isbd_btm_dn4 * locals.var_jd_nvtm_invd) + (locals.var_isbd_btm * locals.var_jd_nvtm_invd_dn4)) * locals.var_t1) + (assign97330_e149768 * locals.var_t1_dn4)), ((((locals.var_isbd_btm_dn5 * locals.var_jd_nvtm_invd) + (locals.var_isbd_btm * locals.var_jd_nvtm_invd_dn5)) * locals.var_t1) + (assign97330_e149768 * locals.var_t1_dn5)), ((((locals.var_isbd_btm_dn6 * locals.var_jd_nvtm_invd) + (locals.var_isbd_btm * locals.var_jd_nvtm_invd_dn6)) * locals.var_t1) + (assign97330_e149768 * locals.var_t1_dn6)), ((((locals.var_isbd_btm_dn7 * locals.var_jd_nvtm_invd) + (locals.var_isbd_btm * locals.var_jd_nvtm_invd_dn7)) * locals.var_t1) + (assign97330_e149768 * locals.var_t1_dn7)), ((((locals.var_isbd_btm_dn8 * locals.var_jd_nvtm_invd) + (locals.var_isbd_btm * locals.var_jd_nvtm_invd_dn8)) * locals.var_t1) + (assign97330_e149768 * locals.var_t1_dn8)), ((((locals.var_isbd_btm_dn9 * locals.var_jd_nvtm_invd) + (locals.var_isbd_btm * locals.var_jd_nvtm_invd_dn9)) * locals.var_t1) + (assign97330_e149768 * locals.var_t1_dn9)), ((((locals.var_isbd_btm_dn10 * locals.var_jd_nvtm_invd) + (locals.var_isbd_btm * locals.var_jd_nvtm_invd_dn10)) * locals.var_t1) + (assign97330_e149768 * locals.var_t1_dn10)), ((((locals.var_isbd_btm_dn11 * locals.var_jd_nvtm_invd) + (locals.var_isbd_btm * locals.var_jd_nvtm_invd_dn11)) * locals.var_t1) + (assign97330_e149768 * locals.var_t1_dn11)), ((((locals.var_isbd_btm_dn14 * locals.var_jd_nvtm_invd) + (locals.var_isbd_btm * locals.var_jd_nvtm_invd_dn14)) * locals.var_t1) + (assign97330_e149768 * locals.var_t1_dn14)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign97330_e149772;
        locals.var_t4_dn0 = assign97330_e149772_d_n0;
        locals.var_t4_dn2 = assign97330_e149772_d_n2;
        locals.var_t4_dn4 = assign97330_e149772_d_n4;
        locals.var_t4_dn5 = assign97330_e149772_d_n5;
        locals.var_t4_dn6 = assign97330_e149772_d_n6;
        locals.var_t4_dn7 = assign97330_e149772_d_n7;
        locals.var_t4_dn8 = assign97330_e149772_d_n8;
        locals.var_t4_dn9 = assign97330_e149772_d_n9;
        locals.var_t4_dn10 = assign97330_e149772_d_n10;
        locals.var_t4_dn11 = assign97330_e149772_d_n11;
        locals.var_t4_dn14 = assign97330_e149772_d_n14;
        locals.var_t4_rv = 0.0;

        let assign97360_e149809: f64 = (p.p514 * locals.var_isbd2_btm);
        locals.var_t12 = assign97360_e149809;
        locals.var_t12_dn0 = (p.p514 * locals.var_isbd2_btm_dn0);
        locals.var_t12_dn2 = (p.p514 * locals.var_isbd2_btm_dn2);
        locals.var_t12_dn4 = (p.p514 * locals.var_isbd2_btm_dn4);
        locals.var_t12_dn5 = (p.p514 * locals.var_isbd2_btm_dn5);
        locals.var_t12_dn6 = (p.p514 * locals.var_isbd2_btm_dn6);
        locals.var_t12_dn7 = (p.p514 * locals.var_isbd2_btm_dn7);
        locals.var_t12_dn8 = (p.p514 * locals.var_isbd2_btm_dn8);
        locals.var_t12_dn9 = (p.p514 * locals.var_isbd2_btm_dn9);
        locals.var_t12_dn10 = (p.p514 * locals.var_isbd2_btm_dn10);
        locals.var_t12_dn11 = (p.p514 * locals.var_isbd2_btm_dn11);
        locals.var_t12_dn14 = (p.p514 * locals.var_isbd2_btm_dn14);
        locals.var_t12_rv = 0.0;

        let assign97380_e149817: f64 = if locals.var_isbd_sws > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2261 = assign97380_e149817;
        locals.var_guard2261_rv = 0.0;

        let (assign97390_e149823, assign97390_e149823_d_n0, assign97390_e149823_d_n2, assign97390_e149823_d_n4, assign97390_e149823_d_n5, assign97390_e149823_d_n6, assign97390_e149823_d_n7, assign97390_e149823_d_n8, assign97390_e149823_d_n9, assign97390_e149823_d_n10, assign97390_e149823_d_n11, assign97390_e149823_d_n14,) = {
    if (locals.var_guard2261 != 0.0) {
        let assign97390_e149821: f64 = (locals.var_isbd2_sws * locals.var_t9);
        (assign97390_e149821, ((locals.var_isbd2_sws_dn0 * locals.var_t9) + (locals.var_isbd2_sws * locals.var_t9_dn0)), ((locals.var_isbd2_sws_dn2 * locals.var_t9) + (locals.var_isbd2_sws * locals.var_t9_dn2)), ((locals.var_isbd2_sws_dn4 * locals.var_t9) + (locals.var_isbd2_sws * locals.var_t9_dn4)), ((locals.var_isbd2_sws_dn5 * locals.var_t9) + (locals.var_isbd2_sws * locals.var_t9_dn5)), ((locals.var_isbd2_sws_dn6 * locals.var_t9) + (locals.var_isbd2_sws * locals.var_t9_dn6)), ((locals.var_isbd2_sws_dn7 * locals.var_t9) + (locals.var_isbd2_sws * locals.var_t9_dn7)), ((locals.var_isbd2_sws_dn8 * locals.var_t9) + (locals.var_isbd2_sws * locals.var_t9_dn8)), ((locals.var_isbd2_sws_dn9 * locals.var_t9) + (locals.var_isbd2_sws * locals.var_t9_dn9)), ((locals.var_isbd2_sws_dn10 * locals.var_t9) + (locals.var_isbd2_sws * locals.var_t9_dn10)), ((locals.var_isbd2_sws_dn11 * locals.var_t9) + (locals.var_isbd2_sws * locals.var_t9_dn11)), ((locals.var_isbd2_sws_dn14 * locals.var_t9) + (locals.var_isbd2_sws * locals.var_t9_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign97390_e149823;
        locals.var_t0_dn0 = assign97390_e149823_d_n0;
        locals.var_t0_dn2 = assign97390_e149823_d_n2;
        locals.var_t0_dn4 = assign97390_e149823_d_n4;
        locals.var_t0_dn5 = assign97390_e149823_d_n5;
        locals.var_t0_dn6 = assign97390_e149823_d_n6;
        locals.var_t0_dn7 = assign97390_e149823_d_n7;
        locals.var_t0_dn8 = assign97390_e149823_d_n8;
        locals.var_t0_dn9 = assign97390_e149823_d_n9;
        locals.var_t0_dn10 = assign97390_e149823_d_n10;
        locals.var_t0_dn11 = assign97390_e149823_d_n11;
        locals.var_t0_dn14 = assign97390_e149823_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign97400_e149830, assign97400_e149830_d_n0, assign97400_e149830_d_n2, assign97400_e149830_d_n4, assign97400_e149830_d_n5, assign97400_e149830_d_n6, assign97400_e149830_d_n7, assign97400_e149830_d_n8, assign97400_e149830_d_n9, assign97400_e149830_d_n10, assign97400_e149830_d_n11, assign97400_e149830_d_n14,) = {
    if (locals.var_guard2261 != 0.0) {
        let assign97400_e149826: f64 = (-locals.var_vbd_jct);
        let assign97400_e149828: f64 = (assign97400_e149826 * locals.var_t10);
        (assign97400_e149828, (((-locals.var_vbd_jct_dn0) * locals.var_t10) + (assign97400_e149826 * locals.var_t10_dn0)), (assign97400_e149826 * locals.var_t10_dn2), (assign97400_e149826 * locals.var_t10_dn4), (assign97400_e149826 * locals.var_t10_dn5), (assign97400_e149826 * locals.var_t10_dn6), (assign97400_e149826 * locals.var_t10_dn7), (assign97400_e149826 * locals.var_t10_dn8), (assign97400_e149826 * locals.var_t10_dn9), (((-locals.var_vbd_jct_dn10) * locals.var_t10) + (assign97400_e149826 * locals.var_t10_dn10)), (assign97400_e149826 * locals.var_t10_dn11), (assign97400_e149826 * locals.var_t10_dn14),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign97400_e149830;
        locals.var_tx_dn0 = assign97400_e149830_d_n0;
        locals.var_tx_dn2 = assign97400_e149830_d_n2;
        locals.var_tx_dn4 = assign97400_e149830_d_n4;
        locals.var_tx_dn5 = assign97400_e149830_d_n5;
        locals.var_tx_dn6 = assign97400_e149830_d_n6;
        locals.var_tx_dn7 = assign97400_e149830_d_n7;
        locals.var_tx_dn8 = assign97400_e149830_d_n8;
        locals.var_tx_dn9 = assign97400_e149830_d_n9;
        locals.var_tx_dn10 = assign97400_e149830_d_n10;
        locals.var_tx_dn11 = assign97400_e149830_d_n11;
        locals.var_tx_dn14 = assign97400_e149830_d_n14;
        locals.var_tx_rv = 0.0;

        let (assign97410_e149835, assign97410_e149835_d_n0, assign97410_e149835_d_n2, assign97410_e149835_d_n4, assign97410_e149835_d_n5, assign97410_e149835_d_n6, assign97410_e149835_d_n7, assign97410_e149835_d_n8, assign97410_e149835_d_n9, assign97410_e149835_d_n10, assign97410_e149835_d_n11, assign97410_e149835_d_n14,) = {
    if (locals.var_guard2261 != 0.0) {
        let assign97410_e149833: f64 = (locals.var_tx).exp();
        (assign97410_e149833, (assign97410_e149833 * locals.var_tx_dn0), (assign97410_e149833 * locals.var_tx_dn2), (assign97410_e149833 * locals.var_tx_dn4), (assign97410_e149833 * locals.var_tx_dn5), (assign97410_e149833 * locals.var_tx_dn6), (assign97410_e149833 * locals.var_tx_dn7), (assign97410_e149833 * locals.var_tx_dn8), (assign97410_e149833 * locals.var_tx_dn9), (assign97410_e149833 * locals.var_tx_dn10), (assign97410_e149833 * locals.var_tx_dn11), (assign97410_e149833 * locals.var_tx_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign97410_e149835;
        locals.var_t2_dn0 = assign97410_e149835_d_n0;
        locals.var_t2_dn2 = assign97410_e149835_d_n2;
        locals.var_t2_dn4 = assign97410_e149835_d_n4;
        locals.var_t2_dn5 = assign97410_e149835_d_n5;
        locals.var_t2_dn6 = assign97410_e149835_d_n6;
        locals.var_t2_dn7 = assign97410_e149835_d_n7;
        locals.var_t2_dn8 = assign97410_e149835_d_n8;
        locals.var_t2_dn9 = assign97410_e149835_d_n9;
        locals.var_t2_dn10 = assign97410_e149835_d_n10;
        locals.var_t2_dn11 = assign97410_e149835_d_n11;
        locals.var_t2_dn14 = assign97410_e149835_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign97420_e149839, assign97420_e149839_d_n0, assign97420_e149839_d_n2, assign97420_e149839_d_n4, assign97420_e149839_d_n5, assign97420_e149839_d_n6, assign97420_e149839_d_n7, assign97420_e149839_d_n8, assign97420_e149839_d_n9, assign97420_e149839_d_n10, assign97420_e149839_d_n11, assign97420_e149839_d_n14,) = {
    if (locals.var_guard2261 != 0.0) {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign97420_e149839;
        locals.var_t3_dn0 = assign97420_e149839_d_n0;
        locals.var_t3_dn2 = assign97420_e149839_d_n2;
        locals.var_t3_dn4 = assign97420_e149839_d_n4;
        locals.var_t3_dn5 = assign97420_e149839_d_n5;
        locals.var_t3_dn6 = assign97420_e149839_d_n6;
        locals.var_t3_dn7 = assign97420_e149839_d_n7;
        locals.var_t3_dn8 = assign97420_e149839_d_n8;
        locals.var_t3_dn9 = assign97420_e149839_d_n9;
        locals.var_t3_dn10 = assign97420_e149839_d_n10;
        locals.var_t3_dn11 = assign97420_e149839_d_n11;
        locals.var_t3_dn14 = assign97420_e149839_d_n14;
        locals.var_t3_rv = 0.0;

        let assign97430_e149842: f64 = if locals.var_vbd_jct < locals.var_vbdt { 1.0 } else { 0.0 };
        locals.var_guard2262 = assign97430_e149842;
        locals.var_guard2262_rv = 0.0;

        let (assign97440_e149850, assign97440_e149850_d_n0, assign97440_e149850_d_n2, assign97440_e149850_d_n4, assign97440_e149850_d_n5, assign97440_e149850_d_n6, assign97440_e149850_d_n7, assign97440_e149850_d_n8, assign97440_e149850_d_n9, assign97440_e149850_d_n10, assign97440_e149850_d_n11, assign97440_e149850_d_n14,) = {
    if ((locals.var_guard2261 != 0.0) && (locals.var_guard2262 != 0.0)) {
        let assign97440_e149848: f64 = (locals.var_vbd_jct * locals.var_jd_nvtm_invd);
        (assign97440_e149848, ((locals.var_vbd_jct_dn0 * locals.var_jd_nvtm_invd) + (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn0)), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn2), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn4), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn5), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn6), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn7), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn8), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn9), ((locals.var_vbd_jct_dn10 * locals.var_jd_nvtm_invd) + (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn10)), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn11), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn14),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign97440_e149850;
        locals.var_tx_dn0 = assign97440_e149850_d_n0;
        locals.var_tx_dn2 = assign97440_e149850_d_n2;
        locals.var_tx_dn4 = assign97440_e149850_d_n4;
        locals.var_tx_dn5 = assign97440_e149850_d_n5;
        locals.var_tx_dn6 = assign97440_e149850_d_n6;
        locals.var_tx_dn7 = assign97440_e149850_d_n7;
        locals.var_tx_dn8 = assign97440_e149850_d_n8;
        locals.var_tx_dn9 = assign97440_e149850_d_n9;
        locals.var_tx_dn10 = assign97440_e149850_d_n10;
        locals.var_tx_dn11 = assign97440_e149850_d_n11;
        locals.var_tx_dn14 = assign97440_e149850_d_n14;
        locals.var_tx_rv = 0.0;

        let assign97450_e149853: f64 = (-3.0);
        let assign97450_e149855: f64 = (assign97450_e149853 * 34.0);
        let assign97450_e149856: f64 = if locals.var_tx < assign97450_e149855 { 1.0 } else { 0.0 };
        locals.var_guard2263 = assign97450_e149856;
        locals.var_guard2263_rv = 0.0;

        let (assign97460_e149864, assign97460_e149864_d_n0, assign97460_e149864_d_n2, assign97460_e149864_d_n4, assign97460_e149864_d_n5, assign97460_e149864_d_n6, assign97460_e149864_d_n7, assign97460_e149864_d_n8, assign97460_e149864_d_n9, assign97460_e149864_d_n10, assign97460_e149864_d_n11, assign97460_e149864_d_n14,) = {
    if (((locals.var_guard2261 != 0.0) && (locals.var_guard2262 != 0.0)) && (locals.var_guard2263 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign97460_e149864;
        locals.var_t1_dn0 = assign97460_e149864_d_n0;
        locals.var_t1_dn2 = assign97460_e149864_d_n2;
        locals.var_t1_dn4 = assign97460_e149864_d_n4;
        locals.var_t1_dn5 = assign97460_e149864_d_n5;
        locals.var_t1_dn6 = assign97460_e149864_d_n6;
        locals.var_t1_dn7 = assign97460_e149864_d_n7;
        locals.var_t1_dn8 = assign97460_e149864_d_n8;
        locals.var_t1_dn9 = assign97460_e149864_d_n9;
        locals.var_t1_dn10 = assign97460_e149864_d_n10;
        locals.var_t1_dn11 = assign97460_e149864_d_n11;
        locals.var_t1_dn14 = assign97460_e149864_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign97470_e149874, assign97470_e149874_d_n0, assign97470_e149874_d_n2, assign97470_e149874_d_n4, assign97470_e149874_d_n5, assign97470_e149874_d_n6, assign97470_e149874_d_n7, assign97470_e149874_d_n8, assign97470_e149874_d_n9, assign97470_e149874_d_n10, assign97470_e149874_d_n11, assign97470_e149874_d_n14,) = {
    if (((locals.var_guard2261 != 0.0) && (locals.var_guard2262 != 0.0)) && (locals.var_guard2263 == 0.0)) {
        let assign97470_e149872: f64 = (locals.var_tx).exp();
        (assign97470_e149872, (assign97470_e149872 * locals.var_tx_dn0), (assign97470_e149872 * locals.var_tx_dn2), (assign97470_e149872 * locals.var_tx_dn4), (assign97470_e149872 * locals.var_tx_dn5), (assign97470_e149872 * locals.var_tx_dn6), (assign97470_e149872 * locals.var_tx_dn7), (assign97470_e149872 * locals.var_tx_dn8), (assign97470_e149872 * locals.var_tx_dn9), (assign97470_e149872 * locals.var_tx_dn10), (assign97470_e149872 * locals.var_tx_dn11), (assign97470_e149872 * locals.var_tx_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign97470_e149874;
        locals.var_t1_dn0 = assign97470_e149874_d_n0;
        locals.var_t1_dn2 = assign97470_e149874_d_n2;
        locals.var_t1_dn4 = assign97470_e149874_d_n4;
        locals.var_t1_dn5 = assign97470_e149874_d_n5;
        locals.var_t1_dn6 = assign97470_e149874_d_n6;
        locals.var_t1_dn7 = assign97470_e149874_d_n7;
        locals.var_t1_dn8 = assign97470_e149874_d_n8;
        locals.var_t1_dn9 = assign97470_e149874_d_n9;
        locals.var_t1_dn10 = assign97470_e149874_d_n10;
        locals.var_t1_dn11 = assign97470_e149874_d_n11;
        locals.var_t1_dn14 = assign97470_e149874_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign97490_e149903, assign97490_e149903_d_n0, assign97490_e149903_d_n2, assign97490_e149903_d_n4, assign97490_e149903_d_n5, assign97490_e149903_d_n6, assign97490_e149903_d_n7, assign97490_e149903_d_n8, assign97490_e149903_d_n9, assign97490_e149903_d_n10, assign97490_e149903_d_n11, assign97490_e149903_d_n14,) = {
    if ((locals.var_guard2261 != 0.0) && (locals.var_guard2262 == 0.0)) {
        (locals.var_jd_expcd, locals.var_jd_expcd_dn0, locals.var_jd_expcd_dn2, locals.var_jd_expcd_dn4, locals.var_jd_expcd_dn5, locals.var_jd_expcd_dn6, locals.var_jd_expcd_dn7, locals.var_jd_expcd_dn8, locals.var_jd_expcd_dn9, locals.var_jd_expcd_dn10, locals.var_jd_expcd_dn11, locals.var_jd_expcd_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign97490_e149903;
        locals.var_t1_dn0 = assign97490_e149903_d_n0;
        locals.var_t1_dn2 = assign97490_e149903_d_n2;
        locals.var_t1_dn4 = assign97490_e149903_d_n4;
        locals.var_t1_dn5 = assign97490_e149903_d_n5;
        locals.var_t1_dn6 = assign97490_e149903_d_n6;
        locals.var_t1_dn7 = assign97490_e149903_d_n7;
        locals.var_t1_dn8 = assign97490_e149903_d_n8;
        locals.var_t1_dn9 = assign97490_e149903_d_n9;
        locals.var_t1_dn10 = assign97490_e149903_d_n10;
        locals.var_t1_dn11 = assign97490_e149903_d_n11;
        locals.var_t1_dn14 = assign97490_e149903_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign97500_e149914, assign97500_e149914_d_n0, assign97500_e149914_d_n2, assign97500_e149914_d_n4, assign97500_e149914_d_n5, assign97500_e149914_d_n6, assign97500_e149914_d_n7, assign97500_e149914_d_n8, assign97500_e149914_d_n9, assign97500_e149914_d_n10, assign97500_e149914_d_n11, assign97500_e149914_d_n14,) = {
    if ((locals.var_guard2261 != 0.0) && (locals.var_guard2262 == 0.0)) {
        let assign97500_e149910: f64 = (locals.var_isbd_sws * locals.var_jd_nvtm_invd);
        let assign97500_e149912: f64 = (assign97500_e149910 * locals.var_t1);
        (assign97500_e149912, ((((locals.var_isbd_sws_dn0 * locals.var_jd_nvtm_invd) + (locals.var_isbd_sws * locals.var_jd_nvtm_invd_dn0)) * locals.var_t1) + (assign97500_e149910 * locals.var_t1_dn0)), ((((locals.var_isbd_sws_dn2 * locals.var_jd_nvtm_invd) + (locals.var_isbd_sws * locals.var_jd_nvtm_invd_dn2)) * locals.var_t1) + (assign97500_e149910 * locals.var_t1_dn2)), ((((locals.var_isbd_sws_dn4 * locals.var_jd_nvtm_invd) + (locals.var_isbd_sws * locals.var_jd_nvtm_invd_dn4)) * locals.var_t1) + (assign97500_e149910 * locals.var_t1_dn4)), ((((locals.var_isbd_sws_dn5 * locals.var_jd_nvtm_invd) + (locals.var_isbd_sws * locals.var_jd_nvtm_invd_dn5)) * locals.var_t1) + (assign97500_e149910 * locals.var_t1_dn5)), ((((locals.var_isbd_sws_dn6 * locals.var_jd_nvtm_invd) + (locals.var_isbd_sws * locals.var_jd_nvtm_invd_dn6)) * locals.var_t1) + (assign97500_e149910 * locals.var_t1_dn6)), ((((locals.var_isbd_sws_dn7 * locals.var_jd_nvtm_invd) + (locals.var_isbd_sws * locals.var_jd_nvtm_invd_dn7)) * locals.var_t1) + (assign97500_e149910 * locals.var_t1_dn7)), ((((locals.var_isbd_sws_dn8 * locals.var_jd_nvtm_invd) + (locals.var_isbd_sws * locals.var_jd_nvtm_invd_dn8)) * locals.var_t1) + (assign97500_e149910 * locals.var_t1_dn8)), ((((locals.var_isbd_sws_dn9 * locals.var_jd_nvtm_invd) + (locals.var_isbd_sws * locals.var_jd_nvtm_invd_dn9)) * locals.var_t1) + (assign97500_e149910 * locals.var_t1_dn9)), ((((locals.var_isbd_sws_dn10 * locals.var_jd_nvtm_invd) + (locals.var_isbd_sws * locals.var_jd_nvtm_invd_dn10)) * locals.var_t1) + (assign97500_e149910 * locals.var_t1_dn10)), ((((locals.var_isbd_sws_dn11 * locals.var_jd_nvtm_invd) + (locals.var_isbd_sws * locals.var_jd_nvtm_invd_dn11)) * locals.var_t1) + (assign97500_e149910 * locals.var_t1_dn11)), ((((locals.var_isbd_sws_dn14 * locals.var_jd_nvtm_invd) + (locals.var_isbd_sws * locals.var_jd_nvtm_invd_dn14)) * locals.var_t1) + (assign97500_e149910 * locals.var_t1_dn14)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign97500_e149914;
        locals.var_t4_dn0 = assign97500_e149914_d_n0;
        locals.var_t4_dn2 = assign97500_e149914_d_n2;
        locals.var_t4_dn4 = assign97500_e149914_d_n4;
        locals.var_t4_dn5 = assign97500_e149914_d_n5;
        locals.var_t4_dn6 = assign97500_e149914_d_n6;
        locals.var_t4_dn7 = assign97500_e149914_d_n7;
        locals.var_t4_dn8 = assign97500_e149914_d_n8;
        locals.var_t4_dn9 = assign97500_e149914_d_n9;
        locals.var_t4_dn10 = assign97500_e149914_d_n10;
        locals.var_t4_dn11 = assign97500_e149914_d_n11;
        locals.var_t4_dn14 = assign97500_e149914_d_n14;
        locals.var_t4_rv = 0.0;

        let assign97530_e149951: f64 = (p.p514 * locals.var_isbd2_sws);
        locals.var_t12 = assign97530_e149951;
        locals.var_t12_dn0 = (p.p514 * locals.var_isbd2_sws_dn0);
        locals.var_t12_dn2 = (p.p514 * locals.var_isbd2_sws_dn2);
        locals.var_t12_dn4 = (p.p514 * locals.var_isbd2_sws_dn4);
        locals.var_t12_dn5 = (p.p514 * locals.var_isbd2_sws_dn5);
        locals.var_t12_dn6 = (p.p514 * locals.var_isbd2_sws_dn6);
        locals.var_t12_dn7 = (p.p514 * locals.var_isbd2_sws_dn7);
        locals.var_t12_dn8 = (p.p514 * locals.var_isbd2_sws_dn8);
        locals.var_t12_dn9 = (p.p514 * locals.var_isbd2_sws_dn9);
        locals.var_t12_dn10 = (p.p514 * locals.var_isbd2_sws_dn10);
        locals.var_t12_dn11 = (p.p514 * locals.var_isbd2_sws_dn11);
        locals.var_t12_dn14 = (p.p514 * locals.var_isbd2_sws_dn14);
        locals.var_t12_rv = 0.0;

        let assign97550_e149959: f64 = if p.p48 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2264 = assign97550_e149959;
        locals.var_guard2264_rv = 0.0;

        let assign97560_e149962: f64 = if locals.var_isbd_swg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2265 = assign97560_e149962;
        locals.var_guard2265_rv = 0.0;

        let (assign97570_e149970, assign97570_e149970_d_n0, assign97570_e149970_d_n2, assign97570_e149970_d_n4, assign97570_e149970_d_n5, assign97570_e149970_d_n6, assign97570_e149970_d_n7, assign97570_e149970_d_n8, assign97570_e149970_d_n9, assign97570_e149970_d_n10, assign97570_e149970_d_n11, assign97570_e149970_d_n14,) = {
    if ((locals.var_guard2264 != 0.0) && (locals.var_guard2265 != 0.0)) {
        let assign97570_e149968: f64 = (locals.var_isbd2_swg * locals.var_t9);
        (assign97570_e149968, ((locals.var_isbd2_swg_dn0 * locals.var_t9) + (locals.var_isbd2_swg * locals.var_t9_dn0)), ((locals.var_isbd2_swg_dn2 * locals.var_t9) + (locals.var_isbd2_swg * locals.var_t9_dn2)), ((locals.var_isbd2_swg_dn4 * locals.var_t9) + (locals.var_isbd2_swg * locals.var_t9_dn4)), ((locals.var_isbd2_swg_dn5 * locals.var_t9) + (locals.var_isbd2_swg * locals.var_t9_dn5)), ((locals.var_isbd2_swg_dn6 * locals.var_t9) + (locals.var_isbd2_swg * locals.var_t9_dn6)), ((locals.var_isbd2_swg_dn7 * locals.var_t9) + (locals.var_isbd2_swg * locals.var_t9_dn7)), ((locals.var_isbd2_swg_dn8 * locals.var_t9) + (locals.var_isbd2_swg * locals.var_t9_dn8)), ((locals.var_isbd2_swg_dn9 * locals.var_t9) + (locals.var_isbd2_swg * locals.var_t9_dn9)), ((locals.var_isbd2_swg_dn10 * locals.var_t9) + (locals.var_isbd2_swg * locals.var_t9_dn10)), ((locals.var_isbd2_swg_dn11 * locals.var_t9) + (locals.var_isbd2_swg * locals.var_t9_dn11)), ((locals.var_isbd2_swg_dn14 * locals.var_t9) + (locals.var_isbd2_swg * locals.var_t9_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign97570_e149970;
        locals.var_t0_dn0 = assign97570_e149970_d_n0;
        locals.var_t0_dn2 = assign97570_e149970_d_n2;
        locals.var_t0_dn4 = assign97570_e149970_d_n4;
        locals.var_t0_dn5 = assign97570_e149970_d_n5;
        locals.var_t0_dn6 = assign97570_e149970_d_n6;
        locals.var_t0_dn7 = assign97570_e149970_d_n7;
        locals.var_t0_dn8 = assign97570_e149970_d_n8;
        locals.var_t0_dn9 = assign97570_e149970_d_n9;
        locals.var_t0_dn10 = assign97570_e149970_d_n10;
        locals.var_t0_dn11 = assign97570_e149970_d_n11;
        locals.var_t0_dn14 = assign97570_e149970_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign97580_e149979, assign97580_e149979_d_n0, assign97580_e149979_d_n2, assign97580_e149979_d_n4, assign97580_e149979_d_n5, assign97580_e149979_d_n6, assign97580_e149979_d_n7, assign97580_e149979_d_n8, assign97580_e149979_d_n9, assign97580_e149979_d_n10, assign97580_e149979_d_n11, assign97580_e149979_d_n14,) = {
    if ((locals.var_guard2264 != 0.0) && (locals.var_guard2265 != 0.0)) {
        let assign97580_e149975: f64 = (-locals.var_vbdi_jct);
        let assign97580_e149977: f64 = (assign97580_e149975 * locals.var_t10);
        (assign97580_e149977, (assign97580_e149975 * locals.var_t10_dn0), (assign97580_e149975 * locals.var_t10_dn2), (assign97580_e149975 * locals.var_t10_dn4), (assign97580_e149975 * locals.var_t10_dn5), (((-locals.var_vbdi_jct_dn6) * locals.var_t10) + (assign97580_e149975 * locals.var_t10_dn6)), (assign97580_e149975 * locals.var_t10_dn7), (assign97580_e149975 * locals.var_t10_dn8), (((-locals.var_vbdi_jct_dn9) * locals.var_t10) + (assign97580_e149975 * locals.var_t10_dn9)), (assign97580_e149975 * locals.var_t10_dn10), (assign97580_e149975 * locals.var_t10_dn11), (assign97580_e149975 * locals.var_t10_dn14),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign97580_e149979;
        locals.var_tx_dn0 = assign97580_e149979_d_n0;
        locals.var_tx_dn2 = assign97580_e149979_d_n2;
        locals.var_tx_dn4 = assign97580_e149979_d_n4;
        locals.var_tx_dn5 = assign97580_e149979_d_n5;
        locals.var_tx_dn6 = assign97580_e149979_d_n6;
        locals.var_tx_dn7 = assign97580_e149979_d_n7;
        locals.var_tx_dn8 = assign97580_e149979_d_n8;
        locals.var_tx_dn9 = assign97580_e149979_d_n9;
        locals.var_tx_dn10 = assign97580_e149979_d_n10;
        locals.var_tx_dn11 = assign97580_e149979_d_n11;
        locals.var_tx_dn14 = assign97580_e149979_d_n14;
        locals.var_tx_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_378(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign97590_e149986, assign97590_e149986_d_n0, assign97590_e149986_d_n2, assign97590_e149986_d_n4, assign97590_e149986_d_n5, assign97590_e149986_d_n6, assign97590_e149986_d_n7, assign97590_e149986_d_n8, assign97590_e149986_d_n9, assign97590_e149986_d_n10, assign97590_e149986_d_n11, assign97590_e149986_d_n14,) = {
    if ((locals.var_guard2264 != 0.0) && (locals.var_guard2265 != 0.0)) {
        let assign97590_e149984: f64 = (locals.var_tx).exp();
        (assign97590_e149984, (assign97590_e149984 * locals.var_tx_dn0), (assign97590_e149984 * locals.var_tx_dn2), (assign97590_e149984 * locals.var_tx_dn4), (assign97590_e149984 * locals.var_tx_dn5), (assign97590_e149984 * locals.var_tx_dn6), (assign97590_e149984 * locals.var_tx_dn7), (assign97590_e149984 * locals.var_tx_dn8), (assign97590_e149984 * locals.var_tx_dn9), (assign97590_e149984 * locals.var_tx_dn10), (assign97590_e149984 * locals.var_tx_dn11), (assign97590_e149984 * locals.var_tx_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign97590_e149986;
        locals.var_t2_dn0 = assign97590_e149986_d_n0;
        locals.var_t2_dn2 = assign97590_e149986_d_n2;
        locals.var_t2_dn4 = assign97590_e149986_d_n4;
        locals.var_t2_dn5 = assign97590_e149986_d_n5;
        locals.var_t2_dn6 = assign97590_e149986_d_n6;
        locals.var_t2_dn7 = assign97590_e149986_d_n7;
        locals.var_t2_dn8 = assign97590_e149986_d_n8;
        locals.var_t2_dn9 = assign97590_e149986_d_n9;
        locals.var_t2_dn10 = assign97590_e149986_d_n10;
        locals.var_t2_dn11 = assign97590_e149986_d_n11;
        locals.var_t2_dn14 = assign97590_e149986_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign97600_e149992, assign97600_e149992_d_n0, assign97600_e149992_d_n2, assign97600_e149992_d_n4, assign97600_e149992_d_n5, assign97600_e149992_d_n6, assign97600_e149992_d_n7, assign97600_e149992_d_n8, assign97600_e149992_d_n9, assign97600_e149992_d_n10, assign97600_e149992_d_n11, assign97600_e149992_d_n14,) = {
    if ((locals.var_guard2264 != 0.0) && (locals.var_guard2265 != 0.0)) {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign97600_e149992;
        locals.var_t3_dn0 = assign97600_e149992_d_n0;
        locals.var_t3_dn2 = assign97600_e149992_d_n2;
        locals.var_t3_dn4 = assign97600_e149992_d_n4;
        locals.var_t3_dn5 = assign97600_e149992_d_n5;
        locals.var_t3_dn6 = assign97600_e149992_d_n6;
        locals.var_t3_dn7 = assign97600_e149992_d_n7;
        locals.var_t3_dn8 = assign97600_e149992_d_n8;
        locals.var_t3_dn9 = assign97600_e149992_d_n9;
        locals.var_t3_dn10 = assign97600_e149992_d_n10;
        locals.var_t3_dn11 = assign97600_e149992_d_n11;
        locals.var_t3_dn14 = assign97600_e149992_d_n14;
        locals.var_t3_rv = 0.0;

        let assign97610_e149995: f64 = if locals.var_vbdi_jct < locals.var_vbdt { 1.0 } else { 0.0 };
        locals.var_guard2266 = assign97610_e149995;
        locals.var_guard2266_rv = 0.0;

        let (assign97620_e150005, assign97620_e150005_d_n0, assign97620_e150005_d_n2, assign97620_e150005_d_n4, assign97620_e150005_d_n5, assign97620_e150005_d_n6, assign97620_e150005_d_n7, assign97620_e150005_d_n8, assign97620_e150005_d_n9, assign97620_e150005_d_n10, assign97620_e150005_d_n11, assign97620_e150005_d_n14,) = {
    if (((locals.var_guard2264 != 0.0) && (locals.var_guard2265 != 0.0)) && (locals.var_guard2266 != 0.0)) {
        let assign97620_e150003: f64 = (locals.var_vbdi_jct * locals.var_jd_nvtm_invd);
        (assign97620_e150003, (locals.var_vbdi_jct * locals.var_jd_nvtm_invd_dn0), (locals.var_vbdi_jct * locals.var_jd_nvtm_invd_dn2), (locals.var_vbdi_jct * locals.var_jd_nvtm_invd_dn4), (locals.var_vbdi_jct * locals.var_jd_nvtm_invd_dn5), ((locals.var_vbdi_jct_dn6 * locals.var_jd_nvtm_invd) + (locals.var_vbdi_jct * locals.var_jd_nvtm_invd_dn6)), (locals.var_vbdi_jct * locals.var_jd_nvtm_invd_dn7), (locals.var_vbdi_jct * locals.var_jd_nvtm_invd_dn8), ((locals.var_vbdi_jct_dn9 * locals.var_jd_nvtm_invd) + (locals.var_vbdi_jct * locals.var_jd_nvtm_invd_dn9)), (locals.var_vbdi_jct * locals.var_jd_nvtm_invd_dn10), (locals.var_vbdi_jct * locals.var_jd_nvtm_invd_dn11), (locals.var_vbdi_jct * locals.var_jd_nvtm_invd_dn14),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign97620_e150005;
        locals.var_tx_dn0 = assign97620_e150005_d_n0;
        locals.var_tx_dn2 = assign97620_e150005_d_n2;
        locals.var_tx_dn4 = assign97620_e150005_d_n4;
        locals.var_tx_dn5 = assign97620_e150005_d_n5;
        locals.var_tx_dn6 = assign97620_e150005_d_n6;
        locals.var_tx_dn7 = assign97620_e150005_d_n7;
        locals.var_tx_dn8 = assign97620_e150005_d_n8;
        locals.var_tx_dn9 = assign97620_e150005_d_n9;
        locals.var_tx_dn10 = assign97620_e150005_d_n10;
        locals.var_tx_dn11 = assign97620_e150005_d_n11;
        locals.var_tx_dn14 = assign97620_e150005_d_n14;
        locals.var_tx_rv = 0.0;

        let assign97630_e150008: f64 = (-3.0);
        let assign97630_e150010: f64 = (assign97630_e150008 * 34.0);
        let assign97630_e150011: f64 = if locals.var_tx < assign97630_e150010 { 1.0 } else { 0.0 };
        locals.var_guard2267 = assign97630_e150011;
        locals.var_guard2267_rv = 0.0;

        let (assign97640_e150021, assign97640_e150021_d_n0, assign97640_e150021_d_n2, assign97640_e150021_d_n4, assign97640_e150021_d_n5, assign97640_e150021_d_n6, assign97640_e150021_d_n7, assign97640_e150021_d_n8, assign97640_e150021_d_n9, assign97640_e150021_d_n10, assign97640_e150021_d_n11, assign97640_e150021_d_n14,) = {
    if ((((locals.var_guard2264 != 0.0) && (locals.var_guard2265 != 0.0)) && (locals.var_guard2266 != 0.0)) && (locals.var_guard2267 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign97640_e150021;
        locals.var_t1_dn0 = assign97640_e150021_d_n0;
        locals.var_t1_dn2 = assign97640_e150021_d_n2;
        locals.var_t1_dn4 = assign97640_e150021_d_n4;
        locals.var_t1_dn5 = assign97640_e150021_d_n5;
        locals.var_t1_dn6 = assign97640_e150021_d_n6;
        locals.var_t1_dn7 = assign97640_e150021_d_n7;
        locals.var_t1_dn8 = assign97640_e150021_d_n8;
        locals.var_t1_dn9 = assign97640_e150021_d_n9;
        locals.var_t1_dn10 = assign97640_e150021_d_n10;
        locals.var_t1_dn11 = assign97640_e150021_d_n11;
        locals.var_t1_dn14 = assign97640_e150021_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign97650_e150033, assign97650_e150033_d_n0, assign97650_e150033_d_n2, assign97650_e150033_d_n4, assign97650_e150033_d_n5, assign97650_e150033_d_n6, assign97650_e150033_d_n7, assign97650_e150033_d_n8, assign97650_e150033_d_n9, assign97650_e150033_d_n10, assign97650_e150033_d_n11, assign97650_e150033_d_n14,) = {
    if ((((locals.var_guard2264 != 0.0) && (locals.var_guard2265 != 0.0)) && (locals.var_guard2266 != 0.0)) && (locals.var_guard2267 == 0.0)) {
        let assign97650_e150031: f64 = (locals.var_tx).exp();
        (assign97650_e150031, (assign97650_e150031 * locals.var_tx_dn0), (assign97650_e150031 * locals.var_tx_dn2), (assign97650_e150031 * locals.var_tx_dn4), (assign97650_e150031 * locals.var_tx_dn5), (assign97650_e150031 * locals.var_tx_dn6), (assign97650_e150031 * locals.var_tx_dn7), (assign97650_e150031 * locals.var_tx_dn8), (assign97650_e150031 * locals.var_tx_dn9), (assign97650_e150031 * locals.var_tx_dn10), (assign97650_e150031 * locals.var_tx_dn11), (assign97650_e150031 * locals.var_tx_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign97650_e150033;
        locals.var_t1_dn0 = assign97650_e150033_d_n0;
        locals.var_t1_dn2 = assign97650_e150033_d_n2;
        locals.var_t1_dn4 = assign97650_e150033_d_n4;
        locals.var_t1_dn5 = assign97650_e150033_d_n5;
        locals.var_t1_dn6 = assign97650_e150033_d_n6;
        locals.var_t1_dn7 = assign97650_e150033_d_n7;
        locals.var_t1_dn8 = assign97650_e150033_d_n8;
        locals.var_t1_dn9 = assign97650_e150033_d_n9;
        locals.var_t1_dn10 = assign97650_e150033_d_n10;
        locals.var_t1_dn11 = assign97650_e150033_d_n11;
        locals.var_t1_dn14 = assign97650_e150033_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign97670_e150066, assign97670_e150066_d_n0, assign97670_e150066_d_n2, assign97670_e150066_d_n4, assign97670_e150066_d_n5, assign97670_e150066_d_n6, assign97670_e150066_d_n7, assign97670_e150066_d_n8, assign97670_e150066_d_n9, assign97670_e150066_d_n10, assign97670_e150066_d_n11, assign97670_e150066_d_n14,) = {
    if (((locals.var_guard2264 != 0.0) && (locals.var_guard2265 != 0.0)) && (locals.var_guard2266 == 0.0)) {
        (locals.var_jd_expcd, locals.var_jd_expcd_dn0, locals.var_jd_expcd_dn2, locals.var_jd_expcd_dn4, locals.var_jd_expcd_dn5, locals.var_jd_expcd_dn6, locals.var_jd_expcd_dn7, locals.var_jd_expcd_dn8, locals.var_jd_expcd_dn9, locals.var_jd_expcd_dn10, locals.var_jd_expcd_dn11, locals.var_jd_expcd_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign97670_e150066;
        locals.var_t1_dn0 = assign97670_e150066_d_n0;
        locals.var_t1_dn2 = assign97670_e150066_d_n2;
        locals.var_t1_dn4 = assign97670_e150066_d_n4;
        locals.var_t1_dn5 = assign97670_e150066_d_n5;
        locals.var_t1_dn6 = assign97670_e150066_d_n6;
        locals.var_t1_dn7 = assign97670_e150066_d_n7;
        locals.var_t1_dn8 = assign97670_e150066_d_n8;
        locals.var_t1_dn9 = assign97670_e150066_d_n9;
        locals.var_t1_dn10 = assign97670_e150066_d_n10;
        locals.var_t1_dn11 = assign97670_e150066_d_n11;
        locals.var_t1_dn14 = assign97670_e150066_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign97680_e150079, assign97680_e150079_d_n0, assign97680_e150079_d_n2, assign97680_e150079_d_n4, assign97680_e150079_d_n5, assign97680_e150079_d_n6, assign97680_e150079_d_n7, assign97680_e150079_d_n8, assign97680_e150079_d_n9, assign97680_e150079_d_n10, assign97680_e150079_d_n11, assign97680_e150079_d_n14,) = {
    if (((locals.var_guard2264 != 0.0) && (locals.var_guard2265 != 0.0)) && (locals.var_guard2266 == 0.0)) {
        let assign97680_e150075: f64 = (locals.var_isbd_swg * locals.var_jd_nvtm_invd);
        let assign97680_e150077: f64 = (assign97680_e150075 * locals.var_t1);
        (assign97680_e150077, ((((locals.var_isbd_swg_dn0 * locals.var_jd_nvtm_invd) + (locals.var_isbd_swg * locals.var_jd_nvtm_invd_dn0)) * locals.var_t1) + (assign97680_e150075 * locals.var_t1_dn0)), ((((locals.var_isbd_swg_dn2 * locals.var_jd_nvtm_invd) + (locals.var_isbd_swg * locals.var_jd_nvtm_invd_dn2)) * locals.var_t1) + (assign97680_e150075 * locals.var_t1_dn2)), ((((locals.var_isbd_swg_dn4 * locals.var_jd_nvtm_invd) + (locals.var_isbd_swg * locals.var_jd_nvtm_invd_dn4)) * locals.var_t1) + (assign97680_e150075 * locals.var_t1_dn4)), ((((locals.var_isbd_swg_dn5 * locals.var_jd_nvtm_invd) + (locals.var_isbd_swg * locals.var_jd_nvtm_invd_dn5)) * locals.var_t1) + (assign97680_e150075 * locals.var_t1_dn5)), ((((locals.var_isbd_swg_dn6 * locals.var_jd_nvtm_invd) + (locals.var_isbd_swg * locals.var_jd_nvtm_invd_dn6)) * locals.var_t1) + (assign97680_e150075 * locals.var_t1_dn6)), ((((locals.var_isbd_swg_dn7 * locals.var_jd_nvtm_invd) + (locals.var_isbd_swg * locals.var_jd_nvtm_invd_dn7)) * locals.var_t1) + (assign97680_e150075 * locals.var_t1_dn7)), ((((locals.var_isbd_swg_dn8 * locals.var_jd_nvtm_invd) + (locals.var_isbd_swg * locals.var_jd_nvtm_invd_dn8)) * locals.var_t1) + (assign97680_e150075 * locals.var_t1_dn8)), ((((locals.var_isbd_swg_dn9 * locals.var_jd_nvtm_invd) + (locals.var_isbd_swg * locals.var_jd_nvtm_invd_dn9)) * locals.var_t1) + (assign97680_e150075 * locals.var_t1_dn9)), ((((locals.var_isbd_swg_dn10 * locals.var_jd_nvtm_invd) + (locals.var_isbd_swg * locals.var_jd_nvtm_invd_dn10)) * locals.var_t1) + (assign97680_e150075 * locals.var_t1_dn10)), ((((locals.var_isbd_swg_dn11 * locals.var_jd_nvtm_invd) + (locals.var_isbd_swg * locals.var_jd_nvtm_invd_dn11)) * locals.var_t1) + (assign97680_e150075 * locals.var_t1_dn11)), ((((locals.var_isbd_swg_dn14 * locals.var_jd_nvtm_invd) + (locals.var_isbd_swg * locals.var_jd_nvtm_invd_dn14)) * locals.var_t1) + (assign97680_e150075 * locals.var_t1_dn14)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign97680_e150079;
        locals.var_t4_dn0 = assign97680_e150079_d_n0;
        locals.var_t4_dn2 = assign97680_e150079_d_n2;
        locals.var_t4_dn4 = assign97680_e150079_d_n4;
        locals.var_t4_dn5 = assign97680_e150079_d_n5;
        locals.var_t4_dn6 = assign97680_e150079_d_n6;
        locals.var_t4_dn7 = assign97680_e150079_d_n7;
        locals.var_t4_dn8 = assign97680_e150079_d_n8;
        locals.var_t4_dn9 = assign97680_e150079_d_n9;
        locals.var_t4_dn10 = assign97680_e150079_d_n10;
        locals.var_t4_dn11 = assign97680_e150079_d_n11;
        locals.var_t4_dn14 = assign97680_e150079_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign97710_e150123, assign97710_e150123_d_n0, assign97710_e150123_d_n2, assign97710_e150123_d_n4, assign97710_e150123_d_n5, assign97710_e150123_d_n6, assign97710_e150123_d_n7, assign97710_e150123_d_n8, assign97710_e150123_d_n9, assign97710_e150123_d_n10, assign97710_e150123_d_n11, assign97710_e150123_d_n14,) = {
    if (locals.var_guard2264 != 0.0) {
        let assign97710_e150121: f64 = (p.p514 * locals.var_isbd2_swg);
        (assign97710_e150121, (p.p514 * locals.var_isbd2_swg_dn0), (p.p514 * locals.var_isbd2_swg_dn2), (p.p514 * locals.var_isbd2_swg_dn4), (p.p514 * locals.var_isbd2_swg_dn5), (p.p514 * locals.var_isbd2_swg_dn6), (p.p514 * locals.var_isbd2_swg_dn7), (p.p514 * locals.var_isbd2_swg_dn8), (p.p514 * locals.var_isbd2_swg_dn9), (p.p514 * locals.var_isbd2_swg_dn10), (p.p514 * locals.var_isbd2_swg_dn11), (p.p514 * locals.var_isbd2_swg_dn14),)
    } else {
        (locals.var_t12, locals.var_t12_dn0, locals.var_t12_dn2, locals.var_t12_dn4, locals.var_t12_dn5, locals.var_t12_dn6, locals.var_t12_dn7, locals.var_t12_dn8, locals.var_t12_dn9, locals.var_t12_dn10, locals.var_t12_dn11, locals.var_t12_dn14,)
    }
};
        locals.var_t12 = assign97710_e150123;
        locals.var_t12_dn0 = assign97710_e150123_d_n0;
        locals.var_t12_dn2 = assign97710_e150123_d_n2;
        locals.var_t12_dn4 = assign97710_e150123_d_n4;
        locals.var_t12_dn5 = assign97710_e150123_d_n5;
        locals.var_t12_dn6 = assign97710_e150123_d_n6;
        locals.var_t12_dn7 = assign97710_e150123_d_n7;
        locals.var_t12_dn8 = assign97710_e150123_d_n8;
        locals.var_t12_dn9 = assign97710_e150123_d_n9;
        locals.var_t12_dn10 = assign97710_e150123_d_n10;
        locals.var_t12_dn11 = assign97710_e150123_d_n11;
        locals.var_t12_dn14 = assign97710_e150123_d_n14;
        locals.var_t12_rv = 0.0;

        let assign97740_e150139: f64 = (p.p534 * locals.var_jd_nvtm_invs);
        locals.var_t10 = assign97740_e150139;
        locals.var_t10_dn0 = (p.p534 * locals.var_jd_nvtm_invs_dn0);
        locals.var_t10_dn2 = (p.p534 * locals.var_jd_nvtm_invs_dn2);
        locals.var_t10_dn4 = (p.p534 * locals.var_jd_nvtm_invs_dn4);
        locals.var_t10_dn5 = (p.p534 * locals.var_jd_nvtm_invs_dn5);
        locals.var_t10_dn6 = (p.p534 * locals.var_jd_nvtm_invs_dn6);
        locals.var_t10_dn7 = (p.p534 * locals.var_jd_nvtm_invs_dn7);
        locals.var_t10_dn8 = (p.p534 * locals.var_jd_nvtm_invs_dn8);
        locals.var_t10_dn9 = (p.p534 * locals.var_jd_nvtm_invs_dn9);
        locals.var_t10_dn10 = (p.p534 * locals.var_jd_nvtm_invs_dn10);
        locals.var_t10_dn11 = (p.p534 * locals.var_jd_nvtm_invs_dn11);
        locals.var_t10_dn14 = (p.p534 * locals.var_jd_nvtm_invs_dn14);
        locals.var_t10_rv = 0.0;

        let assign97750_e150142: f64 = (p.p533 * locals.var_exptemps);
        locals.var_t9 = assign97750_e150142;
        locals.var_t9_dn0 = (p.p533 * locals.var_exptemps_dn0);
        locals.var_t9_dn2 = (p.p533 * locals.var_exptemps_dn2);
        locals.var_t9_dn4 = (p.p533 * locals.var_exptemps_dn4);
        locals.var_t9_dn5 = (p.p533 * locals.var_exptemps_dn5);
        locals.var_t9_dn6 = (p.p533 * locals.var_exptemps_dn6);
        locals.var_t9_dn7 = (p.p533 * locals.var_exptemps_dn7);
        locals.var_t9_dn8 = (p.p533 * locals.var_exptemps_dn8);
        locals.var_t9_dn9 = (p.p533 * locals.var_exptemps_dn9);
        locals.var_t9_dn10 = (p.p533 * locals.var_exptemps_dn10);
        locals.var_t9_dn11 = (p.p533 * locals.var_exptemps_dn11);
        locals.var_t9_dn14 = (p.p533 * locals.var_exptemps_dn14);
        locals.var_t9_rv = 0.0;

        let assign97760_e150145: f64 = if locals.var_isbs_btm > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2268 = assign97760_e150145;
        locals.var_guard2268_rv = 0.0;

        let (assign97770_e150151, assign97770_e150151_d_n0, assign97770_e150151_d_n2, assign97770_e150151_d_n4, assign97770_e150151_d_n5, assign97770_e150151_d_n6, assign97770_e150151_d_n7, assign97770_e150151_d_n8, assign97770_e150151_d_n9, assign97770_e150151_d_n10, assign97770_e150151_d_n11, assign97770_e150151_d_n14,) = {
    if (locals.var_guard2268 != 0.0) {
        let assign97770_e150149: f64 = (locals.var_isbs2_btm * locals.var_t9);
        (assign97770_e150149, ((locals.var_isbs2_btm_dn0 * locals.var_t9) + (locals.var_isbs2_btm * locals.var_t9_dn0)), ((locals.var_isbs2_btm_dn2 * locals.var_t9) + (locals.var_isbs2_btm * locals.var_t9_dn2)), ((locals.var_isbs2_btm_dn4 * locals.var_t9) + (locals.var_isbs2_btm * locals.var_t9_dn4)), ((locals.var_isbs2_btm_dn5 * locals.var_t9) + (locals.var_isbs2_btm * locals.var_t9_dn5)), ((locals.var_isbs2_btm_dn6 * locals.var_t9) + (locals.var_isbs2_btm * locals.var_t9_dn6)), ((locals.var_isbs2_btm_dn7 * locals.var_t9) + (locals.var_isbs2_btm * locals.var_t9_dn7)), ((locals.var_isbs2_btm_dn8 * locals.var_t9) + (locals.var_isbs2_btm * locals.var_t9_dn8)), ((locals.var_isbs2_btm_dn9 * locals.var_t9) + (locals.var_isbs2_btm * locals.var_t9_dn9)), ((locals.var_isbs2_btm_dn10 * locals.var_t9) + (locals.var_isbs2_btm * locals.var_t9_dn10)), ((locals.var_isbs2_btm_dn11 * locals.var_t9) + (locals.var_isbs2_btm * locals.var_t9_dn11)), ((locals.var_isbs2_btm_dn14 * locals.var_t9) + (locals.var_isbs2_btm * locals.var_t9_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign97770_e150151;
        locals.var_t0_dn0 = assign97770_e150151_d_n0;
        locals.var_t0_dn2 = assign97770_e150151_d_n2;
        locals.var_t0_dn4 = assign97770_e150151_d_n4;
        locals.var_t0_dn5 = assign97770_e150151_d_n5;
        locals.var_t0_dn6 = assign97770_e150151_d_n6;
        locals.var_t0_dn7 = assign97770_e150151_d_n7;
        locals.var_t0_dn8 = assign97770_e150151_d_n8;
        locals.var_t0_dn9 = assign97770_e150151_d_n9;
        locals.var_t0_dn10 = assign97770_e150151_d_n10;
        locals.var_t0_dn11 = assign97770_e150151_d_n11;
        locals.var_t0_dn14 = assign97770_e150151_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign97780_e150158, assign97780_e150158_d_n0, assign97780_e150158_d_n2, assign97780_e150158_d_n4, assign97780_e150158_d_n5, assign97780_e150158_d_n6, assign97780_e150158_d_n7, assign97780_e150158_d_n8, assign97780_e150158_d_n9, assign97780_e150158_d_n10, assign97780_e150158_d_n11, assign97780_e150158_d_n14,) = {
    if (locals.var_guard2268 != 0.0) {
        let assign97780_e150154: f64 = (-locals.var_vbs_jct);
        let assign97780_e150156: f64 = (assign97780_e150154 * locals.var_t10);
        (assign97780_e150156, (assign97780_e150154 * locals.var_t10_dn0), (((-locals.var_vbs_jct_dn2) * locals.var_t10) + (assign97780_e150154 * locals.var_t10_dn2)), (assign97780_e150154 * locals.var_t10_dn4), (assign97780_e150154 * locals.var_t10_dn5), (assign97780_e150154 * locals.var_t10_dn6), (assign97780_e150154 * locals.var_t10_dn7), (assign97780_e150154 * locals.var_t10_dn8), (assign97780_e150154 * locals.var_t10_dn9), (assign97780_e150154 * locals.var_t10_dn10), (((-locals.var_vbs_jct_dn11) * locals.var_t10) + (assign97780_e150154 * locals.var_t10_dn11)), (assign97780_e150154 * locals.var_t10_dn14),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign97780_e150158;
        locals.var_tx_dn0 = assign97780_e150158_d_n0;
        locals.var_tx_dn2 = assign97780_e150158_d_n2;
        locals.var_tx_dn4 = assign97780_e150158_d_n4;
        locals.var_tx_dn5 = assign97780_e150158_d_n5;
        locals.var_tx_dn6 = assign97780_e150158_d_n6;
        locals.var_tx_dn7 = assign97780_e150158_d_n7;
        locals.var_tx_dn8 = assign97780_e150158_d_n8;
        locals.var_tx_dn9 = assign97780_e150158_d_n9;
        locals.var_tx_dn10 = assign97780_e150158_d_n10;
        locals.var_tx_dn11 = assign97780_e150158_d_n11;
        locals.var_tx_dn14 = assign97780_e150158_d_n14;
        locals.var_tx_rv = 0.0;

        let (assign97790_e150163, assign97790_e150163_d_n0, assign97790_e150163_d_n2, assign97790_e150163_d_n4, assign97790_e150163_d_n5, assign97790_e150163_d_n6, assign97790_e150163_d_n7, assign97790_e150163_d_n8, assign97790_e150163_d_n9, assign97790_e150163_d_n10, assign97790_e150163_d_n11, assign97790_e150163_d_n14,) = {
    if (locals.var_guard2268 != 0.0) {
        let assign97790_e150161: f64 = (locals.var_tx).exp();
        (assign97790_e150161, (assign97790_e150161 * locals.var_tx_dn0), (assign97790_e150161 * locals.var_tx_dn2), (assign97790_e150161 * locals.var_tx_dn4), (assign97790_e150161 * locals.var_tx_dn5), (assign97790_e150161 * locals.var_tx_dn6), (assign97790_e150161 * locals.var_tx_dn7), (assign97790_e150161 * locals.var_tx_dn8), (assign97790_e150161 * locals.var_tx_dn9), (assign97790_e150161 * locals.var_tx_dn10), (assign97790_e150161 * locals.var_tx_dn11), (assign97790_e150161 * locals.var_tx_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign97790_e150163;
        locals.var_t2_dn0 = assign97790_e150163_d_n0;
        locals.var_t2_dn2 = assign97790_e150163_d_n2;
        locals.var_t2_dn4 = assign97790_e150163_d_n4;
        locals.var_t2_dn5 = assign97790_e150163_d_n5;
        locals.var_t2_dn6 = assign97790_e150163_d_n6;
        locals.var_t2_dn7 = assign97790_e150163_d_n7;
        locals.var_t2_dn8 = assign97790_e150163_d_n8;
        locals.var_t2_dn9 = assign97790_e150163_d_n9;
        locals.var_t2_dn10 = assign97790_e150163_d_n10;
        locals.var_t2_dn11 = assign97790_e150163_d_n11;
        locals.var_t2_dn14 = assign97790_e150163_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign97800_e150167, assign97800_e150167_d_n0, assign97800_e150167_d_n2, assign97800_e150167_d_n4, assign97800_e150167_d_n5, assign97800_e150167_d_n6, assign97800_e150167_d_n7, assign97800_e150167_d_n8, assign97800_e150167_d_n9, assign97800_e150167_d_n10, assign97800_e150167_d_n11, assign97800_e150167_d_n14,) = {
    if (locals.var_guard2268 != 0.0) {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign97800_e150167;
        locals.var_t3_dn0 = assign97800_e150167_d_n0;
        locals.var_t3_dn2 = assign97800_e150167_d_n2;
        locals.var_t3_dn4 = assign97800_e150167_d_n4;
        locals.var_t3_dn5 = assign97800_e150167_d_n5;
        locals.var_t3_dn6 = assign97800_e150167_d_n6;
        locals.var_t3_dn7 = assign97800_e150167_d_n7;
        locals.var_t3_dn8 = assign97800_e150167_d_n8;
        locals.var_t3_dn9 = assign97800_e150167_d_n9;
        locals.var_t3_dn10 = assign97800_e150167_d_n10;
        locals.var_t3_dn11 = assign97800_e150167_d_n11;
        locals.var_t3_dn14 = assign97800_e150167_d_n14;
        locals.var_t3_rv = 0.0;

        let assign97810_e150170: f64 = if locals.var_vbs_jct < locals.var_vbst { 1.0 } else { 0.0 };
        locals.var_guard2269 = assign97810_e150170;
        locals.var_guard2269_rv = 0.0;

        let (assign97820_e150178, assign97820_e150178_d_n0, assign97820_e150178_d_n2, assign97820_e150178_d_n4, assign97820_e150178_d_n5, assign97820_e150178_d_n6, assign97820_e150178_d_n7, assign97820_e150178_d_n8, assign97820_e150178_d_n9, assign97820_e150178_d_n10, assign97820_e150178_d_n11, assign97820_e150178_d_n14,) = {
    if ((locals.var_guard2268 != 0.0) && (locals.var_guard2269 != 0.0)) {
        let assign97820_e150176: f64 = (locals.var_vbs_jct * locals.var_jd_nvtm_invs);
        (assign97820_e150176, (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn0), ((locals.var_vbs_jct_dn2 * locals.var_jd_nvtm_invs) + (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn2)), (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn4), (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn5), (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn6), (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn7), (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn8), (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn9), (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn10), ((locals.var_vbs_jct_dn11 * locals.var_jd_nvtm_invs) + (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn11)), (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn14),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign97820_e150178;
        locals.var_tx_dn0 = assign97820_e150178_d_n0;
        locals.var_tx_dn2 = assign97820_e150178_d_n2;
        locals.var_tx_dn4 = assign97820_e150178_d_n4;
        locals.var_tx_dn5 = assign97820_e150178_d_n5;
        locals.var_tx_dn6 = assign97820_e150178_d_n6;
        locals.var_tx_dn7 = assign97820_e150178_d_n7;
        locals.var_tx_dn8 = assign97820_e150178_d_n8;
        locals.var_tx_dn9 = assign97820_e150178_d_n9;
        locals.var_tx_dn10 = assign97820_e150178_d_n10;
        locals.var_tx_dn11 = assign97820_e150178_d_n11;
        locals.var_tx_dn14 = assign97820_e150178_d_n14;
        locals.var_tx_rv = 0.0;

        let assign97830_e150181: f64 = (-3.0);
        let assign97830_e150183: f64 = (assign97830_e150181 * 34.0);
        let assign97830_e150184: f64 = if locals.var_tx < assign97830_e150183 { 1.0 } else { 0.0 };
        locals.var_guard2270 = assign97830_e150184;
        locals.var_guard2270_rv = 0.0;

        let (assign97840_e150192, assign97840_e150192_d_n0, assign97840_e150192_d_n2, assign97840_e150192_d_n4, assign97840_e150192_d_n5, assign97840_e150192_d_n6, assign97840_e150192_d_n7, assign97840_e150192_d_n8, assign97840_e150192_d_n9, assign97840_e150192_d_n10, assign97840_e150192_d_n11, assign97840_e150192_d_n14,) = {
    if (((locals.var_guard2268 != 0.0) && (locals.var_guard2269 != 0.0)) && (locals.var_guard2270 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign97840_e150192;
        locals.var_t1_dn0 = assign97840_e150192_d_n0;
        locals.var_t1_dn2 = assign97840_e150192_d_n2;
        locals.var_t1_dn4 = assign97840_e150192_d_n4;
        locals.var_t1_dn5 = assign97840_e150192_d_n5;
        locals.var_t1_dn6 = assign97840_e150192_d_n6;
        locals.var_t1_dn7 = assign97840_e150192_d_n7;
        locals.var_t1_dn8 = assign97840_e150192_d_n8;
        locals.var_t1_dn9 = assign97840_e150192_d_n9;
        locals.var_t1_dn10 = assign97840_e150192_d_n10;
        locals.var_t1_dn11 = assign97840_e150192_d_n11;
        locals.var_t1_dn14 = assign97840_e150192_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign97850_e150202, assign97850_e150202_d_n0, assign97850_e150202_d_n2, assign97850_e150202_d_n4, assign97850_e150202_d_n5, assign97850_e150202_d_n6, assign97850_e150202_d_n7, assign97850_e150202_d_n8, assign97850_e150202_d_n9, assign97850_e150202_d_n10, assign97850_e150202_d_n11, assign97850_e150202_d_n14,) = {
    if (((locals.var_guard2268 != 0.0) && (locals.var_guard2269 != 0.0)) && (locals.var_guard2270 == 0.0)) {
        let assign97850_e150200: f64 = (locals.var_tx).exp();
        (assign97850_e150200, (assign97850_e150200 * locals.var_tx_dn0), (assign97850_e150200 * locals.var_tx_dn2), (assign97850_e150200 * locals.var_tx_dn4), (assign97850_e150200 * locals.var_tx_dn5), (assign97850_e150200 * locals.var_tx_dn6), (assign97850_e150200 * locals.var_tx_dn7), (assign97850_e150200 * locals.var_tx_dn8), (assign97850_e150200 * locals.var_tx_dn9), (assign97850_e150200 * locals.var_tx_dn10), (assign97850_e150200 * locals.var_tx_dn11), (assign97850_e150200 * locals.var_tx_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign97850_e150202;
        locals.var_t1_dn0 = assign97850_e150202_d_n0;
        locals.var_t1_dn2 = assign97850_e150202_d_n2;
        locals.var_t1_dn4 = assign97850_e150202_d_n4;
        locals.var_t1_dn5 = assign97850_e150202_d_n5;
        locals.var_t1_dn6 = assign97850_e150202_d_n6;
        locals.var_t1_dn7 = assign97850_e150202_d_n7;
        locals.var_t1_dn8 = assign97850_e150202_d_n8;
        locals.var_t1_dn9 = assign97850_e150202_d_n9;
        locals.var_t1_dn10 = assign97850_e150202_d_n10;
        locals.var_t1_dn11 = assign97850_e150202_d_n11;
        locals.var_t1_dn14 = assign97850_e150202_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign97870_e150231, assign97870_e150231_d_n0, assign97870_e150231_d_n2, assign97870_e150231_d_n4, assign97870_e150231_d_n5, assign97870_e150231_d_n6, assign97870_e150231_d_n7, assign97870_e150231_d_n8, assign97870_e150231_d_n9, assign97870_e150231_d_n10, assign97870_e150231_d_n11, assign97870_e150231_d_n14,) = {
    if ((locals.var_guard2268 != 0.0) && (locals.var_guard2269 == 0.0)) {
        (locals.var_jd_expcs, locals.var_jd_expcs_dn0, locals.var_jd_expcs_dn2, locals.var_jd_expcs_dn4, locals.var_jd_expcs_dn5, locals.var_jd_expcs_dn6, locals.var_jd_expcs_dn7, locals.var_jd_expcs_dn8, locals.var_jd_expcs_dn9, locals.var_jd_expcs_dn10, locals.var_jd_expcs_dn11, locals.var_jd_expcs_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign97870_e150231;
        locals.var_t1_dn0 = assign97870_e150231_d_n0;
        locals.var_t1_dn2 = assign97870_e150231_d_n2;
        locals.var_t1_dn4 = assign97870_e150231_d_n4;
        locals.var_t1_dn5 = assign97870_e150231_d_n5;
        locals.var_t1_dn6 = assign97870_e150231_d_n6;
        locals.var_t1_dn7 = assign97870_e150231_d_n7;
        locals.var_t1_dn8 = assign97870_e150231_d_n8;
        locals.var_t1_dn9 = assign97870_e150231_d_n9;
        locals.var_t1_dn10 = assign97870_e150231_d_n10;
        locals.var_t1_dn11 = assign97870_e150231_d_n11;
        locals.var_t1_dn14 = assign97870_e150231_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign97880_e150242, assign97880_e150242_d_n0, assign97880_e150242_d_n2, assign97880_e150242_d_n4, assign97880_e150242_d_n5, assign97880_e150242_d_n6, assign97880_e150242_d_n7, assign97880_e150242_d_n8, assign97880_e150242_d_n9, assign97880_e150242_d_n10, assign97880_e150242_d_n11, assign97880_e150242_d_n14,) = {
    if ((locals.var_guard2268 != 0.0) && (locals.var_guard2269 == 0.0)) {
        let assign97880_e150238: f64 = (locals.var_isbs_btm * locals.var_jd_nvtm_invs);
        let assign97880_e150240: f64 = (assign97880_e150238 * locals.var_t1);
        (assign97880_e150240, ((((locals.var_isbs_btm_dn0 * locals.var_jd_nvtm_invs) + (locals.var_isbs_btm * locals.var_jd_nvtm_invs_dn0)) * locals.var_t1) + (assign97880_e150238 * locals.var_t1_dn0)), ((((locals.var_isbs_btm_dn2 * locals.var_jd_nvtm_invs) + (locals.var_isbs_btm * locals.var_jd_nvtm_invs_dn2)) * locals.var_t1) + (assign97880_e150238 * locals.var_t1_dn2)), ((((locals.var_isbs_btm_dn4 * locals.var_jd_nvtm_invs) + (locals.var_isbs_btm * locals.var_jd_nvtm_invs_dn4)) * locals.var_t1) + (assign97880_e150238 * locals.var_t1_dn4)), ((((locals.var_isbs_btm_dn5 * locals.var_jd_nvtm_invs) + (locals.var_isbs_btm * locals.var_jd_nvtm_invs_dn5)) * locals.var_t1) + (assign97880_e150238 * locals.var_t1_dn5)), ((((locals.var_isbs_btm_dn6 * locals.var_jd_nvtm_invs) + (locals.var_isbs_btm * locals.var_jd_nvtm_invs_dn6)) * locals.var_t1) + (assign97880_e150238 * locals.var_t1_dn6)), ((((locals.var_isbs_btm_dn7 * locals.var_jd_nvtm_invs) + (locals.var_isbs_btm * locals.var_jd_nvtm_invs_dn7)) * locals.var_t1) + (assign97880_e150238 * locals.var_t1_dn7)), ((((locals.var_isbs_btm_dn8 * locals.var_jd_nvtm_invs) + (locals.var_isbs_btm * locals.var_jd_nvtm_invs_dn8)) * locals.var_t1) + (assign97880_e150238 * locals.var_t1_dn8)), ((((locals.var_isbs_btm_dn9 * locals.var_jd_nvtm_invs) + (locals.var_isbs_btm * locals.var_jd_nvtm_invs_dn9)) * locals.var_t1) + (assign97880_e150238 * locals.var_t1_dn9)), ((((locals.var_isbs_btm_dn10 * locals.var_jd_nvtm_invs) + (locals.var_isbs_btm * locals.var_jd_nvtm_invs_dn10)) * locals.var_t1) + (assign97880_e150238 * locals.var_t1_dn10)), ((((locals.var_isbs_btm_dn11 * locals.var_jd_nvtm_invs) + (locals.var_isbs_btm * locals.var_jd_nvtm_invs_dn11)) * locals.var_t1) + (assign97880_e150238 * locals.var_t1_dn11)), ((((locals.var_isbs_btm_dn14 * locals.var_jd_nvtm_invs) + (locals.var_isbs_btm * locals.var_jd_nvtm_invs_dn14)) * locals.var_t1) + (assign97880_e150238 * locals.var_t1_dn14)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign97880_e150242;
        locals.var_t4_dn0 = assign97880_e150242_d_n0;
        locals.var_t4_dn2 = assign97880_e150242_d_n2;
        locals.var_t4_dn4 = assign97880_e150242_d_n4;
        locals.var_t4_dn5 = assign97880_e150242_d_n5;
        locals.var_t4_dn6 = assign97880_e150242_d_n6;
        locals.var_t4_dn7 = assign97880_e150242_d_n7;
        locals.var_t4_dn8 = assign97880_e150242_d_n8;
        locals.var_t4_dn9 = assign97880_e150242_d_n9;
        locals.var_t4_dn10 = assign97880_e150242_d_n10;
        locals.var_t4_dn11 = assign97880_e150242_d_n11;
        locals.var_t4_dn14 = assign97880_e150242_d_n14;
        locals.var_t4_rv = 0.0;

        let assign97910_e150279: f64 = (p.p537 * locals.var_isbs2_btm);
        locals.var_t12 = assign97910_e150279;
        locals.var_t12_dn0 = (p.p537 * locals.var_isbs2_btm_dn0);
        locals.var_t12_dn2 = (p.p537 * locals.var_isbs2_btm_dn2);
        locals.var_t12_dn4 = (p.p537 * locals.var_isbs2_btm_dn4);
        locals.var_t12_dn5 = (p.p537 * locals.var_isbs2_btm_dn5);
        locals.var_t12_dn6 = (p.p537 * locals.var_isbs2_btm_dn6);
        locals.var_t12_dn7 = (p.p537 * locals.var_isbs2_btm_dn7);
        locals.var_t12_dn8 = (p.p537 * locals.var_isbs2_btm_dn8);
        locals.var_t12_dn9 = (p.p537 * locals.var_isbs2_btm_dn9);
        locals.var_t12_dn10 = (p.p537 * locals.var_isbs2_btm_dn10);
        locals.var_t12_dn11 = (p.p537 * locals.var_isbs2_btm_dn11);
        locals.var_t12_dn14 = (p.p537 * locals.var_isbs2_btm_dn14);
        locals.var_t12_rv = 0.0;

        let assign97930_e150287: f64 = if locals.var_isbs_sws > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2271 = assign97930_e150287;
        locals.var_guard2271_rv = 0.0;

        let (assign97940_e150293, assign97940_e150293_d_n0, assign97940_e150293_d_n2, assign97940_e150293_d_n4, assign97940_e150293_d_n5, assign97940_e150293_d_n6, assign97940_e150293_d_n7, assign97940_e150293_d_n8, assign97940_e150293_d_n9, assign97940_e150293_d_n10, assign97940_e150293_d_n11, assign97940_e150293_d_n14,) = {
    if (locals.var_guard2271 != 0.0) {
        let assign97940_e150291: f64 = (locals.var_isbs2_sws * locals.var_t9);
        (assign97940_e150291, ((locals.var_isbs2_sws_dn0 * locals.var_t9) + (locals.var_isbs2_sws * locals.var_t9_dn0)), ((locals.var_isbs2_sws_dn2 * locals.var_t9) + (locals.var_isbs2_sws * locals.var_t9_dn2)), ((locals.var_isbs2_sws_dn4 * locals.var_t9) + (locals.var_isbs2_sws * locals.var_t9_dn4)), ((locals.var_isbs2_sws_dn5 * locals.var_t9) + (locals.var_isbs2_sws * locals.var_t9_dn5)), ((locals.var_isbs2_sws_dn6 * locals.var_t9) + (locals.var_isbs2_sws * locals.var_t9_dn6)), ((locals.var_isbs2_sws_dn7 * locals.var_t9) + (locals.var_isbs2_sws * locals.var_t9_dn7)), ((locals.var_isbs2_sws_dn8 * locals.var_t9) + (locals.var_isbs2_sws * locals.var_t9_dn8)), ((locals.var_isbs2_sws_dn9 * locals.var_t9) + (locals.var_isbs2_sws * locals.var_t9_dn9)), ((locals.var_isbs2_sws_dn10 * locals.var_t9) + (locals.var_isbs2_sws * locals.var_t9_dn10)), ((locals.var_isbs2_sws_dn11 * locals.var_t9) + (locals.var_isbs2_sws * locals.var_t9_dn11)), ((locals.var_isbs2_sws_dn14 * locals.var_t9) + (locals.var_isbs2_sws * locals.var_t9_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign97940_e150293;
        locals.var_t0_dn0 = assign97940_e150293_d_n0;
        locals.var_t0_dn2 = assign97940_e150293_d_n2;
        locals.var_t0_dn4 = assign97940_e150293_d_n4;
        locals.var_t0_dn5 = assign97940_e150293_d_n5;
        locals.var_t0_dn6 = assign97940_e150293_d_n6;
        locals.var_t0_dn7 = assign97940_e150293_d_n7;
        locals.var_t0_dn8 = assign97940_e150293_d_n8;
        locals.var_t0_dn9 = assign97940_e150293_d_n9;
        locals.var_t0_dn10 = assign97940_e150293_d_n10;
        locals.var_t0_dn11 = assign97940_e150293_d_n11;
        locals.var_t0_dn14 = assign97940_e150293_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign97950_e150300, assign97950_e150300_d_n0, assign97950_e150300_d_n2, assign97950_e150300_d_n4, assign97950_e150300_d_n5, assign97950_e150300_d_n6, assign97950_e150300_d_n7, assign97950_e150300_d_n8, assign97950_e150300_d_n9, assign97950_e150300_d_n10, assign97950_e150300_d_n11, assign97950_e150300_d_n14,) = {
    if (locals.var_guard2271 != 0.0) {
        let assign97950_e150296: f64 = (-locals.var_vbs_jct);
        let assign97950_e150298: f64 = (assign97950_e150296 * locals.var_t10);
        (assign97950_e150298, (assign97950_e150296 * locals.var_t10_dn0), (((-locals.var_vbs_jct_dn2) * locals.var_t10) + (assign97950_e150296 * locals.var_t10_dn2)), (assign97950_e150296 * locals.var_t10_dn4), (assign97950_e150296 * locals.var_t10_dn5), (assign97950_e150296 * locals.var_t10_dn6), (assign97950_e150296 * locals.var_t10_dn7), (assign97950_e150296 * locals.var_t10_dn8), (assign97950_e150296 * locals.var_t10_dn9), (assign97950_e150296 * locals.var_t10_dn10), (((-locals.var_vbs_jct_dn11) * locals.var_t10) + (assign97950_e150296 * locals.var_t10_dn11)), (assign97950_e150296 * locals.var_t10_dn14),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign97950_e150300;
        locals.var_tx_dn0 = assign97950_e150300_d_n0;
        locals.var_tx_dn2 = assign97950_e150300_d_n2;
        locals.var_tx_dn4 = assign97950_e150300_d_n4;
        locals.var_tx_dn5 = assign97950_e150300_d_n5;
        locals.var_tx_dn6 = assign97950_e150300_d_n6;
        locals.var_tx_dn7 = assign97950_e150300_d_n7;
        locals.var_tx_dn8 = assign97950_e150300_d_n8;
        locals.var_tx_dn9 = assign97950_e150300_d_n9;
        locals.var_tx_dn10 = assign97950_e150300_d_n10;
        locals.var_tx_dn11 = assign97950_e150300_d_n11;
        locals.var_tx_dn14 = assign97950_e150300_d_n14;
        locals.var_tx_rv = 0.0;

        let (assign97960_e150305, assign97960_e150305_d_n0, assign97960_e150305_d_n2, assign97960_e150305_d_n4, assign97960_e150305_d_n5, assign97960_e150305_d_n6, assign97960_e150305_d_n7, assign97960_e150305_d_n8, assign97960_e150305_d_n9, assign97960_e150305_d_n10, assign97960_e150305_d_n11, assign97960_e150305_d_n14,) = {
    if (locals.var_guard2271 != 0.0) {
        let assign97960_e150303: f64 = (locals.var_tx).exp();
        (assign97960_e150303, (assign97960_e150303 * locals.var_tx_dn0), (assign97960_e150303 * locals.var_tx_dn2), (assign97960_e150303 * locals.var_tx_dn4), (assign97960_e150303 * locals.var_tx_dn5), (assign97960_e150303 * locals.var_tx_dn6), (assign97960_e150303 * locals.var_tx_dn7), (assign97960_e150303 * locals.var_tx_dn8), (assign97960_e150303 * locals.var_tx_dn9), (assign97960_e150303 * locals.var_tx_dn10), (assign97960_e150303 * locals.var_tx_dn11), (assign97960_e150303 * locals.var_tx_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign97960_e150305;
        locals.var_t2_dn0 = assign97960_e150305_d_n0;
        locals.var_t2_dn2 = assign97960_e150305_d_n2;
        locals.var_t2_dn4 = assign97960_e150305_d_n4;
        locals.var_t2_dn5 = assign97960_e150305_d_n5;
        locals.var_t2_dn6 = assign97960_e150305_d_n6;
        locals.var_t2_dn7 = assign97960_e150305_d_n7;
        locals.var_t2_dn8 = assign97960_e150305_d_n8;
        locals.var_t2_dn9 = assign97960_e150305_d_n9;
        locals.var_t2_dn10 = assign97960_e150305_d_n10;
        locals.var_t2_dn11 = assign97960_e150305_d_n11;
        locals.var_t2_dn14 = assign97960_e150305_d_n14;
        locals.var_t2_rv = 0.0;

    }
}
